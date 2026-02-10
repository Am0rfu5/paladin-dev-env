//! Vision content types for multi-modal AI agent processing.
//!
//! This module provides data structures for handling image inputs in Paladin agents,
//! supporting multiple image formats and quality levels for vision-capable LLM providers.

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Quality/detail level for image processing.
///
/// Controls the level of detail the vision model should use when analyzing an image.
/// Different providers may interpret these levels differently.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum ImageDetail {
    /// Let the model automatically decide the appropriate detail level.
    #[default]
    Auto,
    /// Lower detail, faster processing, lower cost.
    Low,
    /// Higher detail, slower processing, higher cost but more accurate analysis.
    High,
}

/// Vision content variants for different image input methods.
///
/// Supports three ways to provide images to vision-capable models:
/// - URL reference to a publicly accessible image
/// - Base64-encoded image data
/// - Local file path to an image file
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum VisionContent {
    /// Reference to a publicly accessible image URL.
    ImageUrl {
        /// The URL of the image.
        url: String,
        /// Detail level for processing.
        #[serde(default)]
        detail: ImageDetail,
    },
    /// Base64-encoded image data.
    ImageBase64 {
        /// Base64-encoded image data.
        data: String,
        /// MIME type of the image (e.g., "image/png", "image/jpeg").
        media_type: String,
        /// Detail level for processing.
        #[serde(default)]
        detail: ImageDetail,
    },
    /// Path to a local image file.
    ImageFile {
        /// Path to the image file.
        path: PathBuf,
        /// Detail level for processing.
        #[serde(default)]
        detail: ImageDetail,
    },
}

impl VisionContent {
    /// Validates that the image format is supported.
    ///
    /// Supported formats: PNG, JPEG, GIF, WebP
    ///
    /// # Errors
    ///
    /// Returns `VisionError::UnsupportedFormat` if the format is not supported.
    pub fn validate_format(&self) -> Result<(), VisionError> {
        match self {
            VisionContent::ImageUrl { url, .. } => {
                let url_lower = url.to_lowercase();
                if url_lower.ends_with(".png")
                    || url_lower.ends_with(".jpg")
                    || url_lower.ends_with(".jpeg")
                    || url_lower.ends_with(".gif")
                    || url_lower.ends_with(".webp")
                {
                    Ok(())
                } else {
                    Err(VisionError::UnsupportedFormat(
                        "URL must end with .png, .jpg, .jpeg, .gif, or .webp".to_string(),
                    ))
                }
            }
            VisionContent::ImageBase64 { media_type, .. } => {
                if media_type == "image/png"
                    || media_type == "image/jpeg"
                    || media_type == "image/gif"
                    || media_type == "image/webp"
                {
                    Ok(())
                } else {
                    Err(VisionError::UnsupportedFormat(format!(
                        "Unsupported media type: {}. Supported: image/png, image/jpeg, image/gif, image/webp",
                        media_type
                    )))
                }
            }
            VisionContent::ImageFile { path, .. } => {
                if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                    let ext_lower = ext.to_lowercase();
                    if ext_lower == "png"
                        || ext_lower == "jpg"
                        || ext_lower == "jpeg"
                        || ext_lower == "gif"
                        || ext_lower == "webp"
                    {
                        Ok(())
                    } else {
                        Err(VisionError::UnsupportedFormat(format!(
                            "Unsupported file extension: {}. Supported: png, jpg, jpeg, gif, webp",
                            ext
                        )))
                    }
                } else {
                    Err(VisionError::UnsupportedFormat(
                        "File has no extension".to_string(),
                    ))
                }
            }
        }
    }
}

/// A vision request combining text and images.
///
/// This is the primary structure for multi-modal requests to vision-capable LLMs.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VisionRequest {
    /// The text prompt or question about the images.
    pub text: String,
    /// Vector of images to analyze.
    pub images: Vec<VisionContent>,
}

impl VisionRequest {
    /// Creates a new vision request with validation.
    ///
    /// # Errors
    ///
    /// Returns `VisionError` if any image format is invalid or if the request is empty.
    pub fn new(text: String, images: Vec<VisionContent>) -> Result<Self, VisionError> {
        if text.is_empty() {
            return Err(VisionError::InvalidRequest(
                "Text prompt cannot be empty".to_string(),
            ));
        }

        if images.is_empty() {
            return Err(VisionError::InvalidRequest(
                "At least one image is required".to_string(),
            ));
        }

        // Validate all image formats
        for image in &images {
            image.validate_format()?;
        }

        Ok(Self { text, images })
    }

    /// Validates the vision request.
    pub fn validate(&self) -> Result<(), VisionError> {
        if self.text.is_empty() {
            return Err(VisionError::InvalidRequest(
                "Text prompt cannot be empty".to_string(),
            ));
        }

        if self.images.is_empty() {
            return Err(VisionError::InvalidRequest(
                "At least one image is required".to_string(),
            ));
        }

        for image in &self.images {
            image.validate_format()?;
        }

        Ok(())
    }
}

/// Vision-specific errors.
#[derive(Debug, thiserror::Error)]
pub enum VisionError {
    /// Unsupported image format.
    #[error("Unsupported image format: {0}")]
    UnsupportedFormat(String),

    /// Image file is too large.
    #[error("Image file too large: {size} bytes (max: {max})")]
    FileTooLarge { size: usize, max: usize },

    /// Invalid image data.
    #[error("Invalid image data: {0}")]
    InvalidImage(String),

    /// Model does not support vision.
    #[error("Model does not support vision: {0}")]
    ModelNotSupported(String),

    /// Network error.
    #[error("Network error: {0}")]
    NetworkError(String),

    /// Encryption error.
    #[error("Encryption error: {0}")]
    EncryptionError(String),

    /// Invalid request.
    #[error("Invalid vision request: {0}")]
    InvalidRequest(String),

    /// Authentication error (401, invalid API key).
    #[error("Authentication error: {0}")]
    AuthenticationError(String),

    /// Rate limit exceeded (429, too many requests).
    #[error("Rate limit exceeded: {0}")]
    RateLimitExceeded(String),

    /// Provider-specific error (5xx server errors).
    #[error("Provider error: {0}")]
    ProviderError(String),

    /// Request timeout.
    #[error("Request timeout after {0} seconds")]
    Timeout(u64),

    /// Unsupported vision provider.
    #[error("Unsupported vision provider: {0}")]
    UnsupportedProvider(String),

    /// Maximum retry attempts exceeded.
    #[error("Maximum retry attempts exceeded: {0} attempts")]
    MaxRetriesExceeded(u32),

    /// IO error.
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_image_detail_enum() {
        // Test all variants
        let auto = ImageDetail::Auto;
        let low = ImageDetail::Low;
        let high = ImageDetail::High;

        assert_eq!(auto, ImageDetail::Auto);
        assert_eq!(low, ImageDetail::Low);
        assert_eq!(high, ImageDetail::High);

        // Test default
        assert_eq!(ImageDetail::default(), ImageDetail::Auto);

        // Test serialization/deserialization
        let json = serde_json::to_string(&auto).unwrap();
        assert_eq!(json, "\"auto\"");
        let deserialized: ImageDetail = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized, auto);
    }

    #[test]
    fn test_vision_content_validation() {
        // Valid PNG URL
        let valid_png = VisionContent::ImageUrl {
            url: "https://example.com/image.png".to_string(),
            detail: ImageDetail::Auto,
        };
        assert!(valid_png.validate_format().is_ok());

        // Valid JPEG URL
        let valid_jpg = VisionContent::ImageUrl {
            url: "https://example.com/photo.jpg".to_string(),
            detail: ImageDetail::Low,
        };
        assert!(valid_jpg.validate_format().is_ok());

        // Valid GIF URL
        let valid_gif = VisionContent::ImageUrl {
            url: "https://example.com/animation.gif".to_string(),
            detail: ImageDetail::High,
        };
        assert!(valid_gif.validate_format().is_ok());

        // Valid WebP URL
        let valid_webp = VisionContent::ImageUrl {
            url: "https://example.com/image.webp".to_string(),
            detail: ImageDetail::Auto,
        };
        assert!(valid_webp.validate_format().is_ok());

        // Invalid format URL
        let invalid_url = VisionContent::ImageUrl {
            url: "https://example.com/document.pdf".to_string(),
            detail: ImageDetail::Auto,
        };
        assert!(invalid_url.validate_format().is_err());
    }

    #[test]
    fn test_vision_content_base64_validation() {
        // Valid base64 PNG
        let valid_base64 = VisionContent::ImageBase64 {
            data: "iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mNk+M9QDwADhgGAWjR9awAAAABJRU5ErkJggg==".to_string(),
            media_type: "image/png".to_string(),
            detail: ImageDetail::Auto,
        };
        assert!(valid_base64.validate_format().is_ok());

        // Invalid media type
        let invalid_media = VisionContent::ImageBase64 {
            data: "base64data".to_string(),
            media_type: "image/bmp".to_string(),
            detail: ImageDetail::Auto,
        };
        assert!(invalid_media.validate_format().is_err());
    }

    #[test]
    fn test_vision_content_file_validation() {
        // Valid file paths
        let valid_png = VisionContent::ImageFile {
            path: PathBuf::from("/path/to/image.png"),
            detail: ImageDetail::Auto,
        };
        assert!(valid_png.validate_format().is_ok());

        let valid_jpg = VisionContent::ImageFile {
            path: PathBuf::from("/path/to/photo.jpeg"),
            detail: ImageDetail::Auto,
        };
        assert!(valid_jpg.validate_format().is_ok());

        // Invalid file extension
        let invalid_file = VisionContent::ImageFile {
            path: PathBuf::from("/path/to/document.txt"),
            detail: ImageDetail::Auto,
        };
        assert!(invalid_file.validate_format().is_err());

        // No extension
        let no_ext = VisionContent::ImageFile {
            path: PathBuf::from("/path/to/file"),
            detail: ImageDetail::Auto,
        };
        assert!(no_ext.validate_format().is_err());
    }

    #[test]
    fn test_vision_request_creation() {
        let images = vec![VisionContent::ImageUrl {
            url: "https://example.com/image.png".to_string(),
            detail: ImageDetail::Auto,
        }];

        let request = VisionRequest::new("Describe this image".to_string(), images);
        assert!(request.is_ok());

        let request = request.unwrap();
        assert_eq!(request.text, "Describe this image");
        assert_eq!(request.images.len(), 1);
    }

    #[test]
    fn test_vision_request_empty_text() {
        let images = vec![VisionContent::ImageUrl {
            url: "https://example.com/image.png".to_string(),
            detail: ImageDetail::Auto,
        }];

        let request = VisionRequest::new("".to_string(), images);
        assert!(request.is_err());
        match request {
            Err(VisionError::InvalidRequest(msg)) => {
                assert!(msg.contains("Text prompt cannot be empty"));
            }
            _ => panic!("Expected InvalidRequest error"),
        }
    }

    #[test]
    fn test_vision_request_no_images() {
        let request = VisionRequest::new("Describe this".to_string(), vec![]);
        assert!(request.is_err());
        match request {
            Err(VisionError::InvalidRequest(msg)) => {
                assert!(msg.contains("At least one image is required"));
            }
            _ => panic!("Expected InvalidRequest error"),
        }
    }

    #[test]
    fn test_vision_request_multiple_images() {
        let images = vec![
            VisionContent::ImageUrl {
                url: "https://example.com/image1.png".to_string(),
                detail: ImageDetail::Auto,
            },
            VisionContent::ImageUrl {
                url: "https://example.com/image2.jpg".to_string(),
                detail: ImageDetail::Low,
            },
            VisionContent::ImageFile {
                path: PathBuf::from("/local/image.gif"),
                detail: ImageDetail::High,
            },
        ];

        let request = VisionRequest::new("Compare these images".to_string(), images);
        assert!(request.is_ok());

        let request = request.unwrap();
        assert_eq!(request.images.len(), 3);
    }

    #[test]
    fn test_vision_request_invalid_format() {
        let images = vec![VisionContent::ImageUrl {
            url: "https://example.com/document.pdf".to_string(),
            detail: ImageDetail::Auto,
        }];

        let request = VisionRequest::new("Describe this".to_string(), images);
        assert!(request.is_err());
        match request {
            Err(VisionError::UnsupportedFormat(_)) => {
                // Expected
            }
            _ => panic!("Expected UnsupportedFormat error"),
        }
    }

    #[test]
    fn test_vision_error_variants() {
        // Test AuthenticationError
        let auth_err = VisionError::AuthenticationError("Invalid API key".to_string());
        assert!(auth_err.to_string().contains("Authentication error"));
        assert!(auth_err.to_string().contains("Invalid API key"));

        // Test RateLimitExceeded
        let rate_err = VisionError::RateLimitExceeded("Too many requests".to_string());
        assert!(rate_err.to_string().contains("Rate limit exceeded"));
        assert!(rate_err.to_string().contains("Too many requests"));

        // Test ProviderError
        let provider_err = VisionError::ProviderError("Internal server error".to_string());
        assert!(provider_err.to_string().contains("Provider error"));
        assert!(provider_err.to_string().contains("Internal server error"));

        // Test Timeout
        let timeout_err = VisionError::Timeout(30);
        assert!(timeout_err.to_string().contains("timeout"));
        assert!(timeout_err.to_string().contains("30"));

        // Test UnsupportedProvider
        let unsupported_err = VisionError::UnsupportedProvider("unknown-provider".to_string());
        assert!(
            unsupported_err
                .to_string()
                .contains("Unsupported vision provider")
        );
        assert!(unsupported_err.to_string().contains("unknown-provider"));

        // Test MaxRetriesExceeded
        let max_retries_err = VisionError::MaxRetriesExceeded(3);
        assert!(
            max_retries_err
                .to_string()
                .contains("Maximum retry attempts exceeded")
        );
        assert!(max_retries_err.to_string().contains("3"));
    }

    #[test]
    fn test_vision_error_existing_variants() {
        // Test InvalidImage
        let invalid_img = VisionError::InvalidImage("Corrupted data".to_string());
        assert!(invalid_img.to_string().contains("Invalid image data"));

        // Test UnsupportedFormat
        let unsupported_fmt = VisionError::UnsupportedFormat("BMP not supported".to_string());
        assert!(
            unsupported_fmt
                .to_string()
                .contains("Unsupported image format")
        );

        // Test NetworkError
        let network_err = VisionError::NetworkError("Connection failed".to_string());
        assert!(network_err.to_string().contains("Network error"));

        // Test FileTooLarge
        let large_file = VisionError::FileTooLarge {
            size: 10_000_000,
            max: 5_000_000,
        };
        assert!(large_file.to_string().contains("too large"));
        assert!(large_file.to_string().contains("10000000"));
        assert!(large_file.to_string().contains("5000000"));
    }
}
