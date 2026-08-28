#!/usr/bin/env bash
# check-release-consistency.sh
#
# The pre-publish consistency gate (PUBOPS-01). Verifies that a release
# tag's version string exactly matches the manifest version of every
# publishable Cargo workspace package, before `publish-crates` is allowed to
# run. PUBOPS-01's binding claim is "no crate is published until the tag,
# every manifest version and every changelog agree" -- this script is the
# single source of truth both the release.yml gate job and the local `make
# check-release-consistency` target invoke; no gate logic is duplicated in
# workflow YAML.
#
# This script is offline when --metadata-json is supplied, and otherwise
# makes exactly one local `cargo metadata` call -- it never talks to the
# network. It accumulates every mismatch into one report rather than
# stopping at the first, so a release with three broken crates gets one
# report naming all three, never a fix-one-rerun-find-the-next loop. It only
# reads: given the same tag and the same metadata, running it twice produces
# byte-identical output and the same exit code.
#
# Clause implemented in this plan (D-08 clause 1 of 4 -- manifest agreement;
# the remaining three clauses, changelog dates / CI conclusion / SHA
# agreement, are added by a later plan):
#
#   1. Manifest agreement. Every publishable package in the Cargo workspace
#      -- discovered via `cargo metadata --no-deps --format-version 1`,
#      never a hardcoded list -- has a `version` field that is exactly equal
#      (string equality, no semver normalisation or coercion) to the tag's
#      version with at most one leading `v` stripped. `0.8.1` and
#      `0.8.1-rc.2` are different strings and therefore a mismatch. A
#      package is publishable when its manifest's `publish` field is absent
#      (`cargo metadata` reports this as JSON `null`); a package carrying
#      `publish = false` (`cargo metadata` reports this as an empty list)
#      is excluded by design, never counted as a mismatch.
#
# Discovering zero publishable packages is a named ZERO_PACKAGES failure
# with a non-zero exit -- a broken enumeration must never present as a pass
# over an empty set, matching the convention `check-changelogs.sh` and
# `check-workflow-triggers.sh` already establish for this repo's gate
# scripts.
#
# Reserved flags: --sha and --ci-runs-json are accepted but currently
# unused -- a later plan gives them behaviour (the SHA-agreement and
# CI-conclusion clauses). Passing them today is a silent no-op, not a usage
# error, so this plan's CLI surface does not need to change shape again.
#
# Sourcing seam: set CHECK_RELEASE_CONSISTENCY_LIB_ONLY=1 before sourcing
# this file to load the check_release_consistency_main function without
# executing it -- this file's own regression harness (and later plans'
# harnesses) use this to exercise the function directly.
#
# Usage:  ./scripts/check-release-consistency.sh --tag <vX.Y.Z|X.Y.Z>
#             [--metadata-json <path>] [--workspace-root <path>]
#             [--sha <sha>] [--ci-runs-json <path>]
#         --tag is required. --metadata-json, when supplied, is read
#         instead of invoking `cargo metadata` -- the fixture seam this
#         script's regression test uses so it never invokes cargo.
#         --workspace-root overrides the repo root `cargo metadata` is run
#         from (default: derived from BASH_SOURCE); it has no effect when
#         --metadata-json is supplied. An unrecognised flag is a usage
#         error.
# Exit:   0 if every publishable package's manifest version exactly equals
#         the tag version (status OK); non-zero for MISMATCH, ZERO_PACKAGES,
#         MISSING_TAG, or a usage error (unknown flag / missing python3).

set -euo pipefail

check_release_consistency_main() {
    local WORKSPACE_ROOT
    WORKSPACE_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

    local TAG="" METADATA_JSON="" WORKSPACE_ROOT_ARG="" SHA="" CI_RUNS_JSON=""

    while [ "$#" -gt 0 ]; do
        case "$1" in
            --tag)
                if [ "$#" -lt 2 ]; then
                    echo "ERROR: --tag requires a value." >&2
                    return 1
                fi
                TAG="$2"
                shift 2
                ;;
            --metadata-json)
                if [ "$#" -lt 2 ]; then
                    echo "ERROR: --metadata-json requires a value." >&2
                    return 1
                fi
                METADATA_JSON="$2"
                shift 2
                ;;
            --workspace-root)
                if [ "$#" -lt 2 ]; then
                    echo "ERROR: --workspace-root requires a value." >&2
                    return 1
                fi
                WORKSPACE_ROOT_ARG="$2"
                shift 2
                ;;
            --sha)
                # Reserved, currently unused -- see header comment.
                if [ "$#" -lt 2 ]; then
                    echo "ERROR: --sha requires a value." >&2
                    return 1
                fi
                SHA="$2"
                shift 2
                ;;
            --ci-runs-json)
                # Reserved, currently unused -- see header comment.
                if [ "$#" -lt 2 ]; then
                    echo "ERROR: --ci-runs-json requires a value." >&2
                    return 1
                fi
                CI_RUNS_JSON="$2"
                shift 2
                ;;
            *)
                echo "ERROR: unknown flag: $1" >&2
                echo "Usage: check-release-consistency.sh --tag <vX.Y.Z|X.Y.Z> [--metadata-json <path>] [--workspace-root <path>] [--sha <sha>] [--ci-runs-json <path>]" >&2
                return 1
                ;;
        esac
    done
    # SHA and CI_RUNS_JSON are intentionally unused in this plan; reference
    # them so shellcheck does not flag them as unused assignments.
    : "${SHA}" "${CI_RUNS_JSON}"

    if [ -n "${WORKSPACE_ROOT_ARG}" ]; then
        WORKSPACE_ROOT="${WORKSPACE_ROOT_ARG}"
    fi

    if ! command -v python3 >/dev/null 2>&1; then
        echo "ERROR: python3 is required for Cargo metadata parsing." >&2
        return 1
    fi

    local REPORT
    if [ -z "${TAG}" ]; then
        REPORT=$'MISSING_TAG\nFAIL: --tag is required. Usage: check-release-consistency.sh --tag vX.Y.Z (or X.Y.Z, or --metadata-json/--workspace-root for local runs).'
    else
        # Strip at most one leading "v" -- "${TAG#v}" removes the shortest
        # matching prefix once, so "v1.2.3" -> "1.2.3" and "1.2.3" -> "1.2.3"
        # unchanged. No semver parsing anywhere in this script.
        local TAG_VERSION="${TAG#v}"

        local METADATA_PATH="" CLEANUP_METADATA=""
        trap '[ -n "${CLEANUP_METADATA}" ] && rm -f "${CLEANUP_METADATA}"' RETURN

        if [ -n "${METADATA_JSON}" ]; then
            if [ ! -f "${METADATA_JSON}" ]; then
                echo "ERROR: --metadata-json file not found: ${METADATA_JSON}" >&2
                return 1
            fi
            METADATA_PATH="${METADATA_JSON}"
        else
            METADATA_PATH="$(mktemp "${TMPDIR:-/tmp}/check-release-consistency-metadata.XXXXXX.json")"
            CLEANUP_METADATA="${METADATA_PATH}"
            if ! (cd "${WORKSPACE_ROOT}" && cargo metadata --no-deps --format-version 1) > "${METADATA_PATH}"; then
                echo "ERROR: 'cargo metadata --no-deps --format-version 1' failed in ${WORKSPACE_ROOT}." >&2
                return 1
            fi
        fi

        REPORT=$(python3 - "${METADATA_PATH}" "${TAG_VERSION}" <<'PY'
import json
import sys

metadata_path = sys.argv[1]
tag_version = sys.argv[2]

try:
    with open(metadata_path, "r", encoding="utf-8") as fh:
        data = json.load(fh)
except (OSError, json.JSONDecodeError) as exc:
    print("MISMATCH")
    print(f"FAIL: could not read/parse metadata JSON at {metadata_path}: {exc}")
    sys.exit(0)

packages = data.get("packages", []) if isinstance(data, dict) else []
publishable = [
    p for p in packages
    if isinstance(p, dict) and p.get("publish") is None
]

if not publishable:
    print("ZERO_PACKAGES")
    print("FAIL: zero publishable packages discovered from cargo metadata -- a broken "
          "enumeration is a named failure, never a pass over an empty set. A package is "
          "publishable when its manifest omits `publish` (cargo metadata reports this as "
          "`null`); one carrying `publish = false` (reported as an empty list) is excluded "
          "by design, not a bug.")
    sys.exit(0)

mismatches = []
for p in publishable:
    name = p.get("name", "<unknown>")
    version = p.get("version", "")
    if version != tag_version:
        mismatches.append((name, version))

if mismatches:
    print("MISMATCH")
    print(f"FAIL: tag version {tag_version!r} does not match {len(mismatches)} of "
          f"{len(publishable)} publishable package(s):")
    for name, version in sorted(mismatches, key=lambda t: t[0]):
        print(f"  - {name}: manifest version {version!r} != tag version {tag_version!r}")
    sys.exit(0)

print("OK")
print(f"{len(publishable)} publishable package(s) checked, all match tag version {tag_version!r}.")
sys.exit(0)
PY
)
    fi

    local STATUS_LINE DETAIL
    STATUS_LINE=$(head -n1 <<<"${REPORT}")
    DETAIL=$(tail -n +2 <<<"${REPORT}")

    if [ "${STATUS_LINE}" = "OK" ]; then
        echo "✅ OK: ${DETAIL}"
        return 0
    else
        echo "❌ Pre-publish consistency check failed (${STATUS_LINE})"
        echo ""
        echo "${DETAIL}"
        echo ""
        echo "If this failure is unexpected:"
        echo "  1. MISMATCH: a publishable crate's Cargo.toml [package] version does not"
        echo "     equal the tag -- update the crate version (workspace lockstep) or fix"
        echo "     the tag."
        echo "  2. ZERO_PACKAGES: cargo metadata enumerated no publishable packages --"
        echo "     check for a broken workspace or an accidental publish = false on every"
        echo "     crate."
        echo "  3. MISSING_TAG: pass --tag vX.Y.Z (or X.Y.Z)."
        echo "  4. If this guard is wrong about a crate or the tag, fix the guard rather"
        echo "     than working around it."
        return 1
    fi
}

if [ "${CHECK_RELEASE_CONSISTENCY_LIB_ONLY:-0}" != "1" ]; then
    check_release_consistency_main "$@"
fi
