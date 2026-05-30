## Relevant Files

- `src/application/mod.rs` — Remove `pub mod storage;` declaration (Task 2.0)
- `src/application/storage/mod.rs` — Delete (Task 2.0)
- `src/application/storage/sql_store.rs` — Delete (Task 2.0)
- `src/application/storage/user_store.rs` — Delete (Task 2.0)
- `src/infrastructure/repositories/sqlite_content_repository.rs` — Update storage import (Task 2.0)
- `src/infrastructure/repositories/mysql_content_repository.rs` — Update storage import (Task 2.0)
- `src/infrastructure/repositories/sqlite_user_repository.rs` — Update storage import (Task 2.0)
- `src/core/platform/manager/user_service.rs` — Update storage import (Task 2.0)
- `src/config/setup/service_runner.rs` — Update storage import (Task 2.0)
- `tests/repository/mysql_content_repository_test.rs` — Update storage import (Task 2.0)
- `Cargo.toml` — Add `ml = []` feature flag (Task 3.0)
- `src/infrastructure/adapters/input/mod.rs` — Gate `tensorflow_adapter` behind `#[cfg(feature = "ml")]` (Task 3.0)
- `src/infrastructure/adapters/input/tensorflow_adapter.rs` — Add feature doc comment (Task 3.0)
- `CHANGELOG.md` — Document `src/application/storage/` removal with migration paths (Task 5.0)
- `project/Milestone_8-Facade-Cleanup-Shim-Resolution/Epic_3/infrastructure-adapter-disposition.md` — New disposition record (Task 4.0)

### Notes

- Run `cargo test --workspace` after each parent task before committing.
- Run `cargo fmt --check` and `cargo clippy --workspace -- -D warnings` before each commit.
- Commit messages must use conventional commit format with `-m` flags. No `!` characters in commit messages (bash history expansion guard).
- See `prd-relocate-remaining-misplaced-modules.md` §4.2 for the exact consumer import table.
- See `prd-relocate-remaining-misplaced-modules.md` §4.3 for the full disposition table.
- The `paladin_ports` crate is already a dependency — no `Cargo.toml` changes needed for consumer updates.

## Instructions for Completing Tasks

**IMPORTANT:** As you complete each task, you must check it off in this markdown file by changing `- [ ]` to `- [x]`. This helps track progress and ensures you don't skip any steps.

Example:

- `- [ ] 1.1 Read file` → `- [x] 1.1 Read file` (after completing)

Update the file after completing each sub-task, not just after completing an entire parent task.

## Tasks

- [x] 0.0 Create feature branch
  - [x] 0.1 Create and checkout a new branch: `git checkout -b feature/milestone_8-epic_3-relocate-misplaced-modules`
  - [x] 0.2 Confirm branch is active: `git branch --show-current`

- [x] 1.0 Close Task 3.1 — Notifications (documentation-only)
  <!-- RESOLVED: `src/application/notifications/` was deleted in Epic 2. The infrastructure adapter at
       `src/infrastructure/adapters/notifications/mod.rs` correctly implements a dual re-export pattern:
       `paladin-notifications` crate re-exports when `notifications` feature is enabled; local fallback
       adapter modules otherwise. No file moves required. This is architecturally correct and deliberately
       preserved (see PRD §4.1). -->
  - [x] 1.1 Read `src/infrastructure/adapters/notifications/mod.rs` and confirm the dual re-export pattern is intact (feature-gated crate re-exports when `notifications` feature is on; local fallback modules when off).
  - [x] 1.2 Mark this task `[x]` with the inline comment: "Resolved — `src/application/notifications/` was deleted in Epic 2. The infrastructure adapter at `src/infrastructure/adapters/notifications/mod.rs` correctly implements a dual re-export pattern: `paladin-notifications` crate when `notifications` feature is enabled, local fallback adapters otherwise. No file moves required."
  - [x] 1.3 Commit the task file update: `git add project/ && git commit -m "docs(m8-e3): close Task 3.1 notifications as already resolved"`

- [ ] 2.0 Delete storage re-export shims and update all 6 consumers
  - [ ] 2.1 Read the current contents of all 3 storage shim files (`sql_store.rs`, `user_store.rs`, `mod.rs`) and all 6 consumer files to understand the exact import lines that need updating before making any changes.
  - [ ] 2.2 Update `src/infrastructure/repositories/sqlite_content_repository.rs`: replace `use crate::application::storage::sql_store::{...}` with `use paladin_ports::output::repository_port::{...}` (keep the exact same imported names).
  - [ ] 2.3 Update `src/infrastructure/repositories/mysql_content_repository.rs`: same replacement as 2.2.
  - [ ] 2.4 Update `src/infrastructure/repositories/sqlite_user_repository.rs`: replace `use crate::application::storage::user_store::UserRepositoryPort` with `use paladin_ports::output::user_repository_port::UserRepositoryPort`.
  - [ ] 2.5 Update `src/core/platform/manager/user_service.rs`: replace `use crate::application::storage::user_store::UserRepositoryPort` with `use paladin_ports::output::user_repository_port::UserRepositoryPort`.
  - [ ] 2.6 Update `src/config/setup/service_runner.rs`: replace `use crate::application::storage::sql_store::MigrationManager` with `use paladin_ports::output::repository_port::MigrationManager`.
  - [ ] 2.7 Update `tests/repository/mysql_content_repository_test.rs`: replace `use paladin::application::storage::sql_store::ContentRepository` with `use paladin_ports::output::repository_port::ContentRepository`.
  - [ ] 2.8 Remove `pub mod storage;` from `src/application/mod.rs` (line 143).
  - [ ] 2.9 Delete the three storage shim files: `rm src/application/storage/sql_store.rs src/application/storage/user_store.rs src/application/storage/mod.rs && rmdir src/application/storage/`
  - [ ] 2.10 Run `cargo build --workspace` — confirm exit 0 with zero errors.
  - [ ] 2.11 Run `cargo test --workspace` — confirm all tests pass, zero failures.
  - [ ] 2.12 Run `cargo fmt --all -- --check` and `cargo clippy --workspace -- -D warnings` — confirm both pass clean.
  - [ ] 2.13 Add a `### Removed` entry to `CHANGELOG.md` under `[Unreleased]` documenting the deletion of `src/application/storage/` with the migration table:
    - `paladin::application::storage::sql_store::*` → `paladin_ports::output::repository_port::*`
    - `paladin::application::storage::user_store::UserRepositoryPort` → `paladin_ports::output::user_repository_port::UserRepositoryPort`
  - [ ] 2.14 Commit: `git add -A && git commit -m "refactor(m8-e3): delete storage re-export shims, update 6 consumers to paladin_ports" -m "- Deleted src/application/storage/ (sql_store.rs, user_store.rs, mod.rs)" -m "- Updated 6 consumers to import directly from paladin_ports::output::repository_port and user_repository_port" -m "- Removed pub mod storage from application/mod.rs" -m "- Updated CHANGELOG.md with migration paths"`

- [ ] 3.0 Apply `ml` feature gate to `tensorflow_adapter.rs`
  - [ ] 3.1 Read `src/infrastructure/adapters/input/mod.rs` (line 11) and `src/infrastructure/adapters/input/tensorflow_adapter.rs` (first 20 lines) to understand the current declaration and any existing cfg attributes.
  - [ ] 3.2 Add `ml = []` to the `[features]` section of `Cargo.toml`, grouped with the other subsystem feature flags (after the `vision = []` line). Do not add it to the `full` convenience flag — ML is an explicit opt-in.
  - [ ] 3.3 In `src/infrastructure/adapters/input/mod.rs`, wrap `pub mod tensorflow_adapter;` (line 11) with `#[cfg(feature = "ml")]`.
  - [ ] 3.4 Add a module-level doc comment to `src/infrastructure/adapters/input/tensorflow_adapter.rs` stating: "Requires the `ml` feature flag. Placeholder for a future `paladin-ml` crate (Milestone 9+). Enable with `paladin = { features = [\"ml\"] }`."
  - [ ] 3.5 Run `cargo build --workspace` (without `--features ml`) — confirm `tensorflow_adapter.rs` is not compiled and exit is 0.
  - [ ] 3.6 Run `cargo build --workspace --features ml` — confirm the adapter compiles cleanly with the flag on.
  - [ ] 3.7 Run `cargo test --workspace` — confirm all tests pass.
  - [ ] 3.8 Run `cargo fmt --all -- --check` and `cargo clippy --workspace -- -D warnings` — confirm both pass clean.
  - [ ] 3.9 Commit: `git add -A && git commit -m "refactor(m8-e3): gate tensorflow_adapter.rs behind ml feature flag" -m "- Added ml = [] to Cargo.toml features (opt-in, not in full)" -m "- Wrapped pub mod tensorflow_adapter in cfg(feature = ml) in input/mod.rs" -m "- Added doc comment flagging it as Milestone 9+ paladin-ml placeholder"`

- [ ] 4.0 Write infrastructure adapter disposition record
  - [ ] 4.1 Create `project/Milestone_8-Facade-Cleanup-Shim-Resolution/Epic_3/infrastructure-adapter-disposition.md` with a header, date, and a table covering all 16 adapter groups from PRD §4.3. For each group include: adapter path, epic-3 decision (stays/feature-gated/etc.), one-sentence rationale, M9 extraction candidate (yes with target crate / no), and any action taken in Epic 3.
  - [ ] 4.2 Verify the garrison and sanctum entries explicitly include: consumer evidence (mention examples/ and tests/ consumers), "stays — active bridge" decision, and optional-consolidation note for M9 `paladin-memory` work.
  - [ ] 4.3 Verify the `output/api_content_deliverer.rs` entry explicitly flags it as an M9 extraction candidate to `paladin-web` and notes the dual-pattern groundwork already in `output/mod.rs`.
  - [ ] 4.4 Verify the TensorFlow entry reflects the feature gate applied in Task 3.0 and the Milestone 9+ `paladin-ml` target.
  - [ ] 4.5 Verify the List B cross-references are present for: `citadel/file_citadel.rs` → `paladin-memory`; `file_storage/minio.rs` → `paladin-storage`; `queue/redis.rs` → `paladin-storage`; `document/` → `paladin-content`.
  - [ ] 4.6 Commit: `git add project/ && git commit -m "docs(m8-e3): add infrastructure adapter disposition record" -m "- Covers all 16 adapter groups under src/infrastructure/adapters/" -m "- Flags M9 extraction candidates with target crates and List B cross-references" -m "- Documents garrison/sanctum bridge shim rationale and optional M9 consolidation"`

- [ ] 5.0 Run full quality gate and final commit
  - [ ] 5.1 `cargo build --workspace` — exit code 0, zero errors
  - [ ] 5.2 `cargo test --workspace` — all tests pass, zero failures
  - [ ] 5.3 `cargo clippy --workspace -- -D warnings` — zero warnings
  - [ ] 5.4 `cargo fmt --all -- --check` — exit code 0 (no formatting drift)
  - [ ] 5.5 `find src/ -name "*.rs" | wc -l` — confirm file count decreased by 3 from Epic 2 baseline (163 → 160)
  - [ ] 5.6 Mark all tasks `[x]` in this file (all parent tasks and sub-tasks)
  - [ ] 5.7 Commit: `git add project/ && git commit -m "docs(m8-e3): mark all Epic 3 tasks complete" -m "- Tasks 0.0-5.0 complete" -m "- Storage shims deleted, 6 consumers updated, ml feature gate applied" -m "- Disposition record written for all 16 infra adapter groups" -m "- Quality gate passed: build, test, clippy, fmt, file count 160"`
