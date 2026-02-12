# Task List: Epic 22 — Battalion & Commander Hardening

**Epic:** Epic 22  
**Feature:** Battalion & Commander Hardening  
**Duration:** 2 weeks (10 days)  
**Priority:** High  
**Dependencies:** Epic 19 (Herald & Domain Type Consolidation)

---

## Relevant Files

### Files to Create
- `src/application/ports/output/paladin_registry.rs` - Registry trait definition with Send + Sync bounds for async compatibility
- `src/infrastructure/adapters/paladin_registry.rs` - HashMap-based registry implementation with thread-safe access
- `tests/helpers/mock_llm_adapter.rs` - Mock LLM adapter for testing with configurable responses and failure simulation
- `tests/unit/paladin_registry_tests.rs` - Unit tests for registry operations

### Files to Modify
- `src/application/use_cases/battalion/council_service.rs` - Integrate Paladin registry for participant resolution (line 160)
- `src/application/use_cases/battalion/grove_service.rs` - Add LLM-based routing and registry integration (line 475)
- `src/application/use_cases/battalion/phalanx_service.rs` - Add per-paladin timing and token metrics (line 270)
- `src/application/use_cases/battalion/commander.rs` - Add metadata export and enable ignored tests (lines 562, 617, 1850, 1875, 2017, 2025, 2033, 2041)
- `src/core/platform/container/battalion/grove.rs` - Add routing_fallback and min_confidence config fields
- `src/core/platform/container/battalion/commander_config.rs` - Add metadata_output_dir config field
- `src/core/platform/container/battalion/battalion_result.rs` - Extend BattalionMetadata with per_paladin_times, per_paladin_tokens, total_tokens
- `src/core/platform/container/battalion/error.rs` - Add new error variants (PaladinNotFound, GroveRoutingFailed, MetadataExportFailed)

### Test Files
- `tests/unit/council_tests.rs` - Test registry integration in Council
- `tests/unit/grove_tests.rs` - Test LLM routing and registry integration
- `tests/unit/phalanx_tests.rs` - Test per-paladin metrics collection
- `tests/integration/battalion_integration_tests.rs` - End-to-end Battalion execution tests
- `src/application/use_cases/battalion/commander.rs` (test module) - Enable and fix ignored tests

### Notes

- Unit tests in Rust are typically placed in the same file within a `#[cfg(test)]` module
- Use `cargo test` to run all tests
- Use `cargo test test_name` to run specific tests
- Use `cargo test -- --nocapture` to see println! output
- Follow TDD approach: write tests first, then implementation
- All new traits must be `Send + Sync` for async compatibility
- Use `Arc<Paladin>` for shared ownership across services

---

## Instructions for Completing Tasks

**IMPORTANT:** As you complete each task, you must check it off in this markdown file by changing `- [ ]` to `- [x]`. This helps track progress and ensures you don't skip any steps.

Example:
- `- [ ] 1.1 Read file` → `- [x] 1.1 Read file` (after completing)

**Completion Protocol (from Rust Task Management):**
1. When you finish a sub-task, mark it `[x]`
2. If all subtasks under a parent are `[x]`:
   - Run `cargo test` (all tests must pass)
   - Run `cargo fmt --check` (code must be formatted)
   - Run `cargo clippy` (no warnings allowed)
   - Clean up temporary code, debug prints
   - Stage changes: `git add .`
   - Commit with conventional format using `-m` flags:
     ```bash
     git commit -m "feat: [parent task summary]" -m "- [key change 1]" -m "- [key change 2]" -m "Related to Epic 22 US-22.X"
     ```
3. Mark parent task `[x]`
4. Stop and wait for user go-ahead before next major task

Update the file after completing each sub-task, not just after completing parent tasks.

---

## Tasks

### Phase 1: Foundation (Days 1-3)

- [x] 0.0 Create feature branch
  - [x] 0.1 Create and checkout new branch: `git checkout -b feature/epic-22-battalion-commander-hardening`
  - [x] 0.2 Verify branch created: `git branch --show-current`

- [x] 1.0 Define Paladin Registry trait and implementation (US-22.1 Foundation)
  - [x] 1.1 Create `src/application/ports/output/paladin_registry.rs`
  - [x] 1.2 Define `PaladinRegistry` trait with `Send + Sync` bounds
  - [x] 1.3 Add trait methods: `register()`, `get()`, `contains()`, `list_ids()`
  - [x] 1.4 Define `RegistryError` enum with `DuplicateId` and `InvalidId` variants
  - [x] 1.5 Add module documentation explaining registry purpose and usage
  - [x] 1.6 Export `PaladinRegistry` trait from `src/application/ports/output/mod.rs`
  - [x] 1.7 Write unit test: test trait is object-safe (compiles with `dyn` usage)
  - [x] 1.8 Create `src/infrastructure/adapters/paladin_registry.rs`
  - [x] 1.9 Implement `HashMapPaladinRegistry` struct with `RwLock<HashMap<String, Arc<Paladin>>>`
  - [x] 1.10 Implement `new()` constructor
  - [x] 1.11 Implement `PaladinRegistry` trait for `HashMapPaladinRegistry`
  - [x] 1.12 Add thread-safe register method (check for duplicates)
  - [x] 1.13 Add thread-safe get method with `Arc::clone()` for shared ownership
  - [x] 1.14 Add thread-safe contains method (read lock only)
  - [x] 1.15 Add thread-safe list_ids method
  - [x] 1.16 Export `HashMapPaladinRegistry` from `src/infrastructure/adapters/mod.rs`
  - [x] 1.17 Write unit test: test_registry_register_and_get
  - [x] 1.18 Write unit test: test_registry_duplicate_id_error
  - [x] 1.19 Write unit test: test_registry_contains
  - [x] 1.20 Write unit test: test_registry_list_ids
  - [x] 1.21 Write unit test: test_registry_thread_safety (spawn threads, concurrent access)
  - [x] 1.22 Run tests: `cargo test paladin_registry`
  - [x] 1.23 Run clippy: `cargo clippy -- -D warnings`
  - [x] 1.24 Format code: `cargo fmt`

- [ ] 2.0 Extend error types for Battalion operations
  - [ ] 2.1 Open `src/core/platform/container/battalion/error.rs`
  - [ ] 2.2 Add `PaladinNotFound(String)` variant to `BattalionError`
  - [ ] 2.3 Add `GroveRoutingFailed(String)` variant to `BattalionError`
  - [ ] 2.4 Add `MetadataExportFailed(String)` variant to `BattalionError`
  - [ ] 2.5 Implement `Display` and error messages for new variants
  - [ ] 2.6 Add `From<RegistryError>` conversion for `BattalionError`
  - [ ] 2.7 Write unit test: test error messages format correctly
  - [ ] 2.8 Write unit test: test RegistryError converts to BattalionError
  - [ ] 2.9 Run tests: `cargo test battalion::error`
  - [ ] 2.10 Run clippy: `cargo clippy -- -D warnings`
  - [ ] 2.11 Format code: `cargo fmt`

### Phase 2: Registry Integration & Routing (Days 4-6)

- [ ] 3.0 Integrate Paladin Registry into Council service (US-22.1)
  - [ ] 3.1 Open `src/application/use_cases/battalion/council_service.rs`
  - [ ] 3.2 Write failing test: test_council_resolves_participants (TDD)
  - [ ] 3.3 Write failing test: test_council_paladin_not_found_error (TDD)
  - [ ] 3.4 Add `registry: Arc<dyn PaladinRegistry>` field to `CouncilService`
  - [ ] 3.5 Update `CouncilService::new()` to accept registry parameter
  - [ ] 3.6 In `execute()` method, locate TODO comment at line 160
  - [ ] 3.7 Replace participant ID storage with registry resolution logic
  - [ ] 3.8 Before discussion rounds, resolve all participant IDs using `registry.get()`
  - [ ] 3.9 Return `BattalionError::PaladinNotFound` if any participant missing
  - [ ] 3.10 Update discussion loop to use resolved `Arc<Paladin>` instances
  - [ ] 3.11 Run test: `cargo test test_council_resolves_participants`
  - [ ] 3.12 Run test: `cargo test test_council_paladin_not_found_error`
  - [ ] 3.13 Write integration test: test_council_full_execution_with_registry
  - [ ] 3.14 Run integration test: `cargo test test_council_full_execution_with_registry`
  - [ ] 3.15 Run all Council tests: `cargo test council_service`
  - [ ] 3.16 Run clippy: `cargo clippy -- -D warnings`
  - [ ] 3.17 Format code: `cargo fmt`

- [ ] 4.0 Integrate Paladin Registry into Grove service (US-22.1)
  - [ ] 4.1 Open `src/application/use_cases/battalion/grove_service.rs`
  - [ ] 4.2 Write failing test: test_grove_resolves_routed_agent (TDD)
  - [ ] 4.3 Write failing test: test_grove_paladin_not_found_error (TDD)
  - [ ] 4.4 Add `registry: Arc<dyn PaladinRegistry>` field to `GroveService`
  - [ ] 4.5 Update `GroveService::new()` to accept registry parameter
  - [ ] 4.6 In `execute()` method, after routing decision is made
  - [ ] 4.7 Add registry resolution: resolve agent ID to `Arc<Paladin>`
  - [ ] 4.8 Return `BattalionError::PaladinNotFound` if agent ID not in registry
  - [ ] 4.9 Execute resolved Paladin instance
  - [ ] 4.10 Run test: `cargo test test_grove_resolves_routed_agent`
  - [ ] 4.11 Run test: `cargo test test_grove_paladin_not_found_error`
  - [ ] 4.12 Write integration test: test_grove_full_execution_with_registry
  - [ ] 4.13 Run integration test: `cargo test test_grove_full_execution_with_registry`
  - [ ] 4.14 Update Commander at line 562 to populate registry for Council
  - [ ] 4.15 Update Commander at line 617 to populate registry for Grove
  - [ ] 4.16 Write test: test_commander_populates_registry_for_council
  - [ ] 4.17 Write test: test_commander_populates_registry_for_grove
  - [ ] 4.18 Run all Grove tests: `cargo test grove_service`
  - [ ] 4.19 Run clippy: `cargo clippy -- -D warnings`
  - [ ] 4.20 Format code: `cargo fmt`

- [ ] 5.0 Implement Grove LLM-based routing (US-22.2)
  - [ ] 5.1 Open `src/core/platform/container/battalion/grove.rs`
  - [ ] 5.2 Add `routing_fallback: String` field to `GroveConfig` (default: "keyword")
  - [ ] 5.3 Add `min_confidence: f32` field to `GroveConfig` (default: 0.5)
  - [ ] 5.4 Add validation method: validate `routing_fallback` is "keyword" or "error"
  - [ ] 5.5 Add validation method: validate `min_confidence` in range [0.0, 1.0]
  - [ ] 5.6 Write test: test_grove_config_validation
  - [ ] 5.7 Open `src/application/use_cases/battalion/grove_service.rs`
  - [ ] 5.8 Locate TODO/unimplemented!() at line 475
  - [ ] 5.9 Write failing test: test_route_with_llm_successful (TDD)
  - [ ] 5.10 Write failing test: test_route_with_llm_low_confidence (TDD)
  - [ ] 5.11 Write failing test: test_route_with_llm_invalid_json (TDD)
  - [ ] 5.12 Write failing test: test_route_with_llm_fallback_to_keyword (TDD)
  - [ ] 5.13 Create `route_with_llm()` method signature
  - [ ] 5.14 Build routing prompt with: user input, agent descriptions, JSON format instruction
  - [ ] 5.15 Call LLM via `self.llm_port.generate()`
  - [ ] 5.16 Parse JSON response using `serde_json::from_str()`
  - [ ] 5.17 Define `RoutingResponse` struct: tree_name, agent_id, confidence, reasoning
  - [ ] 5.18 Validate confidence >= min_confidence threshold
  - [ ] 5.19 Validate agent_id exists in Grove config
  - [ ] 5.20 If validation passes, return routed agent ID with reasoning
  - [ ] 5.21 If validation fails, check `routing_fallback` config
  - [ ] 5.22 If fallback == "keyword", call existing keyword routing method
  - [ ] 5.23 If fallback == "error", return `GroveRoutingFailed` error
  - [ ] 5.24 Add logging: log routing decisions with reasoning
  - [ ] 5.25 Handle LLM API errors with proper error propagation
  - [ ] 5.26 Run test: `cargo test test_route_with_llm_successful`
  - [ ] 5.27 Run test: `cargo test test_route_with_llm_low_confidence`
  - [ ] 5.28 Run test: `cargo test test_route_with_llm_invalid_json`
  - [ ] 5.29 Run test: `cargo test test_route_with_llm_fallback_to_keyword`
  - [ ] 5.30 Write integration test: test_grove_llm_routing_end_to_end
  - [ ] 5.31 Run integration test
  - [ ] 5.32 Run all Grove tests: `cargo test grove`
  - [ ] 5.33 Run clippy: `cargo clippy -- -D warnings`
  - [ ] 5.34 Format code: `cargo fmt`

### Phase 3: Metrics & Export (Days 7-8)

- [ ] 6.0 Extend BattalionMetadata for enhanced metrics (US-22.3)
  - [ ] 6.1 Open `src/core/platform/container/battalion/battalion_result.rs`
  - [ ] 6.2 Define `TokenUsage` struct with fields: prompt_tokens, completion_tokens, total_tokens
  - [ ] 6.3 Derive `Debug, Clone, Serialize, Deserialize` for `TokenUsage`
  - [ ] 6.4 Add `per_paladin_times: HashMap<String, Duration>` to `BattalionMetadata`
  - [ ] 6.5 Add `per_paladin_tokens: HashMap<String, TokenUsage>` to `BattalionMetadata`
  - [ ] 6.6 Add `total_tokens: u64` to `BattalionMetadata`
  - [ ] 6.7 Update `BattalionMetadata::new()` to initialize new fields as empty/zero
  - [ ] 6.8 Ensure all fields derive `Serialize, Deserialize` for JSON export
  - [ ] 6.9 Write test: test_battalion_metadata_serialization
  - [ ] 6.10 Write test: test_token_usage_aggregation_calculation
  - [ ] 6.11 Run tests: `cargo test battalion_metadata`
  - [ ] 6.12 Run clippy: `cargo clippy -- -D warnings`
  - [ ] 6.13 Format code: `cargo fmt`

- [ ] 7.0 Implement Phalanx per-paladin metrics collection (US-22.3)
  - [ ] 7.1 Open `src/application/use_cases/battalion/phalanx_service.rs`
  - [ ] 7.2 Locate TODO comment at line 270
  - [ ] 7.3 Write failing test: test_phalanx_per_paladin_timing (TDD)
  - [ ] 7.4 Write failing test: test_phalanx_per_paladin_tokens (TDD)
  - [ ] 7.5 Write failing test: test_phalanx_metrics_with_partial_failures (TDD)
  - [ ] 7.6 In `execute()` method's parallel execution loop
  - [ ] 7.7 Record start time: `let start = Instant::now()` before each Paladin execution
  - [ ] 7.8 Execute Paladin and capture result
  - [ ] 7.9 Record end time and calculate duration: `start.elapsed()`
  - [ ] 7.10 Store duration in HashMap: `per_paladin_times.insert(paladin_id, duration)`
  - [ ] 7.11 Extract token usage from `PaladinResult::metadata` if available
  - [ ] 7.12 Parse token usage into `TokenUsage` struct
  - [ ] 7.13 Store in HashMap: `per_paladin_tokens.insert(paladin_id, token_usage)`
  - [ ] 7.14 Track success/failure counts during execution
  - [ ] 7.15 Calculate `paladin_success_count` from successful results
  - [ ] 7.16 Calculate `paladin_failure_count` from error results
  - [ ] 7.17 Calculate `total_tokens` as sum of all token_usage.total_tokens
  - [ ] 7.18 Populate `metadata.per_paladin_times` with collected HashMap
  - [ ] 7.19 Populate `metadata.per_paladin_tokens` with collected HashMap
  - [ ] 7.20 Populate `metadata.total_tokens` with calculated sum
  - [ ] 7.21 Ensure metrics survive error conditions (use `unwrap_or_default()`)
  - [ ] 7.22 Run test: `cargo test test_phalanx_per_paladin_timing`
  - [ ] 7.23 Run test: `cargo test test_phalanx_per_paladin_tokens`
  - [ ] 7.24 Run test: `cargo test test_phalanx_metrics_with_partial_failures`
  - [ ] 7.25 Run all Phalanx tests: `cargo test phalanx`
  - [ ] 7.26 Run clippy: `cargo clippy -- -D warnings`
  - [ ] 7.27 Format code: `cargo fmt`

- [ ] 8.0 Add Commander metadata export configuration (US-22.4)
  - [ ] 8.1 Open `src/core/platform/container/battalion/commander_config.rs`
  - [ ] 8.2 Add `metadata_output_dir: Option<PathBuf>` field to `CommanderConfig`
  - [ ] 8.3 Update `CommanderConfig::default()` to set `metadata_output_dir: None`
  - [ ] 8.4 Add validation method: validate path is writable if Some
  - [ ] 8.5 Update config deserialization to handle optional PathBuf
  - [ ] 8.6 Write test: test_commander_config_with_metadata_dir
  - [ ] 8.7 Write test: test_commander_config_without_metadata_dir
  - [ ] 8.8 Update `config.yml` or `config.test.yml` with example configuration
  - [ ] 8.9 Run tests: `cargo test commander_config`
  - [ ] 8.10 Run clippy: `cargo clippy -- -D warnings`
  - [ ] 8.11 Format code: `cargo fmt`

- [ ] 9.0 Implement Commander metadata export logic (US-22.4)
  - [ ] 9.1 Open `src/application/use_cases/battalion/commander.rs`
  - [ ] 9.2 Write failing test: test_metadata_export_creates_file (TDD, use temp dir)
  - [ ] 9.3 Write failing test: test_metadata_export_correct_naming (TDD)
  - [ ] 9.4 Write failing test: test_metadata_export_json_structure (TDD)
  - [ ] 9.5 Write failing test: test_metadata_export_error_handling (TDD)
  - [ ] 9.6 Add helper method: `export_metadata(metadata: &BattalionMetadata, strategy: &str, config: &CommanderConfig)`
  - [ ] 9.7 Check if `config.metadata_output_dir.is_some()`
  - [ ] 9.8 If None, return early (no export)
  - [ ] 9.9 Generate timestamp: `Local::now().format("%Y%m%d_%H%M%S")`
  - [ ] 9.10 Generate short UUID: `Uuid::new_v4().to_string()[..8]`
  - [ ] 9.11 Build filename: `{strategy}_{timestamp}_{uuid}.json`
  - [ ] 9.12 Build full path: `metadata_output_dir.join(filename)`
  - [ ] 9.13 Create output directory if it doesn't exist: `std::fs::create_dir_all()`
  - [ ] 9.14 Serialize metadata to JSON: `serde_json::to_string_pretty(metadata)`
  - [ ] 9.15 Write JSON to file: `std::fs::write(path, json)`
  - [ ] 9.16 Log success: "Metadata exported to {path}"
  - [ ] 9.17 Handle errors: wrap in match, log errors, return Ok (non-fatal)
  - [ ] 9.18 Call `export_metadata()` after each Battalion execution in Commander
  - [ ] 9.19 Run test: `cargo test test_metadata_export_creates_file`
  - [ ] 9.20 Run test: `cargo test test_metadata_export_correct_naming`
  - [ ] 9.21 Run test: `cargo test test_metadata_export_json_structure`
  - [ ] 9.22 Run test: `cargo test test_metadata_export_error_handling`
  - [ ] 9.23 Write integration test: test_commander_exports_metadata_end_to_end
  - [ ] 9.24 Run integration test
  - [ ] 9.25 Run all Commander tests: `cargo test commander` (will have ignored tests)
  - [ ] 9.26 Run clippy: `cargo clippy -- -D warnings`
  - [ ] 9.27 Format code: `cargo fmt`

### Phase 4: Test Hardening (Days 9-10)

- [ ] 10.0 Create MockLlmAdapter test infrastructure (US-22.5)
  - [ ] 10.1 Check if `tests/helpers/mock_llm_adapter.rs` exists
  - [ ] 10.2 If not, create file and module structure
  - [ ] 10.3 Define `MockLlmAdapter` struct with configurable responses
  - [ ] 10.4 Add field: `responses: Arc<Mutex<VecDeque<Result<String, LlmError>>>>`
  - [ ] 10.5 Add field: `call_count: Arc<Mutex<usize>>`
  - [ ] 10.6 Implement `new()` constructor
  - [ ] 10.7 Add method: `add_response(response: Result<String, LlmError>)` to queue responses
  - [ ] 10.8 Add method: `add_success(content: impl Into<String>)` helper
  - [ ] 10.9 Add method: `add_failure(error: LlmError)` helper
  - [ ] 10.10 Add method: `call_count() -> usize` to retrieve invocation count
  - [ ] 10.11 Add method: `reset()` to clear state between tests
  - [ ] 10.12 Implement `LlmPort` trait for `MockLlmAdapter`
  - [ ] 10.13 In `generate()`: pop response from queue, increment counter
  - [ ] 10.14 In `generate()`: return default if queue empty
  - [ ] 10.15 Implement `generate_stream()` as unimplemented or simple wrapper
  - [ ] 10.16 Create helper function: `create_test_paladin_with_mock(mock: Arc<MockLlmAdapter>)`
  - [ ] 10.17 Create helper function: `create_mock_with_responses(responses: Vec<&str>)`
  - [ ] 10.18 Write test: test_mock_llm_adapter_returns_configured_responses
  - [ ] 10.19 Write test: test_mock_llm_adapter_tracks_call_count
  - [ ] 10.20 Write test: test_mock_llm_adapter_handles_failures
  - [ ] 10.21 Export MockLlmAdapter from `tests/helpers/mod.rs`
  - [ ] 10.22 Run tests: `cargo test mock_llm_adapter`
  - [ ] 10.23 Run clippy: `cargo clippy --tests -- -D warnings`
  - [ ] 10.24 Format code: `cargo fmt`

- [ ] 11.0 Enable and fix Campaign and ChainOfCommand tests (US-22.5 Phase 1)
  - [ ] 11.1 Open `src/application/use_cases/battalion/commander.rs` test module
  - [ ] 11.2 Locate `#[ignore]` attribute on `test_execute_campaign` (line ~1850)
  - [ ] 11.3 Remove `#[ignore]` attribute
  - [ ] 11.4 Run test to see current failure: `cargo test test_execute_campaign`
  - [ ] 11.5 Update test to use MockLlmAdapter instead of real LLM
  - [ ] 11.6 Create 4+ test Paladins with mock LLM for DAG nodes
  - [ ] 11.7 Configure mock responses for each node
  - [ ] 11.8 Define Campaign DAG with dependencies: A → B, A → C, B+C → D
  - [ ] 11.9 Execute Campaign via Commander
  - [ ] 11.10 Verify execution order respects DAG dependencies
  - [ ] 11.11 Verify all node results collected in final result
  - [ ] 11.12 Verify metadata shows correct execution sequence
  - [ ] 11.13 Run test: `cargo test test_execute_campaign` (should pass)
  - [ ] 11.14 Locate `#[ignore]` on `test_execute_chain_of_command` (line ~1875)
  - [ ] 11.15 Remove `#[ignore]` attribute
  - [ ] 11.16 Run test to see current failure: `cargo test test_execute_chain_of_command`
  - [ ] 11.17 Update test to use MockLlmAdapter
  - [ ] 11.18 Create supervisor Paladin and 2 worker Paladins with mocks
  - [ ] 11.19 Configure Chain of Command hierarchy in config
  - [ ] 11.20 Configure mock responses for supervisor and workers
  - [ ] 11.21 Execute Chain of Command via Commander
  - [ ] 11.22 Verify supervisor delegates to workers
  - [ ] 11.23 Verify worker results aggregated correctly
  - [ ] 11.24 Verify delegation flow in metadata
  - [ ] 11.25 Run test: `cargo test test_execute_chain_of_command` (should pass)
  - [ ] 11.26 Run both tests together: `cargo test test_execute_campaign test_execute_chain_of_command`
  - [ ] 11.27 Run clippy: `cargo clippy --tests -- -D warnings`
  - [ ] 11.28 Format code: `cargo fmt`

- [ ] 12.0 Enable and fix error handling tests (US-22.5 Phase 2)
  - [ ] 12.1 Open `src/application/use_cases/battalion/commander.rs` test module
  - [ ] 12.2 Locate `test_error_handling_fail_fast` (line ~2017)
  - [ ] 12.3 Remove `#[ignore]` attribute
  - [ ] 12.4 Run test to see current state: `cargo test test_error_handling_fail_fast`
  - [ ] 12.5 Update test to use MockLlmAdapter with failure simulation
  - [ ] 12.6 Create Formation with 3 Paladins, second one configured to fail
  - [ ] 12.7 Set `continue_on_error: false` in config
  - [ ] 12.8 Execute Formation via Commander
  - [ ] 12.9 Verify execution stops after first failure
  - [ ] 12.10 Verify error propagated in result
  - [ ] 12.11 Verify remaining Paladins not executed
  - [ ] 12.12 Run test: `cargo test test_error_handling_fail_fast` (should pass)
  - [ ] 12.13 Locate `test_error_handling_continue_on_error` (line ~2025)
  - [ ] 12.14 Remove `#[ignore]` attribute
  - [ ] 12.15 Run test: `cargo test test_error_handling_continue_on_error`
  - [ ] 12.16 Update test with MockLlmAdapter with failure in middle
  - [ ] 12.17 Set `continue_on_error: true` in config
  - [ ] 12.18 Verify all Paladins execute despite failure
  - [ ] 12.19 Verify partial results returned with failure details
  - [ ] 12.20 Run test: `cargo test test_error_handling_continue_on_error` (should pass)
  - [ ] 12.21 Locate `test_error_handling_retry_then_continue` (line ~2033)
  - [ ] 12.22 Remove `#[ignore]` attribute
  - [ ] 12.23 Run test: `cargo test test_error_handling_retry_then_continue`
  - [ ] 12.24 Update test with MockLlmAdapter: fail twice, then succeed
  - [ ] 12.25 Configure retry policy: max_retries: 2
  - [ ] 12.26 Verify 3 total attempts made (original + 2 retries)
  - [ ] 12.27 Verify execution continues after exhausted retries
  - [ ] 12.28 Run test: `cargo test test_error_handling_retry_then_continue` (should pass)
  - [ ] 12.29 Locate `test_partial_failure_handling` (line ~2041)
  - [ ] 12.30 Remove `#[ignore]` attribute
  - [ ] 12.31 Run test: `cargo test test_partial_failure_handling`
  - [ ] 12.32 Update test for Phalanx with mixed success/failure
  - [ ] 12.33 Configure 4 parallel Paladins: 2 succeed, 2 fail
  - [ ] 12.34 Execute Phalanx
  - [ ] 12.35 Verify successful results preserved in output
  - [ ] 12.36 Verify failure details captured in metadata
  - [ ] 12.37 Verify success_count = 2, failure_count = 2
  - [ ] 12.38 Run test: `cargo test test_partial_failure_handling` (should pass)
  - [ ] 12.39 Run all error handling tests together
  - [ ] 12.40 Run all Commander tests: `cargo test commander` (all should pass now)
  - [ ] 12.41 Run clippy: `cargo clippy --tests -- -D warnings`
  - [ ] 12.42 Format code: `cargo fmt`

- [ ] 13.0 Integration testing and final validation
  - [ ] 13.1 Open or create `tests/integration/battalion_integration_tests.rs`
  - [ ] 13.2 Write integration test: test_council_with_registry_full_flow
  - [ ] 13.3 Write integration test: test_grove_with_llm_routing_full_flow
  - [ ] 13.4 Write integration test: test_phalanx_with_metrics_full_flow
  - [ ] 13.5 Write integration test: test_commander_with_metadata_export_full_flow
  - [ ] 13.6 Write integration test: test_campaign_dag_execution_full_flow
  - [ ] 13.7 Write integration test: test_chain_of_command_delegation_full_flow
  - [ ] 13.8 Write integration test: test_battalion_error_handling_scenarios
  - [ ] 13.9 Run all integration tests: `cargo test --test battalion_integration_tests`
  - [ ] 13.10 Run full test suite: `cargo test`
  - [ ] 13.11 Verify no test failures
  - [ ] 13.12 Verify no ignored tests remain (except pre-existing from excluded scope)
  - [ ] 13.13 Run test with coverage: `cargo tarpaulin --out Html` (if available)
  - [ ] 13.14 Verify ≥80% coverage for new code
  - [ ] 13.15 Run clippy on entire project: `cargo clippy --all-targets -- -D warnings`
  - [ ] 13.16 Format entire project: `cargo fmt --all`
  - [ ] 13.17 Run audit: `cargo audit` (if available)
  - [ ] 13.18 Build release: `cargo build --release`
  - [ ] 13.19 Verify no build warnings

- [ ] 14.0 Update documentation and examples
  - [ ] 14.1 Open `docs/BATTALION.md`
  - [ ] 14.2 Update Formation section with metadata export information
  - [ ] 14.3 Update Phalanx section with per-paladin metrics information
  - [ ] 14.4 Update Council section with registry usage
  - [ ] 14.5 Update Grove section with LLM routing and fallback configuration
  - [ ] 14.6 Add section: "Paladin Registry" with trait description and usage
  - [ ] 14.7 Add section: "Commander Metadata Export" with configuration examples
  - [ ] 14.8 Add section: "Performance Metrics" explaining new timing and token metrics
  - [ ] 14.9 Update configuration reference with new fields
  - [ ] 14.10 Open `docs/COMMANDER.md` or create if not exists
  - [ ] 14.11 Document Commander metadata export feature
  - [ ] 14.12 Add YAML configuration example for metadata_output_dir
  - [ ] 14.13 Document metadata JSON structure
  - [ ] 14.14 Add troubleshooting section for common issues
  - [ ] 14.15 Create or update `examples/commander_with_metadata_export.rs`
  - [ ] 14.16 Create or update `examples/grove_llm_routing.rs`
  - [ ] 14.17 Create or update `examples/phalanx_with_metrics.rs`
  - [ ] 14.18 Create or update `examples/council_with_registry.rs`
  - [ ] 14.19 Test all example files compile: `cargo build --examples`
  - [ ] 14.20 Run example: `cargo run --example commander_with_metadata_export`
  - [ ] 14.21 Verify example works correctly
  - [ ] 14.22 Update CHANGELOG.md with Epic 22 changes
  - [ ] 14.23 Add entry for Paladin Registry feature
  - [ ] 14.24 Add entry for Grove LLM routing
  - [ ] 14.25 Add entry for enhanced Phalanx metrics
  - [ ] 14.26 Add entry for Commander metadata export
  - [ ] 14.27 Add entry for test hardening (Campaign, ChainOfCommand, error handling)
  - [ ] 14.28 Update README.md if necessary
  - [ ] 14.29 Run doc tests: `cargo test --doc`
  - [ ] 14.30 Generate docs: `cargo doc --no-deps --open`
  - [ ] 14.31 Review generated documentation for completeness

---

**Status:** Sub-tasks generated - Ready for implementation

**Next Step:** Begin implementation with Task 0.0 (Create feature branch). After completing all sub-tasks for a parent task, follow the completion protocol: test, format, lint, commit. Stop after each parent task for user go-ahead.
