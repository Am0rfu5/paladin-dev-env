## Epic 14: Autonomous Agent Features

**Theme:** Agent Self-Direction and Planning  
**Duration:** 2 weeks  
**Priority:** Critical  
**Dependencies:** Epic 1 (Paladin Domain)  

### Description
Implement advanced agent autonomy features including automatic planning mode, dynamic temperature adjustment, auto-generated prompts, and agent handoffs for dynamic task delegation.

### User Stories

#### US-14.1: Autonomous Planning Mode
**As a** developer  
**I want** Paladins to automatically plan and execute subtasks  
**So that** complex tasks are handled without manual decomposition

**Acceptance Criteria:**
- [ ] `MaxLoops::Auto` enum variant supported
- [ ] `PlanningService` decomposes tasks into subtasks
- [ ] Subtasks executed sequentially with dependency tracking
- [ ] Planning uses dedicated planning prompt
- [ ] Final summary synthesizes all subtask results
- [ ] Configurable max subtasks limit

**Definition of Done:**
```rust
pub enum MaxLoops {
    Fixed(u32),
    Auto { max_subtasks: u32 },
}

pub struct PlanningService {
    llm: Arc<dyn LlmPort>,
}

impl PlanningService {
    pub async fn create_plan(&self, task: &str) -> Result<TaskPlan, PlanningError>;
    pub async fn execute_plan(&self, plan: TaskPlan, paladin: &Paladin) -> Result<PlanResult, PlanningError>;
}

pub struct TaskPlan {
    pub original_task: String,
    pub subtasks: Vec<Subtask>,
    pub dependencies: HashMap<usize, Vec<usize>>,
}

pub struct Subtask {
    pub id: usize,
    pub description: String,
    pub expected_output: String,
}
```

---

#### US-14.2: Auto-Generate System Prompt
**As a** developer  
**I want** automatic system prompt generation  
**So that** I can create agents without manual prompt engineering

**Acceptance Criteria:**
- [ ] `PaladinBuilder::auto_generate_prompt(true)` flag
- [ ] Uses agent name and description to generate prompt
- [ ] `PromptGenerationService` creates contextual prompts
- [ ] Generated prompts include role, capabilities, constraints
- [ ] Caching of generated prompts for reuse

**Definition of Done:**
```rust
impl PaladinBuilder {
    pub fn auto_generate_prompt(mut self, enabled: bool) -> Self;
    pub fn agent_description(mut self, desc: impl Into<String>) -> Self;
}

pub struct PromptGenerationService {
    llm: Arc<dyn LlmPort>,
}

impl PromptGenerationService {
    pub async fn generate_prompt(
        &self,
        agent_name: &str,
        agent_description: &str,
        task_context: Option<&str>,
    ) -> Result<String, PromptError>;
}

// Example generated prompt:
// "You are {agent_name}, an AI assistant specialized in {description}.
//  Your role is to {capabilities}. You should {constraints}."
```

---

#### US-14.3: Dynamic Temperature Adjustment
**As a** developer  
**I want** automatic temperature adjustment based on task  
**So that** responses are appropriately creative or precise

**Acceptance Criteria:**
- [ ] `PaladinBuilder::dynamic_temperature(true)` flag
- [ ] `TemperatureService` analyzes task complexity
- [ ] Lower temperature for: factual queries, code, math
- [ ] Higher temperature for: creative writing, brainstorming
- [ ] Temperature bounds: 0.1 - 1.0
- [ ] Logging of temperature decisions

**Definition of Done:**
```rust
impl PaladinBuilder {
    pub fn dynamic_temperature(mut self, enabled: bool) -> Self;
    pub fn temperature_bounds(mut self, min: f32, max: f32) -> Self;
}

pub struct TemperatureService;

impl TemperatureService {
    pub fn analyze_task(&self, task: &str) -> TaskType;
    pub fn recommend_temperature(&self, task_type: TaskType) -> f32;
}

pub enum TaskType {
    Factual,      // 0.1 - 0.3
    Analytical,   // 0.3 - 0.5
    Conversational, // 0.5 - 0.7
    Creative,     // 0.7 - 1.0
}
```

---

#### US-14.4: Agent Handoff Infrastructure
**As a** developer  
**I want** to configure agents that can delegate to specialists  
**So that** complex tasks route to appropriate experts

**Acceptance Criteria:**
- [ ] `PaladinBuilder::with_handoffs(agents)` method
- [ ] `HandoffService` routes tasks to specialists
- [ ] Handoff decision based on task analysis
- [ ] Handoff includes context transfer
- [ ] Maximum handoff depth to prevent loops
- [ ] Handoff history tracked in result

**Definition of Done:**
```rust
impl PaladinBuilder {
    pub fn with_handoffs(mut self, agents: Vec<Arc<Paladin>>) -> Self;
    pub fn handoff_strategy(mut self, strategy: HandoffStrategy) -> Self;
}

pub enum HandoffStrategy {
    Automatic,  // LLM decides when to handoff
    Explicit,   // Only handoff when tool called
    Threshold { confidence: f32 }, // Handoff when confidence low
}

pub struct HandoffService {
    llm: Arc<dyn LlmPort>,
}

impl HandoffService {
    pub async fn should_handoff(
        &self,
        task: &str,
        current_agent: &Paladin,
        available_agents: &[Arc<Paladin>],
    ) -> Option<HandoffDecision>;

    pub async fn execute_handoff(
        &self,
        decision: HandoffDecision,
        context: HandoffContext,
    ) -> Result<PaladinResult, HandoffError>;
}

pub struct HandoffDecision {
    pub target_agent: Arc<Paladin>,
    pub reason: String,
    pub context_to_transfer: String,
}
```

---

#### US-14.5: Handoff Tool for Agents
**As a** developer  
**I want** agents to use a handoff tool  
**So that** handoffs can happen during execution

**Acceptance Criteria:**
- [ ] `handoff_to_agent` tool registered automatically
- [ ] Tool schema includes target agent and message
- [ ] Tool execution triggers handoff service
- [ ] Result returned to original agent for synthesis
- [ ] Handoff chain visible in execution trace

**Definition of Done:**
```rust
// Automatically registered tool when handoffs configured
pub struct HandoffTool {
    available_agents: Vec<AgentInfo>,
    handoff_service: Arc<HandoffService>,
}

impl HandoffTool {
    pub fn schema(&self) -> Armament {
        Armament {
            name: "handoff_to_agent".to_string(),
            description: "Transfer this task to a specialized agent".to_string(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "agent_name": {
                        "type": "string",
                        "enum": self.available_agents.iter().map(|a| &a.name).collect::<Vec<_>>()
                    },
                    "message": {
                        "type": "string",
                        "description": "Context and task for the target agent"
                    }
                },
                "required": ["agent_name", "message"]
            }),
            required_params: vec!["agent_name".to_string(), "message".to_string()],
        }
    }
}
```

---

### Epic 14 Completion Criteria
- [ ] All 5 user stories completed and tested
- [ ] `max_loops="auto"` planning mode functional
- [ ] Auto-generate prompt working
- [ ] Dynamic temperature adjustment working
- [ ] Handoff infrastructure complete
- [ ] Documentation in `docs/AUTONOMOUS.md`
- [ ] Example: `examples/autonomous_planning.rs`
- [ ] Example: `examples/agent_handoffs.rs`

---
