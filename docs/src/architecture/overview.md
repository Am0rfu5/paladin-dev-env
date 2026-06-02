# Paladin Architecture Overview

This document provides a comprehensive overview of Paladin's architecture, design principles, and system organization.

## Table of Contents

- [Executive Summary](#executive-summary)
- [Architectural Principles](#architectural-principles)
- [Three-Layer Architecture](#three-layer-architecture)
- [System Components](#system-components)
- [Data Flow](#data-flow)
- [Deployment Architecture](#deployment-architecture)
- [Technology Stack](#technology-stack)
- [Design Decisions](#design-decisions)

## Executive Summary

Paladin is an enterprise-grade multi-agent orchestration framework built with **Hexagonal Architecture** (Ports and Adapters) and **Domain-Driven Design** principles. The system enables autonomous AI agents (Paladins) to execute complex tasks through coordinated multi-agent patterns (Battalions), external tool integration (Arsenal), and persistent memory (Garrison).

**Key Characteristics:**
- **Clean Architecture**: Strict dependency rules with core business logic isolated from infrastructure
- **Provider Agnostic**: Support for multiple LLM providers (OpenAI, DeepSeek, Anthropic, custom)
- **Extensible**: Plugin-based tool system via Model Context Protocol (MCP)
- **Production-Ready**: Comprehensive error handling, observability, and state management
- **Type-Safe**: Leverages Rust's type system for compile-time guarantees

## Architectural Principles

### 1. Hexagonal Architecture (Ports & Adapters)

Paladin follows the hexagonal architecture pattern to achieve:

```
┌─────────────────────────────────────────────────────────┐
│                    External World                        │
│  (LLMs, Databases, File Systems, APIs, Message Queues)  │
└────────────┬─────────────────────────────┬──────────────┘
             │                             │
             │  Adapters (Infrastructure)  │
             │                             │
┌────────────▼─────────────────────────────▼──────────────┐
│                        Ports                             │
│            (Application Interfaces)                      │
└────────────┬─────────────────────────────┬──────────────┘
             │                             │
             │    Use Cases & Services     │
             │                             │
┌────────────▼─────────────────────────────▼──────────────┐
│                    Core Domain                           │
│  (Paladin, Battalion, Garrison, Arsenal - Pure Logic)   │
└──────────────────────────────────────────────────────────┘
```

**Benefits:**
- Business logic independent of external dependencies
- Easy to test (mock adapters)
- Flexibility to swap implementations (e.g., change LLM provider)
- Clear boundaries and responsibilities

### 2. Domain-Driven Design (DDD)

Paladin applies DDD principles:

**Ubiquitous Language**: Medieval Military theme provides clear, consistent terminology
- **Paladin** = AI agent
- **Battalion** = Multi-agent orchestration
- **Garrison** = Memory system
- **Arsenal** = Tool registry
- **Armament** = Individual tool
- **Citadel** = State persistence

**Bounded Contexts**: Clear boundaries between subsystems
- **Agent Context**: Paladin execution and lifecycle
- **Memory Context**: Garrison storage and retrieval
- **Tool Context**: Arsenal management and execution
- **Orchestration Context**: Battalion coordination

**Aggregates**: Entities with clear ownership
- **Paladin** is an aggregate root containing configuration and state
- **Battalion** is an aggregate coordinating multiple Paladins
- **GarrisonEntry** is owned by Garrison aggregate

### 3. Dependency Inversion Principle

```rust
// High-level modules don't depend on low-level modules
// Both depend on abstractions (traits)

// Core Domain (high-level)
pub struct Paladin { /* ... */ }

// Application Port (abstraction)
#[async_trait]
pub trait LlmPort: Send + Sync {
    async fn generate(&self, prompt: &str) -> Result<String>;
}

// Infrastructure Adapter (low-level)
pub struct OpenAiAdapter { /* ... */ }

impl LlmPort for OpenAiAdapter {
    async fn generate(&self, prompt: &str) -> Result<String> {
        // Implementation details
    }
}
```

Dependencies flow inward: Infrastructure → Application → Core

## Three-Layer Architecture

### Layer 1: Core Domain (`src/core/`)

**Purpose**: Pure business logic with zero external dependencies

**Responsibilities:**
- Define domain entities (Paladin, Battalion, Garrison, Arsenal)
- Implement business rules and invariants
- Provide domain events and value objects

**Key Modules:**
```
src/core/
├── base/                    # Framework primitives
│   ├── node.rs             # Node<T> entity pattern
│   ├── collection.rs       # Collection management
│   ├── field.rs            # Field definitions
│   └── message.rs          # Message types
├── platform/
│   └── container/
│       ├── paladin.rs          # Paladin entity
│       ├── paladin_config.rs   # Configuration
│       ├── garrison.rs         # Memory domain
│       ├── arsenal.rs          # Tool domain
│       ├── citadel.rs          # State persistence
│       └── battalion/
│           ├── mod.rs          # Battalion types
│           ├── formation.rs    # Sequential pattern
│           ├── phalanx.rs      # Concurrent pattern
│           ├── campaign.rs     # Graph pattern
│           └── chain_of_command.rs  # Hierarchical pattern
└── manager/
    ├── scheduler.rs
    ├── queue_service.rs
    └── event_manager.rs
```

**Design Constraints:**
- ❌ No imports from `application` or `infrastructure`
- ❌ No I/O operations
- ❌ No framework dependencies (except serialization)
- ✅ Pure functions and data structures
- ✅ Domain logic only

**Example:**
```rust
// Core domain entity - pure business logic
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaladinData {
    pub system_prompt: String,
    pub name: String,
    pub model: String,
    pub temperature: f32,
    pub max_loops: u32,
    pub status: PaladinStatus,
}

pub type Paladin = Node<PaladinData>;

// Business rules enforced in the domain
impl PaladinData {
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

### Layer 2: Application (`src/application/`)

**Purpose**: Use cases, orchestration, and port definitions

**Responsibilities:**
- Define port interfaces (traits) for external systems
- Implement use case services
- Coordinate domain entities
- Handle application-level concerns (retries, transactions)

**Key Modules:**
```
src/application/
├── ports/
│   ├── input/
│   │   ├── content_ingestion_port.rs
│   │   └── ml_port.rs
│   └── output/
│       ├── paladin_port.rs        # Paladin execution
│       ├── garrison_port.rs       # Memory operations
│       ├── arsenal_port.rs        # Tool operations
│       ├── battalion_port.rs      # Orchestration
│       ├── citadel_port.rs        # State persistence
│       ├── llm_port.rs            # LLM providers
│       ├── file_storage_port.rs   # File storage
│       └── notification_port.rs   # Notifications
├── services/
│   ├── paladin/
│   │   ├── paladin_builder.rs
│   │   └── paladin_execution_service.rs
│   ├── battalion/
│   │   ├── formation_service.rs
│   │   ├── phalanx_service.rs
│   │   ├── campaign_service.rs
│   │   ├── chain_of_command_service.rs
│   │   └── commander.rs
│   └── content/
└── storage/
    └── repository traits
```

**Port Example:**
```rust
/// Port abstraction for LLM providers
#[async_trait]
pub trait LlmPort: Send + Sync {
    /// Generate completion from prompt
    async fn generate(&self, prompt: &PromptItem) -> Result<LlmResponse, LlmError>;

    /// Generate with streaming
    async fn generate_stream(&self, prompt: &PromptItem)
        -> Result<LlmStream, LlmError>;

    /// Validate model availability
    fn validate_model(&self, model: &str) -> Result<(), LlmError>;
}
```

**Use Case Example:**
```rust
/// Service implementing Paladin execution use case
pub struct PaladinExecutionService {
    llm_port: Arc<dyn LlmPort>,
    garrison_port: Option<Arc<dyn GarrisonPort>>,
    arsenal_registry: Arc<ArsenalRegistry>,
}

impl PaladinExecutionService {
    pub async fn execute(
        &self,
        paladin: &Paladin,
        input: &str
    ) -> Result<PaladinResult, PaladinError> {
        // 1. Retrieve context from Garrison
        let history = if let Some(garrison) = &self.garrison_port {
            garrison.get_window(4000).await?
        } else {
            vec![]
        };

        // 2. Build prompt with context
        let prompt = self.build_prompt(paladin, input, &history);

        // 3. Execute LLM call
        let response = self.llm_port.generate(&prompt).await?;

        // 4. Check for tool calls
        if let Some(tool_call) = response.tool_calls.first() {
            let result = self.arsenal_registry.invoke(tool_call).await?;
            // Process tool result...
        }

        // 5. Store in Garrison
        if let Some(garrison) = &self.garrison_port {
            garrison.add_entry(create_entry(&response)).await?;
        }

        Ok(PaladinResult { /* ... */ })
    }
}
```

### Layer 3: Infrastructure (`src/infrastructure/`)

**Purpose**: Adapter implementations for external systems

**Responsibilities:**
- Implement port traits with concrete technology
- Handle I/O, networking, database operations
- Manage external dependencies
- Provide configuration and initialization

**Key Modules:**
```
src/infrastructure/
├── adapters/
│   ├── llm/
│   │   ├── openai_adapter.rs      # OpenAI API
│   │   ├── deepseek_adapter.rs    # DeepSeek API
│   │   └── anthropic_adapter.rs   # Anthropic API
│   ├── garrison/
│   │   ├── in_memory_garrison.rs  # RAM storage
│   │   └── sqlite_garrison.rs     # SQLite persistence
│   ├── arsenal/
│   │   ├── mcp_client.rs          # MCP protocol
│   │   ├── mcp_stdio_adapter.rs   # STDIO servers
│   │   └── mcp_sse_adapter.rs     # SSE servers
│   ├── citadel/
│   │   └── file_citadel.rs        # File-based state
│   ├── queue/
│   │   └── redis_adapter.rs       # Redis queues
│   └── file_storage/
│       └── minio_adapter.rs       # S3-compatible storage
└── repositories/
    ├── mysql/
    └── sqlite/
```

**Adapter Example:**
```rust
/// OpenAI implementation of LlmPort
pub struct OpenAiAdapter {
    client: reqwest::Client,
    api_key: String,
    base_url: String,
    default_model: String,
}

#[async_trait]
impl LlmPort for OpenAiAdapter {
    async fn generate(&self, prompt: &PromptItem) -> Result<LlmResponse, LlmError> {
        let request = self.build_request(prompt)?;

        let response = self.client
            .post(&format!("{}/chat/completions", self.base_url))
            .bearer_auth(&self.api_key)
            .json(&request)
            .send()
            .await
            .map_err(|e| LlmError::NetworkError(e.to_string()))?;

        let openai_response: OpenAiResponse = response.json().await
            .map_err(|e| LlmError::ParseError(e.to_string()))?;

        Ok(self.convert_response(openai_response))
    }

    // ... other methods
}
```

## System Components

### Paladin Agent

**Purpose**: Autonomous AI agent capable of reasoning and action

**Key Features:**
- Configurable behavior via system prompts
- Multi-turn conversation support
- Tool calling capabilities
- Loop detection and stop conditions
- State persistence

**Lifecycle:**
```
┌─────────────────────────────────────────────────────────┐
│                    Paladin Lifecycle                     │
└─────────────────────────────────────────────────────────┘

   ┌──────────┐
   │  Create  │  ← PaladinBuilder constructs agent
   └────┬─────┘
        │
        ▼
   ┌──────────┐
   │   Idle   │  ← Waiting for input
   └────┬─────┘
        │
        ▼
   ┌──────────┐
   │ Running  │  ← Executing reasoning loop
   └────┬─────┘
        │
        ├─────→ Tool Call? ──→ Execute Tool ──┐
        │                                      │
        │◄─────────────────────────────────────┘
        │
        ├─────→ Max Loops? ──→ Stop
        │
        ├─────→ Stop Word? ──→ Stop
        │
        ▼
   ┌──────────┐
   │ Complete │  ← Return result
   └──────────┘
```

### Battalion Orchestration

**Purpose**: Multi-agent coordination patterns

**Patterns:**

1. **Formation (Sequential)**
   ```
   Paladin 1 → Output → Paladin 2 → Output → Paladin 3
   ```
   Use case: Pipeline processing (research → analyze → write)

2. **Phalanx (Concurrent)**
   ```
         ┌─→ Paladin 1 ─┐
   Input ├─→ Paladin 2 ─┤→ Aggregate
         └─→ Paladin 3 ─┘
   ```
   Use case: Parallel reviews (technical, security, UX)

3. **Campaign (Graph/DAG)**
   ```
        ┌─→ Paladin 2 ─┐
   P1 ──┤              ├─→ P5
        └─→ Paladin 3 ─┤
               │        │
               ▼        │
            Paladin 4 ──┘
   ```
   Use case: Conditional workflows

4. **Chain of Command (Hierarchical)**
   ```
         Commander
            │
      ┌─────┼─────┐
      ▼     ▼     ▼
   Spec1  Spec2  Spec3
   ```
   Use case: Dynamic delegation

### Garrison Memory System

**Purpose**: Conversation context and long-term knowledge

**Storage Types:**
- **In-Memory**: Fast, volatile, for active sessions
- **SQLite**: Persistent, queryable, for session history
- **Vector**: Semantic search with embeddings

**Memory Types:**
- **Episodic**: Specific events and experiences
- **Semantic**: General facts and knowledge
- **Procedural**: How-to instructions

### Arsenal Tool System

**Purpose**: External tool integration and execution

**Protocol Support:**
- **MCP STDIO**: Command-line tool servers
- **MCP SSE**: Web-based tool servers
- **Custom**: Native Rust tool implementations

**Tool Flow:**
```
┌─────────────────────────────────────────────────────┐
│              Arsenal Tool Execution                  │
└─────────────────────────────────────────────────────┘

Paladin → LLM decides tool needed
   │
   ▼
ArmamentCall created
   │
   ▼
Arsenal validates call
   │
   ▼
Route to correct adapter (STDIO/SSE/Custom)
   │
   ▼
Execute tool
   │
   ▼
ArmamentResult returned
   │
   ▼
Inject result into Paladin context
   │
   ▼
Paladin continues with tool output
```

## Data Flow

### Request Flow (Single Paladin)

```
┌────────────────────────────────────────────────────────────┐
│                    Request Flow                             │
└────────────────────────────────────────────────────────────┘

1. User Input
   │
   ▼
2. PaladinBuilder creates Paladin
   │
   ▼
3. PaladinExecutionService.execute()
   │
   ├─→ Load context from Garrison
   │
   ├─→ Build prompt with system + context + user input
   │
   ├─→ Call LlmPort.generate()
   │   │
   │   └─→ OpenAiAdapter.generate()
   │       │
   │       └─→ HTTP POST to api.openai.com
   │
   ├─→ Check for tool calls
   │   │
   │   └─→ If yes: Arsenal.invoke()
   │       │
   │       └─→ Execute tool, inject result
   │
   ├─→ Save response to Garrison
   │
   └─→ Return PaladinResult to user
```

### Battalion Flow (Multi-Agent)

```
┌────────────────────────────────────────────────────────────┐
│              Battalion Execution Flow                       │
└────────────────────────────────────────────────────────────┘

Formation (Sequential):
   Input → P1 → out1 → P2 → out2 → P3 → Final Result

Phalanx (Concurrent):
   Input ─┬→ spawn(P1.execute()) ─┬→ Aggregate Results
          ├→ spawn(P2.execute()) ─┤
          └→ spawn(P3.execute()) ─┘

Campaign (Graph):
   Input → Evaluate edges → Execute node
         → Follow conditions → Next node
         → Repeat until terminal

Chain of Command:
   Input → Commander analyzes
         → Commander delegates to specialists
         → Collect specialist results
         → Commander synthesizes final answer
```

## Deployment Architecture

### Single-Instance Deployment

```
┌─────────────────────────────────────────────────────────┐
│                   Docker Container                       │
│                                                          │
│  ┌────────────────────────────────────────────────┐    │
│  │           Paladin Application                   │    │
│  │  ┌──────────┐  ┌──────────┐  ┌──────────┐    │    │
│  │  │ Paladin  │  │Battalion │  │ Garrison │    │    │
│  │  │ Service  │  │ Service  │  │ Service  │    │    │
│  │  └──────────┘  └──────────┘  └──────────┘    │    │
│  └────────────────────────────────────────────────┘    │
│                                                          │
│  External Dependencies:                                 │
│  • OpenAI API (LLM)                                    │
│  • SQLite (Garrison persistence)                        │
│  • MCP Servers (Tools)                                 │
└─────────────────────────────────────────────────────────┘
```

### Kubernetes Deployment

```
┌─────────────────────────────────────────────────────────────┐
│                    Kubernetes Cluster                        │
│                                                              │
│  ┌────────────────────────────────────────────────────┐    │
│  │             Paladin Deployment                      │    │
│  │                                                     │    │
│  │  ┌─────────┐  ┌─────────┐  ┌─────────┐           │    │
│  │  │ Pod 1   │  │ Pod 2   │  │ Pod 3   │           │    │
│  │  │ Paladin │  │ Paladin │  │ Paladin │           │    │
│  │  └─────────┘  └─────────┘  └─────────┘           │    │
│  └──────────────────┬──────────────────────────────────┘    │
│                     │                                        │
│  ┌──────────────────▼─────────────────────────────────┐    │
│  │                Service (LoadBalancer)               │    │
│  └──────────────────┬──────────────────────────────────┘    │
│                     │                                        │
│  ┌──────────────────▼─────────────────────────────────┐    │
│  │             ConfigMap & Secrets                     │    │
│  │  • LLM API Keys                                     │    │
│  │  • Configuration                                    │    │
│  └──────────────────────────────────────────────────────┘    │
│                                                              │
│  External:                                                   │
│  • Redis (Queue) - StatefulSet                              │
│  • MinIO (Storage) - StatefulSet                            │
│  • PostgreSQL (Garrison) - StatefulSet                      │
└─────────────────────────────────────────────────────────────┘
```

## Technology Stack

### Core Technologies
- **Language**: Rust 1.70+
- **Async Runtime**: Tokio
- **Serialization**: Serde (JSON, YAML)
- **Error Handling**: thiserror, anyhow
- **CLI**: clap
- **Logging**: tracing, tracing-subscriber

### External Integrations
- **LLM Providers**: OpenAI, DeepSeek, Anthropic (via reqwest)
- **Databases**: SQLite (sqlx), MySQL (sqlx)
- **Object Storage**: MinIO (S3-compatible via rusoto_s3)
- **Message Queue**: Redis (redis-rs)
- **Protocol**: Model Context Protocol (MCP)

### Testing & Quality
- **Testing**: cargo test, testcontainers
- **Benchmarking**: Criterion
- **Coverage**: cargo-llvm-cov
- **Linting**: clippy
- **Formatting**: rustfmt
- **Security**: cargo-audit

### Deployment
- **Containerization**: Docker (multi-stage builds)
- **Orchestration**: Kubernetes
- **CI/CD**: GitHub Actions
- **Monitoring**: Prometheus, Grafana (planned)

## Design Decisions

### Why Hexagonal Architecture?

**Decision**: Use Hexagonal Architecture instead of layered or MVC

**Rationale:**
- Testability: Can mock all external dependencies via ports
- Flexibility: Easy to swap LLM providers without touching business logic
- Maintainability: Clear separation of concerns
- Independence: Core domain has no external dependencies

**Trade-offs:**
- More abstractions (ports/adapters)
- Learning curve for developers
- More files and boilerplate

### Why Rust?

**Decision**: Build in Rust instead of Python or TypeScript

**Rationale:**
- Performance: Near-C++ speed for token processing
- Memory Safety: Compile-time guarantees prevent crashes
- Concurrency: Fearless concurrency with tokio for Battalion parallelism
- Type Safety: Strong typing catches errors at compile time
- Zero-Cost Abstractions: No runtime overhead

**Trade-offs:**
- Steeper learning curve
- Slower development initially
- Smaller ecosystem than Python for AI/ML

### Why Medieval Military Theme?

**Decision**: Use Medieval Military terminology (Paladin, Battalion, etc.)

**Rationale:**
- Ubiquitous Language: DDD principle for clear communication
- Memorable: Easier to remember than generic terms
- Hierarchical: Military structure maps well to agent coordination
- Consistent: Single metaphor throughout codebase

**Trade-offs:**
- Learning curve for new developers
- May seem unusual initially

### Why Multiple LLM Providers?

**Decision**: Support OpenAI, DeepSeek, Anthropic, and custom providers

**Rationale:**
- Vendor Independence: No lock-in to single provider
- Cost Optimization: Choose provider based on task/budget
- Reliability: Fallback if one provider is down
- Feature Access: Different models have different capabilities

**Trade-offs:**
- More code to maintain
- Provider-specific quirks to handle
- Testing complexity

### Why MCP for Tools?

**Decision**: Use Model Context Protocol for tool integration

**Rationale:**
- Standard Protocol: Open standard for AI tool integration
- Interoperability: Works with any MCP-compliant server
- Ecosystem: Growing number of MCP servers available
- Flexibility: STDIO and SSE support

**Trade-offs:**
- Protocol complexity
- Limited adoption currently
- Need to maintain MCP client

## Next Steps

- **[Hexagonal Design](hexagonal-design.md)** - Deep dive into ports and adapters
- **[Domain Model](domain-model.md)** - DDD entities and relationships
- **[Design Patterns](design-patterns.md)** - Patterns used throughout Paladin
- **[Deployment Guide](../deployment/docker.md)** - Production deployment documentation
