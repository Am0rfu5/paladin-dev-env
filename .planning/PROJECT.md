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

The product **already works**. It is a brownfield project at v0.7.0 with a Cargo workspace of
**ten library crates** (`paladin-core`, `paladin-ports`, `paladin-battalion`, `paladin-herald`,
`paladin-llm`, `paladin-memory`, `paladin-storage`, `paladin-notifications`, `paladin-content`,
`paladin-web`) plus a `doc-examples` crate and the root facade package `paladin-ai`, 22 runnable
examples, a multi-arch Docker image and reference Kubernetes manifests. This planning setup exists
to close out and verify milestones that already shipped, not to build the framework.

*(Corrected by ingest run 3: this file previously said "9-crate workspace", and the Milestone 5/6
source documents assume six. The tree was read directly — `crates/` holds eleven directories, ten
of them library crates. **Closed by ingest run 4:** all ten now have an ingested requirement.
`paladin-storage`, `paladin-notifications`, `paladin-content` and `paladin-web` come from Milestone
7 Epic 1's extraction PRD and its cost-benefit gate; `paladin-herald` was created by the 2026-06-04
facade-cleanup reconciliation rather than by any PRD — inside an Epic whose non-goals named it as
out of scope — which is exactly why no ingested requirement described it and why the "9 crates"
figure was wrong.)*

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
*Milestone 1 as-shipped ledger* (115 IDs), *Milestone 2-3 as-shipped ledger* (118 IDs) and
*Milestone 4-6 as-shipped ledger* (115 IDs).

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

**Milestones 4-6 — the refactor that restructured Milestones 1-3** (the best-evidenced block in
this planning set: 22 claims verified directly against `Cargo.toml` contents, type definitions and
file existence during ingest run 3, recorded in `intel/code-verification.md`):

- ✓ Feature-flag expansion — `default = ["llm-openai"]` replacing the old three-flag default;
  per-provider `llm-openai` / `llm-anthropic` / `llm-deepseek` / `llm-all`; subsystem flags
  `content-processing`, `web-server`, `notifications`, `vision`; a `full` convenience flag; and a
  `feature-flags.yml` CI matrix. The planned `mcp-arsenal` flag was **eliminated** by a dated PRD
  note and no MCP flag exists (Milestone 4 Epic 1)
- ✓ CLI isolation — a single `cli` feature gating the whole `src/application/cli/` tree, and
  `[[bin]] paladin-cli` with `required-features = ["cli"]`, plus a `cli_isolation` test target run
  in CI. Three CLI-only dependencies are still unconditional — see Active (Milestone 4 Epic 3)
- ✓ API-surface tooling — `scripts/{extract-public-api,check-api-surface,check-deprecations,check-all-examples}.sh`,
  `final-api.txt`, `api_surface_current.txt`, and the stable-API catalogue (now an mdbook chapter).
  The CI job that consumes them is broken — see Active (Milestone 4 Epic 2)
- ✓ Cargo workspace — root `[workspace] members = [".", "crates/*"]` with `resolver = "2"` and a
  full `[workspace.dependencies]` pin set; `paladin-core`, `paladin-ports`, `paladin-battalion`,
  `paladin-llm` and `paladin-memory` all extracted, `src/application/ports/` **fully deleted**
  (no shim), `paladin::prelude` shipped, and a `crate-isolation` CI job proving each crate builds
  alone (Milestone 5, all six epics)
- ✓ Upward-dependency resolution — `PaladinResult`, `StopReason`, `TokenUsage`, `RegistryError`
  and `HandoffError` moved into `paladin-core` with the ports reduced to re-exports, per the
  corpus's only Approved-status decision record. `PaladinError` was deliberately excluded
  (Milestone 5 Epic 1)
- ✓ Config decomposition — `application_settings.rs` **deleted** and replaced by per-domain
  modules split across the facade (`src/config/`), `paladin-memory` and `paladin-llm`, with an
  `EnvOverridable` trait and a `read_env` helper replacing ~30 copies of the env-override pattern
  (Milestone 6 Epic 1)
- ✓ Orchestration relocation — six manager-layer services moved out of
  `src/core/platform/manager/` and renamed to `*Orchestrator`, landing under
  `src/application/services/`; the manager module retains only `content_service`, `event_manager`
  and `user_service` (Milestone 6 Epic 2)
- ✓ Maneuver DSL co-location — the lexer, AST, parser, domain type, execution service and
  visualizer all consolidated under `crates/paladin-battalion/src/maneuver/`, and every parser
  reference removed from `paladin-core`. This **reverses** a Milestone 5 requirement that had just
  moved the parser into `paladin-core` (Milestone 6 Epic 3)
- ✓ `CircuitBreaker` relocation — moved to `src/infrastructure/resilience/`, with the old
  application-layer path **intentionally retired** and no re-export left behind. A `paladin-infra`
  crate and a `CircuitBreakerPort` trait were both explicitly rejected (Milestone 6 Epic 4)

**Milestone 7 — production hardening and the first published release** (verified against the tree
during ingest run 4):

- ✓ Four further crate extractions behind a written cost-benefit gate that returned **four Go, zero
  Defer** — `paladin-storage`, `paladin-notifications`, `paladin-content`, `paladin-web`
  (Milestone 7 Epic 1)
- ✓ Production build infrastructure — `Dockerfile.chef` with a pinned `cargo-chef 0.1.77` and a
  workspace recipe, ten per-crate `make test-*` targets, a workspace feature-flag CI matrix, and a
  publish dry-run job (Milestone 7 Epic 2)
- ✓ Benchmark migration — all five suites moved into their owning crates with **zero `.disabled`
  files** left anywhere, three obsolete suites deprecated rather than restored, and a non-blocking
  `benchmark-regression-signal` CI job (Milestone 7 Epic 3)
- ✓ API stabilization through a real release — **`v0.1.0-rc.1` at commit `a9530fc`**, all ten crates
  published at `0.1.0` with a GO sign-off and docs.rs verification, and the crates.io collisions
  that forced the `paladin-ai` / `paladin-ai-core` package renames (Milestone 7 Epic 4).
  **This is history, not current state** — the tree is at `0.6.0` on `release/v0.7.0`

**Milestone 8 — facade cleanup, and a reconciliation that went further than the plan** (verified
during ingest run 4):

- ✓ 25 dead files deleted along with five orphaned directories; `src/core/` reduced to **exactly
  six files** (Milestone 8 Epic 2)
- ✓ `use_cases` → `services` renamed in **both** the facade and `paladin-content`, as a clean break
  with no compatibility alias — a workspace-wide grep for `use_cases` returns zero
  (Milestone 8 Epics 4 and 6)
- ✓ `paladin-web` consolidated on axum — actix-web removed entirely and **banned in `deny.toml`** —
  with the three delivery endpoints revived as mounted axum routes rather than deleted
  (Milestone 8 Epic 7)
- ✓ The 2026-06-04 reconciliation — **fifteen commits, ~10,250 net LOC removed, one new leaf
  crate** — which found the Epic 1 audit had mis-described ~4,400 LOC of orphaned uncompiled
  duplicates as "active bridges that stay", then executed the relocations Epic 3 had deferred to
  Milestone 9: `FileCitadel` → `paladin-memory`, MinIO/S3 and Redis → `paladin-storage`,
  `HashMapPaladinRegistry` → `paladin-battalion`, Herald formatters → the new `paladin-herald`

### Active

Current scope is **milestone close-out**: make the planning record match the shipped code, resolve
the contested type and gate definitions, close the residual functional gaps, make the quality
numbers real, and make the release and security gates actually hold. **62 requirements across 11
phases** — see `.planning/ROADMAP.md`.

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

*Milestone 4-6 close-out (Phases 7-8, 12 requirements):*

- [ ] Upgrade the Milestone 4-6 ledger to `file:line` per-criterion verdicts for all 115 run-3
      requirements, and record the corrected workspace shape (ARCH-01)
- [ ] Fix the milestone/tier numbering collision at its source — the Milestone 4-6 overviews
      number themselves "Milestone 1/2/3" by refactoring tier, and PRDs cross-reference
      "Milestone 1 / Epic 2" meaning Milestone 4 Epic 2 (ARCH-02)
- [ ] Record one answer per run-3 variant pair: Rust edition, the `paladin-core` dependency
      allowlist, ownership of `PaladinResult`/`StopReason`/`TokenUsage`, and the LLM config bridge
      location — all four settled by shipped code, three of four PRDs unamended (ARCH-03)
- [ ] Record the Milestone 6 facade re-export policy and whether it makes Milestone 6 a breaking
      change requiring a major version bump (ARCH-04)
- [ ] Correct the five documented positions shipped code contradicts, and record that the four
      "missing" documentation deliverables are relocated into the mdbook (ARCH-05)
- [ ] Answer and document the binary-target architecture question Milestone 4 Epic 3 left open
      (ARCH-06), and make the ≥ 50% incremental-rebuild target falsifiable (ARCH-07)
- [ ] Fix the `api-surface` CI job, which has failed on every run since the `project/` → `.project/`
      rename left its baseline path stale (DEBT-01)
- [ ] Add the `#[deprecated]` annotations Milestone 4 Epic 2 FR-8 requires — zero exist in the tree
      — or withdraw the requirement with a recorded reason (DEBT-02)
- [ ] Re-enable `paladin-ports` doctests and drop the CI `--exclude`, so the ~25 port traits have
      executing examples again (DEBT-03)
- [ ] Finish CLI dependency isolation — `structopt`, `colored` and `comfy-table` still compile into
      library-only builds (DEBT-04)
- [ ] Consolidate the three shipped `TokenUsage` definitions to one (DEBT-05)

*Milestone 7-8 close-out (Phases 9-11, 16 requirements):*

- [ ] Reconcile the RustSec exception set across the four surfaces that encode it differently, give
      every suppressed advisory an owner and an expiry, and dispose of the **2026-09-30** acceptance
      before it lapses — the only dated item in the corpus (SEC-01)
- [ ] Settle whether the project is MIT or `MIT OR Apache-2.0` and make the manifests say so
      (SEC-02); add a crates.io name-collision guardrail earlier than dry-run, or accept the dry run
      with its cost recorded (SEC-03)
- [ ] Add the missing `paladin-herald` CHANGELOG (SEC-04) and stop `Dockerfile.chef`'s planner COPY
      list going stale on every crate addition (SEC-05)
- [ ] Upgrade the Milestone 7-8 ledger to `file:line` per-criterion verdicts for all 86 run-4
      requirements, with the 14 "superseded by outcome" entries unmissable (HARD-01)
- [ ] Make `facade-cleanup-RECONCILIATION-2026-06-04.md` the authoritative account of Milestone 8,
      preserving its three in-execution corrections, and record that Epics 3 and 6 are complete and
      `paladin-herald` exists despite the non-goal that forbade it (HARD-02)
- [ ] Record the version trajectory as history so REL-01 does not converge on an rc.1 figure
      (HARD-03), and close the fourth milestone-numbering collision (HARD-04)
- [ ] Decide whether the extracted-crate dependency rule permits optional feature-gated leaf-to-leaf
      edges — it is stated absolutely and violated once (HARD-05); decide whether PDF extraction is
      still supported, since `pdf = []` gates nothing while an advisory suppression assumes
      `pdf-extract` is in the graph (HARD-06); pick one `cargo doc` bar (HARD-07)
- [ ] Close deferred item D5 — the 17 `println!` occurrences across 6 files (FACADE-01) — and give
      D1-D4 decisions with owners instead of effort ratings (FACADE-02)
- [ ] Record both deliberately removed features with their reintroduction conditions intact,
      especially the `paladin-ml` leaf-crate placement condition (FACADE-03)
- [ ] Triage the Milestone 9 candidate list the reconciliation superseded, before ingest run 5
      reads it and re-plans relocations that already happened (FACADE-04)

### Out of Scope

- **Milestones 9-12, Deferred-QA-CICD-Completion and project-management scope** — not yet
  ingested (run 5 of 5). Shipped code still awaiting requirements: Milestone 12's Axum HTTP API
  surface (auth, rate limiting, OpenAPI, SSE streaming). **Run 4 closed the crate gap** — all ten
  library crates now have an ingested requirement.
- **Implementing the 14 requirements that shipped code superseded by outcome** — actix-web in
  `paladin-web`, the `storage-sqlite` flag, the per-crate ordered publish dry run, the `ml` feature
  gate, the Milestone 8 Epic 3 no-extraction mandate, the 160-file facade target (the tree reads
  136), and the root-path `STABLE_API.md` and `docs/*.md` deliverables the Milestone 11 overhaul
  relocated. Recording them is HARD-01; implementing them would undo shipped work.
- **Building `paladin-arsenal`, `paladin-sanctum` or `paladin-ml`** — none exists. The first two
  are named only by a superseded disposition record that contradicts its own governing PRD
  (FACADE-04 triages the list); the third is a *placement condition* on reintroducing a removed
  feature (FACADE-03), not a deliverable.
- **Treating any `v0.1.0-rc.1` artefact as current state** — the published-crate list, the docs.rs
  verification and the GO sign-off all describe `0.1.0`. HARD-03 records the trajectory as history.
- **Re-planning shipped work** — Milestone 1 is 98% checked, Milestones 2-3 are shipped wholesale,
  and Milestones 4-6 are verified shipped against the tree. Anything already satisfied by code is
  recorded in a ledger, not re-planned as a phase. That explicitly includes the entire workspace
  decomposition and all four Milestone 6 relocations.
- **Converting open checkbox counts into requirements** — 542 items are unchecked, and the two
  largest blocks (Conclave 129, Sanctum 111) are verified shipped. Verification precedes planning.
- **Picking winners among the 20 competing variant groups** — recording answers is in scope;
  choosing inside an ingest artefact is not. Explicitly requested: variants are expected and
  settling past disagreements is not the goal of this ingest. Where shipped code settles a variant,
  that is recorded as a **fact about the tree** at the top of the precedence order, not as a
  decision taken in a planning file.
- **Promoting the two ADR candidates into locked decisions** — doing so requires re-tagging the
  source documents via `--manifest` and re-running ingest, not an edit here. See Key Decisions.
- **Building `STABLE_API.md`, `docs/FEATURE_FLAGS.md`, `docs/MIGRATION.md` or
  `docs/CONFIGURATION.md`** — absent from the paths six run-3 documents name, but shipping as
  mdbook chapters under `docs/src/`. Recording the relocation is ARCH-05.
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

**Milestones 4-6 restructured everything Milestones 1-3 built — and it is all in the tree.**
Ingest run 3 is the first run where documents and code mostly *agree*. Verified directly:
`[workspace] members = [".", "crates/*"]` in the root `Cargo.toml`; `src/application/ports/` gone
entirely (full deletion, not a shim); `application_settings.rs` gone, replaced by
`src/config/{agents,arsenal,citadel,env_utils,file_storage,herald,notifications,queue,scheduler,settings,web_server}.rs`
plus `crates/paladin-memory/src/config/` and `crates/paladin-llm/src/config/`;
`src/application/use_cases/` gone entirely, with the orchestrators under
`src/application/services/`; `crates/paladin-battalion/src/maneuver/` holding the whole Flow DSL;
`src/infrastructure/resilience/circuit_breaker.rs`; `src/prelude.rs`; and the `crate-isolation`
job at `ci.yml:228`. **Relocation is not contradiction** — the supersession chains are recorded in
`REQUIREMENTS.md`.

**Five documented positions are contradicted by shipped code and must not be applied literally.**
`vision` gating `chacha20poly1305` and `zeroize` (shipped `vision = []` gates nothing; the two
crates serve user auth and Citadel encryption, so gating them would break
`cargo build --no-default-features` — the epic's own dependency-matrix audit said so and the PRD
was wrong); the MCP transport feature flags (none exist; the PRD's dated elimination note is what
shipped); `web-server` gating `actix-web` (shipped as axum only); a `paladin-cli` workspace crate
(never built — the CLI is a feature plus a binary target); and `src/application/use_cases/` as the
orchestration home (shipped under `src/application/services/`). Correcting these at source is
ARCH-05.

**Run 3 is the first run where a checkbox count proved trustworthy — and the first where one
overstated completion.** Milestone 4's 20 open items are real: `grep -rn '#\[deprecated' src crates`
returns 0, and `DEPRECATIONS.md` agrees. Milestone 6's 0 open items are real: all four relocations
are complete. But Milestone 4 Epic 3's task list is fully checked while three CLI-only
dependencies remain unconditional. The lesson is not "checkboxes understate" — it is
**"verify each count against the tree"**.

**Five verified open defects, all small.** The `api-surface` CI job fails on every run because
`ci.yml:171,181,186` and both scripts point at `project/current-exports.txt` while the file lives
at `.project/current-exports.txt` after commit `928c6d5` — so the project's only automated
public-API guard has been inert. Zero `#[deprecated]` annotations exist against Milestone 4 Epic 2
FR-8. `paladin-ports` sets `[lib] doctest = false` deferring the fix to an unwritten "Task 7.0",
and CI excludes the crate from `--doc`. Three CLI-only dependencies still compile into library
builds. Three `TokenUsage` structs ship where the decision record names one. These are Phase 8.

**The corpus now has a second ADR candidate, and it is stronger than the first.**
`Milestone_5/Epic_1/decisions/battalion-result-upward-dependency-decision.md` carries
`Status: Approved`, `Decision Date: 2026-05-13`, `Chosen Option: Option A`, a Rationale, a Rejected
Options section and an implementation checklist, with a full three-option trade-off analysis in its
`-options.md` sibling. It is the only decision/options pair in all 263 documents. It is
nevertheless manifest-typed **DOC**, so it sits at the lowest precedence tier and a PRD published
two days later contradicts it — which means mechanical precedence would pull `PaladinResult`,
`StopReason` and `TokenUsage` back out of `paladin-core` and reintroduce the exact upward
dependency the decision removed. **Two caveats on the record, both important:** it settles the
*location* of five value/error types and nothing else, and despite its filename it **never
mentions `BattalionResult`** — the run-1 `BattalionResult` variant is closed by shipped code, not
by this document.

**Two run-1/run-2 questions were closed by run-3 code verification.** `BattalionResult`'s field set
resolves to a merged superset at `battalion/mod.rs:549` that satisfies all three consumers, so
RECON-03 became a recording task and GAP-07 lost its code change. `BattalionConfig` resolves to the
Epic 4 form exactly, and `CommanderConfig` — the third claimed owner of `metadata_output_dir` —
does not exist anywhere in the tree. The competing `ErrorStrategy` variant sets turned out to be
two distinct enums in two different crates, which Milestone 6 physically separated.

**Milestone 7-8 is the first block where a document audits itself against the tree — and it is the
most reliable thing in the corpus.** `facade-cleanup-RECONCILIATION-2026-06-04.md` re-audited `src/`
file by file, found that the Epic 1 audit and the Epic 3 disposition record had described ~4,400 LOC
of *orphaned, uncompiled duplicate files* as "active bridges that stay" ("they are not bridges; they
are dead corpses left behind when the real code was copied into leaf crates"), and then executed in
fifteen commits the relocations Epic 3 had deferred to Milestone 9 — creating `paladin-herald`
inside an Epic whose §5 non-goals state "No new crates created. `paladin-herald`, `paladin-ml`, etc.
are not in scope". Its verification method is stated and reproducible, and the tree confirms every
target. **Three of its in-execution corrections matter more than the deletions**: `paladin_registry.rs`
was *not* a duplicate (the facade's 418-LOC impl was richer than battalion's 67-LOC `pub(crate)`
copy, so the richer one was consolidated *into* battalion); `sqlite_*_repository.rs` were *not*
redundant (they were the active default-build impl, resolved by making `paladin-storage`
non-optional); everything else genuinely was orphaned. **Recording this as the authoritative account
of Milestone 8 is HARD-02** — and the reason the earlier "9 crates" figure was wrong.

**Two Milestone 8 epics are complete despite their own records saying otherwise.** Epic 6 is filed
"Not verified; low priority" by the reconciliation and omitted from `deferred-items.md`, yet
`crates/paladin-content/src/services/` ships, `lib.rs` declares `pub mod services;`, and a
workspace-wide grep for `use_cases` returns zero. Epic 3 is filed "PUNTED" and is complete in
substance. Both of Milestone 8's three open checkboxes are contradicted by code — the same pattern
runs 1-2 found for Conclave and Sanctum.

**The security gates do not hold, and one of them has a deadline.** Four surfaces encode four
different RustSec exception sets: `rustsec-remediation-plan.md` formally risk-accepts **two**
advisories with owner Platform Security and **review/expiry target 2026-09-30**; `.cargo/audit.toml`
suppresses **five**; `deny.toml` suppresses **fifteen** while its own header claims to mirror
`audit.toml` and instructs "keep these two files in sync"; and `ci.yml` runs **two independent
`cargo audit` jobs** — a bare one at `:77` reading `audit.toml`'s five, and one at `:406` passing
the original two inline. Thirteen of `deny.toml`'s fifteen have no entry in the formal
risk-acceptance register; they carry inline one-line reasoning but no owner and no expiry, against
acceptance criteria that require both. Both `cargo audit` and `cargo deny` gate CI. **2026-09-30 is
the only date anywhere in the 153-document corpus**, and nothing in `.planning/` other than SEC-01
surfaces it.

**Four small verified defects sit alongside those gates.** `paladin-herald` has a `README.md` but no
`CHANGELOG.md`, against a criterion the Epic 4 completion summary records as Met (the crate was
created after Epic 4 closed). `Dockerfile.chef` enumerates nine crate manifests in its planner stage
and omits the tenth, so the cache-tightness FR-01 exists to deliver is not achieved for herald. The
`api-surface` baseline path is unchanged since run 3 — and is now **written into a run-4
requirement**, M8 Epic 7 FR-10, so DEBT-01 must fix the requirement as well as the tooling.
`paladin-ports` doctests remain disabled behind the same unwritten "Task 7.0".

**Two architecture questions are worth surfacing rather than assuming.** The extracted-crate
dependency rule is stated absolutely — "No extracted crate may depend on another extracted crate" —
and violated exactly once, by `paladin-content`'s optional `paladin-llm` edge, which the same PRD's
own complexity assessment anticipated without amending the rule (HARD-05). And `pdf = []` in
`paladin-content` gates no dependency while the facade's `content-processing` omits it entirely —
yet `.cargo/audit.toml` suppresses an advisory on the stated grounds that `pdf-extract` *is* in the
graph (HARD-06). The second blocks an honest reconciliation of the first set of advisories.

**The version trajectory is history, and must not be mistaken for state.** Milestone 7 Epic 4 cut
**`v0.1.0-rc.1`** at commit `a9530fc` on 2026-05-28 — all ten crates published at `0.1.0`, every
release gate PASS, a GO sign-off, docs.rs verification for all ten, and an external smoke project
compiling against `paladin-ai = "0.1.0"`. Its own PRD had targeted lockstep `0.2.0`. Milestone 8
targeted v0.2.0; its Epic 7, written 2026-06-06, targets "post-v0.5.1", so v0.3.0 through v0.5.1 all
shipped in between. The tree is at `0.6.0` on `release/v0.7.0` with latest tag `v0.5.1`. HARD-03
records the trajectory; REL-01 converges the three-way disagreement and must not converge on an rc.1
figure.

**The trustworthy remaining-work signal is the deferred registers, not checkbox arithmetic.**
`deferred-items.md` (D1-D5) and `deferred-features.md` (the `paladin user` CLI surface and the
TensorFlow adapter) are verified exact against the tree — D5's claim of 17
`println!`/`eprintln!`/`dbg!` occurrences across 6 files matches to the occurrence, and `src/core/`'s
six files, the three mis-layered manager services and both feature removals all check out. The
`deferred-features.md` TensorFlow entry carries the load-bearing constraint: any future ML adapter
must live in a dedicated `paladin-ml` **leaf crate**, never the facade — the surviving half of the
non-goal that `paladin-herald` overrode.

**The precedence order this project uses**, most authoritative first:
**shipped tree → `.planning/codebase/` map → `intel/code-verification.md` → PRD → DOC →
task-list checkbox.** Three ingest runs have now found checkbox state wrong in both directions, so
it sits last by evidence rather than by preference.

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
(36 docs) in run **1 of 5**, merged `.project/Milestone_2-Missing_features` +
`.project/Milestone_3-Completion` (45 docs) in run **2 of 5**, merged
`.project/Milestone_4-Refactor-Crates-Features` + `.project/Milestone_5-Workspace-Decomposition` +
`.project/Milestone_6-Architectural-Refinements` (32 docs) in run **3 of 5**, and merged
`.project/Milestone_7-Production-Hardening` +
`.project/Milestone_8-Facade-Cleanup-Shim-Resolution` (40 docs) in run **4 of 5** — 153 documents so
far (50 PRD, 103 DOC, 0 ADR, 0 SPEC), 434 requirements, 56 preserved variant entries across 28
groups, 0 blockers. Run 5 covers Milestones 9-12 plus Deferred-QA-CICD-Completion and
project-management. This document, `REQUIREMENTS.md` and `ROADMAP.md` are structured so that run
**appends** (a new milestone section, continuous phase numbering from Phase 12) rather than
restructures. Note: run-1 text in some files still says "run 1 of 14" — same run, renumbered
program.

**Run 4 also ingested a document that reaches into run 5's scope.**
`Milestones-8-11_Dependency-Graph.md` records M8 → M9 as a **hard** dependency ("M9 work should not
begin until M8 Epic 4 is complete"), M8 → M11 hard on path stability with M11 Epics 3-4 waiting on
M9 Epics 1-3, M9 → M11 hard on API stability, and M8 → M10 soft. Critical path
M8 → M9 → M11 Epics 3-5 = 11-17 sprints; M10 is entirely off it. It is preserved as a **historical
planning artefact** — M9 and M10 are recorded 100% complete and M11 92% — so run 5 should use its
dependency semantics and release-gate criteria, not its schedule.

**Nothing here is locked.** Zero ADR-typed and zero SPEC-typed documents exist across all 153
ingested documents, so every technical assertion in the ingested material sits at PRD or DOC
precedence and is auto-overridable. Run 2 produced eight documented supersessions of run-1
requirements; run 3 produced eleven more; run 4 produced eleven more still — including the corpus's
first case of **a document superseding another document by name**. Where a contested definition
matters, this project points at the codebase map or the shipped code rather than declaring a
winner.

**Constraint-shaped material is abundant and entirely untyped.** Run 3 is the most
constraint-dense set so far — three milestones of build-system contracts, dependency layering and
module boundaries — yet 0 SPEC-typed documents exist, so all of it lives as PRD acceptance
criteria. `intel/constraints.md` inventories what would become real constraints if the carriers
were re-tagged: the 25-port extraction inventory, every per-crate `[features]` table, the three
dependency allowlists, the workspace `Cargo.toml` template, the `config.yml` deserialization
contract, ~20 numeric build and coverage targets, and the `#[cfg]`-guard and import-migration
protocols. The two strongest re-tag candidates are the **dependency allowlists** and the
**`config.yml` deserialization contract**, precisely because shipped code already contradicts
both.

## Constraints

- **Tech stack**: Rust workspace (ten library crates + a `doc-examples` crate + the root
  `paladin-ai` facade), Tokio async throughout, Serde, SQLx, `thiserror` — pinned toolchain
  `rust-toolchain.toml` at 1.97.1. Not negotiable; the entire public surface is Rust traits.
  Shared dependency versions are pinned once in `[workspace.dependencies]` and referenced with
  `{ workspace = true }`.
- **Feature-gating is the compile-time contract**: `default = ["llm-openai"]`; per-provider
  `llm-openai` / `llm-anthropic` / `llm-deepseek` / `llm-all`; subsystem flags
  `content-processing`, `web-server` (axum), `notifications`, `vision`; storage flags `qdrant`
  (qdrant-client 1.14), `redis-queue`, `s3-storage`, `storage-mysql`, `openai-embeddings`; test
  flags `integration-tests`, `live-api-tests`; a `cli` flag that must never reach `default`; and a
  `full` convenience flag. `LlmPort` always compiles — only concrete adapters are gated.
  Unavailable adapters must fail at **compile time**, never at runtime, and `#[allow(dead_code)]`
  must not be used to paper over a `cfg` gate. Arsenal and its MCP transports are deliberately
  **not** feature-gated.
- **Dependency allowlists per crate are the enforcement mechanism for hexagonal purity** — and
  they are currently stale: `paladin-core` declares an "exhaustive" six and ships fourteen;
  `paladin-ports` declares seven and ships ten. The substantive invariant still holds (no LLM SDK,
  database driver, HTTP framework or object-storage client below the adapter layer). Reconciling
  the text with the tree is ARCH-03(b).
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
- **Licensing/repo**: `github.com/DF3NDR/paladin-dev-env`. **The licence has three recorded
  answers** — the shipped root `Cargo.toml` says `license = "MIT"`, the M7 Epic 4 PRD and overview
  say MIT, and a signed decision checklist (approver `DF3NDR`, 2026-05-28, 551-package inventory)
  says **`MIT OR Apache-2.0`** with MPL-2.0 explicitly accepted for unmodified use. `deny.toml`'s
  permissive-only allow-list plus eight per-crate MPL-2.0 exceptions already follows the checklist.
  SEC-02 settles it; do not infer.
- **Dependency advisories are a gated, governed surface — and the governance has drifted.** Both
  `cargo audit` and `cargo deny check` gate CI. The exception set is encoded four different ways
  (2 documented with owner and a **2026-09-30 expiry** / 5 in `.cargo/audit.toml` / 15 in
  `deny.toml` / 2 inline at `ci.yml:406`, plus a second bare `cargo audit` job at `ci.yml:77`), and
  `deny.toml`'s own stated sync invariant is violated. Every suppression must carry owner, expiry,
  affected scope and compensating control — thirteen currently do not. SEC-01.
- **Crate dependency direction between leaf crates is contested, not settled.** M7 Epic 1 §6.1
  states "No extracted crate may depend on another extracted crate or on the `paladin` facade"
  absolutely; `crates/paladin-content` declares an optional `paladin-llm` edge behind its `llm`
  feature. Until HARD-05 restates the rule, treat leaf-to-leaf edges as requiring a decision rather
  than as permitted or forbidden.
- **`actix-web` is banned**, not merely unused: `deny.toml:99-103` denies it with the reason
  "paladin-web standardizes on axum; no second web framework". Reintroducing a second HTTP framework
  is a deliberate, reviewed decision — the guardrail is live and enforced in CI.
- **Edition is mixed, and the documents disagree too**: verified 2026-07-30, the root package
  and every crate declare `edition = "2024"` **except** `crates/paladin-ports` and
  `crates/paladin-notifications`, which declare `"2021"`. Milestone 5 Epics 1-4 require 2021 and
  Epic 5 plus the milestone overview require 2024, so neither the code nor the record is
  self-consistent. Builds succeed today but the mix is brittle (`codebase/CONCERNS.md`). ARCH-03(a)
  records the answer; REL-02 applies it.

## Key Decisions

<!-- LOCKED DECISIONS (from ADR-typed documents). Empty by evidence, not by omission. -->

| Decision | Rationale | Outcome |
|----------|-----------|---------|
| *(none)* | Ingest runs 1-4 surfaced **0 ADR-typed and 0 SPEC-typed documents across 153 files** (50 PRD, 103 DOC). No source doc carried an ADR status field with a manifest ADR type, and none carried `locked: true`. Nothing is recorded here speculatively. | — Pending |

Everything asserted in the ingested PRDs and DOCs is **supersedable** — demonstrated, not
theoretical: run 2 produced eight documented supersessions of run-1 requirements, run 3 produced
eleven more including the entire monolith → workspace path migration, and run 4 produced eleven more
still, including the corpus's first document-supersedes-document notice (see *Superseded but
preserved* in `REQUIREMENTS.md`). The first real entries in this table are expected from Phase 1
(six ADRs, one per competing variant pair), Phase 5 (four recorded answers), Phase 7 (six more),
Phases 9-10 (the RustSec exception set, the licence posture, the leaf-crate dependency rule, the PDF
capability and the `cargo doc` bar), and any ADR-typed documents arriving in run 5.

**Six ADR candidates now exist, and none is entered here** — doing so would manufacture a locked
decision from a DOC-precedence assertion. The four added by run 4 come first because one of them has
an operational cost attached to leaving it untagged:

1. **`Milestone_5/Epic_1/decisions/battalion-result-upward-dependency-decision.md`** (run 3) — the
   only decision/options pair in all 263 documents, carrying `Status: Approved`,
   `Decision Date: 2026-05-13`, `Chosen Option: Option A`, a Rationale, a Rejected Options section
   and an implementation checklist. It settles where `PaladinResult`, `StopReason`, `TokenUsage`,
   `RegistryError` and `HandoffError` live, and shipped code implements it. It is manifest-typed
   DOC, so a PRD published two days later outranks it — and that PRD's rule would undo the fix.
   **This is the strongest candidate in the corpus and the one with real consequences if left
   unprotected.**
2. **`Epic_17.5/epic17-5.md`** (run 2) — the CLI belongs in `src/application/cli` because "CLI is
   an input adapter in the application layer, not infrastructure". Also already applied in code,
   also outranked by a PRD that says otherwise.
3. **`Milestone_7/Epic_4/rustsec-remediation-plan.md`** (run 4) — a formal **risk acceptance**:
   two advisories, **owner Platform Security (Milestone 7)**, **review/expiry target 2026-09-30**,
   with compensating controls and required exit evidence. **The only item in all 153 documents
   carrying an expiry date, and the only candidate where not promoting it has an ongoing
   operational cost** — nothing else in `.planning/` will surface that date. Its governing epic
   states the acceptance criteria it satisfies. SEC-01 acts on the drift; promotion is a separate,
   user-owned step.
4. **`Milestone_7/Epic_1/cost-benefit-assessment.md`** (run 4) — a go/defer record with an explicit
   **"Self-Approval (Task 1.6)"** block, a named approver and an approval date of 2026-05-25,
   scoring four candidate extractions on four criteria with measured evidence, issuing four Go
   decisions and fixing an extraction order. Its governing PRD calls it "the authoritative source of
   record for *why* a decision was made". Everything an ADR needs except the type tag.
5. **`Milestone_7/Epic_4/license-compatibility-decision-checklist.md`** (run 4) — a licensing policy
   with a named approver (`DF3NDR`), an approval date (2026-05-28), a 551-package inventory and an
   explicit accept-or-replace decision on MPL-2.0. Contradicted by the shipped manifests → SEC-02.
6. **`Milestone_8/facade-cleanup-RECONCILIATION-2026-06-04.md`** (run 4) — an explicit supersession
   notice carrying `Supersedes (corrects):` two named documents, which then resolved all six of its
   own listed open decisions **in execution rather than by a recorded decision**. The same
   "resolved by outcome" pattern run 3 flagged for the binary-target question → HARD-02.

Promoting any of these requires re-tagging the source document via `--manifest` and re-running
ingest.

---
*Last updated: 2026-07-30 after ingest run 4 of 5
(`.project/Milestone_7-Production-Hardening` + `.project/Milestone_8-Facade-Cleanup-Shim-Resolution`,
40 docs; cumulative 153 docs, 434 requirements, 56 variant entries across 28 groups, 0 locked
decisions, 0 blockers)*
