//! Snapshot tests for CLI table output formatting
//!
//! Uses insta for snapshot testing to ensure table rendering consistency.
//! Run tests with `cargo test` and review snapshots with `cargo insta review`.

use paladin::application::cli::formatters::table::TableFormatter;

#[test]
fn test_simple_table() {
    // Test basic table with headers and rows
    let mut table = TableFormatter::new();
    table
        .set_header(vec!["Name", "Age", "City"])
        .add_row(vec!["Alice", "30", "New York"])
        .add_row(vec!["Bob", "25", "Los Angeles"])
        .add_row(vec!["Charlie", "35", "Chicago"]);

    let output = table.render();
    insta::assert_snapshot!("simple_table", output);
}

#[test]
fn test_table_with_long_content() {
    // Test table with longer content that may wrap
    let mut table = TableFormatter::new();
    table
        .set_header(vec!["Feature", "Description", "Status"])
        .add_row(vec![
            "Authentication",
            "User authentication with JWT tokens and refresh token support",
            "Completed",
        ])
        .add_row(vec![
            "Database",
            "PostgreSQL database with connection pooling",
            "In Progress",
        ])
        .add_row(vec![
            "API",
            "RESTful API with comprehensive endpoint documentation",
            "Planned",
        ]);

    let output = table.render();
    insta::assert_snapshot!("table_with_long_content", output);
}

#[test]
fn test_styled_cells() {
    // Test table with styled cells (success, error, warning, info)
    let mut table = TableFormatter::new();
    table.set_header(vec!["Component", "Status", "Details"]);

    // Add rows with different styled cells
    let success_cell = table.success_cell("✓ Operational");
    let info_cell = table.info_cell("All systems normal");
    table.add_styled_row(vec!["API Server".into(), success_cell, info_cell]);

    let warning_cell = table.warning_cell("⚠ Degraded");
    let warning_info = table.warning_cell("High latency detected");
    table.add_styled_row(vec!["Database".into(), warning_cell, warning_info]);

    let error_cell = table.error_cell("✗ Down");
    let error_info = table.error_cell("Connection refused");
    table.add_styled_row(vec!["Cache".into(), error_cell, error_info]);

    let output = table.render();
    insta::assert_snapshot!("styled_cells_table", output);
}

#[test]
fn test_empty_table() {
    // Test empty table (only structure, no rows)
    let table = TableFormatter::new();
    let output = table.render();
    insta::assert_snapshot!("empty_table", output);
}

#[test]
fn test_single_column_table() {
    // Test table with single column
    let mut table = TableFormatter::new();
    table
        .set_header(vec!["Task"])
        .add_row(vec!["Complete setup"])
        .add_row(vec!["Run tests"])
        .add_row(vec!["Deploy to production"]);

    let output = table.render();
    insta::assert_snapshot!("single_column_table", output);
}

#[test]
fn test_table_with_numbers() {
    // Test table with numeric data (good for alignment testing)
    let mut table = TableFormatter::new();
    table
        .set_header(vec!["Metric", "Value", "Change"])
        .add_row(vec!["Response Time", "125ms", "+5%"])
        .add_row(vec!["Throughput", "1,250 req/s", "-3%"])
        .add_row(vec!["Error Rate", "0.12%", "-15%"])
        .add_row(vec!["CPU Usage", "45%", "+8%"]);

    let output = table.render();
    insta::assert_snapshot!("table_with_numbers", output);
}

#[test]
fn test_table_with_special_characters() {
    // Test table with unicode and special characters
    let mut table = TableFormatter::new();
    table
        .set_header(vec!["Symbol", "Description", "Unicode"])
        .add_row(vec!["✓", "Check Mark", "U+2713"])
        .add_row(vec!["✗", "Cross Mark", "U+2717"])
        .add_row(vec!["⚠", "Warning Sign", "U+26A0"])
        .add_row(vec!["→", "Right Arrow", "U+2192"]);

    let output = table.render();
    insta::assert_snapshot!("table_with_special_chars", output);
}

#[test]
fn test_battalion_result_table() {
    // Test table format for Battalion execution results
    let mut table = TableFormatter::new();
    table.set_header(vec![
        "Paladin",
        "Status",
        "Time",
        "Tokens",
        "Output Preview",
    ]);

    let success = table.success_cell("✓ Success");
    table.add_styled_row(vec![
        "DataAnalyzer".into(),
        success,
        "1.2s".into(),
        "450".into(),
        "Analysis complete: 3 insights found...".into(),
    ]);

    let success2 = table.success_cell("✓ Success");
    table.add_styled_row(vec![
        "ReportGenerator".into(),
        success2,
        "0.8s".into(),
        "320".into(),
        "Generated report in Markdown format...".into(),
    ]);

    let warning = table.warning_cell("⚠ Timeout");
    table.add_styled_row(vec![
        "DataValidator".into(),
        warning,
        "5.0s".into(),
        "120".into(),
        "Partial validation completed...".into(),
    ]);

    let output = table.render();
    insta::assert_snapshot!("battalion_result_table", output);
}
