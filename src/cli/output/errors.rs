//! CLI-specific error types

use std::path::PathBuf;
use thiserror::Error;

/// CLI operation errors with actionable error messages
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

    /// Generic error
    #[error("{0}")]
    Other(String),
}

impl CliError {
    /// Format error with actionable guidance per FR-20
    pub fn format_detailed(&self) -> String {
        match self {
            CliError::ConfigFileNotFound { path } => {
                format!(
                    "\n\x1b[31mError:\x1b[0m Configuration file not found\n\n\
                     \x1b[33mDetails:\x1b[0m The configuration file '{}' does not exist.\n\n\
                     \x1b[32mSuggestion:\x1b[0m Check the file path and ensure the file exists. \
                     You can create a new configuration with:\n\n\
                     \x1b[36mpaladin agent new -n my-agent -o config.yaml\x1b[0m\n",
                    path.display()
                )
            }
            CliError::InvalidYaml { path, source } => {
                format!(
                    "\n\x1b[31mError:\x1b[0m Invalid YAML syntax\n\n\
                     \x1b[33mDetails:\x1b[0m The configuration file '{}' contains invalid YAML:\n{}\n\n\
                     \x1b[32mSuggestion:\x1b[0m Check the YAML syntax. Common issues include:\n\
                     - Missing or extra colons\n\
                     - Incorrect indentation (use spaces, not tabs)\n\
                     - Unquoted special characters\n",
                    path.display(),
                    source
                )
            }
            CliError::MissingRequiredField { field, message } => {
                format!(
                    "\n\x1b[31mError:\x1b[0m Missing required field\n\n\
                     \x1b[33mDetails:\x1b[0m The configuration is missing the required field '{}'.\n{}\n\n\
                     \x1b[32mSuggestion:\x1b[0m Add the field to your configuration file.\n\n\
                     \x1b[36mExample:\x1b[0m\n{}: <value>\n",
                    field, message, field
                )
            }
            CliError::InvalidFieldValue { field, message } => {
                format!(
                    "\n\x1b[31mError:\x1b[0m Invalid field value\n\n\
                     \x1b[33mDetails:\x1b[0m The field '{}' has an invalid value: {}\n\n\
                     \x1b[32mSuggestion:\x1b[0m Check the field value and ensure it meets the requirements.\n",
                    field, message
                )
            }
            CliError::MissingApiKey { provider, env_var } => {
                format!(
                    "\n\x1b[31mError:\x1b[0m Missing API key\n\n\
                     \x1b[33mDetails:\x1b[0m The environment variable '{}' is not set for provider '{}'.\n\n\
                     \x1b[32mSuggestion:\x1b[0m Set the environment variable:\n\n\
                     \x1b[36mexport {}=<your-api-key>\x1b[0m\n",
                    env_var, provider, env_var
                )
            }
            CliError::FileAlreadyExists { path } => {
                format!(
                    "\n\x1b[31mError:\x1b[0m File already exists\n\n\
                     \x1b[33mDetails:\x1b[0m The file '{}' already exists.\n\n\
                     \x1b[32mSuggestion:\x1b[0m Choose a different output path or delete the existing file.\n",
                    path.display()
                )
            }
            _ => format!("\n\x1b[31mError:\x1b[0m {}\n", self),
        }
    }
}

impl From<std::io::Error> for CliError {
    fn from(error: std::io::Error) -> Self {
        CliError::IoError {
            message: error.to_string(),
            source: error,
        }
    }
}

impl From<serde_yaml::Error> for CliError {
    fn from(error: serde_yaml::Error) -> Self {
        CliError::Other(format!("YAML error: {}", error))
    }
}

impl From<crate::application::use_cases::paladin::error::PaladinError> for CliError {
    fn from(error: crate::application::use_cases::paladin::error::PaladinError) -> Self {
        CliError::ExecutionError {
            message: error.to_string(),
        }
    }
}
