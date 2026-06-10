//! Runnable example: boot the Paladin HTTP API in-process and call an agent.
//!
//! Hermetic — backed by [`MockLlmAdapter`], so it needs no network or provider keys and runs
//! with `cargo run --example http_service_host --features web-server`.
//!
//! It assembles the app exactly as the `paladin-server` binary does (agent router under
//! `/v1`, OpenAPI docs, cross-cutting layers, auth enabled with a sample key), serves it on an
//! ephemeral port, then drives it over real HTTP: lists agents, runs one buffered and one
//! streamed execution, and reads the OpenAPI title — printing each result.

use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;

use paladin::MockLlmAdapter;
use paladin::application::services::paladin::paladin_builder::PaladinBuilder;
use paladin::application::services::paladin::paladin_execution_service::PaladinExecutionService;
use paladin::infrastructure::resilience::circuit_breaker::CircuitBreaker;
use paladin::infrastructure::web::openapi::{build_openapi, docs_router};
use paladin::infrastructure::web::{
    AgentApiState, AgentAuthConfig, AgentRegistry, HttpLayersConfig, Principal, agent_router,
    with_http_layers,
};
use paladin_core::platform::container::user::UserRole;
use paladin_ports::output::llm_port::LlmPort;
use paladin_ports::output::paladin_executor_port::PaladinExecutorPort;
use paladin_ports::output::streaming_executor_port::StreamingExecutorPort;

const API_KEY: &str = "sk-example-admin";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Build a hermetic, streaming-capable agent backed by a mock LLM.
    let llm: Arc<dyn LlmPort> = Arc::new(MockLlmAdapter::new());
    let breaker = Arc::new(CircuitBreaker::new(5, 2, Duration::from_secs(30)));
    let service = Arc::new(PaladinExecutionService::new(
        Arc::clone(&llm),
        breaker,
        None,
        None,
    ));
    let executor: Arc<dyn PaladinExecutorPort> = service.clone();
    let streamer: Arc<dyn StreamingExecutorPort> = service;
    let paladin = PaladinBuilder::new(llm)
        .name("researcher")
        .system_prompt("You research topics thoroughly.")
        .model("mock")
        .build()
        .await?;

    let registry = AgentRegistry::new();
    registry.insert_with_streaming("researcher", Arc::new(paladin), executor, Some(streamer));

    // 2. Enable auth with a single admin API key (mirrors the server's secure default).
    let mut api_keys = HashMap::new();
    api_keys.insert(
        API_KEY.to_string(),
        Principal {
            id: "example".to_string(),
            role: UserRole::Admin,
        },
    );
    let state = AgentApiState::new(Arc::new(registry)).with_auth(AgentAuthConfig {
        enabled: true,
        api_keys,
        jwt: None,
    });

    // 3. Assemble the app like `paladin-server`: /v1 routes + docs + cross-cutting layers.
    let spec = build_openapi(state.clone());
    let app = with_http_layers(
        agent_router(state).merge(docs_router(spec)),
        &HttpLayersConfig::default(),
    );

    // 4. Serve on an ephemeral port with graceful shutdown.
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await?;
    let addr = listener.local_addr()?;
    let (shutdown_tx, shutdown_rx) = tokio::sync::oneshot::channel::<()>();
    let server = tokio::spawn(async move {
        let _ = axum::serve(listener, app)
            .with_graceful_shutdown(async move {
                let _ = shutdown_rx.await;
            })
            .await;
    });

    let base = format!("http://{addr}");
    let client = reqwest::Client::new();
    println!("paladin-server (example) listening on {base}");

    // Discovery (authenticated).
    let agents: serde_json::Value = client
        .get(format!("{base}/v1/agents"))
        .header("x-api-key", API_KEY)
        .send()
        .await?
        .json()
        .await?;
    println!("GET /v1/agents -> {agents}");

    // Buffered execution.
    let result: serde_json::Value = client
        .post(format!("{base}/v1/agents/researcher/execute"))
        .header("x-api-key", API_KEY)
        .json(&serde_json::json!({ "input": "Tell me about Rust." }))
        .send()
        .await?
        .json()
        .await?;
    println!("POST …/execute -> output: {}", result["output"]);

    // Streaming execution (SSE).
    let stream = client
        .post(format!("{base}/v1/agents/researcher/execute/stream"))
        .header("x-api-key", API_KEY)
        .json(&serde_json::json!({ "input": "Stream it." }))
        .send()
        .await?
        .text()
        .await?;
    let chunks = stream.matches("event: chunk").count();
    println!("POST …/execute/stream -> {chunks} chunk event(s)");

    // OpenAPI title (no credential required).
    let spec: serde_json::Value = client
        .get(format!("{base}/openapi.json"))
        .send()
        .await?
        .json()
        .await?;
    println!("GET /openapi.json -> {}", spec["info"]["title"]);

    println!("docs UI available at {base}/docs");

    // 5. Shut down.
    let _ = shutdown_tx.send(());
    let _ = server.await;
    Ok(())
}
