//! Compiled example for `docs/src/deployment-topologies/http-service-host.md`
//! (Epic 6 of Milestone 11).
//!
//! Pulled into the page via mdBook `{{#include}}`, so `cargo check
//! -p paladin-doc-examples` keeps it matching the current `axum` + Paladin API.
//! The example compiles in full — including the `axum::serve` bind line — because
//! `cargo check` never *runs* it, so no port is bound during the gate.
#![allow(unused_variables, unused_imports, dead_code)]

// ANCHOR: http_host
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;

use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::routing::post;
use axum::{Json, Router};
use serde::{Deserialize, Serialize};

use paladin::MockLlmAdapter;
use paladin::application::services::paladin::paladin_execution_service::PaladinExecutionService;
use paladin::infrastructure::resilience::circuit_breaker::CircuitBreaker;
use paladin::prelude::*; // PaladinBuilder, LlmPort, Paladin, PaladinResult

/// Shared state: a registry of distinct agents, each with its own execution
/// service, all resident in this one long-running process.
#[derive(Clone)]
struct AppState {
    agents: Arc<HashMap<String, (Paladin, Arc<PaladinExecutionService>)>>,
}

#[derive(Deserialize)]
struct ExecuteRequest {
    input: String,
}

#[derive(Serialize)]
struct ExecuteResponse {
    output: String,
}

/// `POST /agents/{id}/execute` — look the agent up by id and run it. This handler
/// is **yours to write**: Paladin ships no agent-execution endpoint
/// (`paladin-web::create_app_router` is a separate user/auth API, not an agent
/// runner), so you compose `axum` + `PaladinExecutionService` yourself.
async fn execute_agent(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(req): Json<ExecuteRequest>,
) -> Result<Json<ExecuteResponse>, StatusCode> {
    let (paladin, service) = state.agents.get(&id).ok_or(StatusCode::NOT_FOUND)?;
    let result: PaladinResult = service
        .execute(paladin, &req.input)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(ExecuteResponse {
        output: result.output,
    }))
}

/// Wire the agent registry into an `axum` router.
fn agent_router(state: AppState) -> Router {
    Router::new()
        .route("/agents/{id}/execute", post(execute_agent))
        .with_state(state)
}

/// Build a couple of distinct agents and serve them over HTTP. Concurrent
/// requests share the registry and run on the `tokio` runtime.
pub async fn serve_agents() -> Result<(), Box<dyn std::error::Error>> {
    let llm: Arc<dyn LlmPort> = Arc::new(MockLlmAdapter::new());
    let breaker = Arc::new(CircuitBreaker::new(5, 2, Duration::from_secs(30)));

    let mut agents = HashMap::new();
    for (name, prompt) in [
        ("researcher", "You research topics thoroughly."),
        ("summarizer", "You write concise summaries."),
    ] {
        let agent = PaladinBuilder::new(llm.clone())
            .name(name)
            .system_prompt(prompt)
            .build()
            .await?;
        let service = Arc::new(PaladinExecutionService::new(
            llm.clone(),
            breaker.clone(),
            None,
            None,
        ));
        agents.insert(name.to_string(), (agent, service));
    }

    let state = AppState {
        agents: Arc::new(agents),
    };
    let app = agent_router(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, app).await?;
    Ok(())
}
// ANCHOR_END: http_host
