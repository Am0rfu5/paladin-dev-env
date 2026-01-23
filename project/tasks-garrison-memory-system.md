# Task List: Garrison Memory System Implementation

**Epic:** Epic 2  
**Priority:** High  
**Effort:** 2-3 weeks  
**PRD:** [prd-garrison-memory-system.md](prd-garrison-memory-system.md)

---

## Relevant Files

### Core Layer
- `src/core/platform/container/garrison.rs` - Domain entities (GarrisonEntry, ConversationHistory, GarrisonType)
- `src/core/platform/container/mod.rs` - Module exports for garrison domain

### Application Layer
- `src/application/ports/output/garrison_port.rs` - GarrisonPort and LongTermGarrisonPort trait definitions
- `src/application/ports/output/mod.rs` - Port module exports
- `src/application/use_cases/paladin/paladin_builder.rs` - Updated with Garrison integration
- `src/application/use_cases/paladin/paladin_execution_service.rs` - Updated with memory management

### Infrastructure Layer
- `src/infrastructure/adapters/garrison/mod.rs` - Garrison adapter module
- `src/infrastructure/adapters/garrison/in_memory_garrison.rs` - In-memory implementation
- `src/infrastructure/adapters/garrison/sqlite_garrison.rs` - SQLite persistent implementation
- `src/infrastructure/adapters/garrison/token_counter.rs` - Token counting utilities
- `src/infrastructure/adapters/mod.rs` - Infrastructure module exports
- `migrations/001_create_garrison_tables.sql` - Database schema for SQLite

### Configuration
- `src/config/application_settings.rs` - Garrison configuration settings
- `config.yml` - Example configuration
- `config.test.yml` - Test configuration

### Tests
- `tests/unit/garrison_entry_test.rs` - Unit tests for GarrisonEntry
- `tests/unit/conversation_history_test.rs` - Unit tests for ConversationHistory
- `tests/unit/in_memory_garrison_test.rs` - Unit tests for InMemoryGarrison
- `tests/integration/sqlite_garrison_test.rs` - Integration tests for SQLite Garrison
- `tests/integration/paladin_with_garrison_test.rs` - Integration tests for Paladin+Garrison
- `tests/functional/garrison_lifecycle_test.rs` - Functional tests for complete workflows

### Examples
- `examples/garrison_in_memory.rs` - Example using in-memory garrison
- `examples/garrison_persistent.rs` - Example using SQLite garrison
- `examples/garrison_semantic_search.rs` - Example with vector search

### Documentation
- `docs/GARRISON.md` - Garrison system documentation

### Notes

- All Rust code must pass `cargo clippy` with no warnings
- Unit test coverage must be ≥ 80%
- Follow Test-Driven Development (TDD): write tests before implementation
- Use `cargo test` to run all tests
- Use `cargo test --test [test_file]` to run specific integration tests
- Follow Hexagonal Architecture: Core → Application → Infrastructure dependency flow
- Run `cargo fmt` before committing
- Update Cargo.toml with new dependencies (tiktoken-rs, sqlite-vss)

---

## Instructions for Completing Tasks

**IMPORTANT:** As you complete each task, you must check it off in this markdown file by changing `- [ ]` to `- [x]`. This helps track progress and ensures you don't skip any steps.

Example:
- `- [ ] 1.1 Read file` → `- [x] 1.1 Read file` (after completing)

Update the file after completing each sub-task, not just after completing an entire parent task.

---

## Tasks

- [x] 0.0 Create feature branch
  - [x] 0.1 Create and checkout branch `feature/epic_2-garrison-memory-system`

- [x] 1.0 Implement Core Domain Layer (Garrison Entities)
  - [x] 1.1 Create `src/core/platform/container/garrison.rs` file
  - [x] 1.2 Define `ConversationRole` enum (System, User, Assistant, Tool) with Serialize/Deserialize
  - [x] 1.3 Define `GarrisonEntry` struct with all fields (id, role, content, timestamp, metadata, token_count)
  - [x] 1.4 Implement validation methods for `GarrisonEntry` (validate_required_fields)
  - [x] 1.5 Define `GarrisonType` enum (ShortTerm, LongTerm, Episodic)
  - [x] 1.6 Define `ConversationHistory` struct with VecDeque, max_entries, max_tokens
  - [x] 1.7 Implement `ConversationHistory::new()`, `add()`, `get_recent()`, `total_tokens()` methods
  - [x] 1.8 Implement windowing logic in `ConversationHistory` (auto-evict when limits exceeded)
  - [x] 1.9 Define `GarrisonConfig` struct with Builder pattern support
  - [x] 1.10 Define `EvictionStrategy` enum (ImportanceBased, FIFO, SlidingWindow)
  - [x] 1.11 Add all garrison types to `src/core/platform/container/mod.rs` exports
  - [x] 1.12 Run `cargo build` to verify compilation

- [x] 2.0 Define Application Layer Ports (GarrisonPort Traits)
  - [x] 2.1 Create `src/application/ports/output/garrison_port.rs` file
  - [x] 2.2 Define `GarrisonStats` struct (entry_count, total_tokens, size_bytes)
  - [x] 2.3 Define `GarrisonError` enum with thiserror (StorageError, SerializationError, TokenizationError, NotFound, ConfigurationError)
  - [x] 2.4 Implement Display and Error traits for `GarrisonError` using thiserror macros
  - [x] 2.5 Define `GarrisonPort` trait with async methods (remember, recall_recent, search, forget_all, stats)
  - [x] 2.6 Add `Send + Sync` bounds to `GarrisonPort` trait
  - [x] 2.7 Define `LongTermGarrisonPort` trait extending `GarrisonPort` (remember_with_embedding, search_similar)
  - [x] 2.8 Add comprehensive rustdoc comments to all trait methods
  - [x] 2.9 Add garrison_port to `src/application/ports/output/mod.rs` exports
  - [x] 2.10 Run `cargo build` to verify trait definitions compile

- [x] 3.0 Implement In-Memory Garrison Adapter
  - [x] 3.1 Create `src/infrastructure/adapters/garrison/` directory
  - [x] 3.2 Create `src/infrastructure/adapters/garrison/mod.rs` with module structure
  - [x] 3.3 Create `src/infrastructure/adapters/garrison/in_memory_garrison.rs`
  - [x] 3.4 Define `InMemoryGarrison` struct with `RwLock<VecDeque<GarrisonEntry>>` and `GarrisonConfig`
  - [x] 3.5 Implement `InMemoryGarrison::new(config)` constructor
  - [x] 3.6 Implement `GarrisonPort::remember()` - add entry with write lock
  - [x] 3.7 Implement `GarrisonPort::recall_recent()` - retrieve N most recent with read lock
  - [x] 3.8 Implement `GarrisonPort::search()` - simple substring search
  - [x] 3.9 Implement `GarrisonPort::forget_all()` - clear all entries
  - [x] 3.10 Implement `GarrisonPort::stats()` - calculate statistics
  - [x] 3.11 Implement importance-based eviction logic in private method
  - [x] 3.12 Add comprehensive error handling for all operations
  - [x] 3.13 Add to `src/infrastructure/adapters/mod.rs` exports
  - [x] 3.14 Run `cargo build` to verify InMemoryGarrison compiles

- [ ] 4.0 Implement SQLite Garrison Adapter
  - [ ] 4.1 Add sqlx and sqlite-vss dependencies to `Cargo.toml`
  - [ ] 4.2 Create `migrations/001_create_garrison_tables.sql` with schema from PRD
  - [ ] 4.3 Add indexes (idx_paladin_timestamp, idx_paladin_role) to migration
  - [ ] 4.4 Create virtual table for vector search (garrison_embeddings) in migration
  - [ ] 4.5 Create `src/infrastructure/adapters/garrison/sqlite_garrison.rs`
  - [ ] 4.6 Define `SqliteGarrison` struct with SqlitePool and GarrisonConfig
  - [ ] 4.7 Implement `SqliteGarrison::connect(path)` - create connection pool
  - [ ] 4.8 Implement `SqliteGarrison::initialize()` - run migrations
  - [ ] 4.9 Implement `GarrisonPort::remember()` - INSERT entry into database
  - [ ] 4.10 Implement `GarrisonPort::recall_recent()` - SELECT with LIMIT and ORDER BY
  - [ ] 4.11 Implement `GarrisonPort::search()` - full-text search query
  - [ ] 4.12 Implement `GarrisonPort::forget_all()` - DELETE all entries
  - [ ] 4.13 Implement `GarrisonPort::stats()` - aggregate queries
  - [ ] 4.14 Implement `LongTermGarrisonPort::remember_with_embedding()` - store entry with vector
  - [ ] 4.15 Implement `LongTermGarrisonPort::search_similar()` - vector similarity search using sqlite-vss
  - [ ] 4.16 Add connection pooling configuration
  - [ ] 4.17 Add proper error handling and conversion from sqlx errors
  - [ ] 4.18 Run `cargo build` to verify SqliteGarrison compiles

- [x] 5.0 Implement Token Counting System
  - [x] 5.1 Add tiktoken-rs dependency to `Cargo.toml`
  - [x] 5.2 Create `src/infrastructure/adapters/garrison/token_counter.rs`
  - [x] 5.3 Define `TokenCounter` trait with `count_tokens(&str) -> Result<u32>`
  - [x] 5.4 Define `TiktokenCounter` struct wrapping tiktoken CoreBPE
  - [x] 5.5 Implement `TiktokenCounter::new(model_name)` - initialize tokenizer for model
  - [x] 5.6 Implement `TokenCounter` trait for `TiktokenCounter`
  - [x] 5.7 Add caching logic to avoid redundant tokenization
  - [x] 5.8 Create factory method `TokenCounter::for_model(name)` returning appropriate counter
  - [x] 5.9 Add error handling for unsupported models
  - [x] 5.10 Update `GarrisonEntry` to accept optional token_count in constructor
  - [x] 5.11 Add method to calculate and cache token count on entry
  - [x] 5.12 Run `cargo build` to verify token counter compiles

- [ ] 6.0 Integrate Garrison with Paladin System
  - [ ] 6.1 Read existing `src/application/use_cases/paladin/paladin_builder.rs`
  - [ ] 6.2 Add `garrison: Option<Arc<dyn GarrisonPort>>` field to `PaladinBuilder`
  - [ ] 6.3 Implement `.with_garrison(garrison: Arc<dyn GarrisonPort>)` method
  - [ ] 6.4 Pass garrison to Paladin/service during build
  - [ ] 6.5 Read existing `src/application/use_cases/paladin/paladin_execution_service.rs`
  - [ ] 6.6 Add `garrison: Option<Arc<dyn GarrisonPort>>` field to `PaladinExecutionService`
  - [ ] 6.7 Update constructor to accept garrison parameter
  - [ ] 6.8 Modify `execute()` method to store user input as GarrisonEntry before LLM call
  - [ ] 6.9 Add history retrieval: `garrison.recall_recent(limit)` before prompt building
  - [ ] 6.10 Update prompt building to include conversation history
  - [ ] 6.11 Store LLM response as GarrisonEntry after generation
  - [ ] 6.12 Add logic to check for garrison requirement in multi-turn scenarios
  - [ ] 6.13 Return `PaladinError::GarrisonRequired` when garrison missing for multi-turn
  - [ ] 6.14 Update `PaladinError` enum to include `GarrisonError(GarrisonError)` variant
  - [ ] 6.15 Implement `From<GarrisonError>` for `PaladinError`
  - [ ] 6.16 Run `cargo build` to verify integration compiles

- [ ] 7.0 Implement Configuration and Error Handling
  - [ ] 7.1 Read existing `src/config/application_settings.rs`
  - [ ] 7.2 Define `GarrisonSettings` struct (garrison_type, path, max_entries, max_tokens, tokenizer, eviction_strategy)
  - [ ] 7.3 Add `garrison: GarrisonSettings` field to `ApplicationSettings`
  - [ ] 7.4 Implement Default for `GarrisonSettings` with sensible defaults
  - [ ] 7.5 Update `config.yml` with garrison configuration section from PRD
  - [ ] 7.6 Update `config.test.yml` with test garrison configuration
  - [ ] 7.7 Add validation logic for garrison configuration
  - [ ] 7.8 Ensure all `GarrisonError` variants have clear error messages
  - [ ] 7.9 Add logging statements at key points (error, warn, info, debug levels)
  - [ ] 7.10 Run `cargo build` to verify configuration loads correctly

- [ ] 8.0 Write Unit Tests
  - [ ] 8.1 Create `tests/unit/garrison_entry_test.rs`
  - [ ] 8.2 Write test: `test_garrison_entry_creation` - verify all fields populated
  - [ ] 8.3 Write test: `test_garrison_entry_validation` - test required field validation
  - [ ] 8.4 Write test: `test_garrison_entry_serialization` - JSON roundtrip
  - [ ] 8.5 Write test: `test_conversation_role_enum` - all variants serialize correctly
  - [ ] 8.6 Create `tests/unit/conversation_history_test.rs`
  - [ ] 8.7 Write test: `test_conversation_history_windowing` - max_entries enforcement
  - [ ] 8.8 Write test: `test_token_limit_enforcement` - max_tokens enforcement
  - [ ] 8.9 Write test: `test_importance_based_eviction` - system prompts preserved
  - [ ] 8.10 Write test: `test_fifo_eviction` - oldest removed first
  - [ ] 8.11 Write test: `test_empty_history_operations` - edge cases
  - [ ] 8.12 Create `tests/unit/in_memory_garrison_test.rs`
  - [ ] 8.13 Write test: `test_remember_and_recall` - basic storage and retrieval
  - [ ] 8.14 Write test: `test_search_functionality` - substring search works
  - [ ] 8.15 Write test: `test_forget_all` - clear operation
  - [ ] 8.16 Write test: `test_garrison_stats` - statistics calculation
  - [ ] 8.17 Write test: `test_concurrent_access` - thread-safety with RwLock
  - [ ] 8.18 Write test: `test_token_counter` - tiktoken counts accurately
  - [ ] 8.19 Run `cargo test` and ensure ≥80% coverage for garrison module
  - [ ] 8.20 Fix any failing tests

- [ ] 9.0 Write Integration Tests
  - [ ] 9.1 Create `tests/integration/sqlite_garrison_test.rs`
  - [ ] 9.2 Write test: `test_sqlite_garrison_persistence` - data survives connection close
  - [ ] 9.3 Write test: `test_sqlite_garrison_crud_operations` - full CRUD workflow
  - [ ] 9.4 Write test: `test_sqlite_migration_execution` - migrations run correctly
  - [ ] 9.5 Write test: `test_sqlite_connection_pooling` - concurrent connections work
  - [ ] 9.6 Write test: `test_vector_search` - semantic search returns relevant results
  - [ ] 9.7 Create `tests/integration/paladin_with_garrison_test.rs`
  - [ ] 9.8 Write test: `test_paladin_multi_turn_conversation` - context maintained across turns
  - [ ] 9.9 Write test: `test_paladin_without_garrison_single_turn` - works without garrison
  - [ ] 9.10 Write test: `test_paladin_without_garrison_multi_turn_fails` - returns GarrisonRequired error
  - [ ] 9.11 Write test: `test_token_limit_enforcement_in_conversation` - windowing works in real scenario
  - [ ] 9.12 Create `tests/functional/garrison_lifecycle_test.rs`
  - [ ] 9.13 Write test: `test_garrison_recovery_after_restart` - full lifecycle with persistence
  - [ ] 9.14 Write test: `test_large_conversation_performance` - benchmark with 1000 entries
  - [ ] 9.15 Run `cargo test --test [test_name]` for each integration test
  - [ ] 9.16 Verify all integration tests pass
  - [ ] 9.17 Run `cargo test` to verify entire test suite passes

- [ ] 10.0 Create Documentation and Examples
  - [ ] 10.1 Add rustdoc comments to all public items in garrison.rs
  - [ ] 10.2 Add rustdoc comments to all trait methods in garrison_port.rs
  - [ ] 10.3 Add rustdoc comments to InMemoryGarrison implementation
  - [ ] 10.4 Add rustdoc comments to SqliteGarrison implementation
  - [ ] 10.5 Create `examples/garrison_in_memory.rs` - basic in-memory usage (from PRD)
  - [ ] 10.6 Create `examples/garrison_persistent.rs` - SQLite persistence example (from PRD)
  - [ ] 10.7 Create `examples/garrison_semantic_search.rs` - vector search example (from PRD)
  - [ ] 10.8 Test all examples: `cargo run --example garrison_in_memory`
  - [ ] 10.9 Test all examples: `cargo run --example garrison_persistent`
  - [ ] 10.10 Test all examples: `cargo run --example garrison_semantic_search`
  - [ ] 10.11 Create `docs/GARRISON.md` with architecture overview
  - [ ] 10.12 Document configuration options in GARRISON.md
  - [ ] 10.13 Document common usage patterns in GARRISON.md
  - [ ] 10.14 Add troubleshooting section to GARRISON.md
  - [ ] 10.15 Update main README.md to reference Garrison system
  - [ ] 10.16 Run `cargo doc --open` to verify documentation builds correctly
  - [ ] 10.17 Review generated docs for clarity and completeness

- [ ] 11.0 Final Validation and Cleanup
  - [ ] 11.1 Run `cargo fmt` to format all code
  - [ ] 11.2 Run `cargo clippy` and fix all warnings
  - [ ] 11.3 Run `cargo test` and ensure all tests pass
  - [ ] 11.4 Run `cargo build --release` to verify release build
  - [ ] 11.5 Verify test coverage ≥ 80% using `cargo tarpaulin` or similar
  - [ ] 11.6 Review all acceptance criteria from PRD - ensure all met
  - [ ] 11.7 Create pull request with comprehensive description
  - [ ] 11.8 Request code review
  - [ ] 11.9 Address review feedback
  - [ ] 11.10 Merge to develop branch after approval

---

**Status:** Complete task breakdown ready for implementation.
