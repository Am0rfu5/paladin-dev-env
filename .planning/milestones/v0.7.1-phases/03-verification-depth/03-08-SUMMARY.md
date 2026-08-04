---
phase: 03-verification-depth
plan: 08
subsystem: testing

tags: [requirements-amendment, roadmap-amendment, milestone-ledger, adr-0006, human-checkpoint, qual-02, qual-03]

# Dependency graph
requires:
  - phase: 03-verification-depth
    provides: "Plan 03-01's entry coverage measurement, plan 03-07's exit coverage measurement and QUAL-03 critical-path exerciser evidence, and plan 03-06's src/bin/paladin-server.rs deferral decision — all of it transcribed into this plan's amendments rather than re-derived"
provides:
  - "QUAL-02 and QUAL-03 amended at source in REQUIREMENTS.md with dated provenance, original text retained"
  - "ROADMAP.md Phase 3 success criterion 2 amended at source, no longer treating Redis and MinIO as the same position"
  - "Milestone ledger amended: REQ-commander-error-strategy row updated in place, five-plus deferred-with-reason rows and closure rows added"
  - "Human confirmation of both coverage figures, the ratchet decision, and the full amendment set, recorded in 03-coverage-measurement.md"
affects: [gsd-secure-phase, milestone-close-audit, phase-05-verify, phase-15-pipe]

tech-stack:
  added: []
  patterns: ["Amend-at-source-with-provenance-note: superseded text retained verbatim, dated amendment appended beside it citing the evidence file, never replaced silently"]

key-files:
  created: []
  modified:
    - .planning/REQUIREMENTS.md
    - .planning/ROADMAP.md
    - .planning/ledgers/milestone-01.md
    - .planning/phases/03-verification-depth/03-coverage-measurement.md

key-decisions:
  - "In-place amendment with provenance notes only (no separate amendments record) — matches Phase 2's precedent, keeps one evidence file per measurement rather than forking the account into two places that can drift"
  - "QUAL-0N traceability rows left in the file-wide Complete/Pending vocabulary rather than switched to the ledger's five-class verdicts, for consistency with every other requirement row in that table — flagged to the human approver rather than silently resolved"
  - "Ratchet not fired: exit delta 1.92pp is 0.08pt short of ADR-0006's 2pt trigger; approver was offered the option to raise the floor to 85% now and declined by approving as-is, leaving the raise question with milestone close"

requirements-completed: [QUAL-01, QUAL-02, QUAL-03, QUAL-04, QUAL-05]

coverage:
  - id: D1
    description: "QUAL-02's stale eleven-file offender list amended in place: nine contradicted entries corrected with claimed vs. measured figures, redis.rs confirmed and closed at 34.69%, minio.rs recorded outside ADR-0006's scope with a named owner"
    requirement: "QUAL-02"
    verification:
      - kind: other
        ref: "grep -q '03-coverage-measurement' .planning/REQUIREMENTS.md && grep -q 'COVERAGE_ANALYSIS.md' .planning/REQUIREMENTS.md && grep -q 'arsenal_execution_service.rs' .planning/REQUIREMENTS.md && grep -q 'minio.rs' .planning/REQUIREMENTS.md"
        status: pass
    human_judgment: false
  - id: D2
    description: "QUAL-03's superseded percentage clause amended in place with disposition 'superseded by shipped code' citing ADR-0006, surviving substance restated as three named passing D-19-bar exercisers, no new percentage introduced"
    requirement: "QUAL-03"
    verification:
      - kind: other
        ref: "grep -q 'superseded by shipped code' .planning/REQUIREMENTS.md && grep -q 'ADR-0006' .planning/REQUIREMENTS.md"
        status: pass
    human_judgment: false
  - id: D3
    description: "All five QUAL-0N traceability rows moved off Pending to the class the evidence supports"
    requirement: "QUAL-01"
    verification:
      - kind: other
        ref: "grep -cE '^\\| QUAL-0[1-5] \\| Phase 3 \\| Pending \\|' .planning/REQUIREMENTS.md -> 0"
        status: pass
    human_judgment: false
  - id: D4
    description: "ROADMAP.md Phase 3 success criterion 2 amended at source, no longer naming files the measurement shows already covered, no longer treating Redis and MinIO as the same position"
    verification:
      - kind: other
        ref: "grep -q '03-coverage-measurement' .planning/ROADMAP.md && grep -qE 'Amended by Phase 3' .planning/ROADMAP.md"
        status: pass
    human_judgment: false
  - id: D5
    description: "REQ-commander-error-strategy ledger row amended in place (not duplicated) citing the four relocated commander_error_paths_test.rs tests; new deferred-with-reason and closure rows added, each with a named owner or an explicit no-owner statement"
    requirement: "QUAL-04"
    verification:
      - kind: other
        ref: "grep -c 'REQ-commander-error-strategy' .planning/ledgers/milestone-01.md -> 1; grep -c 'deferred with reason' .planning/ledgers/milestone-01.md increased by >= 5"
        status: pass
    human_judgment: false
  - id: D6
    description: "Human has confirmed both recorded coverage figures (85.56% entry, 85.92% exit), the ratchet non-trigger decision, and the full amendment set through the blocking checkpoint"
    requirement: "QUAL-01"
    verification: []
    human_judgment: true
    rationale: "This is precisely the blocking human-verify checkpoint the plan requires — the numbers and the amendment set must be independently reviewed and approved by a human, not self-certified by the executing agent. The approval was received as the literal resume signal 'approved' and is now recorded in 03-coverage-measurement.md's Human confirmation section."

duration: ~30min
completed: 2026-08-02
status: complete
---

# Phase 3 Plan 08: Requirement & Roadmap Amendment at Source Summary

**QUAL-02 and QUAL-03 corrected at source in REQUIREMENTS.md with dated provenance, ROADMAP criterion 2 and the milestone ledger amended to match, and a human has confirmed both coverage figures (85.56% entry / 85.92% exit) and the ratchet non-trigger decision through a blocking checkpoint.**

## Performance

- **Duration:** ~30 min (Tasks 1-2 by a prior agent, Task 3 resumed and completed by this continuation)
- **Started:** 2026-08-02T17:58:xxZ (approx, prior agent)
- **Completed:** 2026-08-02T18:28:12Z
- **Tasks:** 3
- **Files modified:** 4

## Accomplishments

- QUAL-02 amended in place: original eleven-file offender list retained verbatim, dated amendment appended naming all nine contradicted files with claimed vs. measured figures, `redis.rs` confirmed as the one true positive and closed at 34.69%, `minio.rs` recorded outside ADR-0006's default-feature scope with owner VERIFY-05/PIPE-02.
- QUAL-03 amended in place: original percentage clause retained, disposition recorded as `superseded by shipped code` citing ADR-0006, surviving substance restated as three named critical paths each with a passing D-19-bar exerciser, zero coverage percentages introduced.
- All five QUAL-0N traceability rows moved off `Pending` to the class the phase's evidence actually supports.
- ROADMAP.md Phase 3 success criterion 2 amended in the same parenthetical style as criterion 1's existing plan-01-12 amendment — Redis and MinIO no longer treated as the same position, stale file enumeration replaced with a dated pointer at `03-coverage-measurement.md`.
- `REQ-commander-error-strategy` milestone ledger row amended in place (not duplicated) to cite the four relocated `commander_error_paths_test.rs` tests; five-plus `deferred with reason` rows and multiple `satisfied`/`present, unproven` closure rows added, each at the D-19 bar or with a named owner.
- Human confirmation recorded in `03-coverage-measurement.md`: approver is the project owner (git user `Am0rfu5`), resume signal `approved`, both figures (85.56% entry at `bb35554`, 85.92% exit at `1ad8be5`) and their re-derived arithmetic confirmed, the ratchet non-trigger decision confirmed (1.92pt exit delta, 0.08pt short of the 2pt trigger — approver declined the offered option to raise the floor now), `src/bin/paladin-server.rs`'s Phase 5/VERIFY-05 deferral confirmed, and the QUAL-0N traceability-vocabulary asymmetry flagged rather than resolved.

## Task Commits

Each task was committed atomically:

1. **Task 1: Amend QUAL-02 and QUAL-03 at source in REQUIREMENTS.md** - `bcde4d8` (docs) — completed by prior agent
2. **Task 2: Amend ROADMAP criterion 2 and the milestone ledger rows in place** - `2885e3c` (docs) — completed by prior agent
3. **Task 3: Human confirmation of both coverage figures, the ratchet decision and the amendment set** - `393b450` (docs) — completed this continuation, recording the `approved` resume signal

**Plan metadata:** committed separately after this SUMMARY (see final commit below).

## Files Created/Modified

- `.planning/REQUIREMENTS.md` - QUAL-02 and QUAL-03 amended in place with dated provenance; all five QUAL-0N traceability rows moved off `Pending`
- `.planning/ROADMAP.md` - Phase 3 success criterion 2 amended in place with dated provenance
- `.planning/ledgers/milestone-01.md` - `REQ-commander-error-strategy` row amended in place; new deferred-with-reason and closure rows added
- `.planning/phases/03-verification-depth/03-coverage-measurement.md` - `## Human confirmation` section appended recording the checkpoint's approval

## Decisions Made

- In-place amendment with provenance notes only, no separate amendments record — matches Phase 2's precedent and this plan's own recorded decision, avoiding a forked account that could drift.
- QUAL-0N traceability rows kept in the file-wide `Complete`/`Pending` vocabulary rather than switched to the ledger's five-class verdicts, for internal consistency with every other requirement row in that table. Flagged to the human approver at the checkpoint rather than resolved unilaterally, since it is a documented tension rather than a defect this plan is chartered to fix.
- Ratchet trigger not fired: the exit delta of 1.92 percentage points falls 0.08 points short of ADR-0006's 2-point threshold. The approver was explicitly offered the option to raise the floor to 85% now instead of waiting for milestone close, and declined by approving the phase's findings as written — ADR-0006 remains unamended and the raise question stays with the milestone-close audit.

## Deviations from Plan

None - plan executed exactly as written across all three tasks. Tasks 1 and 2 were completed by a prior agent and verified present via `git show --stat bcde4d8 2885e3c` before this continuation proceeded; no rework was needed.

## Issues Encountered

None. The checkpoint's `how-to-verify` steps (arithmetic re-derivation, ratchet-decision review, closure-ledger review, `git diff --stat` scope check) were all satisfied before the `approved` resume signal was recorded.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- This is the last plan in Phase 3 (wave 4, depends on 03-07). All 8 plans in `03-verification-depth` are now complete.
- QUAL-01 through QUAL-05 are all recorded off `Pending` with evidence-backed dispositions; the only open QUAL-02 item is `src/bin/paladin-server.rs`, already owned by Phase 5 / VERIFY-05.
- ADR-0006's 84% floor is confirmed unamended and human-approved at both the entry (85.56%) and exit (85.92%) figures; the ratchet-raise question is explicitly deferred to milestone close, not silently dropped.
- No blockers for Phase 3 closure or for Phase 5 / VERIFY-05's later work on `paladin-server.rs`.

---
*Phase: 03-verification-depth*
*Completed: 2026-08-02*

## Self-Check: PASSED

All modified files verified present on disk; all three task commits (`bcde4d8`, `2885e3c`, `393b450`) verified present in `git log --oneline --all`.
