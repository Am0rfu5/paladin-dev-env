#!/usr/bin/env bash
# check-crate-names.sh
#
# Bidirectional set-equality guard over the workspace's publishable crates.io
# package names. `.crate-names.txt` is the committed, hand-edited allow-list
# of names this project already owns; this script is the only place that
# consumes it. "Publishable" is determined by parsing each manifest's
# [package] table with tomllib and reading its `publish` field -- a crate is
# exempt only when its manifest says `publish = false`, never by directory
# name. A crates/<name>/ directory with no Cargo.toml is not a crate and is
# skipped silently. Comparison is exact string equality: no case-folding, no
# hyphen/underscore normalisation, no substring matching -- a name differing
# only in those respects is a different crates.io package.
#
# The guard asserts BOTH directions of set equality:
#   - a tree package name absent from the allow-list is an unowned/new name,
#     the exact collision risk this guard exists to catch;
#   - an allow-list entry with no corresponding tree package is a stale
#     entry -- a different integrity failure, but checking only the first
#     direction would let the guard pass vacuously against a newly added
#     colliding name (it would report green while never having examined the
#     risk).
#
# This is deliberately offline: a live crates.io availability query was
# rejected (crates.io returns HTTP 403 in this environment, so it could be
# written but never demonstrated) -- see ADR-0026.
#
# Usage:  ./scripts/check-crate-names.sh
# Exit:   0 if the tree's publishable package names exactly match the
#         allow-list (as sets); non-zero otherwise.

set -euo pipefail

WORKSPACE_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
ALLOWLIST="${WORKSPACE_ROOT}/.crate-names.txt"

if [ ! -f "${ALLOWLIST}" ]; then
    echo "⚠️  No allow-list found at ${ALLOWLIST}"
    echo "   Create it: one crates.io package name per line, hand-edited (see ADR-0026)."
    exit 1
fi

if ! command -v python3 >/dev/null 2>&1; then
    echo "ERROR: python3 is required for Cargo.toml parsing." >&2
    exit 1
fi

echo "🔍 Checking crates.io package-name allow-list against the workspace ..."

REPORT=$(python3 - "${WORKSPACE_ROOT}" "${ALLOWLIST}" <<'PY'
import glob
import os
import sys
import tomllib

workspace_root, allowlist_path = sys.argv[1], sys.argv[2]

# --- Read the allow-list as a set. Order is irrelevant; blank lines and
# comment lines (first non-whitespace character is '#') are ignored, and
# surrounding whitespace is stripped from each name. ---
allowed = set()
with open(allowlist_path, "r", encoding="utf-8") as fh:
    for line in fh:
        stripped = line.strip()
        if not stripped or stripped.startswith("#"):
            continue
        allowed.add(stripped)

# --- Walk the root Cargo.toml and every crates/*/Cargo.toml. Read
# [package].name specifically -- never grep for a name line, since the root
# manifest also carries [[bin]], [[test]] and [[bench]] tables with their
# own `name` keys, and at least one crate's [lib] name differs from its
# [package] name. Skip any manifest whose [package].publish is false. Skip a
# crates/*/ directory with no Cargo.toml without raising an error. ---
manifest_paths = [os.path.join(workspace_root, "Cargo.toml")]
manifest_paths += sorted(glob.glob(os.path.join(workspace_root, "crates", "*", "Cargo.toml")))

tree_names = set()
for manifest_path in manifest_paths:
    if not os.path.isfile(manifest_path):
        continue
    with open(manifest_path, "rb") as fh:
        manifest = tomllib.load(fh)
    package = manifest.get("package", {})
    name = package.get("name")
    if name is None:
        continue
    # publish = false is the only exemption signal; absence of the key, or
    # any other value, means the crate is publishable by Cargo's own default.
    if package.get("publish") is False:
        continue
    tree_names.add(name)

if not tree_names:
    print("ZERO_CRATES")
    print("FAIL: zero publishable crates discovered in the workspace -- this "
          "looks like a broken glob or an empty workspace, not success.")
    sys.exit(0)

# Exact set equality, both directions. No case-folding, no hyphen/underscore
# normalisation, no substring matching -- comparison is exact string
# equality on the raw name, so e.g. `paladin-web` does not satisfy an entry
# of `Paladin-Web` or `paladin_web`.
unlisted = sorted(tree_names - allowed)
stale = sorted(allowed - tree_names)

if unlisted or stale:
    print("MISMATCH")
    if unlisted:
        print(f"FAIL: {len(unlisted)} tree package name(s) not on the allow-list:")
        for name in unlisted:
            print(f"  - {name}")
    if stale:
        plural = "y" if len(stale) == 1 else "ies"
        print(f"FAIL: {len(stale)} allow-list entr{plural} with no corresponding tree crate:")
        for name in stale:
            print(f"  - {name}")
    sys.exit(0)

print("OK")
print(f"{len(tree_names)} publishable crate(s) checked, all match the allow-list exactly.")
sys.exit(0)
PY
)

STATUS_LINE=$(head -n1 <<<"${REPORT}")
DETAIL=$(tail -n +2 <<<"${REPORT}")

if [ "${STATUS_LINE}" = "OK" ]; then
    echo "✅ ${DETAIL}"
    exit 0
else
    echo "❌ Crate-name allow-list check failed"
    echo ""
    echo "${DETAIL}"
    echo ""
    echo "If this failure is unexpected:"
    echo "  1. An unlisted tree name means a new crate name was added. Confirm it is"
    echo "     available on crates.io, then add it to .crate-names.txt yourself --"
    echo "     the list is hand-edited and never auto-generated from the tree."
    echo "  2. A stale allow-list entry means a crate was removed or renamed. Remove"
    echo "     its old name from .crate-names.txt."
    exit 1
fi
