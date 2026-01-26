//! JSON Herald formatter implementation
//!
//! The JsonHerald provides JSON serialization for Paladin and Battalion execution results.
//! It supports both pretty-printed and compact JSON output, with configurable metadata inclusion.
//!
//! # Examples
//!
//! ```rust,ignore
//! use paladin::infrastructure::adapters::herald::JsonHerald;
//! use paladin::core::platform::container::herald::Herald;
//!
//! let herald = JsonHerald::new();
//! let formatted = herald.format_paladin_result(&result)?;
//! println!("{}", formatted);
//! ```

use crate::core::platform::container::herald::{
    BattalionResult, ExecutionMetadata, Herald, HeraldError, PaladinError, PaladinResult,
    StreamChunk,
};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

/// Configuration for JSON Herald formatter
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonHeraldConfig {
    /// Enable pretty-printing (formatted with indentation)
    pub pretty: bool,
    /// Include metadata fields in output
    pub include_metadata: bool,
}

impl Default for JsonHeraldConfig {
    fn default() -> Self {
        Self {
            pretty: true,
            include_metadata: true,
        }
    }
}

/// JSON formatter for Paladin execution results
///
/// The JsonHerald converts Paladin and Battalion results into JSON format,
/// making them suitable for API responses, logging systems, and programmatic
/// consumption.
///
/// # Thread Safety
///
/// JsonHerald is thread-safe and can be shared across threads using `Arc`.
///
/// # Configuration
///
/// - **pretty**: When true, outputs formatted JSON with indentation
/// - **include_metadata**: When true, includes execution metadata in output
///
/// # Examples
///
/// ```rust,ignore
/// // Create with default configuration (pretty: true, include_metadata: true)
/// let herald = JsonHerald::new();
///
/// // Create with custom configuration
/// let config = JsonHeraldConfig {
///     pretty: false,
///     include_metadata: true,
/// };
/// let herald = JsonHerald::with_config(config);
/// ```
#[derive(Debug, Clone)]
pub struct JsonHerald {
    config: JsonHeraldConfig,
}

impl JsonHerald {
    /// Create a new JsonHerald with default configuration
    ///
    /// Default configuration:
    /// - pretty: true (formatted output)
    /// - include_metadata: true (all metadata included)
    pub fn new() -> Self {
        Self {
            config: JsonHeraldConfig::default(),
        }
    }

    /// Create a new JsonHerald with custom configuration
    ///
    /// # Arguments
    ///
    /// * `config` - Configuration for the JSON formatter
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// let config = JsonHeraldConfig {
    ///     pretty: false,
    ///     include_metadata: false,
    /// };
    /// let herald = JsonHerald::with_config(config);
    /// ```
    pub fn with_config(config: JsonHeraldConfig) -> Self {
        Self { config }
    }

    /// Serialize value to JSON string based on configuration
    fn serialize<T: Serialize>(&self, value: &T) -> Result<String, HeraldError> {
        if self.config.pretty {
            serde_json::to_string_pretty(value)
        } else {
            serde_json::to_string(value)
        }
        .map_err(|e| HeraldError::SerializationError(format!("JSON serialization failed: {}", e)))
    }

    /// Convert PaladinResult to JSON Value
    fn paladin_result_to_json(&self, result: &PaladinResult) -> Value {
        let mut json = json!({
            "paladin_id": result.paladin_id,
            "paladin_name": result.paladin_name,
            "status": result.status,
            "output": result.output,
        });

        if self.config.include_metadata {
            json["metadata"] = json!({
                "timestamp": chrono::Utc::now().to_rfc3339(),
            });
        }

        json
    }

    /// Convert BattalionResult to JSON Value
    fn battalion_result_to_json(&self, result: &BattalionResult) -> Value {
        let paladin_results: Vec<Value> = result
            .results
            .iter()
            .map(|r| self.paladin_result_to_json(r))
            .collect();

        let mut json = json!({
            "battalion_id": result.battalion_id,
            "battalion_name": result.battalion_name,
            "status": result.status,
            "paladin_results": paladin_results,
        });

        if self.config.include_metadata {
            json["metadata"] = json!({
                "paladin_count": result.results.len(),
                "timestamp": chrono::Utc::now().to_rfc3339(),
            });
        }

        json
    }
}

impl Default for JsonHerald {
    fn default() -> Self {
        Self::new()
    }
}

impl Herald for JsonHerald {
    fn format_paladin_result(&self, result: &PaladinResult) -> Result<String, HeraldError> {
        let json = self.paladin_result_to_json(result);
        self.serialize(&json)
    }

    fn format_battalion_result(&self, result: &BattalionResult) -> Result<String, HeraldError> {
        let json = self.battalion_result_to_json(result);
        self.serialize(&json)
    }

    fn format_stream_chunk(&self, chunk: &StreamChunk) -> Result<Option<String>, HeraldError> {
        // For JSON streaming, we use NDJSON (newline-delimited JSON) format
        // Each chunk is emitted as a separate JSON object on its own line
        let json = json!({
            "content": chunk.content,
            "is_final": chunk.is_final,
            "timestamp": chrono::Utc::now().to_rfc3339(),
        });

        // Use compact serialization for streaming (pretty printing not suitable for NDJSON)
        let serialized = serde_json::to_string(&json).map_err(|e| {
            HeraldError::SerializationError(format!("Stream chunk serialization failed: {}", e))
        })?;

        Ok(Some(format!("{}\n", serialized)))
    }

    fn finalize_stream(&self, metadata: &ExecutionMetadata) -> Result<String, HeraldError> {
        let json = json!({
            "type": "metadata",
            "execution_time_ms": metadata.execution_time_ms,
            "total_tokens": metadata.total_tokens,
            "timestamp": chrono::Utc::now().to_rfc3339(),
        });

        let serialized = serde_json::to_string(&json).map_err(|e| {
            HeraldError::SerializationError(format!("Metadata serialization failed: {}", e))
        })?;

        Ok(format!("{}\n", serialized))
    }

    fn format_error(&self, error: &PaladinError) -> String {
        let json = json!({
            "error": true,
            "message": error.message,
            "timestamp": chrono::Utc::now().to_rfc3339(),
        });

        // Best-effort serialization, never fail
        serde_json::to_string_pretty(&json)
            .unwrap_or_else(|_| format!(r#"{{"error": true, "message": "{}"}}"#, error.message))
    }

    fn name(&self) -> &str {
        "json"
    }

    fn mime_type(&self) -> &str {
        "application/json"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_paladin_result() -> PaladinResult {
        PaladinResult {
            paladin_id: "test-id-123".to_string(),
            paladin_name: "TestPaladin".to_string(),
            status: "success".to_string(),
            output: "Test output content".to_string(),
        }
    }

    fn create_test_battalion_result() -> BattalionResult {
        BattalionResult {
            battalion_id: "bat-id-456".to_string(),
            battalion_name: "TestBattalion".to_string(),
            status: "success".to_string(),
            results: vec![
                create_test_paladin_result(),
                PaladinResult {
                    paladin_id: "test-id-789".to_string(),
                    paladin_name: "SecondPaladin".to_string(),
                    status: "success".to_string(),
                    output: "Second output".to_string(),
                },
            ],
        }
    }

    #[test]
    fn test_new_creates_default_config() {
        let herald = JsonHerald::new();
        assert!(herald.config.pretty);
        assert!(herald.config.include_metadata);
    }

    #[test]
    fn test_with_config_uses_custom_config() {
        let config = JsonHeraldConfig {
            pretty: false,
            include_metadata: false,
        };
        let herald = JsonHerald::with_config(config);
        assert!(!herald.config.pretty);
        assert!(!herald.config.include_metadata);
    }

    #[test]
    fn test_format_paladin_result_success() {
        let herald = JsonHerald::new();
        let result = create_test_paladin_result();

        let formatted = herald.format_paladin_result(&result).unwrap();

        // Verify it's valid JSON
        let parsed: Value = serde_json::from_str(&formatted).unwrap();
        assert_eq!(parsed["paladin_id"], "test-id-123");
        assert_eq!(parsed["paladin_name"], "TestPaladin");
        assert_eq!(parsed["status"], "success");
        assert_eq!(parsed["output"], "Test output content");
    }

    #[test]
    fn test_format_paladin_result_includes_metadata() {
        let herald = JsonHerald::new();
        let result = create_test_paladin_result();

        let formatted = herald.format_paladin_result(&result).unwrap();
        let parsed: Value = serde_json::from_str(&formatted).unwrap();

        assert!(parsed["metadata"].is_object());
        assert!(parsed["metadata"]["timestamp"].is_string());
    }

    #[test]
    fn test_format_paladin_result_without_metadata() {
        let config = JsonHeraldConfig {
            pretty: false,
            include_metadata: false,
        };
        let herald = JsonHerald::with_config(config);
        let result = create_test_paladin_result();

        let formatted = herald.format_paladin_result(&result).unwrap();
        let parsed: Value = serde_json::from_str(&formatted).unwrap();

        assert!(parsed["metadata"].is_null());
    }

    #[test]
    fn test_format_battalion_result_success() {
        let herald = JsonHerald::new();
        let result = create_test_battalion_result();

        let formatted = herald.format_battalion_result(&result).unwrap();

        let parsed: Value = serde_json::from_str(&formatted).unwrap();
        assert_eq!(parsed["battalion_id"], "bat-id-456");
        assert_eq!(parsed["battalion_name"], "TestBattalion");
        assert_eq!(parsed["status"], "success");
        assert!(parsed["paladin_results"].is_array());
        assert_eq!(parsed["paladin_results"].as_array().unwrap().len(), 2);
    }

    #[test]
    fn test_format_battalion_result_includes_metadata() {
        let herald = JsonHerald::new();
        let result = create_test_battalion_result();

        let formatted = herald.format_battalion_result(&result).unwrap();
        let parsed: Value = serde_json::from_str(&formatted).unwrap();

        assert!(parsed["metadata"].is_object());
        assert_eq!(parsed["metadata"]["paladin_count"], 2);
    }

    #[test]
    fn test_format_stream_chunk_ndjson() {
        let herald = JsonHerald::new();
        let chunk = StreamChunk {
            content: "partial content".to_string(),
            is_final: false,
        };

        let formatted = herald.format_stream_chunk(&chunk).unwrap();
        assert!(formatted.is_some());

        let output = formatted.unwrap();
        assert!(output.ends_with('\n'));

        // Remove trailing newline and parse
        let json_str = output.trim_end();
        let parsed: Value = serde_json::from_str(json_str).unwrap();
        assert_eq!(parsed["content"], "partial content");
        assert_eq!(parsed["is_final"], false);
    }

    #[test]
    fn test_finalize_stream() {
        let herald = JsonHerald::new();
        let metadata = ExecutionMetadata {
            execution_time_ms: 1234,
            total_tokens: 500,
        };

        let formatted = herald.finalize_stream(&metadata).unwrap();
        assert!(formatted.ends_with('\n'));

        let json_str = formatted.trim_end();
        let parsed: Value = serde_json::from_str(json_str).unwrap();
        assert_eq!(parsed["type"], "metadata");
        assert_eq!(parsed["execution_time_ms"], 1234);
        assert_eq!(parsed["total_tokens"], 500);
    }

    #[test]
    fn test_format_error() {
        let herald = JsonHerald::new();
        let error = PaladinError {
            message: "Something went wrong".to_string(),
        };

        let formatted = herald.format_error(&error);

        let parsed: Value = serde_json::from_str(&formatted).unwrap();
        assert_eq!(parsed["error"], true);
        assert_eq!(parsed["message"], "Something went wrong");
    }

    #[test]
    fn test_name() {
        let herald = JsonHerald::new();
        assert_eq!(herald.name(), "json");
    }

    #[test]
    fn test_mime_type() {
        let herald = JsonHerald::new();
        assert_eq!(herald.mime_type(), "application/json");
    }

    #[test]
    fn test_pretty_vs_compact_output() {
        let result = create_test_paladin_result();

        let pretty_herald = JsonHerald::with_config(JsonHeraldConfig {
            pretty: true,
            include_metadata: false,
        });
        let pretty_output = pretty_herald.format_paladin_result(&result).unwrap();

        let compact_herald = JsonHerald::with_config(JsonHeraldConfig {
            pretty: false,
            include_metadata: false,
        });
        let compact_output = compact_herald.format_paladin_result(&result).unwrap();

        // Pretty output should have more characters due to whitespace
        assert!(pretty_output.len() > compact_output.len());

        // Both should parse to the same data
        let pretty_parsed: Value = serde_json::from_str(&pretty_output).unwrap();
        let compact_parsed: Value = serde_json::from_str(&compact_output).unwrap();
        assert_eq!(pretty_parsed["paladin_id"], compact_parsed["paladin_id"]);
    }

    #[test]
    fn test_roundtrip_paladin_result() {
        let herald = JsonHerald::new();
        let original = create_test_paladin_result();

        // Format to JSON
        let formatted = herald.format_paladin_result(&original).unwrap();

        // Parse back
        let parsed: Value = serde_json::from_str(&formatted).unwrap();

        // Verify key fields match
        assert_eq!(parsed["paladin_id"].as_str().unwrap(), original.paladin_id);
        assert_eq!(
            parsed["paladin_name"].as_str().unwrap(),
            original.paladin_name
        );
        assert_eq!(parsed["status"].as_str().unwrap(), original.status);
        assert_eq!(parsed["output"].as_str().unwrap(), original.output);
    }

    #[test]
    fn test_roundtrip_battalion_result() {
        let herald = JsonHerald::new();
        let original = create_test_battalion_result();

        // Format to JSON
        let formatted = herald.format_battalion_result(&original).unwrap();

        // Parse back
        let parsed: Value = serde_json::from_str(&formatted).unwrap();

        // Verify key fields match
        assert_eq!(
            parsed["battalion_id"].as_str().unwrap(),
            original.battalion_id
        );
        assert_eq!(
            parsed["battalion_name"].as_str().unwrap(),
            original.battalion_name
        );
        assert_eq!(parsed["paladin_results"].as_array().unwrap().len(), 2);
    }

    #[test]
    fn test_herald_is_send_sync() {
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<JsonHerald>();
    }

    #[test]
    fn test_default_trait() {
        let herald = JsonHerald::default();
        assert!(herald.config.pretty);
        assert!(herald.config.include_metadata);
    }
}
