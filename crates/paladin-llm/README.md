# paladin-llm

LLM provider adapters for the Paladin framework.

## Purpose

`paladin-llm` provides configurable adapters for OpenAI, Anthropic, DeepSeek, and mock-backed testing providers.

## Key Modules

- `provider_factory`: Provider selection and construction.
- `config`: Provider configuration structures.
- `error`: Error types for provider operations.
- `llm_analysis_service`: Higher-level LLM orchestration helpers.
- `openai`, `anthropic`, `deepseek`, `mock`: Provider-specific adapters.

## Usage

```rust
use paladin_llm::provider_factory::LlmProviderFactory;

// Create providers by name at runtime.
let _factory = LlmProviderFactory::new();
```

## Feature Flags

- `default = ["openai", "mock"]`
- `openai`: Enable OpenAI provider adapter.
- `anthropic`: Enable Anthropic provider adapter.
- `deepseek`: Enable DeepSeek provider adapter.
- `mock`: Enable mock provider for tests.
- `vision`: Enable multimodal support on compatible providers.
- `openai-embeddings`: Enable OpenAI embedding utilities.
