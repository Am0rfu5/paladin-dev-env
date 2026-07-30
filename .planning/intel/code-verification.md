# Code Verification of Ingest Claims

Direct verification of contested ingest claims against the shipped tree on `release/v0.7.0`,
performed 2026-07-30 during ingest run 2. Evidence is file existence and dependency
declarations, not LLM inference.

**Purpose:** several ingested documents claim work is complete that task checklists show open,
and vice versa. This file records what the code actually contains, so downstream planning does
not re-plan shipped features or drop genuine gaps.

**Precedence note:** shipped code outranks every ingested document. Where this file contradicts
a PRD, DOC, or `task-completion-state.md` count, this file wins.

## Verified SHIPPED

| Feature | Claim in docs | Evidence in tree |
|---|---|---|
| Conclave (mixture-of-agents) | Epic 15 completion report says COMPLETE; `tasks-conclave-mixture-of-agents.md` has **129 open** checkboxes | `crates/paladin-core/src/platform/container/battalion/conclave.rs`, `crates/paladin-battalion/src/conclave_execution_service.rs`, `examples/conclave_expert_panel.rs`, referenced from `battalion/mod.rs` and `commander.rs` |
| Sentinel vision | M3 release notes list vision under "What's Next (Milestone 4)" as *not delivered* | `crates/paladin-ports/src/output/vision_port.rs`, `crates/paladin-ports/src/output/vision_llm_port.rs`, `tests/integration/vision_integration_test.rs`, `examples/vision_analysis.rs`, `examples/vision_battalion.rs`, `docs/src/appendix/battalion-vision-support.md` |
| Qdrant Sanctum adapter | `EPIC_11_COMPLETION_SUMMARY.md` records "Task 5.0: Qdrant Adapter (DEFERRED) — Not implemented" | `qdrant-client = "1.14"` in root `Cargo.toml` (optional, behind `qdrant` feature); integration test target `qdrant_sanctum_integration` |
| Council pattern | Named as "Epic 20" in M3 release notes; belongs to M2 Epic 16 | `examples/council_discussion.rs`, `examples/commander_council.rs`, `tests/integration/commander_integration_tests.rs` |
| Grove pattern | Named as "Epic 21" in M3 release notes; belongs to M2 Epic 16 | `examples/grove_routing.rs`, `examples/commander_grove.rs`, `tests/integration/commander_integration_tests.rs` |
| Maneuver / Flow DSL | Named as "Epic 22" in M3 release notes; belongs to M2 Epic 17 | `examples/maneuver_basic.rs`, `examples/maneuver_nested_flow.rs`, `examples/maneuver_dynamic_flow.rs` |

## Resolved variants

### Vision API surface — BOTH shipped, neither superseded

The run-2 conflict report preserved Epic 13's `VisionCapableLlm` trait surface against Epic 20's
`VisionPort` surface as competing variants. **Both exist in the tree:**

- `crates/paladin-ports/src/output/vision_llm_port.rs` — the Epic 13 lineage
- `crates/paladin-ports/src/output/vision_port.rs` — the Epic 20 lineage

This is not an unresolved contradiction; it is two coexisting ports. Do not plan a migration
from one to the other on the strength of the PRD conflict alone — confirm intent first.

### Milestone 3 epic numbering — plan numbering is authoritative

`RELEASE_NOTES_MILESTONE_3.md` numbers Epics 19-23 as Conclave / Council / Grove / Maneuver /
Commander Enhancement. Those four patterns are Milestone **2** features (Epics 15, 16, 16, 17),
all verified shipped above. `Project_Plan_Milestone_3.md`, the six `epic19..24.md` definitions,
every `prd-*.md`, and every `tasks-*.md` in Milestone 3 instead use: 19 = Herald & Domain Type
Consolidation, 20 = Vision Pipeline Completion, 21 = Autonomous Agent Completion, 22 = Battalion
& Commander Hardening, 23 = CLI/Config/Infrastructure Completion, 24 = Test Hardening.

**Resolution: the plan/epic-definition numbering is authoritative** — 8 of 9 Milestone-3
documents plus all task lists agree. The release-notes numbering is a documentation defect and
must not reach ROADMAP.md as provenance keys.

### Release-notes forward-look is stale

`RELEASE_NOTES_MILESTONE_3.md` "What's Next (Milestone 4)" describes vision and autonomous-agent
work as planned. Vision is verified shipped. Treat that section as a point-in-time forward-look
that was overtaken, not as scope.

## Implication for open-checkbox counts

`task-completion-state.md` records 542 open items across 75 task lists (93.3% complete). The two
largest concentrations — Conclave 129 and Sanctum 111 — are both **shipped**. Checkbox state was
not maintained through to completion in at least these cases, and run 1 independently found the
same pattern (Chain of Command and Herald wiring marked open but implemented).

**Do not treat open checkbox counts as a work backlog.** Every one requires verification against
the tree before it becomes a planned requirement. The genuine remaining-work signal lives in the
Deferred-QA-CICD-Completion and Milestone_8 deferred documents (ingest run 5), not in checkbox
arithmetic.

## Not yet verified

These carry open checkboxes and have not been checked against code:

- `tasks-epic22-battalion-commander-hardening.md` — 81 open
- `tasks-autonomous-agent-features.md` — 45 open
- `tasks-test-hardening-benchmarks-qa.md` — 29 open
- `tasks-content-rewrite.md` — 26 open (Milestone 11 documentation)
- `tasks-harden-port-traits-stable-api.md` — 20 open
- `tasks-provider-expansion.md` — 19 open (Milestone 1; live-API tests explicitly deferred)
