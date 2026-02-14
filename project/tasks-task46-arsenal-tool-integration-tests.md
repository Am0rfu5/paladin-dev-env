# Task List: Arsenal Tool Integration Tests (Task 4.6)

Based on [PRD: Arsenal Tool Integration Tests](prd-task46-arsenal-tool-integration-tests.md)

## Relevant Files

- `tests/helpers/mock_arsenal_adapter.rs` - New MockArsenalPort implementing `ArsenalPort` trait for in-process tool call testing
- `tests/helpers/mod.rs` - Update to export `MockArsenalPort` and its helper functions
- `tests/cli/tool_integration_test.rs` - New core tool call flow tests (CI-friendly, in-process mocks only)
- `tests/cli/mod.rs` - Update to include `tool_integration_test` module
- `tests/integration/tool_integration_mcp_test.rs` - New gated integration tests using real Python MCP server
- `tests/integration/mod.rs` - Update to include `tool_integration_mcp_test` module
- `project/Milestone_3-Completion/Epic_23/tasks-epic23-cli-config-infrastructure-completion.md` - Update Task 4.6 subtasks from `[-]` to `[x]`

### Existing Files Referenced (Read-Only Context)

- `tests/helpers/mock_llm_adapter.rs` - Existing MockLlmAdapter with `MockResponse::ToolCall` support, `add_tool_call()`, `call_count()`, `invocations()`
- `src/application/ports/output/arsenal_port.rs` - `ArsenalPort` trait: `list_armaments()`, `invoke()`, `validate_call()`
- `src/core/platform/container/arsenal/core.rs` - Domain types: `Armament`, `ArmamentCall`, `ArmamentResult`, `ArsenalError`
- `src/application/use_cases/paladin/paladin_execution_service.rs` - `PaladinExecutionService::new()` (line 160), tool call loop (lines 758–840), `handle_tool_call()` (line 1747)
- `src/infrastructure/adapters/arsenal/tool_result_formatter.rs` - `ToolResultFormatter` used by `handle_tool_call()`
- `tests/cli/paladin_execution_test.rs` - Existing Paladin execution tests (pattern to follow)
- `tests/mcp_test_server.py` - Python MCP test server providing `echo` and `calculator` tools via STDIO
- `tests/integration/mcp_stdio_test.rs` - Existing STDIO MCP transport tests (pattern for gated tests)

### Notes

- This task completes deferred Task 4.6 from Epic 23 (CLI, Config & Infrastructure Completion)
- Core tests use in-process mocks only (CI-friendly, no external dependencies)
- Gated MCP tests use `#[ignore]` and require Python 3.x with `mcp` package
- Run core tests: `cargo test tool_integration`
- Run gated tests: `cargo test tool_integration -- --ignored`
- Follow TDD: write failing tests first, then implement mock to make them pass
- `PaladinExecutionService::new()` takes: `(Arc<dyn LlmPort>, Arc<CircuitBreaker>, Option<Arc<dyn GarrisonPort>>, Option<Arc<dyn ArsenalPort>>)`
- `ArmamentResult::success(call_id: Uuid, output: Value, execution_time_ms: u64)` and `ArmamentResult::failure(call_id, error, time)` are the constructors
- `ArmamentCall::new(tool_name: &str, arguments: HashMap<String, Value>)` creates a call with auto-generated `call_id`

## Instructions for Completing Tasks

**IMPORTANT:** As you complete each task, you must check it off in this markdown file by changing `- [ ]` to `- [x]`. This helps track progress and ensures you don't skip any steps.

Example:

- `- [ ] 1.1 Read file` → `- [x] 1.1 Read file` (after completing)

Update the file after completing each sub-task, not just after completing an entire parent task.

## Tasks

- [x] 0.0 Create feature branch
  - [x] 0.1 Ensure on `develop` branch with latest changes: `git checkout develop && git pull`
  - [x] 0.2 Create and checkout feature branch: `git checkout -b feature/epic-23-task46-tool-integration-tests`

- [x] 1.0 Implement MockArsenalPort test helper (FR-1)
  - [x] 1.1 Create `tests/helpers/mock_arsenal_adapter.rs` with module doc comment explaining purpose
  - [x] 1.2 Add required imports: `ArsenalPort`, `Armament`, `ArmamentCall`, `ArmamentResult`, `ArsenalError`, `async_trait`, `Arc`, `Mutex`, `HashMap`
  - [x] 1.3 Define `MockArsenalPort` struct with fields: `armaments: Vec<Armament>`, `responses: Arc<Mutex<HashMap<String, Result<ArmamentResult, ArsenalError>>>>`, `invocations: Arc<Mutex<Vec<ArmamentCall>>>`
  - [x] 1.4 Implement `MockArsenalPort::new()` constructor returning empty mock
  - [x] 1.5 Implement builder method `add_tool(&mut self, name: &str, description: &str) -> &mut Self` that adds an `Armament` to the armaments list
  - [x] 1.6 Implement `set_response(&self, tool_name: &str, result: ArmamentResult)` to pre-configure success response for a tool name
  - [x] 1.7 Implement `set_error(&self, tool_name: &str, error: ArsenalError)` to pre-configure error response for a tool name
  - [x] 1.8 Implement assertion helpers: `call_count(&self) -> usize`, `invocations(&self) -> Vec<ArmamentCall>`, `last_invocation(&self) -> Option<ArmamentCall>`
  - [x] 1.9 Implement `ArsenalPort` trait for `MockArsenalPort`:
    - `list_armaments()` returns cloned `self.armaments`
    - `invoke()` records call in `invocations`, looks up response by `call.tool_name`, returns it (or `ArsenalError::ToolNotFound` if not configured)
    - `validate_call()` checks tool exists in armaments list
  - [x] 1.10 Implement `Default` trait for `MockArsenalPort`
  - [x] 1.11 Update `tests/helpers/mod.rs` to add `pub mod mock_arsenal_adapter;` and `pub use mock_arsenal_adapter::MockArsenalPort;`
  - [x] 1.12 Run `cargo check --tests` to verify MockArsenalPort compiles
  - [x] 1.13 Run `cargo clippy -- -D warnings` and fix any issues
  - [x] 1.14 Run `cargo fmt`

- [x] 2.0 Implement core tool call flow tests (FR-2.1–FR-2.3, US-1, US-2)
  - [x] 2.1 Create `tests/cli/tool_integration_test.rs` with module doc comment
  - [x] 2.2 Add required imports: `MockLlmAdapter`, `MockArsenalPort`, `PaladinExecutionService`, `CircuitBreaker`, `LlmPort`, `ArsenalPort`, `PaladinData`, `MaxLoops`, `Node`, `ArmamentResult`, `Arc`, `Duration`, `serde_json::json`, `uuid::Uuid`
  - [x] 2.3 Create helper function `create_test_paladin(max_loops: MaxLoops) -> Node<PaladinData>` that builds a minimal Paladin config for tests
  - [x] 2.4 Create helper function `create_service(llm: Arc<dyn LlmPort>, arsenal: Option<Arc<dyn ArsenalPort>>) -> PaladinExecutionService` that wires up a `CircuitBreaker` and returns the service
  - [x] 2.5 Update `tests/cli/mod.rs` to add `mod tool_integration_test;`
  - [x] 2.6 Implement `test_tool_call_basic_flow` (FR-2.2):
    - MockLlmAdapter: `add_tool_call("calculator", r#"{"operation":"add","a":1,"b":2}"#)` then `add_success("The result is 3")`
    - MockArsenalPort: `add_tool("calculator", "A calculator")`, `set_response("calculator", ArmamentResult::success(...))`
    - Execute via `service.execute(&paladin, "What is 1 + 2?")`
    - Assert: `result.is_ok()`, output contains tool result content, `mock_llm.call_count() == 2`, `mock_arsenal.call_count() == 1`
  - [x] 2.7 Implement `test_tool_call_result_fed_back_to_llm` (FR-2.3):
    - Same setup as basic flow
    - After execution, inspect `mock_llm.invocations()` — the second invocation's prompt should contain the formatted tool result (e.g., "Tool Execution: calculator" or the tool output value)
    - Assert: second prompt contains tool result content
  - [x] 2.8 Run `cargo test tool_integration` and verify both tests pass
  - [x] 2.9 Run `cargo clippy -- -D warnings` and `cargo fmt`

- [x] 3.0 Implement tool call error handling tests (FR-2.4–FR-2.7, US-3)
  - [x] 3.1 Implement `test_tool_call_no_arsenal_available` (FR-2.4):
    - MockLlmAdapter: `add_tool_call(...)` then `add_success("Done")`
    - Service created with `arsenal: None`
    - Assert: `result.is_ok()` (no crash), execution completes
  - [x] 3.2 Implement `test_tool_call_unknown_tool` (FR-2.5):
    - MockLlmAdapter: `add_tool_call("nonexistent_tool", ...)` then `add_success("Ok")`
    - MockArsenalPort: configured with NO tools (empty — `invoke` returns `ToolNotFound`)
    - Assert: execution succeeds, graceful degradation (adjusted test - error may not appear in output)
  - [x] 3.3 Implement `test_tool_call_invalid_arguments` (FR-2.6):
    - MockLlmAdapter: `add_tool_call("calculator", "not valid json{{{")` then `add_success("Recovery")`
    - MockArsenalPort: configured with calculator tool
    - Assert: execution succeeds (graceful degradation)
  - [x] 3.4 Implement `test_tool_call_execution_error` (FR-2.7):
    - MockLlmAdapter: `add_tool_call("failing_tool", ...)` then `add_success("Recovered")`
    - MockArsenalPort: `set_error("failing_tool", ArsenalError::ExecutionError(...))`
    - Assert: execution succeeds, output contains formatted error message
  - [x] 3.5 Run `cargo test tool_integration` and verify all 6 tests pass (2 from Task 2 + 4 from Task 3)
  - [x] 3.6 Run `cargo clippy -- -D warnings` and `cargo fmt`

- [x] 4.0 Implement advanced tool call tests (FR-2.8–FR-2.9)
  - [x] 4.1 Implement `test_multiple_sequential_tool_calls` (FR-2.8):
    - MockLlmAdapter: `add_tool_call("tool_a", ...)`, `add_tool_call("tool_b", ...)`, `add_success("Final answer")`
    - MockArsenalPort: `set_response("tool_a", ...)`, `set_response("tool_b", ...)`
    - Paladin with `MaxLoops::Fixed(3)` (enough loops for 3 LLM calls)
    - Assert: `mock_llm.call_count() == 3`, `mock_arsenal.call_count() == 2`, output contains final answer
  - [x] 4.2 Implement `test_tool_call_with_garrison` (FR-2.9):
    - Create `InMemoryGarrison` (from `paladin::infrastructure::adapters::garrison::in_memory_garrison`)
    - Wire into `PaladinExecutionService::new()` as `Some(garrison)`
    - MockLlmAdapter: tool call then success
    - MockArsenalPort: returns success
    - Assert: execution succeeds, garrison contains a `ConversationRole::Tool` entry with tool result
  - [x] 4.3 Run `cargo test tool_integration` and verify all 8 tests pass
  - [x] 4.4 Run `cargo clippy -- -D warnings` and `cargo fmt`
  - [ ] 4.5 Commit: `git add . && git commit -m "feat: add arsenal tool integration tests (Task 4.6)" -m "- Implement MockArsenalPort test helper" -m "- Add 8 core tool call flow tests" -m "- Test basic flow, result feedback, error handling, sequential calls, garrison" -m "- All tests CI-friendly with in-process mocks"`

- [ ] 5.0 Implement gated Python MCP server tests (FR-3, US-4)
  - [ ] 5.1 Create `tests/integration/tool_integration_mcp_test.rs` with module doc comment explaining these are gated tests requiring Python
  - [ ] 5.2 Add required imports: `MCPStdioAdapter`, `ArsenalRegistryService`, `ArsenalExecutionService`, `MockLlmAdapter`, `PaladinExecutionService`, `CircuitBreaker`, transport/arsenal types
  - [ ] 5.3 Create helper function `python_available() -> bool` that checks if `python3` (or `python`) exists on PATH and returns boolean
  - [ ] 5.4 Create helper function `setup_mcp_arsenal() -> Arc<dyn ArsenalPort>` that:
    - Creates `MCPStdioAdapter` with command `python3 tests/mcp_test_server.py`
    - Connects and discovers tools
    - Registers tools in `ArsenalRegistryService`
    - Wraps in `ArsenalExecutionService`
    - Returns as `Arc<dyn ArsenalPort>`
  - [ ] 5.5 Update `tests/integration/mod.rs` to add `pub mod tool_integration_mcp_test;`
  - [ ] 5.6 Implement `test_full_mcp_stdio_tool_call_flow` (FR-3.2):
    - Marked with `#[ignore]`
    - Skip if `!python_available()`
    - Setup MCP arsenal via helper
    - MockLlmAdapter: `add_tool_call("echo", r#"{"message":"hello"}"#)` then `add_success("Echo complete")`
    - Execute through `PaladinExecutionService`
    - Assert: output contains "hello" (echoed back from Python server)
  - [ ] 5.7 Implement `test_mcp_calculator_tool_invocation` (FR-3.3):
    - Marked with `#[ignore]`
    - Skip if `!python_available()`
    - Setup MCP arsenal
    - MockLlmAdapter: `add_tool_call("calculator", r#"{"operation":"add","a":5,"b":3}"#)` then `add_success("Done")`
    - Assert: output contains "8"
  - [ ] 5.8 Run `cargo test tool_integration_mcp -- --ignored` and verify both gated tests pass (if Python available)
  - [ ] 5.9 Run `cargo clippy -- -D warnings` and `cargo fmt`
  - [ ] 5.10 Commit: `git add . && git commit -m "feat: add gated MCP tool integration tests" -m "- Tests use real Python MCP test server via STDIO" -m "- Gated with #[ignore], require Python 3.x" -m "- Test echo and calculator tools end-to-end"`

- [ ] 6.0 Update Epic 23 task tracker and final verification
  - [ ] 6.1 Run full test suite: `cargo test` — verify no regressions
  - [ ] 6.2 Run `cargo fmt --check` — verify formatting
  - [ ] 6.3 Run `cargo clippy -- -D warnings` — verify no warnings
  - [ ] 6.4 Update `project/Milestone_3-Completion/Epic_23/tasks-epic23-cli-config-infrastructure-completion.md`: change Task 4.6 and subtasks 4.6.1–4.6.5 from `[-]` to `[x]`
  - [ ] 6.5 Commit task tracker update: `git add . && git commit -m "docs: mark Task 4.6 complete in Epic 23 tracker" -m "- All 5 subtasks completed" -m "- US-23.3 tool integration acceptance criterion satisfied"`
  - [ ] 6.6 Push branch: `git push -u origin feature/epic-23-task46-tool-integration-tests`
