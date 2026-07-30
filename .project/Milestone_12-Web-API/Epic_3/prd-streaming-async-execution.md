# PRD: Streaming & Asynchronous Execution (Milestone 12, Epic 3)

**Project:** Paladin Framework
**Milestone:** 12 — Web API / HTTP Service Host Topology, Out of the Box
**Epic:** 3 — Streaming & Asynchronous Execution
**Version Target:** v0.6.0 (Unreleased)
**Status:** Ready for Implementation
**Created:** 2026-06-08
**Author:** AI Coding Agent (Claude Code)
**Depends on:** Milestone 12 Epic 1 (agent registry & execution API) and Epic 2 (config host & server binary)

---

## 1. Introduction / Overview

Epics 1–2 deliver a runnable agent HTTP API, but every call is **buffered**: the client waits for
the whole result, and a long run holds an open request with no way to bound it. This Epic adds the
three things a real agent API needs for non-trivial workloads:

1. **Token streaming** — incremental output over Server-Sent Events (SSE), so clients render
   tokens as they are produced.
2. **Timeouts & cancellation** — bounded execution at the config, per-agent, and per-request
   level, aborting in-flight work on expiry.
3. **Asynchronous jobs** — a fire-and-poll API for long runs that shouldn't hold a connection.

### The streaming gap this Epic closes

Today there is **no per-agent streaming path**. `LlmPort::generate_stream` exists (implemented by
the OpenAI / Anthropic / DeepSeek / mock adapters) and `PaladinStreamChunk` / `PaladinStream`
(`= mpsc::Receiver<Result<PaladinStreamChunk, PaladinError>>`) are defined in `paladin-ports`. But
`PaladinExecutionService` exposes no `execute_stream`, and Epic 1's registry stores
`Arc<dyn PaladinExecutorPort>`, which has only `execute`. This Epic implements real streaming on the
execution service and threads it through the registry **additively**.

### Scope decisions (from PRD clarification)

- **Streaming depth:** **true token streaming** — implement `execute_stream` over
  `LlmPort::generate_stream` and emit real incremental tokens over SSE.
- **Registry impact:** **additive** — a new, focused streaming port; registry entries gain an
  *optional* streaming handle. `PaladinExecutorPort` and the existing buffered path are unchanged.
- **Async jobs:** **included** — an in-process, in-memory fire-and-poll job API.
- **Timeouts:** config **default → per-agent override → per-request override (clamped to a server
  max)**, with **true cancellation** (abort the in-flight future / close the stream) → `504`.

---

## 2. Goals

1. `POST /agents/{id}/execute/stream` streams real, incremental LLM tokens to the client over SSE,
   ending with a terminal event carrying final metadata.
2. `PaladinExecutionService` gains a working `execute_stream` built on `LlmPort::generate_stream`.
3. Streaming is threaded through the registry **without changing** `PaladinExecutorPort` or any
   Epic 1/2 buffered behavior; agents without a streaming backend degrade gracefully.
4. Every execution path (buffered, streaming, job) honors a timeout resolved as request → agent →
   config-default, clamped to a server maximum, and **cancels** the underlying work on expiry,
   returning a `504`-style error.
5. `POST /agents/{id}/jobs` enqueues an in-process job and returns a job id; `GET
   /agents/{id}/jobs/{job_id}` reports status and (when finished) the result.
6. All new code compiles warning-free and passes `cargo fmt`/`clippy -D warnings`/`cargo test`;
   streaming, timeout, and job paths have unit + integration coverage.

---

## 3. User Stories

- **As a UI developer**, I want to stream an agent's tokens as they're generated, so users see
  output appear live instead of waiting for the full response.
- **As an integrator**, I want a long-running agent call to be bounded by a timeout that actually
  stops the work, so a stuck LLM call can't pin a worker indefinitely.
- **As an operator**, I want a sensible default timeout I can override per agent (and let trusted
  callers shorten per request), so I can tune latency/cost without redeploying.
- **As a batch client**, I want to submit a long job, get an id immediately, and poll for the
  result, so I don't have to hold an HTTP connection for minutes.
- **As an API client**, I want streaming and job responses to use the same stable error envelope as
  the buffered API, so error handling is uniform.

---

## 4. Functional Requirements

### 4.1 Streaming executor port (`paladin-ports`)

1. The system **must** define a focused streaming port (e.g. `StreamingExecutorPort`) in
   `paladin-ports`:
   ```rust
   #[async_trait]
   pub trait StreamingExecutorPort: Send + Sync {
       async fn execute_stream(&self, paladin: &Paladin, input: &str)
           -> Result<PaladinStream, PaladinError>;
   }
   ```
   It **must not** modify `PaladinExecutorPort` (the buffered port stays as-is).

### 4.2 Real streaming on the execution service (facade)

2. `PaladinExecutionService` **must** implement `StreamingExecutorPort::execute_stream`, driving
   `LlmPort::generate_stream` and forwarding incremental text as `PaladinStreamChunk`s over a
   `PaladinStream` (`mpsc` channel), ending with a final chunk (`is_final = true`) carrying summary
   metadata (token/loop counts where available).
3. Errors mid-stream **must** be delivered as `Err(PaladinError)` items on the stream (not a panic),
   and the channel **must** close cleanly when the client disconnects or the timeout fires.
4. If a provider/model does not support streaming, `execute_stream` **must** surface a clear error
   (or a single-chunk fallback — see Open Q2), never hang.

### 4.3 Registry & state threading (additive — `paladin-web` + facade wiring)

5. `paladin-web` registry entries **must** carry an **optional** streaming handle, e.g.
   `AgentEntry = (Arc<Paladin>, Arc<dyn PaladinExecutorPort>, Option<Arc<dyn StreamingExecutorPort>>)`.
   Existing buffered behavior and the existing `insert`/`get`/`list` ergonomics **must** be
   preserved (e.g. a 3-arg `insert` stays valid via a convenience that defaults the streaming handle
   to `None`, or an `insert_streaming` variant).
6. The Epic 2 builder (`build_agent`/`build_agent_registry`) and `FacadeProvisioner` **must** attach
   the streaming handle (the same `PaladinExecutionService` instance, which now implements both
   ports) so config-loaded and runtime-registered agents stream.
7. An agent with **no** streaming handle **must** behave per Open Q2 (reject with a clear error, or
   pseudo-stream the buffered result) — chosen behavior documented and tested.

### 4.4 SSE streaming endpoint (`paladin-web`)

8. The system **must** add `POST /agents/{id}/execute/stream` returning `Content-Type:
   text/event-stream`, driven by the agent's `StreamingExecutorPort`.
9. It **must** emit one SSE event per chunk (incremental `text`) and a terminal event marking
   completion (with final metadata), and map: unknown id → `404`; invalid body → `400`; an agent
   without streaming → per FR7; mid-stream failure → an SSE error event then close.
10. It **must** handle client disconnect by dropping the stream and cancelling the underlying work.
11. The route **must** mount via `agent_router` alongside the existing routes and remain
    **unauthenticated** in this milestone (auth → Epic 5).

### 4.5 Timeouts & cancellation

12. The system **must** support a configurable **default** execution timeout, an optional
    **per-agent** override (in the agent config / `AgentSpec`), and an optional **per-request**
    override (in the request body), with resolution order request → agent → default and the
    effective value **clamped to a server maximum**.
13. Timeouts **must** apply to buffered execute, streaming, and jobs. On expiry the system **must**
    **cancel** the in-flight work (drop the future / close the stream) and return a `504`-style
    error (`{ "error": ... }` for buffered/job; a terminal error event for SSE).
14. A per-request timeout above the server max **must** be clamped (not rejected), and a
    non-positive value **must** be treated as invalid (`400`).

### 4.6 Asynchronous job execution (in-process — `paladin-web`)

15. The system **must** add `POST /agents/{id}/jobs`: validate the agent + body, enqueue an
    in-process job (spawned task running the buffered execute under the resolved timeout), and
    return `202 Accepted` with a generated `job_id`.
16. The system **must** add `GET /agents/{id}/jobs/{job_id}` returning the job's status
    (`pending`/`running`/`completed`/`failed`/`timed_out`) and, when finished, the `ExecuteResponse`
    or error. Unknown job id → `404`.
17. Jobs **must** be stored in an in-memory, thread-safe job store. Jobs are **ephemeral** (lost on
    restart) and **must not** require external infrastructure (distributed execution remains the
    [queue/worker topology](../../../docs/src/deployment-topologies/queue-worker.md)).
18. The job store **should** bound growth (e.g. cap retained completed jobs / TTL) and **must** log
    if it drops records — see Open Q3.

### 4.7 Quality & tests

19. Every new public item **must** have rustdoc.
20. Unit/handler tests **must** cover: `execute_stream` assembling chunks → full output (mock LLM);
    SSE endpoint success (collect events → output) + `404`/`400`/no-streaming paths; timeout →
    `504`/terminal error with cancellation; job lifecycle (`202` → poll `running` → `completed`
    with result; unknown job → `404`; job timeout → `timed_out`).
21. The Epic 2 boot smoke test (or a new one) **should** exercise the streaming endpoint and a job
    round-trip end-to-end over real HTTP with a mock provider (no network/keys).

---

## 5. Non-Goals (Out of Scope)

- **Distributed / durable jobs**, retries, or backpressure — that is the
  [queue/worker topology](../../../docs/src/deployment-topologies/queue-worker.md); Epic 3 jobs are
  in-process and ephemeral.
- **Authentication / authorization** on the new routes — Epic 5.
- **Health/readiness, CORS, rate limiting, unified error model** — Epic 4 (this Epic reuses the
  interim `{ "error": ... }` body and adds an SSE error event).
- **OpenAPI / Swagger** for the new routes — Epic 6.
- **Garrison/arsenal** for agents — still out (LLM + prompt only).
- **WebSocket / bidirectional streaming** — SSE only.
- Changing `PaladinExecutorPort` or the buffered execution behavior.

---

## 6. Design Considerations

### Route additions

| Method | Path | Purpose | Success | Errors |
|--------|------|---------|---------|--------|
| `POST` | `/agents/{id}/execute/stream` | Stream tokens (SSE) | `200` event-stream | `404`, `400`, `504` (terminal event) |
| `POST` | `/agents/{id}/jobs` | Enqueue async job | `202` `{ job_id }` | `404`, `400` |
| `GET`  | `/agents/{id}/jobs/{job_id}` | Poll job | `200` status/result | `404` |

### SSE event shape (illustrative)

```text
event: chunk
data: {"text":"Hel"}

event: chunk
data: {"text":"lo, world"}

event: done
data: {"output":"Hello, world","token_count":12,"execution_time_ms":840,"stop_reason":"completed"}
```
Mid-stream failure emits `event: error` with `{ "error": "..." }`, then the stream closes.

### Job lifecycle

```text
POST /agents/{id}/jobs ──► 202 { job_id }   (spawn task: execute under timeout)
                              │
GET .../jobs/{job_id} ──► { status: "running" }
                              │ (task completes / fails / times out)
GET .../jobs/{job_id} ──► { status: "completed", result: { output, ... } }
                          { status: "failed",    error: "..." }
                          { status: "timed_out", error: "..." }
```

### Timeout resolution

```text
effective = clamp( request.timeout_seconds
                   ?? agent.timeout_seconds
                   ?? config.default_timeout_seconds,
                   max = config.max_timeout_seconds )
```

---

## 7. Technical Considerations

- **Crates / layers:**
  - `paladin-ports`: new `StreamingExecutorPort` (uses existing `Paladin`, `PaladinStream`,
    `PaladinError`).
  - facade (`paladin-ai`): `impl StreamingExecutorPort for PaladinExecutionService` over
    `LlmPort::generate_stream`; Epic 2 `agent_host`/`FacadeProvisioner` attach the streaming handle;
    timeout config fields.
  - `paladin-web`: optional streaming handle on `AgentEntry`/`AgentApiState`; the SSE handler; the
    job store + job handlers; timeout application.
- **Dependencies:** axum's SSE (`axum::response::sse::{Sse, Event}`) is available in the 0.8 dep.
  Adapting `PaladinStream` (`mpsc::Receiver`) into an axum SSE `Stream` likely needs
  `tokio-stream` (`ReceiverStream`) and/or `futures` in `paladin-web` — add as needed.
- **Cancellation:** wrap execution in `tokio::time::timeout`; for streaming, dropping the
  `Sse`/response future drops the receiver and ends the producer task. Ensure the producer task
  observes channel closure and stops calling the LLM.
- **Config:** add timeout fields to the `host`/`server` or a new `timeouts` section in `Settings`,
  plus an optional `timeout_seconds` on `AgentDefinition` and on the execute/stream/job request
  bodies; reuse Epic 2's lenient-parsing approach.
- **Registry evolution:** the `AgentEntry` tuple change touches Epic 1's `paladin-web` registry and
  Epic 2's builder/provisioner — acceptable because Epic 3 stacks on those branches. Keep the
  buffered call sites working (default streaming handle to `None`).
- **No new `paladin-web` → facade dependency**; the streaming port lives in `paladin-ports`.

---

## 8. Success Metrics

1. A client `POST`ing to `/agents/{id}/execute/stream` receives multiple incremental `chunk` events
   followed by a `done` event whose assembled text equals the buffered `execute` output (verified
   with a multi-response mock LLM).
2. A request that exceeds the effective timeout is cancelled and returns `504` (buffered/job) or a
   terminal `error` event (SSE) — verified by a test with a deliberately slow mock.
3. `POST /agents/{id}/jobs` returns `202` + id; polling transitions `running` → `completed` with the
   same result a buffered call would produce; unknown job → `404`; timed-out job → `timed_out`.
4. Buffered `execute` and all Epic 1/2 tests still pass unchanged (additive, no regressions).
5. `cargo test` (incl. streaming/job integration tests), `fmt`, `clippy -D warnings`, and `make
   deny` are green; the facade API-surface check passes (new public items recorded as additive).

---

## 9. Open Questions

1. **Per-request timeout field placement:** add `timeout_seconds` to the shared request body for
   all three endpoints, or only execute/stream? (Default: all three; jobs included.)
2. **No-streaming-backend behavior (FR7):** for an agent whose executor doesn't implement
   `StreamingExecutorPort`, should the SSE endpoint return `501`/`409`, or **pseudo-stream** the
   buffered result as a single `chunk` + `done`? (Default assumption: pseudo-stream so the endpoint
   always works; document clearly.)
3. **Job retention:** cap by count, by TTL, or both — and what's the default? (Default assumption: a
   bounded LRU of recent completed jobs with a configurable cap; log evictions.)
4. **Stream metadata fidelity:** `LlmPort::generate_stream` may not provide per-chunk token/loop
   counts; is best-effort metadata on the terminal event acceptable? (Default: yes.)
5. **`execute_stream` on the port vs a second method:** confirm a *separate* `StreamingExecutorPort`
   (chosen) over reusing the existing richer `PaladinPort` (which bundles `execute` + `validate` +
   `execute_stream`) — the focused port keeps the registry's optional handle minimal.

---

*Next step: run `/generate-tasks` against this PRD to produce
`tasks-streaming-async-execution.md` in this `Epic_3/` folder.*
