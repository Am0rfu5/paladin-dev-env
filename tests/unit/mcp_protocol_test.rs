//! Unit tests for the rmcp-backed MCP client facade (Phase 12.1 D-01/D-04)
//!
//! The hand-rolled JSON-RPC types (`MCPMessage`/`MCPRequest`/`MCPResponse`/
//! `MCPNotification`/`MCPTransport`/`ToolInfo`/`MCPCapabilities`) are retired;
//! `MCPClient` is now a thin facade over `rmcp::service::RunningService`.
//! These tests exercise `connect_stdio`'s handshake + capability negotiation
//! and the `discover_tools`/`invoke_tool` surface against the real spec-strict
//! mock server (the same one `tests/integration/mcp_stdio_test.rs` uses).

use paladin::core::platform::container::arsenal::ArsenalError;
use paladin::infrastructure::adapters::arsenal::mcp_protocol::MCPClient;
use std::collections::HashMap;
use std::env;
use std::path::{Path, PathBuf};

fn get_test_server_path() -> PathBuf {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set");
    PathBuf::from(manifest_dir)
        .join("tests")
        .join("mcp_test_server.py")
}

fn stdio_args(server_path: &Path) -> Vec<String> {
    vec!["-u".to_string(), server_path.to_str().unwrap().to_string()]
}

/// D-04: `connect_stdio` performs the full handshake before returning, and
/// the negotiated server capabilities are genuinely retrievable afterward
/// (previously an always-`None`, `#[allow(dead_code)]` field).
#[tokio::test]
async fn connect_stdio_populates_server_capabilities() {
    let server_path = get_test_server_path();
    let client = MCPClient::connect_stdio("python3", &stdio_args(&server_path))
        .await
        .expect("handshake should succeed against the spec-strict mock server");

    let capabilities = client
        .server_capabilities()
        .expect("capabilities should be populated after a successful handshake");
    assert_eq!(capabilities.server_info.name, "paladin-mcp-test-server");
    assert_eq!(capabilities.server_info.version, "0.1.0");
}

/// A missing launcher binary fails loud with `ArsenalError::TransportError`,
/// never panicking (T-12.1-01).
#[tokio::test]
async fn connect_stdio_missing_binary_fails_loud() {
    let result = MCPClient::connect_stdio("definitely-not-a-real-mcp-launcher-xyz", &[]).await;

    match result {
        Err(ArsenalError::TransportError(msg)) => {
            assert!(msg.contains("failed to spawn"), "got: {msg}");
        }
        Err(other) => panic!("expected TransportError, got: {other}"),
        Ok(_) => panic!("expected connecting to a nonexistent binary to fail"),
    }
}

#[tokio::test]
async fn discover_tools_returns_expected_tool_set() {
    let server_path = get_test_server_path();
    let client = MCPClient::connect_stdio("python3", &stdio_args(&server_path))
        .await
        .expect("connect");

    let tools = client.discover_tools().await.expect("discover_tools");
    assert_eq!(tools.len(), 2);
    assert!(tools.iter().any(|t| t.name == "echo"));
    assert!(tools.iter().any(|t| t.name == "calculator"));
}

#[tokio::test]
async fn invoke_tool_unknown_tool_fails_loud() {
    let server_path = get_test_server_path();
    let client = MCPClient::connect_stdio("python3", &stdio_args(&server_path))
        .await
        .expect("connect");

    let result = client.invoke_tool("nonexistent", HashMap::new()).await;
    assert!(result.is_err());
    match result {
        Err(ArsenalError::ProtocolError(msg)) => {
            assert!(
                msg.contains("not found") || msg.contains("Tool not found"),
                "got: {msg}"
            );
        }
        other => panic!("expected ProtocolError, got {other:?}"),
    }
}
