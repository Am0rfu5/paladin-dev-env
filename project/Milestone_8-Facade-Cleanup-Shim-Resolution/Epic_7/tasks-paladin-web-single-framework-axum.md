# Tasks: `paladin-web` — Single Web Framework (axum), remove actix-web (Milestone 8, Epic 7)

Derived from `prd-paladin-web-single-framework-axum.md`. Audience: a junior developer.

> **PRD assumptions carried into these tasks** (open questions in the PRD): delivery routes are
> **public** (no auth) for behavior parity; delivery routes are exposed via a dedicated
> `create_delivery_routes(...)` builder **merged** into the app router; the cargo-deny ban targets
> `actix-web` specifically. Adjust if these decisions change.

## Relevant Files

- `crates/paladin-web/src/adapters/api_content_deliverer.rs` - Remove the actix `configure()` + 3
  actix handlers and the `actix_web` import; keep the reqwest-based `ApiContentDeliverer` service.
- `crates/paladin-web/src/delivery_controller.rs` - **(new)** axum handlers + `create_delivery_routes`
  route-builder for the delivery endpoints (mirrors `user_controller.rs`). In-file `#[cfg(test)]`
  unit tests for the handlers.
- `crates/paladin-web/src/app.rs` - Mount the delivery routes into the application router.
- `crates/paladin-web/src/lib.rs` - Declare the new module; fix the crate doc-comment to say axum
  only (remove "actix-web and axum").
- `crates/paladin-web/Cargo.toml` - Remove the `actix-web` dependency.
- `deny.toml` - Add `actix-web` to the banned-crates list with a rationale.
- `project/current-exports.txt` - Regenerated public API-surface baseline.
- `CHANGELOG.md` - `[Unreleased]` entry documenting the framework consolidation.

### Notes

- Rust unit tests live in the same file under `#[cfg(test)] mod tests { ... }`. Mirror the test
  style in `crates/paladin-web/src/user_controller.rs`.
- For exercising a built axum `Router` in tests, use `tower::ServiceExt::oneshot` with
  `axum::http::Request` + `axum::body::Body` (add `tower` as a dev-dependency only if not already
  present). Alternatively, call the handler functions directly as `user_controller.rs` does.
- Run tests with `cargo test -p paladin-web`; run the full gate with `cargo test`,
  `cargo fmt --check`, `cargo clippy -- -D warnings`.
- Verify dependency removal with `cargo tree -p paladin-web | rg actix` (expect no output) and the
  ban with `make deny`.
- Behavior parity matters: keep the same paths, methods, JSON shapes, and status codes as the
  original actix handlers (see PRD §4).
- The three `ApiContentDeliverer` methods to call already exist and are public:
  `deliver_content_async(DeliveryRequest) -> Result<DeliveryResponse, ContentDeliveryError>`,
  `get_delivery_status(Uuid) -> Result<DeliveryResponse, _>`,
  `get_delivery_stats(None) -> Result<DeliveryStats, _>`.

## Tasks

- [x] 0.0 Create feature branch
  - [x] 0.1 `git checkout main && git pull --ff-only origin main` to start from the latest tip.
  - [x] 0.2 `git checkout -b feature/m8-epic7-paladin-web-axum-only`.

- [x] 1.0 Reimplement the three delivery endpoints as axum handlers (TDD: tests first)
  - [x] 1.1 Create `crates/paladin-web/src/delivery_controller.rs` with a `#[cfg(test)] mod tests`
    block **first**. Write failing tests that build the router from `create_delivery_routes(...)`
    (Task 2.1) with a real `ApiContentDeliverer` (no scheduler) and assert:
    (a) `GET /api/delivery/stats` → `200 OK`;
    (b) `GET /api/delivery/status/{random-uuid}` → `404 Not Found`;
    (c) `GET /api/delivery/status/not-a-uuid` → `400 Bad Request` with body
    `{"error":"Invalid delivery ID format"}`;
    (d) `POST /api/delivery/deliver` with a minimal valid `DeliveryRequest` → asserts a `2xx`/`4xx`
    matching `deliver_content_async`'s result shape.
  - [x] 1.2 Add a small JSON error helper that returns `{ "error": "<message>" }` (parity with the
    old actix responses), or reuse an existing `ApiResponse`-style helper if one fits.
  - [x] 1.3 Implement `async fn deliver_content(State<Arc<ApiContentDeliverer>>, Json<DeliveryRequest>)`:
    call `deliver_content_async`; `Ok(resp)` → `(StatusCode::OK, Json(resp))`,
    `Err(e)` → `(StatusCode::BAD_REQUEST, Json(error_json(e)))`.
  - [x] 1.4 Implement `async fn get_delivery_status(State<...>, Path<String>)`: `Uuid::parse_str` →
    on parse error `400` (`"Invalid delivery ID format"`); on success call `get_delivery_status`,
    `Ok` → `200`, `Err` → `404`.
  - [x] 1.5 Implement `async fn get_delivery_stats(State<...>)`: call `get_delivery_stats(None)`;
    `Ok` → `200`, `Err` → `500`.
  - [x] 1.6 `cargo test -p paladin-web` — the new tests now pass.

- [x] 2.0 Expose a `create_delivery_routes` builder and mount it into the application router
  - [x] 2.1 In `delivery_controller.rs`, add
    `pub fn create_delivery_routes(deliverer: Arc<ApiContentDeliverer>) -> axum::Router` registering
    `POST /api/delivery/deliver`, `GET /api/delivery/status/{delivery_id}`,
    `GET /api/delivery/stats`, finished with `.with_state(deliverer)`.
  - [x] 2.2 In `app.rs`, merge the delivery routes into the composed app. Add a
    `deliverer: Arc<ApiContentDeliverer>` parameter to `create_app_router` (or add a sibling
    composed builder) and `.merge(create_delivery_routes(deliverer))`. Keep the existing user
    public/protected/admin routes and auth middleware unchanged.
  - [x] 2.3 Update any callers/tests of `create_app_router` to pass the new argument.
  - [x] 2.4 Add a router-level test (in `app.rs` or `delivery_controller.rs`) asserting a delivery
    route is reachable through the fully composed router.
  - [x] 2.5 `cargo build -p paladin-web` and `cargo test -p paladin-web`.

- [x] 3.0 Remove actix-web from `paladin-web`
  - [x] 3.1 In `api_content_deliverer.rs`, delete `configure()` and the three actix handler
    functions (`deliver_content_handler`, `get_delivery_status_handler`,
    `get_delivery_stats_handler`) and remove
    `use actix_web::{HttpResponse, Result as ActixResult, web};`. Leave `ApiContentDeliverer` and
    its `ContentDeliveryService`/`BatchContentDeliveryService`/reqwest code untouched.
  - [x] 3.2 In `lib.rs`, add `pub mod delivery_controller;` and change the crate doc-comment from
    "actix-web and axum" to "axum".
  - [x] 3.3 Remove `actix-web = { version = "4.0" }` from `crates/paladin-web/Cargo.toml`.
  - [x] 3.4 `rg actix crates/paladin-web/` → expect **no matches**; `cargo build -p paladin-web`.

- [x] 4.0 Add the cargo-deny guardrail banning `actix-web`
  - [x] 4.1 In `deny.toml`, add `actix-web` under `[bans] deny` with a one-line rationale
    (e.g. "paladin-web standardizes on axum; a second HTTP framework is not allowed").
  - [x] 4.2 Run `make deny` (or `cargo deny check bans`) → passes (actix-web is gone).
  - [x] 4.3 (Verification, optional) temporarily re-add `actix-web` to `paladin-web` and confirm
    `make deny` fails, then revert.

- [x] 5.0 Refresh the public API-surface baseline and update `CHANGELOG.md`
  - [x] 5.1 Regenerate: `./scripts/extract-public-api.sh project/current-exports.txt`.
  - [x] 5.2 Review the diff — it should show only the expected `paladin-web` surface change
    (actix `configure` removed; new `delivery_controller` / `create_delivery_routes` added).
  - [x] 5.3 `./scripts/check-api-surface.sh project/current-exports.txt` → passes.
  - [x] 5.4 Add a `CHANGELOG.md` `[Unreleased]` entry (under the Milestone 8 section): **Added** the
    served axum delivery routes; **Changed/Removed** actix-web from `paladin-web`; note the
    cargo-deny ban.

- [ ] 6.0 Final verification, commit, and PR
  - [ ] 6.1 `cargo test` (default) and `cargo test --features web-server` → all green.
  - [ ] 6.2 `cargo fmt --check` and `cargo clippy --workspace --features web-server -- -D warnings`.
  - [ ] 6.3 `cargo tree -p paladin-web | rg actix` → no output; note the dropped transitive crates.
  - [ ] 6.4 Commit (conventional message referencing M8 Epic 7); push and open a PR against `main`.
