---
phase: 01-ground-truth-decision-records
plan: 08
subsystem: docs
tags: [adr, requirements-ledger, roadmap, planning-corpus, coverage-gate]

# Dependency graph
requires:
  - phase: 01-ground-truth-decision-records (plans 01-02, 01-03, 01-06, 01-07)
    provides: ADR-0001 through ADR-0005, and the completed Milestone 1 ledger
      (.planning/ledgers/milestone-01.md)
provides:
  - "PROJECT.md Key Decisions table with five real ADR rows (0001-0005) plus an
    explicit pending placeholder row for ADR-0006"
  - "A recorded, evidence-based finding that REQUIREMENTS.md's Milestone 1 ledger
    body cannot yet be safely reduced to a pointer — one requirement ID
    (REQ-battalion-result-v1) is missing from the destination ledger"
affects: [phase-01-completion, phase-03-planning (coverage criterion still unamended)]

# Tech tracking
tech-stack:
  added: []
  patterns:
    - "ADR Key Decisions table rows link out to .planning/decisions/*.md rather than
      restating ADR content, per D-02/D-03"
    - "Pending-decision placeholder row (no fabricated link) used when an ADR's
      source plan did not run"

key-files:
  created: []
  modified:
    - .planning/PROJECT.md
    - .planning/WINDOWS.md

key-decisions:
  - "Task 1 executed partially: five real ADR rows (0001-0005) added; ADR-0006 recorded as an explicit pending placeholder, not fabricated"
  - "Task 2 NOT executed (halted per the plan's own safety-check instruction): the REQ-* ID subset check failed, so REQUIREMENTS.md's Milestone 1 ledger body was left unreduced"
  - "Task 3 NOT executed per explicit orchestrator instruction: ROADMAP.md is untouched, Phase 3's coverage criterion remains unamended, forward-owned to plan 01-04/RECON-07"

requirements-completed: []

coverage: []

# Metrics
duration: ~35min
completed: 2026-07-31
status: complete
---

# Phase 1 Plan 08: Point Every Reader-Facing Document at Ground Truth (Partial) Summary

**PROJECT.md's Key Decisions table now links five real Phase 1 ADRs (0001-0005) with an explicit pending placeholder for ADR-0006; REQUIREMENTS.md's ledger reduction and ROADMAP.md's coverage-criterion amendment were both correctly NOT performed, because their prerequisites (a complete milestone-01.md ledger and a recorded ADR-0006 coverage number) do not exist yet.**

This plan completes **partially**, by design, given `depends_on: ["01-02","01-03","01-04","01-07"]` and plan 01-04 (RECON-07, the coverage-measurement gate) never ran — `cargo-llvm-cov` cannot be installed in this environment (crates.io returns HTTP 403), Docker is unavailable so the `--features integration-tests` scope cannot execute, and the repository's existing `lcov.info` predates the workspace migration. No ADR-0006, no measured coverage figure, and no `01-coverage-measurement.md` exist anywhere in this tree.

## Performance

- **Duration:** ~35 min (not precisely timestamped; parallel worktree execution)
- **Completed:** 2026-07-31T13:23:58Z
- **Tasks:** 1 of 3 fully executed as scoped, 1 of 3 halted by its own safety check (correct outcome), 1 of 3 intentionally not executed
- **Files modified:** 2 (`PROJECT.md`, `WINDOWS.md`)

## Accomplishments

- **Task 1 (partial, as explicitly scoped by the orchestrator):** `PROJECT.md`'s `## Key Decisions` table now holds six rows — five real ADR-linked rows (ADR-0001 BattalionConfig, ADR-0002 BattalionResult, ADR-0003 Formation minimum Paladin count, ADR-0004 temperature validation, ADR-0005 Herald trait signature) plus one explicit "pending" placeholder row for ADR-0006 naming the exact blocker (no `cargo-llvm-cov`, no Docker, stale `lcov.info`) with no fabricated file link. Added a pointer to `.planning/decisions/PROMOTION.md` after the eleven-ADR-candidate inventory, noting it now carries the promotion procedure and owner assignment for all eleven candidates.
- **Task 2 (verification run; reduction correctly withheld):** Ran the plan's own mandatory safety check — extracted all 112 `REQ-*` row IDs from REQUIREMENTS.md's `## Milestone 1 as-shipped ledger` body and confirmed each exists in `.planning/ledgers/milestone-01.md`. **The check failed**: `REQ-battalion-result-v1` (Epic 4 FR-4.2) is present in REQUIREMENTS.md as `Variant (group 4)` and is discussed in ADR-0002's own `## Considered Options` section ("superseded by the shipped superset... wholly subsumed"), but carries **no row anywhere** in `milestone-01.md`'s Epic 4 table (only `REQ-battalion-result-v2` appears, in Epic 5's table). Per the plan's explicit instruction — "If any identifier is missing from the new ledger, HALT and report it" — the reduction was **not performed**. `REQUIREMENTS.md` is byte-identical to its state before this plan ran. The finding is recorded as an open `unmet-truth` entry in `.planning/WINDOWS.md` for a future plan to fix (add the missing row to `milestone-01.md`, then re-run the reduction).
- **Task 3 (not executed, per explicit orchestrator instruction):** `.planning/ROADMAP.md` is untouched. Phase 3's success criterion 1 still names unit and integration coverage separately rather than the single workspace-wide figure D-08 would fix, because ADR-0006 (which would supply that number, scope and command) does not exist. This is forward-owned to whichever plan eventually re-runs RECON-07/plan 01-04's coverage measurement.

## Task Commits

1. **Task 1: Populate PROJECT.md Key Decisions with all six Phase 1 ADRs (partial — five real + one pending placeholder)** - `1bde299` (docs)
2. **Task 2: Verify-before-reduce safety check on REQUIREMENTS.md's Milestone 1 ledger — HALTED, no reduction performed** - `197d946` (docs)
3. **Task 3: Amend ROADMAP.md Phase 3's coverage criterion** - not executed (blocked on plan 01-04/ADR-0006, per explicit orchestrator instruction; no commit)

**Plan metadata:** this SUMMARY's own commit (see below)

## Files Created/Modified

- `.planning/PROJECT.md` - Added five real ADR rows (0001-0004, joining the existing 0005 row) plus one pending-placeholder row for ADR-0006, and a `PROMOTION.md` pointer after the eleven-ADR-candidate inventory. `git diff --stat`: 14 insertions, 4 deletions — well under the plan's 100-line scope bound.
- `.planning/WINDOWS.md` - Created (did not previously exist). One open `unmet-truth` entry recording the `REQ-battalion-result-v1` gap discovered by Task 2's safety check.
- `.planning/REQUIREMENTS.md` - **Not modified.** Task 2's reduction step did not run; the file is byte-identical to its pre-plan state.
- `.planning/ROADMAP.md` - **Not modified.** Task 3 did not run, per explicit instruction.

## Decisions Made

- **Followed the plan's own designed HALT-and-report protocol for Task 2's safety check literally**, rather than treating the discovered gap as a Rule-1 auto-fixable bug. The plan's action text is explicit ("HALT and report it — reducing the body to a pointer while a row is missing destroys the record the pointer claims to relocate") and anticipates exactly this failure mode; patching `milestone-01.md` (a file outside this plan's declared scope, and the deliverable of a different plan, 01-07) to manufacture a passing check would have masked a real data-completeness question about that ledger rather than surfacing it.
- **Recorded the missing-ID finding in `.planning/WINDOWS.md`** as an `unmet-truth` entry so it is visible at ship time and does not require re-discovering the same gap in a future run.
- **Did not touch ROADMAP.md at all**, per the orchestrator's explicit instruction, even though Task 3's own read-only research (reading ADR-0006, which doesn't exist) would have been trivial to attempt and fail loudly — the safer action was to not attempt it.

## Deviations from Plan

### Auto-fixed Issues

None — no Rule 1/2/3 auto-fixes were applied. The one anomaly found (the missing `REQ-battalion-result-v1` row) was handled per the plan's own explicit HALT instruction, not via the deviation-rules auto-fix path, since the plan text itself specifies the exact required response to this exact scenario.

---

**Total deviations:** 0 auto-fixed.
**Impact on plan:** Task 1 executed to the full extent the missing ADR-0006 allows; Task 2 correctly declined to destroy a record by pointing at an incomplete destination; Task 3 correctly declined to invent a coverage number. No scope creep, no fabricated content.

## Known Stubs

- **`.planning/PROJECT.md` Key Decisions table, ADR-0006 row** — intentionally a "pending" placeholder, not a stub in the harmful sense: it names the exact blocker and carries no fabricated link. Will resolve when plan 01-04 (RECON-07) runs and authors `.planning/decisions/0006-coverage-gate.md`.

## Issues Encountered

- **`.planning/ledgers/milestone-01.md` is missing a row for `REQ-battalion-result-v1`.** This requirement ID (Epic 4 FR-4.2) appears in REQUIREMENTS.md's original Milestone 1 ledger as `Variant (group 4)`, and ADR-0002 (which this plan read as part of Task 1) already analyzes it by name in its `## Considered Options` section, concluding it is "superseded by the shipped superset... wholly subsumed." Despite that, no row citing this ID exists anywhere in `milestone-01.md`'s Epic 4 or Epic 5 tables (Epic 5's table has `REQ-battalion-result-v2` but Epic 4's table, which is where the `-v1` half belongs, has no equivalent row). This is very likely an authoring omission in plan 01-07 (the plan that built `milestone-01.md`'s Epic 4-10 tables) rather than a genuine open question — the answer is already written down in ADR-0002, it just was never turned into a ledger row. **Recorded in `.planning/WINDOWS.md`, forward-owned to whoever next touches `milestone-01.md` or re-attempts this plan's Task 2.**

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- **Not ready for phase close-out.** Two of three tasks in this plan are blocked on prerequisites this plan cannot supply: Task 2 needs one added row in `milestone-01.md` before its safety check will pass; Task 3 needs `.planning/decisions/0006-coverage-gate.md` to exist (i.e., plan 01-04/RECON-07 must actually run — `cargo-llvm-cov` needs to become installable, or Docker needs to become available, or an alternative coverage-measurement path needs to be found and approved).
- Task 1's ADR-0006 placeholder row in `PROJECT.md` and Task 3's unamended ROADMAP.md criterion are both explicitly designed to be revisited by a follow-up plan once ADR-0006 exists — no further scope invention is needed at that point, just filling in the two rows this plan already marked as pending.
- The `REQ-battalion-result-v1` ledger gap is independent of the coverage blocker and could be fixed at any time by adding one row to `milestone-01.md`'s Epic 4 table (content already available from ADR-0002), after which Task 2's reduction could be safely re-run.

---
*Phase: 01-ground-truth-decision-records*
*Completed: 2026-07-31*
