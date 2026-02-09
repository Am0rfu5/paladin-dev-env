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

// Re-export actual domain types for Herald consumers
pub use crate::application::ports::output::paladin_port::PaladinResult;
pub use crate::application::use_cases::paladin::error::PaladinError;
pub use crate::core::platform::container::battalion::BattalionResult;

// Re-export HeraldError for convenience
pub use super::herald_error::HeraldError;

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

/// Streaming chunk of output
///
/// TODO: Define complete StreamChunk structure with full metadata
#[derive(Debug, Clone)]
pub struct StreamChunk {
    pub content: String,
    pub is_final: bool,
}

/// Execution metadata for streaming
///
/// TODO: Define complete ExecutionMetadata structure with full telemetry
#[derive(Debug, Clone)]
pub struct ExecutionMetadata {
    pub execution_time_ms: u64,
    pub total_tokens: u32,
}

#[cfg(test)]
mod tests {
    use super::*;

    // Mock Herald implementation for testing
    struct MockHerald;

    impl Herald for MockHerald {
        fn format_paladin_result(&self, result: &PaladinResult) -> Result<String, HeraldError> {
            Ok(format!("MOCK: {}", result.output))
        }

        fn format_battalion_result(&self, result: &BattalionResult) -> Result<String, HeraldError> {
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
            format!("ERROR: {}", error)
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
        use crate::application::ports::output::paladin_port::StopReason;
        let herald = MockHerald;
        let result = PaladinResult {
            output: "Test output".to_string(),
            token_count: 100,
            execution_time_ms: 1500,
            loop_count: 1,
            stop_reason: StopReason::Completed,
        };

        let formatted = herald.format_paladin_result(&result).unwrap();
        assert_eq!(formatted, "MOCK: Test output");
    }

    #[test]
    fn test_format_battalion_result() {
        use crate::core::platform::container::battalion::{BattalionStatus, BattalionStrategy};
        use chrono::Utc;
        use uuid::Uuid;
        let herald = MockHerald;
        let result = BattalionResult {
            battalion_id: Uuid::new_v4(),
            battalion_name: "TestBattalion".to_string(),
            started_at: Utc::now(),
            completed_at: Utc::now(),
            final_output: "Combined output".to_string(),
            paladin_results: vec![],
            status: BattalionStatus::Completed,
            strategy_used: BattalionStrategy::Formation,
            strategy_selection_reasoning: None,
            strategy_selection_time_ms: 0,
            per_paladin_times: vec![],
            paladin_success_count: 0,
            paladin_failure_count: 0,
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
        let error = PaladinError::ExecutionError("Something went wrong".to_string());

        let formatted = herald.format_error(&error);
        assert!(formatted.contains("ERROR"));
        assert!(formatted.contains("Something went wrong"));
    }

    #[test]
    fn test_herald_is_send_sync() {
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<MockHerald>();
    }
}
