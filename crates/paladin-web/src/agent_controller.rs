//! Axum HTTP controller for the agent-execution API (the HTTP service-host topology).
//!
//! This module defines the wire types and shared state for running resident agents
//! over HTTP. The handlers, error helpers, and router are added in later tasks:
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

use serde::{Deserialize, Serialize};

use paladin_core::platform::container::execution_result::{PaladinResult, StopReason};
use paladin_core::platform::container::paladin::Paladin;

use crate::agent_registry::{AgentProvisioner, AgentRegistry};

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

#[cfg(test)]
mod tests {
    use super::*;

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
