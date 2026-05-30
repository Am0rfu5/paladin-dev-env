# PRD: Orchestrator End-to-End Workflow Execution

**Epic:** 1 — Orchestrator End-to-End Workflow Execution
**Milestone:** 9 — Classic Orchestrator, Content Pipeline, and Agent-Orchestrator Bridge
**Version Target:** v0.3.0
**Priority:** Critical
**Status:** Ready for Implementation
**Created:** 2026-05-30
**Document Version:** 1.0

---

## 1. Introduction / Overview

The Paladin framework was designed as a dual-purpose platform: a classic content orchestration
system and an AI agent orchestration framework. Milestones 4–8 built and refined the AI agent side.
The classic orchestration subsystem — `Orchestrator`, `SchedulerOrchestrator`,
`ListenerOrchestrator`, `QueueService`, and the workflow engine — was placed structurally in earlier
milestones but is operationally incomplete.

The `Orchestrator` (now located at `src/application/services/orchestration/mod.rs` after the
Milestone 8 `use_cases → services` rename) has the right method surface: `start()`, `schedule_job()`,
`queue_job()`, `process_content()`, `create_workflow()`, `process_event()`, `execute_job()`. But the
**workflow execution loop does not exist**. In `create_workflow()`, every `WorkflowExecutionOrder`
arm (`Sequential`, `Parallel`, `EventDriven`, `Custom`) is a `println!` placeholder that stores the
workflow without ever executing its jobs. There is no method that walks a stored workflow's jobs,
dispatches them through the registered `TaskService` implementations, tracks their state, and marks
the workflow complete.

**This Epic makes workflows actually run.** It implements a real `execute_workflow()` method
covering all four execution orders, wires the `ScheduledJob → TaskService::execute()` dispatch path
to honor each job's error strategy, persists workflow state through a new `WorkflowRepository` port
so workflows survive restarts, and proves the whole thing with a full-lifecycle integration test.

**Problem being solved:** Today a developer can define and store a workflow, but starting the
orchestrator never causes that workflow's jobs to execute in the declared order with observable,
testable results. This Epic closes that gap and is the foundation that Epics 2, 3, and 4 build upon.

---

## 2. Goals

1. Implement a functional `execute_workflow()` method on `Orchestrator` that executes a stored
   workflow according to its `WorkflowExecutionOrder` (`Sequential`, `Parallel`, `EventDriven`,
   `Custom`/staged).
2. For sequential execution, thread the output of job N into the execution context consumed by job
   N+1.
3. For parallel execution, run all jobs concurrently and aggregate their results without dropping
   sibling failures.
4. For custom/staged execution, run stages in declared order and run the jobs within a stage
   concurrently.
5. Wire the full `ScheduledJob → TaskService::execute()` dispatch path: resolve the service by name,
   execute the job's tasks, collect results, and apply the job's error strategy.
6. Replace the placeholder default `TaskService` implementations (`DataBackupService`,
   `ContentIndexingService`, `EmailNotificationService`) with real, observable logic.
7. Track job and workflow state transitions internally (orchestrator-private, not public API) so
   they are observable to tests and persistable.
8. Persist workflow state through a new `WorkflowRepository` port with a SQLite-backed adapter, and
   recover incomplete workflows on `Orchestrator::start()`.
9. Prove the full lifecycle (create → start → execute → complete → read results) with an integration
   test.

---

## 3. User Stories

- **As a framework user**, I want to define a workflow of sequential jobs and have the orchestrator
  execute them in order so that each job can build on the previous job's output.
- **As a framework user**, I want a parallel workflow to run all of its jobs concurrently and give
  me every result (including which jobs failed) so I can process independent work efficiently.
- **As a framework user**, I want a staged (custom) workflow to run stages in order, with jobs
  inside a stage running together, so I can model fan-out/fan-in pipelines.
- **As an operator**, I want in-flight workflow state to survive an orchestrator restart so that a
  crash does not silently lose or restart completed work.
- **As a developer building Epic 2 (scheduler/queue) and Epics 3–4 (bridges)**, I need a working
  `execute_workflow()` and a working `TaskService` dispatch path to build on.
- **As a reviewer**, I need observable job/workflow state transitions and a deterministic
  integration test so I can verify execution order and completion without relying on log scraping.

---

## 4. Functional Requirements

### Task 1.1 — Implement the Workflow Execution Loop

1. The developer **must** add an `execute_workflow(&self, workflow_id: Uuid) -> Result<(),
   OrchestratorError>` method (or equivalent signature returning a result handle) to `Orchestrator`
   that looks up the stored `Workflow` and executes it according to its `execution_order`.
2. Looking up a non-existent workflow **must** return `OrchestratorError::WorkflowNotFound(id)`
   (this variant already exists in `types.rs`).
3. **Sequential** (`WorkflowExecutionOrder::Sequential`): the developer **must** execute the
   workflow's `jobs` in `Vec` order, one at a time. The result/output of job N **must** be placed
   into the `OrchestrationContext` (or an equivalent per-workflow result map) so it is available to
   job N+1 before job N+1 executes. A test **must** assert that job N+1 observed job N's output.
4. **Parallel** (`WorkflowExecutionOrder::Parallel`): the developer **must** spawn all jobs
   concurrently (e.g., `tokio::task::JoinSet` or `futures::future::join_all`) and collect every
   job's result. A failure in one job **must not** silently cancel or drop the results of sibling
   jobs; all results (success and failure) **must** be aggregated.
5. **Custom / staged** (`WorkflowExecutionOrder::Custom(Vec<WorkflowStage>)`): the developer **must**
   execute stages in `Vec` order. Within a single stage, the jobs identified by `stage.job_ids`
   **must** execute concurrently. The next stage **must not** begin until all jobs in the current
   stage have reached a terminal state. (The `WorkflowStage` type already exposes `job_ids`,
   `dependencies`, and `execution_mode`.)
6. **Event-driven** (`WorkflowExecutionOrder::EventDriven`): the developer **must** ensure
   event-driven workflows register their listeners (the existing `create_workflow_listener` path)
   such that an incoming matching event causes the target job to execute via the same dispatch path
   used by the other execution orders. Full event-pipeline *validation* (firing events, edge cases)
   is owned by Epic 2; this Epic only requires that the wiring routes a triggered job through
   `execute_job`/dispatch rather than a `println!`.
7. The developer **must** replace the four `println!`-only arms in `create_workflow()` so that
   workflow creation no longer pretends to configure execution. Creation **may** remain separate
   from execution (i.e., `create_workflow()` stores the workflow; `execute_workflow()` runs it), but
   the placeholder log-only branches **must** be removed.
8. The developer **must** introduce internal (crate-private, non-`pub`) `WorkflowState` and
   `JobState` representations sufficient to record transitions: at minimum `Pending → Running →
   Completed | Failed` for jobs and `Pending → Running → Completed | Failed` for the workflow. These
   **must not** be added to the public API (see Non-Goals); they exist to make state observable to
   the orchestrator, its persistence layer, and tests.
9. Job and workflow state transitions **must** be recorded as they happen so that, after
   `execute_workflow()` returns, the terminal state and per-job results are retrievable by the
   orchestrator (e.g., via an internal results map keyed by workflow id, or via the persistence
   layer from Task 1.3).
10. The developer **must** add unit tests for each execution-order variant:
    - Sequential: asserts ordered execution and N→N+1 context threading.
    - Parallel: asserts concurrent execution and aggregation of a mixed success/failure set.
    - Custom/staged: asserts stage ordering with intra-stage concurrency.

### Task 1.2 — Wire TaskService Execution

11. The developer **must** implement the dispatch path so that executing a job resolves each task's
    target `TaskService` by name from the orchestrator's registered services
    (`task_services: HashMap<String, Box<dyn TaskService>>`) and invokes `TaskService::execute()`
    for the task's action, collecting the returned `Option<serde_json::Value>` results.
12. Resolving a task whose `service_name` is **not** registered **must** return a typed error
    (`OrchestratorError::ServiceError(..)` or a more specific existing variant). The dispatch path
    **must not** `panic!` or `unwrap()` on a missing service.
13. The developer **must** apply each job's **error strategy** using the job's existing execution
    semantics. Two strategies **must** be honored:
    - **Fail-fast:** the first failing task aborts the remaining tasks in that job and marks the job
      `Failed`.
    - **Continue-on-error:** a failing task is recorded, and remaining tasks in the job still
      execute; the job's terminal state reflects partial completion.
    The strategy **must** be derived from the existing `Job`/`JobExecutionMode` configuration (the
    `Job::execute(&services)` method and `JobExecutionMode` already exist in
    `paladin-core::platform::container::job`); the developer **must** reuse that mechanism rather
    than inventing a parallel one.
14. **Retry, backoff, and dead-letter behavior are explicitly out of scope for this Epic** (see
    Non-Goals). Only the per-job fail-fast vs. continue-on-error strategy is required here.
15. The developer **must** replace the placeholder logic in the three default `TaskService`
    implementations in `crates/paladin-core/src/platform/container/task.rs` with real, observable
    behavior:
    - `DataBackupService` — perform a real, verifiable backup operation against `backup_path`
      (e.g., copy/write files to the target path) and return a result describing what was backed up.
    - `ContentIndexingService` — perform a real indexing operation against `index_name` (e.g.,
      build and persist a simple index structure) and return a result describing the index.
    - `EmailNotificationService` — perform a real notification dispatch (e.g., send via the
      configured SMTP server, or write to a pluggable sink that the production system wires to a
      real transport) and return a result describing the delivery.
    Each implementation **must** remove its `tokio::time::sleep` simulation and `println!`
    "simulate ..." scaffolding, and **must** return a typed `TaskError` on failure rather than
    succeeding unconditionally.
16. Each rewritten default service **must** have unit tests that assert its observable side effect
    (e.g., backup file exists at `backup_path`, index artifact created, notification sink received
    the message) and that it returns a `TaskError` on a forced failure (e.g., unwritable path).

### Task 1.3 — Implement Workflow State Persistence

17. The developer **must** define a new `WorkflowRepository` port trait in the `paladin-ports`
    crate (output port). At minimum it **must** support: persisting/updating a workflow's execution
    state (workflow id, current stage/index, per-job state, job results, error history, terminal
    state), loading a single workflow's state by id, and listing incomplete (non-terminal) workflows
    for recovery.
18. The `WorkflowRepository` trait **must** be `Send + Sync` and use `#[async_trait]`, consistent
    with the other output ports in `paladin-ports`.
19. The developer **must** implement a SQLite-backed adapter for `WorkflowRepository`. It **may**
    build on the existing SQLite storage facilities used elsewhere in the workspace
    (`paladin-storage` / `SqliteStore`); the chosen location **must** follow the existing adapter
    placement convention for SQLite adapters.
20. The `Orchestrator` **must** be able to hold an optional `WorkflowRepository` (e.g.,
    `Option<Arc<dyn WorkflowRepository>>`) so that existing in-memory behavior remains the default
    and persistence is opt-in via construction/wiring. Constructing an `Orchestrator` without a
    repository **must** continue to work (in-memory only).
21. When a `WorkflowRepository` is configured, `execute_workflow()` **must** persist workflow and
    job state transitions as they occur (at minimum on each job terminal transition and on workflow
    terminal transition), sufficient to resume.
22. On `Orchestrator::start()`, when a `WorkflowRepository` is configured, the orchestrator **must**
    load incomplete workflows and resume them from their last persisted position (the next
    unfinished job in sequential mode, or the next unfinished stage in staged mode) rather than
    re-executing already-completed jobs.
23. The developer **must** add a crash-recovery test: persist a partially-completed workflow's
    state, construct a new `Orchestrator` backed by the same repository, call `start()`, and assert
    that execution resumes from the last completed job/stage and that the workflow reaches
    `Completed` without re-running already-completed jobs.

### Task 1.4 — Integration Test: Full Workflow Lifecycle

24. The developer **must** add an integration test under `tests/` (following the existing
    integration-test layout) that:
    - Registers mock `TaskService` implementations whose execution produces an **observable side
      effect** (e.g., pushes the job/task name onto a shared, synchronized `Vec` or increments
      ordered counters).
    - Creates a workflow with **3 sequential jobs**.
    - Starts the orchestrator and triggers execution of the workflow.
    - Asserts that all 3 jobs executed **in order** (verified via the observable side effect, not
      via stdout).
    - Asserts the workflow's terminal state is `Completed`.
    - Asserts the per-job results are retrievable after completion.
25. The integration test **must** be deterministic (no reliance on wall-clock timing or log
    scraping) and **must** pass in CI under `cargo test`.

---

## 5. Non-Goals (Out of Scope)

- **No scheduler tick-loop validation.** Verifying `next_run` computation and timed dispatch for
  `Schedule::Interval | Cron | Once` is **Epic 2**.
- **No queue/Redis integration validation.** `QueueService ↔ RedisQueueAdapter` correctness,
  enqueue/dequeue through `QueuePort`, retry, and dead-letter behavior are **Epic 2**.
- **No retry, backoff, or dead-letter logic in the orchestrator.** This Epic implements only the
  per-job fail-fast vs. continue-on-error strategy.
- **No event-pipeline validation.** Firing events, matching listeners, and edge cases (no match,
  multiple matches, rate limiting) are **Epic 2**. This Epic only routes a triggered event-driven
  job through the real dispatch path instead of a `println!`.
- **No content → agent bridge.** `PaladinContentProcessor` / `BattalionContentProcessor` are
  **Epic 3**. This Epic does not implement AI-agent-backed content processing.
- **No agent → orchestrator bridge.** That is **Epic 4**.
- **No new public `WorkflowState` / `JobState` API.** State tracking stays internal to the
  orchestrator (per decision 4C). No new public state enums are exported from `paladin-core` or the
  facade in this Epic.
- **No user/admin auth work.** That is **Epic 5**.

---

## 6. Design Considerations

### Affected modules

| Concern | Location |
|---|---|
| Orchestrator + execution loop | `src/application/services/orchestration/mod.rs` |
| Coordination error/stats/processor types | `src/application/services/orchestration/types.rs` |
| Scheduler orchestrator | `src/application/services/orchestration/scheduler.rs` |
| Workflow / stage / execution-order types | `crates/paladin-core/src/platform/container/workflow.rs` |
| Job + `JobExecutionMode` + error strategy | `crates/paladin-core/src/platform/container/job.rs` |
| `TaskService` trait + default services | `crates/paladin-core/src/platform/container/task.rs` |
| New `WorkflowRepository` port | `crates/paladin-ports/src/output/` |
| SQLite adapter for the port | per existing SQLite adapter convention (`paladin-storage`) |
| Integration test | `tests/` |

### State tracking (decision 4C — internal only)

`WorkflowState` and `JobState` are internal types. They may be represented with crate-private enums
or by reusing fields the persistence layer needs. They must be rich enough to (a) drive recovery and
(b) let tests assert terminal state and ordering, but they must **not** be added to the public
`paladin-core`/facade API in this Epic. Note that `paladin-ports` already defines a public
`JobStatus` enum for the scheduler port; reusing or mapping to that for persistence is acceptable,
but no new public workflow-state surface is introduced here.

### Execution-order semantics summary

```
Sequential : jobs[0] → jobs[1] → ... ; output(jobs[i]) injected into context before jobs[i+1]
Parallel   : spawn(all jobs) ; join_all ; aggregate every result (no sibling cancellation on failure)
Custom     : for stage in stages: spawn(stage.job_ids) ; await all ; then next stage
EventDriven: register listeners → matching event → target job dispatched via the real path
```

### Persistence shape (illustrative, not prescriptive)

The `WorkflowRepository` should be able to round-trip enough to resume:

```
workflow_id, terminal_state, current_index_or_stage,
per_job: { job_id, state, result_json, error }, error_history
```

---

## 7. Technical Considerations

- **Reuse `Job::execute(&services)`.** The `Job` aggregate already executes its tasks against a
  `HashMap<String, Box<dyn TaskService>>` and already encodes execution mode via `JobExecutionMode`.
  The orchestrator's existing `execute_job()` clones registered services into that map. The
  execution loop and error strategy (Task 1.2) **must** build on this existing mechanism rather than
  duplicating task dispatch.
- **Concurrency.** Prefer `tokio::task::JoinSet` for parallel and intra-stage execution so that
  individual job failures are collected rather than aborting the set. Ensure `Send` bounds hold for
  spawned work (services are `Send + Sync`).
- **Default-service side effects must be testable.** For `EmailNotificationService`, avoid hard
  dependence on a live SMTP server in unit tests; inject a sink/transport seam so tests assert
  delivery without network access, while production wiring can point at a real transport. Apply the
  same principle (injectable target/seam) to backup and indexing so their tests are hermetic.
- **Security (OWASP).** The new SQLite adapter **must** use parameterized queries / bound parameters
  (no string-formatted SQL) to avoid injection. Persisted job results may contain
  externally-influenced data — treat them as data, never interpolate into SQL or shell. File paths
  used by `DataBackupService` and the indexer **must** be validated/constrained to avoid path
  traversal outside the configured target directory.
- **No panics on the dispatch path.** Missing services, unwritable paths, and serialization
  failures **must** surface as typed errors, consistent with the existing `OrchestratorError` /
  `TaskError` patterns (`thiserror`, `#[from]` conversions).
- **Backward compatibility.** `Orchestrator::new()` and the existing in-memory behavior **must**
  keep working; persistence is additive and opt-in.

---

## 8. Success Metrics

Epic 1 is complete when **all** of the following are true:

- [ ] `execute_workflow()` exists and correctly executes Sequential, Parallel, and Custom/staged
      workflows; EventDriven workflows route triggered jobs through the real dispatch path.
- [ ] The four `println!`-only arms in `create_workflow()` are removed/replaced.
- [ ] Sequential execution threads job N output into job N+1 context (proven by a unit test).
- [ ] Parallel execution runs jobs concurrently and aggregates a mixed success/failure set (proven
      by a unit test).
- [ ] Custom/staged execution runs stages in order with intra-stage concurrency (proven by a unit
      test).
- [ ] `TaskService` dispatch resolves services by name, honors fail-fast vs. continue-on-error, and
      returns a typed error for an unregistered service (no panics).
- [ ] `DataBackupService`, `ContentIndexingService`, and `EmailNotificationService` perform real,
      observable work, return `TaskError` on failure, and have passing unit tests for both success
      and failure.
- [ ] A `WorkflowRepository` port exists in `paladin-ports` with a SQLite-backed adapter using
      parameterized queries.
- [ ] Workflow state persists and an interrupted workflow resumes on `start()` from the last
      completed job/stage without re-running completed jobs (proven by a crash-recovery test).
- [ ] A deterministic full-lifecycle integration test (3 sequential jobs → Completed → results
      readable) passes in CI.
- [ ] No new public `WorkflowState`/`JobState` API is introduced.
- [ ] `cargo build --workspace`, `cargo test --workspace`, `cargo clippy --workspace -- -D warnings`,
      and `cargo fmt --all -- --check` all pass.

---

## 9. Open Questions

1. **`execute_workflow()` return shape.** Should the method return `Result<(), OrchestratorError>`
   (with results retrieved separately from the orchestrator/repository) or return a
   `WorkflowExecutionResult` handle directly? Default assumption: return `Result<(), _>` and expose a
   separate getter, to keep parity with existing `schedule_job`/`execute_job` signatures.
2. **EventDriven minimal scope boundary.** This Epic wires triggered jobs into the real dispatch
   path but defers firing/matching/edge-case validation to Epic 2. Confirm that "wiring only, no
   firing tests" is the intended boundary.
3. **`EmailNotificationService` production transport.** The Epic requires a real dispatch with an
   injectable sink for tests. Which production transport should the default point at (e.g., reuse
   `paladin-notifications` channels), or is the seam alone sufficient for this Epic with the concrete
   transport deferred?
4. **Repository location.** Confirm the SQLite `WorkflowRepository` adapter should live in
   `paladin-storage` (alongside existing SQLite stores) versus a new module.

---

## Task Checklist

### Task 1.1 — Implement Workflow Execution Loop
- [ ] Add `execute_workflow()` with workflow lookup + `WorkflowNotFound` handling
- [ ] Implement Sequential execution with N→N+1 context threading
- [ ] Implement Parallel execution with `JoinSet`/`join_all` + full result aggregation
- [ ] Implement Custom/staged execution (stage ordering, intra-stage concurrency)
- [ ] Route EventDriven triggered jobs through the real dispatch path
- [ ] Remove the four `println!`-only arms in `create_workflow()`
- [ ] Add internal `WorkflowState`/`JobState` transition tracking (non-public)
- [ ] Unit tests: sequential, parallel (mixed success/failure), staged

### Task 1.2 — Wire TaskService Execution
- [ ] Implement service-by-name resolution + typed error on missing service
- [ ] Honor fail-fast vs. continue-on-error via existing `Job`/`JobExecutionMode`
- [ ] Rewrite `DataBackupService` with real backup logic + `TaskError` on failure
- [ ] Rewrite `ContentIndexingService` with real indexing logic + `TaskError` on failure
- [ ] Rewrite `EmailNotificationService` with real dispatch (injectable sink) + `TaskError`
- [ ] Unit tests for each default service (success side effect + forced failure)

### Task 1.3 — Implement Workflow State Persistence
- [ ] Define `WorkflowRepository` output port in `paladin-ports` (`#[async_trait]`, `Send + Sync`)
- [ ] Implement SQLite-backed adapter (parameterized queries only)
- [ ] Add optional repository to `Orchestrator` (in-memory remains default)
- [ ] Persist workflow/job state transitions during `execute_workflow()`
- [ ] Resume incomplete workflows on `start()` without re-running completed jobs
- [ ] Crash-recovery test

### Task 1.4 — Integration Test: Full Workflow Lifecycle
- [ ] Mock `TaskService` impls with observable side effects
- [ ] 3-sequential-job workflow: create → start → execute
- [ ] Assert ordered execution, `Completed` terminal state, retrievable results
- [ ] Deterministic; passes under `cargo test` in CI

### Quality Gate
- [ ] `cargo build --workspace`
- [ ] `cargo test --workspace`
- [ ] `cargo clippy --workspace -- -D warnings`
- [ ] `cargo fmt --all -- --check`
