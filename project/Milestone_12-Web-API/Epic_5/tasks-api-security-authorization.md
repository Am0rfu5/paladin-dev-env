# Tasks: API Security & Authorization (Milestone 12, Epic 5)

**PRD:** [prd-api-security-authorization.md](prd-api-security-authorization.md)
**Crate:** `paladin-web` (auth middleware/authz) + facade config/binary wiring
**Base:** `main` (Milestone 12 Epics 1–4 merged — PRs #19, #21, #22, #23)
**Status:** Phase 2 — sub-tasks expanded, ready for implementation

---

## Relevant Files

- `crates/paladin-web/src/agent_auth.rs` - **New.** `Principal { id, role }`, `AgentAuthConfig { enabled, api_keys, jwt }`, the authentication middleware (`X-API-Key` + `Authorization: Bearer` via `AuthPort`), and authorization helpers (per-agent `allowed_roles`, admin gate). Returns `401`/`403` as `ApiError`. Unit tests in-file.
- `crates/paladin-web/src/agent_registry.rs` - **Modify.** `AgentEntry.allowed_roles` + `AgentSpec.allowed_roles`.
- `crates/paladin-web/src/agent_controller.rs` - **Modify.** `AgentApiState.auth: AgentAuthConfig`; apply the auth layer to the agent routes (not health) in `agent_router`; handler authz (per-agent `allowed_roles` on execute/stream/jobs; admin gate on register/deregister) using the attached `Principal`.
- `crates/paladin-web/src/request_log.rs` - **Modify/verify.** Confirm no header/credential is logged (redaction).
- `crates/paladin-web/src/user_controller.rs` - **Modify (optional, Open Q5).** Align the user-route `401`/`403` to `ApiError`.
- `crates/paladin-web/src/lib.rs` - **Modify.** Declare/doc `agent_auth`; re-export `Principal`/`AgentAuthConfig`.
- `src/config/agents.rs` - **Modify.** `AgentDefinition.allowed_roles`; an `AuthConfig` (`enabled`, `api_keys: [{key,name,role}]`, `jwt`) on `WebHttpConfig`.
- `src/infrastructure/web/agent_host.rs` / `facade_provisioner.rs` - **Modify.** Carry `allowed_roles` into the registry entry (same seam as `timeout_secs`).
- `src/bin/paladin-server.rs` - **Modify.** Build `AgentAuthConfig` from config; wire `InMemoryTokenAuthAdapter` (`AuthPort`) for the JWT path when configured; fail-closed startup; log the posture.
- `tests/paladin_server_smoke.rs` - **Modify.** Authenticated round-trip (`401` without, `200` with a key) over real HTTP.
- `config.example.yml` / `README.md` / `CHANGELOG.md` / `project/current-exports.txt` - **Modify.** Document + record the additions.

### Notes

- **TDD (Red-Green-Refactor):** failing test first for each behavior-bearing sub-task.
- Run with `cargo test --features web-server`. Before committing a parent task: `cargo test` →
  `cargo fmt --check` → `cargo clippy -- -D warnings` → `make deny`.
- **Reused (verified):** `AuthPort::verify_token` + `AuthClaims { user_id, role }` (paladin-ports);
  `UserRole { Admin, User }` (paladin-core); `InMemoryTokenAuthAdapter` (facade) as the JWT
  `AuthPort` impl; the Epic 4 `ApiError` for `401`/`403`; the `timeout_secs`/`AgentEntry` and
  config-builder/provisioner seams from Epics 2–3.
- **Hexagonal:** all auth/authz code in `paladin-web` (depends on `paladin-ports`/`paladin-core`);
  the JWT `AuthPort` impl is **injected by the binary** — `paladin-web` gains **no** facade dep.
- **Bypass:** `GET /health` and `GET /ready` must remain unauthenticated.
- **Fail-closed:** auth enabled (default) + no credentials configured ⇒ startup error; `enabled:
  false` ⇒ open with a logged warning.
- **Out of scope** (later/other epics): OpenAPI security schemes (6), Docker/k8s (7), OAuth/OIDC,
  mTLS, key rotation/storage, identity-based rate limiting.

## Tasks

- [ ] 0.0 Create feature branch
  - [ ] 0.1 Update `main` (Epics 1–4 merged) and create/checkout `feature/m12-epic5-api-security-authorization` from it.
  - [ ] 0.2 Confirm a clean baseline: `cargo build --features web-server` and `cargo test --features web-server` pass before any changes.

- [x] 1.0 Add authentication (`agent_auth`: `Principal`, `AgentAuthConfig`, API-key + JWT middleware → `401`)
  - [x] 1.1 Created `crates/paladin-web/src/agent_auth.rs` (+ `pub mod`/re-exports). `Principal { id, role }`, `AgentAuthConfig { enabled, api_keys: HashMap<String, Principal>, jwt: Option<Arc<dyn AuthPort>> }` + `has_credentials()`. Added `ApiError::unauthorized`/`forbidden`.
  - [x] 1.2 **(Test first)** Unit tests: valid `X-API-Key` → `Principal` (constant-time `ct_eq`); valid `Authorization: Bearer` via mock `AuthPort` → `Principal`; missing/invalid → `401`.
  - [x] 1.3 Implemented `authenticate(headers, &AgentAuthConfig) -> Result<Principal, ApiError>` (bearer-JWT first, then API key) + `require_authentication` middleware (attaches `Principal`; `401` on failure; when disabled attaches an open-access `Admin` principal and passes through).
  - [x] 1.4 Added `auth` to `AgentApiState` (+ `with_auth`; library default = **disabled/open** so existing tests/embedding are unaffected — the secure default lives in the server). Applied the middleware to agent routes via `route_layer`; merged `health_routes` stay open.
  - [x] 1.5 **(Test first)** Router tests: auth enabled + key — no credential → `401`, valid `X-API-Key` → `200`; `/health` + `/ready` → `200` without a credential.
  - [x] 1.6 Rustdoc; `fmt`/`clippy --all-targets -D warnings` clean; agent_auth 8 tests, full paladin-web 104 pass (no regressions).

- [x] 2.0 Add authorization (per-agent `allowed_roles` → `403`; admin gate on register/deregister)
  - [x] 2.1 Added `allowed_roles: Vec<UserRole>` to `AgentEntry` and (`#[serde(default)]`) to `AgentSpec`; `UserRole` deserializes from lowercase (`"admin"`/`"user"`).
  - [x] 2.2 **(Test first)** Helper tests (in `agent_auth`): `authorize_invoke` (empty ⇒ allow; listed ⇒ allow; else `403`) and `require_admin` (Admin ⇒ ok, else `403`).
  - [x] 2.3 Enforced per-agent `allowed_roles` in `execute_agent`/`execute_agent_stream`/`enqueue_job` and the admin gate in `register_agent`/`deregister_agent`, reading a required `Extension<Principal>` (fail-closed; the middleware always inserts it). `register_agent` carries `spec.allowed_roles` into the entry.
  - [x] 2.4 **(Test first)** Handler tests: disallowed role → `403`; listed role → success; non-admin register/deregister → `403`; admin → success. (Existing handler tests pass an `admin()` principal extension; the two invalid-body router tests switched to `agent_router` so the default-disabled auth layer attaches a principal.)
  - [x] 2.5 Carried `allowed_roles` through `register_built` (new param) and the `AgentSpec` path (handler reads `spec.allowed_roles`; `FacadeProvisioner` needs no change as `ProvisionedAgent` is role-agnostic). Config wiring of `def.allowed_roles` deferred to 3.4. `fmt`/`clippy --workspace --all-targets -D warnings`/`make deny` clean; paladin-web 108, facade web 12 pass.

- [x] 3.0 Config + `paladin-server` wiring (`auth` config, JWT `AuthPort`, fail-closed posture)
  - [x] 3.1 Added `AuthConfig { enabled(default true), api_keys: [ApiKeyConfig{key,name,role}], jwt: JwtAuthConfig{enabled} }` on `WebHttpConfig`, and `allowed_roles` on `AgentDefinition`. **(Test first)** 4 parse tests incl. default-enabled and unknown-role-string → error (`UserRole` lowercase serde).
  - [x] 3.2 `paladin-server` `build_auth_config` maps config → `AgentAuthConfig` (api_keys → `Principal{id:name, role}`), wiring `InMemoryTokenAuthAdapter` as the JWT `AuthPort` when `jwt.enabled` (documented in-process caveat). `WebHttpConfig::default` carries `AuthConfig::default` (enabled) so `Settings` default is secure; `user_config` fixture (`http: None`) unaffected.
  - [x] 3.3 **Fail-closed:** auth enabled + no credentials ⇒ startup error (verified by boot); `enabled: false` logs a warning and serves open; the resolved posture is logged. Verified end-to-end: no key → `401`, valid `X-API-Key` → `200`, `/health` open.
  - [x] 3.4 `build_agent_registry` passes `def.allowed_roles` into `register_built`. `fmt`/`clippy --workspace --all-targets -D warnings` clean; facade lib 395, paladin-web 108, config 52, smoke 1 pass.

- [x] 4.0 Secret hygiene (redact credential headers; reconfirm discovery; align user-route errors to `ApiError`)
  - [x] 4.1 Documented the request logger's hygiene (logs only method/path/status/latency/request-id — never headers/bodies, so `Authorization`/`X-API-Key` are never logged) + a test that `authenticate` errors never echo the presented credential.
  - [x] 4.2 Reconfirmed discovery: the existing leak-canary tests (`list_agents…no_prompt_leak`, `describe_agent…without_prompt_leak`) confirm the raw system prompt isn't returned; API keys/provider config never reach `paladin-web` entities, so they cannot leak via discovery.
  - [x] 4.3 **(Open Q5)** Aligned the user-route `auth_middleware` `unauthorized()`/`forbidden()` to the unified `ApiError` envelope; `auth_rbac` and middleware tests assert status only, so they remained green unchanged.
  - [x] 4.4 Rustdoc; `fmt`/`clippy --all-targets -D warnings` clean; paladin-web 109 + auth_rbac 5 pass.

- [ ] 5.0 Tests: `401`/`403` paths (key + JWT), `allowed_roles`, admin gate, health open, redaction + boot smoke
  - [ ] 5.1 Confirm unit/handler coverage from 1.0–4.0 is in place (authn resolution, authz helpers, handler `401`/`403`, config parse + fail-closed, redaction).
  - [ ] 5.2 **(Test first)** Extend `tests/paladin_server_smoke.rs`: build state with auth enabled + a configured API key; assert no credential → `401`, valid `X-API-Key` → `200` for execute, and `/health` reachable without a credential.

- [ ] 6.0 Finalize: config sample, docs, CHANGELOG, API baseline, and quality gates
  - [ ] 6.1 Update `config.example.yml`: an `http.auth` block (enabled + sample `api_keys` + `jwt`) and a per-agent `allowed_roles` example, with a note that keys belong in env/secrets.
  - [ ] 6.2 Update `README.md`: authentication (API key + JWT), per-agent `allowed_roles`, the admin gate, and the fail-closed posture / disable flag.
  - [ ] 6.3 Full gate: `cargo test --features web-server`, `cargo fmt --check`, `cargo clippy --workspace --all-targets --features web-server -- -D warnings`, `make deny`. Remove any debug prints.
  - [ ] 6.4 Regenerate `project/current-exports.txt` (new `Principal`/`AgentAuthConfig` + config `auth`/`allowed_roles` fields) — review the diff (additive expected).
  - [ ] 6.5 Add a `CHANGELOG.md [Unreleased]` entry (Milestone 12 — Epic 5): auth (key + JWT), per-agent authz, admin gate, fail-closed default.
  - [ ] 6.6 Commit referencing Milestone 12 / Epic 5; mark parent tasks complete and **stop for go-ahead**.
