# Tool Integration Guide

This guide covers how to integrate external tools and capabilities into your Paladins using the Arsenal system and Model Context Protocol (MCP).

## Table of Contents

- [Overview](#overview)
- [Arsenal Architecture](#arsenal-architecture)
- [MCP Protocol](#mcp-protocol)
- [STDIO Tool Servers](#stdio-tool-servers)
- [SSE Tool Servers](#sse-tool-servers)
- [Custom Tool Development](#custom-tool-development)
- [Tool Result Handling](#tool-result-handling)
- [Best Practices](#best-practices)
- [Troubleshooting](#troubleshooting)

## Overview

The Arsenal system enables Paladins to:
- Execute external tools and capabilities
- Search the web, access databases, run calculations
- Interact with APIs and services
- Extend functionality without modifying core code

**Key Concepts:**
- **Arsenal**: The registry of available tools
- **Armament**: A single tool or capability
- **MCP (Model Context Protocol)**: Standard protocol for tool servers
- **Tool Call**: Request from Paladin to execute a tool
- **Tool Result**: Response from tool execution

## Arsenal Architecture

### Core Components

```rust
// Armament - Tool definition
pub struct Armament {
    pub name: String,
    pub description: String,
    pub schema: ToolSchema,
    pub required_params: Vec<String>,
}

// Arsenal Port - Tool execution interface
#[async_trait]
pub trait ArsenalPort: Send + Sync {
    async fn list_tools(&self) -> Result<Vec<Armament>>;
    async fn invoke(&self, call: &ArmamentCall) -> Result<ArmamentResult>;
}

// Armament Call - Tool invocation request
pub struct ArmamentCall {
    pub tool_name: String,
    pub parameters: HashMap<String, Value>,
    pub call_id: Uuid,
}

// Armament Result - Tool execution response
pub struct ArmamentResult {
    pub call_id: Uuid,
    pub success: bool,
    pub output: String,
    pub error: Option<String>,
}
```

### Tool Flow

```
Paladin → LLM decides to use tool → ArmamentCall
    ↓
ArsenalPort validates call → Routes to correct Armament
    ↓
Tool executes (MCP server, API, local function)
    ↓
ArmamentResult → Injected into Paladin context
    ↓
Paladin continues reasoning with tool result
```

## MCP Protocol

The Model Context Protocol (MCP) is an open standard for connecting LLM applications to external tools and data sources.

### MCP Server Types

1. **STDIO Servers**: Command-line tools communicating via stdin/stdout
2. **SSE Servers**: Web services using Server-Sent Events

### MCP Message Format

```json
// Tool Discovery Request
{
  "jsonrpc": "2.0",
  "method": "tools/list",
  "id": 1
}

// Tool Discovery Response
{
  "jsonrpc": "2.0",
  "result": {
    "tools": [
      {
        "name": "web_search",
        "description": "Search the web for information",
        "inputSchema": {
          "type": "object",
          "properties": {
            "query": {
              "type": "string",
              "description": "Search query"
            }
          },
          "required": ["query"]
        }
      }
    ]
  },
  "id": 1
}

// Tool Invocation Request
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "web_search",
    "arguments": {
      "query": "Rust async programming"
    }
  },
  "id": 2
}

// Tool Invocation Response
{
  "jsonrpc": "2.0",
  "result": {
    "content": [
      {
        "type": "text",
        "text": "Search results: ..."
      }
    ]
  },
  "id": 2
}
```

## STDIO Tool Servers

STDIO servers are command-line programs that communicate via standard input/output.

### Connecting a STDIO Server

```rust
use paladin::arsenal::*;
use paladin::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let llm_adapter = Arc::new(OpenAiAdapter::new().build()?);

    // Connect to an MCP STDIO server
    let web_search = MCPStdioAdapter::new()
        .command("uvx")
        .args(vec!["mcp-server-fetch"])
        .build()
        .await?;

    // Build Paladin with tool access
    let paladin = PaladinBuilder::new(llm_adapter)
        .name("ResearchAssistant")
        .system_prompt("You are a research assistant with web search capabilities. \
                        Use the web_search tool to find current information. \
                        Always cite your sources.")
        .add_armament(Arc::new(web_search))
        .build()?;

    // Paladin will automatically use tools when needed
    let response = paladin.execute("What are the latest Rust features in 2024?").await?;
    println!("{}", response.content);

    Ok(())
}
```

### Popular STDIO MCP Servers

```bash
# Web search
uvx mcp-server-fetch

# File system access
uvx mcp-server-filesystem --allowed-directory ~/Documents

# Git operations
uvx mcp-server-git --repository /path/to/repo

# Database queries
uvx mcp-server-sqlite --db-path database.db

# Calculator
uvx mcp-server-calculator
```

### Configuration Example

```yaml
arsenal:
  mcp_servers:
    - name: "web_search"
      type: "stdio"
      command: "uvx"
      args: ["mcp-server-fetch"]
      enabled: true

    - name: "filesystem"
      type: "stdio"
      command: "uvx"
      args:
        - "mcp-server-filesystem"
        - "--allowed-directory"
        - "/home/user/workspace"
      enabled: true

    - name: "calculator"
      type: "stdio"
      command: "uvx"
      args: ["mcp-server-calculator"]
      enabled: true
```

### Advanced STDIO Configuration

```rust
let web_search = MCPStdioAdapter::new()
    .command("uvx")
    .args(vec!["mcp-server-fetch"])
    .working_directory("/tmp")
    .env("API_KEY", api_key)
    .timeout(Duration::from_secs(30))
    .max_retries(3)
    .build()
    .await?;
```

## SSE Tool Servers

SSE (Server-Sent Events) servers are web services that provide MCP tools over HTTP.

### Connecting an SSE Server

```rust
use paladin::arsenal::*;
use paladin::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let llm_adapter = Arc::new(OpenAiAdapter::new().build()?);

    // Connect to an MCP SSE server
    let api_tools = MCPSseAdapter::new()
        .endpoint("https://api.example.com/mcp")
        .api_key(std::env::var("API_KEY")?)
        .build()
        .await?;

    let paladin = PaladinBuilder::new(llm_adapter)
        .name("APIAssistant")
        .system_prompt("You have access to company APIs. Use them to retrieve data.")
        .add_armament(Arc::new(api_tools))
        .build()?;

    let response = paladin.execute("Get user statistics for last month").await?;
    println!("{}", response.content);

    Ok(())
}
```

### SSE Configuration

```rust
let api_server = MCPSseAdapter::new()
    .endpoint("https://api.example.com/mcp")
    .api_key("your-api-key")
    .bearer_token("bearer-token")  // Alternative auth
    .headers(HashMap::from([
        ("X-Custom-Header", "value"),
    ]))
    .timeout(Duration::from_secs(60))
    .retry_config(RetryConfig {
        max_attempts: 3,
        initial_delay: Duration::from_secs(1),
        max_delay: Duration::from_secs(10),
        exponential_backoff: true,
    })
    .build()
    .await?;
```

### SSE Health Checks

```rust
// Verify server is reachable
if api_server.health_check().await? {
    println!("SSE server is healthy");
}

// List available tools
let tools = api_server.list_tools().await?;
for tool in tools {
    println!("Tool: {} - {}", tool.name, tool.description);
}
```

## Custom Tool Development

Create your own tools by implementing the `ArsenalPort` trait.

### Simple Custom Tool

```rust
use paladin::arsenal::*;
use async_trait::async_trait;

pub struct CalculatorTool;

#[async_trait]
impl ArsenalPort for CalculatorTool {
    async fn list_tools(&self) -> Result<Vec<Armament>, ArsenalError> {
        Ok(vec![
            Armament {
                name: "add".to_string(),
                description: "Add two numbers".to_string(),
                schema: ToolSchema::new()
                    .add_param("a", ParamType::Number, "First number", true)
                    .add_param("b", ParamType::Number, "Second number", true),
                required_params: vec!["a".to_string(), "b".to_string()],
            },
            Armament {
                name: "multiply".to_string(),
                description: "Multiply two numbers".to_string(),
                schema: ToolSchema::new()
                    .add_param("a", ParamType::Number, "First number", true)
                    .add_param("b", ParamType::Number, "Second number", true),
                required_params: vec!["a".to_string(), "b".to_string()],
            },
        ])
    }

    async fn invoke(&self, call: &ArmamentCall) -> Result<ArmamentResult, ArsenalError> {
        let a = call.parameters.get("a")
            .and_then(|v| v.as_f64())
            .ok_or_else(|| ArsenalError::InvalidParameter("a".to_string()))?;

        let b = call.parameters.get("b")
            .and_then(|v| v.as_f64())
            .ok_or_else(|| ArsenalError::InvalidParameter("b".to_string()))?;

        let result = match call.tool_name.as_str() {
            "add" => a + b,
            "multiply" => a * b,
            _ => return Err(ArsenalError::ToolNotFound(call.tool_name.clone())),
        };

        Ok(ArmamentResult {
            call_id: call.call_id,
            success: true,
            output: result.to_string(),
            error: None,
            execution_time_ms: 1,
        })
    }

    fn validate_call(&self, call: &ArmamentCall) -> Result<(), ArsenalError> {
        // Validate tool exists
        let tools = self.list_tools().await?;
        if !tools.iter().any(|t| t.name == call.tool_name) {
            return Err(ArsenalError::ToolNotFound(call.tool_name.clone()));
        }

        // Validate required parameters
        let tool = tools.iter().find(|t| t.name == call.tool_name).unwrap();
        for param in &tool.required_params {
            if !call.parameters.contains_key(param) {
                return Err(ArsenalError::MissingParameter(param.clone()));
            }
        }

        Ok(())
    }
}

// Use the custom tool
let calculator = Arc::new(CalculatorTool);

let paladin = PaladinBuilder::new(llm_adapter)
    .add_armament(calculator)
    .build()?;
```

### API Integration Tool

```rust
use reqwest::Client;

pub struct WeatherTool {
    client: Client,
    api_key: String,
}

impl WeatherTool {
    pub fn new(api_key: String) -> Self {
        Self {
            client: Client::new(),
            api_key,
        }
    }
}

#[async_trait]
impl ArsenalPort for WeatherTool {
    async fn list_tools(&self) -> Result<Vec<Armament>, ArsenalError> {
        Ok(vec![
            Armament {
                name: "get_weather".to_string(),
                description: "Get current weather for a location".to_string(),
                schema: ToolSchema::new()
                    .add_param("location", ParamType::String, "City name or coordinates", true)
                    .add_param("units", ParamType::String, "Temperature units (celsius/fahrenheit)", false),
                required_params: vec!["location".to_string()],
            },
        ])
    }

    async fn invoke(&self, call: &ArmamentCall) -> Result<ArmamentResult, ArsenalError> {
        let location = call.parameters.get("location")
            .and_then(|v| v.as_str())
            .ok_or_else(|| ArsenalError::InvalidParameter("location".to_string()))?;

        let units = call.parameters.get("units")
            .and_then(|v| v.as_str())
            .unwrap_or("celsius");

        // Call weather API
        let url = format!(
            "https://api.openweathermap.org/data/2.5/weather?q={}&appid={}&units={}",
            location, self.api_key, units
        );

        let response = self.client.get(&url)
            .send()
            .await
            .map_err(|e| ArsenalError::ExecutionError(e.to_string()))?;

        let weather_data = response.json::<serde_json::Value>()
            .await
            .map_err(|e| ArsenalError::ExecutionError(e.to_string()))?;

        let temp = weather_data["main"]["temp"].as_f64().unwrap_or(0.0);
        let description = weather_data["weather"][0]["description"]
            .as_str()
            .unwrap_or("unknown");

        let output = format!(
            "Weather in {}: {} with temperature of {}°",
            location, description, temp
        );

        Ok(ArmamentResult {
            call_id: call.call_id,
            success: true,
            output,
            error: None,
            execution_time_ms: 200,
        })
    }

    fn validate_call(&self, call: &ArmamentCall) -> Result<(), ArsenalError> {
        if call.tool_name != "get_weather" {
            return Err(ArsenalError::ToolNotFound(call.tool_name.clone()));
        }

        if !call.parameters.contains_key("location") {
            return Err(ArsenalError::MissingParameter("location".to_string()));
        }

        Ok(())
    }
}

// Usage
let weather = Arc::new(WeatherTool::new(api_key));

let paladin = PaladinBuilder::new(llm_adapter)
    .system_prompt("You can check weather. Use get_weather tool.")
    .add_armament(weather)
    .build()?;
```

### Database Query Tool

```rust
use sqlx::SqlitePool;

pub struct DatabaseTool {
    pool: SqlitePool,
}

impl DatabaseTool {
    pub async fn new(database_url: &str) -> Result<Self, sqlx::Error> {
        let pool = SqlitePool::connect(database_url).await?;
        Ok(Self { pool })
    }
}

#[async_trait]
impl ArsenalPort for DatabaseTool {
    async fn list_tools(&self) -> Result<Vec<Armament>, ArsenalError> {
        Ok(vec![
            Armament {
                name: "query_database".to_string(),
                description: "Execute a read-only SQL query".to_string(),
                schema: ToolSchema::new()
                    .add_param("query", ParamType::String, "SQL SELECT query", true),
                required_params: vec!["query".to_string()],
            },
        ])
    }

    async fn invoke(&self, call: &ArmamentCall) -> Result<ArmamentResult, ArsenalError> {
        let query = call.parameters.get("query")
            .and_then(|v| v.as_str())
            .ok_or_else(|| ArsenalError::InvalidParameter("query".to_string()))?;

        // Security: Only allow SELECT queries
        if !query.trim().to_lowercase().starts_with("select") {
            return Ok(ArmamentResult {
                call_id: call.call_id,
                success: false,
                output: String::new(),
                error: Some("Only SELECT queries are allowed".to_string()),
                execution_time_ms: 0,
            });
        }

        let start = std::time::Instant::now();

        let rows = sqlx::query(query)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| ArsenalError::ExecutionError(e.to_string()))?;

        // Convert rows to JSON
        let result_json = serde_json::to_string_pretty(&rows)
            .unwrap_or_else(|_| "[]".to_string());

        Ok(ArmamentResult {
            call_id: call.call_id,
            success: true,
            output: result_json,
            error: None,
            execution_time_ms: start.elapsed().as_millis() as u64,
        })
    }

    fn validate_call(&self, call: &ArmamentCall) -> Result<(), ArsenalError> {
        if !call.parameters.contains_key("query") {
            return Err(ArsenalError::MissingParameter("query".to_string()));
        }
        Ok(())
    }
}
```

## Tool Result Handling

### Automatic Context Injection

When a Paladin invokes a tool, the result is automatically added to the conversation context:

```rust
// Paladin execution loop
loop {
    let response = llm.generate(context).await?;

    if let Some(tool_call) = response.tool_calls.first() {
        // Execute tool
        let result = arsenal.invoke(tool_call).await?;

        // Add result to context
        context.add_tool_result(result);

        // Continue reasoning with tool output
        continue;
    }

    // No more tool calls, return final response
    break Ok(response);
}
```

### Custom Result Processing

```rust
pub struct LoggingArsenalPort<T: ArsenalPort> {
    inner: T,
}

#[async_trait]
impl<T: ArsenalPort> ArsenalPort for LoggingArsenalPort<T> {
    async fn invoke(&self, call: &ArmamentCall) -> Result<ArmamentResult, ArsenalError> {
        println!("Invoking tool: {}", call.tool_name);
        println!("Parameters: {:?}", call.parameters);

        let start = std::time::Instant::now();
        let result = self.inner.invoke(call).await?;
        let duration = start.elapsed();

        println!("Tool completed in {:?}", duration);
        println!("Success: {}", result.success);

        if let Some(error) = &result.error {
            eprintln!("Tool error: {}", error);
        }

        Ok(result)
    }

    // Forward other methods
    async fn list_tools(&self) -> Result<Vec<Armament>, ArsenalError> {
        self.inner.list_tools().await
    }

    fn validate_call(&self, call: &ArmamentCall) -> Result<(), ArsenalError> {
        self.inner.validate_call(call)
    }
}

// Usage
let weather_tool = Arc::new(WeatherTool::new(api_key));
let logged_tool = Arc::new(LoggingArsenalPort { inner: weather_tool });

paladin.add_armament(logged_tool);
```

### Error Handling

```rust
match arsenal.invoke(&call).await {
    Ok(result) if result.success => {
        // Tool succeeded
        process_result(&result.output);
    }
    Ok(result) => {
        // Tool failed but returned error message
        eprintln!("Tool failed: {}", result.error.unwrap_or_default());
        // Decide: retry, use fallback, or fail
    }
    Err(ArsenalError::ToolNotFound(name)) => {
        eprintln!("Tool not found: {}", name);
        // Handle missing tool
    }
    Err(ArsenalError::Timeout) => {
        eprintln!("Tool execution timed out");
        // Retry with longer timeout
    }
    Err(e) => {
        eprintln!("Arsenal error: {}", e);
        // Handle other errors
    }
}
```

## Best Practices

### 1. Clear Tool Descriptions

```rust
// ❌ Bad: Vague description
Armament {
    name: "search",
    description: "Search for stuff",
    // ...
}

// ✅ Good: Clear, specific description
Armament {
    name: "web_search",
    description: "Search the web using Google. Returns top 10 results with titles, \
                  URLs, and snippets. Use this when you need current information \
                  not in your training data.",
    // ...
}
```

### 2. Validate Inputs

```rust
fn validate_call(&self, call: &ArmamentCall) -> Result<(), ArsenalError> {
    // Check required parameters
    for param in &self.required_params {
        if !call.parameters.contains_key(param) {
            return Err(ArsenalError::MissingParameter(param.clone()));
        }
    }

    // Validate parameter types and values
    if let Some(url) = call.parameters.get("url") {
        if !url.as_str().unwrap_or("").starts_with("http") {
            return Err(ArsenalError::InvalidParameter("url must start with http".into()));
        }
    }

    Ok(())
}
```

### 3. Set Timeouts

```rust
let tool = CustomTool::new()
    .timeout(Duration::from_secs(30))  // Prevent hanging
    .build()?;
```

### 4. Implement Retries for Flaky Operations

```rust
async fn invoke_with_retry(&self, call: &ArmamentCall) -> Result<ArmamentResult, ArsenalError> {
    let mut attempts = 0;
    let max_attempts = 3;

    loop {
        attempts += 1;

        match self.invoke(call).await {
            Ok(result) => return Ok(result),
            Err(e) if attempts < max_attempts && e.is_retryable() => {
                tokio::time::sleep(Duration::from_secs(2_u64.pow(attempts))).await;
                continue;
            }
            Err(e) => return Err(e),
        }
    }
}
```

### 5. Sanitize Inputs

```rust
fn sanitize_sql(query: &str) -> Result<String, ArsenalError> {
    // Remove dangerous keywords
    let dangerous = ["DROP", "DELETE", "UPDATE", "INSERT", "CREATE", "ALTER"];
    let query_upper = query.to_uppercase();

    for keyword in dangerous {
        if query_upper.contains(keyword) {
            return Err(ArsenalError::SecurityViolation(
                format!("Query contains forbidden keyword: {}", keyword)
            ));
        }
    }

    Ok(query.to_string())
}
```

### 6. Rate Limiting

```rust
use std::sync::Arc;
use tokio::sync::Semaphore;

pub struct RateLimitedTool<T: ArsenalPort> {
    inner: T,
    semaphore: Arc<Semaphore>,
}

impl<T: ArsenalPort> RateLimitedTool<T> {
    pub fn new(inner: T, max_concurrent: usize) -> Self {
        Self {
            inner,
            semaphore: Arc::new(Semaphore::new(max_concurrent)),
        }
    }
}

#[async_trait]
impl<T: ArsenalPort> ArsenalPort for RateLimitedTool<T> {
    async fn invoke(&self, call: &ArmamentCall) -> Result<ArmamentResult, ArsenalError> {
        let _permit = self.semaphore.acquire().await
            .map_err(|e| ArsenalError::ExecutionError(e.to_string()))?;

        self.inner.invoke(call).await
    }

    // Forward other methods...
}
```

### 7. Structured Output

```rust
// Return structured data that's easy to parse
let output = serde_json::json!({
    "status": "success",
    "data": {
        "temperature": 72.5,
        "conditions": "partly cloudy",
        "humidity": 65
    },
    "timestamp": chrono::Utc::now().to_rfc3339()
});

Ok(ArmamentResult {
    call_id: call.call_id,
    success: true,
    output: output.to_string(),
    error: None,
    execution_time_ms: 150,
})
```

## Troubleshooting

### Tool Not Being Called

**Problem**: Paladin doesn't use the tool even though it should.

**Solutions**:
1. Check tool description is clear and relevant
2. Update system prompt to mention tool availability
3. Verify tool appears in `list_tools()` output
4. Ensure LLM supports function calling (GPT-4, Claude 3+)

```rust
// Make tool usage explicit in system prompt
.system_prompt("You have access to a web_search tool. USE IT to find current information. \
                Always search before answering questions about recent events.")
```

### MCP Server Connection Failed

**Problem**: Cannot connect to MCP STDIO server.

**Solutions**:
1. Verify command is in PATH: `which uvx`
2. Test command manually: `uvx mcp-server-fetch`
3. Check server logs for errors
4. Verify environment variables are set

```rust
let tool = MCPStdioAdapter::new()
    .command("uvx")
    .args(vec!["mcp-server-fetch"])
    .debug_mode(true)  // Enable verbose logging
    .build()
    .await?;
```

### Tool Execution Timeout

**Problem**: Tools timing out frequently.

**Solutions**:
1. Increase timeout duration
2. Optimize tool implementation
3. Add caching for expensive operations
4. Use async/parallel execution where possible

```rust
let tool = CustomTool::new()
    .timeout(Duration::from_secs(120))  // Longer timeout
    .build()?;
```

### Invalid Parameters

**Problem**: Tool receives wrong parameter types.

**Solutions**:
1. Strengthen parameter validation
2. Add type coercion in invoke()
3. Improve tool schema definitions
4. Add examples to tool descriptions

```rust
// Robust parameter extraction
let count = call.parameters.get("count")
    .and_then(|v| {
        // Try as number, then as string
        v.as_i64()
            .or_else(|| v.as_str().and_then(|s| s.parse::<i64>().ok()))
    })
    .unwrap_or(10);  // Default value
```

### SSE Server Authentication

**Problem**: SSE server returns 401 Unauthorized.

**Solutions**:
1. Verify API key is correct
2. Check token hasn't expired
3. Ensure correct authentication method (bearer vs api-key)
4. Check server CORS settings

```rust
let tool = MCPSseAdapter::new()
    .endpoint("https://api.example.com/mcp")
    .bearer_token("your-token")  // Use bearer auth instead of api_key
    .build()
    .await?;
```

## Testing Tools

### Unit Testing Custom Tools

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_calculator_add() {
        let calc = CalculatorTool;

        let call = ArmamentCall {
            tool_name: "add".to_string(),
            parameters: HashMap::from([
                ("a".to_string(), json!(5.0)),
                ("b".to_string(), json!(3.0)),
            ]),
            call_id: Uuid::new_v4(),
        };

        let result = calc.invoke(&call).await.unwrap();

        assert!(result.success);
        assert_eq!(result.output, "8");
    }

    #[tokio::test]
    async fn test_invalid_parameter() {
        let calc = CalculatorTool;

        let call = ArmamentCall {
            tool_name: "add".to_string(),
            parameters: HashMap::from([
                ("a".to_string(), json!(5.0)),
                // Missing 'b' parameter
            ]),
            call_id: Uuid::new_v4(),
        };

        assert!(calc.invoke(&call).await.is_err());
    }
}
```

### Integration Testing with Paladin

```rust
#[tokio::test]
async fn test_paladin_uses_tool() {
    let llm_adapter = Arc::new(MockLlmAdapter::new());
    let calc = Arc::new(CalculatorTool);

    let paladin = PaladinBuilder::new(llm_adapter)
        .system_prompt("You have a calculator. Use it for math.")
        .add_armament(calc)
        .build()
        .unwrap();

    let response = paladin.execute("What is 15 + 27?").await.unwrap();

    assert!(response.content.contains("42"));
}
```

## Examples

See working examples:
- `examples/arsenal_stdio_tools.rs` - MCP STDIO integration
- `examples/arsenal_sse_tools.rs` - MCP SSE integration
- `examples/custom_tools.rs` - Custom tool implementation
- `examples/tool_error_handling.rs` - Error handling patterns

## Next Steps

- **[Memory Management](memory-management.md)** - Use Garrison with tools
- **[Battalion Patterns](battalion-patterns.md)** - Tools in multi-agent systems
- **[API Reference](https://docs.rs/paladin)** - Arsenal API documentation

## Related Resources

- [MCP Specification](https://modelcontextprotocol.io/)
- [MCP Server Examples](https://github.com/modelcontextprotocol/servers)
- [Tool Development Best Practices](../contributing/adapter-development.md)
