//! Example: Arsenal Tool System - STDIO MCP Concept
//!
//! This example demonstrates the concepts of using Arsenal with STDIO-based
//! MCP (Model Context Protocol) servers. It shows:
//! - Tool registration in Arsenal
//! - Tool metadata structure
//! - Integration patterns
//!
//! For actual MCP STDIO implementation, see:
//! - `src/infrastructure/adapters/arsenal/mcp_stdio_adapter.rs`
//! - `tests/integration/mcp_stdio_test.rs`

use paladin::application::services::arsenal::arsenal_registry_service::ArsenalRegistryService;
use paladin::core::platform::container::arsenal::Armament;
use paladin_ports::output::arsenal_port::ArsenalRegistry;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("══════════════════════════════════════════════════════");
    println!("  Arsenal STDIO MCP - Concept Demonstration");
    println!("══════════════════════════════════════════════════════\n");

    println!("STDIO MCP Transport:");
    println!("  • Communicates via stdin/stdout");
    println!("  • Spawns subprocess for tool servers");
    println!("  • JSON-RPC 2.0 protocol over pipes");
    println!("  • Examples: Python scripts, Node.js tools, CLI programs\n");

    // Step 1: Create registry
    println!("Step 1: Creating Arsenal registry...");
    let registry = ArsenalRegistryService::new();
    println!("  ✓ Registry created\n");

    // Step 2: Register tools (these would come from MCP discovery)
    println!("Step 2: Registering tools from MCP server...");

    let calculator = Armament {
        name: "calculator".to_string(),
        description: "Performs arithmetic operations (add, subtract, multiply, divide)".to_string(),
        parameters: json!({
            "type": "object",
            "properties": {
                "operation": {
                    "type": "string",
                    "enum": ["add", "subtract", "multiply", "divide"]
                },
                "x": {"type": "number", "description": "First operand"},
                "y": {"type": "number", "description": "Second operand"}
            }
        }),
        required_params: vec!["operation".to_string(), "x".to_string(), "y".to_string()],
    };

    let file_reader = Armament {
        name: "read_file".to_string(),
        description: "Reads content from a file".to_string(),
        parameters: json!({
            "type": "object",
            "properties": {
                "path": {"type": "string", "description": "File path to read"}
            }
        }),
        required_params: vec!["path".to_string()],
    };

    registry.register(calculator.clone()).await;
    registry.register(file_reader.clone()).await;

    println!("  ✓ Registered 2 tools\n");

    // Step 3: Display registered tools
    println!("Step 3: Available tools:");
    println!("  📦 calculator");
    println!("     {}", calculator.description);
    println!("     Required: {:?}\n", calculator.required_params);

    println!("  📦 read_file");
    println!("     {}", file_reader.description);
    println!("     Required: {:?}\n", file_reader.required_params);

    // Step 4: Show usage pattern
    println!("Step 4: How to use STDIO MCP in production:");
    println!("  ```rust");
    println!("  // Create STDIO adapter");
    println!("  let adapter = MCPStdioAdapter::new(\"python3\", vec![\"server.py\"]);");
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
    println!("      .build().await?;");
    println!("  ```\n");

    println!("══════════════════════════════════════════════════════");
    println!("  Key Concepts:");
    println!("    • Tools are discovered via MCP protocol");
    println!("    • Arsenal manages tool registry");
    println!("    • Paladin invokes tools during execution");
    println!("    • Results are formatted and injected into context");
    println!();
    println!("  See also:");
    println!("    • examples/arsenal_sse_tools.rs (SSE transport)");
    println!("    • tests/integration/mcp_stdio_test.rs");
    println!("══════════════════════════════════════════════════════");

    Ok(())
}
