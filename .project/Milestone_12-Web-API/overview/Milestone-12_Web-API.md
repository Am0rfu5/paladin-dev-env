# Milestone 12: Web API for the Paladin Framework

**Project:** Paladin Framework
**Milestone:** 12 — Web API / HTTP Service Host Topology, Out of the Box
**Version Target:** v0.6.0
**Status:** Planning
**Created:** 2026-06-07
**Document Version:** 1.0

---

## Executive Summary

Paladin documents five [deployment topologies](../../../docs/src/deployment-topologies/overview.md).
Four of them ship as usable building blocks today; the **[HTTP Service Host](../../../docs/src/deployment-topologies/http-service-host.md)**
does not. As the topology page states plainly:

> **Paladin ships no agent-execution endpoint.** The web crate's `create_app_router` wires a
> **user-management / auth** REST API — it does *not* run agents. The agent endpoint is yours
> to compose.

The current `paladin-web` crate is the *start* of a web surface — an axum user-management /
auth REST API (`/users/register`, `/users/login`, user CRUD) plus a content-delivery controller
(Milestone 8, Epic 7) — but a consumer who wants "a running instance you hit to invoke agents"
must hand-write the agent registry, the execution handlers, the config wiring, and every
production concern (health checks, auth on agent routes, streaming, observability, OpenAPI,
container image) themselves. The compiled `doc-examples/src/http_service_host.rs` example proves
it can be done, but it is a *teaching example*, not a shipped, configurable, secured server.

**This Milestone turns the HTTP Service Host topology into a first-class, out-of-the-box
capability.** It builds an agent registry abstraction and an agent-execution HTTP API into
`paladin-web`, ships a runnable, config-driven server binary that composes the full router
(agents + user/auth + delivery), and adds the production concerns a real service needs:
streaming, security, observability, an OpenAPI contract, and deployment artifacts. The outcome
is that "run Paladin agents behind an HTTP API" becomes `cargo install` + a `config.yml` + `paladin-server`,
not a do-it-yourself integration exercise.

### Success Criteria

- A consumer can run a Paladin HTTP service that executes configured agents **without writing
  any Rust** — provide a `config.yml`, run the shipped server binary, and `POST` to an agent
  endpoint.
- An **agent registry** abstraction exists, is populated from configuration, and is shared
  (`Arc`) across concurrent requests.
- `POST /agents/{id}/execute` runs the named agent via `PaladinExecutionService` and returns a
  structured JSON result; `GET /agents` and `GET /agents/{id}` describe what is available.
- A **streaming** execution endpoint (SSE) exists for token-by-token responses.
- Agent endpoints are **secured** (bearer / API key), reusing the existing auth surface, and
  authorization can be scoped per agent.
- The service exposes **health / readiness** endpoints, a **consistent JSON error model**,
  request **tracing/logging**, CORS, and graceful shutdown.
- An **OpenAPI** specification is generated and served with interactive docs (Swagger UI / RapiDoc).
- A **Dockerfile**, compose snippet, and **k8s** manifest run the server; the
  `deployment-topologies/http-service-host.md` page is updated to reflect that the API now ships.
- All new code compiles warning-free, passes `cargo fmt`/`clippy`/`cargo-deny`, and is covered by
  unit + integration tests (unit ≥ 80%, integration ≥ 70%).

---

## Parallel Execution Context

This Milestone is **largely self-contained within `paladin-web`** (plus a new server binary and
deployment artifacts). It depends on the agent-execution APIs already delivered in earlier
milestones (`PaladinExecutionService`, `Paladin`, `paladin-ai` composition root) and on
**Milestone 8 Epic 7** (the `paladin-web` axum-only cleanup) landing first, so the new routes are
added to a single-framework crate.

- **Epic 1 (Agent Registry & Execution API)** is the foundation; most other epics build on it.
- **Epic 2 (Configurable Host & Server Binary)** depends on Epic 1 (it composes and serves the
  router Epic 1 produces).
- **Epics 3–6 (Streaming, Cross-Cutting, Security, OpenAPI)** depend on Epic 1 and can proceed
  largely in parallel with one another once the route surface exists.
- **Epic 7 (Deployment Artifacts, Examples & Docs)** is the finalizing step and depends on a
  runnable server (Epic 2) and the stabilized route surface (Epics 1, 3–6).

Each Epic below is sized to become its own PRD (`/create-prd`) and from there a task list
(`/generate-tasks`).

---

## Epic 1: Agent Registry & Execution API

**Priority:** High
**Estimated Effort:** Large
**Dependencies:** Milestone 8 Epic 7 (paladin-web axum-only); `PaladinExecutionService` available

### Objective

Build the heart of the topology: a shared **agent registry** and the **agent-execution HTTP API**
that the docs currently say consumers must write themselves. Agents are looked up by id in a
registry and run via `PaladinExecutionService`.

### Tasks

- **Task 1.1: Agent registry abstraction.** Define a registry type that holds named, resident
  agents (`HashMap<AgentId, Arc<Paladin>>` or an `AgentRegistry` port) constructed once and shared
  (`Arc`) across requests. Include construction from an in-memory list (config wiring lands in
  Epic 2) and id-based lookup with a clear not-found path.
- **Task 1.2: Execution endpoint.** `POST /agents/{id}/execute` — deserialize an input request
  body, look the agent up, call `PaladinExecutionService::execute`, and return
  `200 { "output": ... }` with a structured result. Map missing agent → `404`, bad input → `400`,
  execution failure → `502/500` with the consistent error body.
- **Task 1.3: Discovery endpoints.** `GET /agents` (list agent ids + metadata: name, model,
  description) and `GET /agents/{id}` (describe a single agent's public config). No secrets in
  responses.
- **Task 1.4: Request/response DTOs.** `serde` request/response types decoupled from domain
  entities, with validation. Keep the wire contract stable and documented.
- **Task 1.5: Router composition.** Mount the agent routes into the application router alongside
  the existing user/auth and delivery routes, behind shared state.
- **Task 1.6: Tests.** Unit + handler tests (via `tower::ServiceExt::oneshot`) covering success,
  not-found, validation failure, and execution-error paths with a mocked execution service.

### Deliverables

- `paladin-web` agent registry + `agent_controller.rs` (or equivalent) with the three endpoints.
- Wired into `create_app_router`.
- Handler tests green.

---

## Epic 2: Configurable Web Host & Server Binary

**Priority:** High
**Estimated Effort:** Medium
**Dependencies:** Epic 1

### Objective

Make the topology runnable **without writing Rust**: a config schema for the host and its agents,
and a shipped server binary that loads config, builds the registry, composes the full router, and
serves with graceful shutdown.

### Tasks

- **Task 2.1: Host + agents config schema.** Formalize the `config.yml` shape sketched in the docs
  (`host.bind_address`, `agents: [{ id, model, system_prompt, temperature, max_loops, ... }]`),
  layered with env-var overrides, loaded via the existing settings mechanism.
- **Task 2.2: Registry-from-config builder.** Construct the agent registry from the config's
  `agents` list, wiring each agent's LLM provider, garrison, and arsenal as configured.
- **Task 2.3: Server binary.** A `paladin-server` binary (in `paladin-web` or a thin new crate)
  that: loads config → builds registry + services → composes router → binds `axum::serve` →
  handles SIGINT/SIGTERM for graceful shutdown. Behind a `web-server` feature flag.
- **Task 2.4: Startup validation & diagnostics.** Validate config at boot (every agent resolvable,
  bind address parseable), fail fast with actionable errors, and log the served route map + bound
  address on startup.
- **Task 2.5: Tests.** Config parsing tests (valid/invalid), registry-build test, and a smoke
  integration test that boots the server on an ephemeral port and hits `GET /agents`.

### Deliverables

- Config schema + loader for host/agents.
- Runnable `paladin-server` binary with graceful shutdown.
- Boot/diagnostics + tests.

---

## Epic 3: Streaming & Asynchronous Execution

**Priority:** Medium
**Estimated Effort:** Medium
**Dependencies:** Epic 1

### Objective

Support responses that don't fit a single request/response round trip: token streaming and
optional fire-and-poll async execution for long-running quests.

### Tasks

- **Task 3.1: SSE streaming endpoint.** `POST /agents/{id}/execute/stream` returning a
  `text/event-stream` driven by `PaladinExecutionService::execute_stream`, emitting incremental
  tokens and a terminal event. Handle client disconnects and stream errors cleanly.
- **Task 3.2: Execution timeouts & cancellation.** Per-request timeout (configurable, with a
  per-agent override) that aborts the underlying execution and returns a `504`-style structured
  error.
- **Task 3.3 (optional/stretch): Async job execution.** `POST /agents/{id}/jobs` returns a job id;
  `GET /agents/{id}/jobs/{job_id}` polls status/result. Note the relationship to the
  [queue/worker topology](../../../docs/src/deployment-topologies/queue-worker.md) and keep the
  in-process implementation simple (defer distributed execution to that topology).
- **Task 3.4: Tests.** Stream-assembly test (collect SSE events → full output), timeout test, and
  (if implemented) job lifecycle test.

### Deliverables

- SSE streaming endpoint with parity to the non-streaming result.
- Configurable timeouts/cancellation.
- Tests for streaming and timeout paths.

---

## Epic 4: API Cross-Cutting Concerns

**Priority:** High
**Estimated Effort:** Medium
**Dependencies:** Epic 1 (route surface to wrap)

### Objective

Add the production middleware a real service needs, applied uniformly across all routes.

### Tasks

- **Task 4.1: Health & readiness.** `GET /health` (liveness) and `GET /ready` (readiness — registry
  built, dependencies reachable) returning structured status, suitable for k8s probes.
- **Task 4.2: Consistent error model.** A single error type that renders every failure as a stable
  JSON body (e.g. `{ "error": { "code", "message", "details" } }`) with correct status codes,
  shared by agent, user, and delivery routes.
- **Task 4.3: Observability.** `tower-http` tracing/`TraceLayer` for structured request/response
  logs, request-id propagation, and latency timing; integrate with the workspace's existing
  `tracing` setup.
- **Task 4.4: CORS, body limits, timeouts.** Configurable CORS layer, request body size limit, and
  a global request timeout layer.
- **Task 4.5: Rate limiting (basic).** A simple per-client/IP rate-limit layer (configurable, off
  by default) to protect agent endpoints.
- **Task 4.6: Tests.** Probe-endpoint tests, error-rendering tests across status codes, and a CORS
  preflight test.

### Deliverables

- Health/readiness endpoints.
- Unified JSON error model adopted by all controllers.
- Tracing, CORS, body-limit, timeout, and basic rate-limit layers wired into the router.

---

## Epic 5: API Security & Authorization

**Priority:** High
**Estimated Effort:** Medium
**Dependencies:** Epic 1; reuses existing `auth_middleware` / `AuthPort`

### Objective

Secure the agent-execution surface, reusing the existing bearer-token auth and adding the
authorization model an agent API needs (it is far more sensitive than the user CRUD routes).

### Tasks

- **Task 5.1: Authenticate agent routes.** Apply `require_auth` to agent execution + discovery
  routes so they require a valid bearer token, consistent with the existing protected user routes.
- **Task 5.2: API-key auth option.** Add static/service API-key authentication (header-based) as an
  alternative to user JWTs, for service-to-service callers; configurable via `config.yml`.
- **Task 5.3: Per-agent authorization.** Allow restricting which principals/roles may invoke a
  given agent (e.g. an `allowed_roles` list per agent in config), enforced before execution.
- **Task 5.4: Secrets hygiene.** Ensure agent/system-prompt/provider secrets are never returned by
  discovery endpoints or logged; redact sensitive fields in traces.
- **Task 5.5: Tests.** Unauthenticated → `401`, unauthorized-role → `403`, valid token/key →
  success; API-key path tests; redaction test.

### Deliverables

- Auth applied to agent routes (JWT and API-key paths).
- Per-agent authorization enforcement.
- Security tests + secret-redaction guarantees.

---

## Epic 6: OpenAPI Specification & Interactive Docs

**Priority:** Medium
**Estimated Effort:** Medium
**Dependencies:** Epics 1, 3, 4, 5 (stable route + DTO surface)

### Objective

Publish a machine-readable contract for the Web API so consumers can generate clients and explore
endpoints interactively.

### Tasks

- **Task 6.1: Annotate routes & DTOs.** Use `utoipa` (or equivalent) to derive an OpenAPI 3 spec
  from the handlers and request/response types, including error schemas and security schemes.
- **Task 6.2: Serve spec + UI.** Expose `GET /openapi.json` and mount Swagger UI / RapiDoc at a
  docs path; gate behind a feature/config flag for production.
- **Task 6.3: API versioning.** Prefix the served API under a version segment (e.g. `/v1`) and
  document the versioning/stability policy.
- **Task 6.4: Spec drift guard.** A test that regenerates the spec and fails if it drifts from a
  committed `openapi.json` baseline (mirrors the `current-exports.txt` API-surface guard pattern).
- **Task 6.5: Tests.** Spec-generation test, served-endpoint test, drift-guard test.

### Deliverables

- Generated, served OpenAPI spec + interactive UI.
- Versioned route prefix and stability policy.
- Committed spec baseline + drift test.

---

## Epic 7: Deployment Artifacts, Examples & Documentation

**Priority:** Medium
**Estimated Effort:** Medium
**Dependencies:** Epic 2 (runnable server); Epics 1, 3–6 (stable surface)

### Objective

Make the shipped server deployable and discoverable: container/orchestration artifacts, worked
examples, and documentation that reflects the API now ships out of the box.

### Tasks

- **Task 7.1: Container image.** A `Dockerfile` (multi-stage, minimal runtime) building and running
  `paladin-server`, plus a `docker-compose` snippet wiring an example `config.yml` and any backing
  services.
- **Task 7.2: Kubernetes manifest.** A `k8s/` Deployment + Service (and probes pointing at
  `/health` and `/ready`) for the server, consistent with existing k8s assets.
- **Task 7.3: Update deployment docs.** Rewrite
  [`deployment-topologies/http-service-host.md`](../../../docs/src/deployment-topologies/http-service-host.md)
  to document the shipped API and server binary (replacing the "Paladin ships no agent-execution
  endpoint / compose your own" framing), and update the topology
  [overview](../../../docs/src/deployment-topologies/overview.md) table accordingly.
- **Task 7.4: Worked example & doc-example.** Update/extend `doc-examples/src/http_service_host.rs`
  and add an `examples/` runnable that boots the server from a sample config and calls an agent.
- **Task 7.5: End-to-end integration tests.** A test suite that boots the real server, registers/logs
  in (where auth applies), executes an agent (buffered + streaming), and asserts health/readiness
  and error behavior.
- **Task 7.6: CHANGELOG & API surface.** Update `CHANGELOG.md [Unreleased]`, regenerate
  `project/current-exports.txt`, and bump versions toward **v0.6.0**.

### Deliverables

- Dockerfile + compose + k8s manifests for the server.
- Updated deployment-topology docs and overview table.
- Runnable example + e2e integration tests.
- CHANGELOG, API-surface baseline, and version bump.

---

## Schedule Overview

| Phase | Epic | Estimated Effort | Predecessors |
|-------|------|------------------|--------------|
| Phase 1 | Epic 1: Agent Registry & Execution API | Large | M8 Epic 7 |
| Phase 2 | Epic 2: Configurable Host & Server Binary | Medium | Epic 1 |
| Phase 2 | Epic 3: Streaming & Async Execution | Medium | Epic 1 |
| Phase 2 | Epic 4: API Cross-Cutting Concerns | Medium | Epic 1 |
| Phase 2 | Epic 5: API Security & Authorization | Medium | Epic 1 |
| Phase 3 | Epic 6: OpenAPI Specification & Docs | Medium | Epics 1, 3–5 |
| Phase 3 | Epic 7: Deployment Artifacts, Examples & Docs | Medium | Epics 1–6 |

**Total: ~5–7 sprints.** Epic 1 unblocks everything; Epics 2–5 proceed in parallel once the route
surface exists; Epics 6–7 finalize the contract, deployment story, and documentation.

---

## Out of Scope

- **Distributed scale-out / worker pools** — that is the
  [queue/worker topology](../../../docs/src/deployment-topologies/queue-worker.md); this Milestone
  delivers the single-process HTTP host. (Epic 3's optional async jobs stay in-process.)
- **Per-agent process isolation** — that is the
  [sidecar topology](../../../docs/src/deployment-topologies/sidecar.md).
- **New agent/LLM/orchestration capabilities** — this Milestone *exposes* existing execution APIs
  over HTTP; it does not extend `PaladinExecutionService` behavior.
- **Changes to delivery data models / ports** or the user-management domain beyond wiring them into
  the unified router, error model, and OpenAPI spec.
