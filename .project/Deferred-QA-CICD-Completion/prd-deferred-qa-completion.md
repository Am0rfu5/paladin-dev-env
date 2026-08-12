# PRD: Deferred QA & Documentation Completion

> **Correction (dated 2026-08-10, ORCH-03(b)-(e)):** This document names four paths that have
> since moved or changed shape. Each is verified against the tree this session and recorded here;
> original text is retained throughout with no line deleted. The same four relocations are
> recorded as ledger rows in `.planning/ledgers/milestone-09-12.md` (`REQ-listener-service-test-coverage`
> D-13(b), `REQ-llm-tool-calling-port` D-13(c), `REQ-arch-doc-modernization` D-13(d),
> `REQ-asciinema-demos` D-13(e)) — this annotation states the same two paths per relocation as
> those rows.
>
> **(b) `listener_service.rs`:** `src/core/platform/manager/listener_service.rs` (referenced below
> at G8 and elsewhere) is absent — confirmed via `test -f src/core/platform/manager/listener_service.rs`,
> re-run this session (Milestone 6 Epic 2 relocation). The module ships as
> `src/application/services/orchestration/listener.rs` (`pub struct ListenerOrchestrator` at
> `:141`, confirmed present, re-run this session). The 602 LOC / ~57.83% baseline (dated
> 2026-02-14) is stale in **both path and number** — Milestone 9 Epic 2 added
> `tests/event_trigger_pipeline.rs` (5 passing tests) against this module after that baseline was
> struck. Scope real, arithmetic not; not remeasured here. Owner **Phase 15 / DEFER-03**. Full
> detail in the dated correction banner atop `DEFERRED_COVERAGE.md`.
>
> **(c) `llm_port.rs`:** `src/application/ports/output/llm_port.rs` (referenced below at FR-27.1)
> is absent — the whole `src/application/ports/` directory was deleted by Milestone 5 Epic 2,
> confirmed via `test -d src/application/ports`, re-run this session. The port ships as
> `crates/paladin-ports/src/output/llm_port.rs`, confirmed present this session. This is the same
> file **WEB-03** and **WEB-04** (Phase 14) act on.
>
> **(d) `Design_and_Architecture.md`:** `docs/Design/Design_and_Architecture.md` (referenced below
> at item 19 and G3) is absent — confirmed via `test -f docs/Design/Design_and_Architecture.md`,
> re-run this session (Milestone 11 mdbook overhaul). The document ships as
> `docs/src/appendix/design-and-architecture.md` (confirmed present, 311 lines — `wc -l`, re-run
> this session). **Two facts, two labels, not merged:** the *move* is closed — the file exists at
> its new address. The *rewrite* this PRD's FR-26.1 requires is **open** — the file is still 311
> lines, the identical figure this PRD cites below as the pre-rewrite state, with zero occurrences
> of Commander, Council, Conclave, Grove, Maneuver, Sanctum or Sentinel and zero ` ```mermaid ` code
> blocks (all re-confirmed this session), because Milestone 11 Epic 3's non-goals exempt the
> appendix chapter that Epic 2 moved it into. Owner for the rewrite: **Phase 16 / DOCS-02**.
>
> **(e) asciinema / README embedding (FR-26.4, G4):** the clause below targets a `README.md` that
> has since changed shape — Milestone 11 Epic 5 rewrote it into a 193-line landing page (`wc -l
> README.md`, re-run this session) with zero occurrences of "asciinema" or "demo"
> (`grep -ic 'asciinema\|demo' README.md`, re-run this session). `docs/assets/` (the save path FR-26.4
> names) **does not exist** — confirmed via `test -d docs/assets`, re-run this session, which fails;
> this corrects `.planning/intel/requirements.md`'s prior "exists and is empty" framing, superseded
> by the ledger's own re-verification. The path that does exist, `docs/src/assets/`, holds six
> architecture SVGs unrelated to demo content and is not a candidate save location for `.cast`
> recordings. `docs/DEMOS.md` does not exist — confirmed via `test -f docs/DEMOS.md`, re-run this
> session, which fails. Owner **Phase 16 / DOCS-04**.
>
> If a successor path cannot be located in the tree, it is annotated as absent with no successor
> found and the search command recorded, rather than given a guessed replacement (none of the four
> above hit that case — all four successors were located and confirmed present this session).
> Nothing under `.project/Milestone_12-Web-API/` is touched by this correction; Phase 8 already
> annotated it and plan 13-08 owns the route annotations.

> **Correction (dated 2026-08-12, WEB-04):** Epic 27 below (**LLM Tool Calling Implementation &
> Tests**) is recorded as a **future capability improvement — not current functionality and not
> scheduled work** — by `.planning/decisions/0042-llm-native-tool-calling-deferred.md` (ADR-0042).
> The deferred register this epic lives in is a source of future version improvements, not a
> description of what ships today (D-10); recording this as a fourth deferred-register entry was
> explicitly rejected (D-11) — ADR-0042 plus this banner is the whole record. Every line of Epic
> 27's original text below is retained; five sites carry their own inline pointer to ADR-0042: the
> user-story block, the functional-requirements section, the breaking-change/phased-approach risk
> note, the epic-priority ordering row, and both of Epic 27's open questions (still `Open` —
> answering them is a precondition for ADR-0042's reintroduction trigger, not an afterthought).

## Document Info

| Field | Value |
|-------|-------|
| **PRD ID** | PRD-DQC-001 |
| **Feature Name** | Deferred QA & Documentation Completion |
| **Epic Range** | Epics 25–29 |
| **Priority** | High (Next Sprint) |
| **Origin** | Epic 24 Deferred Subtasks |
| **Date** | February 14, 2026 |
| **Status** | Draft |

---

## 1. Introduction / Overview

During the execution of **Epic 24 (Test Hardening, Benchmarks & QA)**, 25 subtasks were deferred due to scope, time constraints, or missing prerequisites. These deferred items fall into five distinct areas:

1. **CI/CD Pipeline Enhancement** — GitHub Actions workflows lack CLI test jobs, benchmark compilation checks, coverage reporting in the main pipeline, and coverage threshold enforcement.
2. **Documentation & Demo Assets** — The architecture document (`Design_and_Architecture.md`) is missing 7 newer systems (Commander, Council, Conclave, Grove, Maneuver, Sanctum, Sentinel), 12 rustdoc warnings remain unresolved, public API documentation is incomplete, and no demo recordings exist.
3. **LLM Tool Calling Implementation & Tests** — All three LLM adapters (OpenAI, DeepSeek, Anthropic) declare tool-calling capabilities in `ProviderCapabilities` but hardcode `function_call: None` and `LlmRequest` has no field for tool definitions. Live API tests for tool calling were skipped.
4. **User Service Test Coverage** — `user_service.rs` (488 LOC) sits at ~4.23% coverage. Requires mock infrastructure for `UserRepositoryPort`, `LogPort`, and `NotificationService`. Detailed in `project/DEFERRED_COVERAGE.md`.
5. **Listener Service Test Coverage & Observability** — `listener_service.rs` (602 LOC) is at ~57.83% coverage. Requires specialized async/concurrency test infrastructure. Detailed in `project/DEFERRED_COVERAGE.md`.
   > **Corrected (dated 2026-08-10, ORCH-03(b)):** the module named above ships as
   > `src/application/services/orchestration/listener.rs`; see the correction banner at the top of
   > this document for the full relocation and stale-baseline record.

This PRD defines five epics (25–29) to systematically close these gaps, bringing the project to full Milestone 3 quality standards.

---

## 2. Goals

| ID | Goal | Measurable Target |
|----|------|--------------------|
| G1 | Automate quality gates in CI | CLI tests, benchmarks, and coverage all run in GitHub Actions with pass/fail enforcement |
| G2 | Establish coverage baselines and thresholds | Coverage report generated on every PR; threshold gate ≥ 78% overall |
| G3 | Update architecture documentation to reflect current system | `Design_and_Architecture.md` covers all 15+ components including Milestone 3 additions |
| G4 | Create demo assets for onboarding and marketing | ≥ 4 asciinema terminal recordings (Paladin, Formation, Council, Grove) |
| G5 | Achieve zero rustdoc warnings | `cargo doc` runs clean with no warnings |
| G6 | Implement functional LLM tool calling | All three providers can send tool definitions and parse tool-call responses |
| G7 | Achieve ≥ 80% test coverage on user_service.rs | Up from 4.23% with comprehensive unit test suite |
| G8 | Achieve ≥ 80% test coverage on listener_service.rs | Up from 57.83% with async/concurrency test suite |
| G9 | Ensure all public APIs have rustdoc comments | 100% of `pub` items in `src/` have `///` documentation |

---

## 3. User Stories

### Epic 25: CI/CD Pipeline Enhancement

**US-25.1**: As a **developer**, I want CI to automatically run CLI snapshot tests on every PR so that CLI output regressions are caught before merge.

**US-25.2**: As a **developer**, I want CI to verify benchmark compilation on every PR so that benchmark bitrot is prevented.

**US-25.3**: As a **maintainer**, I want a coverage report generated on every PR with a minimum threshold gate so that coverage regressions are blocked automatically.

**US-25.4**: As a **developer**, I want `make coverage` and `make coverage-html` targets in the Makefile so that generating coverage locally is a single command.

### Epic 26: Documentation & Demo Assets

**US-26.1**: As a **new contributor**, I want `Design_and_Architecture.md` to document all current systems (including Commander, Council, Conclave, Grove, Maneuver, Sanctum, Sentinel) so that I can understand the full system design.

**US-26.2**: As a **new user**, I want asciinema demo recordings showing basic Paladin execution, Battalion Formation, Council discussion, and Grove routing so that I can quickly understand the framework's capabilities.

**US-26.3**: As a **developer**, I want zero rustdoc warnings when building documentation so that `cargo doc` is a reliable quality gate.

**US-26.4**: As a **contributor**, I want every public API to have rustdoc comments with examples so that I can learn usage from the documentation.

### Epic 27: LLM Tool Calling Implementation & Tests

**US-27.1**: As an **agent developer**, I want to define tools/functions in an `LlmRequest` and have the LLM provider return structured tool-call responses so that Paladins can invoke Arsenal tools via LLM reasoning.

**US-27.2**: As a **developer**, I want live API integration tests for tool calling across all three providers (OpenAI, DeepSeek, Anthropic) so that tool-calling behavior is verified against real APIs.

**US-27.3**: As a **developer**, I want unit tests for tool-call request serialization and response parsing in each adapter so that the implementation is validated without requiring API keys.

> **Correction (dated 2026-08-12, WEB-04):** neither US-27.1, US-27.2 nor US-27.3 above describes
> current functionality — LLM-native tool calling is recorded as a future capability improvement,
> not built, by ADR-0042 (see the correction banner at the top of this document).

### Epic 28: User Service Test Coverage

**US-28.1**: As a **developer**, I want reusable mock implementations for `UserRepositoryPort`, `LogPort`, and `NotificationService` so that user service tests don't require external services.

**US-28.2**: As a **developer**, I want comprehensive unit tests for user registration, authentication, and profile management so that regressions in critical user flows are caught.

**US-28.3**: As a **security engineer**, I want tests validating password hashing (Argon2), input validation, and authentication edge cases so that security-critical code paths are verified.

### Epic 29: Listener Service Test Coverage & Observability

**US-29.1**: As a **developer**, I want a specialized async testing framework for event listeners so that concurrent and time-dependent behavior can be tested reliably.

**US-29.2**: As a **developer**, I want comprehensive unit tests for event registration, filtering, trigger generation, and batch processing so that the event system is thoroughly validated.

**US-29.3**: As an **operator**, I want concurrency stress tests for the listener service so that race conditions and deadlocks are detected before production.

---

## 4. Functional Requirements

### Epic 25: CI/CD Pipeline Enhancement & Coverage Reporting

> **Source**: Deferred tasks 8.6, 8.9, 8.10, 8.11, 8.12, 8.13
> **Estimated Effort**: 1–2 story points (2–4 days)

#### FR-25.1: CLI Snapshot Test CI Job
1. Add a job to `.github/workflows/ci.yml` that runs `cargo test --test cli` (or equivalent pattern matching all CLI snapshot tests).
2. The job must run on every push and PR to `main` and `develop`.
3. The job must fail the pipeline if any snapshot test fails.
4. The job must use the same Rust toolchain version as the existing `test` job.

#### FR-25.2: Benchmark Compilation CI Job
5. Add a job or step to `.github/workflows/ci.yml` that runs `cargo bench --no-run` to verify benchmark compilation.
6. The job must run on every push and PR (compilation check only; actual benchmarks remain schedule/manual).
7. The job must fail the pipeline if any benchmark fails to compile.

#### FR-25.3: Coverage Reporting in Main CI
8. Add `cargo-llvm-cov` coverage generation to the main `ci.yml` pipeline (currently only in `integration-tests.yml`).
9. Generate an LCOV report and upload to Codecov (or equivalent) on every PR.
10. Configure a coverage threshold gate of **78%** minimum. PRs dropping below this threshold must fail.
11. Generate an HTML coverage report as a downloadable artifact for developer review.

#### FR-25.4: Makefile Targets
12. Add `make coverage` target that runs `cargo llvm-cov --lcov --output-path lcov.info`.
13. Add `make coverage-html` target that runs `cargo llvm-cov --html --output-dir target/coverage`.
14. Add `make test-cli` target that runs CLI snapshot tests.
15. Add `make bench-check` target that runs `cargo bench --no-run`.

#### FR-25.5: Coverage Configuration
16. Add a `.cargo-llvm-cov.toml` or equivalent configuration file if needed.
17. Exclude test files, benchmarks, and examples from coverage calculation.
18. Document the coverage tooling setup in `CONTRIBUTING.md`.

---

### Epic 26: Documentation, Architecture Update & Demo Assets

> **Source**: Deferred tasks 7.9–7.17, 7.28, 7.29
> **Estimated Effort**: 3–5 story points (1–1.5 weeks)

#### FR-26.1: Architecture Document Update

> **Corrected (dated 2026-08-10, ORCH-03(d) / DOCS-02):** item 19's path below is superseded — the
> file ships as `docs/src/appendix/design-and-architecture.md` after the Milestone 11 mdbook
> overhaul. It is still 311 lines, the same figure item 19 cites as the pre-rewrite state, with
> none of the 7 newer subsystems and no Mermaid diagrams. The move is closed; the rewrite this FR
> requires is open, owned by Phase 16 / DOCS-02. See the correction banner at the top of this
> document.

19. Read and audit the current `docs/Design/Design_and_Architecture.md` (311 lines, 10 sections).
20. Add or expand the **AI Agent System** section (currently ~20 lines) to cover all 15+ components:
    - Existing (brief refresh): Paladin, Garrison, Arsenal, Battalion (Formation, Phalanx, Campaign, Chain of Command), Herald, Citadel
    - New (detailed): Commander, Council, Conclave, Grove, Maneuver, Sanctum, Sentinel
21. Add Mermaid architecture diagrams for:
    - Overall system architecture (hexagonal layers)
    - Battalion orchestration patterns (Formation, Phalanx, Campaign, Chain of Command, Council, Grove, Maneuver)
    - Data flow through a Paladin execution cycle
    - Arsenal/MCP tool integration flow
22. Update the **Data Flow** section to include the AI agent execution pipeline (not just content processing).
23. Update the **Deployment Architecture** section (currently marked "Draft") with current Docker Compose and Kubernetes support.
24. Add a **Configuration** section covering `config.yml` structure for LLM providers, Garrison, Arsenal, and Sanctum.
25. Remove or update stale references (original content-management-heavy framing where outdated).

#### FR-26.2: Rustdoc Cleanup
26. Run `cargo doc --no-deps 2>&1` and catalog all warnings.
27. Fix all rustdoc warnings (currently 12 minor formatting issues).
28. Ensure `cargo doc --no-deps` produces zero warnings.
29. Add `cargo doc --no-deps 2>&1 | grep -c warning` check to CI (fail if > 0).

#### FR-26.3: Public API Documentation Audit
30. Enumerate all `pub` items in `src/` that lack `///` documentation.
31. Add rustdoc comments to all undocumented public functions, structs, enums, traits, and type aliases.
32. Add at least one `/// # Examples` code block to all public API entry points (builders, service constructors, port traits).
33. Verify documentation renders correctly with `cargo doc --open`.

#### FR-26.4: asciinema Terminal Recordings
34. Install `asciinema` (or equivalent terminal recording tool).
35. Record demo: **Basic Paladin Execution** — create a Paladin with a system prompt, send a query, display the response. Duration: 30–60 seconds.
36. Record demo: **Battalion Formation** — create 2–3 Paladins in a sequential Formation, show output flowing between them. Duration: 45–90 seconds.
37. Record demo: **Council Discussion** — create a Council with 3 expert Paladins, show multi-turn deliberation and final summary. Duration: 60–120 seconds.
38. Record demo: **Grove Routing** — create a Grove with 3 specialized Trees, show a query being routed to the correct tree with confidence scores. Duration: 45–90 seconds.
39. Save all recordings to `docs/assets/` in `.cast` format (asciinema native).
40. Optionally convert recordings to `.gif` or `.svg` for embedding in README.
41. Update `README.md` to embed or link demo recordings in the appropriate sections (Paladin example, Battalion example, Council example, Grove example).
42. Add a `docs/DEMOS.md` index page listing all available demos with descriptions.

> **Corrected (dated 2026-08-10, ORCH-03(e) / DOCS-04):** items 39-42 target paths and a document
> shape that have changed. `docs/assets/` (item 39's save path) does not exist in the tree —
> confirmed via `test -d docs/assets`, re-run this session, which fails; the path that does exist,
> `docs/src/assets/`, holds six architecture SVGs unrelated to demo content. `docs/DEMOS.md`
> (item 42) does not exist — confirmed via `test -f docs/DEMOS.md`, re-run this session. `README.md`
> (item 41's embedding target) was rewritten by Milestone 11 Epic 5 into a 193-line landing page
> with zero occurrences of "asciinema" or "demo" — a document that has changed shape, not merely
> gained a section. Owner **Phase 16 / DOCS-04**. See the correction banner at the top of this
> document.

---

### Epic 27: LLM Tool Calling Implementation & Tests

> **Source**: Deferred tasks 5.8, 5.13, 5.18
> **Estimated Effort**: 3–5 story points (1–1.5 weeks)
> **Prerequisite**: Tool calling must be _implemented_ in adapters before tests can be written.

#### FR-27.1: LlmRequest Tool Definitions

> **Corrected (dated 2026-08-10, ORCH-03(c) / WEB-04):** item 43's path below is superseded — the
> whole `src/application/ports/` directory was deleted by Milestone 5 Epic 2 (`test -d
> src/application/ports` fails, re-run this session). `LlmRequest` and `LlmPort` now ship at
> `crates/paladin-ports/src/output/llm_port.rs` (confirmed present this session) — the same file
> WEB-03 and WEB-04 (Phase 14) act on. See the correction banner at the top of this document.

43. Add a `tools: Option<Vec<ToolDefinition>>` field to `LlmRequest` (in `src/application/ports/output/llm_port.rs`).
44. Define the `ToolDefinition` struct with fields: `name: String`, `description: String`, `parameters: serde_json::Value` (JSON Schema).
45. Ensure `ToolDefinition` implements `Debug`, `Clone`, `Serialize`, `Deserialize`.

#### FR-27.2: LlmResponse Tool Call Parsing
46. Ensure `LlmResponse.function_call` is populated from actual API responses (currently hardcoded to `None` in all three adapters).
47. Add a `tool_calls: Option<Vec<ToolCall>>` field to `LlmResponse` for multi-tool-call support (OpenAI supports parallel tool calls).
48. Define `ToolCall` struct: `{ id: String, function: FunctionCall }`.

#### FR-27.3: OpenAI Adapter Tool Calling
49. Update `openai_adapter.rs` `generate()` to include `tools` array in the API request body when `LlmRequest.tools` is `Some`.
50. Parse `tool_calls` from the OpenAI response JSON and populate `LlmResponse.tool_calls`.
51. Parse single `function_call` from the OpenAI response for backward compatibility.
52. Handle `finish_reason: "tool_calls"` in addition to existing `"function_call"`.
53. Update `generate_stream()` to handle streamed tool-call deltas.

#### FR-27.4: Anthropic Adapter Tool Calling
54. Update `anthropic_adapter.rs` `generate()` to include `tools` array in the API request body (Anthropic format: `tools: [{name, description, input_schema}]`).
55. Parse `tool_use` content blocks from Anthropic responses and map to `LlmResponse.tool_calls`.
56. Handle `stop_reason: "tool_use"` in addition to existing stop reasons.
57. Update `generate_stream()` to handle streamed `content_block_start`/`content_block_delta` for tool_use blocks.

#### FR-27.5: DeepSeek Adapter Tool Calling
58. Investigate DeepSeek API tool calling support (may mirror OpenAI format).
59. If supported: implement tool calling following the same pattern as OpenAI adapter.
60. If not supported: update `ProviderCapabilities` to accurately report `supports_tool_calling: false` and document the limitation.

#### FR-27.6: Unit Tests for Tool Calling
61. Add unit tests in each adapter file for tool-call request serialization (verify correct JSON structure sent to API).
62. Add unit tests for tool-call response deserialization (mock API responses with tool-call payloads).
63. Add unit tests for multi-tool-call responses (parallel tool calls).
64. Add unit tests for edge cases: empty tool list, malformed tool responses, missing tool call IDs.
65. Test `ProviderCapabilities` accurately reflects actual support.

#### FR-27.7: Live API Integration Tests
66. Add `test_openai_tool_calling` to `tests/integration/llm_live_api_tests.rs` — send a request with a simple tool definition (e.g., `get_weather`), verify the model returns a tool call.
67. Add `test_anthropic_tool_calling` — same test adapted for Anthropic's tool-use format.
68. Add `test_deepseek_tool_calling` — if DeepSeek supports tools; otherwise skip with documented reason.
69. All tool-calling tests must be gated behind `#[cfg(feature = "live-api-tests")]` and `#[ignore]`.
70. Tests must skip gracefully if the provider's API key is not set.

> **Correction (dated 2026-08-12, WEB-04):** the whole FR-27.1 … FR-27.7 section above is unbuilt
> and is now a recorded future capability, per ADR-0042 (see the correction banner at the top of
> this document). The flag-honesty half of the original problem statement in §1 item 3 — all three
> adapters declaring tool-calling capability while hardcoding `function_call: None` — was
> separately closed: `ProviderCapabilities::supports_function_calling` now matches adapter
> behaviour, pinned by a correspondence test
> (`crates/paladin-llm/src/lib.rs::test_capabilities_tool_calling_matches_request_surface`).

---

### Epic 28: User Service Test Coverage

> **Source**: Deferred task 6.5; detailed in `project/DEFERRED_COVERAGE.md`
> **Estimated Effort**: 2–3 story points (3–5 days)

#### FR-28.1: Mock Infrastructure
71. Create `MockUserRepository` implementing `UserRepositoryPort` with in-memory `HashMap` storage.
72. Create `MockLogPort` implementing `LogPort` with a `Vec<LogEntry>` for assertion.
73. Create `MockNotificationService` implementing the notification trait with a sent-messages vector.
74. Place mocks in `tests/common/mocks/` or `tests/unit/mocks/` for reuse across test files.
75. All mocks must be `Send + Sync` for async test compatibility.

#### FR-28.2: Registration Test Suite
76. Test successful user registration (happy path): verify user persisted, welcome email sent, action logged.
77. Test duplicate username rejection.
78. Test duplicate email rejection.
79. Test invalid username formats (too short, too long, special characters).
80. Test invalid email formats.
81. Test password hashing: verify hashed password is not plaintext, verify hash is valid Argon2.

#### FR-28.3: Authentication Test Suite
82. Test successful authentication with correct password.
83. Test failed authentication with incorrect password.
84. Test authentication for non-existent user.
85. Test authentication for deactivated account.
86. Test login attempt tracking (count increments on failure).

#### FR-28.4: Profile Management Test Suite
87. Test profile update (name, bio, etc.).
88. Test email change with verification requirement.
89. Test account activation / deactivation.
90. Test email verification flow.

#### FR-28.5: Query Operations Test Suite
91. Test find user by ID (exists / not found).
92. Test find user by email (exists / not found).
93. Test find by active status (returns only active users).
94. Test find by verification status.
95. Test user count statistics.

#### FR-28.6: Edge Cases & Error Handling
96. Test behavior when repository returns an error (database down simulation).
97. Test behavior when notification service fails (should not block registration).
98. Test concurrent registration attempts with same username.
99. Test Unicode username and password handling.
100. Test empty/whitespace-only inputs.

#### FR-28.7: Coverage Verification
101. Run `cargo llvm-cov` targeting `user_service.rs` module.
102. Verify coverage ≥ 80% for `user_service.rs`.
103. Document any intentionally untested code paths with justification.

---

### Epic 29: Listener Service Test Coverage & Observability

> **Source**: Deferred tasks 6.7–6.12; detailed in `project/DEFERRED_COVERAGE.md`
> **Estimated Effort**: 3–5 story points (1–2 weeks)

#### FR-29.1: Async Test Infrastructure
104. Create `MockEventSource` that emits configurable event sequences with controlled timing.
105. Create `MockTriggerExecutor` that records trigger executions for assertion.
106. Create test utilities for controlling Tokio time (`tokio::time::pause()` / `advance()`).
107. Create test event generators (factory functions for common event payloads).
108. Place all test infrastructure in `tests/common/event_testing/` for reuse.

#### FR-29.2: Event Listener Registration Tests
109. Test registering a listener for a specific event type.
110. Test registering multiple listeners for the same event type.
111. Test unregistering a listener.
112. Test listener lifecycle (registered → active → paused → unregistered).
113. Test registering a listener with complex filter conditions.

#### FR-29.3: Event Processing Tests
114. Test single event delivery to matching listener.
115. Test event delivery to multiple matching listeners.
116. Test event filtering (non-matching events are not delivered).
117. Test batch event processing (multiple events in sequence).
118. Test event ordering guarantees (events delivered in order).

#### FR-29.4: Trigger Generation Tests
119. Test trigger creation from matched event.
120. Test trigger status tracking (created → executing → completed/failed).
121. Test trigger with condition evaluation (only triggers when condition met).
122. Test trigger failure handling and retry behavior.
123. Test trigger execution coordination with `MockTriggerExecutor`.

#### FR-29.5: Concurrency & Stress Tests
124. Test concurrent event emission from multiple producers.
125. Test concurrent listener registration/unregistration during event processing.
126. Test high-volume event burst (1000+ events in rapid succession).
127. Test for deadlocks under contention (Tokio Mutex + RwLock interactions).
128. Test graceful shutdown during active event processing.
129. Use `loom` or manual concurrency testing patterns for race condition detection.

#### FR-29.6: Statistics & Monitoring Tests
130. Test event processing count metrics.
131. Test trigger success/failure rate calculation.
132. Test health check endpoint/method returns correct status.

#### FR-29.7: Coverage Verification
133. Run `cargo llvm-cov` targeting `listener_service.rs` module.
134. Verify coverage ≥ 80% for `listener_service.rs`.
135. Document any intentionally untested code paths with justification.

---

## 5. Non-Goals (Out of Scope)

1. **New feature development** — This PRD covers only testing, CI/CD, and documentation for _existing_ functionality. No new agent capabilities, orchestration patterns, or provider integrations.
2. **Content management system tests** — The `todo!()` stubs found in `content_filtering_service.rs`, `sqlite_content_repository.rs`, and `mysql_content_repository.rs` are part of the content subsystem and out of scope for this PRD.
3. **Commander ignored tests** — The 4 `#[ignore]` tests in `commander.rs` (requiring `MockPaladinPort` error injection) are tracked separately.
4. **Performance optimization** — Benchmarks exist from Epic 24; no new performance work is included.
5. **Kubernetes deployment hardening** — The k8s smoke test exists in CI; production deployment improvements are a separate concern.
6. **Database migration or schema changes** — No changes to MySQL/SQLite schemas.
7. **New LLM provider integrations** — Only existing providers (OpenAI, DeepSeek, Anthropic) are in scope.
8. **Rate limiting and retry logic** — Already implemented in adapters; deferred tasks 5.20/5.21 are confirmed complete.

---

## 6. Design Considerations

### Architecture Document Update (Epic 26)

The current `Design_and_Architecture.md` is 311 lines and content-management-centric. The update should:
- Expand to ~600–800 lines to adequately cover the AI agent system
- Use Mermaid diagrams (GitHub-native rendering) rather than external image dependencies
- Follow the existing section structure but rebalance toward the AI agent system
- Maintain links to standalone component docs (GARRISON.md, ARSENAL.md, etc.)

### Demo Recordings (Epic 26)

- Use `asciinema` for terminal recordings (`.cast` format)
- Each recording should demonstrate a self-contained workflow
- Recordings require live LLM API keys (OpenAI preferred for demos)
- Consider using `asciinema-agg` or `svg-term-cli` to convert `.cast` → `.svg`/`.gif` for README embedding
- Store originals in `docs/assets/recordings/` and rendered versions in `docs/assets/`

### Mock Infrastructure (Epics 28–29)

- Design mocks as reusable components, not per-test one-offs
- Place in `tests/common/mocks/` with a `mod.rs` re-exporting all mocks
- Use `Arc<Mutex<Vec<T>>>` pattern for recording calls in async contexts
- Consider using `mockall` crate if manual mocks become unwieldy (evaluate trade-off vs. compile time)

### Tool Calling Implementation (Epic 27)

- Follow OpenAI's tool-calling format as the canonical model (JSON Schema-based tool definitions)
- Anthropic uses a different format (`input_schema` vs. `parameters`) — adapter must translate
- DeepSeek may use OpenAI-compatible format (confirm via documentation)
- The `ToolDefinition` type should be provider-agnostic (defined in the port layer)
- Provider-specific serialization happens in the adapter layer (hexagonal pattern)

---

## 7. Technical Considerations

### Dependencies

| Dependency | Version | Purpose | Epic |
|------------|---------|---------|------|
| `cargo-llvm-cov` | 0.7+ | Coverage reporting | 25 |
| `asciinema` | 2.4+ | Terminal recording | 26 |
| `mockall` | 0.13+ | Mock generation (optional) | 28, 29 |
| `loom` | 0.7+ | Concurrency testing (optional) | 29 |
| `tokio-test` | (matches tokio) | Async test utilities | 29 |

### Existing Infrastructure

- **CI/CD**: Three GitHub Actions workflows exist (`ci.yml`, `integration-tests.yml`, `release.yml`). Changes should modify existing workflows, not create new ones.
- **Coverage**: `cargo-llvm-cov` is already installed in `integration-tests.yml`; extend to `ci.yml`.
- **Codecov**: Already integrated for integration test coverage; extend to include unit test coverage.
- **Makefile**: Existing targets for `clean-code`, `test-all`, `dev`. New targets should follow the same naming conventions.

### Constraints

- All code must pass `cargo clippy -- -D warnings` and `cargo fmt --check`.
- All public items must have rustdoc comments.
- Test coverage must not decrease existing module coverage.
- Live API tests must remain gated behind `#[cfg(feature = "live-api-tests")]` and `#[ignore]`.
- Mock infrastructure must be `Send + Sync` for async compatibility.
- No `unwrap()` in production code; `unwrap()` acceptable only in tests.

### Risk: Tool Calling Implementation Scope

Epic 27 requires modifying the `LlmPort` trait (adding `tools` to `LlmRequest`), which is a breaking change to the port interface. All adapter implementations must be updated simultaneously. Consider a phased approach:
1. Phase 1: Add `tools` field as `Option<Vec<ToolDefinition>>` (backward compatible — `None` means no tools).
2. Phase 2: Implement tool-call sending in each adapter.
3. Phase 3: Implement tool-call response parsing.
4. Phase 4: Write live API integration tests.

> **Correction (dated 2026-08-12, WEB-04):** the four-step phased approach above is preserved as
> the shape the future work would take if the reintroduction trigger in ADR-0042 fires; the
> breaking-change cost this note flags — modifying the `LlmPort` trait, requiring all three shipped
> adapters and the mock to change together — is part of the recorded reason the capability is
> deferred rather than built now (see the correction banner at the top of this document).

### Codebase TODOs Discovered During Research

The following `todo!()` items were found in the codebase but are **out of scope** for this PRD. They are documented here for awareness:

| File | Line(s) | Description |
|------|---------|-------------|
| `content_filtering_service.rs` | 21–44 | 8× `todo!()` in struct fields |
| `content_llm_analysis_service.rs` | 313 | 1× `todo!()` |
| `sqlite_content_repository.rs` | 646–690 | 8× `todo!()` for ContentList CRUD |
| `mysql_content_repository.rs` | 622–666 | 8× `todo!()` for ContentList CRUD |
| `grove_service.rs` | 535 | Model hardcoded `"gpt-4"` — TODO: Make configurable |
| `council_service.rs` | 733 | TODO: Add registry parameter |
| `planning_service.rs` | 433 | TODO: Ask LLM for expected output |
| `trigger.rs` | multiple | TODO for payload matching and cooldown |
| `file_content_fetcher.rs` | multiple | 4× TODO for video/audio/image processing |
| `service_runner.rs` | 508 | TODO for notification adapters |
| `commander.rs` | 2176–3214 | 4× `#[ignore]` tests requiring mock error injection |

---

## 8. Success Metrics

| Metric | Current | Target | Measurement |
|--------|---------|--------|-------------|
| CI pipeline coverage | Integration only | Unit + Integration + CLI | Coverage report in `ci.yml` |
| Coverage threshold gate | None | ≥ 78% enforced | Codecov PR check |
| `Design_and_Architecture.md` components documented | 8 of 15+ | 15+ of 15+ | Manual audit |
| Rustdoc warnings | 12 | 0 | `cargo doc --no-deps 2>&1 \| grep warning` |
| Public API documentation | Partial | 100% | `cargo doc` + audit script |
| Demo recordings | 0 | ≥ 4 | Files in `docs/assets/` |
| OpenAI tool calling | Stubbed (None) | Functional | Unit + live API tests pass |
| Anthropic tool calling | Stubbed (None) | Functional | Unit + live API tests pass |
| `user_service.rs` coverage | 4.23% | ≥ 80% | `cargo llvm-cov` |
| `listener_service.rs` coverage | 57.83% | ≥ 80% | `cargo llvm-cov` |
| Overall project coverage | ~76–77% | ≥ 80% | `cargo llvm-cov` |

---

## 9. Epic Execution Order & Dependencies

```
Epic 25 (CI/CD)  ─────────────────────────────────────┐
                                                        │
Epic 26 (Docs & Demos) ──────────────────────────┐     │
                                                   │     │
Epic 27 (Tool Calling) ────────┐                   │     │
                                │                   │     │
Epic 28 (user_service) ───┐    │                   │     │
                           │    │                   │     │
Epic 29 (listener_service) │    │                   │     │
                           ▼    ▼                   ▼     ▼
                      Coverage gates enforce via Epic 25 CI
```

**Recommended execution order:**

| Order | Epic | Rationale |
|-------|------|-----------|
| 1st | **Epic 25** (CI/CD) | Establishes quality gates that validate all subsequent work |
| 2nd | **Epic 27** (Tool Calling) | Implementation + tests; highest technical complexity |
| — | **Correction (dated 2026-08-12, WEB-04):** superseded for this epic only — recorded as a deferred future capability by ADR-0042, not scheduled work now; the ordering of Epics 25, 26, 28 and 29 is untouched | see the correction banner at the top of this document |
| 3rd | **Epic 28** (User Service) | Builds reusable mock infrastructure needed by Epic 29 |
| 4th | **Epic 29** (Listener Service) | Leverages mock patterns from Epic 28 |
| 5th | **Epic 26** (Docs & Demos) | Final polish; demos showcase all completed features |

Epics 27, 28, and 29 can also run in parallel if multiple developers are available.

---

## 10. Open Questions

| ID | Question | Impact | Status |
|----|----------|--------|--------|
| OQ-1 | Does DeepSeek's API support tool calling? Their adapter reports `supports_tool_calling: false`. Need to verify current API docs. | Determines scope of Epic 27 for DeepSeek. | Open |
| — | **Correction (dated 2026-08-12, WEB-04):** OQ-1 remains `Open`; its being unanswered is part of the reasoning ADR-0042 records for deferring Epic 27 — answering it is a precondition for the reintroduction trigger, not an afterthought. | see ADR-0042 | Open |
| OQ-2 | Should `mockall` crate be adopted for Epics 28–29, or should mocks remain hand-written? `mockall` adds compile-time cost but reduces boilerplate. | Affects mock infrastructure design. | Open |
| OQ-3 | Should the coverage threshold gate be a hard fail (block merge) or a soft warning initially? Moving from no gate to 78% hard fail could block legitimate PRs during ramp-up. | Affects Epic 25 CI configuration. | Open |
| OQ-4 | Are asciinema recordings acceptable, or does the team prefer a different format (e.g., VHS tape files, Terminalizer, or plain GIFs)? | Affects tooling choice in Epic 26. | Open |
| OQ-5 | Should `LlmRequest.tools` use the OpenAI JSON Schema format as canonical, or should we define our own provider-agnostic schema? | Affects `ToolDefinition` struct design in Epic 27. | Open |
| — | **Correction (dated 2026-08-12, WEB-04):** OQ-5 remains `Open`; its being unanswered is part of the reasoning ADR-0042 records for deferring Epic 27 — answering it is a precondition for the reintroduction trigger, not an afterthought. | see ADR-0042 | Open |
| OQ-6 | What Rust toolchain version should CI pin to? Currently the matrix includes `stable` and `beta`. Should coverage/CLI tests only run on `stable`? | Affects CI job configuration in Epic 25. | Open |

---

## Appendix A: Deferred Task Traceability

This table maps every deferred subtask from Epic 24 to its target epic in this PRD.

| Original Task | Subtask | Description | Target Epic |
|---------------|---------|-------------|-------------|
| 5.0 | 5.8 | OpenAI tool calling live test | **27** |
| 5.0 | 5.13 | DeepSeek tool calling live test | **27** |
| 5.0 | 5.18 | Anthropic tool calling live test | **27** |
| 6.0 | 6.5 | Write unit tests for user_service.rs | **28** |
| 6.0 | 6.11 | Re-generate coverage report | **25** |
| 6.0 | 6.12 | Verify overall project coverage | **25** |
| 6.0 | 6.13 | Generate coverage badge/report | **25** |
| 7.0 | 7.9 | Install asciinema | **26** |
| 7.0 | 7.10 | Record demo: basic Paladin execution | **26** |
| 7.0 | 7.11 | Record demo: Battalion Formation | **26** |
| 7.0 | 7.12 | Record demo: Council discussion | **26** |
| 7.0 | 7.13 | Record demo: Grove routing | **26** |
| 7.0 | 7.14 | Save recordings to docs/assets/ | **26** |
| 7.0 | 7.15 | Update README to link demos | **26** |
| 7.0 | 7.16 | Read Design_and_Architecture.md | **26** |
| 7.0 | 7.17 | Update architecture doc with Milestone 3 | **26** |
| 7.0 | 7.28 | Fix rustdoc warnings | **26** |
| 7.0 | 7.29 | Verify all public APIs have documentation | **26** |
| 8.0 | 8.6 | Generate coverage report | **25** |
| 8.0 | 8.9 | Read .github/workflows/ files | **25** |
| 8.0 | 8.10 | Add CI job for CLI tests | **25** |
| 8.0 | 8.11 | Add CI job for benchmark compilation | **25** |
| 8.0 | 8.12 | Update CI to run coverage reporting | **25** |
| 8.0 | 8.13 | Verify CI configuration syntax | **25** |

**Total deferred subtasks: 25** (mapped across 5 epics)

---

## Appendix B: Related Documents

- [project/DEFERRED_COVERAGE.md](../project/DEFERRED_COVERAGE.md) — Detailed coverage analysis for user_service.rs and listener_service.rs
- [project/tasks-test-hardening-benchmarks-qa.md](../project/tasks-test-hardening-benchmarks-qa.md) — Epic 24 task list (source of deferred items)
- [CONTRIBUTING.md](../CONTRIBUTING.md) — Contributing guidelines including testing standards
- [RELEASE_NOTES_MILESTONE_3.md](../RELEASE_NOTES_MILESTONE_3.md) — Milestone 3 release notes
- [docs/Design/Design_and_Architecture.md](../docs/Design/Design_and_Architecture.md) — Architecture document to be updated
- [.github/workflows/ci.yml](../.github/workflows/ci.yml) — Main CI pipeline
- [.github/workflows/integration-tests.yml](../.github/workflows/integration-tests.yml) — Integration test pipeline
