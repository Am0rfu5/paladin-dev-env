// src/infrastructure/adapters/llm/config_bridge.rs
//
// The `From<&LlmProviderConfig>` conversions for OpenAI, Anthropic, DeepSeek
// and the vision `From<&VisionConfig>` conversion now live in
// `crates/paladin-llm/src/config/bridge.rs`.  Both source and target types
// are defined in `paladin-llm`, so the impls must live there (orphan rule).
//
// This file is kept for the integration-level tests that exercise those
// conversions from the root-crate perspective.

// ── Tests ────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use paladin_llm::config::llm::LlmProviderConfig;

    #[cfg(feature = "llm-openai")]
    use paladin_llm::openai::adapter::OpenAIConfig;

    #[cfg(feature = "llm-anthropic")]
    use paladin_llm::anthropic::adapter::AnthropicConfig;

    #[cfg(feature = "llm-deepseek")]
    use paladin_llm::deepseek::adapter::DeepSeekConfig;

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
