# Context (from DOC-typed docs)

Ingest run 1 of 14 — source set: `.project/Milestone_1-MVP`. 25 DOCs consumed.
Topic-keyed notes with source attribution. Content here is lower precedence than
`requirements.md` (PRD) under `ADR > SPEC > PRD > DOC`.

---

## Topic: Project vision and scope
- source: /workspace/.project/Milestone_1-MVP/Paladin Project Completion Plan.md
- Paladin is a Rust-based, enterprise-grade multi-agent orchestration framework built on Hexagonal Architecture and Domain-Driven Design.
- Vision: enable enterprises to deploy autonomous AI agents (Paladins) with configurable behaviors; orchestrate multi-agent systems using military-inspired command structures; integrate external tools through standardized protocols; scale from single-agent applications to large distributed agent networks.

## Topic: Development methodology
- source: /workspace/.project/Milestone_1-MVP/Paladin Project Completion Plan.md
- Domain-Driven Design: rich domain models with ubiquitous language, bounded contexts, domain events for cross-context communication, aggregates protecting invariants.
- Test-Driven Development: Red-Green-Refactor for all features; unit test coverage minimum 80%; integration test coverage minimum 70%; contract tests for port implementations.
- Hexagonal Architecture: domain logic isolated from infrastructure; ports define abstract interfaces; adapters implement infrastructure concerns; dependency inversion throughout.
- Professional standards: code review required for all changes; documentation for all public APIs; semantic versioning for releases; continuous integration and deployment.

## Topic: Ubiquitous language (Medieval Military naming)
- source: /workspace/.project/Milestone_1-MVP/Paladin Project Completion Plan.md (Appendix A)
- Paladin = autonomous AI agent capable of reasoning and action. Battalion = coordinated group of Paladins. Formation = sequential execution. Phalanx = concurrent execution. Campaign = graph-based orchestration. Chain of Command = hierarchical delegation. Commander = dynamic Battalion strategy router. Garrison = Paladin memory and context storage. Arsenal = tool and capability registry. Armament = a single tool or capability. Citadel = state persistence and recovery. Herald = output formatting. Armory = CLI tools for development.
- Note: the same document's introductory naming list (section "Naming Convention") describes Garrison as "Agent memory and state persistence", which overlaps Citadel's definition in Appendix A. Appendix A is the fuller glossary.

## Topic: Current state assessment (as of plan authoring)
- source: /workspace/.project/Milestone_1-MVP/Paladin Project Completion Plan.md
- Already complete: Orchestrator (job/task coordination), Scheduler, Queue Service, Listener Service, LLM Port, Notification Ports, Storage Ports (SQL/NoSQL/File), Content Processing.
- Partial: OpenAI Adapter (basic completion support only).
- Required/not yet built at plan time: Paladin Entity (Critical), Paladin Builder (Critical), Garrison memory (High), Arsenal tools (High), MCP Protocol (High), Formation Battalion (Critical), Phalanx Battalion (Critical), Campaign Battalion (High), Chain of Command (High), Commander Router (Medium), Additional Providers DeepSeek/Anthropic (High), State Persistence (Medium).

## Topic: Epic plan, dependencies, and schedule
- source: /workspace/.project/Milestone_1-MVP/Paladin Project Completion Plan.md
- Epic 1 Paladin Domain Foundation — Critical, 3-4 weeks, no dependencies.
- Epic 2 Garrison Memory System — High, 2-3 weeks, depends Epic 1.
- Epic 3 Arsenal Tool System — High, 3-4 weeks, depends Epic 1.
- Epic 4 Battalion Orchestration — Critical, 4-5 weeks, depends Epics 1, 2.
- Epic 5 Commander Strategy Router — Medium, 2 weeks, depends Epic 4.
- Epic 6 Provider Expansion — High, 2-3 weeks, depends Epic 1.
- Epic 7 Citadel State Persistence — Medium, 2 weeks, depends Epics 1, 2.
- Epic 8 Herald Output Formatting — Low, 1-2 weeks, depends Epic 1.
- Epic 9 Armory CLI Tools — Medium, 2-3 weeks, depends Epics 1-4.
- Epic 10 Validation & Documentation — High, 2-3 weeks, depends on all.
- Total estimated duration: 18-24 weeks (4.5-6 months).
- Phases: Phase 1 Foundation (weeks 1-4, Epic 1); Phase 2 Core Capabilities (weeks 5-10, Epics 2, 3, 6); Phase 3 Orchestration (weeks 8-14, Epics 4, 7); Phase 4 Advanced Features (weeks 13-18, Epics 5, 8, 9); Phase 5 Production Ready (weeks 17-20, Epic 10).
- Milestones: M1 Alpha week 6 (single Paladin execution working); M2 Beta week 12 (all Battalion types functional); M3 RC1 week 18 (full feature complete); M4 Release week 20 (production ready).

## Topic: Risk register
- source: /workspace/.project/Milestone_1-MVP/Paladin Project Completion Plan.md
- Technical: MCP protocol complexity (Medium/High — mitigate with early spike); LLM provider API changes (Low/Medium — abstraction via ports, adapter versioning); performance at scale (Medium/High — early load testing, async optimization); graph cycle detection (Low/Low — use proven petgraph library).
- Schedule: scope creep (Medium/High — strict Epic boundaries, change control); integration delays (Medium/Medium — parallel development, mock services); testing bottleneck (Low/Medium — TDD throughout, automated CI).

## Topic: Target source tree layout
- source: /workspace/.project/Milestone_1-MVP/Paladin Project Completion Plan.md (Appendix B)
- `src/core/platform/container/`: paladin.rs, paladin_config.rs, garrison.rs, arsenal.rs, citadel.rs, battalion/{mod.rs, formation.rs, phalanx.rs, campaign.rs, chain_of_command.rs}.
- `src/application/ports/output/`: paladin_port.rs, garrison_port.rs, arsenal_port.rs, battalion_port.rs, citadel_port.rs.
- `src/application/use_cases/`: paladin/{mod.rs, paladin_builder.rs, paladin_execution_service.rs}, battalion/{mod.rs, formation_service.rs, phalanx_service.rs, campaign_service.rs, chain_of_command_service.rs, commander.rs}.
- `src/infrastructure/adapters/`: llm/{openai_adapter.rs, deepseek_adapter.rs, anthropic_adapter.rs}, garrison/{in_memory_garrison.rs, sqlite_garrison.rs}, arsenal/{mcp_client.rs, mcp_stdio_adapter.rs, mcp_sse_adapter.rs}, citadel/file_citadel.rs.
- `src/bin/paladin-cli.rs`.
- Note: this document names the Battalion base module `battalion/battalion.rs` in the Epic 4 technical design while Appendix B and epic4.md name it `battalion/mod.rs`. See INGEST-CONFLICTS.md.

---

## Topic: Epic 1 definition — Paladin Domain Foundation
- source: /workspace/.project/Milestone_1-MVP/Epic_1/epic1.md
- Critical priority, 3-4 weeks, no dependencies, 2 developers. Objective: establish the Paladin as the core domain entity with full lifecycle management, configuration, and execution.
- Technical design blocks give concrete Rust contracts for PaladinData, PaladinStatus, `pub type Paladin = Node<PaladinData>`, PaladinConfig, PaladinPort, PaladinBuilder method chain, PaladinExecutionService (execute, check_stop_words, build_prompt).
- Named unit tests: test_paladin_builder_creates_valid_paladin, test_paladin_builder_validates_required_fields, test_paladin_builder_rejects_invalid_temperature, test_paladin_status_transitions, test_paladin_serialization_roundtrip, test_stop_word_detection, test_max_loops_enforcement.
- Named integration tests: test_paladin_executes_with_mock_llm, test_paladin_respects_timeout, test_paladin_retry_on_failure.
- Definition of Done: all tests passing, code reviewed and approved, documentation complete, merged to main, example code demonstrating usage.

## Topic: Epic 2 definition — Garrison Memory System
- source: /workspace/.project/Milestone_1-MVP/Epic_2/epic2.md
- High priority, 2-3 weeks, depends Epic 1, 1-2 developers.
- Adds a `GarrisonType` enum not present in the Epic 2 PRD: ShortTerm (active conversation context), LongTerm (persisted knowledge), Episodic (specific event memories).
- Provides Rust contracts for GarrisonEntry, ConversationRole, ConversationHistory (entries VecDeque, max_entries, max_tokens), GarrisonPort, LongTermGarrisonPort, InMemoryGarrison, SqliteGarrison.
- Named unit tests: test_garrison_entry_creation, test_conversation_history_windowing, test_token_limit_enforcement, test_memory_search_accuracy, test_garrison_serialization.

## Topic: Epic 3 definition — Arsenal Tool System
- source: /workspace/.project/Milestone_1-MVP/Epic_3/epic3.md
- High priority, 3-4 weeks, depends Epic 1, 2 developers.
- Provides Rust contracts for Armament, ArmamentCall, ArmamentResult, ArsenalPort, ArsenalRegistry (register/unregister/get), MCPClient with MCPCapabilities, MCPTransport, MCPStdioAdapter, MCPSseAdapter.
- Adds a builder method not in the Epic 3 PRD: `PaladinBuilder::add_armament(armament, handler)` for direct in-process tool registration alongside add_mcp_stdio / add_mcp_sse.
- Definition of Done includes MCP protocol compliance verified and documentation including a tool authoring guide.

## Topic: Epic 4 definition — Battalion Orchestration
- source: /workspace/.project/Milestone_1-MVP/Epic_4/epic4.md
- Critical priority, 4-5 weeks, depends Epics 1, 2, 2-3 developers.
- Provides Rust contracts for BattalionConfig, BattalionResult (battalion_id, battalion_name, started_at, completed_at, final_output, paladin_results, status), ErrorStrategy, Formation (id, config, paladins, shared_context), Phalanx (id, config, paladins, aggregation), AggregationStrategy, Campaign (DiGraph<Paladin, CampaignEdge>, entry_points), CampaignEdge, EdgeCondition, ChainOfCommand (commander, specialists, delegation_strategy), DelegationStrategy, BattalionPort (execute, status, cancel), FormationExecutionService, PhalanxExecutionService.
- Battalion base module named `battalion/mod.rs` here.
- Named tests: test_formation_paladin_ordering, test_formation_output_passing, test_phalanx_parallel_execution, test_phalanx_aggregation_strategies, test_campaign_graph_validation, test_campaign_edge_conditions, test_chain_of_command_delegation; integration: test_formation_end_to_end, test_phalanx_concurrent_llm_calls, test_campaign_conditional_routing, test_chain_of_command_specialist_selection.

## Topic: Epic 5 definition — Commander Strategy Router
- source: /workspace/.project/Milestone_1-MVP/Epic_5/epic5.md
- Medium priority, 2 weeks, depends Epic 4, 1 developer.
- Provides Rust contract for BattalionStrategy enum and Commander struct holding the four Arc-wrapped services, plus `resolve_strategy` / `analyze_and_select` control flow with `Auto => unreachable!("resolved above")`.
- Constructor shown as `Commander::new(strategy, paladins)`; the Epic 5 PRD instead specifies a fluent `Commander::builder()`.

## Topic: Epic 6 definition — Provider Expansion
- source: /workspace/.project/Milestone_1-MVP/Epic_6/epic6.md
- High priority, 2-3 weeks, depends Epic 1, 1-2 developers.
- Provides Rust contracts for DeepSeekAdapter/DeepSeekConfig and AnthropicAdapter/AnthropicConfig, and the LlmPort impl surface (generate, generate_stream, validate_model, get_available_models, get_provider_name). Note this DOC's LlmPort surface omits `get_capabilities()`, which the Epic 6 PRD requires (REQ-1/REQ-2).
- Acceptance: DeepSeek and Anthropic work with all Paladin features; providers interchangeable via configuration; streaming works for both; tool use format supported for Anthropic.

## Topic: Epic 7 definition — Citadel State Persistence
- source: /workspace/.project/Milestone_1-MVP/Epic_7/epic7.md
- Medium priority, 2 weeks, depends Epics 1, 2, 1 developer.
- Provides Rust contracts for PaladinState, BattalionState, CitadelPort, and PaladinBuilder integration (enable_autosave, save_state_dir, restore_from).
- `restore_from(self, state_id: Uuid) -> Self` here; the Epic 7 PRD specifies `restore_from(mut self, state_id: Uuid) -> Result<Self, PaladinError>`. PRD governs — see INGEST-CONFLICTS.md INFO.
- Acceptance: state persists across restarts; autosave triggers on configurable events; Battalion workflows resume from checkpoints; state files human-readable JSON.

## Topic: Epic 8 definition — Herald Output Formatting
- source: /workspace/.project/Milestone_1-MVP/Epic_8/epic8.md
- Low priority, 1-2 weeks, depends Epic 1, 1 developer.
- Provides a two-method Herald trait (format_paladin_result, format_battalion_result, both returning String) and MarkdownHerald / JsonHerald / TableHerald structs, with a concrete MarkdownHerald format! implementation. The Epic 8 PRD requires a larger method set — PRD governs.
- Acceptance: results formattable as Markdown, JSON, or tables; metadata included; Battalion results show individual Paladin contributions.

## Topic: Epic 9 definition — Armory CLI Tools
- source: /workspace/.project/Milestone_1-MVP/Epic_9/epic9.md
- Medium priority, 2-3 weeks, depends Epics 1-4, 1 developer.
- Provides the clap derive skeleton for `src/bin/paladin-cli.rs`: Cli -> Commands { Agent, Battalion, Arsenal } -> AgentCommands { Run { config, input }, New { name, output } }.
- Example invocations: `paladin agent run --config analyst.yaml --input "Analyze Q4 revenue"`; `paladin battalion run --type formation --config workflow.yaml`; `paladin arsenal list`; `paladin arsenal test --mcp-stdio "uvx mcp-hn"`.

## Topic: Epic 10 definition — Validation & Documentation
- source: /workspace/.project/Milestone_1-MVP/Epic_10/epic10.md
- High priority, 2-3 weeks, depends on all previous epics, full team.
- Deliverables: integration test suite (end-to-end Paladin, multi-Paladin Battalion, MCP server, provider tests with mocks, load testing for concurrent Phalanx); API reference via rustdoc; user guide; architecture documentation; examples gallery.
- Acceptance: integration test coverage >= 70%; all public APIs documented; examples compile and run; performance benchmarks established; production deployment guide complete.

---

## Topic: Implementation status — Epic 1 (Paladin Domain Foundation)
- source: /workspace/.project/Milestone_1-MVP/Epic_1/tasks-paladin-domain-foundation.md
- 182 of 182 checklist items complete (100%). All parent tasks done: feature branch, core domain layer, application ports, PaladinBuilder with validation (TDD), circuit breaker, PaladinExecutionService with retry (TDD), mock LLM adapter, integration tests, documentation and examples.

## Topic: Implementation status — Epic 2 (Garrison Memory System)
- source: /workspace/.project/Milestone_1-MVP/Epic_2/tasks-garrison-memory-system.md
- 161 of 165 complete (98%). Parent tasks 0.0-10.0 done (core domain, GarrisonPort traits, in-memory adapter, SQLite adapter, token counting, Paladin integration, config and error handling, unit tests, integration tests, docs and examples).
- Outstanding: 11.0 Final Validation and Cleanup — specifically 11.5 verify coverage >= 80% via `cargo llvm-cov`, 11.6 review all PRD acceptance criteria. Also deferred: 9.14 `test_large_conversation_performance` benchmark with 1000 entries (marked future enhancement).

## Topic: Implementation status — Epic 3 (Arsenal Tool System)
- source: /workspace/.project/Milestone_1-MVP/Epic_3/tasks-arsenal-tool-system.md
- 220 of 223 complete (99%). All feature parent tasks done: domain tool definitions (FR-1), Arsenal ports (FR-2), MCP protocol core (FR-3), STDIO transport (FR-4), SSE transport (FR-5), PaladinBuilder integration and configuration (FR-6), resource controls (FR-7), error handling and context injection (FR-8, FR-9), comprehensive testing and documentation.
- Outstanding items are version-control steps only: 9.30 commit changes, 9.31 push feature branch `feature/epic3-arsenal-tool-system`.

## Topic: Implementation status — Epic 4 (Battalion Orchestration)
- source: /workspace/.project/Milestone_1-MVP/Epic_4/tasks-battalion-orchestration.md
- 233 of 235 complete (99% of items) but two parent tasks are NOT complete: 6.0 Implement Chain of Command Pattern (Phase 2 — Hierarchical Delegation) and 7.0 Integration Testing, Performance Validation & Documentation.
- Complete: core Battalion infrastructure (domain), application layer (ports and base services), Formation (Phase 1 sequential), Phalanx (Phase 1 concurrent), Campaign (Phase 2 graph orchestration).

## Topic: Implementation status — Epic 5 (Commander Strategy Router)
- source: /workspace/.project/Milestone_1-MVP/Epic_5/tasks-commander-strategy-router.md
- 150 of 154 complete (97%). Committed: BattalionStrategy enum and core types (6c62406), Commander struct and builder (6c8f059), Auto-mode selection logic (6a15ca4, noted 13/14 tests passing), execute method and service delegation (6a15ca4), error handling strategies, configuration passthrough, unit tests, integration tests for all strategies, example usage, documentation (e7cdd75).
- Outstanding: 5.0 Implement result normalization and telemetry metadata — 5.10 metadata export to file when `metadata_output_dir` is configured (deferred, requires file I/O), 5.14 `test_metadata_export_to_file` (deferred). Also failing: 3.11 `test_auto_selects_campaign_for_workflow_keywords` (marked FAILING - needs fix).

## Topic: Implementation status — Epic 6 (Provider Expansion)
- source: /workspace/.project/Milestone_1-MVP/Epic_6/tasks-provider-expansion.md
- 180 of 199 complete (90%). Complete: LlmPort trait enhanced with provider capabilities, DeepSeek adapter, Anthropic adapter, provider factory and configuration system, OpenAI adapter consistency pass, unit tests with mocked HTTP (27 new tests: 9 DeepSeek, 10 Anthropic, others), examples and documentation, backward-compatibility validation and final QA.
- Outstanding: entire parent task 7.0 "Write integration tests for live API validation" is marked DEFERRED with rationale "unit tests with mocks provide sufficient coverage" — 18 subtasks (7.1-7.18) covering `tests/integration/llm/` organization, DeepSeek/Anthropic live tests marked `#[ignore]`, provider-switching tests, CI configuration notes for optional live API tests (REQ-26), and documentation of how to run them.

## Topic: Implementation status — Epic 7 (Citadel State Persistence)
- source: /workspace/.project/Milestone_1-MVP/Epic_7/tasks-citadel-state-persistence.md
- 169 of 169 complete (100%). All parent tasks done: Citadel domain types, CitadelPort trait, FileCitadel adapter, PaladinBuilder integration, serialization support for domain entities, unit tests, integration tests for file persistence, configuration support, documentation and examples, final validation and cleanup.

## Topic: Implementation status — Epic 8 (Herald Output Formatting)
- source: /workspace/.project/Milestone_1-MVP/Epic_8/tasks-herald-output-formatting.md
- 139 of 141 complete (99%) but parent task 7.0 "Integrate Herald with Paladin/Battalion execution" is NOT complete.
- Complete: core Herald infrastructure (trait, errors, types), Herald Registry, JsonHerald, MarkdownHerald, TableHerald, configuration integration, streaming support, comprehensive testing suite, documentation and examples.
- Outstanding: 7.0 integration with execution path; 7.13 integration tests for Battalion with Herald (deferred — needs Battalion execution setup).

## Topic: Implementation status — Epic 9 (Armory CLI Tools)
- source: /workspace/.project/Milestone_1-MVP/Epic_9/tasks-armory-cli-tools.md
- 238 of 241 complete (99%). All 15 parent tasks complete: CLI project structure, clap argument parsing, configuration loading and validation, `agent new`, `agent run`, `battalion new`, `battalion run`, `arsenal list`, `arsenal test`, output formatting and error handling, interactive prompts, unit tests, integration tests, documentation and example configurations, final validation.
- Outstanding: 13.4-13.6 end-to-end tests (run Paladin from config with mock LLM adapter; run Formation with multiple mock Paladins; run Phalanx with parallel execution) — all deferred pending CLI mock provider support.

## Topic: Implementation status — Epic 10 (Validation & Documentation)
- source: /workspace/.project/Milestone_1-MVP/Epic_10/tasks-epic10-validation-documentation.md
- 103 of 103 checklist items complete (100%). Parent tasks 0.0-6.0 all marked complete: feature branch, user-facing documentation (FR-USER, FR-API), technical documentation (FR-ARCH, FR-DEPLOY, FR-OPS, FR-CONTRIB), integration testing infrastructure (FR-INT), performance benchmarking (FR-PERF), deployment automation (FR-DEPLOY), validation and QA (all FRs).
- This checklist contains no Task 7.0. The Epic 10 validation report references an outstanding Task 7.0 — see INGEST-CONFLICTS.md WARNING.

## Topic: Implementation status — unit test coverage improvements
- source: /workspace/.project/Milestone_1-MVP/unit-test-improvements/tasks-improve-unit-test-coverage.md
- 42 of 44 complete (95%). Complete: coverage report analysis and priority-file identification, tests for low-coverage files (<50%), tests for moderate-coverage files (50-80%), critical-path coverage in core functionality.
- Outstanding: 2.0 Add unit tests for files with 0% coverage; 6.0 Improve unit test coverage in gaps and verify coverage improvements.
- Targeted areas per scope: src/main.rs, user_controller, sqlite_user_repository, MinIO adapter, OpenAI/Anthropic/DeepSeek adapters, content_ingestion_service, content_aggregator_service, CLI commands (agent, arsenal, battalion, interactive, user_commands), config/user_config, config/setup/service_runner.

---

## Topic: Measured critical-path coverage analysis
- source: /workspace/.project/Milestone_1-MVP/unit-test-improvements/COVERAGE_ANALYSIS.md
- Stated target 85% overall; stated current baseline 70.56% (with Task 4.0 improvements). 959 tests passing at time of writing.
- Well covered: paladin_execution_service.rs 94% lines (296/372), paladin.rs 95%+, paladin_config.rs 100%, paladin_builder.rs 85%+, circuit_breaker.rs 100%, error.rs 100%; formation_service.rs 88.14%, phalanx_service.rs 87.93%, commander.rs 81.79%, error_aggregation.rs 99.60%, retry.rs 100%; phalanx.rs entity 97.96%, chain_of_command.rs entity 93.36%, campaign.rs entity 74.38%; garrison.rs 92.23%, in_memory_garrison.rs 96.49%, sqlite_garrison.rs 81.75%, token_counter.rs 98.44%, garrison_port.rs 100%; citadel.rs 99.32%, file_citadel.rs 85.85%, citadel_port.rs 100%, citadel_error.rs 98.06%; tool_result_formatter.rs 90.31%, resource_controls.rs 96.94%, arsenal.rs entity 100%.
- Critical gaps ranked: (P1) arsenal_execution_service.rs 0.00% (0/46 lines) and arsenal_registry_service.rs 0.00% (0/28 lines) — no tests for tool invocation workflow or tool discovery/registration; (P2) campaign_service.rs 4.26% lines — graph/DAG orchestration untested; (P3) chain_of_command_service.rs 13.41% lines, 0 tests; (P4) mcp_protocol.rs 15.83%, mcp_stdio_adapter.rs 25.33%, mcp_sse_adapter.rs 27.84%.
- Missing test files identified: Campaign service integration tests, Chain of Command service integration tests.
- Recommended new test files: tests/integration/arsenal_execution_integration_test.rs, tests/integration/battalion_campaign_integration_test.rs, tests/integration/battalion_chain_of_command_integration_test.rs, tests/integration/mcp_protocol_integration_test.rs.
- Projected impact of closing these gaps: overall coverage ~71% -> ~82-85%; ~40-50 new integration tests; projected total ~1,000-1,010 tests.

## Topic: Validation and QA results (Epic 10 Task 6.0)
- source: /workspace/.project/Milestone_1-MVP/Epic_10/task6.0-validation-report.md
- Dated January 27, 2026. Overall status recorded as PASSED with caveats; 16 of 16 subtasks validated. No blockers recorded.
- Unit tests: 706 passed, 0 failed, 6 ignored, 4.59s. Integration tests: 385 passed, 0 failed, 10 ignored, 19.87s. Doc tests: 133 passed, 0 failed, 76 ignored. Total 1091 tests, zero failures.
- Unit test coverage measured 60.88% (27,700/45,498 lines) against an 80% target — 19.12 points short. Integration coverage measured 67.79% (30,619/45,563) against a 70% target — 2.21 points short. Both were explicitly judged "acceptable for MVP".
- Low-coverage modules recorded: openai_adapter.rs 18.67%, deepseek_adapter.rs 15.02%, anthropic_adapter.rs 28.19%, mysql_content_repository.rs 5.89%, sqlite_content_repository.rs 13.85%; zero coverage: redis.rs, minio.rs, user_controller.rs, sqlite_user_repository.rs.
- Code quality: `cargo fmt --check` fully compliant; `cargo clippy -- -D warnings` at 0 warnings after fixing 102 across 48 files (81 collapsible_if, 8 type_complexity, 6 derivable_impls, 2 too_many_arguments, 1 manual_strip, 1 let_and_return, 1 doc_lazy_continuation).
- Security: `cargo audit` reports 2 medium transitive vulnerabilities — RUSTSEC-2023-0071 rsa 0.9.10 (Marvin Attack, via sqlx-mysql, no fix available, judged not exploitable for MySQL-only use) and RUSTSEC-2025-0111 tokio-tar 0.3.1 (PAX header file smuggling, via testcontainers, dev/test only). Plus 9 unmaintained-crate warnings: ansi_term, atty (also unsound), dotenv, fxhash, gcc, number_prefix, proc-macro-error, rustls-pemfile. Recommended replacements: dotenv -> dotenvy, structopt -> clap v4, alternatives to fasthash.
- Examples: 22 of 22 compile. Named: arsenal_sse_tools, arsenal_stdio_tools, basic_paladin, battalion_checkpoint_recovery, campaign_workflow, chain_of_command_delegation, citadel_autosave, citadel_restore, commander_auto, commander_basic, commander_full_config, formation_sequential, garrison_in_memory, garrison_persistent, garrison_semantic_search, herald_custom_formatter, herald_json_output, herald_markdown_output, herald_streaming, llm_provider_selection, paladin_with_config, phalanx_parallel.
- Benchmarks: DISABLED. paladin_benchmarks.rs needs LlmPort trait; battalion_benchmarks.rs needs Campaign/ChainOfCommand API updates; garrison_benchmarks.rs, herald_benchmarks.rs fully commented out; arsenal_benchmarks.rs needs domain rewrite. "No regressions possible (no active benchmarks)". Restore planned post-MVP.
- Docker: build 5m31s against a <5min target (+10%); image size 112MB against a <500MB target. Link checker skipped (no tool installed).
- Documentation: 24 files, ~5,000+ lines; user guides (QUICKSTART, INSTALLATION, 6 specialized guides), technical docs, deployment, operations, contributing, 22-example gallery.
- Recorded next task: Task 7.0 Final Documentation Review (6 subtasks); Epic 10 progress stated as 101 of 102 subtasks (99%).

## Topic: Deployment automation infrastructure (Epic 10 Task 5.0)
- source: /workspace/.project/Milestone_1-MVP/Epic_10/task5.0-completion-summary.md
- Docker: multi-stage Dockerfile (61 lines) with Rust 1.82 builder and `gcr.io/distroless/cc-debian12` runtime, non-root UID 65532, exposes 8080 (HTTP) and 9090 (metrics), includes migrations and config. `.dockerignore` (72 lines). `Dockerfile.chef` (100 lines) kept as a cargo-chef backup.
- Kubernetes manifests in `k8s/`: namespace.yaml (18), configmap.yaml (74), secret.yaml.example (73), deployment.yaml (189), service.yaml (55), redis.yaml (45), minio.yaml (67), README.md (467).
- Deployment characteristics: 3 replicas, RollingUpdate (maxSurge 1, maxUnavailable 0), init containers wait-for-redis and wait-for-minio, runAsNonRoot UID/GID 65532 with fsGroup 65532 and seccomp RuntimeDefault, allowPrivilegeEscalation false, readOnlyRootFilesystem where possible, DROP ALL capabilities, pod anti-affinity.
- Resources: CPU 500m request / 2000m limit; memory 256Mi request / 512Mi limit — sized from benchmark results (Garrison ops 170-380ns single and 3.35µs realistic; Battalion overhead 1.8µs Formation, 25µs Phalanx; Herald formatting 570ns-23µs).
- Probes: liveness /health every 30s (3 failures, 10s timeout); readiness /ready every 10s (3 failures, 5s timeout); startup /ready every 3s (10 failures, 30s max startup). Volumes: config ConfigMap, data EmptyDir 1Gi, logs EmptyDir 500Mi.
- Services: ClusterIP (80 -> 8080, 9090 -> 9090), Headless for direct pod access, Metrics for Prometheus scraping.
- GitHub Actions: release.yml (220 lines) triggered on v*.*.* tags and manual dispatch — create-release with changelog, build-docker multi-arch (linux/amd64, linux/arm64 via QEMU + Buildx cache, push to ghcr.io, warns if image >500MB), build-binaries cross-compiled for x86_64/aarch64 linux-gnu and x86_64/aarch64 apple-darwin with stripping, compression and SHA256 checksums. integration-tests.yml (~180 lines) triggered on PR, push to main/develop, daily 02:00 UTC and manual — integration-tests job (Redis + MinIO as services, `cargo test --features integration-tests` with --test-threads=1, cargo-llvm-cov, Codecov upload under 'integration' flag), docker-integration job (docker-compose, logs on failure, `down -v`), kubernetes-smoke-test job (kind cluster, deploy Redis/MinIO then Paladin manifests, 180s pod wait, measures startup time and validates the <30s requirement, cleanup).
- Cargo.toml changes: edition changed from "2024" to "2021" for stable Rust compatibility; benchmark targets commented out for Docker builds.
- Total: 13 files, ~1,621 lines of deployment infrastructure.
- Self-reported status at time of writing: Task 5.0 was 10 of 14 subtasks complete (71%), with Docker image size verification, local kind deployment testing, and pod startup time validation still pending. The Task 6.0 report later records measured Docker outcomes (112MB image, 5m31s build). See INGEST-CONFLICTS.md INFO.

---

# Context — Ingest run 2 (Milestone 2 + Milestone 3)

Ingest run 2 of 5 — 30 DOC-typed documents from `.project/Milestone_2-Missing_features`
and `.project/Milestone_3-Completion`. Appended in merge mode; run-1 topics above are
unchanged.

Completion state for these milestones is NOT taken from prose in these documents. It
is taken from `.planning/intel/task-completion-state.md` (deterministic checkbox
counts). Where a document claims completion that the checkbox counts or the committed
codebase map contradict, the claim is recorded here as a claim and the contradiction is
logged in `.planning/INGEST-CONFLICTS.md`.

---

## Topic: Milestone 3 plan — structure, scope origin and exclusions
- source: /workspace/.project/Milestone_3-Completion/Project_Plan_Milestone_3.md
- Status recorded as "Draft", created February 9, 2026. Six new epics (19-24), 8-10 weeks total.
- Purpose: close all deferred tasks from Epics 1-18 plus outstanding inline `TODO` items. Scope was derived mechanically from (1) `grep -ir "deferred" ./project/Milestone_*/**/tasks-*.md` and (2) inline TODOs in `./src/`, `./tests/`, `./benches/`.
- Timeline: Weeks 1-2 Epic 19, 2-3 Epic 20, 3-5 Epic 21, 5-7 Epic 22, 7-8 Epic 23, 9-10 Epic 24.
- Epic table: 19 Herald & Domain Type Consolidation (Critical, no deps, origin Epic 8 TODOs); 20 Vision Pipeline Completion (High, dep 19, origin Epic 13); 21 Autonomous Agent Completion (Critical, dep 19, origin Epic 14 — 23 deferred items); 22 Battalion & Commander Hardening (High, dep 19, origin Epics 4, 5, 15, 16); 23 CLI, Config & Infrastructure Completion (Medium, dep 19-22, origin Epics 9, 10, 18); 24 Test Hardening, Benchmarks & QA (High, dep all, origin deferred tests across all milestones).
- Explicitly OUT of scope (pre-dates the multi-agent epics): `src/application/storage/sql_store.rs`, the three `content_*_service.rs` files, `src/config/setup/service_runner.rs`, `src/core/platform/container/trigger.rs`, `src/core/platform/manager/queue_service.rs`, `src/infrastructure/adapters/input/file_content_fetcher.rs`, `email_notification_adapter.rs`, `file_content_repository.rs`, `mysql_content_repository.rs`, `sqlite_content_repository.rs`, `tests/integration/system_log_integration_test.rs`.
- Cross-cutting coverage targets stated in this plan: core domain >= 85%, application services >= 80%, infrastructure adapters >= 70%, CLI commands >= 70%, overall >= 75%. See INGEST-CONFLICTS.md — this is a third position on the project-wide coverage gate.
- Quality gates per epic: `cargo test`, `cargo fmt --check`, `cargo clippy -- -D warnings`, `cargo doc --no-deps`, Snyk scan with no new vulnerabilities.
- Appendix A is a full traceability table mapping 9 Milestone-1 deferred items and 31 Milestone-2 deferred items to target epics, plus a 30-row inline-TODO table with file and line numbers. Appendix B lists key files per epic.
- The Epic 23 section in this plan was later edited in place to `Status: COMPLETE (February 14, 2026)` with all its checklist items flipped to `[x]`; Epics 19-22 and 24 remain unchecked in the same document.

## Topic: Epic 11 definition — Sanctum Memory Foundation
- source: /workspace/.project/Milestone_2-Missing_features/Epic_11/epic11.md
- Theme "Vector Embedding Infrastructure", 2 weeks, Critical, no dependencies.
- Five user stories: US-11.1 EmbeddingPort, US-11.2 OpenAI embedding adapter, US-11.3 SanctumPort, US-11.4 **In-Memory Vector Store**, US-11.5 Sanctum domain model. Qdrant is not part of Epic 11 in this document.
- Definition-of-Done code blocks give concrete signatures: `EmbeddingPort {embed_text, embed_batch, dimension, model_name}`, `Embedding {vector, model, dimension, token_count}`, `SanctumPort {store, store_batch, search, delete, update, count}`, `SanctumEntry {id, paladin_id, content, embedding, metadata, timestamp}`, `SanctumQuery {embedding, top_k, filter, min_score}`, `SanctumSearchResult {entry, score}`, `InMemorySanctum {entries: Arc<RwLock<HashMap>>, config}`, `Memory {id, paladin_id, content, memory_type, importance, access_count, last_accessed, created_at, metadata}`, `MemoryType {Episodic, Semantic, Procedural}`.
- Note: the US-11.1 acceptance bullet writes `embed_text(&str) -> Result<Vec<f32>, EmbeddingError>` while the Definition of Done in the same section returns `Result<Embedding, EmbeddingError>`. The PRD form (Embedding) wins by precedence.

## Topic: Implementation status — Epic 11 (Sanctum Memory Foundation)
- source: /workspace/.project/Milestone_2-Missing_features/Epic_11/EPIC_11_COMPLETION_SUMMARY.md
- Self-reported status COMPLETE, branch `feature/epic11-sanctum-memory-foundation`, completion date January 30, 2026.
- Delivered: EmbeddingPort + EmbeddingError + Embedding; OpenAI embedding adapter with retry, rate-limit handling and batch up to 2048; Sanctum domain models (Memory, MemoryType, MemoryDecayStrategy, SanctumEntry) with builder; SanctumPort with SanctumQuery/SanctumFilter/SanctumSearchResult; InMemorySanctum with brute-force cosine, LRU eviction, full filtering, `Arc<RwLock<HashMap>>`.
- **Task 5.0 Qdrant Adapter: DEFERRED** — "Not implemented (requires external Qdrant service)". Feature flag `qdrant` prepared in Cargo.toml; architecture designed for future addition; docs include Qdrant usage patterns. Listed again under "Future Enhancements — Qdrant Adapter (Epic 12?)".
- Measured performance (InMemory): single store ~640 ns at all dimensions; batch store of 100 ~30 µs; search at 10K vectors under 100ms (reported ~80-90ms); batch throughput ~3000/sec.
- Test state reported: 999 unit tests passing; 17 Sanctum integration tests passing; domain model coverage 100%; unit coverage stated as "> 80%".
- Documentation produced: `docs/SANCTUM.md`, `docs/SANCTUM_DEPLOYMENT.md`, `docs/SANCTUM_MIGRATION.md`, `docs/SANCTUM_BENCHMARKS.md`; 5 examples (`sanctum_basic_inmemory.rs`, `paladin_with_sanctum.rs`, `sanctum_qdrant_production.rs`, `sanctum_adapter_migration.rs`, `sanctum_configuration.rs`); `benches/sanctum_benchmarks.rs`.
- Code metrics: 23 new files, 8 modified, ~15,000 lines added of which ~7,400 documentation, 12 conventional commits.
- Known limitations recorded: InMemory unsuitable above 10K vectors (O(n) search), no persistence, single-process; Qdrant adapter absent; 2 transitive dependency advisories (rustls-pemfile unmaintained, atty unsound).
- Garrison vs Sanctum comparison table is the clearest statement of the memory-system boundary in the corpus: Garrison = short-term, single session, sequential/recent retrieval, dozens of entries, in-memory only; Sanctum = long-term, across sessions, semantic similarity search, thousands of entries, persistent.
- The document also asserts "All acceptance criteria from the original PRD have been met or exceeded" — see INGEST-CONFLICTS.md, the Epic 11 PRD required the Qdrant adapter and a 100K-vector benchmark.
- Proposed next step at time of writing: tag release `v0.2.0-sanctum`; implement Qdrant in Epic 12.

## Topic: Epic 12 definition — Sanctum RAG Integration
- source: /workspace/.project/Milestone_2-Missing_features/Epic_12/epic12.md
- Theme "Retrieval-Augmented Generation", 2 weeks, Critical, depends on Epic 11.
- Five user stories: Qdrant vector store adapter, Paladin long-term memory integration, RAG retrieval service, memory extraction service, PaladinExecutionService RAG integration.
- Definition-of-Done blocks give `QdrantSanctum {client, config}` with `QdrantSanctumConfig {url, api_key, collection_name, vector_size, distance, on_disk}`; `MemoryExtractionStrategy {EveryTurn, OnCompletion, Manual, Threshold{importance}}`; `RagConfig {top_k, min_similarity, max_tokens, retrieval_trigger}`; `RagRetrievalService {sanctum, embedding, config}` with `retrieve_context` and `format_for_prompt`; `MemoryExtractionService {llm, embedding, sanctum}` with `extract_memories` and `store_memories`.
- The document sketches the revised `PaladinExecutionService::execute()` as: retrieve memories -> build prompt with memory context -> execute LLM -> extract and store new memories.

## Topic: Epic 13 definition — Sentinel Vision System
- source: /workspace/.project/Milestone_2-Missing_features/Epic_13/epic13.md
- Theme "Multi-Modal Input Processing", 2 weeks, Critical, depends on Epic 6 (Provider Expansion).
- Six user stories: vision request model, OpenAI vision, Anthropic vision, Paladin vision API, PDF text extraction, document ingestion port.
- Definition-of-Done blocks place `VisionContent`/`ImageDetail`/`VisionRequest` in `src/core/platform/container/vision.rs`, add `generate_with_vision()` to `OpenAILlmAdapter`, define `VisionCapableLlm: LlmPort` with `generate_with_vision` and `supports_vision`, give `Paladin::run_with_vision(task, images)`, define `PdfExtractor::{extract, extract_bytes}` returning `Document {pages, metadata, total_chars}`, and define `DocumentPort {ingest, chunk}` with `DocumentSource {File, Bytes{data, format}, Url}` and `ChunkConfig {chunk_size, chunk_overlap, separator}`.
- Anthropic `supports_vision()` is sketched as `self.model.starts_with("claude-3")`.
- The DOC's `DocumentPort::chunk` returns `Vec<DocumentChunk>` directly; the PRD example calls it fallibly with `?`. PRD wins by precedence.

## Topic: Epic 14 definition — Autonomous Agent Features
- source: /workspace/.project/Milestone_2-Missing_features/Epic_14/epic14.md
- Theme "Agent Self-Direction and Planning", 2 weeks, Critical, depends on Epic 1.
- Five user stories: autonomous planning mode, auto-generate system prompt, dynamic temperature, handoff infrastructure, handoff tool.
- Definition-of-Done blocks define `MaxLoops {Fixed(u32), Auto{max_subtasks}}`, `PlanningService {llm}` with `create_plan`/`execute_plan`, `TaskPlan {original_task, subtasks, dependencies: HashMap<usize, Vec<usize>>}`, `Subtask {id, description, expected_output}`, `PromptGenerationService::generate_prompt(agent_name, agent_description, task_context)`, `TemperatureService::{analyze_task, recommend_temperature}` with `TaskType {Factual, Analytical, Conversational, Creative}`, `HandoffStrategy {Automatic, Explicit, Threshold{confidence}}`, `HandoffService::{should_handoff, execute_handoff}`, `HandoffDecision {target_agent, reason, context_to_transfer}`, and `HandoffTool::schema()` returning an `Armament` named `handoff_to_agent` with required params `agent_name` and `message`.
- Temperature bounds stated in this DOC: 0.1 - 1.0. Note this differs from the `[0.0, 1.0]` builder clamp (Epic 1) and the 0.0-2.0 DeepSeek range (Epic 6) already recorded in run 1.
- Epic 14 completion criteria refer to the planning mode as `max_loops="auto"` (string form) while the type is an enum.

## Topic: Epic 15 definition — Conclave (MixtureOfAgents)
- source: /workspace/.project/Milestone_2-Missing_features/Epic_15/epic15.md
- Theme "Expert Synthesis Orchestration", 2 weeks, High, depends on Epic 4.
- Four user stories: Conclave domain model, execution service, Commander Conclave strategy, Conclave CLI support.
- The DOC's `ConclaveConfig` has no `retry_attempts` and no `observability_level`; its `ConclaveResult` has no `expert_execution_times` and no `retry_counts`. The PRD adds all four. PRD wins by precedence.
- The DOC records the `BattalionStrategy` enum at this point as `{Formation, Phalanx, Campaign, ChainOfCommand, Conclave, Auto}`.
- Auto-strategy heuristics for Conclave stated as: multiple agents with distinct descriptions; task requires synthesis/comparison; keywords "compare", "synthesize", "combine perspectives".

## Topic: Implementation status — Epic 15 (Conclave)
- source: /workspace/.project/Milestone_2-Missing_features/Epic_15/epic-15-completion-report.md and /workspace/.project/Milestone_2-Missing_features/Epic_15/epic-15-pull-request-description.md
- Self-reported status COMPLETE / "PRODUCTION-READY", branch `feature/epic-15-conclave-pattern`, 5 days actual, dated February 2, 2026, 9 commits (b8d9175 domain model, da4a749 execution service, e341068 Commander integration, b20cb74 CLI/YAML, 2caf2f2 docs/examples, plus fixes).
- Files reported: `src/core/platform/container/battalion/conclave.rs` (382 lines), `src/application/use_cases/battalion/conclave_execution_service.rs` (757 lines), `commander.rs` modified, **`src/cli/config/battalion_config.rs`** modified (pre-Epic-17.5 CLI path), `examples/conclave_expert_panel.rs` (510 lines), two YAML configs, `docs/guides/conclave-pattern.md` (900+ lines), 35 domain tests + 10 execution-service tests + 19 CLI tests.
- Test state reported: 1,292 unit tests passing (0 failed, 6 ignored), 168 doc tests, 560+ integration tests, estimated 85%+ coverage for Conclave code, zero clippy warnings, all 22+ examples compile.
- `ConclaveStatus` is reported as `{Completed, PartialSuccess, Failed}` — the PRD and epic DOC both specify `Success` rather than `Completed`.
- Validation reported as "≥2 experts required"; the duplicate-agent-name check required by PRD FR-C4 is not mentioned.
- Design decisions recorded: `tokio::spawn` per expert with `join_all`; exponential backoff `2^attempt` seconds with 20% jitter and per-expert retry counters; Conclave continues if at least 1 expert succeeds; three observability levels with Standard as default; optional `max_expert_output_tokens` truncation at roughly 4 chars/token.
- Known limitations recorded: aggregation is sequential (no streaming), no expert cancellation or early quorum, `synthesis_prompt` fully replaces the aggregator system prompt with no templating, and limited testing against Anthropic/DeepSeek.
- Security assessment: 2 medium transitive advisories (rsa 0.9.10 RUSTSEC-2023-0071, tokio-tar 0.3.1 RUSTSEC-2025-0111), 9 unmaintained transitive crates, no hardcoded secrets.
- The report states "Deviations from Original PRD: None" and lists five enhancements beyond the PRD (token management, expert attribution, custom synthesis prompt, three observability levels, CLI template generation).
- Claim to verify: `task-completion-state.md` records 129 open checkboxes in `tasks-conclave-mixture-of-agents.md`, the largest open concentration in the whole corpus, while this report states "All tasks marked complete".

## Topic: Epic 16 definition — Advanced Battalion Patterns (Council, Grove)
- source: /workspace/.project/Milestone_2-Missing_features/Epic_16/epic16.md
- Theme "Council and Tree-based routing", 2 weeks, Medium, depends on Epic 4 and Epic 15.
- Five user stories: Council domain model (called GroupChat), Council execution service, Grove domain model, Grove execution service, Commander integration.
- Definition-of-Done blocks define `Council {name, participants, moderator: Option<Paladin>, config}`, `CouncilConfig {max_rounds, turn_strategy, termination_condition, include_history}`, `TurnStrategy {RoundRobin, Random, ModeratorDirected, VoluntaryWithTimeout{timeout_ms}}`, `TerminationCondition {MaxRounds, Consensus, ModeratorDecision, Keyword(String)}`, `CouncilMessage {speaker, content, round, timestamp}`, `CouncilExecutionService::convene`, `CouncilResult {transcript, conclusion, rounds_completed, termination_reason}`, `Grove {name, trees, config}`, `Tree {name, agents}`, `TreeAgent {paladin, expertise_keywords, expertise_embedding}`, `GroveConfig {routing_strategy, fallback_tree, similarity_threshold}`, `RoutingStrategy {KeywordMatch, SemanticSimilarity, LlmRouting}`, `GroveExecutionService {paladin_port, embedding_port: Option}` with `execute` and `route_task`, `RoutingDecision {selected_tree, selected_agent, confidence, reasoning}`.
- The DOC lists `Random` and `VoluntaryWithTimeout` as TurnStrategy variants; the Epic 16 PRD defers both (NG-6). PRD wins by precedence.
- The DOC records `BattalionStrategy` as `{Formation, Phalanx, Campaign, ChainOfCommand, Conclave, Council, Grove, Auto}`.
- Grove is also referred to as "ForestSwarm" in the Epic 16 completion criteria.

## Topic: Epic 17 definition — Flow DSL & Agent Rearrangement
- source: /workspace/.project/Milestone_2-Missing_features/Epic_17/epic17.md
- Theme "Flexible Workflow Definition", 2 weeks, Medium, depends on Epics 4, 15, 16.
- Five user stories: Flow DSL parser, Maneuver domain model (AgentRearrange), Maneuver execution service, Commander Maneuver strategy, flow visualization.
- Definition-of-Done blocks define `FlowParser::parse(&str) -> Result<FlowExpression, FlowParseError>`, `FlowExpression {Agent(String), Sequential(Vec), Parallel(Vec)}`, `Maneuver {name, agents: HashMap<String, Paladin>, flow, config}` with `Maneuver::new(name, agents: Vec<Paladin>, flow: &str) -> Result<Self, ManeuverError>`, `ManeuverConfig {timeout_seconds, error_strategy, pass_output_as_input}`, `ManeuverExecutionService::{execute, execute_expression}`, `ManeuverResult {final_output, step_outputs, execution_order}`, `CommanderBuilder::flow(&str)`, `FlowVisualizer::{to_ascii, to_mermaid}`.
- The DOC's `ManeuverConfig` and `ManeuverResult` are narrower than the PRD's (no agent_timeout_seconds, output_format, collect_timing_metrics, capture_intermediate_outputs, timing_metrics). PRD wins by precedence.
- CLI form recorded here: `paladin battalion run --type maneuver --flow "researcher -> writer, editor -> reviewer"`.

## Topic: Epic 17.5 — CLI consolidation refactoring (de-facto architecture decision)
- source: /workspace/.project/Milestone_2-Missing_features/Epic_17.5/epic17-5.md
- Records that two parallel CLI module trees existed: `src/cli` (from Epic 10 / Armory CLI Tools — commands for Agent, Battalion, Arsenal, Maneuver; config loading incl. `paladin_config.rs` and `battalion_config.rs`; output error handling and formatting; templates; `interactive.rs`; `user_commands.rs`) and `src/application/cli` (from Epic 18 — Onboarding, SetupCheck, Features, Muster, Council commands; `error.rs`; rich formatters; wizard framework; environment templates).
- Problems recorded: duplicate `commands/`, `templates/` and interactive utilities; inconsistent import paths (`paladin::cli::` vs `paladin::application::cli::`); DRY violation between `formatter.rs` and `output.rs`; unclear place to add new CLI features.
- Decision recorded (rationale section, no ADR status field): consolidate on `src/application/cli` because "CLI is an **input adapter** in the application layer, not infrastructure", Epic 18's structure has richer formatters, wizard framework and error handling, and the application-layer structure is more maintainable.
- Target layout given in full: `src/application/cli/{mod.rs, error.rs, commands/, config/, formatters/, interactive/, templates/, user_commands.rs}` with agent/battalion/arsenal/maneuver commands and loader/paladin_config/battalion_config moved in from `src/cli`.
- Migration steps: move command files, keep Epic 18 formatters and delete `formatter.rs`, consolidate on `error.rs`, move config loading, rewrite all `use paladin::cli::` imports to `use paladin::application::cli::`, update `paladin-cli.rs`, delete `src/cli` entirely, then run tests/clippy/fmt. `lib.rs` loses `pub mod cli;`.
- This is the only architecture-level module-ownership decision in the run-2 corpus and it is recorded at DOC precedence with no ADR — flagged as an ADR candidate in INGEST-CONFLICTS.md.

## Topic: Epic 18 definition — CLI Enhancement & Polish
- source: /workspace/.project/Milestone_2-Missing_features/Epic_18/epic18.md
- Theme "Developer Experience", 2 weeks, Medium, depends on Epics 11-17.
- Six user stories: onboarding wizard, setup check, features discovery, muster, council command, rich CLI output.
- Contains worked terminal transcripts for `paladin onboarding`, `paladin setup-check`, `paladin features`, `paladin muster --task "..."`, `paladin council --topic ... --participants 3 --max-rounds 5` and a battalion execution summary box.
- The `paladin features` transcript is the canonical list of the eight orchestration patterns: Formation (sequential), Phalanx (parallel), Campaign (DAG/graph), ChainOfCommand (hierarchical delegation), Conclave (expert synthesis / MoA), Council (group discussion), Grove (tree-based routing), Maneuver (Flow DSL); and the memory systems: Garrison short-term (in-memory, SQLite), Sanctum long-term (Qdrant, in-memory).
- Libraries named for rich output: indicatif (progress), console/colored (colours), comfy-table (tables).

## Topic: Epic 19 definition — Herald & Domain Type Consolidation
- source: /workspace/.project/Milestone_3-Completion/Epic_19/epic19.md
- Theme "Replace placeholder domain types, harden Herald system", 1-2 weeks, Critical, no dependencies, origin Epic 8 inline TODOs.
- Two user stories with exact source lines: herald.rs 147 (placeholder PaladinResult), 158 (placeholder BattalionResult), 169 (incomplete StreamChunk), 178 (incomplete ExecutionMetadata), 187 (placeholder PaladinError); herald_registry.rs 186 (auto-register built-in formatters).
- States that all real domain types now exist (`PaladinResult`, `BattalionResult`, `PaladinError`) so the placeholders can be removed. Locates `BattalionResult` in `battalion/` (the PRD narrows this to `battalion/mod.rs`).
- Epic 19 is declared a hard prerequisite by Epics 20, 21 and 22.

## Topic: Epic 20 definition — Vision Pipeline Completion
- source: /workspace/.project/Milestone_3-Completion/Epic_20/epic20.md
- Theme "Complete multi-modal vision API integration", 1-2 weeks, High, depends on Epic 19, origin Epic 13 inline TODOs.
- Records that Epic 13 delivered the vision type system and adapter scaffolding but left the actual API calls and execution-service integration as TODO stubs at `openai_vision.rs:212`, `anthropic_vision.rs:220` and `paladin_execution_service.rs:371`.
- Three user stories: OpenAI vision API call, Anthropic vision API call, vision execution in PaladinExecutionService.
- Completion criteria: both adapters make real HTTP calls, vision execution fully implemented, all tests pass with mocked responses, clippy and fmt clean.

## Topic: Epic 21 definition — Autonomous Agent Completion
- source: /workspace/.project/Milestone_3-Completion/Epic_21/epic21.md
- Theme "Complete handoff execution, planning integration, and PaladinResult enhancements", 2 weeks, Critical, depends on Epic 19, origin Epic 14 with 23 deferred task items.
- Five user stories with per-story deferred-task provenance: US-21.1 handoff execution (Epic 14 tasks 5.17, 5.18, 5.27-5.28, 6.9-6.14, 6.17-6.22), US-21.2 handoff tool auto-registration (6.5, 6.6), US-21.3 PaladinResult metadata (5.23-5.24, 7.11-7.14), US-21.4 orchestration (7.15-7.20), US-21.5 configurable model (planning_service.rs 128/305/426/538, prompt_generation_service.rs 146).
- The description cross-references `project/Milestone_2-Missing_features/Epic_14/epic14.mdd` — a filename typo for `epic14.md`, in an unclosed parenthetical.

## Topic: Epic 22 definition — Battalion & Commander Hardening
- source: /workspace/.project/Milestone_3-Completion/Epic_22/epic22.md
- Theme "Complete Battalion integration gaps, fix Commander delegation", 2 weeks, High, depends on Epic 19, origin Epics 4, 5, 15, 16.
- Five integration gaps named: Council and Grove store Paladin IDs but cannot resolve them to instances; Phalanx does not track per-paladin timing; Grove LLM-based routing is stubbed out; Commander metadata export is unimplemented; six Commander tests are ignored.
- Source lines: `council_service.rs:160`, `commander.rs:562` and `:617` (registry), `grove_service.rs:475` (LLM routing), `phalanx_service.rs:270` (timing), `commander.rs:1850, 1875, 2017, 2025, 2033, 2041` (ignored tests). Deferred tasks: Epic 5 task 5.10 and 5.14 (metadata export).
- The DOC states Grove "falls back to keyword matching if LLM call fails" unconditionally; the PRD makes this configurable (keyword or error). PRD wins by precedence.

## Topic: Epic 23 definition — CLI, Config & Infrastructure Completion
- source: /workspace/.project/Milestone_3-Completion/Epic_23/epic23.md
- Theme "Wire remaining CLI configuration, complete infrastructure stubs", 1-2 weeks, Medium, depends on Epics 19-22, origin Epics 9, 10, 18.
- Five user stories: CLI garrison configuration (`agent.rs:293`, Epic 9 task 5.8), CLI arsenal/MCP configuration (`agent.rs:296`, Epic 9 task 5.9), CLI integration tests with mock provider (Epic 10 tasks 13.4-13.6), CLI end-to-end and environment testing (Epic 18 tasks 9.1-9.7), API content deliverer scheduler integration (`api_content_deliverer.rs:297`).
- The user stories refer to the affected command as `muster` while the source file is `commands/agent.rs`.

## Topic: Implementation status — Epic 23 (CLI, Config & Infrastructure)
- source: /workspace/.project/Milestone_3-Completion/Epic_23/EPIC_23_COMPLETION_SUMMARY.md
- Self-reported status COMPLETE, branch `feature/epic-23-cli-config-infrastructure-completion`, dated February 14, 2026. All 5 user stories reported complete with every acceptance criterion met.
- Delivered per the summary: garrison config parsing (in_memory, sqlite) with 9 unit tests; arsenal MCP config parsing (stdio, sse) with tool registration and 8 unit tests; `MockLlmAdapter` (`tests/helpers/mock_llm_adapter.rs`, 264 lines, commit 981f026); `MockArsenalPort` (`tests/helpers/mock_arsenal_adapter.rs`, 372 lines, 9 tests); `MockPaladinPort` for Battalion testing; `SchedulerPort` (`src/application/ports/output/scheduler_port.rs`) exposing `schedule_job()`, `cancel_job()`, `list_jobs()`, `get_job_info()` with JobId/JobSpec/JobInfo/JobStatus and SchedulerError; `TokioCronSchedulerAdapter` (`src/infrastructure/adapters/scheduling/tokio_cron_adapter.rs`) on **tokio-cron-scheduler v0.13** with 13 inline tests; `SchedulerConfig {enabled, default_cron, channel_size}` in application_settings.
- Test tally: 84 Tier-1 CLI tests (9 garrison, 8 arsenal, 21 scheduler, 14 error handling, 6 Paladin execution, 4 Formation, 5 Phalanx, 8 tool integration, 9 mock infrastructure), all passing in under 5 seconds with no external dependencies; 6 Docker-gated Tier-2 tests; 5 API-key-gated Tier-3 tests. Overall `cargo test` reported at 1,674 tests passing, clippy clean, `cargo audit` clean.
- New test suites: `tests/cli/{garrison_config_test.rs, arsenal_config_test.rs, error_handling_test.rs, paladin_execution_test.rs, formation_execution_test.rs, phalanx_execution_test.rs, tool_integration_test.rs (529 lines)}`, `tests/unit/scheduler_tests.rs`, `tests/integration/scheduler_integration_test.rs`.
- Documentation: `docs/cli/CONFIGURATION.md` (500+ lines), updates to `docs/cli/TESTING.md` and `docs/CLI_USAGE.md`; example configs `paladin_with_garrison.yaml`, `paladin_with_arsenal.yaml`, `paladin_full_config.yaml`.
- Modified for wiring: `src/application/cli/commands/agent.rs` (TODOs at 293 and 296 removed), `config/paladin_config.rs`, `config/loader.rs`, `error.rs` (GarrisonConfigError, ArsenalConfigError), `api_content_deliverer.rs` (line 297 stub removed), `application_settings.rs`.
- Known limitations recorded: MCP WebSocket transport not implemented; garrison semantic search / vector context retrieval deferred (recency-based selection only); Tier 2 and 3 tests need manual execution; real MCP server integrations tested manually; CLI streaming output not implemented; no custom MCP server examples.
- The "Related Work / Dependencies" section mislabels Epics 19-22 (calls Epic 19 "Battalion orchestration patterns", Epic 20 "Commander and Grove routing", Epic 22 "Maneuver flow-based orchestration") and attributes deferred garrison semantic search to "Epic 24". See INGEST-CONFLICTS.md.

## Topic: Epic 24 definition — Test Hardening, Benchmarks & QA
- source: /workspace/.project/Milestone_3-Completion/Epic_24/epic24.md
- Theme "Enable deferred tests, fix benchmarks, final quality assurance", 1-2 weeks, High, depends on all preceding epics.
- Eight user stories with source lines: benchmarks (`battalion_benchmarks.rs` 297/390/950), prompt generation tests (`tests/unit/mod.rs:22`), timeout tests (`paladin_execution_service_test.rs` 237/239), Qdrant integration tests (`rag_integration_tests.rs:147`, `qdrant_sanctum_test.rs:62`), deferred coverage (user_service 4.23%, listener_service 57.83%), CLI snapshot testing plus docs (Epic 18 tasks 7.10-7.14, 8.11, 8.13, 8.14), provider live API tests (Epic 6 task 7.0), final documentation and demo assets (Epic 16 tasks 11.23-11.24, Epic 18 tasks 9.16 and 7.17).

## Topic: Milestone 3 release notes — feature narrative and measured state
- source: /workspace/.project/Milestone_3-Completion/RELEASE_NOTES_MILESTONE_3.md
- Declares version 0.3.0, February 2026, codename "The Grand Assembly". Highlights: four new Battalion patterns (Conclave, Council, Grove, Maneuver), 218+ Battalion tests, CLI snapshot testing, live API integration tests, Battalion benchmarks with <10ms overhead.
- **This document assigns Milestone 3 epic numbers to entirely different features than the Milestone 3 project plan and the epic19-24 definitions do**: "Epic 19: Conclave Pattern", "Epic 20: Council Pattern", "Epic 21: Grove Pattern", "Epic 22: Maneuver Pattern (Flow DSL)", "Epic 23: Commander Enhancement", "Epic 24: Test Hardening, Benchmarks & QA". Only Epic 24 agrees. See INGEST-CONFLICTS.md — this is the highest-impact conflict in run 2.
- API surfaces recorded here that differ from the Epic 16/17 PRDs: `CouncilBuilder::participants(3)` (count, not `add_participant`), `TerminationCondition::MaxRounds(3)` (tuple variant), `council_service.execute(&council, &experts, topic)` (not `convene`), `result.summary` (not `conclusion`), `GroveConfig {routing_strategy, confidence_threshold: 0.6}`, `RoutingStrategy::PerformanceBased` with "Dynamic Learning: Performance-based routing improves over time", `Maneuver::new(flow, paladins, config)`, and CLI `paladin maneuver validate|visualize` plus `paladin council "question" -n 5 --rounds 3`.
- Performance figures recorded: Battalion orchestration <10ms overhead for 100+ concurrent battalions; Garrison queries <50ms for a 1000-entry in-memory garrison; Herald formatting 0.0095ms for a 10KB result against a <1ms target.
- Measured coverage table: Battalion patterns 85 unit / 133 integration / ~80%; Council ~85%; Grove ~82%; Conclave ~87%; Maneuver ~83%; CLI 43 snapshot tests / 100%; **Overall 720 unit passing, 133 integration passing, ~78%**.
- Error strategies listed under Improvements as "FailFast, ContinueOnError, RetryThenContinue" while the Maneuver section lists "FailFast, ContinueParallel, IgnoreErrors".
- New dependencies recorded: insta 1.40, criterion 0.5; updated tokio 1.42, serde 1.0. No breaking changes, no deprecations.
- Migration guidance offered: Chain of Command with broadcast -> Council; Campaign with manual routing -> Grove.
- New documentation named: `docs/cli/TESTING.md`, `project/DEFERRED_COVERAGE.md`, `CONTRIBUTING.md`, `docs/MANEUVER.md`, `docs/COMMANDER.md`. New examples: conclave_expert_panel, council_discussion, grove_routing, maneuver_basic, maneuver_dynamic_flow, commander_council, commander_grove.
- Known issues recorded: CLI snapshot tests fail under `cargo llvm-cov` (workaround `cargo llvm-cov --lib`); user_service and listener_service low coverage.
- **"What's Next (Milestone 4)" lists "Sentinel Vision: Advanced vision capabilities and multi-modal processing" and "Autonomous Agents: Self-directed agents with planning and goal management" as planned, plus Grove semantic-similarity routing and RAG integration, and "Epic 28 (user_service) and Epic 29 (listener_service) coverage"** — this contradicts the Milestone 3 plan, which places Vision (Epic 20) and Autonomous (Epic 21) inside Milestone 3. See INGEST-CONFLICTS.md.

## Topic: Post-Epic 24 cleanup — legacy OpenAI adapter removal
- source: /workspace/.project/Milestone_3-Completion/Post-Epic_24-cleanup/LEGACY_CODE_CLEANUP_PLAN.md and LEGACY_CLEANUP_SUMMARY.md
- Dated 2026-04-08, branch `bugs/epic-24-post-fixes`. Investigation found two OpenAI adapters coexisting.
- Kept: `src/infrastructure/adapters/llm/openai_adapter.rs` exporting `OpenAIAdapter` + `OpenAIConfig`, used in 20+ locations including `tests/integration/llm_live_api_tests.rs`, `provider_factory.rs`, `openai_vision.rs` and several examples.
- Removed: `src/infrastructure/adapters/output/openai_llm_adapter.rs` exporting `OpenAILlmAdapter` + a duplicate `OpenAIConfig`, with zero real code usage (only println documentation strings in `examples/llm_provider_selection.rs` and stale lcov data). ~580+ lines of dead code deleted.
- Architectural note recorded: `adapters/output/` is a legacy adapter location from an earlier architecture; the current convention is that LLM adapters live in `adapters/llm/`. `output/api_content_deliverer.rs` remains in use.
- Follow-up edits: `output/mod.rs` export removed, `docs/HERALD.md` updated to `OpenAIAdapter::new(config)`, `examples/llm_provider_selection.rs` println examples updated, CHANGELOG removal notice added.
- Risk assessed as none; no breaking changes; `cargo check` clean afterwards.
- Recorded process lesson: time was wasted fixing `_index` -> `index` in the legacy adapter before discovering the tests used the current adapter.

## Topic: Post-Epic 24 cleanup — build and test fixes after adapter removal
- source: /workspace/.project/Milestone_3-Completion/Post-Epic_24-cleanup/BUILD_TEST_FIXES.md
- Dated 2026-04-08. Four issues fixed in `tests/integration/openai_content_analysis_integration_test.rs` and `src/infrastructure/adapters/llm/openai_adapter.rs`:
  1. Import updated from `infrastructure::adapters::output::openai_llm_adapter::{OpenAIConfig, OpenAILlmAdapter}` to `infrastructure::adapters::llm::openai_adapter::{OpenAIAdapter, OpenAIConfig}` (3 occurrences).
  2. `#[allow(dead_code)]` added to 5 deserialization-only fields (`OpenAIResponse.id`, `OpenAIChoice.index`, `OpenAIStreamChunk.id`, `OpenAIStreamChoice.index`, `OpenAIStreamDelta.role`).
  3. `OpenAIConfig::new()` takes a single `api_key: String` argument; the test was passing api_key plus base_url. The invalid-URL test was removed because `new()` accepts no URL.
  4. `get_provider_name()` returns lowercase `"openai"`; the test expected `"OpenAI"`.
- Result reported: `cargo check` clean, 3 passed / 2 ignored in the affected test file.

## Topic: Post-Epic 24 cleanup — live API test behaviour change
- source: /workspace/.project/Milestone_3-Completion/Post-Epic_24-cleanup/LIVE_API_TESTS_FIX.md
- Four problems recorded: (1) tests printed "SKIPPED" and returned early, counting as PASS; (2) an empty `DEEPSEEK_API_KEY=` returned `Ok("")` and the adapter then panicked; (3) `.env` was only loaded by `main.rs`, never in tests; (4) Anthropic tests sent a SystemPrompt where Anthropic requires user messages.
- Changes made to `tests/integration/llm_live_api_tests.rs`: added `init_test_env()` using `Once` to load `.env` once per test run; changed `require_api_key()` to **panic with a helpful message instead of returning a Result**; changed `create_test_prompt()` to use `UserPrompt` with a `query` field; removed all `match require_api_key()` handling.
- The document states the intended new behaviour explicitly: "Tests will now properly FAIL when keys are missing" and "Tests fail loudly with clear error messages". This reverses the graceful-skip requirement in the Epic 23 and Epic 24 PRDs — see INGEST-CONFLICTS.md.
- Documented invocation: `cargo test --features live-api-tests -- --ignored`; per-provider selectors `test_openai`, `test_deepseek`, `test_anthropic` (4 tests each).
- The body of this document also contains a plaintext OpenAI API key while explaining that a stray space had broken the key. The key value is deliberately NOT reproduced in any intel or report file. The user has confirmed the key is already rotated. Recommend redacting the source document.

## Topic: Post-Epic 24 cleanup — live API test results and adapter fixes
- source: /workspace/.project/Milestone_3-Completion/Post-Epic_24-cleanup/LIVE_API_TESTS_SUCCESS.md, SESSION_SUMMARY.md, QUICK_SUMMARY.md
- Dated 2025-01-26 in all three files (sibling cleanup docs on the same branch are dated 2026-04-08).
- Results: OpenAI 4/4 core tests passing (6/6 counting the 2 content-analysis tests), Anthropic 4/4 passing, DeepSeek 1/4 passing with three failures caused by insufficient API credits (HTTP 402), i.e. 9 passed / 3 failed across the 12 live tests. Unit tests reported at 1606 passing, 0 failed, 10 ignored.
- Five fixes recorded:
  1. OpenAI model assertion changed from exact equality to `model.starts_with("gpt-3.5-turbo")` because the API returns versioned identifiers such as `gpt-3.5-turbo-0125`.
  2. OpenAI streaming parse errors handled gracefully — log the chunk error and `continue` rather than panicking on JSON split across SSE network boundaries.
  3. Anthropic model changed from `claude-3-5-sonnet-20240620` (and 20241022, and `claude-3-opus-20240229`, all 404 for this key tier) to `claude-3-haiku-20240307`.
  4. Anthropic deserialization fixed in `src/infrastructure/adapters/llm/anthropic_adapter.rs`: `ClaudeResponse._id` -> `id` and `ClaudeContent._content_type` -> `content_type`, since serde ignores underscore-prefixed fields; `#[allow(dead_code)]` used instead.
  5. `provider_factory.rs` test made environment-agnostic so it passes whether or not `DEEPSEEK_API_KEY` is set.
- Recorded provider capability snapshots: OpenAI — streaming, tool calling, function calling, vision, embeddings, system messages all true, max context 128000. Anthropic — streaming, tool calling, vision, system messages true; function calling and embeddings false; max context 200000.
- Technical insights captured as best practices: SSE chunks may split mid-JSON so parse errors must be tolerated; never underscore-prefix serde fields; use prefix matching for model assertions; not all models are available to all API key tiers; environment-dependent tests must handle both states.
- Recommendations recorded: add content-analysis integration tests for Anthropic and DeepSeek to match OpenAI; document the graceful streaming pattern for all adapters; make test model selection configurable via `ANTHROPIC_TEST_MODEL` / `OPENAI_TEST_MODEL` / `DEEPSEEK_TEST_MODEL`; add DeepSeek credits or mock responses for CI.
- QUICK_SUMMARY.md headlines "ALL TESTS PASSING" and omits the three DeepSeek failures that its sibling documents record.

## Topic: Post-Epic 24 cleanup — Redis integration tests and DevContainer
- source: /workspace/.project/Milestone_3-Completion/Post-Epic_24-cleanup/REDIS_TEST_FIX.md
- Problem: Redis queue integration tests failed with `SocketNotFoundError(/var/run/docker.sock)` because they used `testcontainers`, which needs Docker socket access, while the DevContainer used a standalone Dockerfile with no socket mounted and no `REDIS_URL`.
- Fix in `tests/integration/redis_queue_integration_test.rs`: an environment-first `RedisSource` enum — `Existing { host, port }` parsed from `REDIS_URL`, otherwise `Testcontainer { container: Box<ContainerAsync<GenericImage>>, port }`. The large enum variant was boxed to satisfy clippy.
- DevContainer changed from `{"dockerFile": "Dockerfile.dev", runArgs, containerEnv}` to `{"dockerComposeFile": "docker-compose.yml", "service": "paladin-dev", "workspaceFolder": "/workspace"}`, giving the dev container Redis at `redis:6379`, MinIO at `minio:9000`, MySQL at `mysql:3306`, a shared network and docker-compose environment variables including `REDIS_URL=redis://redis:6379`.
- Alternative without rebuilding: `make test-integration-docker`.
- Commits: c4a1df7, 22951e8, fdf016e, 3ddcfc8 on `bugs/epic-24-post-fixes`; the DevContainer change was still pending commit when written.
- This document states "16 Redis queue integration tests were failing"; the CHANGELOG update for the same work states "Redis Queue Integration Tests: All 6 tests passing".

## Topic: Post-Epic 24 cleanup — CHANGELOG consolidation
- source: /workspace/.project/Milestone_3-Completion/Post-Epic_24-cleanup/CHANGELOG_UPDATE.md
- Dated 2026-04-08. Added a 177-line `## Milestone 3: Post-Epic 24 Completion & Test Hardening` section at line 30 of CHANGELOG.md, sitting between `## [Unreleased]` and `## Epic 23: CLI, Config & Infrastructure Completion`.
- Section contents: DevContainer docker-compose integration (Redis, MySQL, MinIO with health checks); Redis queue integration tests all 6 passing with FLUSHDB cleanup and testcontainers dependency removed; SQLite Garrison integration tests all 12 passing with per-test file databases; LLM provider integration tests modernised to `OpenAIAdapter`; dead-code warnings resolved; test cleanup (superfluous `vec!`, formatting); provider factory test made environment-agnostic; devcontainer.json reformatted and rust-analyzer settings fixed.
- Records 6 resolved technical-debt issues, "100% test success", zero warnings and **coverage statistics of 1,628 total tests**, CI-ready status confirmed.
- Documents all 9 commits on `bugs/epic-24-post-fixes`: 430d64d, 32780e6, f2f1307, c4a1df7, 22951e8, 0b09828, 0bb158e, fdf016e, 3ddcfc8.
- Concludes that Milestone 3 documentation is complete and the branch is ready to merge into `develop` and release.

## Topic: Reported test-count timeline across run-2 documents
- source: EPIC_11_COMPLETION_SUMMARY.md, epic-15-completion-report.md, EPIC_23_COMPLETION_SUMMARY.md, SESSION_SUMMARY.md, CHANGELOG_UPDATE.md, RELEASE_NOTES_MILESTONE_3.md
- Point-in-time totals as reported: 999 unit tests (Epic 11, Jan 30 2026) -> 1,292 unit + 168 doc + 560+ integration (Epic 15, Feb 2 2026) -> 1,674 total passing (Epic 23, Feb 14 2026) -> 1,606 unit passing / 1,628 total (post-Epic-24 cleanup) -> 720 unit + 133 integration passing (Milestone 3 release notes).
- These are recorded as reported figures only. The release-notes total is far below the Epic 23 figure from an earlier date; the discrepancy is logged in INGEST-CONFLICTS.md rather than reconciled here.

## Topic: Codebase-map anchors for run-2 features (authoritative on shipped code)
- source: /workspace/.planning/codebase/STACK.md, ARCHITECTURE.md, STRUCTURE.md, CONCERNS.md, TESTING.md, INTEGRATIONS.md
- The committed codebase map is authoritative on current shipped code and should be consulted instead of the planning documents whenever the two disagree.
- Shipped and mapped: Sanctum as a core domain container and a `paladin-memory` adapter family including Qdrant (`qdrant-client` 1.14, gated by the `qdrant` feature); `tokio-cron-scheduler` 0.13 behind a `scheduler` feature with a scheduler adapter in `paladin-storage`; Maneuver Flow DSL under `crates/paladin-battalion/src/maneuver/`; a `vision` feature with OpenAI and Anthropic multimodal support and a `vision_integration_test.rs`; Herald as its own `paladin-herald` crate; handoff error types in the core error module; RAG context injection as a step in the documented execution flow.
- `codebase/STRUCTURE.md` lists the Battalion crate contents as "Formation, Phalanx, Campaign, ChainOfCommand, Commander, Maneuver DSL" and does not name Conclave or Council; `codebase/CONCERNS.md` does name `crates/paladin-battalion/src/grove_service.rs`.
- `codebase/CONCERNS.md` records an open defect directly relevant to Epic 22: "Grove Service Model Hardcoded" — `grove_service.rs:537` hardcodes `model: "gpt-4".to_string()` with a TODO to make it configurable, so Grove routing ignores the configured LLM provider. This is the same class of defect Epic 21 US-21.5 removed from the planning and prompt-generation services.
- `codebase/CONCERNS.md` also records a single-threaded scheduler concern in `src/application/services/orchestration/scheduler.rs` and recommends leaning harder on `tokio_cron_scheduler`, which is already a dependency.

---

# Ingest run 3 of 5 — Milestones 4, 5 and 6 (19 DOCs consumed)

**Structural note before reading anything below.** These three milestones RESTRUCTURED what
Milestones 1-3 built: feature-flag expansion and port-trait hardening (M4), decomposition of the
monolith into a Cargo workspace (M5), and layer relocations (M6). Every module path recorded in
run-1 and run-2 context is therefore superseded, and several paths recorded *here* were superseded
again by milestones outside this run. The supersession is intended and is not a defect. Resolve
current locations through `.planning/intel/code-verification.md` and `.planning/codebase/*.md`.

**Milestone-numbering warning.** The M4 overview is titled "Milestone 1", the M5 overview is
titled "Milestone 2", and the M6 overview describes its prerequisites as "Completed in Milestones
1 and 2". These documents number by *refactoring tier* (Tier 1 = Milestone 4, Tier 2 = Milestone 5,
Tier 3 = Milestone 6), while the directory names and `task-completion-state.md` number by
milestone. Cross-references such as "hardened in Milestone 1 / Epic 2" mean **Milestone 4 Epic 2**.
See INGEST-CONFLICTS.md WARNINGS.

---

## Topic: Refactoring initiative — initial analysis and tiered recommendations
- source: /workspace/.project/Milestone_4-Refactor-Crates-Features/Milestone-Overview/Init-Analysis.md
- This is the origin document for Milestones 4, 5 and 6. It assesses the single-crate structure as "not wrong" and warns that "splitting too eagerly is a real danger in Rust" — Cargo workspaces cost longer cold compile times from linker overhead, more complex `Cargo.toml` maintenance, and circular-dependency risk. "The benefit has to exceed that friction."
- Reported size at authoring: roughly 102,000 lines across 200+ source files in one crate. LOC by area: `core/platform` 25k, `application/use_cases` 24k, `infrastructure/adapters` 23k, `application/cli` 12k, `application/ports` 6k, `core/base` 4k, `config` 4k, `infrastructure/repositories` 3k.
- Six specific friction points: (1) every change rebuilds everything; (2) only five feature flags exist (`redis-queue`, `s3-storage`, `openai-embeddings`, `qdrant`, `integration-tests`) so orchestration-only users still compile Redis, MinIO, notifications, web server and content processing; (3) the 12k-line CLI is embedded in the application layer and drags in `clap`, `dialoguer`, `indicatif`, `comfy-table`, `colored`; (4) the Battalion subsystem (~9k LOC domain + ~4k LOC services) is a standalone orchestration runtime with no hard dependency on content, notifications or web; (5) LLM adapters share only `LlmPort` and are fully swappable; (6) `src/lib.rs` glob-re-exports everything, making the entire internal module graph public by default.
- **Tier 1 (high value, low risk):** expand feature flags; harden port traits as the stable API contract ("a documentation and visibility discipline task, not a refactor"); move the CLI into a binary-only compilation unit.
- **Tier 2 (workspace split, when compile times become a bottleneck):** extraction priority is `paladin-core` first ("the safest possible move"), `paladin-battalion` second ("the primary public value"), `paladin-llm` as a single crate with feature-flagged providers rather than one crate per provider, `paladin-memory` for garrison + sanctum.
- **Tier 3 (architectural refinements):** `application_settings.rs` is too large (**3,172 LOC**); domain services are leaking into `core/platform/manager` (`notification_service.rs`, `queue_service.rs`, `orchestrator.rs`, `log_service.rs`) — "this is the one genuine architectural inconsistency"; the Maneuver DSL should be co-located with the battalion domain types; `CircuitBreaker` is an infrastructure concern, not a domain use case.
- Summary judgment ordering: feature flags -> CLI isolation -> split `application_settings.rs` -> move manager services -> extract `paladin-core` and `paladin-battalion` "when build times become a genuine friction point". "The full workspace decomposition shown in the diagram is the right long-term shape, but it's not urgent today."

## Topic: Milestone 4 (Tier 1) plan — feature flags, port hardening, CLI isolation
- source: /workspace/.project/Milestone_4-Refactor-Crates-Features/Milestone-Overview/Milesone-4-Tier-1-High-Value-Low-Risk.md
- Titled "Milestone 1: High-Value, Low-Risk Foundations", document version 1.0, dated 2026-04-14. Three Epics: feature-flag expansion (Critical, Large), port-trait hardening (High, Medium, parallel with Epic 1), CLI isolation (High, Medium, benefits from Epic 1). Schedule 3-5 sprints.
- Success criteria: all 1,487+ tests keep passing; an orchestration-only consumer no longer compiles Redis, MinIO, notification, web-server or content-processing dependencies; `paladin-cli` and its 8+ CLI-only dependencies are excluded from library compilation; port traits documented as the stable API with internals `pub(crate)`; zero clippy/fmt regressions; incremental build time for adapter changes down by >= 30%.
- Out of scope: workspace decomposition, `paladin-core`/`paladin-battalion` extraction (Tier 2), splitting `application_settings.rs`, moving manager services, relocating Maneuver DSL or `CircuitBreaker` (Tier 3).
- Risk register: feature-flag expansion introducing conditional-compilation bugs (Medium/High); downstream consumers depending on currently-public internal types (Medium/Medium); CLI separation breaking the binary entry point (Low/High); dead-code warnings from `cfg`-gated code (Low/Low).
- **Epic 1 acceptance criterion 1 requires MCP transport feature flags** — `mcp-stdio`, `mcp-sse` (or a combined `mcp-transports`) gating `MCPStdioAdapter` and `MCPSseAdapter` and their infrastructure dependencies — while explicitly keeping the Arsenal domain types, port traits and application services unconditional. Task 1.5 explains the distinction at length: gating the Arsenal itself "would require `#[cfg]` guards across every execution path in the framework for negligible dependency savings (the Arsenal core depends only on `serde`, `serde_json`, `uuid`, and `async-trait`)". Appendix B's proposed `[features]` block lists `mcp-transports = ["mcp-stdio", "mcp-sse"]`. **This was later eliminated** — see INGEST-CONFLICTS.md INFO.
- Appendix A records the current feature block: `default = ["redis-queue", "s3-storage", "openai-embeddings"]`, plus `redis-queue`, `s3-storage`, `openai-embeddings`, `qdrant`, `integration-tests`, `live-api-tests`.
- Appendix B's proposed post-milestone block: `default = ["llm-openai"]`; `web-server = ["actix-web"]` (actix only); `notifications = ["lettre"]`; `content-processing = ["pdf-extract", "scraper", "tiktoken-rs", "rss"]`; `vision = []`; `cli = ["clap", "dialoguer", "indicatif", "comfy-table", "colored", "console"]`; `full` enumerating everything including `cli`.
- Appendix C is a 14-row port trait inventory with a caveat: "Exact trait names and module paths to be confirmed during Task 2.1 audit." It names `SearchPort` and `RpcGatewayPort`, and `ContentIngestionPort` at `ports::input::content_ingestion_port`.
- Cross-epic deliverables: a single migration guide covering all three Epics, `CHANGELOG.md` entries per Epic, `README.md` feature table, `CONTRIBUTING.md` feature-flag testing requirements.

## Topic: Dependency classification matrix (Milestone 4 Epic 1)
- source: /workspace/.project/Milestone_4-Refactor-Crates-Features/Epic_1/dependency-matrix.md
- Task 1.1 deliverable, dated April 14 2026. Classifies every `Cargo.toml` entry as **core** (always compiled), **optional** (feature-gated) or **note**.
- Classified **core** and explicitly not gated: `reqwest` ("shared by all LLM providers; gating adapter modules is sufficient"), `serde`, `serde_json`, `tokio`, `tokio-util`, `async-trait`, `futures`, `uuid`, `chrono`, `thiserror`, `sqlx` ("used by garrison/sanctum/user repos"), `dotenv`, `env_logger`, `log`, `config`, `tracing-subscriber`, `once_cell`, `lazy_static`, `regex`, `url`, `urlencoding`, `base64`, `bytes`, `mime_guess`, `sha2`, `blake3`, `md5`, `murmur3`, `fasthash`, `argon2`, `lock_api`, `toml`, `rand`, `petgraph` ("graph data structures for Campaign battalion"), `tempfile`, `tokio-cron-scheduler`, `serde_yaml`.
- **Key finding that contradicts the Epic 1 PRD:** `chacha20poly1305` and `zeroize` are classified **core**, on the stated evidence that they "live in `src/infrastructure/security/encryption.rs` for *general* encryption (user auth, citadel). They are **not** vision-only and remain unconditional." The PRD gates both under `vision`. See INGEST-CONFLICTS.md WARNINGS.
- Classified **optional**: `redis` -> `redis-queue`; `rust-s3` -> `s3-storage`; `qdrant-client` -> `qdrant`; `actix-web` **and** `axum` -> `web-server` (axum is "used in `web/user_controller.rs`"); `lettre` **and** `handlebars` -> `notifications` (handlebars "**only** used in `email_notification_adapter.rs`"); `pdf-extract`, `scraper`, `tiktoken-rs`, `rss` -> `content-processing`; `clap`, `structopt`, `colored`, `comfy-table`, `indicatif`, `console`, `dialoguer` -> `cli` (deferred to Epic 3).
- Classified **note**: `testcontainers-modules` is in `[dependencies]` but "should be in `[dev-dependencies]`" — flagged as cleanup, out of scope.
- The feature-flag summary table still lists `mcp-arsenal` (gating `adapters/arsenal/*`, no dependencies because it is pure Rust) — this matrix predates the PRD's elimination note.
- Also notes `scraper` and `rss` are present in `Cargo.toml` with no `use` statements found in `src/`: "May be unused or intended for future content adapters. Still gated behind `content-processing` for correctness."

## Topic: Public API surface audit (Milestone 4 Epic 2)
- source: /workspace/.project/Milestone_4-Refactor-Crates-Features/Epic_2/api-audit.md
- Task 2.1 deliverable, dated 2026-04-15. Inventories the types exposed by the four glob re-exports (`pub use application::*; pub use config::*; pub use core::*; pub use infrastructure::*;`) — "~200+ types estimated".
- **Port traits (~25, always public, never feature-gated):** 19 output — `LlmPort`, `GarrisonPort`, `SanctumPort`, `EmbeddingPort`, `ArsenalPort`, `BattalionPort`, `CitadelPort`, `QueuePort`, `NotificationPort`, `FileStoragePort`, `PaladinPort`, `PaladinExecutorPort`, `PaladinRegistry`, `SchedulerPort`, `LogPort`, `ContentDeliveryPort`, `SearchEnginePort`, `VisionPort`, `VisionLlmPort`; 6 input — `ContentInputPort`, `DocumentPort`, `ListenerPort`, `MlPort`, `NlpPort`, `RpcPort`.
- **Domain entities (~50-60, public):** Paladin family, Battalion family (including `Council`, `Grove`, `Conclave`, `BattalionError`, `BattalionResult`), Garrison, Arsenal (`Armament`, `ArmamentCall`, `ArmamentResult`), Sanctum, Citadel, base types (`Node<T>`, `Collection`, `Field`, `Message`), Herald family (`StreamChunk`, `StreamChunkBuilder`, `ExecutionMetadata`, `ExecutionMetadataBuilder`), plus `Document`, `Content`, `Task`, `Job`, `Trigger`, `Workflow`, `Comment`, `Tag`.
- **Builders (~9):** `PaladinBuilder`, `PaladinConfigBuilder`, `CommanderBuilder`, `CouncilBuilder`, `GroveBuilder`, `MemoryBuilder`, `LogEntryBuilder`, `StreamChunkBuilder`, `ExecutionMetadataBuilder`.
- **To restrict (`pub(crate)`):** adapters (~50-80 types), repositories (~20-30), managers (~10-15: `Scheduler`, `QueueService`, `EventManager`), CLI (~50-100 types / 12,000 LOC), web server (~30-50).
- **Twelve ambiguous cases needing discussion**, with findings from an examples scan: `CircuitBreaker` is used in `vision_battalion.rs` and `vision_analysis.rs`, recommendation "make public as part of builder/execution utilities"; execution services (`PaladinExecutionService`, `FormationExecutionService`) are used directly in examples; adapters (`OpenAIAdapter`, `OpenAIConfig`, `InMemorySanctum`) are instantiated directly in examples, with the proposed resolution "mark adapters as `#[doc(hidden)]` but leave public"; vision types (`VisionContent`, `ImageDetail`, `VisionError`) are used directly and recommended public as domain entities.
- Target surface: **104-124 public types** against a current ~200+, i.e. a 40-50% reduction. Note this is materially looser than the PRD's `<= 50` target — see INGEST-CONFLICTS.md.
- Five open questions carried forward, including whether `Node<T>`/`Collection`/`Field`/`Message` should be public (recommendation: yes) and whether a `paladin::prelude::*` module is needed.

## Topic: Deprecation tracking (Milestone 4 Epic 2)
- source: /workspace/.project/Milestone_4-Refactor-Crates-Features/Epic_2/DEPRECATIONS.md
- Dated 2026-04-15. Declares a four-step version timeline: v0.1.0 all types via glob; **v0.2.0 (this Epic)** add deprecation warnings and begin curated exports; v0.3.0 remove deprecated exports and finalise the stable API; v1.0.0 stable public API guarantee.
- Three deprecation categories: immediate deprecation (clear migration path), soft deprecation (`#[doc(hidden)]` but still accessible), internal-only (change to `pub(crate)`, no deprecation needed).
- **Keeping public with no deprecation:** port traits (25), domain entities (50-60), builders (9), configuration (5-10), error types (15-20), plus four "practical utilities documented as stable" — `CircuitBreaker`, `PaladinExecutionService`, the Battalion execution services, and the vision types. Rationale: "these types have proven valuable in real usage (examples/tests) and don't expose unwanted implementation details."
- **Soft deprecation (`#[doc(hidden)]`, stay public, NO `#[deprecated]` annotation):** all adapter implementations — `OpenAIAdapter`, `AnthropicAdapter`, `DeepSeekAdapter`, `InMemoryGarrison`, `SqliteGarrison`, `QdrantSanctum`, `InMemorySanctum`, `RedisAdapter`, `MinioAdapter`. Migration path: prefer port traits; factory functions to be added in Epic 3; direct instantiation still works but undocumented.
- **Immediate deprecation:** manager services — "None identified yet"; repository implementations — change to `pub(crate)`, no deprecation needed, "these were never intended as public API"; internal utilities — "TBD based on usage analysis".
- **Internal-only, restrict without deprecating:** all CLI command handlers/formatters/utilities/progress bars/prompts; all web route handlers, middleware and WebSocket infrastructure (also feature-gated).
- Deprecation annotation template requires `since` and a `note` naming the alternative, linking migration docs and stating the removal version. Guidelines require testing that deprecation warnings compile and documenting all deprecations in `CHANGELOG.md`.
- **Current status as written: "Deprecated Items: 0 (none yet)", "Restricted Items: 0 (to be done in Task 6.0)", deprecation log empty.** Tasks 3.0 (adding warnings) and 6.0 (curating exports) marked in progress.
- Four open questions: confirm `#[doc(hidden)]` versus full deprecation for adapters; provide factory functions in v0.2.0 or wait for Epic 3; add a `paladin::prelude::*`; wait for the manager refactoring before deprecating manager types.

## Topic: Milestone 5 (Tier 2) plan — workspace decomposition
- source: /workspace/.project/Milestone_5-Workspace-Decomposition/overview/Milestone_5-Tier_2-Workspace-Decomposition.md
- Titled "Milestone 2: Workspace Decomposition", document version 1.0, dated 2026-04-14. Six Epics; schedule 5-8 sprints with Epics 3, 4 and 5 parallelisable. Extraction order: `paladin-core` -> `paladin-ports` -> `paladin-battalion` -> `paladin-llm` -> `paladin-memory` -> facade/CI/finalisation.
- Prerequisites declared complete in Milestone 4: expanded feature flags, hardened port traits with curated `lib.rs` exports, CLI isolated behind the `cli` feature, CI feature-flag matrix, `STABLE_API.md` published.
- Success criteria include: all 1,487+ tests pass via `cargo test --workspace`; **`paladin-core` "depends only on `serde`, `uuid`, `chrono`, and `thiserror`"** (four crates — its own Epic 1 acceptance criterion 5 lists six, adding `async-trait` and `serde_json`); a consumer can take `paladin-core` + `paladin-ports` + `paladin-battalion` with no LLM provider, storage backend, web server or notification dependency; per-crate incremental builds >= 50% faster than the monolith baseline.
- Risk register calls the `battalion/mod.rs` -> `application::ports::output::paladin_port::PaladinResult` upward dependency High likelihood / High impact, with the mitigation "Resolve during **Epic 2** by defining `BattalionResult` in `paladin-core` or introducing a shared result type in `paladin-ports`". Appendix B repeats this as two rows (`PaladinResult`, `RegistryError`). **Both the epic assignment and the type named were superseded**: the Epic 1 decision record resolved it in Epic 1 by moving `PaladinResult`, and `BattalionResult` was never the coupling.
- Also flags "Cross-crate type visibility breaks internal test compilation" as High likelihood, and accepts a cold-build regression if incremental improvement exceeds 50%.
- **Target workspace structure includes a `crates/paladin-cli/` member** ("CLI binary (already isolated in Milestone 1)", depending on the `paladin` facade). Appendix D's `Cargo.toml` template lists `"crates/paladin-cli"` in `members`. **The Epic 6 PRD explicitly reverses this** — see INGEST-CONFLICTS.md INFO.
- Appendix D also sets `[workspace.package] version = "0.2.0"` and **`edition = "2024"`**, contradicting the edition 2021 requirement in the Epic 1-4 PRDs. Shared `[workspace.dependencies]`: serde 1, serde_json 1.0, uuid 1.8, chrono 0.4, thiserror 2, async-trait 0.1, tokio 1 (full), log 0.4, reqwest 0.12 (json), futures 0.3, plus the five path references.
- Epic 3's rationale states "The execution service moves to `paladin-battalion`; **the parser primitives remain in `paladin-core`**" — deliberately superseded by Milestone 6 Epic 3, which moves them to `paladin-battalion`.
- Appendix C's crate dependency matrix: core depends on nothing; ports on core; battalion, llm and memory each on core + ports; the facade on all five; `paladin-cli` on the facade only.
- Epic 6 Task 6.4 requires `docs/MIGRATION_WORKSPACE.md`, README/CONTRIBUTING/STABLE_API updates and a CHANGELOG entry; Task 6.5 requires a retrospective and a Tier 3 readiness assessment. **The Epic 6 PRD drops both** as out of scope.

## Topic: Milestone 5 per-Epic definitions (Epics 2, 3, 4, 5) — verbatim extracts
- source: /workspace/.project/Milestone_5-Workspace-Decomposition/Epic_2/Milestone_5-Tier_2-Workspace-Decomposition-Epic_2.md, Epic_3/Milestone_5-Tier_2-Epic_3-Battalion_Extraction.md, Epic_4/Milestone_5-Epic_4.md, Epic_5/Milestone_5-Epic_5.md
- These four DOCs are **verbatim copies of the corresponding sections of the Milestone 5 overview** — identical objective, rationale, acceptance criteria and task text, with no independent content. They are recorded here once rather than four times.
- Consequence for planning: they carry the same superseded statements as the overview (notably Epic 3's "the parser primitives remain in `paladin-core`") and add no new requirement surface. Every position they state loses to the matching `prd-*.md` in the same directory under `ADR > SPEC > PRD > DOC`.
- Epic 2's task 2.1 description names `PaladinPort (with PaladinResult, StopReason)` and `LlmPort (… TokenUsage …)` as types to move into `paladin-ports`, which is the DOC-level echo of the competing position recorded as `REQ-port-value-type-ownership-v2`.
- Epic 5's acceptance criterion 5 leaves the token-counter gate open — "with `tiktoken` behind the existing `content-processing` or a new `tokenizer` feature" — which the Epic 5 PRD closes in favour of `content-processing`.

## Topic: Epic 1 decision record — resolving the core-to-application upward dependency
- source: /workspace/.project/Milestone_5-Workspace-Decomposition/Epic_1/decisions/battalion-result-upward-dependency-decision.md
- **This is the only decision record in the entire 263-document corpus, and its `-options.md` sibling makes it the only decision/options pair.** Header: "Decision Date: 2026-05-13", "Chosen Option: **Option A — Move Pure Value Types to `paladin-core`**", "Status: Approved — implementation sub-tasks appended to task list as 3.6a-3.6k".
- **What it actually settles: the LOCATION of five types.** `PaladinResult`, `StopReason`, `TokenUsage`, `RegistryError` and `HandoffError` move into `src/core/platform/container/` (their permanent home for the Epic; they travel to `paladin-core` in Task 5.0), and the application ports that currently define them become thin re-exports.
- **What it does NOT settle: it never mentions `BattalionResult` at all.** The run-1 `REQ-battalion-result-v1/-v2` competing field sets are untouched by this record. (Shipped code does settle them — see `code-verification.md`.)
- Rationale as recorded: all five types depend only on `serde` and `thiserror`, "exactly the deps allowed in `paladin-core`"; `TaskPlan` and `HandoffRecord` (referenced by `PaladinResult`) are already in core so no circular dependency arises; "application ports re-exporting from core is architecturally correct: the application layer depends on core, never the reverse"; zero breaking changes because all existing `paladin::application::ports::output::…` paths keep resolving.
- `PaladinError` is deliberately excluded because it carries `#[from] GarrisonError` and `GarrisonError` lives in `application::`. The consequence is that the convenience `pub use crate::application::use_cases::paladin::error::PaladinError` in `herald.rs` is **removed**, and callers must import `PaladinError` from `application::use_cases::paladin::error` directly.
- Rejected options as recorded: **Option B** (new parallel types `ExecutionOutcome`/`ExecutionStopReason`/`LlmTokenUsage` with `From` conversions at the boundary) — "introduces duplicate type hierarchies and large refactor scope, violating the 'structural refactor only' constraint"; **Option C** (defer to Epic 2 behind a `#[cfg(not(workspace))]` or cargo-feature shim) — "explicitly fails FR-16. Deferring creates a technical debt dependency between Epics."
- Confirmed implementation checklist: new `execution_result.rs`, `token_usage.rs`, `registry_error.rs` under `container/`; `src/application/errors/handoff_error.rs` moved to `container/arsenal/handoff_error.rs` because "`HandoffError` is an Arsenal-domain error type"; four application files reduced to `pub use` re-exports; import updates in `container/mod.rs`, `arsenal/mod.rs`, `battalion/mod.rs`, `battalion/conclave.rs`, `herald.rs` and `arsenal/handoff_tool.rs`.
- **Precedence status: manifest-typed DOC, `locked: false`.** It has an explicit `Status: Approved`, a `Chosen Option`, a decision date, a Rationale section and a Rejected Options section — it reads as a genuine ADR — but because it is typed DOC it creates **no locked decision** and is overridable by any PRD. It is the strongest ADR candidate in the corpus alongside `Epic_17.5/epic17-5.md`. See INGEST-CONFLICTS.md INFO.

## Topic: Epic 1 options analysis — the three upward-dependency strategies
- source: /workspace/.project/Milestone_5-Workspace-Decomposition/Epic_1/decisions/battalion-result-upward-dependency-options.md
- Companion to the decision record above. Status line reads "**Awaiting decision (Task 3.3)**", so on its own it would be a *Proposed* rather than Accepted ADR. Authored 2026-05-13 by "Automated analysis".
- Problem inventory — four files in `src/core/platform/container/` import from `application::`: `herald.rs` (`TokenUsage`, `PaladinResult`, `StopReason`, `PaladinError` — `pub use` re-exports only plus a test), `battalion/mod.rs` (`PaladinResult`, `StopReason`, `RegistryError` — actual type usage in structs and methods), `battalion/conclave.rs` (`PaladinResult` — actual usage), `arsenal/handoff_tool.rs` (`HandoffError` — actual usage).
- Explicitly scopes out `src/core/platform/manager/`: it "also imports from `application::` but is NOT in the extraction scope for this Epic. No changes required there." That deferred problem is exactly what Milestone 6 Epic 2 later solves.
- Type inventory with transitive dependencies: `PaladinResult` (serde; depends on `TaskPlan` and `HandoffRecord`, both already in core), `StopReason` (serde), `TokenUsage` (serde), `RegistryError` (thiserror), `HandoffError` (thiserror), `PaladinError` (thiserror; depends on `ArsenalError` in core **and** `GarrisonError` in application), `GarrisonError` (thiserror).
- Recorded cons of the recommended Option A: coordinating the core additions with the application re-export wiring in one PR to avoid a transient broken state; and the loss of `herald.rs`'s convenience `pub use PaladinError`, described as "a minor but concrete downstream change".
- Section 7 contains four open implementer-interview questions, confirming the decision was not final in this document: scope confirmation on type ownership changing; explicit acceptance of removing `pub use PaladinError`; whether the moved types should instead live in a dedicated `crates/paladin-core/src/types/` module; and an option-override path if A is rejected.

## Topic: Build-time benchmark report (Milestone 5 Epic 6)
- source: /workspace/.project/Milestone_5-Workspace-Decomposition/Epic_6/build-benchmarks.md
- Task 5.0 deliverable, dated 2026-05-21, branch `feature/milestone_5-epic_6-workspace-finalization`. Environment: Intel Xeon E3-1505M v5 @ 2.80GHz, 62 GiB RAM, Debian 12, rustc 1.93.1, `dev` profile. Workspace commit `e616059`; monolith baseline commit `08dc944` ("origin/main — last pre-decomposition commit").
- Describes the workspace as **6 crates** (`paladin-core`, `paladin-ports`, `paladin-llm`, `paladin-memory`, `paladin-battalion`, `paladin`) versus 1 in the monolith.
- Medians of three runs, workspace: clean build 257,492 ms; `paladin-core` incremental 14,029 ms; `paladin-llm` incremental 9,583 ms; `paladin-memory` incremental 8,618 ms; `paladin-battalion` only 1,571 ms (the first run of that scenario, 37,091 ms, was a cold start and is excluded).
- Medians, monolith baseline: clean build 275,681 ms; `src/lib.rs` incremental 17,302 ms. Baseline clean-build run 2 took 439,566 ms — roughly 60% longer than the other two — attributed to "a transient resource-contention event in the dev-container environment"; the median is used and the outlier does not affect it.
- Improvements: clean build −6.6% (target missed); core incremental −18.9% (missed); llm incremental −44.6% (missed); memory incremental −50.2% (met); battalion-only −90.9% (met).
- **Methodology caveat the report states about itself:** the monolith incremental was measured by touching `src/lib.rs`, "primarily a module-tree re-export file", so "Rust's incremental compilation pipeline detects no fingerprint changes in downstream modules and can skip the bulk of recompilation — producing a fast relink. This represents the *best-case* monolith incremental." Real changes deeper in the tree "would be significantly slower", making the comparisons "conservative estimates of the workspace advantage".
- Conclusion asserts "**Overall verdict: Target achieved**" while the summary table shows 2 of 5 scenarios meeting the >= 50% bar; the prose also states the clean-build shortfall twice with different numbers (−6.6% in the table, "(−5%)" in the conclusion). See INGEST-CONFLICTS.md WARNINGS.
- Three recommended follow-ups: scope the `.git/hooks/pre-commit` clippy invocation to `--workspace --all-targets --all-features -- -D warnings` "to prevent incremental-cache masking of workspace-wide lint errors in CI"; add an `sccache` layer in CI on top of the existing `cache-from: type=gha`; confirm `paladin-ports` stays a pure interface crate as the project grows.
- Also recommends strengthening the baseline in a future measurement by touching a mid-tree implementation file (e.g. `src/infrastructure/adapters/llm_adapter.rs`) instead of the crate root.

## Topic: Milestone 6 (Tier 3) plan — architectural refinements
- source: /workspace/.project/Milestone_6-Architectural-Refinements/overview/Milestone_6-Tier_3-Architectural-Refinements.md
- Document version 1.0, dated 2026-04-14. Four Epics: config decomposition (High, Medium), manager-service relocation (High, Large), Maneuver DSL co-location (Medium, Medium), `CircuitBreaker` relocation (Medium, Small). Schedule 3-5 sprints with Epics 2, 3 and 4 parallelisable.
- Framing: "these are refinements that correct genuine architectural violations in the hexagonal layering". With the workspace in place, "a misplaced type or a layering violation is caught at compile time by the crate dependency graph rather than relying on module-level discipline alone".
- Success criteria: 1,487+ tests keep passing; clippy/fmt/doc clean at `--workspace`; no new public API surface beyond `STABLE_API.md`; `paladin-core` contains zero imports from `application::` or `infrastructure::`, "verified by crate boundary enforcement"; `application_settings.rs` replaced by per-domain modules; developer onboarding time for config structure measurably reduced (qualitative).
- Risk register: config decomposition causing runtime config-loading regressions (Medium/High); manager relocation creating circular crate dependencies (Medium/High); "Maneuver DSL co-location decision conflicts with Milestone 5 crate boundaries" (Low/Medium) with the mitigation that moving the parser to battalion "is the correct direction"; `CircuitBreaker` relocation breaking `PaladinExecutionService` imports (Low/Low) mitigated by "facade crate re-exports absorb the change"; team velocity reduced by Milestone 5 fatigue (Medium/Medium).
- **Epic 1 acceptance criterion 1 specifies a single-directory config layout** — `config/{mod,agent,garrison,arsenal,notifications,queue,file_storage,web_server,llm,battalion,logging}.rs` — all inside `src/config/`, with no distribution into `paladin-memory` or `paladin-llm`. The Epic 1 PRD replaces this with a cross-crate map. Appendix B repeats the single-directory layout and adds `env_utils.rs` and `setup/`.
- Epic 1's rationale states `application_settings.rs` is **3,172 lines** and enumerates its contents: `Settings` (15+ optional fields), `QueueConfig`, `FileStorageConfig`, `NotificationConfig`, `MCPServerConfig`, `LlmProviderConfig`/`OpenAIConfig`/`AnthropicConfig`/`DeepSeekConfig`, `GarrisonConfig` references, `WebServerConfig`, `LoggingConfig` and "various helper methods for environment variable override logic".
- **Epic 2 targets four services, not six** — `notification_service.rs`, `queue_service.rs`, `orchestrator.rs`, `log_service.rs` — into `application/use_cases/{notifications,queue,orchestration,logging}/`, and Appendix A classifies `scheduler.rs`, `event_manager.rs`, `listener_service.rs`, `content_service.rs` and `user_service.rs` as "Borderline — evaluate", with scheduler and listener "Likely stays in core". Its acceptance criterion 6 requires "the facade crate re-exports maintain backward compatibility for any types that were publicly accessible".
- Epic 3 acceptance criteria 6 and 7 require "**All 113 Maneuver-related tests pass**" and "**All 32 Maneuver benchmarks pass**", and Task 3.4 requires moving the 57 parser tests and 21 maneuver domain tests out of `tests/unit/` into `paladin-battalion`, consolidating with 3 execution-service, 16 commander-integration and 12 visualization tests. The Epic 3 PRD contradicts all of this.
- Epic 4 Task 4.1 poses three target-location options: **A** `paladin/src/infrastructure/resilience/circuit_breaker.rs` ("simple, minimal disruption"), **B** a new `paladin-infra` crate for shared infrastructure utilities ("cleaner separation but introduces a new crate"), **C** inside `paladin-battalion` ("pragmatic but semantically imprecise"). Its acceptance criterion 5 requires "the facade crate re-exports `CircuitBreaker` at the original path for backward compatibility". The Epic 4 PRD selects Option A and reverses the re-export requirement.
- Cross-epic deliverables: an architecture compliance report verifying layer purity and no violations in the workspace dependency graph; documentation suite updates (`STABLE_API.md`, `README.md`, `CONTRIBUTING.md` with a "core vs application vs infrastructure decision tree", per-crate rustdoc, CHANGELOG); and a technical debt inventory including "candidate improvements for a future Refactoring Tier 4 (if warranted)".
- Appendix C fixes the post-Milestone-6 layer rules: **core** (`paladin-core`) holds domain entities, value objects, pure domain logic, domain events and "pure schedulers, event buses (if no port deps)", and must not hold port trait references, adapter-dispatching coordination, infrastructure concerns or configuration loading; **application** (`paladin-ports`, `paladin-battalion`, facade use-cases) holds port definitions, use-case services, the notification/queue/log coordinators, the general orchestrator and planning/prompt services; **infrastructure** (`paladin-llm`, `paladin-memory`, infra modules) holds concrete adapters, `CircuitBreaker`/retry/rate-limiting, HTTP clients, DB connections, delivery adapters and "configuration loading and env var handling".

## Topic: Milestone 6 per-Epic definitions (Epics 1, 2, 3, 4) — verbatim extracts
- source: /workspace/.project/Milestone_6-Architectural-Refinements/Epic_1/Milestone_6-Epic_1-Decompose_App_Settings.md, Epic_2/Milestone_6-Epic_2-Relocation_Manager_Layer_Orch_services.md, Epic_3/Milestone_6-Epic_3-Co-locate_Maneuver_DSL_w_Battalion.md, Epic_4/Milestone_6-Epic_4-Relocate_CircuitBreaker_Infra_Layer.md
- All four are **verbatim copies of the corresponding sections of the Milestone 6 overview**, with no independent content. Recorded once. Every position they carry — the single-directory config layout, the four-service relocation, the 113-test/32-benchmark Maneuver targets, the three `CircuitBreaker` location options and both facade-re-export requirements — is the DOC-level statement of a conflict that the matching `prd-*.md` resolves differently.
- Together with the four Milestone 5 epic DOCs, **9 of the 19 run-3 DOCs are duplicate extracts of two milestone overviews.** Treat them as provenance pointers, not as independent evidence, and do not double-count their acceptance criteria when building the roadmap.

## Topic: Epic 2 dependency analysis — manager-layer orchestration services
- source: /workspace/.project/Milestone_6-Architectural-Refinements/Epic_2/dependency-analysis.md
- Task 1.0 deliverable for `tasks-relocate-orchestration-services.md`. This is the evidence base that expanded Epic 2 from four services to six.
- **Six services to relocate** with targets: `notification_service.rs` -> `notification_orchestrator/`; `queue_service.rs` -> `queue_orchestrator/`; `log_service.rs` -> `log_orchestrator/`; and `orchestrator.rs`, `listener_service.rs`, `scheduler.rs` all -> `orchestration/`.
- **Staying in core:** `content_service.rs` ("no `paladin_ports` or `application::` imports — pure domain versioning service"), `event_manager.rs` ("pure in-process event bus"), `user_service.rs` ("already crosses boundaries (`application::storage`, `paladin_ports`) but deferred to future Epic; only import path update this Epic"), `admin/` and `user/` ("comment-only stubs"; the user stubs have "dead import paths").
- Per-file import listings. **Only `log_service.rs` has a direct port import** — `paladin_ports::output::log_port::{LogError, LogHealthCheck, LogPort, LogQuery, LogResult, LogStats}` — "confirms this is an application-layer coordination service". `notification_service.rs`, `queue_service.rs`, `listener_service.rs` and `scheduler.rs` are each recorded as **"Port dependencies: None"**, and `orchestrator.rs` as "None direct, but it imports all three sibling services (listener, queue, scheduler). Must move as a group." This contradicts the Epic 2 PRD's stated relocation reason for `notification_service.rs` — see INGEST-CONFLICTS.md INFO.
- Two critical re-exports to preserve: `queue_service.rs` does `pub use crate::core::platform::container::queue_config::QueueConfig` and `orchestrator.rs` does `pub use crate::core::platform::container::orchestration_context::OrchestrationContext` — the latter "must be preserved in new `orchestration/mod.rs`".
- A 32-row type classification table applying the PRD's §4.2 placement rules. Nine types are classified `paladin-core` eligible **and moved**: `NotificationServiceStats` and `NotificationServiceConfig` -> `container/notification.rs`; `QueueStats` -> `container/queue_config.rs`; `Schedule`, `ScheduledJob`, `ScheduledJobInfo`, `SchedulerStats` -> a new `container/schedule.rs`; `ListenerConfig` and `ListenerStats` -> `container/trigger.rs`.
- Two types are classified core-eligible but **deliberately kept in the application layer** with a stated reason: `LogServiceConfig` ("contains `MessageServiceConfig` and orchestration tuning fields; part of `LogOrchestrator` setup") and `ContentAnalysisType` ("tightly coupled to orchestration use-case semantics").
- `scheduler.rs` instantiates `ContentIndexingService`, `DataBackupService` and `EmailNotificationService` from `core::platform::container::task` — the concrete-task-service coupling that justifies classifying it application-layer.
- Import-path update map for five downstream consumers, plus a line-level map for `service_runner.rs` (lines 6, 7 and an inline `NotificationServiceConfig` construction at line 516).

## Topic: Codebase-map and code-verification anchors for run-3 features
- source: /workspace/.planning/intel/code-verification.md (run-3 section), /workspace/.planning/codebase/STRUCTURE.md, STACK.md, ARCHITECTURE.md, CONCERNS.md
- The workspace shipped and is **larger than run 3 describes**: 10 library crates plus `doc-examples`, not the 6 the M5/M6 overviews and `build-benchmarks.md` assume. `paladin-herald`, `paladin-storage`, `paladin-content`, `paladin-notifications` and `paladin-web` came from milestones outside this run.
- All four Milestone 6 relocations are verifiably complete in the tree, and `application_settings.rs`, `src/application/ports/` and `src/application/use_cases/` no longer exist. The orchestrator modules ship under `src/application/services/`, not `use_cases/`.
- The Epic 1 decision record's Option A shipped exactly as written: all five value types live at the paths it specified.
- Five run-3 statements are contradicted by shipped code and should not be planned as-is: the edition-2021 rule (the tree is mixed), the "exhaustive" `paladin-core` 6-dependency list (14 ship), the `vision`-gates-encryption claim (nothing is gated), the MCP transport feature flags (none exist) and the `paladin-cli` crate (none exists).
- Genuine remaining work found in run 3: the `api-surface` CI job's stale `project/current-exports.txt` baseline path after the `.project` rename; zero `#[deprecated]` annotations against M4 Epic 2 FR-8; `paladin-ports` doctests disabled with a named "Task 7.0" follow-up; three CLI-only dependencies still unconditional; and three competing `TokenUsage` definitions shipping simultaneously.
- `STABLE_API.md`, `docs/FEATURE_FLAGS.md`, `docs/MIGRATION.md` and `docs/CONFIGURATION.md` are **not missing** — they ship as mdbook pages under `docs/src/api-reference/` and `docs/src/getting-started/` after the Milestone 11 documentation overhaul. Do not plan them as gaps.

---

# Ingest run 4 of 5 — `.project/Milestone_7-Production-Hardening` + `.project/Milestone_8-Facade-Cleanup-Shim-Resolution`

29 DOC-typed documents. Topics are keyed by subject and attributed to source. Where a DOC records a
state claim, the tree reading is appended under `verified:` and the full evidence lives in
`code-verification.md` (run-4 section).

---

## Topic: Milestone 7 scope, structure and self-numbering
- source: /workspace/.project/Milestone_7-Production-Hardening/overview/Milestone_7-Tier_4-Production_Hardening.md
- **The document titles itself "Milestone 4: Production Hardening and Extended Workspace
  Decomposition" while its path is `Milestone_7-Production-Hardening`.** Its subtitle is
  "Tier 4 — Production Hardening, Extended Crate Extraction, and API Stabilization", and its
  Prerequisites section credits "Milestones 1-3" with work the directory numbering assigns to
  Milestones 4-6 (feature flags and CI matrix; core workspace crates; `application_settings.rs`
  decomposition; manager-service relocation; Maneuver DSL co-location; `CircuitBreaker`
  relocation). This is the **third instance** of the refactoring-tier-vs-directory numbering
  collision that the Roadmap Extension Protocol item 8 predicted. Directory numbering is
  authoritative here.
- Three themes: extended workspace decomposition (web, notifications, content, repositories into
  optional crates); production build infrastructure (Docker, CI/CD, Makefile, benchmarks,
  integration tests); API stabilization and pre-release preparation for "a 0.2.0 (or 1.0.0)
  release".
- Success criteria: the workspace contains all logically separable crates; Docker images build with
  per-crate layer caching; CI/CD supports per-crate testing, per-crate publishing and
  workspace-level release orchestration; every public crate has complete rustdoc, README, CHANGELOG
  and crates.io metadata; all benchmarks pass with a documented baseline; integration tests work
  with the workspace and Docker services; "a release candidate tag can be cut with confidence".
- Out of scope: new feature development; performance optimization beyond documenting baselines;
  **actual crates.io publishing** ("this Milestone prepares for it; publishing is a release
  decision"); Kubernetes manifests; GUI/dashboard.
- Risk register (5 rows): diminishing returns from extended extraction, mitigated by the Epic 1
  Task 1.1 cost-benefit gate; slower Docker workspace builds, mitigated by `cargo-chef` with a
  workspace recipe; per-crate versioning dependency hell, mitigated by starting lockstep;
  integration tests defeating isolation, mitigated by root-`tests/` placement; **crates.io name
  squatting on `paladin-*`, mitigated by "reserve crate names early with placeholder publishes;
  use `paladin-framework-*` prefix if needed"** — the risk that actually materialised, resolved
  instead by renaming to `paladin-ai-core` and `paladin-ai`.
- Appendix A target structure lists **`crates/paladin-cli/` (~12k LOC)** as a workspace crate, and
  Appendix B makes it publish Step 5. Run-3 verification already established that no `paladin-cli`
  crate exists — the CLI is a `cli` feature plus a `[[bin]] paladin-cli` target. Appendix A also
  omits `paladin-herald`, which did not yet exist.
- Appendix C versioning strategy: Phase 1 lockstep at `[workspace.package] version = "0.2.0"` with
  crates inheriting `version.workspace = true`; Phase 2 independent versioning gated on four
  criteria (two stable lockstep releases; evidence crates evolve at different rates; downstream
  requests for individual pinning; tooling such as `cargo-release` or `release-plz` in place).
- Appendix D maps six downstream consumer profiles to crate sets: Orchestration Only, Full
  Framework, Custom LLM Provider, Memory System Only, Content Pipeline, Web API + Orchestration.
- Schedule: Phase 1 Epic 1 (3-4 sprints) → Phase 2A Epic 2 / Phase 2B Epic 3 in parallel (2-3 and
  1-2) → Phase 3 Epic 4 (2-3). Total 5-8 sprints.

## Topic: Milestone 7 Epic definitions (verbatim extracts of the overview)
- source: /workspace/.project/Milestone_7-Production-Hardening/Epic_1/Milestone_7-Epic_1-Ext_Workspace_Decomp_Infra_Crates.md, Epic_2/Milestone_7-Epic_2-Production_Build_Infra_Adaptation.md, Epic_3/Milestone_7-Epic_3-Benchmark_Suite_Migration.md, Epic_4/Milestone_7-Epic_4-API_Stabilization_Pre-Release_Preparation.md
- All four Epic DOCs are **verbatim extracts** of the corresponding sections of the milestone
  overview — same objectives, background, acceptance criteria and task breakdowns, no additional
  content. Same pattern run 3 found for the Milestone 5 and 6 epic files.
- Epic 1 background sizing (used to justify the extractions): web server ~3-4k LOC plus the
  `actix-web` dependency tree; notification adapters ~2k LOC; content processing ~5-6k LOC "with
  heavy external dependencies"; SQL repositories ~3k LOC.
- Epic 1 AC 3 states the post-extraction facade is *"reduced to re-exports, the `ServiceRunner`
  composition root, and application-layer use-case services that don't warrant their own crate"* —
  the definition of the facade's role that Milestone 8 Epic 5 later writes into `src/lib.rs`.
- Epic 4 background: "The workspace now contains approximately 10 crates." Each needs correct
  `[package]` metadata, a crates.io README, a Keep-a-Changelog `CHANGELOG.md`, explicit
  `#![deny(missing_docs)]` **or** documented public API coverage, and semantic version alignment.
  Note the overview says `#![deny(missing_docs)]` in the background prose but `#![warn(missing_docs)]`
  in AC 7 and Task 4.4 — the PRD settles on `warn`.
- Epic 4 AC 1 specifies `license (MIT)`; the Task 5.7 checklist later records the policy as
  `MIT OR Apache-2.0`.

## Topic: Milestone 7 Epic 1 cost-benefit assessment — the extraction go/defer record
- source: /workspace/.project/Milestone_7-Production-Hardening/Epic_1/cost-benefit-assessment.md
- Measured on `feature/milestone_7` at 2026-05-25 HEAD. Methodology: `cargo tree -p paladin
  [--features <flag>] | wc -l` for transitive dep count; `git log --oneline --since="2025-01-01"`
  per source path for change frequency; `wc -l` for extraction LOC. **Baseline: 1,235 dep-tree
  lines** at default features, which already includes `sqlx` "because it is in
  `[workspace.dependencies]` without `optional = true` and the facade includes it unconditionally."
- Scoring thresholds: dependency weight HIGH ≥ 150 added lines, MEDIUM 50-149, LOW < 50; change
  frequency HIGH ≥ 20 commits, MEDIUM 10-19, LOW < 10.
- `paladin-storage` — 2,274 LOC across four files; 21 commits; HIGH selectivity; MEDIUM-HIGH
  complexity with one **prerequisite**: the repository port traits (`ContentRepository`,
  `ContentListRepository`, `MigrationManager`) live in `src/application/storage/sql_store.rs` and
  must move to `paladin-ports` first "before `paladin-storage` can implement them without creating
  an architectural inversion". **GO**, extracted first.
- `paladin-notifications` — 1,080 LOC; **+41** dep-tree lines from `lettre` + `handlebars`;
  9 commits; HIGH selectivity; LOW-MEDIUM complexity. **GO**, second. Justification notes deferral
  "would leave `lettre`'s OpenSSL linkage in any facade build even when SMTP is never used".
- `paladin-content` — ~5,000 LOC (largest); **+145**; **32 commits (highest of all four)**; HIGH
  selectivity; HIGH complexity for four stated reasons including that use-case services depend on
  `paladin-llm`, "creating an inter-crate dependency that must be handled carefully". **GO**, third.
- `paladin-web` — 1,595 LOC; **+210, the largest dep reduction**, from `actix-web` + `axum`;
  15 commits; HIGH selectivity; MEDIUM complexity. **GO**, fourth. Justification: "Web frameworks
  are infamous for deep dependency trees with C library bindings (OpenSSL, ring, h2, etc.)".
- **All four Go, zero Defer**, so PRD sub-tasks 1.4 (mark deferred entries) and 1.5 (create
  Milestone-8+ backlog tickets) were recorded N/A.
- Self-approval block: approved by "AI Agent (GitHub Copilot), acting as sole developer", 2026-05-25.

## Topic: Milestone 7 Epic 3 benchmark assessment — inventory, dispositions and shipped outcome
- source: /workspace/.project/Milestone_7-Production-Hardening/Epic_3/benchmark-assessment.md
- Starting state: "a fully root-owned benchmark layout". Root `dev-dependencies` already carried
  `criterion = { version = "0.5", features = ["async_tokio"] }`. `[[bench]]` stanzas for
  `battalion_benchmarks`, `herald_benchmarks`, `garrison_benchmarks` and `sanctum_benchmarks` were
  present **only as comments**; `paladin_benchmarks` and `arsenal_benchmarks` existed as
  `.rs.disabled` files. No crate-local `benches/` directories existed.
- Config-benchmark ownership finding closes PRD open question 1: `Settings` is defined in
  `src/config/settings.rs`, `impl Settings` there holds the current `Settings::new()` loading path,
  and no extracted crate owns the type — so the config benchmark belongs in the root crate "unless
  a later architectural change moves configuration ownership into a dedicated crate."
- **Final disposition of all five disabled benchmarks — none directly restored:**
  - `battalion_benchmarks.rs` — remove and replace. It "targets a broad root-level orchestration
    surface including Formation, Phalanx, Campaign, Chain of Command, Maneuver parsing, and
    visualization"; Epic 3's accepted scope is narrower (Formation/Phalanx/Campaign execution
    overhead only), so "a direct restore would preserve the wrong ownership and benchmark scope".
  - `garrison_benchmarks.rs` — remove and replace. Benchmarks a root-level `ConversationHistory`
    surface with broad mixed operations; Epic 3 requires crate-local read/write at 100/1000/10000.
  - `herald_benchmarks.rs` — **deprecate and remove.** "Herald formatting is not part of the
    confirmed critical-path benchmark set for Epic 3."
  - `paladin_benchmarks.rs.disabled` — deprecate and remove. Already documented as requiring
    `LlmPort` trait implementation work; benchmarks the old aggregate execution boundary.
  - `arsenal_benchmarks.rs.disabled` — deprecate and remove. Already documented as needing a
    rewrite around current armament domain types; Arsenal is not on the critical path.
- Final target/ownership map: `config_benchmarks` → root `paladin` crate; `battalion_benchmarks` →
  `paladin-battalion`; `sanctum_benchmarks` and `garrison_benchmarks` → `paladin-memory`;
  `llm_serialization_benchmarks` → `paladin-llm`.
- Sanctum migration detail: imports rewritten from the facade to
  `paladin_core::platform::container::sanctum::{MemoryBuilder, MemoryType, SanctumEntry}`,
  `paladin_memory::sanctum::InMemorySanctum`, and
  `paladin_ports::output::sanctum_port::{SanctumFilter, SanctumPort, SanctumQuery}`;
  `crates/paladin-memory/Cargo.toml` now owns the Criterion registration and a local `criterion`
  dev-dependency; the stale root placeholder was removed.
- CI regression signal as shipped: job `benchmark-regression-signal`, triggered on pull requests and
  manual dispatch, threshold "more than 3 Criterion `Performance has regressed.` notices in one
  run", covering `config_benchmarks`, `battalion_benchmarks`, `garrison_benchmarks` and
  `llm_serialization_benchmarks`, non-blocking via `continue-on-error: true` at job level.
- Environment notes: `cargo bench --workspace --no-run` is the structural compile-validation
  command; active benchmark targets are intentionally no-network and need no live provider
  credentials; `paladin-memory`'s optional `sqlite`/`qdrant` features are not used for benchmark
  compile validation; `paladin-llm`'s serialization benchmark uses shared request/response types.
- All six PRD success metrics are recorded **Satisfied** in the document's own status table.
- verified: all five benchmark files ship at exactly these locations; no `.disabled` benchmark file
  remains anywhere in the tree; `benchmark-regression-signal` is `ci.yml:531`.

## Topic: Milestone 7 Epic 4 release outcome — v0.1.0-rc.1 and the crates.io package renames
- source: /workspace/.project/Milestone_7-Production-Hardening/Epic_4/epic-4-completion-summary.md, release-readiness-audit-report.md, release-candidate-signoff-checklist.md, release-notes-v0.1.0-rc.1.md, post-release-verification-v0.1.0-rc.1.md
- **This is release history, not current state.** Tag `v0.1.0-rc.1` at commit `a9530fc`, dated
  2026-05-28. The project is now on `release/v0.7.0` with `Cargo.toml` at `0.6.0` and latest tag
  `v0.5.1`. Do not treat any rc.1 artifact as describing the current release posture.
- **Crate package renames driven by crates.io collisions** — the single most consequential outcome
  of Epic 4, and the reason the workspace's package names and library names diverge:
  - `paladin-core` package renamed to **`paladin-ai-core`**, "while preserving crate import
    compatibility via lib target name".
  - Root facade package renamed to **`paladin-ai`**, "preserving code imports with lib name
    `paladin`".
- Published crate set at 0.1.0 (all ten): `paladin-ai-core`, `paladin-ports`, `paladin-battalion`,
  `paladin-llm`, `paladin-memory`, `paladin-storage`, `paladin-notifications`, `paladin-content`,
  `paladin-web`, `paladin-ai`. Dependency-order publishing was executed in practice as
  `paladin-ai-core → paladin-ports → leaf crates → paladin-ai`.
- Four blockers closed from a prior NO-GO state: the `paladin-core` name collision; the
  `paladin-ports` packaged-verification failure (resolved after the dependency remap and publishing
  `paladin-ai-core` first); the root facade name conflict; and crates.io publish **rate limits**,
  "handled by retrying at the documented unlock times and continuing dependency-order publishing".
- Audit results table, all PASS: `cargo test --workspace` (after "stabilizing env-sensitive settings
  tests"); `cargo clippy --workspace -- -D warnings`; `cargo fmt --all -- --check`;
  `cargo doc --workspace --no-deps`; dry-run publish for all public crates; **`cargo audit` marked
  "PASS (policy-managed)"** with approved exceptions for RUSTSEC-2023-0071 and RUSTSEC-2025-0111;
  license compatibility; dependency-tree/binary-size review (informational).
- Nine Epic 4 acceptance criteria all recorded **Met** with traceability, including "7. Missing docs
  policy and >=90% coverage posture" and "8. STABLE_API per-crate stabilization tiers".
- Sign-off: **GO**, on the stated rationale that no Epic 4 release-readiness blockers remain.
- Post-release verification (2026-05-28): all ten crates resolve on docs.rs with HTTP 200 — note
  `https://docs.rs/paladin-ai-core/latest/paladin_core/` and
  `https://docs.rs/paladin-ai/latest/paladin/`, the package/lib name split made visible. A fresh
  external binary project with `paladin-ai = "0.1.0"` passed `cargo check` with the marker
  `SMOKE_OK: paladin-ai dependency resolved and compiled`. Annotated tag pushed at `a9530fc`.
- Key commits recorded: `a9530fc` (Epic 4 release closure), `cb9a5eb` (stabilize settings config
  tests), `957c3a4` (rename root package to paladin-ai), `687ca1f` (rename core package to
  paladin-ai-core), `8bbdea7` (resolve crates.io paladin-core collision).
- verified: root `Cargo.toml` declares `[package] name = "paladin-ai"` with `[lib] name = "paladin"`,
  and `paladin-core = { package = "paladin-ai-core", … }` in `[workspace.dependencies]`. Version is
  `0.6.0` throughout, not `0.1.0`.

## Topic: RustSec risk acceptance — two advisories, an owner and an expiry
- source: /workspace/.project/Milestone_7-Production-Hardening/Epic_4/rustsec-remediation-plan.md
- **Flagged prominently: these are live security risk acceptances, not closed findings.**
- `RUSTSEC-2023-0071` — `rsa 0.9.10`, Marvin timing side-channel. Path
  `rsa -> sqlx-mysql -> sqlx -> workspace crates`. **No fixed upgrade available.**
- `RUSTSEC-2025-0111` — `tokio-tar 0.3.1`, PAX header parsing enabling file smuggling. Path
  `tokio-tar -> testcontainers -> testcontainers-modules`. **No fixed upgrade available.**
- Hardening completed 2026-05-28: `testcontainers-modules` moved to `dev-dependencies`; MySQL
  repository compilation gated on `storage-mysql` in `src/infrastructure/repositories/mod.rs`;
  `sqlx` default features disabled at workspace level with required features listed explicitly.
  `cargo tree -i tokio-tar` then showed only dev-dependency paths through `testcontainers`.
- **`cargo audit` still reports both advisories** "because Cargo.lock includes dev/optional
  dependency graphs and no fixed upstream versions are available."
- Exception governance: `make audit` runs
  `cargo audit --ignore RUSTSEC-2023-0071 --ignore RUSTSEC-2025-0111`; the CI security job enforces
  the same command. **Owner: Platform Security (Milestone 7). Review/expiry target: 2026-09-30.**
- Open action items that the document does **not** mark complete: two tracked impact-analysis issues
  (one per advisory); `audit.toml` exception entries "only if approved, each with expiry date and
  owner"; and re-running `cargo audit` after mitigation with evidence attached to the Epic 4 report.
- Track A also requires confirming whether `sqlx-mysql` is needed in the default release path and
  "prefer SQLite-only release profile where acceptable", plus documenting that RSA exposure is
  dependency-only and that untrusted tar extraction is absent from production runtime paths.
- Track B: investigate `sqlx-mysql` alternatives; ensure published crates do not require the
  `tokio-tar` chain; subscribe to upstream advisory trackers for `rsa`, `sqlx`, `tokio-tar`,
  `testcontainers`.
- verified: `.cargo/audit.toml` now ignores **five** advisories (the two above plus
  RUSTSEC-2026-0187 lopdf, RUSTSEC-2026-0194 and RUSTSEC-2026-0195 quick-xml); `deny.toml` mirrors
  only the original two plus six unmaintained notices; `ci.yml:406` still passes only the original
  two IDs on the command line. The three files disagree. See `INGEST-CONFLICTS.md`.

## Topic: License compatibility policy and sign-off
- source: /workspace/.project/Milestone_7-Production-Hardening/Epic_4/license-compatibility-decision-checklist.md
- Policy: **`MIT OR Apache-2.0`** (Rust-style dual licensing). Approval rule — MIT, Apache-2.0, or
  any SPDX expression containing a permissive MIT/Apache branch is acceptable by default;
  non-permissive or unresolved entries require explicit decision and sign-off.
- Inventory: 551 packages, 0 unknown entries after resolution. MPL-2.0 observed in `colored 2.2.0`
  and `colored 3.0.0`. `r-efi 5.3.0` carries `MIT OR Apache-2.0 OR LGPL-2.1-or-later`.
- `fuchsia-cprng 0.1.1` resolved from the crates.io artifact — the `.crate` includes a `LICENSE`
  file and `Cargo.toml` sets `license-file = "LICENSE"`; effective classification BSD-3-Clause-style
  permissive.
- Decisions recorded: MPL-2.0 **explicitly accepted for unmodified use**, so replacing the `colored`
  chain is N/A; `r-efi` accepted via its permissive branch. Approver **`DF3NDR` (repository owner)**,
  approval date **2026-05-28**. Task 5.7 status: **COMPLETE**.
- Sign-off evidence: transitive inventory built with `cargo metadata` + `jq`.

## Topic: Milestone 8 scope and the facade crate's permanent role
- source: /workspace/.project/Milestone_8-Facade-Cleanup-Shim-Resolution/overview/Milestone-8_Facade-Cleanup-Shim-Resolution.md
- Version target v0.2.0. Success criteria: every `src/` file has an explicit disposition; all dead
  re-export shims from Milestones 5-6 removed; `src/application/ports/` empty or deleted;
  **`src/application/notifications/` channel services moved to `paladin-notifications`**;
  `src/application/use_cases/` renamed to `services/`; the crate compiles with all workspace tests
  passing and no new warnings; and the facade's role documented as "application assembly and
  composition root."
- Parallel-execution context: M8 has **no dependencies** on M9-11 and can begin immediately. It is a
  prerequisite for M9 (needs a clean facade with a stable directory structure) and M11 (needs
  finalized paths and module names to document).
- Epic 4 **Task 4.3 "Add Backward-Compatible Re-Export (Optional)"**: "If any external consumer or
  example uses `paladin::application::use_cases::*`, add a temporary `pub use services as
  use_cases;` in `application/mod.rs` with a `#[deprecated]` attribute." The Epic 4 PRD explicitly
  rejects this. PRD outranks DOC.
- Epic 3 **Task 3.1** requires moving `email_notifications.rs`, `push_notifications.rs` and
  `system_notifications.rs` to `crates/paladin-notifications/src/` with facade re-exports added
  "if needed for backward compatibility". The Epic 2 PRD deletes all three as orphaned dead code.
- Schedule: Phase 1 Epic 1 audit (0.5-1 sprint) → Phase 2 Epics 2 and 3 in parallel (1 and 1-2) →
  Phase 3 Epic 4 rename (0.5) → Phase 4 Epic 5 finalize (0.5). Total 3-4 sprints.

## Topic: Milestones 8-11 dependency graph — preserve for run 5
- source: /workspace/.project/Milestone_8-Facade-Cleanup-Shim-Resolution/overview/Milestones-8-11_Dependency-Graph.md
- **Carried forward deliberately: this document spans milestones that arrive in ingest run 5.**
- Milestone summary: **M8** Facade Cleanup & Shim Resolution, v0.2.0, 3-4 sprints, **no hard
  dependencies**. **M9** Classic Orchestrator Completion, v0.3.0, 6-10 sprints, hard-depends on M8.
  **M10** CI Hardening & Release Automation, v0.4.0, 2-3 sprints, **no hard dependencies**.
  **M11** Documentation Overhaul & Publish, v0.5.0, 4-6 sprints, hard-depends on M8 and partially
  on M9.
- Edge semantics:
  - M8 → M9: **HARD**. "M9 work should not begin until M8 Epic 4 (rename) is complete" — otherwise
    the `services/` rename invalidates M9 import paths written concurrently.
  - M8 → M11: **HARD** (paths stable). M11 Epics 1-2 (Doc Audit, MDBook Setup) start immediately;
    **M11 Epics 3-4 wait for M9 Epics 1-3** to reach API stability.
  - M9 → M11: **HARD** on APIs. M11 Epics 3-4 use "document after merge — only write about APIs
    that are merged to `main`".
  - M8 → M10: **SOFT**, and only for M10 Epic 3 (release automation), which "waits for M8 to
    finalize crate structure" because it references crate names M8 might change. M10 Epics 1-2
    (Pre-commit, Security Scanning) start immediately.
- Epic inventory referenced: M8 E1 Audit / E2 Shim Removal / E3 Relocate / E4 Rename / E5 Finalize;
  M9 E1 Orchestrator E2E / E2 Queue-Scheduler / E3 Content→Agent / E4 Agent→Orchestrator /
  E5 User-Admin / E6 Finalize; M10 E1 Pre-commit / E2 Security Scanning / E3 Release Automation /
  E4 Finalize; M11 E1 Doc Audit / E2 MDBook Setup / E3 Rewrite / E4 New Docs / E5 Publish+Finalize.
- **Critical path: M8 (3-4) → M9 (6-10) → M11 Epics 3-5 (2-3) = 11-17 sprints.** M10 is entirely
  off the critical path. M11 Epics 1-2 are off it too — net addition 0 sprints.
- Release gates: v0.2.0 "all shims resolved, `services/` rename complete, quality gates green";
  v0.3.0 "orchestrator E2E, both bridges working, user/admin auth functional"; v0.4.0 "pre-commit
  hooks, security scanning, release automation operational"; v0.5.0 "MDBook published, all docs
  current, all code examples compile".
- **Ordering caveat recorded in the source:** v0.4.0 (M10) may ship before v0.3.0 (M9) since M10 has
  no dependency on M9 and is smaller, giving v0.2.0 → v0.4.0 → v0.3.0 → v0.5.0. "If this is
  unacceptable, hold M10's tag until M9 ships." **Alternative numbering if strict ordering is
  preferred: v0.2.0 (M8), v0.2.1 (M10 — minor CI tooling, no API changes), v0.3.0 (M9 — new
  features), v0.4.0 (M11 — docs)."
- Two-developer parallel track and a sprint-by-sprint single-developer schedule are both given.
- Five risks with mitigations, including "M9 scope creep delays all downstream milestones —
  M9 has a clear cut line: if User/Admin (Epic 5) isn't ready, ship v0.3.0 without it and defer to
  v0.3.1", and "Developer context-switching between milestones reduces velocity — batch milestone
  work in 2-sprint blocks; don't interleave within a single sprint."
- Status against `task-completion-state.md`: M9 is 100.0% complete (0 open across 6 task lists),
  M10 is 100.0% (0 open across 5), M11 is 92.0% (26 open, all in `tasks-content-rewrite.md`). The
  graph is therefore a **historical planning artefact**, not a live schedule — but its dependency
  semantics and release-gate criteria are what run 5 should connect M9-M11 to.

## Topic: Milestone 8 Epic definitions (verbatim extracts, plus two originals)
- source: /workspace/.project/Milestone_8-Facade-Cleanup-Shim-Resolution/Epic_1..Epic_5/Milestone_8-Epic_*.md
- Epics 1-5 DOCs are **verbatim extracts** of the corresponding overview sections. Epics 6 and 7
  are original documents with no overview counterpart — they were added after the milestone plan
  was written.
- Epic 5 Task 5.3 deliverable: "v0.2.0 release candidate ready".
- Epic 6 DOC adds a **Root Cause** section absent from the overview: "The Epic 4 rename scope was
  defined as the facade crate only (`src/`). The leaf crate `paladin-content` was not in scope for
  that Epic. As a result the facade's re-export bridge was updated to the new path, but
  `paladin-content` still publishes the old path, leaving the bridge broken under the
  `content-processing` feature flag."
- Epic 7 DOC carries a **"Decisions (from PRD clarification)"** section: port the three endpoints to
  axum **and mount them** (revive the API) rather than deleting them; add `actix-web` to
  `deny.toml`'s banned crates. It also states the workflow used — "run `/generate-tasks` against
  that PRD to produce the implementation checklist".

## Topic: The facade crate audit — 189 files, three lists
- source: /workspace/.project/Milestone_8-Facade-Cleanup-Shim-Resolution/Epic_1/facade-audit.md
- Status Complete, created and completed 2026-05-29. **189 files audited: 151 stay (List C),
  13 move (List B), 25 delete (List A).**
- List B by target: `paladin-notifications` 3, `paladin-content` 2, `paladin-storage` 6,
  `paladin-memory` 1, `paladin-web` 2 ("wait for Epic 3 co-ordination — `paladin-web` already holds
  some of these"). ~8,564 lines to relocate.
- Key findings:
  - `src/application/ports/` **does not exist** — already removed prior to Milestone 8.
  - `src/application/notifications/` is not declared in `src/application/mod.rs`; all three files
    are dead code.
  - `src/core/platform/manager/admin/` and `user/` are neither declared in `manager/mod.rs`; all
    files unreachable.
  - Most `lib.rs` `pub use` short-form aliases have **0** internal consumers — workspace code uses
    full paths. Only `MockLlmAdapter`, `OpenAIAdapter`, `AnthropicAdapter` and `DeepSeekAdapter`
    have 13 consumers via short paths.
  - **PRD correction:** `planning_error.rs` and `prompt_error.rs` under `src/application/errors/`
    "are real application-service error types with tests — they are NOT shims and STAY." Only
    `citadel_error.rs` and `handoff_error.rs` are shims.
- List A totals ~1,471 lines of dead/comment-only code across 25 files, with three cascade `mod.rs`
  deletions noted (`subject/`, `admin/`, `user/`).
- List B file sizes: `email_notification_adapter.rs` 752, `system_notification_adapter.rs` 320,
  `document_adapter.rs` 480, `pdf_extractor.rs` 350, `minio.rs` 1,198, `redis.rs` 1,570,
  `file_content_repository.rs` 723, `mysql_content_repository.rs` 780,
  `sqlite_content_repository.rs` 810, `sqlite_user_repository.rs` 676, `file_citadel.rs` 581,
  `api_content_deliverer.rs` 724, `user_controller.rs` 870.
- **This document is explicitly superseded** by `facade-cleanup-RECONCILIATION-2026-06-04.md`,
  which states its List B and the Epic 3 disposition "contain factual errors" — specifically that
  ~4,400 LOC described as "active bridges that stay" are in fact orphaned uncompiled duplicates.

## Topic: Infrastructure adapter disposition record — Epic 3's 20-row table
- source: /workspace/.project/Milestone_8-Facade-Cleanup-Shim-Resolution/Epic_3/infrastructure-adapter-disposition.md
- Twenty adapter groups, **every one recorded "Stays"** in Epic 3, with the sole action taken being
  the `tensorflow_adapter.rs` `ml` gate plus the storage-shim deletions and their six consumer
  updates.
- M9 candidates split into two lists. **List A — "Leaf crate already owns the domain, adapter can be
  inlined":** the five `input/*` content fetchers → `paladin-content`; `arsenal/` (all 5 adapters) →
  future `paladin-arsenal`. **List B — "Leaf crate needs to be created or extended first":**
  `citadel/file_citadel.rs` → `paladin-memory`; `document/` → `paladin-content`;
  `file_storage/minio.rs` → `paladin-storage`; `output/api_content_deliverer.rs` → `paladin-web`;
  `queue/redis.rs` → `paladin-storage`; `input/tensorflow_adapter.rs` → future `paladin-ml`.
- No-extraction-target groups (facade-level concerns): `herald/` ("cross-cutting output
  formatting"), `logs/` ("cross-cutting logging infrastructure"), `paladin_registry.rs` ("facade
  orchestration registry"), `scheduling/` ("single concrete scheduler"), `llm/config_bridge.rs`
  ("config mapping stays in facade").
- The record **disagrees with its own governing PRD** on two rows: `arsenal/` is flagged an M9
  candidate here but "No" in the PRD table; `sanctum/` targets a future `paladin-sanctum` here but
  `paladin-memory` in the PRD.
- It also describes as "Active bridge" several files the reconciliation later proves **orphaned and
  uncompiled** — `document/`, the `input/*` fetchers, `output/api_content_deliverer.rs`.
- Dated `2025-01`, which is inconsistent with every other Milestone 8 document (2026-05/06).
- **Explicitly superseded** by `facade-cleanup-RECONCILIATION-2026-06-04.md`.

## Topic: The 2026-06-04 facade-cleanup reconciliation — what actually shipped
- source: /workspace/.project/Milestone_8-Facade-Cleanup-Shim-Resolution/facade-cleanup-RECONCILIATION-2026-06-04.md
- **The highest-value document in run 4.** A fresh independent file-by-file audit of `src/` (159
  files at the time) reconciling recorded status against verified status.
- Reconciled Epic status: **E1 done but with errors** (needs correction); **E2 DONE** (all 25 List A
  files gone; `notifications/`, `storage/` stubs, `subject/`, `admin/`, `user/` dirs confirmed
  deleted); **E3 PUNTED** ("disposition kept everything in facade, deferred to M9. Only tensorflow
  gating + 3 storage-shim deletions were done. This is the remaining work."); **E4 DONE** (tree has
  `src/application/services/`); **E5 largely done** (`lib.rs` carries facade/composition-root docs);
  **E6 "Not verified; low priority"**.
- "Bottom line: the deletions and the rename landed; the *relocations never happened*, and the audit
  that was supposed to drive them mislabeled the easy wins. Finishing Milestone 8 ≈ finishing
  Epic 3 correctly."
- Verification method for the orphan claim: "`rg \"mod <name>\"` across `src/` returns nothing for
  each; the `mod.rs` in each directory only does `pub use paladin_<crate>::…`; the leaf crate file
  exists." Conclusion: "They are not bridges; they are dead corpses left behind when the real code
  was copied into leaf crates."
- Five risk-ordered categories: **C1** orphaned dead files, delete outright, ~4,465 LOC, zero risk;
  **C2** compiled but redundant, ~2,627 LOC, low risk; **C3** genuine relocations — the un-done
  Epic 3, medium risk; **C4** judgment calls needing a decision; **C5** cross-cutting hygiene.
- C5 starting state: **435 `println!`/`eprintln!`/`dbg!` occurrences across 36 files**, "much of it
  legitimate CLI output (keep), but the orchestration/services hits should become `log::*`. Scope to
  `services/` + `infrastructure/`, not `cli/`." Plus ~7 stray `#[allow(dead_code)]` markers and
  stale `mod.rs` doc comments in `application/` and `infrastructure/`.
- Execution log, branch `chore/facade-cleanup-m8-finish`, 2026-06-04 → 06-05, each commit passing
  build + clippy `-D warnings` + fmt + tests: `e5b2011` (Tier 1 orphan deletion, ~4,465 LOC);
  `2edc031` (`file_content_repository`, 723 LOC); `3d48768` (deferred features — `user` CLI,
  tensorflow, `ml` flag — ~1,700 LOC); `ca7e4e8` (`HashMapPaladinRegistry` → `paladin-battalion`);
  `8bd7073` (`FileCitadel` → `paladin-memory`); `66f6c4e` (**Herald formatters → new
  `paladin-herald` crate**, "keeps `paladin-core` light"); `897e77e` (`paladin-storage`
  non-optional, facade sqlite fallbacks dropped, `storage-sqlite` retired, ~1,486 LOC);
  `ff829e2` (MinIO/S3 → `paladin-storage` `s3` feature, crate bumped to edition 2024, facade
  `rust-s3` dropped); `5a7c901` (Redis queue → `paladin-storage` `redis-queue` feature, facade
  `redis` dropped); `cf17559` (dead notification fallback adapters deleted, ~1,072 LOC).
- Follow-on commits: `6704807` (orphaned `user_controller.rs` duplicate deleted — "investigation
  showed it was uncompiled residue; paladin-web already owns the live copy and
  `UserServiceTrait`/DTOs already live in paladin-core, so **no user-service split was needed**");
  `4c7857e` (~100 production `println!` → `log::*`, CLI untouched); `74ddf11` (`application/mod.rs`
  + `infrastructure/mod.rs` docs refreshed); `a1e4901` (11 remaining `#[allow(dead_code)]` markers
  justified).
- **Three in-execution corrections to the audit:** `paladin_registry.rs` was **not** a duplicate —
  the facade's 418-LOC impl is richer than battalion's 67-LOC `pub(crate)` copy, so the richer one
  was consolidated *into* battalion rather than deleted blindly; `sqlite_*_repository.rs` were
  **not** redundant in the default build (they were the active default-build impl), resolved via
  the non-optional-storage change; `mysql_content_repository.rs`, the `input/*` fetchers,
  `document/*`, `output/api_content_deliverer.rs` and `error_log_adapter.rs` **were** orphaned and
  uncompiled — safe deletes.
- **Final tally: 15 commits, ~10,250 net LOC removed, one new leaf crate (`paladin-herald`).**
  Persistence adapters (sqlite/mysql/minio/redis) consolidated in `paladin-storage`; Citadel in
  `paladin-memory`. Workspace green on default and across all rewired feature flags.
- Deliberately left in the facade: the Category 4 service relocations (`planning`/`prompt`/
  `temperature`/`handoff` services, `content_ingestion_service`) and the `src/core/` re-export
  shims — "facade-internal decisions the original Epic-3 disposition chose to keep; not slop.
  Revisit only if a future milestone wants them in leaf crates."

## Topic: Milestone 8 deferred registers — the genuine forward-work signal
- source: /workspace/.project/Milestone_8-Facade-Cleanup-Shim-Resolution/deferred-items.md, deferred-features.md
- `deferred-items.md` (2026-06-07, "verified against `main`") is framed as a "Record of intentional
  non-goals (not bugs / not oversights)" and carries an **"Already done (for context — NOT
  deferred)"** section precisely "so readers don't mistake completed work for a gap". That section
  confirms: orphaned dead files and `file_content_repository` deleted; user CLI + TensorFlow stub +
  `ml` flag removed; `HashMapPaladinRegistry` → `paladin-battalion`; `FileCitadel` →
  `paladin-memory`; Herald formatters → new `paladin-herald`; MinIO/S3, Redis and SQLite
  repositories → `paladin-storage` (now non-optional, facade re-exports); notification adapters →
  `paladin-notifications` (facade keeps a re-export only); facade `user_controller.rs` removed;
  actix-web removed from `paladin-web` with a cargo-deny ban; `println!` hygiene down from ~435 to
  **17 across 6 files**.
- Five deferred items D1-D5 with effort/risk ratings and a suggested grouping — recorded in full as
  `REQ-m8-deferred-items-register`. No owners are named and no target milestone is assigned.
- `deferred-features.md` (2026-06-04) preserves the intent of two removed features so they "can be
  reintroduced deliberately (with tests, TDD) when prioritized": the `paladin user …` CLI command
  surface (1,065 LOC, eight subcommands, backend intact, "mostly re-wiring, not new domain work")
  and the TensorFlow ML adapter (636 LOC, `#[doc(hidden)]` placeholder, stub model
  loading/prediction). Both "recoverable verbatim from git history at the Milestone 8 removal commit
  on branch `chore/facade-cleanup-m8-finish`".
- The TensorFlow entry sets a **placement condition on reintroduction**: implement in a dedicated
  `paladin-ml` leaf crate, "consistent with the hexagonal layout — ML inference is an infrastructure
  adapter, not facade code — rather than re-adding it to the facade", with the `ml` feature flag
  moved onto that crate. The port contract `paladin_ports::input::ml_port::MlPort` "remains in the
  workspace, so the integration point is stable".
- Cross-reference note: `deferred-items.md` points at `Epic_7/deferred-features.md`, but the file is
  at the milestone root, not under `Epic_7/`.

## Topic: paladin-ports publish verification — a deferred blocker, closed
- source: /workspace/.project/Milestone_7-Production-Hardening/Epic_4/deferred-paladin-ports-publish-verification.md
- Status **Resolved**. The Task 5.5 `cargo publish --dry-run -p paladin-ports` failure is closed;
  all four deferral exit criteria are recorded satisfied, and `paladin-ai-core` being published
  first is what unblocked it.
- **The only residue is a standing follow-up: "Keep CI/package guardrails that detect crates.io
  package-name collisions early."** Given that name collisions cost Epic 4 two package renames and a
  NO-GO cycle, this is a small but real carry-forward.

## Topic: Version trajectory across runs 1-4 (history, not current state)
- source: run-4 release documents + `.planning/intel/code-verification.md` (run-4 section)
- M7 Epic 4 PRD anchors the first publishable release at lockstep **`0.2.0`**; the M7 overview
  Appendix C shows `[workspace.package] version = "0.2.0"`.
- What actually shipped from M7 Epic 4: **all ten crates published at `0.1.0`**, tagged
  **`v0.1.0-rc.1`** at commit `a9530fc`, 2026-05-28.
- Milestone 8 targets **v0.2.0** throughout (Epics 1-5). Epic 6 targets "post-v0.2.0 patch".
  Epic 7 — created 2026-06-06, the latest run-4 document — targets **"post-v0.5.1 (Unreleased)"**,
  showing v0.3.0 through v0.5.1 all shipped between Epic 5 and Epic 7.
- Current tree: `Cargo.toml` at **`0.6.0`**, branch **`release/v0.7.0`**, latest tag **v0.5.1**.
- The Milestones 8-11 dependency graph's release sequence (v0.2.0 → v0.3.0 → v0.4.0 → v0.5.0)
  therefore completed, and the project has since moved two minors beyond it.
- **Do not treat any v0.1.0-rc.1 artifact as current.** The published-crate lists, docs.rs
  verification and GO sign-off all describe the 0.1.0 state.

## Topic: Code-verification anchors for run-4 claims
- source: /workspace/.planning/intel/code-verification.md (run-4 section)
- Verified SHIPPED: `paladin-herald` crate (three formatters + `lib.rs`); actix-web fully absent
  from `paladin-web` with the `deny.toml` ban in place and three axum delivery routes mounted;
  `paladin-content/src/services/` with zero `use_cases` references anywhere in the workspace;
  `src/application/services/`; all six `src/core/` files exactly as specified; `src/README.md`;
  ten per-crate `make test-*` targets; all five migrated/new benchmarks in their owning crates with
  zero `.disabled` files; `paladin-storage` with `s3`, `redis-queue` and `scheduler` features;
  `crates/paladin-memory/src/citadel/file_citadel.rs`; the `println!` residue at exactly 17 across
  6 files; the `tensorflow_adapter` and `ml` feature both fully absent; `user.rs` absent from the
  CLI command set.
- Verified OPEN or drifted: the RustSec exception list has grown to five vulnerability advisories
  with three encoding files disagreeing; `paladin-herald` has no `CHANGELOG.md`; `Dockerfile.chef`'s
  explicit planner COPY list omits `paladin-herald`; the `api-surface` CI job's stale
  `project/current-exports.txt` baseline is unchanged from run 3 and is re-asserted by M8 Epic 7
  FR-10; `paladin-ports` doctests remain disabled with the same "Task 7.0" comment.
- Superseded by outcome: the `storage-sqlite` feature and optional `paladin-storage`; the per-crate
  ordered publish dry run; the actix-web-in-`paladin-web` requirement; the `STABLE_API.md`
  root-path requirement (relocated into the mdbook); the `160` facade file count (tree reads 136).

---

# Ingest run 5 of 5 — context topics (16)

Source set: Milestones 9-12, `Deferred-QA-CICD-Completion`, `project-management` (46 docs).

## Topic: The Milestone 9 epic plan — completing the *other* half of the platform
- source: /workspace/.project/Milestone_9-Classic-Orchestrator-Completion/overview/Milestone-9_Classic-Orchestrator-Completion.md plus the six `Milestone_9-Epic_N-*.md` extracts
- Framing of record: "The Paladin framework was designed as a dual-purpose platform: a classic
  content orchestration system **and** an AI agent orchestration framework. Milestones 4-8 built and
  refined the AI agent side. The classic orchestration subsystem … was placed structurally in earlier
  milestones but is **operationally incomplete**."
- The concrete gap: in `create_workflow()`, every `WorkflowExecutionOrder` arm was a `println!`
  placeholder that stored a workflow without ever executing its jobs. "Today a developer can define
  and store a workflow, but starting the orchestrator never causes that workflow's jobs to execute."
- Epic sequence and dependency: Epic 1 (execution loop) is the foundation Epics 2, 3 and 4 build on;
  Epic 2 validates the three things that *tell* the orchestrator to run (scheduler tick, durable
  queue, event listener); Epics 3 and 4 build the two directions of the content/agent bridge; Epic 5
  completes user/admin auth; Epic 6 finalizes at v0.3.0.
- **All six Epic DOCs are near-verbatim extracts of the milestone overview.** They add no independent
  content and are consolidated into this topic. Do not double-count their acceptance criteria.

## Topic: The bidirectional agent/orchestrator bridge — the architectural centrepiece of run 5
- source: M9 Epic 3 and Epic 4 PRDs and Epic DOCs
- The vision statement of record: *"ingest a news article → extract/summarize text → invoke an AI
  agent to analyze sentiment and extract entities → store the enriched result"* had **no bridge**
  between the content pipeline (`paladin-content`) and the agent runtime.
- **Direction 1 (content → agent, Epic 3):** `PaladinContentProcessor` and
  `BattalionContentProcessor` implement the pre-existing `ContentProcessor` trait, whose only prior
  implementation was `DefaultContentProcessor` — "a stub that sleeps 100 ms and returns a canned JSON
  blob".
- **Direction 2 (agent → orchestrator, Epic 4):** `OrchestratorPort` lets an agent schedule a job,
  enqueue an item, fire an event or send a notification, bounded by `BridgePolicy`.
- **Both directions hit the same crate-placement wall and resolve it identically:** the concrete
  implementations must live in the **root crate** because a lower crate cannot depend on it. Epic 4
  §6.2 names Epic 3 as precedent — "this is the same constraint encountered and documented in Epic 3".
- The safety framing is explicit and unusual for this corpus: `BridgePolicy` exists so "a misbehaving
  or **prompt-injected** agent cannot schedule unbounded work or spam notifications". This is the
  only place in 199 documents where prompt injection is named as a threat with a designed control.

## Topic: Milestone 10 has five epics, not four
- source: Milestone_10 overview + the five Epic DOCs and PRDs
- The milestone overview and every Epic 1-4 PRD header describe **four** epics ("Epic 2 of 4").
  A fifth — **Epic 5: Release Branch Protection (Tag-from-Main Enforcement)** — exists and was added
  afterwards.
- Its PRD states the reason plainly: the policy exists because of **"the `v0.4.0`-from-feature-branch
  incident"**. Epic 4 cut the v0.4.0 tag; Epic 5 was written to make that impossible again.
- Epic 5's own non-goals acknowledge the residue: **"No rewrite of the existing `v0.4.0` tag/release.
  Reconciling `main` with the released code is a maintainer merge action, noted in docs but not
  performed by this epic."** and, wryly, "Cutting a tag from this feature branch would, correctly, now
  be blocked by the new guards."
- **This is the corpus's only recorded process failure with a corrective epic attached**, and it is
  the clearest example of the project learning from an incident rather than a plan.

## Topic: The supply-chain governance surface — origin policy versus shipped state
- source: M10 Epic 2 DOC and PRD; verified against the tree
- **What the origin documents actually say.** The Epic DOC (Task 2.1) specifies a **single**
  mechanism — "a documented exception process for false positives or unpatched advisories" — with no
  advisory IDs, no counts, and **no mention of `.cargo/audit.toml` or inline `ci.yml` ignores**. The
  PRD is more specific and stricter: FR-1 requires the ignore-list be sourced from `audit.toml`
  *"so the workflow and the config cannot drift"*; FR-3 fixes a four-field comment schema; the
  non-goals name exactly **two** preserved advisories; §8 makes "no inline advisory-ignore flags
  remain in CI" a success metric. **One allow-list, one exception surface, two baseline advisories.**
- **What the tree contains** (verified independently — see `code-verification.md` run 5):
  `.cargo/audit.toml` carries **5** vulnerability ignores, each with dated reasoning.
  `deny.toml` carries **15**: the same 5 vulnerabilities **(now correctly mirrored — this corrects
  run 4)** plus 10 unmaintained/maintenance notices in an explicitly separate, rationalised class.
  `ci.yml:77` runs a bare `cargo audit` with a comment restating the single-source rule — correct.
  `ci.yml:406` runs `cargo audit --ignore RUSTSEC-2023-0071 --ignore RUSTSEC-2025-0111` from a
  **duplicate `security` job that was never removed**, and both jobs carry the identical display name
  "Security Audit".
- **The mechanism of the drift is now known.** The Epic 25 PRD's Appendix B tabulates the *pre-change*
  `ci.yml` as 7 jobs, of which #4 is `security`. Milestone 10 Epic 2 **added** the compliant
  `security-audit` job at `ci.yml:60` without deleting its predecessor at `ci.yml:390`. Milestone 10
  Epic 4's non-goals then locked the configs — "No changes to `deny.toml` or `.cargo/audit.toml`" and
  "No new CI jobs — the Epic 3 pipeline is complete" — so nothing in the milestone was positioned to
  notice.
- **The governance gap, stated precisely.** All 5 audit.toml entries meet FR-3's four-field schema.
  But only the **original 2** carry a formal risk-acceptance record with an **owner (Platform
  Security) and an expiry (2026-09-30)** — that record is `rustsec-remediation-plan.md`, ingested in
  run 4. The 3 newer vulnerability ignores and the 10 unmaintained notices have inline comments and
  revisit conditions but **no owner and no expiry**. The only ingested authorisation for adding
  entries beyond the Epic 2 baseline is Milestone 10 Epic 4 FR-1 step 5, which authorises
  **unmaintained** advisories specifically — not vulnerability advisories.
- **Conclusion:** the drift is **unsanctioned by the origin policy** — it is not an approved exception
  regime. The most consequential single item is the duplicate `security` job, because it makes the
  Epic 2 success metric false on a milestone recorded 100% complete.

## Topic: The mdbook relocation is the corpus's most persistent false-positive generator
- source: M11 Epic 2 PRD §4.5 + §5; corroborated across runs 3, 4 and 5
- `REQ-mdbook-chapter-hierarchy` established the rule: **"Docs with no single-chapter home are placed
  in an `appendix/` chapter rather than dropped."** Combined with Epic 2's non-goal — "Content
  accuracy … is Epic 3's responsibility. This Epic migrates content as-is" — every legacy document
  was moved, unchanged, into `docs/src/appendix/`.
- **Every "missing deliverable" in runs 3, 4 and 5 traces to this one rule.** The complete list now
  reads: `STABLE_API.md`, `docs/FEATURE_FLAGS.md`, `docs/MIGRATION.md`, `docs/CONFIGURATION.md`,
  `docs/BUILD_BASELINES.md`, `docs/INTEGRATION_TESTS.md`, `docs/PERFORMANCE_BASELINE.md`,
  `docs/RELEASE_CHECKLIST.md`, `docs/VERSIONING_POLICY.md`, `docs/SECURITY_SCANNING.md`,
  `docs/RELEASE_AUTOMATION.md`, `docs/BRANCH_PROTECTION.md`, `docs/Design/Design_and_Architecture.md`.
  All thirteen exist under `docs/src/appendix/` or `docs/src/api-reference/`. **None is a gap.**
- **But the rule has a sharp edge.** Milestone 11 Epic 3's non-goals exempt the appendix from
  rewriting — "the 35 appendix files are reference/archive material and are not rewritten in this
  Epic". So `design-and-architecture.md` was relocated into the one chapter nobody was required to
  fix, and it remains **exactly 311 lines with zero coverage of the seven newer subsystems** — the
  same state the February 2026 Deferred-QA PRD described. **The relocation preserved the file and
  froze the gap.**

## Topic: Milestone 11 Epic 6 wrote down a gap, and that created Milestone 12
- source: M11 Epic 6 PRD (FR-5, non-goals, OQ-2/OQ-3) → M12 Epic 1 PRD §1
- The deployment-topologies epic required the HTTP-service-host page to carry **an honest note** that
  "`paladin-web::create_app_router` is the user-management API, not an agent endpoint", and the
  sidecar page to carry a **"no built-in IPC/RPC"** callout. Its non-goals were explicit: "A
  first-class agent-HTTP endpoint or sidecar transport — explicitly out of scope; documented as
  consumer-composed and recorded as open questions (OQ-2, OQ-3)."
- Milestone 12 Epic 1 then opens by **quoting that page back**: *"**Paladin ships no
  agent-execution endpoint.** The web crate's `create_app_router` wires a user-management / auth REST
  API — it does *not* run agents. The agent endpoint is yours to compose."*
- Milestone 12 Epic 7 closes the loop by requiring the docs to "accurately describe the **shipped**
  API + server binary (replacing the old 'compose your own endpoint' framing)". **Verified done** —
  the stale framing returns zero grep matches across `docs/src/`.
- **A documentation epic that recorded a capability gap instead of papering over it directly caused
  the last milestone in the corpus.** This is the single best argument in 199 documents for the
  "record the open question, do not implement it here" discipline that M11 Epics 4 and 6 both state.

## Topic: The Milestone 12 epic architecture — one seam, seven epics
- source: Milestone-12_Web-API.md overview + the seven Epic PRDs
- Every epic is organised around one invariant: **`paladin-web` never depends on the `paladin-ai`
  facade.** Epic 1 defines the `PaladinExecutorPort` seam and the `AgentProvisioner` port; Epic 2 puts
  the concrete implementations in the composition root (the `paladin-server` binary); Epic 5 restates
  the rule for the auth verifier — "mirroring the executor/provisioner seam".
- Deliberate sequencing with explicit deferral at each step: Epic 1 mounts routes **unauthenticated**
  and says so, deferring auth to Epic 5 with the requirement that it be "layerable onto these routes
  later **without changing their handlers' signatures**"; Epic 1 ships an interim
  `{ "error": "<message>" }` body and requires error-body construction be **centralised in one helper
  "so that migration is a single change point"** when Epic 4 lands the unified model. Both promises
  were kept.
- Epic 5's justification for eventually securing it is blunt: "anyone who can reach the port can run,
  register, or delete agents. That is unacceptable for a real deployment: **agent execution spends
  money (LLM calls)** and runtime registration is a powerful capability."
- Status of record: Planning, Version Target v0.6.0, Document Version 1.0, Created 2026-06-07.
  Depends on Milestone 8 Epic 7 landing first. Out of scope: distributed scale-out, per-agent process
  isolation, and any new agent/LLM/orchestration capability.

## Topic: The API is documented as JWT and implemented as opaque tokens
- source: M9 Epic 5 §6.1 + §5; M12 Epic 5 §4.1, §6, §7, OQ-4; verified against the tree
- **Milestone 9 chose opaque bearer tokens deliberately** and listed "JWT/OIDC/OAuth or any external
  identity provider integration" as an explicit **non-goal**. Recorded rationale: no `jsonwebtoken`
  dependency, no signing-key management, immediate revocation (which stateless JWTs cannot do),
  deterministic unit testing, and no new dependencies since `rand` and `sha2` were already present.
- **Milestone 12 specifies JWT throughout** — `Authorization: Bearer <jwt>`, `http.auth.jwt.enabled`,
  `jwt: Option<Arc<dyn AuthPort>>`, "constructs an `AuthPort` **JWT verifier**".
- **The tree implements the M12 shape over the M9 mechanism.** `crates/paladin-web/src/agent_auth.rs`
  has the field named `jwt`, checks bearer first then `x-api-key`, and does constant-time key
  comparison — exactly M12's design. But **no `jsonwebtoken` dependency exists anywhere in the
  workspace**, and the only `AuthPort` implementation is
  `src/infrastructure/adapters/auth/in_memory_token_auth_adapter.rs` — M9's opaque, in-process,
  hashed-token store.
- **M12 Open Question 4 is unanswered because it is unanswerable for the shipped adapter:** "which
  concrete `AuthPort` impl does `paladin-server` wire, and what does it need (signing
  secret/algorithm) from config/env?" An opaque-token store has neither.
- **The operational consequence is real and nobody connected it.** M9 §6.1 recorded the trade-off
  itself — "tokens are validated against an in-process store, so a **multi-process deployment would
  later need a shared store**". Milestone 12 Epic 7 then ships a Kubernetes **Deployment** whose
  whole purpose is multi-process serving. Neither document references the other.

## Topic: Deferred-QA-CICD-Completion — the last deferred register, and the least reliable
- source: prd-deferred-qa-completion.md (PRD-DQC-001) + DEFERRED_COVERAGE.md + Epic_25/prd-cicd-pipeline-enhancement.md
- Origin: **25 subtasks deferred during Epic 24 (Test Hardening, Benchmarks & QA)**, mapped in
  Appendix A to five epics — 25 (CI/CD), 26 (docs and demos), 27 (LLM tool calling), 28 (user_service
  coverage), 29 (listener_service coverage) — across **135 numbered functional requirements**.
- Recommended execution order with recorded rationale: **25 first** ("establishes quality gates that
  validate all subsequent work"), then **27** (highest technical complexity), then **28** ("builds
  reusable mock infrastructure needed by Epic 29"), then **29**, then **26** ("final polish; demos
  showcase all completed features"). 27, 28 and 29 can parallelise given the people.
- Dated **February 14 / February 20, 2026**, status **Draft** — the second-oldest material in run 5
  after the master project plan.
- **Reliability assessment.** Run 4 established Milestone 8's `deferred-items.md` and
  `deferred-features.md` as the highest-fidelity documents in the corpus — every verifiable claim
  matched the tree exactly. **This register is materially weaker.** `listener_service.rs` moved to
  `src/application/services/orchestration/listener.rs` in Milestone 6; `LlmRequest`'s path
  `src/application/ports/output/llm_port.rs` was deleted in Milestone 5; `Design_and_Architecture.md`
  moved into the mdbook appendix in Milestone 11; the README it targets was rewritten by Milestone 11
  Epic 5; and the 57.83% listener baseline predates Milestone 9 Epic 2's test work on the same module.
- **But its scope is real and largely unbuilt.** Epic 25, Epic 26's architecture rewrite and demos,
  and Epic 27 in its entirety are verified open against the tree. **Treat its scope as live and its
  paths and numbers as needing re-measurement.**

## Topic: The three deferred registers, compared
- source: M8 `deferred-items.md` + `deferred-features.md` (run 4); `DEFERRED_COVERAGE.md` +
  `prd-deferred-qa-completion.md` + `Epic_25/prd-cicd-pipeline-enhancement.md` (run 5)
- Together these are the **only trustworthy forward-work signal in the corpus**. Checkbox arithmetic
  is not — see the topic below.
- **M8 `deferred-items.md` (D1-D5):** framed as "Record of intentional non-goals (not bugs / not
  oversights)". Verified exact against the tree. No owners, no target milestone beyond a suggested
  grouping.
- **M8 `deferred-features.md`:** two removed features with explicit reintroduction conditions, both
  recoverable verbatim from git history. Verified exact.
- **Deferred-QA (Epics 25-29):** the largest by requirement count (135 FRs), the oldest, and the one
  with stale paths. Carries effort estimates, story points, sprint ranges and priorities that the
  other two lack, plus a named sign-off and a "Next Review: Epic 27 or Epic 28 planning" trigger that
  was never reached.
- **One direct collision.** `DEFERRED_COVERAGE.md` plans to *test* `user_service.rs` to ≥80%; M8
  `deferred-items.md` D2 plans to **split** the same file across `paladin-core`/`paladin-ports` and a
  facade app-service. Doing D2 first would invalidate much of Epic 28's test surface. **Sequence them
  deliberately or the work is done twice.**

## Topic: Checkbox counts in run 5 — three of four milestones contradicted or vacuous
- source: /workspace/.planning/intel/task-completion-state.md; verified against the tree
- **Milestone 9: 0 open across 6 task lists (100%). Corroborated.** `execute_workflow` at
  `orchestration/mod.rs:382`, `workflow_repository_port.rs`, `sqlite_workflow_repository.rs`,
  `processors/`, `orchestrator_bridge.rs`, `orchestrator_port.rs`, `auth_port.rs` and `argon2` all
  ship.
- **Milestone 10: 0 open across 5 task lists (100%). Corroborated in artefacts, contradicted in one
  acceptance criterion.** Every deliverable exists — but Epic 2's success metric "no inline
  advisory-ignore flags remain in CI" is false, because `ci.yml:406` still carries them.
- **Milestone 11: 26 open, all in `tasks-content-rewrite.md` (92.0%). Plausible and unverifiable by
  file existence.** The open items are tasks 6.0 (six user-guide in-place updates), 7.0 (eight
  deployment/operations updates) and 1.2 (a linkcheck report review). **All fourteen target files
  exist**; whether their *content* is current cannot be settled from the tree. This is the only run-5
  count that is genuinely open work.
- **Milestone 12: 3 open, all in `tasks-api-security-authorization.md` (99.0%). Contradicted.** All
  three are Task 0.0 scaffolding — "Create feature branch", "Update `main` and create/checkout
  `feature/m12-epic5-api-security-authorization`", "Confirm a clean baseline". **The Epic 5 work
  itself shipped:** `crates/paladin-web/src/agent_auth.rs` implements API-key + bearer auth with
  constant-time comparison, redaction tests and a mock verifier. Zero real work is represented.
- **project-management: 1 open (0.0%). Vacuous.** The single item is
  `- [ ] 1.1 Create template → - [x] 1.1 Create template (after completing)` — a **formatting example
  inside a template**, not a task.
- **Final corpus position: of 542 open checkboxes across 75 task lists, the verified genuine
  remainder is small.** Runs 1-2 found counts understating reality (Conclave 129, Sanctum 111 — both
  shipped). Run 3 found the first accurate count (M4's 20). Run 4 found M8's three contradicted.
  Run 5 finds M12's three vacuous and project-management's one nonexistent.

## Topic: Version trajectory completed — v0.3.0 → v0.6.0 across four milestones
- source: M9 Epic 6, M10 Epic 4, M11 Epic 5, M12 Epic 7 + the tree
- Milestone 9 → **0.3.0** (release candidate tag); Milestone 10 → **0.4.0** (annotated tag, first
  crates.io publish through the new pipeline); Milestone 11 → **0.5.0** (docs published to GitHub
  Pages, README rewritten); Milestone 12 → **0.6.0**.
- Current tree: `Cargo.toml` at **0.6.0**, branch **`release/v0.7.0`**, latest tag **v0.5.1**.
- **The run-4 Milestones 8-11 dependency graph's release sequence (v0.2.0 → v0.3.0 → v0.4.0 →
  v0.5.0) completed exactly as planned**, and the project has since moved one further minor plus a
  patch. Run 4 recorded that the graph's *schedule* should be discarded; its *sequence* held.
- Every version-bump requirement in run 5 is **history**. Do not plan them.
- One nuance worth keeping: Milestone 9 Epic 6's non-goals explicitly refuse to reconcile whether the
  prior published version "should" have been 0.2.0 — "this Epic targets `0.3.0` per the Epic
  specification regardless of intervening version numbers". The v0.1.0-rc.1 / v0.2.0 discrepancy run
  4 recorded was noticed and consciously not fixed.

## Topic: `project/current-exports.txt` — the stale path is now written into nine places
- source: M12 Epics 1, 5, 6 and 7; M8 Epic 7 FR-10; `scripts/`; `ci.yml`
- The directory was renamed `project/` → `.project/` in commit `928c6d5`. The baseline now lives at
  `.project/current-exports.txt` (442 KB); `project/current-exports.txt` does not exist.
- Run 3 found five stale references (two scripts, three `ci.yml` lines) and established that
  `check-api-surface.sh` exits 1 with "No baseline found" when the file is absent — **so the
  `api-surface` CI job fails on every run**. Run 4 found all five unchanged and added a sixth: M8
  Epic 7 FR-10 mandates the stale path inside a requirement.
- **Run 5 adds three more**, all in Milestone 12: Epic 1 §7 ("new public items will change
  `project/current-exports.txt`; regenerate the baseline"), Epic 5 §7, Epic 6 `cross_refs`, and Epic 7
  FR-4.6 ("`CHANGELOG.md` and `project/current-exports.txt` reflect the release").
- **Nine references across two scripts, three workflow lines and four requirements, unchanged across
  three ingest runs.** This extends `DEBT-01`. It is the longest-lived unfixed defect in the corpus
  and the cheapest to fix.

## Topic: What Milestone 12 explicitly did not build
- source: M12 overview + the seven Epic PRDs' non-goals
- Recorded as out of scope, with reasons: **distributed scale-out** and **per-agent process
  isolation** (milestone-level); **Garrison (memory) and Arsenal (tools/MCP) wiring for agents** —
  "agents are LLM + prompt only here", deferred as "a later enhancement"; **durable/distributed jobs,
  retries and backpressure** — deferred to the queue/worker topology; **WebSocket / bidirectional
  streaming** — SSE only; **OAuth/OIDC, mTLS, signed requests**; **API-key storage backends, rotation
  and per-key rate limits** — "keys are static config; rotation is operational (edit config +
  restart)"; **fine-grained scopes beyond `allowed_roles`**; **per-route/per-agent rate limits** — the
  limiter is global and IP-based; **metrics/Prometheus and distributed-tracing exporters**;
  **migration off `log` to `tracing`**; **TLS termination and ingress**; **Helm charts and Kustomize**;
  **multi-arch / static-musl images**; **hot-reload of `config.yml`**; **registry persistence across
  restarts**; **client SDK generation**; **versioning the user/delivery routes**.
- **The Garrison/Arsenal omission is the most consequential.** Agents served over HTTP have no memory
  and no tools — they are prompt-plus-model only. That is a substantial functional gap between the
  embedded-library topology and the HTTP-service-host topology, and it is stated once, in a non-goal.

## Topic: The master project plan — origin of Epics 11-18, not new scope
- source: /workspace/.project/project-management/paladin-project-plan-final.md
- Status **Draft**, version 1.0, **January 29, 2026** — the earliest document in run 5 and the
  highest-level planning document in the corpus. 1,937 lines.
- Defines eight epics over 14-18 weeks with the dependency graph **11 → 12 → {13,14} → 15 → {16,17}
  → 18**: Sanctum Memory Foundation, Sanctum RAG Integration, Sentinel Vision System, Autonomous
  Agent Features, Conclave Expert Synthesis, Advanced Battalion Patterns (Council, Grove), Tactical
  Flow DSL, Armory CLI Enhancement. Marks Epics 1-10 **Complete**.
- **Every one of these eight epics shipped and was already ingested in run 2** from
  `.project/Milestone_2-Missing_features`. `code-verification.md` runs 1-2 verified Conclave,
  Sanctum/Qdrant, Council, Grove, Maneuver and Sentinel vision in the tree.
- **Its value to planning is provenance only.** It is the sole record of the epic-level dependency
  graph, the risk assessment and the success metrics for that expansion. Retagging it as PRD would
  raise the precedence of positions that shipped a year ago; it would add no scope.

## Topic: Code-verification anchors for run-5 claims
- source: /workspace/.planning/intel/code-verification.md (run-5 section)
- **Verified SHIPPED:** the whole Milestone 9 orchestrator subsystem (`execute_workflow`,
  `WorkflowRepository` port + SQLite adapter, content processors, `OrchestratorPort` +
  `orchestrator_bridge.rs`, `AuthPort` + argon2); the whole Milestone 10 tooling set
  (`.pre-commit-config.yaml`, `release.toml`, `.github/rulesets/{protect-main-branch,protect-release-tags}.json`,
  `pre-commit.yml`, `verify-tag-source`, `publish-crates`, `make security`/`sbom`/`hooks`/`release`/
  `publish-dry-run`/`release-check`, `cargo-deny` and `osv-scanner` CI jobs, CycloneDX in
  `release.yml`); the Milestone 11 mdbook with linkcheck at `warning-policy = "error"` and all six
  deployment-topology pages; and the whole Milestone 12 web API (`agent_registry.rs`,
  `agent_controller.rs`, `agent_auth.rs`, `error.rs`, `health.rs`, `http_layers.rs`, `job_store.rs`,
  `openapi.rs`, `request_log.rs`, `timeout.rs`, `openapi.json`, `paladin-server` binary,
  `Dockerfile.server`, `k8s/`, utoipa + Swagger UI + tower_governor, version 0.6.0).
- **Verified OPEN:** Epic 25 in its entirety bar one item (no `cli-tests`/`bench-check`/`coverage`
  jobs, no `.codecov.yml`, no Makefile coverage targets, 8 deprecated-action references remaining);
  Epic 26's architecture rewrite (`design-and-architecture.md` still 311 lines, 0 of 7 newer
  subsystems, 0 mermaid blocks), demos (`docs/assets/` empty, no `docs/DEMOS.md`); Epic 27 in its
  entirety (no `tools` field, no `ToolDefinition`, no `ToolCall`); the Epic 28/29 mock infrastructure
  (`tests/common/` does not exist); and the duplicate `security` job at `ci.yml:390-406` violating
  M10 Epic 2's success metric.
- **Corrected from run 4:** `deny.toml` **does** now mirror all five vulnerability advisories from
  `.cargo/audit.toml`. Run 4's "the three 2026 advisories are absent" is no longer true. The
  remaining gap is owner/expiry coverage, not synchronisation.
- **Relocated, not missing:** `listener_service.rs`; `llm_port.rs`; `Design_and_Architecture.md`;
  `BRANCH_PROTECTION.md`; `SECURITY_SCANNING.md`; `RELEASE_AUTOMATION.md`.
