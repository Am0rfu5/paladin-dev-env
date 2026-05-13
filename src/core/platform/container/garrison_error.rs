//! Garrison error types
//!
//! This module defines error types for Garrison (memory) operations.
use thiserror::Error;

/// Errors that can occur during Garrison memory operations.
#[derive(Debug, Error)]
pub enum GarrisonError {
    /// Error occurred in underlying storage (database, file system, network, etc.).
    ///
    /// **Retryable**: Yes (may be transient network/database issue)
    #[error("Storage error: {0}")]
    StorageError(String),

    /// Failed to serialize or deserialize data.
    ///
    /// **Retryable**: No (data format issue)
    #[error("Serialization error: {0}")]
    SerializationError(String),

    /// Failed to calculate token count.
    ///
    /// **Retryable**: Yes (if using external tokenizer service)
    #[error("Tokenization error: {0}")]
    TokenizationError(String),

    /// Requested entry was not found.
    ///
    /// **Retryable**: No (entry doesn't exist)
    #[error("Entry not found: {0}")]
    NotFound(String),

    /// Configuration is invalid.
    ///
    /// **Retryable**: No (requires configuration fix)
    #[error("Configuration error: {0}")]
    ConfigurationError(String),

    /// Generic error with custom message.
    ///
    /// **Retryable**: Implementation-specific
    #[error("{0}")]
    Custom(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_storage_error_display() {
        let e = GarrisonError::StorageError("db timeout".to_string());
        assert_eq!(e.to_string(), "Storage error: db timeout");
    }

    #[test]
    fn test_not_found_display() {
        let e = GarrisonError::NotFound("entry-123".to_string());
        assert_eq!(e.to_string(), "Entry not found: entry-123");
    }
}
