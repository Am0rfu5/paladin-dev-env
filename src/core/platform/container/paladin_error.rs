//! Paladin error types
//!
//! This module defines error types for Paladin execution operations.
use crate::core::platform::container::arsenal::ArsenalError;
use crate::core::platform::container::garrison_error::GarrisonError;
use thiserror::Error;

/// Errors that can occur during Paladin operations.
///
/// # Example
///
/// ```
/// use paladin::core::platform::container::paladin_error::PaladinError;
///
/// let error = PaladinError::ConfigurationError("Invalid temperature".to_string());
/// assert_eq!(error.to_string(), "Configuration error: Invalid temperature");
/// ```
#[derive(Debug, Error)]
pub enum PaladinError {
    /// Configuration validation failed
    #[error("Configuration error: {0}")]
    ConfigurationError(String),

    /// Error during Paladin execution
    #[error("Execution error: {0}")]
    ExecutionError(String),

    /// Error from the LLM provider
    #[error("LLM error: {0}")]
    LlmError(String),

    /// Execution exceeded the configured timeout
    #[error("Timeout after {0} seconds")]
    Timeout(u64),

    /// A stop word was detected in the output
    #[error("Stop word detected: {0}")]
    StopWordDetected(String),

    /// Circuit breaker is open, rejecting requests
    #[error("Circuit breaker open: too many failures")]
    CircuitBreakerOpen,

    /// Maximum retry attempts exceeded
    #[error("Maximum retry attempts ({0}) exceeded")]
    MaxRetriesExceeded(u32),

    /// Error from the Garrison memory system
    #[error("Garrison error: {0}")]
    GarrisonError(#[from] GarrisonError),

    /// Garrison is required for multi-turn conversations but not provided
    #[error("Garrison is required for multi-turn conversations")]
    GarrisonRequired,

    /// Error from the Arsenal tool system
    #[error("Arsenal error: {0}")]
    ArsenalError(#[from] ArsenalError),
}

impl PaladinError {
    /// Check if this error is retryable.
    pub fn is_retryable(&self) -> bool {
        matches!(
            self,
            PaladinError::LlmError(_) | PaladinError::ExecutionError(_)
        )
    }

    /// Check if this error represents a terminal state.
    pub fn is_terminal(&self) -> bool {
        matches!(
            self,
            PaladinError::Timeout(_)
                | PaladinError::StopWordDetected(_)
                | PaladinError::CircuitBreakerOpen
                | PaladinError::MaxRetriesExceeded(_)
                | PaladinError::GarrisonRequired
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_configuration_error_message() {
        let error = PaladinError::ConfigurationError("Invalid parameter".to_string());
        assert_eq!(error.to_string(), "Configuration error: Invalid parameter");
    }

    #[test]
    fn test_timeout_error_message() {
        let error = PaladinError::Timeout(300);
        assert_eq!(error.to_string(), "Timeout after 300 seconds");
    }

    #[test]
    fn test_is_retryable() {
        assert!(PaladinError::LlmError("temp".to_string()).is_retryable());
        assert!(PaladinError::ExecutionError("temp".to_string()).is_retryable());
        assert!(!PaladinError::ConfigurationError("temp".to_string()).is_retryable());
        assert!(!PaladinError::Timeout(100).is_retryable());
    }

    #[test]
    fn test_is_terminal() {
        assert!(PaladinError::Timeout(100).is_terminal());
        assert!(PaladinError::CircuitBreakerOpen.is_terminal());
        assert!(PaladinError::GarrisonRequired.is_terminal());
        assert!(!PaladinError::LlmError("temp".to_string()).is_terminal());
    }

    #[test]
    fn test_garrison_error_conversion() {
        let garrison_error = GarrisonError::StorageError("test".to_string());
        let paladin_error: PaladinError = garrison_error.into();
        assert!(matches!(paladin_error, PaladinError::GarrisonError(_)));
    }
}
