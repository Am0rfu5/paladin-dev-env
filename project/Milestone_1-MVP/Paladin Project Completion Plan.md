## Enterprise-Grade Multi-Agent Orchestration Framework

_Project Plan & Technical Roadmap_

---

## Executive Summary

Paladin is a Rust-based, enterprise-grade multi-agent orchestration framework built on Hexagonal Architecture and Domain-Driven Design principles. This project plan outlines the development roadmap to complete Paladin's capabilities as a production-ready platform for orchestrating intelligent AI agents at scale.

The plan follows Test-Driven Development (TDD) methodology with comprehensive test coverage requirements, adhering to professional software engineering best practices throughout.

### Project Vision

Paladin will enable enterprises to:

- Deploy autonomous AI agents (Paladins) with configurable behaviors
- Orchestrate multi-agent systems using military-inspired command structures
- Integrate external tools through standardized protocols
- Scale from single-agent applications to large distributed agent networks

### Naming Convention

Following the Paladin theme, multi-agent structures use Medieval Military terminology:

- **Paladin** - An autonomous AI agent
- **Battalion** - A coordinated group of Paladins working together
- **Formation** - Sequential agent execution pattern
- **Phalanx** - Concurrent agent execution pattern
- **Campaign** - Graph-based agent orchestration
- **Chain of Command** - Hierarchical agent delegation
- **Commander** - Dynamic battalion strategy router
- **Garrison** - Agent memory and state persistence

---

## Project Methodology

### Development Principles

1. **Domain-Driven Design (DDD)**
    
    - Rich domain models with ubiquitous language
    - Bounded contexts with clear boundaries
    - Domain events for cross-context communication
    - Aggregates protecting invariants
2. **Test-Driven Development (TDD)**
    
    - Red-Green-Refactor cycle for all features
    - Unit test coverage minimum 80%
    - Integration test coverage minimum 70%
    - Contract tests for port implementations
3. **Hexagonal Architecture**
    
    - Domain logic isolated from infrastructure
    - Ports define abstract interfaces
    - Adapters implement infrastructure concerns
    - Dependency inversion throughout
4. **Professional Standards**
    
    - Code review required for all changes
    - Documentation for all public APIs
    - Semantic versioning for releases
    - Continuous integration and deployment

---

## Current State Assessment

### Existing Capabilities

Paladin currently provides:

|Component|Status|Description|
|---|---|---|
|Orchestrator|✓ Complete|Job and task coordination|
|Scheduler|✓ Complete|Timed execution management|
|Queue Service|✓ Complete|Async message processing|
|Listener Service|✓ Complete|Event-driven triggers|
|LLM Port|✓ Complete|Language model abstraction|
|OpenAI Adapter|◐ Partial|Basic completion support|
|Notification Ports|✓ Complete|Multi-channel notifications|
|Storage Ports|✓ Complete|SQL/NoSQL/File storage|
|Content Processing|✓ Complete|Ingestion and analysis|

### Required Capabilities

|Component|Priority|Description|
|---|---|---|
|Paladin Entity|Critical|Core agent domain model|
|Paladin Builder|Critical|Fluent agent configuration|
|Garrison (Memory)|High|Conversation and context storage|
|Arsenal (Tools)|High|Tool execution framework|
|MCP Protocol|High|Model Context Protocol support|
|Formation Battalion|Critical|Sequential multi-agent|
|Phalanx Battalion|Critical|Concurrent multi-agent|
|Campaign Battalion|High|DAG-based orchestration|
|Chain of Command|High|Hierarchical delegation|
|Commander Router|Medium|Dynamic strategy selection|
|Additional Providers|High|DeepSeek, Anthropic adapters|
|State Persistence|Medium|Autosave and recovery|

---

## Epic Overview

|Epic|Name|Priority|Duration|Dependencies|
|---|---|---|---|---|
|1|Paladin Domain Foundation|Critical|3-4 weeks|None|
|2|Garrison Memory System|High|2-3 weeks|Epic 1|
|3|Arsenal Tool System|High|3-4 weeks|Epic 1|
|4|Battalion Orchestration|Critical|4-5 weeks|Epics 1, 2|
|5|Commander Strategy Router|Medium|2 weeks|Epic 4|
|6|Provider Expansion|High|2-3 weeks|Epic 1|
|7|Citadel State Persistence|Medium|2 weeks|Epics 1, 2|
|8|Herald Output Formatting|Low|1-2 weeks|Epic 1|
|9|Armory CLI Tools|Medium|2-3 weeks|Epics 1-4|
|10|Validation & Documentation|High|2-3 weeks|All|

**Total Estimated Duration:** 18-24 weeks (4.5-6 months)

---

## Epic 1: Paladin Domain Foundation

### Overview

**Priority:** Critical  
**Effort:** 3-4 weeks  
**Dependencies:** None  
**Team:** 2 developers

**Objective:** Establish the Paladin as the core domain entity representing an autonomous AI agent with full lifecycle management, configuration, and execution capabilities.

### User Stories

1. **As a developer**, I want to create a Paladin with a system prompt so that it can respond according to defined behavior.
2. **As a developer**, I want to configure Paladin parameters (temperature, max_loops) so that I can control response generation.
3. **As a developer**, I want to execute a Paladin with user input so that it generates intelligent responses.
4. **As a developer**, I want to define stop words so that the Paladin knows when to terminate processing.

### Technical Design

#### Domain Layer (core/platform/container/)

**paladin.rs - Paladin Entity**

```rust
/// Core Paladin domain entity
/// Represents an autonomous AI agent capable of reasoning and action
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaladinData {
    /// System prompt defining Paladin behavior
    pub system_prompt: String,
    /// Display name for the Paladin
    pub name: String,
    /// Name for the user in conversations
    pub user_name: String,
    /// LLM model identifier
    pub model: String,
    /// Response randomness (0.0-1.0)
    pub temperature: f32,
    /// Maximum reasoning iterations
    pub max_loops: u32,
    /// Tokens that signal completion
    pub stop_words: Vec<String>,
    /// Current execution status
    pub status: PaladinStatus,
}

pub type Paladin = Node<PaladinData>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PaladinStatus {
    Idle,
    Reasoning,
    Executing,
    Completed,
    Failed(String),
}
```

**paladin_config.rs - Configuration**

```rust
/// Runtime configuration for Paladin execution
#[derive(Debug, Clone, Builder, Serialize, Deserialize)]
pub struct PaladinConfig {
    pub retry_attempts: u32,
    pub timeout_seconds: u64,
    pub enable_planning: bool,
    pub planning_prompt: Option<String>,
    pub output_format: OutputFormat,
}
```

#### Application Layer (application/)

**ports/output/paladin_port.rs**

```rust
/// Port abstraction for Paladin execution
#[async_trait]
pub trait PaladinPort: Send + Sync {
    /// Execute Paladin with given input
    async fn execute(&self, paladin: &Paladin, input: &str) -> Result<PaladinResult, PaladinError>;
    
    /// Execute with streaming response
    async fn execute_stream(&self, paladin: &Paladin, input: &str) 
        -> Result<PaladinStream, PaladinError>;
    
    /// Validate Paladin configuration
    fn validate(&self, paladin: &Paladin) -> Result<(), PaladinError>;
}
```

**use_cases/paladin/paladin_builder.rs**

```rust
/// Fluent builder for Paladin construction
pub struct PaladinBuilder {
    llm_port: Arc<dyn LlmPort>,
    data: PaladinData,
    config: PaladinConfig,
}

impl PaladinBuilder {
    pub fn new(llm_port: Arc<dyn LlmPort>) -> Self;
    pub fn system_prompt(self, prompt: &str) -> Self;
    pub fn name(self, name: &str) -> Self;
    pub fn user_name(self, name: &str) -> Self;
    pub fn model(self, model: &str) -> Self;
    pub fn temperature(self, temp: f32) -> Self;
    pub fn max_loops(self, loops: u32) -> Self;
    pub fn add_stop_word(self, word: &str) -> Self;
    pub fn enable_planning(self, prompt: Option<&str>) -> Self;
    pub fn build(self) -> Result<Paladin, PaladinError>;
}
```

**use_cases/paladin/paladin_execution_service.rs**

```rust
/// Service coordinating Paladin execution
pub struct PaladinExecutionService {
    llm_port: Arc<dyn LlmPort>,
    config: PaladinConfig,
}

impl PaladinExecutionService {
    /// Execute reasoning loop with retry logic
    pub async fn execute(&self, paladin: &Paladin, input: &str) 
        -> Result<PaladinResult, PaladinError>;
    
    /// Check for stop word presence in output
    fn check_stop_words(&self, output: &str, stop_words: &[String]) -> bool;
    
    /// Build prompt from Paladin configuration and input
    fn build_prompt(&self, paladin: &Paladin, input: &str, history: &[Message]) -> PromptItem;
}
```

### Test Requirements

#### Unit Tests (>80% coverage)

- `test_paladin_builder_creates_valid_paladin`
- `test_paladin_builder_validates_required_fields`
- `test_paladin_builder_rejects_invalid_temperature`
- `test_paladin_status_transitions`
- `test_paladin_serialization_roundtrip`
- `test_stop_word_detection`
- `test_max_loops_enforcement`

#### Integration Tests

- `test_paladin_executes_with_mock_llm`
- `test_paladin_respects_timeout`
- `test_paladin_retry_on_failure`

### Acceptance Criteria

- [ ] Paladin can be constructed via builder pattern with validation
- [ ] Paladin executes queries against LlmPort with proper prompt formatting
- [ ] Paladin respects max_loops configuration
- [ ] Paladin stops on configured stop words
- [ ] Unit test coverage ≥ 80%
- [ ] All public APIs documented with rustdoc
- [ ] Code passes clippy with no warnings

### Definition of Done

- [ ] All tests passing
- [ ] Code reviewed and approved
- [ ] Documentation complete
- [ ] Merged to main branch
- [ ] Example code demonstrating usage

---

## Epic 2: Garrison Memory System

### Overview

**Priority:** High  
**Effort:** 2-3 weeks  
**Dependencies:** Epic 1  
**Team:** 1-2 developers

**Objective:** Implement the Garrison memory system enabling Paladins to maintain conversation context and persist knowledge across sessions.

### User Stories

1. **As a developer**, I want Paladins to remember conversation history so that context is maintained.
2. **As a developer**, I want to configure memory window size so that token limits are respected.
3. **As a developer**, I want to persist Paladin memory so that sessions can be resumed.
4. **As a developer**, I want to search memory by content so that relevant context can be retrieved.

### Technical Design

#### Domain Layer

**garrison.rs - Memory Domain**

```rust
/// A single memory entry in the Garrison
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GarrisonEntry {
    pub id: Uuid,
    pub role: ConversationRole,
    pub content: String,
    pub timestamp: DateTime<Utc>,
    pub metadata: HashMap<String, Value>,
    pub token_count: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConversationRole {
    System,
    User,
    Assistant,
    Tool,
}

/// Conversation history with windowing support
#[derive(Debug, Clone)]
pub struct ConversationHistory {
    entries: VecDeque<GarrisonEntry>,
    max_entries: usize,
    max_tokens: Option<u32>,
}

/// Memory type classification
#[derive(Debug, Clone)]
pub enum GarrisonType {
    /// Active conversation context
    ShortTerm,
    /// Persisted knowledge
    LongTerm,
    /// Specific event memories
    Episodic,
}
```

#### Application Layer

**ports/output/garrison_port.rs**

```rust
/// Port for memory operations
#[async_trait]
pub trait GarrisonPort: Send + Sync {
    /// Add entry to memory
    async fn remember(&self, entry: GarrisonEntry) -> Result<(), GarrisonError>;
    
    /// Retrieve recent entries
    async fn recall_recent(&self, limit: usize) -> Result<Vec<GarrisonEntry>, GarrisonError>;
    
    /// Search memory by content
    async fn search(&self, query: &str, limit: usize) -> Result<Vec<GarrisonEntry>, GarrisonError>;
    
    /// Clear all memory
    async fn forget_all(&self) -> Result<(), GarrisonError>;
    
    /// Get memory statistics
    async fn stats(&self) -> Result<GarrisonStats, GarrisonError>;
}

/// Extended port for long-term memory with vector search
#[async_trait]
pub trait LongTermGarrisonPort: GarrisonPort {
    /// Add entry with embedding
    async fn remember_with_embedding(&self, entry: GarrisonEntry, embedding: Vec<f32>) 
        -> Result<(), GarrisonError>;
    
    /// Semantic similarity search
    async fn search_similar(&self, embedding: Vec<f32>, limit: usize) 
        -> Result<Vec<GarrisonEntry>, GarrisonError>;
}
```

#### Infrastructure Layer

**adapters/garrison/in_memory_garrison.rs**

```rust
/// In-memory garrison for short-term storage
pub struct InMemoryGarrison {
    entries: RwLock<VecDeque<GarrisonEntry>>,
    config: GarrisonConfig,
}
```

**adapters/garrison/sqlite_garrison.rs**

```rust
/// SQLite-backed garrison for persistent storage
pub struct SqliteGarrison {
    pool: SqlitePool,
    config: GarrisonConfig,
}
```

### Test Requirements

#### Unit Tests

- `test_garrison_entry_creation`
- `test_conversation_history_windowing`
- `test_token_limit_enforcement`
- `test_memory_search_accuracy`
- `test_garrison_serialization`

#### Integration Tests

- `test_sqlite_garrison_persistence`
- `test_paladin_with_garrison_context`
- `test_garrison_recovery_after_restart`

### Acceptance Criteria

- [ ] Paladins maintain conversation context across multiple interactions
- [ ] Memory window limits prevent context overflow
- [ ] Memory can be persisted to SQLite and restored
- [ ] Search returns relevant entries based on content
- [ ] Unit test coverage ≥ 80%

### Definition of Done

- [ ] All tests passing
- [ ] Code reviewed and approved
- [ ] Documentation complete
- [ ] Integration with Epic 1 verified

---

## Epic 3: Arsenal Tool System

### Overview

**Priority:** High  
**Effort:** 3-4 weeks  
**Dependencies:** Epic 1  
**Team:** 2 developers

**Objective:** Implement the Arsenal tool system with MCP (Model Context Protocol) support, enabling Paladins to interact with external services and capabilities.

### User Stories

1. **As a developer**, I want to register tools with Paladins so that they can perform actions.
2. **As a developer**, I want to connect MCP servers so that Paladins can use external tools.
3. **As a developer**, I want Paladins to automatically invoke tools so that complex tasks are completed.
4. **As a developer**, I want tool results injected into context so that Paladins can reason about outcomes.

### Technical Design

#### Domain Layer

**arsenal.rs - Tool Domain**

```rust
/// Definition of a tool in the Arsenal
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Armament {
    pub name: String,
    pub description: String,
    pub parameters: JsonSchema,
    pub required_params: Vec<String>,
}

/// A request to invoke a tool
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArmamentCall {
    pub tool_name: String,
    pub arguments: HashMap<String, Value>,
    pub call_id: Uuid,
}

/// Result of tool invocation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArmamentResult {
    pub call_id: Uuid,
    pub success: bool,
    pub output: Option<Value>,
    pub error: Option<String>,
    pub execution_time_ms: u64,
}
```

#### Application Layer

**ports/output/arsenal_port.rs**

```rust
/// Port for tool operations
#[async_trait]
pub trait ArsenalPort: Send + Sync {
    /// List available tools
    async fn list_armaments(&self) -> Vec<Armament>;
    
    /// Invoke a tool
    async fn invoke(&self, call: ArmamentCall) -> Result<ArmamentResult, ArsenalError>;
    
    /// Validate tool call arguments
    fn validate_call(&self, call: &ArmamentCall) -> Result<(), ArsenalError>;
}

/// Registry for managing tool collections
#[async_trait]
pub trait ArsenalRegistry: Send + Sync {
    /// Register a new tool
    async fn register(&self, armament: Armament, handler: Box<dyn ArmamentHandler>);
    
    /// Unregister a tool
    async fn unregister(&self, name: &str) -> Option<Armament>;
    
    /// Get tool by name
    async fn get(&self, name: &str) -> Option<&Armament>;
}
```

**MCP Protocol Implementation**

**adapters/arsenal/mcp_client.rs**

```rust
/// MCP protocol client
pub struct MCPClient {
    transport: Box<dyn MCPTransport>,
    capabilities: MCPCapabilities,
}

#[async_trait]
pub trait MCPTransport: Send + Sync {
    async fn send(&self, message: MCPMessage) -> Result<MCPResponse, MCPError>;
    async fn receive(&self) -> Result<MCPMessage, MCPError>;
}
```

**adapters/arsenal/mcp_stdio_adapter.rs**

```rust
/// MCP adapter for STDIO-based tool servers
pub struct MCPStdioAdapter {
    command: String,
    args: Vec<String>,
    process: Option<Child>,
}
```

**adapters/arsenal/mcp_sse_adapter.rs**

```rust
/// MCP adapter for SSE-based tool servers
pub struct MCPSseAdapter {
    endpoint: String,
    client: reqwest::Client,
}
```

### Builder Integration

```rust
impl PaladinBuilder {
    /// Add a tool to the Paladin's arsenal
    pub fn add_armament(self, armament: Armament, handler: Box<dyn ArmamentHandler>) -> Self;
    
    /// Add STDIO MCP server
    pub async fn add_mcp_stdio(self, command: &str, args: &[&str]) -> Self;
    
    /// Add SSE MCP server
    pub async fn add_mcp_sse(self, name: &str, endpoint: &str) -> Self;
}
```

### Test Requirements

#### Unit Tests

- `test_armament_schema_validation`
- `test_armament_call_serialization`
- `test_arsenal_registry_operations`
- `test_tool_result_handling`

#### Integration Tests

- `test_mcp_stdio_server_connection`
- `test_mcp_sse_server_connection`
- `test_paladin_tool_invocation`
- `test_tool_result_context_injection`

### Acceptance Criteria

- [ ] Paladins can invoke registered tools during execution
- [ ] MCP STDIO servers can be connected and discovered
- [ ] MCP SSE servers can be connected and discovered
- [ ] Tool results are properly formatted and injected into context
- [ ] Tool call failures are handled gracefully
- [ ] Unit test coverage ≥ 80%

### Definition of Done

- [ ] All tests passing
- [ ] Code reviewed and approved
- [ ] MCP protocol compliance verified
- [ ] Documentation includes tool authoring guide

---

## Epic 4: Battalion Orchestration

### Overview

**Priority:** Critical  
**Effort:** 4-5 weeks  
**Dependencies:** Epics 1, 2  
**Team:** 2-3 developers

**Objective:** Implement Battalion structures for multi-Paladin orchestration including Formation (sequential), Phalanx (concurrent), Campaign (graph), and Chain of Command (hierarchical) patterns.

### User Stories

1. **As a developer**, I want to create a Formation so that Paladins execute in sequence with output passing.
2. **As a developer**, I want to create a Phalanx so that Paladins execute in parallel.
3. **As a developer**, I want to create a Campaign so that Paladins follow a directed graph workflow.
4. **As a developer**, I want to create a Chain of Command so that a leader Paladin delegates to specialists.

### Technical Design

#### Domain Layer

**battalion/mod.rs - Battalion Base**

```rust
/// Configuration for Battalion operations
#[derive(Debug, Clone, Builder, Serialize, Deserialize)]
pub struct BattalionConfig {
    pub name: String,
    pub description: Option<String>,
    pub timeout_seconds: u64,
    pub retry_policy: RetryPolicy,
    pub error_strategy: ErrorStrategy,
    pub metadata_output_dir: Option<PathBuf>,
}

/// Result of Battalion execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BattalionResult {
    pub battalion_id: Uuid,
    pub battalion_name: String,
    pub started_at: DateTime<Utc>,
    pub completed_at: DateTime<Utc>,
    pub final_output: String,
    pub paladin_results: Vec<PaladinResult>,
    pub status: BattalionStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ErrorStrategy {
    FailFast,
    ContinueOnError,
    RetryThenContinue,
}
```

**battalion/formation.rs - Sequential Execution**

```rust
/// Formation: Sequential multi-Paladin execution
/// Output of Paladin N becomes input to Paladin N+1
#[derive(Debug, Clone)]
pub struct Formation {
    pub id: Uuid,
    pub config: BattalionConfig,
    pub paladins: Vec<Paladin>,
    pub shared_context: Option<String>,
}

impl Formation {
    pub fn new(name: &str, paladins: Vec<Paladin>) -> Self;
    pub fn with_config(config: BattalionConfig, paladins: Vec<Paladin>) -> Self;
    pub fn with_shared_context(self, context: &str) -> Self;
}
```

**battalion/phalanx.rs - Concurrent Execution**

```rust
/// Phalanx: Parallel multi-Paladin execution
/// All Paladins receive same input, results aggregated
#[derive(Debug, Clone)]
pub struct Phalanx {
    pub id: Uuid,
    pub config: BattalionConfig,
    pub paladins: Vec<Paladin>,
    pub aggregation: AggregationStrategy,
}

#[derive(Debug, Clone)]
pub enum AggregationStrategy {
    /// Return all results
    CollectAll,
    /// Return first successful result
    FirstSuccess,
    /// Return majority consensus
    Majority,
    /// Custom aggregation function
    Custom(Arc<dyn Fn(Vec<PaladinResult>) -> String + Send + Sync>),
}
```

**battalion/campaign.rs - Graph Execution**

```rust
/// Campaign: DAG-based multi-Paladin orchestration
/// Paladins connected by conditional edges
#[derive(Debug)]
pub struct Campaign {
    pub id: Uuid,
    pub config: BattalionConfig,
    pub graph: DiGraph<Paladin, CampaignEdge>,
    pub entry_points: Vec<NodeIndex>,
}

#[derive(Debug, Clone)]
pub struct CampaignEdge {
    pub condition: Option<EdgeCondition>,
    pub transform: Option<Arc<dyn Fn(&str) -> String + Send + Sync>>,
}

#[derive(Debug, Clone)]
pub enum EdgeCondition {
    Always,
    Contains(String),
    Regex(String),
    Custom(Arc<dyn Fn(&str) -> bool + Send + Sync>),
}

impl Campaign {
    pub fn new(name: &str) -> Self;
    pub fn add_paladin(&mut self, paladin: Paladin) -> NodeIndex;
    pub fn add_edge(&mut self, from: NodeIndex, to: NodeIndex, edge: CampaignEdge);
    pub fn validate(&self) -> Result<(), CampaignError>;
}
```

**battalion/chain_of_command.rs - Hierarchical Execution**

```rust
/// Chain of Command: Hierarchical delegation pattern
/// Commander delegates to specialists, aggregates results
#[derive(Debug)]
pub struct ChainOfCommand {
    pub id: Uuid,
    pub config: BattalionConfig,
    pub commander: Paladin,
    pub specialists: Vec<Paladin>,
    pub delegation_strategy: DelegationStrategy,
}

#[derive(Debug, Clone)]
pub enum DelegationStrategy {
    /// Commander chooses based on task analysis
    Automatic,
    /// Delegate to all specialists
    Broadcast,
    /// Round-robin assignment
    RoundRobin,
    /// Custom delegation logic
    Custom(Arc<dyn Fn(&str, &[Paladin]) -> Vec<usize> + Send + Sync>),
}
```

#### Application Layer

**ports/output/battalion_port.rs**

```rust
/// Port for Battalion execution
#[async_trait]
pub trait BattalionPort: Send + Sync {
    /// Execute the battalion with given input
    async fn execute(&self, input: &str) -> Result<BattalionResult, BattalionError>;
    
    /// Get current execution status
    async fn status(&self) -> BattalionStatus;
    
    /// Cancel ongoing execution
    async fn cancel(&self) -> Result<(), BattalionError>;
}
```

**use_cases/battalion/formation_service.rs**

```rust
pub struct FormationExecutionService {
    paladin_service: Arc<PaladinExecutionService>,
    garrison_port: Arc<dyn GarrisonPort>,
}

impl FormationExecutionService {
    pub async fn execute(&self, formation: &Formation, input: &str) 
        -> Result<BattalionResult, BattalionError> {
        let mut current_input = input.to_string();
        let mut results = Vec::new();
        
        for paladin in &formation.paladins {
            let result = self.paladin_service.execute(paladin, &current_input).await?;
            current_input = result.output.clone();
            results.push(result);
        }
        
        Ok(BattalionResult::from_paladin_results(results))
    }
}
```

**use_cases/battalion/phalanx_service.rs**

```rust
pub struct PhalanxExecutionService {
    paladin_service: Arc<PaladinExecutionService>,
}

impl PhalanxExecutionService {
    pub async fn execute(&self, phalanx: &Phalanx, input: &str) 
        -> Result<BattalionResult, BattalionError> {
        let futures: Vec<_> = phalanx.paladins.iter()
            .map(|p| self.paladin_service.execute(p, input))
            .collect();
        
        let results = futures::future::join_all(futures).await;
        let aggregated = phalanx.aggregation.aggregate(results)?;
        
        Ok(aggregated)
    }
}
```

### Test Requirements

#### Unit Tests

- `test_formation_paladin_ordering`
- `test_formation_output_passing`
- `test_phalanx_parallel_execution`
- `test_phalanx_aggregation_strategies`
- `test_campaign_graph_validation`
- `test_campaign_edge_conditions`
- `test_chain_of_command_delegation`

#### Integration Tests

- `test_formation_end_to_end`
- `test_phalanx_concurrent_llm_calls`
- `test_campaign_conditional_routing`
- `test_chain_of_command_specialist_selection`

### Acceptance Criteria

- [ ] Formation passes output correctly between sequential Paladins
- [ ] Phalanx executes Paladins in parallel using tokio
- [ ] Campaign respects DAG structure and edge conditions
- [ ] Chain of Command properly delegates and aggregates
- [ ] All error strategies implemented and tested
- [ ] Unit test coverage ≥ 80%

### Definition of Done

- [ ] All tests passing
- [ ] Code reviewed and approved
- [ ] Performance benchmarks documented
- [ ] Example for each Battalion type

---

## Epic 5: Commander Strategy Router

### Overview

**Priority:** Medium  
**Effort:** 2 weeks  
**Dependencies:** Epic 4  
**Team:** 1 developer

**Objective:** Implement the Commander router providing a unified interface for dynamic Battalion strategy selection.

### User Stories

1. **As a developer**, I want a single interface to execute any Battalion type so that code is simplified.
2. **As a developer**, I want automatic strategy selection so that optimal orchestration is chosen.
3. **As a developer**, I want consistent result formats so that downstream processing is uniform.

### Technical Design

#### Application Layer

**use_cases/battalion/commander.rs**

```rust
/// Battalion strategy types
#[derive(Debug, Clone)]
pub enum BattalionStrategy {
    Formation,
    Phalanx,
    Campaign,
    ChainOfCommand,
    /// Automatically select based on task analysis
    Auto,
}

/// Commander: Unified Battalion routing and execution
pub struct Commander {
    strategy: BattalionStrategy,
    paladins: Vec<Paladin>,
    config: BattalionConfig,
    formation_service: Arc<FormationExecutionService>,
    phalanx_service: Arc<PhalanxExecutionService>,
    campaign_service: Arc<CampaignExecutionService>,
    chain_service: Arc<ChainOfCommandService>,
}

impl Commander {
    pub fn new(strategy: BattalionStrategy, paladins: Vec<Paladin>) -> Self;
    
    pub async fn execute(&self, input: &str) -> Result<BattalionResult, BattalionError> {
        match self.resolve_strategy(input) {
            BattalionStrategy::Formation => self.execute_formation(input).await,
            BattalionStrategy::Phalanx => self.execute_phalanx(input).await,
            BattalionStrategy::Campaign => self.execute_campaign(input).await,
            BattalionStrategy::ChainOfCommand => self.execute_chain(input).await,
            BattalionStrategy::Auto => unreachable!("resolved above"),
        }
    }
    
    fn resolve_strategy(&self, input: &str) -> BattalionStrategy {
        if self.strategy != BattalionStrategy::Auto {
            return self.strategy.clone();
        }
        self.analyze_and_select(input)
    }
    
    fn analyze_and_select(&self, input: &str) -> BattalionStrategy;
}
```

### Acceptance Criteria

- [ ] Commander executes any registered Battalion type
- [ ] Auto mode selects appropriate strategy based on heuristics
- [ ] Consistent BattalionResult format across all strategies
- [ ] Unit test coverage ≥ 80%

---

## Epic 6: Provider Expansion

### Overview

**Priority:** High  
**Effort:** 2-3 weeks  
**Dependencies:** Epic 1  
**Team:** 1-2 developers

**Objective:** Expand LLM provider support to include DeepSeek, Anthropic, and establish patterns for future providers.

### Technical Design

#### Infrastructure Layer

**adapters/llm/deepseek_adapter.rs**

```rust
pub struct DeepSeekAdapter {
    client: reqwest::Client,
    config: DeepSeekConfig,
}

#[derive(Debug, Clone)]
pub struct DeepSeekConfig {
    pub api_key: String,
    pub base_url: String,
    pub model: String,
    pub timeout_seconds: u64,
}

#[async_trait]
impl LlmPort for DeepSeekAdapter {
    async fn generate(&self, request: LlmRequest) -> Result<LlmResponse, LlmError>;
    async fn generate_stream(&self, request: LlmRequest) 
        -> Result<Box<dyn Stream<Item = Result<StreamingResponse, LlmError>> + Send>, LlmError>;
    async fn validate_model(&self, model: &str) -> Result<bool, LlmError>;
    async fn get_available_models(&self) -> Result<Vec<String>, LlmError>;
    fn get_provider_name(&self) -> &'static str { "deepseek" }
}
```

**adapters/llm/anthropic_adapter.rs**

```rust
pub struct AnthropicAdapter {
    client: reqwest::Client,
    config: AnthropicConfig,
}

#[derive(Debug, Clone)]
pub struct AnthropicConfig {
    pub api_key: String,
    pub base_url: String,
    pub model: String,
    pub max_tokens: u32,
}

#[async_trait]
impl LlmPort for AnthropicAdapter {
    // Implementation with Claude-specific message format
}
```

### Acceptance Criteria

- [ ] DeepSeek provider works with all Paladin features
- [ ] Anthropic provider works with all Paladin features
- [ ] Providers are interchangeable via configuration
- [ ] Streaming works correctly for both providers
- [ ] Tool use format supported for Anthropic

---

## Epic 7: Citadel State Persistence

### Overview

**Priority:** Medium  
**Effort:** 2 weeks  
**Dependencies:** Epics 1, 2  
**Team:** 1 developer

**Objective:** Implement the Citadel persistence layer for autosave and state restoration of Paladins and Battalions.

### Technical Design

#### Domain Layer

**citadel.rs - State Management**

```rust
/// Serializable Paladin state for persistence
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaladinState {
    pub paladin: Paladin,
    pub garrison: Vec<GarrisonEntry>,
    pub execution_history: Vec<ExecutionRecord>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Serializable Battalion state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BattalionState {
    pub battalion_type: String,
    pub config: BattalionConfig,
    pub paladin_states: Vec<PaladinState>,
    pub checkpoint: Option<CheckpointData>,
}
```

#### Application Layer

**ports/output/citadel_port.rs**

```rust
#[async_trait]
pub trait CitadelPort: Send + Sync {
    async fn save_paladin(&self, state: &PaladinState) -> Result<(), CitadelError>;
    async fn load_paladin(&self, id: Uuid) -> Result<Option<PaladinState>, CitadelError>;
    async fn save_battalion(&self, state: &BattalionState) -> Result<(), CitadelError>;
    async fn load_battalion(&self, id: Uuid) -> Result<Option<BattalionState>, CitadelError>;
    async fn list_saved(&self) -> Result<Vec<StateSummary>, CitadelError>;
}
```

### Builder Integration

```rust
impl PaladinBuilder {
    /// Enable automatic state persistence
    pub fn enable_autosave(self) -> Self;
    
    /// Set directory for state files
    pub fn save_state_dir(self, path: &str) -> Self;
    
    /// Restore from saved state
    pub fn restore_from(self, state_id: Uuid) -> Self;
}
```

### Acceptance Criteria

- [ ] Paladin state persists across restarts
- [ ] Autosave triggers on configurable events
- [ ] Battalion workflows can resume from checkpoints
- [ ] State files are human-readable JSON

---

## Epic 8: Herald Output Formatting

### Overview

**Priority:** Low  
**Effort:** 1-2 weeks  
**Dependencies:** Epic 1  
**Team:** 1 developer

**Objective:** Implement the Herald formatting system for structured output from Paladins and Battalions.

### Technical Design

**herald.rs - Output Formatters**

```rust
pub trait Herald: Send + Sync {
    fn format_paladin_result(&self, result: &PaladinResult) -> String;
    fn format_battalion_result(&self, result: &BattalionResult) -> String;
}

pub struct MarkdownHerald;
pub struct JsonHerald;
pub struct TableHerald;

impl Herald for MarkdownHerald {
    fn format_paladin_result(&self, result: &PaladinResult) -> String {
        format!(
            "## Paladin: {}\n\n**Status:** {}\n\n### Output\n\n{}\n\n### Metadata\n\n- Loops: {}\n- Tokens: {}\n",
            result.paladin_name,
            result.status,
            result.output,
            result.loop_count,
            result.token_usage.total_tokens
        )
    }
}
```

### Acceptance Criteria

- [ ] Results can be formatted as Markdown, JSON, or tables
- [ ] Metadata included in formatted output
- [ ] Battalion results show individual Paladin contributions

---

## Epic 9: Armory CLI Tools

### Overview

**Priority:** Medium  
**Effort:** 2-3 weeks  
**Dependencies:** Epics 1-4  
**Team:** 1 developer

**Objective:** Provide the Armory CLI for rapid Paladin development, testing, and deployment.

### Technical Design

**src/bin/paladin-cli.rs**

```rust
#[derive(Parser)]
#[command(name = "paladin")]
#[command(about = "Paladin Multi-Agent Orchestration CLI")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Paladin operations
    Agent {
        #[command(subcommand)]
        action: AgentCommands,
    },
    /// Battalion operations
    Battalion {
        #[command(subcommand)]
        action: BattalionCommands,
    },
    /// Arsenal tool management
    Arsenal {
        #[command(subcommand)]
        action: ArsenalCommands,
    },
}

#[derive(Subcommand)]
enum AgentCommands {
    /// Run a Paladin from configuration
    Run {
        #[arg(short, long)]
        config: PathBuf,
        #[arg(short, long)]
        input: Option<String>,
    },
    /// Create a new Paladin configuration
    New {
        #[arg(short, long)]
        name: String,
        #[arg(short, long)]
        output: PathBuf,
    },
}
```

### CLI Examples

```bash
# Run a Paladin from config
paladin agent run --config analyst.yaml --input "Analyze Q4 revenue"

# Execute a Formation
paladin battalion run --type formation --config workflow.yaml

# List available tools
paladin arsenal list

# Test an MCP server
paladin arsenal test --mcp-stdio "uvx mcp-hn"
```

### Acceptance Criteria

- [ ] Paladins can be defined and run via CLI
- [ ] Battalions can be orchestrated via CLI
- [ ] Configuration validation with helpful error messages
- [ ] Interactive mode for testing

---

## Epic 10: Validation & Documentation

### Overview

**Priority:** High  
**Effort:** 2-3 weeks  
**Dependencies:** All previous Epics  
**Team:** Full team

**Objective:** Comprehensive integration testing, performance validation, and documentation for production readiness.

### Deliverables

#### Integration Test Suite

- End-to-end Paladin execution tests
- Multi-Paladin Battalion integration tests
- MCP server integration tests
- Provider integration tests with mocks
- Load testing for concurrent Phalanx execution

#### Documentation

1. **API Reference** (rustdoc)
    
    - All public types documented
    - Examples for each major component
    - Error handling guidance
2. **User Guide**
    
    - Getting started tutorial
    - Paladin configuration guide
    - Battalion patterns cookbook
    - Tool integration guide
3. **Architecture Documentation**
    
    - System overview diagrams
    - Domain model documentation
    - Port/adapter mapping
    - Extension guide
4. **Examples Gallery**
    
    - Single Paladin examples
    - Formation workflow examples
    - Phalanx parallel processing
    - Campaign graph orchestration
    - Chain of Command delegation
    - MCP tool integration

### Acceptance Criteria

- [ ] Integration test coverage ≥ 70%
- [ ] All public APIs documented
- [ ] Examples compile and run successfully
- [ ] Performance benchmarks established
- [ ] Production deployment guide complete

---

## Project Schedule

### Phase 1: Foundation (Weeks 1-4)

- Epic 1: Paladin Domain Foundation

### Phase 2: Core Capabilities (Weeks 5-10)

- Epic 2: Garrison Memory System
- Epic 3: Arsenal Tool System
- Epic 6: Provider Expansion

### Phase 3: Orchestration (Weeks 8-14)

- Epic 4: Battalion Orchestration
- Epic 7: Citadel State Persistence

### Phase 4: Advanced Features (Weeks 13-18)

- Epic 5: Commander Strategy Router
- Epic 8: Herald Output Formatting
- Epic 9: Armory CLI Tools

### Phase 5: Production Ready (Weeks 17-20)

- Epic 10: Validation & Documentation

### Milestones

|Milestone|Target Week|Deliverables|
|---|---|---|
|M1: Alpha|Week 6|Single Paladin execution working|
|M2: Beta|Week 12|All Battalion types functional|
|M3: RC1|Week 18|Full feature complete|
|M4: Release|Week 20|Production ready|

---

## Risk Management

### Technical Risks

|Risk|Probability|Impact|Mitigation|
|---|---|---|---|
|MCP protocol complexity|Medium|High|Early spike on protocol implementation|
|LLM provider API changes|Low|Medium|Abstraction via ports, adapter versioning|
|Performance at scale|Medium|High|Early load testing, async optimization|
|Graph cycle detection|Low|Low|Use proven petgraph library|

### Schedule Risks

|Risk|Probability|Impact|Mitigation|
|---|---|---|---|
|Scope creep|Medium|High|Strict Epic boundaries, change control|
|Integration delays|Medium|Medium|Parallel development, mock services|
|Testing bottleneck|Low|Medium|TDD throughout, automated CI|

---

## Appendix A: Naming Glossary

|Term|Definition|
|---|---|
|**Paladin**|An autonomous AI agent capable of reasoning and action|
|**Battalion**|A coordinated group of Paladins working together|
|**Formation**|Sequential Paladin execution pattern|
|**Phalanx**|Concurrent Paladin execution pattern|
|**Campaign**|Graph-based Paladin orchestration|
|**Chain of Command**|Hierarchical Paladin delegation|
|**Commander**|Dynamic Battalion strategy router|
|**Garrison**|Paladin memory and context storage|
|**Arsenal**|Tool and capability registry|
|**Armament**|A single tool or capability|
|**Citadel**|State persistence and recovery system|
|**Herald**|Output formatting system|
|**Armory**|CLI tools for development|

---

## Appendix B: File Structure

```
src/
├── core/
│   └── platform/
│       └── container/
│           ├── paladin.rs
│           ├── paladin_config.rs
│           ├── garrison.rs
│           ├── arsenal.rs
│           ├── citadel.rs
│           └── battalion/
│               ├── mod.rs
│               ├── formation.rs
│               ├── phalanx.rs
│               ├── campaign.rs
│               └── chain_of_command.rs
├── application/
│   ├── ports/
│   │   └── output/
│   │       ├── paladin_port.rs
│   │       ├── garrison_port.rs
│   │       ├── arsenal_port.rs
│   │       ├── battalion_port.rs
│   │       └── citadel_port.rs
│   └── use_cases/
│       ├── paladin/
│       │   ├── mod.rs
│       │   ├── paladin_builder.rs
│       │   └── paladin_execution_service.rs
│       └── battalion/
│           ├── mod.rs
│           ├── formation_service.rs
│           ├── phalanx_service.rs
│           ├── campaign_service.rs
│           ├── chain_of_command_service.rs
│           └── commander.rs
├── infrastructure/
│   └── adapters/
│       ├── llm/
│       │   ├── openai_adapter.rs
│       │   ├── deepseek_adapter.rs
│       │   └── anthropic_adapter.rs
│       ├── garrison/
│       │   ├── in_memory_garrison.rs
│       │   └── sqlite_garrison.rs
│       ├── arsenal/
│       │   ├── mcp_client.rs
│       │   ├── mcp_stdio_adapter.rs
│       │   └── mcp_sse_adapter.rs
│       └── citadel/
│           └── file_citadel.rs
└── bin/
    └── paladin-cli.rs
```