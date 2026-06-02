## Relevant Files

### New Files
- `docs/src/api-reference/crate-map.md` — New consumer/dependency crate map: crate table, Mermaid dependency graph, feature-flag reference, consumer profiles (FR-20, FR-21)
- `docs/src/user-guides/content-processing.md` — New content pipeline guide: ingestion, aggregation/filtering, processing, content→agent bridge, delivery (FR-11–FR-14)
- `docs/src/user-guides/agent-orchestrator-bridge.md` — New bridge guide: both bridge directions, config examples, ≥4 use-case recipes (FR-15–FR-19)

### Files Rewritten
- `docs/src/user-guides/orchestration.md` — Full rewrite into comprehensive orchestration guide: 5 patterns + scheduling + events (FR-1–FR-10)

### Files Updated (In-Place)
- `docs/src/SUMMARY.md` — Register the 3 new pages under User Guides / API Reference (FR-23)
- `docs/src/architecture/crate-map.md` — Add cross-link to the new api-reference crate map (Design Considerations: cross-linking)
- `docs/src/api-reference/feature-flags.md` — Reconcile with / cross-link the new feature-flag table (FR-21.3)

### Tooling (extended for FR-22)
- `scripts/check-doc-config.sh` — New script: extract fenced YAML/config snippets from `docs/src/**/*.md` and validate against the current config schema (FR-22)
- `.github/workflows/docs.yml` — Add a config-snippet validation step to the `build` job (FR-22)
- `.pre-commit-config.yaml` — Add config-check as a `pre-push` stage hook (FR-22)
- `Makefile` — Add a `check-doc-config` convenience target mirroring `check-doc-examples` (FR-22)

### Source anchors (read-only — for verifying API names; DO NOT modify)
- `crates/paladin-battalion/src/` — `FormationExecutionService`, `PhalanxExecutionService`, `CampaignExecutionService`, `ChainOfCommandExecutionService`, `Commander`, `CommanderBuilder`, `ErrorStrategy`
- `crates/paladin-ports/src/output/orchestrator_port.rs` — `OrchestratorPort`, `BridgeAction`, `BridgePolicy`, `OrchestratorBridgeError`, request types
- `crates/paladin-ports/src/` — `JobSpec`, `JobId`, `JobInfo`, `JobStatus`, `ScheduleJobRequest`, `FullQueuePort`, `BatchQueuePort`, `QueueItemRequest`, `QueueStats`
- `crates/paladin-core/src/platform/container/trigger.rs` — `Trigger`, `TriggerConfig`, `TriggerCondition`, `TimeCondition`, `TriggerStatus`
- `crates/paladin-content/src/adapters/` and `crates/paladin-content/src/services/` — content fetchers, analyzers, aggregator/filter/delivery services
- Each crate's `Cargo.toml` — authoritative feature flags

### Notes

- This Epic modifies **no Rust source code**. Do not edit `*.rs` files or `Cargo.toml` (except reading them to verify API/flag names). If a documented capability is missing from the workspace, record it as a documented limitation (per OQ-6) — do not implement it.
- All content edits are in `docs/src/`; the only non-docs changes are the config-check tooling (`scripts/`, `Makefile`, `.pre-commit-config.yaml`, `.github/workflows/docs.yml`).
- **Authoring order matters:** build the crate map first (Task 1.0) so canonical crate/package/flag names are settled before the guides reference them; write the bridge guide (Task 4.0) after orchestration and content, since it builds on both vocabularies.
- All fenced Rust code blocks must use mdBook hiding-line syntax (`# use ...;`) for imports and `# #[allow(unused)]` where needed to suppress dead-code warnings, so they pass `scripts/check-doc-examples.sh` (already wired into CI + pre-push from Epic 3).
- All dependency/version examples target the **current v0.4.3** workspace, not the unpublished 0.5.0 (per OQ-3).
- Use `make dev` to start services (Redis, MinIO, MySQL, Qdrant) before testing any example that requires a running service (e.g. `redis-queue` job-queue examples).
- Verification commands (run from inside `docs/`): `mdbook build` must exit 0 with zero warnings and linkcheck (`warning-policy = "error"`) passing. Run `make check-doc-examples` and `make check-doc-config` for the code/config gates.

## Instructions for Completing Tasks

**IMPORTANT:** As you complete each task, you must check it off in this markdown file by changing `- [ ]` to `- [x]`. This helps track progress and ensures you don't skip any steps.

Example:

- `- [ ] 1.1 Read file` → `- [x] 1.1 Read file` (after completing)

Update the file after completing each sub-task, not just after completing an entire parent task.

> **Completion-protocol note:** This Epic changes **no Rust source**, so the standard `cargo test` / `cargo fmt` / `cargo clippy` sequence does not apply. The per-parent-task completion gate here is: the new/changed docs pass `make check-doc-examples` (Rust blocks) and `make check-doc-config` (YAML blocks, once Task 5.0 lands), and `mdbook build` stays warning-free. Commit after each parent task and stop for go-ahead.

## Tasks

- [x] 0.0 Create feature branch
  - [x] 0.1 Create and checkout a new branch: `git checkout -b feature/milestone-11-epic-4-new-documentation`

- [x] 1.0 Create the crate map & feature-flag reference — `api-reference/crate-map.md` (FR-20, FR-21)
  - [x] 1.1 Read every crate's `Cargo.toml` (root + `crates/*`) to record package name, version, description, and the `[features]` table; read each crate's `src/lib.rs` for key public exports
  - [x] 1.2 Build the **workspace crate table**: directory, published package name (note `paladin-core` → `paladin-ai-core`), layer, purpose, key exports — all nine crates (FR-21.1)
  - [x] 1.3 Extract inter-crate dependencies from each `Cargo.toml` and build the **Mermaid `graph TD` dependency graph**, verified against actual deps (FR-21.2)
  - [x] 1.4 Enumerate **every feature flag** across all crate `Cargo.toml`s and build the feature-flag reference table (flag, crate, enables, external dependency gated) (FR-21.3)
  - [x] 1.5 Write **≥3 consumer-profile `Cargo.toml` snippets** (minimal / standard / full) targeting current v0.4.3 declarations (FR-21.4)
  - [x] 1.6 Assemble `docs/src/api-reference/crate-map.md` with intro and bidirectional cross-links to `architecture/crate-map.md` and `api-reference/feature-flags.md`
  - [x] 1.7 Validate: fenced blocks are `toml`/`mermaid` only (no Rust to `cargo check`); table data verified against source `Cargo.toml`s. Full `mdbook build`/linkcheck deferred to Task 7.0 (depends on content-processing.md + SUMMARY entries)

- [x] 2.0 Rewrite the orchestration guide — `user-guides/orchestration.md` (FR-1–FR-10)
  - [x] 2.1 Read the existing `orchestration.md`; identify still-accurate Commander prose to fold into the Commander section
  - [x] 2.2 Verify battalion service APIs against source: constructors, `execute`/run signatures, and config types (`BattalionConfig`, `ErrorStrategy`) for Formation/Phalanx/Campaign/ChainOfCommand/Commander
  - [x] 2.3 Write the page skeleton: title, Table of Contents, and the **Workflow Patterns Overview** with a Mermaid `flowchart TD` decision chart (FR-2)
  - [x] 2.4 Write **Formation (Sequential)** section: setup, chaining, error/short-circuit behavior, full working example (FR-3)
  - [x] 2.5 Write **Phalanx (Parallel)** section: setup, concurrent execution, result aggregation, concurrency limits, full example (FR-4)
  - [x] 2.6 Write **Campaign (DAG)** section: setup, DAG definition, conditional edges, full example (FR-5)
  - [x] 2.7 Write **Chain of Command (Hierarchical)** section: setup, supervisor→subordinate delegation, full example (FR-6)
  - [x] 2.8 Write **Commander (Dynamic Routing)** section: builder, Auto vs explicit, full example; fold salvaged prose from 2.1 (FR-7)
  - [x] 2.9 Write **Job Scheduling** section: `JobSpec`/`SchedulerPort`/queue ports, retry/timeout, example with `redis-queue` feature-flag note (FR-8)
  - [x] 2.10 Write **Event & Trigger System** section: trigger model, `fire_event`, trigger-initiated workflow example (FR-9)
  - [x] 2.11 Add the **"See also" link** to `agent-orchestrator-bridge.md` (FR-10)
  - [x] 2.12 Ran `make check-doc-examples` — 0 failed (all `rust,ignore` blocks skipped per Epic 3 convention); YAML matches canonical `battalion:` schema

- [x] 3.0 Write the content processing guide — `user-guides/content-processing.md` (FR-11–FR-14)
  - [x] 3.1 Verified content adapters/services against source. **OQ-6 resolved:** `web-scraping`/`rss` features declare deps but have **no adapter** (crates never referenced); `content_filtering_service` is **commented out** in `services/mod.rs` (not compiled); ml/nlp service files are dead (not in mod tree)
  - [x] 3.2 Write **Content Ingestion Sources** section: PDF, HTTP, news/feed, file/local — configured via Rust constructors (no `content:` config schema exists, so no speculative YAML); web-scraping/RSS documented as not-yet-implemented (FR-12)
  - [x] 3.3 Write **Aggregation and Processing Pipeline** section: `FetchContent`/`AggregateContent`/`ContentSummarizer`/`AnalyzeContent` stage table + Mermaid `flowchart LR`; filtering/dedup documented as disabled (FR-13)
  - [x] 3.4 Write **Content → Agent Bridge** section: `LlmContentAnalyzer` full async example (feed → analysis → JSON output), feature `llm` (FR-14)
  - [x] 3.5 Write **Content Delivery** section: `DeliverContentUseCase` / `content_delivery_port`, notification cross-link to bridge guide (FR-14)
  - [x] 3.6 Ran `make check-doc-examples` — 0 failed. Also corrected `crate-map.md`: removed non-compiled `ContentFilter` from key exports; marked `web-scraping`/`rss` flags as reserved/not-implemented

- [ ] 4.0 Write the agent↔orchestrator bridge guide — `user-guides/agent-orchestrator-bridge.md` (FR-15–FR-19)
  - [ ] 4.1 Verify `OrchestratorPort`, `BridgeAction`, `BridgePolicy`, request types, and `PaladinExecutorPort`/`BattalionPort` against source
  - [ ] 4.2 Write **Agents Triggering Orchestration** section: bridge actions, tool-based invocation, `BridgePolicy` caps, error handling, sequenceDiagram, full example (FR-16)
  - [ ] 4.3 Write **Orchestration Invoking Agents** section: workflow→agent call, context passing, returning output, full example (FR-17)
  - [ ] 4.4 Write **Configuration Examples** section with `config.yml` bridge snippets (FR-18)
  - [ ] 4.5 Write **Use-Case Recipes** section: ≥4 recipes (news monitoring, research workflow, scheduled batch enrichment, trigger-initiated run) (FR-19)
  - [ ] 4.6 Run `make check-doc-examples`; fix any failing block

- [ ] 5.0 Extend the doc-check gate with config/YAML validation (FR-22)
  - [ ] 5.1 Read `scripts/check-doc-examples.sh` to mirror its structure and conventions
  - [ ] 5.2 Identify the config struct/loader entry point the framework uses to deserialize `config.yml` (resolve OQ-7)
  - [ ] 5.3 Write `scripts/check-doc-config.sh`: extract fenced `yaml` snippets from `docs/src/**/*.md` and validate each against the config schema, with `trap` cleanup and per-snippet failure reporting
  - [ ] 5.4 Add a `check-doc-config` target to the `Makefile` (mirroring `check-doc-examples`)
  - [ ] 5.5 Add `check-doc-config` as a `pre-push` stage hook in `.pre-commit-config.yaml`
  - [ ] 5.6 Add a config-snippet validation step to the `build` job in `.github/workflows/docs.yml`
  - [ ] 5.7 Run `make check-doc-config` locally and confirm it passes on the new guides

- [ ] 6.0 Register new pages in `SUMMARY.md` and wire cross-links (FR-23, cross-linking)
  - [ ] 6.1 Add `orchestration.md` (if not already), `content-processing.md`, and `agent-orchestrator-bridge.md` under **User Guides**, and `crate-map.md` under **API Reference** in `docs/src/SUMMARY.md`
  - [ ] 6.2 Add a cross-link from `docs/src/architecture/crate-map.md` to the new `api-reference/crate-map.md`
  - [ ] 6.3 Reconcile and cross-link `docs/src/api-reference/feature-flags.md` with the new feature-flag table
  - [ ] 6.4 Verify every new internal cross-link resolves (no dangling references)

- [ ] 7.0 Final verification and commit (FR-24)
  - [ ] 7.1 Run `make check-doc-examples` across the whole book — zero failures
  - [ ] 7.2 Run `make check-doc-config` across the whole book — zero failures
  - [ ] 7.3 Run `mdbook build` from inside `docs/` — exits 0 with zero warnings and linkcheck (`warning-policy = "error"`) passing
  - [ ] 7.4 Update the Definition of Done checklist in the Epic doc; final commit
