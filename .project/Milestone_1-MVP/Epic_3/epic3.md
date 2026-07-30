## Epic 3: Arsenal Tool System

### Overview

**Priority:** High  
**Effort:** 3-4 weeks  
**Dependencies:** Epic 1  
**Team:** 2 developers

**Objective:** Implement the Arsenal tool system with MCP (Model Context Protocol) support, enabling Paladins to interact with external services and capabilities.

### User Stories

1. **As a developer**, I want to register tools with Paladins so that they can perform actions.
2. **As a developer**, I want to connect MCP servers so that Paladins can use external tools.
3. **As a developer**, I want Paladins to automatically invoke tools so that complex tasks are completed.
4. **As a developer**, I want tool results injected into context so that Paladins can reason about outcomes.

### Technical Design

#### Domain Layer

**arsenal.rs - Tool Domain**

```rust
/// Definition of a tool in the Arsenal
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Armament {
    pub name: String,
    pub description: String,
    pub parameters: JsonSchema,
    pub required_params: Vec<String>,
}

/// A request to invoke a tool
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArmamentCall {
    pub tool_name: String,
    pub arguments: HashMap<String, Value>,
    pub call_id: Uuid,
}

/// Result of tool invocation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArmamentResult {
    pub call_id: Uuid,
    pub success: bool,
    pub output: Option<Value>,
    pub error: Option<String>,
    pub execution_time_ms: u64,
}
```

#### Application Layer

**ports/output/arsenal_port.rs**

```rust
/// Port for tool operations
#[async_trait]
pub trait ArsenalPort: Send + Sync {
    /// List available tools
    async fn list_armaments(&self) -> Vec<Armament>;

    /// Invoke a tool
    async fn invoke(&self, call: ArmamentCall) -> Result<ArmamentResult, ArsenalError>;

    /// Validate tool call arguments
    fn validate_call(&self, call: &ArmamentCall) -> Result<(), ArsenalError>;
}

/// Registry for managing tool collections
#[async_trait]
pub trait ArsenalRegistry: Send + Sync {
    /// Register a new tool
    async fn register(&self, armament: Armament, handler: Box<dyn ArmamentHandler>);

    /// Unregister a tool
    async fn unregister(&self, name: &str) -> Option<Armament>;

    /// Get tool by name
    async fn get(&self, name: &str) -> Option<&Armament>;
}
```

**MCP Protocol Implementation**

**adapters/arsenal/mcp_client.rs**

```rust
/// MCP protocol client
pub struct MCPClient {
    transport: Box<dyn MCPTransport>,
    capabilities: MCPCapabilities,
}

#[async_trait]
pub trait MCPTransport: Send + Sync {
    async fn send(&self, message: MCPMessage) -> Result<MCPResponse, MCPError>;
    async fn receive(&self) -> Result<MCPMessage, MCPError>;
}
```

**adapters/arsenal/mcp_stdio_adapter.rs**

```rust
/// MCP adapter for STDIO-based tool servers
pub struct MCPStdioAdapter {
    command: String,
    args: Vec<String>,
    process: Option<Child>,
}
```

**adapters/arsenal/mcp_sse_adapter.rs**

```rust
/// MCP adapter for SSE-based tool servers
pub struct MCPSseAdapter {
    endpoint: String,
    client: reqwest::Client,
}
```

### Builder Integration

```rust
impl PaladinBuilder {
    /// Add a tool to the Paladin's arsenal
    pub fn add_armament(self, armament: Armament, handler: Box<dyn ArmamentHandler>) -> Self;

    /// Add STDIO MCP server
    pub async fn add_mcp_stdio(self, command: &str, args: &[&str]) -> Self;

    /// Add SSE MCP server
    pub async fn add_mcp_sse(self, name: &str, endpoint: &str) -> Self;
}
```

### Test Requirements

#### Unit Tests

- `test_armament_schema_validation`
- `test_armament_call_serialization`
- `test_arsenal_registry_operations`
- `test_tool_result_handling`

#### Integration Tests

- `test_mcp_stdio_server_connection`
- `test_mcp_sse_server_connection`
- `test_paladin_tool_invocation`
- `test_tool_result_context_injection`

### Acceptance Criteria

- [ ] Paladins can invoke registered tools during execution
- [ ] MCP STDIO servers can be connected and discovered
- [ ] MCP SSE servers can be connected and discovered
- [ ] Tool results are properly formatted and injected into context
- [ ] Tool call failures are handled gracefully
- [ ] Unit test coverage ≥ 80%

### Definition of Done

- [ ] All tests passing
- [ ] Code reviewed and approved
- [ ] MCP protocol compliance verified
- [ ] Documentation includes tool authoring guide

---
