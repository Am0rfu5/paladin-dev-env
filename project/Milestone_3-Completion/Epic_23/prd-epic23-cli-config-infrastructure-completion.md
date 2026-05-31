# Product Requirements Document: Epic 23 - CLI, Config & Infrastructure Completion

## Document Information

- **Version:** 1.0
- **Created:** February 13, 2026
- **Epic:** Epic 23: CLI, Config & Infrastructure Completion
- **Milestone:** 3 - Completion & Polish
- **Priority:** Medium
- **Estimated Duration:** 1-2 weeks
- **Dependencies:** Epics 19-22

---

## 1. Introduction/Overview

### Problem Statement

During the development of Epics 9 (Armory CLI Tools), 10 (Validation & Documentation), and 18 (CLI Enhancement & Polish), several critical features were deferred due to cross-cutting dependencies and incomplete infrastructure:

1. **Configuration Wiring Gaps**: The CLI `muster` command has TODO items (lines 293, 296 in `src/application/cli/commands/agent.rs`) for wiring garrison (memory) and arsenal (tools) configuration from YAML files. Currently, agents launched from config files lack proper memory and tool access.

2. **Test Coverage Gaps**: Multiple CLI integration tests were deferred because they required mock LLM provider support that didn't exist at the time. Without these tests, CLI reliability in production environments is unvalidated.

3. **Infrastructure Stubs**: The API content deliverer has a scheduler stub at line 297 in `src/infrastructure/adapters/output/api_content_deliverer.rs` that needs completion for production time-based delivery.

With the full feature set now in place from Milestones 1 and 2, this epic completes all infrastructure and CLI wiring to achieve a production-ready CLI experience.

### Goal

Complete all deferred CLI configuration wiring, implement comprehensive testing with mock providers, and finish infrastructure stubs to deliver a fully functional, production-ready CLI tool for the Paladin multi-agent framework.

---

## 2. Goals

1. **Complete Configuration Pipeline**: Enable full YAML-based configuration of garrison (memory) and arsenal (tools) for CLI-launched agents
2. **Achieve Test Coverage**: Implement mock LLM provider infrastructure and achieve standard test coverage (happy path + error handling + edge cases) for all CLI commands
3. **Enable Production Deployment**: Complete scheduler integration for time-based content delivery
4. **Validate Reliability**: Ensure CLI works across different environments, terminal types, and CI/CD scenarios
5. **Close Technical Debt**: Address all deferred tasks from Epics 9, 10, and 18

---

## 3. User Stories

### US-23.1: CLI Garrison Configuration

**As a** developer  
**I want** the CLI `muster` command to configure garrison from YAML  
**So that** agents launched from config files have proper memory management

**Acceptance Criteria:**
- Parse garrison configuration from YAML (type: `in_memory` | `sqlite`, path, max_entries)
- Instantiate appropriate garrison adapter based on config type
- Pass configured garrison to `PaladinBuilder` during agent construction
- Validate garrison config and provide actionable error messages for invalid configurations
- Unit tests demonstrate successful garrison configuration from sample YAML
- Integration test validates garrison persistence across agent executions

**Technical Context:**
- Source: `src/application/cli/commands/agent.rs` line 293 (Task 5.8 from Epic 9)
- Related Files: `src/application/cli/config/paladin_config.rs`, garrison adapter implementations

---

### US-23.2: CLI Arsenal/MCP Configuration

**As a** developer  
**I want** the CLI `muster` command to configure arsenal from YAML  
**So that** agents launched from config files have access to external tools via MCP

**Acceptance Criteria:**
- Parse MCP server configuration from YAML including:
  - Server name and type (`stdio` | `sse`)
  - Command and arguments for STDIO servers
  - URL and authentication for SSE servers
- Instantiate MCP adapters based on configuration type
- Register discovered tools in arsenal registry with proper capability mapping
- Pass configured arsenal to `PaladinBuilder` during agent construction
- Validate arsenal config and provide clear error messages for:
  - Invalid MCP server type
  - Missing required fields (command for stdio, url for sse)
  - MCP server connection failures
- Unit tests with sample configurations for both STDIO and SSE server types
- Integration test validates tool discovery and invocation through configured arsenal

**Technical Context:**
- Source: `src/application/cli/commands/agent.rs` line 296 (Task 5.9 from Epic 9)
- Related Files: `src/application/cli/config/paladin_config.rs`, MCP adapter implementations
- Dependencies: Arsenal and MCP infrastructure from earlier epics

---

### US-23.3: CLI Integration Tests with Mock Provider

**As a** developer  
**I want** CLI integration tests that use configurable mock LLM providers  
**So that** end-to-end CLI workflows are validated without requiring API keys or external dependencies

**Acceptance Criteria:**
- Implement configurable mock LLM provider that can:
  - Return predefined responses for tests
  - Simulate different response types (simple text, tool calls, streaming)
  - Simulate error conditions (rate limits, API failures, timeouts)
  - Simulate streaming behavior with chunked responses
- Create integration tests for:
  - **Paladin from config**: Run single agent with mock LLM using YAML configuration
  - **Formation execution**: Sequential multi-agent workflow with output chaining
  - **Phalanx execution**: Parallel multi-agent execution with result aggregation
  - **Error handling**: Test graceful failure and error propagation
  - **Tool integration**: Test arsenal tool invocation through mock provider
- All tests run in CI without external dependencies (no API keys required)
- Tests validate both success and failure scenarios
- Mock provider is reusable across different test scenarios

**Technical Context:**
- Source: Deferred tasks from Epic 10 (Task 13.4-13.6)
- Related Files: `tests/integration/cli_integration_test.rs`, test utilities
- Pattern: Follow existing test infrastructure patterns from core framework tests

---

### US-23.4: CLI End-to-End & Environment Testing

**As a** developer  
**I want** CLI tested across real environments and scenarios  
**So that** the CLI works reliably in production across different deployment contexts

**Acceptance Criteria:**

**Standard Coverage Testing** (run in all CI environments):
- **Happy path tests**: Core functionality for each command works as expected
- **Error handling tests**: Graceful handling of missing arguments, invalid configs, connection failures
- **Edge case tests**: Empty inputs, very large inputs, malformed YAML, concurrent operations

**Environment-Specific Testing** (gated by feature flags or environment variables):
- **Full user journey test**: New user onboarding → configuration → first agent run (with mock provider)
- **Service integration test**: `setup-check` command with real services (Redis, Qdrant, MinIO) - requires Docker
- **Real LLM provider tests**: Test `muster` command with actual OpenAI/DeepSeek/Anthropic APIs - requires API keys
- **Council command test**: End-to-end multi-agent discussion - requires API keys
- **Non-interactive mode**: All commands work in CI/CD without prompts (using --non-interactive flag or equivalent)
- **Terminal compatibility**: Test with `NO_COLOR` environment variable, different terminal types (basic validation)

**Test Organization:**
- Core functionality tests run in standard CI pipeline
- Environment-specific tests are clearly documented with setup requirements
- Tests provide clear skip messages when prerequisites aren't met
- Test results are actionable (failures indicate specific issues, not just "test failed")

**Technical Context:**
- Source: Deferred tasks from Epic 18 (Task 9.1-9.7)
- Related Files: `tests/cli/*.rs`, CI configuration files
- Approach: Layered testing strategy with appropriate gating for different scenarios

---

### US-23.5: API Content Deliverer Scheduler Integration

**As a** developer  
**I want** scheduled content delivery to use a production-ready scheduler  
**So that** time-based delivery works reliably in production deployments

**Acceptance Criteria:**
- Integrate `tokio-cron-scheduler` or equivalent production-grade scheduler
- Implement `SchedulerAdapter` that wraps the scheduler with application-specific logic:
  - Job creation and scheduling based on cron expressions or intervals
  - Job cancellation and removal for pending deliveries
  - Job state tracking (scheduled, running, completed, failed)
  - Error handling and retry logic for failed jobs
- Update `APIContentDeliverer::schedule_delivery()` to create real scheduled jobs (remove stub implementation at line 297)
- Implement cancellation support for pending scheduled deliveries
- Unit tests with mock scheduler demonstrate:
  - Job creation and scheduling
  - Job cancellation
  - Job execution lifecycle
- Integration test validates scheduled job execution at specified times
- Documentation for scheduler configuration and monitoring

**Technical Context:**
- Source: `src/infrastructure/adapters/output/api_content_deliverer.rs` line 297
- Approach: Moderate complexity - wrap scheduler in adapter pattern following hexagonal architecture
- Related Patterns: Similar to existing adapters (Redis queue, MinIO storage)

---

## 4. Functional Requirements

### FR-23.1: Garrison Configuration (US-23.1)

**FR-23.1.1** - The system MUST parse garrison configuration from YAML files with the following schema:
```yaml
garrison:
  type: "in_memory" | "sqlite"  # Required
  path: "./data/garrison.db"     # Required for sqlite, optional for in_memory
  max_entries: 1000              # Optional, default varies by type
  ttl_seconds: 3600              # Optional, time-to-live for entries
```

**FR-23.1.2** - The system MUST instantiate the appropriate garrison adapter based on the `type` field:
- `in_memory` → `InMemoryGarrison`
- `sqlite` → `SqliteGarrison`

**FR-23.1.3** - The system MUST validate garrison configuration before instantiation:
- `type` field is present and valid
- `path` is present for sqlite type
- `path` is writable or parent directory exists for sqlite type
- `max_entries` is positive if specified
- `ttl_seconds` is positive if specified

**FR-23.1.4** - The system MUST provide actionable error messages for invalid garrison configurations:
- Missing type: "garrison.type is required (valid values: in_memory, sqlite)"
- Invalid type: "garrison.type must be 'in_memory' or 'sqlite', got: {value}"
- Missing path for sqlite: "garrison.path is required for type: sqlite"
- Invalid path: "garrison.path is not writable: {path} - {error}"
- Invalid max_entries: "garrison.max_entries must be positive, got: {value}"

**FR-23.1.5** - The system MUST pass the configured garrison to `PaladinBuilder` during agent construction

### FR-23.2: Arsenal Configuration (US-23.2)

**FR-23.2.1** - The system MUST parse arsenal configuration from YAML files with the following schema:
```yaml
arsenal:
  mcp_servers:
    - name: "web_search"           # Required
      type: "stdio"                 # Required: stdio | sse
      command: "uvx"                # Required for stdio
      args: ["mcp-web-search"]      # Optional for stdio
    - name: "api_service"
      type: "sse"
      url: "https://api.example.com/mcp"  # Required for sse
      auth_token: "${MCP_AUTH_TOKEN}"      # Optional for sse
```

**FR-23.2.2** - The system MUST instantiate appropriate MCP adapters based on the `type` field:
- `stdio` → `MCPStdioAdapter`
- `sse` → `MCPSseAdapter`

**FR-23.2.3** - The system MUST validate arsenal configuration before instantiation:
- Each server has required `name` and `type` fields
- STDIO servers have required `command` field
- SSE servers have required `url` field
- URLs are valid HTTP/HTTPS endpoints
- Environment variable references (${VAR}) can be resolved

**FR-23.2.4** - The system MUST register discovered tools in the arsenal registry with:
- Tool name and description from MCP server
- Capability mapping (tool parameters → MCP schema)
- Server reference for tool invocation

**FR-23.2.5** - The system MUST provide actionable error messages for invalid arsenal configurations:
- Missing name: "arsenal.mcp_servers[{index}].name is required"
- Missing type: "arsenal.mcp_servers[{index}].type is required (valid values: stdio, sse)"
- Invalid type: "arsenal.mcp_servers[{name}].type must be 'stdio' or 'sse', got: {value}"
- Missing command for stdio: "arsenal.mcp_servers[{name}].command is required for type: stdio"
- Missing url for sse: "arsenal.mcp_servers[{name}].url is required for type: sse"
- Invalid url: "arsenal.mcp_servers[{name}].url is not a valid HTTP(S) endpoint: {url}"
- Unresolved env var: "Environment variable not found: {var_name}"
- Connection failure: "Failed to connect to MCP server '{name}': {error}"

**FR-23.2.6** - The system MUST pass the configured arsenal registry to `PaladinBuilder` during agent construction

### FR-23.3: Mock LLM Provider (US-23.3)

**FR-23.3.1** - The system MUST implement a `MockLlmAdapter` that implements the `LlmPort` trait

**FR-23.3.2** - The `MockLlmAdapter` MUST support configurable response modes:
- **Simple text**: Return predefined text response
- **Tool calls**: Return response with tool invocation requests
- **Streaming**: Return chunked responses simulating streaming behavior
- **Errors**: Simulate API failures, rate limits, timeouts

**FR-23.3.3** - The `MockLlmAdapter` MUST allow configuration via test setup:
```rust
let mock = MockLlmAdapter::new()
    .with_response("Mocked response text")
    .with_tool_call("web_search", json!({"query": "test"}))
    .with_streaming(vec!["chunk1", "chunk2", "chunk3"])
    .with_error(LlmError::RateLimit);
```

**FR-23.3.4** - The `MockLlmAdapter` MUST record invocations for test assertions:
- Number of calls made
- Prompts received
- Models requested
- Tool calls made

**FR-23.3.5** - Integration tests MUST validate:
- Single Paladin execution from YAML config with mock provider
- Formation (sequential) execution with multiple Paladins
- Phalanx (parallel) execution with multiple Paladins
- Error handling and recovery
- Tool integration through arsenal

**FR-23.3.6** - Mock-based tests MUST run in CI without external dependencies (no API keys)

### FR-23.4: Environment Testing (US-23.4)

**FR-23.4.1** - The system MUST provide a comprehensive test suite with three tiers:

**Tier 1 - Core Functionality** (always run in CI):
- Happy path tests for all commands
- Error handling for common failure scenarios
- Edge case handling (empty inputs, large inputs, malformed configs)

**Tier 2 - Service Integration** (Docker-gated):
- `setup-check` command with real Redis, Qdrant, MinIO services
- Service health validation
- Connection error handling

**Tier 3 - Provider Integration** (API-key-gated):
- Real LLM provider execution (OpenAI, DeepSeek, Anthropic)
- Council command end-to-end test
- Streaming response handling

**FR-23.4.2** - The system MUST support non-interactive mode for CI/CD:
- All commands accept required arguments via flags (no prompts)
- `--non-interactive` flag disables all interactive prompts
- Missing required arguments result in clear error messages, not hanging prompts

**FR-23.4.3** - The system MUST handle environment variations:
- `NO_COLOR` environment variable disables ANSI color codes
- Works in basic terminal environments (no fancy Unicode)
- Proper line buffering in CI/CD environments

**FR-23.4.4** - Tests MUST provide clear skip messages when prerequisites aren't met:
```
test service_integration ... skipped (requires Docker services: run 'make services-up')
test openai_provider ... skipped (requires OPENAI_API_KEY environment variable)
```

### FR-23.5: Scheduler Integration (US-23.5)

**FR-23.5.1** - The system MUST integrate `tokio-cron-scheduler` crate (or equivalent) for job scheduling

**FR-23.5.2** - The system MUST implement `SchedulerAdapter` following hexagonal architecture:
- Lives in `src/infrastructure/adapters/scheduling/`
- Implements application-layer port trait `SchedulerPort`
- Wraps `tokio-cron-scheduler` with application-specific logic

**FR-23.5.3** - The `SchedulerAdapter` MUST support:
- Creating scheduled jobs from cron expressions or intervals
- Canceling pending jobs by job ID
- Tracking job state (scheduled, running, completed, failed)
- Error handling with configurable retry logic
- Async job execution

**FR-23.5.4** - The `APIContentDeliverer` MUST use the scheduler adapter:
- Replace stub at line 297 with real implementation
- `schedule_delivery()` creates real scheduled jobs
- Jobs execute content delivery at specified times
- Job failures are logged and optionally retried

**FR-23.5.5** - The system MUST provide unit tests with mock scheduler:
- Job creation and scheduling
- Job cancellation
- Job execution with success
- Job execution with failure and retry
- State transitions

**FR-23.5.6** - The system MUST provide integration test:
- Schedule job with short delay (e.g., 2 seconds)
- Verify job executes at expected time (within tolerance)
- Verify job state updates correctly

**FR-23.5.7** - The system MUST document scheduler configuration:
```yaml
scheduler:
  max_concurrent_jobs: 10        # Optional, default: 10
  retry_failed_jobs: true        # Optional, default: false
  max_retries: 3                 # Optional, default: 3
  retry_delay_seconds: 60        # Optional, default: 60
```

---

## 5. Non-Goals (Out of Scope)

1. **Advanced Garrison Features**: Semantic search, vector embeddings, or RAG integration in garrison configuration (covered by other epics)

2. **Advanced Arsenal Features**: Custom tool development, tool composition, or arsenal orchestration patterns (separate epic)

3. **Scheduler UI/Management**: Web interface or dashboard for viewing/managing scheduled jobs (future enhancement)

4. **Distributed Scheduling**: Multi-node scheduler coordination or distributed job execution (not needed for MVP)

5. **CLI Feature Additions**: New CLI commands or capabilities beyond completing deferred work

6. **Performance Optimization**: Benchmarking or optimization of CLI performance (covered by Epic 24)

7. **Configuration Hot-Reload**: Runtime configuration updates without restarting (future enhancement)

8. **Legacy Code Cleanup**: TODOs in pre-agent-epic code (explicitly excluded per Milestone 3 plan)

---

## 6. Design Considerations

### Configuration Schema Design

**YAML Structure** - Follow existing patterns from Epic 18:
```yaml
# config/paladin.yaml
paladin:
  system_prompt: "You are a helpful assistant"
  model: "gpt-4"
  temperature: 0.7
  max_loops: 3

garrison:
  type: "sqlite"
  path: "./data/garrison.db"
  max_entries: 1000

arsenal:
  mcp_servers:
    - name: "web_search"
      type: "stdio"
      command: "uvx"
      args: ["mcp-web-search"]
```

### Mock Provider Design

**Trait Implementation** - Implement `LlmPort` trait fully:
```rust
pub struct MockLlmAdapter {
    responses: Vec<MockResponse>,
    current_index: AtomicUsize,
    invocations: Arc<Mutex<Vec<Invocation>>>,
}

impl LlmPort for MockLlmAdapter {
    async fn generate(&self, request: LlmRequest) -> Result<LlmResponse, LlmError>;
    async fn generate_stream(&self, request: LlmRequest) -> Result<LlmStream, LlmError>;
    fn validate_model(&self, model: &str) -> Result<(), LlmError>;
}
```

### Scheduler Architecture

**Port/Adapter Pattern**:
```rust
// Application layer port
#[async_trait]
pub trait SchedulerPort: Send + Sync {
    async fn schedule_job(&self, spec: JobSpec) -> Result<JobId, SchedulerError>;
    async fn cancel_job(&self, job_id: JobId) -> Result<(), SchedulerError>;
    async fn get_job_status(&self, job_id: JobId) -> Result<JobStatus, SchedulerError>;
}

// Infrastructure adapter
pub struct TokioCronSchedulerAdapter {
    scheduler: JobScheduler,
    job_tracker: Arc<Mutex<HashMap<JobId, JobHandle>>>,
}
```

### Test Organization

**Directory Structure**:
```
tests/
├── cli/
│   ├── integration_tests.rs          # Mock-based integration tests
│   ├── paladin_execution_test.rs     # Single agent tests
│   ├── formation_execution_test.rs   # Sequential workflow tests
│   ├── phalanx_execution_test.rs     # Parallel execution tests
│   └── environment_tests.rs          # Environment-specific tests
└── integration/
    ├── cli_real_services_test.rs     # Docker-gated tests
    └── cli_real_providers_test.rs    # API-key-gated tests
```

---

## 7. Technical Considerations

### Dependencies

**New Crate Dependencies**:
- `tokio-cron-scheduler = "0.9"` - For scheduler integration
- `mockito = "1.2"` (dev-dependency) - For HTTP mocking in tests if needed

**Internal Dependencies**:
- Garrison adapters from earlier epics
- Arsenal/MCP infrastructure from earlier epics
- LLM provider traits and implementations
- CLI configuration infrastructure from Epic 18

### Integration Points

**Configuration Loading** - Extend `src/application/cli/config/loader.rs`:
```rust
pub struct ConfigLoader {
    // Existing fields...
}

impl ConfigLoader {
    pub fn load_garrison_config(&self, config: &YamlConfig) -> Result<Arc<dyn GarrisonPort>>;
    pub fn load_arsenal_config(&self, config: &YamlConfig) -> Result<ArsenalRegistry>;
}
```

**Agent Construction** - Update `src/application/cli/commands/agent.rs`:
```rust
pub async fn handle_agent_run(args: AgentRunArgs) -> Result<(), CliError> {
    let config = ConfigLoader::load(&args.config)?;

    // Load garrison (line 293)
    let garrison = config.load_garrison_config()?;

    // Load arsenal (line 296)
    let arsenal = config.load_arsenal_config()?;

    let paladin = PaladinBuilder::new(llm_port)
        .system_prompt(config.system_prompt)
        .with_garrison(garrison)
        .with_arsenal(arsenal)
        .build()?;

    // Execute...
}
```

### Error Handling

**Unified Error Types** - Extend `src/application/cli/error.rs`:
```rust
#[derive(Debug, thiserror::Error)]
pub enum CliError {
    #[error("Configuration error: {0}")]
    ConfigError(String),

    #[error("Garrison initialization failed: {0}")]
    GarrisonError(#[from] GarrisonError),

    #[error("Arsenal initialization failed: {0}")]
    ArsenalError(#[from] ArsenalError),

    #[error("Scheduler error: {0}")]
    SchedulerError(#[from] SchedulerError),

    // Existing variants...
}
```

### Testing Strategy

**Layered Testing Approach**:

1. **Unit Tests** - Test configuration parsing, validation, and error handling in isolation
2. **Mock Integration Tests** - Test CLI workflows with mock LLM provider (no external deps)
3. **Service Integration Tests** - Test with real Docker services (gated by feature flag or env var)
4. **Provider Integration Tests** - Test with real LLM APIs (gated by API key presence)

**Test Gating Pattern**:
```rust
#[test]
#[cfg_attr(not(feature = "integration-tests"), ignore)]
fn test_with_real_services() {
    // Docker services required
}

#[test]
fn test_with_real_provider() {
    if std::env::var("OPENAI_API_KEY").is_err() {
        eprintln!("Skipped: requires OPENAI_API_KEY");
        return;
    }
    // Test with real API
}
```

### Migration from Stubs

**Scheduler Stub Replacement** - Current stub at line 297:
```rust
// Before (stub):
pub async fn schedule_delivery(&self, _content_id: String, _schedule: Schedule) -> Result<(), ContentDeliveryError> {
    // TODO: Implement actual scheduling
    unimplemented!("Scheduler integration pending")
}

// After (production implementation):
pub async fn schedule_delivery(&self, content_id: String, schedule: Schedule) -> Result<JobId, ContentDeliveryError> {
    let job_spec = JobSpec::new(schedule, move || {
        async move {
            self.deliver_content(&content_id).await
        }
    });

    self.scheduler.schedule_job(job_spec).await
        .map_err(|e| ContentDeliveryError::SchedulingFailed(e.to_string()))
}
```

---

## 8. Success Metrics

### Completion Metrics

1. **Configuration Coverage**:
   - ✅ 100% of garrison types supported (in_memory, sqlite)
   - ✅ 100% of arsenal types supported (stdio, sse)
   - ✅ Zero TODO comments in `agent.rs` lines 293-296

2. **Test Coverage**:
   - ✅ ≥80% unit test coverage for new configuration code
   - ✅ ≥70% integration test coverage for CLI workflows
   - ✅ All deferred tests from Epics 9, 10, 18 implemented
   - ✅ Mock provider supports all core test scenarios

3. **Infrastructure Completion**:
   - ✅ Scheduler stub replaced with production implementation
   - ✅ Scheduled jobs execute successfully in integration tests
   - ✅ Zero unimplemented!() macros in production code paths

### Quality Metrics

1. **Code Quality**:
   - ✅ `cargo clippy -- -D warnings` passes (zero warnings)
   - ✅ `cargo fmt --check` passes (formatted code)
   - ✅ `cargo test` passes (all tests green)
   - ✅ `cargo audit` passes (no security vulnerabilities)

2. **Test Reliability**:
   - ✅ All mock-based tests run successfully in CI without external dependencies
   - ✅ Service integration tests pass when Docker services available
   - ✅ Provider integration tests pass when API keys available
   - ✅ Clear skip messages for gated tests

3. **Documentation Quality**:
   - ✅ Configuration schema fully documented
   - ✅ Test execution requirements documented (Docker, API keys)
   - ✅ Error messages are actionable
   - ✅ Example configurations provided

### User Experience Metrics

1. **Configuration Usability**:
   - ✅ User can configure garrison from YAML without reading source code
   - ✅ User can configure arsenal/MCP from YAML without reading source code
   - ✅ Validation errors are clear and actionable
   - ✅ Example configs work out of the box

2. **Testing Experience**:
   - ✅ Developers can run tests locally without API keys (using mocks)
   - ✅ CI pipeline completes in reasonable time (<10 minutes)
   - ✅ Test failures provide clear debugging information
   - ✅ Integration tests are opt-in (don't block quick iteration)

---

## 9. Open Questions

### Resolved by User Answers

1. ✅ **Priority**: Configuration first, then testing (sequential approach)
2. ✅ **Mock LLM Sophistication**: Configurable mock with response types, errors, streaming
3. ✅ **Testing Scope**: Standard coverage (happy path + errors + edge cases)
4. ✅ **Scheduler Complexity**: Moderate (adapter wrapper with job management)

### Remaining Open Questions

1. **Garrison Path Validation**: Should the system auto-create parent directories for SQLite garrison paths, or require they exist?
   - **Recommendation**: Auto-create parent directories (better UX, similar to MinIO adapter behavior)

2. **Arsenal Connection Timeout**: What timeout should be used when connecting to MCP servers during configuration?
   - **Recommendation**: 10 seconds default, configurable via `arsenal.connection_timeout_seconds`

3. **Mock Provider Placement**: Should `MockLlmAdapter` live in test utilities or as a feature-flagged module?
   - **Recommendation**: Test utilities (`tests/common/mock_llm.rs`) - not production code

4. **Scheduler Job Persistence**: Should scheduled jobs persist across application restarts?
   - **Recommendation**: Phase 1 (this epic) - in-memory only. Phase 2 (future) - persistent storage

5. **Test Parallelization**: Should CLI integration tests run in parallel or sequentially?
   - **Recommendation**: Sequential for tests that modify global config, parallel for read-only tests

6. **Error Recovery**: Should garrison/arsenal initialization failures be fatal or allow agent to run with degraded capabilities?
   - **Recommendation**: Fatal errors (fail fast) - missing memory or tools is a configuration issue, not a runtime condition

---

## 10. Implementation Sequence

### Phase 1: Configuration Wiring (Week 1)

**Tasks**:
1. Implement garrison configuration parsing and validation
2. Implement arsenal configuration parsing and validation
3. Wire garrison to `PaladinBuilder` in `agent.rs` (line 293)
4. Wire arsenal to `PaladinBuilder` in `agent.rs` (line 296)
5. Write unit tests for configuration loading
6. Create example YAML configurations

**Acceptance**: Agent can be launched from YAML with full garrison and arsenal configuration

### Phase 2: Mock Provider Infrastructure (Week 1-2)

**Tasks**:
1. Design and implement `MockLlmAdapter`
2. Support configurable response modes (text, tools, streaming, errors)
3. Add invocation recording for test assertions
4. Write unit tests for mock provider
5. Document mock provider usage for test authors

**Acceptance**: Mock provider is available and documented for test use

### Phase 3: CLI Integration Tests (Week 2)

**Tasks**:
1. Implement Paladin execution test with mock provider
2. Implement Formation execution test
3. Implement Phalanx execution test
4. Implement error handling tests
5. Implement tool integration tests
6. Verify all tests run in CI without external dependencies

**Acceptance**: All deferred CLI integration tests are implemented and passing

### Phase 4: Environment Testing (Week 2)

**Tasks**:
1. Implement core functionality test suite (Tier 1)
2. Implement service integration tests with Docker gating (Tier 2)
3. Implement provider integration tests with API key gating (Tier 3)
4. Add non-interactive mode support
5. Add environment variation handling (NO_COLOR, etc.)
6. Document test execution requirements

**Acceptance**: Comprehensive test coverage across all environments and scenarios

### Phase 5: Scheduler Integration (Week 2)

**Tasks**:
1. Add `tokio-cron-scheduler` dependency
2. Design and implement `SchedulerPort` trait
3. Implement `TokioCronSchedulerAdapter`
4. Replace scheduler stub in `api_content_deliverer.rs` (line 297)
5. Write unit tests with mock scheduler
6. Write integration test with real scheduler
7. Document scheduler configuration

**Acceptance**: Time-based content delivery works with production scheduler

### Phase 6: Validation & Documentation (Week 2)

**Tasks**:
1. Run full test suite (`make test-all`)
2. Run code quality checks (`make clean-code`)
3. Verify all TODO items resolved
4. Update relevant documentation
5. Create example configurations
6. Prepare Epic 23 completion summary

**Acceptance**: All acceptance criteria met, documentation complete, zero TODOs

---

## 11. Related Documents

- **Epic Document**: `project/Milestone_3-Completion/Epic_23/epic23.md`
- **Project Plan**: `project/Milestone_3-Completion/Project_Plan_Milestone_3.md`
- **Deferred Tasks**:
  - `project/Milestone_1-MVP/Epic_9/tasks-armory-cli-tools.md` (Tasks 5.8, 5.9)
  - `project/Milestone_1-MVP/Epic_10/tasks-epic10-validation-documentation.md` (Tasks 13.4-13.6)
  - `project/Milestone_2-Missing_features/Epic_18/tasks-epic-18-cli-enhancement.md` (Tasks 9.1-9.7)
- **Architecture**:
  - `docs/Design/Design_and_Architecture.md`
  - `notes/hexagonal-arch.md`
- **Coding Standards**: `.github/copilot-instructions.md`

---

## Document History

| Version | Date | Author | Changes |
|---------|------|--------|---------|
| 1.0 | 2026-02-13 | AI Assistant | Initial PRD based on Epic 23 and user clarifications |

---

## Appendix: Deferred Work Summary

### From Epic 9 (Armory CLI Tools)

**Task 5.8** - Line 293 in `src/application/cli/commands/agent.rs`:
```rust
// TODO: Wire garrison configuration from YAML
// - Parse garrison config from loaded YAML
// - Instantiate appropriate garrison adapter (InMemory or SQLite)
// - Pass to PaladinBuilder
```

**Task 5.9** - Line 296 in `src/application/cli/commands/agent.rs`:
```rust
// TODO: Wire arsenal configuration from YAML
// - Parse MCP server configs from YAML
// - Instantiate MCP adapters (STDIO or SSE)
// - Register tools in arsenal registry
// - Pass to PaladinBuilder
```

### From Epic 10 (Validation & Documentation)

**Task 13.4** - CLI integration test for Paladin execution (deferred - requires mock provider)

**Task 13.5** - CLI integration test for Formation execution (deferred - requires mock provider)

**Task 13.6** - CLI integration test for Phalanx execution (deferred - requires mock provider)

### From Epic 18 (CLI Enhancement & Polish)

**Tasks 9.1-9.7** - End-to-end CLI testing (deferred pending infrastructure):
- 9.1 - Full user journey test (onboarding → first run)
- 9.2 - `setup-check` with real services (Docker-gated)
- 9.3 - `muster` with real LLM providers (env-var-gated)
- 9.4 - `council` command end-to-end (env-var-gated)
- 9.5 - Non-interactive mode testing (CI/CD)
- 9.6 - `NO_COLOR` environment variable testing
- 9.7 - Terminal type compatibility testing

### Infrastructure Stubs

**Line 297** in `src/infrastructure/adapters/output/api_content_deliverer.rs`:
```rust
pub async fn schedule_delivery(&self, _content_id: String, _schedule: Schedule) -> Result<(), ContentDeliveryError> {
    // TODO: Implement actual scheduling with tokio-cron-scheduler
    unimplemented!("Scheduler integration pending")
}
```

---

**End of Document**
