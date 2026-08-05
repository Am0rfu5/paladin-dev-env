//! Paladin YAML configuration types for CLI agent management
//!
//! This module defines the YAML configuration schema for Paladin agents,
//! including LLM provider settings, garrison (memory), and arsenal (tools).
//!
//! # Configuration Schema
//!
//! ```yaml
//! name: "my-assistant"
//! system_prompt: "You are a helpful assistant"
//! model: "gpt-4"
//! temperature: 0.7
//! max_loops: 3
//! timeout_seconds: 300
//!
//! provider:
//!   type: "openai"
//!   api_key: "${OPENAI_API_KEY}"
//!
//! # Optional memory
//! garrison:
//!   type: "sqlite"
//!   path: "./memory.db"
//!
//! # Optional tools
//! arsenal:
//!   mcp_servers:
//!     - name: "web_search"
//!       server_type: "stdio"
//!       command: "uvx"
//!       args: ["mcp-web-search"]
//!
//! # Optional autonomous features (planning, prompt generation, dynamic
//! # temperature, handoffs) -- reuses paladin-core's AutonomousConfig
//! # directly (D-06). The four --auto-plan / --auto-prompt / --dynamic-temp
//! # / --enable-handoffs CLI flags layer on top of this section additively.
//! autonomous:
//!   planning:
//!     enabled: true
//!     max_subtasks: 10
//!   prompt_generation:
//!     enabled: false
//!   dynamic_temperature:
//!     enabled: false
//!     min: 0.1
//!     max: 0.9
//!   handoffs:
//!     enabled: false
//!     strategy: Automatic
//!     max_depth: 5
//!     retry:
//!       max_retries: 3
//!       initial_backoff_ms: 1000
//!       backoff_multiplier: 2.0
//! ```

use crate::application::cli::error::CliError;
use crate::core::platform::container::autonomous_config::AutonomousConfig;
use crate::core::platform::container::paladin::MaxLoops;
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

    /// Maximum reasoning loops (or planning subtasks in Auto mode)
    #[serde(default = "default_max_loops")]
    pub max_loops: MaxLoops,

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

    /// Optional autonomous features configuration (planning, prompt
    /// generation, dynamic temperature, handoffs).
    ///
    /// Reuses `paladin-core`'s `AutonomousConfig` directly rather than a
    /// CLI-local mirror type like `GarrisonConfig`/`ArsenalConfig` above —
    /// the mapping cost across its four sub-configs was judged not worth the
    /// decoupling (D-06, `06-CONTEXT.md`). The four `--auto-plan` /
    /// `--auto-prompt` / `--dynamic-temp` / `--enable-handoffs` CLI flags
    /// layer on top of this baseline additively; see `handle_agent_run`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub autonomous: Option<AutonomousConfig>,

    /// Enable vision capabilities
    #[serde(default)]
    pub vision_enabled: bool,

    /// Paths to image files for vision analysis
    #[serde(default)]
    pub images: Vec<String>,

    /// Paths to document files for processing (PDF, TXT, MD)
    #[serde(default)]
    pub documents: Vec<String>,
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

    /// Server type: "stdio" | "streamable_http" (replaces "sse"; "sse" is
    /// deprecated and fails loud with a migration message — Phase 12.1 D-02b).
    #[serde(rename = "type")]
    pub server_type: String,

    /// Command for stdio type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command: Option<String>,

    /// Arguments for stdio type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub args: Option<Vec<String>>,

    /// Endpoint URL for streamable_http type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<String>,

    /// Name of the environment variable holding the bearer/auth token for a
    /// `streamable_http` server (D-03). An env-var REFERENCE (e.g.
    /// `"ETHERSCAN_API_KEY"`), NEVER a literal secret — resolved host-side
    /// from the named env var at connect time, never serialized back out.
    #[serde(default, skip_serializing)]
    pub auth_token_env: Option<String>,
}

// Default values
fn default_temperature() -> f32 {
    0.7
}

fn default_max_loops() -> MaxLoops {
    MaxLoops::Fixed(3)
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
        if self.max_loops.as_u32() == 0 {
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
                // "sse" remains a valid *shape* here so the loader's own
                // deprecation error (loader.rs: "the mislabeled plain-HTTP
                // adapter has been retired. Use 'streamable_http' instead.")
                // is what the caller sees, rather than a generic
                // schema-validation rejection at this earlier stage.
                // "streamable_http" is the real, currently-implemented remote
                // transport (D-02/D-03) and MUST be accepted here — this
                // allowlist previously excluded it, which meant every
                // `type: "streamable_http"` config was rejected by
                // `validate()` before ever reaching the loader (Phase 12.1
                // Plan 05 regression fix).
                let valid_server_types = ["stdio", "sse", "streamable_http"];
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

                // Validate sse/streamable_http types have endpoint
                if (server.server_type == "sse" || server.server_type == "streamable_http")
                    && server.endpoint.is_none()
                {
                    return Err(CliError::MissingRequiredField {
                        field: format!("arsenal.mcp_servers.{}.endpoint", server.name),
                        message: format!("{} server requires endpoint field", server.server_type),
                    });
                }
            }
        }

        // Validate vision configuration
        if self.vision_enabled {
            // If vision_enabled is true, at least one image or document should be provided
            if self.images.is_empty() && self.documents.is_empty() {
                return Err(CliError::InvalidFieldValue {
                    field: "vision_enabled".to_string(),
                    message: "vision_enabled is true but no images or documents provided"
                        .to_string(),
                });
            }
        }

        // Validate image paths exist and have valid formats
        for image_path in &self.images {
            let path = std::path::Path::new(image_path);
            if !path.exists() {
                return Err(CliError::InvalidFilePath {
                    path: image_path.clone(),
                    message: format!("Image file not found: {}", image_path),
                });
            }

            // Check supported image formats
            if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                let ext_lower = ext.to_lowercase();
                if !["png", "jpg", "jpeg", "gif", "webp"].contains(&ext_lower.as_str()) {
                    return Err(CliError::UnsupportedFormat {
                        format: ext.to_string(),
                        supported: "png, jpg, jpeg, gif, webp".to_string(),
                    });
                }
            } else {
                return Err(CliError::UnsupportedFormat {
                    format: "unknown".to_string(),
                    supported: "png, jpg, jpeg, gif, webp".to_string(),
                });
            }
        }

        // Validate document paths exist and have valid formats
        for doc_path in &self.documents {
            let path = std::path::Path::new(doc_path);
            if !path.exists() {
                return Err(CliError::InvalidFilePath {
                    path: doc_path.clone(),
                    message: format!("Document file not found: {}", doc_path),
                });
            }

            // Check supported document formats
            if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                let ext_lower = ext.to_lowercase();
                if !["pdf", "txt", "md", "markdown"].contains(&ext_lower.as_str()) {
                    return Err(CliError::UnsupportedFormat {
                        format: ext.to_string(),
                        supported: "pdf, txt, md, markdown".to_string(),
                    });
                }
            } else {
                return Err(CliError::UnsupportedFormat {
                    format: "unknown".to_string(),
                    supported: "pdf, txt, md, markdown".to_string(),
                });
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
            max_loops: MaxLoops::Fixed(3),
            timeout_seconds: 300,
            stop_words: vec![],
            provider: ProviderConfig {
                provider_type: "openai".to_string(),
            },
            garrison: None,
            arsenal: None,
            autonomous: None,
            vision_enabled: false,
            images: vec![],
            documents: vec![],
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
            max_loops: MaxLoops::Fixed(3),
            timeout_seconds: 300,
            stop_words: vec![],
            provider: ProviderConfig {
                provider_type: "openai".to_string(),
            },
            garrison: None,
            arsenal: None,
            autonomous: None,
            vision_enabled: false,
            images: vec![],
            documents: vec![],
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
            max_loops: MaxLoops::Fixed(3),
            timeout_seconds: 300,
            stop_words: vec![],
            provider: ProviderConfig {
                provider_type: "openai".to_string(),
            },
            garrison: None,
            arsenal: None,
            autonomous: None,
            vision_enabled: false,
            images: vec![],
            documents: vec![],
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
            max_loops: MaxLoops::Fixed(3),
            timeout_seconds: 300,
            stop_words: vec![],
            provider: ProviderConfig {
                provider_type: "invalid".to_string(),
            },
            garrison: None,
            arsenal: None,
            autonomous: None,
            vision_enabled: false,
            images: vec![],
            documents: vec![],
        };

        assert!(matches!(
            config.validate(),
            Err(CliError::InvalidFieldValue { field, .. }) if field == "provider.type"
        ));
    }

    #[test]
    fn test_vision_enabled_parsing() {
        let config = PaladinYamlConfig {
            name: "test".to_string(),
            system_prompt: "You are a helpful assistant".to_string(),
            model: "gpt-4".to_string(),
            temperature: 0.7,
            max_loops: MaxLoops::Fixed(3),
            timeout_seconds: 300,
            stop_words: vec![],
            provider: ProviderConfig {
                provider_type: "openai".to_string(),
            },
            garrison: None,
            arsenal: None,
            autonomous: None,
            vision_enabled: true,
            images: vec![],
            documents: vec![],
        };

        // Should fail because vision_enabled is true but no images/documents
        assert!(matches!(
            config.validate(),
            Err(CliError::InvalidFieldValue { field, .. }) if field == "vision_enabled"
        ));
    }

    #[test]
    fn test_images_field_parsing() {
        use std::io::Write;
        use tempfile::NamedTempFile;

        // Create a temporary image file
        let mut temp_file = NamedTempFile::new().unwrap();
        temp_file.write_all(b"fake image data").unwrap();
        let temp_path = temp_file.path().to_str().unwrap().to_string();

        // Rename to have .png extension
        let png_path = format!("{}.png", temp_path);
        std::fs::copy(&temp_path, &png_path).unwrap();

        let config = PaladinYamlConfig {
            name: "test".to_string(),
            system_prompt: "You are a helpful assistant".to_string(),
            model: "gpt-4".to_string(),
            temperature: 0.7,
            max_loops: MaxLoops::Fixed(3),
            timeout_seconds: 300,
            stop_words: vec![],
            provider: ProviderConfig {
                provider_type: "openai".to_string(),
            },
            garrison: None,
            arsenal: None,
            autonomous: None,
            vision_enabled: true,
            images: vec![png_path.clone()],
            documents: vec![],
        };

        assert!(config.validate().is_ok());

        // Cleanup
        std::fs::remove_file(&png_path).ok();
    }

    #[test]
    fn test_documents_field_parsing() {
        use std::io::Write;
        use tempfile::NamedTempFile;

        // Create a temporary PDF file
        let mut temp_file = NamedTempFile::new().unwrap();
        temp_file.write_all(b"fake pdf data").unwrap();
        let temp_path = temp_file.path().to_str().unwrap().to_string();

        // Rename to have .pdf extension
        let pdf_path = format!("{}.pdf", temp_path);
        std::fs::copy(&temp_path, &pdf_path).unwrap();

        let config = PaladinYamlConfig {
            name: "test".to_string(),
            system_prompt: "You are a helpful assistant".to_string(),
            model: "gpt-4".to_string(),
            temperature: 0.7,
            max_loops: MaxLoops::Fixed(3),
            timeout_seconds: 300,
            stop_words: vec![],
            provider: ProviderConfig {
                provider_type: "openai".to_string(),
            },
            garrison: None,
            arsenal: None,
            autonomous: None,
            vision_enabled: false,
            images: vec![],
            documents: vec![pdf_path.clone()],
        };

        assert!(config.validate().is_ok());

        // Cleanup
        std::fs::remove_file(&pdf_path).ok();
    }

    #[test]
    fn test_missing_image_file() {
        let config = PaladinYamlConfig {
            name: "test".to_string(),
            system_prompt: "You are a helpful assistant".to_string(),
            model: "gpt-4".to_string(),
            temperature: 0.7,
            max_loops: MaxLoops::Fixed(3),
            timeout_seconds: 300,
            stop_words: vec![],
            provider: ProviderConfig {
                provider_type: "openai".to_string(),
            },
            garrison: None,
            arsenal: None,
            autonomous: None,
            vision_enabled: true,
            images: vec!["/nonexistent/image.png".to_string()],
            documents: vec![],
        };

        assert!(matches!(
            config.validate(),
            Err(CliError::InvalidFilePath { .. })
        ));
    }

    #[test]
    fn test_unsupported_image_format() {
        use std::io::Write;
        use tempfile::NamedTempFile;

        // Create a temporary file with unsupported extension
        let mut temp_file = NamedTempFile::new().unwrap();
        temp_file.write_all(b"fake data").unwrap();
        let temp_path = temp_file.path().to_str().unwrap().to_string();

        // Rename to have .bmp extension (unsupported)
        let bmp_path = format!("{}.bmp", temp_path);
        std::fs::copy(&temp_path, &bmp_path).unwrap();

        let config = PaladinYamlConfig {
            name: "test".to_string(),
            system_prompt: "You are a helpful assistant".to_string(),
            model: "gpt-4".to_string(),
            temperature: 0.7,
            max_loops: MaxLoops::Fixed(3),
            timeout_seconds: 300,
            stop_words: vec![],
            provider: ProviderConfig {
                provider_type: "openai".to_string(),
            },
            garrison: None,
            arsenal: None,
            autonomous: None,
            vision_enabled: true,
            images: vec![bmp_path.clone()],
            documents: vec![],
        };

        assert!(matches!(
            config.validate(),
            Err(CliError::UnsupportedFormat { .. })
        ));

        // Cleanup
        std::fs::remove_file(&bmp_path).ok();
    }

    #[test]
    fn test_multiple_images_and_documents() {
        use std::io::Write;
        use tempfile::NamedTempFile;

        // Create temporary files
        let mut img1 = NamedTempFile::new().unwrap();
        img1.write_all(b"fake image 1").unwrap();
        let img1_path = format!("{}.png", img1.path().to_str().unwrap());
        std::fs::copy(img1.path(), &img1_path).unwrap();

        let mut img2 = NamedTempFile::new().unwrap();
        img2.write_all(b"fake image 2").unwrap();
        let img2_path = format!("{}.jpg", img2.path().to_str().unwrap());
        std::fs::copy(img2.path(), &img2_path).unwrap();

        let mut doc1 = NamedTempFile::new().unwrap();
        doc1.write_all(b"fake pdf").unwrap();
        let doc1_path = format!("{}.pdf", doc1.path().to_str().unwrap());
        std::fs::copy(doc1.path(), &doc1_path).unwrap();

        let config = PaladinYamlConfig {
            name: "test".to_string(),
            system_prompt: "You are a helpful assistant".to_string(),
            model: "gpt-4".to_string(),
            temperature: 0.7,
            max_loops: MaxLoops::Fixed(3),
            timeout_seconds: 300,
            stop_words: vec![],
            provider: ProviderConfig {
                provider_type: "openai".to_string(),
            },
            garrison: None,
            arsenal: None,
            autonomous: None,
            vision_enabled: true,
            images: vec![img1_path.clone(), img2_path.clone()],
            documents: vec![doc1_path.clone()],
        };

        assert!(config.validate().is_ok());

        // Cleanup
        std::fs::remove_file(&img1_path).ok();
        std::fs::remove_file(&img2_path).ok();
        std::fs::remove_file(&doc1_path).ok();
    }

    fn base_config_with_arsenal(arsenal: ArsenalConfig) -> PaladinYamlConfig {
        PaladinYamlConfig {
            name: "test-paladin".to_string(),
            system_prompt: "You are a helpful assistant".to_string(),
            model: "gpt-4".to_string(),
            temperature: 0.7,
            max_loops: MaxLoops::Fixed(3),
            timeout_seconds: 300,
            stop_words: vec![],
            provider: ProviderConfig {
                provider_type: "openai".to_string(),
            },
            garrison: None,
            arsenal: Some(arsenal),
            autonomous: None,
            vision_enabled: false,
            images: vec![],
            documents: vec![],
        }
    }

    #[test]
    fn test_validate_accepts_streamable_http_server_type() {
        // Regression test (Phase 12.1 Plan 05): `validate()`'s allowlist
        // previously omitted "streamable_http" entirely, so every real,
        // config-driven remote MCP server (D-02/D-03) was rejected before
        // ever reaching the loader's `instantiate_arsenal` match arm.
        let config = base_config_with_arsenal(ArsenalConfig {
            mcp_servers: vec![McpServerConfig {
                name: "etherscan".to_string(),
                server_type: "streamable_http".to_string(),
                command: None,
                args: None,
                endpoint: Some("https://mcp.etherscan.io/mcp".to_string()),
                auth_token_env: Some("ETHERSCAN_API_KEY".to_string()),
            }],
        });

        assert!(
            config.validate().is_ok(),
            "streamable_http must be a valid arsenal.mcp_servers[].type"
        );
    }

    #[test]
    fn test_validate_rejects_streamable_http_missing_endpoint() {
        let config = base_config_with_arsenal(ArsenalConfig {
            mcp_servers: vec![McpServerConfig {
                name: "etherscan".to_string(),
                server_type: "streamable_http".to_string(),
                command: None,
                args: None,
                endpoint: None,
                auth_token_env: None,
            }],
        });

        assert!(matches!(
            config.validate(),
            Err(CliError::MissingRequiredField { .. })
        ));
    }

    #[test]
    fn test_validate_still_accepts_deprecated_sse_shape() {
        // "sse" must still pass this schema-level `validate()` check so the
        // loader's own actionable deprecation error (not a generic
        // validation rejection) is what the caller sees.
        let config = base_config_with_arsenal(ArsenalConfig {
            mcp_servers: vec![McpServerConfig {
                name: "legacy".to_string(),
                server_type: "sse".to_string(),
                command: None,
                args: None,
                endpoint: Some("http://localhost:8080/mcp".to_string()),
                auth_token_env: None,
            }],
        });

        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_validate_rejects_unknown_server_type() {
        let config = base_config_with_arsenal(ArsenalConfig {
            mcp_servers: vec![McpServerConfig {
                name: "bogus".to_string(),
                server_type: "websocket".to_string(),
                command: None,
                args: None,
                endpoint: None,
                auth_token_env: None,
            }],
        });

        assert!(matches!(
            config.validate(),
            Err(CliError::InvalidFieldValue { .. })
        ));
    }

    // ------------------------------------------------------------------
    // D-06: `autonomous` section (CLOSE-02, Epic 14 cluster 8.0)
    // ------------------------------------------------------------------

    #[test]
    fn test_load_paladin_config_with_autonomous_section() {
        // A YAML document carrying an `autonomous:` block with
        // `planning.enabled: true` deserializes into a `PaladinYamlConfig`
        // whose `autonomous` is `Some` and whose `autonomous.planning.enabled`
        // is `true` -- reusing `paladin-core`'s `AutonomousConfig` directly
        // (D-06), with no CLI-local mirror type.
        //
        // Hand-written YAML rather than a serialize-then-deserialize round
        // trip: `serde_yaml` 0.9 (deprecated) has a known limitation
        // deserializing `MaxLoops`'s untagged enum representation back out
        // of a document that also carries other nested enums (here,
        // `HandoffStrategy`) -- unrelated to the `autonomous` wiring under
        // test.
        let yaml = r#"
name: "test-paladin"
system_prompt: "You are a helpful assistant"
model: "gpt-4"
provider:
  type: "openai"
autonomous:
  planning:
    enabled: true
    max_subtasks: 7
  prompt_generation:
    enabled: false
  dynamic_temperature:
    enabled: false
    min: 0.1
    max: 0.9
  handoffs:
    enabled: false
    strategy: Automatic
    max_depth: 5
    retry:
      max_retries: 3
      initial_backoff_ms: 1000
      backoff_multiplier: 2.0
"#;

        let reloaded: PaladinYamlConfig =
            serde_yaml::from_str(yaml).expect("deserialize PaladinYamlConfig");

        let autonomous = reloaded
            .autonomous
            .expect("autonomous section must deserialize as Some");
        assert!(autonomous.planning.enabled);
        assert_eq!(autonomous.planning.max_subtasks, 7);
    }

    #[test]
    fn test_load_paladin_config_without_autonomous_section() {
        // A YAML document with no `autonomous:` key deserializes with
        // `autonomous` equal to `None` -- an existing config gains no new
        // capability by upgrading (T-06-03-01).
        let yaml = r#"
name: "test-paladin"
system_prompt: "You are a helpful assistant"
model: "gpt-4"
provider:
  type: "openai"
"#;

        let config: PaladinYamlConfig =
            serde_yaml::from_str(yaml).expect("deserialize PaladinYamlConfig");
        assert!(config.autonomous.is_none());
    }

    #[test]
    fn test_paladin_yaml_config_omits_autonomous_when_none() {
        // Serializing a `PaladinYamlConfig` whose `autonomous` is `None`
        // produces output with no `autonomous` key, matching the
        // `garrison`/`arsenal` optional-section convention.
        let config = base_config_with_arsenal(ArsenalConfig {
            mcp_servers: vec![],
        });
        assert!(config.autonomous.is_none());

        let yaml = serde_yaml::to_string(&config).expect("serialize PaladinYamlConfig");
        assert!(
            !yaml.contains("autonomous"),
            "serialized YAML must omit the autonomous key when None: {yaml}"
        );
    }
}
