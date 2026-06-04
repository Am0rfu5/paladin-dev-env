# Facade Cleanup — Reconciliation & Refreshed Plan

**Date:** 2026-06-04
**Author:** fresh file-by-file audit of `src/` (facade crate `paladin-ai` / lib `paladin`)
**Supersedes (corrects):** `Epic_1/facade-audit.md` and `Epic_3/infrastructure-adapter-disposition.md`
**Status:** Proposed

---

## 1. Why this document exists

Milestone 8 ("Facade Cleanup & Shim Resolution") was created 2026-05-29 and **partially
executed**. A fresh, independent file-by-file audit of the current `src/` tree (159 files)
shows that:

1. Several Epics already ran — the tree no longer matches the original audit.
2. The one Epic that mattered most for "removing slop" (Epic 3, relocations) was **punted** —
   its disposition record marked every misplaced adapter "Stays / defer to Milestone 9+".
3. The original Epic 1 audit and the Epic 3 disposition record contain **factual errors**:
   they describe ~4,400 LOC of *orphaned, uncompiled duplicate files* as "active bridges that
   stay." They are not bridges; they are dead corpses left behind when the real code was copied
   into leaf crates. They can be deleted outright.

This is the "bit of slop" being sensed. It is concentrated and mostly trivial to remove.

---

## 2. Reconciled status of Milestone 8

| Epic | Title | Recorded status | **Actual status (verified 2026-06-04)** |
|------|-------|-----------------|------------------------------------------|
| 1 | Facade Crate Audit | Complete | Done, **but with errors** (see §3) — needs correction |
| 2 | Remove Dead Shims / Empty Modules | — | **DONE** ✅ — all 25 "List A" files are gone (`notifications/`, `storage/` stubs, `subject/`, `admin/`, `user/` dirs confirmed deleted) |
| 3 | Relocate Misplaced Modules | — | **PUNTED** ❌ — disposition kept everything in facade, deferred to "M9". Only tensorflow gating + 3 storage-shim deletions were done. This is the remaining work. |
| 4 | `use_cases` → `services` Rename | — | **DONE** ✅ — tree now has `src/application/services/` |
| 5 | Document Facade Role | — | Largely done — `lib.rs` carries facade/composition-root docs |
| 6 | `paladin-content` rename | — | Not verified; low priority |

**Bottom line:** the deletions and the rename landed; the *relocations never happened*, and the
audit that was supposed to drive them mislabeled the easy wins. Finishing Milestone 8 ≈ finishing
Epic 3 correctly.

---

## 3. Corrections to the prior audit

The prior audit's **"List B — move to crate"** and the Epic 3 disposition treated the files below
as live bridges. They are **orphaned**: not declared as `mod` anywhere in the facade, so they are
**not compiled at all**, and the destination leaf crate **already contains an identical copy**.
The "move" already happened months ago — these are leftover duplicates on disk.

Verification performed: `rg "mod <name>"` across `src/` returns nothing for each; the `mod.rs`
in each directory only does `pub use paladin_<crate>::...`; the leaf crate file exists.

---

## 4. Refreshed disposition — the remaining slop, by risk

### Category 1 — Orphaned dead files: delete outright (ZERO risk, not compiled)

| File | LOC | Note |
|------|----:|------|
| `infrastructure/adapters/document/document_adapter.rs` | 480 | dup in `paladin-content`; not a module |
| `infrastructure/adapters/document/pdf_extractor.rs` | 350 | dup in `paladin-content`; not a module |
| `infrastructure/adapters/input/file_content_fetcher.rs` | 328 | dup in `paladin-content`; not a module |
| `infrastructure/adapters/input/file_content_list_fetcher.rs` | 218 | dup in `paladin-content`; not a module |
| `infrastructure/adapters/input/http_content_fetcher.rs` | 169 | dup in `paladin-content`; not a module |
| `infrastructure/adapters/input/local_file_fetcher.rs` | 14 | dup in `paladin-content`; not a module |
| `infrastructure/adapters/input/news_api_fetcher.rs` | 527 | dup in `paladin-content`; not a module |
| `infrastructure/adapters/output/api_content_deliverer.rs` | 724 | dup in `paladin-web`; not a module |
| `infrastructure/adapters/logs/error_log_adapter.rs` | 875 | `mod` line is commented out in `logs/mod.rs`; dead |
| `infrastructure/repositories/mysql_content_repository.rs` | 780 | dup in `paladin-storage`; not a module |
| **Subtotal** | **~4,465** | deletable with `cargo build`/`cargo test` proving zero impact |

### Category 2 — Compiled but redundant: delete + small repoint (LOW risk)

| File | LOC | Action |
|------|----:|--------|
| `infrastructure/repositories/file_content_repository.rs` | 723 | No consumers (only its own `mod` line) → delete + drop `pub mod` |
| `infrastructure/adapters/paladin_registry.rs` | 418 | Duplicate of `paladin-battalion::in_memory_registry`; 1 consumer (`examples/council_discussion.rs`) → delete + repoint example to `paladin_battalion::` |
| `infrastructure/repositories/sqlite_content_repository.rs` | 810 | `#[cfg(not(storage-sqlite))]` fallback dup of `paladin-storage` → delete, make `paladin-storage` re-export unconditional |
| `infrastructure/repositories/sqlite_user_repository.rs` | 676 | same |

### Category 3 — Genuine relocations: real impl, belongs in a crate, no dup yet (MEDIUM risk — this is the un-done Epic 3)

| File | LOC | Target crate | Note |
|------|----:|--------------|------|
| `infrastructure/adapters/file_storage/minio.rs` | 1,198 | `paladin-storage` | real S3 adapter |
| `infrastructure/adapters/queue/redis.rs` | 1,570 | `paladin-storage` (or new `paladin-queue`) | real Redis adapter |
| `infrastructure/adapters/citadel/file_citadel.rs` | 581 | `paladin-storage` or `paladin-memory` | only impl of `CitadelPort` |
| `infrastructure/adapters/notifications/email_notification_adapter.rs` | 752 | `paladin-notifications` | `#[cfg(not(notifications))]` fallback dup |
| `infrastructure/adapters/notifications/system_notification_adapter.rs` | 320 | `paladin-notifications` | same |
| `infrastructure/web/user_controller.rs` | 888 | `paladin-web` | web layer; `paladin-web` already has the shape |
| `infrastructure/adapters/herald/{json,markdown,table}_herald.rs` | ~1,900 | `paladin-core` or new `paladin-herald` | formatter impls; trait lives in `paladin-core` (orphaned-pattern) — **decision** |

### Category 4 — Judgment calls (need a decision before acting)

| Item | LOC | Question |
|------|----:|----------|
| `application/cli/commands/user.rs` | 1,065 | Wired (`pub mod user`) but no dispatch handler in the CLI binary — dead feature. Delete, or finish wiring it? |
| `infrastructure/adapters/input/tensorflow_adapter.rs` | 636 | Feature-gated `ml` stub, no real impl. Delete (no users), or keep as explicit placeholder? |
| `application/services/paladin/{planning,prompt_generation,temperature,handoff}_service.rs` | ~2,750 | Use-case logic, but tightly entangled with `paladin_builder`/`paladin_execution_service`. Move to `paladin-battalion`/`paladin-llm`, or keep in facade? (Original audit said keep.) |
| `application/services/content/content_ingestion_service.rs` | 1,211 | Domain logic — move to `paladin-content`, or keep? |
| `src/core/` re-export shims (`mod.rs`, `platform/mod.rs`) | ~180 | 275+ facade-internal consumers use `crate::core::…`. Keep for ergonomics, or rip out to force direct `paladin_core::` paths? (High churn, low ROI — recommend KEEP.) |

### Category 5 — Cross-cutting hygiene (separate sweep)

- **`println!`/`eprintln!`/`dbg!`: 435 occurrences across 36 files.** Much of it is legitimate CLI
  output (keep), but the orchestration/services hits should become `log::*`. Scope to
  `services/` + `infrastructure/`, not `cli/`.
- Stray `#[allow(dead_code)]` markers (≈7) — resolve or justify.
- `mod.rs` doc-comments in `application/` and `infrastructure/` describe a structure that has
  since changed (e.g. ports that no longer live here) — refresh.

---

## 5. Proposed execution plan (chunked, smallest-blast-radius first)

> Each phase ends with `cargo build && cargo test && cargo fmt --check && cargo clippy -- -D warnings`
> and a single conventional commit. Stop for go-ahead between phases per the repo protocol.

- **Phase 0 — Correct the record.** Annotate `Epic_1/facade-audit.md` and
  `Epic_3/infrastructure-adapter-disposition.md` as superseded; this doc is the source of truth.
- **Phase 1 — Delete Category 1 (orphaned dead files, ~4,465 LOC).** Pure removals; tests must be
  unchanged. One commit. *This alone removes the bulk of the visible slop with zero risk.*
- **Phase 2 — Category 2 (redundant, ~2,627 LOC).** Delete + repoint the one example; make
  `paladin-storage` re-exports unconditional. One commit.
- **Phase 3 — Category 3 relocations (the real Epic 3).** One file/crate per PR: minio → storage,
  redis → storage, file_citadel → storage, notifications adapters → notifications,
  user_controller → web, herald → (decided crate). Verify each independently.
- **Phase 4 — Category 4 decisions**, then act on each.
- **Phase 5 — Category 5 hygiene sweep** (`println!`→`log`, dead-code markers, mod docs).

**Indicative impact:** Phases 1–2 alone remove ~7,000 LOC of dead/duplicate code from the facade
with effectively zero behavioral risk.

---

## 6. Open decisions (blockers for Phases 3–4)

1. **Finish relocations now vs. defer to Milestone 9?** The original plan deferred them; doing them
   now gives a genuinely clean facade but touches leaf-crate `Cargo.toml`s and public paths.
2. **Herald formatters home:** `paladin-core` (co-locate with the trait) vs. a new `paladin-herald`.
3. **Queue home:** fold `redis.rs` into `paladin-storage` vs. a dedicated `paladin-queue` crate.
4. **`commands/user.rs`:** delete the dead command, or wire it into the CLI.
5. **`tensorflow_adapter.rs`:** delete vs. keep as a gated placeholder.
6. **`src/core/` shims:** keep (recommended) vs. remove.
