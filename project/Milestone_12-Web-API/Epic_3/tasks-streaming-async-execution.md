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

- [x] 3.0 Thread an optional streaming handle through the registry and Epic 2 wiring
  - [x] 3.1 **(Test first)** Added `insert_with_streaming_attaches_streamer` (with/without streamer); updated existing registry tests to the struct entry; all buffered tests stay green.
  - [x] 3.2 Replaced the `AgentEntry` tuple with a struct `{ paladin, executor, streamer: Option<Arc<dyn StreamingExecutorPort>> }` (timeout slot reserved for task 5). Kept the 3-arg `insert` (delegates with `streamer = None`), added `insert_with_streaming`, and updated `get`/`list`/`from_agents`. Added a `ProvisionedAgent` return type for the provisioner and updated `AgentProvisioner::provision` + its re-export; updated the `execute`/`describe`/`register` handlers + `MockProvisioner`.
  - [x] 3.3 `build_agent_with_llm` now constructs one `Arc<PaladinExecutionService>` and returns it as **both** `Arc<dyn PaladinExecutorPort>` and `Arc<dyn StreamingExecutorPort>` (`BuiltAgent` triple); `build_agent_registry` registers via `register_built`/`insert_with_streaming`, so config-loaded agents stream.
  - [x] 3.4 `FacadeProvisioner::provision` returns `ProvisionedAgent { paladin, executor, streamer }` from the shared `build_agent` path, so runtime-registered agents stream too.
  - [x] 3.5 Rustdoc updated; `fmt`/`clippy --all-targets -D warnings` clean; paladin-web (63+5), facade `infrastructure::web` (12), and the boot smoke test all pass.

- [x] 4.0 Add the SSE streaming endpoint `POST /agents/{id}/execute/stream`
  - [x] 4.1 Added `futures` + `tokio-stream` to `crates/paladin-web/Cargo.toml` (`ReceiverStream` adapts `PaladinStream` → an axum SSE body).
  - [x] 4.2 **(Test first)** Handler tests: streaming agent → asserts `chunk` events (`{"text":"Hel"}`, `{"text":"lo"}`) then `done`; agent **without** a streamer → pseudo-streams the buffered result as `chunk` + `done` (Open Q2 default); unknown id → `404`. (Invalid body → `400` already covered by the extractor on the buffered route.)
  - [x] 4.3 Implemented `execute_agent_stream(...) -> Response`: drives the agent's `StreamingExecutorPort` mapping `PaladinStreamChunk` → `Event` (`chunk`/`done`/`error`) via `chunk_to_event`, or falls back to a buffered single `chunk` + `done`; up-front execution failure → `502`; client disconnect drops the receiver (cancels the producer).
  - [x] 4.4 Mounted `POST /agents/{id}/execute/stream` in `agent_router`; updated the module doc route table.
  - [x] 4.5 Rustdoc; `fmt`/`clippy --all-targets -D warnings` clean; paladin-web tests 66 + 5 pass.

- [x] 5.0 Add timeouts & cancellation (default → per-agent → per-request, clamped) across buffered/stream/job
  - [x] 5.1 **(Test first)** New `crate::timeout` module: `TimeoutPolicy { default_secs, max_secs }` + pure `resolve_timeout(request, agent, policy)` (precedence request→agent→default, clamp to `[1, max]`, `Some(0)` request → `InvalidTimeout`). 5 unit tests.
  - [x] 5.2 Config: `AgentDefinition.timeout_seconds` + `AgentSpec.timeout_seconds` + `ExecuteRequest.timeout_seconds` (all `#[serde(default)]`); `AgentTimeoutsConfig { default_seconds, max_seconds }` + `Settings.timeouts`. Registry `AgentEntry.timeout_secs` + `insert_entry`; `AgentApiState.timeouts` + `with_timeouts`.
  - [x] 5.3 **(Test first → impl)** Buffered `execute_agent` wraps `executor.execute` in `tokio::time::timeout` → `504` on expiry (future dropped = cancelled); invalid `timeout_seconds: 0` → `400`. Tests: `execute_times_out_with_504`, `execute_zero_timeout_is_400`.
  - [x] 5.4 SSE stream bounded by `timed_event_stream` (async-stream; races each chunk vs a deadline) → terminal `error` event + drop producer; buffered fallback wrapped in `timeout` → `504`. Test: `stream_times_out_with_terminal_error_event`. (Swapped `tokio-stream`→`async-stream` for the deadline race.)
  - [x] 5.5 `paladin-server` builds `TimeoutPolicy` from `settings.timeouts` and passes it via `AgentApiState::with_timeouts`. Rustdoc; `fmt`/`clippy -D warnings` (lib+bins+all-targets) clean; paladin-web 74+5, facade config 48 + infra::web 12 + smoke 1 all pass.

- [x] 6.0 Add in-process async jobs (job store + `POST /agents/{id}/jobs` + `GET …/jobs/{job_id}`)
  - [x] 6.1 **(Test first)** `job_store` unit tests: create → `Running`; transitions to `Completed`/`Failed`/`TimedOut`; `get` unknown → `None`; bounded retention evicts oldest (+ logs); update-after-eviction is a no-op. 5 tests.
  - [x] 6.2 Implemented `crates/paladin-web/src/job_store.rs`: uuid `job_id`, `JobStatus` (`running`/`completed`/`failed`/`timed_out`), `JobRecord { status, result: Option<Value>, error }` (result kept as JSON to stay decoupled from the controller), `JobStore` (RwLock map + insertion-order deque, capacity cap, poison-safe).
  - [x] 6.3 **(Test first)** Handler tests: `POST …/jobs` → `202` + `job_id`, poll → `completed` with `result.output`; unknown agent → `404`; unknown job → `404`; slow mock + 1s timeout → `timed_out`.
  - [x] 6.4 Implemented `enqueue_job` (agent lookup `404`, timeout resolve `400`, `create` job, spawn task running `execute` under `tokio::time::timeout`, record complete/fail/time_out) and `get_job` (record or `404`). Added `jobs: Arc<JobStore>` to `AgentApiState`.
  - [x] 6.5 Mounted `POST /agents/{id}/jobs` + `GET /agents/{id}/jobs/{job_id}`; updated module doc table; re-exported `JobStore`/`JobStatus`/`JobRecord`. Rustdoc; `fmt`/`clippy -D warnings` clean; paladin-web 83 + 5, facade infra::web 12 + smoke 1 pass.

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
