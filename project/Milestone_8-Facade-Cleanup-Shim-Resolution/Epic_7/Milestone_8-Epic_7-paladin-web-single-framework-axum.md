## Epic 7: `paladin-web` — Single Web Framework (remove actix-web)

**Priority:** Medium
**Estimated Effort:** Small–Medium
**Dependencies:** None (independent cleanup within the `paladin-web` leaf crate)

### Background

`paladin-web` depends on **two** HTTP frameworks — **axum** and **actix-web**. The crate's
served HTTP API (user management) is entirely axum (`app.rs`, `user_controller.rs`,
`auth_middleware.rs`). `actix-web` appears only in
`crates/paladin-web/src/adapters/api_content_deliverer.rs`, which defines an actix `configure()`
plus three handlers (`deliver`, `status`, `stats`).

Those actix handlers are **orphaned**: no actix `HttpServer` is ever started anywhere in the
workspace, and `configure()` is never called, so the endpoints are never served. The dependency
exists solely to compile dead code, which is wasteful (a full second async-HTTP stack, larger
build + attack surface) and confusing (two frameworks, unclear ownership).

The useful part of the file — `ApiContentDeliverer`, the **reqwest-based** outbound delivery
service implementing the `ContentDeliveryService` / `BatchContentDeliveryService` ports (used by
the scheduler tests) — does **not** use actix and is kept as-is.

### Objective

Revive the content-delivery endpoints as **served axum routes**, mount them into the application
router, remove the `actix-web` dependency from `paladin-web`, and add a **cargo-deny ban** to
prevent a second web framework from returning. Update the public API-surface baseline and
`CHANGELOG.md` accordingly.

### Decisions (from PRD clarification)

- **Handler disposition:** port the three endpoints to axum **and mount them** (revive the API),
  rather than deleting them.
- **Guardrail:** add `actix-web` to `deny.toml`'s banned crates.

### Scope

- In scope: `crates/paladin-web/` (adapter handlers → axum, router mounting, `Cargo.toml`,
  `lib.rs` docs, handler unit tests); `deny.toml`; `project/current-exports.txt`; `CHANGELOG.md`.
- Out of scope: changing delivery data models / ports / `ApiContentDeliverer` logic; adding auth to
  the delivery routes; any other crate.

### Acceptance Criteria

- `actix-web` removed from `paladin-web` (`rg actix crates/paladin-web/` → no matches;
  `cargo tree -p paladin-web` no longer shows actix-web).
- `POST /api/delivery/deliver`, `GET /api/delivery/status/{id}`, `GET /api/delivery/stats` exist as
  axum routes, mounted, with behavior parity (status codes + `{ "error": ... }` bodies) and unit
  tests.
- `make deny` fails if `actix-web` is reintroduced.
- API-surface baseline regenerated; `CHANGELOG.md [Unreleased]` updated.
- Workspace builds/clippy/fmt/tests green on default and `web-server` features.

See `prd-paladin-web-single-framework-axum.md` for full functional requirements, and run
`/generate-tasks` against that PRD to produce the implementation checklist
(`tasks-paladin-web-single-framework-axum.md`).
