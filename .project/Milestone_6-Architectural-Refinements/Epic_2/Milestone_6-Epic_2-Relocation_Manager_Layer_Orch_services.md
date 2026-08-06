
## Epic 2: Relocate Manager-Layer Orchestration Services to the Application Layer

> **See [ADR-0014](../../../.planning/decisions/0014-milestone-4-6-tier-numbering.md)** (dated
> 2026-08-06) for the corrected Milestone/Tier numbering this document's Milestone-numbering
> references predate. This document is a byte-equivalent copy of
> `.project/Milestone_6-Architectural-Refinements/overview/Milestone_6-Tier_3-Architectural-Refinements.md`,
> carrying no independent content beyond that source, which is corrected there. Not corrected
> inline here.

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
