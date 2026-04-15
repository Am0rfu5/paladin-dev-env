# Public API Surface Audit - Epic 2: Harden Port Traits

**Created:** 2026-04-15
**Epic:** Epic 2 - Milestone 4, Tier 1
**Purpose:** Classify all currently exported types to determine the stable public API surface

---

## Current Export Pattern

`src/lib.rs` performs glob re-exports of ALL modules:
```rust
pub use application::*;
pub use config::*;
pub use core::*;
pub use infrastructure::*;
```

This exposes **every public type** from the entire codebase (~200+ types estimated).

---

## Classification Categories

### ✅ PORT TRAITS (Should Remain Public - PRIMARY API)

These are the stable public API contract. **Always public, never feature-gated.**

#### Output Ports (`src/application/ports/output/`)
1. `LlmPort` - LLM provider abstraction
2. `GarrisonPort` - Memory system abstraction
3. `SanctumPort` - Long-term memory abstraction
4. `EmbeddingPort` - Embedding generation abstraction
5. `ArsenalPort` - Tool/capability registry abstraction
6. `BattalionPort` - Multi-agent orchestration abstraction
7. `CitadelPort` - State persistence abstraction
8. `QueuePort` - Queue system abstraction
9. `NotificationPort` - Notification system abstraction
10. `FileStoragePort` - File storage abstraction
11. `PaladinPort` - Paladin execution abstraction (if exists as separate trait)
12. `PaladinExecutorPort` - Paladin executor abstraction
13. `PaladinRegistry` - Paladin registry abstraction
14. `SchedulerPort` - Scheduler abstraction
15. `LogPort` - Logging abstraction
16. `ContentDeliveryPort` - Content delivery abstraction
17. `SearchEnginePort` - Search engine abstraction
18. `VisionPort` - Vision processing abstraction
19. `VisionLlmPort` - Vision-enabled LLM abstraction

#### Input Ports (`src/application/ports/input/`)
20. `ContentInputPort` - Content ingestion
21. `DocumentPort` - Document processing
22. `ListenerPort` - Event listening
23. `MlPort` - Machine learning operations
24. `NlpPort` - Natural language processing
25. `RpcPort` - RPC communications

**Total Port Traits: ~25**

---

### ✅ ESSENTIAL DOMAIN ENTITIES (Should Remain Public)

Core domain types required by port trait signatures and user code.

#### Paladin Domain (`src/core/platform/container/`)
- `Paladin` - Core agent entity (Node<PaladinData>)
- `PaladinData` - Paladin data payload
- `PaladinConfig` - Paladin configuration
- `PaladinResult` - Paladin execution result
- `PaladinStatus` - Paladin status enum
- `PaladinError` - Paladin error type

#### Application Services & Utilities (Should Remain Public)
- `PaladinExecutionService` - Core execution service (used in tests/examples)
- `CircuitBreaker` - Circuit breaker pattern utility (used in tests/examples)
- `PaladinBuilder` - Primary builder (used extensively)
- Formation/Phalanx/Campaign execution services (used in examples)

#### Battalion Domain (`src/core/platform/container/battalion/`)
- `Battalion` - Base battalion type (if exists)
- `Formation` - Sequential execution pattern
- `Phalanx` - Parallel execution pattern
- `Campaign` - Graph-based orchestration
- `ChainOfCommand` - Hierarchical delegation
- `Council` - Multi-agent discussion pattern
- `Grove` - Routing/selection pattern
- `Conclave` - Expert panel pattern
- `BattalionError` - Battalion error type
- `BattalionResult` - Battalion result type

#### Garrison Domain
- `Garrison` - Memory system entity
- `GarrisonData` - Memory data type
- `GarrisonError` - Garrison error type
- `MemoryEntry` - Individual memory item (if public)

#### Arsenal Domain (`src/core/platform/container/arsenal/`)
- `Arsenal` - Tool registry entity
- `Armament` - Individual tool/capability
- `ArmamentCall` - Tool invocation request
- `ArmamentResult` - Tool execution result
- `ArsenalError` - Arsenal error type
- `ArsenalRegistry` - Registry interface (if separate from port)

#### Sanctum Domain
- `Sanctum` - Long-term memory entity
- `Memory` - Memory item
- `MemoryBuilder` - Memory construction
- `SanctumError` - Sanctum error type

#### Citadel Domain
- `Citadel` - State persistence entity
- `CitadelError` - Citadel error type

#### Base Types (`src/core/base/`)
- `Node<T>` - Base entity pattern (critical for framework)
- `Collection` - Collection management
- `Field` - Field definitions
- `Message` - Message types

#### Herald Domain (Output Formatting)
- `Herald` - Output formatter
- `HeraldError` - Herald error type
- `StreamChunk` - Streaming output chunk
- `StreamChunkBuilder` - Stream chunk construction
- `ExecutionMetadata` - Execution metadata
- `ExecutionMetadataBuilder` - Metadata construction

#### Supporting Domain Types
- `Document` - Document entity (if used in public APIs)
- `Content` - Content entity (if used in public APIs)
- `Task` - Task entity
- `Job` - Job entity
- `Trigger` - Trigger entity
- `Workflow` - Workflow entity
- `Comment` - Comment entity
- `Tag` - Tag entity

**Total Domain Entities: ~50-60 types**

---

### ✅ BUILDERS (Should Remain Public)

Builder pattern types for constructing domain entities.

- `PaladinBuilder` - Primary builder for Paladin construction
- `PaladinConfigBuilder` - Configuration builder
- `CommanderBuilder` - Battalion commander builder
- `CouncilBuilder` - Council pattern builder
- `GroveBuilder` - Grove pattern builder
- `MemoryBuilder` - Sanctum memory builder
- `LogEntryBuilder` - Log entry builder
- `StreamChunkBuilder` - Herald stream chunk builder
- `ExecutionMetadataBuilder` - Execution metadata builder

**Total Builders: ~9 core builders (CLI builders excluded)**

---

### ✅ CONFIGURATION TYPES (Should Remain Public)

Configuration structs needed by users.

- `ApplicationSettings` - Main application configuration
- `PaladinConfig` - Paladin-specific config (may be part of domain)
- `AutonomousConfig` - Autonomous agent configuration
- `LogConfig` - Logging configuration (if separate)
- Subsystem configs (selected types only, not all implementation details)

**Total Configuration Types: ~5-10 key types**

---

### ✅ ERROR TYPES (Should Remain Public)

All domain error enums for proper error handling.

- `PaladinError` - Paladin errors
- `BattalionError` - Battalion errors
- `GarrisonError` - Garrison errors
- `ArsenalError` - Arsenal errors
- `SanctumError` - Sanctum errors
- `CitadelError` - Citadel errors
- `HeraldError` - Herald errors
- `LlmError` - LLM-related errors (if separate)
- `ConfigError` - Configuration errors
- `QueueError` - Queue errors
- `NotificationError` - Notification errors
- `FileStorageError` - File storage errors

**Total Error Types: ~15-20 error enums**

---

### ❌ INTERNAL ADAPTERS (Should Be Restricted - pub(crate))

Infrastructure implementations that should NOT be part of public API.

#### LLM Adapters (`src/infrastructure/adapters/llm/`)
- `OpenAIAdapter` - OpenAI implementation
- `AnthropicAdapter` - Anthropic implementation
- `DeepSeekAdapter` - DeepSeek implementation
- All adapter internal types, configs, and utilities

#### Garrison Adapters (`src/infrastructure/adapters/garrison/`)
- `InMemoryGarrison` - In-memory implementation
- `SqliteGarrison` - SQLite implementation
- All adapter internal types

#### Sanctum Adapters (`src/infrastructure/adapters/sanctum/`)
- Concrete sanctum implementations
- All adapter internal types

#### Queue Adapters (`src/infrastructure/adapters/queue/`)
- `RedisAdapter` - Redis queue implementation
- All adapter internal types

#### File Storage Adapters (`src/infrastructure/adapters/file_storage/`)
- `MinioAdapter` - MinIO/S3 implementation
- All adapter internal types

#### Notification Adapters (`src/infrastructure/adapters/notification/`)
- Email, SMS, push notification adapters
- All adapter internal types

#### Arsenal Adapters (`src/infrastructure/adapters/arsenal/`)
- `MCPStdioAdapter` - MCP STDIO transport
- `MCPSseAdapter` - MCP SSE transport
- All adapter internal types

#### Embedding Adapters (`src/infrastructure/adapters/embedding/`)
- OpenAI, Qdrant, or other embedding adapters
- All adapter internal types

#### Vision Adapters (`src/infrastructure/adapters/vision/`)
- Vision pipeline implementations
- All adapter internal types

**Total Adapter Modules: ~50-80 implementation types**

---

### ❌ INTERNAL REPOSITORIES (Should Be Restricted - pub(crate))

Database repository implementations.

#### MySQL Repositories (`src/infrastructure/repositories/mysql/`)
- All MySQL-specific repository implementations
- Connection management types
- Query builders

#### SQLite Repositories (`src/infrastructure/repositories/sqlite/`)
- All SQLite-specific repository implementations
- Connection management types

**Total Repository Types: ~20-30 implementation types**

---

### ❌ INTERNAL MANAGERS (Should Be Restricted - pub(crate))

Manager-layer services (these may need refactoring to application layer per Tier 3).

#### Manager Services (`src/manager/`)
- `Scheduler` - Scheduling service
- `QueueService` - Queue management service
- `EventManager` - Event management service
- All manager internal types

**Total Manager Types: ~10-15 types**

---

### ❌ CLI MODULES (Should Be Restricted - pub(crate))

CLI is a binary-only concern, not library API (Epic 3 will fully isolate).

#### CLI (`src/application/cli/`)
- All CLI command handlers
- All CLI formatters
- All CLI interactive prompts
- All CLI utilities
- `ProgressBarBuilder`, `PromptBuilder`, etc.

**Total CLI Types: ~50-100 types (12,000 LOC)**

---

### ❌ WEB SERVER MODULES (Should Be Restricted - pub(crate))

Web server is optional and should be feature-gated (Epic 1 coordination).

#### Web (`src/infrastructure/web/`)
- All route handlers
- All middleware
- WebSocket infrastructure
- All web-related types

**Total Web Types: ~30-50 types**

---

### 🤔 NEEDS DISCUSSION (Ambiguous Cases)

Types that require team discussion before classification.

1. **`ApplicationSettings` sub-configs**: Should all sub-configs be public or only top-level?
2. **Factory/Registry types**: Are these part of public API or internal?
3. **Event types**: Should domain events be public for extensibility?
4. **Utility types**: Helper types like `CircuitBreaker`, `RateLimiter` - public or internal?
   - **FINDING:** `CircuitBreaker` is used in examples (`vision_battalion.rs`, `vision_analysis.rs`)
   - **RECOMMENDATION:** Make public as part of builder/execution utilities
5. **Execution Services**: `PaladinExecutionService`, `FormationExecutionService`, etc.
   - **FINDING:** Used directly in examples (e.g., `vision_battalion.rs`, `formation_sequential.rs`)
   - **QUESTION:** Should users use these directly or only through port traits?
   - **RECOMMENDATION:** If needed publicly, export explicitly; otherwise provide alternative patterns
6. **Adapter Direct Instantiation**: Examples instantiate adapters directly
   - **FINDING:** `OpenAIAdapter`, `OpenAIConfig`, `InMemorySanctum` used in examples
   - **CURRENT:** Acceptable for examples and testing scenarios
   - **DECISION:** Mark adapters as `#[doc(hidden)]` but leave public for advanced use cases?
   - **ALTERNATIVE:** Keep adapters internal; examples use factory functions
7. **Maneuver DSL types**: If Maneuver is a user-facing DSL, types should be public
8. **User/UserGroup types**: Are these framework concepts or domain concepts?
9. **Handoff types**: Agent handoff - public domain concept or internal?
10. **Planning types**: Autonomous planning - public or internal?
11. **Prompt types**: Prompt management - public or internal?
12. **Vision types**: Vision entity (`VisionContent`, `ImageDetail`, `VisionError`) - used in examples
    - **FINDING:** Used in examples (`vision_battalion.rs`, `vision_analysis.rs`)
    - **RECOMMENDATION:** Public as domain entities

**Total Ambiguous: ~12 items requiring decisions**

**Key Findings from Examples Scan:**
- Adapters currently instantiated directly (OpenAIAdapter, InMemorySanctum)
- CircuitBreaker used as external utility
- Vision types (VisionContent, ImageDetail, VisionError) used directly
- Execution services sometimes used instead of port traits
- Examples mix port trait usage with direct adapter usage

**Recommendation:** Create clear guidance on:
1. When to use port traits vs. adapters
2. When to use builders vs. execution services
3. Provide convenience factory functions for common adapter setups

---

## Summary Statistics

| Category | Estimated Count | Visibility |
|----------|----------------|------------|
| **Port Traits** | 25 | **PUBLIC** (Stable API) |
| **Domain Entities** | 50-60 | **PUBLIC** (Essential) |
| **Builders** | 9 | **PUBLIC** (Essential) |
| **Configuration** | 5-10 | **PUBLIC** (Essential) |
| **Error Types** | 15-20 | **PUBLIC** (Essential) |
| **Adapters** | 50-80 | **INTERNAL** (pub(crate)) |
| **Repositories** | 20-30 | **INTERNAL** (pub(crate)) |
| **Managers** | 10-15 | **INTERNAL** (pub(crate)) |
| **CLI** | 50-100 | **INTERNAL** (pub(crate) / Epic 3) |
| **Web Server** | 30-50 | **INTERNAL** (pub(crate) / feature-gated) |
| **Needs Discussion** | 10-15 | **TBD** |

### Target Public API Surface
**Proposed: 104-124 public types** (Port Traits + Domain + Builders + Config + Errors)

**Current estimate: ~200+ types publicly exposed**

**Reduction target: ~40-50% reduction in public API surface**

---

## Next Steps (Tasks 1.6 - 1.9)

1. Scan `examples/` directory for commonly used types
2. Scan `tests/integration/` for commonly used types
3. Refine "Needs Discussion" items
4. Generate baseline count with cargo-public-api (once tool issue resolved)
5. Finalize deprecation strategy for transitional types

---

## Open Questions for Resolution

1. Should `Node<T>`, `Collection`, `Field`, `Message` be public? (Recommendation: YES - core patterns)
2. Should adapter factory functions be public even if adapter structs are internal?
3. Should we provide any `pub use` convenience re-exports for common combinations?
4. What is the deprecation timeline for types we're restricting?
5. Do we need a `paladin::prelude::*` module for common imports?

---

**Status:** Initial audit complete. Ready for example/test scanning and refinement.
