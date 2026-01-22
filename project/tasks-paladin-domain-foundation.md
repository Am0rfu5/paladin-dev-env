# Task List: Paladin Domain Foundation (Epic 1)

**Based on:** PRD - Paladin Domain Foundation  
**Epic:** Epic 1  
**Priority:** Critical  
**Target Milestone:** M1 - Alpha (Week 6)  
**Estimated Effort:** 3-4 weeks

---

## Instructions for Completing Tasks

**IMPORTANT:** As you complete each task, you must check it off in this markdown file by changing `- [ ]` to `- [x]`. This helps track progress and ensures you don't skip any steps.

Example:
- `- [ ] 1.1 Read file` → `- [x] 1.1 Read file` (after completing)

Update the file after completing each sub-task, not just after completing an entire parent task.

---

## Relevant Files

### Core Domain Layer
- `src/core/platform/container/paladin.rs` - Paladin domain entity using Node<T> pattern
- `src/core/platform/container/paladin_config.rs` - Configuration types for Paladin execution
- `src/core/platform/container/mod.rs` - Module exports for container types

### Application Layer
- `src/application/ports/output/paladin_port.rs` - Port trait for Paladin execution abstraction
- `src/application/ports/output/mod.rs` - Module exports for output ports
- `src/application/use_cases/paladin/mod.rs` - Paladin use case module
- `src/application/use_cases/paladin/paladin_builder.rs` - Fluent builder for Paladin construction
- `src/application/use_cases/paladin/paladin_execution_service.rs` - Main execution service with retry/circuit breaker
- `src/application/use_cases/paladin/circuit_breaker.rs` - Circuit breaker implementation
- `src/application/use_cases/paladin/error.rs` - PaladinError types using thiserror
- `src/application/use_cases/mod.rs` - Module exports for use cases

### Infrastructure Layer
- `src/infrastructure/adapters/llm/mock_llm_adapter.rs` - Mock LLM adapter for testing
- `src/infrastructure/adapters/llm/mod.rs` - Module exports for LLM adapters

### Test Files
- `tests/unit/paladin_entity_test.rs` - Unit tests for Paladin domain entity
- `tests/unit/paladin_config_test.rs` - Unit tests for PaladinConfig
- `tests/unit/paladin_builder_test.rs` - Unit tests for PaladinBuilder
- `tests/unit/circuit_breaker_test.rs` - Unit tests for circuit breaker
- `tests/unit/paladin_execution_service_test.rs` - Unit tests for execution service
- `tests/integration/paladin_integration_test.rs` - Integration tests for complete Paladin flow

### Example Files
- `examples/basic_paladin.rs` - Basic Paladin creation and execution example
- `examples/paladin_with_config.rs` - Advanced configuration example

### Notes

- Unit tests should be in `tests/unit/` directory following Rust conventions
- Integration tests should be in `tests/integration/` directory
- Use `cargo test` to run all tests
- Use `cargo test [test_name]` to run specific tests
- Use `cargo test --test [test_file]` to run a specific test file
- Follow TDD: Write failing test first (Red), implement minimal code (Green), refactor
- All public APIs must have rustdoc comments with examples
- Run `cargo clippy` to check for lints
- Run `cargo fmt` to format code

---

## Tasks

- [x] 0.0 Create feature branch
  - [x] 0.1 Verify feature branch `feature/epic_1-paladin-domain-foundation` is checked out (already created)
  - [x] 0.2 Ensure branch is up to date with develop branch

- [x] 1.0 Implement Core Domain Layer (Paladin Entity & Configuration)
  - [x] 1.1 Read existing `src/core/base/entity/node.rs` to understand Node<T> pattern
  - [x] 1.2 Create `src/core/platform/container/paladin.rs`
  - [x] 1.3 Define `PaladinStatus` enum (Idle, Reasoning, Executing, Completed, Failed)
  - [x] 1.4 Define `PaladinData` struct with all required fields (system_prompt, name, user_name, model, temperature, max_loops, stop_words, status)
  - [x] 1.5 Implement `Serialize` and `Deserialize` derives for PaladinData
  - [x] 1.6 Create type alias `pub type Paladin = Node<PaladinData>`
  - [x] 1.7 Create `src/core/platform/container/paladin_config.rs`
  - [x] 1.8 Define `OutputFormat` enum (Text, Json, Structured)
  - [x] 1.9 Define `PaladinConfig` struct with Builder derive (retry_attempts, timeout_seconds, enable_planning, planning_prompt, output_format)
  - [x] 1.10 Add default implementations for PaladinConfig fields
  - [x] 1.11 Update `src/core/platform/container/mod.rs` to export paladin and paladin_config modules
  - [x] 1.12 Write unit test: `test_paladin_status_transitions` in `tests/unit/paladin_entity_test.rs`
  - [x] 1.13 Write unit test: `test_paladin_data_serialization_roundtrip`
  - [x] 1.14 Write unit test: `test_paladin_data_clone`
  - [x] 1.15 Write unit test: `test_paladin_config_defaults` in `tests/unit/paladin_config_test.rs`
  - [x] 1.16 Write unit test: `test_paladin_config_builder`
  - [x] 1.17 Run tests: `cargo test paladin_entity` and `cargo test paladin_config`
  - [x] 1.18 Verify all tests pass

- [ ] 2.0 Implement Application Layer Ports
  - [ ] 2.1 Create `src/application/use_cases/paladin/error.rs`
  - [ ] 2.2 Define `PaladinError` enum using thiserror (ConfigurationError, ExecutionError, LlmError, Timeout, StopWordDetected)
  - [ ] 2.3 Implement Display and descriptive error messages for all variants
  - [ ] 2.4 Write unit test: `test_paladin_error_messages` in `tests/unit/paladin_error_test.rs`
  - [ ] 2.5 Create `src/application/ports/output/paladin_port.rs`
  - [ ] 2.6 Define `PaladinResult` struct (output: String, token_count: u32, execution_time_ms: u64, loop_count: u32, stop_reason: StopReason)
  - [ ] 2.7 Define `StopReason` enum (MaxLoops, StopWord, Completed, Timeout)
  - [ ] 2.8 Define `PaladinStream` type for streaming (use tokio::sync::mpsc or similar)
  - [ ] 2.9 Define `PaladinPort` trait with async methods: execute, execute_stream, validate
  - [ ] 2.10 Add trait bounds: Send + Sync for async compatibility
  - [ ] 2.11 Add comprehensive rustdoc comments to PaladinPort trait
  - [ ] 2.12 Update `src/application/ports/output/mod.rs` to export paladin_port
  - [ ] 2.13 Update `src/application/use_cases/paladin/mod.rs` to export error module
  - [ ] 2.14 Run `cargo check` to verify compilation

- [ ] 3.0 Implement PaladinBuilder with Validation (TDD)
  - [ ] 3.1 Write failing test: `test_paladin_builder_creates_valid_paladin` in `tests/unit/paladin_builder_test.rs`
  - [ ] 3.2 Write failing test: `test_paladin_builder_validates_required_fields`
  - [ ] 3.3 Write failing test: `test_paladin_builder_rejects_invalid_temperature`
  - [ ] 3.4 Write failing test: `test_paladin_builder_rejects_invalid_max_loops`
  - [ ] 3.5 Write failing test: `test_paladin_builder_sets_defaults`
  - [ ] 3.6 Write failing test: `test_paladin_builder_method_chaining`
  - [ ] 3.7 Run tests to confirm they fail: `cargo test paladin_builder`
  - [ ] 3.8 Create `src/application/use_cases/paladin/paladin_builder.rs`
  - [ ] 3.9 Define `PaladinBuilder` struct with fields (llm_port: Arc<dyn LlmPort>, data: PaladinData, config: PaladinConfig)
  - [ ] 3.10 Implement `new(llm_port: Arc<dyn LlmPort>)` constructor with defaults
  - [ ] 3.11 Implement fluent setter methods: system_prompt, name, user_name, model, temperature, max_loops, add_stop_word
  - [ ] 3.12 Implement config methods: retry_attempts, timeout_seconds, enable_planning, output_format
  - [ ] 3.13 Implement `validate()` private method checking all constraints
  - [ ] 3.14 Validate system_prompt is non-empty
  - [ ] 3.15 Validate temperature is in [0.0, 1.0]
  - [ ] 3.16 Validate max_loops is in [1, 100]
  - [ ] 3.17 Implement `build() -> Result<Paladin, PaladinError>` method
  - [ ] 3.18 Add rustdoc comments with usage examples
  - [ ] 3.19 Update `src/application/use_cases/paladin/mod.rs` to export paladin_builder
  - [ ] 3.20 Run tests: `cargo test paladin_builder`
  - [ ] 3.21 Verify all tests pass (Green)
  - [ ] 3.22 Refactor builder code for clarity if needed
  - [ ] 3.23 Re-run tests to ensure refactoring didn't break anything
  - [ ] 3.24 Run `cargo clippy` on builder module

- [ ] 4.0 Implement Circuit Breaker Pattern
  - [ ] 4.1 Write failing test: `test_circuit_breaker_closed_state` in `tests/unit/circuit_breaker_test.rs`
  - [ ] 4.2 Write failing test: `test_circuit_breaker_opens_after_threshold_failures`
  - [ ] 4.3 Write failing test: `test_circuit_breaker_half_open_state`
  - [ ] 4.4 Write failing test: `test_circuit_breaker_closes_after_success`
  - [ ] 4.5 Write failing test: `test_circuit_breaker_concurrent_access`
  - [ ] 4.6 Run tests to confirm they fail: `cargo test circuit_breaker`
  - [ ] 4.7 Create `src/application/use_cases/paladin/circuit_breaker.rs`
  - [ ] 4.8 Define `CircuitState` enum (Closed, Open { opened_at: Instant }, HalfOpen { successes: u32 })
  - [ ] 4.9 Define `CircuitBreaker` struct with fields (state: RwLock<CircuitState>, failure_threshold, success_threshold, timeout)
  - [ ] 4.10 Implement `new()` constructor with configurable thresholds
  - [ ] 4.11 Implement `call<F, T>(&self, f: F) -> Result<T, PaladinError>` method for wrapping operations
  - [ ] 4.12 Implement state transition logic: Closed -> Open on threshold failures
  - [ ] 4.13 Implement state transition logic: Open -> HalfOpen after timeout
  - [ ] 4.14 Implement state transition logic: HalfOpen -> Closed on success threshold
  - [ ] 4.15 Implement state transition logic: HalfOpen -> Open on failure
  - [ ] 4.16 Add logging for all state transitions using `tracing` crate
  - [ ] 4.17 Add rustdoc comments explaining circuit breaker pattern
  - [ ] 4.18 Update `src/application/use_cases/paladin/mod.rs` to export circuit_breaker
  - [ ] 4.19 Run tests: `cargo test circuit_breaker`
  - [ ] 4.20 Verify all tests pass (Green)
  - [ ] 4.21 Refactor if needed
  - [ ] 4.22 Run `cargo clippy` on circuit_breaker module

- [ ] 5.0 Implement PaladinExecutionService with Retry Logic (TDD)
  - [ ] 5.1 Write failing test: `test_execution_service_executes_successfully` in `tests/unit/paladin_execution_service_test.rs`
  - [ ] 5.2 Write failing test: `test_execution_service_respects_max_loops`
  - [ ] 5.3 Write failing test: `test_execution_service_detects_stop_words`
  - [ ] 5.4 Write failing test: `test_execution_service_enforces_timeout`
  - [ ] 5.5 Write failing test: `test_execution_service_retries_on_failure`
  - [ ] 5.6 Write failing test: `test_execution_service_exponential_backoff`
  - [ ] 5.7 Write failing test: `test_execution_service_uses_circuit_breaker`
  - [ ] 5.8 Write failing test: `test_execution_service_tracks_metadata`
  - [ ] 5.9 Run tests to confirm they fail: `cargo test paladin_execution_service`
  - [ ] 5.10 Create `src/application/use_cases/paladin/paladin_execution_service.rs`
  - [ ] 5.11 Define `PaladinExecutionService` struct with fields (llm_port: Arc<dyn LlmPort>, circuit_breaker: Arc<CircuitBreaker>)
  - [ ] 5.12 Implement `new()` constructor
  - [ ] 5.13 Implement private `build_prompt()` method combining config and input
  - [ ] 5.14 Implement private `check_stop_words()` method (case-insensitive, exact match)
  - [ ] 5.15 Implement private `execute_with_retry()` method with exponential backoff (100ms, 200ms, 400ms)
  - [ ] 5.16 Implement main reasoning loop in private `execute_loop()` method
  - [ ] 5.17 Add loop counter and max_loops enforcement
  - [ ] 5.18 Add stop word checking after each LLM call
  - [ ] 5.19 Implement `execute(&self, paladin: &Paladin, input: &str) -> Result<PaladinResult, PaladinError>` public method
  - [ ] 5.20 Wrap LLM calls with circuit breaker
  - [ ] 5.21 Wrap execution with `tokio::time::timeout` using config timeout_seconds
  - [ ] 5.22 Track execution metadata (start time, loop count, token count)
  - [ ] 5.23 Add structured logging with `tracing` (execution_id, loop iterations, stop reason)
  - [ ] 5.24 Log at appropriate levels (info for start/complete, debug for loops, warn for retries, error for failures)
  - [ ] 5.25 Implement `validate()` method checking Paladin configuration
  - [ ] 5.26 Add comprehensive rustdoc comments with usage examples
  - [ ] 5.27 Update `src/application/use_cases/paladin/mod.rs` to export paladin_execution_service
  - [ ] 5.28 Run tests: `cargo test paladin_execution_service`
  - [ ] 5.29 Verify all tests pass (Green)
  - [ ] 5.30 Refactor for clarity and eliminate duplication
  - [ ] 5.31 Re-run tests after refactoring
  - [ ] 5.32 Run `cargo clippy` on execution service module

- [ ] 6.0 Create Mock LLM Adapter for Testing
  - [ ] 6.1 Read existing `src/application/ports/output/llm_port.rs` to understand LlmPort trait
  - [ ] 6.2 Write failing test: `test_mock_llm_returns_configured_response` in `tests/unit/mock_llm_adapter_test.rs`
  - [ ] 6.3 Write failing test: `test_mock_llm_simulates_delay`
  - [ ] 6.4 Write failing test: `test_mock_llm_simulates_failure`
  - [ ] 6.5 Write failing test: `test_mock_llm_tracks_call_history`
  - [ ] 6.6 Write failing test: `test_mock_llm_configurable_token_count`
  - [ ] 6.7 Run tests to confirm they fail: `cargo test mock_llm`
  - [ ] 6.8 Create `src/infrastructure/adapters/llm/mock_llm_adapter.rs`
  - [ ] 6.9 Define `MockLlmAdapter` struct with fields (responses: Vec<String>, delays: Vec<Duration>, failures: Vec<bool>, call_history: Arc<Mutex<Vec<String>>>)
  - [ ] 6.10 Implement builder methods: with_response, with_delay, with_failure, with_token_count
  - [ ] 6.11 Implement `LlmPort` trait for MockLlmAdapter
  - [ ] 6.12 Implement async `generate()` method returning configured responses
  - [ ] 6.13 Add delay simulation using `tokio::time::sleep`
  - [ ] 6.14 Add failure simulation returning LlmError
  - [ ] 6.15 Track all calls in call_history for test assertions
  - [ ] 6.16 Implement method to retrieve call history for assertions
  - [ ] 6.17 Add rustdoc comments explaining mock usage
  - [ ] 6.18 Update `src/infrastructure/adapters/llm/mod.rs` to export mock_llm_adapter
  - [ ] 6.19 Run tests: `cargo test mock_llm`
  - [ ] 6.20 Verify all tests pass (Green)
  - [ ] 6.21 Refactor if needed
  - [ ] 6.22 Run `cargo clippy` on mock adapter module

- [ ] 7.0 Implement Integration Tests
  - [ ] 7.1 Create `tests/integration/paladin_integration_test.rs`
  - [ ] 7.2 Write test: `test_paladin_end_to_end_execution` using MockLlmAdapter
  - [ ] 7.3 Test complete flow: build Paladin -> execute -> verify result
  - [ ] 7.4 Write test: `test_paladin_with_stop_word` verifying early termination
  - [ ] 7.5 Write test: `test_paladin_max_loops_enforcement` verifying loop limit
  - [ ] 7.6 Write test: `test_paladin_timeout_enforcement` using slow mock
  - [ ] 7.7 Write test: `test_paladin_retry_on_transient_failure` with intermittent failures
  - [ ] 7.8 Write test: `test_paladin_circuit_breaker_opens` with sustained failures
  - [ ] 7.9 Write test: `test_paladin_invalid_configuration` verifying validation
  - [ ] 7.10 Write test: `test_paladin_builder_validation_errors` checking error messages
  - [ ] 7.11 Run integration tests: `cargo test --test paladin_integration_test`
  - [ ] 7.12 Verify all integration tests pass
  - [ ] 7.13 Check test coverage: `cargo tarpaulin` or `cargo llvm-cov` (if available)
  - [ ] 7.14 Ensure unit test coverage ≥ 80%
  - [ ] 7.15 Ensure integration test coverage ≥ 70%

- [ ] 8.0 Documentation and Examples
  - [ ] 8.1 Create `examples/basic_paladin.rs`
  - [ ] 8.2 Implement basic example: create Paladin, execute with simple input, print result
  - [ ] 8.3 Add comments explaining each step for junior developers
  - [ ] 8.4 Test example: `cargo run --example basic_paladin`
  - [ ] 8.5 Create `examples/paladin_with_config.rs`
  - [ ] 8.6 Implement advanced example: custom configuration, retry logic, timeout handling
  - [ ] 8.7 Demonstrate stop word usage and max_loops
  - [ ] 8.8 Test example: `cargo run --example paladin_with_config`
  - [ ] 8.9 Review all rustdoc comments in core domain layer
  - [ ] 8.10 Review all rustdoc comments in application layer
  - [ ] 8.11 Review all rustdoc comments in infrastructure layer
  - [ ] 8.12 Ensure all public APIs have rustdoc examples
  - [ ] 8.13 Generate documentation: `cargo doc --no-deps --open`
  - [ ] 8.14 Review generated documentation for clarity
  - [ ] 8.15 Add module-level documentation to `src/core/platform/container/paladin.rs`
  - [ ] 8.16 Add module-level documentation to `src/application/use_cases/paladin/mod.rs`
  - [ ] 8.17 Create or update `docs/paladin_user_guide.md` with usage guide
  - [ ] 8.18 Document error handling patterns
  - [ ] 8.19 Document retry and circuit breaker behavior
  - [ ] 8.20 Run final checks: `cargo fmt`, `cargo clippy`, `cargo test`
  - [ ] 8.21 Verify zero clippy warnings
  - [ ] 8.22 Verify all tests pass
  - [ ] 8.23 Create PR description summarizing Epic 1 implementation
  - [ ] 8.24 Request code review from team

---

**Phase 2 Complete:** All tasks have been broken down into actionable sub-tasks following TDD methodology (Red-Green-Refactor). Each task includes specific file paths, test requirements, and validation steps suitable for a junior developer to follow.
