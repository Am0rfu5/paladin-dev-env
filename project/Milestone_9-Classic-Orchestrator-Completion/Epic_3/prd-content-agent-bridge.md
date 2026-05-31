# PRD: Content → Agent Bridge

**Epic:** 3 — Content → Agent Bridge
**Milestone:** 9 — Classic Orchestrator, Content Pipeline, and Agent-Orchestrator Bridge
**Version Target:** v0.3.0
**Priority:** High
**Status:** Ready for Implementation
**Created:** 2026-05-31
**Document Version:** 1.0

---

## 1. Introduction / Overview

Epic 1 made the `Orchestrator` execute workflows, and Epic 2 proved the time-driven and
event-driven entry points (scheduler, queue, event pipeline) reliable end-to-end. The orchestrator
can now run workflows on a schedule or in response to events — but those workflows cannot yet invoke
AI agents to *do anything intelligent with content*.

The `ContentProcessor` trait already exists in the application layer
(`src/application/services/orchestration/types.rs`) with `name()`, `process_content()`, and
`clone_box()` methods. Today the only implementation is `DefaultContentProcessor`, a stub that sleeps
100 ms and returns a canned JSON blob. `Orchestrator::process_content()` already looks up a processor
by name, opens a session, calls `process_content()`, and closes the session — but there is no
processor that actually runs an agent.

This Epic builds the **content → agent** direction of the bidirectional integration: it implements
two real `ContentProcessor`s that turn a `ContentItem` into an agent prompt, execute a Paladin (or a
Battalion of Paladins), and parse the agent's response back into a `ContentProcessingResult` with
enrichment metadata. It wires them into the orchestrator's existing registration path and validates
one full ingestion → enrichment path. The reverse direction (*agent → orchestrator*) is Epic 4.

**Problem being solved:** The architectural vision — *ingest a news article → extract/summarize text
→ invoke an AI agent to analyze sentiment and extract entities → store the enriched result* — has no
bridge between the content pipeline (`paladin-content`) and the agent runtime (`PaladinExecutionService`,
`paladin-battalion`). This Epic builds that bridge.

---

## 2. Goals

1. Implement `PaladinContentProcessor`: a `ContentProcessor` that converts a `ContentItem` into a
   prompt via a configurable template, executes a single Paladin through `PaladinExecutionService`,
   and parses the response into a `ContentProcessingResult` with enrichment metadata.
2. Make the prompt template and the output-parsing strategy configurable, defaulting to a
   **raw-text** strategy (store the agent's response verbatim as enrichment) while also supporting a
   JSON-extraction strategy. Parse failures degrade gracefully to a diagnostic-bearing result rather
   than panicking.
3. Implement `BattalionContentProcessor`: a `ContentProcessor` that runs a Battalion pattern over the
   content and merges multi-agent outputs into a single coherent `ContentProcessingResult`. Support
   **both** Formation (sequential pipeline) and Phalanx (parallel analysts) patterns.
4. Keep both processors free of concrete LLM dependencies — depend only on the `PaladinPort` /
   `LlmPort` abstractions (and the Battalion execution services, which are themselves
   `PaladinPort`-driven) so they remain unit-testable with a mock adapter.
5. Wire both processors into `Orchestrator::process_content()` via the existing
   `register_content_processor()` path so workflow job definitions can reference them by name;
   referencing an unregistered processor returns a typed `OrchestratorError::ProcessorNotFound`.
6. Validate at least one full, deterministic ingestion → enrichment path under the
   `content-processing` feature, plus an optional live/ignored end-to-end test that exercises a real
   network fetch and a real LLM.
7. Honor hexagonal boundaries and avoid circular crate dependencies (see §7): the new processors
   live in the **root crate** beside the `ContentProcessor` trait, not in `paladin-content`.
8. Ensure `cargo build`, `cargo test`, `cargo clippy -- -D warnings`, and `cargo fmt --check` all
   pass.

---

## 3. User Stories

- **As a framework user**, I want to register a content processor that runs a single AI agent so that
  an ingested article can be summarized and tagged automatically.
- **As a framework user**, I want a content processor that runs a *team* of agents (sequential or
  parallel) so that I can summarize, classify, and extract entities in one pass and get a single
  merged result.
- **As a developer**, I want the processors to depend only on the `PaladinPort`/`LlmPort`
  abstractions so that I can unit-test them with a mock LLM and no network.
- **As an operator**, I want a workflow job that names a registered content processor to run that
  processor and store the enriched result, and to get a clear typed error if the named processor
  isn't registered.
- **As a developer**, I want a malformed agent response to produce a defined degraded result (with
  diagnostics) rather than crash the pipeline, so that one bad LLM reply doesn't take down a batch.
- **As a maintainer**, I want at least one full ingestion path validated deterministically in CI
  (local fixture + mock LLM), with the real-network/real-LLM path available but not run by default,
  so the suite stays fast and non-flaky.

---

## 4. Functional Requirements

### Task 3.1 — Implement `PaladinContentProcessor`

1. The developer **must** implement a `PaladinContentProcessor` struct that implements the
   `ContentProcessor` trait (`name`, `process_content`, `clone_box`) and lives in the root crate
   beside the trait (e.g. `src/application/services/orchestration/processors/`).
2. `process_content()` **must** convert the incoming `ContentItem` into a prompt string using a
   **configurable prompt template** (a template that interpolates the content body and, where
   available, its metadata/title).
3. The processor **must** execute the prompt by calling `PaladinExecutionService::execute(&paladin,
   &prompt)` (single-agent path), where the `Paladin` and execution service are supplied at
   construction time.
4. The processor **must** parse the resulting `PaladinResult` into a `ContentProcessingResult` whose
   `content_id` is the source item's UUID, `processor_name` is this processor's name,
   `processing_time_ms` reflects wall-clock execution time, `success` reflects the outcome, and
   `result_data` carries the parsed enrichment.
5. The processor **must** support a **configurable output-parsing strategy** with at least two
   variants:
   - **Raw text (default):** store the agent's response text verbatim under `result_data` (e.g.
     `{"enrichment": "<agent text>"}`).
   - **JSON extraction:** attempt to parse the agent's response as JSON into `result_data`.
6. The processor **must** attach **enrichment metadata** to the result's `metadata` map (e.g. the
   Paladin/agent name, the model used if available, the parsing strategy applied, and a token/length
   indicator where cheaply available).
7. A **malformed agent response** under the JSON-extraction strategy **must** yield the defined
   degraded-result behavior: `success = false` (or a clearly flagged partial success), `error`
   populated with a diagnostic, and the raw text preserved under `result_data`/`metadata` so no data
   is lost. This behavior **must** be asserted in a unit test.
8. The processor **must not** depend on any concrete LLM adapter; it depends on
   `PaladinExecutionService` / `PaladinPort` / `LlmPort` so unit tests use a mock LLM adapter.
9. The developer **must** add unit tests with a mock LLM adapter covering: (a) a `ContentItem` is
   converted to a prompt, executed, and parsed into a `ContentProcessingResult` with enrichment
   metadata; (b) the raw-text strategy stores the response verbatim; (c) the JSON strategy parses a
   well-formed JSON response; (d) a malformed JSON response produces the degraded result from
   requirement 7.

### Task 3.2 — Implement `BattalionContentProcessor`

10. The developer **must** implement a `BattalionContentProcessor` struct that implements the
    `ContentProcessor` trait and runs a Battalion pattern over the content.
11. The processor **must** support **Formation** (sequential pipeline, e.g. summarizer → classifier
    → entity extractor) by delegating to `FormationExecutionService::execute()`.
12. The processor **must** support **Phalanx** (parallel analysts) by delegating to
    `PhalanxExecutionService::execute()`.
13. The pattern selection **must** be configurable at construction time (e.g. an enum/config choosing
    Formation vs. Phalanx and supplying the corresponding `Formation`/`Phalanx` domain object).
14. The processor **must** aggregate the Battalion's multi-agent output into a single coherent
    `ContentProcessingResult` using a **clearly defined merge strategy** (documented in code): the
    Formation path threads outputs through the pipeline and surfaces the final output; the Phalanx
    path merges parallel analyst outputs (e.g. keyed by agent name) into `result_data`.
15. The processor **must** attach metadata identifying the pattern used and the participating agents.
16. The processor **must** depend only on the Battalion execution services (which are driven by
    `PaladinPort`), so unit tests use mock agents/LLM with no network.
17. The developer **must** add unit tests with mock agents covering: (a) a Formation configuration
    runs agents sequentially and threads outputs through the pipeline into a single result; (b) a
    Phalanx configuration runs agents in parallel and merges their outputs into a single result.

### Task 3.3 — Wire Content Processing into the Orchestrator

18. The developer **must** confirm/extend `Orchestrator::register_content_processor()` so it accepts
    both `PaladinContentProcessor` and `BattalionContentProcessor` (as `Box<dyn ContentProcessor>`)
    and registers them by name.
19. `Orchestrator::process_content(content, processor_name, context)` **must** dispatch to the named
    processor, run it within the existing session lifecycle, and return its
    `ContentProcessingResult`.
20. Referencing an **unregistered** processor name **must** return
    `OrchestratorError::ProcessorNotFound(name)` (typed error, asserted in a test).
21. The content-processing step **must** participate in the existing job/session lifecycle and
    context threading established in Epic 1 (no new lifecycle machinery).
22. The developer **must** add an integration test: register a `PaladinContentProcessor` (mock LLM),
    drive a `ContentItem` through `Orchestrator::process_content()`, and assert the enriched result
    is produced and carries the expected enrichment metadata.

### Task 3.4 — Content Ingestion Pipeline Validation

23. The developer **must** add a **deterministic** end-to-end integration test for at least one
    ingestion path, gated behind the `content-processing` feature flag, that uses a **local fixture**
    (e.g. `FileContentListFetcher` / a fixture file or an in-test `ContentItem`) and a **mock LLM**:
    ingest → extract/aggregate text → invoke the agent via a content processor → produce a stored
    enriched result. This test **must** pass without any network access.
24. The developer **must** add a separate **live** end-to-end test that exercises a *real* ingestion
    path (e.g. `HttpContentFetcher` against a real URL) and a *real* LLM provider, marked `#[ignore]`
    (and/or feature/env-gated, e.g. requiring an API key) so it is **not** run in default CI but can
    be invoked on demand. It **must** be clearly documented how to run it.
25. The deterministic path from requirement 23 **must** assert the full chain produces the expected
    enriched `ContentProcessingResult` (content id preserved, enrichment present, success true).

### Cross-Cutting Requirements

26. All new public items **must** have rustdoc comments; the codebase's `missing_docs`/clippy
    settings **must** continue to pass.
27. All new tests **must** be deterministic and non-flaky: the default suite uses mock LLM adapters
    and local fixtures; any real-network/real-LLM test is `#[ignore]`/gated.
28. No concrete LLM adapter dependency may leak into the processors; they depend only on
    `PaladinPort`/`LlmPort`/Battalion execution services.
29. `cargo build`, `cargo test`, `cargo clippy -- -D warnings`, and `cargo fmt --check` **must** all
    pass (including a build/test with the `content-processing` feature enabled).

---

## 5. Non-Goals (Out of Scope)

1. **No agent → orchestrator bridge.** Triggering orchestrator workflows *from* an agent is Epic 4.
2. **No new ingestion sources.** Only the fetchers/extractors already in `paladin-content`
   (`PdfExtractor`, `HttpContentFetcher`, `FileContentListFetcher`, `NewsApiFetcher`) are used.
3. **No new Battalion patterns.** Only the existing Formation and Phalanx services are bridged;
   Campaign, Chain of Command, Conclave, Council, Grove, Maneuver, and Commander are out of scope for
   this Epic (Maneuver-flow configuration is deferred to an Open Question).
4. **No changes to the `ContentProcessor` trait signature** unless a concrete gap is found while
   implementing; the existing `name`/`process_content`/`clone_box` contract is honored.
5. **No new workflow execution-loop changes.** That was Epic 1.
6. **No public API surface expansion** beyond the two new processors and any minimal configuration
   types they require.
7. **No persistence/storage redesign.** "Store the enriched result" means returning a
   `ContentProcessingResult` through the existing orchestrator/session path; durable persistence
   beyond what already exists is not in scope.

---

## 6. Design Considerations

- **`PaladinContentProcessor`** holds an `Arc<PaladinExecutionService>` and an `Arc<Paladin>` (plus a
  prompt template and a parsing-strategy enum). Its `process_content()` builds the prompt, calls
  `execute()`, and maps `PaladinResult` → `ContentProcessingResult`.
- **`BattalionContentProcessor`** holds a pattern selector that owns either a `Formation` +
  `FormationExecutionService` or a `Phalanx` + `PhalanxExecutionService`. The merge strategy is
  explicit and documented per pattern.
- **Output parsing** is modeled as a small enum (e.g. `OutputParsing::{RawText, Json}`) so new
  strategies (typed extraction) can be added later without changing the trait.
- **Degraded results** reuse the existing `ContentProcessingResult` fields (`success`, `error`,
  `result_data`, `metadata`) — no new error enum is introduced unless a gap is found.
- **Tests** use a mock `LlmPort` adapter (the existing `paladin-llm` mock feature) and construct
  `Paladin`s via `PaladinBuilder`. Battalion tests construct `Formation`/`Phalanx` domain objects
  with mock-backed Paladins.
- **End-to-end test** lives under `tests/` and is gated by `#[cfg(feature = "content-processing")]`;
  the live variant is additionally `#[ignore]`.

---

## 7. Technical Considerations

- **Crate placement (important):** The `ContentProcessor` trait and `OrchestratorError` live in the
  **root crate** (`paladin-ai`), which already depends on `paladin-content` and `paladin-battalion`.
  Implementing the processors *inside* `paladin-content` would require `paladin-content` to depend on
  the root crate (for the trait/error) and on `paladin-battalion` (for the Battalion processor),
  creating a **circular dependency**. Therefore the processors are placed in the **root crate** under
  `src/application/services/orchestration/processors/`, where `PaladinExecutionService`, the Battalion
  services, and the `ContentProcessor` trait are all already reachable. (This overrides the initial
  "inside `paladin-content`" placement preference, which is not buildable.)
- **Hexagonal boundaries:** processors depend on ports (`PaladinPort`/`LlmPort`) and application
  services, never on concrete LLM adapters. `ContentItem`/`OrchestrationContext` are core domain
  types.
- **Reuse existing types:** `ContentProcessingResult`, `OrchestratorError`, `PaladinResult`,
  `BattalionResult` are reused as-is.
- **Feature gating:** the end-to-end ingestion test requires the `content-processing` feature (which
  pulls in `paladin-content` and its sub-features). The live test additionally requires real
  credentials and is `#[ignore]`.
- **Mock LLM:** the `paladin-llm` crate exposes a `mock` feature already enabled in the root crate's
  dependency set; tests use it to drive deterministic agent responses.
- **No `!` in commit messages** and use `set +H &&` before git commits (bash history-expansion
  guard), per repo convention.

---

## 8. Success Metrics

- [ ] `PaladinContentProcessor` converts a `ContentItem` → prompt → agent execution → parsed
      `ContentProcessingResult` with enrichment metadata (unit tests green).
- [ ] Raw-text strategy stores the agent response verbatim; JSON strategy parses well-formed JSON.
- [ ] A malformed agent response under the JSON strategy yields the defined degraded result (asserted
      in a test) without panicking or losing the raw text.
- [ ] `BattalionContentProcessor` runs a Formation sequentially and threads outputs into one result.
- [ ] `BattalionContentProcessor` runs a Phalanx in parallel and merges outputs into one result.
- [ ] Both processors register via `Orchestrator::register_content_processor()` and run through
      `Orchestrator::process_content()`.
- [ ] Referencing an unregistered processor returns `OrchestratorError::ProcessorNotFound`.
- [ ] A deterministic end-to-end ingestion path (local fixture + mock LLM) passes under
      `content-processing` with no network.
- [ ] A live end-to-end test exists, is `#[ignore]`/gated, and is documented.
- [ ] `cargo build`, `cargo test`, `cargo clippy -- -D warnings`, and `cargo fmt --check` all pass.

## Task Checklist

- [ ] **Task 3.1** — `PaladinContentProcessor` implemented, configurable prompt + parsing, degraded
      handling, unit tested with mock LLM.
- [ ] **Task 3.2** — `BattalionContentProcessor` implemented for Formation + Phalanx with a defined
      merge strategy, unit tested with mock agents.
- [ ] **Task 3.3** — Processors wired into `Orchestrator::process_content()`; unregistered-name error
      path validated; integration test green.
- [ ] **Task 3.4** — Deterministic end-to-end ingestion path validated under `content-processing`;
      live path added behind `#[ignore]`/gating.
- [ ] **Quality Gate** — build, test (incl. `content-processing`), clippy `-D warnings`, and fmt all
      pass.

---

## 9. Open Questions

1. **Processor placement — RESOLVED.** Initial preference was to place the processors inside
   `paladin-content`. Because the `ContentProcessor` trait and `OrchestratorError` live in the root
   crate (which already depends on `paladin-content` and `paladin-battalion`), implementing them in
   `paladin-content` would create a circular dependency. **Resolution:** place the processors in the
   root crate under `src/application/services/orchestration/processors/`.
2. **Output parsing default — RESOLVED.** Configurable, defaulting to **raw text** (store the agent
   response verbatim), with a JSON-extraction strategy also available; typed extraction is a future
   extension point on the `OutputParsing` enum.
3. **Battalion patterns — RESOLVED.** Both **Formation** and **Phalanx** are supported in this Epic.
4. **End-to-end test strategy — RESOLVED.** The default CI path is deterministic (local fixture +
   mock LLM) under `content-processing`; a **real network + real LLM** path is added as a separate
   `#[ignore]`/credential-gated test so it does not run in default CI.
5. **Maneuver-flow configuration — OPEN.** The Epic doc mentions configuring the Battalion processor
   "via battalion config or a Maneuver flow expression." This PRD scopes `BattalionContentProcessor`
   to direct Formation/Phalanx configuration; Maneuver-flow-driven configuration is deferred unless
   it proves trivial to add via the existing `paladin-battalion::maneuver` service.
