//! Unit tests for CLI formatters

#[cfg(test)]
mod output_formatter_tests {
    use crate::application::cli::formatters::output::OutputFormatter;

    #[test]
    fn test_formatter_creation() {
        let formatter = OutputFormatter::new();
        assert!(!formatter.use_color());
    }

    #[test]
    fn test_style_without_color() {
        let formatter = OutputFormatter::with_color(false);
        let styled = formatter.style("test", colored::Color::Green);
        assert_eq!(styled, "test");
    }

    #[test]
    fn test_success_message() {
        let formatter = OutputFormatter::with_color(false);
        let success = formatter.success("Operation succeeded");
        assert!(success.contains("✓"));
        assert!(success.contains("Operation succeeded"));
    }

    #[test]
    fn test_error_message() {
        let formatter = OutputFormatter::with_color(false);
        let error = formatter.error("Operation failed");
        assert!(error.contains("✗"));
        assert!(error.contains("Operation failed"));
    }

    #[test]
    fn test_warning_message() {
        let formatter = OutputFormatter::with_color(false);
        let warning = formatter.warning("Potential issue");
        assert!(warning.contains("⚠"));
        assert!(warning.contains("Potential issue"));
    }

    #[test]
    fn test_info_message() {
        let formatter = OutputFormatter::with_color(false);
        let info = formatter.info("Informational message");
        assert!(info.contains("ℹ"));
        assert!(info.contains("Informational message"));
    }

    #[test]
    fn test_header() {
        let formatter = OutputFormatter::with_color(false);
        let header = formatter.header("Section Title");
        assert!(header.contains("Section Title"));
        assert!(header.contains("====="));
    }

    #[test]
    fn test_box_message() {
        let formatter = OutputFormatter::with_color(false);
        let boxed = formatter.box_message("Important Notice");
        assert!(boxed.contains("╔"));
        assert!(boxed.contains("╗"));
        assert!(boxed.contains("║"));
        assert!(boxed.contains("╚"));
        assert!(boxed.contains("╝"));
        assert!(boxed.contains("Important Notice"));
    }

    #[test]
    fn test_separator() {
        let formatter = OutputFormatter::new();
        let separator = formatter.separator(40);
        assert_eq!(separator.len(), 40);
        assert!(separator.chars().all(|c| c == '─'));
    }

    #[test]
    fn test_bullet_list() {
        let formatter = OutputFormatter::new();
        let items = vec!["Item 1", "Item 2", "Item 3"];
        let list = formatter.bullet_list(&items);
        assert!(list.contains("• Item 1"));
        assert!(list.contains("• Item 2"));
        assert!(list.contains("• Item 3"));
    }
}

#[cfg(test)]
mod table_formatter_tests {
    use crate::application::cli::formatters::table::TableFormatter;

    #[test]
    fn test_table_creation() {
        let table = TableFormatter::new();
        let output = table.render();
        assert!(!output.is_empty());
    }

    #[test]
    fn test_table_with_headers() {
        let mut table = TableFormatter::new();
        table.set_header(vec!["Name", "Value", "Status"]);
        let output = table.render();
        assert!(output.contains("Name"));
        assert!(output.contains("Value"));
        assert!(output.contains("Status"));
    }

    #[test]
    fn test_table_with_rows() {
        let mut table = TableFormatter::new();
        table.set_header(vec!["Name", "Value"]);
        table.add_row(vec!["Item 1", "100"]);
        table.add_row(vec!["Item 2", "200"]);
        let output = table.render();
        assert!(output.contains("Item 1"));
        assert!(output.contains("100"));
        assert!(output.contains("Item 2"));
        assert!(output.contains("200"));
    }
}

#[cfg(test)]
mod progress_tests {
    use crate::application::cli::formatters::progress::{Spinner, ProgressBarBuilder};

    #[test]
    fn test_spinner_creation() {
        let _spinner = Spinner::new("Loading...");
        // Spinner created successfully
        assert!(true);
    }

    #[test]
    fn test_spinner_finish() {
        let mut spinner = Spinner::new("Processing...");
        spinner.finish_with_message("Complete!");
        // Spinner finished successfully
        assert!(true);
    }

    #[test]
    fn test_progress_bar_builder() {
        let progress = ProgressBarBuilder::new(100)
            .with_message("Downloading")
            .build();
        assert!(progress.position() == 0);
    }

    #[test]
    fn test_progress_bar_increment() {
        let progress = ProgressBarBuilder::new(10).build();
        progress.inc(5);
        assert!(progress.position() == 5);
    }

    #[test]
    fn test_progress_bar_finish() {
        let progress = ProgressBarBuilder::new(100).build();
        progress.set_position(50);
        progress.finish_with_message("Done!");
        assert!(progress.is_finished());
    }
}
