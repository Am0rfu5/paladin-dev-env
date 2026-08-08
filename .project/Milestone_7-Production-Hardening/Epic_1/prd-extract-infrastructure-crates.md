# PRD: Extract Remaining Infrastructure Subsystems into Dedicated Crates

**Feature Name:** extract-infrastructure-crates
**Milestone:** 7 — Production Hardening and Extended Workspace Decomposition
**Epic:** 1 — Extended Workspace Decomposition — Remaining Infrastructure Crates
**Status:** Ready for Implementation
**Created:** 2026-05-25
**Author:** AI-assisted, reviewed by team

> **Correction (dated 2026-08-08, HARD-05):** Two clauses in this document — Goal 2 below and §6.1
> "Crate Dependency Rules" below, both struck — state the extracted-crate dependency rule in an
> absolute form ("never... on the facade") that the shipped tree satisfies only in its **default
> build**.
> [ADR-0031](../../../.planning/decisions/0031-extracted-crate-dependency-rule.md) restates the
> enforceable invariant against the default build: no extracted crate may depend on another
> extracted crate or on the facade in its default build; a non-default optional feature may
> declare such an edge only where the facade opts in explicitly and the dependent code is
> `cfg`-gated. This is a **promotion** of the invariant to its general form — the original absolute
> form becomes the special case of a crate with no non-default features — and **not** permission
> for a default-build edge: `paladin-content`'s `llm` feature already satisfies both conditions
> today. The original text below is retained with inline corrections; nothing is deleted.
>
> **This document is annotated by two plans in this phase.** Plan 10-04 owns Goal 2 and §6.1
> (HARD-05, this banner). Plan 10-05 owns §4.4.1 and §4.4.6 (HARD-06) and adds its own separate
> dated blockquote in a later wave. This banner covers only the two clauses named in it.

---

## 1. Introduction / Overview

After Milestones 1–6, the Paladin workspace contains six crates (`paladin-core`, `paladin-ports`, `paladin-battalion`, `paladin-llm`, `paladin-memory`, and the `paladin` facade). Four substantial infrastructure subsystems remain compiled directly into the facade crate, gated only by feature flags:

- **Web server** (`infrastructure/web/`, `infrastructure/adapters/output/api_content_deliverer.rs`) — Actix-web + Axum HTTP/WebSocket server.
- **Notification adapters** (`infrastructure/adapters/notifications/`) — Email (`lettre`), push, and system notification adapters.
- **Content processing** (`infrastructure/adapters/document/`, `infrastructure/adapters/input/`, `application/use_cases/content/`) — PDF extraction, HTTP/file/RSS/News-API fetching, web scraping, and thirteen content use-case services.
- **SQL repositories** (`infrastructure/repositories/`) — SQLite and MySQL `sqlx`-backed repository implementations and migration management.

**The problem this solves:** A downstream consumer who only wants Paladin's agent orchestration capabilities (`paladin-core` + `paladin-ports` + `paladin-battalion` + `paladin-llm`) currently transitively receives `actix-web`, `axum`, `lettre`, `pdf-extract`, `scraper`, `sqlx`, and all their dependency trees, even if all four feature flags are disabled at compile time. Feature flags eliminate compilation but not dependency resolution. Only crate extraction makes these dependencies truly opt-in at the `Cargo.toml` level.

**The goal:** Extract the four subsystems into `crates/paladin-web`, `crates/paladin-notifications`, `crates/paladin-content`, and `crates/paladin-storage`. Each new crate must be independently compilable and testable. The facade crate retains only re-exports, the `ServiceRunner` composition root, and application-layer use-case services that do not warrant their own crate. All four new crates are opt-in (not in `default` features of the facade).

---

## 2. Goals

1. Create four new workspace crates: `paladin-web`, `paladin-notifications`, `paladin-content`, `paladin-storage`.
2. ~~Each new crate must depend only on `paladin-core`, `paladin-ports`, and workspace-shared dependencies — never on other new infrastructure crates or on the facade.~~

   **Corrected (dated 2026-08-08, HARD-05):** [ADR-0031](../../../.planning/decisions/0031-extracted-crate-dependency-rule.md) restates this as a default-build invariant: no extracted crate may depend on another extracted crate or on the facade **in its default build**; a non-default optional feature may declare such an edge only where the facade opts in explicitly and the dependent code is `cfg`-gated. Checkable per crate via `cargo tree --no-default-features`.
3. All four new crates are **opt-in** from the facade crate's perspective; they are not enabled in `default` features.
4. All new crates are versioned at `0.1.0` in lockstep with the rest of the workspace.
5. A downstream consumer depending on `paladin-core + paladin-ports + paladin-battalion + paladin-llm` must not transitively pull in `actix-web`, `axum`, `lettre`, `pdf-extract`, `scraper`, or `sqlx`.
6. Unit tests for each subsystem are migrated into the corresponding new crate's own test modules (`#[cfg(test)]` blocks and per-crate `tests/` directory). Integration tests referencing multiple crates remain at the workspace-root `tests/` directory.
7. `cargo build --workspace` and `cargo test --workspace` both pass after every extraction step.
8. Before any extraction begins, a cost-benefit assessment gates the go/defer decision for each of the four crates.

---

## 3. User Stories

**As a framework consumer building a lightweight agent application,**
I want to add `paladin-core`, `paladin-ports`, `paladin-battalion`, and `paladin-llm` to my `Cargo.toml` without pulling in web server, email, PDF, or database dependencies,
so that my binary is small and my build is fast.

**As a developer working exclusively on notification adapters,**
I want `paladin-notifications` to compile in isolation with `cargo build -p paladin-notifications`,
so that I can iterate on email/push/system adapters without recompiling the entire workspace.

**As a developer integrating only the content pipeline,**
I want to add `paladin-content` with specific feature flags (`pdf`, `web-scraping`, `rss`) to opt into only the capabilities I need,
so that I do not pay the compile-time cost of capabilities I never use.

**As a developer integrating SQL persistence,**
I want `paladin-storage` to expose `sqlite` and `mysql` feature flags independently,
so that I can include only the database backend my application actually uses.

**As a security engineer auditing the dependency tree,**
I want `actix-web`, `lettre`, `pdf-extract`, `scraper`, and `sqlx` to appear only in the `Cargo.lock` entries for the crates that explicitly declare them,
so that I can audit each crate's attack surface in isolation.

---

## 4. Functional Requirements

### 4.1 Task 1.1 — Cost-Benefit Assessment (Hard Gate)

4.1.1 Before any code is moved, the developer must produce a cost-benefit matrix for each of the four candidate extractions.

4.1.2 The matrix must evaluate each candidate on four criteria:
  - (a) **Dependency weight** — compile-time cost introduced by the subsystem's external dependencies.
  - (b) **Change frequency** — how often this subsystem changes independently of others.
  - (c) **Consumer selectivity** — likelihood that a downstream consumer wants this subsystem without the others.
  - (d) **Extraction complexity** — estimated effort (circular imports, test migration, service-runner integration).

4.1.3 Each candidate must receive a **Go** or **Defer** decision with written justification.

4.1.4 The following steps constitute the **definition of done** for Task 1.1:
  1. Write the full matrix and all Go/Defer decisions to `project/Milestone_7-Production-Hardening/Epic_1/cost-benefit-assessment.md`.
  2. For each **Defer** decision: update the corresponding task entry in the Epic tracker (this directory) to mark it as `deferred — see cost-benefit-assessment.md`.
  3. For each **Defer** decision: create a backlog ticket titled `Extract paladin-{name} crate` tagged `milestone-8+-candidate`, with a link to the assessment findings, so the deferral is not lost during future milestone planning.

4.1.5 The remaining extractions proceed without the deferred crate(s). The cost-benefit matrix is the authoritative source of record for *why* a decision was made.

---

### 4.2 Task 1.2 — Extract `paladin-web` Crate

4.2.1 Create `crates/paladin-web/` with a `Cargo.toml` that declares `actix-web` and `axum` as **direct** (non-optional) dependencies. These must NOT appear in the facade crate's `[dependencies]` after extraction.

4.2.2 Move the following source files into `crates/paladin-web/src/`:
  - `src/infrastructure/web/mod.rs` → `crates/paladin-web/src/lib.rs` (or `mod.rs` equivalent)
  - `src/infrastructure/web/user_controller.rs` → `crates/paladin-web/src/user_controller.rs`
  - `src/infrastructure/adapters/output/api_content_deliverer.rs` → `crates/paladin-web/src/adapters/api_content_deliverer.rs`

4.2.3 The crate must compile in isolation: `cargo build -p paladin-web` must succeed.

4.2.4 The `ServiceRunner` in the facade crate must be updated to conditionally depend on `paladin-web` when the `web-server` feature flag is active.

4.2.5 The facade crate's `web-server` feature flag is redefined to activate the `paladin-web` dependency rather than the raw `actix-web`/`axum` dependencies.

4.2.6 All `#[cfg(test)]` unit tests originally co-located with the moved source files must be relocated into the new crate's source. Integration tests in `tests/` that reference web types must be updated to import from `paladin-web`.

4.2.7 `cargo build --workspace` and `cargo test --workspace` must pass after this task.

---

### 4.3 Task 1.3 — Extract `paladin-notifications` Crate

4.3.1 Create `crates/paladin-notifications/` with feature flags: `email` (gates `lettre` + `handlebars`), `push`, `system`.

4.3.2 Move the following source files into `crates/paladin-notifications/src/`:
  - `src/infrastructure/adapters/notifications/email_notification_adapter.rs`
  - `src/infrastructure/adapters/notifications/push_notification_adapter.rs`
  - `src/infrastructure/adapters/notifications/system_notification_adapter.rs`
  - `src/infrastructure/adapters/notifications/mod.rs`

4.3.3 `lettre` and `handlebars` must NOT appear in the facade crate's `[dependencies]` after extraction.

4.3.4 The crate must compile with no default features: `cargo build -p paladin-notifications --no-default-features` must succeed (compiles the crate skeleton without any adapter implementations).

4.3.5 The facade crate's `notifications` feature flag is redefined to activate the `paladin-notifications` dependency (with `features = ["email", "push", "system"]`) rather than `lettre` and `handlebars` directly.

4.3.6 Unit tests for each adapter are relocated into the new crate. Integration tests for notification delivery remain at the workspace root.

4.3.7 `cargo build --workspace` and `cargo test --workspace` must pass after this task.

---

### 4.4 Task 1.4 — Extract `paladin-content` Crate

4.4.1 Create `crates/paladin-content/` with feature flags: `pdf` (gates `pdf-extract`), `web-scraping` (gates `scraper`), `rss` (gates the `rss` crate), `news-api` (gates `NewsApiFetcher` HTTP logic), `tiktoken` (gates `tiktoken-rs`).

4.4.2 Move the following infrastructure adapters into `crates/paladin-content/src/adapters/`:
  - `src/infrastructure/adapters/document/pdf_extractor.rs`
  - `src/infrastructure/adapters/document/document_adapter.rs`
  - `src/infrastructure/adapters/document/mod.rs`
  - `src/infrastructure/adapters/input/file_content_fetcher.rs`
  - `src/infrastructure/adapters/input/file_content_list_fetcher.rs`
  - `src/infrastructure/adapters/input/http_content_fetcher.rs`
  - `src/infrastructure/adapters/input/local_file_fetcher.rs`
  - `src/infrastructure/adapters/input/news_api_fetcher.rs`
  - `src/infrastructure/adapters/input/mod.rs`

4.4.3 Move the following application-layer use-case services into `crates/paladin-content/src/use_cases/`:
  - `src/application/use_cases/content/content_aggregator_service.rs`
  - `src/application/use_cases/content/content_analysis_service.rs`
  - `src/application/use_cases/content/content_delivery_service.rs`
  - `src/application/use_cases/content/content_fetching_service.rs`
  - `src/application/use_cases/content/content_filtering_service.rs`
  - `src/application/use_cases/content/content_ingestion_service.rs`
  - `src/application/use_cases/content/content_list_fetching_service.rs`
  - `src/application/use_cases/content/content_list_ingestion_service.rs`
  - `src/application/use_cases/content/content_list_service.rs`
  - `src/application/use_cases/content/content_llm_analysis_service.rs`
  - `src/application/use_cases/content/content_ml_analysis_service.rs`
  - `src/application/use_cases/content/content_nlp_analysis_service.rs`
  - `src/application/use_cases/content/content_summarizer_service.rs`
  - `src/application/use_cases/content/mod.rs`

4.4.4 `pdf-extract`, `scraper`, `tiktoken-rs`, and the `rss` crate must NOT appear in the facade crate's `[dependencies]` after extraction.

4.4.5 The crate must compile with no default features: `cargo build -p paladin-content --no-default-features` must succeed.

4.4.6 The facade crate's `content-processing` feature flag is redefined to activate the `paladin-content` dependency (with all capability features enabled) rather than the raw `pdf-extract`, `scraper`, `tiktoken-rs`, and `rss` dependencies directly.

4.4.7 Unit tests for each adapter and service are relocated into the new crate. Content-related integration tests remain at the workspace root.

4.4.8 `cargo build --workspace` and `cargo test --workspace` must pass after this task.

---

### 4.5 Task 1.5 — Extract `paladin-storage` Crate

4.5.1 Create `crates/paladin-storage/` with feature flags: `sqlite` (gates `sqlx` with the `sqlite` runtime), `mysql` (gates `sqlx` with the `mysql` runtime).

4.5.2 Move the following source files into `crates/paladin-storage/src/`:
  - `src/infrastructure/repositories/sqlite_content_repository.rs`
  - `src/infrastructure/repositories/sqlite_user_repository.rs`
  - `src/infrastructure/repositories/mysql_content_repository.rs`
  - `src/infrastructure/repositories/mod.rs`

  > **Note:** `src/infrastructure/repositories/file_content_repository.rs` is **not** moved. Despite its filename, it implements `ContentDeliveryService` / `BatchContentDeliveryService` from `paladin-ports` and writes content to the local filesystem; it does not use `sqlx` and has no relationship to SQL-backed persistence. It stays in the facade crate. A future content-delivery crate (Milestone 8+) is its correct long-term home.

4.5.3 Move SQLite migration files from `migrations/` to `crates/paladin-storage/migrations/`. The `sqlx::migrate!` macro path must be updated accordingly.

4.5.4 `sqlx` (with `sqlite` and `mysql` features) must NOT appear in the facade crate's `[workspace.dependencies]` or `[dependencies]` as a direct dependency after extraction. The facade depends on `paladin-storage` to pull in `sqlx` transitively when storage is needed.

4.5.5 The crate must compile with the sqlite feature only: `cargo build -p paladin-storage --features sqlite` must succeed. `cargo build -p paladin-storage --features mysql` must also succeed independently.

4.5.6 The facade crate gains two granular optional feature flags that mirror the pattern used by `paladin-llm` and `paladin-memory`:

  ```toml
  # In paladin-storage/Cargo.toml [features]:
  default = []
  sqlite = ["dep:sqlx/sqlite"]
  mysql  = ["dep:sqlx/mysql"]

  # In facade Cargo.toml [features]:
  storage-sqlite = ["dep:paladin-storage", "paladin-storage/sqlite"]
  storage-mysql  = ["dep:paladin-storage", "paladin-storage/mysql"]
  ```

  A single `storage` convenience alias may also be added that activates both: `storage = ["storage-sqlite", "storage-mysql"]`. A downstream consumer depending only on SQLite must not link `libmysqlclient`, and vice versa.

4.5.7 Unit tests for each repository implementation are relocated into the new crate. Database integration tests (requiring Docker services) remain at the workspace root `tests/integration/`.

4.5.8 `cargo build --workspace` and `cargo test --workspace` must pass after this task.

---

### 4.6 Task 1.6 — Update Facade Crate and Workspace Metadata

4.6.1 After all Go-decisions from Task 1.1 have been executed, update `Cargo.toml` (workspace root / facade crate) as follows:
  - Add each extracted crate to `[workspace.members]`.
  - Add each extracted crate to `[workspace.dependencies]` with `path = "crates/paladin-<name>"`.
  - Add each extracted crate as an **optional** dependency in the facade `[dependencies]` section.
  - Redefine the facade's feature flags (`web-server`, `notifications`, `content-processing`, `storage-sqlite`, `storage-mysql`) to activate the corresponding new crate rather than raw third-party dependencies. Add a `storage` convenience alias that enables both `storage-sqlite` and `storage-mysql`.
  - Remove from the facade's `[dependencies]` all third-party packages that are now owned exclusively by an extracted crate.

4.6.2 The facade crate's `full` convenience feature must continue to enable all extracted crates (in addition to its existing capabilities).

4.6.3 All public types previously re-exported from the facade crate and originating in an extracted crate must continue to be re-exported from the same facade path (backward-compatible re-exports). No public API paths may be silently removed.

4.6.4 `cargo doc --workspace --no-deps` must produce zero errors and zero warnings.

4.6.5 `cargo test --workspace` must pass with all features enabled (`--all-features`).

---

## 5. Non-Goals (Out of Scope)

- New feature development (new orchestration patterns, new LLM providers, new notification channels).
- Performance optimization beyond what is required to keep existing benchmarks passing.
- Actual publishing to crates.io (this Epic prepares for it; publishing is a release decision covered in Epic 3).
- Extracting infrastructure subsystems that are **not** listed in this Epic (e.g., `infrastructure/adapters/arsenal`, `infrastructure/adapters/citadel`, `infrastructure/adapters/queue`, `infrastructure/resilience`, `infrastructure/security`).
- Changing the public API of any subsystem during extraction — the move is a pure relocation.
- Introducing new feature flags that did not exist in some form prior to this Epic.
- Updating Kubernetes deployment manifests or Docker Compose files (covered in Epic 2).
- Changing `config.yml` structure or the `Settings` public API.

---

## 6. Design Considerations

### 6.1 Crate Dependency Rules

The extracted crates must follow the hexagonal architecture dependency direction:

```
paladin-web          → paladin-ports, paladin-core
paladin-notifications → paladin-ports, paladin-core
paladin-content      → paladin-ports, paladin-core
paladin-storage      → paladin-ports, paladin-core
```

~~No extracted crate may depend on another extracted crate or on the `paladin` facade.~~

**Corrected (dated 2026-08-08, HARD-05):** [ADR-0031](../../../.planning/decisions/0031-extracted-crate-dependency-rule.md) restates the enforceable invariant against the default build: no extracted crate may depend on another extracted crate or on the facade in its default build; a non-default optional feature may declare such an edge only where the facade opts in explicitly and the dependent code is `cfg`-gated (e.g., `paladin-content`'s `llm` feature, gating `paladin-llm` behind an explicit facade opt-in at `Cargo.toml:275`). Checkable per crate via `cargo tree --no-default-features`, which must show no other extracted crate and no facade in the resulting tree.

### 6.2 Recommended Extraction Order

The recommended order is: **storage → notifications → content → web**. This order proceeds from simplest (fewest internal dependencies) to most complex. The developer may reorder based on risk assessment from Task 1.1 results.

| Step | Crate | Rationale for position |
|------|-------|----------------------|
| 1 | `paladin-storage` | Self-contained repository implementations; no use-case service co-location; simpler module graph |
| 2 | `paladin-notifications` | Small (~3 adapter files); isolated from the rest of the content pipeline |
| 3 | `paladin-content` | Largest extraction; 13 use-case services plus 9 adapter files; do last to benefit from lessons learned |
| 4 | `paladin-web` | Depends on `api_content_deliverer` which may reference content types; do after content is extracted |

### 6.3 Temporary Re-Exports During Migration

To avoid a "big bang" migration, each extraction must follow this incremental pattern:

1. Create the new crate and move the source files.
2. In the original location, add a temporary `pub use paladin_<name>::<module>::*;` re-export.
3. Verify `cargo test --workspace` passes.
4. Update all internal consumers to import from the new crate directly.
5. Remove the temporary re-export.

### 6.4 Test Migration Strategy

- **Unit tests** (`#[cfg(test)]` blocks co-located with source): move with the source file into the new crate.
- **Integration tests** (`tests/integration/`) that test a single subsystem in isolation: move into the new crate's `tests/` directory.
- **Integration tests** that compose multiple crates or require Docker services: remain at the workspace-root `tests/integration/` and update import paths to use the new crate.

### 6.5 Versioning

All new crates are initialized at version `0.1.0` to match the current workspace lockstep version. The `[workspace.package]` version field governs all crates uniformly until independent versioning is introduced (out of scope for this Epic).

---

## 7. Technical Considerations

### 7.1 `sqlx` Migration File Path

`sqlx::migrate!()` resolves its path relative to the crate root at compile time. When migrations are moved to `crates/paladin-storage/migrations/`, all `migrate!()` call sites must be updated to the new relative path. Verify with `cargo sqlx migrate info` after the move.

### 7.2 Feature Flag Propagation

The facade crate's feature flags (`web-server`, `notifications`, `content-processing`) currently gate raw third-party crates directly. After extraction, they gate the new workspace crates. The facade's feature names remain unchanged for backward compatibility. Example post-extraction definition:

```toml
# In the facade Cargo.toml [features]:
web-server         = ["dep:paladin-web"]
notifications      = ["dep:paladin-notifications"]
content-processing = ["dep:paladin-content"]
storage-sqlite     = ["dep:paladin-storage", "paladin-storage/sqlite"]
storage-mysql      = ["dep:paladin-storage", "paladin-storage/mysql"]
storage            = ["storage-sqlite", "storage-mysql"]  # convenience alias

# In [dependencies]:
paladin-web           = { workspace = true, optional = true }
paladin-notifications = { workspace = true, optional = true }
paladin-content       = { workspace = true, optional = true }
paladin-storage       = { workspace = true, optional = true }
```

### 7.3 `tensorflow_adapter.rs`

`src/infrastructure/adapters/input/tensorflow_adapter.rs` **stays in the facade crate** for Milestone 7. It is not moved into `paladin-content`.

Rationale: `TensorFlowAdapter` implements `MlPort` from `paladin-ports` and performs ML model inference (model loading, predictions, sentiment analysis, classification). It is an ML adapter, not a content-processing adapter. The `analyze_content` convenience method enriches a `ContentItem` with ML predictions, but that is incidental. Placing it in `paladin-content` would be semantically incorrect — content processing covers fetching, parsing, and transforming content.

The correct long-term home is a future `paladin-ml` crate alongside `NlpPort` and related adapters. However, creating a crate for a single placeholder implementation is premature for Milestone 7.

**Action required before Task 1.4:** Ensure `tensorflow_adapter.rs` is gated behind an `ml` feature flag in the facade crate if it is not already. Document the `paladin-ml` crate as a Milestone 8+ candidate in the backlog.

### 7.4 `ServiceRunner` Conditional Compilation

The `ServiceRunner` composition root assembles all adapters. After extraction, it must use conditional compilation (`#[cfg(feature = "web-server")]`, etc.) to conditionally import from the new crates. Verify that the `ServiceRunner` compiles correctly with each combination of feature flags (no features, each feature individually, all features together).

### 7.5 `sqlx` in `[workspace.dependencies]`

The `sqlx` declaration **stays in `[workspace.dependencies]`** — it is not moved into `crates/paladin-storage/Cargo.toml` exclusively. Both `paladin-memory` (which uses `sqlx` for its `SqliteStore`) and `paladin-storage` reference `sqlx = { workspace = true, optional = true }` and gate it behind their own feature flags. The workspace-level declaration provides the version lock and ensures Cargo resolves a single copy in the dependency graph.

The workspace declaration must be updated to include the `mysql` feature set so `paladin-storage/mysql` can activate it:

```toml
# In [workspace.dependencies]:
sqlx = { version = "0.8", features = ["runtime-tokio-rustls", "sqlite", "mysql", "chrono", "uuid", "json"] }
```

Individual crates still gate which backend actually compiles via their own feature flags — the workspace declaration only makes the feature set *available*.

---

## 8. Success Metrics

1. `cargo build -p paladin-web` succeeds in isolation.
2. `cargo build -p paladin-notifications --no-default-features` succeeds in isolation.
3. `cargo build -p paladin-content --no-default-features` succeeds in isolation.
4. `cargo build -p paladin-storage --features sqlite` succeeds in isolation.
5. `cargo build --workspace --all-features` succeeds.
6. `cargo test --workspace` passes with no regressions.
7. `cargo tree -p paladin-core --all-features` output does not contain `actix-web`, `axum`, `lettre`, `pdf-extract`, `scraper`, or `sqlx`.
8. `cargo tree -p paladin-battalion --all-features` output does not contain `actix-web`, `axum`, `lettre`, `pdf-extract`, `scraper`, or `sqlx`.
9. `cargo doc --workspace --no-deps` produces zero errors and zero warnings.
10. The cost-benefit matrix from Task 1.1 is committed to the repository.

---

## 9. Open Questions

*All pre-implementation questions have been resolved. Decisions are reflected in the relevant sections above.*

| # | Question | Resolution | PRD Section |
|---|----------|------------|-------------|
| 1 | Where does `tensorflow_adapter.rs` go? | Stays in facade; `paladin-ml` crate deferred to Milestone 8+ | §7.3 |
| 2 | `storage` feature flag naming — single or granular? | Two granular flags: `storage-sqlite` and `storage-mysql`; `storage` convenience alias enables both | §4.5.6, §7.2 |
| 3 | Does `paladin-memory` share `sqlx` with `paladin-storage`? | Yes — `sqlx` stays in `[workspace.dependencies]`; both crates reference it with `{ workspace = true, optional = true }` | §7.5 |
| 4 | Does `file_content_repository.rs` go into `paladin-storage`? | No — it is a content delivery adapter, not a SQL repository; stays in facade | §4.5.2 |
| 5 | Where are defer decisions recorded? | In `cost-benefit-assessment.md` (analysis) + Epic tracker (status) + backlog ticket (Milestone 8+ tag) | §4.1.4 |
