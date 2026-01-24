# Arsenal Tool System - Implementation Task List

## Relevant Files

### Domain Layer (Core)
- `src/core/platform/container/arsenal.rs` - Arsenal domain entities (Armament, ArmamentCall, ArmamentResult)
- `src/core/platform/container/mod.rs` - Module exports for arsenal

### Application Layer
- `src/application/ports/output/arsenal_port.rs` - ArsenalPort and ArsenalRegistry trait definitions
- `src/application/ports/output/mod.rs` - Port module exports
- `src/application/use_cases/arsenal/mod.rs` - Arsenal use case module
- `src/application/use_cases/arsenal/arsenal_registry_service.rs` - Registry implementation
- `src/application/use_cases/arsenal/arsenal_execution_service.rs` - Tool execution service
- `src/application/use_cases/mod.rs` - Use cases module exports

### Infrastructure Layer
- `src/infrastructure/adapters/arsenal/mod.rs` - Arsenal adapter module
- `src/infrastructure/adapters/arsenal/mcp_protocol.rs` - MCP protocol types and client
- `src/infrastructure/adapters/arsenal/mcp_stdio_adapter.rs` - STDIO transport implementation
- `src/infrastructure/adapters/arsenal/mcp_sse_adapter.rs` - SSE transport implementation
- `src/infrastructure/adapters/arsenal/tool_result_formatter.rs` - Tool result formatting for context
- `src/infrastructure/adapters/mod.rs` - Adapter module exports

### Configuration
- `src/config/application_settings.rs` - Arsenal configuration structure
- `config.yml` - Arsenal configuration section
- `config.test.yml` - Test configuration for arsenal

### Builder Integration
- `src/application/use_cases/paladin/paladin_builder.rs` - Add arsenal methods to builder

### Tests
- `tests/unit/arsenal_domain_test.rs` - Domain entity tests
- `tests/unit/arsenal_port_test.rs` - Port trait tests
- `tests/integration/mcp_stdio_test.rs` - STDIO adapter integration tests
- `tests/integration/mcp_sse_test.rs` - SSE adapter integration tests
- `tests/functional/paladin_tool_invocation_test.rs` - End-to-end tool execution tests

### Examples
- `examples/arsenal_stdio_tools.rs` - Example using STDIO MCP server
- `examples/arsenal_sse_tools.rs` - Example using SSE MCP server

### Dependencies
- `Cargo.toml` - Add jsonschema, tokio process support dependencies

### Notes

- Follow TDD: Write tests before implementation
- All tests should use `cargo test` and pass `cargo clippy` with no warnings
- Maintain hexagonal architecture boundaries (core → application → infrastructure)
- Use `thiserror` for error types
- All public APIs need rustdoc comments
- Integration tests should use mock MCP servers, not real external services

## Instructions for Completing Tasks

**IMPORTANT:** As you complete each task, you must check it off in this markdown file by changing `- [ ]` to `- [x]`. This helps track progress and ensures you don't skip any steps.

Example:
- `- [ ] 1.1 Read file` → `- [x] 1.1 Read file` (after completing)

Update the file after completing each sub-task, not just after completing an entire parent task.

**Before marking a parent task complete:**
1. Ensure all sub-tasks are checked `[x]`
2. Run `cargo test` for the relevant module
3. Run `cargo clippy` and fix all warnings
4. Run `cargo fmt` to format code

## Tasks

- [x] 0.0 Create feature branch
  - [x] 0.1 Create and checkout a new branch: `git checkout -b feature/epic3-arsenal-tool-system`
  - [x] 0.2 Verify you're on the correct branch: `git branch --show-current`

- [x] 1.0 Domain Layer - Tool Definitions (FR-1)
  - [x] 1.1 Create `src/core/platform/container/arsenal.rs` file
  - [x] 1.2 Define `Armament` struct with name, description, parameters (JSON Schema), and required_params fields
  - [x] 1.3 Add serialization derives (Debug, Clone, Serialize, Deserialize) to `Armament`
  - [x] 1.4 Define `ArmamentCall` struct with tool_name, arguments (HashMap<String, Value>), and call_id (Uuid)
  - [x] 1.5 Add serialization derives to `ArmamentCall`
  - [x] 1.6 Define `ArmamentResult` struct with call_id, success, optional output, optional error, execution_time_ms
  - [x] 1.7 Add serialization derives to `ArmamentResult`
  - [x] 1.8 Define `ArsenalError` enum using thiserror with variants: ToolNotFound, InvalidArguments, Timeout, ProtocolError, TransportError
  - [x] 1.9 Export arsenal module in `src/core/platform/container/mod.rs`
  - [x] 1.10 Create `tests/unit/arsenal_domain_test.rs` file
  - [x] 1.11 Write unit test `test_armament_serialization` for Armament struct
  - [x] 1.12 Write unit test `test_armament_call_creation` for ArmamentCall
  - [x] 1.13 Write unit test `test_armament_result_success` for successful result
  - [x] 1.14 Write unit test `test_armament_result_failure` for failed result
  - [x] 1.15 Write unit test `test_arsenal_error_display` for error messages
  - [x] 1.16 Run `cargo test arsenal_domain` and verify all tests pass
  - [x] 1.17 Run `cargo clippy -- -D warnings` and fix any issues

- [x] 2.0 Application Layer - Arsenal Ports (FR-2)
  - [x] 2.1 Create `src/application/ports/output/arsenal_port.rs` file
  - [x] 2.2 Define `ArsenalPort` trait with async methods: list_armaments(), invoke(), validate_call()
  - [x] 2.3 Add `Send + Sync` bounds to `ArsenalPort` trait
  - [x] 2.4 Add rustdoc comments to all `ArsenalPort` methods
  - [x] 2.5 Define `ArsenalRegistry` trait with methods: register(), unregister(), get()
  - [x] 2.6 Add `Send + Sync` bounds to `ArsenalRegistry` trait
  - [x] 2.7 Add rustdoc comments to all `ArsenalRegistry` methods
  - [x] 2.8 Export arsenal_port in `src/application/ports/output/mod.rs`
  - [x] 2.9 Create `src/application/use_cases/arsenal/mod.rs` directory and file
  - [x] 2.10 Create `src/application/use_cases/arsenal/arsenal_registry_service.rs`
  - [x] 2.11 Implement `ArsenalRegistryService` struct with HashMap storage for armaments
  - [x] 2.12 Implement `ArsenalRegistry` trait for `ArsenalRegistryService`
  - [x] 2.13 Create `src/application/use_cases/arsenal/arsenal_execution_service.rs`
  - [x] 2.14 Implement `ArsenalExecutionService` struct that holds registry and transport references
  - [x] 2.15 Implement `ArsenalPort` trait for `ArsenalExecutionService`
  - [x] 2.16 Export arsenal module in `src/application/use_cases/mod.rs`
  - [x] 2.17 Create `tests/unit/arsenal_port_test.rs` file
  - [x] 2.18 Write unit test `test_registry_register_tool` for registering a tool
  - [x] 2.19 Write unit test `test_registry_get_tool` for retrieving a tool
  - [x] 2.20 Write unit test `test_registry_unregister_tool` for removing a tool
  - [x] 2.21 Write unit test `test_validate_call_success` for valid tool call
  - [x] 2.22 Write unit test `test_validate_call_missing_required_param` for invalid call
  - [x] 2.23 Run `cargo test arsenal_port` and verify all tests pass
  - [x] 2.24 Run `cargo clippy -- -D warnings` and fix any issues

- [x] 3.0 MCP Protocol Core Implementation (FR-3)
  - [x] 3.1 Create `src/infrastructure/adapters/arsenal/mod.rs` directory and file
  - [x] 3.2 Create `src/infrastructure/adapters/arsenal/mcp_protocol.rs` file
  - [x] 3.3 Define `MCPMessage` enum for JSON-RPC 2.0 message types (Request, Response, Notification)
  - [x] 3.4 Define `MCPRequest` struct with jsonrpc, id, method, params fields
  - [x] 3.5 Define `MCPResponse` struct with jsonrpc, id, result, error fields
  - [x] 3.6 Define `MCPError` struct with code, message, data fields
  - [x] 3.7 Define `MCPCapabilities` struct for server capability reporting
  - [x] 3.8 Define `MCPTransport` trait with async methods: send(), receive()
  - [x] 3.9 Add `Send + Sync` bounds to `MCPTransport` trait
  - [x] 3.10 Implement `MCPClient` struct with transport field and capabilities
  - [x] 3.11 Implement `MCPClient::new()` constructor
  - [x] 3.12 Implement `MCPClient::discover_tools()` method using tools/list request
  - [x] 3.13 Implement `MCPClient::invoke_tool()` method using tools/call request
  - [x] 3.14 Add rustdoc comments to all MCP types and methods
  - [x] 3.15 Export mcp_protocol in arsenal adapter module
  - [x] 3.16 Create `tests/unit/mcp_protocol_test.rs` file
  - [x] 3.17 Write unit test `test_mcp_request_serialization` for request format
  - [x] 3.18 Write unit test `test_mcp_response_deserialization` for response parsing
  - [x] 3.19 Write unit test `test_mcp_error_handling` for error responses
  - [x] 3.20 Write unit test `test_mcp_capabilities_parsing` for capabilities
  - [x] 3.21 Run `cargo test mcp_protocol` and verify all tests pass
  - [x] 3.22 Run `cargo clippy -- -D warnings` and fix any issues

- [x] 4.0 STDIO Transport Adapter (FR-4)
  - [x] 4.1 Create `src/infrastructure/adapters/arsenal/mcp_stdio_adapter.rs` file
  - [x] 4.2 Define `MCPStdioAdapter` struct with command, args, process (Option<Child>) fields
  - [x] 4.3 Implement `MCPStdioAdapter::new()` constructor
  - [x] 4.4 Implement `MCPStdioAdapter::connect()` async method to spawn process
  - [x] 4.5 Implement process spawning with stdin/stdout/stderr piped
  - [x] 4.6 Implement `MCPTransport` trait for `MCPStdioAdapter`
  - [x] 4.7 Implement `send()` method to write JSON to stdin
  - [x] 4.8 Implement `receive()` method to read JSON from stdout
  - [x] 4.9 Implement `Drop` trait for cleanup on shutdown
  - [x] 4.10 Add process kill and wait in Drop implementation
  - [x] 4.11 Add error handling for process spawn failures
  - [x] 4.12 Add error handling for pipe communication errors
  - [x] 4.13 Add rustdoc comments to all methods
  - [x] 4.14 Export mcp_stdio_adapter in arsenal adapter module
  - [x] 4.15 Create `tests/integration/mcp_stdio_test.rs` file
  - [x] 4.16 Create a mock STDIO MCP server script for testing (Python/Node.js)
  - [x] 4.17 Write integration test `test_stdio_connect` for connection
  - [x] 4.18 Write integration test `test_stdio_discover_tools` for tool discovery
  - [x] 4.19 Write integration test `test_stdio_invoke_tool` for tool invocation
  - [x] 4.20 Write integration test `test_stdio_cleanup` for process cleanup
  - [x] 4.21 Run `cargo test mcp_stdio` and verify all tests pass
  - [x] 4.22 Run `cargo clippy -- -D warnings` and fix any issues

- [ ] 5.0 SSE Transport Adapter (FR-5)
  - [ ] 5.1 Create `src/infrastructure/adapters/arsenal/mcp_sse_adapter.rs` file
  - [ ] 5.2 Define `MCPSseAdapter` struct with endpoint, client (reqwest::Client) fields
  - [ ] 5.3 Implement `MCPSseAdapter::new()` constructor
  - [ ] 5.4 Implement `MCPSseAdapter::connect()` async method with SSE connection
  - [ ] 5.5 Configure reqwest client with timeout settings
  - [ ] 5.6 Implement `MCPTransport` trait for `MCPSseAdapter`
  - [ ] 5.7 Implement `send()` method using HTTP POST to endpoint
  - [ ] 5.8 Implement `receive()` method using SSE stream reading
  - [ ] 5.9 Implement retry logic with exponential backoff (3 retries, 1s, 2s, 4s)
  - [ ] 5.10 Add connection timeout handling (default 10 seconds)
  - [ ] 5.11 Add error handling for HTTP errors
  - [ ] 5.12 Add error handling for SSE parsing errors
  - [ ] 5.13 Add rustdoc comments to all methods
  - [ ] 5.14 Export mcp_sse_adapter in arsenal adapter module
  - [ ] 5.15 Create `tests/integration/mcp_sse_test.rs` file
  - [ ] 5.16 Create a mock SSE MCP server using mockito or similar
  - [ ] 5.17 Write integration test `test_sse_connect` for connection
  - [ ] 5.18 Write integration test `test_sse_discover_tools` for tool discovery
  - [ ] 5.19 Write integration test `test_sse_invoke_tool` for tool invocation
  - [ ] 5.20 Write integration test `test_sse_retry_on_failure` for retry logic
  - [ ] 5.21 Write integration test `test_sse_connection_timeout` for timeout
  - [ ] 5.22 Run `cargo test mcp_sse` and verify all tests pass
  - [ ] 5.23 Run `cargo clippy -- -D warnings` and fix any issues

- [ ] 6.0 PaladinBuilder Integration & Configuration (FR-6)
  - [ ] 6.1 Read existing `src/application/use_cases/paladin/paladin_builder.rs` file
  - [ ] 6.2 Add `arsenal_registry: Option<Arc<dyn ArsenalRegistry>>` field to PaladinBuilder
  - [ ] 6.3 Add `mcp_servers: Vec<MCPServerConfig>` field to PaladinBuilder
  - [ ] 6.4 Implement `add_mcp_stdio(command: &str, args: &[&str])` method
  - [ ] 6.5 Implement `add_mcp_sse(name: &str, endpoint: &str)` method
  - [ ] 6.6 Implement `with_arsenal_registry(registry: Arc<dyn ArsenalRegistry>)` method
  - [ ] 6.7 Update `build()` method to initialize MCP servers and discover tools
  - [ ] 6.8 Add validation for MCP server connections in build()
  - [ ] 6.9 Update `config.yml` with arsenal configuration section
  - [ ] 6.10 Add `default_timeout_seconds`, `max_concurrent_tools` config fields
  - [ ] 6.11 Add `mcp_servers` array with name, type, command, args, endpoint fields
  - [ ] 6.12 Update `src/config/application_settings.rs` to include ArsenalConfig struct
  - [ ] 6.13 Define `ArsenalConfig` struct with timeout, concurrency, mcp_servers fields
  - [ ] 6.14 Define `MCPServerConfig` struct with name, server_type, command, args, endpoint
  - [ ] 6.15 Update ApplicationSettings deserialization to include arsenal field
  - [ ] 6.16 Update `config.test.yml` with test arsenal configuration
  - [ ] 6.17 Write unit test `test_builder_add_mcp_stdio` for STDIO addition
  - [ ] 6.18 Write unit test `test_builder_add_mcp_sse` for SSE addition
  - [ ] 6.19 Write unit test `test_builder_validates_mcp_connections` for validation
  - [ ] 6.20 Write unit test `test_arsenal_config_loading` for config deserialization
  - [ ] 6.21 Run `cargo test paladin_builder` and verify all tests pass
  - [ ] 6.22 Run `cargo clippy -- -D warnings` and fix any issues

- [ ] 7.0 Resource Controls - Timeout & Concurrency (FR-7)
  - [ ] 7.1 Create `src/infrastructure/adapters/arsenal/resource_controls.rs` file
  - [ ] 7.2 Define `TimeoutWrapper` struct with duration field
  - [ ] 7.3 Implement `TimeoutWrapper::new(duration: Duration)` constructor
  - [ ] 7.4 Implement `TimeoutWrapper::execute()` method using tokio::time::timeout
  - [ ] 7.5 Define `ConcurrencyLimiter` struct with semaphore field
  - [ ] 7.6 Implement `ConcurrencyLimiter::new(max_concurrent: usize)` constructor
  - [ ] 7.7 Implement `ConcurrencyLimiter::acquire()` method for permit acquisition
  - [ ] 7.8 Update `ArsenalExecutionService` to include TimeoutWrapper and ConcurrencyLimiter
  - [ ] 7.9 Modify `invoke()` method to acquire permit before execution
  - [ ] 7.10 Wrap tool invocation with timeout in invoke() method
  - [ ] 7.11 Return timeout error if execution exceeds duration
  - [ ] 7.12 Add execution time tracking using tokio::time::Instant
  - [ ] 7.13 Update ArmamentResult with actual execution_time_ms
  - [ ] 7.14 Add rustdoc comments to resource control types
  - [ ] 7.15 Export resource_controls in arsenal adapter module
  - [ ] 7.16 Create `tests/unit/resource_controls_test.rs` file
  - [ ] 7.17 Write unit test `test_timeout_within_limit` for successful execution
  - [ ] 7.18 Write unit test `test_timeout_exceeds_limit` for timeout error
  - [ ] 7.19 Write unit test `test_concurrency_limit_enforced` for max concurrent
  - [ ] 7.20 Write unit test `test_concurrency_queuing` for waiting on permits
  - [ ] 7.21 Run `cargo test resource_controls` and verify all tests pass
  - [ ] 7.22 Run `cargo clippy -- -D warnings` and fix any issues

- [ ] 8.0 Error Handling & Context Injection (FR-8, FR-9)
  - [ ] 8.1 Create `src/infrastructure/adapters/arsenal/tool_result_formatter.rs` file
  - [ ] 8.2 Define `ToolResultFormatter` struct
  - [ ] 8.3 Implement `format_result()` method to convert ArmamentResult to text
  - [ ] 8.4 Format successful results with tool name, arguments, output, execution time
  - [ ] 8.5 Format failed results with tool name, arguments, error message, execution time
  - [ ] 8.6 Ensure formatted output is LLM-readable (structured markdown-like format)
  - [ ] 8.7 Add rustdoc comments to formatter methods
  - [ ] 8.8 Export tool_result_formatter in arsenal adapter module
  - [ ] 8.9 Read existing Paladin execution service implementation
  - [ ] 8.10 Integrate ToolResultFormatter into Paladin execution loop
  - [ ] 8.11 Detect tool calls in LLM responses (function calling format)
  - [ ] 8.12 Invoke tools via ArsenalPort when tool calls are detected
  - [ ] 8.13 Format tool results using ToolResultFormatter
  - [ ] 8.14 Inject formatted results into conversation context
  - [ ] 8.15 Continue execution loop after tool invocation
  - [ ] 8.16 Add logging for tool failures at warn level
  - [ ] 8.17 Add logging for tool success at debug level
  - [ ] 8.18 Implement error conversion from ArsenalError to PaladinError
  - [ ] 8.19 Create `tests/unit/tool_result_formatter_test.rs` file
  - [ ] 8.20 Write unit test `test_format_success_result` for successful formatting
  - [ ] 8.21 Write unit test `test_format_error_result` for error formatting
  - [ ] 8.22 Create `tests/integration/context_injection_test.rs` file
  - [ ] 8.23 Write integration test `test_tool_call_detection` for detecting calls
  - [ ] 8.24 Write integration test `test_tool_invocation_and_injection` for full flow
  - [ ] 8.25 Write integration test `test_paladin_continues_after_tool_failure` for resilience
  - [ ] 8.26 Run `cargo test tool_result` and verify all tests pass
  - [ ] 8.27 Run `cargo clippy -- -D warnings` and fix any issues

- [ ] 9.0 Comprehensive Testing & Documentation
  - [ ] 9.1 Create `tests/functional/paladin_tool_invocation_test.rs` file
  - [ ] 9.2 Write end-to-end test `test_paladin_with_stdio_tool` using mock STDIO server
  - [ ] 9.3 Write end-to-end test `test_paladin_with_sse_tool` using mock SSE server
  - [ ] 9.4 Write end-to-end test `test_multiple_tool_invocations` for sequential calls
  - [ ] 9.5 Write end-to-end test `test_concurrent_tool_invocations` for parallel calls
  - [ ] 9.6 Write end-to-end test `test_tool_timeout_handling` for timeout scenario
  - [ ] 9.7 Write end-to-end test `test_tool_failure_resilience` for failure recovery
  - [ ] 9.8 Create `examples/arsenal_stdio_tools.rs` file
  - [ ] 9.9 Implement example with PaladinBuilder using add_mcp_stdio()
  - [ ] 9.10 Add example invocation with a common STDIO MCP server (e.g., calculator)
  - [ ] 9.11 Create `examples/arsenal_sse_tools.rs` file
  - [ ] 9.12 Implement example with PaladinBuilder using add_mcp_sse()
  - [ ] 9.13 Add example invocation with a web-based MCP server
  - [ ] 9.14 Update Cargo.toml with new dependencies: jsonschema, tokio process features
  - [ ] 9.15 Add feature flag for arsenal if desired: `arsenal = ["jsonschema"]`
  - [ ] 9.16 Review all public APIs and ensure rustdoc comments are complete
  - [ ] 9.17 Add module-level rustdoc for arsenal domain module
  - [ ] 9.18 Add module-level rustdoc for arsenal ports module
  - [ ] 9.19 Add module-level rustdoc for arsenal adapters module
  - [ ] 9.20 Create documentation file `docs/ARSENAL.md` with usage guide
  - [ ] 9.21 Document MCP protocol compliance and requirements in ARSENAL.md
  - [ ] 9.22 Document configuration options in ARSENAL.md
  - [ ] 9.23 Document tool authoring best practices in ARSENAL.md
  - [ ] 9.24 Run full test suite: `cargo test --all-features`
  - [ ] 9.25 Run clippy on entire codebase: `cargo clippy --all-features -- -D warnings`
  - [ ] 9.26 Run cargo fmt: `cargo fmt --all`
  - [ ] 9.27 Generate documentation: `cargo doc --no-deps --open`
  - [ ] 9.28 Review generated docs for completeness
  - [ ] 9.29 Run `make test-all` if available
  - [ ] 9.30 Commit all changes with message: "feat: implement Arsenal Tool System (Epic 3)"
  - [ ] 9.31 Push feature branch: `git push -u origin feature/epic3-arsenal-tool-system`

---

**Implementation Notes:**

- Follow TDD strictly - write tests before implementation
- Maintain hexagonal architecture - no domain dependencies on infrastructure
- Use existing error handling patterns (thiserror)
- All async code should be tokio-based
- Mock external dependencies in tests (no real MCP servers in CI)
- Keep commits atomic and well-described
- Reference Epic 3 PRD for detailed requirements

**Testing Strategy:**
- Unit tests: Domain types, ports, protocol logic
- Integration tests: MCP adapters with mock servers
- Functional tests: End-to-end Paladin tool execution

**Success Criteria:**
- ≥ 80% unit test coverage
- All integration tests pass
- Zero clippy warnings
- Complete rustdoc for all public APIs
- Examples run successfully
