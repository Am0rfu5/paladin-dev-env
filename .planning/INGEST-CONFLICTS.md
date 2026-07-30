## Conflict Detection Report

Ingest run 1 of 14. MODE=new. Source set: `.project/Milestone_1-MVP` (36 docs:
11 PRD, 25 DOC, 0 ADR, 0 SPEC). Precedence applied: ADR > SPEC > PRD > DOC.
No per-doc precedence overrides were present. No `locked: true` docs.

### BLOCKERS (0)

None. No ADR-typed docs exist in this run, so no LOCKED-vs-LOCKED contradiction
is possible. No doc was classified UNKNOWN or low-confidence. Cross-ref cycle
detection found no cycles. MODE=new, so there is no existing `.planning/`
context to contradict.

### WARNINGS (8)

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

### INFO (11)

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
