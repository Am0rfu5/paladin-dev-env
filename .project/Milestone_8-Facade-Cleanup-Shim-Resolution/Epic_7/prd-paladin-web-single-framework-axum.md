# PRD: `paladin-web` — Consolidate on a Single Web Framework (axum) and Remove actix-web (Milestone 8, Epic 7)

> **Correction (dated 2026-08-06, DEBT-01):** This document instructs a future implementer to
> write the public-API surface baseline to the pre-rename `project/` path in two places (FR-10 and
> §7 "Technical Considerations", both struck below) — a path that has not existed since commit
> `928c6d5` renamed `project/` to `.project/`. The baseline lives at `.project/current-exports.txt`,
> confirmed present at 442,369 bytes via `ls -la .project/current-exports.txt`, re-run during this
> task; the pre-rename path is confirmed absent via `ls` on it, which returns "No such file or
> directory", also re-run during this task. This is one of five requirement documents
> Phase 8 / DEBT-01 corrects on the requirement-text side; the corresponding tooling
> (`scripts/check-api-surface.sh`, `scripts/extract-public-api.sh`,
> `.github/workflows/ci.yml`) was corrected separately in plan 08-02. Original text is retained
> below with inline corrections — nothing is deleted.

**Project:** Paladin Framework
**Milestone:** 8 — Facade Cleanup, Shim Resolution, and Directory Stabilization
**Epic:** 7 — `paladin-web` single-framework consolidation (remove actix-web)
**Version Target:** post-v0.5.1 (Unreleased)
**Status:** Ready for Implementation
**Created:** 2026-06-06
**Author:** AI Coding Agent (Claude Code)

---

## 1. Introduction / Overview

The `paladin-web` crate currently depends on **two** HTTP web frameworks: **axum** and
**actix-web**. The crate's real, served HTTP API (user management) is built entirely with
**axum** (`app.rs`, `user_controller.rs`, `auth_middleware.rs`). **actix-web** is used in exactly
one place — the content-delivery adapter at
`crates/paladin-web/src/adapters/api_content_deliverer.rs` — which defines an actix
`configure()` function plus three actix HTTP handlers (`deliver`, `status`, `stats`).

Those actix handlers are **orphaned**: nothing in the workspace ever starts an actix
`HttpServer`, and `configure()` is never called, so the handlers are never mounted or served.
The dependency pulls an entire second async-HTTP framework into the build solely to compile dead
endpoints, which is wasteful (large transitive dependency tree, longer builds, larger attack
surface) and confusing (two frameworks in one crate; unclear which one "owns" the HTTP layer).

This Epic **revives the content-delivery endpoints as real, served axum routes**, mounts them
into the application router, **removes the `actix-web` dependency** from `paladin-web`, and adds a
**cargo-deny ban** so a second web framework cannot silently return.

The genuinely useful part of the adapter — `ApiContentDeliverer`, the **reqwest-based outbound
delivery service** that implements the `ContentDeliveryService` / `BatchContentDeliveryService`
ports (and is exercised by the scheduler tests) — is **kept unchanged**; it does not use actix.

## 2. Goals

1. `paladin-web` depends on **exactly one** HTTP web framework (axum); `actix-web` is removed from
   `crates/paladin-web/Cargo.toml`.
2. The three content-delivery endpoints (`deliver`, `status`, `stats`) are reimplemented as axum
   handlers and are **mounted into the application router** so they are actually served.
3. No regression: the existing axum user-management API and the `ApiContentDeliverer` service
   behave exactly as before; all workspace tests pass.
4. A guardrail (cargo-deny) prevents `actix-web` (or another redundant web framework) from being
   reintroduced without an explicit, reviewed decision.
5. The public API-surface baseline and `CHANGELOG.md` are updated to reflect the change, so the
   `API Surface Tracking` CI job passes.

## 3. User Stories

- **As a maintainer**, I want `paladin-web` to use a single web framework so the HTTP layer is
  consistent, the dependency tree is smaller, and new contributors aren't confused about which
  framework to use.
- **As a contributor adding an HTTP endpoint**, I want one obvious pattern (axum `Router` +
  `State` extractors) to copy, so I don't accidentally write actix handlers that never get served.
- **As an operator/integrator**, I want the content-delivery endpoints (`POST /api/delivery/deliver`,
  `GET /api/delivery/status/{id}`, `GET /api/delivery/stats`) to be real, reachable routes when the
  web server runs.
- **As a security reviewer**, I want a CI guard that fails the build if a second web framework is
  added, so framework sprawl can't slip in unnoticed.

## 4. Functional Requirements

1. The system **must** reimplement the following endpoints as axum handlers, preserving the paths,
   methods, request/response bodies, and status-code semantics of the existing actix handlers:
   1.1. `POST /api/delivery/deliver` — body `DeliveryRequest` (JSON) → calls
        `ApiContentDeliverer::deliver_content_async`; on `Ok` returns `200 OK` with the
        `DeliveryResponse` JSON; on `Err` returns `400 Bad Request` with `{ "error": "<message>" }`.
   1.2. `GET /api/delivery/status/{delivery_id}` — path param `delivery_id` (UUID string) → on a
        valid UUID, calls `ApiContentDeliverer::get_delivery_status`; `Ok` → `200 OK` with the
        `DeliveryResponse` JSON; `Err` → `404 Not Found` with `{ "error": ... }`; an unparseable
        UUID → `400 Bad Request` with `{ "error": "Invalid delivery ID format" }`.
   1.3. `GET /api/delivery/stats` — calls `ApiContentDeliverer::get_delivery_stats(None)`; `Ok` →
        `200 OK` with `DeliveryStats` JSON; `Err` → `500 Internal Server Error` with `{ "error": ... }`.
2. The system **must** expose a public route-builder for these endpoints following the existing
   convention (mirroring `create_app_router` / the user routes), e.g.
   `pub fn create_delivery_routes(deliverer: Arc<ApiContentDeliverer>) -> axum::Router`, using an
   axum `State<Arc<ApiContentDeliverer>>` extractor for dependency injection.
3. The system **must** mount the delivery routes into the application router so they are served
   alongside the user-management routes (extend `create_app_router`, or provide a composed router
   that merges both; the chosen approach must keep the existing user routes and their auth
   middleware unchanged).
4. The system **must** remove the actix `configure()` function and the three actix handler
   functions from `api_content_deliverer.rs`, and remove the
   `use actix_web::{HttpResponse, Result as ActixResult, web};` import.
5. The system **must** remove `actix-web` from `crates/paladin-web/Cargo.toml` dependencies.
6. The system **must not** modify the `ApiContentDeliverer` struct or its `ContentDeliveryService` /
   `BatchContentDeliveryService` / reqwest behavior, beyond what is needed to call its existing
   public methods from the new axum handlers.
7. The system **must** update the `paladin-web` crate-level doc comment
   (`crates/paladin-web/src/lib.rs`) to state that the crate uses **axum** (removing the
   "actix-web and axum" wording).
8. The system **must** add `actix-web` to the banned-crates list in `deny.toml` with a short
   rationale, so `make deny` / the CI dependency-policy job fails if it is reintroduced.
9. The system **must** add unit tests for the three new axum handlers covering: a successful
   response, the error/`404` path for unknown delivery id, and the `400` path for an invalid UUID
   (mirroring the test style already used in `user_controller.rs`).
10. ~~The system **must** regenerate the public API-surface baseline~~
    ~~(`./scripts/extract-public-api.sh project/current-exports.txt`) and add a `CHANGELOG.md`~~
    ~~`[Unreleased]` entry describing the framework consolidation and any public-API change in~~
    ~~`paladin-web` (the removal of the actix `configure`/handlers and the new axum route-builder).~~
    **Corrected (dated 2026-08-06, DEBT-01):** The correct baseline path is
    `.project/current-exports.txt`, not the pre-rename path struck above — the directory was
    renamed by commit `928c6d5`. The correct command is
    `./scripts/extract-public-api.sh .project/current-exports.txt`. Confirmed via
    `ls -la .project/current-exports.txt` (442,369 bytes present) and `ls` on the struck path
    ("No such file or directory"), both re-run during this task. The `CHANGELOG.md` clause is
    unaffected.
11. The workspace **must** build, lint (`cargo clippy -- -D warnings`), format-check, and test
    cleanly on the default feature set and with the `web-server` feature enabled.

## 5. Non-Goals (Out of Scope)

- Adding authentication/authorization to the delivery endpoints (the original actix handlers had
  none; this Epic preserves behavior — auth can be a follow-up).
- Changing the delivery request/response data models, the `ContentDeliveryService` port, or the
  `ApiContentDeliverer` delivery/retry/scheduling logic.
- Migrating any other crate or introducing a shared HTTP-server abstraction.
- Adding OpenAPI/schema generation, rate limiting, or new delivery features.
- Wiring the `paladin-web` server into the binary's runtime startup if it is not already (server
  bootstrap/runtime composition is a separate concern).

## 6. Design Considerations

- **Endpoint parity:** keep the exact paths and JSON shapes so the change is behavior-preserving
  (a revival, not a redesign). Error responses keep the `{ "error": "<message>" }` shape used by
  the actix handlers.
- **Consistency:** mirror the patterns already in `user_controller.rs` / `app.rs` — `Router::new()`
  with `.route(...)`, typed handlers using `State<T>` and `Json<T>`, and `.with_state(...)` to
  inject the `Arc<ApiContentDeliverer>`. A junior developer should be able to copy the user-routes
  structure almost verbatim.

## 7. Technical Considerations

- ~~**Crate / layer:** all changes are confined to the `paladin-web` leaf crate, plus `deny.toml`,~~
  ~~`project/current-exports.txt`, and `CHANGELOG.md` at the workspace root. No facade (`src/`)~~
  ~~source changes are expected (the facade already re-exports `paladin_web::*` via~~
  ~~`src/infrastructure/web/mod.rs` under the `web-server` feature).~~
  **Corrected (dated 2026-08-06, DEBT-01):** The workspace-root artefact this bullet names is
  `.project/current-exports.txt`, not the pre-rename path struck above — see the FR-10 correction
  above for the evidence. The rest of this bullet (`deny.toml`, `CHANGELOG.md`, the no-facade-change
  expectation) is unaffected.
- **State injection:** actix used `web::Data<ApiContentDeliverer>`; the axum equivalent is
  `State<Arc<ApiContentDeliverer>>`. `ApiContentDeliverer` is already `Clone` and built around
  `Arc<Mutex<...>>` internals, so sharing via `Arc` is straightforward.
- **Service methods to call** (already public on `ApiContentDeliverer`):
  `deliver_content_async(DeliveryRequest) -> Result<DeliveryResponse, ContentDeliveryError>`,
  `get_delivery_status(Uuid) -> Result<DeliveryResponse, _>`,
  `get_delivery_stats(None) -> Result<DeliveryStats, _>`.
- **Router composition:** prefer extending `create_app_router` (or adding a sibling
  `create_delivery_routes` that the composition root merges) so existing user routes + auth
  middleware are untouched. If `create_app_router`'s signature changes, update its callers/tests
  in `paladin-web` accordingly.
- **Public-API impact:** removing the actix `configure`/handlers and adding an axum route-builder
  changes `paladin-web`'s public surface, which the facade re-exports — hence the mandatory
  baseline regen + CHANGELOG entry (Requirement 10). There are no external consumers, so this is
  acceptable; document it as a change.
- **cargo-deny:** add `actix-web` (and reasonably `actix-*` core crates if the ban should be
  framework-wide) under `[bans] deny` in `deny.toml`; verify with `make deny`.
- **Edition/deps:** `axum 0.8` is already a dependency; no new dependency should be required. After
  removal, confirm `actix-web` (and its now-unused transitive deps) drop out of `Cargo.lock`.

## 8. Success Metrics

- `rg actix crates/paladin-web/` returns **zero** matches in source and `Cargo.toml`.
- `cargo tree -p paladin-web` no longer lists `actix-web`.
- The three delivery endpoints are present in the mounted router and covered by passing unit tests.
- `make deny` fails if `actix-web` is re-added (verified once).
- CI is green, including `API Surface Tracking` (baseline regenerated) and the `web-server`
  build/test matrix entry.
- Net reduction in `paladin-web` dependency count and build time (qualitative; note the removed
  transitive crates in the PR).

## 9. Open Questions

1. **Delivery-route auth:** the original endpoints were unauthenticated. Should the revived axum
   routes remain public, or be placed behind the existing `require_auth` middleware? (Assumed
   public for behavior parity; flag if auth is desired.)
2. **Mount path ownership:** should `/api/delivery/*` be merged into the single `create_app_router`,
   or returned as a separate `Router` the composition root mounts? (Either satisfies the
   requirements; pick the one that keeps `create_app_router` cohesive.)
3. **Ban breadth:** ban only `actix-web`, or all `actix-*` framework crates? (Default: `actix-web`;
   widen if the team wants a hard framework-level guard.)
