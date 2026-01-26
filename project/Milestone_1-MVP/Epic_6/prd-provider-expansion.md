# Product Requirements Document: LLM Provider Expansion

**Epic:** Epic 6 - Provider Expansion  
**Priority:** High  
**Estimated Effort:** 2-3 weeks  
**Dependencies:** Epic 1 (Paladin Domain Foundation)  
**Target Audience:** Junior to Mid-level Rust Developers  
**Document Version:** 1.0  
**Date:** January 25, 2026

---

## 1. Introduction/Overview

### Problem Statement

Currently, Paladin only supports OpenAI as an LLM provider. This creates several limitations:
- **Vendor Lock-in:** Users cannot switch providers without significant code changes
- **Cost Constraints:** Organizations cannot optimize costs by using different models for different tasks
- **Feature Limitations:** Users cannot leverage provider-specific capabilities (e.g., Anthropic's extended context, DeepSeek's cost efficiency)
- **Reliability Risks:** No fallback options if a provider experiences downtime

### Solution

Expand Paladin's LLM provider support to include **DeepSeek** and **Anthropic (Claude)** while establishing a clean architecture pattern for adding future providers. This will enable per-Paladin provider configuration, allowing different agents to use different models based on their specific requirements.

### Goal

Enable Paladin to support multiple LLM providers through a provider-agnostic architecture that allows developers to:
1. Configure different providers for different Paladin instances
2. Switch providers via configuration without code changes
3. Leverage provider-specific capabilities through a feature detection API
4. Maintain backward compatibility with existing OpenAI integrations

---

## 2. Goals

### Primary Goals

1. **Multi-Provider Support**
   - Implement fully functional DeepSeek adapter with streaming support
   - Implement fully functional Anthropic adapter with Claude-specific message formatting
   - Maintain existing OpenAI adapter with no regressions

2. **Per-Paladin Configuration**
   - Allow each Paladin instance to specify its own LLM provider
   - Support provider configuration via config files (YAML/TOML)
   - Enable runtime provider selection without code changes

3. **Feature Detection API**
   - Expose provider capabilities (streaming, tool calling, embeddings, vision, etc.)
   - Allow developers to query provider features before execution
   - Provide graceful degradation when features are unavailable

4. **Backward Compatibility**
   - Existing Paladin code continues to work without modifications
   - Minimal migration effort (config file updates only)
   - Default behavior remains unchanged (OpenAI if configured)

### Secondary Goals

5. **Testing Infrastructure**
   - Unit tests with mocked provider responses (≥80% coverage)
   - Optional live API integration tests for CI/CD
   - Local test server support for offline development

6. **Documentation**
   - Provider comparison guide (features, pricing, use cases)
   - Migration guide for adding new providers
   - Configuration examples for each provider

---

## 3. User Stories

### Story 1: Basic Provider Configuration
**As a** Paladin developer  
**I want to** configure which LLM provider a Paladin uses via config file  
**So that** I can choose the best provider for each agent without changing code

**Acceptance Criteria:**
- Configuration file specifies provider (openai/deepseek/anthropic)
- Paladin builder accepts provider configuration
- Invalid provider names produce clear error messages
- Default provider (OpenAI) used if not specified

---

### Story 2: Per-Agent Provider Selection
**As a** system architect  
**I want to** assign different providers to different Paladins in the same application  
**So that** I can optimize costs by using cheaper models for simple tasks and advanced models for complex reasoning

**Acceptance Criteria:**
- Multiple Paladins can run simultaneously with different providers
- Provider selection is per-instance, not global
- No conflicts between provider configurations
- Each Paladin maintains its own provider connection/client

---

### Story 3: DeepSeek Integration
**As a** developer on a budget  
**I want to** use DeepSeek's cost-effective models for my Paladins  
**So that** I can reduce LLM costs while maintaining good performance

**Acceptance Criteria:**
- DeepSeek adapter implements core LlmPort trait
- Supports standard completions and streaming
- API key configuration via environment variables or config file
- Model validation ensures specified models exist
- Error messages are clear and actionable

---

### Story 4: Anthropic Claude Integration
**As a** developer building complex reasoning systems  
**I want to** use Anthropic's Claude models for advanced reasoning tasks  
**So that** I can leverage Claude's extended context windows and superior reasoning

**Acceptance Criteria:**
- Anthropic adapter implements core LlmPort trait
- Correctly formats messages for Claude's API (system, user, assistant roles)
- Supports streaming responses
- Handles Claude-specific parameters (max_tokens required)
- Tool use support for function calling (if Claude API supports it)

---

### Story 5: Feature Detection
**As a** developer  
**I want to** query what features a provider supports  
**So that** I can write code that adapts to provider capabilities

**Acceptance Criteria:**
- `get_capabilities()` method on LlmPort returns provider features
- Features include: streaming, tool_calling, vision, embeddings, function_calling
- Documentation explains how to use capability detection
- Example code demonstrates graceful degradation

---

### Story 6: Provider Comparison
**As a** decision maker  
**I want to** understand the trade-offs between providers  
**So that** I can make informed choices about which provider to use

**Acceptance Criteria:**
- Documentation includes provider comparison table
- Comparison covers: pricing, speed, context limits, special features
- Use case recommendations for each provider
- Performance benchmarks (if available)

---

### Story 7: Easy Provider Addition
**As a** contributor  
**I want to** add support for a new LLM provider  
**So that** the community can expand Paladin's capabilities

**Acceptance Criteria:**
- Clear documentation on implementing LlmPort trait
- Example adapter template with TODOs
- Test harness for validating new providers
- Contribution guide for submitting new providers

---

## 4. Functional Requirements

### 4.1 Core Provider Interface

**REQ-1:** All LLM providers MUST implement the `LlmPort` trait with the following methods:
- `async fn generate(&self, request: LlmRequest) -> Result<LlmResponse, LlmError>`
- `async fn generate_stream(&self, request: LlmRequest) -> Result<Box<dyn Stream<Item = Result<StreamingResponse, LlmError>> + Send>, LlmError>`
- `async fn validate_model(&self, model: &str) -> Result<bool, LlmError>`
- `async fn get_available_models(&self) -> Result<Vec<String>, LlmError>`
- `fn get_provider_name(&self) -> &'static str`
- `fn get_capabilities(&self) -> ProviderCapabilities`

**REQ-2:** The `ProviderCapabilities` struct MUST expose the following features:
```rust
pub struct ProviderCapabilities {
    pub supports_streaming: bool,
    pub supports_tool_calling: bool,
    pub supports_function_calling: bool,
    pub supports_vision: bool,
    pub supports_embeddings: bool,
    pub max_context_tokens: Option<u32>,
    pub supports_system_messages: bool,
}
```

### 4.2 DeepSeek Adapter

**REQ-3:** DeepSeek adapter MUST be implemented at `src/infrastructure/adapters/llm/deepseek_adapter.rs`

**REQ-4:** DeepSeek adapter MUST support the following configuration:
```rust
pub struct DeepSeekConfig {
    pub api_key: String,
    pub base_url: String,
    pub model: String,
    pub timeout_seconds: u64,
}
```

**REQ-5:** DeepSeek adapter MUST support standard completion requests with:
- Temperature control (0.0-2.0)
- Max tokens limit
- Top-p sampling
- Frequency/presence penalties

**REQ-6:** DeepSeek adapter MUST support streaming responses via Server-Sent Events (SSE)

**REQ-7:** DeepSeek adapter MUST validate API keys before making requests and return clear error messages for authentication failures

**REQ-8:** DeepSeek adapter MUST map DeepSeek-specific errors to Paladin's `LlmError` enum

### 4.3 Anthropic Adapter

**REQ-9:** Anthropic adapter MUST be implemented at `src/infrastructure/adapters/llm/anthropic_adapter.rs`

**REQ-10:** Anthropic adapter MUST support the following configuration:
```rust
pub struct AnthropicConfig {
    pub api_key: String,
    pub base_url: String,
    pub model: String,
    pub max_tokens: u32,
}
```

**REQ-11:** Anthropic adapter MUST correctly format messages for Claude API:
- System messages sent via `system` parameter (not in messages array)
- User/assistant messages alternate in messages array
- Handle multi-turn conversations correctly

**REQ-12:** Anthropic adapter MUST require `max_tokens` parameter (Claude API requirement)

**REQ-13:** Anthropic adapter MUST support streaming via SSE

**REQ-14:** If Claude supports tool/function calling, Anthropic adapter SHOULD implement tool use formatting

**REQ-15:** Anthropic adapter MUST handle Claude-specific rate limits and retry logic

### 4.4 Configuration System

**REQ-16:** Provider configuration MUST be supported in `config.yml`:
```yaml
llm:
  openai:
    api_key: "${OPENAI_API_KEY}"
    base_url: "https://api.openai.com/v1"
    default_model: "gpt-4"
    timeout_seconds: 60
  
  deepseek:
    api_key: "${DEEPSEEK_API_KEY}"
    base_url: "https://api.deepseek.com/v1"
    default_model: "deepseek-chat"
    timeout_seconds: 60
  
  anthropic:
    api_key: "${ANTHROPIC_API_KEY}"
    base_url: "https://api.anthropic.com/v1"
    default_model: "claude-3-5-sonnet-20241022"
    max_tokens: 4096
```

**REQ-17:** Paladin builder MUST accept provider specification:
```rust
let paladin = PaladinBuilder::new(llm_port)
    .provider("deepseek")  // or "openai", "anthropic"
    .model("deepseek-chat")
    .build()?;
```

**REQ-18:** Environment variables MUST be supported for API keys (e.g., `OPENAI_API_KEY`, `DEEPSEEK_API_KEY`, `ANTHROPIC_API_KEY`)

**REQ-19:** Provider selection MUST fail fast with clear error if provider not configured or API key missing

### 4.5 Backward Compatibility

**REQ-20:** Existing Paladin code without provider specification MUST continue to work using OpenAI as default (if configured)

**REQ-21:** Existing config files without provider sections MUST continue to work

**REQ-22:** Breaking changes MUST NOT be introduced to public APIs in Epic 1

### 4.6 Error Handling

**REQ-23:** All provider adapters MUST map provider-specific errors to standardized `LlmError` variants:
- `AuthenticationError` - Invalid API keys
- `RateLimitError` - Rate limit exceeded
- `ModelNotFoundError` - Invalid model name
- `InvalidRequestError` - Malformed request
- `TimeoutError` - Request timeout
- `NetworkError` - Connection issues
- `ProviderError` - Provider-specific errors with context

**REQ-24:** Error messages MUST include actionable information (e.g., "Invalid API key for DeepSeek. Check DEEPSEEK_API_KEY environment variable.")

### 4.7 Testing

**REQ-25:** Unit tests MUST achieve ≥80% code coverage using mocked HTTP responses

**REQ-26:** Integration tests MUST be available for live API testing but SHOULD be optional (feature-flagged or CI-only)

**REQ-27:** Test suite MUST include:
- Provider configuration loading
- API key validation
- Request/response serialization
- Error mapping
- Streaming functionality
- Capability detection

**REQ-28:** Mock test servers SHOULD be provided for offline development

### 4.8 Documentation

**REQ-29:** Provider comparison documentation MUST include:
- Feature matrix (streaming, tools, context limits)
- Pricing comparison (if publicly available)
- Performance characteristics
- Use case recommendations

**REQ-30:** Migration guide MUST document:
- How to add provider configuration to existing config files
- How to update Paladin builder code for provider selection
- How to implement a new provider adapter

**REQ-31:** API documentation MUST include examples for each provider

---

## 5. Non-Goals (Out of Scope)

### Explicitly NOT Included in This Epic

1. **Automatic Provider Fallback**
   - Will NOT implement automatic switching to backup provider on failure
   - Future enhancement for Epic 11+ (Resilience & Monitoring)

2. **Embedding/Vision Support**
   - Will NOT implement provider-specific embedding endpoints in this epic
   - Will NOT implement vision/multimodal capabilities
   - Provider capabilities exposed via feature detection for future use

3. **Provider-Specific Advanced Features**
   - Will NOT implement Anthropic's Claude artifacts
   - Will NOT implement OpenAI's DALL-E integration
   - Will NOT implement DeepSeek's specialized models beyond chat

4. **Cost Tracking**
   - Will NOT implement cost tracking across providers
   - Will NOT implement budget limits or cost alerts
   - Future enhancement for Epic 11+

5. **Provider Load Balancing**
   - Will NOT implement request distribution across multiple providers
   - Will NOT implement provider health checking
   - Will NOT implement circuit breaker patterns

6. **Additional Providers**
   - Will NOT add Google Gemini, Cohere, Mistral, or other providers in this epic
   - Pattern established for community contributions

7. **Provider-Agnostic Prompt Optimization**
   - Will NOT implement automatic prompt reformatting for provider differences
   - Users responsible for provider-appropriate prompts

8. **Token Counting Abstraction**
   - Will NOT implement provider-agnostic token counting
   - Each provider uses its own tokenization

---

## 6. Design Considerations

### 6.1 Architecture Pattern

Follow **Hexagonal Architecture** with adapters pattern:

```
Application Layer (Port)
    ↓
    LlmPort trait (interface)
    ↓
Infrastructure Layer (Adapters)
    ├── OpenAiAdapter
    ├── DeepSeekAdapter
    └── AnthropicAdapter
```

### 6.2 Provider Factory Pattern

Implement a provider factory for clean instantiation:

```rust
pub struct LlmProviderFactory;

impl LlmProviderFactory {
    pub fn create(config: &LlmConfig, provider: &str) 
        -> Result<Arc<dyn LlmPort>, LlmError> {
        match provider {
            "openai" => Ok(Arc::new(OpenAiAdapter::new(config.openai.clone())?)),
            "deepseek" => Ok(Arc::new(DeepSeekAdapter::new(config.deepseek.clone())?)),
            "anthropic" => Ok(Arc::new(AnthropicAdapter::new(config.anthropic.clone())?)),
            _ => Err(LlmError::InvalidProvider(provider.to_string())),
        }
    }
}
```

### 6.3 Configuration Loading

Use existing `ApplicationSettings` pattern from `src/config/application_settings.rs` with new LLM section.

### 6.4 Streaming Architecture

All providers must use `futures::Stream` for consistency:

```rust
type LlmStream = Box<dyn Stream<Item = Result<StreamingResponse, LlmError>> + Send>;
```

### 6.5 Capability Detection Usage

Example of feature detection in application code:

```rust
let capabilities = llm_port.get_capabilities();

if capabilities.supports_streaming {
    // Use streaming
    let stream = llm_port.generate_stream(request).await?;
} else {
    // Fallback to batch
    let response = llm_port.generate(request).await?;
}
```

---

## 7. Technical Considerations

### 7.1 Dependencies

**New Dependencies Required:**

- No additional HTTP client needed (reuse `reqwest`)
- Consider `async-stream` for easier stream implementations
- Consider `mockito` or `wiremock` for HTTP mocking in tests

### 7.2 API Compatibility

**DeepSeek API:**
- Compatible with OpenAI API format
- Can reuse OpenAI request/response structures
- Different base URL and models

**Anthropic API:**
- Different message format (system separate from messages)
- Requires `max_tokens` parameter
- Different streaming format
- Tool use format differs from OpenAI

### 7.3 Rate Limiting

Each provider has different rate limits:
- OpenAI: Per-minute token limits
- DeepSeek: Per-minute request limits
- Anthropic: Per-minute request and token limits

Adapters should implement exponential backoff with jitter for rate limit errors.

### 7.4 Security

- API keys MUST be stored in environment variables or secure config
- API keys MUST NOT be logged
- API keys MUST NOT be included in error messages
- Use `secrecy` crate for sensitive data handling

### 7.5 Performance

- Reuse HTTP clients (connection pooling)
- Support request timeout configuration per provider
- Consider caching model validation results

### 7.6 Integration with Epic 1

This epic depends on Epic 1's Paladin foundation:
- `PaladinBuilder` must be extended to accept provider configuration
- `PaladinExecutionService` must use the configured provider
- No changes to core Paladin domain entity required

---

## 8. Success Metrics

### 8.1 Functional Success

- [ ] All three providers (OpenAI, DeepSeek, Anthropic) pass integration tests
- [ ] Same Paladin prompt produces valid responses from all providers
- [ ] Provider switching via config requires zero code changes
- [ ] All provider-specific errors are properly mapped and actionable

### 8.2 Code Quality

- [ ] Unit test coverage ≥80% for all new adapters
- [ ] All public APIs have rustdoc documentation
- [ ] Code passes `cargo clippy` with zero warnings
- [ ] Code formatted with `cargo fmt`

### 8.3 Performance

- [ ] Provider initialization adds <100ms overhead
- [ ] Streaming latency comparable to direct API calls
- [ ] No memory leaks in long-running streaming

### 8.4 Developer Experience

- [ ] Junior developer can add a new provider in <4 hours following guide
- [ ] Migration from single-provider to multi-provider in <30 minutes
- [ ] Error messages clearly indicate provider-specific issues

### 8.5 Documentation Quality

- [ ] Provider comparison guide complete with examples
- [ ] All three providers have working example code in `examples/`
- [ ] Migration guide tested by external developer
- [ ] API documentation includes capability detection examples

---

## 9. Open Questions

### Questions Requiring Decision Before Implementation

1. **Provider Registry Pattern**
   - Should we implement a provider registry for plugin-style provider loading?
   - Or keep simple factory pattern for Epic 6?
   - **Decision needed by:** Sprint planning

2. **Streaming Error Recovery**
   - How should partial streaming failures be handled?
   - Retry entire request or expose partial results?
   - **Decision needed by:** Technical design review

3. **Configuration Precedence**
   - If provider specified in both config file AND builder, which wins?
   - Recommendation: Builder > Config > Default
   - **Decision needed by:** Sprint planning

4. **Model Aliasing**
   - Should we support model aliases (e.g., "fast" → provider-specific fast model)?
   - Or require explicit model names per provider?
   - **Decision needed by:** Technical design review

5. **Testing with Live APIs**
   - Should CI run live API tests on every commit or scheduled?
   - How to handle API costs for testing?
   - **Decision needed by:** DevOps/CI planning

### Questions for Future Epics

6. **Provider Health Monitoring** (Epic 11+)
   - How to monitor provider availability and performance?

7. **Automatic Failover** (Epic 11+)
   - When to automatically switch providers?

8. **Cost Optimization** (Epic 12+)
   - How to track and optimize costs across providers?

---

## Appendix A: Provider API Comparison

| Feature | OpenAI | DeepSeek | Anthropic |
|---------|--------|----------|-----------|
| **Streaming** | ✅ SSE | ✅ SSE | ✅ SSE |
| **Tool Calling** | ✅ | ❓ TBD | ✅ (Different format) |
| **Vision** | ✅ | ❌ | ✅ |
| **Max Context** | 128K (GPT-4) | 64K | 200K (Claude 3) |
| **System Messages** | ✅ In array | ✅ In array | ✅ Separate param |
| **Required Params** | `messages` | `messages` | `messages`, `max_tokens` |
| **API Format** | OpenAI | OpenAI-compatible | Anthropic-specific |

---

## Appendix B: Example Usage

### Basic Provider Configuration

```rust
// config.yml
llm:
  deepseek:
    api_key: "${DEEPSEEK_API_KEY}"
    base_url: "https://api.deepseek.com/v1"
    default_model: "deepseek-chat"

// main.rs
let config = ApplicationSettings::load()?;
let llm_port = LlmProviderFactory::create(&config.llm, "deepseek")?;

let paladin = PaladinBuilder::new(llm_port)
    .system_prompt("You are a helpful assistant")
    .build()?;

let result = paladin.execute("Hello!").await?;
```

### Per-Paladin Provider Selection

```rust
let openai_port = LlmProviderFactory::create(&config.llm, "openai")?;
let deepseek_port = LlmProviderFactory::create(&config.llm, "deepseek")?;

// Expensive reasoning task
let strategist = PaladinBuilder::new(openai_port)
    .system_prompt("You are a strategic advisor")
    .model("gpt-4")
    .build()?;

// Simple classification task
let classifier = PaladinBuilder::new(deepseek_port)
    .system_prompt("Classify this text")
    .model("deepseek-chat")
    .build()?;
```

### Feature Detection

```rust
let capabilities = llm_port.get_capabilities();

if capabilities.supports_tool_calling {
    paladin = paladin.add_tool(web_search_tool);
} else {
    println!("Warning: Tool calling not supported by provider");
}

if let Some(max_tokens) = capabilities.max_context_tokens {
    println!("Provider supports up to {} tokens", max_tokens);
}
```

---

## Appendix C: Testing Strategy

### Unit Tests (Mocked)

```rust
#[tokio::test]
async fn test_deepseek_basic_completion() {
    let mock_server = MockServer::start().await;
    
    Mock::given(method("POST"))
        .and(path("/v1/chat/completions"))
        .respond_with(ResponseTemplate::new(200)
            .set_body_json(json!({
                "choices": [{
                    "message": {
                        "content": "Hello, I'm DeepSeek!"
                    }
                }]
            })))
        .mount(&mock_server)
        .await;
    
    let adapter = DeepSeekAdapter::new(DeepSeekConfig {
        api_key: "test-key".into(),
        base_url: mock_server.uri(),
        model: "deepseek-chat".into(),
        timeout_seconds: 30,
    }).unwrap();
    
    let response = adapter.generate(LlmRequest {
        messages: vec![Message::user("Hello!")],
        temperature: 0.7,
        ..Default::default()
    }).await.unwrap();
    
    assert_eq!(response.content, "Hello, I'm DeepSeek!");
}
```

### Integration Tests (Optional, Live API)

```rust
#[tokio::test]
#[ignore] // Only run with --ignored flag
async fn test_deepseek_live_api() {
    let api_key = env::var("DEEPSEEK_API_KEY")
        .expect("DEEPSEEK_API_KEY required for integration test");
    
    let adapter = DeepSeekAdapter::new(DeepSeekConfig {
        api_key,
        base_url: "https://api.deepseek.com/v1".into(),
        model: "deepseek-chat".into(),
        timeout_seconds: 60,
    }).unwrap();
    
    let response = adapter.generate(LlmRequest {
        messages: vec![Message::user("Say hello in 5 words")],
        temperature: 0.1,
        max_tokens: Some(20),
        ..Default::default()
    }).await.unwrap();
    
    assert!(!response.content.is_empty());
    assert!(response.content.split_whitespace().count() <= 7);
}
```

---

**End of PRD**
