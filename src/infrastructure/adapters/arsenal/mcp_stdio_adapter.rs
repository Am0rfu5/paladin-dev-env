//! STDIO transport builder for MCP (rmcp-backed)
//!
//! This module used to own the raw subprocess stdin/stdout plumbing and
//! implement the (now-retired) hand-rolled `MCPTransport` trait. Since
//! `rmcp::transport::TokioChildProcess` already owns spawning, framing, and
//! kill-on-drop process cleanup (verified: `ChildWithCleanup::drop` kills the
//! child process automatically — no manual `Drop` impl is needed here per
//! RESEARCH's Don't Hand-Roll guidance), `MCPStdioAdapter` is now a thin
//! builder that stores the command/args and delegates the actual connect
//! (spawn + MCP handshake) to [`MCPClient::connect_stdio`].
//!
//! # Example
//!
//! ```no_run
//! use paladin::infrastructure::adapters::arsenal::mcp_stdio_adapter::MCPStdioAdapter;
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let adapter = MCPStdioAdapter::new("python3", vec!["-u", "server.py"]);
//! let client = adapter.connect().await?;
//! let _ = client.discover_tools().await?;
//! # Ok(())
//! # }
//! ```

use crate::core::platform::container::arsenal::ArsenalError;
use crate::infrastructure::adapters::arsenal::mcp_protocol::MCPClient;

/// STDIO transport builder for MCP servers.
///
/// Stores the command and arguments used to spawn the server subprocess;
/// [`connect`](Self::connect) performs the actual spawn + MCP handshake via
/// [`MCPClient::connect_stdio`].
#[doc(hidden)]
pub struct MCPStdioAdapter {
    /// Command to execute (e.g., "python3", "node", "uvx")
    command: String,
    /// Command arguments (e.g., ["-u", "server.py"])
    args: Vec<String>,
}

impl MCPStdioAdapter {
    /// Creates a new STDIO adapter with the given command and arguments
    ///
    /// # Arguments
    ///
    /// * `command` - Command to execute (e.g., "python3", "node")
    /// * `args` - Command arguments (e.g., vec!["-u", "server.py"])
    ///
    /// # Example
    ///
    /// ```
    /// use paladin::infrastructure::adapters::arsenal::mcp_stdio_adapter::MCPStdioAdapter;
    ///
    /// let adapter = MCPStdioAdapter::new("python3", vec!["-u", "server.py"]);
    /// ```
    pub fn new(command: impl Into<String>, args: Vec<impl Into<String>>) -> Self {
        Self {
            command: command.into(),
            args: args.into_iter().map(|a| a.into()).collect(),
        }
    }

    /// The configured command.
    pub fn command(&self) -> &str {
        &self.command
    }

    /// The configured command-line arguments.
    pub fn args(&self) -> &[String] {
        &self.args
    }

    /// Spawns the configured command and performs the full MCP stdio
    /// handshake, returning a ready-to-use [`MCPClient`].
    ///
    /// # Errors
    ///
    /// Returns `ArsenalError::TransportError` if the subprocess fails to
    /// spawn, or `ArsenalError::ProtocolError` if the MCP handshake fails.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use paladin::infrastructure::adapters::arsenal::mcp_stdio_adapter::MCPStdioAdapter;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let adapter = MCPStdioAdapter::new("python3", vec!["-u", "server.py"]);
    /// let client = adapter.connect().await?;
    /// # let _ = client;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn connect(&self) -> Result<MCPClient, ArsenalError> {
        MCPClient::connect_stdio(&self.command, &self.args).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_adapter_creation() {
        let adapter = MCPStdioAdapter::new("echo", vec!["test"]);
        assert_eq!(adapter.command(), "echo");
        assert_eq!(adapter.args(), &["test".to_string()]);
    }
}
