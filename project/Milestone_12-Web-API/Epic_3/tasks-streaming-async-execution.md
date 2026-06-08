# Tasks: Streaming & Asynchronous Execution (Milestone 12, Epic 3)

**PRD:** [prd-streaming-async-execution.md](prd-streaming-async-execution.md)
**Crates:** `paladin-ports` (new port), `paladin-ai` facade (streaming impl + wiring), `paladin-web` (SSE, jobs, timeouts)
**Status:** Phase 2 — sub-tasks expanded, ready for implementation
**Base:** `main` (Milestone 12 Epics 1 & 2 are merged — PRs #19, #20)

---

## Relevant Files

- `crates/paladin-ports/src/output/streaming_executor_port.rs` - **New.** `StreamingExecutorPort` trait (`execute_stream → PaladinStream`). Declared in `output/mod.rs`.
- `src/application/services/paladin/paladin_execution_service.rs` - **Modify.** `impl StreamingExecutorPort for PaladinExecutionService` over `LlmPort::generate_stream`. In-file unit tests with a mock LLM.
- `crates/paladin-web/src/agent_registry.rs` - **Modify.** `AgentEntry` gains an optional `Arc<dyn StreamingExecutorPort>`; `insert`/`get`/`from_agents` evolve additively (3-arg insert preserved).
- `crates/paladin-web/src/agent_controller.rs` - **Modify.** SSE handler (`execute_agent_stream`), job handlers, timeout application, new routes in `agent_router`, `AgentApiState` (job store + timeout config).
- `crates/paladin-web/src/job_store.rs` - **New.** Thread-safe in-memory job store (status + result, bounded retention). Unit tests in-file.
- `crates/paladin-web/Cargo.toml` - **Modify.** Add `tokio-stream` and `futures` (adapt `PaladinStream` → axum SSE).
- `src/infrastructure/web/agent_host.rs` - **Modify.** Attach the streaming handle in `build_agent`; thread timeout config.
- `src/infrastructure/web/facade_provisioner.rs` - **Modify.** Attach the streaming handle for runtime-registered agents.
- `src/config/agents.rs` / `src/config/settings.rs` - **Modify.** Optional per-agent `timeout_seconds`; default + max timeout config.
- `src/bin/paladin-server.rs` - **Modify.** Pass timeout config + job-store settings into `AgentApiState`.
- `tests/paladin_server_smoke.rs` - **Modify/Extend.** End-to-end streaming + job round-trip over real HTTP (mock provider).
- `config.example.yml` / `README.md` / `CHANGELOG.md` - **Modify.** Document streaming, jobs, timeout config.

### Notes

- **TDD (Red-Green-Refactor):** write the failing test first for each behavior-bearing sub-task.
- Rust unit tests live in-file under `#[cfg(test)]`; HTTP/boot tests in `tests/`. Run with
  `cargo test --features web-server`. Before committing a parent task: `cargo test` →
  `cargo fmt --check` → `cargo clippy -- -D warnings` → `make deny`.
- **Additive constraint:** do **not** change `PaladinExecutorPort` or buffered behavior. The
  streaming port is new; the registry's streaming handle is optional; Epic 1/2 tests must stay green.
- **Hexagonal:** the streaming port lives in `paladin-ports`; `paladin-web` depends on it, **not**
  the facade. The streaming impl + wiring live in the facade (composition root).
- **Reused (verified):** `LlmPort::generate_stream` (adapters implement it), `PaladinStreamChunk`/
  `PaladinStream` (`mpsc::Receiver`) in `paladin-ports`, axum 0.8 SSE (`axum::response::sse`),
  `tokio::time::timeout` for cancellation.
- **Out of scope** (later epics): auth (5), health/CORS/error-model (4), OpenAPI (6), Docker/k8s (7),
  durable/distributed jobs (queue-worker topology), garrison/arsenal.

## Tasks

- [x] 0.0 Create feature branch
  - [x] 0.1 Updated `main` (now contains M12 Epics 1 & 2 — PRs #19, #21 merged) and created `feature/m12-epic3-streaming-async-execution` from it. Epic 3 PRD/tasks committed.
  - [x] 0.2 Clean baseline confirmed: `cargo build --features web-server` OK; `infrastructure::web` tests (12) and the boot smoke test pass.

- [x] 1.0 Add the `StreamingExecutorPort` trait (`paladin-ports`)
  - [x] 1.1 Created `crates/paladin-ports/src/output/streaming_executor_port.rs`; declared `pub mod streaming_executor_port;` in `output/mod.rs`.
  - [x] 1.2 Defined `#[async_trait] pub trait StreamingExecutorPort` with `execute_stream(&self, &Paladin, &str) -> Result<PaladinStream, PaladinError>`, reusing the existing `PaladinStream`/`PaladinStreamChunk`. Full rustdoc + a passing `no_run` doc-test. `PaladinExecutorPort` untouched.
  - [x] 1.3 `cargo test -p paladin-ports --doc` passes; `clippy -D warnings` + `fmt` clean.

- [x] 2.0 Implement real `execute_stream` on `PaladinExecutionService` (facade)
  - [x] 2.1 **(Test first)** Unit test with `MockLlmAdapter` (whose `generate_stream` emits a content delta + final marker): `execute_stream` yields chunks whose concatenated `text` equals the buffered output (`"Mock LLM response"`) and ends with `is_final = true`.
  - [x] 2.2 Implemented `impl StreamingExecutorPort for PaladinExecutionService`: composes the prompt (mirrors the buffered no-history path), opens `LlmPort::generate_stream`, and a spawned task forwards each delta as a `PaladinStreamChunk` over an `mpsc` channel (final chunk when `finish_reason` is present, or a synthesized terminal chunk if the stream ends), delivers mid-stream errors as `Err(PaladinError)`, and stops on receiver drop (`tx.send` error → return).
  - [x] 2.3 An unsupported provider errors **up front** (`generate_stream(...).await?` before spawning), never hangs; the web layer's fallback decision (Open Q2) is handled in task 4.
  - [x] 2.4 Rustdoc; `fmt`/`clippy -D warnings` clean; the streaming test + all 10 execution-service tests pass (buffered `execute` unchanged).

- [ ] 3.0 Thread an optional streaming handle through the registry and Epic 2 wiring
  - [ ] 3.1 **(Test first)** Update `agent_registry` tests: register an agent with and without a streaming handle; `get` returns the optional handle; existing buffered tests stay green.
  - [ ] 3.2 Replace the `AgentEntry` tuple with a small struct carrying `paladin`, `executor: Arc<dyn PaladinExecutorPort>`, `streamer: Option<Arc<dyn StreamingExecutorPort>>` (and, reserved for task 5, an optional per-agent `timeout`). Keep the 3-arg `insert` working (defaults `streamer`/`timeout` to `None`) and add an `insert_streaming`/builder variant; update `get`/`list`/`from_agents`.
  - [ ] 3.3 Wire `infrastructure::web::agent_host::build_agent`: construct one `Arc<PaladinExecutionService>` and register it as **both** `Arc<dyn PaladinExecutorPort>` and `Arc<dyn StreamingExecutorPort>` (clone the `Arc`), so config-loaded agents stream.
  - [ ] 3.4 Wire `FacadeProvisioner` the same way so runtime-registered (`POST /agents`) agents stream.
  - [ ] 3.5 Rustdoc; gates; verify Epic 1/2 tests + the boot smoke test still pass.

- [ ] 4.0 Add the SSE streaming endpoint `POST /agents/{id}/execute/stream`
  - [ ] 4.1 Add `tokio-stream` + `futures` to `crates/paladin-web/Cargo.toml` (adapt `PaladinStream` `mpsc::Receiver` → axum SSE `Stream`).
  - [ ] 4.2 **(Test first)** Handler tests: streaming-capable agent → collect SSE events, assert `chunk` events then a `done` event whose assembled text matches; unknown id → `404`; invalid body → `400`; agent **without** a streamer → pseudo-stream the buffered result as one `chunk` + `done` (Open Q2 default).
  - [ ] 4.3 Implement `execute_agent_stream(State, Path(id), Json(ExecuteRequest)) -> impl IntoResponse` returning `axum::response::sse::Sse`: drive the streamer (or buffered fallback), mapping `PaladinStreamChunk` → `Event` (`chunk`) and a terminal `done` event; mid-stream `Err` → an `error` event then close; client disconnect drops the stream (cancels the producer).
  - [ ] 4.4 Mount `POST /agents/{id}/execute/stream` in `agent_router`.
  - [ ] 4.5 Rustdoc; gates.

- [ ] 5.0 Add timeouts & cancellation (default → per-agent → per-request, clamped) across buffered/stream/job
  - [ ] 5.1 **(Test first)** Config + resolution tests: parse per-agent `timeout_seconds`; a `TimeoutPolicy { default, max }`; a pure `resolve_timeout(request, agent, policy)` helper (precedence + clamp to max; non-positive request → invalid).
  - [ ] 5.2 Add config: `AgentDefinition.timeout_seconds: Option<u64>`; default + max timeout in `Settings` (new `timeouts` section or `server` fields); add optional `timeout_seconds` to the execute / stream / job request bodies. Store the resolved per-agent timeout in the registry `AgentEntry` and the `TimeoutPolicy` in `AgentApiState`.
  - [ ] 5.3 **(Test first → impl)** Buffered: wrap `executor.execute` in `tokio::time::timeout`; on expiry cancel (drop the future) and return `504` with the standard error body. Test with a deliberately slow mock.
  - [ ] 5.4 Apply the same timeout to the SSE stream (emit a terminal `error`/timeout event and drop the producer) — test the timeout path.
  - [ ] 5.5 Thread `TimeoutPolicy` from config into `AgentApiState` (builder + `paladin-server`). Rustdoc; gates.

- [ ] 6.0 Add in-process async jobs (job store + `POST /agents/{id}/jobs` + `GET …/jobs/{job_id}`)
  - [ ] 6.1 **(Test first)** `job_store` unit tests: create (`pending`/`running`) → update (`completed`/`failed`/`timed_out`); `get` unknown → `None`; bounded retention evicts oldest + logs.
  - [ ] 6.2 Implement `crates/paladin-web/src/job_store.rs`: `JobId` (uuid), `JobStatus` enum, `JobRecord` (status + optional `ExecuteResponse`/error), `JobStore` (thread-safe map with a configurable retention cap).
  - [ ] 6.3 **(Test first)** Handler tests: `POST /agents/{id}/jobs` → `202` + `job_id`; the spawned task completes and `GET` returns `completed` + result equal to a buffered call; unknown agent → `404`; unknown job → `404`; slow mock + low timeout → `timed_out`.
  - [ ] 6.4 Implement `enqueue_job` (validate agent + body; create job; spawn a task running `executor.execute` under the resolved timeout; update the store) and `get_job` (status/result or `404`). Add `Arc<JobStore>` to `AgentApiState`.
  - [ ] 6.5 Mount `POST /agents/{id}/jobs` + `GET /agents/{id}/jobs/{job_id}` in `agent_router`. Rustdoc; gates.

- [ ] 7.0 Tests: streaming, timeout, and job integration + boot smoke round-trip
  - [ ] 7.1 Confirm unit coverage from 1.0–6.0 is in place (stream impl, registry, SSE, timeout resolution + 504, job store + lifecycle).
  - [ ] 7.2 **(Test first)** Extend `tests/paladin_server_smoke.rs` (real HTTP, mock provider): stream round-trip (collect SSE events → assembled output) and job round-trip (`202` → poll → `completed` with result).
  - [ ] 7.3 Add a timeout integration assertion if feasible (slow mock agent → `504` / terminal timeout event); otherwise document the unit-level coverage.

- [ ] 8.0 Finalize: config sample, docs, CHANGELOG, API baseline, and quality gates
  - [ ] 8.1 Update `config.example.yml`: `timeouts` (default/max) + a per-agent `timeout_seconds` example.
  - [ ] 8.2 Update `README.md` (and/or the deployment doc): streaming endpoint, async jobs, and timeout behavior.
  - [ ] 8.3 Full gate: `cargo test --features web-server`, `cargo fmt --check`, `cargo clippy --workspace --all-targets --features web-server -- -D warnings`, `make deny`. Remove any debug prints.
  - [ ] 8.4 Regenerate `project/current-exports.txt` (new default-surface items: `StreamingExecutorPort`, config timeout fields, registry/state additions) — confirm additive-only.
  - [ ] 8.5 Add a `CHANGELOG.md [Unreleased]` entry (Milestone 12 — Epic 3): streaming, timeouts/cancellation, async jobs.
  - [ ] 8.6 Commit with a conventional-commit message referencing Milestone 12 / Epic 3; mark parent tasks complete and **stop for go-ahead**.
