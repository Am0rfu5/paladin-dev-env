## Relevant Files

### New Crate Files (to be created)

- `crates/paladin-storage/Cargo.toml` — New crate manifest; declares `sqlx` with `sqlite`/`mysql` feature flags.
- `crates/paladin-storage/src/lib.rs` — Crate root; `pub mod` declarations and conditional re-exports.
- `crates/paladin-storage/src/sqlite_content_repository.rs` — Moved from `src/infrastructure/repositories/sqlite_content_repository.rs`.
- `crates/paladin-storage/src/sqlite_user_repository.rs` — Moved from `src/infrastructure/repositories/sqlite_user_repository.rs`.
- `crates/paladin-storage/src/mysql_content_repository.rs` — Moved from `src/infrastructure/repositories/mysql_content_repository.rs`.
- `crates/paladin-notifications/Cargo.toml` — New crate manifest; `email`/`push`/`system` feature flags gating `lettre`/`handlebars`.
- `crates/paladin-notifications/src/lib.rs` — Crate root; conditional `pub mod` declarations.
- `crates/paladin-notifications/src/email_notification_adapter.rs` — Moved from `src/infrastructure/adapters/notifications/email_notification_adapter.rs`.
- `crates/paladin-notifications/src/push_notification_adapter.rs` — Moved from `src/infrastructure/adapters/notifications/push_notification_adapter.rs`.
- `crates/paladin-notifications/src/system_notification_adapter.rs` — Moved from `src/infrastructure/adapters/notifications/system_notification_adapter.rs`.
- `crates/paladin-content/Cargo.toml` — New crate manifest; `pdf`/`web-scraping`/`rss`/`news-api`/`tiktoken` feature flags.
- `crates/paladin-content/src/lib.rs` — Crate root; conditional `pub mod` for adapters and use_cases sub-modules.
- `crates/paladin-content/src/adapters/document/` — Moved from `src/infrastructure/adapters/document/`.
- `crates/paladin-content/src/adapters/input/` — Moved from `src/infrastructure/adapters/input/` (excluding `tensorflow_adapter.rs`).
- `crates/paladin-content/src/use_cases/` — Moved from `src/application/use_cases/content/`.
- `crates/paladin-web/Cargo.toml` — New crate manifest; `actix-web` and `axum` as direct (non-optional) dependencies.
- `crates/paladin-web/src/lib.rs` — Crate root; moved from `src/infrastructure/web/mod.rs`.
- `crates/paladin-web/src/user_controller.rs` — Moved from `src/infrastructure/web/user_controller.rs`.
- `crates/paladin-web/src/adapters/api_content_deliverer.rs` — Moved from `src/infrastructure/adapters/output/api_content_deliverer.rs`.

### Port Trait Files (prerequisite move for storage extraction)

- `crates/paladin-ports/src/output/repository_port.rs` — **New file.** Repository port traits moved here from `src/application/storage/sql_store.rs`: `ContentRepository`, `ContentListRepository`, `MigrationManager`, `TransactionManager`, `SqlStore`, `RepositoryError`, `RepositoryStats`.
- `crates/paladin-ports/src/output/mod.rs` — Add `pub mod repository_port;`.
- `crates/paladin-ports/src/lib.rs` — Add re-exports for new repository port types.

### Existing Files (to be modified)

- `Cargo.toml` — Workspace root/facade: add new crates to `[workspace.members]` and `[workspace.dependencies]`; add optional deps; redefine feature flags; remove extracted third-party deps.
- `src/application/storage/sql_store.rs` — Replace trait definitions with `pub use paladin_ports::output::repository_port::*;` re-exports after traits move to `paladin-ports`.
- `src/infrastructure/repositories/mod.rs` — Replace direct module declarations with re-exports from `paladin-storage` (temporary bridge, then removed).
- `src/infrastructure/adapters/notifications/mod.rs` — Replace direct module declarations with re-exports from `paladin-notifications` (temporary bridge, then removed).
- `src/infrastructure/adapters/document/mod.rs` — Replace direct module declarations with re-exports from `paladin-content` (temporary bridge, then removed).
- `src/infrastructure/adapters/input/mod.rs` — Replace non-tensorflow entries with re-exports from `paladin-content`; keep `tensorflow_adapter` declaration in place.
- `src/infrastructure/adapters/output/mod.rs` — Replace `api_content_deliverer` declaration with re-export from `paladin-web` (temporary bridge, then removed).
- `src/infrastructure/web/mod.rs` — Replace with re-export from `paladin-web` (temporary bridge, then removed).
- `src/config/setup/service_runner.rs` — Gate `SqliteStore` import behind `#[cfg(feature = "storage-sqlite")]`; update import path to `paladin_storage`.
- `src/lib.rs` — Update re-exports for types that have moved to new crates; add `#[cfg(feature = "...")]` guards.

### Test Files (import paths to update)

- `tests/repository/mysql_content_repository_test.rs` — Update imports to use `paladin_storage`.
- `tests/repository.rs` — Update module declarations.
- `tests/integration/notification_system_integration_test.rs` — Update imports to use `paladin_notifications`.
- `tests/functional/content_fetching_pipeline_test.rs` — Update imports to use `paladin_content`.
- `tests/functional/content_lifecycle_test.rs` — Update imports to use `paladin_content`.
- `tests/functional/content_llm_analysis_pipeline_test.rs` — Update imports to use `paladin_content`.
- `tests/integration/openai_content_analysis_integration_test.rs` — Update imports to use `paladin_content`.

### Assessment and Planning Files

- `project/Milestone_7-Production-Hardening/Epic_1/cost-benefit-assessment.md` — Cost-benefit matrix produced by Task 1.0.

### Notes

- Unit tests in Rust live **in the same file as the code they test**, inside a `#[cfg(test)] mod tests { use super::*; }` block. Move them with their source file into the new crate.
- Integration tests requiring Docker services (`mysql`, `minio`) remain at `tests/integration/` with updated import paths.
- Run `cargo test --workspace` after each crate extraction to catch regressions before proceeding to the next.
- The recommended extraction order is: **storage → notifications → content → web** (simplest to most complex). Reorder only if Task 1.0 findings justify it.
- Task 1.0 is a **hard gate**: no source files may be moved until Go/Defer decisions are recorded.
- `tensorflow_adapter.rs` stays in the facade crate throughout this milestone; do not move it.
- `file_content_repository.rs` stays in the facade crate throughout this milestone; do not move it.
- The repository port traits (`ContentRepository`, `ContentListRepository`, `MigrationManager`, etc.) currently live in `src/application/storage/sql_store.rs`. They must move to `paladin-ports` **before** `paladin-storage` can implement them — this is sub-task 2.1.

---

## Instructions for Completing Tasks

**IMPORTANT:** As you complete each task, you must check it off in this markdown file by changing `- [ ]` to `- [x]`. This helps track progress and ensures you don't skip any steps.

Example:

- `- [ ] 1.1 Read file` → `- [x] 1.1 Read file` (after completing)

Update the file after completing each sub-task, not just after completing an entire parent task.

---

## Tasks

- [x] 0.0 Create feature branch
  - [x] 0.1 Create new branch into  (`git checkout -b feature/milestone_7-epic_1-production_hardening`).

- [x] 1.0 Cost-Benefit Assessment (Hard Gate — must complete before any extraction)
  - [x] 1.1 Read `project/Milestone_7-Production-Hardening/Epic_1/prd-extract-infrastructure-crates.md` §4.1 and §6.2 to confirm the four assessment criteria and recommended order before starting the matrix.
  - [x] 1.2 For each of the four candidates (`paladin-storage`, `paladin-notifications`, `paladin-content`, `paladin-web`), evaluate: (a) dependency weight — run `cargo tree -p paladin --features <flag>` to measure transitive deps introduced; (b) change frequency — review recent git log for each subsystem directory; (c) consumer selectivity — assess whether a typical agent-only consumer would want this crate; (d) extraction complexity — count files, imports, circular dependency risks.
  - [x] 1.3 Write findings and Go/Defer decisions to `project/Milestone_7-Production-Hardening/Epic_1/cost-benefit-assessment.md`.
  - [x] 1.4 For each **Defer** decision: update the corresponding task entry below to mark it `[DEFERRED]` with a one-line reason and a link to the assessment file. _(N/A — all four candidates received Go decisions)_
  - [x] 1.5 For each **Defer** decision: create a backlog ticket titled `Extract paladin-{name} crate` tagged `milestone-8+-candidate` with a link to `cost-benefit-assessment.md`. _(N/A — no Defer decisions)_
  - [x] 1.6 Confirm all Go decisions with the team lead or via documented self-approval before proceeding to Task 2.0. _(Self-approved in `cost-benefit-assessment.md`, 2026-05-25)_

- [ ] 2.0 Extract `paladin-storage` crate
  - [ ] 2.1 **Prerequisite — move repository port traits to `paladin-ports`:** Create `crates/paladin-ports/src/output/repository_port.rs`. Move the following from `src/application/storage/sql_store.rs`: `RepositoryError`, `RepositoryStats`, `ContentRepository`, `ContentListRepository`, `TransactionManager`, `MigrationManager`, `SqlStore`. Update `crates/paladin-ports/src/output/mod.rs` to add `pub mod repository_port;`. Update `crates/paladin-ports/src/lib.rs` with re-exports. Replace the original definitions in `sql_store.rs` with `pub use paladin_ports::output::repository_port::*;`.
  - [ ] 2.2 Verify `cargo build --workspace` still passes after the port trait move (no regressions before any file is moved).
  - [ ] 2.3 Create `crates/paladin-storage/Cargo.toml` with `[package]` at version `0.1.0`, `[features]` (`sqlite`, `mysql`), and dependencies: `paladin-ports = { workspace = true }`, `paladin-core = { workspace = true }`, `sqlx = { workspace = true, optional = true }`, `tokio`, `thiserror`, `serde`, `uuid`, `chrono` (all `{ workspace = true }`).
  - [ ] 2.4 Create `crates/paladin-storage/src/lib.rs` with `#[cfg(feature = "sqlite")] pub mod sqlite_content_repository;`, `#[cfg(feature = "sqlite")] pub mod sqlite_user_repository;`, `#[cfg(feature = "mysql")] pub mod mysql_content_repository;`.
  - [ ] 2.5 Move `src/infrastructure/repositories/sqlite_content_repository.rs` → `crates/paladin-storage/src/sqlite_content_repository.rs`. Update all `crate::` imports to use `paladin_ports::output::repository_port::` for the trait types.
  - [ ] 2.6 Move `src/infrastructure/repositories/sqlite_user_repository.rs` → `crates/paladin-storage/src/sqlite_user_repository.rs`. Update `crate::` imports similarly.
  - [ ] 2.7 Move `src/infrastructure/repositories/mysql_content_repository.rs` → `crates/paladin-storage/src/mysql_content_repository.rs`. Update `crate::` imports similarly.
  - [ ] 2.8 Add temporary re-exports in `src/infrastructure/repositories/mod.rs`: `#[cfg(feature = "storage-sqlite")] pub use paladin_storage::sqlite_content_repository::*;` etc. This keeps existing consumers compiling during the transition.
  - [ ] 2.9 Verify `cargo build -p paladin-storage --features sqlite` succeeds in isolation.
  - [ ] 2.10 Verify `cargo build -p paladin-storage --features mysql` succeeds in isolation.
  - [ ] 2.11 Update `[workspace.dependencies]` in root `Cargo.toml` to add `mysql` to the `sqlx` features list: `sqlx = { version = "0.8", features = ["runtime-tokio-rustls", "sqlite", "mysql", "chrono", "uuid", "json"] }`.
  - [ ] 2.12 Add `paladin-storage = { path = "crates/paladin-storage" }` to `[workspace.dependencies]`. Add `"crates/paladin-storage"` to `[workspace.members]`.
  - [ ] 2.13 Add to facade `[dependencies]`: `paladin-storage = { workspace = true, optional = true }`. Add to facade `[features]`: `storage-sqlite = ["dep:paladin-storage", "paladin-storage/sqlite"]`, `storage-mysql = ["dep:paladin-storage", "paladin-storage/mysql"]`, `storage = ["storage-sqlite", "storage-mysql"]`.
  - [ ] 2.14 Update `src/config/setup/service_runner.rs`: gate the `SqliteStore` import with `#[cfg(feature = "storage-sqlite")]`; update import path to `paladin_storage::sqlite_content_repository::SqliteStore`.
  - [ ] 2.15 Update workspace-root `tests/repository/mysql_content_repository_test.rs` and `tests/repository.rs` to import from `paladin_storage` instead of the facade.
  - [ ] 2.16 Remove the temporary re-exports from `src/infrastructure/repositories/mod.rs` once all internal consumers have been updated to import from `paladin_storage` directly.
  - [ ] 2.17 Run `cargo test --workspace` and confirm all tests pass.

- [ ] 3.0 Extract `paladin-notifications` crate
  - [ ] 3.1 Create `crates/paladin-notifications/Cargo.toml` with version `0.1.0`, feature flags `email = ["dep:lettre", "dep:handlebars"]`, `push`, `system`, and dependencies: `paladin-ports = { workspace = true }`, `paladin-core = { workspace = true }`, `lettre = { version = "0.11.17", ..., optional = true }`, `handlebars = { version = "6.3.2", optional = true }`, `async-trait`, `thiserror`, `serde` (workspace).
  - [ ] 3.2 Create `crates/paladin-notifications/src/lib.rs` with conditional `pub mod` declarations for each adapter, gated by their respective feature flags.
  - [ ] 3.3 Move `src/infrastructure/adapters/notifications/email_notification_adapter.rs` → `crates/paladin-notifications/src/email_notification_adapter.rs`. Update `crate::` imports to use `paladin_ports`.
  - [ ] 3.4 Move `src/infrastructure/adapters/notifications/push_notification_adapter.rs` → `crates/paladin-notifications/src/push_notification_adapter.rs`. Update imports.
  - [ ] 3.5 Move `src/infrastructure/adapters/notifications/system_notification_adapter.rs` → `crates/paladin-notifications/src/system_notification_adapter.rs`. Update imports.
  - [ ] 3.6 Add temporary re-exports in `src/infrastructure/adapters/notifications/mod.rs`: `pub use paladin_notifications::*;` (gated on `notifications` feature). Keep the file in place as a bridge.
  - [ ] 3.7 Verify `cargo build -p paladin-notifications --no-default-features` succeeds (crate skeleton compiles without adapters).
  - [ ] 3.8 Verify `cargo build -p paladin-notifications --all-features` succeeds.
  - [ ] 3.9 Add `paladin-notifications = { path = "crates/paladin-notifications" }` to `[workspace.dependencies]`. Add `"crates/paladin-notifications"` to `[workspace.members]`.
  - [ ] 3.10 Update facade `Cargo.toml`: add `paladin-notifications = { workspace = true, optional = true }` to `[dependencies]`. Redefine `notifications` feature: `notifications = ["dep:paladin-notifications", "paladin-notifications/email", "paladin-notifications/push", "paladin-notifications/system"]`. Remove `lettre` and `handlebars` from facade `[dependencies]`.
  - [ ] 3.11 Update `tests/integration/notification_system_integration_test.rs` imports to reference `paladin_notifications` types directly.
  - [ ] 3.12 Remove temporary re-exports from `src/infrastructure/adapters/notifications/mod.rs` once all internal consumers are updated.
  - [ ] 3.13 Run `cargo test --workspace` and confirm all tests pass.

- [ ] 4.0 Extract `paladin-content` crate
  - [ ] 4.1 Create `crates/paladin-content/Cargo.toml` with version `0.1.0`, feature flags `pdf = ["dep:pdf-extract"]`, `web-scraping = ["dep:scraper"]`, `rss = ["dep:rss"]`, `news-api`, `tiktoken = ["dep:tiktoken-rs"]`, and dependencies: `paladin-ports = { workspace = true }`, `paladin-core = { workspace = true }`, `paladin-llm = { workspace = true, optional = true }` (needed for LLM analysis services), `reqwest`, `tokio`, `async-trait`, `serde`, `serde_json`, `thiserror`, `chrono`, `pdf-extract`, `scraper`, `rss`, `tiktoken-rs` (all optional, workspace or versioned).
  - [ ] 4.2 Create directory structure: `crates/paladin-content/src/adapters/document/`, `crates/paladin-content/src/adapters/input/`, `crates/paladin-content/src/use_cases/`.
  - [ ] 4.3 Move document adapter files: `pdf_extractor.rs`, `document_adapter.rs` → `crates/paladin-content/src/adapters/document/`. Create `crates/paladin-content/src/adapters/document/mod.rs` re-exporting them.
  - [ ] 4.4 Move input adapter files (excluding `tensorflow_adapter.rs`): `file_content_fetcher.rs`, `file_content_list_fetcher.rs`, `http_content_fetcher.rs`, `local_file_fetcher.rs`, `news_api_fetcher.rs` → `crates/paladin-content/src/adapters/input/`. Create `crates/paladin-content/src/adapters/input/mod.rs`.
  - [ ] 4.5 Verify `tensorflow_adapter.rs` remains at `src/infrastructure/adapters/input/tensorflow_adapter.rs`. If it has no `ml` feature flag gate yet, add `#[cfg(feature = "ml")]` and add `ml` to the facade's `[features]` table.
  - [ ] 4.6 Move all 13 content use-case services and `mod.rs` from `src/application/use_cases/content/` → `crates/paladin-content/src/use_cases/`. Update all `crate::` imports to use `paladin_ports`, `paladin_core`, or `paladin_llm` as appropriate.
  - [ ] 4.7 Create `crates/paladin-content/src/adapters/mod.rs` (re-exports `document` and `input` sub-modules) and `crates/paladin-content/src/lib.rs` (re-exports `adapters` and `use_cases`, all gated on relevant feature flags).
  - [ ] 4.8 Add temporary re-exports in the original facade locations:
        - `src/infrastructure/adapters/document/mod.rs` → `pub use paladin_content::adapters::document::*;`
        - `src/infrastructure/adapters/input/mod.rs` → add `pub use paladin_content::adapters::input::*;` (keep `pub mod tensorflow_adapter;` in place)
        - `src/application/use_cases/content/mod.rs` → `pub use paladin_content::use_cases::*;`
  - [ ] 4.9 Verify `cargo build -p paladin-content --no-default-features` succeeds.
  - [ ] 4.10 Verify `cargo build -p paladin-content --all-features` succeeds.
  - [ ] 4.11 Add `paladin-content = { path = "crates/paladin-content" }` to `[workspace.dependencies]`. Add `"crates/paladin-content"` to `[workspace.members]`.
  - [ ] 4.12 Update facade `Cargo.toml`: add `paladin-content = { workspace = true, optional = true }`. Redefine `content-processing` feature: `content-processing = ["dep:paladin-content", "paladin-content/pdf", "paladin-content/web-scraping", "paladin-content/rss", "paladin-content/news-api", "paladin-content/tiktoken"]`. Remove `pdf-extract`, `scraper`, `tiktoken-rs`, and `rss` from facade `[dependencies]`.
  - [ ] 4.13 Update `tests/functional/content_fetching_pipeline_test.rs`, `tests/functional/content_lifecycle_test.rs`, `tests/functional/content_llm_analysis_pipeline_test.rs`, and `tests/integration/openai_content_analysis_integration_test.rs` to import from `paladin_content`.
  - [ ] 4.14 Remove temporary re-exports from `src/infrastructure/adapters/document/mod.rs`, `src/infrastructure/adapters/input/mod.rs` (content entries only), and `src/application/use_cases/content/mod.rs` once all internal consumers are updated.
  - [ ] 4.15 Run `cargo test --workspace` and confirm all tests pass.

- [ ] 5.0 Extract `paladin-web` crate
  - [ ] 5.1 Create `crates/paladin-web/Cargo.toml` with version `0.1.0` and direct (non-optional) dependencies: `actix-web = "4.0"`, `axum = "0.8.4"`, `paladin-ports = { workspace = true }`, `paladin-core = { workspace = true }`, `tokio`, `serde`, `serde_json`, `thiserror` (workspace).
  - [ ] 5.2 Create directory structure: `crates/paladin-web/src/adapters/`.
  - [ ] 5.3 Move `src/infrastructure/web/user_controller.rs` → `crates/paladin-web/src/user_controller.rs`. Update `crate::` imports.
  - [ ] 5.4 Move `src/infrastructure/adapters/output/api_content_deliverer.rs` → `crates/paladin-web/src/adapters/api_content_deliverer.rs`. Update imports (port traits come from `paladin_ports`).
  - [ ] 5.5 Create `crates/paladin-web/src/lib.rs` from the contents of `src/infrastructure/web/mod.rs`. Add `pub mod user_controller;` and `pub mod adapters;`. Update `crate::` imports.
  - [ ] 5.6 Create `crates/paladin-web/src/adapters/mod.rs` re-exporting `api_content_deliverer`.
  - [ ] 5.7 Add temporary re-exports:
        - `src/infrastructure/web/mod.rs` → `pub use paladin_web::*;`
        - `src/infrastructure/adapters/output/mod.rs` → `#[cfg(feature = "web-server")] pub use paladin_web::adapters::*;`
  - [ ] 5.8 Verify `cargo build -p paladin-web` succeeds in isolation.
  - [ ] 5.9 Update `src/config/setup/service_runner.rs`: wrap all web-related imports and fields with `#[cfg(feature = "web-server")]`; update import paths to use `paladin_web`.
  - [ ] 5.10 Add `paladin-web = { path = "crates/paladin-web" }` to `[workspace.dependencies]`. Add `"crates/paladin-web"` to `[workspace.members]`.
  - [ ] 5.11 Update facade `Cargo.toml`: add `paladin-web = { workspace = true, optional = true }`. Redefine `web-server` feature: `web-server = ["dep:paladin-web"]`. Remove `actix-web` and `axum` from facade `[dependencies]`.
  - [ ] 5.12 Remove temporary re-exports from `src/infrastructure/web/mod.rs` and `src/infrastructure/adapters/output/mod.rs` once all internal consumers are updated.
  - [ ] 5.13 Run `cargo test --workspace` and confirm all tests pass.

- [ ] 6.0 Update facade crate and workspace metadata
  - [ ] 6.1 Confirm all four new crates appear in `[workspace.members]` in root `Cargo.toml` (added incrementally in Tasks 2–5; verify completeness now).
  - [ ] 6.2 Confirm all four new crates appear in `[workspace.dependencies]` with correct paths.
  - [ ] 6.3 Update the facade's `full` convenience feature to enable all four new crates alongside existing capabilities: `full = [..., "storage", "notifications", "content-processing", "web-server", ...]`.
  - [ ] 6.4 Audit facade `[dependencies]` for any remaining third-party packages that are now owned exclusively by an extracted crate. Remove them (e.g. verify `lettre`, `handlebars`, `pdf-extract`, `scraper`, `tiktoken-rs`, `rss`, `actix-web`, `axum` are gone).
  - [ ] 6.5 Verify all public re-exports in `src/lib.rs` still resolve correctly. Add `#[cfg(feature = "...")]` guards around re-exports for types in optional crates.
  - [ ] 6.6 Cross-check `STABLE_API.md` and `api_surface_current.txt` against current public paths. Confirm no public paths have been silently removed.
  - [ ] 6.7 Run `cargo doc --workspace --no-deps` and fix any errors or warnings.

- [ ] 7.0 Final workspace validation
  - [ ] 7.1 Run `cargo build --workspace` (default features) — must pass.
  - [ ] 7.2 Run `cargo build --workspace --all-features` — must pass.
  - [ ] 7.3 Run `cargo build --workspace --no-default-features` — must pass (verifies no accidental unconditional deps were introduced).
  - [ ] 7.4 Run `cargo test --workspace` — must pass with no regressions.
  - [ ] 7.5 Run `cargo clippy --workspace -- -D warnings` — must pass with zero warnings.
  - [ ] 7.6 Run `cargo fmt --check` — must pass.
  - [ ] 7.7 Run `cargo tree -p paladin-core --all-features` and verify output does **not** contain `actix-web`, `axum`, `lettre`, `pdf-extract`, `scraper`, or `sqlx`.
  - [ ] 7.8 Run `cargo tree -p paladin-battalion --all-features` and verify the same set of crates is absent.
  - [ ] 7.9 Run `cargo doc --workspace --no-deps` — zero errors, zero warnings.
  - [ ] 7.10 Stage all changes: `git add .`
  - [ ] 7.11 Commit using conventional commit format:
        ```
        git commit \
          -m "feat: extract paladin-storage, paladin-notifications, paladin-content, paladin-web crates" \
          -m "- Move repository port traits to paladin-ports/output/repository_port.rs" \
          -m "- Extract SqliteStore, MySql, SqliteUserRepository into paladin-storage (sqlite/mysql features)" \
          -m "- Extract email/push/system notification adapters into paladin-notifications" \
          -m "- Extract 9 content adapters and 13 use-case services into paladin-content" \
          -m "- Extract actix-web/axum server and user_controller into paladin-web" \
          -m "- Redefine facade feature flags to activate new crates rather than raw third-party deps" \
          -m "- All four new crates are opt-in; paladin-core/paladin-battalion no longer transitively pull heavy deps" \
          -m "Closes Epic 1, Milestone 7"
        ```
