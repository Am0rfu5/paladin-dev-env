# Tasks: User and Admin System Completion (Milestone 9, Epic 5)

Source PRD: [prd-user-admin-system-completion.md](prd-user-admin-system-completion.md)
Epic spec: [Milestone_9-Epic_5-user-admin-system-completion.md](Milestone_9-Epic_5-user-admin-system-completion.md)

## Relevant Files

- `crates/paladin-core/src/platform/container/user.rs` - Modified: add `UserRole` enum, `role` field on `UserData`, `role()`/`set_role()` accessors.
- `crates/paladin-ports/src/output/auth_port.rs` - New: `AuthPort` trait, `AuthToken`, `AuthClaims`, `AuthError`.
- `crates/paladin-ports/src/output/mod.rs` - Modified: register/re-export the new `auth_port` module.
- `crates/paladin-ports/src/output/user_repository_port.rs` - Reference: existing `delete`/query methods used by the service surface.
- `src/infrastructure/adapters/auth/in_memory_token_auth_adapter.rs` - New: concrete `AuthPort` adapter (opaque tokens, hashed store, expiry, revoke).
- `src/infrastructure/adapters/auth/mod.rs` - New: module registration for the auth adapter.
- `src/core/platform/manager/user_service.rs` - Modified: add `delete_user`/`list_users` to the service surface; issue a token on login (or expose a login-with-token path).
- `crates/paladin-core/src/platform/manager/user_service.rs` - Modified: extend `UserServiceTrait` + DTOs (`UserAuthenticationResult` token fields, delete/list).
- `src/infrastructure/repositories/sqlite_user_repository.rs` - Modified: persist `role` (schema column + row mapping + idempotent migration).
- `crates/paladin-storage/src/sqlite_user_repository.rs` - Modified: persist `role` (schema column + row mapping + idempotent migration).
- `crates/paladin-web/src/auth_middleware.rs` - New: Axum auth extractor/middleware (`401`) + admin role guard (`403`).
- `crates/paladin-web/src/user_controller.rs` - Modified: login returns token; add admin-only `delete`/`list` routes; self-scope checks.
- `crates/paladin-web/src/app.rs` - New: `create_app_router(user_service, auth_port)` composing public + protected routes.
- `crates/paladin-web/src/lib.rs` - Modified: declare/re-export new modules.
- `crates/paladin-web/Cargo.toml` - Modified: add `tower` + `http-body-util` dev-dependencies for router tests.
- `crates/paladin-web/tests/auth_rbac.rs` - New: offline integration tests (401/200/403/admin-success).

### Notes

- Unit tests live in `#[cfg(test)] mod tests` blocks within the modules they test (`user.rs`,
  `auth_port.rs`, `in_memory_token_auth_adapter.rs`, `auth_middleware.rs`).
- Integration tests for the web layer go in `crates/paladin-web/tests/` and use
  `tower::ServiceExt::oneshot` + `http-body-util` with a mock `UserServiceTrait` and the in-memory
  `AuthPort` adapter — fully offline and deterministic.
- Run `cargo test` for all tests; `cargo test -p paladin-web` for the web crate;
  `cargo test --test auth_rbac -p paladin-web` for the integration suite.
- Quality gate per parent task: `cargo test`, `cargo fmt --check`, `cargo clippy -- -D warnings`.
- Crate boundary: `paladin-ports` must NOT depend on the root crate; the auth adapter (root crate)
  depends on both; `paladin-web` depends only on `paladin-ports` + `paladin-core` and never does
  cryptography itself (it takes `Arc<dyn AuthPort>`).
- Use `set +H &&` before git commits; stage only the specific files changed in each parent task.
  Do NOT stage the pre-existing prompt-file changes in the working tree.

## Instructions for Completing Tasks

**IMPORTANT:** As you complete each task, check it off by changing `- [ ]` to `- [x]`. Update after
each sub-task, not just each parent task.

## Tasks

- [x] 0.0 Create feature branch
  - [x] 0.1 Create and checkout `feature/milestone_9-epic_5-user-admin-system-completion` from the current branch.

- [ ] 1.0 Add `UserRole` to the domain and persist it (Task 5.3 foundation; FR 1–4)
  - [ ] 1.1 Add a `UserRole` enum (`Admin`, `User`) to `crates/paladin-core/src/platform/container/user.rs` with `as_str()` and `FromStr`/`from_str`-style parsing, plus rustdoc; derive `Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize` and a `Default` of `User`.
  - [ ] 1.2 Add `role: UserRole` to `UserData` (default `User`); update the user constructor(s) so existing call sites still compile.
  - [ ] 1.3 Add `User::role()` accessor and `User::set_role(UserRole)` mutator following the `Node<UserData>` pattern.
  - [ ] 1.4 Update `src/infrastructure/repositories/sqlite_user_repository.rs`: add `role` column (`TEXT NOT NULL DEFAULT 'user'`) with idempotent migration; read/write the role in row mapping.
  - [ ] 1.5 Update `crates/paladin-storage/src/sqlite_user_repository.rs` the same way (schema + idempotent migration + row mapping).
  - [ ] 1.6 Fix any other `UserData` construction sites broken by the new field (grep for `UserData {`).
  - [ ] 1.7 Unit tests: `UserRole` string round-trip + default; user role accessor/mutator; repository persists and reads back a non-default role.
  - [ ] 1.8 Quality gate (`cargo test`, `cargo fmt --check`, `cargo clippy -- -D warnings`), then commit.

- [ ] 2.0 Define the `AuthPort` interface in `paladin-ports` (Task 5.2; FR 5–7)
  - [ ] 2.1 Create `crates/paladin-ports/src/output/auth_port.rs` with module-level rustdoc.
  - [ ] 2.2 Define `AuthToken { token: String, expires_at: DateTime<Utc> }` and `AuthClaims { user_id: Uuid, role: UserRole, expires_at: DateTime<Utc> }` (serializable where appropriate).
  - [ ] 2.3 Define `AuthError` (thiserror): `MissingToken`, `InvalidToken`, `Expired`, `Internal(String)`.
  - [ ] 2.4 Define `AuthPort: Send + Sync` (`async_trait`) with `issue_token`, `verify_token`, `revoke_token` returning `Result<_, AuthError>`; full rustdoc.
  - [ ] 2.5 Register `pub mod auth_port;` and re-export public types in `crates/paladin-ports/src/output/mod.rs`.
  - [ ] 2.6 Unit tests for value-object/`AuthError` basics expressible in the port crate.
  - [ ] 2.7 Quality gate, then commit.

- [ ] 3.0 Implement the token auth adapter in the root crate (Task 5.2; FR 8–11)
  - [ ] 3.1 Create `src/infrastructure/adapters/auth/in_memory_token_auth_adapter.rs` with module docs; register `mod.rs`.
  - [ ] 3.2 Implement opaque token generation (`rand`, 32 random bytes hex), a hashed in-memory store (`sha2`, `RwLock<HashMap<hash, AuthClaims>>`), and a configurable TTL (default 24h).
  - [ ] 3.3 Implement `AuthPort`: `issue_token` (store hash → claims), `verify_token` (hash lookup, reject missing/invalid/expired), `revoke_token` (remove entry).
  - [ ] 3.4 Wire the module into the infrastructure adapters tree and re-export the adapter type.
  - [ ] 3.5 Unit tests: issue→verify round-trip; expired token rejected (`Expired`); revoked token rejected; unknown token rejected (`InvalidToken`).
  - [ ] 3.6 Quality gate, then commit.

- [ ] 4.0 Complete user CRUD + token-issuing login on the service surface (Task 5.1; FR 12, 18–20)
  - [ ] 4.1 Extend `UserServiceTrait` (in `paladin-core`) with `delete_user(user_id)` and `list_users()` (backed by existing repository methods).
  - [ ] 4.2 Add token fields to the login result (or a new `login_with_token` path) so a successful login returns a token + expiry; keep the existing method working for backward compatibility.
  - [ ] 4.3 Implement the new trait methods on the concrete `UserService` (root crate), delegating delete/list to the repository and issuing the token via `AuthPort` (injected as `Arc<dyn AuthPort>`).
  - [ ] 4.4 Ensure all user responses/DTOs continue to omit the password hash.
  - [ ] 4.5 Unit tests: delete removes a user; list returns users; login returns a non-empty token with future expiry.
  - [ ] 4.6 Quality gate, then commit.

- [ ] 5.0 Add Axum auth middleware + RBAC guard in `paladin-web` (Tasks 5.2–5.3; FR 13–17)
  - [ ] 5.1 Create `crates/paladin-web/src/auth_middleware.rs`: an extractor/middleware that reads `Authorization: Bearer <token>`, calls `AuthPort::verify_token`, injects `AuthClaims` into request extensions, and returns `401` (JSON body, non-revealing) on missing/invalid/expired.
  - [ ] 5.2 Add an admin role guard that returns `403` when `AuthClaims.role != Admin`.
  - [ ] 5.3 Add a self-scope check helper so a non-admin may only access their own `:id` (else `403`).
  - [ ] 5.4 Unit tests for the middleware/guard decision logic using a mock `AuthPort` (401 vs 200 vs 403 outcomes at the function level).
  - [ ] 5.5 Quality gate, then commit.

- [ ] 6.0 Protect routes and compose the app router (Task 5.3; FR 16–21)
  - [ ] 6.1 Update `user_controller.rs`: login handler returns the issued token; add admin-only `DELETE /users/:id` and `GET /users` (list) handlers; apply self-scope checks to `get`/`update`.
  - [ ] 6.2 Create `crates/paladin-web/src/app.rs` with `create_app_router(user_service, auth_port)` composing public routes (`register`, `login`) and protected routes (everything else) with auth middleware + admin guard layered appropriately.
  - [ ] 6.3 Declare/re-export the new `auth_middleware` and `app` modules in `crates/paladin-web/src/lib.rs`.
  - [ ] 6.4 Quality gate, then commit.

- [ ] 7.0 Integration tests for auth + RBAC (Task 5.2–5.3; FR 22–23)
  - [ ] 7.1 Add `tower` + `http-body-util` dev-dependencies to `crates/paladin-web/Cargo.toml`.
  - [ ] 7.2 Create `crates/paladin-web/tests/auth_rbac.rs` with a mock `UserServiceTrait` and the in-memory `AuthPort` adapter; build the app via `create_app_router`.
  - [ ] 7.3 Assert: protected route without token → `401`; with valid token → `200`.
  - [ ] 7.4 Assert: admin-only route with `user`-role token → `403`; with `admin`-role token → success.
  - [ ] 7.5 Quality gate, then commit.

- [ ] 8.0 Final verification & Epic close-out (FR 24)
  - [ ] 8.1 Run full quality gate: `cargo build`, `cargo test`, `cargo clippy -- -D warnings`, `cargo fmt --check` (including the `web-server` feature path).
  - [ ] 8.2 Run `snyk_code_scan` on new first-party code; fix and rescan until clean (or substitute static compiler/clippy checks if the tool is unavailable, and note it).
  - [ ] 8.3 Mark the PRD Task Checklist items complete; ensure "Relevant Files" above is accurate.
  - [ ] 8.4 Final commit if any cleanup remains.
