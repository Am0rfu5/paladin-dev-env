# PRD: API Security & Authorization (Milestone 12, Epic 5)

> **Correction (dated 2026-08-06, DEBT-01):** This document's §7 "API surface" bullet (struck
> below) instructs a future implementer to write the public-API surface baseline to the pre-rename
> `project/` path — a path that has not existed since commit `928c6d5` renamed `project/` to
> `.project/`. The baseline lives at `.project/current-exports.txt`, confirmed present at 442,369
> bytes via `ls -la .project/current-exports.txt`, re-run during this task; the pre-rename path is
> confirmed absent via `ls` on it, which returns "No such file or directory", also re-run during
> this task. This document was created 2026-06-09, months after commit `928c6d5` renamed the
> directory, so the defect it names propagates forward rather than decaying. This is one of five
> requirement documents Phase 8 / DEBT-01 corrects on the requirement-text side; the corresponding
> tooling (`scripts/check-api-surface.sh`, `scripts/extract-public-api.sh`,
> `.github/workflows/ci.yml`) was corrected separately in plan 08-02. Original text is retained
> below with inline corrections — nothing is deleted.

> **Note (dated 2026-08-10, Phase 13 / 13-REVIEW.md IN-01):** the `442,369 bytes` figure above is a
> correct point-in-time measurement taken on 2026-08-06 and is retained unchanged. The file has
> since grown to `446,377 bytes` (Epics 3 and 4 regenerated the export baseline additively). The
> banner's load-bearing claim is the *path* (`.project/current-exports.txt`), which remains correct;
> the byte count is a snapshot and will continue to drift, so it should not be relied on as current.

> **Correction (dated 2026-08-10, ADR-0037):** This document's route text — every unprefixed
> `/agents...` path below — is **superseded provenance, not a live contract**. The shipped agent
> API is served under a `/v1` prefix, confirmed against the committed
> `crates/paladin-web/openapi.json` drift-guard baseline and enforced live by `openapi.rs`'s
> `spec_paths_are_versioned_under_v1` test. The recorded answer is
> `.planning/decisions/0037-agent-route-surface-v1.md`. Original text is retained below; each
> occurrence of an unprefixed route is followed by a new note line marking it superseded —
> nothing is struck, rewritten, or removed.

**Project:** Paladin Framework
**Milestone:** 12 — Web API / HTTP Service Host Topology, Out of the Box
**Epic:** 5 — API Security & Authorization
**Version Target:** v0.6.0 (Unreleased)
**Status:** Ready for Implementation
**Created:** 2026-06-09
**Author:** AI Coding Agent (Claude Code)
**Depends on:** Milestone 12 Epics 1–4 (agent API, server binary, streaming/jobs, error model + layers)

---

## 1. Introduction / Overview

The agent HTTP API has been **intentionally unauthenticated** through Epics 1–4 — anyone who can
reach the port can run, register, or delete agents. That is unacceptable for a real deployment:
agent execution spends money (LLM calls) and runtime registration is a powerful capability.

**This Epic secures the agent-execution surface.** It adds authentication (a configured **API-key**
mechanism for service-to-service callers *and* the existing **JWT** bearer path via `AuthPort`),
**role-based per-agent authorization** (who may invoke which agent), an **admin gate** on runtime
registration/deregistration, and confirms **secret hygiene** (no credentials or prompts leak into
responses or logs). All rejections use the Epic 4 [`ApiError`] envelope (`401`/`403`).

### Scope decisions (from PRD clarification)

- **Posture:** **required by default, but disable-able** — auth is enforced unless explicitly turned
  off in config (fail-closed default with a documented dev escape hatch).
- **Mechanisms:** **API keys + JWT** — a config'd static API-key list (header) *and* user JWTs via
  the existing `AuthPort::verify_token`.
- **Per-agent authorization:** **role-based** — an optional `allowed_roles` per agent; empty/absent
  ⇒ any authenticated caller.
- **Route privilege:** **admin** required for `POST /agents` and `DELETE /agents/{id}`;
  execute/stream/jobs/discovery require any authenticated caller (subject to per-agent authz).
  > *(superseded — ADR-0037: shipped as `/v1/agents` and `/v1/agents/{id}`)*

---

## 2. Goals

1. With auth enabled (the default), every agent route requires a valid credential; unauthenticated
   requests get `401` and unauthorized ones `403` — both as `ApiError`.
2. Two credential types are accepted: a configured **API key** (`X-API-Key` header) mapped to a
   principal+role, and a **JWT** (`Authorization: Bearer`) verified via the existing `AuthPort`.
3. Per-agent `allowed_roles` restricts invocation; `POST`/`DELETE /agents` require an admin role.
   > *(superseded — ADR-0037: shipped under `/v1/agents`)*
4. Auth is **fail-closed**: if enabled but no credentials are configured (no API keys and no JWT
   verifier), the server refuses to serve protected routes (clear startup error), unless auth is
   explicitly disabled.
5. No secret (API key, JWT, system prompt, provider config) appears in any response body or log
   line; credential headers are redacted from request logging.
6. Health/readiness remain **unauthenticated** (probes must not require credentials).
7. New code compiles warning-free; `fmt`/`clippy -D warnings`/`cargo test` and `make deny` pass;
   the API-surface baseline is updated.

---

## 3. User Stories

- **As an operator**, I want the agent API to require a credential by default so I can't accidentally
  expose agent execution to the open internet.
- **As a service integrator**, I want to authenticate with a static API key in a header, without
  standing up a user/JWT system.
- **As a platform using Paladin's user system**, I want existing user JWTs to work against the agent
  API so I don't need a separate credential.
- **As an operator**, I want to restrict sensitive agents to specific roles (`allowed_roles`) and
  keep runtime registration admin-only, so least privilege is enforced.
- **As a security reviewer**, I want assurance that keys/tokens/prompts never appear in responses or
  logs.
- **As a dev**, I want to disable auth locally with one config flag.

---

## 4. Functional Requirements

### 4.1 Authentication mechanisms

1. The system **must** accept an **API key** via the `X-API-Key` header, resolving it (constant-time
   compare) against a configured map of key → principal `{ name, role }`. A valid key authenticates
   the request with that principal's role.
2. The system **must** accept a **JWT** via `Authorization: Bearer <token>`, verified through the
   existing `AuthPort::verify_token`, yielding `AuthClaims { user_id, role }`. JWT is available only
   when an `AuthPort` verifier is configured.
3. When both headers are present, a deterministic precedence **must** apply (e.g. `Authorization`
   bearer first, then `X-API-Key`) — documented.
4. A successful authentication **must** attach a unified principal (role + identifier) to the request
   (extensions) for downstream authorization; failure **must** return `401` (`ApiError`).

### 4.2 Posture & configuration (fail-closed)

5. Auth **must** be configurable under the `http` config (e.g. `http.auth`): `enabled` (default
   **true**), an `api_keys` list, and JWT settings.
6. When `enabled` and **no** credential source is configured (no API keys and no JWT verifier), the
   server **must** fail closed — refuse to start serving protected routes with a clear error telling
   the operator to configure credentials or set `enabled: false`.
7. When `enabled: false`, the agent routes serve unauthenticated (current behavior) — intended for
   trusted/dev environments; this **must** be logged as a warning at startup.

### 4.3 Authorization

8. **Per-agent invocation:** each agent **may** declare `allowed_roles`. On `execute`,
   `execute/stream`, and `jobs`, the caller's role **must** be in the agent's `allowed_roles`;
   empty/absent ⇒ any authenticated caller. A disallowed role **must** get `403` (`ApiError`).
9. **Admin-gated routes:** `POST /agents` (register) and `DELETE /agents/{id}` (deregister) **must**
   require an admin role; non-admin authenticated callers get `403`.
   > *(superseded — ADR-0037: shipped as `/v1/agents` and `/v1/agents/{id}`)*
10. Discovery (`GET /agents`, `GET /agents/{id}`) and `GET /agents/{id}/jobs/{job_id}` require
    authentication (any role) when auth is enabled.
    > *(superseded — ADR-0037: shipped as `/v1/agents`, `/v1/agents/{id}`, and
    > `/v1/agents/{id}/jobs/{job_id}`)*
11. `GET /health` and `GET /ready` **must** remain unauthenticated regardless of config.

### 4.4 Secret hygiene

12. Discovery responses **must not** include the raw system prompt, API keys, JWTs, or provider
    configuration (already true for prompts — reconfirm and test).
13. No log line **must** contain an API key or JWT; the request logger **must** redact the
    `Authorization` and `X-API-Key` headers (it already logs no headers/bodies — keep it that way
    and ensure auth code logs neither credential).
14. Auth/authz error messages **must not** echo the supplied credential.

### 4.5 Composition & integration

15. Authentication **must** be applied as a layer/middleware over the protected agent routes (not the
    health routes), composed in `with_http_layers` / the router so it runs before handlers; the
    per-agent and admin authorization checks run in the handlers (or a thin extractor) using the
    attached principal and the agent's `allowed_roles`.
16. The `paladin-server` binary **must** build the auth context from config: load the API-key map,
    and wire an `AuthPort` JWT verifier when configured. The agent's `allowed_roles` **must** be
    carried into the registry entry (alongside `timeout_secs`) by the config builder and the runtime
    provisioner.
17. `paladin-web` **must not** gain a dependency on the `paladin-ai` facade (the JWT `AuthPort`
    implementation is injected by the binary, mirroring the executor/provisioner seam).

### 4.6 Quality & tests

18. Every new public item **must** have rustdoc.
19. Tests **must** cover: unauthenticated → `401`; invalid key/JWT → `401`; valid API key → success;
    valid JWT → success; caller role not in `allowed_roles` → `403`; non-admin register/deregister →
    `403`; admin register → success; health/ready reachable without a credential; and a redaction
    test (a key/token does not appear in the logged line / responses).

---

## 5. Non-Goals (Out of Scope)

- **User-management changes** (registration/login already exist with their own auth) beyond reusing
  `AuthPort`.
- **OAuth/OIDC, mTLS, signed requests** — only static API keys + the existing JWT path.
- **API-key storage backends / rotation / per-key rate limits** — keys are static config; rotation is
  operational (edit config + restart). Epic 4's rate limiter remains IP-based.
- **Fine-grained scopes/permissions** beyond `allowed_roles` + the admin gate.
- **OpenAPI security schemes** — documented in Epic 6.
- **Encrypting config at rest** — secrets management is the operator's responsibility (as with LLM
  keys).

---

## 6. Design Considerations

### Config shape (illustrative)

```yaml
http:
  auth:
    enabled: true            # default; set false for trusted/dev (logged as a warning)
    api_keys:
      - key: "sk-svc-abc123" # value from env/secret in practice
        name: "ci-runner"
        role: "admin"
      - key: "sk-svc-def456"
        name: "frontend"
        role: "user"
    jwt:
      enabled: true          # use the AuthPort bearer path (verifier wired by the binary)

agents:
  - id: "researcher"
    model: "gpt-4"
    system_prompt: "…"
    allowed_roles: ["admin", "user"]   # absent/empty ⇒ any authenticated caller
```

### Request flow

```text
request ─► [auth layer] ──(401 if required & invalid)──► attach Principal{role}
                              │
        health/ready ─────────┘ (bypass: always open)
                              ▼
         handler ─► admin gate (register/deregister) ─► per-agent allowed_roles ─► run
                       (403)                               (403)
```

### Credentials

- **API key:** `X-API-Key: <key>`; resolved against the configured map (constant-time compare),
  yielding `{ name, role }`.
- **JWT:** `Authorization: Bearer <jwt>`; `AuthPort::verify_token` → `AuthClaims { user_id, role }`.
- Unified `Principal { id: String, role: UserRole }` attached to the request extensions.

---

## 7. Technical Considerations

- **Crate / layer:** new auth code in `paladin-web` (e.g. `agent_auth.rs`): the `Principal` type, an
  `AgentAuthConfig { enabled, api_keys: HashMap<String, Principal>, jwt: Option<Arc<dyn AuthPort>> }`,
  the authentication middleware, and authorization helpers. `AuthPort`/`AuthClaims`/`UserRole` come
  from `paladin-ports`/`paladin-core` (already deps) — **no facade dependency**.
- **Reused building blocks:** `AuthPort::verify_token` + `AuthClaims` (paladin-ports); `UserRole`
  (paladin-core); the existing `auth_middleware` patterns (`require_auth`); the Epic 4 `ApiError`
  for `401`/`403`; `AgentApiState` (gains the auth context) and `AgentEntry` (gains `allowed_roles`).
- **Registry/state:** add `allowed_roles: Vec<UserRole>` (or `Vec<String>` parsed to roles) to
  `AgentEntry`; `AgentDefinition` + `AgentSpec` gain `allowed_roles`; the builder + provisioner carry
  it (same seam as `timeout_secs`). `AgentApiState` gains `auth: AgentAuthConfig`.
- **Binary wiring:** `paladin-server` builds `AgentAuthConfig` from `Settings.http.auth` (API-key map)
  and constructs an `AuthPort` JWT verifier when configured (injecting the facade's implementation),
  then applies the auth layer to the agent routes (not health). Fail-closed validation at startup.
- **Existing auth_middleware:** its `401`/`403` responses predate `ApiError`; align the new agent-auth
  rejections to `ApiError` (and optionally migrate the user-route middleware for consistency — see
  Open Q5).
- **Config:** extend `WebHttpConfig` with an `auth` section (lenient serde defaults); API-key values
  should come from env/secret indirection in practice (documented), not committed config.
- ~~**API surface:** new public items (`Principal`, `AgentAuthConfig`, auth middleware, config types)~~
  ~~will change `project/current-exports.txt` — regenerate (expected additive, plus the~~
  ~~`allowed_roles`/auth fields).~~
  **Corrected (dated 2026-08-06, DEBT-01):** The correct baseline path is
  `.project/current-exports.txt`, not the pre-rename path struck above — the directory was renamed
  by commit `928c6d5`. Confirmed via `ls -la .project/current-exports.txt` (442,369 bytes present)
  and `ls` on the struck path ("No such file or directory"), both re-run during this task. The rest
  of the bullet (the additive-change expectation) is unaffected.

---

## 8. Success Metrics

1. With auth enabled and a configured API key, `X-API-Key: <key>` succeeds; a missing/invalid
   credential returns `401`; all verified by tests.
2. A user JWT (existing `AuthPort`) authenticates against the agent API.
3. A caller whose role is not in an agent's `allowed_roles` gets `403`; a non-admin `POST /agents`
   gets `403`; an admin succeeds.
   > *(superseded — ADR-0037: shipped as `/v1/agents`)*
4. `GET /health` and `GET /ready` succeed with no credential.
5. With auth enabled and **no** credentials configured, the server fails closed at startup with an
   actionable message; `enabled: false` serves open with a logged warning.
6. No API key/JWT/prompt appears in any response or log (redaction test + discovery test).
7. `cargo test --features web-server`, `fmt`, `clippy --workspace --all-targets -D warnings`, and
   `make deny` are green; API-surface baseline updated.

---

## 9. Open Questions

1. **API-key → role mapping:** is `{ key, name, role }` per entry sufficient, or do keys need
   multiple roles / scopes? (Default: single role per key.)
2. **Role source/representation:** reuse `UserRole` (Admin/User/…) for both JWT and API-key
   principals, parsing `role` strings from config to `UserRole`? (Default: yes; unknown role string
   → startup error.)
3. **Disable in release builds:** should `enabled: false` be permitted in release builds, or only
   debug (to prevent accidentally shipping open)? (Default: permitted but loudly warned.)
4. **JWT verifier construction:** which concrete `AuthPort` impl does `paladin-server` wire, and what
   does it need (signing secret/algorithm) from config/env? (Confirm the available adapter during
   implementation.)
5. **User-route middleware alignment:** also migrate the existing user-route `401`/`403`
   (`auth_middleware`) responses to `ApiError` for whole-API consistency, or leave them as-is this
   epic? (Default: align them, small follow-on to Epic 4's error unification.)

---

*Next step: run `/generate-tasks` against this PRD to produce
`tasks-api-security-authorization.md` in this `Epic_5/` folder.*
