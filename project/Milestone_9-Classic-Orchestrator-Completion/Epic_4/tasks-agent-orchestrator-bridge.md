# Tasks: Agent → Orchestrator Bridge (Milestone 9, Epic 4)

Source PRD: [prd-agent-orchestrator-bridge.md](prd-agent-orchestrator-bridge.md)
Epic spec: [Milestone_9-Epic_4-agent-orchestrator-bridge.md](Milestone_9-Epic_4-agent-orchestrator-bridge.md)

## Relevant Files

- `crates/paladin-ports/src/output/orchestrator_port.rs` - New: `OrchestratorPort` trait, request/result value objects, `OrchestratorBridgeError`, `BridgePolicy`, `BridgeAction`.
- `crates/paladin-ports/src/output/mod.rs` - Modified: register and re-export the new `orchestrator_port` module.
- `src/application/services/orchestration/orchestrator_bridge.rs` - New: concrete `OrchestratorBridgeAdapter` (root crate) implementing `OrchestratorPort` over `Orchestrator` + `NotificationDeliveryPort`.
- `src/application/services/orchestration/mod.rs` - Modified: declare/re-export the `orchestrator_bridge` module.
- `src/application/services/paladin/paladin_execution_service.rs` - Modified: add optional `orchestrator_port: Option<Arc<dyn OrchestratorPort>>` field + backward-compatible setter.
- `tests/agent_orchestrator_bridge.rs` - New: integration test driving a scripted agent tool-call → `schedule_job` → assert observable in orchestrator state.

### Notes

- Unit tests live in `#[cfg(test)] mod tests` blocks within `orchestrator_port.rs` and `orchestrator_bridge.rs`.
- Integration tests go in the top-level `tests/` directory.
- Run `cargo test` for all tests, `cargo test --test agent_orchestrator_bridge` for the integration test.
- Quality gate per parent task: `cargo test`, `cargo fmt --check`, `cargo clippy -- -D warnings`.
- Crate boundary: `paladin-ports` must NOT depend on the root crate; the adapter (root crate) depends on both.
- Use `set +H &&` before git commits; stage only the specific files changed in each parent task.

## Instructions for Completing Tasks

**IMPORTANT:** As you complete each task, check it off by changing `- [ ]` to `- [x]`. Update after each sub-task, not just each parent task.

## Tasks

- [x] 0.0 Create feature branch
  - [x] 0.1 Create and checkout `feature/milestone_9-epic_4-agent-orchestrator-bridge` from the current branch.

- [x] 1.0 Design & define the `OrchestratorPort` interface in `paladin-ports` (Task 4.1; FR 1–11)
  - [x] 1.1 Create `crates/paladin-ports/src/output/orchestrator_port.rs` with module-level rustdoc documenting the Option A vs B trade-off and the chosen approach (Option A).
  - [x] 1.2 Define `BridgeAction` enum (`ScheduleJob`, `QueueItem`, `FireEvent`, `SendNotification`) with rustdoc.
  - [x] 1.3 Define request value objects: `ScheduleJobRequest`, `QueueItemRequest`, `FireEventRequest`, `SendNotificationRequest` (serializable, LLM-friendly, using only `paladin-core`/primitive/`serde_json` types) and `EventDispatchResult`.
  - [x] 1.4 Define `OrchestratorBridgeError` (thiserror) with `ActionNotAllowed`, `QuotaExceeded { action, limit }`, `InvalidRequest`, `OrchestratorError`.
  - [x] 1.5 Define `BridgePolicy` with an action allow-list + per-execution caps, a conservative `Default`, and constructors/builder; add a `check(action)`-style method signature contract (enforcement implemented in adapter).
  - [x] 1.6 Define `OrchestratorPort: Send + Sync` (`async_trait`) with the four methods returning `Result<_, OrchestratorBridgeError>`; full rustdoc on trait + methods.
  - [x] 1.7 Register `pub mod orchestrator_port;` and re-export public types in `crates/paladin-ports/src/output/mod.rs`.
  - [x] 1.8 Unit tests in the module: `BridgePolicy::default` contents, allow-list checks, and cap accounting helpers.
  - [x] 1.9 Quality gate (`cargo test`, `cargo fmt --check`, `cargo clippy -- -D warnings`), then commit.

- [x] 2.0 Implement the concrete bridge adapter in the root crate (Task 4.2; FR 12–18)
  - [x] 2.1 Create `src/application/services/orchestration/orchestrator_bridge.rs` with module-level docs explaining root-crate placement (depends on `Orchestrator`).
  - [x] 2.2 Define `OrchestratorBridgeAdapter { orchestrator: Arc<Orchestrator>, policy: BridgePolicy, notification: Option<Arc<dyn NotificationDeliveryPort>>, counters }` with a constructor; thread-safe per-execution counters.
  - [x] 2.3 Implement `schedule_job`: enforce policy → build `Job::new` → call `Orchestrator::schedule_job` → return `Uuid`; map errors.
  - [x] 2.4 Implement `queue_item`: enforce policy → enqueue via orchestrator queue capability → return item `Uuid`; map errors.
  - [x] 2.5 Implement `fire_event`: enforce policy → build `Event::new` → dispatch via listener `process_event` → return `EventDispatchResult`; map errors.
  - [x] 2.6 Implement `send_notification`: enforce policy + require `NotificationDeliveryPort` → build `Notification` → `deliver_notification` → return `Uuid`; map errors.
  - [x] 2.7 Declare `pub mod orchestrator_bridge;` and re-export `OrchestratorBridgeAdapter` in `src/application/services/orchestration/mod.rs`.
  - [x] 2.8 Unit tests for all four actions: success path + `ActionNotAllowed` + `QuotaExceeded`, using mock collaborators (mock orchestrator port or mock notification port).
  - [x] 2.9 Quality gate, then commit.

- [ ] 3.0 Wire the bridge into `PaladinExecutionService` (Task 4.2; FR 19–21)
  - [ ] 3.1 Add `orchestrator_port: Option<Arc<dyn OrchestratorPort>>` field; initialize to `None` in existing `new(...)` so current call sites keep compiling.
  - [ ] 3.2 Add backward-compatible `with_orchestrator_port(...)` setter (builder-style) and rustdoc.
  - [ ] 3.3 Confirm no behavioral change when `None` (existing tests still pass); add a small unit test asserting attach/detach wiring.
  - [ ] 3.4 Quality gate, then commit.

- [ ] 4.0 Integration test — agent triggers orchestration (Task 4.3; FR 23–24)
  - [ ] 4.1 Create `tests/agent_orchestrator_bridge.rs`; build a real `Orchestrator` + `OrchestratorBridgeAdapter` + `PaladinExecutionService` with a scripted mock LLM emitting a tool/function call for `schedule_job`.
  - [ ] 4.2 Execute the agent; assert the scheduled job is observable in the orchestrator scheduler state (deterministic, no network).
  - [ ] 4.3 Add an assertion that policy is honored (e.g., a disallowed action path returns the typed error) if cheaply expressible at integration level; otherwise rely on unit coverage.
  - [ ] 4.4 Quality gate, then commit.

- [ ] 5.0 Final verification & Epic close-out (FR 25)
  - [ ] 5.1 Run full quality gate: `cargo build`, `cargo test`, `cargo clippy -- -D warnings`, `cargo fmt --check`.
  - [ ] 5.2 Run `snyk_code_scan` on new first-party code; fix and rescan until clean.
  - [ ] 5.3 Mark PRD Task Checklist items complete; ensure "Relevant Files" above is accurate.
  - [ ] 5.4 Final commit if any cleanup remains.
