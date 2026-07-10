//! Unit tests for Arsenal port implementations

use paladin::application::services::arsenal::arsenal_execution_service::ArsenalExecutionService;
use paladin::application::services::arsenal::arsenal_registry_service::ArsenalRegistryService;
use paladin::core::platform::container::arsenal::{Armament, ArmamentCall, ArsenalError};
use paladin_ports::output::arsenal_port::{ArsenalPort, ArsenalRegistry};
use serde_json::{Value, json};
use std::collections::HashMap;
use std::sync::Arc;

#[tokio::test]
async fn test_registry_register_tool() {
    let registry = ArsenalRegistryService::new();

    let tool = Armament {
        name: "calculator".to_string(),
        description: "Performs basic calculations".to_string(),
        parameters: json!({
            "type": "object",
            "properties": {
                "operation": {"type": "string"},
                "a": {"type": "number"},
                "b": {"type": "number"}
            }
        }),
        required_params: vec!["operation".to_string(), "a".to_string(), "b".to_string()],
    };

    registry.register(tool.clone()).await;

    let retrieved = registry.get("calculator").await;
    assert!(retrieved.is_some());
    assert_eq!(retrieved.unwrap().name, "calculator");
}

#[tokio::test]
async fn test_registry_get_tool() {
    let registry = ArsenalRegistryService::new();

    // Tool should not exist initially
    assert!(registry.get("nonexistent").await.is_none());

    // Register a tool
    let tool = Armament {
        name: "web_search".to_string(),
        description: "Search the web".to_string(),
        parameters: json!({"type": "object"}),
        required_params: vec!["query".to_string()],
    };
    registry.register(tool.clone()).await;

    // Should be able to retrieve it
    let retrieved = registry.get("web_search").await;
    assert!(retrieved.is_some());
    let retrieved_tool = retrieved.unwrap();
    assert_eq!(retrieved_tool.name, "web_search");
    assert_eq!(retrieved_tool.description, "Search the web");
}

#[tokio::test]
async fn test_registry_unregister_tool() {
    let registry = ArsenalRegistryService::new();

    let tool = Armament {
        name: "temp_tool".to_string(),
        description: "Temporary tool".to_string(),
        parameters: json!({}),
        required_params: vec![],
    };

    registry.register(tool).await;
    assert!(registry.get("temp_tool").await.is_some());

    // Unregister should return the removed tool
    let removed = registry.unregister("temp_tool").await;
    assert!(removed.is_some());
    assert_eq!(removed.unwrap().name, "temp_tool");

    // Tool should no longer exist
    assert!(registry.get("temp_tool").await.is_none());

    // Unregistering non-existent tool should return None
    assert!(registry.unregister("never_existed").await.is_none());
}

#[tokio::test]
async fn test_validate_call_success() {
    let registry = Arc::new(ArsenalRegistryService::new());
    let service = ArsenalExecutionService::new(registry.clone());

    let tool = Armament {
        name: "calculator".to_string(),
        description: "Calculates".to_string(),
        parameters: json!({"type": "object"}),
        required_params: vec!["operation".to_string()],
    };
    registry.register(tool).await;

    let mut args = HashMap::new();
    args.insert("operation".to_string(), Value::String("add".to_string()));
    args.insert("a".to_string(), Value::Number(5.into()));
    args.insert("b".to_string(), Value::Number(3.into()));

    let call = ArmamentCall::new("calculator", args);

    // Basic validation should pass
    assert!(service.validate_call(&call).is_ok());
}

#[tokio::test]
async fn test_validate_call_empty_tool_name() {
    let registry = Arc::new(ArsenalRegistryService::new());
    let service = ArsenalExecutionService::new(registry);

    let args = HashMap::new();
    let call = ArmamentCall::new("", args);

    // Should fail with empty tool name
    let result = service.validate_call(&call);
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("cannot be empty"));
}

#[tokio::test]
async fn test_registry_count() {
    let registry = ArsenalRegistryService::new();
    assert_eq!(registry.count().await, 0);

    let tool1 = Armament {
        name: "tool1".to_string(),
        description: "First tool".to_string(),
        parameters: json!({}),
        required_params: vec![],
    };
    registry.register(tool1).await;
    assert_eq!(registry.count().await, 1);

    let tool2 = Armament {
        name: "tool2".to_string(),
        description: "Second tool".to_string(),
        parameters: json!({}),
        required_params: vec![],
    };
    registry.register(tool2).await;
    assert_eq!(registry.count().await, 2);
}

#[tokio::test]
async fn test_registry_clear() {
    let registry = ArsenalRegistryService::new();

    let tool = Armament {
        name: "tool".to_string(),
        description: "Test tool".to_string(),
        parameters: json!({}),
        required_params: vec![],
    };
    registry.register(tool).await;
    assert_eq!(registry.count().await, 1);

    registry.clear().await;
    assert_eq!(registry.count().await, 0);
}

#[tokio::test]
async fn test_execution_service_invoke_without_a_registered_mcp_client_is_tool_not_found() {
    // As of Phase 12.1 Plan 03, `ArsenalExecutionService::invoke` routes to
    // the real MCP client serving a tool via `register_client` — it no
    // longer fakes a successful result. A tool that is only present in the
    // registry (metadata) but has no serving client registered correctly
    // fails with `ToolNotFound`, not a simulated success.
    let registry = Arc::new(ArsenalRegistryService::new());
    let service = ArsenalExecutionService::new(registry.clone());

    let tool = Armament {
        name: "test_tool".to_string(),
        description: "Test tool".to_string(),
        parameters: json!({}),
        required_params: vec![],
    };
    registry.register(tool).await;

    let args = HashMap::new();
    let call = ArmamentCall::new("test_tool", args);

    let result = service.invoke(call.clone()).await;

    match result {
        Err(ArsenalError::ToolNotFound(name)) => assert_eq!(name, "test_tool"),
        other => panic!("expected ToolNotFound (no serving client registered), got: {other:?}"),
    }
}
