# Memory Management Guide

This guide covers how to use the Garrison memory system to give your Paladins conversation context, long-term knowledge, and semantic search capabilities.

## Table of Contents

- [Overview](#overview)
- [Garrison Architecture](#garrison-architecture)
- [In-Memory Garrison](#in-memory-garrison)
- [Persistent Garrison](#persistent-garrison)
- [Memory Windowing](#memory-windowing)
- [Semantic Search](#semantic-search)
- [Memory Types](#memory-types)
- [Best Practices](#best-practices)
- [Advanced Patterns](#advanced-patterns)
- [Troubleshooting](#troubleshooting)

## Overview

The Garrison system provides Paladins with:
- **Conversation Context**: Maintain multi-turn dialogue history
- **Memory Windowing**: Manage token limits intelligently
- **Persistence**: Save and restore sessions across restarts
- **Semantic Search**: Retrieve relevant memories by meaning, not just keywords
- **Embeddings**: Vector-based similarity for long-term memory

**Key Concepts:**
- **Garrison**: Memory storage system for a Paladin
- **GarrisonEntry**: Single memory record (message, observation, fact)
- **ConversationHistory**: Ordered sequence of interactions
- **Memory Window**: Limited context size respecting token limits
- **Long-Term Memory**: Persistent storage with semantic retrieval

## Garrison Architecture

### Core Components

```rust
// Single memory entry
pub struct GarrisonEntry {
    pub id: Uuid,
    pub role: ConversationRole,
    pub content: String,
    pub timestamp: DateTime<Utc>,
    pub metadata: HashMap<String, String>,
    pub token_count: Option<u32>,
}

// Conversation roles
pub enum ConversationRole {
    System,    // System prompts
    User,      // User messages
    Assistant, // Paladin responses
    Tool,      // Tool execution results
}

// Memory interface
#[async_trait]
pub trait GarrisonPort: Send + Sync {
    async fn add_entry(&self, entry: GarrisonEntry) -> Result<()>;
    async fn get_history(&self, limit: usize) -> Result<Vec<GarrisonEntry>>;
    async fn get_window(&self, max_tokens: u32) -> Result<Vec<GarrisonEntry>>;
    async fn search(&self, query: &str, limit: usize) -> Result<Vec<GarrisonEntry>>;
    async fn clear(&self) -> Result<()>;
    async fn stats(&self) -> Result<GarrisonStats>;
}

// Extended port for long-term memory
#[async_trait]
pub trait LongTermGarrisonPort: GarrisonPort {
    async fn add_with_embedding(
        &self,
        entry: GarrisonEntry,
        embedding: Vec<f32>
    ) -> Result<()>;

    async fn semantic_search(
        &self,
        query_embedding: Vec<f32>,
        limit: usize
    ) -> Result<Vec<(GarrisonEntry, f32)>>;
}
```

### Memory Flow

```
User Input → Garrison adds User entry
    ↓
Paladin retrieves relevant history (window or search)
    ↓
LLM generates response with full context
    ↓
Garrison adds Assistant entry
    ↓
(Optional) Tool calls → Garrison adds Tool entries
    ↓
Repeat for next interaction
```

## In-Memory Garrison

Fastest option for short-lived sessions where persistence isn't needed.

### Basic Usage

```rust
use paladin::garrison::*;
use paladin::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let llm_adapter = Arc::new(OpenAiAdapter::new().build()?);

    // Create in-memory garrison
    let garrison = Arc::new(InMemoryGarrison::new(
        GarrisonConfig::default()
            .with_max_entries(100)
            .with_max_tokens(4000)
    ));

    // Build Paladin with memory
    let paladin = PaladinBuilder::new(llm_adapter)
        .name("ChatBot")
        .system_prompt("You are a helpful assistant with memory of our conversation.")
        .with_garrison(garrison.clone())
        .build()?;

    // First interaction
    let response1 = paladin.execute("My name is Alice").await?;
    println!("Bot: {}", response1.content);

    // Second interaction - Paladin remembers
    let response2 = paladin.execute("What's my name?").await?;
    println!("Bot: {}", response2.content);  // Should say "Alice"

    // Check garrison statistics
    let stats = garrison.stats().await?;
    println!("Total memories: {}", stats.total_entries);
    println!("Total tokens: {}", stats.total_tokens);

    Ok(())
}
```

### Configuration Options

```rust
let garrison = InMemoryGarrison::new(
    GarrisonConfig::default()
        // Maximum number of entries to retain
        .with_max_entries(100)

        // Maximum total tokens across all entries
        .with_max_tokens(4000)

        // Token estimation strategy
        .with_token_counter(TokenCounter::Gpt4)

        // Eviction policy when limits reached
        .with_eviction_policy(EvictionPolicy::Fifo)  // First-in-first-out
);
```

### Eviction Policies

```rust
pub enum EvictionPolicy {
    // Remove oldest entries first
    Fifo,

    // Remove least recently accessed entries
    Lru,

    // Remove entries based on importance score
    ImportanceBased,

    // Custom eviction logic
    Custom(Arc<dyn Fn(&[GarrisonEntry]) -> Vec<Uuid> + Send + Sync>),
}

// Example: Custom eviction keeping system prompts
let garrison = InMemoryGarrison::new(
    GarrisonConfig::default()
        .with_eviction_policy(EvictionPolicy::Custom(Arc::new(|entries| {
            // Never evict system prompts, evict oldest user messages
            entries.iter()
                .filter(|e| e.role == ConversationRole::User)
                .take(10)
                .map(|e| e.id)
                .collect()
        })))
);
```

## Persistent Garrison

SQLite-backed storage for sessions that need to survive restarts.

### Setup

```rust
use paladin::garrison::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create persistent garrison
    let garrison = Arc::new(
        SqliteGarrison::new("garrison.db")
            .await?
            .with_config(GarrisonConfig::default())
    );

    let paladin = PaladinBuilder::new(llm_adapter)
        .with_garrison(garrison)
        .build()?;

    // All interactions are automatically persisted
    paladin.execute("Remember this important fact!").await?;

    Ok(())
}
```

### Session Management

```rust
// Create session-based garrison
let session_id = Uuid::new_v4();

let garrison = Arc::new(
    SqliteGarrison::new("garrison.db")
        .await?
        .with_session_id(session_id)
);

// Later, restore the same session
let garrison_restored = Arc::new(
    SqliteGarrison::new("garrison.db")
        .await?
        .with_session_id(session_id)  // Same session ID
);

// History is preserved
let history = garrison_restored.get_history(100).await?;
println!("Restored {} memories", history.len());
```

### Multiple Users

```rust
pub struct UserGarrison {
    db: SqliteGarrison,
    user_id: String,
}

impl UserGarrison {
    pub async fn new(db_path: &str, user_id: String) -> Result<Self> {
        let db = SqliteGarrison::new(db_path).await?;
        Ok(Self { db, user_id })
    }
}

#[async_trait]
impl GarrisonPort for UserGarrison {
    async fn add_entry(&self, mut entry: GarrisonEntry) -> Result<()> {
        // Tag entries with user_id
        entry.metadata.insert("user_id".to_string(), self.user_id.clone());
        self.db.add_entry(entry).await
    }

    async fn get_history(&self, limit: usize) -> Result<Vec<GarrisonEntry>> {
        // Filter by user_id
        let all_entries = self.db.get_history(limit * 2).await?;
        Ok(all_entries.into_iter()
            .filter(|e| e.metadata.get("user_id") == Some(&self.user_id))
            .take(limit)
            .collect())
    }

    // Implement other methods...
}

// Usage
let alice_garrison = Arc::new(UserGarrison::new("garrison.db", "alice".to_string()).await?);
let bob_garrison = Arc::new(UserGarrison::new("garrison.db", "bob".to_string()).await?);

let alice_paladin = PaladinBuilder::new(llm_adapter.clone())
    .with_garrison(alice_garrison)
    .build()?;

let bob_paladin = PaladinBuilder::new(llm_adapter)
    .with_garrison(bob_garrison)
    .build()?;
```

### Database Schema

```sql
-- migrations/001_create_garrison_tables.sql
CREATE TABLE IF NOT EXISTS garrison_entries (
    id TEXT PRIMARY KEY,
    session_id TEXT NOT NULL,
    role TEXT NOT NULL,
    content TEXT NOT NULL,
    timestamp INTEGER NOT NULL,
    metadata TEXT,
    token_count INTEGER,
    embedding BLOB,

    INDEX idx_session_timestamp (session_id, timestamp),
    INDEX idx_session_role (session_id, role)
);

CREATE TABLE IF NOT EXISTS garrison_sessions (
    session_id TEXT PRIMARY KEY,
    user_id TEXT,
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL,
    metadata TEXT
);
```

## Memory Windowing

Intelligently manage context size to respect LLM token limits.

### Token-Based Windowing

```rust
// Get most recent entries that fit within token limit
let window = garrison.get_window(4000).await?;

println!("Window contains {} entries", window.len());
println!("Total tokens: {}",
    window.iter().map(|e| e.token_count.unwrap_or(0)).sum::<u32>());
```

### Sliding Window

```rust
pub struct SlidingWindowGarrison {
    garrison: Arc<dyn GarrisonPort>,
    window_size: u32,
}

impl SlidingWindowGarrison {
    pub fn new(garrison: Arc<dyn GarrisonPort>, window_size: u32) -> Self {
        Self { garrison, window_size }
    }
}

#[async_trait]
impl GarrisonPort for SlidingWindowGarrison {
    async fn get_history(&self, _limit: usize) -> Result<Vec<GarrisonEntry>> {
        // Always return windowed history
        self.garrison.get_window(self.window_size).await
    }

    // Forward other methods to inner garrison
    async fn add_entry(&self, entry: GarrisonEntry) -> Result<()> {
        self.garrison.add_entry(entry).await
    }

    // ... other methods
}

// Usage - Paladin always sees only recent context
let windowed = Arc::new(SlidingWindowGarrison::new(garrison, 4000));

let paladin = PaladinBuilder::new(llm_adapter)
    .with_garrison(windowed)
    .build()?;
```

### Smart Windowing with Priorities

```rust
pub struct PriorityWindowGarrison {
    garrison: Arc<dyn GarrisonPort>,
    window_size: u32,
}

impl PriorityWindowGarrison {
    async fn get_prioritized_window(&self) -> Result<Vec<GarrisonEntry>> {
        let all_entries = self.garrison.get_history(1000).await?;

        // Always include system prompts
        let system_entries: Vec<_> = all_entries.iter()
            .filter(|e| e.role == ConversationRole::System)
            .cloned()
            .collect();

        // Calculate remaining token budget
        let system_tokens: u32 = system_entries.iter()
            .map(|e| e.token_count.unwrap_or(0))
            .sum();

        let remaining_budget = self.window_size.saturating_sub(system_tokens);

        // Fill with most recent non-system entries
        let mut recent_entries: Vec<_> = all_entries.iter()
            .filter(|e| e.role != ConversationRole::System)
            .rev()
            .cloned()
            .collect();

        let mut token_sum = 0u32;
        let mut windowed_recent = Vec::new();

        for entry in recent_entries {
            let entry_tokens = entry.token_count.unwrap_or(0);
            if token_sum + entry_tokens <= remaining_budget {
                token_sum += entry_tokens;
                windowed_recent.push(entry);
            } else {
                break;
            }
        }

        // Combine: system + recent (chronological order)
        windowed_recent.reverse();
        let mut result = system_entries;
        result.extend(windowed_recent);

        Ok(result)
    }
}
```

### Summarization for Compression

```rust
pub struct SummarizingGarrison {
    garrison: Arc<dyn GarrisonPort>,
    summarizer: Arc<dyn LlmPort>,
    window_size: u32,
    summary_threshold: usize,
}

impl SummarizingGarrison {
    async fn maybe_summarize(&self) -> Result<()> {
        let entries = self.garrison.get_history(self.summary_threshold).await?;

        if entries.len() >= self.summary_threshold {
            // Create summary of old entries
            let old_entries: Vec<_> = entries.iter()
                .take(self.summary_threshold / 2)
                .collect();

            let conversation_text = old_entries.iter()
                .map(|e| format!("{:?}: {}", e.role, e.content))
                .collect::<Vec<_>>()
                .join("\n");

            let prompt = format!(
                "Summarize this conversation in 2-3 paragraphs, preserving key facts:\n\n{}",
                conversation_text
            );

            let summary = self.summarizer.generate(&prompt).await?;

            // Replace old entries with summary
            for entry in old_entries {
                self.garrison.remove_entry(entry.id).await?;
            }

            self.garrison.add_entry(GarrisonEntry {
                id: Uuid::new_v4(),
                role: ConversationRole::System,
                content: format!("Previous conversation summary: {}", summary),
                timestamp: Utc::now(),
                metadata: HashMap::from([
                    ("type".to_string(), "summary".to_string()),
                ]),
                token_count: None,
            }).await?;
        }

        Ok(())
    }
}
```

## Semantic Search

Retrieve relevant memories by meaning using embeddings.

### Setup with Embeddings

```rust
use paladin::garrison::*;
use paladin::embeddings::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create garrison with embedding support
    let embedding_service = Arc::new(OpenAIEmbeddingService::new(api_key)?);

    let garrison = Arc::new(
        VectorGarrison::new("garrison.db")
            .await?
            .with_embedding_service(embedding_service)
    );

    let paladin = PaladinBuilder::new(llm_adapter)
        .with_garrison(garrison.clone())
        .build()?;

    // Add entries - embeddings generated automatically
    paladin.execute("I love hiking in the mountains").await?;
    paladin.execute("My favorite color is blue").await?;
    paladin.execute("I work as a software engineer").await?;

    // Semantic search
    let results = garrison.semantic_search("outdoor activities", 5).await?;

    for (entry, similarity) in results {
        println!("Similarity: {:.2} - {}", similarity, entry.content);
    }
    // Output: High similarity for "hiking in the mountains"

    Ok(())
}
```

### Hybrid Search (Keyword + Semantic)

```rust
pub struct HybridGarrison {
    garrison: Arc<dyn LongTermGarrisonPort>,
}

impl HybridGarrison {
    pub async fn hybrid_search(
        &self,
        query: &str,
        limit: usize,
    ) -> Result<Vec<GarrisonEntry>> {
        // Get keyword matches
        let keyword_results = self.garrison.search(query, limit * 2).await?;

        // Get semantic matches
        let embedding = self.embedding_service.embed(query).await?;
        let semantic_results = self.garrison
            .semantic_search(embedding, limit * 2)
            .await?;

        // Merge and deduplicate
        let mut combined: HashMap<Uuid, (GarrisonEntry, f32)> = HashMap::new();

        // Add keyword results with base score
        for entry in keyword_results {
            combined.insert(entry.id, (entry, 0.5));
        }

        // Add semantic results, boosting score if already present
        for (entry, similarity) in semantic_results {
            combined.entry(entry.id)
                .and_modify(|(_, score)| *score += similarity * 0.5)
                .or_insert((entry, similarity * 0.5));
        }

        // Sort by combined score
        let mut sorted: Vec<_> = combined.into_values().collect();
        sorted.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

        Ok(sorted.into_iter()
            .take(limit)
            .map(|(entry, _)| entry)
            .collect())
    }
}
```

### RAG (Retrieval-Augmented Generation)

```rust
pub struct RAGPaladin {
    paladin: Paladin,
    garrison: Arc<dyn LongTermGarrisonPort>,
}

impl RAGPaladin {
    pub async fn execute_with_rag(&self, query: &str) -> Result<PaladinResult> {
        // Retrieve relevant context from long-term memory
        let embedding = self.embedding_service.embed(query).await?;
        let relevant_memories = self.garrison
            .semantic_search(embedding, 5)
            .await?;

        // Build augmented prompt
        let context = relevant_memories.iter()
            .map(|(entry, _)| entry.content.as_str())
            .collect::<Vec<_>>()
            .join("\n\n");

        let augmented_query = format!(
            "Context from previous conversations:\n{}\n\n\
             Current question: {}",
            context, query
        );

        // Execute with retrieved context
        self.paladin.execute(&augmented_query).await
    }
}

// Usage
let rag_paladin = RAGPaladin {
    paladin,
    garrison: vector_garrison,
};

let response = rag_paladin.execute_with_rag(
    "What programming languages do I know?"
).await?;
```

## Memory Types

### Episodic Memory

Memory of specific events and experiences.

```rust
// Add episodic memory
garrison.add_entry(GarrisonEntry {
    id: Uuid::new_v4(),
    role: ConversationRole::User,
    content: "I visited Paris last summer".to_string(),
    timestamp: Utc::now(),
    metadata: HashMap::from([
        ("memory_type".to_string(), "episodic".to_string()),
        ("event_type".to_string(), "travel".to_string()),
        ("location".to_string(), "Paris, France".to_string()),
        ("timeframe".to_string(), "summer 2023".to_string()),
    ]),
    token_count: Some(10),
}).await?;
```

### Semantic Memory

General knowledge and facts.

```rust
// Add semantic memory (facts)
garrison.add_entry(GarrisonEntry {
    id: Uuid::new_v4(),
    role: ConversationRole::System,
    content: "User prefers Python over JavaScript for backend development".to_string(),
    timestamp: Utc::now(),
    metadata: HashMap::from([
        ("memory_type".to_string(), "semantic".to_string()),
        ("category".to_string(), "preferences".to_string()),
        ("topic".to_string(), "programming".to_string()),
    ]),
    token_count: Some(15),
}).await?;
```

### Procedural Memory

Knowledge about how to do things.

```rust
// Add procedural memory
garrison.add_entry(GarrisonEntry {
    id: Uuid::new_v4(),
    role: ConversationRole::System,
    content: "To deploy this project: cargo build --release && docker build -t app .".to_string(),
    timestamp: Utc::now(),
    metadata: HashMap::from([
        ("memory_type".to_string(), "procedural".to_string()),
        ("task".to_string(), "deployment".to_string()),
    ]),
    token_count: Some(20),
}).await?;
```

## Best Practices

### 1. Choose the Right Garrison Type

```rust
// ✅ Use InMemoryGarrison for:
// - Temporary chatbots
// - Stateless services
// - Testing and development

let garrison = Arc::new(InMemoryGarrison::new(
    GarrisonConfig::default().with_max_tokens(4000)
));

// ✅ Use SqliteGarrison for:
// - Multi-session applications
// - User-specific contexts
// - Production services needing persistence

let garrison = Arc::new(
    SqliteGarrison::new("garrison.db").await?
        .with_session_id(session_id)
);

// ✅ Use VectorGarrison for:
// - Long-term knowledge bases
// - RAG applications
// - Semantic retrieval needs

let garrison = Arc::new(
    VectorGarrison::new("garrison.db").await?
        .with_embedding_service(embedding_service)
);
```

### 2. Set Appropriate Token Limits

```rust
// Model context windows
const GPT_4_TURBO: u32 = 128_000;
const GPT_4: u32 = 8_192;
const GPT_3_5: u32 = 16_385;
const CLAUDE_3: u32 = 200_000;

// Reserve tokens for: system prompt + response + buffer
let response_tokens = 1000;
let system_prompt_tokens = 500;
let buffer = 500;

let available_for_history = GPT_4 - response_tokens - system_prompt_tokens - buffer;

let garrison = InMemoryGarrison::new(
    GarrisonConfig::default()
        .with_max_tokens(available_for_history)  // ~6000 tokens
);
```

### 3. Add Metadata for Better Organization

```rust
garrison.add_entry(GarrisonEntry {
    id: Uuid::new_v4(),
    role: ConversationRole::User,
    content: message.clone(),
    timestamp: Utc::now(),
    metadata: HashMap::from([
        ("user_id".to_string(), user_id.clone()),
        ("session_id".to_string(), session_id.to_string()),
        ("channel".to_string(), "web".to_string()),
        ("language".to_string(), "en".to_string()),
        ("importance".to_string(), "high".to_string()),
    ]),
    token_count: Some(estimate_tokens(&message)),
}).await?;
```

### 4. Clean Up Old Memories

```rust
// Periodic cleanup
pub async fn cleanup_old_memories(
    garrison: &SqliteGarrison,
    days_to_keep: i64,
) -> Result<usize> {
    let cutoff = Utc::now() - Duration::days(days_to_keep);

    let removed = garrison
        .remove_before(cutoff)
        .await?;

    println!("Removed {} old memories", removed);
    Ok(removed)
}

// Scheduled cleanup
tokio::spawn(async move {
    let mut interval = tokio::time::interval(Duration::from_secs(86400)); // Daily
    loop {
        interval.tick().await;
        if let Err(e) = cleanup_old_memories(&garrison, 30).await {
            eprintln!("Cleanup failed: {}", e);
        }
    }
});
```

### 5. Implement Conversation Branching

```rust
pub struct BranchingGarrison {
    garrison: Arc<dyn GarrisonPort>,
    current_branch: RwLock<Uuid>,
}

impl BranchingGarrison {
    pub async fn create_branch(&self, from_entry: Uuid) -> Result<Uuid> {
        let branch_id = Uuid::new_v4();

        // Copy history up to branch point
        let history = self.garrison.get_history(1000).await?;
        let branch_history: Vec<_> = history.into_iter()
            .take_while(|e| e.id != from_entry)
            .collect();

        // Store branch metadata
        self.garrison.add_entry(GarrisonEntry {
            id: Uuid::new_v4(),
            role: ConversationRole::System,
            content: format!("Branch created from entry {}", from_entry),
            timestamp: Utc::now(),
            metadata: HashMap::from([
                ("type".to_string(), "branch".to_string()),
                ("branch_id".to_string(), branch_id.to_string()),
                ("parent_entry".to_string(), from_entry.to_string()),
            ]),
            token_count: None,
        }).await?;

        *self.current_branch.write().await = branch_id;
        Ok(branch_id)
    }
}
```

## Advanced Patterns

### Memory Consolidation

```rust
pub struct ConsolidatingGarrison {
    garrison: Arc<dyn GarrisonPort>,
    llm: Arc<dyn LlmPort>,
}

impl ConsolidatingGarrison {
    pub async fn consolidate_memories(&self) -> Result<()> {
        let entries = self.garrison.get_history(100).await?;

        // Group by topic using LLM
        let topics = self.extract_topics(&entries).await?;

        // Create consolidated memory for each topic
        for (topic, topic_entries) in topics {
            let facts = self.extract_facts(&topic_entries).await?;

            self.garrison.add_entry(GarrisonEntry {
                id: Uuid::new_v4(),
                role: ConversationRole::System,
                content: format!("Consolidated facts about {}: {}", topic, facts),
                timestamp: Utc::now(),
                metadata: HashMap::from([
                    ("type".to_string(), "consolidated".to_string()),
                    ("topic".to_string(), topic),
                    ("source_count".to_string(), topic_entries.len().to_string()),
                ]),
                token_count: None,
            }).await?;
        }

        Ok(())
    }

    async fn extract_topics(&self, entries: &[GarrisonEntry]) -> Result<HashMap<String, Vec<GarrisonEntry>>> {
        // Use LLM to categorize entries by topic
        // Implementation details...
        Ok(HashMap::new())
    }

    async fn extract_facts(&self, entries: &[GarrisonEntry]) -> Result<String> {
        let conversation = entries.iter()
            .map(|e| &e.content)
            .cloned()
            .collect::<Vec<_>>()
            .join("\n");

        let prompt = format!(
            "Extract key facts from this conversation:\n\n{}",
            conversation
        );

        self.llm.generate(&prompt).await
    }
}
```

### Attention Mechanism

```rust
pub struct AttentionGarrison {
    garrison: Arc<dyn LongTermGarrisonPort>,
}

impl AttentionGarrison {
    pub async fn get_attended_context(
        &self,
        query: &str,
        context_size: u32,
    ) -> Result<Vec<GarrisonEntry>> {
        // Get semantic matches
        let query_embedding = self.embed(query).await?;
        let candidates = self.garrison
            .semantic_search(query_embedding, 50)
            .await?;

        // Score each candidate using attention mechanism
        let mut scored: Vec<_> = candidates.into_iter()
            .map(|(entry, similarity)| {
                let recency_score = self.recency_score(&entry);
                let importance_score = self.importance_score(&entry);

                // Weighted combination
                let attention = similarity * 0.5 + recency_score * 0.3 + importance_score * 0.2;

                (entry, attention)
            })
            .collect();

        // Sort by attention score
        scored.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

        // Select top entries within token budget
        let mut selected = Vec::new();
        let mut token_sum = 0u32;

        for (entry, _) in scored {
            let entry_tokens = entry.token_count.unwrap_or(0);
            if token_sum + entry_tokens <= context_size {
                token_sum += entry_tokens;
                selected.push(entry);
            }
        }

        Ok(selected)
    }

    fn recency_score(&self, entry: &GarrisonEntry) -> f32 {
        let age = (Utc::now() - entry.timestamp).num_seconds() as f32;
        let decay_rate = 0.0001;  // Adjust for desired decay speed
        (-decay_rate * age).exp()
    }

    fn importance_score(&self, entry: &GarrisonEntry) -> f32 {
        // Extract importance from metadata or content
        entry.metadata.get("importance")
            .and_then(|s| s.parse::<f32>().ok())
            .unwrap_or(0.5)
    }
}
```

### Memory Reflection

```rust
pub struct ReflectiveGarrison {
    garrison: Arc<dyn GarrisonPort>,
    llm: Arc<dyn LlmPort>,
}

impl ReflectiveGarrison {
    pub async fn generate_reflections(&self) -> Result<()> {
        let recent_entries = self.garrison.get_history(50).await?;

        // Prompt LLM to reflect on conversation
        let conversation = recent_entries.iter()
            .map(|e| format!("{:?}: {}", e.role, e.content))
            .collect::<Vec<_>>()
            .join("\n");

        let prompt = format!(
            "Reflect on this conversation and extract:\n\
             1. Key insights about the user\n\
             2. Patterns in the discussion\n\
             3. Important facts to remember\n\n\
             Conversation:\n{}",
            conversation
        );

        let reflection = self.llm.generate(&prompt).await?;

        // Store reflection as high-importance memory
        self.garrison.add_entry(GarrisonEntry {
            id: Uuid::new_v4(),
            role: ConversationRole::System,
            content: format!("Reflection: {}", reflection),
            timestamp: Utc::now(),
            metadata: HashMap::from([
                ("type".to_string(), "reflection".to_string()),
                ("importance".to_string(), "high".to_string()),
            ]),
            token_count: None,
        }).await?;

        Ok(())
    }
}
```

## Troubleshooting

### Memory Not Persisting

**Problem**: Garrison entries disappear after restart.

**Solutions**:
1. Verify using `SqliteGarrison`, not `InMemoryGarrison`
2. Check database file path is correct and writable
3. Ensure proper async handling (`.await` on all operations)

```rust
// ❌ Won't persist
let garrison = Arc::new(InMemoryGarrison::new(config));

// ✅ Will persist
let garrison = Arc::new(SqliteGarrison::new("garrison.db").await?);
```

### Context Window Overflow

**Problem**: Errors about exceeding maximum context length.

**Solutions**:
1. Reduce `max_tokens` in `GarrisonConfig`
2. Use `get_window()` instead of `get_history()`
3. Implement summarization for old memories

```rust
// Calculate safe token limit
let model_limit = 8192;  // GPT-4
let response_budget = 1000;
let system_prompt_tokens = 500;
let safety_buffer = 500;

let garrison_limit = model_limit - response_budget - system_prompt_tokens - safety_buffer;

let garrison = InMemoryGarrison::new(
    GarrisonConfig::default().with_max_tokens(garrison_limit)
);
```

### Slow Semantic Search

**Problem**: Embedding-based search is taking too long.

**Solutions**:
1. Add database indexes on embedding columns
2. Use approximate nearest neighbor (ANN) algorithms
3. Cache embeddings for frequent queries
4. Limit search scope with filters

```sql
-- Add index for faster vector search
CREATE INDEX idx_embeddings ON garrison_entries(embedding);

-- Consider using specialized vector databases
-- PostgreSQL with pgvector extension
-- Qdrant, Milvus, or Weaviate for production
```

### Memory Leaks in Long Sessions

**Problem**: Memory usage grows unbounded.

**Solutions**:
1. Set `max_entries` in config
2. Implement periodic cleanup
3. Use eviction policies
4. Monitor with `garrison.stats()`

```rust
// Periodic memory management
tokio::spawn(async move {
    let mut interval = tokio::time::interval(Duration::from_secs(3600));
    loop {
        interval.tick().await;

        let stats = garrison.stats().await.unwrap();

        if stats.total_entries > 1000 {
            // Trigger cleanup
            garrison.compact().await.unwrap();
        }
    }
});
```

## Testing

### Unit Testing

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_garrison_add_and_retrieve() {
        let garrison = InMemoryGarrison::new(GarrisonConfig::default());

        let entry = GarrisonEntry {
            id: Uuid::new_v4(),
            role: ConversationRole::User,
            content: "Test message".to_string(),
            timestamp: Utc::now(),
            metadata: HashMap::new(),
            token_count: Some(2),
        };

        garrison.add_entry(entry.clone()).await.unwrap();

        let history = garrison.get_history(10).await.unwrap();
        assert_eq!(history.len(), 1);
        assert_eq!(history[0].content, "Test message");
    }

    #[tokio::test]
    async fn test_token_window() {
        let garrison = InMemoryGarrison::new(
            GarrisonConfig::default().with_max_tokens(100)
        );

        // Add entries totaling 150 tokens
        for i in 0..15 {
            garrison.add_entry(GarrisonEntry {
                id: Uuid::new_v4(),
                role: ConversationRole::User,
                content: format!("Message {}", i),
                timestamp: Utc::now(),
                metadata: HashMap::new(),
                token_count: Some(10),
            }).await.unwrap();
        }

        // Window should respect token limit
        let window = garrison.get_window(100).await.unwrap();
        let total_tokens: u32 = window.iter()
            .map(|e| e.token_count.unwrap_or(0))
            .sum();

        assert!(total_tokens <= 100);
    }
}
```

## Examples

See working examples:
- `examples/garrison_in_memory.rs` - Basic in-memory usage
- `examples/garrison_persistent.rs` - SQLite persistence
- `examples/garrison_semantic_search.rs` - Embedding-based retrieval
- `examples/memory_windowing.rs` - Token management strategies

## Next Steps

- **[Tool Integration](tool-integration.md)** - Combine memory with tools
- **[Battalion Patterns](battalion-patterns.md)** - Shared memory in multi-agent systems
- **[API Reference](https://docs.rs/paladin)** - Garrison API documentation

## Related Resources

- [Token Counting Strategies](../architecture/token-estimation.md)
- [Vector Database Integration](../contributing/vector-db-setup.md)
- [Production Deployment](../deployment/garrison-scaling.md)
