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

**Block verdict — `tasks-autonomous-agent-features.md`: partially outstanding.** Eleven of the
twelve parent-task clusters (`0.0`–`11.0`) verify against the current tree, several of them
`satisfied by shipped code` despite unchecked or `DEFERRED`-annotated subtasks — the corpus's
dominant checkbox-understates-reality pattern, repeated here for planning integration, handoff
execution and PaladinResult metadata. One cluster genuinely fails: **`8.0` YAML & CLI Configuration
Support** — the CLI flags (`--auto-plan`, `--auto-prompt`, `--dynamic-temp`, `--enable-handoffs`)
are declared but never read anywhere in the codebase, and the CLI's own YAML loader
(`PaladinYamlConfig`) carries no `autonomous` field at all; `config.example.yml` has no autonomous
section. Per D-06, a block verdicts `satisfied by shipped code` only if every cluster verifies;
that bar is not met here, so the block is `partially outstanding` and **`8.0`'s CLI-facing gap is
exactly Phase 6 CLOSE-02's scope for this block and nothing else** — the domain-level config
surface (`PaladinConfig.autonomous`, `AutonomousConfig` with full serde + validation) is itself
satisfied and tested; only the CLI wiring is missing.

`.planning/intel/task-completion-state.md` records **45 open** items for this task list (the
`Milestone_2-Missing_features` section total and the corpus-wide ranking both cite this figure).
Transcribed here, not re-derived: the 45 open items sit under `5.0` (Handoff Infrastructure, 6
open subtasks), `6.0` (Handoff Tool, 9 open subtasks), `7.0` (Integration, 10 open subtasks), `8.0`
(YAML & CLI Configuration, 3 open subtasks — the genuine gap), and a handful of scattered
`9.18`/`11.x` process-commit checkboxes — every one of them verified below against the tree rather
than accepted at face value.

| Parent task | Verdict | Evidence |
|---|---|---|
| 0.0 Create feature branch | satisfied | Development-process step, not a shipped-code capability. `tasks-autonomous-agent-features.md:96-99` records the branch and its own "Epic 14 Summary" section (`:417-421`) records 12 commits and a completion date of 2026-02-02, corroborated by the CHANGELOG's dedicated "Epic 14: Autonomous Agent Features" section at `CHANGELOG.md:1594` |
| 1.0 Foundation - Domain Models & Configuration | satisfied | `MaxLoops::Auto { max_subtasks: u32 }` / `MaxLoops::Fixed(u32)` at `crates/paladin-core/src/platform/container/paladin.rs:42,72`; `TaskPlan`/`Subtask` at `crates/paladin-core/src/platform/container/planning.rs`; `HandoffDecision`/`HandoffStrategy`/`HandoffContext` at `crates/paladin-core/src/platform/container/handoff.rs:11,61,143`; `PlanningError` at `src/application/errors/planning_error.rs:22`, `PromptError` at `src/application/errors/prompt_error.rs:22`, `HandoffError` at `crates/paladin-core/src/platform/container/arsenal/handoff_error.rs:27`; `AutonomousConfig`/`PlanningConfig`/`PromptConfig`/`TemperatureConfig`/`HandoffConfig` at `crates/paladin-core/src/platform/container/autonomous_config.rs:70,135,172,208,262`; exercised by `cargo test --offline -p paladin-ai-core paladin::` — 8/8 passed, `cargo test --offline -p paladin-ai-core planning::` — 10/10 passed, `cargo test --offline -p paladin-ai-core handoff::` — 16/16 passed, `cargo test --offline -p paladin-ai-core arsenal::handoff` — 14/14 passed, `cargo test --offline -p paladin-ai-core autonomous_config::` — 20/20 passed, `cargo test --offline -p paladin-ai application::errors::planning_error application::errors::prompt_error application::errors::handoff_error` — 21/21 passed, all run during this task |
| 2.0 Planning Service (US-14.1) | satisfied | `PlanningService` at `src/application/services/paladin/planning_service.rs` with `create_plan()`, `execute_subtasks()`, `synthesize_results()`; exercised by `cargo test --offline -p paladin-ai application::services::paladin::planning_service` — 11/11 passed including `test_create_plan_basic`, `test_execute_subtasks_with_dependencies`, `test_synthesize_results`, `test_create_plan_enforces_max_subtasks`, run during this task |
| 3.0 Prompt Generation Service (US-14.2) [PRE-IMPLEMENTED] | satisfied | Verified against the tree, not the annotation: `PromptGenerationService` at `src/application/services/paladin/prompt_generation_service.rs:48`, with caching (`cache: Arc<Mutex<HashMap<String,String>>>` at `:53`), `PaladinBuilder::auto_generate_prompt()`/`agent_description()` at `paladin_builder.rs:311,337`; exercised by `cargo test --offline -p paladin-ai application::services::paladin::prompt_generation_service` — 8/8 passed including `test_prompt_caching`, `test_cache_invalidation`, `test_deterministic_generation`, run during this task. The disabled-test-module question (`tests/unit/mod.rs` line 22 mock, `prompt_generation_service_test`) belongs to `REQ-prompt-generation-test-reenable` (Epic 24, plan 05-07) — not re-verified here |
| 4.0 Temperature Service (US-14.3) [PRE-IMPLEMENTED] | satisfied, with caveat | Verified against the tree, not the annotation: `TemperatureService` at `src/application/services/paladin/temperature_service.rs:51` with `calculate_optimal_temperature()`; exercised by `cargo test --offline -p paladin-ai application::services::paladin::temperature_service` — 17/17 passed, run during this task. **Caveat, not a cluster failure:** the shipped classification is 3 categories (`TaskType::{Creative, Analytical, Standard}`, fixed temps 0.85/0.2/0.6) with LLM-based detection as the *default* (`enable_llm_detection: true`), not the ingested PRD's 4-band heuristic-only spec — see `REQ-dynamic-temperature` below for the requirement-level divergence |
| 5.0 Handoff Infrastructure (US-14.4) | satisfied by shipped code | Checkbox unchecked in the source (`:154`) because subtasks `5.17/5.18` (handoff execution), `5.23/5.24` (PaladinResult handoff history) and `5.27/5.28` (integration test) are marked `DEFERRED` — but every one of these shipped: `HandoffService::execute_handoff()` with retry/backoff at `src/application/services/paladin/handoff_service.rs:466`; `PaladinResult.handoff_history: Vec<HandoffRecord>` at `crates/paladin-core/src/platform/container/execution_result.rs:68`; exercised by `cargo test --offline -p paladin-ai --test lib -- unit::handoff_service_test` — 27/27 passed, including `test_execute_handoff_delegates_to_specialist`, `test_execute_handoff_retries_transient_errors`, `test_execute_handoff_detects_circular_handoff`, `test_execute_handoff_enforces_max_depth`, run during this task. This is the dominant corpus pattern again: the record understates the tree |
| 6.0 Handoff Tool (US-14.5) | satisfied by shipped code, with caveat | `HandoffTool::new()`/`get_schema()`/`validate_parameters()` at `crates/paladin-core/src/platform/container/arsenal/handoff_tool.rs:40,52,112`, emitting `handoff_to_agent` with `agent_name`/`message`; exercised by `cargo test --offline -p paladin-ai --test lib -- unit::arsenal::handoff_tool_test` — 7/7 passed, run during this task. **Real caveat, not a cluster failure:** subtasks `6.9/6.10` (`HandoffTool::execute()`) are correctly marked `DEFERRED` — no `execute()` method exists on this struct — but the end-to-end capability (LLM calls a handoff tool → validated → executed via `HandoffService` → result returned, chain tracked) still ships and is tested: `PaladinBuilder::generate_handoff_tool()` at `paladin_builder.rs:1184` independently emits an inline tool named `handoff_to_specialist` (`specialist_name`/`task_description` — the Epic 21/23 v2 shape, see `REQ-handoff-tool-v1` below), and `PaladinExecutionService::is_handoff_tool_call()`/`execute_handoff()` at `paladin_execution_service.rs:1414,1434` dispatch on that same v2 name, exercised by the same 27/27 `handoff_service_test` run above (`test_execute_handoff_delegates_to_specialist`) plus `test_graceful_degradation_no_prompt_service`/`_no_planning_service` — 2/2 passed. The Epic-14-authored `HandoffTool` struct itself is never invoked by the wired auto-registration or dispatch path; see `REQ-handoff-tool-v1`'s row |
| 7.0 Integration with PaladinExecutionService [LARGELY PRE-IMPLEMENTED] | satisfied by shipped code | Checked in the source (`:224`) despite subtasks `7.11-7.20` (`PaladinResult` planning/handoff metadata, orchestration logic, integration tests) marked `DEFERRED` — all shipped: `PaladinResult.plan: Option<TaskPlan>` at `execution_result.rs:60`, `.handoff_history` at `:68`; orchestration Layers 1-3 at `paladin_execution_service.rs:1231` (`apply_layer1_planning`, graceful degradation on `PlanningError`), `:1300` (`apply_layer1_prompt_generation`, graceful degradation on `PromptError`), `:1379` (`apply_layer2_dynamic_temperature`), `:1414,1434` (Layer 3 handoff detection/execution); exercised by `cargo test --offline -p paladin-ai --test lib -- unit::paladin_execution_service_test::test_graceful_degradation` — 2/2 passed (`test_graceful_degradation_no_planning_service`, `test_graceful_degradation_no_prompt_service`), run during this task |
| 8.0 YAML & CLI Configuration Support | genuinely outstanding | Checkbox unchecked in the source (`:245`), and the tree confirms a real gap, not just a stale checkbox: `src/application/cli/commands/agent.rs:79,83,87,91` declares `--auto-plan`/`--auto-prompt`/`--dynamic-temp`/`--enable-handoffs` on `AgentRunArgs`, but `grep -rn '\.auto_plan\b\|\.auto_prompt\b\|\.dynamic_temp\b\|\.enable_handoffs\b' src/` returns **zero** matches outside the struct's own field declarations and `Default` impls — the flags are read by no code, run during this task. `src/application/cli/config/paladin_config.rs`'s `PaladinYamlConfig` (`:40`) has no `autonomous` field, and `config.example.yml` has zero `autonomous` occurrences (`grep -c autonomous config.example.yml` → 0). This matches the source file's own `8.11`/`8.17`/`8.18` subtask annotations ("NEEDS: Integration test with agent run" / "NEEDS: Config merging implementation" / "NEEDS: Handler code to merge CLI args with YAML config") — the task list correctly records this as unfinished. Domain-level config is separately satisfied: see `REQ-autonomous-configuration` below |
| 9.0 Documentation (AUTONOMOUS.md) | satisfied by shipped code, with caveat | Checked in the source (`:270`) despite `docs/AUTONOMOUS.md` not existing at that path (`find . -iname AUTONOMOUS.md` → none, run during this task) — Milestone 11's docs overhaul relocated this content into the mdbook: `docs/src/user-guides/paladin-agents.md:191-260` carries a full "Autonomous Features" section covering planning, auto-prompts, dynamic temperature and handoffs with code examples, corroborated by `grep -c -i autonomous docs/src/user-guides/paladin-agents.md` → 15. **Caveat:** the handoff code example at `paladin-agents.md:249` calls `.with_specialist(Arc::new(...))`, a method that does not exist (`grep -n 'pub fn with_specialist' paladin_builder.rs` → none; the shipped method is `with_handoffs(Vec<Arc<Paladin>>)` at `paladin_builder.rs:859`) — a real doc-accuracy defect, but the "documentation and examples exist" capability claim is still met via this different artefact, matching the Epic 22 precedent (05-05) for docs relocated into mdbook |
| 10.0 Examples Implementation | satisfied | All 5 named examples present: `examples/{autonomous_planning,autonomous_prompt_generation,dynamic_temperature,agent_handoffs,autonomous_full_config}.rs`; exercised by `cargo build --offline --examples -p paladin-ai` — exit 0, all examples compile clean, run during this task |
| 11.0 Testing & Quality Assurance | satisfied | `CHANGELOG.md:1594` carries a full "Epic 14: Autonomous Agent Features" section; zero `#[ignore]` attributes across the Epic 14 test surface checked during this task (`tests/unit/handoff_service_test.rs`, `tests/unit/arsenal/handoff_tool_test.rs`, and the four `application/services/paladin/*.rs` service modules); every scoped `cargo test`/`cargo build` invocation run during Tasks 1-2 of this plan passed clean (0 failures across ~140 Epic-14-scoped test executions) |

| ID | Verdict | Evidence |
|---|---|---|
| REQ-max-loops-auto | present, unproven | `MaxLoops::Auto { max_subtasks: u32 }` exists and is tested in isolation at `crates/paladin-core/src/platform/container/paladin.rs:42,72` (`cargo test --offline -p paladin-ai-core paladin::` — 8/8 passed, including `test_max_loops_auto`, run during this task) — but the requirement's specific behavioral claims are unproven against the wired system: (1) `PaladinBuilder::max_loops(u32)` at `src/application/services/paladin/paladin_builder.rs:285` only ever constructs `MaxLoops::Fixed(max_loops)` — there is no builder path that produces `MaxLoops::Auto` at all; (2) `MaxLoops::Auto` is never read by `PaladinExecutionService` (`grep -n 'is_auto()\|max_loops.is_auto' src/` returns zero call sites, run during this task) — planning is instead gated by a wholly separate `autonomous_planning: bool` field (`paladin_execution_service.rs:1238`), decoupled from `max_loops`; (3) the planning `max_subtasks` cap is hardcoded to `10` in `apply_layer1_planning`'s call to `create_plan(input, 10, model)` (`:1268`), not read from any `MaxLoops::Auto{max_subtasks}` value. The example meant to demonstrate this (`examples/autonomous_planning.rs:69,163`) says so itself: "`Note: In fully integrated version, would use MaxLoops::Auto`" / "`Full MaxLoops::Auto integration coming soon`" |
| REQ-planning-service | satisfied | `PlanningService` at `src/application/services/paladin/planning_service.rs:56` with `create_plan()` (`:112`), `execute_subtasks()` and `synthesize_results()`; task decomposition, dependency ordering and logging (`tracing`/`log` calls throughout) all present; exercised by `cargo test --offline -p paladin-ai application::services::paladin::planning_service` — 11/11 passed including `test_create_plan_basic`, `test_execute_subtasks_with_dependencies`, `test_synthesize_results`, `test_create_plan_enforces_max_subtasks`, `test_planning_failure_invalid_json`, run during this task |
| REQ-prompt-generation-service | satisfied | `PromptGenerationService` at `src/application/services/paladin/prompt_generation_service.rs:48` with `generate_prompt()` (`:100`, includes role/capability generation from `agent_description`), caching (`cache` field `:53`, `invalidate_cache()`/`clear_cache()` `:186,194`), deterministic generation (`temperature: Some(0.0)` `:140`); `PaladinBuilder::auto_generate_prompt()`/`agent_description()`/`regenerate_prompt()` at `paladin_builder.rs:311,337,362`, and a manual `.system_prompt()` call overrides auto-generation (`manual_temperature_override`-style guard, verified by the builder's own override precedence logic); exercised by `cargo test --offline -p paladin-ai application::services::paladin::prompt_generation_service` — 8/8 passed including `test_prompt_caching`, `test_cache_invalidation`, `test_deterministic_generation`, `test_generate_prompt_empty_description`, run during this task. **Not re-verified here:** the disabled `prompt_generation_service_test` module and its `tests/unit/mod.rs:22` mock are `REQ-prompt-generation-test-reenable`'s scope (Epic 24, plan 05-07) — cross-reference rather than duplicate, per this plan's explicit instruction |
| REQ-dynamic-temperature | superseded by shipped code | `TemperatureService` at `src/application/services/paladin/temperature_service.rs:51` with `calculate_optimal_temperature()` (`:78`); `PaladinBuilder::auto_temperature(bool)` at `paladin_builder.rs:397` (not the ingested `dynamic_temperature(bool)` name), no `temperature_bounds(f32, f32)` builder method exists (`grep -n 'pub fn temperature_bounds' paladin_builder.rs` → none, run during this task); exercised by `cargo test --offline -p paladin-ai application::services::paladin::temperature_service` — 17/17 passed, run during this task. The shipped answer diverges from the ingested spec on three points and is recorded as authoritative: (1) 3 task-type categories (`TaskType::{Creative, Analytical, Standard}` at `temperature_service.rs:14`) with fixed temps 0.85/0.2/0.6, not the PRD's 4 bands (Factual 0.1-0.3, Analytical 0.3-0.5, Conversational 0.5-0.7, Creative 0.7-1.0); (2) classification defaults to an LLM call (`enable_llm_detection: true` at `:45`, `detect_task_type_with_llm` at `:120`), not the PRD's heuristics-only, ≤50ms requirement — a heuristic fallback exists (`detect_task_type_heuristic` at `:179`) but is not the default path; (3) selected temperature is not bounded by any configurable min/max — the category-based fixed values are used as-is. A separate, unrelated mechanism also named "dynamic temperature" ships alongside this: `PaladinBuilder::enable_dynamic_temperature(bool)` at `:607` linearly increases temperature toward 1.0 across `max_loops` iterations (Layer 2, `apply_layer2_dynamic_temperature` at `paladin_execution_service.rs:1379`) — a per-loop escalation, not task-type classification, and not what the PRD or this REQ ID describes |
| REQ-handoff-infrastructure | satisfied | `PaladinBuilder::with_handoffs(Vec<Arc<Paladin>>)` at `paladin_builder.rs:859`, `handoff_config(Arc<HandoffConfig>)` at `:894`; `HandoffStrategy::{Automatic, Explicit, Threshold{confidence}}` at `crates/paladin-core/src/platform/container/handoff.rs:11`; `HandoffService::should_handoff()` (task complexity/confidence-based) at `src/application/services/paladin/handoff_service.rs:115`; circular-delegation and max-depth enforcement in `validate_handoff()`/`can_handoff_to()` at `:319,354`; `HandoffContext` (task, history, metadata) at `handoff.rs:61`; `PaladinResult.handoff_history: Vec<HandoffRecord>` at `crates/paladin-core/src/platform/container/execution_result.rs:68`; all handoff decisions logged (`info!`/`debug!`/`warn!` throughout `handoff_service.rs`); exercised by `cargo test --offline -p paladin-ai --test lib -- unit::handoff_service_test` — 27/27 passed including `test_should_handoff_automatic_low_confidence`, `test_validate_handoff_circular_delegation`, `test_validate_handoff_max_depth`, `test_execute_handoff_tracks_chain_depth`, run during this task |
| REQ-handoff-tool-v1 | present, unproven | `HandoffTool` at `crates/paladin-core/src/platform/container/arsenal/handoff_tool.rs:25`, emitting a `handoff_to_agent` tool schema with required `agent_name` (enum) and `message` at `get_schema()` (`:52,63,68,73,78`); `validate_parameters()` at `:112` checks `agent_name` against specialists. Its own unit tests exercise these methods in isolation and pass: `cargo test --offline -p paladin-ai --test lib -- unit::arsenal::handoff_tool_test` — 7/7 passed, run during this task. **Unproven against the wired system, per this plan's variant-group instruction:** nothing in the actual auto-registration or execution-dispatch path calls this struct. `PaladinBuilder::generate_handoff_tool()` (`paladin_builder.rs:1184`) independently builds its own inline tool definition named `handoff_to_specialist` with `specialist_name`/`task_description` — the competing `REQ-handoff-tool-v2` shape (Epic 21, plan 05-12) — and `PaladinExecutionService::is_handoff_tool_call()` (`paladin_execution_service.rs:1414`) dispatches on that same `handoff_to_specialist` name, never `handoff_to_agent`. The requirement's "tool MUST validate agent_name...execute via HandoffService...return the specialist result...track the chain across invocations" claims are proven for the wired v2 shape (see the `6.0` cluster row above and `test_execute_handoff_delegates_to_specialist`), not for this v1 artefact. Recorded, not pre-empting 05-12's `REQ-handoff-tool-v2` row |
| REQ-autonomous-configuration | present, unproven | Domain-level configuration is satisfied and tested: `PaladinConfig.autonomous: Option<AutonomousConfig>` at `crates/paladin-core/src/platform/container/paladin_config.rs:61`, `PaladinConfigBuilder::autonomous()` at `:141` with build-time validation (`:158`); `AutonomousConfig`/`PlanningConfig`/`PromptConfig`/`TemperatureConfig`/`HandoffConfig` all `Serialize`/`Deserialize` with `enabled: false` opt-in defaults (`autonomous_config.rs:70-284`); exercised by `cargo test --offline -p paladin-ai-core paladin_config::tests::test_paladin_config_with_autonomous paladin_config::tests::test_paladin_config_autonomous_validation` — 2/2 passed, run during this task. **Unproven at the CLI surface** — see the `8.0` cluster row above: the CLI's `PaladinYamlConfig` (`src/application/cli/config/paladin_config.rs:40`) has no `autonomous` field, `config.example.yml` has none either, and the four `--auto-*`/`--dynamic-temp`/`--enable-handoffs` CLI flags on `AgentRunArgs` (`agent.rs:79-92`) are read by no code in the tree. "All autonomous features MUST be configurable via `PaladinConfig`" and "opt-in by default" are proven; "YAML MUST support every autonomous feature flag" and "CLI MUST support flags" are not — the citations exist but nothing exercises the CLI-facing half |
| REQ-autonomous-error-handling | satisfied | All three error enums exist: `PlanningError` at `src/application/errors/planning_error.rs:22` (11 variants: `InvalidPlan`, `MaxSubtasksExceeded`, `ExecutionFailed`, `GenerationFailed`, `CircularDependency`, `InvalidDependency`, `SynthesisFailed`, `LlmError`, `EmptyPlan`, `Timeout`, `ConfigurationError`); `PromptError` at `src/application/errors/prompt_error.rs:22` (10 variants: `GenerationFailed`, `InvalidDescription`, `CacheMiss`, `InvalidPrompt`, `LlmError`, `RegenerationFailed`, `ConfigurationError`, `CacheError`, `Timeout`, `PromptTooLong`); `HandoffError` at `crates/paladin-core/src/platform/container/arsenal/handoff_error.rs:27` (re-exported at `src/application/errors/handoff_error.rs:6`, a single canonical type, not a divergence). Graceful degradation — the specific criterion the run-2 ledger left uninspected — is implemented, not just claimed: `apply_layer1_planning()` at `paladin_execution_service.rs:1231` catches `PlanningError` and falls back to non-planning execution (`Err(e) => { warn!(...); None }` at `:1279`), `apply_layer1_prompt_generation()` at `:1300` catches `PromptError` and falls back to the existing system prompt (`:1349`); exercised by `cargo test --offline -p paladin-ai --test lib -- unit::paladin_execution_service_test::test_graceful_degradation` — 2/2 passed (`test_graceful_degradation_no_planning_service`, `test_graceful_degradation_no_prompt_service`), and `cargo test --offline -p paladin-ai application::errors::planning_error application::errors::prompt_error application::errors::handoff_error` — 21/21 passed, both run during this task. All autonomous decisions are logged at INFO level throughout (`info!`/`warn!`/`debug!` calls in every service touched by this plan) |

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

**Block verdict — `tasks-epic22-battalion-commander-hardening.md`: satisfied by shipped code.** All
fifteen parent-task clusters (`0.0`–`14.0`) verify against the current tree, including the three the
source task list still marks open (`3.0`, `4.0`, `5.0`) — Council registry integration, Grove
registry integration and Grove's LLM-based routing all shipped, with passing tests, via commits
`761c49c`, `0cdf8dd` and `5f05db7` respectively; the task list's own checkboxes were simply never
updated. Per D-06, a block verdicts `satisfied by shipped code` only if every cluster verifies; that
bar is met here, so Phase 6's CLOSE-02 has **no work required** for this block.

`.planning/intel/task-completion-state.md` records **81 open** items for this task list (both the
`Milestone_3-Completion` section total and the corpus-wide top-line ranking cite the same figure) —
the largest of the three VERIFY-02 blocks. Transcribed here, not re-derived: every one of the 81 open
checkboxes sits under the three parent-task clusters (`3.0`, `4.0`, `5.0`) verified below, plus the
six explicitly-deferred `14.x` documentation sub-items the source file itself records in its own
"Deferred/Optional Tasks from Task 14.0" section (`tasks-epic22-battalion-commander-hardening.md:446-469`)
— none of the 81 represents unverified forward work once the parent-task capability claims are
checked against the tree.

| Parent task | Verdict | Evidence |
|---|---|---|
| 0.0 Create feature branch | satisfied | Development-process step, not a shipped-code capability. The epic's own commit trail confirms the branch produced this work: `git log --oneline --all --grep "Epic 22"` returns commits `019b1e8`, `bdc63fd`, `b7152df`, `761c49c`, `0cdf8dd`, `5f05db7`, `f9b408d`, all merged into the current tree, run during this task |
| 1.0 Define Paladin Registry trait and implementation (US-22.1 Foundation) | satisfied | `PaladinRegistry` trait (`register`/`get`/`contains`/`list_ids`) at `crates/paladin-ports/src/output/paladin_registry.rs:94`; `RegistryError` at `crates/paladin-core/src/platform/container/registry_error.rs:10`; `HashMapPaladinRegistry` struct at `crates/paladin-battalion/src/in_memory_registry.rs:64`, `impl PaladinRegistry` at `:175`; exercised by `cargo test --offline -p paladin-battalion in_memory_registry::` — 9/9 passed, and `cargo test --offline -p paladin-ports paladin_registry` — 2/2 passed, both run during this task |
| 2.0 Extend error types for Battalion operations | satisfied | `BattalionError::PaladinNotFound`/`GroveRoutingFailed`/`MetadataExportFailed` at `crates/paladin-core/src/platform/container/battalion/mod.rs:813,817,821`, `From<RegistryError>` conversion; exercised by `cargo test --offline -p paladin-ai-core battalion::` — 93/93 passed (including `test_battalion_error_variants`, `test_registry_error_conversion`, `test_new_error_messages_format`), run during this task |
| 3.0 Integrate Paladin Registry into Council service (US-22.1) | satisfied by shipped code | Checkbox unchecked in the source file, but the capability shipped: `registry: Arc<dyn PaladinRegistry>` field at `crates/paladin-battalion/src/council_service.rs:65-66`, resolution logic (`self.registry.get(participant_id)`, returning `PaladinNotFound` on miss) at `:130-160`; shipped via commit `761c49c` "feat: integrate Paladin Registry into Council service"; exercised by `cargo test --offline -p paladin-battalion council_service::` — 10/10 passed including `test_council_resolves_participants` and `test_council_paladin_not_found_error`, run during this task. This is the dominant corpus pattern: the record understates the tree |
| 4.0 Integrate Paladin Registry into Grove service (US-22.1) | satisfied by shipped code | Checkbox unchecked in the source file, but the capability shipped: `registry: Arc<dyn PaladinRegistry>` field at `crates/paladin-battalion/src/grove_service.rs:104-105`, resolution at `:184-190`; Commander populates a per-execution registry before routing to Council (`commander.rs:579-592`) and Grove (`:637-678`); shipped via commit `0cdf8dd` "feat: integrate Paladin Registry into Grove service"; exercised by `cargo test --offline -p paladin-battalion grove_service::` — 15/15 passed including `test_grove_resolves_routed_agent` and `test_grove_paladin_not_found_error`, run during this task |
| 5.0 Implement Grove LLM-based routing (US-22.2) | satisfied by shipped code | Checkbox unchecked in the source file, but the capability shipped in full: `route_by_llm` at `crates/paladin-battalion/src/grove_service.rs:479`, `RoutingResponse` struct at `:60`, `routing_fallback`/`min_confidence` config fields at `crates/paladin-core/src/platform/container/battalion/grove.rs:233,240` with validation exercised by `test_grove_builder_validation_invalid_fallback`/`_invalid_threshold`; shipped via commit `5f05db7` "feat: implement Grove LLM-based routing (Task 5.0)"; exercised by `cargo test --offline -p paladin-battalion grove_service::` — 15/15 passed including all four PRD-named TDD tests (`test_route_with_llm_successful`, `test_route_with_llm_low_confidence`, `test_route_with_llm_invalid_json`, `test_route_with_llm_fallback_to_keyword`), run during this task. **Caveat, not a cluster failure:** `grove_service.rs:537` hardcodes `model: "gpt-4".to_string()` — a real, separately-tracked defect (`codebase/CONCERNS.md` §"Grove Service Model Hardcoded") already owned by **Phase 6 / CLOSE-01** (see the `REQ-grove-llm-routing` row below); it does not defeat this cluster's own capability claim (LLM-based routing with a confidence threshold and configurable fallback), which ships and is fully tested |
| 6.0 Extend BattalionMetadata for enhanced metrics (US-22.3) | satisfied | `TokenUsage` struct at `crates/paladin-core/src/platform/container/battalion/mod.rs:497`; `BattalionMetadata.per_paladin_times: HashMap<String, u64>` at `:584`, `.per_paladin_tokens: HashMap<String, TokenUsage>` at `:587`, `.total_tokens: u64` at `:590` — relocated from the PRD's `battalion/battalion_result.rs` per this ledger's head-note path caveat, not a fresh divergence; exercised by `cargo test --offline -p paladin-ai-core battalion::` (`test_battalion_metadata_serialization`, `test_token_usage_aggregation_calculation`), part of the 93/93 run above |
| 7.0 Implement Phalanx per-paladin metrics collection (US-22.3) | satisfied | Per-Paladin timing/token collection loop at `crates/paladin-battalion/src/phalanx_service.rs:264-300`; exercised by `cargo test --offline -p paladin-battalion phalanx_service::` — 14/14 passed including `test_phalanx_per_paladin_timing`, `test_phalanx_per_paladin_tokens`, `test_phalanx_metrics_with_partial_failures`, run during this task |
| 8.0 Add Commander metadata export configuration (US-22.4) | satisfied | `metadata_output_dir: Option<PathBuf>` field on `BattalionConfig` at `crates/paladin-core/src/platform/container/battalion/mod.rs:54` (the PRD's `CommanderConfig` never existed — already a settled fact per the *Milestone 1 as-shipped ledger*'s RECON-03 resolution, not re-decided here), `validate_metadata_dir()` at `:123`; exercised by `cargo test --offline -p paladin-ai-core battalion::` (`test_battalion_config_with_metadata_dir`, `test_battalion_config_metadata_dir_auto_creates`) and `cargo test --offline -p paladin-battalion commander::` (`test_commander_build_with_valid_metadata_dir`, `test_commander_build_without_metadata_dir`) — all pass |
| 9.0 Implement Commander metadata export logic (US-22.4) | satisfied | `export_metadata()` at `crates/paladin-battalion/src/commander.rs:880`; exercised by `cargo test --offline -p paladin-battalion commander::` — 50/50 passed including `test_metadata_export_creates_file`, `test_metadata_export_correct_naming`, `test_metadata_export_json_structure`, `test_metadata_export_no_dir_configured`, run during this task |
| 10.0 Create MockLlmAdapter test infrastructure (US-22.5) | satisfied | `MockLlmAdapter` struct at `tests/helpers/mock_llm_adapter.rs:66` with `add_response`/`add_success`/`add_failure`/`call_count`/`reset` at `:81,86,91,110,134`, helper functions `create_test_paladin_with_mock`/`create_mock_with_responses` at `:331,354`; exercised by `cargo test --offline -p paladin-ai --test lib -- helpers::mock_llm_adapter` — 10/10 passed, run during this task, including the three PRD-named tests `test_mock_llm_adapter_returns_configured_responses`, `test_mock_llm_adapter_tracks_call_count`, `test_mock_llm_adapter_handles_failures`. **New finding (plan 05-05):** this file carries no explicit `[[test]]` entry in `Cargo.toml`; it is reachable only because `tests/lib.rs` (which declares `pub mod helpers; pub mod integration; pub mod unit;`) is itself auto-discovered by Cargo's default `tests/*.rs` convention as target `lib`. The capability and its tests are real and passing; the wiring is implicit rather than declared, so renaming or deleting `tests/lib.rs` would silently stop compiling roughly 700 tests with no `Cargo.toml` diff to flag it |
| 11.0 Enable and fix Campaign and ChainOfCommand tests (US-22.5 Phase 1) | satisfied | `test_execute_routes_to_campaign_service` and `test_execute_routes_to_chain_service` (renamed from the PRD's `test_execute_campaign`/`test_execute_chain_of_command`) at `crates/paladin-battalion/src/commander.rs:2006,2035`; zero `#[ignore]` attributes remain in the file (`grep -c '#\[ignore\]' crates/paladin-battalion/src/commander.rs` → `0`, run during this task); exercised by `cargo test --offline -p paladin-battalion commander::` — both pass, part of the 50/50 run above |
| 12.0 Enable and fix error handling tests (US-22.5 Phase 2) | satisfied | `test_error_handling_fail_fast`, `test_error_handling_continue_on_error`, `test_error_handling_retry_then_continue`, `test_partial_failure_handling` at `crates/paladin-battalion/src/commander.rs:3041,3077,3110,3150`, none `#[ignore]`d; exercised by `cargo test --offline -p paladin-battalion commander::` — all four pass, part of the 50/50 run above |
| 13.0 Integration testing and final validation | satisfied | Integration suite relocated from the PRD's single `tests/integration/battalion_integration_tests.rs` into `tests/integration/battalion/{campaign,chain_of_command,council,formation,grove,load,phalanx}_integration_test.rs` plus three top-level `tests/integration/battalion_*.rs` files — a path relocation, not a capability gap; wired in via `tests/lib.rs`'s `pub mod integration;`; exercised by `cargo test --offline -p paladin-ai --test lib -- integration::battalion` — 76/76 passed, run during this task |
| 14.0 Update documentation and examples | satisfied | `examples/commander_with_metadata_export.rs` builds clean (`cargo build --offline --example commander_with_metadata_export -p paladin-ai`, exit 0, run during this task) and demonstrates `metadata_output_dir`, `per_paladin_times`, `per_paladin_tokens`; `CHANGELOG.md:1111-1174` carries a full "Epic 22: Battalion & Commander Hardening" section. **Caveat:** the PRD's `docs/BATTALION.md`/`docs/COMMANDER.md` targets don't exist at those paths — Milestone 11's docs overhaul relocated Battalion documentation into `docs/src/{appendix,user-guides,deployment-topologies}/battalion-*.md`, and none of those mdbook pages mention `metadata_output_dir`, `per_paladin_times`, or Paladin Registry by name; the source task list's own "Deferred/Optional Tasks from Task 14.0" section (lines 446-469) records six documentation sub-items as explicitly deferred. The working example plus the CHANGELOG entry clear the D-01 evidence bar; the mdbook content gap is real but does not defeat the "documentation and examples exist" capability claim, which a different, working artefact satisfies |

| ID | Verdict | Evidence |
|---|---|---|
| REQ-paladin-registry-port | satisfied | `PaladinRegistry` trait at `crates/paladin-ports/src/output/paladin_registry.rs:94` (`register`/`get`/`contains`/`list_ids` methods, `Send + Sync` bound); exercised by `cargo test --offline -p paladin-ports paladin_registry` — 2/2 passed (`test_trait_is_object_safe`, `test_registry_error_display`) and by `cargo test --offline -p paladin-battalion in_memory_registry::` — 9/9 passed against the trait's own implementor, both run during this task |
| REQ-paladin-registry-adapter | satisfied | `HashMapPaladinRegistry` struct at `crates/paladin-battalion/src/in_memory_registry.rs:64`, `impl PaladinRegistry for HashMapPaladinRegistry` at `:175` (thread-safe via `RwLock<HashMap<String, Arc<Paladin>>>`); exercised by `cargo test --offline -p paladin-battalion in_memory_registry::` — 9/9 passed including `test_registry_thread_safety`, run during this task |
| REQ-council-grove-registry-resolution | satisfied | Registry resolution shipped in both consumers despite unchecked source-file checkboxes (see the parent-task cluster table above, rows `3.0`/`4.0`): `crates/paladin-battalion/src/council_service.rs:130-160` and `crates/paladin-battalion/src/grove_service.rs:184-190`; exercised by `cargo test --offline -p paladin-battalion council_service::` — 10/10 passed and `cargo test --offline -p paladin-battalion grove_service::` — 15/15 passed, both run during this task |
| REQ-grove-llm-routing | genuinely outstanding | `crates/paladin-battalion/src/grove_service.rs:537` hardcodes `model: "gpt-4".to_string(), // TODO: Make configurable` in production code (the `#[cfg(test)]` boundary begins at `:732`); Grove's LLM-based routing capability itself ships and is fully tested (`route_by_llm` at `:479`, all 4 named TDD tests passing per the cluster table's `5.0` row), but the requirement's specific claim — that routing honours the Paladin's *configured* LLM provider — is not met; the model is hardcoded regardless of configuration. Verified open against the tree during this task (`codebase/CONCERNS.md` §"Grove Service Model Hardcoded"). **Owner: Phase 6 / CLOSE-01.** Not fixed here, per this phase's prohibition on editing `.rs` files |
| REQ-phalanx-per-paladin-metrics | satisfied | Per-Paladin timing/token collection at `crates/paladin-battalion/src/phalanx_service.rs:264-300`, populating `BattalionMetadata.per_paladin_times`/`.per_paladin_tokens`/`.total_tokens`; exercised by `cargo test --offline -p paladin-battalion phalanx_service::` — 14/14 passed including `test_phalanx_per_paladin_timing`, `test_phalanx_per_paladin_tokens`, `test_phalanx_metrics_with_partial_failures`, run during this task |
| REQ-battalion-metadata-extension | satisfied | `TokenUsage` struct at `crates/paladin-core/src/platform/container/battalion/mod.rs:497`, `BattalionMetadata.per_paladin_times`/`.per_paladin_tokens`/`.total_tokens` at `:584,587,590` — relocated from the PRD's `battalion/battalion_result.rs` to `battalion/mod.rs`, treated under this ledger's head-note path caveat rather than a fresh divergence; exercised by `cargo test --offline -p paladin-ai-core battalion::` — 93/93 passed including `test_battalion_metadata_serialization` and `test_token_usage_aggregation_calculation`, run during this task |
| REQ-commander-metadata-export | satisfied | `export_metadata()` at `crates/paladin-battalion/src/commander.rs:880`, called after each Battalion execution; exercised by `cargo test --offline -p paladin-battalion commander::` — 50/50 passed including `test_metadata_export_creates_file`, `test_metadata_export_correct_naming`, `test_metadata_export_json_structure`, run during this task, plus the working `examples/commander_with_metadata_export.rs` (builds clean, exit 0) |
| REQ-commander-config-metadata-dir-v3 | satisfied | `metadata_output_dir: Option<PathBuf>` field and `validate_metadata_dir()` on `BattalionConfig` at `crates/paladin-core/src/platform/container/battalion/mod.rs:54,123` (shipped as `BattalionConfig`, not the PRD's `CommanderConfig`, which never existed — already a settled fact per the *Milestone 1 as-shipped ledger*'s RECON-03 resolution, not re-decided here); exercised by `cargo test --offline -p paladin-ai-core battalion::` (`test_battalion_config_with_metadata_dir`, `test_battalion_config_metadata_dir_auto_creates`) and `cargo test --offline -p paladin-battalion commander::` (`test_commander_build_with_valid_metadata_dir`, `test_commander_build_without_metadata_dir`), both run during this task |
| REQ-commander-test-hardening | satisfied | Zero `#[ignore]` attributes measured in `crates/paladin-battalion/src/commander.rs` (`grep -c '#\[ignore\]' crates/paladin-battalion/src/commander.rs` → **0**, run during this task) — every test the PRD names for enablement is present and un-ignored: `test_execute_routes_to_campaign_service`/`test_execute_routes_to_chain_service` at `commander.rs:2006,2035`, `test_error_handling_fail_fast`/`test_error_handling_continue_on_error`/`test_error_handling_retry_then_continue`/`test_partial_failure_handling` at `:3041,3077,3110,3150`; exercised by `cargo test --offline -p paladin-battalion commander::` — 50/50 passed, 0 ignored, run during this task |
| REQ-grove-config-v2 | satisfied | `GroveConfig.routing_fallback: String` and `.min_confidence: f32` at `crates/paladin-core/src/platform/container/battalion/grove.rs:233,240`, with builder-level validation (not a bare `.validate()` method as the PRD sketched, but functionally equivalent); exercised by `cargo test --offline -p paladin-ai-core grove::` — 15/15 passed including `test_grove_builder_validation_invalid_fallback` and `test_grove_builder_validation_invalid_threshold`, run during this task |

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
