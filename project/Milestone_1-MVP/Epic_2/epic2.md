## Epic 2: Garrison Memory System

### Overview

**Priority:** High  
**Effort:** 2-3 weeks  
**Dependencies:** Epic 1  
**Team:** 1-2 developers

**Objective:** Implement the Garrison memory system enabling Paladins to maintain conversation context and persist knowledge across sessions.

### User Stories

1. **As a developer**, I want Paladins to remember conversation history so that context is maintained.
2. **As a developer**, I want to configure memory window size so that token limits are respected.
3. **As a developer**, I want to persist Paladin memory so that sessions can be resumed.
4. **As a developer**, I want to search memory by content so that relevant context can be retrieved.

### Technical Design

#### Domain Layer

**garrison.rs - Memory Domain**

```rust
/// A single memory entry in the Garrison
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GarrisonEntry {
    pub id: Uuid,
    pub role: ConversationRole,
    pub content: String,
    pub timestamp: DateTime<Utc>,
    pub metadata: HashMap<String, Value>,
    pub token_count: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConversationRole {
    System,
    User,
    Assistant,
    Tool,
}

/// Conversation history with windowing support
#[derive(Debug, Clone)]
pub struct ConversationHistory {
    entries: VecDeque<GarrisonEntry>,
    max_entries: usize,
    max_tokens: Option<u32>,
}

/// Memory type classification
#[derive(Debug, Clone)]
pub enum GarrisonType {
    /// Active conversation context
    ShortTerm,
    /// Persisted knowledge
    LongTerm,
    /// Specific event memories
    Episodic,
}
```

#### Application Layer

**ports/output/garrison_port.rs**

```rust
/// Port for memory operations
#[async_trait]
pub trait GarrisonPort: Send + Sync {
    /// Add entry to memory
    async fn remember(&self, entry: GarrisonEntry) -> Result<(), GarrisonError>;
    
    /// Retrieve recent entries
    async fn recall_recent(&self, limit: usize) -> Result<Vec<GarrisonEntry>, GarrisonError>;
    
    /// Search memory by content
    async fn search(&self, query: &str, limit: usize) -> Result<Vec<GarrisonEntry>, GarrisonError>;
    
    /// Clear all memory
    async fn forget_all(&self) -> Result<(), GarrisonError>;
    
    /// Get memory statistics
    async fn stats(&self) -> Result<GarrisonStats, GarrisonError>;
}

/// Extended port for long-term memory with vector search
#[async_trait]
pub trait LongTermGarrisonPort: GarrisonPort {
    /// Add entry with embedding
    async fn remember_with_embedding(&self, entry: GarrisonEntry, embedding: Vec<f32>) 
        -> Result<(), GarrisonError>;
    
    /// Semantic similarity search
    async fn search_similar(&self, embedding: Vec<f32>, limit: usize) 
        -> Result<Vec<GarrisonEntry>, GarrisonError>;
}
```

#### Infrastructure Layer

**adapters/garrison/in_memory_garrison.rs**

```rust
/// In-memory garrison for short-term storage
pub struct InMemoryGarrison {
    entries: RwLock<VecDeque<GarrisonEntry>>,
    config: GarrisonConfig,
}
```

**adapters/garrison/sqlite_garrison.rs**

```rust
/// SQLite-backed garrison for persistent storage
pub struct SqliteGarrison {
    pool: SqlitePool,
    config: GarrisonConfig,
}
```

### Test Requirements

#### Unit Tests

- `test_garrison_entry_creation`
- `test_conversation_history_windowing`
- `test_token_limit_enforcement`
- `test_memory_search_accuracy`
- `test_garrison_serialization`

#### Integration Tests

- `test_sqlite_garrison_persistence`
- `test_paladin_with_garrison_context`
- `test_garrison_recovery_after_restart`

### Acceptance Criteria

- [ ] Paladins maintain conversation context across multiple interactions
- [ ] Memory window limits prevent context overflow
- [ ] Memory can be persisted to SQLite and restored
- [ ] Search returns relevant entries based on content
- [ ] Unit test coverage ≥ 80%

### Definition of Done

- [ ] All tests passing
- [ ] Code reviewed and approved
- [ ] Documentation complete
- [ ] Integration with Epic 1 verified

---