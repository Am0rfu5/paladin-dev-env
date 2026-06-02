# Paladin Design Patterns

This document describes the key design patterns used throughout the Paladin codebase, with implementation examples and best practices.

## Table of Contents

- [Overview](#overview)
- [Structural Patterns](#structural-patterns)
- [Creational Patterns](#creational-patterns)
- [Behavioral Patterns](#behavioral-patterns)
- [Architectural Patterns](#architectural-patterns)
- [Pattern Guidelines](#pattern-guidelines)

## Overview

Paladin uses well-established design patterns to achieve:
- **Maintainability**: Clear, consistent code structure
- **Testability**: Patterns that facilitate unit and integration testing
- **Extensibility**: Easy addition of new providers, tools, and patterns
- **Type Safety**: Leveraging Rust's type system for compile-time guarantees

## Structural Patterns

### 1. Node<T> Pattern

**Purpose**: Provide a consistent wrapper for domain entities with metadata.

**Structure**:
```rust
/// Generic node wrapper for domain entities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Node<T> {
    pub id: Uuid,
    pub data: T,
    pub metadata: Metadata,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl<T> Node<T> {
    pub fn new(data: T) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            data,
            metadata: Metadata::default(),
            created_at: now,
            updated_at: now,
        }
    }

    pub fn with_id(mut self, id: Uuid) -> Self {
        self.id = id;
        self
    }

    pub fn with_metadata(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.metadata.insert(key.into(), value.into());
        self
    }
}
```

**Usage**:
```rust
/// Paladin uses Node pattern
pub type Paladin = Node<PaladinData>;

/// Creating a Paladin
let paladin = Node::new(PaladinData {
    system_prompt: "You are a helpful assistant".into(),
    name: "Helper".into(),
    // ... other fields
})
.with_metadata("version", "1.0")
.with_metadata("environment", "production");
```

**Benefits**:
- Consistent ID management across entities
- Built-in timestamps for auditing
- Extensible metadata without schema changes
- Generic implementation reused across domain

### 2. Port/Adapter Pattern (Hexagonal Architecture)

**Purpose**: Decouple core business logic from external dependencies.

**Structure**:
```
┌─────────────────────────────────────────┐
│         Application Core                │
│                                          │
│   ┌──────────────────────────────┐     │
│   │    Port (Trait)              │     │
│   │    pub trait LlmPort {       │     │
│   │      fn generate(...) -> ... │     │
│   │    }                          │     │
│   └──────────────────────────────┘     │
│                                          │
└──────────────────┬───────────────────────┘
                   │
                   │ implements
                   │
┌──────────────────▼───────────────────────┐
│         Infrastructure Layer             │
│                                          │
│   ┌──────────────────────────────┐     │
│   │    Adapter (Implementation)   │     │
│   │    pub struct OpenAiAdapter { │     │
│   │      // ... fields            │     │
│   │    }                          │     │
│   │    impl LlmPort for OpenAi...│     │
│   └──────────────────────────────┘     │
└──────────────────────────────────────────┘
```

**Port Definition**:
```rust
// application/ports/output/llm_port.rs
#[async_trait]
pub trait LlmPort: Send + Sync {
    async fn generate(
        &self,
        model: &str,
        messages: &[Message],
        temperature: f32,
    ) -> Result<LlmResponse, LlmError>;

    async fn generate_stream(
        &self,
        model: &str,
        messages: &[Message],
        temperature: f32,
    ) -> Result<Pin<Box<dyn Stream<Item = LlmChunk> + Send>>, LlmError>;

    fn supports_tools(&self, model: &str) -> bool;
}
```

**Adapter Implementation**:
```rust
// infrastructure/adapters/llm/openai_adapter.rs
pub struct OpenAiAdapter {
    api_key: String,
    base_url: String,
    client: reqwest::Client,
}

#[async_trait]
impl LlmPort for OpenAiAdapter {
    async fn generate(
        &self,
        model: &str,
        messages: &[Message],
        temperature: f32,
    ) -> Result<LlmResponse, LlmError> {
        let request = OpenAiRequest {
            model: model.to_string(),
            messages: messages.iter().map(|m| m.into()).collect(),
            temperature,
        };

        let response = self.client
            .post(&format!("{}/chat/completions", self.base_url))
            .bearer_auth(&self.api_key)
            .json(&request)
            .send()
            .await?;

        let openai_response: OpenAiResponse = response.json().await?;
        Ok(openai_response.into())
    }

    // ... other methods
}
```

**Benefits**:
- Easy to swap implementations (OpenAI → Anthropic)
- Testability with mock adapters
- Clear dependency boundaries

### 3. Adapter Registry Pattern

**Purpose**: Manage multiple adapters with runtime selection.

**Structure**:
```rust
/// Registry for managing multiple adapters
pub struct AdapterRegistry<P: ?Sized> {
    adapters: HashMap<String, Arc<P>>,
    default: Option<Arc<P>>,
}

impl<P: ?Sized> AdapterRegistry<P> {
    pub fn new() -> Self {
        Self {
            adapters: HashMap::new(),
            default: None,
        }
    }

    pub fn register(&mut self, name: impl Into<String>, adapter: Arc<P>) {
        self.adapters.insert(name.into(), adapter);
    }

    pub fn set_default(&mut self, adapter: Arc<P>) {
        self.default = Some(adapter);
    }

    pub fn get(&self, name: &str) -> Option<Arc<P>> {
        self.adapters.get(name).cloned()
    }

    pub fn get_or_default(&self, name: &str) -> Option<Arc<P>> {
        self.get(name).or_else(|| self.default.clone())
    }
}
```

**Usage**:
```rust
// Create registry for LLM providers
let mut llm_registry: AdapterRegistry<dyn LlmPort> = AdapterRegistry::new();

// Register adapters
llm_registry.register("openai", Arc::new(openai_adapter));
llm_registry.register("anthropic", Arc::new(anthropic_adapter));
llm_registry.set_default(Arc::new(openai_adapter));

// Use at runtime
let provider = config.llm_provider.as_deref().unwrap_or("openai");
let llm = llm_registry.get_or_default(provider)
    .ok_or_else(|| Error::ProviderNotFound(provider.into()))?;
```

**Benefits**:
- Dynamic provider selection
- Centralized adapter management
- Fallback to default adapter

## Creational Patterns

### 1. Builder Pattern

**Purpose**: Construct complex objects step-by-step with validation.

**Structure**:
```rust
/// Paladin builder
pub struct PaladinBuilder {
    llm_port: Arc<dyn LlmPort>,
    data: PaladinData,
    config: PaladinConfig,
    garrison: Option<Arc<dyn GarrisonPort>>,
    arsenal: Vec<Arc<dyn ArsenalPort>>,
}

impl PaladinBuilder {
    pub fn new(llm_port: Arc<dyn LlmPort>) -> Self {
        Self {
            llm_port,
            data: PaladinData::default(),
            config: PaladinConfig::default(),
            garrison: None,
            arsenal: Vec::new(),
        }
    }

    /// Set system prompt
    pub fn system_prompt(mut self, prompt: impl Into<String>) -> Self {
        self.data.system_prompt = prompt.into();
        self
    }

    /// Set Paladin name
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.data.name = name.into();
        self
    }

    /// Set temperature
    pub fn temperature(mut self, temp: f32) -> Self {
        self.data.temperature = temp;
        self
    }

    /// Set max loops
    pub fn max_loops(mut self, loops: u32) -> Self {
        self.data.max_loops = loops;
        self
    }

    /// Add stop word
    pub fn stop_word(mut self, word: impl Into<String>) -> Self {
        self.data.stop_words.push(word.into());
        self
    }

    /// Attach garrison for memory
    pub fn with_garrison(mut self, garrison: Arc<dyn GarrisonPort>) -> Self {
        self.garrison = Some(garrison);
        self
    }

    /// Add tool to arsenal
    pub fn add_armament(mut self, armament: Arc<dyn ArsenalPort>) -> Self {
        self.arsenal.push(armament);
        self
    }

    /// Build final Paladin with validation
    pub fn build(self) -> Result<Paladin, PaladinError> {
        self.validate()?;

        let data = self.data;
        let mut paladin = Node::new(data);

        // Attach ports
        if let Some(garrison) = self.garrison {
            paladin = paladin.with_metadata("garrison", "enabled");
        }

        if !self.arsenal.is_empty() {
            paladin = paladin.with_metadata("arsenal_count", self.arsenal.len().to_string());
        }

        Ok(paladin)
    }

    fn validate(&self) -> Result<(), PaladinError> {
        if self.data.system_prompt.is_empty() {
            return Err(PaladinError::ConfigurationError(
                "System prompt is required".into()
            ));
        }

        if !(0.0..=2.0).contains(&self.data.temperature) {
            return Err(PaladinError::ConfigurationError(
                format!("Temperature {} must be between 0.0 and 2.0", self.data.temperature)
            ));
        }

        if self.data.max_loops == 0 {
            return Err(PaladinError::ConfigurationError(
                "max_loops must be greater than 0".into()
            ));
        }

        Ok(())
    }
}
```

**Usage**:
```rust
let paladin = PaladinBuilder::new(llm_port)
    .name("Research Assistant")
    .system_prompt("You are an expert researcher")
    .temperature(0.7)
    .max_loops(5)
    .stop_word("DONE")
    .with_garrison(garrison_port)
    .add_armament(web_search_tool)
    .add_armament(calculator_tool)
    .build()?;
```

**Benefits**:
- Fluent, readable API
- Validation before construction
- Default values for optional fields
- Type-safe construction

### 2. Factory Pattern

**Purpose**: Create objects based on configuration or type.

**Structure**:
```rust
/// Factory for creating Garrison implementations
pub struct GarrisonFactory;

impl GarrisonFactory {
    pub fn create(
        config: &GarrisonConfig
    ) -> Result<Arc<dyn GarrisonPort>, GarrisonError> {
        match config.storage_type.as_str() {
            "in_memory" => Ok(Arc::new(InMemoryGarrison::new(
                config.max_entries,
                config.max_tokens,
            ))),

            "sqlite" => {
                let path = config.path.as_ref()
                    .ok_or_else(|| GarrisonError::ConfigError("path required for sqlite"))?;

                Ok(Arc::new(SqliteGarrison::new(
                    path,
                    config.max_entries,
                    config.max_tokens,
                )?))
            }

            other => Err(GarrisonError::UnsupportedType(other.to_string())),
        }
    }
}
```

**Usage**:
```rust
let garrison_config = GarrisonConfig {
    storage_type: "sqlite".into(),
    path: Some("./garrison.db".into()),
    max_entries: 1000,
    max_tokens: Some(8000),
};

let garrison = GarrisonFactory::create(&garrison_config)?;
```

**Benefits**:
- Centralized creation logic
- Easy to add new implementations
- Configuration-driven instantiation

## Behavioral Patterns

### 1. Strategy Pattern

**Purpose**: Select algorithm at runtime (e.g., error handling, aggregation).

**Structure**:
```rust
/// Error handling strategies for Battalion
#[derive(Debug, Clone)]
pub enum ErrorStrategy {
    FailFast,
    Continue,
    RetryThenContinue { max_retries: u32 },
}

impl ErrorStrategy {
    /// Handle error according to strategy
    pub async fn handle<F, T, E>(
        &self,
        operation: F,
    ) -> Result<T, E>
    where
        F: Fn() -> Future<Output = Result<T, E>>,
        E: std::error::Error,
    {
        match self {
            ErrorStrategy::FailFast => operation().await,

            ErrorStrategy::Continue => {
                match operation().await {
                    Ok(result) => Ok(result),
                    Err(e) => {
                        eprintln!("Error (continuing): {}", e);
                        // Return default or skip
                        Err(e)
                    }
                }
            }

            ErrorStrategy::RetryThenContinue { max_retries } => {
                let mut attempts = 0;
                loop {
                    match operation().await {
                        Ok(result) => return Ok(result),
                        Err(e) if attempts < *max_retries => {
                            attempts += 1;
                            eprintln!("Retry {}/{}: {}", attempts, max_retries, e);
                            tokio::time::sleep(Duration::from_secs(1)).await;
                        }
                        Err(e) => {
                            eprintln!("Max retries exceeded: {}", e);
                            return Err(e);
                        }
                    }
                }
            }
        }
    }
}
```

**Usage**:
```rust
let battalion = BattalionBuilder::new()
    .error_strategy(ErrorStrategy::RetryThenContinue { max_retries: 3 })
    .build()?;

// Strategy automatically applied during execution
battalion.execute(&input).await?;
```

**Benefits**:
- Runtime algorithm selection
- Easy to add new strategies
- Encapsulated behavior

### 2. Chain of Responsibility Pattern

**Purpose**: Pass request through chain of handlers.

**Structure**:
```rust
/// Fallback chain for LLM providers
pub struct LlmFallbackChain {
    providers: Vec<Arc<dyn LlmPort>>,
}

impl LlmFallbackChain {
    pub fn new(providers: Vec<Arc<dyn LlmPort>>) -> Self {
        Self { providers }
    }

    pub async fn generate(
        &self,
        model: &str,
        messages: &[Message],
        temperature: f32,
    ) -> Result<LlmResponse, LlmError> {
        let mut last_error = None;

        for provider in &self.providers {
            match provider.generate(model, messages, temperature).await {
                Ok(response) => return Ok(response),
                Err(e) => {
                    eprintln!("Provider failed: {:?}", e);
                    last_error = Some(e);
                    // Try next provider
                }
            }
        }

        Err(last_error.unwrap_or_else(|| LlmError::NoProvidersAvailable))
    }
}
```

**Usage**:
```rust
let fallback_chain = LlmFallbackChain::new(vec![
    Arc::new(openai_adapter),
    Arc::new(anthropic_adapter),
    Arc::new(local_llm_adapter),
]);

// Automatically falls back to next provider on error
let response = fallback_chain.generate("gpt-4", &messages, 0.7).await?;
```

**Benefits**:
- Automatic failover
- Ordered fallback logic
- Resilience to provider failures

### 3. Observer Pattern (Event Publishing)

**Purpose**: Notify subscribers of state changes.

**Structure**:
```rust
/// Event publisher trait
pub trait EventPublisher: Send + Sync {
    fn publish(&self, event: PaladinEvent);
}

/// In-memory event bus
pub struct EventBus {
    subscribers: Arc<RwLock<Vec<Arc<dyn EventSubscriber>>>>,
}

pub trait EventSubscriber: Send + Sync {
    fn on_event(&self, event: &PaladinEvent);
}

impl EventBus {
    pub fn new() -> Self {
        Self {
            subscribers: Arc::new(RwLock::new(Vec::new())),
        }
    }

    pub fn subscribe(&self, subscriber: Arc<dyn EventSubscriber>) {
        self.subscribers.write().unwrap().push(subscriber);
    }
}

impl EventPublisher for EventBus {
    fn publish(&self, event: PaladinEvent) {
        let subscribers = self.subscribers.read().unwrap();
        for subscriber in subscribers.iter() {
            subscriber.on_event(&event);
        }
    }
}
```

**Usage**:
```rust
// Create event bus
let event_bus = Arc::new(EventBus::new());

// Subscribe to events
event_bus.subscribe(Arc::new(LoggingSubscriber::new()));
event_bus.subscribe(Arc::new(MetricsSubscriber::new()));

// Publish events
event_bus.publish(PaladinEvent::ExecutionStarted {
    paladin_id: paladin.id,
    input: input.to_string(),
    timestamp: Utc::now(),
});
```

**Benefits**:
- Decoupled event handling
- Multiple subscribers
- Extensible event system

## Architectural Patterns

### 1. Repository Pattern

**Purpose**: Abstract data persistence.

**Structure**:
```rust
/// Generic repository trait
#[async_trait]
pub trait Repository<T>: Send + Sync {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<T>, RepositoryError>;
    async fn find_all(&self) -> Result<Vec<T>, RepositoryError>;
    async fn save(&self, entity: &T) -> Result<(), RepositoryError>;
    async fn delete(&self, id: Uuid) -> Result<(), RepositoryError>;
}

/// Paladin-specific repository
#[async_trait]
pub trait PaladinRepository: Repository<Paladin> {
    async fn find_by_name(&self, name: &str) -> Result<Option<Paladin>, RepositoryError>;
    async fn find_active(&self) -> Result<Vec<Paladin>, RepositoryError>;
}

/// SQLite implementation
pub struct SqlitePaladinRepository {
    pool: SqlitePool,
}

#[async_trait]
impl PaladinRepository for SqlitePaladinRepository {
    async fn find_by_name(&self, name: &str) -> Result<Option<Paladin>, RepositoryError> {
        let row = sqlx::query_as::<_, PaladinRow>(
            "SELECT * FROM paladins WHERE name = ?"
        )
        .bind(name)
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(|r| r.into()))
    }

    async fn find_active(&self) -> Result<Vec<Paladin>, RepositoryError> {
        let rows = sqlx::query_as::<_, PaladinRow>(
            "SELECT * FROM paladins WHERE status = 'Running'"
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(rows.into_iter().map(|r| r.into()).collect())
    }
}
```

**Benefits**:
- Database abstraction
- Easy to swap storage backends
- Testability with in-memory repositories

### 2. Unit of Work Pattern

**Purpose**: Group multiple operations into a transaction.

**Structure**:
```rust
/// Unit of work for coordinated operations
pub struct UnitOfWork {
    garrison: Arc<dyn GarrisonPort>,
    citadel: Arc<dyn CitadelPort>,
    transaction: Option<Transaction>,
}

impl UnitOfWork {
    pub fn new(
        garrison: Arc<dyn GarrisonPort>,
        citadel: Arc<dyn CitadelPort>,
    ) -> Self {
        Self {
            garrison,
            citadel,
            transaction: None,
        }
    }

    /// Start transaction
    pub async fn begin(&mut self) -> Result<(), Error> {
        self.transaction = Some(Transaction::begin().await?);
        Ok(())
    }

    /// Add garrison entry
    pub async fn add_entry(&self, entry: GarrisonEntry) -> Result<(), Error> {
        self.garrison.add(entry).await?;
        Ok(())
    }

    /// Create checkpoint
    pub async fn create_checkpoint(&self, checkpoint: Checkpoint) -> Result<(), Error> {
        self.citadel.save(checkpoint).await?;
        Ok(())
    }

    /// Commit all changes
    pub async fn commit(mut self) -> Result<(), Error> {
        if let Some(tx) = self.transaction.take() {
            tx.commit().await?;
        }
        Ok(())
    }

    /// Rollback changes
    pub async fn rollback(mut self) -> Result<(), Error> {
        if let Some(tx) = self.transaction.take() {
            tx.rollback().await?;
        }
        Ok(())
    }
}
```

**Usage**:
```rust
let mut uow = UnitOfWork::new(garrison, citadel);
uow.begin().await?;

// Perform multiple operations
uow.add_entry(user_message).await?;
uow.add_entry(assistant_response).await?;
uow.create_checkpoint(checkpoint).await?;

// Commit or rollback
if success {
    uow.commit().await?;
} else {
    uow.rollback().await?;
}
```

**Benefits**:
- Transactional consistency
- All-or-nothing operations
- Simplified error handling

### 3. Dependency Injection Pattern

**Purpose**: Provide dependencies to objects.

**Structure**:
```rust
/// Service with injected dependencies
pub struct PaladinExecutionService {
    llm_port: Arc<dyn LlmPort>,
    garrison_port: Option<Arc<dyn GarrisonPort>>,
    arsenal_registry: Arc<ArsenalRegistry>,
    event_publisher: Arc<dyn EventPublisher>,
}

impl PaladinExecutionService {
    /// Constructor injection
    pub fn new(
        llm_port: Arc<dyn LlmPort>,
        garrison_port: Option<Arc<dyn GarrisonPort>>,
        arsenal_registry: Arc<ArsenalRegistry>,
        event_publisher: Arc<dyn EventPublisher>,
    ) -> Self {
        Self {
            llm_port,
            garrison_port,
            arsenal_registry,
            event_publisher,
        }
    }

    pub async fn execute(&self, paladin: &Paladin, input: &str) -> Result<PaladinResult> {
        // Use injected dependencies
        self.event_publisher.publish(PaladinEvent::ExecutionStarted { /* ... */ });

        let response = self.llm_port.generate(/* ... */).await?;

        if let Some(garrison) = &self.garrison_port {
            garrison.add(/* ... */).await?;
        }

        Ok(result)
    }
}
```

**Manual DI Container**:
```rust
/// Simple DI container
pub struct Container {
    llm_port: Arc<dyn LlmPort>,
    garrison_port: Arc<dyn GarrisonPort>,
    arsenal_registry: Arc<ArsenalRegistry>,
    event_publisher: Arc<dyn EventPublisher>,
}

impl Container {
    pub fn new(config: &ApplicationConfig) -> Result<Self, Error> {
        // Create adapters
        let llm_port = Arc::new(OpenAiAdapter::new(&config.openai)?);
        let garrison_port = Arc::new(SqliteGarrison::new(&config.garrison_path)?);
        let arsenal_registry = Arc::new(ArsenalRegistry::new());
        let event_publisher = Arc::new(EventBus::new());

        Ok(Self {
            llm_port,
            garrison_port,
            arsenal_registry,
            event_publisher,
        })
    }

    /// Create execution service with dependencies
    pub fn paladin_execution_service(&self) -> PaladinExecutionService {
        PaladinExecutionService::new(
            self.llm_port.clone(),
            Some(self.garrison_port.clone()),
            self.arsenal_registry.clone(),
            self.event_publisher.clone(),
        )
    }
}
```

**Benefits**:
- Loose coupling
- Easy testing with mocks
- Centralized dependency management

## Pattern Guidelines

### When to Use Builder Pattern

✅ **Use when**:
- Object has many optional parameters
- Construction requires validation
- Construction is multi-step
- You want a fluent API

❌ **Don't use when**:
- Object is simple (< 3 fields)
- All fields are required
- No validation needed

### When to Use Factory Pattern

✅ **Use when**:
- Creating objects based on configuration
- Multiple implementations of an interface
- Complex instantiation logic
- Runtime type selection

❌ **Don't use when**:
- Only one implementation exists
- Construction is trivial
- Direct instantiation is clear

### When to Use Repository Pattern

✅ **Use when**:
- Abstracting data persistence
- Multiple storage backends
- Testing with in-memory storage
- Complex queries

❌ **Don't use when**:
- Simple CRUD only
- No need for abstraction
- Performance-critical path (consider direct access)

### When to Use Strategy Pattern

✅ **Use when**:
- Algorithm varies at runtime
- Multiple related behaviors
- Encapsulating behavior
- Avoiding conditionals

❌ **Don't use when**:
- Only one algorithm
- Algorithm never changes
- Simple conditional logic

## Next Steps

- **[Hexagonal Design](hexagonal-design.md)** - Port/adapter implementation
- **[Domain Model](domain-model.md)** - Entity relationships
- **[Adapter Development](../contributing/contributing-providers.md)** - Create custom adapters
