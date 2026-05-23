//! Conversion bridges between [`LlmProviderConfig`]/[`VisionConfig`]
//! (application config types with serde) and the runtime adapter config types
//! within the same crate.
//!
//! Placing the impls here keeps all `paladin-llm` types together and avoids
//! the Rust orphan rule restriction (both source and target types are local to
//! this crate).

#[cfg(any(feature = "openai", feature = "anthropic", feature = "deepseek"))]
use crate::config::llm::LlmProviderConfig;

#[cfg(feature = "openai")]
use crate::openai::adapter::OpenAIConfig;

#[cfg(feature = "anthropic")]
use crate::anthropic::adapter::AnthropicConfig;

#[cfg(feature = "deepseek")]
use crate::deepseek::adapter::DeepSeekConfig;

#[cfg(feature = "vision")]
use crate::config::vision::VisionConfig as ConfigVisionConfig;
#[cfg(feature = "vision")]
use crate::openai::vision::{
    VisionConfig as AdapterVisionConfig, VisionProviderConfig as AdapterVisionProviderConfig,
    VisionRetryConfig as AdapterVisionRetryConfig,
};

// ── OpenAI ───────────────────────────────────────────────────────────────────

#[cfg(feature = "openai")]
impl From<&LlmProviderConfig> for OpenAIConfig {
    /// Convert an [`LlmProviderConfig`] into an [`OpenAIConfig`].
    ///
    /// `organization` is always `None`; set it via env var
    /// (`OPENAI_ORGANIZATION`) or construct `OpenAIConfig` directly.
    fn from(cfg: &LlmProviderConfig) -> Self {
        Self {
            api_key: cfg.api_key.clone(),
            base_url: cfg
                .base_url
                .clone()
                .unwrap_or_else(|| "https://api.openai.com/v1".to_string()),
            organization: None,
            timeout_seconds: cfg.timeout_seconds.unwrap_or(300),
            max_retries: cfg.max_retries.unwrap_or(3),
        }
    }
}

// ── Anthropic ────────────────────────────────────────────────────────────────

#[cfg(feature = "anthropic")]
impl From<&LlmProviderConfig> for AnthropicConfig {
    /// Convert an [`LlmProviderConfig`] into an [`AnthropicConfig`].
    fn from(cfg: &LlmProviderConfig) -> Self {
        Self {
            api_key: cfg.api_key.clone(),
            base_url: cfg
                .base_url
                .clone()
                .unwrap_or_else(|| "https://api.anthropic.com/v1".to_string()),
            model: cfg
                .default_model
                .clone()
                .unwrap_or_else(|| "claude-3-5-sonnet-20241022".to_string()),
            max_tokens: 4096,
            timeout_seconds: cfg.timeout_seconds.unwrap_or(300),
        }
    }
}

// ── DeepSeek ─────────────────────────────────────────────────────────────────

#[cfg(feature = "deepseek")]
impl From<&LlmProviderConfig> for DeepSeekConfig {
    /// Convert an [`LlmProviderConfig`] into a [`DeepSeekConfig`].
    fn from(cfg: &LlmProviderConfig) -> Self {
        Self {
            api_key: cfg.api_key.clone(),
            base_url: cfg
                .base_url
                .clone()
                .unwrap_or_else(|| "https://api.deepseek.com/v1".to_string()),
            model: cfg
                .default_model
                .clone()
                .unwrap_or_else(|| "deepseek-chat".to_string()),
            timeout_seconds: cfg.timeout_seconds.unwrap_or(60),
        }
    }
}

// ── Vision config ────────────────────────────────────────────────────────────

#[cfg(feature = "vision")]
impl From<&ConfigVisionConfig> for AdapterVisionConfig {
    /// Convert the application [`VisionConfig`](ConfigVisionConfig) (with serde)
    /// into the runtime [`VisionConfig`](AdapterVisionConfig) used by adapters.
    fn from(cfg: &ConfigVisionConfig) -> Self {
        Self {
            retry: AdapterVisionRetryConfig {
                max_retries: cfg.retry.max_retries,
                initial_backoff_ms: cfg.retry.initial_backoff_ms,
                backoff_multiplier: cfg.retry.backoff_multiplier,
            },
            openai: AdapterVisionProviderConfig {
                max_tokens: cfg.openai.max_tokens,
            },
            anthropic: AdapterVisionProviderConfig {
                max_tokens: cfg.anthropic.max_tokens,
            },
        }
    }
}
