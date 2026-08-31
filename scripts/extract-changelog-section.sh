#!/usr/bin/env bash
# extract-changelog-section.sh
#
# Extracts one version's curated section from the root CHANGELOG.md so the
# `create-release` job can hand it to `create-or-reuse-release.sh --body-file`
# as the GitHub release body, byte-for-byte (ARTIFACT-01). This closes the
# gap left by the old "Generate changelog" step, which built the release body
# from `git log --pretty=format:"- %s"` over every commit since the previous
# tag -- for v0.8.0 that was 1,014 commit subjects -- discarding the
# hand-authored Keep-a-Changelog corpus this project maintains on purpose.
#
# There is no alternate body source in this script, and none is added to the
# calling job: a version with no `## [X.Y.Z]` heading is a hard, named
# failure (D-01), never a silent fall back to a commit-log summary.
#
# Boundary contract (mirrors scripts/check-release-consistency.sh's Clause 2
# `heading_re` at scripts/check-release-consistency.sh:401 exactly -- the two
# implementations must never silently diverge on where a section starts):
# the start boundary is the first line matching `^##\s*\[<escaped
# version>\](\s|$)`; the stop boundary is the next line matching `^##\s*\[`,
# or end of file. Everything strictly between the two boundaries is the
# section -- the heading line itself is never part of the output, and
# nothing is trimmed, re-wrapped, case-folded, or line-ending-rewritten. A
# heading-only section (no body text before the next heading) extracts to an
# empty string and still exits 0 -- presence of the heading is the pass
# signal, not presence of content (D-02).
#
# Version matching is byte-exact after stripping at most one leading `v`,
# matching check-release-consistency.sh's `${TAG#v}` exactly: `0.8.1` never
# matches a `0.8.1-rc.2` or `0.8.10` heading, and a trailing ` - <date>` on
# the heading line is ignored. The version is escaped with `re.escape`
# before matching (in a `python3 - <<'PY'` heredoc, as every other
# text-processing guard in scripts/ does) so a prerelease version's `.` and
# `-` are never interpreted as regex metacharacters.
#
# Sourcing seam: set EXTRACT_CHANGELOG_SECTION_LIB_ONLY=1 before sourcing
# this file to load the extract_changelog_section_main function without
# executing it -- this file's own regression harness uses this to exercise
# the function directly.
#
# Usage:  ./scripts/extract-changelog-section.sh --version <X.Y.Z|vX.Y.Z>
#             [--changelog <path>] [--output <path>]
#         --version is required. --changelog defaults to CHANGELOG.md
#         (relative to the caller's working directory). --output, when
#         given, writes the section there and prints `section_file=<path>`
#         to stdout, appending the same line to $GITHUB_OUTPUT when that
#         variable is set and non-empty; when omitted, the section is
#         printed to stdout instead. An unrecognised flag is a usage error.
# Exit:   0 when a `## [<version>]` section is found (including an empty
#         heading-only section); non-zero when no section exists for the
#         given version, the changelog file is missing, python3 is
#         unavailable, or a usage error.

set -euo pipefail

extract_changelog_section_main() {
    local CHANGELOG="CHANGELOG.md"
    local VERSION=""
    local OUTPUT=""

    while [ "$#" -gt 0 ]; do
        case "$1" in
            --changelog)
                if [ "$#" -lt 2 ]; then
                    echo "ERROR: --changelog requires a value." >&2
                    return 1
                fi
                CHANGELOG="$2"
                shift 2
                ;;
            --version)
                if [ "$#" -lt 2 ]; then
                    echo "ERROR: --version requires a value." >&2
                    return 1
                fi
                VERSION="$2"
                shift 2
                ;;
            --output)
                if [ "$#" -lt 2 ]; then
                    echo "ERROR: --output requires a value." >&2
                    return 1
                fi
                OUTPUT="$2"
                shift 2
                ;;
            *)
                echo "ERROR: unknown flag: $1" >&2
                echo "Usage: extract-changelog-section.sh --version <X.Y.Z|vX.Y.Z> [--changelog <path>] [--output <path>]" >&2
                return 1
                ;;
        esac
    done

    if [ -z "${VERSION}" ]; then
        echo "ERROR: --version is required." >&2
        return 1
    fi

    if ! command -v python3 >/dev/null 2>&1; then
        echo "ERROR: python3 is required for changelog section extraction." >&2
        return 1
    fi

    if [ ! -f "${CHANGELOG}" ]; then
        echo "ERROR: changelog not found at ${CHANGELOG}" >&2
        return 1
    fi

    # Strip at most one leading "v" -- "${VERSION#v}" removes the shortest
    # matching prefix once, matching check-release-consistency.sh's
    # "${TAG#v}" exactly (scripts/check-release-consistency.sh:301).
    local NORMALISED_VERSION="${VERSION#v}"

    local SECTION_FILE="" CLEANUP_SECTION=""
    if [ -n "${OUTPUT}" ]; then
        SECTION_FILE="${OUTPUT}"
    else
        SECTION_FILE="$(mktemp "${TMPDIR:-/tmp}/extract-changelog-section.XXXXXX")"
        CLEANUP_SECTION="${SECTION_FILE}"
    fi
    trap '[ -n "${CLEANUP_SECTION}" ] && rm -f "${CLEANUP_SECTION}"' RETURN

    local FOUND
    FOUND=$(python3 - "${CHANGELOG}" "${NORMALISED_VERSION}" "${SECTION_FILE}" <<'PY'
import re
import sys

changelog_path = sys.argv[1]
version = sys.argv[2]
out_path = sys.argv[3]

# newline="" on both the read and the write side: no universal-newline
# translation in either direction, so a CRLF source's line endings survive
# extraction unchanged -- part of the "no re-wrapping, no line-ending
# rewriting" contract.
with open(changelog_path, "r", encoding="utf-8", newline="") as fh:
    lines = fh.readlines()

# Mirrors check-release-consistency.sh's Clause 2 heading_re exactly
# (scripts/check-release-consistency.sh:401) -- the two implementations
# must never silently diverge on where a section starts.
heading_re = re.compile(r"^##\s*\[" + re.escape(version) + r"\](\s|$)")
any_heading_re = re.compile(r"^##\s*\[")

start_idx = None
for i, line in enumerate(lines):
    if heading_re.match(line):
        start_idx = i
        break

if start_idx is None:
    print("NOT_FOUND")
    sys.exit(0)

end_idx = len(lines)
for j in range(start_idx + 1, len(lines)):
    if any_heading_re.match(lines[j]):
        end_idx = j
        break

# Strictly between the two boundaries -- the heading line itself (start_idx)
# is never part of the output.
section_lines = lines[start_idx + 1:end_idx]

with open(out_path, "w", encoding="utf-8", newline="") as fh:
    fh.writelines(section_lines)

print("FOUND")
PY
)

    if [ "${FOUND}" != "FOUND" ]; then
        echo "::error::no ## [${NORMALISED_VERSION}] section in CHANGELOG.md -- run make release VERSION=${NORMALISED_VERSION} (finalizes changelogs) before tagging" >&2
        return 1
    fi

    if [ -n "${OUTPUT}" ]; then
        echo "section_file=${SECTION_FILE}"
        if [ -n "${GITHUB_OUTPUT:-}" ]; then
            echo "section_file=${SECTION_FILE}" >> "${GITHUB_OUTPUT}"
        fi
    else
        cat "${SECTION_FILE}"
    fi

    return 0
}

if [ "${EXTRACT_CHANGELOG_SECTION_LIB_ONLY:-0}" != "1" ]; then
    extract_changelog_section_main "$@"
fi
