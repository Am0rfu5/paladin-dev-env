# PRD: Epic 4 — New Documentation

**Project:** Paladin Framework
**Milestone:** 11 — Documentation Review, Reorganization, MDBook Publish
**Epic:** 4 — New Documentation
**Version Target:** v0.5.0 (documentation written against the current **v0.4.3** workspace)
**Status:** Not Started
**Created:** 2026-06-02
**Author:** Paladin Framework Contributors

---

## 1. Introduction / Overview

Epic 2 stood up the MDBook site and Epic 3 rewrote all existing content so it compiles and links correctly against the current v0.4.3 workspace. However, several major subsystems that shipped in Milestones 8–9 still have **no dedicated documentation**:

- The full **orchestration system** — the six Battalion patterns (Formation, Phalanx, Campaign, Chain of Command, Commander, Maneuver), job scheduling, and the event/trigger system. The existing `orchestration.md` only documents the Commander strategy router; the other patterns are scattered or undocumented as a cohesive guide.
- The **content processing pipeline** (`paladin-content`) — ingestion sources, aggregation, filtering, the processing/analysis stages, and content delivery. No guide exists.
- The **agent ↔ orchestrator bridge** (`OrchestratorPort` + `BridgePolicy` in `paladin-ports`) — how Paladin agents trigger orchestration and how workflows invoke agents. Undocumented.
- A consolidated **crate map and feature-flag reference** with consumer-profile `Cargo.toml` recipes. An architecture-layer crate map exists, but no api-reference page combining the full crate topology, every feature flag, and copy-paste dependency profiles.

This Epic writes all net-new documentation for those gaps. Every document must include working code examples that pass `cargo check` against the current workspace, and every `config.yml` snippet must match the current schema.

**Key decisions carried into this PRD** (resolved with the maintainer — see §9):

1. **`orchestration.md` is fully rewritten** into a single comprehensive guide. The existing Commander content is folded into the "Commander" section.
2. **A new `api-reference/crate-map.md`** is created (crate table + dependency graph + full feature-flag reference + consumer profiles). The existing `architecture/crate-map.md` is retained as the architecture-layer view and the two are cross-linked.
3. **All examples target the current v0.4.3 crates** (path/workspace deps as published today), not the unpublished 0.5.0, so snippets are accurate and checkable now.
4. **The Epic 3 code-check tooling is reused as a hard gate**, extended with `config.yml` / YAML snippet validation for the new guides.

---

## 2. Goals

1. **Write a comprehensive orchestration guide** (`user-guides/orchestration.md`, full rewrite) covering all six Battalion patterns plus job scheduling and the event/trigger system, each with a working example.
2. **Write a content processing guide** (`user-guides/content-processing.md`, new) covering every available ingestion source, aggregation/filtering/dedup, the processing/analysis pipeline, the content→agent bridge, and delivery.
3. **Write an agent↔orchestrator bridge guide** (new standalone `user-guides/agent-orchestrator-bridge.md`, linked from the orchestration guide) covering both directions of the bridge with at least four use-case recipes.
4. **Write a consolidated crate map & feature-flag reference** (`api-reference/crate-map.md`, new) with the full workspace crate table, a Mermaid dependency graph, every feature flag, and at least three consumer-profile `Cargo.toml` snippets — cross-linked with `architecture/crate-map.md`.
5. **Register all new pages in `docs/src/SUMMARY.md`** so they appear in the rendered book.
6. **Every fenced Rust example passes `cargo check`** via the existing `scripts/check-doc-examples.sh` gate (pre-push hook + `docs.yml` CI).
7. **Every `config.yml` / YAML snippet validates** against the current schema via an extended config-check step.
8. **`mdbook build` succeeds with zero warnings** (with `[output.linkcheck]` enforcing) after all new content is added.

---

## 3. User Stories

- **As a developer choosing an orchestration pattern**, I want one guide that explains when to use Formation vs. Phalanx vs. Campaign vs. Chain of Command vs. Commander, with a decision flowchart and a working example for each, so I can pick the right pattern without reading source code.
- **As a developer building a content pipeline**, I want a guide that shows how to ingest from PDFs, HTTP endpoints, and news/feed sources, then aggregate, filter, analyze, and deliver, so I can assemble a working pipeline from documented building blocks.
- **As a developer wiring AI into a workflow**, I want the bridge guide to show me both how an agent can trigger a workflow and how a workflow step can call an agent, with `config.yml` examples and recipes, so I can build agent-driven and orchestration-driven flows.
- **As a developer adding Paladin to a project**, I want a crate map with copy-paste `Cargo.toml` profiles (minimal / standard / full) and a feature-flag table, so I can declare exactly the dependencies and features I need.
- **As any reader**, I want every code example in these new pages to compile against the version I'm using, so I'm never misled by aspirational APIs.

---

## 4. Functional Requirements

> **Naming accuracy mandate (applies to every FR):** The PRD lists API names from the current workspace, but the author **must verify every type, method, module path, and feature flag against the source before publishing**. The actual service types are `FormationExecutionService`, `PhalanxExecutionService`, `CampaignExecutionService`, `ChainOfCommandExecutionService`, `ManeuverExecutionService`, and `Commander` / `CommanderBuilder` (note the `ExecutionService` suffix — the Epic's shorthand `FormationService` etc. is **not** the real name). When the Epic asks for a capability that has **no corresponding adapter/type in the workspace**, document the actual available capability and record the gap as an open question rather than inventing an API.

### Task 4.1 — Orchestration Guide

#### FR-1: Full rewrite of `user-guides/orchestration.md`
The existing 873-line Commander-only page is **replaced** with a comprehensive guide. The existing Commander material is preserved by folding it into FR-8 below. The guide must open with a Table of Contents and cover FR-2 through FR-10.

#### FR-2: Workflow Patterns Overview
- A comparison of when to use **Formation** (sequential), **Phalanx** (parallel), **Campaign** (DAG), **Chain of Command** (hierarchical), and **Commander** (dynamic routing). (Maneuver/Flow-DSL already has its own guide — link to it rather than duplicating.)
- A **Mermaid decision flowchart** (`flowchart TD`) for selecting a pattern.

#### FR-3: Formation (Sequential)
- `FormationExecutionService` setup.
- Chaining Paladins so the output of step N feeds step N+1.
- Error handling / short-circuit behavior, referencing `ErrorStrategy` and `BattalionConfig`.
- A full working example that passes `cargo check`.

#### FR-4: Phalanx (Parallel)
- `PhalanxExecutionService` setup.
- Concurrent Paladin execution and result aggregation.
- Concurrency-limit configuration.
- A full working example.

#### FR-5: Campaign (DAG)
- `CampaignExecutionService` setup.
- Defining a DAG of Paladin steps and conditional edges.
- A full working example.

#### FR-6: Chain of Command (Hierarchical)
- `ChainOfCommandExecutionService` setup.
- Supervisor → subordinate Paladin delegation.
- A full working example.

#### FR-7: Commander (Dynamic Strategy Routing)
- `Commander` / `CommanderBuilder` setup and strategy selection (Auto vs. explicit).
- Auto-routing based on task characteristics.
- A full working example. (Fold the salvageable, still-accurate prose from the existing page here.)

#### FR-8: Job Scheduling
- Creating and queuing jobs using the real port types: `JobSpec`, `JobId`, `JobInfo`, `JobStatus`, `ScheduleJobRequest`, and the queue port (`FullQueuePort` / `BatchQueuePort`, `QueueItemRequest`, `QueueStats`).
- Queue management, retry, and timeout configuration.
- A working example (note any feature flag required, e.g. `redis-queue`).

#### FR-9: Event and Trigger System
- Available event types and the trigger model (`Trigger`, `TriggerConfig`, `TriggerCondition`, `TimeCondition`, `TriggerStatus` from `paladin-core`).
- Subscribing to events / firing events (`FireEventRequest`, `EventDispatchResult`).
- Trigger-based workflow initiation, with an example.

#### FR-10: Link to the bridge guide
The orchestration guide ends with a "See also" pointer to `agent-orchestrator-bridge.md` (FR-15–FR-19).

### Task 4.2 — Content Processing Guide

#### FR-11: New file `user-guides/content-processing.md`
Registered in `SUMMARY.md` under **User Guides**. Covers FR-12 through FR-14.

#### FR-12: Content Ingestion Sources
Document the **actually available** input adapters in `crates/paladin-content/src/adapters/input/` and the document adapter:
- **PDF extraction** — `PdfExtractor` / `DocumentAdapter`.
- **HTTP endpoint fetching** — `HttpContentFetcher`.
- **News/feed ingestion** — `NewsApiFetcher`.
- **File / local sources** — `FileContentFetcher`, `FileContentListFetcher`, `LocalFileFetcher`, `FileSourceRepository`.
- Configuration for each source type, with `config.yml` snippets.
- **Honesty requirement:** the Epic lists "web scraping" and "RSS feed consumption" as distinct sources. If no dedicated scraper/RSS adapter exists (only `HttpContentFetcher` + `NewsApiFetcher`), document what *is* available and how to approximate the others, and log the gap as an open question — do not document a non-existent adapter.

#### FR-13: Aggregation, Filtering, and the Processing Pipeline
- Aggregating content from multiple sources — `AggregateContent` / `content_aggregator_service`.
- Filtering rules and deduplication — `ContentFilter` / `content_filtering_service`.
- Pipeline / analysis stages — `content_analysis_service`, `content_summarizer_service`, `ContentSummarizer`, and the NLP/ML/LLM analyzers (`NlpContentAnalyzer`, `content_ml_analysis_service`, `LlmContentAnalyzer`).
- A pipeline-stages overview diagram (Mermaid).

#### FR-14: Content → Agent Bridge and Delivery
- How processed content is passed to a Paladin agent for AI enrichment — `LlmContentAnalyzer` / `LlmContentAnalysisConfig` / `LlmContentAnalysisInput` (`content_llm_analysis_service`).
- A **full working example**: feed/news source → filtering → Paladin analysis → output.
- Content delivery destinations and notification integration — `DeliverContentUseCase` / `content_delivery_service`, plus a cross-link to the bridge guide and the notifications setup.

### Task 4.3 — Agent ↔ Orchestrator Bridge Guide

#### FR-15: New standalone file `user-guides/agent-orchestrator-bridge.md`
Per the maintainer decision (separate file), registered in `SUMMARY.md` under **User Guides** and linked from `orchestration.md` (FR-10). Covers FR-16 through FR-19.

#### FR-16: Agents Triggering Orchestration
- How a Paladin agent initiates a workflow via the bridge — `OrchestratorPort`, `BridgeAction`, and the request types (`ScheduleJobRequest`, `QueueItemRequest`, `FireEventRequest`, `SendNotificationRequest`).
- Tool-based invocation of orchestration from within an agent loop.
- `BridgePolicy` (allow-lists / caps via `is_allowed`, `cap_for`, `allow`) and `OrchestratorBridgeError` handling.
- A full working example.

#### FR-17: Orchestration Invoking Agents
- How a workflow step calls a Paladin agent (`PaladinExecutorPort` / `BattalionPort`).
- Passing context from the workflow into the agent and returning agent output back to the workflow.
- A full working example.

#### FR-18: Configuration Examples
- `config.yml` snippets for common bridge patterns (must validate against the current schema — FR-22).

#### FR-19: Use-Case Recipes (at least four)
1. **News monitoring with AI analysis** — feed/news source → aggregation → Paladin summarization → notification.
2. **Research workflow** — web/HTTP tool → Paladin synthesis → Formation report assembly.
3. **Two additional real-world patterns** (e.g., scheduled batch enrichment via the job queue; event/trigger-initiated agent run).

### Task 4.4 — Crate Map & Feature-Flag Reference

#### FR-20: New file `api-reference/crate-map.md`
Registered in `SUMMARY.md` under **API Reference**. Cross-linked bidirectionally with `architecture/crate-map.md` (architecture page = layer view; api-reference page = consumer/dependency view). Covers FR-21's four sections.

#### FR-21: Crate Map content
1. **Workspace crate table** — every one of the nine workspace crates with its directory, **published package name** (note `paladin-core` → package `paladin-ai-core`), layer, purpose, and key exports.
2. **Crate dependency graph** — a Mermaid `graph TD` showing inter-crate dependencies (must match actual `Cargo.toml` dependencies).
3. **Feature-flag reference table** — every Cargo feature flag across the workspace, the crate it belongs to, what it enables, and the external dependency it gates. Sourced directly from each crate's `Cargo.toml` (cross-reference / reconcile with `api-reference/feature-flags.md`).
4. **Consumer profiles** — at least three copy-paste `Cargo.toml` snippets (minimal / standard / full), using **current-version dependency declarations consistent with how the crates are published today (v0.4.3)**, not the unpublished `0.5`.

### Cross-cutting Requirements

#### FR-22: Code-check and config-check gate
- All new fenced ```rust blocks must pass the existing `scripts/check-doc-examples.sh` (already wired into the `.pre-commit-config.yaml` pre-push hook and `.github/workflows/docs.yml` from Epic 3). No new Rust tooling is needed; the new files are picked up automatically because the script globs `docs/src/**/*.md`.
- **Extend the gate with config validation:** every fenced ```yaml / `config.yml` snippet in the new guides must be validated against the current configuration schema (e.g., parsed/loaded by the same config types the framework uses, or schema-checked in a new check step). Add this as a step in `docs.yml` and, if practical, the pre-push hook.

#### FR-23: SUMMARY.md registration
All four new/rewritten pages must be linked in `docs/src/SUMMARY.md` so `mdbook build` includes them with no "file not in SUMMARY" warnings.

#### FR-24: Zero-warning build
After all content is added, `mdbook build` must complete with zero warnings, with `[output.linkcheck]` (`warning-policy = "error"`) enforcing — every internal cross-link from the new pages must resolve.

---

## 5. Non-Goals (Out of Scope)

- **Rust source-code changes** — This Epic modifies only `docs/src/` markdown, `docs/src/SUMMARY.md`, and the check tooling (`scripts/`, `docs.yml`, `.pre-commit-config.yaml`, `Makefile`) as needed for FR-22. No `*.rs` changes and no API additions. If a documented capability is missing, it is recorded as an open question, **not** implemented here.
- **Rewriting existing Epic 3 pages** — Only `orchestration.md` is rewritten (per the maintainer decision); all other existing pages were finished in Epic 3 and are untouched except for new cross-links.
- **Appendix consolidation** — Existing appendix pages (e.g., `battalion-patterns-guide.md`, `conclave-pattern.md`, `council.md`, `grove.md`, `sentinel.md`) are not merged or rewritten; the new guides may link to them.
- **Maneuver / Flow-DSL guide** — Already documented (`user-guides/maneuver-flow-dsl.md`); the orchestration guide links to it instead of duplicating it.
- **Publishing / GitHub Pages deployment** — That is Epic 5.
- **External link validation** — `follow-web-links = false`; external URLs are not verified.
- **Bumping crate versions to 0.5.0** — Examples target the current v0.4.3 workspace; a version bump is a release concern, not part of this Epic.

---

## 6. Design Considerations

### Suggested authoring order
1. **FR-20/FR-21 (crate map)** first — it establishes the authoritative crate names, package names, and feature flags every other page references.
2. **FR-1–FR-10 (orchestration)** — defines the pattern vocabulary the bridge guide builds on.
3. **FR-11–FR-14 (content processing)**.
4. **FR-15–FR-19 (bridge)** — depends on both the orchestration and content vocabularies.
5. **FR-22/FR-23/FR-24 (gates + SUMMARY + build)** verified continuously, with a final full `mdbook build` + `make check-doc-examples` pass at the end.

### Code-example conventions (inherited from Epic 3)
- Fenced Rust blocks are self-contained or use mdBook `# `-hiding lines for boilerplate imports.
- Use `# #[allow(unused)]` hiding lines to avoid dead-code `cargo check` warnings.
- Examples needing a running service carry a `> **Prerequisites:** Run \`make dev\` first.` callout. Job-queue examples that require Redis must note the `redis-queue` feature and the running service.

### Mermaid diagrams
- `flowchart TD` for the pattern decision flowchart (FR-2); `graph TD` for the crate dependency graph (FR-21) and the pipeline-stages overview (FR-13); `sequenceDiagram` for bridge interaction flows (FR-16/FR-17). All must render under the current mdbook-mermaid build.

### Cross-linking
- `orchestration.md` ↔ `agent-orchestrator-bridge.md`, and both → `content-processing.md` for the news-monitoring recipe.
- `api-reference/crate-map.md` ↔ `architecture/crate-map.md` ↔ `api-reference/feature-flags.md`.

---

## 7. Technical Considerations

### Workspace crates (v0.4.3)
| Directory | Package name | Layer |
|---|---|---|
| `crates/paladin-core` | `paladin-ai-core` | Core |
| `crates/paladin-ports` | `paladin-ports` | Application (ports) |
| `crates/paladin-battalion` | `paladin-battalion` | Application |
| `crates/paladin-llm` | `paladin-llm` | Infrastructure |
| `crates/paladin-memory` | `paladin-memory` | Infrastructure |
| `crates/paladin-storage` | `paladin-storage` | Infrastructure |
| `crates/paladin-content` | `paladin-content` | Application/Infra |
| `crates/paladin-notifications` | `paladin-notifications` | Infrastructure |
| `crates/paladin-web` | `paladin-web` | Infrastructure |

### Verified source anchors for the new guides
- **Battalion services:** `crates/paladin-battalion/src/` — `FormationExecutionService`, `PhalanxExecutionService`, `CampaignExecutionService`, `ChainOfCommandExecutionService`, `ManeuverExecutionService`, `Commander`, `CommanderBuilder`, `ErrorStrategy`. Root re-exports: `BattalionConfig`, `BattalionError` (`src/lib.rs`).
- **Scheduling / events:** `crates/paladin-ports/src/` — `JobSpec`, `JobId`, `JobInfo`, `JobStatus`, `ScheduleJobRequest`, `SchedulerError`, `FullQueuePort`, `BatchQueuePort`, `QueueItemRequest`, `QueueStats`, `QueueError`. Triggers: `crates/paladin-core/src/platform/container/trigger.rs` — `Trigger`, `TriggerConfig`, `TriggerCondition`, `TimeCondition`, `TriggerStatus`.
- **Bridge:** `crates/paladin-ports/src/output/orchestrator_port.rs` — `OrchestratorPort`, `BridgeAction`, `BridgePolicy`, `OrchestratorBridgeError`, `ScheduleJobRequest`, `QueueItemRequest`, `FireEventRequest`, `SendNotificationRequest`, `EventDispatchResult`. Agent execution: `PaladinExecutorPort`, `BattalionPort`.
- **Content:** `crates/paladin-content/src/adapters/input/` (`HttpContentFetcher`, `NewsApiFetcher`, `FileContentFetcher`, `FileContentListFetcher`, `LocalFileFetcher`), `adapters/document/` (`PdfExtractor`, `DocumentAdapter`), and `services/` (`content_aggregator_service`, `content_filtering_service`, `content_analysis_service`, `content_summarizer_service`, `content_llm_analysis_service`, `content_delivery_service`). Types: `AggregateContent`, `ContentFilter`, `ContentSummarizer`, `LlmContentAnalyzer`, `DeliverContentUseCase`.

### Tooling reuse (FR-22)
- `scripts/check-doc-examples.sh` already extracts and `cargo check`s every ```rust block under `docs/src/**/*.md`; new files are covered automatically.
- The config-validation extension is **new** — the simplest implementation parses each fenced YAML snippet through the framework's config loader/types so an invalid key fails CI. Confirm the exact config type and loader entry point before implementing.

### Dependencies
- Milestone 9 (orchestrator, bridge, content pipeline) is complete — all documented APIs exist in final form.
- Epic 2 (MDBook) and Epic 3 (content rewrite + link/code gates) are merged to `main`.

---

## 8. Success Metrics

| Metric | Target |
|---|---|
| New/rewritten pages delivered | 4 (`orchestration.md` rewrite, `content-processing.md`, `agent-orchestrator-bridge.md`, `api-reference/crate-map.md`) |
| Battalion patterns documented with working example | 5 core (Formation, Phalanx, Campaign, Chain of Command, Commander) + scheduling + events |
| Bridge use-case recipes | ≥ 4 |
| Workspace crates in crate-map table | 9 / 9 |
| Feature flags documented | Every flag across all crate `Cargo.toml`s |
| Consumer-profile `Cargo.toml` snippets | ≥ 3 (minimal / standard / full) |
| Fenced Rust blocks failing `cargo check` | 0 |
| Fenced YAML/config snippets failing schema validation | 0 |
| `mdbook build` warnings (linkcheck enforcing) | 0 |
| New pages registered in `SUMMARY.md` | 4 / 4 |

---

## 9. Open Questions

| ID | Question | Resolution |
|---|---|---|
| OQ-1 | `orchestration.md` already exists as a Commander-only guide — rewrite, expand, or new file? | **Full rewrite** into a comprehensive guide; fold existing Commander content into the Commander section. (Maintainer decision.) |
| OQ-2 | `architecture/crate-map.md` already exists — where does Task 4.4's crate map live? | **New `api-reference/crate-map.md`** (consumer/dependency view) cross-linked with the existing architecture-layer page. (Maintainer decision.) |
| OQ-3 | What version should consumer profiles / code examples target, given the workspace is v0.4.3 but the milestone targets 0.5.0? | **Target current v0.4.3** so snippets are accurate and checkable now; update to 0.5 when released. (Maintainer decision.) |
| OQ-4 | Should the new docs be gated by Epic 3's `check-doc-examples.sh`? | **Yes — reuse it as a hard gate, and extend it with `config.yml`/YAML schema validation.** (Maintainer decision.) |
| OQ-5 | Bridge content: section in `orchestration.md` or standalone file? | **Standalone `user-guides/agent-orchestrator-bridge.md`**, linked from the orchestration guide (the combined length would overload a single page). |
| OQ-6 | The Epic lists "web scraping" and "RSS feed" as ingestion sources, but the workspace appears to expose only `HttpContentFetcher` and `NewsApiFetcher`. | **Document the actual available adapters**; describe how to approximate web-scraping/RSS with what exists; flag any genuinely missing source as a documented limitation rather than inventing an API. Author to confirm against source during writing. |
| OQ-7 | Exact config type/loader entry point for the YAML validation step (FR-22)? | **To be confirmed by the author** when implementing the config-check; identify the struct the framework deserializes `config.yml` into and reuse it. |

---

## Relevant Files

### New Files
- `docs/src/user-guides/content-processing.md` — content pipeline guide (FR-11–FR-14)
- `docs/src/user-guides/agent-orchestrator-bridge.md` — bridge guide (FR-15–FR-19)
- `docs/src/api-reference/crate-map.md` — crate map, dependency graph, feature flags, consumer profiles (FR-20–FR-21)

### Files Rewritten
- `docs/src/user-guides/orchestration.md` — full rewrite into comprehensive orchestration guide (FR-1–FR-10)

### Files Updated (In-Place)
- `docs/src/SUMMARY.md` — register the three new pages (FR-23)
- `docs/src/architecture/crate-map.md` — add cross-link to the new api-reference page
- `docs/src/api-reference/feature-flags.md` — reconcile / cross-link with the new feature-flag table

### Tooling (extended for FR-22)
- `.github/workflows/docs.yml` — add a YAML/config-snippet validation step
- `.pre-commit-config.yaml` — (optional) add config-check to the pre-push stage
- `scripts/check-doc-examples.sh` (or a new `scripts/check-doc-config.sh`) — config-snippet validation
- `Makefile` — convenience target for the config check (mirroring `check-doc-examples`)
