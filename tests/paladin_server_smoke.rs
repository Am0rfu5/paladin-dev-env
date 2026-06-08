//! Boot smoke test for the HTTP service host (Milestone 12, Epic 2).
//!
//! Builds a hermetic agent backed by [`MockLlmAdapter`] (no network / API keys), serves
//! the agent router on an ephemeral port via `axum::serve`, and drives it over real HTTP
//! with `reqwest` — mirroring what the `paladin-server` binary does. Also asserts that
//! the server shuts down cleanly when its graceful-shutdown signal fires.
#![cfg(feature = "web-server")]

use std::sync::Arc;
use std::time::Duration;

use paladin::MockLlmAdapter;
use paladin::application::services::paladin::paladin_builder::PaladinBuilder;
use paladin::application::services::paladin::paladin_execution_service::PaladinExecutionService;
use paladin::infrastructure::resilience::circuit_breaker::CircuitBreaker;
use paladin::infrastructure::web::{AgentApiState, AgentRegistry, agent_router};
use paladin_ports::output::llm_port::LlmPort;
use paladin_ports::output::paladin_executor_port::PaladinExecutorPort;
use paladin_ports::output::streaming_executor_port::StreamingExecutorPort;

/// Build an `AgentApiState` holding a single mock-backed, streaming-capable agent.
async fn state_with_mock_agent(id: &str) -> AgentApiState {
    let llm: Arc<dyn LlmPort> = Arc::new(MockLlmAdapter::new());
    let breaker = Arc::new(CircuitBreaker::new(5, 2, Duration::from_secs(30)));
    // One service backs both the buffered and streaming handles.
    let service = Arc::new(PaladinExecutionService::new(
        Arc::clone(&llm),
        breaker,
        None,
        None,
    ));
    let executor: Arc<dyn PaladinExecutorPort> = service.clone();
    let streamer: Arc<dyn StreamingExecutorPort> = service;
    let paladin = PaladinBuilder::new(llm)
        .name(id)
        .system_prompt("You are a smoke-test agent.")
        .model("mock")
        .build()
        .await
        .expect("agent builds");

    let registry = AgentRegistry::new();
    registry.insert_with_streaming(id, Arc::new(paladin), executor, Some(streamer));
    AgentApiState::new(Arc::new(registry))
}

#[tokio::test]
async fn server_boots_serves_agents_and_shuts_down_cleanly() {
    let app = agent_router(state_with_mock_agent("researcher").await);

    // Bind an ephemeral port and serve with a graceful-shutdown trigger.
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
        .await
        .expect("bind ephemeral port");
    let addr = listener.local_addr().expect("local addr");
    let (shutdown_tx, shutdown_rx) = tokio::sync::oneshot::channel::<()>();
    let server = tokio::spawn(async move {
        axum::serve(listener, app)
            .with_graceful_shutdown(async move {
                let _ = shutdown_rx.await;
            })
            .await
            .expect("server runs");
    });

    let base = format!("http://{addr}");
    let client = reqwest::Client::new();

    // GET /agents → 200 with the registered agent.
    let resp = client
        .get(format!("{base}/agents"))
        .send()
        .await
        .expect("GET /agents");
    assert_eq!(resp.status().as_u16(), 200);
    let agents: serde_json::Value = resp.json().await.expect("agents json");
    assert_eq!(agents.as_array().map(|a| a.len()), Some(1));
    assert_eq!(agents[0]["id"], "researcher");

    // POST /agents/researcher/execute → 200 with an output string.
    let resp = client
        .post(format!("{base}/agents/researcher/execute"))
        .json(&serde_json::json!({ "input": "hello" }))
        .send()
        .await
        .expect("POST execute");
    assert_eq!(resp.status().as_u16(), 200);
    let body: serde_json::Value = resp.json().await.expect("execute json");
    assert!(
        body["output"].is_string(),
        "expected an output string, got {body}"
    );

    // Unknown agent → 404.
    let resp = client
        .post(format!("{base}/agents/ghost/execute"))
        .json(&serde_json::json!({ "input": "hi" }))
        .send()
        .await
        .expect("POST unknown");
    assert_eq!(resp.status().as_u16(), 404);

    // POST /agents/researcher/execute/stream → SSE chunk(s) then a done event.
    let resp = client
        .post(format!("{base}/agents/researcher/execute/stream"))
        .json(&serde_json::json!({ "input": "hello" }))
        .send()
        .await
        .expect("POST stream");
    assert_eq!(resp.status().as_u16(), 200);
    let sse = resp.text().await.expect("stream body");
    assert!(sse.contains("event: chunk"), "expected chunk events: {sse}");
    assert!(sse.contains("event: done"), "expected a done event: {sse}");

    // POST /agents/researcher/jobs → 202 + job_id, then poll to completion.
    let resp = client
        .post(format!("{base}/agents/researcher/jobs"))
        .json(&serde_json::json!({ "input": "hello" }))
        .send()
        .await
        .expect("POST jobs");
    assert_eq!(resp.status().as_u16(), 202);
    let body: serde_json::Value = resp.json().await.expect("job json");
    let job_id = body["job_id"].as_str().expect("job_id").to_string();

    let mut completed = false;
    for _ in 0..50 {
        let resp = client
            .get(format!("{base}/agents/researcher/jobs/{job_id}"))
            .send()
            .await
            .expect("GET job");
        assert_eq!(resp.status().as_u16(), 200);
        let record: serde_json::Value = resp.json().await.expect("job record");
        if record["status"] != "running" {
            assert_eq!(record["status"], "completed", "job record: {record}");
            assert!(record["result"]["output"].is_string());
            completed = true;
            break;
        }
        tokio::time::sleep(Duration::from_millis(20)).await;
    }
    assert!(completed, "job did not complete in time");

    // Trigger graceful shutdown; the server task must complete cleanly.
    shutdown_tx.send(()).expect("send shutdown");
    server.await.expect("server task joins after shutdown");
}
