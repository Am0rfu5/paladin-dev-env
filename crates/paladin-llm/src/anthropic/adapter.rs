//! Anthropic Claude LLM Adapter
//!
//! Provides integration with Anthropic's Claude API.
//! Supports standard completions, streaming, and all core LlmPort functionality.
//! Claude has unique requirements: system messages separate from messages array,
//! max_tokens required, and different message structure.

use async_trait::async_trait;
use chrono::Utc;
use futures::{Stream, StreamExt};
use reqwest::{
    Client,
    header::{CONTENT_TYPE, HeaderMap, HeaderValue},
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;
use std::time::Duration;
use uuid::Uuid;

use paladin_core::platform::container::prompt::PromptType;
use paladin_ports::output::llm_port::{
    FinishReason, LlmError, LlmPort, LlmRequest, LlmResponse, ProviderCapabilities,
    StreamingResponse, TokenUsage,
};

/// Configuration for Anthropic Claude LLM adapter.
#[derive(Debug, Clone)]
pub struct AnthropicConfig {
    /// API key for Anthropic authentication.
    pub api_key: String,
    /// Base URL for Anthropic API.
    pub base_url: String,
    /// Default model to use (e.g., claude-3-5-sonnet-20241022).
    pub model: String,
    /// Default max tokens for responses (required by Claude API).
    pub max_tokens: u32,
    /// Request timeout in seconds.
    pub timeout_seconds: u64,
}

impl AnthropicConfig {
    /// Load configuration from environment variables.
    ///
    /// # Environment Variables
    /// - `ANTHROPIC_API_KEY` (required): Anthropic API key
    /// - `ANTHROPIC_BASE_URL` (optional): API base URL
    /// - `ANTHROPIC_MODEL` (optional): Default model
    /// - `ANTHROPIC_MAX_TOKENS` (optional): Default max tokens
    /// - `ANTHROPIC_TIMEOUT_SECONDS` (optional): Request timeout
    ///
    /// # Errors
    /// Returns error if required environment variables are missing or invalid.
    pub fn from_env() -> Result<Self, String> {
        let api_key = env::var("ANTHROPIC_API_KEY")
            .map_err(|_| "ANTHROPIC_API_KEY environment variable not set")?;

        let base_url = env::var("ANTHROPIC_BASE_URL")
            .unwrap_or_else(|_| "https://api.anthropic.com/v1".to_string());

        let model = env::var("ANTHROPIC_MODEL")
            .unwrap_or_else(|_| "claude-3-5-sonnet-20241022".to_string());

        let max_tokens = env::var("ANTHROPIC_MAX_TOKENS")
            .unwrap_or_else(|_| "4096".to_string())
            .parse()
            .map_err(|_| "Invalid ANTHROPIC_MAX_TOKENS value")?;

        let timeout_seconds = env::var("ANTHROPIC_TIMEOUT_SECONDS")
            .unwrap_or_else(|_| "60".to_string())
            .parse()
            .map_err(|_| "Invalid ANTHROPIC_TIMEOUT_SECONDS value")?;

        let config = Self {
            api_key,
            base_url,
            model,
            max_tokens,
            timeout_seconds,
        };

        config.validate()?;
        Ok(config)
    }

    /// Create configuration with custom values.
    pub fn new(api_key: String, base_url: String, model: String, max_tokens: u32) -> Self {
        Self {
            api_key,
            base_url,
            model,
            max_tokens,
            timeout_seconds: 60,
        }
    }

    /// Validate configuration.
    pub fn validate(&self) -> Result<(), String> {
        if self.api_key.is_empty() {
            return Err("API key cannot be empty".to_string());
        }
        if self.base_url.is_empty() {
            return Err("Base URL cannot be empty".to_string());
        }
        if !self.base_url.starts_with("http://") && !self.base_url.starts_with("https://") {
            return Err("Base URL must start with http:// or https://".to_string());
        }
        if self.model.is_empty() {
            return Err("Model cannot be empty".to_string());
        }
        if self.max_tokens == 0 {
            return Err("Max tokens must be greater than 0".to_string());
        }
        Ok(())
    }
}

/// Anthropic Claude LLM Adapter implementing [`LlmPort`].
pub struct AnthropicAdapter {
    pub(crate) client: Client,
    pub(crate) config: AnthropicConfig,
}

impl AnthropicAdapter {
    /// Create a new Anthropic adapter with the given configuration.
    ///
    /// # Errors
    /// Returns error if configuration is invalid or HTTP client cannot be created.
    pub fn new(config: AnthropicConfig) -> Result<Self, LlmError> {
        config
            .validate()
            .map_err(|e| LlmError::AuthenticationError(format!("Invalid configuration: {}", e)))?;

        let client = Client::builder()
            .timeout(Duration::from_secs(config.timeout_seconds))
            .build()
            .map_err(|e| LlmError::NetworkError(format!("Failed to create HTTP client: {}", e)))?;

        Ok(Self { client, config })
    }

    /// Build HTTP headers for Anthropic API requests.
    fn build_headers(&self) -> Result<HeaderMap, LlmError> {
        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

        let api_key_header = HeaderValue::from_str(&self.config.api_key)
            .map_err(|e| LlmError::AuthenticationError(format!("Invalid API key format: {}", e)))?;
        headers.insert("x-api-key", api_key_header);
        headers.insert("anthropic-version", HeaderValue::from_static("2023-06-01"));

        Ok(headers)
    }

    /// Convert LlmRequest to Claude API format.
    fn build_request(&self, request: &LlmRequest) -> Result<ClaudeRequest, LlmError> {
        let mut system_message = None;
        let mut messages = Vec::new();

        match &request.prompt.node.node.prompt_type {
            PromptType::System(system_prompt) => {
                system_message = Some(system_prompt.instructions.clone());
            }
            PromptType::User(user_prompt) => {
                messages.push(ClaudeMessage {
                    role: "user".to_string(),
                    content: user_prompt.query.clone(),
                });
            }
            PromptType::Text(text_prompt) => {
                let role = match &text_prompt.role {
                    paladin_core::platform::container::prompt::PromptRole::System => {
                        system_message = Some(text_prompt.content.clone());
                        return Ok(ClaudeRequest {
                            model: request.model.clone(),
                            messages: vec![],
                            system: system_message,
                            max_tokens: request
                                .prompt
                                .node
                                .node
                                .parameters
                                .max_tokens
                                .unwrap_or(self.config.max_tokens),
                            // Phase 23.1: current Claude models (e.g. claude-opus-4-8)
                            // reject an explicit `temperature` ("deprecated for this
                            // model") — and restrict other sampling overrides — so omit
                            // both and let the API default apply. `skip_serializing_if`
                            // drops the `None`s from the request body entirely.
                            temperature: None,
                            top_p: None,
                            stream: false,
                        });
                    }
                    paladin_core::platform::container::prompt::PromptRole::User => "user",
                    paladin_core::platform::container::prompt::PromptRole::Assistant => "assistant",
                    paladin_core::platform::container::prompt::PromptRole::Function => "user",
                };
                messages.push(ClaudeMessage {
                    role: role.to_string(),
                    content: text_prompt.content.clone(),
                });
            }
            PromptType::Assistant(assistant_prompt) => {
                messages.push(ClaudeMessage {
                    role: "assistant".to_string(),
                    content: assistant_prompt.response.clone(),
                });
            }
            PromptType::Function(_) => {
                return Err(LlmError::InvalidPrompt(
                    "Function prompts not yet supported for Claude".to_string(),
                ));
            }
        }

        let max_tokens = request
            .prompt
            .node
            .node
            .parameters
            .max_tokens
            .unwrap_or(self.config.max_tokens);

        Ok(ClaudeRequest {
            model: request.model.clone(),
            messages,
            system: system_message,
            max_tokens,
            // Phase 23.1: current Claude models (e.g. claude-opus-4-8) reject an
            // explicit `temperature` ("deprecated for this model") and restrict other
            // sampling overrides — omit both and let the API default apply
            // (`skip_serializing_if` drops the `None`s from the request body).
            temperature: None,
            top_p: None,
            stream: false,
        })
    }

    /// Parse Claude API response into LlmResponse.
    ///
    /// Fails loud via [`detect_no_text_content`] instead of ever returning
    /// `Ok` with empty content — an empty-string success is indistinguishable
    /// from a valid empty answer to every downstream caller, and is exactly
    /// what produced a 0-byte `PoC.t.sol` before this fix. Recovers ALL
    /// text-bearing blocks via [`concat_text_blocks`] in array order, not
    /// just the first, so a text block that follows a `thinking` block is no
    /// longer silently dropped.
    fn parse_response(
        &self,
        request_id: Uuid,
        response: ClaudeResponse,
    ) -> Result<LlmResponse, LlmError> {
        if let Some(err) =
            detect_no_text_content(&response.content, response.stop_reason.as_deref())
        {
            return Err(err);
        }

        let content = concat_text_blocks(&response.content);

        let finish_reason = match response.stop_reason.as_deref() {
            Some("end_turn") => FinishReason::Stop,
            Some("max_tokens") => FinishReason::Length,
            Some("stop_sequence") => FinishReason::Stop,
            _ => FinishReason::Error("unknown".to_string()),
        };

        Ok(LlmResponse {
            id: Uuid::new_v4(),
            request_id,
            model: response.model,
            content,
            finish_reason,
            usage: TokenUsage {
                prompt_tokens: response.usage.input_tokens,
                completion_tokens: response.usage.output_tokens,
                total_tokens: response.usage.input_tokens + response.usage.output_tokens,
            },
            created_at: Utc::now(),
            metadata: HashMap::new(),
            function_call: None,
        })
    }

    /// Map Anthropic API errors to LlmError.
    fn map_error(&self, status: u16, body: &str) -> LlmError {
        match status {
            401 => LlmError::AuthenticationError(
                "Invalid API key. Check your ANTHROPIC_API_KEY environment variable.".to_string(),
            ),
            403 => LlmError::AuthenticationError(
                "API key does not have permission for this resource.".to_string(),
            ),
            429 => LlmError::RateLimitExceeded,
            400 => {
                if body.contains("max_tokens") {
                    LlmError::InvalidPrompt(
                        "Invalid max_tokens value. Claude requires max_tokens to be set."
                            .to_string(),
                    )
                } else {
                    LlmError::InvalidPrompt(format!("Bad request: {}", body))
                }
            }
            500..=599 => LlmError::ProcessingError(format!(
                "Anthropic server error ({}). Please retry.",
                status
            )),
            _ => LlmError::ProcessingError(format!(
                "Request failed with status {}: {}",
                status, body
            )),
        }
    }

    /// Execute request with retry logic.
    async fn execute_with_retry<F, Fut, T>(
        &self,
        operation: F,
        max_retries: u32,
    ) -> Result<T, LlmError>
    where
        F: Fn() -> Fut,
        Fut: std::future::Future<Output = Result<T, LlmError>>,
    {
        let mut attempt = 0;
        let mut delay_ms = 1000u64;

        loop {
            match operation().await {
                Ok(result) => return Ok(result),
                Err(e) => {
                    attempt += 1;

                    // `EmptyCompletion` is deliberately non-retryable (D-04): the
                    // retried request is byte-for-byte identical, so a
                    // no-text truncation reproduces deterministically, and
                    // each retry is a multi-minute frontier-model
                    // generation. Mirrors the do-not-retry rule already
                    // documented on `LlmError::EmptyCompletion` itself.
                    if matches!(
                        e,
                        LlmError::AuthenticationError(_)
                            | LlmError::InvalidPrompt(_)
                            | LlmError::EmptyCompletion(_)
                    ) {
                        return Err(e);
                    }

                    if attempt >= max_retries {
                        return Err(e);
                    }

                    let jitter = (rand::random::<f64>() * 200.0) as u64;
                    tokio::time::sleep(Duration::from_millis(delay_ms + jitter)).await;
                    delay_ms = (delay_ms * 2).min(10000);
                }
            }
        }
    }
}

#[async_trait]
impl LlmPort for AnthropicAdapter {
    async fn generate(&self, request: LlmRequest) -> Result<LlmResponse, LlmError> {
        let claude_request = self.build_request(&request)?;
        let headers = self.build_headers()?;

        let operation = || async {
            let response = self
                .client
                .post(format!("{}/messages", self.config.base_url))
                .headers(headers.clone())
                .json(&claude_request)
                .send()
                .await
                .map_err(|e| LlmError::ProcessingError(format!("Request failed: {}", e)))?;

            let status = response.status().as_u16();

            if !response.status().is_success() {
                let body = response
                    .text()
                    .await
                    .unwrap_or_else(|_| "Unknown error".to_string());
                return Err(self.map_error(status, &body));
            }

            let body = response.text().await.map_err(|e| {
                LlmError::NetworkError(format!("Failed to read Anthropic response body: {}", e))
            })?;

            let claude_response: ClaudeResponse = serde_json::from_str(&body).map_err(|e| {
                LlmError::ProcessingError(format!(
                    "Deserialization of Anthropic response body failed (likely schema drift \
                     — see the `thinking`-block precedent in this adapter's tests): {} — \
                     body excerpt: {}",
                    e,
                    bounded_excerpt(&body, RESPONSE_EXCERPT_CHAR_BUDGET)
                ))
            })?;

            self.parse_response(request.id, claude_response)
        };

        self.execute_with_retry(operation, 3).await
    }

    async fn generate_stream(
        &self,
        request: LlmRequest,
    ) -> Result<Box<dyn Stream<Item = Result<StreamingResponse, LlmError>> + Send>, LlmError> {
        let mut claude_request = self.build_request(&request)?;
        claude_request.stream = true;

        let headers = self.build_headers()?;

        let response = self
            .client
            .post(format!("{}/messages", self.config.base_url))
            .headers(headers)
            .json(&claude_request)
            .send()
            .await
            .map_err(|e| LlmError::ProcessingError(format!("Stream request failed: {}", e)))?;

        if !response.status().is_success() {
            let status = response.status().as_u16();
            let body = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(self.map_error(status, &body));
        }

        let stream = response
            .bytes_stream()
            .map(move |chunk_result| match chunk_result {
                Ok(bytes) => {
                    let text = String::from_utf8_lossy(&bytes);

                    for line in text.lines() {
                        if let Some(json_str) = line.strip_prefix("data: ") {
                            if json_str.trim() == "[DONE]" {
                                continue;
                            }

                            if let Ok(event) = serde_json::from_str::<ClaudeStreamEvent>(json_str) {
                                match event.event_type.as_str() {
                                    "content_block_delta" => {
                                        if let Some(delta) = event.delta
                                            && let Some(text) = delta.text
                                        {
                                            return Ok(StreamingResponse {
                                                id: Uuid::new_v4(),
                                                delta: text,
                                                finish_reason: None,
                                            });
                                        }
                                    }
                                    "message_stop" => {
                                        return Ok(StreamingResponse {
                                            id: Uuid::new_v4(),
                                            delta: String::new(),
                                            finish_reason: Some(FinishReason::Stop),
                                        });
                                    }
                                    _ => {}
                                }
                            }
                        }
                    }

                    Ok(StreamingResponse {
                        id: Uuid::new_v4(),
                        delta: String::new(),
                        finish_reason: None,
                    })
                }
                Err(e) => Err(LlmError::ProcessingError(format!("Stream error: {}", e))),
            });

        Ok(Box::new(stream))
    }

    async fn validate_model(&self, model: &str) -> Result<bool, LlmError> {
        let valid_models = [
            "claude-3-5-sonnet-20241022",
            "claude-3-5-sonnet-20240620",
            "claude-3-opus-20240229",
            "claude-3-sonnet-20240229",
            "claude-3-haiku-20240307",
            "claude-2.1",
            "claude-2.0",
            "claude-instant-1.2",
        ];
        Ok(valid_models.contains(&model))
    }

    async fn get_available_models(&self) -> Result<Vec<String>, LlmError> {
        Ok(vec![
            "claude-3-5-sonnet-20241022".to_string(),
            "claude-3-5-sonnet-20240620".to_string(),
            "claude-3-opus-20240229".to_string(),
            "claude-3-sonnet-20240229".to_string(),
            "claude-3-haiku-20240307".to_string(),
            "claude-2.1".to_string(),
            "claude-2.0".to_string(),
            "claude-instant-1.2".to_string(),
        ])
    }

    fn get_provider_name(&self) -> &'static str {
        "anthropic"
    }

    fn get_capabilities(&self) -> ProviderCapabilities {
        ProviderCapabilities {
            supports_streaming: true,
            supports_tool_calling: true,
            supports_function_calling: false,
            supports_vision: true,
            supports_embeddings: false,
            max_context_tokens: Some(200_000),
            supports_system_messages: true,
            temperature_range: None,
        }
    }
}

// ── Claude API request/response types ───────────────────────────────────────

#[derive(Debug, Serialize)]
struct ClaudeRequest {
    model: String,
    messages: Vec<ClaudeMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    system: Option<String>,
    max_tokens: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    top_p: Option<f32>,
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    stream: bool,
}

#[derive(Debug, Serialize, Deserialize)]
struct ClaudeMessage {
    role: String,
    content: String,
}

#[derive(Debug, Deserialize)]
struct ClaudeResponse {
    #[allow(dead_code)]
    id: String,
    model: String,
    content: Vec<ClaudeContent>,
    stop_reason: Option<String>,
    usage: ClaudeUsage,
}

/// A single content block in an Anthropic response.
///
/// Claude 5 models emit heterogeneous block types in the same `content`
/// array. A `thinking` block (extended/interleaved thinking) carries
/// `thinking` and `signature` fields and NO `text` key at all — verified
/// live against `claude-opus-5` and `claude-sonnet-5` on 2026-07-26 (see
/// this module's captured-fixture tests). `text` must therefore be
/// optional, not required: a required `String` here was the entire root
/// cause of this adapter's original thinking-block deserialization bug.
/// `content_type` is read by the block-type census in
/// [`detect_no_text_content`], so it is genuinely load-bearing, not dead
/// code kept only for documentation.
#[derive(Debug, Deserialize)]
struct ClaudeContent {
    #[serde(rename = "type")]
    content_type: String,
    #[serde(default)]
    text: Option<String>,
}

#[derive(Debug, Deserialize)]
struct ClaudeUsage {
    input_tokens: u32,
    output_tokens: u32,
}

/// Character budget for the diagnostic body excerpt shown in a
/// deserialization-failure message. Large enough to reach the offending
/// field in a typical response body; small enough not to dump a full
/// generation (a captured production `thinking` block alone ran past
/// 10,000 characters) into a single log line.
const RESPONSE_EXCERPT_CHAR_BUDGET: usize = 512;

/// Build a diagnostic excerpt of a response body, bounded by character
/// count rather than byte count.
///
/// Slicing a UTF-8 `&str` by byte offset panics when the offset lands
/// mid-character, and panics are forbidden in this library — a captured
/// production response body is full of multi-byte characters in its
/// markdown text. When `body` is longer than `budget` characters, an ASCII
/// elision marker is appended reporting the total byte length of the
/// untruncated body, so the reader knows how much was withheld.
fn bounded_excerpt(body: &str, budget: usize) -> String {
    if body.chars().count() <= budget {
        return body.to_string();
    }

    let truncated: String = body.chars().take(budget).collect();
    format!("{truncated}... [truncated, {} total bytes]", body.len())
}

/// Concatenate the text of every text-bearing content block, in array
/// order.
///
/// Any block without a `text` value — `thinking`, `tool_use`,
/// `redacted_thinking`, or any block type invented after this code was
/// written — is skipped by construction. This is deliberate: an unfamiliar
/// block type must degrade to being ignored, never to a hard failure. No
/// separator is inserted between blocks; the Anthropic API already splits
/// blocks at semantic boundaries rather than mid-word, and the streaming
/// path (see `generate_stream` below) likewise accumulates deltas without
/// inserting anything between them.
fn concat_text_blocks(content: &[ClaudeContent]) -> String {
    content
        .iter()
        .filter_map(|block| block.text.as_deref())
        .collect()
}

/// Detect a response with no recoverable text.
///
/// Fires when every content block is skipped by [`concat_text_blocks`] —
/// the concatenated text is empty or whitespace-only. This is the same
/// class of failure DeepSeek's `detect_empty_completion` guards against: a
/// reasoning model can spend its entire `max_tokens` budget on hidden
/// thinking and emit no visible answer at all (verified live against
/// `claude-sonnet-5`, see this module's captured-fixture tests). Returning
/// `Ok("")` here would let that truncation masquerade as a valid empty
/// completion — exactly what produced a 0-byte `PoC.t.sol` in production.
///
/// The message names the block-type census and the stop reason so the next
/// reader learns "the model spent its whole budget thinking" in one step
/// instead of three rounds of debugging. Remediation advice is deliberately
/// NOT repeated here — [`LlmError::EmptyCompletion`]'s own `Display` already
/// appends it, and duplicating it would read as noise.
fn detect_no_text_content(
    content: &[ClaudeContent],
    stop_reason: Option<&str>,
) -> Option<LlmError> {
    if !concat_text_blocks(content).trim().is_empty() {
        return None;
    }

    let block_types = content
        .iter()
        .map(|block| block.content_type.as_str())
        .collect::<Vec<_>>()
        .join(", ");

    Some(LlmError::EmptyCompletion(format!(
        "{} content block(s) with no text (types: [{}]), stop_reason={}",
        content.len(),
        block_types,
        stop_reason.unwrap_or("none")
    )))
}

#[derive(Debug, Deserialize)]
struct ClaudeStreamEvent {
    #[serde(rename = "type")]
    event_type: String,
    #[serde(default)]
    delta: Option<ClaudeDelta>,
}

#[derive(Debug, Deserialize)]
struct ClaudeDelta {
    #[serde(rename = "type")]
    #[allow(dead_code)]
    delta_type: Option<String>,
    text: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use std::sync::atomic::{AtomicU32, Ordering};

    #[test]
    fn test_anthropic_config_validation() {
        let config = AnthropicConfig::new(
            "sk-ant-test123".to_string(),
            "https://api.anthropic.com/v1".to_string(),
            "claude-3-5-sonnet-20241022".to_string(),
            4096,
        );
        assert!(config.validate().is_ok());

        let config_empty_key = AnthropicConfig::new(
            "".to_string(),
            "https://api.anthropic.com/v1".to_string(),
            "claude-3-5-sonnet-20241022".to_string(),
            4096,
        );
        assert!(config_empty_key.validate().is_err());

        let config_invalid_url = AnthropicConfig::new(
            "sk-ant-test123".to_string(),
            "invalid-url".to_string(),
            "claude-3-5-sonnet-20241022".to_string(),
            4096,
        );
        assert!(config_invalid_url.validate().is_err());

        let config_zero_tokens = AnthropicConfig::new(
            "sk-ant-test123".to_string(),
            "https://api.anthropic.com/v1".to_string(),
            "claude-3-5-sonnet-20241022".to_string(),
            0,
        );
        assert!(config_zero_tokens.validate().is_err());
    }

    #[tokio::test]
    async fn test_anthropic_adapter_creation() {
        let config = AnthropicConfig::new(
            "sk-ant-test123".to_string(),
            "https://api.anthropic.com/v1".to_string(),
            "claude-3-5-sonnet-20241022".to_string(),
            4096,
        );
        assert!(AnthropicAdapter::new(config).is_ok());
    }

    #[tokio::test]
    async fn test_anthropic_provider_capabilities() {
        let config = AnthropicConfig::new(
            "sk-ant-test123".to_string(),
            "https://api.anthropic.com/v1".to_string(),
            "claude-3-5-sonnet-20241022".to_string(),
            4096,
        );
        let adapter = AnthropicAdapter::new(config).unwrap();
        let capabilities = adapter.get_capabilities();

        assert!(capabilities.supports_streaming);
        assert!(capabilities.supports_tool_calling);
        assert!(capabilities.supports_vision);
        assert!(capabilities.supports_system_messages);
        assert_eq!(capabilities.max_context_tokens, Some(200_000));
        assert_eq!(adapter.get_provider_name(), "anthropic");
    }

    // ── Fixtures ─────────────────────────────────────────────────────────
    //
    // The two small bodies below are captured verbatim (byte-for-byte) from
    // real Anthropic API responses recorded against the operator's key on
    // 2026-07-26. See `.planning/quick/260726-hac-...-des/captured/` for
    // the source files. Per D-05, the third (opus-5 thinking+text) body
    // keeps every key and every block but abbreviates the 10,136-character
    // opaque `signature` value and shortens the long markdown `text` value
    // — abbreviating `signature` cannot reopen the fixture-vs-reality gap
    // this bug came from, because no declared field ever reads it.

    /// Captured verbatim: `claude-opus-4-8`, `content` = `[text]`,
    /// `stop_reason` = `end_turn`. Today's working path.
    const TEXT_ONLY_OPUS_4_8_JSON: &str = r#"{"model":"claude-opus-4-8","id":"msg_011CdQ3yemAxSE8oJ3n9XtDa","type":"message","role":"assistant","content":[{"type":"text","text":"OK"}],"stop_reason":"end_turn","stop_sequence":null,"stop_details":null,"usage":{"input_tokens":16,"cache_creation_input_tokens":0,"cache_read_input_tokens":0,"cache_creation":{"ephemeral_5m_input_tokens":0,"ephemeral_1h_input_tokens":0},"output_tokens":4,"output_tokens_details":{"thinking_tokens":0},"service_tier":"standard","inference_geo":"global"}}"#;

    /// Captured verbatim: `claude-sonnet-5`, `content` = `[thinking]` and
    /// nothing else, `stop_reason` = `max_tokens`. The model spent its
    /// entire budget thinking and emitted no answer.
    const THINKING_ONLY_SONNET_5_JSON: &str = r#"{"model":"claude-sonnet-5","id":"msg_011CdPt7Pro2EKHRPvJrdHka","type":"message","role":"assistant","content":[{"type":"thinking","thinking":"","signature":"EvQBCokBCBAYAipA072s7SF3R2MEF7V6RjWZjqAyf1pvJ0UhG0E/yloUS36ysANfS3KsrPpPUV9s/SKSsiscE4UVvGTSvThmP2tWJzIPY2xhdWRlLXNvbm5ldC01OABCCHRoaW5raW5nWiQzNGU2YjdkMC02OGQxLTRiNjktOWJkMS0zMzYyNzdiOTBmY2MSDH6s/Y59weopm1Hw8xoMegJjpp+/E13Deon8IjDQtR8fSoDm3N1Xiw0btRU5hjQ/qkcts/89FAImYi3/ECk9PFmvJk+8txOjPRIU1ZIqGMM90P6i63HaR5a+FeRF7JFRIdZw5ukoEBgB"}],"stop_reason":"max_tokens","stop_sequence":null,"stop_details":null,"usage":{"input_tokens":8,"cache_creation_input_tokens":0,"cache_read_input_tokens":0,"cache_creation":{"ephemeral_5m_input_tokens":0,"ephemeral_1h_input_tokens":0},"output_tokens":4,"output_tokens_details":{"thinking_tokens":3},"service_tier":"standard","inference_geo":"global"}}"#;

    /// The recovered text of [`THINKING_TEXT_OPUS_5_JSON`] below — kept as
    /// its own constant so the round-trip assertion reads as an exact
    /// equality against a named value rather than a second inline literal.
    const EXPECTED_OPUS_5_TEXT: &str = "Reasoning (abbreviated for fixture — see \
captured/anthropic-thinking-text-opus-5.json for the full 11873-byte body). Step 1: in \
steady state a donation is pro-rata neutral; every holder gains in proportion to their \
stake, so an attacker donating to himself alone is a strict loss.";

    /// Derived from the real `claude-opus-5` capture: every key and every
    /// block is verbatim, but the 10,136-character opaque `signature` value
    /// and the long markdown `text` value are abbreviated per D-05 (see the
    /// fixtures header comment above). `content` = `[thinking, text]`,
    /// `stop_reason` = `max_tokens` — this is the exact shape that broke
    /// deserialization before this fix.
    const THINKING_TEXT_OPUS_5_JSON: &str = r#"{"model":"claude-opus-5","id":"msg_011CdQ41GjfNCpc1qNdjCKbC","type":"message","role":"assistant","content":[{"type":"thinking","thinking":"","signature":"CAISqTsKhwEIEBgCKkABoVeK3KR0Vu9cHJkwnm+QmD0N7bLxg9vCCS10YT+h5M820zj1fxEx2WzWTayWtMdJqwnnOTY3eMdd36FcJwN0Mg1jbGF1ZGUtb3B1cy01OAFCCHRoaW5raW5n[ABBREVIATED-10136-chars-see-captured-fixture]EBgB"},{"type":"text","text":"Reasoning (abbreviated for fixture — see captured/anthropic-thinking-text-opus-5.json for the full 11873-byte body). Step 1: in steady state a donation is pro-rata neutral; every holder gains in proportion to their stake, so an attacker donating to himself alone is a strict loss."}],"stop_reason":"max_tokens","stop_sequence":null,"stop_details":null,"usage":{"input_tokens":85,"cache_creation_input_tokens":0,"cache_read_input_tokens":0,"cache_creation":{"ephemeral_5m_input_tokens":0,"ephemeral_1h_input_tokens":0},"output_tokens":3000,"output_tokens_details":{"thinking_tokens":2561},"service_tier":"standard","inference_geo":"global"}}"#;

    /// A shape-extension, NOT a capture — no live body of this exact shape
    /// was recorded. Built to prove the durable lesson of this bug: a
    /// `tool_use` block and a `redacted_thinking` block are both skipped,
    /// never fatal, and two text blocks separated by non-text blocks still
    /// concatenate in array order.
    const MIXED_BLOCK_SHAPE_EXTENSION_JSON: &str = r#"{"model":"claude-opus-5","id":"msg_mixed_shape_extension","type":"message","role":"assistant","content":[{"type":"thinking","thinking":"reasoning...","signature":"sig"},{"type":"text","text":"Part one. "},{"type":"tool_use","id":"toolu_01","name":"lookup","input":{"query":"foo"}},{"type":"redacted_thinking","data":"opaque"},{"type":"text","text":"Part two."}],"stop_reason":"end_turn","stop_sequence":null,"usage":{"input_tokens":10,"output_tokens":20}}"#;

    fn test_adapter() -> AnthropicAdapter {
        let config = AnthropicConfig::new(
            "sk-ant-test123".to_string(),
            "https://api.anthropic.com/v1".to_string(),
            "claude-3-5-sonnet-20241022".to_string(),
            4096,
        );
        AnthropicAdapter::new(config).expect("test config must build a valid adapter")
    }

    // ── Task 1: block-type-tolerant deserialization + ordered recovery ────

    #[test]
    fn test_thinking_plus_text_response_deserializes_and_recovers_text_verbatim() {
        let response: ClaudeResponse = serde_json::from_str(THINKING_TEXT_OPUS_5_JSON)
            .expect("captured opus-5 thinking+text body must deserialize");

        let adapter = test_adapter();
        let llm_response = adapter
            .parse_response(Uuid::new_v4(), response)
            .expect("thinking+text response must recover its text, not error");

        assert_eq!(llm_response.content, EXPECTED_OPUS_5_TEXT);
    }

    #[test]
    fn test_thinking_plus_text_usage_tolerates_undeclared_keys() {
        // Verifies the tolerates-unknown-fields assumption against the real
        // body, rather than merely assuming serde's documented default
        // behaviour: six undeclared `usage` keys (cache_creation_input_tokens,
        // cache_read_input_tokens, nested cache_creation, nested
        // output_tokens_details, service_tier, inference_geo) plus four
        // undeclared top-level keys (type, role, stop_sequence, stop_details)
        // surround the declared fields in this fixture.
        let response: ClaudeResponse = serde_json::from_str(THINKING_TEXT_OPUS_5_JSON)
            .expect("captured opus-5 thinking+text body must deserialize");

        assert_eq!(response.usage.input_tokens, 85);
        assert_eq!(response.usage.output_tokens, 3000);
    }

    #[test]
    fn test_text_only_response_is_unchanged_from_todays_behaviour() {
        let response: ClaudeResponse = serde_json::from_str(TEXT_ONLY_OPUS_4_8_JSON)
            .expect("captured opus-4-8 text-only body must deserialize");

        let adapter = test_adapter();
        let llm_response = adapter
            .parse_response(Uuid::new_v4(), response)
            .expect("text-only response must succeed exactly as it does today");

        assert_eq!(llm_response.content, "OK");
        assert!(matches!(llm_response.finish_reason, FinishReason::Stop));
    }

    #[test]
    fn test_thinking_only_response_yields_empty_completion_not_empty_success() {
        let response: ClaudeResponse = serde_json::from_str(THINKING_ONLY_SONNET_5_JSON)
            .expect("captured sonnet-5 thinking-only body must deserialize");

        let adapter = test_adapter();
        let result = adapter.parse_response(Uuid::new_v4(), response);

        match result {
            Err(LlmError::EmptyCompletion(msg)) => {
                assert!(
                    msg.contains("thinking"),
                    "message must name the block type it saw: {msg}"
                );
                assert!(
                    msg.contains("max_tokens"),
                    "message must name the stop reason it arrived with: {msg}"
                );
            }
            other => panic!("expected Err(LlmError::EmptyCompletion(_)), got {other:?}"),
        }
    }

    #[test]
    fn test_mixed_block_shape_recovers_both_text_blocks_in_order_never_fatal() {
        let response: ClaudeResponse = serde_json::from_str(MIXED_BLOCK_SHAPE_EXTENSION_JSON)
            .expect("mixed-block shape-extension body must deserialize without error");

        let recovered = concat_text_blocks(&response.content);
        assert_eq!(recovered, "Part one. Part two.");
    }

    // ── Task 2: diagnosable deserialization failures + bounded excerpts ───

    #[test]
    fn test_bounded_excerpt_returns_input_unchanged_when_shorter_than_budget() {
        let body = "a short body well under the budget";
        assert_eq!(bounded_excerpt(body, RESPONSE_EXCERPT_CHAR_BUDGET), body);
    }

    #[test]
    fn test_bounded_excerpt_is_char_boundary_safe_on_multibyte_input() {
        // Each 'あ' is 3 bytes in UTF-8; a naive byte-index slice at the
        // budget would land mid-character and panic. Build an input well
        // past the budget entirely out of multi-byte characters.
        let body: String = std::iter::repeat_n('あ', 600).collect();
        let budget = 100;

        let excerpt = bounded_excerpt(&body, budget);

        assert!(
            excerpt.contains("truncated"),
            "excerpt must carry an elision marker: {excerpt}"
        );
        let expected_prefix: String = body.chars().take(budget).collect();
        assert!(
            excerpt.starts_with(&expected_prefix),
            "excerpt must not split a character at the boundary"
        );
        assert_eq!(expected_prefix.chars().count(), budget);
    }

    #[tokio::test(start_paused = true)]
    async fn test_execute_with_retry_invokes_operation_exactly_once_on_empty_completion() {
        let adapter = test_adapter();
        let calls = Arc::new(AtomicU32::new(0));
        let calls_clone = Arc::clone(&calls);

        let result: Result<(), LlmError> = adapter
            .execute_with_retry(
                move || {
                    let calls = Arc::clone(&calls_clone);
                    async move {
                        calls.fetch_add(1, Ordering::SeqCst);
                        Err(LlmError::EmptyCompletion("no text".to_string()))
                    }
                },
                3,
            )
            .await;

        assert!(result.is_err());
        assert_eq!(
            calls.load(Ordering::SeqCst),
            1,
            "a no-text response must not be retried — the retried request is identical"
        );
    }

    #[tokio::test(start_paused = true)]
    async fn test_execute_with_retry_still_retries_a_retryable_processing_error() {
        let adapter = test_adapter();
        let calls = Arc::new(AtomicU32::new(0));
        let calls_clone = Arc::clone(&calls);

        let result: Result<(), LlmError> = adapter
            .execute_with_retry(
                move || {
                    let calls = Arc::clone(&calls_clone);
                    async move {
                        calls.fetch_add(1, Ordering::SeqCst);
                        Err(LlmError::ProcessingError("retry me".to_string()))
                    }
                },
                3,
            )
            .await;

        assert!(result.is_err());
        assert_eq!(
            calls.load(Ordering::SeqCst),
            3,
            "a genuinely retryable error must still be retried up to max_retries"
        );
    }
}
