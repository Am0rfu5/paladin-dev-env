/// Unit tests for Memory Extraction Service
use std::collections::HashMap;
use std::sync::Arc;

use async_trait::async_trait;

use paladin::application::ports::output::embedding_port::{EmbeddingError, EmbeddingPort, Embedding};
use paladin::application::ports::output::garrison_port::GarrisonEntry;
use paladin::application::ports::output::llm_port::{LlmError, LlmPort, LlmRequest, LlmResponse, FinishReason, TokenUsage};
use paladin::application::ports::output::sanctum_port::{
    SanctumEntry, SanctumError, SanctumPort, SanctumQuery, SanctumSearchResult,
};
use paladin::application::use_cases::sanctum::{
    ExtractedMemory, MemoryExtractionService, MemoryExtractionStrategy,
};
use paladin::core::platform::container::garrison::ConversationRole;
use paladin::core::platform::container::sanctum::{Memory, MemoryType, MemoryBuilder};
use serde_json::Value;

// Mock LLM Port
struct MockLlmPort {
    response: String,
    should_fail: bool,
}

#[async_trait]
impl LlmPort for MockLlmPort {
    async fn generate(
        &self,
        request: LlmRequest,
    ) -> Result<LlmResponse, LlmError> {
        if self.should_fail {
            Err(LlmError::ProcessingError("Mock LLM failure".to_string()))
        } else {
            Ok(LlmResponse {
                id: uuid::Uuid::new_v4(),
                request_id: request.id,
                model: "mock-model".to_string(),
                content: self.response.clone(),
                finish_reason: FinishReason::Stop,
                usage: TokenUsage {
                    prompt_tokens: 10,
                    completion_tokens: 20,
                    total_tokens: 30,
                },
                created_at: chrono::Utc::now(),
                metadata: HashMap::new(),
                function_call: None,
            })
        }
    }

    async fn generate_stream(
        &self,
        _request: LlmRequest,
    ) -> Result<tokio::sync::mpsc::Receiver<Result<String, LlmError>>, LlmError> {
        unimplemented!()
    }

    fn model_name(&self) -> &str {
        "mock-model"
    }

    fn validate_model(&self, _model: &str) -> Result<(), LlmError> {
        Ok(())
    }
}

// Mock Embedding Port
struct MockEmbeddingPort {
    dimension: usize,
    should_fail: bool,
}

#[async_trait]
impl EmbeddingPort for MockEmbeddingPort {
    async fn embed_text(&self, _text: &str) -> Result<Embedding, EmbeddingError> {
        if self.should_fail {
            Err(EmbeddingError::ProcessingError(
                "Mock embedding failure".to_string(),
            ))
        } else {
            Ok(Embedding {
                vector: vec![0.1; self.dimension],
                model: "mock-embedding-model".to_string(),
                dimension: self.dimension,
            })
        }
    }

    async fn embed_batch(&self, texts: &[String]) -> Result<Vec<Embedding>, EmbeddingError> {
        if self.should_fail {
            Err(EmbeddingError::ProcessingError(
                "Mock embedding failure".to_string(),
            ))
        } else {
            Ok(texts
                .iter()
                .map(|_| Embedding {
                    vector: vec![0.1; self.dimension],
                    model: "mock-embedding-model".to_string(),
                    dimension: self.dimension,
                })
                .collect())
        }
    }

    fn dimension(&self) -> usize {
        self.dimension
    }

    fn model_name(&self) -> &str {
        "mock-embedding-model"
    }
}

// Mock Sanctum Port
struct MockSanctumPort {
    stored_entries: std::sync::Mutex<Vec<SanctumEntry>>,
    should_fail_store: bool,
    duplicate_threshold: f32,
}

impl MockSanctumPort {
    fn new() -> Self {
        Self {
            stored_entries: std::sync::Mutex::new(Vec::new()),
            should_fail_store: false,
            duplicate_threshold: 0.95,
        }
    }

    fn with_duplicate_threshold(mut self, threshold: f32) -> Self {
        self.duplicate_threshold = threshold;
        self
    }
}

#[async_trait]
impl SanctumPort for MockSanctumPort {
    async fn store(&self, entry: SanctumEntry) -> Result<(), SanctumError> {
        if self.should_fail_store {
            Err(SanctumError::StorageError("Mock storage failure".to_string()))
        } else {
            self.stored_entries.lock().unwrap().push(entry);
            Ok(())
        }
    }

    async fn search(&self, query: SanctumQuery) -> Result<Vec<SanctumSearchResult>, SanctumError> {
        // Check if min_score is above duplicate threshold
        if query.min_score.unwrap_or(0.0) >= self.duplicate_threshold {
            // Return a mock duplicate result
            let mock_memory = MemoryBuilder::new(
                "test-paladin".to_string(),
                "Existing duplicate memory".to_string(),
            )
            .memory_type(MemoryType::Semantic)
            .build()
            .unwrap();

            let mock_entry = SanctumEntry::new(
                mock_memory,
                vec![0.1; 1536],
            )
            .unwrap();

            Ok(vec![SanctumSearchResult::new(mock_entry, 0.96)])
        } else {
            Ok(Vec::new())
        }
    }

    async fn delete(&self, _entry_id: &str) -> Result<(), SanctumError> {
        unimplemented!()
    }

    async fn update(&self, _entry: SanctumEntry) -> Result<(), SanctumError> {
        unimplemented!()
    }

    async fn count(&self, _paladin_id: Option<&str>) -> Result<usize, SanctumError> {
        Ok(self.stored_entries.lock().unwrap().len())
    }
}

#[tokio::test]
async fn test_successful_extraction_with_multiple_memory_types() {
    let llm_response = r#"[
        {
            "content": "User prefers dark mode in all applications",
            "memory_type": "Semantic",
            "importance": 0.8,
            "metadata": {"category": "ui"}
        },
        {
            "content": "User is learning Rust programming language",
            "memory_type": "Semantic",
            "importance": 0.9,
            "metadata": {"topic": "programming"}
        },
        {
            "content": "User wants to build a web application",
            "memory_type": "Episodic",
            "importance": 0.85,
            "metadata": {}
        }
    ]"#;

    let llm = Arc::new(MockLlmPort {
        response: llm_response.to_string(),
        should_fail: false,
    });
    let embedding = Arc::new(MockEmbeddingPort {
        dimension: 1536,
        should_fail: false,
    });
    let sanctum = Arc::new(MockSanctumPort::new());

    let service = MemoryExtractionService::new(llm, embedding, sanctum.clone());

    let conversation = vec![
        GarrisonEntry::new(
            ConversationRole::User,
            "I prefer dark mode".to_string(),
        ),
        GarrisonEntry::new(
            ConversationRole::Assistant,
            "Noted! I'll remember your preference".to_string(),
        ),
    ];

    let result = service
        .extract_memories("test-paladin", &conversation)
        .await
        .unwrap();

    assert_eq!(result.len(), 3);
    assert_eq!(sanctum.stored_entries.lock().unwrap().len(), 3);
}

#[tokio::test]
async fn test_importance_scoring_correctly_assigned() {
    let llm_response = r#"[
        {
            "content": "High importance memory",
            "memory_type": "Semantic",
            "importance": 0.95,
            "metadata": {}
        },
        {
            "content": "Low importance memory",
            "memory_type": "Episodic",
            "importance": 0.3,
            "metadata": {}
        }
    ]"#;

    let llm = Arc::new(MockLlmPort {
        response: llm_response.to_string(),
        should_fail: false,
    });
    let embedding = Arc::new(MockEmbeddingPort {
        dimension: 1536,
        should_fail: false,
    });
    let sanctum = Arc::new(MockSanctumPort::new());

    let service = MemoryExtractionService::new(llm, embedding, sanctum.clone());

    let conversation = vec![GarrisonEntry::new(
        ConversationRole::User,
        "Test content".to_string(),
    )];

    let result = service
        .extract_memories("test-paladin", &conversation)
        .await
        .unwrap();

    assert_eq!(result.len(), 2);
    assert_eq!(result[0].memory.importance, 0.95);
    assert_eq!(result[1].memory.importance, 0.3);
}

#[tokio::test]
async fn test_duplicate_detection_prevents_restorage() {
    let llm_response = r#"[
        {
            "content": "Duplicate memory content",
            "memory_type": "Semantic",
            "importance": 0.8,
            "metadata": {}
        }
    ]"#;

    let llm = Arc::new(MockLlmPort {
        response: llm_response.to_string(),
        should_fail: false,
    });
    let embedding = Arc::new(MockEmbeddingPort {
        dimension: 1536,
        should_fail: false,
    });
    // Set duplicate threshold to trigger duplicate detection
    let sanctum = Arc::new(MockSanctumPort::new().with_duplicate_threshold(0.95));

    let service = MemoryExtractionService::new(llm, embedding, sanctum.clone());

    let conversation = vec![GarrisonEntry::new(
        ConversationRole::User,
        "Duplicate content".to_string(),
    )];

    let result = service
        .extract_memories("test-paladin", &conversation)
        .await
        .unwrap();

    // Should be 0 because duplicate was detected
    assert_eq!(result.len(), 0);
    assert_eq!(sanctum.stored_entries.lock().unwrap().len(), 0);
}

#[tokio::test]
async fn test_llm_failure_handled_gracefully() {
    let llm = Arc::new(MockLlmPort {
        response: String::new(),
        should_fail: true,
    });
    let embedding = Arc::new(MockEmbeddingPort {
        dimension: 1536,
        should_fail: false,
    });
    let sanctum = Arc::new(MockSanctumPort::new());

    let service = MemoryExtractionService::new(llm, embedding, sanctum.clone());

    let conversation = vec![GarrisonEntry {
        role: "user".to_string(),
        content: "Test".to_string(),
        timestamp: chrono::Utc::now(),
        metadata: HashMap::new(),
        token_count: 1,
    }];

    // Should not fail, just return empty vec
    let result = service
        .extract_memories("test-paladin", &conversation)
        .await
        .unwrap();

    assert_eq!(result.len(), 0);
}

#[tokio::test]
async fn test_empty_conversation_returns_empty() {
    let llm = Arc::new(MockLlmPort {
        response: String::new(),
        should_fail: false,
    });
    let embedding = Arc::new(MockEmbeddingPort {
        dimension: 1536,
        should_fail: false,
    });
    let sanctum = Arc::new(MockSanctumPort::new());

    let service = MemoryExtractionService::new(llm, embedding, sanctum);

    let result = service
        .extract_memories("test-paladin", &[])
        .await
        .unwrap();

    assert_eq!(result.len(), 0);
}

#[tokio::test]
async fn test_malformed_json_response_handled() {
    let llm = Arc::new(MockLlmPort {
        response: "This is not valid JSON".to_string(),
        should_fail: false,
    });
    let embedding = Arc::new(MockEmbeddingPort {
        dimension: 1536,
        should_fail: false,
    });
    let sanctum = Arc::new(MockSanctumPort::new());

    let service = MemoryExtractionService::new(llm, embedding, sanctum);

    let conversation = vec![GarrisonEntry {
        role: "user".to_string(),
        content: "Test".to_string(),
        timestamp: chrono::Utc::now(),
        metadata: HashMap::new(),
        token_count: 1,
    }];

    // Should not fail, just return empty vec
    let result = service
        .extract_memories("test-paladin", &conversation)
        .await
        .unwrap();

    assert_eq!(result.len(), 0);
}

#[tokio::test]
async fn test_embedding_failure_skips_memory() {
    let llm_response = r#"[
        {
            "content": "Test memory",
            "memory_type": "Semantic",
            "importance": 0.8,
            "metadata": {}
        }
    ]"#;

    let llm = Arc::new(MockLlmPort {
        response: llm_response.to_string(),
        should_fail: false,
    });
    let embedding = Arc::new(MockEmbeddingPort {
        dimension: 1536,
        should_fail: true,
    });
    let sanctum = Arc::new(MockSanctumPort::new());

    let service = MemoryExtractionService::new(llm, embedding, sanctum.clone());

    let conversation = vec![GarrisonEntry {
        role: "user".to_string(),
        content: "Test".to_string(),
        timestamp: chrono::Utc::now(),
        metadata: HashMap::new(),
        token_count: 1,
    }];

    let result = service
        .extract_memories("test-paladin", &conversation)
        .await
        .unwrap();

    // Should be 0 because embedding failed
    assert_eq!(result.len(), 0);
}

#[test]
fn test_extraction_strategy_serialization() {
    let strategy = MemoryExtractionStrategy::Threshold { importance: 8 };
    let json = serde_json::to_string(&strategy).unwrap();
    let deserialized: MemoryExtractionStrategy = serde_json::from_str(&json).unwrap();
    assert_eq!(strategy, deserialized);
}
