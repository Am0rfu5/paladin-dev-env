# Tasks: Agent Registry & Execution API (Milestone 12, Epic 1)

**PRD:** [prd-agent-registry-execution-api.md](prd-agent-registry-execution-api.md)
**Crate:** `paladin-web` (adapter layer)
**Status:** Phase 2 — sub-tasks expanded, ready for implementation

---

## Relevant Files

- `crates/paladin-web/src/agent_registry.rs` - **New.** The `AgentRegistry` (concurrent map of `id → (Arc<Paladin>, Arc<dyn PaladinExecutorPort>)`) plus the `AgentProvisioner` port and `AgentSpec`/`ProvisionError` types. Unit tests in-file.
- `crates/paladin-web/src/agent_controller.rs` - **New.** Axum handlers for the five routes (`execute`, `list`, `describe`, `register`, `deregister`), the request/response DTOs, the shared `AgentApiState`, the `agent_router(...)` constructor, and the interim error-body helper. Handler + concurrency tests in-file.
- `crates/paladin-web/src/lib.rs` - **Modify.** Declare and document the new `agent_registry` and `agent_controller` modules; re-export the public surface (`AgentRegistry`, `AgentProvisioner`, `AgentSpec`, `agent_router`).
- `crates/paladin-web/src/app.rs` - **Modify.** Merge the agent sub-router into `create_app_router` (or document how it composes alongside the user/auth + delivery routers).
- `crates/paladin-web/Cargo.toml` - **Verify only.** Confirm no new deps needed (axum, serde, serde_json, tokio, async-trait, uuid, chrono present; `tower` + `http-body-util` dev-deps present). Add a dep only if a gap is found.
- `project/current-exports.txt` - **Modify.** Regenerate the public API-surface baseline to include the new exports.
- `CHANGELOG.md` - **Modify.** Add an `[Unreleased]` entry for the agent registry + execution API.

### Notes

- **TDD (Red-Green-Refactor):** every sub-task that adds behavior writes the failing test first.
- Rust unit/handler tests live in the same file under `#[cfg(test)] mod tests { ... }`. Handler
  tests use `tower::ServiceExt::oneshot` with a **mock `PaladinExecutorPort`** and a **mock
  `AgentProvisioner`** (no real LLM calls, no real Paladin construction).
- Run tests with `cargo test -p paladin-web`. Run the full gate before committing the parent task:
  `cargo test` → `cargo fmt --check` → `cargo clippy -- -D warnings` → `make deny`.
- **Architectural guardrail:** `paladin-web` must gain **no dependency on the `paladin-ai`
  facade** — depend only on `PaladinExecutorPort` (`paladin-ports`) and `Paladin`/`PaladinResult`/
  `PaladinError` (`paladin-core`). Verify with `cargo tree -p paladin-web`.
- **Out of scope here** (later epics): auth on routes (Epic 5), SSE/streaming + async jobs
  (Epic 3), health/unified-error-model/tracing/CORS (Epic 4), OpenAPI (Epic 6), config loading +
  server binary + the concrete `AgentProvisioner` impl (Epic 2).

## Tasks

- [x] 0.0 Create feature branch
  - [x] 0.1 Create and checkout `feature/m12-epic1-agent-registry-execution-api`, rooted on `main` (which already contains the fully-merged M8 Epic 7: actix-web removed, deny ban added, axum delivery controller mounted — PR #17). M12 docs re-applied on top.
  - [x] 0.2 Confirm a clean baseline: `cargo build -p paladin-web` and `cargo test -p paladin-web` pass before any changes (build OK; 35 + 5 tests pass).

- [x] 1.0 Define the `AgentRegistry` and `AgentProvisioner` port (`agent_registry.rs`)
  - [x] 1.1 Create `crates/paladin-web/src/agent_registry.rs` and declare it in `lib.rs` (`pub mod agent_registry;`).
  - [x] 1.2 **(Test first)** `#[cfg(test)]` unit tests for the registry: construct-empty, construct-from-initial-list + `list`, `get` (found/`None`, asserts metadata), `insert` (new + duplicate refused), `remove` (found/not-found). Uses an in-test `StubExecutor` impl of `PaladinExecutorPort` and a `Paladin` built via `Node::new` + `PaladinData { .. ..Default::default() }`.
  - [x] 1.3 Define `AgentSpec` (`id`, `name`, `model`, `system_prompt`, optional `temperature`, `stop_words`; optional fields `#[serde(default)]`) and `ProvisionError` (`thiserror`: `InvalidSpec`, `Failed`).
  - [x] 1.4 Define the `AgentProvisioner` port (`#[async_trait]`, returns `(Paladin, Arc<dyn PaladinExecutorPort>)`). Documented that the concrete impl lives in the Epic 2 composition root.
  - [x] 1.5 Implement `AgentRegistry` over `RwLock<HashMap<String, AgentEntry>>` with `new`/`from_agents`/`get`/`contains`/`list`/`insert`/`remove`/`len`/`is_empty`. Lock held only briefly, never across `.await`; `get` returns cloned `Arc`s; poison recovered via `into_inner` (no panics); `insert` returns `false` on duplicate (→ `409`), `remove` returns `bool` (→ `404`).
  - [x] 1.6 Rustdoc on every public item; `cargo fmt`, `cargo clippy -- -D warnings`, and tests all green (40 + 5).

- [x] 2.0 Define request/response DTOs and `AgentApiState` (`agent_controller.rs`)
  - [x] 2.1 Create `crates/paladin-web/src/agent_controller.rs`; declare it in `lib.rs`.
  - [x] 2.2 Define DTOs: `ExecuteRequest { input }`, `ExecuteResponse { output, token_count, execution_time_ms, loop_count, stop_reason }` (+ `From<PaladinResult>` and a stable `stop_reason` label), and `AgentSummary { id, name, model, description }` via `AgentSummary::from_agent(id, &Paladin)` (id lives in the registry key, so a `From<&Paladin>` can't carry it). The summary previews only the first line of the system prompt and **omits secrets / the full prompt** per PRD §4.3 / Open Q1 (note: secrets never live on `PaladinData` anyway).
  - [x] 2.3 Define `AgentApiState { registry: Arc<AgentRegistry>, provisioner: Option<Arc<dyn AgentProvisioner>> }` (`#[derive(Clone)]`, with `new`/`with_provisioner`).
  - [~] 2.4 **Error-body helper deferred to 3.0.** Implementing `ok_body`/`error_body`/`execution_error_response` here would be dead code until a handler consumes them, which fails `clippy -D warnings`. Moved to task 3.0 (first consumer = the execute handler), with the `{ "error": ... }` render test alongside. 4 DTO/summary unit tests added in 2.0 instead.

- [x] 3.0 Implement the execution endpoint `POST /agents/{id}/execute`
  - [x] 3.0a **(Moved from 2.4)** Implemented the interim error helpers (`JsonValue` alias, `ok_body`, `error_body`, `execution_error_response` → `502`) mirroring `delivery_controller.rs`, centralized so Epic 4 can swap them in one place. Added the `{ "error": "<message>" }` render test.
  - [x] 3.1 **(Test first)** Added a reusable mock `PaladinExecutorPort` (`MockExecutor::{Succeeds,Fails}`). `oneshot`/direct handler tests: success → `200` + `ExecuteResponse` (asserts output + all metadata fields), unknown id → `404`, executor `Err` → `502` (asserts message), invalid body → `400` (via router `oneshot`).
  - [x] 3.2 Implemented `execute_agent(State, Path(id), Json(ExecuteRequest))`: `registry.get(id)` → `404`; `executor.execute(paladin.as_ref(), &input)`; `Ok` → `200` `ExecuteResponse`, `Err` → `502` via `execution_error_response`. No `unwrap`/`expect`/`panic!`.
  - [x] 3.3 Rustdoc on handler + helpers; `fmt`, `clippy -D warnings` (plain + `--all-targets`), tests all green (49 + 5).

- [x] 4.0 Implement the discovery endpoints `GET /agents` and `GET /agents/{id}`
  - [x] 4.1 **(Test first)** Tests: `GET /agents` → `200` with an array of summaries (order-independent id assertion) + empty-registry → `[]`; `GET /agents/{id}` → `200` for a known id and `404` for unknown. A `LEAK_CANARY` second prompt line asserts **the raw system prompt never appears** in either response body.
  - [x] 4.2 Implemented `list_agents(State)` → `registry.list()` mapped to `AgentSummary` → `200 [..]`, and `describe_agent(State, Path(id))` → summary or `404`. Green.
  - [x] 4.3 Rustdoc on both handlers; `fmt`, `clippy -D warnings` (plain + `--all-targets`), tests all green (53 + 5).

- [x] 5.0 Implement the runtime registration endpoints `POST /agents` and `DELETE /agents/{id}`
  - [x] 5.1 **(Test first)** Added a mock `AgentProvisioner` (`Succeeds`/`Fails`). Tests: `POST /agents` success → `201` + summary and the agent is afterward retrievable via `describe_agent`; duplicate id → `409`; provision failure → `422` (asserts message); invalid body → `400` (router `oneshot`); **no provisioner wired** → `501`.
  - [x] 5.2 **(Test first)** `DELETE /agents/{id}` tests: known id → `204` and subsequently `404` on describe; unknown id → `404`.
  - [x] 5.3 Implemented `register_agent(State, Json(AgentSpec))`: `provisioner` `None` → `501`; early duplicate check → `409`; `provision(&spec)` → `Ok` insert (race-safe re-check → `409`) + `201`, `Err` → `422`; invalid body → `400` via extractor.
  - [x] 5.4 Implemented `deregister_agent(State, Path(id))` returning `Result<StatusCode, (StatusCode, JsonValue)>` so `204` has a truly empty body; missing id → `404`.
  - [x] 5.5 Rustdoc on both handlers; `fmt`, `clippy -D warnings` (plain + `--all-targets`), tests all green (60 + 5).

- [x] 6.0 Compose and mount the agent router; export the public surface (`lib.rs`, `app.rs`)
  - [x] 6.1 Implemented `agent_router(state: AgentApiState) -> Router`: the five routes via method chaining (`/agents` GET+POST, `/agents/{id}` GET+DELETE, `/agents/{id}/execute` POST) with `.with_state(...)`. No auth layer (Epic 5); handler signatures kept layer-compatible.
  - [x] 6.2 **(Test first)** Added `agent_router_merges_with_other_routes_without_conflict`: builds `agent_router` and `merge`s it with a user/auth-style placeholder router, asserting both `GET /agents` and `POST /users/login` resolve `200` (no path/state clash).
  - [x] 6.3 Wired into `app.rs` via a **sibling** `create_app_router_with_agents(user_service, auth_port, deliverer, agent_state)` = `create_app_router(..).merge(agent_router(..))`. Keeps the existing `create_app_router` signature (and facade re-export) unchanged — non-breaking.
  - [x] 6.4 Updated `lib.rs`: documented the two new modules and added root re-exports (`AgentApiState`, `AgentSummary`, `ExecuteRequest`, `ExecuteResponse`, `agent_router`, `AgentRegistry`, `AgentProvisioner`, `AgentSpec`, `ProvisionError`). `#![warn(missing_docs)]` passes. Updated the stale module doc.

- [ ] 7.0 Add concurrency tests and verify the architectural/dependency guardrails
  - [ ] 7.1 **(Test first)** Concurrency test: spawn concurrent `tokio` tasks doing `execute`/`list` reads while another registers and removes agents; assert no deadlock/panic and consistent results (use `tokio::test(flavor = "multi_thread")`).
  - [ ] 7.2 Verify `cargo tree -p paladin-web` shows **no `paladin-ai` facade** dependency and no new web framework; verify `Cargo.toml` is unchanged except any justified additions.
  - [ ] 7.3 Run `make deny` to confirm the dependency guardrails still pass.

- [ ] 8.0 Finalize: docs, API-surface baseline, CHANGELOG, and quality gates
  - [ ] 8.1 Run the full gate: `cargo test` (workspace), `cargo fmt --check`, `cargo clippy -- -D warnings`. Address all findings.
  - [ ] 8.2 Remove any debug prints / temporary code; confirm coverage of the new modules is ≥ 80% (e.g. via the project's coverage target if available).
  - [ ] 8.3 Regenerate `project/current-exports.txt` to capture the new public exports.
  - [ ] 8.4 Add a `CHANGELOG.md [Unreleased]` entry describing the agent registry + execution API.
  - [ ] 8.5 Commit with a conventional-commit message referencing Milestone 12 / Epic 1, then mark the parent tasks complete and **stop for go-ahead**.
