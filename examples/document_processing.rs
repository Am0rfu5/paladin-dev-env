//! Document Processing Example
//!
//! Demonstrates PDF text extraction and intelligent document chunking.
//! This example shows how to:
//! - Extract text from PDF documents
//! - Access document metadata
//! - Chunk documents for RAG or analysis
//! - Process documents with Paladins
//!
//! Run with: `cargo run --example document_processing`

use paladin::infrastructure::adapters::document::DocumentAdapter;
use paladin::infrastructure::adapters::document::PdfExtractor;
use paladin_ports::input::document_port::{ChunkConfig, DocumentPort, DocumentSource};
use std::path::{Path, PathBuf};
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    env_logger::init();

    println!("📄 Sentinel Document Processing Example\n");

    // Step 1: Create PDF extractor
    println!("📋 Step 1: Creating PDF extractor...");
    let extractor = PdfExtractor::new();
    println!("✅ PDF extractor ready\n");

    // Step 2: Create document adapter
    println!("📋 Step 2: Creating document adapter...");
    let adapter: Arc<dyn DocumentPort> = Arc::new(DocumentAdapter::new());
    println!("✅ Document adapter ready\n");

    // Step 3: Check for sample PDF or create dummy content
    let pdf_path = PathBuf::from("./examples/assets/sample_document.pdf");

    if !pdf_path.exists() {
        println!("⚠️  Sample PDF not found, demonstrating with text content instead\n");
        demonstrate_text_processing(&adapter).await?;
    } else {
        demonstrate_pdf_processing(&adapter, &extractor, &pdf_path).await?;
    }

    println!();
    println!("{}", "=".repeat(80));
    println!("✅ Document processing example completed successfully!");
    println!("{}", "=".repeat(80));

    Ok(())
}

/// Demonstrate PDF processing with extraction and chunking
async fn demonstrate_pdf_processing(
    adapter: &Arc<dyn DocumentPort>,
    extractor: &PdfExtractor,
    pdf_path: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", "=".repeat(80));
    println!("📄 PDF EXTRACTION");
    println!("{}", "=".repeat(80));
    println!();

    // Extract text using low-level extractor
    println!("🔍 Extracting text from: {:?}", pdf_path);
    let document = extractor.extract(pdf_path)?;

    // Display metadata
    println!("📊 DOCUMENT METADATA");
    println!("{}", "─".repeat(80));
    println!("Title: {:?}", document.metadata.title);
    println!("Author: {:?}", document.metadata.author);
    println!("Pages: {}", document.metadata.page_count);
    println!("Created: {:?}", document.metadata.creation_date);
    println!("Total Characters: {}", document.total_chars);
    println!("{}", "─".repeat(80));
    println!();

    // Display page content
    println!("📑 PAGE CONTENT (First 3 pages):");
    for page in document.pages.iter().take(3) {
        println!(
            "\n┌─ Page {} ({} chars) ─┐",
            page.number,
            page.content.len()
        );

        // Show first 200 characters of each page
        let preview = if page.content.len() > 200 {
            format!("{}...", &page.content[..200])
        } else {
            page.content.clone()
        };

        println!("{}", preview);
        println!("└{}", "─".repeat(79));
    }

    // Step 4: Ingest document using adapter
    println!("\n📋 Step 4: Ingesting document via DocumentAdapter...");
    let ingested_doc = adapter
        .ingest(DocumentSource::File(pdf_path.to_path_buf()))
        .await?;
    println!("✅ Document ingested: {} pages\n", ingested_doc.pages.len());

    // Step 5: Demonstrate chunking
    demonstrate_chunking(adapter, &ingested_doc).await?;

    Ok(())
}

/// Demonstrate text file processing
async fn demonstrate_text_processing(
    adapter: &Arc<dyn DocumentPort>,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", "=".repeat(80));
    println!("📝 TEXT DOCUMENT PROCESSING");
    println!("{}", "=".repeat(80));
    println!();

    // Create sample text content
    let sample_text = r#"
# Artificial Intelligence and Machine Learning

Artificial Intelligence (AI) is revolutionizing how we interact with technology.
Machine learning, a subset of AI, enables computers to learn from data without
being explicitly programmed.

## Deep Learning

Deep learning uses neural networks with multiple layers to process complex patterns.
These networks have achieved remarkable results in computer vision, natural language
processing, and reinforcement learning.

## Applications

AI is being applied across various domains:

- Healthcare: Diagnostic systems and drug discovery
- Finance: Fraud detection and algorithmic trading
- Transportation: Autonomous vehicles
- Manufacturing: Predictive maintenance

## Challenges

Despite progress, AI faces several challenges:

1. Data quality and availability
2. Ethical considerations and bias
3. Interpretability of complex models
4. Computational requirements

## Future Directions

The future of AI includes:

- More efficient architectures
- Better generalization capabilities
- Enhanced human-AI collaboration
- Responsible AI development

AI continues to evolve rapidly, promising transformative changes across industries.
"#;

    // Save as temporary file for ingestion
    let temp_path = PathBuf::from("/tmp/sample_ai_document.txt");
    std::fs::write(&temp_path, sample_text)?;

    println!("📄 Created sample text document");
    println!("📏 Size: {} characters", sample_text.len());

    // Ingest the document
    println!("\n📋 Ingesting text document...");
    let document = adapter
        .ingest(DocumentSource::File(temp_path.clone()))
        .await?;

    println!("✅ Document ingested successfully");
    println!("   Pages: {}", document.pages.len());
    println!("   Total Characters: {}", document.total_chars);

    // Clean up temp file
    std::fs::remove_file(&temp_path).ok();

    // Demonstrate chunking
    demonstrate_chunking(adapter, &document).await?;

    Ok(())
}

/// Demonstrate intelligent document chunking for RAG
async fn demonstrate_chunking(
    adapter: &Arc<dyn DocumentPort>,
    document: &paladin::core::platform::container::document::Document,
) -> Result<(), Box<dyn std::error::Error>> {
    println!();
    println!("{}", "=".repeat(80));
    println!("✂️  INTELLIGENT DOCUMENT CHUNKING");
    println!("{}", "=".repeat(80));
    println!();

    // Configuration 1: Small chunks for precise RAG retrieval
    println!("📐 Configuration 1: Small Chunks (RAG-optimized)");
    let config_rag = ChunkConfig {
        chunk_size: 500,
        chunk_overlap: 100,
        separator: "\n\n".to_string(),
    };

    let chunks_rag = adapter.chunk(document, config_rag).await?;

    println!("   Chunk Size: 500 characters");
    println!("   Overlap: 100 characters (20%)");
    println!("   Separator: Double newline (paragraphs)");
    println!("   Total Chunks: {}\n", chunks_rag.len());

    display_chunks(&chunks_rag, 3);

    // Configuration 2: Large chunks for summarization
    println!("\n📐 Configuration 2: Large Chunks (Summarization)");
    let config_summary = ChunkConfig {
        chunk_size: 2000,
        chunk_overlap: 200,
        separator: "\n\n".to_string(),
    };

    let chunks_summary = adapter.chunk(document, config_summary).await?;

    println!("   Chunk Size: 2000 characters");
    println!("   Overlap: 200 characters (10%)");
    println!("   Separator: Double newline (paragraphs)");
    println!("   Total Chunks: {}\n", chunks_summary.len());

    display_chunks(&chunks_summary, 2);

    // Configuration 3: Sentence-based chunking
    println!("\n📐 Configuration 3: Sentence-Based Chunks");
    let config_sentence = ChunkConfig {
        chunk_size: 300,
        chunk_overlap: 50,
        separator: ". ".to_string(),
    };

    let chunks_sentence = adapter.chunk(document, config_sentence).await?;

    println!("   Chunk Size: 300 characters");
    println!("   Overlap: 50 characters (16%)");
    println!("   Separator: Period+space (sentences)");
    println!("   Total Chunks: {}\n", chunks_sentence.len());

    display_chunks(&chunks_sentence, 3);

    Ok(())
}

/// Helper function to display chunks
fn display_chunks(
    chunks: &[paladin_ports::input::document_port::DocumentChunk],
    max_display: usize,
) {
    for (i, chunk) in chunks.iter().take(max_display).enumerate() {
        println!("┌─ Chunk {} ─┐", i + 1);
        println!("│ Index: {}", chunk.chunk_index);
        println!("│ Size: {} chars", chunk.content.len());

        // Display first 150 characters
        let preview = if chunk.content.len() > 150 {
            format!("{}...", &chunk.content[..150].replace('\n', " "))
        } else {
            chunk.content.replace('\n', " ")
        };

        println!("│ Preview: {}", preview);
        println!("└{}", "─".repeat(79));
        println!();
    }

    if chunks.len() > max_display {
        println!("... and {} more chunks\n", chunks.len() - max_display);
    }
}
