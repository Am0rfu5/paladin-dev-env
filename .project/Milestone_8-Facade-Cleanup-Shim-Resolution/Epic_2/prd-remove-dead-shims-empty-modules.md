# PRD: Remove Dead Shims and Empty Modules

**Epic:** 2 — Remove Dead Shims and Empty Modules
**Milestone:** 8 — Facade Cleanup, Shim Resolution, and Directory Stabilization
**Version Target:** v0.2.0
**Priority:** High
**Status:** Ready
**Created:** 2026-05-29
**Document Version:** 1.2
**Depends On:** Epic 1 (`facade-audit.md` completed)

---

## 1. Introduction / Overview

The facade crate (`src/`) has accumulated dead re-export shims, empty files, comment-only stubs, and orphaned directories that are unreachable from the module tree. Epic 1 identified exactly 25 such files (List A in `facade-audit.md`). Additionally, `src/lib.rs` contains approximately 50 `pub use` short-path aliases with zero workspace consumers that contribute unnecessary noise to the public API surface.

This Epic deletes all 25 List A files in **module-area batches** with a `cargo build --workspace` verification after each batch. It also repurposes the original "clean `src/application/ports/`" task into an audit for any lingering `crate::application::ports::` import references left by prior milestone work. Finally, it removes dead `lib.rs` re-export lines as a clean v0.2.0 API break.

No source file in List C (151 staying files) is modified in this Epic except to update `mod.rs` declarations that reference deleted files, and to remove dead `pub use` lines from `src/lib.rs`.

---

## 2. Goals

1. Delete all 25 dead-code files identified in `facade-audit.md` List A with no regressions.
2. Leave the workspace compiling and all tests passing after every individual batch deletion.
3. Remove any workspace code that still imports via `crate::application::ports::` (a path that no longer exists).
4. Remove dead `pub use` lines from `src/lib.rs` that point to modules being deleted, and clean up zero-consumer short-path aliases that clutter the public API.

---

## 3. User Stories

- **As a developer** working on the Paladin codebase, I want dead and empty files removed so that my IDE does not surface phantom completions and unreachable code warnings when navigating `src/`.
- **As a new contributor**, I want the module tree to reflect only real, reachable code so that I can trust that every file I open contains something meaningful.
- **As a maintainer**, I want `src/lib.rs` to export only types that are actively consumed, so that the public API surface is minimal and honest before v0.2.0 is released.

---

## 4. Functional Requirements

### 4.1 — Task 2.1: Delete Dead Files in Module-Area Batches

Delete all 25 files from `facade-audit.md` List A, grouped by module area. After each batch:

1. `rm` the listed files.
2. Update any `mod.rs` that declared the deleted file with `pub mod <name>;` — remove that declaration.
3. Run `cargo build --workspace` and confirm it succeeds before proceeding to the next batch.
4. Commit the batch with a descriptive message (e.g., `refactor(m8-e2): delete orphaned admin/ directory`). Each batch must be a separate commit so that `git bisect` can isolate regressions to a specific deletion group.

After all batches: run `cargo test --workspace` and confirm all tests pass.

**No deprecation stubs are required.** Epic 1 confirmed zero workspace consumers for all 25 files.

**File count verification:** The 7 batches below total exactly 25 files to delete (3 + 4 + 3 + 5 + 4 + 4 + 2 = 25). Batch 4 additionally cascade-deletes `subject/mod.rs`, which is not in the 25-file List A count because it is not itself dead code — it becomes empty only after its children are deleted. The cascade deletion is a consequence of Batch 4, not an independent List A item. After all batches complete, the net file reduction is 26 (25 List A files + 1 cascade `mod.rs`). The Success Metrics section reflects both counts.

---

#### Batch 1 — `src/application/notifications/` (3 files)

This entire directory is orphaned: `src/application/mod.rs` never declared `pub mod notifications;`, making all three files unreachable from the module tree.

> ⚠️ Before deleting `email_notifications.rs`, open the file and confirm it does not contain logic needed by Epic 3 (which moves the full email adapter to `paladin-notifications`). If it does, preserve a copy outside `src/` for reference. The Epic 1 audit notes it as a 392-LOC candidate for review.

| File to Delete | LOC | Reason |
|----------------|-----|--------|
| `src/application/notifications/email_notifications.rs` | 392 | Orphaned — not declared in `application/mod.rs` |
| `src/application/notifications/push_notifications.rs` | 0 | Empty + orphaned |
| `src/application/notifications/system_notifications.rs` | 0 | Empty + orphaned |

**Post-deletion cleanup:** `src/application/mod.rs` does **not** need updating (it never declared the module). Delete the now-empty `src/application/notifications/` directory.

---

#### Batch 2 — `src/application/storage/` stubs (4 files)

These are comment-only stub files with zero consumers, declared in `src/application/storage/mod.rs`.

| File to Delete | LOC | Reason |
|----------------|-----|--------|
| `src/application/storage/file_store.rs` | 6 | Comment-only stub; zero consumers |
| `src/application/storage/key_store.rs` | 21 | Comment-only stub; zero consumers |
| `src/application/storage/key_value_store.rs` | 13 | Comment-only stub; zero consumers |
| `src/application/storage/nosql_store.rs` | 5 | Comment-only stub; zero consumers |

**Post-deletion cleanup:** Update `src/application/storage/mod.rs` — remove the four declarations:
```rust
pub mod file_store;       // delete this line
pub mod key_store;        // delete this line
pub mod key_value_store;  // delete this line
pub mod nosql_store;      // delete this line
```
The two remaining active files (`sql_store.rs`, `user_store.rs`) and their declarations stay.

---

#### Batch 3 — `src/application/use_cases/content/` empty files (3 files)

Three empty placeholder files declared in `src/application/use_cases/content/mod.rs`.

| File to Delete | LOC | Reason |
|----------------|-----|--------|
| `src/application/use_cases/content/content_list_ingestion_service.rs` | 0 | Empty file |
| `src/application/use_cases/content/content_list_service.rs` | 0 | Empty file |
| `src/application/use_cases/content/content_ml_analysis_service.rs` | 0 | Empty file |

**Post-deletion cleanup:** Update `src/application/use_cases/content/mod.rs` — remove the three declarations for the deleted files. All other `pub mod` lines in that file stay.

---

#### Batch 4 — `src/application/use_cases/subject/` (5 files + cascade)

The four service stub files are declared in `subject/mod.rs` but are completely empty (single newline each). `subject.rs` is a comment-only stub that was never declared in `subject/mod.rs`. After deleting the four stubs, `subject/mod.rs` has no live children and must be cascade-deleted.

| File to Delete | LOC | Reason |
|----------------|-----|--------|
| `src/application/use_cases/subject/subject.rs` | 4 | Comment-only stub; not declared in `subject/mod.rs` |
| `src/application/use_cases/subject/subject_build_service.rs` | 1 | Empty (single newline) |
| `src/application/use_cases/subject/subject_search_service.rs` | 1 | Empty (single newline) |
| `src/application/use_cases/subject/subject_service.rs` | 1 | Empty (single newline) |
| `src/application/use_cases/subject/subject_tagging_service.rs` | 1 | Empty (single newline) |

**Post-deletion cascade:**
1. Delete `src/application/use_cases/subject/mod.rs` (now an empty declaration file with no live children — this is a cascade deletion, not a List A item).
2. Delete the now-empty `src/application/use_cases/subject/` directory.
3. Update `src/application/use_cases/mod.rs` — remove `pub mod subject;`.

---

#### Batch 5 — `src/core/platform/manager/admin/` (4 files — entire directory)

The `admin/` sub-directory is not declared in `src/core/platform/manager/mod.rs`, making the entire tree unreachable.

| File to Delete | LOC | Reason |
|----------------|-----|--------|
| `src/core/platform/manager/admin/mod.rs` | 2 | Orphaned directory root |
| `src/core/platform/manager/admin/admin_console_service.rs` | 4 | Comment-only stub; orphaned |
| `src/core/platform/manager/admin/admin_logging_service.rs` | 4 | Comment-only stub; orphaned |
| `src/core/platform/manager/admin/admin_notification_service.rs` | 4 | Comment-only stub; orphaned |

**Post-deletion cleanup:** Delete the now-empty `src/core/platform/manager/admin/` directory. `src/core/platform/manager/mod.rs` does **not** need updating — it never declared `pub mod admin;`.

---

#### Batch 6 — `src/core/platform/manager/user/` (4 files — entire directory)

The `user/` sub-directory is not declared in `src/core/platform/manager/mod.rs`, making the entire tree unreachable. `user_notification_service.rs` additionally references a non-existent `crate::core::domain::entities::*` import path.

| File to Delete | LOC | Reason |
|----------------|-----|--------|
| `src/core/platform/manager/user/mod.rs` | 1 | Orphaned directory root |
| `src/core/platform/manager/user/user_account_service.rs` | 0 | Empty + orphaned |
| `src/core/platform/manager/user/user_notification_service.rs` | 12 | References non-existent paths; orphaned |
| `src/core/platform/manager/user/user_settings_service.rs` | 0 | Empty + orphaned |

**Post-deletion cleanup:** Delete the now-empty `src/core/platform/manager/user/` directory. `src/core/platform/manager/mod.rs` does **not** need updating — it never declared `pub mod user;`.

---

#### Batch 7 — Infrastructure empty adapter stubs (2 files)

Two empty stub files, each one line, declared in their respective `mod.rs` files.

| File to Delete | LOC | Reason |
|----------------|-----|--------|
| `src/infrastructure/adapters/logs/access_log_adapter.rs` | 1 | Empty (single newline) |
| `src/infrastructure/adapters/notifications/push_notification_adapter.rs` | 1 | Empty (single newline) |

**Post-deletion cleanup:**
- Update `src/infrastructure/adapters/logs/mod.rs` — remove `pub mod access_log_adapter;`.
- Update `src/infrastructure/adapters/notifications/mod.rs` — remove the declaration for `push_notification_adapter`.
- Run `cargo build --workspace` to confirm no breakage.

---

### 4.2 — Task 2.2: Audit Remaining `crate::application::ports::` References

Epic 1 confirmed that `src/application/ports/` does **not exist** — it was removed in a prior milestone. However, stale import paths of the form `crate::application::ports::*` or `paladin::application::ports::*` may still appear in `src/`, `tests/`, `examples/`, or `benches/`.

**Steps:**

1. Run the following grep across the full workspace:
   ```bash
   grep -rn "application::ports::" src/ crates/ tests/ examples/ benches/ --include="*.rs"
   ```
2. For every match found, update the import to use the correct direct path (typically `paladin_ports::` or the full `paladin::application::use_cases::...` path, depending on context).
3. Run `cargo build --workspace` after all fixes to confirm no compile errors.
4. If no matches are found, document the result as "confirmed zero stale `application::ports::` references" in the task notes.

**Expected outcome based on Epic 1 audit:** Zero or near-zero matches. The task is primarily a safety check.

---

### 4.3 — Task 2.3: Verify `src/core/` Minimum Re-Export Structure

After Batches 5 and 6 complete, verify that `src/core/` has been reduced to its minimum necessary structure. The following 6 files are the only legitimate inhabitants of `src/core/`:

| File | Role | Status |
|------|------|--------|
| `src/core/mod.rs` | Bridge shim — 275+ workspace consumers | Must stay |
| `src/core/platform/mod.rs` | Bridge shim with Maneuver injection logic | Must stay |
| `src/core/platform/manager/mod.rs` | Module declaration | Must stay |
| `src/core/platform/manager/content_service.rs` | Application service (~385 LOC) | Must stay |
| `src/core/platform/manager/event_manager.rs` | Application service (~345 LOC) | Must stay |
| `src/core/platform/manager/user_service.rs` | Application service (~414 LOC) | Must stay |

**Steps:**

1. Run `find src/core/ -name "*.rs" | sort` and confirm only the 6 files above are present.
2. Verify that `src/core/mod.rs` still compiles and that its re-exports remain valid (i.e., the deleted `admin/` and `user/` sub-trees were never referenced by it).
3. Verify that `src/core/platform/manager/mod.rs` declares exactly three modules: `content_service`, `event_manager`, `user_service` — and nothing else.
4. Run `cargo test --workspace` to confirm no regression.

---

### 4.4 — Task 2.4: Remove Dead `pub use` Lines from `src/lib.rs`

`src/lib.rs` exports a curated stable API via `pub use` short-path aliases. Epic 1 Appendix B Section 2 identified that most of these aliases have **zero workspace consumers** — workspace code uses full module paths (e.g., `paladin::application::use_cases::paladin::paladin_builder::PaladinBuilder`), not the short form (e.g., `paladin::PaladinBuilder`).

For v0.2.0 this is a **clean break** — no deprecated re-exports are needed since there are no known consumers of the zero-consumer aliases.

**Exceptions — lines that MUST stay (confirmed consumers):**

| `pub use` line | Consumers |
|----------------|-----------|
| `pub use paladin_llm::mock::{MockLlmAdapter, MultiStepMockLlmPort};` | 13 consumers |
| `pub use paladin_llm::openai::{OpenAIAdapter, OpenAIConfig};` | Part of same 13-consumer group |
| `pub use paladin_llm::anthropic::{AnthropicAdapter, AnthropicConfig};` | Part of same 13-consumer group |
| `pub use paladin_llm::deepseek::{DeepSeekAdapter, DeepSeekConfig};` | Part of same 13-consumer group |
| `pub use core::platform::container::paladin::{Paladin, PaladinData, PaladinStatus};` | 17 consumers of `paladin::Paladin` |

All other zero-consumer `pub use` lines are candidates for removal. The types they re-export still exist at their full module paths.

**Steps:**

1. Open `src/lib.rs`.
2. Using `facade-audit.md` Appendix B Section 2 as the reference, identify every `pub use` line with `Has Consumers? = No`.
3. For each such line, confirm the type still exists at its source path (i.e., we are removing only the alias, not the type itself).
4. Remove the dead `pub use` lines. Keep the five exceptions listed above and any module-level `pub mod` declarations — do not touch those.
5. Run `cargo build --workspace` — any remaining broken re-exports will surface as compile errors here; fix them.
6. Run `cargo test --workspace` to confirm no regression.
7. Update `STABLE_API.md` to reflect the removed aliases (this is a v0.2.0 breaking change, document it in `CHANGELOG.md` under "Removed" for v0.2.0).

**CHANGELOG example format:**

```markdown
### Removed (v0.2.0)
- `paladin::LlmPort` — use `paladin_ports::output::llm_port::LlmPort`
- `paladin::GarrisonPort` — use `paladin_ports::output::garrison_port::GarrisonPort`
- `paladin::ArsenalError` — use `paladin_core::platform::container::arsenal::ArsenalError`
```

Note: Use replacement paths that reference stable crate locations (`paladin_ports::`, `paladin_core::`, `paladin_battalion::`) rather than facade-internal paths (`paladin::application::use_cases::...`). Facade-internal paths will change in Epic 4 (`use_cases` → `services` rename) and should not appear as recommended replacements in the CHANGELOG.

---

## 5. Non-Goals (Out of Scope)

- **Moving List B files** — the 13 files to be relocated to leaf crates (`paladin-notifications`, `paladin-storage`, etc.) are handled in Epic 3, not here.
- **Renaming `use_cases` → `services`** — that is Epic 4.
- **Adding new documentation** beyond STABLE_API.md and CHANGELOG.md updates in Task 2.4.
- **Modifying any List C file's logic** — this Epic only touches `mod.rs` declarations and `lib.rs` re-export lines within List C files; no business logic changes.
- **Adding `#[deprecated]` stubs** — Epic 1 confirmed zero consumers; no backward compatibility shims are needed.
- **Touching any file under `crates/`** — this Epic is facade-crate-only (except Task 2.2 which may fix stale imports in `crates/` if found).

---

## 6. Technical Considerations

### Verification Gate Between Batches

Every batch ends with `cargo build --workspace`. If the build fails after a batch, stop and fix the build before proceeding to the next batch. The expected output after each batch is:

```
Compiling paladin v0.1.0 (/workspace)
Finished `dev` profile [unoptimized + debuginfo] target(s) in N.Ns
```

with no errors and no new warnings introduced by the deletions.

### Git Commit Strategy

Each batch must be committed separately before starting the next batch. This provides clean `git bisect` points if a regression is discovered later, and ensures a failed batch in the middle does not require re-doing successful earlier batches. Commit messages should follow the pattern: `refactor(m8-e2): delete <description>`.

Recommended commit sequence:
1. `refactor(m8-e2): delete orphaned src/application/notifications/ directory`
2. `refactor(m8-e2): delete comment-only storage stubs`
3. `refactor(m8-e2): delete empty content use_case placeholders`
4. `refactor(m8-e2): delete empty subject use_case stubs and cascade mod.rs`
5. `refactor(m8-e2): delete orphaned core/platform/manager/admin/ directory`
6. `refactor(m8-e2): delete orphaned core/platform/manager/user/ directory`
7. `refactor(m8-e2): delete empty infrastructure adapter stubs`
8. `refactor(m8-e2): audit and clean stale application::ports:: references` (Task 2.2)
9. `refactor(m8-e2): verify src/core/ minimum structure` (Task 2.3)
10. `refactor(m8-e2): remove dead pub use lines from lib.rs` (Task 2.4)

### `mod.rs` Declaration Cleanup Rules

When a file is deleted that was declared in a parent `mod.rs`, the `pub mod <name>;` line **must** be removed from that `mod.rs` in the same batch. Leaving a dangling `pub mod` declaration for a deleted file will cause a compile error.

When a `mod.rs` itself is deleted (cascade scenarios in Batches 4, 5, 6), the `pub mod <dir_name>;` declaration in the **grandparent** `mod.rs` must also be removed in the same batch.

### Orphaned Directories (Batches 1, 5, 6)

For `src/application/notifications/`, `src/core/platform/manager/admin/`, and `src/core/platform/manager/user/`, the parent `mod.rs` (respectively `application/mod.rs` and `core/platform/manager/mod.rs`) never declared these directories. This is what makes them orphaned — no compilation path reaches them. Because they were never declared, **no update to the parent `mod.rs` is needed** for these three batches; simply deleting the files and directories is sufficient.

### `subject/mod.rs` Cascade (Batch 4)

After deleting the four service stubs, `subject/mod.rs` still compiles (it declares modules that no longer exist at the file level, which causes a compile error). This means the `cargo build --workspace` check at the end of Batch 4 will fail if `subject/mod.rs` is not also deleted. The cascade delete of `subject/mod.rs` and the removal of `pub mod subject;` from `use_cases/mod.rs` must happen **in the same Batch 4 operation** before running the build check.

### Task 2.4 and `STABLE_API.md`

The removed `lib.rs` aliases represent the first intentional public API contraction for the project. `CHANGELOG.md` must have a `### Removed` section under the v0.2.0 entry that lists each removed alias and its replacement full path.

Replacement paths in the CHANGELOG must reference stable crate-level locations (`paladin_ports::`, `paladin_core::`, `paladin_battalion::`, `paladin_llm::`) rather than facade-internal module paths (`paladin::application::use_cases::...`). The facade-internal paths will change in Epic 4 when `use_cases` is renamed to `services`, and listing them as recommended replacements would make the CHANGELOG stale one Epic later.

---

## 7. Success Metrics

| Metric | Target |
|--------|--------|
| List A files deleted | Exactly 25 (matching `facade-audit.md` List A) |
| Cascade `mod.rs` deletions | 1 (`subject/mod.rs` — consequence of Batch 4, not a List A item) |
| Total files removed | 26 (25 List A + 1 cascade) |
| `find src/ -name "*.rs" \| wc -l` after all deletions | 163 (189 − 26) |
| `cargo build --workspace` after each batch | Exit code 0, zero new errors |
| `cargo test --workspace` after all batches | All previously-passing tests pass; zero new failures |
| `cargo clippy --workspace -- -D warnings` | Zero new warnings introduced |
| `grep -r "application::ports::" src/` (Task 2.2) | Zero matches remaining |
| Dangling `pub mod` declarations | Zero — every deleted file has its declaration removed |
| `src/lib.rs` dead `pub use` lines removed | All zero-consumer aliases removed per Appendix B Section 2 |
| Git commits | One commit per batch (7) + one per task (Tasks 2.2, 2.3, 2.4) = 10 commits |

---

## 8. Open Questions

1. **`email_notifications.rs` review (Batch 1, row 1):** Before deletion, confirm whether the 392-LOC content in `src/application/notifications/email_notifications.rs` overlaps with `src/infrastructure/adapters/notifications/email_notification_adapter.rs` (752 LOC, List B). If it contains unique logic, stage it as an input artifact for Epic 3 Task 3.1 before deleting.

2. **`src/lib.rs` module-level `pub mod` declarations:** Task 2.4 removes `pub use` lines but does not remove `pub mod` declarations. After this Epic, `src/lib.rs` will still declare `pub mod application`, `pub mod core`, `pub mod infrastructure`, etc. Epic 5 (documentation finalization) should re-evaluate whether all top-level `pub mod` declarations in `lib.rs` remain appropriate.

3. **`prelude.rs` dead exports:** `src/prelude.rs` has zero workspace consumers but is explicitly preserved as a public API convenience module. This Epic does not touch it. Epic 5 should decide whether to populate it with the types that survive Task 2.4 or remove it.

---

## Task Checklist

### Task 2.1 — Delete Dead Files in Module-Area Batches
- [ ] Batch 1: Delete `src/application/notifications/` (3 files), delete directory, commit
- [ ] `cargo build --workspace` — green
- [ ] Batch 2: Delete 4 storage stubs, update `storage/mod.rs`, commit
- [ ] `cargo build --workspace` — green
- [ ] Batch 3: Delete 3 content empty files, update `content/mod.rs`, commit
- [ ] `cargo build --workspace` — green
- [ ] Batch 4: Delete 5 subject stubs + cascade `subject/mod.rs`, update `use_cases/mod.rs`, delete directory, commit
- [ ] `cargo build --workspace` — green
- [ ] Batch 5: Delete `src/core/platform/manager/admin/` (4 files), delete directory, commit
- [ ] `cargo build --workspace` — green
- [ ] Batch 6: Delete `src/core/platform/manager/user/` (4 files), delete directory, commit
- [ ] `cargo build --workspace` — green
- [ ] Batch 7: Delete 2 infrastructure stubs, update respective `mod.rs` files, commit
- [ ] `cargo build --workspace` — green
- [ ] Run `cargo test --workspace` — all tests pass

### Task 2.2 — Audit Remaining `crate::application::ports::` References
- [ ] Run workspace-wide grep for `application::ports::`
- [ ] Fix any matches found (update imports to `paladin_ports::`)
- [ ] `cargo build --workspace` — green (if changes were made)
- [ ] Document result, commit

### Task 2.3 — Verify `src/core/` Minimum Re-Export Structure
- [ ] Run `find src/core/ -name "*.rs" | sort` — confirm exactly 6 files
- [ ] Verify `core/mod.rs` compiles and re-exports are valid
- [ ] Verify `manager/mod.rs` declares exactly 3 modules
- [ ] `cargo test --workspace` — green
- [ ] Document result, commit

### Task 2.4 — Remove Dead `pub use` Lines from `src/lib.rs`
- [ ] Identify all zero-consumer `pub use` lines from Appendix B Section 2
- [ ] Confirm each type still exists at its source path
- [ ] Remove dead lines (keep 5 exceptions)
- [ ] `cargo build --workspace` — green
- [ ] `cargo test --workspace` — green
- [ ] Update `STABLE_API.md` with removed aliases
- [ ] Add `### Removed` section to `CHANGELOG.md` for v0.2.0 (use crate-level replacement paths, not facade-internal paths)
- [ ] Commit
