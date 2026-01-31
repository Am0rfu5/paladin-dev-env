//! OpenAI Vision Extension
//!
//! Extends OpenAI adapter with vision capabilities for multimodal requests.
//! Supports GPT-4o, GPT-4 Vision Preview, and GPT-4o-mini models.

use async_trait::async_trait;
use base64::{Engine as _, engine::general_purpose};
use serde::{Deserialize, Serialize};
use std::path::Path;
use tokio::fs;

use crate::application::ports::output::llm_port::{LlmError, LlmRequest, LlmResponse};
use crate::application::ports::output::vision_llm_port::VisionCapableLlm;
use crate::core::platform::container::vision::{ImageDetail, VisionContent, VisionRequest};

use super::openai_adapter::OpenAIAdapter;

/// OpenAI vision message content types
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
enum OpenAIContentPart {
    Text { text: String },
    ImageUrl { image_url: OpenAIImageUrl },
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct OpenAIImageUrl {
    url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    detail: Option<String>,
}

/// OpenAI vision message with multimodal content
#[derive(Debug, Serialize, Deserialize, Clone)]
struct OpenAIVisionMessage {
    role: String,
    content: Vec<OpenAIContentPart>,
}

/// Vision-capable models supported by OpenAI
const VISION_MODELS: &[&str] = &[
    "gpt-4o",
    "gpt-4o-mini",
    "gpt-4-vision-preview",
    "gpt-4-turbo",
    "gpt-4-turbo-2024-04-09",
];

impl OpenAIAdapter {
    /// Check if a model supports vision
    pub fn is_vision_model(model: &str) -> bool {
        VISION_MODELS.contains(&model)
    }

    /// Convert VisionContent to OpenAI format
    async fn convert_vision_content(
        &self,
        content: &VisionContent,
    ) -> Result<OpenAIContentPart, LlmError> {
        match content {
            VisionContent::ImageUrl { url, detail } => Ok(OpenAIContentPart::ImageUrl {
                image_url: OpenAIImageUrl {
                    url: url.clone(),
                    detail: Self::convert_detail(*detail),
                },
            }),
            VisionContent::ImageBase64 {
                data,
                media_type,
                detail,
            } => {
                // Format: data:image/jpeg;base64,/9j/4AAQ...
                let data_url = format!("data:{};base64,{}", media_type, data);
                Ok(OpenAIContentPart::ImageUrl {
                    image_url: OpenAIImageUrl {
                        url: data_url,
                        detail: Self::convert_detail(*detail),
                    },
                })
            }
            VisionContent::ImageFile { path, detail } => {
                // Read file and convert to base64
                let image_data = fs::read(path).await.map_err(|e| {
                    LlmError::ProcessingError(format!("Failed to read image file: {}", e))
                })?;

                // Detect MIME type
                let mime_type = Self::detect_mime_type(path)?;

                // Encode to base64
                let base64_data = general_purpose::STANDARD.encode(&image_data);
                let data_url = format!("data:{};base64,{}", mime_type, base64_data);

                Ok(OpenAIContentPart::ImageUrl {
                    image_url: OpenAIImageUrl {
                        url: data_url,
                        detail: Self::convert_detail(*detail),
                    },
                })
            }
        }
    }

    /// Convert ImageDetail to OpenAI detail string
    fn convert_detail(detail: ImageDetail) -> Option<String> {
        match detail {
            ImageDetail::Auto => None, // Use default
            ImageDetail::Low => Some("low".to_string()),
            ImageDetail::High => Some("high".to_string()),
        }
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

    /// Build vision messages for OpenAI API
    async fn build_vision_messages(
        &self,
        request: &LlmRequest,
        vision: &VisionRequest,
    ) -> Result<Vec<OpenAIVisionMessage>, LlmError> {
        // Validate model supports vision
        if !Self::is_vision_model(&request.model) {
            return Err(LlmError::ModelNotAvailable(format!(
                "Model {} does not support vision. Supported models: {}",
                request.model,
                VISION_MODELS.join(", ")
            )));
        }

        // Build content parts: text + images
        let mut content_parts = vec![];

        // Add text prompt
        if !vision.text.is_empty() {
            content_parts.push(OpenAIContentPart::Text {
                text: vision.text.clone(),
            });
        }

        // Add images
        for image in &vision.images {
            let image_part = self.convert_vision_content(image).await?;
            content_parts.push(image_part);
        }

        // Create user message with multimodal content
        Ok(vec![OpenAIVisionMessage {
            role: "user".to_string(),
            content: content_parts,
        }])
    }

    /// Estimate tokens for an image based on detail level
    ///
    /// OpenAI pricing:
    /// - Low detail: 85 tokens
    /// - High detail: 85 + 170 * (num_tiles), where tiles depend on image size
    #[allow(dead_code)] // Used in future implementation
    fn estimate_image_tokens(detail: ImageDetail, width: Option<u32>, height: Option<u32>) -> u32 {
        match detail {
            ImageDetail::Low => 85,
            ImageDetail::Auto | ImageDetail::High => {
                // Estimate high detail tokens
                // OpenAI resizes to fit within 2048x2048, then creates 512x512 tiles
                if let (Some(w), Some(h)) = (width, height) {
                    let scale = (2048.0 / w.max(h) as f32).min(1.0);
                    let scaled_w = (w as f32 * scale) as u32;
                    let scaled_h = (h as f32 * scale) as u32;

                    let tiles_w = scaled_w.div_ceil(512);
                    let tiles_h = scaled_h.div_ceil(512);
                    let num_tiles = tiles_w * tiles_h;

                    85 + (170 * num_tiles)
                } else {
                    // Conservative estimate: assume 4 tiles
                    85 + (170 * 4)
                }
            }
        }
    }
}

#[async_trait]
impl VisionCapableLlm for OpenAIAdapter {
    async fn generate_with_vision(
        &self,
        request: LlmRequest,
        vision: VisionRequest,
    ) -> Result<LlmResponse, LlmError> {
        // Build vision messages
        let _messages = self.build_vision_messages(&request, &vision).await?;

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
    use crate::infrastructure::adapters::llm::openai_adapter::OpenAIConfig;
    use std::collections::HashMap;
    use uuid::Uuid;

    fn create_test_adapter() -> OpenAIAdapter {
        let config = OpenAIConfig::new("test-key".to_string());
        OpenAIAdapter::new(config).unwrap()
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
        assert!(OpenAIAdapter::is_vision_model("gpt-4o"));
        assert!(OpenAIAdapter::is_vision_model("gpt-4o-mini"));
        assert!(OpenAIAdapter::is_vision_model("gpt-4-vision-preview"));
        assert!(OpenAIAdapter::is_vision_model("gpt-4-turbo"));
        assert!(!OpenAIAdapter::is_vision_model("gpt-3.5-turbo"));
        assert!(!OpenAIAdapter::is_vision_model("gpt-4"));
    }

    #[test]
    fn test_convert_detail() {
        assert_eq!(OpenAIAdapter::convert_detail(ImageDetail::Auto), None);
        assert_eq!(
            OpenAIAdapter::convert_detail(ImageDetail::Low),
            Some("low".to_string())
        );
        assert_eq!(
            OpenAIAdapter::convert_detail(ImageDetail::High),
            Some("high".to_string())
        );
    }

    #[test]
    fn test_detect_mime_type() {
        let path_jpg = Path::new("test.jpg");
        assert_eq!(
            OpenAIAdapter::detect_mime_type(path_jpg).unwrap(),
            "image/jpeg"
        );

        let path_png = Path::new("test.png");
        assert_eq!(
            OpenAIAdapter::detect_mime_type(path_png).unwrap(),
            "image/png"
        );

        let path_gif = Path::new("test.gif");
        assert_eq!(
            OpenAIAdapter::detect_mime_type(path_gif).unwrap(),
            "image/gif"
        );

        let path_webp = Path::new("test.webp");
        assert_eq!(
            OpenAIAdapter::detect_mime_type(path_webp).unwrap(),
            "image/webp"
        );

        let path_invalid = Path::new("test.txt");
        assert!(OpenAIAdapter::detect_mime_type(path_invalid).is_err());
    }

    #[tokio::test]
    async fn test_convert_vision_content_image_url() {
        let adapter = create_test_adapter();
        let content = VisionContent::ImageUrl {
            url: "https://example.com/image.png".to_string(),
            detail: ImageDetail::High,
        };

        let result = adapter.convert_vision_content(&content).await.unwrap();

        match result {
            OpenAIContentPart::ImageUrl { image_url } => {
                assert_eq!(image_url.url, "https://example.com/image.png");
                assert_eq!(image_url.detail, Some("high".to_string()));
            }
            _ => panic!("Expected ImageUrl content part"),
        }
    }

    #[tokio::test]
    async fn test_convert_vision_content_base64() {
        let adapter = create_test_adapter();
        let content = VisionContent::ImageBase64 {
            data: "abc123".to_string(),
            media_type: "image/png".to_string(),
            detail: ImageDetail::Low,
        };

        let result = adapter.convert_vision_content(&content).await.unwrap();

        match result {
            OpenAIContentPart::ImageUrl { image_url } => {
                assert_eq!(image_url.url, "data:image/png;base64,abc123");
                assert_eq!(image_url.detail, Some("low".to_string()));
            }
            _ => panic!("Expected ImageUrl content part"),
        }
    }

    #[tokio::test]
    async fn test_build_vision_messages_non_vision_model() {
        let adapter = create_test_adapter();
        let request = create_test_request("gpt-3.5-turbo");
        let vision = VisionRequest::new(
            "Describe this image".to_string(),
            vec![VisionContent::ImageUrl {
                url: "https://example.com/image.png".to_string(),
                detail: ImageDetail::Auto,
            }],
        )
        .unwrap();

        let result = adapter.build_vision_messages(&request, &vision).await;
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            LlmError::ModelNotAvailable(_)
        ));
    }

    #[tokio::test]
    async fn test_build_vision_messages_single_image() {
        let adapter = create_test_adapter();
        let request = create_test_request("gpt-4o");
        let vision = VisionRequest::new(
            "What's in this image?".to_string(),
            vec![VisionContent::ImageUrl {
                url: "https://example.com/image.png".to_string(),
                detail: ImageDetail::High,
            }],
        )
        .unwrap();

        let result = adapter.build_vision_messages(&request, &vision).await;
        assert!(result.is_ok());

        let messages = result.unwrap();
        assert_eq!(messages.len(), 1);
        assert_eq!(messages[0].role, "user");
        assert_eq!(messages[0].content.len(), 2); // Text + image
    }

    #[tokio::test]
    async fn test_build_vision_messages_multiple_images() {
        let adapter = create_test_adapter();
        let request = create_test_request("gpt-4o");
        let vision = VisionRequest::new(
            "Compare these images".to_string(),
            vec![
                VisionContent::ImageUrl {
                    url: "https://example.com/image1.png".to_string(),
                    detail: ImageDetail::Auto,
                },
                VisionContent::ImageUrl {
                    url: "https://example.com/image2.png".to_string(),
                    detail: ImageDetail::High,
                },
            ],
        )
        .unwrap();

        let result = adapter.build_vision_messages(&request, &vision).await;
        assert!(result.is_ok());

        let messages = result.unwrap();
        assert_eq!(messages.len(), 1);
        assert_eq!(messages[0].content.len(), 3); // Text + 2 images
    }

    #[test]
    fn test_estimate_image_tokens_low_detail() {
        let tokens = OpenAIAdapter::estimate_image_tokens(ImageDetail::Low, None, None);
        assert_eq!(tokens, 85);
    }

    #[test]
    fn test_estimate_image_tokens_high_detail() {
        // 512x512 image = 1 tile
        let tokens = OpenAIAdapter::estimate_image_tokens(ImageDetail::High, Some(512), Some(512));
        assert_eq!(tokens, 85 + 170); // 255

        // 1024x1024 image = 4 tiles (2x2)
        let tokens =
            OpenAIAdapter::estimate_image_tokens(ImageDetail::High, Some(1024), Some(1024));
        assert_eq!(tokens, 85 + (170 * 4)); // 765
    }

    #[test]
    fn test_supports_vision() {
        let adapter = create_test_adapter();
        assert!(adapter.supports_vision());
    }
}
