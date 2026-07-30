# PRD: Arsenal Tool Integration Tests (Task 4.6)

## Introduction/Overview

Task 4.6 of Epic 23 (CLI, Config & Infrastructure Completion) was deferred during initial implementation with the note "requires MCP infrastructure from future tasks." That infrastructure now exists — the full Arsenal pipeline (MCP adapters, ArsenalExecutionService, ArsenalRegistryService) is implemented, `MockLlmAdapter` supports tool call responses, and `PaladinExecutionService` contains a complete tool call loop. However, **no test currently exercises the end-to-end flow** where an LLM response triggers a tool invocation via Arsenal, and the tool result is fed back into the Paladin's execution context.

This PRD defines the work needed to close that gap and fulfill the Epic 23 PRD acceptance criterion: *"Tool integration: Test arsenal tool invocation through mock provider"* (US-23.3, FR-23.3.5).

### Problem Statement

The Paladin tool call loop in `PaladinExecutionService.execute()` is a critical code path that:

1. Detects `function_call` in an LLM response
2. Parses arguments into a `HashMap<String, Value>`
3. Creates an `ArmamentCall` and invokes the Arsenal
4. Formats the tool result via `ToolResultFormatter`
5. Appends the result to `accumulated_output` for the next LLM iteration

This entire flow has **zero test coverage**. Each individual component is tested in isolation (MockLlmAdapter tool calls, ArsenalExecutionService invocations, MCP transport connect/discover/invoke), but they have never been wired together in a test.

## Goals

1. **Close the end-to-end tool call test gap**: Verify the complete LLM → Arsenal → result-back-to-LLM loop functions correctly in `PaladinExecutionService`
2. **Test CLI-level integration**: Verify that YAML-configured arsenal works when agents are constructed and executed through the CLI pipeline
3. **Validate error paths**: Ensure tool call failures (unknown tool, invalid arguments, execution errors) are handled gracefully and don't crash the agent loop
4. **CI-friendly by default**: Core tests use in-process mocks requiring no external dependencies, API keys, or network access
5. **Optional deep integration**: Gated tests using the real Python MCP test server verify actual MCP protocol communication

## User Stories

### US-1: PaladinExecutionService Tool Call Verification

**As a** developer  
**I want** unit-level tests that wire `MockLlmAdapter` (returning tool call responses) with a mock `ArsenalPort` implementation through `PaladinExecutionService`  
**So that** the tool call detection, invocation, result formatting, and context injection are verified end-to-end

**Acceptance Criteria:**
- Test that when MockLlmAdapter returns `MockResponse::ToolCall`, the mock Arsenal's `invoke()` is called with correct `ArmamentCall`
- Test that the tool result is formatted and appended to the accumulated output
- Test that the next LLM iteration receives the tool result in its context
- Test multi-turn: LLM requests tool → tool returns result → LLM produces final answer
- Test that tool call with `arsenal: None` produces a warning (no crash)
- All tests use in-process mocks only (no network, no Python process)

### US-2: CLI-Level Arsenal Integration

**As a** developer  
**I want** integration tests that construct a Paladin with arsenal configuration (similar to YAML-based setup) and execute through the full CLI pipeline  
**So that** the config → arsenal setup → execution → tool call path is validated

**Acceptance Criteria:**
- Test constructs `PaladinExecutionService` with a mock Arsenal (in-process `MockArsenalPort`)
- MockLlmAdapter configured with: [ToolCall response, then Text response (final answer)]
- Execution produces output containing both the tool result and the final LLM answer
- Test verifies MockLlmAdapter was called exactly twice (tool call + final)
- Test verifies mock Arsenal was invoked once with expected tool name and arguments

### US-3: Tool Call Error Handling

**As a** developer  
**I want** tests that verify error handling in the tool call path  
**So that** I know the system degrades gracefully when tools fail

**Acceptance Criteria:**
- Test: Arsenal returns `ArsenalError::ToolNotFound` → error message injected into context, execution continues
- Test: Arsenal returns `ArsenalError::ExecutionError` → error message injected, LLM sees formatted error
- Test: Tool call arguments are invalid JSON → `ArsenalError::InvalidArguments` is returned
- Test: Multiple tool calls in sequence, one fails → execution continues with remaining calls
- All error scenarios result in graceful degradation (no panic, no crash)

### US-4: Python MCP Server Integration (Optional/Gated)

**As a** developer  
**I want** gated integration tests that use the real Python MCP test server  
**So that** the full STDIO MCP transport → tool discovery → tool invocation path is validated with a real MCP server

**Acceptance Criteria:**
- Test uses `tests/mcp_test_server.py` as STDIO MCP server
- Test creates a real `MCPStdioAdapter` → connects → discovers tools → registers in `ArsenalRegistryService`
- MockLlmAdapter returns a tool call matching the Python server's `echo` or `calculator` tool
- `PaladinExecutionService` executes with the real arsenal and mock LLM
- Tool result from Python server appears in execution output
- Tests are gated behind `#[ignore]` or a feature flag and only run when explicitly requested
- Test runner checks for Python availability before executing

## Functional Requirements

### FR-1: MockArsenalPort Implementation

**FR-1.1** - Create a `MockArsenalPort` test helper that implements the `ArsenalPort` trait (`list_armaments`, `invoke`, `validate_call`)

**FR-1.2** - `MockArsenalPort` MUST support:
- Configurable list of available armaments (tools)
- Pre-configured responses for `invoke()` keyed by tool name
- Invocation recording for assertions (call count, arguments received)
- Configurable error responses per tool name

**FR-1.3** - `MockArsenalPort` MUST be `Send + Sync` and usable with `Arc<dyn ArsenalPort>`

**FR-1.4** - Place `MockArsenalPort` in `tests/helpers/mock_arsenal_adapter.rs` alongside existing `MockLlmAdapter`

### FR-2: PaladinExecutionService Tool Call Tests

**FR-2.1** - Create test file `tests/cli/tool_integration_test.rs` for the core tool call flow tests

**FR-2.2** - Test: `test_tool_call_basic_flow`
- MockLlmAdapter: [ToolCall("calculator", `{"operation":"add","a":1,"b":2}`), Text("The result is 3")]
- MockArsenalPort: returns `ArmamentResult { success: true, output: "3", ... }` for "calculator"
- Assert: execution succeeds, output contains tool result, LLM called twice

**FR-2.3** - Test: `test_tool_call_result_fed_back_to_llm`
- Verify that the second LLM call's prompt/context includes the formatted tool result from the first call

**FR-2.4** - Test: `test_tool_call_no_arsenal_available`
- MockLlmAdapter returns ToolCall, but `PaladinExecutionService` has `arsenal: None`
- Assert: execution completes without error (warning logged, no crash)

**FR-2.5** - Test: `test_tool_call_unknown_tool`
- MockArsenalPort returns `ArsenalError::ToolNotFound`
- Assert: error message injected into context, execution continues to next iteration

**FR-2.6** - Test: `test_tool_call_invalid_arguments`
- MockLlmAdapter returns ToolCall with malformed JSON arguments
- Assert: `ArsenalError::InvalidArguments` handled gracefully

**FR-2.7** - Test: `test_tool_call_execution_error`
- MockArsenalPort returns `ArsenalError::ExecutionError`
- Assert: formatted error message in output, execution continues

**FR-2.8** - Test: `test_multiple_sequential_tool_calls`
- MockLlmAdapter: [ToolCall("tool_a"), ToolCall("tool_b"), Text("final")]
- MockArsenalPort: different results for each tool
- Assert: both tools invoked, both results in context, LLM called three times

**FR-2.9** - Test: `test_tool_call_with_garrison`
- Configure `PaladinExecutionService` with both arsenal and an in-memory garrison
- Assert: tool result is stored in garrison as `ConversationRole::Tool` entry

### FR-3: Gated Python MCP Server Tests

**FR-3.1** - Create test file `tests/integration/tool_integration_mcp_test.rs` for gated MCP tests

**FR-3.2** - Test: `test_full_mcp_stdio_tool_call_flow` (gated with `#[ignore]`)
- Start Python MCP test server via `MCPStdioAdapter`
- Discover tools (echo, calculator)
- Register in `ArsenalRegistryService` → wrap in `ArsenalExecutionService`
- Wire with MockLlmAdapter (returns ToolCall for "echo" then Text final)
- Execute through `PaladinExecutionService`
- Assert: echo tool result appears in output

**FR-3.3** - Test: `test_mcp_calculator_tool_invocation` (gated with `#[ignore]`)
- Same setup as FR-3.2 but invokes calculator tool with `{"operation":"add","a":5,"b":3}`
- Assert: result contains "8"

**FR-3.4** - Gated tests MUST check for Python availability at test start and skip gracefully if unavailable

## Non-Goals (Out of Scope)

1. **SSE MCP server integration tests**: The SSE transport is already tested via mockito in `mcp_sse_test.rs`. Adding an SSE server for gated tests is not required.
2. **Real LLM provider testing**: All tests use MockLlmAdapter. Testing with actual OpenAI/Anthropic/DeepSeek is outside scope.
3. **New Arsenal features**: No new tool types, tool composition, or arsenal orchestration patterns.
4. **Performance benchmarks**: No benchmark tests for tool call latency or throughput.
5. **Handoff tool call testing**: The `handoff_to_specialist` special tool call has separate tests in the handoff module.
6. **Streaming with tool calls**: Testing tool calls within streaming execution is out of scope.

## Design Considerations

### Test Architecture

```
tests/
├── helpers/
│   ├── mock_llm_adapter.rs         # EXISTING - already supports ToolCall
│   ├── mock_arsenal_adapter.rs     # NEW - MockArsenalPort
│   └── mod.rs                      # UPDATE - export new mock
├── cli/
│   ├── tool_integration_test.rs    # NEW - core tool call tests (CI-friendly)
│   └── mod.rs                      # UPDATE - include new module
└── integration/
    ├── tool_integration_mcp_test.rs # NEW - gated MCP tests
    └── mod.rs                       # UPDATE - include new module
```

### MockArsenalPort Design

```rust
pub struct MockArsenalPort {
    armaments: Vec<Armament>,
    responses: Arc<Mutex<HashMap<String, Result<ArmamentResult, ArsenalError>>>>,
    invocations: Arc<Mutex<Vec<ArmamentCall>>>,
}

impl MockArsenalPort {
    pub fn new() -> Self { /* ... */ }
    pub fn add_tool(&mut self, name: &str, description: &str) -> &mut Self { /* ... */ }
    pub fn set_response(&self, tool_name: &str, result: ArmamentResult) { /* ... */ }
    pub fn set_error(&self, tool_name: &str, error: ArsenalError) { /* ... */ }
    pub fn call_count(&self) -> usize { /* ... */ }
    pub fn invocations(&self) -> Vec<ArmamentCall> { /* ... */ }
}
```

### Test Pattern

Each test follows the established CLI test pattern from `paladin_execution_test.rs`:

```rust
#[tokio::test]
async fn test_tool_call_basic_flow() {
    // Arrange: Set up MockLlmAdapter with tool call + final response
    let mock_llm = Arc::new(MockLlmAdapter::new());
    mock_llm.add_tool_call("calculator", r#"{"operation":"add","a":1,"b":2}"#);
    mock_llm.add_success("The result is 3");

    // Arrange: Set up MockArsenalPort with expected tool response
    let mock_arsenal = Arc::new(MockArsenalPort::new());
    mock_arsenal.set_response("calculator", ArmamentResult::success("3"));

    // Arrange: Create Paladin and execution service
    let paladin = create_test_paladin(MaxLoops::Fixed(3));
    let circuit_breaker = Arc::new(CircuitBreaker::new(3, 2, Duration::from_secs(30)));
    let service = PaladinExecutionService::new(
        mock_llm.clone() as Arc<dyn LlmPort>,
        circuit_breaker,
        None,
        Some(mock_arsenal.clone() as Arc<dyn ArsenalPort>),
    );

    // Act
    let result = service.execute(&paladin, "What is 1 + 2?").await;

    // Assert
    assert!(result.is_ok());
    let output = result.unwrap();
    assert!(output.output.contains("3"));
    assert_eq!(mock_llm.call_count(), 2); // tool call + final
    assert_eq!(mock_arsenal.call_count(), 1);
}
```

## Technical Considerations

### Dependencies

- **No new crate dependencies required**. All mocking is done with in-process Rust structs.
- The Python MCP test server (`tests/mcp_test_server.py`) already exists and supports `echo` and `calculator` tools.

### Existing Infrastructure to Leverage

| Component | Location | Status |
|-----------|----------|--------|
| `MockLlmAdapter` | `tests/helpers/mock_llm_adapter.rs` | Exists, supports `MockResponse::ToolCall` |
| `ArsenalPort` trait | `src/application/ports/output/arsenal_port.rs` | Exists |
| `ArsenalExecutionService` | `src/application/use_cases/arsenal/arsenal_execution_service.rs` | Exists |
| `ArsenalRegistryService` | `src/application/use_cases/arsenal/arsenal_registry_service.rs` | Exists |
| `PaladinExecutionService` | `src/application/use_cases/paladin/paladin_execution_service.rs` | Exists, tool call loop at ~line 758 |
| `ToolResultFormatter` | Part of `PaladinExecutionService` | Exists |
| `ArmamentCall`, `ArmamentResult` | `src/core/platform/container/arsenal.rs` | Exists |
| Python MCP test server | `tests/mcp_test_server.py` | Exists (echo + calculator tools) |
| `MCPStdioAdapter` | `src/infrastructure/adapters/arsenal/mcp_stdio_adapter.rs` | Exists |

### Key Code Paths Under Test

1. **`PaladinExecutionService::execute()`** — Lines ~745-840 of `paladin_execution_service.rs`: The main loop that checks `response.function_call`, routes to `handle_tool_call()`, appends result to `accumulated_output`, and stores in garrison.

2. **`PaladinExecutionService::handle_tool_call()`** — Lines ~1747-1785: Parses function call arguments, creates `ArmamentCall`, invokes arsenal, formats result.

3. **Error injection path** — Line ~815-830: When `handle_tool_call` returns `Err`, a formatted error message is appended to `accumulated_output` for the LLM to see.

### CI Compatibility

- **Core tests** (`tests/cli/tool_integration_test.rs`): Use only in-process mocks. No API keys, no network, no Python. Run in standard `cargo test`.
- **Gated tests** (`tests/integration/tool_integration_mcp_test.rs`): Marked with `#[ignore]`. Require Python 3.x with `mcp` package. Run explicitly via `cargo test -- --ignored` or in CI jobs that have Python configured.

### Hexagonal Architecture Compliance

- Tests instantiate `PaladinExecutionService` (application layer) with trait objects (`Arc<dyn LlmPort>`, `Arc<dyn ArsenalPort>`)
- Mock implementations satisfy port contracts without touching infrastructure
- No direct imports from infrastructure layer in the core test file

## Success Metrics

| Metric | Target |
|--------|--------|
| Core tool call tests passing | ≥ 8 tests green in CI |
| Gated MCP tests passing | ≥ 2 tests green when run with `--ignored` and Python available |
| Tool call loop code coverage | Lines 758-840 and 1747-1785 of `paladin_execution_service.rs` covered |
| No external dependencies for core tests | `cargo test tool_integration` passes with no env vars or network |
| Task 4.6 subtasks completed | All 5 subtasks (4.6.1-4.6.5) marked `[x]` |
| Epic 23 acceptance criteria met | US-23.3 "Tool integration: Test arsenal tool invocation through mock provider" satisfied |

## Open Questions

1. **Garrison integration in tool tests**: Should the garrison-with-tool-call test use the existing `InMemoryGarrison` adapter or a mock garrison? (Suggested: use `InMemoryGarrison` since it's already in-process and lightweight)

2. **MockArsenalPort reuse**: Should `MockArsenalPort` be designed for reuse beyond Epic 23 (e.g., by Battalion tests that may need arsenal in the future)? (Suggested: yes, design it generically in `tests/helpers/`)

3. **ToolResultFormatter verification**: Should tests assert on the exact format of tool results (coupling to formatter implementation), or only that tool results are present in output? (Suggested: assert presence + key content, not exact format)
