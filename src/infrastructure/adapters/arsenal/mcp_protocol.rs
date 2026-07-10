//! Model Context Protocol (MCP) client facade over the official `rmcp` SDK.
//!
//! # Engine Swap (Phase 12.1 D-01/D-04)
//!
//! This module used to hand-roll the MCP JSON-RPC 2.0 wire protocol
//! (`MCPMessage`/`MCPRequest`/`MCPResponse`/`MCPNotification`/`MCPCapabilities`
//! /`ServerInfo`/`ToolInfo`/`MCPTransport`) and never performed the spec
//! `initialize -> notifications/initialized` handshake before issuing
//! `tools/list`/`tools/call` requests — the #1 correctness gap that blocked
//! spec-strict or hosted MCP servers.
//!
//! `MCPClient` is now a thin facade over `rmcp::service::RunningService`.
//! `rmcp::ServiceExt::serve()` performs the ENTIRE MCP handshake internally
//! for every transport it supports — the hand-rolled engine and the
//! `MCPTransport` trait are retired entirely (superseded by `rmcp::model::*`
//! and rmcp's own transport abstraction).
//!
//! # Example
//!
//! ```no_run
//! use paladin::infrastructure::adapters::arsenal::mcp_protocol::MCPClient;
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let client = MCPClient::connect_stdio("python3", &["server.py".to_string()]).await?;
//! let tools = client.discover_tools().await?;
//! println!("Available tools: {:?}", tools);
//! # Ok(())
//! # }
//! ```

use crate::core::platform::container::arsenal::{Armament, ArsenalError};
use http::{HeaderName, HeaderValue};
use rmcp::model::{CallToolRequestParams, CallToolResult, ClientInfo, InitializeResult, Tool};
use rmcp::service::{ClientInitializeError, RunningService};
use rmcp::transport::streamable_http_client::{
    StreamableHttpClientTransport, StreamableHttpClientTransportConfig,
};
use rmcp::transport::{ConfigureCommandExt, TokioChildProcess};
use rmcp::{RoleClient, ServiceError, ServiceExt};
use serde_json::Value;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use tokio::process::Command;
use tokio::time::timeout;

/// Bounded timeout wrapped around the ENTIRE Streamable-HTTP connect +
/// `initialize -> notifications/initialized` handshake (T-12.1-06) — mirrors
/// `crates/audit-mcp/src/etherscan.rs`'s spawn+handshake timeout wrap. A
/// hung/unreachable remote server must fold to a typed `ArsenalError::Timeout`,
/// never hang the caller indefinitely.
const STREAMABLE_HTTP_HANDSHAKE_TIMEOUT: Duration = Duration::from_secs(30);

/// MCP client for interacting with tool servers, backed by the official
/// `rmcp` SDK.
///
/// Wraps an already-`serve()`d rmcp peer/service. Connection, the full
/// `initialize -> notifications/initialized` handshake, and capability
/// negotiation all happen inside the `connect_*` constructors via
/// `rmcp::ServiceExt::serve()` — never hand-rolled here (D-04).
#[doc(hidden)]
pub struct MCPClient {
    /// The running rmcp client-role service (post-handshake).
    running: RunningService<RoleClient, ClientInfo>,
}

impl MCPClient {
    /// Spawns `command` with `args` as a subprocess and performs the full MCP
    /// stdio handshake: `initialize -> notifications/initialized` (D-04).
    ///
    /// # Errors
    ///
    /// Returns `ArsenalError::TransportError` if the subprocess fails to
    /// spawn, or `ArsenalError::ProtocolError` if the MCP handshake fails.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use paladin::infrastructure::adapters::arsenal::mcp_protocol::MCPClient;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = MCPClient::connect_stdio("python3", &["server.py".to_string()]).await?;
    /// let _ = client.server_capabilities();
    /// # Ok(())
    /// # }
    /// ```
    pub async fn connect_stdio(command: &str, args: &[String]) -> Result<Self, ArsenalError> {
        let owned_args = args.to_vec();
        let transport = TokioChildProcess::new(Command::new(command).configure(|cmd| {
            cmd.args(&owned_args);
        }))
        .map_err(|e| ArsenalError::TransportError(format!("failed to spawn MCP server: {e}")))?;

        let running = ClientInfo::default()
            .serve(transport)
            .await
            .map_err(|e| ArsenalError::ProtocolError(format!("MCP handshake failed: {e}")))?;

        Ok(Self { running })
    }

    /// Connects to a remote Streamable-HTTP MCP server (D-02 remote half),
    /// authenticating via an optional bearer token (D-03) and/or custom
    /// headers, then performs the same full `initialize ->
    /// notifications/initialized` handshake (D-04) as [`Self::connect_stdio`]
    /// — just over a different transport. The entire connect+handshake is
    /// wrapped in a bounded timeout (T-12.1-06).
    ///
    /// `bearer_token`, if provided, must NOT include a `"Bearer "` prefix —
    /// rmcp's `auth_header()` adds it internally via reqwest's
    /// `.bearer_auth()`. Passing an already-prefixed token double-prefixes
    /// and breaks auth (RESEARCH.md Anti-Patterns).
    ///
    /// # Errors
    ///
    /// - `ArsenalError::AuthFailed` if the server rejects the connection with
    ///   an auth/credential-shaped error (401/403).
    /// - `ArsenalError::Timeout` if connect+handshake exceeds the bounded
    ///   timeout.
    /// - `ArsenalError::ProtocolError`/`ArsenalError::TransportError` for any
    ///   other handshake/transport fault.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use paladin::infrastructure::adapters::arsenal::mcp_protocol::MCPClient;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = MCPClient::connect_streamable_http(
    ///     "https://mcp.example.com/mcp",
    ///     Some("my-bearer-token"),
    ///     None,
    /// )
    /// .await?;
    /// let _ = client.server_capabilities();
    /// # Ok(())
    /// # }
    /// ```
    pub async fn connect_streamable_http(
        uri: &str,
        bearer_token: Option<&str>,
        custom_headers: Option<HashMap<HeaderName, HeaderValue>>,
    ) -> Result<Self, ArsenalError> {
        let config = build_streamable_http_config(uri, bearer_token, custom_headers);

        let served = timeout(STREAMABLE_HTTP_HANDSHAKE_TIMEOUT, async {
            // `reqwest_mcp` is an aliased dependency on rmcp's OWN pinned
            // reqwest major version (0.13.x) — see Cargo.toml comment; this
            // crate's plain `reqwest` name is a different (0.12.x) instance
            // and does NOT implement rmcp's `StreamableHttpClient` trait.
            let transport =
                StreamableHttpClientTransport::<reqwest_mcp::Client>::from_config(config);
            ClientInfo::default().serve(transport).await
        })
        .await;

        match served {
            Ok(Ok(running)) => Ok(Self { running }),
            Ok(Err(e)) => Err(map_streamable_http_connect_error(e)),
            Err(_elapsed) => Err(ArsenalError::Timeout(
                STREAMABLE_HTTP_HANDSHAKE_TIMEOUT.as_secs(),
            )),
        }
    }

    /// Server capabilities negotiated during `initialize` (D-04) — the peer
    /// info rmcp captured during the handshake performed by `connect_stdio`.
    /// Previously an always-`None`, `#[allow(dead_code)]` field in the
    /// hand-rolled engine; now genuinely populated.
    pub fn server_capabilities(&self) -> Option<Arc<InitializeResult>> {
        self.running.peer_info()
    }

    /// Discovers available tools from the server via `tools/list`.
    ///
    /// Uses rmcp's `list_all_tools()`, which pages through `tools/list`
    /// automatically until the server reports no further cursor.
    ///
    /// # Errors
    ///
    /// Returns `ArsenalError` if communication fails or the server rejects
    /// the request.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use paladin::infrastructure::adapters::arsenal::mcp_protocol::MCPClient;
    /// # async fn example(client: MCPClient) -> Result<(), Box<dyn std::error::Error>> {
    /// let tools = client.discover_tools().await?;
    /// for tool in tools {
    ///     println!("Found tool: {}", tool.name);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn discover_tools(&self) -> Result<Vec<Armament>, ArsenalError> {
        let tools = self
            .running
            .list_all_tools()
            .await
            .map_err(map_service_error)?;

        Ok(tools.into_iter().map(tool_to_armament).collect())
    }

    /// Invokes a tool on the server via `tools/call`.
    ///
    /// # Errors
    ///
    /// Returns `ArsenalError` if communication fails, the server reports a
    /// tool-execution error, or the result carries no usable content.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use paladin::infrastructure::adapters::arsenal::mcp_protocol::MCPClient;
    /// # use std::collections::HashMap;
    /// # use serde_json::Value;
    /// # async fn example(client: MCPClient) -> Result<(), Box<dyn std::error::Error>> {
    /// let mut args = HashMap::new();
    /// args.insert("query".to_string(), Value::String("Rust".to_string()));
    /// let result = client.invoke_tool("web_search", args).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn invoke_tool(
        &self,
        tool_name: &str,
        arguments: HashMap<String, Value>,
    ) -> Result<Value, ArsenalError> {
        let params = CallToolRequestParams::new(tool_name.to_string())
            .with_arguments(serde_json::Map::from_iter(arguments));

        let result = self
            .running
            .call_tool(params)
            .await
            .map_err(map_service_error)?;

        if result.is_error.unwrap_or(false) {
            return Err(map_tool_result_error(&result));
        }

        extract_tool_result_value(&result)
    }
}

/// Convert an rmcp `Tool` (from `tools/list`) into Paladin's domain
/// `Armament`, extracting `required` parameter names from the JSON Schema
/// (mirrors the extraction the hand-rolled engine used to do from `ToolInfo`).
fn tool_to_armament(tool: Tool) -> Armament {
    let parameters = Value::Object((*tool.input_schema).clone());
    let required_params = parameters
        .get("required")
        .and_then(|r| r.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|v| v.as_str().map(String::from))
                .collect()
        })
        .unwrap_or_default();

    Armament {
        name: tool.name.to_string(),
        description: tool.description.map(|d| d.to_string()).unwrap_or_default(),
        parameters,
        required_params,
    }
}

/// Pure config builder for the Streamable-HTTP transport — separated from
/// [`MCPClient::connect_streamable_http`] so unit tests can assert
/// header-wiring behavior (no manual `"Bearer "` prefix, custom headers
/// applied) WITHOUT making a real network connection.
fn build_streamable_http_config(
    uri: &str,
    bearer_token: Option<&str>,
    custom_headers: Option<HashMap<HeaderName, HeaderValue>>,
) -> StreamableHttpClientTransportConfig {
    let mut config = StreamableHttpClientTransportConfig::with_uri(uri.to_string());
    if let Some(token) = bearer_token {
        // Deliberately NOT prefixed with "Bearer " -- rmcp's auth_header()
        // sends it via reqwest's .bearer_auth(), which adds the prefix
        // itself (RESEARCH.md Anti-Patterns).
        config = config.auth_header(token);
    }
    if let Some(headers) = custom_headers {
        config = config.custom_headers(headers);
    }
    config
}

/// Map an rmcp client-initialize error (from `.serve()` on the
/// Streamable-HTTP transport) to a fail-loud `ArsenalError`, distinguishing
/// an auth/credential rejection (401/403-shaped) from a general
/// transport/protocol fault (T-12.1-06/D-03).
fn map_streamable_http_connect_error(e: ClientInitializeError) -> ArsenalError {
    let msg = e.to_string();
    if looks_like_auth_failure(&msg) {
        ArsenalError::AuthFailed(msg)
    } else {
        ArsenalError::ProtocolError(format!("MCP handshake failed: {msg}"))
    }
}

/// Heuristic: does an error string indicate a credential/authorization
/// rejection (vs. a general transport/reachability fault)? Mirrors
/// `crates/audit-mcp/src/etherscan.rs::looks_like_auth_failure` — reuse the
/// same heuristic rather than inventing a second one in this workspace.
fn looks_like_auth_failure(msg: &str) -> bool {
    let lower = msg.to_lowercase();
    lower.contains("401")
        || lower.contains("403")
        || lower.contains("unauthorized")
        || lower.contains("forbidden")
        || lower.contains("auth")
        || lower.contains("api key")
        || lower.contains("apikey")
}

/// Map an rmcp transport/service error to a fail-loud `ArsenalError`, never
/// panicking on an unexpected variant (T-12.1-01).
fn map_service_error(e: ServiceError) -> ArsenalError {
    match e {
        ServiceError::Timeout { timeout } => ArsenalError::Timeout(timeout.as_secs()),
        ServiceError::TransportSend(err) => ArsenalError::TransportError(err.to_string()),
        ServiceError::TransportClosed => {
            ArsenalError::TransportError("MCP transport closed".to_string())
        }
        other => ArsenalError::ProtocolError(other.to_string()),
    }
}

/// Map a tool result flagged `is_error` to a fail-loud `ArsenalError`,
/// surfacing the server's own error text.
fn map_tool_result_error(result: &CallToolResult) -> ArsenalError {
    let text = result
        .content
        .iter()
        .find_map(|block| block.as_text().map(|t| t.text.clone()))
        .unwrap_or_else(|| "MCP tool reported an error".to_string());
    ArsenalError::ProtocolError(text)
}

/// Extract a usable `Value` from a successful tool result: prefer structured
/// content, else surface the first text content block in the same
/// `{"type": "text", "text": ...}` shape the hand-rolled engine returned
/// (keeps existing callers' `result.get("text")`-style access unchanged).
fn extract_tool_result_value(result: &CallToolResult) -> Result<Value, ArsenalError> {
    if let Some(structured) = &result.structured_content {
        return Ok(structured.clone());
    }

    let text = result
        .content
        .iter()
        .find_map(|block| block.as_text().map(|t| t.text.clone()));

    match text {
        Some(text) => Ok(serde_json::json!({ "type": "text", "text": text })),
        None => Err(ArsenalError::ProtocolError(
            "MCP tool result contained no text or structured content".to_string(),
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rmcp::model::ContentBlock;

    #[test]
    fn tool_to_armament_extracts_required_params() {
        let schema = serde_json::json!({
            "type": "object",
            "properties": {"operation": {"type": "string"}},
            "required": ["operation"]
        })
        .as_object()
        .expect("object")
        .clone();
        let tool = Tool::new("calculator", "Basic calculator", schema);

        let armament = tool_to_armament(tool);
        assert_eq!(armament.name, "calculator");
        assert_eq!(armament.description, "Basic calculator");
        assert_eq!(armament.required_params, vec!["operation".to_string()]);
    }

    #[test]
    fn tool_to_armament_defaults_missing_description() {
        let schema = serde_json::json!({"type": "object"})
            .as_object()
            .expect("object")
            .clone();
        let tool = Tool::new_with_raw("no_desc", None, schema);

        let armament = tool_to_armament(tool);
        assert_eq!(armament.description, "");
        assert!(armament.required_params.is_empty());
    }

    #[test]
    fn extract_tool_result_value_prefers_structured_content() {
        let result = CallToolResult::structured(serde_json::json!({"answer": 42}));
        let value = extract_tool_result_value(&result).expect("value");
        assert_eq!(value, serde_json::json!({"answer": 42}));
    }

    #[test]
    fn extract_tool_result_value_falls_back_to_text_block() {
        let result = CallToolResult::success(vec![ContentBlock::text("Echo: hi")]);
        let value = extract_tool_result_value(&result).expect("value");
        assert_eq!(value["text"], "Echo: hi");
    }

    #[test]
    fn extract_tool_result_value_fails_loud_when_empty() {
        let result = CallToolResult::success(vec![]);
        assert!(extract_tool_result_value(&result).is_err());
    }

    #[test]
    fn map_tool_result_error_surfaces_server_text() {
        let result = CallToolResult::error(vec![ContentBlock::text("no rows matched")]);
        let err = map_tool_result_error(&result);
        assert!(err.to_string().contains("no rows matched"));
    }

    // --- Streamable-HTTP transport (D-02/D-03/T-12.1-06) ---

    #[test]
    fn build_streamable_http_config_sends_bearer_token_without_manual_prefix() {
        // rmcp's auth_header() adds the "Bearer " prefix internally via
        // reqwest's .bearer_auth() -- passing an already-prefixed token would
        // double-prefix and break auth (RESEARCH.md Anti-Patterns).
        let config =
            build_streamable_http_config("http://localhost:1234/mcp", Some("raw-token-abc"), None);
        assert_eq!(config.auth_header.as_deref(), Some("raw-token-abc"));
    }

    #[test]
    fn build_streamable_http_config_omits_auth_header_when_no_token_given() {
        let config = build_streamable_http_config("http://localhost:1234/mcp", None, None);
        assert!(config.auth_header.is_none());
    }

    #[test]
    fn build_streamable_http_config_applies_custom_headers() {
        let mut headers = HashMap::new();
        headers.insert(
            HeaderName::from_static("x-custom-header"),
            HeaderValue::from_static("custom-value"),
        );
        let config =
            build_streamable_http_config("http://localhost:1234/mcp", None, Some(headers.clone()));
        assert_eq!(config.custom_headers, headers);
    }

    #[test]
    fn looks_like_auth_failure_detects_401_403_and_auth_keywords() {
        assert!(looks_like_auth_failure(
            "HTTP 401 Unauthorized: token invalid"
        ));
        assert!(looks_like_auth_failure("HTTP 403 Forbidden"));
        assert!(looks_like_auth_failure("missing api key"));
        assert!(!looks_like_auth_failure("connection refused"));
        assert!(!looks_like_auth_failure("request timed out"));
    }

    #[tokio::test]
    async fn connect_streamable_http_maps_401_to_auth_failed() {
        let mut server = mockito::Server::new_async().await;
        let _m = server
            .mock("POST", "/mcp")
            .with_status(401)
            .create_async()
            .await;
        let uri = format!("{}/mcp", server.url());

        let result = MCPClient::connect_streamable_http(&uri, Some("bad-token"), None).await;

        match result {
            Err(ArsenalError::AuthFailed(msg)) => {
                assert!(msg.contains("401"), "expected '401' in error text: {msg}");
            }
            Err(other) => panic!("expected AuthFailed, got a different ArsenalError: {other}"),
            Ok(_) => panic!("expected connect to fail against a 401-only mock server"),
        }
    }

    #[tokio::test]
    async fn connect_streamable_http_folds_unreachable_endpoint_to_typed_error_not_panic() {
        // Port 1 is reserved/unassigned -- connection is refused immediately,
        // so this proves the failure path never panics without waiting out
        // the full handshake timeout.
        let result = MCPClient::connect_streamable_http("http://127.0.0.1:1/mcp", None, None).await;
        assert!(result.is_err());
    }
}
