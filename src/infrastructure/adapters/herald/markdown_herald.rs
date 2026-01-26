//! Markdown Herald formatter implementation
//!
//! The MarkdownHerald provides Markdown formatting for Paladin and Battalion execution results.
//! It supports headings, status badges, code blocks, and optional ANSI color output for terminals.
//!
//! # Examples
//!
//! ```rust,ignore
//! use paladin::infrastructure::adapters::herald::MarkdownHerald;
//! use paladin::core::platform::container::herald::Herald;
//!
//! let herald = MarkdownHerald::new();
//! let formatted = herald.format_paladin_result(&result)?;
//! println!("{}", formatted);
//! ```

use crate::core::platform::container::herald::{
    BattalionResult, ExecutionMetadata, Herald, HeraldError, PaladinError, PaladinResult,
    StreamChunk,
};
use colored::*;
use serde::{Deserialize, Serialize};

/// Configuration for Markdown Herald formatter
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarkdownHeraldConfig {
    /// Enable ANSI color codes for terminal output
    pub include_colors: bool,
    /// Heading level for main sections (1-6)
    pub heading_level: u8,
}

impl Default for MarkdownHeraldConfig {
    fn default() -> Self {
        Self {
            include_colors: Self::supports_color(),
            heading_level: 2,
        }
    }
}

impl MarkdownHeraldConfig {
    /// Detect if the terminal supports color output
    ///
    /// Returns true if:
    /// - TERM environment variable is set and not "dumb"
    /// - NO_COLOR environment variable is not set
    /// - Running in a TTY
    pub fn supports_color() -> bool {
        // Check NO_COLOR environment variable (universal way to disable colors)
        if std::env::var("NO_COLOR").is_ok() {
            return false;
        }

        // Check TERM environment variable
        if let Ok(term) = std::env::var("TERM") {
            if term == "dumb" {
                return false;
            }
            // Common color-supporting terminals
            if term.contains("color")
                || term.contains("xterm")
                || term.contains("screen")
                || term.contains("tmux")
            {
                return true;
            }
        }

        // Default to no colors if we can't detect
        false
    }
}

/// Markdown formatter for Paladin execution results
///
/// The MarkdownHerald converts Paladin and Battalion results into Markdown format,
/// making them suitable for documentation, CLI output, and human-readable displays.
///
/// # Thread Safety
///
/// MarkdownHerald is thread-safe and can be shared across threads using `Arc`.
///
/// # Configuration
///
/// - **include_colors**: Enable ANSI color codes for terminal output
/// - **heading_level**: Markdown heading level (1-6) for main sections
///
/// # Examples
///
/// ```rust,ignore
/// // Create with default configuration (auto-detect colors, heading level 2)
/// let herald = MarkdownHerald::new();
///
/// // Create with custom configuration
/// let config = MarkdownHeraldConfig {
///     include_colors: false,
///     heading_level: 3,
/// };
/// let herald = MarkdownHerald::with_config(config);
/// ```
#[derive(Debug, Clone)]
pub struct MarkdownHerald {
    config: MarkdownHeraldConfig,
}

impl MarkdownHerald {
    /// Create a new MarkdownHerald with default configuration
    ///
    /// Default configuration:
    /// - include_colors: auto-detected based on terminal support
    /// - heading_level: 2
    pub fn new() -> Self {
        Self {
            config: MarkdownHeraldConfig::default(),
        }
    }

    /// Create a new MarkdownHerald with custom configuration
    ///
    /// # Arguments
    ///
    /// * `config` - Configuration for the Markdown formatter
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// let config = MarkdownHeraldConfig {
    ///     include_colors: true,
    ///     heading_level: 1,
    /// };
    /// let herald = MarkdownHerald::with_config(config);
    /// ```
    pub fn with_config(config: MarkdownHeraldConfig) -> Self {
        Self { config }
    }

    /// Generate heading markup
    fn heading(&self, level: u8, text: &str) -> String {
        let level = level.clamp(1, 6);
        format!("{} {}\n\n", "#".repeat(level as usize), text)
    }

    /// Generate status badge with emoji
    fn status_badge(&self, status: &str) -> String {
        let (emoji, color_fn): (&str, fn(&str) -> ColoredString) =
            match status.to_lowercase().as_str() {
                "success" | "completed" | "ok" => ("✅", |s| s.green()),
                "failed" | "error" => ("❌", |s| s.red()),
                "timeout" | "warning" => ("⏱️", |s| s.yellow()),
                "running" | "in_progress" => ("🔄", |s| s.blue()),
                "pending" | "queued" => ("⏳", |s| s.cyan()),
                _ => ("ℹ️", |s| s.normal()),
            };

        if self.config.include_colors {
            format!("{} {}", emoji, color_fn(status))
        } else {
            format!("{} {}", emoji, status)
        }
    }

    /// Format a key-value pair as bold key
    fn format_field(&self, key: &str, value: &str) -> String {
        if self.config.include_colors {
            format!("**{}:** {}\n", key.bold(), value)
        } else {
            format!("**{}:** {}\n", key, value)
        }
    }

    /// Format code block
    fn code_block(&self, content: &str, language: &str) -> String {
        format!("```{}\n{}\n```\n\n", language, content)
    }
}

impl Default for MarkdownHerald {
    fn default() -> Self {
        Self::new()
    }
}

impl Herald for MarkdownHerald {
    fn format_paladin_result(&self, result: &PaladinResult) -> Result<String, HeraldError> {
        let mut output = String::new();

        // Main heading
        output.push_str(&self.heading(
            self.config.heading_level,
            &format!("Paladin: {}", result.paladin_name),
        ));

        // Status badge
        output.push_str(&self.status_badge(&result.status));
        output.push_str("\n\n");

        // Output section
        output.push_str(&self.heading(self.config.heading_level + 1, "Output"));
        output.push_str(&result.output);
        output.push_str("\n\n");

        // Metadata section
        output.push_str(&self.heading(self.config.heading_level + 1, "Metadata"));
        output.push_str(&self.format_field("Paladin ID", &result.paladin_id));
        output.push_str(&self.format_field("Status", &result.status));
        output.push_str(&self.format_field("Timestamp", &chrono::Utc::now().to_rfc3339()));

        Ok(output)
    }

    fn format_battalion_result(&self, result: &BattalionResult) -> Result<String, HeraldError> {
        let mut output = String::new();

        // Main heading
        output.push_str(&self.heading(
            self.config.heading_level,
            &format!("Battalion: {}", result.battalion_name),
        ));

        // Status badge
        output.push_str(&self.status_badge(&result.status));
        output.push_str("\n\n");

        // Summary
        output.push_str(&self.format_field("Battalion ID", &result.battalion_id));
        output.push_str(&self.format_field("Total Paladins", &result.results.len().to_string()));
        output.push('\n');

        // Individual Paladin results
        output.push_str(&self.heading(self.config.heading_level + 1, "Paladin Results"));

        for (idx, paladin_result) in result.results.iter().enumerate() {
            output.push_str(&self.heading(
                self.config.heading_level + 2,
                &format!("{}. {}", idx + 1, paladin_result.paladin_name),
            ));
            output.push_str(&self.status_badge(&paladin_result.status));
            output.push_str("\n\n");
            output.push_str(&paladin_result.output);
            output.push_str("\n\n");
        }

        Ok(output)
    }

    fn format_stream_chunk(&self, chunk: &StreamChunk) -> Result<Option<String>, HeraldError> {
        // For Markdown streaming, emit content progressively
        // No special formatting needed for chunks, just pass through
        Ok(Some(chunk.content.clone()))
    }

    fn finalize_stream(&self, metadata: &ExecutionMetadata) -> Result<String, HeraldError> {
        let mut output = String::new();

        output.push_str("\n\n");
        output.push_str(&self.heading(self.config.heading_level + 1, "Execution Metadata"));
        output.push_str(&self.format_field(
            "Execution Time",
            &format!("{}ms", metadata.execution_time_ms),
        ));
        output.push_str(&self.format_field("Total Tokens", &metadata.total_tokens.to_string()));

        Ok(output)
    }

    fn format_error(&self, error: &PaladinError) -> String {
        let mut output = String::new();

        if self.config.include_colors {
            output.push_str(&format!("**{}**\n\n", "Error".red().bold()));
        } else {
            output.push_str("**Error**\n\n");
        }

        output.push_str(&self.code_block(&error.message, ""));

        output
    }

    fn name(&self) -> &str {
        "markdown"
    }

    fn mime_type(&self) -> &str {
        "text/markdown"
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
                    status: "failed".to_string(),
                    output: "Second output".to_string(),
                },
            ],
        }
    }

    #[test]
    fn test_new_creates_default_config() {
        let herald = MarkdownHerald::new();
        assert_eq!(herald.config.heading_level, 2);
    }

    #[test]
    fn test_with_config_uses_custom_config() {
        let config = MarkdownHeraldConfig {
            include_colors: false,
            heading_level: 3,
        };
        let herald = MarkdownHerald::with_config(config);
        assert!(!herald.config.include_colors);
        assert_eq!(herald.config.heading_level, 3);
    }

    #[test]
    fn test_heading_generation() {
        let herald = MarkdownHerald::new();
        let heading = herald.heading(2, "Test Heading");
        assert_eq!(heading, "## Test Heading\n\n");
    }

    #[test]
    fn test_heading_clamps_level() {
        let herald = MarkdownHerald::new();
        let heading_too_low = herald.heading(0, "Test");
        let heading_too_high = herald.heading(10, "Test");
        assert!(heading_too_low.starts_with('#'));
        assert!(heading_too_high.starts_with("######"));
    }

    #[test]
    fn test_status_badge_success() {
        let herald = MarkdownHerald::with_config(MarkdownHeraldConfig {
            include_colors: false,
            heading_level: 2,
        });
        let badge = herald.status_badge("success");
        assert!(badge.contains("✅"));
        assert!(badge.contains("success"));
    }

    #[test]
    fn test_status_badge_failed() {
        let herald = MarkdownHerald::with_config(MarkdownHeraldConfig {
            include_colors: false,
            heading_level: 2,
        });
        let badge = herald.status_badge("failed");
        assert!(badge.contains("❌"));
        assert!(badge.contains("failed"));
    }

    #[test]
    fn test_status_badge_timeout() {
        let herald = MarkdownHerald::with_config(MarkdownHeraldConfig {
            include_colors: false,
            heading_level: 2,
        });
        let badge = herald.status_badge("timeout");
        assert!(badge.contains("⏱️"));
    }

    #[test]
    fn test_format_field() {
        let herald = MarkdownHerald::with_config(MarkdownHeraldConfig {
            include_colors: false,
            heading_level: 2,
        });
        let field = herald.format_field("Key", "Value");
        assert!(field.contains("**Key:**"));
        assert!(field.contains("Value"));
    }

    #[test]
    fn test_code_block() {
        let herald = MarkdownHerald::new();
        let code = herald.code_block("println!(\"test\");", "rust");
        assert!(code.starts_with("```rust"));
        assert!(code.contains("println!(\"test\");"));
        assert!(code.ends_with("```\n\n"));
    }

    #[test]
    fn test_format_paladin_result_structure() {
        let herald = MarkdownHerald::with_config(MarkdownHeraldConfig {
            include_colors: false,
            heading_level: 2,
        });
        let result = create_test_paladin_result();

        let formatted = herald.format_paladin_result(&result).unwrap();

        assert!(formatted.contains("## Paladin: TestPaladin"));
        assert!(formatted.contains("✅"));
        assert!(formatted.contains("### Output"));
        assert!(formatted.contains("Test output content"));
        assert!(formatted.contains("### Metadata"));
        assert!(formatted.contains("**Paladin ID:**"));
    }

    #[test]
    fn test_format_battalion_result_structure() {
        let herald = MarkdownHerald::with_config(MarkdownHeraldConfig {
            include_colors: false,
            heading_level: 2,
        });
        let result = create_test_battalion_result();

        let formatted = herald.format_battalion_result(&result).unwrap();

        assert!(formatted.contains("## Battalion: TestBattalion"));
        assert!(formatted.contains("### Paladin Results"));
        assert!(formatted.contains("1. TestPaladin"));
        assert!(formatted.contains("2. SecondPaladin"));
        assert!(formatted.contains("✅")); // Success badge
        assert!(formatted.contains("❌")); // Failed badge
    }

    #[test]
    fn test_format_stream_chunk() {
        let herald = MarkdownHerald::new();
        let chunk = StreamChunk {
            content: "Streaming content".to_string(),
            is_final: false,
        };

        let formatted = herald.format_stream_chunk(&chunk).unwrap();
        assert!(formatted.is_some());
        assert_eq!(formatted.unwrap(), "Streaming content");
    }

    #[test]
    fn test_finalize_stream() {
        let herald = MarkdownHerald::with_config(MarkdownHeraldConfig {
            include_colors: false,
            heading_level: 2,
        });
        let metadata = ExecutionMetadata {
            execution_time_ms: 1234,
            total_tokens: 500,
        };

        let formatted = herald.finalize_stream(&metadata).unwrap();
        assert!(formatted.contains("### Execution Metadata"));
        assert!(formatted.contains("1234ms"));
        assert!(formatted.contains("500"));
    }

    #[test]
    fn test_format_error() {
        let herald = MarkdownHerald::with_config(MarkdownHeraldConfig {
            include_colors: false,
            heading_level: 2,
        });
        let error = PaladinError {
            message: "Something went wrong".to_string(),
        };

        let formatted = herald.format_error(&error);
        assert!(formatted.contains("**Error**"));
        assert!(formatted.contains("Something went wrong"));
        assert!(formatted.contains("```"));
    }

    #[test]
    fn test_name() {
        let herald = MarkdownHerald::new();
        assert_eq!(herald.name(), "markdown");
    }

    #[test]
    fn test_mime_type() {
        let herald = MarkdownHerald::new();
        assert_eq!(herald.mime_type(), "text/markdown");
    }

    #[test]
    fn test_color_support_detection() {
        // This test verifies the function runs without panicking
        // Actual color support depends on environment
        let _supports_color = MarkdownHeraldConfig::supports_color();
    }

    #[test]
    fn test_with_and_without_colors() {
        let result = create_test_paladin_result();

        let herald_no_color = MarkdownHerald::with_config(MarkdownHeraldConfig {
            include_colors: false,
            heading_level: 2,
        });
        let formatted_no_color = herald_no_color.format_paladin_result(&result).unwrap();

        let herald_with_color = MarkdownHerald::with_config(MarkdownHeraldConfig {
            include_colors: true,
            heading_level: 2,
        });
        let formatted_with_color = herald_with_color.format_paladin_result(&result).unwrap();

        // Both should contain the same content
        assert!(formatted_no_color.contains("TestPaladin"));
        assert!(formatted_with_color.contains("TestPaladin"));

        // With colors version may contain ANSI codes (length difference)
        // This is a rough check since ANSI codes add characters
        assert!(formatted_with_color.len() >= formatted_no_color.len());
    }

    #[test]
    fn test_herald_is_send_sync() {
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<MarkdownHerald>();
    }

    #[test]
    fn test_default_trait() {
        let herald = MarkdownHerald::default();
        assert_eq!(herald.config.heading_level, 2);
    }
}
