
## Epic 17: Flow DSL & Agent Rearrangement

**Theme:** Flexible Workflow Definition  
**Duration:** 2 weeks  
**Priority:** Medium  
**Dependencies:** Epic 4, Epic 15, Epic 16  

### Description
Implement the AgentRearrange pattern with a simple string-based DSL for defining complex agent relationships, enabling flexible workflow definition without verbose configuration.

### User Stories

#### US-17.1: Flow DSL Parser
**As a** developer  
**I want** to define workflows with simple string syntax  
**So that** I can quickly express agent relationships

**Acceptance Criteria:**
- [ ] Parser for flow expressions like `"a -> b, c"` and `"a -> b -> c"`
- [ ] Supports: sequential (`->`), parallel (`,`), groups (`()`)
- [ ] `FlowExpression` AST representation
- [ ] Validation of agent names against registered agents
- [ ] Clear error messages for invalid syntax

**Definition of Done:**
```rust
// Supported syntax:
// "a -> b"           Sequential: a then b
// "a -> b, c"        Fan-out: a then b and c in parallel
// "a, b -> c"        Fan-in: a and b in parallel, then c
// "a -> (b -> c), d" Nested: a then (b->c) and d in parallel
// "a -> b -> c"      Chain: a then b then c

pub struct FlowParser;

impl FlowParser {
    pub fn parse(expression: &str) -> Result<FlowExpression, FlowParseError>;
}

pub enum FlowExpression {
    Agent(String),
    Sequential(Vec<FlowExpression>),
    Parallel(Vec<FlowExpression>),
}
```

---

#### US-17.2: Maneuver Domain Model (AgentRearrange)
**As a** framework developer  
**I want** domain models for flow-based orchestration  
**So that** the pattern has clear structure

**Acceptance Criteria:**
- [ ] `Maneuver` struct in `src/core/platform/container/battalion/maneuver.rs`
- [ ] Contains: agents map, flow expression, configuration
- [ ] Validates flow references valid agents
- [ ] `ManeuverConfig` with execution settings

**Definition of Done:**
```rust
pub struct Maneuver {
    pub name: String,
    pub agents: HashMap<String, Paladin>,
    pub flow: FlowExpression,
    pub config: ManeuverConfig,
}

pub struct ManeuverConfig {
    pub timeout_seconds: u64,
    pub error_strategy: ErrorStrategy,
    pub pass_output_as_input: bool,
}

impl Maneuver {
    pub fn new(
        name: impl Into<String>,
        agents: Vec<Paladin>,
        flow: &str,
    ) -> Result<Self, ManeuverError>;
}
```

---

#### US-17.3: Maneuver Execution Service
**As a** developer  
**I want** to execute flow-based workflows  
**So that** complex patterns run correctly

**Acceptance Criteria:**
- [ ] `ManeuverExecutionService` interprets flow expressions
- [ ] Executes sequential steps in order
- [ ] Executes parallel steps concurrently
- [ ] Handles nested expressions recursively
- [ ] Passes outputs between steps as configured

**Definition of Done:**
```rust
pub struct ManeuverExecutionService {
    paladin_port: Arc<dyn PaladinPort>,
}

impl ManeuverExecutionService {
    pub async fn execute(
        &self,
        maneuver: &Maneuver,
        input: &str,
    ) -> Result<ManeuverResult, BattalionError>;
    
    async fn execute_expression(
        &self,
        expr: &FlowExpression,
        agents: &HashMap<String, Paladin>,
        input: &str,
    ) -> Result<StepResult, BattalionError>;
}

pub struct ManeuverResult {
    pub final_output: String,
    pub step_outputs: HashMap<String, PaladinResult>,
    pub execution_order: Vec<String>,
}
```

---

#### US-17.4: Commander Maneuver Strategy
**As a** developer  
**I want** Commander to support flow-based workflows  
**So that** I can use the unified API

**Acceptance Criteria:**
- [ ] `BattalionStrategy::Maneuver` variant
- [ ] CommanderBuilder accepts flow expression
- [ ] Auto-strategy does NOT select Maneuver (explicit only)
- [ ] CLI support with flow syntax

**Definition of Done:**
```rust
impl CommanderBuilder {
    pub fn flow(mut self, expression: &str) -> Self;
}

// CLI usage:
// paladin battalion run --type maneuver --flow "researcher -> writer, editor -> reviewer"

// YAML config:
// type: maneuver
// flow: "researcher -> writer, editor -> reviewer"
// agents:
//   - name: researcher
//     ...
```

---

#### US-17.5: Flow Visualization
**As a** developer  
**I want** to visualize flow expressions  
**So that** I can understand and debug workflows

**Acceptance Criteria:**
- [ ] `FlowVisualizer` generates ASCII/Mermaid diagrams
- [ ] CLI command: `paladin battalion visualize --flow "..."`
- [ ] Shows execution order and parallelism
- [ ] Useful for documentation

**Definition of Done:**
```rust
pub struct FlowVisualizer;

impl FlowVisualizer {
    pub fn to_ascii(expr: &FlowExpression) -> String;
    pub fn to_mermaid(expr: &FlowExpression) -> String;
}

// Example ASCII output for "a -> b, c -> d":
// ┌───┐
// │ a │
// └─┬─┘
//   │
// ┌─┴─┬───┐
// │   │   │
// ▼   ▼   │
// ┌───┐ ┌───┐
// │ b │ │ c │
// └─┬─┘ └─┬─┘
//   │     │
//   └──┬──┘
//      │
//      ▼
//    ┌───┐
//    │ d │
//    └───┘
```

---

### Epic 17 Completion Criteria
- [ ] All 5 user stories completed and tested
- [ ] Flow DSL parser complete with tests
- [ ] Maneuver execution service functional
- [ ] Commander integration
- [ ] Visualization working
- [ ] Documentation in `docs/guides/flow-dsl.md`
- [ ] Example: `examples/maneuver_workflow.rs`

---
