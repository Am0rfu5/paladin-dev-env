# Tasks: OpenAPI Specification & Interactive Docs (Milestone 12, Epic 6)

**PRD:** [prd-openapi-spec-interactive-docs.md](prd-openapi-spec-interactive-docs.md)
**Crate:** `paladin-web` (spec/UI/routes) + facade config/binary wiring
**Base:** `main` (Milestone 12 Epics 1–5 merged — PRs #19, #21, #22, #23, #24).
**Status:** Phase 2 — sub-tasks expanded, ready for implementation

---

## Relevant Files

- `crates/paladin-web/Cargo.toml` - **Modify.** Add `utoipa` (5.x), `utoipa-axum` (0.2), `utoipa-swagger-ui` (9.x, `axum` feature).
- `crates/paladin-web/src/agent_controller.rs` - **Modify.** `#[utoipa::path]` on each handler; `#[derive(ToSchema)]` on `ExecuteRequest`/`ExecuteResponse`/`AgentSummary`; migrate `agent_router` to a `utoipa-axum` `OpenApiRouter` and `/v1` nesting; keep auth `route_layer` + health exemption.
- `crates/paladin-web/src/agent_registry.rs` - **Modify.** `#[derive(ToSchema)]` on `AgentSpec`.
- `crates/paladin-web/src/job_store.rs` - **Modify.** `#[derive(ToSchema)]` on `JobRecord`/`JobStatus`.
- `crates/paladin-web/src/error.rs` - **Modify.** A documented `ToSchema` for the error envelope (`{ error: { code, message, details } }`).
- `crates/paladin-web/src/openapi.rs` - **New.** `build_openapi() -> utoipa::openapi::OpenApi` (title/version, security schemes: `X-API-Key` + bearer JWT), and helpers to serve `/openapi.json` + Swagger UI.
- `crates/paladin-web/src/agent_auth.rs` - **Modify (maybe).** Security-scheme name constants reused by `#[utoipa::path(security(...))]`.
- `crates/paladin-web/src/lib.rs` - **Modify.** Declare/doc `openapi`; re-export the builder + `DocsConfig` if surfaced here.
- `crates/paladin-web/openapi.json` - **New.** Committed spec baseline for the drift guard.
- `crates/paladin-web/src/http_layers.rs` - **Modify (maybe).** Ensure `with_http_layers` composes cleanly with the `/v1` + docs routers.
- `src/config/agents.rs` - **Modify.** `DocsConfig { enabled }` on `WebHttpConfig` (default true).
- `src/bin/paladin-server.rs` - **Modify.** Mount `/v1` agent API + (gated) `/openapi.json` + `/docs`; update the startup route log to `/v1`.
- `tests/paladin_server_smoke.rs` - **Modify.** Move agent paths to `/v1/...`; add spec/docs reachability + docs-disabled cases.
- `config.example.yml` / `README.md` / `CHANGELOG.md` / `project/current-exports.txt` - **Modify.** Document + record; add the versioning/stability policy.

### Notes

- **TDD:** write the failing test first for each behavior-bearing sub-task.
- Run with `cargo test --features web-server`. Before committing a parent task: `cargo test` →
  `cargo fmt --check` → `cargo clippy -- -D warnings` → `make deny`.
- **Verified deps:** `utoipa` 5.5, `utoipa-axum` 0.2, `utoipa-swagger-ui` 9.0.2 resolve against
  axum 0.8 / http 1.x.
- **Hexagonal:** all spec/UI code in `paladin-web`; no facade dependency. The binary only mounts +
  gates what `paladin-web` builds.
- **Unversioned & open:** `/health`, `/ready`, `/openapi.json`, `/docs` stay at root and
  unauthenticated; only the agent API moves under `/v1` (and stays behind auth).
- **`/v1` ripple:** existing smoke + `agent_auth` router tests and the binary route log move to
  `/v1/...`; health/docs stay at root.
- **Out of scope:** versioning the user/delivery routes, `/v2`, client SDK generation, auth on docs,
  deployment artifacts (Epic 7).

## Tasks

- [x] 0.0 Create feature branch
  - [x] 0.1 Branched `feature/m12-epic6-openapi-docs` from `main` (Epics 1–5 merged).
  - [x] 0.2 Baseline confirmed: `cargo build --features web-server` + `cargo test -p paladin-web` (109) green before changes.

- [x] 1.0 Add OpenAPI tooling and schema the DTOs + error envelope (`ToSchema`)
  - [x] 1.1 Added `utoipa = "5"` (`axum_extras`/`chrono`/`uuid`), `utoipa-axum = "0.2"`, `utoipa-swagger-ui = "9"` (`axum`) to `paladin-web`.
  - [x] 1.2 Derived `ToSchema` on `ExecuteRequest`/`ExecuteResponse`/`AgentSummary`, `AgentSpec` (`allowed_roles` via `#[schema(value_type = Vec<String>)]` to avoid `UserRole` needing utoipa in core), and `JobRecord`/`JobStatus` (`result` via `value_type = Object, nullable`).
  - [x] 1.3 **(Test first)** Added `ApiErrorBody { error: ApiErrorDetail { code, message, details } }` (`ToSchema`) mirroring `ApiError::to_body()`; test round-trips a real `ApiError` body through the schema type 1:1.
  - [x] 1.4 `cargo build -p paladin-web` clean; rustdoc on new items; paladin-web 110 + auth_rbac 5 pass; fmt/clippy clean.

- [x] 2.0 Annotate handlers and migrate the agent router to `utoipa-axum` `OpenApiRouter`
  - [x] 2.1 Added `#[utoipa::path(...)]` to all 8 handlers (unprefixed `/agents/...` paths, params, request bodies, response codes referencing `ApiErrorBody`). The stream endpoint documents `text/event-stream` with a prose note on the `chunk`/`done`/`error` events.
  - [x] 2.2 Tagged protected operations with `security(("api_key" = []), ("jwt" = []))` (string literals — utoipa requires literals, not consts); health/docs carry none.
  - [x] 2.3 Added `agent_openapi_router(state) -> OpenApiRouter` (`routes!(...)`, auth `route_layer`, state) and rebuilt `agent_router` on top of it via `split_for_parts()` + health merge. Paths remain at `/agents` for now (the `/v1` prefix is task 3.0).
  - [x] 2.4 Existing router tests stay green (construction unchanged externally); added `openapi_spec_contains_agent_operation_paths` asserting the assembled `OpenApi.paths` include all six route templates.
  - [x] 2.5 `fmt`/`clippy --all-targets -D warnings` clean; paladin-web 111 + auth_rbac 5 + smoke 2 pass. (Freed a full disk via `cargo clean` after the swagger-ui assets bloated `target/`.)

- [x] 3.0 Introduce the `/v1` version prefix (agent API versioned; health/docs unversioned)
  - [x] 3.1 `agent_router` now nests `agent_openapi_router` under `API_V1_PREFIX = "/v1"` and `split_for_parts()`s into the axum `Router` (+ discarded `OpenApi`), then merges the unversioned `health_routes` at root.
  - [x] 3.2 **(Test first)** `agent_api_is_versioned_under_v1`: `GET /v1/agents` → `200`, `GET /agents` → `404`, `/health` → `200`; existing router/auth tests moved to `/v1/agents`.
  - [x] 3.3 `with_http_layers` composes unchanged; the global-timeout exemption (`path.ends_with("/execute/stream")`) still matches `/v1/agents/{id}/execute/stream` — no change needed (the existing layer test still passes).
  - [x] 3.4 Moved `tests/paladin_server_smoke.rs` agent paths to `/v1/...` (health/ready at root); updated the binary startup route log to `/v1`. `fmt`/`clippy` clean; paladin-web 112 + smoke 2 pass.

- [x] 4.0 Build + serve the spec (`/openapi.json`) and Swagger UI (`/docs`), gated by `http.docs.enabled`
  - [x] 4.1 Created `crates/paladin-web/src/openapi.rs`: `build_openapi(state)`/`openapi_spec()` assemble the `/v1` document (via the shared `versioned_agent_parts`) and `decorate` it with info (title/version/description) + the `api_key` (header `X-API-Key`) and `jwt` (bearer) security schemes. `pub mod openapi` in `lib.rs`.
  - [x] 4.2 Added `DocsConfig { enabled }` (default true) to `WebHttpConfig` + exported via `config::mod`.
  - [x] 4.3 `docs_router(spec)` mounts `SwaggerUi::new("/docs").url("/openapi.json", spec)` — serves both the spec and the UI; merged only when enabled.
  - [x] 4.4 `paladin-server` builds the `/v1` app, merges `docs_router(build_openapi(state))` when `http.docs.enabled`, applies `with_http_layers`, and logs the docs posture.
  - [x] 4.5 **(Test first)** Tests: spec has info + both security schemes + `/v1` paths; `docs_router` serves `/openapi.json` (`200`) + `/docs/`; without it, `/openapi.json` → `404` and `/health` still `200`. Verified by boot: `/openapi.json` → `200 application/json` (`title: "Paladin Agent API"`), `/docs/` → `200`. paladin-web 116; fmt/clippy clean.

- [ ] 5.0 Spec drift guard (committed `openapi.json` baseline + regenerate-and-compare test)
  - [ ] 5.1 Generate and commit `crates/paladin-web/openapi.json` (pretty-printed `build_openapi()` output).
  - [ ] 5.2 **(Test first)** Add a drift test: serialize `build_openapi()` and assert equality with the committed baseline, failing with a clear "regenerate the baseline" message on mismatch.
  - [ ] 5.3 Document the regeneration path (a `make openapi` target or an env-gated update test, mirroring `extract-public-api.sh`); ensure deterministic ordering so the baseline is stable.
  - [ ] 5.4 fmt/clippy; gates.

- [ ] 6.0 Tests: spec generation, served endpoints, docs-disabled `404`, `/v1` routing, drift guard
  - [ ] 6.1 Confirm coverage from 1.0–5.0 (schema shape, operation paths present, security schemes present, `/v1` routing, docs gating, drift).
  - [ ] 6.2 **(Test first)** Extend `tests/paladin_server_smoke.rs`: with docs enabled, `GET /openapi.json` returns a spec whose `paths` include `/v1/agents/{id}/execute` and whose components declare the security schemes; `GET /docs` returns `200`.

- [ ] 7.0 Finalize: config sample, README (docs + versioning/stability policy), CHANGELOG, API baseline, gates
  - [ ] 7.1 `config.example.yml`: add `http.docs.enabled` with a note (disable in production if desired).
  - [ ] 7.2 `README.md`: document `/openapi.json` + `/docs`, the `/v1` prefix, and the **versioning/stability policy** (additive within `/v1`; breaking → `/v2`).
  - [ ] 7.3 Full gate: `cargo test --features web-server`, `cargo fmt --check`, `cargo clippy --workspace --all-targets --features web-server -- -D warnings`, `make deny`. Remove debug prints.
  - [ ] 7.4 Regenerate `project/current-exports.txt` (new `ToSchema` impls, the OpenApi builder, `DocsConfig`) — review the diff (additive expected).
  - [ ] 7.5 Add a `CHANGELOG.md [Unreleased]` entry (Milestone 12 — Epic 6): OpenAPI spec, Swagger UI, `/v1` versioning + policy, drift guard. **Note the `/v1` route move.**
  - [ ] 7.6 Commit referencing Milestone 12 / Epic 6; mark parent tasks complete and **stop for go-ahead**.
