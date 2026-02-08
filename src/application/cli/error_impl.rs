//! CliError implementation - conversions, formatting, and helper methods

use super::error::CliError;

impl CliError {
    /// Create a configuration error
    pub fn configuration(msg: impl Into<String>) -> Self {
        Self::ValidationError {
            message: msg.into(),
        }
    }

    /// Create a network error
    pub fn network(msg: impl Into<String>) -> Self {
        Self::NetworkError(msg.into())
    }

    /// Create a validation error
    pub fn validation(msg: impl Into<String>) -> Self {
        Self::ValidationError {
            message: msg.into(),
        }
    }

    /// Create a user input error
    pub fn user_input(msg: impl Into<String>) -> Self {
        Self::UserInputError(msg.into())
    }

    /// Create an LLM error
    pub fn llm(msg: impl Into<String>) -> Self {
        Self::LlmError {
            message: msg.into(),
        }
    }

    /// Create an execution error
    pub fn execution(msg: impl Into<String>) -> Self {
        Self::ExecutionError {
            message: msg.into(),
        }
    }

    /// Create a missing field error
    pub fn missing_field(field: impl Into<String>) -> Self {
        Self::MissingRequiredField {
            field: field.into(),
            message: String::new(),
        }
    }

    /// Create an invalid argument error
    pub fn invalid_argument(msg: impl Into<String>) -> Self {
        Self::InvalidFieldValue {
            field: "argument".into(),
            message: msg.into(),
        }
    }

    /// Format error with actionable guidance
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
            CliError::InvalidFilePath { path, message } => {
                format!(
                    "\n\x1b[31mError:\x1b[0m Invalid file path\n\n\
                     \x1b[33mDetails:\x1b[0m The file '{}' is invalid:\n  {}\n\n\
                     \x1b[32mSuggestion:\x1b[0m Check that the file exists and the path is correct.\n\n\
                     \x1b[36mExample:\x1b[0m\n  ls -la {}\n",
                    path, message, path
                )
            }
            CliError::UnsupportedFormat { format, supported } => {
                format!(
                    "\n\x1b[31mError:\x1b[0m Unsupported file format\n\n\
                     \x1b[33mDetails:\x1b[0m The format '{}' is not supported.\n\n\
                     \x1b[32mSupported formats:\x1b[0m {}\n\n\
                     \x1b[32mSuggestion:\x1b[0m Convert the file to a supported format or use a different file.\n",
                    format, supported
                )
            }
            CliError::FileReadError { path, message } => {
                format!(
                    "\n\x1b[31mError:\x1b[0m Failed to read file\n\n\
                     \x1b[33mDetails:\x1b[0m Could not read '{}':\n  {}\n\n\
                     \x1b[32mSuggestion:\x1b[0m Check file permissions and ensure the file is not corrupted.\n\n\
                     \x1b[36mExample:\x1b[0m\n  chmod 644 {}\n  file {}\n",
                    path, message, path, path
                )
            }
            CliError::VisionProcessingError { message } => {
                format!(
                    "\n\x1b[31mError:\x1b[0m Vision processing failed\n\n\
                     \x1b[33mDetails:\x1b[0m {}\n\n\
                     \x1b[32mSuggestion:\x1b[0m Check that:\n\
                     • The image file is valid and not corrupted\n\
                     • The image format is supported (png, jpg, jpeg, gif, webp)\n\
                     • The LLM model supports vision capabilities\n\
                     • Vision mode is enabled in the configuration\n",
                    message
                )
            }
            CliError::DocumentProcessingError { message } => {
                format!(
                    "\n\x1b[31mError:\x1b[0m Document processing failed\n\n\
                     \x1b[33mDetails:\x1b[0m {}\n\n\
                     \x1b[32mSuggestion:\x1b[0m Check that:\n\
                     • The document file is valid and not corrupted\n\
                     • The document format is supported (pdf, txt, md)\n\
                     • The file is not encrypted or password-protected\n\
                     • The file has appropriate read permissions\n",
                    message
                )
            }
            CliError::NetworkError(message) => {
                format!(
                    "\n\x1b[31mError:\x1b[0m Network error\n\n\
                     \x1b[33mDetails:\x1b[0m {}\n\n\
                     \x1b[32mSuggestion:\x1b[0m Check your internet connection and try again.\n",
                    message
                )
            }
            CliError::UserInputError(message) => {
                format!(
                    "\n\x1b[31mError:\x1b[0m Invalid user input\n\n\
                     \x1b[33mDetails:\x1b[0m {}\n\n\
                     \x1b[32mSuggestion:\x1b[0m Check the input format and try again.\n",
                    message
                )
            }
            CliError::JsonError(message) => {
                format!(
                    "\n\x1b[31mError:\x1b[0m JSON parsing error\n\n\
                     \x1b[33mDetails:\x1b[0m {}\n\n\
                     \x1b[32mSuggestion:\x1b[0m Check the JSON format and ensure it's valid.\n",
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
    /// Exit codes:
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
            | CliError::FileAlreadyExists { .. }
            | CliError::InvalidFilePath { .. }
            | CliError::UnsupportedFormat { .. }
            | CliError::UserInputError(_) => 1,

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
            | CliError::FileReadError { .. }
            | CliError::VisionProcessingError { .. }
            | CliError::DocumentProcessingError { .. }
            | CliError::NetworkError(_)
            | CliError::JsonError(_)
            | CliError::Other(_) => 2,
        }
    }
}

// Note: Old CLI error conversion removed as src/cli/ has been deleted
// All code now uses unified CliError from application::cli::error

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

impl From<serde_json::Error> for CliError {
    fn from(error: serde_json::Error) -> Self {
        CliError::JsonError(error.to_string())
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
    fn test_helper_methods() {
        let _ = CliError::configuration("test");
        let _ = CliError::network("test");
        let _ = CliError::validation("test");
        let _ = CliError::user_input("test");
        let _ = CliError::llm("test");
        let _ = CliError::execution("test");
        let _ = CliError::missing_field("test");
        let _ = CliError::invalid_argument("test");
    }

    #[test]
    fn test_exit_code_user_errors() {
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
    }

    #[test]
    fn test_exit_code_runtime_errors() {
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
    }

    #[test]
    fn test_exit_code_cancelled() {
        assert_eq!(CliError::Cancelled.exit_code(), 130);
    }

    #[test]
    fn test_io_error_conversion() {
        let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "file not found");
        let cli_err: CliError = io_err.into();
        assert!(matches!(cli_err, CliError::IoError { .. }));
    }

    #[test]
    fn test_yaml_error_conversion() {
        let yaml_err = serde_yaml::from_str::<serde_yaml::Value>("bad: [yaml").unwrap_err();
        let cli_err: CliError = yaml_err.into();
        assert!(matches!(cli_err, CliError::Other(_)));
    }

    #[test]
    fn test_json_error_conversion() {
        let json_err = serde_json::from_str::<serde_json::Value>("{bad json").unwrap_err();
        let cli_err: CliError = json_err.into();
        assert!(matches!(cli_err, CliError::JsonError(_)));
    }
}
