## Relevant Files

*(To be populated after sub-tasks are generated)*

### Notes

- This Epic is **destructive** — files are deleted, not moved or modified. Always verify `cargo build --workspace` succeeds after each batch before proceeding to the next.
- The primary reference document for all file lists is `project/Milestone_8-Facade-Cleanup-Shim-Resolution/Epic_1/facade-audit.md` (List A).
- Run all shell commands from the workspace root (`/workspace`).
- The PRD for this Epic is `project/Milestone_8-Facade-Cleanup-Shim-Resolution/Epic_2/prd-remove-dead-shims-empty-modules.md`.

## Instructions for Completing Tasks

**IMPORTANT:** As you complete each task, you must check it off in this markdown file by changing `- [ ]` to `- [x]`. This helps track progress and ensures you don't skip any steps.

Example:

- `- [ ] 1.1 Read file` → `- [x] 1.1 Read file` (after completing)

Update the file after completing each sub-task, not just after completing an entire parent task.

## Tasks

- [ ] 0.0 Create feature branch
- [ ] 1.0 Pre-deletion review of `email_notifications.rs`
- [ ] 2.0 Delete application-layer dead files (Batches 1–4) and verify build after each
- [ ] 3.0 Delete orphaned `core/platform/manager/admin/` and `user/` directories (Batches 5–6) and verify build
- [ ] 4.0 Delete infrastructure empty stubs (Batch 7) and verify build
- [ ] 5.0 Audit and remediate stale `crate::application::ports::` import references
- [ ] 6.0 Verify `src/core/` is reduced to its minimum 6-file structure
- [ ] 7.0 Remove dead `pub use` lines from `src/lib.rs` and update STABLE_API.md / CHANGELOG.md
- [ ] 8.0 Run full quality gate and commit
