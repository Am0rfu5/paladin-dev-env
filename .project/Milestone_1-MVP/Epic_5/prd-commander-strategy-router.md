# Product Requirements Document: Commander Strategy Router

**Epic:** Epic 5 - Commander Strategy Router  
**Priority:** Medium  
**Estimated Effort:** 2 weeks  
**Dependencies:** Epic 4 (Battalion Orchestration)  
**Target Audience:** Developers using the Paladin framework  
**Document Version:** 2.0  
**Last Updated:** January 25, 2026

---

## 1. Introduction/Overview

### Problem Statement

Currently, developers using the Paladin framework must manually select and configure the appropriate Battalion orchestration pattern (Formation, Phalanx, Campaign, or Chain of Command) for their multi-agent workflows. This requires:

- Deep understanding of each Battalion type's characteristics and use cases
- Repetitive boilerplate code for instantiating and configuring different Battalion services
- Manual error handling and result normalization across different execution patterns
- Difficulty switching between orchestration strategies during development/experimentation

### Solution

The **Commander** provides a unified, high-level interface for Battalion orchestration that:

- Offers a single, consistent API for executing any Battalion type
- Provides rule-based automatic strategy selection when developers are uncertain which pattern to use
- Normalizes results and error handling across all Battalion types
- Supports comprehensive BattalionConfig passthrough for full control
- Delivers detailed telemetry about strategy selection and execution performance

### Goal

Enable developers to orchestrate multi-Paladin workflows through a simplified, unified interface that reduces complexity, accelerates development, and provides intelligent defaults while maintaining full control when needed.

---

## 2. Goals

1. **Simplify Multi-Agent Orchestration**: Reduce code complexity by providing a single interface for all Battalion types
2. **Enable Intelligent Defaults**: Provide rule-based automatic strategy selection for common use cases (important but manual selection acceptable initially)
3. **Maintain Flexibility**: Allow explicit strategy selection when developers know their requirements
4. **Comprehensive Configuration**: Support full BattalionConfig passthrough for all execution parameters
5. **Configurable Error Handling**: Let developers define failure handling strategy (fail-fast, retry, continue)
6. **Detailed Observability**: Provide telemetry about strategy selection reasoning and execution metrics
7. **Future-Proof Architecture**: Design for extensibility without initial implementation (strategy composition, job integration)

---

## 3. User Stories

### US-1: Simple Strategy Selection
**As a** developer  
**I want to** create a Commander with an explicit strategy type  
**So that** I can execute my Paladins using a known orchestration pattern without managing service instances directly

**Acceptance Criteria:**
- Can instantiate Commander with BattalionStrategy::Formation
- Can instantiate Commander with BattalionStrategy::Phalanx
- Can instantiate Commander with BattalionStrategy::Campaign
- Can instantiate Commander with BattalionStrategy::ChainOfCommand
- Commander handles service instantiation internally

### US-2: Automatic Strategy Selection (High Priority)
**As a** developer  
**I want to** use BattalionStrategy::Auto  
**So that** the framework selects the optimal orchestration pattern based on simple heuristics when I'm unsure

**Acceptance Criteria:**
- Auto mode analyzes input task and Paladin configuration
- Uses rule-based heuristics (e.g., Paladin count, keyword detection)
- Selection reasoning is logged for transparency
- Falls back to Formation if heuristics are inconclusive
- Manual strategy selection is acceptable alternative if Auto is uncertain

### US-3: Unified Execution Interface
**As a** developer  
**I want to** call a single `execute()` method  
**So that** my code doesn't change when I switch between Battalion strategies

**Acceptance Criteria:**
- Single async `execute(input: &str)` method for all strategies
- Returns consistent `BattalionResult` type
- Error type is unified (`BattalionError`)
- Execution behavior matches underlying Battalion type semantics

### US-4: Comprehensive Configuration
**As a** developer  
**I want to** pass full BattalionConfig to Commander  
**So that** I can control all execution parameters (timeout, retry, error handling, checkpointing, metadata)

**Acceptance Criteria:**
- Commander accepts complete BattalionConfig
- Config is passed through to underlying Battalion services
- Default config is provided if none specified
- Config validation happens at Commander construction time

### US-5: Configurable Error Handling
**As a** developer  
**I want to** define how Commander handles execution failures  
**So that** I can choose between fail-fast, retry, or custom recovery strategies

**Acceptance Criteria:**
- Error handling strategy is part of BattalionConfig
- Supports FailFast, ContinueOnError, RetryThenContinue
- Strategy is respected by Commander wrapper logic
- Error details preserved in BattalionResult

### US-6: Detailed Execution Telemetry
**As a** developer  
**I want to** receive detailed telemetry about strategy selection and paladin performance  
**So that** I can understand performance characteristics and optimize my workflows

**Acceptance Criteria:**
- Logs include selected strategy and selection reasoning
- BattalionResult contains per-Paladin execution times
- Total execution time tracked
- Strategy selection time tracked separately
- Telemetry includes Paladin success/failure counts
- Metadata can be exported to files if configured

---

## 4. Functional Requirements

### FR-1: Commander Construction
**Must:** Commander must be constructible with the following parameters:
- `strategy: BattalionStrategy` - Explicit or Auto
- `paladins: Vec<Paladin>` - Ordered list of Paladins to orchestrate
- `config: BattalionConfig` (optional) - Full configuration passthrough

**Must:** Validate at construction time:
- At least one Paladin is provided
- All Paladins are valid (not in failed state)
- Config is internally consistent
- Return detailed error if validation fails

### FR-2: Strategy Types
**Must:** Support the following `BattalionStrategy` enum variants:
- `Formation` - Sequential execution with output chaining
- `Phalanx` - Concurrent parallel execution
- `Campaign` - Graph/DAG-based orchestration
- `ChainOfCommand` - Hierarchical delegation pattern
- `Auto` - Rule-based automatic selection

### FR-3: Automatic Strategy Selection (Auto Mode)
**Must:** When `BattalionStrategy::Auto` is selected, apply these rules in order:

1. **Single Paladin**: Select `Formation` (trivial case)
2. **Keyword Detection**: Scan input for strategy hints:
   - "sequential", "pipeline", "chain", "step by step" → `Formation`
   - "parallel", "concurrent", "all at once", "simultaneously" → `Phalanx`
   - "workflow", "graph", "conditional", "if-then" → `Campaign`
   - "delegate", "hierarchy", "specialist", "expert" → `ChainOfCommand`
3. **Paladin Count Heuristics**:
   - 2-3 Paladins → `Formation` (default for small groups)
   - 4+ Paladins with similar roles → `Phalanx`
   - 4+ Paladins with specialized roles → `ChainOfCommand`
4. **Default Fallback**: If no rules match, select `Formation`

**Must:** Log the selection reasoning including:
- Rule that triggered selection
- Detected keywords (if applicable)
- Paladin count and role analysis

**Note:** Auto mode is high priority but manual selection is acceptable if Auto is uncertain. Focus on 80/20 rule - cover common cases clearly.

### FR-4: Execute Method
**Must:** Provide async execution method:
```rust
pub async fn execute(&self, input: &str) -> Result<BattalionResult, BattalionError>
```

**Must:** Implementation steps:
1. Resolve strategy (if Auto, run selection logic)
2. Build appropriate Battalion structure (Formation/Phalanx/Campaign/ChainOfCommand)
3. Delegate to corresponding service (FormationExecutionService, etc.)
4. Wrap result in telemetry metadata
5. Return normalized BattalionResult

### FR-5: Result Normalization
**Must:** Return consistent `BattalionResult` containing:
- `battalion_id: Uuid` - Unique execution identifier
- `strategy_used: BattalionStrategy` - Actual strategy executed (resolved from Auto)
- `paladin_results: Vec<PaladinResult>` - Individual Paladin outcomes
- `final_output: String` - Aggregated or final result
- `execution_time_ms: u64` - Total execution duration
- `status: BattalionStatus` - Overall success/partial/failed status
- `metadata: BattalionMetadata` - Telemetry and execution details

**Must:** Include in metadata (detailed telemetry):
- `strategy_selection_reasoning: Option<String>` - Why this strategy was chosen (for Auto)
- `strategy_selection_time_ms: u64` - Time spent in selection logic
- `per_paladin_times: Vec<u64>` - Individual execution times
- `paladin_success_count: usize` - Number of successful Paladins
- `paladin_failure_count: usize` - Number of failed Paladins
- `timestamp: DateTime<Utc>` - Execution start time

### FR-6: Configurable Error Handling Strategy
**Must:** Support error strategies via `BattalionConfig::error_strategy`:
- `ErrorStrategy::FailFast` - Stop on first Paladin failure, return error immediately
- `ErrorStrategy::ContinueOnError` - Continue executing remaining Paladins, collect all errors
- `ErrorStrategy::RetryThenContinue` - Retry failed Paladin up to N times, then continue

**Must:** Respect retry configuration from `BattalionConfig::retry_attempts`

**Must:** Preserve all error details in `BattalionResult::errors: Vec<PaladinError>`

**Must:** Allow developers to configure strategy per Commander instance via BattalionConfig

### FR-7: Configuration Passthrough (Comprehensive)
**Must:** Accept and forward complete `BattalionConfig` including:
- `name: String` - Battalion name for logging
- `timeout_seconds: u64` - Overall execution timeout
- `retry_attempts: u32` - Per-Paladin retry count
- `error_strategy: ErrorStrategy` - Failure handling mode
- `enable_checkpointing: bool` - State persistence flag
- `metadata_output_dir: Option<PathBuf>` - Telemetry output location

**Must:** Apply config consistently across all underlying Battalion services

**Must:** Provide sensible defaults for all config fields if not specified

### FR-8: Service Composition
**Must:** Internally compose these services:
- `Arc<FormationExecutionService>` - For Formation execution
- `Arc<PhalanxExecutionService>` - For Phalanx execution
- `Arc<CampaignExecutionService>` - For Campaign execution
- `Arc<ChainOfCommandService>` - For Chain of Command execution

**Should:** Lazy-initialize services (don't instantiate unused services)

### FR-9: Logging and Telemetry (Detailed)
**Must:** Log the following events with structured fields:
- Commander construction with strategy and Paladin count
- Strategy selection (Auto mode) with reasoning
- Execution start with resolved strategy
- Each Paladin execution (delegated to services)
- Execution completion with summary statistics

**Must:** Use structured logging with contextual fields:
- `commander_id`
- `strategy`
- `paladin_count`
- `execution_time_ms`
- `strategy_selection_reasoning` (for Auto)

**Must:** Export metadata to configured output directory if `metadata_output_dir` is set

### FR-10: Validation
**Must:** Validate at construction:
- Paladins vector is not empty
- All Paladins have valid configurations
- BattalionConfig is internally consistent

**Must:** Validate before execution:
- Input string is not empty (or allow empty based on Paladin requirements)
- Commander is not in error state from construction

---

## 5. Non-Goals (Out of Scope)

The following are explicitly **not** included in the initial implementation:

### NG-1: LLM-Powered Strategy Selection
**Not included:** Using an LLM to analyze the task and intelligently select strategy  
**Reason:** Adds complexity and latency; rule-based heuristics provide 80% of value  
**Future:** Can be added as enhanced Auto mode in future iteration

### NG-2: Strategy Composition/Chaining
**Not included:** Ability to chain multiple strategies (e.g., Phalanx → Formation)  
**Reason:** Unclear use cases; adds significant complexity  
**Future:** Future enhancement after usage patterns are established

### NG-3: Job Orchestration Integration
**Not included:** Direct integration with existing job/task/scheduler system  
**Reason:** Commander should be standalone; integration can be external wrapper  
**Future:** Future consideration - build standalone first, integrate later based on needs

### NG-4: Dynamic Strategy Switching
**Not included:** Changing strategy mid-execution based on runtime conditions  
**Reason:** Complex state management; unclear semantics  
**Future:** Possible enhancement after stability proven

### NG-5: Custom Strategy Plugins
**Not included:** User-defined strategy types beyond the four core patterns  
**Reason:** Core patterns should cover 95% of use cases  
**Future:** Extension point design if demand emerges

### NG-6: Visual Strategy Builder
**Not included:** GUI or DSL for defining Commander configurations  
**Reason:** Code-first approach is sufficient for target users (developers)  
**Future:** Potential Armory CLI enhancement

---

## 6. Design Considerations

### 6.1 Module Structure
```
src/application/use_cases/battalion/
├── mod.rs                          # Re-exports
├── formation_service.rs            # (Epic 4)
├── phalanx_service.rs              # (Epic 4)
├── campaign_service.rs             # (Epic 4)
├── chain_of_command_service.rs     # (Epic 4)
└── commander.rs                    # NEW - This Epic
```

### 6.2 Key Types
**BattalionStrategy Enum:**
```rust
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum BattalionStrategy {
    Formation,
    Phalanx,
    Campaign,
    ChainOfCommand,
    Auto,
}
```

**Commander Struct:**
```rust
pub struct Commander {
    id: Uuid,
    strategy: BattalionStrategy,
    paladins: Vec<Paladin>,
    config: BattalionConfig,
    // Service dependencies (Arc for sharing)
    formation_service: Arc<FormationExecutionService>,
    phalanx_service: Arc<PhalanxExecutionService>,
    campaign_service: Arc<CampaignExecutionService>,
    chain_service: Arc<ChainOfCommandService>,
}
```

### 6.3 Builder Pattern
Use fluent builder for construction:
```rust
Commander::builder()
    .strategy(BattalionStrategy::Auto)
    .paladins(vec![paladin1, paladin2, paladin3])
    .config(battalion_config)
    .build()?
```

### 6.4 Strategy Selection Algorithm
Implement as private method with clear test coverage:
```rust
fn analyze_and_select(&self, input: &str) -> (BattalionStrategy, String)
```
Returns tuple of (selected_strategy, reasoning)

### 6.5 Error Handling
Use existing `BattalionError` enum, add variants if needed:
```rust
#[derive(Debug, thiserror::Error)]
pub enum BattalionError {
    #[error("Commander validation error: {0}")]
    CommanderValidation(String),

    #[error("Strategy selection failed: {0}")]
    StrategySelection(String),

    // Existing variants...
}
```

---

## 7. Technical Considerations

### 7.1 Dependencies
**Required from Epic 4:**
- All four Battalion execution services must be complete
- BattalionConfig, BattalionResult, BattalionError types defined
- BattalionPort trait (Commander implements this)

### 7.2 Testing Strategy
**Unit Tests (≥80% coverage):**
- Strategy selection logic (all heuristic rules)
- Configuration validation
- Error handling for each ErrorStrategy
- Result normalization
- Builder pattern validation

**Integration Tests:**
- Execute Formation through Commander
- Execute Phalanx through Commander
- Execute Campaign through Commander
- Execute ChainOfCommand through Commander
- Auto mode selects Formation correctly
- Auto mode selects Phalanx correctly
- Auto mode selects Campaign correctly
- Auto mode selects ChainOfCommand correctly
- All three error handling strategies work correctly
- Configuration passthrough is accurate
- Telemetry data is complete and accurate

### 7.3 Performance Considerations
- **Strategy selection overhead:** Must be <10ms for Auto mode
- **Memory overhead:** Commander should not duplicate Paladin data
- **Service instantiation:** Consider lazy-loading services to avoid unnecessary initialization

### 7.4 Logging Standards
Use structured logging (via tracing or log crate):
```rust
info!(
    commander_id = %self.id,
    strategy = ?resolved_strategy,
    paladin_count = self.paladins.len(),
    reasoning = %selection_reasoning,
    "Commander executing with resolved strategy"
);
```

### 7.5 Async Considerations
- All execution must be async (delegating to underlying services)
- Proper error propagation with `?` operator
- Use `tokio::time::timeout` for overall timeout enforcement from BattalionConfig

### 7.6 Future Extension Points
Design for future enhancements without implementing:
- **Strategy composition:** Consider builder pattern for chaining (future)
- **Job integration:** Commander should be designed to potentially implement a JobExecutable trait (future)
- **Custom heuristics:** Make selection rules configurable via CommanderConfig (future)

---

## 8. Success Metrics

### 8.1 Functional Metrics
- **Test Coverage:** ≥80% unit test coverage, ≥70% integration test coverage
- **API Completeness:** All four strategies executable through Commander
- **Error Handling:** All three error strategies functional and tested
- **Validation:** 100% of invalid configs rejected at construction
- **Auto Mode Accuracy:** Auto selection works for defined test scenarios

### 8.2 Performance Metrics
- **Strategy Selection Time:** <10ms for Auto mode (p95)
- **Execution Overhead:** Commander adds <5% overhead vs direct service calls
- **Memory Overhead:** Commander instance <1KB additional per Paladin

### 8.3 Usability Metrics
- **Code Reduction:** Example code should be 30-50% shorter using Commander vs direct services
- **Auto Accuracy:** Auto mode selects reasonable strategy ≥80% of time in test scenarios
- **Documentation:** All public APIs have rustdoc with examples

### 8.4 Quality Metrics
- **Clippy Clean:** Zero clippy warnings
- **Doc Tests:** All examples in rustdoc are executable and pass
- **Error Messages:** All error messages provide actionable guidance

---

## 9. Open Questions

### Q1: Heuristic Configuration
**Question:** Should heuristic rules be configurable in initial release or hardcoded?  
**Context:** Making them configurable adds complexity but increases flexibility  
**Recommendation:** Start hardcoded, gather feedback, make configurable in iteration  
**Owner:** Lead Developer  
**Resolution:** Before implementation start

### Q2: Strategy Selection Caching
**Question:** Should Commander cache strategy selection results for identical inputs?  
**Context:** Could improve performance but adds statefulness  
**Recommendation:** No caching in v1; measure performance first  
**Owner:** Tech Lead  
**Resolution:** Week 1

### Q3: Partial Execution Results
**Question:** For ContinueOnError strategy, should partial results be usable?  
**Context:** Some Paladins succeeded, some failed - is output meaningful?  
**Recommendation:** Yes, return partial results with clear error indicators in metadata  
**Owner:** Product  
**Resolution:** Week 1

### Q4: Commander Reusability
**Question:** Should Commander instance be reusable for multiple execute() calls?  
**Context:** Affects state management and service lifecycle  
**Recommendation:** Yes, make stateless and reusable; services are Arc-wrapped  
**Owner:** Architect  
**Resolution:** Before implementation start

### Q5: Keyword Detection Sensitivity
**Question:** How strict should keyword matching be? Case-sensitive? Stemming?  
**Context:** Affects Auto mode reliability  
**Recommendation:** Case-insensitive, exact word match initially; can enhance later  
**Owner:** Implementation Lead  
**Resolution:** Week 1

---

## 10. Related Documents

- **Epic Definition:** [epic5.md](./epic5.md)
- **Project Plan:** [Paladin Project Completion Plan.md](../Paladin%20Project%20Completion%20Plan.md)
- **Epic 4 (Dependency):** [Epic_4/](../Epic_4/)
- **Architecture Guide:** [docs/Design/Design_and_Architecture.md](../../../docs/Design/Design_and_Architecture.md)
- **Hexagonal Architecture Notes:** [notes/hexagonal-arch.md](../../../notes/hexagonal-arch.md)

---

## 11. Appendix: Example Usage

### Example 1: Explicit Strategy
```rust
use paladin::application::use_cases::battalion::Commander;
use paladin::core::platform::container::battalion::BattalionStrategy;

// Create paladins (from Epic 1)
let researcher = PaladinBuilder::new(llm_port.clone())
    .system_prompt("You are a research specialist")
    .build()?;

let analyst = PaladinBuilder::new(llm_port.clone())
    .system_prompt("You are a data analyst")
    .build()?;

// Create Commander with explicit Formation strategy
let commander = Commander::builder()
    .strategy(BattalionStrategy::Formation)
    .paladins(vec![researcher, analyst])
    .build()?;

// Execute
let result = commander.execute("Research AI trends and analyze the data").await?;
println!("Result: {}", result.final_output);
```

### Example 2: Auto Mode with Telemetry
```rust
// Same paladins...

// Let Commander choose strategy automatically
let commander = Commander::builder()
    .strategy(BattalionStrategy::Auto)
    .paladins(vec![researcher, analyst])
    .build()?;

let result = commander.execute(
    "Please research AI trends and then analyze the data sequentially"
).await?;

// Check what strategy was selected
println!("Used strategy: {:?}", result.strategy_used);
println!("Selection reasoning: {}",
    result.metadata.strategy_selection_reasoning.unwrap());
println!("Execution time: {}ms", result.execution_time_ms);
println!("Per-paladin times: {:?}", result.metadata.per_paladin_times);
```

### Example 3: Full Configuration with Error Handling
```rust
use paladin::core::platform::container::battalion::{BattalionConfig, ErrorStrategy};

let config = BattalionConfig::builder()
    .name("AI Analysis Battalion")
    .timeout_seconds(300)
    .retry_attempts(2)
    .error_strategy(ErrorStrategy::RetryThenContinue)
    .enable_checkpointing(true)
    .metadata_output_dir(Some(PathBuf::from("./telemetry")))
    .build();

let commander = Commander::builder()
    .strategy(BattalionStrategy::Phalanx)
    .paladins(vec![paladin1, paladin2, paladin3, paladin4])
    .config(config)
    .build()?;

let result = commander.execute("Analyze this from multiple perspectives").await?;

// Access detailed telemetry
println!("Execution time: {}ms", result.execution_time_ms);
println!("Successes: {}, Failures: {}",
    result.metadata.paladin_success_count,
    result.metadata.paladin_failure_count);
```

---

**End of Document**
