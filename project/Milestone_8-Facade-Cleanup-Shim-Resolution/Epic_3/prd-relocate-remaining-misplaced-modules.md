# PRD: Relocate Remaining Misplaced Modules (Milestone 8, Epic 3)

**Project:** Paladin Framework
**Epic:** 3 — Relocate Remaining Misplaced Modules
**Milestone:** 8 — Facade Cleanup, Shim Resolution, and Directory Stabilization
**Version Target:** v0.2.0
**Status:** Ready for Implementation
**Created:** 2026-05-29
**Dependencies:** Epic 1 (Facade Audit) ✅ Complete, Epic 2 (Remove Dead Shims) ✅ Complete

---

## 1. Introduction / Overview

After Epics 1 and 2, the facade crate (`src/`) has had all dead re-export shims deleted and its
public API surface trimmed to confirmed consumers. Three residual concerns remain before the facade
directory structure can be considered stable:

1. **Storage re-export shims** (`src/application/storage/`) — three files that do nothing but
   re-export port traits already living in `paladin_ports`. Six internal consumers still import
   through this extra hop. The indirection is pure noise: the real definitions are in
   `paladin_ports::output::repository_port` and `paladin_ports::output::user_repository_port`.
   Every new contributor must trace through two files to find the actual type definition.

2. **Notification adapter status** (`src/infrastructure/adapters/notifications/`) — Task 3.1
   from the original Epic 3 spec called for moving channel services to `paladin-notifications`.
   This was rendered moot by Epic 2: the notification channel services were already deleted, and
   the infrastructure adapter directory now correctly implements a dual re-export pattern
   (feature-gated crate re-exports + local fallbacks). This task needs to be formally closed as
   already resolved.

3. **Remaining infrastructure adapter disposition** (`src/infrastructure/adapters/`) — A number
   of adapters (Herald formatters, Citadel state adapter, log adapters, TensorFlow ML adapter,
   and several others) have no documented disposition. Milestone 9 cannot safely refactor the
   facade without this record. The TensorFlow adapter in particular has no feature gate despite
   implementing a speculative ML integration that has never been wired into the composition root.

**Goal:** Produce a clean, documented, compile-clean facade by: (a) deleting the storage shim
directory and updating its 6 consumers, (b) formally closing the notification task as resolved,
and (c) writing a disposition record for all remaining infrastructure adapters and applying the
missing `ml` feature gate to `tensorflow_adapter.rs`.

---

## 2. Goals

1. Delete `src/application/storage/` (3 files) and update all 6 internal consumers to import
   directly from `paladin_ports::` — eliminating one level of indirection with zero behavior
   change.
2. Formally document Task 3.1 (notifications) as resolved in the task list and provide a brief
   explanation of why the dual pattern is architecturally correct.
3. Produce an infrastructure adapter disposition record covering every remaining adapter group
   under `src/infrastructure/adapters/`, so Milestone 9 engineers have an explicit go/no-go for
   extraction on each group.
4. Apply an `ml` feature gate to `tensorflow_adapter.rs` to prevent it from compiling (and
   pulling in future heavy ML dependencies) unless explicitly requested.
5. `cargo build --workspace`, `cargo test --workspace`, `cargo clippy --workspace -- -D warnings`,
   and `cargo fmt --all -- --check` all pass clean after every change.

---

## 3. User Stories

- **As a contributor** reading the facade source, I want storage trait imports to lead me directly
  to `paladin_ports::` without passing through a re-export shim, so I can find the canonical
  definition in one step.

- **As a Milestone 9 engineer**, I want a disposition document for every remaining infrastructure
  adapter in the facade, so I know exactly what is in-scope for extraction and what has been
  deliberately kept.

- **As a contributor** adding ML capabilities, I want `tensorflow_adapter.rs` behind a feature
  flag, so it does not silently pull in ML dependencies or compile dead code for users who have no
  ML integration.

- **As an architect** reviewing the facade state, I want every Epic 3 task either completed or
  formally closed with a written rationale, so the task list is an accurate reflection of what
  was done and why.

---

## 4. Functional Requirements

### 4.1 Task 3.1 — Close Notification Task as Resolved

1. The task file must mark Task 3.1 as `[x]` complete with a comment explaining that
   `src/application/notifications/` was removed in Epic 2 and that
   `src/infrastructure/adapters/notifications/mod.rs` already implements the correct dual
   re-export pattern (feature-gated crate re-exports when `notifications` feature is on; local
   fallback modules when off).
2. No files must be moved, added, or deleted as part of this task — it is a documentation-only
   close-out.

### 4.2 Task 3.2 — Delete Storage Re-export Shims

3. The following files must be deleted:
   - `src/application/storage/sql_store.rs`
   - `src/application/storage/user_store.rs`
   - `src/application/storage/mod.rs`
4. The `pub mod storage;` declaration must be removed from `src/application/mod.rs`.
5. The following 6 consumers must be updated to import directly from `paladin_ports::`:

   | File | Old import path | New import path |
   |------|----------------|-----------------|
   | `src/infrastructure/repositories/sqlite_content_repository.rs` | `crate::application::storage::sql_store::{...}` | `paladin_ports::output::repository_port::{...}` |
   | `src/infrastructure/repositories/mysql_content_repository.rs` | `crate::application::storage::sql_store::{...}` | `paladin_ports::output::repository_port::{...}` |
   | `src/infrastructure/repositories/sqlite_user_repository.rs` | `crate::application::storage::user_store::UserRepositoryPort` | `paladin_ports::output::user_repository_port::UserRepositoryPort` |
   | `src/core/platform/manager/user_service.rs` | `crate::application::storage::user_store::UserRepositoryPort` | `paladin_ports::output::user_repository_port::UserRepositoryPort` |
   | `src/config/setup/service_runner.rs` | `crate::application::storage::sql_store::MigrationManager` | `paladin_ports::output::repository_port::MigrationManager` |
   | `tests/repository/mysql_content_repository_test.rs` | `paladin::application::storage::sql_store::ContentRepository` | `paladin_ports::output::repository_port::ContentRepository` |

6. `cargo build --workspace` must exit 0 after the deletion and consumer updates.
7. `cargo test --workspace` must pass with zero failures after the deletion.
8. No `#[deprecated]` re-export must be added — this is an internal-only path with no public API
   contract. Clean break only.

### 4.3 Task 3.3 — Infrastructure Adapter Disposition Record + TensorFlow Feature Gate

9. A disposition record must be written as a Markdown section in the task file (or as a
   separate `infrastructure-adapter-disposition.md` in the Epic 3 folder) covering every adapter
   group under `src/infrastructure/adapters/`. For each group, the record must state:
   - **Decision**: stays in facade / extract to crate X / delete / flag for Milestone 9+
   - **Rationale**: one sentence justifying the decision
   - **Action required**: none / update imports / create crate / add feature gate

10. The disposition decisions confirmed during Epic 3 planning are:

    | Adapter group | Decision | Rationale |
    |--------------|----------|-----------|
    | `arsenal/` | Stays in facade | MCP wiring is facade composition-root responsibility |
    | `citadel/` | Stays in facade | Single file consumed by facade composition root only |
    | `document/` | Stays in facade | Content extraction is facade I/O concern |
    | `file_storage/` | Stays in facade | MinIO wiring belongs to composition root |
    | `garrison/` | Stays in facade | Shim re-exports from `paladin-memory`; no logic |
    | `herald/` | Stays in facade | 1,900 LOC output formatting; tightly coupled to `PaladinExecutionService` |
    | `input/` | Stays in facade (except TensorFlow) | Content fetchers are facade I/O concern |
    | `llm/` | Stays in facade | Config bridge wires LLM config to facade composition |
    | `logs/` | Stays in facade | Log adapters consumed by `ServiceRunner` in composition root |
    | `notifications/` | Stays in facade (dual pattern) | Already resolved in Task 3.1 |
    | `output/` | Stays in facade | Content delivery adapter is facade I/O concern |
    | `paladin_registry.rs` | Stays in facade | Registry is a composition root concern |
    | `queue/` | Stays in facade | Redis wiring belongs to composition root |
    | `sanctum/` | Stays in facade | Vector store wiring is composition root concern |
    | `scheduling/` | Stays in facade | Cron adapter consumed by `ServiceRunner` |
    | `tensorflow_adapter.rs` | Feature-gate `ml` + flag for Milestone 9+ | Speculative ML adapter; never wired in; should not compile by default |

11. `tensorflow_adapter.rs` must be gated behind a new `ml` feature flag:
    - Add `ml = []` to `[features]` in `Cargo.toml` (root workspace manifest, under the
      `paladin-ai` package features).
    - Wrap the `tensorflow_adapter` module declaration in
      `src/infrastructure/adapters/input/mod.rs` with `#[cfg(feature = "ml")]`.
    - Add a doc comment to `tensorflow_adapter.rs` stating it requires `features = ["ml"]` and
      is a placeholder for a future `paladin-ml` crate (Milestone 9+).
    - `cargo build --workspace` (without `--features ml`) must exit 0 after the gate is applied.

12. The disposition record must note for each "Stays in facade" adapter whether it is a candidate
    for extraction in a future milestone, and if so, which milestone.

### 4.4 Quality Gate

13. After all tasks are complete:
    - `cargo build --workspace` exits 0.
    - `cargo test --workspace` passes with zero failures.
    - `cargo clippy --workspace -- -D warnings` reports zero warnings.
    - `cargo fmt --all -- --check` exits 0.
    - `CHANGELOG.md` has a `### Removed` entry documenting the deletion of
      `src/application/storage/` with migration paths.
    - The task file has all tasks marked `[x]`.

---

## 5. Non-Goals (Out of Scope)

- **No crate extractions in this Epic.** Herald, Citadel, and Log adapters stay in the facade.
  Any extraction is deferred to Milestone 9.
- **No changes to `src/infrastructure/adapters/notifications/`.** The dual re-export pattern is
  correct and deliberately preserved.
- **No changes to `paladin-notifications` crate.** The crate already has the adapter
  implementations; this Epic does not touch it.
- **No `use_cases` → `services` rename.** That is Epic 4.
- **No new crates created.** `paladin-herald`, `paladin-ml`, etc. are not in scope.
- **No feature flag changes** other than adding the `ml` flag for `tensorflow_adapter.rs`.
- **No breaking changes to public API** beyond what is documented in `CHANGELOG.md`. The storage
  shim paths were never part of `STABLE_API.md` and are internal-only.

---

## 6. Technical Considerations

### Architecture Boundary Rule
All consumer updates follow the inward-dependency rule: infrastructure repositories may import
from `paladin_ports::` (an output-layer crate) without violating hexagonal boundaries.
`paladin_ports` is already a direct dependency of the facade workspace, so no `Cargo.toml`
changes are needed for the consumer updates.

### Feature Flag Convention
The `ml` feature flag follows the existing pattern established by `redis-queue`, `s3-storage`,
`notifications`, etc. in the root `Cargo.toml`. It is opt-in and disabled by default.

### Consumer Update Strategy
All 6 storage shim consumers use mechanical import path replacements. No logic changes are
required. The types themselves do not change — only the import path changes.

### Commit Strategy (per project convention)
Each completed parent task gets its own commit after passing `cargo test`, `cargo fmt --check`,
and `cargo clippy`. Commit messages use conventional commit format with `-m` flags. No `!`
characters in commit messages (bash history expansion guard).

### Reference Files
- `paladin_ports::output::repository_port` — canonical home of `SqlStore`, `ContentRepository`,
  `ContentListRepository`, `MigrationManager`, `RepositoryError`, `RepositoryStats`,
  `TransactionManager`
- `paladin_ports::output::user_repository_port` — canonical home of `UserRepositoryPort`
- `src/infrastructure/adapters/notifications/mod.rs` — dual re-export pattern reference
- `project/Milestone_8-Facade-Cleanup-Shim-Resolution/Epic_2/email_notifications_review.rs`
  — staging artifact from Epic 1/2 for the email application service (Epic 3 does not touch this)

---

## 7. Success Metrics

1. `src/application/storage/` directory no longer exists after Epic 3 completes.
2. All 6 former storage shim consumers compile cleanly using `paladin_ports::` import paths.
3. `cargo build --workspace` without `--features ml` does not compile `tensorflow_adapter.rs`.
4. Every adapter group under `src/infrastructure/adapters/` has a written disposition decision
   in the Epic 3 record.
5. `cargo test --workspace` passes with zero failures and zero new ignored tests.
6. `CHANGELOG.md` documents the `src/application/storage/` removal with replacement paths.
7. The task file shows all sub-tasks and parent tasks as `[x]` complete.

---

## 8. Open Questions

1. **`garrison/mod.rs` in infra adapters** — This appears to be a re-export shim for
   `paladin-memory`. Should it be audited for zero-consumer status and deleted (like the storage
   shims), or is it outside this Epic's scope? Recommend auditing during Task 3.3 and noting in
   the disposition record whether it qualifies for deletion as a follow-up in Epic 4 or a
   separate PR.

2. **`sanctum/mod.rs` in infra adapters** — Similar question to `garrison/mod.rs`. Likely a
   re-export shim. The disposition record should note whether it is a deletion candidate.

3. **`output/api_content_deliverer.rs`** — 629-line output adapter. Should it be evaluated for
   extraction to `paladin-content` or `paladin-web`? The disposition record should flag this
   explicitly for Milestone 9 review.
