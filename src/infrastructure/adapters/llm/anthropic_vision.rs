//! Anthropic Claude Vision Extension
//!
//! Extends Anthropic adapter with vision capabilities for Claude 3 models.
//! Supports Claude 3 Opus, Sonnet, and Haiku with multimodal content blocks.
//!
//! **Important**: Anthropic requires all images to be base64-encoded.
//! URLs are automatically downloaded and converted to base64.

use async_trait::async_trait;
use base64::{Engine as _, engine::general_purpose};
use serde::{Deserialize, Serialize};
use std::path::Path;
use tokio::fs;

use crate::application::ports::output::llm_port::{LlmError, LlmRequest, LlmResponse};
use crate::application::ports::output::vision_llm_port::VisionCapableLlm;
use crate::core::platform::container::vision::{VisionContent, VisionRequest};

use super::anthropic_adapter::AnthropicAdapter;

/// Anthropic content block types for multimodal messages
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
enum ClaudeContentBlock {
    Text { text: String },
    Image { source: ClaudeImageSource },
}

/// Anthropic image source (base64 only)
#[derive(Debug, Serialize, Deserialize, Clone)]
struct ClaudeImageSource {
    #[serde(rename = "type")]
    source_type: String, // Always "base64"
    media_type: String, // e.g., "image/jpeg", "image/png"
    data: String,       // Base64-encoded image data
}

/// Anthropic vision message with content blocks
#[derive(Debug, Serialize, Deserialize, Clone)]
#[allow(dead_code)] // Used in future implementation
struct ClaudeVisionMessage {
    role: String,
    content: Vec<ClaudeContentBlock>,
}

/// Vision-capable Claude 3 models
const VISION_MODELS: &[&str] = &[
    "claude-3-opus-20240229",
    "claude-3-sonnet-20240229",
    "claude-3-haiku-20240307",
    "claude-3-5-sonnet-20240620",
    "claude-3-5-sonnet-20241022",
    "claude-3-5-haiku-20241022",
];

impl AnthropicAdapter {
    /// Check if a model supports vision
    pub fn is_vision_model(model: &str) -> bool {
        VISION_MODELS.contains(&model)
    }

    /// Convert VisionContent to Anthropic base64 format
    ///
    /// Anthropic requires all images to be base64-encoded, so URLs and files
    /// are automatically converted.
    async fn convert_vision_content(
        &self,
        content: &VisionContent,
    ) -> Result<ClaudeContentBlock, LlmError> {
        match content {
            VisionContent::ImageUrl { url, .. } => {
                // Download image from URL and convert to base64
                let (data, media_type) = Self::download_and_encode_image(url).await?;
                Ok(ClaudeContentBlock::Image {
                    source: ClaudeImageSource {
                        source_type: "base64".to_string(),
                        media_type,
                        data,
                    },
                })
            }
            VisionContent::ImageBase64 {
                data, media_type, ..
            } => {
                // Already base64, just wrap in Claude format
                Ok(ClaudeContentBlock::Image {
                    source: ClaudeImageSource {
                        source_type: "base64".to_string(),
                        media_type: media_type.clone(),
                        data: data.clone(),
                    },
                })
            }
            VisionContent::ImageFile { path, .. } => {
                // Read file and convert to base64
                let image_data = fs::read(path).await.map_err(|e| {
                    LlmError::ProcessingError(format!("Failed to read image file: {}", e))
                })?;

                let media_type = Self::detect_mime_type(path)?;
                let base64_data = general_purpose::STANDARD.encode(&image_data);

                Ok(ClaudeContentBlock::Image {
                    source: ClaudeImageSource {
                        source_type: "base64".to_string(),
                        media_type,
                        data: base64_data,
                    },
                })
            }
        }
    }

    /// Download image from URL and convert to base64
    async fn download_and_encode_image(url: &str) -> Result<(String, String), LlmError> {
        let response = reqwest::get(url)
            .await
            .map_err(|e| LlmError::NetworkError(format!("Failed to download image: {}", e)))?;

        if !response.status().is_success() {
            return Err(LlmError::NetworkError(format!(
                "Image download failed with status: {}",
                response.status()
            )));
        }

        // Get content type from headers
        let media_type = response
            .headers()
            .get("content-type")
            .and_then(|v| v.to_str().ok())
            .unwrap_or("image/jpeg")
            .to_string();

        // Validate media type
        if !media_type.starts_with("image/") {
            return Err(LlmError::InvalidPrompt(format!(
                "URL does not point to an image. Content-Type: {}",
                media_type
            )));
        }

        let image_bytes = response
            .bytes()
            .await
            .map_err(|e| LlmError::NetworkError(format!("Failed to read image data: {}", e)))?;

        let base64_data = general_purpose::STANDARD.encode(&image_bytes);

        Ok((base64_data, media_type))
    }

    /// Detect MIME type from file extension
    fn detect_mime_type(path: &Path) -> Result<String, LlmError> {
        let extension = path
            .extension()
            .and_then(|ext| ext.to_str())
            .ok_or_else(|| LlmError::InvalidPrompt("Image file has no extension".to_string()))?
            .to_lowercase();

        match extension.as_str() {
            "jpg" | "jpeg" => Ok("image/jpeg".to_string()),
            "png" => Ok("image/png".to_string()),
            "gif" => Ok("image/gif".to_string()),
            "webp" => Ok("image/webp".to_string()),
            _ => Err(LlmError::InvalidPrompt(format!(
                "Unsupported image format: {}",
                extension
            ))),
        }
    }

    /// Build vision content blocks for Anthropic API
    ///
    /// Creates a message with text and image content blocks following
    /// Anthropic's multimodal message format.
    async fn build_vision_content_blocks(
        &self,
        request: &LlmRequest,
        vision: &VisionRequest,
    ) -> Result<Vec<ClaudeContentBlock>, LlmError> {
        // Validate model supports vision
        if !Self::is_vision_model(&request.model) {
            return Err(LlmError::ModelNotAvailable(format!(
                "Model {} does not support vision. Supported models: {}",
                request.model,
                VISION_MODELS.join(", ")
            )));
        }

        let mut content_blocks = vec![];

        // Add text content block
        if !vision.text.is_empty() {
            content_blocks.push(ClaudeContentBlock::Text {
                text: vision.text.clone(),
            });
        }

        // Add image content blocks
        for image in &vision.images {
            let image_block = self.convert_vision_content(image).await?;
            content_blocks.push(image_block);
        }

        Ok(content_blocks)
    }
}

#[async_trait]
impl VisionCapableLlm for AnthropicAdapter {
    async fn generate_with_vision(
        &self,
        request: LlmRequest,
        vision: VisionRequest,
    ) -> Result<LlmResponse, LlmError> {
        // Build vision content blocks
        let _content_blocks = self.build_vision_content_blocks(&request, &vision).await?;

        // TODO: Implement actual API call
        // For now, return a placeholder
        Err(LlmError::ProcessingError(
            "Vision API call not yet implemented".to_string(),
        ))
    }

    fn supports_vision(&self) -> bool {
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::platform::container::prompt::{
        PromptItem, PromptRole, PromptType, TextPrompt,
    };
    use crate::core::platform::container::vision::ImageDetail;
    use crate::infrastructure::adapters::llm::anthropic_adapter::AnthropicConfig;
    use std::collections::HashMap;
    use uuid::Uuid;

    fn create_test_adapter() -> AnthropicAdapter {
        let config = AnthropicConfig::new(
            "sk-ant-test-key".to_string(),
            "https://api.anthropic.com/v1".to_string(),
            "claude-3-opus-20240229".to_string(),
            4096,
        );
        AnthropicAdapter::new(config).unwrap()
    }

    fn create_test_request(model: &str) -> LlmRequest {
        let text_prompt = TextPrompt {
            content: "Test prompt".to_string(),
            role: PromptRole::User,
        };
        LlmRequest {
            id: Uuid::new_v4(),
            model: model.to_string(),
            prompt: PromptItem::new(PromptType::Text(text_prompt)).unwrap(),
            attachments: vec![],
            stream: false,
            metadata: HashMap::new(),
        }
    }

    #[test]
    fn test_is_vision_model() {
        assert!(AnthropicAdapter::is_vision_model("claude-3-opus-20240229"));
        assert!(AnthropicAdapter::is_vision_model(
            "claude-3-sonnet-20240229"
        ));
        assert!(AnthropicAdapter::is_vision_model("claude-3-haiku-20240307"));
        assert!(AnthropicAdapter::is_vision_model(
            "claude-3-5-sonnet-20240620"
        ));
        assert!(AnthropicAdapter::is_vision_model(
            "claude-3-5-sonnet-20241022"
        ));
        assert!(AnthropicAdapter::is_vision_model(
            "claude-3-5-haiku-20241022"
        ));
        assert!(!AnthropicAdapter::is_vision_model("claude-2.1"));
        assert!(!AnthropicAdapter::is_vision_model("claude-instant-1.2"));
    }

    #[test]
    fn test_detect_mime_type() {
        let path_jpg = Path::new("test.jpg");
        assert_eq!(
            AnthropicAdapter::detect_mime_type(path_jpg).unwrap(),
            "image/jpeg"
        );

        let path_png = Path::new("test.png");
        assert_eq!(
            AnthropicAdapter::detect_mime_type(path_png).unwrap(),
            "image/png"
        );

        let path_gif = Path::new("test.gif");
        assert_eq!(
            AnthropicAdapter::detect_mime_type(path_gif).unwrap(),
            "image/gif"
        );

        let path_webp = Path::new("test.webp");
        assert_eq!(
            AnthropicAdapter::detect_mime_type(path_webp).unwrap(),
            "image/webp"
        );

        let path_invalid = Path::new("test.txt");
        assert!(AnthropicAdapter::detect_mime_type(path_invalid).is_err());
    }

    #[tokio::test]
    async fn test_convert_vision_content_base64() {
        let adapter = create_test_adapter();
        let content = VisionContent::ImageBase64 {
            data: "iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mNk+M9QDwADhgGAWjR9awAAAABJRU5ErkJggg==".to_string(),
            media_type: "image/png".to_string(),
            detail: ImageDetail::High,
        };

        let result = adapter.convert_vision_content(&content).await.unwrap();

        match result {
            ClaudeContentBlock::Image { source } => {
                assert_eq!(source.source_type, "base64");
                assert_eq!(source.media_type, "image/png");
                assert_eq!(
                    source.data,
                    "iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mNk+M9QDwADhgGAWjR9awAAAABJRU5ErkJggg=="
                );
            }
            _ => panic!("Expected Image content block"),
        }
    }

    #[tokio::test]
    async fn test_build_vision_content_blocks_non_vision_model() {
        let adapter = create_test_adapter();
        let request = create_test_request("claude-2.1");
        let vision = VisionRequest::new(
            "Describe this image".to_string(),
            vec![VisionContent::ImageBase64 {
                data: "abc123".to_string(),
                media_type: "image/png".to_string(),
                detail: ImageDetail::Auto,
            }],
        )
        .unwrap();

        let result = adapter.build_vision_content_blocks(&request, &vision).await;
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            LlmError::ModelNotAvailable(_)
        ));
    }

    #[tokio::test]
    async fn test_build_vision_content_blocks_single_image() {
        let adapter = create_test_adapter();
        let request = create_test_request("claude-3-opus-20240229");
        let vision = VisionRequest::new(
            "What's in this image?".to_string(),
            vec![VisionContent::ImageBase64 {
                data: "test_data".to_string(),
                media_type: "image/jpeg".to_string(),
                detail: ImageDetail::High,
            }],
        )
        .unwrap();

        let result = adapter.build_vision_content_blocks(&request, &vision).await;
        assert!(result.is_ok());

        let blocks = result.unwrap();
        assert_eq!(blocks.len(), 2); // Text + image

        // First block should be text
        match &blocks[0] {
            ClaudeContentBlock::Text { text } => {
                assert_eq!(text, "What's in this image?");
            }
            _ => panic!("Expected text block first"),
        }

        // Second block should be image
        match &blocks[1] {
            ClaudeContentBlock::Image { source } => {
                assert_eq!(source.media_type, "image/jpeg");
                assert_eq!(source.data, "test_data");
            }
            _ => panic!("Expected image block second"),
        }
    }

    #[tokio::test]
    async fn test_build_vision_content_blocks_multiple_images() {
        let adapter = create_test_adapter();
        let request = create_test_request("claude-3-sonnet-20240229");
        let vision = VisionRequest::new(
            "Compare these images".to_string(),
            vec![
                VisionContent::ImageBase64 {
                    data: "image1_data".to_string(),
                    media_type: "image/png".to_string(),
                    detail: ImageDetail::Auto,
                },
                VisionContent::ImageBase64 {
                    data: "image2_data".to_string(),
                    media_type: "image/jpeg".to_string(),
                    detail: ImageDetail::High,
                },
            ],
        )
        .unwrap();

        let result = adapter.build_vision_content_blocks(&request, &vision).await;
        assert!(result.is_ok());

        let blocks = result.unwrap();
        assert_eq!(blocks.len(), 3); // Text + 2 images
    }

    #[test]
    fn test_supports_vision() {
        let adapter = create_test_adapter();
        assert!(adapter.supports_vision());
    }

    #[tokio::test]
    async fn test_convert_vision_content_handles_detail_ignored() {
        // Anthropic doesn't use detail parameter, but we shouldn't error
        let adapter = create_test_adapter();
        let content = VisionContent::ImageBase64 {
            data: "test".to_string(),
            media_type: "image/png".to_string(),
            detail: ImageDetail::Low, // Should be ignored but not cause error
        };

        let result = adapter.convert_vision_content(&content).await;
        assert!(result.is_ok());
    }
}
