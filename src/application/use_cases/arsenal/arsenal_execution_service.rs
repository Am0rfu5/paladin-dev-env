//! Arsenal execution service implementation.
//!
//! Provides tool validation and execution coordination.

use crate::application::ports::output::arsenal_port::{ArsenalPort, ArsenalRegistry};
use crate::core::platform::container::arsenal::{
    Armament, ArmamentCall, ArmamentResult, ArsenalError,
};
use async_trait::async_trait;
use serde_json::Value;
use std::sync::Arc;

/// Service for executing tools through the Arsenal.
///
/// `ArsenalExecutionService` coordinates tool execution by validating
/// requests against the registry and delegating to appropriate handlers.
///
/// # Example
///
/// ```rust,ignore
/// use paladin::application::use_cases::arsenal::arsenal_execution_service::ArsenalExecutionService;
/// use paladin::application::use_cases::arsenal::arsenal_registry_service::ArsenalRegistryService;
///
/// let registry = Arc::new(ArsenalRegistryService::new());
/// let service = ArsenalExecutionService::new(registry);
/// ```
pub struct ArsenalExecutionService {
    /// Registry for looking up tool metadata
    registry: Arc<dyn ArsenalRegistry>,
}

impl ArsenalExecutionService {
    /// Creates a new Arsenal execution service.
    ///
    /// # Arguments
    ///
    /// * `registry` - The tool registry for looking up tool metadata
    ///
    /// # Example
    ///
    /// ```rust
    /// use paladin::application::use_cases::arsenal::arsenal_execution_service::ArsenalExecutionService;
    /// use paladin::application::use_cases::arsenal::arsenal_registry_service::ArsenalRegistryService;
    /// use std::sync::Arc;
    ///
    /// let registry = Arc::new(ArsenalRegistryService::new());
    /// let service = ArsenalExecutionService::new(registry);
    /// ```
    pub fn new(registry: Arc<dyn ArsenalRegistry>) -> Self {
        Self { registry }
    }

    /// Validates call arguments against the tool's parameter schema.
    ///
    /// Checks that all required parameters are present in the call arguments.
    fn validate_parameters(
        &self,
        armament: &Armament,
        call: &ArmamentCall,
    ) -> Result<(), ArsenalError> {
        // Check all required parameters are present
        for required_param in &armament.required_params {
            if !call.arguments.contains_key(required_param) {
                return Err(ArsenalError::InvalidArguments(format!(
                    "Missing required parameter: {}",
                    required_param
                )));
            }
        }

        Ok(())
    }
}

#[async_trait]
impl ArsenalPort for ArsenalExecutionService {
    async fn list_armaments(&self) -> Vec<Armament> {
        // Note: This is a simplified implementation
        // In a full implementation, we'd iterate through the registry
        // For now, return empty vector as registry doesn't expose list()
        Vec::new()
    }

    async fn invoke(&self, call: ArmamentCall) -> Result<ArmamentResult, ArsenalError> {
        // Validate the call first
        self.validate_call(&call)?;

        // Get the armament from registry to validate parameters
        let armament = self
            .registry
            .get(&call.tool_name)
            .await
            .ok_or_else(|| ArsenalError::ToolNotFound(call.tool_name.clone()))?;

        // Validate parameters against schema
        self.validate_parameters(&armament, &call)?;

        // For now, return a mock successful result
        // In a full implementation, this would delegate to MCP transport
        let start = tokio::time::Instant::now();

        // Simulate successful execution
        let output = Value::String(format!("Tool {} executed successfully", call.tool_name));

        let execution_time = start.elapsed().as_millis() as u64;

        Ok(ArmamentResult::success(
            call.call_id,
            output,
            execution_time,
        ))
    }

    fn validate_call(&self, call: &ArmamentCall) -> Result<(), ArsenalError> {
        // This is a synchronous validation that will be completed when we have
        // a way to synchronously access the registry, or we make validate_call async
        // For now, we'll do basic validation

        if call.tool_name.is_empty() {
            return Err(ArsenalError::InvalidArguments(
                "Tool name cannot be empty".to_string(),
            ));
        }

        Ok(())
    }
}
