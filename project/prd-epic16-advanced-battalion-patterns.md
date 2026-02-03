# Product Requirements Document: Epic 16 - Advanced Battalion Patterns

## Introduction/Overview

This PRD defines the requirements for implementing two advanced multi-agent orchestration patterns in the Paladin framework: **Council** (conversational collaboration) and **Grove** (tree-based agent routing). These patterns extend the existing Battalion system to support expert panel discussions and intelligent task routing to specialized agents.

**Problem Statement:** Current Battalion patterns (Formation, Phalanx, Campaign, ChainOfCommand, Conclave) lack support for:
1. Multi-agent conversational collaboration where agents discuss and reach consensus
2. Automatic routing of tasks to best-fit specialist agents based on expertise

**Solution:** Implement Council for group discussions and Grove for intelligent agent routing, both integrated with the existing Commander orchestration system.

---

## Goals

1. **Council Pattern:** Enable multiple Paladins to engage in turn-based conversations to collaboratively solve complex problems
2. **Grove Pattern:** Enable automatic routing of tasks to the most appropriate specialist Paladin based on semantic matching
3. **Integration:** Seamlessly integrate both patterns into the Commander system for unified orchestration
4. **Extensibility:** Design patterns that can be extended with additional strategies and configurations
5. **Performance:** Ensure both patterns maintain enterprise-grade performance and reliability
6. **Observability:** Provide comprehensive logging and execution tracking for both patterns

---

## User Stories

### Primary User Stories

**US-16.1: Council Domain Model**
- **As a** framework developer
- **I want** domain models for conversational multi-agent collaboration
- **So that** agents can discuss problems and reach consensus through structured dialogue
- **Acceptance:** Council struct with participants, moderator, turn-taking strategies, and conversation tracking

**US-16.2: Council Execution Service**
- **As a** developer
- **I want** to execute group discussions between Paladins
- **So that** I can leverage collective intelligence for complex problem-solving
- **Acceptance:** Service manages conversation flow, tracks history, implements turn-taking, detects termination conditions

**US-16.3: Grove Domain Model**
- **As a** framework developer
- **I want** domain models for tree-based agent routing
- **So that** tasks can be routed to agents with matching expertise
- **Acceptance:** Grove struct with trees of specialized agents, expertise definitions, routing strategies

**US-16.4: Grove Execution Service**
- **As a** developer
- **I want** automatic routing to best-fit agents
- **So that** tasks are handled by the most qualified specialist
- **Acceptance:** Service calculates similarity, selects best agent, executes task, returns routing decision

**US-16.5: Commander Integration**
- **As a** developer
- **I want** Commander to support Council and Grove patterns
- **So that** I have a unified interface for all orchestration patterns
- **Acceptance:** Commander recognizes Council/Grove strategies, auto-routing works, CLI support added

---

## Functional Requirements

### Council Pattern

**FR-1.1: Council Data Structure**
- MUST define `Council` struct in `src/core/platform/container/battalion/council.rs`
- MUST include: name, list of participant Paladins, optional moderator Paladin, configuration
- MUST define `CouncilConfig` with: max_rounds, turn_strategy, termination_condition, include_history flag
- MUST define `CouncilMessage` with: speaker name, content, round number, timestamp

**FR-1.2: Turn-Taking Strategies** (Initial Implementation: RoundRobin + ModeratorDirected)
- MUST implement `TurnStrategy::RoundRobin` - participants take turns in sequence
- MUST implement `TurnStrategy::ModeratorDirected` - moderator decides who speaks next
- SHOULD prepare enum structure for future strategies: `Random`, `VoluntaryWithTimeout`
- MUST handle edge cases: speaker unavailable, moderator offline

**FR-1.3: Termination Conditions**
- MUST support `TerminationCondition::MaxRounds` - stop after N rounds
- MUST support `TerminationCondition::ModeratorDecision` - moderator declares end
- SHOULD support `TerminationCondition::Consensus` - detect agreement keywords
- SHOULD support `TerminationCondition::Keyword(String)` - custom keyword triggers end

**FR-1.4: Council Execution Service**
- MUST implement `CouncilExecutionService` in `src/application/use_cases/battalion/council_service.rs`
- MUST provide `convene(council, topic)` method to start discussions
- MUST track conversation history as ordered list of `CouncilMessage`
- MUST implement turn-taking logic according to selected strategy
- MUST evaluate termination condition after each turn
- MUST return `CouncilResult` with: transcript, conclusion, rounds_completed, termination_reason

**FR-1.5: Council-Garrison Integration**
- MUST store conversation history in Garrison for context continuity
- MUST retrieve conversation history for follow-up discussions
- MUST support conversation branching (multiple councils on same topic)

### Grove Pattern

**FR-2.1: Grove Data Structure**
- MUST define `Grove` struct in `src/core/platform/container/battalion/grove.rs`
- MUST include: name, list of Trees, GroveConfig
- MUST define `Tree` struct with: name, list of TreeAgents
- MUST define `TreeAgent` struct with: Paladin reference, expertise_keywords, optional expertise_embedding

**FR-2.2: Routing Strategies** (All Three Strategies)
- MUST implement `RoutingStrategy::KeywordMatch` - match task keywords to agent expertise_keywords (default)
- MUST implement `RoutingStrategy::SemanticSimilarity` - use embeddings for similarity scoring
- MUST implement `RoutingStrategy::LlmRouting` - use LLM to decide best agent with reasoning
- MUST define fallback behavior when no good match found (use fallback_tree if configured)
- MUST support configurable similarity_threshold for SemanticSimilarity

**FR-2.3: Grove Configuration**
- MUST define `GroveConfig` with: routing_strategy, optional fallback_tree name, similarity_threshold
- MUST validate configuration on Grove creation
- MUST provide sensible defaults: KeywordMatch strategy, threshold 0.7

**FR-2.4: Grove Execution Service**
- MUST implement `GroveExecutionService` in `src/application/use_cases/battalion/grove_service.rs`
- MUST provide `execute(grove, task)` method
- MUST implement `route_task(grove, task)` internal method for agent selection
- MUST return `GroveResult` with: selected agent, routing decision, execution result
- MUST define `RoutingDecision` with: selected_tree, selected_agent, confidence score, reasoning

**FR-2.5: Grove Routing Logic**
- KeywordMatch: MUST count matching keywords between task and agent expertise
- SemanticSimilarity: MUST calculate cosine similarity between task and agent embeddings
- LlmRouting: MUST send task + agent descriptions to LLM for selection with JSON response
- MUST select agent with highest score/confidence
- MUST use fallback_tree if no agent meets threshold

**FR-2.6: Grove-Arsenal Integration**
- SHOULD allow TreeAgents to specify required Arsenal tools
- SHOULD validate agent has access to required tools before routing
- SHOULD include tool availability in routing decision

### Commander Integration

**FR-3.1: Strategy Enum Extensions**
- MUST add `BattalionStrategy::Council` variant
- MUST add `BattalionStrategy::Grove` variant
- MUST update `BattalionStrategy::Auto` logic to consider Council/Grove based on input

**FR-3.2: Commander Routing**
- MUST route Council requests to `CouncilExecutionService`
- MUST route Grove requests to `GroveExecutionService`
- MUST support explicit strategy selection via config
- MUST support auto-detection: "discuss", "debate", "collaborate" → Council; "expert", "specialist", "route" → Grove

**FR-3.3: Commander CLI Support**
- MUST add `--strategy council` flag support
- MUST add `--strategy grove` flag support
- MUST add example configs in `examples/cli_configs/council_*.yml`
- MUST add example configs in `examples/cli_configs/grove_*.yml`

### Error Handling

**FR-4.1: Council Errors**
- MUST handle: empty participants list, missing moderator when required, invalid turn strategy config
- MUST provide clear error messages for termination failures
- MUST handle participant execution failures gracefully (skip to next speaker)

**FR-4.2: Grove Errors**
- MUST handle: empty trees, no agents in grove, invalid routing strategy
- MUST handle: missing embeddings when SemanticSimilarity selected
- MUST handle: LLM routing failures with fallback to KeywordMatch
- MUST provide routing decision even on failure (with reasoning)

### Testing Requirements

**FR-5.1: Unit Tests**
- MUST test Council turn-taking logic for RoundRobin
- MUST test Council turn-taking logic for ModeratorDirected
- MUST test all Council termination conditions
- MUST test Grove routing for all three strategies
- MUST test Grove fallback behavior
- MUST test error handling for both patterns

**FR-5.2: Integration Tests**
- MUST test Council with real Paladins and LLM calls
- MUST test Grove with embedding service integration
- MUST test Commander routing to Council/Grove
- MUST test Garrison integration with Council history
- MUST test concurrent Council and Grove execution

**FR-5.3: Example Applications**
- MUST create `examples/council_discussion.rs` - expert panel solving a problem
- MUST create `examples/grove_routing.rs` - task routing to specialists
- MUST create `examples/commander_council.rs` - Commander orchestrating Council
- MUST create `examples/commander_grove.rs` - Commander orchestrating Grove

---

## Non-Goals (Out of Scope)

**NG-1:** Dynamic addition/removal of participants during Council discussion (future enhancement)

**NG-2:** Real-time streaming of Council conversations to UI (future feature)

**NG-3:** Grove learning from routing decisions to improve future matches (future ML feature)

**NG-4:** Council voting mechanisms beyond consensus detection (future enhancement)

**NG-5:** Grove support for hierarchical tree structures beyond flat trees (future feature)

**NG-6:** Implementing `TurnStrategy::Random` and `TurnStrategy::VoluntaryWithTimeout` (deferred to future iteration)

**NG-7:** Cross-council communication (councils discussing with other councils)

**NG-8:** Grove agent load balancing based on current workload

---

## Design Considerations

### Architecture Alignment

**Hexagonal Architecture:**
- **Core Layer:** Council and Grove domain models in `src/core/platform/container/battalion/`
- **Application Layer:** Execution services in `src/application/use_cases/battalion/`
- **Application Layer:** Port definitions in `src/application/ports/output/battalion_port.rs`
- **Infrastructure Layer:** No new adapters needed (uses existing LlmPort, GarrisonPort, EmbeddingPort)

**Domain-Driven Design:**
- Council is an Aggregate containing CouncilConfig and CouncilMessages
- Grove is an Aggregate containing Trees and TreeAgents
- Use ubiquitous language: Council, Grove, Tree, Moderator, Routing

### Data Flow

**Council Execution Flow:**
```
1. Commander receives Council strategy request
2. CouncilExecutionService.convene() called with topic
3. For each round until termination:
   a. Determine next speaker based on TurnStrategy
   b. Execute speaker Paladin with topic + history
   c. Record CouncilMessage
   d. Store in Garrison
   e. Check termination condition
4. Return CouncilResult with transcript
```

**Grove Execution Flow:**
```
1. Commander receives Grove strategy request  
2. GroveExecutionService.execute() called with task
3. Route task:
   a. Calculate task-to-agent similarity (all strategies)
   b. Select highest scoring agent above threshold
   c. Fallback if no match
4. Execute selected Paladin
5. Return GroveResult with routing decision + result
```

### UI/UX Considerations

**Council Output:**
- Display conversation transcript with speaker names
- Highlight moderator decisions
- Show rounds completed and termination reason
- Format as dialogue for readability

**Grove Output:**
- Display routing decision with confidence score
- Show reasoning for agent selection
- Include tree and agent names
- Display fallback notifications if occurred

---

## Technical Considerations

### Dependencies

**Existing Systems:**
- `PaladinPort` - for executing individual agents (Epic 4)
- `GarrisonPort` - for conversation history storage
- `EmbeddingPort` - for SemanticSimilarity routing (may need to create if not exists)
- `LlmPort` - for LlmRouting strategy
- `Commander` - for unified orchestration (Epic 15)

**External Crates:**
- No new external dependencies required
- Use existing `tokio` for async execution
- Use existing `serde` for serialization

### Configuration Schema

**Council Configuration:**
```yaml
battalion:
  strategy: council
  council:
    max_rounds: 5
    turn_strategy: round_robin  # or moderator_directed
    termination_condition: max_rounds  # or consensus, moderator_decision, keyword
    include_history: true
    participants:
      - name: "Legal Expert"
        system_prompt: "You are a legal expert..."
      - name: "Technical Expert"  
        system_prompt: "You are a technical expert..."
    moderator:
      name: "Facilitator"
      system_prompt: "You are a discussion moderator..."
```

**Grove Configuration:**
```yaml
battalion:
  strategy: grove
  grove:
    routing_strategy: keyword_match  # or semantic_similarity, llm_routing
    similarity_threshold: 0.7
    fallback_tree: "Generalists"
    trees:
      - name: "Security Experts"
        agents:
          - name: "OWASP Specialist"
            expertise_keywords: ["security", "owasp", "vulnerabilities", "xss", "sql injection"]
          - name: "Cryptography Expert"
            expertise_keywords: ["encryption", "crypto", "tls", "certificates"]
      - name: "Generalists"
        agents:
          - name: "General Developer"
            expertise_keywords: ["programming", "development", "coding"]
```

### Performance Considerations

**Council:**
- Each round requires N Paladin executions (N = participants)
- Max execution time = max_rounds × participants × avg_paladin_time
- Example: 5 rounds × 3 participants × 5s = 75s maximum
- Implement timeout per speaker to prevent blocking

**Grove:**
- Routing calculation is fast for KeywordMatch (milliseconds)
- SemanticSimilarity requires embedding computation (1-2s)
- LlmRouting requires LLM call (2-5s)
- Task execution time is same as single Paladin

### State Persistence

**Council State:**
- Store conversation history in Garrison
- Store CouncilResult in Citadel for recovery
- Include checkpoint after each round for interruption recovery

**Grove State:**
- Store routing decisions for audit trail
- Store agent performance metrics for future optimization
- No intermediate state needed (single execution)

### Concurrency

**Council:**
- Sequential execution within a Council (by design)
- Multiple Councils can run concurrently
- Thread-safe conversation history access

**Grove:**
- Parallel routing calculation for all agents (when possible)
- Single agent execution after routing
- Multiple Grove executions can run concurrently

---

## Success Metrics

**SM-1: Functional Completeness**
- ✅ All 5 user stories (US-16.1 through US-16.5) completed
- ✅ All functional requirements (FR-1 through FR-5) implemented
- ✅ All tests passing with ≥80% unit test coverage

**SM-2: Council Pattern**
- ✅ Successful multi-agent discussion with coherent conversation flow
- ✅ Both RoundRobin and ModeratorDirected strategies functional
- ✅ Conversation history correctly stored in Garrison
- ✅ Termination conditions trigger correctly

**SM-3: Grove Pattern**
- ✅ Accurate routing to specialist agents (≥85% routing accuracy in test cases)
- ✅ All three routing strategies functional
- ✅ Fallback behavior works when no good match
- ✅ Routing decision confidence scores meaningful

**SM-4: Integration**
- ✅ Commander correctly routes to Council/Grove
- ✅ Auto-strategy detection works for keywords
- ✅ CLI examples run successfully
- ✅ No breaking changes to existing Battalion patterns

**SM-5: Performance**
- ✅ Grove routing completes in <3s (including LlmRouting)
- ✅ Council discussions don't exceed reasonable timeouts
- ✅ No memory leaks in long-running discussions

**SM-6: Developer Experience**
- ✅ Clear documentation for both patterns
- ✅ Working examples demonstrating usage
- ✅ Error messages provide actionable guidance
- ✅ Configuration schema is intuitive

---

## Open Questions

**OQ-1:** Should Council support nested moderators (sub-councils)?
- **Impact:** Would enable hierarchical decision-making
- **Recommendation:** Defer to future enhancement

**OQ-2:** Should Grove support weighted expertise (priority rankings)?
- **Impact:** Could improve routing accuracy for multi-skilled agents
- **Recommendation:** Implement if simple, otherwise defer

**OQ-3:** How should we handle Council participants with different LLM providers?
- **Impact:** May cause inconsistent response times/quality
- **Recommendation:** Allow but document potential issues

**OQ-4:** Should Grove cache embeddings for agents to improve performance?
- **Impact:** Would reduce SemanticSimilarity routing time
- **Recommendation:** Implement simple in-memory cache

**OQ-5:** How should we visualize Council conversations in logs?
- **Impact:** Affects debuggability and user experience
- **Recommendation:** Use Herald for formatted output

**OQ-6:** Should Grove support agent warm-up/preloading?
- **Impact:** Could reduce first-task latency
- **Recommendation:** Defer to future optimization

**OQ-7:** How should we handle Council participant conflicts (disagreements)?
- **Impact:** May require conflict resolution strategies
- **Recommendation:** Document patterns, implement consensus detection

**OQ-8:** Should we implement EmbeddingPort if it doesn't exist?
- **Impact:** Required for SemanticSimilarity routing
- **Recommendation:** Check existing codebase, create minimal implementation if needed

---

## Implementation Phases

### Phase 1: Council Foundation (Week 1)
- Implement Council domain models (US-16.1)
- Implement CouncilExecutionService with RoundRobin (US-16.2)
- Write unit tests for turn-taking logic
- Create basic `examples/council_discussion.rs`

### Phase 2: Grove Foundation (Week 1)
- Implement Grove domain models (US-16.3)
- Implement GroveExecutionService with KeywordMatch (US-16.4)
- Write unit tests for routing logic
- Create basic `examples/grove_routing.rs`

### Phase 3: Advanced Strategies (Week 2)
- Add ModeratorDirected to Council
- Add SemanticSimilarity and LlmRouting to Grove
- Implement all termination conditions
- Expand test coverage

### Phase 4: Integration & Polish (Week 2)
- Integrate with Commander (US-16.5)
- Add CLI support
- Complete Garrison integration
- Write integration tests
- Update documentation

---

## Acceptance Criteria Summary

**Epic 16 is complete when:**

1. ✅ All 5 user stories implemented and tested
2. ✅ Council pattern supports RoundRobin and ModeratorDirected turn-taking
3. ✅ Grove pattern supports all three routing strategies (KeywordMatch default)
4. ✅ Both patterns integrated with Commander
5. ✅ Garrison stores Council conversation history
6. ✅ Examples run successfully: `council_discussion.rs`, `grove_routing.rs`
7. ✅ CLI supports `--strategy council` and `--strategy grove`
8. ✅ All tests passing with ≥80% coverage
9. ✅ Documentation complete for both patterns
10. ✅ No regressions in existing Battalion patterns

---

## References

- Epic 16 Original: `/project/Milestone_2-Missing_features/Epic_16/epic16.md`
- Hexagonal Architecture Guide: `/notes/hexagonal-arch.md`
- Battalion Design: `/docs/BATTALION.md`
- Commander Documentation: Update required after implementation
- Garrison Documentation: `/docs/GARRISON.md`

---

## Appendices

### Appendix A: Code Structure

```
src/
├── core/platform/container/battalion/
│   ├── council.rs          # NEW: Council domain models
│   └── grove.rs            # NEW: Grove domain models
├── application/use_cases/battalion/
│   ├── council_service.rs  # NEW: Council execution
│   └── grove_service.rs    # NEW: Grove execution
├── application/ports/output/
│   └── embedding_port.rs   # NEW if needed
examples/
├── council_discussion.rs   # NEW
├── grove_routing.rs        # NEW
├── commander_council.rs    # NEW
└── commander_grove.rs      # NEW
```

### Appendix B: Error Types

```rust
#[derive(Debug, thiserror::Error)]
pub enum CouncilError {
    #[error("No participants configured")]
    NoParticipants,
    #[error("Moderator required for ModeratorDirected strategy")]
    ModeratorRequired,
    #[error("Participant execution failed: {0}")]
    ParticipantError(String),
    #[error("Invalid turn strategy configuration: {0}")]
    InvalidStrategy(String),
}

#[derive(Debug, thiserror::Error)]
pub enum GroveError {
    #[error("No trees configured")]
    NoTrees,
    #[error("No agents in grove")]
    NoAgents,
    #[error("Routing failed: {0}")]
    RoutingFailed(String),
    #[error("No agent meets similarity threshold {0}")]
    NoMatchingAgent(f32),
    #[error("Embeddings required for SemanticSimilarity strategy")]
    EmbeddingsRequired,
}
```

### Appendix C: Example Usage

**Council Example:**
```rust
let council = CouncilBuilder::new()
    .name("Security Review Panel")
    .max_rounds(5)
    .turn_strategy(TurnStrategy::RoundRobin)
    .add_participant(security_expert)
    .add_participant(compliance_officer)
    .add_participant(dev_lead)
    .build()?;

let result = council_service.convene(&council, 
    "Review the proposed authentication changes").await?;

println!("Discussion concluded: {}", result.conclusion.unwrap());
```

**Grove Example:**
```rust
let grove = GroveBuilder::new()
    .name("Engineering Specialists")
    .routing_strategy(RoutingStrategy::SemanticSimilarity)
    .add_tree(security_tree)
    .add_tree(performance_tree)
    .build()?;

let result = grove_service.execute(&grove, 
    "Analyze this code for SQL injection vulnerabilities").await?;

println!("Routed to: {} (confidence: {})", 
    result.routing_decision.selected_agent,
    result.routing_decision.confidence);
```

---

**Document Version:** 1.0  
**Created:** February 2, 2026  
**Status:** Ready for Implementation  
**Epic:** Epic 16 - Advanced Battalion Patterns
