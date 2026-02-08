//! Unified CLI error types with user-friendly formatting
//!
//! This module defines all error types used by the Paladin CLI,
//! with Display implementations that provide helpful error messages.
//!
//! # Error Categories
//!
//! - **Validation errors**: Invalid config, missing fields
//! - **Execution errors**: LLM failures, timeout, stop words
//! - **I/O errors**: File not found, permission denied
//! - **User interaction**: Cancelled operations

use std::path::PathBuf;
use thiserror::Error;

/// Result type for CLI operations
pub type CliResult<T> = Result<T, CliError>;

/// Unified CLI operation errors with actionable error messages
#[derive(Debug, Error)]
pub enum CliError {
    /// Configuration file not found
    #[error("Configuration file not found: {path}")]
    ConfigFileNotFound { path: PathBuf },

    /// Invalid YAML syntax
    #[error("Invalid YAML syntax in configuration file")]
    InvalidYaml {
        path: PathBuf,
        source: serde_yaml::Error,
    },

    /// Configuration validation failed
    #[error("Configuration validation failed: {message}")]
    ValidationError { message: String },

    /// Missing required field
    #[error("Missing required field: {field}. {message}")]
    MissingRequiredField { field: String, message: String },

    /// Invalid field value
    #[error("Invalid value for field '{field}': {message}")]
    InvalidFieldValue { field: String, message: String },

    /// Missing API key
    #[error("Missing API key for {provider}. Please set {env_var}")]
    MissingApiKey { provider: String, env_var: String },

    /// File already exists
    #[error("File already exists: {path}")]
    FileAlreadyExists { path: PathBuf },

    /// IO error
    #[error("IO error: {message}")]
    IoError {
        message: String,
        #[source]
        source: std::io::Error,
    },

    /// User cancelled operation
    #[error("Operation cancelled by user")]
    Cancelled,

    /// LLM execution error
    #[error("LLM execution error: {message}")]
    LlmError { message: String },

    /// LLM provider creation error
    #[error("LLM provider error: {message}")]
    LlmProviderError { message: String },

    /// Paladin execution error
    #[error("Paladin execution error: {message}")]
    ExecutionError { message: String },

    /// Battalion execution error
    #[error("Battalion execution error: {message}")]
    BattalionError { message: String },

    /// Tool execution error
    #[error("Tool execution error: {message}")]
    ToolError { message: String },

    /// MCP connection error
    #[error("MCP connection error: {message}")]
    McpConnectionError { message: String },

    /// Serialization error
    #[error("Serialization error: {message}")]
    SerializationError { message: String },

    /// Invalid file path
    #[error("Invalid file path '{path}': {message}")]
    InvalidFilePath { path: String, message: String },

    /// Unsupported file format
    #[error("Unsupported format '{format}'. Supported formats: {supported}")]
    UnsupportedFormat { format: String, supported: String },

    /// File read error
    #[error("Failed to read file '{path}': {message}")]
    FileReadError { path: String, message: String },

    /// Vision processing error
    #[error("Vision processing error: {message}")]
    VisionProcessingError { message: String },

    /// Document processing error
    #[error("Document processing error: {message}")]
    DocumentProcessingError { message: String },

    /// Network error
    #[error("Network error: {0}")]
    NetworkError(String),

    /// User input error
    #[error("User input error: {0}")]
    UserInputError(String),

    /// JSON parsing error
    #[error("JSON parsing error: {0}")]
    JsonError(String),

    /// Generic error
    #[error("{0}")]
    Other(String),
}
