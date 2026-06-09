//! Liveness and readiness endpoints (Milestone 12, Epic 4).
//!
//! - `GET /health` — liveness: always `200 { "status": "ok" }`; depends on nothing, so a
//!   Kubernetes liveness probe only fails if the process is truly wedged.
//! - `GET /ready` — readiness: a **shallow** check returning `200 { "status": "ready",
//!   "agents": N }` once the registry/state is built and serving. It performs no network
//!   I/O (see the PRD's Open Question on a deeper provider check).

use axum::extract::State;
use axum::http::StatusCode;
use axum::{Json, Router, routing::get};
use serde_json::{Value, json};

use crate::agent_controller::AgentApiState;

/// `GET /health` — liveness probe.
pub async fn health() -> (StatusCode, Json<Value>) {
    (StatusCode::OK, Json(json!({ "status": "ok" })))
}

/// `GET /ready` — readiness probe (shallow: ready once the registry is built).
pub async fn ready(State(state): State<AgentApiState>) -> (StatusCode, Json<Value>) {
    (
        StatusCode::OK,
        Json(json!({ "status": "ready", "agents": state.registry.len() })),
    )
}

/// Health/readiness routes, bound to the shared [`AgentApiState`].
///
/// Merged into the application router alongside the agent routes.
pub fn health_routes(state: AgentApiState) -> Router {
    Router::new()
        .route("/health", get(health))
        .route("/ready", get(ready))
        .with_state(state)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::agent_registry::AgentRegistry;
    use std::sync::Arc;

    #[tokio::test]
    async fn health_is_ok() {
        let (status, Json(body)) = health().await;
        assert_eq!(status, StatusCode::OK);
        assert_eq!(body["status"], "ok");
    }

    #[tokio::test]
    async fn ready_reports_agent_count() {
        let state = AgentApiState::new(Arc::new(AgentRegistry::new()));
        let (status, Json(body)) = ready(State(state)).await;
        assert_eq!(status, StatusCode::OK);
        assert_eq!(body["status"], "ready");
        assert_eq!(body["agents"], 0);
    }
}
