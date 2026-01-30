# Task List: Sanctum RAG Integration (Epic 12)

**Feature:** Retrieval-Augmented Generation with Sanctum Memory System  
**PRD:** `prd-sanctum-rag-integration.md`  
**Epic:** Epic 12  
**Estimated Duration:** 2 weeks (14 days)  
**Dependencies:** Epic 11 (Sanctum Memory Foundation) must be complete

---

## Relevant Files

### Core Domain Layer
- `src/core/platform/container/sanctum.rs` - Memory and MemoryType domain models (Epic 11)
- `src/core/platform/container/paladin.rs` - Paladin domain entity extensions for RAG
- `src/core/platform/container/paladin_config.rs` - RAG configuration structures

### Application Layer (Ports)
- `src/application/ports/output/sanctum_port.rs` - SanctumPort trait (Epic 11)
- `src/application/ports/output/embedding_port.rs` - EmbeddingPort trait (Epic 11)

### Application Layer (Use Cases)
- `src/application/use_cases/sanctum/mod.rs` - Sanctum use cases module
- `src/application/use_cases/sanctum/rag_retrieval_service.rs` - RAG retrieval logic (NEW)
- `src/application/use_cases/sanctum/memory_extraction_service.rs` - Memory extraction logic (NEW)
- `src/application/use_cases/paladin/paladin_builder.rs` - Builder extensions for Sanctum
- `src/application/use_cases/paladin/paladin_execution_service.rs` - RAG integration into execution

### Infrastructure Layer (Adapters)
- `src/infrastructure/adapters/sanctum/mod.rs` - Sanctum adapters module
- `src/infrastructure/adapters/sanctum/qdrant_sanctum.rs` - Qdrant vector store adapter (NEW)
- `src/infrastructure/adapters/sanctum/in_memory_sanctum.rs` - In-memory adapter (Epic 11)

### Configuration
- `config.yml` - Global configuration with sanctum, rag, and memory_extraction sections
- `config.test.yml` - Test configuration

### Tests
- `tests/unit/sanctum/qdrant_sanctum_test.rs` - Qdrant adapter unit tests
- `tests/unit/sanctum/rag_retrieval_service_test.rs` - RAG retrieval service tests
- `tests/unit/sanctum/memory_extraction_service_test.rs` - Memory extraction service tests
- `tests/integration/qdrant_integration_test.rs` - Qdrant Docker container integration tests
- `tests/functional/paladin_rag_test.rs` - End-to-end RAG functionality test

### Examples
- `examples/paladin_with_rag.rs` - Basic RAG usage example
- `examples/cli_configs/paladin_rag.yaml` - Configuration example

### Documentation
- `docs/SANCTUM.md` - Update with RAG integration details
- `README.md` - Update with RAG quick start section

### Notes

- **Testing Strategy**: Follow TDD (Test-Driven Development) - write tests first, then implementation
- **Test Commands**:
  - `cargo test` - Run all unit tests
  - `cargo test --test qdrant_integration_test` - Run Qdrant integration tests
  - `cargo test --test paladin_rag_test` - Run functional tests
  - `make test-integration-docker` - Run integration tests with Docker services
- **Code Quality**:
  - `cargo fmt` - Format code
  - `cargo clippy -- -D warnings` - Lint with warnings as errors
  - `make clean-code` - Format + lint + check
- **Docker Services**: Use `make dev` to start Redis, MinIO, and add Qdrant container
- **Hexagonal Architecture**: Maintain clear boundaries - Core → Application → Infrastructure

---

## Instructions for Completing Tasks

**IMPORTANT:** As you complete each task, you must check it off in this markdown file by changing `- [ ]` to `- [x]`. This helps track progress and ensures you don't skip any steps.

Example:
- `- [ ] 1.1 Read file` → `- [x] 1.1 Read file` (after completing)

**Rust Task Completion Protocol:**
1. Complete the sub-task implementation
2. Write/update tests for the sub-task
3. Run `cargo test` to verify tests pass
4. Run `cargo fmt --check` to verify formatting
5. Run `cargo clippy` to check for warnings
6. Mark sub-task as complete `[x]`
7. Once all sub-tasks under a parent are complete:
   - Run full test suite: `make test-all`
   - Run quality checks: `make clean-code`
   - Stage changes: `git add .`
   - Commit with descriptive message using conventional commits format
   - Mark parent task as complete `[x]`

---

## Tasks

### Phase 1: Foundation & Setup (Days 1-3)

- [x] 0.0 Create feature branch
  - [x] 0.1 Create and checkout new branch: `git checkout -b feature/epic-12-sanctum-rag-integration`
  - [x] 0.2 Verify Epic 11 completion: check that all Epic 11 files exist and tests pass

- [x] 1.0 Set up dependencies and project structure
  - [x] 1.1 Add `qdrant-client = "1.7"` to `Cargo.toml` dependencies
  - [x] 1.2 Create `src/application/use_cases/sanctum/` directory
  - [x] 1.3 Create `src/application/use_cases/sanctum/mod.rs` module file
  - [x] 1.4 Create `src/infrastructure/adapters/sanctum/` directory
  - [x] 1.5 Create `src/infrastructure/adapters/sanctum/mod.rs` module file
  - [x] 1.6 Create `tests/unit/sanctum/` directory for unit tests
  - [x] 1.7 Create `tests/integration/` directory if it doesn't exist
  - [x] 1.8 Update `docker/docker-compose.yml` to include Qdrant service (port 6333)
  - [x] 1.9 Run `cargo check` to verify dependencies resolve correctly
  - [x] 1.10 Run `cargo fmt` and commit initial structure changes

- [x] 2.0 Implement Qdrant Vector Store Adapter (US-12.1)
  - [x] 2.1 Read Epic 11 `SanctumPort` trait definition from `src/application/ports/output/sanctum_port.rs`
  - [x] 2.2 Create `src/infrastructure/adapters/sanctum/qdrant_sanctum.rs` file (already exists from Epic 11)
  - [x] 2.3 Define `QdrantSanctumConfig` struct with fields: url, api_key (Option), collection_name, vector_size, distance, on_disk (already implemented)
  - [x] 2.4 Implement `Default` trait for `QdrantSanctumConfig` with sensible defaults (already implemented)
  - [x] 2.5 Define `QdrantSanctum` struct with `client: QdrantClient` and `config: QdrantSanctumConfig` (already implemented)
  - [x] 2.6 Implement `QdrantSanctum::new(config)` constructor that initializes Qdrant client (already implemented)
  - [x] 2.7 Implement `QdrantSanctum::ensure_collection()` private method to create collection if not exists (already implemented)
  - [x] 2.8 Implement `SanctumPort::store()` - upsert vector with metadata using Qdrant upsert API (already implemented)
  - [x] 2.9 Implement `SanctumPort::search()` - query vectors with cosine similarity, return top-k results (already implemented)
  - [x] 2.10 Implement `SanctumPort::delete()` - delete entry by ID using Qdrant delete API (already implemented)
  - [x] 2.11 Implement `SanctumPort::update()` - update existing entry (upsert with same ID) (already implemented)
  - [x] 2.12 Implement `SanctumPort::count()` - return total entries in collection (already implemented)
  - [x] 2.13 Implement `health_check()` method - verify collection exists and client can connect (available via ensure_collection)
  - [x] 2.14 Add proper error handling: map Qdrant errors to `SanctumError` variants (already implemented)
  - [x] 2.15 Add logging statements for all operations (debug level for operations, error for failures) (already implemented)
  - [x] 2.16 Export `QdrantSanctum` and config from `src/infrastructure/adapters/sanctum/mod.rs` (already exported)
  - [x] 2.17 Write unit tests in `tests/unit/sanctum/qdrant_sanctum_test.rs` with mocked Qdrant client (placeholder tests added)
  - [x] 2.18 Run `cargo test qdrant_sanctum_test` to verify unit tests pass
  - [x] 2.19 Run `cargo clippy` and fix any warnings
  - [x] 2.20 Run `cargo fmt` and commit Qdrant adapter implementation

### Phase 2: Retrieval & Extraction Services (Days 4-9)

- [x] 3.0 Implement RAG Retrieval Service (US-12.3)
  - [x] 3.1 Create `src/application/use_cases/sanctum/rag_retrieval_service.rs` file
  - [x] 3.2 Define `RagConfig` struct with fields: top_k (usize), min_similarity (f32), max_tokens (usize), retrieval_trigger (RetrievalTrigger)
  - [x] 3.3 Define `RetrievalTrigger` enum with variants: Always, KeywordBased, SemanticThreshold
  - [x] 3.4 Implement `Default` for `RagConfig`: top_k=5, min_similarity=0.7, max_tokens=2000, trigger=Always
  - [x] 3.5 Define `RagRetrievalService` struct with fields: sanctum (Arc<dyn SanctumPort>), embedding (Arc<dyn EmbeddingPort>), config (RagConfig)
  - [x] 3.6 Implement `RagRetrievalService::new(sanctum, embedding, config)` constructor
  - [x] 3.7 Implement `retrieve_context(&self, paladin_id: &str, query: &str)` async method
  - [x] 3.8 In `retrieve_context`: Generate query embedding using `embedding.embed_text(query)`
  - [x] 3.9 In `retrieve_context`: Build `SanctumQuery` with embedding, top_k, and paladin_id filter
  - [x] 3.10 In `retrieve_context`: Call `sanctum.search(query)` to get results
  - [x] 3.11 In `retrieve_context`: Filter results by `min_similarity` threshold
  - [x] 3.12 Implement `deduplicate_memories(memories: Vec<Memory>)` - remove memories with >0.95 similarity
  - [x] 3.13 Implement `rank_by_relevance(memories: Vec<Memory>)` - sort by score descending
  - [x] 3.14 Implement `truncate_to_token_budget(memories: Vec<Memory>, max_tokens: usize)` - estimate tokens and remove lowest-scoring
  - [x] 3.15 In `retrieve_context`: Apply deduplication, ranking, and truncation
  - [x] 3.16 Implement `format_for_prompt(&self, memories: &[Memory]) -> String` - format as "## Relevant Context" section
  - [x] 3.17 Add async timeout wrapper: use `tokio::time::timeout(Duration::from_secs(5), retrieve_context)`
  - [x] 3.18 Handle timeout gracefully: return empty Vec with warning log
  - [x] 3.19 Add comprehensive error handling with proper `SanctumError` propagation
  - [x] 3.20 Export `RagRetrievalService` and `RagConfig` from `src/application/use_cases/sanctum/mod.rs`
  - [x] 3.21 Write unit tests in `tests/unit/sanctum/rag_retrieval_service_test.rs` with mocked ports
  - [x] 3.22 Test case: successful retrieval with multiple memories
  - [x] 3.23 Test case: filtering by min_similarity threshold
  - [x] 3.24 Test case: deduplication removes near-identical memories
  - [x] 3.25 Test case: token budget truncation works correctly
  - [x] 3.26 Test case: timeout returns empty context gracefully
  - [x] 3.27 Run `cargo test rag_retrieval_service_test` to verify tests pass
  - [x] 3.28 Run `cargo clippy` and fix warnings
  - [x] 3.29 Run `cargo fmt` and commit RAG retrieval service

- [x] 4.0 Implement Memory Extraction Service (US-12.4)
  - [x] 4.1 Create `src/application/use_cases/sanctum/memory_extraction_service.rs` file
  - [x] 4.2 Define `MemoryExtractionStrategy` enum: EveryTurn, OnCompletion, Manual, Threshold { importance: f32 }
  - [x] 4.3 Implement `Default` for `MemoryExtractionStrategy` returning `OnCompletion`
  - [x] 4.4 Define `MemoryExtractionService` struct with fields: llm (Arc<dyn LlmPort>), embedding (Arc<dyn EmbeddingPort>), sanctum (Arc<dyn SanctumPort>)
  - [x] 4.5 Implement `MemoryExtractionService::new(llm, embedding, sanctum)` constructor
  - [x] 4.6 Create `EXTRACTION_PROMPT` constant with prompt template for LLM to identify memorable content
  - [x] 4.7 Implement `extract_memories(&self, paladin_id: &str, conversation: &[GarrisonEntry])` async method
  - [x] 4.8 In `extract_memories`: Build extraction prompt from conversation history
  - [x] 4.9 In `extract_memories`: Call `llm.generate()` to get structured memory extraction response
  - [x] 4.10 Implement `parse_extraction_response(response: &str) -> Vec<ExtractedMemory>` to parse LLM JSON output
  - [x] 4.11 Define `ExtractedMemory` struct with fields: content, memory_type, importance, metadata
  - [x] 4.12 In `extract_memories`: Generate embeddings for each extracted memory using `embedding.embed_text()`
  - [x] 4.13 Implement `check_for_duplicates(&self, memory: &Memory)` - search for similar existing memories (>0.95 similarity)
  - [x] 4.14 In `extract_memories`: Filter out duplicate memories before storage
  - [x] 4.15 Implement `store_memories(&self, memories: Vec<Memory>)` to batch store via `sanctum.store()`
  - [x] 4.16 Add metrics logging: count, average importance, duration
  - [x] 4.17 Handle all errors gracefully: log but don't fail if extraction fails
  - [x] 4.18 Export `MemoryExtractionService` and `MemoryExtractionStrategy` from module
  - [x] 4.19 Write unit tests in `tests/unit/sanctum/memory_extraction_service_test.rs`
  - [x] 4.20 Test case: successful extraction with multiple memory types
  - [x] 4.21 Test case: importance scoring correctly assigned
  - [x] 4.22 Test case: duplicate detection prevents re-storage
  - [x] 4.23 Test case: LLM failure handled gracefully
  - [x] 4.24 Run `cargo test memory_extraction_service_test` to verify tests pass
  - [x] 4.25 Run `cargo clippy` and fix warnings
  - [x] 4.26 Run `cargo fmt` and commit memory extraction service

### Phase 3: Paladin Integration (Days 7-11)

- [ ] 5.0 Extend PaladinBuilder with Sanctum integration (US-12.2)
  - [ ] 5.1 Read existing `PaladinBuilder` from `src/application/use_cases/paladin/paladin_builder.rs`
  - [ ] 5.2 Add optional fields to `PaladinBuilder`: sanctum (Option<Arc<dyn SanctumPort>>), embedding_port (Option<Arc<dyn EmbeddingPort>>), memory_extraction_strategy (MemoryExtractionStrategy)
  - [ ] 5.3 Implement `with_sanctum(mut self, sanctum: Arc<dyn SanctumPort>) -> Self` method
  - [ ] 5.4 Implement `with_embedding_port(mut self, embedding: Arc<dyn EmbeddingPort>) -> Self` method
  - [ ] 5.5 Implement `memory_extraction_strategy(mut self, strategy: MemoryExtractionStrategy) -> Self` method
  - [ ] 5.6 In `build()` method: validate that if sanctum is provided, embedding_port must also be provided
  - [ ] 5.7 Read `PaladinConfig` from `src/core/platform/container/paladin_config.rs`
  - [ ] 5.8 Add RAG-related fields to `PaladinConfig`: rag_config (Option<RagConfig>), extraction_strategy (Option<MemoryExtractionStrategy>)
  - [ ] 5.9 Update `PaladinBuilder::build()` to store RAG configuration in `PaladinConfig`
  - [ ] 5.10 Write unit tests for builder validation logic
  - [ ] 5.11 Test case: building with sanctum but no embedding_port fails with clear error
  - [ ] 5.12 Test case: building with both sanctum and embedding_port succeeds
  - [ ] 5.13 Test case: default extraction strategy is OnCompletion
  - [ ] 5.14 Run `cargo test paladin_builder` to verify tests pass
  - [ ] 5.15 Run `cargo clippy` and fix warnings
  - [ ] 5.16 Run `cargo fmt` and commit PaladinBuilder extensions

- [ ] 6.0 Integrate RAG into PaladinExecutionService (US-12.5)
  - [ ] 6.1 Read existing `PaladinExecutionService` from `src/application/use_cases/paladin/paladin_execution_service.rs`
  - [ ] 6.2 Add optional fields: rag_retrieval_service (Option<Arc<RagRetrievalService>>), memory_extraction_service (Option<Arc<MemoryExtractionService>>)
  - [ ] 6.3 Update `PaladinExecutionService::new()` to accept optional RAG services
  - [ ] 6.4 Create `check_sanctum_configured(&self, paladin: &Paladin) -> bool` helper method
  - [ ] 6.5 In `execute()` method: add step 1 - check if Sanctum configured
  - [ ] 6.6 In `execute()`: if configured, call `rag_retrieval_service.retrieve_context(paladin.id(), input)`
  - [ ] 6.7 In `execute()`: wrap retrieval in `tokio::time::timeout` with 5-second limit
  - [ ] 6.8 In `execute()`: on retrieval success, format memories and inject into system prompt
  - [ ] 6.9 In `execute()`: on retrieval failure/timeout, log warning and continue with empty context
  - [ ] 6.10 Implement `inject_memories_into_prompt(system_prompt: &str, memories: &str) -> String` helper
  - [ ] 6.11 In `execute()`: after successful LLM response, check extraction strategy
  - [ ] 6.12 In `execute()`: if strategy is `OnCompletion`, spawn async task for memory extraction
  - [ ] 6.13 Implement `extract_memories_async(service, paladin_id, conversation)` that runs in background
  - [ ] 6.14 Add metrics collection: retrieval_latency_ms, memories_retrieved_count, extraction_triggered bool
  - [ ] 6.15 Ensure all errors are logged with appropriate severity levels
  - [ ] 6.16 Write unit tests in `tests/unit/paladin/paladin_execution_service_test.rs`
  - [ ] 6.17 Test case: execution with RAG retrieval injects context correctly
  - [ ] 6.18 Test case: execution continues gracefully when retrieval times out
  - [ ] 6.19 Test case: memory extraction triggered on completion
  - [ ] 6.20 Test case: execution without Sanctum works as before (backward compatibility)
  - [ ] 6.21 Run `cargo test paladin_execution_service_test` to verify tests pass
  - [ ] 6.22 Run `cargo clippy` and fix warnings
  - [ ] 6.23 Run `cargo fmt` and commit PaladinExecutionService integration

### Phase 4: Configuration & Testing (Days 9-14)

- [ ] 7.0 Add configuration support
  - [ ] 7.1 Read existing `config.yml` and identify structure
  - [ ] 7.2 Add `sanctum` section with fields: provider (qdrant/in_memory)
  - [ ] 7.3 Add `sanctum.qdrant` subsection: url, api_key, collection_name, vector_size, distance, on_disk
  - [ ] 7.4 Add `sanctum.in_memory` subsection: max_entries, eviction_strategy
  - [ ] 7.5 Add `rag` section: top_k, min_similarity, max_tokens, retrieval_trigger, timeout_seconds
  - [ ] 7.6 Add `memory_extraction` section: strategy (on_completion/every_turn/manual/threshold), threshold value
  - [ ] 7.7 Update `src/config/application_settings.rs` to parse new configuration sections
  - [ ] 7.8 Define `SanctumConfig`, `QdrantConfig`, `RagConfig`, `MemoryExtractionConfig` structs in config module
  - [ ] 7.9 Implement deserialization with serde for all config structs
  - [ ] 7.10 Add validation logic in config parsing: fail fast on invalid values
  - [ ] 7.11 Update `config.test.yml` with test configuration values
  - [ ] 7.12 Write configuration example in `examples/cli_configs/paladin_rag.yaml`
  - [ ] 7.13 Test configuration loading: `cargo run -- --config examples/cli_configs/paladin_rag.yaml --help`
  - [ ] 7.14 Run `cargo clippy` and fix warnings
  - [ ] 7.15 Run `cargo fmt` and commit configuration support

- [ ] 8.0 Write comprehensive tests
  - [ ] 8.1 Create `tests/integration/qdrant_integration_test.rs` file
  - [ ] 8.2 In integration test: start Qdrant container using testcontainers-rs
  - [ ] 8.3 Test: create collection and verify it exists
  - [ ] 8.4 Test: store multiple vectors and verify count
  - [ ] 8.5 Test: search with query vector and verify top-k results
  - [ ] 8.6 Test: update existing entry and verify changes
  - [ ] 8.7 Test: delete entry and verify it's gone
  - [ ] 8.8 Test: health check returns success
  - [ ] 8.9 Run integration test: `cargo test --test qdrant_integration_test -- --nocapture`
  - [ ] 8.10 Create `tests/functional/paladin_rag_test.rs` file
  - [ ] 8.11 In functional test: create Paladin with in-memory Sanctum
  - [ ] 8.12 Test: run Paladin, verify memories extracted after completion
  - [ ] 8.13 Test: run Paladin again, verify previous memories retrieved
  - [ ] 8.14 Test: verify retrieved context injected into prompt
  - [ ] 8.15 Test: verify RAG improves response quality (check for reference to past context)
  - [ ] 8.16 Run functional test: `cargo test --test paladin_rag_test -- --nocapture`
  - [ ] 8.17 Run all tests: `make test-all`
  - [ ] 8.18 Verify test coverage with `cargo tarpaulin` (if available)
  - [ ] 8.19 Fix any failing tests
  - [ ] 8.20 Run `cargo fmt` and commit comprehensive tests

- [ ] 9.0 Create examples and documentation
  - [ ] 9.1 Create `examples/paladin_with_rag.rs` file
  - [ ] 9.2 In example: demonstrate creating Paladin with OpenAI embeddings and in-memory Sanctum
  - [ ] 9.3 In example: run multiple tasks showing memory persistence
  - [ ] 9.4 In example: show how retrieved memories affect responses
  - [ ] 9.5 Add comprehensive comments explaining each step
  - [ ] 9.6 Test example runs successfully: `cargo run --example paladin_with_rag`
  - [ ] 9.7 Read existing `docs/SANCTUM.md` file
  - [ ] 9.8 Add "RAG Integration" section to `docs/SANCTUM.md`
  - [ ] 9.9 Document architecture: how RAG fits into Paladin execution flow
  - [ ] 9.10 Document configuration options with YAML examples
  - [ ] 9.11 Document API usage with Rust code examples
  - [ ] 9.12 Add troubleshooting section: common issues and solutions
  - [ ] 9.13 Add performance tuning section: top_k, similarity threshold, token budget
  - [ ] 9.14 Read existing `README.md` file
  - [ ] 9.15 Add "RAG Quick Start" section to README
  - [ ] 9.16 Include minimal example code snippet in README
  - [ ] 9.17 Link to full documentation and examples
  - [ ] 9.18 Verify all documentation links work
  - [ ] 9.19 Run `cargo doc --open` to verify rustdoc generates correctly
  - [ ] 9.20 Run `cargo fmt` and commit examples and documentation

- [ ] 10.0 Final quality checks and Epic completion
  - [ ] 10.1 Run full test suite: `make test-all`
  - [ ] 10.2 Verify all tests pass without warnings
  - [ ] 10.3 Run code quality checks: `make clean-code`
  - [ ] 10.4 Fix any clippy warnings or formatting issues
  - [ ] 10.5 Run integration tests with Docker: `make test-integration-docker`
  - [ ] 10.6 Verify all examples run: `cargo run --example paladin_with_rag`
  - [ ] 10.7 Run security audit: `cargo audit`
  - [ ] 10.8 Fix any security vulnerabilities found
  - [ ] 10.9 Review all acceptance criteria in PRD: verify each is met
  - [ ] 10.10 Review US-12.1 acceptance criteria (Qdrant adapter)
  - [ ] 10.11 Review US-12.2 acceptance criteria (Paladin integration)
  - [ ] 10.12 Review US-12.3 acceptance criteria (RAG retrieval)
  - [ ] 10.13 Review US-12.4 acceptance criteria (Memory extraction)
  - [ ] 10.14 Review US-12.5 acceptance criteria (Execution service integration)
  - [ ] 10.15 Verify all 66 functional requirements (FR-1 through FR-10) are implemented
  - [ ] 10.16 Check performance targets: retrieval < 500ms, extraction < 3s
  - [ ] 10.17 Update Epic 12 status document with completion details
  - [ ] 10.18 Create PR description summarizing changes
  - [ ] 10.19 Stage all changes: `git add .`
  - [ ] 10.20 Commit with message: `git commit -m "feat(epic-12): complete Sanctum RAG integration" -m "- Qdrant vector store adapter" -m "- RAG retrieval service" -m "- Memory extraction service" -m "- PaladinBuilder extensions" -m "- Execution service integration" -m "- Configuration support" -m "- Comprehensive tests and examples" -m "- Documentation updates" -m "Closes Epic 12"`
  - [ ] 10.21 Push feature branch: `git push origin feature/epic-12-sanctum-rag-integration`
  - [ ] 10.22 Create pull request to main branch
  - [ ] 10.23 Request code review
  - [ ] 10.24 Address review feedback if any
  - [ ] 10.25 Merge to main after approval

---

**Status:** Sub-tasks Generated - Ready for Implementation

**Total Sub-tasks:** 220  
**Estimated Completion:** 14 days (2 weeks)

**Next Steps:**
1. Start with Task 0.0 (Create feature branch)
2. Follow TDD methodology: write tests first, then implementation
3. Mark each sub-task complete as you finish it
4. Run quality checks after each parent task completion
5. Commit frequently with descriptive messages
