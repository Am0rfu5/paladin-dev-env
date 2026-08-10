#!/usr/bin/env bash
# check-workflow-suppressions_test.sh
#
# Committed regression harness for scripts/check-workflow-suppressions.sh
# (IN-01, Phase 12 review). The guard's correctness was previously proven
# only with ephemeral `mktemp -d` fixtures built and torn down during plan
# execution -- nothing in the repo re-ran them, which is exactly how the two
# Critical bypasses it shipped with (CR-01: hyphenated `cargo-audit`/
# `cargo +toolchain audit` command-position variants; CR-02: quoted/tab
# `--ignore` flags) went undetected until manual review. This script pins
# those fixtures, plus the WR-03 `${{ }}` expression-indirection fix
# (CLAUSE3_UNRESOLVABLE_EXPRESSION), as a committed, re-runnable test.
#
# Every fixture is built under a single `mktemp -d` scratch directory that
# is removed on exit (trap, so it cleans up even on failure/interrupt). The
# real `.github/workflows/` tree is only ever read, never written -- the
# guard itself is read-only (see its own header), and this script never
# calls `git` in a way that could stage or modify anything there. The final
# assertion double-checks the real tree is untouched via
# `git status --porcelain -- .github/workflows/`.
#
# Fixtures accumulate into $FAILED rather than exiting on the first
# mismatch, matching the "report everything, don't short-circuit" house
# style the guard itself follows.
#
# Usage:  ./tests/scripts/check-workflow-suppressions_test.sh
#         Or via `make test-shell-guards`.
# Exit:   0 if every assertion passes; non-zero otherwise, with a report of
#         which assertion(s) failed.

set -uo pipefail

WORKSPACE_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
GUARD="${WORKSPACE_ROOT}/scripts/check-workflow-suppressions.sh"
REAL_WORKFLOWS_DIR="${WORKSPACE_ROOT}/.github/workflows"

if [ ! -f "${GUARD}" ]; then
    echo "ERROR: guard script not found at ${GUARD}" >&2
    exit 1
fi

SCRATCH="$(mktemp -d "${TMPDIR:-/tmp}/check-workflow-suppressions-test.XXXXXX")"
cleanup() {
    rm -rf "${SCRATCH}"
}
trap cleanup EXIT

FAILED=0
ASSERTIONS=0

# A bare, compliant `cargo audit` step used as a "companion" step in
# fixtures whose case-under-test line does not itself count toward clause
# 2's invocation counter (e.g. any `cargo deny`/`cargo-deny` case). Keeping
# clause 2 satisfied (exactly one invocation) isolates clause 1/clause 3 as
# the only possible source of a failure in those fixtures, so a clause 1/3
# regression is what actually gets caught rather than being masked by an
# unrelated clause 2 failure.
COMPANION_STEP='      - name: companion audit
        run: cargo audit'

# mkdir_fixture NAME -> echoes path to a fresh empty dir under $SCRATCH/NAME
mkdir_fixture() {
    local dir="${SCRATCH}/$1"
    mkdir -p "${dir}"
    echo "${dir}"
}

# write_workflow DIR STEPS_YAML -> writes a single workflow.yml into DIR
# whose one job's `steps:` block is exactly STEPS_YAML (already indented).
write_workflow() {
    local dir="$1" steps="$2"
    {
        echo "name: fixture"
        echo "on: push"
        echo "jobs:"
        echo "  fixture-job:"
        echo "    runs-on: ubuntu-latest"
        echo "    steps:"
        echo "${steps}"
    } > "${dir}/workflow.yml"
}

# run_guard DIR -> sets $LAST_OUTPUT and $LAST_STATUS
run_guard() {
    LAST_OUTPUT="$("${GUARD}" "$1" 2>&1)"
    LAST_STATUS=$?
}

# assert_fire DIR NEEDLE DESC -> expects non-zero exit AND $LAST_OUTPUT to
# contain NEEDLE (the specific named clause -- pins which clause fired, not
# just that something did).
assert_fire() {
    local dir="$1" needle="$2" desc="$3"
    ASSERTIONS=$((ASSERTIONS + 1))
    run_guard "${dir}"
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

# assert_silent DIR DESC -> expects zero exit.
assert_silent() {
    local dir="$1" desc="$2"
    ASSERTIONS=$((ASSERTIONS + 1))
    run_guard "${dir}"
    if [ "${LAST_STATUS}" -ne 0 ]; then
        echo "FAIL: expected zero exit (silent) for: ${desc} (got ${LAST_STATUS})"
        echo "${LAST_OUTPUT}" | sed 's/^/  | /'
        FAILED=$((FAILED + 1))
        return
    fi
    echo "PASS (silent): ${desc}"
}

CLAUSE1='CLAUSE1_INLINE_SUPPRESSION'
CLAUSE3='CLAUSE3_UNRESOLVABLE_EXPRESSION'

# --- Fire cases: known-bad command shapes that must be caught. -------------

# 1. Plain space-separated flag.
d="$(mkdir_fixture fire-01)"
write_workflow "${d}" '      - name: audit
        run: cargo audit --ignore RUSTSEC-2024-0001'
assert_fire "${d}" "${CLAUSE1}" "cargo audit --ignore X"

# 2. --ignore= form.
d="$(mkdir_fixture fire-02)"
write_workflow "${d}" '      - name: audit
        run: cargo audit --ignore=RUSTSEC-2024-0001'
assert_fire "${d}" "${CLAUSE1}" "cargo audit --ignore=X"

# 3. cargo deny (space-separated) -- needs companion for clause 2.
d="$(mkdir_fixture fire-03)"
write_workflow "${d}" "${COMPANION_STEP}"'
      - name: deny
        run: cargo deny check --ignore RUSTSEC-2024-0001'
assert_fire "${d}" "${CLAUSE1}" "cargo deny check --ignore X"

# 4. Hyphenated direct-binary form (CR-01 regression class).
d="$(mkdir_fixture fire-04)"
write_workflow "${d}" '      - name: audit
        run: cargo-audit --ignore RUSTSEC-2024-0001'
assert_fire "${d}" "${CLAUSE1}" "cargo-audit --ignore X"

# 5. Hyphenated cargo-deny -- needs companion for clause 2.
d="$(mkdir_fixture fire-05)"
write_workflow "${d}" "${COMPANION_STEP}"'
      - name: deny
        run: cargo-deny check --ignore RUSTSEC-2024-0001'
assert_fire "${d}" "${CLAUSE1}" "cargo-deny check --ignore X"

# 6. Toolchain-pinned invocation (CR-01 regression class).
d="$(mkdir_fixture fire-06)"
write_workflow "${d}" '      - name: audit
        run: cargo +nightly audit --ignore RUSTSEC-2024-0001'
assert_fire "${d}" "${CLAUSE1}" "cargo +nightly audit --ignore X"

# 7. Quoted flag (CR-02 regression class).
d="$(mkdir_fixture fire-07)"
write_workflow "${d}" '      - name: audit
        run: cargo audit "--ignore" RUSTSEC-2024-0001'
assert_fire "${d}" "${CLAUSE1}" 'cargo audit "--ignore" X'

# 8. Tab-separated flag (CR-02 regression class).
d="$(mkdir_fixture fire-08)"
printf 'name: fixture\non: push\njobs:\n  fixture-job:\n    runs-on: ubuntu-latest\n    steps:\n      - name: audit\n        run: "cargo audit\\t--ignore\\tRUSTSEC-2024-0001"\n' > "${d}/workflow.yml"
assert_fire "${d}" "${CLAUSE1}" "tab-separated cargo audit --ignore X"

# 9. Backslash-continuation split across two lines.
d="$(mkdir_fixture fire-09)"
write_workflow "${d}" '      - name: audit
        run: |
          cargo audit \
            --ignore RUSTSEC-2024-0001'
assert_fire "${d}" "${CLAUSE1}" "backslash-continuation split cargo audit --ignore X"

# 10. cargo install ... && cargo-audit --ignore X (install-then-invoke chain).
d="$(mkdir_fixture fire-10)"
write_workflow "${d}" '      - name: install-and-audit
        run: cargo install cargo-audit --locked && cargo-audit --ignore RUSTSEC-2024-0001'
assert_fire "${d}" "${CLAUSE1}" "cargo install ... && cargo-audit --ignore X"

# 11. cargo install ...; cargo-deny check --ignore X -- needs companion for clause 2.
d="$(mkdir_fixture fire-11)"
write_workflow "${d}" "${COMPANION_STEP}"'
      - name: install-and-deny
        run: cargo install cargo-deny --locked; cargo-deny check --ignore RUSTSEC-2024-0001'
assert_fire "${d}" "${CLAUSE1}" "cargo install ...; cargo-deny check --ignore X"

# 12. WR-03/change-1: ${{ }} expression indirection.
d="$(mkdir_fixture fire-12)"
write_workflow "${d}" '      - name: audit
        run: cargo audit ${{ vars.AUDIT_EXTRA_ARGS }}'
assert_fire "${d}" "${CLAUSE3}" 'cargo audit ${{ vars.AUDIT_EXTRA_ARGS }}'

# --- Silent cases: known-good shapes that must NOT be caught. --------------

# a. The real, unmodified .github/workflows/ tree.
assert_silent "${REAL_WORKFLOWS_DIR}" "real unmodified .github/workflows/ tree"

# b. cargo install cargo-audit --locked, alone (companion keeps clause 2 at 1).
d="$(mkdir_fixture silent-b)"
write_workflow "${d}" "${COMPANION_STEP}"'
      - name: install
        run: cargo install cargo-audit --locked'
assert_silent "${d}" "cargo install cargo-audit --locked alone"

# c. mc mb ... --ignore-existing.
d="$(mkdir_fixture silent-c)"
write_workflow "${d}" "${COMPANION_STEP}"'
      - name: minio
        run: mc mb myminio/mybucket --ignore-existing'
assert_silent "${d}" "mc mb ... --ignore-existing"

# d. cargo test -- --ignored --nocapture.
d="$(mkdir_fixture silent-d)"
write_workflow "${d}" "${COMPANION_STEP}"'
      - name: test
        run: cargo test -- --ignored --nocapture'
assert_silent "${d}" "cargo test -- --ignored --nocapture"

# e. Multi-line run: | block mixing (c) and (d).
d="$(mkdir_fixture silent-e)"
write_workflow "${d}" "${COMPANION_STEP}"'
      - name: mixed
        run: |
          mc mb myminio/mybucket --ignore-existing
          cargo test -- --ignored --nocapture'
assert_silent "${d}" "multi-line run: | block mixing mc mb --ignore-existing and cargo test -- --ignored"

# f. A ${{ }} expression elsewhere, on an unrelated run: line, must not trip
#    clause 3 -- co-occurrence on the SAME logical line is required.
d="$(mkdir_fixture silent-f)"
write_workflow "${d}" "${COMPANION_STEP}"'
      - name: unrelated
        run: echo "building ${{ github.sha }}"'
assert_silent "${d}" "unrelated \${{ }} expression on a non-cargo-audit/deny line"

# g. Differently-cased planted line must not be flagged (case-sensitivity,
#    documented in the guard's own header).
d="$(mkdir_fixture silent-g)"
write_workflow "${d}" "${COMPANION_STEP}"'
      - name: cased
        run: echo "Cargo Audit --Ignore RUSTSEC-2024-0001"'
assert_silent "${d}" "differently-cased planted line must not be flagged"

# --- Idempotency: two runs against the same input are byte-identical. ------
ASSERTIONS=$((ASSERTIONS + 1))
out1="$("${GUARD}" "${REAL_WORKFLOWS_DIR}" 2>&1)"
status1=$?
out2="$("${GUARD}" "${REAL_WORKFLOWS_DIR}" 2>&1)"
status2=$?
if [ "${out1}" = "${out2}" ] && [ "${status1}" -eq "${status2}" ]; then
    echo "PASS (idempotent): two runs against the real tree are byte-identical"
else
    echo "FAIL: two runs against the real tree were not byte-identical"
    FAILED=$((FAILED + 1))
fi

# --- The real tree must never be mutated by this test. ----------------------
ASSERTIONS=$((ASSERTIONS + 1))
git_status="$(cd "${WORKSPACE_ROOT}" && git status --porcelain -- .github/workflows/)"
if [ -z "${git_status}" ]; then
    echo "PASS (no mutation): git status --porcelain -- .github/workflows/ is empty"
else
    echo "FAIL: .github/workflows/ was mutated by this test run:"
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
