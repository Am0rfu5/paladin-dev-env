# Product Requirements Document: Vision Pipeline Completion

## Document Information

- **Feature Name:** Vision Pipeline Completion
- **Epic:** Epic 20 - Milestone 3
- **Version:** 1.0
- **Created:** February 10, 2026
- **Status:** Draft
- **Priority:** High
- **Dependencies:** Epic 19 (Herald & Domain Type Consolidation)
- **Estimated Duration:** 1–2 weeks

---

## 1. Introduction/Overview

### Problem Statement

Epic 13 established the foundation for multi-modal vision capabilities in the Paladin framework, including type definitions and adapter scaffolding. However, the actual API integration with OpenAI and Anthropic vision endpoints was deferred, leaving `TODO` stubs in place. Without complete vision API calls and execution service integration, Paladins cannot perform image analysis tasks.

### Solution

Complete the vision pipeline by implementing real HTTP API calls to OpenAI GPT-4 Vision and Anthropic Claude Vision endpoints, integrating vision execution into `PaladinExecutionService`, and ensuring proper error handling, retry logic, and testing infrastructure.

### Goal

Enable Paladins to analyze images using state-of-the-art vision models through a robust, production-ready integration that respects the framework's hexagonal architecture and follows established patterns for LLM adapter implementation.

---

## 2. Goals

1. **Complete OpenAI Vision Integration:** Implement actual API calls to OpenAI's Chat Completions endpoint with vision support (GPT-4 Vision, GPT-4o)
2. **Complete Anthropic Vision Integration:** Implement actual API calls to Anthropic's Messages endpoint with vision support (Claude 3 Vision models)
3. **Wire Vision Execution:** Integrate vision processing into `PaladinExecutionService` for end-to-end image analysis
4. **Robust Error Handling:** Handle API-specific errors (rate limits, invalid images, unsupported formats) gracefully
5. **Configurable Retry Logic:** Implement retry with configurable exponential backoff for transient failures
6. **Testability:** Create comprehensive unit tests with mocked HTTP responses and environment-gated integration tests
7. **Configuration Management:** Support configurable retry parameters via application settings

---

## 3. User Stories

### US-20.1: Implement OpenAI Vision API Call

**As a** developer  
**I want** the OpenAI vision adapter to make actual API calls  
**So that** image analysis works with GPT-4 Vision

**Acceptance Criteria:**
- [ ] `OpenAIVisionAdapter` sends multimodal content (image_url parts) to OpenAI Chat Completions API
- [ ] Supports both URL-based and base64-encoded images
- [ ] Delegates image format validation to the OpenAI API (supports all formats accepted by the provider)
- [ ] Parses and returns structured vision response as `VisionResponse`
- [ ] Handles API errors: rate limits (429), invalid images (400), authentication (401), server errors (500+)
- [ ] Implements retry with configurable exponential backoff (max retries and intervals from config)
- [ ] Unit tests with mocked HTTP responses for success and all error cases
- [ ] Integration test structure gated by environment variable (`ENABLE_VISION_TESTS=true`)

**Source Files:**
- `src/infrastructure/adapters/llm/openai_vision.rs` — line 212

---

### US-20.2: Implement Anthropic Vision API Call

**As a** developer  
**I want** the Anthropic vision adapter to make actual API calls  
**So that** image analysis works with Claude Vision

**Acceptance Criteria:**
- [ ] `AnthropicVisionAdapter` sends multimodal content blocks to Anthropic Messages API
- [ ] Supports both URL-based and base64-encoded images
- [ ] Handles Anthropic-specific content block format: `{type: "image", source: {type, media_type, data}}`
- [ ] Delegates image format validation to the Anthropic API (supports all formats accepted by the provider)
- [ ] Parses and returns structured vision response as `VisionResponse`
- [ ] Handles API errors appropriately (rate limits, invalid content, authentication, server errors)
- [ ] Implements retry with configurable exponential backoff
- [ ] Unit tests with mocked HTTP responses for success and all error cases
- [ ] Integration test structure gated by environment variable (`ENABLE_VISION_TESTS=true`)

**Source Files:**
- `src/infrastructure/adapters/llm/anthropic_vision.rs` — line 220

---

### US-20.3: Complete Vision Execution in PaladinExecutionService

**As a** developer  
**I want** the execution service to perform full vision processing  
**So that** Paladins can analyze images end-to-end

**Acceptance Criteria:**
- [ ] `PaladinExecutionService::execute_with_vision()` builds multimodal prompts from text + images
- [ ] Vision provider selected based on Paladin's configured LLM provider (same provider for text and vision)
- [ ] Calls the appropriate vision adapter (`OpenAIVisionAdapter` or `AnthropicVisionAdapter`)
- [ ] Parses vision results and integrates into the reasoning loop
- [ ] Respects existing execution configuration: `max_loops`, `stop_words`, `timeout_seconds`
- [ ] Vision responses are **non-streaming** (complete analysis returned before proceeding)
- [ ] Updates garrison with vision interaction context if garrison is configured
- [ ] Unit tests with mock vision adapter verify full execution flow
- [ ] Example updated: `examples/sentinel_vision.rs` demonstrates real image analysis

**Source Files:**
- `src/application/use_cases/paladin/paladin_execution_service.rs` — line 371
- `examples/sentinel_vision.rs`

---

## 4. Functional Requirements

### FR-1: OpenAI Vision Adapter Implementation

**FR-1.1:** `OpenAIVisionAdapter::analyze_image()` must construct HTTP POST request to `https://api.openai.com/v1/chat/completions`

**FR-1.2:** Request must include:
- `model`: Vision-capable model (e.g., `"gpt-4-vision-preview"`, `"gpt-4o"`)
- `messages`: Array with system prompt and user message containing text + image content
- `max_tokens`: Configurable via adapter settings
- Headers: `Authorization: Bearer {api_key}`, `Content-Type: application/json`

**FR-1.3:** Image content must support two formats:
- **URL-based:** `{"type": "image_url", "image_url": {"url": "https://..."}}`
- **Base64-encoded:** `{"type": "image_url", "image_url": {"url": "data:image/jpeg;base64,..."}}`

**FR-1.4:** Must parse successful response (200 OK) and extract:
- Vision analysis text from `choices[0].message.content`
- Token usage from `usage` object
- Model used from `model` field

**FR-1.5:** Must handle error responses:
- **400 Bad Request:** Invalid image format or unsupported content → `VisionError::InvalidImage`
- **401 Unauthorized:** Invalid API key → `VisionError::AuthenticationError`
- **429 Too Many Requests:** Rate limit exceeded → Trigger retry with backoff
- **500+ Server Errors:** OpenAI service issues → Trigger retry with backoff

**FR-1.6:** Retry logic must:
- Read `max_retries` from configuration (default: 3)
- Read `initial_backoff_ms` and `backoff_multiplier` from configuration
- Calculate delay: `initial_backoff_ms * (backoff_multiplier ^ retry_attempt)`
- Only retry on transient errors (429, 500, 502, 503, 504)
- Not retry on permanent errors (400, 401, 403, 404)

---

### FR-2: Anthropic Vision Adapter Implementation

**FR-2.1:** `AnthropicVisionAdapter::analyze_image()` must construct HTTP POST request to `https://api.anthropic.com/v1/messages`

**FR-2.2:** Request must include:
- `model`: Vision-capable model (e.g., `"claude-3-opus-20240229"`, `"claude-3-sonnet-20240229"`)
- `max_tokens`: Configurable via adapter settings
- `messages`: Array with user role containing text + image content blocks
- Headers: `x-api-key: {api_key}`, `anthropic-version: 2023-06-01`, `Content-Type: application/json`

**FR-2.3:** Image content must use Anthropic's content block format:
```json
{
  "type": "image",
  "source": {
    "type": "base64" | "url",
    "media_type": "image/jpeg" | "image/png" | "image/gif" | "image/webp",
    "data": "<base64_string>" // for base64 type
    // OR
    "url": "<image_url>" // for url type
  }
}
```

**FR-2.4:** Must parse successful response (200 OK) and extract:
- Vision analysis text from `content[0].text`
- Token usage from `usage` object
- Model used from `model` field

**FR-2.5:** Must handle error responses:
- **400 Bad Request:** Invalid content → `VisionError::InvalidImage`
- **401 Unauthorized:** Invalid API key → `VisionError::AuthenticationError`
- **429 Too Many Requests:** Rate limit exceeded → Trigger retry with backoff
- **500+ Server Errors:** Anthropic service issues → Trigger retry with backoff

**FR-2.6:** Retry logic must follow same pattern as FR-1.6 with configurable parameters

---

### FR-3: Vision Execution Service Integration

**FR-3.1:** `PaladinExecutionService::execute_with_vision()` must accept:
- `paladin: &Paladin` - Agent configuration
- `prompt: &str` - Text prompt/question about the image
- `images: Vec<VisionImage>` - One or more images to analyze

**FR-3.2:** Must construct multimodal prompt combining:
- Paladin's system prompt
- User's text prompt
- Image references (URLs or base64 data)

**FR-3.3:** Provider selection logic:
- Extract provider from `paladin.model()` (e.g., `"gpt-4"` → OpenAI, `"claude-3"` → Anthropic)
- Use same provider for vision as configured for text LLM
- Return `VisionError::UnsupportedProvider` if provider doesn't support vision

**FR-3.4:** Must invoke appropriate vision adapter:
- `OpenAIVisionAdapter` for OpenAI models
- `AnthropicVisionAdapter` for Anthropic models

**FR-3.5:** Vision execution must be **non-streaming**:
- Wait for complete vision analysis response
- Parse entire response before continuing
- Do not stream partial vision results

**FR-3.6:** Must respect execution configuration:
- `max_loops`: Vision analysis counts as one reasoning loop iteration
- `stop_words`: Check vision response for stop words
- `timeout_seconds`: Vision API call subject to overall timeout

**FR-3.7:** If garrison is configured, must store:
- Original prompt + images
- Vision analysis result
- Timestamp and metadata

**FR-3.8:** Must return `VisionResult` containing:
- Analysis text
- Token usage
- Model used
- Execution metadata (duration, loops used)

---

### FR-4: Configuration Management

**FR-4.1:** Add vision retry configuration to `config.yml`:
```yaml
vision:
  retry:
    max_retries: 3
    initial_backoff_ms: 1000
    backoff_multiplier: 2.0
  openai:
    max_tokens: 4096
  anthropic:
    max_tokens: 4096
```

**FR-4.2:** Configuration must be loaded into `VisionConfig` struct in `src/config/application_settings.rs`

**FR-4.3:** Adapters must read retry parameters from config via constructor injection

---

### FR-5: Error Handling

**FR-5.1:** Define `VisionError` enum in `src/core/platform/container/sentinel/vision_types.rs`:
```rust
#[derive(Debug, thiserror::Error)]
pub enum VisionError {
    #[error("Invalid image: {0}")]
    InvalidImage(String),
    #[error("Unsupported image format: {0}")]
    UnsupportedFormat(String),
    #[error("Authentication failed: {0}")]
    AuthenticationError(String),
    #[error("Rate limit exceeded, retry after {0}s")]
    RateLimitExceeded(u64),
    #[error("Provider error: {0}")]
    ProviderError(String),
    #[error("Network error: {0}")]
    NetworkError(String),
    #[error("Timeout after {0}s")]
    Timeout(u64),
    #[error("Unsupported provider: {0}")]
    UnsupportedProvider(String),
    #[error("Max retries exceeded: {0}")]
    MaxRetriesExceeded(String),
}
```

**FR-5.2:** All adapter methods must return `Result<VisionResponse, VisionError>`

**FR-5.3:** Execution service must convert `VisionError` to `PaladinError` at boundary

---

### FR-6: Testing Requirements

**FR-6.1:** Unit tests for `OpenAIVisionAdapter`:
- [ ] Test successful API call with URL-based image
- [ ] Test successful API call with base64-encoded image
- [ ] Test multiple images in single request
- [ ] Test error handling: 400, 401, 429, 500 responses
- [ ] Test retry logic: exponential backoff calculation
- [ ] Test max retries exceeded scenario
- [ ] Mock HTTP client using `mockito` or similar

**FR-6.2:** Unit tests for `AnthropicVisionAdapter`:
- [ ] Test successful API call with URL-based image
- [ ] Test successful API call with base64-encoded image
- [ ] Test Anthropic content block format
- [ ] Test error handling: 400, 401, 429, 500 responses
- [ ] Test retry logic
- [ ] Mock HTTP client

**FR-6.3:** Unit tests for `PaladinExecutionService::execute_with_vision()`:
- [ ] Test provider selection (OpenAI vs Anthropic)
- [ ] Test multimodal prompt construction
- [ ] Test vision result integration into reasoning loop
- [ ] Test garrison storage of vision interactions
- [ ] Test timeout enforcement
- [ ] Test stop word detection in vision responses
- [ ] Mock vision adapters

**FR-6.4:** Integration tests (gated by `ENABLE_VISION_TESTS` environment variable):
- [ ] Test real OpenAI API call with sample image (requires `OPENAI_API_KEY`)
- [ ] Test real Anthropic API call with sample image (requires `ANTHROPIC_API_KEY`)
- [ ] Test end-to-end vision execution via `PaladinExecutionService`
- [ ] Use sample images from `tests/fixtures/` directory
- [ ] Document how to run integration tests in `tests/integration/vision_integration_test.rs`

---

### FR-7: Example and Documentation

**FR-7.1:** Update `examples/sentinel_vision.rs` to demonstrate:
- Building a Paladin with vision capabilities
- Analyzing a single image with URL
- Analyzing a single image with base64 encoding
- Analyzing multiple images in one request
- Error handling patterns

**FR-7.2:** Document vision capabilities in `docs/SENTINEL.md`:
- Supported providers and models
- Image format requirements
- Configuration options
- Error handling patterns
- Example usage

---

## 5. Non-Goals (Out of Scope)

### NG-1: Vision Streaming
- Vision responses will **not** support streaming in this epic
- Complete analysis must be returned before proceeding
- Streaming can be added in future enhancement if needed

### NG-2: Custom Vision Models
- Only OpenAI and Anthropic official vision models supported
- No support for self-hosted or fine-tuned vision models
- Custom model support deferred to future enhancement

### NG-3: Advanced Image Processing
- No image preprocessing (resizing, cropping, filtering)
- No OCR-specific optimizations
- No object detection bounding boxes
- Adapters accept images as-is and delegate all processing to API

### NG-4: Vision-Specific Prompting Strategies
- No automatic prompt engineering for vision tasks
- No vision-specific system prompts or templates
- User/developer provides prompts; adapter sends them as-is

### NG-5: Image Format Conversion
- Adapters do not convert image formats (e.g., PNG to JPEG)
- Image format validation delegated to API providers
- Conversion can be done by caller before passing to adapter

### NG-6: Batch Vision Processing
- No specialized batch processing API for multiple images
- Concurrent analysis of multiple images handled at Battalion level
- Single execution service call processes one prompt + images

### NG-7: Vision Result Caching
- No caching of vision analysis results by image hash
- Each request makes fresh API call
- Caching can be added as future optimization

---

## 6. Design Considerations

### DC-1: Provider Parity
- OpenAI and Anthropic adapters should follow similar patterns
- Both implement same `VisionPort` trait
- Error handling and retry logic should be consistent

### DC-2: Hexagonal Architecture
- Vision adapters are infrastructure layer components
- `VisionPort` trait defined in application layer
- Core domain types (`VisionImage`, `VisionResponse`) in core layer
- No core/application dependencies on infrastructure

### DC-3: Configuration-Driven Behavior
- All retry parameters configurable via `config.yml`
- Model selection and max_tokens configurable per provider
- Default values reasonable for production use

### DC-4: Type Safety
- Use strong types for image sources (`ImageUrl`, `Base64Image`)
- Vision responses use structured types, not raw strings
- Provider-specific types converted to domain types at adapter boundary

---

## 7. Technical Considerations

### TC-1: HTTP Client
- Use existing `reqwest` HTTP client (already a project dependency)
- Reuse HTTP client instance across requests (connection pooling)
- Configure timeouts at HTTP client level

### TC-2: Dependencies
- No new external dependencies required beyond existing ones
- `reqwest`, `serde`, `serde_json`, `tokio` already in `Cargo.toml`
- `mockito` for HTTP mocking in tests (dev dependency)

### TC-3: API Key Management
- OpenAI API key: Read from `OPENAI_API_KEY` environment variable
- Anthropic API key: Read from `ANTHROPIC_API_KEY` environment variable
- Both loaded via `ApplicationSettings` struct
- Never log or expose API keys in error messages

### TC-4: Image Size Limits
- Delegate image size validation to API providers
- OpenAI limit: ~20MB per image
- Anthropic limit: varies by model
- Document limits in `docs/SENTINEL.md`

### TC-5: Token Usage Tracking
- Both adapters track prompt tokens and completion tokens
- Return token usage in `VisionResponse`
- Execution service aggregates token usage across loops

### TC-6: Retry State Management
- Retry logic is stateless (no persistent retry state)
- Each request's retries independent
- Retry count and backoff calculated per-request

### TC-7: Async/Await
- All adapter methods are `async`
- Use `tokio::time::sleep` for retry backoff
- Respect Rust async best practices (no blocking I/O)

### TC-8: Testing Isolation
- Unit tests mock HTTP layer completely
- Integration tests isolated via environment variable gate
- Test fixtures (sample images) committed to `tests/fixtures/`

---

## 8. Success Metrics

### SM-1: Functional Completeness
- [ ] All three user stories (US-20.1, US-20.2, US-20.3) acceptance criteria met
- [ ] All functional requirements (FR-1 through FR-7) implemented
- [ ] Zero `TODO` comments remaining in vision-related files

### SM-2: Code Quality
- [ ] All unit tests pass: `cargo test`
- [ ] All clippy warnings resolved: `cargo clippy`
- [ ] Code formatted: `cargo fmt --check`
- [ ] Test coverage ≥80% for vision adapters and execution service integration

### SM-3: Integration Validation
- [ ] Integration tests pass with real API keys (when `ENABLE_VISION_TESTS=true`)
- [ ] Example `examples/sentinel_vision.rs` runs successfully with real images
- [ ] Documentation in `docs/SENTINEL.md` complete and accurate

### SM-4: Error Resilience
- [ ] Retry logic successfully handles transient failures (429, 500)
- [ ] Permanent errors (400, 401) fail fast without unnecessary retries
- [ ] All error cases covered by tests

### SM-5: Performance
- [ ] Vision API calls complete within reasonable timeframe (< 30s typical)
- [ ] Retry backoff does not cause excessive delays (< 60s total retry time)
- [ ] No memory leaks in long-running vision executions

---

## 9. Open Questions

### OQ-1: Image Preprocessing
- **Question:** Should we provide utility functions for common image preprocessing (resize, optimize)?
- **Impact:** Would improve usability but increases scope
- **Recommendation:** Defer to future enhancement; document preprocessing best practices

### OQ-2: Vision Model Selection
- **Question:** Should we auto-select vision-specific models (e.g., `gpt-4-vision-preview` vs `gpt-4`)?
- **Impact:** Would simplify configuration but reduces explicit control
- **Recommendation:** Require explicit vision model in config; validate model supports vision

### OQ-3: Multi-Image Analysis Strategies
- **Question:** Should we provide guidance on optimal prompt strategies for multi-image analysis?
- **Impact:** Would improve results quality but requires research
- **Recommendation:** Document basic patterns in examples; let users experiment

### OQ-4: Vision Result Structured Extraction
- **Question:** Should we support extracting structured data (JSON) from vision responses?
- **Impact:** Would enable downstream processing but complicates parsing
- **Recommendation:** Support raw text responses; users can add JSON extraction in prompts

### OQ-5: Cost Tracking
- **Question:** Should we track estimated API costs for vision calls?
- **Impact:** Useful for budget management but requires pricing data maintenance
- **Recommendation:** Defer to future enhancement; track tokens only

---

## 10. Implementation Phases

### Phase 1: OpenAI Vision Adapter (Days 1-3)
1. Implement `OpenAIVisionAdapter::analyze_image()`
2. Add retry logic with configurable backoff
3. Write unit tests with mocked HTTP responses
4. Add configuration support for OpenAI vision settings

### Phase 2: Anthropic Vision Adapter (Days 4-6)
1. Implement `AnthropicVisionAdapter::analyze_image()`
2. Handle Anthropic-specific content block format
3. Add retry logic
4. Write unit tests with mocked HTTP responses
5. Add configuration support for Anthropic vision settings

### Phase 3: Execution Service Integration (Days 7-8)
1. Implement `PaladinExecutionService::execute_with_vision()`
2. Add provider selection logic
3. Integrate garrison storage
4. Write unit tests with mock adapters

### Phase 4: Integration Tests & Examples (Days 9-10)
1. Create integration tests with environment variable gating
2. Update `examples/sentinel_vision.rs`
3. Add test fixtures (sample images)
4. Document integration test setup

### Phase 5: Documentation & Polish (Days 11-12)
1. Update `docs/SENTINEL.md` with vision capabilities
2. Add inline documentation to all public APIs
3. Final testing and validation
4. Submit PR for review

---

## Appendix A: File Structure

```
src/
├── core/platform/container/sentinel/
│   └── vision_types.rs          # VisionError enum additions
├── infrastructure/adapters/llm/
│   ├── openai_vision.rs         # FR-1: OpenAI implementation
│   └── anthropic_vision.rs      # FR-2: Anthropic implementation
├── application/use_cases/paladin/
│   └── paladin_execution_service.rs  # FR-3: Vision execution
└── config/
    └── application_settings.rs  # FR-4: Vision config

tests/
├── integration/
│   └── vision_integration_test.rs  # FR-6.4: Integration tests
└── fixtures/
    ├── sample_image.jpg         # Test image
    └── sample_image.png         # Test image

examples/
└── sentinel_vision.rs           # FR-7.1: Updated example

docs/
└── SENTINEL.md                  # FR-7.2: Documentation

config.yml                       # FR-4.1: Vision configuration
```

---

## Appendix B: Configuration Example

```yaml
# config.yml - Vision section

vision:
  retry:
    max_retries: 3
    initial_backoff_ms: 1000
    backoff_multiplier: 2.0
  
  openai:
    max_tokens: 4096
    models:
      - "gpt-4-vision-preview"
      - "gpt-4o"
      - "gpt-4o-mini"
  
  anthropic:
    max_tokens: 4096
    models:
      - "claude-3-opus-20240229"
      - "claude-3-sonnet-20240229"
      - "claude-3-haiku-20240307"
```

---

## Appendix C: Example Usage

```rust
use paladin::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Build Paladin with vision-capable model
    let llm_port = Arc::new(OpenAIAdapter::new(config.clone()));
    let vision_adapter = Arc::new(OpenAIVisionAdapter::new(config.clone()));
    
    let paladin = PaladinBuilder::new(llm_port)
        .name("Vision Analyst")
        .system_prompt("You are an expert image analyst.")
        .model("gpt-4-vision-preview")
        .build()?;
    
    // Analyze image
    let image = VisionImage::from_url("https://example.com/chart.png")?;
    
    let result = paladin_execution_service
        .execute_with_vision(
            &paladin,
            "What trends do you see in this chart?",
            vec![image],
        )
        .await?;
    
    println!("Analysis: {}", result.content);
    println!("Tokens used: {}", result.token_usage.total_tokens);
    
    Ok(())
}
```

---

**End of Document**
