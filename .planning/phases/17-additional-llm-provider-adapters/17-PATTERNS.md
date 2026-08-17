# Phase 17: Additional LLM Provider Adapters - Pattern Map

**Mapped:** 2026-08-17
**Files analyzed:** 20 (new) + 6 (modified)
**Analogs found:** 20 / 20

## File Classification

| New/Modified File | Role | Data Flow | Closest Analog | Match Quality |
|---|---|---|---|---|
| `crates/paladin-llm/src/compat/mod.rs` | module root | — | `crates/paladin-llm/src/deepseek/mod.rs` (module re-export shape) | role-match |
| `crates/paladin-llm/src/compat/types.rs` | model (wire types) | request-response | `crates/paladin-llm/src/deepseek/adapter.rs` (private struct block, lines 104-187) | exact (generalization source) |
| `crates/paladin-llm/src/compat/engine.rs` | service (adapter core) | streaming + request-response | `crates/paladin-llm/src/deepseek/adapter.rs` (whole file, generalized) | exact (generalization source) |
| `crates/paladin-llm/src/kimi/mod.rs` | module root | — | `crates/paladin-llm/src/deepseek/mod.rs` | exact |
| `crates/paladin-llm/src/kimi/adapter.rs` | service (thin preset adapter) | request-response + streaming | `crates/paladin-llm/src/deepseek/adapter.rs` (Config + `LlmPort` impl shell) | exact |
| `crates/paladin-llm/src/qwen/mod.rs` | module root | — | `crates/paladin-llm/src/deepseek/mod.rs` | exact |
| `crates/paladin-llm/src/qwen/adapter.rs` | service (thin preset adapter) | request-response + streaming | `crates/paladin-llm/src/deepseek/adapter.rs` | exact |
| `crates/paladin-llm/src/grok/mod.rs` | module root | — | `crates/paladin-llm/src/deepseek/mod.rs` | exact |
| `crates/paladin-llm/src/grok/adapter.rs` | service (thin preset adapter) | request-response + streaming | `crates/paladin-llm/src/deepseek/adapter.rs` | exact |
| `crates/paladin-llm/src/ollama/mod.rs` | module root | — | `crates/paladin-llm/src/deepseek/mod.rs` | exact |
| `crates/paladin-llm/src/ollama/adapter.rs` | service (thin preset adapter, keyless) | request-response + streaming | `crates/paladin-llm/src/deepseek/adapter.rs` (config shape only — no key requirement) | role-match |
| `crates/paladin-llm/src/openai_compatible/mod.rs` | module root | — | `crates/paladin-llm/src/deepseek/mod.rs` | role-match |
| `crates/paladin-llm/src/openai_compatible/adapter.rs` | service (config-driven generic adapter) | request-response + streaming | `crates/paladin-llm/src/deepseek/adapter.rs` (engine usage) + `paladin_ports::llm_port::ProviderCapabilities::default()` (pessimistic-default precedent) | role-match |
| `crates/paladin-llm/src/gemini/mod.rs` | module root | — | `crates/paladin-llm/src/anthropic/mod.rs` | exact |
| `crates/paladin-llm/src/gemini/adapter.rs` | service (bespoke-protocol adapter) | request-response + streaming | `crates/paladin-llm/src/anthropic/adapter.rs` (whole file, structural template) | exact |
| `crates/paladin-llm/src/provider_factory.rs` (MODIFIED) | service (registry/factory) | request-response | itself (current hardcoded match, `:44-150`) | exact (refactor target) |
| `crates/paladin-llm/src/config/llm.rs` (MODIFIED) | config model | CRUD (config load/validate) | itself (current 3-field struct + `#[cfg(test)]` module) | exact (extend target) |
| `crates/paladin-llm/src/config/bridge.rs` (MODIFIED) | config adapter/mapper | transform | itself (existing `From<&LlmProviderConfig> for *Config` impls) | exact |
| `crates/paladin-llm/src/lib.rs` (MODIFIED) | crate root / capability test | — | itself (`capability_invariants` module, `:85-`) | exact |
| `crates/paladin-llm/Cargo.toml` (MODIFIED) | config (feature flags) | — | itself | exact |
| `Cargo.toml` (root, MODIFIED) | config (facade feature flags) | — | itself (`:55`, `:264-270`) | exact |
| `tests/unit/llm/provider_factory_test.rs` (MODIFIED) | test | request-response | itself (282 lines, existing cases) | exact |
| `tests/integration/ollama_docker_test.rs` (NEW) | test (Docker-gated Tier 2) | streaming/request-response | `tests/integration/provider_switching_test.rs` (mockito pattern) + `docker/docker-compose.test.yml` service pattern | role-match |
| `docker/docker-compose.test.yml` (MODIFIED) | config (Docker service) | — | itself (`redis-test`/`minio-test` service blocks) | exact |
| per-adapter `#[cfg(test)]` mock-transport tests (in each `*/adapter.rs`) | test | request-response + streaming | `tests/integration/provider_switching_test.rs` (mockito setup, `:20-60`) — see Open Question 1 in RESEARCH.md on placement | role-match |

## Pattern Assignments

### `crates/paladin-llm/src/compat/engine.rs` (service, streaming + request-response)

**Analog:** `crates/paladin-llm/src/deepseek/adapter.rs` (1,368 lines — this is the extraction source, not a sibling to imitate)

**Imports pattern** (`deepseek/adapter.rs:6-23`):
```rust
use async_trait::async_trait;
use chrono::Utc;
use futures::{Stream, StreamExt};
use reqwest::{
    Client,
    header::{AUTHORIZATION, CONTENT_TYPE, HeaderMap, HeaderValue},
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;
use std::time::Duration;
use uuid::Uuid;

use paladin_core::platform::container::prompt::{PromptItem, PromptType};
use paladin_ports::output::llm_port::{
    FinishReason, LlmError, LlmPort, LlmRequest, LlmResponse, ProviderCapabilities,
    StreamingResponse, TokenUsage,
};
```

**Client construction / auth header pattern** (`deepseek/adapter.rs:370-393`):
```rust
pub fn new(config: DeepSeekConfig) -> Result<Self, LlmError> {
    config.validate().map_err(|e| {
        LlmError::AuthenticationError(format!("Invalid DeepSeek configuration: {}", e))
    })?;
    let timeout = Duration::from_secs(config.timeout_seconds);
    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    headers.insert(
        AUTHORIZATION,
        HeaderValue::from_str(&format!("Bearer {}", config.api_key)).map_err(|e| {
            LlmError::AuthenticationError(format!("Invalid API key format: {}", e))
        })?,
    );
    let client = Client::builder()
        .timeout(timeout)
        .default_headers(headers)
        .build()
        .map_err(|e| LlmError::NetworkError(format!("Failed to create HTTP client: {}", e)))?;
    Ok(Self { client, config })
}
```
**Ollama note:** the engine must accept a `None`/placeholder API key path — Ollama's docs say auth is "required but ignored." Recommend the engine send a fixed placeholder (`"ollama"`) rather than making the `Authorization` header itself `Option`, to keep the header-construction code path identical across all five presets (see RESEARCH.md Wire-Level Facts § Ollama).

**Request struct** (`deepseek/adapter.rs:104-119`, generalize as `CompatRequest`):
```rust
#[derive(Debug, Serialize)]
struct DeepSeekRequest {
    model: String,
    messages: Vec<DeepSeekMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_tokens: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    top_p: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    frequency_penalty: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    presence_penalty: Option<f32>,
    stream: bool,
}
```

**Null-tolerant message content** (`deepseek/adapter.rs:121-138`, plus `deserialize_null_as_empty_string` near `:260`) — keep this in the shared core; a reasoning-model preset among the four new providers could hit the same null-on-truncation shape:
```rust
#[derive(Debug, Serialize, Deserialize)]
struct DeepSeekMessage {
    role: String,
    #[serde(default, deserialize_with = "deserialize_null_as_empty_string")]
    content: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    reasoning_content: Option<String>,
}
```

**Prompt-to-messages conversion** (`deepseek/adapter.rs:396-467`) — copy the `match &prompt.node.node.prompt_type { PromptType::System(..) | User(..) | Text(..) | Assistant(..) | Function(..) }` arm structure verbatim; it is provider-agnostic and belongs in the shared engine, not per-preset.

**Finish-reason mapping** (`deepseek/adapter.rs:474-483`):
```rust
fn map_finish_reason(reason: Option<String>) -> FinishReason {
    match reason.as_deref() {
        Some("stop") => FinishReason::Stop,
        Some("length") => FinishReason::Length,
        Some("content_filter") => FinishReason::ContentFilter,
        Some("function_call") => FinishReason::FunctionCall,
        Some(other) => FinishReason::Error(format!("Unknown finish reason: {}", other)),
        None => FinishReason::Stop,
    }
}
```

**Credential redaction / diagnostic excerpt** (`deepseek/adapter.rs:487-497`, plus `redact_credentials`/`bounded_excerpt` near `:260-356`) — copy verbatim into the shared engine; every preset needs this identically:
```rust
fn diagnostic_excerpt(&self, body: &str) -> String {
    let redacted = redact_credentials(body, &self.config.api_key);
    bounded_excerpt(&redacted, RESPONSE_EXCERPT_CHAR_BUDGET)
}
```

**Error mapping (`map_error`)** (`deepseek/adapter.rs:499-517`) — generalize the status-code switch (401→AuthenticationError, 429→RateLimitExceeded, 404→ModelNotAvailable, 400→InvalidPrompt, default→ProcessingError); leave the 402/`UsageLimitExceeded` arm as a per-preset override point since it is DeepSeek-specific (no other of the four new providers is documented to use 402 the same way):
```rust
fn map_error(&self, status: u16, message: &str) -> LlmError {
    match status {
        401 => LlmError::AuthenticationError(format!("Invalid API key ... {}", message)),
        429 => LlmError::RateLimitExceeded,
        404 => LlmError::ModelNotAvailable(message.to_string()),
        400 => LlmError::InvalidPrompt(message.to_string()),
        _ => LlmError::ProcessingError(format!("API error ({}): {}", status, message)),
    }
}
```

**Retry-with-backoff** (`deepseek/adapter.rs:546-599`) — copy the whole `call_api_with_retry` verbatim into the engine, including its non-retryable set (`AuthenticationError | InvalidPrompt | EmptyCompletion | UsageLimitExceeded`) and its doc comment explaining the deliberate non-goal of NOT unifying attempt-count with `anthropic/adapter.rs`'s loop:
```rust
async fn call_api_with_retry<F, Fut, T>(&self, operation: F, max_retries: u32) -> Result<T, LlmError>
where F: Fn() -> Fut, Fut: std::future::Future<Output = Result<T, LlmError>> {
    for attempt in 0..=max_retries {
        match operation().await {
            Ok(result) => return Ok(result),
            Err(e) => {
                if matches!(e, LlmError::AuthenticationError(_) | LlmError::InvalidPrompt(_)
                    | LlmError::EmptyCompletion(_) | LlmError::UsageLimitExceeded { .. }) {
                    return Err(e);
                }
                if attempt >= max_retries { return Err(e); }
                let backoff = Duration::from_millis(100 * 2_u64.pow(attempt));
                let jitter = Duration::from_millis(rand::random::<u64>() % 100);
                tokio::time::sleep(backoff + jitter).await;
            }
        }
    }
    unreachable!()
}
```

**SSE streaming assembly** (documented at `deepseek/adapter.rs:736-785`, cited verbatim in RESEARCH.md `## Code Examples`) — split on lines, strip `"data: "` prefix, terminal sentinel is the literal string `"[DONE]"` (not a JSON object). Copy this loop shape into the engine's `generate_stream()`.

**Body-read-then-parse pattern (Pitfall 3 — MUST replicate, not `Response::json()`)** — read body via `.text()` first, then `serde_json::from_str` separately (`deepseek/adapter.rs:632-666` per RESEARCH.md); this is what distinguishes a network/timeout failure from a schema-drift failure in the error message. Do not use `reqwest::Response::json()` anywhere in the new engine.

---

### `crates/paladin-llm/src/kimi/adapter.rs`, `qwen/adapter.rs`, `grok/adapter.rs`, `ollama/adapter.rs` (service, thin presets)

**Analog:** `crates/paladin-llm/src/deepseek/adapter.rs` `*Config` struct + `from_env()`/`new()`/`validate()` shape (`:25-100`)

**Config `from_env()` pattern to copy per-preset, substituting D-12's env-var names** (`deepseek/adapter.rs:38-72`):
```rust
pub fn from_env() -> Result<Self, String> {
    let api_key = env::var("MOONSHOT_API_KEY")            // D-12 per-vendor env var
        .map_err(|_| "MOONSHOT_API_KEY environment variable not set")?;
    let base_url = env::var("MOONSHOT_BASE_URL")
        .unwrap_or_else(|_| "https://api.moonshot.ai/v1".to_string());   // RESEARCH Wire-Level Facts
    let model = env::var("MOONSHOT_MODEL").unwrap_or_else(|_| "<vendor default>".to_string());
    let timeout_seconds = env::var("MOONSHOT_TIMEOUT_SECONDS")
        .unwrap_or_else(|_| "60".to_string())
        .parse()
        .map_err(|_| "Invalid MOONSHOT_TIMEOUT_SECONDS value")?;
    let config = Self { api_key, base_url, model, timeout_seconds };
    config.validate()?;
    Ok(config)
}
```
For `ollama/adapter.rs`: no required env var for the key (D-12) — send the fixed placeholder string documented in RESEARCH.md instead of erroring when unset; `base_url` default `http://localhost:11434/v1` via `OLLAMA_BASE_URL` override.

**Preset supplies to the engine** (per RESEARCH.md Pattern 1): `base_url` default + override env var, api-key env var name, default model string, `get_available_models()` curated fallback list (D-13), `get_capabilities()` block, `get_provider_name()` literal.

---

### `crates/paladin-llm/src/openai_compatible/adapter.rs` (service, config-driven generic adapter)

**Analog:** same engine usage as the presets above, but every engine parameter is config-driven (D-03/D-04/D-09) instead of a vendor default.

**Pessimistic-default capabilities config** (RESEARCH.md Pattern 2, illustrative but directly usable):
```rust
#[derive(Debug, Clone, Deserialize)]
pub struct OpenAiCompatibleCapabilitiesConfig {
    #[serde(default = "default_true")]
    pub supports_streaming: bool,       // D-04: true by default (in-spec)
    #[serde(default)]
    pub supports_tool_calling: bool,    // D-04: false by default
    #[serde(default)]
    pub supports_function_calling: bool,
    #[serde(default)]
    pub supports_vision: bool,
    #[serde(default)]
    pub supports_embeddings: bool,
    pub max_context_tokens: Option<u32>,
    pub temperature_range: Option<(f32, f32)>,
}
fn default_true() -> bool { true }
```
Write the Red-step test FIRST per RESEARCH.md Pitfall 5: construct with every field omitted, assert every non-streaming flag is `false` and both `Option` fields are `None`.

`get_provider_name()` returns the fixed literal `"openai-compatible"` (D-07/D-09) — do not leak the configured `base_url` or any operator-supplied name into this method.

---

### `crates/paladin-llm/src/gemini/adapter.rs` (service, bespoke protocol)

**Analog:** `crates/paladin-llm/src/anthropic/adapter.rs` (1,180 lines) — structural template only; do NOT reuse `compat::engine` (D-08).

**Header comment / module doc pattern** (`anthropic/adapter.rs:1-6`):
```rust
//! Anthropic Claude LLM Adapter
//!
//! Provides integration with Anthropic's Claude API.
//! Supports standard completions, streaming, and all core LlmPort functionality.
//! Claude has unique requirements: system messages separate from messages array,
//! max_tokens required, and different message structure.
```
Gemini's own doc comment should name its own divergence: `systemInstruction` is a top-level sibling field (not in `contents[]`), `x-goog-api-key` header auth, `:generateContent`/`:streamGenerateContent?alt=sse` URL suffixes (see RESEARCH.md Wire-Level Facts § Gemini for the exact request/response shape and `GeminiRequest`/`GeminiResponse` struct sketch — copy those struct definitions directly).

**Vendor-specific error-signature constant pattern** (`anthropic/adapter.rs:26-33`) — Gemini's `map_error` should follow this same "named, documented, narrowly-matched signature constant" pattern for its `RESOURCE_EXHAUSTED` vs. rate-limit ambiguity (RESEARCH.md Open Question 2 — map conservatively to `RateLimitExceeded` and document the assumption in a doc comment, mirroring this precedent):
```rust
/// The exact phrase observed VERBATIM in Anthropic's HTTP 400
/// `invalid_request_error` body when an account has reached its configured
/// API usage limit (live run `4a3b749d`). Matched narrowly and deliberately...
const ANTHROPIC_USAGE_CAP_SIGNATURE: &str = "You have reached your specified API usage limits";
```

**No tool-calling surface:** omit `tools`/`toolConfig` from `GeminiRequest` entirely (RESEARCH.md Anti-Patterns); `get_capabilities().supports_tool_calling` and `.supports_vision` must both be `false`.

---

### `crates/paladin-llm/src/provider_factory.rs` (MODIFIED — service, registry/factory)

**Analog:** itself — the file being refactored (D-10)

**Current hardcoded match to replace** (`provider_factory.rs:62-117`):
```rust
pub fn create(&self, provider_name: &str) -> Result<Arc<dyn LlmPort>, ProviderFactoryError> {
    match provider_name.to_lowercase().as_str() {
        #[cfg(feature = "openai")]
        "openai" => { /* OpenAIConfig::from_env() + OpenAIAdapter::new(..) */ }
        #[cfg(feature = "deepseek")]
        "deepseek" => { /* ... */ }
        #[cfg(feature = "anthropic")]
        "anthropic" => { /* ... */ }
        other => Err(ProviderFactoryError::UnknownProvider(other.to_string())),
    }
}
```

**Current un-gated defect to fix structurally** (`provider_factory.rs:119-149` — this is the exact live bug D-10 removes):
```rust
pub fn get_default_provider() -> Option<String> {
    if std::env::var("OPENAI_API_KEY").is_ok() { return Some("openai".to_string()); }
    if std::env::var("DEEPSEEK_API_KEY").is_ok() { return Some("deepseek".to_string()); }
    if std::env::var("ANTHROPIC_API_KEY").is_ok() { return Some("anthropic".to_string()); }
    None
}
// list_available_providers() at :137 has the identical un-cfg-gated shape
```
Replace both with iteration over the same `cfg`-gated registry table `create()` derives from — no direct `std::env::var` check outside the table (RESEARCH.md Pitfall 1 warning sign).

**`UnknownProvider` error text to keep in sync** (`provider_factory.rs:16`):
```rust
#[error("Unknown provider: {0}. Supported providers: openai, deepseek, anthropic")]
UnknownProvider(String),
```
Must be generated/derived from the same table (nine names once all providers ship), not hand-edited.

**Existing test module shape to extend** (`provider_factory.rs:158-185`):
```rust
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_factory_creation() { ... }
    #[test]
    fn test_unknown_provider_returns_error() { ... }
    #[test]
    fn test_list_available_providers_returns_vec() { ... }
}
```

---

### `crates/paladin-llm/src/config/llm.rs` (MODIFIED — config model)

**Analog:** itself — current 3-field struct (`config/llm.rs:22-116`)

**Current shape being extended:**
```rust
pub struct LlmConfig {
    pub default_provider: Option<String>,
    pub openai: Option<LlmProviderConfig>,
    pub deepseek: Option<LlmProviderConfig>,
    pub anthropic: Option<LlmProviderConfig>,
}
```
Per RESEARCH.md "Claude's Discretion — Config surface shape," either add 6 more named fields (kimi/qwen/grok/ollama/gemini/openai_compatible — matches existing convention) or restructure to a map/list. **Whichever shape is chosen, existing YAML with only `openai`/`deepseek`/`anthropic` keys must keep deserializing unchanged** — the existing `#[cfg(test)]` module's five tests (`config/llm.rs:118-282`, e.g. `test_llm_config_validate_success`, `test_llm_config_get_provider_config_case_insensitive`) are the regression bar; write equivalent new-provider cases alongside them, do not replace them.

`validate()`'s per-provider match arms (`config/llm.rs:52-78`) and `get_provider_config()`'s match (`config/llm.rs:104-109`) both need one arm/entry per new provider name, generated the same hand-maintained way as today (or, if the struct is restructured to a map, replaced by a single lookup) — this is the config-surface half of what D-10's registry pattern models for the factory.

---

### `Cargo.toml` (root, MODIFIED) and `crates/paladin-llm/Cargo.toml` (MODIFIED) (config, feature flags)

**Analog:** itself — current feature graph

**Root `Cargo.toml` current defect (D-11 target)** — unconditional feature list at `Cargo.toml:55` (verified):
```toml
paladin-llm = { version = "0.8.0", path = "crates/paladin-llm", default-features = false, features = ["openai", "anthropic", "deepseek", "mock", "vision"] }
```
and the empty stub flags:
```toml
[features]
default = ["llm-openai"]
llm-openai = []
llm-anthropic = []
llm-deepseek = []
llm-all = ["llm-openai", "llm-anthropic", "llm-deepseek"]
```
D-11 fix pattern: forward each flag into the dependency instead of listing providers unconditionally —
```toml
llm-openai = ["paladin-llm/openai"]
llm-anthropic = ["paladin-llm/anthropic"]
llm-deepseek = ["paladin-llm/deepseek"]
llm-kimi = ["paladin-llm/kimi"]
llm-qwen = ["paladin-llm/qwen"]
llm-grok = ["paladin-llm/grok"]
llm-ollama = ["paladin-llm/ollama"]
llm-gemini = ["paladin-llm/gemini"]
llm-openai-compatible = ["paladin-llm/openai-compatible"]
llm-all = ["llm-openai", "llm-anthropic", "llm-deepseek", "llm-kimi", "llm-qwen", "llm-grok", "llm-ollama", "llm-gemini", "llm-openai-compatible"]
```
and the dependency line becomes feature-free (or driven purely by facade flags) rather than hardcoding `features = [...]`. **This is a CHANGELOG `BREAKING` entry** (D-11).

**`crates/paladin-llm/Cargo.toml` current feature block to extend** (verified, full file read):
```toml
[features]
default = ["openai", "mock"]
openai = ["dep:reqwest", "dep:rand"]
anthropic = ["dep:reqwest", "dep:rand"]
deepseek = ["dep:reqwest", "dep:rand"]
mock = []
vision = ["openai", "dep:base64"]
openai-embeddings = ["openai"]
```
Add one flag per new provider following the identical `["dep:reqwest", "dep:rand"]` shape (all five new surfaces are plain REST — RESEARCH.md Package Legitimacy Audit confirms no new dependency needed):
```toml
kimi = ["dep:reqwest", "dep:rand"]
qwen = ["dep:reqwest", "dep:rand"]
grok = ["dep:reqwest", "dep:rand"]
ollama = ["dep:reqwest", "dep:rand"]
gemini = ["dep:reqwest", "dep:rand"]
openai-compatible = ["dep:reqwest", "dep:rand"]
```
Also update `description` and `keywords` (PROV-04 requirement, RESEARCH.md canonical refs) — current values:
```toml
description = "LLM provider adapters for the Paladin framework — OpenAI, Anthropic, DeepSeek, and mock"
keywords = ["ai", "llm", "openai", "anthropic", "deepseek"]
```

---

### `crates/paladin-llm/src/lib.rs` (MODIFIED — crate root / capability test)

**Analog:** itself — `capability_invariants` test module (`lib.rs:85-`)

**Current module doc table to extend** (`lib.rs:9-16`):
```rust
//! | Feature flag | Provider | Types |
//! |---|---|---|
//! | `openai` (default) | OpenAI | [`openai::OpenAIAdapter`], [`openai::OpenAIConfig`] |
//! | `anthropic` | Anthropic | [`anthropic::AnthropicAdapter`], [`anthropic::AnthropicConfig`] |
//! | `deepseek` | DeepSeek | [`deepseek::DeepSeekAdapter`], [`deepseek::DeepSeekConfig`] |
//! | `mock` (default) | Testing | [`mock::MockLlmAdapter`], [`mock::MultiStepMockLlmPort`] |
```
Add rows for `kimi`/`qwen`/`grok`/`ollama`/`gemini`/`openai-compatible`. `#![deny(unsafe_code)]` and `#![warn(missing_docs)]` (crate-level lints, `lib.rs:41`+) apply unchanged to all new modules.

**`capability_invariants` test pattern to extend** (`lib.rs:85-116`, RESEARCH.md Open Question 3 recommends a sibling module):
```rust
mod capability_invariants {
    use crate::anthropic::{AnthropicAdapter, AnthropicConfig};
    use crate::deepseek::{DeepSeekAdapter, DeepSeekConfig};
    use crate::openai::{OpenAIAdapter, OpenAIConfig};
    use paladin_ports::output::llm_port::LlmPort;

    #[test]
    fn test_capabilities_tool_calling_matches_request_surface() {
        const REQUEST_SURFACE_SUPPORTS_TOOL_CALLING: bool = false;
        const RESPONSE_SURFACE_SUPPORTS_FUNCTION_CALLING: bool = false;
        // ... assert every adapter's get_capabilities() matches these constants
    }
}
```
Add a `#[cfg(all(test, feature = "kimi", feature = "qwen", feature = "grok", feature = "ollama", feature = "gemini", feature = "openai-compatible"))]` sibling (or widen the existing gate) asserting the same `false`/`false` correspondence for all six new adapters — this directly enforces RESEARCH.md Pitfall 4.

---

### `tests/unit/llm/provider_factory_test.rs` (MODIFIED — test)

**Analog:** itself (282 lines, existing) — add cases proving: (1) each new provider name resolves via `create()` when its feature is enabled, (2) a provider whose feature is compiled out is absent from `list_available_providers()` even when its env var is set (the D-10 regression test for Pitfall 1), (3) `get_default_provider()` priority ordering includes the new providers reasonably.

---

### `docker/docker-compose.test.yml` (MODIFIED — Docker service) / `tests/integration/ollama_docker_test.rs` (NEW)

**Analog:** `redis-test` service block (`docker-compose.test.yml:2-15`) — the smallest existing Tier-2 service pattern:
```yaml
redis-test:
  image: redis:7-alpine
  container_name: paladin-redis-test
  ports:
    - "6380:6379"
  command: redis-server --appendonly no --save "" --protected-mode no
  tmpfs:
    - /data
  networks:
    - paladin-test-network
  healthcheck:
    test: [ "CMD", "redis-cli", "ping" ]
    interval: 5s
    timeout: 3s
    retries: 5
```
`ollama-test` should follow this shape: `image: ollama/ollama:latest`, a mapped port (e.g. `11434`), a healthcheck hitting `/api/tags` or `/v1/models`, `tmpfs`/volume for model storage, and — since Ollama needs a model pulled before tests run — either an init container (mirroring `minio-test-init`'s `depends_on: condition: service_healthy` + entrypoint script pattern, `docker-compose.test.yml:33-56`) that runs `ollama pull <small-model>`, or a `command:` override on the main service.

**Test-file analog:** `tests/integration/provider_switching_test.rs:1-60` — module doc explaining the offline/keyless posture, `mockito::Server::new_async()` setup, `#[tokio::test]` async test fn. For the Docker-gated Ollama suite, swap the `mockito` server for the real `ollama-test` service URL (from an env var such as `OLLAMA_TEST_URL`), and gate the test file itself the way the existing `integration-tests` feature and `make test-integration-docker` target already gate Redis/MinIO tests.

---

## Shared Patterns

### Credential redaction (security-critical, apply to ALL new adapters)
**Source:** `crates/paladin-llm/src/deepseek/adapter.rs:250-356` (`redact_credentials`, `bounded_excerpt`, `diagnostic_excerpt`)
**Apply to:** `compat/engine.rs` (once, shared by all 4 presets + generic provider) AND `gemini/adapter.rs` (separately, since Gemini doesn't use the shared engine)
```rust
fn diagnostic_excerpt(&self, body: &str) -> String {
    let redacted = redact_credentials(body, &self.config.api_key);
    bounded_excerpt(&redacted, RESPONSE_EXCERPT_CHAR_BUDGET)
}
```
Ordering is load-bearing: redact BEFORE truncating, never the reverse (a truncation-first order can slice a secret in half and leak the surviving prefix).

### Retry / non-retryable error set (apply to compat::engine only; Gemini has its own `execute_with_retry` modeled on `anthropic/adapter.rs`)
**Source:** `crates/paladin-llm/src/deepseek/adapter.rs:546-599`
**Apply to:** `compat/engine.rs`
Non-retryable: `AuthenticationError | InvalidPrompt | EmptyCompletion | UsageLimitExceeded`. Retryable (implicitly, everything else): `NetworkError | Timeout | ProcessingError | RateLimitExceeded | ModelNotAvailable | TokenLimitExceeded`.

### Body-read-then-parse (never `Response::json()`)
**Source:** `crates/paladin-llm/src/deepseek/adapter.rs:632-666` (RESEARCH.md Pitfall 3)
**Apply to:** every new adapter's HTTP call sites, without exception — `.text()` first, then `serde_json::from_str` separately.

### Pessimistic capability defaults (D-04)
**Source:** `paladin_ports::output::llm_port::ProviderCapabilities::default()` (framework-wide precedent, `crates/paladin-ports/src/output/llm_port.rs:869-882`)
**Apply to:** `openai_compatible/adapter.rs`'s config-driven capabilities specifically; all five named presets' `get_capabilities()` must also report `supports_tool_calling: false` / `supports_function_calling: false` per the `capability_invariants` correspondence.

### `#![warn(missing_docs)]` / rustdoc on all public items
**Source:** `crates/paladin-llm/src/lib.rs:41`+ (crate-level lint)
**Apply to:** every new public struct/fn/module — enforced automatically by `cargo doc`/CI, no opt-out.

### Env-var-only credentials, never in config files or logs
**Source:** PROJECT.md constraint, precedent in every existing `*Config::from_env()`
**Apply to:** all six new `Config::from_env()` implementations — D-12's vendor-native env var names (`MOONSHOT_API_KEY`, `DASHSCOPE_API_KEY`, `XAI_API_KEY`, `GEMINI_API_KEY`, none for Ollama).

## No Analog Found

None — every file in the phase's scope has a strong (exact or role-match) analog already in the shipped tree. The generic `openai_compatible/adapter.rs`'s capability-defaulting behavior is the only piece without a direct 1:1 file precedent; it is modeled on `ProviderCapabilities::default()`'s existing pessimistic posture at the port layer (see Shared Patterns above) rather than on another adapter file.

## Metadata

**Analog search scope:** `crates/paladin-llm/src/` (all provider directories, `provider_factory.rs`, `config/`, `lib.rs`, `Cargo.toml`), root `Cargo.toml` feature section, `tests/unit/llm/`, `tests/integration/`, `docker/docker-compose.test.yml`
**Files scanned:** `deepseek/adapter.rs` (1,368 lines, primary template), `anthropic/adapter.rs` (bespoke-protocol template, header + error-signature pattern read), `provider_factory.rs` (185 lines, full), `config/llm.rs` (282 lines, full), `lib.rs` (194 lines, header + capability_invariants read), `paladin-llm/Cargo.toml` (48 lines, full), root `Cargo.toml` (feature section), `tests/integration/provider_switching_test.rs` (mockito pattern), `docker/docker-compose.test.yml` (full)
**Pattern extraction date:** 2026-08-17
</content>
