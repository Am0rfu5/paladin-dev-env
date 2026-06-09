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

- [ ] 2.0 Annotate handlers and migrate the agent router to `utoipa-axum` `OpenApiRouter`
  - [ ] 2.1 Add `#[utoipa::path(...)]` to each agent handler (`execute_agent`, `execute_agent_stream`, `list_agents`, `describe_agent`, `register_agent`, `deregister_agent`, `enqueue_job`, `get_job`): method, path (the unprefixed `/agents/...` form), params, request body, and response codes (`200`/`201`/`202`/`204`/`400`/`401`/`403`/`404`/`409`/`422`/`502`/`504`) referencing the error schema. Document the stream endpoint as `text/event-stream` with a prose note on the `chunk`/`done`/`error` events.
  - [ ] 2.2 Tag protected operations with `security(...)` referencing the API-key + bearer schemes (defined in 4.1); leave none on health/docs.
  - [ ] 2.3 Migrate `agent_router` to build a `utoipa_axum::router::OpenApiRouter` via `routes!(...)`, preserving the auth `route_layer` and the health-route exemption (health merged outside the secured/spec scope). Provide an internal accessor that yields the agent `OpenApiRouter` for nesting/spec assembly.
  - [ ] 2.4 **(Test first)** Keep the existing router tests green (adjust to the new construction); add a test that the assembled `OpenApi` contains the expected agent operation paths.
  - [ ] 2.5 fmt/clippy; `cargo test -p paladin-web`.

- [ ] 3.0 Introduce the `/v1` version prefix (agent API versioned; health/docs unversioned)
  - [ ] 3.1 Nest the agent `OpenApiRouter` under `/v1` (so routes serve at `/v1/agents/...` and the spec records the `/v1` paths); `split_for_parts()` into the axum `Router` + `OpenApi`. Merge the unversioned `health_routes` at root.
  - [ ] 3.2 **(Test first)** Router tests: `GET /v1/agents` resolves (auth applies), `GET /agents` → `404`, `/health` + `/ready` resolve at root.
  - [ ] 3.3 Ensure `with_http_layers` still composes (request-log/CORS/body-limit/timeout/rate-limit) over the `/v1` + health router; the global-timeout streaming exemption still matches `/v1/agents/{id}/execute/stream` (update the suffix match if needed).
  - [ ] 3.4 Update `tests/paladin_server_smoke.rs` agent paths to `/v1/...`; update the binary startup route log. fmt/clippy; gates.

- [ ] 4.0 Build + serve the spec (`/openapi.json`) and Swagger UI (`/docs`), gated by `http.docs.enabled`
  - [ ] 4.1 Create `crates/paladin-web/src/openapi.rs`: `build_openapi() -> utoipa::openapi::OpenApi` assembling the nested `/v1` paths + components, with `info` (title/version) and the two `SecurityScheme`s (`X-API-Key` apiKey-in-header, bearer JWT). Declare `pub mod openapi` in `lib.rs`.
  - [ ] 4.2 Add `DocsConfig { enabled }` (default true) to `WebHttpConfig` (`src/config/agents.rs`) + export; update `Settings` default usage paths as needed.
  - [ ] 4.3 Provide a `paladin-web` helper to mount the docs routes (`GET /openapi.json` serving the serialized spec + `SwaggerUi` at `/docs`) and a way for the composer/binary to include them only when enabled.
  - [ ] 4.4 Wire `paladin-server`: build the `/v1` app, conditionally mount the docs routes when `http.docs.enabled`, apply `with_http_layers`, and log whether docs are served.
  - [ ] 4.5 **(Test first)** Router tests: docs enabled → `GET /openapi.json` is `200 application/json` and `GET /docs` is reachable; docs disabled → both `404` while `/v1/agents` still works. fmt/clippy.

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
