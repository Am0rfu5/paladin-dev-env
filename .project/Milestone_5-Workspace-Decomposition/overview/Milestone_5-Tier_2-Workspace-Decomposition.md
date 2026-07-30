# Milestone 2: Workspace Decomposition

**Project:** Paladin Framework Refactoring Initiative
**Milestone:** Tier 2 — Cargo Workspace Split
**Status:** Planning
**Target Start:** Upon completion of Milestone 1 (Tier 1)
**Target Completion:** TBD
**Document Version:** 1.0
**Last Updated:** 2026-04-14

---

## Executive Summary

Milestone 2 decomposes the Paladin monolithic crate into a Cargo workspace of purpose-built crates. This is the structural refactor that converts the module-level hexagonal boundaries — validated and hardened during Milestone 1 — into crate-level boundaries with enforced dependency direction, independent compilation, and explicit public API surfaces.

The workspace decomposition follows a precise extraction order designed to minimize coupling risk: `paladin-core` first (pure domain types with near-zero external dependencies), `paladin-ports` second (the trait contracts that define the architectural backbone), `paladin-battalion` third (the primary public orchestration runtime), `paladin-llm` fourth (provider adapters behind feature flags), and `paladin-memory` fifth (garrison and sanctum storage adapters). A root `paladin` facade crate re-exports the workspace for backward compatibility.

### Prerequisites (Completed in Milestone 1)

- Feature flags expanded to cover the full optional surface.
- Port traits hardened as the stable public API contract with curated `lib.rs` exports.
- CLI isolated from the library compilation path behind the `cli` feature flag.
- CI pipeline configured with feature-flag matrix testing.
- `STABLE_API.md` published documenting the public API surface.

### Success Criteria

- The workspace compiles successfully with `cargo build` from the workspace root.
- All existing tests (1,487+) pass when run from the workspace root via `cargo test --workspace`.
- `paladin-core` has zero dependencies on any other workspace crate and depends only on `serde`, `uuid`, `chrono`, and `thiserror`.
- `paladin-battalion` depends only on `paladin-core` and `paladin-ports`, with no dependency on infrastructure adapters.
- A downstream consumer can depend on `paladin-core` + `paladin-ports` + `paladin-battalion` without pulling in any LLM provider, storage backend, web server, or notification dependency.
- Per-crate incremental build times are measurably faster than the monolithic baseline (target: ≥50% reduction for isolated crate changes).
- The root `paladin` facade crate provides backward-compatible re-exports for existing consumers.
- `cargo doc --workspace --no-deps` produces clean documentation with no broken intra-doc links.

---

## Milestone Scope & Boundaries

### In Scope

- Cargo workspace creation and root `Cargo.toml` configuration.
- Extraction of `paladin-core` crate (domain entities and base types).
- Extraction of `paladin-ports` crate (port trait definitions).
- Extraction of `paladin-battalion` crate (orchestration runtime).
- Extraction of `paladin-llm` crate (LLM provider adapters with per-provider feature flags).
- Extraction of `paladin-memory` crate (garrison and sanctum storage adapters).
- Root `paladin` facade crate with backward-compatible re-exports.
- Workspace-level CI configuration.
- Migration guide for workspace consumers.

### Out of Scope

- Splitting `application_settings.rs` into per-domain configs (Tier 3).
- Moving manager-layer services to the application use-cases layer (Tier 3).
- Relocating the Maneuver DSL parser co-location (Tier 3).
- Relocating `CircuitBreaker` to infrastructure layer (Tier 3).
- Per-provider LLM crates (a single `paladin-llm` crate with feature flags is preferred over fragmentation).
- Extraction of content processing, notification, or web server into separate crates (future consideration beyond Tier 3).

### Dependencies & Assumptions

- Milestone 1 is fully completed: feature flags are in place, port traits are hardened, CLI is isolated.
- The team has access to all downstream consumer repositories for compatibility testing.
- CI infrastructure supports workspace-level builds and per-crate test isolation.
- No circular dependencies exist between the identified crate boundaries (verified during Milestone 1 port hardening).

### Risk Register

| Risk | Likelihood | Impact | Mitigation |
|------|-----------|--------|------------|
| Circular dependency discovered during extraction | Medium | High | Strict extraction order (core → ports → battalion); dependency analysis tooling (`cargo-depgraph`) run before each extraction |
| Cross-crate type visibility breaks internal test compilation | High | Medium | Maintain `pub(crate)` escape hatches during transition; refactor tests incrementally per crate |
| Linker overhead increases cold build times despite faster incremental builds | Medium | Low | Benchmark cold and incremental builds; accept cold build regression if incremental improvement exceeds 50% |
| `battalion/mod.rs` references `application::ports::output::paladin_port::PaladinResult` creating an upward dependency from core to application | High | High | Resolve during Epic 2 by defining `BattalionResult` in `paladin-core` or introducing a shared result type in `paladin-ports` |
| Existing examples and integration tests have deeply nested import paths | Medium | Medium | Facade crate re-exports preserve old paths; provide `use paladin::prelude::*` convenience module |
| Third-party dependency version conflicts across workspace crates | Low | Medium | Use workspace-level `[workspace.dependencies]` for shared dependency version management |

---

## Target Workspace Structure

```
paladin/
├── Cargo.toml                    # Workspace root
├── crates/
│   ├── paladin-core/             # Pure domain types, zero infrastructure deps
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── base/             # Node, Collection, Field primitives (~4k LOC)
│   │       └── platform/
│   │           └── container/    # Paladin, Battalion types, Garrison, Arsenal,
│   │                             # Citadel, Herald, Sanctum domain entities (~25k LOC)
│   │
│   ├── paladin-ports/            # Port trait definitions (architectural backbone)
│   │   ├── Cargo.toml            # Depends on: paladin-core
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── input/            # ContentIngestionPort, RpcGatewayPort, MlPort, NlpPort
│   │       └── output/           # LlmPort, GarrisonPort, SanctumPort, EmbeddingPort,
│   │                             # ArsenalPort, CitadelPort, QueuePort, NotificationPort,
│   │                             # LogPort, SearchPort, FileStoragePort, PaladinPort,
│   │                             # BattalionPort, PaladinRegistry (~6k LOC)
│   │
│   ├── paladin-battalion/        # Orchestration runtime (primary public value)
│   │   ├── Cargo.toml            # Depends on: paladin-core, paladin-ports
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── formation.rs      # Sequential execution service
│   │       ├── phalanx.rs        # Parallel execution service
│   │       ├── campaign.rs       # Graph/DAG workflow service
│   │       ├── chain_of_command.rs # Hierarchical delegation service
│   │       ├── conclave.rs       # Multi-expert synthesis service
│   │       ├── council.rs        # Discussion orchestration service
│   │       ├── grove.rs          # Intelligent routing service
│   │       ├── maneuver/         # Flow DSL (lexer, AST, parser, service)
│   │       └── commander.rs      # Strategy router (~13k LOC total)
│   │
│   ├── paladin-llm/              # LLM provider adapters (feature-flagged)
│   │   ├── Cargo.toml            # Depends on: paladin-core, paladin-ports
│   │   └── src/                  # Features: openai, anthropic, deepseek, mock
│   │       ├── lib.rs
│   │       ├── openai.rs         # OpenAI adapter + embedding adapter
│   │       ├── anthropic.rs      # Anthropic adapter
│   │       ├── deepseek.rs       # DeepSeek adapter
│   │       └── mock.rs           # Mock adapter for testing
│   │
│   ├── paladin-memory/           # Memory and vector storage adapters
│   │   ├── Cargo.toml            # Depends on: paladin-core, paladin-ports
│   │   └── src/                  # Features: sqlite, qdrant
│   │       ├── lib.rs
│   │       ├── garrison/         # InMemoryGarrison, SqliteGarrison, TokenCounter
│   │       └── sanctum/          # InMemorySanctum, QdrantSanctum
│   │
│   └── paladin-cli/              # CLI binary (already isolated in Milestone 1)
│       ├── Cargo.toml            # Depends on: paladin (facade)
│       └── src/
│           └── main.rs
│
├── src/                          # Facade crate (backward-compatible re-exports)
│   └── lib.rs                    # pub use paladin_core, paladin_ports, etc.
│
├── tests/                        # Workspace-level integration tests
├── examples/                     # Workspace-level examples
└── benches/                      # Workspace-level benchmarks
```

---

## Epic 1: Workspace Initialization and `paladin-core` Extraction

**Epic Owner:** TBD
**Priority:** Critical
**Estimated Effort:** Large
**Dependencies:** Milestone 1 complete

### Objective

Create the Cargo workspace structure and extract the first crate — `paladin-core` — containing all pure domain types from `src/core/base/` and `src/core/platform/container/`. This is the foundational extraction that every subsequent crate will depend on. It must have zero dependencies on the application or infrastructure layers and depend only on minimal external crates (`serde`, `uuid`, `chrono`, `thiserror`, `async-trait`).

### Background & Rationale

The `src/core/` module tree (~29k LOC) is already designed as a dependency-free domain layer following DDD principles. The `core/mod.rs` documentation explicitly states "zero dependencies on outer layers." However, one known coupling exists: `battalion/mod.rs` imports `PaladinResult` from `application::ports::output::paladin_port`, creating an upward dependency from the core domain to the application layer. This must be resolved before extraction.

The `core/base/` subtree (~4k LOC) contains foundation primitives (`Node<T>`, `Collection`, `Field`, `Message`) with essentially no external dependencies beyond `serde`, `uuid`, `chrono`, and `thiserror`. The `core/platform/container/` subtree (~25k LOC) contains all domain entities: `Paladin`, `Battalion` types (Formation, Phalanx, Campaign, ChainOfCommand, Conclave, Council, Grove, Maneuver), `Garrison`, `Arsenal`, `Citadel`, `Herald`, `Sanctum`, and supporting types.

### Acceptance Criteria

1. A `Cargo.toml` workspace root exists at the repository root with `members = ["crates/*"]` and a workspace-level `[workspace.dependencies]` section for shared dependency version management.
2. `crates/paladin-core/` exists as an independent crate with its own `Cargo.toml`.
3. `paladin-core` contains all types from `src/core/base/` and `src/core/platform/container/`.
4. `paladin-core` has zero `use` statements referencing `application::` or `infrastructure::` modules.
5. `paladin-core`'s `[dependencies]` section contains only: `serde` (with `derive`), `uuid` (with `v4`, `serde`), `chrono` (with `serde`), `thiserror`, `async-trait`, and `serde_json`.
6. The `BattalionResult` / `PaladinResult` upward dependency is resolved (either by defining the result type in core or by introducing a shared type in `paladin-ports`).
7. All existing unit tests that test core domain types pass when run from `paladin-core`.
8. `cargo build -p paladin-core` succeeds in isolation.
9. The main `paladin` crate compiles by depending on `paladin-core` and re-exporting its types.

### Tasks

#### Task 1.1: Create Workspace Root and Scaffold Crate Directories

**Description:** Initialize the Cargo workspace by converting the root `Cargo.toml` into a workspace manifest. Create the `crates/` directory and scaffold `paladin-core/` with a minimal `Cargo.toml` and `src/lib.rs`. Configure `[workspace.dependencies]` for shared dependency versions (`serde`, `uuid`, `chrono`, `thiserror`, `tokio`, `async-trait`, `serde_json`, `reqwest`, `log`).

**Deliverables:**
- Workspace root `Cargo.toml` with `[workspace]` section.
- `crates/paladin-core/Cargo.toml` referencing workspace dependencies.
- Skeleton `crates/paladin-core/src/lib.rs`.
- `cargo build` from workspace root succeeds (even if `paladin-core` is empty).

**Estimated Effort:** Small

#### Task 1.2: Resolve Core-to-Application Layer Dependency

**Description:** The `battalion/mod.rs` module imports `PaladinResult` from `application::ports::output::paladin_port` and `RegistryError` from `application::ports::output::paladin_registry`. These upward dependencies violate the hexagonal layering and prevent clean extraction of `paladin-core`.

Resolution options (evaluate and select):
- **Option A:** Move a minimal `PaladinResult` definition into `paladin-core` as a domain result type. The application layer version becomes a re-export or extension.
- **Option B:** Define a `BattalionOutcome` type in `paladin-core` that the battalion domain uses natively, and convert to/from `PaladinResult` at the application boundary.
- **Option C:** Introduce the result type in `paladin-ports` (extracted in Epic 2) and have both core and application depend on ports.

**Deliverables:**
- Analysis document evaluating the three options with trade-offs.
- Implementation of the selected approach.
- `battalion/mod.rs` no longer imports from `application::`.
- All battalion tests pass with the new type structure.

**Estimated Effort:** Medium

#### Task 1.3: Extract `core/base/` to `paladin-core`

**Description:** Move the `src/core/base/` module tree (~4k LOC) into `crates/paladin-core/src/base/`. This includes `Node<T>`, `Collection`, `Field`, `Message`, `Action`, `Event`, and related primitives. Update all `use` paths within the moved code. Verify zero external layer dependencies.

**Deliverables:**
- `crates/paladin-core/src/base/` containing all base module files.
- Updated `mod.rs` declarations and internal `use` paths.
- `cargo build -p paladin-core` succeeds.
- Unit tests for base types pass in isolation.

**Estimated Effort:** Medium

#### Task 1.4: Extract `core/platform/container/` to `paladin-core`

**Description:** Move the `src/core/platform/container/` module tree (~25k LOC) into `crates/paladin-core/src/platform/container/`. This is the bulk of the domain model: `Paladin`, `PaladinData`, `PaladinConfig`, all Battalion domain types (Formation, Phalanx, Campaign, ChainOfCommand, Conclave, Council, Grove, Maneuver with its lexer/AST/parser), `Garrison`, `GarrisonEntry`, `GarrisonConfig`, `Arsenal`, `Armament`, `Citadel`, `Herald`, `Sanctum`, `SanctumEntry`, `Memory`, `MemoryBuilder`, and all supporting types.

**Deliverables:**
- `crates/paladin-core/src/platform/` containing all platform container files.
- All internal cross-references updated to crate-local paths.
- No references to `application::` or `infrastructure::` modules remain.
- `cargo build -p paladin-core` succeeds.
- All domain entity unit tests pass.

**Estimated Effort:** Large

#### Task 1.5: Wire `paladin-core` into the Main Crate

**Description:** Update the root `paladin` crate to depend on `paladin-core` and re-export its types. Remove the now-duplicated source files from `src/core/`. Ensure all existing imports throughout the codebase resolve correctly through the re-exports.

**Deliverables:**
- `paladin` crate's `Cargo.toml` lists `paladin-core` as a dependency.
- `src/lib.rs` re-exports `paladin_core` types under the existing `core::` module path.
- `src/core/` directory removed (or reduced to a re-export shim).
- `cargo build --workspace` succeeds.
- `cargo test --workspace` passes all 1,487+ tests.

**Estimated Effort:** Medium

#### Task 1.6: Dependency Validation and Documentation

**Description:** Run `cargo-depgraph` or equivalent tooling to verify the dependency graph. Confirm `paladin-core` has no transitive dependencies on application or infrastructure crates. Update workspace documentation.

**Deliverables:**
- Dependency graph visualization showing clean layering.
- Updated `README.md` with workspace structure documentation.
- `CHANGELOG.md` entry for workspace introduction.

**Estimated Effort:** Small

---

## Epic 2: Extract `paladin-ports` Crate

**Epic Owner:** TBD
**Priority:** Critical
**Estimated Effort:** Medium
**Dependencies:** Epic 1 (paladin-core must exist)

### Objective

Extract the port trait definitions from `src/application/ports/` (~6k LOC, ~20 traits) into a dedicated `paladin-ports` crate. This crate defines the architectural contracts that all adapter crates implement. It depends only on `paladin-core` (for domain types referenced in trait signatures) and serves as the stable API boundary between the application layer and infrastructure.

### Background & Rationale

Port traits are the architectural backbone of the hexagonal design. They were hardened and documented as the stable public API in Milestone 1 (Epic 2). Extracting them into their own crate enforces the contract: infrastructure adapters depend on `paladin-ports` to implement traits, and application services depend on `paladin-ports` to declare their requirements. Neither can bypass the other.

The port traits reference domain types in their signatures (e.g., `GarrisonPort` references `GarrisonEntry`, `SanctumPort` references `SanctumEntry` and `Memory`). These types now live in `paladin-core`, so `paladin-ports` depends on `paladin-core`.

### Acceptance Criteria

1. `crates/paladin-ports/` exists with its own `Cargo.toml`.
2. `paladin-ports` depends only on `paladin-core`, `async-trait`, `serde`, `thiserror`, `uuid`, `chrono`, and `tokio` (for `mpsc` in streaming traits).
3. All ~20 port traits are defined in `paladin-ports` with their associated error types, request/response types, and configuration types.
4. `cargo build -p paladin-ports` succeeds in isolation.
5. The main `paladin` crate re-exports port traits at the existing `application::ports::` path.
6. All existing tests pass with updated import paths.

### Tasks

#### Task 2.1: Extract Output Port Traits

**Description:** Move all output port trait definitions to `crates/paladin-ports/src/output/`. This includes: `LlmPort` (with `LlmRequest`, `LlmResponse`, `LlmError`, `TokenUsage`, `FinishReason`, `StreamingResponse`), `GarrisonPort` (with `GarrisonError`, `GarrisonStats`), `SanctumPort` (with `SanctumError`, `SanctumQuery`, `SanctumFilter`, `SanctumSearchResult`), `EmbeddingPort` (with `Embedding`, `EmbeddingError`), `ArsenalPort`/`ArsenalRegistry`, `CitadelPort`, `QueuePort`, `NotificationPort`, `LogPort`, `SearchPort`, `FileStoragePort`, `PaladinPort` (with `PaladinResult`, `StopReason`), `BattalionPort`, and `PaladinRegistry` (with `RegistryError`).

**Deliverables:**
- `crates/paladin-ports/src/output/` containing all output port modules.
- All associated types (errors, requests, responses, configs) co-located with their port trait.
- `cargo build -p paladin-ports` succeeds.

**Estimated Effort:** Medium

#### Task 2.2: Extract Input Port Traits

**Description:** Move input port trait definitions to `crates/paladin-ports/src/input/`. This includes `ContentIngestionPort`, `RpcGatewayPort`, `MlPort`, and `NlpPort`.

**Deliverables:**
- `crates/paladin-ports/src/input/` containing all input port modules.
- `cargo build -p paladin-ports` succeeds.

**Estimated Effort:** Small

#### Task 2.3: Wire `paladin-ports` into Dependent Crates

**Description:** Update the main `paladin` crate and any modules that reference port traits to depend on `paladin-ports`. Remove duplicated source files from `src/application/ports/`. Ensure re-exports preserve backward compatibility.

**Deliverables:**
- `paladin` crate depends on `paladin-ports`.
- `src/application/ports/` removed or reduced to re-export shim.
- All adapter implementations updated to import from `paladin_ports::`.
- `cargo test --workspace` passes.

**Estimated Effort:** Medium

---

## Epic 3: Extract `paladin-battalion` Crate

**Epic Owner:** TBD
**Priority:** High
**Estimated Effort:** Large
**Dependencies:** Epic 1 (paladin-core), Epic 2 (paladin-ports)

### Objective

Extract the complete orchestration runtime — all eight Battalion patterns plus the Commander strategy router — into a dedicated `paladin-battalion` crate. This is the primary public value of the framework. After extraction, a downstream consumer who only needs multi-agent orchestration can depend on `paladin-core` + `paladin-ports` + `paladin-battalion` without pulling in any LLM provider implementation, storage backend, web server, or notification dependency.

### Background & Rationale

The Battalion subsystem consists of approximately 13k LOC of execution services across the application use-cases layer (`src/application/use_cases/battalion/`) plus ~9k LOC of domain types already extracted to `paladin-core` in Epic 1. The execution services include `FormationExecutionService`, `PhalanxExecutionService`, `CampaignExecutionService`, `ChainOfCommandExecutionService`, `ConclaveExecutionService`, `CouncilExecutionService`, `GroveExecutionService`, `ManeuverExecutionService`, and the `Commander` strategy router.

These services depend only on port traits (from `paladin-ports`) and domain types (from `paladin-core`). They operate through `Arc<dyn PaladinPort>`, `Arc<dyn PaladinRegistry>`, and similar trait objects — they never reference concrete adapter implementations. This clean dependency inversion makes extraction straightforward.

The Maneuver DSL (lexer, AST, parser) is currently split between `core/platform/container/battalion/parser/` (already in `paladin-core` after Epic 1) and `application/use_cases/battalion/maneuver_service.rs`. The execution service moves to `paladin-battalion`; the parser primitives remain in `paladin-core`.

### Acceptance Criteria

1. `crates/paladin-battalion/` exists with its own `Cargo.toml`.
2. `paladin-battalion` depends only on `paladin-core` and `paladin-ports` (plus `tokio`, `log`, `uuid`, `async-trait`, `serde`).
3. `paladin-battalion` does NOT depend on any concrete adapter, LLM provider, storage backend, or infrastructure crate.
4. All eight orchestration patterns are functional and tested within the crate.
5. The `Commander` strategy router and auto-detection heuristics work correctly.
6. `cargo build -p paladin-battalion` succeeds in isolation.
7. All battalion-related tests (unit + integration where applicable) pass.
8. The `paladin` facade crate re-exports battalion types at the existing paths.

### Tasks

#### Task 3.1: Extract Execution Services

**Description:** Move all battalion execution service modules from `src/application/use_cases/battalion/` to `crates/paladin-battalion/src/`. This includes: `formation_service.rs`, `phalanx_service.rs`, `campaign_service.rs`, `chain_of_command_service.rs`, `conclave_execution_service.rs`, `council_service.rs`, `grove_service.rs`, `maneuver_service.rs`, and `commander.rs`.

**Deliverables:**
- All execution service files relocated to `paladin-battalion`.
- Internal `use` paths updated to reference `paladin_core::` and `paladin_ports::`.
- `cargo build -p paladin-battalion` succeeds.

**Estimated Effort:** Large

#### Task 3.2: Extract Flow Visualizer and Battalion Utilities

**Description:** Move the battalion flow visualizer, configuration utilities, and any shared battalion helpers that support the execution services.

**Deliverables:**
- Visualizer and utility modules relocated.
- No remaining battalion execution code in the main `paladin` crate's `use_cases/`.
- All battalion-related functionality accessible through `paladin-battalion`.

**Estimated Effort:** Medium

#### Task 3.3: Verify Dependency Isolation

**Description:** Run dependency analysis to confirm `paladin-battalion` has no transitive dependency on infrastructure crates, LLM providers, or storage backends. Verify that `cargo build -p paladin-battalion` does not download or compile `reqwest`, `actix-web`, `sqlx`, `redis`, `qdrant-client`, `lettre`, or any other infrastructure dependency.

**Deliverables:**
- `cargo tree -p paladin-battalion` output showing clean dependency tree.
- Dependency graph visualization confirming isolation.
- Any leaking dependencies identified and resolved.

**Estimated Effort:** Small

#### Task 3.4: Wire `paladin-battalion` into the Facade Crate

**Description:** Update the root `paladin` crate to depend on `paladin-battalion` and re-export its types. Remove duplicated battalion execution services from the main crate source. Verify all examples and integration tests compile.

**Deliverables:**
- `paladin` crate depends on `paladin-battalion`.
- `src/application/use_cases/battalion/` removed or reduced to re-export shim.
- All existing examples compile and run correctly.
- `cargo test --workspace` passes.

**Estimated Effort:** Medium

#### Task 3.5: Battalion Crate Documentation and Examples

**Description:** Write crate-level documentation for `paladin-battalion` including usage examples showing how to use the crate independently with custom `PaladinPort` implementations. Verify `cargo doc -p paladin-battalion` produces clean output.

**Deliverables:**
- Crate-level `//!` documentation in `lib.rs`.
- At least one standalone example per orchestration pattern.
- `cargo doc -p paladin-battalion --no-deps` clean with no warnings.

**Estimated Effort:** Medium

---

## Epic 4: Extract `paladin-llm` Crate

**Epic Owner:** TBD
**Priority:** High
**Estimated Effort:** Medium
**Dependencies:** Epic 2 (paladin-ports must define `LlmPort`)

### Objective

Extract all LLM provider adapters into a single `paladin-llm` crate with per-provider feature flags (`openai`, `anthropic`, `deepseek`, `mock`). This consolidates all `LlmPort` implementations in one coherent location, allowing provider maintainers to work independently and downstream users to pay only for the providers they use.

### Background & Rationale

The current LLM adapters in `src/infrastructure/adapters/llm/` include `openai_adapter.rs`, `anthropic_adapter.rs`, `deepseek_adapter.rs`, `openai_embedding_adapter.rs`, and mock adapters. They share only the `LlmPort` trait (now in `paladin-ports`) and each brings its own HTTP client configuration and provider-specific logic.

A single `paladin-llm` crate with feature flags is preferred over one crate per provider because it avoids over-fragmentation, keeps the `LlmPort` trait implementations discoverable in one place, and simplifies version coordination across providers. The Milestone 1 feature flags (`llm-openai`, `llm-anthropic`, `llm-deepseek`) translate directly to this crate's internal feature flags.

### Acceptance Criteria

1. `crates/paladin-llm/` exists with its own `Cargo.toml`.
2. `paladin-llm` depends on `paladin-core` and `paladin-ports`.
3. Each provider is gated behind a feature flag: `openai`, `anthropic`, `deepseek`.
4. A `mock` feature provides `MockLlmPort` and `MultiStepMockLlmPort` for testing.
5. `cargo build -p paladin-llm --no-default-features` compiles (producing an empty crate with no provider).
6. `cargo build -p paladin-llm --features openai` compiles only the OpenAI adapter.
7. The OpenAI embedding adapter is included under the `openai` feature alongside the chat adapter.
8. All LLM adapter tests pass when their respective features are enabled.
9. The `paladin` facade crate re-exports provider adapters at existing paths.

### Tasks

#### Task 4.1: Scaffold `paladin-llm` Crate with Feature Flags

**Description:** Create `crates/paladin-llm/` with a `Cargo.toml` that defines provider feature flags and optional dependencies. Map `reqwest` and provider-specific dependencies to their respective flags.

**Deliverables:**
- `crates/paladin-llm/Cargo.toml` with `openai`, `anthropic`, `deepseek`, `mock` feature flags.
- `crates/paladin-llm/src/lib.rs` with `#[cfg(feature = "...")]` module declarations.
- `cargo build -p paladin-llm --no-default-features` succeeds.

**Estimated Effort:** Small

#### Task 4.2: Extract OpenAI Adapter

**Description:** Move `openai_adapter.rs` and `openai_embedding_adapter.rs` from `src/infrastructure/adapters/llm/` to `crates/paladin-llm/src/openai/`. Update imports to reference `paladin_core` and `paladin_ports`.

**Deliverables:**
- `crates/paladin-llm/src/openai/` containing both adapters.
- `cargo build -p paladin-llm --features openai` succeeds.
- OpenAI adapter unit and integration tests pass.

**Estimated Effort:** Small

#### Task 4.3: Extract Anthropic and DeepSeek Adapters

**Description:** Move `anthropic_adapter.rs` and `deepseek_adapter.rs` to their respective modules in `paladin-llm`.

**Deliverables:**
- `crates/paladin-llm/src/anthropic.rs` and `crates/paladin-llm/src/deepseek.rs`.
- Each compiles independently behind its feature flag.
- Provider-specific tests pass.

**Estimated Effort:** Small

#### Task 4.4: Extract Mock Adapters and Wire into Facade

**Description:** Move mock LLM adapters (`MockLlmPort`, `MultiStepMockLlmPort`) behind a `mock` feature flag. Update the `paladin` facade crate to depend on `paladin-llm` and re-export adapters. Remove duplicated adapter source from the main crate.

**Deliverables:**
- Mock adapters available under `paladin-llm`'s `mock` feature.
- `paladin` facade depends on `paladin-llm` with appropriate default features.
- All existing tests that use mock adapters compile and pass.
- `cargo test --workspace` passes.

**Estimated Effort:** Medium

---

## Epic 5: Extract `paladin-memory` Crate

**Epic Owner:** TBD
**Priority:** High
**Estimated Effort:** Medium
**Dependencies:** Epic 2 (paladin-ports must define `GarrisonPort` and `SanctumPort`)

### Objective

Extract all memory and vector storage adapter implementations into a `paladin-memory` crate with feature-flagged backends. This consolidates `GarrisonPort` implementations (in-memory, SQLite) and `SanctumPort` implementations (in-memory, Qdrant) alongside the token counting utilities and RAG retrieval service.

### Background & Rationale

The memory subsystem spans multiple directories:
- `src/infrastructure/adapters/garrison/` — `InMemoryGarrison`, `SqliteGarrison`, `TokenCounter` implementations.
- `src/infrastructure/adapters/sanctum/` — `InMemorySanctum`, `QdrantSanctumAdapter`.
- `src/application/use_cases/sanctum/` — `MemoryExtractionService`, `RagRetrievalService`.

These components share the `GarrisonPort` and `SanctumPort` traits (now in `paladin-ports`) and the domain types `GarrisonEntry`, `SanctumEntry`, `Memory`, `MemoryBuilder` (now in `paladin-core`). The SQLite garrison depends on `sqlx`, and the Qdrant sanctum depends on `qdrant-client` — both are heavy dependencies that should be optional.

### Acceptance Criteria

1. `crates/paladin-memory/` exists with its own `Cargo.toml`.
2. `paladin-memory` depends on `paladin-core` and `paladin-ports`.
3. Feature flags: `sqlite` (gates `sqlx`), `qdrant` (gates `qdrant-client`). In-memory implementations are always available.
4. `MemoryExtractionService` and `RagRetrievalService` are included (they depend on port traits, not concrete adapters).
5. `TokenCounter` and `TiktokenCounter` are included (with `tiktoken` behind the existing `content-processing` or a new `tokenizer` feature).
6. `cargo build -p paladin-memory --no-default-features` compiles with only in-memory implementations.
7. All memory-related tests pass when appropriate features are enabled.
8. The `paladin` facade crate re-exports memory types at existing paths.

### Tasks

#### Task 5.1: Scaffold `paladin-memory` Crate

**Description:** Create the crate structure with feature flags for storage backends.

**Deliverables:**
- `crates/paladin-memory/Cargo.toml` with `sqlite`, `qdrant` feature flags.
- Module structure: `garrison/`, `sanctum/`, `services/`.
- `cargo build -p paladin-memory --no-default-features` succeeds.

**Estimated Effort:** Small

#### Task 5.2: Extract Garrison Adapters

**Description:** Move `InMemoryGarrison`, `SqliteGarrison`, `TokenCounter`, and `TiktokenCounter` to `crates/paladin-memory/src/garrison/`. Gate `SqliteGarrison` behind the `sqlite` feature.

**Deliverables:**
- Garrison adapter modules relocated.
- `SqliteGarrison` compiles only with `--features sqlite`.
- `InMemoryGarrison` compiles unconditionally.
- Garrison unit tests pass.

**Estimated Effort:** Medium

#### Task 5.3: Extract Sanctum Adapters

**Description:** Move `InMemorySanctum` and `QdrantSanctumAdapter` to `crates/paladin-memory/src/sanctum/`. Gate `QdrantSanctumAdapter` behind the `qdrant` feature.

**Deliverables:**
- Sanctum adapter modules relocated.
- `QdrantSanctumAdapter` compiles only with `--features qdrant`.
- `InMemorySanctum` compiles unconditionally.
- Sanctum unit tests pass.

**Estimated Effort:** Medium

#### Task 5.4: Extract Memory Services and Wire into Facade

**Description:** Move `MemoryExtractionService` and `RagRetrievalService` to `crates/paladin-memory/src/services/`. Update the `paladin` facade crate to depend on `paladin-memory` and re-export. Remove duplicated source.

**Deliverables:**
- Memory services relocated.
- `paladin` facade depends on `paladin-memory`.
- All memory and RAG integration tests pass.
- `cargo test --workspace` passes.

**Estimated Effort:** Medium

---

## Epic 6: Facade Crate, CI Pipeline, and Workspace Finalization

**Epic Owner:** TBD
**Priority:** Critical
**Estimated Effort:** Medium
**Dependencies:** Epics 1–5

### Objective

Finalize the `paladin` root facade crate as the backward-compatible entry point, configure workspace-level CI, produce a comprehensive migration guide, and validate the complete workspace against all quality gates.

### Background & Rationale

After Epics 1–5, the source code is distributed across six crates. The facade crate must provide a seamless migration path for existing consumers: the same `use paladin::...` import paths should work without modification. Additionally, CI must test the workspace holistically (all crates together) and individually (each crate in isolation) to catch dependency leaks.

### Acceptance Criteria

1. `use paladin::core::*`, `use paladin::application::ports::*`, and all existing import paths continue to work via facade re-exports.
2. A `paladin::prelude` module provides the most commonly used types for convenient import.
3. CI tests: workspace build, per-crate isolated build, per-crate tests, feature-flag matrix, documentation build.
4. `cargo clippy --workspace -- -D warnings` passes.
5. `cargo fmt --all -- --check` passes.
6. `cargo doc --workspace --no-deps` produces clean documentation.
7. Build time benchmarks show measurable improvement over monolithic baseline.
8. Migration guide documents all changes with before/after examples.
9. `CHANGELOG.md` updated with comprehensive workspace refactor entry.

### Tasks

#### Task 6.1: Finalize Facade Crate Re-Exports

**Description:** Audit all existing `use paladin::...` statements across examples, tests, integration tests, and documented code. Ensure the facade crate re-exports cover every path. Add a `prelude` module with the most commonly used types.

**Deliverables:**
- Complete re-export mapping in `src/lib.rs`.
- `paladin::prelude` module with curated types.
- All examples compile without import path changes.

**Estimated Effort:** Medium

#### Task 6.2: Configure Workspace CI Pipeline

**Description:** Update the CI configuration to:
- Build the full workspace (`cargo build --workspace`).
- Build each crate in isolation (`cargo build -p paladin-core`, etc.).
- Test the full workspace (`cargo test --workspace`).
- Test each crate in isolation.
- Run the feature-flag matrix from Milestone 1 at the workspace level.
- Run `cargo clippy --workspace -- -D warnings` and `cargo fmt --all -- --check`.
- Build documentation (`cargo doc --workspace --no-deps`).

**Deliverables:**
- Updated CI configuration files.
- All pipeline stages green.
- Per-crate isolated build verification passing.

**Estimated Effort:** Medium

#### Task 6.3: Benchmark Build Times

**Description:** Measure and compare build times across the key scenarios:
- Clean workspace build vs. monolithic clean build.
- Incremental rebuild after changing a single file in `paladin-core`.
- Incremental rebuild after changing a single LLM adapter.
- Incremental rebuild after changing a battalion execution service.
- `paladin-core` + `paladin-ports` + `paladin-battalion` only (no infrastructure).

**Deliverables:**
- Build time comparison report with before/after measurements.
- Identification of any regressions (e.g., linker overhead on cold builds).
- Recommendations for further optimization if needed.

**Estimated Effort:** Small

#### Task 6.4: Migration Guide and Documentation

**Description:** Produce a comprehensive migration guide covering the workspace transition. Update all project documentation to reflect the new structure.

**Deliverables:**
- `docs/MIGRATION_WORKSPACE.md` with step-by-step upgrade instructions.
- Updated `README.md` with workspace structure, crate descriptions, and dependency diagram.
- Updated `CONTRIBUTING.md` with workspace development workflow (how to add code to the right crate, how to run per-crate tests).
- Updated `STABLE_API.md` with crate-level API surface.
- `CHANGELOG.md` entry for the workspace refactor.

**Estimated Effort:** Medium

#### Task 6.5: Workspace Retrospective and Tier 3 Preparation

**Description:** Conduct a team retrospective on the workspace extraction. Document lessons learned, identify any remaining coupling concerns, and prepare a handoff assessment for Milestone 3 (Tier 3 architectural refinements).

**Deliverables:**
- Retrospective notes document.
- Known issues or technical debt list carried forward to Tier 3.
- Tier 3 readiness assessment.

**Estimated Effort:** Small

---

## Milestone Schedule Overview

| Phase | Epic | Estimated Duration | Predecessors |
|-------|------|--------------------|--------------|
| Phase 1 | Epic 1: Workspace Init + `paladin-core` Extraction | 2–3 sprints | Milestone 1 complete |
| Phase 2 | Epic 2: `paladin-ports` Extraction | 1–2 sprints | Epic 1 |
| Phase 3 | Epic 3: `paladin-battalion` Extraction | 2–3 sprints | Epics 1, 2 |
| Phase 3 | Epic 4: `paladin-llm` Extraction (parallel with Epic 3) | 1–2 sprints | Epic 2 |
| Phase 3 | Epic 5: `paladin-memory` Extraction (parallel with Epics 3, 4) | 1–2 sprints | Epic 2 |
| Phase 4 | Epic 6: Facade, CI, Finalization | 1–2 sprints | Epics 1–5 |

**Total Estimated Duration:** 5–8 sprints (Epics 3, 4, 5 can be parallelized across team members)

---

## Completion Checklist

- [ ] Workspace root `Cargo.toml` configured with `[workspace]` and `[workspace.dependencies]`.
- [ ] `paladin-core` extracted with zero application/infrastructure dependencies.
- [ ] Core-to-application layer dependency (`PaladinResult` import in `battalion/mod.rs`) resolved.
- [ ] `paladin-ports` extracted with all ~20 port traits.
- [ ] `paladin-battalion` extracted with all 8 orchestration patterns + Commander.
- [ ] `paladin-battalion` dependency tree contains no infrastructure crates.
- [ ] `paladin-llm` extracted with per-provider feature flags.
- [ ] `paladin-memory` extracted with `sqlite` and `qdrant` feature flags.
- [ ] Facade crate provides backward-compatible re-exports for all existing import paths.
- [ ] `paladin::prelude` module available.
- [ ] `cargo build --workspace` succeeds.
- [ ] `cargo test --workspace` passes all 1,487+ tests.
- [ ] Each crate builds in isolation (`cargo build -p <crate>`).
- [ ] `cargo clippy --workspace -- -D warnings` clean.
- [ ] `cargo fmt --all -- --check` clean.
- [ ] `cargo doc --workspace --no-deps` clean.
- [ ] CI pipeline updated with workspace-level and per-crate testing.
- [ ] Build time benchmarks documented showing improvement.
- [ ] Migration guide published (`docs/MIGRATION_WORKSPACE.md`).
- [ ] `README.md`, `CONTRIBUTING.md`, `STABLE_API.md` updated.
- [ ] `CHANGELOG.md` updated.
- [ ] Retrospective completed and Tier 3 readiness assessed.

---

## Appendix A: Dependency Direction Diagram

```
                    ┌──────────────────┐
                    │   paladin-core   │  Zero external layer deps
                    │  (domain types)  │  serde, uuid, chrono, thiserror
                    └────────┬─────────┘
                             │
                    ┌────────▼─────────┐
                    │  paladin-ports   │  Port trait definitions
                    │  (API contracts) │  Depends on: paladin-core
                    └────────┬─────────┘
                             │
              ┌──────────────┼──────────────┐
              │              │              │
     ┌────────▼───────┐ ┌───▼────────┐ ┌──▼──────────────┐
     │paladin-battalion│ │paladin-llm │ │ paladin-memory  │
     │ (orchestration) │ │(providers) │ │(garrison+sanctum│
     └────────┬───────┘ └───┬────────┘ └──┬──────────────┘
              │              │              │
              └──────────────┼──────────────┘
                             │
                    ┌────────▼─────────┐
                    │     paladin      │  Facade crate
                    │  (re-exports +   │  Backward-compatible
                    │   remaining app) │  entry point
                    └────────┬─────────┘
                             │
                    ┌────────▼─────────┐
                    │   paladin-cli    │  Binary only
                    │   (CLI tooling)  │  Not compiled as library
                    └──────────────────┘
```

## Appendix B: Known Upward Dependencies to Resolve

| Source Location | Offending Import | Resolution |
|----------------|-----------------|------------|
| `core::platform::container::battalion::mod` | `application::ports::output::paladin_port::PaladinResult` | Move result type to `paladin-core` or `paladin-ports` |
| `core::platform::container::battalion::mod` | `application::ports::output::paladin_registry::RegistryError` | Move error type to `paladin-ports` |

## Appendix C: Crate Dependency Matrix

| Crate | paladin-core | paladin-ports | paladin-battalion | paladin-llm | paladin-memory |
|-------|:---:|:---:|:---:|:---:|:---:|
| **paladin-core** | — | | | | |
| **paladin-ports** | ✓ | — | | | |
| **paladin-battalion** | ✓ | ✓ | — | | |
| **paladin-llm** | ✓ | ✓ | | — | |
| **paladin-memory** | ✓ | ✓ | | | — |
| **paladin (facade)** | ✓ | ✓ | ✓ | ✓ | ✓ |
| **paladin-cli** | | | | | |

*Note: `paladin-cli` depends on the `paladin` facade crate, not individual workspace crates.*

## Appendix D: Workspace `Cargo.toml` Template

```toml
[workspace]
members = [
    "crates/paladin-core",
    "crates/paladin-ports",
    "crates/paladin-battalion",
    "crates/paladin-llm",
    "crates/paladin-memory",
    "crates/paladin-cli",
]
resolver = "2"

[workspace.package]
version = "0.2.0"
edition = "2024"
authors = ["Am0rfu5"]
license = "MIT"
repository = "https://github.com/DF3NDR/paladin-dev-env"

[workspace.dependencies]
# Shared dependency versions — all crates reference these
serde = { version = "1", features = ["derive"] }
serde_json = "1.0"
uuid = { version = "1.8", features = ["v4", "serde"] }
chrono = { version = "0.4", features = ["serde"] }
thiserror = "2"
async-trait = "0.1"
tokio = { version = "1", features = ["full"] }
log = "0.4"
reqwest = { version = "0.12", features = ["json"] }
futures = "0.3"

# Cross-crate workspace references
paladin-core = { path = "crates/paladin-core" }
paladin-ports = { path = "crates/paladin-ports" }
paladin-battalion = { path = "crates/paladin-battalion" }
paladin-llm = { path = "crates/paladin-llm" }
paladin-memory = { path = "crates/paladin-memory" }
```
