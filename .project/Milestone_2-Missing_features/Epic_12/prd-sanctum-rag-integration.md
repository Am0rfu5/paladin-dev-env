# Product Requirements Document: Sanctum RAG Integration

**Epic:** Epic 12  
**Feature:** Retrieval-Augmented Generation (RAG) with Sanctum Memory System  
**Version:** 1.0  
**Date:** January 30, 2026  
**Status:** Draft  
**Estimated Duration:** 2 weeks  

---

## 1. Introduction/Overview

This PRD defines the requirements for integrating the Sanctum long-term memory system with Paladin agents to enable Retrieval-Augmented Generation (RAG) capabilities. RAG allows AI agents to retrieve and incorporate relevant historical context from a vector database before generating responses, significantly improving response quality, consistency, and context awareness across sessions.

### Problem Statement

Currently, Paladin agents only have access to short-term memory (Garrison) which is limited to the current conversation session. This means:
- Agents cannot remember information across different sessions
- Important insights from previous interactions are lost
- Agents must re-learn user preferences and facts repeatedly
- Context from related past conversations is not available

### Solution

Integrate Sanctum (the vector-based long-term memory system built in Epic 11) with the Paladin execution flow to:
- Automatically store important conversation insights as vector embeddings
- Retrieve relevant historical context before LLM calls
- Enrich prompts with retrieved memories
- Provide configuration options for memory extraction and retrieval strategies

---

## 2. Goals

1. **Enable Long-Term Memory**: Paladins can access memories from previous sessions via Sanctum
2. **Automatic Memory Management**: Extract and store important information without manual intervention
3. **Context Enrichment**: Automatically retrieve and inject relevant memories into prompts
4. **Production-Ready Vector Storage**: Support Qdrant as the primary production vector database
5. **Developer-Friendly Testing**: Provide in-memory vector store for local development and testing
6. **Graceful Degradation**: Continue execution even if memory retrieval fails
7. **Configurable Behavior**: Support both global defaults and per-Paladin overrides
8. **Performance Optimization**: Rank memories by relevance and respect token limits

---

## 3. User Stories

### US-12.1: Qdrant Vector Store Adapter
**As a** developer  
**I want** to use Qdrant as a production vector database  
**So that** I can scale to millions of memories with production-grade performance

**Acceptance Criteria:**
- `QdrantSanctum` implements `SanctumPort` trait from Epic 11
- Connects via `qdrant-client` Rust crate
- Supports collection creation with configurable vector dimensions and distance metrics
- Supports payload filtering on metadata fields
- Configurable connection (local, cloud, API key authentication)
- Implements health check method to verify connectivity
- Integration test using Qdrant Docker container

### US-12.2: Paladin Long-Term Memory Integration
**As a** developer  
**I want** to configure a Paladin with long-term memory capabilities  
**So that** my agent can remember information across sessions

**Acceptance Criteria:**
- `PaladinBuilder::with_sanctum(sanctum: Arc<dyn SanctumPort>)` method added
- `PaladinBuilder::with_embedding_port(embedding: Arc<dyn EmbeddingPort>)` method added
- `PaladinBuilder::memory_extraction_strategy(strategy: MemoryExtractionStrategy)` method added
- Default strategy is `OnCompletion` (extract memories when task completes)
- Memory extraction runs automatically based on configured strategy
- Configuration validates that embedding port is provided if sanctum is configured

### US-12.3: RAG Retrieval Service
**As a** developer  
**I want** automatic context retrieval from long-term memory before LLM calls  
**So that** my Paladins have relevant historical context

**Acceptance Criteria:**
- `RagRetrievalService` created in `src/application/use_cases/sanctum/rag_retrieval_service.rs`
- Retrieves top-k relevant memories based on semantic similarity to current input
- Configurable retrieval parameters: `top_k`, `min_similarity`, `max_tokens`
- Formats retrieved memories into structured text for prompt injection
- Implements deduplication to remove near-identical memories
- Ranks memories by relevance score and truncates to fit token budget
- Asynchronous execution with timeout handling
- Continues execution (returns empty context) if retrieval fails or times out

### US-12.4: Memory Extraction Service
**As a** developer  
**I want** automatic extraction of memorable information from conversations  
**So that** important insights are preserved for future retrieval

**Acceptance Criteria:**
- `MemoryExtractionService` created in `src/application/use_cases/sanctum/memory_extraction_service.rs`
- Uses LLM to analyze conversation and identify memorable content
- Extracts structured information: facts, preferences, events, instructions
- Assigns importance scores (0.0-1.0) based on content analysis
- Categorizes by memory type (Episodic, Semantic, Procedural)
- Implements deduplication check against existing memories (no merging)
- Stores extracted memories via `SanctumPort`
- Extraction triggered `OnCompletion` (default) - when Paladin task finishes

### US-12.5: PaladinExecutionService RAG Integration
**As a** developer  
**I want** RAG integrated seamlessly into the Paladin execution flow  
**So that** memory retrieval happens automatically without manual intervention

**Acceptance Criteria:**
- `PaladinExecutionService::execute()` queries Sanctum before LLM call if configured
- Retrieved memories injected into system prompt as "Relevant Context" section
- Retrieval runs asynchronously with fallback to empty context on failure/timeout
- Memory extraction runs after successful execution if strategy is `OnCompletion`
- Configurable via `PaladinConfig` with sensible defaults
- Metrics collected: retrieval latency, hit rate (memories found vs queries), extraction count
- Global defaults in `config.yml`, per-Paladin overrides via builder methods

---

## 4. Functional Requirements

### FR-1: Qdrant Adapter Implementation
1. **FR-1.1**: Implement `QdrantSanctum` struct in `src/infrastructure/adapters/sanctum/qdrant_sanctum.rs`
2. **FR-1.2**: Support connection configuration: URL, API key (optional), collection name
3. **FR-1.3**: Implement `SanctumPort::store()` to upsert vectors with metadata
4. **FR-1.4**: Implement `SanctumPort::search()` with cosine similarity search
5. **FR-1.5**: Implement `SanctumPort::delete()` to remove entries by ID
6. **FR-1.6**: Implement `SanctumPort::update()` to modify existing entries
7. **FR-1.7**: Implement `SanctumPort::count()` to return total entries
8. **FR-1.8**: Support payload filtering using Qdrant's filter syntax
9. **FR-1.9**: Implement health check method to verify collection exists and is accessible
10. **FR-1.10**: Handle connection errors with appropriate error types from `SanctumError` enum

### FR-2: In-Memory Adapter (Testing Support)
11. **FR-2.1**: Ensure `InMemorySanctum` from Epic 11 fully implements `SanctumPort`
12. **FR-2.2**: Verify thread-safety for concurrent access
13. **FR-2.3**: Support all CRUD operations with in-memory HashMap storage
14. **FR-2.4**: Use brute-force cosine similarity for search (acceptable for < 10k vectors)

### FR-3: Paladin Builder Extensions
15. **FR-3.1**: Add `with_sanctum(sanctum: Arc<dyn SanctumPort>)` to `PaladinBuilder`
16. **FR-3.2**: Add `with_embedding_port(embedding: Arc<dyn EmbeddingPort>)` to `PaladinBuilder`
17. **FR-3.3**: Add `memory_extraction_strategy(strategy: MemoryExtractionStrategy)` to `PaladinBuilder`
18. **FR-3.4**: Validate that if `sanctum` is provided, `embedding_port` must also be provided
19. **FR-3.5**: Store RAG configuration in `PaladinConfig` or `PaladinData`

### FR-4: Memory Extraction Strategy
20. **FR-4.1**: Define `MemoryExtractionStrategy` enum with variants: `EveryTurn`, `OnCompletion`, `Manual`, `Threshold { importance: f32 }`
21. **FR-4.2**: Default strategy is `OnCompletion`
22. **FR-4.3**: `OnCompletion` triggers extraction when `Paladin::run()` completes successfully
23. **FR-4.4**: `Manual` requires explicit `extract_memories()` call
24. **FR-4.5**: `Threshold` extracts only if importance score exceeds specified value

### FR-5: RAG Retrieval Service
25. **FR-5.1**: Create `RagRetrievalService` in `src/application/use_cases/sanctum/rag_retrieval_service.rs`
26. **FR-5.2**: Implement `retrieve_context(paladin_id: &str, query: &str) -> Result<Vec<Memory>, SanctumError>`
27. **FR-5.3**: Generate query embedding using configured `EmbeddingPort`
28. **FR-5.4**: Call `SanctumPort::search()` with query embedding and `top_k` parameter
29. **FR-5.5**: Filter results by `min_similarity` threshold (default 0.7)
30. **FR-5.6**: Implement deduplication: remove memories with > 0.95 similarity to each other
31. **FR-5.7**: Rank memories by relevance score (descending)
32. **FR-5.8**: Truncate to fit `max_tokens` budget by removing lowest-scoring memories
33. **FR-5.9**: Implement `format_for_prompt(memories: &[Memory]) -> String` to structure memories
34. **FR-5.10**: Run retrieval asynchronously with 5-second timeout
35. **FR-5.11**: Return empty Vec on failure/timeout (graceful degradation)

### FR-6: RagConfig Structure
36. **FR-6.1**: Define `RagConfig` struct with fields: `top_k`, `min_similarity`, `max_tokens`, `retrieval_trigger`
37. **FR-6.2**: Default values: `top_k: 5`, `min_similarity: 0.7`, `max_tokens: 2000`
38. **FR-6.3**: `retrieval_trigger` enum: `Always`, `KeywordBased`, `SemanticThreshold`
39. **FR-6.4**: Default trigger is `Always` (retrieve for every execution)
40. **FR-6.5**: Support configuration via YAML and builder methods

### FR-7: Memory Extraction Service
41. **FR-7.1**: Create `MemoryExtractionService` in `src/application/use_cases/sanctum/memory_extraction_service.rs`
42. **FR-7.2**: Implement `extract_memories(paladin_id: &str, conversation: &[GarrisonEntry]) -> Result<Vec<Memory>, SanctumError>`
43. **FR-7.3**: Build extraction prompt that asks LLM to identify memorable content
44. **FR-7.4**: LLM response parsed into structured `Memory` objects
45. **FR-7.5**: Each memory includes: content, type (Episodic/Semantic/Procedural), importance (0.0-1.0)
46. **FR-7.6**: Generate embeddings for extracted memories using `EmbeddingPort`
47. **FR-7.7**: Check for duplicates using semantic similarity (> 0.95 = duplicate)
48. **FR-7.8**: Store new memories via `SanctumPort::store()`
49. **FR-7.9**: Log extraction metrics: count, average importance, duration

### FR-8: PaladinExecutionService Integration
50. **FR-8.1**: Modify `PaladinExecutionService::execute()` to check for Sanctum configuration
51. **FR-8.2**: If Sanctum configured, call `RagRetrievalService::retrieve_context()` before LLM call
52. **FR-8.3**: If retrieval succeeds, inject formatted memories into system prompt under "## Relevant Context"
53. **FR-8.4**: If retrieval fails/timeouts, log warning and continue with empty context
54. **FR-8.5**: After successful execution, check memory extraction strategy
55. **FR-8.6**: If strategy is `OnCompletion`, call `MemoryExtractionService::extract_memories()`
56. **FR-8.7**: Memory extraction runs asynchronously (doesn't block response)
57. **FR-8.8**: Collect metrics: retrieval_latency_ms, memories_retrieved_count, extraction_triggered_bool

### FR-9: Configuration Support
58. **FR-9.1**: Add `sanctum` section to `config.yml` with Qdrant and in-memory options
59. **FR-9.2**: Add `rag` section with default `RagConfig` values
60. **FR-9.3**: Add `memory_extraction` section with default strategy
61. **FR-9.4**: Support per-Paladin overrides via builder methods
62. **FR-9.5**: Validate configuration on startup: fail fast if invalid

### FR-10: Error Handling
63. **FR-10.1**: `SanctumError` enum includes: `ConnectionError`, `QueryError`, `StorageError`, `EmbeddingError`
64. **FR-10.2**: All errors are logged with appropriate severity
65. **FR-10.3**: Retrieval failures are non-fatal (graceful degradation)
66. **FR-10.4**: Extraction failures are logged but don't affect Paladin response

---

## 5. Non-Goals (Out of Scope)

The following are explicitly **NOT** included in Epic 12:

1. **Memory Consolidation/Merging**: No automatic merging of similar memories (deferred to future epic)
2. **Multi-Tenant Memory Isolation**: No user-level or organization-level memory segregation
3. **Memory Expiration/TTL**: No automatic deletion of old memories
4. **Advanced Filtering**: No complex query DSL beyond basic metadata filtering
5. **Memory Visualization**: No UI for browsing or managing memories
6. **Synchronous Retrieval Requirement**: Retrieval failures will not block execution
7. **Vector Databases Beyond Qdrant/In-Memory**: No Pinecone, Weaviate, or other providers in initial release
8. **Memory Compression**: No LLM-based summarization to reduce token usage
9. **Cross-Agent Memory Sharing**: Memories are scoped to individual Paladin IDs
10. **Memory Versioning**: No tracking of memory updates/history

---

## 6. Design Considerations

### 6.1 Architecture

The RAG integration follows the hexagonal architecture:

```
Core Layer (Domain)
├── Memory (value object)
├── MemoryType (enum)
└── SanctumEntry (from Epic 11)

Application Layer (Use Cases)
├── Ports
│   ├── SanctumPort (trait - from Epic 11)
│   └── EmbeddingPort (trait - from Epic 11)
└── Services
    ├── RagRetrievalService
    └── MemoryExtractionService

Infrastructure Layer (Adapters)
├── QdrantSanctum (implements SanctumPort)
├── InMemorySanctum (implements SanctumPort - from Epic 11)
└── OpenAIEmbeddingAdapter (implements EmbeddingPort - from Epic 11)
```

### 6.2 Execution Flow

```
1. User calls Paladin::run(task)
2. PaladinExecutionService::execute() invoked
3. IF sanctum configured:
   a. RagRetrievalService::retrieve_context() called
   b. Query embedded using EmbeddingPort
   c. SanctumPort::search() queries vector DB
   d. Results filtered, ranked, formatted
   e. Context injected into system prompt
4. LLM call executed with enriched prompt
5. Response returned to user
6. IF extraction_strategy == OnCompletion:
   a. MemoryExtractionService::extract_memories() called async
   b. LLM analyzes conversation
   c. Memories embedded using EmbeddingPort
   d. Duplicates checked via similarity search
   e. New memories stored via SanctumPort
```

### 6.3 Memory Format in Prompt

Retrieved memories are formatted as:

```
## Relevant Context

The following memories may be relevant to your current task:

**Memory 1** (Similarity: 0.92)
Type: Semantic
Content: User prefers Python over JavaScript for backend development.
Source: Conversation on 2026-01-15

**Memory 2** (Similarity: 0.88)
Type: Episodic
Content: User successfully implemented RAG system using Qdrant last month.
Source: Conversation on 2025-12-20

---
```

### 6.4 Configuration Example

```yaml
# config.yml
sanctum:
  provider: "qdrant"  # or "in_memory"
  qdrant:
    url: "http://localhost:6333"
    api_key: "${QDRANT_API_KEY}"  # optional
    collection_name: "paladin_memories"
    vector_size: 1536
    distance: "Cosine"
    on_disk: true

rag:
  top_k: 5
  min_similarity: 0.7
  max_tokens: 2000
  retrieval_trigger: "always"  # always, keyword_based, semantic_threshold
  timeout_seconds: 5

memory_extraction:
  strategy: "on_completion"  # every_turn, on_completion, manual, threshold
  threshold: 0.7  # only used if strategy = threshold
```

### 6.5 Per-Paladin Override Example

```rust
let paladin = PaladinBuilder::new(llm_port)
    .name("ResearchAssistant")
    .system_prompt("You are a research assistant...")
    .with_sanctum(qdrant_sanctum)
    .with_embedding_port(openai_embeddings)
    .memory_extraction_strategy(MemoryExtractionStrategy::Threshold { importance: 0.8 })
    .build()?;
```

---

## 7. Technical Considerations

### 7.1 Dependencies

- **Qdrant Client**: Add `qdrant-client = "1.7"` to `Cargo.toml`
- **Async Runtime**: Ensure Tokio runtime configured for async operations
- **Epic 11 Completion**: All Epic 11 components (ports, types, in-memory adapter) must be complete

### 7.2 Performance Targets

- **Retrieval Latency**: < 500ms p95 for queries with < 100k vectors
- **Extraction Latency**: < 3 seconds p95 for conversations < 10 messages
- **Memory Overhead**: < 100MB for in-memory store with 10k vectors
- **Concurrent Requests**: Support 10+ concurrent Paladin executions with shared Sanctum

### 7.3 Testing Strategy

- **Unit Tests**: All service methods with mocked dependencies
- **Integration Tests**: Qdrant container tests for full CRUD operations
- **Functional Tests**: End-to-end Paladin execution with RAG enabled
- **Load Tests**: Verify performance targets with 100k vectors

### 7.4 Error Scenarios

| Scenario | Behavior |
|----------|----------|
| Qdrant unreachable during retrieval | Log warning, continue with empty context |
| Qdrant unreachable during storage | Log error, continue (memory not stored) |
| Embedding generation fails | Log error, skip retrieval/storage |
| Extraction LLM call fails | Log error, no memories stored |
| Duplicate memory detected | Skip storage, log info |
| Token budget exceeded | Truncate lowest-scoring memories, log warning |

### 7.5 Security Considerations

- **API Key Storage**: Use environment variables, never commit to git
- **Memory Access Control**: No authentication in Epic 12 (single-tenant)
- **Data Sanitization**: Validate memory content before storage
- **PII Handling**: No automatic PII detection (future epic)

---

## 8. Success Metrics

The following metrics will determine if Epic 12 is successful:

### 8.1 Functional Metrics
- ✅ All 5 user stories pass acceptance criteria
- ✅ 100% unit test coverage for new services
- ✅ Integration tests pass with Qdrant Docker container
- ✅ Functional test demonstrates end-to-end RAG flow

### 8.2 Performance Metrics
- 📊 Retrieval latency < 500ms p95
- 📊 Extraction latency < 3 seconds p95
- 📊 Memory retrieval hit rate > 80% (memories found for queries)
- 📊 Zero degradation in Paladin execution time when retrieval fails

### 8.3 Quality Metrics
- 🔍 Zero clippy warnings
- 🔍 Code formatted with `cargo fmt`
- 🔍 All public APIs documented with rustdoc
- 🔍 Examples run successfully

### 8.4 Documentation Metrics
- 📚 `docs/SANCTUM.md` updated with RAG integration details
- 📚 Example `examples/paladin_with_rag.rs` demonstrates basic usage
- 📚 Example `examples/cli_configs/paladin_rag.yaml` shows configuration
- 📚 README includes RAG quick start section

---

## 9. Open Questions

### Q1: Memory Namespace Strategy
**Question**: Should memories be namespaced by Paladin instance ID, or by user ID?  
**Current Assumption**: Paladin instance ID (each Paladin has isolated memories)  
**Impact**: Affects memory retrieval queries and deduplication

### Q2: Embedding Model Selection
**Question**: Should we support multiple embedding models, or standardize on one?  
**Current Assumption**: Use `text-embedding-3-small` (1536 dim) as default  
**Impact**: Vector dimension must match across all memories in collection

### Q3: Memory Importance Scoring
**Question**: Should importance scoring use LLM analysis, or simpler heuristics?  
**Current Assumption**: Use LLM analysis for quality, despite higher cost  
**Impact**: Extraction latency and cost trade-off

### Q4: Retrieval Timeout Handling
**Question**: Should timeout be configurable per-Paladin, or global only?  
**Current Assumption**: Global default with per-Paladin override support  
**Impact**: Builder API design

### Q5: Background Extraction
**Question**: Should memory extraction run in background thread pool or inline?  
**Current Assumption**: Tokio task spawn (non-blocking async)  
**Impact**: Resource usage and error handling complexity

---

## 10. Risks and Mitigations

| Risk | Impact | Probability | Mitigation |
|------|--------|-------------|------------|
| Qdrant integration complexity | High | Medium | Start with in-memory, add Qdrant incrementally |
| Retrieval latency exceeds targets | High | Medium | Implement caching, optimize query filters |
| Memory extraction cost (LLM calls) | Medium | High | Use cheaper model (GPT-3.5), batch operations |
| Token budget management complexity | Medium | Medium | Implement simple truncation first, optimize later |
| Duplicate detection false positives | Low | Low | Tune similarity threshold based on testing |

---

## 11. Implementation Phases

### Phase 1: Foundation (Days 1-3)
- [ ] Implement `QdrantSanctum` adapter
- [ ] Add Qdrant integration tests
- [ ] Extend `PaladinBuilder` with Sanctum methods

### Phase 2: Retrieval (Days 4-6)
- [ ] Implement `RagRetrievalService`
- [ ] Add deduplication and ranking logic
- [ ] Integrate retrieval into `PaladinExecutionService`
- [ ] Add configuration support

### Phase 3: Extraction (Days 7-9)
- [ ] Implement `MemoryExtractionService`
- [ ] Add extraction prompt templates
- [ ] Integrate extraction into execution flow
- [ ] Add async extraction handling

### Phase 4: Testing & Polish (Days 10-14)
- [ ] Write comprehensive unit tests
- [ ] Write integration tests
- [ ] Write functional end-to-end tests
- [ ] Update documentation
- [ ] Create examples
- [ ] Performance testing and optimization

---

## 12. Acceptance Criteria (Epic-Level)

Epic 12 is considered complete when:

- [x] Epic 11 completed (prerequisite)
- [ ] All 5 user stories meet acceptance criteria
- [ ] `QdrantSanctum` adapter fully functional with tests
- [ ] `RagRetrievalService` retrieves and formats memories correctly
- [ ] `MemoryExtractionService` extracts and stores memories correctly
- [ ] `PaladinExecutionService` integrates RAG seamlessly
- [ ] Configuration supported via YAML and builder API
- [ ] Global defaults work with per-Paladin overrides
- [ ] Asynchronous retrieval with graceful degradation
- [ ] Deduplication prevents duplicate storage
- [ ] Token budget management via relevance ranking
- [ ] All tests pass (unit, integration, functional)
- [ ] Performance targets met (< 500ms retrieval, < 3s extraction)
- [ ] Documentation complete and examples working
- [ ] Zero clippy warnings, code formatted
- [ ] Snyk security scan passed

---

## 13. Appendix

### A. Related Documentation
- Epic 11 PRD: Sanctum Memory Foundation
- `docs/SANCTUM.md`: Sanctum system architecture
- `docs/PALADIN.md`: Paladin agent architecture
- `docs/GARRISON.md`: Short-term memory system

### B. Glossary
- **RAG**: Retrieval-Augmented Generation - technique of enriching LLM prompts with retrieved context
- **Sanctum**: Paladin's long-term memory system using vector embeddings
- **Garrison**: Paladin's short-term memory (conversation history)
- **Memory**: A stored piece of information with vector embedding
- **Embedding**: Vector representation of text for semantic similarity
- **Vector Database**: Database optimized for similarity search on vector embeddings

### C. Reference Implementation Links
- Qdrant Documentation: https://qdrant.tech/documentation/
- Qdrant Rust Client: https://github.com/qdrant/rust-client
- OpenAI Embeddings API: https://platform.openai.com/docs/guides/embeddings

---

**Document End**
