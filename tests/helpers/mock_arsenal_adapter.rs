//! Mock Arsenal adapter for testing tool integration
//!
//! Provides a configurable mock implementation of `ArsenalPort` that returns
//! pre-configured responses for tool invocations, tracks invocation history,
//! and supports error simulation. Designed for use in tests that verify the
//! full Paladin → LLM → Arsenal tool call loop without external dependencies.
//!
//! # Examples
//!
//! ```rust,ignore
//! use std::sync::Arc;
//! let mut mock = MockArsenalPort::new();
//! mock.add_tool("calculator", "A basic calculator");
//! let mock = Arc::new(mock);
//!
//! // Pre-configure a success response
//! mock.set_response("calculator", ArmamentResult::success(
//!     uuid::Uuid::new_v4(),
//!     serde_json::json!("42"),
//!     100,
//! ));
//!
//! // After test execution, verify invocations
//! assert_eq!(mock.call_count(), 1);
//! ```

use async_trait::async_trait;
use paladin::core::platform::container::arsenal::{
    Armament, ArmamentCall, ArmamentResult, ArsenalError,
};
use paladin_ports::output::arsenal_port::ArsenalPort;
use serde_json::Value;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// Mock Arsenal port for testing tool call flows
///
/// Supports configurable tool definitions, pre-set responses (success or error)
/// per tool name, and invocation recording for test assertions.
pub struct MockArsenalPort {
    /// Registered tools (returned by `list_armaments`)
    armaments: Vec<Armament>,

    /// Pre-configured responses keyed by tool name.
    /// If a tool name is not present, `invoke()` returns `ArsenalError::ToolNotFound`.
    responses: Arc<Mutex<HashMap<String, Result<ArmamentResult, ArsenalError>>>>,

    /// Record of all `invoke()` calls for test assertions
    invocations: Arc<Mutex<Vec<ArmamentCall>>>,
}

impl MockArsenalPort {
    /// Create a new empty MockArsenalPort with no tools or responses
    pub fn new() -> Self {
        Self {
            armaments: Vec::new(),
            responses: Arc::new(Mutex::new(HashMap::new())),
            invocations: Arc::new(Mutex::new(Vec::new())),
        }
    }

    /// Add a tool definition to the mock arsenal
    ///
    /// The tool will appear in `list_armaments()` results and pass `validate_call()`.
    /// You still need to call `set_response()` or `set_error()` to configure
    /// what `invoke()` returns for this tool.
    ///
    /// # Arguments
    ///
    /// * `name` - Tool name (must match the `tool_name` in `ArmamentCall`)
    /// * `description` - Human-readable description
    pub fn add_tool(&mut self, name: &str, description: &str) -> &mut Self {
        self.armaments.push(Armament {
            name: name.to_string(),
            description: description.to_string(),
            parameters: Value::Object(serde_json::Map::new()),
            required_params: vec![],
        });
        self
    }

    /// Pre-configure a successful response for a tool name
    ///
    /// When `invoke()` is called with a matching `tool_name`, this result is returned.
    /// The response is consumed on use and replaced with `ToolNotFound` for subsequent calls
    /// unless set again — **except** this implementation clones the response so it can be
    /// reused across multiple invocations.
    ///
    /// # Arguments
    ///
    /// * `tool_name` - The tool name to match
    /// * `result` - The `ArmamentResult` to return
    pub fn set_response(&self, tool_name: &str, result: ArmamentResult) {
        self.responses
            .lock()
            .unwrap()
            .insert(tool_name.to_string(), Ok(result));
    }

    /// Pre-configure an error response for a tool name
    ///
    /// When `invoke()` is called with a matching `tool_name`, this error is returned.
    ///
    /// # Arguments
    ///
    /// * `tool_name` - The tool name to match
    /// * `error_msg` - Error message string (wrapped in `ArsenalError::ToolNotFound` variant
    ///   matching the error type you want — use `set_error_variant` for specific variants)
    pub fn set_error(&self, tool_name: &str, error_msg: &str) {
        self.responses.lock().unwrap().insert(
            tool_name.to_string(),
            Err(ArsenalError::ToolNotFound(error_msg.to_string())),
        );
    }

    /// Pre-configure a specific `ArsenalError` variant for a tool name
    ///
    /// # Arguments
    ///
    /// * `tool_name` - The tool name to match
    /// * `error` - The specific `ArsenalError` to return
    pub fn set_error_variant(&self, tool_name: &str, error: ArsenalError) {
        // ArsenalError doesn't impl Clone, so we store a string representation
        // and reconstruct. For testing, we store the variant info.
        let error_string = format!("{}", error);
        self.responses.lock().unwrap().insert(
            tool_name.to_string(),
            Err(Self::reconstruct_error(&error, &error_string)),
        );
    }

    /// Get the total number of `invoke()` calls
    pub fn call_count(&self) -> usize {
        self.invocations.lock().unwrap().len()
    }

    /// Get all recorded invocations
    pub fn invocations(&self) -> Vec<ArmamentCall> {
        self.invocations.lock().unwrap().clone()
    }

    /// Get the most recent invocation, if any
    pub fn last_invocation(&self) -> Option<ArmamentCall> {
        self.invocations.lock().unwrap().last().cloned()
    }

    /// Reset all recorded invocations (does not clear responses)
    pub fn reset_invocations(&self) {
        self.invocations.lock().unwrap().clear();
    }

    /// Reconstruct an ArsenalError from a reference (since ArsenalError doesn't impl Clone)
    fn reconstruct_error(error: &ArsenalError, _msg: &str) -> ArsenalError {
        match error {
            ArsenalError::ToolNotFound(s) => ArsenalError::ToolNotFound(s.clone()),
            ArsenalError::InvalidArguments(s) => ArsenalError::InvalidArguments(s.clone()),
            ArsenalError::Timeout(t) => ArsenalError::Timeout(*t),
            ArsenalError::ProtocolError(s) => ArsenalError::ProtocolError(s.clone()),
            ArsenalError::TransportError(s) => ArsenalError::TransportError(s.clone()),
            ArsenalError::AuthFailed(s) => ArsenalError::AuthFailed(s.clone()),
        }
    }

    /// Clone a stored response (since ArsenalError doesn't impl Clone)
    fn clone_response(
        response: &Result<ArmamentResult, ArsenalError>,
    ) -> Result<ArmamentResult, ArsenalError> {
        match response {
            Ok(result) => Ok(result.clone()),
            Err(error) => {
                let msg = format!("{}", error);
                Err(Self::reconstruct_error(error, &msg))
            }
        }
    }
}

impl Default for MockArsenalPort {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl ArsenalPort for MockArsenalPort {
    async fn list_armaments(&self) -> Vec<Armament> {
        self.armaments.clone()
    }

    async fn invoke(&self, call: ArmamentCall) -> Result<ArmamentResult, ArsenalError> {
        // Record invocation for test assertions
        self.invocations.lock().unwrap().push(call.clone());

        // Look up pre-configured response by tool name
        let responses = self.responses.lock().unwrap();
        match responses.get(&call.tool_name) {
            Some(response) => Self::clone_response(response),
            None => Err(ArsenalError::ToolNotFound(format!(
                "No mock response configured for tool: {}",
                call.tool_name
            ))),
        }
    }

    fn validate_call(&self, call: &ArmamentCall) -> Result<(), ArsenalError> {
        // Check if tool exists in registered armaments
        if self.armaments.iter().any(|a| a.name == call.tool_name) {
            Ok(())
        } else {
            Err(ArsenalError::ToolNotFound(format!(
                "Tool not registered: {}",
                call.tool_name
            )))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use uuid::Uuid;

    #[test]
    fn test_mock_arsenal_new_is_empty() {
        let mock = MockArsenalPort::new();
        assert_eq!(mock.call_count(), 0);
        assert!(mock.invocations().is_empty());
        assert!(mock.last_invocation().is_none());
    }

    #[test]
    fn test_mock_arsenal_add_tool() {
        let mut mock = MockArsenalPort::new();
        mock.add_tool("calculator", "A calculator tool");
        mock.add_tool("search", "A search tool");

        let rt = tokio::runtime::Runtime::new().unwrap();
        let armaments = rt.block_on(mock.list_armaments());
        assert_eq!(armaments.len(), 2);
        assert_eq!(armaments[0].name, "calculator");
        assert_eq!(armaments[1].name, "search");
    }

    #[tokio::test]
    async fn test_mock_arsenal_invoke_success() {
        let mut mock = MockArsenalPort::new();
        mock.add_tool("calculator", "A calculator");
        let mock = Arc::new(mock);

        let call_id = Uuid::new_v4();
        mock.set_response(
            "calculator",
            ArmamentResult::success(call_id, json!("42"), 100),
        );

        let mut args = HashMap::new();
        args.insert("a".to_string(), json!(1));
        let call = ArmamentCall::new("calculator", args);
        let result = mock.invoke(call).await;

        assert!(result.is_ok());
        let result = result.unwrap();
        assert!(result.success);
        assert_eq!(result.output, Some(json!("42")));
        assert_eq!(mock.call_count(), 1);
    }

    #[tokio::test]
    async fn test_mock_arsenal_invoke_not_configured() {
        let mock = MockArsenalPort::new();
        let call = ArmamentCall::new("unknown", HashMap::new());
        let result = mock.invoke(call).await;

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(
            format!("{}", err).contains("No mock response configured"),
            "Error should indicate no response configured: {}",
            err
        );
    }

    #[tokio::test]
    async fn test_mock_arsenal_invoke_error() {
        let mock = MockArsenalPort::new();
        mock.set_error("failing_tool", "Simulated failure");

        let call = ArmamentCall::new("failing_tool", HashMap::new());
        let result = mock.invoke(call).await;

        assert!(result.is_err());
        assert_eq!(mock.call_count(), 1);
    }

    #[test]
    fn test_mock_arsenal_validate_call() {
        let mut mock = MockArsenalPort::new();
        mock.add_tool("calculator", "A calculator");

        let valid_call = ArmamentCall::new("calculator", HashMap::new());
        assert!(mock.validate_call(&valid_call).is_ok());

        let invalid_call = ArmamentCall::new("nonexistent", HashMap::new());
        assert!(mock.validate_call(&invalid_call).is_err());
    }

    #[tokio::test]
    async fn test_mock_arsenal_tracks_invocations() {
        let mut mock = MockArsenalPort::new();
        mock.add_tool("tool_a", "Tool A");
        mock.add_tool("tool_b", "Tool B");
        let mock = Arc::new(mock);

        let id_a = Uuid::new_v4();
        let id_b = Uuid::new_v4();
        mock.set_response("tool_a", ArmamentResult::success(id_a, json!("a"), 10));
        mock.set_response("tool_b", ArmamentResult::success(id_b, json!("b"), 20));

        let _ = mock
            .invoke(ArmamentCall::new("tool_a", HashMap::new()))
            .await;
        let _ = mock
            .invoke(ArmamentCall::new("tool_b", HashMap::new()))
            .await;

        assert_eq!(mock.call_count(), 2);
        let invocations = mock.invocations();
        assert_eq!(invocations[0].tool_name, "tool_a");
        assert_eq!(invocations[1].tool_name, "tool_b");
        assert_eq!(mock.last_invocation().unwrap().tool_name, "tool_b");
    }

    #[tokio::test]
    async fn test_mock_arsenal_reusable_response() {
        let mock = MockArsenalPort::new();
        let call_id = Uuid::new_v4();
        mock.set_response(
            "reusable",
            ArmamentResult::success(call_id, json!("result"), 50),
        );

        // Invoke the same tool twice — response should be reusable
        let r1 = mock
            .invoke(ArmamentCall::new("reusable", HashMap::new()))
            .await;
        let r2 = mock
            .invoke(ArmamentCall::new("reusable", HashMap::new()))
            .await;

        assert!(r1.is_ok());
        assert!(r2.is_ok());
        assert_eq!(mock.call_count(), 2);
    }

    #[test]
    fn test_mock_arsenal_default() {
        let mock = MockArsenalPort::default();
        assert_eq!(mock.call_count(), 0);
    }
}
