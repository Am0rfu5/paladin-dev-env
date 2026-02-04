# Task List: Epic 16 - Advanced Battalion Patterns

## Relevant Files

### Council Pattern Files
- `src/core/platform/container/battalion/council.rs` - Council domain models (Council, CouncilConfig, CouncilMessage, TurnStrategy, TerminationCondition)
- `src/application/use_cases/battalion/council_service.rs` - Council execution service implementing turn-taking and conversation management
- `tests/unit/battalion/council_tests.rs` - Unit tests for Council domain models
- `tests/unit/battalion/council_service_tests.rs` - Unit tests for CouncilExecutionService
- `tests/integration/battalion/council_integration_tests.rs` - Integration tests for Council with real Paladins

### Grove Pattern Files
- `src/core/platform/container/battalion/grove.rs` - Grove domain models (Grove, Tree, TreeAgent, GroveConfig, RoutingStrategy)
- `src/application/use_cases/battalion/grove_service.rs` - Grove execution service implementing routing strategies
- `src/application/ports/output/embedding_port.rs` - Embedding port for semantic similarity (if not exists)
- `tests/unit/battalion/grove_tests.rs` - Unit tests for Grove domain models
- `tests/unit/battalion/grove_service_tests.rs` - Unit tests for GroveExecutionService
- `tests/integration/battalion/grove_integration_tests.rs` - Integration tests for Grove with embedding service

### Commander Integration Files
- `src/application/use_cases/battalion/commander.rs` - Update Commander with Council/Grove strategy support
- `src/core/platform/container/battalion/mod.rs` - Update BattalionStrategy enum
- `tests/unit/battalion/commander_tests.rs` - Update Commander tests for new strategies

### Example Files
- `examples/council_discussion.rs` - Expert panel discussion example
- `examples/grove_routing.rs` - Task routing to specialists example
- `examples/commander_council.rs` - Commander orchestrating Council
- `examples/commander_grove.rs` - Commander orchestrating Grove
- `examples/cli_configs/council_basic.yml` - Basic Council configuration
- `examples/cli_configs/council_moderated.yml` - Moderator-directed Council configuration
- `examples/cli_configs/grove_keyword.yml` - Grove with KeywordMatch strategy
- `examples/cli_configs/grove_semantic.yml` - Grove with SemanticSimilarity strategy
- `examples/cli_configs/grove_llm.yml` - Grove with LlmRouting strategy

### Documentation Files
- `docs/BATTALION.md` - Update with Council and Grove patterns
- `docs/COUNCIL.md` - New documentation for Council pattern
- `docs/GROVE.md` - New documentation for Grove pattern
- `docs/examples/council_examples.md` - Council usage examples
- `docs/examples/grove_examples.md` - Grove usage examples

### Notes

- All unit tests should achieve ≥80% coverage
- Integration tests should use mock LLM and embedding services where appropriate
- Follow TDD: Write tests first, then implementation
- Follow hexagonal architecture: Core → Application → Infrastructure
- Use existing error handling patterns with `thiserror`
- All public APIs must have rustdoc comments
- Run `cargo test` after each major task completion
- Run `cargo fmt` and `cargo clippy` before committing

## Instructions for Completing Tasks

**IMPORTANT:** As you complete each task, you must check it off in this markdown file by changing `- [ ]` to `- [x]`. This helps track progress and ensures you don't skip any steps.

Example:
- `- [ ] 1.1 Read file` → `- [x] 1.1 Read file` (after completing)

Update the file after completing each sub-task, not just after completing an entire parent task.

**Test & Commit Protocol:**
When all subtasks under a parent task are complete:
1. Run `cargo test` to ensure all tests pass
2. Run `cargo fmt --check` to verify formatting
3. Run `cargo clippy` to check for warnings
4. Stage changes with `git add .`
5. Commit with conventional commit format using the `-m` flag pattern
6. Mark the parent task as complete `[x]`

## Tasks

- [x] 0.0 Create feature branch
  - [x] 0.1 Create and checkout new branch `feature/epic-16-advanced-battalion-patterns`
  - [x] 0.2 Verify current branch with `git branch`

- [x] 1.0 Council Domain Models (US-16.1)
  - [x] 1.1 Read existing battalion module files to understand patterns (`src/core/platform/container/battalion/mod.rs`, `formation.rs`, `phalanx.rs`)
  - [x] 1.2 Create `src/core/platform/container/battalion/council.rs` file
  - [x] 1.3 Define `TurnStrategy` enum with variants: `RoundRobin`, `ModeratorDirected`, `Random`, `VoluntaryWithTimeout { timeout_ms: u64 }`
  - [x] 1.4 Define `TerminationCondition` enum with variants: `MaxRounds`, `Consensus`, `ModeratorDecision`, `Keyword(String)`
  - [x] 1.5 Define `CouncilMessage` struct with fields: speaker (String), content (String), round (u32), timestamp (DateTime<Utc>)
  - [x] 1.6 Implement `Serialize` and `Deserialize` for `CouncilMessage`
  - [x] 1.7 Define `CouncilConfig` struct with fields: max_rounds (u32), turn_strategy (TurnStrategy), termination_condition (TerminationCondition), include_history (bool)
  - [x] 1.8 Implement `Default` for `CouncilConfig` with sensible defaults
  - [x] 1.9 Define `CouncilData` struct with fields: name (String), participant_ids (Vec<String>), moderator_id (Option<String>), config (CouncilConfig)
  - [x] 1.10 Create `Council` type using `Node<CouncilData>` pattern
  - [x] 1.11 Implement builder pattern: `CouncilBuilder` struct
  - [x] 1.12 Implement `CouncilBuilder::new()`, `name()`, `add_participant()`, `moderator()`, `config()`, `build()` methods
  - [x] 1.13 Add validation in `build()` method (non-empty participants, moderator required for ModeratorDirected, etc.)
  - [x] 1.14 Add rustdoc comments for all public types and methods
  - [x] 1.15 Update `src/core/platform/container/battalion/mod.rs` to export council module

- [x] 2.0 Council Execution Service (US-16.2)
  - [x] 2.1 Read existing execution service patterns (`src/application/use_cases/battalion/formation_service.rs`)
  - [x] 2.2 Create `src/application/use_cases/battalion/council_service.rs` file
  - [x] 2.3 Define `CouncilResult` struct with fields: transcript (Vec<CouncilMessage>), conclusion (Option<String>), rounds_completed (u32), termination_reason (TerminationCondition)
  - [x] 2.4 Define `CouncilExecutionService` struct with fields: paladin_port (Arc<dyn PaladinPort>), garrison_port (Option<Arc<dyn GarrisonPort>>)
  - [x] 2.5 Implement `CouncilExecutionService::new()` constructor
  - [x] 2.6 Implement `convene()` method signature: `async fn convene(&self, council: &Council, topic: &str) -> Result<CouncilResult, BattalionError>`
  - [x] 2.7 Implement conversation loop: initialize empty transcript
  - [x] 2.8 Implement `determine_next_speaker()` helper method for turn-taking logic
  - [x] 2.9 Implement RoundRobin strategy: cycle through participants in order
  - [x] 2.10 Implement ModeratorDirected strategy: moderator decides next speaker by parsing response
  - [x] 2.11 Implement speaker execution: call paladin_port with topic + conversation history
  - [x] 2.12 Implement conversation history formatting: convert transcript to readable string
  - [x] 2.13 Record each response as `CouncilMessage` with speaker, content, round, timestamp
  - [x] 2.14 Store each message in Garrison if garrison_port is available
  - [x] 2.15 Implement termination condition checking after each turn
  - [x] 2.16 Implement MaxRounds termination: check rounds_completed >= max_rounds
  - [x] 2.17 Implement Consensus termination: detect keywords like "agree", "consensus reached"
  - [x] 2.18 Implement ModeratorDecision termination: detect moderator saying "discussion concluded"
  - [x] 2.19 Implement Keyword termination: check for custom keyword in responses
  - [x] 2.20 Extract conclusion from final messages (last message or moderator summary)
  - [x] 2.21 Return `CouncilResult` with complete transcript and metadata
  - [x] 2.22 Add error handling for participant execution failures (skip to next speaker)
  - [x] 2.23 Add timeout handling per speaker to prevent blocking
  - [x] 2.24 Add rustdoc comments for all public methods
  - [x] 2.25 Update `src/application/use_cases/battalion/mod.rs` to export council_service

- [x] 3.0 Grove Domain Models (US-16.3)
  - [x] 3.1 Create `src/core/platform/container/battalion/grove.rs` file
  - [x] 3.2 Define `RoutingStrategy` enum with variants: `KeywordMatch`, `SemanticSimilarity`, `LlmRouting`
  - [x] 3.3 Define `TreeAgent` struct with fields: paladin_id (String), expertise_keywords (Vec<String>), expertise_embedding (Option<Vec<f32>>)
  - [x] 3.4 Implement `Serialize` and `Deserialize` for `TreeAgent`
  - [x] 3.5 Define `Tree` struct with fields: name (String), agents (Vec<TreeAgent>)
  - [x] 3.6 Define `GroveConfig` struct with fields: routing_strategy (RoutingStrategy), fallback_tree (Option<String>), similarity_threshold (f32)
  - [x] 3.7 Implement `Default` for `GroveConfig` (KeywordMatch, None, 0.7)
  - [x] 3.8 Define `GroveData` struct with fields: name (String), trees (Vec<Tree>), config (GroveConfig)
  - [x] 3.9 Create `Grove` type using `Node<GroveData>` pattern
  - [x] 3.10 Implement builder pattern: `GroveBuilder` struct
  - [x] 3.11 Implement `GroveBuilder::new()`, `name()`, `add_tree()`, `config()`, `routing_strategy()`, `similarity_threshold()`, `fallback_tree()`, `build()` methods
  - [x] 3.12 Add validation in `build()` method (non-empty trees, at least one agent, valid threshold range)
  - [x] 3.13 Add rustdoc comments for all public types and methods
  - [x] 3.14 Update `src/core/platform/container/battalion/mod.rs` to export grove module

- [x] 4.0 Grove Execution Service (US-16.4)
  - [x] 4.1 Check if `src/application/ports/output/embedding_port.rs` exists
  - [x] 4.2 Create `EmbeddingPort` trait if needed with `embed()` method
  - [x] 4.3 Create `src/application/use_cases/battalion/grove_service.rs` file
  - [x] 4.4 Define `RoutingDecision` struct with fields: selected_tree (String), selected_agent (String), confidence (f32), reasoning (String)
  - [x] 4.5 Define `GroveResult` struct with fields: routing_decision (RoutingDecision), execution_result (String), metadata (HashMap<String, String>)
  - [x] 4.6 Define `GroveExecutionService` struct with fields: paladin_port, embedding_port (Option<Arc<dyn EmbeddingPort>>), llm_port (Arc<dyn LlmPort>)
  - [x] 4.7 Implement `GroveExecutionService::new()` constructor
  - [x] 4.8 Implement `execute()` method signature: `async fn execute(&self, grove: &Grove, task: &str) -> Result<GroveResult, BattalionError>`
  - [x] 4.9 Implement `route_task()` helper method: `async fn route_task(&self, grove: &Grove, task: &str) -> Result<RoutingDecision, BattalionError>`
  - [x] 4.10 Implement KeywordMatch routing: tokenize task, count matching keywords per agent, select highest score
  - [x] 4.11 Implement keyword scoring: case-insensitive matching, normalize task and keywords
  - [x] 4.12 Implement SemanticSimilarity routing: embed task using embedding_port
  - [x] 4.13 Calculate cosine similarity between task embedding and agent embeddings
  - [x] 4.14 Select agent with highest similarity score above threshold
  - [x] 4.15 Implement LlmRouting: create prompt with task description and agent expertise
  - [x] 4.16 Parse LLM JSON response for selected agent and reasoning
  - [x] 4.17 Implement fallback logic: use fallback_tree if no agent meets threshold
  - [x] 4.18 Implement default fallback: select first agent in first tree if no fallback_tree
  - [x] 4.19 Execute selected paladin with original task input
  - [x] 4.20 Build and return `GroveResult` with routing decision and execution result
  - [x] 4.21 Add error handling for routing failures with fallback to KeywordMatch
  - [x] 4.22 Add error handling for missing embeddings when SemanticSimilarity selected
  - [x] 4.23 Add rustdoc comments for all public methods
  - [x] 4.24 Update `src/application/use_cases/battalion/mod.rs` to export grove_service

- [x] 5.0 Commander Integration (US-16.5)
  - [x] 5.1 Read current `src/application/use_cases/battalion/commander.rs` implementation
  - [x] 5.2 Read `BattalionStrategy` enum definition in `src/core/platform/container/battalion/mod.rs`
  - [x] 5.3 Add `Council` variant to `BattalionStrategy` enum
  - [x] 5.4 Add `Grove` variant to `BattalionStrategy` enum
  - [x] 5.5 Update Commander imports to include council_service and grove_service
  - [x] 5.6 Update Commander `execute()` method to handle `BattalionStrategy::Council`
  - [x] 5.7 Update Commander `execute()` method to handle `BattalionStrategy::Grove`
  - [x] 5.8 Implement auto-detection in `analyze_and_select()`: "discuss", "debate", "collaborate" → Council
  - [x] 5.9 Implement auto-detection in `analyze_and_select()`: "route", "expertise", "best agent" → Grove
  - [x] 5.10 Update Commander documentation to include Council and Grove strategies
  - [x] 5.11 Update Auto Mode heuristics documentation with Council keywords
  - [x] 5.12 Update Auto Mode heuristics documentation with Grove keywords
  - [x] 5.13 Remove "specialist" keyword from Grove to avoid conflict with ChainOfCommand
  - [x] 5.14 Update GroveExecutionService to make llm_port optional (fix integration issue)
  - [x] 5.15 Fix grove_service route_by_keywords to always select fallback agent
  - [x] 5.16 Run tests and fix all integration issues

- [ ] 6.0 Error Handling & Validation
  - [ ] 6.1 Read existing `BattalionError` definition
  - [ ] 6.2 Add `CouncilError` variant to `BattalionError` enum
  - [ ] 6.3 Add `GroveError` variant to `BattalionError` enum
  - [ ] 6.4 Define `CouncilError` enum with variants from Appendix B in PRD
  - [ ] 6.5 Implement `Display` trait for `CouncilError`
  - [ ] 6.6 Implement `From<CouncilError>` for `BattalionError`
  - [ ] 6.7 Define `GroveError` enum with variants from Appendix B in PRD
  - [ ] 6.8 Implement `Display` trait for `GroveError`
  - [ ] 6.9 Implement `From<GroveError>` for `BattalionError`
  - [ ] 6.10 Add validation for empty participants in Council
  - [ ] 6.11 Add validation for moderator requirement in ModeratorDirected strategy
  - [ ] 6.12 Add validation for empty trees in Grove
  - [ ] 6.13 Add validation for similarity threshold range (0.0-1.0)
  - [ ] 6.14 Add error handling for missing embeddings in SemanticSimilarity

- [ ] 7.0 Unit Tests
  - [ ] 7.1 Create `tests/unit/battalion/council_tests.rs` file
  - [ ] 7.2 Write test for `CouncilBuilder` basic construction
  - [ ] 7.3 Write test for `CouncilBuilder` validation (empty participants should fail)
  - [ ] 7.4 Write test for `CouncilBuilder` moderator requirement validation
  - [ ] 7.5 Write test for `CouncilConfig` default values
  - [ ] 7.6 Write test for `TurnStrategy` serialization/deserialization
  - [ ] 7.7 Write test for `TerminationCondition` serialization/deserialization
  - [ ] 7.8 Create `tests/unit/battalion/council_service_tests.rs` file
  - [ ] 7.9 Create mock `PaladinPort` for testing
  - [ ] 7.10 Write test for RoundRobin turn-taking logic (3 participants, 2 rounds)
  - [ ] 7.11 Write test for ModeratorDirected turn-taking logic
  - [ ] 7.12 Write test for MaxRounds termination condition
  - [ ] 7.13 Write test for Consensus termination condition (keyword detection)
  - [ ] 7.14 Write test for ModeratorDecision termination condition
  - [ ] 7.15 Write test for Keyword termination condition
  - [ ] 7.16 Write test for conversation history formatting
  - [ ] 7.17 Write test for participant failure handling (should skip to next)
  - [ ] 7.18 Create `tests/unit/battalion/grove_tests.rs` file
  - [ ] 7.19 Write test for `GroveBuilder` basic construction
  - [ ] 7.20 Write test for `GroveBuilder` validation (empty trees should fail)
  - [ ] 7.21 Write test for `GroveConfig` default values
  - [ ] 7.22 Write test for `RoutingStrategy` serialization/deserialization
  - [ ] 7.23 Create `tests/unit/battalion/grove_service_tests.rs` file
  - [ ] 7.24 Write test for KeywordMatch routing with exact matches
  - [ ] 7.25 Write test for KeywordMatch routing with partial matches
  - [ ] 7.26 Write test for KeywordMatch routing with no matches (fallback)
  - [ ] 7.27 Write test for SemanticSimilarity routing with mock embeddings
  - [ ] 7.28 Write test for SemanticSimilarity below threshold (fallback)
  - [ ] 7.29 Write test for LlmRouting with mock LLM response
  - [ ] 7.30 Write test for LlmRouting parsing JSON response
  - [ ] 7.31 Write test for fallback_tree routing behavior
  - [ ] 7.32 Write test for default fallback when no fallback_tree configured
  - [ ] 7.33 Update `tests/unit/battalion/commander_tests.rs` if exists
  - [ ] 7.34 Write test for Commander routing to Council strategy
  - [ ] 7.35 Write test for Commander routing to Grove strategy
  - [ ] 7.36 Write test for auto-detection of Council keywords
  - [ ] 7.37 Write test for auto-detection of Grove keywords
  - [ ] 7.38 Run `cargo test` to verify all unit tests pass
  - [ ] 7.39 Check test coverage with `cargo tarpaulin` or similar (target ≥80%)

- [ ] 8.0 Integration Tests
  - [ ] 8.1 Create `tests/integration/battalion/council_integration_tests.rs` file
  - [ ] 8.2 Set up test fixtures with mock LLM adapter
  - [ ] 8.3 Write integration test: Council with 3 Paladins, RoundRobin, 2 rounds
  - [ ] 8.4 Write integration test: Council with moderator, ModeratorDirected strategy
  - [ ] 8.5 Write integration test: Council with Garrison integration (history storage)
  - [ ] 8.6 Write integration test: Council with MaxRounds termination
  - [ ] 8.7 Write integration test: Council with Consensus termination
  - [ ] 8.8 Create `tests/integration/battalion/grove_integration_tests.rs` file
  - [ ] 8.9 Write integration test: Grove with KeywordMatch routing
  - [ ] 8.10 Write integration test: Grove with SemanticSimilarity routing (mock embedding service)
  - [ ] 8.11 Write integration test: Grove with LlmRouting
  - [ ] 8.12 Write integration test: Grove fallback behavior when no match
  - [ ] 8.13 Write integration test: Grove with multiple trees
  - [ ] 8.14 Create `tests/integration/battalion/commander_integration_tests.rs` or update existing
  - [ ] 8.15 Write integration test: Commander executing Council strategy end-to-end
  - [ ] 8.16 Write integration test: Commander executing Grove strategy end-to-end
  - [ ] 8.17 Write integration test: Commander auto-detecting Council from input
  - [ ] 8.18 Write integration test: Commander auto-detecting Grove from input
  - [ ] 8.19 Write integration test: Concurrent Council and Grove execution
  - [ ] 8.20 Run `make test-all` to verify all integration tests pass

- [ ] 9.0 Example Applications
  - [ ] 9.1 Create `examples/council_discussion.rs` file
  - [ ] 9.2 Set up example with 3 expert Paladins (Security, Legal, Technical)
  - [ ] 9.3 Configure Council with RoundRobin, 3 rounds, MaxRounds termination
  - [ ] 9.4 Implement example topic: "Should we implement two-factor authentication?"
  - [ ] 9.5 Execute council and display formatted transcript
  - [ ] 9.6 Add comments explaining configuration options
  - [ ] 9.7 Test example runs successfully: `cargo run --example council_discussion`
  - [ ] 9.8 Create `examples/grove_routing.rs` file
  - [ ] 9.9 Set up Grove with 2 trees: "Security Experts" and "Performance Experts"
  - [ ] 9.10 Add 2-3 specialized agents per tree with expertise keywords
  - [ ] 9.11 Configure Grove with KeywordMatch strategy
  - [ ] 9.12 Implement example tasks: security review, performance optimization
  - [ ] 9.13 Execute grove and display routing decisions with confidence
  - [ ] 9.14 Test example runs successfully: `cargo run --example grove_routing`
  - [ ] 9.15 Create `examples/commander_council.rs` file
  - [ ] 9.16 Configure Commander with Council strategy
  - [ ] 9.17 Show different turn strategies and termination conditions
  - [ ] 9.18 Test example runs successfully: `cargo run --example commander_council`
  - [ ] 9.19 Create `examples/commander_grove.rs` file
  - [ ] 9.20 Configure Commander with Grove strategy
  - [ ] 9.21 Show all three routing strategies (KeywordMatch, SemanticSimilarity, LlmRouting)
  - [ ] 9.22 Test example runs successfully: `cargo run --example commander_grove`

- [ ] 10.0 CLI Configuration Support
  - [ ] 10.1 Create `examples/cli_configs/council_basic.yml` file
  - [ ] 10.2 Define basic Council configuration: 3 participants, RoundRobin, 5 max_rounds
  - [ ] 10.3 Create `examples/cli_configs/council_moderated.yml` file
  - [ ] 10.4 Define moderated Council: moderator + participants, ModeratorDirected strategy
  - [ ] 10.5 Create `examples/cli_configs/grove_keyword.yml` file
  - [ ] 10.6 Define Grove with KeywordMatch: 2 trees with specialized agents
  - [ ] 10.7 Create `examples/cli_configs/grove_semantic.yml` file
  - [ ] 10.8 Define Grove with SemanticSimilarity: include embeddings placeholder
  - [ ] 10.9 Create `examples/cli_configs/grove_llm.yml` file
  - [ ] 10.10 Define Grove with LlmRouting: agent descriptions for LLM
  - [ ] 10.11 Update CLI parser to load Council configurations from YAML
  - [ ] 10.12 Update CLI parser to load Grove configurations from YAML
  - [ ] 10.13 Test CLI with council config: `cargo run -- --config examples/cli_configs/council_basic.yml --strategy council`
  - [ ] 10.14 Test CLI with grove config: `cargo run -- --config examples/cli_configs/grove_keyword.yml --strategy grove`

- [ ] 11.0 Documentation
  - [ ] 11.1 Read existing `docs/BATTALION.md` structure
  - [ ] 11.2 Update `docs/BATTALION.md` to add Council and Grove sections
  - [ ] 11.3 Add Council overview with diagram of conversation flow
  - [ ] 11.4 Add Grove overview with diagram of routing flow
  - [ ] 11.5 Create `docs/COUNCIL.md` file
  - [ ] 11.6 Write Council introduction and use cases
  - [ ] 11.7 Document turn-taking strategies with examples
  - [ ] 11.8 Document termination conditions with examples
  - [ ] 11.9 Document Garrison integration for conversation history
  - [ ] 11.10 Add configuration examples for Council
  - [ ] 11.11 Add code examples showing Council usage
  - [ ] 11.12 Create `docs/GROVE.md` file
  - [ ] 11.13 Write Grove introduction and use cases
  - [ ] 11.14 Document all three routing strategies with examples
  - [ ] 11.15 Document expertise definition (keywords vs embeddings)
  - [ ] 11.16 Document fallback behavior and configuration
  - [ ] 11.17 Add configuration examples for Grove
  - [ ] 11.18 Add code examples showing Grove usage
  - [ ] 11.19 Create `docs/examples/council_examples.md` file
  - [ ] 11.20 Add 3-5 practical Council examples (expert panels, decision-making, brainstorming)
  - [ ] 11.21 Create `docs/examples/grove_examples.md` file
  - [ ] 11.22 Add 3-5 practical Grove examples (task routing, specialist selection, load distribution)
  - [ ] 11.23 Update main `README.md` to mention Council and Grove patterns
  - [ ] 11.24 Update `docs/QUICKSTART.md` if exists with Council/Grove quickstart

- [ ] 12.0 Final Testing & Validation
  - [ ] 12.1 Run full test suite: `cargo test`
  - [ ] 12.2 Run integration tests with Docker services: `make test-integration-docker`
  - [ ] 12.3 Verify all examples run successfully
  - [ ] 12.4 Run `cargo fmt` to format all code
  - [ ] 12.5 Run `cargo clippy -- -D warnings` to check for warnings
  - [ ] 12.6 Run `cargo audit` to check for security vulnerabilities
  - [ ] 12.7 Verify test coverage meets ≥80% threshold
  - [ ] 12.8 Validate all rustdoc comments are present: `cargo doc --no-deps`
  - [ ] 12.9 Manual test: Council discussion with real LLM (OpenAI/DeepSeek)
  - [ ] 12.10 Manual test: Grove routing with real embedding service
  - [ ] 12.11 Manual test: Commander auto-detection with various inputs
  - [ ] 12.12 Review all error messages for clarity and actionability
  - [ ] 12.13 Verify no breaking changes to existing Battalion patterns
  - [ ] 12.14 Run benchmarks if applicable: `cargo bench`
  - [ ] 12.15 Create PR description summarizing changes
  - [ ] 12.16 Stage all changes: `git add .`
  - [ ] 12.17 Final commit: `git commit -m "feat(battalion): implement Council and Grove patterns" -m "- Add Council conversational collaboration pattern" -m "- Add Grove tree-based agent routing pattern" -m "- Integrate both patterns with Commander" -m "- Add comprehensive tests and documentation" -m "- Implements Epic 16 (US-16.1 through US-16.5)"`
  - [ ] 12.18 Push branch: `git push origin feature/epic-16-advanced-battalion-patterns`
  - [ ] 12.19 Create pull request targeting main branch
  - [ ] 12.20 Verify CI/CD pipeline passes all checks
