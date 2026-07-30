# Synthesis Summary

Entry point for `gsd-roadmapper`. Produced by `gsd-doc-synthesizer`.

- **Ingest run:** 1 of 14
- **Source set:** `/workspace/.project/Milestone_1-MVP`
- **Mode:** new (no pre-existing `.planning/PROJECT.md`, `REQUIREMENTS.md`, `ROADMAP.md` or `STATE.md`)
- **Precedence applied:** ADR > SPEC > PRD > DOC (no per-doc overrides present)

---

## Doc counts by type

| Type | Count |
|---|---|
| PRD | 11 |
| DOC | 25 |
| ADR | 0 |
| SPEC | 0 |
| UNKNOWN | 0 |
| **Total** | **36** |

All 36 classifications consumed. Every one carried `manifest_override: true` and
`confidence: high`. Total source volume read: ~516 KB.

## Decisions

- Decisions extracted: **0**
- Decisions locked: **0**
- Source paths: none

No ADR-typed documents exist in this run. `decisions.md` is intentionally empty
of entries. Every technical decision in this milestone currently sits at PRD or
DOC precedence and is auto-overridable by any ADR arriving in runs 2-14.

## Requirements

- Requirements extracted: **107** (across 11 PRDs)
- Of which competing variants preserved unmerged: **12** (6 variant pairs)

Grouped by source PRD:

| Source PRD | Requirements |
|---|---|
| Epic_1/prd-paladin-domain-foundation.md | 8 (+1 variant) |
| Epic_2/prd-garrison-memory-system.md | 10 |
| Epic_3/prd-arsenal-tool-system.md | 9 |
| Epic_4/prd-battalion-orchestration.md | 17 (+3 variants) |
| Epic_5/prd-commander-strategy-router.md | 10 (+3 variants) |
| Epic_6/prd-provider-expansion.md | 7 (+1 variant) |
| Epic_7/prd-citadel-state-persistence.md | 10 |
| Epic_8/prd-herald-output-formatting.md | 8 (+2 variants) |
| Epic_9/prd-armory-cli-tools.md | 12 |
| Epic_10/prd-epic10-validation-documentation.md | 9 |
| unit-test-improvements/prd-improve-unit-test-coverage.md | 1 (+2 variants) |

Variant pairs requiring user resolution before routing:
`REQ-test-coverage-target-v1/v2`, `REQ-temperature-range-v1/v2`,
`REQ-battalion-config-v1/v2`, `REQ-battalion-result-v1/v2`,
`REQ-formation-min-paladins-v1/v2`, `REQ-herald-trait-v1/v2`.

## Constraints

- Constraints extracted: **0**
- Type breakdown: api-contract 0, schema 0, nfr 0, protocol 0

No SPEC-typed documents exist in this run. API-contract and schema material
(port trait signatures, SQLite DDL, YAML config schemas, JSON state schemas,
CLI grammar) is present in the source set but every carrier doc was
manifest-typed PRD or DOC, so that material lives in `requirements.md`
acceptance criteria or `context.md` instead.

## Context

- Context topics recorded: **29** (across 25 DOCs)
- Groups: project vision/methodology/naming/current-state/schedule/risk/tree (7 topics from the project plan), per-epic definitions (10 topics), per-workstream implementation status (11 topics), measured coverage analysis, validation and QA results, deployment infrastructure

Load-bearing context for planning: task-list checkbox state shows **1,817 of
1,857 items complete (98%)**. Known-incomplete work is concentrated in Epic 4
(Chain of Command pattern, integration testing), Epic 8 (Herald execution-path
integration), Epic 6 (live-API integration tests, explicitly deferred), Epic 5
(result normalization/telemetry, one failing test), Epic 2 (final validation),
and the unit-test-coverage workstream (0%-coverage files, gap verification).
Measured quality state as of Epic 10 Task 6.0: 1,091 tests passing with zero
failures, 0 clippy warnings, unit coverage 60.88% against an 80% target,
integration coverage 67.79% against a 70% target, 2 medium transitive security
advisories, benchmarks disabled.

## Conflicts

- **Blockers: 0**
- **Competing variants (warnings): 8**
- **Auto-resolved / informational: 11**

Warnings, in descending planning impact:
1. Competing unit test coverage targets — 80% (nine Epic PRDs) vs 85% (coverage PRD)
2. Competing temperature validation ranges — [0.0, 1.0] (Epic 1) vs 0.0-2.0 (Epic 6 DeepSeek)
3. Competing `BattalionConfig` field sets — Epic 4 vs Epic 5 (`retry_policy` vs `retry_attempts`)
4. Competing `BattalionResult` field sets — Epic 4 vs Epic 5 vs Epic 8 consumer needs
5. Competing minimum Paladin count for Formation — >= 2 (Epic 4) vs 1 allowed (Epic 5 Auto rule)
6. Competing Herald trait signatures — within Epic 8 PRD (FR-1 vs section 6.2)
7. Contradictory Epic 10 completion state — task list 103/103 done vs report claiming Task 7.0 outstanding
8. Contradictory Battalion base module path — `battalion/mod.rs` vs `battalion/battalion.rs`

Cycle detection: cross-ref graph is acyclic (36 nodes, max depth 3, cap 50).

## Files

- Conflicts report: `/workspace/.planning/INGEST-CONFLICTS.md`
- Decisions: `/workspace/.planning/intel/decisions.md` (no entries)
- Requirements: `/workspace/.planning/intel/requirements.md`
- Constraints: `/workspace/.planning/intel/constraints.md` (no entries)
- Context: `/workspace/.planning/intel/context.md`
- Classifications: `/workspace/.planning/intel/classifications/` (36 JSON files)

## Status

**AWAITING USER** — 0 blockers, but 8 competing variants need resolution before
routing. Six of the eight are structural PRD-vs-PRD conflicts on shared types
and quality gates; resolving them changes what a roadmap must contain.

Subsequent runs (Milestones 2-12, Deferred-QA-CICD-Completion,
project-management) will merge into these same intel files in merge mode.
