---
phase: 06-verified-gap-closure
plan: 06
subsystem: docs
tags: [adr, changelog, grove, routing, decision-record]

# Dependency graph
requires:
  - phase: 06-verified-gap-closure
    provides: "plan 06-01's shipped code (GroveConfig.routing_model, GroveBuilder.routing_model, the route_by_llm hard-error guard) and its checkpoint outcome (proceed-as-locked), cited verbatim by ADR-0013's Code Locations and Decision sections"
provides:
  - "ADR-0013, recording D-01 (routing_model config surface), D-02 (hard-error guard, no fallback), and D-03 (three-way recording requirement) with shipped-code file:line citations"
  - "PROMOTION.md numbering index row for 0013 and Next free ADR number bumped to 0014"
  - "CHANGELOG.md ## [Unreleased] entry naming the routing_model field, the RoutingError break, the migration instruction, and the KeywordMatch-default scoping fact"
  - "PROJECT.md Key Decisions row linking ADR-0013"
affects: [06-07 (Phase 6 close-out marks CLOSE-01 satisfied in REQUIREMENTS.md, citing this ADR)]

# Tech tracking
tech-stack:
  added: []
  patterns: [ADR three-way recording (ADR + CHANGELOG + rustdoc) for a one-way runtime break, per D-03]

key-files:
  created:
    - .planning/decisions/0013-grove-routing-model.md
  modified:
    - .planning/decisions/PROMOTION.md
    - CHANGELOG.md
    - .planning/PROJECT.md

key-decisions:
  - "ADR-0013's ## Code Conformance reads conforms (not must change) because this plan depends on 06-01, which already shipped the code — the ADR cites the shipped line numbers, not predicted ones."
  - "CHANGELOG entry uses ### Added / ### Changed directly under ## [Unreleased] (three-hash), matching standard Keep a Changelog convention — not the #### Added/Changed nesting the ## [0.7.0] section uses, which exists only because that section wraps an extra ### Phase 12.1 sub-heading that ## [Unreleased] has no equivalent of."
  - "Observation recorded, not acted on: PROJECT.md's Key Decisions table has no rows for ADR-0010, ADR-0011 or ADR-0012 (Phase 5 minted three ADRs without adding their PROMOTION.md step-6 rows). Back-filling those three is outside this plan's and this phase's scope — named next owner is whichever phase next touches PROJECT.md's Key Decisions table (candidate: Phase 7, ARCH-* work, since Phase 7 is the next phase already scheduled to touch decision records)."

patterns-established: []

requirements-completed: [CLOSE-01]

coverage:
  - id: D1
    description: "ADR-0013 exists with all seven required H2 headings in PROMOTION.md's exact order, Considered Options and Code Locations as bulleted lists, and a conforms verdict naming CLOSE-01"
    requirement: "CLOSE-01"
    verification:
      - kind: other
        ref: "grep -c '^## ' .planning/decisions/0013-grove-routing-model.md => 7, in Status/Context/Decision/Considered Options/Code Locations/Code Conformance/Downstream Consumers order"
        status: pass
      - kind: other
        ref: "grep -c 'RoutingError|get_available_models|CLOSE-01' .planning/decisions/0013-grove-routing-model.md => 4/2/7 respectively (all >= 1)"
        status: pass
    human_judgment: false
  - id: D2
    description: "PROMOTION.md's numbering index carries the 0013 row and Next free ADR number reads 0014, not 0013"
    requirement: "CLOSE-01"
    verification:
      - kind: other
        ref: "grep -c 'Next free ADR number: 0014' .planning/decisions/PROMOTION.md => 1; grep -c '| 0013 |' => 1; grep -c 'Next free ADR number: 0013' => 0"
        status: pass
    human_judgment: false
  - id: D3
    description: "CHANGELOG.md's ## [Unreleased] section names routing_model, RoutingError, ADR-0013, and a migration instruction under a ### Changed heading, without touching the ## [0.7.0] section"
    requirement: "CLOSE-01"
    verification:
      - kind: other
        ref: "awk '/^## \\[Unreleased\\]/{f=1;next} /^## \\[/{f=0} f' CHANGELOG.md | grep -c 'routing_model' => 7; grep -c 'RoutingError' => 1; grep -c 'ADR-0013' => 1; grep -cE '^### (Changed|Breaking)' => 1"
        status: pass
      - kind: other
        ref: "git diff --stat CHANGELOG.md => 22 insertions, 0 deletions (0.7.0 section untouched)"
        status: pass
    human_judgment: false
  - id: D4
    description: "PROJECT.md's Key Decisions table carries a row linking ADR-0013, added via a scoped, additive Edit (not a whole-file Write)"
    requirement: "CLOSE-01"
    verification:
      - kind: other
        ref: "grep -c '0013-grove-routing-model.md' .planning/PROJECT.md => 1; grep -c 'ADR-0013' => 1; git diff --stat .planning/PROJECT.md => 1 insertion, 0 deletions"
        status: pass
    human_judgment: false

duration: ~35min
completed: 2026-08-05
status: complete
---

# Phase 6 Plan 06: Grove Routing-Model Break — ADR-0013 and CHANGELOG Record Summary

**ADR-0013 records D-01/D-02/D-03's Grove routing-model decision with shipped-code `file:line` citations from plan 06-01, cites the human's `proceed-as-locked` checkpoint approval, and a `## [Unreleased]` CHANGELOG entry states the runtime break and migration instruction for any operator upgrading across it.**

## Performance

- **Duration:** ~35 min
- **Started:** 2026-08-05
- **Completed:** 2026-08-05
- **Tasks:** 2
- **Files modified:** 4 (1 created, 3 modified)

## Accomplishments

- `.planning/decisions/0013-grove-routing-model.md` created with all seven required H2 headings in PROMOTION.md's order, quoting the pre-change hardcoded literal verbatim, recording D-01's config surface and D-02's hard-error guard with their four rejected alternatives, citing plan 06-01's shipped `file:line` locations (not predicted ones), and closing with a `conforms` verdict naming CLOSE-01.
- `.planning/decisions/PROMOTION.md` updated: numbering-index row added for `0013 | grove-routing-model`, `Next free ADR number` line bumped from `0013` to `0014`.
- `CHANGELOG.md`'s `## [Unreleased]` section carries an `### Added` entry for `GroveConfig.routing_model` / `GroveBuilder::routing_model(..)`, and an `### Changed` entry stating the runtime break in operator-facing terms — the affected configuration, the resulting error type, the migration instruction, and the `KeywordMatch`-default scoping fact — linked to ADR-0013.
- `.planning/PROJECT.md`'s `## Key Decisions` table carries a new row linking ADR-0013, added via a scoped `Edit` anchored on the last existing row.

## Task Commits

Each task was committed atomically:

1. **Task 1: Author ADR-0013 and update the numbering index** — `08d7675` (docs)
2. **Task 2: Write the CHANGELOG entry and the PROJECT.md Key Decisions row** — `5afb774` (docs)

**Plan metadata:** committed in the same pass as this SUMMARY (worktree mode — STATE.md/ROADMAP.md updates deferred to the orchestrator after wave merge).

## Files Created/Modified

- `.planning/decisions/0013-grove-routing-model.md` — new ADR: Status/Context/Decision/Considered Options/Code Locations/Code Conformance/Downstream Consumers
- `.planning/decisions/PROMOTION.md` — numbering-index row for `0013`, `Next free ADR number: 0014`
- `CHANGELOG.md` — `## [Unreleased]` `### Added` and `### Changed` entries for the Grove routing-model break
- `.planning/PROJECT.md` — `## Key Decisions` table row linking ADR-0013

## Decisions Made

- ADR-0013's `## Code Conformance` verdict is `conforms`, not `must change`, because this plan's `depends_on: [06-01]` guaranteed the code had already shipped by the time this plan ran — the ADR cites the actual shipped line numbers (re-read from the post-06-01 tree via `grep`, not transcribed from the plan's predictions).
- CHANGELOG sub-headings use `### Added` / `### Changed` (three-hash) directly under `## [Unreleased]`, matching the standard Keep a Changelog convention the file declares at its top. The existing `## [0.7.0]` section uses `#### Added` / `#### Changed` (four-hash) only because it wraps an extra `### Phase 12.1 — ...` sub-heading between the version heading and the change-type headings; `## [Unreleased]` has no equivalent phase-grouping need, so three-hash is correct and matches the plan's own acceptance criteria (which grep for `^### (Changed|Breaking)`, not `^#### `).
- The two sibling `"gpt-4"`/`"gpt-4o"` occurrences in `council_service.rs:816` and `conclave_execution_service.rs:600` were re-verified (not transcribed from CONTEXT.md) to sit inside `#[cfg(test)]` blocks starting at lines 521 and 512 respectively, confirming they remain out of CLOSE-01's scope.

## Deviations from Plan

None — plan executed exactly as written. Both tasks' acceptance criteria (heading count/order, bulleted-list sections, grep thresholds on `RoutingError`/`get_available_models`/`CLOSE-01`/`routing_model`/`ADR-0013`, PROMOTION.md's next-free-number transition, and the additive-only `git diff --stat` shape on `CHANGELOG.md` and `PROJECT.md`) were verified directly after each edit and all passed on the first attempt.

## Issues Encountered

None.

## Observation Carried Forward (not acted on, per plan instruction)

`.planning/PROJECT.md`'s `## Key Decisions` table now carries rows for ADR-0001 through ADR-0006, ADR-0008, ADR-0009 and (as of this plan) ADR-0013, but **still has no rows for ADR-0010, ADR-0011 or ADR-0012** — Phase 5 minted all three without adding their PROMOTION.md step-6 rows. Back-filling those three rows is explicitly outside this plan's and this phase's scope (Task 2's `<action>` instructs recording the gap, not fixing it). Carried forward here as an unowned, unfixed gap rather than silently dropped; the next phase that touches `.planning/PROJECT.md`'s Key Decisions table (a plausible candidate is Phase 7's ARCH-* work, which continues the decision-recording thread) should either add the three missing rows or explicitly re-defer them with a reason.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- D-03 is fully satisfied: the Grove routing-model break is now recorded three ways — ADR-0013 (this plan), the `## [Unreleased]` CHANGELOG entry (this plan), and the `GroveConfig.routing_model` rustdoc (plan 06-01, already shipped).
- `.planning/decisions/PROMOTION.md`'s shared numbering state is correct for the next phase that mints an ADR (`Next free ADR number: 0014`).
- Plan 06-07 (Phase 6 close-out) can now mark CLOSE-01 satisfied in `.planning/REQUIREMENTS.md`, citing ADR-0013.
- No blockers for downstream plans in this wave.

---
*Phase: 06-verified-gap-closure*
*Completed: 2026-08-05*

## Self-Check: PASSED

- FOUND: `.planning/decisions/0013-grove-routing-model.md`
- FOUND: `.planning/decisions/PROMOTION.md` (modified)
- FOUND: `CHANGELOG.md` (modified)
- FOUND: `.planning/PROJECT.md` (modified)
- FOUND: commit `08d7675` (docs — Task 1)
- FOUND: commit `5afb774` (docs — Task 2)
