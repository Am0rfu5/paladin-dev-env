# Product Requirements Document: Autonomous Agent Completion

## Document Information

- **Feature:** Autonomous Agent Completion (Epic 21)
- **Version:** 1.0
- **Created:** February 11, 2026
- **Status:** Ready for Implementation
- **Epic:** Epic 21 - Milestone 3
- **Priority:** Critical
- **Dependencies:** Epic 19 (Herald & Domain Type Consolidation)
- **Estimated Duration:** 2 weeks
- **Target Branch:** `feature/epic-21-autonomous-agent-completion`

---

## 1. Introduction/Overview

Epic 14 established the foundational autonomous agent capabilities in Paladin (planning, prompt generation, dynamic temperature, and handoffs) but deferred 23 task items that required cross-service integration, domain type modifications, and full execution orchestration. Epic 21 completes this work by implementing the deferred integration points, enhancing domain types with autonomous metadata, and orchestrating all autonomous features into a cohesive execution flow.

**Problem:** The autonomous agent features exist as isolated components but cannot function end-to-end. Handoffs don't actually execute specialist agents, planning and prompt generation use hardcoded models, PaladinResult lacks autonomous metadata, and there's no orchestration to coordinate all features together.

**Solution:** Complete the autonomous agent pipeline by implementing handoff execution, auto-registration, metadata tracking, service orchestration, and configuration-driven model selection. The solution follows a **bottom-up implementation approach**, starting with foundational changes (configurable models, domain types) and building up to complex integrations (handoff execution, orchestration).

---

## 2. Goals

1. **Enable End-to-End Handoff Execution**: Specialist agents can be invoked via handoffs with full result flow, chain tracking, and cycle detection
2. **Zero Breaking Changes**: All enhancements are optional and backward compatible with existing code
3. **Configuration-Driven Autonomy**: Remove hardcoded model references; services respect Paladin configuration
4. **Full Observability**: PaladinResult captures planning and handoff metadata for execution tracing
5. **Layered Feature Integration**: Core execution always runs; autonomous features enhance it when enabled
6. **Comprehensive Test Coverage**: Unit, integration, and E2E tests validate all workflows with mocked LLM interactions

---

## 3. User Stories

### US-21.1: Handoff Execution Integration

**As a** developer  
**I want** handoffs to actually execute specialist agents  
**So that** multi-agent delegation works end-to-end with full traceability

**Acceptance Criteria:**
- `HandoffService::execute_handoff()` delegates tasks to specialist Paladins via `PaladinExecutionService`
- Results flow back: specialist executes → result returned → original agent continues with context
- Handoff chain tracking maintained across all tool invocations
- Circular handoff detection prevents infinite loops in integration context
- Max depth exceeded validation works end-to-end with configurable limits
- Handoff tool calls visible in execution trace for debugging
- **Error Handling**: Configurable retry with exponential backoff (similar to vision retry logic)
  - Retry on transient failures (network errors, timeouts)
  - Fail immediately on permanent errors (invalid specialist, circular reference)
  - Configurable: `max_handoff_retries`, `initial_backoff_ms`, `backoff_multiplier`

**Implementation Priority:** 5 (Last - requires all other components)

---

### US-21.2: Handoff Tool Auto-Registration

**As a** developer  
**I want** handoff tools auto-registered when handoffs are configured  
**So that** the LLM can invoke handoffs via tool calling without manual setup

**Acceptance Criteria:**
- `PaladinBuilder::build()` auto-registers handoff tool when `with_handoffs()` was called
- Handoff tool appears in arsenal with correct JSON schema for tool calling
- Tool schema includes: specialist names, descriptions, parameter requirements
- Unit tests verify auto-registration behavior
- Auto-registration is idempotent (safe to call build multiple times)
- Tool is properly removed/updated if handoffs are reconfigured

**Implementation Priority:** 3 (Mid - supports handoff execution)

---

### US-21.3: PaladinResult Metadata Enhancement

**As a** developer  
**I want** `PaladinResult` to include planning and handoff metadata  
**So that** execution traces contain full autonomous context for debugging and analysis

**Acceptance Criteria:**
- `PaladinResult` includes `plan: Option<TaskPlan>` field (default: `None`)
- `PaladinResult` includes `handoff_history: Vec<HandoffRecord>` field (default: `Vec::new()`)
- **Zero Breaking Changes**: All new fields are `Option` or `Vec` with defaults
- Existing code works without modifications (backward compatible)
- Serialization/deserialization tests pass (JSON, MessagePack, etc.)
- All existing tests continue to pass without changes
- New tests verify metadata is captured when autonomous features are enabled
- Documentation updated with metadata field descriptions and usage examples

**Data Structures:**
```rust
pub struct TaskPlan {
    pub goal: String,
    pub subtasks: Vec<Subtask>,
    pub created_at: DateTime<Utc>,
}

pub struct HandoffRecord {
    pub specialist_name: String,
    pub task_description: String,
    pub timestamp: DateTime<Utc>,
    pub result: Option<String>,
    pub depth: usize,
}
```

**Implementation Priority:** 2 (Early - foundation for observability)

---

### US-21.4: Autonomous Execution Orchestration

**As a** developer  
**I want** all autonomous features orchestrated in the execution service  
**So that** planning, prompts, temperature, and handoffs work together seamlessly

**Acceptance Criteria:**
- `PaladinExecutionService` coordinates autonomous features in layered approach:
  - **Layer 0 (Core)**: Basic LLM execution (always runs)
  - **Layer 1 (Enhancement)**: Planning → Prompt Generation (if enabled)
  - **Layer 2 (Adaptation)**: Dynamic Temperature (if enabled)
  - **Layer 3 (Delegation)**: Handoff Handling (if enabled)
- Each layer is independently enabled/disabled via configuration
- Core execution never fails if optional features are disabled
- Feature interaction edge cases handled gracefully:
  - Planning with handoffs: Subtasks can trigger handoffs
  - Prompt generation with dynamic temperature: Temperature adjusted per loop
  - All features together: Full autonomous workflow
- Integration tests cover each layer independently
- Integration tests cover feature combinations (planning + handoffs, etc.)
- End-to-end test with all features enabled validates full workflow
- Performance impact measured (layered approach should minimize overhead)

**Orchestration Flow:**
```
Input → [Planning?] → [Prompt Gen?] → Core Execute → [Dynamic Temp?] → [Handoff?] → Output
         └─Optional─┘   └─Optional─┘      Always      └─Optional──┘   └─Optional┘
```

**Implementation Priority:** 4 (Late - integrates all components)

---

### US-21.5: Configurable Model in Autonomous Services

**As a** developer  
**I want** the LLM model used by planning and prompt generation to be configurable  
**So that** services respect the Paladin's configured model instead of hardcoded values

**Acceptance Criteria:**
- `PlanningService` reads model from Paladin config instead of hardcoded `"gpt-4"`
- `PromptGenerationService` reads model from Paladin config instead of hardcoded `"gpt-4"`
- Subtask expected output generated by LLM instead of hardcoded placeholder string
- Unit tests verify model is read from config and passed correctly to LLM port
- Configuration validation ensures model is vision-capable if vision features are used
- Fallback to default model if config is invalid (with warning log)
- Documentation updated with model configuration examples

**Affected Source Files:**
- `src/application/use_cases/paladin/planning_service.rs` — lines 128, 305, 426, 538
- `src/application/use_cases/paladin/prompt_generation_service.rs` — line 146

**Implementation Priority:** 1 (First - foundational change affecting all services)

---

## 4. Functional Requirements

### FR-1: Configurable Model Selection (US-21.5)

**FR-1.1**: `PlanningService` MUST read the `model` field from `Paladin.config`  
**FR-1.2**: `PromptGenerationService` MUST read the `model` field from `Paladin.config`  
**FR-1.3**: Services MUST pass the configured model to `LlmPort` methods  
**FR-1.4**: Services MUST validate model compatibility with required features (e.g., vision)  
**FR-1.5**: Services MUST log warnings and fall back to safe defaults on invalid configuration  
**FR-1.6**: Subtask expected output MUST be generated by LLM, not hardcoded strings  

### FR-2: PaladinResult Metadata (US-21.3)

**FR-2.1**: `PaladinResult` MUST include `plan: Option<TaskPlan>` field  
**FR-2.2**: `PaladinResult` MUST include `handoff_history: Vec<HandoffRecord>` field  
**FR-2.3**: All new fields MUST have zero-impact defaults (`None`, `Vec::new()`)  
**FR-2.4**: Serialization MUST support JSON and MessagePack formats  
**FR-2.5**: Deserialization MUST handle missing fields gracefully (backward compatibility)  
**FR-2.6**: `TaskPlan` MUST include goal, subtasks, and creation timestamp  
**FR-2.7**: `HandoffRecord` MUST include specialist name, task, timestamp, result, and depth  

### FR-3: Handoff Tool Auto-Registration (US-21.2)

**FR-3.1**: `PaladinBuilder::build()` MUST detect if `with_handoffs()` was called  
**FR-3.2**: Builder MUST auto-register handoff tool in arsenal when handoffs are configured  
**FR-3.3**: Handoff tool schema MUST include all configured specialist names  
**FR-3.4**: Handoff tool schema MUST include parameter: `specialist_name` (enum of specialists)  
**FR-3.5**: Handoff tool schema MUST include parameter: `task_description` (string)  
**FR-3.6**: Auto-registration MUST be idempotent (no duplicates)  

### FR-4: Autonomous Execution Orchestration (US-21.4)

**FR-4.1**: `PaladinExecutionService` MUST support layered execution flow  
**FR-4.2**: Core execution (Layer 0) MUST always run regardless of feature flags  
**FR-4.3**: Planning (Layer 1) MUST be skipped if `autonomous_planning` is disabled  
**FR-4.4**: Prompt generation (Layer 1) MUST be skipped if `autonomous_prompts` is disabled  
**FR-4.5**: Dynamic temperature (Layer 2) MUST be skipped if `dynamic_temperature` is disabled  
**FR-4.6**: Handoff handling (Layer 3) MUST be skipped if `handoffs` is not configured  
**FR-4.7**: Layer failures MUST NOT prevent core execution (graceful degradation)  
**FR-4.8**: Orchestration MUST populate metadata fields in `PaladinResult` when features are active  

### FR-5: Handoff Execution Integration (US-21.1)

**FR-5.1**: `HandoffService::execute_handoff()` MUST delegate to `PaladinExecutionService`  
**FR-5.2**: Handoff MUST pass specialist Paladin instance to execution service  
**FR-5.3**: Handoff result MUST flow back to original agent as tool response  
**FR-5.4**: Handoff chain MUST be tracked in `HandoffRecord` with depth counter  
**FR-5.5**: Circular handoff MUST be detected (same specialist at same depth)  
**FR-5.6**: Max depth MUST be enforced (configurable, default: 5)  
**FR-5.7**: Handoff calls MUST be visible in execution trace/logs  
**FR-5.8**: Handoff errors MUST support configurable retry with exponential backoff  
**FR-5.9**: Retry configuration MUST include: `max_handoff_retries`, `initial_backoff_ms`, `backoff_multiplier`  
**FR-5.10**: Transient errors (network, timeout) MUST trigger retry  
**FR-5.11**: Permanent errors (invalid specialist, circular) MUST fail immediately  

### FR-6: Configuration Schema

**FR-6.1**: Paladin configuration MUST support:
```yaml
model: "gpt-4o"  # Used by planning, prompt generation, execution
autonomous_planning: true
autonomous_prompts: true
dynamic_temperature: true
handoffs:
  enabled: true
  max_depth: 5
  retry:
    max_retries: 3
    initial_backoff_ms: 1000
    backoff_multiplier: 2.0
  specialists:
    - name: "CodeExpert"
      description: "Python code analysis and generation"
    - name: "DataAnalyst"
      description: "Statistical analysis and visualization"
```

### FR-7: Error Handling

**FR-7.1**: Services MUST log errors at appropriate levels (ERROR for failures, WARN for degradation)  
**FR-7.2**: Configuration errors MUST fail fast at build time (before execution)  
**FR-7.3**: Execution errors MUST be wrapped in domain-specific error types  
**FR-7.4**: Retry logic MUST respect exponential backoff to avoid API rate limits  
**FR-7.5**: Circuit breaker MUST integrate with handoff retry logic  

---

## 5. Non-Goals (Out of Scope)

**NG-1**: Multi-model handoffs (each specialist uses a different LLM model) - Future enhancement  
**NG-2**: Async/streaming handoff execution - Deferred to performance epic  
**NG-3**: Persistent handoff history storage (database) - Current scope is in-memory only  
**NG-4**: Handoff result caching - Future optimization  
**NG-5**: Visual handoff graph visualization - UI enhancement, not core functionality  
**NG-6**: Breaking changes to existing APIs - Strict backward compatibility required  
**NG-7**: Performance optimization of autonomous features - Focus on correctness first  
**NG-8**: Autonomous feature auto-tuning (ML-based) - Future research  

---

## 6. Technical Considerations

### 6.1 Implementation Order (Bottom-Up Approach)

Based on selection **1C**, implement in this sequence:

1. **Phase 1 (Week 1, Days 1-2)**: US-21.5 - Configurable Models
   - Replace hardcoded `"gpt-4"` references
   - Add model configuration validation
   - Update unit tests

2. **Phase 2 (Week 1, Days 3-4)**: US-21.3 - PaladinResult Enhancement
   - Add optional metadata fields
   - Update serialization tests
   - Verify backward compatibility

3. **Phase 3 (Week 1, Days 5-7)**: US-21.2 - Handoff Tool Auto-Registration
   - Implement builder logic
   - Generate tool schemas
   - Unit test auto-registration

4. **Phase 4 (Week 2, Days 1-4)**: US-21.4 - Autonomous Orchestration
   - Implement layered execution flow
   - Integration tests for feature combinations
   - E2E test with all features

5. **Phase 5 (Week 2, Days 5-7)**: US-21.1 - Handoff Execution
   - Implement delegation to specialists
   - Add retry logic with backoff
   - Full integration tests

### 6.2 Backward Compatibility Strategy

Per selection **2D** (optional fields only):

- **Domain Types**: Use `Option<T>` and `Vec<T>` for all new fields
- **Serialization**: Derive `Default` for all enhanced structs
- **Configuration**: New settings are opt-in with sensible defaults
- **Tests**: Existing tests must pass without modifications
- **Migration**: No migration scripts needed; zero breaking changes

### 6.3 Error Handling Strategy

Per selection **3C** (configurable retry):

- **Retry Configuration**:
  ```rust
  pub struct HandoffRetryConfig {
      pub max_retries: u32,           // Default: 3
      pub initial_backoff_ms: u64,    // Default: 1000
      pub backoff_multiplier: f64,    // Default: 2.0
  }
  ```

- **Error Classification**:
  - **Transient**: Network errors, timeouts, rate limits → **Retry**
  - **Permanent**: Invalid specialist, circular reference, config errors → **Fail immediately**

- **Backoff Formula**: `delay = initial_backoff_ms * (backoff_multiplier ^ attempt)`
  - Attempt 1: 1000ms
  - Attempt 2: 2000ms
  - Attempt 3: 4000ms

### 6.4 Testing Strategy

Per selection **4C** (comprehensive coverage):

**Unit Tests**:
- Mock all external dependencies (LLM port, execution service)
- Test each component in isolation
- Verify configuration handling
- Test error paths and edge cases

**Integration Tests**:
- Use real service implementations with mock LLM responses
- Test cross-service interactions (planning → execution, handoff → specialist)
- Verify metadata population
- Test feature combinations

**End-to-End Tests**:
- Full workflow with all features enabled
- Mock LLM responses for deterministic testing
- Verify execution traces and logs
- Test error recovery flows

**Test Coverage Targets**:
- Unit tests: ≥90% line coverage
- Integration tests: All user stories covered
- E2E tests: At least 3 full workflow scenarios

### 6.5 Feature Orchestration Architecture

Per selection **5C** (layered approach):

```
┌──────────────────────────────────────────────┐
│         PaladinExecutionService              │
│                                              │
│  ┌────────────────────────────────────────┐ │
│  │ Layer 0: Core Execution (Always)       │ │
│  │   - LLM call                           │ │
│  │   - Response parsing                   │ │
│  │   - Basic loop control                 │ │
│  └────────────────────────────────────────┘ │
│                    ↑                         │
│  ┌────────────────────────────────────────┐ │
│  │ Layer 1: Enhancement (Optional)        │ │
│  │   - Planning (if enabled)              │ │
│  │   - Prompt Generation (if enabled)     │ │
│  └────────────────────────────────────────┘ │
│                    ↑                         │
│  ┌────────────────────────────────────────┐ │
│  │ Layer 2: Adaptation (Optional)         │ │
│  │   - Dynamic Temperature (if enabled)   │ │
│  └────────────────────────────────────────┘ │
│                    ↑                         │
│  ┌────────────────────────────────────────┐ │
│  │ Layer 3: Delegation (Optional)         │ │
│  │   - Handoff Processing (if enabled)    │ │
│  └────────────────────────────────────────┘ │
└──────────────────────────────────────────────┘
```

**Benefits**:
- Core execution never breaks
- Features can be tested independently
- Performance impact is minimal when features are disabled
- Easy to add new enhancement layers

### 6.6 Dependencies

**Hard Dependencies**:
- Epic 19 (Herald & Domain Type Consolidation) - MUST be complete
- Existing Epic 14 foundation (planning, prompts, temperature, handoffs)

**Service Dependencies**:
- `PaladinExecutionService` (core)
- `PlanningService` (Layer 1)
- `PromptGenerationService` (Layer 1)
- `HandoffService` (Layer 3)
- `LlmPort` (all layers)

**Crate Dependencies**:
- No new external dependencies required
- All functionality uses existing crates

---

## 7. Success Metrics

### SM-1: Functional Completeness
- ✅ All 23 deferred tasks from Epic 14 completed
- ✅ All 5 user stories pass acceptance criteria
- ✅ 0 `TODO` comments remaining in autonomous agent code

### SM-2: Quality Metrics
- ✅ Unit test coverage ≥90% for autonomous components
- ✅ All integration tests passing
- ✅ At least 3 E2E scenarios validated
- ✅ `cargo clippy` produces 0 warnings
- ✅ `cargo fmt --check` passes
- ✅ Release build succeeds

### SM-3: Backward Compatibility
- ✅ All existing tests pass without modification
- ✅ No breaking changes to public APIs
- ✅ Existing examples continue to work
- ✅ Configuration is backward compatible

### SM-4: Documentation
- ✅ All new fields documented in rustdoc
- ✅ Configuration examples added to docs
- ✅ Handoff workflow documented with diagrams
- ✅ CHANGELOG.md updated with Epic 21 changes

### SM-5: Performance
- ✅ Core execution performance unchanged when features disabled
- ✅ Handoff execution completes within 2x single agent time
- ✅ Retry logic respects backoff timing (no busy loops)
- ✅ Memory usage remains stable with handoff chains

---

## 8. Open Questions

**Q1**: Should handoff retry configuration be global (per Paladin) or per-handoff (per specialist)?  
**Decision Needed By**: Phase 5 (Day 5)  
**Options**: Global is simpler, per-handoff is more flexible

**Q2**: How should we handle concurrent handoffs (if agent calls multiple specialists in parallel)?  
**Decision Needed By**: Phase 5 (Day 6)  
**Options**: Sequential (safer), Parallel (faster but complex)

**Q3**: Should `TaskPlan` validation happen at planning time or execution time?  
**Decision Needed By**: Phase 4 (Day 2)  
**Options**: Planning time (fail fast), Execution time (more flexible)

**Q4**: What should happen if a specialist in a handoff chain is removed/reconfigured mid-execution?  
**Decision Needed By**: Phase 5 (Day 5)  
**Options**: Fail with clear error, Use fallback specialist, Continue without that handoff

**Q5**: Should handoff history be capped to prevent unbounded growth in long-running agents?  
**Decision Needed By**: Phase 2 (Day 4)  
**Options**: No cap (simplest), Cap at N records, Ring buffer

## Epic 21 Open Questions - Recommendations

| Question | Recommendation | Rationale |
|----------|----------------|-----------|
| Q1 | **Global (per Paladin)** | Start simple, design for extensibility |
| Q2 | **Sequential** | Correctness first, optimize later |
| Q3 | **Planning time (fail fast)** | Better developer experience |
| Q4 | **Fail with clear error** | Explicit > implicit |
| Q5 | **Cap at N records (default: 100)** | Bounded memory, useful history |

---

## Detailed Rationale

### Q1: Handoff Retry Configuration Scope
**Recommendation: Global (per Paladin) with extensibility**

```yaml
# Phase 1: Global configuration
handoffs:
  retry:
    max_retries: 3
    initial_backoff_ms: 1000
    backoff_multiplier: 2.0
  specialists:
    - name: "CodeExpert"
    - name: "DataAnalyst"

# Phase 2: Per-specialist override
handoffs:
  retry:  # Default
    max_retries: 3
  specialists:
    - name: "CodeExpert"
      retry:  # Override
        max_retries: 5
```

**Why**: Global is simpler to implement and covers 90% of use cases. The configuration schema can support per-specialist overrides in the future without breaking changes.

---

### Q2: Concurrent Handoffs
**Recommendation: Sequential execution**

```
Agent calls: [CodeExpert, DataAnalyst, SecurityAuditor]

Sequential (implement now):
  CodeExpert → wait → DataAnalyst → wait → SecurityAuditor → aggregate

Parallel (future optimization):
  ┌→ CodeExpert ────┐
  ├→ DataAnalyst ───┼→ aggregate
  └→ SecurityAuditor┘
```

**Why**:
- **Error handling is cleaner**: One failure doesn't complicate others
- **Resource usage is predictable**: No API rate limit surprises
- **Debugging is easier**: Linear execution trace
- **Correctness first**: Matches our Phase 5 priority

Add a config flag for future:
```yaml
handoffs:
  concurrent: false  # Default, sequential execution
```

---

### Q3: TaskPlan Validation Timing
**Recommendation: Planning time (fail fast)**

**Why**:
- **Better DX**: Developers see errors immediately, not after partial execution
- **Resource efficient**: No wasted LLM calls on invalid plans
- **Predictable**: Either we have a valid plan or we don't execute
- **Matches existing patterns**: Similar to config validation in `PaladinBuilder::build()`

---

### Q4: Specialist Removed Mid-Execution
**Recommendation: Fail with clear error**

**Why**:
- **Explicit is better than implicit**: Silent fallbacks hide bugs
- **Debugging clarity**: Clear error message with chain context
- **User expectation**: If they configured a specialist, they expect it to be used
- **Safety**: Fallback specialist might not be appropriate for the task

**Alternative considered but rejected**:
- *Fallback specialist*: Could produce incorrect results if fallback isn't suitable
- *Continue without handoff*: Silently changes behavior, hard to debug

---

### Q5: Handoff History Cap
**Recommendation: Cap at N records (default: 100)**

**Why**:
- **Bounded memory**: Prevents OOM in long-running agents
- **Useful history preserved**: 100 records covers typical debugging needs
- **Simple implementation**: Vec with remove(0) is O(n) but acceptable at this scale
- **Configurable**: Power users can adjust based on their needs

**Configuration**:
```yaml
handoffs:
  history:
    max_records: 100  # Default
    eviction: oldest_first
```

---

## Summary Configuration Schema

```yaml
paladin:
  name: "AutonomousAgent"
  model: "gpt-4o"

  handoffs:
    enabled: true
    max_depth: 5
    concurrent: false  # Q2: Sequential for now

    retry:  # Q1: Global configuration
      max_retries: 3
      initial_backoff_ms: 1000
      backoff_multiplier: 2.0

    history:  # Q5: Capped history
      max_records: 100
      eviction: oldest_first

    on_specialist_unavailable: fail  # Q4: Explicit failure
    # Future options: fallback, skip

    specialists:
      - name: "CodeExpert"
        description: "Code analysis"
      - name: "DataAnalyst"
        description: "Data analysis"

  planning:
    validate_at: planning_time  # Q3: Fail fast
    # Future option: execution_time
```

---

## 9. Appendices

### Appendix A: Deferred Task Mapping

| Epic 14 Task | User Story | Phase |
|--------------|------------|-------|
| 5.17, 5.18 | US-21.1 | Phase 5 |
| 5.23, 5.24 | US-21.3 | Phase 2 |
| 5.27, 5.28 | US-21.1 | Phase 5 |
| 6.5, 6.6 | US-21.2 | Phase 3 |
| 6.9-6.14 | US-21.1 | Phase 5 |
| 6.17-6.22 | US-21.1 | Phase 5 |
| 7.11-7.14 | US-21.3 | Phase 2 |
| 7.15-7.20 | US-21.4 | Phase 4 |
| Planning/Prompt hardcoded models | US-21.5 | Phase 1 |

**Total Deferred Tasks Resolved**: 23

### Appendix B: Configuration Examples

**Minimal Configuration** (all features disabled):
```yaml
paladin:
  name: "BasicAgent"
  model: "gpt-4o"
  autonomous_planning: false
  autonomous_prompts: false
  dynamic_temperature: false
```

**Full Autonomous Configuration**:
```yaml
paladin:
  name: "FullyAutonomous"
  model: "gpt-4o"
  autonomous_planning: true
  autonomous_prompts: true
  dynamic_temperature: true
  handoffs:
    enabled: true
    max_depth: 5
    retry:
      max_retries: 3
      initial_backoff_ms: 1000
      backoff_multiplier: 2.0
    specialists:
      - name: "CodeExpert"
        description: "Expert in code analysis and generation"
        model: "gpt-4o"  # Can override per specialist
      - name: "DataAnalyst"
        description: "Expert in data analysis and visualization"
        model: "gpt-4o"
```

### Appendix C: Error Code Reference

| Code | Error | Retry? | Description |
|------|-------|--------|-------------|
| E-HANDOFF-001 | CircularHandoff | No | Specialist called itself or created a cycle |
| E-HANDOFF-002 | MaxDepthExceeded | No | Handoff chain exceeded max_depth limit |
| E-HANDOFF-003 | SpecialistNotFound | No | Requested specialist not in configuration |
| E-HANDOFF-004 | ExecutionFailed | Yes | Specialist execution failed (network/timeout) |
| E-HANDOFF-005 | InvalidResponse | No | Specialist returned malformed result |
| E-PLAN-001 | PlanGenerationFailed | Yes | LLM failed to generate valid plan |
| E-PROMPT-001 | PromptGenerationFailed | Yes | LLM failed to generate prompt |
| E-CONFIG-001 | InvalidModel | No | Configured model doesn't exist |
| E-CONFIG-002 | ModelNotCapable | No | Model lacks required capabilities |

---

## 10. Acceptance Checklist

Before marking Epic 21 complete, verify:

- [ ] All 5 user stories pass acceptance criteria
- [ ] All 23 deferred Epic 14 tasks completed
- [ ] Unit test coverage ≥90%
- [ ] All integration tests passing
- [ ] E2E tests cover at least 3 scenarios
- [ ] `cargo test` passes (0 failures)
- [ ] `cargo clippy -- -D warnings` passes (0 warnings)
- [ ] `cargo fmt --check` passes
- [ ] `cargo build --release` succeeds
- [ ] All existing tests pass without modification
- [ ] No breaking changes to public APIs
- [ ] rustdoc updated for all new types/fields
- [ ] Configuration examples added to documentation
- [ ] CHANGELOG.md updated
- [ ] Examples updated (agent_handoffs.rs, autonomous_*.rs)
- [ ] No `TODO` comments in production code
- [ ] PR created with link to this PRD
- [ ] Code review completed by 2+ maintainers

---

**Epic 21: Autonomous Agent Completion** - Ready for Implementation 🚀
