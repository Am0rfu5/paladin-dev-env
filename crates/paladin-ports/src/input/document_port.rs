//! Document processing port for ingestion and chunking.
//!
//! This port defines the interface for document processing operations,
//! supporting various document sources and chunking strategies.

use async_trait::async_trait;
use paladin_core::platform::container::document::{Document, DocumentError};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Source for document ingestion.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum DocumentSource {
    /// Load document from a file path.
    File(PathBuf),
    /// Load document from bytes with explicit format.
    Bytes {
        /// Raw document data.
        data: Vec<u8>,
        /// Document format/extension (e.g., "pdf", "txt", "md").
        format: DocumentFormat,
    },
    /// Load document from a URL (future implementation).
    Url(String),
}

/// Document format specification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DocumentFormat {
    /// Portable Document Format.
    Pdf,
    /// Plain text file.
    Txt,
    /// Markdown file.
    Md,
}

/// Configuration for document chunking.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChunkConfig {
    /// Maximum size of each chunk in characters.
    pub chunk_size: usize,
    /// Number of overlapping characters between chunks.
    pub chunk_overlap: usize,
    /// Separator to use for splitting (e.g., "\n\n" for paragraphs).
    pub separator: String,
}

impl Default for ChunkConfig {
    fn default() -> Self {
        Self {
            chunk_size: 1000,
            chunk_overlap: 100,
            separator: "\n\n".to_string(),
        }
    }
}

impl ChunkConfig {
    /// Creates a new chunk configuration.
    pub fn new(chunk_size: usize, chunk_overlap: usize, separator: String) -> Self {
        Self {
            chunk_size,
            chunk_overlap,
            separator,
        }
    }

    /// Validates the configuration.
    pub fn validate(&self) -> Result<(), DocumentError> {
        if self.chunk_size == 0 {
            return Err(DocumentError::InvalidDocument(
                "Chunk size must be greater than 0".to_string(),
            ));
        }
        if self.chunk_overlap >= self.chunk_size {
            return Err(DocumentError::InvalidDocument(
                "Chunk overlap must be less than chunk size".to_string(),
            ));
        }
        Ok(())
    }
}

/// A chunk of a document.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocumentChunk {
    /// The text content of the chunk.
    pub content: String,
    /// Metadata about the chunk (e.g., source page, position).
    pub metadata: ChunkMetadata,
    /// Index of this chunk in the sequence.
    pub chunk_index: usize,
}

/// Metadata for a document chunk.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct ChunkMetadata {
    /// Source page number (if applicable).
    pub page_number: Option<usize>,
    /// Character position in the original document.
    pub char_position: usize,
    /// Total number of chunks.
    pub total_chunks: Option<usize>,
}

/// Port for document processing operations.
///
/// Provides methods for ingesting documents from various sources
/// and chunking them for processing by LLMs.
#[async_trait]
pub trait DocumentPort: Send + Sync {
    /// Ingests a document from the specified source.
    ///
    /// # Errors
    ///
    /// Returns `DocumentError` if the document cannot be loaded or parsed.
    async fn ingest(&self, source: DocumentSource) -> Result<Document, DocumentError>;

    /// Splits a document into chunks according to the configuration.
    ///
    /// # Errors
    ///
    /// Returns `DocumentError` if chunking fails or configuration is invalid.
    async fn chunk(
        &self,
        document: &Document,
        config: ChunkConfig,
    ) -> Result<Vec<DocumentChunk>, DocumentError>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_document_source_file() {
        let source = DocumentSource::File(PathBuf::from("/path/to/doc.pdf"));
        match source {
            DocumentSource::File(path) => {
                assert_eq!(path, PathBuf::from("/path/to/doc.pdf"));
            }
            _ => panic!("Expected File variant"),
        }
    }

    #[test]
    fn test_document_source_bytes() {
        let data = vec![1, 2, 3, 4, 5];
        let source = DocumentSource::Bytes {
            data: data.clone(),
            format: DocumentFormat::Pdf,
        };

        match source {
            DocumentSource::Bytes { data: d, format } => {
                assert_eq!(d, data);
                assert_eq!(format, DocumentFormat::Pdf);
            }
            _ => panic!("Expected Bytes variant"),
        }
    }

    #[test]
    fn test_document_source_url() {
        let source = DocumentSource::Url("https://example.com/doc.pdf".to_string());
        match source {
            DocumentSource::Url(url) => {
                assert_eq!(url, "https://example.com/doc.pdf");
            }
            _ => panic!("Expected Url variant"),
        }
    }

    #[test]
    fn test_document_format() {
        assert_eq!(DocumentFormat::Pdf, DocumentFormat::Pdf);
        assert_eq!(DocumentFormat::Txt, DocumentFormat::Txt);
        assert_eq!(DocumentFormat::Md, DocumentFormat::Md);

        // Test serialization
        let json = serde_json::to_string(&DocumentFormat::Pdf).unwrap();
        assert_eq!(json, "\"pdf\"");
    }

    #[test]
    fn test_chunk_config_default() {
        let config = ChunkConfig::default();
        assert_eq!(config.chunk_size, 1000);
        assert_eq!(config.chunk_overlap, 100);
        assert_eq!(config.separator, "\n\n");
    }

    #[test]
    fn test_chunk_config_custom() {
        let config = ChunkConfig::new(500, 50, "\n".to_string());
        assert_eq!(config.chunk_size, 500);
        assert_eq!(config.chunk_overlap, 50);
        assert_eq!(config.separator, "\n");
    }

    #[test]
    fn test_chunk_config_validation() {
        let valid_config = ChunkConfig::new(1000, 100, "\n\n".to_string());
        assert!(valid_config.validate().is_ok());

        let invalid_size = ChunkConfig::new(0, 0, "\n\n".to_string());
        assert!(invalid_size.validate().is_err());

        let invalid_overlap = ChunkConfig::new(100, 100, "\n\n".to_string());
        assert!(invalid_overlap.validate().is_err());

        let invalid_overlap2 = ChunkConfig::new(100, 150, "\n\n".to_string());
        assert!(invalid_overlap2.validate().is_err());
    }

    #[test]
    fn test_document_chunk() {
        let metadata = ChunkMetadata {
            page_number: Some(1),
            char_position: 0,
            total_chunks: Some(5),
        };

        let chunk = DocumentChunk {
            content: "This is a chunk of text.".to_string(),
            metadata: metadata.clone(),
            chunk_index: 0,
        };

        assert_eq!(chunk.content, "This is a chunk of text.");
        assert_eq!(chunk.chunk_index, 0);
        assert_eq!(chunk.metadata, metadata);
    }

    #[test]
    fn test_chunk_metadata() {
        let metadata = ChunkMetadata {
            page_number: Some(3),
            char_position: 1500,
            total_chunks: Some(10),
        };

        assert_eq!(metadata.page_number, Some(3));
        assert_eq!(metadata.char_position, 1500);
        assert_eq!(metadata.total_chunks, Some(10));
    }

    #[test]
    fn test_document_chunk_serialization() {
        let chunk = DocumentChunk {
            content: "Test content".to_string(),
            metadata: ChunkMetadata::default(),
            chunk_index: 0,
        };

        let json = serde_json::to_string(&chunk).unwrap();
        let deserialized: DocumentChunk = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized, chunk);
    }
}
