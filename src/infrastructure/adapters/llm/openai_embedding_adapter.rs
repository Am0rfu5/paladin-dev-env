/// OpenAI Embedding Adapter
///
/// Implementation of EmbeddingPort for OpenAI's embedding API.
/// Supports text-embedding-3-small, text-embedding-3-large, and text-embedding-ada-002 models.
///
/// # Features
/// - Automatic retry with exponential backoff
/// - Batch processing with API limit enforcement (max 2048 inputs)
/// - Comprehensive error handling
/// - Configurable timeouts and retry attempts
use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::Duration;

use paladin_ports::output::embedding_port::{Embedding, EmbeddingError, EmbeddingPort};

/// Configuration for OpenAI Embedding Adapter
#[doc(hidden)]
#[derive(Debug, Clone)]
pub struct OpenAIEmbeddingConfig {
    /// OpenAI API key
    pub api_key: String,

    /// Model to use for embeddings
    /// Supported: "text-embedding-3-small", "text-embedding-3-large", "text-embedding-ada-002"
    pub model: String,

    /// Base URL for OpenAI API
    pub base_url: String,

    /// Maximum number of retry attempts for failed requests
    pub max_retries: u32,

    /// Request timeout in seconds
    pub timeout_seconds: u64,
}

impl Default for OpenAIEmbeddingConfig {
    fn default() -> Self {
        Self {
            api_key: String::new(),
            model: "text-embedding-3-small".to_string(),
            base_url: "https://api.openai.com/v1".to_string(),
            max_retries: 3,
            timeout_seconds: 30,
        }
    }
}

/// OpenAI Embedding Adapter
#[doc(hidden)]
pub struct OpenAIEmbeddingAdapter {
    client: Client,
    config: OpenAIEmbeddingConfig,
}

impl OpenAIEmbeddingAdapter {
    /// Create a new OpenAI embedding adapter
    ///
    /// # Arguments
    ///
    /// * `config` - Configuration for the adapter
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use paladin::infrastructure::adapters::llm::openai_embedding_adapter::{
    ///     OpenAIEmbeddingAdapter, OpenAIEmbeddingConfig
    /// };
    ///
    /// let config = OpenAIEmbeddingConfig {
    ///     api_key: "sk-...".to_string(),
    ///     model: "text-embedding-3-small".to_string(),
    ///     ..Default::default()
    /// };
    ///
    /// let adapter = OpenAIEmbeddingAdapter::new(config);
    /// ```
    pub fn new(config: OpenAIEmbeddingConfig) -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(config.timeout_seconds))
            .build()
            .expect("Failed to create HTTP client");

        Self { client, config }
    }

    /// Get the dimension for the configured model
    fn get_model_dimension(&self) -> usize {
        match self.config.model.as_str() {
            "text-embedding-3-small" => 1536,
            "text-embedding-3-large" => 3072,
            "text-embedding-ada-002" => 1536,
            _ => 1536, // Default to 1536 for unknown models
        }
    }

    /// Make an API request with retry logic
    async fn make_request(
        &self,
        texts: Vec<String>,
    ) -> Result<OpenAIEmbeddingResponse, EmbeddingError> {
        let mut attempt = 0;
        let mut last_error = None;

        while attempt <= self.config.max_retries {
            let request = OpenAIEmbeddingRequest {
                input: texts.clone(),
                model: self.config.model.clone(),
            };

            let response = self
                .client
                .post(format!("{}/embeddings", self.config.base_url))
                .header("Authorization", format!("Bearer {}", self.config.api_key))
                .header("Content-Type", "application/json")
                .json(&request)
                .send()
                .await;

            match response {
                Ok(resp) => {
                    let status = resp.status();

                    if status.is_success() {
                        let body = resp.text().await.map_err(|e| {
                            EmbeddingError::NetworkError(format!("Failed to read response: {}", e))
                        })?;

                        let embedding_response: OpenAIEmbeddingResponse =
                            serde_json::from_str(&body).map_err(|e| {
                                EmbeddingError::ProviderError(format!(
                                    "Failed to parse response: {}",
                                    e
                                ))
                            })?;

                        return Ok(embedding_response);
                    } else if status.as_u16() == 429 {
                        // Rate limited - retry with backoff
                        last_error = Some(EmbeddingError::RateLimited(format!(
                            "Rate limit exceeded (attempt {}/{})",
                            attempt + 1,
                            self.config.max_retries + 1
                        )));

                        if attempt < self.config.max_retries {
                            let backoff_ms = 1000 * (2_u64.pow(attempt));
                            tokio::time::sleep(Duration::from_millis(backoff_ms)).await;
                        }
                    } else {
                        // Other error - try to parse error message
                        let error_text = resp
                            .text()
                            .await
                            .unwrap_or_else(|_| "Unknown error".to_string());
                        return Err(EmbeddingError::ProviderError(format!(
                            "API error {}: {}",
                            status, error_text
                        )));
                    }
                }
                Err(e) => {
                    last_error = Some(EmbeddingError::NetworkError(format!(
                        "Network error (attempt {}/{}): {}",
                        attempt + 1,
                        self.config.max_retries + 1,
                        e
                    )));

                    if attempt < self.config.max_retries {
                        let backoff_ms = 1000 * (2_u64.pow(attempt));
                        tokio::time::sleep(Duration::from_millis(backoff_ms)).await;
                    }
                }
            }

            attempt += 1;
        }

        Err(last_error
            .unwrap_or_else(|| EmbeddingError::NetworkError("Max retries exceeded".to_string())))
    }
}

#[async_trait]
impl EmbeddingPort for OpenAIEmbeddingAdapter {
    async fn embed_text(&self, text: &str) -> Result<Embedding, EmbeddingError> {
        // Validate input
        if text.is_empty() {
            return Err(EmbeddingError::InvalidInput(
                "Text cannot be empty".to_string(),
            ));
        }

        let response = self.make_request(vec![text.to_string()]).await?;

        if response.data.is_empty() {
            return Err(EmbeddingError::ProviderError(
                "No embeddings returned".to_string(),
            ));
        }

        let embedding_data = &response.data[0];
        let dimension = embedding_data.embedding.len();

        Ok(Embedding {
            vector: embedding_data.embedding.clone(),
            model: response.model,
            dimension,
            token_count: Some(response.usage.prompt_tokens),
        })
    }

    async fn embed_batch(&self, texts: &[&str]) -> Result<Vec<Embedding>, EmbeddingError> {
        // Validate batch size
        if texts.is_empty() {
            return Err(EmbeddingError::InvalidInput(
                "Batch cannot be empty".to_string(),
            ));
        }

        if texts.len() > 2048 {
            return Err(EmbeddingError::InvalidInput(format!(
                "Batch size {} exceeds OpenAI limit of 2048",
                texts.len()
            )));
        }

        // Check for empty strings
        for (i, text) in texts.iter().enumerate() {
            if text.is_empty() {
                return Err(EmbeddingError::InvalidInput(format!(
                    "Text at index {} is empty",
                    i
                )));
            }
        }

        let text_strings: Vec<String> = texts.iter().map(|s| s.to_string()).collect();
        let response = self.make_request(text_strings).await?;

        if response.data.len() != texts.len() {
            return Err(EmbeddingError::ProviderError(format!(
                "Expected {} embeddings, got {}",
                texts.len(),
                response.data.len()
            )));
        }

        let embeddings = response
            .data
            .into_iter()
            .map(|data| {
                let dimension = data.embedding.len();
                Embedding {
                    vector: data.embedding,
                    model: response.model.clone(),
                    dimension,
                    token_count: Some(response.usage.prompt_tokens / texts.len() as u32),
                }
            })
            .collect();

        Ok(embeddings)
    }

    fn dimension(&self) -> usize {
        self.get_model_dimension()
    }

    fn model_name(&self) -> &str {
        &self.config.model
    }
}

// API Request/Response structures

#[derive(Debug, Serialize)]
struct OpenAIEmbeddingRequest {
    input: Vec<String>,
    model: String,
}

#[derive(Debug, Deserialize)]
struct OpenAIEmbeddingResponse {
    data: Vec<OpenAIEmbeddingData>,
    model: String,
    usage: OpenAIUsage,
}

#[derive(Debug, Deserialize)]
struct OpenAIEmbeddingData {
    embedding: Vec<f32>,
    #[allow(dead_code)]
    index: usize,
}

#[derive(Debug, Deserialize)]
struct OpenAIUsage {
    prompt_tokens: u32,
    #[allow(dead_code)]
    total_tokens: u32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_default() {
        let config = OpenAIEmbeddingConfig::default();
        assert_eq!(config.model, "text-embedding-3-small");
        assert_eq!(config.base_url, "https://api.openai.com/v1");
        assert_eq!(config.max_retries, 3);
        assert_eq!(config.timeout_seconds, 30);
    }

    #[test]
    fn test_dimension_mapping() {
        let config_small = OpenAIEmbeddingConfig {
            model: "text-embedding-3-small".to_string(),
            ..Default::default()
        };
        let adapter_small = OpenAIEmbeddingAdapter::new(config_small);
        assert_eq!(adapter_small.get_model_dimension(), 1536);

        let config_large = OpenAIEmbeddingConfig {
            model: "text-embedding-3-large".to_string(),
            ..Default::default()
        };
        let adapter_large = OpenAIEmbeddingAdapter::new(config_large);
        assert_eq!(adapter_large.get_model_dimension(), 3072);

        let config_ada = OpenAIEmbeddingConfig {
            model: "text-embedding-ada-002".to_string(),
            ..Default::default()
        };
        let adapter_ada = OpenAIEmbeddingAdapter::new(config_ada);
        assert_eq!(adapter_ada.get_model_dimension(), 1536);
    }
}
