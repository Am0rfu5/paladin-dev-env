//! CLI-specific error types

use thiserror::Error;

/// Result type for CLI operations
pub type CliResult<T> = Result<T, CliError>;

/// Errors that can occur during CLI operations
#[derive(Debug, Error)]
pub enum CliError {
    /// Configuration error
    #[error("Configuration error: {0}")]
    ConfigurationError(String),

    /// Network error
    #[error("Network error: {0}")]
    NetworkError(String),

    /// Validation error
    #[error("Validation error: {0}")]
    ValidationError(String),

    /// User input error
    #[error("User input error: {0}")]
    UserInputError(String),

    /// File I/O error
    #[error("File I/O error: {0}")]
    IoError(#[from] std::io::Error),

    /// YAML parsing error
    #[error("YAML parsing error: {0}")]
    YamlError(#[from] serde_yaml::Error),

    /// JSON parsing error
    #[error("JSON parsing error: {0}")]
    JsonError(#[from] serde_json::Error),

    /// LLM error
    #[error("LLM error: {0}")]
    LlmError(String),

    /// Command execution error
    #[error("Command execution error: {0}")]
    ExecutionError(String),

    /// User cancelled operation
    #[error("Operation cancelled by user")]
    Cancelled,

    /// Missing required field
    #[error("Missing required field: {0}")]
    MissingField(String),

    /// Invalid argument
    #[error("Invalid argument: {0}")]
    InvalidArgument(String),
}

impl CliError {
    /// Create a configuration error
    pub fn configuration(msg: impl Into<String>) -> Self {
        Self::ConfigurationError(msg.into())
    }

    /// Create a network error
    pub fn network(msg: impl Into<String>) -> Self {
        Self::NetworkError(msg.into())
    }

    /// Create a validation error
    pub fn validation(msg: impl Into<String>) -> Self {
        Self::ValidationError(msg.into())
    }

    /// Create a user input error
    pub fn user_input(msg: impl Into<String>) -> Self {
        Self::UserInputError(msg.into())
    }

    /// Create an LLM error
    pub fn llm(msg: impl Into<String>) -> Self {
        Self::LlmError(msg.into())
    }

    /// Create an execution error
    pub fn execution(msg: impl Into<String>) -> Self {
        Self::ExecutionError(msg.into())
    }

    /// Create a missing field error
    pub fn missing_field(field: impl Into<String>) -> Self {
        Self::MissingField(field.into())
    }

    /// Create an invalid argument error
    pub fn invalid_argument(msg: impl Into<String>) -> Self {
        Self::InvalidArgument(msg.into())
    }
}
