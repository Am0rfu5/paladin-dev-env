# PRD: Agent Registry & Execution API (Milestone 12, Epic 1)

> **Correction (dated 2026-08-06, DEBT-01):** This document's §7 "API surface" bullet (struck
> below) instructs a future implementer to write the public-API surface baseline to the pre-rename
> `project/` path — a path that has not existed since commit `928c6d5` renamed `project/` to
> `.project/`. The baseline lives at `.project/current-exports.txt`, confirmed present at 442,369
> bytes via `ls -la .project/current-exports.txt`, re-run during this task; the pre-rename path is
> confirmed absent via `ls` on it, which returns "No such file or directory", also re-run during
> this task. This document was created 2026-06-07, months after commit `928c6d5` renamed the
> directory, so the defect it names propagates forward rather than decaying. This is one of five
> requirement documents Phase 8 / DEBT-01 corrects on the
> requirement-text side; the corresponding tooling (`scripts/check-api-surface.sh`,
> `scripts/extract-public-api.sh`, `.github/workflows/ci.yml`) was corrected separately in plan
> 08-02. Original text is retained below with inline corrections — nothing is deleted.

> **Correction (dated 2026-08-10, ADR-0037):** This document's route text — every unprefixed
> agent-route path below (execute, register, deregister, list, and describe) — is **superseded
> provenance, not a live contract**. The shipped agent API is served under a `/v1` prefix,
> confirmed against the committed `crates/paladin-web/openapi.json` drift-guard baseline (all six
> agent paths `/v1`-prefixed, re-confirmed via a grep of the committed spec this session) and
> enforced live by `openapi.rs`'s `spec_paths_are_versioned_under_v1` test. The recorded answer is
> `.planning/decisions/0037-agent-route-surface-v1.md`. Original text is retained below; each
> occurrence of an unprefixed route is followed by a new note line marking it superseded — nothing
> is struck, rewritten, or removed.

**Project:** Paladin Framework
**Milestone:** 12 — Web API / HTTP Service Host Topology, Out of the Box
**Epic:** 1 — Agent Registry & Execution API
**Version Target:** v0.6.0 (Unreleased)
**Status:** Ready for Implementation
**Created:** 2026-06-07
**Author:** AI Coding Agent (Claude Code)

---

## 1. Introduction / Overview

Paladin documents five [deployment topologies](../../../docs/src/deployment-topologies/overview.md).
Four ship as usable building blocks; the **[HTTP Service Host](../../../docs/src/deployment-topologies/http-service-host.md)**
does not. The topology page says so directly:

> **Paladin ships no agent-execution endpoint.** The web crate's `create_app_router` wires a
> **user-management / auth** REST API — it does *not* run agents. The agent endpoint is yours to
> compose.

Today, anyone who wants "a running instance you hit to invoke agents" must hand-write the agent
registry, the execution handler, and the request/response types — exactly what the compiled
`doc-examples/src/http_service_host.rs` teaching example demonstrates. `paladin-web` itself only
serves user/auth (`app.rs`, `user_controller.rs`) and content-delivery
(`delivery_controller.rs`) routes.

**This Epic builds the missing piece into `paladin-web`:** a shared **agent registry** and the
**agent-execution HTTP API**. Agents are looked up by id in the registry and run via the
`PaladinExecutorPort`. This is the foundation of Milestone 12 — Epics 2–7 (config-driven host,
streaming, cross-cutting concerns, security, OpenAPI, deployment) all build on the route surface
and registry defined here.

### Architectural seam (key constraint)

`paladin-web` is an infrastructure/adapter crate. It already depends on `paladin-ports` and
`paladin-core` (`paladin-ai-core`) but **not** on the `paladin-ai` facade that contains the
concrete `PaladinExecutionService`. Per the project's dependency-flow rule (infrastructure →
core + ports, never the facade), the registry and handlers depend **only on the
`PaladinExecutorPort` trait** (`paladin-ports::output::paladin_executor_port`) and the `Paladin`
entity (`paladin-core`). The concrete `PaladinExecutionService` — which already implements
`PaladinExecutorPort` (`src/application/services/paladin/paladin_execution_service.rs:1829`) — is
injected at composition time by the server binary (Milestone 12, Epic 2). `paladin-web` gains **no
new dependency on `paladin-ai`**.

---

## 2. Goals

1. A shared, thread-safe **agent registry** exists in `paladin-web`, holding agents by id and
   usable concurrently across requests behind `Arc`.
2. `POST /agents/{id}/execute` runs the named agent via `PaladinExecutorPort::execute` and returns
   a structured JSON result.
   > *(superseded — ADR-0037: shipped as `/v1/agents/{id}/execute`)*
3. `GET /agents` lists registered agent ids + safe metadata, and `GET /agents/{id}` describes one
   agent — with **no secrets** (no API keys, no full provider config) in either response.
   > *(superseded — ADR-0037: shipped as `/v1/agents` and `/v1/agents/{id}`)*
4. The registry supports **runtime mutation**: `POST /agents` registers a new agent from a spec and
   `DELETE /agents/{id}` removes one, both safe under concurrent reads.
   > *(superseded — ADR-0037: shipped as `/v1/agents` and `/v1/agents/{id}`)*
5. `paladin-web` gains **no dependency on the `paladin-ai` facade**; it depends only on
   `PaladinExecutorPort` + `Paladin` (+ an injected provisioner for runtime registration).
6. All new routes are mounted into the application router alongside the existing user/auth and
   delivery routes.
7. New code compiles warning-free and passes `cargo fmt --check`, `cargo clippy -- -D warnings`,
   and `cargo test`; unit/handler coverage ≥ 80%.

---

## 3. User Stories

- **As an integrator**, I want to `POST` an input to `/agents/researcher/execute` and get the
  agent's output back as JSON, so I can call Paladin agents from any HTTP client without writing
  Rust.
  > *(superseded — ADR-0037: shipped as `/v1/agents/researcher/execute`)*
- **As a client developer**, I want `GET /agents` to tell me which agents exist and their basic
  shape (id, name, model, description), so I can discover capabilities at runtime.
  > *(superseded — ADR-0037: shipped as `/v1/agents`)*
- **As an operator**, I want to register and remove agents at runtime (`POST /agents`,
  `DELETE /agents/{id}`), so I can add or retire agents without restarting the service.
  > *(superseded — ADR-0037: shipped as `/v1/agents` and `/v1/agents/{id}`)*
- **As a framework maintainer**, I want the web layer to depend only on the `PaladinExecutorPort`
  trait, so the HTTP adapter stays decoupled from the facade and honors the dependency-flow rule.
- **As a security reviewer**, I want discovery responses to omit secrets and the execution path to
  surface failures as clean status codes, so the API doesn't leak config or panic.

---

## 4. Functional Requirements

### 4.1 Agent registry

1. The system **must** provide an `AgentRegistry` type in `paladin-web` that maps an agent id
   (`String`) to an agent handle. Per the executor-model decision, each entry is a **per-agent
   pair**: `(Arc<Paladin>, Arc<dyn PaladinExecutorPort>)`. Different agents may therefore be backed
   by different executor instances (different circuit breakers, RAG, herald, etc.).
2. The registry **must** be safe for concurrent reads during request handling and for runtime
   mutation (register/deregister). It **must** use interior mutability (e.g. `RwLock<HashMap<…>>`
   or an equivalent concurrent map) so a shared `Arc<AgentRegistry>` can be cloned into router
   state.
3. The registry **must** support: construct-empty, construct-from-an-initial-list of
   `(id, Paladin, Arc<dyn PaladinExecutorPort>)`, `get(id)`, `list()` (ids + metadata), `insert`,
   and `remove(id)`.
4. `get`/`remove` on an unknown id **must** return a clear "not found" signal (not a panic and not
   a default).

### 4.2 Execution endpoint

5. The system **must** expose `POST /agents/{id}/execute`. The request body **must** deserialize an
   `ExecuteRequest { input: String }` (additional optional fields may be added in later epics; the
   contract here is `input`).
   > *(superseded — ADR-0037: shipped as `/v1/agents/{id}/execute`)*
6. On a known id, the handler **must** call `PaladinExecutorPort::execute(&paladin, &input)` and, on
   `Ok(PaladinResult)`, return `200 OK` with an `ExecuteResponse` JSON body that includes at least
   `output: String`, and **should** also surface the safe result metadata already on `PaladinResult`
   (`token_count`, `execution_time_ms`, `loop_count`, `stop_reason`).
7. If the id is unknown, the handler **must** return `404 Not Found` with the standard error body.
8. If the request body is missing/invalid (e.g. absent `input`), the handler **must** return
   `400 Bad Request` with the standard error body.
9. If `PaladinExecutorPort::execute` returns `Err(PaladinError)`, the handler **must** return
   `502 Bad Gateway` (upstream/LLM/execution failure) — *not* `500` — with the standard error body
   carrying the error message. (The unified error model arrives in Epic 4; Epic 1 uses the interim
   body in §4.6.)
10. The handler **must not** `unwrap()`/`expect()`/`panic!` on any request-driven path.

### 4.3 Discovery endpoints

11. The system **must** expose `GET /agents` returning a JSON array of agent summaries. Each summary
    **must** include the agent `id` and safe metadata derived from `PaladinData` (e.g. `name`,
    `model`, and a `description`/system-prompt-derived summary). It **must not** include secrets,
    credentials, or full provider configuration.
    > *(superseded — ADR-0037: shipped as `/v1/agents`)*
12. The system **must** expose `GET /agents/{id}` returning the single-agent summary, or `404` with
    the standard error body if unknown.
    > *(superseded — ADR-0037: shipped as `/v1/agents/{id}`)*
13. Discovery responses **must not** expose the raw system prompt if it is considered sensitive;
    at minimum, secrets/API keys **must** never appear. (If the full system prompt should be
    redacted vs returned, see Open Questions Q1.)

### 4.4 Runtime registration

14. The system **must** expose `POST /agents` accepting an `AgentSpec` describing the agent to
    create (id, model, system prompt, and other public `PaladinData`-equivalent fields). On success
    it **must** return `201 Created` with the new agent's summary.
    > *(superseded — ADR-0037: shipped as `/v1/agents`)*
15. Because `paladin-web` cannot itself build a `Paladin` (that needs an `LlmPort` and the builder,
    which live behind the facade), runtime registration **must** delegate construction to an
    injected **provisioner** abstraction. The system **must** define an `AgentProvisioner` port (in
    `paladin-web`, or `paladin-ports` if shared) of the shape:

    ```rust
    #[async_trait]
    pub trait AgentProvisioner: Send + Sync {
        async fn provision(
            &self,
            spec: &AgentSpec,
        ) -> Result<(Paladin, Arc<dyn PaladinExecutorPort>), ProvisionError>;
    }
    ```

    The registry/handler calls `provision(&spec)` to materialize the `(Paladin, executor)` pair, then
    inserts it. The concrete `AgentProvisioner` impl lives in the composition root (Epic 2's binary)
    and is the only place that touches the facade/builder.
16. `POST /agents` with a duplicate id **must** return `409 Conflict`; with an invalid/unprovisionable
    spec **must** return `400 Bad Request` (validation) or `422 Unprocessable Entity` (provision
    failure), each with the standard error body.
    > *(superseded — ADR-0037: shipped as `/v1/agents`)*
17. The system **must** expose `DELETE /agents/{id}` returning `204 No Content` on success and `404`
    if the id is unknown.
    > *(superseded — ADR-0037: shipped as `/v1/agents/{id}`)*
18. If no `AgentProvisioner` is wired into the router state, `POST /agents` **must** fail closed with
    a clear error (e.g. `501 Not Implemented` / `503`) rather than panicking. (Discovery and execute
    remain functional without a provisioner.)
    > *(superseded — ADR-0037: shipped as `/v1/agents`)*

### 4.5 Router composition & state

19. The system **must** provide a constructor (e.g. `agent_router(state)` or a contribution into
    `create_app_router`) that mounts all five routes under a shared router with the registry (and
    optional provisioner) as `axum` `State`.
20. The agent routes **must** be mountable alongside the existing user/auth and delivery routers
    without conflicting paths or state-type clashes (use a dedicated state struct for agent routes,
    composed via `merge`/`nest` as the existing crate does).
21. Per the auth-timing decision, Epic 1 mounts these routes **without authentication**.
    Authentication, API keys, and per-agent authorization are **deferred to Epic 5** and **must**
    be layerable onto these routes later without changing their handlers' signatures.

### 4.6 Error body (interim, until Epic 4)

22. Error responses in this Epic **must** use the crate's existing convention from
    `delivery_controller.rs`: a JSON body `{ "error": "<message>" }` with the appropriate status
    code. Epic 4 will replace this with the milestone-wide unified error model; handlers **should**
    centralize error-body construction (a small helper) so that migration is a single change point.

### 4.7 Quality & tests

23. Every public item (registry type and methods, handlers, DTOs, the `AgentProvisioner` port)
    **must** have rustdoc.
24. The system **must** include handler tests using `tower::ServiceExt::oneshot` (the crate already
    has `tower` + `http-body-util` dev-deps) covering, at minimum: execute-success,
    execute-unknown-id (`404`), execute-bad-body (`400`), execute-error (`502`) using a **mock
    `PaladinExecutorPort`**; `GET /agents` and `GET /agents/{id}` (found + `404`); `POST /agents`
    (created, duplicate `409`, no-provisioner failure) using a **mock `AgentProvisioner`**; and
    `DELETE /agents/{id}` (`204` + `404`).
    > *(superseded — ADR-0037: all four route mentions above shipped `/v1`-prefixed —
    > `/v1/agents`, `/v1/agents/{id}`, `/v1/agents` (POST), `/v1/agents/{id}` (DELETE))*
25. A concurrency test **must** demonstrate that reads (`execute`/`list`) and a concurrent
    register/remove do not deadlock or panic.

---

## 5. Non-Goals (Out of Scope)

- **Authentication / authorization** on the agent routes — Epic 5. Epic 1 ships them open.
- **Streaming / SSE** (`execute_stream`) and **async job execution** — Epic 3.
- **Health/readiness endpoints, unified error model, tracing, CORS, rate limiting** — Epic 4.
- **OpenAPI spec / Swagger UI / API versioning prefix** — Epic 6.
- **Loading agents/registry from `config.yml`** and the **runnable server binary** — Epic 2.
  Epic 1 constructs the registry from an in-memory list and is exercised via tests.
- **The concrete `AgentProvisioner` implementation** (which builds real `Paladin`s via the facade
  builder + LLM adapters) — Epic 2's composition root. Epic 1 defines the port and uses a mock.
- Changing `PaladinExecutorPort`, `PaladinExecutionService`, `Paladin`, or `PaladinResult`.
- Persisting registry state across restarts.

---

## 6. Design Considerations

### Route surface (Epic 1)

| Method | Path | Purpose | Success | Errors |
|--------|------|---------|---------|--------|
| `POST` | `/agents/{id}/execute` | Run agent, buffered | `200` `ExecuteResponse` | `404`, `400`, `502` |
| `GET`  | `/agents` | List agents | `200` `[AgentSummary]` | — |
| `GET`  | `/agents/{id}` | Describe one agent | `200` `AgentSummary` | `404` |
| `POST` | `/agents` | Register at runtime | `201` `AgentSummary` | `400`/`422`, `409`, `501`/`503` |
| `DELETE` | `/agents/{id}` | Deregister | `204` | `404` |

> *(superseded — ADR-0037: every path in this table is `/v1`-prefixed in the shipped API —
> `/v1/agents/{id}/execute`, `/v1/agents`, `/v1/agents/{id}`, `/v1/agents`, `/v1/agents/{id}`,
> in table row order)*

### DTOs (illustrative — decoupled from domain types)

```rust
#[derive(Deserialize)]
struct ExecuteRequest { input: String }

#[derive(Serialize)]
struct ExecuteResponse {
    output: String,
    token_count: u32,
    execution_time_ms: u64,
    loop_count: u32,
    stop_reason: String,
}

#[derive(Serialize)]
struct AgentSummary { id: String, name: String, model: String, description: String }

#[derive(Deserialize)]
struct AgentSpec { id: String, name: String, model: String, system_prompt: String, /* … */ }
```

### State sharing

```text
AgentApiState {
    registry: Arc<AgentRegistry>,                 // RwLock<HashMap<String,(Arc<Paladin>, Arc<dyn PaladinExecutorPort>)>>
    provisioner: Option<Arc<dyn AgentProvisioner>>, // injected by Epic 2; None ⇒ POST /agents fails closed
}
```

Mirror the existing crate's pattern: build a dedicated sub-router with `.with_state(...)` and
`merge` it into the application router so it composes with the user/auth and delivery routers.

---

## 7. Technical Considerations

- **Crate / layer:** all new code in `crates/paladin-web/` (a new `agent_controller.rs` +
  `agent_registry.rs`, exported from `lib.rs`). Adapter layer.
- **Dependencies (ports/core only):**
  - `PaladinExecutorPort` — `crates/paladin-ports/src/output/paladin_executor_port.rs`
    (`async fn execute(&self, paladin: &Paladin, input: &str) -> Result<PaladinResult, PaladinError>`).
  - `Paladin` = `Node<PaladinData>` — `crates/paladin-core/src/platform/container/paladin.rs`.
  - `PaladinResult` — `crates/paladin-core/src/platform/container/execution_result.rs`
    (`output`, `token_count`, `execution_time_ms`, `loop_count`, `stop_reason`).
  - `PaladinError` — `paladin-core`.
- **No new heavy deps expected.** `axum`, `serde`, `serde_json`, `tokio`, `async-trait`, `uuid`,
  `chrono`, and the `tower`/`http-body-util` dev-deps are already in `paladin-web/Cargo.toml`.
- **Concurrency:** prefer `tokio::sync::RwLock` if any provisioning happens while holding the guard;
  otherwise `std::sync::RwLock` is fine for a pure in-memory map (do not hold a guard across an
  `.await`). Document the choice.
- **`AgentProvisioner` placement:** define in `paladin-web` unless a second consumer is identified;
  if it belongs to the shared port surface, place it in `paladin-ports` (note: it references
  `Paladin` + `PaladinExecutorPort`, both already in core/ports, so either placement is clean).
- **Reference example:** `crates/doc-examples/src/http_service_host.rs` already shows the
  `(Paladin, Arc<PaladinExecutionService>)` registry + `execute_agent` handler shape; Epic 1
  generalizes it (port-typed, mutable, with discovery + registration) and moves it into the crate.
- **Error mapping** lives in one helper now so Epic 4 can swap it for the unified model in a single
  place.
- ~~**API surface:** new public items will change `project/current-exports.txt`; regenerate the
  baseline and update `CHANGELOG.md [Unreleased]` as part of the parent-task completion protocol.~~
  **Corrected (dated 2026-08-06, DEBT-01):** The correct baseline path is
  `.project/current-exports.txt`, not the pre-rename path struck above — the directory was renamed
  by commit `928c6d5`. Confirmed via `ls -la .project/current-exports.txt` (442,369 bytes present)
  and `ls` on the struck path ("No such file or directory"), both re-run during this task. The
  `CHANGELOG.md` clause is unaffected.

---

## 8. Success Metrics

1. A test (or the Epic 2 binary, later) can register two agents and `POST /agents/{id}/execute`
   returns each agent's output as JSON — with **zero** lines of bespoke handler/registry code
   written by the consumer.
   > *(superseded — ADR-0037: shipped as `/v1/agents/{id}/execute`)*
2. `paladin-web`'s dependency graph still shows **no `paladin-ai` facade dependency**
   (`cargo tree -p paladin-web` unchanged except for nothing new toward the facade).
3. `cargo test -p paladin-web` passes, including the new handler + concurrency tests; coverage of
   the new modules ≥ 80%.
4. `cargo fmt --check`, `cargo clippy -- -D warnings`, and `make deny` are green.
5. All five routes behave per the status-code table in §6 (verified by tests).

---

## 9. Open Questions

1. **System-prompt exposure:** should `GET /agents/{id}` return the full `system_prompt`, a
   truncated/derived `description`, or omit it entirely? (Default assumption: return a short
   `description` and omit the raw prompt; confirm during implementation.)
   > *(superseded — ADR-0037: shipped as `/v1/agents/{id}`)*
2. **`AgentProvisioner` location:** keep in `paladin-web` (single consumer today) or promote to
   `paladin-ports` now for reuse by future topologies (sidecar/worker)? (Default: `paladin-web`;
   promote only if a second consumer appears.)
3. **Registry id source on `POST /agents`:** is the id always client-supplied in the `AgentSpec`,
   or may the server generate one (e.g. `uuid`) when omitted? (Default: client-supplied, required;
   `409` on duplicate.)
   > *(superseded — ADR-0037: shipped as `/v1/agents`)*
4. **Execute error granularity:** is a single `502` sufficient for all `PaladinError` variants in
   Epic 1, or should timeouts/stop-word/config errors map to distinct codes now? (Default: single
   `502` in Epic 1; refine with the unified error model in Epic 4.)

---

*Next step: run `/generate-tasks` against this PRD to produce
`tasks-agent-registry-execution-api.md` in this `Epic_1/` folder.*
