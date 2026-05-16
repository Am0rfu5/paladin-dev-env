//! Citadel Error Types
//!
//! This module defines error types for the Citadel state persistence system.
//! All errors follow the fail-fast principle with descriptive messages for debugging.

use std::io;
use thiserror::Error;
use uuid::Uuid;

/// Errors that can occur during Citadel state persistence operations
///
/// All errors include descriptive messages to aid in debugging and follow
/// the fail-fast principle - no partial recovery is attempted.
///
/// # Examples
///
/// ```rust
/// use paladin_core::platform::container::citadel_error::CitadelError;
/// use uuid::Uuid;
///
/// let error = CitadelError::StateNotFound(Uuid::new_v4());
/// assert!(error.to_string().contains("State not found"));
/// ```
#[derive(Debug, Error)]
pub enum CitadelError {
    /// State file was not found for the given ID
    #[error("State not found: {0}")]
    StateNotFound(Uuid),

    /// State file exists but contains corrupted or invalid JSON
    #[error("Corrupted state file: {0}")]
    CorruptedState(String),

    /// State file has incompatible schema version
    #[error("Incompatible state version: expected {expected}, found {found}")]
    IncompatibleVersion {
        /// Expected schema version
        expected: String,
        /// Found schema version in file
        found: String,
    },

    /// File system I/O error occurred
    #[error("I/O error: {0}")]
    IoError(#[from] io::Error),

    /// JSON serialization/deserialization error
    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),

    /// Invalid state directory path
    #[error("Invalid state directory: {0}")]
    InvalidDirectory(String),

    /// Permission denied for state directory operations
    #[error("Permission denied: {0}")]
    PermissionDenied(String),

    /// State directory creation failed
    #[error("Failed to create state directory: {0}")]
    DirectoryCreationFailed(String),
}

impl CitadelError {
    /// Creates a corrupted state error with context
    pub fn corrupted(message: impl Into<String>) -> Self {
        Self::CorruptedState(message.into())
    }

    /// Creates an incompatible version error
    pub fn incompatible(expected: impl Into<String>, found: impl Into<String>) -> Self {
        Self::IncompatibleVersion {
            expected: expected.into(),
            found: found.into(),
        }
    }

    /// Creates an invalid directory error
    pub fn invalid_directory(message: impl Into<String>) -> Self {
        Self::InvalidDirectory(message.into())
    }

    /// Creates a permission denied error
    pub fn permission_denied(message: impl Into<String>) -> Self {
        Self::PermissionDenied(message.into())
    }

    /// Creates a directory creation failed error
    pub fn directory_creation_failed(message: impl Into<String>) -> Self {
        Self::DirectoryCreationFailed(message.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_state_not_found_error() {
        let id = Uuid::new_v4();
        let error = CitadelError::StateNotFound(id);
        let error_msg = error.to_string();
        assert!(error_msg.contains("State not found"));
        assert!(error_msg.contains(&id.to_string()));
    }

    #[test]
    fn test_corrupted_state_error() {
        let error = CitadelError::corrupted("Invalid JSON structure");
        let error_msg = error.to_string();
        assert!(error_msg.contains("Corrupted state file"));
        assert!(error_msg.contains("Invalid JSON structure"));
    }

    #[test]
    fn test_incompatible_version_error() {
        let error = CitadelError::incompatible("1.0.0", "2.0.0");
        let error_msg = error.to_string();
        assert!(error_msg.contains("Incompatible state version"));
        assert!(error_msg.contains("expected 1.0.0"));
        assert!(error_msg.contains("found 2.0.0"));
    }

    #[test]
    fn test_io_error_conversion() {
        let io_err = io::Error::new(io::ErrorKind::NotFound, "file not found");
        let citadel_err: CitadelError = io_err.into();
        assert!(matches!(citadel_err, CitadelError::IoError(_)));
    }

    #[test]
    fn test_serialization_error_conversion() {
        let json = "{invalid json}";
        let serde_err = serde_json::from_str::<serde_json::Value>(json).unwrap_err();
        let citadel_err: CitadelError = serde_err.into();
        assert!(matches!(citadel_err, CitadelError::SerializationError(_)));
    }

    #[test]
    fn test_invalid_directory_error() {
        let error = CitadelError::invalid_directory("/invalid/path");
        assert!(error.to_string().contains("Invalid state directory"));
        assert!(error.to_string().contains("/invalid/path"));
    }

    #[test]
    fn test_permission_denied_error() {
        let error = CitadelError::permission_denied("Cannot write to directory");
        assert!(error.to_string().contains("Permission denied"));
    }

    #[test]
    fn test_directory_creation_failed_error() {
        let error = CitadelError::directory_creation_failed("Path already exists as file");
        assert!(
            error
                .to_string()
                .contains("Failed to create state directory")
        );
    }
}
