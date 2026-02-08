//! Integration tests for SSE MCP transport

use mockito::{Matcher, Server};
use paladin::infrastructure::adapters::arsenal::mcp_protocol::{MCPClient, MCPError, MCPResponse};
use paladin::infrastructure::adapters::arsenal::mcp_sse_adapter::MCPSseAdapter;
use serde_json::json;
use std::collections::HashMap;
use std::time::Duration;

#[tokio::test]
async fn test_sse_connect() {
    let mut server = Server::new_async().await;

    // Mock the health check endpoint
    let _mock = server
        .mock("GET", "/")
        .with_status(200)
        .with_body("OK")
        .create_async()
        .await;

    let mut adapter = MCPSseAdapter::new(server.url());

    // Should not be connected initially
    assert!(!adapter.is_connected());

    // Connect to the server
    let result = adapter.connect().await;
    assert!(result.is_ok(), "Failed to connect: {:?}", result.err());

    // Should be connected now
    assert!(adapter.is_connected());
}

#[tokio::test]
async fn test_sse_connection_failure() {
    // Use a non-existent server
    let mut adapter = MCPSseAdapter::new("http://localhost:9999");

    let result = adapter.connect().await;
    assert!(result.is_err(), "Expected connection to fail");

    if let Err(e) = result {
        let error_msg = e.to_string();
        assert!(
            error_msg.contains("Failed to connect") || error_msg.contains("connect"),
            "Expected connection error, got: {}",
            error_msg
        );
    }
}

#[tokio::test]
async fn test_sse_discover_tools() {
    let mut server = Server::new_async().await;

    // Mock health check
    let _health_mock = server
        .mock("GET", "/")
        .with_status(200)
        .create_async()
        .await;

    // Mock tools/list response
    let tools_response = MCPResponse {
        jsonrpc: "2.0".to_string(),
        id: serde_json::Value::String("test".to_string()),
        result: Some(json!({
            "tools": [
                {
                    "name": "weather",
                    "description": "Get weather information",
                    "inputSchema": {
                        "type": "object",
                        "properties": {
                            "location": {"type": "string"}
                        },
                        "required": ["location"]
                    }
                },
                {
                    "name": "calculator",
                    "description": "Perform calculations",
                    "inputSchema": {
                        "type": "object",
                        "properties": {
                            "expression": {"type": "string"}
                        },
                        "required": ["expression"]
                    }
                }
            ]
        })),
        error: None,
    };

    let _tools_mock = server
        .mock("POST", "/")
        .match_body(Matcher::Regex(".*tools/list.*".to_string()))
        .with_status(200)
        .with_body(serde_json::to_string(&tools_response).unwrap())
        .create_async()
        .await;

    let mut adapter = MCPSseAdapter::new(server.url());
    adapter.connect().await.expect("Failed to connect");

    let client = MCPClient::new(Box::new(adapter));
    let tools = client
        .discover_tools()
        .await
        .expect("Failed to discover tools");

    // Should have 2 tools
    assert_eq!(tools.len(), 2);

    // Check weather tool
    let weather_tool = tools.iter().find(|t| t.name == "weather");
    assert!(weather_tool.is_some(), "Weather tool not found");
    let weather = weather_tool.unwrap();
    assert_eq!(weather.description, "Get weather information");
    assert_eq!(weather.required_params, vec!["location"]);

    // Check calculator tool
    let calc_tool = tools.iter().find(|t| t.name == "calculator");
    assert!(calc_tool.is_some(), "Calculator tool not found");
}

#[tokio::test]
async fn test_sse_invoke_tool() {
    let mut server = Server::new_async().await;

    // Mock health check
    let _health_mock = server
        .mock("GET", "/")
        .with_status(200)
        .create_async()
        .await;

    // Mock tool invocation response
    let invoke_response = MCPResponse {
        jsonrpc: "2.0".to_string(),
        id: serde_json::Value::String("test".to_string()),
        result: Some(json!({
            "content": {
                "type": "text",
                "text": "The weather in London is sunny, 22°C"
            }
        })),
        error: None,
    };

    let _invoke_mock = server
        .mock("POST", "/")
        .match_body(Matcher::Regex(".*tools/call.*".to_string()))
        .with_status(200)
        .with_body(serde_json::to_string(&invoke_response).unwrap())
        .create_async()
        .await;

    let mut adapter = MCPSseAdapter::new(server.url());
    adapter.connect().await.expect("Failed to connect");

    let client = MCPClient::new(Box::new(adapter));

    // Invoke weather tool
    let mut args = HashMap::new();
    args.insert("location".to_string(), serde_json::json!("London"));

    let result = client
        .invoke_tool("weather", args)
        .await
        .expect("Failed to invoke weather tool");

    // Check result contains weather data
    let text = result
        .get("text")
        .and_then(|v| v.as_str())
        .expect("No text in result");
    assert!(text.contains("weather"));
    assert!(text.contains("London"));
}

#[tokio::test]
async fn test_sse_error_handling() {
    let mut server = Server::new_async().await;

    // Mock health check
    let _health_mock = server
        .mock("GET", "/")
        .with_status(200)
        .create_async()
        .await;

    // Mock error response
    let error_response = MCPResponse {
        jsonrpc: "2.0".to_string(),
        id: serde_json::Value::String("test".to_string()),
        result: None,
        error: Some(MCPError::new(MCPError::METHOD_NOT_FOUND, "Tool not found")),
    };

    let _error_mock = server
        .mock("POST", "/")
        .with_status(200)
        .with_body(serde_json::to_string(&error_response).unwrap())
        .create_async()
        .await;

    let mut adapter = MCPSseAdapter::new(server.url());
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
async fn test_sse_retry_on_failure() {
    let mut server = Server::new_async().await;

    // Mock health check
    let _health_mock = server
        .mock("GET", "/")
        .with_status(200)
        .create_async()
        .await;

    // First two requests fail, third succeeds
    let _fail_mock1 = server
        .mock("POST", "/")
        .with_status(500)
        .expect(1)
        .create_async()
        .await;

    let _fail_mock2 = server
        .mock("POST", "/")
        .with_status(503)
        .expect(1)
        .create_async()
        .await;

    let success_response = MCPResponse {
        jsonrpc: "2.0".to_string(),
        id: serde_json::Value::String("test".to_string()),
        result: Some(json!({"tools": []})),
        error: None,
    };

    let _success_mock = server
        .mock("POST", "/")
        .with_status(200)
        .with_body(serde_json::to_string(&success_response).unwrap())
        .expect(1)
        .create_async()
        .await;

    let mut adapter = MCPSseAdapter::new(server.url());
    adapter.connect().await.expect("Failed to connect");

    let client = MCPClient::new(Box::new(adapter));

    // This should succeed after retries
    let result = client.discover_tools().await;
    assert!(
        result.is_ok(),
        "Expected success after retries, got: {:?}",
        result.err()
    );
}

#[tokio::test]
async fn test_sse_connection_timeout() {
    let mut adapter =
        MCPSseAdapter::with_timeout("http://localhost:9999", Duration::from_millis(100));

    // Connection should timeout quickly
    let result = adapter.connect().await;
    assert!(result.is_err(), "Expected connection to timeout");
}

#[tokio::test]
async fn test_sse_multiple_calls() {
    let mut server = Server::new_async().await;

    // Mock health check
    let _health_mock = server
        .mock("GET", "/")
        .with_status(200)
        .create_async()
        .await;

    // Mock multiple successful calls
    for i in 1..=3 {
        let response = MCPResponse {
            jsonrpc: "2.0".to_string(),
            id: serde_json::Value::String("test".to_string()),
            result: Some(json!({
                "content": {
                    "type": "text",
                    "text": format!("Result {}", i)
                }
            })),
            error: None,
        };

        let _mock = server
            .mock("POST", "/")
            .with_status(200)
            .with_body(serde_json::to_string(&response).unwrap())
            .expect(1)
            .create_async()
            .await;
    }

    let mut adapter = MCPSseAdapter::new(server.url());
    adapter.connect().await.expect("Failed to connect");

    let client = MCPClient::new(Box::new(adapter));

    // Make multiple calls
    for i in 1..=3 {
        let mut args = HashMap::new();
        args.insert("input".to_string(), serde_json::json!(format!("test{}", i)));

        let result = client
            .invoke_tool("test_tool", args)
            .await
            .unwrap_or_else(|_| panic!("Failed on iteration {}", i));

        let text = result
            .get("text")
            .and_then(|v| v.as_str())
            .expect("No text in result");
        assert!(text.contains(&format!("Result {}", i)));
    }
}

#[tokio::test]
async fn test_sse_http_error() {
    let mut server = Server::new_async().await;

    // Mock health check
    let _health_mock = server
        .mock("GET", "/")
        .with_status(200)
        .create_async()
        .await;

    // Mock server error
    let _error_mock = server
        .mock("POST", "/")
        .with_status(500)
        .with_body("Internal Server Error")
        .create_async()
        .await;

    let mut adapter = MCPSseAdapter::new(server.url());
    adapter.connect().await.expect("Failed to connect");

    let client = MCPClient::new(Box::new(adapter));

    // This should fail after all retries
    let result = client.discover_tools().await;
    assert!(result.is_err(), "Expected error for server failure");
}
