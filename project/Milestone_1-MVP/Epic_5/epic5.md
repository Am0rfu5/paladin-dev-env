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
