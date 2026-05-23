//! LLM provider configuration types for the paladin-llm crate.

use serde::{Deserialize, Serialize};

/// Configuration for an individual LLM provider.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LlmProviderConfig {
    /// API key for the provider (can use `${ENV_VAR}` syntax).
    pub api_key: String,
    /// Base URL for the API endpoint.
    pub base_url: Option<String>,
    /// Default model to use.
    pub default_model: Option<String>,
    /// Default temperature (0.0–2.0).
    pub default_temperature: Option<f32>,
    /// Default timeout in seconds.
    pub timeout_seconds: Option<u64>,
    /// Maximum retries for failed requests.
    pub max_retries: Option<u32>,
}

/// Configuration for all LLM providers.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LlmConfig {
    /// Default provider to use if not specified.
    /// Options: `"openai"`, `"deepseek"`, `"anthropic"`.
    pub default_provider: Option<String>,
    /// OpenAI configuration.
    pub openai: Option<LlmProviderConfig>,
    /// DeepSeek configuration.
    pub deepseek: Option<LlmProviderConfig>,
    /// Anthropic configuration.
    pub anthropic: Option<LlmProviderConfig>,
}

impl Default for LlmConfig {
    fn default() -> Self {
        Self {
            default_provider: Some("openai".to_string()),
            openai: None,
            deepseek: None,
            anthropic: None,
        }
    }
}

impl LlmConfig {
    /// Validate LLM configuration.
    ///
    /// Returns `Err(String)` describing the first validation failure found.
    pub fn validate(&self) -> Result<(), String> {
        if let Some(default) = &self.default_provider {
            match default.as_str() {
                "openai" if self.openai.is_none() => {
                    return Err(
                        "Default provider is 'openai' but openai config is not present".to_string(),
                    );
                }
                "deepseek" if self.deepseek.is_none() => {
                    return Err(
                        "Default provider is 'deepseek' but deepseek config is not present"
                            .to_string(),
                    );
                }
                "anthropic" if self.anthropic.is_none() => {
                    return Err(
                        "Default provider is 'anthropic' but anthropic config is not present"
                            .to_string(),
                    );
                }
                "openai" | "deepseek" | "anthropic" => {}
                _ => {
                    return Err(format!(
                        "Invalid default provider: {}. Must be 'openai', 'deepseek', or 'anthropic'",
                        default
                    ));
                }
            }
        }

        if let Some(openai) = &self.openai
            && openai.api_key.is_empty()
        {
            return Err("OpenAI API key cannot be empty".to_string());
        }

        if let Some(deepseek) = &self.deepseek
            && deepseek.api_key.is_empty()
        {
            return Err("DeepSeek API key cannot be empty".to_string());
        }

        if let Some(anthropic) = &self.anthropic
            && anthropic.api_key.is_empty()
        {
            return Err("Anthropic API key cannot be empty".to_string());
        }

        Ok(())
    }

    /// Get the provider config for a specific provider name (case-insensitive).
    pub fn get_provider_config(&self, provider_name: &str) -> Option<&LlmProviderConfig> {
        match provider_name.to_lowercase().as_str() {
            "openai" => self.openai.as_ref(),
            "deepseek" => self.deepseek.as_ref(),
            "anthropic" => self.anthropic.as_ref(),
            _ => None,
        }
    }

    /// Get the default provider name.
    pub fn get_default_provider_name(&self) -> Option<String> {
        self.default_provider.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_llm_config_default() {
        let config = LlmConfig::default();
        assert_eq!(config.default_provider, Some("openai".to_string()));
        assert!(config.openai.is_none());
        assert!(config.deepseek.is_none());
        assert!(config.anthropic.is_none());
    }

    #[test]
    fn test_llm_config_validate_default_provider_must_be_configured() {
        let config = LlmConfig {
            default_provider: Some("openai".to_string()),
            openai: None,
            deepseek: None,
            anthropic: None,
        };
        assert!(config.validate().is_err());

        let config = LlmConfig {
            default_provider: Some("deepseek".to_string()),
            openai: None,
            deepseek: None,
            anthropic: None,
        };
        assert!(config.validate().is_err());

        let config = LlmConfig {
            default_provider: Some("anthropic".to_string()),
            openai: None,
            deepseek: None,
            anthropic: None,
        };
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_llm_config_validate_invalid_provider_name() {
        let config = LlmConfig {
            default_provider: Some("invalid_provider".to_string()),
            openai: Some(LlmProviderConfig {
                api_key: "key".to_string(),
                base_url: None,
                default_model: None,
                default_temperature: None,
                timeout_seconds: None,
                max_retries: None,
            }),
            deepseek: None,
            anthropic: None,
        };
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_llm_config_validate_empty_api_key() {
        let config = LlmConfig {
            default_provider: Some("openai".to_string()),
            openai: Some(LlmProviderConfig {
                api_key: "".to_string(),
                base_url: None,
                default_model: None,
                default_temperature: None,
                timeout_seconds: None,
                max_retries: None,
            }),
            deepseek: None,
            anthropic: None,
        };
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_llm_config_validate_success() {
        let config = LlmConfig {
            default_provider: Some("deepseek".to_string()),
            openai: None,
            deepseek: Some(LlmProviderConfig {
                api_key: "test-key".to_string(),
                base_url: Some("https://api.deepseek.com/v1".to_string()),
                default_model: Some("deepseek-chat".to_string()),
                default_temperature: Some(0.7),
                timeout_seconds: Some(300),
                max_retries: Some(3),
            }),
            anthropic: None,
        };
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_llm_config_get_provider_config() {
        let openai_config = LlmProviderConfig {
            api_key: "openai-key".to_string(),
            base_url: None,
            default_model: None,
            default_temperature: None,
            timeout_seconds: None,
            max_retries: None,
        };

        let config = LlmConfig {
            default_provider: Some("openai".to_string()),
            openai: Some(openai_config),
            deepseek: None,
            anthropic: None,
        };

        let retrieved = config.get_provider_config("openai");
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().api_key, "openai-key");

        let not_found = config.get_provider_config("deepseek");
        assert!(not_found.is_none());
    }

    #[test]
    fn test_llm_config_get_provider_config_case_insensitive() {
        let deepseek_config = LlmProviderConfig {
            api_key: "deepseek-key".to_string(),
            base_url: None,
            default_model: None,
            default_temperature: None,
            timeout_seconds: None,
            max_retries: None,
        };

        let config = LlmConfig {
            default_provider: Some("deepseek".to_string()),
            openai: None,
            deepseek: Some(deepseek_config),
            anthropic: None,
        };

        assert!(config.get_provider_config("DeepSeek").is_some());
        assert!(config.get_provider_config("DEEPSEEK").is_some());
        assert!(config.get_provider_config("deepseek").is_some());
    }

    #[test]
    fn test_llm_config_get_default_provider_name() {
        let config = LlmConfig {
            default_provider: Some("anthropic".to_string()),
            openai: None,
            deepseek: None,
            anthropic: Some(LlmProviderConfig {
                api_key: "key".to_string(),
                base_url: None,
                default_model: None,
                default_temperature: None,
                timeout_seconds: None,
                max_retries: None,
            }),
        };

        assert_eq!(
            config.get_default_provider_name(),
            Some("anthropic".to_string())
        );
    }
}
