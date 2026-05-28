# paladin-content

Content ingestion and processing services for the Paladin framework.

## Purpose

`paladin-content` provides content acquisition adapters and content-processing use cases for downstream analysis pipelines.

## Key Modules

- `adapters`: Source-specific fetchers and ingestion integrations.
- `use_cases`: Processing workflows and orchestration logic.

## Usage

```rust
use paladin_content::adapters;
use paladin_content::use_cases;

// Compose content adapters with processing use cases in application services.
let _adapters_module = std::any::type_name::<adapters::web_content_adapter::WebContentAdapter>();
let _use_cases_module = std::any::type_name::<use_cases::content_processing_service::ContentProcessingService>();
```

## Feature Flags

- `pdf`: Enable PDF extraction and parsing support.
- `web-scraping`: Enable HTML scraping adapters.
- `rss`: Enable RSS feed parsing support.
- `news-api`: Enable NewsAPI integration.
- `tiktoken`: Enable token-counting utilities.
- `llm`: Enable LLM-backed content analysis paths.
