//! Vision Port Trait
//!
//! Defines the port interface for vision-capable LLM providers.
//! This trait abstracts vision analysis operations, allowing different
//! providers (OpenAI, Anthropic) to implement their specific vision APIs.

use async_trait::async_trait;
use paladin_core::platform::container::vision::{VisionContent, VisionError};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Result from vision analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisionResult {
    /// The analysis/description of the image(s)
    pub content: String,

    /// Model used for the analysis
    pub model: String,

    /// Token usage information
    pub token_usage: VisionTokenUsage,

    /// Additional metadata from the provider
    #[serde(default)]
    pub metadata: HashMap<String, String>,

    /// Timestamp of the response
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

/// Token usage for vision requests
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisionTokenUsage {
    /// Tokens used for the prompt (text + images)
    pub prompt_tokens: u32,

    /// Tokens used for the completion
    pub completion_tokens: u32,

    /// Total tokens used
    pub total_tokens: u32,
}

/// Vision port trait for multi-modal image analysis
///
/// # Choosing a vision surface
///
/// `VisionPort` is the **recommended entry point for application code**. It is reached via
/// `PaladinExecutionService::execute_with_vision`, which validates that vision is enabled on
/// the Paladin, resolves the provider's vision adapter, and calls
/// [`analyze_image`](VisionPort::analyze_image) on it.
///
/// The sibling surface, `VisionCapableLlm`, is the **adapter-author surface**: the trait an
/// adapter author implements when adding a vision-capable provider, reached via
/// `PaladinBuilder::enable_vision`. Application code should generally call `VisionPort` via
/// `execute_with_vision` instead of reaching for `VisionCapableLlm` directly.
///
/// Both traits ship deliberately, at different layers of the framework. Neither is legacy, and
/// no migration between them is planned or recommended for either audience. See the recorded
/// decision at `.planning/decisions/0011-vision-port-surfaces.md`.
#[async_trait]
pub trait VisionPort: Send + Sync {
    /// Analyze one or more images with a text prompt
    ///
    /// # Arguments
    ///
    /// * `prompt` - Text prompt/question about the image(s)
    /// * `images` - One or more images to analyze
    /// * `model` - Model to use for analysis (must be vision-capable)
    /// * `max_tokens` - Maximum tokens for the response
    ///
    /// # Returns
    ///
    /// * `Ok(VisionResult)` - Successful analysis with content and metadata
    /// * `Err(VisionError)` - Error during analysis
    ///
    /// # Errors
    ///
    /// - `VisionError::ModelNotSupported` - Model doesn't support vision
    /// - `VisionError::InvalidImage` - Image format/data is invalid
    /// - `VisionError::AuthenticationError` - Invalid API credentials
    /// - `VisionError::RateLimitExceeded` - Rate limit hit
    /// - `VisionError::ProviderError` - Provider-specific error
    /// - `VisionError::Timeout` - Request timed out
    async fn analyze_image(
        &self,
        prompt: &str,
        images: Vec<VisionContent>,
        model: &str,
        max_tokens: Option<u32>,
    ) -> Result<VisionResult, VisionError>;

    /// Check if a specific model supports vision
    ///
    /// # Arguments
    ///
    /// * `model` - The model name to check
    ///
    /// # Returns
    ///
    /// `true` if the model supports vision, `false` otherwise
    fn is_vision_model(&self, model: &str) -> bool;

    /// Get the provider name (e.g., "openai", "anthropic")
    fn provider_name(&self) -> &str;
}
