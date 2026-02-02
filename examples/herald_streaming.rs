//! Example: Streaming Herald formatters
//!
//! This example demonstrates the streaming behavior of different Herald formatters,
//! showing how each formatter handles progressive output.

use paladin::core::platform::container::herald::{ExecutionMetadata, Herald, StreamChunk};
use paladin::infrastructure::adapters::herald::{JsonHerald, MarkdownHerald, TableHerald};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Herald Streaming Example ===\n");

    // Simulate streaming chunks
    let chunks = vec![
        StreamChunk {
            content: "The capital of France is ".to_string(),
            is_final: false,
        },
        StreamChunk {
            content: "Paris. It is known for ".to_string(),
            is_final: false,
        },
        StreamChunk {
            content: "the Eiffel Tower, ".to_string(),
            is_final: false,
        },
        StreamChunk {
            content: "the Louvre Museum, ".to_string(),
            is_final: false,
        },
        StreamChunk {
            content: "and its rich cultural heritage.".to_string(),
            is_final: true,
        },
    ];

    let metadata = ExecutionMetadata {
        execution_time_ms: 2500,
        total_tokens: 125,
    };

    // Example 1: JSON Herald (NDJSON streaming)
    println!("--- Example 1: JSON Herald (NDJSON) ---\n");
    println!("Streaming behavior: Each chunk is a separate JSON object on a new line\n");
    {
        let herald: Arc<dyn Herald> = Arc::new(JsonHerald::new());

        println!("Simulating stream:");
        for (i, chunk) in chunks.iter().enumerate() {
            if let Some(formatted) = herald.format_stream_chunk(chunk)? {
                print!("{}", formatted);
                thread::sleep(Duration::from_millis(300)); // Simulate delay
            }
            if i < chunks.len() - 1 {
                println!("  <- Chunk {}", i + 1);
            }
        }

        let final_line = herald.finalize_stream(&metadata)?;
        println!("{}", final_line);
        println!("  <- Metadata\n");

        println!("Characteristics:");
        println!("  ✓ Each line is valid JSON");
        println!("  ✓ Can be parsed incrementally");
        println!("  ✓ Standard NDJSON format");
        println!("  ✓ Ideal for streaming APIs\n");
    }

    // Example 2: Markdown Herald (Progressive streaming)
    println!("--- Example 2: Markdown Herald (Progressive) ---\n");
    println!("Streaming behavior: Content appears immediately as plain text\n");
    {
        let herald: Arc<dyn Herald> = Arc::new(MarkdownHerald::new());

        println!("Simulating stream:");
        print!("Output: ");
        for chunk in &chunks {
            if let Some(formatted) = herald.format_stream_chunk(chunk)? {
                print!("{}", formatted);
                thread::sleep(Duration::from_millis(300)); // Simulate delay
            }
        }

        let final_line = herald.finalize_stream(&metadata)?;
        println!("\n{}\n", final_line);

        println!("Characteristics:");
        println!("  ✓ Immediate text visibility");
        println!("  ✓ Natural reading experience");
        println!("  ✓ Metadata appended at end");
        println!("  ✓ Ideal for CLI and terminals\n");
    }

    // Example 3: Table Herald (Buffered streaming)
    println!("--- Example 3: Table Herald (Buffered) ---\n");
    println!("Streaming behavior: Buffers all chunks, renders complete table at end\n");
    {
        let herald: Arc<dyn Herald> = Arc::new(TableHerald::default());

        println!("Simulating stream:");
        for (chunk_count, chunk) in chunks.iter().enumerate() {
            let result = herald.format_stream_chunk(chunk)?;
            if result.is_some() {
                println!("  Chunk {} returned content (unexpected!)", chunk_count + 1);
            } else {
                println!("  Chunk {} buffered (returns None)", chunk_count + 1);
            }
            thread::sleep(Duration::from_millis(300)); // Simulate delay
        }

        println!("\nFinalizing stream...");
        let final_output = herald.finalize_stream(&metadata)?;
        println!("{}\n", final_output);

        println!("Characteristics:");
        println!("  ✓ No partial output during streaming");
        println!("  ✓ Complete table rendered at end");
        println!("  ✓ Proper table formatting guaranteed");
        println!("  ✓ Ideal for dashboards and reports\n");
    }

    // Example 4: Comparing streaming strategies
    println!("--- Example 4: Strategy Comparison ---\n");
    {
        println!("┌─────────────┬──────────────┬─────────────────┬──────────────────┐");
        println!("│ Formatter   │ Strategy     │ Progressive?    │ Best For         │");
        println!("├─────────────┼──────────────┼─────────────────┼──────────────────┤");
        println!("│ JSON        │ NDJSON       │ ✓ Line-by-line  │ APIs, logging    │");
        println!("│ Markdown    │ Progressive  │ ✓ Immediate     │ CLI, humans      │");
        println!("│ Table       │ Buffered     │ ✗ Wait for end  │ Dashboards       │");
        println!("└─────────────┴──────────────┴─────────────────┴──────────────────┘\n");
    }

    // Example 5: Real-world streaming simulation
    println!("--- Example 5: Real-world Streaming Simulation ---\n");
    println!("Simulating a long-running analysis with progress updates:\n");
    {
        let herald: Arc<dyn Herald> = Arc::new(MarkdownHerald::new());

        let analysis_steps = vec![
            StreamChunk {
                content: "## Analysis Progress\n\n".to_string(),
                is_final: false,
            },
            StreamChunk {
                content: "1. Loading data... ✓\n".to_string(),
                is_final: false,
            },
            StreamChunk {
                content: "2. Preprocessing... ✓\n".to_string(),
                is_final: false,
            },
            StreamChunk {
                content: "3. Running analysis... ✓\n".to_string(),
                is_final: false,
            },
            StreamChunk {
                content: "4. Generating insights... ✓\n\n".to_string(),
                is_final: false,
            },
            StreamChunk {
                content: "**Results:**\n- Found 15 key patterns\n- Confidence: 92%\n- Recommendations: 8\n"
                    .to_string(),
                is_final: true,
            },
        ];

        for chunk in &analysis_steps {
            if let Some(formatted) = herald.format_stream_chunk(chunk)? {
                print!("{}", formatted);
                thread::sleep(Duration::from_millis(500)); // Simulate processing
            }
        }

        let analysis_metadata = ExecutionMetadata {
            execution_time_ms: 8500,
            total_tokens: 450,
        };

        println!("{}", herald.finalize_stream(&analysis_metadata)?);
    }

    // Example 6: Handling errors in streams
    println!("\n--- Example 6: Error Handling in Streams ---\n");
    {
        let herald: Arc<dyn Herald> = Arc::new(JsonHerald::new());

        println!("Processing chunks with potential errors:\n");

        let risky_chunks = vec![
            StreamChunk {
                content: "Processing... ".to_string(),
                is_final: false,
            },
            StreamChunk {
                content: "Warning: Unusual pattern detected. ".to_string(),
                is_final: false,
            },
            StreamChunk {
                content: "Continuing analysis... ".to_string(),
                is_final: false,
            },
            StreamChunk {
                content: "Completed successfully.".to_string(),
                is_final: true,
            },
        ];

        for chunk in &risky_chunks {
            match herald.format_stream_chunk(chunk) {
                Ok(Some(formatted)) => {
                    print!("{}", formatted);
                    println!("  <- Success");
                }
                Ok(None) => {
                    println!("  <- Buffered (no output yet)");
                }
                Err(e) => {
                    eprintln!("  <- Error: {}", e);
                }
            }
        }

        println!();
    }

    println!("=== End of Streaming Examples ===\n");
    println!("Key Takeaways:");
    println!("- JSON Herald: Line-by-line NDJSON for machine parsing");
    println!("- Markdown Herald: Progressive text for human consumption");
    println!("- Table Herald: Buffered for complete table rendering");
    println!("- Choose strategy based on your use case and audience");

    Ok(())
}
