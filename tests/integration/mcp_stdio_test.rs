//! Integration tests for STDIO MCP transport

use paladin::infrastructure::adapters::arsenal::mcp_protocol::MCPClient;
use paladin::infrastructure::adapters::arsenal::mcp_stdio_adapter::MCPStdioAdapter;
use std::collections::HashMap;
use std::env;
use std::path::PathBuf;

/// Helper to get the path to the test server script
fn get_test_server_path() -> PathBuf {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set");
    PathBuf::from(manifest_dir)
        .join("tests")
        .join("mcp_test_server.py")
}

#[tokio::test]
async fn test_stdio_connect() {
    let server_path = get_test_server_path();
    let mut adapter = MCPStdioAdapter::new("python3", vec!["-u", server_path.to_str().unwrap()]);

    // Should not be connected initially
    assert!(!adapter.is_connected());

    // Connect to the server
    let result = adapter.connect().await;
    assert!(result.is_ok(), "Failed to connect: {:?}", result.err());

    // Should be connected now
    assert!(adapter.is_connected());
}

#[tokio::test]
async fn test_stdio_discover_tools() {
    let server_path = get_test_server_path();
    let mut adapter = MCPStdioAdapter::new("python3", vec!["-u", server_path.to_str().unwrap()]);

    adapter.connect().await.expect("Failed to connect");

    let client = MCPClient::new(Box::new(adapter));
    let tools = client
        .discover_tools()
        .await
        .expect("Failed to discover tools");

    // Should have 2 tools: echo and calculator
    assert_eq!(tools.len(), 2);

    // Check echo tool
    let echo_tool = tools.iter().find(|t| t.name == "echo");
    assert!(echo_tool.is_some(), "Echo tool not found");
    let echo = echo_tool.unwrap();
    assert_eq!(echo.description, "Echoes the input back");
    assert_eq!(echo.required_params, vec!["message"]);

    // Check calculator tool
    let calc_tool = tools.iter().find(|t| t.name == "calculator");
    assert!(calc_tool.is_some(), "Calculator tool not found");
    let calc = calc_tool.unwrap();
    assert_eq!(calc.description, "Performs basic arithmetic");
    assert_eq!(calc.required_params.len(), 3);
    assert!(calc.required_params.contains(&"operation".to_string()));
    assert!(calc.required_params.contains(&"a".to_string()));
    assert!(calc.required_params.contains(&"b".to_string()));
}

#[tokio::test]
async fn test_stdio_invoke_tool_echo() {
    let server_path = get_test_server_path();
    let mut adapter = MCPStdioAdapter::new("python3", vec!["-u", server_path.to_str().unwrap()]);

    adapter.connect().await.expect("Failed to connect");

    let client = MCPClient::new(Box::new(adapter));

    // Invoke echo tool
    let mut args = HashMap::new();
    args.insert("message".to_string(), serde_json::json!("Hello, MCP!"));

    let result = client
        .invoke_tool("echo", args)
        .await
        .expect("Failed to invoke echo tool");

    // Check result contains the echoed message
    let text = result
        .get("text")
        .and_then(|v| v.as_str())
        .expect("No text in result");
    assert!(text.contains("Hello, MCP!"));
}

#[tokio::test]
async fn test_stdio_invoke_tool_calculator() {
    let server_path = get_test_server_path();
    let mut adapter = MCPStdioAdapter::new("python3", vec!["-u", server_path.to_str().unwrap()]);

    adapter.connect().await.expect("Failed to connect");

    let client = MCPClient::new(Box::new(adapter));

    // Test addition
    let mut args = HashMap::new();
    args.insert("operation".to_string(), serde_json::json!("add"));
    args.insert("a".to_string(), serde_json::json!(5));
    args.insert("b".to_string(), serde_json::json!(3));

    let result = client
        .invoke_tool("calculator", args)
        .await
        .expect("Failed to invoke calculator");

    let text = result
        .get("text")
        .and_then(|v| v.as_str())
        .expect("No text in result");
    assert!(
        text.contains("8"),
        "Expected result to contain 8, got: {}",
        text
    );

    // Test multiplication
    let mut args = HashMap::new();
    args.insert("operation".to_string(), serde_json::json!("multiply"));
    args.insert("a".to_string(), serde_json::json!(4));
    args.insert("b".to_string(), serde_json::json!(7));

    let result = client
        .invoke_tool("calculator", args)
        .await
        .expect("Failed to invoke calculator");

    let text = result
        .get("text")
        .and_then(|v| v.as_str())
        .expect("No text in result");
    assert!(
        text.contains("28"),
        "Expected result to contain 28, got: {}",
        text
    );
}

#[tokio::test]
async fn test_stdio_error_handling() {
    let server_path = get_test_server_path();
    let mut adapter = MCPStdioAdapter::new("python3", vec!["-u", server_path.to_str().unwrap()]);

    adapter.connect().await.expect("Failed to connect");

    let client = MCPClient::new(Box::new(adapter));

    // Try to invoke non-existent tool
    let args = HashMap::new();
    let result = client.invoke_tool("nonexistent_tool", args).await;

    assert!(result.is_err(), "Expected error for non-existent tool");

    if let Err(e) = result {
        let error_msg = e.to_string();
        assert!(
            error_msg.contains("not found") || error_msg.contains("Tool not found"),
            "Expected 'not found' in error message, got: {}",
            error_msg
        );
    }
}

#[tokio::test]
async fn test_stdio_cleanup() {
    let server_path = get_test_server_path();
    let mut adapter = MCPStdioAdapter::new("python3", vec!["-u", server_path.to_str().unwrap()]);

    adapter.connect().await.expect("Failed to connect");
    assert!(adapter.is_connected());

    // Drop the adapter - should clean up process
    drop(adapter);

    // If we get here without hanging, cleanup worked
    // (process was killed and didn't block on drop)
}

#[tokio::test]
async fn test_stdio_multiple_calls() {
    let server_path = get_test_server_path();
    let mut adapter = MCPStdioAdapter::new("python3", vec!["-u", server_path.to_str().unwrap()]);

    adapter.connect().await.expect("Failed to connect");

    let client = MCPClient::new(Box::new(adapter));

    // Make multiple calls to the same tool
    for i in 1..=5 {
        let mut args = HashMap::new();
        args.insert(
            "message".to_string(),
            serde_json::json!(format!("Message {}", i)),
        );

        let result = client
            .invoke_tool("echo", args)
            .await
            .expect(&format!("Failed to invoke echo on iteration {}", i));

        let text = result
            .get("text")
            .and_then(|v| v.as_str())
            .expect("No text in result");
        assert!(text.contains(&format!("Message {}", i)));
    }
}

#[tokio::test]
async fn test_stdio_connection_failure() {
    // Try to connect to non-existent command
    let mut adapter = MCPStdioAdapter::new("nonexistent_command_xyz", Vec::<String>::new());

    let result = adapter.connect().await;
    assert!(result.is_err(), "Expected connection to fail");

    if let Err(e) = result {
        let error_msg = e.to_string();
        assert!(
            error_msg.contains("Failed to spawn") || error_msg.contains("spawn"),
            "Expected spawn error, got: {}",
            error_msg
        );
    }
}
