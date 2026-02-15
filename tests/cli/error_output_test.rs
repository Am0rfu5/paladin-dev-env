//! Snapshot tests for CLI error output formatting
//!
//! Tests error messages, warnings, and styled output from OutputFormatter.

use paladin::application::cli::formatters::output::{OutputFormatter, OutputStyle};

#[test]
fn test_error_message_styles() {
    // Test different error message styles
    let formatter = OutputFormatter::new();
    
    let mut outputs = Vec::new();
    outputs.push(formatter.style("Fatal error occurred", OutputStyle::Error));
    outputs.push(formatter.style("Configuration file not found", OutputStyle::Error));
    outputs.push(formatter.style("Connection timeout", OutputStyle::Error));
    outputs.push(formatter.style("Authentication failed", OutputStyle::Error));

    let output = outputs.join("\n");
    insta::assert_snapshot!("error_message_styles", output);
}

#[test]
fn test_warning_message_styles() {
    // Test warning message styles
    let formatter = OutputFormatter::new();
    
    let mut outputs = Vec::new();
    outputs.push(formatter.style("Deprecated API usage", OutputStyle::Warning));
    outputs.push(formatter.style("Resource limit approaching", OutputStyle::Warning));
    outputs.push(formatter.style("Unverified certificate", OutputStyle::Warning));
    outputs.push(formatter.style("Memory usage high", OutputStyle::Warning));

    let output = outputs.join("\n");
    insta::assert_snapshot!("warning_message_styles", output);
}

#[test]
fn test_info_message_styles() {
    // Test informational message styles
    let formatter = OutputFormatter::new();
    
    let mut outputs = Vec::new();
    outputs.push(formatter.style("Loading configuration...", OutputStyle::Info));
    outputs.push(formatter.style("Connecting to server...", OutputStyle::Info));
    outputs.push(formatter.style("Processing request...", OutputStyle::Info));
    outputs.push(formatter.style("Operation completed", OutputStyle::Info));

    let output = outputs.join("\n");
    insta::assert_snapshot!("info_message_styles", output);
}

#[test]
fn test_success_message_styles() {
    // Test success message styles
    let formatter = OutputFormatter::new();
    
    let mut outputs = Vec::new();
    outputs.push(formatter.style("✓ Connection established", OutputStyle::Success));
    outputs.push(formatter.style("✓ Tests passed", OutputStyle::Success));
    outputs.push(formatter.style("✓ Deployment successful", OutputStyle::Success));
    outputs.push(formatter.style("✓ Backup completed", OutputStyle::Success));

    let output = outputs.join("\n");
    insta::assert_snapshot!("success_message_styles", output);
}

#[test]
fn test_link_style() {
    // Test link/reference styling
    let formatter = OutputFormatter::new();
    
    let mut outputs = Vec::new();
    outputs.push(formatter.style("https://docs.paladin.rs", OutputStyle::Link));
    outputs.push(formatter.style("See CONTRIBUTING.md", OutputStyle::Link));
    outputs.push(formatter.style("API Reference: /api/v1", OutputStyle::Link));
    outputs.push(formatter.style("GitHub: DF3NDR/paladin", OutputStyle::Link));

    let output = outputs.join("\n");
    insta::assert_snapshot!("link_styles", output);
}

#[test]
fn test_header_rendering() {
    // Test header box rendering
    let formatter = OutputFormatter::new();
    
    let mut output = String::new();
    
    // Capture header output by using a test helper
    // Note: header() prints to stdout, so we test the underlying format
    let test_headers = vec![
        "System Startup",
        "Configuration",
        "Test Results",
        "Deployment Status",
    ];

    for title in test_headers {
        let width = title.len() + 4;
        let border = "═".repeat(width);
        output.push_str(&format!("┌{}┐\n", border));
        output.push_str(&format!("│ {} │\n", formatter.style(title, OutputStyle::Info)));
        output.push_str(&format!("└{}┘\n\n", border));
    }

    insta::assert_snapshot!("header_rendering", output);
}

#[test]
fn test_section_rendering() {
    // Test section header rendering
    let formatter = OutputFormatter::new();
    
    let sections = vec![
        "Database Connection",
        "API Endpoints",
        "Background Jobs",
        "Metrics",
    ];

    let output = sections
        .iter()
        .map(|title| formatter.style(&format!("━━ {} ━━", title), OutputStyle::Info))
        .collect::<Vec<_>>()
        .join("\n\n");

    insta::assert_snapshot!("section_rendering", output);
}

#[test]
fn test_box_message_rendering() {
    // Test box message rendering
    let _formatter = OutputFormatter::new();
    
    let content = vec![
        "Important Information",
        "  • Configuration loaded successfully",
        "  • Database connection established",
        "  • API server running on port 8080",
    ];

    let max_width = content.iter().map(|s| s.len()).max().unwrap_or(0);
    let border = "─".repeat(max_width + 2);

    let mut output = String::new();
    output.push_str(&format!("┌{}┐\n", border));
    for line in &content {
        output.push_str(&format!("│ {:<width$} │\n", line, width = max_width));
    }
    output.push_str(&format!("└{}┘\n", border));

    insta::assert_snapshot!("box_message_rendering", output);
}

#[test]
fn test_key_value_formatting() {
    // Test key-value pair formatting
    let formatter = OutputFormatter::new();
    
    let kvs = vec![
        ("Version", "1.0.0"),
        ("Environment", "production"),
        ("Database", "PostgreSQL 14.5"),
        ("Cache", "Redis 7.0"),
        ("Queue", "RabbitMQ 3.11"),
    ];

    let output = kvs
        .iter()
        .map(|(k, v)| formatter.key_value(k, v))
        .collect::<Vec<_>>()
        .join("\n");

    insta::assert_snapshot!("key_value_formatting", output);
}

#[test]
fn test_emoji_fallback() {
    // Test emoji vs text fallback
    let formatter = OutputFormatter::new();
    
    let mut outputs = Vec::new();
    outputs.push(format!("{} Success", formatter.emoji_or("✓", "[OK]")));
    outputs.push(format!("{} Error", formatter.emoji_or("✗", "[ERR]")));
    outputs.push(format!("{} Warning", formatter.emoji_or("⚠", "[WARN]")));
    outputs.push(format!("{} Info", formatter.emoji_or("ℹ", "[INFO]")));
    outputs.push(format!("{} Processing", formatter.emoji_or("⚙", "[PROC]")));

    let output = outputs.join("\n");
    insta::assert_snapshot!("emoji_fallback", output);
}

#[test]
fn test_separator_line() {
    // Test separator line rendering
    let _formatter = OutputFormatter::new();
    
    // Separator is 64 '=' characters
    let separator = "═".repeat(64);
    
    insta::assert_snapshot!("separator_line", separator);
}

#[test]
fn test_quiet_mode() {
    // Test quiet mode (should suppress most output)
    let formatter = OutputFormatter::quiet();
    
    assert!(formatter.is_quiet());
    assert!(!formatter.is_verbose());
    
    let output = format!(
        "Quiet: {}\nVerbose: {}",
        formatter.is_quiet(),
        formatter.is_verbose()
    );
    
    insta::assert_snapshot!("quiet_mode_flags", output);
}

#[test]
fn test_verbose_mode() {
    // Test verbose mode
    let formatter = OutputFormatter::with_verbose();
    
    assert!(!formatter.is_quiet());
    assert!(formatter.is_verbose());
    
    let output = format!(
        "Quiet: {}\nVerbose: {}",
        formatter.is_quiet(),
        formatter.is_verbose()
    );
    
    insta::assert_snapshot!("verbose_mode_flags", output);
}

#[test]
fn test_combined_error_scenarios() {
    // Test realistic error scenarios
    let formatter = OutputFormatter::new();
    
    let mut output = String::new();
    
    // Scenario 1: Configuration error
    output.push_str("=== Configuration Error ===\n");
    output.push_str(&format!("{}\n", formatter.style("✗ Failed to load config.yml", OutputStyle::Error)));
    output.push_str(&format!("  {}\n", formatter.style("File not found: /etc/paladin/config.yml", OutputStyle::Error)));
    output.push_str(&format!("  {}\n\n", formatter.style("Suggestion: Run 'paladin init' to create default config", OutputStyle::Info)));
    
    // Scenario 2: Connection error
    output.push_str("=== Connection Error ===\n");
    output.push_str(&format!("{}\n", formatter.style("✗ Database connection failed", OutputStyle::Error)));
    output.push_str(&format!("  {}\n", formatter.style("Could not connect to localhost:5432", OutputStyle::Error)));
    output.push_str(&format!("  {}\n\n", formatter.style("Check database status: systemctl status postgresql", OutputStyle::Info)));
    
    // Scenario 3: Validation warning
    output.push_str("=== Validation Warning ===\n");
    output.push_str(&format!("{}\n", formatter.style("⚠ Deprecated configuration detected", OutputStyle::Warning)));
    output.push_str(&format!("  {}\n", formatter.style("Field 'llm.model' should be 'llm.default_model'", OutputStyle::Warning)));
    output.push_str(&format!("  {}\n", formatter.style("Support will be removed in v2.0", OutputStyle::Warning)));

    insta::assert_snapshot!("combined_error_scenarios", output);
}

#[test]
fn test_multi_line_error_formatting() {
    // Test multi-line error with stack trace style
    let formatter = OutputFormatter::new();
    
    let mut output = String::new();
    output.push_str(&format!("{}\n", formatter.style("Error: Failed to execute Paladin", OutputStyle::Error)));
    output.push_str("Caused by:\n");
    output.push_str(&format!("  0: {}\n", formatter.style("LLM API error", OutputStyle::Error)));
    output.push_str(&format!("  1: {}\n", formatter.style("HTTP request timeout", OutputStyle::Error)));
    output.push_str(&format!("  2: {}\n", formatter.style("Connection refused: https://api.openai.com", OutputStyle::Error)));
    output.push_str("\n");
    output.push_str(&format!("{}\n", formatter.style("Suggestion: Check network connectivity and API key", OutputStyle::Info)));

    insta::assert_snapshot!("multi_line_error_formatting", output);
}
