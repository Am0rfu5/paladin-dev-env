## Relevant Files

- `project/Milestone_8-Facade-Cleanup-Shim-Resolution/Epic_1/facade-audit.md` - The primary deliverable. Created and populated during this Epic. Contains the full inventory table (Appendix A), consumer reference matrix (Appendix B), and the three disposition lists.
- `project/Milestone_8-Facade-Cleanup-Shim-Resolution/Epic_1/prd-facade-crate-audit.md` - The PRD driving this task list. Reference for classification rules, required columns, and success metrics.
- `src/lib.rs` - The facade crate root. Read-only during this Epic. Each individual `pub use` line must be audited in Task 1.3.
- `src/application/ports/` - Expected to be all re-export shims pointing to `paladin-ports`. Audit in Tasks 2–4.
- `src/application/errors/` - Confirmed re-export shims (4 files). Audit in Tasks 2–4.
- `src/core/` - Expected to be mostly re-export shims pointing to `paladin-core`. Audit in Tasks 2–4.
- `src/application/notifications/` - Expected to contain misplaced channel services. Classify in Task 3.
- `src/application/storage/` - May contain port traits or implementations. Classify in Task 3.
- `src/infrastructure/adapters/` - May contain adapters not yet extracted in Milestone 7. Classify in Task 3.

### Notes

- This Epic is **read-only** — no source files in `src/` are modified. The only output is `facade-audit.md`.
- All shell commands should be run from the workspace root (`/workspace`).
- Do **not** run `cargo build` or `cargo check` as part of this Epic — the codebase is unchanged.
- LOC counts are approximate; use `wc -l` or any convenient tool.
- When grepping for consumers, always search the full workspace: `src/ crates/ tests/ examples/ benches/`.

## Instructions for Completing Tasks

**IMPORTANT:** As you complete each task, you must check it off in this markdown file by changing `- [ ]` to `- [x]`. This helps track progress and ensures you don't skip any steps.

Example:

- `- [ ] 1.1 Read file` → `- [x] 1.1 Read file` (after completing)

Update the file after completing each sub-task, not just after completing an entire parent task.

## Tasks

- [x] 0.0 Create feature branch
  - [x] 0.1 Create and checkout a new branch: `git checkout -b feature/milestone_8-epic_1-facade-audit`
  - [x] 0.2 Confirm the branch is active with `git branch --show-current`

- [x] 1.0 Set up the audit document scaffold
  - [x] 1.1 Create `project/Milestone_8-Facade-Cleanup-Shim-Resolution/Epic_1/facade-audit.md` with the required document structure (all section headings, but no data yet):
    - `# Facade Crate Audit`
    - `## Summary` (placeholder counts)
    - `## List A — Files to Delete`
    - `## List B — Files to Move` (with placeholder sub-headings for each known target crate: `paladin-notifications`, `paladin-storage`, `paladin-ports`)
    - `## List C — Files That Stay`
    - `## Appendix A — Full Inventory Table` (with column headers: `| Path | LOC | Content Type | References | Disposition | Target Crate / Justification |`)
    - `## Appendix B — Consumer Reference Matrix` (with column headers: `| Shim File | Re-exported Path | Consumers (file:line) | Has Consumers? |`)
  - [x] 1.2 Run `find src/ -name "*.rs" | wc -l` and record the total count in the Summary section as "Total files audited: N (pending)"

- [x] 2.0 Enumerate and inventory all `src/` files (PRD Task 1.1)
  - [x] 2.1 Run `find src/ -name "*.rs" | sort` and capture the full sorted list of all `.rs` files
  - [x] 2.2 For each file, open it and record: **Path**, **LOC** (`wc -l`), **Content Type** (one of: `re-export shim`, `application service`, `infrastructure adapter`, `config module`, `binary entry point`, `test module`, `dead code`), and **References** (which external crates it imports from or re-exports)
  - [x] 2.3 Populate Appendix A rows for all files under `src/bin/` and `src/config/` (expected: binary entry points and config modules)
  - [x] 2.4 Populate Appendix A rows for all files under `src/application/cli/` (expected: application services / CLI support)
  - [x] 2.5 Populate Appendix A rows for all files under `src/application/use_cases/` (expected: application services)
  - [x] 2.6 Populate Appendix A rows for `src/application/ports/` — **FINDING: directory does NOT exist; already removed prior to Milestone 8; noted in Summary**
  - [x] 2.7 Populate Appendix A rows for all files under `src/application/errors/` — **FINDING: `planning_error.rs` and `prompt_error.rs` are real application-service error types, NOT shims; only `citadel_error.rs` and `handoff_error.rs` are shims**
  - [x] 2.8 Populate Appendix A rows for all files under `src/application/notifications/` — **FINDING: directory not declared in `application/mod.rs`; all 3 files are dead code**
  - [x] 2.9 Populate Appendix A rows for all files under `src/application/storage/` — **FINDING: `file_store.rs`, `key_store.rs`, `key_value_store.rs`, `nosql_store.rs` are comment-only stubs (dead); `sql_store.rs` and `user_store.rs` are active shims**
  - [x] 2.10 Populate Appendix A rows for all files under `src/core/` — **FINDING: `admin/` and `user/` sub-directories not declared in `manager/mod.rs`; 8 files are orphaned dead code**
  - [x] 2.11 Populate Appendix A rows for all files under `src/infrastructure/adapters/`
  - [x] 2.12 Populate Appendix A rows for all files under `src/infrastructure/repositories/`, `src/infrastructure/resilience/`, `src/infrastructure/security/`, and `src/infrastructure/web/`
  - [x] 2.13 Add the `src/lib.rs` row to Appendix A (crate root; individual `pub use` lines audited in Task 4)
  - [x] 2.14 Verify the row count in Appendix A matches `find src/ -name "*.rs" | wc -l` (189 rows confirmed); Summary updated

- [x] 3.0 Classify every file by disposition (PRD Task 1.2)
  - [x] 3.1 Apply disposition rules to every row in Appendix A
  - [x] 3.2 Classify all `src/application/ports/` files — **N/A: directory does not exist**
  - [x] 3.3 Classify all `src/application/errors/` files — `citadel_error.rs` and `handoff_error.rs`: stays (active shims); `planning_error.rs` and `prompt_error.rs`: stays (real types); `mod.rs`: stays
  - [x] 3.4 Classify `src/application/notifications/` — all 3 files dead (not in module tree) → **delete**
  - [x] 3.5 Classify `src/application/storage/` files — `sql_store.rs` and `user_store.rs`: stays (active shims to `paladin-ports`); 4 comment-only stubs: **delete**
  - [x] 3.6 Classify all `src/core/` files — bridge shims and real services stay; 8 orphaned files deleted
  - [x] 3.7 Classify all `src/infrastructure/adapters/` files
  - [x] 3.8 Classify `src/infrastructure/resilience/` files — `circuit_breaker.rs`: stays (no target crate); `mod.rs`: stays
  - [x] 3.9 Classify `src/infrastructure/security/` files — facade-level utilities; all stay
  - [x] 3.10 Every row in Appendix A has a non-empty Disposition column
  - [x] 3.11 Build **List B — Files to Move** (13 files, grouped by target crate)
  - [x] 3.12 Build **List C — Files That Stay** (151 files)
  - [x] 3.13 Build **List A — Files to Delete** (25 files)

- [x] 4.0 Validate shim consumer references workspace-wide (PRD Task 1.3)
  - [x] 4.1 Identified re-exported paths for every shim file in Appendix A
  - [x] 4.2 Ran grep for each re-exported path; recorded consumers
  - [x] 4.3 Audited `src/application/ports/` shims — **N/A: directory does not exist**
  - [x] 4.4 Audited `src/application/errors/` shims — results in Appendix B Section 1
  - [x] 4.5 Audited all `src/core/` shims — results in Appendix B Section 1
  - [x] 4.6 Audited all remaining shim files — results in Appendix B Section 1
  - [x] 4.7 Audited each individual `pub use` line in `src/lib.rs` — results in Appendix B Section 2; finding: workspace code uses full paths; only LLM adapters have 13 short-path consumers
  - [x] 4.8 No shim files flipped to delete (all shims that stayed have active consumers or are preserved public API)
  - [x] 4.9 All Appendix B rows with consumers confirmed as stays in List C
  - [x] 4.10 Dead `lib.rs` re-export lines noted in Appendix B Section 2 (0 workspace consumers; public API, no deletion yet)

- [x] 5.0 Finalize lists and verify audit completeness
  - [x] 5.1 Review List A — all 25 entries have reasons
  - [x] 5.2 Review List B — all 13 entries specify existing target crates
  - [x] 5.3 Review List C — entries have justifications in Appendix A
  - [x] 5.4 25 + 13 + 151 = 189 ✓ — complete coverage confirmed
  - [x] 5.5 Summary section updated with final counts
  - [x] 5.6 Appendix B has rows for all shim files and all `lib.rs` `pub use` lines
  - [x] 5.7 PRD open questions answered: `resilience/` stays (no target crate); `security/` stays (facade utilities); `errors/` correction documented in Summary
  - [x] 5.8 Checked `benches/` — no imports of List A paths found; no bench dependencies on dead-code paths

- [ ] 6.0 Verify no source files were modified
  - [ ] 6.1 Run `git status` and confirm the only new/modified file is `project/Milestone_8-Facade-Cleanup-Shim-Resolution/Epic_1/facade-audit.md` (and this task list)
  - [ ] 6.2 Run `git diff --stat src/` and confirm zero changes to any `src/` file
  - [ ] 6.3 Stage and commit: `git add project/Milestone_8-Facade-Cleanup-Shim-Resolution/Epic_1/` and commit with message `docs(milestone-8): complete Epic 1 facade crate audit`
