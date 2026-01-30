use mockito::Server;
use paladin::application::ports::output::embedding_port::{EmbeddingError, EmbeddingPort};
use paladin::infrastructure::adapters::llm::openai_embedding_adapter::{
    OpenAIEmbeddingAdapter, OpenAIEmbeddingConfig,
};
use serde_json::json;

#[cfg(test)]
mod openai_embedding_integration_tests {
    use super::*;

    fn create_test_config(server_url: &str) -> OpenAIEmbeddingConfig {
        OpenAIEmbeddingConfig {
            api_key: "test-key".to_string(),
            model: "text-embedding-3-small".to_string(),
            base_url: server_url.to_string(),
            max_retries: 2,
            timeout_seconds: 5,
        }
    }

    #[tokio::test]
    async fn test_successful_single_embedding() {
        let mut server = Server::new_async().await;

        let mock = server
            .mock("POST", "/embeddings")
            .match_header("authorization", "Bearer test-key")
            .match_header("content-type", "application/json")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(
                json!({
                    "object": "list",
                    "data": [
                        {
                            "object": "embedding",
                            "embedding": vec![0.1, 0.2, 0.3],
                            "index": 0
                        }
                    ],
                    "model": "text-embedding-3-small",
                    "usage": {
                        "prompt_tokens": 5,
                        "total_tokens": 5
                    }
                })
                .to_string(),
            )
            .create_async()
            .await;

        let config = create_test_config(&server.url());
        let adapter = OpenAIEmbeddingAdapter::new(config);

        let result = adapter.embed_text("Hello, world!").await;

        mock.assert_async().await;
        assert!(result.is_ok());

        let embedding = result.unwrap();
        assert_eq!(embedding.vector, vec![0.1, 0.2, 0.3]);
        assert_eq!(embedding.model, "text-embedding-3-small");
        assert_eq!(embedding.dimension, 3);
        assert_eq!(embedding.token_count, Some(5));
    }

    #[tokio::test]
    async fn test_successful_batch_embedding() {
        let mut server = Server::new_async().await;

        let mock = server
            .mock("POST", "/embeddings")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(
                json!({
                    "object": "list",
                    "data": [
                        {
                            "object": "embedding",
                            "embedding": vec![0.1, 0.2],
                            "index": 0
                        },
                        {
                            "object": "embedding",
                            "embedding": vec![0.3, 0.4],
                            "index": 1
                        }
                    ],
                    "model": "text-embedding-3-small",
                    "usage": {
                        "prompt_tokens": 10,
                        "total_tokens": 10
                    }
                })
                .to_string(),
            )
            .create_async()
            .await;

        let config = create_test_config(&server.url());
        let adapter = OpenAIEmbeddingAdapter::new(config);

        let result = adapter.embed_batch(&["text1", "text2"]).await;

        mock.assert_async().await;
        assert!(result.is_ok());

        let embeddings = result.unwrap();
        assert_eq!(embeddings.len(), 2);
        assert_eq!(embeddings[0].vector, vec![0.1, 0.2]);
        assert_eq!(embeddings[1].vector, vec![0.3, 0.4]);
    }

    #[tokio::test]
    async fn test_rate_limit_error() {
        let mut server = Server::new_async().await;

        let mock = server
            .mock("POST", "/embeddings")
            .with_status(429)
            .with_header("content-type", "application/json")
            .with_body(
                json!({
                    "error": {
                        "message": "Rate limit exceeded",
                        "type": "rate_limit_error"
                    }
                })
                .to_string(),
            )
            .expect(3) // Initial attempt + 2 retries (max_retries=2 in config)
            .create_async()
            .await;

        let config = create_test_config(&server.url());
        let adapter = OpenAIEmbeddingAdapter::new(config);

        let result = adapter.embed_text("test").await;

        mock.assert_async().await;
        assert!(result.is_err());

        match result.unwrap_err() {
            EmbeddingError::RateLimited(_) => (),
            e => panic!("Expected RateLimited error, got: {:?}", e),
        }
    }

    #[tokio::test]
    async fn test_network_error() {
        let config = OpenAIEmbeddingConfig {
            api_key: "test-key".to_string(),
            model: "text-embedding-3-small".to_string(),
            base_url: "http://invalid-url-that-does-not-exist-12345.com".to_string(),
            max_retries: 1,
            timeout_seconds: 1,
        };

        let adapter = OpenAIEmbeddingAdapter::new(config);
        let result = adapter.embed_text("test").await;

        assert!(result.is_err());
        match result.unwrap_err() {
            EmbeddingError::NetworkError(_) => (),
            e => panic!("Expected NetworkError, got: {:?}", e),
        }
    }

    #[tokio::test]
    async fn test_invalid_input_empty_text() {
        let server = Server::new_async().await;
        let config = create_test_config(&server.url());
        let adapter = OpenAIEmbeddingAdapter::new(config);

        let result = adapter.embed_text("").await;

        assert!(result.is_err());
        match result.unwrap_err() {
            EmbeddingError::InvalidInput(_) => (),
            e => panic!("Expected InvalidInput error, got: {:?}", e),
        }
    }

    #[tokio::test]
    async fn test_batch_size_limit() {
        let server = Server::new_async().await;
        let config = create_test_config(&server.url());
        let adapter = OpenAIEmbeddingAdapter::new(config);

        // Create more than 2048 texts (API limit)
        let texts: Vec<&str> = (0..2049).map(|_| "test").collect();
        let text_refs: Vec<&str> = texts.iter().map(|s| s.as_ref()).collect();

        let result = adapter.embed_batch(&text_refs).await;

        assert!(result.is_err());
        match result.unwrap_err() {
            EmbeddingError::InvalidInput(msg) => {
                assert!(msg.contains("2048") || msg.contains("batch size"));
            }
            e => panic!("Expected InvalidInput error for batch size, got: {:?}", e),
        }
    }

    #[tokio::test]
    async fn test_dimension_method() {
        let server = Server::new_async().await;

        // Test text-embedding-3-small (1536 dimensions)
        let config_small = OpenAIEmbeddingConfig {
            api_key: "test-key".to_string(),
            model: "text-embedding-3-small".to_string(),
            base_url: server.url(),
            max_retries: 2,
            timeout_seconds: 5,
        };
        let adapter_small = OpenAIEmbeddingAdapter::new(config_small);
        assert_eq!(adapter_small.dimension(), 1536);

        // Test text-embedding-3-large (3072 dimensions)
        let config_large = OpenAIEmbeddingConfig {
            api_key: "test-key".to_string(),
            model: "text-embedding-3-large".to_string(),
            base_url: server.url(),
            max_retries: 2,
            timeout_seconds: 5,
        };
        let adapter_large = OpenAIEmbeddingAdapter::new(config_large);
        assert_eq!(adapter_large.dimension(), 3072);

        // Test text-embedding-ada-002 (1536 dimensions)
        let config_ada = OpenAIEmbeddingConfig {
            api_key: "test-key".to_string(),
            model: "text-embedding-ada-002".to_string(),
            base_url: server.url(),
            max_retries: 2,
            timeout_seconds: 5,
        };
        let adapter_ada = OpenAIEmbeddingAdapter::new(config_ada);
        assert_eq!(adapter_ada.dimension(), 1536);
    }

    #[tokio::test]
    async fn test_model_name_method() {
        let server = Server::new_async().await;
        let config = create_test_config(&server.url());
        let adapter = OpenAIEmbeddingAdapter::new(config);

        assert_eq!(adapter.model_name(), "text-embedding-3-small");
    }
}
