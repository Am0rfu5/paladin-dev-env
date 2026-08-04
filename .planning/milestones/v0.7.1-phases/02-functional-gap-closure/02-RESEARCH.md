# Phase 2: Functional Gap Closure - Research

**Researched:** 2026-07-31
**Domain:** Rust multi-agent orchestration framework — gap closure on existing Battalion/Commander/Herald code, plus three ADR-mandated type edits
**Confidence:** HIGH (nearly everything below was verified directly against the tree and by running `cargo test`, not inferred from documents)

<user_constraints>
## User Constraints (from CONTEXT.md)

### Locked Decisions

**Scope posture toward the Phase 1 ledger**
- **D-01: Re-prove by execution, not by citation.** Phase 2 opens with a measured `cargo test --workspace` run recorded the way Phase 1 recorded its coverage measurement — raw pasted stdout, `rustc -vV`, `cargo --version`, `git rev-parse HEAD`, `date -u` — and each of SC1, SC2 and SC4 gets one executable check whose output is pasted into the phase record.
- **D-02: On disagreement, amend `milestone-01.md` in place.** Edit the contradicted row with the new verdict, the command that produced it, and the date. Sets the convention Phases 5, 7, 10, 13 inherit. Reversibility: costly.
- **D-03: Deferrals are `deferred with reason` ledger rows.** No new document class. Exception: where a deferral *overrides a written requirement*, it escalates to an ADR (see D-08).
- **D-04: GAP-06 closes as `superseded` + a real review.** Epic 2 task 11.5 (coverage ≥ 80%) is `superseded by shipped code` (ADR-0006's 84% workspace floor replaced the per-module target). Task 11.6 gets a written pass over Epic 2's PRD acceptance criteria against shipped code, one verdict per criterion at the D-19 bar.

**The `present, unproven` line**
- **D-05: `REQ-battalion-cancellation` is recorded Phalanx-only; the other three patterns are deferred.** `execute_with_cancellation` exists only on Phalanx (`phalanx_service.rs:151`, tested at `:758`). Building it for Formation/Campaign/ChainOfCommand is out of scope. This contradicts `REQ-battalion-cancellation` as written, so per D-03's exception it escalates to an ADR (D-08).
- **D-06: SC3 closes with one Formation-driven end-to-end test file, three Heralds.** Drive a real `FormationExecutionService` run with mock Paladins, pipe the resulting `BattalionResult` through the JSON, Markdown and Table Heralds, and assert SC3's five named content requirements — Battalion name/ID/type, per-Paladin results in execution order, aggregated token usage, and partial results — including one deliberately-failed Paladin for the partial-results case.
- **D-07: Existence in Phase 2, depth in Phase 3.** Phase 2 proves each of the four Battalion patterns has a passing, non-`#[ignore]`d integration exerciser and that the Phalanx performance claims hold. Phase 3 owns coverage, the four `#[ignore]`d Commander error tests (`commander.rs:2180,2188,2196,2204`), and MCP failure-mode tests. Note for planner: the shared `Send + Sync` failing-mock asset those tests need is a shared asset Phase 3 should build as shared, not local.
- **D-08: An ADR records the cancellation deferral.** Numbered after 0006, written to ADR-0004's shape, recording the requirement text, the Phalanx-only shipped reality, why a four-pattern cancellation contract is out of scope, and the forward owner.
- **D-09: CLI tests in, provider-switching in, CI config out.** Epic 9 tasks 13.4-13.6 (CLI-level tests through Paladin/Formation/Phalanx via mock LLM) and Epic 6 tasks 7.10-7.12 (provider-switching integration tests) are in scope. Epic 6 task 7.14 (CI configuration for `live-api-tests`) is deferred to Phase 15 (PIPE), blocked in substance on Phase 5's VERIFY-06.
- **D-10: Wire in `tests/unit/llm/` and fix it, with a per-file fallback rule.** `tests/unit/mod.rs` never declares `pub mod llm;`, so 25 test functions across 3 files have never compiled or run.
- **D-11: The fallback rule is per-file and keyed on failure kind, not a clock.** Mechanical breakage (import paths, renamed types, `mockito::Server::new()` needing `new_async()`, signature drift) gets fixed, however many occurrences. Structural breakage (asserts behaviour the adapter no longer has) means those specific tests are deleted with a per-test `superseded by shipped code` note. Judged per file.
- **D-12: One sweep for other never-compiled test source.** Cross-check every directory under `tests/` against `[[test]]` targets and barrel `mod.rs` files. Findings get ledger rows. Fixing anything found beyond the LLM module is a separate call.

**GAP-07 and Phase 14's WEB-03**
- **D-13: WEB-03 is pulled forward into GAP-07 and lands in the same change.** Both edit `ProviderCapabilities` at `llm_port.rs:754` and every adapter's construction site. Phase 2 adds `temperature_range` per ADR-0004 **and** corrects `supports_tool_calling` per WEB-03, touching each construction site exactly once. Reversibility: costly.
- **D-14: `supports_tool_calling` becomes `false` on all three adapters, with a correspondence test.** `LlmRequest` has no tools field; neither OpenAI nor Anthropic adapter sends `tools` or parses `tool_calls`. DeepSeek already reports `false`.
- **D-15: Real adapters populate `temperature_range`; every other construction site takes `None`.** OpenAI/Anthropic `[0.0, 1.0]`, DeepSeek `[0.0, 2.0]`. Every other site (test fixtures, in-service defaults) takes `None` = the named `[0.0, 1.0]` fallback. Use `..Default::default()` where a site already permits it.
- **D-16: The ports change lands first, as a tracer.** Land `ProviderCapabilities` (widest blast radius) first with WEB-03, the three adapters, and the correspondence test. The `citadel.rs` rename and the Formation bound follow independently.

### Claude's Discretion

- The renamed identifier for the `citadel.rs:280` struct (ADR-0001 already fixed this to `BattalionCheckpointConfig` — not re-open).
- Plan decomposition and count beyond D-16's tracer-first rule, including whether the baseline run is its own plan or the tracer's first task.
- Whether ROADMAP.md's Phase 2 success-criteria wording is amended at source now the ledger has overtaken its premise.
- How the GAP-05 finding (SC1's named failing test already passes) is restated, and where.
- `REQ-provider-error-mapping`'s dead `LlmProviderError` conversion path — fold into the sweep's findings or leave to Phase 3.
- Whether the D-12 sweep also covers `benches/` and `examples/`.

### Deferred Ideas (OUT OF SCOPE)

- Battalion-wide cancellation for Formation, Campaign and ChainOfCommand (D-05/D-08). No forward owner named yet — planner should name one.
- Epic 6 task 7.14, CI configuration for `live-api-tests` — Phase 15 (PIPE), blocked on Phase 5's VERIFY-06.
- The four `#[ignore]`d, empty-bodied Commander error tests — Phase 3 (QUAL-04), with the shared failing-mock harness.
- Repairing anything the D-12 sweep finds beyond `tests/unit/llm/`.
- `REQ-provider-error-mapping`'s dead conversion path.
- WEB-04 — whether Paladin builds or withdraws LLM tool calling — stays in Phase 14.
- Raising coverage to ADR-0006's 84% floor and the 0%-coverage file list — Phase 3 (QUAL-01/QUAL-02).
</user_constraints>

<phase_requirements>
## Phase Requirements

| ID | Description | Research Support |
|----|-------------|------------------|
| GAP-01 | Chain of Command end-to-end: commander selects specialists, survives failure via fallback, synthesizes answer; tests cover all 4 delegation strategies | **Verified already satisfied.** 104 passing tests reference `chain_of_command` across unit + integration layers (see "Chain of Command: already satisfied" below). No build work — D-01's re-proof is a re-run + citation, not new code. |
| GAP-02 | Battalion integration/perf tests for all 4 patterns, incl. Phalanx ≥10 concurrent / <1s | **Verified already satisfied.** `tests/integration/battalion/load_test.rs` (5 tests) compiles and passes today via `tests/lib.rs → integration::battalion::load_test`. Formation/Campaign/ChainOfCommand/Phalanx integration test files all present and wired (see "Test wiring map"). |
| GAP-03 | Herald on Battalion path: JSON/Markdown/Table show name/ID/type, ordered per-Paladin results, aggregated tokens, partial results | **NOT satisfied by existing code — this is the phase's real code-change item.** JSON Herald omits `strategy_used`(type)/`total_tokens`/`node_errors`; Markdown Herald omits type + aggregated tokens; **Table Herald's `format_battalion_result` is a complete stub that ignores its `_result` parameter and prints hardcoded placeholder rows.** `FormationExecutionService` itself never populates `per_paladin_times`/`per_paladin_tokens`/`total_tokens` (Phalanx does — reusable reference implementation exists). See "GAP-03: the real finding" below. |
| GAP-04 | Commander normalized result (strategy, timings, success/failure counts, preserved errors) + `metadata_output_dir` telemetry | **Verified already satisfied.** `commander.rs:846-865` enriches `result.strategy_used`/`strategy_selection_reasoning`/`strategy_selection_time_ms` after every strategy dispatch and calls `export_metadata()` (`commander.rs:880-913`), which writes `{strategy}_{timestamp}_{uuid}.json` to `metadata_output_dir` when configured, non-fatally. |
| GAP-05 | `test_auto_selects_campaign_for_workflow_keywords` passes; Auto routing correct for all 4 keyword families | **Verified passing today**, run live: `cargo test -p paladin-battalion test_auto_selects_campaign_for_workflow_keywords` → `1 passed; 0 failed`. SC1's premise ("fails today") is stale. |
| GAP-06 | Garrison final validation: measured coverage + PRD-acceptance review | No code change. Garrison PRD at `.project/Milestone_1-MVP/Epic_2/prd-garrison-memory-system.md`. D-04 already dispositions the coverage half as superseded; the review is a writing task against that PRD's acceptance criteria. |
| GAP-07 | Apply Phase 1's reconciled definitions in code: Formation min-Paladin ≥1, provider-aware temperature, `citadel.rs` `BattalionConfig` rename | **Real code work**, 3 distinct edits (ADR-0001, ADR-0003, ADR-0004) plus WEB-03 folded in (D-13). See "GAP-07: three edits" below, including a compile-breaking gotcha ADR-0004 does not mention (`Eq` derive vs `f32`). |
</phase_requirements>

## Summary

This phase's own CONTEXT.md already did the heavy lifting of scoping — this research's job was to **run the ground truth** the plan will need and to find what CONTEXT.md's framing could not surface without reading source line-by-line. Two headline results:

1. **The `cargo test --workspace` baseline is clean today.** A live, full run (`rustc 1.97.1`, `cargo 1.97.1`, commit `fb4b9420b2c5c0e3ef8bc2e672ca6960ceb2efb6`, branch `release/v0.7.0`) produced **2790 passed, 0 failed, 126 ignored** across 35 test binaries/doctest groups, including the named SC1 test. This matches D-01's expectation and gives the phase's first plan a ready-made, already-passing baseline to paste as evidence — no fixing required for SC1.

2. **GAP-03 (Herald ↔ Battalion) is the one requirement whose "satisfied" framing does not survive direct code reading, and it is the largest real code-change item in this phase.** The ledger records `REQ-herald-battalion-result-fields` as `satisfied` because `format_battalion_result` exists and is called by passing tests. But those tests assert almost nothing about content. Direct reads of all three Heralds show: **`TableHerald::format_battalion_result` (`table_herald.rs:145-184`) takes `_result: &BattalionResult` (unused!) and renders two hardcoded placeholder rows ("paladin_1"/"paladin_2", "1.2s"/"2.1s", "400"/"550") regardless of what Battalion actually ran.** JSON Herald's `battalion_result_to_json` (`json_herald.rs:138-160`) never includes `strategy_used` (the "type" SC3 asks for), `total_tokens`, `per_paladin_tokens`, or `node_errors`. Markdown Herald renders success/failure counts and per-Paladin results but also omits type and aggregated tokens. Underneath that, `FormationExecutionService::execute_internal` (`formation_service.rs:187-278`) never populates `per_paladin_times`/`per_paladin_tokens`/`total_tokens` on the `BattalionResult` it builds — it just calls `BattalionResult::new(...)`, which defaults those fields to empty/zero. **`PhalanxExecutionService` already does the aggregation Formation needs** (`phalanx_service.rs:264-282`, keyed by `paladin.node.name`), giving the plan a ready-made reference implementation to port. D-06's "one Formation-driven end-to-end test" will fail immediately against current code unless these gaps are closed first — this is code work, not just test-writing, and the plan should budget for it explicitly.

A third, smaller but concrete surprise: the CLI test cluster D-09 calls "genuinely outstanding" (Epic 9 tasks 13.4-13.6) is **not un-written code** — `tests/cli/{formation,phalanx,paladin}_execution_test.rs`, `error_handling_test.rs` and `tool_integration_test.rs` (1,895 lines total) already exist, use current APIs (`..Default::default()`, `MaxLoops::Fixed`, etc.), and reference `crate::helpers::{MockLlmAdapter, MockPaladinPort, create_mock_with_responses}` — but `tests/cli/mod.rs` has all five `mod` declarations **commented out** ("missing helpers module (Task 4.0)"), and no `tests/cli/helpers.rs` exists. The fix is almost certainly wiring a `tests/cli/helpers.rs` shim to the existing `tests/helpers/` mocks and un-commenting five lines — much cheaper than writing from scratch.

Fourth: `ProviderCapabilities` currently derives `#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]` (`llm_port.rs:753`). ADR-0004's `temperature_range: Option<(f32, f32)>` **will not compile** under that derive list — `f32` does not implement `Eq`. The `Eq` derive must be dropped (no in-tree usage depends on it; `PartialEq` alone covers every `assert_eq!`/`==` site found). ADR-0004 and CONTEXT.md do not mention this; it is a genuine new finding.

**Primary recommendation:** Structure the phase as (1) a baseline/re-proof plan that runs and pastes the workspace test result plus each of SC1/SC2/SC4's named checks and amends the ledger; (2) the ports tracer plan (ADR-0004 + WEB-03 + adapters + `Eq`-derive fix, per D-16); (3) the `Formation::validate` + `citadel.rs` rename plan (ADR-0003, ADR-0001, doctest fix at `citadel_port.rs:358,371`); (4) the Herald/Formation aggregation plan (GAP-03 — port Phalanx's aggregation pattern into Formation, then extend all three Heralds, then write D-06's end-to-end test); (5) the test-closure plan(s) (`tests/unit/llm/` per D-10/D-11, the CLI helpers wiring per D-09, the D-12 sweep); (6) GAP-06's Garrison review and the D-08 cancellation ADR as smaller, independent plans.

## Architectural Responsibility Map

| Capability | Primary Tier | Secondary Tier | Rationale |
|------------|-------------|----------------|-----------|
| Battalion orchestration patterns (Formation/Phalanx/Campaign/ChainOfCommand) | Domain (`paladin-core`) construction + Application (`paladin-battalion`) execution | — | Structs/validation live in `paladin-core`; execution services (retry, aggregation, Herald wiring) live in `paladin-battalion`, which depends on `paladin-core` + `paladin-ports` (dependencies flow inward, per CLAUDE.md) |
| Commander strategy routing & result normalization | Application (`paladin-battalion::commander`) | — | Commander is a router/facade over the four execution services; it owns Auto-mode heuristics and post-hoc result enrichment, not new domain rules |
| Herald output formatting | Infrastructure/adapter (`paladin-herald` crate) | Domain (`paladin-core::herald` trait) | Trait contract lives in core; concrete JSON/Markdown/Table renderers are adapters implementing that port — GAP-03's fixes are adapter-layer changes only, no port signature change |
| LLM provider capability declaration (`ProviderCapabilities`, `temperature_range`, `supports_tool_calling`) | Application-layer port (`paladin-ports::output::llm_port`) | Infrastructure (`paladin-llm` adapters) | The struct is a published contract in the ports crate; each adapter (`paladin-llm`) is the infrastructure implementation that populates it — GAP-07/WEB-03 touch both tiers in the same change (D-13) |
| Temperature validation | Application (`PaladinBuilder::validate`, `src/application/services/paladin/paladin_builder.rs`) | Domain (`autonomous_config.rs` band bounds) | The builder already holds `Arc<dyn LlmPort>` and can call the synchronous `get_capabilities()` to read `temperature_range` — no new dependency needed to make validation provider-aware |
| Formation minimum-Paladin validation | Domain (`paladin-core::battalion::formation::Formation::validate`) | — | Pure domain invariant; Commander's Auto routing (application tier) is a consumer, not touched by ADR-0003 |
| `BattalionConfig` / `BattalionCheckpointConfig` | Domain (`paladin-core::battalion::mod` / `paladin-core::citadel`) | Infrastructure (`paladin-memory::citadel::file_citadel`) | Both types are domain value objects; the memory crate only consumes the persisted shape — GAP-07's rename constrains itself to avoid touching that consumer's serialized format |
| Test infrastructure (mocks, harnesses) | Test-only (`tests/helpers/`, `tests/unit/`, `tests/cli/`) | — | Not part of the shipped crate graph; wiring gaps here (dead `tests/unit/llm/`, commented-out `tests/cli/mod.rs` entries) are build-script/Cargo.toml-declaration problems, not architecture problems |

## Package Legitimacy Audit

**Not applicable — this phase introduces no new external dependencies.** Every crate referenced by GAP-01…GAP-07 (`mockito`, `tokio`, `serde`, `chrono`, `uuid`, `comfy-table`, `async-trait`) is already a workspace dependency in `Cargo.toml` (root and/or per-crate), already used by passing tests, and already audited by the project's existing `make audit`/`make deny` gates. No `npm view`/`pip index`/`cargo search` verification step applies. If a plan discovers it needs a new crate (e.g., for the shared `Send + Sync` failing-mock harness Phase 3 will build), that plan should re-run this gate at that time — none is needed for the work identified in this research.

## Standard Stack

Not applicable in the conventional "recommend a library" sense — this is 100% internal Rust code editing an existing workspace. The "stack" for this phase is the set of already-shipped internal patterns to reuse:

| Component | Location | Purpose | Why it's the standard here |
|-----------|----------|---------|----------------------------|
| `PhalanxExecutionService`'s per-Paladin aggregation loop | `crates/paladin-battalion/src/phalanx_service.rs:264-282` | Builds `per_paladin_times`/`per_paladin_tokens`/`total_tokens` keyed by Paladin name from a `Vec<PaladinResult>` | Exact aggregation Formation needs for GAP-03; already tested, already correct, avoid inventing a second implementation |
| `MockLlmAdapter` / `create_mock_with_responses` / `MockPaladinPort` | `tests/helpers/mock_llm_adapter.rs`, `tests/helpers/mock_paladin_port.rs` (barrel: `tests/helpers/mod.rs`) | Queued-response, tool-call, streaming, and error-injection mock `LlmPort`/`PaladinPort` | Already shipped, already used by `tests/integration/*` and `tests/lib.rs`; the CLI test files under `tests/cli/` already import these by name and only need a path shim |
| `IntegrationMockPaladinPort` pattern | `tests/integration/commander_integration_tests.rs:20-50` | Configurable-failure `PaladinPort` mock with an execution log | Reference pattern D-06's new end-to-end test should follow for injecting one deliberately-failed Paladin into a Formation run |
| `Server::new_async().await` (mockito 1.7.0) | `tests/integration/openai_embedding_tests.rs:22,68,116,...` | Async-safe mockito server construction inside `#[tokio::test]` | Proven working local pattern for fixing `tests/unit/llm/`'s `Server::new()` (sync) calls under D-11 — no external research needed, the fix already exists elsewhere in this repo |
| `unsafe { std::env::set_var(...) }` wrapping | `tests/lib.rs:82-84`, `tests/integration/system_log_integration_test.rs:372-374,399-401`, `tests/integration/cli_integration_test.rs:610,622` | Required under this toolchain: `std::env::set_var`/`remove_var` are `unsafe fn` as of the Rust version in use here (confirmed live: `rustc 1.97.1`), regardless of edition | `tests/unit/llm/provider_factory_test.rs` calls `env::set_var`/`remove_var` **unwrapped** at least 8 times — this alone will fail to compile once wired into `mod.rs`, independent of any other breakage; existing wrapped call sites are the copy-paste fix |

**Installation:** None required — no new packages.

**Version verification:** N/A (no new packages). Existing pinned versions relevant to this phase, confirmed live: `mockito = "1.7.0"` (root `Cargo.toml:144`, also `paladin-content`/`paladin-web`), `rustc 1.97.1`, `cargo 1.97.1`.

## Architecture Patterns

### System Architecture Diagram — Commander dispatch → Battalion execution → Herald rendering

```
                    ┌─────────────────────┐
   caller input --->│  Commander::execute │
                    │  (or explicit       │
                    │   strategy)         │
                    └──────────┬──────────┘
                               │ Auto? -> analyze_and_select(input)
                               │ else -> use explicit strategy
                               v
        ┌──────────────────────────────────────────────────┐
        │  dispatch to one of 4 (+3 extended) services      │
        │  Formation | Phalanx | Campaign | ChainOfCommand   │
        │  (Conclave | Council | Grove — extended patterns)  │
        └───────────────────────┬────────────────────────────┘
                                 │ each service loops/forks over
                                 │ Paladins via PaladinPort::execute()
                                 v
                    ┌─────────────────────────┐
                    │  Vec<PaladinResult>      │  <- per-Paladin output,
                    │  (output, token_count,   │     token_count,
                    │   execution_time_ms,     │     execution_time_ms
                    │   stop_reason)           │     (NAME NOT CARRIED
                    └───────────┬──────────────┘      on PaladinResult —
                                │                      caller must zip
                                │ build BattalionResult with paladin names)
                                v
        ┌────────────────────────────────────────────────────┐
        │  BattalionResult                                     │
        │  { battalion_id, battalion_name, strategy_used,      │
        │    paladin_results (ORDER PRESERVED),                │
        │    per_paladin_times, per_paladin_tokens,             │  <- Phalanx populates
        │    total_tokens, paladin_success/failure_count,       │     this; Formation
        │    node_errors }                                      │     currently does NOT
        └───────────────────────┬───────────────────────────────┘
                                 │ Commander enriches strategy_used/
                                 │ selection_reasoning/selection_time_ms
                                 │ then calls export_metadata() -> optional
                                 │ metadata_output_dir/*.json
                                 v
                ┌────────────────────────────────────┐
                │ Herald::format_battalion_result()   │
                │  JsonHerald | MarkdownHerald |       │  <- GAP-03 gap lives here:
                │  TableHerald                         │     Table is a stub;
                └────────────────────────────────────┘        JSON/Markdown omit
                                                                type + aggregated tokens
```

### Recommended edit sequence (per D-16 tracer-first + dependency order)

```
1. ports tracer:  llm_port.rs (ProviderCapabilities: +temperature_range, fix Eq derive,
                   supports_tool_calling->false) + 3 adapters + doc/test sites
2. domain edits:  formation.rs (min-Paladin bound) ; citadel.rs (BattalionConfig rename)
                   + citadel_port.rs doctest fix
3. aggregation:   formation_service.rs (port Phalanx's per-Paladin times/tokens pattern)
4. Herald fixes:  json_herald.rs, markdown_herald.rs, table_herald.rs
                   (add strategy_used/type, total_tokens, node_errors rendering;
                    replace Table's stub with real result)
5. new test:      one Formation-driven end-to-end test x 3 Heralds (D-06)
6. test wiring:   tests/unit/mod.rs (+ pub mod llm;) and its 3 files (D-10/D-11)
                   tests/cli/mod.rs (uncomment 5 lines) + new tests/cli/helpers.rs (D-09)
7. sweep:         tests/ vs [[test]] targets and barrel mod.rs files (D-12)
8. baseline+ADR:  cargo test --workspace re-run, ledger amendments, D-08 cancellation ADR,
                   GAP-06 Garrison review
```

### Pattern: Formation's aggregation gap, with the fix already shipped nearby

**What:** `FormationExecutionService::execute_internal` (`formation_service.rs:187-278`) builds `paladin_results: Vec<PaladinResult>` correctly (each entry carries real `token_count`/`execution_time_ms`), then calls:
```rust
// Source: crates/paladin-battalion/src/formation_service.rs:269-275 (verified live)
let result = BattalionResult::new(
    battalion_id,
    formation.config.name.clone(),
    started_at,
    current_input, // Final output from last Paladin
    paladin_results,
);
```
`BattalionResult::new` (`crates/paladin-core/src/platform/container/battalion/mod.rs:605-642`) defaults `per_paladin_times`/`per_paladin_tokens` to empty `HashMap` and `total_tokens` to `0`. `PaladinResult` itself carries no Paladin-name field, so a name has to come from zipping against `formation.paladins`.

**When to use:** Any time GAP-03 work touches Formation's result construction.

**Example — the pattern to port, already shipped and tested in Phalanx:**
```rust
// Source: crates/paladin-battalion/src/phalanx_service.rs:264-282 (verified live)
let mut per_paladin_times = HashMap::new();
let mut per_paladin_tokens = HashMap::new();
let mut total_tokens: u64 = 0;

for (i, result) in paladin_results.iter().enumerate() {
    if let Some(name) = successful_names.get(i) {
        per_paladin_times.insert((*name).clone(), result.execution_time_ms);
        per_paladin_tokens
            .insert((*name).clone(), TokenUsage::from_total(result.token_count));
        total_tokens += u64::from(result.token_count);
    }
}
```
Formation's loop already knows each Paladin's name at iteration time (`for (index, paladin) in formation.paladins.iter().enumerate()`, `formation_service.rs:204`), so the equivalent map can be built inline during the existing loop rather than reconstructed after the fact.

### Anti-Patterns to Avoid

- **Hand-constructing a `BattalionResult` to "test" Herald rendering:** Epic 8 task 7.13's own note ("needs Battalion execution setup") and D-06 both explicitly reject this — it is exactly what `TableHerald`'s existing unit test does (constructs a literal with `paladin_results: vec![]`, then only asserts the stub's hardcoded header strings appear), and it is why the stub has survived undetected.
- **Testing all 3 Heralds × all 4 Battalion patterns (12 combinations):** rejected by D-06 — `BattalionResult` is a single merged type, so a Herald cannot tell which pattern produced it; 11 of 12 combinations add nothing beyond the first.
- **Fixing `tests/unit/llm/`'s `Server::new()` by upgrading to a newer mockito major version:** unnecessary — the already-pinned `mockito = "1.7.0"` supports `Server::new_async()` today (proven by `tests/integration/openai_embedding_tests.rs`), so this is a call-site fix, not a dependency bump.
- **Adding `#[non_exhaustive]` + `Default` to `ProviderCapabilities` to solve the "every construction site" problem generically:** explicitly rejected in CONTEXT.md Q3 (D-15) as a breaking change for downstream constructors, deferred to a Phase 4/REL-01 version decision.

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| Per-Paladin token/time aggregation for Formation | A new aggregation helper/struct | Inline the loop pattern already shipped at `phalanx_service.rs:264-282` | Identical shape needed; a second implementation risks a second, subtly different bug (e.g., Phalanx's `successful_names` exclusion-of-failed logic is itself a nuance worth copying, not re-deriving) |
| Mock LLM/Paladin behavior for new tests | New mock types | `tests/helpers::{MockLlmAdapter, MockPaladinPort, create_mock_with_responses}` and `tests/integration/commander_integration_tests.rs`'s `IntegrationMockPaladinPort` | Already shipped, already used across dozens of passing tests; CLI test files already import these by name |
| Async-safe HTTP mocking for `tests/unit/llm/` fixes | A custom async wrapper around mockito | `Server::new_async().await`, proven in `tests/integration/openai_embedding_tests.rs` | Same crate version already in the workspace; zero new dependency risk |

**Key insight:** every piece of infrastructure this phase needs (aggregation logic, mocks, async-mockito pattern) already exists somewhere in the tree. The work is almost entirely "port an existing pattern to a second call site" or "wire up an existing file that was never connected" — not net-new design.

## Common Pitfalls

### Pitfall 1: `ProviderCapabilities`'s `Eq` derive breaks under ADR-0004's new field
**What goes wrong:** Adding `pub temperature_range: Option<(f32, f32)>` to a struct that derives `#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]` (`llm_port.rs:753`) fails to compile with an `Eq`-not-implemented-for-`f32` error.
**Why it happens:** `f32`/`f64` do not implement `Eq` (NaN breaks reflexivity), and `Eq` cannot be derived for a struct containing a non-`Eq` field, even inside an `Option<(f32, f32)>`.
**How to avoid:** Drop `Eq` from the derive list on `ProviderCapabilities` (keep `PartialEq`). Confirmed live via `grep`: no code in the tree constructs a `HashSet<ProviderCapabilities>`/`BTreeSet<ProviderCapabilities>` or otherwise requires `Eq` — every usage found is `assert_eq!`/`==`, which only needs `PartialEq`.
**Warning signs:** `cargo build` failing on `llm_port.rs:753` with "the trait `Eq` is not implemented for `f32`" the moment `temperature_range` is added.

### Pitfall 2: `std::env::set_var`/`remove_var` are `unsafe` on this toolchain
**What goes wrong:** `tests/unit/llm/provider_factory_test.rs` calls `env::set_var(...)`/`env::remove_var(...)` unwrapped (≥8 call sites in `test_get_default_provider` and `test_list_available_providers`). Confirmed live with `rustc --edition 2024`: these calls now require an `unsafe` block.
**Why it happens:** The standard library made `env::set_var`/`remove_var` `unsafe fn` (thread-safety around process environment mutation); this applies at the rustc-version level (1.97.1 here), not gated by crate edition alone.
**How to avoid:** Wrap each call in `unsafe { ... }`, following the exact pattern already used elsewhere in this repo: `tests/lib.rs:82-84`, `tests/integration/system_log_integration_test.rs:372-374,399-401`, `tests/integration/cli_integration_test.rs:610,622`.
**Warning signs:** `error[E0133]: call to unsafe function 'set_var' is unsafe and requires unsafe block` once `tests/unit/mod.rs` gains `pub mod llm;` and the crate is rebuilt.

### Pitfall 3: `tests/unit/llm/*.rs` construct `LlmRequest`/`PromptItem`/`PromptData` against a stale API shape
**What goes wrong:** All three files' shared helper functions (`create_test_request`, `create_user_request`, `setup_mock_server`) construct:
- `LlmRequest { id, model, prompt, attachments }` — missing the current struct's `stream: bool` and `metadata: HashMap<String,String>` fields (`llm_port.rs:464-477` has exactly 6 fields).
- `PromptItem::new(PromptData { id, prompt_type, parameters })` — but the live `PromptItem::new` signature is `fn new(prompt_type: PromptType) -> Result<Self, PromptItemError>` (`prompt.rs:201`), taking only a `PromptType`, not a full `PromptData`, and `PromptData` itself no longer has an `id` field (`prompt.rs:21-31`).
- `SystemPrompt { instructions, constraints, examples: None }` and `UserPrompt { context, examples: None }` — neither `SystemPrompt` (`instructions`, `constraints`) nor `UserPrompt` (`query`, `context`) has an `examples` field today (`prompt.rs:176-185`), and `UserPrompt` requires `query`, which the test never supplies.
**Why it happens:** These files were written before the `paladin-ports` extraction and facade rewiring (both landed after the commit that added these tests, per D-11's own git-log note) — confirmed structurally by direct comparison above.
**How to avoid:** Per D-11, this is judged mechanical (a construction-API adaptation, not a removed capability) since the underlying HTTP-mock behavior being tested (401/429/streaming/timeout) is unaffected. Fix the 1-2 shared helper functions per file to match the current shapes; this likely fixes most/all test bodies in each file in one pass, since they all call the shared helpers rather than constructing requests inline.
**Warning signs:** Compile errors naming `PromptItemError`, "no field `id` on `PromptData`", "no field `examples`", or "missing fields `stream`, `metadata`" the moment `pub mod llm;` is added to `tests/unit/mod.rs`.

### Pitfall 4: `mockito::Server::new()` inside a `#[tokio::test]` (sync-in-async)
**What goes wrong:** `setup_mock_server()` in both `deepseek_adapter_test.rs:16` and `anthropic_adapter_test.rs:17` calls the blocking `Server::new()` from within helper functions invoked by `#[tokio::test]` async test bodies — mockito 1.x's synchronous `Server::new()` spins up its own Tokio runtime internally, which panics ("Cannot start a runtime from within a runtime") when called from inside an already-running async context.
**Why it happens:** Named explicitly in D-11 as an anticipated mechanical breakage.
**How to avoid:** Change `setup_mock_server()` to `async fn`, call `Server::new_async().await` instead of `Server::new()`, and `.await` the helper at each call site. Proven working pattern already in this repo: `tests/integration/openai_embedding_tests.rs:22,68,116,171,186,207,245`.
**Warning signs:** Test panics with a runtime-nesting message rather than a compile error — this one only surfaces at `cargo test` runtime, not at `cargo build`, so a "the module compiles now" check alone is not sufficient; run the tests, not just the build.

### Pitfall 5: The Table Herald's existing unit test looks like coverage but tests nothing real
**What goes wrong:** `TableHerald`'s own `test_format_battalion_result` (`table_herald.rs:308-343`) constructs a `BattalionResult` with `paladin_results: vec![]` (empty!) and only asserts the output contains the literal strings `"Battalion Execution Results"`, `"Paladin"`, `"Status"`, `"Duration"`, `"Tokens"` — which are the stub's hardcoded header/placeholder text, not data derived from the input. A future contributor could reasonably read "there's already a passing test for this" and skip verifying the implementation.
**Why it happens:** The stub and its test were written together, both matching each other rather than matching real behavior.
**How to avoid:** When implementing GAP-03, replace both the implementation AND rewrite this test to assert content that could only appear if the real `result` argument were read (e.g., a distinctive Paladin name, a specific non-round token count, more than 2 rows for more than 2 Paladins).
**Warning signs:** A Herald test whose assertions would still pass if the `result` parameter were replaced with a different `BattalionResult` value — that's the exact litmus test used to find this stub.

## Code Examples

### Provider-aware temperature validation is directly reachable from `PaladinBuilder`

```rust
// Source: src/application/services/paladin/paladin_builder.rs:76-102 (verified live)
pub struct PaladinBuilder {
    llm_port: Arc<dyn LlmPort>,   // <- already held; get_capabilities() is sync
    data: PaladinData,
    // ...
}
```
```rust
// Source: crates/paladin-ports/src/output/llm_port.rs:1264 (verified live)
fn get_capabilities(&self) -> ProviderCapabilities;   // synchronous trait method
```
```rust
// Source: src/application/services/paladin/paladin_builder.rs:1112-1118 (verified live, current)
// Validate temperature is in [0.0, 1.0]
if !(0.0..=1.0).contains(&self.data.temperature) {
    return Err(PaladinError::ConfigurationError(format!(
        "temperature must be between 0.0 and 1.0, got {}",
        self.data.temperature
    )));
}
```
`self.data.temperature` is `f32` (`paladin.rs:156`), matching ADR-0004's `Option<(f32, f32)>` exactly — no type conversion needed. The provider-aware version reads `self.llm_port.get_capabilities().temperature_range.unwrap_or((0.0, 1.0))` and checks `t >= min && t <= max` per ADR-0004's exact inclusive-both-ends contract.

### The doctest that will break silently if the `citadel.rs` rename misses it

```rust
// Source: crates/paladin-ports/src/output/citadel_port.rs:356-374 (verified live, `rust,no_run` — compiles under `cargo test`)
use paladin_ports::output::citadel_port::CitadelPort;
use paladin_core::platform::container::citadel::{BattalionState, BattalionConfig, CheckpointData};
// ...
let battalion_state = BattalionState::new(
    "Formation",
    BattalionConfig::default(),
    vec![],
    Some(checkpoint),
);
```
This doc comment is a real doctest (`rust,no_run` still type-checks and compiles, only skips *running*). ADR-0001's rename target is `citadel::BattalionConfig` -> `citadel::BattalionCheckpointConfig`. This import/usage at `citadel_port.rs:358,371` must be updated in the same commit or `cargo test` (doctests) fails workspace-wide — this exact site is not called out in ADR-0001 or CONTEXT.md's canonical refs.

### The CLI test files already exist — the missing piece is one shim file

```rust
// Source: tests/cli/mod.rs (verified live, current)
// Commented out - missing helpers module (Task 4.0)
// mod arsenal_config_test;
// mod environment_tests;
// mod error_handling_test;
// mod formation_execution_test;
// mod garrison_config_test;
// mod integration_tests;
// mod paladin_execution_test;
// mod phalanx_execution_test;
// mod tool_integration_test;
```
```rust
// Source: tests/cli/formation_execution_test.rs:1-16 (verified live — already uses current APIs)
use paladin::application::services::battalion::formation_service::FormationExecutionService;
use paladin::infrastructure::resilience::circuit_breaker::CircuitBreaker;
// ...
use crate::helpers::{MockLlmAdapter, MockPaladinPort};
```
`crate::helpers` does not exist anywhere under `tests/cli/`. `tests/helpers/mod.rs` (a sibling directory, used by the separate `unit`/`lib` test binaries) already exports `MockLlmAdapter`, `MockPaladinPort`, `create_mock_with_responses`. Since `cli` is declared as its own `[[test]]` target (`Cargo.toml:211-214`, `path = "tests/cli/mod.rs"`, `required-features = ["cli"]`), it does not automatically see `tests/helpers/`. The fix is a `tests/cli/helpers.rs` (or `mod.rs` entry) that does something equivalent to `#[path = "../helpers/mod.rs"] pub mod helpers;`, then uncommenting the five `mod` lines above (`error_handling_test`/`formation_execution_test`/`paladin_execution_test`/`phalanx_execution_test`/`tool_integration_test` — leave `arsenal_config_test`/`environment_tests`/`garrison_config_test`/`integration_tests` commented unless the D-12 sweep separately confirms those are in scope for Phase 2).

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|---------------|--------|
| Global `[0.0, 1.0]` temperature clamp in `PaladinBuilder::validate` | Provider-aware `temperature_range` on `ProviderCapabilities`, global clamp becomes the `None` fallback | This phase (GAP-07, ADR-0004) | Makes DeepSeek's documented `0.0-2.0` range reachable through the normal Paladin path for the first time |
| `Formation::validate` rejects <2 Paladins | Rejects <1 Paladin (0 still rejected) | This phase (GAP-07, ADR-0003) | Resolves the shipped-code self-contradiction where `test_auto_selects_formation_for_single_paladin` passes but would fail at execution |
| `env::set_var`/`remove_var` callable directly | Requires `unsafe` block (confirmed on `rustc 1.97.1`) | Upstream Rust stdlib change, already adopted elsewhere in this repo | Any new/reactivated test using these functions unguarded will fail to compile — a recurring hazard for exactly the kind of dead-code reactivation D-10/D-12 do |
| `mockito::Server::new()` (sync) | `Server::new_async().await` inside async tests | Already the working pattern in `openai_embedding_tests.rs`; not yet applied in `tests/unit/llm/` | Direct fix for the D-11-named mechanical breakage |

**Deprecated/outdated:** The `tests/unit/llm/` helper functions' `PromptItem`/`PromptData` construction API (pre-dates the `paladin-ports` extraction commit `be131f3` and facade rewiring commit `69a15af`).

## Test wiring map (grounding for D-12's sweep and the plan's `<read_first>` lists)

Verified live against `Cargo.toml`'s `[[test]]` declarations and `cargo test --workspace --list`:

| Path | Wired via | Status today |
|------|-----------|---------------|
| `tests/unit/mod.rs` (`[[test]] name = "unit"`) | Declared directly in `Cargo.toml:171-173` | Compiles and runs; **missing `pub mod llm;`** — the only gap (D-10) |
| `tests/unit/llm/{mod.rs,deepseek_adapter_test.rs,anthropic_adapter_test.rs,provider_factory_test.rs}` | Would be pulled in by `tests/unit/mod.rs`'s missing `pub mod llm;` | **Never compiled.** 25 test fns (8 DeepSeek + 9 Anthropic + 8 Factory) |
| `tests/lib.rs` (auto-discovered top-level `tests/*.rs`) | Cargo auto-discovery (no `[[test]]` entry needed for direct children of `tests/`) | Compiles and runs; declares `pub mod integration;` |
| `tests/integration/mod.rs` | `tests/lib.rs:61` `pub mod integration;` | Compiles and runs; declares `pub mod battalion;`, `pub mod battalion_campaign_integration_test;`, `pub mod battalion_chain_of_command_integration_test;`, `pub mod commander_integration_tests;`, `pub mod herald_integration_test;`, etc. |
| `tests/integration/battalion/{mod.rs,load_test.rs,formation_integration_test.rs,phalanx_integration_test.rs,campaign_integration_test.rs,chain_of_command_integration_test.rs,council_integration_test.rs,grove_integration_test.rs}` | `tests/integration/mod.rs:14` `pub mod battalion;` | **All compile and run** — confirmed live: `test integration::battalion::load_test::test_performance_orchestration_overhead ... ok` (and 4 sibling load tests), plus formation/phalanx/campaign/chain_of_command integration tests all passing. GAP-02 is genuinely satisfied already. |
| `tests/cli/mod.rs` (`[[test]] name = "cli"`, `required-features = ["cli"]`) | `Cargo.toml:211-214` | Compiles and runs, but 5 of 9 potential sub-modules are **commented out**: `formation_execution_test`, `phalanx_execution_test`, `paladin_execution_test`, `error_handling_test`, `tool_integration_test` (all reference a nonexistent `crate::helpers`). Only `error_output_test`, `help_output_test`, `progress_output_test`, `table_output_test` (Epic 24 snapshot tests) are active. |
| `tests/helpers/{mod.rs,mock_llm_adapter.rs,mock_arsenal_adapter.rs,mock_paladin_port.rs}` | `tests/lib.rs` presumably (shared with `unit`/other top-level targets) | Compiles; exports `MockLlmAdapter`, `MockPaladinPort`, `create_mock_with_responses`, `create_mock_with_tool_calls`, `create_mock_with_mixed_responses`, `create_test_paladin_with_mock`, `MockArsenalPort` |

**D-12 sweep starting point:** the pattern to check is "file exists under `tests/` but its containing `mod.rs`/barrel does not declare it, or the barrel itself is not reachable from any `[[test]]` target or auto-discovered top-level file." The one confirmed instance beyond `tests/unit/llm/` is `tests/cli/`'s five commented-out modules — but that one is already accounted for by D-09, so the sweep's job is to confirm there is nothing else (check e.g. `tests/unit/arsenal/`, `tests/unit/battalion/` subdirectories are actually pulled in — both are, via `tests/unit/mod.rs:5` `pub mod battalion;` and `pub mod arsenal;` at line 1, confirmed by passing tests observed in the live run at `unit::battalion::*`).

## Chain of Command: already satisfied (GAP-01, SC2)

Verified live in the full `cargo test --workspace` run — 104 lines matching `chain_of_command` end in `... ok`, spanning:
- `crates/paladin-battalion/src/commander.rs` unit tests (via `paladin_battalion` crate's own `#[cfg(test)]`)
- `tests/unit/battalion/chain_of_command_tests.rs` and `chain_of_command_service_tests.rs` (via `tests/unit/mod.rs -> battalion`) — covers all 4 delegation strategies by name: `automatic_delegation_tests`, `broadcast_delegation_tests`, `round_robin_delegation_tests`, `custom_delegation_tests`
- `tests/integration/battalion/chain_of_command_integration_test.rs` and `tests/integration/battalion_chain_of_command_integration_test.rs` (both wired via `tests/integration/mod.rs`) — end-to-end delegation, specialist selection, broadcast, round-robin, concurrent broadcasts, timeout config
- `tests/integration/commander_integration_tests.rs::test_commander_executes_chain_of_command_end_to_end`

`examples/chain_of_command_delegation.rs` exists on disk (confirmed via `find`). No code change needed for GAP-01/SC2 — this is a re-proof-and-cite item exactly as D-01 frames it.

## GAP-07: three edits, precisely located

1. **`crates/paladin-ports/src/output/llm_port.rs:753-769`** — `ProviderCapabilities` struct. Add `pub temperature_range: Option<(f32, f32)>`; drop `Eq` from the derive list (Pitfall 1); update `Default` impl (`llm_port.rs:771-782`) to set `temperature_range: None`; update the compiling doctest at `llm_port.rs:737-751` to include the new field (or switch it to `..Default::default()`).
2. **Three adapters** — `crates/paladin-llm/src/openai/adapter.rs:642-651` (`temperature_range: Some((0.0, 1.0))`, `supports_tool_calling: false`), `crates/paladin-llm/src/anthropic/adapter.rs:518-527` (same range, `supports_tool_calling: false`), `crates/paladin-llm/src/deepseek/adapter.rs:559-568` (`temperature_range: Some((0.0, 2.0))`, `supports_tool_calling` already `false`, unchanged). Two adapters' existing tests must flip: `openai/adapter.rs` `test_get_capabilities` (~line 698-705) currently asserts `assert!(caps.supports_tool_calling)` — must become `assert!(!caps.supports_tool_calling)`; `anthropic/adapter.rs` `test_get_capabilities`-equivalent at `test_anthropic_provider_capabilities` (~line 740-756) same flip.
3. **Every other construction site** (D-15's list, spot-checked live and confirmed present): `crates/paladin-llm/src/mock.rs:265-273,376-384` (two `get_capabilities` impls), plus the sites named in CONTEXT.md's canonical refs under `src/application/services/paladin/{temperature_service,planning_service,prompt_generation_service,paladin_execution_service}.rs` and `crates/paladin-battalion/src/grove_service.rs:1110` — all take `temperature_range: None` (or `..Default::default()` where the site already uses that pattern).
4. **`Formation::validate`** — `crates/paladin-core/src/platform/container/battalion/formation.rs:108-117`. Change `if self.paladins.len() < 2` to `< 1`, update the error message's "at least 2" to "at least 1". Leave `test_auto_selects_formation_for_single_paladin` (`commander.rs:1911-1927`) and Phalanx's independent Majority-≥3 check (`phalanx.rs:141-146`) untouched.
5. **`citadel.rs` rename** — `crates/paladin-core/src/platform/container/citadel.rs:279-290` struct `BattalionConfig` -> `BattalionCheckpointConfig` (keep 3 fields + serde shape unchanged per ADR-0001). Update in-file consumers at `citadel.rs:233,257,442,456,659`, `crates/paladin-memory/src/citadel/file_citadel.rs:507,541`, and **the doctest at `crates/paladin-ports/src/output/citadel_port.rs:358,371`** (not previously cited in ADR-0001's Code Locations — a genuine new finding from this research).

## Assumptions Log

| # | Claim | Section | Risk if Wrong |
|---|-------|---------|---------------|
| A1 | No in-tree code requires `Eq` on `ProviderCapabilities` (only `PartialEq`) | Pitfall 1 / GAP-07 | Low — verified by grep across `crates/` and `src/` for HashSet/BTreeSet/derive usage; found none. If a downstream consumer does rely on `Eq` in a way grep missed (e.g. via a generic bound satisfied transitively), dropping `Eq` would surface as a compile error at that call site, not a silent behavior change |
| A2 | The `tests/cli/` shim only needs to re-export `tests/helpers/`'s existing mocks, with no additional missing symbols | Code Examples / CLI wiring | Medium — only spot-checked `formation_execution_test.rs`'s and `mock_paladin_port.rs`'s import lines; the full compile has not been attempted (would require adding the shim file and feature-flag build, out of scope for research). The plan should treat "wire + compile + fix whatever surfaces" as the task, not assume zero remaining breakage |
| A3 | Fixing `tests/unit/llm/`'s shared helper functions (not individual test bodies) resolves most/all compile errors in each file | Pitfall 3 | Medium — confirmed the helpers are shared and the breakage is concentrated there for the 2 files inspected in the most detail (deepseek, anthropic); did not exhaustively diff every individual test body against current mock-response/assertion APIs, so some individual tests may still need their own fixes after the helper fix |

**If this table is empty:** N/A — three low/medium-risk assumptions remain; everything else in this document was directly verified by reading source or running `cargo test`.

## Open Questions

1. **Does GAP-03's plan also need to change `Commander`'s Council/Conclave/Grove/ChainOfCommand branches (which currently zero out `paladin_results`/`per_paladin_times`/`per_paladin_tokens` in `commander.rs`), or is Formation alone sufficient?**
   - What we know: D-06 explicitly scopes the new end-to-end test to a **direct** `FormationExecutionService` run (bypassing Commander), so Commander's other-strategy branches are out of the test's scope.
   - What's unclear: whether the Herald *rendering* fixes (adding `strategy_used`/`total_tokens` output) should be judged sufficient once Formation's producer-side gap is closed, given that Council/Conclave/Grove/ChainOfCommand branches in `commander.rs` will still produce `BattalionResult`s with empty aggregation maps when run through Commander.
   - Recommendation: scope the code fix to Formation's aggregation (matching D-06's test scope) and the three Heralds' rendering (which benefits every producer once Formation supplies real data); treat the other strategies' zeroed-out fields in `commander.rs` as a known, separately-tracked gap (candidate for the D-12 sweep's ledger notes or a Phase 3 QUAL item), not something this phase's success criteria require closing.

2. **Should the newly-implemented Table Herald rendering also gain its own richer unit test (replacing the stub-matching one at `table_herald.rs:308-343`), or is the new D-06 integration test sufficient?**
   - What we know: the existing unit test would trivially "pass" against a fixed implementation without ever asserting real content, since it uses `paladin_results: vec![]`.
   - What's unclear: whether leaving that unit test as-is (now passing against a real implementation but still asserting only header strings) creates the same "satisfied but shallow" ledger risk this research flagged for the JSON Herald.
   - Recommendation: the plan should rewrite this specific unit test alongside the implementation fix, asserting on non-placeholder content (a real Paladin name, a specific token count) — cheap, and closes the exact gap this research found.

## Environment Availability

| Dependency | Required By | Available | Version | Fallback |
|------------|------------|-----------|---------|----------|
| `rustc`/`cargo` | All work in this phase | ✓ | rustc 1.97.1, cargo 1.97.1 | — |
| `mockito` 1.7.0 | `tests/unit/llm/` fixes (D-10/D-11) | ✓ (already a workspace dependency) | 1.7.0 | — |
| `cargo-llvm-cov` | GAP-06's coverage measurement, if re-run | ✗ (confirmed absent; crates.io returns HTTP 403 in this sandbox, no Docker) | — | Use the offline `rustc -C instrument-coverage` -> `llvm-profdata merge` -> `llvm-cov` pipeline documented at `.planning/phases/01-ground-truth-decision-records/01-coverage-measurement.md`, already proven working in this sandbox (Phase 1's tracer) |
| `cli` feature flag | Compiling `tests/cli/` target and any Battalion-CLI mock work | ✓ (declared in `Cargo.toml:284`, `["dep:clap","dep:dialoguer","dep:indicatif","dep:console","dep:serde_yaml"]`) | — | Run `cargo test --features cli --test cli` explicitly; the default feature set (`default = ["llm-openai"]`) does not include `cli` |

**Missing dependencies with no fallback:** none block this phase's scope — coverage tooling has a documented, already-proven fallback, and it is out of scope per D-04 anyway.

**Missing dependencies with fallback:** `cargo-llvm-cov` (only relevant if GAP-06's review needs a fresh number, which D-04 says it should not).

## Validation Architecture

### Test Framework

| Property | Value |
|----------|-------|
| Framework | Native `cargo test` (unit `#[test]`/`#[tokio::test]` + integration `[[test]]` binaries + doctests). No `nextest`/other harness detected. |
| Config file | Root `Cargo.toml` `[[test]]` section (`Cargo.toml:171-219`); no separate test-framework config file |
| Quick run command | `cargo test -p paladin-battalion <test_name>` (per-crate, per-test — proven fast: the GAP-05 check ran in 0.01s) |
| Full suite command | `cargo test --workspace` (proven live: full run completed well within timeout; 2790 passed / 0 failed / 126 ignored across 35 binaries) |

### Phase Requirements -> Test Map

| Req ID | Behavior | Test Type | Automated Command | File Exists? |
|--------|----------|-----------|-------------------|--------------|
| GAP-01 | Chain of Command 4 strategies + fallback + synthesis | unit + integration (already passing) | `cargo test -p paladin-battalion chain_of_command` and `cargo test --test unit -- battalion::chain_of_command` | ✅ (existing, verified passing) |
| GAP-02 | Phalanx ≥10 concurrent / <1s overhead | integration (already passing) | `cargo test --test lib -- integration::battalion::load_test` | ✅ (existing, verified passing) |
| GAP-03 | Herald renders Battalion name/ID/type/ordered results/aggregated tokens/partial results | new integration test (D-06) | new test in `tests/integration/` (or extend `herald_integration_test.rs`) | ❌ — Wave 0 gap (new file/test needed, plus implementation fixes it will exercise) |
| GAP-04 | Commander normalization + telemetry export | unit (already passing) | `cargo test -p paladin-battalion commander::tests` | ✅ (existing, verified passing) |
| GAP-05 | Auto keyword routing, all 4 families | unit (already passing) | `cargo test -p paladin-battalion test_auto_selects` | ✅ (existing, verified passing — ran live) |
| GAP-06 | Garrison PRD-acceptance review | manual review, not automated | N/A — a written document, not a test | N/A |
| GAP-07 | Formation ≥1, provider-aware temperature, `Eq`-safe `ProviderCapabilities`, `citadel.rs` rename | unit (new + existing) | `cargo test -p paladin-core formation`, `cargo test -p paladin-llm capabilities`, `cargo test -p paladin-ports`, `cargo test --workspace` (doctests) | Partially — Formation/temperature tests need new assertions for the new bound/range; the `Eq`-drop and rename are compile-time-verified by `cargo build`/`cargo test --workspace` doctests |

### Sampling Rate
- **Per task commit:** targeted `cargo test -p <crate> <test_name_or_module>` for the crate touched.
- **Per wave merge:** `cargo test --workspace` (proven to complete without issue in this environment).
- **Phase gate:** Full `cargo test --workspace` green (2790+ passed, 0 failed) pasted as the D-01 baseline evidence, before `/gsd-verify-work`.

### Wave 0 Gaps
- [ ] A new Formation-driven, 3-Herald end-to-end test (D-06) — no existing file covers this; recommend either a new `tests/integration/battalion_herald_end_to_end_test.rs` or extending `tests/integration/herald_integration_test.rs`'s existing (but too-shallow) `test_battalion_formation_with_herald` (`herald_integration_test.rs:426-490`).
- [ ] `tests/cli/helpers.rs` (or equivalent `#[path]` shim) — does not exist; blocks D-09's CLI test cluster from compiling at all.
- [ ] Updated assertions for `Formation::validate`'s new boundary (`test_formation_rejects_zero_paladins` alongside whatever currently tests the old ≥2 bound) — the existing test asserting "at least 2 Paladins" behavior will need its expectation changed, not just left in place.

*(No gap beyond the above — GAP-01/02/04/05's test infrastructure is fully in place and green today.)*

## Security Domain

`security_enforcement` is not set in `.planning/config.json` (absent = enabled), so this section is included, scoped honestly to what a Rust orchestration-library gap-closure phase actually touches.

### Applicable ASVS Categories

| ASVS Category | Applies | Standard Control |
|---------------|---------|-------------------|
| V2 Authentication | No | This phase touches no auth code path (Commander/Formation/Herald/LLM adapters carry no credentials beyond the existing `api_key` fields, unchanged by this phase) |
| V3 Session Management | No | Not applicable — no session state introduced or modified |
| V4 Access Control | No | Not applicable |
| V5 Input Validation | Yes | `Formation::validate` (Paladin-count bound) and `PaladinBuilder::validate` (temperature-range bound) are exactly the input-validation surfaces GAP-07 edits. Standard control already in use: explicit range/count checks returning typed `Err(...ConfigurationError/ValidationError)` rather than panicking — this phase must preserve that pattern (CLAUDE.md: no `unwrap()`/`expect()`/`panic!` in library code) |
| V6 Cryptography | No | Not touched — no key material, hashing, or encryption code is part of this phase's scope |

### Known Threat Patterns for this stack

| Pattern | STRIDE | Standard Mitigation |
|---------|--------|----------------------|
| Silent acceptance of out-of-range temperature (e.g., forgetting the provider-range check order) leading to a request sent to a provider that rejects/mishandles it | Tampering (of implicit contract) | ADR-0004's explicit ordering ("provider range checked first") and inclusive-both-ends comparison, implemented as a typed validation error rather than clamping (ADR-0004 explicitly rejected silent clamping, since it hides the substitution from the caller) |
| Metadata export (`export_metadata`, `commander.rs:880-913`) writing to an operator-configured directory | Information Disclosure (low severity — local filesystem only) | Already non-fatal and logs warnings rather than failing the run; no new risk introduced by this phase since `metadata_output_dir` is unchanged by GAP-07 |
| Panicking on malformed/impossible states in newly-touched validation code | Denial of Service (of the calling process) | CLAUDE.md's existing rule: return `Result`, never `panic!`/`unwrap()`/`expect()` in library code — applies directly to the `Formation::validate` and `PaladinBuilder::validate` edits this phase makes |

## Sources

### Primary (HIGH confidence — direct code reads and live command execution, this session)
- `git rev-parse HEAD` -> `fb4b9420b2c5c0e3ef8bc2e672ca6960ceb2efb6`; `git branch --show-current` -> `release/v0.7.0`; `rustc -vV` -> `rustc 1.97.1 (8bab26f4f 2026-07-14)`; `cargo --version` -> `cargo 1.97.1 (c980f4866 2026-06-30)`
- `cargo test --workspace` (live, full run) -> 2790 passed / 0 failed / 126 ignored across 35 binaries/doctest groups
- `cargo test -p paladin-battalion test_auto_selects_campaign_for_workflow_keywords` (live) -> 1 passed / 0 failed
- Direct reads: `crates/paladin-battalion/src/commander.rs`, `formation_service.rs`, `phalanx_service.rs`; `crates/paladin-core/src/platform/container/{battalion/mod.rs,battalion/formation.rs,citadel.rs,execution_result.rs,prompt.rs,autonomous_config.rs}`; `crates/paladin-herald/src/{json_herald.rs,markdown_herald.rs,table_herald.rs}`; `crates/paladin-ports/src/output/{llm_port.rs,citadel_port.rs}`; `crates/paladin-llm/src/{openai,anthropic,deepseek}/adapter.rs`, `mock.rs`, `provider_factory.rs`; `src/application/services/paladin/{paladin_builder.rs,temperature_service.rs}`; `tests/unit/mod.rs`, `tests/unit/llm/*.rs`, `tests/cli/mod.rs`, `tests/cli/*_test.rs`, `tests/integration/mod.rs`, `tests/integration/battalion/*.rs`, `tests/integration/commander_integration_tests.rs`, `tests/integration/herald_integration_test.rs`, `tests/helpers/*.rs`; `Cargo.toml` `[[test]]`/`[features]` sections
- `rustc --edition 2024` compile of a `std::env::set_var` one-liner (live) -> confirmed `E0133` unsafe-fn error

### Secondary (MEDIUM confidence)
- `.planning/decisions/000{1,2,3,4,5}-*.md` (ADR-0001…0005) and `.planning/REQUIREMENTS.md:195-259` — read in full, cross-checked against the live code cited above (all cross-checks confirmed the ADRs' code-location claims accurate, with the two additions noted above: the `Eq`-derive gotcha and the `citadel_port.rs` doctest site)

### Tertiary (LOW confidence)
- None used — no web search was performed (this phase required no external library research; all providers configured for this project returned unavailable, and the phase domain is 100% internal-codebase gap closure)

## Metadata

**Confidence breakdown:**
- Standard stack / reused patterns: HIGH — every cited pattern was read directly from the live source tree
- Architecture (Herald gap, Formation aggregation gap, CLI test wiring gap): HIGH — each finding was reproduced by reading the actual implementation, not inferred from the ledger's verdicts
- GAP-07 mechanics (Eq derive, doctest site): HIGH — reproduced with a standalone `rustc` compile and direct `grep`/`Read` of the exact files
- Pitfalls around `tests/unit/llm/`: HIGH for the 2 files inspected in depth (deepseek, anthropic); MEDIUM for exhaustiveness of every individual test body (see Assumptions Log A3)
- GAP-06 (Garrison review): LOW-effort research needed and given — this is a writing task against an already-located PRD, not a code question

**Research date:** 2026-07-31
**Valid until:** Effectively pinned to the current commit (`fb4b9420b2c5c0e3ef8bc2e672ca6960ceb2efb6`) — any further commits to the crates cited above (especially `commander.rs`, `formation_service.rs`, the three Herald files, or `llm_port.rs`) should trigger a re-check of the specific line numbers cited here before the plan is executed.
