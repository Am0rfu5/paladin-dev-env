# ADR-0010: Milestone 3 epic numbering

## Status

Accepted

**Date:** 2026-08-04

## Context

`.project/Milestone_3-Completion/RELEASE_NOTES_MILESTONE_3.md` asserts 19 = Conclave Pattern
(Multi-Expert Synthesis), 20 = Council Pattern (Iterative Discussion), 21 = Grove Pattern
(Contextual Routing), 22 = Maneuver Pattern (Flow DSL), 23 = Commander Enhancement. The
plan/epic-definition set — `Project_Plan_Milestone_3.md` and the six `Epic_19/epic19.md` through
`Epic_24/epic24.md` definitions, agreed by 8 of the 9 Milestone-3 documents and every task list —
instead assigns 19 = Herald & Domain Type Consolidation, 20 = Vision Pipeline Completion,
21 = Autonomous Agent Completion, 22 = Battalion & Commander Hardening,
23 = CLI/Config/Infrastructure Completion, 24 = Test Hardening. Only Epic 24 agrees between the
two schemes.

The real-world consequence: the content the release notes attributes to Milestone 3 Epics
19-22 (Conclave, Council, Grove, Maneuver) is in fact Milestone **2** work — Conclave is Epic 15,
Council and Grove are both Epic 16, and Maneuver is Epic 17 / 17.5. A reader who trusts the
release notes and goes looking for Conclave, Council, Grove or Maneuver inside Milestone 3 is
looking in the wrong milestone's ledger entirely. Epic numbers are this corpus's provenance keys
— every one of Phases 6-16's citations against a Milestone 3 epic number depends on which of the
two schemes the citer meant, so this defect propagates silently until it is fixed at the source.

Two further release-notes claims, embedded inside the same defective sections, are verified
absent from or divergent against the shipped tree rather than merely mislabeled: the
`RoutingStrategy::PerformanceBased` variant advertised under the mislabeled "Epic 21: Grove
Pattern" section, and the Council/Maneuver API forms shown in the mislabeled "Epic 20" and
"Epic 22" code examples.

## Decision

The plan/epic-definition set is authoritative. It is the numbering every Phase 5-16 citation of a
Milestone 3 epic number uses from this point forward. The mapping below records both sides of
every colliding integer — never a silent choice of one side — plus the actual Milestone/Epic home
of whatever content the release notes misattributed to that integer.

| Epic | Authoritative content | Release-notes assertion | Actual home of the release-notes content |
|---|---|---|---|
| 19 | Herald & Domain Type Consolidation | Conclave Pattern (Multi-Expert Synthesis) | Milestone 2 Epic 15 |
| 20 | Vision Pipeline Completion | Council Pattern (Iterative Discussion) | Milestone 2 Epic 16 |
| 21 | Autonomous Agent Completion | Grove Pattern (Contextual Routing) | Milestone 2 Epic 16 |
| 22 | Battalion & Commander Hardening | Maneuver Pattern (Flow DSL) | Milestone 2 Epic 17 / 17.5 |
| 23 | CLI/Config/Infrastructure Completion | Commander Enhancement | Milestone 3 Epic 22 (Battalion & Commander Hardening) — a second-order collision: the release notes' own "Epic 23" content is itself the authoritative Epic 22 |
| 24 | Test Hardening | — (release notes' Epic 24 heading at :160 already matches) | Milestone 3 Epic 24 — no correction needed |

## Considered Options

- The release-notes numbering as the authoritative set — rejected. It is one document
  (`RELEASE_NOTES_MILESTONE_3.md`) against eight of the nine Milestone-3 documents
  (`Project_Plan_Milestone_3.md` plus the six `epic19.md`…`epic24.md` definitions) and every
  `tasks-*.md` task list in the corpus.
- Renumbering the plan/epic-definition set to match the release notes — rejected. It would
  falsify every task-list filename (`tasks-epic19-*.md` through `tasks-epic24-*.md`) and the
  ROADMAP's own phase text, both of which already cite the plan/epic-definition numbering as fact.
- Recording the conflict without choosing an authoritative side — rejected. VERIFY-03 requires
  the defect recorded "once and permanently," and a recorded non-answer would leave every
  downstream Phase 6-16 citation of a Milestone 3 epic number ambiguous between two schemes,
  which is the exact defect this ADR exists to close.

## Code Locations

- `.project/Milestone_3-Completion/RELEASE_NOTES_MILESTONE_3.md:21` — `### Epic 19: Conclave
  Pattern (Multi-Expert Synthesis)` heading; authoritative Epic 19 is Herald & Domain Type
  Consolidation, and this section's actual content is Milestone 2 Epic 15.
- `.project/Milestone_3-Completion/RELEASE_NOTES_MILESTONE_3.md:48` — `### Epic 20: Council
  Pattern (Iterative Discussion)` heading; authoritative Epic 20 is Vision Pipeline Completion,
  and this section's actual content is Milestone 2 Epic 16. This section also carries the Council
  API-form claim below.
- `.project/Milestone_3-Completion/RELEASE_NOTES_MILESTONE_3.md:76` — `### Epic 21: Grove Pattern
  (Contextual Routing)` heading; authoritative Epic 21 is Autonomous Agent Completion, and this
  section's actual content is Milestone 2 Epic 16.
- `.project/Milestone_3-Completion/RELEASE_NOTES_MILESTONE_3.md:106` — the `RoutingStrategy::
  PerformanceBased` ("Adaptive routing based on historical success") bullet inside the mislabeled
  "Epic 21" section, which also contradicts Epic 16 non-goal NG-3 ("Grove learning from routing
  decisions to improve future matches (future ML feature)"). Verified absent from the tree:
  `grep -rn "PerformanceBased" crates/ src/` returns no matches. The shipped
  `RoutingStrategy` enum at `crates/paladin-core/src/platform/container/battalion/grove.rs:54`
  has exactly three variants — `KeywordMatch` (`#[default]`), `SemanticSimilarity`, `LlmRouting`
  — confirmed by direct read of `grove.rs:52-73`; `crates/paladin-battalion/src/grove_service.rs`
  implements routing against that same three-variant set.
- `.project/Milestone_3-Completion/RELEASE_NOTES_MILESTONE_3.md:111` — `### Epic 22: Maneuver
  Pattern (Flow DSL)` heading; authoritative Epic 22 is Battalion & Commander Hardening, and this
  section's actual content is Milestone 2 Epic 17 / 17.5. This section also carries the Maneuver
  constructor-argument-order claim below.
- `.project/Milestone_3-Completion/RELEASE_NOTES_MILESTONE_3.md:147` — `### Epic 23: Commander
  Enhancement` heading; authoritative Epic 23 is CLI/Config/Infrastructure Completion, and this
  section's actual content (Commander auto-detection, Council/Grove/Conclave integration) is
  itself Milestone 3 Epic 22, Battalion & Commander Hardening — the one integer where a release-
  notes misattribution and the authoritative numbering collide on two different Milestone-3
  epics rather than a Milestone-2 one.
- `.project/Milestone_3-Completion/RELEASE_NOTES_MILESTONE_3.md:320` — `## 🔮 What's Next
  (Milestone 4)` forward-look section, listing Sentinel vision as planned, not delivered. Stale:
  `.planning/intel/code-verification.md` §"Sentinel vision" records vision as verified shipped
  (`crates/paladin-ports/src/output/vision_port.rs`, `vision_llm_port.rs`,
  `tests/integration/vision_integration_test.rs`).
- Council API-form claim, `RELEASE_NOTES_MILESTONE_3.md:59-70`: shows
  `council_service.execute(&council, &experts, topic)` and `result.summary`. The shipped surface
  at `crates/paladin-battalion/src/council_service.rs:118` is
  `CouncilExecutionService::convene(&self, council: &Council, topic: &str) -> Result<CouncilResult,
  BattalionError>`, and `CouncilResult` at `council_service.rs:25-29` carries a `conclusion:
  Option<String>` field, not `summary`.
- Maneuver constructor-argument-order claim, `RELEASE_NOTES_MILESTONE_3.md:135`: shows
  `Maneuver::new(flow3, paladins, config)` (flow first, config third, no name argument). The
  shipped constructor at `crates/paladin-battalion/src/maneuver/mod.rs:148-153` is
  `Maneuver::new(name: impl Into<String>, agents: HashMap<String, Paladin>, flow: FlowExpression,
  config: ManeuverConfig) -> Result<Self, ManeuverError>` — name first, flow third, config fourth.

## Code Conformance

conforms

This is a documentation defect, not a code defect. No Rust source file is wrong: the
`RoutingStrategy` enum, the Council `convene`/`conclusion` surface and the `Maneuver::new`
constructor all ship correctly and consistently with the authoritative Milestone 3 plan/epic
definitions cited above. The only executing work this defect requires is this plan's own Task 2
edit to `RELEASE_NOTES_MILESTONE_3.md` — no phase in Phases 5-16 changes any `.rs` file, any
`Cargo.toml`, or any `.github/workflows/` file as a consequence of this ADR.

## Downstream Consumers

- Phase 5 ledger plans 05-08 through 05-12 — the Epic 15 (Conclave), Epic 16 (Council, Grove) and
  Epic 17 (Maneuver) ledger rows in `.planning/ledgers/milestone-02-03.md` cite this ADR's mapping
  table when recording the release-notes attribution defect against each requirement.
- Plan 05-13 — advances `.planning/decisions/PROMOTION.md`'s "Next free ADR number" line past
  0010.
- Any Phase 6-16 reader citing a Milestone 3 epic number — resolves through this ADR before
  either `RELEASE_NOTES_MILESTONE_3.md` or any task list, per this project's precedence order
  (ADR → shipped tree → `.planning/codebase/` map → `intel/code-verification.md` → PRD → DOC →
  task-list checkbox), per D-00b.
