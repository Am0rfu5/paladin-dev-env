# PRD: User and Admin System Completion (Milestone 9, Epic 5)

**Project:** Paladin Framework
**Milestone:** 9 — Classic Orchestrator, Content Pipeline, and Agent-Orchestrator Bridge
**Epic:** 5 of 6
**Status:** Approved for implementation
**Epic spec:** [Milestone_9-Epic_5-user-admin-system-completion.md](Milestone_9-Epic_5-user-admin-system-completion.md)

> This PRD was generated with the "no questions" workflow. Where the Epic spec left
> choices open, the most self-contained, deterministic, dependency-light option was
> selected and documented under [Design Considerations](#6-design-considerations).

---

## 1. Introduction / Overview

Paladin already has a working **user management foundation**: a `User` domain entity, a
`UserService` with Argon2 password hashing, a `UserRepositoryPort` with SQLite adapters, and a
set of Axum REST endpoints (`register`, `login`, `get`, `update`, `activate`, `deactivate`,
`verify`). What is missing is the **security layer** that makes the platform safe to deploy:

1. **Authentication** — `login` verifies a password but issues **no token**, and there is **no
   middleware** that protects endpoints. Any caller can hit any route.
2. **Authorization (RBAC)** — there is **no role concept** at all; every authenticated caller has
   identical privileges. Admin-only operations (activating/deactivating/deleting users) are
   unprotected.
3. **Admin operations** — the admin service modules exist only as empty stubs.

This Epic completes the user and admin system so that the platform can be deployed **with access
control**: tokens are issued on login, a middleware rejects unauthenticated requests with `401`,
and a role guard rejects under-privileged requests with `403`.

**Goal:** Deliver token-based authentication and admin/user RBAC, enforced through reusable Axum
middleware, with deterministic unit and integration tests — without breaking the existing user
endpoints or requiring network access at test time.

## 2. Goals

1. Issue a bearer token on successful login and validate it on every protected request.
2. Reject missing/invalid credentials with HTTP `401 Unauthorized`.
3. Introduce an `admin` vs `user` role, persisted on the user record.
4. Reject authenticated-but-under-privileged requests to admin-only routes with HTTP `403 Forbidden`.
5. Expose complete user CRUD over the API, including admin-only `delete`.
6. Keep the `paladin-ports` → adapter hexagonal boundary intact (no concrete crypto in the web crate).
7. Provide deterministic, offline unit + integration tests covering the auth and RBAC paths.
8. Keep all existing user endpoints and tests passing (backward compatible).

## 3. User Stories

- **As a new user**, I can register and then log in to receive a token, so that I can make
  authenticated requests.
- **As an authenticated user**, I can read and update **my own** profile, but I cannot perform
  admin operations, so that privilege boundaries are enforced.
- **As an unauthenticated caller**, when I omit or present an invalid token to a protected
  endpoint, I receive `401`, so that protected data is never exposed.
- **As an admin**, I can list users and activate/deactivate/verify/delete any user, so that I can
  operate the platform.
- **As a non-admin user**, when I call an admin-only endpoint, I receive `403`, so that I cannot
  escalate privileges.
- **As a platform operator**, I can run the full test suite offline and deterministically, so that
  CI is reliable.

## 4. Functional Requirements

### 4.1 Roles (domain)

1. The system must define a `UserRole` enum with at least `Admin` and `User` variants, with a
   string representation (`"admin"` / `"user"`) and a parser from string.
2. `UserData` must carry a `role: UserRole`, defaulting to `UserRole::User` for newly created
   users.
3. `User` must expose a `role()` accessor and a `set_role(UserRole)` mutator (consistent with the
   existing `Node<UserData>` accessor pattern).
4. The role must be **persisted**: the `users` table gains a `role` column
   (`TEXT NOT NULL DEFAULT 'user'`), applied idempotently so existing databases upgrade without
   data loss; SQLite repository row mapping must read and write the role.

### 4.2 Authentication port (in `paladin-ports`)

5. Define an `AuthPort: Send + Sync` (`async_trait`) with:
   - `issue_token(&self, user_id: Uuid, role: UserRole) -> Result<AuthToken, AuthError>`
   - `verify_token(&self, token: &str) -> Result<AuthClaims, AuthError>`
   - `revoke_token(&self, token: &str) -> Result<(), AuthError>`
6. Define value objects: `AuthToken { token: String, expires_at: DateTime<Utc> }` and
   `AuthClaims { user_id: Uuid, role: UserRole, expires_at: DateTime<Utc> }`.
7. Define `AuthError` (thiserror) with at least: `MissingToken`, `InvalidToken`, `Expired`,
   `Internal(String)`.

### 4.3 Authentication adapter (in the **root** crate)

8. Implement a concrete `AuthPort` adapter that issues opaque, cryptographically-random bearer
   tokens and validates them.
9. Tokens must be stored **hashed** (never in plaintext) and compared via hash lookup (no
   plaintext token comparison).
10. Tokens must carry an expiry; `verify_token` must reject expired tokens with `AuthError::Expired`.
11. `revoke_token` must invalidate a previously issued token so subsequent `verify_token` calls fail.

### 4.4 Login issues a token

12. The login flow must, on successful password verification, issue a token via `AuthPort` and
    return it to the caller (token string + expiry) alongside existing user identity fields.

### 4.5 Authentication middleware (in `paladin-web`)

13. Provide reusable Axum middleware/extractor that reads the `Authorization: Bearer <token>`
    header, calls `AuthPort::verify_token`, and on success injects `AuthClaims` into the request
    so handlers can read the caller identity/role.
14. Missing or malformed `Authorization` header, or an invalid/expired token, must produce
    `401 Unauthorized` with a JSON error body and must **not** reveal which part failed.

### 4.6 Authorization / RBAC (in `paladin-web`)

15. Provide a role guard that requires `UserRole::Admin`; when the authenticated caller is not an
    admin, it must produce `403 Forbidden`.
16. Admin-only endpoints — `list users`, `activate`, `deactivate`, `verify`, `delete` — must be
    protected by both authentication (FR 13–14) and the admin guard (FR 15).
17. Self-service endpoints — `get own profile`, `update own profile` — must require authentication
    (FR 13–14). A non-admin caller must only be able to read/update **their own** record; access to
    another user's record by a non-admin must produce `403`.

### 4.7 User CRUD completeness

18. Add a `delete_user` operation to the user service surface and an admin-only `DELETE /users/:id`
    route (the repository already supports delete).
19. Add a `list_users` admin endpoint backed by existing repository/service query methods.
20. All user-data responses must continue to omit the password hash.

### 4.8 Routing composition

21. Provide a single composition function (e.g. `create_app_router(user_service, auth_port)`) that
    assembles public routes (`register`, `login`) and protected routes (everything else) with the
    appropriate middleware/guards applied.

### 4.9 Tests

22. Unit tests must cover the auth adapter (issue → verify round-trip, expiry rejection, revoke
    rejection, invalid token rejection) and the role/string conversions.
23. Integration tests (offline, deterministic) must exercise the assembled router and assert:
    - protected route without a token → `401`;
    - protected route with a valid token → `200`;
    - admin-only route with a `user`-role token → `403`;
    - admin-only route with an `admin`-role token → success.

### 4.10 Quality gates

24. `cargo build`, `cargo test`, `cargo clippy -- -D warnings`, and `cargo fmt --check` must all
    pass. The web crate must additionally build/test with the `web-server` feature path it lives
    behind.

## 5. Non-Goals (Out of Scope)

- JWT/OIDC/OAuth or any external identity provider integration.
- Multi-tenant authorization or fine-grained per-resource ACLs beyond the admin/user split.
- Refresh-token rotation, MFA, rate limiting, or audit-log subsystems (may be referenced as future
  work but are not implemented here).
- Replacing the in-memory token store with a database-backed store (the port allows it later).
- A production secret-management story for signing keys (the adapter is self-contained).

## 6. Design Considerations

### 6.1 Token strategy — opaque bearer tokens vs JWT

- **Chosen: opaque, randomly-generated bearer tokens with a server-side hashed store.**
- Rationale: avoids adding a JWT dependency (`jsonwebtoken`) and a signing-key management story;
  supports immediate **revocation** (logout) which stateless JWTs cannot; trivially deterministic
  to unit test. The root crate already has `rand` (token generation) and `sha2` (hashing the
  stored token), so **no new dependencies** are required.
- Trade-off: tokens are validated against an in-process store, so a multi-process deployment would
  later need a shared store. This is acceptable because validation is hidden behind `AuthPort`, so
  the store can be swapped without touching the web layer.

### 6.2 Crate placement (hexagonal boundary)

- `AuthPort`, `AuthClaims`, `AuthToken`, `AuthError` live in `paladin-ports` (which may depend on
  `paladin-core` for `UserRole`).
- The concrete token adapter lives in the **root crate** (it needs `rand`/`sha2`/`chrono`).
- `paladin-web` depends only on `paladin-ports` + `paladin-core`; its middleware is generic over
  `Arc<dyn AuthPort>` and never performs cryptography itself. This mirrors the Epic 4 bridge
  pattern (port in `paladin-ports`, adapter in root, consumer takes `Arc<dyn _>`).

### 6.3 Role placement

- The role is stored on `UserData` (persisted) rather than only in the token, so that privileges
  survive re-login and are the single source of truth. The token merely **carries** the role for
  fast authorization without a DB round-trip on every request.

### 6.4 Backward compatibility

- The existing `create_user_routes` and handlers remain functional; new protected composition is
  additive. `UserData` gains `role` with a default so existing construction paths and the DB
  upgrade without breaking.

## 7. Technical Considerations

- **Framework:** Axum 0.8 (already a `paladin-web` dependency). Router integration tests use
  `tower::ServiceExt::oneshot` + `http-body-util` as **dev-dependencies** of `paladin-web`.
- **Mocks:** Integration tests use a mock `UserServiceTrait` and the real in-memory `AuthPort`
  adapter (or a mock) so no database or network is required.
- **Constant-time / hashing:** stored tokens are hashed with `sha2`; lookups are by hash, avoiding
  plaintext comparison. Passwords continue to use Argon2 (already implemented).
- **Migration safety:** the `role` column is added with `ADD COLUMN ... DEFAULT 'user'` guarded so
  re-running against an already-migrated DB is a no-op.
- **Feature gating:** web pieces are exercised behind the existing `web-server` feature; the
  `AuthPort` and `UserRole` are always-compiled in `paladin-ports`/`paladin-core`.
- **No `unwrap()` in production paths;** use typed errors (`AuthError`, `UserError`) and map to
  HTTP status codes at the boundary.

## 8. Success Metrics

- 100% of protected routes reject unauthenticated requests with `401` (verified by integration
  tests).
- 100% of admin-only routes reject `user`-role tokens with `403` (verified by integration tests).
- Auth adapter unit tests cover issue/verify/expiry/revoke/invalid paths.
- Existing user endpoints and their tests continue to pass (zero regressions).
- `cargo build`, `cargo test`, `cargo clippy -- -D warnings`, `cargo fmt --check` all green.

## 9. Open Questions

- Should logout (`revoke_token`) be exposed as an endpoint in this Epic, or only as a port method?
  (Default: implement the port method + unit test; expose an endpoint only if cheap.)
- Token TTL default value (proposed: 24h) — configurable later via settings; hard-coded sensible
  default for now.
- Should the first registered user be auto-promoted to `admin`? (Default: no; admin role is set
  explicitly via `set_role`/repository, keeping registration uniform. A test helper will create an
  admin directly.)

---

## Task Checklist

High-level mapping to the Epic's tasks (detailed sub-tasks live in the companion
`tasks-user-admin-system-completion.md`).

- [x] **Task 5.1 — User CRUD via API.** Add `delete`/`list` to the service surface and admin-only
  routes; keep Argon2 hashing; ensure responses omit password hashes. (FR 18–20)
- [x] **Task 5.2 — Authentication flow.** Define `AuthPort` + value objects in `paladin-ports`;
  implement the opaque-token adapter in the root crate; issue a token on login; add Axum auth
  middleware returning `401`. (FR 5–14)
- [x] **Task 5.3 — Role-based access control.** Add `UserRole` to the domain + persistence; add the
  admin role guard returning `403`; protect admin-only and self-service routes; compose the app
  router. (FR 1–4, 15–17, 21)
- [x] **Tests & quality gate.** Unit + offline integration tests for auth and RBAC; full quality
  gate green. (FR 22–24)
