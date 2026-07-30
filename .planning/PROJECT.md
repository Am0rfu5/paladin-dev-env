# Paladin

## What This Is

Paladin (root crate `paladin-ai`) is a Rust workspace that provides an enterprise AI
orchestration framework: autonomous agents (**Paladins**) coordinated through eight multi-agent
patterns (**Formation** sequential, **Phalanx** concurrent, **Campaign** DAG, **Chain of
Command** hierarchical, **Conclave** expert synthesis, **Council** group discussion, **Grove**
tree-based routing, **Maneuver** Flow DSL), assembled behind hexagonal ports so that pluggable LLM
providers (OpenAI, Anthropic, DeepSeek, Mock), MCP tool servers, short-term conversation memory
(**Garrison**), long-term semantic memory with RAG (**Sanctum**), multi-modal vision
(**Sentinel**), state persistence (**Citadel**), output formatters (**Herald**), a CLI
(**Armory**) and an HTTP API are all swappable adapters.

Its audience is Rust developers and teams embedding agent orchestration inside their own
services — not end users of a hosted product.

The product **already works**. It is a brownfield project at v0.7.0 with a 9-crate Cargo
workspace, 22 runnable examples, a multi-arch Docker image and reference Kubernetes manifests.
This planning setup exists to close out and verify milestones that already shipped, not to build
the framework.

## Core Value

A Rust developer can compose and run multi-agent workflows against any supported LLM provider
through stable port abstractions — without their own domain code depending on a provider,
transport, or storage implementation.

## Success Metric (derived — not user-specified)

No developer-facing success metric was supplied. This one is derived strictly from the
measured evidence in the ingest set (`Epic_10/task6.0-validation-report.md`, dated
2026-01-27; `unit-test-improvements/COVERAGE_ANALYSIS.md`; and
`Milestone_3-Completion/RELEASE_NOTES_MILESTONE_3.md`):

**On a clean clone of the release branch, `cargo fmt --check`, `cargo clippy -- -D warnings`
and `cargo test --workspace` all pass with zero failures and zero warnings — and `cargo llvm-cov`
reports a coverage figure at or above whichever gate this project records, up from the measured
60.88% unit / 67.79% integration Milestone-1 baseline and the ~78% overall figure reported at
Milestone 3.**

Why not something more ambitious: the evidence supports claims about build, lint, test and
measured-coverage state. It does not support performance, throughput, or onboarding-speed
claims. The Milestone-1 benchmark suites were disabled; benchmarks now exist per-crate but no
baseline document has been produced, so no verified performance number exists. The
"< 15 minutes to first working agent" figure in Epic 10's PRD and the "90% of new users run
their first agent within 5 minutes" figure in Epic 18's PRD were both never measured. Those
remain documented *targets*, tracked in Phase 3 and Phase 4 respectively — not the current metric.

Reported test totals are deliberately excluded from the metric: across the corpus they run
999 → 1,292 → 1,674 → 1,628 → 853, i.e. not a monotonic series, so no single figure is
trustworthy enough to anchor a gate.

## Requirements

### Validated

Shipped in the v0.7.0 workspace. Full per-requirement ledgers: `.planning/REQUIREMENTS.md` →
*Milestone 1 as-shipped ledger* (115 IDs) and *Milestone 2-3 as-shipped ledger* (118 IDs).

**Milestone 1 — the MVP framework** (confirmed by task-list checkbox state, 1,817 of 1,857 items,
and the codebase map):

- ✓ Paladin domain foundation — entity, builder, config, port, execution service with reasoning
  loop / retry / circuit breaker / stop words / timeout, error types, tracing, mock LLM (Epic 1)
- ✓ Garrison memory — entries, windowing and eviction, port + long-term port, in-memory and
  SQLite adapters, tokenizer-based counting, Paladin integration, config, errors (Epic 2)
- ✓ Arsenal tool system — Armament/Call/Result domain types, ports and registry, MCP client and
  transports, builder integration, timeout/concurrency controls, graceful degradation, context
  injection (Epic 3)
- ✓ Battalion orchestration — Formation, Phalanx, Campaign, error strategies, retry policy,
  status, logging, cancellation (Epic 4; Chain of Command needs verification — see Active)
- ✓ Commander strategy router — strategy enum, construction validation, Auto rule-based
  selection, unified `execute()`, error strategies, config passthrough, service composition
  (Epic 5)
- ✓ Provider expansion — capability-aware `LlmPort`, DeepSeek and Anthropic adapters, provider
  factory and per-Paladin selection, backward compatibility, error mapping, docs (Epic 6)
- ✓ Citadel state persistence — Paladin and Battalion state serialization, autosave, restore,
  checkpoint resumption, port and file adapter, state directory management (Epic 7)
- ✓ Herald output formatting — trait, JSON/Markdown/Table formatters, registry, streaming,
  configuration, default + per-execution override, error fallback (Epic 8)
- ✓ Armory CLI — `paladin agent|battalion|arsenal` command tree, YAML config schema, env-var
  API keys, validation and exit codes, output formatting, interactive prompts (Epic 9)
- ✓ Validation and documentation — integration test infrastructure, rustdoc and doc tests,
  24 user/technical docs, 22 examples, multi-arch Docker, Kubernetes manifests, GitHub Actions
  release and integration workflows (Epic 10)

**Milestones 2-3 — the capability build-out and its completion** (component-level file evidence in
the tree, verified by direct inspection on `release/v0.7.0`; per-criterion confirmation is Phase 5):

- ✓ Sanctum long-term memory — `EmbeddingPort`, OpenAI embedding adapter, `SanctumPort`,
  in-memory and **Qdrant** adapters (`qdrant-client` 1.14, `qdrant` feature), Memory/MemoryType/
  decay domain model, configuration (Epics 11-12). The Epic 11 summary's "Qdrant DEFERRED" record
  is stale — `intel/code-verification.md` verifies it shipped.
- ✓ RAG pipeline — `RagRetrievalService`, `MemoryExtractionService`, `MemoryExtractionStrategy`,
  `RagConfig`, builder wiring (`with_sanctum`, `with_embedding_port`), context injection in the
  documented execution flow (Epic 12)
- ✓ Sentinel vision and documents — `VisionContent`/`ImageDetail`/`VisionRequest`/`VisionError`,
  OpenAI and Anthropic vision modules, **two coexisting ports** (`vision_llm_port.rs`,
  `vision_port.rs`), both Paladin entry points (`enable_vision`, `execute_with_vision`), PDF
  extractor, `DocumentPort`, CLI `--image`/`--document` (Epics 13, 20). The Milestone 3 release
  notes list this as unshipped Milestone-4 work; that is a stale forward-look.
- ✓ Autonomous agents — `MaxLoops::{Fixed, Auto}` enum, `PlanningService`, `TaskPlan`/`Subtask`,
  `PromptGenerationService`, `TemperatureService` with task-type bands, `HandoffService`,
  `HandoffStrategy`, the handoff tool, `AutonomousConfig` (Epics 14, 21)
- ✓ Conclave (mixture-of-agents) — domain model, parallel expert execution service with retry and
  partial success, Commander strategy, CLI/YAML, examples (Epic 15). Verified shipped against
  129 unchecked task items.
- ✓ Council and Grove — Council domain model with turn strategies and termination conditions plus
  its execution service; Grove with trees, tree agents, keyword/semantic/LLM routing and its
  execution service; both wired into Commander with examples and integration tests (Epic 16)
- ✓ Maneuver Flow DSL — grammar, lexer/parser/AST, `Maneuver` domain model, recursive execution
  service, ASCII and Mermaid visualizer, top-level CLI command group (Epic 17)
- ✓ CLI consolidation and enhancement — `src/cli` deleted and everything consolidated under
  `src/application/cli/`; `onboarding`, `setup-check`, `features`, `muster`, `council` and
  `maneuver` commands; rich formatters with `insta` snapshot tests (Epics 17.5, 18)
- ✓ Herald consolidation — placeholder types removed from `herald.rs`, real domain types imported,
  `TokenUsage` extracted to its own container (Epic 19)
- ✓ Battalion and Commander hardening — `PaladinRegistry` port and in-memory adapter,
  per-Paladin timing and token metrics on `BattalionMetadata`, Commander metadata export
  (Epic 22). One gap remains — see Active.
- ✓ CLI config and infrastructure completion — YAML garrison and arsenal/MCP configuration wired
  into `PaladinBuilder`, `MockLlmAdapter` and `MockArsenalPort`, three-tier test strategy,
  `SchedulerPort` with a `tokio-cron-scheduler` 0.13 adapter, content-deliverer scheduling
  (Epic 23). The most reliably complete epic in the run-2 corpus.
- ✓ Test hardening — benchmarks relocated into per-crate `benches/`, CLI snapshot suite, live-API
  test suite behind a feature flag (Epic 24)

### Active

Current scope is **milestone close-out**: make the planning record match the shipped code, resolve
the contested type and gate definitions, close the residual functional gaps, and make the quality
numbers real. **34 requirements across 6 phases** — see `.planning/ROADMAP.md`.

*Milestone 1 close-out (Phases 1-4, 25 requirements):*

- [ ] Reconcile `.planning/` against shipped v0.7.0 code; produce a cited status ledger for the
      ~40 outstanding Milestone-1 task items (RECON-01, RECON-08)
- [ ] Record one answer per competing variant pair — `BattalionConfig`, `BattalionResult`,
      Formation minimum Paladin count, temperature range, Herald trait signature, coverage gate
      (RECON-02 … RECON-07)
- [ ] Close residual functional gaps — Chain of Command completion and tests, Battalion
      integration/performance tests, Herald on the Battalion execution path, Commander result
      normalization and telemetry export, the one failing Auto-selection test, Garrison final
      validation, and applying the reconciled type definitions in code (GAP-01 … GAP-07)
- [ ] Make quality numbers real — coverage to the recorded gate, no 0%-coverage first-party
      files, integration coverage ≥ 70%, `#[ignore]`d error-path tests activated, MCP failure
      modes tested, benchmarks re-enabled with documented baselines (QUAL-01 … QUAL-05)
- [ ] Release coherence — version metadata agreement, one valid Rust edition across crates,
      advisory posture with written rationale, documentation final review and a measured
      quickstart, and the full gate suite green in CI (REL-01 … REL-05)

*Milestone 2-3 close-out (Phases 5-6, 9 requirements):*

- [ ] Upgrade the Milestone 2-3 ledger from component-level file evidence to `file:line`
      per-criterion verdicts for all 118 run-2 requirements, recording the historical-path caveat
      (VERIFY-01)
- [ ] Verify the three open-checkbox blocks `code-verification.md` left unverified — Epic 22 (81),
      Epic 14 (45), Epic 24 (29) — producing a verdict per block, not a task list (VERIFY-02)
- [ ] Fix the Milestone 3 epic-numbering defect at its source and withdraw the release-notes claims
      verified absent from the tree (VERIFY-03)
- [ ] Record the two vision surfaces as deliberate coexistence, and answer whether Epic 13's
      encryption-at-rest requirement was consciously dropped (VERIFY-04)
- [ ] Extend the coverage answer across all four competing gate positions and place the two
      module-scoped gates relative to it (VERIFY-05)
- [ ] Record one answer for live-API-test behaviour when keys are missing (VERIFY-06)
- [ ] Close the one verified defect — Grove routing's hardcoded `model: "gpt-4"` at
      `grove_service.rs:537` (CLOSE-01) — plus whatever VERIFY-02 proves outstanding (CLOSE-02) and
      apply the Phase 5 decisions that have code consequences (CLOSE-03)

### Out of Scope

- **Milestones 4-12, Deferred-QA-CICD-Completion and project-management scope** — not yet
  ingested (runs 3-5 of 5). Notably the nine-crate workspace decomposition itself is a Milestone 5
  deliverable, which is why every `src/...` path in the run-1 and run-2 corpus is historical.
  Shipped code still awaiting requirements: the Axum HTTP API (auth, rate limiting, OpenAPI, SSE
  streaming), notifications, and the content ingestion pipeline.
- **Re-planning shipped work** — Milestone 1 is 98% checked and Milestones 2-3 are shipped
  wholesale. Anything already satisfied by code is recorded in a ledger, not re-planned as a phase.
- **Converting open checkbox counts into requirements** — 542 items are unchecked, and the two
  largest blocks (Conclave 129, Sanctum 111) are verified shipped. Verification precedes planning.
- **Picking winners among the 16 competing variant groups** — recording answers is in scope;
  choosing inside an ingest artefact is not. Explicitly requested: variants are expected and
  settling past disagreements is not the goal of this ingest.
- **Migrating between the two shipped vision port surfaces** — both ship deliberately;
  `code-verification.md` says confirm intent before planning a migration.
- **Decomposing the three oversized service files** (2,757 / 2,294 / 1,840 lines) — real tech
  debt, but no ingested requirement demands it. Tracked as v2.
- **Clone/lock-contention performance work** — the 383 `.clone()` calls and the 9 orchestrator
  locks are flagged in `codebase/CONCERNS.md`, but optimizing before Phase 3 restores
  benchmark baselines would be guesswork. Tracked as v2.

## Context

**Where the code actually is.** The committed codebase map (`.planning/codebase/`, refreshed
2026-07-30) plus `.planning/intel/code-verification.md` are authoritative on current state, and
both are *ahead of* the ingested documents in several places:

- MCP is implemented on the official `rmcp` 2.1.0 SDK with STDIO and **Streamable-HTTP**
  transports — the Milestone-1 PRD specified a hand-rolled client with an **SSE** transport.
- Herald is already wired into `PaladinExecutionService` (`with_herald`, used in the execution
  path) and `chain_of_command_service.rs` exists — both were listed as incomplete in the
  January task lists. **The task lists are a point-in-time snapshot; the code is the arbiter.**
- Epic 9 declared "no REPL or interactive shell" a non-goal; an interactive REPL now ships. A
  documented non-goal has already been superseded by later work — which is exactly why nothing in
  this file is treated as locked.
- Conclave, Council, Grove, the Maneuver Flow DSL, Sentinel vision and the Qdrant Sanctum adapter
  are all **verified shipped** despite documents that variously declare them deferred, unstarted,
  or scheduled for Milestone 4.

**The precedence order this project uses**, most authoritative first:
**shipped tree → `.planning/codebase/` map → `intel/code-verification.md` → PRD → DOC →
task-list checkbox.** Two ingest runs have now independently found checkbox state understating
shipped reality, so it sits last by evidence rather than by preference.

**Two coexisting vision surfaces, on purpose.** Epic 13's `VisionCapableLlm` lineage
(`crates/paladin-ports/src/output/vision_llm_port.rs`, reached via
`PaladinBuilder::enable_vision`) and Epic 20's `VisionPort` lineage (`vision_port.rs`, reached via
`PaladinExecutionService::execute_with_vision`) both ship. The ingest report preserved these as
competing variants; `code-verification.md` overrides that — they are two coexisting ports, not an
unresolved contradiction. Confirm intent before planning any migration (VERIFY-04).

**One documentation defect is propagating through the corpus.**
`RELEASE_NOTES_MILESTONE_3.md` numbers Milestone 3 Epics 19-23 as Conclave / Council / Grove /
Maneuver / Commander Enhancement. Those four patterns are Milestone **2** features (Epics 15, 16,
16, 17), all verified shipped. The Milestone 3 plan, all six epic definitions, every PRD and every
task list instead use 19 = Herald consolidation, 20 = Vision, 21 = Autonomous, 22 = Battalion
hardening, 23 = CLI/Config, 24 = Test hardening. **The plan/epic-definition numbering is
authoritative** (8 of 9 documents plus all task lists) and is the only mapping used in
`ROADMAP.md` and `REQUIREMENTS.md`. Four further documents mislabel epic numbers in
cross-references. Fixing this at the source is VERIFY-03.

**Measured quality state.** Milestone 1 (Epic 10 Task 6.0, 2026-01-27): 1,091 tests passing / 0
failures (706 unit, 385 integration, 133 doc); `cargo fmt --check` clean; `cargo clippy -- -D
warnings` at 0 warnings after fixing 102 across 48 files; unit coverage 60.88%, integration 67.79%;
2 medium transitive advisories; 22/22 examples compiling; Docker image 112 MB built in 5m31s; all
benchmarks disabled. Milestone 3 release notes report ~78% overall coverage, Battalion
orchestration overhead < 10 ms for 100+ concurrent battalions, Garrison queries < 50 ms on a
1,000-entry store, and Herald formatting at 0.0095 ms for a 10 KB result. Reported test totals
across the corpus (999 → 1,292 → 1,674 → 1,628 → 853) are not monotonic and none is treated as
authoritative.

**Version state is incoherent right now.** Branch `release/v0.7.0`, workspace `Cargo.toml`
version `0.6.0`, latest tag `v0.5.1`. Three different answers to "what version is this".
Phase 4 resolves it.

**Ingest program.** This planning setup was bootstrapped from `.project/Milestone_1-MVP`
(36 docs) in run **1 of 5**, then merged `.project/Milestone_2-Missing_features` +
`.project/Milestone_3-Completion` (45 docs) in run **2 of 5** — 81 documents so far (26 PRD,
55 DOC, 0 ADR, 0 SPEC), 233 requirements, 30 preserved variant entries, 0 blockers. Run 3 covers
Milestones 4-6, run 4 Milestones 7-8, run 5 Milestones 9-12 plus Deferred-QA-CICD-Completion and
project-management. This document, `REQUIREMENTS.md` and `ROADMAP.md` are structured so those runs
**append** (new milestone sections, continuous phase numbering from Phase 7) rather than
restructure. Note: run-1 text in some files still says "run 1 of 14" — same run, renumbered
program.

**Nothing here is locked.** Zero ADR-typed and zero SPEC-typed documents exist across all 81
ingested documents, so every technical assertion in the ingested material sits at PRD or DOC
precedence and is auto-overridable. Run 2 alone produced eight documented supersessions of run-1
requirements. Where a contested definition matters, this project points at the codebase map or the
shipped code rather than declaring a winner.

**The strongest ADR candidate in the corpus, if the user later wants to protect a decision:**
`Epic_17.5/epic17-5.md` decides that the CLI belongs in `src/application/cli` because "CLI is an
input adapter in the application layer, not infrastructure", directs deletion of the entire
`src/cli` tree, and gives a full rationale plus target layout. **The decision is already applied in
code** — `src/cli` is absent from the tree and `src/application/cli/commands/` carries the full
command set. But it has no ADR status field, no Consequences section and no `locked` flag, so it
sits at DOC precedence and loses to any PRD — including Epic 17's own PRD, which places CLI
integration under `src/infrastructure/adapters/cli/`. Promoting it to a real ADR is the cheapest way
to stop that from being re-litigated. **Not done here:** manufacturing it as a locked decision
would fabricate authority the corpus does not contain.

## Constraints

- **Tech stack**: Rust workspace (9 member crates + facade), Tokio async throughout, Serde,
  SQLx, `thiserror` — pinned toolchain `rust-toolchain.toml` at 1.97.1. Not negotiable; the
  entire public surface is Rust traits. Optional capabilities are feature-gated: `qdrant`
  (qdrant-client 1.14), `scheduler` (tokio-cron-scheduler 0.13), `vision`, `live-api-tests`.
- **Architecture**: Hexagonal, dependencies flow inward only (core → nothing; ports → core;
  adapters → core + ports). Bypassing a port to import an adapter directly is an anti-pattern
  the codebase map calls out by name. The CLI is an **input adapter in the application layer**
  (`src/application/cli/`), not infrastructure — see the ADR-candidate note in Context.
- **Ubiquitous language**: Medieval military terms (Paladin, Battalion, Formation, Phalanx,
  Campaign, Chain of Command, Conclave, Council, Grove, Maneuver, Commander, Garrison, Arsenal,
  Armament, Citadel, Herald, Armory, Sanctum, Sentinel, Quest) are mandatory in code, docs and
  comments — they are the domain vocabulary, not decoration.
- **Error handling**: No `unwrap()`/`expect()`/`panic!` in library code; return `Result`. Layer-
  specific error enums converted at boundaries via `From`. `codebase/CONCERNS.md` lists existing
  violations to work down, not to imitate. Note the deliberate exception now shipping:
  `require_api_key()` in the live-API test harness panics by design — contested, see VERIFY-06.
- **Optional features degrade gracefully, never fatally**: RAG retrieval failure returns empty
  context and continues; memory extraction failure must not affect the Paladin response; a
  disabled autonomous layer must never fail core execution; Herald formatting errors fall back.
- **Methodology**: TDD (Red-Green-Refactor), rustdoc with compiling examples on all public
  items, `make clean-code` before committing, conventional commits.
- **Testing must work offline**: unit tests run with no external dependencies; anything needing
  Redis/MinIO/Qdrant/live APIs is feature-gated or `#[ignore]`d. The shipped three-tier strategy is
  Tier 1 always-in-CI, Tier 2 Docker-gated, Tier 3 API-key-gated. Provider API keys come from
  environment variables only — never CLI args, never config files, never logs.
- **Deploy targets**: Docker (distroless/slim, multi-arch amd64 + arm64) and Kubernetes.
  Image budget < 500 MB, pod startup < 30 s — both currently met.
- **Licensing/repo**: MIT, `github.com/DF3NDR/paladin-dev-env`.
- **Edition declaration is currently invalid**: several crates declare `edition = "2024"`, others
  `"2021"`. Builds succeed today but the mix is brittle (`codebase/CONCERNS.md`). One consistent
  answer is required before release (REL-02).

## Key Decisions

<!-- LOCKED DECISIONS (from ADR-typed documents). Empty by evidence, not by omission. -->

| Decision | Rationale | Outcome |
|----------|-----------|---------|
| *(none)* | Ingest runs 1-2 surfaced **0 ADR-typed and 0 SPEC-typed documents across 81 files** (26 PRD, 55 DOC). No source doc carried an ADR status field, a Decision/Consequences structure, or `locked: true`. Nothing is recorded here speculatively. | — Pending |

Everything asserted in the ingested PRDs and DOCs is **supersedable** — demonstrated, not
theoretical: run 2 produced eight documented supersessions of run-1 requirements (see *Superseded
but preserved* in `REQUIREMENTS.md`). The first real entries in this table are expected from
Phase 1 (six ADRs, one per competing variant pair), Phase 5 (four recorded answers), and any
ADR-typed documents arriving in ingest runs 3-5.

The strongest ADR candidate found so far is Epic 17.5's CLI-location decision — see Context. It is
deliberately **not** entered in this table, because doing so would manufacture a locked decision
from a DOC-precedence assertion.

---
*Last updated: 2026-07-30 after ingest run 2 of 5 (`.project/Milestone_2-Missing_features` +
`.project/Milestone_3-Completion`, 45 docs; cumulative 81 docs, 233 requirements)*
