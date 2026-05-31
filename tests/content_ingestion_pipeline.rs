//! End-to-end content ingestion → enrichment pipeline tests (Milestone 9, Epic 3).
//!
//! These tests are gated behind the `content-processing` feature because they
//! exercise the `paladin-content` ingestion adapters. Run them with:
//!
//! ```bash
//! cargo test --features content-processing --test content_ingestion_pipeline
//! ```
//!
//! The default (`deterministic_*`) test is fully offline: it ingests a local
//! fixture file via [`FileContentFetcher`] and enriches it with a
//! [`MockLlmAdapter`](paladin::MockLlmAdapter)-backed
//! [`PaladinContentProcessor`], so no network access or LLM credentials are
//! required.
//!
//! The `live_*` test is marked `#[ignore]` and exercises the real
//! `HttpContentFetcher` plus a real OpenAI-backed agent; see its docs for run
//! instructions.
#![cfg(feature = "content-processing")]

use std::sync::Arc;
use std::time::Duration;

use paladin::MockLlmAdapter;
use paladin::application::services::orchestration::Orchestrator;
use paladin::application::services::orchestration::processors::{
    OutputParsing, PaladinContentProcessor,
};
use paladin::application::services::paladin::paladin_builder::PaladinBuilder;
use paladin::application::services::paladin::paladin_execution_service::PaladinExecutionService;
use paladin::core::platform::container::content::{ContentItem, ContentType, TextContent};
use paladin::core::platform::container::orchestration_context::OrchestrationContext;
use paladin::infrastructure::resilience::circuit_breaker::CircuitBreaker;
use paladin_content::adapters::input::file_content_fetcher::FileContentFetcher;
use paladin_ports::input::content_input_port::ContentIngestionPort;
use paladin_ports::output::llm_port::LlmPort;
use url::Url;

/// Path to the local fixture article shipped with the test suite.
fn fixture_path() -> std::path::PathBuf {
    std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join("sample_article.txt")
}

/// Ingests the local fixture file into a [`ContentItem`] via [`FileContentFetcher`].
///
/// This is the deterministic, offline "extract" stage of the pipeline.
fn ingest_fixture() -> ContentItem {
    let path = fixture_path();
    let url = Url::from_file_path(&path).expect("fixture path is absolute");

    // Seed item carries only the file URL; the fetcher reads the file and
    // produces a fully populated ContentItem.
    let seed_text = TextContent::new(Some(path.to_string_lossy().to_string()), None)
        .expect("seed text content");
    let mut seed = ContentItem::new(ContentType::Text(seed_text)).expect("seed content item");
    seed.set_url(Some(url));

    FileContentFetcher
        .fetch_content(seed)
        .expect("fixture ingestion succeeds")
}

/// Builds a [`PaladinContentProcessor`] backed by a mock LLM returning `response`.
async fn mock_processor(response: &str) -> PaladinContentProcessor {
    let llm_port: Arc<dyn LlmPort> =
        Arc::new(MockLlmAdapter::new().with_response(response.to_string()));
    let paladin = PaladinBuilder::new(llm_port.clone())
        .system_prompt("You summarize technical articles")
        .name("Summarizer")
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

    PaladinContentProcessor::new(service, Arc::new(paladin))
        .with_output_parsing(OutputParsing::RawText)
}

#[tokio::test]
async fn deterministic_local_fixture_ingestion_to_enrichment() {
    // Extract: ingest the local fixture (no network).
    let item = ingest_fixture();
    let content_id = item.uuid();

    // The ingested item carries the fixture's text.
    if let ContentType::Text(text) = item.content() {
        assert!(
            text.content
                .as_ref()
                .expect("fixture text present")
                .contains("memory safety"),
            "fixture body should be preserved through ingestion"
        );
    } else {
        panic!("expected text content from the fixture");
    }

    // Enrich: drive the ingested item through the orchestrator with a mock LLM.
    let orchestrator = Orchestrator::new();
    orchestrator
        .register_content_processor(Box::new(
            mock_processor("Summary: Rust guarantees memory safety at compile time.").await,
        ))
        .await
        .expect("processor registration succeeds");

    let result = orchestrator
        .process_content(
            item,
            "PaladinContentProcessor",
            OrchestrationContext::new("ingestion-pipeline".to_string(), "test".to_string()),
        )
        .await
        .expect("content processing succeeds");

    // The full chain preserves the content id, includes enrichment, and succeeds.
    assert!(result.success, "pipeline should report success");
    assert_eq!(
        result.content_id, content_id,
        "content id must be preserved end to end"
    );
    let data = result.result_data.expect("enrichment present");
    assert_eq!(
        data["enrichment"],
        "Summary: Rust guarantees memory safety at compile time."
    );
    assert_eq!(result.metadata["agent_name"], "Summarizer");
}

/// Live end-to-end test: real `HttpContentFetcher` + real OpenAI-backed agent.
///
/// This test is `#[ignore]` by default because it requires network access and a
/// valid `OPENAI_API_KEY`. It is additionally gated on the `llm-openai` feature.
///
/// Run it explicitly with:
///
/// ```bash
/// OPENAI_API_KEY=sk-... \
///   cargo test --features "content-processing llm-openai" \
///   --test content_ingestion_pipeline -- --ignored live_http_fetch_and_real_llm_enrichment
/// ```
///
/// If `OPENAI_API_KEY` is not set, the test exits early so an accidental
/// `--ignored` run does not fail.
#[cfg(feature = "llm-openai")]
#[tokio::test]
#[ignore = "requires network access and OPENAI_API_KEY"]
async fn live_http_fetch_and_real_llm_enrichment() {
    use paladin::OpenAIAdapter;
    use paladin_content::adapters::input::http_content_fetcher::HttpContentFetcher;
    use paladin_content::services::content_fetching_service::ContentFetchingService;

    if std::env::var("OPENAI_API_KEY").is_err() {
        eprintln!("OPENAI_API_KEY not set; skipping live ingestion test");
        return;
    }

    // Extract: fetch a real page over HTTP (blocking fetcher on a blocking task).
    let item = tokio::task::spawn_blocking(|| {
        HttpContentFetcher::new()
            .fetch_content("https://example.com")
            .expect("live fetch succeeds")
    })
    .await
    .expect("fetch task joins");

    // Enrich: run the fetched content through a real OpenAI-backed agent.
    let llm_port: Arc<dyn LlmPort> =
        Arc::new(OpenAIAdapter::from_env().expect("OpenAI adapter from env"));
    let paladin = PaladinBuilder::new(llm_port.clone())
        .system_prompt("You summarize web pages in one sentence")
        .name("Summarizer")
        .model("gpt-4o-mini")
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
    let processor = PaladinContentProcessor::new(service, Arc::new(paladin));

    let orchestrator = Orchestrator::new();
    orchestrator
        .register_content_processor(Box::new(processor))
        .await
        .expect("processor registration succeeds");

    let result = orchestrator
        .process_content(
            item,
            "PaladinContentProcessor",
            OrchestrationContext::new("live-ingestion".to_string(), "test".to_string()),
        )
        .await
        .expect("live content processing succeeds");

    assert!(result.success);
    assert!(result.result_data.is_some(), "live enrichment present");
}
