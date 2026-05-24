## Relevant Files

### New Files (to be created)

- `src/application/use_cases/notification_orchestrator/mod.rs` — `NotificationOrchestrator` struct and impl (relocated and renamed from `NotificationService`).
- `src/application/use_cases/notification_orchestrator/types.rs` — `NotificationServiceError`, `NotificationServiceConfig`, `NotificationServiceStats`, `NotificationChannelHandler` trait, `NotificationTemplateProcessor` trait (coordination types that stay in the application layer).
- `src/application/use_cases/queue_orchestrator/mod.rs` — `QueueOrchestrator` struct and impl (relocated and renamed from `QueueService`).
- `src/application/use_cases/queue_orchestrator/types.rs` — `QueueError` and `Queue` internal struct (coordination types that stay in the application layer).
- `src/application/use_cases/log_orchestrator/mod.rs` — `LogOrchestrator` struct and impl (relocated and renamed from `LogService`).
- `src/application/use_cases/log_orchestrator/types.rs` — `LogServiceConfig`, `LogMessageHandler` struct (coordination types that stay in the application layer).
- `src/application/use_cases/orchestration/mod.rs` — `Orchestrator` struct and impl (relocated).
- `src/application/use_cases/orchestration/listener.rs` — `ListenerOrchestrator` struct and impl (relocated from `listener_service.rs`), `EventListener` trait.
- `src/application/use_cases/orchestration/scheduler.rs` — `SchedulerOrchestrator` struct and impl (relocated from `scheduler.rs`).
- `src/application/use_cases/orchestration/types.rs` — `OrchestratorStats`, `OrchestratorError`, `ContentProcessingResult`, `ContentProcessor` trait, `DefaultContentProcessor`, `ScheduledJob`, `ScheduledJobInfo`, `SchedulerStats`, `SchedulerError`, `ListenerConfig`, `ListenerStats`, `ListenerError` (coordination types).
- `project/Milestone_6-Architectural-Refinements/Epic_2/dependency-analysis.md` — Dependency matrix produced in Task 1.0.

### Existing Files (to be modified or deleted)

- `src/core/platform/manager/notification_service.rs` — **Deleted** after content is moved to the application layer.
- `src/core/platform/manager/queue_service.rs` — **Deleted** after content is moved to the application layer.
- `src/core/platform/manager/log_service.rs` — **Deleted** after content is moved to the application layer.
- `src/core/platform/manager/orchestrator.rs` — **Deleted** after content is moved to the application layer.
- `src/core/platform/manager/listener_service.rs` — **Deleted** after content is moved to the application layer.
- `src/core/platform/manager/scheduler.rs` — **Deleted** after content is moved to the application layer.
- `src/core/platform/manager/mod.rs` — Remove `pub mod` declarations for all six relocated services.
- `src/application/use_cases/mod.rs` — Add `pub mod notification_orchestrator; pub mod queue_orchestrator; pub mod log_orchestrator; pub mod orchestration;`.
- `src/config/setup/service_runner.rs` — Update imports: `NotificationService` → `NotificationOrchestrator`, `Scheduler` → `SchedulerOrchestrator`.
- `src/config/user_config.rs` — Update imports: `NotificationService` → `NotificationOrchestrator`, `NotificationServiceConfig` path.
- `src/core/platform/manager/user_service.rs` — Update import path for `NotificationOrchestrator` (service stays in core for now).
- `src/application/use_cases/content/content_ingestion_service.rs` — Update import path: `core::platform::manager::orchestrator::*` → `application::use_cases::orchestration::*`.
- `src/lib.rs` — Update `pub use core::platform::manager::queue_service::QueueError` to new application-layer path.
- `crates/paladin-core/Cargo.toml` — Remove `paladin-ports` dependency if present (verified in Task 7.0).

### Domain Types (evaluated in Task 2.0 — destinations TBD by placement rules)

- `NotificationServiceStats` — Evaluate: no port fields → likely `paladin-core` container.
- `NotificationServiceConfig` — Evaluate: plain config struct → likely `paladin-core` container.
- `QueueStats` — Evaluate: plain stats struct, check if already in `paladin-core/src/platform/container/`.
- `Schedule` (enum), `ScheduledJob`, `ScheduledJobInfo`, `SchedulerStats` — Evaluate: no port fields → may move to `paladin-core`.
- `ListenerConfig`, `ListenerStats` — Evaluate: no port fields → may move to `paladin-core`.

### Notes

- Unit tests in Rust live **in the same file as the code they test**, inside a `#[cfg(test)] mod tests { use super::*; }` block.
- The workspace must compile successfully (`cargo build`) at the end of **every sub-task that modifies source files**, not just at the end of parent tasks.
- When deleting a source file, always remove its `pub mod` declaration from the parent `mod.rs` in the same commit.
- Do **not** change the behavior of any service. This is a pure structural relocation — no logic changes.
- After each parent task commits, the `cargo test` suite must be fully green before moving to the next parent task.

---

## Instructions for Completing Tasks

**IMPORTANT:** As you complete each task, you must check it off in this markdown file by changing `- [ ]` to `- [x]`. This helps track progress and ensures you don't skip any steps.

Example:

- `- [ ] 1.1 Read file` → `- [x] 1.1 Read file` (after completing)

Update the file after completing each sub-task, not just after completing an entire parent task.

**Rust quality gates — run at the end of every parent task before marking it complete:**

```bash
cargo build                   # Must succeed with zero errors
cargo test                    # All tests must pass
cargo fmt --check             # No formatting issues
cargo clippy -- -D warnings   # Zero warnings
```

**Commit format** (after each parent task passes all quality gates):

```bash
git add .
git commit \
  -m "refactor(epic-2): <summary>" \
  -m "- <bullet 1>" \
  -m "- <bullet 2>" \
  -m "Part of Milestone 6 Epic 2 — Relocate orchestration services"
```

---

## Tasks

- [x] 0.0 Create feature branch
  - [x] 0.1 Confirm you are on `feature/milestone_6`: `git branch --show-current`
  - [x] 0.2 Pull latest changes: `git pull`
  - [x] 0.3 Create and checkout the Epic 2 branch: `git checkout -b feature/milestone_6-epic_2-relocate-orchestration-services`

- [x] 1.0 Dependency analysis of all manager services
  - [x] 1.1 Read `src/core/platform/manager/notification_service.rs` in full. List every `use` statement and every `pub struct`, `pub enum`, `pub trait` it defines.
  - [x] 1.2 Read `src/core/platform/manager/queue_service.rs` in full. List every `use` statement and every `pub struct`, `pub enum` it defines. Note that `QueueConfig` and `QueueItem` are already in `paladin-core` — verify by checking `src/core/platform/manager/queue_service.rs` line 1–25 for the `pub use` re-export.
  - [x] 1.3 Read `src/core/platform/manager/log_service.rs` in full. List every `use` statement and every type it defines. Confirm `LogLevel`, `LogDestination`, `LogMessage`, `LogEntry` are imported from `paladin-core` (not defined here).
  - [x] 1.4 Read `src/core/platform/manager/orchestrator.rs` in full. Note its imports from `listener_service`, `queue_service`, and `scheduler`. List every `pub struct`, `pub enum`, `pub trait` it defines — including `OrchestratorStats`, `ContentProcessingResult`, `ContentProcessor`, `DefaultContentProcessor`, `OrchestratorError`.
  - [x] 1.5 Read `src/core/platform/manager/listener_service.rs` in full. Note it only imports from `core::base::component::event` and `paladin-core` container types. List `ListenerConfig`, `ListenerStats`, `ListenerError`, `EventListener` trait, `ListenerService`.
  - [x] 1.6 Read `src/core/platform/manager/scheduler.rs` in full. Note that it instantiates `ContentIndexingService`, `DataBackupService`, `EmailNotificationService` in `register_default_services()`. List `ScheduledJob`, `Schedule`, `ScheduledJobInfo`, `SchedulerStats`, `SchedulerError`.
  - [x] 1.7 Read `src/core/platform/manager/content_service.rs` in full. Classify it: if it imports from `paladin_ports` or `application::`, it is application-layer; otherwise it is core-appropriate. Record the result.
  - [x] 1.8 Read `src/core/platform/manager/event_manager.rs` in full. Classify it: check for any `paladin_ports` or `application::` imports. Record the result (expected: core-appropriate pure event bus).
  - [x] 1.9 Read `src/core/platform/manager/user_service.rs` lines 1–30. Confirm it imports `NotificationService` and `paladin_ports::output::log_port::LogPort`. Record that it stays in core this Epic with only an import path update after Task 3.0.
  - [x] 1.10 Apply the placement rules from PRD §4.2 to every type listed in steps 1.1–1.6. For each type, record: (a) does it hold `Arc<dyn Port>` fields? (b) does it require `async_trait`? (c) classification: `paladin-core` container or application layer `types.rs`.
  - [x] 1.11 Create `project/Milestone_6-Architectural-Refinements/Epic_2/dependency-analysis.md` with a table summarising each service's imports, types defined, and classification. Include a "Confirmed placement" column for each type.

- [x] 2.0 Extract domain types from service files into `paladin-core`
  - [x] 2.1 For `NotificationServiceStats` (defined in `notification_service.rs`): confirm it has no `Arc<dyn Port>` fields (it only holds `HashMap<NotificationChannel, u64>` counters and datetime fields). Move it to `crates/paladin-core/src/platform/container/notification.rs`. Add `pub use` in `crates/paladin-core/src/platform/container/mod.rs` if not already present.
  - [x] 2.2 For `NotificationServiceConfig` (defined in `notification_service.rs`): confirm it is a plain config struct with `Default` impl and no port refs. Move it to `crates/paladin-core/src/platform/container/notification.rs` alongside the existing notification domain types.
  - [x] 2.3 For `QueueStats` (defined in `queue_service.rs`): check whether it is already re-exported from `paladin-core` (a `pub use` in `queue_service.rs` suggests it may be). If it is defined inline in `queue_service.rs` and not yet in `paladin-core`, move it to `crates/paladin-core/src/platform/container/queue_config.rs`. If already there, confirm its path and do nothing.
  - [x] 2.4 For `Schedule` (enum) and `ScheduledJob` in `scheduler.rs`: confirm neither holds port fields. Move both to a new file `crates/paladin-core/src/platform/container/schedule.rs`. Expose via `crates/paladin-core/src/platform/container/mod.rs`.
  - [x] 2.5 For `ListenerConfig` and `ListenerStats` in `listener_service.rs`: confirm neither holds port fields (they are plain data structs with `Default` impls). Move both to `crates/paladin-core/src/platform/container/trigger.rs` (alongside the existing `Trigger`, `TriggerConfig` types) or a new `listener.rs` container file if that is cleaner.
  - [x] 2.6 For each type moved in steps 2.1–2.5, add a `#[cfg(test)] mod tests` block in the same file with unit tests.
  - [x] 2.7 Update all `use` references to the moved types throughout `src/` and `crates/` — search with `grep -r "NotificationServiceStats\|NotificationServiceConfig\|QueueStats\|Schedule\|ScheduledJob\|ListenerConfig\|ListenerStats" src/` to find callers.
  - [x] 2.8 Run `cargo build -p paladin-core` and fix any compilation errors before proceeding.
  - [x] 2.9 Run `cargo test -p paladin-core` and confirm the new unit tests pass.
  - [x] 2.10 Run `cargo build` (full workspace) to catch any broken imports in dependent crates.
  - [x] 2.11 Run `cargo fmt --check` and `cargo clippy -- -D warnings`; address all warnings.
  - [x] 2.12 Commit: `git commit -m "refactor(epic-2): extract domain types from manager services to paladin-core" ...`

- [ ] 3.0 Relocate `notification_service.rs` to the application layer
  - [ ] 3.1 Create the directory `src/application/use_cases/notification_orchestrator/`.
  - [ ] 3.2 Create `src/application/use_cases/notification_orchestrator/types.rs`. Move the following from `notification_service.rs` into it: `NotificationServiceError` (error enum), `NotificationChannelHandler` (trait), `NotificationTemplateProcessor` (trait), `NotificationDeliveryResult` (if defined inline). Update all intra-file references to use `super::types::` or `crate::application::use_cases::notification_orchestrator::types::`.
  - [ ] 3.3 Create `src/application/use_cases/notification_orchestrator/mod.rs`. Copy the remaining content of `notification_service.rs` (the `NotificationService` struct and all its `impl` blocks) into this file. Rename the struct from `NotificationService` to `NotificationOrchestrator` throughout. Update `use` statements: replace `crate::core::platform::manager::notification_service::` self-references with `crate::application::use_cases::notification_orchestrator::`. Add `pub mod types;` at the top.
  - [ ] 3.4 Add `pub mod notification_orchestrator;` to `src/application/use_cases/mod.rs`.
  - [ ] 3.5 Update `src/config/setup/service_runner.rs`:
    - Replace `use crate::core::platform::manager::notification_service::NotificationService;` with `use crate::application::use_cases::notification_orchestrator::NotificationOrchestrator;`.
    - Replace every occurrence of `NotificationService` in this file with `NotificationOrchestrator`.
    - Replace the inline `crate::core::platform::manager::notification_service::NotificationServiceConfig { ... }` struct literal (line 516) with `crate::application::use_cases::notification_orchestrator::types::NotificationServiceConfig { ... }`.
  - [ ] 3.6 Update `src/config/user_config.rs`:
    - Replace `use crate::core::platform::manager::notification_service::NotificationService;` with `use crate::application::use_cases::notification_orchestrator::NotificationOrchestrator;`.
    - Replace `use crate::core::platform::manager::notification_service::NotificationServiceConfig;` with `use crate::application::use_cases::notification_orchestrator::types::NotificationServiceConfig;`.
    - Replace `Arc<NotificationService>` with `Arc<NotificationOrchestrator>` throughout.
  - [ ] 3.7 Update `src/core/platform/manager/user_service.rs`:
    - Replace `use crate::core::platform::manager::notification_service::NotificationService;` with `use crate::application::use_cases::notification_orchestrator::NotificationOrchestrator;`.
    - Replace `Arc<NotificationService>` with `Arc<NotificationOrchestrator>` throughout.
  - [ ] 3.8 Check `src/lib.rs` for any `pub use` referencing `notification_service` — update the path if found.
  - [ ] 3.9 Delete `src/core/platform/manager/notification_service.rs`.
  - [ ] 3.10 Remove `pub mod notification_service;` from `src/core/platform/manager/mod.rs`.
  - [ ] 3.11 Run `cargo build` — fix all compilation errors before proceeding.
  - [ ] 3.12 Run `cargo test` — all tests must pass.
  - [ ] 3.13 Run `cargo fmt --check` and `cargo clippy -- -D warnings`; address all warnings.
  - [ ] 3.14 Commit: `git commit -m "refactor(epic-2): relocate notification_service to application layer" -m "- Move NotificationService → NotificationOrchestrator in use_cases/notification_orchestrator/" -m "- Update service_runner.rs, user_config.rs, user_service.rs import paths" -m "- Remove notification_service from core/platform/manager/"`

- [ ] 4.0 Relocate `queue_service.rs` to the application layer
  - [ ] 4.1 Create the directory `src/application/use_cases/queue_orchestrator/`.
  - [ ] 4.2 Create `src/application/use_cases/queue_orchestrator/types.rs`. Move `QueueError` (error enum) and the internal `Queue` struct (with its `impl`) into this file. These are coordination types — `Queue` manages in-memory queue state and is not a domain value object.
  - [ ] 4.3 Create `src/application/use_cases/queue_orchestrator/mod.rs`. Move the `QueueService` struct and all its `impl` blocks into this file. Rename `QueueService` to `QueueOrchestrator` throughout. Add `pub mod types;` at the top. Update `use` statements to remove `crate::core::platform::manager::queue_service::` self-references and import `QueueStats` from its new `paladin-core` location (confirmed in Task 2.3).
  - [ ] 4.4 Add `pub mod queue_orchestrator;` to `src/application/use_cases/mod.rs`.
  - [ ] 4.5 Update `src/lib.rs`: change `pub use core::platform::manager::queue_service::QueueError;` to `pub use crate::application::use_cases::queue_orchestrator::types::QueueError;`.
  - [ ] 4.6 Search for any other consumers of `queue_service`: `grep -r "core::platform::manager::queue_service" src/` — update any remaining import paths found.
  - [ ] 4.7 Delete `src/core/platform/manager/queue_service.rs`.
  - [ ] 4.8 Remove `pub mod queue_service;` from `src/core/platform/manager/mod.rs`.
  - [ ] 4.9 Run `cargo build` — fix all compilation errors before proceeding.
  - [ ] 4.10 Run `cargo test` — all tests must pass.
  - [ ] 4.11 Run `cargo fmt --check` and `cargo clippy -- -D warnings`; address all warnings.
  - [ ] 4.12 Commit: `git commit -m "refactor(epic-2): relocate queue_service to application layer" -m "- Move QueueService → QueueOrchestrator in use_cases/queue_orchestrator/" -m "- Update src/lib.rs pub use path for QueueError" -m "- Remove queue_service from core/platform/manager/"`

- [ ] 5.0 Relocate `log_service.rs` to the application layer
  - [ ] 5.1 Create the directory `src/application/use_cases/log_orchestrator/`.
  - [ ] 5.2 Create `src/application/use_cases/log_orchestrator/types.rs`. Move `LogServiceConfig` (config struct) and `LogMessageHandler` (struct with its `impl`) into this file. These are coordination types — `LogMessageHandler` holds `Arc<dyn LogPort>` (a port reference).
  - [ ] 5.3 Create `src/application/use_cases/log_orchestrator/mod.rs`. Move the `LogService` struct and all its `impl` blocks into this file. Rename `LogService` to `LogOrchestrator` throughout. Add `pub mod types;` at the top. Update `use` statements: `LogLevel`, `LogDestination`, `LogMessage`, `LogEntry` are imported from `paladin-core::platform::container::log` — verify these paths are correct.
  - [ ] 5.4 Add `pub mod log_orchestrator;` to `src/application/use_cases/mod.rs`.
  - [ ] 5.5 Search for consumers: `grep -r "core::platform::manager::log_service" src/` — update any import paths found (expected: `service_runner.rs` if it uses `LogService` directly).
  - [ ] 5.6 Check `src/lib.rs` for any `pub use` referencing `log_service` — update if found.
  - [ ] 5.7 Delete `src/core/platform/manager/log_service.rs`.
  - [ ] 5.8 Remove `pub mod log_service;` from `src/core/platform/manager/mod.rs`.
  - [ ] 5.9 Run `cargo build` — fix all compilation errors before proceeding.
  - [ ] 5.10 Run `cargo test` — all tests must pass.
  - [ ] 5.11 Run `cargo fmt --check` and `cargo clippy -- -D warnings`; address all warnings.
  - [ ] 5.12 Commit: `git commit -m "refactor(epic-2): relocate log_service to application layer" -m "- Move LogService → LogOrchestrator in use_cases/log_orchestrator/" -m "- LogLevel/LogDestination/LogMessage/LogEntry remain in paladin-core::container::log" -m "- Remove log_service from core/platform/manager/"`

- [ ] 6.0 Relocate `orchestrator.rs`, `listener_service.rs`, and `scheduler.rs` to the application layer
  - [ ] 6.1 Create the directory `src/application/use_cases/orchestration/`.
  - [ ] 6.2 Create `src/application/use_cases/orchestration/types.rs`. Move the following coordination types into it from their source files:
    - From `orchestrator.rs`: `OrchestratorStats` (references `SchedulerStats`, `QueueStats`, `ListenerStats` — all are coordination types), `OrchestratorError`, `ContentProcessingResult`, `ContentProcessor` trait, `DefaultContentProcessor`.
    - From `listener_service.rs`: `ListenerError` (error enum — coordination error, not pure domain).
    - From `scheduler.rs`: `SchedulerError`, `ScheduledJobInfo` (references `ActionStatus` and `Schedule` — evaluate: if `ActionStatus` is a paladin-core type this may move to paladin-core).
  - [ ] 6.3 Create `src/application/use_cases/orchestration/listener.rs`. Move the `ListenerService` struct and all its `impl` blocks into this file. Rename to `ListenerOrchestrator`. Move the `EventListener` trait here as well. Update `use` statements: `ListenerConfig` and `ListenerStats` now come from their new `paladin-core` container location (moved in Task 2.5).
  - [ ] 6.4 Create `src/application/use_cases/orchestration/scheduler.rs`. Move the `Scheduler` struct and all its `impl` blocks into this file. Rename to `SchedulerOrchestrator`. Update `use` statements: `Schedule` and `ScheduledJob` now come from their new `paladin-core` container location (moved in Task 2.4). Keep the import for `ContentIndexingService`, `DataBackupService`, `EmailNotificationService` from `crate::core::platform::container::task` — this is a legal application→core dependency.
  - [ ] 6.5 Create `src/application/use_cases/orchestration/mod.rs`. Move the `Orchestrator` struct and all its `impl` blocks into this file. Add at the top: `pub mod listener; pub mod scheduler; pub mod types;`. Update all internal references: replace `crate::core::platform::manager::listener_service::` with `crate::application::use_cases::orchestration::listener::`, replace `crate::core::platform::manager::queue_service::` with `crate::application::use_cases::queue_orchestrator::`, replace `crate::core::platform::manager::scheduler::` with `crate::application::use_cases::orchestration::scheduler::`. The `pub use` for `OrchestrationContext` in `orchestrator.rs` line 18 (`pub use crate::core::platform::container::orchestration_context::OrchestrationContext;`) must be preserved in the new `mod.rs` so existing consumers get it via `orchestration::OrchestrationContext`.
  - [ ] 6.6 Add `pub mod orchestration;` to `src/application/use_cases/mod.rs`.
  - [ ] 6.7 Update `src/application/use_cases/content/content_ingestion_service.rs`:
    - Replace `use crate::core::platform::manager::orchestrator::{ContentAnalysisType, OrchestrationContext, Orchestrator};` with `use crate::application::use_cases::orchestration::{ContentAnalysisType, OrchestrationContext, Orchestrator};`.
    - Note: `ContentAnalysisType` must be exported from the new `orchestration` module — add a `pub use` in `mod.rs` if it is defined in `types.rs`.
  - [ ] 6.8 Update `src/config/setup/service_runner.rs`: the `Scheduler` import was updated in Task 3.5 — confirm it now references `SchedulerOrchestrator` from the new path `crate::application::use_cases::orchestration::scheduler::SchedulerOrchestrator`. If Task 3.5 updated it to a different interim path, correct it now.
  - [ ] 6.9 Search for any remaining consumers: `grep -r "core::platform::manager::orchestrator\|core::platform::manager::listener_service\|core::platform::manager::scheduler" src/` — update any remaining import paths found.
  - [ ] 6.10 Delete `src/core/platform/manager/orchestrator.rs`, `src/core/platform/manager/listener_service.rs`, and `src/core/platform/manager/scheduler.rs`.
  - [ ] 6.11 Remove `pub mod orchestrator; pub mod listener_service; pub mod scheduler;` from `src/core/platform/manager/mod.rs`.
  - [ ] 6.12 Run `cargo build` — fix all compilation errors before proceeding. This is the most complex relocation; expect multiple rounds of import fixes.
  - [ ] 6.13 Run `cargo test` — all tests must pass.
  - [ ] 6.14 Run `cargo fmt --check` and `cargo clippy -- -D warnings`; address all warnings.
  - [ ] 6.15 Commit: `git commit -m "refactor(epic-2): relocate orchestrator, listener_service, and scheduler to application layer" -m "- Orchestrator → use_cases/orchestration/mod.rs" -m "- ListenerService → ListenerOrchestrator in orchestration/listener.rs" -m "- Scheduler → SchedulerOrchestrator in orchestration/scheduler.rs" -m "- Update content_ingestion_service.rs and service_runner.rs import paths" -m "- Remove all three services from core/platform/manager/"`

- [ ] 7.0 Verify core layer purity and finalize `core/platform/manager/`
  - [ ] 7.1 Run `cargo build -p paladin-core` in isolation. Confirm zero errors and zero warnings.
  - [ ] 7.2 Run `cargo tree -p paladin-core --edges normal` and inspect the output. Confirm that `paladin-ports` does **not** appear as a dependency edge. If it does, open `crates/paladin-core/Cargo.toml` and remove the `paladin-ports` dependency entry, then re-run `cargo build -p paladin-core`.
  - [ ] 7.3 Search for any remaining `core::platform::manager::` references to relocated services across the entire workspace: `grep -r "core::platform::manager::\(notification_service\|queue_service\|log_service\|orchestrator\|listener_service\|scheduler\)" src/ crates/` — fix any that remain.
  - [ ] 7.4 Verify `src/core/platform/manager/mod.rs` contains `pub mod` declarations for only the remaining services: `content_service`, `event_manager`, `user_service`, `admin`, `user`. Confirm no relocated services appear.
  - [ ] 7.5 Verify `src/application/use_cases/mod.rs` exposes all four new sub-modules: `notification_orchestrator`, `queue_orchestrator`, `log_orchestrator`, `orchestration`.
  - [ ] 7.6 Run the full workspace test suite: `cargo test`. All tests must pass.
  - [ ] 7.7 Run `cargo clippy -- -D warnings` across the workspace; fix any warnings.
  - [ ] 7.8 Run `cargo fmt`; commit any formatting changes.
  - [ ] 7.9 Commit: `git commit -m "refactor(epic-2): verify core layer purity and finalize manager mod.rs" -m "- paladin-core builds in isolation with zero paladin-ports dependency" -m "- core/platform/manager/ retains only content_service, event_manager, user_service, admin/, user/" -m "- All 4 new orchestrator sub-modules registered in use_cases/mod.rs" -m "- Full test suite green"`
