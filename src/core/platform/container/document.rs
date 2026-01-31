//! Document processing types for multi-modal AI agent processing.
//!
//! This module provides data structures for handling document inputs,
//! particularly PDF documents, with support for text extraction and metadata.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// A single page from a document.
///
/// Contains the page number and extracted text content.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Page {
    /// Page number (1-indexed).
    pub number: usize,
    /// Extracted text content from the page.
    pub content: String,
}

impl Page {
    /// Creates a new page.
    pub fn new(number: usize, content: String) -> Self {
        Self { number, content }
    }
}

/// Metadata about a document.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct DocumentMetadata {
    /// Document title, if available.
    pub title: Option<String>,
    /// Document author, if available.
    pub author: Option<String>,
    /// Total number of pages.
    pub page_count: usize,
    /// Document creation date, if available.
    pub creation_date: Option<DateTime<Utc>>,
}

impl DocumentMetadata {
    /// Creates new document metadata.
    pub fn new(page_count: usize) -> Self {
        Self {
            title: None,
            author: None,
            page_count,
            creation_date: None,
        }
    }

    /// Sets the title.
    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Sets the author.
    pub fn with_author(mut self, author: impl Into<String>) -> Self {
        self.author = Some(author.into());
        self
    }

    /// Sets the creation date.
    pub fn with_creation_date(mut self, date: DateTime<Utc>) -> Self {
        self.creation_date = Some(date);
        self
    }
}

/// A complete document with pages and metadata.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Document {
    /// Vector of pages in the document.
    pub pages: Vec<Page>,
    /// Document metadata.
    pub metadata: DocumentMetadata,
    /// Total character count across all pages.
    pub total_chars: usize,
}

impl Document {
    /// Creates a new document.
    pub fn new(pages: Vec<Page>, metadata: DocumentMetadata) -> Self {
        let total_chars = pages.iter().map(|p| p.content.len()).sum();
        Self {
            pages,
            metadata,
            total_chars,
        }
    }

    /// Returns the number of pages in the document.
    pub fn page_count(&self) -> usize {
        self.pages.len()
    }

    /// Returns the total word count (approximate).
    pub fn word_count(&self) -> usize {
        self.pages
            .iter()
            .map(|p| p.content.split_whitespace().count())
            .sum()
    }
}

/// Document-specific errors.
#[derive(Debug, thiserror::Error)]
pub enum DocumentError {
    /// Unsupported document format.
    #[error("Unsupported document format: {0}")]
    UnsupportedFormat(String),

    /// PDF is encrypted and requires a password.
    #[error("PDF is encrypted and requires a password")]
    EncryptedPdf,

    /// Document file is corrupted or malformed.
    #[error("Document file is corrupted: {0}")]
    CorruptedFile(String),

    /// Text extraction failed.
    #[error("Text extraction failed: {0}")]
    ExtractionFailed(String),

    /// IO error.
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    /// Invalid document data.
    #[error("Invalid document: {0}")]
    InvalidDocument(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_page_creation() {
        let page = Page::new(1, "This is page one content.".to_string());
        assert_eq!(page.number, 1);
        assert_eq!(page.content, "This is page one content.");
    }

    #[test]
    fn test_page_serialization() {
        let page = Page::new(2, "Content".to_string());
        let json = serde_json::to_string(&page).unwrap();
        let deserialized: Page = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized, page);
    }

    #[test]
    fn test_document_metadata_creation() {
        let metadata = DocumentMetadata::new(10);
        assert_eq!(metadata.page_count, 10);
        assert!(metadata.title.is_none());
        assert!(metadata.author.is_none());
        assert!(metadata.creation_date.is_none());
    }

    #[test]
    fn test_document_metadata_builder() {
        let now = Utc::now();
        let metadata = DocumentMetadata::new(5)
            .with_title("Test Document")
            .with_author("John Doe")
            .with_creation_date(now);

        assert_eq!(metadata.title, Some("Test Document".to_string()));
        assert_eq!(metadata.author, Some("John Doe".to_string()));
        assert_eq!(metadata.creation_date, Some(now));
        assert_eq!(metadata.page_count, 5);
    }

    #[test]
    fn test_document_creation() {
        let pages = vec![
            Page::new(1, "First page content.".to_string()),
            Page::new(2, "Second page content.".to_string()),
        ];
        let metadata = DocumentMetadata::new(2).with_title("Test Doc");

        let document = Document::new(pages.clone(), metadata.clone());

        assert_eq!(document.pages.len(), 2);
        assert_eq!(document.metadata, metadata);
        assert_eq!(document.page_count(), 2);
        assert_eq!(
            document.total_chars,
            "First page content.".len() + "Second page content.".len()
        );
    }

    #[test]
    fn test_document_word_count() {
        let pages = vec![
            Page::new(1, "This is page one.".to_string()),
            Page::new(2, "This is page two.".to_string()),
        ];
        let metadata = DocumentMetadata::new(2);
        let document = Document::new(pages, metadata);

        assert_eq!(document.word_count(), 8); // 4 words per page
    }

    #[test]
    fn test_document_serialization() {
        let pages = vec![Page::new(1, "Content".to_string())];
        let metadata = DocumentMetadata::new(1).with_title("Test");
        let document = Document::new(pages, metadata);

        let json = serde_json::to_string(&document).unwrap();
        let deserialized: Document = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized, document);
    }
}
