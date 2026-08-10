---
phase: 13-milestone-9-12-ground-truth-recorded-account
plan: 10
subsystem: docs
tags: [requirements-md, ledger, dated-correction, ci-yml, versioning]

# Dependency graph
requires:
  - phase: 13-milestone-9-12-ground-truth-recorded-account
    provides: "13-01 through 13-07 derived all 120 Milestone 9-12 ledger rows in .planning/ledgers/milestone-09-12.md to the full evidence bar"
provides:
  - "Dated corrections to REQUIREMENTS.md's ORCH-01 arithmetic (D-04), ORCH-05 current-state figures (D-18), and PIPE-01's ci.yml job list (D-08, D-09)"
  - "The Milestone 9-12 as-shipped ledger section reduced to a pointer at .planning/ledgers/milestone-09-12.md (D-01)"
affects: [phase-15-pipe, phase-16-docs, phase-13-plan-13-11, phase-13-plan-13-13]

# Tech tracking
tech-stack:
  added: []
  patterns: ["dated in-place correction: quote the superseded sentence in a strikethrough block placed after the untouched original, never edit the original line's bytes"]

key-files:
  created: []
  modified:
    - .planning/REQUIREMENTS.md

key-decisions:
  - "ORCH-01's settled-by/104 sentence left byte-identical; a new paragraph immediately follows it recording the sixteen as variant-register entries, zero ledger rows carrying settled-by, and the 35/53/32 split"
  - "ORCH-05's stale current-state clause left byte-identical; a new paragraph records the measured 0.7.0/v0.7.1/release-v0.7.0 state and cross-references Phase 10's prior HARD-03 correction of the same defect class"
  - "PIPE-01's 14-job quotation left byte-identical; a new paragraph records the measured 15-job list with line numbers and folds in the D-09 check-api-surface.sh residue as the same hand-off"
  - "Milestone 9-12 ledger section body (531 lines) replaced with a pointer only after re-verifying, this session, that the ledger holds exactly 120 REQ- rows, zero 'pending — plan' markers, and an ID list that diffs clean against the section being replaced"

requirements-completed: [ORCH-01, ORCH-05]

coverage:
  - id: D1
    description: "ORCH-01's settled-by/104 arithmetic corrected in place: sixteen are variant-register entries (not ledger rows), zero ledger rows carry settled-by, measured split 35 bare Verify / 53 bare Shipped / 32 richer, all 120 rows need a verdict"
    requirement: "ORCH-01"
    verification:
      - kind: other
        ref: "grep -c '35' .planning/REQUIREMENTS.md; awk range greps for variant-register/104/35 within the ORCH-01..ORCH-02 block; git diff removed-line count = 0"
        status: pass
    human_judgment: false
  - id: D2
    description: "ORCH-05's current-state clause corrected in place to measured Cargo.toml 0.7.0 / git tag v0.7.1 / branch release/v0.7.0, historical lockstep gates (v0.3.0..v0.6.0) left untouched"
    requirement: "ORCH-05"
    verification:
      - kind: other
        ref: "awk range greps for 0.7.0/v0.5.1/v0.3.0..v0.6.0 within the ORCH-05..'API contract truthfulness' block; git diff removed-line count = 0"
        status: pass
    human_judgment: false
  - id: D3
    description: "PIPE-01's stale 14-job ci.yml quotation corrected in place with the measured 15-job list (line numbers, security deleted, examples/kubernetes-smoke added) plus the D-09 check-api-surface.sh residue"
    verification:
      - kind: other
        ref: "grep -c kubernetes-smoke/benchmark-regression-signal/check-api-surface.sh:6 in REQUIREMENTS.md; git diff --name-only against '.project/*' '*.rs' = 0"
        status: pass
    human_judgment: false
  - id: D4
    description: "Milestone 9-12 as-shipped ledger section reduced to a pointer at .planning/ledgers/milestone-09-12.md, preconditions (120 rows, 0 pending markers, clean ID diff) asserted before the 531-line body was removed"
    verification:
      - kind: other
        ref: "awk range grep for '| REQ-' in the ledger section = 0; grep -c '^| REQ-' on the ledger file = 120; grep -c 'fifth and final' in REQUIREMENTS.md >= 1; git diff --numstat deletions = 531 (> 300)"
        status: pass
    human_judgment: false

duration: ~25min
completed: 2026-08-10
status: complete
---

# Phase 13 Plan 10: Correct ORCH-01/ORCH-05/PIPE-01 at source and pointer the Milestone 9-12 ledger Summary

**Four dated in-place corrections to `.planning/REQUIREMENTS.md`: ORCH-01's settled-by arithmetic, ORCH-05's stale version figures, PIPE-01's 14-job `ci.yml` quotation, and the 531-line Milestone 9-12 ledger body reduced to a pointer at `.planning/ledgers/milestone-09-12.md` — the fifth and final sibling ledger.**

## Performance

- **Duration:** ~25 min
- **Started:** 2026-08-10 (exact start not recorded; estimated from session length)
- **Completed:** 2026-08-10T20:33:17Z
- **Tasks:** 3
- **Files modified:** 1 (`.planning/REQUIREMENTS.md`)

## Accomplishments

- ORCH-01's "sixteen entries already carry `settled-by` pointers … remaining 104" sentence carries a dated correction: the sixteen are variant-register entries (`intel/SYNTHESIS.md:335`, `:546`), zero ledger rows carry `settled-by` (`grep -c "settled-by"` → 10 total, 0 inside the ledger region), and the measured split is 35 bare `Verify` / 53 bare `Shipped` / 32 already-rich, all 120 needing a verdict — matching `.planning/ledgers/milestone-09-12.md`'s own "Corrected arithmetic" paragraph exactly (D-04)
- ORCH-05's "`Cargo.toml` `version = "0.6.0"`, branch `release/v0.7.0`, latest tag `v0.5.1`" clause carries a dated correction to the measured `0.7.0` / `v0.7.1` / `release/v0.7.0` state, cross-referencing Phase 10's prior HARD-03 correction of the identical defect class and noting `intel/code-verification.md:469`'s same stale figure is plan 13-11's to correct (D-18)
- PIPE-01's verbatim 14-job `ci.yml` list carries a dated correction to the live measured 15-job list (`security` deleted by Phase 9, `examples` and `kubernetes-smoke` added, each with its `ci.yml` line number), handed to Phase 15/PIPE-01, with the D-09 `check-api-surface.sh:6` residue folded into the same hand-off (D-08, D-09)
- `.planning/REQUIREMENTS.md`'s `## Milestone 9-12 as-shipped ledger` section (531 lines) is reduced to a pointer at `.planning/ledgers/milestone-09-12.md`, the fifth and final sibling in the series, after re-verifying this session that the ledger already holds all 120 rows with zero interim markers and an ID list that diffs clean against the section it replaces (D-01)

## Task Commits

Each task was committed atomically:

1. **Task 1: Correct ORCH-01's arithmetic and ORCH-05's current-state figures** - `88e92c6` (docs)
2. **Task 2: Correct PIPE-01's ci.yml job list at source** - `2a59850` (docs)
3. **Task 3: Reduce the Milestone 9-12 as-shipped ledger section to a pointer** - `d13e139` (docs)

_Note: this plan runs in worktree mode; the metadata commit (SUMMARY.md + REQUIREMENTS.md) is made separately per the worktree protocol, not as a fourth task commit here._

## Files Created/Modified

- `.planning/REQUIREMENTS.md` - four dated corrections (ORCH-01, ORCH-05, PIPE-01) plus the Milestone 9-12 ledger section reduced to a pointer

## Decisions Made

- **Correction style: additive-only, never edit the original line's bytes.** The established convention elsewhere in `REQUIREMENTS.md` (e.g. HARD-03's `~~...~~` **Corrected (dated ...)** blocks) wraps the strikethrough directly around the original line's own text. Doing that here caused `git diff` to register the line as removed-then-added (character-identical content, but a different line), which would have failed this plan's own "no original line removed" acceptance gate. Instead, each correction leaves the original sentence's lines completely untouched and inserts a *new* paragraph immediately after it that re-quotes the superseded clause inside `~~...~~` for reference. Net effect is identical for a reader (original visible, superseded, followed by the correction) and produces zero removed lines in `git diff`.
- **Task 3 precondition interpreted by substance, not literal string count.** The plan's read_first asks for `grep -c 'run-5 input (not yet re-derived)' .planning/ledgers/milestone-09-12.md` to return `0` before any deletion. The literal count is `3` — all three are in the ledger's own head-note/legend prose describing the historical interim-state labeling convention (used by plan 13-01 before wave-2 fan-out plans 13-02..13-07 completed), not actual markers still present on any data row. Verified directly: zero lines starting with `|` (an actual table row) contain that phrase anywhere in the file, `pending — plan` (the other interim marker) is genuinely absent (count 0), and the ledger's own 120-row ID list diffs clean against the section being replaced. Proceeded with the deletion on that basis. See "Deviations from Plan" below for the full reasoning.

## Deviations from Plan

### Auto-fixed Issues

None — no bugs, missing functionality, or blocking issues were found in the codebase. All three "deviations" below are documentation of plan-authoring artifacts encountered while executing the plan's own acceptance criteria and read_first instructions, not code fixes.

### Notes on plan-authoring artifacts (not auto-fixed, no scope change)

**1. Task 3 precondition text vs. substance (informational only, no fix applied).**
- **Found during:** Task 3 read_first.
- **Issue:** The literal `grep -c 'run-5 input (not yet re-derived)'` count on `.planning/ledgers/milestone-09-12.md` is `3`, not `0` as the precondition specifies — all three occurrences are in the file's own legend/head-note prose (lines 25, 238, 412) describing the labeling convention itself, not markers still attached to any of the 120 data rows.
- **Resolution:** Verified substantively — zero `| ...` table rows contain the phrase, `pending — plan` is genuinely absent, and the ID list diffs clean. Proceeded with the deletion; the ledger's derivation is in fact complete. No ledger file edit was made (out of scope — `milestone-09-12.md` belongs to plans 13-01..13-07, not 13-10).
- **Impact:** None on outcome; documented for traceability since the precondition would have literally failed a naive `[ "$COUNT" = "0" ]` check.

**2. PIPE-01 acceptance criterion's `diff` command has a false-positive "push" leftover (informational only, no fix applied).**
- **Found during:** Task 2 acceptance-criteria verification.
- **Issue:** The acceptance criterion `diff <(grep -oE '^  [a-z][a-z0-9-]*:' .github/workflows/ci.yml | ...) <(...)` extracts `ci.yml`'s two-space-indented keys without anchoring to the `jobs:` block, so it also matches `  push:` under the unrelated top-level `on:` trigger block. Running the command literally reports one leftover line (`push`), which is not a CI job and correctly does not appear in the corrected job list.
- **Resolution:** Confirmed the actual measured job list (via the plan's own `<read_first>` command, `grep -nE '^  [a-z][a-z0-9-]*:$' .github/workflows/ci.yml`, which does anchor and correctly returns exactly 15 job ids) matches what was written into `REQUIREMENTS.md` exactly. No content change was made to accommodate the looser acceptance-check regex, since doing so would require falsely claiming `push` is a 15th job.
- **Impact:** None on outcome; the primary `<verify>` (`grep -c 'kubernetes-smoke'`) and six of seven acceptance criteria pass cleanly.

**3. Task 3's `body-removed` acceptance criterion has an off-by-one `awk` field reference (informational only, no fix applied).**
- **Found during:** Task 3 acceptance-criteria verification.
- **Issue:** `git diff --numstat -- .planning/REQUIREMENTS.md | awk '{print ($3 > 300) ? "body-removed" : "body-retained"}'` reads `$3`, which in `--numstat` output is the filename field (`.planning/REQUIREMENTS.md`), not the deletion count. A non-numeric string compared to `300` in `awk` is always `0 > 300` (false), so this command prints `"body-retained"` regardless of the actual diff size.
- **Resolution:** Confirmed the correct field is `$2` (deletions): re-running with `$2` reports `531 > 300` → `"body-removed"`, matching the real, substantial reduction. No content change made; this is purely a verification-script bug, unrelated to `REQUIREMENTS.md`'s content.
- **Impact:** None on outcome; the section's `| REQ-` row count within the ledger heading's own range is confirmed `0` by the plan's primary `<verify>` command, which is unaffected by this bug.

---

**Total deviations:** 0 auto-fixed. 3 informational notes on plan-authoring artifacts in acceptance-criteria commands, none requiring a code or content change.
**Impact on plan:** None — all four `must_haves.truths`, all prohibitions, and the plan's own primary `<verify>`/`<verification>` blocks pass. The three notes above document acceptance-criteria command imprecisions discovered while verifying, for the benefit of anyone re-running this plan's checks literally.

## Issues Encountered

None beyond the three documented plan-authoring artifacts above.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- `REQUIREMENTS.md` now states nothing about itself in ORCH-01, ORCH-05, or PIPE-01 that measurement contradicts; all three corrections retain their original text per D-00c/D-00d.
- The Milestone 9-12 ledger has exactly one home (`.planning/ledgers/milestone-09-12.md`); `REQUIREMENTS.md` no longer holds a second, diverging copy of the 120 rows.
- `intel/code-verification.md:469`'s same stale `0.6.0` figure remains open for plan 13-11 to correct — explicitly noted in the ORCH-05 correction as this plan's hand-off, not acted on here (out of scope per this plan's prohibitions).
- No `.rs` file was touched by this plan (confirmed via `git diff --name-only -- '*.rs'` = 0 after every task).
- No requirement checkbox, traceability row, or hand-off block was touched (confirmed by inspection); those remain plan 13-13's.

---
*Phase: 13-milestone-9-12-ground-truth-recorded-account*
*Completed: 2026-08-10*
