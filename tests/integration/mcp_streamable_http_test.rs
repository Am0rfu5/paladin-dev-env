//! Hermetic Streamable-HTTP round-trip test (Phase 12.1 Plan 04, SC2).
//!
//! Stands up a REAL in-process, spec-strict Streamable-HTTP MCP server using
//! axum + rmcp's own server-side `transport-streamable-http-server` feature
//! -- the identical pattern rmcp's own test suite
//! (`tests/test_streamable_http_*.rs`) uses to test its client against its
//! own server -- rather than a hand-mocked HTTP double. This is stronger
//! hermetic evidence than a mock: it exercises the real MCP
//! `initialize -> notifications/initialized` handshake, session semantics,
//! and JSON-RPC framing that `rmcp`'s `StreamableHttpService` implements.
//!
//! The fixture server REQUIRES a specific bearer token (enforced by an axum
//! middleware layer placed in FRONT of the MCP endpoint, returning 401 with a
//! `WWW-Authenticate` header for any missing/incorrect token) -- proving the
//! negative-path test that `MCPClient::connect_streamable_http` actually
//! sends the `Authorization` header, not merely that the happy path is
//! lenient (RESEARCH.md Pitfall 2 / VALIDATION D-06 / threat T-12.1-09).

use axum::Router;
use axum::extract::Request;
use axum::http::{StatusCode, header};
use axum::middleware::{self, Next};
use axum::response::{IntoResponse, Response};
use paladin::core::platform::container::arsenal::ArsenalError;
use paladin::infrastructure::adapters::arsenal::mcp_protocol::MCPClient;
use rmcp::model::{
    CallToolRequestParams, CallToolResult, ContentBlock, Implementation, ListToolsResult,
    PaginatedRequestParams, ServerCapabilities, ServerInfo, Tool,
};
use rmcp::service::RequestContext;
use rmcp::transport::streamable_http_server::{
    StreamableHttpServerConfig, StreamableHttpService, session::local::LocalSessionManager,
};
use rmcp::{ErrorData as McpError, RoleServer, ServerHandler};
use std::collections::HashMap;
use tokio_util::sync::CancellationToken;

/// The ONLY bearer token the fixture server accepts. Any other value (or no
/// `Authorization` header at all) must be rejected with 401.
const EXPECTED_BEARER_TOKEN: &str = "hermetic-fixture-bearer-token-12345";
const ECHO_TOOL_NAME: &str = "echo";

/// A minimal spec-strict MCP server exposing a single `echo` tool, built by
/// hand-implementing `ServerHandler` (no `#[tool_router]`/`#[tool]` macros --
/// this test enables only the `server` + `transport-streamable-http-server`
/// rmcp dev-features, not `macros`, keeping the dev-only feature surface
/// minimal).
#[derive(Clone, Default)]
struct FixtureServer;

impl ServerHandler for FixtureServer {
    fn get_info(&self) -> ServerInfo {
        ServerInfo::new(ServerCapabilities::builder().enable_tools().build())
            .with_server_info(Implementation::new(
                "paladin-streamable-http-fixture",
                "1.0.0",
            ))
            .with_instructions("Hermetic Streamable-HTTP fixture server (Phase 12.1 Plan 04)")
    }

    async fn list_tools(
        &self,
        _request: Option<PaginatedRequestParams>,
        _context: RequestContext<RoleServer>,
    ) -> Result<ListToolsResult, McpError> {
        Ok(ListToolsResult::with_all_items(vec![Tool::new(
            ECHO_TOOL_NAME,
            "Echoes the input message back",
            echo_input_schema(),
        )]))
    }

    async fn call_tool(
        &self,
        request: CallToolRequestParams,
        _context: RequestContext<RoleServer>,
    ) -> Result<CallToolResult, McpError> {
        if request.name != ECHO_TOOL_NAME {
            return Err(McpError::invalid_params(
                format!("unknown tool `{}`", request.name),
                None,
            ));
        }
        let message = request
            .arguments
            .as_ref()
            .and_then(|args| args.get("message"))
            .and_then(|v| v.as_str())
            .unwrap_or_default();
        Ok(CallToolResult::success(vec![ContentBlock::text(format!(
            "echo: {message}"
        ))]))
    }
}

/// The `echo` tool's JSON Schema input (a bare `serde_json::Map`, no
/// `schemars` derive needed for a hand-built `Tool`).
fn echo_input_schema() -> serde_json::Map<String, serde_json::Value> {
    let mut properties = serde_json::Map::new();
    properties.insert("message".to_string(), serde_json::json!({"type": "string"}));

    let mut schema = serde_json::Map::new();
    schema.insert("type".to_string(), serde_json::json!("object"));
    schema.insert(
        "properties".to_string(),
        serde_json::Value::Object(properties),
    );
    schema.insert("required".to_string(), serde_json::json!(["message"]));
    schema
}

/// Axum middleware enforcing bearer-token auth on every request reaching the
/// MCP endpoint -- placed IN FRONT of `StreamableHttpService` so a
/// missing/incorrect token never reaches rmcp's own request handling at all.
/// Returns a `WWW-Authenticate` header on rejection so rmcp's client-side
/// `StreamableHttpError::AuthRequired` path is triggered (proving the
/// negative test exercises the SAME error taxonomy a real OAuth-gated MCP
/// server would produce).
async fn require_bearer_token(request: Request, next: Next) -> Response {
    let authorized = request
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|value| value.to_str().ok())
        .map(|value| value == format!("Bearer {EXPECTED_BEARER_TOKEN}"))
        .unwrap_or(false);

    if !authorized {
        return (
            StatusCode::UNAUTHORIZED,
            [(header::WWW_AUTHENTICATE, "Bearer")],
            "missing or invalid bearer token",
        )
            .into_response();
    }

    next.run(request).await
}

/// Spawns the fixture server on an ephemeral localhost port and returns its
/// `/mcp` endpoint URI plus a `CancellationToken` the test uses to shut it
/// down cleanly.
async fn spawn_fixture_server() -> (String, CancellationToken) {
    let ct = CancellationToken::new();
    let config = StreamableHttpServerConfig::default()
        .with_sse_keep_alive(None)
        .with_cancellation_token(ct.child_token());

    let service: StreamableHttpService<FixtureServer, LocalSessionManager> =
        StreamableHttpService::new(|| Ok(FixtureServer), Default::default(), config);

    let router = Router::new()
        .nest_service("/mcp", service)
        .layer(middleware::from_fn(require_bearer_token));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
        .await
        .expect("bind an ephemeral localhost port");
    let addr = listener.local_addr().expect("resolve bound local addr");

    tokio::spawn({
        let ct = ct.clone();
        async move {
            let _ = axum::serve(listener, router)
                .with_graceful_shutdown(async move { ct.cancelled_owned().await })
                .await;
        }
    });

    (format!("http://{addr}/mcp"), ct)
}

/// SC2 (hermetic): a correct bearer token completes the full
/// `initialize -> notifications/initialized -> tools/list -> tools/call`
/// round-trip against the real in-process spec-strict server.
#[tokio::test]
async fn streamable_http_round_trip_with_correct_bearer_token_succeeds() {
    let (uri, ct) = spawn_fixture_server().await;

    let client = MCPClient::connect_streamable_http(&uri, Some(EXPECTED_BEARER_TOKEN), None)
        .await
        .expect("authenticated handshake must succeed against the strict fixture server");

    let capabilities = client
        .server_capabilities()
        .expect("server capabilities should be populated after a successful handshake");
    assert_eq!(
        capabilities.server_info.name,
        "paladin-streamable-http-fixture"
    );

    let tools = client
        .discover_tools()
        .await
        .expect("tools/list must succeed once authenticated");
    assert_eq!(tools.len(), 1, "expected exactly the fixture echo tool");
    assert_eq!(tools[0].name, ECHO_TOOL_NAME);

    let mut arguments = HashMap::new();
    arguments.insert(
        "message".to_string(),
        serde_json::json!("hello streamable-http"),
    );
    let result = client
        .invoke_tool(ECHO_TOOL_NAME, arguments)
        .await
        .expect("tools/call must succeed once authenticated");
    let text = result
        .get("text")
        .and_then(|v| v.as_str())
        .expect("echo result should carry text content");
    assert!(text.contains("hello streamable-http"));

    ct.cancel();
}

/// D-06 negative-path proof: connecting with NO bearer token at all must be
/// rejected -- proving the server enforces auth rather than merely being
/// lenient enough for a correctly-authenticated client to happen to pass.
#[tokio::test]
async fn streamable_http_round_trip_rejects_missing_bearer_token() {
    let (uri, ct) = spawn_fixture_server().await;

    let result = MCPClient::connect_streamable_http(&uri, None, None).await;

    match result {
        Err(ArsenalError::AuthFailed(_)) => {}
        Err(other) => panic!("expected ArsenalError::AuthFailed, got a different error: {other}"),
        Ok(_) => panic!(
            "expected the connection to be rejected without a bearer token, but it succeeded"
        ),
    }

    ct.cancel();
}

/// D-06 negative-path proof, second half: an INCORRECT bearer token must
/// also be rejected -- proving the server actually validates the token
/// value, not merely its presence.
#[tokio::test]
async fn streamable_http_round_trip_rejects_incorrect_bearer_token() {
    let (uri, ct) = spawn_fixture_server().await;

    let result = MCPClient::connect_streamable_http(&uri, Some("wrong-token"), None).await;

    match result {
        Err(ArsenalError::AuthFailed(_)) => {}
        Err(other) => panic!("expected ArsenalError::AuthFailed, got a different error: {other}"),
        Ok(_) => panic!(
            "expected the connection to be rejected with an incorrect bearer token, but it succeeded"
        ),
    }

    ct.cancel();
}
