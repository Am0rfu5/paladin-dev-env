# Product Requirements Document: Epic 22 — Battalion & Commander Hardening

## Document Information

- **Epic:** Epic 22
- **Version:** 1.0
- **Created:** February 12, 2026
- **Status:** Draft
- **Theme:** Complete Battalion integration gaps, fix Commander delegation
- **Duration:** 2 weeks
- **Priority:** High
- **Dependencies:** Epic 19 (Herald & Domain Type Consolidation)

---

## 1. Introduction/Overview

Epic 22 addresses critical integration gaps in the Battalion orchestration subsystem that were deferred during Milestones 1 and 2. The primary issues are:

1. **Council and Grove** store Paladin IDs (strings) but cannot resolve them to actual `Paladin` instances for execution
2. **Grove's LLM-based routing** is stubbed with `unimplemented!()`, limiting intelligent agent selection
3. **Phalanx** doesn't track per-paladin execution metrics, making performance analysis impossible
4. **Commander** metadata export to files is unimplemented
5. **Commander tests** for Campaign, ChainOfCommand, and error handling are ignored/disabled

These gaps prevent Battalion patterns from being production-ready. This epic completes the integration work, enabling full multi-agent orchestration with proper observability and reliability.

**Problem Statement:** Battalion subsystems can configure multi-agent patterns but cannot execute them properly due to missing runtime resolution, stubbed logic, and incomplete observability.

**Solution:** Implement a trait-based Paladin registry, complete LLM routing logic, add comprehensive metrics, enable metadata export, and harden all tests.

---

## 2. Goals

1. **Enable Production-Ready Multi-Agent Orchestration:** All Battalion patterns (Formation, Phalanx, Campaign, Chain of Command, Council, Grove) must execute end-to-end with real Paladin instances
2. **Complete Intelligent Routing:** Grove must use LLM for semantic routing with configurable fallback to keyword matching
3. **Comprehensive Observability:** Phalanx must track per-paladin execution times and token counts for performance analysis
4. **Persistent Metadata:** Commander must export execution metadata to JSON files for auditing and analysis
5. **Full Test Coverage:** All ignored Commander tests enabled and passing with proper mock infrastructure
6. **Zero Breaking Changes:** Maintain backward compatibility with existing Battalion APIs

---

## 3. User Stories

### US-22.1: Paladin Registry for Council and Grove

**As a** developer  
**I want** Council and Grove to resolve Paladin IDs to actual Paladin instances  
**So that** multi-agent patterns can execute with real agents instead of just storing string IDs

**Acceptance Criteria:**
- [ ] Define a `PaladinRegistry` trait with methods: `register(id: String, paladin: Arc<Paladin>)`, `get(&id: &str) -> Option<Arc<Paladin>>`, `contains(&id: &str) -> bool`
- [ ] Implement `HashMapPaladinRegistry` as default concrete implementation
- [ ] Council's `execute()` method resolves participant IDs to `Paladin` instances before discussion rounds
- [ ] Grove's `execute()` method resolves routed agent ID to `Paladin` instance before execution
- [ ] Commander populates registry when creating Council/Grove battalions from configuration
- [ ] Registry is passed to Council/Grove services via constructor (trait object `Arc<dyn PaladinRegistry>`)
- [ ] Error handling: return `BattalionError::PaladinNotFound(id)` if ID cannot be resolved
- [ ] Unit tests verify:
  - Registry registration and lookup work correctly
  - Council resolves all participants (test with 3 paladins)
  - Grove resolves selected agent
  - Error handling when Paladin ID is missing
- [ ] Integration tests verify full execution flow with resolved Paladins

**Technical Details (from user selections):**
- Use trait-based registry pattern (not HashMap-only or thread-safe shared singleton)
- Registry passed as dependency to services, not globally accessible
- Registry should be usable across async boundaries (`Send + Sync`)

**Source Files:**
- `src/application/use_cases/battalion/council_service.rs` — line 160
- `src/application/use_cases/battalion/commander.rs` — lines 562, 617
- Create: `src/application/ports/output/paladin_registry.rs` (trait definition)
- Create: `src/infrastructure/adapters/paladin_registry.rs` (HashMap implementation)

---

### US-22.2: Grove LLM-Based Routing Implementation

**As a** developer  
**I want** Grove to use the LLM for intelligent routing based on semantic understanding  
**So that** input is routed to the best-matching agent instead of relying solely on keyword matching

**Acceptance Criteria:**
- [ ] `GroveService::route_with_llm()` sends routing prompt to configured LLM provider
- [ ] Routing prompt includes:
  - User input text
  - List of available agents with descriptions and specializations
  - Instruction to return JSON: `{"tree_name": "...", "agent_id": "...", "confidence": 0.0-1.0, "reasoning": "..."}`
- [ ] Parse JSON response using `serde_json`
- [ ] Validate parsed response: `confidence` in range [0.0, 1.0], `agent_id` exists in Grove config
- [ ] Configurable fallback behavior via `GroveConfig::routing_fallback`:
  - `"keyword"` → fall back to keyword matching on LLM failure
  - `"error"` → return error, do not fall back
- [ ] Configurable confidence threshold via `GroveConfig::min_confidence` (default: 0.5)
  - If parsed `confidence < min_confidence`, treat as routing failure
- [ ] Error handling for:
  - LLM API call failure (network, timeout, rate limit)
  - Invalid JSON response
  - Missing fields in JSON
  - Unknown `agent_id` in response
- [ ] Unit tests with mock LLM responses:
  - Successful routing (high confidence)
  - Low confidence routing (triggers fallback if configured)
  - Invalid JSON response
  - LLM call failure
  - Fallback to keyword matching works correctly
- [ ] Integration test for full LLM-based routing flow with real LLM adapter (mocked HTTP)

**Technical Details (from user selections):**
- Use configurable fallback: keyword matching OR error (not always fallback)
- Add `routing_fallback: String` and `min_confidence: f32` fields to `GroveConfig`

**Source Files:**
- `src/application/use_cases/battalion/grove_service.rs` — line 475
- `src/core/platform/container/battalion/grove.rs` — add config fields

---

### US-22.3: Phalanx Per-Paladin Timing & Token Metrics

**As a** developer  
**I want** Phalanx to track individual Paladin execution times and token usage  
**So that** I can identify performance bottlenecks and cost drivers in parallel execution

**Acceptance Criteria:**
- [ ] `PhalanxService::execute()` records start and end time for each Paladin execution
- [ ] `BattalionMetadata::per_paladin_times` populated as `HashMap<String, Duration>`
  - Key: Paladin ID
  - Value: Execution duration
- [ ] `BattalionMetadata::per_paladin_tokens` populated as `HashMap<String, TokenUsage>`
  - Key: Paladin ID
  - Value: Token counts (prompt_tokens, completion_tokens, total_tokens)
- [ ] Extract token usage from `PaladinResult::metadata` if available
- [ ] Calculate aggregate metrics:
  - `paladin_success_count`: Number of Paladins that completed successfully
  - `paladin_failure_count`: Number of Paladins that returned errors
  - `total_tokens`: Sum of all Paladin token usage
- [ ] Metrics survive all error conditions (partial failures, timeouts)
- [ ] Unit tests verify:
  - Timing accuracy (within 10ms tolerance)
  - Token counts correctly aggregated from multiple Paladins
  - Success/failure counts accurate
  - Metrics captured even when some Paladins fail

**Technical Details (from user selections):**
- Track per-paladin times + token counts (not memory or full telemetry)
- Token usage from `PaladinResult` metadata, not instrumented separately

**Source Files:**
- `src/application/use_cases/battalion/phalanx_service.rs` — line 270
- `src/core/platform/container/battalion/battalion_result.rs` — extend `BattalionMetadata`

---

### US-22.4: Commander Metadata Export

**As a** developer  
**I want** Commander to export execution metadata to JSON files  
**So that** I can analyze orchestration performance, trace execution paths, and debug issues

**Acceptance Criteria:**
- [ ] If `CommanderConfig::metadata_output_dir` is configured (not None), Commander writes metadata JSON after each execution
- [ ] File naming convention: `<metadata_output_dir>/<strategy>_<timestamp>_<uuid>.json`
  - Example: `./metadata/formation_20260212_153045_a1b2c3d4.json`
  - `strategy`: formation, phalanx, campaign, etc.
  - `timestamp`: `YYYYMMDD_HHMMSS` in local time
  - `uuid`: Short UUID (8 chars) for uniqueness
- [ ] JSON structure includes full `BattalionMetadata`:
  - Strategy used, total duration, Paladin count
  - Per-paladin times and tokens
  - Success/failure counts
  - Configuration snapshot (sanitized, no secrets)
- [ ] Error handling:
  - Directory creation if it doesn't exist
  - File write failures are logged but non-fatal (execution continues)
  - Permissions errors logged with clear message
- [ ] Unit tests:
  - Verify file created with correct naming convention
  - Verify JSON content matches metadata structure
  - Verify directory creation if missing
  - Verify error handling for write failures (use temp directory)
- [ ] Integration test: Full Commander execution with metadata export enabled

**Technical Details (from user selections):**
- JSON format only (no CSV/YAML/Markdown)
- Metadata export is opt-in via config

**Source Files:**
- `src/application/use_cases/battalion/commander.rs` (add export logic after execution)
- `src/core/platform/container/battalion/commander_config.rs` — add `metadata_output_dir: Option<PathBuf>`

---

### US-22.5: Commander Ignored Test Completion

**As a** developer  
**I want** all ignored Commander tests to be enabled and passing  
**So that** Campaign, ChainOfCommand, and error handling are fully validated and regression-safe

**Acceptance Criteria:**

**Phase 1: Campaign and ChainOfCommand Tests (Priority)**
- [ ] `test_execute_campaign` enabled and passing:
  - Mock DAG-based execution with 4+ nodes
  - Verify execution order respects dependencies
  - Verify results collected correctly
- [ ] `test_execute_chain_of_command` enabled and passing:
  - Mock hierarchical delegation (supervisor → 2 workers)
  - Verify delegation flow works correctly
  - Verify results aggregated from delegated Paladins

**Phase 2: Error Handling Tests**
- [ ] `test_error_handling_fail_fast` enabled and passing:
  - Mock Paladin failure in Formation
  - Verify execution stops immediately
  - Verify error propagated correctly
- [ ] `test_error_handling_continue_on_error` enabled and passing:
  - Mock Paladin failure with `continue_on_error: true`
  - Verify remaining Paladins execute
  - Verify partial results returned
- [ ] `test_error_handling_retry_then_continue` enabled and passing:
  - Mock Paladin failure with retry policy
  - Verify retry attempts made
  - Verify continuation after exhausted retries
- [ ] `test_partial_failure_handling` enabled and passing:
  - Mock mixed success/failure in Phalanx
  - Verify successful results preserved
  - Verify failure details captured

**Mock Infrastructure:**
- [ ] Use real `Paladin` instances with mock LLM adapter (per user selection 7D)
- [ ] Create `MockLlmAdapter` that can be configured to:
  - Return specific responses
  - Simulate failures
  - Count invocations for verification
- [ ] Helpers for building test Paladins with mock LLM

**Source Files:**
- `src/application/use_cases/battalion/commander.rs` — lines 1850, 1875, 2017, 2025, 2033, 2041
- Create: `tests/helpers/mock_llm_adapter.rs` (or enhance existing)

---

## 4. Functional Requirements

### FR-1: Paladin Registry Trait
- FR-1.1: Define `PaladinRegistry` trait in `application/ports/output/`
- FR-1.2: Trait must be `Send + Sync` for async compatibility
- FR-1.3: Methods: `register()`, `get()`, `contains()`, `list_ids()`
- FR-1.4: Return types use `Arc<Paladin>` for shared ownership

### FR-2: Paladin Registry Implementation
- FR-2.1: Implement `HashMapPaladinRegistry` using `HashMap<String, Arc<Paladin>>`
- FR-2.2: Thread-safe access via `RwLock` or `Mutex`
- FR-2.3: Constructor: `new()` creates empty registry

### FR-3: Council Integration
- FR-3.1: `CouncilService` accepts `Arc<dyn PaladinRegistry>` in constructor
- FR-3.2: `execute()` resolves all participant IDs before discussion rounds
- FR-3.3: Return `BattalionError::PaladinNotFound` if any participant ID missing

### FR-4: Grove Integration
- FR-4.1: `GroveService` accepts `Arc<dyn PaladinRegistry>` in constructor
- FR-4.2: After routing decision, resolve agent ID to Paladin instance
- FR-4.3: Return `BattalionError::PaladinNotFound` if routed agent ID missing

### FR-5: Grove LLM Routing
- FR-5.1: Add `route_with_llm()` method to `GroveService`
- FR-5.2: Build routing prompt with agent descriptions and input
- FR-5.3: Call LLM via existing `LlmPort`
- FR-5.4: Parse JSON response with required fields: `tree_name`, `agent_id`, `confidence`, `reasoning`
- FR-5.5: Validate `confidence` against `min_confidence` threshold
- FR-5.6: Implement fallback strategy based on `routing_fallback` config
- FR-5.7: Log routing decisions with reasoning for observability

### FR-6: Grove Configuration Extension
- FR-6.1: Add `routing_fallback: String` field to `GroveConfig` (values: "keyword" or "error")
- FR-6.2: Add `min_confidence: f32` field (default: 0.5, range: 0.0-1.0)
- FR-6.3: Validation: reject invalid fallback values or out-of-range confidence

### FR-7: Phalanx Metrics
- FR-7.1: Record start/end time for each Paladin in parallel execution
- FR-7.2: Extract token usage from `PaladinResult::metadata`
- FR-7.3: Populate `per_paladin_times: HashMap<String, Duration>`
- FR-7.4: Populate `per_paladin_tokens: HashMap<String, TokenUsage>`
- FR-7.5: Calculate `paladin_success_count` and `paladin_failure_count`
- FR-7.6: Calculate `total_tokens` as sum of all Paladin token usage

### FR-8: BattalionMetadata Extension
- FR-8.1: Add `per_paladin_times` field to `BattalionMetadata`
- FR-8.2: Add `per_paladin_tokens` field to `BattalionMetadata`
- FR-8.3: Add `total_tokens` field for aggregate token count
- FR-8.4: All new fields must be serializable (derive Serialize/Deserialize)

### FR-9: Commander Metadata Export
- FR-9.1: After each execution, check if `metadata_output_dir` is configured
- FR-9.2: If configured, serialize `BattalionMetadata` to JSON
- FR-9.3: Create output directory if it doesn't exist
- FR-9.4: Write JSON to file with naming convention: `<strategy>_<timestamp>_<uuid>.json`
- FR-9.5: Log success/failure of metadata export
- FR-9.6: Export failure must not prevent execution success (non-fatal)

### FR-10: Commander Configuration Extension
- FR-10.1: Add `metadata_output_dir: Option<PathBuf>` to `CommanderConfig`
- FR-10.2: If None, metadata export is disabled (default behavior)
- FR-10.3: If Some, validate path is writable before first execution

### FR-11: Commander Test Infrastructure
- FR-11.1: Create or enhance `MockLlmAdapter` for test use
- FR-11.2: `MockLlmAdapter` must support:
  - Configurable responses per call
  - Failure simulation
  - Call count tracking
- FR-11.3: Helper functions to build test Paladins with mock LLM
- FR-11.4: All Campaign and ChainOfCommand tests enabled (remove `#[ignore]`)
- FR-11.5: All error handling tests enabled and passing

---

## 5. Non-Goals (Out of Scope)

### NG-1: Advanced Registry Features
- Multi-tenancy or namespace support in registry
- Persistent registry (disk-backed)
- Distributed registry across multiple nodes

### NG-2: Metadata Export Formats
- CSV, YAML, Markdown, or other non-JSON formats
- Real-time streaming metadata to external systems
- Metadata compression or encryption

### NG-3: Enhanced Metrics
- Memory usage tracking per Paladin
- GPU utilization tracking
- Network I/O metrics
- Custom metric plugins

### NG-4: Grove Routing Enhancements
- Multi-agent routing (route to multiple agents simultaneously)
- Contextual routing history (learn from past routing decisions)
- A/B testing different routing strategies

### NG-5: Backward Compatibility Breaking Changes
- Changing existing Battalion API signatures
- Renaming existing types or methods
- Removing deprecated functionality

### NG-6: Performance Optimization
- Parallel LLM calls in routing
- Caching routing decisions
- Optimizing registry lookup performance

---

## 6. Design Considerations

### 6.1 Architecture Patterns
- **Hexagonal Architecture:** Paladin registry defined as port (trait) in application layer, implemented in infrastructure
- **Dependency Injection:** Registry passed to services via constructor, not global singleton
- **Error Handling:** Use `thiserror` for domain-specific errors, propagate with context

### 6.2 Paladin Registry Design
```rust
// Application layer port
pub trait PaladinRegistry: Send + Sync {
    fn register(&self, id: String, paladin: Arc<Paladin>) -> Result<(), RegistryError>;
    fn get(&self, id: &str) -> Option<Arc<Paladin>>;
    fn contains(&self, id: &str) -> bool;
    fn list_ids(&self) -> Vec<String>;
}

// Infrastructure layer implementation
pub struct HashMapPaladinRegistry {
    paladins: RwLock<HashMap<String, Arc<Paladin>>>,
}
```

### 6.3 Grove LLM Routing Flow
1. Build routing prompt with agent metadata
2. Call LLM via `LlmPort::generate()`
3. Parse JSON response
4. Validate confidence threshold
5. If valid, return routed agent ID
6. If invalid/error, check fallback strategy
7. If fallback enabled, use keyword matching
8. If fallback disabled, return error

### 6.4 Metadata Export File Structure
```json
{
  "strategy": "formation",
  "timestamp": "2026-02-12T15:30:45Z",
  "duration_ms": 1234,
  "paladin_count": 3,
  "success_count": 2,
  "failure_count": 1,
  "total_tokens": 1500,
  "per_paladin_times": {
    "analyzer": 450,
    "writer": 600,
    "reviewer": 184
  },
  "per_paladin_tokens": {
    "analyzer": {"prompt": 100, "completion": 200, "total": 300},
    "writer": {"prompt": 150, "completion": 350, "total": 500},
    "reviewer": {"prompt": 200, "completion": 500, "total": 700}
  },
  "config_snapshot": {
    "max_loops": 3,
    "temperature": 0.7
  }
}
```

### 6.5 Test Infrastructure Design
- Use real `Paladin` instances (not mock Paladins)
- Mock only the LLM adapter layer
- `MockLlmAdapter` should allow per-test configuration
- Test helpers should make it easy to create test Paladins

---

## 7. Technical Considerations

### 7.1 Dependencies
- **Required:** Epic 19 (Herald & Domain Type Consolidation) must be complete
- **LLM Integration:** Uses existing `LlmPort` trait for Grove routing
- **File I/O:** Uses `std::fs` and `serde_json` for metadata export
- **Threading:** Registry must be thread-safe (RwLock or Mutex)

### 7.2 Performance
- Registry lookup is O(1) via HashMap
- Metadata export is async-safe but file I/O may block briefly
- Phalanx metrics collection adds minimal overhead (<1% of execution time)

### 7.3 Error Handling
- New error variants:
  - `BattalionError::PaladinNotFound(String)`
  - `BattalionError::GroveRoutingFailed(String)`
  - `BattalionError::MetadataExportFailed(String)` (non-fatal)
  - `RegistryError::DuplicateId(String)`
  - `RegistryError::InvalidId(String)`

### 7.4 Configuration Schema
```yaml
commander:
  metadata_output_dir: "./metadata"  # Optional
  
grove:
  trees:
    - name: "support"
      routing_fallback: "keyword"  # or "error"
      min_confidence: 0.6
      agents:
        - id: "billing_expert"
          description: "Handles billing and payment questions"
```

### 7.5 Testing Strategy
- **Unit Tests:** Test each component in isolation with mocks
- **Integration Tests:** Test full execution flows with real components (except LLM)
- **Test Coverage Target:** ≥80% line coverage for new code
- **TDD Approach:** Write tests first for US-22.5

### 7.6 Migration Path
- All new features are opt-in or backward-compatible
- Existing Battalion configurations work without changes
- Registry is required only for Council and Grove (other patterns unaffected)

---

## 8. Success Metrics

### 8.1 Functional Metrics
- [ ] 100% of deferred Epic 22 tasks completed
- [ ] All 6 ignored Commander tests enabled and passing
- [ ] Zero regressions in existing Battalion tests

### 8.2 Quality Metrics
- [ ] ≥80% unit test coverage for new code
- [ ] All clippy warnings resolved
- [ ] All integration tests passing
- [ ] Documentation updated for new features

### 8.3 Performance Metrics
- [ ] Registry lookup overhead <1ms per operation
- [ ] Metadata export overhead <50ms per execution
- [ ] Phalanx metrics collection overhead <1% of total execution time

### 8.4 Observability Metrics
- [ ] Per-paladin timing visible in metadata
- [ ] Token usage tracked for all Paladins in parallel execution
- [ ] Metadata export success rate logged

---

## 9. Open Questions

### Q1: Registry Population Timing
**Question:** Should Commander populate the registry before or after Battalion configuration validation?  
**Impact:** Affects error handling and resource allocation  
**Recommendation:** Populate after validation to avoid unnecessary Paladin instantiation for invalid configs

### Q2: Grove Routing Prompt Engineering
**Question:** What should the exact LLM prompt structure be for routing?  
**Impact:** Affects routing accuracy and reliability  
**Recommendation:** Start with simple prompt, iterate based on evaluation results; document prompt template in code

### Q3: Metadata Export Rotation/Retention
**Question:** Should old metadata files be automatically deleted or rotated?  
**Impact:** Disk space management in long-running deployments  
**Resolution:** Out of scope for this epic; add to backlog for future consideration

### Q4: Token Count Propagation
**Question:** Do all LLM adapters currently populate token usage in `PaladinResult::metadata`?  
**Impact:** Completeness of per-paladin token metrics  
**Action Required:** Audit existing adapters (OpenAI, DeepSeek, Anthropic) and fix if missing

### Q5: Commander Test Data Requirements
**Question:** What test scenarios should Campaign and ChainOfCommand tests cover?  
**Impact:** Test comprehensiveness  
**Recommendation:** Minimum: happy path, single failure, multiple dependencies, invalid DAG

---

## 10. Implementation Checklist

### Phase 1: Foundation (Days 1-3)
- [ ] Define `PaladinRegistry` trait
- [ ] Implement `HashMapPaladinRegistry`
- [ ] Unit tests for registry operations
- [ ] Update Commander to use registry

### Phase 2: Council & Grove Integration (Days 4-6)
- [ ] Integrate registry into CouncilService
- [ ] Integrate registry into GroveService
- [ ] Implement Grove LLM routing
- [ ] Add Grove config fields
- [ ] Unit and integration tests

### Phase 3: Metrics & Export (Days 7-8)
- [ ] Extend BattalionMetadata with new fields
- [ ] Implement Phalanx per-paladin metrics
- [ ] Implement Commander metadata export
- [ ] Unit tests for metrics and export

### Phase 4: Test Hardening (Days 9-10)
- [ ] Create/enhance MockLlmAdapter
- [ ] Enable Campaign tests
- [ ] Enable ChainOfCommand tests
- [ ] Enable error handling tests
- [ ] Verify all tests pass in CI

---

## 11. Acceptance Criteria for Epic Completion

- [ ] All 5 User Stories (US-22.1 through US-22.5) completed with acceptance criteria met
- [ ] All functional requirements (FR-1 through FR-11) implemented
- [ ] All deferred tasks from Epics 4, 5, 15, 16 resolved
- [ ] All inline TODOs in Battalion and Commander files resolved
- [ ] `cargo test` passes with ≥80% coverage for new code
- [ ] `cargo clippy` passes with no warnings
- [ ] `cargo fmt` applied to all modified files
- [ ] Integration tests pass for full Battalion execution flows
- [ ] Documentation updated:
  - Battalion pattern docs (Formation, Phalanx, Council, Grove)
  - Commander configuration reference
  - Metadata export usage guide
- [ ] Example code updated to demonstrate new features
- [ ] Code review completed and feedback addressed
- [ ] All changes committed with conventional commit messages

---

## Appendix A: File Inventory

### Files to Create
1. `src/application/ports/output/paladin_registry.rs` — Registry trait definition
2. `src/infrastructure/adapters/paladin_registry.rs` — HashMap implementation
3. `tests/helpers/mock_llm_adapter.rs` — Mock LLM for testing (if not exists)

### Files to Modify
1. `src/application/use_cases/battalion/council_service.rs`
2. `src/application/use_cases/battalion/grove_service.rs`
3. `src/application/use_cases/battalion/phalanx_service.rs`
4. `src/application/use_cases/battalion/commander.rs`
5. `src/core/platform/container/battalion/grove.rs`
6. `src/core/platform/container/battalion/commander_config.rs`
7. `src/core/platform/container/battalion/battalion_result.rs`

### Test Files to Update
1. `tests/unit/council_tests.rs`
2. `tests/unit/grove_tests.rs`
3. `tests/unit/phalanx_tests.rs`
4. `tests/integration/battalion_integration_tests.rs`
5. `src/application/use_cases/battalion/commander.rs` (test module)

---

## Appendix B: Related Epics & Dependencies

- **Epic 4:** Formation & Phalanx (original Battalion patterns)
- **Epic 5:** Commander Orchestration (original Commander implementation)
- **Epic 15:** Council & Grove (multi-agent discussion patterns)
- **Epic 16:** Conclave (expert panels with voting)
- **Epic 19:** Herald & Domain Type Consolidation (prerequisite)
- **Epic 24:** Test Hardening & Benchmarks (follow-on epic)

---

**End of PRD**
