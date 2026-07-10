//! Integration tests for Arsenal execution service.
//!
//! Tests tool discovery, invocation, error handling, timeouts, and resource limits.
//!
//! # Phase 12.1 Plan 03 behavior change
//!
//! `ArsenalExecutionService::invoke` used to return a hardcoded
//! `"Tool ... executed successfully"` string for ANY registered tool,
//! regardless of whether a real MCP server backed it. It now routes through
//! `clients_by_tool` to the real `MCPClient` registered for that tool name
//! (via `register_client`, called by the config/arsenal loader after
//! `discover_tools()`), and returns `ArsenalError::ToolNotFound` when no
//! client is registered.
//!
//! Because `MCPClient` can only be constructed via a real rmcp handshake
//! (subprocess spawn or HTTP connect) and this file is an external
//! integration-test crate (no access to the crate-private test seam used by
//! `arsenal_execution_service.rs`'s own in-file unit tests), the tests below
//! exercise `ArsenalExecutionService` exactly as `ArsenalExecutionService::new`
//! constructs it with no MCP client registered — proving that validation
//! (tool exists, required params present) still runs correctly and that the
//! final routing step now correctly reports `ToolNotFound` instead of
//! fabricating success. The full real-client success path is proven by
//! `arsenal_execution_service.rs`'s in-file fake-invoker tests, and the
//! end-to-end LLM-driven dispatch is proven by
//! `tests/integration/arsenal_bridge_regression_test.rs`.

use paladin::application::services::arsenal::arsenal_execution_service::ArsenalExecutionService;
use paladin::application::services::arsenal::arsenal_registry_service::ArsenalRegistryService;
use paladin::core::platform::container::arsenal::{Armament, ArmamentCall, ArsenalError};
use paladin_ports::output::arsenal_port::{ArsenalPort, ArsenalRegistry};
use serde_json::{Value, json};
use std::collections::HashMap;
use std::sync::Arc;

/// Helper to create a test armament (tool)
fn create_test_armament(name: &str, required_params: Vec<String>) -> Armament {
    Armament {
        name: name.to_string(),
        description: format!("Test tool: {}", name),
        parameters: json!({
            "type": "object",
            "properties": {
                "input": {
                    "type": "string",
                    "description": "Input parameter"
                }
            }
        }),
        required_params,
    }
}

/// Helper to create test arguments
fn create_args(key: &str, value: Value) -> HashMap<String, Value> {
    let mut args = HashMap::new();
    args.insert(key.to_string(), value);
    args
}

/// Asserts `invoke` reached the MCP-routing step (validation passed) and
/// correctly failed with `ToolNotFound` because no client was registered.
fn assert_tool_not_found(
    result: Result<paladin::core::platform::container::arsenal::ArmamentResult, ArsenalError>,
    expected_name: &str,
) {
    match result {
        Err(ArsenalError::ToolNotFound(name)) => assert_eq!(name, expected_name),
        other => panic!(
            "expected ToolNotFound for '{expected_name}' (no serving client registered), got: {other:?}"
        ),
    }
}

#[tokio::test]
async fn test_arsenal_execution_service_creation() {
    let registry = Arc::new(ArsenalRegistryService::new());
    let _service = ArsenalExecutionService::new(registry);

    // Should create successfully without panicking
    // (No Debug trait required)
}

#[tokio::test]
async fn test_invoke_registered_tool_with_no_mcp_client_is_tool_not_found() {
    let registry = Arc::new(ArsenalRegistryService::new());
    let service = ArsenalExecutionService::new(registry.clone());

    // Register a tool's metadata, but never register a serving MCP client.
    let tool = create_test_armament("calculator", vec!["operation".to_string()]);
    registry.register(tool).await;

    // Create a valid call (passes validate_call + validate_parameters)
    let args = create_args("operation", json!("add"));
    let call = ArmamentCall::new("calculator", args);

    let result = service.invoke(call).await;
    assert_tool_not_found(result, "calculator");
}

#[tokio::test]
async fn test_invoke_tool_not_found_when_not_even_registered_in_registry() {
    let registry = Arc::new(ArsenalRegistryService::new());
    let service = ArsenalExecutionService::new(registry);

    // Create a call to a non-existent tool
    let args = HashMap::new();
    let call = ArmamentCall::new("nonexistent_tool", args);

    // Invoke should fail
    let result = service.invoke(call).await;

    assert!(result.is_err());
    match result {
        Err(ArsenalError::ToolNotFound(name)) => {
            assert_eq!(name, "nonexistent_tool");
        }
        _ => panic!("Expected ToolNotFound error"),
    }
}

#[tokio::test]
async fn test_invoke_missing_required_parameter() {
    let registry = Arc::new(ArsenalRegistryService::new());
    let service = ArsenalExecutionService::new(registry.clone());

    // Register a tool with required parameters
    let tool = create_test_armament("validator", vec!["email".to_string(), "age".to_string()]);
    registry.register(tool).await;

    // Create a call missing a required parameter
    let args = create_args("email", json!("test@example.com"));
    let call = ArmamentCall::new("validator", args);

    // Invoke should fail with InvalidArguments -- validation runs BEFORE the
    // client-routing lookup, so this is unaffected by the no-client change.
    let result = service.invoke(call).await;

    assert!(result.is_err());
    match result {
        Err(ArsenalError::InvalidArguments(msg)) => {
            assert!(msg.contains("Missing required parameter"));
            assert!(msg.contains("age"));
        }
        _ => panic!("Expected InvalidArguments error"),
    }
}

#[tokio::test]
async fn test_validate_call_empty_tool_name() {
    let registry = Arc::new(ArsenalRegistryService::new());
    let service = ArsenalExecutionService::new(registry);

    // Create a call with empty tool name
    let call = ArmamentCall::new("", HashMap::new());

    // Validation should fail
    let result = service.validate_call(&call);

    assert!(result.is_err());
    match result {
        Err(ArsenalError::InvalidArguments(msg)) => {
            assert!(msg.contains("Tool name cannot be empty"));
        }
        _ => panic!("Expected InvalidArguments error"),
    }
}

#[tokio::test]
async fn test_validate_call_valid() {
    let registry = Arc::new(ArsenalRegistryService::new());
    let service = ArsenalExecutionService::new(registry);

    // Create a valid call
    let args = HashMap::new();
    let call = ArmamentCall::new("test_tool", args);

    // Validation should succeed
    let result = service.validate_call(&call);
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_invoke_tool_with_all_required_parameters_passes_validation_then_tool_not_found() {
    let registry = Arc::new(ArsenalRegistryService::new());
    let service = ArsenalExecutionService::new(registry.clone());

    // Register a tool with multiple required parameters
    let tool = create_test_armament(
        "multi_param_tool",
        vec![
            "param1".to_string(),
            "param2".to_string(),
            "param3".to_string(),
        ],
    );
    registry.register(tool).await;

    // Create a call with all required parameters
    let mut args = HashMap::new();
    args.insert("param1".to_string(), json!("value1"));
    args.insert("param2".to_string(), json!("value2"));
    args.insert("param3".to_string(), json!("value3"));
    let call = ArmamentCall::new("multi_param_tool", args);

    // Parameter validation passes (proving all-required-params-present logic
    // still works); routing then correctly reports ToolNotFound since no
    // client is registered.
    let result = service.invoke(call).await;
    assert_tool_not_found(result, "multi_param_tool");
}

#[tokio::test]
async fn test_invoke_tool_with_extra_parameters_passes_validation_then_tool_not_found() {
    let registry = Arc::new(ArsenalRegistryService::new());
    let service = ArsenalExecutionService::new(registry.clone());

    // Register a tool with one required parameter
    let tool = create_test_armament("simple_tool", vec!["required".to_string()]);
    registry.register(tool).await;

    // Create a call with required + extra parameters
    let mut args = HashMap::new();
    args.insert("required".to_string(), json!("value"));
    args.insert("optional".to_string(), json!("extra"));
    args.insert("another_optional".to_string(), json!(123));
    let call = ArmamentCall::new("simple_tool", args);

    // Extra params are allowed (validation passes); routing then correctly
    // reports ToolNotFound since no client is registered.
    let result = service.invoke(call).await;
    assert_tool_not_found(result, "simple_tool");
}

#[tokio::test]
async fn test_list_armaments_empty_registry() {
    let registry = Arc::new(ArsenalRegistryService::new());
    let service = ArsenalExecutionService::new(registry);

    // List should return empty vector
    let tools = service.list_armaments().await;
    assert_eq!(tools.len(), 0);
}

#[tokio::test]
async fn test_list_armaments_returns_real_registered_tools() {
    // OQ1 fix: list_armaments() now delegates to the registry's real list(),
    // instead of hard-returning an empty Vec.
    let registry = Arc::new(ArsenalRegistryService::new());
    let service = ArsenalExecutionService::new(registry.clone());

    for i in 1..=3 {
        registry
            .register(create_test_armament(&format!("tool{i}"), vec![]))
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

#[tokio::test]
async fn test_invoke_with_no_client_fails_fast_not_hung() {
    let registry = Arc::new(ArsenalRegistryService::new());
    let service = ArsenalExecutionService::new(registry.clone());

    // Register a tool
    let tool = create_test_armament("timer_tool", vec![]);
    registry.register(tool).await;

    // Invoking without a registered client should fail immediately with
    // ToolNotFound, not hang or panic.
    let call = ArmamentCall::new("timer_tool", HashMap::new());
    let result = service.invoke(call).await;
    assert_tool_not_found(result, "timer_tool");
}

#[tokio::test]
async fn test_concurrent_tool_invocations_without_clients_all_report_tool_not_found() {
    let registry = Arc::new(ArsenalRegistryService::new());
    let service = Arc::new(ArsenalExecutionService::new(registry.clone()));

    // Register multiple tools (metadata only, no serving clients)
    for i in 1..=5 {
        let tool = create_test_armament(&format!("tool{}", i), vec![]);
        registry.register(tool).await;
    }

    // Create concurrent invocations
    let mut handles = vec![];
    for i in 1..=5 {
        let service_clone = service.clone();
        let handle = tokio::spawn(async move {
            let call = ArmamentCall::new(format!("tool{}", i), HashMap::new());
            (i, service_clone.invoke(call).await)
        });
        handles.push(handle);
    }

    // Wait for all invocations
    let mut results = vec![];
    for handle in handles {
        results.push(handle.await.unwrap());
    }

    // All should consistently report ToolNotFound under concurrent access to
    // the shared RwLock-guarded clients_by_tool map -- proving there's no
    // race/deadlock in the new routing path.
    assert_eq!(results.len(), 5);
    for (i, result) in results {
        assert_tool_not_found(result, &format!("tool{}", i));
    }
}

#[tokio::test]
async fn test_tool_invocation_with_complex_json_arguments_passes_validation_then_tool_not_found() {
    let registry = Arc::new(ArsenalRegistryService::new());
    let service = ArsenalExecutionService::new(registry.clone());

    // Register a tool
    let tool = create_test_armament("complex_tool", vec!["config".to_string()]);
    registry.register(tool).await;

    // Create a call with complex nested JSON
    let mut args = HashMap::new();
    args.insert(
        "config".to_string(),
        json!({
            "server": {
                "host": "localhost",
                "port": 8080,
                "features": ["logging", "monitoring", "metrics"]
            },
            "database": {
                "connections": 10,
                "timeout": 5000
            }
        }),
    );
    let call = ArmamentCall::new("complex_tool", args);

    // Complex nested JSON arguments don't trip validation; routing then
    // correctly reports ToolNotFound since no client is registered.
    let result = service.invoke(call).await;
    assert_tool_not_found(result, "complex_tool");
}

#[tokio::test]
async fn test_invoke_error_preserves_tool_name_not_original_call_id() {
    let registry = Arc::new(ArsenalRegistryService::new());
    let service = ArsenalExecutionService::new(registry.clone());

    // Register a tool
    let tool = create_test_armament("id_test_tool", vec![]);
    registry.register(tool).await;

    // Create a call
    let call = ArmamentCall::new("id_test_tool", HashMap::new());

    // With no client registered, invoke fails before an ArmamentResult (and
    // its call_id-echoing success path) is ever constructed; the returned
    // error still names the correct tool.
    let result = service.invoke(call).await;
    assert_tool_not_found(result, "id_test_tool");
}

#[tokio::test]
async fn test_validate_parameters_no_required_params_then_tool_not_found() {
    let registry = Arc::new(ArsenalRegistryService::new());
    let service = ArsenalExecutionService::new(registry.clone());

    // Register a tool with no required parameters
    let tool = create_test_armament("optional_tool", vec![]);
    registry.register(tool).await;

    // Create a call with no arguments
    let call = ArmamentCall::new("optional_tool", HashMap::new());

    // No required params means validate_parameters passes trivially; routing
    // then correctly reports ToolNotFound since no client is registered.
    let result = service.invoke(call).await;
    assert_tool_not_found(result, "optional_tool");
}
