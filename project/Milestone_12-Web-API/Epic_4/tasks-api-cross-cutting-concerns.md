# Tasks: API Cross-Cutting Concerns (Milestone 12, Epic 4)

**PRD:** [prd-api-cross-cutting-concerns.md](prd-api-cross-cutting-concerns.md)
**Crate:** `paladin-web` (adapter) + facade config/binary wiring
**Base:** `main` (Milestone 12 Epics 1–3 merged — PRs #19, #21, #22)
**Status:** Phase 2 — sub-tasks expanded, ready for implementation

---

## Relevant Files

- `crates/paladin-web/src/error.rs` - **New.** `ApiError` (code/message/details + status) implementing `IntoResponse` → nested `{ "error": { … } }`; constructors per status. Unit tests in-file.
- `crates/paladin-web/src/agent_controller.rs` - **Modify.** Adopt `ApiError`; remove interim `ok_body`/`error_body`/`execution_error_response`; SSE `error` event uses the envelope; update tests (`body["error"]["message"]`).
- `crates/paladin-web/src/delivery_controller.rs` - **Modify.** Replace its `ok_body`/`error_body` with `ApiError`; update tests.
- `crates/paladin-web/src/user_controller.rs` - **Modify.** Migrate `user_error_to_response` / `ApiResponse`-based errors to `ApiError`; update tests.
- `crates/paladin-web/src/health.rs` - **New.** `GET /health` + `GET /ready` handlers (+ a small router). Tests in-file.
- `crates/paladin-web/src/request_log.rs` - **New.** Request-logging middleware: request-id (honour/generate), `x-request-id` response header, one `log` line per request (method/path/status/latency).
- `crates/paladin-web/src/http_layers.rs` - **New.** `HttpLayersConfig` (CORS, body limit, global timeout, rate limit) + `with_http_layers(router, config)` composer wiring tower-http + tower-governor + the request-log + health routes; the global timeout excludes the streaming route.
- `crates/paladin-web/src/lib.rs` - **Modify.** Declare/doc new modules; re-export `ApiError`, `HttpLayersConfig`, health/router helpers.
- `crates/paladin-web/src/app.rs` - **Modify.** Apply `with_http_layers` / health in `create_app_router_with_agents` (or document the composer the binary calls).
- `crates/paladin-web/Cargo.toml` - **Modify.** Add `tower`, `tower-http` (features `cors`,`limit`,`timeout`), `tower-governor`.
- `src/config/web.rs` (or extend `agents.rs`/`settings.rs`) - **Modify.** A web/http config section (CORS, body-limit, global-timeout, rate-limit) on `Settings`.
- `src/bin/paladin-server.rs` - **Modify.** Map the config into `HttpLayersConfig` and apply the layers.
- `tests/paladin_server_smoke.rs` - **Modify.** Assert `/health` + `/ready` respond and an error uses the nested envelope.
- `config.example.yml` / `README.md` / `CHANGELOG.md` / `project/current-exports.txt` - **Modify.** Document + record the additions.

### Notes

- **TDD (Red-Green-Refactor):** failing test first for each behavior-bearing sub-task.
- Unit/handler tests in-file under `#[cfg(test)]`; HTTP/boot tests in `tests/`. Run with
  `cargo test --features web-server`. Before committing a parent task: `cargo test` →
  `cargo fmt --check` → `cargo clippy -- -D warnings` → `make deny`.
- **Breaking change:** the error body moves from flat `{ "error": "<message>" }` (and the user
  controller's `ApiResponse`) to nested `{ "error": { code, message, details } }`. All existing
  `paladin-web` error-asserting tests must be updated.
- **Hexagonal:** all middleware/error code in `paladin-web`; config values are mapped from
  `Settings` into a `paladin-web` config struct by the binary (the Epic 3 `TimeoutPolicy` seam).
  `paladin-web` must gain **no** dependency on the `paladin-ai` facade.
- **Streaming caveat:** the global request-timeout layer must **not** apply to
  `POST /agents/{id}/execute/stream` (long-lived SSE); scope it to non-streaming routes or leave it
  off by default. Streaming stays bounded by Epic 3's per-execution timeout.
- **Out of scope** (later/other epics): auth (5), OpenAPI (6), Docker/k8s (7), metrics/Prometheus,
  per-route/identity rate limits, migrating the workspace to `tracing`.

## Tasks

- [x] 0.0 Create feature branch
  - [x] 0.1 Updated `main` (Epics 1–3 merged) and created `feature/m12-epic4-api-cross-cutting-concerns`; Epic 4 PRD/tasks committed.
  - [x] 0.2 Clean baseline confirmed: `cargo build --features web-server` OK; `paladin-web` 83 + 5 tests pass.

- [x] 1.0 Add the unified error model (`ApiError`) in `paladin-web`
  - [x] 1.1 Created `crates/paladin-web/src/error.rs`; declared `pub mod error;` in `lib.rs`.
  - [x] 1.2 **(Test first)** 3 unit tests: nested envelope with `details: null` when absent; `with_details` rendered; every constructor maps to the expected status + `code`. (Chose `details: null` over omission — matches the PRD example and gives clients a stable key.)
  - [x] 1.3 Defined `ApiError { status, code, message, details }` + `IntoResponse` + `to_body()` (for SSE reuse). Constructors: `bad_request`/`not_found`/`conflict`/`unprocessable`/`payload_too_large`/`too_many_requests`/`not_implemented`/`bad_gateway`/`gateway_timeout`/`internal`. Rustdoc complete.
  - [x] 1.4 Re-exported `ApiError` from `lib.rs`; build + clippy + fmt clean; 3 tests pass.

- [x] 2.0 Adopt `ApiError` across all controllers (agent, user, delivery) + SSE error event
  - [x] 2.1 Updated agent-controller tests to assert the nested envelope via `err.status()` + `err.to_body()["error"]["code"|"message"]` across `404`/`400`/`502`/`409`/`422`/`501`/`504`.
  - [x] 2.2 Migrated `agent_controller`: fallible handlers now return `Result<_, ApiError>` (using `?`/`ok_or_else`/`map_err`); removed `error_body`/`execution_error_response`; kept `ok_body` (success, `null` fallback). Status mapping preserved.
  - [x] 2.3 SSE `error` events (mid-stream + timeout) now carry the `ApiError` envelope via `to_body()`; stream tests still assert `event: error` + `timed out`.
  - [x] 2.4 Migrated `delivery_controller` (handlers → `Result<_, ApiError>`) and `user_controller` (`user_error_to_response` → `ApiError` with granular snake_case codes; inline not-founds + `ensure_self_or_admin` → `ApiError`; success path keeps `ApiResponse<T>`). Updated all affected tests.
  - [x] 2.5 Refreshed the module docs (no more flat-shape notes); `cargo test -p paladin-web` (85 lib + 5 auth_rbac), facade infra::web (12) + smoke (1) green; `clippy --all-targets -D warnings` + `fmt` clean.

- [x] 3.0 Add health & readiness endpoints (`GET /health`, `GET /ready`)
  - [x] 3.1 Created `crates/paladin-web/src/health.rs`; declared `pub mod health;` in `lib.rs`.
  - [x] 3.2 **(Test first)** Tests: `health()` → `200 { "status": "ok" }`; `ready(State)` → `200 { "status": "ready", "agents": N }` (N from `registry.len()`); no network I/O.
  - [x] 3.3 Implemented `health`/`ready` handlers + `health_routes(state)`; `agent_router` now merges them so the probes serve alongside the agent routes.
  - [x] 3.4 Rustdoc; `fmt`/`clippy --all-targets -D warnings` clean; 2 tests pass.

- [x] 4.0 Add the request-logging middleware (request-id + `x-request-id`)
  - [x] 4.1 Created `crates/paladin-web/src/request_log.rs`; declared `pub mod request_log;` in `lib.rs`.
  - [x] 4.2 **(Test first)** Tests via a small router + `oneshot`: response carries an `x-request-id`; a well-formed inbound `x-request-id` is echoed; `is_acceptable_request_id` rejects empty/spaced/over-long ids.
  - [x] 4.3 Implemented `request_log` (`from_fn`): honours a well-formed inbound id else generates a `uuid`, logs `request_id=… METHOD PATH STATUS Nms` via `log::info!`, sets the `x-request-id` response header. No secrets/bodies logged.
  - [x] 4.4 Rustdoc; `fmt`/`clippy --all-targets -D warnings` clean; 3 tests pass.

- [x] 5.0 Add edge layers: CORS, body limit, global timeout (excl. streaming), rate limiting
  - [x] 5.1 Added `tower = "0.5"`, `tower-http = "0.6"` (`cors`,`limit`,`timeout`), `tower_governor = "0.8"` (`axum`) — all compatible with axum 0.8 / http 1.x.
  - [x] 5.2 Defined `HttpLayersConfig` (`cors_allow_origins`, `body_limit_bytes` [1 MiB], `global_timeout_secs` [0 = disabled], `rate_limit { enabled[false], per_second, burst }`) + `RateLimitConfig`, both `Default`.
  - [x] 5.3 **(Test first)** 4 tests: CORS preflight sets `access-control-allow-origin`; oversized body → `413`; rate-limit enabled (1 rps / burst 1, keyed by `x-real-ip`) → second request `429`; disabled → passthrough.
  - [x] 5.4 Built `cors_layer` (permissive when unset, else explicit origins), `body_limit_layer`, and `apply_rate_limit` (optional `GovernorLayer` + `SmartIpKeyExtractor`, only when enabled; invalid config logged + skipped). Global timeout config carried for the composer (task 6) to apply scoped to non-streaming routes.
  - [x] 5.5 `429` renders via `ApiError` (`GovernorLayer::error_handler` → `ApiError::too_many_requests`). `413` uses tower-http's status (body is its default; documented). `fmt`/`clippy --all-targets -D warnings` clean; 94 + 5 tests pass.

- [ ] 6.0 Compose the layers (`with_http_layers`) and wire config into `paladin-server`
  - [ ] 6.1 Create `crates/paladin-web/src/http_layers.rs` with `pub fn with_http_layers(router: Router, config: &HttpLayersConfig) -> Router` applying request-logging + CORS + body-limit + rate-limit (+ health routes) uniformly, keeping the streaming route clear of the global timeout.
  - [ ] 6.2 **(Test first)** A composition test: a router wrapped by `with_http_layers` still serves an agent route and `/health`, and carries `x-request-id`.
  - [ ] 6.3 Add a web/http config section to `Settings` (facade): CORS, body-limit, global-timeout, rate-limit. Update `Settings::default()` + the `user_config` test fixture.
  - [ ] 6.4 In `paladin-server`: map `Settings` → `HttpLayersConfig` and apply `with_http_layers` to the composed router; log the enabled layers on startup.
  - [ ] 6.5 Rustdoc; gates.

- [ ] 7.0 Tests: probes, error envelope, request-id, CORS preflight, body-limit, 429 + boot-smoke extension
  - [ ] 7.1 Confirm unit/handler coverage from 1.0–6.0 is in place (error envelope, health, request-id, CORS, body-limit, 429, composition).
  - [ ] 7.2 **(Test first)** Extend `tests/paladin_server_smoke.rs`: assert `GET /health` and `GET /ready` respond with the documented shapes, an `x-request-id` header is present, and an error response (unknown agent) uses the nested envelope.

- [ ] 8.0 Finalize: config sample, docs, CHANGELOG, API baseline, and quality gates
  - [ ] 8.1 Update `config.example.yml`: a documented `http`/web section (CORS, body limit, global timeout, rate limit off-by-default).
  - [ ] 8.2 Update `README.md`: health/readiness endpoints, the error envelope, request-id, and the configurable layers.
  - [ ] 8.3 Full gate: `cargo test --features web-server`, `cargo fmt --check`, `cargo clippy --workspace --all-targets --features web-server -- -D warnings`, `make deny`. Remove any debug prints.
  - [ ] 8.4 Regenerate `project/current-exports.txt` (new `ApiError`/`HttpLayersConfig`/health items + config fields) — review the diff (error-shape change is internal to `paladin-web`; config additions are additive default-surface).
  - [ ] 8.5 Add a `CHANGELOG.md [Unreleased]` entry (Milestone 12 — Epic 4), **calling out the breaking error-body change**.
  - [ ] 8.6 Commit referencing Milestone 12 / Epic 4; mark parent tasks complete and **stop for go-ahead**.
