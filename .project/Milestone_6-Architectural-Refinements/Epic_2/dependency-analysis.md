# Epic 2 — Dependency Analysis: Manager-Layer Orchestration Services

> Generated during Task 1.0 of `tasks-relocate-orchestration-services.md`

---

## 1. Services to Relocate

| File | Current Location | Target Location |
|------|-----------------|-----------------|
| `notification_service.rs` | `src/core/platform/manager/` | `src/application/use_cases/notification_orchestrator/` |
| `queue_service.rs` | `src/core/platform/manager/` | `src/application/use_cases/queue_orchestrator/` |
| `log_service.rs` | `src/core/platform/manager/` | `src/application/use_cases/log_orchestrator/` |
| `orchestrator.rs` | `src/core/platform/manager/` | `src/application/use_cases/orchestration/` |
| `listener_service.rs` | `src/core/platform/manager/` | `src/application/use_cases/orchestration/` |
| `scheduler.rs` | `src/core/platform/manager/` | `src/application/use_cases/orchestration/` |

---

## 2. Services Staying in Core

| File | Reason |
|------|--------|
| `content_service.rs` | No `paladin_ports` or `application::` imports — pure domain versioning service |
| `event_manager.rs` | No `paladin_ports` or `application::` imports — pure in-process event bus |
| `user_service.rs` | Already crosses boundaries (`application::storage`, `paladin_ports`) but deferred to future Epic; only import path update this Epic |
| `admin/` | Comment-only stubs; no action |
| `user/` | Comment-only stubs (dead import paths); no action this Epic |

---

## 3. Import Analysis per Service File

### 3.1 `notification_service.rs`

**`use` statements:**
```
async_trait::async_trait
chrono::{DateTime, Utc}
std::collections::HashMap
std::sync::Arc
tokio::sync::{RwLock, mpsc}
uuid::Uuid
crate::core::base::entity::message::{Location, Message}
crate::core::base::service::message_service::{MessageError, MessageHandler, MessageResult, MessageService}
crate::core::platform::container::notification::{Notification, NotificationChannel, NotificationContent,
  NotificationDomainError, NotificationEvent, NotificationPriority, NotificationRecipient,
  NotificationStatus, NotificationTemplate}
```

**Port dependencies:** None direct — all types come from `core::base` and `core::platform::container`.

### 3.2 `queue_service.rs`

**`use` statements:**
```
pub use crate::core::platform::container::queue_config::QueueConfig  ← re-export (already paladin-core)
crate::core::platform::container::queue_item::QueueItem
chrono::{DateTime, Utc}
serde::{Deserialize, Serialize}
std::collections::{HashMap, VecDeque}
std::sync::Arc
thiserror::Error
tokio::sync::{Mutex, RwLock}
uuid::Uuid
```

**Port dependencies:** None.

### 3.3 `log_service.rs`

**`use` statements:**
```
async_trait::async_trait
chrono::Utc
std::collections::HashMap
std::sync::Arc
tokio::sync::RwLock
crate::core::base::entity::message::Location
crate::core::base::service::message_service::{MessageError, MessageHandler, MessageResult,
  MessageService, MessageServiceConfig}
crate::core::platform::container::log::{Log, LogContainer, LogDestination, LogEntry,
  LogEntryExt, LogLevel, LogMessage}
paladin_ports::output::log_port::{LogError, LogHealthCheck, LogPort, LogQuery, LogResult, LogStats}
```

**Port dependencies:** `paladin_ports::output::log_port::LogPort` — confirms this is an application-layer coordination service.

### 3.4 `orchestrator.rs`

**`use` statements:**
```
crate::core::base::component::action::{Action, ActionPriority}
crate::core::base::component::event::Event
crate::core::base::entity::message::{Location, Message, MessagePriority}
crate::core::platform::container::content::ContentItem
crate::core::platform::container::job::{Job, JobError}
pub use crate::core::platform::container::orchestration_context::OrchestrationContext  ← re-export
crate::core::platform::container::queue_item::QueueItem
crate::core::platform::container::task::{Task, TaskError, TaskService}
crate::core::platform::container::trigger::{Trigger, TriggerCondition}
crate::core::platform::container::workflow::{Workflow, WorkflowExecutionOrder, WorkflowListener}
crate::core::platform::manager::listener_service::{EventListener, ListenerError, ListenerService}
crate::core::platform::manager::queue_service::{QueueError, QueueService}
crate::core::platform::manager::scheduler::{Schedule, Scheduler, SchedulerError}
async_trait::async_trait
chrono::Utc
serde::{Deserialize, Serialize}
std::collections::HashMap
std::sync::Arc
thiserror::Error
tokio::sync::{Mutex, RwLock}
uuid::Uuid
```

**Port dependencies:** None direct, but it imports all three sibling services (listener, queue, scheduler). Must move as a group.

**Critical re-export:** `pub use crate::core::platform::container::orchestration_context::OrchestrationContext;` — must be preserved in new `orchestration/mod.rs`.

### 3.5 `listener_service.rs`

**`use` statements:**
```
crate::core::base::component::event::Event
crate::core::platform::container::trigger::{Trigger, TriggerCondition, TriggerConfig, TriggerStatus, TriggerSummary}
async_trait::async_trait
chrono::{DateTime, Utc}
serde::{Deserialize, Serialize}
std::collections::{HashMap, VecDeque}
std::sync::Arc
thiserror::Error
tokio::sync::{Mutex, RwLock}
uuid::Uuid
```

**Port dependencies:** None — imports only from `core::base` and `core::platform::container`.

### 3.6 `scheduler.rs`

**`use` statements:**
```
crate::core::base::component::action::{ActionPriority, ActionStatus}
crate::core::platform::container::job::{Job, JobError}
crate::core::platform::container::task::{ContentIndexingService, DataBackupService,
  EmailNotificationService, Task, TaskService}
chrono::{DateTime, Datelike, Utc}
std::collections::HashMap
std::time::Duration
thiserror::Error
tokio::time::interval
uuid::Uuid
```

**Port dependencies:** None — imports only from `core::base` and `core::platform::container`.

---

## 4. Type Classification Table

PRD §4.2 placement rules applied to each type:

| Type | Source File | Has `Arc<dyn Port>` field? | Requires `async_trait`? | Classification | Target Location |
|------|-------------|--------------------------|------------------------|---------------|-----------------|
| `NotificationServiceStats` | notification_service.rs | No | No | ✅ paladin-core eligible | `paladin-core::container::notification` |
| `NotificationServiceConfig` | notification_service.rs | No | No | ✅ paladin-core eligible | `paladin-core::container::notification` |
| `NotificationServiceError` | notification_service.rs | No | No | ⚠️ Application layer | `notification_orchestrator/types.rs` (references `NotificationDomainError`, `MessageError`) |
| `NotificationChannelHandler` | notification_service.rs | No (it IS a port-like trait) | Yes (`async_trait`) | ⚠️ Application layer | `notification_orchestrator/types.rs` |
| `NotificationTemplateProcessor` | notification_service.rs | No | Yes (`async_trait`) | ⚠️ Application layer | `notification_orchestrator/types.rs` |
| `NotificationDeliveryResult` | notification_service.rs | No | No | ⚠️ Application layer | `notification_orchestrator/types.rs` (orchestration result) |
| `NotificationService` | notification_service.rs | Yes (`Arc<dyn NotificationChannelHandler>`) | No | ⚠️ Application layer | `notification_orchestrator/mod.rs` → rename `NotificationOrchestrator` |
| `QueueConfig` | queue_service.rs | No | No | Already in paladin-core | `paladin-core::container::queue_config` (re-exported, no change) |
| `QueueStats` | queue_service.rs | No | No | ✅ paladin-core eligible | `paladin-core::container::queue_config` |
| `QueueError` | queue_service.rs | No | No | ⚠️ Application layer | `queue_orchestrator/types.rs` (coordination error) |
| `Queue` (internal) | queue_service.rs | No | No | ⚠️ Application layer | `queue_orchestrator/types.rs` (internal impl detail) |
| `QueueService` | queue_service.rs | No direct, but owns `Arc<Mutex<Queue>>` | No | ⚠️ Application layer | `queue_orchestrator/mod.rs` → rename `QueueOrchestrator` |
| `LogServiceConfig` | log_service.rs | No | No | ✅ paladin-core eligible | **Kept in application layer** — contains `MessageServiceConfig` and orchestration tuning fields; part of `LogOrchestrator` setup |
| `LogMessageHandler` | log_service.rs | Yes (`Option<Arc<dyn LogPort>>`) | No | ⚠️ Application layer | `log_orchestrator/types.rs` |
| `LogService` | log_service.rs | Yes (`Option<Arc<dyn LogPort>>`) | No | ⚠️ Application layer | `log_orchestrator/mod.rs` → rename `LogOrchestrator` |
| `OrchestratorStats` | orchestrator.rs | No (plain aggregate of sub-stats) | No | ⚠️ Application layer | `orchestration/types.rs` (references SchedulerStats, QueueStats, ListenerStats) |
| `OrchestratorError` | orchestrator.rs | No | No | ⚠️ Application layer | `orchestration/types.rs` (coordination error, wraps sub-errors) |
| `ContentProcessingResult` | orchestrator.rs | No | No | ⚠️ Application layer | `orchestration/types.rs` (references `OrchestrationContext`) |
| `ContentProcessor` | orchestrator.rs | No | Yes (`async_trait`) | ⚠️ Application layer | `orchestration/types.rs` |
| `DefaultContentProcessor` | orchestrator.rs | No | No | ⚠️ Application layer | `orchestration/types.rs` |
| `ContentAnalysisType` | orchestrator.rs | No | No | ✅ paladin-core eligible | **Kept in application layer** for now — tightly coupled to orchestration use-case semantics |
| `Orchestrator` | orchestrator.rs | Yes (owns `Arc<QueueService>`, `Arc<ListenerService>`) | No | ⚠️ Application layer | `orchestration/mod.rs` |
| `ListenerConfig` | listener_service.rs | No | No | ✅ paladin-core eligible | `paladin-core::container::trigger` (alongside `TriggerConfig`) |
| `ListenerStats` | listener_service.rs | No | No | ✅ paladin-core eligible | `paladin-core::container::trigger` |
| `ListenerError` | listener_service.rs | No | No | ⚠️ Application layer | `orchestration/types.rs` (coordination error) |
| `EventListener` | listener_service.rs | No | Yes (`async_trait`) | ⚠️ Application layer | `orchestration/listener.rs` |
| `ListenerService` | listener_service.rs | No direct, owns listeners map | No | ⚠️ Application layer | `orchestration/listener.rs` → rename `ListenerOrchestrator` |
| `Schedule` | scheduler.rs | No | No | ✅ paladin-core eligible | `paladin-core::container::schedule` (new file) |
| `ScheduledJob` | scheduler.rs | No | No | ✅ paladin-core eligible | `paladin-core::container::schedule` |
| `ScheduledJobInfo` | scheduler.rs | No | No | ✅ paladin-core eligible | `paladin-core::container::schedule` (references `ActionStatus`, `Schedule`) |
| `SchedulerStats` | scheduler.rs | No | No | ✅ paladin-core eligible | `paladin-core::container::schedule` |
| `SchedulerError` | scheduler.rs | No | No | ⚠️ Application layer | `orchestration/types.rs` (coordination error, wraps `JobError`) |
| `Scheduler` | scheduler.rs | No direct, owns service map | No | ⚠️ Application layer | `orchestration/scheduler.rs` → rename `SchedulerOrchestrator` |

---

## 5. Types Moving to `paladin-core` (Task 2.0 targets)

| Type | Destination File | Action |
|------|-----------------|--------|
| `NotificationServiceStats` | `crates/paladin-core/src/platform/container/notification.rs` | Add to existing file |
| `NotificationServiceConfig` | `crates/paladin-core/src/platform/container/notification.rs` | Add to existing file |
| `QueueStats` | `crates/paladin-core/src/platform/container/queue_config.rs` | Add to existing file |
| `Schedule` | `crates/paladin-core/src/platform/container/schedule.rs` | New file |
| `ScheduledJob` | `crates/paladin-core/src/platform/container/schedule.rs` | New file |
| `ScheduledJobInfo` | `crates/paladin-core/src/platform/container/schedule.rs` | New file |
| `SchedulerStats` | `crates/paladin-core/src/platform/container/schedule.rs` | New file |
| `ListenerConfig` | `crates/paladin-core/src/platform/container/trigger.rs` | Add to existing file |
| `ListenerStats` | `crates/paladin-core/src/platform/container/trigger.rs` | Add to existing file |

---

## 6. Import Path Update Map (for downstream consumers)

| Old Path | New Path | Updated In |
|----------|----------|-----------|
| `core::platform::manager::notification_service::NotificationService` | `application::use_cases::notification_orchestrator::NotificationOrchestrator` | `service_runner.rs`, `user_config.rs`, `user_service.rs` |
| `core::platform::manager::notification_service::NotificationServiceConfig` | `application::use_cases::notification_orchestrator::types::NotificationServiceConfig` | `service_runner.rs`, `user_config.rs` |
| `core::platform::manager::queue_service::QueueError` | `application::use_cases::queue_orchestrator::types::QueueError` | `src/lib.rs` |
| `core::platform::manager::orchestrator::{ContentAnalysisType, OrchestrationContext, Orchestrator}` | `application::use_cases::orchestration::{ContentAnalysisType, OrchestrationContext, Orchestrator}` | `content_ingestion_service.rs` |
| `core::platform::manager::scheduler::Scheduler` | `application::use_cases::orchestration::scheduler::SchedulerOrchestrator` | `service_runner.rs` |

---

## 7. `service_runner.rs` Import Dependency Map

Lines requiring update after each task:

| Task | Line | Old | New |
|------|------|-----|-----|
| 3.0 | 6 | `core::platform::manager::notification_service::NotificationService` | `application::use_cases::notification_orchestrator::NotificationOrchestrator` |
| 3.0 | 7 | `core::platform::manager::scheduler::Scheduler` | `application::use_cases::orchestration::scheduler::SchedulerOrchestrator` |
| 3.0 | 516 | inline `notification_service::NotificationServiceConfig { ... }` | `notification_orchestrator::types::NotificationServiceConfig { ... }` |

---

## 8. Core Layer Services — Classification Summary

| File | Verdict | Reason |
|------|---------|--------|
| `content_service.rs` | ✅ Core-appropriate | No `paladin_ports`, no `application::` imports |
| `event_manager.rs` | ✅ Core-appropriate | No `paladin_ports`, no `application::` imports — pure event bus |
| `user_service.rs` | ⚠️ Stays in core this Epic | Already violates hexagonal (imports `application::storage::user_store`, `paladin_ports`) — deferred to future Epic |
