<!-- refreshed: 2026-07-30 -->
# Architecture

**Analysis Date:** 2026-07-30

## System Overview

Paladin is an enterprise multi-agent orchestration framework built with **Hexagonal Architecture** (Ports & Adapters) and **Domain-Driven Design**. The system separates business logic from infrastructure through clear layer boundaries and port-based contracts.

```text
┌──────────────────────────────────────────────────────────────────┐
│                    Web API Layer (HTTP)                          │
│             `crates/paladin-web`, `src/infrastructure/web`       │
│  Agent Controllers → REST Endpoints → SSE Streaming              │
└──────────────────────────────┬──────────────────────────────────┘
                               │
┌──────────────────────────────▼──────────────────────────────────┐
│               Application Services Layer                         │
│     `src/application/services/` (Facade composition root)       │
│                                                                  │
│  PaladinExecutionService ──┐                                    │
│  FormationService          ├─> compose ports & leaf crates      │
│  PhalanxService            │                                    │
│  CampaignService           │                                    │
│  ChainOfCommandService  ───┘                                    │
└──────────────────────────────┬──────────────────────────────────┘
                               │
        ┌──────────────────────┼──────────────────────┐
        │                      │                      │
┌───────▼────────┐   ┌────────▼─────────┐   ┌──────▼─────────┐
│  Port Traits   │   │  Domain Entities │   │  Infrastructure│
│ (Contracts)    │   │   (Pure Logic)   │   │  (Adapters)    │
│                │   │                  │   │                │
│ paladin-ports  │   │ paladin-core     │   │ paladin-llm    │
│ `src/ports.rs` │   │ `src/core.rs`    │   │ paladin-memory │
│                │   │                  │   │ paladin-storage│
│ • LlmPort      │   │ • Paladin        │   │ paladin-content│
│ • GarrisonPort │   │ • Battalion      │   │ paladin-herald │
│ • ArsenalPort  │   │ • Formation      │   │ paladin-web    │
│ • RepositoryPort  │   │ • Garrison      │   │ (+ adapters in │
│ • QueuePort    │   │ • Arsenal       │   │  src/infra)    │
│ • etc.         │   │ • Citadel       │   │                │
└────────────────┘   │ • Sanctum       │   └────────────────┘
                     │ • Herald        │
                     └────────────────┘
```

## Component Responsibilities

| Component | Responsibility | File |
|-----------|----------------|------|
| **Paladin** | Autonomous AI agent with reasoning & memory | `crates/paladin-core/src/platform/container/paladin.rs` |
| **Battalion** | Multi-agent orchestration patterns | `crates/paladin-core/src/platform/container/battalion/` |
| **Garrison** | Conversation memory & context storage | `crates/paladin-core/src/platform/container/garrison.rs` |
| **Arsenal** | External tool execution (MCP protocol) | `crates/paladin-core/src/platform/container/arsenal/` |
| **Sanctum** | Vector search & semantic memory | `crates/paladin-core/src/platform/container/sanctum.rs` |
| **Citadel** | State persistence & recovery | `crates/paladin-core/src/platform/container/citadel.rs` |
| **Herald** | Output formatting & serialization | `crates/paladin-core/src/platform/container/herald.rs` |
| **PaladinExecutionService** | Core reasoning loop orchestration | `src/application/services/paladin/paladin_execution_service.rs` |
| **Formation Service** | Sequential Paladin execution | `crates/paladin-battalion/src/formation_service.rs` |
| **Phalanx Service** | Concurrent Paladin execution | `crates/paladin-battalion/src/phalanx_service.rs` |
| **Campaign Service** | DAG-based Paladin orchestration | `crates/paladin-battalion/src/campaign_service.rs` |
| **Chain of Command Service** | Hierarchical delegation | `crates/paladin-battalion/src/chain_of_command_service.rs` |
| **Commander** | Dynamic Battalion strategy router | `crates/paladin-battalion/src/commander.rs` |

## Pattern Overview

**Overall:** Three-layer Hexagonal Architecture with Facade pattern

**Key Characteristics:**
- **Core layer isolation**: Pure business logic with zero external dependencies
- **Port-based contracts**: Infrastructure integrates via trait definitions
- **Facade assembly**: Root `paladin` crate wires together all leaf crates
- **Distributed adapters**: Implementations live in dedicated crates (paladin-llm, paladin-memory, etc.)
- **Medieval military terminology**: Consistent domain language (Paladin, Battalion, Garrison, Arsenal, Citadel)
- **Builder pattern**: Complex objects use fluent construction
- **Node<T> pattern**: Generic entity wrapping with versioning support

## Layers

**Core Domain Layer (`paladin-core`):**
- Purpose: Pure business logic and domain entities
- Location: `crates/paladin-core/src/`
- Contains: Paladin, Battalion, Garrison, Arsenal, Citadel, Sanctum, Herald domain types
- Depends on: Serde, UUID, Chrono, PetGraph (no I/O or external services)
- Used by: All other crates

**Application Layer (Facade `paladin`):**
- Purpose: Application services and composition root
- Location: `src/application/services/`, `src/config/`
- Contains: PaladinExecutionService, Battalion services, orchestrators, registry services, CLI
- Depends on: Core + Ports + Infrastructure adapters
- Used by: Web API handlers, CLI commands, examples

**Infrastructure Layer:**
- Purpose: Adapter implementations and external integrations
- Location: `src/infrastructure/adapters/`, dedicated adapter crates
- Contains: LLM providers (OpenAI, Anthropic, DeepSeek), memory adapters (SQLite, in-memory, Qdrant), MCP protocol handlers, file storage (MinIO), queuing (Redis)
- Depends on: Ports + Core

**Port/Adapter Crates:**
- `paladin-ports`: Port trait definitions (contracts)
- `paladin-llm`: LLM adapter implementations
- `paladin-memory`: Garrison (conversation) & Sanctum (vector) adapters
- `paladin-storage`: Repository & persistence adapters
- `paladin-battalion`: Battalion orchestration services
- `paladin-content`: Content ingestion adapters
- `paladin-notifications`: Notification channel adapters
- `paladin-herald`: Output formatter adapters
- `paladin-web`: Web API (Axum)

## Data Flow

### Primary Request Path (Agent Execution)

1. **HTTP Request** (`crates/paladin-web/src/agent_controller.rs:execute`)
   - POST `/v1/agents/{id}/execute` with `{input: "query"}`

2. **Web Layer Routes** (`crates/paladin-web/src/app.rs`)
   - Auth middleware validates the `X-API-Key` header or an opaque server-issued bearer token
   - Rate limiter applied
   - Request timeout enforced

3. **PaladinExecutionService Orchestration** (`src/application/services/paladin/paladin_execution_service.rs:execute()`)
   - Validates Paladin configuration
   - Loads conversation history from Garrison if present
   - **Loop (up to max_loops):**
     a. **Prompt Construction**: Build LLM request (system prompt + conversation history + user input)
     b. **RAG Context Injection**: If Sanctum enabled, retrieve semantic context
     c. **LLM Call**: `llm_port.generate()` → respects circuit breaker, retries, timeout
     d. **Stop Word Detection**: Check response against configured stop words
     e. **Tool Call Handling**: If LLM returns `FunctionCall`:
        - `arsenal_port.execute()` → MCP protocol handling (STDIO/HTTP)
        - Format tool result and inject back into prompt
     f. **Response Formatting**: `herald_formatter.format()` for output structure
     g. **Memory Storage**: Save interaction to Garrison if enabled
     h. **Planning/Decomposition**: Optional - decompose into sub-tasks (Layer 1)

4. **LLM Adapter Invocation** (one of):
   - `paladin-llm/src/openai/adapter.rs` → OpenAI API
   - `paladin-llm/src/anthropic/adapter.rs` → Anthropic API
   - `paladin-llm/src/deepseek/adapter.rs` → DeepSeek API
   - `paladin-llm/src/mock.rs` → Mock for testing

5. **Tool Execution via Arsenal** (`src/infrastructure/adapters/arsenal/mcp_*.rs`)
   - MCP client discovers available tools
   - Executes via STDIO (`mcp_stdio_adapter.rs`) or HTTP (`mcp_streamable_http_adapter.rs`)
   - Resource controls enforce limits (memory, CPU, timeout)

6. **Response Collection & Return**
   - Execution metadata: loop count, token usage, timestamp
   - Final formatted output through Herald
   - HTTP 200 with `PaladinResult { output, loop_count, token_count, metadata }`

### Streaming Response Path

1. **HTTP Request** → `/v1/agents/{id}/execute/stream` (SSE)

2. **PaladinExecutionService::execute_stream()**
   - Returns `PaladinStream` (async stream of `PaladinStreamChunk`)

3. **Web Layer Conversion** (`crates/paladin-web/src/agent_controller.rs`)
   - Converts `PaladinStream` to Server-Sent Events
   - Each `PaladinStreamChunk` sent as `data: {json}\n\n`
   - Connection stays open until completion or error

### Multi-Agent Orchestration Path (Formation Example)

1. **HTTP Request** → Battalion endpoint

2. **FormationService::execute()** (`crates/paladin-battalion/src/formation_service.rs`)
   - Sequential execution: Paladin N → Paladin N+1
   - Output of N becomes input to N+1
   - Shares Garrison & context through handoff mechanism

3. **Maneuver DSL Optional** (`crates/paladin-battalion/src/maneuver/`)
   - Flow specifications: control structures (if/for/while), branching
   - Compiled to execution graph
   - Supports dynamic routing via Commander pattern

### State Management

- **Conversation Memory**: Stored in Garrison (SQLite or in-memory)
- **Semantic Memory**: Stored in Sanctum (Qdrant vector DB)
- **Persistent State**: Citadel recovery system
- **Job State**: Async job tracking in repository
- **Workflow State**: Configuration and status persisted in SQLite
- **Context Injection**: Token-budget aware context window management via RagRetrievalService

## Key Abstractions

**Paladin (Agent):**
- Purpose: Autonomous AI entity with configurable behavior
- Examples: `crates/paladin-core/src/platform/container/paladin.rs`
- Pattern: Node<PaladinData> wrapper with PaladinConfig
- Used by: ExecutionService, Formation, Phalanx, Campaign

**Battalion (Multi-Agent Patterns):**
- Purpose: Coordinate multiple Paladins with orchestration strategies
- Examples:
  - Formation: Sequential execution (`crates/paladin-battalion/src/formation_service.rs`)
  - Phalanx: Concurrent execution (`crates/paladin-battalion/src/phalanx_service.rs`)
  - Campaign: DAG execution (`crates/paladin-battalion/src/campaign_service.rs`)
  - Chain of Command: Hierarchical delegation (`crates/paladin-battalion/src/chain_of_command_service.rs`)
- Pattern: Each service takes a Battalion config, returns aggregated results

**Garrison (Memory):**
- Purpose: Persistent conversation history and context
- Examples:
  - SQLite: `paladin-memory/src/garrison/sqlite_garrison.rs`
  - In-memory: `paladin-memory/src/garrison/in_memory_garrison.rs`
- Pattern: Append-only conversation log with role (User/Assistant/Tool)

**Arsenal (Tools):**
- Purpose: Execute external capabilities via Model Context Protocol
- Examples: Web search, code execution, file operations
- Pattern: Tool registry + MCP transports (STDIO, HTTP)

**Sanctum (Vector Search):**
- Purpose: Semantic memory retrieval for RAG
- Examples: Qdrant adapter for vector similarity search
- Pattern: Embedding generation + vector storage + cosine similarity queries

## Entry Points

**Default Binary (`paladin`):**
- Location: `src/main.rs`
- Triggers: `cargo run` (no features) or as library
- Responsibilities: Service initialization (if running as binary), application setup

**CLI Binary (`paladin-cli`):**
- Location: `src/bin/paladin-cli.rs`
- Triggers: `cargo run --bin paladin-cli --features cli`
- Responsibilities: Command-line agent management, config wizard, interactive REPL

**Web Server Binary (`paladin-server`):**
- Location: `src/bin/paladin-server.rs` (uses `setup_and_run()` from `src/config/setup/mod.rs`)
- Triggers: `cargo run --bin paladin-server --features web-server`
- Responsibilities:
  - Loads YAML config (`config.yml`)
  - Initializes ServiceRunner (database, queues, file storage, event system)
  - Starts Axum HTTP server (default: `0.0.0.0:8080`)
  - Registers agents from config
  - Serves REST API + interactive Swagger docs

## Architectural Constraints

- **Threading**: Async/await throughout (Tokio runtime). No blocking operations in services.
- **Global state**: Minimal—ServiceRunner owns singleton instances (database, queue, event service), passed via Arc
- **Circular imports**: None at crate level. Dependency flow: infrastructure → ports ← core/application
- **Memory management**: Garrison (conversation) and Sanctum (vectors) are optional but recommended for production
- **Configuration**: YAML-based with environment variable overrides (per `.env` or system secrets)
- **Timeout enforcement**: Applied at multiple layers (HTTP timeout, LLM call timeout, loop count limit)

## Anti-Patterns

### Using `.unwrap()` in Libraries

**What happens:** Code panics on error instead of returning Result
**Why it's wrong:** Breaks resilience; aborts entire service
**Do this instead:** Return `Result<T, E>` and use `?` operator. See `src/application/services/paladin/paladin_execution_service.rs` for proper error propagation

### Bypassing Port Abstraction

**What happens:** Direct imports of adapter implementations (e.g., OpenAI client) bypass the LlmPort contract
**Why it's wrong:** Breaks decoupling; tests can't mock; swapping providers requires code changes
**Do this instead:** Always inject via Arc<dyn Port> trait objects. Example: `PaladinExecutionService::new(llm_port: Arc<dyn LlmPort>, …)`

### Blocking Operations in Async Context

**What happens:** Using `std::thread::sleep()` or blocking I/O in `.await` blocks Tokio worker
**Why it's wrong:** Starves other tasks; degrades concurrency
**Do this instead:** Use `tokio::time::sleep()` and async variants. See `src/application/services/paladin/paladin_execution_service.rs:execute()` for proper async patterns

### Storing Secrets in Config

**What happens:** API keys hardcoded in `config.yml` or committed to VCS
**Why it's wrong:** Credentials leaked in git history; security incident
**Do this instead:** Load from environment (see `config.example.yml` and `src/infrastructure/adapters/llm/config_bridge.rs`)

## Error Handling

**Strategy:** Layered error propagation with circuit breaker fallback

**Patterns:**

1. **Domain Errors** (paladin-core):
   - `PaladinError`: Agent configuration/execution failures
   - `BattalionError`: Orchestration failures
   - `GarrisonError`: Memory operation failures

2. **Port Errors** (paladin-ports):
   - Each port defines its own error enum
   - Impl `From<PortError>` for conversion at layer boundaries

3. **Application Errors** (`src/application/errors/`):
   - `PlanningError`, `PromptError`, `HandoffError`, `CitadelError`
   - Cross-cutting concerns

4. **Resilience Patterns**:
   - **Circuit Breaker** (`src/infrastructure/resilience/circuit_breaker.rs`): Detects cascading failures, returns fast after N failures
   - **Exponential Backoff** (`crates/paladin-battalion/src/retry.rs`): 100ms → 200ms → 400ms
   - **Timeout Enforcement** (`src/application/services/paladin/paladin_execution_service.rs`): Respects max_loops and duration_seconds

## Cross-Cutting Concerns

**Logging:**
- Framework: `log` crate with `env_logger`
- Level control: Set `RUST_LOG=paladin=debug` or in `.env`
- Structured: Via `log::info!()`, `log::error!()` macros

**Validation:**
- Input validation in service constructors (e.g., `PaladinBuilder::build()`)
- Config validation in Settings loading
- Request validation in HTTP handlers

**Authentication:**
- Web API: `X-API-Key` header or an opaque server-issued bearer token
- Adapter: `InMemoryTokenAuthAdapter` (`src/infrastructure/adapters/auth/`)
- Roles: `admin` (register agents) or `user` (invoke agents)

**Timeout:**
- Request-level (HTTP): `global_timeout_seconds` in web config
- Agent-level: `timeout_seconds` per agent or global default
- Loop-level: `max_loops` in PaladinConfig

---

*Architecture analysis: 2026-07-30*
