---
phase: 10-milestone-7-8-ground-truth-recorded-account
plan: 01
subsystem: docs
tags: [ledger, requirements-traceability, adr-precedence, record-writing, milestone-7-8]

requires: []
provides:
  - ".planning/ledgers/milestone-07-08.md" — the fourth sibling ledger: head notes, seven-class verdict legend, 13-row "Superseded by outcome" summary table, twelve epic sections, 86 REQ-* row stubs, Milestone 8 Epic 4 fully derived
  - "REQUIREMENTS.md's Milestone 7-8 section reduced to a pointer at the ledger"
  - "four dated in-place corrections to HARD-01/03/05/07's own text, plus one to ROADMAP criterion 1"
affects: ["10-07", "10-08", "10-09", "10-10", "10-11"]

tech-stack:
  added: []
  patterns:
    - "Ledger scaffold-then-fan-out: wave-1 plan writes all 86 row stubs plus one fully-derived section end-to-end; wave-3 plans replace Verdict/Evidence cells in place within disjoint epic ranges"
    - "Inline strike-and-correct (Shape A) for dated corrections to requirement/roadmap text: ~~original~~ retained, followed by a bolded 'Corrected (dated ..., HARD-NN):' clause naming the re-run command or file:line"

key-files:
  created:
    - .planning/ledgers/milestone-07-08.md
  modified:
    - .planning/REQUIREMENTS.md
    - .planning/ROADMAP.md

key-decisions:
  - "Built the 13-row 'Superseded by outcome' summary table directly from intel/code-verification.md:365-381 (re-counted this session: 15 lines = 1 header + 1 separator + 13 data rows), not from the '14-row' figure HARD-01's own text asserted — corrected that figure in five places (three in REQUIREMENTS.md, one in ROADMAP.md, matching the summary table's own preamble)"
  - "Mapped each of the 13 supersession-table rows to its owning REQ-* ID by cross-referencing the 86-row run-4 ledger's verdict text; flagged three rows (†) as mdbook relocations that this ledger's own tie-break rule will later classify `relocated` rather than `superseded by outcome`, and two rows (‡) as HARD-05/HARD-06 subjects whose wave-3 ADRs are expected to flip them to `satisfied`, so a future reader doesn't mistake the summary table's pre-ADR snapshot for a final verdict"
  - "Fully derived Milestone 8 Epic 4's four rows (satisfied, satisfied, present-unproven, satisfied) by re-running every citation this session rather than carrying forward the run-4 'Verify → HARD-01' status word, per the evidence bar's explicit instruction not to reach for `satisfied` without something exercising the claim"
  - "For REQ-rename-doc-updates, declined `satisfied` and recorded `present, unproven`: three of five current-facing markdown hits are intentional migration-guide documentation of the old path (correct usage), but .github/copilot-instructions.md's architecture diagram is a genuine unannotated stale reference outside this plan's D-23 boundary to fix"

requirements-completed: [HARD-01, HARD-03, HARD-05, HARD-07]

coverage:
  - id: D1
    description: "Milestone 7-8 ledger scaffold created with head notes, legend, 13-row supersession summary table, twelve epic sections, and 86 REQ-* row stubs"
    requirement: "HARD-01"
    verification:
      - kind: other
        ref: "grep -c '^| REQ-' .planning/ledgers/milestone-07-08.md == 86; grep -o '^| REQ-[a-z0-9-]*' | sort -u | wc -l == 86; grep -c '^### ' == 12"
        status: pass
    human_judgment: false
  - id: D2
    description: "Milestone 8 Epic 4's four rows fully derived end-to-end with re-run citations and named exercising commands"
    requirement: "HARD-01"
    verification:
      - kind: other
        ref: "grep -A6 '`use_cases` → `services` Rename' .planning/ledgers/milestone-07-08.md | grep -c 'pending — plan' == 0"
        status: pass
    human_judgment: false
  - id: D3
    description: "REQUIREMENTS.md's inline 86-row Milestone 7-8 ledger reduced to a pointer at the new ledger file, no diverging second copy left behind"
    requirement: "HARD-01"
    verification:
      - kind: other
        ref: "awk '/^## Milestone 7-8 as-shipped ledger/{f=1;next}/^## /{f=0}f' .planning/REQUIREMENTS.md | grep -c '^| REQ-' == 0"
        status: pass
    human_judgment: false
  - id: D4
    description: "Four dated in-place corrections to HARD-01/03/05/07's own stale text, plus one to ROADMAP criterion 1, with original text retained struck"
    requirement: "HARD-03"
    verification:
      - kind: other
        ref: "grep -c 'Corrected (dated 2026-08-08, HARD-' .planning/REQUIREMENTS.md == 5; grep -c 'Corrected (dated 2026-08-08' .planning/ROADMAP.md == 1"
        status: pass
    human_judgment: false
  - id: D5
    description: "No .rs file modified by this plan (D-23 boundary held)"
    verification:
      - kind: other
        ref: "git status --porcelain -- '*.rs' — empty"
        status: pass
    human_judgment: false

duration: ~55min
completed: 2026-08-08
status: complete
---

# Phase 10 Plan 01: Milestone 7-8 Ledger Scaffold & Requirement-Text Corrections Summary

**Created the fourth sibling status ledger (86 REQ-* row stubs, one epic fully derived end-to-end) and corrected five stale figures/citations inside HARD-01/03/05/07's own text and ROADMAP criterion 1 — no `.rs` file touched.**

## Performance

- **Duration:** ~55 min
- **Completed:** 2026-08-08
- **Tasks:** 3
- **Files modified:** 3 (1 created, 2 modified)

## Accomplishments

- Created `.planning/ledgers/milestone-07-08.md` — head notes (supersession, primary key, evidence bar, manifest carve-out with re-grepped CI anchors, path caveats, workspace-shape provenance, checkbox corroboration), the seven-class verdict legend with the HARD-01 mapping callout and tie-break rule, a 13-row "Superseded by outcome" summary table built directly from `intel/code-verification.md:365-381`, the row-order/amendment convention with the five-plan ledger contention table, and all 86 `REQ-*` row stubs across the twelve run-4 epic sections in their original order.
- Fully derived Milestone 8 Epic 4's four rows (`REQ-use-cases-services-rename`, `REQ-rename-clean-break`, `REQ-rename-doc-updates`, `REQ-rename-changelog-breaking`) end-to-end, re-running every citation this session — the workspace-wide `use_cases` grep, the `pub use services as use_cases` absence check, and a cross-check of `CHANGELOG.md:670-686`'s migration table against `ls src/application/services/`.
- Reduced `REQUIREMENTS.md`'s `## Milestone 7-8 as-shipped ledger` section (previously ~196 lines, 86 inline rows) to a pointer at the new ledger file, removing 177 lines from `REQUIREMENTS.md` (4136 → 3959) with no rows left behind.
- Corrected the "14-row"/"fourteen" superseded-by-outcome figure to 13 in five places (three in `REQUIREMENTS.md`, one in `ROADMAP.md` criterion 1, matching the ledger's own summary-table preamble), and corrected HARD-03's stale current-state clause, HARD-05's stale citation and version quote, and HARD-07's stale `ci.yml:225` citation — all five as dated, in-place, strike-and-retain corrections.

## Task Commits

Each task was committed atomically:

1. **Task 1: Create the Milestone 7-8 ledger** — `0f33ddc` (feat)
2. **Task 2: Reduce REQUIREMENTS.md's inline ledger to a pointer** — `e3a476a` (docs)
3. **Task 3: Correct stale figures in HARD-01/03/05/07 and ROADMAP criterion 1** — `fe06cee` (docs)

_No plan-metadata commit: per the worktree execution contract, STATE.md and ROADMAP.md progress-tracking updates are owned by the orchestrator after all wave agents complete. The ROADMAP.md edit in Task 3 is a content correction to Phase 10's own success criterion, not a progress update, and is committed within Task 3 above._

## Files Created/Modified

- `.planning/ledgers/milestone-07-08.md` — new ledger file, 347 lines, 86 rows across 12 sections
- `.planning/REQUIREMENTS.md` — Milestone 7-8 section reduced to a pointer; five dated corrections applied
- `.planning/ROADMAP.md` — one dated correction to Phase 10's success criterion 1

## Decisions Made

- Built the summary table from the counting command's actual output (13 data rows), not from the "14" figure asserted in three places in `REQUIREMENTS.md` — corrected all three plus the matching ROADMAP figure, recording the counting command verbatim in each correction per D-25.
- For the summary table's 13 rows, mapped each to its owning `REQ-*` ID(s) by cross-referencing the run-4 86-row ledger text rather than inventing IDs (D-00f); flagged rows whose eventual epic-section verdict will differ from "superseded by outcome" once the tie-break rule or a wave-3 ADR applies, so the table's pre-fan-out snapshot isn't mistaken for a final verdict.
- Declined `satisfied` for `REQ-rename-doc-updates` (recorded `present, unproven` instead) because the "57 markdown references" claim wasn't exhaustively re-checked and one genuine stale reference (`.github/copilot-instructions.md`'s architecture diagram) surfaced outside this plan's D-23 config-surface boundary — following the plan's explicit instruction not to reach for `satisfied` without a real exercising check.
- Used the inline strike-and-correct shape (Pattern A from `10-PATTERNS.md`) for all five text corrections: original phrase retained inside `~~...~~`, followed by a bold `**Corrected (dated 2026-08-08, HARD-NN):**` clause naming the re-run command or `file:line`.

## Deviations from Plan

None — plan executed exactly as written. Two self-corrections were made during execution before committing (not deviations from the plan, but formatting fixes to my own draft):

1. Initial table rows used backtick-wrapped `REQ-*` IDs (e.g. `` | `REQ-xxx` | ``), which failed the plan's own `grep -c '^| REQ-'` acceptance check (the ID column must be unwrapped to match `^| REQ-`, matching `milestone-04-06.md`'s own row format). Fixed via a scoped script that stripped backticks only from per-epic-table ID cells (leaving the summary table's backtick-wrapped IDs untouched, since those must *not* match the same grep pattern). Fixed before the Task 1 commit.
2. Epic section headings were initially written as `##` instead of the required `###`, and the summary table's own header row (`| REQ-* ID | ... |`) briefly false-matched the row-counting grep. Both fixed before the Task 1 commit; final `grep -c '^| REQ-'` reads exactly 86.

## Issues Encountered

None beyond the two self-corrections above, both caught and fixed by the plan's own automated verification before committing.

## User Setup Required

None — no external service configuration required.

## Known Stubs

82 of the 86 ledger rows carry `pending — plan 10-NN` verdicts by design — this is the wave-1 scaffold's explicit contract (D-27): plan 10-01 creates the structure and fully derives one section (M8 Epic 4, 4 rows) to prove the mechanism; plans 10-07 through 10-10 (wave 3) replace the remaining 82 rows' Verdict/Evidence cells in place within their own disjoint epic ranges. This is not a defect — it is the documented decomposition — but is recorded here for the verifier's visibility per the stub-tracking convention. No `.planning/WINDOWS.md` entry was appended for these: `pending — plan 10-NN` stubs with a named owning plan in the same phase are the scaffold's intended interim state, not an unresolved defect the ship gate should block on; they are all closed by the end of Phase 10 wave 3 (plans already exist and are scheduled).

## Next Phase Readiness

- The ledger structure, legend, and contention table are in place; plans 10-07, 10-08, 10-09 and 10-10 can proceed in parallel over their disjoint epic ranges once their own wave order is reached.
- `REQUIREMENTS.md` and `ROADMAP.md` corrections are in place for HARD-01/03/05/07; HARD-02, HARD-04 and HARD-06's own text corrections are out of this plan's scope (owned by wave-3 plans 10-07/10-08's ADR-adjacent work per the phase's plan index).
- No blockers. No `.rs` file was touched; `git status --porcelain -- '*.rs'` is empty, satisfying this plan's phase-note acceptance criterion.

---
*Phase: 10-milestone-7-8-ground-truth-recorded-account*
*Completed: 2026-08-08*
