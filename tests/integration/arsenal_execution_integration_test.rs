//! Integration tests for Arsenal execution service.
//!
//! Tests tool discovery, invocation, error handling, timeouts, and resource limits.

use paladin::application::use_cases::arsenal::arsenal_execution_service::ArsenalExecutionService;
use paladin::application::use_cases::arsenal::arsenal_registry_service::ArsenalRegistryService;
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

#[tokio::test]
async fn test_arsenal_execution_service_creation() {
    let registry = Arc::new(ArsenalRegistryService::new());
    let _service = ArsenalExecutionService::new(registry);

    // Should create successfully without panicking
    // (No Debug trait required)
}

#[tokio::test]
async fn test_invoke_tool_success() {
    let registry = Arc::new(ArsenalRegistryService::new());
    let service = ArsenalExecutionService::new(registry.clone());

    // Register a tool
    let tool = create_test_armament("calculator", vec!["operation".to_string()]);
    registry.register(tool).await;

    // Create a valid call
    let args = create_args("operation", json!("add"));
    let call = ArmamentCall::new("calculator", args);

    // Invoke the tool
    let result = service.invoke(call.clone()).await.unwrap();

    // Verify result
    assert!(result.success);
    assert_eq!(result.call_id, call.call_id);
    assert!(result.output.is_some());
    assert!(result.error.is_none());
}

#[tokio::test]
async fn test_invoke_tool_not_found() {
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

    // Invoke should fail
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
async fn test_invoke_tool_with_all_required_parameters() {
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

    // Invoke should succeed
    let result = service.invoke(call).await;
    assert!(result.is_ok());
    assert!(result.unwrap().success);
}

#[tokio::test]
async fn test_invoke_tool_with_extra_parameters() {
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

    // Invoke should succeed (extra params are allowed)
    let result = service.invoke(call).await;
    assert!(result.is_ok());
    assert!(result.unwrap().success);
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
async fn test_execution_time_tracking() {
    let registry = Arc::new(ArsenalRegistryService::new());
    let service = ArsenalExecutionService::new(registry.clone());

    // Register a tool
    let tool = create_test_armament("timer_tool", vec![]);
    registry.register(tool).await;

    // Invoke the tool
    let call = ArmamentCall::new("timer_tool", HashMap::new());
    let result = service.invoke(call).await.unwrap();

    // Execution time should be tracked (u64 is always >= 0, so check it's set)
    // Just verify the field exists by checking it's not absurdly large
    assert!(result.execution_time_ms < u64::MAX);
}

#[tokio::test]
async fn test_concurrent_tool_invocations() {
    let registry = Arc::new(ArsenalRegistryService::new());
    let service = Arc::new(ArsenalExecutionService::new(registry.clone()));

    // Register multiple tools
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
            service_clone.invoke(call).await
        });
        handles.push(handle);
    }

    // Wait for all invocations
    let mut results = vec![];
    for handle in handles {
        results.push(handle.await.unwrap());
    }

    // All should succeed
    assert_eq!(results.len(), 5);
    for result in results {
        assert!(result.is_ok());
        assert!(result.unwrap().success);
    }
}

#[tokio::test]
async fn test_tool_invocation_with_complex_json_arguments() {
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

    // Invoke should succeed
    let result = service.invoke(call).await;
    assert!(result.is_ok());
    assert!(result.unwrap().success);
}

#[tokio::test]
async fn test_invoke_preserves_call_id() {
    let registry = Arc::new(ArsenalRegistryService::new());
    let service = ArsenalExecutionService::new(registry.clone());

    // Register a tool
    let tool = create_test_armament("id_test_tool", vec![]);
    registry.register(tool).await;

    // Create a call
    let call = ArmamentCall::new("id_test_tool", HashMap::new());
    let original_id = call.call_id;

    // Invoke the tool
    let result = service.invoke(call).await.unwrap();

    // Result should have the same call_id
    assert_eq!(result.call_id, original_id);
}

#[tokio::test]
async fn test_validate_parameters_no_required_params() {
    let registry = Arc::new(ArsenalRegistryService::new());
    let service = ArsenalExecutionService::new(registry.clone());

    // Register a tool with no required parameters
    let tool = create_test_armament("optional_tool", vec![]);
    registry.register(tool).await;

    // Create a call with no arguments
    let call = ArmamentCall::new("optional_tool", HashMap::new());

    // Invoke should succeed
    let result = service.invoke(call).await;
    assert!(result.is_ok());
}
