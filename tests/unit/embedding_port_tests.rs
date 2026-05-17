use paladin_ports::output::embedding_port::{Embedding, EmbeddingError};

#[cfg(test)]
mod embedding_error_tests {
    use super::*;

    #[test]
    fn test_network_error_display() {
        let error = EmbeddingError::NetworkError("Connection timeout".to_string());
        assert_eq!(error.to_string(), "Network error: Connection timeout");
    }

    #[test]
    fn test_rate_limited_error_display() {
        let error = EmbeddingError::RateLimited("Too many requests".to_string());
        assert_eq!(error.to_string(), "Rate limited: Too many requests");
    }

    #[test]
    fn test_invalid_input_error_display() {
        let error = EmbeddingError::InvalidInput("Empty text".to_string());
        assert_eq!(error.to_string(), "Invalid input: Empty text");
    }

    #[test]
    fn test_provider_error_display() {
        let error = EmbeddingError::ProviderError("API key invalid".to_string());
        assert_eq!(error.to_string(), "Provider error: API key invalid");
    }

    #[test]
    fn test_error_is_send_sync() {
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<EmbeddingError>();
    }
}

#[cfg(test)]
mod embedding_tests {
    use super::*;

    #[test]
    fn test_embedding_creation() {
        let vector = vec![0.1, 0.2, 0.3];
        let embedding = Embedding {
            vector: vector.clone(),
            model: "test-model".to_string(),
            dimension: 3,
            token_count: Some(10),
        };

        assert_eq!(embedding.vector, vector);
        assert_eq!(embedding.model, "test-model");
        assert_eq!(embedding.dimension, 3);
        assert_eq!(embedding.token_count, Some(10));
    }

    #[test]
    fn test_embedding_without_token_count() {
        let embedding = Embedding {
            vector: vec![0.5, 0.6],
            model: "model-2".to_string(),
            dimension: 2,
            token_count: None,
        };

        assert_eq!(embedding.dimension, 2);
        assert_eq!(embedding.token_count, None);
    }

    #[test]
    fn test_embedding_serialization() {
        let embedding = Embedding {
            vector: vec![1.0, 2.0, 3.0],
            model: "gpt-embed".to_string(),
            dimension: 3,
            token_count: Some(5),
        };

        let json = serde_json::to_string(&embedding).unwrap();
        let deserialized: Embedding = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.vector, embedding.vector);
        assert_eq!(deserialized.model, embedding.model);
        assert_eq!(deserialized.dimension, embedding.dimension);
        assert_eq!(deserialized.token_count, embedding.token_count);
    }

    #[test]
    fn test_embedding_clone() {
        let original = Embedding {
            vector: vec![0.1, 0.2],
            model: "test".to_string(),
            dimension: 2,
            token_count: Some(1),
        };

        let cloned = original.clone();
        assert_eq!(cloned.vector, original.vector);
        assert_eq!(cloned.model, original.model);
        assert_eq!(cloned.dimension, original.dimension);
    }

    #[test]
    fn test_embedding_is_send_sync() {
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<Embedding>();
    }
}
