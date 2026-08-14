#!/usr/bin/env bash
# check-workflow-triggers.sh
#
# The D-09b regression guard: fails if a workflow's `on:` trigger surface
# drifts from the recorded trigger-policy table in
# docs/src/contributing/branching-model.md -- a workflow file with no table
# row, a row whose recorded trigger types or push branch filter disagrees
# with what the YAML actually declares, or a push branch filter narrowed
# away from the match-all pattern outside the two documented exceptions.
# `check-workflow-suppressions.sh` already proved this register-plus-guard
# shape for advisory-suppression governance (ADR-0036); this script applies
# the identical discipline to trigger-surface governance.
#
# This script is offline and makes no network call. It parses workflow YAML
# structurally with PyYAML -- it never scrapes raw file text to decide what
# a workflow's trigger surface is -- and reports every violation it finds
# rather than stopping at the first. It only reads: it writes nothing and
# creates no temporary file, so running it twice in succession, with no
# change to any input, produces identical output and the same exit code.
#
# Three clauses are asserted, all accumulated into one shared failure list
# before the verdict is decided:
#
#   1. Coverage. Every workflow file discovered under the workflows
#      directory has a row in the trigger-policy table. A new workflow with
#      no entry is the primary case this guard exists to catch, since
#      nothing else in the repository would notice it -- the workflow
#      linter validates syntax, not intent.
#   2. Drift. For each workflow with a row: the trigger types the row
#      records (matched by searching the row's "Triggers" cell for the
#      literal words `push`, `pull_request`, `workflow_dispatch` and
#      `schedule`, so surrounding prose such as "(tags only)" does not
#      break the match) must equal the trigger keys the YAML actually
#      declares; and the row's recorded push branch filter must equal the
#      YAML's actual `on.push.branches` list. Separately, any workflow whose
#      actual push branch filter is a list other than the match-all pattern
#      (`['**']`) and which is not one of the two documented exceptions
#      (`docs.yml`, `release.yml`) is its own drift failure -- this is the
#      literal reintroduced-branch-filter case D-09b names as the guard's
#      reason to exist.
#   3. Context resolution. Every required-status-check context pinned in
#      the trunk ruleset (`.github/rulesets/protect-main-branch.json`)
#      resolves to a job display name declared in some workflow file. A job
#      whose declared `name:` contains a `${{ ... }}` expression, or whose
#      job carries a `strategy.matrix` block, is matched as a *prefix* --
#      the literal text up to the first `${{` (or the whole literal name,
#      for a matrix job with no expression in its name, since GitHub Actions
#      appends the matrix leg value in parentheses automatically) -- against
#      each pinned context, because a matrix job's concrete display names
#      only exist once its legs expand at run time. This closes the
#      residual risk in pinning contexts by display name -- which is the
#      only thing the platform matches on -- namely that renaming a job
#      silently drops its gate. Skipped, with a clear message, if the
#      ruleset file is absent, so the guard stays usable in a checkout that
#      has not applied protection.
#
# Table parsing: the policy table lives in a Markdown file, not a fenced
# TOML block, so this script reads it with a small line-based reader --
# lines beginning with the table delimiter are split on that delimiter, each
# cell stripped, and the header/separator rows skipped. This is the one
# piece with no existing local analog; it is deliberately small and
# tolerant of surrounding prose, and fails loudly if it finds no data rows
# at all.
#
# A workflows directory matching zero `*.yml`/`*.yaml` files is a named
# non-zero failure -- a silently-empty comparison that reports success over
# nothing is its own defect, exactly as `check-workflow-suppressions.sh` and
# `check-advisory-register.sh` treat a missing/empty input. A workflow file
# that fails to parse as YAML, or a policy table that parses to zero data
# rows, is also a named failure, never a silent skip.
#
# Usage:  ./scripts/check-workflow-triggers.sh [workflows-dir] [policy-table-file] [ruleset-file]
#         The optional positional arguments override, in order, the scanned
#         workflows directory (default: <repo-root>/.github/workflows), the
#         policy-table document (default:
#         <repo-root>/docs/src/contributing/branching-model.md), and the
#         trunk ruleset JSON the context clause validates against (default:
#         <repo-root>/.github/rulesets/protect-main-branch.json). They exist
#         so this guard's own regression test can point it at fixtures
#         without ever mutating the real tree.
# Exit:   0 if every workflow file has a table row, every row's recorded
#         trigger surface matches the YAML, no push branch filter has
#         drifted away from the match-all pattern outside the two
#         documented exceptions, and every pinned required-check context
#         resolves to a declared job name (or the ruleset file is absent);
#         non-zero otherwise.

set -euo pipefail

WORKSPACE_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
WORKFLOWS_DIR="${1:-${WORKSPACE_ROOT}/.github/workflows}"
POLICY_TABLE="${2:-${WORKSPACE_ROOT}/docs/src/contributing/branching-model.md}"
RULESET_FILE="${3:-${WORKSPACE_ROOT}/.github/rulesets/protect-main-branch.json}"

if ! command -v python3 >/dev/null 2>&1; then
    echo "ERROR: python3 is required for YAML/JSON parsing." >&2
    exit 1
fi

echo "🔍 Checking workflow trigger surfaces against the recorded policy table ..."

REPORT=$(python3 - "${WORKFLOWS_DIR}" "${POLICY_TABLE}" "${RULESET_FILE}" <<'PY'
import glob
import json
import os
import re
import sys

import yaml

workflows_dir = sys.argv[1]
policy_table_path = sys.argv[2]
ruleset_path = sys.argv[3]

failures = []

MATCH_ALL = ['**']
EXCEPTION_FILES = {'docs.yml', 'release.yml'}
TRIGGER_WORD_RE = re.compile(r'\b(push|pull_request|workflow_dispatch|schedule)\b')
BRACKET_RE = re.compile(r'\[([^\]]*)\]')

files = sorted(
    set(glob.glob(os.path.join(workflows_dir, '*.yml')))
    | set(glob.glob(os.path.join(workflows_dir, '*.yaml')))
)

if not files:
    print('ZERO_FILES')
    print(f'FAIL: no *.yml or *.yaml files found under {workflows_dir} -- a '
          'broken glob or an empty workflows directory is a named failure, '
          'never a silently-empty pass that reports success over nothing.')
    sys.exit(0)


def get_on_block(data):
    """PyYAML's default (YAML 1.1) resolver treats a bare `on` mapping key
    as the boolean True, not the string 'on' -- this repo's workflow files
    all key their trigger block as `on:`, so it round-trips through PyYAML
    as `data[True]`. Handle both so a fixture written either way parses the
    same."""
    on = data.get(True, data.get('on'))
    if on is None:
        return {}
    if isinstance(on, str):
        return {on: None}
    if isinstance(on, list):
        return {str(k): None for k in on}
    if isinstance(on, dict):
        return on
    return {}


def yaml_trigger_types(on_block):
    return set(str(k) for k in on_block.keys())


def yaml_push_branches(on_block):
    """Returns one of: None (no push trigger at all), 'TAGS_ONLY' (a push
    trigger that fires on tags, not branches -- release.yml's shape), or a
    list of branch patterns (possibly the bare-unrestricted match-all
    equivalent, for a `push:` value with no `branches:` and no `tags:` sub-key)."""
    if 'push' not in on_block:
        return None
    push_val = on_block['push']
    if not isinstance(push_val, dict):
        # A bare `push:` (e.g. `on: push`) fires on every branch push.
        return list(MATCH_ALL)
    if 'branches' in push_val:
        return list(push_val['branches'] or [])
    if 'tags' in push_val:
        return 'TAGS_ONLY'
    return list(MATCH_ALL)


# --- Load and structurally parse every workflow file. -----------------------
workflow_data = {}
for path in files:
    basename = os.path.basename(path)
    try:
        with open(path, 'r', encoding='utf-8') as fh:
            data = yaml.safe_load(fh)
    except yaml.YAMLError as exc:
        failures.append(f'PARSE_ERROR: {path} did not parse as YAML: {exc}')
        continue
    if not isinstance(data, dict):
        failures.append(f'PARSE_ERROR: {path} did not parse to a mapping.')
        continue
    workflow_data[basename] = data

# --- Parse the policy table: a lightweight line-based Markdown reader. ------
# Take every line beginning with the table delimiter, split on it, strip
# each cell. Skip the header row (first) and the separator row (a row whose
# cells are made up only of dashes/colons). No existing local pattern parses
# a Markdown table structurally, so this is the one genuinely new piece.
try:
    with open(policy_table_path, 'r', encoding='utf-8') as fh:
        table_text = fh.read()
except OSError as exc:
    failures.append(f'MISSING_POLICY_TABLE: could not read {policy_table_path}: {exc}')
    table_text = ''

table_lines = [line.strip() for line in table_text.splitlines() if line.strip().startswith('|')]


def is_separator_row(cells):
    return all(re.fullmatch(r':?-+:?', c.strip()) for c in cells if c.strip() != '') and bool(cells)


rows = []
for line in table_lines:
    cells = [c.strip() for c in line.strip('|').split('|')]
    if is_separator_row(cells):
        continue
    rows.append(cells)

# The first surviving row (after the separator is dropped) is the header;
# every row after it is a data row.
data_rows = rows[1:] if rows else []

if not data_rows:
    print('EMPTY_POLICY_TABLE')
    print(f'FAIL: {policy_table_path} contains no trigger-policy table data '
          'rows -- an unparsed or emptied table is a named failure, never a '
          'silently-empty pass.')
    sys.exit(0)


def parse_workflow_cell(cell):
    return cell.strip('`').strip()


def parse_branch_cell(cell):
    """Returns 'NOT_APPLICABLE' if the cell records that no push branch
    filter applies, a list of branch patterns if the cell names one, or None
    if the cell could not be parsed at all (its own drift signal)."""
    if 'not applicable' in cell.lower():
        return 'NOT_APPLICABLE'
    m = BRACKET_RE.search(cell)
    if not m:
        return None
    inner = m.group(1)
    items = [item.strip().strip("'\"") for item in inner.split(',') if item.strip()]
    return items


register = {}
for cells in data_rows:
    if len(cells) < 4:
        failures.append(f'MALFORMED_ROW: policy table row does not have 4 cells: {cells!r}')
        continue
    workflow_cell, triggers_cell, branches_cell, rationale_cell = cells[0], cells[1], cells[2], cells[3]
    wf_name = parse_workflow_cell(workflow_cell)
    if not wf_name:
        continue
    register[wf_name] = {
        'triggers': set(TRIGGER_WORD_RE.findall(triggers_cell)),
        'branches': parse_branch_cell(branches_cell),
        'raw_branches_cell': branches_cell,
    }

# --- Clause 1: coverage. Every discovered workflow has a register row. -----
coverage_failures = []
for basename in sorted(workflow_data.keys()):
    if basename not in register:
        coverage_failures.append(
            f'CLAUSE_UNCOVERED: {basename} was discovered under {workflows_dir} but has '
            f'no row in the trigger-policy table at {policy_table_path}.'
        )
failures.extend(coverage_failures)

# --- Clause 2: drift. Recorded rows must match the YAML, and no push filter
# may have narrowed away from the match-all pattern outside the two
# documented exceptions. -----------------------------------------------------
drift_failures = []
for basename, data in sorted(workflow_data.items()):
    if basename not in register:
        continue  # already reported as CLAUSE_UNCOVERED above
    row = register[basename]
    on_block = get_on_block(data)

    yaml_triggers = yaml_trigger_types(on_block)
    if row['triggers'] != yaml_triggers:
        drift_failures.append(
            f'CLAUSE_DRIFT: {basename} declares trigger types {sorted(yaml_triggers)} '
            f'but the table row records {sorted(row["triggers"])}.'
        )

    yaml_branches = yaml_push_branches(on_block)
    doc_branches = row['branches']

    if yaml_branches is None or yaml_branches == 'TAGS_ONLY':
        if doc_branches != 'NOT_APPLICABLE':
            drift_failures.append(
                f'CLAUSE_DRIFT: {basename} declares no push-to-branch trigger (push is '
                f'absent or tag-only) but the table row records a branch filter of '
                f'{row["raw_branches_cell"]!r} instead of "not applicable".'
            )
    else:
        if doc_branches is None:
            drift_failures.append(
                f'CLAUSE_DRIFT: {basename}\'s table row branch-filter cell '
                f'{row["raw_branches_cell"]!r} could not be parsed as a bracketed list.'
            )
        elif set(doc_branches) != set(yaml_branches):
            drift_failures.append(
                f'CLAUSE_DRIFT: {basename} declares push branch filter {yaml_branches!r} '
                f'but the table row records {doc_branches!r}.'
            )
        if list(yaml_branches) != MATCH_ALL and basename not in EXCEPTION_FILES:
            drift_failures.append(
                f'CLAUSE_DRIFT: {basename} declares push branch filter {yaml_branches!r}, '
                f'narrowed away from the match-all pattern {MATCH_ALL!r}, and is not one '
                f'of the two documented exceptions (docs.yml, release.yml) -- this is the '
                f'reintroduced-branch-filter case this guard exists to catch.'
            )
failures.extend(drift_failures)

# --- Clause 3: every pinned required-status-check context resolves to a
# declared job name (exact match, or prefix match for a matrix job). --------
context_failures = []
if not os.path.isfile(ruleset_path):
    print(f'CONTEXT_CLAUSE_SKIPPED: {ruleset_path} is absent -- skipping the pinned-context '
          f'resolution clause so this guard stays usable in a checkout that has not applied '
          f'branch protection.', file=sys.stderr)
else:
    try:
        with open(ruleset_path, 'r', encoding='utf-8') as fh:
            ruleset = json.load(fh)
    except (OSError, json.JSONDecodeError) as exc:
        failures.append(f'RULESET_PARSE_ERROR: could not read/parse {ruleset_path}: {exc}')
        ruleset = None

    if ruleset is not None:
        pinned_contexts = []
        for rule in ruleset.get('rules', []):
            if rule.get('type') == 'required_status_checks':
                for check in rule.get('parameters', {}).get('required_status_checks', []):
                    ctx = check.get('context')
                    if ctx:
                        pinned_contexts.append(ctx)

        exact_names = set()
        prefixes = []
        for basename, data in workflow_data.items():
            jobs = data.get('jobs')
            if not isinstance(jobs, dict):
                continue
            for job_id, job in jobs.items():
                if not isinstance(job, dict):
                    continue
                name = job.get('name', job_id)
                has_matrix = (
                    isinstance(job.get('strategy'), dict)
                    and 'matrix' in job['strategy']
                )
                if '${{' in name:
                    prefixes.append(name.split('${{', 1)[0])
                elif has_matrix:
                    prefixes.append(name)
                else:
                    exact_names.add(name)

        for ctx in pinned_contexts:
            if ctx in exact_names:
                continue
            if any(ctx.startswith(p) for p in prefixes):
                continue
            context_failures.append(
                f'CLAUSE_CONTEXT: required-status-check context {ctx!r} pinned in '
                f'{ruleset_path} resolves to no declared job name (exact or matrix-prefix) '
                f'in any workflow file under {workflows_dir} -- a job rename may have '
                f'silently dropped this gate.'
            )
        failures.extend(context_failures)

if failures:
    print('FAIL')
    for f in failures:
        print(f'FAIL: {f}')
    sys.exit(0)

print('OK')
print(f'{len(workflow_data)} workflow file(s) scanned, {len(register)} policy-table row(s) '
      f'read; coverage, drift and context clauses all pass.')
sys.exit(0)
PY
)

STATUS_LINE=$(head -n1 <<<"${REPORT}")
DETAIL=$(tail -n +2 <<<"${REPORT}")

if [ "${STATUS_LINE}" = "OK" ]; then
    echo "✅ ${DETAIL}"
    exit 0
else
    echo "❌ Workflow-trigger-policy check failed (${STATUS_LINE})"
    echo ""
    echo "${DETAIL}"
    echo ""
    echo "If this failure is unexpected:"
    echo "  1. CLAUSE_UNCOVERED: a workflow file has no row in the trigger-policy"
    echo "     table -- add one to docs/src/contributing/branching-model.md."
    echo "  2. CLAUSE_DRIFT (trigger types or branch filter): update the table row"
    echo "     to describe the YAML, or -- if the YAML changed by mistake -- fix"
    echo "     the workflow file instead of the table."
    echo "  3. CLAUSE_DRIFT (narrowed away from match-all): this is very likely the"
    echo "     reintroduced-branch-filter defect D-03/D-09b exist to prevent --"
    echo "     restore the match-all push filter rather than documenting a new"
    echo "     exception, unless a third exception is a deliberate, reviewed choice."
    echo "  4. CLAUSE_CONTEXT: a required-status-check context pinned in the trunk"
    echo "     ruleset no longer resolves to any declared job name -- a job was"
    echo "     probably renamed. Update the ruleset JSON's context to match, or"
    echo "     revert the rename."
    echo "  5. If this guard is wrong about a workflow or the table, fix the guard"
    echo "     rather than working around it."
    exit 1
fi
