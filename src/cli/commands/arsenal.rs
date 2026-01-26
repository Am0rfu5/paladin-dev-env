//! Arsenal command implementations

use crate::cli::output::errors::CliError;
use clap::Subcommand;
use colored::Colorize;

#[derive(Debug, Subcommand)]
pub enum ArsenalCommands {
    /// List available MCP tools
    List,
    /// Test an MCP server connection
    Test(ArsenalTestArgs),
}

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

/// Test an MCP server connection (stub for Task 9.0)
async fn handle_arsenal_test(_args: ArsenalTestArgs) -> Result<(), CliError> {
    Err(CliError::Other(
        "arsenal test command not yet implemented (Task 9.0)".to_string(),
    ))
}
