## Epic 16: Advanced Battalion Patterns

**Theme:** Council and Tree-based routing
**Duration:** 2 weeks  
**Priority:** Medium  
**Dependencies:** Epic 4, Epic 15  

### Description
Implement additional orchestration patterns including Council for conversational collaboration and tree-based agent routing.

### User Stories

#### US-16.1: Council Domain Model (GroupChat)
**As a** framework developer  
**I want** domain models for conversational multi-agent  
**So that** agents can discuss and reach consensus

**Acceptance Criteria:**
- [ ] `Council` struct in `src/core/platform/container/battalion/council.rs`
- [ ] Contains: participant agents, moderator (optional), configuration
- [ ] `CouncilConfig` with turn settings, max rounds
- [ ] `CouncilMessage` for conversation tracking
- [ ] Turn-taking strategies: RoundRobin, Random, Moderator

**Definition of Done:**
```rust
pub struct Council {
    pub name: String,
    pub participants: Vec<Paladin>,
    pub moderator: Option<Paladin>,
    pub config: CouncilConfig,
}

pub struct CouncilConfig {
    pub max_rounds: u32,
    pub turn_strategy: TurnStrategy,
    pub termination_condition: TerminationCondition,
    pub include_history: bool,
}

pub enum TurnStrategy {
    RoundRobin,
    Random,
    ModeratorDirected,
    VoluntaryWithTimeout { timeout_ms: u64 },
}

pub enum TerminationCondition {
    MaxRounds,
    Consensus,
    ModeratorDecision,
    Keyword(String),
}

pub struct CouncilMessage {
    pub speaker: String,
    pub content: String,
    pub round: u32,
    pub timestamp: DateTime<Utc>,
}
```

---

#### US-16.2: Council Execution Service
**As a** developer  
**I want** to execute group discussions  
**So that** agents can collaboratively solve problems

**Acceptance Criteria:**
- [ ] `CouncilExecutionService` manages conversation flow
- [ ] Tracks conversation history
- [ ] Implements turn-taking logic
- [ ] Detects termination conditions
- [ ] Returns full transcript and conclusion

**Definition of Done:**
```rust
pub struct CouncilExecutionService {
    paladin_port: Arc<dyn PaladinPort>,
}

impl CouncilExecutionService {
    pub async fn convene(
        &self,
        council: &Council,
        topic: &str,
    ) -> Result<CouncilResult, BattalionError>;
}

pub struct CouncilResult {
    pub transcript: Vec<CouncilMessage>,
    pub conclusion: Option<String>,
    pub rounds_completed: u32,
    pub termination_reason: TerminationCondition,
}
```

---

#### US-16.3: Grove Domain Model
**As a** framework developer  
**I want** domain models for tree-based agent routing  
**So that** tasks route to best-fit experts

**Acceptance Criteria:**
- [ ] `Grove` struct in `src/core/platform/container/battalion/grove.rs`
- [ ] `Tree` struct containing related agents
- [ ] Agents have expertise keywords/embeddings
- [ ] `GroveConfig` with routing settings
- [ ] Routing based on semantic similarity to task

**Definition of Done:**
```rust
pub struct Grove {
    pub name: String,
    pub trees: Vec<Tree>,
    pub config: GroveConfig,
}

pub struct Tree {
    pub name: String,
    pub agents: Vec<TreeAgent>,
}

pub struct TreeAgent {
    pub paladin: Paladin,
    pub expertise_keywords: Vec<String>,
    pub expertise_embedding: Option<Vec<f32>>,
}

pub struct GroveConfig {
    pub routing_strategy: RoutingStrategy,
    pub fallback_tree: Option<String>,
    pub similarity_threshold: f32,
}

pub enum RoutingStrategy {
    KeywordMatch,
    SemanticSimilarity,
    LlmRouting,
}
```

---

#### US-16.4: Grove Execution Service
**As a** developer  
**I want** automatic routing to best-fit agents  
**So that** tasks are handled by experts

**Acceptance Criteria:**
- [ ] `GroveExecutionService` routes tasks
- [ ] Calculates task-to-agent similarity
- [ ] Selects best tree, then best agent
- [ ] Falls back if no good match
- [ ] Returns routing decision and result

**Definition of Done:**
```rust
pub struct GroveExecutionService {
    paladin_port: Arc<dyn PaladinPort>,
    embedding_port: Option<Arc<dyn EmbeddingPort>>,
}

impl GroveExecutionService {
    pub async fn execute(
        &self,
        grove: &Grove,
        task: &str,
    ) -> Result<GroveResult, BattalionError>;

    async fn route_task(
        &self,
        grove: &Grove,
        task: &str,
    ) -> Result<RoutingDecision, BattalionError>;
}

pub struct RoutingDecision {
    pub selected_tree: String,
    pub selected_agent: String,
    pub confidence: f32,
    pub reasoning: String,
}
```

---

#### US-16.5: Commander Integration
**As a** developer  
**I want** Commander to support Council and Grove  
**So that** I have unified orchestration

**Acceptance Criteria:**
- [ ] `BattalionStrategy::Council` variant
- [ ] `BattalionStrategy::Grove` variant
- [ ] Auto-strategy considers these patterns
- [ ] CLI support for both types

**Definition of Done:**
```rust
pub enum BattalionStrategy {
    Formation,
    Phalanx,
    Campaign,
    ChainOfCommand,
    Conclave,
    Council,  // NEW
    Grove,    // NEW
    Auto,
}
```

---

### Epic 16 Completion Criteria
- [ ] All 5 user stories completed and tested
- [ ] Council (GroupChat) fully functional
- [ ] Grove (ForestSwarm) fully functional
- [ ] Commander integration complete
- [ ] Documentation for both patterns
- [ ] Example: `examples/council_discussion.rs`
- [ ] Example: `examples/grove_routing.rs`

---
