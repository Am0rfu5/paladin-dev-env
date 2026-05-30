## Relevant Files

- `crates/paladin-content/src/use_cases/` — Directory to be renamed to `src/services/` via `git mv`.
- `crates/paladin-content/src/services/` — Target directory after rename; all 13 service files must be present.
- `crates/paladin-content/src/lib.rs` — Update `pub mod use_cases` → `pub mod services`; update `//!` doc comment.
- `crates/paladin-content/src/adapters/input/http_content_fetcher.rs` — Replace 1 `crate::use_cases` ref.
- `crates/paladin-content/src/adapters/input/file_content_list_fetcher.rs` — Replace 1 `crate::use_cases` ref.
- `crates/paladin-content/src/adapters/input/news_api_fetcher.rs` — Replace 2 `crate::use_cases` refs.
- `crates/paladin-content/src/services/content_llm_analysis_service.rs` — Replace 1 `crate::use_cases` ref (path also updated by directory rename).
- `crates/paladin-content/README.md` — Update all `use_cases` references in prose and code examples.
- `CHANGELOG.md` — Add `fix:` entry under `## [Unreleased]` → `### Fixed`.
- `src/application/services/content/mod.rs` — **No changes required** — already references `paladin_content::services::*`.
- `project/Milestone_8-Facade-Cleanup-Shim-Resolution/Epic_6/tasks-paladin-content-use_cases-services-rename.md` — This task file.

### Notes

- All work continues on the existing branch `feature/milestone_8-epic_4-use_cases-services-rename`. No new branch is needed (task 0.0 is a verification step, not a creation step).
- Use `git mv` for the directory rename — not `mv`. Verify `git status` shows `renamed:` entries, not `deleted:` + `untracked:`.
- The key regression test is `cargo build --workspace --features content-processing`. Default `cargo test`/`cargo build` will NOT catch the broken re-exports because they are `#[cfg(feature = "content-processing")]`-gated.
- `src/application/services/content/mod.rs` already references `paladin_content::services::*` — touching it would be a regression.
- Commit message must use `-m` flags and **no `!` character** (bash history expansion guard).
- Pre-commit hook runs `cargo check` + `cargo fmt --check` automatically.

## Instructions for Completing Tasks

**IMPORTANT:** As you complete each task, you must check it off in this markdown file by changing `- [ ]` to `- [x]`. This helps track progress and ensures you don't skip any steps.

Example:

- `- [ ] 1.1 Read file` → `- [x] 1.1 Read file` (after completing)

Update the file after completing each sub-task, not just after completing an entire parent task.

## Tasks

- [ ] 0.0 Verify active branch and pre-flight checks
  - [ ] 0.1 create a new branch `feature/milestone_8-epic_6-use-cases-services-rename` from `feature/milestone_8`.
  - [ ] 0.2 Confirm `crates/paladin-content/src/use_cases/` still exists: `ls crates/paladin-content/src/`.
  - [ ] 0.3 Confirm the exact `crate::use_cases` reference count is still 5 occurrences in 4 files: `grep -rn "crate::use_cases" crates/paladin-content/src/ --include="*.rs"`.
  - [ ] 0.4 Confirm `src/application/services/content/mod.rs` references `paladin_content::services::*` (no change needed): `cat src/application/services/content/mod.rs`.

- [ ] 1.0 Rename `use_cases/` directory to `services/` via `git mv`
  - [ ] 1.1 Run: `git mv crates/paladin-content/src/use_cases crates/paladin-content/src/services`
  - [ ] 1.2 Verify the rename was tracked by Git (not delete + untracked): `git status | grep -E "renamed|deleted|untracked"` — must show `renamed:` entries only.
  - [ ] 1.3 Verify all 13 expected files are present in `src/services/`: `ls crates/paladin-content/src/services/` and confirm against the list in FR-2.
  - [ ] 1.4 Verify `crates/paladin-content/src/use_cases/` no longer exists: `ls crates/paladin-content/src/`.

- [ ] 2.0 Update `lib.rs` module declaration and doc comment
  - [ ] 2.1 Read `crates/paladin-content/src/lib.rs` to confirm the exact current text before editing.
  - [ ] 2.2 Change the module declaration from `pub mod use_cases;` to `pub mod services;`.
  - [ ] 2.3 Update the `//!` crate-level doc comment: replace `"use-case services"` (or equivalent `use_cases` wording) with `"application services"` (FR-4).
  - [ ] 2.4 Run `cargo build -p paladin-content` — expect compile errors (internal refs not yet updated). Confirm the errors are only `crate::use_cases` unresolved imports, not anything unexpected.

- [ ] 3.0 Replace all internal `crate::use_cases` references (5 occurrences, 4 files)
  - [ ] 3.1 Update `crates/paladin-content/src/adapters/input/http_content_fetcher.rs`: replace `crate::use_cases` → `crate::services` (1 occurrence).
  - [ ] 3.2 Update `crates/paladin-content/src/adapters/input/file_content_list_fetcher.rs`: replace `crate::use_cases` → `crate::services` (1 occurrence).
  - [ ] 3.3 Update `crates/paladin-content/src/adapters/input/news_api_fetcher.rs`: replace both `crate::use_cases` → `crate::services` (2 occurrences).
  - [ ] 3.4 Update `crates/paladin-content/src/services/content_llm_analysis_service.rs`: replace `crate::use_cases` → `crate::services` (1 occurrence).
  - [ ] 3.5 Verify zero `crate::use_cases` references remain: `grep -rn "crate::use_cases" crates/paladin-content/src/ --include="*.rs"` — must return no output.
  - [ ] 3.6 Run `cargo build -p paladin-content` — must exit 0.

- [ ] 4.0 Update `crates/paladin-content/README.md`
  - [ ] 4.1 Read `crates/paladin-content/README.md` to identify all `use_cases` occurrences (prose, import examples, type-name examples).
  - [ ] 4.2 Replace all `use_cases` references with `services` (module description, `use paladin_content::use_cases;` → `use paladin_content::services;`, any `use_cases::` path prefixes).
  - [ ] 4.3 Verify no `use_cases` references remain: `grep -n "use_cases" crates/paladin-content/README.md` — must return no output.

- [ ] 5.0 Verify `--features content-processing` build resolves the six `E0432` errors
  - [ ] 5.1 Run: `cargo build --workspace --features content-processing 2>&1 | grep -E "^error"` — must produce zero output.
  - [ ] 5.2 Run: `cargo build --workspace` — must exit 0 (default feature set).
  - [ ] 5.3 Run: `cargo test --workspace --features content-processing` — must exit 0.
  - [ ] 5.4 Run: `cargo test --workspace` — must exit 0.

- [ ] 6.0 Add CHANGELOG entry
  - [ ] 6.1 Read the current `## [Unreleased]` block in `CHANGELOG.md` to see whether a `### Fixed` sub-section already exists.
  - [ ] 6.2 Add a `### Fixed` entry under `## [Unreleased]` describing: the `use_cases` → `services` rename in `paladin-content`; resolution of 6 `E0432` errors in `src/application/services/content/mod.rs`; note that errors were masked by the `content-processing` feature gate.

- [ ] 7.0 Full quality gate and commit
  - [ ] 7.1 Run `cargo clippy --workspace -- -D warnings` — must exit 0.
  - [ ] 7.2 Run `cargo fmt --all -- --check` — must exit 0.
  - [ ] 7.3 Stage all changes: `git add crates/paladin-content/ CHANGELOG.md`.
  - [ ] 7.4 Verify `src/application/services/content/mod.rs` is **not** in the staged diff: `git diff --cached --name-only | grep "content/mod.rs"` — must return no output.
  - [ ] 7.5 Confirm zero `use_cases` references remain anywhere in `crates/paladin-content/`: `grep -rn "use_cases" crates/paladin-content/` — must return no output.
  - [ ] 7.6 Commit:
    ```bash
    git commit \
      -m "fix(m8-e6): rename use_cases -> services in paladin-content" \
      -m "- git mv src/use_cases -> src/services" \
      -m "- Updated lib.rs: pub mod services; updated crate doc comment" \
      -m "- Updated internal crate::use_cases refs in adapter files (5 occurrences, 4 files)" \
      -m "- Updated README.md examples and prose" \
      -m "- Resolves E0432 unresolved import errors in facade content/mod.rs" \
      -m "- Closes broken re-export bridge introduced by Epic 4 rename" \
      -m "- Errors were previously masked by content-processing feature gate"
    ```
  - [ ] 7.7 Mark all sub-tasks and parent tasks `[x]` in this file and commit the task file update:
    ```bash
    git add project/Milestone_8-Facade-Cleanup-Shim-Resolution/Epic_6/tasks-paladin-content-use_cases-services-rename.md
    git commit -m "chore(m8-e6): mark all Epic 6 tasks complete"
    ```
