//! Example: Arsenal Tool System - SSE MCP Concept
//!
//! This example demonstrates the concepts of using Arsenal with SSE-based
//! (Server-Sent Events) MCP (Model Context Protocol) servers. It shows:
//! - SSE transport patterns
//! - Remote tool integration
//! - HTTP-based tool communication
//!
//! For actual MCP SSE implementation, see:
//! - `src/infrastructure/adapters/arsenal/mcp_sse_adapter.rs`
//! - `tests/integration/mcp_sse_test.rs`

use paladin::application::ports::output::arsenal_port::ArsenalRegistry;
use paladin::application::use_cases::arsenal::arsenal_registry_service::ArsenalRegistryService;
use paladin::core::platform::container::arsenal::Armament;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("══════════════════════════════════════════════════════");
    println!("  Arsenal SSE MCP - Concept Demonstration");
    println!("══════════════════════════════════════════════════════\n");

    println!("SSE MCP Transport:");
    println!("  • Communicates via HTTP/SSE");
    println!("  • Connects to remote tool servers");
    println!("  • JSON-RPC 2.0 protocol over HTTP");
    println!("  • Supports retry with exponential backoff");
    println!("  • Examples: Web services, cloud functions, microservices\n");

    // Step 1: Create registry
    println!("Step 1: Creating Arsenal registry...");
    let registry = ArsenalRegistryService::new();
    println!("  ✓ Registry created\n");

    // Step 2: Register remote tools (these would come from MCP discovery)
    println!("Step 2: Registering remote tools from MCP server...");

    let web_search = Armament {
        name: "web_search".to_string(),
        description: "Searches the web for information".to_string(),
        parameters: json!({
            "type": "object",
            "properties": {
                "query": {
                    "type": "string",
                    "description": "Search query"
                },
                "num_results": {
                    "type": "number",
                    "description": "Number of results to return",
                    "default": 5
                }
            }
        }),
        required_params: vec!["query".to_string()],
    };

    let weather_api = Armament {
        name: "get_weather".to_string(),
        description: "Gets current weather for a location".to_string(),
        parameters: json!({
            "type": "object",
            "properties": {
                "location": {
                    "type": "string",
                    "description": "City name or coordinates"
                },
                "units": {
                    "type": "string",
                    "enum": ["celsius", "fahrenheit"],
                    "default": "celsius"
                }
            }
        }),
        required_params: vec!["location".to_string()],
    };

    registry.register(web_search.clone()).await;
    registry.register(weather_api.clone()).await;

    println!("  ✓ Registered 2 remote tools\n");

    // Step 3: Display registered tools
    println!("Step 3: Available remote tools:");
    println!("  🌐 web_search");
    println!("     {}", web_search.description);
    println!("     Required: {:?}\n", web_search.required_params);

    println!("  🌐 get_weather");
    println!("     {}", weather_api.description);
    println!("     Required: {:?}\n", weather_api.required_params);

    // Step 4: Show SSE advantages
    println!("Step 4: SSE MCP Advantages:");
    println!("  • Remote tool execution");
    println!("  • HTTP-based (firewall friendly)");
    println!("  • Automatic reconnection");
    println!("  • Load balancing support");
    println!("  • Cloud-native deployment\n");

    // Step 5: Show usage pattern
    println!("Step 5: How to use SSE MCP in production:");
    println!("  ```rust");
    println!("  // Create SSE adapter");
    println!("  let adapter = MCPSseAdapter::new(\"https://tools.example.com/mcp\")");
    println!("      .with_timeout(Duration::from_secs(10))");
    println!("      .with_retry_config(3, Duration::from_secs(1));");
    println!();
    println!("  adapter.connect().await?;");
    println!();
    println!("  // Discover tools");
    println!("  let client = MCPClient::new(Box::new(adapter));");
    println!("  let tools = client.discover_tools().await?;");
    println!();
    println!("  // Register with Arsenal");
    println!("  for tool in tools {{");
    println!("      registry.register(tool)?;");
    println!("  }}");
    println!();
    println!("  // Use in Paladin");
    println!("  let paladin = PaladinBuilder::new(llm)");
    println!("      .with_arsenal_registry(Arc::new(registry))");
    println!("      .build()?;");
    println!("  ```\n");

    println!("══════════════════════════════════════════════════════");
    println!("  Key Features:");
    println!("    • HTTP-based communication");
    println!("    • Connection timeout handling");
    println!("    • Automatic retry with exponential backoff");
    println!("    • Suitable for microservices architecture");
    println!();
    println!("  See also:");
    println!("    • examples/arsenal_stdio_tools.rs (STDIO transport)");
    println!("    • tests/integration/mcp_sse_test.rs");
    println!("══════════════════════════════════════════════════════");

    Ok(())
}
