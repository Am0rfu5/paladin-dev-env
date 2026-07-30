# Synthesis Summary

Entry point for `gsd-roadmapper`. Produced by `gsd-doc-synthesizer`.

- **Ingest runs completed:** 2 of 5
- **Run 1 source set:** `.project/Milestone_1-MVP` (36 docs), MODE=new
- **Run 2 source set:** `.project/Milestone_2-Missing_features` + `.project/Milestone_3-Completion` (45 docs), MODE=merge
- **Precedence applied:** ADR > SPEC > PRD > DOC (no per-doc overrides in either run)
- **Program note:** the ingest was regrouped from 14 milestone-runs into 5 larger runs; run
  numbering below reflects the 5-run program. Run-1 text elsewhere in `.planning/` may still
  say "run 1 of 14" — same run, renumbered program.

---

## Doc counts by type

| Type | Run 1 | Run 2 | Cumulative |
|---|---|---|---|
| PRD | 11 | 15 | 26 |
| DOC | 25 | 30 | 55 |
| ADR | 0 | 0 | 0 |
| SPEC | 0 | 0 | 0 |
| UNKNOWN | 0 | 0 | 0 |
| **Total** | **36** | **45** | **81** |

All 81 classifications consumed. Every one carried `manifest_override: true` and
`confidence: high`. Run-2 source volume read: ~598 KB across 45 files.

Run-2 classifications live in `/workspace/.planning/intel/classifications/run-02/`.

## Decisions

- Decisions extracted: **0** (run 1: 0, run 2: 0)
- Decisions locked: **0**
- Source paths: none

There is still no ADR-typed document anywhere in the corpus. `decisions.md` is
intentionally empty of entries. Every technical position recorded in `requirements.md`
sits at PRD or DOC precedence and is auto-overridable by any ADR arriving in runs 3-5.
No LOCKED-vs-LOCKED hard block is possible.

**Strongest ADR candidate found so far:** `Epic_17.5/epic17-5.md` decides that the CLI
belongs in `src/application/cli` because "CLI is an input adapter in the application
layer, not infrastructure", and directs deletion of the entire `src/cli` tree. It has no
ADR status field or Consequences section, so it is recorded as context only.

## Requirements

- Requirements extracted: **233** cumulative (run 1: 115, run 2: 118)
- Competing variants preserved unmerged: **30** cumulative (run 1: 12, run 2: 18)

Run-2 requirements grouped by source PRD:

| Source PRD | Requirements |
|---|---|
| M2 Epic_11/prd-sanctum-memory-foundation.md | 8 (incl. 1 variant) |
| M2 Epic_12/prd-sanctum-rag-integration.md | 8 (incl. 1 variant) |
| M2 Epic_13/prd-sentinel-vision-system.md | 13 (incl. 5 variants) |
| M2 Epic_14/prd-autonomous-agent-features.md | 8 (incl. 1 variant) |
| M2 Epic_15/prd-conclave-mixture-of-agents.md | 5 |
| M2 Epic_16/prd-epic16-advanced-battalion-patterns.md | 10 (incl. 1 variant) |
| M2 Epic_17/prd-flow-dsl-agent-rearrangement.md | 10 (incl. 1 variant) |
| M2 Epic_18/prd-epic-18-cli-enhancement.md | 7 |
| M3 Epic_19/prd-epic19-herald-consolidation.md | 5 |
| M3 Epic_20/prd-vision-pipeline-completion.md | 6 variants/entries (incl. 5 variants) |
| M3 Epic_21/prd-autonomous-agent-completion.md | 7 (incl. 1 variant) |
| M3 Epic_22/prd-epic22-battalion-commander-hardening.md | 9 (incl. 1 variant) |
| M3 Epic_23/prd-epic23-cli-config-infrastructure-completion.md | 7 |
| M3 Epic_23/prd-task46-arsenal-tool-integration-tests.md | 3 |
| M3 Epic_24/prd-test-hardening-benchmarks-qa.md | 9 |

Run-2 variant sets requiring user resolution before routing:
`REQ-qdrant-sanctum-adapter-v1/v2`, `REQ-vision-format-validation-v1/v2`,
`REQ-openai-vision-adapter-v1/v2`, `REQ-anthropic-vision-adapter-v1/v2`,
`REQ-paladin-vision-api-v1/v2`, `REQ-vision-error-model-v1/v2`,
`REQ-handoff-tool-v1/v2`, `REQ-grove-config-v1/v2`,
`REQ-maneuver-error-strategy-v2` (against run-1 `REQ-battalion-error-strategy`),
`REQ-commander-config-metadata-dir-v3` (against run-1 `REQ-battalion-config-v1/v2`).

Run-2 entries that supersede run-1 requirements without deleting them:
`REQ-max-loops-auto` (supersedes the scalar `max_loops` and its `[1, 100]` validation in
`REQ-paladin-entity` / `REQ-paladin-builder`), `REQ-herald-type-consolidation`
(later position on the run-1 Herald/BattalionResult duplication warnings),
`REQ-battalion-metadata-extension` (later position on `REQ-battalion-result-v1/v2`),
`REQ-autonomous-completion-config-schema` (later position on
`REQ-autonomous-configuration`).

## Constraints

- Constraints extracted: **0**
- Type breakdown: api-contract 0, schema 0, nfr 0, protocol 0

No SPEC-typed documents exist in either run. Substantial api-contract, schema, nfr and
protocol material is present in the run-2 corpus (port traits, the Grove routing JSON
contract, the handoff tool schema, Qdrant collection schema, Commander metadata export
JSON, all the CLI YAML schemas, and roughly twenty numeric performance targets) but every
carrier doc was manifest-typed PRD or DOC. See `constraints.md` for the inventory of what
would become constraints if those docs were re-tagged.

## Context

- Context topics recorded: **59** cumulative (run 1: 31, run 2: 28)
- Run-2 groups: Milestone 3 plan (1 topic), per-epic definitions for Epics 11-24 (14
  topics), implementation-status reports for Epics 11, 15 and 23 (3 topics), Milestone 3
  release notes (1), post-Epic-24 cleanup set (6), reported test-count timeline (1),
  codebase-map anchors (1), Epic 17.5 CLI consolidation decision (1)

Load-bearing context for planning:

1. **Completion state is not taken from prose.** Per `task-completion-state.md`,
   Milestone 2 is 86.2% complete (298 open) and Milestone 3 is 90.0% (132 open). The
   largest open concentrations — Conclave 129, Sanctum 111, Epic 22 hardening 81, Epic 24
   test hardening 29 — are **claims to verify, not confirmed remaining work.** Two of the
   four sit behind documents that declare the epic COMPLETE.
2. **The shipped code is ahead of these documents.** `.planning/codebase/` maps Qdrant
   (qdrant-client 1.14), tokio-cron-scheduler 0.13, the Maneuver Flow DSL, a `vision`
   feature with OpenAI and Anthropic multimodal support, Herald as its own crate, and RAG
   context injection in the documented execution flow — all of which several run-2
   documents describe as deferred or planned. Resolve current locations and behaviour
   through the codebase map, never through these PRDs.
3. **All run-2 file paths are historical.** The run-2 PRDs assume a single-crate
   `src/core|application|infrastructure` layout; the workspace was decomposed into nine
   crates in Milestone 5, outside this run.
4. **One verified open defect.** `codebase/CONCERNS.md` records
   `grove_service.rs:537` still hardcoding `model: "gpt-4"` with a TODO — the same defect
   class Epic 21 removed elsewhere, and a direct miss against Epic 22's completion
   criteria. This is real forward work, not a documentation conflict.

## Conflicts

- **Blockers: 0** (run 1: 0, run 2: 0)
- **Competing variants (warnings): 26** (run 1: 8, run 2: 18)
- **Auto-resolved / informational: 39** (run 1: 11, run 2: 28)

Run-2 warnings in descending planning impact:

1. Milestone 3 epic numbers name different features in the release notes than in the plan
   (release notes: 19=Conclave, 20=Council, 21=Grove, 22=Maneuver, 23=Commander; plan:
   19=Herald, 20=Vision, 21=Autonomous, 22=Battalion hardening, 23=CLI/Config)
2. Release notes push Vision and Autonomous Agents to Milestone 4 while the M3 plan
   schedules them as Epics 20 and 21 of Milestone 3
3. Project-wide coverage gate now has four positions — 80%, 85%, 75% overall (M3 plan
   layered table), measured ~78%
4. Epic 11 declares itself complete while its Qdrant acceptance criteria are explicitly
   deferred (111 open checkboxes)
5. Epic 15 declares itself complete while 129 of its task-list items are unchecked
6. Competing vision API surfaces between Epic 13 and Epic 20 (five paired variants)
7. Contradictory ownership of image format validation (framework vs provider)
8. Vision encryption-at-rest and retention requirements disappear between Epic 13 and 20
9. Competing handoff tool name and parameter names (three names, two parameter sets)
10. MaxLoops changes from scalar to enum, superseding a Milestone 1 requirement
11. Three names and three defaults for the Grove routing threshold
12. Grove `PerformanceBased` routing contradicts an explicit Epic 16 non-goal
13. Competing Council execution API and result shape
14. Competing Maneuver constructor and CLI surface
15. `metadata_output_dir` now has three competing owners (BattalionConfig x2, CommanderConfig)
16. Competing `ErrorStrategy` variant sets for the same enum name
17. Live API tests: PRD-mandated graceful skip vs deliberate loud failure post-cleanup
18. Grove service still hardcodes its LLM model in shipped code (verified open work)

Cycle detection: run-2 cross-ref graph is acyclic (45 nodes, 6 in-set edges, max depth 2,
cap 50). Run 1 was also acyclic (36 nodes, max depth 3).

Security note: one ingested DOC contains a plaintext OpenAI API key. The value was not
copied into any intel or report file. The user has confirmed it is rotated. Redaction of
the source document is recommended — see INGEST-CONFLICTS.md INFO.

## Files

- Conflicts report: `/workspace/.planning/INGEST-CONFLICTS.md` (cumulative, runs 1-2)
- Decisions: `/workspace/.planning/intel/decisions.md` (no entries)
- Requirements: `/workspace/.planning/intel/requirements.md` (233 entries)
- Constraints: `/workspace/.planning/intel/constraints.md` (no entries)
- Context: `/workspace/.planning/intel/context.md` (59 topics)
- Deterministic completion state: `/workspace/.planning/intel/task-completion-state.md`
- Committed codebase map: `/workspace/.planning/codebase/*.md`
- Classifications: `/workspace/.planning/intel/classifications/` (run 1) and
  `/workspace/.planning/intel/classifications/run-02/` (run 2)

## Status

**AWAITING USER** — 0 blockers, but 26 competing variants across runs 1-2 need a decision
before routing. Supersession is expected in this corpus and none of the variants was
resolved by the synthesizer; each is preserved unmerged with the later position noted.

Two run-2 warnings gate roadmap structure rather than content and should be settled first:
the Milestone 3 epic-numbering contradiction (warning 1) because epic numbers are the
provenance keys used throughout the corpus, and the Vision/Autonomous milestone
attribution (warning 2) because it determines whether that work is history to record or
forward work to plan.

## Roadmapper notes

- Honour the Roadmap Extension Protocol at the end of `ROADMAP.md`: new phases start at
  Phase 5, Phases 1-4 are never renumbered, the `### Phase N:` header format is preserved,
  `REQ-*` IDs are the merge keys, and later positions supersede rather than silently
  editing earlier ones.
- Do not re-plan completed work as forward phases. Treat every open checkbox count as a
  claim to verify against `.planning/codebase/` first.
- Runs 3-5 will cover Milestones 4-12, Deferred-QA-CICD-Completion and
  project-management, and will merge into these same intel files.
