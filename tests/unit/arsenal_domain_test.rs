//! Unit tests for Arsenal domain entities

use paladin::core::platform::container::arsenal::{
    Armament, ArmamentCall, ArmamentResult, ArsenalError,
};
use serde_json::{Value, json};
use std::collections::HashMap;
use uuid::Uuid;

#[test]
fn test_armament_serialization() {
    let armament = Armament {
        name: "web_search".to_string(),
        description: "Search the web for information".to_string(),
        parameters: json!({
            "type": "object",
            "properties": {
                "query": {
                    "type": "string",
                    "description": "Search query"
                }
            }
        }),
        required_params: vec!["query".to_string()],
    };

    // Test serialization
    let json_str = serde_json::to_string(&armament).expect("Failed to serialize");
    assert!(json_str.contains("web_search"));
    assert!(json_str.contains("Search the web"));

    // Test deserialization
    let deserialized: Armament = serde_json::from_str(&json_str).expect("Failed to deserialize");
    assert_eq!(deserialized, armament);
}

#[test]
fn test_armament_call_creation() {
    let mut args = HashMap::new();
    args.insert(
        "query".to_string(),
        Value::String("rust programming".to_string()),
    );
    args.insert("limit".to_string(), Value::Number(10.into()));

    let call = ArmamentCall::new("web_search", args.clone());

    assert_eq!(call.tool_name, "web_search");
    assert_eq!(call.arguments.len(), 2);
    assert_eq!(
        call.arguments.get("query"),
        Some(&Value::String("rust programming".to_string()))
    );
    assert_eq!(call.arguments.get("limit"), Some(&Value::Number(10.into())));
    // UUID should be valid (not nil)
    assert_ne!(call.call_id, Uuid::nil());
}

#[test]
fn test_armament_result_success() {
    let call_id = Uuid::new_v4();
    let output = json!({
        "results": [
            "Result 1",
            "Result 2"
        ]
    });

    let result = ArmamentResult::success(call_id, output.clone(), 150);

    assert_eq!(result.call_id, call_id);
    assert!(result.success);
    assert_eq!(result.output, Some(output));
    assert_eq!(result.error, None);
    assert_eq!(result.execution_time_ms, 150);
}

#[test]
fn test_armament_result_failure() {
    let call_id = Uuid::new_v4();
    let error_msg = "Connection timeout";

    let result = ArmamentResult::failure(call_id, error_msg, 5000);

    assert_eq!(result.call_id, call_id);
    assert!(!result.success);
    assert_eq!(result.output, None);
    assert_eq!(result.error, Some(error_msg.to_string()));
    assert_eq!(result.execution_time_ms, 5000);
}

#[test]
fn test_arsenal_error_display() {
    let err1 = ArsenalError::ToolNotFound("calculator".to_string());
    assert_eq!(err1.to_string(), "Tool not found: calculator");

    let err2 = ArsenalError::InvalidArguments("Missing required parameter: query".to_string());
    assert_eq!(
        err2.to_string(),
        "Invalid tool arguments: Missing required parameter: query"
    );

    let err3 = ArsenalError::Timeout(30);
    assert_eq!(err3.to_string(), "Tool execution timeout after 30 seconds");

    let err4 = ArsenalError::ProtocolError("Invalid JSON-RPC response".to_string());
    assert_eq!(
        err4.to_string(),
        "MCP protocol error: Invalid JSON-RPC response"
    );

    let err5 = ArsenalError::TransportError("Connection refused".to_string());
    assert_eq!(err5.to_string(), "Transport error: Connection refused");
}
