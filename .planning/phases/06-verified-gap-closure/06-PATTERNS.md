# Phase 6: Verified Gap Closure - Pattern Map

**Mapped:** 2026-08-05
**Files analyzed:** 12 (create/modify targets across CLOSE-01, CLOSE-02, CLOSE-03)
**Analogs found:** 12 / 12

## File Classification

| New/Modified File | Role | Data Flow | Closest Analog | Match Quality |
|---|---|---|---|---|
| `crates/paladin-core/src/platform/container/battalion/grove.rs` (`GroveConfig`, `GroveBuilder`) | config/model | CRUD (builder-set field) | same file — `fallback_tree` field + its `GroveBuilder` setter | exact (in-file sibling field) |
| `crates/paladin-battalion/src/grove_service.rs:537` | service | request-response (LLM call construction) | `src/application/services/paladin/planning_service.rs` / `prompt_generation_service.rs` (model sourced from config/param, not hardcoded) | role-match, cross-crate precedent |
| `.planning/decisions/0013-*.md` (new ADR) | doc | n/a | `.planning/decisions/0011-vision-port-surfaces.md`, `0012-live-api-test-key-behaviour.md` | exact (same doc family) |
| `src/application/cli/config/paladin_config.rs` (`PaladinYamlConfig.autonomous`) | config | CRUD (deserialize) | same file — `GarrisonConfig`/`ArsenalConfig` fields (`:100`, `:124`) | exact (in-file sibling section) |
| `src/application/cli/commands/agent.rs` (`handle_agent_run` override wiring) | controller/CLI | request-response | same file — image/document flag validation + `PaladinBuilder` composition (`:167-332`) | exact (in-file sibling wiring) |
| `crates/paladin-battalion/benches/battalion_benchmarks.rs` (new `benchmark_chain_of_command`) | test (benchmark) | batch | same file — `benchmark_campaign_branching_dag` (`:122-155`) | exact (in-file sibling benchmark) |
| `docs/src/appendix/performance-baseline.md` (new dated ChainOfCommand row) | doc | n/a | same file — existing P50/P95/P99 derivation section (`:624`) | exact |
| `crates/paladin-battalion/src/campaign_service.rs` (Herald wiring) | service | request-response | `crates/paladin-battalion/src/formation_service.rs` (`:36-125`), `phalanx_service.rs` (`:107`) | exact (identical pattern to replicate) |
| `crates/paladin-battalion/src/chain_of_command_service.rs` (Herald wiring) | service | request-response | `crates/paladin-battalion/src/formation_service.rs` | exact |
| `crates/paladin-battalion/src/commander.rs` (Herald wiring) | service/router | request-response | `crates/paladin-battalion/src/formation_service.rs` | exact |
| `tests/integration/*` (new WARN-01 composite test) | test | event-driven / request-response | `tests/integration/battalion_herald_end_to_end_test.rs` | exact |
| `tests/integration/llm_live_api_tests.rs:61` + `tests/integration/mod.rs:34-35` (doc-only) | test/doc | n/a | same files — `require_api_key`, `#[ignore]` attributes, `#[cfg(feature = "live-api-tests")]` gate | exact (doc-only edit of same file) |
| `crates/paladin-ports/src/output/vision_port.rs`, `vision_llm_port.rs` (rustdoc only) | doc/trait | n/a | same files — existing trait doc header patterns | exact |

## Pattern Assignments

### `crates/paladin-core/src/platform/container/battalion/grove.rs` (config field + builder setter)

**Analog:** same file, `fallback_tree` field/setter pair — the only other `Option<String>` knob on `GroveConfig`.

**GroveConfig field pattern** (lines 213-217):
```rust
/// Optional fallback Tree name if no good match is found
///
/// If routing fails to find a suitable agent (e.g., no keywords match,
/// similarity too low), the Grove can route to a fallback Tree's first agent.
#[serde(skip_serializing_if = "Option::is_none")]
pub fallback_tree: Option<String>,
```
Copy this shape exactly for `routing_model: Option<String>` (D-01), with rustdoc stating it is required for `RoutingStrategy::LlmRouting` and what happens when absent (D-03).

**`Default for GroveConfig`** (lines 241-250) sets `fallback_tree: None` — mirror with `routing_model: None`.

**`GroveBuilder` struct fields** (lines 294-301) and **`GroveBuilder::new()`** (lines 313-322) both list `fallback_tree: Option<String>` / `fallback_tree: None` alongside the other four knobs — add `routing_model` the same way. Builder setter method for `fallback_tree` (not shown above the read window, but same file, immediately following `name()`/`add_tree()` setters at `:327+`) follows the `pub fn <name>(mut self, ...) -> Self { self.<field> = ...; self }` fluent shape used throughout `GroveBuilder`.

---

### `crates/paladin-battalion/src/grove_service.rs:537` (defect site)

**Analog:** `src/application/services/paladin/planning_service.rs` and `prompt_generation_service.rs` — Epic 21 already removed this exact defect class here by taking `model: &str` as a caller-supplied parameter (never a hardcoded literal) and doing `model: model.to_string()` at the `LlmRequest` construction site.

**Current defect** (`grove_service.rs:537-543`):
```rust
let llm_request = LlmRequest {
    id: uuid::Uuid::new_v4(),
    model: "gpt-4".to_string(), // TODO: Make configurable
    prompt: prompt_item,
    attachments: vec![],
    stream: false,
    metadata: HashMap::new(),
};
```

**Precedent to follow** (`planning_service.rs:131`, `:312`, `:552` — all identical shape):
```rust
model: model.to_string(),
```
where `model` arrives as a typed parameter, not a literal. For Grove, D-01/D-02 route this through `grove.node.config.routing_model` instead of a function parameter (Grove has no per-call model argument in its public API) — so the pattern to copy is "source `model` from a config value, never inline a literal", and the guard shape to copy is the existing `llm_port` guard immediately above at `grove_service.rs:487-491`:
```rust
let llm_port = self.llm_port.as_ref().ok_or_else(|| {
    BattalionError::RoutingError(
        "LLM port not configured for LLM-based routing".to_string(),
    )
})?;
```
D-02's hard-error on missing `routing_model` should be a sibling `ok_or_else` returning `BattalionError::RoutingError("routing_model not configured for LLM-based routing".to_string())` — same error variant, same `.ok_or_else` idiom, placed right before/alongside the `llm_port` guard.

**Mock `LlmPort` pattern for D-04's recording mock** — `grove_service.rs:1064+` already contains several `#[async_trait] impl LlmPort for Mock...` blocks returning `model: "mock-model"` in their responses; extend one of these (do not build a parallel harness) to capture `request.model` into a `Mutex<Option<String>>` or similar, then assert against it.

---

### `src/application/cli/config/paladin_config.rs` (`PaladinYamlConfig.autonomous`)

**Analog:** same file — `GarrisonConfig` (lines 66-71) and `ArsenalConfig` (lines 121-126) show the established optional-section shape on `PaladinYamlConfig`.

**Pattern to copy** (lines 66-71):
```rust
/// Optional garrison (memory) configuration
#[serde(skip_serializing_if = "Option::is_none")]
pub garrison: Option<GarrisonConfig>,
```
D-06 adds, immediately alongside:
```rust
/// Optional autonomous features configuration (planning, prompt generation,
/// dynamic temperature, handoffs) — reused directly from `paladin_core`.
#[serde(skip_serializing_if = "Option::is_none")]
pub autonomous: Option<paladin_core::platform::container::autonomous_config::AutonomousConfig>,
```
Note the deliberate deviation from the `GarrisonConfig`/`ArsenalConfig` convention of defining a CLI-local mirror type — D-06 reuses `AutonomousConfig` directly since it already derives `Serialize, Deserialize, PartialEq, Default` (`crates/paladin-core/src/platform/container/autonomous_config.rs:69`).

**Domain mapping precedent** — `crates/paladin-core/src/platform/container/paladin_config.rs:61` already has `pub autonomous: Option<AutonomousConfig>` on `PaladinConfig`, and its builder (`PaladinConfigBuilder::autonomous`, lines 140-144) is the exact fluent-setter shape:
```rust
/// Set autonomous features configuration
pub fn autonomous(mut self, config: AutonomousConfig) -> Self {
    self.autonomous = Some(config);
    self
}
```
This is domain-level and already done (D-08) — do not re-touch it; only the CLI YAML struct and `handle_agent_run` plumbing are missing.

---

### `src/application/cli/commands/agent.rs` (`AgentRunArgs` flags + `handle_agent_run` override wiring)

**Analog:** same file — the four autonomous flags already exist at lines 76-92 (comment: `// Autonomous feature flags (override config file)`) but are unread. The wiring pattern to copy is the file's own image/document validation-then-use flow (lines 167-332) and the final `PaladinBuilder` composition at line 310+:

**Existing flags** (lines 76-92):
```rust
// Autonomous feature flags (override config file)
/// Enable autonomous planning mode (MaxLoops::Auto)
#[arg(long = "auto-plan")]
pub auto_plan: bool,

/// Enable automatic prompt generation
#[arg(long = "auto-prompt")]
pub auto_prompt: bool,

/// Enable dynamic temperature adjustment based on task type
#[arg(long = "dynamic-temp")]
pub dynamic_temp: bool,

/// Enable agent handoff capabilities
#[arg(long = "enable-handoffs")]
pub enable_handoffs: bool,
```

**Builder composition pattern to extend** (lines 310-332):
```rust
let mut builder = PaladinBuilder::new(llm_port)
    // ... existing chained setters ...
    ;
if !args.stop_words.is_empty() { // representative existing conditional-chain idiom
    for word in stop_words { builder = builder.add_stop_word(word); }
}
if args.vision_enabled { // representative
    builder = builder.enable_vision(true);
}
let paladin = builder.build().await?;
```
D-05/D-07's additive-only override: for each flag, `if args.auto_plan { builder = builder.enable_autonomous_planning(true); }` — mirroring the `if <cond> { builder = builder.<setter>(...); }` idiom already used for `vision_enabled`/stop words in this same function, sourcing the YAML baseline first via `config.autonomous` (new field) then applying flag overrides on top per D-07 ("a present flag forces that feature on; an absent flag leaves the YAML value untouched").

**Builder setters already available** (no new domain work per D-08) — `src/application/services/paladin/paladin_builder.rs:546` `enable_autonomous_planning`, `:576` `enable_autonomous_prompts`, `paladin.rs:896`-area `with_handoff_config`. Dynamic temperature setter: `enable_dynamic_temperature` (`paladin_builder.rs:607`).

---

### `crates/paladin-battalion/benches/battalion_benchmarks.rs` (new `benchmark_chain_of_command`)

**Analog:** same file, `benchmark_campaign_branching_dag` (lines 122-154) — closest existing benchmark since ChainOfCommand, like Campaign, needs multi-node construction before the `c.bench_function` call (unlike Formation/Phalanx's flat `Vec<Paladin>`).

**Full pattern to copy** (lines 122-154):
```rust
fn benchmark_campaign_branching_dag(c: &mut Criterion) {
    let rt = Runtime::new().expect("runtime");
    let service = CampaignExecutionService::new(Arc::new(MockPaladinPort));

    let mut campaign = Campaign::new(BattalionConfig::new("campaign-bench"));

    let entry = campaign.add_paladin(create_test_paladin("campaign-entry"));
    let branch_a = campaign.add_paladin(create_test_paladin("campaign-branch-a"));
    let branch_b = campaign.add_paladin(create_test_paladin("campaign-branch-b"));
    let join = campaign.add_paladin(create_test_paladin("campaign-join"));

    campaign
        .add_edge(CampaignEdge::new(entry, branch_a, EdgeCondition::Always))
        .expect("entry->a");
    // ... more edges ...

    c.bench_function("battalion/campaign_branching_dag", |b| {
        b.to_async(&rt).iter(|| async {
            let _ = service
                .execute(&campaign, black_box("campaign-input"))
                .await
                .expect("campaign execute");
        });
    });
}
```
For ChainOfCommand, construct via `ChainOfCommandExecutionService::new(Arc::new(MockPaladinPort))` (constructor confirmed at `chain_of_command_service.rs:83`, same `pub fn new(paladin_port: Arc<dyn PaladinPort>) -> Self` shape as Formation/Phalanx/Campaign) and drive one `DelegationStrategy` (e.g. `Automatic`, imported per the module doc comment at `chain_of_command_service.rs:19,29`).

**Registration site** (lines 156-161):
```rust
criterion_group!(
    battalion_benches,
    benchmark_formation_three_agents,
    benchmark_phalanx_five_agents,
    benchmark_campaign_branching_dag
);
```
Add `benchmark_chain_of_command` to this list. `Cargo.toml:34-36` already declares the `[[bench]]` target (`name = "battalion_benchmarks"`, `harness = false`) — no `Cargo.toml` change needed, this is additive within the existing bench file.

**Shared helper already present** — `create_test_paladin(name: &str) -> Paladin` (lines 20-35) builds a `PaladinData` with `model: "mock-model".to_string()`; reuse directly, no new fixture needed.

---

### `docs/src/appendix/performance-baseline.md` (D-13 dated baseline row)

**Analog:** the file's own existing `### P50 / P95 / P99 Derivation` section (line 624) — the nearest-rank formula and `jq` filter documented there for the existing Formation/Phalanx/Campaign entries. Follow the same table shape, but append the ChainOfCommand row under a clearly separate, dated sub-heading/table rather than merging it into the existing 2026-08-02 table, per D-13.

---

### `crates/paladin-battalion/src/{campaign_service,chain_of_command_service,commander}.rs` (WARN-01 Herald wiring)

**Analog:** `crates/paladin-battalion/src/formation_service.rs` (full pattern — 19 references) and `phalanx_service.rs:107` (same pattern, confirms it's the established shape, not a one-off).

**Field + constructor** (`formation_service.rs:36-63`):
```rust
pub struct FormationExecutionService {
    /// Paladin execution port
    paladin_port: Arc<dyn PaladinPort>,
    /// Optional Herald for formatting Battalion results
    herald: Option<Arc<dyn Herald>>,
}

impl FormationExecutionService {
    pub fn new(paladin_port: Arc<dyn PaladinPort>) -> Self {
        info!("Creating FormationExecutionService");
        Self {
            paladin_port,
            herald: None,
        }
    }
```

**Setter** (`formation_service.rs:66-82`):
```rust
/// Set the Herald for formatting results
///
/// This allows runtime override of the default Herald. If set, this Herald
/// will be used to format Battalion results.
pub fn with_herald(mut self, herald: Arc<dyn Herald>) -> Self {
    self.herald = Some(herald);
    self
}
```

**Format wrapper** (`formation_service.rs:100-121`):
```rust
pub fn format_result(
    &self,
    result: &BattalionResult,
) -> Result<Option<String>, BattalionError> {
    match &self.herald {
        Some(herald) => {
            // Herald now uses actual BattalionResult directly - no conversion needed!
            herald
                .format_battalion_result(result)
                .map(Some)
                .map_err(|e| {
                    BattalionError::FormationError(format!("Herald formatting error: {}", e))
                })
        }
        None => Ok(None),
    }
}
```
Replicate verbatim into `campaign_service.rs`, `chain_of_command_service.rs`, and `commander.rs`, substituting the service-specific error variant (e.g. `BattalionError::CampaignError`, `BattalionError::InvalidGraph`/whatever `chain_of_command_service.rs` and `commander.rs` already use elsewhere in their own error construction — check each file's existing `BattalionError::*` usage for the matching variant name rather than reusing `FormationError`). Import needed: `use paladin_core::platform::container::herald::Herald;` (`formation_service.rs:18`).

`Herald::format_battalion_result` (`crates/paladin-core/src/platform/container/herald.rs:85`) is pattern-agnostic across all Battalion kinds, so no change needed to `herald.rs` itself or the three concrete Heralds (`JsonHerald`, `MarkdownHerald`, `TableHerald`).

---

### WARN-01 composite test (D-15)

**Analog:** `tests/integration/battalion_herald_end_to_end_test.rs` — Phase 2's GAP-03 proof, driving a real `FormationExecutionService` over mock Paladins and formatting the resulting `BattalionResult` through all three Heralds.

**Structure to mirror** (lines 1-30):
```rust
//! End-to-end proof for GAP-03 / Epic 8 task 7.13.
//!
//! Drives a real `FormationExecutionService` over mock Paladins and formats the resulting
//! `BattalionResult` through all three Heralds (`JsonHerald`, `MarkdownHerald`, `TableHerald`)...

use async_trait::async_trait;
use paladin::application::services::battalion::formation_service::FormationExecutionService;
use paladin::core::platform::container::herald::Herald;
use paladin::infrastructure::adapters::herald::{JsonHerald, MarkdownHerald, TableHerald};
use paladin_ports::output::llm_port::{...};
use paladin_ports::output::paladin_port::{PaladinPort, PaladinResult, PaladinStream, StopReason};
```
Key discipline stated in its own header comment: "a hand-built `BattalionResult` literal would close the criterion on paper without proving the producer side. This file contains no such literal; every result comes out of `FormationExecutionService::execute`." D-15's new test must apply the same discipline to `ChainOfCommandExecutionService::execute`, driving a real execution and formatting the resulting `BattalionResult` through at least one Herald (JSON recommended, matching the existing file's primary case) — register in `tests/integration/mod.rs` alongside the existing `pub mod battalion_herald_end_to_end_test;`-style entries (see `mod.rs:14` `pub mod battalion_herald_end_to_end_test;`).

Note: `tests/integration/mod.rs` already lists `pub mod battalion_chain_of_command_integration_test;` (line 13) — check that file first; it may already contain reusable ChainOfCommand construction/mock-Paladin fixtures to import rather than re-deriving them.

---

### `tests/integration/llm_live_api_tests.rs` / `tests/integration/mod.rs` (D-19, doc-only)

**Analog:** the files themselves — no external analog needed, this is a doc-comment correction in place.

**Current (incorrect) doc comment** (`llm_live_api_tests.rs:59-61`):
```rust
/// Skip test if API key is not present or empty, otherwise return the key
///
/// This will panic with a clear message if the API key is missing or empty,
```
Line 61 already says "This will panic" — but the summary line 59 says "Skip test". D-19 corrects the opening summary line to describe the panic behavior accurately (do not touch `require_api_key`'s body/behavior).

**Double-gate to document** in `tests/integration/mod.rs` module header (near lines 33-35):
```rust
#[cfg(feature = "live-api-tests")]
pub mod llm_live_api_tests;
```
plus the 13 `#[ignore]` attributes inside `llm_live_api_tests.rs` (confirmed present at lines 122, 159, 215, 239, 280, and 8 more) — document both gates together as "the actual skip mechanism" per D-19, in the module header comment.

---

### Vision surfaces rustdoc (D-18, doc-only)

**Analog:** the trait definitions themselves — `crates/paladin-ports/src/output/vision_port.rs:47` (`pub trait VisionPort`) and `crates/paladin-ports/src/output/vision_llm_port.rs:30-39` (`VisionCapableLlm`, already has a substantial doc block: "Trait for LLM providers that support vision/image inputs... Implementers must handle image format conversion, validation, and proper API integration for their specific provider.").

`VisionPort` is the recommended application-code entry point (reached via `execute_with_vision`, `src/application/services/paladin/paladin_execution_service.rs:517`); `VisionCapableLlm` is the adapter-author surface (reached via `PaladinBuilder::enable_vision`, `paladin_builder.rs:517`). D-18 adds rustdoc to `EncryptionService` (`src/infrastructure/security/encryption.rs:200,217,68,131`) stating the framework never invokes it on the vision path and when a consumer holding image bytes should — follow the existing doc-comment density/style already on `VisionCapableLlm` (multi-paragraph `///` blocks with a `# Requirements` section) as the house style to match.

---

## Shared Patterns

### Herald optional-formatting pattern
**Source:** `crates/paladin-battalion/src/formation_service.rs:36-121`, confirmed identical in `phalanx_service.rs:107`
**Apply to:** `campaign_service.rs`, `chain_of_command_service.rs`, `commander.rs` (WARN-01, D-14)
Field + `with_herald()` + `format_result()` triad, verbatim except for the `BattalionError` variant used in the formatting-failure arm.

### Config-sourced model, never a literal
**Source:** `src/application/services/paladin/planning_service.rs:131,312,552`, `prompt_generation_service.rs:149` (Epic 21 precedent)
**Apply to:** `crates/paladin-battalion/src/grove_service.rs:537` (CLOSE-01, D-01/D-02)
`model: model.to_string()` (or, for Grove, `model: routing_model.to_string()` sourced from `GroveConfig.routing_model`) — never `model: "gpt-4".to_string()`. Missing-config guard uses the same `.ok_or_else(|| BattalionError::RoutingError(...))?` idiom as the existing `llm_port` guard immediately above at `grove_service.rs:487-491`.

### Optional serde section on a config struct
**Source:** `crates/paladin-core/src/platform/container/battalion/grove.rs:213-217` (`fallback_tree`), `src/application/cli/config/paladin_config.rs:66-71` (`garrison`)
**Apply to:** `GroveConfig.routing_model` (CLOSE-01), `PaladinYamlConfig.autonomous` (CLOSE-02)
`#[serde(skip_serializing_if = "Option::is_none")] pub <field>: Option<T>,` with a preceding `///` doc block explaining when the field is required and what happens if absent.

### Additive CLI flag override on top of YAML config
**Source:** `src/application/cli/commands/agent.rs` — existing vision/stop-word conditional builder chain (`:167-332`)
**Apply to:** the four autonomous flags in `handle_agent_run` (D-05/D-07)
`if args.<flag> { builder = builder.<setter>(true); }` — read YAML baseline into the builder first, then layer flags on top; never negate below.

### Doc-only correction / rustdoc-only change, no behavior touch
**Source:** ADR files `.planning/decisions/0011-*.md`, `0012-*.md`; house doc style on `VisionCapableLlm` (`vision_llm_port.rs:30-39`)
**Apply to:** `llm_live_api_tests.rs:61` comment fix, `EncryptionService` rustdoc (D-16/D-18/D-19)
Amend at source with dated provenance where the doc is an ADR; multi-paragraph `///` blocks matching existing density where the doc is a trait/struct.

## No Analog Found

None — all twelve targets have a same-crate or same-file analog above. The three CI jobs (Epic 24 cluster 8.0) are explicitly deferred to Phase 15 per D-09/D-11 and are **not** files this phase creates; `benchmark:` job at `.github/workflows/ci.yml:783-813` is included above only as read-only reference context for the Phase 15 planner, per D-10's bidirectional-record requirement — no `.github/` file is touched in Phase 6 (D-11 hard constraint).

## Metadata

**Analog search scope:** `crates/paladin-core/src/platform/container/battalion/`, `crates/paladin-battalion/src/`, `crates/paladin-battalion/benches/`, `crates/paladin-ports/src/output/`, `src/application/cli/`, `src/application/services/paladin/`, `src/infrastructure/security/`, `tests/integration/`, `.github/workflows/`, `.planning/decisions/`
**Files scanned:** ~20 read directly, plus grep sweeps across `crates/` and `src/`
**Pattern extraction date:** 2026-08-05
