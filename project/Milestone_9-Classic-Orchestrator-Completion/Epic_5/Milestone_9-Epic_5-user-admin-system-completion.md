# Milestone 9 — Epic 5: User and Admin System Completion

**Project:** Paladin Framework
**Milestone:** 9 — Classic Orchestrator, Content Pipeline, and Agent-Orchestrator Bridge
**Epic:** 5 of 6
**Priority:** Medium
**Estimated Effort:** Medium
**Dependencies:** Epics 1, 2 (services must be operational for auth to protect)
**Status:** Planning

---

## Objective

Complete the user authentication, authorization, and admin operations so the system can be deployed
with access control.

## Background

The orchestration, scheduler, and queue subsystems become operational in Epics 1–2. Before the
platform can be deployed, the surfaces that expose these capabilities (the web API and/or CLI) must
be protected by authentication and authorization. The `UserService` and supporting types exist, but
the auth flow and RBAC enforcement are incomplete.

## Scope

**In scope:**
- User CRUD through the API (or CLI when `paladin-web` is disabled).
- Password hashing with `argon2`.
- API key / token-based authentication and middleware.
- Role-based access control (admin vs. user).

**Out of scope:**
- Full multi-tenant authorization or fine-grained per-resource ACLs beyond the admin/user split.
- OAuth/OIDC or external identity-provider integration.

---

## Tasks

### Task 5.1: User CRUD via API

**Description:** Validate that `UserService` CRUD operations (create, read, update, delete) work
through the web API endpoints (if `paladin-web` is enabled) or via CLI commands.

**Implementation notes:**
- Verify password hashing with `argon2` on create and password-change paths; never store or log
  plaintext passwords.
- Apply input validation at the API boundary (e.g., username/email format, password strength).

**Deliverables:**
- User CRUD functional and tested.
- Password hashing with `argon2` verified.

**Acceptance criteria:**
- Create/read/update/delete round-trips succeed via API and/or CLI.
- Stored credentials are `argon2` hashes; plaintext never persisted or logged.

---

### Task 5.2: Authentication Flow

**Description:** Implement or validate API key or token-based authentication. Requests to protected
endpoints must include a valid credential.

**Implementation notes:**
- Centralize credential verification in auth middleware so all protected routes share one code path.
- Return `401 Unauthorized` for missing/invalid credentials and avoid leaking which part failed.
- Ensure tokens/API keys are compared in constant time and stored hashed where applicable.

**Deliverables:**
- Auth middleware functional.
- Unauthorized requests return 401.
- Integration test for auth flow.

**Acceptance criteria:**
- A valid credential grants access; a missing/invalid one returns `401`.
- The auth path is covered by an integration test.

---

### Task 5.3: Role-Based Access Control

**Description:** Implement basic RBAC: admin vs. user roles. Admin can manage users, view system
health, access all workflows. User can execute agents, create workflows within their scope.

**Implementation notes:**
- Enforce role checks in middleware or a guard layer, not ad hoc in handlers.
- Return `403 Forbidden` (distinct from `401`) when an authenticated user lacks the required role.
- Ensure admin-only operations (user management, system health) are unreachable by the `user` role.

**Deliverables:**
- RBAC enforcement on API endpoints.
- Admin notification and logging services operational.
- Integration tests for role-based access.

**Acceptance criteria:**
- Admin-only endpoints reject `user`-role requests with `403`.
- User-scoped endpoints succeed for the `user` role and are correctly scoped.

---

## Definition of Done

- User CRUD works with `argon2`-hashed credentials.
- Authentication middleware protects endpoints; unauthorized requests return `401`.
- RBAC distinguishes admin and user roles; forbidden requests return `403`.
- Integration tests cover auth and RBAC.
- `cargo build`, `cargo test`, `cargo clippy -- -D warnings`, and `cargo fmt --check` all pass.
