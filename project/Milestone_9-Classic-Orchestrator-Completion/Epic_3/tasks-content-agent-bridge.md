# Tasks: Content → Agent Bridge

**PRD:** `prd-content-agent-bridge.md`
**Epic:** 3 — Milestone 9
**Target:** v0.3.0

## Relevant Files

- `src/application/services/orchestration/types.rs` - Home of the `ContentProcessor` trait, `ContentProcessingResult`, `ContentAnalysisType`, and `OrchestratorError`; referenced by both new processors.
- `src/application/services/orchestration/mod.rs` - `Orchestrator`; `register_content_processor()` and `process_content()` dispatch path the processors plug into; site of the unregistered-name `ProcessorNotFound` assertion.
- `src/application/services/orchestration/processors/mod.rs` - **New** module exposing the two content processors and shared config types (prompt template, `OutputParsing` strategy).
- `src/application/services/orchestration/processors/paladin_processor.rs` - **New** `PaladinContentProcessor` (single-agent bridge) + in-module unit tests with a mock LLM.
- `src/application/services/orchestration/processors/battalion_processor.rs` - **New** `BattalionContentProcessor` (Formation + Phalanx bridge, defined merge strategy) + in-module unit tests with mock agents.
- `src/application/services/paladin/paladin_execution_service.rs` - `PaladinExecutionService::execute()`; invoked by `PaladinContentProcessor`.
- `crates/paladin-battalion/src/formation_service.rs` - `FormationExecutionService::execute()`; invoked by the Formation path of `BattalionContentProcessor`.
- `crates/paladin-battalion/src/phalanx_service.rs` - `PhalanxExecutionService::execute()`; invoked by the Phalanx path of `BattalionContentProcessor`.
- `crates/paladin-core/src/platform/container/content.rs` - `ContentItem`; the input converted into an agent prompt.
- `crates/paladin-core/src/platform/container/orchestration_context.rs` - `OrchestrationContext`; threaded through `process_content()`.
- `tests/content_agent_bridge.rs` - **New** integration tests: register a `PaladinContentProcessor` (mock LLM) and drive a `ContentItem` through `Orchestrator::process_content()`; assert enrichment + `ProcessorNotFound` for an unregistered name.
- `tests/content_ingestion_pipeline.rs` - **New** integration tests (gated by `content-processing`): deterministic local-fixture + mock-LLM ingestion → enrichment path, plus a `#[ignore]` live network + real-LLM path.

### Notes

- The processors live in the **root crate** beside the `ContentProcessor` trait (not in `paladin-content`) to avoid a circular crate dependency — see PRD §7 / Open Question 1.
- Processors depend only on `PaladinPort`/`LlmPort` and the Battalion execution services; no concrete LLM adapter dependency. Unit tests use the `paladin-llm` `mock` feature.
- Unit tests live in `#[cfg(test)]` modules beside each processor; integration tests live under `tests/`.
- Default suite must stay deterministic and offline: mock LLM + local fixtures. The real-network/real-LLM test is `#[ignore]`/credential-gated.
- Use `set +H &&` before git commits and avoid `!` in commit messages (bash history-expansion guard).
- Run `cargo test` (and at least once with `--features content-processing`), `cargo fmt --check`, and `cargo clippy -- -D warnings` before committing each parent task.

## Tasks

- [x] 0.0 Create feature branch
  - [x] 0.1 Create and checkout `feature/milestone_9-epic_3-content-agent-bridge`

- [x] 1.0 Implement `PaladinContentProcessor` (PRD Task 3.1)
  - [x] 1.1 Create the `processors/` module (`mod.rs`) under `src/application/services/orchestration/` and register it; add a shared `OutputParsing` enum (`RawText` default, `Json`) and a prompt-template type
  - [x] 1.2 Implement `PaladinContentProcessor` holding `Arc<PaladinExecutionService>`, `Arc<Paladin>`, prompt template, and parsing strategy; implement `name()` and `clone_box()`
  - [x] 1.3 Implement `process_content()`: convert `ContentItem` → prompt via the configurable template (interpolate body + title/metadata)
  - [x] 1.4 Execute via `PaladinExecutionService::execute(&paladin, &prompt)` and map `PaladinResult` → `ContentProcessingResult` (content_id, processor_name, processing_time_ms, success)
  - [x] 1.5 Implement the `RawText` strategy (store response verbatim) and the `Json` strategy (parse response into `result_data`)
  - [x] 1.6 Attach enrichment metadata (agent name, model if available, parsing strategy, length/token indicator)
  - [x] 1.7 Implement degraded-result handling: malformed JSON under the `Json` strategy → `success=false`, `error` populated, raw text preserved (no panic, no data loss)
  - [x] 1.8 Unit tests with a mock LLM adapter: prompt conversion + enrichment; raw-text verbatim; well-formed JSON parsed; malformed JSON → degraded result
  - [x] 1.9 Add rustdoc for all new public items

- [x] 2.0 Implement `BattalionContentProcessor` (PRD Task 3.2)
  - [x] 2.1 Define a pattern-selector config (Formation vs. Phalanx) owning the corresponding domain object + execution service
  - [x] 2.2 Implement `BattalionContentProcessor` with `name()`/`clone_box()` and `process_content()` building the prompt from the `ContentItem`
  - [x] 2.3 Formation path: delegate to `FormationExecutionService::execute()` and thread outputs through the pipeline
  - [x] 2.4 Phalanx path: delegate to `PhalanxExecutionService::execute()` and run analysts in parallel
  - [x] 2.5 Implement the merge strategy → single `ContentProcessingResult` (Formation surfaces final output; Phalanx merges per-agent outputs into `result_data`), documented in code
  - [x] 2.6 Attach metadata identifying the pattern used and participating agents
  - [x] 2.7 Unit tests with mock agents: Formation runs sequentially into one result; Phalanx runs in parallel and merges into one result
  - [x] 2.8 Add rustdoc for all new public items

- [x] 3.0 Wire content processing into the Orchestrator (PRD Task 3.3)
  - [x] 3.1 Confirm/extend `Orchestrator::register_content_processor()` accepts both processors as `Box<dyn ContentProcessor>` registered by name
  - [x] 3.2 Confirm `Orchestrator::process_content()` dispatches to the named processor within the existing session lifecycle and returns its `ContentProcessingResult`
  - [x] 3.3 Verify the unregistered-name path returns `OrchestratorError::ProcessorNotFound(name)`
  - [x] 3.4 Integration test (`tests/content_agent_bridge.rs`): register a `PaladinContentProcessor` (mock LLM), drive a `ContentItem` through `process_content()`, assert enriched result + metadata
  - [x] 3.5 Integration test: assert referencing an unregistered processor name returns `ProcessorNotFound`

- [x] 4.0 Content ingestion pipeline validation (PRD Task 3.4)
  - [x] 4.1 Add a deterministic end-to-end test (`tests/content_ingestion_pipeline.rs`) gated by `content-processing`: local fixture / `FileContentListFetcher` → extract/aggregate → content processor (mock LLM) → enriched result, with no network
  - [x] 4.2 Assert the full chain preserves content id, includes enrichment, and reports success
  - [x] 4.3 Add a separate live end-to-end test (real `HttpContentFetcher` + real LLM) marked `#[ignore]` and/or credential-gated, with run instructions documented in the test
  - [x] 4.4 Run the suite with `--features content-processing` and confirm the deterministic path passes offline

- [ ] 5.0 Quality gate
  - [ ] 5.1 `cargo fmt --check`
  - [ ] 5.2 `cargo clippy -- -D warnings` (and with `--features content-processing`)
  - [ ] 5.3 `cargo test` (and at least once with `--features content-processing`)
  - [ ] 5.4 `cargo build` clean; mark PRD success metrics and checklist complete
