//! Configuration for the Arsenal tool system (MCP servers).

use serde::{Deserialize, Serialize};

/// Configuration for a single MCP server
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MCPServerConfig {
    /// Name/identifier for the server
    pub name: String,
    /// Type of server: "stdio" | "streamable_http" (replaces "sse"; "sse" is
    /// deprecated and fails loud with a migration message rather than
    /// silently misconnecting — Phase 12.1 D-02b).
    pub server_type: String,
    /// Command to execute (for STDIO servers)
    pub command: Option<String>,
    /// Arguments for the command (for STDIO servers)
    pub args: Option<Vec<String>>,
    /// HTTP endpoint URL (for Streamable-HTTP servers)
    pub endpoint: Option<String>,
    /// Name of the environment variable holding the bearer/auth token for a
    /// `streamable_http` server (D-03). This is an env-var REFERENCE (e.g.
    /// `"ETHERSCAN_API_KEY"`), NEVER a literal secret — the token itself is
    /// resolved host-side from the named env var at connect time and is
    /// never written back to `.mcp.json`/config.yml.
    #[serde(default, skip_serializing)]
    pub auth_token_env: Option<String>,
}

/// Configuration for Arsenal tool system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArsenalConfig {
    /// Default timeout for tool execution in seconds
    pub default_timeout_seconds: u64,
    /// Maximum number of concurrent tool executions
    pub max_concurrent_tools: usize,
    /// List of MCP servers to connect to
    pub mcp_servers: Vec<MCPServerConfig>,
}

impl Default for ArsenalConfig {
    fn default() -> Self {
        Self {
            default_timeout_seconds: 30,
            max_concurrent_tools: 5,
            mcp_servers: Vec::new(),
        }
    }
}
