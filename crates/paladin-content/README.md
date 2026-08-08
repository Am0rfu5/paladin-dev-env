# paladin-content

Content ingestion and processing services for the Paladin framework.

## Purpose

`paladin-content` provides content acquisition adapters and content-processing use cases for downstream analysis pipelines.

## Key Modules

- `adapters`: Source-specific fetchers and ingestion integrations.
- `services`: Processing workflows and orchestration logic.

## Usage

```rust
use paladin_content::adapters;
use paladin_content::services;

// Compose content adapters with processing services in application services.
let _adapters_module = std::any::type_name::<adapters::web_content_adapter::WebContentAdapter>();
let _services_module = std::any::type_name::<services::content_processing_service::ContentProcessingService>();
```

## Feature Flags

**Corrected (dated 2026-08-08, HARD-06, plan 10-11):** the `pdf` feature no longer exists — it
gated no dependency and no code, and was deleted (see ADR-0032,
`.planning/decisions/0032-pdf-extraction-capability.md`). PDF extraction ships unconditionally in
every build of `paladin-content`, via the crate's own unconditional `pdf-extract` dependency.

- `web-scraping`: Enable HTML scraping adapters.
- `rss`: Enable RSS feed parsing support.
- `news-api`: Enable NewsAPI integration.
- `tiktoken`: Enable token-counting utilities.
- `llm`: Enable LLM-backed content analysis paths.
