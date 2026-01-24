//! SSE (Server-Sent Events) transport adapter for MCP
//!
//! This module provides a transport implementation that communicates with
//! MCP servers via HTTP/SSE. This is suitable for web-based MCP servers.
//!
//! # Example
//!
//! ```no_run
//! use paladin::infrastructure::adapters::arsenal::mcp_sse_adapter::MCPSseAdapter;
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let mut adapter = MCPSseAdapter::new("http://localhost:8080/mcp");
//! adapter.connect().await?;
//! // Use adapter with MCPClient
//! # Ok(())
//! # }
//! ```

use crate::core::platform::container::arsenal::ArsenalError;
use crate::infrastructure::adapters::arsenal::mcp_protocol::{MCPMessage, MCPTransport};
use async_trait::async_trait;
use reqwest::Client;
use serde_json;
use std::collections::VecDeque;
use std::time::Duration;
use tokio::sync::Mutex;
use tokio::time::sleep;

/// SSE transport adapter for MCP servers
///
/// Communicates with HTTP-based MCP servers using request/response pattern.
/// Supports retry logic with exponential backoff.
pub struct MCPSseAdapter {
    /// Server endpoint URL
    endpoint: String,
    /// HTTP client for making requests
    client: Client,
    /// Connection timeout duration
    timeout: Duration,
    /// Maximum number of retry attempts
    max_retries: u32,
    /// Response queue for buffered messages
    response_queue: Mutex<VecDeque<MCPMessage>>,
    /// Connection state
    connected: bool,
}

impl MCPSseAdapter {
    /// Creates a new SSE adapter with the given endpoint
    ///
    /// # Arguments
    ///
    /// * `endpoint` - Server endpoint URL (e.g., "http://localhost:8080/mcp")
    ///
    /// # Example
    ///
    /// ```
    /// use paladin::infrastructure::adapters::arsenal::mcp_sse_adapter::MCPSseAdapter;
    ///
    /// let adapter = MCPSseAdapter::new("http://localhost:8080/mcp");
    /// ```
    pub fn new(endpoint: impl Into<String>) -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(10))
            .build()
            .unwrap_or_else(|_| Client::new());

        Self {
            endpoint: endpoint.into(),
            client,
            timeout: Duration::from_secs(10),
            max_retries: 3,
            response_queue: Mutex::new(VecDeque::new()),
            connected: false,
        }
    }

    /// Creates a new SSE adapter with custom timeout
    ///
    /// # Arguments
    ///
    /// * `endpoint` - Server endpoint URL
    /// * `timeout` - Connection and request timeout
    ///
    /// # Example
    ///
    /// ```
    /// use paladin::infrastructure::adapters::arsenal::mcp_sse_adapter::MCPSseAdapter;
    /// use std::time::Duration;
    ///
    /// let adapter = MCPSseAdapter::with_timeout(
    ///     "http://localhost:8080/mcp",
    ///     Duration::from_secs(30)
    /// );
    /// ```
    pub fn with_timeout(endpoint: impl Into<String>, timeout: Duration) -> Self {
        let client = Client::builder()
            .timeout(timeout)
            .build()
            .unwrap_or_else(|_| Client::new());

        Self {
            endpoint: endpoint.into(),
            client,
            timeout,
            max_retries: 3,
            response_queue: Mutex::new(VecDeque::new()),
            connected: false,
        }
    }

    /// Connects to the MCP server
    ///
    /// Performs a health check to verify the server is reachable.
    ///
    /// # Errors
    ///
    /// Returns `ArsenalError::TransportError` if:
    /// - Server is not reachable
    /// - Connection times out
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use paladin::infrastructure::adapters::arsenal::mcp_sse_adapter::MCPSseAdapter;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let mut adapter = MCPSseAdapter::new("http://localhost:8080/mcp");
    /// adapter.connect().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn connect(&mut self) -> Result<(), ArsenalError> {
        // Perform health check
        let response = self
            .client
            .get(&self.endpoint)
            .timeout(self.timeout)
            .send()
            .await
            .map_err(|e| {
                ArsenalError::TransportError(format!("Failed to connect to server: {}", e))
            })?;

        if !response.status().is_success() {
            return Err(ArsenalError::TransportError(format!(
                "Server returned error status: {}",
                response.status()
            )));
        }

        self.connected = true;
        Ok(())
    }

    /// Checks if the adapter is connected
    pub fn is_connected(&self) -> bool {
        self.connected
    }

    /// Sends a request with retry logic
    ///
    /// Implements exponential backoff: 1s, 2s, 4s between retries.
    async fn send_with_retry(&self, message: &MCPMessage) -> Result<MCPMessage, ArsenalError> {
        let mut last_error = None;

        for attempt in 0..=self.max_retries {
            if attempt > 0 {
                // Exponential backoff: 2^(attempt-1) seconds
                let delay = Duration::from_secs(2u64.pow(attempt - 1));
                sleep(delay).await;
            }

            match self.send_request(message).await {
                Ok(response) => return Ok(response),
                Err(e) => {
                    last_error = Some(e);
                    if attempt < self.max_retries {
                        continue;
                    }
                }
            }
        }

        Err(last_error.unwrap_or_else(|| {
            ArsenalError::TransportError("All retry attempts failed".to_string())
        }))
    }

    /// Sends a single request without retry
    async fn send_request(&self, message: &MCPMessage) -> Result<MCPMessage, ArsenalError> {
        let json = serde_json::to_string(message).map_err(|e| {
            ArsenalError::TransportError(format!("Failed to serialize message: {}", e))
        })?;

        let response = self
            .client
            .post(&self.endpoint)
            .header("Content-Type", "application/json")
            .body(json)
            .timeout(self.timeout)
            .send()
            .await
            .map_err(|e| ArsenalError::TransportError(format!("HTTP request failed: {}", e)))?;

        if !response.status().is_success() {
            return Err(ArsenalError::TransportError(format!(
                "Server returned error status: {}",
                response.status()
            )));
        }

        let response_text = response
            .text()
            .await
            .map_err(|e| ArsenalError::TransportError(format!("Failed to read response: {}", e)))?;

        let response_msg: MCPMessage = serde_json::from_str(&response_text).map_err(|e| {
            ArsenalError::TransportError(format!("Failed to deserialize response: {}", e))
        })?;

        Ok(response_msg)
    }
}

#[async_trait]
impl MCPTransport for MCPSseAdapter {
    /// Sends a message to the MCP server via HTTP POST
    ///
    /// Implements retry logic with exponential backoff (1s, 2s, 4s).
    ///
    /// # Errors
    ///
    /// Returns `ArsenalError::TransportError` if:
    /// - Not connected (call connect() first)
    /// - HTTP request fails after all retries
    /// - Server returns error status
    /// - Response deserialization fails
    async fn send(&mut self, message: &MCPMessage) -> Result<(), ArsenalError> {
        if !self.connected {
            return Err(ArsenalError::TransportError(
                "Not connected - call connect() first".to_string(),
            ));
        }

        // Send with retry and buffer the response
        let response = self.send_with_retry(message).await?;
        self.response_queue.lock().await.push_back(response);

        Ok(())
    }

    /// Receives a message from the response queue
    ///
    /// Messages are buffered from previous send() calls.
    ///
    /// # Errors
    ///
    /// Returns `ArsenalError::TransportError` if:
    /// - Not connected (call connect() first)
    /// - No messages in queue
    async fn receive(&mut self) -> Result<MCPMessage, ArsenalError> {
        if !self.connected {
            return Err(ArsenalError::TransportError(
                "Not connected - call connect() first".to_string(),
            ));
        }

        self.response_queue
            .lock()
            .await
            .pop_front()
            .ok_or_else(|| ArsenalError::TransportError("No messages in queue".to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_adapter_creation() {
        let adapter = MCPSseAdapter::new("http://localhost:8080");
        assert_eq!(adapter.endpoint, "http://localhost:8080");
        assert!(!adapter.is_connected());
    }

    #[test]
    fn test_adapter_with_timeout() {
        let adapter = MCPSseAdapter::with_timeout("http://localhost:8080", Duration::from_secs(30));
        assert_eq!(adapter.timeout, Duration::from_secs(30));
    }

    #[test]
    fn test_adapter_not_connected() {
        let adapter = MCPSseAdapter::new("http://localhost:8080");
        assert!(!adapter.is_connected());
    }
}
