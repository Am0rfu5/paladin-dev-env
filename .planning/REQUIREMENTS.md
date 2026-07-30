# Requirements: Paladin

**Defined:** 2026-07-30 (ingest run 1 of 5 — source set `.project/Milestone_1-MVP`, 36 docs)
**Last merge:** 2026-07-30 (ingest run 2 of 5 — `.project/Milestone_2-Missing_features` +
`.project/Milestone_3-Completion`, 45 docs)
**Core Value:** A Rust developer can compose and run multi-agent workflows against any supported
LLM provider through stable port abstractions — without their own domain code depending on a
provider, transport, or storage implementation.

## How to read this file

Paladin is **brownfield at v0.7.0**. Across the 81 documents ingested so far, 7,511 of 8,053
task-list items are checked (93%) — and the shipped tree is *ahead of* even that figure in
several places. So this file separates four different things that are all called "requirements":

| Section | What it holds |
|---|---|
| **v1 Requirements** | Forward scope for the current milestone. Milestone 1 close-out: 25 requirements (Phases 1-4). Milestone 2-3 close-out: 9 requirements (Phases 5-6). Each mapped to exactly one phase. |
| **Competing variants** | 16 variant groups / 30 entries preserved **unmerged** from conflicting PRDs across runs 1-2. No winner is picked here. |
| **Milestone 1 as-shipped ledger** | All 115 run-1 requirement IDs, with status. Not forward scope. |
| **Milestone 2-3 as-shipped ledger** | All 118 run-2 requirement IDs, with status. Not forward scope. |
| **v2 Requirements** | Acknowledged and deferred. Not in the current roadmap. |

**Two unrelated uses of "v1/v2" — do not confuse them:**

- `## v1 Requirements` / `## v2 Requirements` = **release scope** (current milestone vs deferred).
- An ID *suffix* like `REQ-temperature-range-v1` / `-v2` / `-v3` = **competing variant** of the
  same scope from two or three different source PRDs. All are live; none has been chosen.

**ID provenance.** `RECON-*`, `GAP-*`, `QUAL-*`, `REL-*` are Milestone-1 close-out IDs;
`VERIFY-*` and `CLOSE-*` are Milestone 2-3 close-out IDs. Each cites the ingested `REQ-*` IDs it
derives from. `REQ-*` IDs are preserved verbatim from `.planning/intel/requirements.md` so ingest
runs 3-5 can merge against stable keys.

**Ledger verdicts are component-level file evidence, not per-criterion audits.** A `Shipped`
verdict means the named artefact exists in the tree at the cited path. Confirming each PRD
acceptance criterion against that artefact is forward work (RECON-01 / VERIFY-01), not something
this file claims to have done.

**Open checkbox counts are not a backlog.** `intel/task-completion-state.md` records 542 open
items; `intel/code-verification.md` proves the two largest concentrations (Conclave 129,
Sanctum 111) are **shipped**. Only the blocks that `code-verification.md` explicitly lists as
"Not yet verified" are carried here, and they are labelled *Unverified candidate* — never
converted into requirements.

**Nothing is locked.** 0 ADR-typed and 0 SPEC-typed documents exist across all 81 ingested docs.
Every `REQ-*` acceptance criterion sits at PRD or DOC precedence and is supersedable — including
by the shipped code, which is the real arbiter wherever the two disagree. Precedence order for
this project: **shipped tree > `.planning/codebase/` map > `intel/code-verification.md` > PRD > DOC
> task-list checkbox.**

---

## v1 Requirements — Milestone 1 close-out (Phases 1-4)

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
      (`battalion/mod.rs` and `citadel.rs`). *Resolves: REQ-battalion-config-v1 / -v2 /
      REQ-commander-config-metadata-dir-v3 (see variant group 3).*
- [ ] **RECON-03**: `BattalionResult` has exactly one authoritative definition, recorded as an ADR,
      that simultaneously satisfies its four producers (Formation, Phalanx, Campaign, Chain of
      Command) and its consumer (Herald — which needs a Battalion type field and aggregated token
      usage that neither source variant provides).
      *Resolves: REQ-battalion-result-v1 / -v2 + REQ-herald-battalion-result-fields; must account
      for the later position in REQ-battalion-metadata-extension (run 2).*
- [ ] **RECON-04**: The minimum Paladin count for Formation/Phalanx and the Commander's
      single-Paladin Auto routing rule have one consistent answer, recorded as an ADR. Shipped code
      currently contains the contradiction live: `formation.rs:109` rejects fewer than 2 Paladins
      while the Auto rule routes a single Paladin to Formation.
      *Resolves: REQ-formation-min-paladins-v1 / -v2.*
- [ ] **RECON-05**: Temperature validation has one recorded answer — provider-aware (range from
      `ProviderCapabilities`) or globally clamped — as an ADR. Must account for the run-2 dynamic
      temperature bands (Factual 0.1-0.3 … Creative 0.7-1.0) and the Epic 14 DOC's 0.1-1.0 bound.
      *Resolves: REQ-temperature-range-v1 / -v2; interacts with REQ-dynamic-temperature.*
- [ ] **RECON-06**: The `Herald` trait has one recorded method set (fallible vs infallible,
      `format_paladin_stream` vs `format_stream_chunk` + `finalize_stream`, plus `name()`/
      `mime_type()`), as an ADR citing the shipped trait in
      `crates/paladin-core/src/platform/container/herald.rs`.
      *Resolves: REQ-herald-trait-v1 / -v2; must account for REQ-herald-type-consolidation (run 2).*
- [ ] **RECON-07**: One authoritative coverage gate is recorded — a single number and a single
      scope — so that a later phase can objectively pass or fail against it. Measured baselines:
      60.88% unit, 67.79% integration. *Resolves: REQ-test-coverage-target-v1 / -v2; extended to
      the run-2 positions by VERIFY-05.*
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
      REQ-commander-result-normalization, REQ-commander-telemetry (depends on RECON-03). Note run 2
      adds REQ-commander-metadata-export, which specifies the JSON schema and file naming.*
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
- [ ] **QUAL-04**: Error-path tests run instead of being skipped — the `#[ignore]`d Commander
      tests exercise real failure scenarios (retry count increments, partial-failure collection,
      timeout cascade), and MCP failure modes each have a passing test (expired/401 token,
      malformed response, handshake timeout, unknown tool, bad arguments). Four `#[ignore]`
      attributes remain in `crates/paladin-battalion/src/commander.rs` as of v0.7.0.
      *Derives: `codebase/CONCERNS.md` test-coverage gaps; REQ-arsenal-resilience;
      REQ-commander-test-hardening (run 2) specifies the six tests by name.*
- [ ] **QUAL-05**: `cargo bench` completes and a baseline document records throughput, P50/P95/P99
      latency, memory-per-Paladin and startup time for the Paladin execution loop, the Battalion
      patterns, Garrison operations and Arsenal invocation. Benchmarks were relocated into
      per-crate `benches/` directories during the workspace decomposition — the shipped set is
      `crates/paladin-battalion/benches/battalion_benchmarks.rs`,
      `crates/paladin-memory/benches/{garrison,sanctum}_benchmarks.rs`,
      `crates/paladin-llm/benches/llm_serialization_benchmarks.rs` and
      `benches/config_benchmarks.rs`; the Milestone-1 `paladin_benchmarks.rs`,
      `herald_benchmarks.rs` and `arsenal_benchmarks.rs` are not in the tree.
      *Derives: REQ-performance-benchmarking; REQ-battalion-benchmark-repair (run 2).*

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

## v1 Requirements — Milestone 2-3 close-out (Phases 5-6)

Added by ingest run 2. Scope is deliberately small: **Milestones 2 and 3 shipped.** Every
capability those 118 requirements describe — Sanctum, RAG, Sentinel vision, autonomous planning
and handoffs, Conclave, Council, Grove, the Maneuver Flow DSL, the enhanced CLI, Herald
consolidation, the Paladin registry, the scheduler port — has a corresponding artefact in the
v0.7.0 tree. What is *not* recorded is which PRD acceptance criteria those artefacts actually
satisfy, which competing surface each one implements, and what the three unverified
open-checkbox blocks contain. That is the forward work below, plus exactly one verified defect.

### Ground truth (VERIFY)

- [ ] **VERIFY-01**: The *Milestone 2-3 as-shipped ledger* below is upgraded from component-level
      file evidence to per-criterion verdicts with `file:line` citations, for all 118 run-2
      requirement IDs. Must record, per requirement, whether the shipped artefact satisfies the
      PRD acceptance criteria, diverges from them, or only partially covers them. Must also record
      the two systematic path caveats: **every `src/core|application|infrastructure` path in the
      run-2 PRDs predates the nine-crate workspace decomposition** (Milestone 5, ingest run 3), and
      the Milestone-1 benchmark files those PRDs reference have been relocated into per-crate
      `benches/` directories. *Derives: `intel/code-verification.md`; INGEST-CONFLICTS INFO
      "Sentinel and Autonomous docs disagree with the codebase map on where features live".*
- [ ] **VERIFY-02**: The three run-2 open-checkbox blocks that `intel/code-verification.md` leaves
      unverified are checked against the tree, and each produces a written verdict —
      *satisfied by shipped code*, *genuinely outstanding*, or *deferred with reason*. The blocks
      are `tasks-epic22-battalion-commander-hardening.md` (81 open),
      `tasks-autonomous-agent-features.md` (45 open) and
      `tasks-test-hardening-benchmarks-qa.md` (29 open). The deliverable is a verdict per block,
      **not** a task list derived from checkbox arithmetic — run 1 and run 2 both proved checkbox
      state understates reality. *Derives: `intel/task-completion-state.md`;
      `intel/code-verification.md` "Not yet verified".*
- [ ] **VERIFY-03**: The Milestone 3 epic-numbering defect is recorded once and permanently, and
      the defective source document is corrected in-repo. The authoritative numbering is the
      plan/epic-definition set — 19 Herald & Domain Type Consolidation, 20 Vision Pipeline
      Completion, 21 Autonomous Agent Completion, 22 Battalion & Commander Hardening, 23
      CLI/Config/Infrastructure Completion, 24 Test Hardening — agreed by 8 of 9 Milestone-3
      documents and every task list. `RELEASE_NOTES_MILESTONE_3.md` must stop asserting
      19=Conclave / 20=Council / 21=Grove / 22=Maneuver / 23=Commander, and its
      "What's Next (Milestone 4)" section must be marked as a superseded point-in-time
      forward-look (vision shipped). Two further release-notes claims must be corrected or
      withdrawn: `RoutingStrategy::PerformanceBased`, which **does not exist in the tree** and
      contradicts Epic 16 non-goal NG-3, and the Council/Maneuver API forms that disagree with the
      shipped surfaces. *Derives: `intel/code-verification.md` "Milestone 3 epic numbering";
      INGEST-CONFLICTS warnings 1, 2, 12, 13, 14.*
- [ ] **VERIFY-04**: The two vision port surfaces are recorded as **coexisting, not competing** —
      `crates/paladin-ports/src/output/vision_llm_port.rs` (Epic 13 `VisionCapableLlm` lineage,
      reached via `PaladinBuilder::enable_vision`) and
      `crates/paladin-ports/src/output/vision_port.rs` (Epic 20 `VisionPort` lineage, reached via
      `PaladinExecutionService::execute_with_vision`). Both ship. The recorded answer must state
      whether both are intended long-term or one is legacy, and must not plan a migration on the
      strength of the PRD conflict alone. Must also record whether Epic 13's
      encryption-at-rest / retention / zeroization security requirements
      (REQ-vision-security-encryption) were consciously dropped when Epic 20 finished the feature
      or merely not restated — Epic 20's `VisionError` has no `EncryptionError` variant.
      *Derives: `intel/code-verification.md` "Vision API surface — BOTH shipped";
      INGEST-CONFLICTS warnings 6, 8.*
- [ ] **VERIFY-05**: RECON-07's single coverage answer is extended to cover the two positions
      run 2 added, so the gate has one number and one scope across all four positions: 80% (nine
      Milestone-1 Epic PRDs), 85% (`unit-test-improvements` PRD), **overall ≥ 75% with a layered
      per-tier table** (Milestone 3 plan: core ≥ 85%, application ≥ 80%, infrastructure ≥ 70%, CLI
      ≥ 70%) and ≥ 80% / ≥ 70% re-asserted by Epic 24. The recorded answer must state how the
      module-scoped gates coexist with it — Herald ≥ 95% (REQ-herald-consolidation-quality-gates)
      and autonomous components ≥ 90% (REQ-autonomous-completion-quality-gates) — and must be
      falsifiable against the ~78% overall figure reported in the Milestone 3 release notes.
      *Derives: REQ-test-coverage-target-v1 / -v2, REQ-epic24-quality-gates; INGEST-CONFLICTS
      warning 3 and the module-scoped-targets INFO.*
- [ ] **VERIFY-06**: The live-API-test missing-key behaviour has one recorded answer, and the
      shipped harness matches it. Epic 23 FR-23.4.4 and Epic 24 US-24.7 both require graceful skip
      with a clear message; the post-Epic-24 cleanup deliberately reversed this, changing
      `require_api_key()` to panic so that "tests will now properly FAIL when keys are missing".
      Both positions are defensible and the reversal was conscious, so precedence cannot settle it.
      *Derives: REQ-provider-live-api-tests; INGEST-CONFLICTS warning 17.*

### Verified gap closure (CLOSE)

- [ ] **CLOSE-01**: Grove routing uses the LLM model from configuration instead of a hardcoded
      literal. `crates/paladin-battalion/src/grove_service.rs:537` builds its routing `LlmRequest`
      with `model: "gpt-4".to_string(), // TODO: Make configurable` in production code
      (`#[cfg(test)]` begins at line 732), so Grove routing silently ignores the configured
      provider. This is the **only defect in run-2 scope verified open against the tree**, it is
      the same defect class Epic 21 removed from `planning_service.rs` and
      `prompt_generation_service.rs`, and it means Epic 22's completion criterion "all inline TODOs
      in Battalion and Commander files resolved" is not met. *Derives: REQ-grove-llm-routing,
      REQ-autonomous-configurable-model; `codebase/CONCERNS.md` "Grove Service Model Hardcoded";
      INGEST-CONFLICTS warning 18.*
- [ ] **CLOSE-02**: Everything VERIFY-02 classifies as *genuinely outstanding* in Epics 14, 22 and
      24 is either closed or explicitly deferred with a recorded reason. Scope is set by Phase 5's
      verdicts, not by the 155 open checkboxes in those three lists. If VERIFY-02 finds all three
      blocks satisfied by shipped code, this requirement closes with a recorded "no work
      required" verdict rather than being deleted. *Derives: VERIFY-02.*
- [ ] **CLOSE-03**: The Phase 5 recorded answers that have code consequences are applied: the
      VERIFY-06 answer on live-API-test key handling is reflected in
      `tests/integration/llm_live_api_tests.rs`, and the VERIFY-04 answer on the two vision
      surfaces is reflected in the tree (both retained and documented as such, or one deprecated
      with a migration note). No surface is removed without a recorded decision.
      *Derives: VERIFY-04, VERIFY-06.*

---

## Competing variants (preserved unmerged)

**16 variant groups, 30 entries**, carried verbatim in scope from `.planning/intel/requirements.md`
(12 from run 1, 18 from run 2). **No winner is selected here.** Where a variant matters, the
pointer below is to the codebase map, `intel/code-verification.md` or a shipped `file:line` — the
real arbiters — not to a resolution. `INGEST-CONFLICTS.md` counts 26 warnings across these
groups; the entry count is higher because several groups carry three entries.

Observations of shipped code in this section are **facts about the tree, not decisions**. Several
of them plainly favour one variant; recording that as the answer is a Phase 1 / Phase 5 ADR, and
it is deliberately not asserted here. The user has stated that variants are expected and that
resolving past disagreements is not the goal of this ingest.

### Group 1 — project-wide test coverage gate (4 positions)

- **REQ-test-coverage-target-v1** — nine Epic PRDs (Epics 1-8, 10): unit coverage ≥ 80%,
  integration ≥ 70%, measured via cargo-llvm-cov.
- **REQ-test-coverage-target-v2** — `unit-test-improvements/prd-improve-unit-test-coverage.md`:
  overall coverage MUST exceed 85%; functions under 50% MUST reach 80%; stated baseline 67.79%.
- *Run-2 third position (context, not a `REQ-*` entry)* — `Project_Plan_Milestone_3.md`
  "Cross-Cutting Concerns": layered targets, core domain ≥ 85%, application services ≥ 80%,
  infrastructure adapters ≥ 70%, CLI commands ≥ 70%, **overall ≥ 75%**.
- *Run-2 fourth position* — **REQ-epic24-quality-gates** re-asserts ≥ 80% for all modules and
  ≥ 70% integration.
- Status: **unresolved, four ways.** Measured 60.88% unit / 67.79% integration (Milestone 1) and
  ~78% overall (Milestone 3 release notes). ~78% passes the M3 plan gate, fails Epic 24's, fails
  the 85% gate. Module-scoped gates (Herald ≥ 95%, autonomous ≥ 90%) coexist above whatever the
  global number becomes. Recording the answer is RECON-07 + VERIFY-05.

### Group 2 — valid temperature range

- **REQ-temperature-range-v1** — Epic 1 FR-2.3 / US-2: builder MUST validate `[0.0, 1.0]` and
  reject values above 1.0.
- **REQ-temperature-range-v2** — Epic 6 REQ-5: DeepSeek adapter MUST support temperature 0.0-2.0.
- Status: **unresolved.** A build-time `[0.0, 1.0]` clamp makes the DeepSeek range unreachable
  through the normal Paladin path. Run 2 adds a third overlapping surface without settling it:
  **REQ-dynamic-temperature** classifies tasks into bands (Factual 0.1-0.3, Analytical 0.3-0.5,
  Conversational 0.5-0.7, Creative 0.7-1.0) with configurable `temperature_bounds`, and the
  Epic 14 DOC states bounds of 0.1-1.0. Shipped: `TaskType` and the band logic exist in
  `src/application/services/paladin/temperature_service.rs`. See `codebase/ARCHITECTURE.md`.

### Group 3 — `BattalionConfig` field set and `metadata_output_dir` ownership (3 positions)

- **REQ-battalion-config-v1** — Epic 4 FR-4.1: `name`, `description`, `timeout_seconds`,
  `retry_policy` (struct: max_attempts / base_delay / max_delay / exponential_backoff / jitter),
  `error_strategy`, `metadata_output_dir`.
- **REQ-battalion-config-v2** — Epic 5 FR-7: `name: String`, `timeout_seconds: u64`,
  `retry_attempts: u32`, `error_strategy: ErrorStrategy`, `enable_checkpointing: bool`,
  `metadata_output_dir: Option<PathBuf>`; defaults for all fields.
- **REQ-commander-config-metadata-dir-v3** *(run 2, Epic 22 FR-10.1)* — relocates the same field to
  `CommanderConfig` in `src/core/platform/container/battalion/commander_config.rs`, with YAML
  surface `commander.metadata_output_dir` and writability validated before first execution.
- Status: **unresolved, and Epic 22 relocated the conflict rather than closing it.** Two distinct
  `BattalionConfig` structs exist in the tree
  (`crates/paladin-core/src/platform/container/battalion/mod.rs:37` and
  `crates/paladin-core/src/platform/container/citadel.rs:280`, the latter carrying
  `max_concurrency` / `timeout_seconds` / `continue_on_error`). Tree observation, not a decision:
  `metadata_output_dir` ships on `BattalionConfig` at `battalion/mod.rs:54` with a writability
  check at `:116-124`, and the export path is `crates/paladin-battalion/src/commander.rs:870` —
  i.e. the v3 `CommanderConfig` position is not the shipped location. See `codebase/STRUCTURE.md`.

### Group 4 — `BattalionResult` / `BattalionMetadata` field set (4 positions)

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
- **REQ-battalion-metadata-extension** *(run 2, Epic 22 FR-8)* — later position: `BattalionMetadata`
  gains `per_paladin_times`, `per_paladin_tokens` and `total_tokens`, all `Serialize`/`Deserialize`,
  located in `battalion/battalion_result.rs`. Note Epic 5 typed `per_paladin_times` as `Vec<u64>`.
- Status: **unresolved.** Tree observation, not a decision: `BattalionMetadata` ships
  `per_paladin_times: HashMap<String, u64>` (`battalion/mod.rs:582`) and
  `per_paladin_tokens: HashMap<String, TokenUsage>` (`:585`) — the run-2 field names with the
  run-1 module path (`battalion/mod.rs`, not `battalion_result.rs`). See `codebase/ARCHITECTURE.md`.

### Group 5 — minimum Paladin count for Formation

- **REQ-formation-min-paladins-v1** — Epic 4 FR-4.5 / FR-4.8: Formation MUST validate ≥ 2
  Paladins; Phalanx accepts ≥ 2; Majority aggregation requires ≥ 3.
- **REQ-formation-min-paladins-v2** — Epic 5 FR-1 / FR-3: Commander validates only that ≥ 1
  Paladin is provided, and Auto rule 1 routes a single Paladin to Formation as the trivial case.
- Status: **unresolved and live in shipped code** —
  `crates/paladin-core/src/platform/container/battalion/formation.rs:109` errors with
  "Formation requires at least 2 Paladins", so the documented single-Paladin Auto happy path
  fails validation at runtime.

### Group 6 — `Herald` trait signature

- **REQ-herald-trait-v1** — Epic 8 FR-1: infallible `-> String` returns;
  `format_paladin_result`, `format_battalion_result`, `format_paladin_stream -> Option<String>`,
  `format_error`; `Send + Sync`.
- **REQ-herald-trait-v2** — same PRD, section 6.2: fallible `-> Result<String, HeraldError>`;
  `format_stream_chunk`, added `finalize_stream(&ExecutionMetadata)`, `name()`, `mime_type()`.
- Status: **unresolved on paper.** The shipped trait at
  `crates/paladin-core/src/platform/container/herald.rs:49` documents a `HeraldError` return —
  i.e. the code leans fallible — and FR-10's graceful-degradation requirement cannot be expressed
  by the infallible form. Recording that as the answer is RECON-06; it is not asserted here.
  Run-2 note: **REQ-herald-type-consolidation** is a later position on the *placeholder-type* half
  of this question, and the tree carries no placeholder or TODO in `herald.rs`.

### Group 7 — Qdrant Sanctum adapter *(run 2)*

- **REQ-qdrant-sanctum-adapter-v1** — Epic 11 US-11.4 / FR-8: `QdrantSanctumAdapter` in
  `src/infrastructure/adapters/sanctum/qdrant_adapter.rs`, official Qdrant Rust client, host +
  port **6334** + `api_key` + collection + `use_grpc: true`, collection auto-creation, connection
  pooling and retry, feature flag `qdrant`, collection name `paladin_memories_{environment}`,
  1536 dims, Cosine, indexed `paladin_id`/`memory_type`/`created_at`/`importance`,
  < 500 ms top-10 on 100 K vectors.
- **REQ-qdrant-sanctum-adapter-v2** — Epic 12 US-12.1 / FR-1: `QdrantSanctum` in
  `qdrant_sanctum.rs`, URL `http://localhost:6333`, optional API key, `collection_name`,
  `vector_size` 1536, `distance` Cosine, `on_disk: true`, health check verifying the collection,
  error mapping to `SanctumError`, `qdrant-client = "1.7"`.
- Status: **unresolved on naming, connection shape and version — but the feature shipped, so the
  Epic 11-vs-Epic 12 ownership question is moot for forward planning.**
  `intel/code-verification.md` records the adapter as **verified shipped**, contradicting
  `EPIC_11_COMPLETION_SUMMARY.md`'s "Task 5.0: Qdrant Adapter (DEFERRED)". Tree observation:
  `crates/paladin-memory/src/sanctum/qdrant_adapter.rs`, `qdrant-client` **1.14** behind the
  `qdrant` feature, integration target `qdrant_sanctum_integration`. Consult
  `codebase/STACK.md` and `codebase/STRUCTURE.md` for the current shape, not either PRD.

### Group 8 — image format validation ownership *(run 2)*

- **REQ-vision-format-validation-v1** — Epic 13 FR-1.2: the **framework** validates formats and
  accepts only PNG, JPEG, GIF, WebP before dispatch; `VisionError::UnsupportedFormat` and
  `FileTooLarge { size, max }`; CLI reports unsupported formats clearly.
- **REQ-vision-format-validation-v2** — Epic 20 US-20.1 / US-20.2 / NG-3 / NG-5: **adapters
  delegate** validation to the provider API and support every format the provider accepts; no
  conversion, no preprocessing; size limits are the provider's (OpenAI ~20 MB, Anthropic
  model-dependent), documented in `docs/SENTINEL.md`.
- Status: **unresolved, opposite responsibility assignment.** Determines whether an unsupported
  image fails locally with a typed error or as a provider 400, and whether `FileTooLarge` exists
  at all. Tree observation: `VisionError` lives in
  `crates/paladin-core/src/platform/container/vision.rs:189` (the Epic 13 module path).

### Group 9 — OpenAI vision adapter *(run 2)*

- **REQ-openai-vision-adapter-v1** — Epic 13 US-13.2 / FR-2: extend the existing
  `OpenAILlmAdapter`; support `gpt-4-vision-preview`, `gpt-4o`, `gpt-4o-mini`; convert
  `VisionContent` to OpenAI message format for URLs and base64; image token counting.
- **REQ-openai-vision-adapter-v2** — Epic 20 US-20.1 / FR-1: a **dedicated**
  `OpenAIVisionAdapter::analyze_image()` in `openai_vision.rs` POSTing to
  `/v1/chat/completions`; explicit status mapping (400 → `InvalidImage`, 401 →
  `AuthenticationError`, 429/5xx → backoff); configurable `max_retries` (3),
  `initial_backoff_ms`, `backoff_multiplier`; never retry 400/401/403/404; mocked-HTTP unit tests
  plus `ENABLE_VISION_TESTS=true` integration tests.
- Status: **unresolved.** Tree observation: `crates/paladin-llm/src/openai/vision.rs` exists
  alongside `openai/adapter.rs`, i.e. a dedicated vision module *and* a general adapter.

### Group 10 — Anthropic vision adapter *(run 2)*

- **REQ-anthropic-vision-adapter-v1** — Epic 13 US-13.3 / FR-3: extend `AnthropicLlmAdapter`;
  Claude 3 Opus / Sonnet / Haiku; auto-convert URLs to base64; content-block format; rate limiting.
- **REQ-anthropic-vision-adapter-v2** — Epic 20 US-20.2 / FR-2: dedicated
  `AnthropicVisionAdapter::analyze_image()` in `anthropic_vision.rs` POSTing to `/v1/messages`
  with `x-api-key` and `anthropic-version: 2023-06-01`; the same configurable retry contract as
  the OpenAI adapter.
- Status: **unresolved.** Tree observation: `crates/paladin-llm/src/anthropic/vision.rs` exists
  alongside `anthropic/adapter.rs`.

### Group 11 — Paladin vision entry point *(run 2)*

- **REQ-paladin-vision-api-v1** — Epic 13 US-13.4 / FR-5: `Paladin::run_with_vision(task, images)`
  and `PaladinBuilder::enable_vision(bool)`; validate adapter vision support before execution.
- **REQ-paladin-vision-api-v2** — Epic 20 US-20.3 / FR-3:
  `PaladinExecutionService::execute_with_vision(paladin, prompt, images)`; provider derived from
  `paladin.model()` (`gpt-*` → OpenAI, `claude-*` → Anthropic) with
  `VisionError::UnsupportedProvider` otherwise; non-streaming; respects `max_loops`, `stop_words`,
  `timeout_seconds`; Garrison persistence; returns `VisionResult`.
- Status: **both surfaces ship — see VERIFY-04, and `intel/code-verification.md`, which records
  this as coexistence rather than an unresolved contradiction.** Tree observation:
  `PaladinBuilder::enable_vision` at `src/application/services/paladin/paladin_builder.rs:516`
  and `PaladinExecutionService::execute_with_vision` at
  `src/application/services/paladin/paladin_execution_service.rs:517`, both reached from
  `src/application/cli/commands/agent.rs:326,433`. Related, also coexisting rather than
  competing: **REQ-vision-capable-llm-trait** (`vision_llm_port.rs`) and **REQ-vision-port**
  (`vision_port.rs`).

### Group 12 — `VisionError` variant set *(run 2)*

- **REQ-vision-error-model-v1** — Epic 13 FR-12: `UnsupportedFormat`, `FileTooLarge`,
  `InvalidImage`, `ModelNotSupported`, `NetworkError`, `EncryptionError`, `IoError`; plus
  `DocumentError` with `UnsupportedFormat`, `EncryptedPdf`, `CorruptedFile`, `ExtractionFailed`,
  `IoError`.
- **REQ-vision-error-model-v2** — Epic 20 FR-5.1: `InvalidImage`, `UnsupportedFormat`,
  `AuthenticationError`, `RateLimitExceeded(u64)`, `ProviderError`, `NetworkError`,
  `Timeout(u64)`, `UnsupportedProvider`, `MaxRetriesExceeded`; located in
  `container/sentinel/vision_types.rs`; **no** `EncryptionError`, **no** `FileTooLarge`.
- Status: **unresolved, and the divergence is load-bearing** — dropping `EncryptionError` drops
  the only error surface for Epic 13's encryption-at-rest requirement
  (**REQ-vision-security-encryption**), which Epic 20 does not restate. Whether that was a
  conscious decision is part of VERIFY-04. Tree observation: `VisionError` is at
  `crates/paladin-core/src/platform/container/vision.rs:189`, not
  `container/sentinel/vision_types.rs`.

### Group 13 — handoff tool name and parameters *(run 2, 3 names / 2 parameter sets)*

- **REQ-handoff-tool-v1** — Epic 14 FR-5: tool named **`handoff_to_agent`**, required parameters
  **`agent_name`** (enum of available agents) and **`message`**; auto-registered when handoffs are
  configured; validates `agent_name`, executes via `HandoffService`, returns the specialist result
  for synthesis, tracks the chain, errors on circular handoff / invalid name / exceeded depth.
- **REQ-handoff-tool-v2** — Epic 21 FR-3 + Epic 23 Non-Goal 5: auto-registered from
  `PaladinBuilder::build()` detecting a prior `with_handoffs()`; parameters **`specialist_name`**
  (enum of configured specialists) and **`task_description`**; schema carries specialist names,
  descriptions and parameter requirements; registration idempotent and updated when handoffs are
  reconfigured; Epic 23 refers to the tool as **`handoff_to_specialist`**.
- Status: **unresolved on paper.** The tool name and parameter names are part of the JSON schema
  sent to the model, so a mismatch means unroutable tool calls. Tree observation, not a decision:
  `crates/paladin-core/src/platform/container/arsenal/handoff_tool.rs:63` emits
  `"name": "handoff_to_agent"` with `"required": ["agent_name", "message"]` at `:78`.

### Group 14 — Grove routing threshold *(run 2, 3 names / 3 defaults)*

- **REQ-grove-config-v1** — Epic 16 FR-2.3: `GroveConfig { routing_strategy, fallback_tree:
  Option<String>, similarity_threshold }`, defaults `KeywordMatch` and **0.7**, validated on Grove
  creation.
- **REQ-grove-config-v2** — Epic 22 FR-6: `GroveConfig` gains `routing_fallback: String`
  ("keyword" | "error") and `min_confidence: f32` default **0.5**, range 0.0-1.0, with validation
  rejecting invalid fallback values and out-of-range confidence.
- *Third form (context, not a `REQ-*` entry)* — `RELEASE_NOTES_MILESTONE_3.md`:
  `GroveConfig { routing_strategy, confidence_threshold: 0.6 }`.
- Status: **unresolved — either Grove has two thresholds with different semantics (similarity for
  embedding routing, confidence for LLM routing) or one field was renamed twice.** Related
  release-notes claim, **verified absent from the tree**: `RoutingStrategy::PerformanceBased` and
  "dynamic learning" are advertised as shipped but contradict Epic 16 NG-3 and do not exist —
  correcting that is part of VERIFY-03. Shipped routing strategies live in
  `crates/paladin-core/src/platform/container/battalion/grove.rs`.

### Group 15 — `ErrorStrategy` variant sets *(run 2, one type name / two disjoint variant sets)*

- **REQ-battalion-error-strategy** *(run 1, Epic 4 FR-4.3)* — `FailFast`, `ContinueOnError`,
  `RetryThenContinue`. Epic 22 US-22.5 names its Commander tests after exactly these three
  behaviours plus a `continue_on_error: true` config flag.
- **REQ-maneuver-error-strategy-v2** *(run 2, Epic 17 FR-5.1)* — `FailFast`, `ContinueParallel`,
  `IgnoreErrors` for `ManeuverConfig`.
- Status: **unresolved.** Beyond `FailFast` the sets are disjoint, so either two enums share a name
  across modules or one enum grew incompatible variants. `RELEASE_NOTES_MILESTONE_3.md` documents
  both sets in different sections of the same document. Shipped error aggregation:
  `crates/paladin-battalion/src/error_aggregation.rs`.

### Group 16 — `max_loops` scalar vs enum *(run 2 supersession, both preserved)*

- **Run-1 position** — `REQ-paladin-entity` treats `PaladinData.max_loops` as part of a flat field
  set and `REQ-paladin-builder` requires validation in range `[1, 100]`, default 3.
- **REQ-max-loops-auto** *(run 2, Epic 14 FR-1.1)* — `MaxLoops` becomes an enum with
  `MaxLoops::Fixed(u32)` and `MaxLoops::Auto { max_subtasks: u32 }`;
  `PaladinBuilder::max_loops(MaxLoops)` accepts the enum; `Auto` routes execution through
  `PlanningService`; planning loops must not exceed `max_subtasks`.
- Status: **genuine type-level supersession, not a documentation slip.** A range validation defined
  on an integer cannot apply unchanged to an enum, and every Milestone-1 requirement or test
  assuming a numeric `max_loops` is affected. The run-1 criterion is marked superseded, **not
  deleted**. Tree observation: `MaxLoops` is an enum at
  `crates/paladin-core/src/platform/container/paladin.rs:42`.

### Superseded but preserved — run-2 later positions on run-1 requirements

Recorded so that a later reader does not mistake supersession for a contradiction. In every case
the run-1 entry is left intact in the ledger and the run-2 entry is the later position.

| Run-1 requirement | Run-2 later position | Nature of the change |
|---|---|---|
| `REQ-paladin-entity`, `REQ-paladin-builder` (`max_loops` scalar, `[1,100]`) | `REQ-max-loops-auto` | Scalar → enum; see variant group 16 |
| `REQ-herald-trait-v1/-v2`, Herald placeholder types | `REQ-herald-type-consolidation` | Placeholders removed, real domain types imported; no shim required |
| `REQ-battalion-result-v1/-v2` | `REQ-battalion-metadata-extension` | Adds `per_paladin_times`, `per_paladin_tokens`, `total_tokens` |
| `REQ-battalion-config-v1/-v2` (`metadata_output_dir`) | `REQ-commander-config-metadata-dir-v3` | Field relocated to `CommanderConfig`; see variant group 3 |
| `REQ-autonomous-configuration` (Epic 14) | `REQ-autonomous-completion-config-schema` (Epic 21) | Consolidated YAML schema, handoff retry/history/eviction settings, `validate_at: planning_time` |
| `REQ-commander-telemetry` (Epic 5, deferred) | `REQ-commander-metadata-export` (Epic 22) | Adds the JSON schema, `<strategy>_<timestamp>_<uuid>.json` naming, non-fatal I/O, < 50 ms overhead |
| `REQ-provider-testing` (Epic 6, live-API suite deferred) | `REQ-provider-live-api-tests` (Epic 24) + post-cleanup reversal | Un-defers the suite, then reverses the skip semantics; see VERIFY-06 |
| `REQ-performance-benchmarking` (Epic 10, suites disabled) | `REQ-battalion-benchmark-repair` (Epic 24) | Repairs Campaign / ChainOfCommand benchmarks and requires `cargo bench --no-run` in CI |

---

## Milestone 1 as-shipped ledger

All 115 requirement IDs extracted by ingest run 1, with verified status. **Not forward scope** —
listed so nothing is lost and so runs 3-5 merge against stable keys. Acceptance criteria are not
repeated; they live in `.planning/intel/requirements.md`.

Status key: `Shipped` = satisfied by v0.7.0 code and a complete task list · `Verify` = code exists,
completion asserted only by the 2026-01 task list, confirmation is part of RECON-01 ·
`Partial → X` = residual work tracked by forward requirement X · `Variant` = see competing variants
· `Deferred → v2` · `Code diverges` = shipped implementation differs from the ingested requirement.

### Epic 1 — Paladin Domain Foundation (182/182 items, 100%)

| ID | Status |
|---|---|
| REQ-paladin-entity | Shipped — `crates/paladin-core/src/platform/container/paladin.rs`; `max_loops` superseded by `REQ-max-loops-auto` (enum at `paladin.rs:42`) |
| REQ-paladin-builder | Shipped — `src/application/services/paladin/paladin_builder.rs`; `[1,100]` validation superseded, see variant group 16 |
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
| REQ-garrison-longterm-port | Code diverges — semantic retrieval ships as **Sanctum** (Qdrant + in-memory), not as a `sqlite-vss` extension of Garrison. Run 2 supplies the missing requirements: REQ-sanctum-port, REQ-embedding-port, REQ-sanctum-domain-model |
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
| REQ-arsenal-resilience | Partial → QUAL-04 (failure paths untested); run 2 adds REQ-tool-call-loop-tests and REQ-mcp-gated-integration-tests |
| REQ-arsenal-context-injection | Shipped — 90.31% |

### Epic 4 — Battalion Orchestration (233/235 items; tasks 6.0 and 7.0 open)

| ID | Status |
|---|---|
| REQ-battalion-config-v1 | Variant (group 3) |
| REQ-battalion-result-v1 | Variant (group 4) |
| REQ-battalion-error-strategy | Shipped — `error_aggregation.rs` 99.60%; Variant (group 15) against `REQ-maneuver-error-strategy-v2` |
| REQ-battalion-retry-policy | Shipped — `retry.rs` 100% |
| REQ-formation-min-paladins-v1 | Variant (group 5) |
| REQ-formation-construction | Shipped |
| REQ-formation-execution | Shipped — `formation_service.rs` 88.14% |
| REQ-formation-output | Shipped |
| REQ-phalanx-construction | Shipped |
| REQ-phalanx-concurrency | Partial → GAP-02 (concurrency claims unvalidated) |
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
| REQ-battalion-config-v2 | Variant (group 3) |
| REQ-battalion-result-v2 | Variant (group 4) |
| REQ-formation-min-paladins-v2 | Variant (group 5) |
| REQ-commander-construction | Shipped — `commander.rs` 81.79% |
| REQ-commander-strategy-types | Shipped |
| REQ-commander-auto-selection | Partial → GAP-05 (one failing keyword test) |
| REQ-commander-execute | Shipped |
| REQ-commander-result-normalization | Partial → GAP-04 (task 5.0 open) |
| REQ-commander-error-strategy | Partial → QUAL-04 (error-path tests `#[ignore]`d; 4 remain in `commander.rs`) |
| REQ-commander-config-passthrough | Shipped |
| REQ-commander-service-composition | Shipped |
| REQ-commander-telemetry | Partial → GAP-04; later position REQ-commander-metadata-export. Tree observation: export path exists at `crates/paladin-battalion/src/commander.rs:870` |
| REQ-commander-validation | Shipped |

### Epic 6 — Provider Expansion (180/199 items, 90%; task 7.0 deferred)

| ID | Status |
|---|---|
| REQ-llm-port-interface | Shipped — capability struct included |
| REQ-deepseek-adapter | Shipped (15.02% coverage → QUAL-02) |
| REQ-anthropic-adapter | Shipped (28.19% coverage → QUAL-02); post-Epic-24 cleanup fixed serde underscore-prefixed fields |
| REQ-provider-configuration | Shipped — `provider_factory.rs`, env-var keys |
| REQ-provider-backward-compat | Shipped |
| REQ-provider-error-mapping | Shipped |
| REQ-provider-testing | Partial — mocked-HTTP unit tests shipped; live-API suite un-deferred by run 2 (REQ-provider-live-api-tests) with contested skip semantics → VERIFY-06. Remains one of the six `code-verification.md` "Not yet verified" blocks (19 open) |
| REQ-provider-documentation | Shipped |
| REQ-temperature-range-v1 / -v2 | Variant (group 2) |

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
| REQ-herald-trait-v1 / -v2 | Variant (group 6) |
| REQ-herald-builtin-formatters | Shipped — `crates/paladin-herald/src/{json,markdown,table}_herald.rs` |
| REQ-herald-streaming | Shipped |
| REQ-herald-configuration | Shipped — `src/config/herald.rs` |
| REQ-herald-default-and-override | Shipped |
| REQ-herald-paladin-result-fields | Verify — Herald is wired into `paladin_execution_service.rs` (`with_herald`, used at line 428), which the task list listed as open |
| REQ-herald-battalion-result-fields | Partial → GAP-03 (depends on RECON-03) |
| REQ-herald-registry | Shipped — `src/application/services/herald/`; run 2 adds default auto-registration (REQ-herald-formatter-autoregistration) |
| REQ-herald-builder-integration | Shipped |
| REQ-herald-error-handling | Shipped |

### Epic 9 — Armory CLI Tools (238/241 items, 99%)

| ID | Status |
|---|---|
| REQ-cli-structure | Shipped — clap v4 derive, `src/bin/paladin-cli.rs`; **relocated** to `src/application/cli/` by Epic 17.5 — `src/cli` is absent from the tree |
| REQ-cli-agent-run | Shipped — `src/application/cli/commands/agent.rs` |
| REQ-cli-agent-new | Shipped — `src/application/cli/templates/` |
| REQ-cli-battalion-run | Shipped — `src/application/cli/commands/battalion.rs` |
| REQ-cli-battalion-new | Shipped |
| REQ-cli-arsenal-list | Shipped — `src/application/cli/commands/arsenal.rs` |
| REQ-cli-arsenal-test | Shipped |
| REQ-cli-config-format | Shipped — YAML only |
| REQ-cli-env-vars | Shipped |
| REQ-cli-validation-errors | Shipped |
| REQ-cli-output-formatting | Shipped |
| REQ-cli-interactive-mode | Shipped — note the shipped interactive REPL exceeds Epic 9 non-goal NG-7 (RECON-01 records this) |
| *(CLI e2e tests 13.4-13.6)* | Un-deferred by run 2 — REQ-mock-llm-adapter and REQ-cli-tiered-environment-testing supply the mock provider the deferral was blocked on |

### Epic 10 — Validation & Documentation (103/103 items; Task 7.0 disputed)

| ID | Status |
|---|---|
| REQ-integration-testing | Partial → GAP-02, QUAL-03 (67.79% vs 70% gate) |
| REQ-performance-benchmarking | Partial → QUAL-05; benchmarks relocated into per-crate `benches/`; later position REQ-battalion-benchmark-repair |
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
| REQ-test-coverage-target-v1 / -v2 | Variant (group 1) |
| REQ-unit-test-gap-closure | Partial → QUAL-01, QUAL-02 |

---

## Milestone 2-3 as-shipped ledger

All 118 requirement IDs extracted by ingest run 2 (Epics 11-24), with status. **Not forward
scope.** Acceptance criteria live in `.planning/intel/requirements.md`.

**Read this section with the path caveat.** Every run-2 PRD assumes a single-crate
`src/core|application|infrastructure` layout. The workspace was decomposed into nine crates in
Milestone 5 (ingest run 3), so the `src/...` paths in those PRDs are historical. Citations below
are the **current** locations, verified by direct inspection of `release/v0.7.0`.

Status key: `Shipped` = the named artefact exists at the cited path (component-level evidence;
per-criterion confirmation is VERIFY-01) · `Shipped (relocated)` = shipped, at a different path
than the PRD specified · `Verify` = related code exists but the requirement's specific behaviour
was not inspected · `Variant` = see competing variants · `Unverified candidate` = sits behind one
of the three open-checkbox blocks `code-verification.md` leaves unverified → VERIFY-02 ·
`Open defect → X` = verified open work.

### Epic 11 — Sanctum Memory Foundation (8 IDs)

Epic-level note: `EPIC_11_COMPLETION_SUMMARY.md` claims COMPLETE while recording Qdrant as
DEFERRED, and `tasks-sanctum-memory-foundation.md` carries **111 open checkboxes**.
`intel/code-verification.md` records Qdrant as **verified shipped**, so the 111 count is stale and
is *not* carried as forward work.

| ID | Status |
|---|---|
| REQ-embedding-port | Shipped (relocated) — `crates/paladin-ports/src/output/embedding_port.rs` |
| REQ-openai-embedding-adapter | Shipped (relocated) — `crates/paladin-llm/src/openai/embedding.rs` |
| REQ-sanctum-port | Shipped (relocated) — `crates/paladin-ports/src/output/sanctum_port.rs` |
| REQ-qdrant-sanctum-adapter-v1 | Variant (group 7) — shipped as `crates/paladin-memory/src/sanctum/qdrant_adapter.rs`, `qdrant-client` 1.14 behind the `qdrant` feature; the Epic 11 "DEFERRED" record is stale per `code-verification.md` |
| REQ-in-memory-sanctum | Shipped (relocated) — `crates/paladin-memory/src/sanctum/in_memory_adapter.rs` |
| REQ-sanctum-domain-model | Shipped (relocated) — `crates/paladin-core/src/platform/container/sanctum.rs` |
| REQ-sanctum-configuration | Shipped (relocated) — `crates/paladin-memory/src/config/sanctum.rs` |
| REQ-sanctum-garrison-coexistence | Shipped — Garrison and Sanctum are independent module families under `crates/paladin-memory/src/{garrison,sanctum}/` with separate config modules |

### Epic 12 — Sanctum RAG Integration (8 IDs)

| ID | Status |
|---|---|
| REQ-qdrant-sanctum-adapter-v2 | Variant (group 7) — same shipped adapter as v1; neither PRD's naming or connection shape is authoritative, consult `codebase/STACK.md` |
| REQ-paladin-builder-sanctum-integration | Shipped — `with_sanctum` / `with_embedding_port` / `memory_extraction_strategy` on `src/application/services/paladin/paladin_builder.rs` (fields at `:90`, `:135`; methods documented at `:727-745`) |
| REQ-memory-extraction-strategy | Shipped (relocated) — `crates/paladin-memory/src/services/mod.rs`, `memory_extraction_service.rs` |
| REQ-rag-retrieval-service | Shipped (relocated) — `crates/paladin-memory/src/services/rag_retrieval_service.rs` |
| REQ-rag-config | Shipped (relocated) — `crates/paladin-memory/src/config/rag.rs` |
| REQ-memory-extraction-service | Shipped (relocated) — `crates/paladin-memory/src/services/memory_extraction_service.rs` |
| REQ-execution-service-rag-integration | Verify — `codebase/ARCHITECTURE.md` documents RAG context injection as a step in the execution flow; the `## Relevant Context` prompt section, async non-blocking extraction and the three metrics were not inspected → VERIFY-01 |
| REQ-rag-performance-targets | Verify — `crates/paladin-memory/benches/sanctum_benchmarks.rs` exists; no p95 retrieval/extraction baseline is recorded anywhere in `.planning/` → VERIFY-01, QUAL-05 |

### Epic 13 — Sentinel Vision System (13 IDs)

Epic-level note: `intel/code-verification.md` records Sentinel vision as **verified shipped**,
against the Milestone 3 release notes listing it under "What's Next (Milestone 4)".

| ID | Status |
|---|---|
| REQ-vision-content-model | Shipped (relocated) — `ImageDetail` at `crates/paladin-core/src/platform/container/vision.rs:15`, `VisionContent` at `:33`, `VisionRequest` at `:131` |
| REQ-vision-format-validation-v1 | Variant (group 8) |
| REQ-openai-vision-adapter-v1 | Variant (group 9) — `crates/paladin-llm/src/openai/vision.rs` |
| REQ-anthropic-vision-adapter-v1 | Variant (group 10) — `crates/paladin-llm/src/anthropic/vision.rs` |
| REQ-vision-capable-llm-trait | Shipped (relocated) — `crates/paladin-ports/src/output/vision_llm_port.rs`. **Coexists** with `REQ-vision-port`; recorded as coexistence, not a variant → VERIFY-04 |
| REQ-paladin-vision-api-v1 | Variant (group 11) — `PaladinBuilder::enable_vision` at `src/application/services/paladin/paladin_builder.rs:516` |
| REQ-vision-error-model-v1 | Variant (group 12) — `VisionError` at `crates/paladin-core/src/platform/container/vision.rs:189` |
| REQ-vision-security-encryption | **Not found in tree** — no encryption-at-rest, zeroization or retention-policy artefact was located, and Epic 20's `VisionError` omits `EncryptionError`. Whether this was consciously dropped is VERIFY-04. Not carried as forward work until that answer exists |
| REQ-pdf-extraction | Shipped (relocated) — `crates/paladin-content/src/adapters/document/pdf_extractor.rs` |
| REQ-document-port | Shipped (relocated) — `crates/paladin-ports/src/input/document_port.rs`; `Document` domain type at `crates/paladin-core/src/platform/container/document.rs`; adapter at `crates/paladin-content/src/adapters/document/document_adapter.rs` |
| REQ-vision-cli-and-yaml | Shipped — `--image` / `--document` handling in `src/application/cli/commands/agent.rs` (`enable_vision` at `:326`, `execute_with_vision` at `:433`); example `examples/document_processing.rs` |
| REQ-battalion-vision-integration | Verify — narrowed by Epic 20 NG-6 (no batch vision API; concurrency handled at Battalion level). `tests/integration/vision_integration_test.rs`, `examples/vision_battalion.rs` and `docs/src/appendix/battalion-vision-support.md` exist per `code-verification.md`; per-pattern coverage not inspected → VERIFY-01 |
| REQ-vision-performance-and-config | Shipped (relocated) — `crates/paladin-llm/src/config/vision.rs`, `VisionConfig` referenced from `crates/paladin-llm/src/openai/mod.rs`; the end-to-end latency targets have no recorded measurement → VERIFY-01 |

### Epic 20 — Vision Pipeline Completion (6 IDs)

Authoritative numbering: **20 = Vision Pipeline Completion** (plan / epic-definition numbering).
The release notes' "Epic 20 = Council Pattern" mapping is a documentation defect → VERIFY-03.

| ID | Status |
|---|---|
| REQ-vision-format-validation-v2 | Variant (group 8) |
| REQ-openai-vision-adapter-v2 | Variant (group 9) — `crates/paladin-llm/src/openai/vision.rs` is a dedicated vision module, consistent with the v2 shape; retry-contract criteria not inspected → VERIFY-01 |
| REQ-anthropic-vision-adapter-v2 | Variant (group 10) — `crates/paladin-llm/src/anthropic/vision.rs` |
| REQ-vision-port | Shipped (relocated) — `crates/paladin-ports/src/output/vision_port.rs`. **Coexists** with `REQ-vision-capable-llm-trait` → VERIFY-04 |
| REQ-paladin-vision-api-v2 | Variant (group 11) — `PaladinExecutionService::execute_with_vision` at `src/application/services/paladin/paladin_execution_service.rs:517` |
| REQ-vision-error-model-v2 | Variant (group 12) — shipped `VisionError` is at `container/vision.rs:189`, not the specified `container/sentinel/vision_types.rs` |

### Epic 14 — Autonomous Agent Features (8 IDs)

Epic-level note: `tasks-autonomous-agent-features.md` carries **45 open checkboxes** and is one of
the three blocks `code-verification.md` leaves unverified. Shipped artefacts exist for every
requirement below, so the count is a claim to verify (VERIFY-02), not a backlog.

| ID | Status |
|---|---|
| REQ-max-loops-auto | Shipped — `MaxLoops` enum at `crates/paladin-core/src/platform/container/paladin.rs:42`. Supersedes the run-1 scalar; see variant group 16 |
| REQ-planning-service | Shipped — `src/application/services/paladin/planning_service.rs`; domain types at `crates/paladin-core/src/platform/container/planning.rs` |
| REQ-prompt-generation-service | Shipped — `src/application/services/paladin/prompt_generation_service.rs`; example `examples/autonomous_prompt_generation.rs`; test module re-enablement is REQ-prompt-generation-test-reenable |
| REQ-dynamic-temperature | Shipped — `src/application/services/paladin/temperature_service.rs` with `enum TaskType`; interacts with variant group 2 |
| REQ-handoff-infrastructure | Shipped — `src/application/services/paladin/handoff_service.rs`, `crates/paladin-core/src/platform/container/handoff.rs`, `HandoffStrategy` in `autonomous_config.rs`; example `examples/agent_handoffs.rs` |
| REQ-handoff-tool-v1 | Variant (group 13) — `crates/paladin-core/src/platform/container/arsenal/handoff_tool.rs` |
| REQ-autonomous-configuration | Shipped (relocated) — `crates/paladin-core/src/platform/container/autonomous_config.rs`, wired into `paladin_config.rs`. Later position: REQ-autonomous-completion-config-schema |
| REQ-autonomous-error-handling | Shipped — `src/application/errors/handoff_error.rs` and `crates/paladin-core/src/platform/container/arsenal/handoff_error.rs`; `PlanningError` / `PromptError` presence and the graceful-degradation paths not inspected → VERIFY-02 |

### Epic 15 — Conclave / Mixture-of-Agents (5 IDs)

Epic-level note: `intel/code-verification.md` records Conclave as **verified shipped** against
`tasks-conclave-mixture-of-agents.md`'s **129 open checkboxes** — the largest open concentration in
the entire corpus, and stale. Not carried as forward work.

| ID | Status |
|---|---|
| REQ-conclave-domain-model | Shipped (relocated) — `crates/paladin-core/src/platform/container/battalion/conclave.rs`. Note the completion report records the shipped status enum as `{Completed, PartialSuccess, Failed}` while the PRD specifies `Success`; PRD spelling retained in intel, discrepancy recorded → VERIFY-01 |
| REQ-conclave-execution-service | Shipped (relocated) — `crates/paladin-battalion/src/conclave_execution_service.rs` |
| REQ-conclave-commander-strategy | Shipped — referenced from `crates/paladin-battalion/src/commander.rs` and `battalion/mod.rs` per `code-verification.md` |
| REQ-conclave-cli-and-yaml | Shipped — `src/application/cli/commands/battalion.rs`; example `examples/conclave_expert_panel.rs` |
| REQ-conclave-observability | Verify — three observability levels are reported implemented with Standard as default; not inspected → VERIFY-01 |

### Epic 16 — Advanced Battalion Patterns: Council & Grove (11 IDs)

Epic-level note: `intel/code-verification.md` records both Council and Grove as **verified
shipped**. The release notes attribute them to Milestone 3 Epics 20 and 21; they are Milestone 2
Epic 16 → VERIFY-03.

| ID | Status |
|---|---|
| REQ-council-domain-model | Shipped (relocated) — `crates/paladin-core/src/platform/container/battalion/council.rs` |
| REQ-council-turn-strategies | Shipped — `TurnStrategy` in `council.rs` and `crates/paladin-battalion/src/council_service.rs`; PRD defers `Random` and `VoluntaryWithTimeout` (NG-6) |
| REQ-council-termination-conditions | Shipped — in `council.rs`. Note the release notes show `TerminationCondition::MaxRounds(3)` as a tuple variant against the PRD's unit variant → VERIFY-03 |
| REQ-council-execution-service | Shipped (relocated) — `crates/paladin-battalion/src/council_service.rs`; examples `examples/council_discussion.rs`, `examples/commander_council.rs`. Method name / result-field divergence (`convene` vs `execute`, `conclusion` vs `summary`) → VERIFY-03 |
| REQ-council-garrison-integration | Verify — not inspected → VERIFY-01 |
| REQ-grove-domain-model | Shipped (relocated) — `crates/paladin-core/src/platform/container/battalion/grove.rs` |
| REQ-grove-routing-strategies | Shipped — `RoutingStrategy` in `grove.rs` and `crates/paladin-battalion/src/grove_service.rs`. **`RoutingStrategy::PerformanceBased`, advertised by the release notes, is verified absent from the tree** and contradicts NG-3 → VERIFY-03. The `LlmRouting` path carries CLOSE-01 |
| REQ-grove-config-v1 | Variant (group 14) |
| REQ-grove-execution-service | Shipped (relocated) — `crates/paladin-battalion/src/grove_service.rs`; examples `examples/grove_routing.rs`, `examples/commander_grove.rs` |
| REQ-grove-arsenal-integration | Verify — SHOULD-level requirement; not inspected → VERIFY-01 |
| REQ-council-grove-commander-integration | Shipped — `tests/integration/commander_integration_tests.rs` covers both per `code-verification.md` |

### Epic 17 / 17.5 — Flow DSL, Maneuver and CLI consolidation (11 IDs)

Epic-level note: `intel/code-verification.md` records Maneuver / Flow DSL as **verified shipped**.
Epic 17.5's CLI-location decision is **applied in code** — `src/cli` is absent from the tree and
`src/application/cli/` carries the full command set. It remains at DOC precedence with no ADR.

| ID | Status |
|---|---|
| REQ-flow-dsl-syntax | Shipped (relocated) — `crates/paladin-battalion/src/maneuver/parser/` |
| REQ-flow-parser | Shipped (relocated) — `crates/paladin-battalion/src/maneuver/parser/` |
| REQ-flow-expression-ast | Shipped (relocated) — `crates/paladin-battalion/src/maneuver/parser/` |
| REQ-maneuver-domain-model | Shipped (relocated) — `crates/paladin-battalion/src/maneuver/mod.rs`. Constructor argument order differs between Epic 17 (`name, agents, flow`) and the release notes (`flow, paladins, config`) → VERIFY-03 |
| REQ-maneuver-config | Shipped (relocated) — `crates/paladin-battalion/src/maneuver/` |
| REQ-maneuver-error-strategy-v2 | Variant (group 15) |
| REQ-maneuver-execution-service | Shipped (relocated) — `crates/paladin-battalion/src/maneuver/service.rs`; examples `maneuver_basic.rs`, `maneuver_nested_flow.rs`, `maneuver_dynamic_flow.rs` |
| REQ-maneuver-commander-integration | Shipped — `crates/paladin-battalion/src/commander.rs` |
| REQ-maneuver-cli | Shipped, **namespace diverges** — the tree has a top-level `src/application/cli/commands/maneuver.rs` command group, matching the release notes' `paladin maneuver ...` rather than the PRD's `paladin battalion run --type maneuver` / `battalion visualize` → VERIFY-03 |
| REQ-flow-visualization | Shipped (relocated) — `crates/paladin-battalion/src/maneuver/visualizer.rs`, wired to `src/application/cli/commands/maneuver.rs` |
| REQ-maneuver-validation | Verify — construction-time validation not inspected → VERIFY-01 |

### Epic 18 — CLI Enhancement & Polish (7 IDs)

Epic-level note: `tasks-epic-18-cli-enhancement.md` carries 12 open checkboxes; every command
below exists in the tree, so the count is stale rather than a backlog. It is *not* one of the
three blocks flagged for VERIFY-02.

| ID | Status |
|---|---|
| REQ-cli-onboarding-wizard | Shipped (relocated) — `src/application/cli/commands/onboarding.rs` |
| REQ-cli-setup-check | Shipped (relocated) — `src/application/cli/commands/setup_check.rs` |
| REQ-cli-features-discovery | Shipped (relocated) — `src/application/cli/commands/features.rs` |
| REQ-cli-muster-command | Shipped (relocated) — `src/application/cli/commands/muster.rs` |
| REQ-cli-council-command | Shipped (relocated) — `src/application/cli/commands/council.rs` |
| REQ-cli-rich-output | Shipped — `src/application/cli/formatters/`; snapshot tests under `tests/cli/snapshots/`, `insta = "1.34"` in `Cargo.toml:152` |
| REQ-cli-core-infrastructure | Shipped — the PRD's module layout (`commands/`, `formatters/`, `interactive/`, `templates/`, entry point `src/bin/paladin-cli.rs`) matches the tree; the Epic 17.5 consolidation is applied |

### Epic 19 — Herald & Domain Type Consolidation (5 IDs)

| ID | Status |
|---|---|
| REQ-herald-type-consolidation | Shipped — no placeholder types and no `TODO` remain in `crates/paladin-core/src/platform/container/herald.rs`. Later position on run-1 variant group 6 |
| REQ-stream-chunk-complete | Verify — `StreamChunk` exists in the Herald surface; the six required fields, `#[serde(flatten)]` metadata and builder were not inspected → VERIFY-01 |
| REQ-execution-metadata-complete | Verify — `TokenUsage` ships at `crates/paladin-core/src/platform/container/token_usage.rs` and is referenced from `herald.rs:736`; the full `ExecutionMetadata` field set and `calculate_duration()` not inspected → VERIFY-01 |
| REQ-herald-formatter-autoregistration | Verify — `crates/paladin-herald/src/lib.rs` and `src/application/services/herald/`; `HeraldRegistry: Default` behaviour not inspected → VERIFY-01 |
| REQ-herald-consolidation-quality-gates | Verify — the ≥ 95% module coverage gate has no recorded measurement → VERIFY-05 |

### Epic 21 — Autonomous Agent Completion (7 IDs)

| ID | Status |
|---|---|
| REQ-handoff-tool-v2 | Variant (group 13) |
| REQ-autonomous-configurable-model | Shipped for `planning_service.rs` and `prompt_generation_service.rs` per Epic 21; **the same defect class survives in Grove** → CLOSE-01 (`grove_service.rs:537`) |
| REQ-paladin-result-autonomous-metadata | Verify — `TaskPlan` lives in `crates/paladin-core/src/platform/container/planning.rs` and `HandoffRecord` in `container/handoff.rs`; the `PaladinResult` field defaults, MessagePack support and the 100-record cap not inspected → VERIFY-01 |
| REQ-autonomous-orchestration-layers | Verify — the four-layer flow and per-layer config flags in `paladin_execution_service.rs` were not inspected → VERIFY-01 |
| REQ-handoff-execution-integration | Verify — `handoff_service.rs` and `tests/unit/handoff_service_test.rs` exist; retry policy, cycle detection at depth and the `E-HANDOFF-00x` error codes not inspected → VERIFY-01 |
| REQ-autonomous-completion-config-schema | Shipped (relocated) — `crates/paladin-core/src/platform/container/autonomous_config.rs`. Later position on REQ-autonomous-configuration |
| REQ-autonomous-completion-quality-gates | Verify — the ≥ 90% autonomous-component coverage gate has no recorded measurement → VERIFY-05. "Zero remaining TODO comments in autonomous agent code" is contradicted by CLOSE-01 if Grove counts as in scope |

### Epic 22 — Battalion & Commander Hardening (10 IDs)

Epic-level note: `tasks-epic22-battalion-commander-hardening.md` carries **81 open checkboxes** and
is the run-2 block with the strongest corroboration — `codebase/CONCERNS.md` records a real open
TODO in its scope. It is one of the three blocks flagged for VERIFY-02.

| ID | Status |
|---|---|
| REQ-paladin-registry-port | Shipped (relocated) — `crates/paladin-ports/src/output/paladin_registry.rs`; `RegistryError` at `crates/paladin-core/src/platform/container/registry_error.rs` |
| REQ-paladin-registry-adapter | Shipped (relocated) — `crates/paladin-battalion/src/in_memory_registry.rs` (the PRD specified `HashMapPaladinRegistry` under `infrastructure/adapters/`) |
| REQ-council-grove-registry-resolution | Verify — registry port and adapter both ship and both services exist; whether `CouncilService` / `GroveService` take `Arc<dyn PaladinRegistry>` and return `BattalionError::PaladinNotFound` was not inspected → VERIFY-02 |
| REQ-grove-llm-routing | **Partial → CLOSE-01** — `GroveService` LLM routing is implemented (prompt construction and `llm_port.generate()` at `crates/paladin-battalion/src/grove_service.rs:520-545`), but the request hardcodes `model: "gpt-4".to_string(), // TODO: Make configurable` at `:537` in production code. Verified open |
| REQ-phalanx-per-paladin-metrics | Shipped — `crates/paladin-battalion/src/phalanx_service.rs`; metadata fields present, see below |
| REQ-battalion-metadata-extension | Shipped (relocated) — `per_paladin_times: HashMap<String, u64>` at `crates/paladin-core/src/platform/container/battalion/mod.rs:582` and `per_paladin_tokens: HashMap<String, TokenUsage>` at `:585`; the PRD placed these in `battalion/battalion_result.rs`. Variant (group 4) |
| REQ-commander-metadata-export | Shipped — export routine at `crates/paladin-battalion/src/commander.rs:870`; the `<strategy>_<timestamp>_<uuid>.json` naming, sanitised `config_snapshot` and < 50 ms budget not inspected → VERIFY-02 |
| REQ-commander-config-metadata-dir-v3 | Variant (group 3) — shipped location is `BattalionConfig` (`battalion/mod.rs:54`, writability check `:116-124`), **not** `CommanderConfig` |
| REQ-commander-test-hardening | **Partial → VERIFY-02** — four `#[ignore]` attributes remain in `crates/paladin-battalion/src/commander.rs`; the PRD requires all six named tests enabled. `MockLlmAdapter` exists (`tests/helpers/`) |
| REQ-grove-config-v2 | Variant (group 14) |

### Epic 23 — CLI, Config & Infrastructure Completion (10 IDs)

Epic-level note: Epic 23 is the **most reliably complete epic in the run-2 corpus** — the
Milestone 3 plan was edited in place to mark it COMPLETE while Epics 19-22 and 24 remain
unchecked, and `tasks-task46-arsenal-tool-integration-tests.md` has 1 open item, the lowest in the
milestone.

| ID | Status |
|---|---|
| REQ-cli-garrison-configuration | Shipped — `src/application/cli/commands/agent.rs` (the `:293` TODO is reported removed); garrison config at `crates/paladin-memory/src/config/garrison.rs` |
| REQ-cli-arsenal-configuration | Shipped — `src/application/cli/commands/agent.rs` (the `:296` TODO is reported removed). Known limitation recorded by the epic: MCP WebSocket transport not implemented |
| REQ-mock-llm-adapter | Shipped — `MockLlmAdapter` in `tests/helpers/`; also `crates/paladin-llm/src/mock.rs` |
| REQ-cli-tiered-environment-testing | Shipped — Tier-1 suites under `tests/cli/`; Tier 2 Docker-gated and Tier 3 API-key-gated. **Tier-3 skip semantics are contested** → VERIFY-06 |
| REQ-scheduler-port | Shipped (relocated), **version diverges** — `crates/paladin-ports/src/output/scheduler_port.rs` and adapter `crates/paladin-storage/src/scheduler.rs` on `tokio-cron-scheduler` **0.13**; the PRD pinned 0.9 and specified `get_job_status` where the delivered trait has `list_jobs` / `get_job_info` |
| REQ-content-deliverer-scheduling | Shipped — `schedule_delivery` in `crates/paladin-content/src/services/content_delivery_service.rs`; the `unimplemented!("Scheduler integration pending")` stub is gone. Tree check: the remaining `unimplemented!` occurrences in `crates/` are all inside `#[cfg(test)]` mocks or doc comments, so the "zero `unimplemented!()` in production paths" criterion holds |
| REQ-cli-error-types | Verify — `src/application/cli/error.rs` exists; the three `#[from]` variants not inspected → VERIFY-01 |
| REQ-mock-arsenal-port | Shipped — `tests/helpers/mock_arsenal_adapter.rs`, registered in `tests/helpers/mod.rs` |
| REQ-tool-call-loop-tests | Shipped — `tests/cli/tool_integration_test.rs`; the eight named test cases not individually inspected → VERIFY-01 |
| REQ-mcp-gated-integration-tests | Verify — gated MCP STDIO tests and `tests/mcp_test_server.py` not inspected → VERIFY-01 |

### Epic 24 — Test Hardening, Benchmarks & QA (9 IDs)

Epic-level note: `tasks-test-hardening-benchmarks-qa.md` carries **29 open checkboxes** and is one
of the three blocks flagged for VERIFY-02.

| ID | Status |
|---|---|
| REQ-battalion-benchmark-repair | Shipped (relocated) — `crates/paladin-battalion/benches/battalion_benchmarks.rs`, plus `crates/paladin-memory/benches/{garrison,sanctum}_benchmarks.rs` and `crates/paladin-llm/benches/llm_serialization_benchmarks.rs`. `benches/BENCHMARK_FIXES.md` documents the original compilation failures. Whether `cargo bench --no-run` is in CI, and whether `docs/BATTALION_BENCHMARKS.md` exists, not inspected → VERIFY-02, QUAL-05 |
| REQ-prompt-generation-test-reenable | Verify — `tests/unit/prompt_generation_service_test.rs` exists as a file; whether the module is uncommented in `tests/unit/mod.rs` not inspected → VERIFY-02 |
| REQ-timeout-test-hardening | Verify — not inspected → VERIFY-02 |
| REQ-qdrant-integration-tests | Verify — integration target `qdrant_sanctum_integration` exists per `code-verification.md`; `tests/integration/rag_integration_tests.rs` placeholder replacement not inspected → VERIFY-02 |
| REQ-deferred-coverage-review | Verify — `user_service.rs` (4.23%) and `listener_service.rs` (57.83%); `project/DEFERRED_COVERAGE.md` is named by the release notes. The release notes also list these as Milestone-4 "Epic 28 / Epic 29" work → VERIFY-02, VERIFY-05 |
| REQ-cli-snapshot-testing | Shipped — `insta = "1.34"` at `Cargo.toml:152`, `tests/cli/snapshots/` present. The ≥ 10 snapshot count and the QUICKSTART / INSTALLATION / `docs/cli/README.md` updates not inspected → VERIFY-02 |
| REQ-provider-live-api-tests | Shipped, **semantics contested** — `tests/integration/llm_live_api_tests.rs` exists behind `live-api-tests`; the post-Epic-24 cleanup replaced graceful skip with a panic, reversing the PRD criterion → VERIFY-06, CLOSE-03 |
| REQ-final-documentation-and-demo | Verify — the release notes name `docs/MANEUVER.md`, `docs/COMMANDER.md`, `docs/cli/TESTING.md` and `CONTRIBUTING.md`; the demo asset and the CLI-specific CI job not inspected → VERIFY-02. Overlaps REL-04 |
| REQ-epic24-quality-gates | Verify — "no ignored tests remaining" is **contradicted** by the four `#[ignore]` attributes in `commander.rs`; the coverage clauses feed VERIFY-05 |

---

## v2 Requirements

Acknowledged, deferred, not in the current roadmap. Some of these may acquire a real requirement
when ingest runs 3-5 land Milestones 4-12, Deferred-QA-CICD-Completion and project-management.

### Testing

- **Live-provider-API integration tests** — Epic 6 task 7.0 (18 subtasks), deferred at Milestone 1,
  un-deferred by Epic 24 (REQ-provider-live-api-tests) and shipped behind `live-api-tests`. What
  remains open is the skip-vs-fail semantics (VERIFY-06), not the suite.
- **CLI end-to-end tests** — Epic 9 tasks 13.4-13.6, originally blocked on CLI mock-provider
  support. Run 2 supplied the mock provider (REQ-mock-llm-adapter) and the Tier-1 suites, so the
  original blocker is gone.
- **Garrison large-conversation performance test** — Epic 2 task 9.14 (1,000 entries).
- **Bearer-token redaction enforcement test** — prove `BearerToken` cannot be logged
  (`codebase/CONCERNS.md`).
- **Vision end-to-end latency measurement** — REQ-vision-performance-and-config and
  REQ-rag-performance-targets state numeric targets (single image < 5 s, retrieval < 500 ms p95,
  extraction < 3 s p95) with no recorded measurement anywhere in `.planning/`.

### Tech debt (from `codebase/CONCERNS.md`, no ingested requirement)

- Decompose the three oversized service files: `paladin_execution_service.rs` (2,757 lines),
  `paladin_builder.rs` (2,294), `orchestration/mod.rs` (1,840).
- Reduce the 383 `.clone()` calls and the 9-lock orchestrator contention — needs the Phase 3
  benchmark baselines first.
- Replace `structopt` with clap v4 and `dotenv` with `dotenvy`; upgrade `utoipa` off `paste`;
  converge the dual `reqwest` 0.12/0.13 dependency.
- Orchestrator state durability: workflow checkpointing, resume-on-startup, enforced queue
  persistence in production mode.
- Single-threaded orchestration scheduler in `src/application/services/orchestration/scheduler.rs`
  — `codebase/CONCERNS.md` recommends leaning on `tokio-cron-scheduler`, already a dependency and
  already adapted in `crates/paladin-storage/src/scheduler.rs` (REQ-scheduler-port).
- Environment isolation for `system_log_integration_test.rs`.

### Unimplemented code paths (requirement pending later ingest runs)

- Notification adapter wiring (`service_runner.rs:534`) — notifications register but never deliver.
- Media content handlers — video/audio/image stubs in `file_content_fetcher.rs:105-115`.
- Trigger payload conditions (JSONPath) and cooldown checks — `trigger.rs:216, 261`.
- MCP config validation coupling — schema accepts server types the client cannot construct.
- MCP WebSocket transport — recorded as a known limitation by the Epic 23 completion summary.
- *(Removed from this list by run 2: Grove service hardcoded `model: "gpt-4"` — now a first-class
  forward requirement, CLOSE-01, verified at `grove_service.rs:537`.)*

### Deliberately not carried forward from run 2

Recorded so a later reader does not mistake omission for oversight.

- **The 155 open checkboxes in Epics 14, 22 and 24** are not requirements. They are a verification
  task (VERIFY-02). Run 1 and run 2 both proved checkbox state understates shipped reality.
- **The 240 open checkboxes in Epics 11 and 15** (Sanctum 111, Conclave 129) are not carried at
  all — `intel/code-verification.md` verifies both features shipped.
- **Milestone-4 work named by the release notes** (`Epic 28` user_service coverage, `Epic 29`
  listener_service coverage, Grove semantic-similarity routing, RAG integration) is not carried:
  the release-notes forward-look is stale, RAG and Grove semantic routing already ship, and
  Milestone 4 itself arrives in ingest run 3.
- **`RoutingStrategy::PerformanceBased`** is not carried as a requirement — it is verified absent
  from the tree and contradicts Epic 16 NG-3. Correcting the claim is part of VERIFY-03.
- **Epic 17.5's CLI-location decision** is not carried as a requirement — it is already applied in
  code (`src/cli` absent, `src/application/cli/` populated). It is noted in PROJECT.md Context as
  the corpus's strongest ADR candidate should the user want to protect it.

### Awaiting ingest (runs 3-5 of 5)

- **Run 3:** Milestones 4-6 (crate/feature refactor, workspace decomposition, architectural
  refinements). This run supplies the requirements behind the nine-crate layout that makes every
  run-1 and run-2 `src/...` path historical.
- **Run 4:** Milestones 7-8 (production hardening, facade cleanup and shim resolution).
- **Run 5:** Milestones 9-12 (classic orchestrator completion, CI hardening and release
  automation, documentation overhaul, Web API) plus Deferred-QA-CICD-Completion and
  project-management. `intel/code-verification.md` names the Deferred-QA and Milestone-8 deferred
  documents as where the genuine remaining-work signal lives.

Shipped code with **no ingested requirement yet**: the Axum HTTP API (auth, rate limiting,
OpenAPI, SSE streaming), notifications, the content ingestion pipeline, and the nine-crate
workspace structure itself.

---

## Out of Scope

| Feature | Reason |
|---|---|
| Re-implementing shipped Milestone-1/2/3 work | The ledgers record it; the roadmap does not re-plan it |
| Picking a winner for the 16 competing variant groups inside this document | Deliberate, and explicitly requested: shipped code is the arbiter and the decision belongs in an ADR (RECON-02 … RECON-07, VERIFY-03 … VERIFY-06), not in an ingest artefact |
| Synthesizing locked decisions from PRD/DOC assertions | 0 ADR-typed and 0 SPEC-typed docs exist across all 81 documents; asserting locks would fabricate authority |
| Converting open checkbox counts into requirements | 542 open items, and the two largest blocks are verified shipped. Verification first (RECON-01, VERIFY-02) |
| Migrating between the two shipped vision surfaces | Both ship; `code-verification.md` explicitly says confirm intent first (VERIFY-04) |
| Live LLM API calls in the default test run | Cost, flakiness, secret handling; feature-gated only |
| JSON or TOML CLI configuration | Epic 9 NG-4 — YAML only |
| Encrypted config files, keychain/secret-manager integration | Epic 9 NG-2, NG-10 — env vars only |
| Grove learning from past routing decisions | Epic 16 NG-3; the release-notes `PerformanceBased` claim is verified absent from the tree |
| Automatic Garrison-to-Sanctum migration | Epic 11 explicit non-goal |
| Batch vision API | Epic 20 NG-6 — concurrency is a Battalion concern |
| Registry multi-tenancy, persistence, distribution | Epic 22 explicit non-goals |
| Non-Rust client SDKs | The product *is* a Rust library surface |
| Milestone 4-12 feature work | Awaiting ingest runs 3-5; planning it now would guess at requirements that exist on disk |

---

## Traceability

Forward (v1) requirements only. Shipped requirements are tracked in the two ledgers above.

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
| VERIFY-01 | Phase 5 | Pending |
| VERIFY-02 | Phase 5 | Pending |
| VERIFY-03 | Phase 5 | Pending |
| VERIFY-04 | Phase 5 | Pending |
| VERIFY-05 | Phase 5 | Pending |
| VERIFY-06 | Phase 5 | Pending |
| CLOSE-01 | Phase 6 | Pending |
| CLOSE-02 | Phase 6 | Pending |
| CLOSE-03 | Phase 6 | Pending |

**Coverage:**
- v1 requirements: 34 total (25 Milestone-1 close-out + 9 Milestone 2-3 close-out)
- Mapped to phases: 34
- Unmapped: 0 ✓
- Duplicated across phases: 0 ✓

**Ledger coverage:**
- Requirement IDs enumerated from `intel/requirements.md`: 233 (run 1: 115, run 2: 118)
- Recorded in the ledgers: 233 ✓ — 115 in the Milestone 1 ledger, 118 in the Milestone 2-3 ledger
- Competing-variant entries preserved unmerged: 30 across 16 groups (12 from run 1, 18 from run 2)
- Bookkeeping notes:
  - `intel/SYNTHESIS.md` reports 107 requirements for run 1 and its per-PRD table sums to a third
    figure. The enumerated count (115) is authoritative here; reconciling the arithmetic is
    RECON-01.
  - `intel/SYNTHESIS.md`'s run-2 per-PRD table sums to 116 against the 118 IDs actually present.
    The two-entry difference is attribution: `REQ-council-grove-commander-integration` and
    `REQ-maneuver-validation` are counted into adjacent epic groups. The enumerated count (118) is
    authoritative here.
  - SYNTHESIS reports 30 cumulative variant entries; INGEST-CONFLICTS counts 26 warnings. Both are
    correct — several variant groups carry three entries under one warning.

---
*Requirements defined: 2026-07-30*
*Last updated: 2026-07-30 after ingest run 2 of 5 (`.project/Milestone_2-Missing_features` +
`.project/Milestone_3-Completion`, 45 docs). Runs 3-5 append; `REQ-*` IDs are the merge keys.*
