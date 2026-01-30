# Task List: Sanctum Memory Foundation

## Relevant Files

### Application Layer (Ports)
- `src/application/ports/output/embedding_port.rs` - EmbeddingPort trait definition for pluggable embedding providers
- `src/application/ports/output/sanctum_port.rs` - SanctumPort trait definition for vector storage operations
- `src/application/ports/output/mod.rs` - Module exports for new ports

### Core Domain
- `src/core/platform/container/sanctum.rs` - Domain models: Memory, MemoryType, MemoryDecayStrategy, SanctumEntry
- `src/core/platform/container/mod.rs` - Module exports for sanctum domain

### Infrastructure (Adapters)
- `src/infrastructure/adapters/llm/openai_embedding_adapter.rs` - OpenAI API embedding implementation
- `src/infrastructure/adapters/llm/mod.rs` - Module exports for embedding adapters
- `src/infrastructure/adapters/sanctum/in_memory_adapter.rs` - In-memory vector store with brute-force cosine similarity
- `src/infrastructure/adapters/sanctum/qdrant_adapter.rs` - Qdrant vector database adapter
- `src/infrastructure/adapters/sanctum/mod.rs` - Module exports for sanctum adapters
- `src/infrastructure/adapters/mod.rs` - Root adapter module exports

### Configuration
- `src/config/application_settings.rs` - SanctumConfig, EmbeddingConfig structs
- `config.yml` - Sanctum configuration section
- `config.test.yml` - Test configuration for Sanctum

### Documentation
- `docs/SANCTUM.md` - Comprehensive user guide with architecture, usage examples, and API reference

### Examples
- `examples/sanctum_basics.rs` - Basic embedding and vector storage workflow
- `examples/sanctum_qdrant.rs` - Production Qdrant integration example
- `examples/paladin_with_sanctum.rs` - Paladin using both Garrison and Sanctum

### Tests
- `tests/unit/embedding_port_tests.rs` - Unit tests for EmbeddingPort trait contract
- `tests/unit/sanctum_port_tests.rs` - Unit tests for SanctumPort trait contract
- `tests/unit/sanctum_domain_tests.rs` - Unit tests for domain models
- `tests/integration/openai_embedding_tests.rs` - Integration tests with mocked OpenAI API
- `tests/integration/qdrant_sanctum_tests.rs` - Integration tests with Docker Compose Qdrant
- `tests/integration/in_memory_sanctum_tests.rs` - Integration tests for in-memory adapter
- `benches/sanctum_benchmarks.rs` - Performance benchmarks for vector search operations

### Build Configuration
- `Cargo.toml` - Add feature flags and dependencies (qdrant-client, async-trait)
- `docker/docker-compose.yml` - Add Qdrant service for integration tests

### Notes

- **TDD Approach:** Write tests FIRST for each component before implementation
- **Hexagonal Architecture:** Core domain has NO dependencies on application or infrastructure layers
- **Feature Flags:** Use `qdrant` feature flag for optional Qdrant dependency
- **Error Handling:** Use `thiserror` for all error enums following existing patterns
- **Async:** All port methods are async using `#[async_trait]`
- **Thread Safety:** All adapters must be `Send + Sync`
- **Testing:** Run `cargo test` after each sub-task completion
- **Code Quality:** Run `cargo fmt` and `cargo clippy` before committing

## Instructions for Completing Tasks

**IMPORTANT:** As you complete each task, you must check it off in this markdown file by changing `- [ ]` to `- [x]`. This helps track progress and ensures you don't skip any steps.

Example:

- `- [ ] 1.1 Read file` → `- [x] 1.1 Read file` (after completing)

Update the file after completing each sub-task, not just after completing an entire parent task.

**Completion Protocol:**
1. Mark sub-task as `[x]` when completed
2. Run `cargo test` to verify tests pass
3. Run `cargo fmt` and `cargo clippy` to ensure code quality
4. When ALL sub-tasks under a parent are `[x]`, run full test suite: `cargo test`
5. If all tests pass, stage changes: `git add .`
6. Commit with descriptive message using conventional commit format
7. Mark parent task as `[x]`

## Tasks

- [x] 0.0 Create feature branch
  - [x] 0.1 Create and checkout new branch: `git checkout -b feature/epic11-sanctum-memory-foundation`
  - [x] 0.2 Verify you're on the new branch: `git branch --show-current`

- [x] 1.0 Define EmbeddingPort trait and error types (US-11.1)
  - [x] 1.1 Write unit tests for `EmbeddingError` enum in `tests/unit/embedding_port_tests.rs`
  - [x] 1.2 Create `src/application/ports/output/embedding_port.rs` file
  - [x] 1.3 Define `EmbeddingError` enum with variants: NetworkError, RateLimited, InvalidInput, ProviderError
  - [x] 1.4 Add `#[derive(Debug, thiserror::Error)]` to EmbeddingError with error messages
  - [x] 1.5 Define `Embedding` struct with fields: vector (Vec<f32>), model (String), dimension (usize), token_count (Option<u32>)
  - [x] 1.6 Add `#[derive(Debug, Clone)]` and Serde traits to Embedding
  - [x] 1.7 Write unit tests for Embedding struct construction and serialization
  - [x] 1.8 Define `EmbeddingPort` trait with `#[async_trait]` and `Send + Sync` bounds
  - [x] 1.9 Add trait methods: `embed_text()`, `embed_batch()`, `dimension()`, `model_name()`
  - [x] 1.10 Write rustdoc comments for all public types and methods
  - [x] 1.11 Export EmbeddingPort in `src/application/ports/output/mod.rs`
  - [x] 1.12 Run `cargo test` to verify tests pass
  - [x] 1.13 Run `cargo fmt` and `cargo clippy` to ensure code quality

- [ ] 2.0 Implement OpenAI Embedding Adapter (US-11.2)
  - [ ] 2.1 Add dependencies to `Cargo.toml`: ensure `reqwest`, `tokio`, `serde`, `serde_json`, `async-trait`
  - [ ] 2.2 Add feature flag `openai-embeddings = []` to default features in Cargo.toml
  - [ ] 2.3 Write integration test skeleton in `tests/integration/openai_embedding_tests.rs` with mocked HTTP
  - [ ] 2.4 Create `src/infrastructure/adapters/llm/openai_embedding_adapter.rs` file
  - [ ] 2.5 Define `OpenAIEmbeddingConfig` struct with fields: api_key, model, base_url, max_retries, timeout_seconds
  - [ ] 2.6 Implement `Default` for OpenAIEmbeddingConfig (model: "text-embedding-3-small")
  - [ ] 2.7 Define `OpenAIEmbeddingAdapter` struct with client (reqwest::Client) and config fields
  - [ ] 2.8 Implement `new()` constructor for OpenAIEmbeddingAdapter
  - [ ] 2.9 Implement `EmbeddingPort` trait for OpenAIEmbeddingAdapter
  - [ ] 2.10 Implement `embed_text()` with retry logic and exponential backoff
  - [ ] 2.11 Implement `embed_batch()` with API limit enforcement (max 2048 inputs)
  - [ ] 2.12 Implement `dimension()` to return correct dimensions per model (1536 for small, 3072 for large)
  - [ ] 2.13 Implement `model_name()` to return configured model
  - [ ] 2.14 Add error handling for HTTP errors, rate limits, and invalid responses
  - [ ] 2.15 Write unit tests for config validation and dimension mapping
  - [ ] 2.16 Write integration tests with mocked HTTP responses (success, rate limit, error)
  - [ ] 2.17 Add rustdoc comments for all public types and methods
  - [ ] 2.18 Export OpenAIEmbeddingAdapter in `src/infrastructure/adapters/llm/mod.rs`
  - [ ] 2.19 Run `cargo test` to verify all tests pass
  - [ ] 2.20 Run `cargo fmt` and `cargo clippy`

- [ ] 3.0 Define SanctumPort trait and core domain models (US-11.3, US-11.6)
  - [ ] 3.1 Write unit tests for domain models in `tests/unit/sanctum_domain_tests.rs`
  - [ ] 3.2 Create `src/core/platform/container/sanctum.rs` file
  - [ ] 3.3 Define `MemoryType` enum: Episodic, Semantic, Procedural
  - [ ] 3.4 Add `#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]` to MemoryType
  - [ ] 3.5 Define `MemoryDecayStrategy` enum: NoDecay, LinearDecay, AccessBasedDecay, CustomDecay
  - [ ] 3.6 Add derives and rustdoc to MemoryDecayStrategy
  - [ ] 3.7 Define `Memory` struct with all fields: id, paladin_id, content, memory_type, importance, access_count, timestamps, metadata
  - [ ] 3.8 Add builder pattern for Memory construction (MemoryBuilder struct)
  - [ ] 3.9 Add validation methods to Memory (validate_importance range 0.0-1.0)
  - [ ] 3.10 Define `SanctumEntry` struct: memory + embedding (Vec<f32>)
  - [ ] 3.11 Add validation for embedding dimension matching
  - [ ] 3.12 Write unit tests for Memory creation, validation, and serialization
  - [ ] 3.13 Write unit tests for SanctumEntry with various embedding dimensions
  - [ ] 3.14 Export sanctum module in `src/core/platform/container/mod.rs`
  - [ ] 3.15 Create `src/application/ports/output/sanctum_port.rs` file
  - [ ] 3.16 Define `SanctumError` enum: StorageError, SearchError, InvalidDimension, NotFound, ConfigError
  - [ ] 3.17 Add thiserror derives to SanctumError
  - [ ] 3.18 Define `SanctumFilter` struct for metadata filtering
  - [ ] 3.19 Define `SanctumQuery` struct: embedding, top_k, filter, min_score
  - [ ] 3.20 Define `SanctumSearchResult` struct: entry + score (f32)
  - [ ] 3.21 Write unit tests for query and filter construction in `tests/unit/sanctum_port_tests.rs`
  - [ ] 3.22 Define `SanctumPort` trait with `#[async_trait]` and `Send + Sync`
  - [ ] 3.23 Add trait methods: `store()`, `store_batch()`, `search()`, `delete()`, `update()`, `count()`
  - [ ] 3.24 Add rustdoc comments for all domain types and port methods
  - [ ] 3.25 Export SanctumPort in `src/application/ports/output/mod.rs`
  - [ ] 3.26 Run `cargo test` to verify domain model tests pass
  - [ ] 3.27 Run `cargo fmt` and `cargo clippy`

- [ ] 4.0 Implement In-Memory Sanctum adapter (US-11.5)
  - [ ] 4.1 Write integration tests in `tests/integration/in_memory_sanctum_tests.rs` (TDD)
  - [ ] 4.2 Test cases: store, retrieve, search (cosine similarity), delete, update, count
  - [ ] 4.3 Test case: LRU eviction when capacity reached
  - [ ] 4.4 Test case: thread-safety with concurrent operations
  - [ ] 4.5 Create `src/infrastructure/adapters/sanctum/in_memory_adapter.rs` file
  - [ ] 4.6 Define `InMemorySanctumConfig` struct: max_entries, eviction_strategy
  - [ ] 4.7 Define `InMemorySanctum` struct with `Arc<RwLock<HashMap<String, SanctumEntry>>>`
  - [ ] 4.8 Add LRU tracking structure (VecDeque or linked list for access order)
  - [ ] 4.9 Implement `new()` constructor with config
  - [ ] 4.10 Implement helper function for cosine similarity calculation
  - [ ] 4.11 Implement `SanctumPort::store()` with capacity check and LRU eviction
  - [ ] 4.12 Implement `SanctumPort::store_batch()` using store() internally
  - [ ] 4.13 Implement `SanctumPort::search()` with brute-force vector comparison
  - [ ] 4.14 Apply filters (paladin_id, memory_type) in search
  - [ ] 4.15 Sort results by similarity score descending, apply top_k and min_score
  - [ ] 4.16 Implement `SanctumPort::delete()` with bool return (found/not found)
  - [ ] 4.17 Implement `SanctumPort::update()` 
  - [ ] 4.18 Implement `SanctumPort::count()`
  - [ ] 4.19 Add rustdoc comments for all methods
  - [ ] 4.20 Create `src/infrastructure/adapters/sanctum/mod.rs` and export InMemorySanctum
  - [ ] 4.21 Export sanctum adapters in `src/infrastructure/adapters/mod.rs`
  - [ ] 4.22 Run integration tests: `cargo test --test in_memory_sanctum_tests`
  - [ ] 4.23 Verify performance < 100ms for 10K vectors (add benchmark if needed)
  - [ ] 4.24 Run `cargo fmt` and `cargo clippy`

- [ ] 5.0 Implement Qdrant Sanctum adapter (US-11.4)
  - [ ] 5.1 Add `qdrant-client` dependency to Cargo.toml as optional
  - [ ] 5.2 Add feature flag `qdrant = ["qdrant-client"]` to Cargo.toml
  - [ ] 5.3 Add Qdrant service to `docker/docker-compose.yml` (port 6334)
  - [ ] 5.4 Write integration tests in `tests/integration/qdrant_sanctum_tests.rs` (TDD)
  - [ ] 5.5 Test setup: Start Qdrant via Docker Compose, create test collection
  - [ ] 5.6 Test cases: store, search with filters, delete, update, count, batch operations
  - [ ] 5.7 Test case: collection auto-creation on first use
  - [ ] 5.8 Create `src/infrastructure/adapters/sanctum/qdrant_adapter.rs` file
  - [ ] 5.9 Add `#[cfg(feature = "qdrant")]` conditional compilation
  - [ ] 5.10 Define `QdrantSanctumConfig` struct: host, port, api_key, collection, use_grpc
  - [ ] 5.11 Define `QdrantSanctumAdapter` struct with Qdrant client and config
  - [ ] 5.12 Implement `new()` constructor with connection initialization
  - [ ] 5.13 Implement helper: `ensure_collection_exists()` with auto-creation
  - [ ] 5.14 Configure collection with cosine distance metric and proper indexing
  - [ ] 5.15 Implement `SanctumPort::store()` - convert SanctumEntry to Qdrant point
  - [ ] 5.16 Implement `SanctumPort::store_batch()` using Qdrant batch upsert
  - [ ] 5.17 Implement `SanctumPort::search()` with Qdrant search API
  - [ ] 5.18 Convert SanctumFilter to Qdrant filter conditions
  - [ ] 5.19 Implement `SanctumPort::delete()` using Qdrant delete API
  - [ ] 5.20 Implement `SanctumPort::update()` using Qdrant upsert
  - [ ] 5.21 Implement `SanctumPort::count()` using Qdrant count API
  - [ ] 5.22 Add retry logic with exponential backoff for transient errors
  - [ ] 5.23 Add error handling and conversion from Qdrant errors to SanctumError
  - [ ] 5.24 Add rustdoc comments for all methods
  - [ ] 5.25 Export QdrantSanctumAdapter in `src/infrastructure/adapters/sanctum/mod.rs` with feature gate
  - [ ] 5.26 Start Qdrant: `docker-compose -f docker/docker-compose.yml up -d qdrant`
  - [ ] 5.27 Run integration tests: `cargo test --test qdrant_sanctum_tests --features qdrant`
  - [ ] 5.28 Run `cargo fmt` and `cargo clippy`

- [ ] 6.0 Add Sanctum configuration support
  - [ ] 6.1 Read existing `src/config/application_settings.rs` to understand config structure
  - [ ] 6.2 Define `SanctumStorageConfig` enum: InMemory(InMemorySanctumConfig), Qdrant(QdrantSanctumConfig)
  - [ ] 6.3 Define `EmbeddingConfig` struct with provider field and OpenAI config
  - [ ] 6.4 Define `MemoryConfig` struct: default_importance, decay_strategy
  - [ ] 6.5 Define `SanctumConfig` struct: storage, embedding, memory
  - [ ] 6.6 Add SanctumConfig to main ApplicationSettings struct
  - [ ] 6.7 Add validation methods to ensure config consistency
  - [ ] 6.8 Update `config.yml` with sanctum section following PRD structure
  - [ ] 6.9 Update `config.test.yml` with test-specific sanctum config (in-memory)
  - [ ] 6.10 Write unit tests for config loading and validation
  - [ ] 6.11 Test environment variable substitution for API keys
  - [ ] 6.12 Run `cargo test` to verify config tests pass
  - [ ] 6.13 Run `cargo fmt` and `cargo clippy`

- [ ] 7.0 Create Sanctum documentation
  - [ ] 7.1 Create `docs/SANCTUM.md` file
  - [ ] 7.2 Write Introduction section explaining Sanctum vs Garrison
  - [ ] 7.3 Write Architecture section with diagram from PRD
  - [ ] 7.4 Write Quick Start section with basic usage example
  - [ ] 7.5 Write Embedding Providers section (OpenAI setup)
  - [ ] 7.6 Write Vector Storage section (in-memory vs Qdrant)
  - [ ] 7.7 Write Memory Types section (Episodic, Semantic, Procedural)
  - [ ] 7.8 Write Memory Decay Strategies section
  - [ ] 7.9 Write Configuration section with YAML examples
  - [ ] 7.10 Write API Reference section with port trait documentation
  - [ ] 7.11 Write Performance Considerations section with benchmarks
  - [ ] 7.12 Write Integration with Paladin section
  - [ ] 7.13 Write Troubleshooting section with common issues
  - [ ] 7.14 Write Future Enhancements section (local models, other vector DBs)
  - [ ] 7.15 Review documentation for clarity and completeness

- [ ] 8.0 Create example applications
  - [ ] 8.1 Create `examples/sanctum_basics.rs` - Basic embedding and storage workflow
  - [ ] 8.2 Write example: Create OpenAI embedding adapter
  - [ ] 8.3 Write example: Embed text and create SanctumEntry
  - [ ] 8.4 Write example: Store entry in in-memory Sanctum
  - [ ] 8.5 Write example: Search for similar memories
  - [ ] 8.6 Write example: Display search results with scores
  - [ ] 8.7 Add comments explaining each step
  - [ ] 8.8 Test example runs: `cargo run --example sanctum_basics`
  - [ ] 8.9 Create `examples/sanctum_qdrant.rs` - Qdrant integration example
  - [ ] 8.10 Write example: Configure Qdrant adapter from environment
  - [ ] 8.11 Write example: Batch storage and search operations
  - [ ] 8.12 Test example runs: `cargo run --example sanctum_qdrant --features qdrant`
  - [ ] 8.13 Create `examples/paladin_with_sanctum.rs` - Paladin + Garrison + Sanctum
  - [ ] 8.14 Write example: Configure Paladin with both memory systems
  - [ ] 8.15 Write example: Short-term context in Garrison, long-term in Sanctum
  - [ ] 8.16 Write example: Search Sanctum for relevant past experiences
  - [ ] 8.17 Test example runs: `cargo run --example paladin_with_sanctum`
  - [ ] 8.18 Update `examples/README.md` with Sanctum examples section

- [ ] 9.0 Integration testing and performance benchmarks
  - [ ] 9.1 Create `benches/sanctum_benchmarks.rs` file
  - [ ] 9.2 Benchmark: Embedding generation (single and batch)
  - [ ] 9.3 Benchmark: In-memory search at 1K, 5K, 10K vectors
  - [ ] 9.4 Benchmark: Qdrant search at 10K, 50K, 100K vectors
  - [ ] 9.5 Benchmark: Store operations (single and batch)
  - [ ] 9.6 Run benchmarks: `cargo bench --bench sanctum_benchmarks`
  - [ ] 9.7 Verify performance targets: < 100ms in-memory, < 500ms Qdrant
  - [ ] 9.8 Document benchmark results in docs/SANCTUM.md
  - [ ] 9.9 Write end-to-end integration test combining all components
  - [ ] 9.10 Test: Create embedding → Store in Sanctum → Search → Retrieve
  - [ ] 9.11 Test with both in-memory and Qdrant adapters
  - [ ] 9.12 Run full test suite: `cargo test --all-features`
  - [ ] 9.13 Check test coverage: `cargo llvm-cov --summary-only`
  - [ ] 9.14 Ensure ≥ 80% unit coverage, ≥ 70% integration coverage

- [ ] 10.0 Final review, security scan, and polish
  - [ ] 10.1 Run `cargo fmt` on entire codebase
  - [ ] 10.2 Run `cargo clippy -- -D warnings` and fix all warnings
  - [ ] 10.3 Run `cargo audit` to check for security vulnerabilities
  - [ ] 10.4 Run Snyk scan on new code following `.github/instructions/snyk_rules.instructions.md`
  - [ ] 10.5 Fix any security issues found
  - [ ] 10.6 Review all rustdoc comments for completeness and accuracy
  - [ ] 10.7 Run `cargo doc --open` and verify documentation renders correctly
  - [ ] 10.8 Review error messages for clarity and helpfulness
  - [ ] 10.9 Verify all feature flags work correctly (test with and without qdrant)
  - [ ] 10.10 Test backward compatibility: existing code works without Sanctum
  - [ ] 10.11 Run full test suite one final time: `make test-all`
  - [ ] 10.12 Review git diff to ensure no debug code or temp files
  - [ ] 10.13 Stage all changes: `git add .`
  - [ ] 10.14 Commit with message: `feat: add Sanctum memory foundation (Epic 11)` with detailed body
  - [ ] 10.15 Push branch: `git push origin feature/sanctum-memory-foundation`
  - [ ] 10.16 Create pull request with PRD and task list references
  - [ ] 10.17 Update Epic 11 status in project documentation
