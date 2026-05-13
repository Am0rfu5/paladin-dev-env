//! Herald error types
//!
//! This module defines error types for the Herald output formatting system.
//! Herald errors occur during result formatting operations and include
//! serialization failures, template errors, and I/O issues.

use std::io;

/// Errors that can occur during Herald formatting operations
#[derive(Debug, thiserror::Error)]
pub enum HeraldError {
    /// Serialization error occurred while formatting output
    #[error("Serialization error: {0}")]
    SerializationError(String),

    /// Template processing error occurred
    #[error("Template error: {0}")]
    TemplateError(String),

    /// Invalid result structure provided for formatting
    #[error("Invalid result structure: {0}")]
    InvalidResult(String),

    /// I/O error occurred during formatting
    #[error("IO error: {0}")]
    IoError(#[from] io::Error),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialization_error_creation() {
        let error = HeraldError::SerializationError("Failed to serialize JSON".to_string());
        assert_eq!(
            error.to_string(),
            "Serialization error: Failed to serialize JSON"
        );
    }

    #[test]
    fn test_template_error_creation() {
        let error = HeraldError::TemplateError("Missing variable".to_string());
        assert_eq!(error.to_string(), "Template error: Missing variable");
    }

    #[test]
    fn test_invalid_result_error_creation() {
        let error = HeraldError::InvalidResult("Missing required field".to_string());
        assert_eq!(
            error.to_string(),
            "Invalid result structure: Missing required field"
        );
    }

    #[test]
    fn test_io_error_conversion() {
        let io_error = io::Error::new(io::ErrorKind::NotFound, "File not found");
        let herald_error: HeraldError = io_error.into();
        assert!(herald_error.to_string().contains("File not found"));
    }

    #[test]
    fn test_error_is_send_sync() {
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<HeraldError>();
    }
}
