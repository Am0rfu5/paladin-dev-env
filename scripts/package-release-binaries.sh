#!/usr/bin/env bash
# package-release-binaries.sh
#
# Packages the binaries a release leg produced for one target triple
# (ARTIFACT-02, ARTIFACT-05): asserts every binary the target's manifest
# expects actually exists, strips each one, archives them into a
# deterministic-order tarball, and writes a portable sha256 checksum file.
#
# This exists because a bare `cargo build --release --target <triple>`
# silently skips any `[[bin]]` whose `required-features` are unmet -- Cargo
# does not fail the build, it just produces fewer binaries than declared.
# The old `Strip binary` / `Create archive` workflow steps assumed a single
# hardcoded `paladin` binary was always present and never checked; this
# script makes "a leg that produced no executable" a hard, named failure
# instead of a strip step crashing on a missing path three steps later.
#
# The expected-binary set is per-target *data*, owned by the single function
# `expected_binaries_for_target` below -- narrowing one target's set (should
# the aarch64 `cross` leg prove unable to build all three, RESEARCH.md open
# question 2) is a one-line edit to that function's case entries, with the
# reason recorded in a comment next to the change. An unrecognised target
# triple and a recognised target whose manifest resolves to an empty list
# are both hard failures, reported with different messages -- neither is a
# silent no-op, and an empty manifest never archives nothing while reporting
# success (the defect this script exists to remove).
#
# Binary names are matched as exact byte strings against `--release-dir`
# directory entries -- no glob, no case folding, no prefix match, and a
# directory entry that is not a regular file (e.g. a directory sharing the
# binary's name) counts as missing, not present. Archive membership is
# decided by the manifest, not by the directory listing: an extra,
# unexpected executable sitting in `--release-dir` is never archived.
#
# Sourcing seam: set PACKAGE_RELEASE_BINARIES_LIB_ONLY=1 before sourcing this
# file to load package_release_binaries_main (and its helpers, including
# expected_binaries_for_target) without executing it -- this file's own
# regression harness uses this to exercise the manifest function directly
# with a locally overridden implementation, without editing the shipped
# manifest.
#
# Usage:  ./scripts/package-release-binaries.sh --target <triple>
#             --release-dir <path> --archive-name <name>
#             [--strip-cmd <cmd>]
#         --target, --release-dir and --archive-name are required.
#         --release-dir is the directory containing the built binaries (e.g.
#         target/<triple>/release) -- the archive and checksum are written
#         into this same directory. --strip-cmd defaults to `strip`. An
#         unrecognised flag is a usage error.
# Output: on success, prints `archive_path=<path>` and
#         `checksum_path=<path>` to stdout, and appends the same two lines
#         to $GITHUB_OUTPUT when that variable is set and non-empty.
# Exit:   0 on success; non-zero for an unknown target, a missing binary, a
#         strip/archive/checksum failure, or a usage error. No archive is
#         created when any expected binary is missing.

set -euo pipefail

# expected_binaries_for_target TARGET
#
# Prints the whitespace-separated list of binary names expected to exist in
# --release-dir for TARGET. This is the single source of truth for "what
# must this leg have built" -- narrowing a target's set is an edit to this
# function's case entries, with the reason recorded alongside the change.
#
# Returns non-zero (printing nothing) when TARGET is not a recognised triple
# at all -- distinct from a recognised triple whose list is deliberately
# narrowed to empty, which the caller also rejects, but with a different
# message (an empty manifest for a known target is itself a hard failure,
# never a silent no-op).
expected_binaries_for_target() {
    local target="$1"
    case "${target}" in
        x86_64-unknown-linux-gnu | aarch64-unknown-linux-gnu | x86_64-apple-darwin | aarch64-apple-darwin)
            echo "paladin paladin-cli paladin-server"
            ;;
        *)
            return 1
            ;;
    esac
}

# sha256_cmd FILE
#
# Prints a sha256sum-format line ("<hex-digest>  <filename>") for FILE.
# Selects `sha256sum` when it exists on PATH and falls back to
# `shasum -a 256` otherwise -- GitHub's macOS runner images have no GNU
# coreutils `sha256sum`, so the two macOS legs have never reached a checksum
# line without this fallback. Both tools emit the identical line format, so
# callers never need to know which one ran.
sha256_cmd() {
    local file="$1"
    if command -v sha256sum >/dev/null 2>&1; then
        sha256sum "${file}"
    else
        shasum -a 256 "${file}"
    fi
}

package_release_binaries_main() {
    local TARGET="" RELEASE_DIR="" ARCHIVE_NAME="" STRIP_CMD="strip"

    while [ "$#" -gt 0 ]; do
        case "$1" in
            --target)
                if [ "$#" -lt 2 ]; then
                    echo "ERROR: --target requires a value." >&2
                    return 1
                fi
                TARGET="$2"
                shift 2
                ;;
            --release-dir)
                if [ "$#" -lt 2 ]; then
                    echo "ERROR: --release-dir requires a value." >&2
                    return 1
                fi
                RELEASE_DIR="$2"
                shift 2
                ;;
            --archive-name)
                if [ "$#" -lt 2 ]; then
                    echo "ERROR: --archive-name requires a value." >&2
                    return 1
                fi
                ARCHIVE_NAME="$2"
                shift 2
                ;;
            --strip-cmd)
                if [ "$#" -lt 2 ]; then
                    echo "ERROR: --strip-cmd requires a value." >&2
                    return 1
                fi
                STRIP_CMD="$2"
                shift 2
                ;;
            *)
                echo "ERROR: unknown flag: $1" >&2
                echo "Usage: package-release-binaries.sh --target <triple> --release-dir <path> --archive-name <name> [--strip-cmd <cmd>]" >&2
                return 1
                ;;
        esac
    done

    if [ -z "${TARGET}" ]; then
        echo "ERROR: --target is required." >&2
        return 1
    fi
    if [ -z "${RELEASE_DIR}" ]; then
        echo "ERROR: --release-dir is required." >&2
        return 1
    fi
    if [ -z "${ARCHIVE_NAME}" ]; then
        echo "ERROR: --archive-name is required." >&2
        return 1
    fi
    if [ ! -d "${RELEASE_DIR}" ]; then
        echo "ERROR: --release-dir does not exist or is not a directory: ${RELEASE_DIR}" >&2
        return 1
    fi

    # --- (1) Resolve the expected list for --target. -----------------------
    local expected_list
    if ! expected_list="$(expected_binaries_for_target "${TARGET}")"; then
        echo "::error::unknown target for package-release-binaries: ${TARGET}" >&2
        return 1
    fi
    if [ -z "${expected_list}" ]; then
        echo "::error::expected-binary manifest for ${TARGET} is empty -- a known target must never archive nothing" >&2
        return 1
    fi

    # shellcheck disable=SC2206 # expected_list is a script-controlled,
    # whitespace-separated list of bare binary names -- word splitting here
    # is the intended parse, not an unquoted-expansion bug.
    local expected_arr=(${expected_list})

    # --- (2) Assert every expected binary exists as a regular file, by exact
    #     byte-string name, before creating anything. -----------------------
    local missing=()
    local name
    for name in "${expected_arr[@]}"; do
        if [ ! -f "${RELEASE_DIR}/${name}" ]; then
            missing+=("${name}")
        fi
    done

    if [ "${#missing[@]}" -gt 0 ]; then
        local missing_joined
        missing_joined="$(
            IFS=', '
            echo "${missing[*]}"
        )"
        echo "::error::expected binaries not built for ${TARGET}: ${missing_joined}" >&2
        return 1
    fi

    # --- (3) Strip each present binary. -------------------------------------
    for name in "${expected_arr[@]}"; do
        ${STRIP_CMD} "${RELEASE_DIR}/${name}"
    done

    # --- (4) Archive in manifest order, from inside --release-dir, so member
    #     order is deterministic and extra directory entries are never
    #     included -- the manifest decides membership, not the directory
    #     listing. ------------------------------------------------------------
    local archive_path="${RELEASE_DIR}/${ARCHIVE_NAME}.tar.gz"
    (
        cd "${RELEASE_DIR}"
        tar czf "${ARCHIVE_NAME}.tar.gz" -- "${expected_arr[@]}"
    )

    # --- (5) Portable checksum. ----------------------------------------------
    local checksum_path="${RELEASE_DIR}/${ARCHIVE_NAME}.tar.gz.sha256"
    (
        cd "${RELEASE_DIR}"
        sha256_cmd "${ARCHIVE_NAME}.tar.gz" > "${ARCHIVE_NAME}.tar.gz.sha256"
    )

    echo "archive_path=${archive_path}"
    echo "checksum_path=${checksum_path}"
    if [ -n "${GITHUB_OUTPUT:-}" ]; then
        {
            echo "archive_path=${archive_path}"
            echo "checksum_path=${checksum_path}"
        } >> "${GITHUB_OUTPUT}"
    fi

    return 0
}

if [ "${PACKAGE_RELEASE_BINARIES_LIB_ONLY:-0}" != "1" ]; then
    package_release_binaries_main "$@"
fi
