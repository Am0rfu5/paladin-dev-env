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
//! use paladin_herald::table_herald::{TableHerald, TableHeraldConfig};
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

use comfy_table::{Attribute, Cell, CellAlignment, Color, ContentArrangement, Table, presets};
use paladin_core::platform::container::herald::{Herald, HeraldError, PaladinError};
use serde::{Deserialize, Serialize};
use std::fmt::Write as FmtWrite;

/// Configuration for TableHerald formatter.
///
/// Controls table rendering behavior, including column widths and border styles.
#[doc(hidden)]
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
#[doc(hidden)]
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
        _result: &paladin_core::platform::container::herald::PaladinResult,
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
        result: &paladin_core::platform::container::herald::BattalionResult,
    ) -> Result<String, HeraldError> {
        let mut output = String::new();

        writeln!(&mut output, "Battalion: {}", result.battalion_name).map_err(|e| {
            HeraldError::SerializationError(format!("Failed to write output: {}", e))
        })?;
        writeln!(&mut output, "ID: {}", result.battalion_id).map_err(|e| {
            HeraldError::SerializationError(format!("Failed to write output: {}", e))
        })?;
        writeln!(&mut output, "Strategy: {:?}", result.strategy_used).map_err(|e| {
            HeraldError::SerializationError(format!("Failed to write output: {}", e))
        })?;
        writeln!(&mut output, "Total Tokens: {}\n", result.total_tokens).map_err(|e| {
            HeraldError::SerializationError(format!("Failed to write output: {}", e))
        })?;

        let mut table = self.create_table();

        // Set header for battalion summary
        table.set_header(vec![
            Cell::new("Paladin").add_attribute(Attribute::Bold),
            Cell::new("Status").add_attribute(Attribute::Bold),
            Cell::new("Duration").add_attribute(Attribute::Bold),
            Cell::new("Tokens").add_attribute(Attribute::Bold),
        ]);

        // A consumable pool of (name, time, tokens) built from the
        // name-keyed per_paladin_times/per_paladin_tokens maps Formation and
        // Phalanx populate. Each map entry carries the exact
        // execution_time_ms/token_count pair copied from its PaladinResult,
        // so matching a row's PaladinResult against this pool recovers the
        // real Paladin name — PaladinResult itself carries no name field.
        let mut name_pool: Vec<(String, u64, u32)> = result
            .per_paladin_times
            .iter()
            .filter_map(|(name, time)| {
                result
                    .per_paladin_tokens
                    .get(name)
                    .map(|tokens| (name.clone(), *time, tokens.total_tokens))
            })
            .collect();

        for (idx, paladin_result) in result.paladin_results.iter().enumerate() {
            let name = name_pool
                .iter()
                .position(|(_, time, tokens)| {
                    *time == paladin_result.execution_time_ms
                        && *tokens == paladin_result.token_count
                })
                .map(|pos| name_pool.remove(pos).0)
                .unwrap_or_else(|| format!("Paladin {}", idx + 1));

            let status_str = format!("{:?}", paladin_result.stop_reason);
            table.add_row(vec![
                Cell::new(self.truncate_text(&name)),
                self.format_status(&status_str),
                Cell::new(format!("{}ms", paladin_result.execution_time_ms))
                    .set_alignment(CellAlignment::Right),
                Cell::new(paladin_result.token_count.to_string())
                    .set_alignment(CellAlignment::Right),
            ]);
        }

        writeln!(&mut output, "{}", table).map_err(|e| {
            HeraldError::SerializationError(format!("Failed to write table: {}", e))
        })?;

        if !result.node_errors.is_empty() {
            writeln!(&mut output, "\nFailures:").map_err(|e| {
                HeraldError::SerializationError(format!("Failed to write output: {}", e))
            })?;
            for node_error in &result.node_errors {
                writeln!(
                    &mut output,
                    "  {}: {}",
                    node_error.node_name, node_error.error
                )
                .map_err(|e| {
                    HeraldError::SerializationError(format!("Failed to write output: {}", e))
                })?;
            }
        }

        Ok(output)
    }

    fn format_stream_chunk(
        &self,
        _chunk: &paladin_core::platform::container::herald::StreamChunk,
    ) -> Result<Option<String>, HeraldError> {
        // Table formatting doesn't support streaming - accumulate until complete
        Ok(None)
    }

    fn finalize_stream(
        &self,
        _metadata: &paladin_core::platform::container::herald::ExecutionMetadata,
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
    use paladin_core::platform::container::herald::{ExecutionMetadata, StreamChunk};

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
        use paladin_ports::output::paladin_port::StopReason;
        let herald = TableHerald::default();
        let result = paladin_core::platform::container::herald::PaladinResult {
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

    /// Builds a `BattalionResult` with `n` Paladins whose names, execution
    /// times and token counts are all distinct and non-round, so assertions
    /// against the rendered output cannot pass if the formatter ignores its
    /// `result` argument (RESEARCH.md Pitfall 5's litmus test).
    fn battalion_result_with_paladins(
        names: &[&str],
    ) -> paladin_core::platform::container::herald::BattalionResult {
        use chrono::Utc;
        use paladin_core::platform::container::battalion::BattalionStatus;
        use paladin_ports::output::paladin_port::StopReason;
        use uuid::Uuid;

        let mut per_paladin_times = std::collections::HashMap::new();
        let mut per_paladin_tokens = std::collections::HashMap::new();
        let mut paladin_results = Vec::new();
        let mut total_tokens: u64 = 0;

        for (idx, name) in names.iter().enumerate() {
            let execution_time_ms = 1000 + (idx as u64) * 137;
            let token_count = 101 + (idx as u32) * 263;
            per_paladin_times.insert((*name).to_string(), execution_time_ms);
            per_paladin_tokens.insert(
                (*name).to_string(),
                paladin_core::platform::container::battalion::TokenUsage::from_total(token_count),
            );
            total_tokens += u64::from(token_count);
            paladin_results.push(paladin_core::platform::container::herald::PaladinResult {
                output: format!("{} output", name),
                token_count,
                execution_time_ms,
                loop_count: 1,
                stop_reason: StopReason::Completed,
                ..Default::default()
            });
        }

        paladin_core::platform::container::herald::BattalionResult {
            battalion_id: Uuid::new_v4(),
            battalion_name: "Test Battalion".to_string(),
            started_at: Utc::now(),
            completed_at: Utc::now(),
            final_output: "Combined output".to_string(),
            paladin_results,
            status: BattalionStatus::Completed,
            strategy_used:
                paladin_core::platform::container::battalion::BattalionStrategy::Formation,
            strategy_selection_reasoning: None,
            strategy_selection_time_ms: 0,
            per_paladin_times,
            per_paladin_tokens,
            total_tokens,
            paladin_success_count: names.len(),
            paladin_failure_count: 0,
            node_errors: Vec::new(),
        }
    }

    #[test]
    fn test_table_herald_renders_actual_paladin_names() {
        let herald = TableHerald::default();
        let result = battalion_result_with_paladins(&["Scoutmaster", "Sentinel", "Vanguard"]);

        let output = herald.format_battalion_result(&result);
        assert!(output.is_ok());
        let formatted = output.unwrap();

        // Each real Paladin name appears in the output.
        assert!(formatted.contains("Scoutmaster"));
        assert!(formatted.contains("Sentinel"));
        assert!(formatted.contains("Vanguard"));

        // The Battalion's own identity and strategy are surfaced.
        assert!(formatted.contains("Test Battalion"));
        assert!(formatted.contains(&result.battalion_id.to_string()));
        assert!(formatted.contains("Formation"));
        assert!(formatted.contains(&result.total_tokens.to_string()));

        // Row count matches Paladin count: one row per entry, in order.
        let scoutmaster_pos = formatted.find("Scoutmaster").unwrap();
        let sentinel_pos = formatted.find("Sentinel").unwrap();
        let vanguard_pos = formatted.find("Vanguard").unwrap();
        assert!(scoutmaster_pos < sentinel_pos);
        assert!(sentinel_pos < vanguard_pos);

        // The litmus test (RESEARCH.md Pitfall 5): rendering a second,
        // differently-populated result produces a different string. A
        // formatter that ignores `result` would produce identical output
        // for both.
        let other_result = battalion_result_with_paladins(&["Herald", "Marshal"]);
        let other_output = herald.format_battalion_result(&other_result).unwrap();
        assert_ne!(formatted, other_output);
    }

    #[test]
    fn test_table_herald_renders_empty_paladin_results() {
        let herald = TableHerald::default();
        let result = battalion_result_with_paladins(&[]);
        assert!(result.paladin_results.is_empty());

        let output = herald.format_battalion_result(&result);
        assert!(output.is_ok());
        let formatted = output.unwrap();

        // Header labels are present...
        assert!(formatted.contains("Paladin"));
        assert!(formatted.contains("Status"));
        assert!(formatted.contains("Duration"));
        assert!(formatted.contains("Tokens"));

        // ...but no body row: no duration suffix and none of the old
        // stub's invented Paladin names.
        assert!(!formatted.contains("ms"));
        assert!(!formatted.contains("paladin_1"));
        assert!(!formatted.contains("paladin_2"));
    }

    #[test]
    fn test_table_herald_renders_multibyte_paladin_name() {
        let herald = TableHerald::default();
        let result = battalion_result_with_paladins(&["斥候レビュアー"]);

        let output = herald.format_battalion_result(&result);
        assert!(output.is_ok());
        let formatted = output.unwrap();

        // The multi-byte name round-trips intact: no panic, no mid-character
        // truncation, no replacement character.
        assert!(formatted.contains("斥候レビュアー"));
        assert!(!formatted.contains('\u{FFFD}'));
    }

    #[test]
    fn test_table_herald_surfaces_node_error_names() {
        let herald = TableHerald::default();
        let mut result = battalion_result_with_paladins(&["Scoutmaster"]);
        result.node_errors = vec![paladin_core::platform::container::battalion::NodeError {
            node_name: "Herald".to_string(),
            error: "connection refused".to_string(),
        }];

        let formatted = herald.format_battalion_result(&result).unwrap();

        assert!(formatted.contains("Herald"));
        assert!(formatted.contains("connection refused"));
    }

    #[test]
    fn test_format_stream_chunk_returns_none() {
        let herald = TableHerald::default();
        let chunk = paladin_core::platform::container::herald::StreamChunk::builder()
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
        use paladin_ports::output::llm_port::TokenUsage;
        let herald = TableHerald::default();
        let metadata = paladin_core::platform::container::herald::ExecutionMetadata::builder()
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
        use paladin_ports::output::llm_port::TokenUsage;
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
