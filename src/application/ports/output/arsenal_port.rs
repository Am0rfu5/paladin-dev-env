//! Arsenal port definitions for tool operations.
//!
//! This module defines the port interfaces for the Arsenal tool system,
//! following hexagonal architecture principles. These traits abstract
//! the tool execution and registry operations from their implementations.

use crate::core::platform::container::arsenal::{
    Armament, ArmamentCall, ArmamentResult, ArsenalError,
};
use async_trait::async_trait;

/// Port for tool execution operations.
///
/// The `ArsenalPort` trait defines the interface for executing tools,
/// validating tool calls, and listing available tools. Implementations
/// handle the actual execution logic and communication with tool providers.
///
/// # Example
///
/// ```rust,ignore
/// use paladin::application::ports::output::arsenal_port::ArsenalPort;
/// use paladin::core::platform::container::arsenal::ArmamentCall;
///
/// async fn execute_tool(
///     arsenal: &dyn ArsenalPort,
///     call: ArmamentCall,
/// ) -> Result<(), Box<dyn std::error::Error>> {
///     arsenal.validate_call(&call)?;
///     let result = arsenal.invoke(call).await?;
///     println!("Tool executed: {:?}", result);
///     Ok(())
/// }
/// ```
#[async_trait]
pub trait ArsenalPort: Send + Sync {
    /// Lists all available tools in the Arsenal.
    ///
    /// Returns a vector of all registered tools with their metadata,
    /// including name, description, parameters, and requirements.
    ///
    /// # Returns
    ///
    /// A vector of `Armament` instances representing available tools.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let tools = arsenal.list_armaments().await;
    /// for tool in tools {
    ///     println!("Available: {} - {}", tool.name, tool.description);
    /// }
    /// ```
    async fn list_armaments(&self) -> Vec<Armament>;

    /// Invokes a tool with the provided arguments.
    ///
    /// Executes the specified tool with the given arguments and returns
    /// the result. This method handles tool execution, error recovery,
    /// and result formatting.
    ///
    /// # Arguments
    ///
    /// * `call` - The tool invocation request containing tool name and arguments
    ///
    /// # Returns
    ///
    /// An `ArmamentResult` containing the execution outcome, output data,
    /// and execution metrics.
    ///
    /// # Errors
    ///
    /// Returns `ArsenalError` if:
    /// - Tool is not found
    /// - Arguments are invalid
    /// - Execution times out
    /// - Communication with tool fails
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let call = ArmamentCall::new("calculator", args);
    /// let result = arsenal.invoke(call).await?;
    /// if result.success {
    ///     println!("Output: {:?}", result.output);
    /// }
    /// ```
    async fn invoke(&self, call: ArmamentCall) -> Result<ArmamentResult, ArsenalError>;

    /// Validates a tool call before execution.
    ///
    /// Checks that the tool exists, all required parameters are provided,
    /// and arguments match the expected JSON schema.
    ///
    /// # Arguments
    ///
    /// * `call` - The tool invocation request to validate
    ///
    /// # Returns
    ///
    /// `Ok(())` if the call is valid.
    ///
    /// # Errors
    ///
    /// Returns `ArsenalError` if:
    /// - Tool does not exist
    /// - Required parameters are missing
    /// - Parameter types don't match schema
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// if arsenal.validate_call(&call).is_ok() {
    ///     let result = arsenal.invoke(call).await?;
    /// }
    /// ```
    fn validate_call(&self, call: &ArmamentCall) -> Result<(), ArsenalError>;
}

/// Port for tool registry operations.
///
/// The `ArsenalRegistry` trait defines the interface for managing the
/// collection of available tools. Implementations handle tool storage,
/// retrieval, and lifecycle management.
///
/// # Example
///
/// ```rust,ignore
/// use paladin::application::ports::output::arsenal_port::ArsenalRegistry;
/// use paladin::core::platform::container::arsenal::Armament;
///
/// async fn register_tool(
///     registry: &dyn ArsenalRegistry,
///     tool: Armament,
/// ) -> Result<(), Box<dyn std::error::Error>> {
///     registry.register(tool).await;
///     println!("Tool registered successfully");
///     Ok(())
/// }
/// ```
#[async_trait]
pub trait ArsenalRegistry: Send + Sync {
    /// Registers a new tool in the Arsenal.
    ///
    /// Adds a tool to the registry, making it available for invocation.
    /// If a tool with the same name already exists, it will be replaced.
    ///
    /// # Arguments
    ///
    /// * `armament` - The tool metadata to register
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let tool = Armament {
    ///     name: "calculator".to_string(),
    ///     description: "Basic math operations".to_string(),
    ///     parameters: json!({"type": "object"}),
    ///     required_params: vec!["operation".to_string()],
    /// };
    /// registry.register(tool).await;
    /// ```
    async fn register(&self, armament: Armament);

    /// Unregisters a tool from the Arsenal.
    ///
    /// Removes a tool from the registry by name. Returns the removed
    /// tool metadata if it existed.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the tool to remove
    ///
    /// # Returns
    ///
    /// `Some(Armament)` if the tool was found and removed, `None` otherwise.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// if let Some(removed) = registry.unregister("calculator").await {
    ///     println!("Removed tool: {}", removed.name);
    /// }
    /// ```
    async fn unregister(&self, name: &str) -> Option<Armament>;

    /// Retrieves a tool by name.
    ///
    /// Looks up a tool in the registry and returns its metadata if found.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the tool to retrieve
    ///
    /// # Returns
    ///
    /// `Some(Armament)` if the tool exists, `None` otherwise.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// if let Some(tool) = registry.get("calculator").await {
    ///     println!("Found tool: {}", tool.description);
    /// }
    /// ```
    async fn get(&self, name: &str) -> Option<Armament>;
}
