//! Integration tests for the content → agent bridge (Milestone 9, Epic 3).
//!
//! These tests register a [`PaladinContentProcessor`] backed by a mock LLM with
//! the [`Orchestrator`] and drive a [`ContentItem`] through
//! [`Orchestrator::process_content`], asserting that the enriched result and
//! metadata flow back through the orchestration session lifecycle. They also
//! assert that referencing an unregistered processor name surfaces
//! [`OrchestratorError::ProcessorNotFound`].
//!
//! The suite is fully deterministic and offline: it uses
//! [`MockLlmAdapter`](paladin::MockLlmAdapter) and local in-memory fixtures,
//! so no network access or real LLM credentials are required.

use std::sync::Arc;
use std::time::Duration;

use paladin::MockLlmAdapter;
use paladin::application::services::orchestration::Orchestrator;
use paladin::application::services::orchestration::processors::{
    OutputParsing, PaladinContentProcessor,
};
use paladin::application::services::orchestration::types::OrchestratorError;
use paladin::application::services::paladin::paladin_builder::PaladinBuilder;
use paladin::application::services::paladin::paladin_execution_service::PaladinExecutionService;
use paladin::core::platform::container::content::{ContentItem, ContentType, TextContent};
use paladin::core::platform::container::orchestration_context::OrchestrationContext;
use paladin::infrastructure::resilience::circuit_breaker::CircuitBreaker;
use paladin_ports::output::llm_port::LlmPort;

/// Builds a [`PaladinContentProcessor`] backed by a mock LLM returning `response`.
async fn build_processor(response: &str, parsing: OutputParsing) -> PaladinContentProcessor {
    let llm_port: Arc<dyn LlmPort> =
        Arc::new(MockLlmAdapter::new().with_response(response.to_string()));
    let paladin = PaladinBuilder::new(llm_port.clone())
        .system_prompt("You are a content analyst")
        .name("Analyst")
        .model("gpt-4")
        .build()
        .await
        .expect("failed to build paladin");

    let circuit_breaker = Arc::new(CircuitBreaker::new(5, 3, Duration::from_secs(60)));
    let service = Arc::new(PaladinExecutionService::new(
        llm_port,
        circuit_breaker,
        None,
        None,
    ));

    PaladinContentProcessor::new(service, Arc::new(paladin)).with_output_parsing(parsing)
}

/// Builds a simple text [`ContentItem`] with the given body.
fn text_item(body: &str) -> ContentItem {
    let content = TextContent::new(None, Some(body.to_string())).expect("text content");
    ContentItem::new_with_title(ContentType::Text(content), "Test Article".to_string())
        .expect("content item")
}

fn context() -> OrchestrationContext {
    OrchestrationContext::new("integration-test".to_string(), "test".to_string())
}

#[tokio::test]
async fn process_content_enriches_via_registered_paladin_processor() {
    let orchestrator = Orchestrator::new();
    let processor = build_processor(
        "This article explains Rust ownership.",
        OutputParsing::RawText,
    )
    .await;

    orchestrator
        .register_content_processor(Box::new(processor))
        .await
        .expect("processor registration succeeds");

    let item = text_item("Rust enforces memory safety without a garbage collector.");
    let content_id = item.uuid();

    let result = orchestrator
        .process_content(item, "PaladinContentProcessor", context())
        .await
        .expect("content processing succeeds");

    // The enriched result flows back through the orchestration session.
    assert!(result.success, "expected a successful enrichment");
    assert_eq!(result.content_id, content_id);
    assert_eq!(result.processor_name, "PaladinContentProcessor");

    // The agent's response is preserved as enrichment.
    let data = result.result_data.expect("result_data present");
    assert_eq!(data["enrichment"], "This article explains Rust ownership.");

    // Enrichment metadata identifies the agent and parsing strategy.
    assert_eq!(result.metadata["agent_name"], "Analyst");
    assert_eq!(result.metadata["parsing_strategy"], "raw_text");
}

#[tokio::test]
async fn process_content_with_json_strategy_populates_structured_data() {
    let orchestrator = Orchestrator::new();
    let processor = build_processor(
        r#"{"topic": "memory safety", "language": "Rust"}"#,
        OutputParsing::Json,
    )
    .await;

    orchestrator
        .register_content_processor(Box::new(processor))
        .await
        .expect("processor registration succeeds");

    let result = orchestrator
        .process_content(
            text_item("Rust enforces memory safety."),
            "PaladinContentProcessor",
            context(),
        )
        .await
        .expect("content processing succeeds");

    assert!(result.success);
    let data = result.result_data.expect("result_data present");
    assert_eq!(data["topic"], "memory safety");
    assert_eq!(data["language"], "Rust");
}

#[tokio::test]
async fn process_content_unregistered_name_returns_processor_not_found() {
    let orchestrator = Orchestrator::new();

    let err = orchestrator
        .process_content(
            text_item("Some content."),
            "NonexistentProcessor",
            context(),
        )
        .await
        .expect_err("expected ProcessorNotFound for an unregistered name");

    match err {
        OrchestratorError::ProcessorNotFound(name) => {
            assert_eq!(name, "NonexistentProcessor");
        }
        other => panic!("expected ProcessorNotFound, got {other:?}"),
    }
}
