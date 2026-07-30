## Epic 1: Extended Workspace Decomposition — Remaining Infrastructure Crates

**Epic Owner:** TBD
**Priority:** High
**Estimated Effort:** Large
**Dependencies:** Milestone 3 complete

### Objective

Complete the full workspace decomposition by extracting the four remaining infrastructure subsystems into their own optional crates. After this Epic, the `paladin` facade crate contains only re-exports, the application-layer use-case services not covered by dedicated crates, and the `ServiceRunner` composition root. Every substantial subsystem is independently compilable and testable.

### Background & Rationale

After Milestone 2, the workspace contains six crates. The remaining infrastructure code in the facade crate includes:

- **Web server** (`infrastructure/web/`, `infrastructure/adapters/output/api_content_deliverer.rs`) — Actix-web REST API, WebSocket handlers, middleware, content delivery endpoints. Gated behind the `web-server` feature flag since Milestone 1 but still compiled as part of the facade crate. Approximately 3–4k LOC plus the `actix-web` dependency tree.

- **Notification adapters** (`infrastructure/adapters/notifications/`) — Email (`lettre`), push, and system notification adapters implementing `NotificationDeliveryPort` and `NotificationTemplatePort`. The notification service coordinator was relocated to the application layer in Milestone 3; the concrete adapters remain in infrastructure. Approximately 2k LOC.

- **Content processing** (`infrastructure/adapters/document/`, `infrastructure/adapters/input/`, `application/use_cases/content/`) — PDF extraction (`pdf-extract`), web scraping (`scraper`), RSS ingestion (`rss`), HTTP content fetching (`reqwest` blocking client), content aggregation, filtering, summarization, and analysis services. Approximately 5–6k LOC with heavy external dependencies.

- **SQL repositories** (`infrastructure/repositories/`) — SQLite and MySQL repository implementations using `sqlx`, migration management, and the `SqliteStore` abstraction. Approximately 3k LOC.

Each of these is cleanly isolated behind feature flags and port traits. Extraction to separate crates makes the dependency cost explicit and opt-in at the Cargo dependency level rather than just the feature-flag level.

### Acceptance Criteria

1. Four new crates exist: `paladin-web`, `paladin-notifications`, `paladin-content`, `paladin-storage`.
2. Each crate depends only on `paladin-core` and `paladin-ports` (and optionally on shared workspace dependencies).
3. The facade crate's direct source code is reduced to re-exports, the `ServiceRunner` composition root, and application-layer use-case services that don't warrant their own crate.
4. `cargo build --workspace` succeeds.
5. `cargo test --workspace` passes all tests.
6. A downstream consumer can depend on `paladin-core` + `paladin-ports` + `paladin-battalion` + `paladin-llm` without transitively pulling in `actix-web`, `lettre`, `pdf-extract`, `scraper`, `sqlx`, or any other infrastructure dependency from the extracted crates.

### Tasks

#### Task 1.1: Cost-Benefit Assessment per Extraction

**Description:** Before extracting, assess each candidate crate for extraction value vs. effort. Criteria: (a) dependency weight introduced by the subsystem, (b) frequency of independent change, (c) likelihood of downstream consumers wanting it without the rest, (d) extraction complexity. Any subsystem that scores low may be deferred with justification.

**Deliverables:**
- Cost-benefit matrix for all four candidate extractions.
- Go/defer decision for each.
- Updated Epic scope if any extractions are deferred.

**Estimated Effort:** Small

#### Task 1.2: Extract `paladin-web` Crate

**Description:** Move the web server infrastructure — Actix-web application factory, REST API route handlers, WebSocket handlers, middleware, and `ApiContentDeliverer` — into `crates/paladin-web/`. The crate depends on `paladin-core`, `paladin-ports`, and `actix-web`.

**Deliverables:**
- `crates/paladin-web/Cargo.toml` with `actix-web` as a primary dependency.
- All web-related modules relocated.
- `ServiceRunner` updated to conditionally depend on `paladin-web`.
- Web-related integration tests migrated.
- `cargo build -p paladin-web` succeeds in isolation.

**Estimated Effort:** Medium

#### Task 1.3: Extract `paladin-notifications` Crate

**Description:** Move the notification adapter implementations — `EmailNotificationAdapter` (with `lettre`), `SystemNotificationAdapter`, and `PushNotificationAdapter` — into `crates/paladin-notifications/`. Feature flags: `email` (gates `lettre` + `handlebars`), `push`, `system`.

**Deliverables:**
- `crates/paladin-notifications/Cargo.toml` with per-channel feature flags.
- Adapter modules relocated.
- Notification integration tests migrated.
- `cargo build -p paladin-notifications --no-default-features` succeeds.

**Estimated Effort:** Medium

#### Task 1.4: Extract `paladin-content` Crate

**Description:** Move the content processing pipeline — `PdfExtractor`, `HttpContentFetcher`, `FileContentListFetcher`, `NewsApiFetcher`, RSS adapter, web scraper, content aggregation/filtering/summarization/analysis use-case services — into `crates/paladin-content/`. Feature flags: `pdf` (gates `pdf-extract`), `web-scraping` (gates `scraper`), `rss` (gates `rss` crate).

**Deliverables:**
- `crates/paladin-content/Cargo.toml` with per-capability feature flags.
- Content processing modules and use-case services relocated.
- Content-related tests migrated.
- `cargo build -p paladin-content --no-default-features` succeeds.

**Estimated Effort:** Large

#### Task 1.5: Extract `paladin-storage` Crate

**Description:** Move SQL repository implementations — `SqliteStore`, `SqliteUserRepository`, `SqliteContentRepository`, MySQL variants, and migration management — into `crates/paladin-storage/`. Feature flags: `sqlite` (gates `sqlx/sqlite`), `mysql` (gates `sqlx/mysql`).

**Deliverables:**
- `crates/paladin-storage/Cargo.toml` with database backend feature flags.
- Repository modules and migration files relocated.
- Database integration tests migrated.
- `cargo build -p paladin-storage --features sqlite` succeeds.

**Estimated Effort:** Medium

#### Task 1.6: Update Facade Crate and Workspace Metadata

**Description:** Update the `paladin` facade crate to depend on all new crates with appropriate default features. Verify backward-compatible re-exports. Update `[workspace]` members list and `[workspace.dependencies]`.

**Deliverables:**
- Facade crate updated with new dependencies.
- All re-export paths verified.
- `cargo test --workspace` passes.
- `cargo doc --workspace --no-deps` clean.

**Estimated Effort:** Medium

---
