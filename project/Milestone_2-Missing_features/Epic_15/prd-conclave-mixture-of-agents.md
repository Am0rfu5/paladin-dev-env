# Product Requirements Document: Conclave - MixtureOfAgents Pattern

**Version:** 1.0  
**Date:** February 2, 2026  
**Epic:** Epic 15  
**Duration:** 2 weeks  
**Priority:** High  
**Dependencies:** Epic 4 (Battalion Orchestration)

---

## 1. Introduction/Overview

The Conclave pattern implements the MixtureOfAgents (MoA) orchestration approach, where multiple expert agents (Paladins) process the same task in parallel from different perspectives, and a designated aggregator agent synthesizes their outputs into a single, high-quality response.

This pattern solves the problem of **limited perspective and expertise** in single-agent systems. By leveraging multiple specialized agents and synthesizing their outputs, Conclave enables:
- Higher quality responses through diverse expert perspectives
- Reduced hallucinations via cross-validation of outputs
- Specialized expertise on complex, multi-faceted problems
- Consensus-building for critical decision-making tasks

**Goal:** Enable developers to orchestrate multiple expert Paladins that work in parallel and synthesize their outputs into comprehensive, high-quality responses.

---

## 2. Goals

1. **Enable Multi-Expert Orchestration**: Provide infrastructure to execute multiple expert agents in parallel on the same task
2. **Synthesize Expert Outputs**: Automatically combine expert perspectives into coherent, comprehensive responses
3. **Resilient Execution**: Handle partial failures gracefully with retry logic and continue with available expert outputs
4. **Developer-Friendly Configuration**: Support both programmatic API and YAML-based configuration for Conclave workflows
5. **Unified Orchestration**: Integrate Conclave as a first-class Battalion strategy in the Commander system
6. **Observable Execution**: Provide configurable observability levels for debugging and monitoring
7. **Production-Ready Examples**: Deliver working examples demonstrating real-world Conclave use cases

---

## 3. User Stories

### US-15.1: Conclave Domain Model
**As a** framework developer  
**I want** domain models for MoA orchestration  
**So that** the pattern has clear, type-safe structure

**Acceptance Criteria:**
- Conclave struct contains: name, expert agents (Vec<Paladin>), aggregator agent (Paladin), configuration
- ConclaveConfig includes: name, timeout, synthesis prompt customization, expert output token limits, retry configuration
- ConclaveResult captures: individual expert outputs (HashMap<String, PaladinResult>), aggregated output, execution metrics, status
- Validation ensures: minimum 2 experts, exactly 1 aggregator, no duplicate agent names
- Proper error types defined (ConclaveError) with clear messages

**Details:**
- Expert outputs are stored with agent name as key for traceability
- Status enum tracks: Success, PartialSuccess (some experts failed), Failed
- Execution metrics include: per-expert execution time, total execution time, retry counts

---

### US-15.2: Conclave Execution Service
**As a** developer  
**I want** to execute MoA workflows with resilient error handling  
**So that** I get high-quality synthesized outputs even when some experts fail

**Acceptance Criteria:**
- ConclaveExecutionService executes all experts in parallel using tokio tasks
- Failed experts are retried up to N times (configurable, default 2) with exponential backoff
- Execution continues with successful expert outputs even if some fail after retries
- Expert outputs are formatted for the aggregator with optional agent name labels
- Aggregator receives a structured prompt containing all expert outputs
- ConclaveResult includes details on which experts succeeded/failed
- Timeout applies to entire execution (experts + aggregation), not individual agents

**Details:**
- Retry strategy: exponential backoff starting at 1s (configurable)
- Expert output formatting options:
  - With names: "Expert 'TechnicalAnalyst': {output}"
  - Without names: "{output1}\n\n{output2}"
- Aggregator prompt template is customizable but has sensible defaults
- If all experts fail, ConclaveError::AllExpertsFailed is returned

---

### US-15.3: Commander Conclave Strategy
**As a** developer  
**I want** Commander to support Conclave as a first-class strategy  
**So that** I can use unified orchestration API across all Battalion patterns

**Acceptance Criteria:**
- BattalionStrategy::Conclave variant added to enum
- CommanderBuilder supports `.aggregator(paladin)` method to specify aggregator
- Default behavior: last agent in the roster becomes aggregator, rest are experts
- Configurable aggregator selection via `aggregator_index` or `aggregator_name`
- Auto-strategy detection considers Conclave when:
  - Multiple agents (3+) with distinct system prompts/roles
  - Task contains synthesis keywords: "compare", "synthesize", "combine perspectives", "expert panel"
  - Task requires multi-perspective analysis
- Commander validates Conclave configuration before execution

**Details:**
- Auto-strategy scoring for Conclave:
  - +3 points: task contains synthesis keywords
  - +2 points: 3+ agents with different expertise (detected via prompt analysis)
  - +1 point: task is a question requiring comprehensive analysis
- If aggregator not specified, Commander selects based on prompt (looks for "synthesize", "aggregate", "combine")

---

### US-15.4: Conclave CLI Support
**As a** developer  
**I want** to run Conclave workflows from CLI with YAML configuration  
**So that** I can use MoA pattern without writing code

**Acceptance Criteria:**
- Command: `paladin battalion run --type conclave --config <file.yaml>`
- YAML schema supports:
  - Aggregator specification (inline or reference)
  - Expert list (inline or references)
  - Conclave-specific config: retry settings, timeout, synthesis prompt
  - Observability level configuration
- Output formats: JSON, Markdown, plain text
- Output includes both individual expert outputs and aggregated result
- Template generation: `paladin battalion new --type conclave --name <name>`
- Generated template includes 3 example experts and 1 aggregator with sensible defaults

**Details:**
- YAML schema validation with helpful error messages
- Template generates with commented examples and documentation
- CLI respects global config settings (e.g., default LLM provider)

---

## 4. Functional Requirements

### Core Domain (FR-C)
1. **FR-C1**: Conclave struct MUST contain a vector of at least 2 expert Paladins and exactly 1 aggregator Paladin
2. **FR-C2**: ConclaveConfig MUST support configurable retry attempts (0-5, default 2) and timeout (10-3600 seconds, default 300)
3. **FR-C3**: ConclaveResult MUST capture individual expert outputs, aggregated output, execution time, and status (Success/PartialSuccess/Failed)
4. **FR-C4**: Validation MUST ensure no duplicate agent names within a Conclave
5. **FR-C5**: ConclaveError enum MUST provide specific error variants: AllExpertsFailed, AggregatorFailed, ConfigurationError, Timeout

### Execution Service (FR-E)
6. **FR-E1**: ConclaveExecutionService MUST execute all experts in parallel using async/await
7. **FR-E2**: Failed experts MUST be retried up to configured retry limit with exponential backoff (1s, 2s, 4s, etc.)
8. **FR-E3**: Execution MUST continue with available expert outputs if some experts fail after retries
9. **FR-E4**: Expert outputs MUST be formatted into a structured prompt for the aggregator
10. **FR-E5**: Aggregator prompt MUST include all successful expert outputs with optional agent name labels
11. **FR-E6**: Timeout MUST apply to entire Conclave execution (experts + aggregation), not per-agent
12. **FR-E7**: ConclaveResult MUST indicate which experts succeeded and which failed

### Commander Integration (FR-M)
13. **FR-M1**: BattalionStrategy enum MUST include Conclave variant
14. **FR-M2**: CommanderBuilder MUST support `.aggregator()` method for explicit aggregator selection
15. **FR-M3**: Commander MUST default to using last agent as aggregator if not explicitly specified
16. **FR-M4**: Commander MUST validate Conclave has minimum 2 experts and 1 aggregator before execution
17. **FR-M5**: Auto-strategy MUST consider Conclave when task contains synthesis keywords or requires multi-perspective analysis

### CLI & Configuration (FR-I)
18. **FR-I1**: CLI MUST support `paladin battalion run --type conclave --config <file>`
19. **FR-I2**: YAML schema MUST support inline and reference-based agent definitions
20. **FR-I3**: YAML schema MUST support Conclave-specific configuration: retry_attempts, timeout_seconds, synthesis_prompt, include_expert_names
21. **FR-I4**: CLI MUST output both individual expert outputs and aggregated result
22. **FR-I5**: CLI MUST support template generation: `paladin battalion new --type conclave`
23. **FR-I6**: Generated templates MUST include 3 example experts with distinct roles and 1 aggregator

### Observability (FR-O)
24. **FR-O1**: Observability levels MUST be configurable: minimal, standard, verbose
25. **FR-O2**: Standard observability MUST include: execution time per expert, total time, retry counts, success/failure status
26. **FR-O3**: Verbose observability MUST include: full expert outputs, token usage, LLM provider details, timestamps
27. **FR-O4**: Minimal observability MUST include: only final aggregated result and overall status

---

## 5. Non-Goals (Out of Scope)

The following are explicitly **NOT** included in this Epic:

1. **Iterative Refinement**: Aggregator cannot request clarifications or feedback from experts (single-pass only)
2. **Multi-Round Deliberation**: No support for experts discussing or revising their outputs based on others
3. **Dynamic Expert Selection**: All experts are defined upfront; no runtime selection based on task analysis
4. **Expert Weighting**: No explicit weighting system for expert opinions (aggregator determines importance naturally through LLM)
5. **Confidence Scoring**: No automatic confidence scores for expert outputs
6. **Expert Specialization Registry**: No automatic matching of tasks to appropriate experts
7. **Cost Optimization**: No automatic selection of cheaper LLM models for less critical experts
8. **Streaming Aggregation**: Aggregator waits for all expert outputs; no streaming synthesis
9. **Human-in-the-Loop**: No support for human review/intervention during Conclave execution

These features may be considered for future enhancements but are not part of the initial MVP.

---

## 6. Design Considerations

### Conclave Domain Model Structure
```rust
// Located in: src/core/platform/container/battalion/conclave.rs
pub struct Conclave {
    pub name: String,
    pub experts: Vec<Paladin>,
    pub aggregator: Paladin,
    pub config: ConclaveConfig,
}

pub struct ConclaveConfig {
    pub name: String,
    pub timeout_seconds: u64,
    pub retry_attempts: u32,
    pub synthesis_prompt: Option<String>,
    pub include_expert_names: bool,
    pub max_expert_output_tokens: Option<usize>,
    pub observability_level: ObservabilityLevel,
}

pub enum ObservabilityLevel {
    Minimal,
    Standard,
    Verbose,
}

pub struct ConclaveResult {
    pub expert_outputs: HashMap<String, PaladinResult>,
    pub aggregated_output: PaladinResult,
    pub execution_time_ms: u64,
    pub expert_execution_times: HashMap<String, u64>,
    pub retry_counts: HashMap<String, u32>,
    pub status: ConclaveStatus,
}

pub enum ConclaveStatus {
    Success,          // All experts succeeded
    PartialSuccess,   // Some experts failed but aggregation succeeded
    Failed,           // Critical failure (all experts failed or aggregator failed)
}
```

### YAML Configuration Schema
```yaml
# Example: examples/cli_configs/conclave_expert_panel.yaml
type: conclave
name: "TechnicalAnalysisPanel"

config:
  timeout_seconds: 300
  retry_attempts: 2
  include_expert_names: true
  observability_level: standard
  synthesis_prompt: |
    You are synthesizing insights from multiple expert analysts.
    Combine their perspectives into a comprehensive, balanced analysis.
    Highlight areas of agreement and respectfully note any divergences.

aggregator:
  inline:
    name: "ChiefAnalyst"
    system_prompt: "Synthesize expert opinions into comprehensive analysis"
    model: "gpt-4o"
    temperature: 0.3

experts:
  - inline:
      name: "TechnicalExpert"
      system_prompt: "Provide deep technical analysis focusing on architecture and implementation"
      model: "gpt-4o"
      temperature: 0.7
  
  - inline:
      name: "BusinessExpert"
      system_prompt: "Analyze from business perspective: ROI, market fit, competitive advantage"
      model: "gpt-4o"
      temperature: 0.7
  
  - inline:
      name: "SecurityExpert"
      system_prompt: "Identify security risks, vulnerabilities, and compliance concerns"
      model: "gpt-4o"
      temperature: 0.5
```

### Aggregator Prompt Template
Default prompt format sent to aggregator:
```
You are synthesizing outputs from multiple expert agents.

Task: {original_task}

Expert Outputs:
---
Expert 'TechnicalExpert':
{output1}

---
Expert 'BusinessExpert':
{output2}

---
Expert 'SecurityExpert':
{output3}

---

Your goal: Synthesize these expert perspectives into a comprehensive, balanced response.
Highlight areas of consensus and note any divergent opinions respectfully.
Provide a cohesive analysis that integrates all relevant insights.
```

---

## 7. Technical Considerations

### Architecture & Location
- **Domain Model**: `src/core/platform/container/battalion/conclave.rs`
- **Execution Service**: `src/application/use_cases/battalion/conclave_execution_service.rs`
- **Commander Integration**: Enhance `src/application/use_cases/battalion/commander.rs`
- **CLI Support**: Enhance `src/bin/paladin-cli.rs` and `src/application/cli/battalion_commands.rs`

### Dependencies
- **Epic 4**: Battalion orchestration infrastructure must be complete
- **Existing Ports**: `PaladinPort` for agent execution
- **Async Runtime**: `tokio` for parallel execution and retry logic
- **Serialization**: `serde` for YAML configuration

### Error Handling Strategy
```rust
#[derive(Debug, thiserror::Error)]
pub enum ConclaveError {
    #[error("All experts failed after retries")]
    AllExpertsFailed,
    
    #[error("Aggregator failed: {0}")]
    AggregatorFailed(String),
    
    #[error("Configuration error: {0}")]
    ConfigurationError(String),
    
    #[error("Execution timeout after {0} seconds")]
    Timeout(u64),
    
    #[error("Expert '{0}' failed: {1}")]
    ExpertError(String, String),
}
```

### Retry Logic Implementation
- Use exponential backoff: 1s, 2s, 4s, 8s, 16s
- Jitter: ±20% random variance to avoid thundering herd
- Retry only transient errors (network, timeout, rate limit)
- Don't retry permanent errors (invalid API key, model not found)

### Performance Considerations
- Parallel expert execution minimizes total latency
- Timeout mechanism prevents runaway executions
- Retry with backoff prevents API rate limit violations
- Token limit per expert prevents excessive API costs

### Testing Strategy
- **Unit Tests**: Domain model validation, error handling, configuration parsing
- **Integration Tests**: Full Conclave execution with mocked LLM responses
- **Functional Tests**: Real LLM calls with test accounts (marked as `#[ignore]`)
- **Example Tests**: Ensure all examples compile and run successfully

---

## 8. Success Metrics

### Functional Success
1. **All User Stories Completed**: 4/4 user stories pass acceptance criteria
2. **Test Coverage**: ≥80% unit test coverage, ≥70% integration test coverage
3. **Examples Working**: All examples execute successfully and demonstrate key features
4. **Documentation Complete**: `docs/guides/conclave-pattern.md` comprehensive and clear

### Performance Metrics
1. **Execution Efficiency**: Total execution time ≤ (max expert time + aggregation time + 10% overhead)
2. **Retry Success Rate**: ≥80% of retried experts eventually succeed
3. **Partial Success Rate**: ≥90% of Conclaves complete with at least 50% expert success
4. **CLI Responsiveness**: Template generation completes in <1 second

### Quality Metrics
1. **Code Quality**: All code passes `cargo clippy` with zero warnings
2. **Code Format**: All code formatted with `cargo fmt`
3. **Documentation**: All public APIs have rustdoc comments with examples
4. **Error Messages**: All error messages are clear and actionable

### Developer Experience
1. **Configuration Ease**: Developers can create Conclave YAML in <5 minutes
2. **Example Clarity**: Junior developers can understand and modify examples
3. **Error Debugging**: Error messages lead to quick problem resolution

---

## 9. Open Questions

### Resolved by Clarifications
✅ **Q1**: How should partial expert failures be handled?  
**A**: Retry failed experts up to N times, then continue with successful outputs (1C)

✅ **Q2**: How should expert opinions be weighted?  
**A**: Let the aggregator agent determine importance naturally through its LLM reasoning (2C)

✅ **Q3**: What level of observability is needed?  
**A**: Configurable levels (minimal, standard, verbose) to suit different use cases (3D)

✅ **Q4**: Should Conclave support iterative refinement?  
**A**: No, single pass only for MVP (experts → aggregator → done) (4A)

✅ **Q5**: What's the implementation priority?  
**A**: Core functionality + CLI/YAML support + examples (5C)

### Remaining Open Questions

1. **Q6**: Should we provide pre-built expert agent templates (e.g., "TechnicalExpert", "BusinessAnalyst")?
   - **Impact**: Medium - would improve developer experience but adds maintenance burden
   - **Decision Needed By**: Before US-15.4 implementation

2. **Q7**: Should Conclave support mixing different LLM providers for different experts?
   - **Impact**: Low - already supported via Paladin configuration, just needs documentation
   - **Decision Needed By**: Before documentation completion

3. **Q8**: What's the maximum recommended number of experts in a Conclave?
   - **Impact**: Medium - affects performance guidance and cost estimates
   - **Decision Needed By**: Before documentation completion
   - **Suggested Answer**: 3-7 experts (based on context window limits and cost)

4. **Q9**: Should the CLI support interactive mode for building Conclave configurations?
   - **Impact**: Low - nice-to-have for future enhancement
   - **Decision Needed By**: Post-MVP consideration

5. **Q10**: Should we implement Conclave result caching to avoid re-running expensive expert panels?
   - **Impact**: Medium - significant for production use but adds complexity
   - **Decision Needed By**: Post-MVP consideration

---

## 10. Implementation Checklist

### Phase 1: Domain Model (Week 1, Days 1-2)
- [ ] Create `src/core/platform/container/battalion/conclave.rs`
- [ ] Implement Conclave, ConclaveConfig, ConclaveResult structs
- [ ] Implement ConclaveStatus, ObservabilityLevel enums
- [ ] Implement ConclaveError enum
- [ ] Add validation logic (min experts, unique names)
- [ ] Write unit tests for domain model
- [ ] Run `cargo test`, `cargo fmt`, `cargo clippy`

### Phase 2: Execution Service (Week 1, Days 3-5)
- [ ] Create `src/application/use_cases/battalion/conclave_execution_service.rs`
- [ ] Implement parallel expert execution with tokio
- [ ] Implement retry logic with exponential backoff
- [ ] Implement expert output formatting
- [ ] Implement aggregator prompt construction
- [ ] Implement observability levels
- [ ] Write unit tests with mocked PaladinPort
- [ ] Write integration tests
- [ ] Run full test suite

### Phase 3: Commander Integration (Week 2, Days 1-2)
- [ ] Add BattalionStrategy::Conclave variant
- [ ] Enhance CommanderBuilder with `.aggregator()` method
- [ ] Implement default aggregator selection (last agent)
- [ ] Implement Conclave validation in Commander
- [ ] Update auto-strategy detection logic
- [ ] Write tests for Commander Conclave support
- [ ] Run full test suite

### Phase 4: CLI & YAML Support (Week 2, Days 3-4)
- [ ] Define YAML schema for Conclave
- [ ] Implement `paladin battalion run --type conclave`
- [ ] Implement `paladin battalion new --type conclave`
- [ ] Create template generation logic
- [ ] Implement output formatting (JSON, Markdown, text)
- [ ] Write CLI integration tests
- [ ] Run full test suite

### Phase 5: Examples & Documentation (Week 2, Day 5)
- [ ] Create `examples/conclave_expert_panel.rs`
- [ ] Create `examples/cli_configs/conclave_expert_panel.yaml`
- [ ] Create `examples/cli_configs/conclave_code_review.yaml`
- [ ] Write `docs/guides/conclave-pattern.md`
- [ ] Update `docs/BATTALION.md` with Conclave section
- [ ] Update project README with Conclave example
- [ ] Ensure all examples compile and run
- [ ] Run `cargo doc` and verify documentation

### Phase 6: Final Testing & Polish (Week 2, End)
- [ ] Run full test suite: `make test-all`
- [ ] Run `make clean-code` (fmt + clippy + check)
- [ ] Test all examples manually
- [ ] Test CLI commands manually
- [ ] Review all documentation for clarity
- [ ] Security scan with Snyk
- [ ] Performance benchmarking (optional)
- [ ] Create Epic 15 completion report

---

## 11. Related Documentation

- Epic 15 User Stories: `project/Milestone_2-Missing_features/Epic_15/epic15.md`
- Battalion Architecture: `docs/BATTALION.md`
- Commander Guide: `docs/guides/commander-pattern.md`
- CLI Usage: `docs/CLI_USAGE.md`
- Project Plan: `project/Milestone_2-Missing_features/Project Plan Missing Features`

---

## Appendix: Example Use Cases

### Use Case 1: Code Review Panel
**Scenario**: Multi-perspective code review with security, performance, and maintainability experts

**Experts**:
- SecurityExpert: Identifies vulnerabilities and security issues
- PerformanceExpert: Analyzes algorithmic efficiency and bottlenecks
- MaintainabilityExpert: Reviews code structure, readability, and documentation

**Aggregator**: LeadReviewer synthesizes findings into actionable review report

### Use Case 2: Business Strategy Analysis
**Scenario**: Comprehensive business strategy evaluation

**Experts**:
- MarketAnalyst: Market trends and competitive landscape
- FinancialAnalyst: Financial viability and ROI projections
- RiskAnalyst: Risk assessment and mitigation strategies
- CustomerInsightExpert: Customer needs and pain points

**Aggregator**: StrategicAdvisor combines insights into strategic recommendations

### Use Case 3: Technical Architecture Design
**Scenario**: System architecture design with multi-disciplinary input

**Experts**:
- BackendArchitect: Backend design and data flow
- FrontendArchitect: UI/UX and frontend architecture
- InfrastructureArchitect: Deployment, scaling, and infrastructure
- SecurityArchitect: Security architecture and compliance

**Aggregator**: ChiefArchitect synthesizes into cohesive architecture document

---

**End of PRD**
