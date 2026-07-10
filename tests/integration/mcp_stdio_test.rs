//! Integration tests for STDIO MCP transport (rmcp-backed, Phase 12.1)
//!
//! `MCPClient::connect_stdio` spawns the subprocess AND performs the full MCP
//! `initialize -> notifications/initialized` handshake before returning
//! (D-04) -- there is no separate "connect, then wrap" step anymore.

use paladin::infrastructure::adapters::arsenal::mcp_protocol::MCPClient;
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

/// SC1: `initialize -> notifications/initialized` completes against the
/// spec-strict test server before this call returns.
#[tokio::test]
async fn test_stdio_connect() {
    let server_path = get_test_server_path();
    let args = vec!["-u".to_string(), server_path.to_str().unwrap().to_string()];

    let client = MCPClient::connect_stdio("python3", &args).await;
    assert!(client.is_ok(), "Failed to connect: {:?}", client.err());
}

/// D-04: server capabilities negotiated during `initialize` are genuinely
/// populated (previously an always-`None`, dead field).
#[tokio::test]
async fn test_stdio_server_capabilities_populated_after_handshake() {
    let server_path = get_test_server_path();
    let args = vec!["-u".to_string(), server_path.to_str().unwrap().to_string()];

    let client = MCPClient::connect_stdio("python3", &args)
        .await
        .expect("Failed to connect");

    let capabilities = client
        .server_capabilities()
        .expect("server capabilities should be populated after a successful handshake");
    assert_eq!(capabilities.server_info.name, "paladin-mcp-test-server");
}

#[tokio::test]
async fn test_stdio_discover_tools() {
    let server_path = get_test_server_path();
    let args = vec!["-u".to_string(), server_path.to_str().unwrap().to_string()];

    let client = MCPClient::connect_stdio("python3", &args)
        .await
        .expect("Failed to connect");

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
    let args = vec!["-u".to_string(), server_path.to_str().unwrap().to_string()];

    let client = MCPClient::connect_stdio("python3", &args)
        .await
        .expect("Failed to connect");

    // Invoke echo tool
    let mut call_args = HashMap::new();
    call_args.insert("message".to_string(), serde_json::json!("Hello, MCP!"));

    let result = client
        .invoke_tool("echo", call_args)
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
    let args = vec!["-u".to_string(), server_path.to_str().unwrap().to_string()];

    let client = MCPClient::connect_stdio("python3", &args)
        .await
        .expect("Failed to connect");

    // Test addition
    let mut call_args = HashMap::new();
    call_args.insert("operation".to_string(), serde_json::json!("add"));
    call_args.insert("a".to_string(), serde_json::json!(5));
    call_args.insert("b".to_string(), serde_json::json!(3));

    let result = client
        .invoke_tool("calculator", call_args)
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
    let mut call_args = HashMap::new();
    call_args.insert("operation".to_string(), serde_json::json!("multiply"));
    call_args.insert("a".to_string(), serde_json::json!(4));
    call_args.insert("b".to_string(), serde_json::json!(7));

    let result = client
        .invoke_tool("calculator", call_args)
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
    let args = vec!["-u".to_string(), server_path.to_str().unwrap().to_string()];

    let client = MCPClient::connect_stdio("python3", &args)
        .await
        .expect("Failed to connect");

    // Try to invoke non-existent tool
    let call_args = HashMap::new();
    let result = client.invoke_tool("nonexistent_tool", call_args).await;

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
    let args = vec!["-u".to_string(), server_path.to_str().unwrap().to_string()];

    let client = MCPClient::connect_stdio("python3", &args)
        .await
        .expect("Failed to connect");

    // Drop the client -- rmcp's TokioChildProcess owns kill-on-drop process
    // cleanup (verified: ChildWithCleanup::drop kills the child process).
    drop(client);

    // If we get here without hanging, cleanup worked
    // (process was killed and didn't block on drop)
}

#[tokio::test]
async fn test_stdio_multiple_calls() {
    let server_path = get_test_server_path();
    let args = vec!["-u".to_string(), server_path.to_str().unwrap().to_string()];

    let client = MCPClient::connect_stdio("python3", &args)
        .await
        .expect("Failed to connect");

    // Make multiple calls to the same tool
    for i in 1..=5 {
        let mut call_args = HashMap::new();
        call_args.insert(
            "message".to_string(),
            serde_json::json!(format!("Message {}", i)),
        );

        let result = client
            .invoke_tool("echo", call_args)
            .await
            .unwrap_or_else(|_| panic!("Failed to invoke echo on iteration {}", i));

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
    let result = MCPClient::connect_stdio("nonexistent_command_xyz", &[]).await;
    assert!(result.is_err(), "Expected connection to fail");

    if let Err(e) = result {
        let error_msg = e.to_string();
        assert!(
            error_msg.contains("failed to spawn") || error_msg.contains("spawn"),
            "Expected spawn error, got: {}",
            error_msg
        );
    }
}

/// D-06 negative-path proof: talk to the mock server RAW over stdio,
/// bypassing rmcp's client-side sequencing entirely, and send a `tools/list`
/// request as the very first message (no `initialize` ever sent). The
/// spec-strict server must reject it with a JSON-RPC error -- proving the
/// server enforces handshake ordering rather than merely being lenient
/// enough that a correctly-sequenced client happens to pass (RESEARCH
/// Pitfall 2 / VALIDATION D-06).
#[tokio::test]
async fn test_stdio_server_rejects_tools_list_before_handshake() {
    use std::process::Stdio;
    use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};

    let server_path = get_test_server_path();
    let mut child = tokio::process::Command::new("python3")
        .arg("-u")
        .arg(&server_path)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .kill_on_drop(true)
        .spawn()
        .expect("failed to spawn mock MCP server");

    let mut stdin = child.stdin.take().expect("stdin");
    let stdout = child.stdout.take().expect("stdout");
    let mut reader = BufReader::new(stdout);

    // Send a raw tools/list request WITHOUT ever sending `initialize` first.
    let request = serde_json::json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "tools/list",
        "params": {}
    });
    stdin
        .write_all(format!("{}\n", request).as_bytes())
        .await
        .expect("write tools/list");
    stdin.flush().await.expect("flush");

    let mut line = String::new();
    reader
        .read_line(&mut line)
        .await
        .expect("read response from mock server");

    let response: serde_json::Value =
        serde_json::from_str(&line).expect("mock server response must be valid JSON");
    assert!(
        response.get("error").is_some(),
        "expected a JSON-RPC error for tools/list sent before the MCP handshake, got: {line}"
    );
}
