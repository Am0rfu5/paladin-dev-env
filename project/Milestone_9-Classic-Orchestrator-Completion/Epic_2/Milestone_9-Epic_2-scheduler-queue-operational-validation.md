# Milestone 9 — Epic 2: Scheduler and Queue Operational Validation

**Project:** Paladin Framework
**Milestone:** 9 — Classic Orchestrator, Content Pipeline, and Agent-Orchestrator Bridge
**Epic:** 2 of 6
**Priority:** Critical
**Estimated Effort:** Medium
**Dependencies:** Epic 1 (functional `Orchestrator`)
**Status:** Planning

---

## Objective

Validate that the `SchedulerOrchestrator` tick loop dispatches due jobs on time, and that the
`QueueService` integrates with both the in-memory queue and the `RedisQueueAdapter` through the
`QueuePort` trait.

## Background

With Epic 1 making the `Orchestrator` execute workflows, the time- and event-driven entry points
must be proven reliable. The `SchedulerOrchestrator` owns the periodic tick loop that decides which
jobs are due; the `QueueService` and `RedisQueueAdapter` provide the durable, distributable job
transport. The `ListenerOrchestrator` connects external events to triggers and jobs. None of these
paths have been validated end-to-end.

## Scope

**In scope:**
- Scheduler tick-loop correctness for all `Schedule` variants.
- `QueueService` ↔ `RedisQueueAdapter` integration through `QueuePort`.
- Queue retry and dead-letter behavior.
- Event → trigger → job pipeline validation.

**Out of scope:**
- The workflow execution loop itself (Epic 1).
- Content processing and AI agent bridges (Epics 3 and 4).

---

## Tasks

### Task 2.1: Validate Scheduler Tick Loop

**Description:** The `SchedulerOrchestrator::start()` spawns a tokio task that should periodically
check `next_run` times and dispatch due jobs. Verify this loop:
- Calculates `next_run` correctly for `Schedule::Interval`, `Schedule::Cron`, `Schedule::Once`.
- Dispatches jobs whose `next_run` has passed.
- Updates `last_run`, `run_count`, and `next_run` after execution.
- Handles disabled jobs (skips them).

**Implementation notes:**
- Make the tick interval and the clock source injectable so tests can use a controllable/mocked
  clock instead of relying on wall-clock sleeps.
- For `Schedule::Once`, ensure the job does not re-fire after its single execution.
- For `Schedule::Cron`, verify behavior across DST/timezone boundaries if cron expressions are
  timezone-aware; otherwise document UTC assumption.

**Deliverables:**
- Scheduler tick loop verified functional.
- Unit tests for each schedule type.
- Integration test: schedule a job for 1 second from now → verify it executes.

**Acceptance criteria:**
- Each `Schedule` variant computes the correct `next_run`.
- Disabled jobs are skipped without advancing `run_count`.
- `last_run`, `run_count`, and `next_run` update correctly after each dispatch.

---

### Task 2.2: Validate QueueService ↔ RedisQueueAdapter Integration

**Description:** The `QueueService` is an in-memory queue. The `RedisQueueAdapter` implements
`QueuePort`. Validate that:
- `QueueService` can be backed by `RedisQueueAdapter` when the `redis-queue` feature is enabled.
- Enqueue/dequeue operations work through the port trait.
- Queue retry and dead-letter behavior function.
- Health checks report correct status.

**Implementation notes:**
- Use `testcontainers` to spin up an ephemeral Redis for integration tests.
- Exercise the `QueuePort` abstraction directly so both the in-memory and Redis adapters are tested
  against the same contract.
- Verify dead-letter routing after the configured max-retry count is exhausted.

**Deliverables:**
- Integration test with testcontainers for Redis.
- In-memory fallback verified when Redis is unavailable.
- Queue retry logic tested (enqueue → fail → retry → succeed or dead-letter).

**Acceptance criteria:**
- Enqueue/dequeue round-trips succeed through `QueuePort` for both adapters.
- A job that fails up to the retry limit lands in the dead-letter queue.
- Health checks accurately reflect queue/Redis availability.

---

### Task 2.3: Validate Event → Trigger → Job Pipeline

**Description:** The `ListenerOrchestrator` receives events, matches them to registered listeners,
creates triggers, and passes them to the `Orchestrator` for execution. Validate the full pipeline:
- Register a listener with a trigger condition.
- Fire an event that matches the condition.
- Verify a trigger is created.
- Verify the trigger is converted to a job and executed.

**Implementation notes:**
- Cover the negative path (no matching listener) and the fan-out path (multiple matching listeners)
  explicitly.
- If rate limiting exists on listeners, assert that throttled events do not create excess triggers.

**Deliverables:**
- Integration test for the event pipeline.
- Edge case tests: no matching listener, multiple matching listeners, rate-limited listener.

**Acceptance criteria:**
- A matching event produces exactly one trigger per matching listener.
- A non-matching event produces no trigger.
- A created trigger is converted to a job and executed via the Epic 1 dispatch path.

---

## Definition of Done

- Scheduler tick loop validated for all `Schedule` variants with deterministic tests.
- Queue integration validated for both in-memory and Redis adapters, including retry/dead-letter.
- Event → trigger → job pipeline validated, including edge cases.
- `cargo build`, `cargo test` (with `redis-queue` feature), `cargo clippy -- -D warnings`, and
  `cargo fmt --check` all pass.
