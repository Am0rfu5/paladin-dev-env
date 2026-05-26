//! Document adapter implementing the DocumentPort trait.
//!
//! This adapter provides document ingestion and chunking capabilities,
//! supporting PDF, TXT, and MD file formats.

use crate::adapters::document::PdfExtractor;
use async_trait::async_trait;
use paladin_core::platform::container::document::{
    Document, DocumentError, DocumentMetadata, Page,
};
use paladin_ports::input::document_port::{
    ChunkConfig, ChunkMetadata, DocumentChunk, DocumentFormat, DocumentPort, DocumentSource,
};
use std::path::Path;

/// Document adapter for ingestion and chunking.
///
/// Handles PDF, TXT, and MD file formats with configurable chunking strategies.
#[doc(hidden)]
#[derive(Debug, Clone)]
pub struct DocumentAdapter {
    pdf_extractor: PdfExtractor,
}

impl DocumentAdapter {
    /// Creates a new document adapter.
    pub fn new() -> Self {
        Self {
            pdf_extractor: PdfExtractor::new(),
        }
    }

    /// Detects the format of a file based on its extension.
    fn detect_format(path: &Path) -> Result<DocumentFormat, DocumentError> {
        let extension = path.extension().and_then(|e| e.to_str()).ok_or_else(|| {
            DocumentError::UnsupportedFormat("No file extension found".to_string())
        })?;

        match extension.to_lowercase().as_str() {
            "pdf" => Ok(DocumentFormat::Pdf),
            "txt" => Ok(DocumentFormat::Txt),
            "md" | "markdown" => Ok(DocumentFormat::Md),
            _ => Err(DocumentError::UnsupportedFormat(format!(
                "Unsupported file extension: {}",
                extension
            ))),
        }
    }

    /// Ingests a TXT file.
    async fn ingest_txt(&self, path: &Path) -> Result<Document, DocumentError> {
        let content = tokio::fs::read_to_string(path).await.map_err(|e| {
            DocumentError::IoError(std::io::Error::new(
                e.kind(),
                format!("Failed to read TXT file: {}", e),
            ))
        })?;

        // Create a single page document
        let page = Page::new(1, content);
        let metadata = DocumentMetadata::new(1);

        Ok(Document::new(vec![page], metadata))
    }

    /// Ingests a TXT file from bytes.
    async fn ingest_txt_bytes(&self, bytes: &[u8]) -> Result<Document, DocumentError> {
        let content = String::from_utf8(bytes.to_vec()).map_err(|e| {
            DocumentError::ExtractionFailed(format!("Invalid UTF-8 in TXT file: {}", e))
        })?;

        let page = Page::new(1, content);
        let metadata = DocumentMetadata::new(1);

        Ok(Document::new(vec![page], metadata))
    }

    /// Ingests a Markdown file.
    async fn ingest_markdown(&self, path: &Path) -> Result<Document, DocumentError> {
        let content = tokio::fs::read_to_string(path).await.map_err(|e| {
            DocumentError::IoError(std::io::Error::new(
                e.kind(),
                format!("Failed to read Markdown file: {}", e),
            ))
        })?;

        // Create a single page document
        let page = Page::new(1, content);
        let metadata = DocumentMetadata::new(1);

        Ok(Document::new(vec![page], metadata))
    }

    /// Ingests a Markdown file from bytes.
    async fn ingest_markdown_bytes(&self, bytes: &[u8]) -> Result<Document, DocumentError> {
        let content = String::from_utf8(bytes.to_vec()).map_err(|e| {
            DocumentError::ExtractionFailed(format!("Invalid UTF-8 in Markdown file: {}", e))
        })?;

        let page = Page::new(1, content);
        let metadata = DocumentMetadata::new(1);

        Ok(Document::new(vec![page], metadata))
    }
}

impl Default for DocumentAdapter {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl DocumentPort for DocumentAdapter {
    async fn ingest(&self, source: DocumentSource) -> Result<Document, DocumentError> {
        match source {
            DocumentSource::File(path) => {
                let format = Self::detect_format(&path)?;

                match format {
                    DocumentFormat::Pdf => {
                        // Use PdfExtractor for PDF files
                        self.pdf_extractor.extract(&path)
                    }
                    DocumentFormat::Txt => self.ingest_txt(&path).await,
                    DocumentFormat::Md => self.ingest_markdown(&path).await,
                }
            }
            DocumentSource::Bytes { data, format } => match format {
                DocumentFormat::Pdf => {
                    // Use PdfExtractor for PDF bytes
                    self.pdf_extractor.extract_bytes(&data)
                }
                DocumentFormat::Txt => self.ingest_txt_bytes(&data).await,
                DocumentFormat::Md => self.ingest_markdown_bytes(&data).await,
            },
            DocumentSource::Url(_url) => Err(DocumentError::UnsupportedFormat(
                "URL sources are not yet supported".to_string(),
            )),
        }
    }

    async fn chunk(
        &self,
        document: &Document,
        config: ChunkConfig,
    ) -> Result<Vec<DocumentChunk>, DocumentError> {
        // Validate configuration
        config.validate()?;

        // Combine all pages into a single text
        let full_text: String = document
            .pages
            .iter()
            .map(|page| page.content.as_str())
            .collect::<Vec<_>>()
            .join(&config.separator);

        // Split text into chunks
        let mut chunks = Vec::new();
        let mut char_position = 0;

        // Split by separator first
        let segments: Vec<&str> = full_text.split(&config.separator).collect();

        let mut current_chunk = String::new();
        let mut current_chunk_start = 0;

        for segment in segments {
            // Check if adding this segment would exceed chunk size
            if !current_chunk.is_empty()
                && current_chunk.len() + config.separator.len() + segment.len() > config.chunk_size
            {
                // Save current chunk
                if !current_chunk.is_empty() {
                    chunks.push(DocumentChunk {
                        content: current_chunk.clone(),
                        metadata: ChunkMetadata {
                            page_number: None, // Will be set after all chunks are created
                            char_position: current_chunk_start,
                            total_chunks: None, // Will be set after all chunks are created
                        },
                        chunk_index: chunks.len(),
                    });

                    // Handle overlap: keep last N characters
                    if config.chunk_overlap > 0 {
                        let overlap_len = if current_chunk.len() > config.chunk_overlap {
                            config.chunk_overlap
                        } else {
                            current_chunk.len()
                        };
                        let start_pos = current_chunk.len() - overlap_len;
                        let overlap_text = current_chunk[start_pos..].to_string();
                        current_chunk = overlap_text;
                        current_chunk_start = char_position - overlap_len;
                    } else {
                        current_chunk.clear();
                        current_chunk_start = char_position;
                    }
                }
            }

            // Add segment to current chunk
            if !current_chunk.is_empty() {
                current_chunk.push_str(&config.separator);
                char_position += config.separator.len();
            }
            current_chunk.push_str(segment);
            char_position += segment.len();
        }

        // Add the last chunk if not empty
        if !current_chunk.trim().is_empty() {
            chunks.push(DocumentChunk {
                content: current_chunk,
                metadata: ChunkMetadata {
                    page_number: None,
                    char_position: current_chunk_start,
                    total_chunks: None,
                },
                chunk_index: chunks.len(),
            });
        }

        // Update total_chunks in all metadata
        let total_chunks = chunks.len();
        for chunk in &mut chunks {
            chunk.metadata.total_chunks = Some(total_chunks);
        }

        Ok(chunks)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_document_adapter_creation() {
        let adapter = DocumentAdapter::new();
        assert!(adapter.pdf_extractor.extract_bytes(&[]).is_err());
    }

    #[test]
    fn test_document_adapter_default() {
        let adapter = DocumentAdapter::default();
        assert!(adapter.pdf_extractor.extract_bytes(&[]).is_err());
    }

    #[test]
    fn test_detect_format_pdf() {
        let path = Path::new("/path/to/file.pdf");
        let format = DocumentAdapter::detect_format(path).unwrap();
        assert_eq!(format, DocumentFormat::Pdf);
    }

    #[test]
    fn test_detect_format_txt() {
        let path = Path::new("/path/to/file.txt");
        let format = DocumentAdapter::detect_format(path).unwrap();
        assert_eq!(format, DocumentFormat::Txt);
    }

    #[test]
    fn test_detect_format_md() {
        let path = Path::new("/path/to/file.md");
        let format = DocumentAdapter::detect_format(path).unwrap();
        assert_eq!(format, DocumentFormat::Md);
    }

    #[test]
    fn test_detect_format_markdown() {
        let path = Path::new("/path/to/file.markdown");
        let format = DocumentAdapter::detect_format(path).unwrap();
        assert_eq!(format, DocumentFormat::Md);
    }

    #[test]
    fn test_detect_format_case_insensitive() {
        let path = Path::new("/path/to/file.PDF");
        let format = DocumentAdapter::detect_format(path).unwrap();
        assert_eq!(format, DocumentFormat::Pdf);
    }

    #[test]
    fn test_detect_format_unsupported() {
        let path = Path::new("/path/to/file.docx");
        let result = DocumentAdapter::detect_format(path);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            DocumentError::UnsupportedFormat(_)
        ));
    }

    #[test]
    fn test_detect_format_no_extension() {
        let path = Path::new("/path/to/file");
        let result = DocumentAdapter::detect_format(path);
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_ingest_txt_bytes() {
        let adapter = DocumentAdapter::new();
        let content = b"This is a test TXT file.";

        let source = DocumentSource::Bytes {
            data: content.to_vec(),
            format: DocumentFormat::Txt,
        };

        let result = adapter.ingest(source).await.unwrap();
        assert_eq!(result.page_count(), 1);
        assert!(result.pages[0].content.contains("test TXT file"));
    }

    #[tokio::test]
    async fn test_ingest_markdown_bytes() {
        let adapter = DocumentAdapter::new();
        let content = b"# Test Markdown\n\nThis is a **test** MD file.";

        let source = DocumentSource::Bytes {
            data: content.to_vec(),
            format: DocumentFormat::Md,
        };

        let result = adapter.ingest(source).await.unwrap();
        assert_eq!(result.page_count(), 1);
        assert!(result.pages[0].content.contains("Test Markdown"));
        assert!(result.pages[0].content.contains("**test**"));
    }

    #[tokio::test]
    async fn test_ingest_url_unsupported() {
        let adapter = DocumentAdapter::new();
        let source = DocumentSource::Url("https://example.com/doc.pdf".to_string());

        let result = adapter.ingest(source).await;
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            DocumentError::UnsupportedFormat(_)
        ));
    }

    #[tokio::test]
    async fn test_chunk_default_config() {
        let adapter = DocumentAdapter::new();

        let page = Page::new(1, "This is a test document with some content.".to_string());
        let metadata = DocumentMetadata::new(1);
        let document = Document::new(vec![page], metadata);

        let config = ChunkConfig::default();
        let chunks = adapter.chunk(&document, config).await.unwrap();

        assert_eq!(chunks.len(), 1); // Short text = 1 chunk
        assert!(chunks[0].content.contains("test document"));
    }

    #[tokio::test]
    async fn test_chunk_custom_size() {
        let adapter = DocumentAdapter::new();

        // Create a long document with separators
        let paragraphs: Vec<String> = (0..10)
            .map(|i| format!("Paragraph {} with some content to make it longer", i))
            .collect();
        let long_text = paragraphs.join("\n\n");
        let page = Page::new(1, long_text);
        let metadata = DocumentMetadata::new(1);
        let document = Document::new(vec![page], metadata);

        let config = ChunkConfig::new(100, 20, "\n\n".to_string());
        let chunks = adapter.chunk(&document, config).await.unwrap();

        // Should have multiple chunks since we have many paragraphs
        assert!(chunks.len() > 1);
        for chunk in &chunks {
            // Each chunk should be around chunk_size (may vary due to separator logic)
            assert!(chunk.content.len() <= 200); // Allow some tolerance
        }
    }

    #[tokio::test]
    async fn test_chunk_with_overlap() {
        let adapter = DocumentAdapter::new();

        let text = "First paragraph.\n\nSecond paragraph.\n\nThird paragraph.\n\nFourth paragraph.";
        let page = Page::new(1, text.to_string());
        let metadata = DocumentMetadata::new(1);
        let document = Document::new(vec![page], metadata);

        let config = ChunkConfig::new(30, 10, "\n\n".to_string());
        let chunks = adapter.chunk(&document, config).await.unwrap();

        // With overlap, chunks should have some overlapping content
        assert!(chunks.len() >= 2);
    }

    #[tokio::test]
    async fn test_chunk_custom_separator() {
        let adapter = DocumentAdapter::new();

        let text = "Sentence one. Sentence two. Sentence three. Sentence four.";
        let page = Page::new(1, text.to_string());
        let metadata = DocumentMetadata::new(1);
        let document = Document::new(vec![page], metadata);

        let config = ChunkConfig::new(100, 0, ". ".to_string());
        let chunks = adapter.chunk(&document, config).await.unwrap();

        // Should split by period
        assert!(!chunks.is_empty());
    }

    #[tokio::test]
    async fn test_chunk_metadata() {
        let adapter = DocumentAdapter::new();

        let page = Page::new(1, "Test content for metadata.".to_string());
        let metadata = DocumentMetadata::new(1);
        let document = Document::new(vec![page], metadata);

        let config = ChunkConfig::default();
        let chunks = adapter.chunk(&document, config).await.unwrap();

        assert_eq!(chunks[0].chunk_index, 0);
        assert_eq!(chunks[0].metadata.char_position, 0);
        assert_eq!(chunks[0].metadata.total_chunks, Some(chunks.len()));
    }

    #[tokio::test]
    async fn test_chunk_invalid_config() {
        let adapter = DocumentAdapter::new();

        let page = Page::new(1, "Test".to_string());
        let metadata = DocumentMetadata::new(1);
        let document = Document::new(vec![page], metadata);

        // Invalid: chunk_size = 0
        let invalid_config = ChunkConfig::new(0, 0, "\n\n".to_string());
        let result = adapter.chunk(&document, invalid_config).await;
        assert!(result.is_err());

        // Invalid: overlap >= size
        let invalid_config2 = ChunkConfig::new(100, 100, "\n\n".to_string());
        let result2 = adapter.chunk(&document, invalid_config2).await;
        assert!(result2.is_err());
    }

    #[test]
    fn test_thread_safety() {
        // DocumentAdapter should be Send + Sync
        fn assert_send<T: Send>() {}
        fn assert_sync<T: Sync>() {}

        assert_send::<DocumentAdapter>();
        assert_sync::<DocumentAdapter>();
    }

    #[tokio::test]
    async fn test_arc_document_adapter() {
        use std::sync::Arc;

        let adapter = Arc::new(DocumentAdapter::new());
        let adapter_clone = Arc::clone(&adapter);

        // Can use from multiple contexts
        let content = b"Test content";
        let source = DocumentSource::Bytes {
            data: content.to_vec(),
            format: DocumentFormat::Txt,
        };

        let result = adapter_clone.ingest(source).await;
        assert!(result.is_ok());
    }
}
