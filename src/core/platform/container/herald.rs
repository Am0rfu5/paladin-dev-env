//! Herald trait for formatting Paladin execution results
//!
//! The Herald system provides a flexible, extensible mechanism for formatting
//! Paladin and Battalion execution results into multiple output formats (JSON,
//! Markdown, tables, etc.). Heralds enable consistent output formatting across
//! CLI tools, APIs, logging systems, and external integrations.
//!
//! # Examples
//!
//! ```rust,ignore
//! use paladin::core::platform::container::herald::Herald;
//! use paladin::infrastructure::adapters::herald::JsonHerald;
//!
//! let herald = JsonHerald::new();
//! let formatted = herald.format_paladin_result(&result)?;
//! println!("{}", formatted);
//! ```

use super::herald_error::HeraldError;

/// Herald trait for formatting Paladin execution results
///
/// This trait defines the contract for all output formatters in the Paladin system.
/// Implementors transform execution results into specific formats (JSON, Markdown, etc.)
/// while preserving all metadata and supporting both complete and streaming output modes.
///
/// # Thread Safety
///
/// All Herald implementations must be `Send + Sync` to support async execution
/// and concurrent access across multiple threads.
///
/// # Output Modes
///
/// Heralds support two output modes:
/// - **Complete Mode**: Format entire results after execution completes
/// - **Streaming Mode**: Format output progressively as it arrives
pub trait Herald: Send + Sync {
    /// Format a complete Paladin execution result
    ///
    /// This method takes a completed `PaladinResult` and formats it according
    /// to the Herald's output format. All metadata (tokens, timing, errors) must
    /// be included in the formatted output.
    ///
    /// # Arguments
    ///
    /// * `result` - The Paladin execution result to format
    ///
    /// # Returns
    ///
    /// Returns formatted output as a `String` or a `HeraldError` if formatting fails.
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// let formatted = herald.format_paladin_result(&result)?;
    /// println!("{}", formatted);
    /// ```
    fn format_paladin_result(&self, result: &PaladinResult) -> Result<String, HeraldError>;

    /// Format a complete Battalion execution result
    ///
    /// This method formats the aggregated results from a Battalion execution,
    /// including individual Paladin outputs, execution order/parallelism, and
    /// aggregated metadata.
    ///
    /// # Arguments
    ///
    /// * `result` - The Battalion execution result to format
    ///
    /// # Returns
    ///
    /// Returns formatted output as a `String` or a `HeraldError` if formatting fails.
    fn format_battalion_result(&self, result: &BattalionResult) -> Result<String, HeraldError>;

    /// Format a streaming chunk of output
    ///
    /// This method formats partial output as it arrives during streaming execution.
    /// The formatter may buffer chunks or return formatted output immediately,
    /// depending on the format requirements.
    ///
    /// # Arguments
    ///
    /// * `chunk` - A partial output chunk from streaming execution
    ///
    /// # Returns
    ///
    /// Returns `Some(String)` with formatted output if ready, `None` if buffering,
    /// or a `HeraldError` if formatting fails.
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// while let Some(chunk) = stream.next().await {
    ///     if let Some(formatted) = herald.format_stream_chunk(&chunk)? {
    ///         print!("{}", formatted);
    ///     }
    /// }
    /// ```
    fn format_stream_chunk(&self, chunk: &StreamChunk) -> Result<Option<String>, HeraldError>;

    /// Finalize streaming output with metadata
    ///
    /// This method is called after streaming completes to append final metadata
    /// (execution time, token counts, etc.) to the formatted output.
    ///
    /// # Arguments
    ///
    /// * `metadata` - Execution metadata from the completed stream
    ///
    /// # Returns
    ///
    /// Returns formatted metadata as a `String` or a `HeraldError` if formatting fails.
    fn finalize_stream(&self, metadata: &ExecutionMetadata) -> Result<String, HeraldError>;

    /// Format an error for display
    ///
    /// This method formats Paladin errors in a way consistent with the Herald's
    /// output format. Unlike other methods, this never returns an error - it
    /// provides a best-effort formatted representation.
    ///
    /// # Arguments
    ///
    /// * `error` - The Paladin error to format
    ///
    /// # Returns
    ///
    /// Returns formatted error as a `String`.
    fn format_error(&self, error: &PaladinError) -> String;

    /// Get the formatter name/identifier
    ///
    /// Returns a unique identifier for this formatter (e.g., "json", "markdown", "table").
    /// Used for formatter registration and selection.
    fn name(&self) -> &str;

    /// Get the formatter's MIME type
    ///
    /// Returns the MIME type for the formatted output (e.g., "application/json",
    /// "text/markdown", "text/plain"). Useful for HTTP responses and content
    /// negotiation.
    fn mime_type(&self) -> &str;
}

/// Placeholder for PaladinResult (will be defined in Epic 1)
///
/// TODO: Replace with actual PaladinResult from Epic 1 implementation
#[derive(Debug, Clone)]
pub struct PaladinResult {
    pub paladin_id: String,
    pub paladin_name: String,
    pub status: String,
    pub output: String,
}

/// Placeholder for BattalionResult (will be defined in Epic 4)
///
/// TODO: Replace with actual BattalionResult from Epic 4 implementation
#[derive(Debug, Clone)]
pub struct BattalionResult {
    pub battalion_id: String,
    pub battalion_name: String,
    pub status: String,
    pub results: Vec<PaladinResult>,
}

/// Placeholder for StreamChunk (streaming support)
///
/// TODO: Define complete StreamChunk structure
#[derive(Debug, Clone)]
pub struct StreamChunk {
    pub content: String,
    pub is_final: bool,
}

/// Placeholder for ExecutionMetadata (streaming support)
///
/// TODO: Define complete ExecutionMetadata structure
#[derive(Debug, Clone)]
pub struct ExecutionMetadata {
    pub execution_time_ms: u64,
    pub total_tokens: u32,
}

/// Placeholder for PaladinError (will be defined in Epic 1)
///
/// TODO: Replace with actual PaladinError from Epic 1 implementation
#[derive(Debug)]
pub struct PaladinError {
    pub message: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    // Mock Herald implementation for testing
    struct MockHerald;

    impl Herald for MockHerald {
        fn format_paladin_result(&self, result: &PaladinResult) -> Result<String, HeraldError> {
            Ok(format!("MOCK: {}", result.paladin_name))
        }

        fn format_battalion_result(
            &self,
            result: &BattalionResult,
        ) -> Result<String, HeraldError> {
            Ok(format!("MOCK BATTALION: {}", result.battalion_name))
        }

        fn format_stream_chunk(&self, chunk: &StreamChunk) -> Result<Option<String>, HeraldError> {
            if chunk.is_final {
                Ok(Some(chunk.content.clone()))
            } else {
                Ok(None)
            }
        }

        fn finalize_stream(&self, metadata: &ExecutionMetadata) -> Result<String, HeraldError> {
            Ok(format!("Execution time: {}ms", metadata.execution_time_ms))
        }

        fn format_error(&self, error: &PaladinError) -> String {
            format!("ERROR: {}", error.message)
        }

        fn name(&self) -> &str {
            "mock"
        }

        fn mime_type(&self) -> &str {
            "text/plain"
        }
    }

    #[test]
    fn test_herald_trait_object() {
        let herald: Box<dyn Herald> = Box::new(MockHerald);
        assert_eq!(herald.name(), "mock");
        assert_eq!(herald.mime_type(), "text/plain");
    }

    #[test]
    fn test_format_paladin_result() {
        let herald = MockHerald;
        let result = PaladinResult {
            paladin_id: "test-id".to_string(),
            paladin_name: "TestPaladin".to_string(),
            status: "success".to_string(),
            output: "Test output".to_string(),
        };

        let formatted = herald.format_paladin_result(&result).unwrap();
        assert_eq!(formatted, "MOCK: TestPaladin");
    }

    #[test]
    fn test_format_battalion_result() {
        let herald = MockHerald;
        let result = BattalionResult {
            battalion_id: "bat-id".to_string(),
            battalion_name: "TestBattalion".to_string(),
            status: "success".to_string(),
            results: vec![],
        };

        let formatted = herald.format_battalion_result(&result).unwrap();
        assert_eq!(formatted, "MOCK BATTALION: TestBattalion");
    }

    #[test]
    fn test_format_stream_chunk() {
        let herald = MockHerald;
        let chunk = StreamChunk {
            content: "partial output".to_string(),
            is_final: false,
        };

        let result = herald.format_stream_chunk(&chunk).unwrap();
        assert!(result.is_none());

        let final_chunk = StreamChunk {
            content: "final output".to_string(),
            is_final: true,
        };

        let result = herald.format_stream_chunk(&final_chunk).unwrap();
        assert_eq!(result, Some("final output".to_string()));
    }

    #[test]
    fn test_finalize_stream() {
        let herald = MockHerald;
        let metadata = ExecutionMetadata {
            execution_time_ms: 1234,
            total_tokens: 500,
        };

        let formatted = herald.finalize_stream(&metadata).unwrap();
        assert_eq!(formatted, "Execution time: 1234ms");
    }

    #[test]
    fn test_format_error() {
        let herald = MockHerald;
        let error = PaladinError {
            message: "Something went wrong".to_string(),
        };

        let formatted = herald.format_error(&error);
        assert_eq!(formatted, "ERROR: Something went wrong");
    }

    #[test]
    fn test_herald_is_send_sync() {
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<MockHerald>();
    }
}
