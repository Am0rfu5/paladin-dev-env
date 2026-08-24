---
phase: 06-verified-gap-closure
plan: 09
subsystem: docs
tags: [adr, changelog, project-md, grove, routing, provenance]

# Dependency graph
requires:
  - phase: 06-verified-gap-closure
    provides: "06-08's execute()-reachable D-02 fix (resolve_routing_model, pre-dispatch resolution in route_task) and its five execute()-level tests"
provides:
  - "ADR-0013, CHANGELOG.md, and PROJECT.md's Key Decisions row all describe runtime behaviour observable from GroveExecutionService::execute() — the only public entry point — rather than only from the crate-private route_by_llm"
  - "Dated amendment provenance in ADR-0013 recording what 06-VERIFICATION.md found and which plan (06-08) closed it, with the original text retained"
  - "ADR-0013's Code Locations extended with the shipped resolve_routing_model resolver, route_task's pre-dispatch resolution, and five execute()-level exercisers plus the scope-boundary negative-control test"
  - "Explicit scope-boundary language in all three records: the hard error covers only the missing/blank routing_model case; every other Grove routing failure keeps its existing fallback behaviour"
affects: []

# Tech tracking
tech-stack:
  added: []
  patterns:
    - "Amend-at-source with dated provenance: retain the original ADR text and append a bold-labeled, dated paragraph naming what verification found and which plan closed it, matching the house convention already used by ADR-0006's 'Phase N amendment' sections and ADR-0011's 'Resolution note'"

key-files:
  created: []
  modified:
    - .planning/decisions/0013-grove-routing-model.md
    - CHANGELOG.md
    - .planning/PROJECT.md

key-decisions:
  - "Amendment note placed immediately under ## Status as a bold-labeled dated paragraph (Amended (Phase 6, 2026-08-05):), matching the plan's explicit placement instruction rather than nesting it inside ## Decision"
  - "No new ADR number minted — ADR-0013 amended in place per the project's established amend-at-source convention; PROMOTION.md's 0013 claim by D-03 is unaffected"
  - "## Decision, ## Considered Options, and the checkpoint-outcome paragraph left byte-identical (confirmed via git diff) — D-02 is not re-litigated, only the description of what shipped is corrected"

patterns-established:
  - "A behavioural claim in a permanent record must name the entry point a real operator uses (GroveExecutionService::execute()) and cite an exerciser that drives that entry point, not a crate-private function or a test that bypasses it"

requirements-completed: [CLOSE-01]

coverage:
  - id: D1
    description: "ADR-0013 describes the D-02 guarantee at the level it is observable (GroveExecutionService::execute()), cites resolve_routing_model and the execute()-level exercisers by name with resolving line numbers, and records dated provenance for how the gap was found and closed"
    requirement: "CLOSE-01"
    verification:
      - kind: other
        ref: "grep -c 'resolve_routing_model' .planning/decisions/0013-grove-routing-model.md -> 4; grep -c 'GroveExecutionService::execute' -> 6; grep -c 'proceed-as-locked' -> 2; grep -c '06-VERIFICATION' -> 3; every path:line citation confirmed with sed -n against crates/paladin-battalion/src/grove_service.rs"
        status: pass
    human_judgment: false
  - id: D2
    description: "CHANGELOG.md's breaking-change entry names GroveExecutionService::execute() as the entry point, excludes the configuration error from fallback handling explicitly, preserves the Migration paragraph unchanged, and extends Scope with the routing_model-and-LlmRouting boundary"
    requirement: "CLOSE-01"
    verification:
      - kind: other
        ref: "grep -c 'GroveExecutionService::execute' CHANGELOG.md -> 1; grep -c '**Migration:**' -> 1; grep -c 'routing_model' -> 8; grep -c '0013-grove-routing-model.md' -> 1; git diff CHANGELOG.md confirmed confined to the ## [Unreleased] block"
        status: pass
    human_judgment: false
  - id: D3
    description: "PROJECT.md's ADR-0013 Key Decisions row status cell records the full honest history (shipped by 06-01, found unreachable by 06-VERIFICATION.md, closed by 06-08), scoped to exactly one row"
    requirement: "CLOSE-01"
    verification:
      - kind: other
        ref: "grep -c '06-08' .planning/PROJECT.md -> 1; grep -c '0013-grove-routing-model.md' -> 1; git diff --stat .planning/PROJECT.md -> 1 file changed, 1 insertion(+), 1 deletion(-) (single-row change)"
        status: pass
    human_judgment: false

duration: ~10min
completed: 2026-08-05
status: complete
---

# Phase 6 Plan 09: Reconcile ADR-0013, CHANGELOG, and PROJECT.md with the execute()-reachable D-02 fix Summary

**ADR-0013, `CHANGELOG.md`'s `## [Unreleased]` entry, and `.planning/PROJECT.md`'s Key Decisions row now each name `GroveExecutionService::execute()` as the entry point the D-02 hard error is observed from, cite the `resolve_routing_model` resolver and five `execute()`-level exercisers by name with resolving line numbers, and carry dated amendment provenance recording that `06-VERIFICATION.md` found the guard unreachable and plan 06-08 closed it — with `## Decision`, `## Considered Options`, and the checkpoint outcome left byte-identical.**

## Performance

- **Duration:** ~10 min (commit-to-commit: 2026-08-05T21:14:37Z base to 21:20:16Z final task commit)
- **Completed:** 2026-08-05
- **Tasks:** 2
- **Files modified:** 3

## Accomplishments

- Closed `06-VERIFICATION.md` truth 10 for all three records: each now describes runtime behaviour an operator actually observes from `GroveExecutionService::execute()`, not only from the crate-private `route_by_llm` that plan 06-08's guard originally lived in.
- ADR-0013 gained a dated amendment note under `## Status` stating (a) what `06-VERIFICATION.md` found — the guard correct in isolation but unreachable from `execute()` because `route_task`'s blanket fallback arm intercepted it; (b) that this made the original operator-facing claim untrue for every real caller; (c) that plan 06-08 closed it via a pre-dispatch `resolve_routing_model` call in `route_task`, above the fallback arm; and (d) that D-02 itself is unchanged, approved `proceed-as-locked` on 2026-08-05.
- `## Code Locations` extended with `MISSING_ROUTING_MODEL_ERROR` (`grove_service.rs:64`), `resolve_routing_model` (`:252-260`), `route_task`'s pre-dispatch resolution (`:301-303`), and the five `execute()`-level exercisers by name (`test_grove_llm_routing_errors_when_routing_model_absent_through_execute` at `:389`, the inverted `test_grove_llm_routing` at `:235`, and the three `grove_service.rs` unit tests at `:1919`, `:1957`, `:1999`), plus the scope-boundary negative control (`test_grove_llm_routing_falls_back_when_llm_port_absent_but_routing_model_set` at `:457`). Every citation verified against the tree with `sed -n '<line>p'` before being written.
- `## Code Conformance` gained a dated correction retracting the original "no further code work is outstanding" claim, citing plan 06-08's fix and its green `cargo test --workspace` / `cargo fmt --check` / `cargo clippy` gate at HEAD `d83cd36`.
- `## Downstream Consumers`'s operator bullet now names `GroveExecutionService::execute()` explicitly and states the scope boundary in the same sentence.
- `CHANGELOG.md`'s breaking-change entry now says the error is returned "from `GroveExecutionService::execute()`" and that the configuration error is "excluded from Grove's routing fallback handling: no `fallback_tree` substitution and no default agent selection." `**Migration:**` is preserved unchanged; `**Scope:**` is extended to "only Groves that explicitly select `RoutingStrategy::LlmRouting` *and* do not set `routing_model`," naming the four routing-failure modes that keep their existing fallback.
- `.planning/PROJECT.md`'s ADR-0013 Key Decisions row status cell now reads as a history, not a single verdict: shipped by 06-01, found unreachable from the public entry point by `06-VERIFICATION.md`, closed by 06-08's pre-dispatch resolution with the named `execute()`-level test — confirmed a single-row diff, no other row touched.

## Task Commits

Each task was committed atomically:

1. **Task 1: Reconcile ADR-0013 with the shipped, execute()-reachable behaviour** - `5301935` (docs)
2. **Task 2: Reconcile the CHANGELOG breaking-change entry and PROJECT.md's Key Decisions row** - `a98f1c9` (docs)

_Note: per this worktree's `workflow.worktree_skip_hooks: true` authorization, `--no-verify` was used on both commits — this plan touches no `.rs` file, so `cargo fmt`/`clippy` gates have nothing to check; the workspace's green build state was independently confirmed by checking that this worktree's base commit descends from 06-08's own green-gate commit (`d83cd36`, per 06-08-SUMMARY.md) rather than by re-running a full cold `cargo test --workspace`._

## Files Created/Modified

- `.planning/decisions/0013-grove-routing-model.md` - dated amendment note under `## Status`; extended `## Code Locations`; dated correction appended to `## Code Conformance`; precised `## Downstream Consumers` operator bullet
- `CHANGELOG.md` - `### Changed` breaking-change entry corrected to name the entry point and extend the scope boundary; `**Migration:**` preserved unchanged
- `.planning/PROJECT.md` - ADR-0013 row of `## Key Decisions`, status cell rewritten to record the full shipped/found/closed history

## Decisions Made

- Amendment note placed immediately under `## Status` as a bold-labeled dated paragraph (matching the plan's explicit instruction), rather than nested inside `## Decision` the way ADR-0011's "Resolution note" is — the plan's Task 1 action explicitly directed placement under `## Status`.
- No new ADR number minted; ADR-0013 amended in place, consistent with `PROMOTION.md`'s existing D-03 claim on 0013 and the project's amend-at-source convention (already used by ADR-0006 and ADR-0011).
- `## Decision`, `## Considered Options`, and the checkpoint-outcome paragraph left byte-identical, confirmed by `git diff` showing no lines changed inside those sections — D-02 is not re-litigated.

## Deviations from Plan

None - plan executed exactly as written. All acceptance-criteria greps and `sed -n` line-citation checks were run and passed before each commit.

## Issues Encountered

None.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- `06-VERIFICATION.md` truth 10 is now closeable: all three published records describe the `execute()`-reachable behaviour that ships, name the shipped mechanism (`resolve_routing_model`, `route_task`'s pre-dispatch resolution), cite at least one `execute()`-level exerciser each, state the scope boundary explicitly, and carry dated provenance for how the gap was found and closed.
- The recorded D-02 decision is unchanged — `## Decision`, `## Considered Options`, and the checkpoint outcome in ADR-0013 remain exactly as approved on 2026-08-05.
- Wave 2's sibling plan 06-10 (CLOSE-02/CLOSE-03 records) can proceed independently; this plan touched only CLOSE-01's three records and made no change to any file 06-10 is scoped to.

## Self-Check: PASSED

- Files: `.planning/decisions/0013-grove-routing-model.md`, `CHANGELOG.md`, `.planning/PROJECT.md` — all FOUND.
- Commits: `5301935`, `a98f1c9` — both FOUND in `git log`.

---
*Phase: 06-verified-gap-closure*
*Completed: 2026-08-05*
