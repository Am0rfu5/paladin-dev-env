# Requirements: Paladin

**Defined:** 2026-07-30 (ingest run 1 of 14 — source set `.project/Milestone_1-MVP`, 36 docs)
**Core Value:** A Rust developer can compose and run multi-agent workflows against any supported
LLM provider through stable port abstractions — without their own domain code depending on a
provider, transport, or storage implementation.

## How to read this file

Paladin is **brownfield at v0.7.0**. 1,817 of 1,857 Milestone-1 task-list items are complete
(98%). So this file separates three different things that are all called "requirements":

| Section | What it holds |
|---|---|
| **v1 Requirements** | Forward scope for the current milestone (Milestone 1 close-out). 25 requirements, each mapped to exactly one phase. |
| **Competing variants** | 6 pairs / 12 entries preserved **unmerged** from conflicting PRDs. No winner is picked here. |
| **Milestone 1 as-shipped ledger** | All 115 requirement IDs extracted by ingest, with verified status. Not forward scope. |
| **v2 Requirements** | Acknowledged and deferred. Not in the current roadmap. |

**Two unrelated uses of "v1/v2" — do not confuse them:**

- `## v1 Requirements` / `## v2 Requirements` = **release scope** (current milestone vs deferred).
- An ID *suffix* like `REQ-temperature-range-v1` / `-v2` = **competing variant** of the same
  scope from two different source PRDs. Both are live; neither has been chosen.

**ID provenance.** `RECON-*`, `GAP-*`, `QUAL-*`, `REL-*` are new IDs for forward work; each cites
the ingested `REQ-*` IDs it derives from. `REQ-*` IDs are preserved verbatim from
`.planning/intel/requirements.md` so ingest runs 2-14 can merge against stable keys.

**Nothing is locked.** 0 ADR-typed and 0 SPEC-typed documents exist in this ingest run. Every
`REQ-*` acceptance criterion sits at PRD or DOC precedence and is supersedable — including by the
shipped code, which is the real arbiter wherever the two disagree.

---

## v1 Requirements

Scope: close out Milestone 1 so that the planning record, the code, and the quality numbers all
agree. Nothing here re-implements shipped work.

### Reconciliation (RECON)

- [ ] **RECON-01**: A cited status ledger exists that classifies every outstanding Milestone-1
      task item as *already satisfied by v0.7.0 code* (with `file:line`), *genuinely outstanding*,
      or *deferred with reason* — replacing the 2026-01 task-list snapshot as the source of truth.
      Must also record the three places where shipped code diverges from an ingested requirement
      (MCP Streamable-HTTP vs the specified SSE transport; Qdrant/Sanctum vector search vs the
      specified `sqlite-vss`; the shipped interactive REPL vs Epic 9 non-goal NG-7), the
      code-observed Battalion base module path (`battalion/mod.rs`), and the requirement-count
      discrepancy in the ingest bookkeeping (115 IDs enumerated vs 107 reported in SYNTHESIS.md).
      *Derives: all `intel/context.md` implementation-status topics; INGEST-CONFLICTS warnings 7-8.*
- [ ] **RECON-02**: `BattalionConfig` has exactly one authoritative definition, recorded as an ADR
      that names the chosen variant and cites shipped code as evidence. Must account for the fact
      that two distinct `BattalionConfig` structs currently exist in code
      (`battalion/mod.rs` and `citadel.rs`). *Resolves: REQ-battalion-config-v1 / -v2.*
- [ ] **RECON-03**: `BattalionResult` has exactly one authoritative definition, recorded as an ADR,
      that simultaneously satisfies its four producers (Formation, Phalanx, Campaign, Chain of
      Command) and its consumer (Herald — which needs a Battalion type field and aggregated token
      usage that neither source variant provides).
      *Resolves: REQ-battalion-result-v1 / -v2 + REQ-herald-battalion-result-fields.*
- [ ] **RECON-04**: The minimum Paladin count for Formation/Phalanx and the Commander's
      single-Paladin Auto routing rule have one consistent answer, recorded as an ADR. Shipped code
      currently contains the contradiction live: `formation.rs:109` rejects fewer than 2 Paladins
      while the Auto rule routes a single Paladin to Formation.
      *Resolves: REQ-formation-min-paladins-v1 / -v2.*
- [ ] **RECON-05**: Temperature validation has one recorded answer — provider-aware (range from
      `ProviderCapabilities`) or globally clamped — as an ADR.
      *Resolves: REQ-temperature-range-v1 / -v2.*
- [ ] **RECON-06**: The `Herald` trait has one recorded method set (fallible vs infallible,
      `format_paladin_stream` vs `format_stream_chunk` + `finalize_stream`, plus `name()`/
      `mime_type()`), as an ADR citing the shipped trait in
      `crates/paladin-core/src/platform/container/herald.rs`.
      *Resolves: REQ-herald-trait-v1 / -v2.*
- [ ] **RECON-07**: One authoritative coverage gate is recorded — a single number and a single
      scope — so that a later phase can objectively pass or fail against it. Measured baselines:
      60.88% unit, 67.79% integration. *Resolves: REQ-test-coverage-target-v1 / -v2.*
- [ ] **RECON-08**: Whether Epic 10 Task 7.0 (Final Documentation Review) is outstanding is
      answered, and the 102-vs-103 subtask discrepancy between the task list and the validation
      report is explained in the ledger. *Derives: INGEST-CONFLICTS warning 7.*

### Gap closure (GAP)

- [ ] **GAP-01**: A developer can run a Chain of Command battalion end-to-end and see the commander
      select specialists, handle a specialist failure via fallback, and synthesize a final answer —
      with unit and integration tests covering all four delegation strategies. Shipped code already
      contains `chain_of_command_service.rs`; this requirement is satisfied by verifying and
      finishing it, not rewriting it. *Derives: Epic 4 task 6.0; REQ-chain-of-command-construction,
      REQ-chain-of-command-execution, REQ-chain-of-command-aggregation.*
- [ ] **GAP-02**: Battalion integration and performance-validation tests exist and pass for all four
      patterns, including the Phalanx concurrency claims (≥ 10 concurrent Paladins, < 1 s
      orchestration overhead). *Derives: Epic 4 task 7.0; REQ-phalanx-concurrency,
      REQ-integration-testing.*
- [ ] **GAP-03**: Herald is on the Battalion execution path, not just the Paladin one: a Battalion
      result rendered through JSON, Markdown and Table Heralds shows Battalion name/ID/type,
      per-Paladin results in execution order, aggregated token usage, and partial results on error.
      *Derives: Epic 8 task 7.0 and 7.13; REQ-herald-battalion-result-fields (depends on RECON-03).*
- [ ] **GAP-04**: Commander execution produces a normalized result with strategy used, per-Paladin
      timings, success/failure counts and preserved `Vec<PaladinError>`, and writes telemetry
      metadata to `metadata_output_dir` when configured. *Derives: Epic 5 task 5.0 (5.10, 5.14);
      REQ-commander-result-normalization, REQ-commander-telemetry (depends on RECON-03).*
- [ ] **GAP-05**: `test_auto_selects_campaign_for_workflow_keywords`
      (`crates/paladin-battalion/src/commander.rs:1864`) passes, and Auto keyword routing is correct
      for all four keyword families. *Derives: Epic 5 task 3.11; REQ-commander-auto-selection.*
- [ ] **GAP-06**: Garrison final validation is closed — measured coverage recorded and every Epic 2
      PRD acceptance criterion reviewed against shipped code. *Derives: Epic 2 task 11.0 (11.5,
      11.6); REQ-garrison-testing.*
- [ ] **GAP-07**: The reconciled definitions from Phase 1 are applied in code: one `BattalionConfig`,
      one `BattalionResult`, agreed minimum-Paladin behaviour (a single-Paladin Commander in Auto
      mode executes instead of failing validation), the recorded temperature rule, and the recorded
      `Herald` trait signature — with the duplicate `BattalionConfig` in `citadel.rs` resolved.
      *Derives: RECON-02 … RECON-06.*

### Quality gates (QUAL)

- [ ] **QUAL-01**: `cargo llvm-cov` reports unit coverage at or above the gate recorded in
      RECON-07, up from the 60.88% baseline. *Derives: REQ-test-coverage-target-v1 / -v2,
      REQ-unit-test-gap-closure (unit-test-improvements tasks 2.0, 6.0).*
- [ ] **QUAL-02**: No first-party source file reports 0% coverage. Known offenders:
      `arsenal_execution_service.rs` (0/46 lines), `arsenal_registry_service.rs` (0/28),
      `redis.rs`, `minio.rs`, `user_controller.rs`, `sqlite_user_repository.rs`, `main.rs`;
      plus the sub-15% files `campaign_service.rs` (4.26%), `chain_of_command_service.rs` (13.41%),
      `mcp_protocol.rs` (15.83%), `deepseek_adapter.rs` (15.02%).
      *Derives: REQ-unit-test-gap-closure; `unit-test-improvements/COVERAGE_ANALYSIS.md`.*
- [ ] **QUAL-03**: Integration coverage of critical paths (Paladin execution, Battalion
      orchestration, tool invocation) is at or above 70%, up from the 67.79% baseline.
      *Derives: REQ-integration-testing.*
- [ ] **QUAL-04**: Error-path tests run instead of being skipped — the five `#[ignore]`d Commander
      tests (`commander.rs:2180+`) exercise real failure scenarios (retry count increments,
      partial-failure collection, timeout cascade), and MCP failure modes each have a passing test
      (expired/401 token, malformed response, handshake timeout, unknown tool, bad arguments).
      *Derives: `codebase/CONCERNS.md` test-coverage gaps; REQ-arsenal-resilience.*
- [ ] **QUAL-05**: `cargo bench` completes and a baseline document records throughput, P50/P95/P99
      latency, memory-per-Paladin and startup time for the Paladin execution loop, the Battalion
      patterns, Garrison operations and Arsenal invocation. All five suites are currently disabled
      (`paladin_benchmarks.rs`, `battalion_benchmarks.rs`, `garrison_benchmarks.rs`,
      `herald_benchmarks.rs`, `arsenal_benchmarks.rs`). *Derives: REQ-performance-benchmarking.*

### Release coherence (REL)

- [ ] **REL-01**: Version metadata agrees everywhere — workspace `Cargo.toml`, member crate
      versions, the git tag and the release notes tell one story. Current state: branch
      `release/v0.7.0`, `Cargo.toml` `0.6.0`, latest tag `v0.5.1`. *Derives: repo state.*
- [ ] **REL-02**: Every workspace crate declares one consistent, valid Rust edition, and
      `cargo build --workspace` succeeds under it. Ten crates currently declare `edition = "2024"`
      while others declare `"2021"`. *Derives: `codebase/CONCERNS.md` tech debt.*
- [ ] **REL-03**: `cargo audit` and `cargo deny` report no high/critical advisories, and every
      ignored advisory in `deny.toml` carries a written rationale plus a migration or review note.
      Current: 2 medium transitive advisories (RUSTSEC-2023-0071 rsa, RUSTSEC-2025-0111 tokio-tar),
      3 unfixed feature-gated advisories (lopdf, quick-xml ×2), 10 unmaintained-crate ignores, and
      a dual `reqwest` 0.12/0.13 exposure. *Derives: REQ-epic10-quality-gates; CONCERNS.md.*
- [ ] **REL-04**: Documentation final review is complete per the RECON-08 answer, and a developer
      following QUICKSTART on a clean machine reaches a working agent with the elapsed time
      recorded against the documented < 15-minute target. *Derives: REQ-user-documentation,
      REQ-api-documentation, REQ-architecture-documentation, REQ-operations-documentation,
      REQ-contribution-documentation.*
- [ ] **REL-05**: The full gate suite passes in CI on the release branch: `cargo fmt --check`,
      `cargo clippy -- -D warnings`, `cargo test --workspace`, doc tests, all 22 examples,
      multi-arch Docker build within the < 500 MB / < 5 min budget, and the kind-based Kubernetes
      smoke test within the < 30 s pod-startup budget.
      *Derives: REQ-epic10-quality-gates, REQ-deployment-artifacts.*

---

## Competing variants (preserved unmerged)

Six pairs, twelve entries, carried verbatim in scope from `.planning/intel/requirements.md`.
**No winner is selected here.** Where a variant matters, the pointer below is to the codebase map
or the shipped code — the real arbiter — not to a resolution. Recording the answers is
RECON-02 … RECON-07 (Phase 1).

### Pair 1 — project-wide test coverage gate

- **REQ-test-coverage-target-v1** — nine Epic PRDs (Epics 1-8, 10): unit coverage ≥ 80%,
  integration ≥ 70%, measured via cargo-llvm-cov.
- **REQ-test-coverage-target-v2** — `unit-test-improvements/prd-improve-unit-test-coverage.md`:
  overall coverage MUST exceed 85%; functions under 50% MUST reach 80%; stated baseline 67.79%.
- Status: **unresolved.** Measured actual is below both. See `codebase/TESTING.md` (states ≥ 80%
  unit / ≥ 70% integration as the repo's own convention) and
  `Epic_10/task6.0-validation-report.md` for the measured 60.88% / 67.79%.

### Pair 2 — valid temperature range

- **REQ-temperature-range-v1** — Epic 1 FR-2.3 / US-2: builder MUST validate `[0.0, 1.0]` and
  reject values above 1.0.
- **REQ-temperature-range-v2** — Epic 6 REQ-5: DeepSeek adapter MUST support temperature 0.0-2.0.
- Status: **unresolved.** A build-time `[0.0, 1.0]` clamp makes the DeepSeek range unreachable
  through the normal Paladin path. Shipped builder surface (including an `auto_temperature`
  feature that post-dates both PRDs) is in
  `src/application/services/paladin/paladin_builder.rs`; see `codebase/ARCHITECTURE.md`.

### Pair 3 — `BattalionConfig` field set

- **REQ-battalion-config-v1** — Epic 4 FR-4.1: `name`, `description`, `timeout_seconds`,
  `retry_policy` (struct: max_attempts / base_delay / max_delay / exponential_backoff / jitter),
  `error_strategy`, `metadata_output_dir`.
- **REQ-battalion-config-v2** — Epic 5 FR-7: `name: String`, `timeout_seconds: u64`,
  `retry_attempts: u32`, `error_strategy: ErrorStrategy`, `enable_checkpointing: bool`,
  `metadata_output_dir: Option<PathBuf>`; defaults for all fields.
- Status: **unresolved, and the shipped code adds a third shape.** Two distinct
  `BattalionConfig` structs exist: `crates/paladin-core/src/platform/container/battalion/mod.rs:37`
  and `crates/paladin-core/src/platform/container/citadel.rs:280`
  (`max_concurrency` / `timeout_seconds` / `continue_on_error`). See `codebase/STRUCTURE.md`.

### Pair 4 — `BattalionResult` field set

- **REQ-battalion-result-v1** — Epic 4 FR-4.2: `battalion_id`, `battalion_name`, timestamps,
  `final_output`, `paladin_results`, `status`; all intermediate results; per-Paladin and overall
  timing.
- **REQ-battalion-result-v2** — Epic 5 FR-5: `battalion_id: Uuid`, `strategy_used`,
  `paladin_results`, `final_output`, `execution_time_ms`, `status`, `metadata`
  (strategy_selection_reasoning, strategy_selection_time_ms, per_paladin_times,
  paladin_success_count, paladin_failure_count, timestamp), plus `errors: Vec<PaladinError>`.
- Third consumer, unmerged: **REQ-herald-battalion-result-fields** (Epic 8 FR-7) additionally
  requires a Battalion **type** field and **aggregated token usage** — which neither producer
  variant defines.
- Status: **unresolved.** See `codebase/ARCHITECTURE.md` (Battalion services) and
  `crates/paladin-battalion/src/`.

### Pair 5 — minimum Paladin count for Formation

- **REQ-formation-min-paladins-v1** — Epic 4 FR-4.5 / FR-4.8: Formation MUST validate ≥ 2
  Paladins; Phalanx accepts ≥ 2; Majority aggregation requires ≥ 3.
- **REQ-formation-min-paladins-v2** — Epic 5 FR-1 / FR-3: Commander validates only that ≥ 1
  Paladin is provided, and Auto rule 1 routes a single Paladin to Formation as the trivial case.
- Status: **unresolved and live in shipped code** —
  `crates/paladin-core/src/platform/container/battalion/formation.rs:109` errors with
  "Formation requires at least 2 Paladins", so the documented single-Paladin Auto happy path
  fails validation at runtime.

### Pair 6 — `Herald` trait signature

- **REQ-herald-trait-v1** — Epic 8 FR-1: infallible `-> String` returns;
  `format_paladin_result`, `format_battalion_result`, `format_paladin_stream -> Option<String>`,
  `format_error`; `Send + Sync`.
- **REQ-herald-trait-v2** — same PRD, section 6.2: fallible `-> Result<String, HeraldError>`;
  `format_stream_chunk`, added `finalize_stream(&ExecutionMetadata)`, `name()`, `mime_type()`.
- Status: **unresolved on paper.** The shipped trait at
  `crates/paladin-core/src/platform/container/herald.rs:49` documents a `HeraldError` return —
  i.e. the code leans fallible — and FR-10's graceful-degradation requirement cannot be expressed
  by the infallible form. Recording that as the answer is RECON-06; it is not asserted here.

---

## Milestone 1 as-shipped ledger

All 115 requirement IDs extracted by ingest run 1, with verified status. **Not forward scope** —
listed so nothing is lost and so runs 2-14 merge against stable keys. Acceptance criteria are not
repeated; they live in `.planning/intel/requirements.md`.

Status key: `Shipped` = satisfied by v0.7.0 code and a complete task list · `Verify` = code exists,
completion asserted only by the 2026-01 task list, confirmation is part of RECON-01 ·
`Partial → X` = residual work tracked by forward requirement X · `Variant` = see competing variants
· `Deferred → v2` · `Code diverges` = shipped implementation differs from the ingested requirement.

### Epic 1 — Paladin Domain Foundation (182/182 items, 100%)

| ID | Status |
|---|---|
| REQ-paladin-entity | Shipped — `crates/paladin-core/src/platform/container/paladin.rs` |
| REQ-paladin-builder | Shipped — `src/application/services/paladin/paladin_builder.rs` |
| REQ-paladin-config | Shipped |
| REQ-paladin-port | Shipped (streaming impl landed via Epic 6) |
| REQ-paladin-execution-service | Shipped — `paladin_execution_service.rs`, 94% line coverage |
| REQ-paladin-error-handling | Shipped |
| REQ-paladin-observability | Shipped — note code uses `log`/`env_logger` alongside `tracing-subscriber`; the PRD specified `tracing` (RECON-01 records this) |
| REQ-paladin-testing-infra | Shipped — `tests/helpers/mock_llm_adapter.rs`, `paladin-llm/src/mock.rs` |

### Epic 2 — Garrison Memory System (161/165 items, 98%)

| ID | Status |
|---|---|
| REQ-garrison-entry | Shipped |
| REQ-garrison-windowing | Shipped (`test_large_conversation_performance` deferred → v2) |
| REQ-garrison-port | Shipped — 100% port coverage |
| REQ-garrison-longterm-port | Code diverges — semantic retrieval ships as **Sanctum** (Qdrant + in-memory), not as a `sqlite-vss` extension of Garrison |
| REQ-garrison-in-memory | Shipped — 96.49% |
| REQ-garrison-sqlite | Code diverges — SQLite adapter shipped; `sqlite-vss` vector search superseded by Sanctum |
| REQ-garrison-paladin-integration | Shipped |
| REQ-garrison-config | Shipped |
| REQ-garrison-errors | Shipped |
| REQ-garrison-testing | Partial → GAP-06, QUAL-01 (task 11.5/11.6 open) |

### Epic 3 — Arsenal Tool System (220/223 items, 99%)

| ID | Status |
|---|---|
| REQ-arsenal-domain-types | Shipped — entity at 100% coverage |
| REQ-arsenal-port | Shipped (services untested → QUAL-02) |
| REQ-mcp-protocol | Code diverges — built on the official `rmcp` 2.1.0 SDK rather than a hand-rolled JSON-RPC client |
| REQ-mcp-stdio-transport | Shipped — `transport-child-process` |
| REQ-mcp-sse-transport | Code diverges — shipped as **Streamable-HTTP**, not SSE |
| REQ-arsenal-builder-integration | Shipped |
| REQ-arsenal-resource-controls | Shipped — 96.94% |
| REQ-arsenal-resilience | Partial → QUAL-04 (failure paths untested) |
| REQ-arsenal-context-injection | Shipped — 90.31% |

### Epic 4 — Battalion Orchestration (233/235 items; tasks 6.0 and 7.0 open)

| ID | Status |
|---|---|
| REQ-battalion-config-v1 | Variant (pair 3) |
| REQ-battalion-result-v1 | Variant (pair 4) |
| REQ-battalion-error-strategy | Shipped — `error_aggregation.rs` 99.60% |
| REQ-battalion-retry-policy | Shipped — `retry.rs` 100% |
| REQ-formation-min-paladins-v1 | Variant (pair 5) |
| REQ-formation-construction | Shipped |
| REQ-formation-execution | Shipped — `formation_service.rs` 88.14% |
| REQ-formation-output | Shipped |
| REQ-phalanx-construction | Shipped |
| REQ-phalanx-concurrency | Partial → GAP-02 (concurrency claims unvalidated; benchmarks disabled) |
| REQ-phalanx-aggregation | Shipped — `phalanx_service.rs` 87.93% |
| REQ-campaign-graph | Shipped (petgraph) |
| REQ-campaign-edge-conditions | Shipped |
| REQ-campaign-execution | Verify → QUAL-02 (`campaign_service.rs` at 4.26% coverage) |
| REQ-chain-of-command-construction | Verify → GAP-01 |
| REQ-chain-of-command-execution | Partial → GAP-01 (task 6.0 open; service exists at 13.41% coverage) |
| REQ-chain-of-command-aggregation | Partial → GAP-01 |
| REQ-battalion-status | Shipped |
| REQ-battalion-logging | Shipped |
| REQ-battalion-cancellation | Verify → GAP-02 |

### Epic 5 — Commander Strategy Router (150/154 items, 97%)

| ID | Status |
|---|---|
| REQ-battalion-config-v2 | Variant (pair 3) |
| REQ-battalion-result-v2 | Variant (pair 4) |
| REQ-formation-min-paladins-v2 | Variant (pair 5) |
| REQ-commander-construction | Shipped — `commander.rs` 81.79% |
| REQ-commander-strategy-types | Shipped |
| REQ-commander-auto-selection | Partial → GAP-05 (one failing keyword test) |
| REQ-commander-execute | Shipped |
| REQ-commander-result-normalization | Partial → GAP-04 (task 5.0 open) |
| REQ-commander-error-strategy | Partial → QUAL-04 (5 error-path tests `#[ignore]`d) |
| REQ-commander-config-passthrough | Shipped |
| REQ-commander-service-composition | Shipped |
| REQ-commander-telemetry | Partial → GAP-04 (metadata export to file deferred) |
| REQ-commander-validation | Shipped |

### Epic 6 — Provider Expansion (180/199 items, 90%; task 7.0 deferred)

| ID | Status |
|---|---|
| REQ-llm-port-interface | Shipped — capability struct included |
| REQ-deepseek-adapter | Shipped (15.02% coverage → QUAL-02) |
| REQ-anthropic-adapter | Shipped (28.19% coverage → QUAL-02) |
| REQ-provider-configuration | Shipped — `provider_factory.rs`, env-var keys |
| REQ-provider-backward-compat | Shipped |
| REQ-provider-error-mapping | Shipped |
| REQ-provider-testing | Partial — mocked-HTTP unit tests shipped; live-API suite Deferred → v2 |
| REQ-provider-documentation | Shipped |
| REQ-temperature-range-v1 / -v2 | Variant (pair 2) |

### Epic 7 — Citadel State Persistence (169/169 items, 100%)

| ID | Status |
|---|---|
| REQ-citadel-paladin-state-serialization | Shipped — `citadel.rs` 99.32% |
| REQ-citadel-autosave | Shipped |
| REQ-citadel-paladin-restore | Shipped (fallible `restore_from` per PRD) |
| REQ-citadel-battalion-state-serialization | Shipped |
| REQ-citadel-battalion-checkpoint-restore | Shipped — example `battalion_checkpoint_recovery` |
| REQ-citadel-port | Shipped — port 100% |
| REQ-citadel-errors | Shipped — 98.06% |
| REQ-citadel-builder-integration | Shipped |
| REQ-citadel-state-directory | Shipped |
| REQ-citadel-logging-docs | Shipped |

### Epic 8 — Herald Output Formatting (139/141 items; task 7.0 open)

| ID | Status |
|---|---|
| REQ-herald-trait-v1 / -v2 | Variant (pair 6) |
| REQ-herald-builtin-formatters | Shipped — `crates/paladin-herald/src/{json,markdown,table}_herald.rs` |
| REQ-herald-streaming | Shipped |
| REQ-herald-configuration | Shipped — `src/config/herald.rs` |
| REQ-herald-default-and-override | Shipped |
| REQ-herald-paladin-result-fields | Verify — Herald is wired into `paladin_execution_service.rs` (`with_herald`, used at line 428), which the task list listed as open |
| REQ-herald-battalion-result-fields | Partial → GAP-03 (depends on RECON-03) |
| REQ-herald-registry | Shipped — `src/application/services/herald/` |
| REQ-herald-builder-integration | Shipped |
| REQ-herald-error-handling | Shipped |

### Epic 9 — Armory CLI Tools (238/241 items, 99%)

| ID | Status |
|---|---|
| REQ-cli-structure | Shipped — clap v4 derive, `src/bin/paladin-cli.rs` |
| REQ-cli-agent-run | Shipped |
| REQ-cli-agent-new | Shipped — `src/application/cli/templates/` |
| REQ-cli-battalion-run | Shipped |
| REQ-cli-battalion-new | Shipped |
| REQ-cli-arsenal-list | Shipped |
| REQ-cli-arsenal-test | Shipped |
| REQ-cli-config-format | Shipped — YAML only |
| REQ-cli-env-vars | Shipped |
| REQ-cli-validation-errors | Shipped |
| REQ-cli-output-formatting | Shipped |
| REQ-cli-interactive-mode | Shipped — note the shipped interactive REPL exceeds Epic 9 non-goal NG-7 (RECON-01 records this) |
| *(CLI e2e tests 13.4-13.6)* | Deferred → v2 (needs CLI mock-provider support) |

### Epic 10 — Validation & Documentation (103/103 items; Task 7.0 disputed)

| ID | Status |
|---|---|
| REQ-integration-testing | Partial → GAP-02, QUAL-03 (67.79% vs 70% gate) |
| REQ-performance-benchmarking | Partial → QUAL-05 (all five benchmark suites disabled) |
| REQ-api-documentation | Verify → REL-04 |
| REQ-user-documentation | Partial → REL-04 (< 15 min quickstart target never measured) |
| REQ-architecture-documentation | Shipped — 24 docs, ~5,000 lines |
| REQ-deployment-artifacts | Shipped — 112 MB image, k8s manifests, release + integration workflows → REL-05 re-verifies |
| REQ-operations-documentation | Shipped |
| REQ-contribution-documentation | Shipped |
| REQ-epic10-quality-gates | Partial → QUAL-01, QUAL-03, REL-03, REL-05 |

### unit-test-improvements workstream (42/44 items; tasks 2.0 and 6.0 open)

| ID | Status |
|---|---|
| REQ-test-coverage-target-v1 / -v2 | Variant (pair 1) |
| REQ-unit-test-gap-closure | Partial → QUAL-01, QUAL-02 |

---

## v2 Requirements

Acknowledged, deferred, not in the current roadmap. Some of these may acquire a real requirement
when ingest runs 2-14 land Milestones 2-12 and Deferred-QA-CICD-Completion.

### Testing

- **Live-provider-API integration tests** — Epic 6 task 7.0 (18 subtasks), explicitly deferred with
  the rationale that mocked-HTTP unit tests suffice. Keep `#[ignore]`d/feature-gated.
- **CLI end-to-end tests** — Epic 9 tasks 13.4-13.6, blocked on CLI mock-provider support.
- **Garrison large-conversation performance test** — Epic 2 task 9.14 (1,000 entries).
- **Bearer-token redaction enforcement test** — prove `BearerToken` cannot be logged
  (`codebase/CONCERNS.md`).

### Tech debt (from `codebase/CONCERNS.md`, no Milestone-1 requirement)

- Decompose the three oversized service files: `paladin_execution_service.rs` (2,757 lines),
  `paladin_builder.rs` (2,294), `orchestration/mod.rs` (1,840).
- Reduce the 383 `.clone()` calls and the 9-lock orchestrator contention — needs the Phase 3
  benchmark baselines first.
- Replace `structopt` with clap v4 and `dotenv` with `dotenvy`; upgrade `utoipa` off `paste`;
  converge the dual `reqwest` 0.12/0.13 dependency.
- Orchestrator state durability: workflow checkpointing, resume-on-startup, enforced queue
  persistence in production mode.
- Environment isolation for `system_log_integration_test.rs`.

### Unimplemented code paths (requirement pending later ingest runs)

- Notification adapter wiring (`service_runner.rs:534`) — notifications register but never deliver.
- Media content handlers — video/audio/image stubs in `file_content_fetcher.rs:105-115`.
- Trigger payload conditions (JSONPath) and cooldown checks — `trigger.rs:216, 261`.
- Grove service hardcoded `model: "gpt-4"` — `grove_service.rs:537`.
- MCP config validation coupling — schema accepts server types the client cannot construct.

### Awaiting ingest (runs 2-14 of 14)

Milestones 2-12, Deferred-QA-CICD-Completion and project-management. Shipped code with **no
ingested requirement yet**: Sanctum vector search, Grove / Council / Conclave services, the
Maneuver flow DSL, the Axum HTTP API (auth, rate limiting, OpenAPI, SSE streaming), notifications,
the scheduler, and the content ingestion pipeline.

---

## Out of Scope

| Feature | Reason |
|---|---|
| Re-implementing shipped Milestone-1 work | 98% of task items complete; the ledger records it, the roadmap does not re-plan it |
| Picking a winner for the 6 competing variants inside this document | Deliberate: the shipped code is the arbiter and the decision belongs in an ADR (RECON-02 … RECON-07), not in an ingest artifact |
| Synthesizing locked decisions from PRD/DOC assertions | 0 ADR-typed and 0 SPEC-typed docs exist in run 1; asserting locks would fabricate authority |
| Live LLM API calls in the default test run | Cost, flakiness, and secret handling; feature-gated only |
| JSON or TOML CLI configuration | Epic 9 NG-4 — YAML only |
| Encrypted config files, keychain/secret-manager integration | Epic 9 NG-2, NG-10 — env vars only |
| Non-Rust client SDKs | The product *is* a Rust library surface |
| Milestone 2-12 feature work | Awaiting ingest runs 2-14; planning it now would guess at requirements that exist on disk |

---

## Traceability

Forward (v1) requirements only. Shipped requirements are tracked in the ledger above.

| Requirement | Phase | Status |
|-------------|-------|--------|
| RECON-01 | Phase 1 | Pending |
| RECON-02 | Phase 1 | Pending |
| RECON-03 | Phase 1 | Pending |
| RECON-04 | Phase 1 | Pending |
| RECON-05 | Phase 1 | Pending |
| RECON-06 | Phase 1 | Pending |
| RECON-07 | Phase 1 | Pending |
| RECON-08 | Phase 1 | Pending |
| GAP-01 | Phase 2 | Pending |
| GAP-02 | Phase 2 | Pending |
| GAP-03 | Phase 2 | Pending |
| GAP-04 | Phase 2 | Pending |
| GAP-05 | Phase 2 | Pending |
| GAP-06 | Phase 2 | Pending |
| GAP-07 | Phase 2 | Pending |
| QUAL-01 | Phase 3 | Pending |
| QUAL-02 | Phase 3 | Pending |
| QUAL-03 | Phase 3 | Pending |
| QUAL-04 | Phase 3 | Pending |
| QUAL-05 | Phase 3 | Pending |
| REL-01 | Phase 4 | Pending |
| REL-02 | Phase 4 | Pending |
| REL-03 | Phase 4 | Pending |
| REL-04 | Phase 4 | Pending |
| REL-05 | Phase 4 | Pending |

**Coverage:**
- v1 requirements: 25 total
- Mapped to phases: 25
- Unmapped: 0 ✓
- Duplicated across phases: 0 ✓

**Ledger coverage:**
- Requirement IDs enumerated from `intel/requirements.md`: 115 (103 non-variant + 12 variant)
- Recorded in the ledger: 115 ✓
- Bookkeeping note: `intel/SYNTHESIS.md` reports 107 requirements and its per-PRD table sums to a
  third figure. The enumerated count is authoritative here; reconciling the ingest arithmetic is
  part of RECON-01.

---
*Requirements defined: 2026-07-30*
*Last updated: 2026-07-30 after ingest run 1 of 14 (`.project/Milestone_1-MVP`)*
