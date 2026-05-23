//! Unit tests for Arsenal configuration loading

use paladin::config::{ArsenalConfig, MCPServerConfig};
use serde_json;

#[test]
fn test_arsenal_config_default() {
    let config = ArsenalConfig::default();

    assert_eq!(config.default_timeout_seconds, 30);
    assert_eq!(config.max_concurrent_tools, 5);
    assert_eq!(config.mcp_servers.len(), 0);
}

#[test]
fn test_arsenal_config_deserialization_with_servers() {
    let json = r#"{
  "default_timeout_seconds": 60,
  "max_concurrent_tools": 10,
  "mcp_servers": [
    {
      "name": "web_search",
      "server_type": "stdio",
      "command": "uvx",
      "args": ["mcp-web-search"],
      "endpoint": null
    },
    {
      "name": "code_analyzer",
      "server_type": "sse",
      "command": null,
      "args": null,
      "endpoint": "http://localhost:8080/mcp"
    }
  ]
}"#;

    let config: ArsenalConfig = serde_json::from_str(json).expect("Failed to deserialize");

    assert_eq!(config.default_timeout_seconds, 60);
    assert_eq!(config.max_concurrent_tools, 10);
    assert_eq!(config.mcp_servers.len(), 2);

    // Check STDIO server
    let stdio_server = &config.mcp_servers[0];
    assert_eq!(stdio_server.name, "web_search");
    assert_eq!(stdio_server.server_type, "stdio");
    assert_eq!(stdio_server.command, Some("uvx".to_string()));
    assert_eq!(stdio_server.args, Some(vec!["mcp-web-search".to_string()]));
    assert_eq!(stdio_server.endpoint, None);

    // Check SSE server
    let sse_server = &config.mcp_servers[1];
    assert_eq!(sse_server.name, "code_analyzer");
    assert_eq!(sse_server.server_type, "sse");
    assert_eq!(sse_server.command, None);
    assert_eq!(sse_server.args, None);
    assert_eq!(
        sse_server.endpoint,
        Some("http://localhost:8080/mcp".to_string())
    );
}

#[test]
fn test_arsenal_config_deserialization_empty_servers() {
    let json = r#"{
  "default_timeout_seconds": 15,
  "max_concurrent_tools": 3,
  "mcp_servers": []
}"#;

    let config: ArsenalConfig = serde_json::from_str(json).expect("Failed to deserialize");

    assert_eq!(config.default_timeout_seconds, 15);
    assert_eq!(config.max_concurrent_tools, 3);
    assert_eq!(config.mcp_servers.len(), 0);
}

#[test]
fn test_mcp_server_config_stdio_deserialization() {
    let json = r#"{
  "name": "calculator",
  "server_type": "stdio",
  "command": "python",
  "args": ["-m", "mcp_calculator"],
  "endpoint": null
}"#;

    let config: MCPServerConfig = serde_json::from_str(json).expect("Failed to deserialize");

    assert_eq!(config.name, "calculator");
    assert_eq!(config.server_type, "stdio");
    assert_eq!(config.command, Some("python".to_string()));
    assert_eq!(
        config.args,
        Some(vec!["-m".to_string(), "mcp_calculator".to_string()])
    );
    assert_eq!(config.endpoint, None);
}

#[test]
fn test_mcp_server_config_sse_deserialization() {
    let json = r#"{
  "name": "remote_api",
  "server_type": "sse",
  "command": null,
  "args": null,
  "endpoint": "https://api.example.com/mcp"
}"#;

    let config: MCPServerConfig = serde_json::from_str(json).expect("Failed to deserialize");

    assert_eq!(config.name, "remote_api");
    assert_eq!(config.server_type, "sse");
    assert_eq!(config.command, None);
    assert_eq!(config.args, None);
    assert_eq!(
        config.endpoint,
        Some("https://api.example.com/mcp".to_string())
    );
}

#[test]
fn test_arsenal_config_serialization() {
    let config = ArsenalConfig {
        default_timeout_seconds: 45,
        max_concurrent_tools: 7,
        mcp_servers: vec![MCPServerConfig {
            name: "test_tool".to_string(),
            server_type: "stdio".to_string(),
            command: Some("test".to_string()),
            args: Some(vec!["arg".to_string()]),
            endpoint: None,
        }],
    };

    let json = serde_json::to_string(&config).expect("Failed to serialize");

    assert!(json.contains("default_timeout_seconds"));
    assert!(json.contains("45"));
    assert!(json.contains("max_concurrent_tools"));
    assert!(json.contains("7"));
    assert!(json.contains("test_tool"));
    assert!(json.contains("stdio"));
}

#[test]
fn test_arsenal_config_clone() {
    let config1 = ArsenalConfig {
        default_timeout_seconds: 30,
        max_concurrent_tools: 5,
        mcp_servers: vec![MCPServerConfig {
            name: "tool1".to_string(),
            server_type: "stdio".to_string(),
            command: Some("cmd".to_string()),
            args: Some(vec!["arg".to_string()]),
            endpoint: None,
        }],
    };

    let config2 = config1.clone();

    assert_eq!(
        config1.default_timeout_seconds,
        config2.default_timeout_seconds
    );
    assert_eq!(config1.max_concurrent_tools, config2.max_concurrent_tools);
    assert_eq!(config1.mcp_servers.len(), config2.mcp_servers.len());
}
