# Synthesis Summary

Entry point for `gsd-roadmapper`. Produced by `gsd-doc-synthesizer`.

- **Ingest runs completed:** 3 of 5
- **Run 1 source set:** `.project/Milestone_1-MVP` (36 docs), MODE=new
- **Run 2 source set:** `.project/Milestone_2-Missing_features` + `.project/Milestone_3-Completion`
  (45 docs), MODE=merge
- **Run 3 source set:** `.project/Milestone_4-Refactor-Crates-Features` +
  `.project/Milestone_5-Workspace-Decomposition` +
  `.project/Milestone_6-Architectural-Refinements` (32 docs), MODE=merge
- **Precedence applied:** ADR > SPEC > PRD > DOC (no per-doc overrides in any run)
- **Program note:** the ingest was regrouped from 14 milestone-runs into 5 larger runs; run
  numbering below reflects the 5-run program. Run-1 text elsewhere in `.planning/` may still
  say "run 1 of 14" — same run, renumbered program.

---

## Doc counts by type

| Type | Run 1 | Run 2 | Run 3 | Cumulative |
|---|---|---|---|---|
| PRD | 11 | 15 | 13 | 39 |
| DOC | 25 | 30 | 19 | 74 |
| ADR | 0 | 0 | 0 | 0 |
| SPEC | 0 | 0 | 0 | 0 |
| UNKNOWN | 0 | 0 | 0 | 0 |
| **Total** | **36** | **45** | **32** | **113** |

All 113 classifications consumed. Every one carried `manifest_override: true` and
`confidence: high`. Run-3 source volume read: ~530 KB across 32 files.

Run-3 classifications live in `/workspace/.planning/intel/classifications/run-03/`.

**Nine of the 19 run-3 DOCs are verbatim extracts** of two milestone-overview documents
(Milestone 5 Epics 2-5 and Milestone 6 Epics 1-4). They add no independent content and are
consolidated into two context topics rather than nine. Do not double-count their acceptance
criteria.

## Decisions

- Decisions extracted: **0** (run 1: 0, run 2: 0, run 3: 0)
- Decisions locked: **0**
- Source paths: none

There is still no ADR-typed document anywhere in the corpus after 113 files. `decisions.md`
is intentionally empty of entries. No LOCKED-vs-LOCKED hard block is possible and every
technical position in `requirements.md` remains overridable by any ADR arriving in runs 4-5.

**Run 3 contains the corpus's only decision record — and it is still not a locked decision.**
`Milestone_5/Epic_1/decisions/battalion-result-upward-dependency-decision.md` plus its
`-options.md` companion are the only decision/options pair in all 263 documents. The decision
doc carries `Status: Approved`, `Decision Date: 2026-05-13`, `Chosen Option: Option A`, a
Rationale section and a Rejected Options section — it reads as a genuine ADR — but is
manifest-typed DOC with `locked: false`, so it sits at the lowest precedence tier.

What it settles: the **location** of five pure value/error types (`PaladinResult`,
`StopReason`, `TokenUsage`, `RegistryError`, `HandoffError`) moving into `paladin-core`, with
the application ports reduced to thin re-exports and `PaladinError` deliberately excluded.

What it does **not** settle: it never mentions `BattalionResult`. Despite the filename, the
run-1 `REQ-battalion-result-v1/-v2` field-set variant is untouched by it. That variant is
closed by **shipped code** instead — see `code-verification.md`.

**Two ADR candidates now exist:** this decision record and `Epic_17.5/epic17-5.md` (the
run-2 CLI-location decision). Promote both via `--manifest` if their positions should be
protected from future override. A PRD published two days after the decision record already
contradicts it — see WARNINGS.

## Requirements

- Requirements extracted: **348** cumulative (run 1: 115, run 2: 118, run 3: 115)
- Competing variants preserved unmerged: **38** cumulative (run 1: 12, run 2: 18, run 3: 8)

Run-3 requirements grouped by source PRD:

| Source PRD | Requirements |
|---|---|
| M4 Epic_1/prd-expand-feature-flags.md | 7 |
| M4 Epic_2/prd-harden-port-traits-stable-api.md | 9 |
| M4 Epic_3/prd-cli-isolation.md | 9 |
| M5 Epic_1/prd-workspace-initialization-and-paladin-core-extraction.md | 9 (incl. 4 variants) |
| M5 Epic_2/prd-paladin-ports-extraction.md | 9 (incl. 1 variant) |
| M5 Epic_3/prd-paladin-battalion-extraction.md | 8 (incl. 1 variant) |
| M5 Epic_4/prd-paladin-llm-extraction.md | 11 (incl. 1 variant) |
| M5 Epic_5/prd-paladin-memory-extraction.md | 9 (incl. 1 variant) |
| M5 Epic_6/prd-workspace-finalization-epic-6.md | 6 |
| M6 Epic_1/prd-decompose-application-settings.md | 8 (incl. 1 variant) |
| M6 Epic_2/prd-relocate-orchestration-services.md | 9 |
| M6 Epic_3/prd-co-locate-maneuver-dsl.md | 9 |
| M6 Epic_4/prd-relocate-circuitbreaker-infra.md | 8 |

Run-3 variant sets requiring user resolution before routing (4 pairs):
`REQ-workspace-crate-edition-v1/-v2` (Rust edition 2021 versus 2024),
`REQ-paladin-core-dependency-allowlist-v1/-v2` (exhaustive 6-crate list versus petgraph),
`REQ-port-value-type-ownership-v1/-v2` (decision record versus Epic 2 PRD),
`REQ-llm-config-bridge-location-v1/-v2` (root-crate bridge versus paladin-llm config module).

**All four are settled by shipped code**, which is unusual for this corpus — see
`code-verification.md`. The user still needs to decide whether to amend the documents or
accept the code as the resolution, because three of the four PRDs are unamended and their
rules (notably Epic 2's FR-10 "types must not be split across crates") would produce the
wrong answer if applied literally to future work.

Run-3 entries that supersede earlier requirements without deleting them:
`REQ-maneuver-files-moved-from-core` supersedes the Maneuver-parser inclusion in
`REQ-core-container-extraction`; `REQ-circuitbreaker-relocation` supersedes every run-1/run-2
`src/application/use_cases/paladin/circuit_breaker.rs` path reference; the whole M5 crate set
supersedes every `src/{core,application,infrastructure}` path recorded in runs 1-2.

## Constraints

- Constraints extracted: **0**
- Type breakdown: api-contract 0, schema 0, nfr 0, protocol 0

No SPEC-typed documents exist in any run. **Run 3 is by far the most constraint-dense set** —
these three milestones are almost entirely build-system contracts, dependency layering and
module boundaries rather than features. `constraints.md` inventories what would become
constraints if the carriers were re-tagged: the full 25-port extraction inventory, every
per-crate `[features]` table, the three dependency allowlists (6 / 7 / 14-permitted-9-forbidden),
the workspace `Cargo.toml` template, the `config.yml` deserialization contract, ~20 numeric
build/coverage/file-size targets, and the `#[cfg]` guard and import-migration protocols.

The strongest re-tag candidates are the **dependency allowlists** and the **`config.yml`
deserialization contract**, because both are already contradicted by shipped code.

## Context

- Context topics recorded: **73** cumulative (run 1: 31, run 2: 28, run 3: 14)
- Run-3 groups: the origin analysis document (1), the three milestone plans (3), the four
  Milestone-4 analysis artifacts (4: dependency matrix, api audit, deprecations, plus the M4
  plan), the two `decisions/` files (2), the build-benchmark report (1), the Epic 2 dependency
  analysis (1), the two consolidated per-epic-DOC topics (2), and the code-verification anchors
  topic (1)

Load-bearing context for planning:

1. **Run 3 is the first run where checkbox counts are corroborated by code, not contradicted.**
   Milestone 4's 20 open items (all in `tasks-harden-port-traits-stable-api.md`) are real:
   zero `#[deprecated]` annotations exist in the tree. Milestone 6's 0 open items are real:
   all four relocations are verifiably complete. Milestone 5's 17 open items are largely
   contradicted — all crates, the prelude, the CI isolation job and the benchmark report ship.
2. **The workspace is bigger than run 3 describes.** Ten library crates ship, not the six the
   M5/M6 overviews and `build-benchmarks.md` assume. `paladin-herald`, `paladin-storage`,
   `paladin-content`, `paladin-notifications` and `paladin-web` came from later milestones.
3. **Milestone/tier numbering collides.** The M4 overview is titled "Milestone 1", the M5
   overview "Milestone 2", and PRDs cross-reference "Milestone 1 / Epic 2" meaning
   Milestone 4 Epic 2. Same defect class as the run-2 Milestone 3 epic-numbering conflict.
4. **Five documented positions are contradicted by shipped code** and must not be planned
   as-is: edition 2021, the "exhaustive" 6-dependency `paladin-core` allowlist, `vision`
   gating the encryption crates, the MCP transport feature flags, and the `paladin-cli` crate.
5. **STABLE_API.md and the M4 doc deliverables are relocated, not missing** — they ship as
   mdbook pages under `docs/src/api-reference/` after the Milestone 11 overhaul. Do not plan
   them as gaps.
6. **Five verified open defects** (see WARNINGS 8-13): `paladin-ports` doctests disabled with a
   named "Task 7.0" follow-up; the `api-surface` CI job broken by a stale
   `project/current-exports.txt` path after the `.project` rename; zero `#[deprecated]`
   annotations; three CLI-only dependencies still unconditional; three competing `TokenUsage`
   definitions. All are small, concrete and high-value.

## Conflicts

- **Blockers: 0** (run 1: 0, run 2: 0, run 3: 0)
- **Competing variants (warnings): 39** (run 1: 8, run 2: 18, run 3: 13)
- **Auto-resolved / informational: 67** (run 1: 11, run 2: 28, run 3: 28)

Run-3 warnings in descending planning impact:

1. Milestone numbers and refactoring-tier numbers collide across all three milestones
2. Competing Rust edition for the workspace crates (2021 versus 2024; tree is mixed)
3. `paladin-core`'s dependency allowlist is declared exhaustive and then contradicted
4. Competing ownership of `PaladinResult` / `StopReason` / `TokenUsage` (decision record
   versus Epic 2 PRD — precedence hands the win to the architecturally wrong answer)
5. Competing location for the LLM configuration bridge
6. `vision` feature scope contradicts the dependency audit and would break the build
7. Facade re-export policy for the M6 relocations is unsettled and self-contradictory
8. `paladin-ports` doctests disabled in shipped code against an explicit PRD requirement
9. The `api-surface` CI job is broken by a stale baseline path
10. Deprecation annotations required by M4 Epic 2 do not exist anywhere in the tree
11. Three competing `TokenUsage` definitions ship simultaneously
12. CLI dependency isolation is incomplete while its task list shows no open items
13. The build-benchmark report's verdict contradicts its own measurements

**Run 3 closed five earlier warnings using shipped code** — see INFO entries 17-21 and
`code-verification.md`: the run-1 `BattalionResult` variant (merged superset shipped), the
run-1 `BattalionConfig` variant (Epic 4 form shipped), the run-2 `metadata_output_dir`
three-owner conflict (one owner; `CommanderConfig` never built), the run-2 competing
`ErrorStrategy` variant sets (two distinct enums in two crates), and the run-1 Battalion base
module path. Net open variant count is therefore lower than the raw arithmetic suggests.

Cycle detection: run-3 cross-ref graph is acyclic (32 nodes, 3 in-set edges, max depth 2,
cap 50). Runs 1 and 2 were also acyclic.

Security note (carried from run 2): one ingested DOC contains a plaintext OpenAI API key. The
value was not copied into any intel or report file. The user has confirmed it is rotated.

## Files

- Conflicts report: `/workspace/.planning/INGEST-CONFLICTS.md` (cumulative, runs 1-3;
  0 blockers, 39 warnings, 67 info)
- Decisions: `/workspace/.planning/intel/decisions.md` (no entries; run-3 section explains the
  one Approved-status decision record and why it is not locked)
- Requirements: `/workspace/.planning/intel/requirements.md` (348 entries)
- Constraints: `/workspace/.planning/intel/constraints.md` (no entries; run-3 inventory is the
  largest so far)
- Context: `/workspace/.planning/intel/context.md` (73 topics)
- **Direct code verification: `/workspace/.planning/intel/code-verification.md`** — HIGHEST
  precedence. The run-3 section is substantial: 22 verified-shipped claims, 6 resolved variants,
  6 verified-open defects, and a 12-row table of requirement text contradicted by shipped code
- Deterministic completion state: `/workspace/.planning/intel/task-completion-state.md`
- Committed codebase map: `/workspace/.planning/codebase/*.md`
- Classifications: `/workspace/.planning/intel/classifications/` (run 1),
  `classifications/run-02/` (run 2), `classifications/run-03/` (run 3)

## Status

**AWAITING USER** — 0 blockers, but 39 competing variants across runs 1-3 need a decision
before routing. Supersession is expected in this corpus and none of the variants was resolved
by the synthesizer; each is preserved unmerged with the later position noted and, where shipped
code settles it, a pointer to `code-verification.md`.

Three run-3 warnings gate roadmap structure rather than content and should be settled first:

1. **The milestone/tier numbering collision (warning 1)** because milestone numbers are
   provenance keys across 9 of the 19 run-3 DOCs, and this is the second numbering conflict in
   the corpus.
2. **The `PaladinResult` ownership conflict (warning 4)** because it is the one place where
   mechanical precedence produces an architecturally wrong answer — a PRD overriding an
   Approved decision record — and it determines whether the Epic-1 upward-dependency fix
   survives future work.
3. **The M6 facade re-export policy (warning 7)** because it decides whether Milestone 6 was a
   non-breaking internal refactor or a breaking change requiring a major version bump.

Five verified open defects (warnings 8-12) are the clearest forward-work candidates in the
corpus so far. Unlike the run-1/run-2 open-checkbox counts, each is confirmed against the tree
and each has a concrete, small fix.

## Roadmapper notes

- Honour the Roadmap Extension Protocol at the end of `ROADMAP.md`: **new phases start at
  Phase 7**, Phases 1-6 are never renumbered, the `### Phase N:` header format is preserved,
  `REQ-*` IDs are the merge keys, and later positions supersede rather than silently editing
  earlier ones. The `VERIFY-*` and `CLOSE-*` task prefixes are already spent — use new prefixes.
- Do not re-plan completed work. M4 is 93.2% complete (20 open, all in
  `tasks-harden-port-traits-stable-api.md` and all corroborated by code), M5 is 96.4% (17 open,
  mostly contradicted by code), M6 is 100% (0 open, confirmed by code). The 9-crate workspace
  demonstrably shipped — 10 library crates plus `doc-examples` exist.
- Treat every open checkbox count as a claim to verify against `code-verification.md` and
  `.planning/codebase/` first. Run 3 is the first run where a count proved trustworthy.
- Record supersession chains; do not raise relocation as contradiction. Every `src/…` path in
  runs 1-2 and several in run 3 are historical.
- Runs 4-5 will cover Milestones 7-12, Deferred-QA-CICD-Completion and project-management, and
  will merge into these same intel files.
