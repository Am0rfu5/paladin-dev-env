# Hexagonal Architecture in Paladin

This document provides a detailed explanation of how Paladin implements Hexagonal Architecture (also known as Ports and Adapters pattern).

## Table of Contents

- [Overview](#overview)
- [Core Concepts](#core-concepts)
- [Port Definitions](#port-definitions)
- [Adapter Implementations](#adapter-implementations)
- [Dependency Flow](#dependency-flow)
- [Port-Adapter Mapping](#port-adapter-mapping)
- [Benefits](#benefits)
- [Implementation Patterns](#implementation-patterns)
- [Testing Strategy](#testing-strategy)

## Overview

Hexagonal Architecture organizes code into three concentric layers:

```
╔════════════════════════════════════════════════════╗
║            External Systems & Actors               ║
║  (LLMs, Databases, File Systems, APIs, Users)     ║
╚═══════════════════╤════════════════════════════════╝
                    │
        ┌───────────┴───────────┐
        │                       │
╔═══════▼═══════╗      ╔════════▼═══════╗
║   Adapters    ║      ║    Adapters    ║
║  (Driving)    ║      ║    (Driven)    ║
║  CLI, API     ║      ║ OpenAI, SQLite ║
╚═══════╤═══════╝      ╚════════╤═══════╝
        │                       │
        │    ┌─────────────────┬┘
        │    │                 │
╔═══════▼════▼═════╗  ╔════════▼═══════╗
║   Input Ports    ║  ║  Output Ports  ║
║  (Interfaces)    ║  ║  (Interfaces)  ║
╚═══════╤══════════╝  ╚════════╤═══════╝
        │                      │
        │   ┌──────────────────┘
        │   │
╔═══════▼═══▼══════════════════════════╗
║        Application Layer              ║
║     (Use Cases & Services)            ║
╚═══════════════╤═══════════════════════╝
                │
╔═══════════════▼═══════════════════════╗
║          Core Domain                  ║
║  (Paladin, Battalion, Garrison, etc.) ║
║     Pure Business Logic               ║
╚═══════════════════════════════════════╝
```

**Key Principles:**
1. **Core is independent**: No dependencies on frameworks or external systems
2. **Ports define contracts**: Interfaces specify what the application needs
3. **Adapters implement contracts**: Concrete implementations of external systems
4. **Dependencies point inward**: Infrastructure depends on application, not vice versa

## Core Concepts

### 1. Core Domain (Center of the Hexagon)

The innermost layer containing pure business logic.

**Location**: `src/core/`

**Characteristics:**
- Zero external dependencies (except serialization)
- No I/O operations
- No framework coupling
- Pure functions and data structures

**Example - Paladin Entity:**
```rust
// src/core/platform/container/paladin.rs

/// Paladin domain entity - pure business logic
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaladinData {
    pub system_prompt: String,
    pub name: String,
    pub model: String,
    pub temperature: f32,
    pub max_loops: u32,
    pub stop_words: Vec<String>,
    pub status: PaladinStatus,
}

pub type Paladin = Node<PaladinData>;

impl PaladinData {
    /// Business rule: validate configuration
    pub fn validate(&self) -> Result<(), PaladinError> {
        if self.system_prompt.is_empty() {
            return Err(PaladinError::ConfigurationError(
                "System prompt is required".into()
            ));
        }
        
        if !(0.0..=2.0).contains(&self.temperature) {
            return Err(PaladinError::ConfigurationError(
                "Temperature must be between 0.0 and 2.0".into()
            ));
        }
        
        Ok(())
    }
}
```

### 2. Ports (Boundaries of the Hexagon)

Interfaces (traits) defining contracts between layers.

**Location**: `src/application/ports/`

**Types:**
- **Input Ports (Driving)**: How external actors use the application
- **Output Ports (Driven)**: What the application needs from external systems

**Example - Output Port:**
```rust
// src/application/ports/output/llm_port.rs

/// Port for LLM provider integration
#[async_trait]
pub trait LlmPort: Send + Sync {
    /// Generate completion from prompt
    async fn generate(
        &self, 
        prompt: &PromptItem
    ) -> Result<LlmResponse, LlmError>;
    
    /// Generate with streaming
    async fn generate_stream(
        &self, 
        prompt: &PromptItem
    ) -> Result<Pin<Box<dyn Stream<Item = Result<LlmChunk>>>>, LlmError>;
    
    /// Validate model is available
    fn validate_model(&self, model: &str) -> Result<(), LlmError>;
    
    /// Get model capabilities
    fn capabilities(&self) -> ModelCapabilities;
}

/// Request structure for LLM
#[derive(Debug, Clone)]
pub struct PromptItem {
    pub messages: Vec<Message>,
    pub model: String,
    pub temperature: f32,
    pub max_tokens: Option<u32>,
    pub tools: Vec<ToolDefinition>,
}

/// Response from LLM
#[derive(Debug, Clone)]
pub struct LlmResponse {
    pub content: String,
    pub tool_calls: Vec<ToolCall>,
    pub finish_reason: FinishReason,
    pub token_usage: TokenUsage,
}
```

### 3. Adapters (Outside the Hexagon)

Concrete implementations of ports for specific technologies.

**Location**: `src/infrastructure/adapters/`

**Example - OpenAI Adapter:**
```rust
// src/infrastructure/adapters/llm/openai_adapter.rs

/// OpenAI implementation of LlmPort
pub struct OpenAiAdapter {
    client: reqwest::Client,
    api_key: String,
    base_url: String,
    default_model: String,
}

#[async_trait]
impl LlmPort for OpenAiAdapter {
    async fn generate(
        &self, 
        prompt: &PromptItem
    ) -> Result<LlmResponse, LlmError> {
        // Convert application model to OpenAI API format
        let request = OpenAiChatRequest {
            model: prompt.model.clone(),
            messages: self.convert_messages(&prompt.messages),
            temperature: prompt.temperature,
            max_tokens: prompt.max_tokens,
            tools: self.convert_tools(&prompt.tools),
        };
        
        // Make HTTP request to OpenAI API
        let response = self.client
            .post(&format!("{}/chat/completions", self.base_url))
            .bearer_auth(&self.api_key)
            .json(&request)
            .send()
            .await
            .map_err(|e| LlmError::NetworkError(e.to_string()))?;
        
        // Check for errors
        if !response.status().is_success() {
            let error: OpenAiError = response.json().await
                .map_err(|e| LlmError::ParseError(e.to_string()))?;
            return Err(LlmError::ProviderError(error.message));
        }
        
        // Parse OpenAI response
        let openai_response: OpenAiChatResponse = response.json().await
            .map_err(|e| LlmError::ParseError(e.to_string()))?;
        
        // Convert OpenAI format back to application model
        Ok(self.convert_response(openai_response))
    }
    
    // ... other trait methods
}
```

## Port Definitions

### Input Ports (Driving Side)

Define how external actors interact with the application.

```rust
// src/application/ports/input/content_ingestion_port.rs

/// Port for content ingestion use cases
#[async_trait]
pub trait ContentIngestionPort: Send + Sync {
    /// Ingest new content item
    async fn ingest(
        &self, 
        content: ContentItem
    ) -> Result<ContentId, IngestionError>;
    
    /// Get ingestion status
    async fn status(
        &self, 
        id: ContentId
    ) -> Result<IngestionStatus, IngestionError>;
}
```

**Implementation** (in application layer):
```rust
// src/application/use_cases/content/ingestion_service.rs

pub struct ContentIngestionService {
    repository: Arc<dyn ContentRepository>,
    ml_service: Arc<dyn MlPort>,
}

#[async_trait]
impl ContentIngestionPort for ContentIngestionService {
    async fn ingest(
        &self, 
        content: ContentItem
    ) -> Result<ContentId, IngestionError> {
        // Use case logic
        let id = self.repository.save(content).await?;
        self.ml_service.analyze(id).await?;
        Ok(id)
    }
    
    // ... other methods
}
```

### Output Ports (Driven Side)

Define what the application needs from external systems.

#### LlmPort - LLM Provider Integration

```rust
// src/application/ports/output/llm_port.rs

#[async_trait]
pub trait LlmPort: Send + Sync {
    async fn generate(&self, prompt: &PromptItem) -> Result<LlmResponse, LlmError>;
    async fn generate_stream(&self, prompt: &PromptItem) -> Result<LlmStream, LlmError>;
    fn validate_model(&self, model: &str) -> Result<(), LlmError>;
    fn capabilities(&self) -> ModelCapabilities;
}
```

**Adapters:**
- `OpenAiAdapter` - OpenAI API
- `DeepSeekAdapter` - DeepSeek API
- `AnthropicAdapter` - Anthropic API

#### GarrisonPort - Memory Storage

```rust
// src/application/ports/output/garrison_port.rs

#[async_trait]
pub trait GarrisonPort: Send + Sync {
    async fn add_entry(&self, entry: GarrisonEntry) -> Result<(), GarrisonError>;
    async fn get_history(&self, limit: usize) -> Result<Vec<GarrisonEntry>, GarrisonError>;
    async fn get_window(&self, max_tokens: u32) -> Result<Vec<GarrisonEntry>, GarrisonError>;
    async fn search(&self, query: &str, limit: usize) -> Result<Vec<GarrisonEntry>, GarrisonError>;
    async fn clear(&self) -> Result<(), GarrisonError>;
}
```

**Adapters:**
- `InMemoryGarrison` - RAM storage
- `SqliteGarrison` - SQLite persistence
- `PostgresGarrison` - PostgreSQL persistence

#### ArsenalPort - Tool Execution

```rust
// src/application/ports/output/arsenal_port.rs

#[async_trait]
pub trait ArsenalPort: Send + Sync {
    async fn list_tools(&self) -> Result<Vec<Armament>, ArsenalError>;
    async fn invoke(&self, call: &ArmamentCall) -> Result<ArmamentResult, ArsenalError>;
    fn validate_call(&self, call: &ArmamentCall) -> Result<(), ArsenalError>;
}
```

**Adapters:**
- `MCPStdioAdapter` - MCP STDIO protocol
- `MCPSseAdapter` - MCP SSE protocol
- `CustomToolAdapter` - Native Rust tools

#### FileStoragePort - File Persistence

```rust
// src/application/ports/output/file_storage_port.rs

#[async_trait]
pub trait FileStoragePort: Send + Sync {
    async fn upload(&self, path: &str, data: Vec<u8>) -> Result<String, StorageError>;
    async fn download(&self, path: &str) -> Result<Vec<u8>, StorageError>;
    async fn delete(&self, path: &str) -> Result<(), StorageError>;
    async fn exists(&self, path: &str) -> Result<bool, StorageError>;
}
```

**Adapters:**
- `MinioAdapter` - MinIO/S3-compatible storage
- `LocalFileAdapter` - Local filesystem

## Adapter Implementations

### Pattern: Adapter Structure

All adapters follow a consistent structure:

```rust
pub struct AdapterName {
    // Client or connection
    client: ClientType,
    
    // Configuration
    config: AdapterConfig,
    
    // Shared state (if needed)
    state: Arc<RwLock<State>>,
}

impl AdapterName {
    // Constructor
    pub fn new(config: AdapterConfig) -> Self {
        Self {
            client: ClientType::new(),
            config,
            state: Arc::new(RwLock::new(State::default())),
        }
    }
    
    // Builder pattern
    pub fn builder() -> AdapterBuilder {
        AdapterBuilder::default()
    }
    
    // Helper methods (private)
    fn convert_request(&self, app_model: &AppType) -> ApiType {
        // Convert application model to API model
    }
    
    fn convert_response(&self, api_model: ApiType) -> AppType {
        // Convert API model to application model
    }
}

// Implement the port trait
#[async_trait]
impl PortTrait for AdapterName {
    async fn method(&self, input: &Input) -> Result<Output, Error> {
        // Implementation
    }
}
```

### Example: Multiple Adapters for Same Port

```rust
// Port definition
#[async_trait]
pub trait GarrisonPort: Send + Sync {
    async fn add_entry(&self, entry: GarrisonEntry) -> Result<()>;
    async fn get_history(&self, limit: usize) -> Result<Vec<GarrisonEntry>>;
}

// Adapter 1: In-memory
pub struct InMemoryGarrison {
    entries: RwLock<VecDeque<GarrisonEntry>>,
    max_entries: usize,
}

#[async_trait]
impl GarrisonPort for InMemoryGarrison {
    async fn add_entry(&self, entry: GarrisonEntry) -> Result<()> {
        let mut entries = self.entries.write().await;
        
        if entries.len() >= self.max_entries {
            entries.pop_front();
        }
        
        entries.push_back(entry);
        Ok(())
    }
    
    async fn get_history(&self, limit: usize) -> Result<Vec<GarrisonEntry>> {
        let entries = self.entries.read().await;
        Ok(entries.iter()
            .rev()
            .take(limit)
            .cloned()
            .collect())
    }
}

// Adapter 2: SQLite
pub struct SqliteGarrison {
    pool: SqlitePool,
    session_id: Uuid,
}

#[async_trait]
impl GarrisonPort for SqliteGarrison {
    async fn add_entry(&self, entry: GarrisonEntry) -> Result<()> {
        sqlx::query(
            "INSERT INTO garrison_entries (id, session_id, role, content, timestamp) 
             VALUES (?, ?, ?, ?, ?)"
        )
        .bind(entry.id.to_string())
        .bind(self.session_id.to_string())
        .bind(entry.role.to_string())
        .bind(&entry.content)
        .bind(entry.timestamp.timestamp())
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }
    
    async fn get_history(&self, limit: usize) -> Result<Vec<GarrisonEntry>> {
        let rows = sqlx::query_as::<_, GarrisonEntry>(
            "SELECT * FROM garrison_entries 
             WHERE session_id = ? 
             ORDER BY timestamp DESC 
             LIMIT ?"
        )
        .bind(self.session_id.to_string())
        .bind(limit as i64)
        .fetch_all(&self.pool)
        .await?;
        
        Ok(rows)
    }
}

// Usage - easily swap implementations
let garrison: Arc<dyn GarrisonPort> = if persistent {
    Arc::new(SqliteGarrison::new("garrison.db").await?)
} else {
    Arc::new(InMemoryGarrison::new(100))
};
```

## Dependency Flow

### Strict Dependency Rules

```
┌────────────────────────────────────────┐
│        Infrastructure Layer            │
│    (Adapters for LLMs, DBs, etc.)     │
│                                        │
│   Can import from:                     │
│   ✓ Application (ports)                │
│   ✓ Core (entities)                    │
└────────────────────────────────────────┘
                  ▲
                  │ depends on
                  │
┌────────────────────────────────────────┐
│        Application Layer               │
│     (Use Cases, Ports, Services)       │
│                                        │
│   Can import from:                     │
│   ✓ Core (entities)                    │
│   ✗ Infrastructure                     │
└────────────────────────────────────────┘
                  ▲
                  │ depends on
                  │
┌────────────────────────────────────────┐
│            Core Layer                  │
│      (Domain Entities & Logic)         │
│                                        │
│   Can import from:                     │
│   ✓ std library                        │
│   ✓ serde (serialization only)        │
│   ✗ Application                        │
│   ✗ Infrastructure                     │
└────────────────────────────────────────┘
```

### Enforcing Dependency Rules

```rust
// ❌ WRONG - Core importing from Application
// src/core/platform/container/paladin.rs
use crate::paladin_ports::output::llm_port::LlmPort; // ERROR!

pub struct Paladin {
    llm: Arc<dyn LlmPort>, // Core shouldn't know about LlmPort
}

// ✅ CORRECT - Application uses Core
// src/application/use_cases/paladin/paladin_execution_service.rs
use crate::core::platform::container::paladin::Paladin;
use crate::paladin_ports::output::llm_port::LlmPort;

pub struct PaladinExecutionService {
    llm_port: Arc<dyn LlmPort>,
}

impl PaladinExecutionService {
    pub async fn execute(&self, paladin: &Paladin, input: &str) -> Result<String> {
        // Service orchestrates core entities using ports
    }
}

// ✅ CORRECT - Infrastructure implements Application ports
// src/infrastructure/adapters/llm/openai_adapter.rs
use crate::paladin_ports::output::llm_port::LlmPort;

pub struct OpenAiAdapter {
    // ...
}

#[async_trait]
impl LlmPort for OpenAiAdapter {
    // Implementation
}
```

## Port-Adapter Mapping

Complete mapping of all ports to their adapters:

### LLM Provider Ports

| Port | Adapters | Purpose |
|------|----------|---------|
| `LlmPort` | `OpenAiAdapter`<br>`DeepSeekAdapter`<br>`AnthropicAdapter` | LLM completion generation |

### Storage Ports

| Port | Adapters | Purpose |
|------|----------|---------|
| `GarrisonPort` | `InMemoryGarrison`<br>`SqliteGarrison` | Conversation memory storage |
| `FileStoragePort` | `MinioAdapter`<br>`LocalFileAdapter` | File persistence |
| `CitadelPort` | `FileCitadel`<br>`S3Citadel` | State checkpoint storage |

### Tool Ports

| Port | Adapters | Purpose |
|------|----------|---------|
| `ArsenalPort` | `MCPStdioAdapter`<br>`MCPSseAdapter`<br>`CustomToolAdapter` | Tool execution |

### Queue Ports

| Port | Adapters | Purpose |
|------|----------|---------|
| `QueuePort` | `RedisAdapter`<br>`InMemoryQueue` | Async task queueing |

### Repository Ports

| Port | Adapters | Purpose |
|------|----------|---------|
| `ContentRepository` | `MySqlRepository`<br>`SqliteRepository` | Content persistence |
| `UserRepository` | `MySqlRepository`<br>`SqliteRepository` | User data |

## Benefits

### 1. Testability

Mock adapters for testing without external dependencies:

```rust
// Mock LLM adapter for testing
pub struct MockLlmAdapter {
    responses: VecDeque<String>,
}

#[async_trait]
impl LlmPort for MockLlmAdapter {
    async fn generate(&self, _prompt: &PromptItem) -> Result<LlmResponse> {
        let content = self.responses.pop_front().unwrap_or_default();
        Ok(LlmResponse {
            content,
            tool_calls: vec![],
            finish_reason: FinishReason::Stop,
            token_usage: TokenUsage::default(),
        })
    }
    
    // ... other methods
}

// Test without real LLM calls
#[tokio::test]
async fn test_paladin_execution() {
    let mock_llm = Arc::new(MockLlmAdapter::new(vec![
        "Hello, user!".to_string(),
    ]));
    
    let service = PaladinExecutionService::new(mock_llm);
    let paladin = create_test_paladin();
    
    let result = service.execute(&paladin, "Hi").await.unwrap();
    assert_eq!(result.content, "Hello, user!");
}
```

### 2. Flexibility

Swap implementations easily:

```rust
// Development: use in-memory storage
let garrison: Arc<dyn GarrisonPort> = Arc::new(InMemoryGarrison::new(100));

// Production: use persistent storage
let garrison: Arc<dyn GarrisonPort> = Arc::new(
    SqliteGarrison::new("garrison.db").await?
);

// Code using garrison doesn't change
let paladin = PaladinBuilder::new(llm_adapter)
    .with_garrison(garrison)
    .build()?;
```

### 3. Maintainability

Changes to external systems don't affect business logic:

```rust
// If OpenAI changes their API, we only update the adapter
impl LlmPort for OpenAiAdapter {
    async fn generate(&self, prompt: &PromptItem) -> Result<LlmResponse> {
        // API changed from v1 to v2
        let request = self.build_v2_request(prompt)?; // Only change here
        
        // Rest of application unaffected
        let response = self.client.post(&self.v2_endpoint)
            .json(&request)
            .send()
            .await?;
        
        Ok(self.convert_response(response))
    }
}
```

### 4. Independent Development

Teams can work on different layers simultaneously:

- **Core team**: Implements business logic
- **Infrastructure team**: Builds adapters
- **Testing team**: Creates mock adapters

All work in parallel without blocking each other.

## Implementation Patterns

### Pattern 1: Builder for Adapters

```rust
pub struct OpenAiAdapterBuilder {
    api_key: Option<String>,
    base_url: String,
    model: String,
    timeout: Duration,
}

impl OpenAiAdapterBuilder {
    pub fn new() -> Self {
        Self {
            api_key: None,
            base_url: "https://api.openai.com/v1".to_string(),
            model: "gpt-4".to_string(),
            timeout: Duration::from_secs(30),
        }
    }
    
    pub fn api_key(mut self, key: impl Into<String>) -> Self {
        self.api_key = Some(key.into());
        self
    }
    
    pub fn base_url(mut self, url: impl Into<String>) -> Self {
        self.base_url = url.into();
        self
    }
    
    pub fn build(self) -> Result<OpenAiAdapter, AdapterError> {
        let api_key = self.api_key
            .ok_or_else(|| AdapterError::MissingConfiguration("api_key"))?;
        
        Ok(OpenAiAdapter {
            client: reqwest::Client::builder()
                .timeout(self.timeout)
                .build()?,
            api_key,
            base_url: self.base_url,
            default_model: self.model,
        })
    }
}

// Usage
let adapter = OpenAiAdapter::builder()
    .api_key(env::var("OPENAI_API_KEY")?)
    .model("gpt-4-turbo")
    .build()?;
```

### Pattern 2: Adapter Registry

```rust
pub struct AdapterRegistry {
    llm_adapters: HashMap<String, Arc<dyn LlmPort>>,
    storage_adapters: HashMap<String, Arc<dyn FileStoragePort>>,
}

impl AdapterRegistry {
    pub fn new() -> Self {
        Self {
            llm_adapters: HashMap::new(),
            storage_adapters: HashMap::new(),
        }
    }
    
    pub fn register_llm(&mut self, name: &str, adapter: Arc<dyn LlmPort>) {
        self.llm_adapters.insert(name.to_string(), adapter);
    }
    
    pub fn get_llm(&self, name: &str) -> Option<&Arc<dyn LlmPort>> {
        self.llm_adapters.get(name)
    }
}

// Usage
let mut registry = AdapterRegistry::new();

registry.register_llm("openai", Arc::new(openai_adapter));
registry.register_llm("deepseek", Arc::new(deepseek_adapter));

let adapter = registry.get_llm("openai").unwrap();
```

### Pattern 3: Fallback Chain

```rust
pub struct FallbackLlmAdapter {
    primary: Arc<dyn LlmPort>,
    fallback: Arc<dyn LlmPort>,
}

#[async_trait]
impl LlmPort for FallbackLlmAdapter {
    async fn generate(&self, prompt: &PromptItem) -> Result<LlmResponse> {
        match self.primary.generate(prompt).await {
            Ok(response) => Ok(response),
            Err(e) => {
                warn!("Primary LLM failed: {}. Trying fallback.", e);
                self.fallback.generate(prompt).await
            }
        }
    }
}

// Usage
let primary = Arc::new(OpenAiAdapter::builder().build()?);
let fallback = Arc::new(DeepSeekAdapter::builder().build()?);

let adapter: Arc<dyn LlmPort> = Arc::new(FallbackLlmAdapter {
    primary,
    fallback,
});
```

## Testing Strategy

### Unit Tests (Core Layer)

Test business logic without any adapters:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_paladin_validation() {
        let data = PaladinData {
            system_prompt: "".to_string(), // Invalid!
            name: "Test".to_string(),
            model: "gpt-4".to_string(),
            temperature: 0.7,
            max_loops: 3,
            stop_words: vec![],
            status: PaladinStatus::Idle,
        };
        
        assert!(data.validate().is_err());
    }
}
```

### Integration Tests (With Mock Adapters)

Test application layer with mocked ports:

```rust
#[tokio::test]
async fn test_paladin_execution_service() {
    // Mock LLM adapter
    let mock_llm = Arc::new(MockLlmAdapter::new(vec![
        "Response 1".to_string(),
    ]));
    
    // Mock garrison
    let mock_garrison = Arc::new(MockGarrison::new());
    
    // Create service with mocks
    let service = PaladinExecutionService::new(
        mock_llm,
        Some(mock_garrison.clone()),
        Arc::new(ArsenalRegistry::new()),
    );
    
    // Test
    let paladin = create_test_paladin();
    let result = service.execute(&paladin, "Test input").await.unwrap();
    
    assert_eq!(result.content, "Response 1");
    assert_eq!(mock_garrison.entry_count(), 2); // user + assistant
}
```

### End-to-End Tests (With Real Adapters)

Test complete system with real implementations:

```rust
#[tokio::test]
#[ignore] // Requires API key
async fn test_openai_adapter() {
    let api_key = env::var("OPENAI_API_KEY").unwrap();
    
    let adapter = OpenAiAdapter::builder()
        .api_key(api_key)
        .build()
        .unwrap();
    
    let prompt = PromptItem {
        messages: vec![Message {
            role: Role::User,
            content: "Say hello".to_string(),
        }],
        model: "gpt-4".to_string(),
        temperature: 0.7,
        max_tokens: Some(50),
        tools: vec![],
    };
    
    let response = adapter.generate(&prompt).await.unwrap();
    
    assert!(!response.content.is_empty());
}
```

## Best Practices

### 1. Keep Ports Simple

```rust
// ❌ Bad: Port that's too specific to one adapter
#[async_trait]
pub trait LlmPort {
    async fn generate_with_openai_specific_feature(&self, /* ... */);
}

// ✅ Good: Generic port that any LLM can implement
#[async_trait]
pub trait LlmPort {
    async fn generate(&self, prompt: &PromptItem) -> Result<LlmResponse>;
}
```

### 2. Use Domain Types in Ports

```rust
// ❌ Bad: Using adapter-specific types in port
#[async_trait]
pub trait LlmPort {
    async fn generate(&self, request: OpenAiRequest) -> Result<OpenAiResponse>;
}

// ✅ Good: Using domain types
#[async_trait]
pub trait LlmPort {
    async fn generate(&self, prompt: &PromptItem) -> Result<LlmResponse>;
}
```

### 3. Error Handling Across Boundaries

```rust
// Application error type
#[derive(Debug, thiserror::Error)]
pub enum LlmError {
    #[error("Network error: {0}")]
    NetworkError(String),
    
    #[error("Provider error: {0}")]
    ProviderError(String),
    
    #[error("Invalid response: {0}")]
    ParseError(String),
}

// Adapter converts specific errors to application errors
impl From<reqwest::Error> for LlmError {
    fn from(err: reqwest::Error) -> Self {
        LlmError::NetworkError(err.to_string())
    }
}
```

## Next Steps

- **[Domain Model](domain-model.md)** - DDD entities and relationships
- **[Design Patterns](design-patterns.md)** - Patterns used in Paladin
- **[Adapter Development](../contributing/adapter-development.md)** - Create custom adapters
