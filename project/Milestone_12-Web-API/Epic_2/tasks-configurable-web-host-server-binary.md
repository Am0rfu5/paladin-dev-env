# Tasks: Configurable Web Host & Server Binary (Milestone 12, Epic 2)

**PRD:** [prd-configurable-web-host-server-binary.md](prd-configurable-web-host-server-binary.md)
**Crate:** `paladin-ai` (facade / composition root) — reuses `paladin-web` (Epic 1)
**Status:** Phase 2 — sub-tasks expanded, ready for implementation

---

## Relevant Files

- `src/config/agents.rs` - **New.** `AgentDefinition` config struct (id, model, system_prompt, optional provider/temperature/max_loops/stop_words) + unit tests. Wired into `Settings`.
- `src/config/settings.rs` - **Modify.** Add an `agents: Vec<AgentDefinition>` (or `Option<…>`) field to `Settings`; the bind address reuses the existing `server` (`host`/`port`) section from `src/config/web_server.rs`.
- `src/infrastructure/web/agent_host.rs` - **New.** The registry-from-config builder (`build_agent_registry`) and the per-agent build helper (`build_agent`) shared with the provisioner. Unit tests in-file.
- `src/infrastructure/web/facade_provisioner.rs` - **New.** Concrete `AgentProvisioner` impl (`provision(&AgentSpec)`) reusing `build_agent`. Unit tests in-file.
- `src/infrastructure/web/mod.rs` - **New/Modify.** Module wiring for the two files above.
- `src/bin/paladin-server.rs` - **New.** The `paladin-server` binary: load config → build registry + `AgentApiState` (with provisioner) → `agent_router` → `axum::serve` with graceful shutdown + startup diagnostics.
- `Cargo.toml` - **Modify.** Add `[[bin]] name = "paladin-server"`, `required-features = ["web-server"]`; add any binary-only deps behind that feature if needed (e.g. `tokio` signal — already present).
- `config.example.yml` (or extend `config.yml`) - **Modify/Add.** A documented `host` + `agents` example for the server.
- `tests/paladin_server_smoke.rs` - **New.** Integration smoke test: boot the server on `127.0.0.1:0` with a hermetic mock provider; assert `GET /agents` and `POST /agents/{id}/execute`.
- `CHANGELOG.md` - **Modify.** `[Unreleased]` entry for the configurable host + `paladin-server` binary.

### Notes

- **TDD (Red-Green-Refactor):** write the failing test first for each behavior-bearing sub-task.
- Rust unit tests live in-file under `#[cfg(test)] mod tests { ... }`; the boot smoke test is an integration test in `tests/`.
- Run with `cargo test` (or `cargo test --features web-server` for the server paths). Before
  committing a parent task: `cargo test` → `cargo fmt --check` → `cargo clippy -- -D warnings` → `make deny`.
- **Composition-root rule:** all new code lives in the **facade crate** (`paladin-ai`). It may
  depend on both `paladin-web` and `PaladinExecutionService`/LLM adapters. `paladin-web` gains **no**
  new dependency (Epic 1 is reused unchanged) — verify the dependency direction is preserved.
- **Reused (verified) building blocks:** `Settings` (`src/config/settings.rs`, `ServerConfig` has
  `host`+`port`); the `paladin-llm` provider factory (`create(provider) -> Arc<dyn LlmPort>`,
  `get_default_provider`, `list_available_providers`); `PaladinBuilder`
  (`name/system_prompt/model/temperature/max_loops`); `PaladinExecutionService::new(llm, breaker,
  None, None)`; `CircuitBreaker`; `paladin_web::{AgentRegistry, AgentApiState, AgentProvisioner,
  AgentSpec, ProvisionError, agent_router}`.
- **Out of scope** (later epics): user/auth/delivery routes, auth, garrison/arsenal, streaming,
  health/CORS/error-model, OpenAPI, Docker/k8s/TLS, config hot-reload.

## Tasks

- [ ] 0.0 Create feature branch
  - [ ] 0.1 From an up-to-date base, create and checkout `feature/m12-epic2-configurable-web-host-server-binary`. (Epic 2 depends on Epic 1; branch from the Epic 1 branch if it is not yet merged to `main`, otherwise from `main`.)
  - [ ] 0.2 Confirm a clean baseline: `cargo build --features web-server` and `cargo test -p paladin-web` pass before any changes.

- [ ] 1.0 Add the `agents` configuration schema and wire it into `Settings`
  - [ ] 1.1 Create `src/config/agents.rs` with `AgentDefinition` (`#[derive(Debug, Clone, Serialize, Deserialize)]`): `id`, `model`, `system_prompt` (required); `provider`, `temperature: Option<f32>`, `max_loops: Option<u32>`, `stop_words: Vec<String>` (optional, `#[serde(default)]`). Document each field.
  - [ ] 1.2 **(Test first)** Unit tests: deserialize a full agent YAML/JSON; deserialize a minimal one (only required fields) and assert defaults; confirm the bind address derives from the existing `server` (`host`+`port`) section. (Lenient parsing — see PRD Open Q5; do not add `deny_unknown_fields`.)
  - [ ] 1.3 Add `agents: Vec<AgentDefinition>` to `Settings` (defaulting to empty when absent, per PRD Open Q1) and re-export `AgentDefinition` from the config module. Confirm existing config loading (`config.yml` + `APP_*` env) still works.
  - [ ] 1.4 Update `Settings` defaults/tests so the workspace config tests still pass; rustdoc the new field.

- [ ] 2.0 Build the registry-from-config builder (facade `agent_host`)
  - [ ] 2.1 Create `src/infrastructure/web/` (`mod.rs` + `agent_host.rs`) and wire the module into the facade lib (behind `#[cfg(feature = "web-server")]`).
  - [ ] 2.2 **(Test first)** Unit tests for a per-agent build helper `build_agent(def, provider_factory, breaker) -> Result<(Paladin, Arc<dyn PaladinExecutorPort>), HostBuildError>`: builds a `Paladin` with the def's prompt/model/temperature/max_loops; resolves the provider (agent `provider` → `llm.default_provider`); unknown provider → descriptive error. Use a mock/test provider factory or `MockLlmAdapter`.
  - [ ] 2.3 Implement `build_agent` using `PaladinBuilder` + the `paladin-llm` provider factory + `PaladinExecutionService::new(llm, breaker, None, None)`. Define `HostBuildError` (`thiserror`).
  - [ ] 2.4 **(Test first)** Unit tests for `build_agent_registry(settings, ...) -> Result<AgentRegistry, HostBuildError>`: builds N agents from config; **duplicate id → error** (names the id); provider failure → error (names the agent id).
  - [ ] 2.5 Implement `build_agent_registry`: iterate `settings.agents`, call `build_agent`, insert into a fresh `AgentRegistry`, rejecting duplicates. Make tests pass.
  - [ ] 2.6 Rustdoc all public items; refactor.

- [ ] 3.0 Implement the concrete `AgentProvisioner` (facade) for runtime registration
  - [ ] 3.1 Create `src/infrastructure/web/facade_provisioner.rs` with a `FacadeProvisioner` holding what `build_agent` needs (provider factory handle + shared `Arc<CircuitBreaker>` + default provider).
  - [ ] 3.2 **(Test first)** Unit tests: `provision(&AgentSpec)` returns a `(Paladin, executor)` pair for a valid spec; an unknown provider / bad spec maps to `ProvisionError`. (Map `AgentSpec` → the same build path as `build_agent`.)
  - [ ] 3.3 Implement `#[async_trait] impl AgentProvisioner for FacadeProvisioner`, translating `AgentSpec` into the build helper and mapping `HostBuildError` → `ProvisionError`. Green.
  - [ ] 3.4 Rustdoc; refactor (ensure `build_agent` is the single shared build path for both config-load and runtime provisioning).

- [ ] 4.0 Add the `paladin-server` binary (load → build → serve → graceful shutdown)
  - [ ] 4.1 Add `[[bin]] name = "paladin-server"`, `path = "src/bin/paladin-server.rs"`, `required-features = ["web-server"]` to `Cargo.toml`.
  - [ ] 4.2 Implement `main`: initialize logging/tracing (consistent with `paladin-cli`); load `Settings`; build the registry (`build_agent_registry`); construct `AgentApiState::new(Arc::new(registry)).with_provisioner(Arc::new(FacadeProvisioner::…))`; build `agent_router(state)`.
  - [ ] 4.3 Bind a `TcpListener` to the configured `server` address (`host:port`) and `axum::serve(listener, app).with_graceful_shutdown(shutdown_signal())`.
  - [ ] 4.4 Implement `shutdown_signal()`: await `tokio::signal::ctrl_c()`, and on Unix also a `SIGTERM` stream; return on whichever fires first.
  - [ ] 4.5 Ensure no secrets are logged; return a non-zero exit (via `Result` from `main` or explicit `process::exit`) on startup failure.

- [ ] 5.0 Add startup validation and diagnostics (fail-fast + route/address logging)
  - [ ] 5.1 Validate before serving: parseable bind address; every agent's provider available (`list_available_providers`); required fields present; no duplicate ids — surfacing each as a clear, specific error (reusing `HostBuildError`/`build_agent_registry` errors where possible).
  - [ ] 5.2 On success, log the bound address and a summary (agent route paths + count/ids of agents loaded).
  - [ ] 5.3 **(Test first where feasible)** Unit-test the validation helpers (bad bind address, unknown provider, duplicate id) at the function level (not requiring a live bind).

- [ ] 6.0 Tests: config parsing, builder, provisioner, and a boot smoke integration test
  - [ ] 6.1 Ensure unit coverage from 1.0–3.0 is in place (config parse, `build_agent`, `build_agent_registry`, `provision`).
  - [ ] 6.2 **(Test first)** Add `tests/paladin_server_smoke.rs` (gated `#![cfg(feature = "web-server")]`): build state with a **hermetic mock provider** (e.g. `MockLlmAdapter` or `provider: "mock"`), bind `127.0.0.1:0`, spawn the server, then assert `GET /agents` → `200` and `POST /agents/{id}/execute` → `200` via a real HTTP client (`reqwest`) or `oneshot`. No real network/API calls.
  - [ ] 6.3 Add a graceful-shutdown assertion if feasible (trigger the shutdown signal/handle and confirm the serve task completes); otherwise document why it is covered manually.

- [ ] 7.0 Finalize: sample config, docs, CHANGELOG, and quality gates
  - [ ] 7.1 Add/extend a documented sample config (`config.example.yml` or a commented block in `config.yml`) showing the `host` + `agents` shape and the env-var key requirements.
  - [ ] 7.2 Add a short "Running the server" note (README or the relevant doc) with the `cargo run --bin paladin-server --features web-server` command and required env vars.
  - [ ] 7.3 Run the full gate: `cargo test` (incl. `--features web-server`), `cargo fmt --check`, `cargo clippy --workspace --all-targets -- -D warnings`, `make deny`. Address findings; remove any debug prints.
  - [ ] 7.4 Verify the facade **API-surface check still passes** (binary + `#[cfg(feature)]` infra additions shouldn't change the default library surface; regenerate the baseline only if it legitimately changed).
  - [ ] 7.5 Add a `CHANGELOG.md [Unreleased]` entry (Milestone 12 — Epic 2) describing the configurable host + `paladin-server` binary.
  - [ ] 7.6 Commit with a conventional-commit message referencing Milestone 12 / Epic 2; mark parent tasks complete and **stop for go-ahead**.
