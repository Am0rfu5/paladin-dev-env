# Milestone 9: Classic Orchestrator Completion

**Project:** Paladin Framework
**Milestone:** 9 — Classic Orchestrator, Content Pipeline, and Agent-Orchestrator Bridge
**Version Target:** v0.3.0
**Status:** Planning
**Created:** 2026-05-29
**Document Version:** 1.0

---

## Executive Summary

The Paladin framework was designed as a dual-purpose platform: a classic content orchestration system (triggers, schedulers, queues, workflows, content processing) and an AI agent orchestration framework. Milestones 4–8 built and refined the AI agent orchestration side (`PaladinExecutionService`, `PaladinBuilder`, Battalion patterns). The classic orchestration subsystem — `Orchestrator`, `SchedulerOrchestrator`, `ListenerOrchestrator`, `QueueService`, workflow engine, and content processing pipeline — was architecturally placed in Milestone 6 but remains operationally incomplete.

This Milestone gets it working. End to end.

### The Gap

The `Orchestrator` struct has the right methods: `start()`, `schedule_job()`, `queue_job()`, `process_content()`, `create_workflow()`, `process_event()`. The `ServiceRunner` wires up the scheduler, message service, event service, and adapters. But the execution loops contain `println!` statements where dispatch logic should be, the content→agent and agent→orchestrator bridges don't exist, the queue adapter integration through `QueuePort` hasn't been validated end-to-end, and the user/admin system's auth flow is incomplete.

### Success Criteria

- A workflow defined programmatically can be created, scheduled, queued, executed, and completed with observable, testable results.
- The scheduler dispatches due jobs on its tick loop.
- The queue system works end-to-end (in-memory and via Redis adapter).
- A content processing step can invoke a Paladin AI agent.
- A Paladin AI agent can trigger orchestration actions (queue a job, fire an event, send a notification).
- The user/admin authentication and authorization flow functions through the API.
- Integration tests validate each bridge and lifecycle.

---

## Parallel Execution Context

This Milestone **depends on Milestone 8** (facade cleanup must be complete so directory structure is stable). It can run **in parallel with Milestones 10 and 11** once Epic 1 stabilizes the Orchestrator API.

Milestone 11 (Documentation) should not document orchestration features until this Milestone's Epics 1–3 are complete.

---

## Epic 1: Orchestrator End-to-End Workflow Execution

**Priority:** Critical
**Estimated Effort:** Large
**Dependencies:** Milestone 8 complete

### Objective

Replace the scaffold logic in the `Orchestrator` with functional dispatch. A workflow with sequential, parallel, or event-driven execution order runs from creation to completion with tracked state.

### Background

The current `Orchestrator` code (in `src/application/services/orchestration/mod.rs` post-Milestone 8 rename) stores workflows and dispatches jobs, but the actual execution loop — pick next job from queue → execute via task service → handle result → advance workflow state — uses `println!` statements rather than real dispatch. The `execute_trigger()` method converts triggers to jobs but doesn't execute them. The `process_content()` method calls `processor.process_content()` but the default `ContentProcessor` implementations are stubs.

### Tasks

#### Task 1.1: Implement Workflow Execution Loop

**Description:** Implement the core execution loop for `Orchestrator`:
- Sequential: execute jobs in order, passing output of job N as context to job N+1.
- Parallel: spawn all jobs concurrently, collect results.
- Event-driven: register listeners that trigger jobs when events arrive.
- Custom (staged): execute stages in order, jobs within a stage in parallel.

Track job state transitions: Pending → Running → Completed/Failed. Store results in the `OrchestrationContext`.

**Deliverables:**
- Functional `execute_workflow()` method.
- Job state machine with observable transitions.
- Unit tests for each execution order variant.

#### Task 1.2: Wire TaskService Execution

**Description:** The `Orchestrator` registers `TaskService` implementations but the dispatch path from `ScheduledJob` → `TaskService::execute()` is incomplete. Implement the full dispatch: resolve the service by name, pass the job's tasks to the service, collect results, handle errors per the job's error strategy.

**Deliverables:**
- `TaskService` dispatch functional.
- Default services (`DataBackupService`, `ContentIndexingService`, `EmailNotificationService`) execute real logic (or are replaced with meaningful implementations).
- Error handling and retry for failed tasks.

#### Task 1.3: Implement Workflow State Persistence

**Description:** Workflows currently live in an in-memory `HashMap`. For production use, workflow state (current stage, job results, error history) should persist across restarts. Implement via the existing `SqliteStore` or a new `WorkflowRepository`.

**Deliverables:**
- Workflow state persisted to SQLite.
- Workflow recovery on `Orchestrator::start()`.
- Tests for crash-recovery scenario.

#### Task 1.4: Integration Test — Full Workflow Lifecycle

**Description:** Write an integration test that: creates a workflow with 3 sequential jobs → starts the orchestrator → verifies all 3 jobs execute in order → verifies final workflow state is Completed → verifies results are accessible.

**Deliverables:**
- Integration test in `tests/integration/`.
- Test uses mock `TaskService` implementations with observable side effects.

---

## Epic 2: Scheduler and Queue Operational Validation

**Priority:** Critical
**Estimated Effort:** Medium
**Dependencies:** Epic 1

### Objective

Validate that the `SchedulerOrchestrator` tick loop dispatches due jobs on time, and that the `QueueService` integrates with both the in-memory queue and the `RedisQueueAdapter` through the `QueuePort` trait.

### Tasks

#### Task 2.1: Validate Scheduler Tick Loop

**Description:** The `SchedulerOrchestrator::start()` spawns a tokio task that should periodically check `next_run` times and dispatch due jobs. Verify this loop:
- Calculates `next_run` correctly for `Schedule::Interval`, `Schedule::Cron`, `Schedule::Once`.
- Dispatches jobs whose `next_run` has passed.
- Updates `last_run`, `run_count`, and `next_run` after execution.
- Handles disabled jobs (skips them).

**Deliverables:**
- Scheduler tick loop verified functional.
- Unit tests for each schedule type.
- Integration test: schedule a job for 1 second from now → verify it executes.

#### Task 2.2: Validate QueueService ↔ RedisQueueAdapter Integration

**Description:** The `QueueService` is an in-memory queue. The `RedisQueueAdapter` implements `QueuePort`. Validate that:
- `QueueService` can be backed by `RedisQueueAdapter` when the `redis-queue` feature is enabled.
- Enqueue/dequeue operations work through the port trait.
- Queue retry and dead-letter behavior function.
- Health checks report correct status.

**Deliverables:**
- Integration test with testcontainers for Redis.
- In-memory fallback verified when Redis is unavailable.
- Queue retry logic tested (enqueue → fail → retry → succeed or dead-letter).

#### Task 2.3: Validate Event → Trigger → Job Pipeline

**Description:** The `ListenerOrchestrator` receives events, matches them to registered listeners, creates triggers, and passes them to the `Orchestrator` for execution. Validate the full pipeline:
- Register a listener with a trigger condition.
- Fire an event that matches the condition.
- Verify a trigger is created.
- Verify the trigger is converted to a job and executed.

**Deliverables:**
- Integration test for the event pipeline.
- Edge case tests: no matching listener, multiple matching listeners, rate-limited listener.

---

## Epic 3: Content → Agent Bridge

**Priority:** High
**Estimated Effort:** Large
**Dependencies:** Epic 1 (Orchestrator must be functional)

### Objective

Enable content processing workflows to invoke Paladin AI agents for content enrichment, analysis, or action. A `ContentProcessor` implementation wraps `PaladinExecutionService` (or a Battalion pattern) and integrates into the Orchestrator's `process_content()` pipeline.

### Background

The architectural vision: ingest a news article → run through content processing (extract text, summarize) → invoke an AI agent to analyze sentiment and extract key entities → store enriched results. The `ContentProcessor` trait exists with `process_content()` and `clone_box()` methods. The `DefaultContentProcessor` is a stub. The bridge between content processing and AI agent execution doesn't exist.

### Tasks

#### Task 3.1: Implement `PaladinContentProcessor`

**Description:** Create a `ContentProcessor` implementation that:
- Takes a `ContentItem` and an `OrchestrationContext`.
- Converts the content item into a prompt for a Paladin agent.
- Invokes `PaladinExecutionService::execute()` with the prompt.
- Parses the agent's response into a `ContentProcessingResult` with enrichment metadata.

**Deliverables:**
- `PaladinContentProcessor` struct implementing `ContentProcessor`.
- Configurable: which Paladin configuration to use, what prompt template, what output parsing strategy.
- Unit tests with mock LLM adapter.

#### Task 3.2: Implement `BattalionContentProcessor`

**Description:** Create a `ContentProcessor` implementation that invokes a Battalion pattern (e.g., a Phalanx of 3 specialist analysts, or a Formation pipeline of summarizer → classifier → entity extractor).

**Deliverables:**
- `BattalionContentProcessor` struct implementing `ContentProcessor`.
- Supports Formation (sequential pipeline) and Phalanx (parallel analysts) patterns.
- Configurable via battalion config or Maneuver flow expression.
- Unit tests with mock agents.

#### Task 3.3: Wire Content Processing into Orchestrator

**Description:** Update `Orchestrator::process_content()` to use the registered `ContentProcessor` implementations. Enable workflows to include content processing steps that invoke AI agents.

**Deliverables:**
- `Orchestrator::register_content_processor()` accepts `PaladinContentProcessor` and `BattalionContentProcessor`.
- Workflows can reference content processors by name in their job definitions.
- Integration test: ingest content → process with AI agent → verify enriched output.

#### Task 3.4: Content Ingestion Pipeline Validation

**Description:** Validate the full content ingestion pipeline from `paladin-content` crate: `PdfExtractor`, `HttpContentFetcher`, `FileContentListFetcher`, `NewsApiFetcher` → content aggregation → content analysis → AI agent enrichment → delivery.

**Deliverables:**
- End-to-end integration test for at least one ingestion path (e.g., fetch URL → extract text → invoke agent → store result).
- Requires `content-processing` feature flag enabled.

---

## Epic 4: Agent → Orchestrator Bridge

**Priority:** High
**Estimated Effort:** Large
**Dependencies:** Epic 1 (Orchestrator must be functional)

### Objective

Enable Paladin AI agents, during execution, to trigger orchestration actions: schedule a job, enqueue content processing, fire an event, or send a notification. This completes the bidirectional integration between the AI agent system and the classic orchestration system.

### Background

Currently, a Paladin agent can use tools via the Arsenal (MCP servers, skills, prompts). But it cannot interact with the orchestration system. An agent analyzing a document should be able to say "schedule a follow-up analysis for tomorrow" or "queue this content for further processing" or "notify the admin that a critical finding was detected."

### Tasks

#### Task 4.1: Design the Agent → Orchestrator Interface

**Description:** Evaluate two approaches:

**Option A — OrchestratorPort trait in `paladin-ports`:** Define `OrchestratorPort` with methods like `schedule_job()`, `queue_item()`, `fire_event()`, `send_notification()`. Inject into `PaladinExecutionService` alongside `LlmPort` and `ArsenalPort`. The agent accesses orchestrator capabilities through the port.

**Option B — Arsenal tool (Armament) wrapping the Orchestrator:** Create an `OrchestratorArmament` that registers as a tool in the agent's Arsenal. The agent invokes it via natural language tool calls (e.g., "Use the scheduler tool to create a follow-up job"). This leverages the existing MCP/tool-use infrastructure.

**Deliverables:**
- Design document evaluating both options with trade-offs.
- Selected approach.
- Interface definition (trait signature or tool schema).

#### Task 4.2: Implement the Selected Bridge

**Description:** Implement the bridge per the design decision in Task 4.1. Wire the Orchestrator (or a subset of its capabilities) into the Paladin execution context.

**Deliverables:**
- Bridge implementation.
- `PaladinExecutionService` or `PaladinBuilder` updated to accept the bridge.
- Unit tests with mock orchestrator.

#### Task 4.3: Integration Test — Agent Triggers Orchestration

**Description:** Write an integration test: create a Paladin agent with orchestrator access → execute the agent with a prompt that requires scheduling a follow-up → verify the job appears in the scheduler/queue.

**Deliverables:**
- Integration test demonstrating the bidirectional flow.
- Uses mock LLM adapter with a predetermined tool-call response.

---

## Epic 5: User and Admin System Completion

**Priority:** Medium
**Estimated Effort:** Medium
**Dependencies:** Epics 1, 2 (services must be operational for auth to protect)

### Objective

Complete the user authentication, authorization, and admin operations so the system can be deployed with access control.

### Tasks

#### Task 5.1: User CRUD via API

**Description:** Validate that `UserService` CRUD operations (create, read, update, delete) work through the web API endpoints (if `paladin-web` is enabled) or via CLI commands.

**Deliverables:**
- User CRUD functional and tested.
- Password hashing with `argon2` verified.

#### Task 5.2: Authentication Flow

**Description:** Implement or validate API key or token-based authentication. Requests to protected endpoints must include a valid credential.

**Deliverables:**
- Auth middleware functional.
- Unauthorized requests return 401.
- Integration test for auth flow.

#### Task 5.3: Role-Based Access Control

**Description:** Implement basic RBAC: admin vs. user roles. Admin can manage users, view system health, access all workflows. User can execute agents, create workflows within their scope.

**Deliverables:**
- RBAC enforcement on API endpoints.
- Admin notification and logging services operational.
- Integration tests for role-based access.

---

## Epic 6: Finalization and Release

**Priority:** High
**Estimated Effort:** Small
**Dependencies:** Epics 1–5

### Tasks

#### Task 6.1: Full Quality Gate

- `cargo build --workspace`
- `cargo test --workspace`
- `cargo clippy --workspace -- -D warnings`
- `cargo fmt --all -- --check`
- `cargo doc --workspace --no-deps`

#### Task 6.2: CHANGELOG and Version Bump

- Update `CHANGELOG.md` with all Epic deliverables.
- Bump workspace version to `0.3.0`.
- Tag v0.3.0 release candidate.

---

## Schedule Overview

| Phase | Epic | Duration | Predecessors |
|-------|------|----------|-------------|
| Phase 1 | Epic 1: Orchestrator E2E | 2–3 sprints | Milestone 8 |
| Phase 2A | Epic 2: Scheduler/Queue Validation | 1–2 sprints | Epic 1 |
| Phase 2B | Epic 3: Content → Agent Bridge | 2–3 sprints | Epic 1 |
| Phase 2C | Epic 4: Agent → Orchestrator Bridge | 2–3 sprints | Epic 1 |
| Phase 3 | Epic 5: User/Admin System | 1–2 sprints | Epics 1, 2 |
| Phase 4 | Epic 6: Finalize | 0.5 sprint | All |

**Total: 6–10 sprints** (Epics 2, 3, 4 parallelizable after Epic 1)
