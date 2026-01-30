## Epic 15: Conclave - MixtureOfAgents Pattern

**Theme:** Expert Synthesis Orchestration  
**Duration:** 2 weeks  
**Priority:** High  
**Dependencies:** Epic 4 (Battalion Orchestration)  

### Description
Implement the MixtureOfAgents (MoA) orchestration pattern where multiple expert agents process a task in parallel, and an aggregator agent synthesizes their outputs into a final, high-quality response.

### User Stories

#### US-15.1: Conclave Domain Model
**As a** framework developer  
**I want** domain models for MoA orchestration  
**So that** the pattern has clear structure

**Acceptance Criteria:**
- [ ] `Conclave` struct in `src/core/platform/container/battalion/conclave.rs`
- [ ] Contains: expert agents, aggregator agent, configuration
- [ ] `ConclaveConfig` with synthesis settings
- [ ] `ConclaveResult` with individual and aggregated outputs
- [ ] Validation: at least 2 experts, 1 aggregator

**Definition of Done:**
```rust
pub struct Conclave {
    pub name: String,
    pub experts: Vec<Paladin>,
    pub aggregator: Paladin,
    pub config: ConclaveConfig,
}

pub struct ConclaveConfig {
    pub name: String,
    pub timeout_seconds: u64,
    pub synthesis_prompt: Option<String>,
    pub include_expert_names: bool,
    pub max_expert_output_tokens: Option<usize>,
}

pub struct ConclaveResult {
    pub expert_outputs: HashMap<String, PaladinResult>,
    pub aggregated_output: PaladinResult,
    pub execution_time_ms: u64,
    pub status: ConclaveStatus,
}
```

---

#### US-15.2: Conclave Execution Service
**As a** developer  
**I want** to execute MoA workflows  
**So that** I get synthesized expert outputs

**Acceptance Criteria:**
- [ ] `ConclaveExecutionService` in `src/application/use_cases/battalion/`
- [ ] Executes all experts in parallel
- [ ] Collects outputs with error handling
- [ ] Formats outputs for aggregator
- [ ] Executes aggregator with expert context
- [ ] Returns combined result

**Definition of Done:**
```rust
pub struct ConclaveExecutionService {
    paladin_port: Arc<dyn PaladinPort>,
}

impl ConclaveExecutionService {
    pub async fn execute(
        &self,
        conclave: &Conclave,
        input: &str,
    ) -> Result<ConclaveResult, BattalionError>;
    
    fn format_expert_outputs_for_aggregator(
        &self,
        outputs: &HashMap<String, PaladinResult>,
        config: &ConclaveConfig,
    ) -> String;
}

// Aggregator receives prompt like:
// "You are synthesizing outputs from multiple experts.
//  Expert 'Analyst': {output1}
//  Expert 'Researcher': {output2}
//  Expert 'Writer': {output3}
//  
//  Synthesize these into a comprehensive response."
```

---

#### US-15.3: Commander Conclave Strategy
**As a** developer  
**I want** Commander to support Conclave strategy  
**So that** I can use unified orchestration API

**Acceptance Criteria:**
- [ ] `BattalionStrategy::Conclave` variant added
- [ ] Commander builds Conclave from configuration
- [ ] Designates last agent as aggregator by default
- [ ] Configurable aggregator selection
- [ ] Auto-strategy considers Conclave for expert scenarios

**Definition of Done:**
```rust
pub enum BattalionStrategy {
    Formation,
    Phalanx,
    Campaign,
    ChainOfCommand,
    Conclave,  // NEW
    Auto,
}

impl CommanderBuilder {
    pub fn aggregator(mut self, paladin: Paladin) -> Self;
}

// Auto-strategy heuristics for Conclave:
// - Multiple agents with distinct descriptions
// - Task requires synthesis/comparison
// - Keywords: "compare", "synthesize", "combine perspectives"
```

---

#### US-15.4: Conclave CLI Support
**As a** developer  
**I want** to run Conclave from CLI  
**So that** I can use MoA without code

**Acceptance Criteria:**
- [ ] `paladin battalion run --type conclave --config conclave.yaml`
- [ ] YAML schema for Conclave configuration
- [ ] Aggregator specified in YAML
- [ ] Output includes expert outputs and synthesis
- [ ] Template generation: `paladin battalion new --type conclave`

**Definition of Done:**
```yaml
# conclave.yaml
type: conclave
name: "ExpertPanel"

aggregator:
  inline:
    name: "Synthesizer"
    system_prompt: "Combine expert opinions into comprehensive analysis"
    model: "gpt-4"

experts:
  - inline:
      name: "TechnicalExpert"
      system_prompt: "Provide technical analysis"
      model: "gpt-4"
  
  - inline:
      name: "BusinessExpert"  
      system_prompt: "Provide business perspective"
      model: "gpt-4"
  
  - inline:
      name: "RiskExpert"
      system_prompt: "Identify risks and concerns"
      model: "gpt-4"

config:
  timeout_seconds: 300
  include_expert_names: true
```

---

### Epic 15 Completion Criteria
- [ ] All 4 user stories completed and tested
- [ ] Conclave domain model and execution service
- [ ] Commander integration with Conclave strategy
- [ ] CLI and YAML support
- [ ] Documentation in `docs/guides/conclave-pattern.md`
- [ ] Example: `examples/conclave_expert_panel.rs`
- [ ] Example: `examples/cli_configs/conclave.yaml`

---
