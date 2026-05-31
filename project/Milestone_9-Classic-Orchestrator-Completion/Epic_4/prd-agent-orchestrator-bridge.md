# PRD: Agent → Orchestrator Bridge (Milestone 9, Epic 4)

**Project:** Paladin Framework
**Milestone:** 9 — Classic Orchestrator, Content Pipeline, and Agent-Orchestrator Bridge
**Epic:** 4 of 6 — Agent → Orchestrator Bridge
**Status:** Approved for implementation
**Source spec:** [Milestone_9-Epic_4-agent-orchestrator-bridge.md](Milestone_9-Epic_4-agent-orchestrator-bridge.md)
**Companion Epic:** Epic 3 — Content → Agent Bridge (the inverse direction)

---

## 1. Introduction / Overview

Today a Paladin agent can call tools via the Arsenal (MCP servers, skills, prompts) and read/write
its Garrison memory, but it has **no way to drive the classic orchestration system**. An agent that
finishes analyzing a document cannot say "schedule a follow-up analysis for tomorrow", "queue this
content for deeper processing", "fire a `critical_finding` event", or "notify the admin".

This Epic builds the **agent → orchestrator** half of the integration. Epic 3 delivered the
**content → agent** half (the `Orchestrator` invokes agents through `ContentProcessor`
implementations). Epic 4 closes the loop so an agent can, mid-execution, request orchestration
actions through a narrow, guarded interface.

The feature introduces an **`OrchestratorPort`** abstraction (in the `paladin-ports` crate), a
concrete **adapter over the root-crate `Orchestrator`** (in the root crate, because it depends on
`Orchestrator`), and a **`BridgePolicy`** guardrail object that constrains what an agent is allowed
to do. The bridge is injected into `PaladinExecutionService` as an optional dependency, mirroring
how `LlmPort` and `ArsenalPort` are wired today.

---

## 2. Goals

1. Define a stable, fully-specified `OrchestratorPort` trait covering the four agent-triggerable
   orchestration actions: **schedule a job**, **enqueue a content item**, **fire an event**, and
   **send a notification**.
2. Implement a concrete adapter that maps each port method onto real `Orchestrator` capabilities
   (`schedule_job`, `queue_job`/content enqueue, the listener event path, and a
   `NotificationDeliveryPort`).
3. Enforce safety guardrails via a typed `BridgePolicy` (action allow-list + quantitative caps) so an
   agent cannot schedule unbounded work or fire arbitrary actions; out-of-policy requests fail with a
   typed `OrchestratorBridgeError`.
4. Wire the bridge into `PaladinExecutionService` as an optional `Arc<dyn OrchestratorPort>`, with no
   behavioral change when the bridge is absent (`None`).
5. Prove the end-to-end flow with an integration test: a scripted LLM tool-call drives the agent to
   **schedule a job**, and the job is observable in the orchestrator's scheduler state.
6. Keep the core crate free of any concrete `Orchestrator` dependency (preserve hexagonal boundaries).

---

## 3. User Stories

- **As an agent author**, I want my Paladin to schedule a follow-up job when it detects that more
  work is needed, so multi-stage workflows can be initiated autonomously.
- **As an agent author**, I want my Paladin to enqueue a content item for further processing, so
  large pipelines can be fanned out from within an agent run.
- **As an agent author**, I want my Paladin to fire a domain event (e.g., `critical_finding`), so
  registered listeners/triggers can react to agent conclusions.
- **As an agent author**, I want my Paladin to send a notification when something important happens,
  so humans are alerted without a separate integration.
- **As a platform operator**, I want a clear allow-list and caps on what an agent may trigger, so a
  misbehaving or prompt-injected agent cannot schedule unbounded work or spam notifications.
- **As a framework maintainer**, I want the bridge expressed as a port + adapter, so the agent core
  stays decoupled and the bridge is unit-testable with a mock orchestrator.

---

## 4. Functional Requirements

The following requirements are numbered for traceability.

### 4.1 `OrchestratorPort` trait (in `paladin-ports`)

1. The system **must** define an `async_trait` `OrchestratorPort: Send + Sync` in
   `crates/paladin-ports/src/output/orchestrator_port.rs`, re-exported from the crate's `output`
   module.
2. The trait **must** define exactly these four methods, each returning
   `Result<_, OrchestratorBridgeError>`:
   - `async fn schedule_job(&self, request: ScheduleJobRequest) -> Result<Uuid, OrchestratorBridgeError>`
   - `async fn queue_item(&self, request: QueueItemRequest) -> Result<Uuid, OrchestratorBridgeError>`
   - `async fn fire_event(&self, request: FireEventRequest) -> Result<EventDispatchResult, OrchestratorBridgeError>`
   - `async fn send_notification(&self, request: SendNotificationRequest) -> Result<Uuid, OrchestratorBridgeError>`
3. The request value objects (`ScheduleJobRequest`, `QueueItemRequest`, `FireEventRequest`,
   `SendNotificationRequest`) and result/error types **must** be defined in `paladin-ports` so they
   carry no dependency on the root-crate `Orchestrator`. They **must** use only types already
   available to `paladin-ports` (e.g., `paladin-core` domain types such as `Schedule`, `Event`,
   primitive/`serde_json` fields) plus newly-introduced plain structs.
4. Each request type **must** be a simple, serializable value object (LLM-friendly): e.g.,
   `ScheduleJobRequest { name: String, description: String, schedule: Schedule }`,
   `FireEventRequest { event_type: String, payload: serde_json::Value, source: String }`,
   `SendNotificationRequest { channel, recipient, subject, body }`,
   `QueueItemRequest { queue_name: String, payload: serde_json::Value }`. Exact field sets are
   finalized in Task 4.1's interface definition.
5. The trait and all public request/result/error types **must** have rustdoc following the existing
   port documentation conventions.

### 4.2 `OrchestratorBridgeError` (in `paladin-ports`)

6. The system **must** define `OrchestratorBridgeError` using `thiserror`, including at minimum:
   - `ActionNotAllowed(String)` — the requested action is not in the policy allow-list.
   - `QuotaExceeded { action: String, limit: u32 }` — a quantitative cap was hit.
   - `InvalidRequest(String)` — the request value object failed validation.
   - `OrchestratorError(String)` — the underlying orchestrator call failed (stringified at the
     boundary to avoid leaking root-crate error types into `paladin-ports`).
7. Error messages **must** be actionable and must not leak secrets.

### 4.3 `BridgePolicy` guardrails (in `paladin-ports`)

8. The system **must** define a `BridgePolicy` value object with:
   - An **action allow-list** (which of the four actions are permitted), e.g. a set/flags of
     `BridgeAction { ScheduleJob, QueueItem, FireEvent, SendNotification }`.
   - **Quantitative caps** applied per agent execution, e.g. `max_jobs_scheduled`,
     `max_items_queued`, `max_events_fired`, `max_notifications_sent`.
9. `BridgePolicy` **must** provide a sensible, conservative `Default` (recommendation: all four
   actions allowed with small caps, e.g. each cap = a low single-digit number) and a builder or
   explicit constructors for customization.
10. Policy enforcement **must** reject a disallowed action with `ActionNotAllowed` and a
    cap-exceeding action with `QuotaExceeded`, **before** any underlying orchestrator call is made.
11. Caps **must** be enforced per `PaladinExecutionService` execution (counters reset per run), not
    globally for the process lifetime. The counting mechanism is an implementation detail of the
    adapter/service and must be thread-safe.

### 4.4 Concrete adapter (in the **root** crate)

12. The system **must** implement `OrchestratorBridgeAdapter` (concrete `OrchestratorPort`) in the
    root crate under `src/application/services/orchestration/` (e.g.,
    `orchestrator_bridge.rs`), because it depends on the root-crate `Orchestrator`. A module-level
    doc comment **must** explain this placement (identical rationale to Epic 3's processors).
13. The adapter **must** hold an `Arc<Orchestrator>`, a `BridgePolicy`, and an optional
    `Arc<dyn NotificationDeliveryPort>` (notifications are only available when a delivery port is
    injected; absence yields `ActionNotAllowed`/`InvalidRequest` for `send_notification`).
14. `schedule_job` **must** construct a `Job` (via `Job::new`) and call
    `Orchestrator::schedule_job(job, schedule, context)`, returning the resulting `Uuid`.
15. `queue_item` **must** enqueue the content/payload onto the named queue using the orchestrator's
    queue capability (`Orchestrator::queue_job` or the underlying `QueueService`), returning the
    item `Uuid`.
16. `fire_event` **must** construct an `Event` (via `Event::new(event_type, payload, source)`) and
    dispatch it through the orchestrator's listener path
    (`ListenerOrchestrator::process_event`), returning an `EventDispatchResult` describing the
    triggers created (e.g., count and/or ids).
17. `send_notification` **must** build a `Notification` and deliver it through the injected
    `NotificationDeliveryPort::deliver_notification`, returning the notification `Uuid`.
18. Every adapter method **must** consult `BridgePolicy` first (allow-list + cap) and only then
    perform the underlying call. Underlying errors **must** be mapped to
    `OrchestratorBridgeError::OrchestratorError` with a descriptive message.

### 4.5 Wiring into the execution context

19. `PaladinExecutionService` **must** gain an optional `orchestrator_port: Option<Arc<dyn OrchestratorPort>>`
    field, mirroring the existing optional `garrison`/`arsenal` fields.
20. A constructor or builder method **must** allow attaching the bridge without breaking existing
    callers (recommendation: add a `with_orchestrator_port(...)` builder-style setter and/or extend
    construction; the 4-arg `PaladinExecutionService::new(llm_port, circuit_breaker, garrison, arsenal)`
    signature **must** keep compiling for existing call sites — add wiring in a backward-compatible
    way).
21. When `orchestrator_port` is `None`, agent execution behavior **must** be byte-for-byte unchanged
    from today.

### 4.6 Tests

22. Unit tests **must** cover all four port methods against a **mock orchestrator port** (or mock
    `Orchestrator` collaborators), including: success path for each action, `ActionNotAllowed` when
    the action is not in the allow-list, and `QuotaExceeded` when a cap is exceeded.
23. An integration test (`tests/agent_orchestrator_bridge.rs`) **must** deterministically drive a
    real `PaladinExecutionService` (with a `MockLlmAdapter`/`MultiStepMockLlmPort` scripted to emit a
    tool/function call) to perform **`schedule_job`**, and then assert the job is observable in the
    real `Orchestrator`'s scheduler state.
24. The other three actions (`queue_item`, `fire_event`, `send_notification`) **must** be covered by
    unit tests against the mock; only `schedule_job` is required to be driven fully end-to-end in the
    integration test.

### 4.7 Quality gates

25. `cargo build`, `cargo test`, `cargo clippy -- -D warnings`, and `cargo fmt --check` **must** all
    pass with the default feature set. Any feature-gated code introduced must also build/clippy clean
    under its feature.

---

## 5. Non-Goals (Out of Scope)

- **Arsenal-tool bridge (Option B).** This Epic adopts the port approach (Option A). Exposing the
  same capabilities as an LLM-discoverable Arsenal tool (`OrchestratorArmament`) is explicitly
  deferred as a **non-breaking follow-up** that can wrap the `OrchestratorPort`. See §6 trade-off.
- **New orchestration capabilities.** No new `Orchestrator` features beyond those already exposed
  (`schedule_job`, `queue_job`, listener/event dispatch, notification delivery).
- **The content → agent direction.** Delivered in Epic 3.
- **Full RBAC / per-tenant authorization.** `BridgePolicy` is a minimal allow-list + caps, not a full
  policy engine.
- **Driving all four actions end-to-end in integration tests.** Only `schedule_job` is required
  end-to-end; the rest are unit-tested.
- **Persisting per-execution cap counters across restarts.** Caps are per-execution, in-memory.

---

## 6. Design Considerations

### 6.1 Option A (port) vs Option B (Arsenal tool) — documented trade-off

| Criterion | Option A — `OrchestratorPort` (CHOSEN) | Option B — Arsenal tool (`OrchestratorArmament`) |
|---|---|---|
| **Discoverability by LLM** | Not self-describing; agent author wires actions explicitly | Tool schema is self-describing to the LLM (natural tool-use) |
| **Safety / authorization** | Easy to gate centrally via `BridgePolicy` before any call | Gating must live inside tool execution; relies on tool plumbing |
| **Testability / coupling** | Mockable trait; core stays decoupled; mirrors Epic 3 | Reuses Arsenal plumbing but couples capability to tool-call format |
| **Consistency with Arsenal/MCP** | Diverges from tool-use model | Matches existing tool-use model |

**Decision (1A/C):** Adopt **Option A**. It maximizes decoupling, testability, and centralized
safety enforcement, and it mirrors the Epic 3 placement pattern. Option B's discoverability benefit
is preserved as a future, non-breaking enhancement: an `OrchestratorArmament` can simply wrap an
`Arc<dyn OrchestratorPort>` and register in the Arsenal later, without changing the port.

### 6.2 Crate placement (4A)

- `OrchestratorPort`, the request/result types, `OrchestratorBridgeError`, `BridgePolicy`, and
  `BridgeAction` live in **`paladin-ports`** (no `Orchestrator` dependency).
- The concrete `OrchestratorBridgeAdapter` lives in the **root crate** alongside the orchestration
  services, because it depends on the root-crate `Orchestrator`. (A lower crate cannot depend on the
  root crate without a circular dependency — this is the same constraint encountered and documented
  in Epic 3.)

### 6.3 Action surface (2A)

All four methods are defined now so the interface is complete and stable, even though only
`schedule_job` is exercised end-to-end in the integration test. The other three are validated by unit
tests against the mock. This avoids future breaking changes to the trait.

### 6.4 Guardrails (3A)

`BridgePolicy` is an enforced value object: an **allow-list** of permitted `BridgeAction`s plus simple
**quantitative caps** per execution. Violations are rejected up front with a typed
`OrchestratorBridgeError` (`ActionNotAllowed` / `QuotaExceeded`). This is intentionally minimal — not
RBAC — but sufficient to bound a misbehaving or prompt-injected agent.

---

## 7. Technical Considerations

- **Grounded API references (verified in source):**
  - `Orchestrator::schedule_job(job: Job, schedule: Schedule, context: OrchestrationContext) -> Result<Uuid, OrchestratorError>`
    — [src/application/services/orchestration/mod.rs](../../../src/application/services/orchestration/mod.rs)
  - `Orchestrator::queue_job(job: Job, queue_name: &str, context: OrchestrationContext) -> Result<Uuid, OrchestratorError>`
  - `ListenerOrchestrator::process_event(event: Event) -> Result<Vec<Uuid>, ListenerError>`
    — [src/application/services/orchestration/listener.rs](../../../src/application/services/orchestration/listener.rs)
    (Note: `fire_event` maps to this path; the `Orchestrator` does not expose a public `fire_event`
    today, so the adapter dispatches via the listener service it owns.)
  - `Job::new(name: String, description: String, tasks: Vec<Task>) -> Job`
    — [crates/paladin-core/src/platform/container/job.rs](../../../crates/paladin-core/src/platform/container/job.rs)
  - `Schedule` enum (`Interval`, `Daily`, `Weekly`, `Monthly`, `Once`, `OnStartup`)
    — [crates/paladin-core/src/platform/container/schedule.rs](../../../crates/paladin-core/src/platform/container/schedule.rs)
  - `Event::new(event_type: String, payload: serde_json::Value, source: String) -> Event`
    — [crates/paladin-core/src/base/component/event.rs](../../../crates/paladin-core/src/base/component/event.rs)
  - `NotificationDeliveryPort::deliver_notification(Notification) -> NotificationPortResult<NotificationDeliveryResult>`
    — [crates/paladin-ports/src/output/notification_port.rs](../../../crates/paladin-ports/src/output/notification_port.rs)
  - `PaladinExecutionService::new(llm_port, circuit_breaker, garrison, arsenal)` with optional
    `garrison`/`arsenal` fields — [src/application/services/paladin/paladin_execution_service.rs](../../../src/application/services/paladin/paladin_execution_service.rs)
- **Hexagonal boundaries:** dependencies flow inward only. `paladin-ports` must not import the root
  crate. The adapter (root crate) imports both `paladin-ports` and the `Orchestrator`.
- **Backward compatibility:** existing `PaladinExecutionService::new` call sites must keep compiling;
  add the bridge via an additive setter/field defaulting to `None`.
- **Thread-safety:** per-execution cap counters must be safe under concurrent tool execution
  (`Arc`/atomic or `Mutex`); reset per run.
- **Error mapping:** root-crate `OrchestratorError`/`ListenerError`/`NotificationPortError` must be
  stringified into `OrchestratorBridgeError::OrchestratorError` at the adapter boundary.
- **Security:** validate request value objects at the boundary; never expand caps from agent-supplied
  input; do not log secrets in notification payloads. Run `snyk_code_scan` on new first-party code per
  repo policy.

---

## 8. Success Metrics

- ✅ `OrchestratorPort` (4 methods), request/result types, `OrchestratorBridgeError`, `BridgePolicy`,
  and `BridgeAction` exist in `paladin-ports` with rustdoc.
- ✅ `OrchestratorBridgeAdapter` in the root crate implements all four methods over the real
  `Orchestrator` + `NotificationDeliveryPort`, with policy enforced before each call.
- ✅ `PaladinExecutionService` accepts an optional `Arc<dyn OrchestratorPort>`; behavior unchanged when
  `None`.
- ✅ Unit tests: success + `ActionNotAllowed` + `QuotaExceeded` for all four actions (mock-backed).
- ✅ Integration test deterministically drives an agent (scripted tool-call) to `schedule_job`; the
  job is observable in the orchestrator scheduler state.
- ✅ Quality gate green: `cargo build`, `cargo test`, `cargo clippy -- -D warnings`, `cargo fmt --check`.

---

## 9. Open Questions

1. **Resolved (1A/C):** Adopt Option A (`OrchestratorPort`). Option B (Arsenal tool) deferred as a
   non-breaking wrapper follow-up.
2. **Resolved (2A):** Define all four methods now; drive only `schedule_job` end-to-end in
   integration, unit-test the rest.
3. **Resolved (3A):** Enforced `BridgePolicy` = allow-list + quantitative caps + typed
   `OrchestratorBridgeError`.
4. **Resolved (4A):** Port + policy/error types in `paladin-ports`; concrete adapter in root crate;
   `PaladinExecutionService` holds `Option<Arc<dyn OrchestratorPort>>`.
5. **To finalize during Task 4.1:** exact field sets of the four request value objects and the shape
   of `EventDispatchResult` (count vs. trigger ids). Default to the simplest shape that satisfies the
   tests.
6. **To finalize during Task 4.2:** whether `queue_item` reuses `Orchestrator::queue_job` (wrapping a
   `Job`) or enqueues a lighter content message; choose whichever keeps the agent-facing request
   simplest while remaining observable in orchestrator state.

---

## Task Checklist

High-level mapping to the Epic's tasks (detailed sub-tasks are produced in the companion
`tasks-agent-orchestrator-bridge.md`).

- [x] **Task 4.1 — Design & interface.** Document the Option A vs B trade-off and the decision;
  fully specify the `OrchestratorPort` trait, request/result types, `OrchestratorBridgeError`,
  `BridgePolicy`, and `BridgeAction` in `paladin-ports`. (FR 1–11)
- [x] **Task 4.2 — Implement the bridge.** Implement `OrchestratorBridgeAdapter` in the root crate;
  enforce `BridgePolicy`; wire `Option<Arc<dyn OrchestratorPort>>` into `PaladinExecutionService`;
  unit-test all four actions (success + `ActionNotAllowed` + `QuotaExceeded`) against the mock.
  (FR 12–22, 24)
- [x] **Task 4.3 — Integration test.** `tests/agent_orchestrator_bridge.rs`: scripted LLM tool-call →
  `schedule_job` → assert job observable in orchestrator scheduler state. (FR 23)
- [x] **Quality gate.** `cargo build`, `cargo test`, `cargo clippy -- -D warnings`, `cargo fmt --check`
  all pass. (FR 25)
