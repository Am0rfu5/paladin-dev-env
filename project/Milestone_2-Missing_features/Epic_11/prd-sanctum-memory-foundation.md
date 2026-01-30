# Product Requirements Document: Sanctum Memory Foundation

## 1. Introduction/Overview

The Sanctum Memory Foundation establishes vector-based long-term memory infrastructure for Paladin AI agents. While the existing Garrison system handles short-term, conversational memory, Sanctum enables semantic search over historical knowledge, allowing Paladins to recall relevant past experiences and learned information across sessions.

**Problem:** Currently, Paladins rely on short-term context windows (Garrison) which don't persist meaningful insights across sessions or enable semantic retrieval of relevant past experiences. This limits their ability to build cumulative knowledge and provide contextually aware responses based on historical interactions.

**Solution:** Implement a vector embedding infrastructure that converts text into semantic vectors, stores them in a searchable vector database, and enables similarity-based retrieval of relevant memories.

## 2. Goals

1. **Standardized Embedding Interface:** Define port-based architecture for pluggable embedding providers
2. **Production-Ready Vector Storage:** Implement Qdrant adapter for scalable vector search operations
3. **Development Flexibility:** Provide in-memory vector store for testing and prototyping
4. **Semantic Search Foundation:** Enable < 500ms search performance on datasets up to 100K vectors
5. **Memory Type Classification:** Support episodic, semantic, and procedural memory categorization
6. **Configurable Memory Decay:** Allow per-Paladin strategies for memory importance over time
7. **Hexagonal Architecture Compliance:** Maintain clean separation between core domain, ports, and adapters

## 3. User Stories

### 3.1 Embedding Port Definition (US-11.1)
**As a** framework developer  
**I want** a standardized port for generating vector embeddings  
**So that** I can plug in different embedding providers (OpenAI, local models, etc.) without changing core logic

**Acceptance Criteria:**
- `EmbeddingPort` trait defined in `src/application/ports/output/embedding_port.rs`
- Trait includes async methods: `embed_text()`, `embed_batch()`, `dimension()`, `model_name()`
- `Embedding` struct contains vector, model metadata, and token count
- `EmbeddingError` enum covers: NetworkError, RateLimited, InvalidInput, ProviderError
- Unit tests for error handling and trait contract
- Documentation with usage examples

### 3.2 OpenAI Embedding Adapter (US-11.2)
**As a** developer  
**I want** to generate embeddings using OpenAI's API  
**So that** I can use industry-standard embedding models for production deployments

**Acceptance Criteria:**
- `OpenAIEmbeddingAdapter` implements `EmbeddingPort` in `src/infrastructure/adapters/llm/openai_embedding_adapter.rs`
- Supports `text-embedding-3-small` (1536 dimensions, default)
- Supports `text-embedding-3-large` (3072 dimensions)
- Supports `text-embedding-ada-002` (1536 dimensions, legacy)
- Configurable via `OpenAIEmbeddingConfig` struct (API key, model, base URL, retries, timeout)
- Implements exponential backoff retry logic for rate limits
- Batch processing respects API limits (max 2048 inputs per request)
- Feature flag: `openai-embeddings` (enabled by default)
- Integration test with mocked HTTP responses

### 3.3 Vector Store Port Definition (US-11.3)
**As a** framework developer  
**I want** a standardized port for vector storage operations  
**So that** I can plug in different vector databases (Qdrant, Pinecone, etc.) based on deployment needs

**Acceptance Criteria:**
- `SanctumPort` trait defined in `src/application/ports/output/sanctum_port.rs`
- Supports CRUD operations: `store()`, `store_batch()`, `search()`, `delete()`, `update()`, `count()`
- `SanctumQuery` struct with filtering, top-k, and min_score parameters
- `SanctumSearchResult` returns entries with similarity scores
- `SanctumFilter` supports metadata-based filtering (paladin_id, memory_type, date ranges)
- Thread-safe (`Send + Sync` bounds)
- Comprehensive error handling with `SanctumError` enum

### 3.4 Qdrant Vector Store Adapter (US-11.4 Enhanced)
**As a** DevOps engineer  
**I want** a production-ready Qdrant integration  
**So that** I can deploy Sanctum with scalable, high-performance vector search

**Acceptance Criteria:**
- `QdrantSanctumAdapter` implements `SanctumPort` in `src/infrastructure/adapters/sanctum/qdrant_adapter.rs`
- Uses official Qdrant Rust client
- Configurable connection (host, port, API key, gRPC vs HTTP)
- Collection auto-creation with configurable indexing parameters
- Supports metadata filtering via Qdrant's filter syntax
- Connection pooling and retry logic
- Feature flag: `qdrant` (optional dependency)
- Performance: < 500ms for top-10 searches on 100K vectors
- Integration tests with Docker Compose Qdrant container

### 3.5 In-Memory Vector Store (US-11.5)
**As a** developer  
**I want** an in-memory vector store for development and testing  
**So that** I can prototype Sanctum features without external database dependencies

**Acceptance Criteria:**
- `InMemorySanctum` implements `SanctumPort` in `src/infrastructure/adapters/sanctum/in_memory_adapter.rs`
- Uses brute-force cosine similarity for search (acceptable for < 10K vectors)
- Thread-safe with `Arc<RwLock<HashMap<String, SanctumEntry>>>`
- Configurable max capacity with LRU eviction strategy
- Supports all CRUD operations from `SanctumPort`
- Unit tests for all operations
- Performance: < 100ms for searches on 10K vectors
- No external dependencies (always available)

### 3.6 Sanctum Domain Model (US-11.6)
**As a** framework developer  
**I want** domain models for long-term memory concepts  
**So that** the system has clear bounded contexts following DDD principles

**Acceptance Criteria:**
- Core domain types in `src/core/platform/container/sanctum.rs`
- `Memory` struct: id, paladin_id, content, memory_type, importance, access_count, timestamps, metadata
- `MemoryType` enum: Episodic (conversations), Semantic (facts), Procedural (how-to)
- `MemoryDecayStrategy` enum: NoDecay, LinearDecay, AccessBasedDecay, CustomDecay
- `SanctumEntry` struct: memory + embedding + serialization
- Validation for embedding dimensions matching configured model
- Serde serialization for persistence
- Builder pattern for complex construction

## 4. Functional Requirements

### 4.1 Embedding Generation
1. The system **must** provide an `EmbeddingPort` trait for generating vector embeddings from text
2. The system **must** support async batch embedding generation for efficiency
3. The system **must** include OpenAI embedding adapter with retry logic for rate limits
4. The system **must** allow runtime configuration of embedding provider via config files
5. The system **must** use feature flags to enable/disable embedding provider dependencies
6. The system **must** validate embedding dimensions match the configured model

### 4.2 Vector Storage
7. The system **must** provide a `SanctumPort` trait for vector storage operations
8. The system **must** implement Qdrant adapter as the primary production vector database
9. The system **must** implement in-memory adapter for development and testing
10. The system **must** support storing vectors with associated metadata (paladin_id, memory_type, timestamps)
11. The system **must** support batch storage operations for efficiency
12. The system **must** support semantic search with configurable top-k results
13. The system **must** support filtering by metadata (paladin_id, memory_type, date ranges)
14. The system **must** return search results with similarity scores (0.0 - 1.0)

### 4.3 Memory Management
15. The system **must** classify memories as Episodic, Semantic, or Procedural
16. The system **must** assign importance scores (0.0 - 1.0) to memories
17. The system **must** track access count and last accessed timestamp for each memory
18. The system **must** support configurable memory decay strategies per Paladin
19. The system **must** implement LRU eviction for in-memory storage when capacity is reached
20. The system **must** allow deletion and updating of stored memories

### 4.4 Performance
21. The system **must** achieve < 500ms search latency for datasets up to 100K vectors (Qdrant)
22. The system **must** achieve < 100ms search latency for datasets up to 10K vectors (in-memory)
23. The system **must** support concurrent read/write operations safely (thread-safe)

### 4.5 Configuration
24. The system **must** allow configuration of embedding provider (model, API keys, base URL)
25. The system **must** allow configuration of vector database (Qdrant vs in-memory)
26. The system **must** support environment variable substitution for sensitive configuration
27. The system **must** validate configuration at startup and provide clear error messages

### 4.6 Integration with Existing Systems
28. The system **must** integrate with Garrison as complementary memory systems (Garrison = short-term, Sanctum = long-term)
29. The system **must** allow Paladins to optionally use Sanctum (backward compatible)
30. The system **must** use existing `Node<T>` pattern for domain entities
31. The system **must** follow hexagonal architecture (core → application → infrastructure)

## 5. Non-Goals (Out of Scope)

1. **Automatic Memory Migration:** Garrison memories will not auto-migrate to Sanctum (manual or separate feature)
2. **Multiple Vector Databases:** Only Qdrant and in-memory in this phase (Pinecone, Weaviate, Milvus later)
3. **Local Embedding Models:** Only cloud-based providers (OpenAI) in this phase (local models later)
4. **Memory Consolidation:** No automatic merging/deduplication of similar memories
5. **Memory Sharing:** No cross-Paladin memory access in this phase
6. **UI/Dashboard:** No admin interface for browsing memories
7. **Memory Graphs:** No relationship modeling between memories (future enhancement)
8. **Multilingual Embeddings:** English-only focus in this phase

## 6. Design Considerations

### 6.1 Architecture Diagram
```
┌─────────────────────────────────────────────────────────┐
│                    Core Domain                          │
│  ┌─────────────────────────────────────────────────┐   │
│  │ Memory, MemoryType, MemoryDecayStrategy         │   │
│  │ SanctumEntry (domain entity)                    │   │
│  └─────────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────────┘
                         ▲
                         │
┌─────────────────────────────────────────────────────────┐
│               Application Layer (Ports)                 │
│  ┌──────────────────┐       ┌──────────────────┐       │
│  │ EmbeddingPort    │       │  SanctumPort     │       │
│  │ - embed_text()   │       │  - store()       │       │
│  │ - embed_batch()  │       │  - search()      │       │
│  └──────────────────┘       └──────────────────┘       │
└─────────────────────────────────────────────────────────┘
                         ▲
                         │
┌─────────────────────────────────────────────────────────┐
│          Infrastructure Layer (Adapters)                │
│  ┌──────────────────┐       ┌──────────────────┐       │
│  │ OpenAI Embedding │       │  Qdrant Adapter  │       │
│  │   Adapter        │       │  In-Memory       │       │
│  └──────────────────┘       │    Adapter       │       │
│                              └──────────────────┘       │
└─────────────────────────────────────────────────────────┘
```

### 6.2 Data Flow
1. Paladin generates text during execution
2. Text is sent to `EmbeddingPort` implementation (e.g., OpenAI)
3. Vector embedding is returned
4. `SanctumEntry` is created with embedding + metadata
5. Entry is stored via `SanctumPort` implementation (e.g., Qdrant)
6. On retrieval: Query embedding is generated → Sanctum search → Results ranked by similarity

### 6.3 Configuration Structure
```yaml
sanctum:
  storage:
    type: "qdrant"  # or "in_memory"
    qdrant:
      host: "localhost"
      port: 6334
      api_key: "${QDRANT_API_KEY}"
      collection: "paladin_memories"
      use_grpc: true
    in_memory:
      max_entries: 10000
      eviction_strategy: "lru"
  
  embedding:
    provider: "openai"
    openai:
      api_key: "${OPENAI_API_KEY}"
      model: "text-embedding-3-small"
      base_url: "https://api.openai.com/v1"
      max_retries: 3
      timeout_seconds: 30
  
  memory:
    default_importance: 0.5
    decay_strategy: "access_based"  # Per-Paladin override supported
```

## 7. Technical Considerations

### 7.1 Dependencies
- **Qdrant Rust Client:** `qdrant-client` crate (optional via feature flag)
- **OpenAI Client:** Extend existing `reqwest`-based LLM adapter
- **Async Runtime:** Already using `tokio`
- **Serialization:** `serde`, `serde_json` (existing)

### 7.2 Feature Flags
```toml
[features]
default = ["openai-embeddings", "in-memory-sanctum"]
openai-embeddings = []
qdrant = ["qdrant-client"]
in-memory-sanctum = []
```

### 7.3 Error Handling
- Use `thiserror` for domain-specific errors (existing pattern)
- `EmbeddingError`: NetworkError, RateLimited, InvalidInput, ProviderError
- `SanctumError`: StorageError, SearchError, InvalidDimension, NotFound, ConfigError

### 7.4 Testing Strategy
- **Unit Tests:** Port traits, domain models, error handling
- **Integration Tests:** OpenAI embedding adapter (mocked), Qdrant adapter (Docker Compose), in-memory adapter
- **Performance Tests:** Benchmarks for search operations at 1K, 10K, 100K vectors
- **TDD Approach:** Write tests first per project methodology

### 7.5 Database Schema (Qdrant)
- **Collection Name:** `paladin_memories_{environment}`
- **Vector Dimension:** 1536 (configurable based on model)
- **Distance Metric:** Cosine similarity
- **Indexed Fields:** `paladin_id`, `memory_type`, `created_at`, `importance`

### 7.6 Migration from Garrison
Not implemented in this phase, but design considerations:
- Garrison remains unchanged (short-term memory)
- Future: Export Garrison entries → Generate embeddings → Store in Sanctum
- Paladins can use both systems simultaneously

## 8. Success Metrics

1. **API Completeness:** 100% of `EmbeddingPort` and `SanctumPort` methods implemented with tests
2. **Performance:** Search latency < 500ms for 100K vectors (Qdrant), < 100ms for 10K vectors (in-memory)
3. **Test Coverage:** ≥ 80% unit test coverage, ≥ 70% integration test coverage
4. **Documentation:** Complete rustdoc for all public APIs, `docs/SANCTUM.md` guide, working examples
5. **Code Quality:** Zero clippy warnings, passes `cargo fmt`, passes `cargo audit`
6. **Example Success:** `examples/sanctum_basics.rs` demonstrates full workflow (embed → store → search)
7. **Integration Success:** Example Paladin uses both Garrison and Sanctum in complementary roles

## 9. Open Questions

1. **Q:** Should we implement automatic memory importance decay in this phase, or just define the strategies?  
   **A:** Define strategies in domain model, implement in later Epic (keeps scope focused)

2. **Q:** How should we handle embedding model version changes (e.g., OpenAI updates dimensions)?  
   **A:** Store model name with each embedding, validate on retrieval, document migration process

3. **Q:** Should Sanctum have a maximum storage limit per Paladin?  
   **A:** Yes, configurable per deployment (e.g., max 100K memories per Paladin with LRU eviction)

4. **Q:** How should we handle embedding generation failures during Paladin execution?  
   **A:** Log error, continue execution without storing to Sanctum (graceful degradation)

5. **Q:** Should we support hybrid search (keyword + vector)?  
   **A:** Not in this phase, pure vector search only (add keyword later if needed)

## 10. Implementation Plan

### Phase 1: Foundation (Week 1)
- [ ] Define `EmbeddingPort` trait and `Embedding` struct (US-11.1)
- [ ] Implement OpenAI embedding adapter (US-11.2)
- [ ] Define `SanctumPort` trait and domain models (US-11.3, US-11.6)
- [ ] Write unit tests for all ports and domain models

### Phase 2: Storage (Week 2)
- [ ] Implement in-memory vector store (US-11.5)
- [ ] Implement Qdrant vector store adapter (US-11.4)
- [ ] Write integration tests with Docker Compose
- [ ] Write performance benchmarks
- [ ] Create documentation: `docs/SANCTUM.md`
- [ ] Create examples: `examples/sanctum_basics.rs`

### Phase 3: Integration & Polish
- [ ] Configuration loading for Sanctum settings
- [ ] Integration example with Paladin + Garrison + Sanctum
- [ ] Run full test suite (`make test-all`)
- [ ] Security scan (`make audit`, Snyk scan)
- [ ] Code quality (`make clean-code`)
- [ ] Documentation review

## 11. Acceptance Criteria for Epic Completion

- [ ] All 5 user stories from Epic 11 completed and tested
- [ ] `EmbeddingPort` trait implemented with OpenAI adapter
- [ ] `SanctumPort` trait implemented with Qdrant and in-memory adapters
- [ ] Domain models follow existing `Node<T>` pattern and hexagonal architecture
- [ ] ≥ 80% unit test coverage, ≥ 70% integration test coverage
- [ ] Documentation in `docs/SANCTUM.md` with architecture diagrams and usage examples
- [ ] Working example: `examples/sanctum_basics.rs` demonstrates embed → store → search workflow
- [ ] Performance benchmarks confirm < 500ms search on 100K vectors (Qdrant)
- [ ] Zero clippy warnings, formatted with `cargo fmt`, passes security audit
- [ ] Configurable via `config.yml` with environment variable support
- [ ] Feature flags allow optional Qdrant dependency
- [ ] Backward compatible (existing Paladin code works without Sanctum)

---

**Document Version:** 1.0  
**Last Updated:** January 29, 2026  
**Author:** GitHub Copilot  
**Reviewers:** Development Team  
**Status:** Ready for Implementation
