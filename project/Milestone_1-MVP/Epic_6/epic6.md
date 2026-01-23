## Epic 6: Provider Expansion

### Overview

**Priority:** High  
**Effort:** 2-3 weeks  
**Dependencies:** Epic 1  
**Team:** 1-2 developers

**Objective:** Expand LLM provider support to include DeepSeek, Anthropic, and establish patterns for future providers.

### Technical Design

#### Infrastructure Layer

**adapters/llm/deepseek_adapter.rs**

```rust
pub struct DeepSeekAdapter {
    client: reqwest::Client,
    config: DeepSeekConfig,
}

#[derive(Debug, Clone)]
pub struct DeepSeekConfig {
    pub api_key: String,
    pub base_url: String,
    pub model: String,
    pub timeout_seconds: u64,
}

#[async_trait]
impl LlmPort for DeepSeekAdapter {
    async fn generate(&self, request: LlmRequest) -> Result<LlmResponse, LlmError>;
    async fn generate_stream(&self, request: LlmRequest) 
        -> Result<Box<dyn Stream<Item = Result<StreamingResponse, LlmError>> + Send>, LlmError>;
    async fn validate_model(&self, model: &str) -> Result<bool, LlmError>;
    async fn get_available_models(&self) -> Result<Vec<String>, LlmError>;
    fn get_provider_name(&self) -> &'static str { "deepseek" }
}
```

**adapters/llm/anthropic_adapter.rs**

```rust
pub struct AnthropicAdapter {
    client: reqwest::Client,
    config: AnthropicConfig,
}

#[derive(Debug, Clone)]
pub struct AnthropicConfig {
    pub api_key: String,
    pub base_url: String,
    pub model: String,
    pub max_tokens: u32,
}

#[async_trait]
impl LlmPort for AnthropicAdapter {
    // Implementation with Claude-specific message format
}
```

### Acceptance Criteria

- [ ] DeepSeek provider works with all Paladin features
- [ ] Anthropic provider works with all Paladin features
- [ ] Providers are interchangeable via configuration
- [ ] Streaming works correctly for both providers
- [ ] Tool use format supported for Anthropic

---
