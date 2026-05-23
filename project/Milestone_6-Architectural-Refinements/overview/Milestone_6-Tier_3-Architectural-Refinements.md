# Milestone 6: Architectural Refinements

**Project:** Paladin Framework Refactoring Initiative
**Milestone:** 6 (Refactoring Tier 3 — Architectural Refinements Within the Existing Structure
**Status:** Planning
**Target Start:** Upon completion of Milestone 5 (Refactoring Tier 2)
**Target Completion:** TBD
**Document Version:** 1.0
**Last Updated:** 2026-04-14

---

## Executive Summary

Milestone 6 addresses the internal architectural inconsistencies identified in the refactoring analysis that were deliberately deferred while the high-value structural changes (feature flags, workspace decomposition) took priority. These are refinements that correct genuine architectural violations in the hexagonal layering, improve long-term maintainability, and reduce cognitive load for developers navigating the codebase.

With the workspace decomposition complete (Milestone 5), each refinement now occurs within a well-scoped crate with enforced dependency boundaries. This makes the changes safer — a misplaced type or a layering violation is caught at compile time by the crate dependency graph rather than relying on module-level discipline alone.

The four Epics in this Milestone are:

1. **Split `application_settings.rs`** — Break the 3,172-line monolithic configuration file into per-domain configuration modules.
2. **Relocate Manager-Layer Services** — Move orchestration services that depend on ports out of the `core/platform/manager` layer and into the application use-cases layer where they belong in strict hexagonal architecture.
3. **Co-locate the Maneuver DSL** — Consolidate the tightly coupled lexer, AST, parser, and execution service so they travel together rather than being split across layers.
4. **Relocate `CircuitBreaker`** — Move this infrastructure concern out of the application use-cases layer into the infrastructure layer alongside retry logic and rate limiting.

### Prerequisites (Completed in Milestones 1 and 2)

- Feature flags expanded and CI matrix testing in place (Milestone 1).
- Port traits hardened as stable API contracts (Milestone 1).
- CLI isolated from library compilation (Milestone 1).
- Cargo workspace decomposed into `paladin-core`, `paladin-ports`, `paladin-battalion`, `paladin-llm`, `paladin-memory`, and the `paladin` facade crate (Milestone 5).
- Backward-compatible facade re-exports verified (Milestone 5).

### Success Criteria

- All existing tests (1,487+) continue to pass after each Epic is completed.
- `cargo clippy --workspace -- -D warnings` remains clean.
- `cargo fmt --all -- --check` passes.
- `cargo doc --workspace --no-deps` produces clean documentation with no broken links.
- No new public API surface is exposed beyond what was documented in `STABLE_API.md`.
- The `core` layer (within `paladin-core`) contains zero imports from `application::` or `infrastructure::` modules — verified by crate boundary enforcement.
- `application_settings.rs` is replaced by per-domain config modules totaling the same functionality with improved navigability.
- Developer onboarding time for understanding configuration structure is measurably reduced (qualitative assessment).

---

## Milestone Scope & Boundaries

### In Scope

- Decomposition of `application_settings.rs` into per-domain configuration modules.
- Relocation of manager-layer orchestration services to the application use-cases layer.
- Co-location of the Maneuver DSL components (lexer, AST, parser, execution service).
- Relocation of `CircuitBreaker` from application use-cases to infrastructure.
- Documentation updates for all relocated components.
- Test migration and verification for all moved code.

### Out of Scope

- Further workspace crate extractions beyond those completed in Milestone 5.
- New feature development or new orchestration patterns.
- Performance optimization of the relocated components.
- Refactoring the notification service domain model (it was rebuilt as a prior Epic and is architecturally sound).
- Breaking changes to the public API surface defined in `STABLE_API.md`.

### Dependencies & Assumptions

- Milestone 5 is fully completed: workspace crates exist and CI passes at the workspace level.
- The facade crate re-export paths are stable and will absorb internal relocations transparently.
- No concurrent feature development is modifying the same files targeted by these Epics.

### Risk Register

| Risk | Likelihood | Impact | Mitigation |
|------|-----------|--------|------------|
| Settings decomposition introduces runtime config loading regressions | Medium | High | Maintain identical deserialization behavior; property-level integration tests comparing old vs. new config loading |
| Manager service relocation creates circular dependencies between crates | Medium | High | Dependency analysis before each move; the workspace crate boundaries enforce direction at compile time |
| Maneuver DSL co-location decision conflicts with Milestone 5 crate boundaries | Low | Medium | The parser is already in `paladin-core` (Epic 1.4 of Milestone 5); the service is in `paladin-battalion`; co-location means moving parser to battalion, which is the correct direction |
| CircuitBreaker relocation breaks `PaladinExecutionService` import paths | Low | Low | Facade crate re-exports absorb the change; update internal imports in `paladin-battalion` |
| Team velocity reduced by Milestone 5 fatigue | Medium | Medium | Tier 3 Epics are independent and can be tackled incrementally with lower urgency |

---

## Epic 1: Decompose `application_settings.rs` into Per-Domain Configuration Modules

**Epic Owner:** TBD
**Priority:** High
**Estimated Effort:** Medium
**Dependencies:** None (can begin immediately after Milestone 5)

### Objective

Replace the monolithic 3,172-line `src/config/application_settings.rs` file with a set of focused, per-domain configuration modules. Each subsystem — agent execution, garrison memory, arsenal tools, notifications, queue infrastructure, file storage, web server, LLM providers, and battalion orchestration — gets its own configuration struct in its own file. A root `Settings` struct composes them, preserving the existing deserialization contract with `config.yml` and environment variable overrides.

### Background & Rationale

A single file carrying all configuration types for every subsystem is a well-known Rust anti-pattern that compounds over time. The current `application_settings.rs` contains:

- `Settings` (root aggregate with 15+ optional fields)
- `QueueConfig` (Redis queue configuration with 10+ fields and env var override logic)
- `FileStorageConfig` (MinIO/S3 configuration with env var overrides)
- `NotificationConfig` (notification channel and template settings)
- `MCPServerConfig` (MCP Arsenal server definitions)
- `LlmProviderConfig` / `OpenAIConfig` / `AnthropicConfig` / `DeepSeekConfig`
- `GarrisonConfig` references and defaults
- `WebServerConfig`
- `LoggingConfig`
- Various helper methods for environment variable override logic

Every developer who needs to understand or modify any single subsystem's configuration must navigate a 3,172-line file. Configuration changes for unrelated subsystems create merge conflicts. The env var override methods are duplicated per-config-struct with identical patterns.

### Acceptance Criteria

1. `application_settings.rs` is replaced by a `config/` module directory with individual files:
   - `config/mod.rs` — Re-exports and the root `Settings` struct.
   - `config/agent.rs` — Paladin agent execution configuration.
   - `config/garrison.rs` — Garrison memory system configuration.
   - `config/arsenal.rs` — Arsenal and MCP server configuration.
   - `config/notifications.rs` — Notification channel and template configuration.
   - `config/queue.rs` — Redis queue configuration.
   - `config/file_storage.rs` — MinIO/S3 file storage configuration.
   - `config/web_server.rs` — Web server and API configuration.
   - `config/llm.rs` — LLM provider configurations (OpenAI, Anthropic, DeepSeek).
   - `config/battalion.rs` — Battalion orchestration defaults.
   - `config/logging.rs` — Logging and tracing configuration.
2. The root `Settings` struct remains the single entry point for config loading, composing all sub-configs via `#[serde(default)]` fields.
3. `Settings::new()`, `Settings::load_from_file()`, and all environment variable override methods produce identical results to the pre-refactor implementation.
4. A shared `env_override` utility function or trait eliminates the duplicated env var override pattern across config structs.
5. All existing config-related tests pass without behavioral changes.
6. `config.yml` deserialization is fully backward-compatible — no changes to the YAML schema.
7. No file in the `config/` module exceeds 400 lines.

### Tasks

#### Task 1.1: Audit and Map Configuration Domains

**Description:** Analyze `application_settings.rs` and produce a detailed map of every struct, field, method, and env var override. Classify each into its target domain module. Identify shared patterns (env var override logic) that should become utilities.

**Deliverables:**
- Configuration domain mapping document (struct → target file).
- Inventory of duplicated patterns to extract into shared utilities.
- Dependency graph showing which config structs reference others.

**Estimated Effort:** Small

#### Task 1.2: Create Shared Environment Override Utility

**Description:** Extract the repeated pattern of "read env var, parse, override field" into a reusable utility. The current code repeats this pattern approximately 30 times across `get_queue_config()`, `get_file_storage_config()`, and similar methods.

**Deliverables:**
- `config/env_utils.rs` (or similar) with generic env var override helpers.
- Helper functions or a trait for typed env var reading with fallback: `env_override::<T>(var_name, &mut field)`.
- Unit tests for the utility with various types (String, u16, u64, bool, Option).

**Estimated Effort:** Small

#### Task 1.3: Extract Per-Domain Configuration Modules

**Description:** Move each configuration struct and its associated methods into its designated file. Update the root `Settings` struct to compose sub-configs. Ensure `#[serde(default)]` and `#[serde(flatten)]` attributes preserve YAML compatibility.

**Deliverables:**
- Individual config files as listed in the acceptance criteria.
- Root `Settings` struct in `config/mod.rs` composing all sub-configs.
- All `use` paths updated throughout the codebase.
- `config.yml` deserialization verified identical via snapshot testing.

**Estimated Effort:** Medium

#### Task 1.4: Migrate Config Tests and Verify Backward Compatibility

**Description:** Migrate all existing configuration tests to the new module structure. Add regression tests that load a known `config.yml` and assert field-by-field equality against expected values. Verify all environment variable overrides work correctly.

**Deliverables:**
- Migrated config tests passing in new locations.
- New regression test loading a reference `config.yml` and asserting all fields.
- Env var override integration tests for each config domain.
- No behavioral changes detected.

**Estimated Effort:** Small

---

## Epic 2: Relocate Manager-Layer Orchestration Services to the Application Layer

**Epic Owner:** TBD
**Priority:** High
**Estimated Effort:** Large
**Dependencies:** Epic 1 is recommended first (config changes reduce file sizes) but not strictly required

### Objective

Move orchestration services that depend on port traits out of `core/platform/manager/` and into the application use-cases layer. In strict hexagonal architecture, the core layer should contain only entities, value objects, and pure domain logic with zero dependencies on ports or external interfaces. Services that coordinate between ports, dispatch to adapters, or manage infrastructure concerns belong in the application layer.

### Background & Rationale

The analysis identified that the `core/platform/manager/` directory contains services with orchestration logic that properly belongs in the application use-cases layer. The affected modules are:

- **`notification_service.rs`** — A platform-level orchestrator that integrates with `MessageService`, manages notification channels, coordinates template rendering, and handles delivery via port-backed adapters. It imports from `core::base::service::message_service` and exposes types consumed by infrastructure adapters. While its domain model (`Notification`, `NotificationChannel`, `NotificationContent`) correctly lives in `core`, the service itself is an application-layer coordinator.

- **`queue_service.rs`** — Manages queue operations including job dispatch, priority handling, and retry logic. Depends on queue configuration types and coordinates between the `Orchestrator` and external queue adapters. The `QueueConfig`, `QueueItem`, and `QueueStats` types are domain types (they belong in core), but the service that dispatches to adapters is application-layer logic.

- **`orchestrator.rs`** — The general-purpose orchestration engine coordinating workflows, jobs, tasks, triggers, listeners, schedulers, and queue processors. It imports from `core::base::component::action`, `core::base::entity::message`, multiple container types, and three other manager services (`listener_service`, `queue_service`, `scheduler`). It defines `OrchestrationContext` and extensive orchestration logic. This is the most complex relocation.

- **`log_service.rs`** — Manages structured logging with destination routing, message handling, and configuration. The `LogLevel`, `LogDestination`, `LogMessage`, and `LogEntry` types are domain types in `core::platform::container::log_entry`, but the service that routes and dispatches logs is application-layer coordination.

The remaining managers — `content_service.rs`, `event_manager.rs`, `listener_service.rs`, `scheduler.rs`, `user_service.rs` — also warrant analysis but may be borderline cases where pure domain coordination (no port dependencies) keeps them legitimately in the core layer. This Epic focuses on the clearly misplaced services.

### Acceptance Criteria

1. `notification_service.rs`, `queue_service.rs`, `orchestrator.rs`, and `log_service.rs` are relocated from `core/platform/manager/` to `application/use_cases/` (or appropriate sub-modules within the application layer).
2. Domain types that these services currently define inline (e.g., `OrchestrationContext`, `NotificationServiceStats`, `QueueConfig`) are separated: pure value objects remain in or move to `paladin-core`; service coordination logic moves to the application layer.
3. After relocation, `paladin-core` has zero remaining imports from `application::` or `infrastructure::` — enforced by the workspace crate boundary.
4. The `core/platform/manager/` directory retains only services that contain pure domain logic without port dependencies (e.g., `scheduler.rs` if it operates purely on domain types, `event_manager.rs` if it is a pure event bus).
5. All existing tests pass with updated import paths.
6. The facade crate re-exports maintain backward compatibility for any types that were publicly accessible.

### Tasks

#### Task 2.1: Dependency Analysis of Manager Services

**Description:** For each service in `core/platform/manager/`, produce a dependency map showing: (a) what it imports from `core::`, (b) what it imports from `application::` or `infrastructure::`, (c) what types it defines that are consumed by other layers. Classify each service as "core-appropriate" (pure domain logic) or "application-layer" (depends on ports or coordinates infrastructure).

**Deliverables:**
- Manager service dependency matrix document.
- Classification of each service with justification.
- Identification of inline types that need separation (domain types vs. service coordination types).
- Proposed target locations in the application layer for each relocated service.

**Estimated Effort:** Medium

#### Task 2.2: Separate Domain Types from Service Logic

**Description:** Before moving services, extract any pure domain value objects or entities that are currently defined inside the service files and move them to `paladin-core` container modules. For example, if `orchestrator.rs` defines `OrchestrationContext` as a pure data struct with no port dependencies, it should live in `core/platform/container/`. If `notification_service.rs` defines `NotificationServiceStats`, it should be evaluated for core vs. application placement.

**Deliverables:**
- Domain types extracted to appropriate `paladin-core` container modules.
- Service files reduced to contain only coordination/orchestration logic.
- All references to extracted types updated.
- `cargo build -p paladin-core` succeeds with the new types.

**Estimated Effort:** Medium

#### Task 2.3: Relocate `notification_service.rs`

**Description:** Move the notification service from `core/platform/manager/notification_service.rs` to an appropriate location in the application layer (e.g., `application/use_cases/notifications/notification_orchestrator.rs` or within the `paladin` facade crate's application module). Update all imports. Verify the notification system's end-to-end flow (domain model → service → adapter) remains functional.

**Deliverables:**
- Notification service relocated to the application layer.
- All notification-related tests pass.
- `ServiceRunner` integration updated to instantiate from the new location.
- No compilation of the notification service when building `paladin-core` in isolation.

**Estimated Effort:** Medium

#### Task 2.4: Relocate `queue_service.rs`

**Description:** Move the queue service from `core/platform/manager/queue_service.rs` to the application layer. The `QueueConfig`, `QueueItem`, and `QueueStats` domain types remain in core; the dispatch and coordination logic moves.

**Deliverables:**
- Queue service relocated to the application layer.
- Domain types (`QueueConfig`, `QueueStats`) confirmed in `paladin-core`.
- Queue-related tests pass.
- `Orchestrator` references updated (or relocated simultaneously — see Task 2.5).

**Estimated Effort:** Medium

#### Task 2.5: Relocate `orchestrator.rs`

**Description:** Move the general-purpose orchestrator from `core/platform/manager/orchestrator.rs` to the application layer. This is the most complex relocation because the orchestrator depends on `listener_service`, `queue_service`, `scheduler`, and multiple container types. If `queue_service` was relocated in Task 2.4, the orchestrator's dependency on it already crosses the core→application boundary, making this move necessary.

Evaluate whether `listener_service.rs` and `scheduler.rs` should also relocate or if they are pure domain services that the application-layer orchestrator can depend on via trait abstraction.

**Deliverables:**
- Orchestrator relocated to the application layer.
- `OrchestrationContext` and `OrchestratorStats` placed in the appropriate layer (core if pure data, application if they reference ports).
- Remaining `core/platform/manager/` services either confirmed as correctly placed or flagged for future work.
- All orchestrator and workflow tests pass.

**Estimated Effort:** Large

#### Task 2.6: Relocate `log_service.rs`

**Description:** Move the log service from `core/platform/manager/log_service.rs` to the application layer. The `LogLevel`, `LogDestination`, `LogMessage`, and `LogEntry` domain types remain in `paladin-core`. The `LogService` coordinator with its `LogMessageHandler` and destination routing logic moves to the application layer.

**Deliverables:**
- Log service relocated to the application layer.
- Domain types (`LogLevel`, `LogDestination`, `LogMessage`, `LogEntry`) confirmed in `paladin-core`.
- Log configuration and message handler tests pass.
- `ServiceRunner` integration updated.

**Estimated Effort:** Medium

#### Task 2.7: Verify Core Layer Purity

**Description:** After all relocations, perform a comprehensive audit of `paladin-core` to confirm it has zero remaining dependencies on application or infrastructure modules. Run `cargo build -p paladin-core` in isolation and verify the dependency tree.

**Deliverables:**
- `cargo tree -p paladin-core` output showing only domain-appropriate dependencies.
- Audit report confirming zero application/infrastructure imports in `paladin-core`.
- Updated `core/platform/manager/mod.rs` reflecting the reduced module set.

**Estimated Effort:** Small

---

## Epic 3: Co-locate the Maneuver DSL with the Battalion Execution Layer

**Epic Owner:** TBD
**Priority:** Medium
**Estimated Effort:** Medium
**Dependencies:** Milestone 5 Epic 1 (paladin-core) and Epic 3 (paladin-battalion) must be complete

### Objective

Consolidate the Maneuver DSL components — the lexer, AST, parser (currently in `paladin-core` at `core/platform/container/battalion/parser/`), and the `ManeuverExecutionService` (in `paladin-battalion`) — into a single cohesive location within `paladin-battalion`. These components are tightly coupled and should travel together.

### Background & Rationale

The Maneuver Flow DSL consists of:

- **Lexer** (`core/platform/container/battalion/parser/lexer.rs`) — Tokenizes flow expression strings into `Token` sequences.
- **AST** (`core/platform/container/battalion/parser/ast.rs`) — Defines `FlowExpression` as the parsed tree structure (Agent, Sequential, Parallel nodes).
- **Parser** (`core/platform/container/battalion/parser/mod.rs`) — The `FlowParser` that converts strings to `FlowExpression` ASTs.
- **Error types** (`core/platform/container/battalion/parser/error.rs`) — `FlowParseError` with position tracking and suggestions.
- **Maneuver domain type** (`core/platform/container/battalion/maneuver.rs`) — The `Maneuver` struct and `ManeuverConfig`.
- **Execution service** (`application/use_cases/battalion/maneuver_service.rs`) — `ManeuverExecutionService` that interprets the AST and orchestrates agent execution.
- **Visualization** — ASCII and Mermaid diagram generation for flow expressions.

The parser and AST are only consumed by the Maneuver execution service and the visualization layer. No other battalion pattern uses the Flow DSL parser directly. Keeping the parser in `paladin-core` means it is available to crates that will never use it, and it cannot be modified without rebuilding the core crate.

Moving the parser to `paladin-battalion` means the entire Maneuver subsystem — from string parsing to AST construction to execution to visualization — lives in one crate. The `FlowExpression` AST type, which is the primary public type, can be re-exported from `paladin-core` if needed for backward compatibility, or consumers can depend on `paladin-battalion` directly.

### Acceptance Criteria

1. The `parser/` directory (lexer, AST, parser, error types) moves from `paladin-core` to `paladin-battalion`.
2. The `Maneuver` domain type and `ManeuverConfig` move from `paladin-core` to `paladin-battalion`.
3. `FlowExpression` and `FlowParseError` are publicly exported from `paladin-battalion`.
4. The `paladin` facade crate re-exports `FlowExpression` and `FlowParser` at the original paths for backward compatibility.
5. `paladin-core`'s `battalion/` module no longer contains parser or maneuver modules (only Formation, Phalanx, Campaign, ChainOfCommand, Conclave, Council, Grove domain types, and shared battalion types like `BattalionConfig`, `BattalionResult`, `BattalionStrategy`).
6. All 113 Maneuver-related tests pass from within `paladin-battalion`.
7. All 32 Maneuver benchmarks pass.
8. `cargo build -p paladin-core` no longer compiles any parser code.
9. `cargo doc -p paladin-battalion` documents the complete Maneuver subsystem including parser.

### Tasks

#### Task 3.1: Assess Cross-Crate Impact

**Description:** Identify every file and test that imports from `core::platform::container::battalion::parser` or `core::platform::container::battalion::maneuver`. Map the full impact radius of the move to determine which re-exports are needed in the facade crate.

**Deliverables:**
- Import reference map for parser and maneuver types.
- List of facade re-exports needed for backward compatibility.
- Identification of any types in other crates that depend on `FlowExpression`.

**Estimated Effort:** Small

#### Task 3.2: Move Parser and Maneuver Modules to `paladin-battalion`

**Description:** Physically relocate the `parser/` directory (lexer.rs, ast.rs, error.rs, mod.rs) and `maneuver.rs` from `paladin-core/src/platform/container/battalion/` to `paladin-battalion/src/maneuver/`. Reorganize within `paladin-battalion` so the Maneuver subsystem has a clean internal structure:

```
paladin-battalion/src/
├── maneuver/
│   ├── mod.rs           # Maneuver domain type + ManeuverConfig + re-exports
│   ├── parser/
│   │   ├── mod.rs       # FlowParser
│   │   ├── lexer.rs     # Lexer + Token
│   │   ├── ast.rs       # FlowExpression
│   │   └── error.rs     # FlowParseError
│   ├── service.rs       # ManeuverExecutionService (already here)
│   └── visualizer.rs    # Flow visualization (ASCII + Mermaid)
```

**Deliverables:**
- Parser and maneuver modules relocated.
- `paladin-battalion`'s `Cargo.toml` updated if any new dependencies are needed (unlikely — parser is dependency-free beyond `serde`).
- `cargo build -p paladin-battalion` succeeds.
- `cargo build -p paladin-core` succeeds without parser code.

**Estimated Effort:** Medium

#### Task 3.3: Update Imports and Facade Re-Exports

**Description:** Update all import paths throughout the workspace. Add re-exports in the `paladin` facade crate so that `use paladin::core::platform::container::battalion::parser::FlowParser` continues to resolve (pointing to the new location in `paladin-battalion`).

**Deliverables:**
- All import paths updated.
- Facade re-exports added.
- All examples and integration tests compile.
- `cargo test --workspace` passes.

**Estimated Effort:** Medium

#### Task 3.4: Migrate and Consolidate Tests

**Description:** Move all 57 parser tests and 21 maneuver domain tests from their original locations into `paladin-battalion`'s test suite. Consolidate with the existing 3 execution service tests, 16 commander integration tests, and 12 visualization tests. Verify the full 113-test Maneuver suite passes from `paladin-battalion`.

**Deliverables:**
- All Maneuver tests consolidated in `paladin-battalion`.
- 113 tests passing.
- 32 benchmarks passing.
- No orphaned test files in `paladin-core`.

**Estimated Effort:** Small

---

## Epic 4: Relocate `CircuitBreaker` to the Infrastructure Layer

**Epic Owner:** TBD
**Priority:** Medium
**Estimated Effort:** Small
**Dependencies:** Milestone 5 complete (workspace crates exist)

### Objective

Move the `CircuitBreaker` implementation from `application/use_cases/paladin/circuit_breaker.rs` to the infrastructure layer, alongside retry logic and rate limiting. The circuit breaker pattern is an infrastructure resilience concern, not a domain use case.

### Background & Rationale

The `CircuitBreaker` is currently located at `src/application/use_cases/paladin/circuit_breaker.rs`. It implements the classic circuit breaker pattern with three states (Closed, Open, HalfOpen), failure/success thresholds, and timeout-based recovery. It is generic and reusable — it wraps any fallible operation, not just Paladin-specific operations.

In hexagonal architecture, infrastructure concerns like circuit breaking, retry logic, rate limiting, connection pooling, and timeout management belong in the infrastructure layer. They are implementation details of how the system handles failure, not business logic. The `PaladinExecutionService` would consume the circuit breaker via dependency injection or a port trait, maintaining the proper layering.

The circuit breaker has comprehensive tests (concurrent access, state transitions, threshold behavior) and is used by `PaladinExecutionService` and various example files.

### Acceptance Criteria

1. `CircuitBreaker` and `CircuitState` are relocated from `application/use_cases/paladin/` to an infrastructure module (e.g., `infrastructure/resilience/circuit_breaker.rs` or a dedicated `paladin-infra` utility within the workspace).
2. `PaladinExecutionService` imports `CircuitBreaker` from its new location.
3. All circuit breaker tests pass in the new location.
4. All integration tests and examples that use `CircuitBreaker` compile and pass.
5. The facade crate re-exports `CircuitBreaker` at the original path for backward compatibility.
6. The `application/use_cases/paladin/` directory no longer contains infrastructure concerns.

### Tasks

#### Task 4.1: Determine Target Location

**Description:** Evaluate where the `CircuitBreaker` should live in the workspace:

- **Option A:** `paladin/src/infrastructure/resilience/circuit_breaker.rs` — Keep it in the main facade crate's infrastructure layer. Simple, minimal disruption.
- **Option B:** A new `paladin-infra` crate for shared infrastructure utilities (circuit breaker, retry policies, rate limiters). Cleaner separation but introduces a new crate.
- **Option C:** Within `paladin-battalion` since it's primarily used by execution services. Pragmatic but semantically imprecise.

**Deliverables:**
- Decision document with trade-off analysis.
- Selected target location.

**Estimated Effort:** Small

#### Task 4.2: Relocate `CircuitBreaker`

**Description:** Move `circuit_breaker.rs` to the selected location. Update all imports in `PaladinExecutionService`, examples, and test files.

**Deliverables:**
- `CircuitBreaker` and `CircuitState` relocated.
- All imports updated.
- `cargo build --workspace` succeeds.

**Estimated Effort:** Small

#### Task 4.3: Add Facade Re-Export and Update Documentation

**Description:** Add a re-export in the facade crate so the original import path continues to work. Update the `STABLE_API.md` to reflect the new canonical location. Update `rustdoc` for the circuit breaker module.

**Deliverables:**
- Facade re-export added.
- `STABLE_API.md` updated.
- `cargo doc --workspace --no-deps` clean.
- All tests pass.

**Estimated Effort:** Small

---

## Cross-Epic Deliverables

### Architecture Compliance Report

Upon completion of all four Epics, produce a comprehensive architecture compliance report that verifies:

- The `core` layer contains only domain entities, value objects, and pure domain logic.
- The `application` layer contains use-case orchestration services that depend on port traits.
- The `infrastructure` layer contains all adapter implementations and infrastructure concerns.
- No layer violations exist in the workspace dependency graph.

### Updated Documentation Suite

- `STABLE_API.md` updated with any re-export path changes.
- `README.md` updated with refined architecture description.
- `CONTRIBUTING.md` updated with guidance on where new code belongs (core vs. application vs. infrastructure decision tree).
- Per-crate `rustdoc` verified clean.
- `CHANGELOG.md` entries for all four Epics.

### Technical Debt Inventory

Produce a final technical debt inventory documenting:

- Any remaining architectural concerns not addressed in Refactoring Tiers 1–3.
- Candidate improvements for a future Refactoring Tier 4 (if warranted).
- Performance optimization opportunities identified during the refactoring.
- Areas where additional test coverage would be valuable.

---

## Milestone Schedule Overview

| Phase | Epic | Estimated Duration | Predecessors |
|-------|------|--------------------|--------------|
| Phase 1 | Epic 1: Settings Decomposition | 1–2 sprints | Milestone 5 complete |
| Phase 2A | Epic 2: Manager Service Relocation | 2–3 sprints | Epic 1 recommended |
| Phase 2B | Epic 3: Maneuver DSL Co-location (parallel with Epic 2) | 1–2 sprints | Milestone 5 complete |
| Phase 2B | Epic 4: CircuitBreaker Relocation (parallel with Epics 2, 3) | 0.5–1 sprint | Milestone 5 complete |
| Wrap-up | Cross-Epic Deliverables | 1 sprint | Epics 1–4 |

**Total Estimated Duration:** 3–5 sprints (Epics 2, 3, 4 can be parallelized)

---

## Completion Checklist

- [ ] `application_settings.rs` replaced by per-domain config modules (no file > 400 lines).
- [ ] Shared env var override utility extracted and reused across all config modules.
- [ ] `config.yml` deserialization verified backward-compatible via regression tests.
- [ ] `notification_service.rs` relocated to the application layer.
- [ ] `queue_service.rs` relocated to the application layer.
- [ ] `orchestrator.rs` relocated to the application layer.
- [ ] `log_service.rs` relocated to the application layer.
- [ ] `paladin-core` verified to have zero application/infrastructure imports (`cargo tree` clean).
- [ ] Maneuver DSL parser (lexer, AST, parser, error) moved to `paladin-battalion`.
- [ ] Maneuver domain type and config moved to `paladin-battalion`.
- [ ] All 113 Maneuver tests passing from `paladin-battalion`.
- [ ] `CircuitBreaker` relocated to the infrastructure layer.
- [ ] All facade re-exports verified for backward compatibility.
- [ ] `cargo build --workspace` succeeds.
- [ ] `cargo test --workspace` passes all tests.
- [ ] `cargo clippy --workspace -- -D warnings` clean.
- [ ] `cargo fmt --all -- --check` clean.
- [ ] `cargo doc --workspace --no-deps` clean with no broken links.
- [ ] Architecture compliance report produced.
- [ ] `STABLE_API.md`, `README.md`, `CONTRIBUTING.md` updated.
- [ ] `CHANGELOG.md` updated for all Epics.
- [ ] Technical debt inventory produced.
- [ ] Milestone retrospective completed.

---

## Appendix A: Manager Services Classification

| Service | Current Location | Classification | Target Location | Justification |
|---------|-----------------|---------------|-----------------|---------------|
| `notification_service.rs` | `core/platform/manager/` | **Application layer** | `application/use_cases/notifications/` | Coordinates delivery via port-backed adapters; depends on `MessageService` integration |
| `queue_service.rs` | `core/platform/manager/` | **Application layer** | `application/use_cases/queue/` | Dispatches to queue adapters; manages retry logic |
| `orchestrator.rs` | `core/platform/manager/` | **Application layer** | `application/use_cases/orchestration/` | Coordinates workflows across multiple services and ports |
| `log_service.rs` | `core/platform/manager/` | **Application layer** | `application/use_cases/logging/` | Routes log messages to destinations via port-backed adapters |
| `scheduler.rs` | `core/platform/manager/` | **Borderline — evaluate** | Likely stays in core | May be pure domain scheduling logic; assess port dependencies |
| `event_manager.rs` | `core/platform/manager/` | **Borderline — evaluate** | Likely stays in core | May be a pure event bus with no port dependencies |
| `listener_service.rs` | `core/platform/manager/` | **Borderline — evaluate** | Likely stays in core | Event listener coordination; assess if it depends on ports |
| `content_service.rs` | `core/platform/manager/` | **Borderline — evaluate** | Assess during Task 2.1 | Content processing coordination; may depend on ports |
| `user_service.rs` | `core/platform/manager/` | **Borderline — evaluate** | Assess during Task 2.1 | User management; depends on repository |

## Appendix B: Configuration File Target Structure

```
src/config/                     (or within the appropriate workspace crate)
├── mod.rs                      # Root Settings struct + re-exports
├── env_utils.rs                # Shared environment variable override utility
├── agent.rs                    # PaladinExecutionConfig, defaults
├── garrison.rs                 # GarrisonConfig overrides and defaults
├── arsenal.rs                  # MCPServerConfig, ArsenalSettings
├── notifications.rs            # NotificationConfig, channel settings
├── queue.rs                    # QueueConfig (Redis), env var overrides
├── file_storage.rs             # FileStorageConfig (MinIO/S3), env var overrides
├── web_server.rs               # WebServerConfig, bind address, TLS
├── llm.rs                      # LlmProviderConfig, OpenAI/Anthropic/DeepSeek
├── battalion.rs                # BattalionDefaults, timeout/retry defaults
├── logging.rs                  # LoggingConfig, tracing subscriber settings
└── setup/
    ├── mod.rs                  # setup_and_run()
    └── service_runner.rs       # ServiceRunner (unchanged)
```

## Appendix C: Hexagonal Layer Rules (Post-Milestone 6)

```
┌─────────────────────────────────────────────────────────┐
│                    CORE LAYER                           │
│                  (paladin-core)                          │
│                                                         │
│  CONTAINS:                                              │
│  • Domain entities (Paladin, Battalion types, etc.)     │
│  • Value objects (configs, IDs, enums)                  │
│  • Pure domain logic (no port dependencies)             │
│  • Domain events                                        │
│  • Pure schedulers, event buses (if no port deps)       │
│                                                         │
│  MUST NOT CONTAIN:                                      │
│  • Port trait references                                │
│  • Service coordination that dispatches to adapters     │
│  • Infrastructure concerns (circuit breaker, retry)     │
│  • Configuration loading logic                          │
├─────────────────────────────────────────────────────────┤
│                 APPLICATION LAYER                        │
│         (paladin-ports, paladin-battalion,               │
│          application use-cases in facade)                │
│                                                         │
│  CONTAINS:                                              │
│  • Port trait definitions (paladin-ports)                │
│  • Use-case services (orchestration, execution)         │
│  • Notification/queue/log service coordinators           │
│  • General orchestrator                                  │
│  • Planning and prompt generation services               │
│                                                         │
│  DEPENDS ON: Core layer (domain types)                  │
│  MUST NOT DEPEND ON: Infrastructure (concrete adapters) │
├─────────────────────────────────────────────────────────┤
│               INFRASTRUCTURE LAYER                       │
│       (paladin-llm, paladin-memory, infra modules)       │
│                                                         │
│  CONTAINS:                                              │
│  • Concrete adapter implementations                      │
│  • CircuitBreaker, retry logic, rate limiting            │
│  • HTTP clients, database connections                    │
│  • File system, queue, notification delivery adapters    │
│  • Configuration loading and env var handling            │
│                                                         │
│  DEPENDS ON: Core layer + Application layer (ports)     │
│  IMPLEMENTS: Port traits defined in application layer    │
└─────────────────────────────────────────────────────────┘
```
