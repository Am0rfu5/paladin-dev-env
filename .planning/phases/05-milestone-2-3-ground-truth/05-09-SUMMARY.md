---
phase: 05-milestone-2-3-ground-truth
plan: 09
subsystem: docs
tags: [ledger, requirements-traceability, sanctum, rag, conclave, mixture-of-agents, adr-cited]

# Dependency graph
requires:
  - phase: 05-milestone-2-3-ground-truth
    provides: "05-01's ledger scaffold (head notes, verdict legend, 118 PENDING-VERDICT stub rows), Epic 11's fully-cited v1 Qdrant adapter row, and its D-01 evidence bar; 05-02's ADR-0010 (Milestone 3 epic numbering, the Conclave attribution correction)"
provides:
  - "Epic 12 (Sanctum RAG Integration, 8 rows) fully cited in .planning/ledgers/milestone-02-03.md"
  - "Epic 15 (Conclave / Mixture-of-Agents, 5 rows) fully cited, with an epic-level note citing ADR-0010 and recording the 129-open-checkbox count as stale"
  - "REQ-qdrant-sanctum-adapter-v2 closed consistently against Epic 11's v1 row — same shipped adapter, v1's naming/port answered, not v2's"
  - "REQ-execution-service-rag-integration's three previously-uninspected criteria (prompt context section, async non-blocking extraction, three metrics) directly inspected and recorded, including the extraction_triggered dead-write finding"
  - "REQ-conclave-domain-model's shipped ConclaveStatus spelling read from the tree and confirmed to match the PRD, correcting the completion report's own stale transcription"
affects: [05-13, "06 (Verified Gap Closure)"]

# Tech tracking
tech-stack:
  added: []
  patterns:
    - "Same-adapter variant-pair resolution (extended from 05-08): REQ-qdrant-sanctum-adapter-v2 cites the identical file:line as Epic 11's v1 row and states which ingested position (naming, path, port) the shipped code actually answers, rather than treating v1/v2 as independent claims"
    - "Nothing-exercises-the-integration-point finding: a capability whose unit-level pieces (domain model, execution service) are well-tested but whose orchestrator-level wiring (Commander's BattalionStrategy::Conclave arm) has zero exercising test anywhere in the tree is verdicted present, unproven at the integration point even though its dependencies are satisfied"

key-files:
  created: []
  modified:
    - .planning/ledgers/milestone-02-03.md

key-decisions:
  - "REQ-qdrant-sanctum-adapter-v2 verdicted present, unproven, consistent with Epic 11's v1 row: same QdrantSanctumAdapter struct/file, same #[ignore]d 15-test exerciser requiring a live Qdrant this sandbox cannot start (no docker binary). The shipped naming (QdrantSanctumAdapter, qdrant_adapter.rs) and connection shape (port 6334) answer v1's ingested position, not v2's (QdrantSanctum, qdrant_sanctum.rs, port 6333); v2's explicit health-check requirement is not implemented."
  - "REQ-execution-service-rag-integration verdicted present, unproven: the prompt context section exists (titled '## Relevant Context from Memory', not the PRD's exact heading) and is directly tested; async non-blocking extraction is confirmed by code inspection (tokio::spawn without awaiting the JoinHandle) but not test-asserted; of the three required metrics, retrieval_latency_ms and memories_retrieved_count are computed and logged, but extraction_triggered is computed, set twice, and never read anywhere — a dead write silenced with an underscore prefix. SanctumError also lacks the ConnectionError/QueryError/EmbeddingError variants this requirement's acceptance text specifies."
  - "REQ-rag-performance-targets verdicted deferred with reason, citing STATE.md:642's deferred-items table entry (owner: v2, per that entry's own 'Deferred to v2' disposition) — sanctum_benchmarks.rs benchmarks InMemorySanctum CRUD operations but never RagRetrievalService or MemoryExtractionService directly, and no p95 figure exists anywhere in .planning/."
  - "REQ-conclave-domain-model verdicted satisfied, not superseded by shipped code: the shipped ConclaveStatus enum (Success/PartialSuccess/Failed, conclave.rs:350) matches the PRD's spelling exactly. The divergence is in epic-15-completion-report.md:36, which itself misrecords the spelling as 'Completed, PartialSuccess, Failed' — recorded as a nested finding, not as a code/PRD divergence."
  - "REQ-conclave-commander-strategy verdicted present, unproven: BattalionStrategy::Conclave, CommanderBuilder::aggregator(), the default-last-Paladin behavior, and the Auto-strategy keyword gate all exist in commander.rs, but zero test anywhere in the tree exercises Commander with the Conclave strategy — commander.rs's own test module has a test_auto_selects_X_for_..._keywords test for every other strategy except Conclave, and no integration test calls Commander::build()/execute() with BattalionStrategy::Conclave. Also missing: aggregator_index/aggregator_name builder methods, and the PRD's point-weighted Auto-selection scoring (shipped code uses a flat keyword+count gate instead)."
  - "REQ-conclave-observability verdicted present, unproven: the three-level ObservabilityLevel gate is real and branches correctly, but Standard's log omits per-expert times/retry counts and Verbose's log omits expert outputs/token usage/provider details/timestamps — both materially reduced versus the PRD's per-level content spec, and no test asserts any level's actual log content beyond the default-value test."

requirements-completed: []  # VERIFY-01/VERIFY-02 span all of plans 05-01..05-13; not individually completable until 05-13 closes the ledger out

coverage:
  - id: D1
    description: "Epic 12's 8 REQ-* rows filled to the D-01 evidence bar, with REQ-qdrant-sanctum-adapter-v2 closed consistently against Epic 11's v1 row and REQ-execution-service-rag-integration's three previously-uninspected criteria directly inspected"
    verification:
      - kind: unit
        ref: "cargo test --offline -p paladin-memory rag -- 17/17 passed"
      - kind: unit
        ref: "cargo test --offline -p paladin-memory memory_extraction -- 15/15 passed"
      - kind: other
        ref: "cargo build --offline -p paladin-memory --features qdrant -- exit 0 (compile-only evidence for the present-unproven Qdrant row)"
        status: pass
      - kind: integration
        ref: "cargo test --offline --features qdrant --test rag_integration -- 9/9 reported ok (3 Qdrant-gated tests skip cleanly, no docker binary in this sandbox; satisfied verdict rests on the 6 non-Qdrant-gated results)"
      - kind: unit
        ref: "cargo test --offline -p paladin-ai --features cli --lib -- paladin_builder::tests (sanctum/embedding_port/memory_extraction_strategy subset) -- 5/5 passed within a 30-test pass"
      - kind: unit
        ref: "cargo test --offline -p paladin-ai --features cli --lib -- paladin_execution_service::tests::test_rag_context_injection paladin_execution_service::tests::test_rag_context_injection_empty -- 2/2 passed"
    human_judgment: true
    rationale: "Ledger-row plans require a human to confirm the extraction_triggered dead-write finding and the SanctumError variant-name divergence are read correctly from the tree, and that the qdrant-sanctum-adapter-v2/v1 pair genuinely doesn't contradict itself (same class of manual check as prior wave plans 05-01/05-08)."
  - id: D2
    description: "Epic 15's 5 REQ-* rows filled to the D-01 evidence bar, with the epic-level note citing ADR-0010 and the stale 129-checkbox count, and the shipped ConclaveStatus spelling read from the tree"
    verification:
      - kind: unit
        ref: "cargo test --offline -p paladin-ai-core conclave:: -- 10/10 passed"
      - kind: unit
        ref: "cargo test --offline -p paladin-battalion conclave -- 8/8 passed"
      - kind: unit
        ref: "cargo test --offline -p paladin-ai --features cli --lib battalion_template -- 7/7 passed, including test_generate_conclave_template"
      - kind: other
        ref: "cargo check --offline --example conclave_expert_panel --features llm-openai -- exit 0"
        status: pass
      - kind: other
        ref: "grep -rn Conclave tests/integration/commander_integration_tests.rs -- 0 matches (evidence for the present-unproven commander-strategy row)"
        status: pass
    human_judgment: true
    rationale: "Requires a human to confirm the ConclaveStatus spelling correction (shipped/PRD 'Success' vs. the completion report's own 'Completed') is read correctly and not silently transcribed from either recorded source, and that the Commander-level 'nothing exercises it' finding is not overstated given the well-tested domain model and execution service it delegates to."
  - id: D3
    description: "Ledger integrity preserved: exactly 118 REQ-* rows, 14 epic sections, no row inserted/deleted/reordered, no .rs/Cargo.toml/.github file touched"
    verification:
      - kind: other
        ref: "grep -c '^| REQ-' .planning/ledgers/milestone-02-03.md equals 118; grep -c '^### Epic ' equals 14; git diff --stat c381bec0..HEAD -- '*.rs' 'Cargo.toml' '.github/' is empty"
        status: pass
    human_judgment: false

duration: ~80min
completed: 2026-08-04
status: complete
---

# Phase 5 Plan 09: Epic 12/15 Sanctum RAG + Conclave ledger rows Summary

**Filled Epic 12 (Sanctum RAG Integration, 8 rows) and Epic 15 (Conclave / Mixture-of-Agents, 5 rows) in the Milestone 2-3 ledger, finding a dead-written `extraction_triggered` metric and a completion-report transcription error along the way, and verdicting Commander's Conclave-strategy wiring `present, unproven` because nothing in the tree exercises it despite the domain model and execution service it delegates to being well-tested.**

## Performance

- **Duration:** ~80 min (dominated by first-time cold compiles of `paladin-memory`, `paladin-core`, `paladin-battalion`, and the `paladin-ai` facade crate — the facade's first `--features cli` compile alone took ~2m33s; all subsequent scoped test runs against the warm `target/` were sub-second to a few seconds)
- **Tasks:** 2
- **Files modified:** 1 (`.planning/ledgers/milestone-02-03.md`)

## Accomplishments

- Filled all 8 Epic 12 rows: `REQ-qdrant-sanctum-adapter-v2`, `REQ-paladin-builder-sanctum-integration`, `REQ-memory-extraction-strategy`, `REQ-rag-retrieval-service`, `REQ-rag-config`, `REQ-memory-extraction-service`, `REQ-execution-service-rag-integration`, `REQ-rag-performance-targets`.
- Filled all 5 Epic 15 rows: `REQ-conclave-domain-model`, `REQ-conclave-execution-service`, `REQ-conclave-commander-strategy`, `REQ-conclave-cli-and-yaml`, `REQ-conclave-observability`, plus an epic-level note citing ADR-0010 and the stale 129-checkbox count.
- Verdict distribution across the 13 rows: **8 `satisfied`** (`REQ-paladin-builder-sanctum-integration`, `REQ-memory-extraction-strategy`, `REQ-rag-retrieval-service`, `REQ-rag-config`, `REQ-memory-extraction-service`, `REQ-conclave-domain-model`, `REQ-conclave-execution-service`, `REQ-conclave-cli-and-yaml`), **4 `present, unproven`** (`REQ-qdrant-sanctum-adapter-v2`, `REQ-execution-service-rag-integration`, `REQ-conclave-commander-strategy`, `REQ-conclave-observability`), **1 `deferred with reason`** (`REQ-rag-performance-targets`), **0 `genuinely outstanding`**, **0 `superseded by shipped code`**.
- **Closed variant group 7 consistently**: `REQ-qdrant-sanctum-adapter-v2` cites the exact same `QdrantSanctumAdapter` struct and file Epic 11's `REQ-qdrant-sanctum-adapter-v1` row (plan 05-01) cited, and records which ingested position the shipped naming/connection shape actually answers — v1's (`QdrantSanctumAdapter`, `qdrant_adapter.rs`, port 6334), not v2's (`QdrantSanctum`, `qdrant_sanctum.rs`, port 6333). Both rows verdict `present, unproven` for the identical reason (the dedicated 15-test exerciser is `#[ignore]`d needing a live Qdrant this sandbox cannot start — no `docker` binary), so the pair is not contradictory.
- **Directly inspected all three previously-uninspected `REQ-execution-service-rag-integration` criteria** rather than trusting `codebase/ARCHITECTURE.md`'s prose: (1) the relevant-context prompt section exists but is titled `## Relevant Context from Memory`, not the PRD's exact `## Relevant Context`; (2) async non-blocking extraction is real (`tokio::spawn` without awaiting the `JoinHandle`), confirmed by code inspection, not by any test that measures the non-blocking effect; (3) of the three required metrics, `retrieval_latency_ms` and `memories_retrieved_count` are computed and reach a log line, but `extraction_triggered` is computed, set twice, and never read anywhere in the file — a dead write, underscore-prefixed to silence the compiler.
- **Read the shipped `ConclaveStatus` spelling directly from `conclave.rs`** rather than transcribing either recorded source: it is `Success`/`PartialSuccess`/`Failed`, matching the PRD exactly. The divergence is in `epic-15-completion-report.md:36`, whose own checklist claims `(Completed, PartialSuccess, Failed)` — recorded as a nested `**New finding (plan 05-09):**` row rather than changing the row's verdict, since the shipped code does not actually diverge from the PRD.
- **Found that nothing in the tree exercises Commander's Conclave-strategy wiring at all**: `commander.rs`'s own test module has a `test_auto_selects_X_for_..._keywords` test for every other strategy (Formation, Phalanx, Campaign, ChainOfCommand, Council, Grove) but none for Conclave, and no integration test anywhere calls `Commander::build()`/`execute()` with `BattalionStrategy::Conclave`. The only tested Conclave path in the corpus is `ConclaveExecutionService` called directly, bypassing Commander — so `REQ-conclave-commander-strategy` verdicts `present, unproven` even though its two dependencies (domain model, execution service) are both well-tested.
- Recorded the epic-level note for Epic 15 with two sentences per the plan: `intel/code-verification.md` records Conclave as verified shipped against the 129 open checkboxes (stale, not forward work), and the release-notes epic-numbering defect is corrected by citing `.planning/decisions/0010-milestone-3-epic-numbering.md` rather than restating it.
- Ran 8 distinct scoped `cargo test`/`cargo build`/`cargo check` commands, all passing (17+15+10+8+7+5+2 = 64 individual test passes across `paladin-memory`, `paladin-ai-core`, `paladin-battalion`, and the `paladin-ai` facade, plus one clean qdrant-feature build, one clean example compile, and one 9-test integration run with 3 Qdrant-gated tests skipping cleanly), and cited each by name in the row that relies on it.

## Task Commits

1. **Task 1: Fill Epic 12's 8 rows** — `e79b6a3` (docs)
2. **Task 2: Fill Epic 15's 5 rows and the epic-level note** — `b78351a` (docs)

`e79b6a3` — `docs(05-09): fill Epic 12 Sanctum RAG integration ledger rows`
`b78351a` — `docs(05-09): fill Epic 15 Conclave ledger rows and correct completion-report spelling`

_No separate plan-metadata commit — SUMMARY.md is committed by this same worktree per the parallel-execution instructions; STATE.md/ROADMAP.md updates are owned by the orchestrator after the wave merges._

**Worktree hook policy note:** this repo's pre-commit hooks (`cargo fmt`, `cargo clippy --workspace --all-targets --all-features`, both `always_run: true`) would cold-compile the entire 12-crate workspace on every commit including this markdown-only one. Per `workflow.worktree_skip_hooks=true`, `--no-verify` was used for both commits, matching plans 05-01/05-05/05-06/05-07/05-08's precedent in this phase.

## Files Created/Modified

- `.planning/ledgers/milestone-02-03.md` — Epic 12 section (8 rows) and Epic 15 section (5 rows plus one epic-level note and one nested `**New finding (plan 05-09):**` row): replaced all 13 `PENDING-VERDICT` stub rows with cited verdicts. No other epic section touched; row count (118) and section count (14) both verified unchanged outside Epics 12/15.

## Decisions Made

See `key-decisions` in the frontmatter for the full, citation-bearing list. Summarized:
- `REQ-qdrant-sanctum-adapter-v2` → `present, unproven`, consistent with Epic 11's v1 row; shipped naming/port answer v1's position, not v2's.
- `REQ-execution-service-rag-integration` → `present, unproven`; all three previously-uninspected criteria directly inspected, with the `extraction_triggered` dead-write and the missing `SanctumError` variant names as the specific unmet sub-clauses.
- `REQ-rag-performance-targets` → `deferred with reason`, citing `STATE.md:642`, owner recorded as "v2" per that entry's own disposition.
- `REQ-conclave-domain-model` → `satisfied`, not `superseded by shipped code` — the shipped spelling matches the PRD; the completion report's own record is what's wrong (nested finding).
- `REQ-conclave-commander-strategy` → `present, unproven` — zero exercising test anywhere for the Commander-level Conclave wiring, plus two named implementation gaps (`aggregator_index`/`aggregator_name`, point-weighted Auto scoring).
- `REQ-conclave-observability` → `present, unproven` — the three-level gate exists and branches, but the per-level log content is a materially reduced subset of the PRD spec, untested beyond the default-value test.

## Deviations from Plan

**1. [Process] Split the single end-of-plan commit into two task-level commits.** The plan's Task 1 action says "Do not commit yet — this plan commits once, after Task 2," and Task 2's action says "Commit this plan's single file in one commit at the end." This executor's parallel-execution instructions explicitly direct committing early and often within a plan ("as soon as Epic 12's rows are written, commit them; then do Epic 15 and commit again... A prior plan in this phase was lost because it ran long without committing"), citing a concrete prior-plan data-loss risk this override exists to prevent. Followed the more specific, risk-mitigating runtime instruction: committed Epic 12's rows (`e79b6a3`) immediately after Task 1's verification passed, then committed Epic 15's rows (`b78351a`) after Task 2's verification passed — same single ledger file, same net content, two commits instead of one. No content or verification difference from the plan's intent.

**2. [Process] Used `--no-verify` on both commits instead of running the plan's specified full pre-commit hooks.** Task 2's action explicitly says "Do not pass `--no-verify`" and directs a ≥300s-timeout commit expecting `cargo fmt`/`cargo clippy --workspace --all-targets --all-features` to run. This executor's parallel-execution instructions state `workflow.worktree_skip_hooks=true` is configured for this project specifically because those hooks cold-compile the entire 12-crate workspace on every commit including markdown-only ones, and authorize `--no-verify` for every commit in this plan, noting the orchestrator runs the full gate once in the main checkout (warm cache) after the wave merges. Followed the current, config-backed runtime policy over the plan's (likely pre-existing) literal text, matching the identical override already applied and documented by sibling plans 05-01, 05-05, 05-06, 05-07, and 05-08 in this same phase.

Neither deviation changed the ledger's content, verdicts, or evidence — both are process-only accommodations to this worktree's execution environment, consistent with prior plans in this phase.

## Issues Encountered

None beyond the two documented deviations above. All 8 scoped `cargo` commands passed on the first run; no flaky or retried invocations.

## User Setup Required

None — no external service configuration required. (A live Qdrant instance on `localhost:6334` would let the 3 Qdrant-gated `rag_integration` tests and the 15 `qdrant_sanctum_integration` tests actually exercise `REQ-qdrant-sanctum-adapter-v2`/`-v1`, but this plan does not request or require that setup — consistent with Epic 11's precedent.)

## Next Phase Readiness

- Epic 12's and Epic 15's ledger sections are complete: 13 cited `REQ-*` rows, one nested finding row, one epic-level ADR-0010/stale-count note. Ledger integrity preserved for the remaining fan-out plans: row count still 118, section count still 14, no row order disturbed outside Epics 12/15.
- Two named, concrete gaps are available for Phase 6 to scope if prioritized (not raised as new CLOSE-0x requirements themselves, since this plan's `must_haves` only named ownership for the RAG-integration and observability inspections already recorded in their rows): (1) `extraction_triggered`'s dead-write in `paladin_execution_service.rs` — trivial to wire into a log line or drop; (2) Commander's zero test coverage for `BattalionStrategy::Conclave` — the aggregator-validation and Auto-selection logic at `commander.rs:491-546,970-993,1484-1499` has never been exercised end-to-end through `Commander`, only through `ConclaveExecutionService` directly.
- No blockers for the next wave.

## Self-Check: PASSED

- FOUND: `.planning/ledgers/milestone-02-03.md` (modified, Epic 12 and Epic 15 sections)
- FOUND: `.planning/phases/05-milestone-2-3-ground-truth/05-09-SUMMARY.md`
- FOUND: commit `e79b6a3` (Task 1, ledger file only)
- FOUND: commit `b78351a` (Task 2, ledger file only)

---
*Phase: 05-milestone-2-3-ground-truth*
*Completed: 2026-08-04*
