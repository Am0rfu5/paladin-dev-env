# Milestone 2-3 cited status ledger

This file **supersedes** `REQUIREMENTS.md`'s `## Milestone 2-3 as-shipped ledger` section (D-21).
That section becomes a pointer to this file. Phases 7, 10 and 13 each add a sibling ledger
(`milestone-04-06.md`, `milestone-07-08.md`, `milestone-09-12.md`) rather than growing
REQUIREMENTS.md further — REQUIREMENTS.md is already ~4,000 lines and five inline sets of
`file:line`-cited verdicts would make it unreadable.

**Primary key: the `REQ-*` requirement ID.** Outstanding task items are nested under the
requirement they belong to, not given their own identifiers — nesting them keeps this ledger
joinable to `REQUIREMENTS.md` and `ROADMAP.md` without inventing new IDs (D-00e).

**Evidence bar.** A `satisfied` verdict requires a `file:line` citation **plus** a named passing
test, example, or command that exercises it. A `file:line` citation with nothing exercising it is
not `satisfied` — it gets its own verdict, `present, unproven` (D-01). This bar applies to all 118
rows below **without exception**, including every row REQUIREMENTS.md's run-2 ledger already
marked `Shipped` or `Shipped (relocated)`: an ingest `Shipped` verdict **is** the bare "the code
exists" claim this bar exists to reject. It is the same bar Phase 1 applied to Milestone 1, and the
same reasoning: "the code exists" has already produced false-positive completions in this corpus
(Milestone 4 Epic 3's task list is fully checked while three CLI-only dependencies remain
unconditional in library builds).

**Path caveat.** Read every row below with two systematic caveats recorded once here, not
repeated per row (D-04): (a) every `src/core|application|infrastructure` path in the run-2 PRDs
predates the Milestone 5 workspace decomposition; the current layout is the ten-library-crate-plus-
facade shape recorded in `.planning/codebase/STRUCTURE.md` and the *Milestone 4-6 as-shipped
ledger*. Citations below use the **current** locations, verified by direct inspection of
`release/v0.7.0` in this worktree. (b) The Milestone-1 benchmark files those PRDs reference have
been relocated into per-crate `benches/` directories. A row whose only divergence from its PRD is
(a) or (b) is `superseded by shipped code` pointing at this note, not a fresh divergence write-up.

## Verdict legend

| Verdict | Meaning |
|---|---|
| `satisfied` | `file:line` citation **and** a named passing test, example, or command exercising it |
| `present, unproven` | `file:line` citation exists, but nothing exercises it |
| `genuinely outstanding` | No shipped code satisfies the requirement |
| `deferred with reason` | Explicitly deferred, with the deferring document and reason cited |
| `superseded by shipped code` | Shipped code answers the requirement differently than the ingested document specified, and the shipped answer is recorded as authoritative |

## Row order and amendment convention

Epic sections below appear in REQUIREMENTS.md's own run-2 order — 11, 12, 13, 20, 14, 15, 16,
17 / 17.5, 18, 19, 21, 22, 23, 24 — and are never re-sorted. Rows within a section appear in the ID
order REQUIREMENTS.md lists them. Later plans replace a row's **Verdict** and **Evidence** cells in
place; they never insert, delete, or reorder rows. The same `file:line` citation may legitimately
appear in more than one row — two requirements citing the same artefact keep separate rows and
separate verdicts, because the `REQ-*` ID, not the citation, is the primary key (D-00e). Amendments
follow D-00g: edit in place, retain superseded text, date every amendment, never a separate
corrections file.

### Epic 11 — Sanctum Memory Foundation (8 IDs)

Epic-level note: `EPIC_11_COMPLETION_SUMMARY.md` claims COMPLETE while recording Qdrant as
DEFERRED and `tasks-sanctum-memory-foundation.md` carries 111 open checkboxes, but
`intel/code-verification.md` records Qdrant as verified shipped, so the 111 count is stale and is
not carried as forward work.

| ID | Verdict | Evidence |
|---|---|---|
| REQ-embedding-port | satisfied | `EmbeddingPort` trait at `crates/paladin-ports/src/output/embedding_port.rs:371` (`Embedding` value type at `:355`, `EmbeddingError` at `:241`); exercised through `OpenAIEmbeddingAdapter`'s trait impl (`crates/paladin-llm/src/openai/embedding.rs:161`) by `cargo test --offline --features openai-embeddings --test openai_embedding_integration` — 8/8 passed, run during this task |
| REQ-openai-embedding-adapter | satisfied | `OpenAIEmbeddingAdapter` struct at `crates/paladin-llm/src/openai/embedding.rs:48`, `impl EmbeddingPort for OpenAIEmbeddingAdapter` at `:161`; exercised by `cargo test --offline --features openai-embeddings --test openai_embedding_integration` — 8/8 passed (mockito-mocked HTTP, no live API key required), run during this task |
| REQ-sanctum-port | satisfied | `SanctumPort` trait at `crates/paladin-ports/src/output/sanctum_port.rs:585`; exercised through `InMemorySanctum`'s trait impl (`crates/paladin-memory/src/sanctum/in_memory_adapter.rs:230`) by `cargo test --offline --test in_memory_sanctum_integration` — 17/17 passed, run during this task |
| REQ-qdrant-sanctum-adapter-v1 | present, unproven | `QdrantSanctumAdapter` struct at `crates/paladin-memory/src/sanctum/qdrant_adapter.rs:59`, `impl SanctumPort for QdrantSanctumAdapter` at `:377`; compiles clean under `cargo build --offline -p paladin-memory --features qdrant` (run during this task, exit 0). Its dedicated exerciser — `tests/integration/qdrant_sanctum_tests.rs`, the `[[test]] name = "qdrant_sanctum_integration"` target at `Cargo.toml:197-199` requiring the `qdrant` feature — carries `#[ignore = "Requires Qdrant running on localhost:6334"]` on all 15 of its `#[tokio::test]` functions and needs a live Qdrant instance; this sandbox has no `docker` binary (`command -v docker` returns nothing), so nothing exercises it here. The Epic 11 "DEFERRED" record is separately confirmed stale — the adapter and its feature wiring are real and compile |
| REQ-in-memory-sanctum | satisfied | `InMemorySanctum` struct at `crates/paladin-memory/src/sanctum/in_memory_adapter.rs:73`, `impl SanctumPort for InMemorySanctum` at `:230`; exercised by `cargo test --offline --test in_memory_sanctum_integration` — 17/17 passed, run during this task |
| REQ-sanctum-domain-model | satisfied | `Memory` struct at `crates/paladin-core/src/platform/container/sanctum.rs:58`, `MemoryType` at `:19`, `MemoryDecayStrategy` at `:38`, `SanctumEntry` at `:204`, `MemoryBuilder` at `:119`; exercised by `cargo test --offline -p paladin-ai-core sanctum::` — 3/3 passed, run during this task (further exercised indirectly by all 17 `in_memory_sanctum_integration` tests and by 10 domain-model unit tests co-located in `qdrant_adapter.rs:587-778`) |
| REQ-sanctum-configuration | satisfied | `SanctumConfig` struct at `crates/paladin-memory/src/config/sanctum.rs:45`, `QdrantSanctumConfig` at `:22`; exercised by `cargo test --offline -p paladin-memory sanctum::` — 13/13 passed, including 6 `config::sanctum::tests::*` cases, run during this task |
| REQ-sanctum-garrison-coexistence | satisfied | Garrison and Sanctum are independently wired module families: `pub mod garrison;` at `crates/paladin-memory/src/lib.rs:42` and `pub mod sanctum;` at `:48`, each with its own directory (`crates/paladin-memory/src/garrison/{in_memory_garrison,sqlite_garrison,token_counter,mod}.rs`, `crates/paladin-memory/src/sanctum/{in_memory_adapter,qdrant_adapter,mod}.rs`) and its own config module (`pub mod garrison;` at `crates/paladin-memory/src/config/mod.rs:3`, `pub mod sanctum;` at `:5`); confirmed by direct listing (`ls crates/paladin-memory/src/{garrison,sanctum,config}`, run during this task) and by the 13/13 pass above, which exercises the Sanctum config path independently of any Garrison test |

### Epic 12 — Sanctum RAG Integration (8 IDs)

| ID | Verdict | Evidence |
|---|---|---|
| REQ-qdrant-sanctum-adapter-v2 | PENDING-VERDICT | 05-09 |
| REQ-paladin-builder-sanctum-integration | PENDING-VERDICT | 05-09 |
| REQ-memory-extraction-strategy | PENDING-VERDICT | 05-09 |
| REQ-rag-retrieval-service | PENDING-VERDICT | 05-09 |
| REQ-rag-config | PENDING-VERDICT | 05-09 |
| REQ-memory-extraction-service | PENDING-VERDICT | 05-09 |
| REQ-execution-service-rag-integration | PENDING-VERDICT | 05-09 |
| REQ-rag-performance-targets | PENDING-VERDICT | 05-09 |

### Epic 13 — Sentinel Vision System (13 IDs)

| ID | Verdict | Evidence |
|---|---|---|
| REQ-vision-content-model | PENDING-VERDICT | 05-08 |
| REQ-vision-format-validation-v1 | PENDING-VERDICT | 05-08 |
| REQ-openai-vision-adapter-v1 | PENDING-VERDICT | 05-08 |
| REQ-anthropic-vision-adapter-v1 | PENDING-VERDICT | 05-08 |
| REQ-vision-capable-llm-trait | PENDING-VERDICT | 05-08 |
| REQ-paladin-vision-api-v1 | PENDING-VERDICT | 05-08 |
| REQ-vision-error-model-v1 | PENDING-VERDICT | 05-08 |
| REQ-vision-security-encryption | PENDING-VERDICT | 05-08 |
| REQ-pdf-extraction | PENDING-VERDICT | 05-08 |
| REQ-document-port | PENDING-VERDICT | 05-08 |
| REQ-vision-cli-and-yaml | PENDING-VERDICT | 05-08 |
| REQ-battalion-vision-integration | PENDING-VERDICT | 05-08 |
| REQ-vision-performance-and-config | PENDING-VERDICT | 05-08 |

### Epic 20 — Vision Pipeline Completion (6 IDs)

| ID | Verdict | Evidence |
|---|---|---|
| REQ-vision-format-validation-v2 | PENDING-VERDICT | 05-08 |
| REQ-openai-vision-adapter-v2 | PENDING-VERDICT | 05-08 |
| REQ-anthropic-vision-adapter-v2 | PENDING-VERDICT | 05-08 |
| REQ-vision-port | PENDING-VERDICT | 05-08 |
| REQ-paladin-vision-api-v2 | PENDING-VERDICT | 05-08 |
| REQ-vision-error-model-v2 | PENDING-VERDICT | 05-08 |

### Epic 14 — Autonomous Agent Features (8 IDs)

| ID | Verdict | Evidence |
|---|---|---|
| REQ-max-loops-auto | PENDING-VERDICT | 05-06 |
| REQ-planning-service | PENDING-VERDICT | 05-06 |
| REQ-prompt-generation-service | PENDING-VERDICT | 05-06 |
| REQ-dynamic-temperature | PENDING-VERDICT | 05-06 |
| REQ-handoff-infrastructure | PENDING-VERDICT | 05-06 |
| REQ-handoff-tool-v1 | PENDING-VERDICT | 05-06 |
| REQ-autonomous-configuration | PENDING-VERDICT | 05-06 |
| REQ-autonomous-error-handling | PENDING-VERDICT | 05-06 |

### Epic 15 — Conclave / Mixture-of-Agents (5 IDs)

| ID | Verdict | Evidence |
|---|---|---|
| REQ-conclave-domain-model | PENDING-VERDICT | 05-09 |
| REQ-conclave-execution-service | PENDING-VERDICT | 05-09 |
| REQ-conclave-commander-strategy | PENDING-VERDICT | 05-09 |
| REQ-conclave-cli-and-yaml | PENDING-VERDICT | 05-09 |
| REQ-conclave-observability | PENDING-VERDICT | 05-09 |

### Epic 16 — Advanced Battalion Patterns: Council & Grove (11 IDs)

| ID | Verdict | Evidence |
|---|---|---|
| REQ-council-domain-model | PENDING-VERDICT | 05-10 |
| REQ-council-turn-strategies | PENDING-VERDICT | 05-10 |
| REQ-council-termination-conditions | PENDING-VERDICT | 05-10 |
| REQ-council-execution-service | PENDING-VERDICT | 05-10 |
| REQ-council-garrison-integration | PENDING-VERDICT | 05-10 |
| REQ-grove-domain-model | PENDING-VERDICT | 05-10 |
| REQ-grove-routing-strategies | PENDING-VERDICT | 05-10 |
| REQ-grove-config-v1 | PENDING-VERDICT | 05-10 |
| REQ-grove-execution-service | PENDING-VERDICT | 05-10 |
| REQ-grove-arsenal-integration | PENDING-VERDICT | 05-10 |
| REQ-council-grove-commander-integration | PENDING-VERDICT | 05-10 |

### Epic 17 / 17.5 — Flow DSL, Maneuver and CLI consolidation (11 IDs)

| ID | Verdict | Evidence |
|---|---|---|
| REQ-flow-dsl-syntax | PENDING-VERDICT | 05-11 |
| REQ-flow-parser | PENDING-VERDICT | 05-11 |
| REQ-flow-expression-ast | PENDING-VERDICT | 05-11 |
| REQ-maneuver-domain-model | PENDING-VERDICT | 05-11 |
| REQ-maneuver-config | PENDING-VERDICT | 05-11 |
| REQ-maneuver-error-strategy-v2 | PENDING-VERDICT | 05-11 |
| REQ-maneuver-execution-service | PENDING-VERDICT | 05-11 |
| REQ-maneuver-commander-integration | PENDING-VERDICT | 05-11 |
| REQ-maneuver-cli | PENDING-VERDICT | 05-11 |
| REQ-flow-visualization | PENDING-VERDICT | 05-11 |
| REQ-maneuver-validation | PENDING-VERDICT | 05-11 |

### Epic 18 — CLI Enhancement & Polish (7 IDs)

| ID | Verdict | Evidence |
|---|---|---|
| REQ-cli-onboarding-wizard | PENDING-VERDICT | 05-10 |
| REQ-cli-setup-check | PENDING-VERDICT | 05-10 |
| REQ-cli-features-discovery | PENDING-VERDICT | 05-10 |
| REQ-cli-muster-command | PENDING-VERDICT | 05-10 |
| REQ-cli-council-command | PENDING-VERDICT | 05-10 |
| REQ-cli-rich-output | PENDING-VERDICT | 05-10 |
| REQ-cli-core-infrastructure | PENDING-VERDICT | 05-10 |

### Epic 19 — Herald & Domain Type Consolidation (5 IDs)

| ID | Verdict | Evidence |
|---|---|---|
| REQ-herald-type-consolidation | PENDING-VERDICT | 05-11 |
| REQ-stream-chunk-complete | PENDING-VERDICT | 05-11 |
| REQ-execution-metadata-complete | PENDING-VERDICT | 05-11 |
| REQ-herald-formatter-autoregistration | PENDING-VERDICT | 05-11 |
| REQ-herald-consolidation-quality-gates | PENDING-VERDICT | 05-11 |

### Epic 21 — Autonomous Agent Completion (7 IDs)

| ID | Verdict | Evidence |
|---|---|---|
| REQ-handoff-tool-v2 | PENDING-VERDICT | 05-12 |
| REQ-autonomous-configurable-model | PENDING-VERDICT | 05-12 |
| REQ-paladin-result-autonomous-metadata | PENDING-VERDICT | 05-12 |
| REQ-autonomous-orchestration-layers | PENDING-VERDICT | 05-12 |
| REQ-handoff-execution-integration | PENDING-VERDICT | 05-12 |
| REQ-autonomous-completion-config-schema | PENDING-VERDICT | 05-12 |
| REQ-autonomous-completion-quality-gates | PENDING-VERDICT | 05-12 |

### Epic 22 — Battalion & Commander Hardening (10 IDs)

| ID | Verdict | Evidence |
|---|---|---|
| REQ-paladin-registry-port | PENDING-VERDICT | 05-05 |
| REQ-paladin-registry-adapter | PENDING-VERDICT | 05-05 |
| REQ-council-grove-registry-resolution | PENDING-VERDICT | 05-05 |
| REQ-grove-llm-routing | PENDING-VERDICT | 05-05 |
| REQ-phalanx-per-paladin-metrics | PENDING-VERDICT | 05-05 |
| REQ-battalion-metadata-extension | PENDING-VERDICT | 05-05 |
| REQ-commander-metadata-export | PENDING-VERDICT | 05-05 |
| REQ-commander-config-metadata-dir-v3 | PENDING-VERDICT | 05-05 |
| REQ-commander-test-hardening | PENDING-VERDICT | 05-05 |
| REQ-grove-config-v2 | PENDING-VERDICT | 05-05 |

### Epic 23 — CLI, Config & Infrastructure Completion (10 IDs)

| ID | Verdict | Evidence |
|---|---|---|
| REQ-cli-garrison-configuration | PENDING-VERDICT | 05-12 |
| REQ-cli-arsenal-configuration | PENDING-VERDICT | 05-12 |
| REQ-mock-llm-adapter | PENDING-VERDICT | 05-12 |
| REQ-cli-tiered-environment-testing | PENDING-VERDICT | 05-12 |
| REQ-scheduler-port | PENDING-VERDICT | 05-12 |
| REQ-content-deliverer-scheduling | PENDING-VERDICT | 05-12 |
| REQ-cli-error-types | PENDING-VERDICT | 05-12 |
| REQ-mock-arsenal-port | PENDING-VERDICT | 05-12 |
| REQ-tool-call-loop-tests | PENDING-VERDICT | 05-12 |
| REQ-mcp-gated-integration-tests | PENDING-VERDICT | 05-12 |

### Epic 24 — Test Hardening, Benchmarks & QA (9 IDs)

| ID | Verdict | Evidence |
|---|---|---|
| REQ-battalion-benchmark-repair | PENDING-VERDICT | 05-07 |
| REQ-prompt-generation-test-reenable | PENDING-VERDICT | 05-07 |
| REQ-timeout-test-hardening | PENDING-VERDICT | 05-07 |
| REQ-qdrant-integration-tests | PENDING-VERDICT | 05-07 |
| REQ-deferred-coverage-review | PENDING-VERDICT | 05-07 |
| REQ-cli-snapshot-testing | PENDING-VERDICT | 05-07 |
| REQ-provider-live-api-tests | PENDING-VERDICT | 05-07 |
| REQ-final-documentation-and-demo | PENDING-VERDICT | 05-07 |
| REQ-epic24-quality-gates | PENDING-VERDICT | 05-07 |
