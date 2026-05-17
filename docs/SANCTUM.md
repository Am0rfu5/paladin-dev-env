# Sanctum: Long-term Memory System

Sanctum is Paladin's long-term memory system that enables AI agents to store, retrieve, and learn from historical interactions using vector embeddings and semantic search.

## Table of Contents

- [Overview](#overview)
- [Architecture](#architecture)
- [Adapters](#adapters)
- [Configuration](#configuration)
- [Usage Examples](#usage-examples)
- [RAG Integration](#rag-integration-retrieval-augmented-generation)
- [Performance](#performance)
- [Deployment](#deployment)
- [Migration Guide](#migration-guide)
- [API Reference](#api-reference)

## Overview

Sanctum provides persistent, searchable memory for Paladin agents through a flexible adapter system that supports both development and production scenarios.

### Key Features

- **Vector-based semantic search**: Find relevant memories using embedding similarity
- **Flexible storage adapters**: Choose between in-memory (dev) and Qdrant (production)
- **Rich metadata filtering**: Filter by paladin ID, memory type, importance, timestamps
- **Memory types**: Episodic (events), Semantic (facts), Procedural (skills)
- **Importance scoring**: Prioritize critical memories (0.0-1.0 scale)
- **Access tracking**: Monitor memory usage patterns
- **Batch operations**: Efficiently store multiple memories

### Use Cases

1. **Conversation History**: Remember past interactions with users
2. **Knowledge Accumulation**: Build long-term knowledge bases
3. **Context Retrieval**: Pull relevant context for current tasks
4. **Learning from Experience**: Improve responses based on historical data
5. **Multi-session Continuity**: Maintain state across agent restarts

## Architecture

Sanctum follows the Hexagonal Architecture pattern with clear separation between domain, application, and infrastructure layers:

```
┌─────────────────────────────────────────────────────────────┐
│                     Application Layer                        │
│  ┌───────────────────────────────────────────────────────┐  │
│  │              SanctumPort (Trait)                      │  │
│  │  - store()                                            │  │
│  │  - store_batch()                                      │  │
│  │  - search()                                           │  │
│  │  - delete()                                           │  │
│  │  - update()                                           │  │
│  │  - count()                                            │  │
│  └───────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────┘
                            │
        ┌───────────────────┴───────────────────┐
        │                                       │
        ▼                                       ▼
┌────────────────────┐              ┌────────────────────┐
│ InMemorySanctum    │              │ QdrantSanctumAdapter│
│ (Development)      │              │ (Production)       │
│                    │              │                    │
│ - HashMap storage  │              │ - Vector database  │
│ - Fast startup     │              │ - Persistent       │
│ - No setup needed  │              │ - Scalable         │
└────────────────────┘              └────────────────────┘
```

### Domain Types

#### Memory

Represents a single memory entry with metadata:

```rust
pub struct Memory {
    pub id: Uuid,
    pub paladin_id: String,
    pub content: String,
    pub memory_type: MemoryType,
    pub importance: f32,
    pub access_count: u32,
    pub created_at: DateTime<Utc>,
    pub last_accessed: DateTime<Utc>,
    pub metadata: HashMap<String, Value>,
}
```

#### MemoryType

Categories for different types of memories:

- **Episodic**: Specific events and experiences ("User asked about Rust")
- **Semantic**: General facts and knowledge ("Rust is a systems programming language")
- **Procedural**: How-to knowledge and skills ("To compile Rust, run `cargo build`")

#### SanctumEntry

Memory paired with its vector embedding:

```rust
pub struct SanctumEntry {
    pub memory: Memory,
    pub embedding: Vec<f32>,
}
```

## Adapters

Sanctum supports multiple storage adapters through the `SanctumPort` trait.

### InMemory Adapter

**Best for**:
- Development and testing
- Prototyping
- Small-scale deployments (<10,000 memories)
- Fast iteration without infrastructure

**Characteristics**:
- ✅ Zero setup required
- ✅ Lightning-fast operations (<1ms)
- ✅ Simple debugging
- ❌ Data lost on restart
- ❌ Limited to single machine
- ❌ Memory constrained by RAM

**Configuration**:
```yaml
sanctum:
  enabled: true
  adapter_type: "in_memory"
```

### Qdrant Adapter

**Best for**:
- Production deployments
- Large-scale applications (>10,000 memories)
- Distributed systems
- Data persistence requirements

**Characteristics**:
- ✅ Persistent storage
- ✅ Scalable to millions of vectors
- ✅ Fast semantic search (<500ms for 100K vectors)
- ✅ Distributed deployment support
- ✅ HNSW indexing for performance
- ❌ Requires Qdrant infrastructure
- ❌ Slightly higher latency than in-memory

**Configuration**:
```yaml
sanctum:
  enabled: true
  adapter_type: "qdrant"
  qdrant:
    url: "http://localhost:6334"
    collection_name: "paladin_memories"
    vector_dimension: 1536  # Must match embedding model
```

### Adapter Comparison

| Feature | InMemory | Qdrant |
|---------|----------|--------|
| Setup Time | Instant | ~1 minute |
| Storage Capacity | RAM limited | Disk limited |
| Persistence | ❌ Ephemeral | ✅ Persistent |
| Search Speed | <1ms | <500ms |
| Scaling | Single node | Distributed |
| Production Ready | ❌ | ✅ |
| Cost | Free | Infrastructure costs |

## Configuration

### Basic Configuration

```yaml
# Minimal development configuration
sanctum:
  enabled: true
  adapter_type: "in_memory"
```

### Production Configuration

```yaml
# Production Qdrant configuration
sanctum:
  enabled: true
  adapter_type: "qdrant"
  qdrant:
    url: "http://qdrant:6334"
    collection_name: "paladin_production_memories"
    vector_dimension: 1536  # OpenAI text-embedding-3-small
```

### Environment Variable Overrides

All configuration can be overridden via environment variables:

```bash
# Enable/disable Sanctum
export APP_SANCTUM_ENABLED=true

# Select adapter
export APP_SANCTUM_ADAPTER_TYPE=qdrant

# Qdrant configuration
export APP_SANCTUM_QDRANT_URL=http://qdrant-cluster:6334
export APP_SANCTUM_QDRANT_COLLECTION_NAME=custom_memories
export APP_SANCTUM_QDRANT_VECTOR_DIMENSION=3072
```

### Vector Dimensions by Model

Choose the dimension that matches your embedding model:

| Model | Dimension | Use Case |
|-------|-----------|----------|
| OpenAI text-embedding-3-small | 1536 | General purpose, cost-effective |
| OpenAI text-embedding-3-large | 3072 | Higher quality, more expensive |
| sentence-transformers/all-mpnet-base-v2 | 768 | Open-source, self-hosted |
| sentence-transformers/all-MiniLM-L6-v2 | 384 | Lightweight, fast |

## Usage Examples

### Creating a Sanctum Adapter

#### Development (InMemory)

```rust
use paladin::infrastructure::adapters::sanctum::InMemorySanctum;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // No configuration needed for in-memory
    let sanctum = InMemorySanctum::new();
    
    println!("InMemory Sanctum ready!");
    Ok(())
}
```

#### Production (Qdrant)

```rust
use paladin::infrastructure::adapters::sanctum::QdrantSanctumAdapter;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Connect to Qdrant
    let sanctum = QdrantSanctumAdapter::new(
        "http://localhost:6334",  // Qdrant gRPC endpoint
        "paladin_memories",        // Collection name
        1536,                      // Vector dimension
    ).await?;
    
    println!("Qdrant Sanctum connected!");
    Ok(())
}
```

### Storing Memories

```rust
use paladin::core::platform::container::sanctum::{MemoryBuilder, MemoryType, SanctumEntry};
use paladin::paladin_ports::output::sanctum_port::SanctumPort;

async fn store_memory(
    sanctum: &dyn SanctumPort,
    embedding_vector: Vec<f32>,
) -> Result<(), Box<dyn std::error::Error>> {
    // Build a memory
    let memory = MemoryBuilder::new(
        "paladin-123".to_string(),
        "User asked about Rust programming".to_string(),
    )
    .memory_type(MemoryType::Episodic)
    .importance(0.8)
    .build()?;
    
    // Create entry with embedding
    let entry = SanctumEntry::new(memory, embedding_vector)?;
    
    // Store in Sanctum
    sanctum.store(entry).await?;
    
    Ok(())
}
```

### Batch Storing

```rust
async fn store_batch(
    sanctum: &dyn SanctumPort,
) -> Result<(), Box<dyn std::error::Error>> {
    let entries: Vec<SanctumEntry> = vec![
        // ... create multiple entries
    ];
    
    // Efficient batch storage
    sanctum.store_batch(entries).await?;
    
    Ok(())
}
```

### Semantic Search

```rust
use paladin::paladin_ports::output::sanctum_port::SanctumQuery;

async fn search_memories(
    sanctum: &dyn SanctumPort,
    query_embedding: Vec<f32>,
) -> Result<(), Box<dyn std::error::Error>> {
    // Create search query
    let query = SanctumQuery::new(query_embedding, 5)  // Top 5 results
        .min_score(0.7);  // Minimum similarity threshold
    
    // Execute search
    let results = sanctum.search(query).await?;
    
    for result in results {
        println!("Score: {:.3} - {}", result.score, result.entry.memory.content);
    }
    
    Ok(())
}
```

### Filtered Search

```rust
use paladin::paladin_ports::output::sanctum_port::SanctumFilter;

async fn filtered_search(
    sanctum: &dyn SanctumPort,
    query_embedding: Vec<f32>,
) -> Result<(), Box<dyn std::error::Error>> {
    // Build filter
    let filter = SanctumFilter::new()
        .paladin_id("paladin-123".to_string())
        .memory_type(MemoryType::Episodic)
        .min_importance(0.5);
    
    // Search with filter
    let query = SanctumQuery::new(query_embedding, 10)
        .filter(filter);
    
    let results = sanctum.search(query).await?;
    
    Ok(())
}
```

### Updating and Deleting

```rust
async fn update_memory(
    sanctum: &dyn SanctumPort,
    entry: SanctumEntry,
) -> Result<(), Box<dyn std::error::Error>> {
    // Update entry (upsert)
    sanctum.update(entry).await?;
    
    Ok(())
}

async fn delete_memory(
    sanctum: &dyn SanctumPort,
    memory_id: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    // Delete by ID
    let deleted = sanctum.delete(memory_id).await?;
    
    if deleted {
        println!("Memory deleted successfully");
    } else {
        println!("Memory not found");
    }
    
    Ok(())
}
```

## Performance

### Benchmarks

Performance characteristics based on testing:

#### InMemory Adapter

| Operation | 100 entries | 1,000 entries | 10,000 entries |
|-----------|-------------|---------------|----------------|
| Store (single) | <1ms | <1ms | <1ms |
| Store (batch) | 2ms | 15ms | 150ms |
| Search (top 10) | <1ms | 3ms | 25ms |
| Delete | <1ms | <1ms | <1ms |

#### Qdrant Adapter

| Operation | 1K entries | 10K entries | 100K entries | 1M entries |
|-----------|------------|-------------|--------------|------------|
| Store (single) | 5ms | 5ms | 5ms | 5ms |
| Store (batch 100) | 50ms | 50ms | 50ms | 50ms |
| Search (top 10) | 15ms | 25ms | 50ms | 200ms |
| Delete | 5ms | 5ms | 5ms | 5ms |

### Performance Recommendations

1. **Use batch operations**: 10-100x faster than individual stores
2. **Set appropriate top_k**: Lower values = faster searches
3. **Use min_score**: Filter low-quality results early
4. **Index design**: HNSW indexing in Qdrant provides sub-linear search time
5. **Monitor memory**: InMemory adapter consumes ~1KB per entry with 1536-dim vectors

### Scaling Guidelines

#### InMemory
- **Comfortable**: Up to 10,000 entries
- **Maximum**: 100,000 entries (requires ~150MB RAM with 1536-dim vectors)
- **Beyond**: Switch to Qdrant

#### Qdrant
- **Single node**: 1-10 million entries
- **Cluster**: 10M+ entries with horizontal scaling
- **Performance target**: <500ms search on 100K entries maintained

## Deployment

See [DEPLOYMENT.md](./DEPLOYMENT.md) for detailed deployment guides including:
- Docker Compose setup
- Kubernetes deployment
- Cloud provider configurations (AWS, GCP, Azure)
- Production best practices
- Monitoring and observability

### Quick Docker Setup

```yaml
# docker-compose.yml
services:
  qdrant:
    image: qdrant/qdrant:latest
    ports:
      - "6333:6333"  # HTTP API
      - "6334:6334"  # gRPC API
    volumes:
      - qdrant_data:/qdrant/storage
    environment:
      - QDRANT__SERVICE__HTTP_PORT=6333
      - QDRANT__SERVICE__GRPC_PORT=6334

volumes:
  qdrant_data:
```

Start with:
```bash
docker-compose up -d qdrant
```

## Migration Guide

See [MIGRATION.md](./MIGRATION.md) for detailed migration guides including:
- Migrating from InMemory to Qdrant
- Exporting and importing memories
- Zero-downtime migration strategies
- Rollback procedures

### Quick Migration Overview

1. **Export** memories from InMemory adapter
2. **Start** Qdrant infrastructure
3. **Configure** Paladin with Qdrant adapter
4. **Import** memories into Qdrant
5. **Validate** data integrity
6. **Switch** to Qdrant adapter

## API Reference

### SanctumPort Trait

The main interface for all Sanctum adapters:

```rust
#[async_trait]
pub trait SanctumPort: Send + Sync {
    /// Store a single memory entry
    async fn store(&self, entry: SanctumEntry) -> Result<(), SanctumError>;
    
    /// Store multiple entries in batch (more efficient)
    async fn store_batch(&self, entries: Vec<SanctumEntry>) -> Result<(), SanctumError>;
    
    /// Search for similar memories using vector similarity
    async fn search(&self, query: SanctumQuery) -> Result<Vec<SanctumSearchResult>, SanctumError>;
    
    /// Delete a memory by ID
    async fn delete(&self, id: &str) -> Result<bool, SanctumError>;
    
    /// Update an existing memory (upsert)
    async fn update(&self, entry: SanctumEntry) -> Result<(), SanctumError>;
    
    /// Count memories matching optional filter
    async fn count(&self, filter: Option<SanctumFilter>) -> Result<usize, SanctumError>;
}
```

### Memory Builder

Fluent API for creating memories:

```rust
let memory = MemoryBuilder::new(paladin_id, content)
    .memory_type(MemoryType::Semantic)
    .importance(0.9)
    .with_metadata("key", json!("value"))
    .build()?;
```

### Query Builder

Build semantic search queries:

```rust
let query = SanctumQuery::new(embedding, top_k)
    .min_score(0.7)
    .filter(filter);
```

### Filter Builder

Build complex filters:

```rust
let filter = SanctumFilter::new()
    .paladin_id("paladin-123")
    .memory_type(MemoryType::Episodic)
    .min_importance(0.5)
    .created_after(start_time)
    .created_before(end_time)
    .with_metadata("category", json!("technical"));
```

## Error Handling

Sanctum operations return `Result<T, SanctumError>`:

```rust
#[derive(Debug, thiserror::Error)]
pub enum SanctumError {
    #[error("Storage error: {0}")]
    StorageError(String),
    
    #[error("Search error: {0}")]
    SearchError(String),
    
    #[error("Memory not found: {0}")]
    NotFound(String),
    
    #[error("Invalid dimension: {0}")]
    InvalidDimension(String),
    
    #[error("Configuration error: {0}")]
    ConfigError(String),
}
```

Handle errors appropriately:

```rust
match sanctum.store(entry).await {
    Ok(()) => println!("Memory stored successfully"),
    Err(SanctumError::StorageError(msg)) => eprintln!("Storage failed: {}", msg),
    Err(e) => eprintln!("Unexpected error: {}", e),
}
```

## RAG Integration (Retrieval-Augmented Generation)

> **New in Epic 12**: Automatic memory retrieval and extraction for Paladin agents

Sanctum now supports seamless RAG integration, enabling Paladin agents to automatically retrieve relevant context before execution and extract memories after completion.

### Overview

RAG (Retrieval-Augmented Generation) enhances Paladin responses by:

1. **Auto-Retrieval**: Fetch relevant memories before LLM calls
2. **Context Injection**: Insert historical context into prompts
3. **Auto-Extraction**: Store important facts after execution
4. **Knowledge Building**: Accumulate wisdom across sessions

### Architecture

```
User Input
    ↓
┌─────────────────────────────┐
│  RagRetrievalService        │
│  • Embed query              │
│  • Search Sanctum (top-k)   │
│  • Filter by similarity     │
│  • Format as context        │
└─────────────┬───────────────┘
              ↓
┌─────────────────────────────┐
│  PaladinExecutionService    │
│  • Inject context to prompt │
│  • Execute LLM with context │
│  • Return enriched response │
└─────────────┬───────────────┘
              ↓
┌─────────────────────────────┐
│  MemoryExtractionService    │
│  • Parse response           │
│  • Identify key facts       │
│  • Generate embeddings      │
│  • Store in Sanctum         │
└─────────────────────────────┘
    ↓
Response
```

### Configuration

Add RAG configuration to your `config.yml`:

```yaml
# Sanctum configuration (required for RAG)
sanctum:
  provider: qdrant  # or 'in_memory'
  qdrant:
    url: http://localhost:6333
    collection_name: paladin_memories
    vector_dimension: 1536  # Match embedding model
    distance: cosine

# RAG Retrieval settings
rag:
  top_k: 5                  # Number of memories to retrieve
  min_similarity: 0.7        # Minimum similarity score (0.0-1.0)
  max_tokens: 2000           # Max tokens for context
  timeout_seconds: 5         # Retrieval timeout

# Memory Extraction settings
memory_extraction:
  enabled: true
  strategy: on_completion    # Options: on_completion, every_turn, manual, threshold
```

### RAG Retrieval Service

#### Basic Usage

```rust
use paladin::application::use_cases::sanctum::rag_retrieval_service::{
    RagRetrievalService, RagConfig
};

let rag_service = RagRetrievalService::new(
    Arc::clone(&sanctum_port),
    Arc::clone(&embedding_port),
    RagConfig::default(),
);

// Retrieve relevant context
let memories = rag_service
    .retrieve_context("paladin-id", "user query")
    .await?;

// Format for prompt injection
let context_text = rag_service.format_for_prompt(&memories);
```

#### Configuration Options

```rust
let rag_config = RagConfig {
    top_k: 5,                              // Retrieve top 5 memories
    min_similarity: 0.7,                   // Only >= 70% match
    max_tokens: 2000,                      // Budget limit
    retrieval_trigger: RetrievalTrigger::Always,  // When to retrieve
};
```

**Retrieval Triggers**:
- `Always`: Retrieve for every query (recommended)
- `KeywordBased`: Retrieve only if keywords detected
- `SemanticThreshold`: Retrieve if query similarity exceeds threshold

#### Advanced Features

**Deduplication**: Automatically removes near-identical memories (>0.95 similarity)

**Ranking**: Sorts memories by relevance score (descending)

**Token Budget**: Truncates context to fit within max_tokens limit

**Timeout Handling**: Gracefully handles retrieval timeouts (returns empty context)

### Memory Extraction Service

#### Basic Usage

```rust
use paladin::application::use_cases::sanctum::memory_extraction_service::{
    MemoryExtractionService, MemoryExtractionStrategy
};

let extraction_service = MemoryExtractionService::new(
    Arc::clone(&llm_port),
    Arc::clone(&embedding_port),
    Arc::clone(&sanctum_port),
);

// Extract memories from conversation
let conversation = vec![
    garrison_entry_1,
    garrison_entry_2,
];

let extracted = extraction_service
    .extract_memories("paladin-id", &conversation)
    .await?;
```

#### Extraction Strategies

```rust
pub enum MemoryExtractionStrategy {
    EveryTurn,                    // Extract after each interaction
    OnCompletion,                 // Extract when conversation ends
    Manual,                       // Explicit extraction calls
    Threshold { importance: f32 },  // Extract if importance >= threshold
}
```

**Strategy Recommendations**:
- **OnCompletion**: Best for most use cases (default)
- **EveryTurn**: For critical interactions needing immediate storage
- **Threshold**: For filtering low-importance content
- **Manual**: For custom extraction logic

#### Memory Quality

The extraction service uses LLM-based analysis to:
- Identify key facts and insights
- Categorize by memory type (Episodic/Semantic/Procedural)
- Assign importance scores (0.0-1.0)
- Add contextual metadata

### Paladin Integration

#### Programmatic Setup

```rust
use paladin::application::use_cases::paladin::paladin_execution_service::PaladinExecutionService;

// Create services
let rag_service = Arc::new(RagRetrievalService::new(
    sanctum_port, embedding_port, rag_config
));

let extraction_service = Arc::new(MemoryExtractionService::new(
    llm_port, embedding_port, sanctum_port
));

// Configure execution service with RAG
let execution_service = PaladinExecutionService::new(llm_port)
    .with_rag_retrieval(rag_service)
    .with_memory_extraction(extraction_service);

// Execute with automatic RAG
let result = execution_service.execute(&paladin, "user input").await?;
// ✓ Context automatically retrieved
// ✓ Response generated with historical context
// ✓ New memories extracted and stored
```

#### Configuration-based Setup

When using `config.yml`, RAG happens automatically:

```rust
// No code changes required!
// RAG is configured via config.yml and happens transparently
let result = paladin.execute("user input").await?;
```

### Performance Tuning

#### Retrieval Optimization

| Parameter       | Impact                  | Recommendation       |
|----------------|-------------------------|----------------------|
| top_k          | Context quality/cost    | Start with 5        |
| min_similarity | Relevance threshold     | 0.6-0.8 range       |
| max_tokens     | Context budget          | 1000-2000 tokens    |
| timeout        | Latency tolerance       | 5 seconds           |

**Trade-offs**:
- ↑ top_k → More context but slower and more expensive
- ↓ min_similarity → More memories but less relevant
- ↑ max_tokens → Better context but higher token costs

#### Extraction Optimization

**Batch Operations**: Extract memories in batches to reduce API calls

```rust
// Batch extract from multiple conversations
let all_conversations = vec![conv1, conv2, conv3];
for conversation in all_conversations {
    extraction_service.extract_memories(paladin_id, &conversation).await?;
}
```

**Duplicate Detection**: Automatic deduplication prevents redundant storage

**Importance Filtering**: Set minimum importance thresholds to reduce noise

### Example Workflow

#### Session 1: Building Knowledge Base

```rust
// First interaction - no prior context
let result1 = execution_service.execute(&paladin, "What is Rust?").await?;
// Output: "Rust is a systems programming language..."
// Memory stored: "Rust is a systems language focused on safety"

// Second interaction - retrieves first memory
let result2 = execution_service.execute(&paladin, "Tell me about ownership").await?;
// Context injected: Previous Rust definition
// Output: "Building on Rust's focus on safety, ownership is..."
// Memory stored: "Ownership prevents memory bugs"
```

#### Session 2: Using Knowledge

```rust
// New session - agent remembers previous learnings
let result3 = execution_service.execute(&paladin, "Explain memory management").await?;
// Context retrieved: Rust definition + ownership explanation
// Output: "Based on our earlier discussion about Rust's ownership..."
// ✓ Response quality improved with historical context
```

### Monitoring & Debugging

#### Enable Debug Logging

```rust
env_logger::init();  // Set RUST_LOG=debug
```

Logs include:
- Retrieval latency and result counts
- Memory extraction statistics
- Context injection details
- Error conditions and fallbacks

#### Metrics

Track these metrics for production:

```rust
// Retrieval metrics
- retrieval_latency_ms
- memories_retrieved_count
- similarity_scores_distribution

// Extraction metrics
- extraction_latency_ms
- memories_stored_count
- importance_scores_distribution

// Quality metrics
- context_injection_rate
- response_improvement_score
```

### Troubleshooting

#### No memories retrieved

**Causes**:
- Empty Sanctum (first interaction)
- Similarity threshold too high
- Embeddings not generated correctly

**Solutions**:
```yaml
rag:
  min_similarity: 0.5  # Lower threshold
  top_k: 10            # Increase candidates
```

#### Irrelevant context

**Causes**:
- Similarity threshold too low
- Poor embedding quality
- Noisy memory storage

**Solutions**:
```yaml
rag:
  min_similarity: 0.8  # Stricter threshold
  top_k: 3             # Fewer, better matches
```

#### Slow execution

**Causes**:
- Large top_k value
- Sanctum query latency
- Embedding generation delay

**Solutions**:
```yaml
rag:
  top_k: 3             # Reduce candidates
  timeout_seconds: 3   # Stricter timeout
```

### Best Practices

1. **Start Simple**: Use default configuration and adjust based on results
2. **Monitor Quality**: Track retrieval relevance and response improvement
3. **Tune Gradually**: Adjust one parameter at a time
4. **Test Thresholds**: Experiment with similarity values for your use case
5. **Production Setup**: Use Qdrant for scalability, in-memory for dev
6. **Error Handling**: RAG degrades gracefully if Sanctum unavailable
7. **Cost Management**: Balance top_k and max_tokens against API costs

### Example Code

See working examples:
- `examples/paladin_with_rag.rs` - RAG configuration demonstration
- `examples/paladin_with_sanctum.rs` - Memory operations
- `examples/cli_configs/paladin_rag.yaml` - Full configuration
- `tests/integration/rag_integration_tests.rs` - Configuration validation

---

## Best Practices

### 1. Memory Management

- Set appropriate importance scores (0.0-1.0)
- Use memory types correctly (Episodic/Semantic/Procedural)
- Add meaningful metadata for filtering
- Implement cleanup strategies for old memories

### 2. Embedding Quality

- Use consistent embedding models
- Ensure vector dimensions match configuration
- Normalize embeddings for better similarity scores
- Consider embedding model costs vs. quality trade-offs

### 3. Search Optimization

- Use filters to reduce search space
- Set reasonable top_k values (5-20 typical)
- Apply min_score thresholds (0.7+ for high relevance)
- Batch operations when possible

### 4. Production Deployment

- Use Qdrant for production workloads
- Monitor search latencies
- Implement proper backup strategies
- Use separate collections for different use cases
- Configure appropriate resource limits

### 5. Development Workflow

- Use InMemory for development
- Test with realistic data volumes
- Validate configuration before production
- Implement graceful degradation if Sanctum unavailable

## Troubleshooting

### Common Issues

#### 1. Dimension Mismatch
**Error**: `InvalidDimension: Expected 1536 dimensions, got 768`

**Solution**: Ensure embedding model matches configured dimension:
```yaml
qdrant:
  vector_dimension: 768  # Match your model's output
```

#### 2. Qdrant Connection Failed
**Error**: `StorageError: Failed to connect to Qdrant`

**Solution**: Verify Qdrant is running and accessible:
```bash
curl http://localhost:6333/health
```

#### 3. Slow Search Performance
**Symptom**: Search takes >1 second

**Solutions**:
- Reduce top_k value
- Add filters to narrow search space
- Check Qdrant resource allocation
- Consider upgrading to Qdrant cluster

#### 4. Memory Not Found After Insert
**Issue**: Inserted memory not immediately searchable

**Solution**: Qdrant indexes asynchronously. Add small delay:
```rust
sanctum.store(entry).await?;
tokio::time::sleep(Duration::from_millis(100)).await;
// Now searchable
```

## Additional Resources

- [Sanctum Architecture](../notes/hexagonal-arch.md)
- [Qdrant Documentation](https://qdrant.tech/documentation/)
- [OpenAI Embeddings](https://platform.openai.com/docs/guides/embeddings)
- [Vector Search Tutorial](./guides/vector-search.md)

## Support

For issues, questions, or contributions:
- GitHub Issues: [paladin-dev-env/issues](https://github.com/DF3NDR/paladin-dev-env/issues)
- Discussions: [paladin-dev-env/discussions](https://github.com/DF3NDR/paladin-dev-env/discussions)

---

**Next Steps**: 
- Review [Configuration Examples](../examples/garrison_persistent.rs)
- Explore [Deployment Guide](./DEPLOYMENT.md)
- Read [Migration Guide](./MIGRATION.md)
