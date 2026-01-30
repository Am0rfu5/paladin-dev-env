## Epic 11: Sanctum Memory Foundation

**Theme:** Vector Embedding Infrastructure  
**Duration:** 2 weeks  
**Priority:** Critical  
**Dependencies:** None  

### Description
Establish the foundational infrastructure for vector-based long-term memory, including embedding generation ports, vector storage traits, and the core data structures needed for semantic search.

### User Stories

#### US-11.1: Embedding Port Definition
**As a** framework developer  
**I want** a standardized port for generating vector embeddings  
**So that** I can plug in different embedding providers

**Acceptance Criteria:**
- [ ] `EmbeddingPort` trait defined in `src/application/ports/output/embedding_port.rs`
- [ ] Trait includes `embed_text(&str) -> Result<Vec<f32>, EmbeddingError>`
- [ ] Trait includes `embed_batch(&[&str]) -> Result<Vec<Vec<f32>>, EmbeddingError>`
- [ ] Trait includes `dimension() -> usize` for vector dimension
- [ ] `EmbeddingError` enum covers: NetworkError, RateLimited, InvalidInput, ProviderError
- [ ] Unit tests for error handling

**Definition of Done:**
```rust
#[async_trait]
pub trait EmbeddingPort: Send + Sync {
    async fn embed_text(&self, text: &str) -> Result<Embedding, EmbeddingError>;
    async fn embed_batch(&self, texts: &[&str]) -> Result<Vec<Embedding>, EmbeddingError>;
    fn dimension(&self) -> usize;
    fn model_name(&self) -> &str;
}

pub struct Embedding {
    pub vector: Vec<f32>,
    pub model: String,
    pub dimension: usize,
    pub token_count: Option<u32>,
}
```

---

#### US-11.2: OpenAI Embedding Adapter
**As a** developer  
**I want** to generate embeddings using OpenAI's API  
**So that** I can use industry-standard embedding models

**Acceptance Criteria:**
- [ ] `OpenAIEmbeddingAdapter` implements `EmbeddingPort`
- [ ] Supports `text-embedding-3-small` (1536 dimensions)
- [ ] Supports `text-embedding-3-large` (3072 dimensions)
- [ ] Supports `text-embedding-ada-002` (1536 dimensions, legacy)
- [ ] Configurable via `OpenAIEmbeddingConfig` struct
- [ ] Implements retry with exponential backoff
- [ ] Batch processing respects API limits (max 2048 inputs)
- [ ] Integration test with mocked responses

**Definition of Done:**
```rust
pub struct OpenAIEmbeddingAdapter {
    client: reqwest::Client,
    config: OpenAIEmbeddingConfig,
}

pub struct OpenAIEmbeddingConfig {
    pub api_key: String,
    pub model: String,  // "text-embedding-3-small" default
    pub base_url: String,
    pub max_retries: u32,
    pub timeout_seconds: u64,
}
```

---

#### US-11.3: Vector Store Port Definition
**As a** framework developer  
**I want** a standardized port for vector storage operations  
**So that** I can plug in different vector databases

**Acceptance Criteria:**
- [ ] `SanctumPort` trait defined in `src/application/ports/output/sanctum_port.rs`
- [ ] Supports `store(id, vector, metadata)` operation
- [ ] Supports `search(vector, top_k, filter)` operation
- [ ] Supports `delete(id)` operation
- [ ] Supports `update(id, vector, metadata)` operation
- [ ] Filter supports metadata-based filtering
- [ ] Returns `SanctumSearchResult` with score and metadata

**Definition of Done:**
```rust
#[async_trait]
pub trait SanctumPort: Send + Sync {
    async fn store(&self, entry: SanctumEntry) -> Result<(), SanctumError>;
    async fn store_batch(&self, entries: Vec<SanctumEntry>) -> Result<(), SanctumError>;
    async fn search(&self, query: SanctumQuery) -> Result<Vec<SanctumSearchResult>, SanctumError>;
    async fn delete(&self, id: &str) -> Result<bool, SanctumError>;
    async fn update(&self, entry: SanctumEntry) -> Result<(), SanctumError>;
    async fn count(&self) -> Result<u64, SanctumError>;
}

pub struct SanctumEntry {
    pub id: String,
    pub paladin_id: String,
    pub content: String,
    pub embedding: Vec<f32>,
    pub metadata: HashMap<String, Value>,
    pub timestamp: DateTime<Utc>,
}

pub struct SanctumQuery {
    pub embedding: Vec<f32>,
    pub top_k: usize,
    pub filter: Option<SanctumFilter>,
    pub min_score: Option<f32>,
}

pub struct SanctumSearchResult {
    pub entry: SanctumEntry,
    pub score: f32,
}
```

---

#### US-11.4: In-Memory Vector Store
**As a** developer  
**I want** an in-memory vector store for development and testing  
**So that** I can prototype without external dependencies

**Acceptance Criteria:**
- [ ] `InMemorySanctum` implements `SanctumPort`
- [ ] Uses brute-force cosine similarity for search
- [ ] Thread-safe with `RwLock`
- [ ] Supports all CRUD operations
- [ ] Configurable max capacity with LRU eviction
- [ ] Unit tests for all operations
- [ ] Performance acceptable for < 10,000 vectors

**Definition of Done:**
```rust
pub struct InMemorySanctum {
    entries: Arc<RwLock<HashMap<String, SanctumEntry>>>,
    config: InMemorySanctumConfig,
}

pub struct InMemorySanctumConfig {
    pub max_entries: usize,
    pub eviction_strategy: EvictionStrategy,
}
```

---

#### US-11.5: Sanctum Domain Model
**As a** framework developer  
**I want** domain models for long-term memory concepts  
**So that** the system has clear bounded contexts

**Acceptance Criteria:**
- [ ] `SanctumEntry` struct in `src/core/platform/container/sanctum.rs`
- [ ] `Memory` value object representing a stored insight
- [ ] `MemoryType` enum: Episodic, Semantic, Procedural
- [ ] Serialization/deserialization support
- [ ] Validation for embedding dimensions

**Definition of Done:**
```rust
pub struct Memory {
    pub id: Uuid,
    pub paladin_id: String,
    pub content: String,
    pub memory_type: MemoryType,
    pub importance: f32,  // 0.0 - 1.0
    pub access_count: u32,
    pub last_accessed: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub metadata: HashMap<String, Value>,
}

pub enum MemoryType {
    Episodic,    // Specific events/conversations
    Semantic,    // Facts and knowledge
    Procedural,  // How-to instructions
}
```

---

### Epic 11 Completion Criteria
- [ ] All 5 user stories completed and tested
- [ ] `EmbeddingPort` trait with OpenAI implementation
- [ ] `SanctumPort` trait with in-memory implementation
- [ ] Domain models for memory concepts
- [ ] Documentation in `docs/SANCTUM.md`
- [ ] Example: `examples/sanctum_basics.rs`

---