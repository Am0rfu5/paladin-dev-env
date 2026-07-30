# Requirements: Paladin

**Defined:** 2026-07-30 (ingest run 1 of 5 — source set `.project/Milestone_1-MVP`, 36 docs)
**Last merge:** 2026-07-30 (ingest run 4 of 5 — `.project/Milestone_7-Production-Hardening` +
`.project/Milestone_8-Facade-Cleanup-Shim-Resolution`, 40 docs)
**Core Value:** A Rust developer can compose and run multi-agent workflows against any supported
LLM provider through stable port abstractions — without their own domain code depending on a
provider, transport, or storage implementation.

## How to read this file

Paladin is **brownfield at v0.7.0**. Across the 153 documents ingested so far, 7,511 of 8,053
task-list items are checked (93%) — and the shipped tree is *ahead of* even that figure in
several places. So this file separates several different things that are all called
"requirements":

| Section | What it holds |
|---|---|
| **v1 Requirements** | Forward scope for the current milestone. Milestone 1 close-out: 25 requirements (Phases 1-4). Milestone 2-3 close-out: 9 requirements (Phases 5-6). Milestone 4-6 close-out: 12 requirements (Phases 7-8). Milestone 7-8 close-out: 16 requirements (Phases 9-11). Each mapped to exactly one phase. |
| **Competing variants** | 28 variant groups / 56 entries preserved **unmerged** from conflicting PRDs across runs 1-4. No winner is picked here. |
| **Milestone 1 as-shipped ledger** | All 115 run-1 requirement IDs, with status. Not forward scope. |
| **Milestone 2-3 as-shipped ledger** | All 118 run-2 requirement IDs, with status. Not forward scope. |
| **Milestone 4-6 as-shipped ledger** | All 115 run-3 requirement IDs, with status. Not forward scope. |
| **Milestone 7-8 as-shipped ledger** | All 86 run-4 requirement IDs, with status. Not forward scope. |
| **v2 Requirements** | Acknowledged and deferred. Not in the current roadmap. |

**Two unrelated uses of "v1/v2" — do not confuse them:**

- `## v1 Requirements` / `## v2 Requirements` = **release scope** (current milestone vs deferred).
- An ID *suffix* like `REQ-temperature-range-v1` / `-v2` / `-v3` = **competing variant** of the
  same scope from two or three different source PRDs. All are live; none has been chosen.

**ID provenance.** `RECON-*`, `GAP-*`, `QUAL-*`, `REL-*` are Milestone-1 close-out IDs;
`VERIFY-*` and `CLOSE-*` are Milestone 2-3 close-out IDs; `ARCH-*` and `DEBT-*` are Milestone 4-6
close-out IDs; `SEC-*`, `HARD-*` and `FACADE-*` are Milestone 7-8 close-out IDs. Each cites the
ingested `REQ-*` IDs it derives from. `REQ-*` IDs are preserved verbatim from
`.planning/intel/requirements.md` so ingest run 5 can merge against stable keys. **All eleven
prefixes are now spent**; run 5 introduces new ones.

**Ledger verdicts are component-level file evidence, not per-criterion audits.** A `Shipped`
verdict means the named artefact exists in the tree at the cited path. Confirming each PRD
acceptance criterion against that artefact is forward work (RECON-01 / VERIFY-01 / ARCH-01 /
HARD-01), not something this file claims to have done. The Milestone 7-8 ledger is now the
best-evidenced of the four: 18 verified-shipped rows, 6 verified-open items, a 14-row
superseded-by-outcome table and 2 favourable contradictions were checked directly against
`Cargo.toml`, `deny.toml`, `.cargo/audit.toml`, workflow contents and grep counts during ingest
run 4, and are recorded in `intel/code-verification.md`. Run 4 is also the first run whose corpus
contains a document that **audits itself against the tree** —
`facade-cleanup-RECONCILIATION-2026-06-04.md` — and its claims are corroborated almost without
exception.

**Open checkbox counts are not a backlog.** `intel/task-completion-state.md` records 542 open
items; `intel/code-verification.md` proves the two largest concentrations (Conclave 129,
Sanctum 111) are **shipped**. Only the blocks that `code-verification.md` explicitly lists as
"Not yet verified" are carried here, and they are labelled *Unverified candidate* — never
converted into requirements.

**Nothing is locked.** 0 ADR-typed and 0 SPEC-typed documents exist across all 153 ingested
docs. Every `REQ-*` acceptance criterion sits at PRD or DOC precedence and is supersedable —
including by the shipped code, which is the real arbiter wherever the two disagree. Precedence
order for this project: **shipped tree > `.planning/codebase/` map > `intel/code-verification.md`
> PRD > DOC > task-list checkbox.**

**Run 3 found the corpus's only decision record — and it is still not a locked decision.**
`Milestone_5/Epic_1/decisions/battalion-result-upward-dependency-decision.md` and its
`-options.md` companion carry `Status: Approved`, `Decision Date: 2026-05-13`,
`Chosen Option: Option A`, a Rationale and a Rejected Options section. Both are manifest-typed
DOC with `locked: false`, so they sit at the lowest precedence tier and create no locked decision.
What the record settles is the **location** of five pure value/error types (`PaladinResult`,
`StopReason`, `TokenUsage`, `RegistryError`, `HandoffError`) moving into `paladin-core`, with the
application ports reduced to thin re-exports and `PaladinError` deliberately excluded. **It never
mentions `BattalionResult`**, despite the filename — the run-1 `BattalionResult` field-set variant
is untouched by it and is closed by shipped code instead (see variant group 4). A PRD published
two days later contradicts it; that is variant group 19.

**Relocation is not contradiction.** Milestones 4-6 deliberately restructured what Milestones 1-3
built, so every `src/core|application|infrastructure` path recorded in runs 1-2 — and several
recorded in run 3 — is historical. Supersession chains are recorded in *Superseded but preserved*
below; they are not defects.

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
      (`battalion/mod.rs` and `citadel.rs`). **Narrowed by ingest run 3:** the *variant choice* is
      settled by shipped code — `battalion/mod.rs:37` is the Epic 4 field set exactly, and
      `CommanderConfig` does not exist anywhere in `crates/` or `src/`, so the v3 position was
      never built. What remains for the ADR is recording that verdict and resolving the
      `citadel.rs` duplicate, which run-3 verification did not address.
      *Resolves: REQ-battalion-config-v1 / -v2 / REQ-commander-config-metadata-dir-v3 (see variant
      group 3).*
- [ ] **RECON-03**: `BattalionResult` has exactly one authoritative definition, recorded as an ADR,
      that simultaneously satisfies its four producers (Formation, Phalanx, Campaign, Chain of
      Command) and its consumer (Herald — which needs a Battalion type field and aggregated token
      usage that neither source variant provides). **Narrowed by ingest run 3:** the shipped struct
      at `crates/paladin-core/src/platform/container/battalion/mod.rs:549` is a merged **superset**
      of all three positions, so this is now a recording task, not a reconciliation task —
      `intel/code-verification.md` says explicitly not to plan the latter. The ADR records what the
      superset chose: `per_paladin_times` in place of Epic 5's `execution_time_ms`, and
      `node_errors: Vec<NodeError>` in place of `errors: Vec<PaladinError>` (because
      `BattalionError` does not derive `Serialize`/`Deserialize` while `BattalionResult` does).
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
- [ ] **GAP-07**: The reconciled definitions from Phase 1 are applied in code: agreed
      minimum-Paladin behaviour (a single-Paladin Commander in Auto mode executes instead of
      failing validation), the recorded temperature rule, the recorded `Herald` trait signature,
      and the duplicate `BattalionConfig` in `citadel.rs` resolved. **Narrowed by ingest run 3:**
      "one `BattalionResult`" and "one `BattalionConfig` field set" are already true in shipped
      code and are dropped from this requirement — the `citadel.rs` duplicate is what survives.
      *Derives: RECON-02 … RECON-06; `intel/code-verification.md` run-3 resolved variants.*

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
      `cargo build --workspace` succeeds under it. **Exact state verified 2026-07-30:** the root
      `paladin-ai` package and nine crates — `paladin-core`, `paladin-battalion`, `paladin-herald`,
      `paladin-llm`, `paladin-memory`, `paladin-storage`, `paladin-content`, `paladin-web` and
      `doc-examples` — declare `edition = "2024"`; exactly two, `crates/paladin-ports` and
      `crates/paladin-notifications`, declare `"2021"`. The documented
      answer is contested (variant group 17) and is recorded in ARCH-03(a); REL-02 is the code fix.
      Whichever of Phase 4 / Phase 7 executes first records the answer, the other applies it.
      *Derives: `codebase/CONCERNS.md` tech debt; REQ-workspace-crate-edition-v1 / -v2 (run 3).*
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
      run-2 PRDs predates the workspace decomposition** (Milestone 5, ingested in run 3 — see the
      *Milestone 4-6 as-shipped ledger* for the current crate layout), and
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

## v1 Requirements — Milestone 4-6 close-out (Phases 7-8)

Added by ingest run 3. These three milestones **restructured what Milestones 1-3 built**:
Milestone 4 expanded the feature-flag surface and hardened the port traits, Milestone 5 decomposed
the monolith into a Cargo workspace, Milestone 6 relocated four layer-violating module groups.
Almost all of it shipped, and unusually for this corpus it was verified directly: the workspace
manifest, all named crates, the prelude, the per-crate feature matrices, the `crate-isolation` CI
job, the `application_settings.rs` deletion and its per-domain replacement, the orchestration
relocation, the Maneuver co-location and the `CircuitBreaker` move are each confirmed against the
tree in `intel/code-verification.md`.

**Run 3 is also the first run where an open-checkbox count proved trustworthy.** Milestone 4's 20
open items — all in `tasks-harden-port-traits-stable-api.md` — are corroborated by code: zero
`#[deprecated]` annotations exist anywhere. Milestone 6's 0 open items are corroborated: all four
relocations are complete. Milestone 5's 17 open items are mostly contradicted; the crates, the
prelude, the CI job and the benchmark report all exist.

Forward scope is therefore narrow and concrete: record what shipped and what the four competing
variant pairs resolved to (ARCH), and close the five defects verification proved open (DEBT).
**No forward phase re-plans the workspace extraction or the Milestone 6 relocations** — they
shipped.

### Ground truth (ARCH)

- [ ] **ARCH-01**: The *Milestone 4-6 as-shipped ledger* below is upgraded from component-level
      file evidence to per-criterion verdicts with `file:line` citations, for all 115 run-3
      requirement IDs. Must record the corrected workspace shape: **ten library crates**
      (`paladin-core`, `paladin-ports`, `paladin-battalion`, `paladin-herald`, `paladin-llm`,
      `paladin-memory`, `paladin-storage`, `paladin-notifications`, `paladin-content`,
      `paladin-web`) plus a `doc-examples` crate plus the root facade package `paladin-ai` —
      not the six the Milestone 5/6 overviews and `build-benchmarks.md` assume, and not the
      "9-crate workspace" figure this planning set carried before run 3. **Narrowed by ingest
      run 4:** the provenance of all five extra crates is now supplied and no longer pending —
      `paladin-storage`, `paladin-notifications`, `paladin-content` and `paladin-web` from
      Milestone 7 Epic 1's extraction PRD and its cost-benefit gate, and `paladin-herald` from the
      2026-06-04 reconciliation (commit `66f6c4e`) rather than from any PRD, which is why no
      ingested requirement described it before run 4. HARD-01 records the detail; ARCH-01 keeps the
      workspace shape. Same for `crates/paladin-battalion/src/in_memory_registry.rs`,
      which is absent from Epic 3's 13-file extraction inventory and appears in Epic 3 of
      Milestone 6 as an existing file.
      *Derives: `intel/code-verification.md` run-3 section (22 verified-shipped claims, 12-row
      contradiction table); INGEST-CONFLICTS run-3 INFO on `in_memory_registry.rs`.*
- [ ] **ARCH-02**: The milestone/tier numbering collision is recorded once and corrected at its
      source. The Milestone 4 overview is titled "Milestone 1: High-Value, Low-Risk Foundations",
      the Milestone 5 overview "Milestone 2: Workspace Decomposition", the Milestone 6 overview
      lists its prerequisites as "Completed in Milestones 1 and 2", and PRDs cross-reference the
      port hardening as "Milestone 1 / Epic 2" meaning Milestone 4 Epic 2. The authoritative
      numbering for GSD is the **directory / task-list numbering** (4 = Tier 1, 5 = Tier 2,
      6 = Tier 3); every "Milestone 1/2/3" reference *inside* these three milestones is a tier
      label. This is the second numbering defect in the corpus — VERIFY-03 fixes the first — and
      both must be fixed with the same convention so `REQ-*` provenance keys resolve.
      *Derives: INGEST-CONFLICTS run-3 warning 1; affects 9 of the 19 run-3 DOCs.*
- [ ] **ARCH-03**: Each of the four run-3 competing variant pairs has exactly one recorded answer,
      citing the shipped code that settles it and stating whether the documents are amended or the
      code is accepted as the resolution. All four are settled *in code* — which is unusual for
      this corpus — but three of the four PRDs are unamended and would produce the wrong answer if
      applied literally to future work:
      **(a) Rust edition** (group 17) — the answer feeds REL-02, which is the code fix;
      **(b) `paladin-core` dependency allowlist** (group 18) — the allowlist is the enforcement
      mechanism for the whole hexagonal-purity argument (M5 Epic 1 FR-24, FR-25, SM-4), so leaving
      it wrong by eight crates makes it unenforceable as written; the same drift applies to
      `paladin-ports`, declared at 7 dependencies and shipping 10;
      **(c) port value-type ownership** (group 19) — the one place where mechanical precedence
      produces an architecturally wrong answer, because a PRD outranks an Approved-status decision
      record; the answer must state whether Epic 2's FR-7/FR-10 are amended or the decision record
      is promoted to a real ADR via `--manifest`;
      **(d) LLM config bridge location** (group 20) — the answer must state whether Epic 4's
      circular-dependency concern was real, since Milestone 6 moved the config into `paladin-llm`
      anyway.
      *Derives: REQ-workspace-crate-edition-v1/-v2, REQ-paladin-core-dependency-allowlist-v1/-v2,
      REQ-port-value-type-ownership-v1/-v2, REQ-llm-config-bridge-location-v1/-v2;
      INGEST-CONFLICTS run-3 warnings 2-5.*
- [ ] **ARCH-04**: The Milestone 6 facade re-export policy has one recorded answer and its version
      consequence is recorded. The Milestone 6 overview requires the facade to re-export relocated
      types at their original paths for backward compatibility (Epic 2 AC 6, Epic 4 AC 5, and the
      risk register's "facade crate re-exports absorb the change"); both PRDs require the
      opposite (Epic 2 Non-Goal 7 "no shim re-exports are added", Epic 4 §4.11 "the old path is
      intentionally retired"). Shipped code follows the PRDs — `src/application/use_cases/` no
      longer exists at all. The recorded answer must state whether Milestone 6 therefore
      constitutes a **breaking change requiring a major version bump**, which is a direct input to
      REL-01. Epic 2's own Open Question 4 undercuts its Non-Goal ("this should be confirmed with
      the team before implementation begins"), so this is an open decision rather than a
      precedence conflict.
      *Derives: REQ-orchestration-no-reexport-shims, REQ-circuitbreaker-old-path-retired,
      REQ-battalion-facade-shim (which took the opposite posture one milestone earlier);
      INGEST-CONFLICTS run-3 warning 7.*
- [ ] **ARCH-05**: The five documented positions that shipped code contradicts are corrected at
      source, so no later work applies them literally. All five verified:
      1. `vision` gating `chacha20poly1305` and `zeroize` — the Epic 1 `dependency-matrix.md`
         audit classifies both as general-purpose deps of `security/encryption.rs` (user auth,
         Citadel) that must stay unconditional; shipped `vision = []` gates no dependency at all.
         Applying the PRD literally would break `cargo build --no-default-features`.
      2. MCP transport feature flags (`mcp-transports` / `mcp-stdio` / `mcp-sse`) — none exists;
         the PRD's dated 2026-04-15 elimination note is what shipped.
      3. `web-server` gating both `actix-web` and `axum` — shipped
         `web-server = ["dep:paladin-web", "dep:axum"]`; actix-web is no longer a root dependency.
      4. A `paladin-cli` workspace crate — never built; the CLI is a `cli` feature plus
         `[[bin]] paladin-cli` with `required-features`. M5 Epic 6's non-goal was correct and the
         overview's target structure was not.
      5. The Milestone 6 Epic 2 target directory `src/application/use_cases/` — the four
         orchestrator modules ship under `src/application/services/` with the PRD's exact module
         names, and `use_cases/` no longer exists.
      Must also record that `STABLE_API.md`, `docs/FEATURE_FLAGS.md`, `docs/MIGRATION.md` and
      `docs/CONFIGURATION.md` are **relocated into the mdbook**
      (`docs/src/api-reference/{stable-api,feature-flags,migration-guide,crate-map}.md`,
      `docs/src/getting-started/installation.md`), not missing — so Milestone 6 Epic 4's FR-4.12
      now applies to `docs/src/api-reference/stable-api.md` and nobody plans them as gaps.
      *Derives: REQ-vision-feature-gating, REQ-feature-flag-matrix, REQ-feature-flag-docs,
      REQ-orchestration-target-structure, REQ-circuitbreaker-stable-api-update;
      `intel/code-verification.md` "Crate-level facts that contradict run-3 requirement text".*
- [ ] **ARCH-06**: The Milestone 4 Epic 3 binary-target architecture question is answered and
      documented, closing FR9.3's never-produced deliverable. Q1 recorded "User selected Option D —
      requires architecture review" and made it a blocker for Task 3.3; no architecture-review
      record exists anywhere in the ingest set. The tree answers it de facto with **three** binary
      targets — `paladin` (`src/main.rs`), `paladin-cli` (`required-features = ["cli"]`) and
      `paladin-server` (`required-features = ["web-server"]`) — i.e. Option A extended. The
      recorded answer must state each binary's intended use case, which is what FR3 asked for.
      *Derives: REQ-binary-target-config, REQ-cli-docs; INGEST-CONFLICTS run-3 INFO.*
- [ ] **ARCH-07**: The build-time benchmark record is made falsifiable.
      `Epic_6/build-benchmarks.md` marks four of five scenarios "Meets ≥ 50% target? No" (clean
      build −6.6%, core incremental −18.9%, llm incremental −44.6%) and two "Yes" (memory −50.2%,
      battalion-only −90.9%), then concludes "**Overall verdict: Target achieved**"; it also states
      the clean-build shortfall as −6.6% in its table and −5% in its conclusion. Its own
      methodology note concedes the monolith baseline was measured by touching `src/lib.rs`,
      "primarily a module-tree re-export file", giving "the *best-case* monolith incremental".
      Either re-run against the fairer mid-tree baseline the report itself recommends, or restate
      Milestone 5 SM-7 as a per-scenario target — so the ≥ 50% figure can be judged pass or fail.
      Distinct from QUAL-05, which covers **runtime** benchmarks; this one is build time.
      *Derives: REQ-build-benchmark-report; INGEST-CONFLICTS run-3 warning 13.*

### Verified defect closure (DEBT)

Unlike the run-1 and run-2 open-checkbox blocks, every item below was confirmed against the
shipped tree during ingest run 3 and re-confirmed on 2026-07-30. Each has a small, concrete fix.

- [ ] **DEBT-01**: The `api-surface` CI job works. `scripts/check-api-surface.sh:6` and
      `scripts/extract-public-api.sh:6` default `BASELINE` to `project/current-exports.txt`, and
      `.github/workflows/ci.yml:171,181,186` pass that literal path — but the directory was renamed
      in commit `928c6d5` ("chore: moved project to .project") and the baseline now lives at
      `.project/current-exports.txt`. `check-api-surface.sh` exits 1 with "No baseline found" on
      every run, so the single automated guard against unintended public-API changes has been
      failing silently and `check-deprecations.sh` runs after the failing step. **Five stale
      references to fix** (2 script defaults, 3 workflow lines). Done when an intentional public
      API change makes the job fail and an unchanged tree makes it pass. Two adjacent items close
      here: M4 Epic 2 FR-7.3's `.public-api-baseline.txt` is recorded as superseded by
      `.project/current-exports.txt` plus `final-api.txt` / `api_surface_current.txt` — a naming
      difference, not a missing capability — and the four remaining deprecated
      `actions-rs/toolchain@v1` steps (`ci.yml:147` in this very job, plus `:317`, `:507` and
      `integration-tests.yml:71`) are upgraded to `dtolnay/rust-toolchain`.
      *Derives: REQ-api-surface-ci, REQ-workspace-ci-upgrade; `intel/code-verification.md` run-3
      verified-open item 1 and 3; INGEST-CONFLICTS run-3 warning 9.*
      **Extended by ingest run 4 — the scope grows from five references to six, and the sixth is a
      requirement rather than code.** All five original references are unchanged as of 2026-07-30
      (`.project/current-exports.txt` exists at 442 KB; `project/current-exports.txt` does not).
      Run 4 adds **M8 Epic 7 FR-10** (`REQ-web-api-baseline-changelog`), which mandates
      `./scripts/extract-public-api.sh project/current-exports.txt` — the same stale path — so the
      defect is now enshrined in an ingested requirement and will be reintroduced by anyone
      implementing that FR literally. `REQ-api-surface-baseline-v020` (M8 Epic 5) depends on the
      same job. **DEBT-01 must correct the requirement text as well as the two script defaults and
      the three workflow lines.** *Derives additionally: REQ-web-api-baseline-changelog,
      REQ-api-surface-baseline-v020; `intel/code-verification.md` run-4 verified-open item 4;
      INGEST-CONFLICTS run-4 INFO on the re-asserted path.*
- [ ] **DEBT-02**: Every type leaving the public API carries
      `#[deprecated(since = "…", note = "…")]` per Milestone 4 Epic 2 FR-8, **or** FR-8 is
      explicitly withdrawn with a recorded reason. `grep -rn '#\[deprecated' src crates` returns
      **0** today, and `Epic_2/DEPRECATIONS.md` corroborates from the other side: "Deprecated
      Items: 0 (none yet)", "Restricted Items: 0 (to be done in Task 6.0)", an empty deprecation
      log, and Tasks 3.0 and 6.0 still in progress. This is the **one genuinely incomplete epic in
      run-3 scope** and the first place in this corpus where an open-checkbox count (20) is
      directly supported by code. Whichever way it resolves, `DEPRECATIONS.md`, the mdbook
      stable-API page and the tree must agree at the end, and the
      v0.2.0 → v0.3.0 → v1.0.0 removal timeline must either start or be withdrawn.
      *Derives: REQ-deprecation-warnings, REQ-stable-api-doc, REQ-api-surface-reduction-target;
      `intel/code-verification.md` run-3 verified-open item 2; INGEST-CONFLICTS run-3 warning 10.*
- [ ] **DEBT-03**: `paladin-ports` doctests compile and run.
      `crates/paladin-ports/Cargo.toml:18` sets `[lib] doctest = false` with the comment
      "Doctests in copied port files reference `paladin::` (root crate) which would require a
      circular dev-dependency. Re-enable in **Task 7.0** after rewriting examples to use
      `paladin_ports::` / `paladin_core::` paths", and `ci.yml:225` runs
      `cargo test --workspace --doc --exclude paladin-ports`. The named "Task 7.0" appears in no
      run-3 task list. The practical consequence: the ~25 port traits whose rustdoc examples are
      the framework's primary integration documentation have **zero** executing example coverage,
      and CI is configured to keep it that way. Done when the port doc examples import via
      `paladin_ports::` / `paladin_core::`, `doctest = false` is removed and the CI exclusion is
      dropped. Directly closes M5 Epic 2 FR-21 and Success Metric 8.
      *Derives: REQ-ports-doctest-compilation, REQ-ports-tests-and-rustdoc, REQ-doc-build-clean;
      `intel/code-verification.md` run-3 verified-open item 4; INGEST-CONFLICTS run-3 warning 8.*
      **Re-asserted by ingest run 4, with a companion question.** `doctest = false` and the identical
      "Task 7.0" comment are unchanged, and `ci.yml:225` still excludes the crate. Run 4 supplies the
      documentation requirements this sits underneath — M7 Epic 4 §4.4.1 (`#![warn(missing_docs)]`
      on all public crates), §4.4.3 (zero `cargo doc --workspace --no-deps` warnings) and §4.4.4
      (>90% documented-public-item coverage per crate) — **all three recorded Met** by
      `epic-4-completion-summary.md`. The exclusion and the coverage claim coexist without either
      document acknowledging the other. **Resolve DEBT-03 together with HARD-07**, which settles
      which `cargo doc` bar the project actually holds. *Derives additionally: REQ-doc-coverage-audit,
      REQ-m8-final-quality-gate; `intel/code-verification.md` run-4 verified-open item 5.*
- [ ] **DEBT-04**: A library-only consumer compiles zero CLI dependencies. The shipped `cli`
      feature is `["dep:clap", "dep:dialoguer", "dep:indicatif", "dep:console", "dep:serde_yaml"]`
      — 5 of the 8 dependencies the Epic 3 PRD and the Epic 1 dependency matrix classify as
      CLI-only — while `structopt = "0.3"` (`Cargo.toml:93`), `colored = "2.1"` (`:125`) and
      `comfy-table = "7.1"` (`:126`) remain unconditional root dependencies. Done when those three
      are `optional = true` and enabled by `cli`, and `cargo tree --lib --no-default-features`
      shows none of them — satisfying M4 Epic 3 FR5.4 and the §8.3 dependency-reduction metric.
      Note the interaction with the v2 tech-debt item that replaces `structopt` with clap v4:
      gating it does not remove the need to migrate it. This is the **inverse** of the run-1 and
      run-2 pattern — here the checkboxes overstate completion rather than understating it.
      *Derives: REQ-cli-dependency-isolation, REQ-library-only-build, REQ-cli-build-time-measurement;
      `intel/code-verification.md` run-3 verified-open item 5; INGEST-CONFLICTS run-3 warning 12.*
- [ ] **DEBT-05**: One `TokenUsage`. Three definitions ship simultaneously —
      `crates/paladin-core/src/platform/container/token_usage.rs:13`,
      `crates/paladin-core/src/platform/container/battalion/mod.rs:497` (carrying its own `new()`
      and `from_total()` constructors) and `crates/paladin-llm/src/llm_analysis_service.rs:51`.
      `BattalionResult.per_paladin_tokens` uses the battalion-local copy while the ports layer uses
      the core copy, so token figures crossing that boundary need conversion. This is exactly the
      duplication run 1 first flagged, that run 2's `REQ-herald-type-consolidation` was meant to
      close, and that the Milestone 5 Epic 1 decision record named `container/token_usage.rs` as
      the canonical answer for — the decision moved *one* copy and left two. Done when
      `grep -rn 'pub struct TokenUsage' crates src` returns exactly one result and the other two
      sites are re-exports. Sequenced after ARCH-03(c), which decides whether `paladin-core` or
      `paladin-ports` owns the canonical type.
      *Derives: REQ-port-value-type-ownership-v1/-v2, REQ-herald-type-consolidation (run 2);
      `intel/code-verification.md` run-3 verified-open item 6; INGEST-CONFLICTS run-3 warning 11.*

---

## v1 Requirements — Milestone 7-8 close-out (Phases 9-11)

Added by ingest run 4 (40 docs, 86 requirements). Milestone 7 hardened the workspace for release —
four more crate extractions, Docker/CI/Makefile/benchmark infrastructure, and API stabilization
through a release candidate. Milestone 8 cleaned the facade — 25 dead files deleted, the
`use_cases` → `services` rename, actix-web removed and banned, and (via a reconciliation the
milestone's own planning documents did not anticipate) the relocations Epic 3 had deferred to
Milestone 9.

**Both milestones are effectively complete** — 98.8% and 99.1% by checkbox, and the tree is ahead
of both figures. Milestone 8's three open items are *contradicted*: Epics 2 and 3 are verifiably
complete and Epic 3 went further than its own task list scoped. Milestone 7's three are plausible,
and their genuine residue is two small defects, both carried below.

**The forward work from run 4 is small, concrete, and not in the checkbox arithmetic.** It is the
security- and release-gate drift (SEC), the record that makes Milestone 7-8 legible and stops the
superseded documents misrouting run 5 (HARD), and the disposition of the five-item deferred
register plus the two deliberately removed features (FACADE).

**No forward requirement below re-plans shipped Milestone 7-8 work.** The four extractions, the
benchmark migration, the release-candidate cycle, the facade deletions, the rename and the
reconciliation's fifteen commits all shipped; they are recorded in the *Milestone 7-8 as-shipped
ledger*.

### Release & security gate integrity (SEC)

Every item below was confirmed by direct inspection of the shipped tree on 2026-07-30. These are
the gates the project believes it already has; four of the five do not hold as documented.

- [ ] **SEC-01**: The RustSec exception set is **one** set, every entry carries the governance the
      acceptance criteria demand, and the 2026-09-30 expiry has a disposition. **Four surfaces
      currently encode four different sets** (verified by direct file reads, not inference):
      1. `Epic_4/rustsec-remediation-plan.md` formally risk-accepts exactly **two** —
         `RUSTSEC-2023-0071` (`rsa 0.9.10`, Marvin timing side-channel, path
         `rsa → sqlx-mysql → sqlx → workspace crates`) and `RUSTSEC-2025-0111` (`tokio-tar 0.3.1`,
         PAX header file smuggling, path `tokio-tar → testcontainers → testcontainers-modules`),
         both "no fixed upgrade available" — with **owner Platform Security (Milestone 7)** and
         **review/expiry target 2026-09-30**.
      2. `.cargo/audit.toml` `[advisories] ignore` holds **five**: those two plus
         `RUSTSEC-2026-0187` (lopdf stack overflow via deeply nested PDF objects, transitive
         through `pdf-extract` under `content-processing`), `RUSTSEC-2026-0194` and
         `RUSTSEC-2026-0195` (quick-xml quadratic attribute parsing and unbounded namespace
         allocation, transitive through `rust-s3`/`aws-creds` under `s3`). Its comment block names
         two further IDs — `RUSTSEC-2026-0185` (quinn-proto) and `RUSTSEC-2026-0190` (anyhow) — as
         **upgraded rather than ignored**, so the file *mentions* seven and *suppresses* five.
      3. `deny.toml` `[advisories] ignore` holds **fifteen**: those five plus ten *unmaintained*
         notices (`RUSTSEC-2021-0139` ansi_term, `-2021-0141` dotenv, `-2022-0104` structopt,
         `-2024-0370` proc-macro-error, `-2024-0375` atty, `-2024-0436` paste, `-2025-0057` fxhash,
         `-2025-0119` number_prefix, `-2025-0121` gcc, `-2025-0134` rustls-pemfile). It *mentions*
         seventeen, the extra two again being -0185/-0190 in a comment. Its own header states "the
         same advisory IDs are mirrored here so cargo-deny and cargo-audit do not contradict each
         other. **Keep these two files in sync**" — an invariant it violates.
      4. `.github/workflows/ci.yml` runs **two independent, differently configured `cargo audit`
         jobs**: `security-audit` at `:77` runs a bare `cargo audit` under a comment declaring
         `.cargo/audit.toml` "the single source of truth" (so: five), and `security` at `:406` runs
         `cargo audit --ignore RUSTSEC-2023-0071 --ignore RUSTSEC-2025-0111` (so: two, inline).
         `Makefile:244-247` `make audit` is bare; `make security` = `audit` + `deny`;
         `cargo deny check` gates at `ci.yml:105`.
      **Thirteen of `deny.toml`'s fifteen have no entry in the formal risk-acceptance register.**
      They carry one-line inline comments, which is documented reasoning but not the "owner, expiry
      date, affected scope, compensating controls, tracked follow-up issue" the plan's own
      acceptance criteria require. This is governance-surface drift on a repository that gates CI on
      both `cargo audit` and `cargo deny` — not undocumented risk-taking, but it needs an owner's
      decision. The plan's own open action items are also unclosed: two tracked impact-analysis
      issues, and "add `audit.toml` exception entries only if approved, each with expiry date and
      owner".
      **Done when** one register is authoritative and the other surfaces mirror it exactly; every
      suppressed ID carries owner, expiry, affected scope and compensating control; the two
      `cargo audit` CI jobs are reconciled to one configuration (or one is removed); and the
      2026-09-30 acceptance is renewed with a new date, closed, or replaced. **Sequenced with
      HARD-06** — the `RUSTSEC-2026-0187` suppression rests on `pdf-extract` being reachable, which
      HARD-06 establishes.
      *Derives: REQ-rustsec-risk-acceptance, REQ-rustsec-hardening-actions;
      `intel/code-verification.md` run-4 verified-open item 1; INGEST-CONFLICTS run-4 warning on the
      RustSec exception list. **The only item in the 153-document corpus carrying an expiry date.***
- [ ] **SEC-02**: The project's licence posture has one answer and the manifests declare it. Three
      positions are live: `Epic_4/license-compatibility-decision-checklist.md` records
      **`MIT OR Apache-2.0`** with approver **`DF3NDR` (repository owner)**, approval date
      **2026-05-28**, an inventory of 551 packages with zero unknown entries, and an explicit
      acceptance of MPL-2.0 for unmodified use; M7 Epic 4 PRD §4.7.7 and the M7 overview AC 1 say
      **MIT**; the shipped root `Cargo.toml` declares `license = "MIT"`. The dual-licence approval
      rule ("any SPDX expression containing a permissive MIT/Apache branch is acceptable by
      default") was the stated basis for accepting `r-efi 5.3.0`'s
      `MIT OR Apache-2.0 OR LGPL-2.1-or-later`; if the project is MIT-only that rationale is weaker
      than recorded, and a 551-package sign-off rests on a policy the manifests do not declare. Note
      that the *enforcement* surface already follows the checklist: `deny.toml [licenses] allow` is
      permissive-only with eight narrowly-scoped per-crate MPL-2.0 exceptions.
      **Done when** the root package and all ten library crates declare the same licence
      expression, `deny.toml` agrees, and either the checklist or the PRD is marked superseded. This
      is a sign-off artefact with a named approver — it must not be resolved by inference.
      *Derives: REQ-license-policy-signoff, REQ-crate-metadata-completion; INGEST-CONFLICTS run-4
      warning on the three-way licence posture.*
- [ ] **SEC-03**: crates.io package-name collisions are caught before they cost a release cycle.
      `Epic_4/deferred-paladin-ports-publish-verification.md` closes Task 5.5 as **Resolved** and
      leaves exactly one residue: "Keep CI/package guardrails that detect crates.io package-name
      collisions early." Collisions were not hypothetical — they cost Epic 4 two package renames
      (`paladin-core` → `paladin-ai-core`, root → `paladin-ai`, which is why every crate's package
      name and lib name now diverge) and a full NO-GO cycle. Today the earliest guard is
      `ci.yml:617` `publish-dry-run` running `cargo publish --workspace --dry-run`, with
      `release.yml:410` doing a per-crate dry run inside the release job itself — both late, and
      neither name-specific.
      **Done when** a name-availability check runs earlier than dry-run, **or** reliance on the dry
      run is recorded as an accepted decision with its known cost. A decision either way closes it.
      *Derives: REQ-paladin-ports-publish-verification-closed, REQ-ci-publish-dry-run-v1/-v2;
      `intel/code-verification.md` run-4 verified-open item 6.*
- [ ] **SEC-04**: `crates/paladin-herald/CHANGELOG.md` exists, or an exemption is recorded. M7
      Epic 4 §4.3.1 and AC 3 require a Keep-a-Changelog `CHANGELOG.md` for **every** public crate,
      and `epic-4-completion-summary.md` records that criterion **Met** ("Per-crate changelogs
      created/backfilled"). Verified 2026-07-30: nine of ten library crates have both a `README.md`
      and a `CHANGELOG.md`; `crates/paladin-herald/` has the README only. The crate was created
      *after* Epic 4 closed, by reconciliation commit `66f6c4e`, so the audit that marked the
      criterion Met never covered it. Small, but it is a release-gate criterion on a published crate
      family.
      *Derives: REQ-per-crate-changelog, REQ-crate-metadata-completion, REQ-release-readiness-audit;
      `intel/code-verification.md` run-4 verified-open item 2.*
- [ ] **SEC-05**: `Dockerfile.chef`'s planner stage cannot silently go stale as crates are added.
      M7 Epic 2 FR-01 requires all `crates/*/Cargo.toml` files in the planner stage, and §6 states
      the purpose: "the dependency layer only invalidates when a `Cargo.toml` changes". Verified:
      `Dockerfile.chef:25-33` enumerates exactly nine crate manifests by name — core, ports,
      battalion, llm, memory, storage, notifications, content, web — omitting
      `crates/paladin-herald/Cargo.toml`. The later `COPY crates ./crates` at `:36` means the image
      still builds, so nothing fails; the cache-tightness FR-01 exists to deliver is simply not
      achieved for `paladin-herald`.
      **Done when** the herald manifest is covered **and** the mechanism cannot miss an eleventh
      crate — an enumerated list that goes stale on every crate addition is the defect, not just the
      one missing line.
      *Derives: REQ-docker-workspace-build; `intel/code-verification.md` run-4 verified-open
      item 3.*

### Milestone 7-8 ground truth & recorded account (HARD)

- [ ] **HARD-01**: The *Milestone 7-8 as-shipped ledger* below is upgraded from component-level file
      evidence to per-criterion verdicts with `file:line` citations, for all **86** run-4
      requirement IDs. Must carry four dispositions, not two: `Shipped`, `Superseded by outcome`
      (the 14-row table in `intel/code-verification.md` — requirements that must **not** be planned
      as written), `Relocated` (deliverables that exist at a different path, chiefly the mdbook), and
      `Deferred with register` (the D1-D5 items and the two removed features). Must also record that
      **the five crates ARCH-01 marked "provenance pending" now have one**: `paladin-storage`,
      `paladin-notifications`, `paladin-content` and `paladin-web` from M7 Epic 1's extraction PRD
      and its cost-benefit gate, and `paladin-herald` from the 2026-06-04 reconciliation rather than
      from any PRD — which is the reason no ingested requirement described it before run 4, and the
      reason the earlier "9-crate workspace" figure was wrong.
      *Derives: all 86 run-4 `REQ-*` IDs; `intel/code-verification.md` run-4 section; narrows
      ARCH-01's pending-provenance clause.*
- [ ] **HARD-02**: `facade-cleanup-RECONCILIATION-2026-06-04.md` is recorded as **the authoritative
      account of Milestone 8**, superseding `Epic_1/facade-audit.md` and
      `Epic_3/infrastructure-adapter-disposition.md`. The record must carry:
      **(a) why it supersedes them** — both describe ~4,400 LOC of *orphaned, uncompiled duplicate
      files* as "active bridges that stay". The reconciliation's verification method is stated and
      reproducible (`rg "mod <name>"` across `src/` returns nothing for each; the directory's
      `mod.rs` only does `pub use paladin_<crate>::…`; the leaf-crate file exists) and the tree
      confirms every relocation target;
      **(b) that it executed relocations Epic 3 had deferred to Milestone 9**, so Epic 3 is complete
      *in substance*, not punted — 15 commits, ~10,250 net LOC removed, one new leaf crate;
      **(c) three in-execution corrections to the audit that must survive into the record** —
      `paladin_registry.rs` was **not** a duplicate (the facade's 418-LOC impl was richer than
      battalion's 67-LOC `pub(crate)` copy, so the richer one was consolidated *into* battalion
      rather than deleted blindly); `sqlite_*_repository.rs` were **not** redundant (they were the
      active default-build impl, resolved by making `paladin-storage` non-optional); the rest —
      `mysql_content_repository.rs`, the `input/*` fetchers, `document/*`,
      `output/api_content_deliverer.rs`, `error_log_adapter.rs` — genuinely were orphaned;
      **(d) that Epic 6 is complete** despite the reconciliation recording it "Not verified; low
      priority" and `deferred-items.md` omitting it — `crates/paladin-content/src/services/` ships,
      `lib.rs` declares `pub mod services;`, and a workspace-wide grep for `use_cases` across
      `src/`, `crates/`, `tests/`, `examples/` and `benches/` returns **zero** matches. Do not plan
      Epic 6 as outstanding;
      **(e) that `paladin-herald` exists**, created inside an Epic whose §5 Non-Goals state "No new
      crates created. `paladin-herald`, `paladin-ml`, etc. are not in scope" — the non-goal names
      the exact crate that was then created, in the same milestone. Record the non-goal as
      **overridden for `paladin-herald` and still holding for `paladin-ml`**, which is the
      distinction FACADE-03 depends on.
      *Derives: REQ-m8-reconciliation-relocations, REQ-facade-audit-document,
      REQ-adapter-disposition-record, REQ-m8-epic3-no-extractions, REQ-paladin-content-services-rename;
      `intel/code-verification.md` run-4 "Claims contradicted by code in the favourable direction";
      INGEST-CONFLICTS run-4 warnings on the disposition record and the new-crate non-goal.*
- [ ] **HARD-03**: The version trajectory is recorded as **history**, and no `v0.1.0-rc.1` artefact
      is treated as current state. What happened: M7 Epic 4's PRD and the overview Appendix C
      anchored the first publishable release at lockstep `0.2.0`; what shipped was **all ten crates
      published at `0.1.0`**, tagged **`v0.1.0-rc.1`** at commit `a9530fc` on 2026-05-28, with a
      release-readiness audit recording every gate PASS, a **GO** sign-off, and post-release
      verification confirming all ten crates resolve on docs.rs (including the package/lib split
      made visible at `docs.rs/paladin-ai-core/latest/paladin_core/` and
      `docs.rs/paladin-ai/latest/paladin/`) plus an external smoke project compiling against
      `paladin-ai = "0.1.0"`. Milestone 8 targeted v0.2.0 throughout; its Epic 7, written
      2026-06-06, targets "post-v0.5.1 (Unreleased)" — so v0.3.0 through v0.5.1 all shipped between
      Epic 5 and Epic 7, and the M8-11 dependency graph's v0.2.0 → v0.3.0 → v0.4.0 → v0.5.0 sequence
      completed. Current tree: `Cargo.toml` `0.6.0`, branch `release/v0.7.0`, latest tag `v0.5.1`.
      **Feeds REL-01**, which converges the three-way version disagreement — and REL-01 must not
      converge on any rc.1 figure.
      *Derives: REQ-versioning-policy, REQ-release-checklist, REQ-release-readiness-audit,
      REQ-crate-metadata-completion; `context.md` Topic: Version trajectory across runs 1-4.*
- [ ] **HARD-04**: The **fourth** milestone-numbering collision is recorded with the same convention
      that closes the first three. The Milestone 7 overview titles itself "Milestone 4: Production
      Hardening and Extended Workspace Decomposition" while its path is
      `Milestone_7-Production-Hardening`, and its Prerequisites credit "Milestones 1-3" with work
      the directory numbering assigns to Milestones 4-6 (feature flags and CI matrix; core workspace
      crates; `application_settings.rs` decomposition; manager-service relocation; Maneuver DSL
      co-location; `CircuitBreaker` relocation). Directory / task-list numbering is authoritative,
      as in VERIFY-03 and ARCH-02. Roadmap Extension Protocol item 8 predicted a third instance;
      this is it. Expect a fifth in run 5.
      *Derives: `context.md` Topic: Milestone 7 scope, structure and self-numbering; same defect
      class as ARCH-02 and VERIFY-03.*
- [ ] **HARD-05**: The extracted-crate dependency rule has **one** stated form. M7 Epic 1 PRD §6.1
      states it absolutely — "No extracted crate may depend on another extracted crate or on the
      `paladin` facade" — and Goal 2 restricts each new crate to `paladin-core`, `paladin-ports` and
      workspace-shared dependencies. The same PRD's §4.4 complexity assessment anticipated the
      violation without amending the rule: "use-case services depend on `paladin-llm` for LLM
      analysis, creating an inter-crate dependency that must be handled carefully". Verified:
      `crates/paladin-content/Cargo.toml` declares
      `paladin-llm = { version = "0.6.0", path = "../paladin-llm", optional = true }` behind its
      `llm` feature, and the facade's `content-processing` enables `paladin-content/llm`. This is
      the invariant that keeps the extraction from re-creating the coupling it was built to remove;
      stated absolutely and violated once, it is unclear whether the rule is "never" or "never,
      except behind an optional feature". **Strongest SPEC candidate in run 4** — if it should bind
      above PRD precedence, re-tag `prd-extract-infrastructure-crates.md` via `--manifest` and
      re-run ingest. **The answer gates FACADE-02**, because D2/D3/D4's proposed relocation targets
      create exactly this class of edge.
      *Derives: REQ-extracted-crate-dependency-rule, REQ-paladin-content-extraction;
      INGEST-CONFLICTS run-4 warning on the dependency rule; `constraints.md` run-4 strongest SPEC
      candidate.*
- [ ] **HARD-06**: Whether PDF extraction is still a supported capability has one answer. Three
      facts point in two directions. `crates/paladin-content/Cargo.toml:18` declares `pdf = []` — a
      feature gating **no dependency at all**, so `pdf-extract` is not reachable through it. The
      facade's `content-processing` enables `web-scraping`, `rss`, `news-api`, `tiktoken`, `llm`
      and `paladin-memory/content-processing` — five of six capability features, omitting `pdf`,
      against M7 Epic 1 §4.4.6's requirement that it activate `paladin-content` "with **all**
      capability features enabled" and §4.4.1's requirement that `pdf` gate `pdf-extract`. Yet
      `.cargo/audit.toml` suppresses `RUSTSEC-2026-0187` on the stated grounds that "lopdf is
      transitive via `pdf-extract` (optional `content-processing`)", which asserts `pdf-extract`
      **is** in the graph. PDF extraction was one of the four subsystems the extraction epic was
      built around (~350 LOC `pdf_extractor.rs`, called out in the overview background and the
      cost-benefit assessment). **SEC-01 cannot be reconciled honestly until this is settled** — if
      `pdf-extract` is unreachable, the -0187 suppression is unnecessary rather than accepted.
      *Derives: REQ-paladin-content-extraction, REQ-content-processing-build-gate,
      REQ-rustsec-risk-acceptance; INGEST-CONFLICTS run-4 warning on the `content-processing`
      feature.*
- [ ] **HARD-07**: One `cargo doc` bar, applied consistently. M7 Epic 4 §4.4.3 and M7 Epic 1 §4.6.4
      / §8.9 require `cargo doc --workspace --no-deps` to complete with **zero warnings**; M8 Epic 5
      FR-19 requires only exit 0 with "**warnings acceptable; must not fail**". The same command is
      a zero-warning gate in one milestone and a warnings-tolerated gate in the next. Combined with
      `crates/paladin-ports/Cargo.toml` still setting `[lib] doctest = false` and `ci.yml:225`
      excluding the crate from `--doc` (DEBT-03), and with M7 Epic 4 §4.4.1's
      `#![warn(missing_docs)]` and §4.4.4's >90% documented-public-item coverage target — both
      recorded **Met** by `epic-4-completion-summary.md` — the documentation bar the project
      actually holds itself to is ambiguous. **Resolve alongside DEBT-03**, not separately: the
      "Task 7.0" doctest re-enable and the gate bar are the same question asked twice.
      *Derives: REQ-doc-coverage-audit, REQ-crate-metadata-completion, REQ-m8-final-quality-gate;
      INGEST-CONFLICTS run-4 warning on the two `cargo doc` bars; extends DEBT-03.*

### Facade residue & deferred register disposition (FACADE)

`deferred-items.md` and `deferred-features.md` are the two most reliable documents in the corpus by
measurement: D5's claim of 17 `println!`/`eprintln!`/`dbg!` occurrences across 6 files matches the
tree **exactly**, as does every other verifiable claim in either register. They — not checkbox
arithmetic — are the Milestone 8 forward-work source.

- [ ] **FACADE-01**: D5 is closed. `grep -rn "println!\|eprintln!\|dbg!" src/application/services/
      src/infrastructure/` returns **exactly 17 occurrences across exactly 6 files**:
      `services/herald/herald_registry.rs`, `services/paladin/paladin_execution_service.rs`,
      `infrastructure/adapters/arsenal/{mcp_protocol,tool_result_formatter}.rs`,
      `infrastructure/adapters/scheduling/tokio_cron_adapter.rs` and
      `infrastructure/resilience/circuit_breaker.rs` — down from ~435 across 36 files before commit
      `4c7857e`. The register rates it low effort / low risk and names it the quick win. Scope is
      `services/` + `infrastructure/` only; `cli/` stdout is intentional and out of scope.
      **Done when** each of the 17 is either converted to `log::*` or annotated as deliberate
      stdout, with the disposition recorded per file.
      *Derives: REQ-m8-deferred-items-register (D5); `intel/code-verification.md` run-4 — count
      verified exact.*
- [ ] **FACADE-02**: D1-D4 each carry a **disposition with an owner**, not an effort rating. The
      register states ratings and recommendations; it names no owners and assigns no target
      milestone. Each must resolve to *do* / *defer with a stated trigger* / *withdraw*:
      **D1 — `src/core/` re-export shims (currently "KEEP, by decision").** Verified: `src/core/`
      is exactly six files (`mod.rs`, `platform/mod.rs`, `platform/manager/{mod,content_service,
      event_manager,user_service}.rs`) and **49 facade files** import via `crate::core::…`. Removal
      means a mechanical path rewrite of those 49 plus preserving `platform/mod.rs`'s
      maneuver/parser path injection, which carries real logic rather than re-exports. Interacts
      with **ARCH-04** — if the facade adopts a "no re-export aliases" policy, D1 follows from it.
      **D2 — mis-layered `src/core/platform/manager/` services.** `content_service.rs`,
      `event_manager.rs` and `user_service.rs` are application/domain services, not composition
      glue. The Epic 1 audit recommends `ContentItemService` → `paladin-core`, `EventService` →
      `paladin-core` or a facade app-service, and `user_service` **split** (trait + DTOs →
      `paladin-core`/`paladin-ports`, concrete impl → a facade app-service). Note two things that
      narrow it: reconciliation commit `6704807` already found "no user-service split was needed"
      for the controller case because `UserServiceTrait` and the DTOs **already live in
      `paladin-core`**; and the *full* `user_service` relocation is already carried as a run-3 v2
      tech-debt item. Do not plan it twice.
      **D3 — entangled Paladin services (currently "KEEP for now").**
      `src/application/services/paladin/{planning_service,prompt_generation_service,
      temperature_service,handoff_service}.rs`, ~2,750 LOC, tightly coupled to `paladin_builder.rs`
      and `paladin_execution_service.rs`. Candidates for `paladin-battalion` (planning/handoff) and
      `paladin-llm` (prompt/temperature) — **both leaf-to-leaf edges, so gated on HARD-05.**
      **D4 — `content_ingestion_service.rs` placement.**
      `src/application/services/content/content_ingestion_service.rs`, ~1,211 LOC. M7 Epic 1's
      extraction PRD listed it as moving to `paladin-content`; the facade kept its own copy, and a
      move needs a dependency-coupling review first.
      The register's own suggested grouping stands as input: architecture pass → D2 plus optionally
      D4; only alongside a broader refactor → D3, and D1 only if a no-alias policy is adopted.
      *Derives: REQ-m8-deferred-items-register (D1-D4); gated on HARD-05; interacts with ARCH-04 and
      the run-3 v2 `user_service` deferral.*
- [ ] **FACADE-03**: The two deliberately removed features have a recorded status, and their
      reintroduction conditions survive in `.planning/` rather than only in a DOC.
      **(a) The `paladin user …` CLI surface** — 1,065 LOC, eight subcommands (`register`, `login`,
      `get`, `update`, `list`, `activate`, `deactivate`, `verify`), removed because it was declared
      but **never dispatched**: no `UserCommands` arm existed in the binary's top-level match, so it
      compiled and did nothing. The backend is intact and in use elsewhere
      (`UserService`/`UserServiceTrait` and the request DTOs), so reintroduction is "mostly
      re-wiring, not new domain work" — add the enum arm plus a dispatch handler using the existing
      `config/user_config.rs` DI wiring, add tests, and **recover the module verbatim from git
      history at the Milestone 8 removal commit on branch `chore/facade-cleanup-m8-finish`** rather
      than rewriting. Verified: `src/application/cli/commands/` holds ten modules and `user.rs` is
      not among them.
      **(b) The TensorFlow ML adapter** — 636 LOC `#[doc(hidden)]` placeholder implementing
      `MlPort` with stub model loading and prediction, plus the `ml = []` flag; both removed
      outright (commit `3d48768`), verified absent from `Cargo.toml` and `src/`. **The load-bearing
      part is the placement condition**: the real adapter MUST be implemented in a dedicated
      `paladin-ml` **leaf crate** — "consistent with the hexagonal layout — ML inference is an
      infrastructure adapter, not facade code — rather than re-adding it to the facade" — with the
      `ml` flag moved onto that crate. `paladin_ports::input::ml_port::MlPort` remains in the
      workspace so the integration point is stable. This condition is carried **only by a DOC**, and
      it is the surviving half of the M8 Epic 3 non-goal that `paladin-herald` overrode (HARD-02e).
      **Done when** both are recorded in `.planning/` as deliberate deferrals with their conditions
      intact, **or** promoted to scope with a phase. Either outcome closes it; silent omission does
      not.
      *Derives: REQ-deferred-cli-user-commands, REQ-deferred-tensorflow-ml-adapter-v3
      (and its `-v1`/`-v2` predecessors, variant group 24); `intel/code-verification.md` run-4 —
      both removals verified.*
- [ ] **FACADE-04**: The Milestone 9 candidate lists the reconciliation superseded are triaged
      **before run 5 consumes them**. `Epic_3/infrastructure-adapter-disposition.md` is the artifact
      the Epic 3 PRD §6 designates "the authoritative cross-reference for the §4.3 M9 flags" — the
      document Milestone 9 was meant to read. It is unsafe as-is on three counts: all 20 of its rows
      record "Stays / No change" while the reconciliation executed most of its List B; its M9
      targets name **two crates that do not exist** (`paladin-arsenal`, `paladin-sanctum`); and it
      disagrees with its own governing PRD on two rows (`arsenal/` is an M9 candidate here but "No"
      in the PRD table on the reasoning that "MCP wiring is facade composition-root
      responsibility"; `sanctum/` targets `paladin-sanctum` here but `paladin-memory` in the PRD).
      It is also dated `2025-01`, inconsistent with every other Milestone 8 document. Note that the
      `arsenal/` rows were **never acted on** either way, so the list is not wholly obsolete.
      **Done when** each surviving row is marked *done* / *not a candidate* / *still open*, and the
      `paladin-arsenal` / `paladin-sanctum` names are confirmed real or recorded as artefacts of a
      mis-written table. `task-completion-state.md` records Milestone 9 at **100% complete**, so run
      5 will be attaching requirements to work that has largely shipped — reading this list
      uncorrected would plan relocations that already happened.
      *Derives: REQ-adapter-disposition-record, REQ-m8-epic3-no-extractions,
      REQ-garrison-sanctum-bridges-kept; INGEST-CONFLICTS run-4 warnings on the disposition record
      and the arsenal/sanctum M9 targets. **Explicitly scoped to unblock ingest run 5.***

---

## Competing variants (preserved unmerged)

**28 variant groups, 56 entries**, carried verbatim in scope from `.planning/intel/requirements.md`
(12 from run 1, 18 from run 2, 8 from run 3, 18 from run 4). **No winner is selected here.** Where a
variant matters, the pointer below is to the codebase map, `intel/code-verification.md` or a shipped
`file:line` — the real arbiters — not to a resolution. `INGEST-CONFLICTS.md` counts 53 warnings
across runs 1-4; the entry count differs because several groups carry three entries and several
warnings are not `-v1`/`-v2` pairs at all (those are listed after group 28).

**Run 4 is the run where shipped code settles the most variants** — six of the eight new groups
carry a `settled-by` pointer. That is recorded as a **fact about the tree**, at the top of the
precedence order, and never as a decision taken here.

**Run 3 closed five earlier warnings with shipped code** — groups 3, 4 and 15 below carry a
*Run-3 code verification* note, and two further run-1 warnings (the Battalion base module path,
and `metadata_output_dir`'s three claimed owners) are resolved inside groups 3 and 4. Those notes
are **observations of the tree recorded at the highest precedence tier**, not decisions taken
here; the entries themselves stay unmerged, and none is deleted.

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
- **Run-3 code verification (highest precedence, not a decision):** the shipped
  `BattalionConfig` at `battalion/mod.rs:37` is `REQ-battalion-config-v1` **exactly** —
  `name`, `description: Option<String>`, `timeout_seconds`, `retry_policy: RetryPolicy`,
  `error_strategy: ErrorStrategy`, `metadata_output_dir: Option<PathBuf>`. Epic 5's
  `retry_attempts: u32` and `enable_checkpointing: bool` are absent and `description` was not
  dropped. `CommanderConfig` **does not exist anywhere** in `crates/` or `src/`, so
  `REQ-commander-config-metadata-dir-v3` was never built and the run-2 "three competing owners"
  warning has exactly one owner. RECON-02 records this; the `citadel.rs` duplicate is untouched by
  it and still needs resolving (GAP-07).

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
- Status: **unresolved on paper.** Tree observation, not a decision: `BattalionMetadata` ships
  `per_paladin_times: HashMap<String, u64>` (`battalion/mod.rs:582`) and
  `per_paladin_tokens: HashMap<String, TokenUsage>` (`:585`) — the run-2 field names with the
  run-1 module path (`battalion/mod.rs`, not `battalion_result.rs`). See `codebase/ARCHITECTURE.md`.
- **Run-3 code verification (highest precedence, not a decision):** the shipped `BattalionResult`
  at `crates/paladin-core/src/platform/container/battalion/mod.rs:549` is a **merged superset of
  all three consumers**, with Epic 5's `metadata` map flattened into top-level fields:
  `battalion_id`, `battalion_name`, `started_at`, `completed_at`, `final_output`,
  `paladin_results: Vec<PaladinResult>`, `status: BattalionStatus`,
  `strategy_used: BattalionStrategy`, `strategy_selection_reasoning: Option<String>`,
  `strategy_selection_time_ms: u64`, `per_paladin_times: HashMap<String, u64>`,
  `per_paladin_tokens: HashMap<String, TokenUsage>`, `total_tokens: u64`,
  `paladin_success_count: usize`, `paladin_failure_count: usize`,
  `node_errors: Vec<NodeError>`. Epic 4's field set is fully present. Epic 5's is present except
  `execution_time_ms` (superseded by `per_paladin_times`) and `errors: Vec<PaladinError>`
  (superseded by `node_errors: Vec<NodeError>`, a plain-data struct, because `BattalionError` does
  not derive `Serialize`/`Deserialize` while `BattalionResult` does). Epic 8's Herald expectation
  is satisfied: Battalion type as `strategy_used`, aggregated tokens as `total_tokens` plus
  `per_paladin_tokens`. `intel/code-verification.md` states explicitly: **do not plan a
  reconciliation task.** RECON-03 records the verdict; GAP-07 no longer carries the code change.
  Note that the Milestone 5 Epic 1 decision record — despite its filename — does **not** settle
  this and never mentions `BattalionResult` (see group 19).

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
- Status: **unresolved on paper, resolved in the tree as two distinct types.**
  `RELEASE_NOTES_MILESTONE_3.md` documents both sets in different sections of the same document.
  Shipped error aggregation: `crates/paladin-battalion/src/error_aggregation.rs`.
- **Run-3 code verification (highest precedence, not a decision):** these are **two enums in two
  crates**, exactly as each PRD describes —
  `crates/paladin-core/src/platform/container/battalion/mod.rs:240` carries `FailFast` (default),
  `ContinueOnError`, `RetryThenContinue` for Battalion; and
  `crates/paladin-battalion/src/maneuver/mod.rs:18` carries `FailFast` (default),
  `ContinueParallel`, `IgnoreErrors` for Maneuver. Milestone 6 Epic 3 physically separated them
  into different crates, which removes the name collision as a practical concern. Both entries
  stand; they describe different types, not competing definitions of one type.

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


### Group 17 — Rust edition for the workspace crates *(run 3)*

- **REQ-workspace-crate-edition-v1** — Milestone 5 Epic 1 FR-5 / §7: `edition = "2021"` for
  `paladin-core`, and "All crates in this workspace must use `edition = \"2021\"`. Do not use an
  older edition." Repeated verbatim by Epic 2 FR-2, Epic 3 FR-2 and Epic 4 FR-2 for
  `paladin-ports`, `paladin-battalion` and `paladin-llm`.
- **REQ-workspace-crate-edition-v2** — Milestone 5 Epic 5 FR-1.2: `paladin-memory` declares
  `edition = "2024"` "(matching the workspace root)", corroborated by the milestone overview's
  Appendix D, which sets `[workspace.package] edition = "2024"`.
- Status: **unresolved on paper and genuinely mixed in the tree.** Verified 2026-07-30: the root
  `paladin-ai` package and every crate except two declare `2024`; `crates/paladin-ports` and
  `crates/paladin-notifications` declare `2021`. Edition affects name resolution, `unsafe`
  handling and macro hygiene, and Epic 5 names `paladin-llm` as the canonical template for new
  crates — so a scaffolded crate inherits whichever value its author read. Recording the answer is
  ARCH-03(a); applying it is REL-02.

### Group 18 — `paladin-core` dependency allowlist *(run 3)*

- **REQ-paladin-core-dependency-allowlist-v1** — Milestone 5 Epic 1 FR-6 / Appendix B: exactly
  six crates — `serde`, `uuid`, `chrono`, `thiserror`, `async-trait`, `serde_json` — and "No other
  dependencies are permitted"; Appendix B calls itself "the complete and exhaustive list", with
  anything else requiring explicit approval and documented justification.
- **REQ-paladin-core-dependency-allowlist-v2** — Milestone 5 Epic 3 §9 Open Question 4 (the later
  PRD, 2026-05-17 vs 2026-04-21): `petgraph` "is used by both `paladin-core` (Campaign DAG domain
  types) and `paladin-battalion`", requiring workspace version alignment — a seventh dependency
  never approved against Appendix B. `Milestone_4/Epic_1/dependency-matrix.md` independently
  classifies `petgraph` as core.
- Status: **unresolved, and wrong by eight in the tree.** Tree observation, not a decision:
  `paladin-core` ships the six plus `tokio`, `sha2`, `blake3`, `petgraph`, `murmur3`, `url`,
  `regex` and `futures`. The same drift affects `paladin-ports`, whose Epic 2 FR-3 allowlist of 7
  ships as 10 (`+ serde_json`, `futures`, `md5`) — though FR-23's substantive constraint (no
  `redis`, `sqlx`, object-storage SDK or LLM provider SDK) still holds, so the architectural
  invariant is intact and only the allowlist text is stale. The Milestone 5 overview additionally
  contradicts itself, quoting four crates in its headline success criteria and five in Epic 1's
  objective paragraph. Recording the answer is ARCH-03(b).

### Group 19 — ownership of `PaladinResult`, `StopReason` and `TokenUsage` *(run 3)*

- **REQ-port-value-type-ownership-v1** — `Milestone_5/Epic_1/decisions/battalion-result-upward-dependency-decision.md`
  (`Status: Approved`, 2026-05-13, `Chosen Option: Option A`): `PaladinResult` and `StopReason`
  move to `core/platform/container/execution_result.rs`, `TokenUsage` to `token_usage.rs`,
  `RegistryError` to `registry_error.rs`, `HandoffError` to `arsenal/handoff_error.rs`; the four
  application-layer files become thin `pub use` re-exports so every existing
  `paladin::application::ports::output::…` path keeps resolving. `PaladinError` is deliberately
  **excluded** because it carries `#[from] GarrisonError` from the application layer, and the
  convenience `pub use PaladinError` in `herald.rs` is removed.
- **REQ-port-value-type-ownership-v2** — Milestone 5 Epic 2 PRD (2026-05-15) FR-7 lists
  `PaladinResult` / `StopReason` as primary exported types of `paladin-ports`' `paladin_port.rs`
  and `TokenUsage` of `llm_port.rs`; FR-10 rules that "all associated types that are defined
  **within** a port module file … must move with their port trait into `paladin-ports`. Types must
  not be split across crates." FR-11 grants a core-re-export carve-out for `RegistryError` **only**.
- Status: **unresolved, and this is the one place where mechanical precedence gives the wrong
  answer.** The decision record has every structural marker of an ADR but is manifest-typed DOC, so
  a PRD published two days later outranks it. Following FR-10 literally would either duplicate the
  three types in `paladin-ports` or move them back out of `paladin-core`, reintroducing the exact
  upward dependency Epic 1 was written to remove. Tree observation, not a decision: shipped code
  implements **v1** — `paladin-core/src/platform/container/{execution_result,token_usage,registry_error}.rs`
  and `arsenal/handoff_error.rs` all exist. Recording the answer is ARCH-03(c); the record is one
  of the corpus's two strongest ADR-promotion candidates. **Scope note:** this record settles
  *location* for five types only. It never mentions `BattalionResult` — see group 4.

### Group 20 — LLM configuration bridge location *(run 3)*

- **REQ-llm-config-bridge-location-v1** — Milestone 5 Epic 4 FR-31 to FR-33: `paladin-llm` "must
  NOT import from `crate::config::application_settings` or any equivalent path in the root
  `paladin` crate. Doing so would create a circular dependency"; the root crate is "solely
  responsible" for converting `ApplicationSettings.llm.*` into the `paladin-llm` `*Config` structs,
  and that conversion lives at `src/infrastructure/adapters/llm/config_bridge.rs`. A
  `paladin-config` crate is an explicit non-goal.
- **REQ-llm-config-bridge-location-v2** — Milestone 6 Epic 1 §4.1 (the later PRD, 2026-05-23 vs
  2026-05-18): `crates/paladin-llm/src/config/{mod,llm,vision}.rs` are created and exposed as
  `pub mod config;`, holding `LlmProviderConfig`, `LlmConfig`, `VisionRetryConfig`,
  `VisionProviderConfig` and `VisionConfig` — i.e. the config Epic 4 built the bridge to keep out.
- Status: **unresolved on paper; both are PRD precedence so no tiebreaker applies.** Either the
  circular-dependency concern was wrong or the Milestone 6 decomposition breaks the boundary
  Epic 4 established. Tree observation, not a decision: `crates/paladin-llm/src/config/bridge.rs`
  and `crates/paladin-llm/src/config/{llm,vision}.rs` exist — the bridge moved **into**
  `paladin-llm`, which is v2. Recording the answer is ARCH-03(d).

### Group 21 — `paladin-web` web framework *(run 4)*

- **REQ-paladin-web-extraction** — M7 Epic 1 PRD §4.2.1: `crates/paladin-web/Cargo.toml` MUST
  declare **`actix-web` and `axum` as direct (non-optional) dependencies**; the M7 overview Epic 1
  Task 1.2 describes the crate as an "Actix-web application factory, REST API route handlers,
  WebSocket handlers, middleware".
- **REQ-actix-removal** — M8 Epic 7 PRD Goal 1 / FR 5: `actix-web` MUST be removed from
  `crates/paladin-web/Cargo.toml`, FR 8 adds it to `deny.toml`'s banned crates, and Success Metric 1
  requires `rg actix crates/paladin-web/` to return zero matches.
- Status: **the two cannot both hold.** M7's rationale was extracting an existing two-framework
  subsystem; M8's was that the actix handlers were orphaned — "nothing in the workspace ever starts
  an actix `HttpServer`, and `configure()` is never called". Tree observation, not a decision:
  zero `actix` references in `crates/paladin-web/`; facade
  `web-server = ["dep:paladin-web", "dep:axum"]`; the ban lives at `deny.toml:99-103`. Recording the
  answer is HARD-01.
- settled-by: `intel/code-verification.md` run-4.

### Group 22 — `paladin-storage` feature-flag shape *(run 4)*

- **REQ-storage-feature-flags-v1** — M7 Epic 1 PRD §4.5.6 / §7.2 / §9 Q2: facade
  `storage-sqlite = ["dep:paladin-storage", "paladin-storage/sqlite"]`,
  `storage-mysql = [...]`, a `storage = ["storage-sqlite", "storage-mysql"]` convenience alias, and
  `paladin-storage = { workspace = true, optional = true }`.
- **REQ-storage-nonoptional-v2** — the 2026-06-04 reconciliation §4 Category 2 / §7 commit
  `897e77e`: `paladin-storage` becomes **non-optional**, the facade sqlite fallback repositories
  (810 + 676 LOC) are deleted, and the `storage-sqlite` feature is **retired**. ~1,486 LOC removed.
- Status: a downstream consumer following the M7 PRD would look for a flag that no longer exists,
  and the M7 promise that a SQLite-only consumer must not link `libmysqlclient` is now delivered by
  a different mechanism. Tree observation: root `Cargo.toml` declares
  `paladin-storage = { workspace = true, features = ["sqlite"] }` with the inline comment "SQLite
  repositories are always available"; only `storage-mysql` and `storage = ["storage-mysql"]`
  survive.
- settled-by: `intel/code-verification.md` run-4.

### Group 23 — publish dry-run shape *(run 4 — coexistence, not supersession)*

- **REQ-ci-publish-dry-run-v1** — M7 Epic 2 FR-26 / M7 Epic 4 §4.5.6 / overview Appendix B: run
  `cargo publish --dry-run -p <crate>` for the ten crates **in dependency order**
  (`paladin-core` → `paladin-ports` → leaf crates → facade), warning that "violating this order will
  cause `cargo publish --dry-run` to fail".
- **REQ-ci-publish-dry-run-v2** — `.github/workflows/ci.yml:617-644`: a single
  `cargo publish --workspace --dry-run`, with an inline counter-rationale that per-crate dry runs
  "cannot work on a version bump: the not-yet-published new version of each sibling fails the
  `version = \"X\"` requirement of its dependents". This position has **no document carrier** and
  therefore no precedence standing.
- Status: tree observation — **both shipped, in different workflows.** `ci.yml:644` is the
  workspace-wide form; `release.yml:410` runs `cargo publish --dry-run -p "$crate"` per crate inside
  the release job. So this is coexistence rather than one superseding the other, which the ingest
  report could not see from the documents alone. Recording it is HARD-01; SEC-03 depends on the
  answer for *when* a collision is detectable.

### Group 24 — TensorFlow / `ml` adapter disposition *(run 4, three-step chain)*

- **REQ-tensorflow-stays-facade-v1** — M7 Epic 1 PRD §9: `tensorflow_adapter.rs` stays in the
  facade; a `paladin-ml` crate is deferred to M8+.
- **REQ-tensorflow-ml-feature-gate-v2** — M8 Epic 3 PRD §4.3 item 11: the adapter is gated behind a
  new `ml = []` feature.
- **REQ-deferred-tensorflow-ml-adapter-v3** — `deferred-features.md` §2: both the 636-LOC adapter
  and the `ml` flag are **deleted outright** (commit `3d48768`), with a placement condition on
  reintroduction — a dedicated `paladin-ml` leaf crate, never the facade.
- Status: a three-step chain ending in removal, not a two-way disagreement. All three are preserved
  because the v3 *condition* is the live artefact and it is carried only by a DOC. FACADE-03
  records it.
- settled-by: `intel/code-verification.md` run-4 — no `tensorflow` reference and no `ml` feature
  anywhere in `Cargo.toml` or `src/`.

### Group 25 — Milestone 8 Epic 3 scope: defer to M9 versus execute now *(run 4)*

- **REQ-m8-epic3-no-extractions** — M8 Epic 3 PRD §4.3 / §5: "Epic 3 performs **no crate
  extractions**"; every List B move defers to Milestone 9; each adapter group is recorded as an
  active bridge that stays; §5 Non-Goals additionally state "**No new crates created.**
  `paladin-herald`, `paladin-ml`, etc. are not in scope".
- **REQ-m8-reconciliation-relocations** — `facade-cleanup-RECONCILIATION-2026-06-04.md` §3 / §7:
  the deferred relocations are executed inside Milestone 8, `paladin-herald` is created, and both
  the audit and the disposition record are declared to "contain factual errors".
- Status: tree observation — every relocation target exists, `paladin-herald` ships as a
  non-optional facade dependency, and `task-completion-state.md`'s three open Milestone 8 items are
  contradicted by code. HARD-02 records the supersession; the non-goal's `paladin-ml` half still
  holds.
- settled-by: `intel/code-verification.md` run-4.

### Group 26 — `arsenal/` and `sanctum/` Milestone 9 targets *(run 4, internal split)*

- **REQ-adapter-disposition-record (PRD position)** — M8 Epic 3 PRD §4.3 table: `arsenal/` is
  **"M9 extraction candidate? No"** ("MCP wiring is facade composition-root responsibility");
  `sanctum/` is an optional consolidation "→ fold into M9 `paladin-memory` work".
- **REQ-adapter-disposition-record (record position)** —
  `Epic_3/infrastructure-adapter-disposition.md` row 1: `arsenal/` (all 5 adapters) is
  **"Yes (List A) → future `paladin-arsenal` (M9)"**; row 19: `sanctum/` → future
  **`paladin-sanctum`**.
- Status: a single requirement whose two carriers disagree, naming **two crates that do not exist**.
  Neither `paladin-arsenal` nor `paladin-sanctum` is in the tree, and Milestone 9 is recorded 100%
  complete. This lands directly in run-5 scope → FACADE-04.

### Group 27 — notification channel services: move or delete *(run 4)*

- **M8 overview success criteria + Epic 3 Task 3.1** —
  `src/application/notifications/{email_notifications,push_notifications,system_notifications}.rs`
  are **moved** to `crates/paladin-notifications/src/`, "with facade re-exports added if needed for
  backward compatibility", and `cargo test -p paladin-notifications` passes.
- **REQ-dead-file-batch-deletion** — M8 Epic 2 PRD §4.1 Batch 1 **deletes** all three as orphaned
  dead code, on the finding that "`src/application/mod.rs` never declared `pub mod notifications;`,
  making all three files unreachable from the module tree".
- Status: deletion is what shipped — `src/application/` contains only `cli`, `errors`, `mod.rs` and
  `services`. The open question the documents leave behind is narrower: `email_notifications.rs` was
  392 LOC, `facade-audit.md` List A row 1 flagged it "Code has value — review for possible move…
  before deletion", and the Epic 2 PRD's own Open Question 1 asked whether it overlapped the 752-LOC
  `email_notification_adapter.rs`. **Neither the overlap review nor its outcome is recorded
  anywhere.** The file is recoverable from git history.
- settled-by: `intel/code-verification.md` run-4.

### Group 28 — `file_content_repository.rs` disposition *(run 4, three-way)*

- **REQ-paladin-storage-extraction (v1 clause)** — M7 Epic 1 PRD §4.5.2 / §9 Q4: it is **not**
  moved. "Despite its filename, it implements `ContentDeliveryService` / `BatchContentDeliveryService`
  from `paladin-ports` and writes content to the local filesystem; it does not use `sqlx` … It stays
  in the facade crate. A future content-delivery crate (Milestone 8+) is its correct long-term
  home."
- **`facade-audit.md` List B (v2)** — assigns it to **`paladin-storage`** (723 LOC, "File-backed
  content repository").
- **Reconciliation §4 Category 2 / §7 commit `2edc031` (v3)** — **deletes it outright**: "No
  consumers (only its own `mod` line) → delete + drop `pub mod`".
- Status: stays-in-facade, move-to-storage and delete-outright are three futures for one file.
  Deletion shipped. The residue is that the M7 position explicitly reserved a **future
  content-delivery crate** that no later document mentions again — if that idea is still live it
  needs a home in run-5 planning; otherwise the M7 note is retired. HARD-01 records the disposition.
- settled-by: `intel/code-verification.md` run-4.

### Run-3 unsettled positions that are not `-v1`/`-v2` variant pairs

Recorded here so they are not lost, and kept out of the group numbering so the variant arithmetic
stays honest (20 groups / 38 entries).

- **Milestone 6 facade re-export policy** — the milestone overview (DOC) requires backward-compatible
  facade re-exports for the relocated types; `REQ-orchestration-no-reexport-shims` and
  `REQ-circuitbreaker-old-path-retired` (both PRD) forbid them, and Epic 2's own Open Question 4
  undercuts its own Non-Goal. Shipped code follows the PRDs. Decides whether Milestone 6 was a
  non-breaking internal refactor or a breaking change requiring a major version bump → ARCH-04.
- **`vision` feature scope** — `REQ-vision-feature-gating` (PRD) requires `vision` to gate
  `chacha20poly1305` and `zeroize`; `Epic_1/dependency-matrix.md` (DOC, and the Task 1.1 audit
  deliverable of that same epic) classifies both as general-purpose encryption dependencies that
  must stay unconditional. Precedence favours the PRD, but applying it would break
  `cargo build --no-default-features`. Shipped `vision = []` gates nothing, so the audit was right
  → ARCH-05(1).
- **Milestone/tier numbering** — the same class of defect as the run-2 Milestone 3 epic-numbering
  conflict, spanning 9 of the 19 run-3 DOCs → ARCH-02.
- **Build-benchmark verdict** — `build-benchmarks.md` both fails and passes Milestone 5 SM-7
  → ARCH-07.

### Run-4 unsettled positions that are not `-v1`/`-v2` variant pairs

Same treatment as run 3: recorded, and kept out of the group numbering so the variant arithmetic
stays honest (28 groups / 56 entries).

- **The RustSec exception set** — four surfaces, four sets (2 documented / 5 in `.cargo/audit.toml`
  / 15 in `deny.toml` / 2 inline at `ci.yml:406`, with a second bare-`cargo audit` job at
  `ci.yml:77`), and `deny.toml`'s own stated sync invariant violated. Not a `-v1`/`-v2` pair because
  no two documents disagree — the *tree* disagrees with itself → SEC-01.
- **The project's licence posture** — `MIT OR Apache-2.0` (signed checklist, named approver) versus
  MIT (PRD, overview, shipped `Cargo.toml`) → SEC-02.
- **The extracted-crate dependency rule** — stated absolutely in M7 Epic 1 §6.1, anticipated and
  unamended in §4.4 of the same PRD, violated once by `paladin-content → paladin-llm` behind an
  optional feature → HARD-05.
- **The `pdf` capability** — `pdf = []` gates nothing and `content-processing` omits it, while
  `.cargo/audit.toml` suppresses an advisory on the grounds that `pdf-extract` is in the graph
  → HARD-06.
- **The `cargo doc` bar** — zero warnings (M7 Epic 4 §4.4.3, M7 Epic 1 §4.6.4) versus warnings
  acceptable (M8 Epic 5 FR-19), on the same command → HARD-07.
- **Milestone/tier numbering, fourth instance** — the M7 overview titles itself "Milestone 4"
  → HARD-04.

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


### Superseded but preserved — run-3 later positions on run-1 and run-2 requirements

Milestones 4-6 exist to restructure Milestones 1-3, so this table is longer than run 2's and every
row is intentional. **Relocation is not contradiction.** In every case the earlier entry stays
intact in its ledger.

| Earlier position | Run-3 later position | Nature of the change |
|---|---|---|
| Every `src/{core,application,infrastructure}` path in the run-1 and run-2 corpus | `REQ-cargo-workspace-root`, `REQ-paladin-core-scaffold`, `REQ-paladin-ports-scaffold`, `REQ-battalion-crate-scaffold`, `REQ-llm-crate-scaffold`, `REQ-memory-crate-scaffold` | Single-crate monolith → Cargo workspace. Those paths are historical; resolve current locations through `.planning/codebase/` or the tree |
| `REQ-core-container-extraction` (Maneuver lexer/AST/parser moved *into* `paladin-core`) | `REQ-maneuver-files-moved-from-core`, `REQ-core-maneuver-cleanup` | Moved *out* again into `paladin-battalion` one milestone later. Both moves shipped; the second is current |
| Run-1/run-2 references to `src/application/use_cases/paladin/circuit_breaker.rs` | `REQ-circuitbreaker-relocation`, `REQ-circuitbreaker-old-path-retired` | Relocated to `src/infrastructure/resilience/circuit_breaker.rs`; the old path is **intentionally** unresolvable, with no re-export |
| Run-1/run-2 references to `src/config/application_settings.rs` | `REQ-config-domain-modules`, `REQ-config-incremental-migration` | File deleted; replaced by per-domain config modules split across the facade, `paladin-memory` and `paladin-llm` |
| Run-1/run-2 references to `src/core/platform/manager/{notification_service,queue_service,log_service,orchestrator,listener_service,scheduler}.rs` | `REQ-six-service-relocation`, `REQ-orchestrator-renaming` | Six services relocated and renamed to `*Orchestrator`. Shipped under `src/application/services/`, not the PRD's `src/application/use_cases/` |
| Run-1 `REQ-paladin-port` / `REQ-llm-port-interface` type homes | `REQ-port-value-type-ownership-v1` | `PaladinResult`, `StopReason`, `TokenUsage`, `RegistryError`, `HandoffError` moved to `paladin-core`; ports became thin re-exports. `PaladinError` deliberately excluded. See group 19 |
| Run-1 `REQ-mcp-sse-transport` and the proposed `mcp-arsenal` / `mcp-stdio` / `mcp-sse` flags | `REQ-feature-flag-matrix` (dated 2026-04-15 elimination note) | MCP feature-gating eliminated from scope: Arsenal and its transports compile unconditionally. No MCP feature flag exists in the tree |
| Run-1 default features `["redis-queue", "s3-storage", "openai-embeddings"]` | `REQ-feature-default-set` | Replaced by `default = ["llm-openai"]` — an intentional breaking change with no deprecation cycle |
| Run-1/run-2 benchmark paths (`benches/paladin_benchmarks.rs`, `herald_benchmarks.rs`, `arsenal_benchmarks.rs`) | The M5 per-crate `benches/` layout | Already recorded in QUAL-05; run 3 supplies the requirement behind the move |
| Run-2 `REQ-herald-type-consolidation` (single source of truth for shared domain types) | `REQ-port-value-type-ownership-v1` moved one `TokenUsage`; two more remain | **Not** a completed supersession — the consolidation is still open. Tracked as DEBT-05 |
| `REQ-battalion-facade-shim` (M5 Epic 3: keep a re-export shim so old paths resolve) | `REQ-orchestration-no-reexport-shims`, `REQ-circuitbreaker-old-path-retired` (M6) | The shim posture **flipped** between milestones. Which posture governs Milestone 6 is an open decision → ARCH-04 |


### Superseded but preserved — run-4 later positions on run-1 to run-3 requirements

Run 4 is the first run where a **document supersedes another document by name**:
`facade-cleanup-RECONCILIATION-2026-06-04.md` carries
`Supersedes (corrects): Epic_1/facade-audit.md and Epic_3/infrastructure-adapter-disposition.md`.
Every earlier entry stays intact in its ledger.

| Earlier position | Run-4 later position | Nature of the change |
|---|---|---|
| `REQ-paladin-web-extraction` (actix-web + axum as direct deps) | `REQ-actix-removal`, `REQ-actix-deny-ban` | Framework **removed and banned** one milestone after it was mandated; see variant group 21 |
| `REQ-storage-feature-flags-v1` (`storage-sqlite`, optional `paladin-storage`) | `REQ-storage-nonoptional-v2` | Flag retired, dependency made non-optional; see group 22 |
| `REQ-tensorflow-stays-facade-v1` → `REQ-tensorflow-ml-feature-gate-v2` | `REQ-deferred-tensorflow-ml-adapter-v3` | Stays → gated → deleted, with a `paladin-ml` placement condition on return; see group 24 |
| `REQ-m8-epic3-no-extractions`, `REQ-adapter-disposition-record` | `REQ-m8-reconciliation-relocations` | Relocations deferred to M9 were **executed in M8**; both source documents declared factually wrong; see group 25 → HARD-02 |
| `REQ-paladin-content-extraction` (target dir `use_cases/`) | `REQ-paladin-content-services-rename` | M8 Epic 6 renamed it to `services/`; the facade bridge had been updated and the leaf crate had not, breaking `content-processing` |
| M8 overview Epic 4 Task 4.3 (optional `pub use services as use_cases;`) | `REQ-rename-clean-break` | Explicitly rejected by the Epic 4 PRD: "there will be no `pub use services as use_cases;`" |
| `REQ-paladin-storage-extraction` (`file_content_repository.rs` stays) | reconciliation commit `2edc031` | Deleted outright; the "future content-delivery crate" it reserved is unmentioned since; see group 28 |
| M8 Epic 3 §5 Non-Goal "No new crates created — `paladin-herald`, `paladin-ml` … not in scope" | reconciliation commit `66f6c4e` | **Overridden for `paladin-herald`** inside the same milestone; **still holding for `paladin-ml`** → HARD-02(e), FACADE-03(b) |
| M7 Epic 4 PRD / overview Appendix C target of lockstep `0.2.0` | tag `v0.1.0-rc.1` at `a9530fc`, all ten crates published at `0.1.0` | Superseded by outcome; the tree has since moved to `0.6.0` on `release/v0.7.0` → HARD-03 |
| M7 Epic 4 §4.6 / M8 Epics 2, 4, 5 root-path `STABLE_API.md`; `docs/{PERFORMANCE_BASELINE,RELEASE_CHECKLIST,VERSIONING_POLICY,BUILD_BASELINES,INTEGRATION_TESTS}.md` | mdbook pages under `docs/src/{api-reference,appendix}/` | Same Milestone 11 relocation ARCH-05 already records for the run-3 deliverables. **Relocated, not missing** — do not plan them |
| `REQ-ci-publish-dry-run-v1` (per-crate, dependency-ordered) | `REQ-ci-publish-dry-run-v2` (workspace-wide) | **Not a supersession** — both ship, in different workflows; see group 23 |

---

## Milestone 1 as-shipped ledger

All 115 requirement IDs extracted by ingest run 1, with verified status. **Not forward scope** —
listed so nothing is lost and so runs 4-5 merge against stable keys. Acceptance criteria are not
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
`src/core|application|infrastructure` layout. The workspace was decomposed in Milestone 5
(ingested in run 3) into what is now ten library crates plus the root facade, so the `src/...`
paths in those PRDs are historical. Citations below are the **current** locations, verified by
direct inspection of `release/v0.7.0`.

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

## Milestone 4-6 as-shipped ledger

All 115 requirement IDs extracted by ingest run 3 — Milestone 4 (feature flags, port-trait
hardening, CLI isolation), Milestone 5 (the Cargo workspace decomposition) and
Milestone 6 (config decomposition, orchestration relocation, Maneuver co-location, CircuitBreaker
move). **Not forward scope.** Acceptance criteria live in `.planning/intel/requirements.md`.

**This ledger is the best-evidenced of the three.** 22 of its claims were verified directly
against `Cargo.toml` contents, type definitions and file existence during ingest run 3, and are
recorded in `intel/code-verification.md`; a further set was re-confirmed on 2026-07-30. Where a
verdict says *verified*, it means the tree was read, not that a document asserted it.

**Read this section with two caveats.** First, the `src/…` paths in these PRDs are themselves
partly historical — Milestone 6 moved what Milestone 5 had just placed, and milestones outside
this run moved more. Citations below are **current** locations. Second, these milestones exist to
restructure Milestones 1-3, so a run-3 entry contradicting a run-1 or run-2 path is supersession,
not conflict; the chains are in *Superseded but preserved* above.

Status key: `Shipped` = the named artefact exists at the cited path · `Shipped (relocated)` =
shipped at a different path than the PRD specified · `Shipped, superseded` = shipped and later
undone or replaced by a subsequent milestone · `Verify` = related code exists but the
requirement's specific criteria were not inspected → ARCH-01 · `Variant` = see competing variants
· `Code diverges` = shipped implementation deliberately differs from the ingested requirement ·
`Open defect → X` = verified open work.

### Milestone 4 Epic 1 — Feature Flag Expansion (7 IDs)

Epic-level note: **fully checked in `task-completion-state.md`, and corroborated** — the feature
matrix ships. Two PRD clauses are contradicted by code and must not be applied literally.

| ID | Status |
|---|---|
| REQ-feature-flag-matrix | Shipped, partly diverged — root `Cargo.toml [features]` carries `llm-openai`, `llm-anthropic`, `llm-deepseek`, `llm-all`, `content-processing`, `web-server`, `notifications`, `vision`, and retains `redis-queue`, `s3-storage`, `openai-embeddings`, `qdrant`, `integration-tests`. Divergences: `web-server = ["dep:paladin-web", "dep:axum"]` gates axum only (actix-web is no longer a root dependency), and **no MCP feature flag of any kind exists** — the PRD's dated 2026-04-15 elimination note is what shipped → ARCH-05 |
| REQ-vision-feature-gating | **Code diverges (verified)** — `vision = []` gates no dependency at all; `chacha20poly1305` and `zeroize` are unconditional, matching the Epic 1 `dependency-matrix.md` audit rather than the PRD. Applying the PRD would break `cargo build --no-default-features` for user auth and Citadel encryption → ARCH-05(1) |
| REQ-feature-default-set | Shipped — `default = ["llm-openai"]` in root `Cargo.toml`. The `CHANGELOG.md` migration-guide and example-update clauses not inspected → ARCH-01 |
| REQ-feature-full-flag | Shipped — `full = ["llm-all", "content-processing", "web-server", "notifications", "storage", "vision", "redis-queue", "s3-storage", "openai-embeddings", "qdrant", "cli"]` |
| REQ-cfg-guard-discipline | Verify — the provider factory moved to `crates/paladin-llm/src/provider_factory.rs` (the PRD's `src/infrastructure/adapters/llm/provider_factory.rs` is historical); guard placement and the no-`#[allow(dead_code)]` rule not inspected → ARCH-01 |
| REQ-feature-flag-docs | Shipped (relocated) — the named `/docs/CONFIGURATION.md`, `/docs/FEATURE_FLAGS.md` and `/docs/MIGRATION.md` do not exist; equivalent pages ship as mdbook chapters at `docs/src/api-reference/{feature-flags,migration-guide}.md` and `docs/src/getting-started/installation.md`. **Not a missing deliverable** → ARCH-05 |
| REQ-feature-ci-matrix | Shipped — `.github/workflows/feature-flags.yml` runs a workspace-scoped feature matrix (`:115`, `:118`) |

### Milestone 4 Epic 2 — Port Trait Hardening & Stable API (9 IDs)

Epic-level note: **the one genuinely incomplete epic in run-3 scope.**
`tasks-harden-port-traits-stable-api.md` carries 20 open items — the only open items in all of
Milestone 4 — and unlike every other count in this corpus they are **corroborated by code**.

| ID | Status |
|---|---|
| REQ-curated-lib-exports | Verify — `src/lib.rs` and `src/prelude.rs` ship; whether the four glob re-exports were removed and the ~20 port traits explicitly exported was not inspected → ARCH-01 |
| REQ-visibility-hardening | Verify — not inspected → ARCH-01 |
| REQ-port-trait-rustdoc | **Partial → DEBT-03** — rustdoc exists on the port traits, but its `# Examples` cannot execute: `crates/paladin-ports/Cargo.toml:18` sets `doctest = false` and `ci.yml:225` excludes the crate from `--doc` |
| REQ-stable-api-doc | Shipped (relocated) — `docs/src/api-reference/stable-api.md`, not root `STABLE_API.md`. Six run-3 documents cross-reference the old path → ARCH-05 |
| REQ-import-path-updates-m4 | Verify — not inspected → ARCH-01 |
| REQ-doc-build-clean | Shipped — `ci.yml:57` runs `cargo doc --workspace --no-deps` and fails on any warning |
| REQ-api-surface-ci | **Open defect → DEBT-01** — the tooling exists (`scripts/{extract-public-api,check-api-surface,check-deprecations,check-all-examples}.sh`, `final-api.txt`, `api_surface_current.txt`) but the job fails on every run: `ci.yml:171,181,186` and both script defaults point at `project/current-exports.txt`, which was renamed to `.project/` in commit `928c6d5`. FR-7.3's `.public-api-baseline.txt` was never created |
| REQ-deprecation-warnings | **Open defect → DEBT-02** — `grep -rn '#\[deprecated' src crates` returns 0; `Epic_2/DEPRECATIONS.md` self-reports "Deprecated Items: 0 (none yet)" |
| REQ-api-surface-reduction-target | Verify — the ≤ 50-exported-type target has no recorded measurement, and the epic's own `api-audit.md` states a different target (104-124 types / 40-50% reduction) → ARCH-01, DEBT-02 |

### Milestone 4 Epic 3 — CLI Isolation (9 IDs)

Epic-level note: task list fully checked, but code shows **the inverse of the run-1/run-2
pattern** — here the checkboxes overstate completion.

| ID | Status |
|---|---|
| REQ-cli-feature-gate | Shipped — `cli = ["dep:clap", "dep:dialoguer", "dep:indicatif", "dep:console", "dep:serde_yaml"]` at `Cargo.toml:284`; `[[bin]] paladin-cli` carries `required-features = ["cli"]` |
| REQ-cli-dependency-isolation | **Open defect → DEBT-04** — 5 of 8 CLI-only dependencies gated; `structopt` (`:93`), `colored` (`:125`) and `comfy-table` (`:126`) remain unconditional |
| REQ-binary-target-config | Shipped, **question formally open** — three binary targets ship (`paladin`, `paladin-cli`, `paladin-server`), i.e. Option A extended, but Q1's "architecture review" never happened and FR9.3's documentation deliverable was never produced → ARCH-06 |
| REQ-cli-test-isolation | Shipped — `feature-flags.yml:141` runs `cargo test --test cli_isolation` |
| REQ-library-only-build | **Partial → DEBT-04** — the library builds, but `cargo tree --lib --no-default-features` still contains three CLI-only crates, so FR5.4 is unmet |
| REQ-library-only-integration-tests | Shipped — the `cli_isolation` test target exists; the ≥ 5-test minimum and the four-Battalion-pattern coverage not inspected → ARCH-01 |
| REQ-cli-build-time-measurement | Verify — no measurement located; any figure taken today would be measured against a partly-isolated tree → ARCH-01, DEBT-04 |
| REQ-cli-ci-matrix | Shipped — `feature-flags.yml` covers the library-only and CLI-enabled combinations |
| REQ-cli-docs | **Partial → ARCH-06** — FR9.3's binary-architecture documentation was never produced; the README/CONTRIBUTING/CHANGELOG clauses not inspected → ARCH-01 |

### Milestone 5 Epic 1 — Workspace Initialization & `paladin-core` (10 IDs)

Epic-level note: 5 open checkboxes, **contradicted** — the workspace and the crate both ship.
This epic also produced the corpus's only decision record.

| ID | Status |
|---|---|
| REQ-cargo-workspace-root | Shipped — root `Cargo.toml` declares `[workspace] members = [".", "crates/*"]`, `resolver = "2"` and a full `[workspace.dependencies]` block pinning every shared crate plus all ten member crates |
| REQ-workspace-crate-edition-v1 | Variant (group 17) — tree is mixed; only `paladin-ports` and `paladin-notifications` are on 2021 |
| REQ-paladin-core-scaffold | Shipped — `crates/paladin-core/` (package name `paladin-ai-core`, lib name `paladin_core`) |
| REQ-paladin-core-dependency-allowlist-v1 | Variant (group 18) — the "exhaustive" six ships as 14 |
| REQ-core-base-extraction | Shipped — `crates/paladin-core/src/base/` |
| REQ-core-container-extraction | Shipped, **partly superseded** — `crates/paladin-core/src/platform/container/`; the clause including the Maneuver lexer/AST/parser was reversed by `REQ-maneuver-files-moved-from-core` one milestone later |
| REQ-core-upward-dependency-resolution | Shipped — resolved by the Epic 1 decision record (Option A); the decision artefact is committed under `Epic_1/decisions/`, satisfying the "committed decision artifact" criterion |
| REQ-port-value-type-ownership-v1 | Variant (group 19) — **this is the shipped shape**: `paladin-core/src/platform/container/{execution_result,token_usage,registry_error}.rs` and `arsenal/handoff_error.rs` all exist |
| REQ-facade-core-reexports | Shipped — `src/core/` is a re-export layer over `paladin-core` (`codebase/STRUCTURE.md:126`) |
| REQ-core-dependency-validation | Verify — no committed `cargo tree -p paladin-core` evidence artefact was located → ARCH-01 |

### Milestone 5 Epic 2 — `paladin-ports` Extraction (10 IDs)

| ID | Status |
|---|---|
| REQ-paladin-ports-scaffold | Shipped — `crates/paladin-ports/`; the crate declares no features of its own, matching the §9 resolution. Ships 10 dependencies against a declared allowlist of 7 (see group 18) |
| REQ-output-ports-extraction | Shipped — `crates/paladin-ports/src/output/`, including both `vision_llm_port.rs` and `vision_port.rs` unconditionally |
| REQ-input-ports-extraction | Shipped — `crates/paladin-ports/src/input/` (e.g. `document_port.rs`) |
| REQ-ports-facade-wiring | Shipped — `src/application/ports/` **does not exist**; full deletion (Option B) is what shipped |
| REQ-ports-import-migration | Shipped by implication of the deletion; the 314-occurrence / 76-file sweep not re-counted → ARCH-01 |
| REQ-ports-doctest-compilation | **Open defect → DEBT-03** — `[lib] doctest = false` with a comment deferring the fix to an unwritten "Task 7.0"; `ci.yml:225` excludes the crate |
| REQ-ports-docs-markdown-update | Verify — the 12 `application::ports::` occurrences across 5 `docs/` files not re-checked, and the docs themselves were reorganised into the mdbook afterwards → ARCH-01 |
| REQ-ports-layering-validation | Verify — the substantive invariant holds (no infra SDK in the crate), but the two named evidence artefacts under `project/Milestone_5-Workspace-Decomposition/Epic_2/` were not located after the `.project` rename → ARCH-01 |
| REQ-ports-tests-and-rustdoc | **Partial → DEBT-03** — rustdoc is preserved but "no documentation may be lost" is undermined by disabled doctests |
| REQ-port-value-type-ownership-v2 | Variant (group 19) — **not** the shipped shape; FR-10 applied literally would reintroduce the upward dependency Epic 1 removed |

### Milestone 5 Epic 3 — `paladin-battalion` Extraction (9 IDs)

Epic-level note: 4 open checkboxes, **contradicted** — the crate and all nine services ship.

| ID | Status |
|---|---|
| REQ-battalion-crate-scaffold | Shipped — `crates/paladin-battalion/` |
| REQ-battalion-service-extraction | Shipped — the nine execution services and the three support utilities are in `crates/paladin-battalion/src/` |
| REQ-battalion-import-migration | Shipped by implication — the crate compiles in isolation in the `crate-isolation` CI job |
| REQ-battalion-inline-tests | Verify — the 12 inline test modules not re-counted → ARCH-01 |
| REQ-battalion-facade-shim | **Shipped, then superseded** — the shim lived at `src/application/use_cases/battalion/mod.rs`; `src/application/use_cases/` no longer exists at all after Milestone 6, so the backward-compatible paths this requirement guaranteed are gone. Whether that is acceptable is the open re-export-policy question → ARCH-04 |
| REQ-battalion-dependency-validation | Verify — the two named evidence artefacts not located → ARCH-01 |
| REQ-battalion-example-verification | Verify — the six named examples not individually re-checked → ARCH-01 |
| REQ-battalion-crate-docs | Verify — crate-level `//!` docs and the `petgraph` version-alignment check not inspected → ARCH-01 |
| REQ-paladin-core-dependency-allowlist-v2 | Variant (group 18) — `petgraph` is confirmed present in `paladin-core`, so this later position matches the tree |

### Milestone 5 Epic 4 — `paladin-llm` Extraction (11 IDs)

Epic-level note: 1 open checkbox, **contradicted** — the crate, all three providers, the mock and
the factory all ship.

| ID | Status |
|---|---|
| REQ-llm-crate-scaffold | Shipped — `crates/paladin-llm/` with per-provider features; the root crate enables `openai, anthropic, deepseek, mock, vision` |
| REQ-llm-provider-error | Verify — `LlmProviderError` variant list and the `From<LlmProviderError> for LlmError` boundary not inspected → ARCH-01 |
| REQ-openai-provider-extraction | Shipped — `crates/paladin-llm/src/openai/{adapter,embedding,vision}.rs` |
| REQ-anthropic-provider-extraction | Shipped — `crates/paladin-llm/src/anthropic/{adapter,vision}.rs` |
| REQ-deepseek-provider-extraction | Shipped — `crates/paladin-llm/src/deepseek/adapter.rs` |
| REQ-llm-mock-adapters | Shipped — `crates/paladin-llm/src/mock.rs`; `MultiStepMockLlmPort`'s existence not re-checked → ARCH-01 |
| REQ-llm-provider-factory | Shipped — `crates/paladin-llm/src/provider_factory.rs` |
| REQ-llm-config-bridge-location-v1 | Variant (group 20) — **not** the shipped shape |
| REQ-llm-test-architecture | Verify — the five named per-provider integration test files and the no-network rule not inspected → ARCH-01 |
| REQ-llm-facade-prelude | Shipped — `src/prelude.rs`; the deliberate non-preservation of old deep import paths is recorded as a supersession, not a regression |
| REQ-llm-build-validation | Verify — `feature-flags.yml` exercises a workspace feature matrix, but the numeric targets (`--no-default-features` under 5 s, ≥ 50% incremental improvement) have no recorded measurement → ARCH-01, ARCH-07 |

### Milestone 5 Epic 5 — `paladin-memory` Extraction (10 IDs)

Epic-level note: the only Milestone 5 epic whose `Cargo.toml` was verified line-for-line against
its PRD — FR-1.2, FR-1.3 and FR-1.6 match exactly.

| ID | Status |
|---|---|
| REQ-memory-crate-scaffold | Shipped — `crates/paladin-memory/Cargo.toml` declares `edition = "2024"`, `[lib] doctest = false` and exactly `default = []`, `sqlite`, `qdrant`, `content-processing`; `sqlx` and `qdrant-client = "1.14"` are hoisted into `[workspace.dependencies]` |
| REQ-workspace-crate-edition-v2 | Variant (group 17) — this is the position the tree follows for every crate except two |
| REQ-memory-module-structure | Shipped — `garrison`, `sanctum`, `services` and a prelude |
| REQ-garrison-adapter-extraction | Shipped — `crates/paladin-memory/src/garrison/`; `#[doc(hidden)]` removal not inspected → ARCH-01 |
| REQ-sanctum-adapter-extraction | Shipped — `crates/paladin-memory/src/sanctum/{in_memory_adapter,qdrant_adapter}.rs` |
| REQ-memory-services-extraction | Shipped — `crates/paladin-memory/src/services/{memory_extraction_service,rag_retrieval_service}.rs`; the crate takes no `paladin-llm` dependency, as required |
| REQ-memory-originals-deletion | Shipped — the monolith `src/infrastructure/adapters/{garrison,sanctum}/` and `src/application/use_cases/sanctum/` directories are gone |
| REQ-memory-facade-reexports | Verify — the named `pub use paladin_memory::…` paths not individually re-checked → ARCH-01 |
| REQ-memory-test-migration | Verify — inline-vs-workspace test placement not inspected → ARCH-01 |
| REQ-memory-build-gates | Verify — the five per-feature build combinations are covered in spirit by `crate-isolation` and `feature-flags.yml`; not individually confirmed → ARCH-01 |

### Milestone 5 Epic 6 — Workspace Finalization (6 IDs)

Epic-level note: 7 open checkboxes, **mostly contradicted** — the prelude, the CI job and the
benchmark script all ship.

| ID | Status |
|---|---|
| REQ-facade-reexport-audit | Verify — `paladin-core` and `paladin-ports` are present in `[workspace.dependencies]` as required; the audit checklist artefact was not located → ARCH-01 |
| REQ-paladin-prelude | Shipped — `src/prelude.rs`; the ~30-name content list not individually checked → ARCH-01 |
| REQ-devcontainer-gh-cli | Verify — `.devcontainer/Dockerfile.dev` not inspected → ARCH-01 |
| REQ-crate-isolation-ci | Shipped — the `crate-isolation` job exists at `.github/workflows/ci.yml:228` |
| REQ-workspace-ci-upgrade | **Partial → DEBT-01** — `--workspace` scoping shipped (`ci.yml:54,57,222,225`; `feature-flags.yml:115,118`), but the toolchain-action upgrade is incomplete: four `actions-rs/toolchain@v1` steps remain (`ci.yml:147,317,507`, `integration-tests.yml:71`), and `ci.yml:225` carries the `--exclude paladin-ports` doc-test exclusion (DEBT-03) |
| REQ-build-benchmark-report | Shipped, **verdict contested → ARCH-07** — `scripts/benchmark-builds.sh` and `Epic_6/build-benchmarks.md` both exist, but the report's summary table fails SM-7 in four of five scenarios while its conclusion declares the target achieved |

### Milestone 6 Epic 1 — `application_settings.rs` Decomposition (8 IDs)

Epic-level note: Milestone 6 shows **0 open checkboxes and code confirms it** — all four
relocations are complete.

| ID | Status |
|---|---|
| REQ-config-domain-modules | Shipped (hybrid) — `src/config/{agents,arsenal,citadel,env_utils,file_storage,herald,notifications,queue,scheduler,settings,web_server}.rs` plus `crates/paladin-memory/src/config/{garrison,rag,sanctum}.rs` and `crates/paladin-llm/src/config/{llm,vision,bridge}.rs`. Neither the PRD map nor the overview map matches exactly: `agents.rs` is the overview's filename, `settings.rs` is an addition, and `battalion.rs` / `logging.rs` / `llm.rs` / `garrison.rs` are absent from `src/config/` → ARCH-01 |
| REQ-env-overridable-trait | Shipped — `src/config/env_utils.rs`; the `read_env` unit-test matrix not inspected → ARCH-01 |
| REQ-settings-root-struct | Verify — `src/config/settings.rs` exists; whether the `Settings` public API is byte-identical was not inspected → ARCH-01 |
| REQ-config-incremental-migration | Shipped — `application_settings.rs` is **deleted**; the migration's intermediate-state discipline is unverifiable after the fact |
| REQ-config-yml-backcompat | Verify — no `config.test.yml` before/after regression test was located. `intel/constraints.md` names this deserialization contract one of the two strongest SPEC re-tag candidates in the corpus → ARCH-01 |
| REQ-rag-config-dedup | Shipped — `crates/paladin-memory/src/config/rag.rs` is the canonical location |
| REQ-config-success-metrics | Verify — the 400-line-per-file ceiling not re-measured → ARCH-01 |
| REQ-llm-config-bridge-location-v2 | Variant (group 20) — **this is the shipped shape** (`crates/paladin-llm/src/config/bridge.rs`) |

### Milestone 6 Epic 2 — Orchestration Service Relocation (9 IDs)

| ID | Status |
|---|---|
| REQ-orchestration-target-structure | Shipped (relocated) — the four orchestrator module groups ship under `src/application/services/{notification_orchestrator,queue_orchestrator,log_orchestrator,orchestration}/` with the PRD's exact module names; `src/application/use_cases/` no longer exists → ARCH-05(5) |
| REQ-six-service-relocation | Shipped — all six moved; `src/core/platform/manager/` no longer declares any of them |
| REQ-domain-type-placement-rules | Verify — the mechanical placement rules were applied but not audited per type → ARCH-01 |
| REQ-manager-services-retained | Shipped — `src/core/platform/manager/` retains exactly `content_service.rs`, `event_manager.rs`, `user_service.rs` and `mod.rs`. The deferred full `user_service` relocation remains an open question the PRD itself flagged |
| REQ-orchestration-consumer-import-updates | Verify — the named consumer files not individually re-checked → ARCH-01 |
| REQ-orchestrator-renaming | Verify — module names match; whether every type was renamed to `*Orchestrator` not inspected → ARCH-01 |
| REQ-core-isolation-verification | Verify — `crate-isolation` builds `paladin-core` independently, which is strong indirect evidence; no `cargo tree` artefact was located → ARCH-01 |
| REQ-orchestration-test-coverage | Verify — per-type `#[cfg(test)]` blocks for relocated domain types not inspected → ARCH-01 |
| REQ-orchestration-no-reexport-shims | Shipped, **policy contested → ARCH-04** — no shims exist, matching the PRD and contradicting the milestone overview. The PRD's own Open Question 4 leaves this unsettled |

### Milestone 6 Epic 3 — Maneuver DSL Co-location (9 IDs)

| ID | Status |
|---|---|
| REQ-maneuver-submodule-structure | Shipped — `crates/paladin-battalion/src/maneuver/{mod.rs,parser/{mod,lexer,ast,error}.rs,service.rs,visualizer.rs}` |
| REQ-maneuver-files-moved-from-core | Shipped — no `parser/` directory and no `maneuver.rs` remain in `paladin-core`. **Supersedes** the Maneuver clause of `REQ-core-container-extraction` |
| REQ-maneuver-files-reorganized | Shipped — the flat `maneuver_service.rs` and `flow_visualizer.rs` are gone, replaced by `maneuver/service.rs` and `maneuver/visualizer.rs` |
| REQ-maneuver-inline-tests | Verify — the ≥ 35 inline test count in `src/maneuver/` not re-counted → ARCH-01 |
| REQ-core-maneuver-cleanup | Shipped — `paladin-core`'s `battalion/mod.rs` retains the pattern types and no parser references |
| REQ-maneuver-facade-reexports | Verify — the explicit `pub mod container` block and the `parser` / `maneuver` forwarding modules not inspected → ARCH-01 |
| REQ-maneuver-battalion-import-updates | Verify — the inline fully-qualified paths in `commander.rs` not re-grepped → ARCH-01 |
| REQ-maneuver-battalion-lib-exports | Verify — crate-root re-exports in `paladin-battalion/src/lib.rs` not inspected → ARCH-01 |
| REQ-maneuver-cargo-dependency-check | Verify — no new dependencies expected; the `paladin-core` dependency-pruning clause not inspected → ARCH-01 |

### Milestone 6 Epic 4 — `CircuitBreaker` Relocation to Infrastructure (8 IDs)

Epic-level note: this PRD closed the milestone overview's three-way choice by selecting Option A
and explicitly rejecting a `paladin-infra` crate and a `CircuitBreakerPort` trait, and recorded a
reasoned acceptance of the resulting layering inversion. It is decision-shaped but PRD-typed, so
it creates no locked decision.

| ID | Status |
|---|---|
| REQ-resilience-module-structure | Shipped — `src/infrastructure/resilience/mod.rs` |
| REQ-circuitbreaker-relocation | Shipped — `src/infrastructure/resilience/circuit_breaker.rs` |
| REQ-circuitbreaker-rustdoc-updates | Verify — doc-example import paths not inspected → ARCH-01 |
| REQ-paladin-execution-service-import | Shipped by implication — `src/application/use_cases/` no longer exists; the execution service lives under `src/application/services/paladin/` and the workspace builds → ARCH-01 |
| REQ-circuitbreaker-example-updates | Verify — the 15 named examples and the README code sample not individually re-checked → ARCH-01 |
| REQ-circuitbreaker-test-updates | Verify — the three CLI test files not inspected → ARCH-01 |
| REQ-circuitbreaker-old-path-retired | Shipped, **policy contested → ARCH-04** — the old path is unresolvable and no re-export exists, matching the PRD and contradicting the milestone overview |
| REQ-circuitbreaker-stable-api-update | Verify — after the mdbook relocation this applies to `docs/src/api-reference/stable-api.md`, not root `STABLE_API.md`; `final-api.txt` and `api_surface_current.txt` exist but regeneration was not confirmed → ARCH-05, DEBT-01 |

---

## Milestone 7-8 as-shipped ledger

All 86 requirement IDs extracted by ingest run 4 — Milestone 7 (four more crate extractions,
production build infrastructure, benchmark migration, API stabilization through a release
candidate) and Milestone 8 (facade cleanup, dead-shim removal, the `use_cases` → `services` rename,
the single-web-framework consolidation, and the 2026-06-04 reconciliation that executed what Epic 3
had deferred). **Not forward scope.** Acceptance criteria live in `.planning/intel/requirements.md`.

**This is the best-evidenced ledger of the four.** Run 4 is the only run whose corpus contains a
document that audits itself against the tree — `facade-cleanup-RECONCILIATION-2026-06-04.md` — and
its claims are corroborated almost without exception. `deferred-items.md` D5's count of 17
`println!`/`eprintln!`/`dbg!` occurrences across 6 files matches the tree **exactly**. That is the
strongest reliability signal in the 153-document corpus and the reason the two deferred registers,
not checkbox arithmetic, are the Milestone 8 forward-work source.

**Two dispositions are new in this ledger.** `Superseded by outcome` marks the 14 requirements that
must **not** be planned as written because shipped code went a different way — the single largest
such block in the corpus. `Deferred with register` marks work that was removed deliberately and
recorded with a reintroduction condition, which is different from work that was never done.

Status key (extends the run-3 key): `Shipped` · `Shipped (relocated)` · `Shipped, superseded` ·
`Superseded by outcome` = do not plan as written · `Deferred with register` = removed on purpose,
condition recorded · `Verify` → HARD-01 · `Variant` · `Code diverges` · `Open defect → X`.

### Milestone 7 Epic 1 — Extended Workspace Decomposition (12 IDs)

Epic-level note: the Epic 1 cost-benefit gate returned **four Go, zero Defer**, and all four crates
ship. Two of its clauses were reversed by Milestone 8 inside the same corpus.

| Requirement | Verdict |
|---|---|
| REQ-m7-cost-benefit-gate | Shipped — `cost-benefit-assessment.md` produced with a self-approval block dated 2026-05-25; four Go decisions, so PRD sub-tasks 1.4/1.5 (mark deferred, create backlog tickets) were correctly recorded N/A |
| REQ-paladin-web-extraction | **Shipped, superseded** — `crates/paladin-web/` exists; its two-framework clause is reversed by `REQ-actix-removal`. Variant group 21 |
| REQ-paladin-notifications-extraction | Shipped — `crates/paladin-notifications/` with README and CHANGELOG; the per-feature `email`/`push`/`system` criteria were not individually re-checked → HARD-01 |
| REQ-paladin-content-extraction | **Shipped (relocated)** — crate exists; its `use_cases/` target directory was renamed to `services/` by M8 Epic 6, and `content_ingestion_service.rs` stayed in the facade → deferred item D4 (FACADE-02) |
| REQ-paladin-storage-extraction | **Shipped, superseded** — crate exists; `file_content_repository.rs` was deleted rather than kept in the facade. Variant group 28 |
| REQ-storage-feature-flags-v1 | **Superseded by outcome** — `storage-sqlite` retired, `paladin-storage` non-optional. Variant group 22 |
| REQ-facade-workspace-metadata | Verify — all four crates are in `[workspace.members]` and `[workspace.dependencies]`; the "no public API paths may be silently removed" clause was not audited → HARD-01 |
| REQ-extracted-crate-dependency-rule | **Code diverges → HARD-05** — `crates/paladin-content/Cargo.toml` declares optional `paladin-llm`, an extracted-to-extracted edge the rule forbids absolutely and the same PRD's §4.4 anticipated |
| REQ-extraction-order-and-shims | Verify — the storage-first order was followed; the shim protocol was not re-checked → HARD-01 |
| REQ-tensorflow-stays-facade-v1 | **Superseded by outcome** — adapter and flag deleted. Variant group 24 |
| REQ-sqlx-workspace-dependency | **Shipped, narrowed** — `sqlx` stays in `[workspace.dependencies]`, but as `default-features = false` with `["runtime-tokio-rustls", "sqlite", "chrono", "uuid", "json", "migrate"]`. **`mysql` is absent** from the workspace feature list against §7.5's explicit form, and `migrate` was added; both changes trace to the RustSec hardening work → SEC-01 |
| REQ-dependency-isolation-metrics | Verify — the dep-tree reduction targets were not re-measured → HARD-01 |

### Milestone 7 Epic 2 — Production Build Infrastructure (13 IDs)

Epic-level note: this is the one Milestone 7 epic whose three open checkboxes are **plausible**.
Its genuine residue is two defects, both carried forward.

| Requirement | Verdict |
|---|---|
| REQ-docker-workspace-build | **Shipped, defect → SEC-05** — `Dockerfile.chef` pins `cargo-chef 0.1.77 --locked`, runs `chef prepare` / `chef cook --release --workspace`, and uses `rust:1.93-slim-bookworm`; its planner COPY list enumerates nine manifests and omits `crates/paladin-herald/Cargo.toml` |
| REQ-build-baselines-doc | **Shipped (relocated)** — `docs/BUILD_BASELINES.md` does not exist; the equivalent ships as `docs/src/appendix/build-baselines.md` after the Milestone 11 overhaul. **Do not plan as missing** |
| REQ-makefile-workspace-targets | Verify → HARD-01 |
| REQ-makefile-per-crate-targets | Shipped — all ten targets at `Makefile:167-212` (`test-core` … `test-facade`) |
| REQ-ci-workflow-triggers | Verify → HARD-01 |
| REQ-ci-per-crate-matrix | Verify → HARD-01 |
| REQ-ci-workspace-job | Shipped — `--workspace` clippy / doc / test at `ci.yml:54,57,222,225`, but `:225` carries `--exclude paladin-ports` → DEBT-03, HARD-07 |
| REQ-ci-integration-job | Verify → HARD-01 |
| REQ-ci-publish-dry-run-v1 | **Coexists, not superseded** — the per-crate dependency-ordered form ships at `release.yml:410`. Variant group 23 |
| REQ-ci-publish-dry-run-v2 | Shipped — `ci.yml:644` runs a single `cargo publish --workspace --dry-run` with an inline counter-rationale. No document carrier, so no precedence standing. Variant group 23 |
| REQ-ci-feature-flag-matrix | Shipped — `feature-flags.yml:115,118`; the library-only isolation test at `:141` |
| REQ-integration-test-placement | Verify → HARD-01 |
| REQ-integration-tests-doc | **Shipped (relocated)** — `docs/src/appendix/integration-tests.md`. Do not plan as missing |

### Milestone 7 Epic 3 — Benchmark Suite Migration (10 IDs)

Epic-level note: **fully corroborated.** All five benchmark files ship at exactly the locations the
assessment names, and no `.disabled` benchmark file remains anywhere in the tree.

| Requirement | Verdict |
|---|---|
| REQ-sanctum-bench-migration | Shipped — `crates/paladin-memory/benches/sanctum_benchmarks.rs`, with imports rewritten to `paladin_core` / `paladin_memory` / `paladin_ports` and Criterion registration owned by the crate |
| REQ-disabled-bench-disposition | Shipped — none of the five was directly restored; `herald_benchmarks`, `paladin_benchmarks` and `arsenal_benchmarks` were deprecated and removed, `battalion` and `garrison` removed and replaced at narrower scope |
| REQ-battalion-benchmarks | Shipped — `crates/paladin-battalion/benches/battalion_benchmarks.rs` |
| REQ-llm-serialization-benchmark | Shipped — `crates/paladin-llm/benches/llm_serialization_benchmarks.rs` |
| REQ-garrison-benchmarks | Shipped — `crates/paladin-memory/benches/garrison_benchmarks.rs` |
| REQ-config-loading-benchmark | Shipped — root `benches/config_benchmarks.rs`; the ownership finding (`Settings` lives in `src/config/settings.rs`, no extracted crate owns it) closes the PRD's open question 1 |
| REQ-critical-path-bench-scope | Shipped — four categories; all six PRD success metrics recorded Satisfied in the assessment's own status table → HARD-01 |
| REQ-workspace-bench-execution | Verify — `cargo bench --workspace --no-run` is the recorded structural compile-validation command → HARD-01 |
| REQ-performance-baseline-doc | **Shipped (relocated)** — `docs/src/appendix/performance-baseline.md`. Note this does **not** close QUAL-05, which owns producing measured *runtime* numbers rather than the document |
| REQ-bench-regression-signal | Shipped — `ci.yml:531` job `benchmark-regression-signal`, threshold "more than 3 Criterion regression notices in one run", non-blocking via `continue-on-error` |

### Milestone 7 Epic 4 — API Stabilization & Pre-Release Preparation (12 IDs)

Epic-level note: this epic produced `v0.1.0-rc.1` and the crates.io package renames. **All of it is
history** (HARD-03) — but three of its gates do not hold today.

| Requirement | Verdict |
|---|---|
| REQ-crate-metadata-completion | **Shipped, contested → SEC-02** — the `paladin-ai` / `paladin-ai-core` renames are applied with lib names preserved (`paladin`, `paladin_core`); the `license` field reads MIT against the signed dual-licence checklist |
| REQ-per-crate-readme | Shipped — all ten library crates have a `README.md` |
| REQ-per-crate-changelog | **Open defect → SEC-04** — nine of ten; `crates/paladin-herald/` has none, and the completion summary records this criterion Met |
| REQ-doc-coverage-audit | **Contested → HARD-07** — the >90% coverage posture is recorded Met while `paladin-ports` sets `doctest = false` and CI excludes it from `--doc` |
| REQ-versioning-policy | **Shipped (relocated), superseded by outcome** — no `docs/VERSIONING_POLICY.md`; the lockstep `0.2.0` target was superseded by the `0.1.0` publish → HARD-03 |
| REQ-release-checklist | **Shipped (relocated)** — `docs/src/appendix/{release-checklist,release-automation}.md` |
| REQ-stable-api-per-crate | **Shipped (relocated)** — no root `STABLE_API.md`; the equivalent ships at `docs/src/api-reference/stable-api.md`. `api_surface_current.txt` (881 KB) and `final-api.txt` (198 KB) do exist at the root → ARCH-05, DEBT-01 |
| REQ-release-readiness-audit | **Shipped (history)** — every gate PASS, GO sign-off, tag `v0.1.0-rc.1` at `a9530fc`, all ten crates verified on docs.rs, external smoke project compiled → HARD-03 |
| REQ-rustsec-risk-acceptance | **Open → SEC-01** — the accepted set has grown beyond the two documented advisories and diverges across four surfaces; the acceptance expires **2026-09-30** |
| REQ-rustsec-hardening-actions | **Partially shipped → SEC-01** — `testcontainers-modules` is in `dev-dependencies`, MySQL compilation is gated on `storage-mysql`, and `sqlx` runs `default-features = false`; the four named open action items (two impact-analysis issues, approved `audit.toml` entries with owner and expiry, post-mitigation re-audit evidence) are unclosed |
| REQ-license-policy-signoff | **Contested → SEC-02** — a signed policy with a named approver that the manifests do not declare |
| REQ-paladin-ports-publish-verification-closed | **Closed** — not forward work. The only residue is the collision guardrail → SEC-03 |

### Milestone 8 Epic 1 — Facade Crate Audit (4 IDs)

Epic-level note: the audit was executed and is **explicitly superseded** by the 2026-06-04
reconciliation on factual grounds. HARD-02 records the supersession; the audit's *method* is worth
keeping, its *classifications* are not.

| Requirement | Verdict |
|---|---|
| REQ-facade-file-inventory | Shipped, superseded — 189 files audited 2026-05-29 |
| REQ-facade-file-classification | **Shipped, superseded → HARD-02** — 151 stay / 13 move / 25 delete, with ~4,400 LOC of orphaned uncompiled duplicates classified as "active bridges that stay" |
| REQ-shim-consumer-validation | Shipped, superseded — the reconciliation's reproducible orphan test (`rg "mod <name>"` returns nothing; the directory `mod.rs` only does `pub use paladin_<crate>::…`) is the version to keep |
| REQ-facade-audit-document | **Shipped, explicitly superseded** — the reconciliation's header names this document by path → HARD-02 |

### Milestone 8 Epic 2 — Dead Shim & Empty Module Removal (4 IDs)

Epic-level note: **the two open checkboxes are contradicted by code.** Everything this epic scoped
is verifiably done.

| Requirement | Verdict |
|---|---|
| REQ-dead-file-batch-deletion | Shipped — all 25 List A files gone, plus the orphaned `notifications/`, `storage/`, `subject/`, `admin/` and `user/` directories. **Residue:** the `email_notifications.rs` (392 LOC) overlap review the PRD's Open Question 1 required is recorded nowhere. Variant group 27 |
| REQ-stale-application-ports-audit | Shipped — `src/application/ports/` did not exist even at audit time; removed before Milestone 8 began |
| REQ-core-minimum-structure | Shipped — `src/core/` is **exactly** the six named files, verified 2026-07-30 |
| REQ-libr-dead-reexport-removal | Verify — the `lib.rs` alias removals were not individually re-checked → HARD-01 |

### Milestone 8 Epic 3 — Relocate Remaining Misplaced Modules (6 IDs)

Epic-level note: **the one open checkbox is contradicted by code, and the epic went further than its
own task list scoped.** Its two governing records are superseded → HARD-02, FACADE-04.

| Requirement | Verdict |
|---|---|
| REQ-notification-task-closeout | Shipped — adapters live in `paladin-notifications` with a facade re-export; the three channel *services* were deleted rather than moved. Variant group 27 |
| REQ-storage-shim-deletion | Shipped — superseded in *mechanism* by commit `897e77e`, which made `paladin-storage` non-optional rather than deleting shims naively |
| REQ-adapter-disposition-record | **Shipped, superseded → HARD-02, FACADE-04** — 20 rows all "Stays"; two rows disagree with the governing PRD; names `paladin-arsenal` and `paladin-sanctum`, neither of which exists. Dated `2025-01`, inconsistent with every other M8 document. Variant group 26 |
| REQ-tensorflow-ml-feature-gate-v2 | **Superseded by outcome** — the gate and the adapter were both deleted. Variant group 24 |
| REQ-garrison-sanctum-bridges-kept | Shipped — both bridges remain with consumer evidence, and the §8 resolved-decisions record stands. Note its own factual correction: `api_content_deliverer.rs` is **724 LOC, not 629** (629 belongs to `tensorflow_adapter.rs`) — and the file was later deleted anyway |
| REQ-m8-epic3-no-extractions | **Superseded by outcome → HARD-02** — the relocations were executed in Milestone 8. Variant group 25 |

### Milestone 8 Epic 4 — `use_cases` → `services` Rename (4 IDs)

| Requirement | Verdict |
|---|---|
| REQ-use-cases-services-rename | Shipped — `src/application/services/` with 11 sub-modules; a workspace-wide grep for `use_cases` across `src/`, `crates/`, `tests/`, `examples/` and `benches/` returns **zero** matches |
| REQ-rename-clean-break | Shipped — no `pub use services as use_cases;` exists; the overview's optional Task 4.3 was explicitly rejected by the PRD |
| REQ-rename-doc-updates | Verify — the 57 markdown references were not individually re-checked → HARD-01 |
| REQ-rename-changelog-breaking | Verify → HARD-01 |

### Milestone 8 Epic 5 — Facade Role Documentation & v0.2.0 Finalization (6 IDs)

| Requirement | Verdict |
|---|---|
| REQ-facade-role-lib-docs | Shipped — `src/lib.rs` carries the facade / composition-root documentation |
| REQ-facade-readme | Shipped — `src/README.md` (3,750 bytes) |
| REQ-stable-api-v020-sync | **Shipped (relocated)** — applies to `docs/src/api-reference/stable-api.md` after the Milestone 11 overhaul → ARCH-05 |
| REQ-changelog-v020-cut | **Shipped (history)** — v0.2.0 shipped and the tree is four minors past it → HARD-03 |
| REQ-api-surface-baseline-v020 | **Open defect → DEBT-01** — regenerating the baseline depends on the `api-surface` job working, which it has not since commit `928c6d5` |
| REQ-m8-final-quality-gate | Shipped — but its FR-19 `cargo doc` bar ("warnings acceptable") contradicts M7 Epic 4 §4.4.3 ("without documentation warnings") on the same command → HARD-07 |

### Milestone 8 Epic 6 — `paladin-content` Services Rename (4 IDs)

Epic-level note: **recorded "Not verified; low priority" by the reconciliation and complete in the
tree.** Do not plan it as outstanding → HARD-02(d).

| Requirement | Verdict |
|---|---|
| REQ-paladin-content-services-rename | Shipped — `crates/paladin-content/src/services/` exists and `lib.rs` declares `pub mod services;`, closing the broken bridge the Epic 6 DOC's Root Cause section describes |
| REQ-paladin-content-readme-update | Verify → HARD-01 |
| REQ-paladin-content-changelog-fix | Verify → HARD-01 |
| REQ-content-processing-build-gate | **Shipped, narrowed → HARD-06** — the workspace builds under `content-processing`, but the facade flag enables five of six capability features and omits `pdf`, whose feature gates no dependency |

### Milestone 8 Epic 7 — `paladin-web` Single Framework (axum) (6 IDs)

| Requirement | Verdict |
|---|---|
| REQ-delivery-endpoints-axum | Shipped — `crates/paladin-web/src/delivery_controller.rs` documents `POST /api/delivery/deliver`, `GET /api/delivery/status/{delivery_id}` and `GET /api/delivery/stats`; `app.rs:24` imports and `app.rs:63` merges `create_delivery_routes(deliverer)`, so they are **mounted**, not merely ported |
| REQ-actix-removal | Shipped — `grep -rn "actix" crates/paladin-web/` returns zero matches. Variant group 21 |
| REQ-actix-deny-ban | Shipped — `deny.toml:99-103` bans `actix-web` with the reason "paladin-web standardizes on axum; no second web framework" |
| REQ-delivery-handler-tests | Verify → HARD-01 |
| REQ-web-api-baseline-changelog | **Open defect → DEBT-01** — FR-10 mandates `./scripts/extract-public-api.sh project/current-exports.txt`, the path that has been stale since commit `928c6d5`. **The defect is now written into a requirement as well as into the tooling**, so DEBT-01 must correct both |
| REQ-web-quality-gate | Verify — the `web-server` feature-matrix entry and the change-confinement clause were not re-checked → HARD-01 |

### Cross-milestone entries carried by DOCs rather than PRDs (5 IDs)

These five have no PRD carrier. Four of them are the run's most reliable content.

| Requirement | Verdict |
|---|---|
| REQ-storage-nonoptional-v2 | Shipped — variant group 22 |
| REQ-m8-reconciliation-relocations | Shipped — 15 commits, ~10,250 net LOC removed, one new leaf crate; every target confirmed in the tree → HARD-02 |
| REQ-m8-deferred-items-register | **Open register → FACADE-01 (D5), FACADE-02 (D1-D4)** — D5's count is verified exact; D1's six `src/core/` files and D2's three manager services all still ship. No owners named, no target milestone assigned |
| REQ-deferred-cli-user-commands | **Deferred with register → FACADE-03(a)** — `user.rs` verified absent from the ten CLI command modules; backend intact; recoverable verbatim from git history |
| REQ-deferred-tensorflow-ml-adapter-v3 | **Deferred with register → FACADE-03(b)** — adapter and `ml` flag verified absent; the `paladin-ml` leaf-crate placement condition is the live artefact. Variant group 24 |

---

## v2 Requirements

Acknowledged, deferred, not in the current roadmap. Some of these may acquire a real requirement
when ingest run 5 lands Milestones 9-12, Deferred-QA-CICD-Completion and project-management.

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
  converge the dual `reqwest` 0.12/0.13 dependency. Note DEBT-04 *gates* `structopt` behind `cli`
  but does not migrate it — the two are complementary, not alternatives.
- Bring the `paladin-core` and `paladin-ports` dependency allowlists back in line with reality, or
  reduce the crates to match them (`paladin-core` ships 14 against a "complete and exhaustive" 6;
  `paladin-ports` ships 10 against 7). The architectural invariant holds either way — no infra SDK
  is present — so this is document-versus-code drift, not a layering break. Depends on ARCH-03(b)
  choosing a direction. *(run 3)*
- Add the planned `retry`, `rate_limiter` and `bulkhead` primitives to
  `src/infrastructure/resilience/`, and consolidate the retry logic currently embedded in
  `mcp_sse_adapter.rs` and `api_content_deliverer.rs` — both explicitly deferred by Milestone 6
  Epic 4. *(run 3)*
- Complete the `user_service` relocation out of `src/core/platform/manager/`, including
  `UserServiceFactory`, `user_config.rs`, the user CLI commands, the user API controller and
  `SqliteUserRepository`. Milestone 6 Epic 2 explicitly scoped this out and flagged it for "a
  future Epic". *(run 3)*
- Orchestrator state durability: workflow checkpointing, resume-on-startup, enforced queue
  persistence in production mode.
- Single-threaded orchestration scheduler in `src/application/services/orchestration/scheduler.rs`
  — `codebase/CONCERNS.md` recommends leaning on `tokio-cron-scheduler`, already a dependency and
  already adapted in `crates/paladin-storage/src/scheduler.rs` (REQ-scheduler-port).
- Environment isolation for `system_log_integration_test.rs`.
- Add the `retry`, `rate_limiter` and `bulkhead` primitives — unchanged by run 4. *(run 3)*
- **Reconsider the `src/core/` re-export shims and the mis-layered manager services** only if
  FACADE-02 decides against keeping them. D1 (six files, 49 importers) and D2 (three manager
  services) are currently *deliberate* keeps, not debt; they become debt only if the facade adopts
  a no-alias policy (ARCH-04). *(run 4)*
- **Untangle the `paladin_builder.rs` / `paladin_execution_service.rs` coupling** so the ~2,750 LOC
  of entangled Paladin services (D3) can move to `paladin-battalion` and `paladin-llm`. Blocked
  twice over: on the coupling itself, and on HARD-05 deciding whether leaf-to-leaf edges are
  permitted. Overlaps the existing oversized-service-file item above. *(run 4)*
- **A future content-delivery crate.** M7 Epic 1 §4.5.2 reserved one as the "correct long-term home"
  for `file_content_repository.rs`; the file was then deleted and no later document mentions the
  crate again. Carried so the idea is not lost silently. *(run 4)*

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
  one of the corpus's two strongest ADR candidates should the user want to protect it.

**Deliberately not carried forward from run 3:**

- **The 9-plus-crate workspace extraction itself** (Milestone 5, all six epics) and **the four
  Milestone 6 relocations** are not planned as forward phases. They shipped, and
  `intel/code-verification.md` verifies each against the tree. The ledger records them.
- **Milestone 5's 17 open checkboxes** are not a backlog — the crates, the prelude, the
  `crate-isolation` CI job and the benchmark report all exist. Only the two residues
  (`paladin-ports` doctests, the toolchain-action cleanup) become DEBT items.
- **Milestone 6's task lists** produce nothing: 0 open items, corroborated by code.
- **`STABLE_API.md`, `docs/FEATURE_FLAGS.md`, `docs/MIGRATION.md`, `docs/CONFIGURATION.md`** are
  not carried as missing deliverables — they are relocated into the mdbook. Recording that is
  ARCH-05, not building them.
- **A `paladin-cli` workspace crate** and **MCP transport feature flags** are not carried at all:
  both are documented positions that shipped code contradicts, and in both cases the later PRD
  agreed with the code.
- **The two ADR candidates are not converted into locked decisions here.** Promoting the Epic 1
  decision record or Epic 17.5's CLI-location decision requires re-tagging the source documents
  via `--manifest` and re-running ingest; manufacturing the lock inside an ingest artefact would
  fabricate authority the corpus does not contain. ARCH-03(c) records the recommendation.

**Deliberately not carried forward from run 4:**

- **Every shipped Milestone 7-8 deliverable.** The four crate extractions and their cost-benefit
  gate, the Docker/Makefile/CI adaptation, the five-benchmark migration, the whole
  release-candidate cycle, the 25 List A deletions, the `use_cases` → `services` rename in both the
  facade and `paladin-content`, the actix removal and ban, the three mounted axum delivery routes,
  and the reconciliation's fifteen commits are all verified in the tree. The ledger records them;
  no phase re-plans them.
- **Milestone 8's three open checkboxes.** Contradicted by code: Epics 2 and 3 are both complete,
  and Epic 3 went *further* than its own task list scoped. Same pattern as Conclave and Sanctum in
  runs 1-2.
- **Milestone 8 Epic 6.** Recorded "Not verified; low priority" by the reconciliation and omitted
  from `deferred-items.md`, but verifiably complete. Not planned.
- **Milestone 7's three open checkboxes as a task list.** Their genuine residue is SEC-05 (the
  stale `Dockerfile.chef` COPY list) and DEBT-01 (the `api-surface` baseline path). The rest of
  Epic 2's apparently-missing deliverables are mdbook relocations.
- **The 14 "superseded by outcome" requirements** are not planned as written — that is the point of
  recording them. Chief among them: actix-web as a `paladin-web` dependency, the `storage-sqlite`
  flag, the per-crate ordered publish dry run, the `ml` feature gate, the M8 Epic 3 no-extraction
  mandate, and the `find src/ -name "*.rs" | wc -l` = 160 target (the tree reads **136**).
- **`docs/PERFORMANCE_BASELINE.md`, `docs/RELEASE_CHECKLIST.md`, `docs/VERSIONING_POLICY.md`,
  `docs/BUILD_BASELINES.md`, `docs/INTEGRATION_TESTS.md` and a root `STABLE_API.md`** — the same
  Milestone 11 mdbook relocation ARCH-05 already covers for the run-3 deliverables. Relocated, not
  missing.
- **`paladin-arsenal` and `paladin-sanctum`** are not planned as crates. Neither exists; both are
  named only by a superseded disposition record that disagrees with its own governing PRD.
  Triaging the list is FACADE-04; building the crates is not in scope.
- **`paladin-ml`** is not planned as a crate either. It is a *condition* on reintroducing a removed
  feature (FACADE-03b), not a deliverable.
- **The four new ADR candidates are not converted into locked decisions.** `cost-benefit-assessment.md`
  (self-approval block, named approver, 2026-05-25), `rustsec-remediation-plan.md` (owner, expiry
  2026-09-30), `license-compatibility-decision-checklist.md` (approver `DF3NDR`, 2026-05-28) and
  `facade-cleanup-RECONCILIATION-2026-06-04.md` (an explicit supersession notice) are all
  manifest-typed DOC. Promoting any requires re-tagging via `--manifest` and re-running ingest.
  SEC-01 and SEC-02 record the recommendations; they do not act on them.

### Awaiting ingest (run 5 of 5)

- **Run 5:** Milestones 9-12 (classic orchestrator completion, CI hardening and release
  automation, documentation overhaul, Web API) plus Deferred-QA-CICD-Completion and
  project-management. `intel/code-verification.md` names the Deferred-QA documents as one of the two
  places where the genuine remaining-work signal lives; run 4 supplied the other
  (`deferred-items.md` and `deferred-features.md`, both verified exact against the tree).
- **Dependency semantics run 5 must connect to.** `Milestones-8-11_Dependency-Graph.md` (ingested in
  run 4 and preserved deliberately) records: M8 → M9 **HARD** ("M9 work should not begin until M8
  Epic 4 is complete"); M8 → M11 **HARD** on path stability, with M11 Epics 3-4 waiting on M9
  Epics 1-3; M9 → M11 **HARD** on API stability ("only write about APIs that are merged to
  `main`"); M8 → M10 **SOFT** and only for M10 Epic 3. Critical path M8 → M9 → M11 Epics 3-5 =
  11-17 sprints; M10 is entirely off it. Release gates: v0.2.0 / v0.3.0 / v0.4.0 / v0.5.0.
  **It is a historical planning artefact, not a live schedule** — `task-completion-state.md`
  records M9 and M10 at 100% and M11 at 92%, so run 5 will be attaching requirements to work that
  has largely shipped. Its dependency *semantics* and release-gate criteria are what run 5 should
  use; its sprint schedule is spent.
- Expect a **fifth** milestone-numbering collision; directory numbering stays authoritative
  (VERIFY-03, ARCH-02, HARD-04).

Shipped code with **no ingested requirement yet**: the Axum HTTP API's auth, rate limiting, OpenAPI
and SSE streaming surfaces (Milestone 12, run 5). **Run 4 closed the crate gap** — all ten library
crates now have an ingested requirement, four from M7 Epic 1's extraction PRD and `paladin-herald`
from the 2026-06-04 reconciliation.

---

## Out of Scope

| Feature | Reason |
|---|---|
| Re-implementing shipped Milestone-1/2/3 work | The ledgers record it; the roadmap does not re-plan it |
| Re-planning the Milestone 5 workspace decomposition or the Milestone 6 relocations | Verified shipped against the tree in `intel/code-verification.md`. Ten library crates exist; `application_settings.rs` is deleted; the orchestrators, the Maneuver DSL and `CircuitBreaker` are all at their new homes |
| Extracting a `paladin-cli` crate, or adding MCP transport feature flags | Both are documented positions the shipped code contradicts, and in both cases the later PRD agreed with the code |
| Picking a winner for the 20 competing variant groups inside this document | Deliberate, and explicitly requested: shipped code is the arbiter and the decision belongs in an ADR (RECON-02 … RECON-07, VERIFY-03 … VERIFY-06, ARCH-03, ARCH-04), not in an ingest artefact. Where shipped code settles a variant, that is recorded as a fact about the tree at the top of the precedence order, not as a decision taken here |
| Synthesizing locked decisions from PRD/DOC assertions | 0 ADR-typed and 0 SPEC-typed docs exist across all 113 documents; asserting locks would fabricate authority. This includes the Approved-status Epic 1 decision record — promoting it requires re-tagging the source via `--manifest`, not an edit here |
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
| Re-planning shipped Milestone 7-8 work | The four crate extractions, the build/CI/benchmark infrastructure, the release-candidate cycle, the facade deletions, the rename, the actix consolidation and the reconciliation's fifteen commits are all verified in the tree |
| Implementing the 14 requirements marked *Superseded by outcome* | Shipped code went a different way, deliberately. Recording that is HARD-01; implementing them would undo it |
| Building `paladin-arsenal`, `paladin-sanctum` or `paladin-ml` | None exists. The first two are named only by a superseded disposition record that contradicts its own PRD (FACADE-04); the third is a placement *condition* on reintroducing a removed feature (FACADE-03b), not a deliverable |
| Rebuilding `docs/{PERFORMANCE_BASELINE,RELEASE_CHECKLIST,VERSIONING_POLICY,BUILD_BASELINES,INTEGRATION_TESTS}.md` or a root `STABLE_API.md` | Relocated into the mdbook by the Milestone 11 overhaul, not missing — same finding ARCH-05 records for the run-3 deliverables |
| Treating any `v0.1.0-rc.1` artefact as current | It is history. The published-crate list, docs.rs verification and GO sign-off all describe the 0.1.0 state; the tree is at 0.6.0 → HARD-03 |
| Milestone 9-12 feature work | Awaiting ingest run 5; planning it now would guess at requirements that exist on disk |

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
| ARCH-01 | Phase 7 | Pending |
| ARCH-02 | Phase 7 | Pending |
| ARCH-03 | Phase 7 | Pending |
| ARCH-04 | Phase 7 | Pending |
| ARCH-05 | Phase 7 | Pending |
| ARCH-06 | Phase 7 | Pending |
| ARCH-07 | Phase 7 | Pending |
| DEBT-01 | Phase 8 | Pending |
| DEBT-02 | Phase 8 | Pending |
| DEBT-03 | Phase 8 | Pending |
| DEBT-04 | Phase 8 | Pending |
| DEBT-05 | Phase 8 | Pending |
| SEC-01 | Phase 9 | Pending |
| SEC-02 | Phase 9 | Pending |
| SEC-03 | Phase 9 | Pending |
| SEC-04 | Phase 9 | Pending |
| SEC-05 | Phase 9 | Pending |
| HARD-01 | Phase 10 | Pending |
| HARD-02 | Phase 10 | Pending |
| HARD-03 | Phase 10 | Pending |
| HARD-04 | Phase 10 | Pending |
| HARD-05 | Phase 10 | Pending |
| HARD-06 | Phase 10 | Pending |
| HARD-07 | Phase 10 | Pending |
| FACADE-01 | Phase 11 | Pending |
| FACADE-02 | Phase 11 | Pending |
| FACADE-03 | Phase 11 | Pending |
| FACADE-04 | Phase 11 | Pending |

**Coverage:**
- v1 requirements: 62 total (25 Milestone-1 close-out + 9 Milestone 2-3 close-out +
  12 Milestone 4-6 close-out + 16 Milestone 7-8 close-out)
- Mapped to phases: 62
- Unmapped: 0 ✓
- Duplicated across phases: 0 ✓

**No run-4 requirement duplicates an existing one.** Three earlier requirements were **extended in
place** rather than duplicated, per the Roadmap Extension Protocol: ARCH-01 (pending crate
provenance now supplied), DEBT-01 (a sixth stale reference, this one inside a requirement) and
DEBT-03 (the documentation gate it sits under, now named). Those edits are recorded at the point of
each requirement.

**Cross-phase couplings** (not duplication — one requirement records the answer, another applies
it; recorded here so neither is planned twice):

| Recording requirement | Applying requirement | Subject |
|---|---|---|
| ARCH-03(a) (Phase 7) | REL-02 (Phase 4) | One Rust edition across the workspace |
| ARCH-04 (Phase 7) | REL-01 (Phase 4) | Whether Milestone 6 forces a major version bump |
| ARCH-03(c) (Phase 7) | DEBT-05 (Phase 8) | Which crate owns the canonical `TokenUsage` |
| RECON-07 (Phase 1) → VERIFY-05 (Phase 5) | QUAL-01 / QUAL-03 (Phase 3) | The coverage gate |
| HARD-06 (Phase 10) | SEC-01 (Phase 9) | Whether `pdf-extract` is reachable — decides if the `RUSTSEC-2026-0187` suppression is needed at all |
| HARD-03 (Phase 10) | REL-01 (Phase 4) | The version trajectory; REL-01 must not converge on an rc.1 figure |
| HARD-05 (Phase 10) | FACADE-02 (Phase 11) | Whether leaf-to-leaf crate edges are permitted, which decides D2/D3/D4's relocation targets |
| HARD-07 (Phase 10) | DEBT-03 (Phase 8) | Which `cargo doc` bar governs, and therefore what re-enabling `paladin-ports` doctests must satisfy |
| ARCH-04 (Phase 7) | FACADE-02 (Phase 11) | Whether a no-re-export-alias policy is adopted, which decides D1 |

**Ledger coverage:**
- Requirement IDs enumerated from `intel/requirements.md`: 434 (run 1: 115, run 2: 118,
  run 3: 115, run 4: 86)
- Recorded in the ledgers: 434 ✓ — 115 in the Milestone 1 ledger, 118 in the Milestone 2-3
  ledger, 115 in the Milestone 4-6 ledger, 86 in the Milestone 7-8 ledger
- Competing-variant entries preserved unmerged: 56 across 28 groups (12 from run 1, 18 from
  run 2, 8 from run 3, 18 from run 4)
- Bookkeeping notes:
  - `intel/SYNTHESIS.md` reports 107 requirements for run 1 and its per-PRD table sums to a third
    figure. The enumerated count (115) is authoritative here; reconciling the arithmetic is
    RECON-01.
  - `intel/SYNTHESIS.md`'s run-2 per-PRD table sums to 116 against the 118 IDs actually present.
    The two-entry difference is attribution: `REQ-council-grove-commander-integration` and
    `REQ-maneuver-validation` are counted into adjacent epic groups. The enumerated count (118) is
    authoritative here.
  - `intel/SYNTHESIS.md`'s run-3 per-PRD table sums to 112 against the 115 IDs actually present.
    Three-entry difference, all attribution: the table has no row for the
    `Epic_1/decisions/` source that carries `REQ-port-value-type-ownership-v1`, and it undercounts
    Milestone 5 Epic 2 and Epic 5 by one each (`REQ-port-value-type-ownership-v2` and
    `REQ-workspace-crate-edition-v2`). Enumerated per-source counts: M4 E1 7, E2 9, E3 9; M5 E1 9
    + 1 decision doc, E2 10, E3 9, E4 11, E5 10, E6 6; M6 E1 8, E2 9, E3 9, E4 8 = 115. The
    enumerated count is authoritative here. Third consecutive run with this class of
    discrepancy — RECON-01 owns the reconciliation.
  - SYNTHESIS reports 38 cumulative variant entries; INGEST-CONFLICTS counts 39 warnings. Both are
    correct — several variant groups carry three entries under one warning, and several run-3
    warnings are not `-v1`/`-v2` pairs at all (see *Run-3 unsettled positions* above).
  - Run 4's per-source enumeration reconciles exactly against SYNTHESIS's headline of 86, so this
    is the **first run without an arithmetic discrepancy**. Enumerated: M7 E1 12, E2 12, E3 10,
    E4 12; M8 E1 4, E2 4, E3 6, E4 4, E5 6, E6 4, E7 6; DOC-carried 5; plus
    `REQ-ci-publish-dry-run-v2`, which is sourced from `.github/workflows/ci.yml` rather than any
    document = 86. Note that this last ID has **no document carrier and therefore no precedence
    standing** — it is recorded because the technical position is substantive and contradicts an
    ingested requirement, not because a document asserts it.
  - Run 4 counts 53 cumulative warnings against 56 variant entries. The gap is the same class as
    run 3's: variant group 24 is a three-step chain under one warning, group 26 is an internal
    split inside one requirement, and six run-4 warnings are not `-v1`/`-v2` pairs at all (see
    *Run-4 unsettled positions* above).

---
*Requirements defined: 2026-07-30*
*Last updated: 2026-07-30 after ingest run 4 of 5
(`.project/Milestone_7-Production-Hardening` +
`.project/Milestone_8-Facade-Cleanup-Shim-Resolution`, 40 docs; cumulative 153 docs, 434
requirements, 56 variant entries across 28 groups, 0 locked decisions, 0 blockers). Run 5 appends;
`REQ-*` IDs are the merge keys, and `RECON-*` / `GAP-*` / `QUAL-*` / `REL-*` / `VERIFY-*` /
`CLOSE-*` / `ARCH-*` / `DEBT-*` / `SEC-*` / `HARD-*` / `FACADE-*` are all spent. Run 4 extended
ARCH-01, DEBT-01 and DEBT-03 in place rather than duplicating them; no phase was renumbered.*
