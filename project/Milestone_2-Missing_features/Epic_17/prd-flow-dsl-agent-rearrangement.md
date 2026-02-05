# Product Requirements Document: Flow DSL & Agent Rearrangement

## Document Information
- **Version:** 1.0
- **Created:** February 5, 2026
- **Epic:** Epic 17
- **Feature:** Flow DSL & Agent Rearrangement (Maneuver Pattern)
- **Priority:** Medium
- **Target Audience:** Internal developers, external developers, enterprise users
- **Dependencies:** Epic 4 (Battalion Orchestration), Epic 15 (Conclave), Epic 16 (Advanced Patterns)

---

## 1. Introduction/Overview

The Flow DSL (Domain-Specific Language) feature provides a simple, string-based syntax for defining complex agent workflows without verbose configuration. The **Maneuver** pattern (formerly "AgentRearrange") enables developers to express sequential, parallel, and nested agent relationships using intuitive operators like `->` for sequential execution and `,` for parallel execution.

### Problem Statement
Currently, defining complex multi-agent workflows in Paladin requires:
- Verbose YAML configuration files
- Manual construction of Battalion objects in code
- Deep understanding of the framework's API
- Repetitive boilerplate for common patterns

This creates friction for rapid prototyping and increases the learning curve for new developers.

### Solution
A lightweight DSL that allows workflows to be expressed as simple strings:
```rust
"researcher -> writer"              // Sequential
"researcher -> writer, editor"      // Fan-out
"researcher, analyst -> writer"     // Fan-in
"planner -> (coder -> tester), docs" // Nested patterns
```

### Goal
Enable developers to define flexible, maintainable agent workflows that can handle moderate complexity (10-30 agents) with configurable error handling and intermediate observability, suitable for development, prototyping, and enterprise production use.

---

## 2. Goals

1. **Simplicity**: Reduce workflow definition from 50+ lines of YAML to a single string expression
2. **Flexibility**: Support common patterns (sequential, parallel, fan-out, fan-in, nested) with moderate complexity
3. **Maintainability**: Clear, readable syntax that non-technical stakeholders can understand
4. **Configurability**: Allow users to configure error handling strategies per workflow
5. **Observability**: Provide intermediate visualization and timing metrics for debugging
6. **Integration**: Seamlessly integrate with existing Commander and Battalion infrastructure
7. **Production-Ready**: Support 10-30 agent workflows with configurable error handling

---

## 3. User Stories

### Primary User Stories

**US-17.1: Flow DSL Parser**
- **As a** developer
- **I want** to define workflows with simple string syntax
- **So that** I can quickly express agent relationships without verbose configuration

**US-17.2: Maneuver Domain Model**
- **As a** framework developer
- **I want** domain models for flow-based orchestration
- **So that** the pattern has clear structure and validates agent references

**US-17.3: Maneuver Execution Service**
- **As a** developer
- **I want** to execute flow-based workflows with configurable error handling
- **So that** complex patterns run reliably and I can control failure behavior

**US-17.4: Commander Maneuver Strategy**
- **As a** developer
- **I want** Commander to support flow-based workflows
- **So that** I can use the unified API and CLI

**US-17.5: Flow Visualization**
- **As a** developer
- **I want** to visualize flow expressions with timing metrics
- **So that** I can understand, debug, and document workflows

---

## 4. Functional Requirements

### FR-1: Flow DSL Syntax
The parser **must** support the following syntax:

| Syntax | Meaning | Example |
|--------|---------|---------|
| `a -> b` | Sequential: execute `a`, then `b` | `"researcher -> writer"` |
| `a, b` | Parallel: execute `a` and `b` concurrently | `"writer, editor"` |
| `a -> b, c` | Fan-out: execute `a`, then `b` and `c` in parallel | `"planner -> coder, tester"` |
| `a, b -> c` | Fan-in: execute `a` and `b` in parallel, then `c` | `"researcher, analyst -> writer"` |
| `(a -> b)` | Grouping: treat `a -> b` as a unit | `"a -> (b -> c), d"` |
| `a -> b -> c` | Chain: sequential execution across 3+ agents | `"plan -> code -> test"` |

### FR-2: Parser Implementation
- **FR-2.1**: Parser must be in `src/core/platform/container/battalion/parser/` module
- **FR-2.2**: Parser must return `Result<FlowExpression, FlowParseError>`
- **FR-2.3**: Parser must validate:
  - Balanced parentheses
  - Valid agent identifiers (alphanumeric, underscore, hyphen)
  - No empty groups
  - No consecutive operators
- **FR-2.4**: Parser must provide clear error messages:
  - Show position of error in expression
  - Suggest corrections for common mistakes
  - Example: `"Expected '->' or ',' at position 15, found ')'. Did you mean '-> )'?"`

### FR-3: FlowExpression AST
- **FR-3.1**: Must represent parsed flow as an Abstract Syntax Tree (AST)
- **FR-3.2**: Must support three node types:
  ```rust
  pub enum FlowExpression {
      Agent(String),                    // Leaf node: agent name
      Sequential(Vec<FlowExpression>),  // Sequential execution
      Parallel(Vec<FlowExpression>),    // Concurrent execution
  }
  ```
- **FR-3.3**: Must be serializable/deserializable for storage and debugging

### FR-4: Maneuver Domain Model
- **FR-4.1**: `Maneuver` struct must be in `src/core/platform/container/battalion/maneuver.rs`
- **FR-4.2**: Must contain:
  - `name: String` - Workflow identifier
  - `agents: HashMap<String, Paladin>` - Agent registry
  - `flow: FlowExpression` - Parsed workflow
  - `config: ManeuverConfig` - Execution configuration
- **FR-4.3**: Must validate that all agent names in flow exist in agents map
- **FR-4.4**: Must support workflows with 10-30 agents with reasonable nesting depth (max 5 levels)

### FR-5: ManeuverConfig
- **FR-5.1**: Must support configurable error strategies:
  ```rust
  pub enum ErrorStrategy {
      FailFast,          // Stop entire workflow on first error
      ContinueParallel,  // Continue parallel branches, fail sequence
      IgnoreErrors,      // Log errors but continue execution
  }
  ```
- **FR-5.2**: Must support timeout configuration:
  - `timeout_seconds: u64` - Overall workflow timeout
  - `agent_timeout_seconds: Option<u64>` - Per-agent timeout override
- **FR-5.3**: Must support output passing configuration:
  - `pass_output_as_input: bool` - Chain outputs in sequential steps
  - `output_format: OutputFormat` - How to format multiple outputs for fan-in
- **FR-5.4**: Must support execution tracing:
  - `collect_timing_metrics: bool` - Record execution time per agent
  - `capture_intermediate_outputs: bool` - Store all outputs (not just final)

### FR-6: Maneuver Execution Service
- **FR-6.1**: Must execute flow expressions recursively
- **FR-6.2**: Sequential execution (`a -> b`):
  - Execute `a` with input
  - If `pass_output_as_input`, use `a`'s output as `b`'s input, else use original input
  - Return `b`'s output as final result
- **FR-6.3**: Parallel execution (`a, b`):
  - Execute both agents concurrently with same input
  - Wait for all to complete (respecting timeout)
  - Aggregate results based on configuration
- **FR-6.4**: Nested execution:
  - Recursively evaluate sub-expressions
  - Maintain proper order of operations
- **FR-6.5**: Error handling:
  - Respect `ErrorStrategy` configuration
  - Provide detailed error context (which agent, at what step)
- **FR-6.6**: Result tracking:
  - Return `ManeuverResult` with:
    - `final_output: String`
    - `step_outputs: HashMap<String, PaladinResult>`
    - `execution_order: Vec<String>` (agents in execution order)
    - `timing_metrics: Option<HashMap<String, Duration>>`

### FR-7: Commander Integration
- **FR-7.1**: Must add `BattalionStrategy::Maneuver` variant
- **FR-7.2**: `CommanderBuilder` must accept:
  - `flow(expression: &str)` method
  - `error_strategy(strategy: ErrorStrategy)` method
- **FR-7.3**: Auto-strategy must **NOT** select Maneuver (explicit only)
- **FR-7.4**: Must validate that all agents referenced in flow are registered with Commander

### FR-8: CLI Support
- **FR-8.1**: Must support command:
  ```bash
  paladin battalion run --type maneuver \
    --flow "researcher -> writer, editor -> reviewer" \
    --config maneuver.yaml
  ```
- **FR-8.2**: Must support YAML configuration:
  ```yaml
  type: maneuver
  flow: "researcher -> writer, editor -> reviewer"
  config:
    error_strategy: continue_parallel
    timeout_seconds: 300
    pass_output_as_input: true
    collect_timing_metrics: true
  agents:
    - name: researcher
      # ... agent config
  ```
- **FR-8.3**: Must provide template generation:
  ```bash
  paladin battalion new --type maneuver > maneuver.yaml
  ```

### FR-9: Flow Visualization
- **FR-9.1**: Must provide ASCII diagram generation:
  - `FlowVisualizer::to_ascii(expr: &FlowExpression) -> String`
  - Show clear visual representation of flow structure
  - Indicate parallel branches and sequential chains
- **FR-9.2**: Must provide Mermaid diagram generation:
  - `FlowVisualizer::to_mermaid(expr: &FlowExpression) -> String`
  - Generate valid Mermaid.js syntax for documentation
- **FR-9.3**: Must provide CLI command:
  ```bash
  paladin battalion visualize --flow "a -> b, c -> d"
  paladin battalion visualize --flow "..." --format mermaid
  ```
- **FR-9.4**: Must show timing metrics when available:
  - Agent execution time overlaid on visualization
  - Total workflow execution time
  - Bottleneck identification (slowest agent highlighted)

### FR-10: Validation & Error Handling
- **FR-10.1**: Must validate flow at construction time:
  - All agent names in flow exist in registry
  - No self-references or circular dependencies
  - Reasonable complexity (configurable max depth/width)
- **FR-10.2**: Must provide clear error messages for:
  - Parse errors with position
  - Missing agent references
  - Timeout exceeded (which agent)
  - Agent execution failures
- **FR-10.3**: Must support graceful degradation:
  - Partial results when possible
  - Clear indication of which steps failed
  - Recovery suggestions in error messages

---

## 5. Non-Goals (Out of Scope)

1. **Dynamic Flow Modification**: Flows cannot be changed during execution (FR confirmed by user selection 5A)
2. **Conditional Branching**: No if/else logic in DSL (use Campaign pattern for this)
3. **Loops/Iteration**: No for/while constructs (use MaxLoops within individual Paladins)
4. **Variable Assignment**: No ability to store/reference intermediate results by name
5. **Complex Aggregation Functions**: No custom merge logic in DSL (use dedicated aggregator Paladin)
6. **Automatic Strategy Selection**: Auto-strategy will not select Maneuver (too application-specific)
7. **Advanced Observability**: No distributed tracing or integration with external monitoring (OpenTelemetry, etc.)
8. **Flow Optimization**: No automatic reordering or optimization of flow structure
9. **Persistent Flow State**: No checkpointing/resumability (use Citadel for that, Epic 7)
10. **Agent Resource Management**: No CPU/memory limits or resource pools

---

## 6. Design Considerations

### 6.1 Architecture
- **Layer**: Application (use case) + Core (domain)
- **Pattern**: Hexagonal architecture maintained
- **Dependencies**: 
  - Core: `FlowExpression`, `Maneuver`, `ManeuverConfig` (domain models)
  - Application: `ManeuverExecutionService`, `FlowParser` (business logic)
  - No infrastructure dependencies for core/app layers

### 6.2 User Interface
- **Primary**: Rust API via `CommanderBuilder`
- **Secondary**: CLI with YAML configuration
- **Tertiary**: Direct `Maneuver` construction for advanced use cases

### 6.3 Performance Considerations
- Parser should handle flows of 30+ agents in < 1ms
- Execution overhead < 10ms for orchestration logic
- Parallel execution should leverage Tokio efficiently
- Memory footprint: O(n) where n = number of agents

### 6.4 Error Handling Philosophy
- **Fail-fast by default**: Catch errors early (parse time, construction time)
- **Configurable runtime behavior**: User chooses error strategy
- **Rich context**: Always include agent name, step, and suggestion in errors

### 6.5 Extensibility
- Flow syntax may be extended in future (e.g., `|` for conditional, `*` for broadcast)
- `FlowExpression` AST is extensible without breaking changes
- `ErrorStrategy` is enum, can add new strategies

---

## 7. Technical Considerations

### 7.1 Module Structure
```
src/
├── core/
│   └── platform/
│       └── container/
│           └── battalion/
│               ├── maneuver.rs         # Domain model
│               └── parser/
│                   ├── mod.rs          # FlowParser
│                   ├── lexer.rs        # Tokenization
│                   ├── ast.rs          # FlowExpression
│                   └── error.rs        # FlowParseError
├── application/
│   └── use_cases/
│       └── battalion/
│           ├── maneuver_service.rs     # ManeuverExecutionService
│           └── flow_visualizer.rs      # FlowVisualizer
└── infrastructure/
    └── adapters/
        └── cli/
            └── battalion_commands.rs    # CLI integration
```

### 7.2 Dependencies
- **Existing**: `tokio`, `futures` (async execution)
- **New (optional)**: 
  - Parser combinator library: `nom` or hand-written recursive descent
  - ASCII diagram: custom implementation or `ptree`

### 7.3 Testing Strategy
- **Unit tests**: Parser with valid/invalid inputs
- **Unit tests**: FlowExpression construction and validation
- **Unit tests**: Each error strategy behavior
- **Integration tests**: Full workflow execution with mock Paladins
- **Integration tests**: Timeout and error handling
- **Performance tests**: 30-agent workflows complete in < 5s

### 7.4 Configuration Integration
```yaml
# config.yml
maneuver:
  default_error_strategy: "continue_parallel"
  default_timeout_seconds: 300
  max_nesting_depth: 5
  max_parallel_branches: 10
  pass_output_as_input: true
  collect_timing_metrics: true
```

### 7.5 Backward Compatibility
- New feature, no breaking changes
- Existing Battalion patterns unaffected
- Commander API extended, not modified

---

## 8. Success Metrics

### 8.1 Adoption Metrics
- **Target**: 30% of new Battalion workflows use Maneuver within 2 months of release
- **Measure**: Count of `BattalionStrategy::Maneuver` instantiations in telemetry

### 8.2 Developer Experience Metrics
- **Target**: Reduce workflow definition LOC by 80% for common patterns
- **Measure**: Compare YAML lines before (Formation/Phalanx) vs. after (Maneuver)
- **Target**: 90% of developers can create working flow within 5 minutes
- **Measure**: User study timing from tutorial start to first successful execution

### 8.3 Performance Metrics
- **Target**: Parsing < 1ms for 99% of flows
- **Target**: Orchestration overhead < 2% of total execution time
- **Target**: Support 30 agents with < 100MB memory overhead
- **Measure**: Automated benchmarks in CI

### 8.4 Reliability Metrics
- **Target**: 100% of parse errors include helpful suggestions
- **Target**: Zero panics from malformed input
- **Target**: All error paths tested and documented
- **Measure**: Code coverage + manual review

### 8.5 Documentation Metrics
- **Target**: 100% of public API has rustdoc with examples
- **Target**: 3+ complete examples in `/examples`
- **Target**: Guide document < 2000 words
- **Measure**: Doc coverage tool + manual review

---

## 9. Open Questions

### 9.1 Parser Implementation
- **Q1**: Use parser combinator library (nom) or hand-written recursive descent?
  - **Recommendation**: Hand-written for simplicity, fewer dependencies
  - **Decision by**: Lead developer review

### 9.2 Output Aggregation for Fan-In
- **Q2**: How should multiple parallel outputs be combined for fan-in scenarios?
  - **Option A**: Concatenate with separator (e.g., `"\n---\n"`)
  - **Option B**: JSON array of outputs
  - **Option C**: User-provided aggregation function
  - **Recommendation**: Start with Option A, make configurable
  - **Decision by**: User story US-17.3 acceptance criteria

### 9.3 Timing Metrics Storage
- **Q3**: Where should timing metrics be persisted?
  - **Option A**: In-memory only (part of ManeuverResult)
  - **Option B**: Optional logging to file
  - **Option C**: Integration with Garrison for historical analysis
  - **Recommendation**: Start with Option A, consider B/C in future
  - **Decision by**: Epic 17 completion review

### 9.4 Visualization Integration
- **Q4**: Should visualization be integrated with documentation generation?
  - **Recommendation**: Yes, add `--output` flag to save diagrams
  - **Decision by**: US-17.5 implementation

### 9.5 Error Recovery
- **Q5**: Should we support retry logic for failed agents?
  - **Consideration**: Paladins already have max_loops for retries
  - **Recommendation**: Defer to Paladin-level retry, not Maneuver-level
  - **Decision by**: Epic 17 kickoff

---

## 10. Implementation Phases

### Phase 1: Foundation (Week 1)
- [ ] US-17.1: Flow DSL Parser (3 days)
  - Implement lexer
  - Implement parser
  - Add validation
  - Write 50+ test cases
- [ ] US-17.2: Maneuver Domain Model (2 days)
  - Define structs in core layer
  - Add validation logic
  - Write unit tests

### Phase 2: Execution (Week 1.5)
- [ ] US-17.3: Maneuver Execution Service (4 days)
  - Implement sequential execution
  - Implement parallel execution
  - Implement error strategies
  - Add timing metrics
  - Write integration tests

### Phase 3: Integration (Week 2)
- [ ] US-17.4: Commander Integration (2 days)
  - Add BattalionStrategy variant
  - Extend CommanderBuilder
  - Add CLI support
  - Write integration tests
- [ ] US-17.5: Flow Visualization (2 days)
  - Implement ASCII generator
  - Implement Mermaid generator
  - Add CLI command
  - Add timing overlay

### Phase 4: Polish (Week 2)
- [ ] Documentation (1 day)
  - Write `docs/guides/flow-dsl.md`
  - Add rustdoc examples
  - Create tutorial
- [ ] Examples (1 day)
  - `examples/maneuver_workflow.rs`
  - `examples/cli_configs/maneuver.yaml`
  - Complex nested example

---

## 11. Acceptance Criteria (Epic Level)

Epic 17 is considered complete when:

1. ✅ All 5 user stories pass acceptance criteria
2. ✅ Flow DSL parser handles all specified syntax
3. ✅ Maneuver execution supports 30-agent workflows
4. ✅ All three error strategies implemented and tested
5. ✅ Commander integration complete with CLI support
6. ✅ Visualization working for ASCII and Mermaid
7. ✅ Test coverage ≥ 80% for new code
8. ✅ All public APIs have rustdoc with examples
9. ✅ Documentation complete in `docs/guides/flow-dsl.md`
10. ✅ At least 2 working examples in `/examples`
11. ✅ Performance benchmarks passing (parse < 1ms, overhead < 2%)
12. ✅ Zero clippy warnings
13. ✅ All code formatted with `cargo fmt`
14. ✅ Integration tests passing in CI

---

## 12. Dependencies & Risks

### 12.1 Dependencies
- **Epic 4**: Battalion Orchestration (must be complete)
- **Epic 15**: Conclave pattern (reference implementation)
- **Epic 16**: Council pattern (similar recursive execution)

### 12.2 Risks

| Risk | Impact | Probability | Mitigation |
|------|--------|-------------|------------|
| Parser complexity exceeds estimates | High | Medium | Start with hand-written parser, add combinator library if needed |
| Performance issues with 30+ agents | High | Low | Benchmark early, optimize parallel execution with join sets |
| Error strategy edge cases | Medium | Medium | Comprehensive integration tests for all strategies |
| Visualization ASCII art difficult | Low | Medium | Use simple box-drawing, add Mermaid as primary option |
| CLI integration conflicts | Medium | Low | Coordinate with Epic 18 (CLI Enhancement) |

### 12.3 Assumptions
1. Tokio runtime available for parallel execution
2. Paladins are independently executable (no shared state issues)
3. Users understand basic graph/DAG concepts
4. Most workflows will be < 20 agents
5. ASCII visualization sufficient for terminal use

---

## 13. Documentation Deliverables

### 13.1 User Guide
- **File**: `docs/guides/flow-dsl.md`
- **Sections**:
  - Introduction & motivation
  - Syntax reference with examples
  - Error handling strategies
  - Visualization guide
  - Best practices
  - Troubleshooting

### 13.2 API Documentation
- Rustdoc for all public items:
  - `FlowParser`
  - `FlowExpression`
  - `Maneuver`
  - `ManeuverConfig`
  - `ManeuverExecutionService`
  - `FlowVisualizer`

### 13.3 Examples
- `examples/maneuver_workflow.rs` - Basic usage
- `examples/maneuver_complex.rs` - Nested patterns
- `examples/maneuver_error_handling.rs` - Error strategies
- `examples/cli_configs/maneuver.yaml` - YAML configuration

### 13.4 Migration Guide
- Not applicable (new feature)

---

## Appendix A: Syntax Examples

### Simple Sequential
```rust
"agent1 -> agent2"
```
Execute agent1, then agent2 with agent1's output as input.

### Simple Parallel
```rust
"agent1, agent2"
```
Execute agent1 and agent2 concurrently with same input.

### Fan-Out
```rust
"researcher -> writer, editor"
```
Execute researcher, then writer and editor in parallel with researcher's output.

### Fan-In
```rust
"researcher, analyst -> writer"
```
Execute researcher and analyst in parallel, then writer with combined outputs.

### Nested Sequential
```rust
"planner -> (coder -> tester), docs"
```
Execute planner, then execute (coder->tester) and docs in parallel.

### Complex Chain
```rust
"requirements -> design -> (implementation, tests) -> review -> deploy"
```
Multi-stage workflow with parallel implementation+tests phase.

---

## Appendix B: Error Message Examples

### Parse Error
```
Error: Invalid flow expression
  --> "agent1 - > agent2"
           ^
  Expected: '->' found: '- '
  
  Suggestion: Remove space between '-' and '>'
  Corrected: "agent1 -> agent2"
```

### Validation Error
```
Error: Agent not found in registry
  Flow: "researcher -> unknown_agent -> writer"
                       ^^^^^^^^^^^^^
  
  Available agents: researcher, writer, editor, reviewer
  
  Suggestion: Did you mean 'writer'?
```

### Execution Error (Fail-Fast)
```
Error: Agent execution failed in Maneuver 'ResearchPipeline'
  Agent: 'analyzer'
  Step: 3 of 5
  Error: LLM timeout after 60s
  
  Execution trace:
    ✓ researcher (2.3s)
    ✓ processor (1.8s)
    ✗ analyzer (timeout)
    ⊘ writer (not started)
    ⊘ reviewer (not started)
  
  Suggestion: Increase timeout_seconds in ManeuverConfig
```

---

## Appendix C: Configuration Example

```yaml
# examples/cli_configs/maneuver_document_processing.yaml
type: maneuver
name: "DocumentProcessingPipeline"

# Flow expression defining the workflow
flow: "extractor -> (summarizer, analyzer), translator -> reviewer"

# Execution configuration
config:
  # How to handle errors
  error_strategy: continue_parallel  # Options: fail_fast, continue_parallel, ignore_errors
  
  # Timeout settings
  timeout_seconds: 600
  agent_timeout_seconds: 120
  
  # Output handling
  pass_output_as_input: true
  output_format: concatenate  # Options: concatenate, json_array
  
  # Observability
  collect_timing_metrics: true
  capture_intermediate_outputs: true

# Agent definitions
agents:
  - name: extractor
    inline:
      system_prompt: "Extract key information from documents"
      model: "gpt-4o"
      temperature: 0.3
      
  - name: summarizer
    inline:
      system_prompt: "Create concise summaries"
      model: "gpt-4o-mini"
      temperature: 0.5
      
  - name: analyzer
    inline:
      system_prompt: "Perform deep analysis"
      model: "gpt-4o"
      temperature: 0.7
      
  - name: translator
    inline:
      system_prompt: "Translate to Spanish"
      model: "gpt-4o"
      temperature: 0.3
      
  - name: reviewer
    inline:
      system_prompt: "Review and validate all outputs"
      model: "gpt-4o"
      temperature: 0.4
```

---

## Revision History

| Version | Date | Author | Changes |
|---------|------|--------|---------|
| 1.0 | 2026-02-05 | GitHub Copilot | Initial PRD based on Epic 17 and user clarifications |

