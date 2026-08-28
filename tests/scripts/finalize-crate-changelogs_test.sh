#!/usr/bin/env bash
# finalize-crate-changelogs_test.sh
#
# Committed regression harness for scripts/finalize-crate-changelogs.sh
# (PUBOPS-01, plan 20-04). Mirrors tests/scripts/check-release-consistency_test.sh's
# fixture-lifecycle pattern: every fixture is built under a single `mktemp -d`
# scratch directory removed on exit via a trap, the real tree
# (CHANGELOG.md, crates/) is only ever read by this test file itself (for the
# mutation-guard baseline), and a closing assertion double-checks the finalize
# script -- which does write files -- never touched the real tree, because
# every fixture's `manifest_path` resolves into the scratch directory instead.
#
# Fixtures accumulate into $FAILED rather than exiting on the first mismatch,
# matching the "report everything, don't short-circuit" house style the
# guard itself follows.
#
# Usage:  ./tests/scripts/finalize-crate-changelogs_test.sh
#         Or via `make test-shell-guards`.
# Exit:   0 if every assertion passes; non-zero otherwise, with a report of
#         which assertion(s) failed.

set -uo pipefail

WORKSPACE_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
GUARD="${WORKSPACE_ROOT}/scripts/finalize-crate-changelogs.sh"

if [ ! -f "${GUARD}" ]; then
    echo "ERROR: guard script not found at ${GUARD}" >&2
    exit 1
fi

SCRATCH="$(mktemp -d "${TMPDIR:-/tmp}/finalize-crate-changelogs-test.XXXXXX")"
cleanup() {
    rm -rf "${SCRATCH}"
}
trap cleanup EXIT

FAILED=0
ASSERTIONS=0

# --- Real-tree mutation baseline, captured before any fixture runs. --------
# finalize-crate-changelogs.sh WRITES files (unlike check-release-consistency.sh,
# which only reads), so this guard is the harness's most important safety net:
# every fixture below points manifest_path into the scratch tree, never the
# real crates/*/Cargo.toml or root Cargo.toml, so the real tree must be
# byte-for-byte unchanged after this whole file runs.
MUTATION_WATCH_PATHS=(Cargo.toml crates CHANGELOG.md .github/workflows)
BEFORE_STATUS="$(cd "${WORKSPACE_ROOT}" && git status --porcelain -- "${MUTATION_WATCH_PATHS[@]}")"

# write_metadata_fixture FILE PAIR... -> writes a minimal
# `cargo metadata --format-version 1`-shaped document containing only the
# `packages` key finalize-crate-changelogs.sh reads: name, manifest_path,
# publish. Each PAIR is `name=manifest_path` (publishable, `publish: null`)
# or `name=manifest_path:false` (a `publish = false` crate, reported by
# `cargo metadata` as `publish: []` -- the one exempt shape this script's
# ZERO_PACKAGES case exercises).
write_metadata_fixture() {
    local file="$1"
    shift
    local entries=() pair name manifest_path publish_json
    for pair in "$@"; do
        name="${pair%%=*}"
        manifest_path="${pair#*=}"
        if [[ "${manifest_path}" == *:false ]]; then
            manifest_path="${manifest_path%:false}"
            publish_json="[]"
        else
            publish_json="null"
        fi
        entries+=("{\"name\": \"${name}\", \"manifest_path\": \"${manifest_path}\", \"publish\": ${publish_json}}")
    done
    local joined
    joined="$(IFS=,; echo "${entries[*]:-}")"
    printf '{"packages": [%s]}\n' "${joined}" > "${file}"
}

# write_changelog FILE CONTENT -> writes CONTENT verbatim to FILE, creating
# FILE's parent directory first.
write_changelog() {
    local file="$1" content="$2"
    mkdir -p "$(dirname "${file}")"
    printf '%s' "${content}" > "${file}"
}

# run_guard ARGS... -> sets $LAST_OUTPUT and $LAST_STATUS.
run_guard() {
    LAST_OUTPUT="$("${GUARD}" "$@" 2>&1)"
    LAST_STATUS=$?
}

# assert_fire helper: DESC NEEDLE ARGS... -> expects non-zero exit AND
# $LAST_OUTPUT to contain NEEDLE (pins which status token/message fired, not
# just that something did).
assert_fire() {
    local desc="$1" needle="$2"
    shift 2
    ASSERTIONS=$((ASSERTIONS + 1))
    run_guard "$@"
    if [ "${LAST_STATUS}" -eq 0 ]; then
        echo "FAIL: expected non-zero exit for: ${desc} (got 0)"
        echo "${LAST_OUTPUT}" | sed 's/^/  | /'
        FAILED=$((FAILED + 1))
        return
    fi
    if ! grep -qF -- "${needle}" <<<"${LAST_OUTPUT}"; then
        echo "FAIL: expected output to contain '${needle}' for: ${desc}"
        echo "${LAST_OUTPUT}" | sed 's/^/  | /'
        FAILED=$((FAILED + 1))
        return
    fi
    echo "PASS (fire): ${desc}"
}

# assert_silent DESC ARGS... -> expects zero exit AND $LAST_OUTPUT to
# contain the OK status token.
assert_silent() {
    local desc="$1"
    shift
    ASSERTIONS=$((ASSERTIONS + 1))
    run_guard "$@"
    if [ "${LAST_STATUS}" -ne 0 ]; then
        echo "FAIL: expected zero exit (silent) for: ${desc} (got ${LAST_STATUS})"
        echo "${LAST_OUTPUT}" | sed 's/^/  | /'
        FAILED=$((FAILED + 1))
        return
    fi
    if ! grep -qF -- "OK" <<<"${LAST_OUTPUT}"; then
        echo "FAIL: expected output to contain 'OK' for: ${desc}"
        echo "${LAST_OUTPUT}" | sed 's/^/  | /'
        FAILED=$((FAILED + 1))
        return
    fi
    echo "PASS (silent): ${desc}"
}

# assert_eq DESC ACTUAL EXPECTED -> generic equality assertion.
assert_eq() {
    local desc="$1" actual="$2" expected="$3"
    ASSERTIONS=$((ASSERTIONS + 1))
    if [ "${actual}" = "${expected}" ]; then
        echo "PASS (eq): ${desc}"
    else
        echo "FAIL: ${desc} -- expected '${expected}', got '${actual}'"
        FAILED=$((FAILED + 1))
    fi
}

UNRELEASED_ONLY_CONTENT=$'# Changelog\n\nAll notable changes.\n\n## [Unreleased]\n\n### Added\n- Something.\n'
ALREADY_PRESENT_CONTENT=$'# Changelog\n\n## [Unreleased]\n\n## [1.2.3] - 2019-01-01\n\n### Added\n- old note.\n'
NO_ANCHOR_CONTENT=$'# Changelog\n\nNo unreleased heading here at all.\n'
OLDER_SECTION_CONTENT=$'# Changelog\n\n## [Unreleased]\n\n## [1.0.0] - 2025-01-01\n\n### Added\n- older.\n'

# =====================================================================
# 1/2. Two fixture packages gain a dated section immediately below
#      '## [Unreleased]', which survives; exit 0.
# =====================================================================
CASE12_DIR="${SCRATCH}/case12"
PKG_A="${CASE12_DIR}/pkg-a"
PKG_B="${CASE12_DIR}/pkg-b"
write_changelog "${PKG_A}/CHANGELOG.md" "${UNRELEASED_ONLY_CONTENT}"
write_changelog "${PKG_B}/CHANGELOG.md" "${UNRELEASED_ONLY_CONTENT}"
CASE12_METADATA="${CASE12_DIR}/metadata.json"
write_metadata_fixture "${CASE12_METADATA}" \
    "pkg-a=${PKG_A}/Cargo.toml" "pkg-b=${PKG_B}/Cargo.toml"

assert_silent "two fresh fixtures both gain a dated 1.2.3 section" \
    --version 1.2.3 --date 2026-01-02 --metadata-json "${CASE12_METADATA}"

ASSERTIONS=$((ASSERTIONS + 1))
if grep -qF -- "## [1.2.3] - 2026-01-02" "${PKG_A}/CHANGELOG.md" \
    && grep -qF -- "## [Unreleased]" "${PKG_A}/CHANGELOG.md"; then
    echo "PASS: pkg-a gained the dated section, Unreleased anchor survives"
else
    echo "FAIL: pkg-a's changelog does not contain both the anchor and the new section"
    FAILED=$((FAILED + 1))
fi

ASSERTIONS=$((ASSERTIONS + 1))
if grep -qF -- "## [1.2.3] - 2026-01-02" "${PKG_B}/CHANGELOG.md" \
    && grep -qF -- "## [Unreleased]" "${PKG_B}/CHANGELOG.md"; then
    echo "PASS: pkg-b gained the dated section, Unreleased anchor survives"
else
    echo "FAIL: pkg-b's changelog does not contain both the anchor and the new section"
    FAILED=$((FAILED + 1))
fi

# New section lands immediately after the anchor (reverse-chronological).
ASSERTIONS=$((ASSERTIONS + 1))
ANCHOR_LINE=$(grep -n -F -- "## [Unreleased]" "${PKG_A}/CHANGELOG.md" | head -n1 | cut -d: -f1)
NEW_SECTION_LINE=$(grep -n -F -- "## [1.2.3] - 2026-01-02" "${PKG_A}/CHANGELOG.md" | head -n1 | cut -d: -f1)
if [ "${NEW_SECTION_LINE}" -gt "${ANCHOR_LINE}" ]; then
    echo "PASS: new dated section is placed after the Unreleased anchor"
else
    echo "FAIL: new dated section is not after the Unreleased anchor (anchor=${ANCHOR_LINE}, new=${NEW_SECTION_LINE})"
    FAILED=$((FAILED + 1))
fi

# Snapshot both files after run 1, for the run-2 idempotency comparison below.
PKG_A_AFTER_RUN1="${SCRATCH}/pkg-a-after-run1.md"
PKG_B_AFTER_RUN1="${SCRATCH}/pkg-b-after-run1.md"
cp "${PKG_A}/CHANGELOG.md" "${PKG_A_AFTER_RUN1}"
cp "${PKG_B}/CHANGELOG.md" "${PKG_B_AFTER_RUN1}"

# =====================================================================
# 2 (continued). Running the identical invocation a second time changes
#      nothing; both files byte-identical to after run 1; both reported
#      already-finalized; exit 0.
# =====================================================================
assert_silent "second identical run reports already-finalized, exit 0" \
    --version 1.2.3 --date 2026-01-02 --metadata-json "${CASE12_METADATA}"

ASSERTIONS=$((ASSERTIONS + 1))
if grep -qF -- "already-finalized" <<<"${LAST_OUTPUT}"; then
    echo "PASS: second run's output names already-finalized"
else
    echo "FAIL: second run's output does not mention already-finalized"
    echo "${LAST_OUTPUT}" | sed 's/^/  | /'
    FAILED=$((FAILED + 1))
fi

ASSERTIONS=$((ASSERTIONS + 1))
if cmp -s "${PKG_A_AFTER_RUN1}" "${PKG_A}/CHANGELOG.md"; then
    echo "PASS (idempotent): pkg-a is byte-identical after run 1 and run 2 (cmp)"
else
    echo "FAIL: pkg-a's changelog changed between run 1 and run 2"
    FAILED=$((FAILED + 1))
fi

ASSERTIONS=$((ASSERTIONS + 1))
if cmp -s "${PKG_B_AFTER_RUN1}" "${PKG_B}/CHANGELOG.md"; then
    echo "PASS (idempotent): pkg-b is byte-identical after run 1 and run 2 (cmp)"
else
    echo "FAIL: pkg-b's changelog changed between run 1 and run 2"
    FAILED=$((FAILED + 1))
fi

# =====================================================================
# 3. A changelog already carrying '## [1.2.3] - 2019-01-01' (a different
#    date): left untouched, reported already-finalized, no second heading.
# =====================================================================
CASE3_DIR="${SCRATCH}/case3"
PKG_C="${CASE3_DIR}/pkg-c"
write_changelog "${PKG_C}/CHANGELOG.md" "${ALREADY_PRESENT_CONTENT}"
PKG_C_BEFORE="${SCRATCH}/pkg-c-before.md"
cp "${PKG_C}/CHANGELOG.md" "${PKG_C_BEFORE}"
CASE3_METADATA="${CASE3_DIR}/metadata.json"
write_metadata_fixture "${CASE3_METADATA}" "pkg-c=${PKG_C}/Cargo.toml"

assert_silent "existing section with a different date is left alone, reported already-finalized" \
    --version 1.2.3 --date 2026-01-02 --metadata-json "${CASE3_METADATA}"

ASSERTIONS=$((ASSERTIONS + 1))
if grep -qF -- "already-finalized" <<<"${LAST_OUTPUT}"; then
    echo "PASS: pkg-c (pre-existing section) reported already-finalized"
else
    echo "FAIL: pkg-c was not reported already-finalized"
    echo "${LAST_OUTPUT}" | sed 's/^/  | /'
    FAILED=$((FAILED + 1))
fi

ASSERTIONS=$((ASSERTIONS + 1))
if cmp -s "${PKG_C_BEFORE}" "${PKG_C}/CHANGELOG.md"; then
    echo "PASS: pkg-c's changelog bytes are unchanged (no duplicate heading written)"
else
    echo "FAIL: pkg-c's changelog was modified even though a section for the version already existed"
    FAILED=$((FAILED + 1))
fi

ASSERTIONS=$((ASSERTIONS + 1))
HEADING_COUNT=$(grep -c -- "^## \[1\.2\.3\]" "${PKG_C}/CHANGELOG.md")
assert_eq "pkg-c has exactly one heading line for 1.2.3 (no duplicate)" "${HEADING_COUNT}" "1"

# =====================================================================
# 4. A changelog with no '## [Unreleased]' anchor: named failure, that
#    file unmodified, exit non-zero, and the OTHER fixture package is
#    still processed (collect-then-report, not fail-fast).
# =====================================================================
CASE4_DIR="${SCRATCH}/case4"
PKG_GOOD="${CASE4_DIR}/pkg-good"
PKG_BAD="${CASE4_DIR}/pkg-bad-no-anchor"
write_changelog "${PKG_GOOD}/CHANGELOG.md" "${UNRELEASED_ONLY_CONTENT}"
write_changelog "${PKG_BAD}/CHANGELOG.md" "${NO_ANCHOR_CONTENT}"
PKG_BAD_BEFORE="${SCRATCH}/pkg-bad-before.md"
cp "${PKG_BAD}/CHANGELOG.md" "${PKG_BAD_BEFORE}"
CASE4_METADATA="${CASE4_DIR}/metadata.json"
write_metadata_fixture "${CASE4_METADATA}" \
    "pkg-good=${PKG_GOOD}/Cargo.toml" "pkg-bad-no-anchor=${PKG_BAD}/Cargo.toml"

assert_fire "missing-anchor file is a named failure" "pkg-bad-no-anchor" \
    --version 1.2.3 --date 2026-01-02 --metadata-json "${CASE4_METADATA}"

ASSERTIONS=$((ASSERTIONS + 1))
if cmp -s "${PKG_BAD_BEFORE}" "${PKG_BAD}/CHANGELOG.md"; then
    echo "PASS: the missing-anchor file's bytes are unchanged"
else
    echo "FAIL: the missing-anchor file was modified despite having no Unreleased anchor"
    FAILED=$((FAILED + 1))
fi

ASSERTIONS=$((ASSERTIONS + 1))
if grep -qF -- "## [1.2.3] - 2026-01-02" "${PKG_GOOD}/CHANGELOG.md"; then
    echo "PASS: the OTHER fixture package (pkg-good) was still processed despite pkg-bad's failure"
else
    echo "FAIL: pkg-good was not processed -- collect-then-report was not honored"
    FAILED=$((FAILED + 1))
fi

# =====================================================================
# 5. A fixture changelog file that does not exist: named failure, exit
#    non-zero.
# =====================================================================
CASE5_DIR="${SCRATCH}/case5"
PKG_MISSING="${CASE5_DIR}/pkg-missing-file"
mkdir -p "${PKG_MISSING}"
# Deliberately no CHANGELOG.md written here.
CASE5_METADATA="${CASE5_DIR}/metadata.json"
write_metadata_fixture "${CASE5_METADATA}" "pkg-missing-file=${PKG_MISSING}/Cargo.toml"

assert_fire "a changelog file that does not exist is a named failure" "pkg-missing-file" \
    --version 1.2.3 --date 2026-01-02 --metadata-json "${CASE5_METADATA}"

# =====================================================================
# 6. A metadata fixture whose only package is non-publishable: zero
#    publishable packages, named failure, exit non-zero.
# =====================================================================
CASE6_DIR="${SCRATCH}/case6"
PKG_EXEMPT="${CASE6_DIR}/pkg-exempt"
write_changelog "${PKG_EXEMPT}/CHANGELOG.md" "${UNRELEASED_ONLY_CONTENT}"
CASE6_METADATA="${CASE6_DIR}/metadata.json"
write_metadata_fixture "${CASE6_METADATA}" "pkg-exempt=${PKG_EXEMPT}/Cargo.toml:false"

assert_fire "the only package present is publish=false -> ZERO_PACKAGES, not OK" "ZERO_PACKAGES" \
    --version 1.2.3 --date 2026-01-02 --metadata-json "${CASE6_METADATA}"

# =====================================================================
# 7. A prerelease version such as 1.2.3-rc.4: stamped exactly like a
#    stable version, bracketed text matching the version string character
#    for character.
# =====================================================================
CASE7_DIR="${SCRATCH}/case7"
PKG_PRE="${CASE7_DIR}/pkg-prerelease"
write_changelog "${PKG_PRE}/CHANGELOG.md" "${UNRELEASED_ONLY_CONTENT}"
CASE7_METADATA="${CASE7_DIR}/metadata.json"
write_metadata_fixture "${CASE7_METADATA}" "pkg-prerelease=${PKG_PRE}/Cargo.toml"

assert_silent "prerelease version 1.2.3-rc.4 is stamped like a stable version" \
    --version 1.2.3-rc.4 --date 2026-01-02 --metadata-json "${CASE7_METADATA}"

ASSERTIONS=$((ASSERTIONS + 1))
if grep -qF -- "## [1.2.3-rc.4] - 2026-01-02" "${PKG_PRE}/CHANGELOG.md"; then
    echo "PASS: prerelease version string is stamped character-for-character"
else
    echo "FAIL: prerelease heading not found verbatim in pkg-prerelease's changelog"
    FAILED=$((FAILED + 1))
fi

# =====================================================================
# 8. A changelog with an existing older section below '## [Unreleased]':
#    the new section lands between them, so the newest release is first.
# =====================================================================
CASE8_DIR="${SCRATCH}/case8"
PKG_OLDER="${CASE8_DIR}/pkg-older-section"
write_changelog "${PKG_OLDER}/CHANGELOG.md" "${OLDER_SECTION_CONTENT}"
CASE8_METADATA="${CASE8_DIR}/metadata.json"
write_metadata_fixture "${CASE8_METADATA}" "pkg-older-section=${PKG_OLDER}/Cargo.toml"

assert_silent "new section lands between Unreleased and the existing older section" \
    --version 1.2.3 --date 2026-01-02 --metadata-json "${CASE8_METADATA}"

ASSERTIONS=$((ASSERTIONS + 1))
ANCHOR_L=$(grep -n -F -- "## [Unreleased]" "${PKG_OLDER}/CHANGELOG.md" | head -n1 | cut -d: -f1)
NEW_L=$(grep -n -F -- "## [1.2.3] - 2026-01-02" "${PKG_OLDER}/CHANGELOG.md" | head -n1 | cut -d: -f1)
OLD_L=$(grep -n -F -- "## [1.0.0] - 2025-01-01" "${PKG_OLDER}/CHANGELOG.md" | head -n1 | cut -d: -f1)
if [ "${ANCHOR_L}" -lt "${NEW_L}" ] && [ "${NEW_L}" -lt "${OLD_L}" ]; then
    echo "PASS: heading order is Unreleased < 1.2.3 (new) < 1.0.0 (older) -- newest first"
else
    echo "FAIL: heading order is wrong (anchor=${ANCHOR_L}, new=${NEW_L}, older=${OLD_L})"
    FAILED=$((FAILED + 1))
fi

# =====================================================================
# 9. No --version at all: exit non-zero, MISSING_VERSION.
# =====================================================================
assert_fire "no --version at all is a named MISSING_VERSION failure" "MISSING_VERSION" \
    --metadata-json "${CASE12_METADATA}"

# =====================================================================
# 10. An unknown flag is a usage error (non-zero), not a silent no-op.
# =====================================================================
assert_fire "an unknown flag is a usage error" "unknown flag" \
    --bogus-flag foo --version 1.2.3 --metadata-json "${CASE12_METADATA}"

# --- The real tree must never be mutated by this test, even though the
#     guard under test WRITES files (Cargo.toml, crates/, CHANGELOG.md and
#     .github/workflows/). ------------------------------------------------
ASSERTIONS=$((ASSERTIONS + 1))
AFTER_STATUS="$(cd "${WORKSPACE_ROOT}" && git status --porcelain -- "${MUTATION_WATCH_PATHS[@]}")"
if [ "${BEFORE_STATUS}" = "${AFTER_STATUS}" ]; then
    echo "PASS (no mutation): git status --porcelain -- Cargo.toml crates CHANGELOG.md .github/workflows is unchanged"
else
    echo "FAIL: Cargo.toml, crates/, CHANGELOG.md or .github/workflows/ was mutated by this test run:"
    echo "before: ${BEFORE_STATUS}" | sed 's/^/  | /'
    echo "after:  ${AFTER_STATUS}" | sed 's/^/  | /'
    FAILED=$((FAILED + 1))
fi

echo ""
if [ "${FAILED}" -eq 0 ]; then
    echo "✅ ${ASSERTIONS} assertion(s) passed."
    exit 0
else
    echo "❌ ${FAILED}/${ASSERTIONS} assertion(s) failed."
    exit 1
fi
