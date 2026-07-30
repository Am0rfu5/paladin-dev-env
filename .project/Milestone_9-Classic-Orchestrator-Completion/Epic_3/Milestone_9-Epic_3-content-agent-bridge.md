# Milestone 9 — Epic 3: Content → Agent Bridge

**Project:** Paladin Framework
**Milestone:** 9 — Classic Orchestrator, Content Pipeline, and Agent-Orchestrator Bridge
**Epic:** 3 of 6
**Priority:** High
**Estimated Effort:** Large
**Dependencies:** Epic 1 (Orchestrator must be functional)
**Status:** Planning

---

## Objective

Enable content processing workflows to invoke Paladin AI agents for content enrichment, analysis, or
action. A `ContentProcessor` implementation wraps `PaladinExecutionService` (or a Battalion pattern)
and integrates into the Orchestrator's `process_content()` pipeline.

## Background

The architectural vision: ingest a news article → run through content processing (extract text,
summarize) → invoke an AI agent to analyze sentiment and extract key entities → store enriched
results. The `ContentProcessor` trait exists with `process_content()` and `clone_box()` methods. The
`DefaultContentProcessor` is a stub. The bridge between content processing and AI agent execution
doesn't exist.

This Epic establishes the *content → agent* direction of the bidirectional integration. Epic 4
establishes the reverse (*agent → orchestrator*) direction.

## Scope

**In scope:**
- A `PaladinContentProcessor` that bridges a single Paladin agent into content processing.
- A `BattalionContentProcessor` that bridges Battalion patterns (Formation, Phalanx) into content
  processing.
- Wiring content processors into `Orchestrator::process_content()`.
- End-to-end validation of at least one ingestion path.

**Out of scope:**
- The agent → orchestrator bridge (Epic 4).
- New ingestion sources beyond those already in `paladin-content`.

---

## Tasks

### Task 3.1: Implement `PaladinContentProcessor`

**Description:** Create a `ContentProcessor` implementation that:
- Takes a `ContentItem` and an `OrchestrationContext`.
- Converts the content item into a prompt for a Paladin agent.
- Invokes `PaladinExecutionService::execute()` with the prompt.
- Parses the agent's response into a `ContentProcessingResult` with enrichment metadata.

**Implementation notes:**
- Make the prompt template and the output-parsing strategy configurable (e.g., raw text, JSON
  schema, or a typed extraction).
- Keep the processor free of concrete LLM dependencies — depend on `LlmPort` /
  `PaladinExecutionService` so it remains testable with a mock adapter.
- Define how parse failures are surfaced (typed error vs. degraded result with diagnostics).

**Deliverables:**
- `PaladinContentProcessor` struct implementing `ContentProcessor`.
- Configurable: which Paladin configuration to use, what prompt template, what output parsing
  strategy.
- Unit tests with mock LLM adapter.

**Acceptance criteria:**
- A `ContentItem` is converted into a prompt, executed, and parsed into a `ContentProcessingResult`.
- Enrichment metadata is attached to the result.
- A malformed agent response yields the defined error/degraded-result behavior (asserted in a test).

---

### Task 3.2: Implement `BattalionContentProcessor`

**Description:** Create a `ContentProcessor` implementation that invokes a Battalion pattern (e.g., a
Phalanx of 3 specialist analysts, or a Formation pipeline of summarizer → classifier → entity
extractor).

**Implementation notes:**
- Support both Formation (sequential pipeline) and Phalanx (parallel analysts) patterns.
- Allow configuration via battalion config or a Maneuver flow expression.
- Aggregate multi-agent outputs into a single `ContentProcessingResult` with a clearly defined merge
  strategy.

**Deliverables:**
- `BattalionContentProcessor` struct implementing `ContentProcessor`.
- Supports Formation (sequential pipeline) and Phalanx (parallel analysts) patterns.
- Configurable via battalion config or Maneuver flow expression.
- Unit tests with mock agents.

**Acceptance criteria:**
- A Formation configuration runs agents sequentially and threads outputs through the pipeline.
- A Phalanx configuration runs agents in parallel and merges their outputs.
- Aggregation produces a single coherent `ContentProcessingResult`.

---

### Task 3.3: Wire Content Processing into Orchestrator

**Description:** Update `Orchestrator::process_content()` to use the registered `ContentProcessor`
implementations. Enable workflows to include content processing steps that invoke AI agents.

**Implementation notes:**
- Register content processors by name so workflow job definitions can reference them.
- Ensure the content-processing step participates in the Epic 1 job state machine and context
  threading.

**Deliverables:**
- `Orchestrator::register_content_processor()` accepts `PaladinContentProcessor` and
  `BattalionContentProcessor`.
- Workflows can reference content processors by name in their job definitions.
- Integration test: ingest content → process with AI agent → verify enriched output.

**Acceptance criteria:**
- A workflow job that names a registered content processor runs it and stores the enriched result.
- Referencing an unregistered processor returns a typed error.

---

### Task 3.4: Content Ingestion Pipeline Validation

**Description:** Validate the full content ingestion pipeline from the `paladin-content` crate:
`PdfExtractor`, `HttpContentFetcher`, `FileContentListFetcher`, `NewsApiFetcher` → content
aggregation → content analysis → AI agent enrichment → delivery.

**Implementation notes:**
- Choose at least one concrete, deterministic ingestion path for the end-to-end test (e.g., a local
  fixture file or a mocked HTTP fetch) to avoid flaky network dependencies.
- Gate the test behind the `content-processing` feature flag.

**Deliverables:**
- End-to-end integration test for at least one ingestion path (e.g., fetch URL → extract text →
  invoke agent → store result).
- Requires `content-processing` feature flag enabled.

**Acceptance criteria:**
- One full ingestion → enrichment → storage path passes deterministically in CI.

---

## Definition of Done

- `PaladinContentProcessor` and `BattalionContentProcessor` implemented and unit tested.
- Content processors wired into `Orchestrator::process_content()`.
- At least one end-to-end ingestion path validated under the `content-processing` feature.
- `cargo build`, `cargo test`, `cargo clippy -- -D warnings`, and `cargo fmt --check` all pass.
