## Epic 3: Relocate Remaining Misplaced Modules

**Priority:** High
**Estimated Effort:** Medium
**Dependencies:** Epic 1

### Objective

Move files that the audit identifies as belonging in extracted crates to their correct locations.

### Tasks

#### Task 3.1: Move Notification Channel Services to `paladin-notifications`

**Description:** `src/application/notifications/email_notifications.rs`, `push_notifications.rs`, and `system_notifications.rs` are channel-specific application services that belong with the notification adapters in `paladin-notifications`. Move them and update imports.

**Deliverables:**
- Three files moved to `crates/paladin-notifications/src/`.
- Facade re-exports added if needed for backward compatibility.
- `cargo test -p paladin-notifications` passes.

#### Task 3.2: Resolve `src/application/storage/` Modules

**Description:** `src/application/storage/` contains `sql_store.rs`, `file_store.rs`, `user_store.rs`. Determine whether these define port traits (→ move to `paladin-ports`) or contain implementations (→ move to `paladin-storage`). If they define repository traits, they are ports. If they contain `SqliteStore` or `MigrationManager` logic, they are implementations.

**Deliverables:**
- Each file moved to its correct crate or confirmed as staying.
- Imports updated.
- Tests passing.

#### Task 3.3: Evaluate Remaining Infrastructure Adapters

**Description:** `src/infrastructure/` may still contain adapters not extracted in Milestone 7 (e.g., `tensorflow_adapter.rs`, citadel adapter, MCP adapters, log adapters). For each, decide: move to an existing crate, gate behind a feature flag and leave in facade, or flag for Milestone 9+.

**Deliverables:**
- Disposition documented for each remaining adapter.
- Extractions completed where warranted.
- Feature flags added where needed.

---
