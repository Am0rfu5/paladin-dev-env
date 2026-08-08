---
phase: 10-milestone-7-8-ground-truth-recorded-account
plan: 09
subsystem: docs
tags: [ledger, requirements-traceability, facade-audit, adr-0028, milestone-8, do-not-re-delete]

requires:
  - phase: 10-milestone-7-8-ground-truth-recorded-account
    provides: "10-01's ledger scaffold (86 row stubs, seven-class legend, supersession summary table) and ADR-0028's reconciliation record (the reproducible orphan test, the three in-execution corrections, Epic 3/Epic 6 completeness, the paladin-herald/paladin-ml non-goal split)"
provides:
  - "Milestone 8 Epic 1 (4 rows), Epic 2 (4 rows) and Epic 3 (6 rows) of .planning/ledgers/milestone-07-08.md fully derived, replacing scaffold pending stubs in place"
  - "Two row-level DO NOT RE-DELETE markers (REQ-storage-shim-deletion, REQ-garrison-sanctum-bridges-kept) plus a third epic-note-level correction, so a planner reading the ledger row sees the corrections before reading ADR-0028"
  - "REQ-adapter-disposition-record names Phase 11 / FACADE-04 as the owner of the two non-existent paladin-arsenal/paladin-sanctum crate names"
affects: ["10-10", "10-11"]

tech-stack:
  added: []
  patterns:
    - "Cell-replacement-only ledger fan-out: two per-task commits inside one file's disjoint epic ranges, verified via grep -c row/section counts and git diff --numstat added==deleted before each commit"
    - "Do-not-re-delete markers placed in the ledger row itself (not only in the linked ADR), so a planner reading the row before asking a question sees the correction inline"

key-files:
  created: []
  modified:
    - .planning/ledgers/milestone-07-08.md

key-decisions:
  - "REQ-facade-file-inventory recorded present, unproven rather than satisfied or superseded: the 189-file enumeration act survives and is cited by ADR-0028, but its List C classification built on top of it is what REQ-facade-file-classification's own row records superseded — the two rows are kept distinct per this plan's instruction to say which part survives"
  - "REQ-facade-file-classification and REQ-facade-audit-document both superseded by outcome, each stating the factual reason (~4,400 LOC of orphaned, uncompiled duplicate files classified as active bridges that stay) rather than a procedural one, per D-07"
  - "REQ-shim-consumer-validation cites ADR-0028's Decision (i) three-step orphan test by section rather than restating the procedure in the cell, per this plan's instruction that a paraphrased procedure is how the procedure drifts"
  - "REQ-libr-dead-reexport-removal upgraded from the run-4 ledger's unactioned Verify -> HARD-01 stub to satisfied: grep -c '^pub use' src/lib.rs returns 10 today (down from the ~50 pre-cleanup baseline), and the five confirmed-consumer exception groups the requirement names are exactly what remains"
  - "REQ-storage-shim-deletion carries the sqlite DO NOT RE-DELETE marker: the sqlite_*_repository.rs files were the active default-build implementation, not redundant, resolved by making paladin-storage non-optional in commit 897e77e"
  - "REQ-garrison-sanctum-bridges-kept carries both the registry DO NOT RE-DELETE marker (paladin_registry.rs consolidated into paladin-battalion via commit ca7e4e8, not deleted) and this row's own factual correction (api_content_deliverer.rs is 724 LOC not 629, corrected from tensorflow_adapter.rs's figure), per this plan's explicit row assignment even though the marker's subject is a different file than the row's own garrison/sanctum topic"
  - "REQ-adapter-disposition-record superseded by outcome carrying three separable facts on one row: the 20-row all-Stays table contradicted by the tree, two rows disagreeing with the governing PRD, and the two invented crate names (paladin-arsenal, paladin-sanctum) confirmed absent from ls crates/ and handed to Phase 11 / FACADE-04 by name"
  - "REQ-m8-epic3-no-extractions superseded by outcome with the 15-commit / net-10,252-LOC figures re-measured independently this session (git log --oneline e5b2011~1..a1e4901 | wc -l; git diff --shortstat), matching ADR-0028's own re-derivation rather than transcribing the reconciliation's prose, and stated to agree in substance with the head-of-file supersession summary table's existing entry under the same ID"
  - "REQ-notification-task-closeout records both deletion halves: the application-layer channel files were deleted as orphaned dead code in Epic 2 Batch 1 (never moved as the M8 overview's success criteria stated), and separately the facade's infrastructure-layer fallback duplicates were deleted by commit cf17559 -- two distinct deletion events, both narrower than 'moved'"
  - "The epic-wide third correction (mysql_content_repository.rs, the five input/* fetchers, document/*, api_content_deliverer.rs and error_log_adapter.rs genuinely were orphaned) is recorded in the M8 Epic 3 epic note rather than in any single row, per this plan's instruction that it applies to the epic as a whole"

requirements-completed: [HARD-01, HARD-02]

coverage:
  - id: D1
    description: "Milestone 8 Epic 1's four facade-audit rows re-derived, with the two superseded-audit rows stating the factual ~4,400 LOC reason and citing ADR-0028"
    requirement: "HARD-01"
    verification:
      - kind: other
        ref: "awk '/^### Milestone 8 Epic 1/{p=1;next}/^### /{p=0}p' .planning/ledgers/milestone-07-08.md | grep -c '^| REQ-' == 4; same range grep -c 'pending — plan' == 0; grep -c 'ADR-0028' .planning/ledgers/milestone-07-08.md >= 2"
        status: pass
    human_judgment: false
  - id: D2
    description: "Milestone 8 Epic 2's four dead-shim/empty-module rows re-derived, including REQ-libr-dead-reexport-removal upgraded from an unactioned Verify stub to satisfied"
    requirement: "HARD-01"
    verification:
      - kind: other
        ref: "awk '/^### Milestone 8 Epic 2/{p=1;next}/^### /{p=0}p' .planning/ledgers/milestone-07-08.md | grep -c '^| REQ-' == 4; grep -n REQ-libr-dead-reexport-removal .planning/ledgers/milestone-07-08.md shows 'satisfied'"
        status: pass
    human_judgment: false
  - id: D3
    description: "Milestone 8 Epic 3's six relocation rows re-derived with two row-level DO NOT RE-DELETE markers and a third epic-note-level correction, plus FACADE-04 named as owner of the two non-existent crate names"
    requirement: "HARD-02"
    verification:
      - kind: other
        ref: "awk '/^### Milestone 8 Epic 3/{p=1;next}/^### /{p=0}p' .planning/ledgers/milestone-07-08.md | grep -c '^| REQ-' == 6; grep -ci 'DO NOT RE-DELETE' .planning/ledgers/milestone-07-08.md >= 2; grep -c 'FACADE-04' .planning/ledgers/milestone-07-08.md >= 1"
        status: pass
    human_judgment: false
  - id: D4
    description: "Ledger row/section inventory unchanged by this plan's cell-replacement-only edits; each task's diff shows added lines equal to deleted lines, and the M8 Epic 4 section is byte-unchanged"
    verification:
      - kind: other
        ref: "grep -c '^| REQ-' .planning/ledgers/milestone-07-08.md == 86; grep -c '^### ' == 12; git diff --numstat for each of the two task commits shows added == deleted (10/10, then 9/9); git diff 3a156d3..HEAD -- ledger shows 19/19"
        status: pass
    human_judgment: false
  - id: D5
    description: "No .rs file modified by this plan (D-23 boundary held)"
    verification:
      - kind: other
        ref: "git status --porcelain -- '*.rs' — empty"
        status: pass
    human_judgment: false

duration: ~35min
completed: 2026-08-08
status: complete
---

# Phase 10 Plan 09: Milestone 8 Epic 1-3 Ledger Derivation Summary

**Derived all 14 Milestone 8 Epic 1-3 ledger rows from the tree and ADR-0028, placing two DO NOT RE-DELETE markers directly in the rows (the consolidated `paladin_registry.rs` and the non-optional sqlite repositories) so a planner sees the corrections before reading the ADR.**

## Performance

- **Duration:** ~35 min
- **Completed:** 2026-08-08
- **Tasks:** 2
- **Files modified:** 1 (`.planning/ledgers/milestone-07-08.md`)

## Accomplishments

- Re-derived all four Milestone 8 Epic 1 rows from `facade-audit.md` and ADR-0028 rather than the run-4 ledger's status words: `REQ-facade-file-classification` and `REQ-facade-audit-document` land `superseded by outcome`, each stating the factual reason — approximately 4,400 LOC of orphaned, uncompiled duplicate files (the `input/*` fetchers, `document/*`, `output/api_content_deliverer.rs`, `logs/error_log_adapter.rs`, `repositories/mysql_content_repository.rs`) classified as "active bridges that stay" — with the LOC figure and the classification's own 151/13/25 totals written into the cell; `REQ-facade-file-inventory` recorded `present, unproven` (the enumeration survives; the classification built on it does not); `REQ-shim-consumer-validation` recorded `satisfied`, citing ADR-0028's three-step orphan test by section.
- Re-derived all four Milestone 8 Epic 2 rows on re-run absence evidence: `find src/ -name "*.rs" | wc -l` reads 136; `src/application/notifications/`, `src/application/storage/`, the `subject/` directory, and the `admin/`/`user/` manager sub-trees are all confirmed absent; `src/core/` is confirmed exactly the six named files; `src/application/ports/` is confirmed absent. `REQ-libr-dead-reexport-removal` — never individually checked in run-4 — is upgraded to `satisfied`: `grep -c '^pub use' src/lib.rs` returns 10, and the five confirmed-consumer exception groups the requirement names (the four LLM provider adapters plus the mock pair, `Paladin`/`PaladinConfig`, `BattalionConfig`/`BattalionError`) are exactly what the current `src/lib.rs:158-188` block carries.
- Re-derived all six Milestone 8 Epic 3 rows with the three do-not-re-delete corrections carried into the record: `REQ-storage-shim-deletion` (satisfied) carries the sqlite marker, citing commit `897e77e` and the root manifest's non-optional `paladin-storage` dependency; `REQ-garrison-sanctum-bridges-kept` (satisfied) carries both the registry marker (`paladin_registry.rs` consolidated into `paladin-battalion` via commit `ca7e4e8`, not deleted) and this row's own factual correction (`api_content_deliverer.rs` is 724 LOC, not 629 — 629 belongs to `tensorflow_adapter.rs`); the epic note carries the third correction (the remaining Category 1 files — `mysql_content_repository.rs`, the five `input/*` fetchers, `document/*`, `api_content_deliverer.rs`, `error_log_adapter.rs` — genuinely were orphaned and were correctly removed in commit `e5b2011`).
- `REQ-adapter-disposition-record` (superseded by outcome) carries three separable facts on one row — the 20-row all-Stays table contradicted by the tree, two rows disagreeing with the governing PRD, and the two invented crate names `paladin-arsenal`/`paladin-sanctum` confirmed absent via `ls crates/` — and names Phase 11 / FACADE-04 as their owner so the two statements do not have to be rediscovered together.
- `REQ-m8-epic3-no-extractions` (superseded by outcome) re-measures the 15-commit range and the net 10,252-LOC removal independently this session (`git log --oneline e5b2011~1..a1e4901 | wc -l`, `git diff --shortstat e5b2011~1 a1e4901`), matching ADR-0028's own figures, and states agreement in substance with the head-of-file supersession summary table's existing entry under the same ID.
- `REQ-tensorflow-ml-feature-gate-v2` (superseded by outcome) confirms both the `ml` feature and `tensorflow_adapter.rs` were deleted outright by commit `3d48768`, and cross-references `REQ-deferred-tensorflow-ml-adapter-v3` (plan 10-10's range) by ID only, per this plan's instruction not to derive that row here.
- `REQ-notification-task-closeout` (satisfied) records both deletion halves: the application-layer `email_notifications.rs`/`push_notifications.rs`/`system_notifications.rs` were deleted as orphaned dead code in Epic 2 Batch 1 rather than moved as the M8 overview's own success criteria stated, and separately the facade's infrastructure-layer fallback duplicates (`email_notification_adapter.rs`, `system_notification_adapter.rs`) were deleted outright by commit `cf17559` — two distinct deletion events, both narrower than "moved".
- Confirmed the ledger's row/section inventory is unchanged: `grep -c '^| REQ-'` still reads `86`, `grep -c '^### '` still reads `12`, each of the two task commits shows equal added/deleted line counts (10/10, then 9/9), and the cumulative diff against the fork base reads 19/19 — cell replacement only, no row inserted, deleted or reordered, and the M8 Epic 4 section (owned by plan 10-01) is byte-unchanged.

## Task Commits

Each task was committed atomically:

1. **Task 1: Derive Milestone 8 Epic 1's four audit rows and Epic 2's four removal rows** — `0aed8ea` (feat)
2. **Task 2: Derive Milestone 8 Epic 3's six relocation rows with the three do-not-re-delete markers** — `1ac95fb` (feat)

_No plan-metadata commit: this executor ran in worktree mode. STATE.md and ROADMAP.md are owned by the orchestrator after all wave-3 agents complete; this SUMMARY.md is committed separately per the worktree execution contract._

## Files Created/Modified

- `.planning/ledgers/milestone-07-08.md` — Milestone 8 Epic 1 (4 rows), Epic 2 (4 rows) and Epic 3 (6 rows) Verdict/Evidence cells replaced in place; epic notes filled for all three sections. No other section touched (confirmed via `git diff` on each task commit, single hunks that do not cross into the M8 Epic 4 heading).

## Decisions Made

- **`REQ-facade-file-inventory`** is `present, unproven`, distinct from `REQ-facade-file-classification`'s `superseded by outcome`: the 189-file enumeration act happened and still exists at `facade-audit.md` Appendix A, but nothing exercises its completeness today and its List C conclusion is what is superseded — the two rows say which part survives rather than collapsing into one verdict.
- **`REQ-libr-dead-reexport-removal`** is upgraded to `satisfied` rather than left at the run-4 ledger's unactioned `Verify → HARD-01` stub, having actually been checked against `src/lib.rs`'s current ten `pub use` lines this session.
- **`REQ-storage-shim-deletion`** and **`REQ-garrison-sanctum-bridges-kept`** both carry `DO NOT RE-DELETE` markers directly in their Evidence cells, not only in ADR-0028, per D-08's instruction that a planner reads the ledger row before asking a question.
- **`REQ-garrison-sanctum-bridges-kept`** additionally carries the registry correction and the `api_content_deliverer.rs` LOC correction, per this plan's own explicit row assignment — an unusual placement (the registry marker's subject is a different file than the row's own garrison/sanctum topic) but the one the plan specified.
- **`REQ-adapter-disposition-record`** states all three separable facts (contradicted table, PRD disagreement, invented crate names) on one row and names Phase 11 / FACADE-04 by ID, so the connection does not have to be rediscovered.
- **`REQ-m8-epic3-no-extractions`** re-measures the 15-commit / 10,252-net-LOC figures independently rather than transcribing ADR-0028's prose, and explicitly states agreement with the head-of-file supersession summary table.
- The M8 Epic 3 epic note carries the third correction (the genuinely orphaned Category 1 remainder) since it applies epic-wide rather than to a single row, per this plan's instruction.

## Deviations from Plan

None — plan executed exactly as written.

One process note, not a deviation: the first draft of each task's epic note was written as a naturally line-wrapped paragraph (matching how the text was composed), which produced an unbalanced `git diff --numstat` (23/10 for Task 1's draft, 7/9 for Task 2's draft) against the single-line placeholder stubs plan 10-01 had scaffolded. Both were corrected by re-collapsing (Task 1) or re-splitting to match the original stub's line count (Task 2) before committing, so every commit's `git diff --numstat` shows added lines equal to deleted lines, per the plan's own acceptance criteria.

## Issues Encountered

None. All read_first sources (the ledger, ADR-0028, `facade-audit.md`, `infrastructure-adapter-disposition.md`, `prd-relocate-remaining-misplaced-modules.md`, `intel/requirements.md`, `intel/code-verification.md`, and the tree itself) were consulted directly through the worktree-relative path `/workspace/.claude/worktrees/agent-a2651e847a2d10583/...`, resolved via `git rev-parse --show-toplevel` per the worktree path warning, with no misdirected reads to the main checkout.

## User Setup Required

None — no external service configuration required.

## Known Stubs

None introduced by this plan.

## Next Phase Readiness

- Milestone 8 Epic 1, Epic 2 and Epic 3 are fully derived; plan 10-10 can proceed independently over its own disjoint epic range (M8 Epic 5-7 plus cross-milestone entries), with no file-content dependency beyond the shared row/section-count invariant, which this plan leaves unchanged at 86/12.
- Phase 11 / FACADE-04 now has a direct citation (this ledger's `REQ-adapter-disposition-record` row) naming the two non-existent crates it must resolve; Phase 11 / FACADE-02 and FACADE-03(b) have ADR-0028's Epic 3 completeness and non-goal-split rows to cite without re-deriving them.
- No blockers. No `.rs` file was touched; `git status --porcelain -- '*.rs'` is empty.

## Self-Check: PASSED

- FOUND: `.planning/ledgers/milestone-07-08.md`
- FOUND: commit `0aed8ea` (Task 1)
- FOUND: commit `1ac95fb` (Task 2)

---
*Phase: 10-milestone-7-8-ground-truth-recorded-account*
*Completed: 2026-08-08*
