/*

LLM Port

A port that defines how the application interacts with the LLM (Low Level Model).

This port is responsible for translating high-level application use cases into interactions
with the LLM. It provides an abstraction layer that allows the application to interact with
the LLM without being tightly coupled to its implementation details.

Typical implementations of this port would be for the adapter to translate the requirements
of the high-level use cases into calls to the LLM, and to translate the results of those calls
back into a format that the application can use.

An LLM Api usually requires a few standard fields to be present in the request and response
like the prompt, max_tokens, different weights for "temperature". The LLM Port handles
these fields and provide a clean interface for the application to interact with the LLM and
for the adapter to translate the application's requirements into calls to the LLM.

*/
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use thiserror::Error;
use uuid::Uuid;

use crate::core::platform::container::content::ContentItem;
use crate::core::platform::container::prompt::PromptItem;

#[derive(Debug, Clone, Error)]
pub enum LlmError {
    #[error("Network error: {0}")]
    NetworkError(String),
    #[error("Authentication failed: {0}")]
    AuthenticationError(String),
    #[error("Invalid prompt: {0}")]
    InvalidPrompt(String),
    #[error("Rate limit exceeded")]
    RateLimitExceeded,
    #[error("Model not available: {0}")]
    ModelNotAvailable(String),
    #[error("Token limit exceeded")]
    TokenLimitExceeded,
    #[error("Processing error: {0}")]
    ProcessingError(String),
    #[error("Timeout: {0}")]
    Timeout(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LlmRequest {
    pub id: Uuid,
    pub model: String,
    pub prompt: PromptItem,
    pub attachments: Vec<ContentItem>,
    pub stream: bool,
    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LlmResponse {
    pub id: Uuid,
    pub request_id: Uuid,
    pub model: String,
    pub content: String,
    pub finish_reason: FinishReason,
    pub usage: TokenUsage,
    pub created_at: DateTime<Utc>,
    pub metadata: HashMap<String, String>,
    /// Function call details if the model requested a function/tool call
    pub function_call: Option<FunctionCall>,
}

/// Function call request from the LLM
///
/// When the LLM wants to invoke a tool, it returns this structure
/// containing the function name and arguments as JSON.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionCall {
    /// Name of the function/tool to call
    pub name: String,
    /// Arguments as a JSON-formatted string
    pub arguments: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FinishReason {
    Stop,
    Length,
    ContentFilter,
    FunctionCall,
    Error(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenUsage {
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
    pub total_tokens: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamingResponse {
    pub id: Uuid,
    pub delta: String,
    pub finish_reason: Option<FinishReason>,
}

/// Provider capabilities for feature detection
///
/// This structure allows clients to query what features a specific
/// LLM provider supports, enabling graceful degradation when features
/// are not available.
///
/// # Example
///
/// ```rust
/// # use paladin::application::ports::output::llm_port::ProviderCapabilities;
/// let capabilities = ProviderCapabilities {
///     supports_streaming: true,
///     supports_tool_calling: true,
///     supports_function_calling: true,
///     supports_vision: false,
///     supports_embeddings: false,
///     max_context_tokens: Some(128000),
///     supports_system_messages: true,
/// };
///
/// if capabilities.supports_streaming {
///     println!("Provider supports streaming responses");
/// }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ProviderCapabilities {
    /// Whether the provider supports streaming responses
    pub supports_streaming: bool,
    /// Whether the provider supports tool calling (external function execution)
    pub supports_tool_calling: bool,
    /// Whether the provider supports function calling (structured output)
    pub supports_function_calling: bool,
    /// Whether the provider supports vision/image inputs
    pub supports_vision: bool,
    /// Whether the provider supports embeddings generation
    pub supports_embeddings: bool,
    /// Maximum context window size in tokens (None if unlimited or unknown)
    pub max_context_tokens: Option<u32>,
    /// Whether the provider supports system messages
    pub supports_system_messages: bool,
}

impl Default for ProviderCapabilities {
    fn default() -> Self {
        Self {
            supports_streaming: false,
            supports_tool_calling: false,
            supports_function_calling: false,
            supports_vision: false,
            supports_embeddings: false,
            max_context_tokens: None,
            supports_system_messages: true,
        }
    }
}

/// Main LLM Port trait
///
/// This trait defines the interface for interacting with Language Model providers.
/// All LLM adapters must implement this trait to be used within the Paladin framework.
///
/// # Provider Implementation
///
/// When implementing this trait for a new provider, ensure:
/// - Error messages are actionable and provider-specific
/// - Rate limiting is handled with exponential backoff
/// - Timeouts are configurable
/// - Streaming is implemented efficiently
#[async_trait]
pub trait LlmPort: Send + Sync {
    /// Generate a completion from the LLM
    ///
    /// # Arguments
    /// * `request` - The LLM request containing prompt, model, and parameters
    ///
    /// # Returns
    /// * `Ok(LlmResponse)` - The generated completion
    /// * `Err(LlmError)` - Error during generation
    async fn generate(&self, request: LlmRequest) -> Result<LlmResponse, LlmError>;

    /// Generate a streaming completion from the LLM
    ///
    /// # Arguments
    /// * `request` - The LLM request containing prompt, model, and parameters
    ///
    /// # Returns
    /// * `Ok(Stream)` - A stream of response chunks
    /// * `Err(LlmError)` - Error during generation
    async fn generate_stream(
        &self,
        request: LlmRequest,
    ) -> Result<Box<dyn futures::Stream<Item = Result<StreamingResponse, LlmError>> + Send>, LlmError>;

    /// Validate if a model is available from this provider
    ///
    /// # Arguments
    /// * `model` - The model identifier to validate
    ///
    /// # Returns
    /// * `Ok(true)` - Model is available
    /// * `Ok(false)` - Model is not available
    /// * `Err(LlmError)` - Error during validation
    async fn validate_model(&self, model: &str) -> Result<bool, LlmError>;

    /// Get a list of available models from this provider
    ///
    /// # Returns
    /// * `Ok(Vec<String>)` - List of available model identifiers
    /// * `Err(LlmError)` - Error retrieving models
    async fn get_available_models(&self) -> Result<Vec<String>, LlmError>;

    /// Get the provider name (e.g., "openai", "deepseek", "anthropic")
    ///
    /// # Returns
    /// A static string identifying the provider
    fn get_provider_name(&self) -> &'static str;

    /// Get the capabilities of this provider
    ///
    /// This allows clients to query what features the provider supports,
    /// enabling feature detection and graceful degradation.
    ///
    /// # Returns
    /// A `ProviderCapabilities` structure describing supported features
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use paladin::application::ports::output::llm_port::LlmPort;
    /// # async fn example(llm_port: &dyn LlmPort) {
    /// let capabilities = llm_port.get_capabilities();
    /// if capabilities.supports_streaming {
    ///     // Use streaming
    /// } else {
    ///     // Fall back to non-streaming
    /// }
    /// # }
    /// ```
    fn get_capabilities(&self) -> ProviderCapabilities;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_provider_capabilities_default() {
        let capabilities = ProviderCapabilities::default();
        assert!(!capabilities.supports_streaming);
        assert!(!capabilities.supports_tool_calling);
        assert!(!capabilities.supports_function_calling);
        assert!(!capabilities.supports_vision);
        assert!(!capabilities.supports_embeddings);
        assert_eq!(capabilities.max_context_tokens, None);
        assert!(capabilities.supports_system_messages);
    }

    #[test]
    fn test_provider_capabilities_creation() {
        let capabilities = ProviderCapabilities {
            supports_streaming: true,
            supports_tool_calling: true,
            supports_function_calling: true,
            supports_vision: false,
            supports_embeddings: false,
            max_context_tokens: Some(128000),
            supports_system_messages: true,
        };

        assert!(capabilities.supports_streaming);
        assert!(capabilities.supports_tool_calling);
        assert_eq!(capabilities.max_context_tokens, Some(128000));
    }

    #[test]
    fn test_provider_capabilities_serialization() {
        let capabilities = ProviderCapabilities {
            supports_streaming: true,
            supports_tool_calling: false,
            supports_function_calling: true,
            supports_vision: false,
            supports_embeddings: false,
            max_context_tokens: Some(64000),
            supports_system_messages: true,
        };

        // Test serialization
        let json = serde_json::to_string(&capabilities).unwrap();
        assert!(json.contains("supports_streaming"));
        assert!(json.contains("max_context_tokens"));

        // Test deserialization
        let deserialized: ProviderCapabilities = serde_json::from_str(&json).unwrap();
        assert_eq!(capabilities, deserialized);
    }

    #[test]
    fn test_provider_capabilities_equality() {
        let caps1 = ProviderCapabilities {
            supports_streaming: true,
            supports_tool_calling: true,
            supports_function_calling: true,
            supports_vision: false,
            supports_embeddings: false,
            max_context_tokens: Some(100000),
            supports_system_messages: true,
        };

        let caps2 = ProviderCapabilities {
            supports_streaming: true,
            supports_tool_calling: true,
            supports_function_calling: true,
            supports_vision: false,
            supports_embeddings: false,
            max_context_tokens: Some(100000),
            supports_system_messages: true,
        };

        assert_eq!(caps1, caps2);
    }
}
