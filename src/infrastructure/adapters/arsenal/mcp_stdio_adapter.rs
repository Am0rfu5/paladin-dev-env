//! STDIO transport adapter for MCP
//!
//! This module provides a transport implementation that communicates with
//! MCP servers via subprocess stdin/stdout. This is the most common MCP
//! transport mechanism for command-line tools.
//!
//! # Example
//!
//! ```no_run
//! use paladin::infrastructure::adapters::arsenal::mcp_stdio_adapter::MCPStdioAdapter;
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let mut adapter = MCPStdioAdapter::new("python3", vec!["-u", "server.py"]);
//! adapter.connect().await?;
//! // Use adapter with MCPClient
//! # Ok(())
//! # }
//! ```

use crate::core::platform::container::arsenal::ArsenalError;
use crate::infrastructure::adapters::arsenal::mcp_protocol::{MCPMessage, MCPTransport};
use async_trait::async_trait;
use serde_json;
use std::process::Stdio;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::process::{Child, ChildStdin, ChildStdout, Command};
use tokio::sync::Mutex;

/// STDIO transport adapter for MCP servers
///
/// Spawns a subprocess and communicates via stdin/stdout using
/// newline-delimited JSON messages.
pub struct MCPStdioAdapter {
    /// Command to execute (e.g., "python3", "node", "uvx")
    command: String,
    /// Command arguments (e.g., ["-u", "server.py"])
    args: Vec<String>,
    /// Running child process (Some after connect())
    process: Option<Child>,
    /// Stdin handle for writing messages
    stdin: Option<Mutex<ChildStdin>>,
    /// Stdout handle for reading messages
    stdout: Option<Mutex<BufReader<ChildStdout>>>,
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
            process: None,
            stdin: None,
            stdout: None,
        }
    }

    /// Connects to the MCP server by spawning the subprocess
    ///
    /// Spawns the command with stdin/stdout/stderr piped and stores
    /// the handles for communication.
    ///
    /// # Errors
    ///
    /// Returns `ArsenalError::TransportError` if:
    /// - Process spawn fails
    /// - Cannot capture stdin/stdout handles
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use paladin::infrastructure::adapters::arsenal::mcp_stdio_adapter::MCPStdioAdapter;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let mut adapter = MCPStdioAdapter::new("python3", vec!["-u", "server.py"]);
    /// adapter.connect().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn connect(&mut self) -> Result<(), ArsenalError> {
        // Spawn the process with piped stdin/stdout/stderr
        let mut child = Command::new(&self.command)
            .args(&self.args)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .kill_on_drop(true)
            .spawn()
            .map_err(|e| ArsenalError::TransportError(format!("Failed to spawn process: {}", e)))?;

        // Take ownership of stdin and stdout
        let stdin = child
            .stdin
            .take()
            .ok_or_else(|| ArsenalError::TransportError("Failed to capture stdin".to_string()))?;

        let stdout = child
            .stdout
            .take()
            .ok_or_else(|| ArsenalError::TransportError("Failed to capture stdout".to_string()))?;

        // Store handles
        self.stdin = Some(Mutex::new(stdin));
        self.stdout = Some(Mutex::new(BufReader::new(stdout)));
        self.process = Some(child);

        Ok(())
    }

    /// Checks if the adapter is connected
    pub fn is_connected(&self) -> bool {
        self.process.is_some()
    }
}

#[async_trait]
impl MCPTransport for MCPStdioAdapter {
    /// Sends a message to the MCP server via stdin
    ///
    /// Serializes the message to JSON and writes it as a single line
    /// to the process stdin.
    ///
    /// # Errors
    ///
    /// Returns `ArsenalError::TransportError` if:
    /// - Not connected (call connect() first)
    /// - JSON serialization fails
    /// - Write to stdin fails
    async fn send(&mut self, message: &MCPMessage) -> Result<(), ArsenalError> {
        let stdin = self.stdin.as_ref().ok_or_else(|| {
            ArsenalError::TransportError("Not connected - call connect() first".to_string())
        })?;

        // Serialize message to JSON
        let json = serde_json::to_string(message).map_err(|e| {
            ArsenalError::TransportError(format!("Failed to serialize message: {}", e))
        })?;

        // Write JSON line to stdin
        let mut stdin_lock = stdin.lock().await;
        stdin_lock.write_all(json.as_bytes()).await.map_err(|e| {
            ArsenalError::TransportError(format!("Failed to write to stdin: {}", e))
        })?;

        stdin_lock
            .write_all(b"\n")
            .await
            .map_err(|e| ArsenalError::TransportError(format!("Failed to write newline: {}", e)))?;

        stdin_lock
            .flush()
            .await
            .map_err(|e| ArsenalError::TransportError(format!("Failed to flush stdin: {}", e)))?;

        Ok(())
    }

    /// Receives a message from the MCP server via stdout
    ///
    /// Reads a line from stdout and deserializes it as JSON.
    ///
    /// # Errors
    ///
    /// Returns `ArsenalError::TransportError` if:
    /// - Not connected (call connect() first)
    /// - Read from stdout fails
    /// - JSON deserialization fails
    async fn receive(&mut self) -> Result<MCPMessage, ArsenalError> {
        let stdout = self.stdout.as_ref().ok_or_else(|| {
            ArsenalError::TransportError("Not connected - call connect() first".to_string())
        })?;

        // Read line from stdout
        let mut stdout_lock = stdout.lock().await;
        let mut line = String::new();

        stdout_lock.read_line(&mut line).await.map_err(|e| {
            ArsenalError::TransportError(format!("Failed to read from stdout: {}", e))
        })?;

        if line.is_empty() {
            return Err(ArsenalError::TransportError(
                "EOF reached - process terminated".to_string(),
            ));
        }

        // Deserialize JSON
        let message: MCPMessage = serde_json::from_str(&line).map_err(|e| {
            ArsenalError::TransportError(format!("Failed to deserialize message: {}", e))
        })?;

        Ok(message)
    }
}

impl Drop for MCPStdioAdapter {
    /// Ensures the subprocess is terminated when the adapter is dropped
    fn drop(&mut self) {
        if let Some(mut process) = self.process.take() {
            // Try to kill the process gracefully
            let _ = process.start_kill();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_adapter_creation() {
        let adapter = MCPStdioAdapter::new("echo", vec!["test"]);
        assert_eq!(adapter.command, "echo");
        assert_eq!(adapter.args, vec!["test"]);
        assert!(!adapter.is_connected());
    }

    #[test]
    fn test_adapter_not_connected() {
        let adapter = MCPStdioAdapter::new("echo", vec!["test"]);
        assert!(!adapter.is_connected());
    }
}
