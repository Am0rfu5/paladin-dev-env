# PRD: Scheduler and Queue Operational Validation

**Epic:** 2 — Scheduler and Queue Operational Validation
**Milestone:** 9 — Classic Orchestrator, Content Pipeline, and Agent-Orchestrator Bridge
**Version Target:** v0.3.0
**Priority:** Critical
**Status:** Ready for Implementation
**Created:** 2026-05-30
**Document Version:** 1.0

---

## 1. Introduction / Overview

Epic 1 made the `Orchestrator` actually execute workflows: it implemented a real
`execute_workflow()` for all execution orders, wired the `ScheduledJob → TaskService::execute()`
dispatch path, and added workflow-state persistence with crash recovery. With workflows now
running, the **time-driven** and **event-driven** entry points that feed the orchestrator must be
proven reliable end-to-end.

Three subsystems supply those entry points and have never been validated together:

- **Scheduler** — two implementations exist:
  - `SchedulerOrchestrator` (application layer,
    `src/application/use_cases/orchestration/scheduler.rs`) owns a manual `tick_interval` loop. On
    each tick `check_and_execute_jobs()` finds jobs whose `next_run` has passed, dispatches them
    through the registered `TaskService` map, then updates `last_run`, `run_count`, and recomputes
    `next_run` via `calculate_next_run()` for the `Schedule::{Interval, Daily, Weekly, Monthly,
    Once, OnStartup}` variants. Disabled jobs and `Once`/`OnStartup` non-repeat behavior already
    exist but are not covered by deterministic tests of the dispatch path.
  - `TokioCronSchedulerAdapter` (infrastructure,
    `src/infrastructure/adapters/scheduling/tokio_cron_adapter.rs`) wraps `tokio-cron-scheduler`
    and implements the `SchedulerPort` trait for cron-expression scheduling. Its `schedule_job`,
    `cancel_job`, status, and lifecycle paths exist but cron firing is not validated.
- **Queue** — `QueueOrchestrator` (aliased `QueueService`,
  `src/application/services/queue_orchestrator/mod.rs`) provides in-process FIFO/priority queues
  with `fail_processing` retry and a `failed_items` dead-letter map. `RedisQueueAdapter`
  (`src/infrastructure/adapters/queue/redis.rs`, behind the `redis-queue` feature) implements the
  `QueuePort` family (`QueuePort`, `BatchQueuePort`, `PriorityQueuePort`, `QueueManagementPort`)
  with retry-up-to-max-then-dead-letter behavior and a `health_check`. The two adapters have never
  been exercised against the **same `QueuePort` contract**, and the Redis path has no integration
  coverage.
- **Event pipeline** — `ListenerOrchestrator`
  (`src/application/services/orchestration/listener.rs`) registers `EventListener`s, matches events
  to `TriggerCondition`s, applies per-listener rate limiting (`max_triggers_per_window`), creates
  `Trigger`s, and enqueues them. The `Orchestrator` exposes `register_event_listener()` and
  `process_event()`. The full event → trigger → job path is not covered by tests, including the
  negative (no match) and fan-out (multiple matches) cases.

**Problem being solved:** The orchestrator can run a workflow when something tells it to, but the
three things that *tell it to* — the periodic scheduler tick, the durable queue transport, and the
event listener pipeline — are unproven. This Epic validates each path with deterministic,
repeatable tests, closes any small behavior gaps discovered (e.g. in-memory dead-letter parity),
and leaves the time/event entry points trustworthy for Epics 3 and 4.

---

## 2. Goals

1. Validate the `SchedulerOrchestrator` tick loop: each `Schedule` variant computes a correct
   `next_run`; due jobs dispatch through `TaskService::execute()`; `last_run`, `run_count`, and
   `next_run` update correctly; disabled jobs are skipped without advancing `run_count`; `Once` and
   `OnStartup` jobs do not re-fire.
2. Validate the `TokioCronSchedulerAdapter`: a cron-scheduled job fires on schedule, invalid cron
   expressions are rejected, and lifecycle (start / schedule / cancel / shutdown) behaves
   correctly.
3. Validate the `QueuePort` contract against **both** the in-memory `QueueOrchestrator` and the
   `RedisQueueAdapter`: enqueue/dequeue round-trips, processing lifecycle
   (`start_processing`/`complete_processing`/`fail_processing`), and health checks.
4. Validate queue retry and dead-letter behavior: an item that fails up to its `max_retries` is
   re-queued for retry, and once retries are exhausted it lands in the dead-letter (failed) store —
   for both adapters. Add minimal dead-letter behavior to the in-memory queue only if it lacks
   parity with the contract.
5. Validate the event → trigger → job pipeline: a matching event produces exactly one trigger per
   matching listener; a non-matching event produces none; a created trigger is converted to a job
   and executed via the Epic 1 dispatch path. Cover rate-limited listeners.
6. Keep all production behavior changes minimal and test-driven — no clock abstraction or scheduler
   refactor; validation uses the existing public/in-module APIs with short, deterministic
   intervals.
7. Ensure `cargo build`, `cargo test` (including the `redis-queue` feature), `cargo clippy -- -D
   warnings`, and `cargo fmt --check` all pass.

---

## 3. User Stories

- **As an operator**, I want a job scheduled for a near-future time to actually fire on time so that
  recurring maintenance and content jobs run without manual intervention.
- **As an operator**, I want disabled jobs to be skipped and one-time jobs to fire exactly once so
  that pausing a job or scheduling a single run behaves predictably.
- **As a developer**, I want the in-memory queue and the Redis queue to behave identically through
  the `QueuePort` trait so that I can swap transports without changing call sites.
- **As an operator**, I want a job that keeps failing to be retried a bounded number of times and
  then moved to a dead-letter store so that poison messages neither retry forever nor disappear
  silently.
- **As a framework user**, I want an external event that matches a registered listener to create a
  trigger and run the associated job so that event-driven workflows fire reliably.
- **As a reviewer**, I want deterministic, non-flaky tests for the scheduler, queue, and event
  pipeline so that I can trust these paths without scraping logs.

---

## 4. Functional Requirements

### Task 2.1 — Validate the Scheduler Tick Loop and Cron Adapter

1. The developer **must** add unit tests verifying `SchedulerOrchestrator::calculate_next_run`
   returns a correct `next_run` for each `Schedule` variant: `Interval`, `Daily`, `Weekly`,
   `Monthly`, `Once` (future → `Some`, past → `None`), and `OnStartup` (`None`). (Treat the Epic
   doc's "Cron" as these recurring variants; the cron-expression path is validated via the Tokio
   adapter in requirement 8.)
2. The developer **must** add a deterministic test of `check_and_execute_jobs()` (callable from the
   in-module test scope) that registers a `TaskService` test double, adds a job whose `next_run`
   is already in the past, runs one tick, and asserts the service was invoked exactly once.
3. After a dispatch, the developer **must** assert that the executed job's `last_run` is set,
   `run_count` incremented by one, and `next_run` recomputed (non-`None` for recurring variants;
   `None` for `Once`/`OnStartup`).
4. The developer **must** assert that a **disabled** job is skipped: after a tick, its `run_count`
   does not advance and its service is not invoked.
5. The developer **must** assert that a `Schedule::Once` job whose time has passed runs **once** and
   then has `next_run == None`, so it does not re-fire on the next tick.
6. The developer **must** add an integration-style test (under `tests/`, or an in-module
   `#[tokio::test]`) that schedules a job a short interval in the future, drives the scheduler, and
   verifies the job executes. The test **must** use a short interval (sub-second to a few seconds)
   and **must not** depend on a real clock abstraction (per the test-only, no-production-refactor
   decision).
7. The developer **must not** add a clock abstraction or otherwise refactor production scheduler
   code to make these tests pass. Tests rely on the existing public API plus in-module access to
   `calculate_next_run`/`check_and_execute_jobs`.
8. The developer **must** add tests for `TokioCronSchedulerAdapter` verifying: (a) a cron job
   scheduled to fire imminently actually fires (observable via a shared counter/flag the job
   closure increments); (b) an invalid cron expression returns
   `SchedulerError::InvalidCronExpression`; (c) scheduling while not running returns
   `SchedulerError::NotRunning`; and (d) `start → schedule → cancel/shutdown` completes without
   error. The UTC assumption for cron evaluation **must** be documented in the test or adapter doc
   comment.

### Task 2.2 — Validate QueueService ↔ RedisQueueAdapter via the QueuePort Contract

9. The developer **must** add a reusable contract test (a helper or macro parameterized over a
   `QueuePort` implementation) that exercises: `create_queue`, `enqueue`, `dequeue` (round-trip
   preserves payload), `start_processing`, `complete_processing`, `queue_length`, and
   `health_check`.
10. The developer **must** run that contract against the in-memory `QueueOrchestrator`/`QueueService`
    as a default (always-on) test.
11. The developer **must** run that contract against the `RedisQueueAdapter`, gated behind the
    `redis-queue` feature. The Redis test **must** connect to the Redis provided by the existing
    docker-compose test stack (e.g. via a `REDIS_URL`/host/port from config or environment) and
    **must** skip gracefully (or be marked `#[ignore]`) when Redis is not reachable, so the default
    `cargo test` run is not broken on machines without Redis.
12. The developer **must** validate retry behavior: configure an item with `max_retries = N`, call
    `fail_processing` and assert it reports the item was re-queued for retry while
    `attempt_count < max_retries`.
13. The developer **must** validate dead-letter behavior: after `max_retries` is exhausted,
    `fail_processing` reports no further retry and the item is moved to the failed/dead-letter store
    (`failed_items` for the in-memory queue; the `failed` hash for Redis), observable via stats or a
    getter. This **must** hold for both adapters.
14. If the in-memory `QueueOrchestrator` is found to lack dead-letter parity with the `QueuePort`
    contract (it currently preserves failed items in `failed_items` when `preserve_failed` is set),
    the developer **must** add the minimal behavior needed to satisfy requirement 13 — and **only**
    that. No retry/back-off/dead-letter redesign is in scope.
15. The developer **must** validate that the in-memory queue is the working fallback when Redis is
    unavailable: the same call sites succeed against `QueueOrchestrator`, and the health check
    accurately reflects availability for each adapter.

### Task 2.3 — Validate the Event → Trigger → Job Pipeline

16. The developer **must** add a test that registers an `EventListener` with a `TriggerCondition`,
    fires a **matching** event through the `Orchestrator`/`ListenerOrchestrator`, and asserts that
    exactly one `Trigger` is created.
17. The developer **must** add a test that fires a **non-matching** event and asserts that **no**
    trigger is created.
18. The developer **must** add a fan-out test: with multiple listeners whose conditions all match a
    single event, assert that exactly one trigger is created **per matching listener**.
19. The developer **must** assert that a created trigger is converted to a job and executed via the
    Epic 1 dispatch path (i.e. through `execute_job`/`TaskService::execute`), observable via a
    `TaskService` test double recording the execution.
20. The developer **must** add a rate-limit test: when a listener's `max_triggers_per_window` is
    exceeded, throttled events **must not** create excess triggers (assert the trigger count is
    capped at the window limit).
21. Where the existing event path requires small glue to route a created trigger into the dispatch
    path, the developer **may** add that minimal wiring only; building a new listener subsystem is
    out of scope.

### Cross-Cutting Requirements

22. All new tests **must** be deterministic and non-flaky: prefer in-process drives, short bounded
    waits, and shared counters/flags over sleeps tied to wall-clock thresholds; any timing-based
    test **must** use generous-enough bounds to avoid CI flakiness.
23. All public items added (if any) **must** have rustdoc comments; the codebase's
    `missing_docs`/clippy settings **must** continue to pass.
24. The developer **must** run `cargo test` with the `redis-queue` feature enabled at least once and
    confirm the Redis contract test passes against the docker-compose Redis (or is correctly
    skipped/ignored when Redis is absent).

---

## 5. Non-Goals (Out of Scope)

1. **No clock abstraction or scheduler production refactor.** Validation is test-only; the manual
   tick loop and `calculate_next_run` are exercised as-is.
2. **No new retry/back-off/dead-letter design.** Existing retry and dead-letter behavior is
   validated; only the minimal change needed to give the in-memory queue contract parity is
   permitted.
3. **No `testcontainers` dependency.** Redis integration uses the existing docker-compose test
   stack.
4. **No new transports.** RabbitMQ, SQS, or other `QueuePort` adapters are not in scope.
5. **No new event/listener subsystem.** Only the existing `ListenerOrchestrator` path is validated,
   with minimal glue if needed.
6. **No workflow execution-loop changes.** That is Epic 1 and is already complete.
7. **No content-processing or AI-agent bridges.** Those are Epics 3 and 4.
8. **No public API surface expansion** beyond what is strictly required for observability of the
   validated behavior.

---

## 6. Design Considerations

- **Scheduler tests** live in the existing `#[cfg(test)] mod tests` of
  `src/application/use_cases/orchestration/scheduler.rs` (for in-module access to
  `calculate_next_run`/`check_and_execute_jobs`) and, for the tokio-cron adapter, in the existing
  test module of `tokio_cron_adapter.rs`.
- **Queue contract tests** should be written once and parameterized over a `&dyn QueuePort` (or a
  generic) so the in-memory and Redis adapters are tested against an identical sequence of
  operations. Place the shared helper where both can import it (e.g. a `tests/` integration module
  or a `#[cfg(test)]` helper).
- **Redis tests** read connection details from the test configuration/environment used by the
  docker-compose test stack (`docker/docker-compose.test.yml`). Guard with
  `#[cfg(feature = "redis-queue")]` and a reachability check.
- **Event-pipeline tests** belong under `tests/integration/` alongside the Epic 1 lifecycle test,
  using `TaskService` and `EventListener` test doubles.

---

## 7. Technical Considerations

- Test doubles follow the Epic 1 pattern: a `RecordingService`/`FakeService` implementing
  `TaskService` that records invocations into an `Arc<Mutex<…>>`.
- The Redis adapter is behind the `redis-queue` feature flag; tests that touch it must be
  feature-gated and must not break the default `cargo test`.
- Cron evaluation in `tokio-cron-scheduler` is UTC-based; document this assumption rather than
  attempting DST-aware tests.
- Reuse existing error types (`SchedulerError`, `QueueError`, `ListenerError`,
  `OrchestratorError`); do not introduce new error enums unless a gap is found.
- Honor hexagonal boundaries: tests in the application layer must not reach into infrastructure
  except through ports; Redis-specific tests live with/near the infrastructure adapter.

---

## 8. Success Metrics

- [ ] Each `Schedule` variant computes the correct `next_run` (unit tests green).
- [ ] A due job dispatches through `TaskService::execute()` on a tick, and `last_run` / `run_count`
      / `next_run` update correctly.
- [ ] Disabled jobs are skipped without advancing `run_count`.
- [ ] A `Schedule::Once` job fires exactly once and does not re-fire.
- [ ] A job scheduled a short interval in the future executes (scheduler integration test green).
- [ ] `TokioCronSchedulerAdapter`: a cron job fires; invalid cron is rejected; not-running and
      lifecycle paths behave correctly.
- [ ] The `QueuePort` contract passes against the in-memory `QueueOrchestrator`.
- [ ] The `QueuePort` contract passes against the `RedisQueueAdapter` with the `redis-queue` feature
      (against docker-compose Redis, or correctly skipped when absent).
- [ ] An item failing up to `max_retries` is retried, then dead-lettered — for both adapters.
- [ ] In-memory queue is verified as the working fallback; health checks reflect availability.
- [ ] A matching event creates exactly one trigger per matching listener; a non-matching event
      creates none; the trigger runs a job via the Epic 1 dispatch path.
- [ ] A rate-limited listener does not create excess triggers.
- [ ] `cargo build`, `cargo test` (incl. `redis-queue`), `cargo clippy -- -D warnings`, and `cargo
      fmt --check` all pass.

## Task Checklist

- [ ] **Task 2.1** — Scheduler tick loop + cron adapter validated.
- [ ] **Task 2.2** — `QueuePort` contract validated for in-memory and Redis, incl. retry/dead-letter.
- [ ] **Task 2.3** — Event → trigger → job pipeline validated, incl. edge cases.
- [ ] **Quality Gate** — build, test (with `redis-queue`), clippy, and fmt all pass.

---

## 9. Open Questions

1. Does the existing docker-compose test stack expose Redis on a well-known host/port that the test
   config already points to, or does the Redis contract test need a new env var? (Resolve during
   Task 2.2 by inspecting `docker/docker-compose.test.yml` and the test config.)
2. Is there existing glue from `ListenerOrchestrator`'s `trigger_queue` into `execute_job`, or must
   a created trigger be manually pulled and dispatched in the test? (Resolve during Task 2.3 by
   inspecting the listener → orchestrator path.)
