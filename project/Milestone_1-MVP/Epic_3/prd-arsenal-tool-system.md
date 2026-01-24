# Product Requirements Document: Arsenal Tool System

## Introduction/Overview

The Arsenal Tool System enables Paladins (AI agents) to interact with external services and capabilities through a unified tool interface. This feature implements the Model Context Protocol (MCP) to allow Paladins to discover, validate, and invoke external tools, significantly expanding their ability to perform complex tasks beyond pure language model capabilities.

**Problem Statement:** AI agents are limited to the knowledge and capabilities encoded in their language models. To perform real-world tasks like web searches, file operations, API calls, or database queries, agents need a standardized way to interact with external tools.

**Solution:** The Arsenal Tool System provides a hexagonal architecture-based tool registry that supports MCP-compliant tool servers via both STDIO and SSE transports, enabling Paladins to dynamically discover and invoke external capabilities with proper error handling and resource controls.

## Goals

1. **Enable External Tool Integration:** Allow Paladins to invoke external tools through a standardized MCP protocol interface.
2. **Support Multiple Transport Mechanisms:** Implement both STDIO (command-line) and SSE (web service) MCP transports for maximum flexibility.
3. **Ensure Resource Safety:** Provide timeout controls and concurrency limits to prevent resource exhaustion.
4. **Maintain Resilience:** Allow Paladins to continue execution when tools fail, with failures noted in context.
5. **Follow Hexagonal Architecture:** Maintain strict separation between domain logic, application ports, and infrastructure adapters.

## User Stories

### Developer Persona

**US-1:** As a developer, I want to register MCP-compliant tool servers with my Paladin so that it can discover and use external capabilities.

**US-2:** As a developer, I want to connect STDIO-based MCP servers (command-line tools) so that my Paladin can leverage local utilities and scripts.

**US-3:** As a developer, I want to connect SSE-based MCP servers (web services) so that my Paladin can access cloud-based tools and APIs.

**US-4:** As a developer, I want tool invocation results automatically injected into the Paladin's context so that the agent can reason about outcomes and make informed decisions.

**US-5:** As a developer, I want to configure timeout and concurrency limits for tool execution so that I can prevent resource exhaustion and ensure system stability.

**US-6:** As a developer, I want tool failures to be logged but not halt Paladin execution so that my agent can gracefully handle unavailable or failing tools.

## Functional Requirements

### FR-1: Domain Layer - Tool Definitions

1.1. The system MUST define an `Armament` struct representing a single tool with:
   - Name (unique identifier)
   - Description (human-readable purpose)
   - JSON Schema for parameters
   - List of required parameter names

1.2. The system MUST define an `ArmamentCall` struct representing a tool invocation request with:
   - Tool name
   - Arguments (HashMap of parameter name to JSON value)
   - Unique call ID (UUID)

1.3. The system MUST define an `ArmamentResult` struct representing tool execution outcome with:
   - Call ID (matching the request)
   - Success boolean flag
   - Optional output (JSON value)
   - Optional error message
   - Execution time in milliseconds

### FR-2: Application Layer - Arsenal Port

2.1. The system MUST provide an `ArsenalPort` trait with these async methods:
   - `list_armaments()` - Returns all available tools
   - `invoke(call: ArmamentCall)` - Executes a tool and returns result
   - `validate_call(call: &ArmamentCall)` - Validates call arguments against schema

2.2. The system MUST provide an `ArsenalRegistry` trait for managing tool collections with:
   - `register()` - Add a new tool with its handler
   - `unregister()` - Remove a tool by name
   - `get()` - Retrieve tool metadata by name

2.3. All port traits MUST be `Send + Sync` for async compatibility.

### FR-3: MCP Protocol Implementation

3.1. The system MUST implement a `MCPClient` that:
   - Manages communication with MCP servers
   - Handles protocol-level message serialization/deserialization
   - Reports server capabilities

3.2. The system MUST define a `MCPTransport` trait with:
   - `send(message: MCPMessage)` - Send a message to the MCP server
   - `receive()` - Receive a message from the MCP server

3.3. The MCP implementation MUST comply with the Model Context Protocol specification for tool discovery and invocation.

### FR-4: STDIO Transport Adapter

4.1. The system MUST implement `MCPStdioAdapter` that:
   - Spawns external processes with configurable command and arguments
   - Communicates via stdin/stdout
   - Manages process lifecycle (start, stop, cleanup)

4.2. The adapter MUST handle process initialization and connect to the MCP server on startup.

4.3. The adapter MUST properly clean up child processes on shutdown or failure.

### FR-5: SSE Transport Adapter

5.1. The system MUST implement `MCPSseAdapter` that:
   - Connects to HTTP/HTTPS endpoints
   - Uses Server-Sent Events (SSE) for receiving messages
   - Uses HTTP POST for sending messages

5.2. The adapter MUST handle connection retry logic with exponential backoff.

5.3. The adapter MUST include proper connection timeout handling.

### FR-6: PaladinBuilder Integration

6.1. The `PaladinBuilder` MUST provide `add_mcp_stdio(command, args)` method to configure STDIO MCP servers.

6.2. The `PaladinBuilder` MUST provide `add_mcp_sse(name, endpoint)` method to configure SSE MCP servers.

6.3. The builder MUST validate MCP server connections during the `build()` phase.

### FR-7: Resource Controls

7.1. The system MUST support configurable timeout for each tool invocation (default: 30 seconds).

7.2. The system MUST support configurable maximum concurrent tool executions (default: 5).

7.3. Tool invocations exceeding the timeout MUST return a timeout error in the `ArmamentResult`.

7.4. Tool invocations exceeding concurrency limits MUST queue and wait for available slots.

### FR-8: Error Handling and Resilience

8.1. When a tool invocation fails, the system MUST:
   - Create an `ArmamentResult` with `success: false`
   - Include the error message
   - Record execution time
   - Return the result to the Paladin

8.2. Paladin execution MUST continue after tool failures, with the failure result injected into context.

8.3. The system MUST log all tool failures with appropriate severity levels.

8.4. Connection failures to MCP servers during builder initialization MUST return a clear error.

### FR-9: Context Injection

9.1. Tool invocation results MUST be formatted as structured text and injected into the Paladin's conversation context.

9.2. The injected context MUST include:
   - Tool name
   - Call arguments
   - Execution outcome (success/failure)
   - Output data or error message
   - Execution time

9.3. The format MUST be readable by the LLM for reasoning about tool outcomes.

## Non-Goals (Out of Scope)

1. **Built-in Tools:** This implementation will NOT include any built-in tools. Developers must provide all tools via MCP servers.

2. **Tool Authoring Framework:** This PRD does NOT cover creating tools or MCP servers—only connecting to existing ones.

3. **Tool Marketplace:** No centralized registry or discovery service for finding third-party tools.

4. **Advanced Sandboxing:** Full containerization or VM-level isolation is out of scope. Basic timeout and resource limits are sufficient.

5. **Tool Caching:** Caching of tool results is not included in this version.

6. **Tool Authentication:** Authentication/authorization for tool access is delegated to the MCP servers themselves.

7. **Tool Versioning:** No version management for tools or MCP protocol versions beyond basic compatibility checks.

## Design Considerations

### Hexagonal Architecture Boundaries

- **Domain Layer** (`core/platform/container/arsenal.rs`): Pure domain types with zero external dependencies
- **Application Layer** (`application/ports/output/arsenal_port.rs`): Port traits defining tool operation interfaces
- **Infrastructure Layer** (`infrastructure/adapters/arsenal/`): MCP protocol implementations and transport adapters

### Error Handling Pattern

Use `thiserror` crate for domain-specific error types:

```rust
#[derive(Debug, thiserror::Error)]
pub enum ArsenalError {
    #[error("Tool not found: {0}")]
    ToolNotFound(String),
    #[error("Invalid tool arguments: {0}")]
    InvalidArguments(String),
    #[error("Tool execution timeout after {0} seconds")]
    Timeout(u64),
    #[error("MCP protocol error: {0}")]
    ProtocolError(String),
    #[error("Transport error: {0}")]
    TransportError(String),
}
```

### Configuration

Tool system configuration should be added to `config.yml`:

```yaml
arsenal:
  default_timeout_seconds: 30
  max_concurrent_tools: 5
  mcp_servers:
    - name: "web_search"
      type: "stdio"
      command: "uvx"
      args: ["mcp-web-search"]
    - name: "github_api"
      type: "sse"
      endpoint: "https://mcp.example.com/github"
```

### MCP Protocol Compliance

The implementation must follow the Model Context Protocol specification:
- JSON-RPC 2.0 message format
- Standard tool discovery methods
- Standard tool invocation methods
- Proper error codes and responses

## Technical Considerations

### Dependencies

- **New Dependencies:**
  - `jsonschema` crate for JSON Schema validation
  - `tokio::process` for STDIO process management
  - `reqwest` with SSE support for HTTP/SSE transport
  - Existing `tokio`, `serde`, `serde_json`, `async-trait`

### Concurrency

- Use `tokio::sync::Semaphore` for concurrency limiting
- Use `tokio::time::timeout` for execution timeout controls
- Ensure all Arsenal operations are thread-safe (Send + Sync)

### Testing Strategy

- **Unit Tests:** Test domain types, validation logic, error handling
- **Integration Tests:** Test MCP protocol compliance, STDIO/SSE transports with mock servers
- **Functional Tests:** Test Paladin execution with tool invocation end-to-end

### Feature Flags

Consider adding a feature flag if MCP adds significant dependencies:
```toml
[features]
default = ["arsenal"]
arsenal = ["jsonschema", "reqwest"]
```

## Success Metrics

### Functional Success

1. **Tool Registration:** 100% of registered MCP servers are discoverable via `list_armaments()`
2. **Tool Invocation:** 95%+ of valid tool calls execute successfully within timeout
3. **Error Resilience:** 100% of tool failures result in graceful continuation with error context
4. **Resource Safety:** Zero instances of resource exhaustion from runaway tools

### Code Quality Metrics

1. **Test Coverage:** ≥ 80% unit test coverage for Arsenal module
2. **Integration Tests:** All MCP transport types have passing integration tests
3. **Documentation:** All public APIs have rustdoc comments with examples
4. **Linting:** Zero clippy warnings in Arsenal module

### Performance Metrics

1. **Tool Latency:** Median tool invocation overhead < 50ms (excluding actual tool execution)
2. **Concurrency:** System handles 5 concurrent tool executions without degradation
3. **Timeout Accuracy:** Timeouts trigger within 5% of configured duration

## Open Questions

1. **MCP Protocol Version:** Which version of the MCP specification should we target? (Current assumption: latest stable)

2. **Tool Result Size Limits:** Should we impose maximum size limits on tool output to prevent context overflow?

3. **Streaming Tool Results:** Should tools support streaming responses for long-running operations, or only batch results?

4. **Tool Cost Tracking:** Should we track and report costs for tool invocations (e.g., API calls with billing)?

5. **Tool Permissions Model:** Should Paladins be able to grant different permission levels to different tools?

6. **MCP Server Health Checks:** Should we implement periodic health checks for MCP servers to detect failures proactively?

## Acceptance Criteria

- [ ] All functional requirements (FR-1 through FR-9) are implemented
- [ ] Both STDIO and SSE MCP transports are functional
- [ ] Paladins can discover tools from connected MCP servers
- [ ] Paladins can invoke tools with proper argument validation
- [ ] Tool results are injected into Paladin context with clear formatting
- [ ] Tool failures are handled gracefully without halting execution
- [ ] Timeout and concurrency controls are enforced correctly
- [ ] Unit test coverage ≥ 80%
- [ ] Integration tests pass for both transport types
- [ ] All code passes `cargo clippy` with no warnings
- [ ] All public APIs have rustdoc documentation
- [ ] Configuration loading works for MCP server definitions

## Implementation Phases

### Phase 1: Domain & Ports (Week 1)
- Implement domain types (Armament, ArmamentCall, ArmamentResult)
- Define ArsenalPort and ArsenalRegistry traits
- Define ArsenalError types
- Write unit tests for domain logic

### Phase 2: MCP Protocol Core (Week 1-2)
- Implement MCPClient and MCPTransport trait
- Implement MCP message serialization/deserialization
- Write unit tests for MCP protocol layer

### Phase 3: STDIO Transport (Week 2)
- Implement MCPStdioAdapter
- Add process lifecycle management
- Write integration tests with mock STDIO server

### Phase 4: SSE Transport (Week 2-3)
- Implement MCPSseAdapter
- Add connection retry and timeout logic
- Write integration tests with mock SSE server

### Phase 5: Builder Integration (Week 3)
- Add MCP methods to PaladinBuilder
- Implement tool discovery during build
- Add configuration file support

### Phase 6: Resource Controls (Week 3-4)
- Implement timeout controls with tokio::time
- Implement concurrency limiting with Semaphore
- Add metrics and logging

### Phase 7: Context Injection (Week 4)
- Format tool results for LLM consumption
- Integrate with Paladin execution loop
- Handle tool result injection into conversation

### Phase 8: Testing & Documentation (Week 4)
- Comprehensive integration testing
- End-to-end functional tests
- Documentation and examples
- Performance benchmarking

## Dependencies

- **Requires:** Epic 1 (Paladin Core) must be complete for PaladinBuilder integration
- **Blocks:** Epic 4 (Battalion Orchestration) - Battalions will use Arsenal for distributed tool execution

---

**Document Version:** 1.0  
**Last Updated:** January 23, 2026  
**Author:** AI Assistant  
**Status:** Ready for Implementation
