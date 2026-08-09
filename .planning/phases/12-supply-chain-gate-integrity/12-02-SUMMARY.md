---
phase: 12-supply-chain-gate-integrity
plan: 02
subsystem: infra
tags: [cargo-audit, cargo-deny, ci, github-actions, pyyaml, security-governance, adr-0036]

# Dependency graph
requires:
  - phase: 12-supply-chain-gate-integrity (plan 01)
    provides: "SUPPLY-01/SUPPLY-02 closed gate transcripts, and the RESOLVED blocking checkpoint (option-a selected 2026-08-09) authorizing this plan to execute as written"
provides:
  - "scripts/check-workflow-suppressions.sh — the D-08 offline regression guard turning ADR-0036's invariant from an observation into a running check"
  - "A check-workflow-suppressions Makefile target wired into the check-gates aggregate"
  - "A Check workflow files for inline advisory suppressions step in ci.yml's cargo-deny job"
affects: [12-03-adr-0036, 12-04-hand-off]

# Tech tracking
tech-stack:
  added: []
  patterns:
    - "Sibling-guard-script convention: shebang, set -euo pipefail, WORKSPACE_ROOT via BASH_SOURCE, magnifying-glass/checkmark/cross-mark status vocabulary, single-quoted python3 heredoc, accumulate-every-failure-then-decide, STATUS_LINE/DETAIL split read by the bash wrapper — matched exactly from check-advisory-register.sh"
    - "Root-directory override argument (WORKFLOWS_DIR=\"${1:-...}\") added as a new convention not present in any of the three existing guard scripts, purely so the positive test can run against a mktemp scratch copy without ever touching the real tree"

key-files:
  created:
    - scripts/check-workflow-suppressions.sh
  modified:
    - Makefile
    - .github/workflows/ci.yml

key-decisions:
  - "Sibling script, not a fourth clause in check-advisory-register.sh (the discretionary choice 12-02-PLAN.md left open). Two reasons: every existing clause in check-advisory-register.sh parses TOML with tomllib, and a YAML-parsing clause would put two parsers in one heredoc against that script's single stated responsibility; and the positive test needs a root-directory override, which is an addition to the new guard's own contract and would otherwise change a script three phases already depend on."
  - "Matching is case-sensitive over the YAML-decoded run: string, stated explicitly in the header and pinned by a test (a differently-cased planted line, 'Cargo Audit --Ignore ...', was proven NOT to fire)."
  - "Clause 2's count is corpus-wide (across all six workflow files), not per-file — this matches the plan's stated D-08c sub-check and correctly caught the space/equals/backslash-continuation positive fixtures as ALSO tripping clause 2, since each added a second cargo audit invocation to the scratch copy."

requirements-completed: [SUPPLY-03]

coverage:
  - id: D1
    description: "The guard fires on a planted violation (space form) and its output names the planted step's file and line"
    requirement: "SUPPLY-03"
    verification:
      - kind: other
        ref: "./scripts/check-workflow-suppressions.sh <scratch-dir> with 'cargo audit --ignore RUSTSEC-2024-0001' planted -> exit non-zero, CLAUSE1_INLINE_SUPPRESSION names the file/job/step/line (this execution)"
        status: pass
    human_judgment: false
  - id: D2
    description: "The guard fires on the cargo deny check --ignore, --ignore= equals, and backslash-continuation forms"
    requirement: "SUPPLY-03"
    verification:
      - kind: other
        ref: "three separate scratch-dir invocations (this execution), each exit non-zero with CLAUSE1_INLINE_SUPPRESSION naming the planted line"
        status: pass
    human_judgment: false
  - id: D3
    description: "The guard stays silent on the two known false-positive tokens already in the real tree (mc mb --ignore-existing, cargo test -- --ignored) and is idempotent"
    requirement: "SUPPLY-03"
    verification:
      - kind: other
        ref: "./scripts/check-workflow-suppressions.sh (no arg, real tree, this execution) -> exit 0 twice, diff -q of the two captures succeeds"
        status: pass
    human_judgment: false
  - id: D4
    description: "Per-logical-line matching: a run: | block invoking cargo audit on one line and an unrelated --ignore-existing on another does not false-positive; clause 2 fires on a two-cargo-audit fixture; an empty scan directory is a named non-zero failure"
    requirement: "SUPPLY-03"
    verification:
      - kind: other
        ref: "synthetic single-job scratch fixture (this execution) -> exit 0; two-cargo-audit scratch fixture -> CLAUSE2_AUDIT_INVOCATION_COUNT non-zero; empty mktemp dir -> ZERO_FILES non-zero"
        status: pass
    human_judgment: false
  - id: D5
    description: "The guard is wired into make check-gates and ci.yml's cargo-deny job with no third wiring point, and both invocation sites reach the script"
    requirement: "SUPPLY-03"
    verification:
      - kind: other
        ref: "make check-workflow-suppressions -> exit 0; make check-gates -> exit 0 (4 guards); python3 structural parse of ci.yml confirms the cargo-deny job step (this execution)"
        status: pass
    human_judgment: false
  - id: D6
    description: "No scratch fixture escaped into the committed tree, and the three sibling guard scripts / three suppression files / all .rs files are untouched"
    requirement: "SUPPLY-03"
    verification:
      - kind: other
        ref: "git status --porcelain -- .github/workflows/ (empty); git status --porcelain -- scripts/check-advisory-register.sh scripts/check-crate-names.sh scripts/check-changelogs.sh deny.toml .cargo/audit.toml SECURITY-EXCEPTIONS.md (empty); git diff --name-only -- '*.rs' | wc -l -> 0 (this execution)"
        status: pass
    human_judgment: false

duration: ~20min
completed: 2026-08-09
status: complete
---

# Phase 12 Plan 02: D-08 Workflow-Suppression Regression Guard Summary

**New offline `scripts/check-workflow-suppressions.sh` guard (PyYAML-structural, two accumulating
clauses) wired into `make check-gates` and `ci.yml`'s `cargo-deny` job — turns ADR-0036's
single-source invariant from prose into a running check that fires on every planted violation form
and stays silent on both known false-positive tokens in the real tree.**

## Performance

- **Duration:** ~20 min
- **Started:** 2026-08-09
- **Completed:** 2026-08-09
- **Tasks:** 2 of 2
- **Files modified:** 3 (1 created: `scripts/check-workflow-suppressions.sh`; 2 modified: `Makefile`, `.github/workflows/ci.yml`)

## Precondition Verified

Read `.planning/phases/12-supply-chain-gate-integrity/12-01-SUMMARY.md` § `## Checkpoint Status`
before starting. Confirmed: **`option-a` was selected by a human on 2026-08-09** ("Proceed as
planned — ADR-0036 and the D-08 guard"), explicitly authorizing plans 12-02, 12-03 and 12-04 to
execute as written. Provenance recorded there: obtained via the runtime's `AskUserQuestion`
mechanism after the orchestrator declined to auto-select under `--auto`. This plan proceeded.

## Accomplishments

- Wrote `scripts/check-workflow-suppressions.sh`, following `check-advisory-register.sh`'s shape
  exactly: shebang, `set -euo pipefail`, `WORKSPACE_ROOT` via `BASH_SOURCE`, the
  🔍/✅/❌ status vocabulary, a single-quoted `python3` heredoc, accumulate-every-failure,
  `STATUS_LINE`/`DETAIL` split read by the bash wrapper. Added one new convention not present in
  any sibling script: a `WORKFLOWS_DIR="${1:-...}"` root-directory override, purely so the
  positive test can run against a scratch copy without ever mutating the real tree.
- Clause 1 (inline-suppression co-occurrence): backslash-continuations are joined before splitting
  a `run:` string into logical lines, and matching happens per logical line — a `cargo audit`/
  `cargo deny` invocation (`CARGO_GATE_RE = r'\bcargo\s+(?:audit|deny)\b'`) co-occurring with an
  `--ignore` flag (`IGNORE_FLAG_RE = r'--ignore(?:[= ]|$)'`) on the *same* logical line is a
  violation. The flag pattern's trailing-character requirement (space, equals, or end-of-string)
  is what excludes `--ignore-existing` and `--ignored` — a plain word-boundary assertion would not.
- Clause 2 (audit invocation count): exactly one word-bounded `cargo audit` must appear across the
  whole scanned corpus (`CARGO_AUDIT_ONLY_RE = r'\bcargo\s+audit\b'`); `cargo install cargo-audit`
  does not count because no whitespace separates the hyphenated binary name's two halves.
- Both clauses accumulate into one shared `failures` list; a zero-file glob and a YAML parse
  failure are also named non-zero failures rather than silent passes.
- Wired the guard into `Makefile`'s `check-gates` prerequisite chain and into `ci.yml`'s
  `cargo-deny` job, immediately after the existing `Check advisory exception register` step — the
  only two invocation sites, matching the plan's "no third wiring point" constraint.

## Task Commits

Each task was committed atomically:

1. **Task 1: Write the guard script and pin its behaviour with a positive/negative pair** -
   `87affa9` (feat)
2. **Task 2: Wire the guard into `make check-gates` and the `cargo-deny:` CI job** - `9eed488`
   (feat)

## Verification Transcript (every invocation this execution, verbatim outcomes)

**Negative — real, unmodified tree, no argument:**
```
$ ./scripts/check-workflow-suppressions.sh
🔍 Checking workflow files for inline advisory-ignore suppressions on cargo audit/deny ...
✅ 6 workflow file(s) scanned, 108 run step(s) examined, 1 cargo audit invocation(s) found; no inline advisory-ignore suppression detected.
```
Exit `0`. Both known false-positive tokens (`ci.yml:428-429` `mc mb ... --ignore-existing`;
`ci.yml:463,466,755,757` `cargo test ... -- --ignored`) stayed silent.

**Idempotency:** ran twice in succession; `diff -q` of the two captures succeeded — byte-identical
output, same exit code.

**Positive — space form** (`cargo audit --ignore RUSTSEC-2024-0001` planted into a `ci.yml`
scratch copy): exit non-zero. Output named the planted file/job (`publish-dry-run`)/step
(`planted`, index 4)/line, and (correctly, since the scratch copy now carried two `cargo audit`
invocations) also raised `CLAUSE2_AUDIT_INVOCATION_COUNT` naming both locations.

**Positive — `cargo deny check --ignore` form:** exit non-zero, `CLAUSE1_INLINE_SUPPRESSION` named
the planted line. Clause 2 stayed silent (correctly — `cargo deny` is not `cargo audit`).

**Positive — `--ignore=` equals form:** exit non-zero, `CLAUSE1_INLINE_SUPPRESSION` named
`cargo audit --ignore=RUSTSEC-2024-0001`; clause 2 also fired (second `cargo audit`).

**Positive — backslash-continuation form** (`cargo audit \` / `  --ignore RUSTSEC-2024-0001` on
the next line inside a `run: |` block): exit non-zero. The continuation was correctly joined into
one logical line (`'cargo audit  --ignore RUSTSEC-2024-0001'` in the failure message) before
matching — proving the join-then-split order in `logical_lines()`.

**Clause-2-only fixture** (a second bare `cargo audit` step, no `--ignore` anywhere): exit
non-zero, `CLAUSE2_AUDIT_INVOCATION_COUNT` naming count `2` and both locations; no
`CLAUSE1_INLINE_SUPPRESSION` line appeared.

**Per-logical-line non-violation** (redone against a synthetic single-job workflow, clause-2-
neutral, rather than a `ci.yml` copy, so the assertion isn't confounded by `ci.yml`'s own existing
`cargo audit` step): a `run: |` block with `cargo audit` on one line and
`mc mb testminio/other --ignore-existing` on the next → exit `0`, `1 cargo audit invocation(s)
found`. Proves the per-logical-line rule directly, not by inspection.

**Empty scan directory:** exit non-zero, `ZERO_FILES` / `FAIL: no *.yml or *.yaml files found ...`
— never a silent pass over nothing.

**Case-sensitivity** (`Cargo Audit --Ignore RUSTSEC-2024-0001` planted, differently cased): exit
`0` — confirms the case-sensitive matching the header declares, pinned by this test rather than
merely asserted.

**Cleanup:** every `mktemp -d` scratch directory used above was `rm -rf`'d immediately after its
assertion. `git status --porcelain -- .github/workflows/` returned empty at task end — the real
tree was never mutated.

**Wiring verification (Task 2):**
```
$ make check-workflow-suppressions
🔍 Checking workflow files for inline advisory-ignore suppressions on cargo audit/deny ...
✅ 6 workflow file(s) scanned, 109 run step(s) examined, 1 cargo audit invocation(s) found; no inline advisory-ignore suppression detected.
```
(109, not 108, because the new CI step is itself now a counted `run:` step; the `cargo audit`
count is unchanged at `1` because the new step invokes neither `cargo audit` nor `cargo deny`.)

```
$ make check-gates
🔍 Checking per-crate CHANGELOG.md coverage ... ✅ 10 publishable crate(s) checked ...
🔍 Checking crates.io package-name allow-list ... ✅ 11 publishable crate(s) checked ...
🔍 Checking the advisory exception register ... ✅ 10 register row(s) checked ...
🔍 Checking workflow files for inline advisory-ignore suppressions ... ✅ 6 workflow file(s) scanned, 109 run step(s), 1 cargo audit invocation(s) ...
```
Exit `0`, all four guards reported.

Structural YAML parse of the modified `ci.yml` confirmed the `cargo-deny` job carries a step named
`Check workflow files for inline advisory suppressions` whose `run` invokes the guard, and that
`Check advisory exception register` is still present.

## Files Created/Modified

- `scripts/check-workflow-suppressions.sh` (new, executable) — the D-08 regression guard
- `Makefile` — new `check-workflow-suppressions` target immediately below `check-advisory-register`
  in the identical two-line shape; appended to the `check-gates` prerequisite list
- `.github/workflows/ci.yml` — one new step, `Check workflow files for inline advisory
  suppressions`, inserted in the `cargo-deny:` job immediately after the existing `Check advisory
  exception register` step; `git diff -U0` confirms the hunk is confined to those 3 inserted lines,
  and `security-audit:` (`:60-78`) is byte-identical

## Decisions Made

- **Sibling script, not a fourth clause in `check-advisory-register.sh`** — the discretionary
  choice `12-02-PLAN.md` left open. Reasons: every existing clause in `check-advisory-register.sh`
  parses TOML with `tomllib`; a YAML-parsing clause would put two parsers in one heredoc against
  that script's single stated responsibility (register vs. `deny.toml`/`.cargo/audit.toml`/
  `Cargo.lock`). The positive test also needs a root-directory override, which is an addition to
  the new guard's own contract and would otherwise be a change to a script three phases already
  depend on. Both layouts satisfy D-08's fixed constraints; this one keeps each script's
  responsibility single.
- **Case-sensitive matching, stated explicitly and pinned by a test** rather than left implicit —
  matches the house rule `check-advisory-register.sh:13-15` states for its own string comparisons.
- **Clause 2's count is corpus-wide**, per the plan's explicit D-08c sub-check text — confirmed
  correct behavior when the space/equals/backslash-continuation positive fixtures each also tripped
  `CLAUSE2_AUDIT_INVOCATION_COUNT` (since each scratch copy started from a full `ci.yml` copy,
  which already carries one legitimate `cargo audit`, and the planted line added a second). This
  is not a bug: it demonstrates both clauses working independently and simultaneously.

## Deviations from Plan

None — plan executed exactly as written. The one test-construction adjustment (using a synthetic
minimal workflow file instead of a `ci.yml` copy for the per-logical-line non-violation case, so
that assertion wasn't confounded by `ci.yml`'s own pre-existing `cargo audit` step tripping clause
2) is a test-methodology refinement within Task 1's own "prove it" instructions, not a deviation
from any `<action>`, `<verify>`, or `<acceptance_criteria>` line — the acceptance criterion just
requires the block to exit `0`, which it does either way; the synthetic fixture makes the
assertion cleaner to read.

## Issues Encountered

None.

## User Setup Required

None — no external service configuration required.

## Next Phase Readiness

- SUPPLY-03 is closed by this plan: the invariant ADR-0036 will record is now enforced by a
  running check, not only asserted in prose.
- Plan 12-03 (ADR-0036, promoting `PROMOTION.md` Part B candidate 7) can now cite this guard's
  `## Code Locations` — `scripts/check-workflow-suppressions.sh`, `Makefile`'s
  `check-workflow-suppressions` target, and `ci.yml`'s `cargo-deny` job step — as its enforcement
  citation.
- No blockers for plan 12-04 (Phase 13 hand-off, requirement closure, `PROMOTION.md` update).

---
*Phase: 12-supply-chain-gate-integrity*
*Completed: 2026-08-09*
