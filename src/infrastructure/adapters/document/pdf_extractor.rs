//! PDF text extraction implementation.
//!
//! This module provides utilities for extracting text and metadata from PDF files
//! using the `pdf-extract` library.

use crate::core::platform::container::document::{Document, DocumentError, DocumentMetadata, Page};
use std::path::Path;

/// PDF text extractor.
///
/// Handles extraction of text content and metadata from PDF files.
/// Supports both file path and byte array input.
#[doc(hidden)]
#[derive(Debug, Clone, Default)]
pub struct PdfExtractor;

impl PdfExtractor {
    /// Creates a new PDF extractor.
    pub fn new() -> Self {
        Self
    }

    /// Extracts text and metadata from a PDF file.
    ///
    /// # Arguments
    ///
    /// * `path` - Path to the PDF file
    ///
    /// # Returns
    ///
    /// A `Document` containing extracted pages and metadata, or an error if extraction fails.
    ///
    /// # Errors
    ///
    /// Returns `DocumentError::EncryptedPdf` if the PDF requires a password.
    /// Returns `DocumentError::CorruptedFile` if the PDF is malformed.
    /// Returns `DocumentError::ExtractionFailed` if text extraction fails.
    pub fn extract(&self, path: &Path) -> Result<Document, DocumentError> {
        // Read file to bytes
        let bytes = std::fs::read(path).map_err(|e| {
            DocumentError::IoError(std::io::Error::new(
                e.kind(),
                format!("Failed to read PDF file: {}", e),
            ))
        })?;

        self.extract_bytes(&bytes)
    }

    /// Extracts text and metadata from PDF bytes.
    ///
    /// # Arguments
    ///
    /// * `bytes` - PDF file content as byte array
    ///
    /// # Returns
    ///
    /// A `Document` containing extracted pages and metadata, or an error if extraction fails.
    ///
    /// # Errors
    ///
    /// Returns `DocumentError::EncryptedPdf` if the PDF requires a password.
    /// Returns `DocumentError::CorruptedFile` if the PDF is malformed.
    /// Returns `DocumentError::ExtractionFailed` if text extraction fails.
    pub fn extract_bytes(&self, bytes: &[u8]) -> Result<Document, DocumentError> {
        // Extract text using pdf-extract
        let extracted_text = pdf_extract::extract_text_from_mem(bytes).map_err(|e| {
            let error_msg = e.to_string();

            // Check for encrypted PDF
            if error_msg.contains("encrypted") || error_msg.contains("password") {
                return DocumentError::EncryptedPdf;
            }

            // Check for corrupted/malformed PDF
            if error_msg.contains("invalid") || error_msg.contains("corrupt") {
                return DocumentError::CorruptedFile(error_msg);
            }

            DocumentError::ExtractionFailed(error_msg)
        })?;

        // Parse the extracted text into pages
        // pdf-extract returns all text as a single string, so we need to split it
        let pages = self.parse_text_into_pages(&extracted_text);

        // Extract metadata
        let metadata = self.extract_metadata_from_bytes(bytes, pages.len())?;

        Ok(Document::new(pages, metadata))
    }

    /// Parses extracted text into individual pages.
    ///
    /// Since pdf-extract returns all text as one string, we attempt to detect
    /// page breaks by looking for form feed characters or multiple consecutive newlines.
    fn parse_text_into_pages(&self, text: &str) -> Vec<Page> {
        // Handle empty text
        if text.trim().is_empty() {
            return Vec::new();
        }

        // Split on form feed character (page break)
        let page_texts: Vec<&str> = text.split('\x0C').collect();

        if page_texts.len() > 1 {
            // Form feed characters found - use those as page breaks
            page_texts
                .into_iter()
                .enumerate()
                .filter(|(_, content)| !content.trim().is_empty())
                .map(|(idx, content)| Page::new(idx + 1, self.preserve_structure(content)))
                .collect()
        } else {
            // No form feed - try to split on multiple newlines (paragraph breaks)
            // This is a heuristic and may not perfectly match actual PDF pages
            let paragraphs: Vec<&str> = text.split("\n\n\n").collect();

            if paragraphs.len() > 1 {
                paragraphs
                    .into_iter()
                    .enumerate()
                    .filter(|(_, content)| !content.trim().is_empty())
                    .map(|(idx, content)| Page::new(idx + 1, self.preserve_structure(content)))
                    .collect()
            } else {
                // Single page or unable to detect breaks - treat as one page
                vec![Page::new(1, self.preserve_structure(text))]
            }
        }
    }

    /// Preserves text structure (paragraphs, spacing).
    ///
    /// Cleans up excessive whitespace while preserving paragraph breaks.
    fn preserve_structure(&self, text: &str) -> String {
        // Split into lines
        let lines: Vec<&str> = text.lines().collect();

        let mut result = String::new();
        let mut prev_empty = false;

        for line in lines {
            let trimmed = line.trim();

            if trimmed.is_empty() {
                // Empty line - preserve one newline for paragraph break
                if !prev_empty && !result.is_empty() {
                    result.push('\n');
                }
                prev_empty = true;
            } else {
                // Non-empty line - add it with newline
                if !result.is_empty() && !prev_empty {
                    result.push(' '); // Join within paragraph
                } else if !result.is_empty() {
                    result.push('\n'); // Paragraph break
                }
                result.push_str(trimmed);
                prev_empty = false;
            }
        }

        result
    }

    /// Extracts metadata from PDF bytes.
    fn extract_metadata_from_bytes(
        &self,
        _bytes: &[u8],
        page_count: usize,
    ) -> Result<DocumentMetadata, DocumentError> {
        // pdf-extract doesn't provide metadata extraction
        // For now, we'll return basic metadata with just page count
        // In a production implementation, you'd use a library like `lopdf` for metadata

        Ok(DocumentMetadata {
            title: None,
            author: None,
            page_count,
            creation_date: None,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pdf_extractor_creation() {
        let extractor = PdfExtractor::new();
        assert!(matches!(extractor, PdfExtractor));
    }

    #[test]
    fn test_pdf_extractor_default() {
        let extractor = PdfExtractor;
        assert!(matches!(extractor, PdfExtractor));
    }

    #[test]
    fn test_extract_bytes_invalid_pdf() {
        let extractor = PdfExtractor::new();
        let invalid_bytes = b"This is not a valid PDF";

        let result = extractor.extract_bytes(invalid_bytes);
        assert!(result.is_err());

        // Should return ExtractionFailed for invalid PDF
        match result {
            Err(DocumentError::ExtractionFailed(_)) => (),
            Err(DocumentError::CorruptedFile(_)) => (),
            _ => panic!("Expected ExtractionFailed or CorruptedFile error"),
        }
    }

    #[test]
    fn test_extract_nonexistent_file() {
        let extractor = PdfExtractor::new();
        let path = Path::new("/nonexistent/file.pdf");

        let result = extractor.extract(path);
        assert!(result.is_err());

        // Should return IoError for nonexistent file
        match result {
            Err(DocumentError::IoError(_)) => (),
            _ => panic!("Expected IoError for nonexistent file"),
        }
    }

    #[test]
    fn test_parse_text_into_pages_single_page() {
        let extractor = PdfExtractor::new();
        let text = "This is page one content.";
        let pages = extractor.parse_text_into_pages(text);

        assert_eq!(pages.len(), 1);
        assert_eq!(pages[0].number, 1);
        assert!(pages[0].content.contains("page one"));
    }

    #[test]
    fn test_parse_text_into_pages_empty_text() {
        let extractor = PdfExtractor::new();
        let text = "";
        let pages = extractor.parse_text_into_pages(text);

        // Empty text should return empty vec (filtered out)
        assert_eq!(pages.len(), 0);
    }

    #[test]
    fn test_parse_text_into_pages_with_form_feed() {
        let extractor = PdfExtractor::new();
        let text = "Page one content.\x0CPage two content.\x0CPage three content.";
        let pages = extractor.parse_text_into_pages(text);

        assert_eq!(pages.len(), 3);
        assert_eq!(pages[0].number, 1);
        assert!(pages[0].content.contains("Page one"));
        assert_eq!(pages[1].number, 2);
        assert!(pages[1].content.contains("Page two"));
        assert_eq!(pages[2].number, 3);
        assert!(pages[2].content.contains("Page three"));
    }

    #[test]
    fn test_parse_text_into_pages_with_triple_newlines() {
        let extractor = PdfExtractor::new();
        let text = "Section one content.\n\n\nSection two content.\n\n\nSection three content.";
        let pages = extractor.parse_text_into_pages(text);

        // Should split on triple newlines
        assert!(!pages.is_empty());
        assert_eq!(pages[0].number, 1);
    }

    #[test]
    fn test_preserve_structure_basic() {
        let extractor = PdfExtractor::new();
        let text = "Line one\nLine two\n\nNew paragraph\nContinuation";
        let result = extractor.preserve_structure(text);

        // Should preserve paragraph breaks
        assert!(result.contains("Line one"));
        assert!(result.contains("paragraph"));
        assert!(result.contains("Continuation"));
    }

    #[test]
    fn test_preserve_structure_excessive_whitespace() {
        let extractor = PdfExtractor::new();
        let text = "Line one  \n\n\n\n  Line two";
        let result = extractor.preserve_structure(text);

        // Should clean up excessive whitespace
        assert!(result.contains("Line one"));
        assert!(result.contains("Line two"));
        // Should not have multiple consecutive spaces
        assert!(!result.contains("  "));
    }

    #[test]
    fn test_preserve_structure_empty() {
        let extractor = PdfExtractor::new();
        let text = "";
        let result = extractor.preserve_structure(text);

        assert_eq!(result, "");
    }

    #[test]
    fn test_preserve_structure_only_whitespace() {
        let extractor = PdfExtractor::new();
        let text = "   \n\n   \n   ";
        let result = extractor.preserve_structure(text);

        // Should return empty string for whitespace-only input
        assert_eq!(result, "");
    }

    #[test]
    fn test_extract_metadata_basic() {
        let extractor = PdfExtractor::new();
        let metadata = extractor.extract_metadata_from_bytes(&[], 5).unwrap();

        assert_eq!(metadata.page_count, 5);
        assert!(metadata.title.is_none());
        assert!(metadata.author.is_none());
        assert!(metadata.creation_date.is_none());
    }

    #[test]
    fn test_extract_metadata_zero_pages() {
        let extractor = PdfExtractor::new();
        let metadata = extractor.extract_metadata_from_bytes(&[], 0).unwrap();

        assert_eq!(metadata.page_count, 0);
    }

    #[test]
    fn test_extract_metadata_large_document() {
        let extractor = PdfExtractor::new();
        let metadata = extractor.extract_metadata_from_bytes(&[], 1000).unwrap();

        assert_eq!(metadata.page_count, 1000);
    }
}
