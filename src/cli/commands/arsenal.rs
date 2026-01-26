//! Arsenal command implementations

use clap::Subcommand;

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
