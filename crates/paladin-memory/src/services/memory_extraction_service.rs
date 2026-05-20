/// Memory Extraction Service
///
/// Extracts meaningful memories from conversation history and stores them
/// in long-term memory (Sanctum) for later retrieval via RAG.
///
/// This service:
/// - Analyzes conversation history using LLM
/// - Identifies important facts, preferences, and context
/// - Generates embeddings for semantic search
/// - Detects and prevents duplicate memories
/// - Stores memories with proper metadata
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;

use paladin_core::platform::container::garrison::GarrisonEntry;
use paladin_core::platform::container::prompt::{PromptItem, PromptRole, PromptType, TextPrompt};
use paladin_core::platform::container::sanctum::{MemoryBuilder, MemoryType, SanctumEntry};
use paladin_ports::output::embedding_port::EmbeddingPort;
use paladin_ports::output::llm_port::{LlmPort, LlmRequest};
use paladin_ports::output::sanctum_port::{SanctumError, SanctumFilter, SanctumPort, SanctumQuery};
use serde_json::Value;

/// Strategy for when to extract memories from conversations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum MemoryExtractionStrategy {
    /// Extract after every conversation turn.
    EveryTurn,

    /// Extract only when conversation completes (recommended).
    #[default]
    OnCompletion,

    /// Manual extraction only (user-triggered).
    Manual,

    /// Extract when importance threshold is exceeded.
    Threshold { importance: u8 },
}

/// Intermediate representation of extracted memory before storage.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtractedMemory {
    pub content: String,
    pub memory_type: MemoryType,
    pub importance: f32,
    pub metadata: HashMap<String, String>,
}

/// Memory Extraction Service.
///
/// Coordinates LLM-based memory extraction and storage via [`SanctumPort`].
/// Depends only on port traits — contains no concrete adapter references.
pub struct MemoryExtractionService {
    llm: Arc<dyn LlmPort>,
    embedding: Arc<dyn EmbeddingPort>,
    sanctum: Arc<dyn SanctumPort>,
}

impl MemoryExtractionService {
    /// Create a new memory extraction service.
    pub fn new(
        llm: Arc<dyn LlmPort>,
        embedding: Arc<dyn EmbeddingPort>,
        sanctum: Arc<dyn SanctumPort>,
    ) -> Self {
        Self {
            llm,
            embedding,
            sanctum,
        }
    }

    /// Extract memories from conversation history.
    ///
    /// Analyzes the conversation, extracts important information, and stores it
    /// in long-term memory (Sanctum) for later retrieval.
    pub async fn extract_memories(
        &self,
        paladin_id: &str,
        conversation: &[GarrisonEntry],
    ) -> Result<Vec<SanctumEntry>, SanctumError> {
        let start = std::time::Instant::now();

        if conversation.is_empty() {
            log::debug!("No conversation history to extract memories from");
            return Ok(Vec::new());
        }

        log::info!(
            "Extracting memories for paladin={}, turns={}",
            paladin_id,
            conversation.len()
        );

        // Build extraction prompt
        let prompt = self.build_extraction_prompt(conversation);

        // Call LLM to extract memories
        let prompt_item = PromptItem::new(PromptType::Text(TextPrompt {
            content: prompt,
            role: PromptRole::User,
        }))
        .map_err(|e| SanctumError::StorageError(format!("Failed to create prompt: {}", e)))?;

        let request = LlmRequest {
            id: uuid::Uuid::new_v4(),
            model: "gpt-4".to_string(),
            prompt: prompt_item,
            attachments: Vec::new(),
            stream: false,
            metadata: HashMap::new(),
        };

        let response = match self.llm.generate(request).await {
            Ok(resp) => resp.content,
            Err(e) => {
                log::warn!(
                    "Memory extraction LLM call failed: {}, continuing without extraction",
                    e
                );
                return Ok(Vec::new());
            }
        };

        // Parse LLM response to get extracted memories
        let extracted = match self.parse_extraction_response(&response) {
            Ok(memories) => memories,
            Err(e) => {
                log::warn!("Failed to parse extraction response: {}, continuing", e);
                return Ok(Vec::new());
            }
        };

        if extracted.is_empty() {
            log::debug!("No memories extracted from conversation");
            return Ok(Vec::new());
        }

        let extracted_count = extracted.len();
        log::debug!("Extracted {} potential memories", extracted_count);

        // Convert to Memory objects and generate embeddings
        let mut memories_to_store = Vec::new();
        for ext_mem in extracted {
            // Generate embedding
            let embedding = match self.embedding.embed_text(&ext_mem.content).await {
                Ok(emb) => emb,
                Err(e) => {
                    log::warn!("Failed to generate embedding for memory: {}, skipping", e);
                    continue;
                }
            };

            // Check for duplicates
            if self
                .check_for_duplicates(paladin_id, &embedding.vector)
                .await?
            {
                log::debug!(
                    "Duplicate memory detected, skipping: {:?}",
                    &ext_mem.content[..50.min(ext_mem.content.len())]
                );
                continue;
            }

            // Convert HashMap<String, String> to HashMap<String, Value>
            let metadata_values: HashMap<String, Value> = ext_mem
                .metadata
                .into_iter()
                .map(|(k, v)| (k, Value::String(v)))
                .collect();

            // Create Memory object
            let memory = MemoryBuilder::new(paladin_id.to_string(), ext_mem.content)
                .memory_type(ext_mem.memory_type)
                .importance(ext_mem.importance)
                .metadata(metadata_values)
                .build()
                .map_err(|e| SanctumError::StorageError(e.to_string()))?;

            // Create SanctumEntry
            let entry = SanctumEntry::new(memory, embedding.vector)
                .map_err(|e| SanctumError::StorageError(e.to_string()))?;

            memories_to_store.push(entry);
        }

        // Store memories in Sanctum
        let stored = self.store_memories(paladin_id, &memories_to_store).await?;

        let duration = start.elapsed();
        let avg_importance = if !stored.is_empty() {
            stored.iter().map(|e| e.memory.importance).sum::<f32>() / stored.len() as f32
        } else {
            0.0
        };

        log::info!(
            "Memory extraction complete: paladin={}, extracted={}, stored={}, avg_importance={:.2}, duration_ms={}",
            paladin_id,
            extracted_count,
            stored.len(),
            avg_importance,
            duration.as_millis()
        );

        Ok(stored)
    }

    /// Build extraction prompt from conversation history.
    fn build_extraction_prompt(&self, conversation: &[GarrisonEntry]) -> String {
        let mut prompt = String::from(EXTRACTION_PROMPT);
        prompt.push_str("\n\nConversation:\n");

        for entry in conversation {
            prompt.push_str(&format!("{:?}: {}\n", entry.role, entry.content));
        }

        prompt.push_str("\n\nExtract important memories as JSON array:");
        prompt
    }

    /// Parse LLM extraction response into structured memories.
    fn parse_extraction_response(&self, response: &str) -> Result<Vec<ExtractedMemory>, String> {
        // Try to extract JSON from response (might be wrapped in markdown)
        let json_str = if let Some(start) = response.find('[') {
            if let Some(end) = response.rfind(']') {
                &response[start..=end]
            } else {
                response
            }
        } else {
            response
        };

        serde_json::from_str::<Vec<ExtractedMemory>>(json_str)
            .map_err(|e| format!("Failed to parse JSON: {}", e))
    }

    /// Check if a similar memory already exists (>0.95 similarity).
    async fn check_for_duplicates(
        &self,
        paladin_id: &str,
        embedding: &[f32],
    ) -> Result<bool, SanctumError> {
        let query = SanctumQuery::new(embedding.to_vec(), 1)
            .with_filter(SanctumFilter::new().paladin_id(paladin_id.to_string()))
            .with_min_score(0.95);

        let results = self.sanctum.search(query).await?;
        Ok(!results.is_empty())
    }

    /// Store memories in batch via Sanctum.
    async fn store_memories(
        &self,
        _paladin_id: &str,
        memories: &[SanctumEntry],
    ) -> Result<Vec<SanctumEntry>, SanctumError> {
        let mut stored: Vec<SanctumEntry> = Vec::new();

        for entry in memories {
            match self.sanctum.store(entry.clone()).await {
                Ok(_) => {
                    stored.push(entry.clone());
                }
                Err(e) => {
                    log::warn!("Failed to store memory: {}, continuing", e);
                }
            }
        }

        Ok(stored)
    }
}

/// LLM prompt template for memory extraction.
const EXTRACTION_PROMPT: &str = r#"You are a memory extraction assistant. Analyze the following conversation and extract important memories.

For each memory, provide:
1. content: The actual information to remember (be specific and complete)
2. memory_type: One of "Episodic", "Semantic", "Procedural"
3. importance: 0.0 to 1.0 score indicating how important this information is
4. metadata: Optional key-value pairs for additional context (as an object)

Memory Types:
- Episodic: Specific events, conversations, or experiences
- Semantic: Facts, knowledge, preferences, and general information  
- Procedural: How-to instructions, procedures, and workflows

Rules:
- Extract only genuinely important information worth remembering long-term
- Be specific and include relevant details
- Avoid extracting trivial or transient information
- Combine related information into single memories when appropriate
- Use proper memory types

Return ONLY a JSON array of memories, no additional text."#;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extraction_strategy_default() {
        assert_eq!(
            MemoryExtractionStrategy::default(),
            MemoryExtractionStrategy::OnCompletion
        );
    }

    #[test]
    fn test_extraction_strategy_equality() {
        assert_eq!(
            MemoryExtractionStrategy::EveryTurn,
            MemoryExtractionStrategy::EveryTurn
        );
        assert_ne!(
            MemoryExtractionStrategy::EveryTurn,
            MemoryExtractionStrategy::OnCompletion
        );
    }

    #[test]
    fn test_extraction_strategy_threshold() {
        let strategy = MemoryExtractionStrategy::Threshold { importance: 7 };
        if let MemoryExtractionStrategy::Threshold { importance } = strategy {
            assert_eq!(importance, 7);
        } else {
            panic!("Expected Threshold variant");
        }
    }

    #[test]
    fn test_extracted_memory_serialization() {
        let mut metadata = HashMap::new();
        metadata.insert("source".to_string(), "conversation".to_string());

        let memory = ExtractedMemory {
            content: "User prefers dark mode".to_string(),
            memory_type: MemoryType::Semantic,
            importance: 0.8,
            metadata,
        };

        let json = serde_json::to_string(&memory).unwrap();
        let deserialized: ExtractedMemory = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.content, memory.content);
        assert_eq!(deserialized.importance, memory.importance);
    }
}
