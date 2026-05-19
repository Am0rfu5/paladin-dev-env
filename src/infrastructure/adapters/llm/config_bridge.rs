// src/infrastructure/adapters/llm/config_bridge.rs
//
// Converts root-crate `ApplicationSettings` config types into the
// provider-specific config structs defined in `paladin-llm`.
//
// Dependency direction: root crate → paladin-llm (never the other way).

#[cfg(feature = "llm-openai")]
use paladin_llm::openai::adapter::OpenAIConfig;

#[cfg(feature = "llm-anthropic")]
use paladin_llm::anthropic::adapter::AnthropicConfig;

#[cfg(feature = "llm-deepseek")]
use paladin_llm::deepseek::adapter::DeepSeekConfig;

#[cfg(feature = "vision")]
use paladin_llm::openai::vision::{VisionConfig, VisionProviderConfig, VisionRetryConfig};

use crate::config::application_settings::LlmProviderConfig;

#[cfg(feature = "vision")]
use crate::config::application_settings::VisionConfig as AppVisionConfig;

// ── OpenAI ───────────────────────────────────────────────────────────────────

#[cfg(feature = "llm-openai")]
impl From<&LlmProviderConfig> for OpenAIConfig {
    /// Convert an [`LlmProviderConfig`] from `ApplicationSettings` into an
    /// [`OpenAIConfig`] for the `paladin-llm` OpenAI adapter.
    ///
    /// `organization` is always set to `None`; set it via env var
    /// (`OPENAI_ORGANIZATION`) or construct `OpenAIConfig` directly if needed.
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

#[cfg(feature = "llm-anthropic")]
impl From<&LlmProviderConfig> for AnthropicConfig {
    /// Convert an [`LlmProviderConfig`] from `ApplicationSettings` into an
    /// [`AnthropicConfig`] for the `paladin-llm` Anthropic adapter.
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

#[cfg(feature = "llm-deepseek")]
impl From<&LlmProviderConfig> for DeepSeekConfig {
    /// Convert an [`LlmProviderConfig`] from `ApplicationSettings` into a
    /// [`DeepSeekConfig`] for the `paladin-llm` DeepSeek adapter.
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
impl From<&AppVisionConfig> for VisionConfig {
    /// Convert the root crate's [`VisionConfig`] into the `paladin-llm`
    /// [`VisionConfig`] used by the OpenAI vision adapter.
    fn from(cfg: &AppVisionConfig) -> Self {
        Self {
            retry: VisionRetryConfig {
                max_retries: cfg.retry.max_retries,
                initial_backoff_ms: cfg.retry.initial_backoff_ms,
                backoff_multiplier: cfg.retry.backoff_multiplier,
            },
            openai: VisionProviderConfig {
                max_tokens: cfg.openai.max_tokens,
            },
            anthropic: VisionProviderConfig {
                max_tokens: cfg.anthropic.max_tokens,
            },
        }
    }
}

// ── Tests ────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::application_settings::LlmProviderConfig;

    fn typical_provider_config() -> LlmProviderConfig {
        LlmProviderConfig {
            api_key: "sk-test-key".to_string(),
            base_url: Some("https://custom.example.com/v1".to_string()),
            default_model: Some("gpt-4o".to_string()),
            default_temperature: Some(0.7),
            timeout_seconds: Some(120),
            max_retries: Some(5),
        }
    }

    fn minimal_provider_config() -> LlmProviderConfig {
        LlmProviderConfig {
            api_key: "sk-minimal".to_string(),
            base_url: None,
            default_model: None,
            default_temperature: None,
            timeout_seconds: None,
            max_retries: None,
        }
    }

    // ── OpenAI ──────────────────────────────────────────────────────────────

    #[cfg(feature = "llm-openai")]
    #[test]
    fn test_openai_config_from_provider_config_typical() {
        let provider = typical_provider_config();
        let cfg = OpenAIConfig::from(&provider);
        assert_eq!(cfg.api_key, "sk-test-key");
        assert_eq!(cfg.base_url, "https://custom.example.com/v1");
        assert_eq!(cfg.timeout_seconds, 120);
        assert_eq!(cfg.max_retries, 5);
        assert!(cfg.organization.is_none());
    }

    #[cfg(feature = "llm-openai")]
    #[test]
    fn test_openai_config_from_provider_config_defaults() {
        let provider = minimal_provider_config();
        let cfg = OpenAIConfig::from(&provider);
        assert_eq!(cfg.base_url, "https://api.openai.com/v1");
        assert_eq!(cfg.timeout_seconds, 300);
        assert_eq!(cfg.max_retries, 3);
    }

    // ── Anthropic ───────────────────────────────────────────────────────────

    #[cfg(feature = "llm-anthropic")]
    #[test]
    fn test_anthropic_config_from_provider_config_typical() {
        let provider = typical_provider_config();
        let cfg = AnthropicConfig::from(&provider);
        assert_eq!(cfg.api_key, "sk-test-key");
        assert_eq!(cfg.base_url, "https://custom.example.com/v1");
        assert_eq!(cfg.model, "gpt-4o");
        assert_eq!(cfg.timeout_seconds, 120);
    }

    #[cfg(feature = "llm-anthropic")]
    #[test]
    fn test_anthropic_config_from_provider_config_defaults() {
        let provider = minimal_provider_config();
        let cfg = AnthropicConfig::from(&provider);
        assert_eq!(cfg.base_url, "https://api.anthropic.com/v1");
        assert_eq!(cfg.model, "claude-3-5-sonnet-20241022");
        assert_eq!(cfg.timeout_seconds, 300);
    }

    // ── DeepSeek ────────────────────────────────────────────────────────────

    #[cfg(feature = "llm-deepseek")]
    #[test]
    fn test_deepseek_config_from_provider_config_typical() {
        let provider = typical_provider_config();
        let cfg = DeepSeekConfig::from(&provider);
        assert_eq!(cfg.api_key, "sk-test-key");
        assert_eq!(cfg.base_url, "https://custom.example.com/v1");
        assert_eq!(cfg.model, "gpt-4o");
        assert_eq!(cfg.timeout_seconds, 120);
    }

    #[cfg(feature = "llm-deepseek")]
    #[test]
    fn test_deepseek_config_from_provider_config_defaults() {
        let provider = minimal_provider_config();
        let cfg = DeepSeekConfig::from(&provider);
        assert_eq!(cfg.base_url, "https://api.deepseek.com/v1");
        assert_eq!(cfg.model, "deepseek-chat");
        assert_eq!(cfg.timeout_seconds, 60);
    }
}
