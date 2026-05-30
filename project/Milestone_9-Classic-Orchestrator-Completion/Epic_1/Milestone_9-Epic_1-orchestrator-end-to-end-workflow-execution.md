# Milestone 9 — Epic 1: Orchestrator End-to-End Workflow Execution

**Project:** Paladin Framework
**Milestone:** 9 — Classic Orchestrator, Content Pipeline, and Agent-Orchestrator Bridge
**Epic:** 1 of 6
**Priority:** Critical
**Estimated Effort:** Large
**Dependencies:** Milestone 8 complete (facade cleanup; directory structure stable)
**Status:** Planning

---

## Objective

Replace the scaffold logic in the `Orchestrator` with functional dispatch. A workflow with
sequential, parallel, or event-driven execution order runs from creation to completion with
tracked state.

## Background

The current `Orchestrator` code (in `src/application/services/orchestration/mod.rs` post-Milestone 8
rename) stores workflows and dispatches jobs, but the actual execution loop — pick next job from
queue → execute via task service → handle result → advance workflow state — uses `println!`
statements rather than real dispatch. The `execute_trigger()` method converts triggers to jobs but
doesn't execute them. The `process_content()` method calls `processor.process_content()` but the
default `ContentProcessor` implementations are stubs.

This Epic is the foundation for the rest of Milestone 9. Epics 2, 3, and 4 all depend on a
functional `Orchestrator`, so this work gates the parallelizable downstream Epics.

## Scope

**In scope:**
- The workflow execution loop for all four execution orders (sequential, parallel, event-driven,
  custom/staged).
- The dispatch path from `ScheduledJob` → `TaskService::execute()`.
- Workflow state persistence and recovery.
- Job state machine with observable transitions.

**Out of scope:**
- Scheduler tick-loop validation (Epic 2).
- Queue/Redis integration validation (Epic 2).
- Content → Agent bridging (Epic 3).
- Agent → Orchestrator bridging (Epic 4).

---

## Tasks

### Task 1.1: Implement Workflow Execution Loop

**Description:** Implement the core execution loop for `Orchestrator`:
- **Sequential:** execute jobs in order, passing output of job N as context to job N+1.
- **Parallel:** spawn all jobs concurrently, collect results.
- **Event-driven:** register listeners that trigger jobs when events arrive.
- **Custom (staged):** execute stages in order, jobs within a stage in parallel.

Track job state transitions: Pending → Running → Completed/Failed. Store results in the
`OrchestrationContext`.

**Implementation notes:**
- Use `tokio::spawn` + `JoinSet` (or `futures::future::join_all`) for the parallel and staged
  variants so failures in one job do not silently drop sibling jobs.
- The sequential variant must thread the previous job's output into the next job's
  `OrchestrationContext` so downstream jobs can consume upstream results.
- Define an explicit `WorkflowState` enum (`Pending`, `Running`, `Completed`, `Failed`, `Cancelled`)
  and an explicit `JobState` enum so transitions are observable and testable.

**Deliverables:**
- Functional `execute_workflow()` method covering all four execution orders.
- Job state machine with observable transitions.
- Unit tests for each execution-order variant (including a mixed success/failure case for parallel
  and staged).

**Acceptance criteria:**
- A sequential workflow passes job N output into job N+1 context (asserted in a test).
- A parallel workflow runs all jobs concurrently and aggregates all results.
- A staged workflow runs stages in order and jobs within a stage in parallel.
- Job and workflow state transitions are recorded and queryable.

---

### Task 1.2: Wire TaskService Execution

**Description:** The `Orchestrator` registers `TaskService` implementations but the dispatch path
from `ScheduledJob` → `TaskService::execute()` is incomplete. Implement the full dispatch: resolve
the service by name, pass the job's tasks to the service, collect results, handle errors per the
job's error strategy.

**Implementation notes:**
- Resolve services from a registry keyed by service name; return a typed error if the named service
  is not registered (do not panic).
- Honor the job's error strategy (fail-fast vs. continue-on-error) and surface partial results.
- Replace placeholder default services with meaningful implementations, or clearly mark them as
  reference/example services with real, observable side effects for testing.

**Deliverables:**
- `TaskService` dispatch functional.
- Default services (`DataBackupService`, `ContentIndexingService`, `EmailNotificationService`)
  execute real logic (or are replaced with meaningful implementations).
- Error handling and retry for failed tasks.

**Acceptance criteria:**
- Dispatching a job to a registered service executes its tasks and collects results.
- Dispatching to an unregistered service returns a typed error (no panic).
- A failing task triggers the configured error strategy (retry or fail per job config).

---

### Task 1.3: Implement Workflow State Persistence

**Description:** Workflows currently live in an in-memory `HashMap`. For production use, workflow
state (current stage, job results, error history) should persist across restarts. Implement via the
existing `SqliteStore` or a new `WorkflowRepository`.

**Implementation notes:**
- Define a `WorkflowRepository` port in `paladin-ports` if no suitable abstraction exists, with a
  SQLite-backed adapter.
- Persist enough state to resume: workflow definition reference, current stage/index, per-job state,
  job results, and error history.
- On `Orchestrator::start()`, load incomplete workflows and resume them from their last persisted
  state.

**Deliverables:**
- Workflow state persisted to SQLite.
- Workflow recovery on `Orchestrator::start()`.
- Tests for crash-recovery scenario.

**Acceptance criteria:**
- A workflow's state survives an `Orchestrator` restart.
- An interrupted workflow resumes from the last completed stage/job, not from the beginning.

---

### Task 1.4: Integration Test — Full Workflow Lifecycle

**Description:** Write an integration test that: creates a workflow with 3 sequential jobs → starts
the orchestrator → verifies all 3 jobs execute in order → verifies final workflow state is Completed
→ verifies results are accessible.

**Deliverables:**
- Integration test in `tests/integration/`.
- Test uses mock `TaskService` implementations with observable side effects.

**Acceptance criteria:**
- The test deterministically verifies execution order via observable side effects.
- The test asserts terminal workflow state is `Completed` and results are retrievable.

---

## Definition of Done

- All four execution orders implemented and unit tested.
- `TaskService` dispatch functional with error handling and retry.
- Workflow state persists and recovers across restarts.
- Full-lifecycle integration test passes.
- `cargo build`, `cargo test`, `cargo clippy -- -D warnings`, and `cargo fmt --check` all pass for
  the affected crates.
