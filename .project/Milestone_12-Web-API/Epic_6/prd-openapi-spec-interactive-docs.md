# PRD: OpenAPI Specification & Interactive Docs (Milestone 12, Epic 6)

**Project:** Paladin Framework
**Milestone:** 12 — Web API / HTTP Service Host Topology, Out of the Box
**Epic:** 6 — OpenAPI Specification & Interactive Docs
**Version Target:** v0.6.0 (Unreleased)
**Status:** Ready for Implementation
**Created:** 2026-06-09
**Author:** AI Coding Agent (Claude Code)
**Depends on:** Milestone 12 Epics 1, 3, 4, 5 (stable route + DTO + error + auth surface)

---

## 1. Introduction / Overview

The agent HTTP API now has a stable surface — execution, streaming, async jobs, discovery, runtime
registration, health/readiness, a unified error envelope, and authentication — but **no
machine-readable contract**. Consumers must read source or docs to integrate, and nothing guarantees
the implementation matches a published shape.

**This Epic publishes an OpenAPI 3 specification** derived from the handlers and DTOs (via `utoipa` +
`utoipa-axum`), serves it at `GET /openapi.json` with a bundled **Swagger UI** at `GET /docs`,
introduces a **`/v1` version prefix** for the agent API as a stability boundary, and adds a **drift
guard** test (mirroring the `current-exports.txt` API-surface guard) so the committed spec can never
silently diverge from the code.

### Scope decisions (from PRD clarification)

- **Versioning:** move the agent API under **`/v1`** (e.g. `/v1/agents/...`). Operational/docs
  endpoints (`/health`, `/ready`, `/openapi.json`, `/docs`) stay **unversioned**. Since nothing is
  released yet, this is not breaking-in-practice.
- **UI:** bundle **Swagger UI** (`utoipa-swagger-ui`) at `/docs`.
- **Gating:** a **runtime config flag** `http.docs.enabled` (default **true**) controls both the spec
  and the UI; operators disable it in production without a rebuild.
- **Tooling:** **`utoipa` + `utoipa-axum`** — derive schemas on DTOs and define routes through
  `OpenApiRouter` so the router and spec are generated together (drift-resistant).

---

## 2. Goals

1. `GET /openapi.json` returns a valid OpenAPI 3.x document describing every agent-API route, its
   request/response DTOs, the error envelope, and the security schemes — when docs are enabled.
2. `GET /docs` serves an interactive Swagger UI backed by that spec.
3. The agent API is served under `/v1`; operational/docs endpoints remain unversioned; a written
   stability policy explains what `/v1` guarantees.
4. The spec is **generated from code** (no hand-maintained duplicate) and a **drift-guard test**
   fails CI if the committed `openapi.json` baseline no longer matches the generated spec.
5. Docs exposure is controlled by `http.docs.enabled` (default true), so production can disable it.
6. New code compiles warning-free; `fmt`/`clippy -D warnings`/`cargo test` and `make deny` pass; the
   API-surface baseline is updated.

---

## 3. User Stories

- **As an API consumer**, I want an OpenAPI spec so I can generate a typed client and see exactly what
  each endpoint accepts and returns.
- **As a developer**, I want an interactive UI to try endpoints (with my API key) without writing
  curl by hand.
- **As an integrator**, I want a `/v1` prefix and a stability policy so I know which changes are
  breaking.
- **As a maintainer**, I want a drift test so the published contract can't quietly fall out of sync
  with the handlers.
- **As an operator**, I want to disable the docs UI in production with one config flag.

---

## 4. Functional Requirements

### 4.1 Specification generation

1. DTOs **must** derive `utoipa::ToSchema`: `ExecuteRequest`, `ExecuteResponse`, `AgentSummary`,
   `AgentSpec`, `JobRecord`/`JobStatus`, and the error envelope (`ApiError`'s `{ error: { code,
   message, details } }` shape, as a documented schema).
2. Each agent-API handler **must** carry a `#[utoipa::path(...)]` annotation describing method, path,
   parameters, request body, and the response codes it returns (incl. `401`/`403`/`404`/`502`/`504`
   referencing the error schema).
3. Routes **must** be assembled via `utoipa-axum`'s `OpenApiRouter` so the served router and the
   generated spec come from one definition (no second source of truth).
4. The spec **must** declare the **security schemes** from Epic 5: an API-key scheme (`X-API-Key`
   header) and an HTTP bearer (JWT) scheme, and mark the protected operations as secured.
5. The spec **must** include API `info` (title, version aligned to the crate/`v1`), and a server entry
   reflecting the `/v1` base for agent routes.

### 4.2 Serving the spec & UI

6. When docs are enabled, `GET /openapi.json` **must** return the generated document
   (`application/json`).
7. When docs are enabled, `GET /docs` **must** serve a Swagger UI that loads `/openapi.json`.
8. Both **must** be gated by `http.docs.enabled` (default true); when disabled, neither route is
   mounted (requests get `404`).
9. The docs/spec routes **must** be **unauthenticated and unversioned** (a consumer needs the contract
   before they have credentials). They must not leak secrets (the spec describes shapes, not values).

### 4.3 Versioning

10. The agent API routes **must** be served under a `/v1` prefix (`/v1/agents`, `/v1/agents/{id}/...`,
    `/v1/agents/{id}/execute[/stream]`, `/v1/agents/{id}/jobs[/{job_id}]`).
11. `/health`, `/ready`, `/openapi.json`, and `/docs` **must** remain unversioned.
12. A **stability policy** (README/docs) **must** state what `/v1` guarantees (additive,
    non-breaking changes within a major version) and how breaking changes will be introduced
    (`/v2`).
13. The cross-cutting layers (auth, request-log, CORS, body-limit, timeout, rate-limit) and the
    health-route exemption from Epics 4–5 **must** continue to apply correctly under the new
    structure.

### 4.4 Drift guard

14. A committed baseline (e.g. `crates/paladin-web/openapi.json`) **must** hold the canonical spec.
15. A test **must** regenerate the spec and assert byte/structural equality with the baseline,
    failing with a clear "run the update step" message on drift (mirroring `current-exports.txt`).
16. A documented command/`make` target (or `cargo test`-driven update) **must** regenerate the
    baseline.

### 4.5 Config & quality

17. `http.docs.enabled` **must** be added to the web config (lenient serde default = true) and wired
    into `paladin-server`.
18. New public items **must** have rustdoc.
19. Tests **must** cover: spec generation (valid, contains expected paths/schemas/security),
    served-endpoint behavior (`/openapi.json` + `/docs` reachable when enabled, `404` when disabled),
    and the drift guard.

---

## 5. Non-Goals (Out of Scope)

- **Versioning the user-management / content-delivery routes** — `/v1` applies to the agent API
  surface this milestone introduced; the pre-existing user/delivery routes keep their paths (may be
  documented separately later).
- **Multiple spec versions / `/v2`** — only `/v1` exists now; the policy describes how `/v2` would
  arrive.
- **Client SDK generation / publishing** — consumers run their own codegen against the spec.
- **Auth on the docs endpoints** — the contract is public; values/secrets are never in the spec.
- **OpenAPI for SSE event framing** beyond documenting the stream endpoint's content type (SSE bodies
  aren't fully expressible in OpenAPI; describe at a high level).
- **Deployment artifacts / examples** — Epic 7.

---

## 6. Design Considerations

### Route & spec assembly (illustrative)

```text
OpenApiRouter (utoipa-axum)
  └── nest "/v1" → agent OpenApiRouter   (execute, stream, jobs, discovery, register)
                     • #[utoipa::path] per handler
                     • route_layer(require_authentication)   (health stays outside)
  .split_for_parts() → (axum::Router, OpenApi)

Final app:
  Router
    ├── /v1/agents/...          (from the parts router)
    ├── /health, /ready         (unversioned, open)
    ├── /openapi.json           (serde_json of OpenApi)   ── gated by http.docs.enabled
    └── /docs                   (SwaggerUi)               ── gated by http.docs.enabled
  + with_http_layers(...)       (request-log, CORS, body-limit, timeout, rate-limit)
```

### Config (illustrative)

```yaml
http:
  docs:
    enabled: true   # serve /openapi.json + /docs; set false in production
```

### Stability policy (to document)

- `/v1` is additive within the major version: new optional fields, new endpoints, new enum variants
  may be added; existing field meanings and required shapes won't change.
- Breaking changes ship under a new prefix (`/v2`); `/v1` is supported through a deprecation window.

---

## 7. Technical Considerations

- **Crate / layer:** the spec lives in `paladin-web` (it owns the routes + DTOs). A function (e.g.
  `agent_openapi_router()` returning `OpenApiRouter`, plus a public `build_openapi() -> utoipa::openapi::OpenApi`)
  lets both the binary serve it and the drift test serialize it. `paladin-web` gains **no** facade
  dependency.
- **Dependencies:** `utoipa` (5.x; `axum_extras`, plus `chrono`/`uuid` schema features as needed),
  `utoipa-axum` (router integration), `utoipa-swagger-ui` (with the `axum` feature). Confirm versions
  resolve against axum 0.8 / http 1.x at implementation time (`cargo add --dry-run`). Swagger UI
  bundles static assets — keep it behind the runtime flag and note the binary-size impact.
- **Router refactor:** `agent_router` migrates to build an `OpenApiRouter`; `with_http_layers`, the
  auth `route_layer`, and the health merge must continue to work (apply layers to the `split_for_parts`
  axum router; mount `/openapi.json` + `/docs` outside the versioned/auth scope).
- **Reused surface:** the Epic 1–5 DTOs/handlers, `ApiError` (Epic 4), `AgentAuthConfig`/security
  (Epic 5), `with_http_layers` (Epic 4), health routes (Epic 4).
- **`/v1` ripple:** smoke tests, `agent_auth` router tests, and the binary's startup route log must
  move to `/v1/...`. Health/docs stay at root.
- **Determinism:** the drift baseline requires stable serialization (sorted maps / stable ordering
  from utoipa). Pretty-print the JSON and commit it; the update path rewrites it.
- **API surface:** new public items (`ToSchema` derives, the OpenApi builder, `DocsConfig`) will
  change `project/current-exports.txt` — regenerate (additive expected).

---

## 8. Success Metrics

1. `GET /openapi.json` returns a spec that validates as OpenAPI 3.x and lists every `/v1/agents/...`
   operation with request/response schemas, the error envelope, and both security schemes.
2. `GET /docs` renders Swagger UI that can call a `/v1` endpoint with an `X-API-Key`.
3. Agent routes respond under `/v1`; `/health`, `/ready`, `/openapi.json`, `/docs` respond at root.
4. With `http.docs.enabled: false`, `/openapi.json` and `/docs` return `404`; agent routes are
   unaffected.
5. The drift-guard test passes against the committed baseline and **fails** when a route/DTO changes
   without updating it.
6. `cargo test --features web-server`, `fmt`, `clippy --workspace --all-targets -D warnings`, and
   `make deny` are green; API-surface baseline updated.

---

## 9. Open Questions

1. **Baseline location & update path:** commit at `crates/paladin-web/openapi.json`? And expose the
   update via a `make openapi` target, an ignored "update" test, or an env-gated test? (Default:
   `crates/paladin-web/openapi.json` + a documented regen command, matching `extract-public-api.sh`.)
2. **`/openapi.json` gating:** gate the raw spec behind `docs.enabled` together with the UI, or keep
   the JSON always available and gate only the UI? (Default: gate both together.)
3. **Swagger UI in production:** default `docs.enabled = true` everywhere with operators opting out —
   acceptable, or should release builds default it off? (Default: on; operators disable.)
4. **utoipa version & feature set:** exact `utoipa`/`utoipa-axum`/`utoipa-swagger-ui` versions
   compatible with axum 0.8 — confirm at implementation (and whether `utoipa-axum` cleanly expresses
   the auth `route_layer` + health exemption, else fall back to manual `#[derive(OpenApi)]` paths).
5. **SSE endpoint documentation depth:** document `/v1/agents/{id}/execute/stream` as
   `text/event-stream` with a prose description of the `chunk`/`done`/`error` events (OpenAPI can't
   model SSE frames precisely) — sufficient?

---

*Next step: run `/generate-tasks` against this PRD to produce
`tasks-openapi-spec-interactive-docs.md` in this `Epic_6/` folder.*
