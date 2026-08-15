#!/usr/bin/env bash
# check-workflow-triggers_test.sh
#
# Committed regression harness for scripts/check-workflow-triggers.sh
# (D-09b, plan 15.1-09). Mirrors tests/scripts/check-workflow-suppressions_test.sh's
# fixture-lifecycle pattern exactly: every fixture is built under a single
# `mktemp -d` scratch directory removed on exit via a trap, the real tree is
# only ever read, and a closing assertion double-checks nothing real was
# mutated -- extended here to cover both `.github/workflows/` and
# `docs/src/contributing/branching-model.md`, since this guard reads both.
#
# Fixtures accumulate into $FAILED rather than exiting on the first
# mismatch, matching the "report everything, don't short-circuit" house
# style the guard itself follows.
#
# Usage:  ./tests/scripts/check-workflow-triggers_test.sh
#         Or via `make test-shell-guards`.
# Exit:   0 if every assertion passes; non-zero otherwise, with a report of
#         which assertion(s) failed.

set -uo pipefail

WORKSPACE_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
GUARD="${WORKSPACE_ROOT}/scripts/check-workflow-triggers.sh"
REAL_WORKFLOWS_DIR="${WORKSPACE_ROOT}/.github/workflows"
REAL_POLICY_TABLE="${WORKSPACE_ROOT}/docs/src/contributing/branching-model.md"
REAL_RULESET="${WORKSPACE_ROOT}/.github/rulesets/protect-main-branch.json"

if [ ! -f "${GUARD}" ]; then
    echo "ERROR: guard script not found at ${GUARD}" >&2
    exit 1
fi

SCRATCH="$(mktemp -d "${TMPDIR:-/tmp}/check-workflow-triggers-test.XXXXXX")"
cleanup() {
    rm -rf "${SCRATCH}"
}
trap cleanup EXIT

FAILED=0
ASSERTIONS=0

# A fixture ruleset with no `required_status_checks` rule at all -- clause 3
# then has zero pinned contexts to resolve, trivially passing, so a fixture
# testing clauses 1/2 is not incidentally failed by clause 3 having nothing
# real to match against.
EMPTY_RULESET="${SCRATCH}/empty-ruleset.json"
echo '{"rules": []}' > "${EMPTY_RULESET}"

# mkdir_fixture NAME -> echoes path to a fresh empty dir under $SCRATCH/NAME
mkdir_fixture() {
    local dir="${SCRATCH}/$1"
    mkdir -p "${dir}"
    echo "${dir}"
}

# write_workflow DIR FILENAME ON_YAML JOBS_YAML -> writes DIR/FILENAME with
# the given (pre-indented) `on:` block content and `jobs:` block content.
write_workflow() {
    local dir="$1" filename="$2" on_yaml="$3" jobs_yaml="$4"
    {
        echo "name: fixture"
        echo "on:"
        echo "${on_yaml}"
        echo "jobs:"
        echo "${jobs_yaml}"
    } > "${dir}/${filename}"
}

# write_policy_table FILE ROWS -> writes a minimal branching-model.md
# fixture with the standard 4-column table header/separator plus ROWS.
write_policy_table() {
    local file="$1" rows="$2"
    {
        echo "# Branching Model (fixture)"
        echo ""
        echo "| Workflow | Triggers | Push branch filter | Rationale |"
        echo "|----------|----------|---------------------|-----------|"
        echo "${rows}"
    } > "${file}"
}

# run_guard WF_DIR TABLE [RULESET] -> sets $LAST_OUTPUT and $LAST_STATUS
run_guard() {
    local wf_dir="$1" table="$2" ruleset="${3:-${EMPTY_RULESET}}"
    LAST_OUTPUT="$("${GUARD}" "${wf_dir}" "${table}" "${ruleset}" 2>&1)"
    LAST_STATUS=$?
}

# assert_fire WF_DIR TABLE RULESET NEEDLE DESC -> expects non-zero exit AND
# $LAST_OUTPUT to contain NEEDLE (pins which clause fired, not just that
# something did).
assert_fire() {
    local wf_dir="$1" table="$2" ruleset="$3" needle="$4" desc="$5"
    ASSERTIONS=$((ASSERTIONS + 1))
    run_guard "${wf_dir}" "${table}" "${ruleset}"
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

# assert_silent WF_DIR TABLE RULESET DESC -> expects zero exit.
assert_silent() {
    local wf_dir="$1" table="$2" ruleset="$3" desc="$4"
    ASSERTIONS=$((ASSERTIONS + 1))
    run_guard "${wf_dir}" "${table}" "${ruleset}"
    if [ "${LAST_STATUS}" -ne 0 ]; then
        echo "FAIL: expected zero exit (silent) for: ${desc} (got ${LAST_STATUS})"
        echo "${LAST_OUTPUT}" | sed 's/^/  | /'
        FAILED=$((FAILED + 1))
        return
    fi
    echo "PASS (silent): ${desc}"
}

CLAUSE_UNCOVERED='CLAUSE_UNCOVERED'
CLAUSE_DRIFT='CLAUSE_DRIFT'
CLAUSE_CONTEXT='CLAUSE_CONTEXT'
CLAUSE_REACHABILITY='CLAUSE_REACHABILITY'
ZERO_FILES='ZERO_FILES'

ON_STANDARD='  push:
    branches: ["**"]
  pull_request:
  workflow_dispatch:'
JOBS_STANDARD='  fixture-job:
    name: Fixture Job
    runs-on: ubuntu-latest
    steps:
      - run: true'
ROW_STANDARD='| `workflow.yml` | `push`, `pull_request`, `workflow_dispatch` | `['"'"'**'"'"']` | fixture row |'

# --- 1. Passing case: a single well-formed fixture workflow with a matching
#        register row and an empty ruleset (zero pinned contexts). ----------
d="$(mkdir_fixture pass-01)"
write_workflow "${d}" workflow.yml "${ON_STANDARD}" "${JOBS_STANDARD}"
t="${SCRATCH}/pass-01-table.md"
write_policy_table "${t}" "${ROW_STANDARD}"
assert_silent "${d}" "${t}" "${EMPTY_RULESET}" "well-formed fixture workflow with a matching register row"

# --- 2. Coverage clause: a second workflow file with no register row. ------
d="$(mkdir_fixture coverage-01)"
write_workflow "${d}" workflow.yml "${ON_STANDARD}" "${JOBS_STANDARD}"
write_workflow "${d}" unregistered.yml "${ON_STANDARD}" "${JOBS_STANDARD}"
t="${SCRATCH}/coverage-01-table.md"
write_policy_table "${t}" "${ROW_STANDARD}"
assert_fire "${d}" "${t}" "${EMPTY_RULESET}" "${CLAUSE_UNCOVERED}" "a workflow file with no register row"

# --- 3. Drift clause: a push branch filter narrowed away from match-all, on
#        a workflow that is not one of the two documented exceptions -- even
#        though the register row correctly records the same narrowed
#        filter, it is still a drift failure because the filter itself is
#        undocumented-exception drift, not merely a row/YAML disagreement. --
d="$(mkdir_fixture drift-01)"
write_workflow "${d}" other.yml \
'  push:
    branches: ["main"]
  pull_request:
  workflow_dispatch:' \
"${JOBS_STANDARD}"
t="${SCRATCH}/drift-01-table.md"
write_policy_table "${t}" '| `other.yml` | `push`, `pull_request`, `workflow_dispatch` | `[main]` | fixture row with a narrowed filter |'
assert_fire "${d}" "${t}" "${EMPTY_RULESET}" "${CLAUSE_DRIFT}" "a push branch filter narrowed away from match-all outside the two documented exceptions"

# --- 4. Empty workflows directory: a named ZERO_FILES failure, never a
#        silently-empty pass. ------------------------------------------------
d="$(mkdir_fixture empty-01)"
t="${SCRATCH}/empty-01-table.md"
write_policy_table "${t}" "${ROW_STANDARD}"
assert_fire "${d}" "${t}" "${EMPTY_RULESET}" "${ZERO_FILES}" "an empty workflows directory"

# --- 5. Context clause: a pinned required-status-check context that
#        resolves to no declared job name in any fixture workflow file. -----
d="$(mkdir_fixture context-01)"
write_workflow "${d}" workflow.yml "${ON_STANDARD}" "${JOBS_STANDARD}"
t="${SCRATCH}/context-01-table.md"
write_policy_table "${t}" "${ROW_STANDARD}"
context_ruleset="${SCRATCH}/context-01-ruleset.json"
cat > "${context_ruleset}" <<'JSON'
{
  "rules": [
    {
      "type": "required_status_checks",
      "parameters": {
        "required_status_checks": [
          {"context": "Nonexistent Check That No Job Declares"}
        ]
      }
    }
  ]
}
JSON
assert_fire "${d}" "${t}" "${context_ruleset}" "${CLAUSE_CONTEXT}" "a pinned required-status-check context resolving to no declared job name"

# --- 5a. Reachability clause: a pinned context whose owning workflow filters
#         its pull_request trigger by paths. The job name resolves fine, so
#         clause 3 stays silent -- but the check never REPORTS on a PR that
#         touches no matching path, and GitHub blocks that PR forever with no
#         failing check to explain it. Regression test for the PR #31 deadlock.
pinned_fixture_ruleset="${SCRATCH}/reach-ruleset.json"
cat > "${pinned_fixture_ruleset}" <<'JSON'
{
  "rules": [
    {
      "type": "required_status_checks",
      "parameters": {
        "required_status_checks": [
          {"context": "Fixture Job"}
        ]
      }
    }
  ]
}
JSON

d="$(mkdir_fixture reach-01)"
write_workflow "${d}" workflow.yml '  push:
    branches: ["**"]
  pull_request:
    paths:
      - "docs/**"
  workflow_dispatch:' "${JOBS_STANDARD}"
t="${SCRATCH}/reach-01-table.md"
write_policy_table "${t}" "${ROW_STANDARD}"
assert_fire "${d}" "${t}" "${pinned_fixture_ruleset}" "${CLAUSE_REACHABILITY}" "a pinned context whose workflow path-filters its pull_request trigger"

# --- 5b. Reachability clause: a pinned context in a workflow with no
#         pull_request trigger at all -- same deadlock, different cause. -----
d="$(mkdir_fixture reach-02)"
write_workflow "${d}" workflow.yml '  push:
    branches: ["**"]
  workflow_dispatch:' "${JOBS_STANDARD}"
t="${SCRATCH}/reach-02-table.md"
write_policy_table "${t}" '| `workflow.yml` | `push`, `workflow_dispatch` | `['"'"'**'"'"']` | fixture row |'
assert_fire "${d}" "${t}" "${pinned_fixture_ruleset}" "${CLAUSE_REACHABILITY}" "a pinned context in a workflow with no pull_request trigger"

# --- 5c. Reachability stays SILENT when the same pinned context lives in an
#         unfiltered pull_request workflow -- proves the clause is not just
#         firing on the presence of a pinned context. ----------------------
d="$(mkdir_fixture reach-03)"
write_workflow "${d}" workflow.yml "${ON_STANDARD}" "${JOBS_STANDARD}"
t="${SCRATCH}/reach-03-table.md"
write_policy_table "${t}" "${ROW_STANDARD}"
assert_silent "${d}" "${t}" "${pinned_fixture_ruleset}" "a pinned context whose workflow has an unfiltered pull_request trigger"

# --- 6. The real, unmodified tree passes clean. -----------------------------
assert_silent "${REAL_WORKFLOWS_DIR}" "${REAL_POLICY_TABLE}" "${REAL_RULESET}" "real unmodified .github/workflows/ tree and branching-model.md register"

# --- Idempotency: two runs against the same input are byte-identical. ------
ASSERTIONS=$((ASSERTIONS + 1))
out1="$("${GUARD}" "${REAL_WORKFLOWS_DIR}" "${REAL_POLICY_TABLE}" "${REAL_RULESET}" 2>&1)"
status1=$?
out2="$("${GUARD}" "${REAL_WORKFLOWS_DIR}" "${REAL_POLICY_TABLE}" "${REAL_RULESET}" 2>&1)"
status2=$?
if [ "${out1}" = "${out2}" ] && [ "${status1}" -eq "${status2}" ]; then
    echo "PASS (idempotent): two runs against the real tree are byte-identical"
else
    echo "FAIL: two runs against the real tree were not byte-identical"
    FAILED=$((FAILED + 1))
fi

# --- The real tree must never be mutated by this test: both the workflows
#     directory and the policy-table document this guard reads. ------------
ASSERTIONS=$((ASSERTIONS + 1))
git_status="$(cd "${WORKSPACE_ROOT}" && git status --porcelain -- .github/workflows/ docs/src/contributing/branching-model.md)"
if [ -z "${git_status}" ]; then
    echo "PASS (no mutation): git status --porcelain -- .github/workflows/ docs/src/contributing/branching-model.md is empty"
else
    echo "FAIL: .github/workflows/ or branching-model.md was mutated by this test run:"
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
