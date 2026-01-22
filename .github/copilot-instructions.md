# Paladin AI Coding Agent Instructions

## Project Overview

Paladin is a **Rust-based enterprise multi-agent orchestration framework** built with **Hexagonal Architecture** (Ports & Adapters) and **Domain-Driven Design** principles. It enables the creation, coordination, and scaling of intelligent AI agents (Paladins) through configurable orchestration patterns with comprehensive LLM integration.

### Project Vision

Build the definitive Rust-based multi-agent orchestration framework enabling enterprises to deploy, coordinate, and manage AI agents with unparalleled reliability, performance, and safety.

### Key Capabilities

- **Paladin Agents**: Autonomous AI entities with configurable behaviors, memory, and tool access
- **Battalion Orchestration**: Multi-agent coordination patterns (Formation, Phalanx, Campaign, Chain of Command)
- **Arsenal Integration**: External tool execution via MCP (Model Context Protocol)
- **Multi-Provider LLM**: Support for OpenAI, DeepSeek, Anthropic, and extensible to others
- **Enterprise-Grade**: Comprehensive logging, error handling, state persistence, and monitoring

## Naming Convention: Medieval Military Theme

All multi-agent constructs follow a consistent Medieval Military naming convention:

| Term | Definition | Module Location |
|------|------------|-----------------|
| **Paladin** | An autonomous AI agent capable of reasoning and action | `core/platform/container/paladin.rs` |
| **Battalion** | A coordinated group of Paladins working together | `core/platform/container/battalion/` |
| **Formation** | Sequential Paladin execution (output N → input N+1) | `battalion/formation.rs` |
| **Phalanx** | Concurrent Paladin execution (parallel processing) | `battalion/phalanx.rs` |
| **Campaign** | Graph/DAG-based Paladin orchestration | `battalion/campaign.rs` |
| **Chain of Command** | Hierarchical Paladin delegation pattern | `battalion/chain_of_command.rs` |
| **Commander** | Dynamic Battalion strategy router | `use_cases/battalion/commander.rs` |
| **Garrison** | Paladin memory and context storage | `core/platform/container/garrison.rs` |
| **Arsenal** | Tool and capability registry | `core/platform/container/arsenal.rs` |
| **Armament** | A single tool or capability | Part of Arsenal |
| **Citadel** | State persistence and recovery system | `core/platform/container/citadel.rs` |
| **Herald** | Output formatting system | `herald.rs` |
| **Armory** | CLI tools for development | `bin/paladin-cli.rs` |
| **Quest** | A task or mission assigned to Paladins | Part of execution |

**Always use these terms consistently in code, documentation, and comments.**

## Architecture: Three-Layer Hexagonal Design

### 1. Core Layer (`src/core/`)
Pure business logic with zero external dependencies:

```
core/
├── base/                    # Framework primitives
│   ├── node.rs             # Node<T> pattern for entities
│   ├── collection.rs       # Collection management
│   ├── field.rs            # Field definitions
│   └── message.rs          # Message types
├── platform/
│   └── container/
│       ├── paladin.rs          # Paladin domain entity
│       ├── paladin_config.rs   # Paladin configuration
│       ├── garrison.rs         # Memory system domain
│       ├── arsenal.rs          # Tool system domain
│       ├── citadel.rs          # State persistence domain
│       ├── battalion/
│       │   ├── mod.rs          # Battalion base types
│       │   ├── formation.rs    # Sequential execution
│       │   ├── phalanx.rs      # Concurrent execution
│       │   ├── campaign.rs     # Graph orchestration
│       │   └── chain_of_command.rs  # Hierarchical delegation
│       ├── content_item.rs     # Content processing (existing)
│       ├── job.rs              # Job management (existing)
│       └── task.rs             # Task management (existing)
└── manager/                 # Core services
    ├── scheduler.rs
    ├── queue_service.rs
    └── event_manager.rs
```

### 2. Application Layer (`src/application/`)
Use cases and port definitions (interfaces):

```
application/
├── ports/
│   ├── input/
│   │   ├── content_ingestion_port.rs  # Existing
│   │   └── ml_port.rs                  # Existing
│   └── output/
│       ├── paladin_port.rs        # Paladin execution abstraction
│       ├── garrison_port.rs       # Memory operations abstraction
│       ├── arsenal_port.rs        # Tool operations abstraction
│       ├── battalion_port.rs      # Battalion execution abstraction
│       ├── citadel_port.rs        # State persistence abstraction
│       ├── llm_port.rs            # LLM provider abstraction (existing)
│       ├── file_storage_port.rs   # File storage (existing)
│       └── notification_port.rs   # Notifications (existing)
├── use_cases/
│   ├── paladin/
│   │   ├── mod.rs
│   │   ├── paladin_builder.rs         # Fluent builder pattern
│   │   └── paladin_execution_service.rs
│   ├── battalion/
│   │   ├── mod.rs
│   │   ├── formation_service.rs
│   │   ├── phalanx_service.rs
│   │   ├── campaign_service.rs
│   │   ├── chain_of_command_service.rs
│   │   └── commander.rs               # Strategy router
│   └── content/                       # Existing use cases
└── storage/
    └── repository traits
```

### 3. Infrastructure Layer (`src/infrastructure/`)
Adapter implementations for external systems:

```
infrastructure/
├── adapters/
│   ├── llm/
│   │   ├── openai_adapter.rs      # Existing, enhance
│   │   ├── deepseek_adapter.rs    # New
│   │   └── anthropic_adapter.rs   # New
│   ├── garrison/
│   │   ├── in_memory_garrison.rs  # Short-term memory
│   │   └── sqlite_garrison.rs     # Persistent memory
│   ├── arsenal/
│   │   ├── mcp_client.rs          # MCP protocol client
│   │   ├── mcp_stdio_adapter.rs   # STDIO MCP servers
│   │   └── mcp_sse_adapter.rs     # SSE MCP servers
│   ├── citadel/
│   │   └── file_citadel.rs        # File-based state persistence
│   ├── queue/
│   │   └── redis_adapter.rs       # Existing
│   └── file_storage/
│       └── minio_adapter.rs       # Existing
└── repositories/
    ├── mysql/
    └── sqlite/
```

**Critical Pattern**: Dependencies flow inward only.
- Core: No imports from application or infrastructure
- Application: Imports core only, never infrastructure
- Infrastructure: Imports both core and application

## Development Methodology

### Test-Driven Development (TDD)

**All features must follow Red-Green-Refactor cycle:**

1. **Red**: Write a failing test first
2. **Green**: Write minimal code to pass the test
3. **Refactor**: Improve code while keeping tests green

**Coverage Requirements:**
- Unit tests: ≥ 80% coverage
- Integration tests: ≥ 70% coverage
- All public APIs must have doc tests

```bash
# Run tests
cargo test                           # Unit tests
make test-all                        # All tests
make test-integration-docker         # Integration with Docker
cargo test --test paladin_tests      # Specific test file
```

### Domain-Driven Design (DDD)

**Apply DDD principles throughout:**

1. **Ubiquitous Language**: Use Medieval Military terms consistently
2. **Bounded Contexts**: Clear boundaries between Paladin, Battalion, Garrison, Arsenal
3. **Aggregates**: Paladin is an aggregate root containing its configuration
4. **Domain Events**: Use for cross-context communication
5. **Value Objects**: Immutable configuration types

### Code Quality Standards

```bash
make clean-code        # Format + lint + check
cargo fmt              # Format code
cargo clippy           # Lint
cargo audit            # Security vulnerabilities
```

**All code must:**
- Pass `cargo clippy` with no warnings
- Be formatted with `cargo fmt`
- Have rustdoc for all public items
- Follow existing error handling patterns

## Code Conventions

### Error Handling Pattern

Each domain has specific error enums using `thiserror`:

```rust
#[derive(Debug, thiserror::Error)]
pub enum PaladinError {
    #[error("Configuration error: {0}")]
    ConfigurationError(String),
    #[error("Execution error: {0}")]
    ExecutionError(String),
    #[error("LLM error: {0}")]
    LlmError(String),
    #[error("Timeout after {0} seconds")]
    Timeout(u64),
    #[error("Stop word detected: {0}")]
    StopWordDetected(String),
}

#[derive(Debug, thiserror::Error)]
pub enum BattalionError {
    #[error("Paladin error: {0}")]
    PaladinError(#[from] PaladinError),
    #[error("Formation error: {0}")]
    FormationError(String),
    #[error("Invalid graph: {0}")]
    InvalidGraph(String),
}
```

**Pattern**: Layer-specific errors, convert at boundaries using `From` trait.

### Port Trait Pattern

All ports must be `Send + Sync` for async compatibility:

```rust
#[async_trait]
pub trait PaladinPort: Send + Sync {
    /// Execute a Paladin with the given input
    async fn execute(
        &self, 
        paladin: &Paladin, 
        input: &str
    ) -> Result<PaladinResult, PaladinError>;
    
    /// Execute with streaming response
    async fn execute_stream(
        &self, 
        paladin: &Paladin, 
        input: &str
    ) -> Result<PaladinStream, PaladinError>;
    
    /// Validate Paladin configuration
    fn validate(&self, paladin: &Paladin) -> Result<(), PaladinError>;
}
```

### Builder Pattern

Use fluent builders for complex object construction:

```rust
pub struct PaladinBuilder {
    llm_port: Arc<dyn LlmPort>,
    data: PaladinData,
    config: PaladinConfig,
    garrison: Option<Arc<dyn GarrisonPort>>,
    arsenal: Vec<Arc<dyn ArsenalPort>>,
}

impl PaladinBuilder {
    pub fn new(llm_port: Arc<dyn LlmPort>) -> Self { /* ... */ }
    
    pub fn system_prompt(mut self, prompt: impl Into<String>) -> Self {
        self.data.system_prompt = prompt.into();
        self
    }
    
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.data.name = name.into();
        self
    }
    
    pub fn max_loops(mut self, loops: u32) -> Self {
        self.data.max_loops = loops;
        self
    }
    
    pub fn with_garrison(mut self, garrison: Arc<dyn GarrisonPort>) -> Self {
        self.garrison = Some(garrison);
        self
    }
    
    pub fn add_armament(mut self, armament: Arc<dyn ArsenalPort>) -> Self {
        self.arsenal.push(armament);
        self
    }
    
    pub fn build(self) -> Result<Paladin, PaladinError> {
        self.validate()?;
        Ok(Paladin::new(self.data, self.config))
    }
    
    fn validate(&self) -> Result<(), PaladinError> {
        if self.data.system_prompt.is_empty() {
            return Err(PaladinError::ConfigurationError(
                "System prompt is required".into()
            ));
        }
        Ok(())
    }
}
```

### Node Pattern for Domain Entities

Follow the existing `Node<T>` pattern for domain entities:

```rust
/// Paladin data payload
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaladinData {
    pub system_prompt: String,
    pub name: String,
    pub user_name: String,
    pub model: String,
    pub temperature: f32,
    pub max_loops: u32,
    pub stop_words: Vec<String>,
    pub status: PaladinStatus,
}

/// Paladin aggregate root using Node pattern
pub type Paladin = Node<PaladinData>;
```

### Service Composition

Services compose other services following hexagonal patterns:

```rust
pub struct PaladinExecutionService {
    llm_port: Arc<dyn LlmPort>,
    garrison_port: Option<Arc<dyn GarrisonPort>>,
    arsenal_registry: Arc<ArsenalRegistry>,
    config: PaladinConfig,
}

impl PaladinExecutionService {
    pub fn new(
        llm_port: Arc<dyn LlmPort>,
        garrison_port: Option<Arc<dyn GarrisonPort>>,
        arsenal_registry: Arc<ArsenalRegistry>,
    ) -> Self { /* ... */ }
    
    pub async fn execute(
        &self, 
        paladin: &Paladin, 
        input: &str
    ) -> Result<PaladinResult, PaladinError> {
        // 1. Build prompt with garrison context
        // 2. Execute LLM call
        // 3. Check for tool calls, execute via arsenal
        // 4. Handle stop words and loops
        // 5. Update garrison with results
    }
}
```

## Integration Points

### LLM Providers

**Existing**: OpenAI adapter at `src/infrastructure/adapters/output/llm_adapter.rs`

**New adapters needed**:
- DeepSeek: `src/infrastructure/adapters/llm/deepseek_adapter.rs`
- Anthropic: `src/infrastructure/adapters/llm/anthropic_adapter.rs`

All must implement `LlmPort` trait with:
- `generate()` - Standard completion
- `generate_stream()` - Streaming completion
- `validate_model()` - Model validation
- Tool/function calling support

### MCP Protocol (Model Context Protocol)

**New integration for Arsenal tools:**

```rust
// STDIO-based MCP servers (command-line tools)
pub struct MCPStdioAdapter {
    command: String,
    args: Vec<String>,
    process: Option<Child>,
}

// SSE-based MCP servers (web services)  
pub struct MCPSseAdapter {
    endpoint: String,
    client: reqwest::Client,
}
```

Both implement `ArsenalPort` for tool discovery and invocation.

### Redis Queue (Existing)

Feature flag: `redis-queue` (enabled by default)
Location: `src/infrastructure/adapters/queue/redis_adapter.rs`

Use for:
- Async Paladin execution queuing
- Battalion task distribution
- Event-driven orchestration

### MinIO File Storage (Existing)

Feature flag: `s3-storage` (enabled by default)
Location: `src/infrastructure/adapters/file_storage/minio_adapter.rs`

Use for:
- Paladin state persistence (Citadel)
- Large context/document storage
- Execution artifacts

### Configuration

Multi-source config loading (`src/config/application_settings.rs`):

```yaml
# config.yml
paladin:
  default_model: "gpt-4"
  default_temperature: 0.7
  default_max_loops: 3
  timeout_seconds: 300

garrison:
  type: "sqlite"  # or "in_memory"
  path: "./garrison.db"
  max_entries: 1000

arsenal:
  mcp_servers:
    - name: "web_search"
      type: "stdio"
      command: "uvx"
      args: ["mcp-web-search"]

llm:
  openai:
    api_key: "${OPENAI_API_KEY}"
    base_url: "https://api.openai.com/v1"
  deepseek:
    api_key: "${DEEPSEEK_API_KEY}"
    base_url: "https://api.deepseek.com/v1"
  anthropic:
    api_key: "${ANTHROPIC_API_KEY}"
```

## Development Workflows

### Build & Test

```bash
make help                          # Show all commands
cargo build                        # Standard build
cargo test                         # Unit tests
make test-all                      # All tests (unit + integration)
make test-integration-docker       # Integration with Docker services

# Run specific tests
cargo test paladin_builder         # Tests matching pattern
cargo test --test formation_test   # Specific test file
```

### Docker Services

```bash
make dev                # Start all services
make services-up        # Services only (Redis, MinIO)
make redis-cli          # Redis CLI
make minio-console      # MinIO console (http://localhost:9001)
make health             # Check service status
```

### Adding New Features

**New Paladin Capability:**
1. Define in `core/platform/container/paladin.rs`
2. Add port method to `application/ports/output/paladin_port.rs`
3. Implement in `PaladinExecutionService`
4. Write tests first (TDD)
5. Update documentation

**New Battalion Pattern:**
1. Create domain type in `core/platform/container/battalion/`
2. Define execution port in `application/ports/output/battalion_port.rs`
3. Implement service in `application/use_cases/battalion/`
4. Add to Commander router
5. Write integration tests

**New LLM Provider:**
1. Create adapter in `infrastructure/adapters/llm/`
2. Implement `LlmPort` trait
3. Add feature flag if external dependency
4. Add configuration section
5. Write integration tests with mocks

**New Arsenal/MCP Integration:**
1. Implement `ArsenalPort` in `infrastructure/adapters/arsenal/`
2. Register in `ArsenalRegistry`
3. Add MCP protocol handling if applicable
4. Document tool capabilities

## Key Files Reference

**Essential Reading:**
- `docs/Design/Design_and_Architecture.md` - Full architecture guide
- `notes/hexagonal-arch.md` - Hexagonal pattern details
- `paladin_project_plan.md` - Project plan with all Epics
- `src/lib.rs` - Module structure overview

**Domain Entity Examples:**
- Existing: `src/core/platform/container/content_item.rs`
- Pattern to follow for Paladin entities

**Port/Adapter Examples:**
- Port: `src/application/ports/output/file_storage_port.rs`
- Adapter: `src/infrastructure/adapters/file_storage/minio_adapter.rs`

**Testing Examples:**
- Integration: `tests/integration/redis_queue_integration_test.rs`
- Functional: `tests/functional/content_lifecycle_test.rs`

## Security & Quality

**Always scan new code:**
```bash
make audit             # cargo-audit for vulnerabilities
make clean-code        # Format, lint, check
```

Run `snyk_code_scan` on first-party code per `.github/instructions/snyk_rules.instructions.md`.

## Current Epic Focus

Refer to `paladin_project_plan.md` for the current development phase. Key milestones:

| Milestone | Target | Deliverables |
|-----------|--------|--------------|
| M1: Alpha | Week 6 | Single Paladin execution working |
| M2: Beta | Week 12 | All Battalion types functional |
| M3: RC1 | Week 18 | Full feature complete |
| M4: Release | Week 20 | Production ready |

**When implementing, always:**
1. Check which Epic the work belongs to
2. Follow the technical design in the project plan
3. Write tests first (TDD)
4. Use the Medieval Military naming consistently
5. Maintain hexagonal architecture boundaries
6. Document all public APIs