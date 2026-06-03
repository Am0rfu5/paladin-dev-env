//! Examples for `docs/src/user-guides/content-processing.md`.
#![allow(unused_variables, unused_imports, dead_code)]

use std::sync::Arc;

use crate::support::{MockDeliveryAdapter, MockListService, text_content_item, text_prompt_item};

// ANCHOR: pdf
use paladin_content::adapters::document::pdf_extractor::PdfExtractor;
use std::path::Path;

/// Extract a PDF (from a path or raw bytes) into a `Document`.
pub fn ingest_pdf() -> Result<(), Box<dyn std::error::Error>> {
    let extractor = PdfExtractor::new();
    let document = extractor.extract(Path::new("./reports/q3-earnings.pdf"))?;
    // Or from bytes already in memory:
    // let document = extractor.extract_bytes(&pdf_bytes)?;
    Ok(())
}
// ANCHOR_END: pdf

// ANCHOR: http
use paladin_content::adapters::input::http_content_fetcher::HttpContentFetcher;
use paladin_content::services::content_fetching_service::{ContentFetchingService, FetchContent};

/// Fetch a URL into a `ContentItem`, directly and via the `FetchContent` use case.
pub fn ingest_http() -> Result<(), Box<dyn std::error::Error>> {
    let fetcher = HttpContentFetcher::new();
    // Direct use:
    let item = fetcher.fetch_content("https://example.com/article")?;

    // Or wrapped in the use case (same trait, swappable adapter):
    let fetch = FetchContent::new(HttpContentFetcher::new());
    let item = fetch.execute("https://example.com/article")?;
    Ok(())
}
// ANCHOR_END: http

// ANCHOR: news
use paladin_content::adapters::input::news_api_fetcher::NewsApiFetcher;

/// Construct a News API fetcher (feature `news-api`).
pub fn ingest_news() {
    let fetcher = NewsApiFetcher::new("YOUR_NEWS_API_KEY".to_string())
        .with_content_fetcher(HttpContentFetcher::new());
}
// ANCHOR_END: news

// ANCHOR: aggregate
use paladin_content::services::content_aggregator_service::AggregateContent;

/// Merge JSON from several sources into one aggregated value.
pub fn aggregate() {
    // `MockListService` implements the `ContentListService` trait.
    let aggregator = AggregateContent::new(MockListService);
    let source_a = serde_json::json!({ "title": "A" });
    let source_b = serde_json::json!({ "title": "B" });
    let aggregated = aggregator.execute(vec![source_a, source_b]);
}
// ANCHOR_END: aggregate

// ANCHOR: summarize
use paladin_content::services::content_summarizer_service::ContentSummarizer;

/// Summarize a `ContentItem` and extract keywords (no LLM call).
pub fn summarize() {
    let item = text_content_item("A long article body about quarterly earnings...");
    let summarizer = ContentSummarizer::new();
    let summary = summarizer.summarize_content(&item, 500); // max 500 chars
    let keywords = summarizer.extract_keywords(&item);
}
// ANCHOR_END: summarize

// ANCHOR: llm_bridge
use paladin_content::services::content_llm_analysis_service::{
    LlmContentAnalysisConfig, LlmContentAnalysisInput, LlmContentAnalyzer,
};
use paladin_llm::llm_analysis_service::LlmAnalysisService;
use paladin_llm::mock::MockLlmAdapter;
use paladin_ports::output::llm_port::LlmPort;

/// Pass content + a prompt to a Paladin LLM service for AI enrichment.
pub async fn content_to_agent() -> Result<(), Box<dyn std::error::Error>> {
    // In production this is a real provider (e.g. OpenAIAdapter); here a mock.
    let llm: Arc<dyn LlmPort> =
        Arc::new(MockLlmAdapter::new().with_response("{\"summary\":\"...\"}"));
    let llm_service = Arc::new(LlmAnalysisService::new(llm));

    let analyzer = LlmContentAnalyzer::new(llm_service);
    let input = LlmContentAnalysisInput {
        prompt: text_prompt_item("Summarize the key risks in this article."),
        content: text_content_item("Latest article body..."),
    };
    let config = LlmContentAnalysisConfig::default(); // gpt-3.5-turbo, 3 retries, 30s timeout

    let analysis = analyzer
        .analyze_with_prompt_async(&input, &config)
        .await
        .map_err(|e| -> Box<dyn std::error::Error> { e.into() })?;
    println!("{}", serde_json::to_string_pretty(&analysis)?);
    Ok(())
}
// ANCHOR_END: llm_bridge

// ANCHOR: delivery
use paladin_content::services::content_delivery_service::DeliverContentUseCase;
use paladin_ports::output::content_delivery_port::{
    ContentPayload, DeliveryMethod, DeliveryPriority, DeliveryRequest,
};

/// Deliver processed content through a `ContentDeliveryService`.
pub fn deliver() -> Result<(), Box<dyn std::error::Error>> {
    let delivery = DeliverContentUseCase::new(MockDeliveryAdapter);

    let request = DeliveryRequest {
        recipient_id: "ops-team".to_string(),
        delivery_method: DeliveryMethod::Email {
            to: "ops@example.com".to_string(),
            subject: "Daily digest".to_string(),
        },
        content_payload: ContentPayload::SingleItem(text_content_item("Digest body...")),
        priority: DeliveryPriority::Normal,
        scheduled_time: None,
        metadata: None,
    };

    let response = delivery.execute(request)?;
    println!("delivery status: {:?}", response.status);
    Ok(())
}
// ANCHOR_END: delivery
