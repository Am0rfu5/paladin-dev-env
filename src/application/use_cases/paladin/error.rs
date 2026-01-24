//! Paladin Error Types
//!
//! This module defines error types for Paladin operations using the `thiserror` crate
//! for ergonomic error handling and clear error messages.

use crate::application::ports::output::garrison_port::GarrisonError;
use crate::core::platform::container::arsenal::ArsenalError;
use thiserror::Error;

/// Errors that can occur during Paladin operations
///
/// All variants include descriptive messages to help with debugging and user feedback.
///
/// # Example
///
/// ```
/// use paladin::application::use_cases::paladin::error::PaladinError;
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
    /// Check if this error is retryable
    ///
    /// Some errors like configuration errors are not worth retrying,
    /// while others like transient LLM errors might succeed on retry.
    pub fn is_retryable(&self) -> bool {
        matches!(
            self,
            PaladinError::LlmError(_) | PaladinError::ExecutionError(_)
        )
    }

    /// Check if this error represents a terminal state
    ///
    /// Terminal errors indicate the Paladin cannot continue processing.
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
    fn test_execution_error_message() {
        let error = PaladinError::ExecutionError("Failed to process".to_string());
        assert_eq!(error.to_string(), "Execution error: Failed to process");
    }

    #[test]
    fn test_llm_error_message() {
        let error = PaladinError::LlmError("API rate limit".to_string());
        assert_eq!(error.to_string(), "LLM error: API rate limit");
    }

    #[test]
    fn test_timeout_error_message() {
        let error = PaladinError::Timeout(300);
        assert_eq!(error.to_string(), "Timeout after 300 seconds");
    }

    #[test]
    fn test_stop_word_detected_message() {
        let error = PaladinError::StopWordDetected("STOP".to_string());
        assert_eq!(error.to_string(), "Stop word detected: STOP");
    }

    #[test]
    fn test_circuit_breaker_open_message() {
        let error = PaladinError::CircuitBreakerOpen;
        assert_eq!(error.to_string(), "Circuit breaker open: too many failures");
    }

    #[test]
    fn test_max_retries_exceeded_message() {
        let error = PaladinError::MaxRetriesExceeded(3);
        assert_eq!(error.to_string(), "Maximum retry attempts (3) exceeded");
    }

    #[test]
    fn test_is_retryable() {
        assert!(PaladinError::LlmError("temp".to_string()).is_retryable());
        assert!(PaladinError::ExecutionError("temp".to_string()).is_retryable());
        assert!(!PaladinError::ConfigurationError("temp".to_string()).is_retryable());
        assert!(!PaladinError::Timeout(100).is_retryable());
        assert!(!PaladinError::CircuitBreakerOpen.is_retryable());
    }

    #[test]
    fn test_is_terminal() {
        assert!(PaladinError::Timeout(100).is_terminal());
        assert!(PaladinError::StopWordDetected("STOP".to_string()).is_terminal());
        assert!(PaladinError::CircuitBreakerOpen.is_terminal());
        assert!(PaladinError::MaxRetriesExceeded(3).is_terminal());
        assert!(PaladinError::GarrisonRequired.is_terminal());
        assert!(!PaladinError::LlmError("temp".to_string()).is_terminal());
        assert!(!PaladinError::ConfigurationError("temp".to_string()).is_terminal());
    }

    #[test]
    fn test_garrison_error_conversion() {
        use crate::application::ports::output::garrison_port::GarrisonError;

        let garrison_error = GarrisonError::StorageError("test".to_string());
        let paladin_error: PaladinError = garrison_error.into();

        assert!(matches!(paladin_error, PaladinError::GarrisonError(_)));
    }

    #[test]
    fn test_garrison_required_error_message() {
        let error = PaladinError::GarrisonRequired;
        assert_eq!(
            error.to_string(),
            "Garrison is required for multi-turn conversations"
        );
    }
}
