---
phase: 01-ground-truth-decision-records
plan: 05
subsystem: docs
tags: [ground-truth, decision-records, ledger, mcp, sanctum, qdrant, battalion, requirements-audit]

# Dependency graph
requires:
  - phase: 01-ground-truth-decision-records
    provides: ".planning/ledgers/milestone-01.md header, D-19 evidence bar, D-20 verdict legend, and the flagged REPL/NG-7 divergence row (plan 01-01)"
provides:
  - "RECON-08 Epic 10 Task 7.0 dispute resolved: the validation report is recorded as wrong, the task list (103/103, no Task 7.0) is the corroborated document"
  - "The 102-vs-103 subtask arithmetic explained: neither total reconciles against a real six-subtask Task 7.0"
  - "MCP Streamable-HTTP and Sanctum/Qdrant divergence rows added, both classified superseded by shipped code with file:line citations and named exercisers"
  - "Battalion base module path resolved to battalion/mod.rs (code-observed), correcting Paladin Project Completion Plan.md's Epic 4 section"
  - "Milestone 1 requirement-count discrepancy recorded: 112 enumerated ledger rows vs 115 reported IDs, explained by three -v2 variant IDs sharing a row with their -v1 partner"
affects: [01-06, 01-07, phase-05, phase-07]

# Tech tracking
tech-stack:
  added: []
  patterns:
    - "RECON-08 resolution pattern: quote both disputed documents in fixed order (task list first, validation report second), record the exhaustive search commands and results before stating a verdict, and state exactly one of two named outcomes with no hedging"
    - "Divergence row evidence pattern: file:line citation to the shipped adapter plus a named test/example; where the named exerciser is #[ignore]-gated behind an external service, flag that half present, unproven rather than upgrading it"

key-files:
  created: []
  modified:
    - .planning/ledgers/milestone-01.md

key-decisions:
  - "RECON-08 verdict: the validation report's Task 7.0 claim is recorded as wrong, not as genuinely outstanding work — an exhaustive grep of .project/Milestone_1-MVP/Epic_10/, the rest of .project/Milestone_1-MVP/, and docs/ found zero corroborating artifact for a six-subtask 'Final Documentation Review', and the report's own 102 total does not reconcile against the task list's 103 under either reading (Task 7.0 added or Task 7.0 alone)"
  - "The Qdrant-specific Sanctum exerciser (tests/integration/qdrant_sanctum_tests.rs) is recorded present, unproven rather than satisfied, because its tests carry #[ignore = \"Requires Qdrant running on localhost:6334\"]; the in-memory Sanctum backend's test_store_and_retrieve (no #[ignore]) is the row's actual passing exerciser per the D-19 evidence bar"
  - "Requirement-count discrepancy resolved as a labeling difference, not a data error: 112 ledger rows and 115 requirement IDs are both correct because three -v2 variant IDs (REQ-herald-trait-v2, REQ-temperature-range-v2, REQ-test-coverage-target-v2) share one ledger row with their -v1 partner instead of getting a distinct row; the ledger records both figures under distinct labels rather than collapsing to one"
  - "Updated the pre-existing Epic 10 per-epic placeholder note (written by plan 01-01) to point at the now-resolved RECON-08 verdict instead of promising to 'carry' the row itself, keeping the partially-authored ledger internally consistent per the plan's backstop truth"

requirements-completed: [RECON-08, RECON-01]

coverage:
  - id: D1
    description: "Epic 10 Task 7.0 dispute (RECON-08) resolved with a recorded search and exactly one of two named verdicts, and the 102-vs-103 arithmetic explained"
    requirement: "RECON-08"
    verification:
      - kind: other
        ref: "for s in 'Final Documentation Review' 'tasks-epic10-validation-documentation.md' 'task6.0-validation-report.md' '102' '103'; do grep -qF \"$s\" .planning/ledgers/milestone-01.md; done && grep -qE 'genuinely outstanding|satisfied' .planning/ledgers/milestone-01.md — all checks pass"
        status: pass
    human_judgment: false
  - id: D2
    description: "All three known divergences (REPL/NG-7, MCP Streamable-HTTP, Sanctum/Qdrant) recorded as superseded by shipped code rows with file:line citations and named exercisers"
    requirement: "RECON-01"
    verification:
      - kind: other
        ref: "for s in 'Streamable-HTTP' 'sqlite-vss' 'Sanctum' 'Qdrant' 'NG-7' 'battalion/mod.rs' 'battalion/battalion.rs'; do grep -qF \"$s\" .planning/ledgers/milestone-01.md; done && [ \"$(grep -c 'superseded by shipped code' .planning/ledgers/milestone-01.md)\" -ge 4 ] — pass, count=4"
        status: pass
    human_judgment: false
  - id: D3
    description: "Battalion base module path resolved to battalion/mod.rs by direct directory listing, correcting Paladin Project Completion Plan.md's Epic 4 section"
    requirement: "RECON-01"
    verification:
      - kind: other
        ref: "[ -f crates/paladin-core/src/platform/container/battalion/mod.rs ] && [ ! -f crates/paladin-core/src/platform/container/battalion/battalion.rs ]"
        status: pass
    human_judgment: false
  - id: D4
    description: "Milestone 1 requirement-count discrepancy recorded with every figure (112, 115, 115) verbatim against its source and the arithmetic difference explained"
    requirement: "RECON-01"
    verification:
      - kind: other
        ref: "awk '/^## Milestone 1 as-shipped ledger/{flag=1;next}/^## /{if(flag){exit}}flag' .planning/REQUIREMENTS.md | grep -c '^| REQ-' returns 112; grep -c '\\*\\*115\\*\\*' .planning/ledgers/milestone-01.md returns 2"
        status: pass
    human_judgment: false

duration: ~35min
completed: 2026-07-31
status: complete
---

# Phase 1 Plan 05: Epic 10 dispute resolution and ingest-bookkeeping corrections Summary

**Resolved the RECON-08 Epic 10 Task 7.0 dispute (validation report recorded as wrong; the task list's 103/103 with no Task 7.0 is corroborated), added the MCP Streamable-HTTP and Sanctum/Qdrant divergence rows, and recorded the Battalion base module path and the 112-vs-115 requirement-count discrepancy in `.planning/ledgers/milestone-01.md`.**

## Performance

- **Duration:** ~35 min
- **Completed:** 2026-07-31
- **Tasks:** 2/2
- **Files modified:** 1

## Accomplishments

- RECON-08 is answered with one of exactly two named outcomes, backed by a recorded exhaustive search across `.project/Milestone_1-MVP/Epic_10/`, the rest of `.project/Milestone_1-MVP/`, and `docs/` — no trace of a "Final Documentation Review" artifact exists anywhere, so the verdict is that the validation report's claim is wrong, not that real work remains
- The 102-vs-103 subtask discrepancy is explained arithmetically: a real six-subtask Task 7.0 layered on the task list's 103 checked items would total at least 109, not 102, so the validation report's own number does not reconcile under either reading
- All three known divergences (interactive REPL vs Epic 9 NG-7, MCP Streamable-HTTP vs specified SSE, Sanctum/Qdrant vs specified `sqlite-vss`) are now recorded as `superseded by shipped code` rows, each with a `file:line` citation and a named exerciser — the Qdrant-specific test is honestly flagged `present, unproven` because it is `#[ignore]`-gated behind a live Qdrant instance, while the in-memory Sanctum backend's passing test is the row's actual exerciser
- The Battalion base module path is resolved to `battalion/mod.rs` by directly listing the directory, correcting `Paladin Project Completion Plan.md`'s Epic 4 section (which already contradicted its own Appendix B)
- The Milestone 1 requirement-count discrepancy (112 enumerated ledger rows vs 115 reported IDs) is fully explained: three `-v2` competing-variant IDs share one ledger row with their `-v1` partner rather than getting a distinct row, so no ID is actually missing — both figures are recorded under distinct labels

## Task Commits

Each task was committed atomically:

1. **Task 1: Resolve the Epic 10 Task 7.0 dispute and the 102-vs-103 discrepancy** - `76ad1a6` (feat)
2. **Task 2: Record the remaining two divergences and the ingest-bookkeeping corrections** - `9c80b01` (feat)

## Files Created/Modified

- `.planning/ledgers/milestone-01.md` - Added the `## Epic 10 Task 7.0 — dispute resolution (RECON-08)` section, two divergence rows (MCP Streamable-HTTP, Sanctum/Qdrant), the `## Ingest bookkeeping corrections (RECON-01)` section (Battalion base module path, requirement-count discrepancy), and updated the Epic 10 per-epic placeholder note

## Decisions Made

- RECON-08 verdict: the validation report is wrong (`satisfied`, task list corroborated), not `genuinely outstanding` — see key-decisions above for the full reasoning
- The Qdrant integration test is recorded `present, unproven` for its half of the exerciser requirement rather than upgraded to a full pass, since it requires a live Qdrant instance and is `#[ignore]`-gated by default; the in-memory Sanctum test is cited as the actually-passing exerciser
- The requirement-count discrepancy is recorded as a labeling difference between "ledger rows" (112) and "distinct requirement IDs" (115), not resolved by picking one figure — both are retained with an explicit statement of which label each number applies to
- Updated the Epic 10 per-epic placeholder (originally written by plan 01-01) to point at this plan's resolved RECON-08 verdict instead of promising to "carry" the row, so the partially-authored ledger stays internally consistent for plans 01-06/01-07 to read

## Deviations from Plan

None — plan executed as written. One authoring correction made mid-execution, not a deviation from plan content: an initial edit combined Task 1's Epic 10 section with Task 2's divergence rows in a single change; this was caught before committing Task 1, and the divergence rows were removed from that edit and re-added cleanly under Task 2's own edit and commit, so the two tasks remain atomically separated as the plan requires. All acceptance criteria and both tasks' `<verify>` commands passed (run as individual single-line commands per this worktree's Bash-tool sandbox, which rejects multi-command chains as unverifiable — same accommodation plan 01-01 recorded).

## Issues Encountered

None beyond the mid-execution authoring correction noted above, which was caught and fixed before either commit landed.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- `.planning/ledgers/milestone-01.md` now holds: the header/legend/REPL row (01-01), the two remaining divergence rows, the RECON-08 dispute resolution, and the RECON-01 bookkeeping corrections (this plan). The ten per-epic `REQ-*` sections remain as headings for plans 01-06 and 01-07 to fill.
- Plans 01-06/01-07 should use this plan's RECON-08 verdict (`satisfied`, validation report wrong) directly when authoring the Epic 10 per-epic section, rather than re-running the dispute search.
- Plans 01-06/01-07 should use the 112-rows/115-IDs distinction recorded here when citing "the Milestone 1 requirement count" — the two labels are not interchangeable.
- No blockers for the next plan in this phase.

---
*Phase: 01-ground-truth-decision-records*
*Completed: 2026-07-31*
