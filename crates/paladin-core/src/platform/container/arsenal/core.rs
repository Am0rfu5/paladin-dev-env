//! Arsenal domain entities for the tool system.
//!
//! This module defines the core domain types for the Arsenal tool system,
//! which enables Paladins to interact with external tools through the
//! Model Context Protocol (MCP).

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use uuid::Uuid;

/// Definition of a tool (weapon) in the Arsenal.
///
/// An `Armament` represents a single tool that can be invoked by a Paladin.
/// It includes metadata about the tool's purpose, parameters, and requirements.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Armament {
    /// Unique name identifying this tool
    pub name: String,

    /// Human-readable description of what this tool does
    pub description: String,

    /// JSON Schema defining the structure of parameters this tool accepts
    pub parameters: Value,

    /// List of parameter names that are required for this tool
    pub required_params: Vec<String>,
}

/// A request to invoke a specific tool.
///
/// An `ArmamentCall` represents a single invocation of a tool with
/// specific arguments. Each call is uniquely identified for tracking purposes.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ArmamentCall {
    /// Name of the tool to invoke
    pub tool_name: String,

    /// Arguments to pass to the tool (parameter name -> value)
    pub arguments: HashMap<String, Value>,

    /// Unique identifier for this tool invocation
    pub call_id: Uuid,
}

impl ArmamentCall {
    /// Creates a new tool invocation with a generated UUID.
    ///
    /// # Arguments
    ///
    /// * `tool_name` - Name of the tool to invoke
    /// * `arguments` - Map of parameter names to values
    ///
    /// # Examples
    ///
    /// ```
    /// use paladin_core::platform::container::arsenal::ArmamentCall;
    /// use std::collections::HashMap;
    /// use serde_json::json;
    ///
    /// let mut args = HashMap::new();
    /// args.insert("query".to_string(), json!("search term"));
    /// let call = ArmamentCall::new("web_search", args);
    /// ```
    pub fn new(tool_name: impl Into<String>, arguments: HashMap<String, Value>) -> Self {
        Self {
            tool_name: tool_name.into(),
            arguments,
            call_id: Uuid::new_v4(),
        }
    }
}

/// Result of a tool invocation.
///
/// An `ArmamentResult` captures the outcome of executing a tool,
/// including success/failure status, output data, and execution metrics.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ArmamentResult {
    /// Call ID matching the original ArmamentCall
    pub call_id: Uuid,

    /// Whether the tool execution succeeded
    pub success: bool,

    /// Output data from the tool (if successful)
    pub output: Option<Value>,

    /// Error message (if failed)
    pub error: Option<String>,

    /// Execution time in milliseconds
    pub execution_time_ms: u64,
}

impl ArmamentResult {
    /// Creates a successful result.
    ///
    /// # Arguments
    ///
    /// * `call_id` - The UUID from the original ArmamentCall
    /// * `output` - The tool's output data
    /// * `execution_time_ms` - How long the tool took to execute
    pub fn success(call_id: Uuid, output: Value, execution_time_ms: u64) -> Self {
        Self {
            call_id,
            success: true,
            output: Some(output),
            error: None,
            execution_time_ms,
        }
    }

    /// Creates a failed result.
    ///
    /// # Arguments
    ///
    /// * `call_id` - The UUID from the original ArmamentCall
    /// * `error` - Description of what went wrong
    /// * `execution_time_ms` - How long the tool ran before failing
    pub fn failure(call_id: Uuid, error: impl Into<String>, execution_time_ms: u64) -> Self {
        Self {
            call_id,
            success: false,
            output: None,
            error: Some(error.into()),
            execution_time_ms,
        }
    }
}

/// Errors that can occur in the Arsenal tool system.
///
/// These errors cover tool discovery, invocation, and communication
/// with external tool providers via MCP (Model Context Protocol).
#[derive(Debug, thiserror::Error)]
pub enum ArsenalError {
    /// Tool not found in the registry
    #[error("Tool not found: {0}")]
    ToolNotFound(String),

    /// Invalid arguments provided for tool invocation
    #[error("Invalid tool arguments: {0}")]
    InvalidArguments(String),

    /// Tool execution exceeded timeout
    #[error("Tool execution timeout after {0} seconds")]
    Timeout(u64),

    /// MCP protocol error
    #[error("MCP protocol error: {0}")]
    ProtocolError(String),

    /// Transport layer error
    #[error("Transport error: {0}")]
    TransportError(String),
}
