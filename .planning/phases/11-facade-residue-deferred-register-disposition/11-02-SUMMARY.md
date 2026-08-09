---
phase: 11-facade-residue-deferred-register-disposition
plan: 02
subsystem: docs
tags: [adr, decision-record, facade-residue, deferred-items, disposition]

# Dependency graph
requires:
  - phase: 11-facade-residue-deferred-register-disposition (plan 01)
    provides: ADR number allocation confirmed (option-a, ADR-0034 free) via checkpoint decision
provides:
  - "ADR-0034: four verdicts (D1-D4) with owners and triggers, replacing effort/risk ratings"
  - "Dated corrections on deferred-items.md's D1-D4 clauses and Suggested grouping section"
affects: [11-05 (PROMOTION.md numbering, ledger amendment), Phase 15 (DEFER-02 sizing, no-alias sweep, builder/execution refactor, dependency-coupling review), run-3 v2 tech-debt item]

# Tech tracking
tech-stack:
  added: []
  patterns: ["ADR shape: no frontmatter, 7 fixed H2 headings", "Shape-A inline dated correction banner (annotation, never rewrite)"]

key-files:
  created:
    - .planning/decisions/0034-d1-d4-facade-relocation-disposition.md
  modified:
    - .project/Milestone_8-Facade-Cleanup-Shim-Resolution/deferred-items.md

key-decisions:
  - "ADR-0034: D1 defers to a facade-wide no-alias sweep (owner unassigned-pending); ADR-0018 does not settle D1 because it retired paths for relocated types, a different construct from src/core/'s surviving re-export layer"
  - "D2's user_service.rs split is WITHDRAWN (three-owner split: split owned by nobody, full relocation owned by run-3 v2 tech-debt item, tests owned by DEFER-02/Phase 15); content_service.rs and event_manager.rs each get an independent DEFER verdict"
  - "D3 and D4 defer to their named triggers (broader builder/execution refactor; dependency-coupling review), gated on ADR-0031's per-edge test since HARD-05 is already answered"
  - "ADR-0031's unratified '⚠ HUMAN REVIEW' status is carried forward explicitly rather than cited as settled authority"

patterns-established:
  - "Shape-A dated correction: append a blockquote after the existing clause, pointing at the governing ADR, with zero deletions of original text"

requirements-completed: [FACADE-02]

coverage:
  - id: D1
    description: "ADR-0034 authored with seven headings, four verdicts (D1-D4) each with a verb, owner, and (where deferred) a concrete trigger; content_service.rs and event_manager.rs given independent verdicts; ADR-0031's unratified status recorded"
    requirement: "FACADE-02"
    verification:
      - kind: other
        ref: "shell verification: heading diff, bullet counts, verb/owner/trigger greps, measurement re-run (find src/core, grep crate::core::, ls manager/) — all documented in ADR Code Locations"
        status: pass
    human_judgment: false
  - id: D2
    description: "deferred-items.md's D1-D4 clauses and Suggested grouping section annotated in place (Shape A), pointing at ADR-0034, zero original text deleted"
    requirement: "FACADE-02"
    verification:
      - kind: other
        ref: "git diff ee9b75a..HEAD -- deferred-items.md shows additions only (zero '-' lines); grep confirms all D1-D4 clauses and 0034 pointer present"
        status: pass
    human_judgment: false

# Metrics
duration: 35min
completed: 2026-08-09
status: complete
---

# Phase 11 Plan 02: D1-D4 Facade Relocation Disposition Summary

**ADR-0034 gives D1-D4 real verdicts with named owners — D1 defers to a no-alias sweep, D2's user_service.rs split is withdrawn to a three-owner split, content_service.rs/event_manager.rs and D3/D4 defer to their own named triggers — and deferred-items.md's four clauses are annotated at source pointing at the ADR, with every original effort/risk rating and recommendation retained.**

## Performance

- **Duration:** 35 min
- **Started:** 2026-08-09T00:05:00Z (approx, worktree spawn)
- **Completed:** 2026-08-09
- **Tasks:** 2
- **Files modified:** 2 (1 created, 1 modified)

## Accomplishments

- Authored ADR-0034 (`.planning/decisions/0034-d1-d4-facade-relocation-disposition.md`) — the seven-heading, no-frontmatter ADR shape, holding four verdicts:
  - **D1** — `src/core/` re-export shims: DEFER, trigger = a facade-wide no-alias sweep, owner = unassigned-pending that sweep. States plainly that ADR-0018 does not settle D1 (different construct: relocated-type paths vs. `src/core/`'s surviving, still-live re-export layer serving 49 importers).
  - **D2** — three independent verdicts for the three manager-service files:
    - `user_service.rs`: split **WITHDRAWN**. Three-owner split stated in one place: the split itself is owned by nobody (withdrawn); the full relocation is owned by the run-3 v2 tech-debt item; the tests are owned by DEFER-02 / Phase 15.
    - `content_service.rs`: DEFER, trigger = the architecture-pass milestone `deferred-items.md`'s own grouping names, owner = unassigned-pending that milestone.
    - `event_manager.rs`: DEFER, same trigger, owner = unassigned-pending that milestone, with the `paladin-core`-vs-facade-app-service target choice explicitly left open for that milestone to resolve.
  - **D3** — entangled Paladin services: DEFER, trigger = the broader builder/execution refactor, owner = unassigned-pending that refactor. States that HARD-05 is answered (ADR-0031 restated the rule) so the per-edge question — non-default, facade-gated, `cfg`-scoped — is what the trigger must answer, not whether leaf-to-leaf edges are permissible at all.
  - **D4** — `content_ingestion_service.rs` placement: DEFER, trigger = the dependency-coupling review, owner = unassigned-pending that review. Legal under ADR-0031 on the same terms as D3.
- Annotated `deferred-items.md`'s D1, D2, D3, D4 clauses in place (Shape A, dated 2026-08-08), each pointing at ADR-0034 and restating its verdict in one or two lines, plus one correction on the `## Suggested grouping` section's two surviving bullets recording that the grouping is now superseded by four individual verdicts.
- No relocation executed; no `.rs` file touched (D-13 held).

## Task Commits

Each task was committed atomically:

1. **Task 1: Write ADR-0034 — the D1-D4 disposition set, four verdicts with owners and triggers** - `cc31aed` (feat)
2. **Task 2: Correct the D1-D4 clauses of `deferred-items.md` at source** - `a14db07` (docs)

## Files Created/Modified

- `.planning/decisions/0034-d1-d4-facade-relocation-disposition.md` - New ADR: four D1-D4 verdicts with owners/triggers, tested against ADR-0031 (unratified status carried forward) and bounded by ADR-0028's executed range
- `.project/Milestone_8-Facade-Cleanup-Shim-Resolution/deferred-items.md` - Dated Shape-A corrections on D1-D4 clauses and the Suggested grouping section, pointing at ADR-0034; zero original text deleted

## Decisions Made

- **ADR number 0034 confirmed still free at execution time.** Re-read `PROMOTION.md:57` this session: `**Next free ADR number: 0034**`, unchanged from plan 11-01's checkpoint resolution (option-a). `ls .planning/decisions/` confirmed 0001-0033 exist, 0034 does not — no race with the parallel 11-03 plan (which takes ADR-0035). Only one ADR number was taken by this plan: **0034**.
- **D2's `content_service.rs` and `event_manager.rs` verdicts derived on merit rather than folded into `user_service.rs`'s withdrawal**, per D-06's explicit "get their own verdicts" (plural) and the plan's Task 1 instruction. Neither file's relocation target falls inside ADR-0028's executed `e5b2011~1..a1e4901` range (that range covers adapter-layer relocations — garrison, sanctum, storage, notifications, herald — not these manager services), so both remain live DEFER items rather than "already resolved by outcome."
- **Both files' targets are not gated by ADR-0031's leaf-to-leaf test** — `paladin-core` is the base crate every extracted crate may already depend on, so `content_service.rs`'s and `event_manager.rs`'s moves are not leaf-to-leaf edges. Their verdict follows purely from D-04's "no relocation executes this phase," not from any legality question.
- **ADR-0031's `⚠ HUMAN REVIEW` / never-ratified status stated explicitly in ADR-0034's `## Context`, `## Decision` (D3 clause), and cited again as a `## Downstream Consumers` instruction** — not laundered into settled authority anywhere in the document, per the explicit prohibition carried into this plan.

## Deviations from Plan

None — plan executed exactly as written, with one **noted pre-existing discrepancy** (not a deviation I introduced, and not fixed, per the plan's own prohibition against deleting or rewriting corrected text):

**Note on `Effort / risk` grep count.** Task 2's acceptance criteria state `grep -c 'Effort / risk' deferred-items.md` should return `5` after this task. The actual baseline **before** this plan's edits was already `6` (not `5`) — plan 11-01's D5 correction banner (committed `ee9b75a`) quotes the phrase `- **Effort / risk:** low / low.` inline inside its own dated-correction blockquote at the top of the file, in addition to the five clause-level occurrences (D1-D5). This plan's edits are strictly additive (verified via `git diff ee9b75a..HEAD` showing zero `-` lines), so the count is unchanged at `6` after this task, not `5`. The underlying invariant the acceptance criterion protects — no original rating was deleted — holds; the exact number `5` in the plan text does not match the tree's actual pre-existing state, which this plan neither caused nor is permitted to correct by deleting text from a prior plan's own correction banner (that would violate D-00c/the plan's own prohibition). The plan's `<verify>` automated block (presence check, not exact count) passes cleanly.

## Issues Encountered

None.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- ADR-0034 exists and is citable by plan 11-05 for the `REQ-m8-deferred-items-register` ledger row amendment.
- Phase 15 has three distinct, non-overlapping owners on `user_service.rs`: the split is withdrawn (no owner), the full relocation is the run-3 v2 tech-debt item's, and DEFER-02's tests are Phase 15's to size against the unsplit file as shipped.
- The facade-wide no-alias sweep (D1), the architecture-pass milestone (D2's `content_service.rs`/`event_manager.rs`), the broader builder/execution refactor (D3), and the dependency-coupling review (D4) are all named as triggers but have no scheduled owner phase yet — recorded as `unassigned-pending` in ADR-0034, consistent with D-04's instruction that a verdict need only name the trigger, not schedule it.
- No blockers for plan 11-05's PROMOTION.md numbering-index update (ADR-0034 row) or ledger amendment.

---
*Phase: 11-facade-residue-deferred-register-disposition*
*Completed: 2026-08-09*
