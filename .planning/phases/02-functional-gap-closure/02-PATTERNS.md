# Phase 2: Functional Gap Closure - Pattern Map

**Mapped:** 2026-07-31
**Files analyzed:** 13 (files to be created/modified, per CONTEXT.md/RESEARCH.md)
**Analogs found:** 12 / 13

## File Classification

| New/Modified File | Role | Data Flow | Closest Analog | Match Quality |
|--------------------|------|-----------|-----------------|----------------|
| `crates/paladin-ports/src/output/llm_port.rs` (`ProviderCapabilities` + `Default` impl + doctest) | model (ports value object) | transform | itself, prior shape at lines 753-782 | exact (self-modification) |
| `crates/paladin-llm/src/openai/adapter.rs` (`get_capabilities`, `test_get_capabilities`) | service (adapter) | request-response | `crates/paladin-llm/src/deepseek/adapter.rs:559-568` (already `false` + a range) | exact — same trait method, target shape already shipped one adapter over |
| `crates/paladin-llm/src/anthropic/adapter.rs` (`get_capabilities`, its capabilities test) | service (adapter) | request-response | `crates/paladin-llm/src/deepseek/adapter.rs:559-568` | exact |
| `crates/paladin-llm/src/deepseek/adapter.rs` (add `temperature_range`, rest unchanged) | service (adapter) | request-response | itself (only additive) | exact |
| `crates/paladin-llm/src/mock.rs:265-273,376-384` (two `get_capabilities` impls) + all other `ProviderCapabilities{ .. }` construction sites (`grove_service.rs:1110`, `temperature_service.rs:356`, `planning_service.rs:641`, `prompt_generation_service.rs:299`, `paladin_execution_service.rs:2541,2631,2715`) | config/fixture | transform | the three adapters above, for the field to add; `..Default::default()` pattern already used at some sites | role-match — mechanical field addition, not new logic |
| `src/application/services/paladin/paladin_builder.rs` (temperature validation, ~line 1112) | service (builder/validator) | request-response | itself; also `Formation::validate` (`formation.rs:108-117`) for the "typed Result, no panic" convention | exact |
| `crates/paladin-core/src/platform/container/battalion/formation.rs:108-117` (`Formation::validate`) | model (domain invariant) | CRUD/validation | itself (bound change only, structure unchanged) | exact |
| `crates/paladin-battalion/src/commander.rs:1911-1927` (`test_auto_selects_formation_for_single_paladin` — verify untouched) | test | request-response | n/a — existing passing test, no analog needed | n/a |
| `crates/paladin-core/src/platform/container/citadel.rs:279-290` (`BattalionConfig` → `BattalionCheckpointConfig`) rename + consumers at `:233,257,442,456,659` | model (domain value object) | CRUD | itself; ADR-0001's fixed target name | exact |
| `crates/paladin-memory/src/citadel/file_citadel.rs:507,541` | service (persistence adapter) | file-I/O | itself (identifier rename only, no shape change) | exact |
| `crates/paladin-ports/src/output/citadel_port.rs:358,371` (doctest) | test (doctest) | transform | itself | exact |
| `crates/paladin-battalion/src/formation_service.rs:187-278` (`execute_internal` — add per-Paladin aggregation) | service (execution service) | event-driven/CRUD | **`crates/paladin-battalion/src/phalanx_service.rs:264-282`** (per-Paladin times/tokens/total aggregation loop) | exact — this is the primary analog for the whole phase |
| `crates/paladin-herald/src/json_herald.rs:138-160` (`battalion_result_to_json`) | service (formatter adapter) | transform | `markdown_herald.rs`'s battalion formatting (richer already) and the target `BattalionResult` struct fields | role-match — needs the fields Markdown already partially covers |
| `crates/paladin-herald/src/markdown_herald.rs` (battalion formatting section, ~90-150 and below) | service (formatter adapter) | transform | itself — extend with `strategy_used`/`total_tokens` | role-match |
| `crates/paladin-herald/src/table_herald.rs:145-184` (`format_battalion_result`) + its test at `:308-343` | service (formatter adapter) | transform | JSON/Markdown Heralds' real (non-stub) `battalion_result_to_*` implementations, for what fields to read from `_result` | role-match — currently a stub, target shape is "what the other two Heralds do" |
| New: `tests/integration/battalion_herald_end_to_end_test.rs` (or extend `herald_integration_test.rs`) | test (integration) | event-driven | `tests/integration/herald_integration_test.rs:426-490` (`test_battalion_formation_with_herald`) + `tests/integration/commander_integration_tests.rs`'s `IntegrationMockPaladinPort` | exact — deepen the existing shallow test, reuse its mock-port pattern |
| New: `tests/cli/helpers.rs` | test (test-helper shim/module) | transform | `tests/helpers/mod.rs` (barrel re-exporting `MockLlmAdapter`/`MockPaladinPort`) | exact — this file's whole job is to re-export that barrel into the `cli` `[[test]]` target's module tree |
| `tests/cli/mod.rs` (uncomment 5 `mod` lines) | config (test-target barrel) | transform | itself (already has the commented-out lines, and the 4 active `mod` lines to model formatting on) | exact |
| `tests/unit/mod.rs` (add `pub mod llm;`) | config (test-target barrel) | transform | itself — sibling `pub mod battalion;` / `pub mod arsenal;` lines already wired the same way | exact |
| `tests/unit/llm/{deepseek_adapter_test.rs,anthropic_adapter_test.rs,provider_factory_test.rs}` (repair shared helpers) | test (unit, HTTP-mock) | request-response | `tests/integration/openai_embedding_tests.rs` (async mockito + current `LlmRequest`/prompt-type construction) | exact — the fix pattern already exists elsewhere in the tree |

## Pattern Assignments

### `crates/paladin-battalion/src/formation_service.rs` (service, event-driven/CRUD) — the phase's central analog

**Analog:** `crates/paladin-battalion/src/phalanx_service.rs:264-282`

**Core aggregation pattern to port** (verified live, `phalanx_service.rs:264-282`):
```rust
let mut per_paladin_times = HashMap::new();
let mut per_paladin_tokens = HashMap::new();
let mut total_tokens: u64 = 0;

// Track which successful results map to which paladin names
// Results are returned in order matching successful paladins
let successful_names: Vec<&String> = paladin_names
    .iter()
    .filter(|name| !failed_names.contains(name))
    .collect();

for (i, result) in paladin_results.iter().enumerate() {
    if let Some(name) = successful_names.get(i) {
        per_paladin_times.insert((*name).clone(), result.execution_time_ms);
        per_paladin_tokens
            .insert((*name).clone(), TokenUsage::from_total(result.token_count));
        total_tokens += u64::from(result.token_count);
    }
}
```
Phalanx also builds `node_errors: Vec<NodeError>` from failure strings (see the same file just above this excerpt) and constructs `BattalionResult { .. }` as a literal (not via `::new()`), setting `per_paladin_times`, `per_paladin_tokens`, `total_tokens` explicitly.

**What Formation currently does instead** (the gap, `formation_service.rs:264-278`, verified live):
```rust
// Create result
let result = BattalionResult::new(
    battalion_id,
    formation.config.name.clone(),
    started_at,
    current_input, // Final output from last Paladin
    paladin_results,
);

Ok(result)
```
`BattalionResult::new` (`crates/paladin-core/src/platform/container/battalion/mod.rs:601-635`) defaults `per_paladin_times`/`per_paladin_tokens` to empty maps and `total_tokens` to `0` — this is the literal producer-side gap GAP-03 closes.

**Port instructions for the plan:** Formation's loop already knows each Paladin's name at iteration time (`for (index, paladin) in formation.paladins.iter().enumerate()`, using `paladin.node.name`, `formation_service.rs:203-209`). Build `per_paladin_times`/`per_paladin_tokens`/`total_tokens` inline in that same loop as results are pushed (simpler than Phalanx's after-the-fact `successful_names` reconciliation, because Formation's loop already conditionally continues vs fails per `ErrorStrategy`), then either switch to constructing `BattalionResult { .. }` as a literal (matching Phalanx's approach) or extend `BattalionResult::new` with the additional maps before calling it.

**Error handling pattern to mirror:** Formation's `ErrorStrategy` match (`FailFast` returns `Err` immediately; `ContinueOnError`/`RetryThenContinue` records into `AggregatedError` and continues) is already correct and untouched by this change — only the success-path aggregation needs adding, alongside `node_errors` population using the same `NodeError { node_name, error }` shape Phalanx uses.

---

### `crates/paladin-herald/src/table_herald.rs` (service/formatter adapter, transform)

**Analog:** JSON Herald's real (non-stub) implementation, for shape; Markdown Herald for the richer already-partial version.

**The stub to replace** (`table_herald.rs:145-184`, verified live — parameter is unused, rows are hardcoded):
```rust
fn format_battalion_result(
    &self,
    _result: &paladin_core::platform::container::herald::BattalionResult,
) -> Result<String, HeraldError> {
    let mut output = String::new();
    let mut table = self.create_table();

    table.set_header(vec![
        Cell::new("Paladin").add_attribute(Attribute::Bold),
        Cell::new("Status").add_attribute(Attribute::Bold),
        Cell::new("Duration").add_attribute(Attribute::Bold),
        Cell::new("Tokens").add_attribute(Attribute::Bold),
    ]);

    // Add placeholder rows for each paladin (will be replaced with actual data)
    table.add_row(vec![
        Cell::new("paladin_1"),
        self.format_status("Success"),
        Cell::new("1.2s").set_alignment(CellAlignment::Right),
        Cell::new("400").set_alignment(CellAlignment::Right),
    ]);
    table.add_row(vec![
        Cell::new("paladin_2"),
        self.format_status("Success"),
        Cell::new("2.1s").set_alignment(CellAlignment::Right),
        Cell::new("550").set_alignment(CellAlignment::Right),
    ]);
    // ...
}
```
Must iterate `result.paladin_results` (order preserved), read real names via `result.per_paladin_times`/`result.per_paladin_tokens` (keyed by Paladin name — see `BattalionResult` field doc comments below), and render `result.strategy_used`, `result.total_tokens`, and partial-result info from `result.node_errors` / `paladin_success_count` / `paladin_failure_count`.

**`BattalionResult`'s real field shape to read from** (`crates/paladin-core/src/platform/container/battalion/mod.rs:580-598`, verified live):
```rust
pub per_paladin_times: HashMap<String, u64>,
pub per_paladin_tokens: HashMap<String, TokenUsage>,
pub total_tokens: u64,
pub paladin_success_count: usize,
pub paladin_failure_count: usize,
pub node_errors: Vec<NodeError>,
```

**Its own unit test to rewrite alongside the fix** (`table_herald.rs:308-343` — currently constructs `paladin_results: vec![]` and only asserts hardcoded header strings, which is the exact litmus-test failure this research identified):
```rust
let result = paladin_core::platform::container::herald::BattalionResult {
    battalion_id: Uuid::new_v4(),
    battalion_name: "Test Battalion".to_string(),
    // ...
    paladin_results: vec![],   // <- must become non-empty with distinctive names/tokens
    // ...
};
let formatted = output.unwrap();
assert!(formatted.contains("Battalion Execution Results"));  // <- assertions must move to content, not headers
```
Per Pitfall 5 in RESEARCH.md, rewrite this test to assert a distinctive Paladin name and a specific non-round token count that could only appear if `_result` were actually read.

**JSON Herald's current battalion conversion, to extend with the missing fields** (`json_herald.rs:138-160`, verified live — omits `strategy_used`/`total_tokens`/`node_errors`):
```rust
fn battalion_result_to_json(&self, result: &BattalionResult) -> Value {
    let paladin_results: Vec<Value> = result
        .paladin_results
        .iter()
        .map(|r| self.paladin_result_to_json(r))
        .collect();

    let mut json = json!({
        "battalion_id": result.battalion_id,
        "battalion_name": result.battalion_name,
        "status": format!("{:?}", result.status),
        "paladin_results": paladin_results,
    });

    if self.config.include_metadata {
        json["metadata"] = json!({
            "paladin_count": result.paladin_results.len(),
            "timestamp": chrono::Utc::now().to_rfc3339(),
        });
    }

    json
}
```
Add `"strategy_used": format!("{:?}", result.strategy_used)`, `"total_tokens": result.total_tokens`, `"per_paladin_tokens"`, `"node_errors"` to the `json!({...})` literal.

---

### `crates/paladin-ports/src/output/llm_port.rs` (`ProviderCapabilities`) — the D-16 tracer

**Analog:** itself; target shape mirrors `crates/paladin-llm/src/deepseek/adapter.rs:559-568` which already reports `supports_tool_calling: false` plus a documented range.

**Current struct + derive to change** (`llm_port.rs:752-769`, verified live):
```rust
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ProviderCapabilities {
    pub supports_streaming: bool,
    pub supports_tool_calling: bool,
    pub supports_function_calling: bool,
    pub supports_vision: bool,
    pub supports_embeddings: bool,
    pub max_context_tokens: Option<u32>,
    pub supports_system_messages: bool,
}
```
**Required change:** drop `Eq` from the derive (kept `PartialEq`) because `f32` does not implement `Eq` — confirmed no in-tree code needs `Eq` (no `HashSet<ProviderCapabilities>`/`BTreeSet` usage found). Add `pub temperature_range: Option<(f32, f32)>` to the struct and to the `Default` impl (`llm_port.rs:771-782`, currently sets six fields — add `temperature_range: None`).

**Adapter-side target shape already shipped** (`crates/paladin-llm/src/deepseek/adapter.rs:559-568` per RESEARCH.md — read this file directly when implementing to confirm exact literal, already `supports_tool_calling: false`):
```
ProviderCapabilities { ..., supports_tool_calling: false, temperature_range: Some((0.0, 2.0)), ... }
```

**OpenAI adapter's current (to-be-flipped) construction** (`crates/paladin-llm/src/openai/adapter.rs:642-651`, verified live):
```rust
fn get_capabilities(&self) -> ProviderCapabilities {
    ProviderCapabilities {
        supports_streaming: true,
        supports_tool_calling: true,   // <- flips to false (D-14)
        supports_function_calling: true,
        supports_vision: true,
        max_context_tokens: Some(128000),
        supports_embeddings: true,
        supports_system_messages: true,
        // + temperature_range: Some((0.0, 1.0))  (new field, D-15)
    }
}
```
Anthropic adapter follows the identical shape at `anthropic/adapter.rs:518-527` with `temperature_range: Some((0.0, 1.0))`.

**Every other construction site** (mock, service defaults, test fixtures) takes `temperature_range: None` — prefer `..Default::default()` where the site already uses that spread pattern.

---

### `crates/paladin-core/src/platform/container/battalion/formation.rs` (domain validation)

**Analog:** itself (bound-only change); `src/application/services/paladin/paladin_builder.rs`'s temperature validation for the sibling "typed `Result`, no panic" convention.

**Current validation to relax** (`formation.rs:99-117`, verified live):
```rust
/// Validate Formation requirements
///
/// Ensures:
/// - At least 2 Paladins are present
///
/// # Returns
///
/// * `Ok(())` - Validation passed
/// * `Err(BattalionError::ValidationError)` - Validation failed
fn validate(&self) -> Result<(), BattalionError> {
    if self.paladins.len() < 2 {
        return Err(BattalionError::ValidationError(format!(
            "Formation requires at least 2 Paladins, got {}",
            self.paladins.len()
        )));
    }
    Ok(())
}
```
Change `< 2` to `< 1` and update both the doc comment ("At least 2" → "At least 1") and the error message ("at least 2" → "at least 1"). 0 Paladins still rejected; 1 now accepted; 2 unchanged. Leave `commander.rs:1911-1927`'s `test_auto_selects_formation_for_single_paladin` and Phalanx's independent `phalanx.rs:141-146` Majority ≥3 check untouched.

---

### `crates/paladin-core/src/platform/container/citadel.rs` (`BattalionConfig` → `BattalionCheckpointConfig`)

**Current placeholder struct to rename** (`citadel.rs:275-290`, verified live):
```rust
/// Configuration parameters for Battalion orchestration
///
/// Contains settings that control how a Battalion executes its Paladins.
/// This is a placeholder and will be expanded in Epic 4.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BattalionConfig {
    #[serde(default)]
    pub max_concurrency: Option<usize>,
    #[serde(default)]
    pub timeout_seconds: Option<u64>,
    #[serde(default)]
    pub continue_on_error: bool,
}
```
Rename to `BattalionCheckpointConfig` (ADR-0001's fixed target). Keep exactly these 3 fields, the `#[serde(default)]` attributes, and the derive list unchanged (no persisted-schema change, no migration). Update in-file consumers at `citadel.rs:233,257,442,456,659` and `crates/paladin-memory/src/citadel/file_citadel.rs:507,541`.

**The doctest site not in ADR-0001's original Code Locations — must not be missed** (`crates/paladin-ports/src/output/citadel_port.rs:356-374`, verified live, `rust,no_run` — still type-checked by `cargo test`):
```rust
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
Both the import and the `BattalionConfig::default()` call must become `BattalionCheckpointConfig` in the same commit, or `cargo test` (doctests) fails workspace-wide.

---

### `tests/cli/helpers.rs` (new file — test-helper shim)

**Analog:** `tests/helpers/mod.rs` (verified live, full file):
```rust
//! Test helpers and utilities
//!
//! Common test infrastructure including mocks, fixtures, and helper functions.

pub mod mock_arsenal_adapter;
pub mod mock_llm_adapter;
pub mod mock_paladin_port;

pub use mock_arsenal_adapter::MockArsenalPort;
pub use mock_llm_adapter::{
    Invocation, MockLlmAdapter, MockResponse, create_mock_with_mixed_responses,
    create_mock_with_responses, create_mock_with_tool_calls, create_test_paladin_with_mock,
};
pub use mock_paladin_port::MockPaladinPort;
```
`tests/cli/*_test.rs` files already `use crate::helpers::{MockLlmAdapter, MockPaladinPort};` (confirmed at `tests/cli/formation_execution_test.rs:1-16`) but `cli` is its own `[[test]]` target (`Cargo.toml:211-214`, `path = "tests/cli/mod.rs"`) and does not automatically see the sibling `tests/helpers/` directory used by the `unit`/`lib` targets. Create `tests/cli/helpers.rs` (or a `mod.rs` entry) using a path-attribute re-export, e.g. `#[path = "../helpers/mod.rs"] pub mod helpers;`, then declare `mod helpers;` in `tests/cli/mod.rs`.

**`tests/cli/mod.rs`'s current barrel to uncomment** (verified live, full commented block):
```rust
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

// CLI output snapshot tests (Task 4.0 - Epic 24)
mod error_output_test;
mod help_output_test;
mod progress_output_test;
mod table_output_test;
```
Per D-09/RESEARCH.md's scoping, uncomment only `error_handling_test`, `formation_execution_test`, `paladin_execution_test`, `phalanx_execution_test`, `tool_integration_test` — leave `arsenal_config_test`, `environment_tests`, `garrison_config_test`, `integration_tests` commented unless D-12's sweep separately confirms they belong in Phase 2. This file already has the `unsafe { std::env::set_var(...) }` convention (`ensure_no_color()`, shown in full above) — the same wrapping pattern is required in any reactivated test that touches env vars (see `tests/unit/llm/provider_factory_test.rs` below).

---

### `tests/unit/llm/{deepseek_adapter_test.rs,anthropic_adapter_test.rs,provider_factory_test.rs}` (unit, HTTP-mock)

**Analog:** `tests/integration/openai_embedding_tests.rs:22,68,116,...` (async-safe mockito pattern already proven in this repo).

**The breakage pattern to fix (mockito sync-in-async):** `setup_mock_server()` in both `deepseek_adapter_test.rs:16` and `anthropic_adapter_test.rs:17` calls blocking `Server::new()` from inside `#[tokio::test]` bodies — panics with a runtime-nesting error at test-run time (not a compile error). Fix: make the helper `async fn`, call `Server::new_async().await`, `.await` at each call site — mirror `openai_embedding_tests.rs`'s existing working calls.

**The env-var unsafe-block pattern to copy** (`tests/lib.rs:82-84`, `tests/integration/system_log_integration_test.rs:372-374,399-401`, `tests/integration/cli_integration_test.rs:610,622` — all already wrap `set_var`/`remove_var`):
```rust
// SAFETY: <justification specific to the call site>
unsafe {
    std::env::set_var("NO_COLOR", "1");
}
```
`provider_factory_test.rs` calls `env::set_var`/`env::remove_var` unwrapped ≥8 times in `test_get_default_provider`/`test_list_available_providers` — these will fail to compile with `E0133` the moment `tests/unit/mod.rs` gains `pub mod llm;`. Wrap each call using the pattern above.

**`tests/unit/mod.rs`'s existing wiring convention to extend** (verified live — sibling modules already wired the same way):
```rust
pub mod battalion;
pub mod arsenal;
// missing: pub mod llm;   <- D-10 adds this line
```

---

## Shared Patterns

### "No panic in library code, typed Result instead" (validation surfaces)
**Source:** `crates/paladin-core/src/platform/container/battalion/formation.rs:108-117` and `src/application/services/paladin/paladin_builder.rs:1112-1118`
**Apply to:** `Formation::validate`'s bound change and `PaladinBuilder`'s provider-aware temperature check — both must keep returning typed `Err(BattalionError::ValidationError(...))` / `Err(PaladinError::ConfigurationError(...))`, never `panic!`/`unwrap()`/`expect()`, per CLAUDE.md.

### Per-Paladin aggregation (times/tokens/total)
**Source:** `crates/paladin-battalion/src/phalanx_service.rs:264-282` (full excerpt above)
**Apply to:** `crates/paladin-battalion/src/formation_service.rs:187-278`'s `execute_internal` — this is the single most load-bearing shared pattern in the phase; every downstream Herald fix depends on Formation actually populating these maps.

### `..Default::default()` spread for mechanical field addition
**Source:** existing `ProviderCapabilities { .. }` construction sites that already use the spread pattern (see D-15's construction-site list in RESEARCH.md/CONTEXT.md)
**Apply to:** every non-adapter `ProviderCapabilities` construction site once `temperature_range` is added — use the spread where the site already permits it so the field addition stays purely mechanical (per D-15).

### `unsafe { std::env::set_var/remove_var }` wrapping
**Source:** `tests/lib.rs:82-84`, `tests/integration/system_log_integration_test.rs:372-374,399-401`, `tests/integration/cli_integration_test.rs:610,622`, `tests/cli/mod.rs`'s `ensure_no_color()`
**Apply to:** `tests/unit/llm/provider_factory_test.rs`'s ≥8 unwrapped `env::set_var`/`remove_var` calls, and any other reactivated test (D-12 sweep findings) touching process environment.

### Async-safe mockito server construction
**Source:** `tests/integration/openai_embedding_tests.rs:22,68,116,171,186,207,245`
**Apply to:** `tests/unit/llm/{deepseek_adapter_test.rs,anthropic_adapter_test.rs}`'s `setup_mock_server()` helpers.

## No Analog Found

| File | Role | Data Flow | Reason |
|------|------|-----------|--------|
| `.planning/decisions/0007-battalion-cancellation-deferral.md` (new ADR, D-08) | doc/config (ADR) | n/a | Not a code file — no source analog applies; follow ADR-0004's shape (`Status · Date · Context · Decision · Considered Options · Code Locations · Code Conformance · Downstream Consumers`) as named in CONTEXT.md, and confirm it parses under `.claude/gsd-core/bin/lib/adr-parser.cjs` the way ADR-0001..0006 do. |

## Metadata

**Analog search scope:** `crates/paladin-battalion/`, `crates/paladin-herald/`, `crates/paladin-ports/src/output/`, `crates/paladin-llm/src/{openai,anthropic,deepseek}/`, `crates/paladin-core/src/platform/container/{battalion,citadel.rs}`, `tests/{unit,integration,cli,helpers}/`.
**Files scanned:** ~20 read directly (targeted `sed`/`Read` ranges, no full-file loads over ~400 lines except where files were already short).
**Pattern extraction date:** 2026-07-31
```
