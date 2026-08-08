#!/usr/bin/env bash
# check-changelogs.sh
#
# Asserts that every publishable crate under crates/*/ carries a CHANGELOG.md.
# "Publishable" is determined by parsing each crate's Cargo.toml [package]
# table with tomllib and reading its `publish` field -- a crate is exempt only
# when its manifest says `publish = false`, never by directory name. A
# crates/<name>/ directory with no Cargo.toml is not a crate and is skipped
# silently. Discovering zero publishable crates is itself a failure, not a
# vacuous pass -- a broken glob must never look like success.
#
# Usage:  ./scripts/check-changelogs.sh
# Exit:   0 if every publishable crate has a CHANGELOG.md, non-zero otherwise.

set -euo pipefail

WORKSPACE_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
CRATES_DIR="${WORKSPACE_ROOT}/crates"

if ! command -v python3 >/dev/null 2>&1; then
    echo "ERROR: python3 is required for Cargo.toml parsing." >&2
    exit 1
fi

echo "🔍 Checking per-crate CHANGELOG.md coverage in ${CRATES_DIR} ..."

REPORT=$(python3 - "${CRATES_DIR}" <<'PY'
import glob
import os
import sys
import tomllib

crates_dir = sys.argv[1]

publishable = []
missing = []

for manifest_path in sorted(glob.glob(os.path.join(crates_dir, "*", "Cargo.toml"))):
    crate_dir = os.path.dirname(manifest_path)

    with open(manifest_path, "rb") as fh:
        manifest = tomllib.load(fh)

    package = manifest.get("package", {})
    name = package.get("name", os.path.basename(crate_dir))

    # publish = false is the only exemption signal; absence of the key means
    # the crate is publishable by Cargo's own default.
    if package.get("publish") is False:
        continue

    publishable.append(name)

    changelog_path = os.path.join(crate_dir, "CHANGELOG.md")
    if not os.path.isfile(changelog_path):
        missing.append(name)

if not publishable:
    print("ZERO_CRATES")
    print("FAIL: zero publishable crates discovered under crates/*/ -- this "
          "looks like a broken glob or an empty workspace, not success.")
    sys.exit(0)

if missing:
    print("MISSING")
    print(f"FAIL: {len(missing)} publishable crate(s) missing CHANGELOG.md:")
    for name in missing:
        print(f"  - {name}")
    sys.exit(0)

print("OK")
print(f"{len(publishable)} publishable crate(s) checked, all have a CHANGELOG.md.")
sys.exit(0)
PY
)

STATUS_LINE=$(head -n1 <<<"${REPORT}")
DETAIL=$(tail -n +2 <<<"${REPORT}")

if [ "${STATUS_LINE}" = "OK" ]; then
    echo "✅ ${DETAIL}"
    exit 0
else
    echo "❌ Per-crate changelog check failed"
    echo ""
    echo "${DETAIL}"
    echo ""
    echo "If this failure is unexpected:"
    echo "  1. Add a CHANGELOG.md to each crate named above, matching the"
    echo "     Keep a Changelog shape used by its sibling crates."
    echo "  2. A crate should only be exempt if its Cargo.toml genuinely sets"
    echo "     publish = false."
    exit 1
fi
