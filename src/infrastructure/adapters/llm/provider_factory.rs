// src/infrastructure/adapters/llm/provider_factory.rs
//
// LLM Provider Factory
//
// Factory for creating LLM provider adapters based on provider name and configuration.
// Supports OpenAI, DeepSeek, and Anthropic providers with automatic fallback to OpenAI
// if no provider is specified and OpenAI is configured.

use std::sync::Arc;
use thiserror::Error;

#[cfg(feature = "llm-anthropic")]
use super::anthropic_adapter::{AnthropicAdapter, AnthropicConfig};
#[cfg(feature = "llm-deepseek")]
use super::deepseek_adapter::{DeepSeekAdapter, DeepSeekConfig};
#[cfg(feature = "llm-openai")]
use super::openai_adapter::{OpenAIAdapter, OpenAIConfig};
use paladin_ports::output::llm_port::LlmPort;

#[doc(hidden)]
#[derive(Debug, Error)]
pub enum ProviderFactoryError {
    #[error("Unknown provider: {0}. Supported providers: openai, deepseek, anthropic")]
    UnknownProvider(String),
    #[error("Provider configuration missing: {0}")]
    ConfigurationMissing(String),
    #[error("Failed to create provider adapter: {0}")]
    AdapterCreationFailed(String),
}

/// Factory for creating LLM provider adapters
///
/// This factory creates instances of LLM adapters based on the provider name.
/// It handles configuration loading from environment variables and provides
/// clear error messages for unknown or misconfigured providers.
///
/// # Supported Providers
///
/// - `openai` - OpenAI GPT models
/// - `deepseek` - DeepSeek models
/// - `anthropic` - Anthropic Claude models
///
/// # Example
///
/// ```rust,no_run
/// use paladin::infrastructure::adapters::llm::provider_factory::LlmProviderFactory;
///
/// # fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let factory = LlmProviderFactory::new();
/// let provider = factory.create("deepseek")?;
/// # Ok(())
/// # }
/// ```
#[doc(hidden)]
pub struct LlmProviderFactory;

impl LlmProviderFactory {
    /// Create a new provider factory
    pub fn new() -> Self {
        Self
    }

    /// Create an LLM provider adapter by name
    ///
    /// # Arguments
    ///
    /// * `provider_name` - Name of the provider ("openai", "deepseek", or "anthropic")
    ///
    /// # Returns
    ///
    /// An `Arc<dyn LlmPort>` wrapping the provider adapter
    ///
    /// # Errors
    ///
    /// Returns `ProviderFactoryError` if:
    /// - Provider name is unknown
    /// - Required environment variables are missing
    /// - Provider configuration is invalid
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use paladin::infrastructure::adapters::llm::provider_factory::LlmProviderFactory;
    ///
    /// # fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let factory = LlmProviderFactory::new();
    ///
    /// // Create DeepSeek adapter
    /// let deepseek = factory.create("deepseek")?;
    ///
    /// // Create Anthropic adapter
    /// let anthropic = factory.create("anthropic")?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn create(&self, provider_name: &str) -> Result<Arc<dyn LlmPort>, ProviderFactoryError> {
        match provider_name.to_lowercase().as_str() {
            #[cfg(feature = "llm-openai")]
            "openai" => {
                let config = OpenAIConfig::from_env().map_err(|e| {
                    ProviderFactoryError::ConfigurationMissing(format!(
                        "OpenAI configuration error: {}. Ensure OPENAI_API_KEY is set.",
                        e
                    ))
                })?;

                let adapter = OpenAIAdapter::new(config).map_err(|e| {
                    ProviderFactoryError::AdapterCreationFailed(format!(
                        "Failed to create OpenAI adapter: {}",
                        e
                    ))
                })?;

                Ok(Arc::new(adapter))
            }
            #[cfg(feature = "llm-deepseek")]
            "deepseek" => {
                let config = DeepSeekConfig::from_env().map_err(|e| {
                    ProviderFactoryError::ConfigurationMissing(format!(
                        "DeepSeek configuration error: {}. Ensure DEEPSEEK_API_KEY is set.",
                        e
                    ))
                })?;

                let adapter = DeepSeekAdapter::new(config).map_err(|e| {
                    ProviderFactoryError::AdapterCreationFailed(format!(
                        "Failed to create DeepSeek adapter: {}",
                        e
                    ))
                })?;

                Ok(Arc::new(adapter))
            }
            #[cfg(feature = "llm-anthropic")]
            "anthropic" => {
                let config = AnthropicConfig::from_env().map_err(|e| {
                    ProviderFactoryError::ConfigurationMissing(format!(
                        "Anthropic configuration error: {}. Ensure ANTHROPIC_API_KEY is set.",
                        e
                    ))
                })?;

                let adapter = AnthropicAdapter::new(config).map_err(|e| {
                    ProviderFactoryError::AdapterCreationFailed(format!(
                        "Failed to create Anthropic adapter: {}",
                        e
                    ))
                })?;

                Ok(Arc::new(adapter))
            }
            _ => Err(ProviderFactoryError::UnknownProvider(
                provider_name.to_string(),
            )),
        }
    }

    /// Create an LLM provider adapter with custom configuration
    ///
    /// This method allows creating providers with specific configurations
    /// rather than loading from environment variables.
    ///
    /// # Arguments
    ///
    /// * `provider_name` - Name of the provider
    /// * `config` - Provider-specific configuration
    ///
    /// # Returns
    ///
    /// An `Arc<dyn LlmPort>` wrapping the provider adapter
    pub fn create_with_config(
        &self,
        provider_name: &str,
        config: ProviderConfig,
    ) -> Result<Arc<dyn LlmPort>, ProviderFactoryError> {
        match (provider_name.to_lowercase().as_str(), config) {
            #[cfg(feature = "llm-openai")]
            ("openai", ProviderConfig::OpenAI(config)) => {
                let adapter = OpenAIAdapter::new(config).map_err(|e| {
                    ProviderFactoryError::AdapterCreationFailed(format!(
                        "Failed to create OpenAI adapter: {}",
                        e
                    ))
                })?;
                Ok(Arc::new(adapter))
            }
            #[cfg(feature = "llm-deepseek")]
            ("deepseek", ProviderConfig::DeepSeek(config)) => {
                let adapter = DeepSeekAdapter::new(config).map_err(|e| {
                    ProviderFactoryError::AdapterCreationFailed(format!(
                        "Failed to create DeepSeek adapter: {}",
                        e
                    ))
                })?;
                Ok(Arc::new(adapter))
            }
            #[cfg(feature = "llm-anthropic")]
            ("anthropic", ProviderConfig::Anthropic(config)) => {
                let adapter = AnthropicAdapter::new(config).map_err(|e| {
                    ProviderFactoryError::AdapterCreationFailed(format!(
                        "Failed to create Anthropic adapter: {}",
                        e
                    ))
                })?;
                Ok(Arc::new(adapter))
            }
            (provider, _) => Err(ProviderFactoryError::UnknownProvider(provider.to_string())),
        }
    }

    /// Get the default provider name
    ///
    /// Returns "openai" if OPENAI_API_KEY is set, otherwise returns the first
    /// available provider based on environment variables.
    ///
    /// Priority order:
    /// 1. OpenAI (if configured)
    /// 2. DeepSeek (if configured)
    /// 3. Anthropic (if configured)
    /// 4. None if no providers are configured
    pub fn get_default_provider() -> Option<String> {
        // Check OpenAI first (legacy default)
        if std::env::var("OPENAI_API_KEY").is_ok() {
            return Some("openai".to_string());
        }

        // Check DeepSeek
        if std::env::var("DEEPSEEK_API_KEY").is_ok() {
            return Some("deepseek".to_string());
        }

        // Check Anthropic
        if std::env::var("ANTHROPIC_API_KEY").is_ok() {
            return Some("anthropic".to_string());
        }

        None
    }

    /// List all available providers based on environment configuration
    ///
    /// Returns a vector of provider names that have their API keys configured
    /// in the environment.
    pub fn list_available_providers() -> Vec<String> {
        let mut providers = Vec::new();

        if std::env::var("OPENAI_API_KEY").is_ok() {
            providers.push("openai".to_string());
        }

        if std::env::var("DEEPSEEK_API_KEY").is_ok() {
            providers.push("deepseek".to_string());
        }

        if std::env::var("ANTHROPIC_API_KEY").is_ok() {
            providers.push("anthropic".to_string());
        }

        providers
    }
}

impl Default for LlmProviderFactory {
    fn default() -> Self {
        Self::new()
    }
}

/// Provider-specific configuration types
///
/// This enum wraps the different configuration types for each provider,
/// allowing type-safe configuration passing.
#[doc(hidden)]
#[derive(Debug, Clone)]
pub enum ProviderConfig {
    #[cfg(feature = "llm-openai")]
    OpenAI(OpenAIConfig),
    #[cfg(feature = "llm-deepseek")]
    DeepSeek(DeepSeekConfig),
    #[cfg(feature = "llm-anthropic")]
    Anthropic(AnthropicConfig),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_factory_creation() {
        let factory = LlmProviderFactory::new();
        assert!(std::mem::size_of_val(&factory) == 0); // Zero-sized type
    }

    #[test]
    fn test_factory_default() {
        let factory = LlmProviderFactory;
        assert!(std::mem::size_of_val(&factory) == 0);
    }

    #[test]
    fn test_unknown_provider_error() {
        let factory = LlmProviderFactory::new();
        let result = factory.create("unknown_provider");

        assert!(result.is_err());
        if let Err(e) = result {
            assert!(matches!(e, ProviderFactoryError::UnknownProvider(_)));
            assert!(e.to_string().contains("unknown_provider"));
            assert!(e.to_string().contains("Supported providers"));
        }
    }

    #[cfg(all(
        feature = "llm-openai",
        feature = "llm-deepseek",
        feature = "llm-anthropic"
    ))]
    #[test]
    fn test_case_insensitive_provider_names() {
        let factory = LlmProviderFactory::new();

        // All these should work case-insensitively
        // If API keys are present in env, they succeed; if not, they fail with ConfigurationMissing
        // But they should NEVER fail with UnknownProvider (case sensitivity is working)
        let result1 = factory.create("DeepSeek");
        let result2 = factory.create("DEEPSEEK");
        let result3 = factory.create("deepseek");

        // All should either succeed (if DEEPSEEK_API_KEY is set) or fail with ConfigurationMissing
        // None should fail with UnknownProvider (which would indicate case sensitivity issue)
        for result in [result1, result2, result3] {
            match result {
                Ok(_) => {} // Success is fine if API key is present
                Err(ProviderFactoryError::ConfigurationMissing(_)) => {} // Expected without API key
                Err(ProviderFactoryError::UnknownProvider(_)) => {
                    panic!("Provider name not recognized - case insensitivity not working");
                }
                Err(e) => panic!("Unexpected error: {:?}", e),
            }
        }
    }

    #[test]
    fn test_list_available_providers() {
        // This test depends on environment variables
        // In a clean environment, should return empty list
        let providers = LlmProviderFactory::list_available_providers();

        // All returned providers should be known (list could be empty in clean environment)
        for provider in providers {
            assert!(
                provider == "openai" || provider == "deepseek" || provider == "anthropic",
                "Unknown provider in list: {}",
                provider
            );
        }
    }

    #[test]
    fn test_get_default_provider() {
        // This test depends on environment variables
        let default = LlmProviderFactory::get_default_provider();

        // If there is a default, it should be a known provider
        if let Some(provider) = default {
            assert!(
                provider == "openai" || provider == "deepseek" || provider == "anthropic",
                "Unknown default provider: {}",
                provider
            );
        }
    }

    #[cfg(feature = "llm-deepseek")]
    #[test]
    fn test_error_messages_are_actionable() {
        let factory = LlmProviderFactory::new();

        // Test unknown provider error
        let result = factory.create("invalid");
        if let Err(e) = result {
            let msg = e.to_string();
            assert!(
                msg.contains("Supported providers"),
                "Error should list supported providers"
            );
        }

        // Test missing configuration error
        let result = factory.create("deepseek");
        if let Err(e) = result {
            let msg = e.to_string();
            assert!(
                msg.contains("DEEPSEEK_API_KEY"),
                "Error should mention required env var"
            );
        }
    }

    #[cfg(feature = "llm-openai")]
    #[test]
    fn test_create_with_config_openai() {
        let factory = LlmProviderFactory::new();
        let config = OpenAIConfig {
            api_key: "test-key".to_string(),
            base_url: "https://api.openai.com/v1".to_string(),
            organization: None,
            timeout_seconds: 30,
            max_retries: 3,
        };

        let result = factory.create_with_config("openai", ProviderConfig::OpenAI(config));
        assert!(result.is_ok(), "Should create OpenAI adapter with config");
    }

    #[cfg(feature = "llm-deepseek")]
    #[test]
    fn test_create_with_config_deepseek() {
        let factory = LlmProviderFactory::new();
        let config = DeepSeekConfig {
            api_key: "test-key".to_string(),
            base_url: "https://api.deepseek.com/v1".to_string(),
            model: "deepseek-chat".to_string(),
            timeout_seconds: 60,
        };

        let result = factory.create_with_config("deepseek", ProviderConfig::DeepSeek(config));
        assert!(result.is_ok(), "Should create DeepSeek adapter with config");
    }

    #[cfg(feature = "llm-anthropic")]
    #[test]
    fn test_create_with_config_anthropic() {
        let factory = LlmProviderFactory::new();
        let config = AnthropicConfig {
            api_key: "test-key".to_string(),
            base_url: "https://api.anthropic.com/v1".to_string(),
            model: "claude-3-opus-20240229".to_string(),
            max_tokens: 4096,
            timeout_seconds: 45,
        };

        let result = factory.create_with_config("anthropic", ProviderConfig::Anthropic(config));
        assert!(
            result.is_ok(),
            "Should create Anthropic adapter with config"
        );
    }

    #[cfg(all(feature = "llm-openai", feature = "llm-deepseek"))]
    #[test]
    fn test_create_with_config_mismatched_provider_and_config() {
        let factory = LlmProviderFactory::new();
        let openai_config = OpenAIConfig {
            api_key: "test-key".to_string(),
            base_url: "https://api.openai.com/v1".to_string(),
            organization: None,
            timeout_seconds: 30,
            max_retries: 3,
        };

        // Try to create DeepSeek with OpenAI config - should fail
        let result = factory.create_with_config("deepseek", ProviderConfig::OpenAI(openai_config));
        assert!(
            result.is_err(),
            "Should fail with mismatched provider and config"
        );
        assert!(matches!(
            result.err().unwrap(),
            ProviderFactoryError::UnknownProvider(_)
        ));
    }

    #[cfg(feature = "llm-openai")]
    #[test]
    fn test_create_with_config_unknown_provider() {
        let factory = LlmProviderFactory::new();
        let config = OpenAIConfig {
            api_key: "test-key".to_string(),
            base_url: "https://api.openai.com/v1".to_string(),
            organization: None,
            timeout_seconds: 30,
            max_retries: 3,
        };

        let result = factory.create_with_config("unknown", ProviderConfig::OpenAI(config));
        assert!(result.is_err());
        assert!(matches!(
            result.err().unwrap(),
            ProviderFactoryError::UnknownProvider(_)
        ));
    }

    #[cfg(feature = "llm-openai")]
    #[test]
    fn test_create_with_config_case_insensitive() {
        let factory = LlmProviderFactory::new();
        let config = OpenAIConfig {
            api_key: "test-key".to_string(),
            base_url: "https://api.openai.com/v1".to_string(),
            organization: None,
            timeout_seconds: 30,
            max_retries: 3,
        };

        // All case variations should work
        let result1 = factory.create_with_config("OpenAI", ProviderConfig::OpenAI(config.clone()));
        let result2 = factory.create_with_config("OPENAI", ProviderConfig::OpenAI(config.clone()));
        let result3 = factory.create_with_config("openai", ProviderConfig::OpenAI(config));

        assert!(result1.is_ok());
        assert!(result2.is_ok());
        assert!(result3.is_ok());
    }

    #[cfg(all(
        feature = "llm-openai",
        feature = "llm-deepseek",
        feature = "llm-anthropic"
    ))]
    #[test]
    fn test_provider_config_enum_variants() {
        // Test that ProviderConfig enum can hold different config types
        let openai_config = ProviderConfig::OpenAI(OpenAIConfig {
            api_key: "test".to_string(),
            base_url: "url".to_string(),
            organization: None,
            timeout_seconds: 30,
            max_retries: 3,
        });

        let deepseek_config = ProviderConfig::DeepSeek(DeepSeekConfig {
            api_key: "test".to_string(),
            base_url: "url".to_string(),
            model: "model".to_string(),
            timeout_seconds: 30,
        });

        let anthropic_config = ProviderConfig::Anthropic(AnthropicConfig {
            api_key: "test".to_string(),
            base_url: "url".to_string(),
            model: "model".to_string(),
            max_tokens: 1000,
            timeout_seconds: 30,
        });

        // Verify we can match on variants
        assert!(matches!(openai_config, ProviderConfig::OpenAI(_)));
        assert!(matches!(deepseek_config, ProviderConfig::DeepSeek(_)));
        assert!(matches!(anthropic_config, ProviderConfig::Anthropic(_)));
    }

    #[cfg(feature = "llm-openai")]
    #[test]
    fn test_provider_config_debug_format() {
        let config = ProviderConfig::OpenAI(OpenAIConfig {
            api_key: "test-key".to_string(),
            base_url: "https://api.openai.com/v1".to_string(),
            organization: None,
            timeout_seconds: 30,
            max_retries: 3,
        });

        let debug_str = format!("{:?}", config);
        assert!(debug_str.contains("OpenAI"));
    }

    #[cfg(feature = "llm-deepseek")]
    #[test]
    fn test_provider_config_clone() {
        let config = ProviderConfig::DeepSeek(DeepSeekConfig {
            api_key: "test-key".to_string(),
            base_url: "https://api.deepseek.com/v1".to_string(),
            model: "deepseek-chat".to_string(),
            timeout_seconds: 60,
        });

        let cloned = config.clone();
        assert!(matches!(cloned, ProviderConfig::DeepSeek(_)));
    }

    #[test]
    fn test_provider_factory_error_debug() {
        let err1 = ProviderFactoryError::UnknownProvider("test".to_string());
        let err2 = ProviderFactoryError::ConfigurationMissing("config".to_string());
        let err3 = ProviderFactoryError::AdapterCreationFailed("failed".to_string());

        // Verify Debug trait works
        assert!(format!("{:?}", err1).contains("UnknownProvider"));
        assert!(format!("{:?}", err2).contains("ConfigurationMissing"));
        assert!(format!("{:?}", err3).contains("AdapterCreationFailed"));
    }

    #[cfg(feature = "llm-openai")]
    #[test]
    fn test_create_openai_without_env() {
        // Ensure OPENAI_API_KEY is not set for this test
        unsafe {
            std::env::remove_var("OPENAI_API_KEY");
        }

        let factory = LlmProviderFactory::new();
        let result = factory.create("openai");

        assert!(result.is_err());
        if let Err(e) = result {
            assert!(matches!(e, ProviderFactoryError::ConfigurationMissing(_)));
            assert!(e.to_string().contains("OPENAI_API_KEY"));
        }
    }

    #[cfg(feature = "llm-anthropic")]
    #[test]
    fn test_create_anthropic_without_env() {
        // Ensure ANTHROPIC_API_KEY is not set for this test
        unsafe {
            std::env::remove_var("ANTHROPIC_API_KEY");
        }

        let factory = LlmProviderFactory::new();
        let result = factory.create("anthropic");

        assert!(result.is_err());
        if let Err(e) = result {
            assert!(matches!(e, ProviderFactoryError::ConfigurationMissing(_)));
            assert!(e.to_string().contains("ANTHROPIC_API_KEY"));
        }
    }

    #[test]
    fn test_empty_provider_name_is_unknown() {
        let factory = LlmProviderFactory::new();
        let result = factory.create("");

        assert!(result.is_err());
        assert!(matches!(
            result.err().unwrap(),
            ProviderFactoryError::UnknownProvider(_)
        ));
    }

    #[test]
    fn test_whitespace_provider_name() {
        let factory = LlmProviderFactory::new();
        let result = factory.create("  openai  ");

        // Should fail because to_lowercase() doesn't trim
        assert!(result.is_err());
    }

    #[cfg(all(
        feature = "llm-openai",
        feature = "llm-deepseek",
        feature = "llm-anthropic"
    ))]
    #[test]
    fn test_get_default_provider_priority() {
        // This test verifies the priority logic exists
        // Actual behavior depends on environment variables

        // Save current env state
        let openai_backup = std::env::var("OPENAI_API_KEY").ok();
        let deepseek_backup = std::env::var("DEEPSEEK_API_KEY").ok();
        let anthropic_backup = std::env::var("ANTHROPIC_API_KEY").ok();

        unsafe {
            // Test with OpenAI set - should return openai
            std::env::set_var("OPENAI_API_KEY", "test");
            std::env::remove_var("DEEPSEEK_API_KEY");
            std::env::remove_var("ANTHROPIC_API_KEY");
            assert_eq!(
                LlmProviderFactory::get_default_provider(),
                Some("openai".to_string())
            );

            // Test with only DeepSeek - should return deepseek
            std::env::remove_var("OPENAI_API_KEY");
            std::env::set_var("DEEPSEEK_API_KEY", "test");
            std::env::remove_var("ANTHROPIC_API_KEY");
            assert_eq!(
                LlmProviderFactory::get_default_provider(),
                Some("deepseek".to_string())
            );

            // Test with only Anthropic - should return anthropic
            std::env::remove_var("OPENAI_API_KEY");
            std::env::remove_var("DEEPSEEK_API_KEY");
            std::env::set_var("ANTHROPIC_API_KEY", "test");
            assert_eq!(
                LlmProviderFactory::get_default_provider(),
                Some("anthropic".to_string())
            );

            // Test with none - should return None
            std::env::remove_var("OPENAI_API_KEY");
            std::env::remove_var("DEEPSEEK_API_KEY");
            std::env::remove_var("ANTHROPIC_API_KEY");
            assert_eq!(LlmProviderFactory::get_default_provider(), None);

            // Restore env state
            if let Some(key) = openai_backup {
                std::env::set_var("OPENAI_API_KEY", key);
            }
            if let Some(key) = deepseek_backup {
                std::env::set_var("DEEPSEEK_API_KEY", key);
            }
            if let Some(key) = anthropic_backup {
                std::env::set_var("ANTHROPIC_API_KEY", key);
            }
        }
    }

    #[cfg(all(
        feature = "llm-openai",
        feature = "llm-deepseek",
        feature = "llm-anthropic"
    ))]
    #[test]
    fn test_list_available_providers_comprehensive() {
        // Save current env state
        let openai_backup = std::env::var("OPENAI_API_KEY").ok();
        let deepseek_backup = std::env::var("DEEPSEEK_API_KEY").ok();
        let anthropic_backup = std::env::var("ANTHROPIC_API_KEY").ok();

        unsafe {
            // Test with all providers configured
            std::env::set_var("OPENAI_API_KEY", "test1");
            std::env::set_var("DEEPSEEK_API_KEY", "test2");
            std::env::set_var("ANTHROPIC_API_KEY", "test3");
            let all_providers = LlmProviderFactory::list_available_providers();
            assert_eq!(all_providers.len(), 3);
            assert!(all_providers.contains(&"openai".to_string()));
            assert!(all_providers.contains(&"deepseek".to_string()));
            assert!(all_providers.contains(&"anthropic".to_string()));

            // Test with no providers configured
            std::env::remove_var("OPENAI_API_KEY");
            std::env::remove_var("DEEPSEEK_API_KEY");
            std::env::remove_var("ANTHROPIC_API_KEY");
            let no_providers = LlmProviderFactory::list_available_providers();
            assert_eq!(no_providers.len(), 0);

            // Test with only one provider
            std::env::set_var("DEEPSEEK_API_KEY", "test");
            std::env::remove_var("OPENAI_API_KEY");
            std::env::remove_var("ANTHROPIC_API_KEY");
            let one_provider = LlmProviderFactory::list_available_providers();
            assert_eq!(one_provider.len(), 1);
            assert_eq!(one_provider[0], "deepseek");

            // Restore env state
            if let Some(key) = openai_backup {
                std::env::set_var("OPENAI_API_KEY", key);
            } else {
                std::env::remove_var("OPENAI_API_KEY");
            }
            if let Some(key) = deepseek_backup {
                std::env::set_var("DEEPSEEK_API_KEY", key);
            } else {
                std::env::remove_var("DEEPSEEK_API_KEY");
            }
            if let Some(key) = anthropic_backup {
                std::env::set_var("ANTHROPIC_API_KEY", key);
            } else {
                std::env::remove_var("ANTHROPIC_API_KEY");
            }
        }
    }
}
