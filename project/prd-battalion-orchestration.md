# Product Requirements Document: Battalion Orchestration System

**Epic:** Epic 4 - Battalion Orchestration  
**Version:** 1.0  
**Date:** January 24, 2026  
**Status:** Ready for Implementation  
**Priority:** Critical  
**Dependencies:** Epic 1 (Paladin Domain Foundation), Epic 2 (Garrison Memory System)

---

## 1. Introduction/Overview

The Battalion Orchestration System enables multi-Paladin coordination through structured orchestration patterns. This feature provides the core capability for developers to compose multiple autonomous AI agents (Paladins) into coordinated teams (Battalions) that can solve complex problems requiring sequential, parallel, graph-based, or hierarchical execution patterns.

**Problem Statement:**  
While individual Paladins can handle single-agent tasks, real-world applications require multiple agents working together in coordinated patterns. Without a robust orchestration framework, developers must manually manage agent coordination, handle inter-agent communication, manage failures, and implement retry logic—all of which are error-prone and time-consuming.

**Solution:**  
Battalion Orchestration provides a production-ready, enterprise-grade multi-agent coordination framework with four fundamental patterns:
- **Formation** - Sequential execution where output flows from one Paladin to the next
- **Phalanx** - Concurrent execution where multiple Paladins process the same input in parallel
- **Campaign** - Graph-based orchestration using Directed Acyclic Graphs (DAG) for complex workflows
- **Chain of Command** - Hierarchical delegation where a commander Paladin coordinates specialist Paladins

---

## 2. Goals

### Primary Goals
1. **Phased Implementation:** Deliver Formation and Phalanx patterns first (Phase 1), followed by Campaign and Chain of Command (Phase 2)
2. **Production-Ready Reliability:** Implement comprehensive retry logic, rollback capabilities, and recovery mechanisms for mission-critical applications
3. **High Performance:** Support efficient parallel execution of 10+ concurrent Paladins with sub-second orchestration overhead
4. **Observability:** Provide status queries and execution logging for debugging and monitoring
5. **Test Coverage:** Achieve ≥80% unit test coverage with comprehensive integration and load/stress testing

### Secondary Goals
- Maintain hexagonal architecture with clean domain/application/infrastructure separation
- Follow Domain-Driven Design principles with clear bounded contexts
- Enable extensibility for future Battalion patterns
- Provide fluent builder APIs for developer experience

---

## 3. User Stories

### Phase 1: Formation & Phalanx

**US-4.1: Formation Sequential Execution**
- **As a developer**, I want to create a Formation so that multiple Paladins execute in sequence with output passing from one to the next
- **Acceptance Criteria:**
  - Formation accepts a list of Paladins in execution order
  - Output from Paladin N becomes input to Paladin N+1
  - Execution stops on first error (FailFast) or continues based on error strategy
  - All intermediate results are captured and returned

**US-4.2: Phalanx Concurrent Execution**
- **As a developer**, I want to create a Phalanx so that multiple Paladins process the same input concurrently
- **Acceptance Criteria:**
  - Phalanx accepts multiple Paladins and an aggregation strategy
  - All Paladins receive the same input simultaneously
  - Results are aggregated according to strategy (CollectAll, FirstSuccess, Majority, Custom)
  - Supports 10+ concurrent Paladins efficiently

**US-4.3: Comprehensive Error Handling**
- **As a developer**, I want configurable error handling strategies so that Battalions can recover from failures gracefully
- **Acceptance Criteria:**
  - FailFast: Stop immediately on first error
  - ContinueOnError: Continue execution despite errors, report all at end
  - RetryThenContinue: Retry failed operations N times before continuing
  - Support for custom retry policies with exponential backoff

**US-4.4: Battalion Status Monitoring**
- **As a developer**, I want to query Battalion execution status so that I can monitor progress and debug issues
- **Acceptance Criteria:**
  - Status query returns current state: Idle, Running, Paused, Completed, Failed
  - Provides list of completed/failed/pending Paladins
  - Includes timing information and error details
  - Supports async status polling

### Phase 2: Campaign & Chain of Command

**US-4.5: Campaign Graph Orchestration**
- **As a developer**, I want to create a Campaign so that Paladins follow a directed graph workflow with conditional routing
- **Acceptance Criteria:**
  - Campaign uses DAG structure with nodes (Paladins) and edges (conditions)
  - Supports conditional execution based on previous results
  - Validates graph for cycles before execution
  - Handles multiple entry points and fan-out/fan-in patterns

**US-4.6: Chain of Command Hierarchical Delegation**
- **As a developer**, I want to create a Chain of Command so that a commander Paladin delegates to specialists
- **Acceptance Criteria:**
  - Commander analyzes task and routes to appropriate specialists
  - Supports multiple delegation strategies (Automatic, Broadcast, RoundRobin, Custom)
  - Commander aggregates specialist results into final response
  - Handles specialist failures with fallback logic

---

## 4. Functional Requirements

### Core Battalion Infrastructure (All Patterns)

**FR-4.1: Battalion Configuration**
- System MUST support `BattalionConfig` with: name, description, timeout_seconds, retry_policy, error_strategy, metadata_output_dir
- System MUST validate configuration before execution
- System MUST serialize/deserialize Battalion configurations

**FR-4.2: Battalion Result Structure**
- System MUST return `BattalionResult` containing: battalion_id, battalion_name, timestamps, final_output, individual paladin_results, status
- System MUST capture all intermediate Paladin results
- System MUST include execution timing for each Paladin and overall Battalion

**FR-4.3: Error Strategy Implementation**
- System MUST implement FailFast: stop immediately on first error, return error result
- System MUST implement ContinueOnError: continue execution, collect all errors, report at end
- System MUST implement RetryThenContinue: retry failed operations up to configured attempts with exponential backoff

**FR-4.4: Retry Policy**
- System MUST support configurable retry attempts (default: 3)
- System MUST support configurable retry delays with exponential backoff
- System MUST support jitter to prevent thundering herd
- System MUST log all retry attempts

### Phase 1: Formation Requirements

**FR-4.5: Formation Construction**
- System MUST accept ordered list of Paladin instances
- System MUST validate at least 2 Paladins are provided
- System MUST support optional shared context injected into all Paladin prompts

**FR-4.6: Formation Execution**
- System MUST execute Paladins sequentially in order
- System MUST pass output of Paladin N as input to Paladin N+1
- System MUST respect timeout_seconds for total execution time
- System MUST respect error_strategy configuration

**FR-4.7: Formation Output**
- System MUST return final output from last Paladin
- System MUST include all intermediate Paladin outputs in result
- System MUST preserve execution order in results

### Phase 1: Phalanx Requirements

**FR-4.8: Phalanx Construction**
- System MUST accept list of Paladin instances (≥2)
- System MUST accept AggregationStrategy: CollectAll, FirstSuccess, Majority, or Custom
- System MUST support Custom aggregation via user-provided function

**FR-4.9: Phalanx Concurrent Execution**
- System MUST execute all Paladins concurrently using Tokio runtime
- System MUST support ≥10 concurrent Paladin executions
- System MUST complete with <1 second orchestration overhead for typical workloads
- System MUST handle partial failures based on error_strategy

**FR-4.10: Phalanx Aggregation**
- CollectAll: MUST return all Paladin results in array
- FirstSuccess: MUST return first successfully completed result, cancel remaining
- Majority: MUST analyze results, return most common output (requires ≥3 Paladins)
- Custom: MUST invoke user-provided aggregation function with all results

### Phase 2: Campaign Requirements

**FR-4.11: Campaign Graph Construction**
- System MUST support DAG structure with nodes (Paladins) and edges (CampaignEdge)
- System MUST allow adding Paladins and edges programmatically
- System MUST validate graph is acyclic before execution
- System MUST validate all edges have valid source/target nodes

**FR-4.12: Campaign Edge Conditions**
- System MUST support EdgeCondition types: Always, Contains(String), Regex(String), Custom(Fn)
- System MUST evaluate edge conditions based on source Paladin output
- System MUST support optional output transformation functions on edges

**FR-4.13: Campaign Execution**
- System MUST execute Paladins respecting dependency order
- System MUST support multiple entry points
- System MUST handle fan-out (1 → N) and fan-in (N → 1) patterns
- System MUST execute independent branches concurrently

### Phase 2: Chain of Command Requirements

**FR-4.14: Chain of Command Construction**
- System MUST accept one commander Paladin and ≥1 specialist Paladins
- System MUST accept DelegationStrategy: Automatic, Broadcast, RoundRobin, Custom

**FR-4.15: Chain of Command Execution**
- Automatic: Commander MUST analyze input, select appropriate specialists
- Broadcast: System MUST delegate to all specialists concurrently
- RoundRobin: System MUST cycle through specialists sequentially
- Custom: System MUST invoke user-provided delegation function

**FR-4.16: Chain of Command Aggregation**
- System MUST have Commander aggregate specialist results
- System MUST inject specialist outputs into Commander's context
- System MUST return Commander's final synthesized response

### Observability & Monitoring

**FR-4.17: Status Queries**
- System MUST provide `status()` method returning current Battalion state
- System MUST support states: Idle, Running, Paused, Completed, Failed, Cancelled
- System MUST include progress information (completed/total Paladins)
- System MUST include timing information (elapsed, estimated remaining)

**FR-4.18: Execution Logging**
- System MUST log Battalion start/completion with metadata
- System MUST log each Paladin execution start/completion
- System MUST log all errors and retry attempts
- System MUST support structured logging (JSON) for observability tools

**FR-4.19: Cancellation Support**
- System MUST provide `cancel()` method to stop ongoing execution
- System MUST gracefully terminate in-progress Paladins
- System MUST return partial results on cancellation
- System MUST mark status as Cancelled

---

## 5. Non-Goals (Out of Scope)

### Explicitly Out of Scope for Initial Release

1. **Distributed Battalion Execution**
   - No cross-process or cross-machine Battalion execution
   - No distributed transaction support
   - No network-based Paladin coordination

2. **Advanced Scheduling**
   - No cron-based Battalion scheduling
   - No event-triggered Battalion execution
   - No conditional Battalion chaining

3. **Visual Workflow Designer**
   - No GUI for designing Campaign graphs
   - No visual Battalion debugging interface
   - No drag-and-drop orchestration builder

4. **State Persistence (Covered in Epic 7)**
   - No automatic Battalion state checkpointing
   - No resume-from-checkpoint capability
   - No long-running Battalion recovery

5. **Advanced Aggregation**
   - No ML-based result consensus
   - No semantic similarity aggregation for Phalanx
   - No advanced voting mechanisms beyond simple majority

6. **Resource Management**
   - No CPU/memory quotas per Paladin
   - No priority-based scheduling
   - No rate limiting per Paladin

### May Be Considered for Future Iterations

- Dynamic Battalion reconfiguration during execution
- Nested Battalions (Battalions containing other Battalions)
- A* or heuristic-based Campaign path optimization
- Battalion templates and presets
- Integration with workflow engines (Temporal, Conductor)

---

## 6. Design Considerations

### Architectural Patterns

**Hexagonal Architecture Compliance:**
- **Domain Layer** (`core/platform/container/battalion/`): Pure business logic, no external dependencies
  - `mod.rs`: Battalion base types, config, results
  - `formation.rs`: Sequential pattern domain
  - `phalanx.rs`: Concurrent pattern domain
  - `campaign.rs`: Graph pattern domain (Phase 2)
  - `chain_of_command.rs`: Hierarchical pattern domain (Phase 2)

- **Application Layer** (`application/`):
  - `ports/output/battalion_port.rs`: BattalionPort trait defining execution interface
  - `use_cases/battalion/formation_service.rs`: Formation execution orchestration
  - `use_cases/battalion/phalanx_service.rs`: Phalanx execution orchestration
  - `use_cases/battalion/campaign_service.rs`: Campaign execution orchestration (Phase 2)
  - `use_cases/battalion/chain_of_command_service.rs`: Chain execution orchestration (Phase 2)

- **Infrastructure Layer**: Adapters for persistence, monitoring (future)

### Domain-Driven Design

**Ubiquitous Language:**
- Battalion, Formation, Phalanx, Campaign, Chain of Command maintain Medieval Military theme
- Clear bounded contexts: Battalion domain vs Paladin domain vs Garrison domain

**Aggregates:**
- Battalion is aggregate root containing Paladins (by reference)
- Battalion owns its configuration and result collection
- Paladins remain independent entities referenced by Battalion

### Concurrency Design

**Phase 1 - Phalanx Concurrency:**
- Use `tokio::spawn` for each Paladin in Phalanx
- Use `futures::future::join_all` for result collection
- Support cancellation via `tokio::select!` and cancel tokens
- Implement semaphore-based concurrency limiting (default: 10)

**Phase 2 - Campaign Concurrency:**
- Use topological sort for execution ordering
- Execute independent branches in parallel
- Use `tokio::sync::mpsc` for inter-node communication

### Error Handling Strategy

**Retry Logic:**
```rust
RetryPolicy {
    max_attempts: 3,
    base_delay: Duration::from_millis(100),
    max_delay: Duration::from_secs(10),
    exponential_backoff: true,
    jitter: true,
}
```

**Error Propagation:**
- Use `BattalionError` enum with variants for each pattern
- Convert `PaladinError` using `From` trait
- Preserve error context through execution stack

### Performance Considerations

**Target Benchmarks:**
- Orchestration overhead: <1 second for typical workloads
- Support 10+ concurrent Paladins without degradation
- Memory: <50MB overhead per Battalion instance
- CPU: Linear scaling with Paladin count

**Optimization Strategies:**
- Use `Arc<Paladin>` to avoid cloning during concurrent execution
- Stream results as they complete (don't wait for all)
- Lazy evaluation where possible
- Connection pooling for shared resources

---

## 7. Technical Considerations

### Dependencies

**Required Crates:**
- `tokio` (≥1.35): Async runtime, required for concurrent execution
- `futures` (≥0.3): Future combinators for Phalanx
- `petgraph` (≥0.6): Graph data structure for Campaign DAG
- `uuid` (≥1.6): Battalion and execution IDs
- `chrono` (≥0.4): Timestamp tracking
- `serde` (≥1.0): Serialization for configs and results
- `thiserror` (≥1.0): Error type definitions

**Feature Flags:**
- `battalion-metrics`: Enable Prometheus metrics collection
- `battalion-tracing`: Enable distributed tracing integration

### Integration Points

**Epic 1 - Paladin Domain:**
- Battalion executes Paladins via `PaladinExecutionService`
- Uses `PaladinPort` trait for abstraction
- Respects Paladin timeout and retry configurations

**Epic 2 - Garrison Memory:**
- Formation passes context through Garrison entries
- Shared context in Formation stored in Garrison
- Chain of Command commander accesses specialist results from Garrison

**Epic 3 - Arsenal Tools:**
- Individual Paladins within Battalion access Arsenal
- Tool calls tracked per-Paladin in Battalion results
- No special Battalion-level tool coordination (out of scope)

### Database Schema (Future - Epic 7)

When state persistence is added, Battalion execution requires:
```sql
CREATE TABLE battalion_executions (
    id UUID PRIMARY KEY,
    battalion_name VARCHAR(255),
    battalion_type VARCHAR(50), -- formation, phalanx, campaign, chain_of_command
    status VARCHAR(50),
    started_at TIMESTAMP,
    completed_at TIMESTAMP,
    config JSONB,
    result JSONB
);

CREATE TABLE battalion_paladin_executions (
    id UUID PRIMARY KEY,
    battalion_execution_id UUID REFERENCES battalion_executions(id),
    paladin_id UUID,
    execution_order INT,
    started_at TIMESTAMP,
    completed_at TIMESTAMP,
    result JSONB
);
```

### Configuration

**config.yml Addition:**
```yaml
battalion:
  default_timeout_seconds: 300
  default_retry_attempts: 3
  default_retry_base_delay_ms: 100
  default_retry_max_delay_ms: 10000
  max_concurrent_paladins: 10
  enable_metrics: true
  enable_tracing: false
  metadata_output_dir: "./battalion_logs"
```

### Security Considerations

- Battalion execution inherits Paladin security context
- No cross-Battalion information leakage
- Sanitize error messages to avoid exposing sensitive data
- Rate limiting to prevent Battalion-based DoS

---

## 8. Success Metrics

### Performance Metrics

1. **Orchestration Overhead:** <1 second for typical Formation/Phalanx (baseline: single Paladin execution time)
2. **Concurrent Execution:** Successfully execute 10 Paladins concurrently in Phalanx without degradation
3. **Throughput:** Process ≥100 Battalion executions/minute with sustained load
4. **Latency P95:** 95th percentile Battalion completion time ≤5 seconds (excluding Paladin LLM time)

### Reliability Metrics

1. **Error Recovery:** ≥95% of transient failures recovered via retry logic
2. **Fault Tolerance:** Battalion continues execution despite single Paladin failure (ContinueOnError mode)
3. **Data Integrity:** 100% of Battalion results correctly capture all Paladin outputs

### Quality Metrics

1. **Test Coverage:** ≥80% unit test coverage across all Battalion modules
2. **Integration Tests:** 100% of user stories covered by integration tests
3. **Load Tests:** Pass stress tests with 50 concurrent Battalions, 10 Paladins each
4. **Code Quality:** Zero clippy warnings, 100% rustdoc coverage for public APIs

### Developer Experience Metrics

1. **API Clarity:** Junior developer can create Formation/Phalanx in <30 minutes (qualitative)
2. **Documentation:** 100% of public types/methods documented with examples
3. **Error Messages:** Actionable error messages with clear remediation steps

### Adoption Metrics (Post-Release)

1. **Usage:** ≥50% of Paladin deployments utilize Battalion orchestration (3 months post-release)
2. **Pattern Distribution:** Measure relative usage of Formation vs Phalanx vs Campaign vs Chain of Command
3. **Community Feedback:** Collect developer feedback on API design and missing features

---

## 9. Open Questions

### Technical Questions

1. **Q: Should Battalion support dynamic Paladin addition during execution?**
   - Context: Useful for adaptive workflows but adds complexity
   - Decision needed by: Epic kickoff
   - Owner: Tech Lead

2. **Q: How should Battalion handle Paladin timeout vs Battalion timeout?**
   - Option A: Battalion timeout includes all Paladin timeouts (cumulative)
   - Option B: Battalion timeout is independent, can override Paladin timeouts
   - Decision needed by: Design review
   - Owner: Architect

3. **Q: Should Phalanx support streaming aggregation (results as they arrive)?**
   - Context: Better UX but complicates aggregation strategies
   - Decision needed by: Phase 1 implementation
   - Owner: Developer

4. **Q: How to handle circular dependencies in Campaign graph validation?**
   - Context: Petgraph provides cycle detection, but error messages need to be clear
   - Decision needed by: Phase 2 start
   - Owner: Developer

### Product Questions

5. **Q: Should Battalion provide progress callbacks for long-running executions?**
   - Context: Useful for UI integration but adds API complexity
   - Decision needed by: Phase 1 mid-point
   - Owner: Product Manager

6. **Q: What metadata should be included in BattalionResult for debugging?**
   - Current: timestamps, status, outputs
   - Potential additions: token counts, cost estimates, tool calls
   - Decision needed by: Before Epic completion
   - Owner: Product Manager + Tech Lead

7. **Q: Should Campaign support conditional branching (if/else logic)?**
   - Context: EdgeCondition provides basic conditions, but full if/else may be needed
   - Decision needed by: Phase 2 design
   - Owner: Architect

### Process Questions

8. **Q: What is the rollout strategy for Phase 1 vs Phase 2?**
   - Option A: Release Phase 1 separately, gather feedback, then Phase 2
   - Option B: Complete both phases, release together
   - Decision needed by: Sprint planning
   - Owner: Project Manager

9. **Q: Should we create example applications demonstrating each pattern?**
   - Context: Helps adoption but adds documentation effort
   - Decision needed by: Before Epic completion
   - Owner: Tech Writer + Developer

---

## Appendix A: Phase Implementation Plan

### Phase 1: Formation & Phalanx (Weeks 1-3)

**Week 1: Domain & Application Layer**
- Implement `BattalionConfig`, `BattalionResult`, error types
- Implement Formation domain entity and builder
- Implement Phalanx domain entity and builder
- Implement BattalionPort trait
- Unit tests: ≥80% coverage

**Week 2: Execution Services**
- Implement FormationExecutionService with retry logic
- Implement PhalanxExecutionService with concurrency
- Implement AggregationStrategy variants
- Integration tests with mock Paladins

**Week 3: Error Handling & Observability**
- Implement comprehensive error strategies
- Implement status queries and cancellation
- Implement execution logging
- Load/stress tests: 10 concurrent Paladins, 50 concurrent Battalions

### Phase 2: Campaign & Chain of Command (Weeks 4-5)

**Week 4: Campaign Implementation**
- Implement Campaign domain with petgraph
- Implement CampaignEdge conditions
- Implement CampaignExecutionService with topological sort
- Graph validation and cycle detection

**Week 5: Chain of Command Implementation**
- Implement Chain of Command domain
- Implement DelegationStrategy variants
- Implement delegation and aggregation logic
- Integration tests and documentation

---

## Appendix B: Example Usage (Target API)

### Formation Example
```rust
let formation = Formation::new("research_pipeline", vec![
    researcher_paladin,
    summarizer_paladin,
    writer_paladin,
])
.with_config(BattalionConfig {
    timeout_seconds: 300,
    error_strategy: ErrorStrategy::RetryThenContinue,
    ..Default::default()
});

let result = formation_service.execute(&formation, "Analyze market trends in AI").await?;
```

### Phalanx Example
```rust
let phalanx = Phalanx::new("consensus_analysis", vec![
    analyst_paladin_1,
    analyst_paladin_2,
    analyst_paladin_3,
])
.with_aggregation(AggregationStrategy::Majority);

let result = phalanx_service.execute(&phalanx, "Is this claim accurate?").await?;
```

---

**Document Control:**
- Created: January 24, 2026
- Last Updated: January 24, 2026
- Next Review: Epic 4 Kickoff Meeting
- Approval Required: Technical Lead, Product Manager
