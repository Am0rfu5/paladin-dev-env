# Facade Crate Audit

**Epic:** 1 — Facade Crate Audit
**Milestone:** 8 — Facade Cleanup, Shim Resolution, and Directory Stabilization
**Status:** Complete
**Created:** 2026-05-29
**Completed:** 2026-05-29

---

## Summary

- **Total files audited:** 189
- **Files staying (List C):** 151
- **Files to move (List B):** 13
  - → `paladin-notifications`: 3
  - → `paladin-content`: 2
  - → `paladin-storage`: 6
  - → `paladin-memory`: 1
  - → `paladin-web`: 2 (wait for Epic 3 co-ordination — `paladin-web` already holds some of these)
- **Files to delete (List A):** 25
- **Key finding — `src/application/ports/`:** Directory does NOT exist; already removed prior to Milestone 8.
- **Key finding — `src/application/notifications/`:** Not declared in `src/application/mod.rs`; all 3 files are dead code.
- **Key finding — `src/core/platform/manager/admin/` and `user/`:** Neither sub-directory is declared in `manager/mod.rs`; all files are unreachable.
- **Key finding — lib.rs stable API:** Most `pub use` short-form aliases have 0 internal workspace consumers (workspace code uses full paths). Only `MockLlmAdapter`, `OpenAIAdapter`, `AnthropicAdapter`, `DeepSeekAdapter` have 13 consumers using short paths.
- **PRD correction — `src/application/errors/`:** `planning_error.rs` and `prompt_error.rs` are real application-service error types with tests — they are NOT shims and STAY. Only `citadel_error.rs` and `handoff_error.rs` are re-export shims.

---

## List A — Files to Delete

> Dead code: empty files, comment-only stubs, orphaned (not in module tree) files, and files using non-existent import paths. All to be removed in Epic 2.

| # | File | LOC | Reason |
|---|------|-----|--------|
| 1 | `src/application/notifications/email_notifications.rs` | 392 | Not declared in `application/mod.rs`; orphaned from module tree. Code has value — review for possible move to `paladin-notifications` before deletion. |
| 2 | `src/application/notifications/push_notifications.rs` | 0 | Empty file. |
| 3 | `src/application/notifications/system_notifications.rs` | 0 | Empty file. |
| 4 | `src/application/storage/file_store.rs` | 6 | Comment-only stub; no code; no consumers. |
| 5 | `src/application/storage/key_store.rs` | 21 | Comment-only stub; no code; no consumers. |
| 6 | `src/application/storage/key_value_store.rs` | 13 | Comment-only stub; no code; no consumers. |
| 7 | `src/application/storage/nosql_store.rs` | 5 | Comment-only stub; no code; no consumers. |
| 8 | `src/application/use_cases/content/content_list_ingestion_service.rs` | 0 | Empty file. |
| 9 | `src/application/use_cases/content/content_list_service.rs` | 0 | Empty file. |
| 10 | `src/application/use_cases/content/content_ml_analysis_service.rs` | 0 | Empty file. |
| 11 | `src/application/use_cases/subject/subject.rs` | 4 | Comment-only stub; not declared in `subject/mod.rs`; not in module tree. |
| 12 | `src/application/use_cases/subject/subject_build_service.rs` | 1 | Empty file (single newline). Declared in `subject/mod.rs` but has no content. |
| 13 | `src/application/use_cases/subject/subject_search_service.rs` | 1 | Empty file (single newline). Declared in `subject/mod.rs` but has no content. |
| 14 | `src/application/use_cases/subject/subject_service.rs` | 1 | Empty file (single newline). Declared in `subject/mod.rs` but has no content. |
| 15 | `src/application/use_cases/subject/subject_tagging_service.rs` | 1 | Empty file (single newline). Declared in `subject/mod.rs` but has no content. |
| 16 | `src/core/platform/manager/admin/mod.rs` | 2 | `admin/` directory not declared in `manager/mod.rs`; entire sub-tree is orphaned. |
| 17 | `src/core/platform/manager/admin/admin_console_service.rs` | 4 | Comment-only stub; orphaned (admin/ not in module tree). |
| 18 | `src/core/platform/manager/admin/admin_logging_service.rs` | 4 | Comment-only stub; orphaned (admin/ not in module tree). |
| 19 | `src/core/platform/manager/admin/admin_notification_service.rs` | 4 | Comment-only stub; orphaned (admin/ not in module tree). |
| 20 | `src/core/platform/manager/user/mod.rs` | 1 | `user/` directory not declared in `manager/mod.rs`; entire sub-tree is orphaned. |
| 21 | `src/core/platform/manager/user/user_account_service.rs` | 0 | Empty file; orphaned. |
| 22 | `src/core/platform/manager/user/user_notification_service.rs` | 12 | Uses non-existent `crate::core::domain::entities::*` paths; also orphaned (user/ not in module tree). |
| 23 | `src/core/platform/manager/user/user_settings_service.rs` | 0 | Empty file; orphaned. |
| 24 | `src/infrastructure/adapters/logs/access_log_adapter.rs` | 1 | Empty file (single newline). |
| 25 | `src/infrastructure/adapters/notifications/push_notification_adapter.rs` | 1 | Empty file (single newline). |

**Total: 25 files** (~1,471 lines of dead/comment-only code removed)

> ⚠️ **Note:** After deleting rows 12–15, `src/application/use_cases/subject/mod.rs` becomes an empty `pub mod` declaration with no live children — cascade-delete in the same PR. Similarly, after deleting rows 17–19, `src/core/platform/manager/admin/mod.rs` (row 16) cascades automatically. After rows 21–23, `src/core/platform/manager/user/mod.rs` (row 20) cascades automatically.

---

## List B — Files to Move

> Infrastructure adapter implementations whose domain belongs in an already-existing leaf crate. Moves to be executed in Epic 3.

### → `paladin-notifications`

| File | LOC | Notes |
|------|-----|-------|
| `src/infrastructure/adapters/notifications/email_notification_adapter.rs` | 752 | Full email notification adapter implementation. |
| `src/infrastructure/adapters/notifications/system_notification_adapter.rs` | 320 | System notification adapter implementation. |

> Also review `src/application/notifications/email_notifications.rs` (List A, row 1) before deletion — it may belong here too once the module tree issue is resolved.

### → `paladin-content`

| File | LOC | Notes |
|------|-----|-------|
| `src/infrastructure/adapters/document/document_adapter.rs` | 480 | Local copy; `paladin-content` already has this adapter. Remove duplicate after verifying `paladin_content::adapters::document::DocumentAdapter` is equivalent. |
| `src/infrastructure/adapters/document/pdf_extractor.rs` | 350 | PDF extraction belongs in the content domain. |

### → `paladin-storage`

| File | LOC | Notes |
|------|-----|-------|
| `src/infrastructure/adapters/file_storage/minio.rs` | 1,198 | MinIO/S3 storage implementation. |
| `src/infrastructure/adapters/queue/redis.rs` | 1,570 | Redis queue implementation. |
| `src/infrastructure/repositories/file_content_repository.rs` | 723 | File-backed content repository. |
| `src/infrastructure/repositories/mysql_content_repository.rs` | 780 | MySQL content repository. |
| `src/infrastructure/repositories/sqlite_content_repository.rs` | 810 | SQLite content repository. |
| `src/infrastructure/repositories/sqlite_user_repository.rs` | 676 | SQLite user repository. |

### → `paladin-memory`

| File | LOC | Notes |
|------|-----|-------|
| `src/infrastructure/adapters/citadel/file_citadel.rs` | 581 | File-based Citadel state persistence. Memory/state domain belongs in `paladin-memory`. |

### → `paladin-web`

| File | LOC | Notes |
|------|-----|-------|
| `src/infrastructure/adapters/output/api_content_deliverer.rs` | 724 | API content delivery adapter; `paladin-web` already declares a shim for this via `output/mod.rs`. Verify before move. |
| `src/infrastructure/web/user_controller.rs` | 870 | User HTTP controller; web layer belongs in `paladin-web`. |

**Total: 13 files** (~8,564 lines to relocate)

---

## List C — Files That Stay

> Application services, config modules, binary entry points, test modules, and active bridge shims — all belong in the facade as the application assembly and composition root.

<details>
<summary>Click to expand (151 files)</summary>

### `src/application/cli/` — CLI Application Services (35 files)

All files in this directory contain CLI commands, formatters, config loaders, interactive prompts, and templates. These are facade-specific application services and stay.

`commands/agent.rs`, `commands/arsenal.rs`, `commands/battalion.rs`, `commands/council.rs`, `commands/features.rs`, `commands/maneuver.rs`, `commands/mod.rs`, `commands/muster.rs`, `commands/onboarding.rs`, `commands/setup_check.rs`, `commands/user.rs`, `config/battalion_config.rs`, `config/loader.rs`, `config/mod.rs`, `config/paladin_config.rs`, `error.rs`, `error_impl.rs`, `formatters/mod.rs`, `formatters/output.rs`, `formatters/progress.rs`, `formatters/table.rs`, `formatters/tests.rs`, `interactive/mod.rs`, `interactive/prompts.rs`, `interactive/tests.rs`, `interactive/utils.rs`, `interactive/wizard.rs`, `mod.rs`, `templates/battalion_template.rs`, `templates/env.rs`, `templates/mod.rs`, `templates/paladin_template.rs`, `tests/command_tests.rs`, `tests/formatter_tests.rs`, `tests/mod.rs`

### `src/application/errors/` — Error Types (5 files)

`citadel_error.rs` (shim, 3 consumers), `handoff_error.rs` (shim, 1 consumer), `mod.rs` (decl), `planning_error.rs` (real error type), `prompt_error.rs` (real error type)

### `src/application/` — Top-level (1 file)

`mod.rs` — module declarations + architecture docs

### `src/application/storage/` — Partial (3 of 7 files stay)

`mod.rs` (decl), `sql_store.rs` (shim, 4 consumers), `user_store.rs` (shim, 2 consumers)

### `src/application/use_cases/` — Use Case Services (42 files)

All use-case service files except the dead stubs. Includes: `analysis/`, `arsenal/`, `battalion/mod.rs` (shim), `content/` (active files), `herald/`, `log_orchestrator/`, `mod.rs`, `notification_orchestrator/`, `orchestration/`, `paladin/`, `queue_orchestrator/`, `sanctum/mod.rs` (shim), `subject/mod.rs` (cascade-delete candidate)

### `src/bin/` (1 file)

`paladin-cli.rs` — binary entry point

### `src/config/` (14 files)

All config files: `arsenal.rs`, `citadel.rs`, `env_utils.rs`, `file_storage.rs`, `herald.rs`, `mod.rs`, `notifications.rs`, `queue.rs`, `scheduler.rs`, `settings.rs`, `setup/mod.rs`, `setup/service_runner.rs`, `user_config.rs`, `web_server.rs`

### `src/core/` (6 files)

`mod.rs` (bridge shim, 275+ consumers), `platform/mod.rs` (bridge shim w/ real maneuver injection logic), `platform/manager/mod.rs` (decl), `platform/manager/content_service.rs` (app svc), `platform/manager/event_manager.rs` (app svc), `platform/manager/user_service.rs` (app svc)

### `src/infrastructure/adapters/` — Local Adapters That Stay (31 files)

Arsenal: `mcp_protocol.rs`, `mcp_sse_adapter.rs`, `mcp_stdio_adapter.rs`, `mod.rs`, `resource_controls.rs`, `tool_result_formatter.rs`

Citadel: `mod.rs` *(after file_citadel.rs moves, this mod.rs may need updating)*

Document: `mod.rs` (shim, 3 consumers) *(local document_adapter.rs and pdf_extractor.rs move to paladin-content)*

File storage: `mod.rs` *(after minio.rs moves)*

Garrison: `mod.rs` (shim, many consumers)

Herald: `json_herald.rs`, `markdown_herald.rs`, `mod.rs`, `table_herald.rs`

Input: `file_content_fetcher.rs`, `file_content_list_fetcher.rs`, `http_content_fetcher.rs`, `local_file_fetcher.rs`, `mod.rs` (bridge), `news_api_fetcher.rs`, `tensorflow_adapter.rs`

LLM: `config_bridge.rs`, `mod.rs`

Logs: `error_log_adapter.rs`, `mod.rs`, `system_log_adapter.rs`

`adapters/mod.rs`, `paladin_registry.rs`

Notifications: `mod.rs` (bridge)

Output: `mod.rs` (shim, 1 consumer)

Queue: `mod.rs`

Sanctum: `mod.rs` (shim, multiple consumers)

Scheduling: `mod.rs`, `tokio_cron_adapter.rs`

### `src/infrastructure/` — Other Infra (11 files)

`mod.rs` (decl+docs), `repositories/mod.rs` (conditional bridge), `resilience/circuit_breaker.rs`, `resilience/mod.rs`, `security/audit.rs`, `security/encryption.rs`, `security/mod.rs`, `security/tls_verification.rs`, `web/mod.rs` (shim — no current workspace consumers but preserves web feature path)

### `src/` — Root (3 files)

`lib.rs` (crate root + stable API), `main.rs` (binary entry), `prelude.rs` (convenience re-exports — 0 workspace consumers but part of public API)

</details>

---

## Appendix A — Full Inventory Table

| Path | LOC | Content Type | Re-exports / References | Disposition |
|------|-----|--------------|------------------------|-------------|
| `src/application/cli/commands/agent.rs` | 743 | application service | CLI agent commands | stays |
| `src/application/cli/commands/arsenal.rs` | 548 | application service | CLI arsenal commands | stays |
| `src/application/cli/commands/battalion.rs` | 1,266 | application service | CLI battalion commands | stays |
| `src/application/cli/commands/council.rs` | 456 | application service | CLI council commands | stays |
| `src/application/cli/commands/features.rs` | 573 | application service | CLI feature flags commands | stays |
| `src/application/cli/commands/maneuver.rs` | 318 | application service | CLI maneuver commands | stays |
| `src/application/cli/commands/mod.rs` | 15 | module declaration | declares command modules | stays |
| `src/application/cli/commands/muster.rs` | 548 | application service | CLI muster commands | stays |
| `src/application/cli/commands/onboarding.rs` | 768 | application service | CLI onboarding wizard | stays |
| `src/application/cli/commands/setup_check.rs` | 543 | application service | CLI setup-check commands | stays |
| `src/application/cli/commands/user.rs` | 1,048 | application service | CLI user management commands | stays |
| `src/application/cli/config/battalion_config.rs` | 467 | config module | CLI battalion configuration | stays |
| `src/application/cli/config/loader.rs` | 534 | config module | CLI config loader | stays |
| `src/application/cli/config/mod.rs` | 9 | module declaration | declares CLI config modules | stays |
| `src/application/cli/config/paladin_config.rs` | 651 | config module | CLI paladin configuration | stays |
| `src/application/cli/error.rs` | 136 | application service | CLI error types | stays |
| `src/application/cli/error_impl.rs` | 498 | application service | CLI error implementations | stays |
| `src/application/cli/formatters/mod.rs` | 12 | module declaration | declares formatter modules | stays |
| `src/application/cli/formatters/output.rs` | 570 | application service | CLI output formatters | stays |
| `src/application/cli/formatters/progress.rs` | 195 | application service | CLI progress formatters | stays |
| `src/application/cli/formatters/table.rs` | 337 | application service | CLI table formatters | stays |
| `src/application/cli/formatters/tests.rs` | 167 | test module | formatter unit tests | stays |
| `src/application/cli/interactive/mod.rs` | 9 | module declaration | declares interactive modules | stays |
| `src/application/cli/interactive/prompts.rs` | 283 | application service | CLI interactive prompts | stays |
| `src/application/cli/interactive/tests.rs` | 70 | test module | interactive unit tests | stays |
| `src/application/cli/interactive/utils.rs` | 150 | application service | CLI interactive utilities | stays |
| `src/application/cli/interactive/wizard.rs` | 319 | application service | CLI interactive wizard | stays |
| `src/application/cli/mod.rs` | 50 | module declaration | declares CLI module tree | stays |
| `src/application/cli/templates/battalion_template.rs` | 585 | application service | CLI battalion templates | stays |
| `src/application/cli/templates/env.rs` | 165 | application service | CLI env templates | stays |
| `src/application/cli/templates/mod.rs` | 5 | module declaration | declares template modules | stays |
| `src/application/cli/templates/paladin_template.rs` | 136 | application service | CLI paladin templates | stays |
| `src/application/cli/tests/command_tests.rs` | 241 | test module | CLI command tests | stays |
| `src/application/cli/tests/formatter_tests.rs` | 139 | test module | CLI formatter tests | stays |
| `src/application/cli/tests/mod.rs` | 50 | test module | CLI test module | stays |
| `src/application/errors/citadel_error.rs` | 6 | re-export shim | `paladin_core::...::CitadelError` | stays — 3 consumers |
| `src/application/errors/handoff_error.rs` | 6 | re-export shim | `crate::core::...::HandoffError` | stays — 1 consumer |
| `src/application/errors/mod.rs` | 4 | module declaration | declares error sub-modules | stays |
| `src/application/errors/planning_error.rs` | 169 | application service | `PlanningError` enum + tests (real type, not a shim) | stays |
| `src/application/errors/prompt_error.rs` | 161 | application service | `PromptError` enum + tests (real type, not a shim) | stays |
| `src/application/mod.rs` | 145 | module declaration | application layer module declarations + architecture docs | stays |
| `src/application/notifications/email_notifications.rs` | 392 | dead code | not declared in `application/mod.rs`; unreachable from module tree | **delete** |
| `src/application/notifications/push_notifications.rs` | 0 | dead code | empty file | **delete** |
| `src/application/notifications/system_notifications.rs` | 0 | dead code | empty file | **delete** |
| `src/application/storage/file_store.rs` | 6 | dead code | comment-only stub; zero consumers | **delete** |
| `src/application/storage/key_store.rs` | 21 | dead code | comment-only stub; zero consumers | **delete** |
| `src/application/storage/key_value_store.rs` | 13 | dead code | comment-only stub; zero consumers | **delete** |
| `src/application/storage/mod.rs` | 6 | module declaration | declares storage sub-modules | stays |
| `src/application/storage/nosql_store.rs` | 5 | dead code | comment-only stub; zero consumers | **delete** |
| `src/application/storage/sql_store.rs` | 13 | re-export shim | `paladin_ports::output::repository_port::*` | stays — 4 consumers |
| `src/application/storage/user_store.rs` | 7 | re-export shim | `paladin_ports::output::user_repository_port::UserRepositoryPort` | stays — 2 consumers |
| `src/application/use_cases/analysis/llm_analysis_service.rs` | 6 | re-export shim | `paladin_llm::llm_analysis_service::*` | stays — 2 consumers |
| `src/application/use_cases/analysis/mod.rs` | 1 | module declaration | declares analysis use-case modules | stays |
| `src/application/use_cases/arsenal/arsenal_execution_service.rs` | 127 | application service | arsenal execution logic | stays |
| `src/application/use_cases/arsenal/arsenal_registry_service.rs` | 129 | application service | arsenal registry logic | stays |
| `src/application/use_cases/arsenal/mod.rs` | 7 | module declaration | declares arsenal use-case modules | stays |
| `src/application/use_cases/battalion/mod.rs` | 31 | re-export shim | `paladin_battalion::*` (+ compat sub-module aliases) | stays — many consumers in tests/examples |
| `src/application/use_cases/content/content_aggregator_service.rs` | 164 | application service | content aggregation orchestration | stays |
| `src/application/use_cases/content/content_analysis_service.rs` | 55 | application service | content analysis orchestration | stays |
| `src/application/use_cases/content/content_delivery_service.rs` | 38 | application service | content delivery orchestration | stays |
| `src/application/use_cases/content/content_fetching_service.rs` | 207 | application service | content fetching orchestration | stays |
| `src/application/use_cases/content/content_filtering_service.rs` | 51 | application service | content filtering orchestration | stays |
| `src/application/use_cases/content/content_ingestion_service.rs` | 1,211 | application service | content ingestion pipeline | stays |
| `src/application/use_cases/content/content_list_fetching_service.rs` | 99 | application service | content list fetching orchestration | stays |
| `src/application/use_cases/content/content_list_ingestion_service.rs` | 0 | dead code | empty file | **delete** |
| `src/application/use_cases/content/content_list_service.rs` | 0 | dead code | empty file | **delete** |
| `src/application/use_cases/content/content_llm_analysis_service.rs` | 585 | application service | LLM-based content analysis | stays |
| `src/application/use_cases/content/content_ml_analysis_service.rs` | 0 | dead code | empty file | **delete** |
| `src/application/use_cases/content/content_nlp_analysis_service.rs` | 31 | application service | NLP-based content analysis | stays |
| `src/application/use_cases/content/content_summarizer_service.rs` | 474 | application service | content summarization orchestration | stays |
| `src/application/use_cases/content/mod.rs` | 14 | module declaration | declares content use-case modules | stays |
| `src/application/use_cases/herald/herald_registry.rs` | 514 | application service | herald registry service | stays |
| `src/application/use_cases/herald/mod.rs` | 3 | module declaration | declares herald use-case modules | stays |
| `src/application/use_cases/log_orchestrator/mod.rs` | 1,072 | application service | log orchestration service | stays |
| `src/application/use_cases/log_orchestrator/types.rs` | 154 | application service | log orchestration types | stays |
| `src/application/use_cases/mod.rs` | 12 | module declaration | declares top-level use-case modules | stays |
| `src/application/use_cases/notification_orchestrator/mod.rs` | 744 | application service | notification orchestration service | stays |
| `src/application/use_cases/notification_orchestrator/types.rs` | 140 | application service | notification orchestration types | stays |
| `src/application/use_cases/orchestration/listener.rs` | 538 | application service | orchestration listener | stays |
| `src/application/use_cases/orchestration/mod.rs` | 1,156 | application service | main orchestration service | stays |
| `src/application/use_cases/orchestration/scheduler.rs` | 856 | application service | orchestration scheduler | stays |
| `src/application/use_cases/orchestration/types.rs` | 188 | application service | orchestration types | stays |
| `src/application/use_cases/paladin/error.rs` | 5 | application service | Paladin-specific error types | stays |
| `src/application/use_cases/paladin/handoff_service.rs` | 610 | application service | Paladin handoff service | stays |
| `src/application/use_cases/paladin/mod.rs` | 12 | module declaration | declares paladin use-case modules | stays |
| `src/application/use_cases/paladin/paladin_builder.rs` | 2,325 | application service | `PaladinBuilder` fluent API | stays |
| `src/application/use_cases/paladin/paladin_execution_service.rs` | 2,529 | application service | `PaladinExecutionService` | stays |
| `src/application/use_cases/paladin/planning_service.rs` | 1,007 | application service | autonomous planning service | stays |
| `src/application/use_cases/paladin/prompt_generation_service.rs` | 476 | application service | prompt generation service | stays |
| `src/application/use_cases/paladin/temperature_service.rs` | 653 | application service | dynamic temperature service | stays |
| `src/application/use_cases/queue_orchestrator/mod.rs` | 484 | application service | queue orchestration service | stays |
| `src/application/use_cases/queue_orchestrator/types.rs` | 269 | application service | queue orchestration types | stays |
| `src/application/use_cases/sanctum/mod.rs` | 23 | re-export shim | `paladin_memory::services::*` (+ compat sub-module aliases) | stays — multiple consumers |
| `src/application/use_cases/subject/mod.rs` | 4 | module declaration | declares subject sub-modules (all empty — cascade-delete candidate) | stays* |
| `src/application/use_cases/subject/subject.rs` | 4 | dead code | comment-only stub; not declared in `subject/mod.rs` | **delete** |
| `src/application/use_cases/subject/subject_build_service.rs` | 1 | dead code | empty file; declared but no content | **delete** |
| `src/application/use_cases/subject/subject_search_service.rs` | 1 | dead code | empty file; declared but no content | **delete** |
| `src/application/use_cases/subject/subject_service.rs` | 1 | dead code | empty file; declared but no content | **delete** |
| `src/application/use_cases/subject/subject_tagging_service.rs` | 1 | dead code | empty file; declared but no content | **delete** |
| `src/bin/paladin-cli.rs` | 193 | binary entry point | CLI binary entry point (`paladin-cli`) | stays |
| `src/config/arsenal.rs` | 39 | config module | arsenal configuration types | stays |
| `src/config/citadel.rs` | 214 | config module | citadel configuration types | stays |
| `src/config/env_utils.rs` | 227 | config module | environment variable utilities | stays |
| `src/config/file_storage.rs` | 194 | config module | file storage configuration types | stays |
| `src/config/herald.rs` | 336 | config module | herald configuration types | stays |
| `src/config/mod.rs` | 52 | config module | config module declarations + settings | stays |
| `src/config/notifications.rs` | 82 | config module | notifications configuration types | stays |
| `src/config/queue.rs` | 84 | config module | queue configuration types | stays |
| `src/config/scheduler.rs` | 42 | config module | scheduler configuration types | stays |
| `src/config/settings.rs` | 333 | config module | top-level application settings | stays |
| `src/config/setup/mod.rs` | 43 | config module | setup module declarations | stays |
| `src/config/setup/service_runner.rs` | 775 | config module | service runner / DI composition root | stays |
| `src/config/user_config.rs` | 355 | config module | user configuration types | stays |
| `src/config/web_server.rs` | 31 | config module | web server configuration types | stays |
| `src/core/mod.rs` | 111 | re-export shim | `paladin_core::base` + `pub mod platform` | stays — 275+ workspace consumers |
| `src/core/platform/manager/admin/admin_console_service.rs` | 4 | dead code | comment-only stub; `admin/` not declared in `manager/mod.rs` | **delete** |
| `src/core/platform/manager/admin/admin_logging_service.rs` | 4 | dead code | comment-only stub; `admin/` not declared in `manager/mod.rs` | **delete** |
| `src/core/platform/manager/admin/admin_notification_service.rs` | 4 | dead code | comment-only stub; `admin/` not declared in `manager/mod.rs` | **delete** |
| `src/core/platform/manager/admin/mod.rs` | 2 | dead code | `admin/` directory not declared in `manager/mod.rs`; orphaned | **delete** |
| `src/core/platform/manager/content_service.rs` | 385 | application service | content management service (w/ versioning) | stays |
| `src/core/platform/manager/event_manager.rs` | 345 | application service | event manager service | stays |
| `src/core/platform/manager/mod.rs` | 3 | module declaration | declares `content_service`, `event_manager`, `user_service` | stays |
| `src/core/platform/manager/user/mod.rs` | 1 | dead code | `user/` directory not declared in `manager/mod.rs`; orphaned | **delete** |
| `src/core/platform/manager/user/user_account_service.rs` | 0 | dead code | empty file; orphaned | **delete** |
| `src/core/platform/manager/user/user_notification_service.rs` | 12 | dead code | uses non-existent `crate::core::domain::entities::*` paths; orphaned | **delete** |
| `src/core/platform/manager/user/user_settings_service.rs` | 0 | dead code | empty file; orphaned | **delete** |
| `src/core/platform/manager/user_service.rs` | 414 | application service | user authentication and management service (argon2 hashing) | stays |
| `src/core/platform/mod.rs` | 69 | re-export shim | `paladin_core::platform::container::*` + maneuver module injection | stays — bridge with real logic |
| `src/infrastructure/adapters/arsenal/mcp_protocol.rs` | 428 | infrastructure adapter | MCP protocol implementation | stays |
| `src/infrastructure/adapters/arsenal/mcp_sse_adapter.rs` | 299 | infrastructure adapter | MCP SSE transport adapter | stays |
| `src/infrastructure/adapters/arsenal/mcp_stdio_adapter.rs` | 236 | infrastructure adapter | MCP STDIO transport adapter | stays |
| `src/infrastructure/adapters/arsenal/mod.rs` | 10 | module declaration | declares arsenal adapter modules | stays |
| `src/infrastructure/adapters/arsenal/resource_controls.rs` | 347 | infrastructure adapter | MCP resource access controls | stays |
| `src/infrastructure/adapters/arsenal/tool_result_formatter.rs` | 467 | infrastructure adapter | MCP tool result formatter | stays |
| `src/infrastructure/adapters/citadel/file_citadel.rs` | 581 | infrastructure adapter | file-based Citadel state adapter | **move → paladin-memory** |
| `src/infrastructure/adapters/citadel/mod.rs` | 1 | module declaration | declares `file_citadel` module | stays |
| `src/infrastructure/adapters/document/document_adapter.rs` | 480 | infrastructure adapter | document adapter (duplicate — also in `paladin-content`) | **move → paladin-content** |
| `src/infrastructure/adapters/document/mod.rs` | 4 | re-export shim | `paladin_content::adapters::document::*` | stays — 3 consumers |
| `src/infrastructure/adapters/document/pdf_extractor.rs` | 350 | infrastructure adapter | PDF text extractor | **move → paladin-content** |
| `src/infrastructure/adapters/file_storage/minio.rs` | 1,198 | infrastructure adapter | MinIO/S3 file storage adapter | **move → paladin-storage** |
| `src/infrastructure/adapters/file_storage/mod.rs` | 2 | module declaration | feature-gated `minio` module declaration | stays |
| `src/infrastructure/adapters/garrison/mod.rs` | 25 | re-export shim | `paladin_memory::garrison::*` (+ compat sub-modules) | stays — many consumers |
| `src/infrastructure/adapters/herald/json_herald.rs` | 638 | infrastructure adapter | JSON herald output adapter | stays |
| `src/infrastructure/adapters/herald/markdown_herald.rs` | 672 | infrastructure adapter | Markdown herald output adapter | stays |
| `src/infrastructure/adapters/herald/mod.rs` | 12 | module declaration | declares herald adapter modules | stays |
| `src/infrastructure/adapters/herald/table_herald.rs` | 588 | infrastructure adapter | table herald output adapter | stays |
| `src/infrastructure/adapters/input/file_content_fetcher.rs` | 328 | infrastructure adapter | file-based content fetcher | stays |
| `src/infrastructure/adapters/input/file_content_list_fetcher.rs` | 218 | infrastructure adapter | file-based content list fetcher | stays |
| `src/infrastructure/adapters/input/http_content_fetcher.rs` | 169 | infrastructure adapter | HTTP content fetcher | stays |
| `src/infrastructure/adapters/input/local_file_fetcher.rs` | 14 | infrastructure adapter | local filesystem fetcher | stays |
| `src/infrastructure/adapters/input/mod.rs` | 11 | re-export shim | `paladin_content::adapters::*` (feature-gated) + local `tensorflow_adapter` | stays — conditional bridge |
| `src/infrastructure/adapters/input/news_api_fetcher.rs` | 527 | infrastructure adapter | news API content fetcher | stays |
| `src/infrastructure/adapters/input/tensorflow_adapter.rs` | 629 | infrastructure adapter | TensorFlow ML inference adapter | stays |
| `src/infrastructure/adapters/llm/config_bridge.rs` | 117 | infrastructure adapter | LLM config bridge (facade config → paladin-llm config) | stays |
| `src/infrastructure/adapters/llm/mod.rs` | 12 | module declaration | declares LLM adapter modules | stays |
| `src/infrastructure/adapters/logs/access_log_adapter.rs` | 1 | dead code | empty file | **delete** |
| `src/infrastructure/adapters/logs/error_log_adapter.rs` | 875 | infrastructure adapter | error log adapter | stays |
| `src/infrastructure/adapters/logs/mod.rs` | 14 | module declaration | declares log adapter modules + re-exports | stays |
| `src/infrastructure/adapters/logs/system_log_adapter.rs` | 622 | infrastructure adapter | system log adapter | stays |
| `src/infrastructure/adapters/mod.rs` | 17 | module declaration | top-level adapter module declarations | stays |
| `src/infrastructure/adapters/notifications/email_notification_adapter.rs` | 752 | infrastructure adapter | email notification adapter | **move → paladin-notifications** |
| `src/infrastructure/adapters/notifications/mod.rs` | 29 | re-export shim | `paladin_notifications::*` (feature-gated) + local module decls | stays — conditional bridge |
| `src/infrastructure/adapters/notifications/push_notification_adapter.rs` | 1 | dead code | empty file | **delete** |
| `src/infrastructure/adapters/notifications/system_notification_adapter.rs` | 320 | infrastructure adapter | system notification adapter | **move → paladin-notifications** |
| `src/infrastructure/adapters/output/api_content_deliverer.rs` | 724 | infrastructure adapter | API content deliverer (also exists in `paladin-web`) | **move → paladin-web** |
| `src/infrastructure/adapters/output/mod.rs` | 2 | re-export shim | `paladin_web::adapters::api_content_deliverer` (feature-gated) | stays — 1 consumer |
| `src/infrastructure/adapters/paladin_registry.rs` | 418 | infrastructure adapter | Paladin registry adapter | stays |
| `src/infrastructure/adapters/queue/mod.rs` | 2 | module declaration | feature-gated `redis` module declaration | stays |
| `src/infrastructure/adapters/queue/redis.rs` | 1,570 | infrastructure adapter | Redis queue adapter | **move → paladin-storage** |
| `src/infrastructure/adapters/sanctum/mod.rs` | 9 | re-export shim | `paladin_memory::sanctum::*` | stays — multiple consumers |
| `src/infrastructure/adapters/scheduling/mod.rs` | 8 | module declaration | declares scheduling adapter modules | stays |
| `src/infrastructure/adapters/scheduling/tokio_cron_adapter.rs` | 420 | infrastructure adapter | Tokio-based cron scheduler | stays |
| `src/infrastructure/mod.rs` | 179 | module declaration | infrastructure layer module declarations + architecture docs | stays |
| `src/infrastructure/repositories/file_content_repository.rs` | 723 | infrastructure adapter | file-backed content repository | **move → paladin-storage** |
| `src/infrastructure/repositories/mod.rs` | 14 | re-export shim | `paladin_storage::*` (feature-gated) + local fallback module decls | stays — conditional bridge |
| `src/infrastructure/repositories/mysql_content_repository.rs` | 780 | infrastructure adapter | MySQL content repository | **move → paladin-storage** |
| `src/infrastructure/repositories/sqlite_content_repository.rs` | 810 | infrastructure adapter | SQLite content repository | **move → paladin-storage** |
| `src/infrastructure/repositories/sqlite_user_repository.rs` | 676 | infrastructure adapter | SQLite user repository | **move → paladin-storage** |
| `src/infrastructure/resilience/circuit_breaker.rs` | 460 | infrastructure adapter | circuit breaker (no target crate exists) | stays |
| `src/infrastructure/resilience/mod.rs` | 24 | module declaration | declares resilience modules | stays |
| `src/infrastructure/security/audit.rs` | 431 | infrastructure adapter | security audit (Sentinel Vision system) | stays |
| `src/infrastructure/security/encryption.rs` | 455 | infrastructure adapter | encryption utilities | stays |
| `src/infrastructure/security/mod.rs` | 44 | module declaration | declares security modules + Sentinel re-exports | stays |
| `src/infrastructure/security/tls_verification.rs` | 72 | infrastructure adapter | TLS certificate verification | stays |
| `src/infrastructure/web/mod.rs` | 2 | re-export shim | `paladin_web::*` (feature-gated) | stays — no current workspace consumers; preserves web feature API path |
| `src/infrastructure/web/user_controller.rs` | 870 | infrastructure adapter | user HTTP controller | **move → paladin-web** |
| `src/lib.rs` | 267 | module declaration | crate root + stable public API re-exports | stays — see Appendix B |
| `src/main.rs` | 61 | binary entry point | application binary entry point | stays |
| `src/prelude.rs` | 59 | re-export shim | `crate::*` convenience re-exports (public API) | stays — 0 current workspace consumers; public API convenience module |

*`subject/mod.rs` marked stays for now; cascade-delete once children (rows 12–15) are removed in Epic 2.

---

## Appendix B — Consumer Reference Matrix

### Section 1 — Re-export Shim Files

> Each shim file with its re-export target and workspace consumer files.

| Shim File | Re-exported From | Consumer Files | Active? |
|-----------|-----------------|----------------|---------|
| `src/application/errors/citadel_error.rs` | `paladin_core::platform::container::citadel_error::CitadelError` | `paladin_builder.rs`, `file_citadel.rs`, `lib.rs` | ✅ yes (3 consumers) |
| `src/application/errors/handoff_error.rs` | `crate::core::platform::container::arsenal::handoff_error::HandoffError` | `handoff_service.rs` | ✅ yes (1 consumer) |
| `src/application/storage/sql_store.rs` | `paladin_ports::output::repository_port::{ContentListRepository, ContentRepository, MigrationManager, RepositoryError, RepositoryStats, SqlStore, TransactionManager}` | `sqlite_content_repository.rs`, `mysql_content_repository.rs`, `service_runner.rs`, `mysql_content_repository_test.rs` | ✅ yes (4 consumers) |
| `src/application/storage/user_store.rs` | `paladin_ports::output::user_repository_port::UserRepositoryPort` | `sqlite_user_repository.rs`, `user_service.rs` | ✅ yes (2 consumers) |
| `src/application/use_cases/analysis/llm_analysis_service.rs` | `paladin_llm::llm_analysis_service::*` | `content_llm_analysis_service.rs`, `content_llm_analysis_pipeline_test.rs` | ✅ yes (2 consumers) |
| `src/application/use_cases/battalion/mod.rs` | `paladin_battalion::{campaign_service, chain_of_command_service, commander, conclave_execution_service, council_service, error_aggregation, formation_service, grove_service, maneuver, phalanx_service, retry}` + compat sub-modules | `mod.rs`, `cli/commands/maneuver.rs`, `cli/commands/battalion.rs`, `lib.rs`, 10+ integration tests, 2+ CLI tests, many examples | ✅ yes (many) |
| `src/application/use_cases/sanctum/mod.rs` | `paladin_memory::services::{ExtractedMemory, MemoryExtractionService, MemoryExtractionStrategy, RagConfig, RagRetrievalService, RetrievalTrigger, retrieve_context_with_timeout}` + compat sub-modules | `paladin_builder.rs`, `paladin_execution_service.rs`, `rag_integration_tests.rs`, `paladin_with_rag.rs` | ✅ yes (4+ consumers) |
| `src/infrastructure/adapters/document/mod.rs` | `paladin_content::adapters::document::{DocumentAdapter, PdfExtractor}` | `cli/commands/agent.rs`, `document_adapter.rs`, `document_processing.rs` | ✅ yes (3 consumers) |
| `src/infrastructure/adapters/garrison/mod.rs` | `paladin_memory::garrison::*` + compat sub-modules | `cli/config/loader.rs`, `infrastructure/mod.rs`, 4+ integration tests, 3 examples | ✅ yes (many) |
| `src/infrastructure/adapters/input/mod.rs` | `paladin_content::adapters::*` (feature-gated) + local `tensorflow_adapter` | `infrastructure/adapters/input/news_api_fetcher.rs` | ✅ yes (conditional bridge) |
| `src/infrastructure/adapters/notifications/mod.rs` | `paladin_notifications::*` (when `notifications` feature active) + local `email_notification_adapter`, `system_notification_adapter` | `config/notifications.rs`, `notification_system_integration_test.rs` | ✅ yes (2 consumers) |
| `src/infrastructure/adapters/output/mod.rs` | `paladin_web::adapters::api_content_deliverer` (when `web-server` feature active) | `scheduler_integration_test.rs` | ✅ yes (1 consumer) |
| `src/infrastructure/adapters/sanctum/mod.rs` | `paladin_memory::sanctum::*` | integration tests (`rag_integration_tests.rs`, `in_memory_sanctum_tests.rs`, `qdrant_sanctum_tests.rs`), examples (`paladin_with_sanctum.rs`, `sanctum_basic_inmemory.rs`, `sanctum_configuration.rs`) | ✅ yes (6+ consumers) |
| `src/infrastructure/repositories/mod.rs` | `paladin_storage::*` (feature-gated) + local repository modules | `service_runner.rs`, integration tests | ✅ yes (conditional bridge) |
| `src/infrastructure/web/mod.rs` | `paladin_web::*` (when `web-server` feature active) | none found in workspace grep | ⚠️ no current workspace consumers |
| `src/core/mod.rs` | `paladin_core::base` + `pub mod platform` | 275+ files across `src/`, `crates/`, `tests/`, `examples/` | ✅ yes (many) |
| `src/core/platform/mod.rs` | `paladin_core::platform::container::*` + maneuver module injection | Many files across the workspace | ✅ yes (bridge with real injection logic) |
| `src/prelude.rs` | `crate::{LlmPort, GarrisonPort, PaladinBuilder, ...}` (convenience re-exports) | 0 real consumers in workspace (doc-comment examples only) | ⚠️ no current workspace consumers |

---

### Section 2 — `src/lib.rs` Individual `pub use` Lines

> Each stable API re-export line in `src/lib.rs`, its source, and workspace consumer status.
> Note: workspace code predominantly uses full module paths (e.g., `paladin::application::use_cases::...`) rather than the short `paladin::TypeName` aliases exported here.
> External library users are the primary consumers of these short aliases.

| `pub use` Target | Exported Type(s) | Short-path Workspace Consumers | Notes |
|-----------------|-----------------|-------------------------------|-------|
| `paladin_ports::output::llm_port::*` | `LlmError, LlmPort, LlmRequest, LlmResponse, ProviderCapabilities, TokenUsage` | 0 (full path used internally) | Stable API; port trait |
| `paladin_llm::error::LlmProviderError` | `LlmProviderError` | 0 | Stable API |
| `paladin_llm::provider_factory::LlmProviderFactory` | `LlmProviderFactory` | 0 | Stable API |
| `paladin_llm::openai::*` (llm-openai feature) | `OpenAIAdapter, OpenAIConfig` | ✅ 13 in examples/tests | Most-used short-path exports |
| `paladin_llm::openai::embedding::*` (openai-embeddings feature) | `OpenAIEmbeddingAdapter, OpenAIEmbeddingConfig` | 0 | Feature-gated |
| `paladin_llm::anthropic::*` (llm-anthropic feature) | `AnthropicAdapter, AnthropicConfig` | ✅ (part of 13 above) | Stable API |
| `paladin_llm::deepseek::*` (llm-deepseek feature) | `DeepSeekAdapter, DeepSeekConfig` | ✅ (part of 13 above) | Stable API |
| `paladin_llm::mock::*` | `MockLlmAdapter, MultiStepMockLlmPort` | ✅ (part of 13 above) | Used in tests/examples |
| `core::platform::container::prompt::PromptItem` | `PromptItem` | 0 | Re-exports via core shim |
| `paladin_ports::output::garrison_port::*` | `GarrisonError, GarrisonPort, GarrisonStats, LongTermGarrisonPort` | 0 | Stable API; port trait |
| `paladin_ports::output::sanctum_port::*` | `SanctumError, SanctumFilter, SanctumPort, SanctumQuery, SanctumSearchResult` | 0 | Stable API; port trait |
| `core::platform::container::sanctum::SanctumEntry` | `SanctumEntry` | 0 | Re-exports via core shim |
| `paladin_memory::garrison::InMemoryGarrison` | `InMemoryGarrison` | 0 | Full path used instead |
| `paladin_memory::garrison::SqliteGarrison` | `SqliteGarrison` | 0 | Full path used instead |
| `paladin_memory::sanctum::InMemorySanctum` | `InMemorySanctum` | 0 | Full path used instead |
| `paladin_memory::sanctum::QdrantSanctumAdapter` (qdrant feature) | `QdrantSanctumAdapter` | 0 | Feature-gated |
| `paladin_memory::services::*` | `ExtractedMemory, MemoryExtractionService, MemoryExtractionStrategy, RagConfig, RagRetrievalService, RetrievalTrigger` | 0 | Full path used instead |
| `paladin_ports::output::embedding_port::*` | `Embedding, EmbeddingError, EmbeddingPort` | 0 | Stable API; port trait |
| `paladin_ports::output::arsenal_port::*` | `ArsenalPort, ArsenalRegistry` | 0 | Stable API; port trait |
| `core::platform::container::arsenal::ArsenalError` | `ArsenalError` | 0 | Re-exports via core shim |
| `paladin_ports::output::citadel_port::CitadelPort` | `CitadelPort` | 0 | Stable API; port trait |
| `application::errors::citadel_error::CitadelError` | `CitadelError` | 0 | Via `application/errors` shim |
| `paladin_ports::output::queue_port::QueuePort` | `QueuePort` | 0 | Stable API; port trait |
| `application::use_cases::queue_orchestrator::QueueError` | `QueueError` | 0 | Application service type |
| `paladin_ports::output::notification_port::*` | `Notification, NotificationChannel, NotificationDeliveryPort, NotificationPortError, NotificationPriority, NotificationStatus, NotificationTemplate, NotificationTemplatePort` | 0 | Stable API; port trait |
| `paladin_ports::output::file_storage_port::*` | `FileStorageError, FileStoragePort` | 0 | Stable API; port trait |
| `paladin_ports::output::paladin_port::*` | `PaladinPort, PaladinResult, StopReason` | 0 | Stable API; port trait |
| `paladin_ports::output::battalion_port::BattalionPort` | `BattalionPort` | 0 | Stable API; port trait |
| `core::platform::container::battalion::*` | `BattalionResult, BattalionStatus` | 0 | Re-exports via core shim |
| `paladin_battalion` | entire `paladin_battalion` crate | 0 via short path; used via `paladin::paladin_battalion::*` | Crate-level re-export |
| `paladin_ports::input::content_input_port::ContentIngestionPort` | `ContentIngestionPort` | 0 | Stable API; input port |
| `paladin_ports::input::document_port::DocumentPort` | `DocumentPort` | 0 | Stable API; input port |
| `paladin_ports::input::ml_port::MlPort` | `MlPort` | 0 | Stable API; input port |
| `core::platform::container::paladin::*` | `Paladin, PaladinData, PaladinStatus` | ✅ 17 (via `paladin::Paladin`) | Domain entity; widely used |
| `core::platform::container::paladin_config::PaladinConfig` | `PaladinConfig` | 0 | Domain entity |
| `core::platform::container::battalion::*` | `BattalionConfig, BattalionError` | 0 | Domain entity |
| `core::platform::container::battalion::campaign::Campaign` | `Campaign` | 0 | Domain entity |
| `core::platform::container::battalion::chain_of_command::ChainOfCommand` | `ChainOfCommand` | 0 | Domain entity |
| `core::platform::container::battalion::formation::Formation` | `Formation` | 0 | Domain entity |
| `core::platform::container::battalion::phalanx::Phalanx` | `Phalanx` | 0 | Domain entity |
| `core::platform::container::arsenal::*` | `Armament, ArmamentCall, ArmamentResult` | 0 | Domain entity |
| `application::use_cases::battalion::commander::CommanderBuilder` | `CommanderBuilder` | 0 (full path used) | Via battalion shim |
| `application::use_cases::paladin::paladin_builder::PaladinBuilder` | `PaladinBuilder` | 0 (full path used) | Application service |
| `core::platform::container::battalion::council::CouncilBuilder` | `CouncilBuilder` | 0 | Domain entity builder |
| `core::platform::container::battalion::grove::GroveBuilder` | `GroveBuilder` | 0 | Domain entity builder |
| `application::errors::citadel_error::CitadelError as CitadelServiceError` | `CitadelServiceError` | 0 | Duplicate alias |
| `application::use_cases::paladin::error::PaladinError` | `PaladinError` | 0 (full path used) | Application service error |
| `core::base::entity::collection::CollectionType` | `CollectionType` | 0 | Base type |
| `core::base::entity::field::Field` | `Field` | 0 | Base type |
| `core::base::entity::message::Message` | `Message` | 0 (full path used) | Base type |
| `core::base::entity::node::Node` | `Node` | 0 (full path used) | Base type |
