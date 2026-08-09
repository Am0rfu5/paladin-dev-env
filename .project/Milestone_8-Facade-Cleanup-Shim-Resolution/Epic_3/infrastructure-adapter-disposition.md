# Infrastructure Adapter Disposition Record — Epic 3

> **SUPERSEDED BY [ADR-0028](../../../.planning/decisions/0028-m8-reconciliation-authoritative.md) — 2026-08-08.**
> This document's 20-row table marks every adapter group as staying in the facade and defers every
> List B move to Milestone 9, both of which the tree contradicts: the relocations executed inside
> Milestone 8 itself (`facade-cleanup-RECONCILIATION-2026-06-04.md` — 15 commits, ~10,250 net LOC
> removed, one new leaf crate, `paladin-herald`), and several of its "stays" rows mis-classified
> orphaned, uncompiled duplicate files as active bridges (ADR-0028's Context cites the specific
> rows). Two further defects survive independent of the supersession: row 1 (`arsenal/`) and row
> 19 (`sanctum/mod.rs`) each name a target crate — `paladin-arsenal` and `paladin-sanctum` — that
> disagrees with the governing PRD and does not exist in the tree; it remains FACADE-04's subject
> **Update — 2026-08-08 (Phase 11, plan 11-04):** all twenty rows of the table below are now triaged at [`facade-04-m9-candidate-triage.md`](../../../.planning/registers/facade-04-m9-candidate-triage.md) — 14 `done`, 6 `not a candidate`, 0 `still open` — and `paladin-arsenal` / `paladin-sanctum` are recorded there as artefacts of this table rather than as future crates.
> for those two names. The original text below is retained unmodified.

**Milestone:** 8 — Facade Cleanup & Shim Resolution
**Epic:** 3 — Relocate Remaining Misplaced Modules
**Branch:** `feature/milestone_8-epic_3-relocate-misplaced-modules`
**Date:** 2025-01

## Purpose

This document records the disposition decision for every adapter group under
`src/infrastructure/adapters/`. For each group it states:

1. Whether it **stays** in the facade crate or was **moved/deleted** in Epic 3.
2. The rationale for that decision.
3. Whether it is a **Milestone 9+ extraction candidate** and, if so, the target
   leaf crate.
4. What concrete action was taken (or is deferred) in Epic 3.

---

## Disposition Table

| # | Adapter path | Epic 3 decision | Rationale | M9 extraction candidate | Target crate | Action in Epic 3 |
|---|---|---|---|---|---|---|
| 1 | `adapters/arsenal/` (5 files: `mcp_protocol.rs`, `mcp_sse_adapter.rs`, `mcp_stdio_adapter.rs`, `resource_controls.rs`, `tool_result_formatter.rs`, `mod.rs`) | **Stays** | Active multi-consumer bridge; no leaf crate owns MCP orchestration yet | Yes (List A) | future `paladin-arsenal` (M9) | No change |
| 2 | `adapters/citadel/file_citadel.rs` | **Stays** | Active bridge; only one consumer (Paladin execution) but removal risks regression without a receiving crate | Yes (List B) | `paladin-memory` (M9) | No change |
| 3 | `adapters/document/` (`document_adapter.rs`, `pdf_extractor.rs`, `mod.rs`) | **Stays** | Active bridge; referenced from content pipeline examples | Yes (List B) | `paladin-content` (M9) | No change |
| 4 | `adapters/file_storage/minio.rs` | **Stays** | Active bridge; consumed by storage and content pipelines | Yes (List B) | `paladin-storage` (M9) | No change |
| 5 | `adapters/garrison/mod.rs` | **Stays** | Active multi-consumer bridge; implements memory port consumed by Paladin core, examples, and tests | Optional (M9) | `paladin-memory` (M9) | No change |
| 6 | `adapters/herald/` (`json_herald.rs`, `markdown_herald.rs`, `table_herald.rs`, `mod.rs`) | **Stays** | Cross-cutting output formatter; no clear single-leaf owner | No — output formatting is facade-level concern | — | No change |
| 7 | `adapters/input/file_content_fetcher.rs` | **Stays** | Already feature-gated behind `content-processing`; delegates to `paladin-content` | Yes (List A) | `paladin-content` (M9) | No change |
| 8 | `adapters/input/file_content_list_fetcher.rs` | **Stays** | Same as #7 | Yes (List A) | `paladin-content` (M9) | No change |
| 9 | `adapters/input/http_content_fetcher.rs` | **Stays** | Same as #7 | Yes (List A) | `paladin-content` (M9) | No change |
| 10 | `adapters/input/local_file_fetcher.rs` | **Stays** | Same as #7 | Yes (List A) | `paladin-content` (M9) | No change |
| 11 | `adapters/input/news_api_fetcher.rs` | **Stays** | Same as #7 | Yes (List A) | `paladin-content` (M9) | No change |
| 12 | `adapters/input/tensorflow_adapter.rs` | **Stays, feature-gated** | Placeholder for future ML adapter; no leaf crate owns ML yet (Milestone 9+) | Yes | future `paladin-ml` (M9+) | **Gated behind `cfg(feature = "ml")`; `ml = []` added to `Cargo.toml`; module-level doc comment added** |
| 13 | `adapters/llm/` (`config_bridge.rs`, `mod.rs`) | **Stays** | Active bridge; `paladin-llm` contains the real providers but config mapping lives here | Optional (M9) | `paladin-llm` (M9) | No change |
| 14 | `adapters/logs/` (`error_log_adapter.rs`, `system_log_adapter.rs`, `mod.rs`) | **Stays** | Cross-cutting concern; consumed across multiple crates | No — logging infrastructure is facade-level | — | No change |
| 15 | `adapters/notifications/` (`email_notification_adapter.rs`, `system_notification_adapter.rs`, `mod.rs`) | **Stays** | Active bridge; already confirmed in Task 1.0 that dual-pattern (facade re-export + leaf crate) is intentional | No change needed | `paladin-notifications` owns the types | No change (Task 1.0 confirmed) |
| 16 | `adapters/output/api_content_deliverer.rs` | **Stays** | Active bridge; consumed by `paladin-web` API layer via facade path | Yes (List B) | `paladin-web` (M9) | No change |
| 17 | `adapters/paladin_registry.rs` | **Stays** | Facade-level registry; orchestrates multiple leaf crate adapters | No | — | No change |
| 18 | `adapters/queue/redis.rs` | **Stays** | Active bridge; consumed by job scheduler and async execution paths | Yes (List B) | `paladin-storage` (M9) | No change |
| 19 | `adapters/sanctum/mod.rs` | **Stays** | Active bridge; implements auth/security port; multi-consumer | Optional (M9) | future `paladin-sanctum` (M9) | No change |
| 20 | `adapters/scheduling/tokio_cron_adapter.rs` | **Stays** | Active bridge; only concrete scheduler implementation | No | — | No change |

---

## Summary

### Epic 3 Actions Taken

| Action | Files affected |
|--------|---------------|
| Feature-gated behind `cfg(feature = "ml")` | `src/infrastructure/adapters/input/tensorflow_adapter.rs`, `src/infrastructure/adapters/input/mod.rs` |
| `ml = []` added to `Cargo.toml` | `Cargo.toml` |
| Storage re-export shims deleted | `src/application/storage/sql_store.rs`, `src/application/storage/user_store.rs`, `src/application/storage/mod.rs` |
| 6 consumers updated to `paladin_ports::` | `sqlite_content_repository.rs`, `mysql_content_repository.rs`, `sqlite_user_repository.rs`, `user_service.rs`, `service_runner.rs`, `tests/repository/mysql_content_repository_test.rs` |

### Milestone 9 Extraction Candidates (Summary)

**List A — Leaf crate already owns the domain, adapter can be inlined:**

| Adapter | Target crate |
|---------|-------------|
| `input/file_content_fetcher.rs` | `paladin-content` |
| `input/file_content_list_fetcher.rs` | `paladin-content` |
| `input/http_content_fetcher.rs` | `paladin-content` |
| `input/local_file_fetcher.rs` | `paladin-content` |
| `input/news_api_fetcher.rs` | `paladin-content` |
| `arsenal/` (all 5 adapters) | future `paladin-arsenal` |

**List B — Leaf crate needs to be created or extended first:**

| Adapter | Target crate |
|---------|-------------|
| `citadel/file_citadel.rs` | `paladin-memory` (extend) |
| `document/` | `paladin-content` (extend) |
| `file_storage/minio.rs` | `paladin-storage` (extend) |
| `output/api_content_deliverer.rs` | `paladin-web` (extend) |
| `queue/redis.rs` | `paladin-storage` (extend) |
| `input/tensorflow_adapter.rs` | future `paladin-ml` (new crate) |

### Adapters with No Extraction Target (Facade-Level Concerns)

- `adapters/herald/` — cross-cutting output formatting
- `adapters/logs/` — cross-cutting logging infrastructure
- `adapters/paladin_registry.rs` — facade orchestration registry
- `adapters/scheduling/` — single concrete scheduler
- `adapters/llm/config_bridge.rs` — config mapping stays in facade
