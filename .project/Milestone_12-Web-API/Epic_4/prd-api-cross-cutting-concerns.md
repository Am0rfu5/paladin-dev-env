# PRD: API Cross-Cutting Concerns (Milestone 12, Epic 4)

**Project:** Paladin Framework
**Milestone:** 12 — Web API / HTTP Service Host Topology, Out of the Box
**Epic:** 4 — API Cross-Cutting Concerns
**Version Target:** v0.6.0 (Unreleased)
**Status:** Ready for Implementation
**Created:** 2026-06-08
**Author:** AI Coding Agent (Claude Code)
**Depends on:** Milestone 12 Epics 1–3 (agent registry/API, server binary, streaming/jobs) — all merged or in review

---

## 1. Introduction / Overview

The agent HTTP API now executes agents (buffered, streaming, async jobs) but lacks the
**production middleware** a real service needs: health probes, a consistent error contract,
request logging, CORS, body-size limits, and rate limiting. Today each controller renders ad-hoc
flat `{ "error": "<message>" }` bodies (the "interim" helpers introduced in Epic 1 explicitly
deferred a unified model to this Epic), there are no liveness/readiness endpoints, and no
cross-cutting layers.

**This Epic adds those concerns, applied uniformly across the served API.** It introduces a single
**structured error model** adopted by *all* controllers (agent, user-management, content-delivery),
liveness/readiness endpoints suitable for Kubernetes probes, a lightweight request-logging
middleware, and configurable CORS / body-limit / global-timeout / rate-limit layers.

### Scope decisions (from PRD clarification)

- **Error body shape:** a **nested envelope** `{ "error": { "code", "message", "details" } }` —
  a deliberate, breaking change from the current flat `{ "error": "<message>" }`.
- **Error adoption scope:** **all controllers** — agent, user-management, and content-delivery —
  share one error type.
- **Observability:** a **log-based** request middleware (method/path/status/latency + request-id)
  via the workspace's existing `log`/`env_logger` setup — **no** new `tracing` backend.
- **Rate limiting:** the **`tower-governor`** crate, wired as a layer, **off by default**.

---

## 2. Goals

1. One structured error type renders every failure across all `paladin-web` controllers as
   `{ "error": { "code", "message", "details } }` with the correct HTTP status.
2. `GET /health` (liveness) and `GET /ready` (readiness) return structured JSON suitable for k8s
   probes.
3. Every request is logged once (method, path, status, latency, request-id) via `log`, with the
   request-id surfaced to the client (`x-request-id` response header).
4. CORS, request body-size limit, a global request timeout, and an (off-by-default) rate limiter
   are configurable and applied uniformly — **without breaking long-lived SSE streaming**.
5. The change is coherent: the SSE `error` events and all handlers use the same error envelope;
   the interim per-controller `ok_body`/`error_body` helpers are removed in favour of the shared
   model.
6. New code compiles warning-free; `fmt`/`clippy -D warnings`/`cargo test` and `make deny` pass;
   the API-surface baseline is updated.

---

## 3. User Stories

- **As a Kubernetes operator**, I want `/health` and `/ready` endpoints so my liveness/readiness
  probes can manage the pod lifecycle.
- **As an API client**, I want every error to have the same shape with a stable machine-readable
  `code`, so I can handle failures uniformly instead of string-matching messages.
- **As an operator debugging an incident**, I want each request logged with a correlation id (also
  returned to the caller) so I can trace a specific call end-to-end.
- **As a browser-app developer**, I want CORS configured so my front-end can call the API.
- **As an operator**, I want a body-size limit and an optional rate limiter so a single client
  can't exhaust the service.
- **As a streaming client**, I want the global request timeout to **not** cut off my SSE stream
  mid-response.

---

## 4. Functional Requirements

### 4.1 Unified error model

1. The system **must** define a single error type (e.g. `ApiError`) in `paladin-web` that carries a
   machine-readable `code` (stable `snake_case` string), a human `message`, optional structured
   `details`, and the HTTP status; and **must** implement `axum::response::IntoResponse` rendering:
   ```json
   { "error": { "code": "not_found", "message": "unknown agent 'x'", "details": null } }
   ```
2. It **must** provide ergonomic constructors for the statuses the API uses today —
   `not_found`, `bad_request`/`invalid`, `conflict`, `unprocessable`, `bad_gateway`,
   `gateway_timeout`, `not_implemented`, `internal` — each with a stable `code`.
3. **All three controllers must adopt it**: the agent-execution controller, the user-management
   controller, and the content-delivery controller. The interim flat `{ "error": "<message>" }`
   helpers (`ok_body`/`error_body`/`execution_error_response`) **must** be removed/replaced.
4. The SSE streaming `error` event **must** carry the same `{ "error": { … } }` envelope as the
   buffered responses (consistency across surfaces).
5. Status-code semantics established in Epics 1–3 **must** be preserved (e.g. unknown agent →
   `404`, execution failure → `502`, timeout → `504`, bad body → `400`, duplicate → `409`,
   provision failure → `422`, no-provisioner → `501`); only the *body shape* changes.

### 4.2 Health & readiness

6. The system **must** add `GET /health` (liveness): returns `200` with a small JSON status
   (e.g. `{ "status": "ok" }`); it **must not** depend on agents or external services.
7. The system **must** add `GET /ready` (readiness): a **shallow** check returning `200`
   `{ "status": "ready", "agents": <count> }` once the registry/state is built and serving (no
   network I/O). (Deep provider checks → Open Q1.)
8. Both endpoints **must** be unauthenticated and mountable alongside the agent routes.

### 4.3 Request logging (observability)

9. The system **must** add an axum middleware that, for every request, generates (or honours an
   inbound `x-request-id`) a **request-id**, and logs **once** at completion: method, path, status
   code, and latency (ms), via `log` (so it flows through `env_logger`).
10. The request-id **must** be returned to the client as an `x-request-id` response header.
11. Logging **must not** emit secrets or full request/response bodies.

### 4.4 CORS, body limit, global timeout

12. The system **must** add a **configurable CORS** layer (allowed origins/methods/headers;
    sensible permissive default for local dev, configurable for production).
13. The system **must** add a **request body-size limit** (configurable; a safe default such as
    1 MiB), returning `413`-style payload-too-large via the unified error model where applicable.
14. The system **must** add a **global request-timeout** layer as a coarse outer bound — but it
    **must not** apply to the SSE streaming route (`/agents/{id}/execute/stream`), whose responses
    are long-lived; streaming remains bounded by Epic 3's per-execution timeout. (Either scope the
    layer to non-streaming routes or set/disable it so streams are unaffected.)
15. These layers **must** be configurable and have safe defaults; applying them **must not** change
    the existing route behavior other than enforcing the limits.

### 4.5 Rate limiting (off by default)

16. The system **must** integrate **`tower-governor`** as a per-client (IP-keyed) rate-limit layer,
    **disabled by default** and enabled/configured via config (e.g. requests-per-second + burst).
17. When enabled, exceeding the limit **must** return `429 Too Many Requests` rendered via the
    unified error model.
18. When disabled (default), the layer **must** add no limiting and minimal overhead.

### 4.6 Composition & config

19. The cross-cutting layers **must** be applied uniformly to the served application (a single
    place — e.g. a `with_http_layers(router, config)` builder in `paladin-web`, or layered in
    `create_app_router_with_agents`), so all routes get health/logging/CORS/body-limit/rate-limit
    consistently.
20. A configuration surface (in `Settings`, mapped into a `paladin-web` config struct by the
    `paladin-server` binary — mirroring the Epic 3 `TimeoutPolicy` pattern) **must** expose: CORS
    settings, body-limit bytes, global-timeout seconds, and rate-limit (enabled/rps/burst).
    `paladin-web` **must not** gain a dependency on the `paladin-ai` facade.

### 4.7 Quality & tests

21. Every new public item **must** have rustdoc.
22. Tests **must** cover: `/health` and `/ready` (`200` + shape); the unified error body for
    representative statuses (`404`, `400`, `502`) across at least the agent controller; an
    `x-request-id` is present on responses; a CORS preflight (`OPTIONS`) returns the expected
    headers; the body-limit rejects an oversized body; and (rate-limit) a basic
    enabled-limit-exceeded → `429` test.
23. The boot smoke test **should** assert `/health` and `/ready` respond and that an error response
    uses the nested envelope.

---

## 5. Non-Goals (Out of Scope)

- **Authentication / authorization** — Epic 5 (these layers are unauthenticated; rate limiting is
  IP-based, not identity-based).
- **OpenAPI / Swagger** — Epic 6.
- **Metrics/Prometheus, distributed tracing exporters** — out of scope; observability here is
  request logging only (no new `tracing` backend).
- **Per-route / per-agent rate limits or quotas** — only a basic global IP limiter.
- **Changing execution semantics** (streaming, jobs, per-execution timeouts) from Epic 3 — the
  global timeout layer is additive and must not interfere with streaming.
- **Migrating the workspace off `log` to `tracing`.**

---

## 6. Design Considerations

### Error envelope

```json
{ "error": { "code": "gateway_timeout", "message": "agent 'x' timed out after 60s", "details": null } }
```
`code` is a stable lowercase identifier (client-switchable); `message` is human-facing; `details`
is an optional object for structured context (e.g. validation field errors). `ApiError:
IntoResponse` sets the status and serializes this body; handlers return `Result<T, ApiError>`.

### Route additions

| Method | Path | Purpose | Success |
|--------|------|---------|---------|
| `GET` | `/health` | Liveness | `200` `{ "status": "ok" }` |
| `GET` | `/ready` | Readiness (shallow) | `200` `{ "status": "ready", "agents": N }` |

### Middleware stack (illustrative order, outermost first)

```text
RateLimit (tower-governor, optional)
  └─ Cors
       └─ RequestBodyLimit
            └─ request logging + x-request-id
                 └─ [global timeout — non-streaming routes only]
                      └─ agent/user/delivery routes + /health + /ready
```

### Streaming caveat

A `tower_http::timeout::TimeoutLayer` applied globally would terminate the SSE response at the
deadline. Mitigation: apply the global timeout only to the non-streaming router (merge the stream
route separately), or omit/raise it for that route. Streaming stays bounded by the Epic 3
per-execution timeout (terminal `error` event).

---

## 7. Technical Considerations

- **Crate / layer:** all new code in `paladin-web` (adapter): an `error` module (`ApiError`), a
  `health` module (handlers), a request-logging middleware, and a `with_http_layers` composer; plus
  a small config struct populated by the `paladin-server` binary from `Settings`.
- **Dependencies (new, in `paladin-web`):** `tower-http` (features: `cors`, `limit`, `timeout`),
  `tower-governor`. (`tower` is already a dev-dep; promote/add as needed.) No `tracing` backend.
- **Error migration:** replace `agent_controller`'s `ok_body`/`error_body`/`execution_error_response`,
  `delivery_controller`'s `ok_body`/`error_body`, and the user controller's error mapping with the
  shared `ApiError`. Update the affected unit/handler tests (they currently assert
  `body["error"]` as a string → now `body["error"]["message"]` / `["code"]`).
- **Config:** extend `Settings` (facade) with a web/http section (CORS, body limit, global timeout,
  rate limit); map it into the `paladin-web` config struct in the binary — same seam as Epic 3's
  `TimeoutPolicy`. Keep config lenient (serde defaults).
- **Request-id:** generate a `uuid` when absent; echo inbound `x-request-id` when present.
- **API surface:** new public items (`ApiError`, health handlers, layer/config types) change
  `project/current-exports.txt` (paladin-web is optional/non-default in the facade surface, but the
  config additions are default-surface) — regenerate the baseline (expected additive, plus the
  error-shape change is internal to `paladin-web`).

---

## 8. Success Metrics

1. Every error response across agent/user/delivery routes is the nested envelope with a stable
   `code` (verified by tests).
2. `/health` and `/ready` return `200` with the documented shapes; k8s-style probes can consume
   them.
3. Each response carries an `x-request-id`, and the server logs one line per request with method,
   path, status, and latency.
4. A CORS preflight returns the configured headers; an oversized body is rejected; with rate
   limiting enabled, exceeding the limit returns `429` (nested error).
5. **SSE streaming is unaffected** by the global timeout layer (a long stream still completes).
6. `cargo test --features web-server`, `fmt`, `clippy --workspace --all-targets -D warnings`, and
   `make deny` are green; API-surface baseline updated.

---

## 9. Open Questions

1. **Readiness depth:** keep `/ready` shallow (chosen default), or later add a deep mode that
   verifies configured providers are usable (keys present / providers listed)? (Default: shallow;
   revisit if operators need misconfig detection at the probe.)
2. **Global timeout vs streaming:** scope the timeout layer to non-streaming routes (router split)
   or simply leave it disabled by default and rely on Epic 3's per-execution timeouts? (Default
   assumption: ship the layer scoped to non-streaming routes, off unless configured.)
3. **CORS default:** permissive (`Any`) for dev convenience, or locked-down (no origins) by default
   requiring explicit config? (Default assumption: permissive in dev builds / when unset, with a
   clear note to restrict in production.)
4. **Error `details` schema:** is a free-form JSON object sufficient for `details`, or do specific
   errors (e.g. validation) need a defined sub-shape now? (Default: free-form `Option<Value>`,
   populated where useful.)
5. **`x-request-id` trust:** always honour an inbound `x-request-id`, or only generate server-side
   (ignoring client-supplied ids) to avoid log-spoofing? (Default: honour if present and well-formed,
   else generate.)

---

*Next step: run `/generate-tasks` against this PRD to produce
`tasks-api-cross-cutting-concerns.md` in this `Epic_4/` folder.*
