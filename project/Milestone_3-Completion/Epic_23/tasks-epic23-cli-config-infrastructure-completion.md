# Task List: Epic 23 - CLI, Config & Infrastructure Completion

**Based on:** PRD Epic 23 - CLI, Config & Infrastructure Completion  
**Created:** February 13, 2026  
**Epic:** Epic 23: CLI, Config & Infrastructure Completion  
**Milestone:** 3 - Completion & Polish  
**Priority:** Medium  
**Estimated Duration:** 1-2 weeks  
**Dependencies:** Epics 19-22

---

## Status Summary

✅ **Task 0.0 COMPLETE**: Feature branch setup  
✅ **Task 1.0 COMPLETE**: Garrison configuration wiring (Commit: 1322f5e)  
✅ **Task 2.0 COMPLETE**: Arsenal/MCP configuration wiring  
✅ **Task 3.0 COMPLETE**: Mock LLM provider implementation (Commit: 981f026)  
✅ **Task 4.0 COMPLETE**: CLI integration tests (All subtasks including 4.6)

**Progress:**
- ✅ 4 of 8 major tasks complete (50%)
- ✅ 50 CLI tests passing (all with MockLlmAdapter, CI-ready)
- ✅ 8 Arsenal tool integration tests passing (Task 4.6 complete)
- ✅ 1,590 library tests passing
- ✅ Formation & Phalanx Battalion patterns tested

**Task Breakdown:**
- Task 0.0: ✅ Feature branch setup (2 sub-tasks)
- Task 1.0: ✅ Garrison configuration (9 sub-tasks, 28 detailed steps)
- Task 2.0: ✅ Arsenal/MCP configuration (10 sub-tasks, 34 detailed steps)
- Task 3.0: ✅ Mock LLM provider (8 sub-tasks, 25 detailed steps)
- Task 4.0: ✅ CLI integration tests (8 of 8 sub-tasks, including previously deferred 4.6)
- Task 5.0: Environment testing (8 sub-tasks, 37 detailed steps)
- Task 6.0: ✅ Scheduler integration (9 sub-tasks, 38 detailed steps)
- Task 7.0: ✅ Final validation and documentation (validation complete)

**Total:** 64 parent sub-tasks, 221 detailed implementation steps

---

## Relevant Files

### Configuration & CLI Files
- `src/application/cli/commands/agent.rs` - Agent command handler with TODOs at lines 293, 296
- `src/application/cli/config/paladin_config.rs` - Paladin YAML configuration structures (to be extended)
- `src/application/cli/config/loader.rs` - Configuration file loading and validation
- `src/application/cli/error.rs` - CLI error types (to be extended)

### Garrison Implementation Files
- `src/infrastructure/adapters/garrison/in_memory_garrison.rs` - In-memory garrison adapter
- `src/infrastructure/adapters/garrison/sqlite_garrison.rs` - SQLite garrison adapter
- `src/application/ports/output/garrison_port.rs` - Garrison port trait

### Arsenal/MCP Implementation Files
- `src/infrastructure/adapters/arsenal/mcp_stdio_adapter.rs` - MCP STDIO adapter
- `src/infrastructure/adapters/arsenal/mcp_sse_adapter.rs` - MCP SSE adapter
- `src/application/ports/output/arsenal_port.rs` - Arsenal port trait
- `src/application/use_cases/arsenal/arsenal_registry.rs` - Arsenal registry for tool management

### Mock Provider Files
- `tests/helpers/mock_llm_adapter.rs` - ✅ Mock LLM adapter for testing (COMPLETE)
- `tests/helpers/mock_paladin_port.rs` - ✅ Mock PaladinPort implementation for Battalion testing (COMPLETE)
- `tests/helpers/mod.rs` - ✅ Test utilities module (exports MockLlmAdapter and MockPaladinPort)

### Scheduler Files
- `src/application/ports/output/scheduler_port.rs` - ✅ SchedulerPort trait with JobId, JobSpec, JobInfo, JobStatus, SchedulerError (6 inline tests - COMPLETE)
- `src/infrastructure/adapters/scheduling/tokio_cron_adapter.rs` - ✅ TokioCronSchedulerAdapter using tokio-cron-scheduler v0.13 (13 inline tests - COMPLETE)
- `src/infrastructure/adapters/scheduling/mod.rs` - ✅ Scheduling module (COMPLETE)
- `src/infrastructure/adapters/output/api_content_deliverer.rs` - ✅ Content deliverer with scheduler integration for schedule_delivery and cancel_delivery (COMPLETE)
- `src/config/application_settings.rs` - ✅ SchedulerConfig struct added (enabled, default_cron, channel_size - COMPLETE)

### Test Files
- `tests/cli/garrison_config_test.rs` - ✅ Garrison configuration unit tests (9 tests - COMPLETE)
- `tests/cli/arsenal_config_test.rs` - ✅ Arsenal configuration unit tests (8 tests - COMPLETE)
- `tests/cli/error_handling_test.rs` - ✅ Error handling integration tests (14 tests - COMPLETE)
- `tests/cli/integration_tests.rs` - ✅ CLI integration tests (3 tests - COMPLETE)
- `tests/cli/paladin_execution_test.rs` - ✅ Paladin execution integration tests (6 tests - COMPLETE)
- `tests/cli/formation_execution_test.rs` - ✅ Formation execution integration tests (4 tests - COMPLETE)
- `tests/cli/phalanx_execution_test.rs` - ✅ Phalanx execution integration tests (5 tests - COMPLETE)
- `tests/helpers/mock_llm_adapter.rs` - ✅ Mock LLM adapter for testing (COMPLETE)
- `tests/helpers/mock_paladin_port.rs` - ✅ Mock Paladin port for Battalion testing (COMPLETE)
- `tests/cli/environment_tests.rs` - Environment-specific tests (to be created)
- `tests/integration/cli_real_services_test.rs` - Docker-gated service tests (to be created)
- `tests/integration/cli_real_providers_test.rs` - API-key-gated provider tests (to be created)
- `tests/unit/scheduler_tests.rs` - ✅ Scheduler unit tests (16 tests - COMPLETE)
- `tests/integration/scheduler_integration_test.rs` - ✅ Scheduler integration tests (5 tests - COMPLETE)

### Configuration Examples
- `examples/cli_configs/paladin_with_garrison.yaml` - Example config with garrison (to be created)
- `examples/cli_configs/paladin_with_arsenal.yaml` - Example config with arsenal (to be created)
- `examples/cli_configs/paladin_full_config.yaml` - Complete config example (to be created)

### Documentation Files
- `docs/cli/CONFIGURATION.md` - CLI configuration guide (to be updated)
- `docs/cli/TESTING.md` - CLI testing guide (to be updated)

### Notes

- Unit tests should be placed in `src/application/cli/tests/` or alongside code files
- Integration tests should be in `tests/cli/` and `tests/integration/`
- Use `cargo test` to run all tests
- Use `cargo test --test cli_integration` for CLI integration tests specifically
- Follow the completion protocol from copilot-instructions.md:
  1. Finish sub-task → mark `[x]`
  2. Run `cargo test`, `cargo fmt --check`, `cargo clippy`
  3. If all tests pass and checks succeed: stage, commit with descriptive message
  4. Mark parent task `[x]`

---

## Instructions for Completing Tasks

**IMPORTANT:** As you complete each task, you must check it off in this markdown file by changing `- [ ]` to `- [x]`. This helps track progress and ensures you don't skip any steps.

Example:
- `- [ ] 1.1 Read file` → `- [x] 1.1 Read file` (after completing)

Update the file after completing each sub-task, not just after completing an entire parent task.

---

## Tasks

- [x] 0.0 Create feature branch
  - [x] 0.1 Create and checkout new branch: `git checkout -b feature/epic-23-cli-config-infrastructure-completion`
  - [x] 0.2 Verify clean working directory with `git status`

- [x] 1.0 Implement garrison configuration wiring (US-23.1)
  - [x] 1.1 Read `src/application/cli/commands/agent.rs` and locate TODO at line 293
  - [x] 1.2 Extend `src/application/cli/config/paladin_config.rs` with garrison configuration schema
    - [x] 1.2.1 Add `GarrisonConfig` struct with fields: type, path, max_entries, ttl_seconds
    - [x] 1.2.2 Add serde derives and validation attributes
    - [x] 1.2.3 Add to main `PaladinYamlConfig` struct
  - [x] 1.3 Implement garrison configuration parsing in `src/application/cli/config/loader.rs`
    - [x] 1.3.1 Add `load_garrison_config()` method to `ConfigLoader`
    - [x] 1.3.2 Implement validation logic (type present, path for sqlite, positive values)
    - [x] 1.3.3 Add environment variable resolution if needed
  - [x] 1.4 Implement garrison adapter instantiation logic
    - [x] 1.4.1 Match on garrison type and create appropriate adapter (InMemory or SQLite)
    - [x] 1.4.2 Pass configuration parameters (path, max_entries, ttl_seconds) to adapter constructors
    - [x] 1.4.3 Handle adapter initialization errors
  - [x] 1.5 Add actionable error messages to `src/application/cli/error.rs`
    - [x] 1.5.1 Add `GarrisonConfigError` variant with descriptive messages
    - [x] 1.5.2 Implement error formatting per FR-23.1.4 requirements
  - [x] 1.6 Wire garrison to PaladinBuilder in `agent.rs` (replace TODO at line 293)
    - [x] 1.6.1 Load garrison config from YAML
    - [x] 1.6.2 Instantiate garrison adapter
    - [x] 1.6.3 Pass garrison to `PaladinBuilder::with_garrison()`
  - [x] 1.7 Write unit tests in `tests/cli/garrison_config_test.rs`
    - [x] 1.7.1 Test parsing valid in_memory garrison config
    - [x] 1.7.2 Test parsing valid sqlite garrison config  
    - [x] 1.7.3 Test validation errors (missing type, invalid type, missing path for sqlite)
    - [x] 1.7.4 Test validation errors (invalid max_entries, invalid ttl_seconds)
    - [x] 1.7.5 Test garrison adapter instantiation for both types
  - [x] 1.8 Create example YAML configurations
    - [x] 1.8.1 Create `examples/cli_configs/paladin_with_garrison.yaml` with in_memory example
    - [x] 1.8.2 Add sqlite garrison example to same file
  - [x] 1.9 Run tests: `cargo test garrison_config` and verify all pass

- [x] 2.0 Implement arsenal/MCP configuration wiring (US-23.2)
  - [x] 2.1 Read `src/application/cli/commands/agent.rs` and locate TODO at line 296
  - [x] 2.2 Extend `src/application/cli/config/paladin_config.rs` with arsenal configuration schema
    - [x] 2.2.1 Add `McpServerConfig` struct with fields: name, type, command, args, url, auth_token
    - [x] 2.2.2 Add `ArsenalConfig` struct with field: mcp_servers (Vec<McpServerConfig>)
    - [x] 2.2.3 Add serde derives and validation attributes
    - [x] 2.2.4 Add to main `PaladinYamlConfig` struct
  - [x] 2.3 Implement arsenal configuration parsing in `src/application/cli/config/loader.rs`
    - [x] 2.3.1 Add `load_arsenal_config()` method to `ConfigLoader`
    - [x] 2.3.2 Implement validation logic (name/type required, command for stdio, url for sse)
    - [x] 2.3.3 Add URL validation for SSE servers
    - [x] 2.3.4 Add environment variable resolution for auth tokens and other fields
  - [x] 2.4 Implement MCP adapter instantiation logic
    - [x] 2.4.1 Iterate over mcp_servers and match on type
    - [x] 2.4.2 Create MCPStdioAdapter for stdio type with command and args
    - [x] 2.4.3 Create MCPSseAdapter for sse type with url and auth_token
    - [x] 2.4.4 Handle adapter connection errors gracefully
  - [x] 2.5 Implement tool registration in arsenal registry
    - [x] 2.5.1 Query each MCP adapter for available tools
    - [x] 2.5.2 Register tools in ArsenalRegistry with name, description, and capability mapping
    - [x] 2.5.3 Store server reference for tool invocation
  - [x] 2.6 Add actionable error messages to `src/application/cli/error.rs`
    - [x] 2.6.1 Add `ArsenalConfigError` variant with descriptive messages
    - [x] 2.6.2 Implement error formatting per FR-23.2.5 requirements
  - [x] 2.7 Wire arsenal to PaladinBuilder in `agent.rs` (replace TODO at line 296)
    - [x] 2.7.1 Load arsenal config from YAML
    - [x] 2.7.2 Instantiate MCP adapters and create arsenal registry
    - [x] 2.7.3 Pass arsenal registry to `PaladinBuilder::with_arsenal()`
  - [x] 2.8 Write unit tests in `tests/cli/arsenal_config_test.rs`
    - [x] 2.8.1 Test parsing valid stdio MCP server config
    - [x] 2.8.2 Test parsing valid sse MCP server config
    - [x] 2.8.3 Test parsing multiple MCP servers
    - [x] 2.8.4 Test validation errors (missing name, missing type, invalid type)
    - [x] 2.8.5 Test validation errors (missing command for stdio, missing url for sse)
    - [x] 2.8.6 Test URL validation for SSE servers
    - [x] 2.8.7 Test environment variable resolution
  - [x] 2.9 Create example YAML configurations
    - [x] 2.9.1 Create `examples/cli_configs/paladin_with_arsenal.yaml` with stdio example
    - [x] 2.9.2 Add sse MCP server example to same file
    - [x] 2.9.3 Create `examples/cli_configs/paladin_full_config.yaml` with garrison + arsenal
  - [x] 2.10 Run tests: `cargo test arsenal_config` and verify all pass

- [x] 3.0 Implement mock LLM provider infrastructure (US-23.3)
  - [x] 3.1 Design MockLlmAdapter structure in `tests/common/mock_llm.rs`
    - [x] 3.1.1 Define `MockResponse` enum (Text, ToolCall, Streaming, Error)
    - [x] 3.1.2 Define `Invocation` struct to record calls (prompt, model, timestamp)
    - [x] 3.1.3 Define `MockLlmAdapter` struct with responses, index, invocations
  - [x] 3.2 Implement LlmPort trait for MockLlmAdapter
    - [x] 3.2.1 Implement `generate()` method to return configured responses
    - [x] 3.2.2 Implement `generate_stream()` method for streaming simulation
    - [x] 3.2.3 Implement `validate_model()` method (always succeeds for mock)
    - [x] 3.2.4 Record each invocation for test assertions
  - [x] 3.3 Implement builder pattern for configurable responses
    - [x] 3.3.1 Add `MockLlmAdapter::new()` constructor
    - [x] 3.3.2 Add `add_success(text)` method for simple text responses
    - [x] 3.3.3 Add `add_tool_call(name, args)` method for tool invocation responses
    - [x] 3.3.4 Add `add_streaming(chunks)` method for streaming responses
    - [x] 3.3.5 Add `add_failure(error)` method to simulate API failures
  - [x] 3.4 Implement invocation recording and query methods
    - [x] 3.4.1 Add `invocations()` method to get all recorded calls
    - [x] 3.4.2 Add `call_count()` method
    - [x] 3.4.3 Add `last_prompt()` method for assertions
    - [x] 3.4.4 Add `reset()` method to clear invocations
  - [x] 3.5 Write unit tests for MockLlmAdapter
    - [x] 3.5.1 Test simple text response
    - [x] 3.5.2 Test tool call response
    - [x] 3.5.3 Test streaming response
    - [x] 3.5.4 Test error simulation (rate limit, timeout, API error)
    - [x] 3.5.5 Test invocation recording
    - [x] 3.5.6 Test multiple sequential responses
  - [x] 3.6 Create test utilities and helper functions
    - [x] 3.6.1 Add `create_test_paladin_with_mock()` helper in `tests/helpers/mod.rs`
    - [x] 3.6.2 Add `create_mock_with_responses()` helper
    - [x] 3.6.3 Add `create_mock_with_tool_calls()` helper
    - [x] 3.6.4 Add `create_mock_with_mixed_responses()` helper
  - [x] 3.7 Document MockLlmAdapter usage in code comments
  - [x] 3.8 Run tests: `cargo test mock_llm` and verify all pass

- [x] 4.0 Implement CLI integration tests with mock provider (US-23.3)
  - [x] 4.1 Set up test infrastructure in `tests/cli/`
    - [x] 4.1.1 Create test module structure
    - [x] 4.1.2 Import MockLlmAdapter and test utilities
    - [x] 4.1.3 Set up test data directory for YAML configs
  - [x] 4.2 Implement Paladin execution test in `tests/cli/paladin_execution_test.rs`
    - [x] 4.2.1 Create test YAML config for single Paladin
    - [x] 4.2.2 Set up MockLlmAdapter with expected response
    - [x] 4.2.3 Load config and execute Paladin via CLI command handler
    - [x] 4.2.4 Assert correct response returned
    - [x] 4.2.5 Assert mock was called with correct prompt
    - [x] 4.2.6 Test with garrison configured
    - [x] 4.2.7 Test with arsenal configured
  - [x] 4.3 Implement Formation execution test in `tests/cli/formation_execution_test.rs`
    - [x] 4.3.1 Create test YAML config for Formation with 3 Paladins
    - [x] 4.3.2 Set up MockLlmAdapter with sequential responses
    - [x] 4.3.3 Execute Formation via CLI command handler
    - [x] 4.3.4 Assert sequential execution (output chaining)
    - [x] 4.3.5 Assert all 3 Paladins were invoked
    - [x] 4.3.6 Verify correct input/output flow between Paladins
  - [x] 4.4 Implement Phalanx execution test in `tests/cli/phalanx_execution_test.rs`
    - [x] 4.4.1 Create test YAML config for Phalanx with 3 Paladins
    - [x] 4.4.2 Set up MockLlmAdapter with parallel responses
    - [x] 4.4.3 Execute Phalanx via CLI command handler
    - [x] 4.4.4 Assert parallel execution (all Paladins run concurrently)
    - [x] 4.4.5 Assert results are aggregated correctly
  - [x] 4.5 Implement error handling tests
    - [x] 4.5.1 Test LLM error propagation (rate limit)
    - [x] 4.5.2 Test timeout handling
    - [x] 4.5.3 Test invalid config error handling
    - [x] 4.5.4 Test missing config file error
    - [x] 4.5.5 Test graceful failure and cleanup
  - [x] 4.6 Implement tool integration tests (COMPLETED - Task 4.6 branch)
    - [x] 4.6.1 Set up mock Arsenal port (MockArsenalPort in tests/helpers/)
    - [x] 4.6.2 Configure Paladin with arsenal via PaladinExecutionService
    - [x] 4.6.3 Set up MockLlmAdapter to request tool call
    - [x] 4.6.4 Execute and verify tool was invoked (8 tests: basic flow, error handling, sequential, garrison)
    - [x] 4.6.5 Verify tool result returned to Paladin and fed back to LLM
  - [x] 4.7 Verify CI compatibility
    - [x] 4.7.1 Run tests in CI environment: `cargo test --test cli_*`
    - [x] 4.7.2 Verify no API keys required
    - [x] 4.7.3 Verify no external network dependencies
  - [x] 4.8 Run all CLI integration tests and verify pass

- [x] 5.0 Implement environment and end-to-end testing (US-23.4)
  - [x] 5.1 Implement Tier 1 tests (core functionality) in `tests/cli/environment_tests.rs`
    - [x] 5.1.1 Test happy path for `agent run` command
    - [x] 5.1.2 Test happy path for `battalion run` command
    - [x] 5.1.3 Test error handling: missing required arguments
    - [x] 5.1.4 Test error handling: invalid YAML config
    - [x] 5.1.5 Test error handling: connection failures (simulated)
    - [x] 5.1.6 Test edge case: empty input
    - [x] 5.1.7 Test edge case: very large input (>10KB)
    - [x] 5.1.8 Test edge case: malformed YAML (syntax errors)
    - [x] 5.1.9 Test edge case: concurrent operations (if applicable)
  - [x] 5.2 Implement Tier 2 tests (Docker-gated) in `tests/integration/cli_real_services_test.rs`
    - [x] 5.2.1 Add test gating: skip if Docker services not available
    - [x] 5.2.2 Test `setup-check` command with real Redis connection
    - [x] 5.2.3 Test `setup-check` command with real Qdrant connection
    - [x] 5.2.4 Test `setup-check` command with real MinIO connection
    - [x] 5.2.5 Test service health validation
    - [x] 5.2.6 Test connection error handling when service unavailable
    - [x] 5.2.7 Add clear skip message: "requires Docker services: run 'make services-up'"
  - [x] 5.3 Implement Tier 3 tests (API-key-gated) in `tests/integration/cli_real_providers_test.rs`
    - [x] 5.3.1 Add test gating: skip if OPENAI_API_KEY not set
    - [x] 5.3.2 Test `muster` command with real OpenAI API
    - [x] 5.3.3 Add test gating: skip if ANTHROPIC_API_KEY not set
    - [x] 5.3.4 Test `muster` command with real Anthropic API
    - [x] 5.3.5 Add test gating: skip if DEEPSEEK_API_KEY not set
    - [x] 5.3.6 Test `muster` command with real DeepSeek API
    - [x] 5.3.7 Test `council` command end-to-end (requires API keys)
    - [x] 5.3.8 Test streaming response handling
    - [x] 5.3.9 Add clear skip messages for each gated test
  - [x] 5.4 Implement non-interactive mode tests
    - [x] 5.4.1 Test all commands work with `--non-interactive` flag
    - [x] 5.4.2 Test commands accept all required arguments via flags
    - [x] 5.4.3 Test missing arguments result in clear error (not hanging prompt)
    - [x] 5.4.4 Verify no interactive prompts in CI/CD mode
  - [x] 5.5 Implement environment variation tests
    - [x] 5.5.1 Test with `NO_COLOR` environment variable (no ANSI codes)
    - [x] 5.5.2 Test in basic terminal environment (no fancy Unicode)
    - [x] 5.5.3 Test proper line buffering in CI/CD environments
    - [x] 5.5.4 Test with different `TERM` environment variable values
  - [x] 5.6 Implement full user journey test (with mock provider)
    - [x] 5.6.1 Simulate new user experience: onboarding → config generation
    - [x] 5.6.2 Run first agent with generated config
    - [x] 5.6.3 Verify end-to-end flow completes successfully
  - [x] 5.7 Document test execution requirements
    - [x] 5.7.1 Update `docs/cli/TESTING.md` with test tier descriptions
    - [x] 5.7.2 Document Docker requirements for Tier 2 tests
    - [x] 5.7.3 Document API key requirements for Tier 3 tests
    - [x] 5.7.4 Document how to run specific test tiers
  - [x] 5.8 Run all environment tests: `cargo test environment_tests` and verify pass

- [x] 6.0 Implement scheduler integration (US-23.5)
  - [x] 6.1 Add tokio-cron-scheduler dependency to Cargo.toml
    - [x] 6.1.1 Add `tokio-cron-scheduler = "0.13"` to dependencies
    - [x] 6.1.2 Run `cargo build` to fetch dependency
  - [x] 6.2 Design and implement SchedulerPort trait in `src/application/ports/output/scheduler_port.rs`
    - [x] 6.2.1 Create file and module structure
    - [x] 6.2.2 Define `JobSpec` struct (schedule, job_fn, metadata)
    - [x] 6.2.3 Define `JobId`, `JobStatus` types
    - [x] 6.2.4 Define `SchedulerPort` trait with async_trait
    - [x] 6.2.5 Add methods: schedule_job, cancel_job, get_job_status
    - [x] 6.2.6 Define `SchedulerError` enum
  - [x] 6.3 Implement TokioCronSchedulerAdapter in `src/infrastructure/adapters/scheduling/tokio_cron_adapter.rs`
    - [x] 6.3.1 Create module structure: `src/infrastructure/adapters/scheduling/`
    - [x] 6.3.2 Define `TokioCronSchedulerAdapter` struct with JobScheduler and job_tracker
    - [x] 6.3.3 Implement `new()` constructor
    - [x] 6.3.4 Implement `schedule_job()` - create job from spec, add to scheduler
    - [x] 6.3.5 Implement `cancel_job()` - remove job from scheduler
    - [x] 6.3.6 Implement `get_job_status()` - query job state
    - [x] 6.3.7 Add job state tracking (scheduled, running, completed, failed)
    - [x] 6.3.8 Add error handling and retry logic
  - [x] 6.4 Replace scheduler stub in api_content_deliverer.rs
    - [x] 6.4.1 Read current stub at line 297 in `src/infrastructure/adapters/output/api_content_deliverer.rs`
    - [x] 6.4.2 Add `SchedulerPort` field to `APIContentDeliverer` struct
    - [x] 6.4.3 Update constructor to accept scheduler adapter
    - [x] 6.4.4 Replace `schedule_delivery()` stub with real implementation
    - [x] 6.4.5 Create JobSpec from content_id and schedule
    - [x] 6.4.6 Call `scheduler.schedule_job()` and handle errors
    - [x] 6.4.7 Update return type to return `JobId`
  - [x] 6.5 Implement cancellation support
    - [x] 6.5.1 Add `cancel_delivery()` method to `APIContentDeliverer`
    - [x] 6.5.2 Track scheduled job IDs internally
    - [x] 6.5.3 Call `scheduler.cancel_job()` when canceling
  - [x] 6.6 Write unit tests in `tests/unit/scheduler_tests.rs`
    - [x] 6.6.1 Create mock SchedulerPort implementation
    - [x] 6.6.2 Test job creation and scheduling
    - [x] 6.6.3 Test job cancellation
    - [x] 6.6.4 Test job execution with success
    - [x] 6.6.5 Test job execution with failure and retry
    - [x] 6.6.6 Test state transitions (scheduled → running → completed)
    - [x] 6.6.7 Test error handling
  - [x] 6.7 Write integration test in `tests/integration/scheduler_integration_test.rs`
    - [x] 6.7.1 Create test with real TokioCronSchedulerAdapter
    - [x] 6.7.2 Schedule job with short delay (2 seconds)
    - [x] 6.7.3 Use tokio::time::sleep to wait for execution
    - [x] 6.7.4 Verify job executes at expected time (with tolerance)
    - [x] 6.7.5 Verify job state updates correctly
    - [x] 6.7.6 Test job cancellation before execution
  - [x] 6.8 Add scheduler configuration schema
    - [x] 6.8.1 Add `SchedulerConfig` struct to application config
    - [x] 6.8.2 Add fields: enabled, default_cron, channel_size
    - [x] 6.8.3 Add to YAML config structure
    - [x] 6.8.4 Document configuration options
  - [x] 6.9 Run scheduler tests: `cargo test scheduler` and verify all pass

- [x] 7.0 Final validation and documentation
  - [x] 7.1 Run full test suite
    - [x] 7.1.1 Run `cargo test` and verify all tests pass
    - [x] 7.1.2 Run `make test-all` if available
    - [x] 7.1.3 Review test output for any warnings or skipped tests
  - [x] 7.2 Run code quality checks
    - [x] 7.2.1 Run `cargo fmt` to format all code
    - [x] 7.2.2 Run `cargo fmt --check` to verify formatting
    - [x] 7.2.3 Run `cargo clippy -- -D warnings` and fix all warnings
    - [x] 7.2.4 Run `cargo clippy --all-targets -- -D warnings`
    - [x] 7.2.5 Run `make clean-code` if available
  - [x] 7.3 Verify all TODO items resolved
    - [x] 7.3.1 Check `src/application/cli/commands/agent.rs` line 293 - should be implemented
    - [x] 7.3.2 Check `src/application/cli/commands/agent.rs` line 296 - should be implemented
    - [x] 7.3.3 Check `src/infrastructure/adapters/output/api_content_deliverer.rs` line 297 - should be implemented
    - [x] 7.3.4 Search for any remaining TODOs: `grep -r "TODO" src/application/cli/`
    - [x] 7.3.5 Search for any remaining `unimplemented!()`: `grep -r "unimplemented!" src/`
  - [x] 7.4 Update CLI documentation
    - [x] 7.4.1 Update `docs/cli/CONFIGURATION.md` with garrison and arsenal config examples
    - [x] 7.4.2 Add scheduler configuration section
    - [x] 7.4.3 Update `docs/CLI_USAGE.md` with new capabilities
    - [x] 7.4.4 Add troubleshooting section for common config errors
  - [x] 7.5 Update testing documentation
    - [x] 7.5.1 Verify `docs/cli/TESTING.md` is complete and accurate
    - [x] 7.5.2 Document mock provider usage for test authors
    - [x] 7.5.3 Document test gating and prerequisites
  - [x] 7.6 Verify example configurations
    - [x] 7.6.1 Test each example YAML config file
    - [x] 7.6.2 Ensure examples work out of the box
    - [x] 7.6.3 Add comments to example files explaining each section
  - [x] 7.7 Create Epic 23 completion summary
    - [x] 7.7.1 Document all completed user stories (US-23.1 through US-23.5)
    - [x] 7.7.2 Document test coverage achieved
    - [x] 7.7.3 Document all deferred work addressed
    - [x] 7.7.4 List any remaining open items or future enhancements
  - [x] 7.8 Update project documentation
    - [x] 7.8.1 Update `project/Milestone_3-Completion/Project_Plan_Milestone_3.md` with Epic 23 completion
    - [x] 7.8.2 Update CHANGELOG.md with Epic 23 changes
  - [x] 7.9 Final test run and verification
    - [x] 7.9.1 Run `cargo test --all-features` and verify all pass
    - [x] 7.9.2 Run `cargo build --release` and verify clean build
    - [x] 7.9.3 Verify no compilation warnings
  
---

## Implementation Notes

**Configuration wiring** addresses TODO items at:
- `src/application/cli/commands/agent.rs` line 293 (garrison)
- `src/application/cli/commands/agent.rs` line 296 (arsenal)

**Scheduler integration** addresses TODO at:
- `src/infrastructure/adapters/output/api_content_deliverer.rs` line 297

**Deferred tests** from:
- Epic 9 (Armory CLI Tools): Tasks 5.8, 5.9
- Epic 10 (Validation & Documentation): Tasks 13.4-13.6
- Epic 18 (CLI Enhancement & Polish): Tasks 9.1-9.7

---
