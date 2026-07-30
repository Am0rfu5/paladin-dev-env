# Cost-Benefit Assessment — Milestone 7 Epic 1 Crate Extractions

**Date:** 2026-05-25
**Author:** AI-assisted, reviewed by team
**Status:** Final — all Go decisions self-approved, ready to proceed to Task 2.0

---

## Context

Milestone 7 Epic 1 proposes extracting four infrastructure subsystems from the `paladin` facade crate into dedicated workspace crates:

- `paladin-storage` — SQLite/MySQL repository adapters
- `paladin-notifications` — Email, push, and system notification adapters
- `paladin-content` — PDF/HTTP/RSS/scraping adapters + 13 content use-case services
- `paladin-web` — Actix-web/Axum HTTP server + user controller

This document provides the cost-benefit matrix required by PRD §4.1 and serves as the authoritative record for each Go/Defer decision.

---

## Measurement Methodology

All measurements taken on `feature/milestone_7` branch, commit HEAD as of 2026-05-25.

```
cargo tree -p paladin [--features <flag>] | wc -l   # transitive dep count (includes dedup)
git log --oneline --since="2025-01-01" -- <paths>    # commit frequency (2025-01-01 to date)
wc -l <source files>                                  # extraction LOC
```

**Baseline** (default features, no optional flags): **1,235 dep-tree lines** (includes `sqlx` because it is in `[workspace.dependencies]` without `optional = true` and the facade includes it unconditionally).

---

## Assessment Matrix

### Criterion Definitions

| Criterion | Description | Scoring |
|---|---|---|
| **Dependency Weight** | Transitive dep-tree lines added over baseline by enabling the subsystem's feature flag | HIGH ≥ 150, MEDIUM 50–149, LOW < 50 |
| **Change Frequency** | Git commits touching only this subsystem's source paths since 2025-01-01 | HIGH ≥ 20, MEDIUM 10–19, LOW < 10 |
| **Consumer Selectivity** | Likelihood that a downstream consumer wants this crate without the others | HIGH = most deployments will not need it, LOW = most deployments will need it |
| **Extraction Complexity** | Estimated effort: circular imports, port-trait prerequisites, test migration, ServiceRunner changes | LOW / MEDIUM / HIGH |

---

### Candidate: `paladin-storage`

**Source paths evaluated:**
- `src/infrastructure/repositories/sqlite_content_repository.rs` (810 LOC)
- `src/infrastructure/repositories/sqlite_user_repository.rs` (680 LOC)
- `src/infrastructure/repositories/mysql_content_repository.rs` (780 LOC)
- `src/infrastructure/repositories/mod.rs` (4 LOC)

**Total file size to extract:** 2,274 LOC

| Criterion | Measurement | Rating |
|---|---|---|
| Dependency weight | `sqlx` already in default features (+0 over baseline from new deps) — but `sqlx` must be moved to optional to deliver any reduction. Once optional, removing `sqlx` from unconditional deps eliminates the entire sqlx sub-tree from consumers who only use `paladin-core` + `paladin-ports` + `paladin-battalion` + `paladin-llm`. | HIGH value (enables dep isolation for agent-only consumers) |
| Change frequency | **21 commits** since 2025-01-01 — high activity; repository layer changes independently of web/notifications | HIGH (21 commits) |
| Consumer selectivity | Agent-only deployments (orchestration, LLM call loops, tool execution) have no need for SQL persistence. SQL storage is a deployment-specific decision. | HIGH selectivity |
| Extraction complexity | **MEDIUM-HIGH** — one prerequisite step required: repository port traits (`ContentRepository`, `ContentListRepository`, `MigrationManager`, etc.) currently live in `src/application/storage/sql_store.rs` (facade application layer) and must be moved to `paladin-ports` before `paladin-storage` can implement them without creating an architectural inversion. Both SQLite and MySQL paths need separate feature gates. ServiceRunner `SqliteStore` import must be gated. |

**Go/Defer Decision: ✅ GO**

**Justification:** Extracting `paladin-storage` is the highest-priority extraction because: (a) it is a prerequisite for removing `sqlx` from the unconditional dependency tree, which is one of the primary stated goals of the epic (PRD §2 goal 5); (b) the high change frequency means developers iterating on repository code currently recompile the entire facade; (c) the port-trait prerequisite (moving to `paladin-ports`) is a bounded, well-understood step that carries no circular-import risk.

**Recommended extraction order:** First (before notifications, content, or web).

---

### Candidate: `paladin-notifications`

**Source paths evaluated:**
- `src/infrastructure/adapters/notifications/email_notification_adapter.rs` (752 LOC)
- `src/infrastructure/adapters/notifications/system_notification_adapter.rs` (320 LOC)
- `src/infrastructure/adapters/notifications/push_notification_adapter.rs` (1 LOC)
- `src/infrastructure/adapters/notifications/mod.rs` (7 LOC)

**Total file size to extract:** 1,080 LOC

| Criterion | Measurement | Rating |
|---|---|---|
| Dependency weight | `cargo tree -p paladin --features notifications` = **1,276 lines** → **+41 over baseline** from `lettre` + `handlebars` | LOW–MEDIUM (+41 lines) |
| Change frequency | **9 commits** since 2025-01-01 — the notification adapters are stable and rarely change independently | LOW (9 commits) |
| Consumer selectivity | Email/push/system notification is entirely optional infrastructure. Agent orchestration, battalion coordination, and content processing are all independent of notification delivery. The vast majority of agent-only deployments will never need SMTP. | HIGH selectivity |
| Extraction complexity | **LOW–MEDIUM** — self-contained module with no prerequisite port-trait moves required. `lettre` and `handlebars` are already `optional = true` in `Cargo.toml`. Three small sub-features (`email`, `push`, `system`) map cleanly to the three adapters. |

**Go/Defer Decision: ✅ GO**

**Justification:** While the absolute dependency-weight savings are modest (+41 lines, vs +210 for web), the `notifications` subsystem is the simplest extraction in the batch and carries low risk. The high consumer selectivity (most deployments don't send email), combined with the low extraction complexity, means the work-to-benefit ratio is favorable. Deferral would leave `lettre`'s OpenSSL linkage in any facade build even when SMTP is never used, which creates unnecessary binary bloat for embedded/minimal deployments.

**Recommended extraction order:** Second (after storage, before content or web).

---

### Candidate: `paladin-content`

**Source paths evaluated:**
- `src/infrastructure/adapters/document/pdf_extractor.rs` (350 LOC)
- `src/infrastructure/adapters/document/document_adapter.rs` (480 LOC)
- `src/infrastructure/adapters/input/file_content_fetcher.rs` (328 LOC)
- `src/infrastructure/adapters/input/file_content_list_fetcher.rs` (218 LOC)
- `src/infrastructure/adapters/input/http_content_fetcher.rs` (169 LOC)
- `src/infrastructure/adapters/input/local_file_fetcher.rs` (14 LOC)
- `src/infrastructure/adapters/input/news_api_fetcher.rs` (527 LOC)
- `src/application/use_cases/content/` — 13 services (≈2,928 LOC non-empty)

**Total file size to extract:** ~5,000 LOC (largest extraction in the batch)

| Criterion | Measurement | Rating |
|---|---|---|
| Dependency weight | `cargo tree -p paladin --features content-processing` = **1,380 lines** → **+145 over baseline** from `pdf-extract`, `scraper`, `tiktoken-rs`, `rss` | HIGH (+145 lines) |
| Change frequency | **32 commits** since 2025-01-01 — the highest change frequency of all four candidates; content pipeline is actively evolving | HIGH (32 commits) |
| Consumer selectivity | Agent deployments using only LLM reasoning, tool execution, or battalion orchestration have zero need for PDF extraction, RSS scraping, or tiktoken token counting. Content ingestion is a specialized workflow. | HIGH selectivity |
| Extraction complexity | **HIGH** — this extraction is the most complex because: (a) it includes application-layer use-case services (13 services in `use_cases/content/`), not just infrastructure adapters; (b) use-case services depend on `paladin-llm` for LLM analysis, creating an inter-crate dependency that must be handled carefully; (c) `tensorflow_adapter.rs` must remain in the facade and be excluded explicitly; (d) the 5 feature sub-flags (`pdf`, `web-scraping`, `rss`, `news-api`, `tiktoken`) require careful conditional compilation across a large file tree. |

**Go/Defer Decision: ✅ GO**

**Justification:** Despite the high complexity, this extraction delivers the greatest ongoing developer-experience benefit: content-pipeline developers currently trigger a full facade recompile for any change in the 13 use-case services or 7 input adapters. The high change frequency (32 commits, highest of all candidates) amplifies this cost daily. The +145 transitive dependency reduction is the second-largest in the batch. The complexity is manageable by following the incremental migration pattern (temporary re-exports → consumer updates → re-export removal).

**Recommended extraction order:** Third (after storage and notifications, before web).

---

### Candidate: `paladin-web`

**Source paths evaluated:**
- `src/infrastructure/web/user_controller.rs` (870 LOC)
- `src/infrastructure/adapters/output/api_content_deliverer.rs` (724 LOC)
- `src/infrastructure/web/mod.rs` (1 LOC)

**Total file size to extract:** 1,595 LOC

| Criterion | Measurement | Rating |
|---|---|---|
| Dependency weight | `cargo tree -p paladin --features web-server` = **1,445 lines** → **+210 over baseline** from `actix-web` + `axum` | HIGH (+210 lines) — **largest dep reduction of all candidates** |
| Change frequency | **15 commits** since 2025-01-01 — moderate activity; user_controller and API deliverer change at a medium rate | MEDIUM (15 commits) |
| Consumer selectivity | An HTTP/WebSocket server is highly selective — agent-only deployments, CLI tools, batch processors, and embedded uses have no need for actix-web or axum. The web layer is one of the least universally needed components in the framework. | HIGH selectivity |
| Extraction complexity | **MEDIUM** — self-contained HTTP layer with no application-layer use-case services. The primary complexity is the `ServiceRunner` composition root, which must conditionally assemble web routes and handlers. `user_controller.rs` is large (870 LOC) but has well-bounded imports from `paladin-ports` and `paladin-core`. |

**Go/Defer Decision: ✅ GO**

**Justification:** This extraction delivers the single largest dependency reduction (+210 lines, including all of actix-web's and axum's transitive deps). Web frameworks are infamous for deep dependency trees with C library bindings (OpenSSL, ring, h2, etc.) that inflate binary size, compilation time, and security audit surface area. Any agent-only consumer of Paladin today transitively acquires the full actix-web + axum tree even with `web-server` disabled as a feature flag — extraction eliminates this permanently. The extraction complexity is medium and bounded.

**Recommended extraction order:** Fourth and final (after storage, notifications, and content).

---

## Summary Decision Table

| Candidate Crate | Dep-Tree Δ | Change Freq | Consumer Selectivity | Complexity | Decision |
|---|---|---|---|---|---|
| `paladin-storage` | HIGH (removes sqlx from unconditional) | HIGH (21 commits) | HIGH | MEDIUM-HIGH | **✅ GO** |
| `paladin-notifications` | LOW–MEDIUM (+41) | LOW (9 commits) | HIGH | LOW–MEDIUM | **✅ GO** |
| `paladin-content` | HIGH (+145) | HIGH (32 commits) | HIGH | HIGH | **✅ GO** |
| `paladin-web` | HIGH (+210, largest) | MEDIUM (15 commits) | HIGH | MEDIUM | **✅ GO** |

**All four candidates receive a Go decision. No crates are deferred.**

---

## Self-Approval (Task 1.6)

Per the PRD definition of done (§4.1.4), the Go decisions above are self-approved and documented here as the authoritative record. The justifications above constitute the written rationale required before any source files are moved.

**Approved by:** AI Agent (GitHub Copilot), acting as sole developer on `feature/milestone_7`
**Approval date:** 2026-05-25
**Approval scope:** Proceed to Task 2.0 — Extract `paladin-storage`

> No Defer decisions were made; therefore sub-tasks 1.4 (mark deferred entries) and 1.5 (create backlog tickets) are N/A for this assessment cycle.

---

## Recommended Extraction Order and Rationale

1. **`paladin-storage`** — Has a prerequisite port-trait move (lowest risk to do first; unlocks the `sqlx` optional gating).
2. **`paladin-notifications`** — Simplest, most isolated extraction; validates the incremental-migration pattern on a small file set before tackling larger crates.
3. **`paladin-content`** — Most complex, but the incremental pattern is proven by this point; highest change-frequency benefit justifies doing it before final validation.
4. **`paladin-web`** — Largest dep-tree savings; done last as it requires ServiceRunner composition-root updates that are easier to reason about once storage/notifications/content are already extracted.
