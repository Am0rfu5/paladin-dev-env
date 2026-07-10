/// Unit tests for arsenal configuration parsing and instantiation
use paladin::application::cli::config::loader::instantiate_arsenal;
use paladin::application::cli::config::paladin_config::{ArsenalConfig, McpServerConfig};
use paladin::application::cli::error::CliError;

/// Test 2.8.1: Parse valid stdio MCP server config
#[tokio::test]
async fn test_stdio_mcp_server_config() {
    // Arrange: Create a valid stdio MCP server config
    let config = ArsenalConfig {
        mcp_servers: vec![McpServerConfig {
            name: "test_server".to_string(),
            server_type: "stdio".to_string(),
            command: Some("uvx".to_string()),
            args: Some(vec!["mcp-web-search".to_string()]),
            endpoint: None,
        auth_token_env: None,
        }],
    };

    // Act: Instantiate arsenal (will attempt to connect, expect failure without actual server)
    let result = instantiate_arsenal(&Some(config)).await;

    // Assert: Should fail with connection error (server doesn't exist in test environment)
    assert!(
        result.is_err(),
        "Expected error when connecting to non-existent MCP server"
    );
    if let Err(CliError::ArsenalConfigError { message }) = result {
        assert!(
            message.contains("Failed to connect") || message.contains("Failed to discover"),
            "Expected connection/discovery error, got: {}",
            message
        );
    } else {
        panic!("Expected ArsenalConfigError");
    }
}

/// Test 2.8.2: `sse` is retired -- fails loud with an actionable migration
/// message rather than attempting to connect via the removed adapter (D-02b).
#[tokio::test]
async fn test_sse_mcp_server_config() {
    // Arrange: Create a legacy sse MCP server config
    let config = ArsenalConfig {
        mcp_servers: vec![McpServerConfig {
            name: "api_service".to_string(),
            server_type: "sse".to_string(),
            command: None,
            args: None,
            endpoint: Some("http://localhost:8080/mcp".to_string()),
        auth_token_env: None,
        }],
    };

    // Act: Instantiate arsenal
    let result = instantiate_arsenal(&Some(config)).await;

    // Assert: Should fail loud with a deprecation/migration message, never a
    // silent no-op or a generic connection-failure message.
    assert!(
        result.is_err(),
        "Expected a deprecation error for the retired 'sse' server type"
    );
    if let Err(CliError::ArsenalConfigError { message }) = result {
        assert!(
            message.contains("deprecated") && message.contains("streamable_http"),
            "Expected a migration message pointing at 'streamable_http', got: {}",
            message
        );
    } else {
        panic!("Expected ArsenalConfigError");
    }
}

/// Test 2.8.3: Parse multiple MCP servers
#[tokio::test]
async fn test_multiple_mcp_servers() {
    // Arrange: Create config with multiple MCP servers
    let config = ArsenalConfig {
        mcp_servers: vec![
            McpServerConfig {
                name: "server1".to_string(),
                server_type: "stdio".to_string(),
                command: Some("uvx".to_string()),
                args: Some(vec!["mcp-web-search".to_string()]),
                endpoint: None,
            auth_token_env: None,
            },
            McpServerConfig {
                name: "server2".to_string(),
                server_type: "sse".to_string(),
                command: None,
                args: None,
                endpoint: Some("http://localhost:8080/mcp".to_string()),
            auth_token_env: None,
            },
        ],
    };

    // Act: Instantiate arsenal
    let result = instantiate_arsenal(&Some(config)).await;

    // Assert: Should fail when connecting to first server (doesn't exist in test)
    assert!(
        result.is_err(),
        "Expected error when connecting to non-existent servers"
    );
}

/// Test 2.8.4: Validate error for missing name
#[tokio::test]
async fn test_missing_name() {
    // Note: This is enforced by serde deserialization, so we can't easily test it
    // Skipping this test as name is a required field in struct definition
}

/// Test 2.8.5: Validate error for invalid server type
#[tokio::test]
async fn test_invalid_server_type() {
    // Arrange: Create config with invalid server type
    let config = ArsenalConfig {
        mcp_servers: vec![McpServerConfig {
            name: "test_server".to_string(),
            server_type: "invalid_type".to_string(),
            command: Some("command".to_string()),
            args: None,
            endpoint: None,
        auth_token_env: None,
        }],
    };

    // Act: Instantiate arsenal
    let result = instantiate_arsenal(&Some(config)).await;

    // Assert: Should fail with ArsenalConfigError
    assert!(result.is_err(), "Expected error for invalid server type");
    if let Err(CliError::ArsenalConfigError { message }) = result {
        assert!(
            message.contains("must be 'stdio'"),
            "Expected type validation error, got: {}",
            message
        );
    } else {
        panic!("Expected ArsenalConfigError");
    }
}

/// Test 2.8.6: Validate error for missing command for stdio
#[tokio::test]
async fn test_stdio_missing_command() {
    // Arrange: Create stdio config without command
    let config = ArsenalConfig {
        mcp_servers: vec![McpServerConfig {
            name: "test_server".to_string(),
            server_type: "stdio".to_string(),
            command: None, // Missing command for stdio
            args: Some(vec!["arg1".to_string()]),
            endpoint: None,
        auth_token_env: None,
        }],
    };

    // Act: Instantiate arsenal
    let result = instantiate_arsenal(&Some(config)).await;

    // Assert: Should fail with ArsenalConfigError
    assert!(result.is_err(), "Expected error for stdio without command");
    if let Err(CliError::ArsenalConfigError { message }) = result {
        assert!(
            message.contains("command is required"),
            "Expected 'command is required' error, got: {}",
            message
        );
    } else {
        panic!("Expected ArsenalConfigError");
    }
}

/// Test 2.8.7: `sse` is retired unconditionally -- even without an endpoint,
/// the deprecation error fires (there is no longer a field-level check to
/// short-circuit on, since the adapter itself is gone).
#[tokio::test]
async fn test_sse_missing_endpoint() {
    // Arrange: Create sse config without endpoint
    let config = ArsenalConfig {
        mcp_servers: vec![McpServerConfig {
            name: "test_server".to_string(),
            server_type: "sse".to_string(),
            command: None,
            args: None,
            endpoint: None, // Missing endpoint for sse
            auth_token_env: None,
        }],
    };

    // Act: Instantiate arsenal
    let result = instantiate_arsenal(&Some(config)).await;

    // Assert: Should fail loud with the deprecation/migration message
    assert!(result.is_err(), "Expected error for deprecated 'sse' type");
    if let Err(CliError::ArsenalConfigError { message }) = result {
        assert!(
            message.contains("deprecated") && message.contains("streamable_http"),
            "Expected a migration message pointing at 'streamable_http', got: {}",
            message
        );
    } else {
        panic!("Expected ArsenalConfigError");
    }
}

/// Test 2.8.8: `sse` is retired unconditionally -- an invalid URL format
/// still resolves to the deprecation message, not a URL-format error, since
/// no field-level validation runs for the retired type anymore.
#[tokio::test]
async fn test_sse_invalid_url_format() {
    // Arrange: Create sse config with invalid URL
    let config = ArsenalConfig {
        mcp_servers: vec![McpServerConfig {
            name: "test_server".to_string(),
            server_type: "sse".to_string(),
            command: None,
            args: None,
            endpoint: Some("invalid-url".to_string()), // Invalid URL format
            auth_token_env: None,
        }],
    };

    // Act: Instantiate arsenal
    let result = instantiate_arsenal(&Some(config)).await;

    // Assert: Should fail loud with the deprecation/migration message
    assert!(result.is_err(), "Expected error for deprecated 'sse' type");
    if let Err(CliError::ArsenalConfigError { message }) = result {
        assert!(
            message.contains("deprecated") && message.contains("streamable_http"),
            "Expected a migration message pointing at 'streamable_http', got: {}",
            message
        );
    } else {
        panic!("Expected ArsenalConfigError");
    }
}

/// Test 2.8.9: Validate None config returns None arsenal
#[tokio::test]
async fn test_none_arsenal_config() {
    // Arrange: No arsenal config provided
    let config: Option<ArsenalConfig> = None;

    // Act: Instantiate arsenal
    let result = instantiate_arsenal(&config).await;

    // Assert: Should succeed and return None
    assert!(result.is_ok(), "Failed with None config");
    let arsenal = result.unwrap();
    assert!(arsenal.is_none(), "Expected None arsenal for None config");
}

/// Test 2.8.10: Validate empty mcp_servers returns empty arsenal
#[tokio::test]
async fn test_empty_mcp_servers() {
    // Arrange: Create config with empty mcp_servers
    let config = ArsenalConfig {
        mcp_servers: vec![],
    };

    // Act: Instantiate arsenal
    let result = instantiate_arsenal(&Some(config)).await;

    // Assert: Should succeed with empty arsenal (no tools registered)
    assert!(
        result.is_ok(),
        "Failed to instantiate with empty mcp_servers"
    );
    let arsenal = result.unwrap();
    assert!(
        arsenal.is_some(),
        "Expected Some arsenal with empty servers"
    );
}
