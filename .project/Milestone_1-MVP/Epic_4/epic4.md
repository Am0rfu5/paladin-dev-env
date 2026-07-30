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
