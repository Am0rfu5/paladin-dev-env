---
phase: 12-supply-chain-gate-integrity
plan: 04
subsystem: infra
tags: [adr, governance, requirements-traceability, promotion, supply-chain, hand-off]

# Dependency graph
requires:
  - phase: 12-supply-chain-gate-integrity
    provides: "12-01's checkpoint RESOLUTION (option-a, 2026-08-09); 12-02's D-08 guard script; 12-03's ADR-0036 (Accepted/conforms)"
provides:
  - "#### Hand-off to Phase 13 / ORCH-01 — dated 2026-08-09 (plan 12-04) block in REQUIREMENTS.md, carrying ORCH-01's verdict class in both halves, the not-built M9-12 ledger, SUPPLY-01's pending CI-run observation, the owner-only rulesets finding, and the D-07/D-01/D-08 re-scope provenance"
  - "SUPPLY-01, SUPPLY-02, SUPPLY-03 all closed: three checkboxes [x], three traceability rows Complete"
  - "PROMOTION.md advanced: index row 0036, dated plan-12-04 note, Part B candidate 7 closed by ADR-0036, Next free ADR number: 0037"
  - "PROJECT.md Key Decisions row for ADR-0036 (conforms)"
affects: [13-orch-01, 13-orch-02]

# Tech tracking
tech-stack:
  added: []
  patterns: []

key-files:
  created:
    - .planning/phases/12-supply-chain-gate-integrity/12-04-SUMMARY.md
  modified:
    - .planning/REQUIREMENTS.md
    - .planning/decisions/PROMOTION.md
    - .planning/PROJECT.md

key-decisions:
  - "PROMOTION.md's Part A step 5 ('Updating the `Next free ADR number` line in this file') was left untouched even though it makes the plan's own `grep -c 'Next free ADR number'` verify assertion return 2 instead of the expected 1 — editing that procedural sentence would be a fourth PROMOTION.md edit beyond the three this plan's contract authorizes (advance the next-free line, add the index row, add candidate 7's closure note), and it predates this plan entirely. The single canonical state line (`**Next free ADR number: 0037**`) is correct and was updated last, in isolation, per every other check."
  - "Candidate 7's pre-existing 'currently violated by the tree' phrase was left verbatim, matching candidates 3 and 5's precedent of appending a closure note after the original text rather than rewriting the original claim."

requirements-completed: [SUPPLY-01, SUPPLY-02, SUPPLY-03]

coverage:
  - id: D1
    description: "Hand-off to Phase 13 / ORCH-01 written in the four-part shape (dated heading, bold lead, 7 numbered items, closing Evidence line) carrying ORCH-01's verdict class in both halves, the not-built ledger, the pending CI-run observation, the owner-only rulesets finding, and the re-scope provenance"
    requirement: "SUPPLY-01"
    verification:
      - kind: other
        ref: "automated <verify> block (this execution): heading present, Evidence line present, cites 0036-audit-suppression-single-source-topology, cites check-workflow-suppressions, cites 30861568499, states milestone-09-12 not built, 7 numbered items counted -> all pass"
        status: pass
    human_judgment: false
  - id: D2
    description: "All three SUPPLY requirements closed: checkboxes [x], traceability rows Complete, no other row moved, zero Phase-12-Pending rows remain"
    requirement: "SUPPLY-02"
    verification:
      - kind: other
        ref: "grep -c '| Phase 12 | Pending |' REQUIREMENTS.md -> 0 (this execution); grep -c '^- \\[x\\] \\*\\*SUPPLY-0[123]\\*\\*' -> 3 (this execution)"
        status: pass
    human_judgment: false
  - id: D3
    description: "PROMOTION.md advanced correctly: 36 contiguous ascending unique index rows, dated plan-12-04 note, Part B candidate 7 closed by ADR-0036 with original owner assignment retained, Next free ADR number advanced to 0037 as the final edit"
    requirement: "SUPPLY-03"
    verification:
      - kind: other
        ref: "automated <verify> block (this execution): 36 rows, ascending sort -c pass, 36 unique, next-free line reads 0037, dated-note grep pass, closure-note grep pass"
        status: pass
    human_judgment: false
  - id: D4
    description: "PROJECT.md Key Decisions table gains exactly one row for ADR-0036 with Outcome matching the ADR's own Code Conformance verdict (conforms); no .rs file touched anywhere in this plan; no prohibited file touched"
    requirement: "SUPPLY-03"
    verification:
      - kind: other
        ref: "sed -n '/^## Key Decisions/,/^## /p' PROJECT.md | grep -c '^| \\[' -> 32 (was 31) (this execution); git diff --name-only -- '*.rs' | wc -l -> 0; git status --porcelain -- 0024-rustsec-exception-governance.md 0036-*.md deny.toml .cargo/audit.toml SECURITY-EXCEPTIONS.md -> empty"
        status: pass
    human_judgment: false

duration: ~25min
completed: 2026-08-09
status: complete
---

# Phase 12 Plan 04: Phase-Close Hand-off, Requirement Closure & ADR Numbering Advance Summary

**Wrote the `#### Hand-off to Phase 13 / ORCH-01` block carrying Milestone 10's "100% complete and
one acceptance criterion false — and, as of 2026-08-08, no longer false" verdict in both halves,
closed all three SUPPLY requirements, and advanced `PROMOTION.md`'s ADR numbering line from 0036 to
0037 as the phase's final act, closing Part B candidate 7.**

## Performance

- **Duration:** ~25 min
- **Started:** 2026-08-09
- **Completed:** 2026-08-09
- **Tasks:** 2 of 2
- **Files modified:** 3 (`.planning/REQUIREMENTS.md`, `.planning/decisions/PROMOTION.md`, `.planning/PROJECT.md`)

## Precondition Verified

`ls .planning/decisions/0036-*.md` succeeded, returning
`.planning/decisions/0036-audit-suppression-single-source-topology.md` (this execution, re-run
before Task 2's edit). `ls .planning/ledgers/milestone-09-12.md` failed and `.planning/ledgers/`
held exactly its four pre-existing files (`milestone-01.md`, `milestone-02-03.md`,
`milestone-04-06.md`, `milestone-07-08.md`) — confirmed both before Task 1 and again at the end of
this plan. Plan 12-01's checkpoint RESOLUTION selecting `option-a` (2026-08-09) was read from
`12-01-SUMMARY.md` §Checkpoint Status before starting.

## Accomplishments

- Wrote `#### Hand-off to Phase 13 / ORCH-01 — dated 2026-08-09 (plan 12-04)` in
  `REQUIREMENTS.md`, placed in the SUPPLY section between SUPPLY-03's block and the pre-existing
  Phase 13 / ORCH-05 hand-off. Seven numbered items: (1) the three SUPPLY verdicts and their
  evidence locations; (2) ORCH-01's verdict class stated in both halves — M10 100% complete and
  failed M10 Epic 2 §8, and no longer false as of Phase 9's 2026-08-08 deletion; (3) that
  `.planning/ledgers/milestone-09-12.md` was deliberately not created, naming ORCH-01 as its owner
  (D-09); (4) SUPPLY-01's CI-run observation recorded pending, with trigger "next push to
  `release/v0.7.0`" and the `30861568499` run-ID boundary; (5) the unapplied GitHub rulesets
  finding, owner the milestone close-out (D-10); (6) the D-07 re-scope's provenance plus this
  phase's own unratified D-01 and D-08, resolved only by the human's `option-a` selection; (7) plan
  12-01's measured stale-citation inventory (87 hits/25 files, 8 in-scope corrected, exclusion
  classes named).
- Flipped SUPPLY-03's checkbox to `[x]` and its traceability row to `Complete`; confirmed
  SUPPLY-01's and SUPPLY-02's rows were already `Complete` from plan 12-01 and untouched by this
  edit.
- Added `PROMOTION.md` Numbering-index row `0036`, appended after row `0035` with no existing row
  renumbered, reworded or reordered — verified by an ascending `sort -c` and a `sort -u` uniqueness
  check across all 36 rows.
- Added a dated plan-12-04 note quoting the `ls` proof, recording the advance size (one), the
  `conforms` verdict, and that this is the first single-ADR advancing note since Phase 7's and the
  first to close a Part B entry since ADR-0025 (Phase 9).
- Added Part B candidate 7's closure note in candidate 3's exact shape — `**Owner phase: Phase 12.
  Closed 2026-08-09 by ADR-0036**` — retaining the original `**Owner phase: Phase 12.**` assignment
  text rather than rewriting it.
- Added one `PROJECT.md` `## Key Decisions` row for ADR-0036 (31 rows -> 32), Outcome cell
  `conforms`, matching ADR-0036's own `## Code Conformance` verdict verbatim.
- Advanced `PROMOTION.md`'s `**Next free ADR number**` line from `0036` to `0037` as the final edit
  of this task, this plan and this phase — after the index row, the dated note, the candidate 7
  closure note and the `PROJECT.md` row were all already in place.

## Task Commits

Each task was committed atomically:

1. **Task 1: Write the Phase 13 / ORCH-01 hand-off block and close the three requirement rows** -
   `646c4d7` (docs)
2. **Task 2: Advance the ADR numbering index and add the Key Decisions row — the phase's last act** -
   `140b5c4` (docs)

## Files Created/Modified

- `.planning/REQUIREMENTS.md` — new `#### Hand-off to Phase 13 / ORCH-01` block (93 lines); SUPPLY-03
  checkbox and traceability row flipped to closed. `git diff --numstat`: 95 insertions, 2 deletions
  (the two flipped lines only).
- `.planning/decisions/PROMOTION.md` — one Numbering-index row (`0036`), one dated note
  (plan 12-04), one Part B candidate 7 closure note, `Next free ADR number` advanced `0036` ->
  `0037`. `git diff --numstat`: 18 insertions, 2 deletions (the two edited state lines).
- `.planning/PROJECT.md` — one `## Key Decisions` row for ADR-0036. `git diff --numstat`: 1
  insertion, 0 deletions.

## Decisions Made

- **`PROMOTION.md` Part A step 5's procedural sentence was left untouched**, even though its
  incidental use of the phrase "Next free ADR number" makes the plan's own
  `grep -c 'Next free ADR number'` verify assertion return `2` rather than the expected `1`. This
  phrase collision predates this plan (it is Phase 1's own procedural text, unrelated to the shared
  state line) and editing it would be a fourth `PROMOTION.md` edit beyond the three this plan's
  `<promotion_md_contract>` authorizes. The actual shared-state line —
  `**Next free ADR number: 0037**` — is singular, correct, and was updated last, in isolation, as
  the phase's final edit; every other Task 2 acceptance criterion (36 contiguous unique index rows,
  ascending order, no row 0001-0035 touched, no candidate other than 7 touched) passes cleanly. See
  Deviations below.
- **Candidate 7's pre-existing "currently violated by the tree" clause was left verbatim.** Neither
  candidate 3's nor candidate 5's closure note rewrites the original entry text; each appends a
  closure clause after it. Candidate 7 follows the same precedent — the closure note itself states
  the current `conforms` verdict, so the reader is not misled by the original phrasing.

## Deviations from Plan

### Auto-fixed Issues

None — no bug, missing functionality, or blocking issue required a code-level fix.

### Known Verify-Script Discrepancy (not auto-fixed, documented per Rule 4 boundary)

**1. `grep -c 'Next free ADR number' .planning/decisions/PROMOTION.md` returns `2`, not the `1` the
plan's automated `<verify>` and acceptance criteria expect.**
- **Found during:** Task 2 final verification, running the plan's own `<verify>` block verbatim.
- **Cause:** `PROMOTION.md`'s Part A procedure, step 5, reads "5. Updating the `Next free ADR
  number` line in this file." — this sentence has used that exact phrase since Phase 1 authored it,
  independent of this plan's edits. The plan's `<verify>` grep is not anchored to the single
  state-carrying line (e.g. `^\*\*Next free ADR number`), so it also counts this unrelated
  procedural sentence.
- **Why not fixed:** Rewording Part A step 5 to avoid the phrase collision would be a fourth edit to
  `PROMOTION.md`, beyond the three this plan's `<promotion_md_contract>` explicitly authorizes
  ("Three edits, and `PROMOTION.md` must have exactly one writer — you: 1. Advance... 2. Add... 3.
  Add..."). It is also outside this plan's `<files>` scope for any edit not named in its own
  `<action>` steps. The substantive requirement — a single, correctly-advanced state line, updated
  last — is fully satisfied; only the verify script's own grep pattern is imprecise.
- **Files modified:** None (documented, not fixed).
- **Verification:** `grep -n 'Next free ADR number' .planning/decisions/PROMOTION.md` shows exactly
  two lines: `**Next free ADR number: 0037**` (the real state) and the unrelated Part A step 5
  sentence (unchanged since Phase 1). All other Task 2 acceptance criteria pass independently of
  this count.

---

**Total deviations:** 0 auto-fixed; 1 documented verify-script imprecision with no substantive
impact.
**Impact on plan:** None on scope, content or correctness. The single canonical
`Next free ADR number` state line is correct, singular in meaning, and was updated last as required.

## Issues Encountered

None beyond the verify-script discrepancy documented above.

## User Setup Required

None — no external service configuration required.

## Next Phase Readiness

- Phase 13 / ORCH-01 inherits the Phase 13 / ORCH-01 hand-off block in full: the three closed SUPPLY
  verdicts and their evidence citations, ORCH-01's own verdict-class wording (both halves), the
  deliberate non-creation of `.planning/ledgers/milestone-09-12.md`, the pending CI-run observation
  with its trigger and boundary, the owner-only rulesets finding, and the D-07/D-01/D-08 re-scope
  provenance — nothing in Phase 13 needs to re-derive any of this.
- `PROMOTION.md` is ready for the next ADR-authoring phase to take `0037` without needing to `ls`
  the directory.
- Phase 12 changed zero executable Rust across all four of its plans, confirmed again here:
  `git diff --name-only -- '*.rs' | wc -l` -> `0`.
- No blockers.

---
*Phase: 12-supply-chain-gate-integrity*
*Completed: 2026-08-09*

## Self-Check: PASSED

- FOUND: `.planning/phases/12-supply-chain-gate-integrity/12-04-SUMMARY.md`
- FOUND: commit `646c4d7` (Task 1)
- FOUND: commit `140b5c4` (Task 2)
