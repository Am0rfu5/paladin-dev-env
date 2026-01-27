//! Arsenal command implementations for MCP tool management
//!
//! This module provides CLI commands for discovering and testing MCP
//! (Model Context Protocol) tools and servers.
//!
//! # MCP Protocol Support
//!
//! - **STDIO**: Command-line tools executed via stdin/stdout
//! - **SSE**: HTTP-based remote tool services
//!
//! # Examples
//!
//! ```bash
//! # List all configured MCP tools
//! paladin arsenal list
//!
//! # Test an STDIO MCP server
//! paladin arsenal test --mcp-stdio "uvx mcp-web-search"
//!
//! # Test an SSE MCP server
//! paladin arsenal test --mcp-sse "http://localhost:8080/mcp"
//! ```

use crate::cli::output::errors::CliError;
use clap::Subcommand;
use colored::Colorize;

/// Arsenal subcommands for MCP tool management
#[derive(Debug, Subcommand)]
pub enum ArsenalCommands {
    /// List available MCP tools
    List,
    /// Test an MCP server connection
    Test(ArsenalTestArgs),
}

/// Arguments for testing MCP server connections
#[derive(Debug, clap::Args)]
pub struct ArsenalTestArgs {
    /// Test STDIO-based MCP server (e.g., "uvx mcp-web-search")
    #[arg(long, conflicts_with = "mcp_sse")]
    pub mcp_stdio: Option<String>,

    /// Test SSE-based MCP server (endpoint URL)
    #[arg(long, conflicts_with = "mcp_stdio")]
    pub mcp_sse: Option<String>,
}

/// Handle the arsenal commands
pub async fn handle_arsenal_command(command: ArsenalCommands) -> Result<(), CliError> {
    match command {
        ArsenalCommands::List => handle_arsenal_list().await,
        ArsenalCommands::Test(args) => handle_arsenal_test(args).await,
    }
}

/// List available MCP tools from configured servers
pub async fn handle_arsenal_list() -> Result<(), CliError> {
    use crate::config::application_settings::Settings;

    println!("{} Discovering MCP tools...\n", "→".cyan().bold());

    // Load configuration
    let config = Settings::new().map_err(|e| CliError::ValidationError {
        message: format!("Failed to load configuration: {}", e),
    })?;

    let arsenal_config = config
        .arsenal
        .as_ref()
        .ok_or_else(|| CliError::ValidationError {
            message: "Arsenal configuration not found".to_string(),
        })?;

    if arsenal_config.mcp_servers.is_empty() {
        println!("{} No MCP servers configured.", "⚠".yellow().bold());
        println!("\nTo add MCP servers, edit your config.yml file:");
        println!("\narsenal:");
        println!("  mcp_servers:");
        println!("    - name: \"web_search\"");
        println!("      server_type: \"stdio\"");
        println!("      command: \"uvx\"");
        println!("      args: [\"mcp-web-search\"]");
        return Ok(());
    }

    println!(
        "{} Found {} configured MCP server(s)\n",
        "✓".green().bold(),
        arsenal_config.mcp_servers.len()
    );

    // Collect all discovered tools
    struct ToolEntry {
        name: String,
        description: String,
        server_name: String,
        server_type: String,
        status: String,
    }

    let mut all_tools: Vec<ToolEntry> = Vec::new();

    // Try to connect to each configured MCP server
    for server_config in &arsenal_config.mcp_servers {
        println!(
            "{} Connecting to '{}' ({})...",
            "→".cyan(),
            server_config.name,
            server_config.server_type
        );

        match server_config.server_type.as_str() {
            "stdio" => {
                let command = server_config.command.as_ref().ok_or_else(|| {
                    CliError::MissingRequiredField {
                        field: "command".to_string(),
                        message: format!(
                            "MCP server '{}' is type 'stdio' but missing 'command' field",
                            server_config.name
                        ),
                    }
                })?;

                let args = server_config.args.as_ref().cloned().unwrap_or_default();

                // Try to connect and discover tools
                match connect_and_discover_stdio(command, args).await {
                    Ok(tools) => {
                        println!("  {} Discovered {} tool(s)", "✓".green(), tools.len());
                        for tool in tools {
                            all_tools.push(ToolEntry {
                                name: tool.name,
                                description: tool.description,
                                server_name: server_config.name.clone(),
                                server_type: server_config.server_type.clone(),
                                status: "connected".to_string(),
                            });
                        }
                    }
                    Err(e) => {
                        println!("  {} Connection failed: {}", "✗".red(), e);
                        all_tools.push(ToolEntry {
                            name: format!("<{}>", server_config.name),
                            description: format!("Connection failed: {}", e),
                            server_name: server_config.name.clone(),
                            server_type: server_config.server_type.clone(),
                            status: "failed".to_string(),
                        });
                    }
                }
            }
            "sse" => {
                println!("  {} SSE servers not yet implemented", "⚠".yellow());
                all_tools.push(ToolEntry {
                    name: format!("<{}>", server_config.name),
                    description: "SSE servers not yet implemented".to_string(),
                    server_name: server_config.name.clone(),
                    server_type: server_config.server_type.clone(),
                    status: "unsupported".to_string(),
                });
            }
            unknown => {
                println!("  {} Unknown server type: {}", "✗".red(), unknown);
            }
        }
    }

    // Display results table
    if all_tools.is_empty() {
        println!("\n{} No tools discovered", "⚠".yellow().bold());
        return Ok(());
    }

    println!("\n{}", "═".repeat(120));
    println!(
        "{:30} | {:50} | {:10} | {:10} | {}",
        "Tool Name".bold(),
        "Description".bold(),
        "Server".bold(),
        "Type".bold(),
        "Status".bold()
    );
    println!("{}", "═".repeat(120));

    for tool in &all_tools {
        let status_colored = match tool.status.as_str() {
            "connected" => tool.status.green(),
            "failed" => tool.status.red(),
            "unsupported" => tool.status.yellow(),
            _ => tool.status.white(),
        };

        // Truncate long descriptions
        let description = if tool.description.len() > 47 {
            format!("{}...", &tool.description[..47])
        } else {
            tool.description.clone()
        };

        println!(
            "{:30} | {:50} | {:10} | {:10} | {}",
            tool.name, description, tool.server_name, tool.server_type, status_colored
        );
    }

    println!("{}", "═".repeat(120));
    println!(
        "\n{} {} tool(s) from {} server(s)",
        "✓".green().bold(),
        all_tools.iter().filter(|t| t.status == "connected").count(),
        arsenal_config.mcp_servers.len()
    );

    Ok(())
}

/// Connect to an MCP STDIO server and discover tools
async fn connect_and_discover_stdio(
    command: &str,
    args: Vec<String>,
) -> Result<Vec<crate::core::platform::container::arsenal::Armament>, String> {
    use crate::infrastructure::adapters::arsenal::mcp_protocol::MCPClient;
    use crate::infrastructure::adapters::arsenal::mcp_stdio_adapter::MCPStdioAdapter;

    // Create and connect STDIO adapter
    let mut adapter = MCPStdioAdapter::new(command, args);
    adapter
        .connect()
        .await
        .map_err(|e| format!("Connection failed: {}", e))?;

    // Create MCP client
    let client = MCPClient::new(Box::new(adapter));

    // Discover tools
    client
        .discover_tools()
        .await
        .map_err(|e| format!("Tool discovery failed: {}", e))
}

/// Test an MCP server connection with timing and diagnostics
async fn handle_arsenal_test(args: ArsenalTestArgs) -> Result<(), CliError> {
    use crate::infrastructure::adapters::arsenal::mcp_protocol::MCPClient;
    use crate::infrastructure::adapters::arsenal::mcp_stdio_adapter::MCPStdioAdapter;
    use std::time::Instant;

    // Validate mutually exclusive args
    if args.mcp_stdio.is_none() && args.mcp_sse.is_none() {
        return Err(CliError::MissingRequiredField {
            field: "mcp_stdio or mcp_sse".to_string(),
            message: "You must specify either --mcp-stdio or --mcp-sse".to_string(),
        });
    }

    println!("{} Testing MCP server connection...\n", "→".cyan().bold());

    // Handle STDIO server testing
    if let Some(stdio_command) = args.mcp_stdio {
        println!("{} Server type: {}", "→".cyan(), "STDIO".bold());
        println!("{} Command string: {}", "→".cyan(), stdio_command.cyan());

        // Parse command and args
        let parts: Vec<&str> = stdio_command.split_whitespace().collect();
        if parts.is_empty() {
            return Err(CliError::InvalidFieldValue {
                field: "mcp_stdio".to_string(),
                message: "Command string cannot be empty".to_string(),
            });
        }

        let command = parts[0];
        let args_vec: Vec<String> = parts[1..].iter().map(|s| s.to_string()).collect();

        println!("{} Parsed command: {}", "→".cyan(), command.yellow());
        if !args_vec.is_empty() {
            println!("{} Arguments: {}", "→".cyan(), args_vec.join(" ").yellow());
        }

        println!("\n{} Connecting to MCP server...", "→".cyan().bold());

        // Create adapter and measure connection time
        let start = Instant::now();
        let mut adapter = MCPStdioAdapter::new(command, args_vec);

        match adapter.connect().await {
            Ok(_) => {
                let connection_time = start.elapsed();
                println!(
                    "{} Connected successfully in {:.2}ms\n",
                    "✓".green().bold(),
                    connection_time.as_secs_f64() * 1000.0
                );

                // Create MCP client and discover tools
                println!("{} Discovering available tools...", "→".cyan());
                let client = MCPClient::new(Box::new(adapter));

                let discovery_start = Instant::now();
                match client.discover_tools().await {
                    Ok(tools) => {
                        let discovery_time = discovery_start.elapsed();
                        println!(
                            "{} Discovered {} tool(s) in {:.2}ms\n",
                            "✓".green().bold(),
                            tools.len(),
                            discovery_time.as_secs_f64() * 1000.0
                        );

                        if tools.is_empty() {
                            println!(
                                "{} No tools available from this server",
                                "⚠".yellow().bold()
                            );
                        } else {
                            // Display tools table
                            println!("{}", "═".repeat(100));
                            println!("{:30} | {}", "Tool Name".bold(), "Description".bold());
                            println!("{}", "═".repeat(100));

                            for tool in &tools {
                                let description = if tool.description.len() > 65 {
                                    format!("{}...", &tool.description[..65])
                                } else {
                                    tool.description.clone()
                                };
                                println!("{:30} | {}", tool.name.cyan(), description);
                            }

                            println!("{}", "═".repeat(100));

                            // Show detailed schema for first tool as example
                            if let Some(first_tool) = tools.first() {
                                println!(
                                    "\n{} Example tool schema ({})",
                                    "→".cyan().bold(),
                                    first_tool.name.cyan()
                                );
                                println!("{}", "─".repeat(100));
                                let schema_json =
                                    serde_json::to_string_pretty(&first_tool.parameters)
                                        .unwrap_or_else(|_| {
                                            "Unable to serialize schema".to_string()
                                        });
                                println!("{}", schema_json.dimmed());
                                println!("{}", "─".repeat(100));
                            }
                        }

                        // Display summary
                        println!("\n{}", "═".repeat(100));
                        println!("{} Connection Test Summary", "📊".cyan().bold());
                        println!("{}", "═".repeat(100));
                        println!(
                            "  {} Connection:  {}",
                            "→".cyan(),
                            "Successful".green().bold()
                        );
                        println!(
                            "  {} Latency:     {:.2}ms",
                            "→".cyan(),
                            connection_time.as_secs_f64() * 1000.0
                        );
                        println!(
                            "  {} Discovery:   {:.2}ms",
                            "→".cyan(),
                            discovery_time.as_secs_f64() * 1000.0
                        );
                        println!(
                            "  {} Tools:       {}",
                            "→".cyan(),
                            tools.len().to_string().yellow().bold()
                        );
                        println!("{}", "═".repeat(100));

                        Ok(())
                    }
                    Err(e) => {
                        println!("{} Tool discovery failed: {}", "✗".red().bold(), e);

                        println!("\n{} Debugging Tips:", "💡".yellow().bold());
                        println!("  • Ensure the MCP server responds to 'tools/list' requests");
                        println!("  • Check server logs for protocol errors");
                        println!("  • Verify the server implements the MCP protocol correctly");

                        Err(CliError::ToolError {
                            message: format!("Tool discovery failed: {}", e),
                        })
                    }
                }
            }
            Err(e) => {
                let connection_time = start.elapsed();
                println!(
                    "{} Connection failed after {:.2}ms",
                    "✗".red().bold(),
                    connection_time.as_secs_f64() * 1000.0
                );
                println!("\n{} Error: {}", "→".red(), e);

                println!("\n{} Debugging Tips:", "💡".yellow().bold());
                println!("  • Verify the command exists and is in your PATH");
                println!("  • Check that the command accepts STDIO communication");
                println!("  • Ensure the command implements the MCP protocol");
                println!(
                    "  • Try running the command manually to test: {}",
                    stdio_command.yellow()
                );

                Err(CliError::McpConnectionError {
                    message: format!("Connection failed: {}", e),
                })
            }
        }
    }
    // Handle SSE server testing
    else if let Some(sse_endpoint) = args.mcp_sse {
        println!("{} Server type: {}", "→".cyan(), "SSE (HTTP)".bold());
        println!("{} Endpoint: {}", "→".cyan(), sse_endpoint.cyan());

        println!(
            "\n{} SSE server support not yet implemented",
            "⚠".yellow().bold()
        );
        println!("\n{} Debugging Tips:", "💡".yellow().bold());
        println!("  • SSE MCP servers will be supported in a future release");
        println!("  • Use --mcp-stdio for command-line MCP servers");

        Err(CliError::Other(
            "SSE server testing not yet implemented".to_string(),
        ))
    } else {
        unreachable!("Validation ensures at least one is Some")
    }
}
