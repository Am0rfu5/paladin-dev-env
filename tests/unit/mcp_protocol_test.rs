//! Unit tests for MCP protocol implementation

use async_trait::async_trait;
use paladin::core::platform::container::arsenal::ArsenalError;
use paladin::infrastructure::adapters::arsenal::mcp_protocol::{
    MCPClient, MCPError, MCPMessage, MCPNotification, MCPRequest, MCPResponse, MCPTransport,
    ToolInfo,
};
use serde_json::{Value, json};
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};

/// Mock transport for testing
struct MockTransport {
    /// Messages to return when receive() is called
    receive_queue: Arc<Mutex<VecDeque<MCPMessage>>>,
    /// Messages that were sent via send()
    sent_messages: Arc<Mutex<Vec<MCPMessage>>>,
}

impl MockTransport {
    fn new() -> Self {
        Self {
            receive_queue: Arc::new(Mutex::new(VecDeque::new())),
            sent_messages: Arc::new(Mutex::new(Vec::new())),
        }
    }

    fn queue_response(&self, message: MCPMessage) {
        self.receive_queue.lock().unwrap().push_back(message);
    }

    #[allow(dead_code)]
    fn get_sent_messages(&self) -> Vec<MCPMessage> {
        self.sent_messages.lock().unwrap().clone()
    }
}

#[async_trait]
impl MCPTransport for MockTransport {
    async fn send(&mut self, message: &MCPMessage) -> Result<(), ArsenalError> {
        self.sent_messages.lock().unwrap().push(message.clone());
        Ok(())
    }

    async fn receive(&mut self) -> Result<MCPMessage, ArsenalError> {
        self.receive_queue
            .lock()
            .unwrap()
            .pop_front()
            .ok_or_else(|| ArsenalError::TransportError("No messages in queue".to_string()))
    }
}

#[test]
fn test_mcp_request_serialization() {
    let request = MCPRequest::new("tools/list", Some(json!({})));

    let serialized = serde_json::to_string(&request).unwrap();
    assert!(serialized.contains("\"jsonrpc\":\"2.0\""));
    assert!(serialized.contains("\"method\":\"tools/list\""));
    assert!(serialized.contains("\"id\""));
}

#[test]
fn test_mcp_request_with_params() {
    let params = json!({
        "name": "calculator",
        "arguments": {
            "operation": "add",
            "a": 5,
            "b": 3
        }
    });

    let request = MCPRequest::new("tools/call", Some(params.clone()));
    assert_eq!(request.method, "tools/call");
    assert_eq!(request.params, Some(params));
}

#[test]
fn test_mcp_response_deserialization() {
    let json_str = r#"{
        "jsonrpc": "2.0",
        "id": "test-123",
        "result": {"status": "success"}
    }"#;

    let response: MCPResponse = serde_json::from_str(json_str).unwrap();
    assert_eq!(response.jsonrpc, "2.0");
    assert_eq!(response.id, Value::String("test-123".to_string()));
    assert!(response.result.is_some());
    assert!(response.error.is_none());
}

#[test]
fn test_mcp_error_response() {
    let json_str = r#"{
        "jsonrpc": "2.0",
        "id": "test-456",
        "error": {
            "code": -32601,
            "message": "Method not found"
        }
    }"#;

    let response: MCPResponse = serde_json::from_str(json_str).unwrap();
    assert!(response.result.is_none());
    assert!(response.error.is_some());

    let error = response.error.unwrap();
    assert_eq!(error.code, MCPError::METHOD_NOT_FOUND);
    assert_eq!(error.message, "Method not found");
}

#[test]
fn test_mcp_error_handling() {
    let error = MCPError::new(MCPError::INVALID_PARAMS, "Missing required parameter");
    assert_eq!(error.code, -32602);
    assert_eq!(error.message, "Missing required parameter");
    assert!(error.data.is_none());

    let error_with_data = MCPError::with_data(
        MCPError::INTERNAL_ERROR,
        "Server error",
        json!({"detail": "Database connection failed"}),
    );
    assert_eq!(error_with_data.code, MCPError::INTERNAL_ERROR);
    assert!(error_with_data.data.is_some());
}

#[test]
fn test_mcp_notification_format() {
    let notification = MCPNotification {
        jsonrpc: "2.0".to_string(),
        method: "progress/update".to_string(),
        params: Some(json!({"progress": 50})),
    };

    let serialized = serde_json::to_string(&notification).unwrap();
    assert!(serialized.contains("\"method\":\"progress/update\""));
    assert!(!serialized.contains("\"id\"")); // Notifications have no ID
}

#[test]
fn test_mcp_capabilities_parsing() {
    let json_str = r#"{
        "server_info": {
            "name": "test-server",
            "version": "1.0.0"
        },
        "tools": [
            {
                "name": "calculator",
                "description": "Basic calculator",
                "inputSchema": {
                    "type": "object",
                    "properties": {
                        "operation": {"type": "string"}
                    },
                    "required": ["operation"]
                }
            }
        ]
    }"#;

    let capabilities: paladin::infrastructure::adapters::arsenal::mcp_protocol::MCPCapabilities =
        serde_json::from_str(json_str).unwrap();

    assert!(capabilities.server_info.is_some());
    let server_info = capabilities.server_info.unwrap();
    assert_eq!(server_info.name, "test-server");
    assert_eq!(server_info.version, "1.0.0");

    assert_eq!(capabilities.tools.len(), 1);
    assert_eq!(capabilities.tools[0].name, "calculator");
}

#[test]
fn test_tool_info_parsing() {
    let json_str = r#"{
        "name": "web_search",
        "description": "Search the web",
        "inputSchema": {
            "type": "object",
            "properties": {
                "query": {"type": "string"}
            },
            "required": ["query"]
        }
    }"#;

    let tool: ToolInfo = serde_json::from_str(json_str).unwrap();
    assert_eq!(tool.name, "web_search");
    assert_eq!(tool.description, "Search the web");

    let schema = tool.input_schema;
    assert!(schema.get("properties").is_some());
    assert!(schema.get("required").is_some());
}

#[tokio::test]
async fn test_mcp_client_discover_tools() {
    let mock = MockTransport::new();

    // Queue a mock response for tools/list
    let response = MCPResponse {
        jsonrpc: "2.0".to_string(),
        id: Value::String("test".to_string()),
        result: Some(json!({
            "tools": [
                {
                    "name": "calculator",
                    "description": "Basic calculator",
                    "inputSchema": {
                        "type": "object",
                        "properties": {
                            "operation": {"type": "string"}
                        },
                        "required": ["operation"]
                    }
                }
            ]
        })),
        error: None,
    };
    mock.queue_response(MCPMessage::Response(response));

    let client = MCPClient::new(Box::new(mock));
    let tools = client.discover_tools().await.unwrap();

    assert_eq!(tools.len(), 1);
    assert_eq!(tools[0].name, "calculator");
    assert_eq!(tools[0].description, "Basic calculator");
    assert_eq!(tools[0].required_params, vec!["operation"]);
}

#[tokio::test]
async fn test_mcp_client_invoke_tool() {
    let mock = MockTransport::new();

    // Queue a mock response for tools/call
    let response = MCPResponse {
        jsonrpc: "2.0".to_string(),
        id: Value::String("test".to_string()),
        result: Some(json!({
            "content": {
                "type": "text",
                "text": "Result: 8"
            }
        })),
        error: None,
    };
    mock.queue_response(MCPMessage::Response(response));

    let client = MCPClient::new(Box::new(mock));

    let mut args = std::collections::HashMap::new();
    args.insert("operation".to_string(), json!("add"));
    args.insert("a".to_string(), json!(5));
    args.insert("b".to_string(), json!(3));

    let result = client.invoke_tool("calculator", args).await.unwrap();
    assert!(result.get("type").is_some());
}

#[tokio::test]
async fn test_mcp_client_error_response() {
    let mock = MockTransport::new();

    // Queue an error response
    let response = MCPResponse {
        jsonrpc: "2.0".to_string(),
        id: Value::String("test".to_string()),
        result: None,
        error: Some(MCPError::new(MCPError::METHOD_NOT_FOUND, "Tool not found")),
    };
    mock.queue_response(MCPMessage::Response(response));

    let client = MCPClient::new(Box::new(mock));

    let args = std::collections::HashMap::new();
    let result = client.invoke_tool("nonexistent", args).await;

    assert!(result.is_err());
    match result {
        Err(ArsenalError::ProtocolError(msg)) => {
            assert!(msg.contains("Tool not found"));
        }
        _ => panic!("Expected ProtocolError"),
    }
}

#[test]
fn test_mcp_message_enum_serialization() {
    // Test Request variant
    let request = MCPRequest::new("test", None);
    let msg = MCPMessage::Request(request);
    let serialized = serde_json::to_string(&msg).unwrap();
    assert!(serialized.contains("\"method\":\"test\""));

    // Test Response variant
    let response = MCPResponse {
        jsonrpc: "2.0".to_string(),
        id: Value::Null,
        result: Some(json!({"ok": true})),
        error: None,
    };
    let msg = MCPMessage::Response(response);
    let serialized = serde_json::to_string(&msg).unwrap();
    assert!(serialized.contains("\"result\""));
}
