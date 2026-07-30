## Conflict Detection Report

Cumulative report across ingest runs. Run 1 entries are preserved verbatim; run 2
entries are appended within the same three buckets.

Run 1 of 5. MODE=new. Source set: `.project/Milestone_1-MVP` (36 docs: 11 PRD, 25 DOC,
0 ADR, 0 SPEC).

Run 2 of 5. MODE=merge. Source set: `.project/Milestone_2-Missing_features` +
`.project/Milestone_3-Completion` (45 docs: 15 PRD, 30 DOC, 0 ADR, 0 SPEC). Precedence
applied: ADR > SPEC > PRD > DOC. No per-doc precedence overrides were present. No
`locked: true` docs. Existing context checked: PROJECT.md, REQUIREMENTS.md, ROADMAP.md,
STATE.md, intel/SYNTHESIS.md, intel/requirements.md, intel/decisions.md,
intel/constraints.md, intel/context.md, plus intel/task-completion-state.md and
codebase/*.md as authoritative sources.

Supersession is expected in this corpus: later milestones and later Epics deliberately
revise earlier ones. Competing variants below are recorded, not resolved. Where the
shipped code settles a question, the entry points at `.planning/codebase/` rather than
asserting a winner.

### BLOCKERS (0)

None, in either run. There are still no ADR-typed or SPEC-typed documents anywhere in
the 81 documents ingested so far, so no LOCKED-vs-LOCKED contradiction is possible and
there is no locked decision in existing `.planning/` context for an ingest decision to
contradict. No document in run 2 was classified UNKNOWN or low-confidence — all 45
carry `manifest_override: true` and `confidence: high`. Cross-reference cycle detection
found no cycles in run 2 (45 nodes, max depth 2, cap 50). Synthesis proceeded on the
full set.

### WARNINGS (26)

-- Run 1 (Milestone_1-MVP), carried forward unchanged: 8 entries --

[WARNING] Competing unit test coverage targets
  Found: /workspace/.project/Milestone_1-MVP/Epic_1/prd-paladin-domain-foundation.md requires "Unit test coverage >= 80%"; the same >= 80% target is repeated in Epic_2/prd-garrison-memory-system.md (FR10.1), Epic_3/prd-arsenal-tool-system.md, Epic_4/prd-battalion-orchestration.md, Epic_5/prd-commander-strategy-router.md (8.1), Epic_6/prd-provider-expansion.md (REQ-25), Epic_7/prd-citadel-state-persistence.md, Epic_8/prd-herald-output-formatting.md and Epic_10/prd-epic10-validation-documentation.md (7.4, 11)
  Found: /workspace/.project/Milestone_1-MVP/unit-test-improvements/prd-improve-unit-test-coverage.md requires "Overall code coverage exceeds 85% as measured by cargo llvm-cov" — same scope "project-wide unit test coverage gate"
  Impact: Two different numeric quality gates for the same metric. A roadmap built on 80% will be judged incomplete against the 85% PRD, and vice versa. Measured actual is below both (60.88% per Epic_10/task6.0-validation-report.md)
  → Pick one authoritative gate (80% or 85%) or scope 85% explicitly to the unit-test-improvements workstream only. Both variants are preserved as REQ-test-coverage-target-v1 and REQ-test-coverage-target-v2

[WARNING] Competing temperature validation ranges
  Found: /workspace/.project/Milestone_1-MVP/Epic_1/prd-paladin-domain-foundation.md FR-2.3 requires "Builder MUST validate temperature is in range [0.0, 1.0]" and US-2 "Builder rejects invalid values (e.g. temperature > 1.0)"
  Found: /workspace/.project/Milestone_1-MVP/Epic_6/prd-provider-expansion.md REQ-5 requires DeepSeek adapter to support "Temperature control (0.0-2.0)" — same scope "valid temperature range for a Paladin request"
  Impact: A build-time [0.0, 1.0] clamp in PaladinBuilder makes the 0.0-2.0 DeepSeek range unreachable. Either the builder validation is wrong or the DeepSeek requirement cannot be satisfied through the normal Paladin path
  → Decide whether temperature validation is provider-aware (range from ProviderCapabilities) or globally clamped to [0.0, 1.0]. Preserved as REQ-temperature-range-v1 and REQ-temperature-range-v2

[WARNING] Competing BattalionConfig field sets
  Found: /workspace/.project/Milestone_1-MVP/Epic_4/prd-battalion-orchestration.md FR-4.1 requires BattalionConfig with "name, description, timeout_seconds, retry_policy, error_strategy, metadata_output_dir"
  Found: /workspace/.project/Milestone_1-MVP/Epic_5/prd-commander-strategy-router.md FR-7 requires BattalionConfig with "name: String, timeout_seconds: u64, retry_attempts: u32, error_strategy: ErrorStrategy, enable_checkpointing: bool, metadata_output_dir: Option<PathBuf>" — same scope, same type name
  Impact: Divergence is structural, not cosmetic: `retry_policy` (a struct per Epic 4 FR-4.4 with max_attempts/base_delay/max_delay/exponential_backoff/jitter) versus `retry_attempts` (a bare u32); Epic 5 adds `enable_checkpointing` and drops `description`. Epic 5 declares Epic 4 a hard dependency, so one definition must give
  → Reconcile into a single BattalionConfig before routing. Preserved as REQ-battalion-config-v1 and REQ-battalion-config-v2

[WARNING] Competing BattalionResult field sets
  Found: /workspace/.project/Milestone_1-MVP/Epic_4/prd-battalion-orchestration.md FR-4.2 requires BattalionResult with "battalion_id, battalion_name, timestamps, final_output, individual paladin_results, status"
  Found: /workspace/.project/Milestone_1-MVP/Epic_5/prd-commander-strategy-router.md FR-5 requires BattalionResult with "battalion_id, strategy_used, paladin_results, final_output, execution_time_ms, status, metadata" plus "errors: Vec<PaladinError>", where metadata carries strategy_selection_reasoning, strategy_selection_time_ms, per_paladin_times, paladin_success_count, paladin_failure_count, timestamp
  Found: /workspace/.project/Milestone_1-MVP/Epic_8/prd-herald-output-formatting.md FR-7 additionally requires the formatted BattalionResult to expose "Battalion name, ID, and type (Formation, Phalanx, Campaign, Chain of Command)" and "total token usage across all Paladins" — neither the Epic 4 nor Epic 5 field set provides a Battalion type field or aggregated token usage
  Impact: Three consumers assume three different shapes for one shared type. Herald cannot render required fields that neither producer defines
  → Define one BattalionResult covering all three consumers (or add the missing type/token-usage fields explicitly). Preserved as REQ-battalion-result-v1 and REQ-battalion-result-v2; the Herald expectation stays in REQ-herald-battalion-result-fields

[WARNING] Competing minimum Paladin count for Formation
  Found: /workspace/.project/Milestone_1-MVP/Epic_4/prd-battalion-orchestration.md FR-4.5 requires Formation to "validate at least 2 Paladins are provided" and FR-4.8 requires Phalanx to accept a list of ">= 2"
  Found: /workspace/.project/Milestone_1-MVP/Epic_5/prd-commander-strategy-router.md FR-1 requires Commander to validate only that "At least one Paladin is provided", and FR-3 rule 1 routes "Single Paladin: Select Formation (trivial case)" — same scope "minimum Paladin count accepted by Formation"
  Impact: Commander Auto mode with one Paladin routes to Formation, which Epic 4 requires to reject fewer than 2 Paladins. The documented happy path fails validation at runtime
  → Either relax Formation to accept 1 Paladin or change Auto rule 1 to a single-Paladin direct-execution path. Preserved as REQ-formation-min-paladins-v1 and REQ-formation-min-paladins-v2

[WARNING] Competing Herald trait signatures within one PRD
  Found: /workspace/.project/Milestone_1-MVP/Epic_8/prd-herald-output-formatting.md FR-1 requires "format_paladin_result(&self, result: &PaladinResult) -> String", "format_battalion_result(...) -> String", "format_paladin_stream(&self, chunk: &StreamChunk) -> Option<String>", "format_error(...) -> String"
  Found: the same document, section 6.2 Trait Design, specifies fallible signatures returning "Result<String, HeraldError>", renames the streaming method to "format_stream_chunk", and adds "finalize_stream(&self, metadata: &ExecutionMetadata)", "name(&self) -> &str" and "mime_type(&self) -> &str" — same scope "Herald trait method set"
  Impact: Same-document divergence on the public trait contract. FR-10 also requires graceful degradation and error context, which the infallible String-returning FR-1 signatures cannot express. Implementations will diverge depending on which section is read
  → Amend the PRD to one signature set (the fallible 6.2 form satisfies FR-10). Preserved as REQ-herald-trait-v1 and REQ-herald-trait-v2

[WARNING] Contradictory Epic 10 completion state
  Found: /workspace/.project/Milestone_1-MVP/Epic_10/tasks-epic10-validation-documentation.md marks all 103 checklist items complete across parent tasks 0.0 through 6.0, and contains no Task 7.0
  Found: /workspace/.project/Milestone_1-MVP/Epic_10/task6.0-validation-report.md states "Epic 10 progress: 101 of 102 subtasks (99%)" and "Only Task 7.0 (Final Documentation Review) remains", naming a 6-subtask Task 7.0 that does not exist in the task list — same scope "Epic 10 remaining work"
  Impact: Both are DOC-precedence, so precedence rules cannot resolve this. The roadmapper cannot tell whether Epic 10 is finished or has an outstanding documentation-review task, and the subtask totals disagree (102 vs 103)
  → Confirm whether Task 7.0 Final Documentation Review is outstanding and, if so, add it to the task list; otherwise correct the validation report

[WARNING] Contradictory Battalion base module path
  Found: /workspace/.project/Milestone_1-MVP/Epic_4/epic4.md names the Battalion base module "battalion/mod.rs", matching Appendix B of the project plan
  Found: /workspace/.project/Milestone_1-MVP/Paladin Project Completion Plan.md names it "battalion/battalion.rs" in the Epic 4 technical design section, contradicting its own Appendix B — same scope "Battalion base module path"
  Impact: Low. Both docs are DOC-precedence so no precedence tiebreaker applies; a wrong path in generated scaffolding or docs is cheap to fix but will produce inconsistent file references
  → Confirm `battalion/mod.rs` (two of three references) and correct the Epic 4 section of the project plan

-- Run 2 (Milestone_2-Missing_features + Milestone_3-Completion): 18 entries --

[WARNING] Milestone 3 epic numbers name different features in the release notes than in the plan
  Found: /workspace/.project/Milestone_3-Completion/Project_Plan_Milestone_3.md and the six matching epic definitions (Epic_19/epic19.md through Epic_24/epic24.md) define Epic 19 = Herald & Domain Type Consolidation, 20 = Vision Pipeline Completion, 21 = Autonomous Agent Completion, 22 = Battalion & Commander Hardening, 23 = CLI/Config/Infrastructure Completion, 24 = Test Hardening/Benchmarks/QA
  Found: /workspace/.project/Milestone_3-Completion/RELEASE_NOTES_MILESTONE_3.md declares "Epic 19: Conclave Pattern", "Epic 20: Council Pattern", "Epic 21: Grove Pattern", "Epic 22: Maneuver Pattern (Flow DSL)", "Epic 23: Commander Enhancement" and "Epic 24: Test Hardening, Benchmarks & QA" — only Epic 24 agrees, and the four patterns it names belong to Epics 15, 16, 16 and 17 of Milestone 2
  Impact: Highest-impact conflict in run 2. Epic numbers are used as provenance keys throughout the corpus and are referenced from six other documents. Any roadmap phase or REQ-* grouping keyed on "Milestone 3 Epic N" will attach to the wrong feature depending on which document is read
  → Decide which numbering is authoritative for GSD (the plan/epic-definition numbering is used by 8 of the 9 Milestone-3 documents and by every task list), and treat the release-notes numbering as a documentation defect. Do not let both mappings reach ROADMAP.md

[WARNING] Milestone 3 release notes push Vision and Autonomous Agents to Milestone 4
  Found: /workspace/.project/Milestone_3-Completion/Project_Plan_Milestone_3.md schedules Epic 20 (Vision Pipeline Completion) in weeks 2-3 and Epic 21 (Autonomous Agent Completion) in weeks 3-5 of Milestone 3, and /workspace/.project/Milestone_3-Completion/Epic_20/prd-vision-pipeline-completion.md and Epic_21/prd-autonomous-agent-completion.md specify both as Milestone 3 deliverables
  Found: /workspace/.project/Milestone_3-Completion/RELEASE_NOTES_MILESTONE_3.md "What's Next (Milestone 4)" lists "Sentinel Vision: Advanced vision capabilities and multi-modal processing" and "Autonomous Agents: Self-directed agents with planning and goal management" as *planned*, not delivered — same scope, opposite delivery milestone
  Impact: Determines whether vision and autonomous-agent work is history to record or forward work to plan. Getting it wrong either re-plans shipped work as a future phase or drops real remaining work
  → Verify against `.planning/codebase/` (a `vision` feature with OpenAI/Anthropic multimodal support and handoff error types are both mapped as shipped) and against `task-completion-state.md` (Epic 20 has 5 open items, Epic 21 has 12). Record the release-notes statement as a stale forward-look, not as scope

[WARNING] Project-wide test coverage gate now has four competing positions
  Found: run-1 WARNING above already records 80% (nine Milestone-1 Epic PRDs) versus 85% (unit-test-improvements PRD), preserved as REQ-test-coverage-target-v1 and -v2
  Found: /workspace/.project/Milestone_3-Completion/Project_Plan_Milestone_3.md "Cross-Cutting Concerns" specifies layered targets — core domain >= 85%, application services >= 80%, infrastructure adapters >= 70%, CLI commands >= 70%, **overall >= 75%** — a lower project-wide bar than either run-1 variant
  Found: /workspace/.project/Milestone_3-Completion/Epic_24/prd-test-hardening-benchmarks-qa.md FR-4.3 re-asserts ">= 80% for all modules" and ">= 70% integration", while /workspace/.project/Milestone_3-Completion/RELEASE_NOTES_MILESTONE_3.md reports measured overall coverage of ~78%
  Impact: Measured ~78% simultaneously passes the M3 plan gate (75%), fails the Epic 24 gate (80%) and fails the unit-test-improvements gate (85%). Any "coverage" acceptance criterion in a generated roadmap is unfalsifiable until one gate is chosen
  → Pick one authoritative project-wide gate, or adopt the layered table and state explicitly that the single-number gates are superseded. Variants remain preserved as REQ-test-coverage-target-v1/-v2 plus REQ-epic24-quality-gates and the M3 plan context entry

[WARNING] Epic 11 declares itself complete while its Qdrant acceptance criteria are explicitly deferred
  Found: /workspace/.project/Milestone_2-Missing_features/Epic_11/prd-sanctum-memory-foundation.md makes the Qdrant adapter a first-class Epic 11 requirement (US-11.4, FR-8) and lists "SanctumPort trait implemented with Qdrant and in-memory adapters" and "Performance benchmarks confirm < 500ms search on 100K vectors (Qdrant)" in its epic-completion criteria
  Found: /workspace/.project/Milestone_2-Missing_features/Epic_11/EPIC_11_COMPLETION_SUMMARY.md records "Task 5.0: Qdrant Adapter (DEFERRED) — Not implemented (requires external Qdrant service)" and yet also asserts "All acceptance criteria from the original PRD have been met or exceeded"
  Impact: The completion claim cannot be used as evidence of PRD satisfaction. `task-completion-state.md` independently records 111 open checkboxes in `tasks-sanctum-memory-foundation.md`, the second largest open concentration in the corpus
  → Treat Epic 11 as "in-memory Sanctum complete, Qdrant deferred to Epic 12" and verify against `.planning/codebase/` before planning any Sanctum work — the codebase map records a shipped Qdrant adapter and `qdrant-client` 1.14 behind a `qdrant` feature, so the deferral was closed later

[WARNING] Epic 15 declares itself complete while 129 of its task-list items are unchecked
  Found: /workspace/.project/Milestone_2-Missing_features/Epic_15/epic-15-completion-report.md states status COMPLETE and PRODUCTION-READY with "All tasks marked complete", and /workspace/.project/Milestone_2-Missing_features/Epic_15/epic-15-pull-request-description.md repeats the same checklist
  Found: /workspace/.planning/intel/task-completion-state.md records 129 open items in `.project/Milestone_2-Missing_features/Epic_15/tasks-conclave-mixture-of-agents.md` — the largest single open concentration across all 75 task lists
  Impact: The two largest apparent blocks of remaining work in the corpus (Conclave 129, Sanctum 111) both sit behind documents that claim completion. Planning either as forward work would re-plan shipped code; ignoring both would hide any genuine gap
  → Verify Conclave against the shipped tree before planning. `.planning/codebase/STRUCTURE.md` lists the Battalion crate as "Formation, Phalanx, Campaign, ChainOfCommand, Commander, Maneuver DSL" and does not name Conclave or Council, so the map alone does not settle it — confirm directly and treat the 129 count as a claim, not confirmed work

[WARNING] Competing vision API surfaces between Epic 13 and Epic 20
  Found: /workspace/.project/Milestone_2-Missing_features/Epic_13/prd-sentinel-vision-system.md defines `VisionContent` / `ImageDetail` / `VisionRequest` in `src/core/platform/container/vision.rs`, a `VisionCapableLlm: LlmPort` trait with `generate_with_vision()` and `supports_vision()`, `Paladin::run_with_vision(task, images)` and `PaladinBuilder::enable_vision(bool)`, implemented by extending `OpenAILlmAdapter` and `AnthropicLlmAdapter`
  Found: /workspace/.project/Milestone_3-Completion/Epic_20/prd-vision-pipeline-completion.md defines `VisionImage` / `VisionResponse` / `VisionResult` and `VisionError` in `src/core/platform/container/sentinel/vision_types.rs`, a `VisionPort` trait, dedicated `OpenAIVisionAdapter` / `AnthropicVisionAdapter` in `openai_vision.rs` / `anthropic_vision.rs`, and `PaladinExecutionService::execute_with_vision(paladin, prompt, images)` — same scope "the vision entry point and its types", different names, module paths and layering
  Impact: Two different public surfaces for one feature. Both are PRD precedence so no tiebreaker applies. Downstream examples disagree too (`vision_analysis.rs` / `document_processing.rs` in Epic 13, `sentinel_vision.rs` in Epic 20)
  → Choose the surviving surface. Preserved as REQ-vision-content-model, REQ-vision-capable-llm-trait, REQ-paladin-vision-api-v1, REQ-openai-vision-adapter-v1, REQ-anthropic-vision-adapter-v1, REQ-vision-error-model-v1 (Epic 13) versus REQ-vision-port, REQ-paladin-vision-api-v2, REQ-openai-vision-adapter-v2, REQ-anthropic-vision-adapter-v2, REQ-vision-error-model-v2 (Epic 20)

[WARNING] Contradictory ownership of image format validation
  Found: /workspace/.project/Milestone_2-Missing_features/Epic_13/prd-sentinel-vision-system.md FR-1.2 requires "System MUST validate image formats (PNG, JPEG, GIF, WebP only)", with `VisionError::UnsupportedFormat` and `FileTooLarge { size, max }` for rejected inputs and a CLI requirement to report unsupported formats
  Found: /workspace/.project/Milestone_3-Completion/Epic_20/prd-vision-pipeline-completion.md US-20.1, US-20.2 and NG-5 require adapters to "delegate image format validation to the OpenAI API (supports all formats accepted by the provider)" and forbid format conversion or preprocessing — same scope, opposite responsibility assignment
  Impact: Determines whether an unsupported image fails locally with a typed error or is forwarded to the provider and fails as a 400. Also determines whether `FileTooLarge` exists at all
  → Decide where validation lives. Preserved as REQ-vision-format-validation-v1 (framework-side) and REQ-vision-format-validation-v2 (provider-side)

[WARNING] Vision encryption-at-rest and retention requirements disappear between Epic 13 and Epic 20
  Found: /workspace/.project/Milestone_2-Missing_features/Epic_13/prd-sentinel-vision-system.md FR-11 requires encryption at rest for temporarily stored image data (aes-gcm or chacha20poly1305, per-session keys), memory zeroization after processing, configurable data retention policies and security-event audit logging, with success metrics of "100% of stored image/document data encrypted" and a `VisionError::EncryptionError` variant
  Found: /workspace/.project/Milestone_3-Completion/Epic_20/prd-vision-pipeline-completion.md completes the vision pipeline with no encryption, retention or zeroization requirement, and its `VisionError` enum (FR-5.1) omits `EncryptionError` entirely
  Impact: A security requirement stated in the originating PRD is silently absent from the PRD that finished the feature. If Epic 20 is treated as the current spec, an explicit security control is dropped without a recorded decision
  → Confirm whether the encryption requirement was consciously dropped or simply not restated. Preserved as REQ-vision-security-encryption against REQ-vision-error-model-v2

[WARNING] Competing handoff tool name and parameter names
  Found: /workspace/.project/Milestone_2-Missing_features/Epic_14/prd-autonomous-agent-features.md FR-5.1 and FR-5.2 register a tool named `handoff_to_agent` with required parameters `agent_name` (enum of available agents) and `message`
  Found: /workspace/.project/Milestone_3-Completion/Epic_21/prd-autonomous-agent-completion.md FR-3.4 and FR-3.5 specify parameters `specialist_name` (enum of specialists) and `task_description`, and /workspace/.project/Milestone_3-Completion/Epic_23/prd-task46-arsenal-tool-integration-tests.md Non-Goal 5 refers to the tool as `handoff_to_specialist` — same tool, three names across two parameter sets
  Impact: The tool name and parameter names are part of the JSON schema sent to the LLM. A mismatch means the model emits calls the framework cannot route, and any test asserting on either name breaks against the other
  → Fix one tool name and one parameter pair. Preserved as REQ-handoff-tool-v1 and REQ-handoff-tool-v2

[WARNING] MaxLoops changes from a scalar to an enum, superseding a Milestone 1 requirement
  Found: run 1 recorded REQ-paladin-entity requiring `PaladinData.max_loops` as part of a flat field set and REQ-paladin-builder requiring "MUST validate max_loops is in range [1, 100], default 3", both from /workspace/.project/Milestone_1-MVP/Epic_1/prd-paladin-domain-foundation.md
  Found: /workspace/.project/Milestone_2-Missing_features/Epic_14/prd-autonomous-agent-features.md FR-1.1 requires `MaxLoops::Auto { max_subtasks: u32 }` "in addition to `MaxLoops::Fixed(u32)`", i.e. `max_loops` becomes an enum, and /workspace/.project/Milestone_3-Completion/Epic_23/prd-task46-arsenal-tool-integration-tests.md test code uses `MaxLoops::Fixed(3)`
  Impact: A range validation defined on an integer cannot be applied unchanged to an enum, and every Milestone-1 requirement or test that assumes a numeric `max_loops` is affected. This is a genuine type-level supersession, not a documentation slip
  → Confirm the enum is the current shape and mark the run-1 numeric-range criterion superseded rather than deleted. Preserved as REQ-max-loops-auto alongside the unchanged run-1 entries

[WARNING] Three names and three defaults for the Grove routing threshold
  Found: /workspace/.project/Milestone_2-Missing_features/Epic_16/prd-epic16-advanced-battalion-patterns.md FR-2.3 defines `GroveConfig.similarity_threshold` with default 0.7
  Found: /workspace/.project/Milestone_3-Completion/Epic_22/prd-epic22-battalion-commander-hardening.md FR-6.2 adds `GroveConfig.min_confidence` with default 0.5 as the gate for accepting an LLM routing decision
  Found: /workspace/.project/Milestone_3-Completion/RELEASE_NOTES_MILESTONE_3.md shows `GroveConfig { routing_strategy, confidence_threshold: 0.6 }` — a third field name and a third default for the same concept
  Impact: Either Grove has two threshold fields with different semantics (similarity for embedding routing, confidence for LLM routing) or one field was renamed twice. A roadmap cannot state the routing acceptance threshold without resolving this
  → Decide whether one threshold or two are intended, and fix the name. Preserved as REQ-grove-config-v1 and REQ-grove-config-v2, with the release-notes form recorded in context.md

[WARNING] Grove "PerformanceBased" routing and learning contradict an explicit Epic 16 non-goal
  Found: /workspace/.project/Milestone_2-Missing_features/Epic_16/prd-epic16-advanced-battalion-patterns.md FR-2.2 defines exactly three routing strategies — KeywordMatch (default), SemanticSimilarity, LlmRouting — and NG-3 explicitly excludes "Grove learning from routing decisions to improve future matches (future ML feature)"
  Found: /workspace/.project/Milestone_3-Completion/RELEASE_NOTES_MILESTONE_3.md advertises `RoutingStrategy::PerformanceBased` ("Adaptive routing based on historical success") and "Dynamic Learning: Performance-based routing improves over time" as shipped Grove features
  Impact: A capability declared out of scope by the owning PRD is announced as delivered. Either an undocumented fourth strategy exists or the release notes describe an unbuilt feature
  → Verify whether `RoutingStrategy::PerformanceBased` exists before recording it as a requirement. Preserved as REQ-grove-routing-strategies (three strategies) with the release-notes claim recorded only in context.md

[WARNING] Competing Council execution API and result shape
  Found: /workspace/.project/Milestone_2-Missing_features/Epic_16/prd-epic16-advanced-battalion-patterns.md FR-1.4 and Epic_16/epic16.md specify `CouncilExecutionService::convene(council, topic)` returning `CouncilResult { transcript, conclusion, rounds_completed, termination_reason }`, a `CouncilBuilder` that takes participants via `add_participant(paladin)`, and a unit `TerminationCondition::MaxRounds` whose count comes from `CouncilConfig.max_rounds`
  Found: /workspace/.project/Milestone_3-Completion/RELEASE_NOTES_MILESTONE_3.md shows `council_service.execute(&council, &experts, topic)`, `result.summary`, `CouncilBuilder::participants(3)` (a count) and `TerminationCondition::MaxRounds(3)` (a tuple variant carrying the count) — same scope, four incompatible differences
  Impact: The method name, the argument list, the result field name and the enum shape all differ. Examples and CLI wiring built against either form will not compile against the other
  → Confirm the current Council surface. Preserved as REQ-council-execution-service and REQ-council-termination-conditions (Epic 16 form) with the release-notes form recorded in context.md

[WARNING] Competing Maneuver constructor and CLI surface
  Found: /workspace/.project/Milestone_2-Missing_features/Epic_17/epic17.md specifies `Maneuver::new(name, agents: Vec<Paladin>, flow: &str)` and /workspace/.project/Milestone_2-Missing_features/Epic_17/prd-flow-dsl-agent-rearrangement.md FR-8.1 and FR-9.3 specify the CLI as `paladin battalion run --type maneuver --flow "..."` and `paladin battalion visualize --flow "..."`
  Found: /workspace/.project/Milestone_3-Completion/RELEASE_NOTES_MILESTONE_3.md shows `Maneuver::new(flow3, paladins, config)` (flow first, config third, no name) and a top-level command group `paladin maneuver validate "..."` / `paladin maneuver visualize "..." --format mermaid`
  Impact: Both the constructor argument order and the CLI command namespace differ. Epic 17.5 also lists a `maneuver.rs` command file under the consolidated CLI, which is consistent with a top-level command group but not with `battalion visualize`
  → Confirm the shipped constructor and command namespace. Preserved as REQ-maneuver-domain-model and REQ-maneuver-cli with the release-notes form recorded in context.md

[WARNING] `metadata_output_dir` now has three competing owners
  Found: run 1 recorded the field on `BattalionConfig` twice — REQ-battalion-config-v1 from /workspace/.project/Milestone_1-MVP/Epic_4/prd-battalion-orchestration.md FR-4.1 and REQ-battalion-config-v2 from /workspace/.project/Milestone_1-MVP/Epic_5/prd-commander-strategy-router.md FR-7
  Found: /workspace/.project/Milestone_3-Completion/Epic_22/prd-epic22-battalion-commander-hardening.md FR-10.1 requires `metadata_output_dir: Option<PathBuf>` on `CommanderConfig` in `src/core/platform/container/battalion/commander_config.rs`, with YAML surface `commander.metadata_output_dir`
  Impact: The run-1 BattalionConfig conflict is not resolved by Epic 22 — it is relocated. Code reading the field from BattalionConfig and code writing it to CommanderConfig will silently disagree, and metadata export (an Epic 5 deferred task) is the feature that depends on it
  → Reconcile config ownership before routing. Preserved as REQ-commander-config-metadata-dir-v3 alongside the unchanged run-1 REQ-battalion-config-v1/-v2

[WARNING] Competing ErrorStrategy variant sets for the same enum name
  Found: run 1 recorded REQ-battalion-error-strategy from /workspace/.project/Milestone_1-MVP/Epic_4/prd-battalion-orchestration.md FR-4.3 as `FailFast`, `ContinueOnError`, `RetryThenContinue`, and /workspace/.project/Milestone_3-Completion/Epic_22/prd-epic22-battalion-commander-hardening.md US-22.5 names Commander tests after exactly those three behaviours plus a `continue_on_error: true` config flag
  Found: /workspace/.project/Milestone_2-Missing_features/Epic_17/prd-flow-dsl-agent-rearrangement.md FR-5.1 defines `ErrorStrategy` as `FailFast`, `ContinueParallel`, `IgnoreErrors` for `ManeuverConfig`
  Found: /workspace/.project/Milestone_3-Completion/RELEASE_NOTES_MILESTONE_3.md documents both sets in different sections of the same document
  Impact: One type name, two disjoint variant sets beyond `FailFast`. Either two distinct enums exist and share a name across modules, or one enum grew incompatible variants
  → Decide whether Maneuver needs its own strategy enum or should reuse the Battalion one. Preserved as REQ-battalion-error-strategy (run 1) and REQ-maneuver-error-strategy-v2

[WARNING] Live API tests: graceful skip versus deliberate loud failure
  Found: /workspace/.project/Milestone_3-Completion/Epic_24/prd-test-hardening-benchmarks-qa.md US-24.7 requires "Tests skip gracefully if API keys not available (no failures, just warnings)", and /workspace/.project/Milestone_3-Completion/Epic_23/prd-epic23-cli-config-infrastructure-completion.md FR-23.4.4 requires clear skip messages when prerequisites are not met
  Found: /workspace/.project/Milestone_3-Completion/Post-Epic_24-cleanup/LIVE_API_TESTS_FIX.md deliberately reverses this: `require_api_key()` was changed to panic instead of returning a Result, on the stated rationale that tests previously "printed SKIPPED and returned early, counting as PASS". The document states "Tests will now properly FAIL when keys are missing"
  Impact: The two PRDs and the shipped test harness disagree on whether a missing key is a skip or a failure. Precedence says PRD wins, but the DOC is later and describes a conscious change, so applying precedence mechanically would record the wrong behaviour
  → Record the post-cleanup behaviour as the current position and mark the PRD criterion superseded, or restore graceful skip. Preserved as REQ-provider-live-api-tests with the reversal recorded in context.md

[WARNING] Grove service still hardcodes its LLM model in shipped code
  Found: /workspace/.project/Milestone_3-Completion/Epic_21/prd-autonomous-agent-completion.md US-21.5 removed hardcoded `"gpt-4"` from `planning_service.rs` and `prompt_generation_service.rs`, and /workspace/.project/Milestone_3-Completion/Epic_22/prd-epic22-battalion-commander-hardening.md epic-completion criteria require "All inline TODOs in Battalion and Commander files resolved"
  Found: /workspace/.planning/codebase/CONCERNS.md records "Grove Service Model Hardcoded" — `crates/paladin-battalion/src/grove_service.rs:537` hardcodes `model: "gpt-4".to_string()` with a TODO to make it configurable, so Grove routing ignores the configured LLM provider
  Impact: The same defect class Epic 21 eliminated elsewhere is still present in the Grove routing path in shipped code, and Epic 22's completion criterion is therefore not met. `task-completion-state.md` independently records 81 open items in `tasks-epic22-battalion-commander-hardening.md`
  → This is verified remaining work, not a documentation conflict. Carry it forward as a real forward-work candidate rather than re-planning the rest of Epic 22

### INFO (39)

-- Run 1 (Milestone_1-MVP), carried forward unchanged: 11 entries --

[INFO] Auto-resolved: PRD > DOC on coverage baseline figure
  Note: /workspace/.project/Milestone_1-MVP/unit-test-improvements/prd-improve-unit-test-coverage.md states current unit coverage is "67.79%"; /workspace/.project/Milestone_1-MVP/Epic_10/task6.0-validation-report.md measures unit coverage at "60.88%" and integration coverage at "67.79%"; /workspace/.project/Milestone_1-MVP/unit-test-improvements/COVERAGE_ANALYSIS.md states "70.56% baseline". PRD outranks both DOCs, so 67.79% is the recorded baseline in synthesized intel. Flagged because the PRD's 67.79% is numerically identical to the validation report's *integration* coverage figure, which suggests the PRD baseline may be a mislabelled metric rather than a genuine unit-coverage reading. The two DOCs also disagree with each other (60.88% vs 70.56%); both lose to the PRD

[INFO] Auto-resolved: PRD > DOC on PaladinBuilder::restore_from signature
  Note: /workspace/.project/Milestone_1-MVP/Epic_7/epic7.md declares "pub fn restore_from(self, state_id: Uuid) -> Self"; /workspace/.project/Milestone_1-MVP/Epic_7/prd-citadel-state-persistence.md FR3.1 and its Builder Integration section declare "restore_from(mut self, state_id: Uuid) -> Result<Self, PaladinError>". PRD wins; the fallible signature is recorded in REQ-citadel-paladin-restore and REQ-citadel-builder-integration. The PRD form is also required by FR3.5 ("fail with clear error if state file not found or invalid JSON"), which the infallible DOC form cannot express

[INFO] Auto-resolved: PRD > DOC on Herald trait method set
  Note: /workspace/.project/Milestone_1-MVP/Epic_8/epic8.md defines a two-method Herald trait (format_paladin_result, format_battalion_result); /workspace/.project/Milestone_1-MVP/Epic_8/prd-herald-output-formatting.md requires at minimum four methods including streaming and error formatting. PRD wins; the DOC's narrower trait is recorded as context only. Note the PRD is itself internally inconsistent on this trait — see the corresponding WARNING

[INFO] Auto-resolved: PRD > DOC on LlmPort surface
  Note: /workspace/.project/Milestone_1-MVP/Epic_6/epic6.md shows the LlmPort impl surface as generate, generate_stream, validate_model, get_available_models, get_provider_name — omitting get_capabilities; /workspace/.project/Milestone_1-MVP/Epic_6/prd-provider-expansion.md REQ-1 requires get_capabilities() and REQ-2 defines ProviderCapabilities. PRD wins; feature detection is retained in REQ-llm-port-interface

[INFO] Auto-resolved: PRD > DOC on Commander construction API
  Note: /workspace/.project/Milestone_1-MVP/Epic_5/epic5.md shows "Commander::new(strategy, paladins)"; /workspace/.project/Milestone_1-MVP/Epic_5/prd-commander-strategy-router.md section 6.3 specifies a fluent "Commander::builder().strategy(..).paladins(..).config(..).build()?". PRD wins; recorded in REQ-commander-construction

[INFO] Reconciled (not precedence-based): Task 5.0 pending verifications closed by Task 6.0 measurements
  Note: /workspace/.project/Milestone_1-MVP/Epic_10/task5.0-completion-summary.md self-reports Task 5.0 at "10/14 subtasks complete (71%)" with Docker image size verification, local kind deployment testing and pod startup time validation still pending; /workspace/.project/Milestone_1-MVP/Epic_10/tasks-epic10-validation-documentation.md marks task 5.0 complete and /workspace/.project/Milestone_1-MVP/Epic_10/task6.0-validation-report.md records the measured outcomes (112MB image, 5m31s build, kubernetes-smoke-test job validating the <30s startup requirement). Treated as a stale point-in-time snapshot whose open items are answered by content elsewhere in the same ingest set, not as a contradiction of intent. No precedence tiebreaker was applied

[INFO] Cross-reference graph is acyclic
  Note: 36 nodes, edges built from the `cross_refs` field of each classification. In-set edges run tasks-*.md -> prd-*.md (Epic_9, Epic_4, Epic_7, Epic_2, Epic_10, Epic_1, Epic_6), prd-commander-strategy-router.md -> epic5.md and -> Paladin Project Completion Plan.md, prd-herald-output-formatting.md -> Paladin Project Completion Plan.md, prd-paladin-domain-foundation.md -> Paladin Project Completion Plan.md and -> epic1.md, epic5.md -> "Epic 4". DFS three-colour marking found no back-edges. Maximum traversal depth 3, well under the depth-50 cap. Synthesis proceeded on the full set

[INFO] No ADR or SPEC layer present in this run
  Note: the precedence list ADR > SPEC > PRD > DOC was applied with only its lower two tiers populated (11 PRD, 25 DOC). Consequently `decisions.md` and `constraints.md` contain no entries. Every technical decision in this milestone is currently asserted only at PRD or DOC level and is therefore auto-overridable by any ADR arriving in ingest runs 2-14

[INFO] All 36 classifications were manifest-driven with high confidence
  Note: every classification carries `manifest_override: true` and `confidence: high`; none is UNKNOWN. No heuristic type inference contributed to this synthesis, so no re-tagging blocker was raised

[INFO] Nine epic*.md DOCs carry PRD/SPEC-shaped content that was manifest-typed DOC
  Note: classifier notes on Epic_1/epic1.md, Epic_2/epic2.md, Epic_3/epic3.md, Epic_4/epic4.md, Epic_5/epic5.md, Epic_6/epic6.md, Epic_7/epic7.md, Epic_8/epic8.md, Epic_9/epic9.md and Epic_10/epic10.md record user stories, acceptance criteria and Rust trait/type contracts (PRD and SPEC signals) that were overridden by MANIFEST_TYPE=DOC. Under precedence these lose to the paired prd-*.md in the same directory, which is what produced the five PRD-over-DOC resolutions above. If those Rust contracts should bind at SPEC precedence, re-tag via --manifest and re-run ingest

[INFO] Implementation is substantially ahead of the plan documents
  Note: task-list checkbox state across the 11 task lists shows 1,817 of 1,857 items complete (98%). Known-incomplete work concentrates in: Epic_4 Chain of Command pattern (task 6.0) and Epic 4 integration testing/performance validation (task 7.0); Epic_8 Herald integration with Paladin/Battalion execution (task 7.0); Epic_6 live-API integration tests (task 7.0, explicitly DEFERRED); Epic_5 result normalization and telemetry metadata (task 5.0) plus one failing test (3.11 test_auto_selects_campaign_for_workflow_keywords); Epic_2 final validation and cleanup (task 11.0); unit-test-improvements tasks 2.0 and 6.0. This is recorded so the roadmapper does not re-plan completed work — see `intel/context.md` implementation-status topics
-- Run 2 (Milestone_2-Missing_features + Milestone_3-Completion): 28 entries --

[INFO] Still no ADR or SPEC layer after 81 documents
  Note: run 2 added 15 PRD and 30 DOC classifications and zero ADR or SPEC. Cumulatively 26 PRD and 55 DOC across runs 1-2, with 0 locked decisions. `decisions.md` and `constraints.md` therefore still contain no entries. Every technical position in `requirements.md` remains overridable by any ADR arriving in runs 3-5, and no LOCKED-vs-LOCKED hard block is possible in this corpus

[INFO] All 45 run-2 classifications were manifest-driven with high confidence
  Note: every classification in `/workspace/.planning/intel/classifications/run-02/` carries `manifest_override: true` and `confidence: high`; none is UNKNOWN and none is low-confidence. No heuristic type inference contributed to this synthesis, so no re-tagging blocker was raised

[INFO] Run-2 cross-reference graph is acyclic
  Note: 45 nodes, edges built from each classification's `cross_refs`. Only six edges point at documents inside the ingest set: prd-conclave-mixture-of-agents.md -> Epic_15/epic15.md; prd-epic19-herald-consolidation.md -> Project_Plan_Milestone_3.md and -> Epic_19/epic19.md; Epic_21/epic21.md -> Epic_14/epic14.md (written `epic14.mdd`); LEGACY_CLEANUP_SUMMARY.md -> LEGACY_CODE_CLEANUP_PLAN.md; QUICK_SUMMARY.md -> LIVE_API_TESTS_SUCCESS.md and -> SESSION_SUMMARY.md. All other cross_refs point at source files, docs/ or `tasks-*.md` (excluded from classification). DFS three-colour marking found no back-edges; maximum traversal depth 2, well under the depth-50 cap

[INFO] Fourteen epic*.md DOCs carry PRD/SPEC-shaped content that was manifest-typed DOC
  Note: classifier notes on Epic_11 through Epic_18 (including Epic_17.5) and Epic_19 through Epic_24 record user stories, acceptance criteria and Rust trait/type "Definition of Done" blocks — PRD and SPEC signals — overridden by MANIFEST_TYPE=DOC. Under precedence these lose to the paired prd-*.md in the same directory, which produced the six PRD-over-DOC resolutions below. If those Rust contracts should bind at SPEC precedence, re-tag via --manifest and re-run ingest

[INFO] Auto-resolved: PRD > DOC on Epic 11 user-story numbering and scope
  Note: /workspace/.project/Milestone_2-Missing_features/Epic_11/epic11.md defines five user stories with US-11.4 = In-Memory Vector Store and US-11.5 = Sanctum Domain Model, and does not include Qdrant in Epic 11 at all; /workspace/.project/Milestone_2-Missing_features/Epic_11/prd-sanctum-memory-foundation.md defines six with US-11.4 = Qdrant, US-11.5 = In-Memory, US-11.6 = Domain Model. PRD wins, which is why the Qdrant adapter is recorded as an Epic 11 requirement (REQ-qdrant-sanctum-adapter-v1) even though the epic DOC omits it

[INFO] Auto-resolved: PRD > DOC on EmbeddingPort return type
  Note: /workspace/.project/Milestone_2-Missing_features/Epic_11/epic11.md states in its US-11.1 acceptance bullet that the trait includes `embed_text(&str) -> Result<Vec<f32>, EmbeddingError>`, while its own Definition-of-Done block in the same section returns `Result<Embedding, EmbeddingError>` and adds `model_name()`. The PRD requires the `Embedding` form with all four methods. PRD wins; recorded in REQ-embedding-port

[INFO] Auto-resolved: PRD > DOC on ConclaveConfig and ConclaveResult field sets
  Note: /workspace/.project/Milestone_2-Missing_features/Epic_15/epic15.md omits `retry_attempts` and `observability_level` from `ConclaveConfig` and omits `expert_execution_times` and `retry_counts` from `ConclaveResult`; /workspace/.project/Milestone_2-Missing_features/Epic_15/prd-conclave-mixture-of-agents.md FR-C2 and FR-C3 require all four. PRD wins; recorded in REQ-conclave-domain-model

[INFO] Auto-resolved: PRD > DOC on Council turn strategies
  Note: /workspace/.project/Milestone_2-Missing_features/Epic_16/epic16.md lists `TurnStrategy::Random` and `TurnStrategy::VoluntaryWithTimeout { timeout_ms }` as enum variants; /workspace/.project/Milestone_2-Missing_features/Epic_16/prd-epic16-advanced-battalion-patterns.md NG-6 explicitly defers both, requiring only RoundRobin and ModeratorDirected. PRD wins; recorded in REQ-council-turn-strategies with the deferral noted

[INFO] Auto-resolved: PRD > DOC on ManeuverConfig and ManeuverResult field sets
  Note: /workspace/.project/Milestone_2-Missing_features/Epic_17/epic17.md defines `ManeuverConfig` with only timeout_seconds, error_strategy and pass_output_as_input, and `ManeuverResult` without timing metrics; /workspace/.project/Milestone_2-Missing_features/Epic_17/prd-flow-dsl-agent-rearrangement.md FR-5.2 to FR-5.4 and FR-6.6 add agent_timeout_seconds, output_format, collect_timing_metrics, capture_intermediate_outputs and `timing_metrics: Option<HashMap<String, Duration>>`. PRD wins; recorded in REQ-maneuver-config and REQ-maneuver-execution-service

[INFO] Auto-resolved: PRD > DOC on Grove LLM routing fallback
  Note: /workspace/.project/Milestone_3-Completion/Epic_22/epic22.md states Grove "falls back to keyword matching if LLM call fails" unconditionally; /workspace/.project/Milestone_3-Completion/Epic_22/prd-epic22-battalion-commander-hardening.md FR-5.6 and FR-6.1 make the fallback configurable via `routing_fallback` with values "keyword" or "error". PRD wins; recorded in REQ-grove-llm-routing and REQ-grove-config-v2

[INFO] ConclaveStatus variant naming differs between the PRD and the completion report
  Note: /workspace/.project/Milestone_2-Missing_features/Epic_15/prd-conclave-mixture-of-agents.md FR-C3 and Epic_15/epic15.md both specify `ConclaveStatus { Success, PartialSuccess, Failed }`; /workspace/.project/Milestone_2-Missing_features/Epic_15/epic-15-completion-report.md reports the implemented enum as `{ Completed, PartialSuccess, Failed }` while also stating "Deviations from Original PRD: None". Recorded, not merged — REQ-conclave-domain-model keeps the PRD spelling. The same report omits the PRD FR-C4 duplicate-agent-name validation from its acceptance list

[INFO] Qdrant adapter naming and connection settings differ between Epic 11 and Epic 12, but the feature shipped
  Note: Epic 11 specifies `QdrantSanctumAdapter` in `src/infrastructure/adapters/sanctum/qdrant_adapter.rs` with host + port 6334 + `use_grpc: true` and collection `paladin_memories_{environment}`; Epic 12 specifies `QdrantSanctum` in `src/infrastructure/adapters/sanctum/qdrant_sanctum.rs` with `url: http://localhost:6333`, `collection_name`, `vector_size`, `distance` and `on_disk`. `.planning/codebase/STACK.md` records `qdrant-client` 1.14 behind a `qdrant` feature and `.planning/codebase/STRUCTURE.md` records a Sanctum vector-search adapter under `crates/paladin-memory/src/sanctum/`, so Qdrant is shipped and the Epic 11-versus-Epic 12 ownership question is moot for forward planning. Both variants preserved as REQ-qdrant-sanctum-adapter-v1/-v2; consult the codebase map rather than either PRD for the current adapter name and connection shape

[INFO] Cluster of epic-number mislabels in cross-references
  Note: four separate documents attribute features to the wrong epic. /workspace/.project/Milestone_2-Missing_features/Epic_18/prd-epic-18-cli-enhancement.md D-2 says "Requires Council orchestration pattern from Epic 15" (Council is Epic 16; Epic 15 is Conclave). /workspace/.project/Milestone_3-Completion/Epic_22/prd-epic22-battalion-commander-hardening.md Appendix B says "Epic 15: Council & Grove" and "Epic 16: Conclave (expert panels with voting)" — the two are swapped, and Conclave has no voting. /workspace/.project/Milestone_3-Completion/Epic_23/EPIC_23_COMPLETION_SUMMARY.md "Related Work" calls Epic 19 "Battalion orchestration patterns", Epic 20 "Commander and Grove routing" and Epic 22 "Maneuver flow-based orchestration", and attributes deferred garrison semantic search to "Epic 24". These are documentation defects with no requirement impact, but they compound the release-notes numbering conflict recorded under WARNINGS

[INFO] Epic 21 DOC cross-reference has a filename typo and an unclosed parenthetical
  Note: /workspace/.project/Milestone_3-Completion/Epic_21/epic21.md references `project/Milestone_2-Missing_features/Epic_14/epic14.mdd`; the file is `epic14.md`. The sentence containing it is never closed. Cycle detection treated the edge as pointing at `epic14.md`; no cycle results either way

[INFO] Reported total test counts do not form a monotonic series
  Note: as reported — 999 unit (Epic 11 summary, 30 Jan 2026), 1,292 unit plus 168 doc plus 560+ integration (Epic 15 report, 2 Feb 2026), 1,674 total (Epic 23 summary, 14 Feb 2026), 1,606 unit / 1,628 total (post-Epic-24 cleanup, Apr 2026), 720 unit plus 133 integration (Milestone 3 release notes). The release-notes figure is roughly 950 tests below the Epic 23 figure from an earlier date in the same milestone. Recorded as reported point-in-time snapshots; no figure is treated as authoritative and none is reconciled

[INFO] Redis integration test count disagrees between two cleanup documents
  Note: /workspace/.project/Milestone_3-Completion/Post-Epic_24-cleanup/REDIS_TEST_FIX.md states "The 16 Redis queue integration tests were failing"; /workspace/.project/Milestone_3-Completion/Post-Epic_24-cleanup/CHANGELOG_UPDATE.md records the same work as "Redis Queue Integration Tests: All 6 tests passing". Both are DOC precedence so no tiebreaker applies. Low planning impact

[INFO] Live API test totals and the "all passing" headline disagree across three sibling documents
  Note: /workspace/.project/Milestone_3-Completion/Post-Epic_24-cleanup/LIVE_API_TESTS_SUCCESS.md says "All 8 essential tests now pass" then tabulates OpenAI 4/4, Anthropic 4/4, DeepSeek 1/4 and reports a full run of "9 passed; 3 failed" out of 12; SESSION_SUMMARY.md repeats 4/4, 4/4, 1/4 while also showing a 6-test OpenAI run; QUICK_SUMMARY.md headlines "Live API Tests - Complete Success — ALL TESTS PASSING" and "OpenAI: 6/6" while omitting the three DeepSeek credit failures. All three are also dated 2025-01-26 while sibling documents on the same branch are dated 2026-04-08. Recorded for transparency; the DeepSeek failures are attributed to insufficient API credits, not code defects

[INFO] Plaintext OpenAI API key present in an ingested source document
  Note: /workspace/.project/Milestone_3-Completion/Post-Epic_24-cleanup/LIVE_API_TESTS_FIX.md contains a plaintext OpenAI API key in its body while explaining that a stray space had broken the value. The key value has NOT been copied into any intel file, this report, or any other synthesis output. The user has confirmed the key is already rotated and asked that it be disregarded for planning purposes. Hygiene recommendation only: redact the literal from the source document, and consider a repository-wide secret scan since the same value may appear in `.env` history or coverage artefacts

[INFO] Module-scoped coverage targets above the global gate
  Note: /workspace/.project/Milestone_3-Completion/Epic_19/prd-epic19-herald-consolidation.md requires >= 95% coverage for modified Herald modules and /workspace/.project/Milestone_3-Completion/Epic_21/prd-autonomous-agent-completion.md requires >= 90% for autonomous components, while Epics 22, 23 and 24 require >= 80%. These are scoped to specific modules and can coexist with a lower global gate, so they are recorded as separate requirements (REQ-herald-consolidation-quality-gates, REQ-autonomous-completion-quality-gates) rather than as competing variants of the project-wide gate

[INFO] BattalionResult module path recorded two different ways in Milestone 3
  Note: /workspace/.project/Milestone_3-Completion/Epic_19/prd-epic19-herald-consolidation.md locates `BattalionResult` in `src/core/platform/container/battalion/mod.rs` (which also settles the run-1 `battalion/mod.rs` versus `battalion/battalion.rs` warning in favour of `mod.rs`); /workspace/.project/Milestone_3-Completion/Epic_22/prd-epic22-battalion-commander-hardening.md places `BattalionMetadata` and by implication `BattalionResult` in `src/core/platform/container/battalion/battalion_result.rs`. Low impact — a wrong path produces inconsistent references, not wrong behaviour. `.planning/codebase/ARCHITECTURE.md` maps the current location as `crates/paladin-core/src/platform/container/battalion/`

[INFO] SchedulerPort surface and crate version differ between plan and outcome
  Note: /workspace/.project/Milestone_3-Completion/Epic_23/prd-epic23-cli-config-infrastructure-completion.md sketches `SchedulerPort` with `schedule_job`, `cancel_job` and `get_job_status` and pins `tokio-cron-scheduler = "0.9"`; /workspace/.project/Milestone_3-Completion/Epic_23/EPIC_23_COMPLETION_SUMMARY.md reports the delivered trait as `schedule_job`, `cancel_job`, `list_jobs`, `get_job_info` on tokio-cron-scheduler v0.13. `.planning/codebase/STACK.md` records tokio-cron-scheduler 0.13 behind a `scheduler` feature, so 0.13 is the shipped version. Recorded as plan-versus-outcome drift, not a conflict requiring a decision

[INFO] Epic 17.5 is the strongest ADR candidate in the corpus so far
  Note: /workspace/.project/Milestone_2-Missing_features/Epic_17.5/epic17-5.md decides that the CLI belongs in `src/application/cli` because "CLI is an input adapter in the application layer, not infrastructure", directs deletion of the entire `src/cli` tree and removal of `pub mod cli;` from `lib.rs`, and gives a rationale plus a full target layout. It has no ADR status field, no Consequences section and no `locked` flag, so under precedence it sits at DOC level and loses to any PRD. Consider promoting it to a real ADR if the CLI location should be protected from future override

[INFO] CLI module location contradicted by an earlier PRD
  Note: /workspace/.project/Milestone_2-Missing_features/Epic_17/prd-flow-dsl-agent-rearrangement.md section 7.1 places CLI integration at `src/infrastructure/adapters/cli/battalion_commands.rs`, while /workspace/.project/Milestone_2-Missing_features/Epic_15/prd-conclave-mixture-of-agents.md section 7 uses `src/application/cli/battalion_commands.rs`, the Epic 15 completion report records the shipped file as `src/cli/config/battalion_config.rs`, and Epic 17.5, /workspace/.project/Milestone_2-Missing_features/Epic_18/prd-epic-18-cli-enhancement.md and /workspace/.project/Milestone_3-Completion/Epic_23/prd-epic23-cli-config-infrastructure-completion.md all use `src/application/cli/`. Three of the four PRD-level positions and the later consolidation decision agree on the application layer, so the Epic 17 infrastructure placement is recorded as an outlier rather than a live variant. REQ-cli-core-infrastructure records the application-layer path

[INFO] Epic 13 Battalion vision integration is narrowed by Epic 20
  Note: /workspace/.project/Milestone_2-Missing_features/Epic_13/prd-sentinel-vision-system.md FR-10 requires all four Battalion patterns to support vision inputs, including `phalanx.run_with_images(...)` and Campaign branching on vision results; /workspace/.project/Milestone_3-Completion/Epic_20/prd-vision-pipeline-completion.md NG-6 states there is no batch vision API and that "concurrent analysis of multiple images [is] handled at Battalion level", with a single execution-service call processing one prompt plus images. Not a direct contradiction — Epic 20 defers rather than denies — but the Epic 13 Battalion-level vision APIs have no Epic 20 counterpart. Preserved as REQ-battalion-vision-integration

[INFO] Epic 11 and Epic 12 both define a `MemoryExtractionStrategy` default and both agree
  Note: /workspace/.project/Milestone_2-Missing_features/Epic_12/epic12.md and prd-sanctum-rag-integration.md both give `MemoryExtractionStrategy { EveryTurn, OnCompletion, Manual, Threshold { importance } }` with `OnCompletion` as the default. Recorded because it is one of the few shared types in the run-2 corpus with no divergence between its DOC and PRD carriers

[INFO] Epic 23 completion claim is the only self-declared completion the M3 plan itself ratifies
  Note: /workspace/.project/Milestone_3-Completion/Project_Plan_Milestone_3.md was edited in place to mark the Epic 23 section `Status: COMPLETE (February 14, 2026)` with all its checklist items flipped to `[x]`, while Epics 19-22 and 24 remain unchecked in the same document. `task-completion-state.md` corroborates: `tasks-task46-arsenal-tool-integration-tests.md` has 1 open item, the lowest in Milestone 3. Recorded because it makes Epic 23 the most reliably-complete epic in the run-2 corpus

[INFO] Milestone 2 and 3 open-item counts are claims to verify, not confirmed forward work
  Note: `.planning/intel/task-completion-state.md` records Milestone 2 at 86.2% complete (298 open) and Milestone 3 at 90.0% (132 open), concentrated in tasks-conclave-mixture-of-agents.md (129), tasks-sanctum-memory-foundation.md (111), tasks-epic22-battalion-commander-hardening.md (81) and tasks-test-hardening-benchmarks-qa.md (29). Run 1 already proved checkbox state can understate reality, and in run 2 the Conclave and Sanctum counts sit behind documents that claim completion while the Epic 22 count is partially corroborated by a real open TODO in `.planning/codebase/CONCERNS.md`. Every open count must be verified against the shipped tree before it becomes a roadmap phase

[INFO] Sentinel and Autonomous docs disagree with the codebase map on where features live
  Note: the run-2 PRDs consistently describe a single-crate layout (`src/core/...`, `src/application/...`, `src/infrastructure/...`) while `.planning/codebase/STRUCTURE.md` maps a Cargo workspace of nine crates (`paladin-core`, `paladin-ports`, `paladin-battalion`, `paladin-llm`, `paladin-memory`, `paladin-storage`, `paladin-content`, `paladin-notifications`, `paladin-web`, plus `paladin-herald`). The workspace decomposition happened in Milestone 5, which is outside this ingest run. Every `src/...` path in run-2 requirements is therefore historical; resolve current locations through the codebase map, not through these PRDs
