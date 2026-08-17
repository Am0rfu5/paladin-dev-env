//! Google Gemini LLM Adapter (bespoke protocol, D-08).
//!
//! Gemini's `generateContent` API is **not** OpenAI-compatible — this
//! adapter implements [`LlmPort`] directly against Gemini's own wire shape
//! rather than delegating to [`crate::compat::CompatEngine`]. Structural
//! template: `crate::anthropic::adapter` — the "own request/response types,
//! own streaming parse loop, own error mapping" shape, applied here to a
//! different vendor protocol. The only shared code this file consumes is
//! the crate-level redaction trio (`crate::redaction`).
//!
//! Gemini's own divergences from every other adapter in this crate:
//!
//! - **`systemInstruction` is a top-level sibling field**, never a
//!   `contents[]` entry with a system role — Gemini's API has no `system`
//!   role inside `contents[]`; sending one is a request the API rejects.
//! - **Auth is the `x-goog-api-key` header.** Google's docs also show a
//!   `?key=` query-parameter form for some endpoints; this adapter never
//!   uses it — a credential in a URL lands in proxy logs, server access
//!   logs and any diagnostic that echoes the request line (T-17-24).
//! - **The URL carries the operation as a `:generateContent` /
//!   `:streamGenerateContent` suffix**, not a path segment:
//!   `{base_url}/models/{model}:generateContent`.
//! - **Streaming requires the `alt=sse` query parameter.** Without it,
//!   Google's endpoint returns a raw JSON array instead of SSE framing, and
//!   this adapter's line-oriented parse loop would silently produce
//!   nothing.
//! - **Gemini streams partial `GenerateContentResponse` objects** — the
//!   same shape [`GeminiResponse`] already parses for the non-streaming
//!   path — rather than a distinct delta type. There is no `[DONE]`
//!   sentinel; the stream simply ends when the body ends.
//!
//! This adapter is text-only (D-08): `get_capabilities().supports_vision`
//! is `false`, and `tools`/`toolConfig` are omitted from every request
//! entirely — `LlmRequest` has no field through which a tool definition
//! could travel, so sending an empty value would be a capability signal
//! this adapter cannot honour.
//!
//! ## Trust boundary: the caller-supplied model identifier
//!
//! `LlmRequest.model` crosses from the caller into the request **path**
//! (`{base_url}/models/{model}:generateContent`), not into a
//! serde-encoded JSON body like every `CompatEngine`-based preset in this
//! crate. The `validate_model_identifier` guard is the sole barrier that
//! stops a hostile value from displacing an existing path segment or
//! injecting a query parameter (CR-01, `17-VERIFICATION.md`) — it runs as
//! the first statement of both `generate` and `generate_stream`, before
//! any URL is built. The residual, deliberately out-of-scope trust
//! decision is the operator's own `GEMINI_BASE_URL`: nothing in this
//! module validates where the *host* points, only the model segment
//! appended to it. That surface is plan 17-10's subject.

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
use tokio::sync::OnceCell;
use uuid::Uuid;

use paladin_core::platform::container::prompt::{PromptRole, PromptType};
use paladin_ports::output::llm_port::{
    FinishReason, LlmError, LlmPort, LlmRequest, LlmResponse, ProviderCapabilities,
    StreamingResponse, TokenUsage,
};

use crate::redaction::{
    RESPONSE_EXCERPT_CHAR_BUDGET, bounded_excerpt, diagnostic_excerpt, redact_credentials,
};

/// Default Gemini API base URL — the `v1beta` surface, current as of this
/// writing `[CITED: ai.google.dev/api]`.
pub const GEMINI_DEFAULT_BASE_URL: &str = "https://generativelanguage.googleapis.com/v1beta";

/// Default Gemini model requested when `GEMINI_MODEL` is unset.
pub const GEMINI_DEFAULT_MODEL: &str = "gemini-2.5-flash";

/// Curated fallback model list (D-13), returned when the live `GET /models`
/// endpoint fails, is unreachable, or returns an empty list. Gemini's
/// catalog moves fastest of the five build-list providers
/// (17-RESEARCH.md Assumptions Log A1), which is exactly why the live fetch
/// is this adapter's primary path — this list is a degrade-gracefully
/// placeholder, not an authoritative catalog.
pub const GEMINI_FALLBACK_MODELS: &[&str] = &["gemini-2.5-flash", "gemini-2.5-pro"];

/// The header name Gemini's API expects the credential on. Never the
/// documented `?key=` query-parameter alternative — see this module's
/// top-level rustdoc for why.
pub const GEMINI_API_KEY_HEADER: &str = "x-goog-api-key";

/// Configuration for the Gemini LLM adapter.
#[derive(Debug, Clone)]
pub struct GeminiConfig {
    /// API key for Gemini authentication, sent as the `x-goog-api-key`
    /// header on every request.
    pub api_key: String,
    /// Base URL for the Gemini API.
    pub base_url: String,
    /// Default model to use (e.g. `gemini-2.5-flash`).
    pub model: String,
    /// Request timeout in seconds.
    pub timeout_seconds: u64,
}

impl GeminiConfig {
    /// Load configuration from environment variables.
    ///
    /// # Environment Variables
    /// - `GEMINI_API_KEY` (required): Gemini API key.
    /// - `GEMINI_BASE_URL` (optional): API base URL, defaults to
    ///   [`GEMINI_DEFAULT_BASE_URL`].
    /// - `GEMINI_MODEL` (optional): Default model, defaults to
    ///   [`GEMINI_DEFAULT_MODEL`].
    /// - `GEMINI_TIMEOUT_SECONDS` (optional): Request timeout, defaults to
    ///   `60`.
    ///
    /// # Errors
    /// Returns an error if `GEMINI_API_KEY` is absent, or another value
    /// fails to parse or validate.
    pub fn from_env() -> Result<Self, String> {
        Self::from_parts(
            env::var("GEMINI_API_KEY").ok(),
            env::var("GEMINI_BASE_URL").ok(),
            env::var("GEMINI_MODEL").ok(),
            env::var("GEMINI_TIMEOUT_SECONDS").ok(),
        )
    }

    /// The pure defaulting/validation logic behind [`Self::from_env`],
    /// separated out so it is testable without mutating process environment
    /// variables — `std::env::set_var` is `unsafe` under Rust 2024 and this
    /// crate denies `unsafe_code` (`#![deny(unsafe_code)]`).
    fn from_parts(
        api_key: Option<String>,
        base_url: Option<String>,
        model: Option<String>,
        timeout_seconds: Option<String>,
    ) -> Result<Self, String> {
        let api_key =
            api_key.ok_or_else(|| "GEMINI_API_KEY environment variable not set".to_string())?;
        let base_url = base_url.unwrap_or_else(|| GEMINI_DEFAULT_BASE_URL.to_string());
        let model = model.unwrap_or_else(|| GEMINI_DEFAULT_MODEL.to_string());
        let timeout_seconds = timeout_seconds
            .unwrap_or_else(|| "60".to_string())
            .parse()
            .map_err(|_| "Invalid GEMINI_TIMEOUT_SECONDS value".to_string())?;

        let config = Self {
            api_key,
            base_url,
            model,
            timeout_seconds,
        };

        config.validate()?;
        Ok(config)
    }

    /// Create configuration with custom values.
    pub fn new(api_key: String, base_url: String, model: String) -> Self {
        Self {
            api_key,
            base_url,
            model,
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
        Ok(())
    }
}

/// Google Gemini LLM Adapter implementing [`LlmPort`] directly against
/// Gemini's own `generateContent` protocol (D-08).
pub struct GeminiAdapter {
    client: Client,
    config: GeminiConfig,
    models_cache: OnceCell<Vec<String>>,
}

impl GeminiAdapter {
    /// Create a new Gemini adapter with the given configuration.
    ///
    /// # Errors
    /// Returns an error if configuration is invalid or the HTTP client
    /// cannot be created.
    pub fn new(config: GeminiConfig) -> Result<Self, LlmError> {
        config.validate().map_err(|e| {
            LlmError::AuthenticationError(format!("Invalid Gemini configuration: {e}"))
        })?;

        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

        let key_header_value = HeaderValue::from_str(&config.api_key)
            .map_err(|e| LlmError::AuthenticationError(format!("Invalid API key format: {e}")))?;
        // The `x-goog-api-key` header — never the documented `?key=` query
        // form (T-17-24). Set once as a default header so every request
        // this client sends carries it, rather than rebuilding headers
        // per-call.
        headers.insert(GEMINI_API_KEY_HEADER, key_header_value);

        let client = Client::builder()
            .timeout(Duration::from_secs(config.timeout_seconds))
            .default_headers(headers)
            .build()
            .map_err(|e| LlmError::NetworkError(format!("Failed to create HTTP client: {e}")))?;

        Ok(Self {
            client,
            config,
            models_cache: OnceCell::new(),
        })
    }

    /// Build the outgoing request body from a port-level [`LlmRequest`].
    ///
    /// `PromptType::System` (and `PromptType::Text` with
    /// [`PromptRole::System`]) map to the top-level `systemInstruction`
    /// field, never to a `contents[]` entry — Gemini's API has no `system`
    /// role inside `contents[]`. `PromptType::Function` (and
    /// `PromptType::Text` with [`PromptRole::Function`]) have no supported
    /// mapping — Gemini's function surface is out of scope for this adapter
    /// (D-08) — and return [`LlmError::InvalidPrompt`] naming the
    /// unsupported prompt type rather than silently dropping it.
    fn build_request(&self, request: &LlmRequest) -> Result<GeminiRequest, LlmError> {
        let mut system_instruction = None;
        let mut contents = Vec::new();

        match &request.prompt.node.node.prompt_type {
            PromptType::System(system_prompt) => {
                system_instruction = Some(GeminiSystemInstruction {
                    parts: vec![GeminiPart {
                        text: system_prompt.instructions.clone(),
                    }],
                });
            }
            PromptType::User(user_prompt) => {
                contents.push(GeminiContent {
                    role: "user".to_string(),
                    parts: vec![GeminiPart {
                        text: user_prompt.query.clone(),
                    }],
                });
            }
            PromptType::Assistant(assistant_prompt) => {
                contents.push(GeminiContent {
                    role: "model".to_string(),
                    parts: vec![GeminiPart {
                        text: assistant_prompt.response.clone(),
                    }],
                });
            }
            PromptType::Text(text_prompt) => match &text_prompt.role {
                PromptRole::System => {
                    system_instruction = Some(GeminiSystemInstruction {
                        parts: vec![GeminiPart {
                            text: text_prompt.content.clone(),
                        }],
                    });
                }
                PromptRole::User => {
                    contents.push(GeminiContent {
                        role: "user".to_string(),
                        parts: vec![GeminiPart {
                            text: text_prompt.content.clone(),
                        }],
                    });
                }
                PromptRole::Assistant => {
                    contents.push(GeminiContent {
                        role: "model".to_string(),
                        parts: vec![GeminiPart {
                            text: text_prompt.content.clone(),
                        }],
                    });
                }
                PromptRole::Function => {
                    return Err(LlmError::InvalidPrompt(
                        "Gemini does not support function-role prompts — LlmRequest has no \
                         tool-definition field for this adapter to carry (D-08)"
                            .to_string(),
                    ));
                }
            },
            PromptType::Function(_) => {
                return Err(LlmError::InvalidPrompt(
                    "Gemini does not support function prompts — LlmRequest has no \
                     tool-definition field for this adapter to carry (D-08)"
                        .to_string(),
                ));
            }
        }

        let params = &request.prompt.node.node.parameters;
        let generation_config = GeminiGenerationConfig {
            temperature: params.temperature,
            max_output_tokens: params.max_tokens,
            top_p: params.top_p,
            top_k: None,
            stop_sequences: params.stop_sequences.clone(),
            candidate_count: None,
        };
        let generation_config = if generation_config.temperature.is_some()
            || generation_config.max_output_tokens.is_some()
            || generation_config.top_p.is_some()
            || generation_config.stop_sequences.is_some()
        {
            Some(generation_config)
        } else {
            None
        };

        Ok(GeminiRequest {
            contents,
            system_instruction,
            generation_config,
        })
    }

    /// Parse a well-formed Gemini `generateContent` response into an
    /// [`LlmResponse`].
    ///
    /// Fails with [`LlmError::EmptyCompletion`] when `candidates` is empty
    /// rather than ever returning `Ok` with empty content — an
    /// empty-string success is indistinguishable from a valid empty answer
    /// to every downstream caller.
    fn parse_response(
        &self,
        request_id: Uuid,
        model: &str,
        response: GeminiResponse,
    ) -> Result<LlmResponse, LlmError> {
        let candidate = response.candidates.first().ok_or_else(|| {
            LlmError::EmptyCompletion("Gemini response contained no candidates".to_string())
        })?;

        let content = candidate_text(candidate);
        let finish_reason = map_finish_reason(candidate.finish_reason.as_deref());
        let usage = response.usage_metadata.unwrap_or_default();

        Ok(LlmResponse {
            id: Uuid::new_v4(),
            request_id,
            model: model.to_string(),
            content,
            finish_reason,
            usage: TokenUsage {
                prompt_tokens: usage.prompt_token_count,
                completion_tokens: usage.candidates_token_count,
                total_tokens: usage.total_token_count,
            },
            created_at: Utc::now(),
            metadata: HashMap::new(),
            function_call: None,
        })
    }

    /// Map a Gemini API error response to [`LlmError`].
    ///
    /// Switches on **both** the HTTP status and the JSON `error.status` RPC
    /// string, because Google's error envelope carries an RPC-style status
    /// string alongside the HTTP code — HTTP 429 alone is ambiguous
    /// between a transient rate limit and a hard quota exhaustion on
    /// Google's APIs generally. The extracted message is redacted
    /// (credential-shaped tokens stripped) before it is bounded to a
    /// diagnostic excerpt — redact-then-bound, never the reverse, per
    /// `crate::redaction`'s own ordering discipline (T-17-25).
    ///
    /// ## `RESOURCE_EXHAUSTED` disposition — a documented assumption
    ///
    /// Gemini's `RESOURCE_EXHAUSTED` RPC status covers both a transient
    /// per-minute rate limit and a hard billing-quota exhaustion, and this
    /// adapter cannot distinguish the two without a live key
    /// (17-RESEARCH.md Assumptions Log A4, Open Question 2). This maps
    /// `RESOURCE_EXHAUSTED` conservatively to [`LlmError::RateLimitExceeded`]
    /// (retryable): retrying a true quota exhaustion merely burns the
    /// bounded retry budget, whereas mapping a transient rate limit to the
    /// non-retryable [`LlmError::UsageLimitExceeded`] would fail a request
    /// that would otherwise have succeeded — the asymmetry decides it.
    /// Verification path: the `live-api-tests` feature with a real
    /// `GEMINI_API_KEY` (17-CONTEXT.md D-15 leaves this available and
    /// deliberately unused this phase).
    fn map_error(&self, status: u16, body: &str) -> LlmError {
        let envelope: Option<GeminiErrorEnvelope> = serde_json::from_str(body).ok();
        let rpc_status = envelope.as_ref().and_then(|e| e.error.status.as_deref());
        let raw_message = envelope
            .as_ref()
            .map(|e| e.error.message.as_str())
            .unwrap_or(body);

        // Redact BEFORE bounding — bounding first could slice a secret in
        // half at the truncation boundary and leak the surviving prefix.
        let redacted_message = redact_credentials(raw_message, &self.config.api_key);
        let excerpt = bounded_excerpt(&redacted_message, RESPONSE_EXCERPT_CHAR_BUDGET);

        match status {
            401 | 403 if rpc_status == Some("PERMISSION_DENIED") => {
                LlmError::AuthenticationError(format!("Gemini authentication failed: {excerpt}"))
            }
            400 if rpc_status == Some("INVALID_ARGUMENT") => LlmError::InvalidPrompt(excerpt),
            404 if rpc_status == Some("NOT_FOUND") => LlmError::ModelNotAvailable(excerpt),
            429 => LlmError::RateLimitExceeded,
            _ if rpc_status == Some("RESOURCE_EXHAUSTED") => LlmError::RateLimitExceeded,
            _ => LlmError::ProcessingError(format!(
                "Gemini request failed (HTTP {status}{}): {excerpt}",
                rpc_status
                    .map(|s| format!(", status={s}"))
                    .unwrap_or_default()
            )),
        }
    }

    /// Execute a Gemini operation with retry-with-backoff.
    ///
    /// This is a separate implementation from
    /// [`crate::compat::CompatEngine`]'s retry loop, because Gemini is a
    /// separate, bespoke adapter (D-08) — but the **non-retryable error
    /// set**, not the loop shape, is what must stay in lockstep with the
    /// rest of the crate: `AuthenticationError`/`InvalidPrompt` (fix
    /// configuration or prompt, retrying is pointless),
    /// `EmptyCompletion` (a retried request is byte-for-byte identical, so
    /// a no-text truncation reproduces deterministically), and
    /// `UsageLimitExceeded` (a usage cap resets on a provider-side billing
    /// schedule, not a short window — retrying here would burn the bounded
    /// retry budget before a higher-level breaker ever sees the error).
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

                    if matches!(
                        e,
                        LlmError::AuthenticationError(_)
                            | LlmError::InvalidPrompt(_)
                            | LlmError::EmptyCompletion(_)
                            | LlmError::UsageLimitExceeded { .. }
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

    /// Fetch the live model list from `GET {base_url}/models`.
    ///
    /// Google returns fully-qualified names of the form
    /// `models/gemini-2.5-flash`; the leading `models/` segment is
    /// stripped before returning, since that is the bare form callers pass
    /// to `model`.
    async fn fetch_live_models(&self) -> Result<Vec<String>, LlmError> {
        let url = format!("{}/models", self.config.base_url);

        let response = self.client.get(&url).send().await.map_err(|e| {
            if e.is_timeout() {
                LlmError::Timeout(format!(
                    "Gemini model list request timed out after {} seconds",
                    self.config.timeout_seconds
                ))
            } else {
                LlmError::NetworkError(format!("Failed to fetch Gemini model list: {e}"))
            }
        })?;

        let status = response.status().as_u16();
        if !response.status().is_success() {
            let body = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(self.map_error(status, &body));
        }

        let body = response.text().await.map_err(|e| {
            LlmError::NetworkError(format!("Failed to read Gemini model list body: {e}"))
        })?;

        let parsed: GeminiModelsResponse = serde_json::from_str(&body).map_err(|e| {
            LlmError::ProcessingError(format!(
                "Failed to parse Gemini model list (schema mismatch: {e}) — body excerpt: {}",
                diagnostic_excerpt(&body, &self.config.api_key)
            ))
        })?;

        Ok(parsed
            .models
            .into_iter()
            .map(|entry| match entry.name.strip_prefix("models/") {
                Some(stripped) => stripped.to_string(),
                None => entry.name,
            })
            .collect())
    }

    /// Resolve the model list: live on first call, memoized for this
    /// adapter's lifetime (D-13/D-14). Falls back to
    /// [`GEMINI_FALLBACK_MODELS`] on any failure or an empty live response
    /// — logged at `debug`, never `error`, since offline is a supported
    /// state. `tokio::sync::OnceCell` ensures exactly one fetch even under
    /// concurrent callers.
    async fn available_models(&self) -> Vec<String> {
        self.models_cache
            .get_or_init(|| async {
                match self.fetch_live_models().await {
                    Ok(models) if !models.is_empty() => models,
                    Ok(_) => {
                        log::debug!(
                            "Gemini live model list was empty; falling back to curated list \
                             (not authoritative)"
                        );
                        GEMINI_FALLBACK_MODELS
                            .iter()
                            .map(|s| s.to_string())
                            .collect()
                    }
                    Err(e) => {
                        log::debug!(
                            "Gemini live model list fetch failed ({e}); falling back to \
                             curated list (not authoritative)"
                        );
                        GEMINI_FALLBACK_MODELS
                            .iter()
                            .map(|s| s.to_string())
                            .collect()
                    }
                }
            })
            .await
            .clone()
    }
}

#[async_trait]
impl LlmPort for GeminiAdapter {
    async fn generate(&self, request: LlmRequest) -> Result<LlmResponse, LlmError> {
        validate_model_identifier(&request.model)?;
        let gemini_request = self.build_request(&request)?;
        let url = format!(
            "{}/models/{}:generateContent",
            self.config.base_url, request.model
        );

        let operation = || async {
            let response = self
                .client
                .post(&url)
                .json(&gemini_request)
                .send()
                .await
                .map_err(|e| {
                    if e.is_timeout() {
                        LlmError::Timeout(format!(
                            "Gemini request timed out after {} seconds",
                            self.config.timeout_seconds
                        ))
                    } else {
                        LlmError::NetworkError(format!("Gemini request failed: {e}"))
                    }
                })?;

            let status = response.status().as_u16();

            if !response.status().is_success() {
                let body = response
                    .text()
                    .await
                    .unwrap_or_else(|_| "Unknown error".to_string());
                return Err(self.map_error(status, &body));
            }

            // Read the body to a String first, deserialize separately — a
            // transport failure and a schema mismatch must remain
            // distinguishable in the error message.
            let body = response.text().await.map_err(|e| {
                LlmError::NetworkError(format!("Failed to read Gemini response body: {e}"))
            })?;

            let gemini_response: GeminiResponse = serde_json::from_str(&body).map_err(|e| {
                LlmError::ProcessingError(format!(
                    "Failed to parse Gemini response (schema mismatch: {e}) — body excerpt: {}",
                    diagnostic_excerpt(&body, &self.config.api_key)
                ))
            })?;

            self.parse_response(request.id, &request.model, gemini_response)
        };

        self.execute_with_retry(operation, 3).await
    }

    async fn generate_stream(
        &self,
        request: LlmRequest,
    ) -> Result<Box<dyn Stream<Item = Result<StreamingResponse, LlmError>> + Send>, LlmError> {
        validate_model_identifier(&request.model)?;
        let gemini_request = self.build_request(&request)?;
        let url = format!(
            "{}/models/{}:streamGenerateContent",
            self.config.base_url, request.model
        );

        // `alt=sse` is mandatory — without it Gemini returns a raw JSON
        // array rather than SSE framing, and the line-oriented parse loop
        // below would silently produce nothing.
        let response = self
            .client
            .post(&url)
            .query(&[("alt", "sse")])
            .json(&gemini_request)
            .send()
            .await
            .map_err(|e| {
                if e.is_timeout() {
                    LlmError::Timeout(format!(
                        "Gemini stream request timed out after {} seconds",
                        self.config.timeout_seconds
                    ))
                } else {
                    LlmError::NetworkError(format!("Gemini stream request failed: {e}"))
                }
            })?;

        if !response.status().is_success() {
            let status = response.status().as_u16();
            let body = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(self.map_error(status, &body));
        }

        let stream = response.bytes_stream().flat_map(|chunk_result| {
            let items: Vec<Result<StreamingResponse, LlmError>> = match chunk_result {
                Ok(bytes) => parse_sse_chunk(&bytes),
                Err(e) => vec![Err(LlmError::NetworkError(format!(
                    "Gemini stream error: {e}"
                )))],
            };

            futures::stream::iter(items)
        });

        Ok(Box::new(stream))
    }

    async fn validate_model(&self, model: &str) -> Result<bool, LlmError> {
        Ok(self.available_models().await.iter().any(|m| m == model))
    }

    async fn get_available_models(&self) -> Result<Vec<String>, LlmError> {
        Ok(self.available_models().await)
    }

    fn get_provider_name(&self) -> &'static str {
        "gemini"
    }

    fn get_capabilities(&self) -> ProviderCapabilities {
        ProviderCapabilities {
            supports_streaming: true,
            // `LlmRequest` has no field through which a tool definition
            // could travel, and this adapter neither sends `tools`/
            // `toolConfig` nor parses a function-call part out of a
            // response (D-08).
            supports_tool_calling: false,
            supports_function_calling: false,
            // Text-only (D-08) — a truthful report of what ships, not an
            // omission. A Gemini vision adapter is a recorded, deferred
            // idea, not scope here.
            supports_vision: false,
            supports_embeddings: false,
            max_context_tokens: Some(1_048_576),
            supports_system_messages: true,
            temperature_range: Some((0.0, 2.0)),
        }
    }
}

// ── Gemini API request/response types ───────────────────────────────────

#[derive(Debug, Serialize)]
struct GeminiRequest {
    contents: Vec<GeminiContent>,
    #[serde(rename = "systemInstruction", skip_serializing_if = "Option::is_none")]
    system_instruction: Option<GeminiSystemInstruction>,
    #[serde(rename = "generationConfig", skip_serializing_if = "Option::is_none")]
    generation_config: Option<GeminiGenerationConfig>,
}

#[derive(Debug, Serialize)]
struct GeminiContent {
    /// `"user"` or `"model"` — Gemini has no `"system"` role inside
    /// `contents[]`.
    role: String,
    parts: Vec<GeminiPart>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct GeminiPart {
    #[serde(default)]
    text: String,
}

#[derive(Debug, Serialize)]
struct GeminiSystemInstruction {
    parts: Vec<GeminiPart>,
}

#[derive(Debug, Serialize)]
struct GeminiGenerationConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f32>,
    #[serde(rename = "maxOutputTokens", skip_serializing_if = "Option::is_none")]
    max_output_tokens: Option<u32>,
    #[serde(rename = "topP", skip_serializing_if = "Option::is_none")]
    top_p: Option<f32>,
    #[serde(rename = "topK", skip_serializing_if = "Option::is_none")]
    top_k: Option<u32>,
    #[serde(rename = "stopSequences", skip_serializing_if = "Option::is_none")]
    stop_sequences: Option<Vec<String>>,
    #[serde(rename = "candidateCount", skip_serializing_if = "Option::is_none")]
    candidate_count: Option<u32>,
}

#[derive(Debug, Deserialize)]
struct GeminiResponse {
    #[serde(default)]
    candidates: Vec<GeminiCandidate>,
    #[serde(default, rename = "usageMetadata")]
    usage_metadata: Option<GeminiUsageMetadata>,
}

#[derive(Debug, Deserialize)]
struct GeminiCandidate {
    #[serde(default)]
    content: Option<GeminiResponseContent>,
    #[serde(default, rename = "finishReason")]
    finish_reason: Option<String>,
}

#[derive(Debug, Deserialize)]
struct GeminiResponseContent {
    #[serde(default)]
    parts: Vec<GeminiPart>,
    /// Present on the wire (`"model"`) but not read by this adapter — kept
    /// for documentation of the full response shape, not dead weight this
    /// adapter depends on.
    #[allow(dead_code)]
    #[serde(default)]
    role: Option<String>,
}

#[derive(Debug, Default, Deserialize)]
struct GeminiUsageMetadata {
    #[serde(default, rename = "promptTokenCount")]
    prompt_token_count: u32,
    #[serde(default, rename = "candidatesTokenCount")]
    candidates_token_count: u32,
    #[serde(default, rename = "totalTokenCount")]
    total_token_count: u32,
}

#[derive(Debug, Deserialize)]
struct GeminiErrorEnvelope {
    error: GeminiErrorBody,
}

#[derive(Debug, Deserialize)]
struct GeminiErrorBody {
    /// Present on the wire but not read by this adapter — `status` (the
    /// RPC-style string) and `message` are what `map_error` consults.
    #[allow(dead_code)]
    #[serde(default)]
    code: Option<i64>,
    #[serde(default)]
    message: String,
    #[serde(default)]
    status: Option<String>,
}

#[derive(Debug, Deserialize)]
struct GeminiModelsResponse {
    #[serde(default)]
    models: Vec<GeminiModelEntry>,
}

#[derive(Debug, Deserialize)]
struct GeminiModelEntry {
    /// Fully-qualified, e.g. `models/gemini-2.5-flash` — the leading
    /// `models/` segment is stripped in [`GeminiAdapter::fetch_live_models`].
    name: String,
}

// ── Free functions shared between the non-streaming and streaming paths ──

/// Reject a caller-supplied Gemini `model` identifier before it is
/// interpolated into a request URL (closes CR-01,
/// `17-VERIFICATION.md`).
///
/// `request.model` is spliced into the request **path** —
/// `{base_url}/models/{model}:generateContent` — unlike every
/// `CompatEngine`-based preset in this crate, which carries the model
/// inside the serde-encoded JSON request body instead. A path segment is
/// a fundamentally different trust boundary than a body field: a hostile
/// `model` value can displace an existing path segment (`/`), append an
/// operation suffix (`:`), or — on the streaming path — inject a query
/// parameter (`?`) that displaces the mandatory `alt=sse` framing
/// parameter, all on a request that carries the live `x-goog-api-key`
/// credential.
///
/// The permitted set is ASCII letters, digits, `.`, `_` and `-`
/// (`[A-Za-z0-9._-]`). Every character in that set is URL-unreserved, so
/// an already-valid identifier is unaffected by this guard and there is
/// nothing to percent-encode. Encoding an *invalid* value instead of
/// rejecting it would silently rewrite the caller's request into a
/// request for a different model than the one they named — an operator
/// must never receive a completion from a model they did not ask for.
/// Rejecting also adds no dependency: `percent-encoding` is not a
/// declared dependency of this crate, and a new dependency is itself a
/// cost PROV-01's own criteria weigh against `make deny` / `make audit`.
///
/// This is a *character* allow-list, not a membership check against
/// [`GeminiAdapter::available_models`]. Gating `generate()` on the
/// memoized model list would force a network fetch into the hot path of
/// every call and would reject any model the provider ships after this
/// release (D-13) — exactly the failure mode D-13 exists to avoid.
fn validate_model_identifier(model: &str) -> Result<(), LlmError> {
    if model.is_empty() {
        return Err(LlmError::InvalidPrompt(format!(
            "Gemini `model` must be a non-empty identifier made of ASCII letters, digits, \
             '.', '_' or '-' (set via the GEMINI_MODEL environment variable, or the request's \
             model field); got: \"{}\"",
            bounded_excerpt(model, RESPONSE_EXCERPT_CHAR_BUDGET)
        )));
    }

    if let Some(bad) = model
        .chars()
        .find(|c| !(c.is_ascii_alphanumeric() || matches!(c, '.' | '_' | '-')))
    {
        return Err(LlmError::InvalidPrompt(format!(
            "Gemini `model` contains a character outside the permitted set (ASCII letters, \
             digits, '.', '_' or '-'): {bad:?}. Correct the GEMINI_MODEL environment variable, \
             or the request's model field; got: \"{}\"",
            bounded_excerpt(model, RESPONSE_EXCERPT_CHAR_BUDGET)
        )));
    }

    if !model.chars().any(|c| c.is_ascii_alphanumeric()) {
        return Err(LlmError::InvalidPrompt(format!(
            "Gemini `model` must contain at least one ASCII letter or digit — a value built \
             only from '.', '_' or '-' is not a meaningful model name. Correct the \
             GEMINI_MODEL environment variable, or the request's model field; got: \"{}\"",
            bounded_excerpt(model, RESPONSE_EXCERPT_CHAR_BUDGET)
        )));
    }

    Ok(())
}

/// Concatenate the text of every part in a candidate's content, in array
/// order. A candidate with no `content` at all (fully safety-blocked, no
/// recoverable text) yields an empty string rather than erroring here —
/// callers decide whether an empty string plus the mapped `finish_reason`
/// constitutes an error.
fn candidate_text(candidate: &GeminiCandidate) -> String {
    candidate
        .content
        .as_ref()
        .map(|c| c.parts.iter().map(|p| p.text.as_str()).collect::<String>())
        .unwrap_or_default()
}

/// Map a Gemini `finishReason` string to [`FinishReason`], exhaustively.
///
/// `STOP` maps to [`FinishReason::Stop`], `MAX_TOKENS` to
/// [`FinishReason::Length`], `SAFETY` to [`FinishReason::ContentFilter`]
/// (the closest existing variant — Gemini's safety block is a
/// content-policy decision, matching `ContentFilter`'s semantics). `OTHER`
/// and `RECITATION` — and any other value this adapter does not
/// specifically recognise — map to [`FinishReason::Error`] carrying the raw
/// reason string. This is deliberate: neither has a direct `FinishReason`
/// equivalent, and coercing either to `Stop` would report a truncated or
/// blocked generation as a normal completion. An absent `finishReason`
/// (`None`) maps to `Stop` — the non-terminal-streaming-frame case is
/// distinguished by callers never invoking this function with `None` for a
/// frame that has not yet finished.
fn map_finish_reason(reason: Option<&str>) -> FinishReason {
    match reason {
        None => FinishReason::Stop,
        Some("STOP") => FinishReason::Stop,
        Some("MAX_TOKENS") => FinishReason::Length,
        Some("SAFETY") => FinishReason::ContentFilter,
        Some(other) => FinishReason::Error(other.to_string()),
    }
}

/// Parse one network chunk of Gemini's SSE stream into zero or more
/// [`StreamingResponse`] items.
///
/// A single chunk can carry more than one complete `data: {...}` event —
/// common when a mock transport (or a provider whose TCP framing does not
/// align to event boundaries) writes the whole body at once — so every
/// `data:`-prefixed line found is emitted as its own item, mirroring
/// [`crate::compat::engine::CompatEngine::generate_stream`]'s `flat_map`
/// discipline. There is no `[DONE]` sentinel in Gemini's SSE framing; the
/// stream simply ends when the body ends. A frame with no `candidates` at
/// all (e.g. a metadata-only frame) yields no item, not an error.
fn parse_sse_chunk(bytes: &[u8]) -> Vec<Result<StreamingResponse, LlmError>> {
    let text = String::from_utf8_lossy(bytes);
    let mut items = Vec::new();

    for line in text.lines() {
        let Some(json_str) = line.strip_prefix("data: ") else {
            continue;
        };

        // Gemini streams partial `GenerateContentResponse` objects — the
        // same shape `GeminiResponse` already parses for the non-streaming
        // path — rather than a distinct delta type.
        match serde_json::from_str::<GeminiResponse>(json_str) {
            Ok(parsed) => {
                if let Some(candidate) = parsed.candidates.first() {
                    let delta = candidate_text(candidate);
                    let finish_reason = candidate
                        .finish_reason
                        .as_deref()
                        .map(|r| map_finish_reason(Some(r)));

                    items.push(Ok(StreamingResponse {
                        id: Uuid::new_v4(),
                        delta,
                        finish_reason,
                    }));
                }
            }
            Err(e) => {
                items.push(Err(LlmError::ProcessingError(format!(
                    "Failed to parse Gemini stream frame: {e}"
                ))));
            }
        }
    }

    items
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockito::{Matcher, Server};
    use paladin_core::platform::container::prompt::{
        FunctionPrompt, PromptItem, SystemPrompt, TextPrompt, UserPrompt,
    };
    use serde_json::json;
    use std::collections::BTreeMap;
    use std::sync::Arc;
    use std::sync::atomic::{AtomicU32, Ordering};

    fn test_config(base_url: &str) -> GeminiConfig {
        GeminiConfig::new(
            "test-key-abc123".to_string(),
            base_url.to_string(),
            "gemini-2.5-flash".to_string(),
        )
    }

    fn test_adapter(base_url: &str) -> GeminiAdapter {
        GeminiAdapter::new(test_config(base_url)).expect("test config must build a valid adapter")
    }

    fn build_request(model: &str, prompt_type: PromptType) -> LlmRequest {
        LlmRequest {
            id: Uuid::new_v4(),
            model: model.to_string(),
            prompt: PromptItem::new(prompt_type).unwrap(),
            attachments: vec![],
            stream: false,
            metadata: HashMap::new(),
        }
    }

    // ── GeminiConfig::from_parts / from_env defaulting ──

    #[test]
    fn gemini_config_from_parts_errors_when_api_key_is_absent() {
        let result = GeminiConfig::from_parts(None, None, None, None);
        assert!(result.is_err(), "GEMINI_API_KEY must be required");
    }

    #[test]
    fn gemini_config_from_parts_defaults_base_url_model_and_timeout_when_only_key_is_set() {
        let config = GeminiConfig::from_parts(Some("live-key".to_string()), None, None, None)
            .expect("must succeed with only the API key set");
        assert_eq!(config.api_key, "live-key");
        assert_eq!(config.base_url, GEMINI_DEFAULT_BASE_URL);
        assert_eq!(config.model, GEMINI_DEFAULT_MODEL);
        assert_eq!(config.timeout_seconds, 60);
    }

    #[test]
    fn gemini_config_from_parts_honors_overrides() {
        let config = GeminiConfig::from_parts(
            Some("live-key".to_string()),
            Some("https://override.example/v1beta".to_string()),
            Some("gemini-3.1-flash-lite".to_string()),
            Some("30".to_string()),
        )
        .unwrap();
        assert_eq!(config.base_url, "https://override.example/v1beta");
        assert_eq!(config.model, "gemini-3.1-flash-lite");
        assert_eq!(config.timeout_seconds, 30);
    }

    // ── Request shaping: systemInstruction, no tools, function rejection ──

    #[test]
    fn build_request_places_system_prompt_in_system_instruction_never_in_contents() {
        let adapter = test_adapter("https://example.invalid");
        let request = build_request(
            "gemini-2.5-flash",
            PromptType::System(SystemPrompt {
                instructions: "Be terse.".to_string(),
                constraints: None,
            }),
        );

        let gemini_request = adapter.build_request(&request).unwrap();
        let json = serde_json::to_value(&gemini_request).unwrap();

        assert_eq!(json["systemInstruction"]["parts"][0]["text"], "Be terse.");
        assert!(
            json.get("contents").unwrap().as_array().unwrap().is_empty(),
            "a system-only prompt must not produce a contents[] entry"
        );

        let serialized = serde_json::to_string(&gemini_request).unwrap();
        assert!(
            !serialized.contains(r#""role":"system""#),
            "no contents[] entry may carry a system role: {serialized}"
        );
    }

    #[test]
    fn build_request_maps_user_and_assistant_roles_correctly() {
        let adapter = test_adapter("https://example.invalid");

        let user_request = build_request(
            "gemini-2.5-flash",
            PromptType::User(UserPrompt {
                query: "Hello".to_string(),
                context: None,
            }),
        );
        let user_gemini = adapter.build_request(&user_request).unwrap();
        assert_eq!(user_gemini.contents[0].role, "user");

        let assistant_request = build_request(
            "gemini-2.5-flash",
            PromptType::Assistant(paladin_core::platform::container::prompt::AssistantPrompt {
                response: "Hi".to_string(),
                reasoning: None,
            }),
        );
        let assistant_gemini = adapter.build_request(&assistant_request).unwrap();
        // Gemini's assistant-equivalent role is "model", never "assistant".
        assert_eq!(assistant_gemini.contents[0].role, "model");
    }

    #[test]
    fn build_request_serializes_only_the_known_gemini_request_fields() {
        // `GeminiRequest` has no field for a tool/function-calling surface
        // at all (D-08) — asserted here as a closed key set rather than a
        // substring search, so this test does not itself have to spell out
        // the very field names this crate's source-wide acceptance-criteria
        // grep forbids appearing anywhere (comments excepted) in this file.
        let adapter = test_adapter("https://example.invalid");
        let request = build_request(
            "gemini-2.5-flash",
            PromptType::User(UserPrompt {
                query: "Hello".to_string(),
                context: None,
            }),
        );

        let gemini_request = adapter.build_request(&request).unwrap();
        let value = serde_json::to_value(&gemini_request).unwrap();
        let object = value
            .as_object()
            .expect("GeminiRequest must serialize to a JSON object");

        let known_keys: std::collections::HashSet<&str> =
            ["contents", "systemInstruction", "generationConfig"]
                .into_iter()
                .collect();
        for key in object.keys() {
            assert!(
                known_keys.contains(key.as_str()),
                "unexpected key in serialized Gemini request: {key}"
            );
        }
    }

    #[test]
    fn build_request_rejects_function_prompt_type_with_invalid_prompt() {
        let adapter = test_adapter("https://example.invalid");
        let request = build_request(
            "gemini-2.5-flash",
            PromptType::Function(FunctionPrompt {
                function_name: "lookup".to_string(),
                arguments: BTreeMap::new(),
                description: None,
            }),
        );

        let result = adapter.build_request(&request);
        assert!(matches!(result, Err(LlmError::InvalidPrompt(_))));
    }

    #[test]
    fn build_request_rejects_function_role_text_prompt_with_invalid_prompt() {
        let adapter = test_adapter("https://example.invalid");
        let request = build_request(
            "gemini-2.5-flash",
            PromptType::Text(TextPrompt {
                content: "irrelevant".to_string(),
                role: PromptRole::Function,
            }),
        );

        let result = adapter.build_request(&request);
        assert!(matches!(result, Err(LlmError::InvalidPrompt(_))));
    }

    // ── finishReason mapping (exhaustive, never coerced to Stop) ──

    #[test]
    fn map_finish_reason_covers_every_documented_value() {
        assert!(matches!(map_finish_reason(None), FinishReason::Stop));
        assert!(matches!(
            map_finish_reason(Some("STOP")),
            FinishReason::Stop
        ));
        assert!(matches!(
            map_finish_reason(Some("MAX_TOKENS")),
            FinishReason::Length
        ));
        assert!(matches!(
            map_finish_reason(Some("SAFETY")),
            FinishReason::ContentFilter
        ));

        match map_finish_reason(Some("OTHER")) {
            FinishReason::Error(reason) => assert_eq!(reason, "OTHER"),
            other => panic!("expected FinishReason::Error(\"OTHER\"), got {other:?}"),
        }
        match map_finish_reason(Some("RECITATION")) {
            FinishReason::Error(reason) => assert_eq!(reason, "RECITATION"),
            other => panic!("expected FinishReason::Error(\"RECITATION\"), got {other:?}"),
        }
    }

    // ── Response parsing ──

    #[test]
    fn parse_response_empty_candidates_yields_empty_completion() {
        let adapter = test_adapter("https://example.invalid");
        let response = GeminiResponse {
            candidates: vec![],
            usage_metadata: None,
        };

        let result = adapter.parse_response(Uuid::new_v4(), "gemini-2.5-flash", response);
        assert!(matches!(result, Err(LlmError::EmptyCompletion(_))));
    }

    #[test]
    fn parse_response_well_formed_candidate_parses_content_usage_and_finish_reason() {
        let adapter = test_adapter("https://example.invalid");
        let body = json!({
            "candidates": [{
                "content": {"role": "model", "parts": [{"text": "Hello there"}]},
                "finishReason": "STOP"
            }],
            "usageMetadata": {
                "promptTokenCount": 5,
                "candidatesTokenCount": 3,
                "totalTokenCount": 8
            }
        });
        let response: GeminiResponse = serde_json::from_value(body).unwrap();

        let llm_response = adapter
            .parse_response(Uuid::new_v4(), "gemini-2.5-flash", response)
            .unwrap();

        assert_eq!(llm_response.content, "Hello there");
        assert!(matches!(llm_response.finish_reason, FinishReason::Stop));
        assert_eq!(llm_response.usage.prompt_tokens, 5);
        assert_eq!(llm_response.usage.completion_tokens, 3);
        assert_eq!(llm_response.usage.total_tokens, 8);
    }

    // ── Error mapping ──

    #[test]
    fn map_error_429_with_resource_exhausted_status_maps_to_rate_limit_exceeded() {
        let adapter = test_adapter("https://example.invalid");
        let body = json!({
            "error": {"code": 429, "message": "Quota exceeded", "status": "RESOURCE_EXHAUSTED"}
        })
        .to_string();

        let error = adapter.map_error(429, &body);
        assert!(matches!(error, LlmError::RateLimitExceeded));
    }

    #[test]
    fn map_error_non_429_with_resource_exhausted_status_still_maps_to_rate_limit_exceeded() {
        let adapter = test_adapter("https://example.invalid");
        let body = json!({
            "error": {"code": 200, "message": "Quota exceeded", "status": "RESOURCE_EXHAUSTED"}
        })
        .to_string();

        // The disposition is keyed on the RPC status, not solely the HTTP
        // code, since Google's error envelope is ambiguous on HTTP 429
        // alone (see this adapter's map_error doc comment).
        let error = adapter.map_error(500, &body);
        assert!(matches!(error, LlmError::RateLimitExceeded));
    }

    #[test]
    fn map_error_400_invalid_argument_maps_to_invalid_prompt() {
        let adapter = test_adapter("https://example.invalid");
        let body = json!({
            "error": {"code": 400, "message": "Bad request", "status": "INVALID_ARGUMENT"}
        })
        .to_string();

        let error = adapter.map_error(400, &body);
        assert!(matches!(error, LlmError::InvalidPrompt(_)));
    }

    #[test]
    fn map_error_404_not_found_maps_to_model_not_available() {
        let adapter = test_adapter("https://example.invalid");
        let body = json!({
            "error": {"code": 404, "message": "Model not found", "status": "NOT_FOUND"}
        })
        .to_string();

        let error = adapter.map_error(404, &body);
        assert!(matches!(error, LlmError::ModelNotAvailable(_)));
    }

    #[test]
    fn map_error_401_permission_denied_maps_to_authentication_error() {
        let adapter = test_adapter("https://example.invalid");
        let body = json!({
            "error": {"code": 401, "message": "Invalid key", "status": "PERMISSION_DENIED"}
        })
        .to_string();

        let error = adapter.map_error(401, &body);
        assert!(matches!(error, LlmError::AuthenticationError(_)));
    }

    #[test]
    fn map_error_echoing_400_never_leaks_the_configured_api_key() {
        let adapter = test_adapter("https://example.invalid");
        let secret = "test-key-abc123";
        let body = json!({
            "error": {
                "code": 400,
                "message": format!("Bad request, header x-goog-api-key: {secret} was rejected"),
                "status": "INVALID_ARGUMENT"
            }
        })
        .to_string();

        let error = adapter.map_error(400, &body);
        let rendered = error.to_string();
        assert!(
            !rendered.contains(secret),
            "map_error leaked the configured API key: {rendered}"
        );
    }

    #[test]
    fn map_error_unrecognised_status_maps_to_processing_error_carrying_http_code_and_status() {
        let adapter = test_adapter("https://example.invalid");
        let body = json!({
            "error": {"code": 500, "message": "Internal error", "status": "INTERNAL"}
        })
        .to_string();

        let error = adapter.map_error(500, &body);
        match error {
            LlmError::ProcessingError(msg) => {
                assert!(msg.contains("500"), "got {msg}");
                assert!(msg.contains("INTERNAL"), "got {msg}");
            }
            other => panic!("expected ProcessingError, got {other:?}"),
        }
    }

    // ── Capabilities / identity ──

    #[test]
    fn get_capabilities_reports_text_only_truthfully() {
        let adapter = test_adapter("https://example.invalid");
        let caps = adapter.get_capabilities();

        assert!(caps.supports_streaming);
        assert!(caps.supports_system_messages);
        assert!(!caps.supports_tool_calling);
        assert!(!caps.supports_function_calling);
        assert!(!caps.supports_vision);
        assert!(!caps.supports_embeddings);
        assert_eq!(caps.temperature_range, Some((0.0, 2.0)));
    }

    #[test]
    fn get_provider_name_returns_gemini() {
        let adapter = test_adapter("https://example.invalid");
        assert_eq!(adapter.get_provider_name(), "gemini");
    }

    // ── Retry semantics ──

    #[tokio::test(start_paused = true)]
    async fn execute_with_retry_invokes_operation_exactly_once_on_non_retryable_error() {
        let adapter = test_adapter("https://example.invalid");
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
        assert_eq!(calls.load(Ordering::SeqCst), 1);
    }

    #[tokio::test(start_paused = true)]
    async fn execute_with_retry_retries_a_retryable_error_up_to_max_retries() {
        let adapter = test_adapter("https://example.invalid");
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
        assert_eq!(calls.load(Ordering::SeqCst), 3);
    }

    // ── generate(): request shaping + response parsing over mock transport ──

    #[tokio::test]
    async fn generate_posts_to_generate_content_with_x_goog_api_key_header() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("POST", "/models/gemini-2.5-flash:generateContent")
            .match_header("x-goog-api-key", "test-key-abc123")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(
                json!({
                    "candidates": [{
                        "content": {"role": "model", "parts": [{"text": "Hi there"}]},
                        "finishReason": "STOP"
                    }],
                    "usageMetadata": {
                        "promptTokenCount": 2,
                        "candidatesTokenCount": 2,
                        "totalTokenCount": 4
                    }
                })
                .to_string(),
            )
            .create_async()
            .await;

        let adapter = test_adapter(&server.url());
        let request = build_request(
            "gemini-2.5-flash",
            PromptType::User(UserPrompt {
                query: "Hello".to_string(),
                context: None,
            }),
        );

        let response = adapter.generate(request).await.unwrap();
        assert_eq!(response.content, "Hi there");
        assert!(matches!(response.finish_reason, FinishReason::Stop));

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn generate_recitation_finish_reason_produces_error_not_stop() {
        let mut server = Server::new_async().await;
        server
            .mock("POST", "/models/gemini-2.5-flash:generateContent")
            .with_status(200)
            .with_body(
                json!({
                    "candidates": [{
                        "content": {"role": "model", "parts": [{"text": "partial"}]},
                        "finishReason": "RECITATION"
                    }]
                })
                .to_string(),
            )
            .create_async()
            .await;

        let adapter = test_adapter(&server.url());
        let request = build_request(
            "gemini-2.5-flash",
            PromptType::User(UserPrompt {
                query: "Hello".to_string(),
                context: None,
            }),
        );

        let response = adapter.generate(request).await.unwrap();
        match response.finish_reason {
            FinishReason::Error(reason) => assert_eq!(reason, "RECITATION"),
            other => panic!("expected FinishReason::Error(\"RECITATION\"), got {other:?}"),
        }
    }

    #[tokio::test]
    async fn generate_429_with_resource_exhausted_body_maps_to_rate_limit_exceeded() {
        let mut server = Server::new_async().await;
        server
            .mock("POST", "/models/gemini-2.5-flash:generateContent")
            .with_status(429)
            .with_body(
                json!({
                    "error": {"code": 429, "message": "Quota exceeded", "status": "RESOURCE_EXHAUSTED"}
                })
                .to_string(),
            )
            .create_async()
            .await;

        let adapter = test_adapter(&server.url());
        let request = build_request(
            "gemini-2.5-flash",
            PromptType::User(UserPrompt {
                query: "Hello".to_string(),
                context: None,
            }),
        );

        let result = adapter.generate(request).await;
        assert!(matches!(result, Err(LlmError::RateLimitExceeded)));
    }

    // ── Streaming ──

    #[tokio::test]
    async fn generate_stream_posts_with_alt_sse_query_parameter() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("POST", "/models/gemini-2.5-flash:streamGenerateContent")
            .match_query(Matcher::UrlEncoded("alt".to_string(), "sse".to_string()))
            .expect(1)
            .with_status(200)
            .with_header("content-type", "text/event-stream")
            .with_body(concat!(
                "data: {\"candidates\":[{\"content\":{\"parts\":[{\"text\":\"Hi\"}]},",
                "\"finishReason\":\"STOP\"}]}\n\n",
            ))
            .create_async()
            .await;

        let adapter = test_adapter(&server.url());
        let request = build_request(
            "gemini-2.5-flash",
            PromptType::User(UserPrompt {
                query: "Hello".to_string(),
                context: None,
            }),
        );

        let stream = adapter.generate_stream(request).await.unwrap();
        let mut stream = Box::into_pin(stream);
        while stream.next().await.is_some() {}

        // Proves the query parameter was actually sent — without it Google
        // returns a raw JSON array, not SSE framing.
        mock.assert_async().await;
    }

    #[tokio::test]
    async fn generate_stream_assembles_three_frames_in_wire_order() {
        let sse_body = concat!(
            "data: {\"candidates\":[{\"content\":{\"parts\":[{\"text\":\"Hel\"}]}}]}\n\n",
            "data: {\"candidates\":[{\"content\":{\"parts\":[{\"text\":\"lo \"}]}}]}\n\n",
            "data: {\"candidates\":[{\"content\":{\"parts\":[{\"text\":\"world\"}],\"role\":\"model\"},",
            "\"finishReason\":\"STOP\"}]}\n\n",
        );

        let mut server = Server::new_async().await;
        server
            .mock("POST", "/models/gemini-2.5-flash:streamGenerateContent")
            // Every `generate_stream()` call carries `?alt=sse`; this test
            // doesn't care about the exact query string, only the frames —
            // `generate_stream_posts_with_alt_sse_query_parameter` above
            // asserts the parameter itself.
            .match_query(Matcher::Any)
            .with_status(200)
            .with_header("content-type", "text/event-stream")
            .with_body(sse_body)
            .create_async()
            .await;

        let adapter = test_adapter(&server.url());
        let request = build_request(
            "gemini-2.5-flash",
            PromptType::User(UserPrompt {
                query: "Hello".to_string(),
                context: None,
            }),
        );

        let stream = adapter.generate_stream(request).await.unwrap();
        let mut stream = Box::into_pin(stream);

        let mut assembled = String::new();
        let mut last_finish_reason = None;
        let mut item_count = 0;
        while let Some(item) = stream.next().await {
            let chunk = item.unwrap();
            item_count += 1;
            assembled.push_str(&chunk.delta);
            if chunk.finish_reason.is_some() {
                last_finish_reason = chunk.finish_reason;
            }
        }

        assert_eq!(item_count, 3);
        assert_eq!(assembled, "Hello world");
        assert!(matches!(last_finish_reason, Some(FinishReason::Stop)));
    }

    #[tokio::test]
    async fn generate_stream_safety_blocked_frame_terminates_without_error() {
        let sse_body = "data: {\"candidates\":[{\"finishReason\":\"SAFETY\"}]}\n\n";

        let mut server = Server::new_async().await;
        server
            .mock("POST", "/models/gemini-2.5-flash:streamGenerateContent")
            .match_query(Matcher::Any)
            .with_status(200)
            .with_header("content-type", "text/event-stream")
            .with_body(sse_body)
            .create_async()
            .await;

        let adapter = test_adapter(&server.url());
        let request = build_request(
            "gemini-2.5-flash",
            PromptType::User(UserPrompt {
                query: "Hello".to_string(),
                context: None,
            }),
        );

        let stream = adapter.generate_stream(request).await.unwrap();
        let mut stream = Box::into_pin(stream);

        let mut last_finish_reason = None;
        while let Some(item) = stream.next().await {
            let chunk = item.expect("a safety-blocked frame must not surface as a stream error");
            if chunk.finish_reason.is_some() {
                last_finish_reason = chunk.finish_reason;
            }
        }

        assert!(matches!(
            last_finish_reason,
            Some(FinishReason::ContentFilter)
        ));
    }

    // ── Model list: live catalog vs. curated fallback (D-13/D-14) ──

    #[tokio::test]
    async fn get_available_models_returns_two_live_entries_without_models_prefix() {
        let mut server = Server::new_async().await;
        server
            .mock("GET", "/models")
            .with_status(200)
            .with_body(
                json!({
                    "models": [
                        {"name": "models/gemini-2.5-flash"},
                        {"name": "models/gemini-2.5-pro"}
                    ]
                })
                .to_string(),
            )
            .create_async()
            .await;

        let adapter = test_adapter(&server.url());
        let models = adapter.get_available_models().await.unwrap();
        assert_eq!(
            models,
            vec!["gemini-2.5-flash".to_string(), "gemini-2.5-pro".to_string()]
        );
    }

    #[tokio::test]
    async fn get_available_models_second_call_does_not_hit_the_mock_again() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("GET", "/models")
            .expect(1)
            .with_status(200)
            .with_body(json!({"models": [{"name": "models/gemini-2.5-flash"}]}).to_string())
            .create_async()
            .await;

        let adapter = test_adapter(&server.url());
        let _first = adapter.get_available_models().await.unwrap();
        let _second = adapter.get_available_models().await.unwrap();

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn get_available_models_two_concurrent_first_calls_hit_the_mock_exactly_once() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("GET", "/models")
            .expect(1)
            .with_status(200)
            .with_body(json!({"models": [{"name": "models/gemini-2.5-flash"}]}).to_string())
            .create_async()
            .await;

        let adapter = test_adapter(&server.url());
        let (first, second) = tokio::join!(
            adapter.get_available_models(),
            adapter.get_available_models()
        );
        first.unwrap();
        second.unwrap();

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn get_available_models_falls_back_to_curated_list_on_failure() {
        let mut server = Server::new_async().await;
        server
            .mock("GET", "/models")
            .with_status(500)
            .with_body("internal error")
            .create_async()
            .await;

        let adapter = test_adapter(&server.url());
        let models = adapter.get_available_models().await.unwrap();
        assert_eq!(
            models,
            GEMINI_FALLBACK_MODELS
                .iter()
                .map(|s| s.to_string())
                .collect::<Vec<_>>()
        );

        assert!(adapter.validate_model("gemini-2.5-pro").await.unwrap());
    }

    #[tokio::test]
    async fn validate_model_accepts_a_model_present_only_in_the_live_list() {
        let mut server = Server::new_async().await;
        server
            .mock("GET", "/models")
            .with_status(200)
            .with_body(json!({"models": [{"name": "models/gemini-3.1-flash-lite"}]}).to_string())
            .create_async()
            .await;

        let adapter = test_adapter(&server.url());
        assert!(
            adapter
                .validate_model("gemini-3.1-flash-lite")
                .await
                .unwrap()
        );
    }

    // ── validate_model_identifier: pure-logic tests over the guard itself ──

    #[test]
    fn validate_model_identifier_accepts_the_default_and_every_fallback_model() {
        for model in
            std::iter::once(GEMINI_DEFAULT_MODEL).chain(GEMINI_FALLBACK_MODELS.iter().copied())
        {
            assert!(
                validate_model_identifier(model).is_ok(),
                "expected Ok for shipped default/fallback {model:?}"
            );
        }
    }

    #[test]
    fn validate_model_identifier_rejects_each_url_metacharacter() {
        let metacharacters = ['/', '?', '#', ':', '@', '%', '&', '=', ' ', '\\', '\n'];
        for c in metacharacters {
            let hostile = format!("gemini-2.5-flash{c}x");
            let result = validate_model_identifier(&hostile);
            assert!(
                matches!(result, Err(LlmError::InvalidPrompt(_))),
                "expected LlmError::InvalidPrompt for metacharacter {c:?} in {hostile:?}, got {result:?}"
            );
        }
    }

    #[test]
    fn validate_model_identifier_rejects_a_value_with_no_alphanumeric_character() {
        for value in [".", "..", "---", "_"] {
            let result = validate_model_identifier(value);
            assert!(
                matches!(result, Err(LlmError::InvalidPrompt(_))),
                "expected LlmError::InvalidPrompt for {value:?} (no alphanumeric char), got {result:?}"
            );
        }
    }

    #[test]
    fn validate_model_identifier_rejects_a_long_multibyte_value_without_panicking() {
        // A 2,000-character multi-byte value. The test completing at all —
        // rather than panicking on a mid-codepoint byte slice — is part of
        // what this test proves.
        let hostile: String = "\u{1F5E1}".repeat(2000);
        let result = validate_model_identifier(&hostile);

        let message = match result {
            Err(LlmError::InvalidPrompt(msg)) => msg,
            other => panic!("expected LlmError::InvalidPrompt, got {other:?}"),
        };

        // The embedded excerpt is capped at RESPONSE_EXCERPT_CHAR_BUDGET
        // characters; the surrounding sentence and elision marker add a
        // small, fixed amount on top. The bound below allows for that
        // fixed prefix/suffix without allowing the excerpt itself to grow
        // unbounded.
        assert!(
            message.chars().count() <= RESPONSE_EXCERPT_CHAR_BUDGET + 400,
            "rejection message was not bounded: {} chars",
            message.chars().count()
        );
    }

    #[test]
    fn validate_model_identifier_accepts_every_character_of_the_allowed_set() {
        assert!(validate_model_identifier("aZ0.9_x-1").is_ok());
    }

    // ── CR-01: a caller-supplied model identifier must never reach the wire
    //    unescaped — regression tests proving the guard above is actually
    //    wired into both LlmPort methods. See
    //    `.planning/phases/17-additional-llm-provider-adapters/17-REVIEW.md`
    //    §CR-01 and `17-VERIFICATION.md`. ──

    #[tokio::test]
    async fn generate_rejects_a_model_containing_a_path_separator_without_issuing_a_request() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("POST", Matcher::Any)
            .match_query(Matcher::Any)
            .expect(0)
            .with_status(200)
            .with_body("{}")
            .create_async()
            .await;

        let adapter = test_adapter(&server.url());
        let request = build_request(
            "gemini-2.5-flash/../v1beta/models/other",
            PromptType::Text(TextPrompt {
                content: "Hello".to_string(),
                role: PromptRole::User,
            }),
        );

        let result = adapter.generate(request).await;
        assert!(
            matches!(&result, Err(LlmError::InvalidPrompt(_))),
            "expected LlmError::InvalidPrompt, got {:?}",
            result.err()
        );

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn generate_stream_rejects_a_model_containing_a_query_delimiter_without_issuing_a_request()
     {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("POST", Matcher::Any)
            .match_query(Matcher::Any)
            .expect(0)
            .with_status(200)
            .with_body("{}")
            .create_async()
            .await;

        let adapter = test_adapter(&server.url());
        let request = build_request(
            "gemini-2.5-flash?alt=json",
            PromptType::Text(TextPrompt {
                content: "Hello".to_string(),
                role: PromptRole::User,
            }),
        );

        let result = adapter.generate_stream(request).await;
        assert!(
            matches!(&result, Err(LlmError::InvalidPrompt(_))),
            "expected LlmError::InvalidPrompt, got {:?}",
            result.as_ref().err()
        );

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn generate_rejects_a_model_containing_a_colon_operation_suffix() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("POST", Matcher::Any)
            .match_query(Matcher::Any)
            .expect(0)
            .with_status(200)
            .with_body("{}")
            .create_async()
            .await;

        let adapter = test_adapter(&server.url());
        let request = build_request(
            "gemini-2.5-flash:streamGenerateContent",
            PromptType::Text(TextPrompt {
                content: "Hello".to_string(),
                role: PromptRole::User,
            }),
        );

        let result = adapter.generate(request).await;
        assert!(
            matches!(&result, Err(LlmError::InvalidPrompt(_))),
            "expected LlmError::InvalidPrompt, got {:?}",
            result.err()
        );

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn generate_rejects_a_model_containing_a_fragment_delimiter() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("POST", Matcher::Any)
            .match_query(Matcher::Any)
            .expect(0)
            .with_status(200)
            .with_body("{}")
            .create_async()
            .await;

        let adapter = test_adapter(&server.url());
        let request = build_request(
            "gemini-2.5-flash#anchor",
            PromptType::Text(TextPrompt {
                content: "Hello".to_string(),
                role: PromptRole::User,
            }),
        );

        let result = adapter.generate(request).await;
        assert!(
            matches!(&result, Err(LlmError::InvalidPrompt(_))),
            "expected LlmError::InvalidPrompt, got {:?}",
            result.err()
        );

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn generate_rejects_an_empty_model() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("POST", Matcher::Any)
            .match_query(Matcher::Any)
            .expect(0)
            .with_status(200)
            .with_body("{}")
            .create_async()
            .await;

        let adapter = test_adapter(&server.url());
        let request = build_request(
            "",
            PromptType::Text(TextPrompt {
                content: "Hello".to_string(),
                role: PromptRole::User,
            }),
        );

        let result = adapter.generate(request).await;
        assert!(
            matches!(&result, Err(LlmError::InvalidPrompt(_))),
            "expected LlmError::InvalidPrompt, got {:?}",
            result.err()
        );

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn generate_rejects_a_model_containing_a_non_ascii_homoglyph() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("POST", Matcher::Any)
            .match_query(Matcher::Any)
            .expect(0)
            .with_status(200)
            .with_body("{}")
            .create_async()
            .await;

        let adapter = test_adapter(&server.url());
        // Cyrillic small letter A (U+0430) in place of the second ASCII `a`
        // — written as an escape, not a raw glyph, so a reviewer can see
        // which character is which.
        let request = build_request(
            "g\u{0430}mini-2.5-flash",
            PromptType::Text(TextPrompt {
                content: "Hello".to_string(),
                role: PromptRole::User,
            }),
        );

        let result = adapter.generate(request).await;
        assert!(
            matches!(&result, Err(LlmError::InvalidPrompt(_))),
            "expected LlmError::InvalidPrompt, got {:?}",
            result.err()
        );

        mock.assert_async().await;
    }

    /// Positive control: proves the guard does not over-reject a value whose
    /// characters are all in the allowed set. Must pass both before and
    /// after Task 2 wires the guard in.
    #[tokio::test]
    async fn generate_accepts_a_model_whose_characters_are_all_in_the_allowed_set() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock(
                "POST",
                "/models/gemini-2.5-flash_preview.01-x:generateContent",
            )
            .match_header("x-goog-api-key", "test-key-abc123")
            .expect(1)
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(
                json!({
                    "candidates": [{
                        "content": {"role": "model", "parts": [{"text": "Hi there"}]},
                        "finishReason": "STOP"
                    }],
                    "usageMetadata": {
                        "promptTokenCount": 2,
                        "candidatesTokenCount": 2,
                        "totalTokenCount": 4
                    }
                })
                .to_string(),
            )
            .create_async()
            .await;

        let adapter = test_adapter(&server.url());
        let request = build_request(
            "gemini-2.5-flash_preview.01-x",
            PromptType::Text(TextPrompt {
                content: "Hello".to_string(),
                role: PromptRole::User,
            }),
        );

        let result = adapter.generate(request).await;
        assert!(result.is_ok(), "expected Ok, got {:?}", result.err());

        mock.assert_async().await;
    }
}
