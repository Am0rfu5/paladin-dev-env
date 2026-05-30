# Tasks: Orchestrator End-to-End Workflow Execution

**PRD:** `prd-orchestrator-end-to-end-workflow-execution.md`
**Epic:** 1 — Milestone 9
**Target:** v0.3.0

## Relevant Files

- `src/application/services/orchestration/mod.rs` - `Orchestrator`; add `execute_workflow()`, dispatch path, persistence wiring, recovery on `start()`.
- `src/application/services/orchestration/types.rs` - Coordination types; add internal `WorkflowState`/`JobState` + execution-result structures.
- `src/application/services/orchestration/scheduler.rs` - `SchedulerOrchestrator` (referenced; minimal/no change expected).
- `crates/paladin-core/src/platform/container/workflow.rs` - `Workflow`, `WorkflowStage`, `WorkflowExecutionOrder` (read; possibly helper accessors).
- `crates/paladin-core/src/platform/container/job.rs` - `Job::execute`, `JobExecutionMode` (reused for error strategy).
- `crates/paladin-core/src/platform/container/task.rs` - `TaskService` trait + `DataBackupService`/`ContentIndexingService`/`EmailNotificationService` rewrites + unit tests.
- `crates/paladin-ports/src/output/workflow_repository_port.rs` - **New** `WorkflowRepositoryPort` trait.
- `crates/paladin-ports/src/output/mod.rs` - Register the new port module.
- `crates/paladin-storage/src/sqlite_workflow_repository.rs` - **New** SQLite adapter implementing the port.
- `crates/paladin-storage/src/lib.rs` - Register the new adapter module.
- `tests/integration/orchestrator_workflow_lifecycle_test.rs` - **New** full-lifecycle integration test.
- `tests/integration.rs` (or `tests/integration/mod.rs`) - Wire the new integration test if required by harness.

### Notes

- Unit tests live in `#[cfg(test)]` modules beside the code (services in `task.rs`, execution loop in `mod.rs`).
- Integration tests live under `tests/`.
- `Job::execute(&services)` already implements per-job error strategy via `JobExecutionMode` (`Sequential` = fail-fast, `SequentialContinueOnError` = continue-on-error) — reuse it; do not duplicate task dispatch.
- SQLite adapters use `sqlx` with bound parameters (see `sqlite_user_repository.rs`).
- Run `cargo test`, `cargo fmt --check`, `cargo clippy -- -D warnings` before committing each parent task.

## Tasks

- [x] 0.0 Create feature branch
  - [x] 0.1 Create and checkout `feature/milestone_9-epic_1-orchestrator-workflow-execution`

- [x] 1.0 Implement the workflow execution loop (PRD Task 1.1)
  - [x] 1.1 Add internal (crate-private) `WorkflowRunState`/`JobRunState` enums and a `WorkflowExecutionResult`/per-job result structure in `types.rs` (non-public API)
  - [x] 1.2 Add an internal results store to `Orchestrator` (e.g., `Arc<RwLock<HashMap<Uuid, WorkflowExecutionResult>>>`) plus a getter for tests
  - [x] 1.3 Add `execute_workflow(&self, workflow_id: Uuid)` with workflow lookup + `WorkflowNotFound` handling
  - [x] 1.4 Implement Sequential execution: run jobs in order, thread job N output into the context/result consumed by job N+1
  - [x] 1.5 Implement Parallel execution: spawn all jobs via `JoinSet`, aggregate every result (no sibling cancellation on failure)
  - [x] 1.6 Implement Custom/staged execution: stages in order, jobs within a stage concurrent, barrier between stages
  - [x] 1.7 Route EventDriven triggered jobs through the real dispatch path (no `println!` placeholder)
  - [x] 1.8 Remove the four `println!`-only arms in `create_workflow()`
  - [x] 1.9 Record job/workflow state transitions into the internal results store as they occur
  - [x] 1.10 Unit tests: sequential ordering + N→N+1 threading; parallel mixed success/failure aggregation; staged ordering with intra-stage concurrency

- [ ] 2.0 Wire TaskService execution + error strategy (PRD Task 1.2, dispatch)
  - [ ] 2.1 Implement a job-dispatch helper that resolves services by name from `task_services` and runs the job via `Job::execute(&services)`
  - [ ] 2.2 Return a typed error (no panic/unwrap) when a task's `service_name` is unregistered
  - [ ] 2.3 Honor fail-fast vs. continue-on-error by mapping to the job's existing `JobExecutionMode`
  - [ ] 2.4 Unit test: unregistered service yields typed error; fail-fast vs. continue-on-error behavior verified

- [ ] 3.0 Rewrite default TaskService implementations (PRD Task 1.2, services)
  - [ ] 3.1 Rewrite `DataBackupService` to perform a real, path-constrained backup write and return a descriptive result; `TaskError` on failure
  - [ ] 3.2 Rewrite `ContentIndexingService` to build/persist a simple index artifact and return a descriptive result; `TaskError` on failure
  - [ ] 3.3 Rewrite `EmailNotificationService` to dispatch via an injectable sink/transport seam and return a descriptive result; `TaskError` on failure
  - [ ] 3.4 Remove `tokio::time::sleep` simulations and `println!` "simulate..." scaffolding from all three
  - [ ] 3.5 Unit tests for each service: success side effect + forced-failure `TaskError`
  - [ ] 3.6 Fix any existing tests/examples that relied on the old simulated behavior

- [ ] 4.0 Workflow state persistence (PRD Task 1.3)
  - [ ] 4.1 Define `WorkflowRepositoryPort` in `paladin-ports/src/output/workflow_repository_port.rs` (`#[async_trait]`, `Send + Sync`): save/update state, load by id, list incomplete
  - [ ] 4.2 Register the port module in `paladin-ports/src/output/mod.rs`
  - [ ] 4.3 Implement `SqliteWorkflowRepository` in `paladin-storage` with `sqlx` + bound parameters + migration
  - [ ] 4.4 Register the adapter module in `paladin-storage/src/lib.rs`
  - [ ] 4.5 Add optional `Option<Arc<dyn WorkflowRepositoryPort>>` to `Orchestrator`; keep `new()` in-memory-only working
  - [ ] 4.6 Persist workflow/job state transitions during `execute_workflow()` when a repository is configured
  - [ ] 4.7 On `start()`, load incomplete workflows and resume from last completed job/stage without re-running completed jobs
  - [ ] 4.8 Unit/integration test: crash-recovery resumes correctly to `Completed`

- [ ] 5.0 Full-lifecycle integration test (PRD Task 1.4)
  - [ ] 5.1 Add mock `TaskService` impls with observable, synchronized side effects (ordered record)
  - [ ] 5.2 Create a 3-sequential-job workflow; start orchestrator; execute it
  - [ ] 5.3 Assert ordered execution, `Completed` terminal state, retrievable per-job results; ensure determinism
  - [ ] 5.4 Wire the test into the integration harness so it runs under `cargo test`

- [ ] 6.0 Quality gate & finalize
  - [ ] 6.1 `cargo build --workspace`
  - [ ] 6.2 `cargo test --workspace`
  - [ ] 6.3 `cargo clippy --workspace -- -D warnings`
  - [ ] 6.4 `cargo fmt --all -- --check`
  - [ ] 6.5 Remove temporary debug prints; update PRD checklist; update Relevant Files
