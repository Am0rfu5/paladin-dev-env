//! Grok (xAI) LLM Adapter.
//!
//! A thin preset (D-01, D-05, D-12) sitting entirely on
//! [`crate::compat::CompatEngine`] — the shared OpenAI-compatible protocol
//! engine. This adapter supplies only Grok's `base_url`, credential env var,
//! default model, curated fallback model list and capabilities block; every
//! other behaviour (request shaping, retry, streaming, error mapping,
//! credential redaction, memoized model-list resolution) is inherited from
//! the engine unchanged.

use async_trait::async_trait;
use futures::Stream;
use std::env;

use paladin_ports::output::llm_port::{
    LlmError, LlmPort, LlmRequest, LlmResponse, ProviderCapabilities, StreamingResponse,
};

use crate::compat::{CompatCapabilities, CompatEngine, CompatEngineConfig};

/// Default Grok (xAI) API base URL.
///
/// `[CITED: docs.x.ai]` — not live-verified at plan-authoring time
/// (17-RESEARCH.md Assumptions Log A1 flags xAI as shipping new model
/// generations mid-2026); this execution environment has no network
/// egress, see the plan's SUMMARY.md for the confirmation status, per
/// D-00e.
pub const GROK_DEFAULT_BASE_URL: &str = "https://api.x.ai/v1";

/// Default Grok model requested when `XAI_MODEL` is unset.
pub const GROK_DEFAULT_MODEL: &str = "grok-4";

/// Curated fallback model list (D-13), returned when the live `/models`
/// endpoint fails, is unreachable, or returns an empty list. Never reported
/// as authoritative — see [`crate::compat::engine::CompatEngine::available_models`].
pub const GROK_FALLBACK_MODELS: &[&str] = &["grok-4", "grok-3"];

/// Configuration for the Grok (xAI) adapter.
#[derive(Debug, Clone)]
pub struct GrokConfig {
    /// API key for Grok (xAI) authentication.
    pub api_key: String,
    /// Base URL for the Grok API.
    pub base_url: String,
    /// Default model to use.
    pub model: String,
    /// Request timeout in seconds.
    pub timeout_seconds: u64,
}

impl GrokConfig {
    /// Load configuration from environment variables.
    ///
    /// # Environment Variables
    /// - `XAI_API_KEY` (required): Grok (xAI) API key.
    /// - `XAI_BASE_URL` (optional): API base URL, defaults to
    ///   [`GROK_DEFAULT_BASE_URL`].
    /// - `XAI_MODEL` (optional): Default model, defaults to
    ///   [`GROK_DEFAULT_MODEL`].
    /// - `XAI_TIMEOUT_SECONDS` (optional): Request timeout, defaults to
    ///   `60`.
    ///
    /// # Errors
    /// Returns an error if `XAI_API_KEY` is absent, or another variable's
    /// value fails to parse or validate.
    pub fn from_env() -> Result<Self, String> {
        Self::from_parts(
            env::var("XAI_API_KEY").ok(),
            env::var("XAI_BASE_URL").ok(),
            env::var("XAI_MODEL").ok(),
            env::var("XAI_TIMEOUT_SECONDS").ok(),
        )
    }

    /// The pure defaulting/validation logic behind [`Self::from_env`],
    /// separated out so it is testable without mutating process environment
    /// variables — `std::env::set_var` is `unsafe` under Rust 2024 and this
    /// crate denies `unsafe_code` (`#![deny(unsafe_code)]`).
    fn from_parts(
        api_key: Option<String>,
        base_url: Option<String>,
        model: Option<String>,
        timeout_seconds: Option<String>,
    ) -> Result<Self, String> {
        let api_key =
            api_key.ok_or_else(|| "XAI_API_KEY environment variable not set".to_string())?;
        let base_url = base_url.unwrap_or_else(|| GROK_DEFAULT_BASE_URL.to_string());
        let model = model.unwrap_or_else(|| GROK_DEFAULT_MODEL.to_string());
        let timeout_seconds = timeout_seconds
            .unwrap_or_else(|| "60".to_string())
            .parse()
            .map_err(|_| "Invalid XAI_TIMEOUT_SECONDS value".to_string())?;

        let config = Self {
            api_key,
            base_url,
            model,
            timeout_seconds,
        };

        config.validate()?;
        Ok(config)
    }

    /// Create configuration with custom values.
    pub fn new(api_key: String, base_url: String, model: String) -> Self {
        Self {
            api_key,
            base_url,
            model,
            timeout_seconds: 60,
        }
    }

    /// Validate the configuration.
    pub fn validate(&self) -> Result<(), String> {
        if self.api_key.is_empty() {
            return Err("API key cannot be empty".to_string());
        }
        if self.base_url.is_empty() {
            return Err("Base URL cannot be empty".to_string());
        }
        if !self.base_url.starts_with("http") {
            return Err("Base URL must start with http or https".to_string());
        }
        if self.model.is_empty() {
            return Err("Model name cannot be empty".to_string());
        }
        Ok(())
    }
}

/// Grok (xAI) LLM Adapter implementing [`LlmPort`].
///
/// Every method delegates to an owned [`CompatEngine`] (D-05) — this struct
/// carries no protocol logic of its own.
pub struct GrokAdapter {
    engine: CompatEngine,
}

impl GrokAdapter {
    /// Create a new Grok adapter.
    ///
    /// # Errors
    /// Returns an error if configuration is invalid or the underlying HTTP
    /// client cannot be created.
    pub fn new(config: GrokConfig) -> Result<Self, LlmError> {
        config.validate().map_err(|e| {
            LlmError::AuthenticationError(format!("Invalid Grok configuration: {}", e))
        })?;

        let engine_config = CompatEngineConfig {
            base_url: config.base_url,
            api_key: config.api_key,
            model: config.model,
            timeout_seconds: config.timeout_seconds,
            max_retries: 3,
            capabilities: CompatCapabilities {
                supports_streaming: true,
                supports_tool_calling: false,
                supports_function_calling: false,
                supports_vision: false,
                supports_embeddings: false,
                max_context_tokens: Some(131_072),
                supports_system_messages: true,
                temperature_range: Some((0.0, 2.0)),
            },
            fallback_models: GROK_FALLBACK_MODELS.iter().map(|s| s.to_string()).collect(),
            error_override: None,
        };

        Ok(Self {
            engine: CompatEngine::new(engine_config)?,
        })
    }
}

#[async_trait]
impl LlmPort for GrokAdapter {
    async fn generate(&self, request: LlmRequest) -> Result<LlmResponse, LlmError> {
        self.engine.generate(request).await
    }

    async fn generate_stream(
        &self,
        request: LlmRequest,
    ) -> Result<Box<dyn Stream<Item = Result<StreamingResponse, LlmError>> + Send>, LlmError> {
        self.engine.generate_stream(request).await
    }

    async fn validate_model(&self, model: &str) -> Result<bool, LlmError> {
        Ok(self.engine.validate_model(model).await)
    }

    async fn get_available_models(&self) -> Result<Vec<String>, LlmError> {
        Ok(self.engine.available_models().await)
    }

    fn get_provider_name(&self) -> &'static str {
        "grok"
    }

    fn get_capabilities(&self) -> ProviderCapabilities {
        self.engine.capabilities()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockito::Server;
    use paladin_core::platform::container::prompt::{PromptItem, PromptType, UserPrompt};
    use paladin_ports::output::llm_port::FinishReason;
    use serde_json::json;
    use std::collections::HashMap;
    use uuid::Uuid;

    fn build_request(model: &str) -> LlmRequest {
        LlmRequest {
            id: Uuid::new_v4(),
            model: model.to_string(),
            prompt: PromptItem::new(PromptType::User(UserPrompt {
                query: "Hello".to_string(),
                context: None,
            }))
            .unwrap(),
            attachments: vec![],
            stream: false,
            metadata: HashMap::new(),
        }
    }

    // ── GrokConfig::from_env() defaulting logic ──

    #[test]
    fn grok_config_from_env_errors_when_api_key_absent() {
        let result = GrokConfig::from_parts(None, None, None, None);
        assert!(result.is_err());
    }

    #[test]
    fn grok_config_defaults_base_url_and_model_when_only_key_is_set() {
        let config = GrokConfig::from_parts(Some("test-key".to_string()), None, None, None)
            .expect("key alone must be sufficient to build a valid config");
        assert_eq!(config.base_url, GROK_DEFAULT_BASE_URL);
        assert_eq!(config.model, GROK_DEFAULT_MODEL);
        assert_eq!(config.timeout_seconds, 60);
    }

    #[test]
    fn grok_config_honors_xai_base_url_override() {
        let config = GrokConfig::from_parts(
            Some("test-key".to_string()),
            Some("https://override.example/v1".to_string()),
            Some("grok-3".to_string()),
            Some("30".to_string()),
        )
        .unwrap();
        assert_eq!(config.base_url, "https://override.example/v1");
        assert_eq!(config.model, "grok-3");
        assert_eq!(config.timeout_seconds, 30);
    }

    // ── Request shaping / response parsing ──

    #[tokio::test]
    async fn generate_posts_to_chat_completions_with_auth_header_and_matching_body() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("POST", "/chat/completions")
            .match_header("authorization", "Bearer test-key")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(
                json!({
                    "id": "cmpl-1",
                    "model": "grok-4",
                    "choices": [{
                        "index": 0,
                        "message": {"role": "assistant", "content": "Hi there"},
                        "finish_reason": "stop"
                    }],
                    "usage": {"prompt_tokens": 5, "completion_tokens": 3, "total_tokens": 8}
                })
                .to_string(),
            )
            .create_async()
            .await;

        let config = GrokConfig::new("test-key".to_string(), server.url(), "grok-4".to_string());
        let adapter = GrokAdapter::new(config).unwrap();

        let response = adapter
            .generate(build_request("grok-4"))
            .await
            .expect("mock server returned a well-formed response");

        assert_eq!(response.content, "Hi there");
        assert!(matches!(response.finish_reason, FinishReason::Stop));

        mock.assert_async().await;
    }

    // ── Streaming ──

    #[tokio::test]
    async fn generate_stream_assembles_deltas_in_wire_order_with_terminal_stop() {
        use futures::StreamExt;

        let sse_body = concat!(
            "data: {\"id\":\"1\",\"choices\":[{\"delta\":{\"content\":\"Hel\"},\"finish_reason\":null}]}\n\n",
            "data: {\"id\":\"1\",\"choices\":[{\"delta\":{\"content\":\"lo \"},\"finish_reason\":null}]}\n\n",
            "data: {\"id\":\"1\",\"choices\":[{\"delta\":{\"content\":\"world\"},\"finish_reason\":\"stop\"}]}\n\n",
            "data: [DONE]\n\n",
        );

        let mut server = Server::new_async().await;
        server
            .mock("POST", "/chat/completions")
            .with_status(200)
            .with_header("content-type", "text/event-stream")
            .with_body(sse_body)
            .create_async()
            .await;

        let config = GrokConfig::new("test-key".to_string(), server.url(), "grok-4".to_string());
        let adapter = GrokAdapter::new(config).unwrap();

        let stream = adapter
            .generate_stream(build_request("grok-4"))
            .await
            .unwrap();
        let mut stream = Box::into_pin(stream);

        let mut assembled = String::new();
        let mut last_finish_reason = None;
        while let Some(item) = stream.next().await {
            let chunk = item.unwrap();
            assembled.push_str(&chunk.delta);
            if chunk.finish_reason.is_some() {
                last_finish_reason = chunk.finish_reason;
            }
        }

        assert_eq!(assembled, "Hello world");
        assert!(matches!(last_finish_reason, Some(FinishReason::Stop)));
    }

    // ── Error mapping ──

    #[tokio::test]
    async fn http_401_maps_to_authentication_error() {
        let mut server = Server::new_async().await;
        server
            .mock("POST", "/chat/completions")
            .with_status(401)
            .with_body(r#"{"error":"invalid key"}"#)
            .create_async()
            .await;

        let config = GrokConfig::new("test-key".to_string(), server.url(), "grok-4".to_string());
        let adapter = GrokAdapter::new(config).unwrap();

        let result = adapter.generate(build_request("grok-4")).await;
        assert!(matches!(result, Err(LlmError::AuthenticationError(_))));
    }

    #[tokio::test]
    async fn http_429_maps_to_rate_limit_exceeded() {
        let mut server = Server::new_async().await;
        server
            .mock("POST", "/chat/completions")
            .with_status(429)
            .with_body(r#"{"error":"slow down"}"#)
            .expect_at_least(1)
            .create_async()
            .await;

        let config = GrokConfig::new("test-key".to_string(), server.url(), "grok-4".to_string());
        let adapter = GrokAdapter::new(config).unwrap();

        let result = adapter.generate(build_request("grok-4")).await;
        assert!(matches!(result, Err(LlmError::RateLimitExceeded)));
    }

    #[tokio::test]
    async fn http_404_maps_to_model_not_available() {
        let mut server = Server::new_async().await;
        server
            .mock("POST", "/chat/completions")
            .with_status(404)
            .with_body(r#"{"error":"no such model"}"#)
            .create_async()
            .await;

        let config = GrokConfig::new("test-key".to_string(), server.url(), "grok-4".to_string());
        let adapter = GrokAdapter::new(config).unwrap();

        let result = adapter.generate(build_request("grok-4")).await;
        assert!(matches!(result, Err(LlmError::ModelNotAvailable(_))));
    }

    #[tokio::test]
    async fn http_400_maps_to_invalid_prompt() {
        let mut server = Server::new_async().await;
        server
            .mock("POST", "/chat/completions")
            .with_status(400)
            .with_body(r#"{"error":"bad request"}"#)
            .create_async()
            .await;

        let config = GrokConfig::new("test-key".to_string(), server.url(), "grok-4".to_string());
        let adapter = GrokAdapter::new(config).unwrap();

        let result = adapter.generate(build_request("grok-4")).await;
        assert!(matches!(result, Err(LlmError::InvalidPrompt(_))));
    }

    // ── Capabilities / identity ──

    #[test]
    fn get_provider_name_returns_grok() {
        let config = GrokConfig::new(
            "test-key".to_string(),
            GROK_DEFAULT_BASE_URL.to_string(),
            GROK_DEFAULT_MODEL.to_string(),
        );
        let adapter = GrokAdapter::new(config).unwrap();
        assert_eq!(adapter.get_provider_name(), "grok");
    }

    #[test]
    fn get_capabilities_reports_no_tool_or_function_calling() {
        let config = GrokConfig::new(
            "test-key".to_string(),
            GROK_DEFAULT_BASE_URL.to_string(),
            GROK_DEFAULT_MODEL.to_string(),
        );
        let adapter = GrokAdapter::new(config).unwrap();
        let caps = adapter.get_capabilities();

        assert!(!caps.supports_tool_calling);
        assert!(!caps.supports_function_calling);
        assert!(!caps.supports_vision);
        assert!(!caps.supports_embeddings);
        assert!(caps.supports_streaming);
        assert!(caps.supports_system_messages);
    }
}
