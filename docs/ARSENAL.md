# Arsenal Tool System

## Overview

The **Arsenal Tool System** enables Paladins (AI agents) to interact with external tools and services through the **Model Context Protocol (MCP)**. This hexagonal architecture implementation provides a clean separation between tool definitions, execution logic, and transport mechanisms.

### Key Concepts

- **Armament**: A single tool or capability (e.g., calculator, file reader, web search)
- **Arsenal**: The collection of available tools and the infrastructure to execute them
- **MCP (Model Context Protocol)**: JSON-RPC 2.0 based protocol for tool communication
- **Transport**: The mechanism for tool invocation (STDIO or SSE)

### Architecture Layers

```
┌─────────────────────────────────────────────────────────┐
│                    Paladin (Agent)                       │
│  - Receives tool calls from LLM                         │
│  - Invokes arsenal                                      │
│  - Injects results back into conversation               │
└─────────────────┬───────────────────────────────────────┘
                  │
┌─────────────────▼───────────────────────────────────────┐
│           Application Layer (Ports)                      │
│  - ArsenalPort: Tool execution interface                │
│  - ArsenalRegistry: Tool registration interface         │
│  - ArsenalExecutionService: Orchestration logic         │
└─────────────────┬───────────────────────────────────────┘
                  │
┌─────────────────▼───────────────────────────────────────┐
│        Infrastructure Layer (Adapters)                   │
│  - MCPStdioAdapter: Command-line tool execution         │
│  - MCPSseAdapter: HTTP/SSE tool execution               │
│  - TimeoutWrapper: Execution time limits                │
│  - ConcurrencyLimiter: Parallel execution control       │
└──────────────────────────────────────────────────────────┘
```

## Quick Start

### Basic Usage

```rust
use paladin::application::use_cases::paladin::PaladinBuilder;
use paladin::application::ports::output::llm_port::LlmPort;
use paladin::infrastructure::adapters::llm::MockLlmAdapter;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create LLM adapter
    let llm_port: Arc<dyn LlmPort> = Arc::new(
        MockLlmAdapter::new()
            .with_responses(vec![
                "I'll help you calculate that.".to_string(),
            ])
    );

    // Build Paladin with tool support
    let paladin = PaladinBuilder::new(llm_port)
        .system_prompt("You are a helpful assistant with calculator capabilities.")
        .name("Calculator Agent")
        .build()?;

    // Execute with tool support
    let result = paladin.execute("What is 12 * 8?").await?;
    println!("Result: {}", result);

    Ok(())
}
```

### With STDIO MCP Server

```rust
use paladin::application::use_cases::arsenal::ArsenalRegistryService;
use paladin::application::ports::output::arsenal_port::ArsenalRegistry;
use paladin::infrastructure::adapters::arsenal::Armament;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create arsenal registry
    let registry = Arc::new(ArsenalRegistryService::new());

    // Register STDIO tool (conceptual - requires actual MCP server)
    let calculator = Armament {
        name: "calculator".to_string(),
        description: "Performs basic arithmetic operations".to_string(),
        parameters: serde_json::json!({
            "type": "object",
            "properties": {
                "operation": {"type": "string", "enum": ["add", "subtract", "multiply", "divide"]},
                "a": {"type": "number"},
                "b": {"type": "number"}
            },
            "required": ["operation", "a", "b"]
        }),
        required_params: vec!["operation".to_string(), "a".to_string(), "b".to_string()],
    };

    registry.register(calculator).await;

    Ok(())
}
```

### With SSE MCP Server

```rust
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let registry = Arc::new(ArsenalRegistryService::new());

    // Register SSE-based remote tool
    let web_search = Armament {
        name: "web_search".to_string(),
        description: "Search the web for information".to_string(),
        parameters: serde_json::json!({
            "type": "object",
            "properties": {
                "query": {"type": "string", "description": "Search query"},
                "max_results": {"type": "integer", "default": 10}
            },
            "required": ["query"]
        }),
        required_params: vec!["query".to_string()],
    };

    registry.register(web_search).await;

    Ok(())
}
```

## Model Context Protocol (MCP)

### Protocol Overview

The Arsenal Tool System implements the [Model Context Protocol](https://spec.modelcontextprotocol.io/) specification, a standardized way for AI agents to interact with external tools and data sources.

**Key Features:**
- JSON-RPC 2.0 message format
- Structured tool discovery via `tools/list`
- Tool invocation via `tools/call`
- Support for both STDIO and SSE transports
- Server capability negotiation

### Message Format

#### Tool Discovery Request

```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "tools/list",
  "params": {}
}
```

#### Tool Discovery Response

```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "result": {
    "tools": [
      {
        "name": "calculator",
        "description": "Performs basic arithmetic operations",
        "inputSchema": {
          "type": "object",
          "properties": {
            "operation": {"type": "string", "enum": ["add", "subtract", "multiply", "divide"]},
            "a": {"type": "number"},
            "b": {"type": "number"}
          },
          "required": ["operation", "a", "b"]
        }
      }
    ]
  }
}
```

#### Tool Invocation Request

```json
{
  "jsonrpc": "2.0",
  "id": 2,
  "method": "tools/call",
  "params": {
    "name": "calculator",
    "arguments": {
      "operation": "multiply",
      "a": 12,
      "b": 8
    }
  }
}
```

#### Tool Invocation Response

```json
{
  "jsonrpc": "2.0",
  "id": 2,
  "result": {
    "content": [
      {
        "type": "text",
        "text": "96"
      }
    ]
  }
}
```

### Transport Mechanisms

#### STDIO Transport

**Use Case**: Local command-line tools, scripts, binaries

**Characteristics:**
- Spawns subprocess using `tokio::process::Command`
- Communicates via stdin/stdout
- Ideal for local development and testing
- Lower latency than network-based transports

**Configuration Example:**

```yaml
arsenal:
  default_timeout_seconds: 30
  max_concurrent_tools: 5
  mcp_servers:
    - name: "calculator"
      type: "stdio"
      command: "python"
      args: ["-m", "calculator_mcp_server"]
```

**Rust Implementation:**

```rust
use paladin::infrastructure::adapters::arsenal::MCPStdioAdapter;

let adapter = MCPStdioAdapter::new(
    "python".to_string(),
    vec!["-m".to_string(), "calculator_mcp_server".to_string()]
);
```

#### SSE (Server-Sent Events) Transport

**Use Case**: Remote web services, cloud-hosted tools, scalable deployments

**Characteristics:**
- HTTP-based communication with SSE streaming
- Supports automatic reconnection
- Works with load balancers and proxies
- Cloud-native architecture

**Configuration Example:**

```yaml
arsenal:
  mcp_servers:
    - name: "web_search"
      type: "sse"
      endpoint: "https://mcp.example.com/search"
```

**Rust Implementation:**

```rust
use paladin::infrastructure::adapters::arsenal::MCPSseAdapter;

let adapter = MCPSseAdapter::new("https://mcp.example.com/search".to_string());
```

## Configuration

### Application Settings

The Arsenal system is configured via `config.yml` (or `config.test.yml` for testing):

```yaml
arsenal:
  # Global timeout for all tool invocations (seconds)
  default_timeout_seconds: 30
  
  # Maximum number of concurrent tool executions
  max_concurrent_tools: 5
  
  # MCP server configurations
  mcp_servers:
    # STDIO-based local tool
    - name: "calculator"
      type: "stdio"
      command: "uvx"
      args: ["mcp-calculator"]
    
    # Another STDIO tool with Python
    - name: "file_reader"
      type: "stdio"
      command: "python"
      args: ["-m", "mcp_file_reader"]
    
    # SSE-based remote tool
    - name: "web_search"
      type: "sse"
      endpoint: "https://api.example.com/mcp/search"
    
    # Another SSE tool
    - name: "weather_api"
      type: "sse"
      endpoint: "https://api.weather.com/mcp"
```

### Environment Variables

Some MCP servers may require authentication:

```bash
# For OpenAI function calling
export OPENAI_API_KEY="sk-..."

# For custom MCP servers
export MCP_AUTH_TOKEN="..."

# For debugging MCP communication
export RUST_LOG="paladin::infrastructure::adapters::arsenal=debug"
```

## Tool Development

### Creating MCP-Compatible Tools

To create a tool that works with the Arsenal system, implement an MCP server that responds to `tools/list` and `tools/call` methods.

#### Python Example (STDIO)

```python
#!/usr/bin/env python3
import json
import sys

def handle_request(request):
    method = request.get("method")
    
    if method == "tools/list":
        return {
            "jsonrpc": "2.0",
            "id": request["id"],
            "result": {
                "tools": [
                    {
                        "name": "calculator",
                        "description": "Basic arithmetic operations",
                        "inputSchema": {
                            "type": "object",
                            "properties": {
                                "operation": {"type": "string"},
                                "a": {"type": "number"},
                                "b": {"type": "number"}
                            },
                            "required": ["operation", "a", "b"]
                        }
                    }
                ]
            }
        }
    
    elif method == "tools/call":
        args = request["params"]["arguments"]
        op = args["operation"]
        a, b = args["a"], args["b"]
        
        if op == "add":
            result = a + b
        elif op == "multiply":
            result = a * b
        # ... other operations
        
        return {
            "jsonrpc": "2.0",
            "id": request["id"],
            "result": {
                "content": [{"type": "text", "text": str(result)}]
            }
        }

if __name__ == "__main__":
    for line in sys.stdin:
        request = json.loads(line)
        response = handle_request(request)
        print(json.dumps(response), flush=True)
```

#### Node.js Example (SSE)

```javascript
const express = require('express');
const app = express();

app.use(express.json());

// Tool list endpoint
app.post('/mcp', (req, res) => {
  const { method, id } = req.body;
  
  if (method === 'tools/list') {
    res.json({
      jsonrpc: '2.0',
      id,
      result: {
        tools: [
          {
            name: 'web_search',
            description: 'Search the web',
            inputSchema: {
              type: 'object',
              properties: {
                query: { type: 'string' }
              },
              required: ['query']
            }
          }
        ]
      }
    });
  } else if (method === 'tools/call') {
    // Perform search and return results
    const { query } = req.body.params.arguments;
    res.json({
      jsonrpc: '2.0',
      id,
      result: {
        content: [{ type: 'text', text: `Results for: ${query}` }]
      }
    });
  }
});

app.listen(3000);
```

### Best Practices

1. **Schema Validation**: Always provide complete JSON Schema for tool parameters
2. **Error Handling**: Return proper JSON-RPC error responses (codes -32xxx)
3. **Timeouts**: Implement internal timeouts shorter than Arsenal's global timeout
4. **Idempotency**: Tools should be idempotent when possible
5. **Documentation**: Provide clear descriptions for tool purpose and parameters

## Resource Controls

### Timeout Management

The Arsenal system enforces execution timeouts to prevent hung tool calls:

```rust
use std::time::Duration;
use paladin::infrastructure::adapters::arsenal::TimeoutWrapper;

let timeout = TimeoutWrapper::new(Duration::from_secs(30));
let result = timeout.execute(async {
    // Tool execution code
}).await?;
```

**Behavior:**
- Default timeout: 30 seconds (configurable via `config.yml`)
- Timeout errors return `ArsenalError::Timeout`
- Execution time is tracked and included in results

### Concurrency Limiting

To prevent resource exhaustion, concurrent tool executions are limited:

```rust
use paladin::infrastructure::adapters::arsenal::ConcurrencyLimiter;

let limiter = ConcurrencyLimiter::new(5); // Max 5 concurrent executions
let permit = limiter.acquire().await?;

// Execute tool with permit held
let result = execute_tool().await?;

drop(permit); // Release permit
```

**Behavior:**
- Default limit: 5 concurrent tools (configurable)
- Requests queue when limit reached
- Fair FIFO ordering for permits

## Error Handling

### Error Types

```rust
pub enum ArsenalError {
    /// Tool not found in registry
    ToolNotFound(String),
    
    /// Invalid arguments provided to tool
    InvalidArguments(String),
    
    /// Tool execution exceeded timeout
    Timeout { tool_name: String, timeout_secs: u64 },
    
    /// MCP protocol error (invalid JSON-RPC)
    ProtocolError(String),
    
    /// Transport-level error (network, process spawn)
    TransportError(String),
}
```

### Error Propagation

Errors are handled gracefully and injected back into the Paladin's context:

```
Tool Call → Arsenal Invocation → Error → Formatted Message → LLM Context
```

Example formatted error message:

```
Tool Execution Failed
Tool: calculator
Arguments: {"operation": "divide", "a": 10, "b": 0}
Error: Division by zero
Execution Time: 5ms

Please try again with valid arguments.
```

## Integration with Paladins

### Automatic Tool Detection

Paladins automatically detect tool calls in LLM responses using function calling format:

```json
{
  "function_call": {
    "name": "calculator",
    "arguments": "{\"operation\": \"multiply\", \"a\": 12, \"b\": 8}"
  }
}
```

### Execution Flow

```
1. LLM generates response with tool call
2. Paladin detects function_call field
3. Arsenal validates tool exists
4. Tool arguments validated against schema
5. Tool executed via appropriate transport
6. Result formatted and injected into context
7. LLM continues with tool results
```

### Context Injection Format

Successful tool executions are formatted as:

```
Tool Execution Result
Tool: calculator
Arguments: {"operation": "multiply", "a": 12, "b": 8}
Output: 96
Execution Time: 12ms
```

## Testing

### Unit Tests

Test domain types and logic:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_armament_creation() {
        let armament = Armament {
            name: "test_tool".to_string(),
            description: "A test tool".to_string(),
            parameters: serde_json::json!({}),
            required_params: vec![],
        };
        
        assert_eq!(armament.name, "test_tool");
    }
}
```

### Integration Tests

Test MCP adapters with mock servers:

```rust
#[tokio::test]
async fn test_stdio_adapter_discovery() {
    let adapter = MCPStdioAdapter::new(
        "python".to_string(),
        vec!["-m".to_string(), "test_mcp_server".to_string()]
    );
    
    let tools = adapter.discover_tools().await?;
    assert!(!tools.is_empty());
}
```

### Functional Tests

End-to-end tests with Paladin integration:

```rust
#[tokio::test]
async fn test_paladin_tool_execution() {
    let paladin = PaladinBuilder::new(mock_llm())
        .system_prompt("Use calculator tool")
        .build()?;
    
    let result = paladin.execute("What is 5 + 3?").await?;
    assert!(result.contains("8"));
}
```

## Troubleshooting

### Common Issues

#### Tool Not Found

**Symptom:** `ArsenalError::ToolNotFound`

**Solutions:**
1. Verify tool is registered in Arsenal registry
2. Check tool name matches exactly (case-sensitive)
3. Ensure MCP server is running and responsive
4. Check logs for discovery errors

#### Timeout Errors

**Symptom:** `ArsenalError::Timeout`

**Solutions:**
1. Increase `default_timeout_seconds` in config
2. Optimize tool implementation for faster execution
3. Check for network latency (SSE transport)
4. Verify tool isn't hanging indefinitely

#### Invalid Arguments

**Symptom:** `ArsenalError::InvalidArguments`

**Solutions:**
1. Check JSON Schema matches tool expectations
2. Ensure LLM is providing all required parameters
3. Validate parameter types (string, number, boolean)
4. Review tool's parameter documentation

#### Protocol Errors

**Symptom:** `ArsenalError::ProtocolError`

**Solutions:**
1. Verify MCP server implements JSON-RPC 2.0 correctly
2. Check for malformed JSON in responses
3. Ensure proper `jsonrpc`, `id`, `method` fields
4. Test MCP server independently with curl/httpie

#### Transport Errors

**Symptom:** `ArsenalError::TransportError`

**Solutions:**
1. STDIO: Check command path and permissions
2. STDIO: Verify all arguments are correct
3. SSE: Test endpoint URL accessibility
4. SSE: Check network connectivity and firewalls
5. Review error logs for specific failure details

### Debugging

Enable debug logging:

```bash
export RUST_LOG="paladin::infrastructure::adapters::arsenal=debug"
cargo run
```

Inspect MCP communication:

```rust
// Add to adapter implementations
tracing::debug!("MCP Request: {:?}", request);
tracing::debug!("MCP Response: {:?}", response);
```

Test MCP server independently:

```bash
# STDIO server
echo '{"jsonrpc":"2.0","id":1,"method":"tools/list","params":{}}' | python -m my_mcp_server

# SSE server
curl -X POST https://mcp.example.com/tools \
  -H "Content-Type: application/json" \
  -d '{"jsonrpc":"2.0","id":1,"method":"tools/list","params":{}}'
```

## Examples

See the `examples/` directory for complete working examples:

- [examples/arsenal_stdio_tools.rs](../examples/arsenal_stdio_tools.rs) - STDIO MCP server usage
- [examples/arsenal_sse_tools.rs](../examples/arsenal_sse_tools.rs) - SSE MCP server usage

Run examples:

```bash
cargo run --example arsenal_stdio_tools
cargo run --example arsenal_sse_tools
```

## API Documentation

Generate and browse complete API documentation:

```bash
cargo doc --no-deps --open
```

Key modules:
- `paladin::core::platform::container::arsenal` - Domain types
- `paladin::application::ports::output::arsenal_port` - Port traits
- `paladin::application::use_cases::arsenal` - Use case services
- `paladin::infrastructure::adapters::arsenal` - MCP adapters

## Contributing

When contributing Arsenal-related changes:

1. Follow TDD: Write tests first
2. Maintain hexagonal architecture boundaries
3. Document all public APIs with rustdoc
4. Run full test suite: `cargo test`
5. Pass clippy: `cargo clippy -- -D warnings`
6. Format code: `cargo fmt`

## License

See [LICENSE](../LICENSE) for details.

## See Also

- [Model Context Protocol Specification](https://spec.modelcontextprotocol.io/)
- [Paladin Design Documentation](Design/Design_and_Architecture.md)
- [Hexagonal Architecture Guide](../notes/hexagonal-arch.md)
- [Garrison Memory System](GARRISON.md)
