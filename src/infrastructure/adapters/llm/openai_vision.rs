//! OpenAI Vision Extension
//!
//! Extends OpenAI adapter with vision capabilities for multimodal requests.
//! Supports GPT-4o, GPT-4 Vision Preview, and GPT-4o-mini models.

use async_trait::async_trait;
use base64::{Engine as _, engine::general_purpose};
use chrono::Utc;
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use std::path::Path;
use tokio::fs;

use crate::application::ports::output::llm_port::{
    FinishReason, LlmError, LlmRequest, LlmResponse, TokenUsage,
};
use crate::application::ports::output::vision_llm_port::VisionCapableLlm;
use crate::config::application_settings::VisionConfig;
use crate::core::platform::container::vision::{
    ImageDetail, VisionContent, VisionError, VisionRequest,
};

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

/// OpenAI vision API request structure
#[derive(Debug, Serialize)]
struct OpenAIVisionApiRequest {
    model: String,
    messages: Vec<OpenAIVisionMessage>,
    max_tokens: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f32>,
}

/// OpenAI vision API response structure
#[derive(Debug, Deserialize)]
struct OpenAIVisionApiResponse {
    #[allow(dead_code)]
    id: String,
    model: String,
    choices: Vec<OpenAIVisionChoice>,
    usage: OpenAIVisionUsage,
}

#[derive(Debug, Deserialize)]
struct OpenAIVisionChoice {
    message: OpenAIVisionResponseMessage,
    finish_reason: Option<String>,
}

#[derive(Debug, Deserialize)]
struct OpenAIVisionResponseMessage {
    content: String,
}

#[derive(Debug, Deserialize)]
struct OpenAIVisionUsage {
    prompt_tokens: u32,
    completion_tokens: u32,
    total_tokens: u32,
}

/// OpenAI error response structure
#[derive(Debug, Deserialize)]
struct OpenAIErrorResponse {
    error: OpenAIErrorDetails,
}

#[derive(Debug, Deserialize)]
struct OpenAIErrorDetails {
    message: String,
    #[allow(dead_code)]
    #[serde(rename = "type")]
    error_type: Option<String>,
}

impl OpenAIAdapter {
    /// Check if a model supports vision
    pub fn is_vision_model(model: &str) -> bool {
        VISION_MODELS.contains(&model)
    }

    /// Calculate exponential backoff delay in milliseconds
    fn calculate_backoff_delay(
        retry_attempt: u32,
        initial_backoff_ms: u64,
        backoff_multiplier: f64,
    ) -> u64 {
        let delay = initial_backoff_ms as f64 * backoff_multiplier.powi(retry_attempt as i32);
        delay as u64
    }

    /// Check if an HTTP status code is transient and should be retried
    fn is_transient_error(status: StatusCode) -> bool {
        matches!(
            status,
            StatusCode::TOO_MANY_REQUESTS
                | StatusCode::INTERNAL_SERVER_ERROR
                | StatusCode::BAD_GATEWAY
                | StatusCode::SERVICE_UNAVAILABLE
                | StatusCode::GATEWAY_TIMEOUT
        )
    }

    /// Map HTTP status code to VisionError
    fn map_status_to_error(status: StatusCode, message: String) -> VisionError {
        match status {
            StatusCode::BAD_REQUEST => VisionError::InvalidImage(message),
            StatusCode::UNAUTHORIZED => VisionError::AuthenticationError(message),
            StatusCode::TOO_MANY_REQUESTS => VisionError::RateLimitExceeded(message),
            StatusCode::INTERNAL_SERVER_ERROR
            | StatusCode::BAD_GATEWAY
            | StatusCode::SERVICE_UNAVAILABLE
            | StatusCode::GATEWAY_TIMEOUT => VisionError::ProviderError(message),
            _ => VisionError::ProviderError(format!("HTTP {}: {}", status.as_u16(), message)),
        }
    }

    /// Execute vision API call with retry logic
    async fn execute_vision_request(
        &self,
        request_body: OpenAIVisionApiRequest,
        vision_config: &VisionConfig,
    ) -> Result<OpenAIVisionApiResponse, VisionError> {
        let max_retries = vision_config.retry.max_retries;
        let mut last_error: Option<VisionError> = None;

        for attempt in 0..=max_retries {
            // Build the request
            let response = self
                .client
                .post(format!("{}/chat/completions", self.config.base_url))
                .header("Authorization", format!("Bearer {}", self.config.api_key))
                .header("Content-Type", "application/json")
                .json(&request_body)
                .send()
                .await
                .map_err(|e| VisionError::NetworkError(format!("Request failed: {}", e)))?;

            let status = response.status();

            // Success
            if status.is_success() {
                let api_response: OpenAIVisionApiResponse = response.json().await.map_err(|e| {
                    VisionError::ProviderError(format!("Failed to parse response: {}", e))
                })?;
                return Ok(api_response);
            }

            // Parse error response
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Failed to read error response".to_string());

            let error_message = if let Ok(error_response) =
                serde_json::from_str::<OpenAIErrorResponse>(&error_text)
            {
                error_response.error.message
            } else {
                error_text
            };

            let error = Self::map_status_to_error(status, error_message);

            // Check if we should retry
            if attempt < max_retries && Self::is_transient_error(status) {
                last_error = Some(error);

                // Calculate backoff delay
                let delay_ms = Self::calculate_backoff_delay(
                    attempt,
                    vision_config.retry.initial_backoff_ms,
                    vision_config.retry.backoff_multiplier,
                );

                tokio::time::sleep(tokio::time::Duration::from_millis(delay_ms)).await;
                continue;
            }

            // Non-retryable error or last attempt
            return Err(error);
        }

        // Max retries exceeded
        Err(last_error.unwrap_or(VisionError::MaxRetriesExceeded(max_retries)))
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
        // Get vision configuration from settings
        let settings = crate::config::application_settings::Settings::new()
            .map_err(|e| LlmError::ProcessingError(format!("Failed to load settings: {}", e)))?;

        let vision_config = settings.get_vision_config();

        // Build vision messages
        let messages = self.build_vision_messages(&request, &vision).await?;

        // Build request body
        let request_body = OpenAIVisionApiRequest {
            model: request.model.clone(),
            messages,
            max_tokens: vision_config.openai.max_tokens,
            temperature: request
                .metadata
                .get("temperature")
                .and_then(|v| v.parse::<f32>().ok()),
        };

        // Execute API call with retry logic
        let api_response = self
            .execute_vision_request(request_body, &vision_config)
            .await
            .map_err(|e| match e {
                VisionError::InvalidImage(msg) => LlmError::InvalidPrompt(msg),
                VisionError::AuthenticationError(msg) => LlmError::AuthenticationError(msg),
                VisionError::RateLimitExceeded(msg) => {
                    // RateLimitExceeded in LlmError doesn't take a message
                    LlmError::ProcessingError(format!("Rate limit exceeded: {}", msg))
                }
                VisionError::NetworkError(msg) => LlmError::NetworkError(msg),
                VisionError::ProviderError(msg) | VisionError::UnsupportedProvider(msg) => {
                    LlmError::ProcessingError(msg)
                }
                VisionError::Timeout(seconds) => LlmError::Timeout(format!("{} seconds", seconds)),
                VisionError::MaxRetriesExceeded(attempts) => LlmError::ProcessingError(format!(
                    "Max retries exceeded: {} attempts",
                    attempts
                )),
                _ => LlmError::ProcessingError(format!("Vision error: {}", e)),
            })?;

        // Extract response content
        let content = api_response
            .choices
            .first()
            .map(|choice| choice.message.content.clone())
            .ok_or_else(|| LlmError::ProcessingError("No response content".to_string()))?;

        // Map finish reason
        let finish_reason = api_response
            .choices
            .first()
            .and_then(|choice| choice.finish_reason.as_ref())
            .map(|reason: &String| match reason.as_str() {
                "stop" => FinishReason::Stop,
                "length" => FinishReason::Length,
                "content_filter" => FinishReason::ContentFilter,
                _ => FinishReason::Error(reason.clone()),
            })
            .unwrap_or(FinishReason::Stop);

        // Build response
        Ok(LlmResponse {
            id: uuid::Uuid::new_v4(),
            request_id: request.id,
            model: api_response.model,
            content,
            finish_reason,
            usage: TokenUsage {
                prompt_tokens: api_response.usage.prompt_tokens,
                completion_tokens: api_response.usage.completion_tokens,
                total_tokens: api_response.usage.total_tokens,
            },
            created_at: Utc::now(),
            metadata: Default::default(),
            function_call: None,
        })
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

    #[test]
    fn test_calculate_backoff_delay() {
        // First retry: 1000 * 2^0 = 1000ms
        let delay = OpenAIAdapter::calculate_backoff_delay(0, 1000, 2.0);
        assert_eq!(delay, 1000);

        // Second retry: 1000 * 2^1 = 2000ms
        let delay = OpenAIAdapter::calculate_backoff_delay(1, 1000, 2.0);
        assert_eq!(delay, 2000);

        // Third retry: 1000 * 2^2 = 4000ms
        let delay = OpenAIAdapter::calculate_backoff_delay(2, 1000, 2.0);
        assert_eq!(delay, 4000);

        // With different multiplier: 500 * 1.5^1 = 750ms
        let delay = OpenAIAdapter::calculate_backoff_delay(1, 500, 1.5);
        assert_eq!(delay, 750);
    }

    #[test]
    fn test_is_transient_error() {
        use reqwest::StatusCode;

        // Transient errors
        assert!(OpenAIAdapter::is_transient_error(
            StatusCode::TOO_MANY_REQUESTS
        ));
        assert!(OpenAIAdapter::is_transient_error(
            StatusCode::INTERNAL_SERVER_ERROR
        ));
        assert!(OpenAIAdapter::is_transient_error(StatusCode::BAD_GATEWAY));
        assert!(OpenAIAdapter::is_transient_error(
            StatusCode::SERVICE_UNAVAILABLE
        ));
        assert!(OpenAIAdapter::is_transient_error(
            StatusCode::GATEWAY_TIMEOUT
        ));

        // Non-transient errors
        assert!(!OpenAIAdapter::is_transient_error(StatusCode::BAD_REQUEST));
        assert!(!OpenAIAdapter::is_transient_error(StatusCode::UNAUTHORIZED));
        assert!(!OpenAIAdapter::is_transient_error(StatusCode::FORBIDDEN));
        assert!(!OpenAIAdapter::is_transient_error(StatusCode::NOT_FOUND));
        assert!(!OpenAIAdapter::is_transient_error(StatusCode::OK));
    }

    #[test]
    fn test_map_status_to_error() {
        use reqwest::StatusCode;

        // Bad Request → InvalidImage
        let error = OpenAIAdapter::map_status_to_error(
            StatusCode::BAD_REQUEST,
            "Invalid image".to_string(),
        );
        assert!(matches!(error, VisionError::InvalidImage(_)));

        // Unauthorized → AuthenticationError
        let error = OpenAIAdapter::map_status_to_error(
            StatusCode::UNAUTHORIZED,
            "Invalid API key".to_string(),
        );
        assert!(matches!(error, VisionError::AuthenticationError(_)));

        // Too Many Requests → RateLimitExceeded
        let error = OpenAIAdapter::map_status_to_error(
            StatusCode::TOO_MANY_REQUESTS,
            "Rate limit".to_string(),
        );
        assert!(matches!(error, VisionError::RateLimitExceeded(_)));

        // Internal Server Error → ProviderError
        let error = OpenAIAdapter::map_status_to_error(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Server error".to_string(),
        );
        assert!(matches!(error, VisionError::ProviderError(_)));

        // Bad Gateway → ProviderError
        let error = OpenAIAdapter::map_status_to_error(
            StatusCode::BAD_GATEWAY,
            "Gateway error".to_string(),
        );
        assert!(matches!(error, VisionError::ProviderError(_)));

        // Service Unavailable → ProviderError
        let error = OpenAIAdapter::map_status_to_error(
            StatusCode::SERVICE_UNAVAILABLE,
            "Service down".to_string(),
        );
        assert!(matches!(error, VisionError::ProviderError(_)));
    }
}
