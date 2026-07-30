# Paladin

## What This Is

Paladin (root crate `paladin-ai`) is a Rust workspace that provides an enterprise AI
orchestration framework: autonomous agents (**Paladins**) coordinated through multi-agent
patterns (**Formation** sequential, **Phalanx** concurrent, **Campaign** DAG, **Chain of
Command** hierarchical), assembled behind hexagonal ports so that pluggable LLM providers
(OpenAI, Anthropic, DeepSeek, Mock), MCP tool servers, conversation memory, vector search,
state persistence, output formatters, a CLI and an HTTP API are all swappable adapters.

Its audience is Rust developers and teams embedding agent orchestration inside their own
services — not end users of a hosted product.

The product **already works**. It is a brownfield project at v0.7.0 with a 9-crate Cargo
workspace, 1,091 passing tests as last measured, 22 runnable examples, a multi-arch Docker
image and reference Kubernetes manifests. This planning setup exists to close out and verify
the first milestone, not to build the framework.

## Core Value

A Rust developer can compose and run multi-agent workflows against any supported LLM provider
through stable port abstractions — without their own domain code depending on a provider,
transport, or storage implementation.

## Success Metric (derived — not user-specified)

No developer-facing success metric was supplied. This one is derived strictly from the
measured evidence in the ingest set (`Epic_10/task6.0-validation-report.md`, dated
2026-01-27, and `unit-test-improvements/COVERAGE_ANALYSIS.md`):

**On a clean clone of the release branch, `cargo fmt --check`, `cargo clippy -- -D warnings`
and `cargo test --workspace` all pass with zero failures and zero warnings — the state last
measured at 1,091 tests / 0 failures / 0 clippy warnings — and `cargo llvm-cov` reports a
coverage figure that has moved up from the measured 60.88% unit / 67.79% integration baseline
toward whichever gate this milestone records.**

Why not something more ambitious: the evidence supports claims about build, lint, test and
measured-coverage state. It does not support performance, throughput, or onboarding-speed
claims. All five benchmark suites are currently **disabled**, so no verified performance number
exists; the "< 15 minutes to first working agent" figure in Epic 10's PRD was never measured.
Those remain documented *targets*, tracked in Phase 3 and Phase 4 respectively — not the
current metric.

## Requirements

### Validated

Shipped in the v0.7.0 workspace and confirmed by both the task-list checkbox state
(1,817 of 1,857 items, 98%) and the codebase map. Full per-requirement ledger:
`.planning/REQUIREMENTS.md` → *Milestone 1 as-shipped ledger*.

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

### Active

Current scope is **Milestone 1 close-out**: make the planning record match the shipped code,
resolve the contested type and gate definitions, close the residual functional gaps, and make
the quality numbers real. 25 requirements across 4 phases — see `.planning/ROADMAP.md`.

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

### Out of Scope

- **Milestones 2-12, Deferred-QA-CICD-Completion and project-management scope** — not yet
  ingested (runs 2-14 of 14). Code for several of these already exists (Grove, Council,
  Conclave, Maneuver DSL, Sanctum vector search, Axum web API, notifications, scheduler,
  content pipeline) but no requirements have been ingested for it, so it is deliberately
  unmapped here rather than guessed at.
- **Re-planning shipped Milestone-1 work** — 98% of the milestone's task items are complete.
  Anything already satisfied by code is recorded in the ledger, not re-planned as a phase.
- **Live-provider-API tests in the default test run** — Epic 6 task 7.0 was explicitly deferred
  with the rationale that mocked-HTTP unit tests give sufficient coverage. Deferral is recorded,
  not reversed. Tracked as v2.
- **Decomposing the three oversized service files** (2,757 / 2,294 / 1,840 lines) — real tech
  debt, but no Milestone-1 requirement demands it. Tracked as v2.
- **Clone/lock-contention performance work** — the 383 `.clone()` calls and the 9 orchestrator
  locks are flagged in `codebase/CONCERNS.md`, but optimizing before Phase 3 restores
  benchmarks would be guesswork. Tracked as v2.

## Context

**Where the code actually is.** The committed codebase map (`.planning/codebase/`, refreshed
2026-07-30) is authoritative on current state and is *ahead of* the Milestone-1 documents in
several places:

- MCP is implemented on the official `rmcp` 2.1.0 SDK with STDIO and **Streamable-HTTP**
  transports — the Milestone-1 PRD specified a hand-rolled client with an **SSE** transport.
- Capabilities exist with no ingested requirement: Sanctum vector search, Grove/Council/Conclave
  services, a Maneuver flow DSL, an Axum HTTP API with auth and OpenAPI, notifications, a
  scheduler, and a content pipeline.
- Herald is already wired into `PaladinExecutionService` (`with_herald`, used in the execution
  path) and `chain_of_command_service.rs` exists — both were listed as incomplete in the
  January task lists. **The task lists are a 2026-01 snapshot; the code is the arbiter.**
- Epic 9 declared "no REPL or interactive shell" a non-goal; an interactive REPL now ships in
  `src/application/cli/interactive/`. A documented Milestone-1 non-goal has already been
  superseded by later work — which is exactly why nothing in this file is treated as locked.

**Measured quality state** (Epic 10 Task 6.0, 2026-01-27): 1,091 tests passing / 0 failures
(706 unit, 385 integration, 133 doc); `cargo fmt --check` clean; `cargo clippy -- -D warnings`
at 0 warnings after fixing 102 across 48 files; unit coverage 60.88% against an 80% target;
integration coverage 67.79% against a 70% target; 2 medium transitive advisories; 22/22 examples
compiling; Docker image 112 MB built in 5m31s; **all benchmarks disabled**.

**Version state is incoherent right now.** Branch `release/v0.7.0`, workspace `Cargo.toml`
version `0.6.0`, latest tag `v0.5.1`. Three different answers to "what version is this".
Phase 4 resolves it.

**Ingest program.** This planning setup was bootstrapped from `.project/Milestone_1-MVP`
(36 docs: 11 PRD, 25 DOC, 0 ADR, 0 SPEC) — run **1 of 14**. Runs 2-14 merge Milestones 2-12,
Deferred-QA-CICD-Completion and project-management into the same intel files. This document,
`REQUIREMENTS.md` and `ROADMAP.md` are structured so those runs **append** (new milestone
sections, continuous phase numbering from Phase 5) rather than restructure.

**Nothing here is locked.** Zero ADR-typed and zero SPEC-typed documents exist in this run, so
every technical assertion in the ingested material sits at PRD or DOC precedence and is
auto-overridable by any ADR arriving later. Later milestones deliberately restructure earlier
ones. Where a contested definition matters, this project points at the codebase map or the
shipped code rather than declaring a winner.

## Constraints

- **Tech stack**: Rust workspace (9 member crates + facade), Tokio async throughout, Serde,
  SQLx, `thiserror` — pinned toolchain `rust-toolchain.toml` at 1.97.1. Not negotiable; the
  entire public surface is Rust traits.
- **Architecture**: Hexagonal, dependencies flow inward only (core → nothing; ports → core;
  adapters → core + ports). Bypassing a port to import an adapter directly is an anti-pattern
  the codebase map calls out by name.
- **Ubiquitous language**: Medieval military terms (Paladin, Battalion, Formation, Phalanx,
  Campaign, Chain of Command, Commander, Garrison, Arsenal, Armament, Citadel, Herald, Armory,
  Sanctum, Quest) are mandatory in code, docs and comments — they are the domain vocabulary,
  not decoration.
- **Error handling**: No `unwrap()`/`expect()`/`panic!` in library code; return `Result`. Layer-
  specific error enums converted at boundaries via `From`. `codebase/CONCERNS.md` lists existing
  violations to work down, not to imitate.
- **Methodology**: TDD (Red-Green-Refactor), rustdoc with compiling examples on all public
  items, `make clean-code` before committing, conventional commits.
- **Testing must work offline**: unit tests run with no external dependencies; anything needing
  Redis/MinIO/live APIs is feature-gated or `#[ignore]`d. Provider API keys come from
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
| *(none)* | Ingest run 1 of 14 surfaced **0 ADR-typed and 0 SPEC-typed documents**. No source doc carried an ADR status field, a Decision/Consequences structure, or `locked: true`. Nothing is recorded here speculatively. | — Pending |

Everything asserted in the ingested PRDs and DOCs is **supersedable**. The first real entries in
this table are expected from Phase 1 (six ADRs, one per competing variant pair) and from ADR-typed
documents arriving in ingest runs 2-14.

---
*Last updated: 2026-07-30 after ingest run 1 of 14 (`.project/Milestone_1-MVP`, 36 docs)*
