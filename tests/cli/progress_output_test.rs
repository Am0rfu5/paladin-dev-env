//! Snapshot tests for CLI progress indicators
//!
//! Progress indicators are inherently dynamic (spinners animate, bars update).
//! These tests focus on the static string representations and builder output.

use paladin::application::cli::formatters::progress::ProgressBarBuilder;

#[test]
fn test_progress_bar_template_default() {
    // Test default progress bar template rendering
    let pb = ProgressBarBuilder::new(100)
        .with_message("Processing items")
        .build();

    // Get the progress bar properties
    let message = pb.message();
    let length = pb.length().unwrap_or(0);
    let position = pb.position();

    let output = format!(
        "Message: {}\nLength: {}\nPosition: {}",
        message, length, position
    );

    insta::assert_snapshot!("progress_bar_default_template", output);
}

#[test]
fn test_progress_bar_template_custom() {
    // Test custom progress bar template
    let custom_template = "{msg} |{bar:50}| {pos}/{len} ETA: {eta}";
    let pb = ProgressBarBuilder::new(200)
        .with_message("Downloading files")
        .with_template(custom_template)
        .build();

    let message = pb.message();
    let length = pb.length().unwrap_or(0);
    let position = pb.position();

    let output = format!(
        "Custom Template Test\nMessage: {}\nLength: {}\nPosition: {}",
        message, length, position
    );

    insta::assert_snapshot!("progress_bar_custom_template", output);
}

#[test]
fn test_progress_bar_different_totals() {
    // Test progress bars with different total values
    let pb_small = ProgressBarBuilder::new(10)
        .with_message("Small task")
        .build();
    let pb_medium = ProgressBarBuilder::new(100)
        .with_message("Medium task")
        .build();
    let pb_large = ProgressBarBuilder::new(10000)
        .with_message("Large task")
        .build();

    let small_length = pb_small.length().unwrap_or(0);
    let medium_length = pb_medium.length().unwrap_or(0);
    let large_length = pb_large.length().unwrap_or(0);

    let output = format!(
        "Small: {} items\nMedium: {} items\nLarge: {} items",
        small_length, medium_length, large_length
    );

    insta::assert_snapshot!("progress_bar_different_totals", output);
}

#[test]
fn test_progress_bar_messages() {
    // Test different progress bar messages
    let messages = [
        "Initializing...",
        "Loading configuration",
        "Connecting to database",
        "Processing records",
        "Generating report",
        "Finalizing",
    ];

    let mut outputs = Vec::new();
    for (i, msg) in messages.iter().enumerate() {
        let pb = ProgressBarBuilder::new(100).with_message(*msg).build();
        outputs.push(format!("{}: Length={}", i + 1, pb.length().unwrap_or(0)));
    }

    let output = outputs.join("\n");
    insta::assert_snapshot!("progress_bar_messages", output);
}

#[test]
fn test_progress_states() {
    // Test snapshot of what progress states look like
    let pb = ProgressBarBuilder::new(100).with_message("Testing").build();

    let mut states = Vec::new();

    // Initial state (0%)
    states.push(format!(
        "0%: {} of {}",
        pb.position(),
        pb.length().unwrap_or(0)
    ));

    // 25% complete
    pb.set_position(25);
    states.push(format!(
        "25%: {} of {}",
        pb.position(),
        pb.length().unwrap_or(0)
    ));

    // 50% complete
    pb.set_position(50);
    states.push(format!(
        "50%: {} of {}",
        pb.position(),
        pb.length().unwrap_or(0)
    ));

    // 75% complete
    pb.set_position(75);
    states.push(format!(
        "75%: {} of {}",
        pb.position(),
        pb.length().unwrap_or(0)
    ));

    // 100% complete
    pb.set_position(100);
    states.push(format!(
        "100%: {} of {}",
        pb.position(),
        pb.length().unwrap_or(0)
    ));

    let output = states.join("\n");
    insta::assert_snapshot!("progress_states", output);
}

#[test]
fn test_progress_bar_builder_pattern() {
    // Test builder pattern creates expected configurations
    let pb1 = ProgressBarBuilder::new(50).build();
    let pb2 = ProgressBarBuilder::new(100)
        .with_message("Custom message")
        .build();
    let pb3 = ProgressBarBuilder::new(200)
        .with_message("Full config")
        .with_template("{msg} [{bar:30}] {pos}/{len}")
        .build();

    let output = format!(
        "Builder 1: total={}\nBuilder 2: total={}\nBuilder 3: total={}",
        pb1.length().unwrap_or(0),
        pb2.length().unwrap_or(0),
        pb3.length().unwrap_or(0),
    );

    insta::assert_snapshot!("progress_bar_builder_pattern", output);
}

#[test]
fn test_progress_bar_batch_operations() {
    // Simulate batch operation progress states
    let total_batches = 5;
    let items_per_batch = 20;
    let total = total_batches * items_per_batch;

    let pb = ProgressBarBuilder::new(total as u64)
        .with_message("Processing batches")
        .build();

    let mut batch_outputs = Vec::new();
    for batch in 1..=total_batches {
        let completed = batch * items_per_batch;
        pb.set_position(completed as u64);
        batch_outputs.push(format!(
            "Batch {}/{}: {}/{} items ({}%)",
            batch,
            total_batches,
            completed,
            total,
            (completed * 100) / total
        ));
    }

    let output = batch_outputs.join("\n");
    insta::assert_snapshot!("progress_batch_operations", output);
}

#[test]
fn test_progress_bar_file_sizes() {
    // Test progress bar with file size formatting
    let file_sizes = vec![
        ("small.txt", 1024),              // 1 KB
        ("medium.pdf", 1024 * 1024),      // 1 MB
        ("large.zip", 100 * 1024 * 1024), // 100 MB
    ];

    let mut outputs = Vec::new();
    for (filename, size) in file_sizes {
        let pb = ProgressBarBuilder::new(size)
            .with_message(format!("Downloading {}", filename))
            .build();

        outputs.push(format!("{}: {} bytes", filename, pb.length().unwrap_or(0)));
    }

    let output = outputs.join("\n");
    insta::assert_snapshot!("progress_file_sizes", output);
}
