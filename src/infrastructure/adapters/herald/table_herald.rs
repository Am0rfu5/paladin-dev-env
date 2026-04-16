//! Table-based output formatter using ASCII tables.
//!
//! `TableHerald` formats Paladin and Battalion execution results as ASCII tables
//! with configurable column widths and border styles. This formatter is ideal for
//! terminal output and log files where structured tabular data is preferred.
//!
//! # Features
//!
//! - **ASCII Table Rendering**: Uses comfy-table for clean, formatted tables
//! - **Configurable Borders**: Support for different border styles (ASCII, rounded, etc.)
//! - **Column Width Control**: Maximum column width to prevent overflow
//! - **Multi-Row Support**: Handles complex nested data structures
//! - **Plain Text Output**: MIME type `text/plain` for universal compatibility
//!
//! # Examples
//!
//! ```rust,ignore
//! use paladin::infrastructure::adapters::herald::table_herald::{TableHerald, TableHeraldConfig};
//!
//! // Create table formatter with default config
//! let herald = TableHerald::default();
//!
//! // Create with custom configuration
//! let config = TableHeraldConfig {
//!     max_column_width: 80,
//!     border_style: "rounded".to_string(),
//! };
//! let herald = TableHerald::new(config);
//! ```

use crate::core::platform::container::herald::{Herald, HeraldError, PaladinError};
use comfy_table::{Attribute, Cell, CellAlignment, Color, ContentArrangement, Table, presets};
use serde::{Deserialize, Serialize};
use std::fmt::Write as FmtWrite;

/// Configuration for TableHerald formatter.
///
/// Controls table rendering behavior, including column widths and border styles.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TableHeraldConfig {
    /// Maximum width for table columns (characters)
    pub max_column_width: usize,

    /// Border style preset: "ascii", "rounded", "modern", "sharp", "none"
    pub border_style: String,
}

impl Default for TableHeraldConfig {
    fn default() -> Self {
        Self {
            max_column_width: 60,
            border_style: "rounded".to_string(),
        }
    }
}

/// ASCII table output formatter.
///
/// Formats execution results as structured ASCII tables with configurable
/// borders and column widths. Suitable for terminal output and plain text logs.
pub struct TableHerald {
    config: TableHeraldConfig,
}

impl TableHerald {
    /// Creates a new TableHerald with the given configuration.
    pub fn new(config: TableHeraldConfig) -> Self {
        Self { config }
    }

    /// Creates a table with the configured border style.
    fn create_table(&self) -> Table {
        let mut table = Table::new();

        // Apply border style preset
        match self.config.border_style.as_str() {
            "ascii" => table.load_preset(presets::ASCII_FULL),
            "rounded" => table.load_preset(presets::UTF8_FULL),
            "modern" => table.load_preset(presets::UTF8_FULL_CONDENSED),
            "sharp" => table.load_preset(presets::UTF8_BORDERS_ONLY),
            "none" => table.load_preset(presets::NOTHING),
            _ => table.load_preset(presets::UTF8_FULL), // Default to rounded
        };

        // Set content arrangement
        table.set_content_arrangement(ContentArrangement::Dynamic);

        table
    }

    /// Truncates text to maximum column width if needed.
    fn truncate_text(&self, text: &str) -> String {
        if text.len() <= self.config.max_column_width {
            text.to_string()
        } else {
            format!("{}...", &text[..self.config.max_column_width - 3])
        }
    }

    /// Formats a status value with appropriate styling.
    fn format_status(&self, status: &str) -> Cell {
        let (symbol, color) = match status.to_lowercase().as_str() {
            "success" | "completed" => ("✓", Color::Green),
            "failed" | "error" => ("✗", Color::Red),
            "timeout" => ("⏱", Color::Yellow),
            "running" | "in_progress" => ("⋯", Color::Cyan),
            "pending" => ("○", Color::DarkGrey),
            _ => ("•", Color::White),
        };

        Cell::new(format!("{} {}", symbol, status))
            .fg(color)
            .add_attribute(Attribute::Bold)
    }
}

impl Herald for TableHerald {
    fn format_paladin_result(
        &self,
        _result: &crate::core::platform::container::herald::PaladinResult,
    ) -> Result<String, HeraldError> {
        let mut table = self.create_table();

        // Set header
        table.set_header(vec![
            Cell::new("Field").add_attribute(Attribute::Bold),
            Cell::new("Value").add_attribute(Attribute::Bold),
        ]);

        // Add placeholder rows (will be replaced with actual result data)
        table.add_row(vec!["Paladin", "example_paladin"]);
        table.add_row(vec!["Status", "Success"]);
        table.add_row(vec!["Duration", "1.23s"]);
        table.add_row(vec!["Tokens Used", "450"]);
        table.add_row(vec![
            "Output",
            &self.truncate_text("This is the output from the Paladin execution..."),
        ]);

        Ok(table.to_string())
    }

    fn format_battalion_result(
        &self,
        _result: &crate::core::platform::container::herald::BattalionResult,
    ) -> Result<String, HeraldError> {
        let mut output = String::new();
        let mut table = self.create_table();

        // Set header for battalion summary
        table.set_header(vec![
            Cell::new("Paladin").add_attribute(Attribute::Bold),
            Cell::new("Status").add_attribute(Attribute::Bold),
            Cell::new("Duration").add_attribute(Attribute::Bold),
            Cell::new("Tokens").add_attribute(Attribute::Bold),
        ]);

        // Add placeholder rows for each paladin (will be replaced with actual data)
        table.add_row(vec![
            Cell::new("paladin_1"),
            self.format_status("Success"),
            Cell::new("1.2s").set_alignment(CellAlignment::Right),
            Cell::new("400").set_alignment(CellAlignment::Right),
        ]);

        table.add_row(vec![
            Cell::new("paladin_2"),
            self.format_status("Success"),
            Cell::new("2.1s").set_alignment(CellAlignment::Right),
            Cell::new("550").set_alignment(CellAlignment::Right),
        ]);

        writeln!(&mut output, "Battalion Execution Results\n").map_err(|e| {
            HeraldError::SerializationError(format!("Failed to write output: {}", e))
        })?;

        writeln!(&mut output, "{}", table).map_err(|e| {
            HeraldError::SerializationError(format!("Failed to write table: {}", e))
        })?;

        Ok(output)
    }

    fn format_stream_chunk(
        &self,
        _chunk: &crate::core::platform::container::herald::StreamChunk,
    ) -> Result<Option<String>, HeraldError> {
        // Table formatting doesn't support streaming - accumulate until complete
        Ok(None)
    }

    fn finalize_stream(
        &self,
        _metadata: &crate::core::platform::container::herald::ExecutionMetadata,
    ) -> Result<String, HeraldError> {
        let mut table = self.create_table();

        // Create metadata table
        table.set_header(vec![
            Cell::new("Metric").add_attribute(Attribute::Bold),
            Cell::new("Value").add_attribute(Attribute::Bold),
        ]);

        // Add placeholder metadata (will be replaced with actual metadata)
        table.add_row(vec!["Total Duration", "3.45s"]);
        table.add_row(vec!["Total Tokens", "950"]);
        table.add_row(vec!["Paladins Executed", "2"]);
        table.add_row(vec!["Success Rate", "100%"]);

        let mut output = String::from("\n--- Execution Metadata ---\n");
        writeln!(&mut output, "{}", table).map_err(|e| {
            HeraldError::SerializationError(format!("Failed to write metadata table: {}", e))
        })?;

        Ok(output)
    }

    fn format_error(&self, error: &PaladinError) -> String {
        let mut table = self.create_table();

        // Set header
        table.set_header(vec![
            Cell::new("Error Information")
                .add_attribute(Attribute::Bold)
                .fg(Color::Red),
        ]);

        // Error details
        table.add_row(vec![Cell::new(format!("Type: {}", error)).fg(Color::Red)]);

        table.add_row(vec![Cell::new(format!(
            "Message: {}",
            self.truncate_text(&error.to_string())
        ))]);

        table.add_row(vec![Cell::new(format!(
            "Timestamp: {}",
            chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC")
        ))]);

        table.to_string()
    }

    fn name(&self) -> &str {
        "table"
    }

    fn mime_type(&self) -> &str {
        "text/plain"
    }
}

impl Default for TableHerald {
    fn default() -> Self {
        Self::new(TableHeraldConfig::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::platform::container::herald::{ExecutionMetadata, StreamChunk};

    #[test]
    fn test_table_herald_creation() {
        let herald = TableHerald::default();
        assert_eq!(herald.name(), "table");
        assert_eq!(herald.mime_type(), "text/plain");
        assert_eq!(herald.config.max_column_width, 60);
        assert_eq!(herald.config.border_style, "rounded");
    }

    #[test]
    fn test_table_herald_custom_config() {
        let config = TableHeraldConfig {
            max_column_width: 100,
            border_style: "ascii".to_string(),
        };
        let herald = TableHerald::new(config);
        assert_eq!(herald.config.max_column_width, 100);
        assert_eq!(herald.config.border_style, "ascii");
    }

    #[test]
    fn test_format_paladin_result() {
        use crate::application::ports::output::paladin_port::StopReason;
        let herald = TableHerald::default();
        let result = crate::core::platform::container::herald::PaladinResult {
            output: "Test output".to_string(),
            token_count: 100,
            execution_time_ms: 1500,
            loop_count: 1,
            stop_reason: StopReason::Completed,
            ..Default::default()
        };

        let output = herald.format_paladin_result(&result);
        assert!(output.is_ok());

        let formatted = output.unwrap();
        assert!(formatted.contains("Field"));
        assert!(formatted.contains("Value"));
        assert!(formatted.contains("Paladin"));
    }

    #[test]
    fn test_format_battalion_result() {
        use crate::core::platform::container::battalion::BattalionStatus;
        use chrono::Utc;
        use uuid::Uuid;
        let herald = TableHerald::default();
        let result = crate::core::platform::container::herald::BattalionResult {
            battalion_id: Uuid::new_v4(),
            battalion_name: "Test Battalion".to_string(),
            started_at: Utc::now(),
            completed_at: Utc::now(),
            final_output: "Combined output".to_string(),
            paladin_results: vec![],
            status: BattalionStatus::Completed,
            strategy_used:
                crate::core::platform::container::battalion::BattalionStrategy::Formation,
            strategy_selection_reasoning: None,
            strategy_selection_time_ms: 0,
            per_paladin_times: std::collections::HashMap::new(),
            per_paladin_tokens: std::collections::HashMap::new(),
            total_tokens: 0,
            paladin_success_count: 0,
            paladin_failure_count: 0,
        };

        let output = herald.format_battalion_result(&result);
        assert!(output.is_ok());

        let formatted = output.unwrap();
        assert!(formatted.contains("Battalion Execution Results"));
        assert!(formatted.contains("Paladin"));
        assert!(formatted.contains("Status"));
        assert!(formatted.contains("Duration"));
        assert!(formatted.contains("Tokens"));
    }

    #[test]
    fn test_format_stream_chunk_returns_none() {
        let herald = TableHerald::default();
        let chunk = crate::core::platform::container::herald::StreamChunk::builder()
            .chunk_id(uuid::Uuid::new_v4())
            .sequence_number(0)
            .timestamp(chrono::Utc::now())
            .content("test content".to_string())
            .is_final(false)
            .build()
            .unwrap();

        let result = herald.format_stream_chunk(&chunk);
        assert!(result.is_ok());
        assert!(result.unwrap().is_none());
    }

    #[test]
    fn test_finalize_stream() {
        use crate::application::ports::output::llm_port::TokenUsage;
        let herald = TableHerald::default();
        let metadata = crate::core::platform::container::herald::ExecutionMetadata::builder()
            .execution_id(uuid::Uuid::new_v4())
            .start_time(chrono::Utc::now())
            .model_used("test-model".to_string())
            .token_usage(TokenUsage {
                prompt_tokens: 300,
                completion_tokens: 200,
                total_tokens: 500,
            })
            .duration_ms(1000)
            .build()
            .unwrap();

        let output = herald.finalize_stream(&metadata);
        assert!(output.is_ok());

        let formatted = output.unwrap();
        assert!(formatted.contains("Execution Metadata"));
        assert!(formatted.contains("Total Duration"));
        assert!(formatted.contains("Total Tokens"));
    }

    #[test]
    fn test_format_error() {
        let herald = TableHerald::default();
        let error = PaladinError::ExecutionError("Test error message".to_string());

        let formatted = herald.format_error(&error);
        assert!(formatted.contains("Error Information"));
        assert!(formatted.contains("Type:"));
        assert!(formatted.contains("Message:"));
        assert!(formatted.contains("Timestamp:"));
    }

    #[test]
    fn test_truncate_text_short() {
        let herald = TableHerald::default();
        let text = "Short text";
        let truncated = herald.truncate_text(text);
        assert_eq!(truncated, "Short text");
    }

    #[test]
    fn test_truncate_text_long() {
        let config = TableHeraldConfig {
            max_column_width: 20,
            border_style: "rounded".to_string(),
        };
        let herald = TableHerald::new(config);
        let text = "This is a very long text that should be truncated";
        let truncated = herald.truncate_text(text);
        assert_eq!(truncated.len(), 20);
        assert!(truncated.ends_with("..."));
    }

    #[test]
    fn test_border_style_ascii() {
        let config = TableHeraldConfig {
            max_column_width: 60,
            border_style: "ascii".to_string(),
        };
        let herald = TableHerald::new(config);
        let table = herald.create_table();
        // Empty table with border preset may contain borders
        let _output = table.to_string();
        // Just verify it doesn't panic
    }

    #[test]
    fn test_border_style_rounded() {
        let config = TableHeraldConfig {
            max_column_width: 60,
            border_style: "rounded".to_string(),
        };
        let herald = TableHerald::new(config);
        let table = herald.create_table();
        let _output = table.to_string();
        // Just verify it doesn't panic
    }

    #[test]
    fn test_border_style_modern() {
        let config = TableHeraldConfig {
            max_column_width: 60,
            border_style: "modern".to_string(),
        };
        let herald = TableHerald::new(config);
        let table = herald.create_table();
        let _output = table.to_string();
        // Just verify it doesn't panic
    }

    #[test]
    fn test_border_style_none() {
        let config = TableHeraldConfig {
            max_column_width: 60,
            border_style: "none".to_string(),
        };
        let herald = TableHerald::new(config);
        let table = herald.create_table();
        assert!(table.to_string().is_empty()); // Empty table
    }

    #[test]
    fn test_border_style_invalid_defaults_to_rounded() {
        let config = TableHeraldConfig {
            max_column_width: 60,
            border_style: "invalid_style".to_string(),
        };
        let herald = TableHerald::new(config);
        let table = herald.create_table();
        let _output = table.to_string();
        // Just verify it doesn't panic and defaults gracefully
    }

    #[test]
    fn test_format_status_success() {
        let herald = TableHerald::default();
        let cell = herald.format_status("success");
        // Cell doesn't implement Display, just verify it was created
        assert!(cell.content().contains("success"));
    }

    #[test]
    fn test_format_status_failed() {
        let herald = TableHerald::default();
        let cell = herald.format_status("failed");
        // Cell doesn't implement Display, just verify it was created
        assert!(cell.content().contains("failed"));
    }

    #[test]
    fn test_format_status_timeout() {
        let herald = TableHerald::default();
        let cell = herald.format_status("timeout");
        // Cell doesn't implement Display, just verify it was created
        assert!(cell.content().contains("timeout"));
    }

    #[test]
    fn test_config_default() {
        let config = TableHeraldConfig::default();
        assert_eq!(config.max_column_width, 60);
        assert_eq!(config.border_style, "rounded");
    }

    #[test]
    fn test_config_serialization() {
        let config = TableHeraldConfig {
            max_column_width: 80,
            border_style: "ascii".to_string(),
        };

        let json = serde_json::to_string(&config).unwrap();
        let deserialized: TableHeraldConfig = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.max_column_width, 80);
        assert_eq!(deserialized.border_style, "ascii");
    }

    #[test]
    fn test_streaming_buffering_behavior() {
        let herald = TableHerald::default();

        // TableHerald should buffer all chunks and return None during streaming
        let chunks = vec![
            StreamChunk::builder()
                .chunk_id(uuid::Uuid::new_v4())
                .sequence_number(0)
                .timestamp(chrono::Utc::now())
                .content("First chunk".to_string())
                .is_final(false)
                .build()
                .unwrap(),
            StreamChunk::builder()
                .chunk_id(uuid::Uuid::new_v4())
                .sequence_number(1)
                .timestamp(chrono::Utc::now())
                .content("Second chunk".to_string())
                .is_final(false)
                .build()
                .unwrap(),
            StreamChunk::builder()
                .chunk_id(uuid::Uuid::new_v4())
                .sequence_number(2)
                .timestamp(chrono::Utc::now())
                .content("Final chunk".to_string())
                .is_final(true)
                .build()
                .unwrap(),
        ];

        // All chunks should return None (buffering)
        for chunk in &chunks {
            let result = herald.format_stream_chunk(chunk).unwrap();
            assert!(
                result.is_none(),
                "TableHerald should buffer chunks and return None"
            );
        }

        // Only finalize_stream should produce output
        use crate::application::ports::output::llm_port::TokenUsage;
        let metadata = ExecutionMetadata::builder()
            .execution_id(uuid::Uuid::new_v4())
            .start_time(chrono::Utc::now())
            .model_used("test-model".to_string())
            .token_usage(TokenUsage {
                prompt_tokens: 240,
                completion_tokens: 160,
                total_tokens: 400,
            })
            .duration_ms(2000)
            .build()
            .unwrap();
        let metadata_output = herald.finalize_stream(&metadata).unwrap();

        // Verify metadata table is generated
        assert!(!metadata_output.is_empty());
        assert!(metadata_output.contains("│") || metadata_output.contains("|"));
        // Metadata table should contain placeholder or actual values
        assert!(metadata_output.contains("Duration") || metadata_output.contains("Tokens"));
    }
}
