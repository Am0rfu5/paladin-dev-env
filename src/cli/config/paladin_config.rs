//! Paladin YAML configuration types

use crate::cli::output::errors::CliError;
use serde::{Deserialize, Serialize};

/// Paladin YAML configuration matching FR-15 schema
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaladinYamlConfig {
    /// Name of the Paladin
    pub name: String,

    /// System prompt defining Paladin behavior
    pub system_prompt: String,

    /// Model to use (e.g., "gpt-4", "deepseek-chat")
    pub model: String,

    /// Temperature for response generation (0.0-2.0)
    #[serde(default = "default_temperature")]
    pub temperature: f32,

    /// Maximum reasoning loops
    #[serde(default = "default_max_loops")]
    pub max_loops: u32,

    /// Timeout in seconds
    #[serde(default = "default_timeout")]
    pub timeout_seconds: u64,

    /// Stop words that trigger early termination
    #[serde(default)]
    pub stop_words: Vec<String>,

    /// LLM provider configuration
    pub provider: ProviderConfig,

    /// Optional garrison (memory) configuration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub garrison: Option<GarrisonConfig>,

    /// Optional arsenal (tools) configuration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arsenal: Option<ArsenalConfig>,
}

/// LLM provider configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderConfig {
    /// Provider type: openai, deepseek, anthropic
    #[serde(rename = "type")]
    pub provider_type: String,
}

/// Garrison (memory) configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GarrisonConfig {
    /// Garrison type: in_memory or sqlite
    #[serde(rename = "type")]
    pub garrison_type: String,

    /// Optional type-specific configuration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<GarrisonTypeConfig>,
}

/// Type-specific garrison configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GarrisonTypeConfig {
    /// Maximum entries for in_memory garrison
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_entries: Option<usize>,

    /// Database path for sqlite garrison
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
}

/// Arsenal (tools) configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArsenalConfig {
    /// List of MCP servers to connect
    #[serde(default)]
    pub mcp_servers: Vec<McpServerConfig>,
}

/// MCP server configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpServerConfig {
    /// Server name
    pub name: String,

    /// Server type: stdio or sse
    #[serde(rename = "type")]
    pub server_type: String,

    /// Command for stdio type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command: Option<String>,

    /// Arguments for stdio type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub args: Option<Vec<String>>,

    /// Endpoint URL for sse type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<String>,
}

// Default values
fn default_temperature() -> f32 {
    0.7
}

fn default_max_loops() -> u32 {
    3
}

fn default_timeout() -> u64 {
    300
}

/// Validation trait for configuration
pub trait Validate {
    fn validate(&self) -> Result<(), CliError>;
}

impl Validate for PaladinYamlConfig {
    fn validate(&self) -> Result<(), CliError> {
        // Validate required fields
        if self.name.is_empty() {
            return Err(CliError::MissingRequiredField {
                field: "name".to_string(),
                message: "Paladin name is required".to_string(),
            });
        }

        if self.system_prompt.is_empty() {
            return Err(CliError::MissingRequiredField {
                field: "system_prompt".to_string(),
                message: "System prompt is required to define Paladin behavior".to_string(),
            });
        }

        if self.model.is_empty() {
            return Err(CliError::MissingRequiredField {
                field: "model".to_string(),
                message: "LLM model name is required".to_string(),
            });
        }

        // Validate temperature range
        if !(0.0..=2.0).contains(&self.temperature) {
            return Err(CliError::InvalidFieldValue {
                field: "temperature".to_string(),
                message: format!("must be between 0.0 and 2.0, got {}", self.temperature),
            });
        }

        // Validate max_loops
        if self.max_loops == 0 {
            return Err(CliError::InvalidFieldValue {
                field: "max_loops".to_string(),
                message: "must be greater than 0".to_string(),
            });
        }

        // Validate provider type
        let valid_providers = ["openai", "deepseek", "anthropic"];
        if !valid_providers.contains(&self.provider.provider_type.as_str()) {
            return Err(CliError::InvalidFieldValue {
                field: "provider.type".to_string(),
                message: format!("must be one of: {}", valid_providers.join(", ")),
            });
        }

        // Validate garrison type if present
        if let Some(garrison) = &self.garrison {
            let valid_garrison_types = ["in_memory", "sqlite"];
            if !valid_garrison_types.contains(&garrison.garrison_type.as_str()) {
                return Err(CliError::InvalidFieldValue {
                    field: "garrison.type".to_string(),
                    message: format!("must be one of: {}", valid_garrison_types.join(", ")),
                });
            }
        }

        // Validate MCP server configurations if present
        if let Some(arsenal) = &self.arsenal {
            for server in &arsenal.mcp_servers {
                let valid_server_types = ["stdio", "sse"];
                if !valid_server_types.contains(&server.server_type.as_str()) {
                    return Err(CliError::InvalidFieldValue {
                        field: format!("arsenal.mcp_servers.{}.type", server.name),
                        message: format!("must be one of: {}", valid_server_types.join(", ")),
                    });
                }

                // Validate stdio type has command
                if server.server_type == "stdio" && server.command.is_none() {
                    return Err(CliError::MissingRequiredField {
                        field: format!("arsenal.mcp_servers.{}.command", server.name),
                        message: "stdio server requires command field".to_string(),
                    });
                }

                // Validate sse type has endpoint
                if server.server_type == "sse" && server.endpoint.is_none() {
                    return Err(CliError::MissingRequiredField {
                        field: format!("arsenal.mcp_servers.{}.endpoint", server.name),
                        message: "sse server requires endpoint field".to_string(),
                    });
                }
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_config() {
        let config = PaladinYamlConfig {
            name: "test".to_string(),
            system_prompt: "You are a helpful assistant".to_string(),
            model: "gpt-4".to_string(),
            temperature: 0.7,
            max_loops: 3,
            timeout_seconds: 300,
            stop_words: vec![],
            provider: ProviderConfig {
                provider_type: "openai".to_string(),
            },
            garrison: None,
            arsenal: None,
        };

        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_missing_name() {
        let config = PaladinYamlConfig {
            name: "".to_string(),
            system_prompt: "You are a helpful assistant".to_string(),
            model: "gpt-4".to_string(),
            temperature: 0.7,
            max_loops: 3,
            timeout_seconds: 300,
            stop_words: vec![],
            provider: ProviderConfig {
                provider_type: "openai".to_string(),
            },
            garrison: None,
            arsenal: None,
        };

        assert!(matches!(
            config.validate(),
            Err(CliError::MissingRequiredField { field, .. }) if field == "name"
        ));
    }

    #[test]
    fn test_invalid_temperature() {
        let config = PaladinYamlConfig {
            name: "test".to_string(),
            system_prompt: "You are a helpful assistant".to_string(),
            model: "gpt-4".to_string(),
            temperature: 3.0, // Invalid: > 2.0
            max_loops: 3,
            timeout_seconds: 300,
            stop_words: vec![],
            provider: ProviderConfig {
                provider_type: "openai".to_string(),
            },
            garrison: None,
            arsenal: None,
        };

        assert!(matches!(
            config.validate(),
            Err(CliError::InvalidFieldValue { field, .. }) if field == "temperature"
        ));
    }

    #[test]
    fn test_invalid_provider() {
        let config = PaladinYamlConfig {
            name: "test".to_string(),
            system_prompt: "You are a helpful assistant".to_string(),
            model: "gpt-4".to_string(),
            temperature: 0.7,
            max_loops: 3,
            timeout_seconds: 300,
            stop_words: vec![],
            provider: ProviderConfig {
                provider_type: "invalid".to_string(),
            },
            garrison: None,
            arsenal: None,
        };

        assert!(matches!(
            config.validate(),
            Err(CliError::InvalidFieldValue { field, .. }) if field == "provider.type"
        ));
    }
}
