//! Model Context Protocol (MCP) implementation
//!
//! This module implements the JSON-RPC 2.0 based MCP protocol for communicating
//! with tool servers. It provides message types, a transport abstraction, and
//! a client for tool discovery and invocation.
//!
//! # Protocol Overview
//!
//! MCP uses JSON-RPC 2.0 for message exchange:
//! - **Request**: Client sends method call (e.g., `tools/list`, `tools/call`)
//! - **Response**: Server returns result or error
//! - **Notification**: One-way messages (not used in current implementation)
//!
//! # Example
//!
//! ```no_run
//! use paladin::infrastructure::adapters::arsenal::mcp_protocol::{MCPClient, MCPRequest};
//! # use std::sync::Arc;
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! # let transport: Arc<dyn paladin::infrastructure::adapters::arsenal::mcp_protocol::MCPTransport> = todo!();
//! let client = MCPClient::new(transport);
//! let tools = client.discover_tools().await?;
//! println!("Available tools: {:?}", tools);
//! # Ok(())
//! # }
//! ```

use crate::core::platform::container::arsenal::{Armament, ArsenalError};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::sync::Arc;
use uuid::Uuid;

/// JSON-RPC 2.0 message types
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MCPMessage {
    /// Request message from client to server
    Request(MCPRequest),
    /// Response message from server to client
    Response(MCPResponse),
    /// Notification message (one-way, no response expected)
    Notification(MCPNotification),
}

/// JSON-RPC 2.0 request message
///
/// Represents a client request to invoke a method on the server.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MCPRequest {
    /// JSON-RPC protocol version (always "2.0")
    pub jsonrpc: String,
    /// Unique identifier for this request
    pub id: Value,
    /// Method name to invoke (e.g., "tools/list", "tools/call")
    pub method: String,
    /// Optional parameters for the method
    #[serde(skip_serializing_if = "Option::is_none")]
    pub params: Option<Value>,
}

impl MCPRequest {
    /// Creates a new MCP request with a UUID identifier
    ///
    /// # Arguments
    ///
    /// * `method` - The method name to invoke
    /// * `params` - Optional parameters for the method
    ///
    /// # Example
    ///
    /// ```
    /// use paladin::infrastructure::adapters::arsenal::mcp_protocol::MCPRequest;
    /// use serde_json::json;
    ///
    /// let request = MCPRequest::new("tools/list", Some(json!({})));
    /// assert_eq!(request.method, "tools/list");
    /// ```
    pub fn new(method: impl Into<String>, params: Option<Value>) -> Self {
        Self {
            jsonrpc: "2.0".to_string(),
            id: Value::String(Uuid::new_v4().to_string()),
            method: method.into(),
            params,
        }
    }
}

/// JSON-RPC 2.0 response message
///
/// Represents a server response to a client request.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MCPResponse {
    /// JSON-RPC protocol version (always "2.0")
    pub jsonrpc: String,
    /// Request ID this response corresponds to
    pub id: Value,
    /// Success result (mutually exclusive with error)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<Value>,
    /// Error result (mutually exclusive with result)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<MCPError>,
}

/// JSON-RPC 2.0 error object
///
/// Represents an error that occurred during request processing.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MCPError {
    /// Error code (standard JSON-RPC codes or application-specific)
    pub code: i64,
    /// Human-readable error message
    pub message: String,
    /// Optional additional error data
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Value>,
}

impl MCPError {
    /// Standard JSON-RPC error codes
    pub const PARSE_ERROR: i64 = -32700;
    pub const INVALID_REQUEST: i64 = -32600;
    pub const METHOD_NOT_FOUND: i64 = -32601;
    pub const INVALID_PARAMS: i64 = -32602;
    pub const INTERNAL_ERROR: i64 = -32603;

    /// Creates a new MCP error
    pub fn new(code: i64, message: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
            data: None,
        }
    }

    /// Creates a new MCP error with additional data
    pub fn with_data(code: i64, message: impl Into<String>, data: Value) -> Self {
        Self {
            code,
            message: message.into(),
            data: Some(data),
        }
    }
}

/// JSON-RPC 2.0 notification message
///
/// One-way message that doesn't expect a response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MCPNotification {
    /// JSON-RPC protocol version (always "2.0")
    pub jsonrpc: String,
    /// Method name
    pub method: String,
    /// Optional parameters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub params: Option<Value>,
}

/// MCP server capabilities
///
/// Describes what features and tools the server supports.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MCPCapabilities {
    /// Server name and version
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_info: Option<ServerInfo>,
    /// List of supported tools
    #[serde(default)]
    pub tools: Vec<ToolInfo>,
    /// Additional capability flags
    #[serde(flatten)]
    pub extensions: HashMap<String, Value>,
}

/// Server information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerInfo {
    /// Server name
    pub name: String,
    /// Server version
    pub version: String,
}

/// Tool information from MCP server
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolInfo {
    /// Tool name
    pub name: String,
    /// Tool description
    pub description: String,
    /// Input schema (JSON Schema)
    #[serde(rename = "inputSchema")]
    pub input_schema: Value,
}

/// Transport abstraction for MCP communication
///
/// Implementations handle the actual communication mechanism (STDIO, SSE, etc.)
#[async_trait]
pub trait MCPTransport: Send + Sync {
    /// Sends a message to the server
    ///
    /// # Errors
    ///
    /// Returns `ArsenalError::TransportError` if sending fails
    async fn send(&mut self, message: &MCPMessage) -> Result<(), ArsenalError>;

    /// Receives a message from the server
    ///
    /// # Errors
    ///
    /// Returns `ArsenalError::TransportError` if receiving fails
    async fn receive(&mut self) -> Result<MCPMessage, ArsenalError>;
}

/// MCP client for interacting with tool servers
///
/// Provides high-level methods for tool discovery and invocation.
pub struct MCPClient {
    /// Transport implementation (STDIO, SSE, etc.)
    transport: Arc<tokio::sync::Mutex<Box<dyn MCPTransport>>>,
    /// Server capabilities (populated after connection)
    #[allow(dead_code)]
    capabilities: Option<MCPCapabilities>,
}

impl MCPClient {
    /// Creates a new MCP client with the given transport
    ///
    /// # Arguments
    ///
    /// * `transport` - Transport implementation for communication
    ///
    /// # Example
    ///
    /// ```no_run
    /// use paladin::infrastructure::adapters::arsenal::mcp_protocol::MCPClient;
    /// use std::sync::Arc;
    /// # async fn example() {
    /// # let transport: Box<dyn paladin::infrastructure::adapters::arsenal::mcp_protocol::MCPTransport> = todo!();
    /// let client = MCPClient::new(transport);
    /// # }
    /// ```
    pub fn new(transport: Box<dyn MCPTransport>) -> Self {
        Self {
            transport: Arc::new(tokio::sync::Mutex::new(transport)),
            capabilities: None,
        }
    }

    /// Discovers available tools from the server
    ///
    /// Sends a `tools/list` request and parses the response.
    ///
    /// # Errors
    ///
    /// Returns `ArsenalError` if:
    /// - Communication fails
    /// - Server returns an error
    /// - Response format is invalid
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use paladin::infrastructure::adapters::arsenal::mcp_protocol::MCPClient;
    /// # async fn example(client: MCPClient) -> Result<(), Box<dyn std::error::Error>> {
    /// let tools = client.discover_tools().await?;
    /// for tool in tools {
    ///     println!("Found tool: {}", tool.name);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn discover_tools(&self) -> Result<Vec<Armament>, ArsenalError> {
        let request = MCPRequest::new("tools/list", Some(serde_json::json!({})));
        let response = self.send_request(request).await?;

        // Parse tools from response
        let tools_array = response
            .get("tools")
            .and_then(|v| v.as_array())
            .ok_or_else(|| {
                ArsenalError::ProtocolError("Invalid tools/list response format".to_string())
            })?;

        let mut armaments = Vec::new();
        for tool_value in tools_array {
            let tool_info: ToolInfo = serde_json::from_value(tool_value.clone()).map_err(|e| {
                ArsenalError::ProtocolError(format!("Failed to parse tool info: {}", e))
            })?;

            // Extract required parameters from schema
            let required_params = tool_info
                .input_schema
                .get("required")
                .and_then(|r| r.as_array())
                .map(|arr| {
                    arr.iter()
                        .filter_map(|v| v.as_str().map(String::from))
                        .collect()
                })
                .unwrap_or_default();

            armaments.push(Armament {
                name: tool_info.name,
                description: tool_info.description,
                parameters: tool_info.input_schema,
                required_params,
            });
        }

        Ok(armaments)
    }

    /// Invokes a tool on the server
    ///
    /// Sends a `tools/call` request with the specified tool name and arguments.
    ///
    /// # Arguments
    ///
    /// * `tool_name` - Name of the tool to invoke
    /// * `arguments` - Tool arguments as a HashMap
    ///
    /// # Errors
    ///
    /// Returns `ArsenalError` if:
    /// - Communication fails
    /// - Server returns an error
    /// - Tool execution fails
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use paladin::infrastructure::adapters::arsenal::mcp_protocol::MCPClient;
    /// # use std::collections::HashMap;
    /// # use serde_json::Value;
    /// # async fn example(client: MCPClient) -> Result<(), Box<dyn std::error::Error>> {
    /// let mut args = HashMap::new();
    /// args.insert("query".to_string(), Value::String("Rust".to_string()));
    /// let result = client.invoke_tool("web_search", args).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn invoke_tool(
        &self,
        tool_name: &str,
        arguments: HashMap<String, Value>,
    ) -> Result<Value, ArsenalError> {
        let params = serde_json::json!({
            "name": tool_name,
            "arguments": arguments,
        });

        let request = MCPRequest::new("tools/call", Some(params));
        let response = self.send_request(request).await?;

        // Extract result from response
        response
            .get("content")
            .cloned()
            .ok_or_else(|| ArsenalError::ProtocolError("Missing content in response".to_string()))
    }

    /// Sends a request and waits for response
    ///
    /// Internal helper method for request/response pattern.
    async fn send_request(&self, request: MCPRequest) -> Result<Value, ArsenalError> {
        let mut transport = self.transport.lock().await;

        // Send request
        transport.send(&MCPMessage::Request(request)).await?;

        // Receive response
        let response_msg = transport.receive().await?;

        match response_msg {
            MCPMessage::Response(response) => {
                if let Some(error) = response.error {
                    return Err(ArsenalError::ProtocolError(format!(
                        "MCP error {}: {}",
                        error.code, error.message
                    )));
                }

                response.result.ok_or_else(|| {
                    ArsenalError::ProtocolError(
                        "Response missing both result and error".to_string(),
                    )
                })
            }
            _ => Err(ArsenalError::ProtocolError(
                "Expected response, got different message type".to_string(),
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mcp_request_creation() {
        let request = MCPRequest::new("test/method", Some(serde_json::json!({"key": "value"})));
        assert_eq!(request.jsonrpc, "2.0");
        assert_eq!(request.method, "test/method");
        assert!(request.params.is_some());
    }

    #[test]
    fn test_mcp_error_codes() {
        assert_eq!(MCPError::PARSE_ERROR, -32700);
        assert_eq!(MCPError::INVALID_REQUEST, -32600);
        assert_eq!(MCPError::METHOD_NOT_FOUND, -32601);
    }
}
