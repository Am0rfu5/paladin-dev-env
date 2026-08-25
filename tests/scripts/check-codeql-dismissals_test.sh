#!/usr/bin/env bash
# check-codeql-dismissals_test.sh
#
# Committed regression harness for scripts/check-codeql-dismissals.sh
# (18-04). Mirrors tests/scripts/check-workflow-triggers_test.sh's
# fixture-lifecycle pattern: every fixture is built under a single
# `mktemp -d` scratch directory removed on exit via a trap, the real tree
# is only ever read, and a closing assertion double-checks nothing real
# was mutated.
#
# This proves the guard fails FIRST, not merely that it passes: five
# distinct malformed-register cases are each asserted to produce a
# non-zero exit and a named rejection message, before either passing case
# (a well-formed populated register, and a well-formed empty one) is
# treated as evidence the guard works at all.
#
# Fixtures accumulate into $FAILED rather than exiting on the first
# mismatch, matching the "report everything, don't short-circuit" house
# style the guard itself follows.
#
# Usage:  ./tests/scripts/check-codeql-dismissals_test.sh
#         Or via `make test-shell-guards`.
# Exit:   0 if every assertion passes; non-zero otherwise, with a report of
#         which assertion(s) failed.

set -uo pipefail

WORKSPACE_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
GUARD="${WORKSPACE_ROOT}/scripts/check-codeql-dismissals.sh"
REAL_REGISTER="${WORKSPACE_ROOT}/CODEQL-DISMISSALS.md"

if [ ! -f "${GUARD}" ]; then
    echo "ERROR: guard script not found at ${GUARD}" >&2
    exit 1
fi

SCRATCH="$(mktemp -d "${TMPDIR:-/tmp}/check-codeql-dismissals-test.XXXXXX")"
cleanup() {
    rm -rf "${SCRATCH}"
}
trap cleanup EXIT

FAILED=0
ASSERTIONS=0

# A future review_date safe against this test's own runtime -- pinned far
# enough out that it never accidentally passes the staleness clause.
FUTURE_DATE="2099-12-31"
PAST_DATE="2000-01-01"

# A file guaranteed to exist under WORKSPACE_ROOT, used as a valid `path`
# value so the reachability clause does not incidentally fire in fixtures
# that are not testing it.
REACHABLE_PATH="Makefile"

# write_register FILE DECLARED ENTRIES_TOML -> writes a minimal, well-formed
# CODEQL-DISMISSALS.md-shaped fixture with the given declared count and raw
# TOML entries block.
write_register() {
    local file="$1" declared="$2" entries="$3"
    {
        echo "# CodeQL Alert Dismissal Register (fixture)"
        echo ""
        echo "Declared dismissals: ${declared}"
        echo ""
        echo "<!-- BEGIN MACHINE-READABLE REGISTER -->"
        echo '```toml'
        echo "${entries}"
        echo '```'
        echo "<!-- END MACHINE-READABLE REGISTER -->"
    } > "${file}"
}

# one_entry ALERT_NUM PATH REVIEW_DATE -> a single well-formed [[dismissal]]
# TOML block with all eleven fields populated.
one_entry() {
    local alert_num="$1" path="$2" review_date="$3"
    cat <<TOML
[[dismissal]]
alert_number = ${alert_num}
rule_id = "rust/hard-coded-cryptographic-value"
path = "${path}"
why_present = "fixture why_present"
why_dismissed = "fixture why_dismissed"
dismissed_reason = "used in tests"
owner = "DF3NDR"
review_date = "${review_date}"
scope = "fixture scope"
compensating_control = "fixture compensating_control"
revisit_condition = "fixture revisit_condition"
TOML
}

# run_guard REGISTER [REPO_ROOT] -> sets $LAST_OUTPUT and $LAST_STATUS
run_guard() {
    local register="$1" repo_root="${2:-${WORKSPACE_ROOT}}"
    LAST_OUTPUT="$("${GUARD}" "${register}" "${repo_root}" 2>&1)"
    LAST_STATUS=$?
}

# assert_fire REGISTER REPO_ROOT NEEDLE DESC -> expects non-zero exit AND
# $LAST_OUTPUT to contain NEEDLE (pins which clause fired, not just that
# something did).
assert_fire() {
    local register="$1" repo_root="$2" needle="$3" desc="$4"
    ASSERTIONS=$((ASSERTIONS + 1))
    run_guard "${register}" "${repo_root}"
    if [ "${LAST_STATUS}" -eq 0 ]; then
        echo "FAIL: expected non-zero exit for: ${desc} (got 0)"
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

# assert_silent REGISTER REPO_ROOT DESC -> expects zero exit.
assert_silent() {
    local register="$1" repo_root="$2" desc="$3"
    ASSERTIONS=$((ASSERTIONS + 1))
    run_guard "${register}" "${repo_root}"
    if [ "${LAST_STATUS}" -ne 0 ]; then
        echo "FAIL: expected zero exit (silent) for: ${desc} (got ${LAST_STATUS})"
        echo "${LAST_OUTPUT}" | sed 's/^/  | /'
        FAILED=$((FAILED + 1))
        return
    fi
    echo "PASS (silent): ${desc}"
}

CLAUSE_SCHEMA='CLAUSE_SCHEMA'
CLAUSE_DRIFT='CLAUSE_DRIFT'
CLAUSE_STALENESS='CLAUSE_STALENESS'
CLAUSE_UNIQUENESS='CLAUSE_UNIQUENESS'
CLAUSE_REACHABILITY='CLAUSE_REACHABILITY'
MISSING_REGISTER='MISSING_REGISTER'

# --- 1. Missing field: drop `owner` from an otherwise well-formed entry. ---
f="${SCRATCH}/missing-field.md"
entry="$(one_entry 1 "${REACHABLE_PATH}" "${FUTURE_DATE}")"
entry="$(grep -v '^owner = ' <<<"${entry}")"
write_register "${f}" 1 "${entry}"
assert_fire "${f}" "${WORKSPACE_ROOT}" "${CLAUSE_SCHEMA}" "an entry missing the owner field"

# --- 2. Past review_date. ---
f="${SCRATCH}/past-review-date.md"
write_register "${f}" 1 "$(one_entry 2 "${REACHABLE_PATH}" "${PAST_DATE}")"
assert_fire "${f}" "${WORKSPACE_ROOT}" "${CLAUSE_STALENESS}" "a review_date already in the past"

# --- 3. Declared-count mismatch: header says 2, payload holds 1. ---
f="${SCRATCH}/count-mismatch.md"
write_register "${f}" 2 "$(one_entry 3 "${REACHABLE_PATH}" "${FUTURE_DATE}")"
assert_fire "${f}" "${WORKSPACE_ROOT}" "${CLAUSE_DRIFT}" "a declared count that disagrees with the entry count"

# --- 4. Duplicate alert_number across two otherwise well-formed entries. ---
f="${SCRATCH}/duplicate-alert.md"
e1="$(one_entry 4 "${REACHABLE_PATH}" "${FUTURE_DATE}")"
e2="$(one_entry 4 "${REACHABLE_PATH}" "${FUTURE_DATE}")"
write_register "${f}" 2 "$(printf '%s\n\n%s\n' "${e1}" "${e2}")"
assert_fire "${f}" "${WORKSPACE_ROOT}" "${CLAUSE_UNIQUENESS}" "a duplicate alert_number across two entries"

# --- 5. Missing register file entirely. ---
assert_fire "${SCRATCH}/does-not-exist.md" "${WORKSPACE_ROOT}" "${MISSING_REGISTER}" "a register file that does not exist"

# --- 5a. Bonus: reachability clause fires on a path that does not exist and
#         whose scope does not record removal. --------------------------------
f="${SCRATCH}/unreachable-path.md"
write_register "${f}" 1 "$(one_entry 5 "src/does/not/exist.rs" "${FUTURE_DATE}")"
assert_fire "${f}" "${WORKSPACE_ROOT}" "${CLAUSE_REACHABILITY}" "a path that does not exist and whose scope does not record removal"

# --- 6. Passing case: a well-formed populated register. -----------------------
f="${SCRATCH}/well-formed.md"
write_register "${f}" 1 "$(one_entry 6 "${REACHABLE_PATH}" "${FUTURE_DATE}")"
assert_silent "${f}" "${WORKSPACE_ROOT}" "a well-formed populated register"

# --- 7. Passing case: a well-formed EMPTY register (declared 0, zero rows). ---
f="${SCRATCH}/well-formed-empty.md"
write_register "${f}" 0 ""
assert_silent "${f}" "${WORKSPACE_ROOT}" "a well-formed empty register with declared count 0"

# --- 8. The real, unmodified register passes clean. ---------------------------
assert_silent "${REAL_REGISTER}" "${WORKSPACE_ROOT}" "real unmodified CODEQL-DISMISSALS.md"

# --- Idempotency: two runs against the same input are byte-identical. --------
ASSERTIONS=$((ASSERTIONS + 1))
out1="$("${GUARD}" "${REAL_REGISTER}" "${WORKSPACE_ROOT}" 2>&1)"
status1=$?
out2="$("${GUARD}" "${REAL_REGISTER}" "${WORKSPACE_ROOT}" 2>&1)"
status2=$?
if [ "${out1}" = "${out2}" ] && [ "${status1}" -eq "${status2}" ]; then
    echo "PASS (idempotent): two runs against the real register are byte-identical"
else
    echo "FAIL: two runs against the real register were not byte-identical"
    FAILED=$((FAILED + 1))
fi

# --- The real tree must never be mutated by this test. ------------------------
ASSERTIONS=$((ASSERTIONS + 1))
git_status="$(cd "${WORKSPACE_ROOT}" && git status --porcelain -- CODEQL-DISMISSALS.md)"
if [ -z "${git_status}" ]; then
    echo "PASS (no mutation): git status --porcelain -- CODEQL-DISMISSALS.md is empty"
else
    echo "FAIL: CODEQL-DISMISSALS.md was mutated by this test run:"
    echo "${git_status}" | sed 's/^/  | /'
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
