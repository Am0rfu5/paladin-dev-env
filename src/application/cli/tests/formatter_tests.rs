//! Tests for CLI formatters (JSON, Markdown, tables)

#[cfg(test)]
mod tests {
    use crate::application::cli::formatters::{OutputFormatter, OutputStyle, TableFormatter};

    // ============ Output Formatter Tests ============

    #[test]
    fn test_output_formatter_success() {
        let formatter = OutputFormatter::new();
        formatter.success("Operation completed");
        // Method prints to stdout, just verify it doesn't panic
    }

    #[test]
    fn test_output_formatter_error() {
        let formatter = OutputFormatter::new();
        formatter.error("Operation failed");
    }

    #[test]
    fn test_output_formatter_warning() {
        let formatter = OutputFormatter::new();
        formatter.warning("Warning message");
    }

    #[test]
    fn test_output_formatter_info() {
        let formatter = OutputFormatter::new();
        formatter.info("Information");
    }

    #[test]
    fn test_output_style_variants() {
        let _success = OutputStyle::Success;
        let _error = OutputStyle::Error;
        let _warning = OutputStyle::Warning;
        let _info = OutputStyle::Info;
        let _default = OutputStyle::Default;
    }

    // ============ Table Formatter Tests ============

    #[test]
    fn test_table_formatter_basic() {
        let mut formatter = TableFormatter::new();
        formatter.set_header(vec!["Name", "Age"]);
        formatter.add_row(vec!["Alice", "30"]);
        formatter.add_row(vec!["Bob", "25"]);

        let result = formatter.render();
        assert!(result.contains("Name"));
        assert!(result.contains("Age"));
        assert!(result.contains("Alice"));
        assert!(result.contains("Bob"));
    }

    #[test]
    fn test_table_formatter_single_row() {
        let mut formatter = TableFormatter::new();
        formatter.set_header(vec!["Col1"]);
        formatter.add_row(vec!["Value"]);

        let result = formatter.render();
        assert!(result.contains("Col1"));
        assert!(result.contains("Value"));
    }

    #[test]
    fn test_table_formatter_empty_rows() {
        let mut formatter = TableFormatter::new();
        formatter.set_header(vec!["Col1", "Col2"]);

        let result = formatter.render();
        assert!(result.contains("Col1"));
        assert!(result.contains("Col2"));
    }

    #[test]
    fn test_table_formatter_wide_content() {
        let mut formatter = TableFormatter::new();
        formatter.set_header(vec!["Short", "Very Long Header Name"]);
        formatter.add_row(vec!["X", "This is a very long piece of content"]);

        let result = formatter.render();
        assert!(result.contains("Short"));
        assert!(result.contains("Very Long Header Name"));
        assert!(result.contains("This is a very long piece of content"));
    }

    #[test]
    fn test_table_formatter_special_chars() {
        let mut formatter = TableFormatter::new();
        formatter.set_header(vec!["Name"]);
        formatter.add_row(vec!["Test | With | Pipes"]);

        let result = formatter.render();
        // Should handle special characters gracefully
        assert!(result.contains("Name"));
    }

    #[test]
    fn test_table_formatter_alignment() {
        let mut formatter = TableFormatter::new();
        formatter.set_header(vec!["Left", "Center", "Right"]);
        formatter.add_row(vec!["A", "B", "C"]);

        let result = formatter.render();
        // Verify table structure exists
        assert!(result.contains("Left"));
        assert!(result.contains("Center"));
        assert!(result.contains("Right"));
    }

    #[test]
    fn test_table_formatter_empty_cells() {
        let mut formatter = TableFormatter::new();
        formatter.set_header(vec!["Col1", "Col2"]);
        formatter.add_row(vec!["Value", ""]);
        formatter.add_row(vec!["", "Another"]);

        let result = formatter.render();
        assert!(result.contains("Value"));
        assert!(result.contains("Another"));
    }

    #[test]
    fn test_table_formatter_numeric_data() {
        let mut formatter = TableFormatter::new();
        formatter.set_header(vec!["ID", "Count", "Percentage"]);
        formatter.add_row(vec!["1", "100", "95.5%"]);
        formatter.add_row(vec!["2", "250", "88.2%"]);

        let result = formatter.render();
        assert!(result.contains("100"));
        assert!(result.contains("95.5%"));
    }
}
