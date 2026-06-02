# Garrison Memory System

The Garrison is Paladin's memory and context management system, enabling AI agents to maintain conversation history, search previous interactions, and persist knowledge across sessions.

## Table of Contents

- [Overview](#overview)
- [Architecture](#architecture)
- [Configuration](#configuration)
- [Usage Patterns](#usage-patterns)
- [Implementations](#implementations)
- [Troubleshooting](#troubleshooting)
- [Performance Considerations](#performance-considerations)

## Overview

### What is a Garrison?

In medieval times, a garrison was a fortified location where troops stored supplies and maintained strategic resources. Similarly, Paladin's Garrison system stores and manages conversation context—the essential "supplies" an AI agent needs to maintain coherent, contextual interactions.

### Key Features

- **Conversation History**: Store and retrieve user-assistant interactions
- **Automatic Windowing**: Manage context size with configurable eviction strategies
- **Full-Text Search**: Find relevant conversations using keyword or phrase queries
- **Persistence**: Optional SQLite storage for durability across restarts
- **Multi-Paladin Isolation**: Multiple agents can share a database with isolated data
- **Extensible**: Pluggable architecture supports custom storage backends

## Architecture

The Garrison system follows **Hexagonal Architecture** (Ports & Adapters):

```
┌─────────────────────────────────────────────────────────────┐
│                     Application Layer                        │
│  ┌───────────────────────────────────────────────────────┐  │
│  │           GarrisonPort (Interface)                    │  │
│  │  - remember(entry)                                    │  │
│  │  - recall_recent(limit)                               │  │
│  │  - search(query, limit)                               │  │
│  │  - forget_all()                                       │  │
│  │  - stats()                                            │  │
│  └───────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────┘
                          │
        ┌─────────────────┴─────────────────┐
        │                                   │
┌───────▼─────────┐               ┌─────────▼────────┐
│ InMemoryGarrison│               │ SqliteGarrison   │
│  (Ephemeral)    │               │  (Persistent)    │
│                 │               │                  │
│  Storage:       │               │  Storage:        │
│  - VecDeque     │               │  - SQLite DB     │
│  - RwLock       │               │  - Connection    │
│                 │               │    Pool          │
│  Use Cases:     │               │  - FTS5 Index    │
│  - Testing      │               │                  │
│  - Dev          │               │  Use Cases:      │
│  - Short-lived  │               │  - Production    │
│    sessions     │               │  - Multi-session │
│                 │               │  - Analytics     │
└─────────────────┘               └──────────────────┘
```

### Core Components

#### Domain Layer (`src/core/platform/container/garrison.rs`)
- **GarrisonEntry**: Individual conversation message
- **ConversationRole**: System, User, Assistant, Tool
- **GarrisonConfig**: Windowing and eviction configuration
- **EvictionStrategy**: FIFO, ImportanceBased, SlidingWindow

#### Application Layer (`src/application/ports/output/garrison_port.rs`)
- **GarrisonPort**: Core interface for memory operations
- **LongTermGarrisonPort**: Extended interface for vector search (future)
- **GarrisonStats**: Statistics and metrics
- **GarrisonError**: Comprehensive error types

#### Infrastructure Layer (`src/infrastructure/adapters/garrison/`)
- **InMemoryGarrison**: Fast, ephemeral implementation
- **SqliteGarrison**: Persistent, production-ready implementation

## Configuration

### GarrisonConfig

```rust
use paladin::core::platform::container::garrison::{
    GarrisonConfig, EvictionStrategy
};

// Default configuration
let config = GarrisonConfig::default();
// max_entries: 100
// max_tokens: Some(4000)
// eviction_strategy: ImportanceBased
// preserve_recent_count: 10

// Custom configuration
let config = GarrisonConfig::new(50, Some(2000))
    .with_eviction_strategy(EvictionStrategy::SlidingWindow)
    .with_preserve_recent(5);
```

### Configuration Options

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `max_entries` | `usize` | 100 | Maximum number of conversation entries to store |
| `max_tokens` | `Option<u32>` | Some(4000) | Token limit across all entries (None = unlimited) |
| `eviction_strategy` | `EvictionStrategy` | ImportanceBased | How to remove old entries when limits are reached |
| `preserve_recent_count` | `usize` | 10 | Minimum recent entries to always keep |

### Eviction Strategies

#### FIFO (First In, First Out)
```rust
.with_eviction_strategy(EvictionStrategy::FIFO)
```
- **Behavior**: Remove the oldest entry when limits are exceeded
- **Use Case**: Simple, predictable behavior for chat applications
- **Pros**: Consistent, easy to understand
- **Cons**: May lose important context like system prompts

#### ImportanceBased
```rust
.with_eviction_strategy(EvictionStrategy::ImportanceBased)
```
- **Behavior**: Preserve system prompts and recent messages, evict middle entries
- **Use Case**: Multi-turn conversations where instructions matter
- **Pros**: Maintains critical context (system prompt) and recent flow
- **Cons**: More complex logic

#### SlidingWindow
```rust
.with_eviction_strategy(EvictionStrategy::SlidingWindow)
```
- **Behavior**: Always keep the N most recent entries
- **Use Case**: Short-term context without historical baggage
- **Pros**: Predictable memory usage, fresh context
- **Cons**: Loses all historical context

## Usage Patterns

### Pattern 1: Simple In-Memory Conversation

```rust
use paladin::infrastructure::adapters::garrison::InMemoryGarrison;
use paladin::paladin_ports::output::garrison_port::GarrisonPort;
use paladin::core::platform::container::garrison::{
    GarrisonConfig, GarrisonEntry, ConversationRole
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = GarrisonConfig::default();
    let garrison = InMemoryGarrison::new(config);

    // Store system prompt
    let system = GarrisonEntry::new(
        ConversationRole::System,
        "You are a helpful assistant.".into()
    );
    garrison.remember(system).await?;

    // Store conversation
    let user_msg = GarrisonEntry::new(
        ConversationRole::User,
        "What is Rust?".into()
    );
    garrison.remember(user_msg).await?;

    let assistant_msg = GarrisonEntry::new(
        ConversationRole::Assistant,
        "Rust is a systems programming language...".into()
    );
    garrison.remember(assistant_msg).await?;

    // Retrieve recent context
    let recent = garrison.recall_recent(10).await?;
    println!("Context has {} entries", recent.len());

    Ok(())
}
```

### Pattern 2: Persistent Conversation with SQLite

```rust
use paladin::infrastructure::adapters::garrison::sqlite_garrison::SqliteGarrison;
use paladin::paladin_ports::output::garrison_port::GarrisonPort;
use paladin::core::platform::container::garrison::GarrisonConfig;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = GarrisonConfig::default();

    // Connect to database (creates if doesn't exist)
    let garrison = SqliteGarrison::connect(
        "./data/garrison.db",
        config,
        "assistant-001"  // Unique paladin ID
    ).await?;

    // Data persists across restarts!
    let previous_history = garrison.recall_recent(100).await?;
    println!("Loaded {} previous entries", previous_history.len());

    // ... store new entries ...

    Ok(())
}
```

### Pattern 3: Context-Aware Search

```rust
// Search for specific topics in conversation history
let results = garrison.search("error handling", 10).await?;

for entry in results {
    println!("[{}] {}",
        match entry.role {
            ConversationRole::User => "User",
            ConversationRole::Assistant => "Assistant",
            _ => "Other",
        },
        entry.content
    );
}

// Phrase search (exact match) for SQLite
let exact_results = garrison.search("\"memory safety\"", 5).await?;
```

### Pattern 4: Integrating with PaladinExecutionService

```rust
use paladin::application::services::paladin::{
    PaladinBuilder, PaladinExecutionService, CircuitBreaker
};
use std::sync::Arc;

// Create garrison
let garrison = Arc::new(SqliteGarrison::connect(
    "./garrison.db",
    config,
    "my-paladin"
).await?);

// Create execution service with garrison
let circuit_breaker = Arc::new(CircuitBreaker::new(3, 2, 30000));
let service = PaladinExecutionService::new(
    llm_port,
    circuit_breaker,
    Some(garrison.clone())  // Enable memory!
);

// Build paladin
let paladin = PaladinBuilder::new(llm_port)
    .name("Assistant")
    .system_prompt("You are helpful.")
    .with_garrison(garrison)
    .build()?;

// Execute - conversation history is automatically managed
let result = service.execute(&paladin, "Hello!").await?;
```

### Pattern 5: Manual Context Window Management

```rust
use paladin::core::platform::container::garrison::ConversationHistory;

// For advanced use cases
let mut history = ConversationHistory::new(config);

history.add(system_entry);
history.add(user_entry);
history.add(assistant_entry);

// Automatic eviction when limits reached
let context_for_llm = history.to_entries();
```

## Implementations

### InMemoryGarrison

**When to use:**
- Development and testing
- Short-lived sessions (< 1 hour)
- Prototyping
- When persistence isn't needed

**Characteristics:**
- **Storage**: In-process memory (VecDeque + RwLock)
- **Performance**: Fastest (microsecond operations)
- **Persistence**: None (data lost on shutdown)
- **Concurrency**: Thread-safe read-write locking
- **Search**: O(N) substring matching

**Example:**
```rust
let garrison = InMemoryGarrison::new(GarrisonConfig::default());
```

### SqliteGarrison

**When to use:**
- Production deployments
- Multi-session conversations
- When you need data recovery
- Analytics and conversation history
- Multiple paladins sharing infrastructure

**Characteristics:**
- **Storage**: SQLite database file
- **Performance**: Fast (connection pooling, indexed searches)
- **Persistence**: Durable across restarts
- **Concurrency**: Connection pool (up to 5 concurrent)
- **Search**: FTS5 full-text search (very fast)
- **Isolation**: Per-paladin data isolation via `paladin_id`

**Example:**
```rust
let garrison = SqliteGarrison::connect(
    "./garrison.db",
    config,
    "paladin-001"
).await?;
```

**Database Schema:**
```sql
CREATE TABLE garrison_entries (
    id TEXT PRIMARY KEY,
    paladin_id TEXT NOT NULL,
    role TEXT NOT NULL,
    content TEXT NOT NULL,
    timestamp TEXT NOT NULL,
    token_count INTEGER,
    metadata TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE INDEX idx_paladin_timestamp ON garrison_entries(paladin_id, timestamp DESC);

CREATE VIRTUAL TABLE garrison_fts USING fts5(content, paladin_id);
```

## Troubleshooting

### Issue: "Out of memory" with InMemoryGarrison

**Cause:** No eviction configured or limits too high

**Solution:**
```rust
// Set reasonable limits
let config = GarrisonConfig::new(50, Some(2000))
    .with_eviction_strategy(EvictionStrategy::SlidingWindow);
```

### Issue: "Database is locked" with SqliteGarrison

**Cause:** Exceeding connection pool limits or long-running transactions

**Solution:**
- Ensure async operations complete promptly
- Check connection pool size (default: 5)
- Avoid holding database locks during slow operations (LLM calls)

### Issue: Search returns no results

**Cause:** FTS5 tokenization or query syntax

**Solution:**
```rust
// Use phrase search for exact matches
let results = garrison.search("\"exact phrase\"", 10).await?;

// For partial matches, use wildcards (SQLite only)
let results = garrison.search("rust*", 10).await?;
```

### Issue: Entries not appearing after restart

**Cause:** Using InMemoryGarrison instead of SqliteGarrison

**Solution:**
```rust
// Switch to persistent storage
let garrison = SqliteGarrison::connect(
    "./persistent_garrison.db",
    config,
    "my-paladin"
).await?;
```

### Issue: Wrong paladin seeing another's conversation

**Cause:** Using same `paladin_id` for different instances

**Solution:**
```rust
// Use unique IDs per paladin
let garrison1 = SqliteGarrison::connect(db, config, "alice").await?;
let garrison2 = SqliteGarrison::connect(db, config, "bob").await?;
```

### Issue: High memory usage even with eviction

**Cause:** Large content per entry or token counting disabled

**Solution:**
```rust
// Enable token counting (requires tiktoken)
use paladin::infrastructure::adapters::garrison::token_counter::TokenCounterFactory;

let counter = TokenCounterFactory::for_model("gpt-4");

// Manually set token counts
let mut entry = GarrisonEntry::new(role, content);
entry.token_count = Some(counter.count_tokens(&content));
garrison.remember(entry).await?;
```

## Performance Considerations

### Benchmarks (Approximate)

| Operation | InMemory | SQLite |
|-----------|----------|--------|
| Write (single) | ~1 μs | ~1 ms |
| Read recent 10 | ~10 μs | ~2 ms |
| Search (100 entries) | ~50 μs | ~5 ms |
| Search (10k entries) | ~5 ms | ~10 ms |
| Startup | instant | ~50 ms |

### Optimization Tips

#### For InMemoryGarrison
1. **Use appropriate config**: Don't store more than needed
   ```rust
   GarrisonConfig::new(20, Some(1500))  // Small window for recent context
   ```

2. **Periodic cleanup**: Call `forget_all()` after long-running sessions
   ```rust
   if session_count > 100 {
       garrison.forget_all().await?;
   }
   ```

#### For SqliteGarrison
1. **Batch operations**: Group multiple remembers if possible (future enhancement)

2. **Optimize searches**:
   ```rust
   // Good: Specific phrase search
   garrison.search("\"memory management\"", 5).await?

   // Avoid: Very broad searches with high limits
   // garrison.search("the", 1000).await?  // Slow!
   ```

3. **Use appropriate limits**: Don't recall more than needed
   ```rust
   // Good: Only what you need
   let context = garrison.recall_recent(10).await?;

   // Avoid: Retrieving everything unnecessarily
   // let all = garrison.recall_recent(100000).await?;
   ```

4. **Regular VACUUM**: Reclaim space periodically
   ```rust
   // Run VACUUM on SQLite database file periodically
   // (requires manual SQL execution)
   ```

### Memory Footprint

**InMemoryGarrison:**
- Base: ~200 bytes
- Per entry: ~300-500 bytes (depending on content length)
- Example: 100 entries ≈ 30-50 KB

**SqliteGarrison:**
- Base: ~1 MB (connection pool)
- Per entry: Disk storage only
- Example: 10,000 entries ≈ 5-10 MB database file

## Next Steps

- See [`examples/garrison_in_memory.rs`](https://github.com/DF3NDR/paladin-dev-env/blob/main/examples/garrison_in_memory.rs) for basic usage
- See [`examples/garrison_persistent.rs`](https://github.com/DF3NDR/paladin-dev-env/blob/main/examples/garrison_persistent.rs) for SQLite usage
- See [`examples/garrison_semantic_search.rs`](https://github.com/DF3NDR/paladin-dev-env/blob/main/examples/garrison_semantic_search.rs) for future vector search
- Review [API documentation](https://docs.rs/paladin) for detailed type information

## Future Enhancements

- **Vector embeddings**: Semantic similarity search via `LongTermGarrisonPort`
- **Batch operations**: Efficient multi-entry storage
- **Compression**: Automatic content compression for old entries
- **Export/import**: Conversation backup and restore
- **Analytics**: Conversation statistics and insights
- **Redis adapter**: Distributed garrison for multi-node deployments
