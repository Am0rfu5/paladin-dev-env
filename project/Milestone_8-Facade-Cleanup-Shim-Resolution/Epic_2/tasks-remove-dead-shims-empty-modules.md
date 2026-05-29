## Relevant Files

### Files to be **deleted** (25 List A + 1 cascade `mod.rs`)

- `src/application/notifications/email_notifications.rs` — Batch 1: orphaned; review for Epic 3 content before deleting
- `src/application/notifications/push_notifications.rs` — Batch 1: empty + orphaned
- `src/application/notifications/system_notifications.rs` — Batch 1: empty + orphaned
- `src/application/storage/file_store.rs` — Batch 2: comment-only stub
- `src/application/storage/key_store.rs` — Batch 2: comment-only stub
- `src/application/storage/key_value_store.rs` — Batch 2: comment-only stub
- `src/application/storage/nosql_store.rs` — Batch 2: comment-only stub
- `src/application/use_cases/content/content_list_ingestion_service.rs` — Batch 3: empty file
- `src/application/use_cases/content/content_list_service.rs` — Batch 3: empty file
- `src/application/use_cases/content/content_ml_analysis_service.rs` — Batch 3: empty file
- `src/application/use_cases/subject/subject.rs` — Batch 4: comment-only stub
- `src/application/use_cases/subject/subject_build_service.rs` — Batch 4: empty (single newline)
- `src/application/use_cases/subject/subject_search_service.rs` — Batch 4: empty (single newline)
- `src/application/use_cases/subject/subject_service.rs` — Batch 4: empty (single newline)
- `src/application/use_cases/subject/subject_tagging_service.rs` — Batch 4: empty (single newline)
- `src/application/use_cases/subject/mod.rs` — Batch 4 **cascade**: becomes childless after stubs deleted; not a List A item
- `src/core/platform/manager/admin/mod.rs` — Batch 5: orphaned directory root
- `src/core/platform/manager/admin/admin_console_service.rs` — Batch 5: comment-only stub; orphaned
- `src/core/platform/manager/admin/admin_logging_service.rs` — Batch 5: comment-only stub; orphaned
- `src/core/platform/manager/admin/admin_notification_service.rs` — Batch 5: comment-only stub; orphaned
- `src/core/platform/manager/user/mod.rs` — Batch 6: orphaned directory root
- `src/core/platform/manager/user/user_account_service.rs` — Batch 6: empty + orphaned
- `src/core/platform/manager/user/user_notification_service.rs` — Batch 6: references non-existent paths; orphaned
- `src/core/platform/manager/user/user_settings_service.rs` — Batch 6: empty + orphaned
- `src/infrastructure/adapters/logs/access_log_adapter.rs` — Batch 7: empty (single newline)
- `src/infrastructure/adapters/notifications/push_notification_adapter.rs` — Batch 7: empty (single newline)

### Files to be **modified** (mod.rs declaration cleanup + API files)

- `src/application/storage/mod.rs` — Batch 2: remove 4 `pub mod` declarations for deleted stubs
- `src/application/use_cases/content/mod.rs` — Batch 3: remove 3 `pub mod` declarations for deleted empties
- `src/application/use_cases/mod.rs` — Batch 4: remove `pub mod subject;` (cascade from subject dir deletion)
- `src/infrastructure/adapters/logs/mod.rs` — Batch 7: remove `pub mod access_log_adapter;`
- `src/infrastructure/adapters/notifications/mod.rs` — Batch 7: remove `push_notification_adapter` declaration
- `src/lib.rs` — Task 7.0: remove all zero-consumer `pub use` short-path aliases
- `STABLE_API.md` — Task 7.0: update to reflect removed aliases (v0.2.0 breaking change)
- `CHANGELOG.md` — Task 7.0: add `### Removed` section under v0.2.0 with crate-level replacement paths

### Reference files (read-only)

- `project/Milestone_8-Facade-Cleanup-Shim-Resolution/Epic_1/facade-audit.md` — primary reference: List A (files to delete) and Appendix B Section 2 (zero-consumer `pub use` lines)
- `project/Milestone_8-Facade-Cleanup-Shim-Resolution/Epic_2/prd-remove-dead-shims-empty-modules.md` — PRD for this Epic

### Notes

- This Epic is **destructive** — files are deleted, not moved or modified. Always verify `cargo build --workspace` succeeds after each batch before proceeding to the next.
- **Batch 4 critical constraint**: The cascade deletion of `subject/mod.rs` AND the removal of `pub mod subject;` from `use_cases/mod.rs` **must** happen in the same operation before running `cargo build --workspace`. If `subject/mod.rs` is left with dangling module declarations, the build will fail.
- **Orphaned dirs (Batches 1, 5, 6)**: The parent `mod.rs` files (`application/mod.rs`, `core/platform/manager/mod.rs`) never declared these directories — no parent `mod.rs` update is needed for these batches.
- Run all shell commands from the workspace root (`/workspace`).
- Success metric: `find src/ -name "*.rs" | wc -l` → **163** after all deletions (189 − 26: 25 List A + 1 cascade `mod.rs`).
- Task 7.0 CHANGELOG replacement paths must reference stable crate locations (`paladin_ports::`, `paladin_core::`, `paladin_battalion::`) — not facade-internal paths that will change in Epic 4.

## Instructions for Completing Tasks

**IMPORTANT:** As you complete each task, you must check it off in this markdown file by changing `- [ ]` to `- [x]`. This helps track progress and ensures you don't skip any steps.

Example:

- `- [ ] 1.1 Read file` → `- [x] 1.1 Read file` (after completing)

Update the file after completing each sub-task, not just after completing an entire parent task.

## Tasks

- [x] 0.0 Create feature branch
  - [x] 0.1 Run `git checkout -b feature/milestone_8-epic_2-remove-dead-shims` from the workspace root
  - [x] 0.2 Confirm with `git branch --show-current` — output must be `feature/milestone_8-epic_2-remove-dead-shims`

- [x] 1.0 Pre-deletion review of `email_notifications.rs`
  - [x] 1.1 Open and read `src/application/notifications/email_notifications.rs` (392 LOC) — note any domain logic, types, or implementations that do not appear in the infrastructure adapter
  - [x] 1.2 Open and read `src/infrastructure/adapters/notifications/email_notification_adapter.rs` (752 LOC, List B — staying in this Epic) — compare against the application file for unique logic
  - [x] 1.3 If unique logic is found that the infrastructure adapter does not cover, copy `email_notifications.rs` to `project/Milestone_8-Facade-Cleanup-Shim-Resolution/Epic_2/email_notifications_review.rs` as a staging artifact for Epic 3
  - [x] 1.4 Document the result as a comment in this task: either "No unique logic found — safe to delete" or "Unique logic staged at Epic_2/email_notifications_review.rs for Epic 3"
  <!-- RESULT: Unique logic staged at Epic_2/email_notifications_review.rs for Epic 3.
       email_notifications.rs is an APPLICATION SERVICE (EmailNotificationService, EmailNotificationServiceFactory,
       EmailRequest, EmailAttachment, EmailTemplate, EmailServiceCapabilities types + tests) that orchestrates
       the NotificationDeliveryPort and NotificationTemplatePort. This logic is NOT present in
       email_notification_adapter.rs, which is a pure SMTP infrastructure adapter (lettre + Handlebars).
       Epic 3 should evaluate whether to move the application service to paladin-notifications alongside
       the infrastructure adapter, or house it in a separate application-layer module. -->

- [x] 2.0 Delete application-layer dead files (Batches 1–4) and verify build after each
  - [x] 2.1 **Batch 1** — Delete 3 orphaned notification files: `rm src/application/notifications/email_notifications.rs src/application/notifications/push_notifications.rs src/application/notifications/system_notifications.rs`
  - [x] 2.2 **Batch 1** — Delete now-empty directory: `rmdir src/application/notifications/` (no `application/mod.rs` update needed — it never declared this dir)
  - [x] 2.3 **Batch 1** — `cargo build --workspace` — confirm exit code 0 and no new errors
  - [x] 2.4 **Batch 1** — `git add -A && git commit -m "refactor(m8-e2): delete orphaned src/application/notifications/ directory"`
  - [x] 2.5 **Batch 2** — Delete 4 comment-only storage stubs: `rm src/application/storage/file_store.rs src/application/storage/key_store.rs src/application/storage/key_value_store.rs src/application/storage/nosql_store.rs`
  - [x] 2.6 **Batch 2** — Update `src/application/storage/mod.rs`: remove the `pub mod file_store;`, `pub mod key_store;`, `pub mod key_value_store;`, and `pub mod nosql_store;` declaration lines (keep `sql_store` and `user_store`)
  - [x] 2.7 **Batch 2** — `cargo build --workspace` — confirm exit code 0 and no new errors
  - [x] 2.8 **Batch 2** — `git add -A && git commit -m "refactor(m8-e2): delete comment-only storage stubs"`
  - [x] 2.9 **Batch 3** — Delete 3 empty content use-case files: `rm src/application/use_cases/content/content_list_ingestion_service.rs src/application/use_cases/content/content_list_service.rs src/application/use_cases/content/content_ml_analysis_service.rs`
  - [x] 2.10 **Batch 3** — Update `src/application/use_cases/content/mod.rs`: remove the `pub mod content_list_ingestion_service;`, `pub mod content_list_service;`, and `pub mod content_ml_analysis_service;` lines (all other `pub mod` lines stay)
  <!-- NOTE: Batch 3 files were NOT declared in content/mod.rs — they were also orphans (never had pub mod entries). No mod.rs update was required. -->
  - [x] 2.11 **Batch 3** — `cargo build --workspace` — confirm exit code 0 and no new errors
  - [x] 2.12 **Batch 3** — `git add -A && git commit -m "refactor(m8-e2): delete empty content use_case placeholders"`
  - [x] 2.13 **Batch 4** — Delete 5 subject stubs: `rm src/application/use_cases/subject/subject.rs src/application/use_cases/subject/subject_build_service.rs src/application/use_cases/subject/subject_search_service.rs src/application/use_cases/subject/subject_service.rs src/application/use_cases/subject/subject_tagging_service.rs`
  - [x] 2.14 **Batch 4 (cascade)** — Delete `src/application/use_cases/subject/mod.rs` (now childless): `rm src/application/use_cases/subject/mod.rs`
  - [x] 2.15 **Batch 4 (cascade)** — Remove the empty directory: `rmdir src/application/use_cases/subject/`
  - [x] 2.16 **Batch 4 (cascade)** — Update `src/application/use_cases/mod.rs`: remove the `pub mod subject;` line (**must be done before the build check**)
  - [x] 2.17 **Batch 4** — `cargo build --workspace` — confirm exit code 0 and no new errors
  - [x] 2.18 **Batch 4** — `git add -A && git commit -m "refactor(m8-e2): delete empty subject use_case stubs and cascade mod.rs"`

- [x] 3.0 Delete orphaned `core/platform/manager/admin/` and `user/` directories (Batches 5–6) and verify build
  - [x] 3.1 **Batch 5** — Delete the entire `admin/` directory: `rm -r src/core/platform/manager/admin/` (no `manager/mod.rs` update needed — it never declared `pub mod admin;`)
  - [x] 3.2 **Batch 5** — `cargo build --workspace` — confirm exit code 0 and no new errors
  - [x] 3.3 **Batch 5** — `git add -A && git commit -m "refactor(m8-e2): delete orphaned core/platform/manager/admin/ directory"`
  - [x] 3.4 **Batch 6** — Delete the entire `user/` directory: `rm -r src/core/platform/manager/user/` (no `manager/mod.rs` update needed — it never declared `pub mod user;`)
  - [x] 3.5 **Batch 6** — `cargo build --workspace` — confirm exit code 0 and no new errors
  - [x] 3.6 **Batch 6** — `git add -A && git commit -m "refactor(m8-e2): delete orphaned core/platform/manager/user/ directory"`

- [x] 4.0 Delete infrastructure empty stubs (Batch 7) and verify build
  - [x] 4.1 **Batch 7** — Delete empty log adapter stub: `rm src/infrastructure/adapters/logs/access_log_adapter.rs`
  - [x] 4.2 **Batch 7** — Update `src/infrastructure/adapters/logs/mod.rs`: remove the `pub mod access_log_adapter;` line
  - [x] 4.3 **Batch 7** — Delete empty push notification adapter stub: `rm src/infrastructure/adapters/notifications/push_notification_adapter.rs`
  - [x] 4.4 **Batch 7** — Update `src/infrastructure/adapters/notifications/mod.rs`: remove the declaration line for `push_notification_adapter`
  <!-- NOTE: Only the #[cfg(not(feature="notifications"))] pub mod push_notification_adapter; line was removed.
       The #[cfg(feature="notifications")] pub use paladin_notifications::push_notification_adapter;
       re-export was kept as it references the paladin-notifications crate, not the deleted local file. -->
  - [x] 4.5 **Batch 7** — `cargo build --workspace` — confirm exit code 0 and no new errors
  - [x] 4.6 **Batch 7** — `git add -A && git commit -m "refactor(m8-e2): delete empty infrastructure adapter stubs"`

- [x] 5.0 Audit and remediate stale `crate::application::ports::` import references
  - [x] 5.1 Run `grep -rn "application::ports::" src/ crates/ tests/ examples/ benches/ --include="*.rs"` from the workspace root
  - [x] 5.2 For each match found, update the import to the correct path — typically `paladin_ports::` for port trait references, or the appropriate full module path
  - [x] 5.3 If any changes were made, run `cargo build --workspace` — confirm exit code 0
  - [x] 5.4 Document result in this task's notes: either "Zero stale `application::ports::` references found" or list the files changed
  <!-- RESULT: Zero live stale references found.
       All matches were in doc comments (/// or //!) — not live use/import statements:
       - crates/paladin-ports/src/output/arsenal_port.rs: doc comment examples (30+ occurrences)
       - crates/paladin-ports/src/output/sanctum_port.rs: doc comment examples
       - tests/unit/herald_consolidation_test.rs:22: single /// doc comment
       No code changes required. cargo build --workspace already green from Batch 7. -->
  - [x] 5.5 `git add -A && git commit -m "refactor(m8-e2): audit and clean stale application::ports:: references"`

- [ ] 6.0 Verify `src/core/` is reduced to its minimum 6-file structure
  - [ ] 6.1 Run `find src/core/ -name "*.rs" | sort` — confirm output contains exactly these 6 files: `src/core/mod.rs`, `src/core/platform/mod.rs`, `src/core/platform/manager/mod.rs`, `src/core/platform/manager/content_service.rs`, `src/core/platform/manager/event_manager.rs`, `src/core/platform/manager/user_service.rs`
  - [ ] 6.2 Read `src/core/platform/manager/mod.rs` — confirm it declares exactly 3 modules (`content_service`, `event_manager`, `user_service`) and nothing else
  - [ ] 6.3 Run `cargo test --workspace` — confirm all previously-passing tests still pass with zero new failures
  - [ ] 6.4 Document verification result in this task's notes; `git add -A && git commit -m "refactor(m8-e2): verify src/core/ minimum structure"` (even if only the task file is updated)

- [ ] 7.0 Remove dead `pub use` lines from `src/lib.rs` and update STABLE_API.md / CHANGELOG.md
  - [ ] 7.1 Open `src/lib.rs` alongside `facade-audit.md` Appendix B Section 2 (the consumer reference matrix)
  - [ ] 7.2 Identify every `pub use` line in `src/lib.rs` that has "Has Consumers? = No" in Appendix B — these are all candidates for removal
  - [ ] 7.3 For each candidate line, confirm the underlying type still exists at its source path in the leaf crates (we are removing only the alias, not the type itself)
  - [ ] 7.4 Edit `src/lib.rs` — remove all zero-consumer `pub use` lines; preserve the following 5 exceptions (confirmed consumers):
    - `pub use paladin_llm::mock::{MockLlmAdapter, MultiStepMockLlmPort};` (13 consumers)
    - `pub use paladin_llm::openai::{OpenAIAdapter, OpenAIConfig};`
    - `pub use paladin_llm::anthropic::{AnthropicAdapter, AnthropicConfig};`
    - `pub use paladin_llm::deepseek::{DeepSeekAdapter, DeepSeekConfig};`
    - `pub use core::platform::container::paladin::{Paladin, PaladinData, PaladinStatus};` (17 consumers)
    - Do **not** remove any `pub mod` declarations — only `pub use` lines
  - [ ] 7.5 `cargo build --workspace` — fix any compile errors surfaced (broken re-exports will appear here)
  - [ ] 7.6 `cargo test --workspace` — confirm no regression
  - [ ] 7.7 Update `STABLE_API.md`: remove the aliases section entries for each alias that was deleted
  - [ ] 7.8 Add `### Removed` section to `CHANGELOG.md` under the v0.2.0 entry — list each removed alias with its stable replacement path using **crate-level** references (`paladin_ports::`, `paladin_core::`, `paladin_battalion::`, `paladin_llm::`) — **not** facade-internal paths like `paladin::application::use_cases::...` (those paths will change in Epic 4)
  - [ ] 7.9 `git add -A && git commit -m "refactor(m8-e2): remove dead pub use lines from lib.rs"`

- [ ] 8.0 Run full quality gate and commit
  - [ ] 8.1 `cargo build --workspace` — exit code 0, zero errors
  - [ ] 8.2 `cargo test --workspace` — all tests pass, zero failures
  - [ ] 8.3 `cargo clippy --workspace -- -D warnings` — zero warnings
  - [ ] 8.4 `cargo fmt --all -- --check` — exit code 0 (no formatting drift)
  - [ ] 8.5 `find src/ -name "*.rs" | wc -l` — confirm output is **163** (189 − 26: 25 List A files + 1 cascade `subject/mod.rs`)
  - [ ] 8.6 Mark all tasks `[x]` in this file
  - [ ] 8.7 `git add project/Milestone_8-Facade-Cleanup-Shim-Resolution/Epic_2/ && git commit -m "docs(m8-e2): mark all Epic 2 tasks complete"`
