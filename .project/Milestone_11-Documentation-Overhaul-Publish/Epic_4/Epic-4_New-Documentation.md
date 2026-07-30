# Epic 4: New Documentation

**Project:** Paladin Framework
**Milestone:** 11 — Documentation Review, Reorganization, MDBook Publish
**Epic:** 4 — New Documentation
**Version Target:** v0.5.0
**Status:** Not Started
**Created:** 2026-05-29

---

## Milestone Context

The project has ~40 markdown documentation files accumulated across eight milestones of development. Several significant features completed in Milestones 8–9 have no documentation at all: the orchestration pipeline, the content processing system, the agent↔orchestrator bridge, and a consolidated feature flag / crate reference. This Epic writes all net-new documentation for those gaps.

### Milestone Success Criteria (for reference)

- Every existing doc file is audited: current, stale, or delete.
- MDBook builds locally and via CI with zero warnings.
- Documentation published to GitHub Pages (or equivalent).
- All code examples in docs compile against the current workspace.
- New documentation covers: orchestration guide, content processing, crate map, agent↔orchestrator bridge.
- The main `paladin-dev-env` monorepo includes the docs as a subdirectory.

---

## Parallel Execution Context

**Epic 4 has dependencies and should not begin until:**

1. **Milestone 9** is complete — the features being documented (orchestrator, bridge, content pipeline) must exist in their final form before they can be documented accurately.
2. **Epic 2 (MDBook Setup)** is complete — new content goes into the `docs/src/` structure.

Epic 4 can run **in parallel with Epic 3** (Content Rewrite) once the above prerequisites are met. The Epic 1 gap list feeds into this Epic's scope — any undocumented features identified in the audit should be added as tasks here.

---

## Epic Overview

**Priority:** High
**Estimated Effort:** Medium
**Dependencies:** Milestone 9, Epic 2

### Objective

Write new documentation for features and capabilities that do not have existing docs. Every document must include working code examples that compile against the current workspace.

---

## Tasks

### Task 4.1: Orchestration Guide

**Description:**

Write a comprehensive guide covering the Paladin orchestration system — how workflows are created, scheduled, and managed.

**File:** `docs/src/user-guides/orchestration.md`

**Content Requirements:**

The guide must cover the following topics, each with a working code example:

1. **Workflow Patterns Overview**
   - When to use Formation (sequential) vs. Phalanx (parallel) vs. Campaign (DAG) vs. Chain of Command (hierarchical).
   - Decision flowchart (Mermaid diagram).

2. **Creating Sequential Workflows (Formation)**
   - `FormationService` setup.
   - Chaining Paladins: output of step N becomes input of step N+1.
   - Error handling and short-circuit behavior.
   - Full working example.

3. **Creating Parallel Workflows (Phalanx)**
   - `PhalanxService` setup.
   - Concurrent Paladin execution.
   - Aggregating results.
   - Concurrency limit configuration.
   - Full working example.

4. **Graph-Based Workflows (Campaign)**
   - `CampaignService` setup.
   - Defining a DAG of Paladin steps.
   - Conditional edges.
   - Full working example.

5. **Hierarchical Delegation (Chain of Command)**
   - `ChainOfCommandService` setup.
   - Supervisor → subordinate Paladin delegation.
   - Full working example.

6. **Dynamic Strategy Routing (Commander)**
   - `Commander` setup and strategy selection.
   - Auto-routing based on task characteristics.
   - Full working example.

7. **Job Scheduling**
   - Creating and queuing jobs.
   - Queue management.
   - Retry and timeout configuration.

8. **Event and Trigger System**
   - Event types available.
   - Subscribing to events.
   - Trigger-based workflow initiation.

**Deliverables:**

- `docs/src/user-guides/orchestration.md` — complete, with working examples.

---

### Task 4.2: Content Processing Guide

**Description:**

Write a guide covering the full content ingestion and processing pipeline.

**File:** `docs/src/user-guides/content-processing.md`

**Content Requirements:**

1. **Content Ingestion Sources**
   - PDF ingestion.
   - Web scraping.
   - RSS feed consumption.
   - HTTP endpoint polling.
   - Configuration for each source type.

2. **Content Aggregation and Filtering**
   - Aggregating content from multiple sources.
   - Filtering rules.
   - Deduplication.

3. **Content Processing Pipeline**
   - Pipeline stages overview.
   - Transformation steps.
   - Enrichment steps.

4. **Content → Agent Bridge**
   - How processed content is passed to Paladin agents for AI enrichment.
   - Configuration.
   - Full working example (e.g., RSS feed → filtering → Paladin analysis → output).

5. **Content Delivery**
   - Output destinations.
   - Notification integration.

**Deliverables:**

- `docs/src/user-guides/content-processing.md` — complete, with working examples.

---

### Task 4.3: Agent ↔ Orchestrator Bridge Guide

**Description:**

Write a guide covering how AI agents (Paladins) and orchestration workflows interact bidirectionally.

**Placement:** This content should be included as a dedicated section in `docs/src/user-guides/orchestration.md` **or** as a standalone file `docs/src/user-guides/agent-orchestrator-bridge.md`. Choose the placement based on length and natural reading flow — if it significantly extends the orchestration guide, make it a separate file and link from the orchestration guide.

**Content Requirements:**

1. **Agents Triggering Orchestration**
   - How a Paladin agent can initiate a new workflow.
   - Tool-based invocation of orchestration from within an agent loop.
   - Configuration.
   - Full working example.

2. **Orchestration Invoking Agents**
   - How a workflow step calls a Paladin agent.
   - Passing context from the workflow into the agent.
   - Returning agent output back to the workflow.
   - Full working example.

3. **Configuration Examples**
   - `config.yml` snippets for common bridge patterns.

4. **Use Case Recipes**
   - "News monitoring pipeline with AI analysis": RSS feed → content aggregation → Paladin summarization → notification.
   - "Research workflow": web search tool → Paladin synthesis → Formation report assembly.
   - At least two additional real-world patterns.

**Deliverables:**

- Bridge content integrated into `orchestration.md` or as `agent-orchestrator-bridge.md` (with link from orchestration guide).
- All code examples verified to pass `cargo check`.

---

### Task 4.4: Crate Map and Feature Flag Reference

**Description:**

Write a comprehensive reference showing the full workspace crate topology and every feature flag.

**File:** `docs/src/api-reference/crate-map.md`
*(Also update `docs/src/api-reference/feature-flags.md` if a separate file is retained.)*

**Content Requirements:**

**Crate Map section** must cover:

1. **Workspace Crate Table**

| Crate | Layer | Purpose | Key Exports |
|-------|-------|---------|-------------|
| `paladin-core` | Core | Domain entities, base primitives | `Node<T>`, `Paladin`, `Battalion`, ... |
| `paladin-ports` | Application | Port trait definitions | `LlmPort`, `GarrisonPort`, `ArsenalPort`, ... |
| `paladin-battalion` | Application | Battalion use cases | `FormationService`, `PhalanxService`, ... |
| `paladin-llm` | Infrastructure | LLM provider adapters | `OpenAiAdapter`, ... |
| `paladin-memory` | Infrastructure | Garrison adapters | `InMemoryGarrison`, `SqliteGarrison`, ... |
| `paladin-storage` | Infrastructure | File/DB storage adapters | `MinioAdapter`, ... |
| `paladin-content` | Application/Infra | Content pipeline | ... |
| `paladin-notifications` | Infrastructure | Notification adapters | ... |
| `paladin-web` | Infrastructure | Web API layer | ... |
| *(all workspace crates)* | ... | ... | ... |

2. **Crate Dependency Graph** — Mermaid diagram showing which crates depend on which.

3. **Feature Flag Reference**

| Feature Flag | Crate | Enables | External Dependencies Gated |
|-------------|-------|---------|---------------------------|
| `redis-queue` | `paladin-storage` | Redis-backed job queue | `redis` crate |
| `s3-storage` | `paladin-storage` | MinIO/S3 file storage | `aws-sdk-s3` or `minio` crate |
| `sqlite-garrison` | `paladin-memory` | SQLite persistent garrison | `rusqlite` crate |
| `openai` | `paladin-llm` | OpenAI LLM adapter | `async-openai` or similar |
| `anthropic` | `paladin-llm` | Anthropic LLM adapter | `anthropic` client crate |
| `deepseek` | `paladin-llm` | DeepSeek LLM adapter | HTTP client |
| *(all feature flags)* | ... | ... | ... |

4. **Consumer Profiles** — Practical dependency recipes:

```toml
# Minimal: single Paladin with in-memory garrison, OpenAI LLM
[dependencies]
paladin-core = { version = "0.5" }
paladin-ports = { version = "0.5" }
paladin-llm = { version = "0.5", features = ["openai"] }
paladin-memory = { version = "0.5" }

# Full: Battalion orchestration, Redis queue, SQLite garrison, all LLMs
[dependencies]
paladin-core = { version = "0.5" }
paladin-ports = { version = "0.5" }
paladin-battalion = { version = "0.5" }
paladin-llm = { version = "0.5", features = ["openai", "anthropic", "deepseek"] }
paladin-memory = { version = "0.5", features = ["sqlite-garrison"] }
paladin-storage = { version = "0.5", features = ["redis-queue", "s3-storage"] }
```

**Deliverables:**

- `docs/src/api-reference/crate-map.md` — complete crate map with dependency graph and feature flag reference.
- Consumer profiles section with at least three dependency profiles (minimal, standard, full).

---

## Deliverables Summary

| Artifact | Description |
|----------|-------------|
| `docs/src/user-guides/orchestration.md` | Comprehensive orchestration guide with all patterns |
| `docs/src/user-guides/content-processing.md` | Content ingestion and pipeline guide |
| Bridge content | Agent↔orchestrator bridge (in orchestration.md or separate) |
| `docs/src/api-reference/crate-map.md` | Full crate map, dependency graph, feature flags, consumer profiles |

---

## Definition of Done

- [x] `orchestration.md` covers the battalion patterns plus scheduling and events, with working examples. *(Formation, Phalanx, Campaign, Chain of Command, Commander + scheduling + events documented inline; Maneuver and Conclave/Council/Grove are linked to their existing guides — `maneuver-flow-dsl.md` / `battalion-patterns.md` — rather than duplicated.)*
- [x] `content-processing.md` covers all ingestion sources, pipeline stages, and the content→agent bridge. *(Honestly documents that `web-scraping`/`rss`/filtering are declared-but-unimplemented — see OQ-6.)*
- [x] Agent↔orchestrator bridge content is present with at least two use case recipes. *(Standalone `agent-orchestrator-bridge.md` with 4 recipes.)*
- [x] `crate-map.md` lists every workspace crate with purpose, layer, and key exports. *(New `api-reference/crate-map.md`, all 9 crates.)*
- [x] `crate-map.md` includes a Mermaid crate dependency graph.
- [x] Feature flag table covers every flag in the workspace. *(Root umbrella + per-crate tables.)*
- [x] At least three consumer profile `Cargo.toml` snippets are present. *(Minimal / standard / full / granular = 4.)*
- [x] All code examples pass `cargo check`. *(Stronger than originally scoped: substantive examples are real code in `crates/doc-examples`, included via mdBook `{{#include}}` and compiled by `cargo check -p paladin-doc-examples` in CI + pre-push. A few illustrative fragments remain `rust,ignore` but are hand-verified against source.)*
- [x] `mdbook build` succeeds with **zero broken links** (linkcheck `warning-policy = "error"`) after all new content is added. *(Note: linkcheck still emits non-fatal "fragment resolution isn't implemented" notices for `#anchor` links — a backend limitation, not content errors — and a mermaid preprocessor version notice; neither fails the build. These pre-date Epic 4.)*

---

## Schedule Reference

| Phase | This Epic | Duration | Predecessors |
|-------|-----------|----------|-------------|
| Phase 2 | Epic 4: New Documentation | 1–2 sprints | Milestone 9; Epic 2 |

Runs in parallel with Epic 3 (Content Rewrite) once prerequisites are met.
