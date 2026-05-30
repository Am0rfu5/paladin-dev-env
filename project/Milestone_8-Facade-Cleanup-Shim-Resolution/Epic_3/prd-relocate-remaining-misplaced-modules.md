# PRD: Relocate Remaining Misplaced Modules (Milestone 8, Epic 3)

**Project:** Paladin Framework
**Epic:** 3 — Relocate Remaining Misplaced Modules
**Milestone:** 8 — Facade Cleanup, Shim Resolution, and Directory Stabilization
**Version Target:** v0.2.0
**Status:** Ready for Implementation
**Created:** 2026-05-29
**Last Updated:** 2026-05-29
**Document Version:** 1.1 (open questions resolved — see §8)
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

> **Scope clarification (v1.1):** Epic 3 performs **no crate extractions.** Epic 1's `facade-audit.md`
> List B identified 13 files as candidates to move into leaf crates. Epic 3 **defers every one of
> those moves to Milestone 9** and instead records, per adapter group, whether the group is a
> confirmed M9 extraction candidate. The disposition record in §4.3 is the authoritative
> reconciliation of List B against the "stays for now" reality — where this PRD's disposition and
> List B appear to disagree, the disposition record is recording a deliberate deferral, not a
> reversal of the audit.

---

## 2. Goals

1. Delete `src/application/storage/` (3 files) and update all 6 internal consumers to import
   directly from `paladin_ports::` — eliminating one level of indirection with zero behavior
   change.
2. Formally document Task 3.1 (notifications) as resolved in the task list and provide a brief
   explanation of why the dual pattern is architecturally correct.
3. Produce an infrastructure adapter disposition record covering every remaining adapter group
   under `src/infrastructure/adapters/`, so Milestone 9 engineers have an explicit go/no-go for
   extraction on each group — including an explicit M9 extraction flag for each List B item that
   is being deferred.
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
  deliberately kept — and for anything being kept "for now," I want to know whether the audit
  already earmarked it for extraction.

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
   - **M9 extraction candidate?**: yes (target crate) / no

10. The disposition decisions confirmed during Epic 3 planning are below. The **M9 candidate**
    column reconciles each group against Epic 1 `facade-audit.md` List B. "Stays in facade" means
    *stays for Epic 3*; it does not override a List B extraction recommendation — it defers it.

    | Adapter group | Decision (Epic 3) | Rationale | M9 extraction candidate? |
    |--------------|-------------------|-----------|--------------------------|
    | `arsenal/` | Stays in facade | MCP wiring is facade composition-root responsibility | No |
    | `citadel/` | Stays in facade | Single file consumed by facade composition root only | **Yes → `paladin-memory`** (`file_citadel.rs`, 581 LOC, List B) |
    | `document/` | Stays in facade | `document/mod.rs` is an active 3-consumer re-export of `paladin_content::adapters::document::*` | **Yes → `paladin-content`** (`document_adapter.rs` 480 LOC, `pdf_extractor.rs` 350 LOC, List B duplicates) |
    | `file_storage/` | Stays in facade | MinIO wiring belongs to composition root | **Yes → `paladin-storage`** (`minio.rs`, 1,198 LOC, List B) |
    | `garrison/` | Stays in facade | 25-LOC re-export of `paladin_memory::garrison::*` + compat sub-modules; many active consumers incl. examples | Optional consolidation → fold into M9 `paladin-memory` work (see §8 Q1) |
    | `herald/` | Stays in facade | ~1,900 LOC output formatting; tightly coupled to `PaladinExecutionService` | No (no target crate exists) |
    | `input/` | Stays in facade (except TensorFlow) | Content fetchers are facade I/O concern | No (fetchers); TensorFlow handled separately below |
    | `llm/` | Stays in facade | Config bridge wires LLM config to facade composition | No |
    | `logs/` | Stays in facade | Log adapters consumed by `ServiceRunner` in composition root | No (no target crate exists) |
    | `notifications/` | Stays in facade (dual pattern) | Already resolved in Task 3.1; `email`/`system` adapters already live in `paladin-notifications` | Adapters already extracted; facade keeps dual-pattern bridge |
    | `output/` | Stays in facade | `output/mod.rs` already implements feature-gated dual pattern; `api_content_deliverer.rs` duplicates the `paladin-web` copy | **Yes → `paladin-web`** (`api_content_deliverer.rs`, **724 LOC**, List B — see §8 Q3) |
    | `paladin_registry.rs` | Stays in facade | Registry is a composition root concern | No |
    | `queue/` | Stays in facade | Redis wiring belongs to composition root | **Yes → `paladin-storage`** (`redis.rs`, 1,570 LOC, List B) |
    | `sanctum/` | Stays in facade | 9-LOC re-export of `paladin_memory::sanctum::*`; 6+ active test/example consumers | Optional consolidation → fold into M9 `paladin-memory` work (see §8 Q2) |
    | `scheduling/` | Stays in facade | Cron adapter consumed by `ServiceRunner` | No |
    | `tensorflow_adapter.rs` | Feature-gate `ml` + flag for Milestone 9+ | Speculative ML adapter (629 LOC); never wired in; should not compile by default | **Yes → future `paladin-ml`** (Milestone 9+) |

11. `tensorflow_adapter.rs` must be gated behind a new `ml` feature flag:
    - Add `ml = []` to `[features]` in `Cargo.toml` (root workspace manifest, under the
      `paladin-ai` package features).
    - Wrap the `tensorflow_adapter` module declaration in
      `src/infrastructure/adapters/input/mod.rs` with `#[cfg(feature = "ml")]`.
    - Add a doc comment to `tensorflow_adapter.rs` stating it requires `features = ["ml"]` and
      is a placeholder for a future `paladin-ml` crate (Milestone 9+).
    - `cargo build --workspace` (without `--features ml`) must exit 0 after the gate is applied.

12. The disposition record must note for each group flagged as an M9 extraction candidate the
    target crate and the originating List B entry, so Milestone 9 has a direct cross-reference back
    to `facade-audit.md`.

13. **Garrison and Sanctum bridge shims must be explicitly documented (not deleted).** The record
    must capture, for each of `src/infrastructure/adapters/garrison/mod.rs` and
    `src/infrastructure/adapters/sanctum/mod.rs`:
    - That Epic 1 (Appendix B, Section 1) already audited them as **active multi-consumer**
      re-export bridges (garrison: `cli/config/loader.rs`, `infrastructure/mod.rs`, 4+ integration
      tests, 3 examples; sanctum: 3 integration tests + 3 examples), so they do **not** qualify for
      zero-consumer deletion.
    - That they **stay** in Epic 3 and are recorded as optional indirection-reduction candidates to
      fold into the Milestone 9 `paladin-memory` extraction (explicitly **not** Epic 4, which is the
      unrelated `use_cases → services` rename).

### 4.4 Quality Gate

14. After all tasks are complete:
    - `cargo build --workspace` exits 0.
    - `cargo test --workspace` passes with zero failures.
    - `cargo clippy --workspace -- -D warnings` reports zero warnings.
    - `cargo fmt --all -- --check` exits 0.
    - `CHANGELOG.md` has a `### Removed` entry documenting the deletion of
      `src/application/storage/` with migration paths.
    - The task file has all tasks marked `[x]`.

---

## 5. Non-Goals (Out of Scope)

- **No crate extractions in this Epic.** Herald, Citadel, Log, MinIO, Redis, the content
  adapters, and the API content deliverer all stay in the facade. Every List B move is deferred to
  Milestone 9 and recorded as such in §4.3.
- **No deletion of `garrison/mod.rs` or `sanctum/mod.rs`.** Both are active multi-consumer bridges;
  any consolidation is deferred (see §8 Q1, Q2).
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

### Why Storage Shims Are Deleted but Garrison/Sanctum Shims Are Not
The storage shims qualified for elimination on three counts that the garrison/sanctum shims fail:
(a) they were internal-only paths never published in `STABLE_API.md`; (b) they targeted a single
canonical port location, making the consumer fix a mechanical 1:1 path swap; and (c) all 6
consumers were internal `src/` and test files. Garrison and Sanctum re-export from
`paladin_memory` *and* serve public-facing `examples/`, and garrison additionally exposes
backward-compatible sub-module paths. Collapsing them would be a higher-risk, public-surface
change inconsistent with this Epic's "minimal churn / no extraction" mandate, so they are kept and
recorded for later consolidation.

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
- `src/infrastructure/adapters/output/mod.rs` — feature-gated dual re-export reference for the
  `api_content_deliverer` M9 extraction
- `project/Milestone_8-Facade-Cleanup-Shim-Resolution/Epic_1/facade-audit.md` — List B (move
  candidates) and Appendix B Section 1 (shim consumer matrix), the authoritative cross-reference
  for the §4.3 M9 flags

---

## 7. Success Metrics

1. `src/application/storage/` directory no longer exists after Epic 3 completes.
2. All 6 former storage shim consumers compile cleanly using `paladin_ports::` import paths.
3. `cargo build --workspace` without `--features ml` does not compile `tensorflow_adapter.rs`.
4. Every adapter group under `src/infrastructure/adapters/` has a written disposition decision
   in the Epic 3 record, including an explicit M9 extraction flag where applicable.
5. `garrison/mod.rs` and `sanctum/mod.rs` each have a written "stays — active bridge" disposition
   with consumer evidence and an optional-consolidation note.
6. `cargo test --workspace` passes with zero failures and zero new ignored tests.
7. `CHANGELOG.md` documents the `src/application/storage/` removal with replacement paths.
8. The task file shows all sub-tasks and parent tasks as `[x]` complete.

---

## 8. Resolved Decisions (formerly Open Questions)

> The three open questions in v1.0 are resolved here. Each decision is reflected in the §4.3
> disposition table and the §2 goals.

1. **`garrison/mod.rs` in infra adapters — RESOLVED: Stays; not a deletion candidate.**
   Epic 1 already audited this file (Appendix B, Section 1) as an active re-export of
   `paladin_memory::garrison::*` plus three backward-compatible sub-module paths
   (`in_memory_garrison`, `sqlite_garrison`, `token_counter`), with **many consumers**:
   `cli/config/loader.rs`, `infrastructure/mod.rs`, 4+ integration tests, and 3 examples. Because
   it is not zero-consumer, the deletion rationale used for the storage shims does not apply.
   It **stays** in Epic 3 and is recorded in §4.3 as an *optional* indirection-reduction candidate
   to fold into the Milestone 9 `paladin-memory` extraction — **not** Epic 4, which is the
   unrelated `use_cases → services` rename.

2. **`sanctum/mod.rs` in infra adapters — RESOLVED: Stays; not a deletion candidate.**
   Same reasoning as Q1. It is a 9-LOC re-export of `paladin_memory::sanctum::InMemorySanctum`
   (plus the feature-gated `QdrantSanctumAdapter`) with 6+ active consumers (integration tests
   `rag_integration_tests.rs`, `in_memory_sanctum_tests.rs`, `qdrant_sanctum_tests.rs`; examples
   `paladin_with_sanctum.rs`, `sanctum_basic_inmemory.rs`, `sanctum_configuration.rs`). It is even
   simpler than garrison (no compat sub-modules), so there is no benefit to touching it now. It
   **stays**, with the same optional-consolidation note for the M9 `paladin-memory` work.

3. **`output/api_content_deliverer.rs` — RESOLVED: Stays in Epic 3; explicitly flagged as a
   Milestone 9 extraction candidate to `paladin-web`.**
   Two corrections from v1.0: (a) the file is **724 LOC**, not 629 — the 629 figure belongs to
   `tensorflow_adapter.rs`; (b) this is a genuine duplicate-extraction candidate, since
   `paladin-web` already holds a counterpart and `output/mod.rs` already implements the
   feature-gated dual re-export pattern (`paladin_web::adapters::api_content_deliverer` when the
   `web-server` feature is active; 1 consumer, `scheduler_integration_test.rs`). Epic 1 List B
   marked it **move → paladin-web** with a "wait for Epic 3 co-ordination" caveat. Per this Epic's
   no-extraction mandate, it **stays for now** but is recorded in §4.3 as a confirmed M9 extraction
   target, with the existing dual pattern noted as groundwork already laid for that extraction.
