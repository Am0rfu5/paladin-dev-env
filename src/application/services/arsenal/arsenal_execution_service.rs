//! Arsenal execution service implementation.
//!
//! Provides tool validation and execution coordination.

use crate::core::platform::container::arsenal::{
    Armament, ArmamentCall, ArmamentResult, ArsenalError,
};
use crate::infrastructure::adapters::arsenal::mcp_protocol::MCPClient;
use async_trait::async_trait;
use paladin_ports::output::arsenal_port::{ArsenalPort, ArsenalRegistry};
use serde_json::Value;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Internal seam over [`MCPClient::invoke_tool`], letting
/// `ArsenalExecutionService::invoke`'s routing/error-mapping logic be
/// unit-tested with a lightweight in-file fake double.
///
/// `MCPClient` can only be constructed via a real rmcp handshake (subprocess
/// spawn or HTTP connect), so it cannot itself stand in for a mock in unit
/// tests. This trait is crate-private: the public facade
/// ([`ArsenalExecutionService::register_client`]) still takes a concrete
/// `Arc<MCPClient>`, so callers never see this seam.
#[async_trait]
trait McpToolInvoker: Send + Sync {
    async fn call_tool(
        &self,
        tool_name: &str,
        arguments: HashMap<String, Value>,
    ) -> Result<Value, ArsenalError>;
}

#[async_trait]
impl McpToolInvoker for MCPClient {
    async fn call_tool(
        &self,
        tool_name: &str,
        arguments: HashMap<String, Value>,
    ) -> Result<Value, ArsenalError> {
        self.invoke_tool(tool_name, arguments).await
    }
}

/// Service for executing tools through the Arsenal.
///
/// `ArsenalExecutionService` coordinates tool execution by validating
/// requests against the registry and delegating to the real MCP client that
/// serves each tool (populated via [`Self::register_client`]).
///
/// # Examples
///
/// ```rust,ignore
/// use paladin::application::services::arsenal::arsenal_execution_service::ArsenalExecutionService;
/// use paladin::application::services::arsenal::arsenal_registry_service::ArsenalRegistryService;
///
/// let registry = Arc::new(ArsenalRegistryService::new());
/// let service = ArsenalExecutionService::new(registry);
/// ```
pub struct ArsenalExecutionService {
    /// Registry for looking up tool metadata
    registry: Arc<dyn ArsenalRegistry>,

    /// Routing table from tool name to the MCP client that serves it.
    /// Populated by [`Self::register_client`] after the config/arsenal
    /// loader connects a server and discovers its tools.
    clients_by_tool: Arc<RwLock<HashMap<String, Arc<dyn McpToolInvoker>>>>,
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
    /// use paladin::application::services::arsenal::arsenal_execution_service::ArsenalExecutionService;
    /// use paladin::application::services::arsenal::arsenal_registry_service::ArsenalRegistryService;
    /// use std::sync::Arc;
    ///
    /// let registry = Arc::new(ArsenalRegistryService::new());
    /// let service = ArsenalExecutionService::new(registry);
    /// ```
    pub fn new(registry: Arc<dyn ArsenalRegistry>) -> Self {
        Self {
            registry,
            clients_by_tool: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Registers `client` as the serving MCP connection for each of
    /// `discovered_tool_names`.
    ///
    /// The config/arsenal loader (`application::cli::config::loader::instantiate_arsenal`)
    /// is the intended caller: after it connects a server via
    /// `MCPClient::connect_stdio`/`connect_streamable_http` and calls
    /// `discover_tools()`, it calls this method so that
    /// [`Self::invoke`]/[`Self::invoke_tool_direct`] have a real client to
    /// route through instead of failing with `ToolNotFound`.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let client = MCPClient::connect_stdio("uvx", &["mcp-web-search".to_string()]).await?;
    /// let tools = client.discover_tools().await?;
    /// let tool_names: Vec<String> = tools.iter().map(|t| t.name.clone()).collect();
    /// for tool in tools {
    ///     registry.register(tool).await;
    /// }
    /// service.register_client(tool_names, Arc::new(client)).await;
    /// ```
    pub async fn register_client(
        &self,
        discovered_tool_names: Vec<String>,
        client: Arc<MCPClient>,
    ) {
        let invoker: Arc<dyn McpToolInvoker> = client;
        let mut map = self.clients_by_tool.write().await;
        for tool_name in discovered_tool_names {
            map.insert(tool_name, invoker.clone());
        }
    }

    /// Deterministic one-shot invocation path (SC3): a non-agent caller
    /// invokes exactly one tool with a hand-built argument map — no LLM
    /// loop, no `PaladinExecutionService` — and gets back the raw tool
    /// output.
    ///
    /// This is a thin convenience wrapper around [`Self::invoke`] that
    /// builds the `ArmamentCall` and unwraps the successful output, which is
    /// exactly the shape a non-agent caller (e.g. Phase 13's re-point) needs.
    ///
    /// # Errors
    ///
    /// Returns `ArsenalError` under the same conditions as [`Self::invoke`]
    /// (tool not found, invalid arguments, no serving client, or a real MCP
    /// tool-call failure).
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let output = service.invoke_tool_direct("web_search", args).await?;
    /// ```
    pub async fn invoke_tool_direct(
        &self,
        tool_name: &str,
        arguments: HashMap<String, Value>,
    ) -> Result<Value, ArsenalError> {
        let call = ArmamentCall::new(tool_name.to_string(), arguments);
        let result = self.invoke(call).await?;
        result.output.ok_or_else(|| {
            ArsenalError::ProtocolError(format!(
                "Tool '{tool_name}' reported success but returned no output"
            ))
        })
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
        self.registry.list().await
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

        // Look up the MCP client actually serving this tool
        let client = {
            let map = self.clients_by_tool.read().await;
            map.get(&call.tool_name).cloned()
        }
        .ok_or_else(|| ArsenalError::ToolNotFound(call.tool_name.clone()))?;

        let start = tokio::time::Instant::now();

        let output = client
            .call_tool(&call.tool_name, call.arguments.clone())
            .await?;

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::application::services::arsenal::arsenal_registry_service::ArsenalRegistryService;
    use serde_json::json;
    use std::sync::atomic::{AtomicUsize, Ordering};

    /// In-file fake standing in for a real `MCPClient` — `MCPClient` itself
    /// can only be constructed via a real rmcp handshake, so this fake
    /// implements the crate-private `McpToolInvoker` seam directly.
    struct FakeInvoker {
        output: Value,
        calls: AtomicUsize,
    }

    impl FakeInvoker {
        fn new(output: Value) -> Self {
            Self {
                output,
                calls: AtomicUsize::new(0),
            }
        }
    }

    #[async_trait]
    impl McpToolInvoker for FakeInvoker {
        async fn call_tool(
            &self,
            _tool_name: &str,
            _arguments: HashMap<String, Value>,
        ) -> Result<Value, ArsenalError> {
            self.calls.fetch_add(1, Ordering::SeqCst);
            Ok(self.output.clone())
        }
    }

    fn make_armament(name: &str, required_params: Vec<String>) -> Armament {
        Armament {
            name: name.to_string(),
            description: format!("Test tool: {name}"),
            parameters: json!({"type": "object"}),
            required_params,
        }
    }

    /// Test-only helper: inserts a fake invoker directly into
    /// `clients_by_tool`, bypassing the public `register_client(Arc<MCPClient>)`
    /// facade (which cannot accept a fake in place of a real `MCPClient`).
    /// Accessible because this test module is a child of the service's own
    /// module, so Rust's privacy rules permit reaching the private field.
    async fn insert_fake_client(
        service: &ArsenalExecutionService,
        tool_name: &str,
        invoker: Arc<dyn McpToolInvoker>,
    ) {
        service
            .clients_by_tool
            .write()
            .await
            .insert(tool_name.to_string(), invoker);
    }

    #[tokio::test]
    async fn invoke_routes_to_the_registered_client_and_returns_real_output() {
        let registry = Arc::new(ArsenalRegistryService::new());
        let service = ArsenalExecutionService::new(registry.clone());

        registry.register(make_armament("calculator", vec![])).await;
        let fake = Arc::new(FakeInvoker::new(json!({"sum": 42})));
        insert_fake_client(&service, "calculator", fake.clone()).await;

        let call = ArmamentCall::new("calculator", HashMap::new());
        let result = service.invoke(call).await.expect("invoke should succeed");

        assert!(result.success);
        assert_eq!(result.output, Some(json!({"sum": 42})));
        assert_eq!(fake.calls.load(Ordering::SeqCst), 1);
    }

    #[tokio::test]
    async fn invoke_never_returns_the_old_hardcoded_stub_string() {
        let registry = Arc::new(ArsenalRegistryService::new());
        let service = ArsenalExecutionService::new(registry.clone());

        registry.register(make_armament("echo", vec![])).await;
        insert_fake_client(
            &service,
            "echo",
            Arc::new(FakeInvoker::new(json!("real mcp output"))),
        )
        .await;

        let call = ArmamentCall::new("echo", HashMap::new());
        let result = service.invoke(call).await.expect("invoke should succeed");

        let output = result.output.expect("output should be present");
        assert_ne!(output, json!("Tool echo executed successfully"));
        assert_eq!(output, json!("real mcp output"));
    }

    #[tokio::test]
    async fn invoke_with_no_serving_client_returns_tool_not_found() {
        let registry = Arc::new(ArsenalRegistryService::new());
        let service = ArsenalExecutionService::new(registry.clone());

        // Registered in the registry, but no MCP client was ever registered
        // to serve it (register_client/insert_fake_client never called).
        registry.register(make_armament("orphaned", vec![])).await;

        let call = ArmamentCall::new("orphaned", HashMap::new());
        let result = service.invoke(call).await;

        match result {
            Err(ArsenalError::ToolNotFound(name)) => assert_eq!(name, "orphaned"),
            other => panic!("expected ToolNotFound, got: {other:?}"),
        }
    }

    #[tokio::test]
    async fn invoke_missing_required_parameter_still_yields_invalid_arguments() {
        // Existing validate_call/validate_parameters behavior preserved
        // unchanged even after wiring in the real MCP routing.
        let registry = Arc::new(ArsenalRegistryService::new());
        let service = ArsenalExecutionService::new(registry.clone());

        registry
            .register(make_armament("validator", vec!["email".to_string()]))
            .await;
        insert_fake_client(
            &service,
            "validator",
            Arc::new(FakeInvoker::new(json!(null))),
        )
        .await;

        let call = ArmamentCall::new("validator", HashMap::new());
        let result = service.invoke(call).await;

        match result {
            Err(ArsenalError::InvalidArguments(msg)) => {
                assert!(msg.contains("email"));
            }
            other => panic!("expected InvalidArguments, got: {other:?}"),
        }
    }

    /// SC3: a non-agent caller invokes a deterministic one-shot tool call
    /// through `ArsenalExecutionService` (no LLM loop, no
    /// `PaladinExecutionService` anywhere in this test) and gets a real
    /// tool result back.
    #[tokio::test]
    async fn invoke_tool_direct_proves_sc3_with_no_llm_loop_involved() {
        let registry = Arc::new(ArsenalRegistryService::new());
        let service = ArsenalExecutionService::new(registry.clone());

        registry
            .register(make_armament("web_search", vec!["query".to_string()]))
            .await;
        insert_fake_client(
            &service,
            "web_search",
            Arc::new(FakeInvoker::new(json!({"results": ["one", "two"]}))),
        )
        .await;

        let mut args = HashMap::new();
        args.insert("query".to_string(), json!("rust async"));

        let output = service
            .invoke_tool_direct("web_search", args)
            .await
            .expect("deterministic one-shot invocation should succeed");

        assert_eq!(output, json!({"results": ["one", "two"]}));
    }

    #[tokio::test]
    async fn invoke_tool_direct_propagates_tool_not_found() {
        let registry = Arc::new(ArsenalRegistryService::new());
        let service = ArsenalExecutionService::new(registry);

        let result = service
            .invoke_tool_direct("never_registered", HashMap::new())
            .await;

        assert!(matches!(result, Err(ArsenalError::ToolNotFound(_))));
    }

    #[tokio::test]
    async fn list_armaments_delegates_to_the_registry_and_returns_n_registered_tools() {
        let registry = Arc::new(ArsenalRegistryService::new());
        let service = ArsenalExecutionService::new(registry.clone());

        assert!(service.list_armaments().await.is_empty());

        for i in 1..=3 {
            registry
                .register(make_armament(&format!("tool{i}"), vec![]))
                .await;
        }

        let mut names: Vec<String> = service
            .list_armaments()
            .await
            .into_iter()
            .map(|a| a.name)
            .collect();
        names.sort();

        assert_eq!(names, vec!["tool1", "tool2", "tool3"]);
    }
}
