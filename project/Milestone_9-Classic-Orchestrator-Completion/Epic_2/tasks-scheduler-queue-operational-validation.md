# Tasks: Scheduler and Queue Operational Validation

**PRD:** `prd-scheduler-queue-operational-validation.md`
**Epic:** 2 — Milestone 9
**Target:** v0.3.0

## Relevant Files

- `src/application/use_cases/orchestration/scheduler.rs` - `SchedulerOrchestrator`; in-module `#[cfg(test)]` tests for `calculate_next_run`, `check_and_execute_jobs` dispatch, disabled-job skip, and `Once` single-fire.
- `src/infrastructure/adapters/scheduling/tokio_cron_adapter.rs` - `TokioCronSchedulerAdapter`; in-module tests for cron firing, invalid cron, not-running, and lifecycle (UTC assumption documented).
- `src/application/services/queue_orchestrator/mod.rs` - In-memory `QueueOrchestrator`/`QueueService`; subject of the `QueuePort` contract + retry/dead-letter validation; minimal dead-letter parity change only if required.
- `src/application/services/queue_orchestrator/types.rs` - Queue internals (`fail_processing`, `failed_items`); referenced for dead-letter assertions.
- `src/infrastructure/adapters/queue/redis.rs` - `RedisQueueAdapter`; subject of the feature-gated `QueuePort` contract + retry/dead-letter validation against docker-compose Redis.
- `crates/paladin-ports/src/output/queue_port.rs` - `QueuePort` trait; defines the contract the shared test exercises.
- `src/application/services/orchestration/listener.rs` - `ListenerOrchestrator`, `EventListener`; subject of event → trigger → job validation incl. rate-limit and fan-out.
- `src/application/services/orchestration/mod.rs` - `Orchestrator`; `register_event_listener()`/`process_event()` and the trigger → `execute_job` dispatch path used by event tests.
- `tests/integration/scheduler_queue_event_validation_test.rs` - **New** integration tests: scheduler fire-on-time, `QueuePort` contract (in-memory + feature-gated Redis), event → trigger → job pipeline.
- `tests/queue_port_contract.rs` - **New** integration tests validating the queue contract (enqueue → dequeue → process lifecycle, retry, dead-letter) for the in-memory `QueueOrchestrator` (always-on) and the feature-gated `RedisQueueAdapter` (skips when Redis unreachable).
- `tests/event_trigger_pipeline.rs` - **New** integration tests validating the event → trigger → job pipeline: matching/non-matching events, multi-listener fan-out, per-listener rate limiting, and trigger → job dispatch through the `Orchestrator` (observed via a `TaskService` test double).
- `tests/integration/mod.rs` - Registers the new integration test module.
- `docker/docker-compose.test.yml` - Reference for Redis host/port used by the feature-gated queue contract test.

### Notes

- Unit tests live in `#[cfg(test)]` modules beside the code (scheduler in `scheduler.rs`, cron adapter in `tokio_cron_adapter.rs`); integration tests live under `tests/`.
- Tests must be deterministic: prefer in-process drives, shared counters/flags, and short bounded waits over wall-clock-threshold sleeps.
- The Redis contract test is gated behind the `redis-queue` feature and must skip gracefully (or be `#[ignore]`) when Redis is unreachable, so default `cargo test` stays green.
- No clock abstraction or scheduler production refactor (test-only). No `testcontainers`. No new retry/dead-letter design — validate existing behavior; add in-memory dead-letter parity only if missing.
- Run `cargo test` (and at least once with `--features redis-queue`), `cargo fmt --check`, and `cargo clippy -- -D warnings` before committing each parent task.

## Tasks

- [x] 0.0 Create feature branch
  - [x] 0.1 Create and checkout `feature/milestone_9-epic_2-scheduler-queue-operational-validation`

- [x] 1.0 Validate scheduler tick loop + cron adapter (PRD Task 2.1)
  - [x] 1.1 Unit tests for `calculate_next_run` across every `Schedule` variant (`Interval`, `Daily`, `Weekly`, `Monthly`, `Once` future→`Some`/past→`None`, `OnStartup`→`None`)
  - [x] 1.2 Deterministic test: register a `TaskService` test double, add a job with `next_run` in the past, run one `check_and_execute_jobs()` tick, assert the service is invoked exactly once
  - [x] 1.3 Assert post-dispatch bookkeeping: `last_run` set, `run_count` incremented, `next_run` recomputed (non-`None` for recurring, `None` for `Once`/`OnStartup`)
  - [x] 1.4 Assert a disabled job is skipped: no service invocation and `run_count` unchanged after a tick
  - [x] 1.5 Assert a past `Schedule::Once` job fires once then has `next_run == None` (does not re-fire on the next tick)
  - [x] 1.6 Scheduler integration test: schedule a job a short interval in the future, drive the scheduler, verify it executes (no clock abstraction)
  - [x] 1.7 `TokioCronSchedulerAdapter` tests: imminent cron job fires (shared counter), invalid cron → `InvalidCronExpression`, scheduling while not running → `NotRunning`, and start→schedule→cancel/shutdown lifecycle; document UTC assumption

- [x] 2.0 Validate QueuePort contract for in-memory + Redis, incl. retry/dead-letter (PRD Task 2.2)
  - [x] 2.1 Write a reusable contract routine parameterized over a `QueuePort` impl: `create_queue` → `enqueue` → `dequeue` (payload round-trip) → `start_processing` → `complete_processing` → `queue_length` → `health_check`
  - [x] 2.2 Run the contract against the in-memory `QueueOrchestrator`/`QueueService` as an always-on test
  - [x] 2.3 Run the contract against `RedisQueueAdapter`, gated behind `redis-queue`, connecting to docker-compose Redis via config/env; skip gracefully / `#[ignore]` when unreachable
  - [x] 2.4 Retry test: item with `max_retries = N`, `fail_processing` reports re-queue while `attempt_count < N` (both adapters)
  - [x] 2.5 Dead-letter test: after retries exhausted, `fail_processing` reports no retry and the item is moved to the failed/dead-letter store, observable via stats/getter (both adapters)
  - [x] 2.6 If the in-memory queue lacks dead-letter parity, add the minimal behavior to satisfy 2.5 (and only that)
  - [x] 2.7 Fallback/health test: in-memory queue succeeds at the same call sites; health check reflects availability per adapter
  - [x] 2.8 Inspect `docker/docker-compose.test.yml` + test config to confirm the Redis host/port the test uses (resolves Open Question 1)

- [x] 3.0 Validate event → trigger → job pipeline (PRD Task 2.3)
  - [x] 3.1 Inspect the `ListenerOrchestrator` `trigger_queue` → `Orchestrator` dispatch path to confirm how a created trigger reaches `execute_job` (resolves Open Question 2)
  - [x] 3.2 Test: register an `EventListener` with a `TriggerCondition`, fire a matching event, assert exactly one `Trigger` is created
  - [x] 3.3 Test: fire a non-matching event, assert no trigger is created
  - [x] 3.4 Fan-out test: multiple matching listeners → exactly one trigger per matching listener
  - [x] 3.5 Test: created trigger is converted to a job and executed via the Epic 1 dispatch path (observed by a `TaskService` test double)
  - [x] 3.6 Rate-limit test: exceeding a listener's `max_triggers_per_window` does not create excess triggers (count capped at window limit)
  - [x] 3.7 Add only minimal glue if needed to route a created trigger into the dispatch path (no new listener subsystem)

- [ ] 4.0 Quality gate & finalize
  - [ ] 4.1 `cargo build --workspace`
  - [ ] 4.2 `cargo test --workspace`
  - [ ] 4.3 `cargo test --workspace --features redis-queue` (Redis contract passes or is correctly skipped)
  - [ ] 4.4 `cargo clippy --workspace --all-targets -- -D warnings`
  - [ ] 4.5 `cargo fmt --all -- --check`
  - [ ] 4.6 Remove temporary debug prints; update PRD Success Metrics + Task Checklist; update Relevant Files
