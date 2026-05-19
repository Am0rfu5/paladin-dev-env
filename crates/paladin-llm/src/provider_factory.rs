//! # LLM Provider Factory
//!
//! Creates [`paladin_ports::output::llm_port::LlmPort`] adapter instances by
//! provider name. Supports any combination of the `openai`, `anthropic`, and
//! `deepseek` feature flags.

use std::sync::Arc;

use paladin_ports::output::llm_port::LlmPort;
use thiserror::Error;

/// Errors that can be returned by [`LlmProviderFactory`].
#[derive(Debug, Error)]
pub enum ProviderFactoryError {
    /// The requested provider name is not recognised.
    #[error("Unknown provider: {0}. Supported providers: openai, deepseek, anthropic")]
    UnknownProvider(String),

    /// Required environment variables or configuration are missing.
    #[error("Provider configuration missing: {0}")]
    ConfigurationMissing(String),

    /// The provider adapter could not be constructed.
    #[error("Failed to create provider adapter: {0}")]
    AdapterCreationFailed(String),
}

/// Factory for creating LLM provider adapters by name.
///
/// Each provider is only available when the corresponding feature flag is
/// enabled (`openai`, `anthropic`, or `deepseek`).
///
/// # Example
///
/// ```rust,no_run
/// # #[cfg(feature = "openai")]
/// # {
/// use paladin_llm::provider_factory::LlmProviderFactory;
///
/// let factory = LlmProviderFactory::new();
/// let provider = factory.create("openai").expect("OPENAI_API_KEY must be set");
/// # }
/// ```
pub struct LlmProviderFactory;

impl LlmProviderFactory {
    /// Create a new provider factory.
    pub fn new() -> Self {
        Self
    }

    /// Create an [`LlmPort`] adapter by provider name.
    ///
    /// Configuration is loaded from environment variables (see each adapter's
    /// `*Config::from_env()` for the expected variable names).
    ///
    /// # Errors
    ///
    /// Returns [`ProviderFactoryError`] if the provider name is unknown, a
    /// required environment variable is absent, or the adapter fails to
    /// initialise.
    pub fn create(&self, provider_name: &str) -> Result<Arc<dyn LlmPort>, ProviderFactoryError> {
        match provider_name.to_lowercase().as_str() {
            #[cfg(feature = "openai")]
            "openai" => {
                use crate::openai::{OpenAIAdapter, OpenAIConfig};
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
            #[cfg(feature = "deepseek")]
            "deepseek" => {
                use crate::deepseek::{DeepSeekAdapter, DeepSeekConfig};
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
            #[cfg(feature = "anthropic")]
            "anthropic" => {
                use crate::anthropic::{AnthropicAdapter, AnthropicConfig};
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
            other => Err(ProviderFactoryError::UnknownProvider(other.to_string())),
        }
    }

    /// Return the name of the first available provider based on environment
    /// variables, or `None` if no API keys are set.
    ///
    /// Priority: OpenAI → DeepSeek → Anthropic.
    pub fn get_default_provider() -> Option<String> {
        if std::env::var("OPENAI_API_KEY").is_ok() {
            return Some("openai".to_string());
        }
        if std::env::var("DEEPSEEK_API_KEY").is_ok() {
            return Some("deepseek".to_string());
        }
        if std::env::var("ANTHROPIC_API_KEY").is_ok() {
            return Some("anthropic".to_string());
        }
        None
    }

    /// Return the names of all providers that have API keys configured.
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_factory_creation() {
        let factory = LlmProviderFactory::new();
        assert_eq!(std::mem::size_of_val(&factory), 0);
    }

    #[test]
    fn test_unknown_provider_returns_error() {
        let factory = LlmProviderFactory::new();
        let result = factory.create("bogus_provider");
        assert!(result.is_err());
        if let Err(ProviderFactoryError::UnknownProvider(name)) = result {
            assert_eq!(name, "bogus_provider");
        } else {
            panic!("Expected UnknownProvider error");
        }
    }

    #[test]
    fn test_list_available_providers_returns_vec() {
        // Smoke test — just ensure it returns without panicking.
        let _ = LlmProviderFactory::list_available_providers();
    }
}
