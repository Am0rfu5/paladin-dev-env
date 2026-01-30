## Epic 12: Sanctum RAG Integration

**Theme:** Retrieval-Augmented Generation  
**Duration:** 2 weeks  
**Priority:** Critical  
**Dependencies:** Epic 11  

### Description
Integrate the Sanctum memory system with Paladin agents to enable RAG (Retrieval-Augmented Generation) capabilities, including automatic memory storage, retrieval during execution, and memory consolidation.

### User Stories

#### US-12.1: Qdrant Vector Store Adapter
**As a** developer  
**I want** to use Qdrant as a production vector database  
**So that** I can scale to millions of memories

**Acceptance Criteria:**
- [ ] `QdrantSanctum` implements `SanctumPort`
- [ ] Connects via `qdrant-client` crate
- [ ] Supports collection creation with configurable settings
- [ ] Supports payload filtering on metadata
- [ ] Configurable connection (local, cloud, API key)
- [ ] Health check implementation
- [ ] Integration test with Qdrant container

**Definition of Done:**
```rust
pub struct QdrantSanctum {
    client: QdrantClient,
    config: QdrantSanctumConfig,
}

pub struct QdrantSanctumConfig {
    pub url: String,
    pub api_key: Option<String>,
    pub collection_name: String,
    pub vector_size: u64,
    pub distance: Distance,  // Cosine, Euclidean, Dot
    pub on_disk: bool,
}
```

---

#### US-12.2: Paladin Long-Term Memory Integration
**As a** developer  
**I want** to configure a Paladin with long-term memory  
**So that** it can remember across sessions

**Acceptance Criteria:**
- [ ] `PaladinBuilder::with_sanctum(sanctum)` method
- [ ] `PaladinBuilder::with_embedding_port(port)` method
- [ ] Paladin stores conversation insights automatically
- [ ] Memory extraction configurable (every turn, on completion, manual)
- [ ] Memory importance scoring based on content analysis

**Definition of Done:**
```rust
impl PaladinBuilder {
    pub fn with_sanctum(mut self, sanctum: Arc<dyn SanctumPort>) -> Self;
    pub fn with_embedding_port(mut self, embedding: Arc<dyn EmbeddingPort>) -> Self;
    pub fn memory_extraction_strategy(mut self, strategy: MemoryExtractionStrategy) -> Self;
}

pub enum MemoryExtractionStrategy {
    EveryTurn,
    OnCompletion,
    Manual,
    Threshold { importance: f32 },
}
```

---

#### US-12.3: RAG Retrieval Service
**As a** developer  
**I want** automatic context retrieval from long-term memory  
**So that** Paladins have relevant historical context

**Acceptance Criteria:**
- [ ] `RagRetrievalService` in `src/application/use_cases/sanctum/`
- [ ] Retrieves top-k relevant memories before LLM call
- [ ] Configurable retrieval trigger (always, keyword-based, semantic threshold)
- [ ] Formats retrieved memories for prompt injection
- [ ] Deduplication of similar memories
- [ ] Respects token budget for context

**Definition of Done:**
```rust
pub struct RagRetrievalService {
    sanctum: Arc<dyn SanctumPort>,
    embedding: Arc<dyn EmbeddingPort>,
    config: RagConfig,
}

pub struct RagConfig {
    pub top_k: usize,
    pub min_similarity: f32,
    pub max_tokens: usize,
    pub retrieval_trigger: RetrievalTrigger,
}

impl RagRetrievalService {
    pub async fn retrieve_context(
        &self,
        paladin_id: &str,
        query: &str,
    ) -> Result<Vec<Memory>, SanctumError>;
    
    pub fn format_for_prompt(&self, memories: &[Memory]) -> String;
}
```

---

#### US-12.4: Memory Extraction Service
**As a** developer  
**I want** automatic extraction of memorable information from conversations  
**So that** important insights are preserved

**Acceptance Criteria:**
- [ ] `MemoryExtractionService` in `src/application/use_cases/sanctum/`
- [ ] Uses LLM to identify memorable content
- [ ] Extracts: facts, preferences, events, instructions
- [ ] Assigns importance scores
- [ ] Categorizes by memory type
- [ ] Deduplicates against existing memories

**Definition of Done:**
```rust
pub struct MemoryExtractionService {
    llm: Arc<dyn LlmPort>,
    embedding: Arc<dyn EmbeddingPort>,
    sanctum: Arc<dyn SanctumPort>,
}

impl MemoryExtractionService {
    pub async fn extract_memories(
        &self,
        paladin_id: &str,
        conversation: &[GarrisonEntry],
    ) -> Result<Vec<Memory>, SanctumError>;
    
    pub async fn store_memories(
        &self,
        memories: Vec<Memory>,
    ) -> Result<(), SanctumError>;
}
```

---

#### US-12.5: PaladinExecutionService RAG Integration
**As a** developer  
**I want** RAG integrated into the execution flow  
**So that** memory retrieval is automatic

**Acceptance Criteria:**
- [ ] `PaladinExecutionService` queries Sanctum before LLM call
- [ ] Retrieved context injected into system prompt or user message
- [ ] New memories extracted after successful execution
- [ ] Configurable via `PaladinConfig`
- [ ] Metrics for retrieval latency and hit rate

**Definition of Done:**
```rust
// Updated execution flow
impl PaladinExecutionService {
    pub async fn execute(&self, paladin: &Paladin, input: &str) -> Result<PaladinResult, PaladinError> {
        // 1. Retrieve relevant memories (if sanctum configured)
        let memories = self.retrieve_memories(paladin, input).await?;
        
        // 2. Build prompt with memory context
        let enriched_prompt = self.build_prompt_with_context(paladin, input, &memories);
        
        // 3. Execute LLM call
        let result = self.execute_llm(paladin, &enriched_prompt).await?;
        
        // 4. Extract and store new memories (if configured)
        self.extract_and_store_memories(paladin, input, &result).await?;
        
        Ok(result)
    }
}
```

---

### Epic 12 Completion Criteria
- [ ] All 5 user stories completed and tested
- [ ] Qdrant adapter with integration tests
- [ ] RAG retrieval integrated into execution flow
- [ ] Memory extraction service functional
- [ ] Configuration via YAML supported
- [ ] Example: `examples/paladin_with_rag.rs`
- [ ] Example: `examples/cli_configs/paladin_rag.yaml`

---
