#!/usr/bin/env bash
# check-workflow-suppressions.sh
#
# The D-08 regression guard (ADR-0036): fails if a `cargo audit` or
# `cargo deny` invocation in any `.github/workflows/*.yml`/`*.yaml` file ever
# carries an inline advisory-ignore flag again. `check-advisory-register.sh`
# already asserts that SECURITY-EXCEPTIONS.md, deny.toml, .cargo/audit.toml
# and Cargo.lock agree with each other -- but nothing, before this script,
# asserted that a *workflow file* carries no inline advisory suppression.
# That is the exact defect SUPPLY-01 fixed (ci.yml ran a second, differently-
# configured `cargo audit` job with two `--ignore` flags baked in). Without
# this guard the defect could be reintroduced tomorrow and every other gate
# would stay green.
#
# This script is offline and makes no network call. It parses workflow YAML
# structurally with PyYAML and walks `jobs.*.steps[].run` -- it never scrapes
# raw file text or comment prose to decide whether a command is a
# suppression, the same house rule `check-advisory-register.sh` states for
# TOML. It reports every violation it finds rather than stopping at the
# first. It only reads: it writes nothing and creates no temporary file, so
# running it twice in succession, with no change to any input, produces
# identical output and the same exit code.
#
# Two clauses are asserted, both accumulated into one shared failure list
# before the verdict is decided:
#
#   1. Inline-suppression co-occurrence. For every logical line of every
#      `run:` string (backslash-continuations joined first, so a
#      continuation cannot evade the match; then matched per logical line,
#      not over the whole block, so an unrelated command on another line of
#      the same `run: |` block cannot false-positive), a `cargo audit` or
#      `cargo deny` invocation and an `--ignore` flag co-occurring on that
#      one line is a violation. Co-occurrence is the primary defence: known
#      false-positive tokens already in the tree -- `mc mb ... --ignore-
#      existing` and `cargo test ... -- --ignored` -- both fail the
#      cargo-audit/cargo-deny half before the flag pattern is ever
#      consulted.
#   2. Audit invocation count. Exactly one logical line matching a
#      word-bounded `cargo audit` must exist across every scanned workflow
#      file. Zero means the compliant job vanished; more than one means the
#      duplicate-job defect SUPPLY-01 closed has come back.
#
# Matching is case-sensitive over the YAML-decoded `run:` string. Shell
# commands are conventionally lowercase, and a case-insensitive match would
# risk firing on unrelated prose that happens to share letters in a
# different case rather than on an actual invocation -- the same precision
# argument `check-advisory-register.sh` makes for its own string comparisons
# (`check-advisory-register.sh:13-15`). This is pinned by a test: a
# differently-cased planted line (`Cargo Audit --Ignore ...`) must NOT be
# flagged.
#
# A workflow file with zero `jobs.*.steps[].run` strings is a clean file,
# not an error. A workflows directory matching zero `*.yml`/`*.yaml` files
# is a named non-zero failure -- a silently-empty comparison that reports
# success over nothing is its own defect, exactly as `check-advisory-
# register.sh` treats a missing input file. A workflow file that fails to
# parse as YAML is also a named failure, never a silent skip.
#
# Usage:  ./scripts/check-workflow-suppressions.sh [workflows-dir]
#         The optional first positional argument overrides the scanned
#         directory (default: <repo-root>/.github/workflows). It exists so
#         this guard's own positive test can run against a scratch copy
#         without ever mutating the real tree.
# Exit:   0 if no workflow file carries an inline advisory-ignore
#         suppression on a cargo audit/deny invocation, and exactly one
#         `cargo audit` invocation exists across the corpus; non-zero
#         otherwise.

set -euo pipefail

WORKSPACE_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
WORKFLOWS_DIR="${1:-${WORKSPACE_ROOT}/.github/workflows}"

if ! command -v python3 >/dev/null 2>&1; then
    echo "ERROR: python3 is required for YAML parsing." >&2
    exit 1
fi

echo "🔍 Checking workflow files for inline advisory-ignore suppressions on cargo audit/deny ..."

REPORT=$(python3 - "${WORKFLOWS_DIR}" <<'PY'
import glob
import os
import re
import sys

import yaml

workflows_dir = sys.argv[1]

failures = []

# Word-bounded "cargo audit"/"cargo deny" -- the gate half of clause 1.
# Also accepts an optional "+toolchain" token between "cargo" and the
# subcommand ("cargo +nightly audit ...") and the hyphenated direct-binary
# form ("cargo-audit"/"cargo-deny") that `cargo install cargo-audit
# --locked` puts on PATH -- both are ordinary, documented ways to invoke
# these tools, not just the space-separated "cargo audit" form. A
# `cargo install cargo-audit --locked` invocation itself is neutralized by
# strip_install_segments() below (see its docstring for why a whole-line
# exclusion is not used) rather than by omission from this pattern,
# because the hyphenated alternative must also match a bare, non-install
# "cargo-audit --ignore ..." invocation.
CARGO_GATE_RE = re.compile(
    r'\bcargo\s+(?:\+\S+\s+)?(?:audit|deny)\b'
    r'|(?<!\w)cargo-(?:audit|deny)\b'
)
# Word-bounded "cargo audit" alone (optionally "+toolchain"-qualified), or
# the hyphenated direct-binary form -- clause 2's invocation counter. Every
# call site runs this against a strip_install_segments()-neutralized line,
# so "cargo install cargo-audit --locked" alone is never counted as an
# invocation.
CARGO_AUDIT_ONLY_RE = re.compile(
    r'\bcargo\s+(?:\+\S+\s+)?audit\b'
    r'|(?<!\w)cargo-audit\b'
)
# Requires whitespace (space or tab), an equals sign, a surrounding quote,
# or end-of-string immediately after "--ignore". This precise trailing-
# character requirement -- not a plain word-boundary assertion -- is what
# excludes "--ignore-existing" (followed by a hyphen) and "--ignored"
# (followed by "d"), while still catching a quoted ("--ignore") or
# tab-separated flag. A word-boundary assertion would NOT exclude
# "--ignore-existing": the transition from "e" to "-" is itself a word
# boundary, so `\b--ignore\b` would still match it.
IGNORE_FLAG_RE = re.compile(r'''--ignore(?:[\s=]|['"]|$)''')
# Matches a `cargo install ...` invocation from the word "install" through
# to (but not including) the next shell command-chaining operator
# (`&&`, `||`, `;`, `|`) or end of line.
INSTALL_SEGMENT_RE = re.compile(r'\bcargo\s+install\b[^;&|]*')


def strip_install_segments(line):
    """Remove every `cargo install ...` segment from a logical line before
    running the gate/flag checks against what remains. A `cargo install`
    invocation only installs a binary, it never invokes cargo-audit/
    cargo-deny itself -- but dropping the *entire* logical line whenever it
    merely contains an install invocation (an earlier version of this
    guard's exclusion) let a chained command that immediately invokes what
    was just installed ride through undetected:
    `cargo install cargo-audit --locked && cargo-audit --ignore RUSTSEC-...`
    is an ordinary shell idiom, not a contrived fixture. Stripping only the
    install segment itself -- up to the next `&&`/`||`/`;`/`|` or end of
    line -- and evaluating the remainder normally handles arbitrary
    chaining uniformly through the same code path, rather than requiring a
    special case per chaining operator."""
    return INSTALL_SEGMENT_RE.sub('', line)


def violates(line):
    """Both patterns matching the same logical line (after neutralizing any
    `cargo install ...` segments, which install a binary rather than
    invoking it) is the violation."""
    remainder = strip_install_segments(line)
    return bool(CARGO_GATE_RE.search(remainder) and IGNORE_FLAG_RE.search(remainder))


def logical_lines(run_text):
    """Join backslash-continuations first (a trailing backslash followed by
    a newline, plus any leading indentation on the continued line, becomes a
    single space), then split into stripped, non-empty logical lines.
    Matching happens per logical line, never over the whole block -- so a
    `run: |` block invoking `cargo audit` on one line and an unrelated
    `--ignore-existing` command on another line does not false-positive,
    and a backslash continuation splitting `cargo audit \\` from
    `--ignore RUSTSEC-...` onto the next line does not false-negative."""
    joined = re.sub(r'\\\r?\n[ \t]*', ' ', run_text)
    return [line.strip() for line in joined.split('\n') if line.strip()]


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

clause1_failures = []
clause2_locations = []
files_scanned = 0
run_steps_examined = 0

for path in files:
    files_scanned += 1
    try:
        with open(path, 'r', encoding='utf-8') as fh:
            data = yaml.safe_load(fh)
    except yaml.YAMLError as exc:
        failures.append(f'PARSE_ERROR: {path} did not parse as YAML: {exc}')
        continue

    # A file that parses to None (empty file), a job with no `steps`, and a
    # step with no `run` are each a clean absence, not an error.
    if not isinstance(data, dict):
        continue

    jobs = data.get('jobs')
    if not isinstance(jobs, dict):
        continue

    for job_name, job in jobs.items():
        if not isinstance(job, dict):
            continue
        steps = job.get('steps')
        if not isinstance(steps, list):
            continue
        for step_idx, step in enumerate(steps):
            if not isinstance(step, dict):
                continue
            run = step.get('run')
            if not isinstance(run, str):
                continue
            run_steps_examined += 1
            step_name = step.get('name', f'step[{step_idx}]')
            for line in logical_lines(run):
                if violates(line):
                    clause1_failures.append(
                        f'CLAUSE1_INLINE_SUPPRESSION: {path} job "{job_name}" '
                        f'step "{step_name}" (index {step_idx}) carries an '
                        f'advisory-ignore flag on a cargo audit/deny '
                        f'invocation: {line!r}'
                    )
                if CARGO_AUDIT_ONLY_RE.search(strip_install_segments(line)):
                    clause2_locations.append(
                        f'{path} job "{job_name}" step "{step_name}" '
                        f'(index {step_idx}): {line!r}'
                    )

# clause1_failures and clause2_locations are already in file-path-then-
# position order: the outer loop walks `files` sorted, and within one file
# jobs/steps/logical-lines are walked in document order (PyYAML preserves
# mapping insertion order), so no further re-sort is needed for two runs
# over an unchanged tree to produce byte-identical output.
failures.extend(clause1_failures)

audit_count = len(clause2_locations)
if audit_count != 1:
    locations = '; '.join(clause2_locations) if clause2_locations else '(none found)'
    failures.append(
        f'CLAUSE2_AUDIT_INVOCATION_COUNT: expected exactly 1 `cargo audit` '
        f'invocation across all workflow files, found {audit_count}. '
        f'Locations: {locations}.'
    )

if failures:
    print('FAIL')
    for f in failures:
        print(f'FAIL: {f}')
    sys.exit(0)

print('OK')
print(f'{files_scanned} workflow file(s) scanned, {run_steps_examined} run '
      f'step(s) examined, {audit_count} cargo audit invocation(s) found; no '
      'inline advisory-ignore suppression detected.')
sys.exit(0)
PY
)

STATUS_LINE=$(head -n1 <<<"${REPORT}")
DETAIL=$(tail -n +2 <<<"${REPORT}")

if [ "${STATUS_LINE}" = "OK" ]; then
    echo "✅ ${DETAIL}"
    exit 0
else
    echo "❌ Workflow-suppression check failed (${STATUS_LINE})"
    echo ""
    echo "${DETAIL}"
    echo ""
    echo "If this failure is unexpected:"
    echo "  1. Put the advisory in .cargo/audit.toml or deny.toml and add its"
    echo "     SECURITY-EXCEPTIONS.md register row rather than passing an"
    echo "     --ignore flag inline in a workflow file."
    echo "  2. If a second cargo audit invocation was added deliberately, it is"
    echo "     the duplicate-job defect SUPPLY-01 closed -- remove it rather"
    echo "     than reintroducing it."
    echo "  3. If this guard is wrong about a command, fix the guard rather"
    echo "     than the workflow."
    exit 1
fi
