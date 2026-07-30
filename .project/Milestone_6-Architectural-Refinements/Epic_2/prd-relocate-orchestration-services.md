# PRD: Relocate Manager-Layer Orchestration Services to the Application Layer

**Feature Name:** relocate-orchestration-services
**Milestone:** 6 — Architectural Refinements
**Epic:** 2
**Status:** Ready for Implementation
**Created:** 2026-05-24
**Author:** AI-assisted, reviewed by team
**Depends on:** Epic 1 (recommended, not required)

---

## 1. Introduction / Overview

`src/core/platform/manager/` currently contains six services that carry orchestration logic belonging in the application layer: `notification_service.rs`, `queue_service.rs`, `orchestrator.rs`, `log_service.rs`, `listener_service.rs`, and `scheduler.rs`. Each imports from `paladin_ports`, coordinates between port-backed adapters, or manages infrastructure concerns — responsibilities that strict hexagonal architecture assigns exclusively to the application layer. The core layer should contain only entities, value objects, and pure domain logic with zero external dependencies.

**The problem this solves:** Having port-dependent services inside `paladin-core` makes it impossible to build the core in isolation, creates circular reasoning about layer boundaries, and couples domain logic to infrastructure concerns. Any developer who adds a new port dependency to a core service implicitly pulls infrastructure into the domain.

**The goal:** Relocate the six misplaced orchestration services from `src/core/platform/manager/` into `src/application/use_cases/`, extract any pure domain value objects those services currently define inline (moving them to `paladin-core`), and verify that `paladin-core` can be built in complete isolation with zero application or infrastructure imports.

---

## 2. Goals

1. Relocate `notification_service.rs`, `queue_service.rs`, `orchestrator.rs`, `log_service.rs`, `listener_service.rs`, and `scheduler.rs` from `src/core/platform/manager/` to `src/application/use_cases/` sub-modules.
2. Extract all pure domain value objects currently defined inside the service files and place them in `paladin-core` container modules (`crates/paladin-core/src/platform/container/`).
3. After relocation, `cargo build -p paladin-core` must succeed with zero imports from `application::`, `infrastructure::`, or `paladin_ports`.
4. `src/core/platform/manager/` retains only stub modules (`admin/`, `user/`) and the modules confirmed as core-appropriate during dependency analysis — then has its `mod.rs` updated to reflect the reduced set.
5. All existing tests pass with updated import paths after each relocation step.
6. Consumers of relocated services (`service_runner.rs`, `user_config.rs`, `user_service.rs`, `content_ingestion_service.rs`, admin sub-modules) have their import paths updated to point to the new application-layer locations.
7. No file in `src/application/use_cases/` added by this Epic exceeds 600 lines.

---

## 3. User Stories

**As a developer building `paladin-core` in isolation,**
I want `cargo build -p paladin-core` to succeed with no references to ports or adapters,
so that the core domain is truly independent and can be reasoned about without infrastructure context.

**As a developer adding a new notification channel,**
I want the `NotificationOrchestrator` to live in `src/application/use_cases/notification_orchestrator/`,
so that I know where to find and extend it without confusion about whether it belongs to the domain or the application layer.

**As a developer tracing a queue dispatch failure,**
I want `QueueOrchestrator` to live alongside other application-layer services in `use_cases/`,
so that I can reason about its port dependencies and adapters without navigating into the core domain tree.

**As a developer onboarding to the codebase,**
I want `src/core/platform/manager/` to contain only pure domain logic,
so that the directory name accurately describes what it holds and hexagonal layer boundaries are self-documenting.

**As a developer maintaining `user_service.rs`,**
I want its import path for `NotificationOrchestrator` to compile after this Epic,
so that my work is not blocked by the relocation, and I can defer a full `user_service` relocation to a future Epic.

---

## 4. Functional Requirements

### 4.1 Target Directory Structure

After relocation, the new application-layer modules must exist at these paths:

```
src/application/use_cases/
├── notification_orchestrator/
│   ├── mod.rs          # NotificationOrchestrator (renamed from NotificationService)
│   └── types.rs        # NotificationServiceError, NotificationServiceConfig,
│                       # NotificationServiceStats, NotificationDeliveryResult,
│                       # NotificationChannelHandler, NotificationTemplateProcessor
│                       # (or moved to paladin-core if they are pure value objects)
├── queue_orchestrator/
│   ├── mod.rs          # QueueOrchestrator (renamed from QueueService)
│   └── types.rs        # QueueError, QueueServiceConfig, and any other
│                       # coordination types not already in paladin-core
├── orchestration/
│   ├── mod.rs          # Orchestrator
│   ├── listener.rs     # ListenerOrchestrator (relocated from listener_service.rs)
│   ├── scheduler.rs    # SchedulerOrchestrator (relocated from scheduler.rs)
│   └── types.rs        # OrchestrationContext (confirm/move from paladin-core),
│                       # OrchestratorStats, ListenerConfig, ListenerStats, ScheduledJob
│                       # (pure value objects → paladin-core; coordination types → here)
└── log_orchestrator/
    ├── mod.rs          # LogOrchestrator (renamed from LogService)
    └── types.rs        # LogServiceConfig, LogMessageHandler, and other coordination
                        # types not already in paladin-core
```

**Note:** The existing `src/application/notifications/` directory (`email_notifications.rs`, `push_notifications.rs`, `system_notifications.rs`) is a separate module of channel-specific adapters — it must **not** be merged with `notification_orchestrator/`. Both directories will coexist.

### 4.2 Domain Type Placement Rules

The following classification rules govern where types land after extraction:

| Rule | Classification | Destination |
|------|---------------|-------------|
| Struct has no `Arc<dyn Port>` fields and no async fn in impl | Pure value object | `paladin-core` container module |
| Enum is a pure error type with no port references | Domain error | `paladin-core` container module |
| Struct holds `Arc<dyn SomePort>` or calls port methods | Service coordination type | Application layer `types.rs` |
| Struct requires `async_trait` to implement | Coordination/service type | Application layer |
| Struct is already in `paladin-core/src/platform/container/` | Already correct | No move needed |

Types already confirmed in `paladin-core` (verified during codebase analysis):
- `OrchestrationContext` — `crates/paladin-core/src/platform/container/orchestration_context.rs`
- `QueueConfig` — `crates/paladin-core/src/platform/container/queue_config.rs`
- `QueueItem` — `crates/paladin-core/src/platform/container/queue_item.rs`
- `LogLevel`, `LogDestination`, `LogMessage`, `LogEntry` — `crates/paladin-core/src/platform/container/log.rs`
- `Notification`, `NotificationChannel`, `NotificationContent` — `crates/paladin-core/src/platform/container/notification.rs`

### 4.3 Services Confirmed for Relocation

The following six services must be relocated in this Epic:

| Service (current path) | Target path | Reason for relocation |
|----------------------|-------------|----------------------|
| `core/platform/manager/notification_service.rs` | `application/use_cases/notification_orchestrator/` | Imports `paladin_ports`, coordinates delivery via port-backed adapters |
| `core/platform/manager/queue_service.rs` | `application/use_cases/queue_orchestrator/` | Coordinates between `Orchestrator` and external queue adapters |
| `core/platform/manager/orchestrator.rs` | `application/use_cases/orchestration/` | Imports `listener_service`, `queue_service`, `scheduler`; complex workflow coordination |
| `core/platform/manager/log_service.rs` | `application/use_cases/log_orchestrator/` | Imports `paladin_ports::output::log_port`; routes log entries via port adapters |
| `core/platform/manager/listener_service.rs` | `application/use_cases/orchestration/listener.rs` | Orchestrator depends on it directly; relocating together prevents cross-layer coupling |
| `core/platform/manager/scheduler.rs` | `application/use_cases/orchestration/scheduler.rs` | Instantiates concrete `TaskService` implementations (`ContentIndexingService`, `DataBackupService`, `EmailNotificationService`); is application-layer orchestration, not pure domain scheduling |

### 4.4 Services That Remain in `core/platform/manager/`

After relocation, the following modules stay in `core/platform/manager/` (or are confirmed during Task 2.1):

| Service | Status | Reason |
|---------|--------|--------|
| `event_manager.rs` | Remains (confirm in Task 2.1) | Appears to be a pure event bus; no port imports visible |
| `content_service.rs` | Remains (confirm in Task 2.1) | Borderline case; evaluate port dependencies in Task 2.1 |
| `user_service.rs` | Remains with import path update | Has port dependencies but relocation scope exceeds this Epic — flagged for Epic N+1 |
| `admin/` sub-modules | Remain (stubs only) | All three admin services are comment-only stubs with no implementation |
| `user/` sub-modules | Remain (stubs only) | `user_notification_service.rs` uses dead import paths; all stubs — no implementation |

### 4.5 Import Path Updates Required (Non-Relocated Consumers)

The following files do **not** move but must have their import paths updated when the services they depend on are relocated:

| File | Current import | Updated import |
|------|---------------|----------------|
| `src/config/setup/service_runner.rs` | `core::platform::manager::notification_service::NotificationService` | `application::use_cases::notification_orchestrator::NotificationOrchestrator` |
| `src/config/setup/service_runner.rs` | `core::platform::manager::scheduler::Scheduler` | `application::use_cases::orchestration::scheduler::SchedulerOrchestrator` |
| `src/config/user_config.rs` | `core::platform::manager::notification_service::NotificationService` | `application::use_cases::notification_orchestrator::NotificationOrchestrator` |
| `src/config/user_config.rs` | `core::platform::manager::notification_service::NotificationServiceConfig` | `application::use_cases::notification_orchestrator::types::NotificationServiceConfig` |
| `src/core/platform/manager/user_service.rs` | `core::platform::manager::notification_service::NotificationService` | `application::use_cases::notification_orchestrator::NotificationOrchestrator` |
| `src/application/use_cases/content/content_ingestion_service.rs` | `core::platform::manager::orchestrator::*` | `application::use_cases::orchestration::*` |
| `src/infrastructure/web/user_controller.rs` | No direct notification/orchestrator dependency — verify during Task 2.1 | N/A |

### 4.6 Service Renaming Convention

Services that are relocated must be renamed to avoid confusion with similarly-named types in other layers. The suffix `Service` in the application layer is acceptable but the following renames make the layer explicit:

| Old name | New name | Note |
|----------|----------|------|
| `NotificationService` | `NotificationOrchestrator` | Matches the module directory name |
| `QueueService` | `QueueOrchestrator` | Matches the module directory name |
| `LogService` | `LogOrchestrator` | Matches the module directory name |
| `ListenerService` | `ListenerOrchestrator` | Relocated as part of the orchestration module |
| `Scheduler` | `SchedulerOrchestrator` | Relocated as part of the orchestration module |
| `Orchestrator` | `Orchestrator` | Name is already descriptive — no rename needed |

> **Note:** Renaming is optional if the existing name does not conflict with a core domain type. Task 2.1 may revise these recommendations based on actual conflicts found.

### 4.7 `paladin-core` Isolation Requirement

After all relocations are complete:

1. `cargo build -p paladin-core` must succeed with no errors.
2. `cargo tree -p paladin-core` must show zero references to `paladin_ports`, `application::`, or `infrastructure::`.
3. `src/core/platform/manager/mod.rs` must be updated to remove `pub mod` declarations for all relocated services.

### 4.8 Test Coverage Requirements

- All tests that existed before the Epic must pass after each individual relocation step. No test may be left failing between tasks.
- For each domain type extracted in Task 2.2 and placed in `paladin-core` (e.g., `QueueStats`, `NotificationServiceStats`), a `#[cfg(test)] mod tests` block must exist in the same file covering:
  - `Default::default()` produces valid values where applicable.
  - `serde_json::to_string` / `from_str` round-trips correctly where the type derives `Serialize`/`Deserialize`.
  - Error variant `Display` messages format as expected where the type is an error enum.
- Existing integration tests in `tests/` that reference relocated services must have their import paths updated — no new integration tests are required.

---

## 5. Non-Goals (Out of Scope)

1. **Full relocation of `user_service.rs`.** Its import path is updated to compile against the notification orchestrator's new location, but the service itself stays in `core/platform/manager/` and is flagged for a future Epic.
2. **Relocation of `content_service.rs` and `event_manager.rs`.** Task 2.1 will classify them; if they are borderline cases they remain in core for this Epic.
3. **Refactoring the admin/ or user/ sub-module stubs.** All three admin services and all user sub-services are comment-only stubs. They will not be moved, deleted, or implemented in this Epic.
4. **Adding new integration tests for end-to-end notification/queue/log flows.** Existing tests are updated; new integration test suites are out of scope.
5. **Changing the behavior of any relocated service.** This Epic is a pure structural relocation. No logic changes, new features, or refactoring of service internals is permitted.
6. **Updating `config.yml` or any configuration file schema.** Config shapes are not affected by service relocation.
7. **Adding pub-use re-exports in `src/lib.rs`.** Backward compatibility is scoped to compilation — callers will update their import paths. No shim re-exports are added.
8. **Feature-flagging the relocated services.** All six services are compiled unconditionally.

---

## 6. Design Considerations

### Layer Boundary Diagram (After Epic 2)

```
┌──────────────────────────────────────────────────────┐
│  src/application/use_cases/                          │
│  ┌────────────────────────┐  ┌──────────────────┐   │
│  │ notification_           │  │ queue_            │   │
│  │ orchestrator/           │  │ orchestrator/     │   │
│  └────────────────────────┘  └──────────────────┘   │
│  ┌────────────────────────┐  ┌──────────────────┐   │
│  │ orchestration/          │  │ log_orchestrator/ │   │
│  │ (+ listener + scheduler)│  │                  │   │
│  └────────────────────────┘  └──────────────────┘   │
│           │ depends on ↓                              │
├──────────────────────────────────────────────────────┤
│  paladin-ports  (traits only, no implementations)    │
├──────────────────────────────────────────────────────┤
│  paladin-core / src/core/                            │
│  ┌────────────────────────────────────────────────┐ │
│  │ platform/container/  (pure value objects)       │ │
│  │  notification.rs  queue_config.rs  log.rs       │ │
│  │  orchestration_context.rs  trigger.rs  job.rs   │ │
│  └────────────────────────────────────────────────┘ │
│  ┌────────────────────────────────────────────────┐ │
│  │ platform/manager/  (pure domain logic only)     │ │
│  │  event_manager.rs  content_service.rs           │ │
│  │  user_service.rs (import-path updated)          │ │
│  │  admin/ user/ (stubs)                           │ │
│  └────────────────────────────────────────────────┘ │
└──────────────────────────────────────────────────────┘
```

### Incremental Relocation Strategy

Each Task (2.3 – 2.6) must leave the workspace in a **green build state** before the next task begins. The recommended sequence respects the dependency order between services:

```
Task 2.3: notification_service   (no inter-service dependencies among the four)
Task 2.4: queue_service          (no dependency on notification)
Task 2.6: log_service            (no dependency on notification or queue)
Task 2.5: orchestrator + listener + scheduler  (depends on queue; do last)
```

`log_service` is moved before `orchestrator` because it has no dependencies on the other three — separating it reduces the diff size for the most complex task (Task 2.5).

---

## 7. Technical Considerations

### 7.1 Known Inter-Service Dependencies

- `orchestrator.rs` imports `QueueService`, `ListenerService`, and `Scheduler` directly (not through traits). All three must be relocated in the same task (Task 2.5) or the orchestrator's imports would temporarily cross the application→core boundary in the wrong direction.
- `user_service.rs` imports `NotificationService` from `core::platform::manager::notification_service`. After Task 2.3, this import breaks — it must be updated to `application::use_cases::notification_orchestrator::NotificationOrchestrator` within the same PR/commit.
- `content_ingestion_service.rs` (already in the application layer) imports `OrchestrationContext` from `core::platform::manager::orchestrator`. After Task 2.5, this import must be updated; however, `OrchestrationContext` is already in `paladin-core/src/platform/container/orchestration_context.rs`, so the import path should change to `paladin_core::platform::container::orchestration_context::OrchestrationContext`.

### 7.2 Scheduler Concrete Task Service Instantiation

`scheduler.rs` currently instantiates `ContentIndexingService`, `DataBackupService`, and `EmailNotificationService` in its `register_default_services()` method — these are concrete types from `core::platform::container::task`. After relocation, `SchedulerOrchestrator` must still be able to reference these task service types. Since they are domain types (defined in `paladin-core`), this cross-crate dependency (`application → paladin-core`) is legal and expected.

### 7.3 `MessageService` Dependency

Both `notification_service.rs` and `log_service.rs` build on `core::base::service::message_service::MessageService`. After relocation, they will import `MessageService` from `paladin-core` via the workspace dependency. No change to `MessageService` itself is required.

### 7.4 `paladin_ports` Import in `log_service.rs`

`log_service.rs` imports directly from `paladin_ports::output::log_port`. After relocation to the application layer, this import is architecturally correct (application→ports is a legal dependency direction). No changes to `paladin-ports` are needed.

### 7.5 Cargo.toml Dependency Check

After relocation, verify that `paladin-core/Cargo.toml` does not list `paladin-ports` as a dependency. If it does, that dependency must be removed as part of Task 2.7, confirming that `paladin-core` no longer needs it.

---

## 8. Relevant Files

### New Files (to be created)

| File | Purpose |
|------|---------|
| `src/application/use_cases/notification_orchestrator/mod.rs` | `NotificationOrchestrator` struct and impl (relocated) |
| `src/application/use_cases/notification_orchestrator/types.rs` | `NotificationServiceError`, `NotificationServiceConfig`, `NotificationServiceStats`, coordination types |
| `src/application/use_cases/queue_orchestrator/mod.rs` | `QueueOrchestrator` struct and impl (relocated) |
| `src/application/use_cases/queue_orchestrator/types.rs` | `QueueError`, queue coordination types not in paladin-core |
| `src/application/use_cases/orchestration/mod.rs` | `Orchestrator` struct and impl (relocated) |
| `src/application/use_cases/orchestration/listener.rs` | `ListenerOrchestrator` (relocated from `listener_service.rs`) |
| `src/application/use_cases/orchestration/scheduler.rs` | `SchedulerOrchestrator` (relocated from `scheduler.rs`) |
| `src/application/use_cases/orchestration/types.rs` | Coordination types: `OrchestratorStats`, `ListenerConfig`, `ListenerStats`, `ScheduledJob` (or moved to paladin-core if pure value objects) |
| `src/application/use_cases/log_orchestrator/mod.rs` | `LogOrchestrator` struct and impl (relocated) |
| `src/application/use_cases/log_orchestrator/types.rs` | `LogServiceConfig`, `LogMessageHandler`, coordination types |

### Existing Files (to be modified)

| File | Change |
|------|--------|
| `src/core/platform/manager/mod.rs` | Remove `pub mod` for all relocated services |
| `src/application/use_cases/mod.rs` | Add `pub mod` for all four new orchestrator sub-modules |
| `src/config/setup/service_runner.rs` | Update import paths for `NotificationOrchestrator`, `SchedulerOrchestrator` |
| `src/config/user_config.rs` | Update import paths for `NotificationOrchestrator`, `NotificationServiceConfig` |
| `src/core/platform/manager/user_service.rs` | Update import path for `NotificationOrchestrator` |
| `src/application/use_cases/content/content_ingestion_service.rs` | Update import path for `OrchestrationContext` to paladin-core |
| `paladin-core/Cargo.toml` | Remove `paladin-ports` dependency if present (verify in Task 2.7) |

### Reference Files (read-only)

| File | Purpose |
|------|---------|
| `crates/paladin-core/src/platform/container/orchestration_context.rs` | Confirms `OrchestrationContext` already in paladin-core |
| `crates/paladin-core/src/platform/container/queue_config.rs` | Confirms `QueueConfig` already in paladin-core |
| `crates/paladin-core/src/platform/container/log.rs` | Confirms log domain types already in paladin-core |
| `crates/paladin-core/src/platform/container/notification.rs` | Confirms notification domain types already in paladin-core |
| `crates/paladin-ports/src/` | Reference for all port traits the relocated services implement or depend on |

---

## 9. Success Metrics

1. `cargo build -p paladin-core` succeeds with zero errors and zero warnings after Task 2.7.
2. `cargo tree -p paladin-core` shows no `paladin_ports` edge in the dependency graph.
3. `cargo test` passes with no failures after each individual relocation task.
4. `src/core/platform/manager/mod.rs` contains `pub mod` declarations for at most: `event_manager`, `content_service`, `user_service`, `admin`, `user` — the six relocated services are absent.
5. No file added by this Epic exceeds 600 lines (`wc -l` check).
6. Unit tests exist for every domain type extracted from service files to `paladin-core` in Task 2.2, covering Default values, serde round-trips, and error Display formatting where applicable.
7. The `cargo clippy -- -D warnings` command produces zero warnings in the modified files.

---

## 10. Open Questions

1. **`content_service.rs` classification:** Task 2.1 will determine whether `content_service.rs` has any port dependency that would require relocation in this Epic or a follow-up. If it does, the scope of Task 2.2 expands slightly.
2. **`event_manager.rs` classification:** Same as above — Task 2.1 will confirm it is a pure event bus with no port references. If port references are found, it should be flagged for a future Epic (not added to this one's scope, to preserve the Epic's size).
3. **`user_service.rs` future Epic:** Should a dedicated Epic be created for the full `user_service` relocation (including `UserServiceFactory`, `user_config.rs`, user CLI commands, user API controller, and `SqliteUserRepository`)? This is a larger blast radius than the services in Epic 2 and warrants its own planning.
4. **Re-export compatibility:** Should `src/lib.rs` or `src/prelude.rs` add `pub use` re-exports pointing from old paths to new paths to ease migration for downstream consumers? The current decision is no re-exports, but this should be confirmed with the team before implementation begins.
