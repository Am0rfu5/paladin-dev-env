# Tool Integration Guide

This guide covers how to integrate external tools and capabilities into your Paladins using the Arsenal system and Model Context Protocol (MCP).

## Table of Contents

- [Overview](#overview)
- [Arsenal Architecture](#arsenal-architecture)
- [MCP Protocol](#mcp-protocol)
- [STDIO Tool Servers](#stdio-tool-servers)
- [Streamable-HTTP Tool Servers](#streamable-http-tool-servers)
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

```rust,ignore
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
    async fn list_armaments(&self) -> Result<Vec<Armament>>;
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
2. **Streamable-HTTP Servers**: Remote, optionally authenticated web services
   (the current, real transport; supersedes the legacy standalone SSE
   transport per the MCP spec — Paladin's own retired `MCPSseAdapter` was
   never actually SSE, just a mislabeled plain-HTTP-POST adapter)

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

```rust,ignore
use paladin::infrastructure::adapters::arsenal::mcp_stdio_adapter::MCPStdioAdapter;
use paladin::application::services::paladin::paladin_builder::PaladinBuilder;
use paladin::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let llm_adapter = Arc::new(OpenAiAdapter::new().build()?);

    // Connect to an MCP STDIO server: MCPStdioAdapter::new(command, args) is
    // a thin builder; connect() spawns the subprocess and performs the full
    // MCP initialize -> notifications/initialized handshake, returning an
    // MCPClient with discover_tools()/invoke_tool().
    let mcp_client = MCPStdioAdapter::new("uvx", vec!["mcp-server-fetch"])
        .connect()
        .await?;
    let tools = mcp_client.discover_tools().await?;

    // Build Paladin with tool access (register discovered tools, or attach
    // an ArsenalRegistry backed by the connected client)
    let paladin = PaladinBuilder::new(llm_adapter)
        .name("ResearchAssistant")
        .system_prompt("You are a research assistant with web search capabilities. \
                        Use the web_search tool to find current information. \
                        Always cite your sources.")
        .build()
        .await?;

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

`MCPStdioAdapter` is currently a minimal builder — it only accepts a command
and its arguments; working-directory, per-server env-var injection, custom
timeouts, and retry policies are not yet exposed on the adapter itself
(pass any required env vars via the spawned command's own args/environment,
or via the process that launches Paladin):

```rust,ignore
let client = MCPStdioAdapter::new("uvx", vec!["mcp-server-fetch"])
    .connect()
    .await?;
```

## Streamable-HTTP Tool Servers

Streamable-HTTP servers are remote, optionally authenticated MCP servers reached over
HTTP(S) — Paladin's real remote transport (Phase 12.1 D-02/D-03). This supersedes the
legacy standalone SSE transport in the MCP spec; Paladin's own retired `MCPSseAdapter`
was never actually SSE, just a mislabeled, unauthenticated plain-HTTP-POST adapter, and
has been removed entirely.

### Connecting a Streamable-HTTP Server

```rust,ignore
use paladin::infrastructure::adapters::arsenal::mcp_streamable_http_adapter::MCPStreamableHttpAdapter;
use paladin::application::services::paladin::paladin_builder::PaladinBuilder;
use paladin::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let llm_adapter = Arc::new(OpenAiAdapter::new().build()?);

    // Connect to a remote MCP server over Streamable-HTTP. The bearer token
    // is read from an env var here in application code -- never hardcode a
    // literal secret, and never accept one as a CLI argument.
    let bearer_token = std::env::var("API_KEY").ok();
    let mut adapter = MCPStreamableHttpAdapter::new("https://api.example.com/mcp");
    if let Some(token) = bearer_token {
        adapter = adapter.with_bearer_token(token);
    }
    let mcp_client = adapter.connect().await?;
    let tools = mcp_client.discover_tools().await?;

    let paladin = PaladinBuilder::new(llm_adapter)
        .name("APIAssistant")
        .system_prompt("You have access to company APIs. Use them to retrieve data.")
        .build()
        .await?;

    let response = paladin.execute("Get user statistics for last month").await?;
    println!("{}", response.content);

    Ok(())
}
```

### Streamable-HTTP Configuration

The config-driven flow (recommended for most use cases) declares the server in
`config.yml`/`.mcp.json` and lets the CLI/loader connect it for you:

```yaml
arsenal:
  mcp_servers:
    - name: "company_api"
      type: "streamable_http"
      endpoint: "https://api.example.com/mcp"
      # NAMES the env var holding the bearer token -- never a literal secret
      # in this file. Omit entirely for an unauthenticated server.
      auth_token_env: "API_KEY"
```

```bash
export API_KEY="your-token"
paladin arsenal test --mcp-streamable-http "https://api.example.com/mcp" \
    --mcp-auth-token-env API_KEY
```

The equivalent programmatic flow — `MCPStreamableHttpAdapter` is a thin builder over
[`MCPClient::connect_streamable_http`], mirroring `MCPStdioAdapter`'s shape for the
remote transport:

```rust,ignore
use paladin::infrastructure::adapters::arsenal::mcp_streamable_http_adapter::MCPStreamableHttpAdapter;

let adapter = MCPStreamableHttpAdapter::new("https://api.example.com/mcp")
    .with_bearer_token(std::env::var("API_KEY")?); // never hardcode the token
    // .with_custom_headers(headers) is also available for non-bearer auth schemes

let client = adapter.connect().await?; // full initialize -> notifications/initialized handshake
```

Or call the underlying `MCPClient` constructor directly:

```rust,ignore
use paladin::infrastructure::adapters::arsenal::mcp_protocol::MCPClient;

let client = MCPClient::connect_streamable_http(
    "https://api.example.com/mcp",
    Some(&std::env::var("API_KEY")?),
    None, // optional custom headers
)
.await?;
```

### Verifying Connectivity and Listing Tools

There is no separate `health_check()` — a successful `connect()`/`connect_streamable_http()`
already means the full MCP handshake succeeded, so `discover_tools()` doubles as the
liveness check:

```rust,ignore
// A successful discover_tools() call confirms the server is reachable and
// the handshake succeeded.
let tools = client.discover_tools().await?;
println!("Streamable-HTTP server is healthy — {} tool(s) available", tools.len());
for tool in tools {
    println!("Tool: {} - {}", tool.name, tool.description);
}
```

## Custom Tool Development

Create your own tools by implementing the `ArsenalPort` trait.

### Simple Custom Tool

```rust,ignore
use paladin_ports::output::arsenal_port::{ArsenalPort, ArsenalRegistry};
use paladin_core::platform::container::arsenal::{Armament, ArmamentCall, ArmamentResult};
use async_trait::async_trait;

pub struct CalculatorTool;

#[async_trait]
impl ArsenalPort for CalculatorTool {
    async fn list_armaments(&self) -> Result<Vec<Armament>, ArsenalError> {
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
        let tools = self.list_armaments().await?;
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

```rust,ignore
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
    async fn list_armaments(&self) -> Result<Vec<Armament>, ArsenalError> {
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

```rust,ignore
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
    async fn list_armaments(&self) -> Result<Vec<Armament>, ArsenalError> {
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

```rust,ignore
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

```rust,ignore
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
    async fn list_armaments(&self) -> Result<Vec<Armament>, ArsenalError> {
        self.inner.list_armaments().await
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

```rust,ignore
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

```rust,ignore
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

```rust,ignore
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

```rust,ignore
let tool = CustomTool::new()
    .timeout(Duration::from_secs(30))  // Prevent hanging
    .build()?;
```

### 4. Implement Retries for Flaky Operations

```rust,ignore
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

```rust,ignore
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

```rust,ignore
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

```rust,ignore
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
3. Verify tool appears in `list_armaments()` output
4. Ensure LLM supports function calling (GPT-4, Claude 3+)

```rust,ignore
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

```rust,ignore
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

```rust,ignore
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

```rust,ignore
// Robust parameter extraction
let count = call.parameters.get("count")
    .and_then(|v| {
        // Try as number, then as string
        v.as_i64()
            .or_else(|| v.as_str().and_then(|s| s.parse::<i64>().ok()))
    })
    .unwrap_or(10);  // Default value
```

### Streamable-HTTP Server Authentication

**Problem**: Streamable-HTTP server rejects the connection with `ArsenalError::AuthFailed`
(a 401/403-shaped rejection during the `initialize` handshake).

**Solutions**:
1. Verify the bearer token is correct and hasn't expired
2. Confirm the token is NOT prefixed with `"Bearer "` yourself — `MCPClient::connect_streamable_http`
   / `MCPStreamableHttpAdapter` add the prefix internally; a manually-prefixed token double-prefixes
   and breaks auth
3. If using `auth_token_env`/`--mcp-auth-token-env`, confirm the NAMED env var is actually set in
   the process environment (a missing env var fails loud with an actionable error, not a silent 401)
4. Check the server's own auth requirements (some accept custom headers instead of a bearer token —
   use `MCPStreamableHttpAdapter::with_custom_headers`)

```rust,ignore
use paladin::infrastructure::adapters::arsenal::mcp_streamable_http_adapter::MCPStreamableHttpAdapter;

let adapter = MCPStreamableHttpAdapter::new("https://api.example.com/mcp")
    .with_bearer_token(std::env::var("API_KEY")?); // never a literal token, never "Bearer "-prefixed

let client = adapter.connect().await?;
```

## Testing Tools

### Unit Testing Custom Tools

```rust,ignore
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

```rust,ignore
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
- `examples/arsenal_streamable_http_tools.rs` - MCP Streamable-HTTP integration (authenticated remote transport)
- `examples/custom_tools.rs` - Custom tool implementation
- `examples/tool_error_handling.rs` - Error handling patterns

## Next Steps

- **[Memory Management](memory-management.md)** - Use Garrison with tools
- **[Battalion Patterns](battalion-patterns.md)** - Tools in multi-agent systems
- **[API Reference](https://docs.rs/paladin)** - Arsenal API documentation

## Related Resources

- [MCP Specification](https://modelcontextprotocol.io/)
- [MCP Server Examples](https://github.com/modelcontextprotocol/servers)
- [Tool Development Best Practices](../contributing/contributing-providers.md)
