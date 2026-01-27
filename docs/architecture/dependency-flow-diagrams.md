# Dependency Flow Diagrams

Visual representation of dependency flows, module interactions, and data flows in Paladin.

## Table of Contents

- [Hexagonal Architecture Dependency Flow](#hexagonal-architecture-dependency-flow)
- [Layer Dependencies](#layer-dependencies)
- [Paladin Execution Flow](#paladin-execution-flow)
- [Battalion Orchestration Flows](#battalion-orchestration-flows)
- [Port and Adapter Dependencies](#port-and-adapter-dependencies)
- [Module Dependency Graph](#module-dependency-graph)

## Hexagonal Architecture Dependency Flow

**Critical Rule**: Dependencies flow **inward only** (from infrastructure → application → core).

```
┌───────────────────────────────────────────────────────────┐
│                   External Systems                         │
│  (OpenAI, DeepSeek, Redis, MinIO, PostgreSQL, etc.)       │
└────────────────────────┬──────────────────────────────────┘
                         │
                         │ HTTP/TCP/Protocol
                         │
┌────────────────────────▼──────────────────────────────────┐
│              Infrastructure Layer                          │
│                                                            │
│  ┌──────────────────────────────────────────────────┐    │
│  │          Adapters (Implementations)               │    │
│  │  - OpenAiAdapter    implements LlmPort            │    │
│  │  - DeepSeekAdapter  implements LlmPort            │    │
│  │  - SqliteGarrison   implements GarrisonPort       │    │
│  │  - McpStdioAdapter  implements ArsenalPort        │    │
│  │  - FileCitadel      implements CitadelPort        │    │
│  │  - MinioAdapter     implements FileStoragePort    │    │
│  └──────────────────────────────────────────────────┘    │
│                         │                                  │
│                         │ implements                       │
│                         │                                  │
└─────────────────────────┼──────────────────────────────────┘
                          │
                          │
┌─────────────────────────▼──────────────────────────────────┐
│              Application Layer                              │
│                                                             │
│  ┌──────────────────────────────────────────────────┐     │
│  │              Ports (Interfaces)                   │     │
│  │  trait LlmPort          - LLM abstraction         │     │
│  │  trait GarrisonPort     - Memory abstraction      │     │
│  │  trait ArsenalPort      - Tool abstraction        │     │
│  │  trait CitadelPort      - State abstraction       │     │
│  │  trait FileStoragePort  - Storage abstraction     │     │
│  └──────────────────────────────────────────────────┘     │
│                         │                                   │
│                         │ used by                           │
│                         │                                   │
│  ┌──────────────────────▼───────────────────────────┐     │
│  │              Use Cases (Services)                 │     │
│  │  - PaladinExecutionService                        │     │
│  │  - FormationService                               │     │
│  │  - PhalanxService                                 │     │
│  │  - CampaignService                                │     │
│  │  - CommanderService                               │     │
│  └──────────────────────────────────────────────────┘     │
│                         │                                   │
│                         │ operates on                       │
│                         │                                   │
└─────────────────────────┼───────────────────────────────────┘
                          │
                          │
┌─────────────────────────▼───────────────────────────────────┐
│                     Core Layer                               │
│                                                              │
│  ┌──────────────────────────────────────────────────┐      │
│  │           Domain Entities                         │      │
│  │  - Paladin (aggregate root)                       │      │
│  │  - Battalion (Formation, Phalanx, Campaign, CoC)  │      │
│  │  - Garrison (memory context)                      │      │
│  │  - Arsenal (tool registry)                        │      │
│  │  - Citadel (state persistence)                    │      │
│  └──────────────────────────────────────────────────┘      │
│                         │                                    │
│  ┌──────────────────────▼───────────────────────────┐      │
│  │              Base Types                           │      │
│  │  - Node<T>           - Entity wrapper             │      │
│  │  - Collection<T>     - Entity collections         │      │
│  │  - Field             - Field definitions          │      │
│  │  - Message           - Message types              │      │
│  └──────────────────────────────────────────────────┘      │
│                                                              │
│  NO DEPENDENCIES ON OUTER LAYERS                            │
│                                                              │
└──────────────────────────────────────────────────────────────┘
```

## Layer Dependencies

### Allowed Dependencies

```
Infrastructure ─────────can import───────────> Application
                                               
Infrastructure ─────────can import───────────> Core

Application ────────────can import───────────> Core

Core ───────────────────CANNOT IMPORT────────X Infrastructure
Core ───────────────────CANNOT IMPORT────────X Application
Application ────────────CANNOT IMPORT────────X Infrastructure
```

### Module Import Rules

```rust
// ✅ ALLOWED: Infrastructure imports application and core
// src/infrastructure/adapters/llm/openai_adapter.rs
use crate::application::ports::output::llm_port::LlmPort;  // ✅
use crate::core::platform::container::paladin::Paladin;    // ✅

// ✅ ALLOWED: Application imports core
// src/application/use_cases/paladin/paladin_execution_service.rs
use crate::core::platform::container::paladin::Paladin;    // ✅
use crate::application::ports::output::llm_port::LlmPort;  // ✅

// ❌ FORBIDDEN: Core imports application
// src/core/platform/container/paladin.rs
use crate::application::ports::output::llm_port::LlmPort;  // ❌ FORBIDDEN!

// ❌ FORBIDDEN: Core imports infrastructure
// src/core/platform/container/paladin.rs
use crate::infrastructure::adapters::llm::OpenAiAdapter;   // ❌ FORBIDDEN!

// ❌ FORBIDDEN: Application imports infrastructure
// src/application/use_cases/paladin/paladin_execution_service.rs
use crate::infrastructure::adapters::llm::OpenAiAdapter;   // ❌ FORBIDDEN!
```

## Paladin Execution Flow

End-to-end flow for executing a single Paladin:

```
┌──────────┐
│  Client  │
└────┬─────┘
     │
     │ execute("input")
     │
     ▼
┌────────────────────────────────────┐
│  PaladinExecutionService           │  (Application Layer)
│  (Use Case)                        │
└────┬─────────────────────────┬─────┘
     │                         │
     │ 1. Build prompt         │ 2. Load context
     │                         │
     ▼                         ▼
┌──────────────────┐    ┌────────────────────┐
│  Garrison        │    │  GarrisonPort      │
│  (Core Domain)   │◄───│  (Interface)       │
└──────────────────┘    └────────┬───────────┘
                                 │ implements
                                 ▼
                        ┌────────────────────┐
                        │ SqliteGarrison     │
                        │ (Infrastructure)   │
                        └────────────────────┘

     │
     │ 3. Call LLM
     │
     ▼
┌──────────────────┐    ┌────────────────────┐
│  LlmPort         │    │  OpenAiAdapter     │
│  (Interface)     │◄───│  (Infrastructure)  │
└────────┬─────────┘    └────────────────────┘
         │                       │
         │                       │ HTTPS
         │                       ▼
         │              ┌────────────────────┐
         │              │  OpenAI API        │
         │              │  (External)        │
         │              └────────────────────┘
         │
         │ 4. Process tool calls (if any)
         │
         ▼
┌──────────────────┐    ┌────────────────────┐
│  Arsenal         │    │  ArsenalPort       │
│  (Core Domain)   │◄───│  (Interface)       │
└──────────────────┘    └────────┬───────────┘
                                 │ implements
                                 ▼
                        ┌────────────────────┐
                        │ McpStdioAdapter    │
                        │ (Infrastructure)   │
                        └────────────────────┘

     │
     │ 5. Check stop conditions
     │
     ▼
┌──────────────────┐
│  Loop control    │
│  - max_loops     │
│  - stop_words    │
│  - timeout       │
└────────┬─────────┘
         │
         │ 6. Save results
         │
         ▼
┌──────────────────┐
│  Update Garrison │
│  with results    │
└────────┬─────────┘
         │
         │ 7. Return
         │
         ▼
┌──────────────────┐
│  PaladinResult   │
└──────────────────┘
```

## Battalion Orchestration Flows

### Formation (Sequential) Flow

```
┌──────────────┐
│ Formation    │
│ Service      │
└──────┬───────┘
       │
       │ execute("input")
       │
       ▼
┌──────────────┐       ┌──────────────┐       ┌──────────────┐
│ Paladin 1    │──────►│ Paladin 2    │──────►│ Paladin 3    │
└──────────────┘       └──────────────┘       └──────────────┘
       │                      │                      │
       │ output 1             │ output 2             │ output 3
       │                      │                      │
       └──────────────────────┴──────────────────────┘
                              │
                              ▼
                    ┌──────────────────┐
                    │ Aggregated       │
                    │ Result           │
                    └──────────────────┘

Data Flow:
  input → Paladin 1 → output 1 → Paladin 2 → output 2 → Paladin 3 → output 3
```

### Phalanx (Parallel) Flow

```
┌──────────────┐
│ Phalanx      │
│ Service      │
└──────┬───────┘
       │
       │ execute("input")
       │
       ├──────────────┬──────────────┬──────────────┐
       │              │              │              │
       ▼              ▼              ▼              ▼
┌──────────────┐ ┌──────────────┐ ┌──────────────┐ ┌──────────────┐
│ Paladin 1    │ │ Paladin 2    │ │ Paladin 3    │ │ Paladin 4    │
└──────┬───────┘ └──────┬───────┘ └──────┬───────┘ └──────┬───────┘
       │              │              │              │
       │ output 1     │ output 2     │ output 3     │ output 4
       │              │              │              │
       └──────────────┴──────────────┴──────────────┘
                      │
                      ▼
            ┌──────────────────┐
            │ Merge Results    │
            │ (all outputs)    │
            └──────────────────┘

All Paladins receive same input, execute concurrently
```

### Campaign (DAG) Flow

```
┌──────────────┐
│ Campaign     │
│ Service      │
└──────┬───────┘
       │
       │ execute("input")
       │
       ▼
┌──────────────┐
│ Paladin A    │ (entry point)
└──────┬───────┘
       │
       ├──────────────┬──────────────┐
       │              │              │
       ▼              ▼              ▼
┌──────────────┐ ┌──────────────┐ ┌──────────────┐
│ Paladin B    │ │ Paladin C    │ │ Paladin D    │
└──────┬───────┘ └──────┬───────┘ └──────────────┘
       │              │
       └──────┬───────┘
              │
              ▼
       ┌──────────────┐
       │ Paladin E    │ (merge point)
       └──────┬───────┘
              │
              ▼
       ┌──────────────┐
       │ Final Result │
       └──────────────┘

Dependencies:
  A → B, C, D (parallel after A)
  B, C → E (E waits for both B and C)
  D is independent branch
```

### Chain of Command (Hierarchical) Flow

```
┌──────────────────┐
│ Commander        │ (top-level Paladin)
│ Paladin          │
└────────┬─────────┘
         │
         │ Analyzes task
         │
         ├──────────────┬──────────────┐
         │              │              │
         │ delegate     │ delegate     │ delegate
         │              │              │
         ▼              ▼              ▼
┌────────────┐  ┌────────────┐  ┌────────────┐
│ Lieutenant │  │ Lieutenant │  │ Lieutenant │
│ Paladin 1  │  │ Paladin 2  │  │ Paladin 3  │
└────────┬───┘  └────────┬───┘  └────────┬───┘
         │              │              │
         │ report       │ report       │ report
         │              │              │
         └──────────────┴──────────────┘
                        │
                        ▼
              ┌────────────────┐
              │ Commander      │
              │ Synthesizes    │
              └────────────────┘

Commander decides which lieutenants to delegate to based on task
```

## Port and Adapter Dependencies

### LLM Provider Chain

```
┌────────────────────────────────────────────────────────────┐
│  Application Layer - Ports                                  │
│                                                             │
│  pub trait LlmPort: Send + Sync {                          │
│      async fn generate(&self, ...) -> Result<LlmResponse>; │
│      async fn generate_stream(&self, ...) -> ...;          │
│      fn validate_model(&self, ...) -> Result<()>;          │
│  }                                                          │
│                                                             │
└────────────────┬───────────────────────────────────────────┘
                 │
                 │ implemented by
                 │
                 ├──────────────┬──────────────┬──────────────┐
                 │              │              │              │
                 ▼              ▼              ▼              ▼
┌──────────────────┐ ┌──────────────────┐ ┌──────────────────┐ ┌──────────────────┐
│ OpenAiAdapter    │ │ DeepSeekAdapter  │ │ AnthropicAdapter │ │ CustomAdapter    │
│ (Infrastructure) │ │ (Infrastructure) │ │ (Infrastructure) │ │ (Infrastructure) │
└────────┬─────────┘ └────────┬─────────┘ └────────┬─────────┘ └────────┬─────────┘
         │                    │                    │                    │
         │ HTTPS              │ HTTPS              │ HTTPS              │ Custom
         │                    │                    │                    │
         ▼                    ▼                    ▼                    ▼
┌──────────────────┐ ┌──────────────────┐ ┌──────────────────┐ ┌──────────────────┐
│ OpenAI API       │ │ DeepSeek API     │ │ Anthropic API    │ │ Custom Provider  │
│ (External)       │ │ (External)       │ │ (External)       │ │ (External)       │
└──────────────────┘ └──────────────────┘ └──────────────────┘ └──────────────────┘
```

### Garrison Storage Chain

```
┌────────────────────────────────────────────────────────────┐
│  Application Layer - Ports                                  │
│                                                             │
│  pub trait GarrisonPort: Send + Sync {                     │
│      async fn add_entry(&self, ...) -> Result<()>;         │
│      async fn get_entries(&self, ...) -> Result<Vec<...>>; │
│      async fn search(&self, ...) -> Result<Vec<...>>;      │
│  }                                                          │
│                                                             │
└────────────────┬───────────────────────────────────────────┘
                 │
                 │ implemented by
                 │
                 ├──────────────┬──────────────┐
                 │              │              │
                 ▼              ▼              ▼
┌──────────────────┐ ┌──────────────────┐ ┌──────────────────┐
│ InMemoryGarrison │ │ SqliteGarrison   │ │ RedisGarrison    │
│ (Infrastructure) │ │ (Infrastructure) │ │ (Infrastructure) │
└────────┬─────────┘ └────────┬─────────┘ └────────┬─────────┘
         │                    │                    │
         │ In-process         │ SQLite             │ Redis protocol
         │                    │                    │
         ▼                    ▼                    ▼
┌──────────────────┐ ┌──────────────────┐ ┌──────────────────┐
│ HashMap/Vec      │ │ garrison.db      │ │ Redis Server     │
│ (Memory)         │ │ (File)           │ │ (External)       │
└──────────────────┘ └──────────────────┘ └──────────────────┘
```

## Module Dependency Graph

### Core Module Dependencies

```
core/
├── base/                    (no dependencies)
│   ├── node.rs
│   ├── collection.rs
│   ├── field.rs
│   └── message.rs
│
└── platform/
    └── container/
        ├── paladin.rs       (depends on: base)
        ├── garrison.rs      (depends on: base)
        ├── arsenal.rs       (depends on: base)
        ├── citadel.rs       (depends on: base)
        └── battalion/
            ├── mod.rs       (depends on: base, paladin)
            ├── formation.rs (depends on: base, paladin, mod)
            ├── phalanx.rs   (depends on: base, paladin, mod)
            ├── campaign.rs  (depends on: base, paladin, mod)
            └── chain_of_command.rs (depends on: base, paladin, mod)
```

### Application Module Dependencies

```
application/
├── ports/
│   ├── input/              (depends on: core)
│   └── output/
│       ├── llm_port.rs     (depends on: core)
│       ├── garrison_port.rs (depends on: core)
│       ├── arsenal_port.rs (depends on: core)
│       └── citadel_port.rs (depends on: core)
│
└── use_cases/
    ├── paladin/            (depends on: core, ports)
    │   ├── paladin_builder.rs
    │   └── paladin_execution_service.rs
    └── battalion/          (depends on: core, ports)
        ├── formation_service.rs
        ├── phalanx_service.rs
        ├── campaign_service.rs
        └── commander.rs
```

### Infrastructure Module Dependencies

```
infrastructure/
├── adapters/
│   ├── llm/                (depends on: core, application/ports)
│   │   ├── openai_adapter.rs
│   │   ├── deepseek_adapter.rs
│   │   └── anthropic_adapter.rs
│   │
│   ├── garrison/           (depends on: core, application/ports)
│   │   ├── in_memory_garrison.rs
│   │   └── sqlite_garrison.rs
│   │
│   ├── arsenal/            (depends on: core, application/ports)
│   │   ├── mcp_stdio_adapter.rs
│   │   └── mcp_sse_adapter.rs
│   │
│   └── citadel/            (depends on: core, application/ports)
│       └── file_citadel.rs
│
└── repositories/           (depends on: core, application)
```

## Dependency Validation

### Enforcing Boundaries

```rust
// Use linting rules to enforce boundaries
// .cargo/config.toml or rust-toolchain.toml

// Or use cargo-modules to visualize:
// cargo install cargo-modules
// cargo modules generate graph --lib | dot -Tpng > modules.png
```

### Testing Boundaries

```rust
#[cfg(test)]
mod architecture_tests {
    use std::path::Path;
    
    #[test]
    fn test_core_has_no_infrastructure_dependencies() {
        // Parse core source files
        // Verify no imports from infrastructure
        assert!(verify_no_imports(
            "src/core",
            &["crate::infrastructure"]
        ));
    }
    
    #[test]
    fn test_core_has_no_application_dependencies() {
        assert!(verify_no_imports(
            "src/core",
            &["crate::application"]
        ));
    }
    
    #[test]
    fn test_application_has_no_infrastructure_dependencies() {
        assert!(verify_no_imports(
            "src/application",
            &["crate::infrastructure"]
        ));
    }
}
```

## Next Steps

- **[Overview](overview.md)** - System architecture overview
- **[Hexagonal Design](hexagonal-design.md)** - Ports and adapters details
- **[Domain Model](domain-model.md)** - Domain entities and relationships
- **[Design Patterns](design-patterns.md)** - Common patterns used
