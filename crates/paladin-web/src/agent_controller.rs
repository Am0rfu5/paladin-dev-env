//! Axum HTTP controller for the agent-execution API (the HTTP service-host topology).
//!
//! This module defines the wire types, shared state, handlers, and router for running
//! resident agents over HTTP:
//!
//! | Method & path | Description |
//! |---------------|-------------|
//! | `POST /agents/{id}/execute` | Run an agent and return its output |
//! | `GET /agents` | List registered agents |
//! | `GET /agents/{id}` | Describe a single agent |
//! | `POST /agents` | Register an agent at runtime |
//! | `DELETE /agents/{id}` | Deregister an agent |
//!
//! Responses follow the same convention as
//! [`delivery_controller`](crate::delivery_controller): a success body is the serialized
//! payload, and an error body is `{ "error": "<message>" }`.

use std::sync::Arc;

use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    routing::{get, post},
};
use serde::{Deserialize, Serialize};
use serde_json::json;

use paladin_core::platform::container::execution_result::{PaladinResult, StopReason};
use paladin_core::platform::container::paladin::Paladin;
use paladin_core::platform::container::paladin_error::PaladinError;

use crate::agent_registry::{AgentProvisioner, AgentRegistry, AgentSpec};

/// Shared state for the agent routes.
///
/// Cloned into every handler by `axum`. The `registry` is always present; the
/// `provisioner` is optional — when it is `None`, runtime registration
/// (`POST /agents`) fails closed rather than panicking, while execution and discovery
/// remain fully functional.
#[derive(Clone)]
pub struct AgentApiState {
    /// The resident agent registry shared across requests.
    pub registry: Arc<AgentRegistry>,
    /// Optional provisioner used to build agents for `POST /agents` (injected by Epic 2).
    pub provisioner: Option<Arc<dyn AgentProvisioner>>,
}

impl AgentApiState {
    /// Create state with a registry and no provisioner (runtime registration disabled).
    pub fn new(registry: Arc<AgentRegistry>) -> Self {
        Self {
            registry,
            provisioner: None,
        }
    }

    /// Attach a provisioner, enabling runtime registration via `POST /agents`.
    pub fn with_provisioner(mut self, provisioner: Arc<dyn AgentProvisioner>) -> Self {
        self.provisioner = Some(provisioner);
        self
    }
}

/// Request body for `POST /agents/{id}/execute`.
///
/// Only `input` is required today; later epics may add optional fields (streaming
/// flags, per-call overrides) without breaking this contract.
#[derive(Debug, Clone, Deserialize)]
pub struct ExecuteRequest {
    /// The task / prompt to run the agent against.
    pub input: String,
}

/// Response body for a successful agent execution.
///
/// Carries the agent output plus the safe execution metadata from
/// [`PaladinResult`]. The `stop_reason` is rendered as a stable lowercase label
/// (`"completed"`, `"max_loops"`, `"stop_word"`, `"timeout"`) rather than the raw
/// serde enum shape, so the wire contract is stable.
#[derive(Debug, Clone, Serialize)]
pub struct ExecuteResponse {
    /// The generated output text.
    pub output: String,
    /// Total tokens used (prompt + completion).
    pub token_count: u32,
    /// Wall-clock execution time in milliseconds.
    pub execution_time_ms: u64,
    /// Number of reasoning loops executed.
    pub loop_count: u32,
    /// Why execution stopped, as a stable label.
    pub stop_reason: String,
}

impl From<PaladinResult> for ExecuteResponse {
    fn from(result: PaladinResult) -> Self {
        Self {
            output: result.output,
            token_count: result.token_count,
            execution_time_ms: result.execution_time_ms,
            loop_count: result.loop_count,
            stop_reason: stop_reason_label(&result.stop_reason).to_string(),
        }
    }
}

/// Map a [`StopReason`] to a stable, lowercase wire label.
fn stop_reason_label(reason: &StopReason) -> &'static str {
    match reason {
        StopReason::MaxLoops => "max_loops",
        StopReason::StopWord(_) => "stop_word",
        StopReason::Completed => "completed",
        StopReason::Timeout => "timeout",
    }
}

/// Safe, public-facing summary of an agent for the discovery endpoints.
///
/// Deliberately omits anything sensitive. Note that secrets (API keys, provider
/// credentials) never live on the [`Paladin`] entity in the first place — they are
/// supplied to executors at composition time — so none can leak here. The raw system
/// prompt is reduced to a short `description` preview rather than returned verbatim
/// (see PRD Open Question 1 on whether to omit it entirely).
#[derive(Debug, Clone, Serialize)]
pub struct AgentSummary {
    /// Registry id (the `{id}` path segment).
    pub id: String,
    /// Human-friendly display name.
    pub name: String,
    /// LLM model identifier.
    pub model: String,
    /// Short, single-line preview derived from the system prompt.
    pub description: String,
}

impl AgentSummary {
    /// Build a summary from a registry id and its agent.
    ///
    /// This is not a `From` impl because a summary needs the registry id, which lives
    /// in the registry key rather than on the [`Paladin`] itself.
    pub fn from_agent(id: impl Into<String>, paladin: &Paladin) -> Self {
        Self {
            id: id.into(),
            name: paladin.node.name.clone(),
            model: paladin.node.model.clone(),
            description: prompt_preview(&paladin.node.system_prompt),
        }
    }
}

/// Maximum number of characters from the system prompt to expose as a description.
const DESCRIPTION_PREVIEW_LEN: usize = 140;

/// Derive a short, single-line preview of a system prompt for discovery responses.
///
/// Takes the first line, trims it, and truncates to [`DESCRIPTION_PREVIEW_LEN`]
/// characters (on a char boundary), appending an ellipsis when truncated.
fn prompt_preview(system_prompt: &str) -> String {
    let first_line = system_prompt.lines().next().unwrap_or("").trim();
    if first_line.chars().count() <= DESCRIPTION_PREVIEW_LEN {
        return first_line.to_string();
    }
    let truncated: String = first_line.chars().take(DESCRIPTION_PREVIEW_LEN).collect();
    format!("{truncated}…")
}

// --- Response helpers (interim) ---------------------------------------------
//
// These mirror `delivery_controller`'s helpers and are kept local on purpose:
// Milestone 12, Epic 4 introduces a unified error model and these become a single
// swap point.

/// JSON response body type used by every agent handler.
pub(crate) type JsonValue = Json<serde_json::Value>;

/// Serialize a successful payload to a JSON body, falling back to an error body if
/// (very unusually) serialization fails.
pub(crate) fn ok_body<T: Serialize>(value: &T) -> JsonValue {
    match serde_json::to_value(value) {
        Ok(v) => Json(v),
        Err(e) => Json(json!({ "error": e.to_string() })),
    }
}

/// Build an `{ "error": "<message>" }` JSON body.
pub(crate) fn error_body(message: impl std::fmt::Display) -> JsonValue {
    Json(json!({ "error": message.to_string() }))
}

/// Map a [`PaladinError`] from agent execution to an HTTP response.
///
/// Execution failures are upstream/LLM/tool failures, so they surface as
/// `502 Bad Gateway` (not `500`), with the error message in the standard body.
pub(crate) fn execution_error_response(error: &PaladinError) -> (StatusCode, JsonValue) {
    (StatusCode::BAD_GATEWAY, error_body(error))
}

// --- Handlers ---------------------------------------------------------------

/// `POST /agents/{id}/execute` — look the agent up by id and run it.
///
/// Returns:
/// - `200 OK` with [`ExecuteResponse`] on success;
/// - `404 Not Found` if no agent is registered under `id`;
/// - `502 Bad Gateway` with `{ "error": ... }` if execution fails;
/// - `400 Bad Request` (via the `Json` extractor) if the body is missing/invalid.
pub async fn execute_agent(
    State(state): State<AgentApiState>,
    Path(id): Path<String>,
    Json(request): Json<ExecuteRequest>,
) -> (StatusCode, JsonValue) {
    let Some(entry) = state.registry.get(&id) else {
        return (
            StatusCode::NOT_FOUND,
            error_body(format!("unknown agent '{id}'")),
        );
    };

    match entry
        .executor
        .execute(entry.paladin.as_ref(), &request.input)
        .await
    {
        Ok(result) => (StatusCode::OK, ok_body(&ExecuteResponse::from(result))),
        Err(error) => execution_error_response(&error),
    }
}

/// `GET /agents` — list every registered agent as a safe [`AgentSummary`].
///
/// Always returns `200 OK` with a JSON array (empty when no agents are registered).
/// Order is unspecified.
pub async fn list_agents(State(state): State<AgentApiState>) -> (StatusCode, JsonValue) {
    let summaries: Vec<AgentSummary> = state
        .registry
        .list()
        .into_iter()
        .map(|(id, paladin)| AgentSummary::from_agent(id, paladin.as_ref()))
        .collect();
    (StatusCode::OK, ok_body(&summaries))
}

/// `GET /agents/{id}` — describe a single agent.
///
/// Returns `200 OK` with the agent's [`AgentSummary`], or `404 Not Found` with
/// `{ "error": ... }` if no agent is registered under `id`.
pub async fn describe_agent(
    State(state): State<AgentApiState>,
    Path(id): Path<String>,
) -> (StatusCode, JsonValue) {
    match state.registry.get(&id) {
        Some(entry) => (
            StatusCode::OK,
            ok_body(&AgentSummary::from_agent(id, entry.paladin.as_ref())),
        ),
        None => (
            StatusCode::NOT_FOUND,
            error_body(format!("unknown agent '{id}'")),
        ),
    }
}

/// `POST /agents` — register a new agent at runtime from an [`AgentSpec`].
///
/// Because `paladin-web` cannot build a [`Paladin`] itself, this delegates to the
/// injected [`AgentProvisioner`]. Returns:
/// - `201 Created` with the new agent's [`AgentSummary`] on success;
/// - `409 Conflict` if an agent is already registered under the spec's id;
/// - `422 Unprocessable Entity` if provisioning fails;
/// - `400 Bad Request` (via the `Json` extractor) if the body is missing/invalid;
/// - `501 Not Implemented` if no provisioner is wired (registration disabled).
pub async fn register_agent(
    State(state): State<AgentApiState>,
    Json(spec): Json<AgentSpec>,
) -> (StatusCode, JsonValue) {
    let Some(provisioner) = state.provisioner.as_ref() else {
        return (
            StatusCode::NOT_IMPLEMENTED,
            error_body("runtime agent registration is not enabled"),
        );
    };

    // Cheap early rejection before paying to provision an agent we'd discard.
    if state.registry.contains(&spec.id) {
        return (
            StatusCode::CONFLICT,
            error_body(format!("agent '{}' already exists", spec.id)),
        );
    }

    match provisioner.provision(&spec).await {
        Ok(provisioned) => {
            let paladin = Arc::new(provisioned.paladin);
            // Re-check on insert closes the race between the `contains` check and here.
            if !state.registry.insert_with_streaming(
                spec.id.clone(),
                Arc::clone(&paladin),
                provisioned.executor,
                provisioned.streamer,
            ) {
                return (
                    StatusCode::CONFLICT,
                    error_body(format!("agent '{}' already exists", spec.id)),
                );
            }
            (
                StatusCode::CREATED,
                ok_body(&AgentSummary::from_agent(spec.id, paladin.as_ref())),
            )
        }
        Err(error) => (StatusCode::UNPROCESSABLE_ENTITY, error_body(error)),
    }
}

/// `DELETE /agents/{id}` — deregister an agent.
///
/// Returns `204 No Content` (empty body) on success, or `404 Not Found` with
/// `{ "error": ... }` if no agent is registered under `id`.
pub async fn deregister_agent(
    State(state): State<AgentApiState>,
    Path(id): Path<String>,
) -> Result<StatusCode, (StatusCode, JsonValue)> {
    if state.registry.remove(&id) {
        Ok(StatusCode::NO_CONTENT)
    } else {
        Err((
            StatusCode::NOT_FOUND,
            error_body(format!("unknown agent '{id}'")),
        ))
    }
}

// --- Router -----------------------------------------------------------------

/// Build the agent-execution sub-router and bind it to its [`AgentApiState`].
///
/// Mounts the five agent routes:
///
/// - `GET    /agents` — list agents
/// - `POST   /agents` — register an agent at runtime
/// - `GET    /agents/{id}` — describe an agent
/// - `DELETE /agents/{id}` — deregister an agent
/// - `POST   /agents/{id}/execute` — run an agent
///
/// The returned `Router` has its state already applied, so it can be `merge`d into the
/// application router alongside the user/auth and delivery routers (see
/// [`create_app_router_with_agents`](crate::app::create_app_router_with_agents)).
///
/// These routes are intentionally **unauthenticated** in Milestone 12, Epic 1;
/// authentication and per-agent authorization are layered on in Epic 5 without changing
/// these handler signatures.
pub fn agent_router(state: AgentApiState) -> Router {
    Router::new()
        .route("/agents", get(list_agents).post(register_agent))
        .route("/agents/{id}", get(describe_agent).delete(deregister_agent))
        .route("/agents/{id}/execute", post(execute_agent))
        .with_state(state)
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;
    use axum::body::Body;
    use axum::http::Request;
    use axum::routing::post;
    use paladin_core::platform::container::paladin::PaladinData;
    use paladin_ports::output::paladin_executor_port::PaladinExecutorPort;
    use tower::ServiceExt; // for `Router::oneshot`

    /// Configurable in-test executor: succeeds with a fixed output, or fails.
    enum MockExecutor {
        Succeeds(String),
        Fails(String),
    }

    #[async_trait]
    impl PaladinExecutorPort for MockExecutor {
        async fn execute(
            &self,
            _paladin: &Paladin,
            _input: &str,
        ) -> Result<PaladinResult, PaladinError> {
            match self {
                MockExecutor::Succeeds(output) => Ok(PaladinResult::new(
                    output.clone(),
                    5,
                    10,
                    1,
                    StopReason::Completed,
                )),
                MockExecutor::Fails(message) => Err(PaladinError::ExecutionError(message.clone())),
            }
        }
    }

    fn test_agent(name: &str) -> Arc<Paladin> {
        let data = PaladinData {
            system_prompt: "You are a test agent.".to_string(),
            name: name.to_string(),
            model: "gpt-4".to_string(),
            ..Default::default()
        };
        Arc::new(Paladin::new(data, Some(name.to_string())))
    }

    /// State holding a single agent `id` backed by `executor`.
    fn state_with_agent(id: &str, executor: MockExecutor) -> AgentApiState {
        let registry = AgentRegistry::new();
        registry.insert(id, test_agent(id), Arc::new(executor));
        AgentApiState::new(Arc::new(registry))
    }

    #[test]
    fn error_body_renders_error_envelope() {
        let Json(value) = error_body("boom");
        assert_eq!(value, json!({ "error": "boom" }));
    }

    #[tokio::test]
    async fn execute_success_returns_200_with_output_and_metadata() {
        let state = state_with_agent("r", MockExecutor::Succeeds("done".to_string()));
        let (status, Json(body)) = execute_agent(
            State(state),
            Path("r".to_string()),
            Json(ExecuteRequest {
                input: "hi".to_string(),
            }),
        )
        .await;

        assert_eq!(status, StatusCode::OK);
        assert_eq!(body["output"], "done");
        assert_eq!(body["token_count"], 5);
        assert_eq!(body["execution_time_ms"], 10);
        assert_eq!(body["loop_count"], 1);
        assert_eq!(body["stop_reason"], "completed");
    }

    #[tokio::test]
    async fn execute_unknown_id_returns_404() {
        let state = state_with_agent("r", MockExecutor::Succeeds("done".to_string()));
        let (status, Json(body)) = execute_agent(
            State(state),
            Path("missing".to_string()),
            Json(ExecuteRequest {
                input: "hi".to_string(),
            }),
        )
        .await;

        assert_eq!(status, StatusCode::NOT_FOUND);
        assert!(body.get("error").is_some(), "expected an error body");
    }

    #[tokio::test]
    async fn execute_executor_error_returns_502() {
        let state = state_with_agent("r", MockExecutor::Fails("upstream down".to_string()));
        let (status, Json(body)) = execute_agent(
            State(state),
            Path("r".to_string()),
            Json(ExecuteRequest {
                input: "hi".to_string(),
            }),
        )
        .await;

        assert_eq!(status, StatusCode::BAD_GATEWAY);
        assert_eq!(
            body.get("error").and_then(|v| v.as_str()),
            Some("Execution error: upstream down")
        );
    }

    #[tokio::test]
    async fn execute_invalid_body_returns_400_through_router() {
        let state = state_with_agent("r", MockExecutor::Succeeds("done".to_string()));
        let app = axum::Router::new()
            .route("/agents/{id}/execute", post(execute_agent))
            .with_state(state);

        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/agents/r/execute")
                    .header("content-type", "application/json")
                    .body(Body::from("{ not valid json "))
                    .expect("request builds"),
            )
            .await
            .expect("router responds");

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }

    use crate::agent_registry::{ProvisionError, ProvisionedAgent};

    /// Configurable in-test provisioner.
    enum MockProvisioner {
        Succeeds,
        Fails(String),
    }

    #[async_trait]
    impl AgentProvisioner for MockProvisioner {
        async fn provision(&self, spec: &AgentSpec) -> Result<ProvisionedAgent, ProvisionError> {
            match self {
                MockProvisioner::Succeeds => {
                    let data = PaladinData {
                        system_prompt: spec.system_prompt.clone(),
                        name: spec.name.clone(),
                        model: spec.model.clone(),
                        ..Default::default()
                    };
                    let paladin = Paladin::new(data, Some(spec.id.clone()));
                    let executor: Arc<dyn PaladinExecutorPort> =
                        Arc::new(MockExecutor::Succeeds("ok".to_string()));
                    Ok(ProvisionedAgent {
                        paladin,
                        executor,
                        streamer: None,
                    })
                }
                MockProvisioner::Fails(message) => Err(ProvisionError::Failed(message.clone())),
            }
        }
    }

    fn sample_spec(id: &str) -> AgentSpec {
        AgentSpec {
            id: id.to_string(),
            name: "Researcher".to_string(),
            model: "gpt-4".to_string(),
            system_prompt: "You research topics.".to_string(),
            temperature: None,
            stop_words: vec![],
        }
    }

    /// State with an empty registry and the given provisioner.
    fn state_with_provisioner(provisioner: MockProvisioner) -> AgentApiState {
        AgentApiState::new(Arc::new(AgentRegistry::new())).with_provisioner(Arc::new(provisioner))
    }

    #[tokio::test]
    async fn register_success_returns_201_and_is_retrievable() {
        let state = state_with_provisioner(MockProvisioner::Succeeds);

        let (status, Json(body)) =
            register_agent(State(state.clone()), Json(sample_spec("new"))).await;
        assert_eq!(status, StatusCode::CREATED);
        assert_eq!(body["id"], "new");
        assert_eq!(body["name"], "Researcher");

        // The shared registry now resolves the new agent.
        let (status, _) = describe_agent(State(state), Path("new".to_string())).await;
        assert_eq!(status, StatusCode::OK);
    }

    #[tokio::test]
    async fn register_duplicate_id_returns_409() {
        let state = state_with_provisioner(MockProvisioner::Succeeds);
        // First registration succeeds.
        let _ = register_agent(State(state.clone()), Json(sample_spec("dup"))).await;
        // Second with the same id conflicts.
        let (status, body) = register_agent(State(state), Json(sample_spec("dup"))).await;
        assert_eq!(status, StatusCode::CONFLICT);
        assert!(body.0.get("error").is_some());
    }

    #[tokio::test]
    async fn register_provision_failure_returns_422() {
        let state = state_with_provisioner(MockProvisioner::Fails("no such model".to_string()));
        let (status, Json(body)) = register_agent(State(state), Json(sample_spec("x"))).await;
        assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY);
        assert_eq!(
            body.get("error").and_then(|v| v.as_str()),
            Some("provisioning failed: no such model")
        );
    }

    #[tokio::test]
    async fn register_without_provisioner_returns_501() {
        // No provisioner wired: registration must fail closed, not panic.
        let state = AgentApiState::new(Arc::new(AgentRegistry::new()));
        let (status, body) = register_agent(State(state), Json(sample_spec("x"))).await;
        assert_eq!(status, StatusCode::NOT_IMPLEMENTED);
        assert!(body.0.get("error").is_some());
    }

    #[tokio::test]
    async fn register_invalid_body_returns_400_through_router() {
        let state = state_with_provisioner(MockProvisioner::Succeeds);
        let app = axum::Router::new()
            .route("/agents", post(register_agent))
            .with_state(state);

        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/agents")
                    .header("content-type", "application/json")
                    .body(Body::from("{ not valid json "))
                    .expect("request builds"),
            )
            .await
            .expect("router responds");

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn deregister_known_id_returns_204_then_404() {
        let state = registry_state(vec![("r", test_agent("Researcher"))]);

        let result = deregister_agent(State(state.clone()), Path("r".to_string())).await;
        match result {
            Ok(status) => assert_eq!(status, StatusCode::NO_CONTENT),
            Err((status, _)) => panic!("expected 204, got {status:?}"),
        }

        // The agent is gone afterward.
        let (status, _) = describe_agent(State(state), Path("r".to_string())).await;
        assert_eq!(status, StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn deregister_unknown_id_returns_404() {
        let state = AgentApiState::new(Arc::new(AgentRegistry::new()));
        let result = deregister_agent(State(state), Path("missing".to_string())).await;
        match result {
            Err((status, _)) => assert_eq!(status, StatusCode::NOT_FOUND),
            Ok(other) => panic!("expected 404, got {other:?}"),
        }
    }

    #[tokio::test]
    async fn agent_router_merges_with_other_routes_without_conflict() {
        let state = registry_state(vec![("r", test_agent("Researcher"))]);
        // A stand-in for the user/auth router, with its own state already applied.
        let other = Router::new().route("/users/login", post(|| async { StatusCode::OK }));
        let app = other.merge(agent_router(state));

        // An agent route resolves.
        let agents = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/agents")
                    .body(Body::empty())
                    .expect("request builds"),
            )
            .await
            .expect("router responds");
        assert_eq!(agents.status(), StatusCode::OK);

        // The merged-in placeholder route also resolves.
        let login = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/users/login")
                    .body(Body::empty())
                    .expect("request builds"),
            )
            .await
            .expect("router responds");
        assert_eq!(login.status(), StatusCode::OK);
    }

    /// Build an agent with a multi-line prompt whose second line is a leak canary.
    fn agent_with_secret_second_line(name: &str) -> Arc<Paladin> {
        let data = PaladinData {
            system_prompt: "Public first line.\nLEAK_CANARY second line.".to_string(),
            name: name.to_string(),
            model: "gpt-4".to_string(),
            ..Default::default()
        };
        Arc::new(Paladin::new(data, Some(name.to_string())))
    }

    fn registry_state(agents: Vec<(&str, Arc<Paladin>)>) -> AgentApiState {
        let registry = AgentRegistry::new();
        for (id, paladin) in agents {
            registry.insert(
                id,
                paladin,
                Arc::new(MockExecutor::Succeeds("x".to_string())),
            );
        }
        AgentApiState::new(Arc::new(registry))
    }

    #[tokio::test]
    async fn list_agents_returns_200_with_summaries_and_no_prompt_leak() {
        let state = registry_state(vec![
            ("researcher", agent_with_secret_second_line("Researcher")),
            ("summarizer", test_agent("Summarizer")),
        ]);

        let (status, Json(body)) = list_agents(State(state)).await;
        assert_eq!(status, StatusCode::OK);

        let arr = body.as_array().expect("body is a JSON array");
        assert_eq!(arr.len(), 2);

        let mut ids: Vec<&str> = arr
            .iter()
            .map(|a| a["id"].as_str().expect("id is a string"))
            .collect();
        ids.sort();
        assert_eq!(ids, vec!["researcher", "summarizer"]);

        // The full multi-line prompt must never appear in a discovery response.
        assert!(
            !body.to_string().contains("LEAK_CANARY"),
            "discovery response leaked the raw system prompt"
        );
    }

    #[tokio::test]
    async fn list_agents_empty_registry_returns_empty_array() {
        let state = AgentApiState::new(Arc::new(AgentRegistry::new()));
        let (status, Json(body)) = list_agents(State(state)).await;
        assert_eq!(status, StatusCode::OK);
        assert_eq!(body.as_array().map(|a| a.len()), Some(0));
    }

    #[tokio::test]
    async fn describe_agent_returns_200_for_known_id_without_prompt_leak() {
        let state = registry_state(vec![("r", agent_with_secret_second_line("Researcher"))]);
        let (status, Json(body)) = describe_agent(State(state), Path("r".to_string())).await;

        assert_eq!(status, StatusCode::OK);
        assert_eq!(body["id"], "r");
        assert_eq!(body["name"], "Researcher");
        assert_eq!(body["model"], "gpt-4");
        assert_eq!(body["description"], "Public first line.");
        assert!(!body.to_string().contains("LEAK_CANARY"));
    }

    #[tokio::test]
    async fn describe_agent_unknown_id_returns_404() {
        let state = registry_state(vec![("r", test_agent("Researcher"))]);
        let (status, Json(body)) = describe_agent(State(state), Path("missing".to_string())).await;
        assert_eq!(status, StatusCode::NOT_FOUND);
        assert!(body.get("error").is_some(), "expected an error body");
    }

    #[test]
    fn execute_response_from_paladin_result_maps_fields_and_label() {
        let result = PaladinResult::new("hi".to_string(), 7, 42, 2, StopReason::MaxLoops);
        let response = ExecuteResponse::from(result);
        assert_eq!(response.output, "hi");
        assert_eq!(response.token_count, 7);
        assert_eq!(response.execution_time_ms, 42);
        assert_eq!(response.loop_count, 2);
        assert_eq!(response.stop_reason, "max_loops");
    }

    #[test]
    fn stop_reason_labels_are_stable() {
        assert_eq!(stop_reason_label(&StopReason::Completed), "completed");
        assert_eq!(stop_reason_label(&StopReason::MaxLoops), "max_loops");
        assert_eq!(
            stop_reason_label(&StopReason::StopWord("x".to_string())),
            "stop_word"
        );
        assert_eq!(stop_reason_label(&StopReason::Timeout), "timeout");
    }

    #[test]
    fn agent_summary_previews_prompt_and_omits_full_text() {
        let data = paladin_core::platform::container::paladin::PaladinData {
            system_prompt: "First line of behavior.\nSecret second line.".to_string(),
            name: "Researcher".to_string(),
            model: "gpt-4".to_string(),
            ..Default::default()
        };
        let paladin = Paladin::new(data, Some("researcher".to_string()));
        let summary = AgentSummary::from_agent("researcher", &paladin);

        assert_eq!(summary.id, "researcher");
        assert_eq!(summary.name, "Researcher");
        assert_eq!(summary.model, "gpt-4");
        // Only the first line is previewed; the second line is not exposed.
        assert_eq!(summary.description, "First line of behavior.");
        assert!(!summary.description.contains("Secret second line"));
    }

    #[test]
    fn long_prompt_preview_is_truncated_with_ellipsis() {
        let long = "a".repeat(DESCRIPTION_PREVIEW_LEN + 50);
        let preview = prompt_preview(&long);
        assert_eq!(preview.chars().count(), DESCRIPTION_PREVIEW_LEN + 1); // +1 for the ellipsis
        assert!(preview.ends_with('…'));
    }
}
