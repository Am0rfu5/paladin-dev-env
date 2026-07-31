---
phase: 01-ground-truth-decision-records
plan: 11
subsystem: docs
tags: [requirements-ledger, adr, planning-corpus, gap-closure]

# Dependency graph
requires:
  - phase: 01-ground-truth-decision-records (plans 01-02, 01-07, 01-08)
    provides: ADR-0002 (BattalionResult field set), the Milestone 1 ledger's Epic 4-10
      tables, and the halted subset-check finding that REQ-battalion-result-v1 was
      missing from milestone-01.md
provides:
  - "A REQ-battalion-result-v1 row in the Milestone 1 ledger's Epic 4 table, sourced
    from ADR-0002's Considered Options, at the pinned position between
    REQ-battalion-config-v2 and REQ-battalion-error-strategy"
  - "A corrected verdict-distribution table (superseded-by-shipped-code bucket and
    total each +1) that still sums to its stated total"
  - "REQUIREMENTS.md's Milestone 1 as-shipped ledger body reduced to a short pointer
    at .planning/ledgers/milestone-01.md, now that the destination is a proven
    superset"
  - "WINDOWS.md open item 1 closed by repair (open_count: 0)"
affects: [phase-01-completion, gsd-ship (WINDOWS.md open_count gate)]

# Tech tracking
tech-stack:
  added: []
  patterns:
    - "Re-run the identifier subset check inside the same task that performs a
      pointer-reduction, immediately before it, rather than trusting an earlier
      run's result — closes the window in which the destination could have changed"

key-files:
  created: []
  modified:
    - .planning/ledgers/milestone-01.md
    - .planning/REQUIREMENTS.md
    - .planning/WINDOWS.md

key-decisions:
  - "Sourced the new ledger row's evidence verbatim from ADR-0002's Considered Options rather than re-deriving it, per the plan's citation-discipline requirement"
  - "Re-verified the battalion/mod.rs:549 citation against the live tree before writing it (still accurate, unchanged)"
  - "Used row-based REQ-* extraction (one token per ID-column row, matching plan 01-08's own method) for the source-set size check, since expanding the three v1/v2-combined shorthand rows would have produced 115 rather than the ~112 the guardrail expects"

requirements-completed: [RECON-01, RECON-03]

coverage:
  - id: D1
    description: "Milestone 1 ledger carries a superset row for REQ-battalion-result-v1 (Epic 4), sourced from ADR-0002, at the correct pinned position, with the verdict-distribution table's arithmetic corrected"
    requirement: "RECON-01"
    verification:
      - kind: other
        ref: "grep -c REQ-battalion-result-v1 .planning/ledgers/milestone-01.md (returns 1); manual arithmetic check 100+23+11+20+1=155"
        status: pass
    human_judgment: false
  - id: D2
    description: "REQUIREMENTS.md's Milestone 1 ledger body re-checked and safely reduced to a pointer at the now-complete destination"
    requirement: "RECON-01"
    verification:
      - kind: other
        ref: "python3 subset-check script: 112 source rows, 0 missing from destination (122 distinct destination identifiers)"
        status: pass
    human_judgment: false
  - id: D3
    description: "WINDOWS.md open item 1 closed via gsd-tools windows fixed 1, after (not instead of) the repair"
    requirement: "RECON-01"
    verification:
      - kind: other
        ref: "node .claude/gsd-core/bin/gsd-tools.cjs windows fixed 1; grep open_count: 0 / fixed_count: 1 .planning/WINDOWS.md"
        status: pass
    human_judgment: false

# Metrics
duration: ~20min
completed: 2026-07-31
status: complete
---

# Phase 1 Plan 11: Close the REQ-battalion-result-v1 Ledger Gap Summary

**Added the one missing ledger row plan 01-08 correctly refused to paper over, re-ran its subset-check safety gate independently (112/112 source identifiers present, zero missing), completed the REQUIREMENTS.md pointer reduction that check now permits, and closed the Broken Windows entry by repair.**

## Performance

- **Duration:** ~20 min
- **Completed:** 2026-07-31
- **Tasks:** 3 of 3
- **Files modified:** 3 (`milestone-01.md`, `REQUIREMENTS.md`, `WINDOWS.md`)

## Accomplishments

- **Task 1:** Added `REQ-battalion-result-v1` to the Milestone 1 ledger's Epic 4 table, between `REQ-battalion-config-v2` and `REQ-battalion-error-strategy`, with verdict `superseded by shipped code` and evidence transcribed from ADR-0002's `## Considered Options` ("wholly subsumed" — every Epic-4 field present in the shipped `BattalionResult` superset, no substitution to record). Re-verified `battalion/mod.rs:549` against the live tree (still accurate). Corrected the verdict-distribution table: `superseded by shipped code` 19 → 20, total 154 → 155, REQ-row label 112 → 113; the other four buckets and the 39-nested-item reconciliation were left untouched. `git diff --numstat`: 3 insertions / 2 deletions, well under the plan's 15-line bound.
- **Task 2:** Re-ran the identifier subset check independently (not trusting Task 1's own claim, plan 01-08's prior run, or any SUMMARY figure). Extracted 112 `REQ-*` row identifiers from REQUIREMENTS.md's Milestone 1 ledger body (one per `| REQ-... |` table row — the same row-count methodology plan 01-08 used, which is why the three `v1 / -v2` combined-shorthand rows count once each rather than expanding to two identifiers apiece) and 122 distinct `REQ-*` identifiers from `.planning/ledgers/milestone-01.md`. All 112 source identifiers are present in the destination — **zero missing**. Both sets are non-empty and the source-set size (112) exactly matches the count plan 01-08 recorded, satisfying the "within one" guardrail with no drift. With the check passed, reduced REQUIREMENTS.md's `## Milestone 1 as-shipped ledger` body (previously ~178 lines across ten per-epic tables) to a 9-line pointer paragraph naming the destination file, the reason for the move (D-17), the destination's own row/item counts (113 `REQ-*` rows, 39 nested items) as Task 1 left them, and the four future sibling ledgers.
- **Task 3:** Ran `node .claude/gsd-core/bin/gsd-tools.cjs windows fixed 1` after Tasks 1-2 completed. `WINDOWS.md`'s frontmatter now reads `open_count: 0`, `fixed_count: 1`; item 1's markdown row and trailing JSON block both show `status: fixed` with a matching `resolved_at` timestamp and an unchanged `description`; the `reason` field is empty (a repair, not a waiver).

## Task Commits

1. **Task 1: Add the missing REQ-battalion-result-v1 row to the Milestone 1 ledger's Epic 4 table** - `4a8dee3` (docs)
2. **Task 2: Re-run the subset check and reduce REQUIREMENTS.md's Milestone 1 ledger body to a pointer** - `2ec2f9d` (docs)
3. **Task 3: Mark Broken Windows item 1 fixed, now that the repair exists** - `47b90ea` (docs)

**Plan metadata:** this SUMMARY's own commit (see below)

## Files Created/Modified

- `.planning/ledgers/milestone-01.md` - Added the `REQ-battalion-result-v1` Epic 4 row and corrected the verdict-distribution table's `superseded by shipped code` bucket and total by exactly one.
- `.planning/REQUIREMENTS.md` - `## Milestone 1 as-shipped ledger` body (status-key legend + ten per-epic tables) reduced to a short pointer paragraph at `.planning/ledgers/milestone-01.md`. Heading, summary-table row near line 31, the four sibling as-shipped ledger headings, and all 30 variant-group sections are untouched.
- `.planning/WINDOWS.md` - Item 1 marked `fixed` via the ledger's own closure command; `open_count` now 0.

## Decisions Made

- **Sourced the new row's evidence from ADR-0002 verbatim**, per this phase's citation discipline — no evidence was authored fresh.
- **Re-ran the subset check from scratch inside this plan** rather than trusting Task 1's own edit, plan 01-08's earlier (failed) run, or any prior SUMMARY figure, per the plan's explicit "backstop" truth and the concurrency threat (T-01-50) in its threat model.
- **Used row-based (not fully-expanded) REQ-* extraction for the source-set count**, matching plan 01-08's own methodology, so the size-guardrail comparison (~112 ± 1) was apples-to-apples. A naive full-text regex extraction across the source body's prose (which also matches forward-reference IDs mentioned in evidence text, e.g. `REQ-sanctum-port`) produced 125 — clearly the wrong denominator, since it counts identifiers that were never actually Milestone-1 ledger rows. Restricting extraction to each row's own ID-column token reproduced exactly 112, confirming this was the intended methodology.

## Deviations from Plan

None — plan executed exactly as written. The subset check passed on this run (unlike plan 01-08's), which was the expected and designed outcome once Task 1's row existed; this is not a deviation, it is the plan's stated purpose.

---

**Total deviations:** 0 auto-fixed.
**Impact on plan:** No scope creep. All three tasks executed to their full stated scope, in the mandatory order (row → re-check → reduce → close register).

## Known Stubs

None.

## Issues Encountered

None. The one thing the plan anticipated as a possible failure mode — the subset check failing again for a different reason — did not occur; the check passed cleanly with 112/112 identifiers accounted for.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- **`.planning/WINDOWS.md` now reports `open_count: 0`**, removing the one register-level blocker on `/gsd-ship` that this plan owned.
- **`.planning/ledgers/milestone-01.md` is now a true superset of every `REQ-*` identifier REQUIREMENTS.md's Milestone 1 body carried** — the gap plan 01-08 found and correctly refused to paper over is closed.
- No further action needed on this specific gap; the ledger's 39-nested-item reconciliation, its four sibling ledgers (Phases 5, 7, 10, 13), and REQUIREMENTS.md's forward requirements and variant groups are all unaffected and untouched by this plan.

---
*Phase: 01-ground-truth-decision-records*
*Completed: 2026-07-31*

## Self-Check: PASSED

- FOUND: `.planning/ledgers/milestone-01.md`
- FOUND: `.planning/REQUIREMENTS.md`
- FOUND: `.planning/WINDOWS.md`
- FOUND: `.planning/phases/01-ground-truth-decision-records/01-11-SUMMARY.md`
- FOUND: commit `4a8dee3`
- FOUND: commit `2ec2f9d`
- FOUND: commit `47b90ea`
- FOUND: commit `27f448b`
