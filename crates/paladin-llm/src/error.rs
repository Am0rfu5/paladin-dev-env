//! # LLM Provider Errors
//!
//! Defines the error type for all LLM provider adapters in this crate.
//! [`LlmProviderError`] is the crate-local error type; a `From` conversion
//! to [`paladin_ports::output::llm_port::LlmError`] is provided so errors
//! propagate cleanly across the port boundary.

use paladin_ports::output::llm_port::LlmError;
use thiserror::Error;

/// Errors that can occur in LLM provider adapters.
///
/// This type is used internally in `paladin-llm`. At the port boundary it is
/// converted into [`LlmError`] via the [`From`] implementation below.
#[derive(Debug, Error, Clone)]
pub enum LlmProviderError {
    /// The API key is missing or invalid.
    #[error("Authentication error: {0}")]
    AuthenticationError(String),

    /// A network-level failure (connection, timeout, DNS, etc.).
    #[error("Network error: {0}")]
    NetworkError(String),

    /// The request was rate-limited by the provider.
    #[error("Rate limit exceeded")]
    RateLimitExceeded,

    /// The prompt or request was rejected as invalid.
    #[error("Invalid prompt: {0}")]
    InvalidPrompt(String),

    /// The response could not be parsed or the provider returned unexpected data.
    #[error("Processing error: {0}")]
    ProcessingError(String),

    /// The token limit was exceeded for this request.
    #[error("Token limit exceeded")]
    TokenLimitExceeded,

    /// The requested model is not available from this provider.
    #[error("Model not available: {0}")]
    ModelNotAvailable(String),

    /// A request timed out.
    #[error("Timeout: {0}")]
    Timeout(String),

    /// Adapter configuration is missing or invalid.
    #[error("Configuration error: {0}")]
    ConfigurationError(String),
}

impl From<LlmProviderError> for LlmError {
    fn from(err: LlmProviderError) -> Self {
        match err {
            LlmProviderError::AuthenticationError(msg) => LlmError::AuthenticationError(msg),
            LlmProviderError::NetworkError(msg) => LlmError::NetworkError(msg),
            LlmProviderError::RateLimitExceeded => LlmError::RateLimitExceeded,
            LlmProviderError::InvalidPrompt(msg) => LlmError::InvalidPrompt(msg),
            LlmProviderError::ProcessingError(msg) => LlmError::ProcessingError(msg),
            LlmProviderError::TokenLimitExceeded => LlmError::TokenLimitExceeded,
            LlmProviderError::ModelNotAvailable(msg) => LlmError::ModelNotAvailable(msg),
            LlmProviderError::Timeout(msg) => LlmError::Timeout(msg),
            LlmProviderError::ConfigurationError(msg) => {
                LlmError::ProcessingError(format!("Configuration error: {}", msg))
            }
        }
    }
}
