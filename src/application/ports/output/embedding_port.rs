/// Embedding generation errors
#[derive(Debug, thiserror::Error)]
pub enum EmbeddingError {
    /// Network-related errors (timeouts, connection failures)
    #[error("Network error: {0}")]
    NetworkError(String),

    /// Rate limiting errors from the embedding provider
    #[error("Rate limited: {0}")]
    RateLimited(String),

    /// Invalid input provided to the embedding service
    #[error("Invalid input: {0}")]
    InvalidInput(String),

    /// Provider-specific errors (API key issues, model not found, etc.)
    #[error("Provider error: {0}")]
    ProviderError(String),
}

/// A vector embedding with associated metadata
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Embedding {
    /// The embedding vector
    pub vector: Vec<f32>,

    /// The model used to generate this embedding
    pub model: String,

    /// The dimension of the embedding vector
    pub dimension: usize,

    /// Optional token count for the embedded text
    pub token_count: Option<u32>,
}

/// Port trait for generating vector embeddings from text
///
/// This trait provides a standardized interface for embedding providers,
/// allowing the system to plug in different embedding services (OpenAI, local models, etc.)
/// without changing core business logic.
///
/// # Examples
///
/// ```ignore
/// use paladin::application::ports::output::embedding_port::{EmbeddingPort, Embedding};
///
/// async fn example(port: &dyn EmbeddingPort) {
///     let text = "Hello, world!";
///     let embedding = port.embed_text(text).await.unwrap();
///     println!("Vector dimension: {}", embedding.dimension);
/// }
/// ```
#[async_trait::async_trait]
pub trait EmbeddingPort: Send + Sync {
    /// Generate an embedding for a single text string
    ///
    /// # Arguments
    ///
    /// * `text` - The text to embed
    ///
    /// # Returns
    ///
    /// Returns an `Embedding` containing the vector and metadata, or an `EmbeddingError`
    ///
    /// # Errors
    ///
    /// * `NetworkError` - If there's a network issue connecting to the provider
    /// * `RateLimited` - If the rate limit has been exceeded
    /// * `InvalidInput` - If the text is empty or exceeds size limits
    /// * `ProviderError` - If there's an API key issue or other provider-specific error
    async fn embed_text(&self, text: &str) -> Result<Embedding, EmbeddingError>;

    /// Generate embeddings for multiple text strings in a batch
    ///
    /// This is more efficient than calling `embed_text` multiple times,
    /// as it can leverage the provider's batch API.
    ///
    /// # Arguments
    ///
    /// * `texts` - A slice of text strings to embed
    ///
    /// # Returns
    ///
    /// Returns a vector of `Embedding` objects in the same order as the input,
    /// or an `EmbeddingError` if the entire batch fails
    ///
    /// # Errors
    ///
    /// Same error types as `embed_text`, plus provider-specific batch size limits
    async fn embed_batch(&self, texts: &[&str]) -> Result<Vec<Embedding>, EmbeddingError>;

    /// Get the dimension of embeddings produced by this provider
    ///
    /// # Returns
    ///
    /// The number of dimensions in the embedding vector (e.g., 1536, 3072)
    fn dimension(&self) -> usize;

    /// Get the name of the model used by this provider
    ///
    /// # Returns
    ///
    /// The model identifier (e.g., "text-embedding-3-small")
    fn model_name(&self) -> &str;
}
