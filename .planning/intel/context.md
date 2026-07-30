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
