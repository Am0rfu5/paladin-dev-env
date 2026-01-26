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
                     \x1b[36mExample:\x1b[0m\n  paladin agent new -n my-agent -o config.yaml\n",
                    path.display()
                )
            }
            CliError::InvalidYaml { path, source } => {
                format!(
                    "\n\x1b[31mError:\x1b[0m Invalid YAML syntax\n\n\
                     \x1b[33mDetails:\x1b[0m The configuration file '{}' contains invalid YAML:\n  {}\n\n\
                     \x1b[32mSuggestion:\x1b[0m Check the YAML syntax. Common issues include:\n\
                     • Missing or extra colons\n\
                     • Incorrect indentation (use spaces, not tabs)\n\
                     • Unquoted special characters\n\
                     • Missing closing brackets or quotes\n\n\
                     \x1b[36mExample:\x1b[0m Correct YAML structure:\n\
                     name: my_agent\n\
                     provider:\n\
                       type: openai\n\
                       model: gpt-4\n",
                    path.display(),
                    source
                )
            }
            CliError::ValidationError { message } => {
                format!(
                    "\n\x1b[31mError:\x1b[0m Configuration validation failed\n\n\
                     \x1b[33mDetails:\x1b[0m {}\n\n\
                     \x1b[32mSuggestion:\x1b[0m Review the configuration file and ensure all fields meet the requirements.\n",
                    message
                )
            }
            CliError::MissingRequiredField { field, message } => {
                format!(
                    "\n\x1b[31mError:\x1b[0m Missing required field\n\n\
                     \x1b[33mDetails:\x1b[0m The configuration is missing the required field '{}'.\n  {}\n\n\
                     \x1b[32mSuggestion:\x1b[0m Add the field to your configuration file.\n\n\
                     \x1b[36mExample:\x1b[0m\n  {}: <value>\n",
                    field, message, field
                )
            }
            CliError::InvalidFieldValue { field, message } => {
                format!(
                    "\n\x1b[31mError:\x1b[0m Invalid field value\n\n\
                     \x1b[33mDetails:\x1b[0m The field '{}' has an invalid value:\n  {}\n\n\
                     \x1b[32mSuggestion:\x1b[0m Check the field value and ensure it meets the requirements.\n",
                    field, message
                )
            }
            CliError::MissingApiKey { provider, env_var } => {
                format!(
                    "\n\x1b[31mError:\x1b[0m Missing API key\n\n\
                     \x1b[33mDetails:\x1b[0m The environment variable '{}' is not set for provider '{}'.\n\n\
                     \x1b[32mSuggestion:\x1b[0m Set the environment variable:\n\n\
                     \x1b[36mExample:\x1b[0m\n  export {}=<your-api-key>\n  # Or add to your ~/.bashrc or ~/.zshrc:\n  echo 'export {}=<your-api-key>' >> ~/.bashrc\n",
                    env_var, provider, env_var, env_var
                )
            }
            CliError::FileAlreadyExists { path } => {
                format!(
                    "\n\x1b[31mError:\x1b[0m File already exists\n\n\
                     \x1b[33mDetails:\x1b[0m The file '{}' already exists.\n\n\
                     \x1b[32mSuggestion:\x1b[0m Choose a different output path or delete the existing file:\n\n\
                     \x1b[36mExample:\x1b[0m\n  rm {}\n  # Or use a different output file:\n  --output ./new-config.yaml\n",
                    path.display(),
                    path.display()
                )
            }
            CliError::IoError { message, .. } => {
                format!(
                    "\n\x1b[31mError:\x1b[0m IO operation failed\n\n\
                     \x1b[33mDetails:\x1b[0m {}\n\n\
                     \x1b[32mSuggestion:\x1b[0m Check file permissions and disk space. Ensure the path is accessible.\n",
                    message
                )
            }
            CliError::Cancelled => "\n\x1b[33mInfo:\x1b[0m Operation cancelled by user\n\n\
                 The operation was interrupted. No changes were made.\n"
                .to_string(),
            CliError::LlmError { message } => {
                format!(
                    "\n\x1b[31mError:\x1b[0m LLM execution failed\n\n\
                     \x1b[33mDetails:\x1b[0m {}\n\n\
                     \x1b[32mSuggestion:\x1b[0m Check your API key, network connection, and LLM service status.\n\
                     • Verify the API key is valid and has sufficient credits\n\
                     • Check if the model name is correct\n\
                     • Ensure you have a stable internet connection\n",
                    message
                )
            }
            CliError::LlmProviderError { message } => {
                format!(
                    "\n\x1b[31mError:\x1b[0m LLM provider configuration failed\n\n\
                     \x1b[33mDetails:\x1b[0m {}\n\n\
                     \x1b[32mSuggestion:\x1b[0m Check your provider configuration in the config file.\n\n\
                     \x1b[36mExample:\x1b[0m Valid providers:\n\
                     provider:\n\
                       type: openai    # or: deepseek, anthropic\n\
                       model: gpt-4    # or appropriate model for provider\n",
                    message
                )
            }
            CliError::ExecutionError { message } => {
                format!(
                    "\n\x1b[31mError:\x1b[0m Paladin execution failed\n\n\
                     \x1b[33mDetails:\x1b[0m {}\n\n\
                     \x1b[32mSuggestion:\x1b[0m Review the Paladin configuration and input.\n\
                     • Check if the system prompt is clear and actionable\n\
                     • Verify that max_loops is set appropriately\n\
                     • Ensure the input is valid for the task\n",
                    message
                )
            }
            CliError::BattalionError { message } => {
                format!(
                    "\n\x1b[31mError:\x1b[0m Battalion execution failed\n\n\
                     \x1b[33mDetails:\x1b[0m {}\n\n\
                     \x1b[32mSuggestion:\x1b[0m Review the Battalion configuration.\n\
                     • Check that all Paladin configurations are valid\n\
                     • Verify the Battalion type matches the configuration\n\
                     • Ensure all required Paladins are configured\n",
                    message
                )
            }
            CliError::ToolError { message } => {
                format!(
                    "\n\x1b[31mError:\x1b[0m Tool execution failed\n\n\
                     \x1b[33mDetails:\x1b[0m {}\n\n\
                     \x1b[32mSuggestion:\x1b[0m Check the tool configuration and MCP server.\n\
                     • Verify the MCP server is running and accessible\n\
                     • Check that the tool parameters are correct\n\
                     • Ensure the tool supports the requested operation\n",
                    message
                )
            }
            CliError::McpConnectionError { message } => {
                format!(
                    "\n\x1b[31mError:\x1b[0m MCP connection failed\n\n\
                     \x1b[33mDetails:\x1b[0m {}\n\n\
                     \x1b[32mSuggestion:\x1b[0m Troubleshoot the MCP server connection.\n\
                     • Verify the command exists and is in your PATH\n\
                     • Check that the server implements the MCP protocol\n\
                     • Test the command manually in a terminal\n\
                     • Review server logs for error messages\n\n\
                     \x1b[36mExample:\x1b[0m Test MCP server:\n  paladin arsenal test --mcp-stdio \"python3 server.py\"\n",
                    message
                )
            }
            CliError::SerializationError { message } => {
                format!(
                    "\n\x1b[31mError:\x1b[0m Serialization failed\n\n\
                     \x1b[33mDetails:\x1b[0m {}\n\n\
                     \x1b[32mSuggestion:\x1b[0m Check that the data format is valid.\n",
                    message
                )
            }
            CliError::Other(message) => {
                format!("\n\x1b[31mError:\x1b[0m {}\n", message)
            }
        }
    }

    /// Get the appropriate exit code for this error
    ///
    /// Exit codes follow FR-21:
    /// - 0: Success
    /// - 1: User errors (config, validation, missing args)
    /// - 2: Runtime errors (LLM, execution, tools)
    /// - 130: SIGINT (Ctrl+C)
    pub fn exit_code(&self) -> i32 {
        match self {
            // User errors: exit code 1
            CliError::ConfigFileNotFound { .. }
            | CliError::InvalidYaml { .. }
            | CliError::ValidationError { .. }
            | CliError::MissingRequiredField { .. }
            | CliError::InvalidFieldValue { .. }
            | CliError::MissingApiKey { .. }
            | CliError::FileAlreadyExists { .. } => 1,

            // Cancellation: exit code 130 (SIGINT)
            CliError::Cancelled => 130,

            // Runtime errors: exit code 2
            CliError::IoError { .. }
            | CliError::LlmError { .. }
            | CliError::LlmProviderError { .. }
            | CliError::ExecutionError { .. }
            | CliError::BattalionError { .. }
            | CliError::ToolError { .. }
            | CliError::McpConnectionError { .. }
            | CliError::SerializationError { .. }
            | CliError::Other(_) => 2,
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_config_file_not_found_formatting() {
        let error = CliError::ConfigFileNotFound {
            path: PathBuf::from("config.yaml"),
        };
        let formatted = error.format_detailed();

        assert!(formatted.contains("Configuration file not found"));
        assert!(formatted.contains("config.yaml"));
        assert!(formatted.contains("paladin agent new"));
        assert!(formatted.contains("Error:"));
        assert!(formatted.contains("Details:"));
        assert!(formatted.contains("Suggestion:"));
        assert!(formatted.contains("Example:"));
    }

    #[test]
    fn test_invalid_yaml_formatting() {
        let yaml_error = serde_yaml::from_str::<serde_yaml::Value>("bad: [yaml").unwrap_err();
        let error = CliError::InvalidYaml {
            path: PathBuf::from("config.yaml"),
            source: yaml_error,
        };
        let formatted = error.format_detailed();

        assert!(formatted.contains("Invalid YAML syntax"));
        assert!(formatted.contains("config.yaml"));
        assert!(formatted.contains("indentation"));
        assert!(formatted.contains("Example:"));
    }

    #[test]
    fn test_missing_api_key_formatting() {
        let error = CliError::MissingApiKey {
            provider: "OpenAI".to_string(),
            env_var: "OPENAI_API_KEY".to_string(),
        };
        let formatted = error.format_detailed();

        assert!(formatted.contains("Missing API key"));
        assert!(formatted.contains("OPENAI_API_KEY"));
        assert!(formatted.contains("export"));
        assert!(formatted.contains("bashrc"));
    }

    #[test]
    fn test_llm_error_formatting() {
        let error = CliError::LlmError {
            message: "API request failed".to_string(),
        };
        let formatted = error.format_detailed();

        assert!(formatted.contains("LLM execution failed"));
        assert!(formatted.contains("API request failed"));
        assert!(formatted.contains("API key"));
        assert!(formatted.contains("network connection"));
    }

    #[test]
    fn test_mcp_connection_error_formatting() {
        let error = CliError::McpConnectionError {
            message: "Server not responding".to_string(),
        };
        let formatted = error.format_detailed();

        assert!(formatted.contains("MCP connection failed"));
        assert!(formatted.contains("Server not responding"));
        assert!(formatted.contains("paladin arsenal test"));
        assert!(formatted.contains("PATH"));
    }

    #[test]
    fn test_battalion_error_formatting() {
        let error = CliError::BattalionError {
            message: "Paladin configuration invalid".to_string(),
        };
        let formatted = error.format_detailed();

        assert!(formatted.contains("Battalion execution failed"));
        assert!(formatted.contains("Paladin configuration invalid"));
        assert!(formatted.contains("Battalion configuration"));
    }

    #[test]
    fn test_cancelled_formatting() {
        let error = CliError::Cancelled;
        let formatted = error.format_detailed();

        assert!(formatted.contains("cancelled"));
        assert!(formatted.contains("No changes were made"));
    }

    #[test]
    fn test_exit_code_user_errors() {
        // User errors should return exit code 1
        assert_eq!(
            CliError::ConfigFileNotFound {
                path: PathBuf::from("test")
            }
            .exit_code(),
            1
        );
        assert_eq!(
            CliError::ValidationError {
                message: "test".into()
            }
            .exit_code(),
            1
        );
        assert_eq!(
            CliError::MissingApiKey {
                provider: "test".into(),
                env_var: "TEST".into()
            }
            .exit_code(),
            1
        );
    }

    #[test]
    fn test_exit_code_runtime_errors() {
        // Runtime errors should return exit code 2
        assert_eq!(
            CliError::LlmError {
                message: "test".into()
            }
            .exit_code(),
            2
        );
        assert_eq!(
            CliError::ExecutionError {
                message: "test".into()
            }
            .exit_code(),
            2
        );
        assert_eq!(
            CliError::BattalionError {
                message: "test".into()
            }
            .exit_code(),
            2
        );
        assert_eq!(
            CliError::McpConnectionError {
                message: "test".into()
            }
            .exit_code(),
            2
        );
    }

    #[test]
    fn test_exit_code_cancelled() {
        // Cancelled (SIGINT) should return exit code 130
        assert_eq!(CliError::Cancelled.exit_code(), 130);
    }

    #[test]
    fn test_file_already_exists_formatting() {
        let error = CliError::FileAlreadyExists {
            path: PathBuf::from("output.yaml"),
        };
        let formatted = error.format_detailed();

        assert!(formatted.contains("File already exists"));
        assert!(formatted.contains("output.yaml"));
        assert!(formatted.contains("--output"));
    }

    #[test]
    fn test_llm_provider_error_formatting() {
        let error = CliError::LlmProviderError {
            message: "Unknown provider".to_string(),
        };
        let formatted = error.format_detailed();

        assert!(formatted.contains("LLM provider configuration failed"));
        assert!(formatted.contains("Unknown provider"));
        assert!(formatted.contains("openai"));
        assert!(formatted.contains("deepseek"));
        assert!(formatted.contains("anthropic"));
    }

    #[test]
    fn test_validation_error_formatting() {
        let error = CliError::ValidationError {
            message: "Temperature must be between 0 and 1".to_string(),
        };
        let formatted = error.format_detailed();

        assert!(formatted.contains("Configuration validation failed"));
        assert!(formatted.contains("Temperature must be between 0 and 1"));
    }

    #[test]
    fn test_missing_required_field_formatting() {
        let error = CliError::MissingRequiredField {
            field: "system_prompt".to_string(),
            message: "System prompt is required for Paladin".to_string(),
        };
        let formatted = error.format_detailed();

        assert!(formatted.contains("Missing required field"));
        assert!(formatted.contains("system_prompt"));
        assert!(formatted.contains("System prompt is required"));
    }

    #[test]
    fn test_tool_error_formatting() {
        let error = CliError::ToolError {
            message: "Tool invocation failed".to_string(),
        };
        let formatted = error.format_detailed();

        assert!(formatted.contains("Tool execution failed"));
        assert!(formatted.contains("Tool invocation failed"));
        assert!(formatted.contains("MCP server"));
    }
}
