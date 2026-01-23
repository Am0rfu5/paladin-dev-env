# Product Requirements Document: Garrison Memory System

**Epic:** Epic 2  
**Priority:** High  
**Effort:** 2-3 weeks  
**Dependencies:** Epic 1 (Paladin Domain Foundation)  
**Status:** Draft

---

## Introduction/Overview

The Garrison Memory System provides Paladins with the ability to maintain conversation context and persist knowledge across sessions. Named after medieval military garrisons (fortified storage facilities), this system acts as the memory infrastructure enabling Paladins to remember previous interactions, maintain coherent multi-turn conversations, and access relevant historical context.

**Problem Statement:** Without memory, Paladins cannot maintain context across multiple interactions, making them unsuitable for conversational workflows, complex multi-step reasoning tasks, or scenarios requiring continuity across sessions.

**Solution:** Implement a flexible memory system supporting both short-term (in-memory) and long-term (persistent) storage with intelligent windowing, token management, and semantic search capabilities.

---

## Goals

1. **Enable Multi-Turn Conversations:** Paladins maintain coherent context across multiple user interactions
2. **Token Efficiency:** Respect LLM context window limits through intelligent memory windowing
3. **Persistent Sessions:** Allow conversation state to be saved and resumed across application restarts
4. **Semantic Retrieval:** Enable Paladins to find relevant past information using vector similarity search
5. **Flexible Integration:** Make Garrison optional for single-turn operations, required for multi-turn scenarios
6. **Production-Ready Performance:** Support both fast in-memory and durable persistent storage backends

---

## User Stories

### Story 1: Maintaining Conversation Context
**As a developer**, I want Paladins to remember conversation history so that they can provide contextually relevant responses across multiple turns.

**Acceptance Criteria:**
- Paladin can access previous messages in the current conversation
- Conversation history is automatically included in LLM prompts
- Memory window prevents context overflow

### Story 2: Managing Token Limits
**As a developer**, I want to configure memory window size based on token counts so that LLM context limits are respected and API costs are controlled.

**Acceptance Criteria:**
- Token counts are calculated using LLM-specific tokenizers
- Oldest messages are intelligently evicted when limits are reached
- System prompts and recent messages are prioritized

### Story 3: Persisting Memory
**As a developer**, I want to persist Paladin memory to disk so that conversations can be resumed after application restart.

**Acceptance Criteria:**
- Conversation history can be saved to SQLite database
- Garrison can be restored with full conversation context
- No data loss occurs during persistence operations

### Story 4: Semantic Memory Search
**As a developer**, I want to search Garrison memory using semantic similarity so that Paladins can retrieve relevant past information even when exact keywords don't match.

**Acceptance Criteria:**
- Garrison entries can be stored with vector embeddings
- Semantic search returns entries ranked by relevance
- Search results can be limited to top-K most similar entries

### Story 5: Optional Memory Integration
**As a developer**, I want to create Paladins without Garrison for simple single-turn tasks so that I'm not forced to use memory when it's not needed.

**Acceptance Criteria:**
- Paladins can be built without Garrison attachment
- Single-turn execution works without memory
- Multi-turn conversations require Garrison and fail gracefully if missing

---

## Functional Requirements

### FR1: Garrison Entry Management

1.1. The system **must** create a `GarrisonEntry` struct containing:
   - Unique identifier (UUID)
   - Conversation role (System, User, Assistant, Tool)
   - Content (message text)
   - Timestamp (UTC)
   - Metadata (extensible key-value map)
   - Token count (optional, for window management)

1.2. The system **must** validate that all required fields are populated before storing an entry.

1.3. The system **must** support serialization/deserialization of entries for persistence.

### FR2: Conversation History Windowing

2.1. The system **must** maintain a `ConversationHistory` that:
   - Stores entries in chronological order
   - Enforces maximum entry count limit
   - Enforces maximum token count limit (when configured)
   - Provides efficient recent-N retrieval

2.2. The system **must** implement importance-based eviction when limits are reached:
   - Always preserve system prompts (role: System)
   - Always preserve most recent N messages
   - Evict oldest user/assistant messages from the middle of history
   - Never evict if within configured limits

2.3. The system **must** calculate token counts using LLM-specific tokenizers:
   - Integrate tiktoken library for OpenAI models
   - Support pluggable tokenizers for other providers
   - Cache token counts to avoid redundant calculations

### FR3: GarrisonPort Trait

3.1. The system **must** define an async `GarrisonPort` trait with methods:
   - `remember(entry)` - Add entry to memory
   - `recall_recent(limit)` - Retrieve N most recent entries
   - `search(query, limit)` - Text-based search
   - `forget_all()` - Clear all memory
   - `stats()` - Return memory statistics (count, tokens, size)

3.2. All trait methods **must** be `Send + Sync` for async compatibility.

3.3. All trait methods **must** return `Result<T, GarrisonError>` for error handling.

### FR4: LongTermGarrisonPort Trait

4.1. The system **must** extend `GarrisonPort` with a `LongTermGarrisonPort` trait adding:
   - `remember_with_embedding(entry, embedding)` - Store entry with vector
   - `search_similar(embedding, limit)` - Semantic similarity search

4.2. The system **must** support embeddings as `Vec<f32>` (standard ML vector format).

4.3. Semantic search **must** return results ranked by cosine similarity.

### FR5: In-Memory Garrison Implementation

5.1. The system **must** implement an `InMemoryGarrison` adapter that:
   - Stores entries in a thread-safe `RwLock<VecDeque<GarrisonEntry>>`
   - Supports all `GarrisonPort` operations
   - Provides O(1) append and O(N) search performance
   - Loses all data on application shutdown

5.2. `InMemoryGarrison` **must** be the default implementation for quick prototyping.

### FR6: SQLite Garrison Implementation

6.1. The system **must** implement a `SqliteGarrison` adapter that:
   - Uses SQLite database for persistent storage
   - Stores entries in a `garrison_entries` table
   - Supports all `GarrisonPort` operations
   - Persists data across application restarts

6.2. The database schema **must** include:
   ```sql
   CREATE TABLE garrison_entries (
       id TEXT PRIMARY KEY,
       paladin_id TEXT NOT NULL,
       role TEXT NOT NULL,
       content TEXT NOT NULL,
       timestamp INTEGER NOT NULL,
       metadata TEXT,
       token_count INTEGER,
       embedding BLOB
   );
   CREATE INDEX idx_paladin_timestamp ON garrison_entries(paladin_id, timestamp);
   ```

6.3. `SqliteGarrison` **must** implement connection pooling using sqlx.

6.4. `SqliteGarrison` **must** support vector search using SQLite-vss extension (for embeddings).

### FR7: Paladin-Garrison Integration

7.1. `PaladinBuilder` **must** provide a `with_garrison(port)` method to attach memory.

7.2. Paladins without Garrison attachment **must** execute successfully for single-turn requests.

7.3. Paladins without Garrison attachment **must** return `PaladinError::GarrisonRequired` when attempting multi-turn conversations.

7.4. `PaladinExecutionService` **must** automatically:
   - Store user input as a Garrison entry
   - Retrieve recent conversation history before LLM calls
   - Inject history into prompt construction
   - Store LLM responses as Garrison entries

### FR8: Configuration

8.1. The system **must** support configuration via `config.yml`:
   ```yaml
   garrison:
     type: "sqlite"  # or "in_memory"
     path: "./garrison.db"
     max_entries: 1000
     max_tokens: 4000
     tokenizer: "tiktoken"  # for OpenAI models
     eviction_strategy: "importance_based"
   ```

8.2. Configuration **must** be loaded via `ApplicationSettings`.

8.3. Invalid configuration **must** result in a `GarrisonError::Configuration` error.

### FR9: Error Handling

9.1. The system **must** define a `GarrisonError` enum:
   - `StorageError(String)` - Database/file errors
   - `SerializationError(String)` - JSON/binary serialization failures
   - `TokenizationError(String)` - Token counting failures
   - `NotFound(String)` - Entry not found
   - `ConfigurationError(String)` - Invalid configuration

9.2. All errors **must** implement `std::error::Error` and `Display` via thiserror.

### FR10: Testing & Quality

10.1. Unit test coverage **must** be ≥ 80%.

10.2. The system **must** include integration tests:
   - `test_sqlite_garrison_persistence` - Verify data survives restart
   - `test_paladin_with_garrison_context` - Verify multi-turn conversations
   - `test_garrison_recovery_after_restart` - Verify session resumption
   - `test_token_limit_enforcement` - Verify windowing works correctly
   - `test_semantic_search_accuracy` - Verify vector search returns relevant results

10.3. All public APIs **must** have rustdoc documentation.

10.4. Code **must** pass `cargo clippy` with no warnings.

---

## Non-Goals (Out of Scope)

The following are explicitly **not** included in Epic 2:

1. **Distributed Memory:** Garrison will not support distributed/shared memory across multiple Paladin instances
2. **Advanced Compression:** No special compression algorithms for stored content
3. **Memory Export/Import:** No built-in functionality to export/import memory to/from external formats
4. **Web Interface:** No UI for browsing/managing Garrison contents
5. **Cross-Paladin Memory Sharing:** Each Paladin has isolated memory; no shared knowledge base
6. **Automatic Summarization:** Garrison will not automatically summarize old conversations
7. **Multi-Modal Memory:** Text only; no support for images, audio, or other media in Garrison
8. **Cloud Sync:** No automatic synchronization to cloud storage services

---

## Design Considerations

### Architecture

The Garrison system follows Paladin's three-layer Hexagonal Architecture:

1. **Core Layer** (`src/core/platform/container/garrison.rs`)
   - Domain entities: `GarrisonEntry`, `ConversationHistory`, `GarrisonType`
   - Pure business logic, no external dependencies

2. **Application Layer** (`src/application/ports/output/garrison_port.rs`)
   - Port traits: `GarrisonPort`, `LongTermGarrisonPort`
   - No implementation details

3. **Infrastructure Layer** (`src/infrastructure/adapters/garrison/`)
   - Adapters: `InMemoryGarrison`, `SqliteGarrison`
   - External dependencies: sqlx, tiktoken, serde

### Data Flow

```
User Input
    ↓
PaladinExecutionService.execute()
    ↓
garrison.remember(user_entry)  ← Store input
    ↓
history ← garrison.recall_recent(limit)  ← Retrieve context
    ↓
prompt ← build_prompt_with_history(input, history)
    ↓
response ← llm_port.generate(prompt)
    ↓
garrison.remember(assistant_entry)  ← Store response
    ↓
Return response to user
```

### Token Counting Strategy

- Use `tiktoken` library (OpenAI's official tokenizer)
- Cache token counts in `GarrisonEntry.token_count`
- Recalculate only when entry content changes
- Provide abstraction for future non-OpenAI tokenizers

### Eviction Algorithm (Importance-Based)

```
When max_tokens or max_entries exceeded:
1. Identify protected entries:
   - All entries with role == System
   - Most recent N entries (configurable, default 10)
2. Calculate eviction candidates:
   - All entries NOT in protected set
3. Sort candidates by timestamp (oldest first)
4. Remove oldest candidates until within limits
5. If still exceeding limits after removing all candidates:
   - Log warning
   - Proceed anyway (protect critical context)
```

---

## Technical Considerations

### Dependencies

New Cargo dependencies required:

```toml
[dependencies]
# Tokenization
tiktoken-rs = "0.5"  # OpenAI tokenizer

# Vector storage (for semantic search)
sqlite-vss = "0.1"  # SQLite vector similarity search extension

# Existing dependencies
sqlx = { version = "0.7", features = ["sqlite", "runtime-tokio-native-tls"] }
tokio = { version = "1", features = ["full"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
uuid = { version = "1", features = ["v4", "serde"] }
chrono = { version = "0.4", features = ["serde"] }
```

### Database Schema

```sql
-- Main entries table
CREATE TABLE garrison_entries (
    id TEXT PRIMARY KEY,
    paladin_id TEXT NOT NULL,
    role TEXT NOT NULL CHECK(role IN ('System', 'User', 'Assistant', 'Tool')),
    content TEXT NOT NULL,
    timestamp INTEGER NOT NULL,
    metadata TEXT,  -- JSON
    token_count INTEGER,
    embedding BLOB,  -- Vector for semantic search
    created_at INTEGER DEFAULT (strftime('%s', 'now'))
);

-- Indexes for performance
CREATE INDEX idx_paladin_timestamp ON garrison_entries(paladin_id, timestamp DESC);
CREATE INDEX idx_paladin_role ON garrison_entries(paladin_id, role);

-- Virtual table for vector search (using sqlite-vss)
CREATE VIRTUAL TABLE garrison_embeddings USING vss0(
    embedding(768)  -- Dimension depends on embedding model
);
```

### Integration Points

1. **Epic 1 Integration:**
   - `PaladinBuilder` extended with `.with_garrison()` method
   - `PaladinExecutionService` modified to use Garrison when available
   - `PaladinConfig` extended with memory window settings

2. **LLM Port Integration:**
   - Prompt construction includes conversation history
   - Token counting aligns with LLM provider's tokenizer

3. **Configuration System:**
   - `ApplicationSettings` loads Garrison configuration
   - Environment variables override defaults

### Error Handling Strategy

All Garrison operations return `Result<T, GarrisonError>`. Errors propagate to `PaladinError::GarrisonError(GarrisonError)` at the service layer.

Example:
```rust
pub async fn execute(&self, input: &str) -> Result<PaladinResult, PaladinError> {
    if let Some(garrison) = &self.garrison {
        garrison.remember(user_entry).await
            .map_err(PaladinError::from)?;  // Convert GarrisonError → PaladinError
    }
    // ... rest of execution
}
```

### Testing Strategy

1. **Unit Tests:** Test individual components in isolation
   - `GarrisonEntry` creation and validation
   - `ConversationHistory` windowing logic
   - Token counting accuracy
   - Eviction algorithm correctness

2. **Integration Tests:** Test adapter implementations
   - SQLite persistence and retrieval
   - In-memory storage operations
   - Database migrations
   - Connection pooling

3. **Functional Tests:** Test end-to-end scenarios
   - Multi-turn Paladin conversations
   - Session resumption after restart
   - Token limit enforcement in real conversations
   - Semantic search with actual embeddings

4. **Performance Tests:** Benchmark critical paths
   - Garrison write throughput
   - Search query latency
   - Memory footprint with large histories

---

## Success Metrics

### Functional Success
- [ ] 100% of Epic 2 acceptance criteria met
- [ ] Unit test coverage ≥ 80%
- [ ] All integration tests passing
- [ ] Zero critical bugs in code review

### Performance Success
- [ ] Garrison write latency < 10ms (in-memory) / < 50ms (SQLite)
- [ ] Garrison read latency < 5ms (in-memory) / < 20ms (SQLite)
- [ ] Semantic search query < 100ms for 1000 entries
- [ ] Memory footprint < 10MB for 1000 entries

### Quality Success
- [ ] Code passes `cargo clippy` with zero warnings
- [ ] All public APIs documented with rustdoc
- [ ] Examples demonstrate all Garrison features
- [ ] Integration with Epic 1 verified by functional tests

### Developer Experience
- [ ] Junior developer can integrate Garrison in < 30 minutes
- [ ] Clear error messages guide debugging
- [ ] Configuration is self-documenting
- [ ] Example code covers common use cases

---

## Open Questions

### Q1: Embedding Model Selection
**Question:** Which embedding model should be used for semantic search?  
**Options:**
- OpenAI text-embedding-3-small (1536 dimensions, requires API)
- Sentence-transformers (open-source, local inference)
- Defer to user configuration

**Impact:** Affects embedding storage requirements and search quality.  
**Decision Needed By:** Before implementing `LongTermGarrisonPort`

### Q2: Migration Strategy
**Question:** How should we handle Garrison schema migrations when structure changes?  
**Options:**
- Use sqlx migrations (automatic)
- Manual migration scripts
- No backward compatibility (break on schema change)

**Impact:** Affects upgrade experience for existing deployments.  
**Decision Needed By:** Before SQLite adapter implementation

### Q3: Concurrent Access Patterns
**Question:** Should multiple Paladins be able to access the same Garrison concurrently?  
**Options:**
- Single-writer, multiple readers (RwLock pattern)
- Fully concurrent (optimistic locking)
- Isolated per-Paladin (no sharing)

**Impact:** Affects locking strategy and complexity.  
**Decision Needed By:** Before finalizing `GarrisonPort` trait

### Q4: Memory Privacy & Security
**Question:** Should Garrison entries be encrypted at rest?  
**Options:**
- No encryption (MVP)
- Optional encryption via configuration
- Always encrypted (GDPR compliance)

**Impact:** Affects storage format and performance.  
**Decision Needed By:** Before release to production environments

### Q5: Observability
**Question:** What level of observability should Garrison provide?  
**Options:**
- Basic logging only
- Metrics (entry count, token usage, query latency)
- Full distributed tracing integration

**Impact:** Affects debugging and production monitoring capabilities.  
**Decision Needed By:** Before integration testing

---

## Appendix: Example Usage

### Example 1: Paladin with In-Memory Garrison

```rust
use paladin::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create in-memory garrison
    let garrison = InMemoryGarrison::new(GarrisonConfig {
        max_entries: 100,
        max_tokens: Some(4000),
        eviction_strategy: EvictionStrategy::ImportanceBased,
    });

    // Build Paladin with memory
    let paladin = PaladinBuilder::new(openai_port)
        .name("MemoryPaladin")
        .system_prompt("You are a helpful assistant with memory.")
        .with_garrison(Arc::new(garrison))
        .build()?;

    // Multi-turn conversation
    let response1 = paladin.execute("My name is Alice.").await?;
    println!("{}", response1.content);  // "Nice to meet you, Alice!"

    let response2 = paladin.execute("What's my name?").await?;
    println!("{}", response2.content);  // "Your name is Alice."

    Ok(())
}
```

### Example 2: Paladin with Persistent SQLite Garrison

```rust
use paladin::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create persistent garrison
    let garrison = SqliteGarrison::connect("./my_conversations.db").await?;

    let paladin = PaladinBuilder::new(openai_port)
        .name("PersistentPaladin")
        .with_garrison(Arc::new(garrison))
        .build()?;

    // Conversation persists across restarts
    paladin.execute("Remember: my favorite color is blue.").await?;

    // ... application restarts ...

    // Load garrison again - history preserved
    let garrison = SqliteGarrison::connect("./my_conversations.db").await?;
    let paladin = PaladinBuilder::new(openai_port)
        .with_garrison(Arc::new(garrison))
        .build()?;

    let response = paladin.execute("What's my favorite color?").await?;
    println!("{}", response.content);  // "Your favorite color is blue."

    Ok(())
}
```

### Example 3: Semantic Search in Garrison

```rust
use paladin::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let garrison = SqliteGarrison::connect("./knowledge_base.db").await?;

    // Store entries with embeddings
    let embedding = embedding_model.encode("Machine learning fundamentals").await?;
    garrison.remember_with_embedding(
        GarrisonEntry::new(ConversationRole::User, "What is machine learning?"),
        embedding
    ).await?;

    // Later: semantic search
    let query_embedding = embedding_model.encode("AI basics").await?;
    let similar_entries = garrison.search_similar(query_embedding, 5).await?;

    for entry in similar_entries {
        println!("Found: {}", entry.content);
    }

    Ok(())
}
```

---

**Document Version:** 1.0  
**Last Updated:** January 22, 2026  
**Author:** GitHub Copilot  
**Reviewers:** TBD
