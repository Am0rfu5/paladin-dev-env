## Relevant Files

- `src/application/use_cases/` → `src/application/services/` — The directory being renamed (39 `.rs` files across 11 sub-modules). All contents move intact.
- `src/application/mod.rs` — Module declaration `pub mod use_cases;` → `pub mod services;` plus ~12 doc-comment links.
- `src/lib.rs` — Top-level re-exports and doc examples that reference `use_cases` paths.
- `src/config/setup/service_runner.rs` — Imports multiple orchestrator services via `use_cases` paths.
- `src/infrastructure/adapters/` — Various adapter files that import application services.
- `src/core/` — Core files that cross-import application services.
- `tests/` — Integration and functional tests that import via `paladin::application::use_cases::`.
- `examples/` — All example files that import via `paladin::application::use_cases::`.
- `benches/` — Benchmark files that reference `use_cases` paths.
- `CHANGELOG.md` — Must gain a `### Breaking Changes` entry with the full migration table.
- `STABLE_API.md` — 7 `use_cases` path declarations must be updated to `services`.
- `README.md` — User-facing import examples updated.
- `CONTRIBUTING.md` — Contributor docs updated.
- `docs/` (all `.md` files) — 57 total markdown hits across docs updated.

### Notes

- Run `cargo test --workspace` after the main rename commit before touching any documentation.
- Run `cargo fmt --check` and `cargo clippy --workspace -- -D warnings` before each commit.
- Commit messages must use conventional commit format with `-m` flags. No `!` characters in commit messages (bash history expansion guard).
- The `find src/ -name "*.rs" | wc -l` count must remain **160** after the rename (files move, none are added or deleted).
- After the rename `grep -r "use_cases" src/ tests/ examples/ benches/ --include="*.rs"` must return **0 hits** (excluding the CHANGELOG migration table).
- This is a **breaking change** — the CHANGELOG entry with the full 11-row migration table is mandatory before the final commit.

## Instructions for Completing Tasks

**IMPORTANT:** As you complete each task, you must check it off in this markdown file by changing `- [ ]` to `- [x]`. This helps track progress and ensures you don't skip any steps.

Example:

- `- [ ] 1.1 Read file` → `- [x] 1.1 Read file` (after completing)

Update the file after completing each sub-task, not just after completing an entire parent task.

## Tasks

- [x] 0.0 Create feature branch
  - [x] 0.1 Create and checkout a new branch: `git checkout -b feature/milestone_8-epic_4-use_cases-services-rename`
  - [x] 0.2 Confirm branch is active: `git branch --show-current`

- [x] 1.0 Rename directory and update module declaration
  - [x] 1.1 Run `git mv src/application/use_cases src/application/services` to rename the directory while preserving git history.
  - [x] 1.2 In `src/application/mod.rs`, change `pub mod use_cases;` to `pub mod services;`. Also update the doc-comment lines in that file that reference `use_cases` paths (the lines containing `crate::application::use_cases::`).
  - [x] 1.3 Run `cargo check 2>&1 | head -20` to get a quick list of the first broken import paths — this confirms the rename worked and shows the scope of remaining work before the full fix pass.

- [ ] 2.0 Update all Rust import paths (`src/`, `tests/`, `examples/`, `benches/`)
  - [ ] 2.1 Run a targeted count before starting: `grep -rn "use_cases" src/ tests/ examples/ benches/ --include="*.rs" | grep -v "^\s*//" | wc -l` — expect ~286. Record this baseline.
  - [ ] 2.2 Replace all `use_cases` occurrences in `src/` Rust files: `find src/ -name "*.rs" -exec sed -i 's/use_cases/services/g' {} +`. This covers `use crate::application::use_cases::`, internal doc-comment links (`crate::application::use_cases::`), and any identifiers containing `use_cases`.
  - [ ] 2.3 Replace all `use_cases` occurrences in `tests/` Rust files: `find tests/ -name "*.rs" -exec sed -i 's/use_cases/services/g' {} +`.
  - [ ] 2.4 Replace all `use_cases` occurrences in `examples/` Rust files: `find examples/ -name "*.rs" -exec sed -i 's/use_cases/services/g' {} +`.
  - [ ] 2.5 Replace all `use_cases` occurrences in `benches/` Rust files: `find benches/ -name "*.rs" -exec sed -i 's/use_cases/services/g' {} +`.
  - [ ] 2.6 Verify zero remaining hits: `grep -rn "use_cases" src/ tests/ examples/ benches/ --include="*.rs" | grep -v "^\s*//"` — must return empty.
  - [ ] 2.7 Run `cargo build --workspace` — confirm exit 0 with zero errors.
  - [ ] 2.8 Run `cargo test --workspace` — confirm all tests pass, zero failures.
  - [ ] 2.9 Run `cargo fmt` to fix any formatting drift introduced by the sed pass, then `cargo fmt --all -- --check` to confirm clean.
  - [ ] 2.10 Run `cargo clippy --workspace -- -D warnings` — confirm zero warnings.
  - [ ] 2.11 Confirm file count unchanged: `find src/ -name "*.rs" | wc -l` — must be **160**.
  - [ ] 2.12 Commit: `git add -A && git commit -m "refactor(m8-e4): rename application/use_cases to application/services" -m "- git mv src/application/use_cases src/application/services" -m "- Updated pub mod declaration in application/mod.rs" -m "- Replaced all 286 use_cases references in src/, tests/, examples/, benches/" -m "- Breaking change: paladin::application::use_cases::* paths now at paladin::application::services::*"`

- [ ] 3.0 Update markdown documentation (`docs/`, root `.md` files, `STABLE_API.md`)
  - [ ] 3.1 Run a baseline count: `grep -rn "use_cases" docs/ README.md CHANGELOG.md CONTRIBUTING.md STABLE_API.md` — expect ~64 total (57 docs + 7 STABLE_API).
  - [ ] 3.2 Replace `use_cases` in all `.md` files under `docs/`: `find docs/ -name "*.md" -exec sed -i 's/use_cases/services/g' {} +`.
  - [ ] 3.3 Replace `use_cases` in root markdown files: `sed -i 's/use_cases/services/g' README.md CONTRIBUTING.md STABLE_API.md`.
  - [ ] 3.4 **Do not** run sed on `CHANGELOG.md` yet — the CHANGELOG update is handled manually in Task 4.0 so the migration table can document both the old and new paths correctly.
  - [ ] 3.5 Verify zero remaining hits (excluding CHANGELOG): `grep -rn "use_cases" docs/ README.md CONTRIBUTING.md STABLE_API.md` — must return empty.
  - [ ] 3.6 Commit: `git add docs/ README.md CONTRIBUTING.md STABLE_API.md && git commit -m "docs(m8-e4): update use_cases to services in all markdown documentation" -m "- Updated docs/ (57 hits), README.md, CONTRIBUTING.md, STABLE_API.md (7 hits)" -m "- CHANGELOG.md updated separately in Task 4.0 with breaking change migration table"`

- [ ] 4.0 Add breaking change entry to `CHANGELOG.md`
  - [ ] 4.1 Add a `### Breaking Changes` subsection under the `[Unreleased]` section in `CHANGELOG.md`. It must include:
    - One-line summary: "`src/application/use_cases/` renamed to `src/application/services/`"
    - The full 11-row migration table from PRD §4.4 (all `paladin::application::use_cases::*` → `paladin::application::services::*` sub-module paths).
  - [ ] 4.2 Replace all remaining `use_cases` references in `CHANGELOG.md` that are **not** part of the migration table (e.g., references from previous changelog entries): `grep -n "use_cases" CHANGELOG.md` — review each hit and update non-table references to `services` manually.
  - [ ] 4.3 Verify: `grep "use_cases" CHANGELOG.md` — only the migration table's "Old path" column should contain `use_cases`; all other references must be `services`.
  - [ ] 4.4 Commit: `git add CHANGELOG.md && git commit -m "docs(m8-e4): add breaking change entry for use_cases to services rename" -m "- Added Breaking Changes section under Unreleased" -m "- Full 11-row migration table for all paladin::application::use_cases sub-modules"`

- [ ] 5.0 Final quality gate and epic close
  - [ ] 5.1 `cargo build --workspace` — exit 0, zero errors.
  - [ ] 5.2 `cargo test --workspace` — all tests pass, zero failures.
  - [ ] 5.3 `cargo clippy --workspace -- -D warnings` — zero warnings.
  - [ ] 5.4 `cargo fmt --all -- --check` — exit 0, no drift.
  - [ ] 5.5 `find src/ -name "*.rs" | wc -l` — confirm **160** (unchanged).
  - [ ] 5.6 `grep -r "use_cases" src/ tests/ examples/ benches/ --include="*.rs"` — **0 hits**.
  - [ ] 5.7 `grep "use_cases" STABLE_API.md` — **0 hits**.
  - [ ] 5.8 Mark all tasks `[x]` in this file.
  - [ ] 5.9 Commit: `git add project/ && git commit -m "docs(m8-e4): mark all Epic 4 tasks complete" -m "- Tasks 0.0-5.0 complete" -m "- use_cases renamed to services, 286 Rust refs updated, 64 markdown refs updated" -m "- Breaking change recorded in CHANGELOG.md with full migration table" -m "- Quality gate passed: build, test, clippy, fmt, file count 160"`
