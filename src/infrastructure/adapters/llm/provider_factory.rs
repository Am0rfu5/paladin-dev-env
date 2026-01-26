// src/infrastructure/adapters/llm/provider_factory.rs
//
// LLM Provider Factory
//
// Factory for creating LLM provider adapters based on provider name and configuration.
// Supports OpenAI, DeepSeek, and Anthropic providers with automatic fallback to OpenAI
// if no provider is specified and OpenAI is configured.

use std::sync::Arc;
use thiserror::Error;

use super::anthropic_adapter::{AnthropicAdapter, AnthropicConfig};
use super::deepseek_adapter::{DeepSeekAdapter, DeepSeekConfig};
use super::openai_adapter::{OpenAIAdapter, OpenAIConfig};
use crate::application::ports::output::llm_port::LlmPort;

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
            ("openai", ProviderConfig::OpenAI(config)) => {
                let adapter = OpenAIAdapter::new(config).map_err(|e| {
                    ProviderFactoryError::AdapterCreationFailed(format!(
                        "Failed to create OpenAI adapter: {}",
                        e
                    ))
                })?;
                Ok(Arc::new(adapter))
            }
            ("deepseek", ProviderConfig::DeepSeek(config)) => {
                let adapter = DeepSeekAdapter::new(config).map_err(|e| {
                    ProviderFactoryError::AdapterCreationFailed(format!(
                        "Failed to create DeepSeek adapter: {}",
                        e
                    ))
                })?;
                Ok(Arc::new(adapter))
            }
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
#[derive(Debug, Clone)]
pub enum ProviderConfig {
    OpenAI(OpenAIConfig),
    DeepSeek(DeepSeekConfig),
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
        let factory = LlmProviderFactory::default();
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

    #[test]
    fn test_case_insensitive_provider_names() {
        let factory = LlmProviderFactory::new();

        // All these should fail with configuration missing (not unknown provider)
        // because the provider names are recognized
        let result1 = factory.create("DeepSeek");
        let result2 = factory.create("DEEPSEEK");
        let result3 = factory.create("deepseek");

        // All should be configuration errors, not unknown provider errors
        assert!(matches!(
            result1.err().unwrap(),
            ProviderFactoryError::ConfigurationMissing(_)
        ));
        assert!(matches!(
            result2.err().unwrap(),
            ProviderFactoryError::ConfigurationMissing(_)
        ));
        assert!(matches!(
            result3.err().unwrap(),
            ProviderFactoryError::ConfigurationMissing(_)
        ));
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
}
