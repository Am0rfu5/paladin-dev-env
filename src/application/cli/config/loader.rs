//! Configuration file loading utilities

use crate::application::cli::config::battalion_config::BattalionYamlConfig;
use crate::application::cli::config::paladin_config::{
    ArsenalConfig, GarrisonConfig, PaladinYamlConfig, Validate,
};
use crate::application::cli::error::CliError;
use crate::application::services::arsenal::arsenal_execution_service::ArsenalExecutionService;
use crate::application::services::arsenal::arsenal_registry_service::ArsenalRegistryService;
use crate::core::platform::container::garrison::{
    EvictionStrategy, GarrisonConfig as CoreGarrisonConfig,
};
use crate::infrastructure::adapters::arsenal::mcp_protocol::MCPClient;
use crate::infrastructure::adapters::arsenal::mcp_streamable_http_adapter::MCPStreamableHttpAdapter;
use crate::infrastructure::adapters::garrison::in_memory_garrison::InMemoryGarrison;
use crate::infrastructure::adapters::garrison::sqlite_garrison::SqliteGarrison;
use paladin_ports::output::arsenal_port::{ArsenalPort, ArsenalRegistry};
use paladin_ports::output::garrison_port::GarrisonPort;
use std::fs;
use std::path::Path;
use std::sync::Arc;

/// Load Paladin configuration from YAML file
///
/// # Arguments
/// * `path` - Path to the YAML configuration file
///
/// # Returns
/// * `Ok(PaladinYamlConfig)` - Loaded and validated configuration
/// * `Err(CliError)` - File not found, invalid YAML, or validation error
pub fn load_paladin_config(path: &Path) -> Result<PaladinYamlConfig, CliError> {
    // Check if file exists
    if !path.exists() {
        return Err(CliError::ConfigFileNotFound {
            path: path.to_path_buf(),
        });
    }

    // Read file contents
    let contents = fs::read_to_string(path).map_err(|e| CliError::IoError {
        message: format!("Failed to read config file: {}", path.display()),
        source: e,
    })?;

    // Parse YAML
    let config: PaladinYamlConfig =
        serde_yaml::from_str(&contents).map_err(|e| CliError::InvalidYaml {
            path: path.to_path_buf(),
            source: e,
        })?;

    // Validate configuration
    config.validate()?;

    Ok(config)
}

/// Load Battalion configuration from YAML file
///
/// # Arguments
/// * `path` - Path to the YAML configuration file
///
/// # Returns
/// * `Ok(BattalionYamlConfig)` - Loaded and validated configuration
/// * `Err(CliError)` - File not found, invalid YAML, or validation error
pub fn load_battalion_config(path: &Path) -> Result<BattalionYamlConfig, CliError> {
    // Check if file exists
    if !path.exists() {
        return Err(CliError::ConfigFileNotFound {
            path: path.to_path_buf(),
        });
    }

    // Read file contents
    let contents = fs::read_to_string(path).map_err(|e| CliError::IoError {
        message: format!("Failed to read config file: {}", path.display()),
        source: e,
    })?;

    // Parse YAML
    let config: BattalionYamlConfig =
        serde_yaml::from_str(&contents).map_err(|e| CliError::InvalidYaml {
            path: path.to_path_buf(),
            source: e,
        })?;

    // Validate configuration
    config.validate()?;

    Ok(config)
}

/// Instantiate a garrison adapter from YAML configuration
///
/// # Arguments
/// * `config` - Optional garrison configuration from YAML
/// * `paladin_name` - Name of the paladin (used as ID for SQLite garrison)
///
/// # Returns
/// * `Ok(Some(Arc<dyn GarrisonPort>))` - If garrison config is provided and valid
/// * `Ok(None)` - If no garrison config is provided
/// * `Err(CliError)` - If garrison instantiation fails
///
/// # Configuration Schema
///
/// ```yaml
/// garrison:
///   type: "in_memory"  # or "sqlite"
///   config:
///     max_entries: 100
///     path: "./garrison.db"  # Required for sqlite type
/// ```
pub async fn instantiate_garrison(
    config: &Option<GarrisonConfig>,
    paladin_name: &str,
) -> Result<Option<Arc<dyn GarrisonPort>>, CliError> {
    let Some(garrison_config) = config else {
        return Ok(None);
    };

    // Validate garrison type
    if garrison_config.garrison_type != "in_memory" && garrison_config.garrison_type != "sqlite" {
        return Err(CliError::GarrisonConfigError {
            message: format!(
                "garrison.type must be 'in_memory' or 'sqlite', got: '{}'",
                garrison_config.garrison_type
            ),
        });
    }

    // Extract configuration parameters with defaults
    let max_entries = garrison_config
        .config
        .as_ref()
        .and_then(|c| c.max_entries)
        .unwrap_or(100);

    // Create core garrison configuration
    let core_config = CoreGarrisonConfig {
        max_entries,
        max_tokens: Some(4000),
        eviction_strategy: EvictionStrategy::ImportanceBased,
        preserve_recent_count: 10,
    };

    // Instantiate the appropriate garrison type
    match garrison_config.garrison_type.as_str() {
        "in_memory" => {
            let garrison = InMemoryGarrison::new(core_config);
            Ok(Some(Arc::new(garrison) as Arc<dyn GarrisonPort>))
        }
        "sqlite" => {
            // Extract path from config
            let path = garrison_config
                .config
                .as_ref()
                .and_then(|c| c.path.as_ref())
                .ok_or_else(|| CliError::GarrisonConfigError {
                    message: "garrison.config.path is required for type: sqlite".to_string(),
                })?;

            // Validate path is writable (check parent directory exists)
            if let Some(parent) = Path::new(path).parent()
                && !parent.exists()
            {
                // Try to create parent directory
                std::fs::create_dir_all(parent).map_err(|e| CliError::GarrisonConfigError {
                    message: format!(
                        "garrison.config.path parent directory does not exist and could not be created: {} - {}",
                        parent.display(),
                        e
                    ),
                })?;
            }

            // Connect to SQLite garrison
            let garrison = SqliteGarrison::connect(path, core_config, paladin_name)
                .await
                .map_err(|e| CliError::GarrisonConfigError {
                    message: format!("Failed to connect to SQLite garrison at '{}': {}", path, e),
                })?;

            Ok(Some(Arc::new(garrison) as Arc<dyn GarrisonPort>))
        }
        _ => unreachable!("Garrison type already validated"),
    }
}

/// Instantiate an arsenal from YAML configuration
///
/// # Arguments
/// * `config` - Optional arsenal configuration from YAML
///
/// # Returns
/// * `Ok(Some(Arc<dyn ArsenalPort>))` - If arsenal config is provided and valid
/// * `Ok(None)` - If no arsenal config is provided
/// * `Err(CliError)` - If arsenal instantiation fails
///
/// # Configuration Schema
///
/// ```yaml
/// arsenal:
///   mcp_servers:
///     - name: "web_search"
///       type: "stdio"
///       command: "uvx"
///       args:
///         - "mcp-web-search"
/// ```
///
/// Note: `type: "streamable_http"` (remote, authenticated MCP servers, D-02/
/// D-03) requires an `endpoint` and, if the server needs auth, an
/// `auth_token_env` naming the environment variable that holds the bearer
/// token — never a literal secret in this config. The retired `type: "sse"`
/// value fails loud with a migration message instead of silently misbehaving.
pub async fn instantiate_arsenal(
    config: &Option<ArsenalConfig>,
) -> Result<Option<Arc<dyn ArsenalPort>>, CliError> {
    let Some(arsenal_config) = config else {
        return Ok(None);
    };

    // Create registry service, wrapping it once so both the loop below (via
    // the concrete type, for `.register()`) and the execution service (via
    // the trait object) share the SAME underlying store.
    let registry = Arc::new(ArsenalRegistryService::new());
    let service = ArsenalExecutionService::new(registry.clone() as Arc<dyn ArsenalRegistry>);

    // If no MCP servers configured, return empty arsenal
    if arsenal_config.mcp_servers.is_empty() {
        return Ok(Some(Arc::new(service) as Arc<dyn ArsenalPort>));
    }

    // Process each MCP server
    for server_config in &arsenal_config.mcp_servers {
        match server_config.server_type.as_str() {
            "stdio" => {
                // Validate required fields for stdio
                let command =
                    server_config
                        .command
                        .as_ref()
                        .ok_or_else(|| CliError::ArsenalConfigError {
                            message: format!(
                                "arsenal.mcp_servers[{}].command is required for stdio type",
                                server_config.name
                            ),
                        })?;

                let args = server_config.args.clone().unwrap_or_default();

                // Connect directly via the rmcp-backed MCPClient (D-01/D-04):
                // the handshake (initialize -> notifications/initialized)
                // happens inside connect_stdio, no separate adapter step.
                let client = MCPClient::connect_stdio(command, &args)
                    .await
                    .map_err(|e| CliError::ArsenalConfigError {
                        message: format!(
                            "Failed to connect to STDIO MCP server '{}': {}",
                            server_config.name, e
                        ),
                    })?;

                let tools =
                    client
                        .discover_tools()
                        .await
                        .map_err(|e| CliError::ArsenalConfigError {
                            message: format!(
                                "Failed to discover tools from MCP server '{}': {}",
                                server_config.name, e
                            ),
                        })?;

                // Register all tools' metadata, then register the client
                // that serves them so `ArsenalExecutionService::invoke` has
                // a real MCP connection to route through (D-05).
                let tool_names: Vec<String> = tools.iter().map(|t| t.name.clone()).collect();
                for tool in tools {
                    registry.register(tool).await;
                }
                service.register_client(tool_names, Arc::new(client)).await;
            }
            "streamable_http" => {
                // D-02/D-03: remote, authenticated MCP server. `endpoint` is
                // required; `auth_token_env`, if present, NAMES the env var
                // holding the bearer token -- the token itself is resolved
                // host-side here and never logged or stored back in config.
                let endpoint = server_config.endpoint.as_ref().ok_or_else(|| {
                    CliError::ArsenalConfigError {
                        message: format!(
                            "arsenal.mcp_servers[{}].endpoint is required for streamable_http type",
                            server_config.name
                        ),
                    }
                })?;

                let bearer_token = match server_config.auth_token_env.as_ref() {
                    Some(env_var_name) => {
                        let token = std::env::var(env_var_name).map_err(|_| {
                            CliError::ArsenalConfigError {
                                message: format!(
                                    "arsenal.mcp_servers[{}].auth_token_env references '{}', but that environment variable is not set",
                                    server_config.name, env_var_name
                                ),
                            }
                        })?;
                        Some(token)
                    }
                    None => None,
                };

                let mut adapter = MCPStreamableHttpAdapter::new(endpoint.clone());
                if let Some(token) = bearer_token {
                    adapter = adapter.with_bearer_token(token);
                }

                let client = adapter
                    .connect()
                    .await
                    .map_err(|e| CliError::ArsenalConfigError {
                        message: format!(
                            "Failed to connect to Streamable-HTTP MCP server '{}': {}",
                            server_config.name, e
                        ),
                    })?;

                let tools =
                    client
                        .discover_tools()
                        .await
                        .map_err(|e| CliError::ArsenalConfigError {
                            message: format!(
                                "Failed to discover tools from MCP server '{}': {}",
                                server_config.name, e
                            ),
                        })?;

                let tool_names: Vec<String> = tools.iter().map(|t| t.name.clone()).collect();
                for tool in tools {
                    registry.register(tool).await;
                }
                service.register_client(tool_names, Arc::new(client)).await;
            }
            "sse" => {
                // D-02b: the "sse" transport was actually a mislabeled,
                // unauthenticated plain-HTTP-POST adapter (never real SSE or
                // Streamable-HTTP) — retired entirely. Fail loud with an
                // actionable migration message rather than silently
                // constructing a since-removed adapter.
                return Err(CliError::ArsenalConfigError {
                    message: format!(
                        "arsenal.mcp_servers[{}].type 'sse' is deprecated: the mislabeled plain-HTTP adapter has been retired. Use 'streamable_http' instead.",
                        server_config.name
                    ),
                });
            }
            other => {
                return Err(CliError::ArsenalConfigError {
                    message: format!(
                        "arsenal.mcp_servers[{}].type must be 'stdio' or 'streamable_http', got: '{}'",
                        server_config.name, other
                    ),
                });
            }
        }
    }

    Ok(Some(Arc::new(service) as Arc<dyn ArsenalPort>))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_load_valid_paladin_config() {
        let yaml = r#"
name: test-paladin
system_prompt: "You are a helpful assistant"
model: gpt-4
temperature: 0.7
max_loops: 3
timeout_seconds: 300
stop_words: []
provider:
  type: openai
"#;

        let mut file = NamedTempFile::new().unwrap();
        file.write_all(yaml.as_bytes()).unwrap();
        file.flush().unwrap();

        let config = load_paladin_config(file.path()).unwrap();
        assert_eq!(config.name, "test-paladin");
        assert_eq!(config.model, "gpt-4");
        assert_eq!(config.provider.provider_type, "openai");
    }

    #[test]
    fn test_load_paladin_config_file_not_found() {
        let result = load_paladin_config(Path::new("/nonexistent/file.yaml"));
        assert!(matches!(result, Err(CliError::ConfigFileNotFound { .. })));
    }

    #[test]
    fn test_load_paladin_config_invalid_yaml() {
        let yaml = r#"
name: test
invalid yaml syntax: [unclosed
"#;

        let mut file = NamedTempFile::new().unwrap();
        file.write_all(yaml.as_bytes()).unwrap();
        file.flush().unwrap();

        let result = load_paladin_config(file.path());
        assert!(matches!(result, Err(CliError::InvalidYaml { .. })));
    }

    #[test]
    fn test_load_paladin_config_missing_required_field() {
        let yaml = r#"
name: test
# Missing system_prompt
model: gpt-4
provider:
  type: openai
"#;

        let mut file = NamedTempFile::new().unwrap();
        file.write_all(yaml.as_bytes()).unwrap();
        file.flush().unwrap();

        let result = load_paladin_config(file.path());
        // Serde will fail on missing required field during deserialization
        assert!(matches!(result, Err(CliError::InvalidYaml { .. })));
    }

    #[test]
    fn test_load_paladin_config_validation_error() {
        // Test validation errors (empty fields that pass deserialization)
        let yaml = r#"
name: ""
system_prompt: "test"
model: gpt-4
provider:
  type: openai
"#;

        let mut file = NamedTempFile::new().unwrap();
        file.write_all(yaml.as_bytes()).unwrap();
        file.flush().unwrap();

        let result = load_paladin_config(file.path());
        assert!(matches!(result, Err(CliError::MissingRequiredField { .. })));
    }

    #[test]
    fn test_load_valid_formation_config() {
        let yaml = r#"
type: formation
name: test-formation
pass_output_to_next: true
paladins:
  - file: paladin1.yaml
  - file: paladin2.yaml
"#;

        let mut file = NamedTempFile::new().unwrap();
        file.write_all(yaml.as_bytes()).unwrap();
        file.flush().unwrap();

        let config = load_battalion_config(file.path()).unwrap();
        assert_eq!(config.battalion_type(), "formation");
    }

    #[test]
    fn test_load_paladin_config_with_vision_fields() {
        use std::io::Write;

        // Create temporary image and document files
        let mut img_file = NamedTempFile::new().unwrap();
        img_file.write_all(b"fake image").unwrap();
        let img_path_temp = img_file.path().to_str().unwrap().to_string();
        let img_path = format!("{}.png", img_path_temp);
        std::fs::copy(&img_path_temp, &img_path).unwrap();

        let mut doc_file = NamedTempFile::new().unwrap();
        doc_file.write_all(b"fake pdf").unwrap();
        let doc_path_temp = doc_file.path().to_str().unwrap().to_string();
        let doc_path = format!("{}.pdf", doc_path_temp);
        std::fs::copy(&doc_path_temp, &doc_path).unwrap();

        let yaml = format!(
            r#"
name: vision-paladin
system_prompt: "You are a vision-capable assistant"
model: gpt-4
temperature: 0.7
max_loops: 3
timeout_seconds: 300
vision_enabled: true
images:
  - {}
documents:
  - {}
provider:
  type: openai
"#,
            img_path, doc_path
        );

        let mut file = NamedTempFile::new().unwrap();
        file.write_all(yaml.as_bytes()).unwrap();
        file.flush().unwrap();

        let config = load_paladin_config(file.path()).unwrap();
        assert_eq!(config.name, "vision-paladin");
        assert!(config.vision_enabled);
        assert_eq!(config.images.len(), 1);
        assert_eq!(config.documents.len(), 1);

        // Cleanup
        std::fs::remove_file(&img_path).ok();
        std::fs::remove_file(&doc_path).ok();
    }

    #[test]
    fn test_load_paladin_config_vision_enabled_without_files() {
        let yaml = r#"
name: test-paladin
system_prompt: "You are a helpful assistant"
model: gpt-4
temperature: 0.7
max_loops: 3
timeout_seconds: 300
vision_enabled: true
images: []
documents: []
provider:
  type: openai
"#;

        let mut file = NamedTempFile::new().unwrap();
        file.write_all(yaml.as_bytes()).unwrap();
        file.flush().unwrap();

        let result = load_paladin_config(file.path());
        // Should fail validation: vision_enabled true but no files
        assert!(matches!(
            result,
            Err(CliError::InvalidFieldValue { field, .. }) if field == "vision_enabled"
        ));
    }

    // --- instantiate_arsenal: streamable_http (D-02/D-03) ---

    use crate::application::cli::config::paladin_config::McpServerConfig;

    #[tokio::test]
    async fn test_instantiate_arsenal_streamable_http_missing_endpoint_errors() {
        let config = ArsenalConfig {
            mcp_servers: vec![McpServerConfig {
                name: "etherscan".to_string(),
                server_type: "streamable_http".to_string(),
                command: None,
                args: None,
                endpoint: None,
                auth_token_env: None,
            }],
        };

        let result = instantiate_arsenal(&Some(config)).await;

        assert!(result.is_err(), "expected an error for missing endpoint");
        if let Err(CliError::ArsenalConfigError { message }) = result {
            assert!(
                message.contains("endpoint is required"),
                "expected an endpoint-required error, got: {message}"
            );
        } else {
            panic!("Expected ArsenalConfigError");
        }
    }

    #[tokio::test]
    async fn test_instantiate_arsenal_streamable_http_missing_auth_token_env_var_errors() {
        // auth_token_env NAMES an env var that is (deliberately, for this
        // test) never set -- must fail loud rather than connecting
        // unauthenticated (D-03).
        let config = ArsenalConfig {
            mcp_servers: vec![McpServerConfig {
                name: "etherscan".to_string(),
                server_type: "streamable_http".to_string(),
                command: None,
                args: None,
                endpoint: Some("https://mcp.etherscan.io/mcp".to_string()),
                auth_token_env: Some("PALADIN_TEST_UNSET_AUTH_TOKEN_ENV_VAR_12_1_02".to_string()),
            }],
        };

        let result = instantiate_arsenal(&Some(config)).await;

        assert!(
            result.is_err(),
            "expected an error when auth_token_env references an unset var"
        );
        if let Err(CliError::ArsenalConfigError { message }) = result {
            assert!(
                message.contains("PALADIN_TEST_UNSET_AUTH_TOKEN_ENV_VAR_12_1_02")
                    && message.contains("not set"),
                "expected an unset-env-var error, got: {message}"
            );
        } else {
            panic!("Expected ArsenalConfigError");
        }
    }

    #[tokio::test]
    async fn test_instantiate_arsenal_streamable_http_attempts_connect_when_configured() {
        // No live server exists at this endpoint in the test environment --
        // this proves the streamable_http arm reaches MCPClient::connect_streamable_http
        // (rather than silently no-op'ing) by asserting it fails with a
        // connection error, not a validation error.
        let config = ArsenalConfig {
            mcp_servers: vec![McpServerConfig {
                name: "unreachable".to_string(),
                server_type: "streamable_http".to_string(),
                command: None,
                args: None,
                endpoint: Some("http://127.0.0.1:1/mcp".to_string()),
                auth_token_env: None,
            }],
        };

        let result = instantiate_arsenal(&Some(config)).await;

        assert!(result.is_err(), "expected a connection error");
        if let Err(CliError::ArsenalConfigError { message }) = result {
            assert!(
                message.contains("Failed to connect"),
                "expected a connection-failure error, got: {message}"
            );
        } else {
            panic!("Expected ArsenalConfigError");
        }
    }
}
