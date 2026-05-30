# Autonomous Agent Features

> **Epic 14: Autonomous Agent Features** - Advanced AI capabilities for intelligent task handling and agent collaboration

## Table of Contents

1. [Introduction](#introduction)
2. [Autonomous Planning Mode](#autonomous-planning-mode)
3. [Auto-Generate System Prompts](#auto-generate-system-prompts)
4. [Dynamic Temperature Adjustment](#dynamic-temperature-adjustment)
5. [Agent Handoff Infrastructure](#agent-handoff-infrastructure)
6. [Handoff Tool](#handoff-tool)
7. [Configuration](#configuration)
8. [Best Practices](#best-practices)
9. [Error Handling](#error-handling)
10. [Troubleshooting](#troubleshooting)
11. [Advanced Usage](#advanced-usage)
12. [API Reference](#api-reference)

---

## Introduction

Paladin's autonomous agent features enable AI agents to intelligently handle complex tasks with minimal human intervention. These features are designed to make agents more capable, adaptive, and collaborative.

### Features Overview

| Feature | Purpose | Status |
|---------|---------|--------|
| **Autonomous Planning** | Decompose complex tasks into subtasks automatically | ✅ Available |
| **Auto-Generate Prompts** | Dynamically create system prompts based on agent role | ✅ Available |
| **Dynamic Temperature** | Adjust creativity based on task type | ✅ Available |
| **Agent Handoffs** | Delegate tasks to specialist agents | ✅ Available |
| **Handoff Tool** | Mid-execution agent delegation via LLM tool calls | ✅ Available |

### Key Benefits

- **Reduced Configuration Overhead**: Less manual prompt engineering and parameter tuning
- **Improved Task Handling**: Automatic decomposition of complex tasks
- **Adaptive Behavior**: Temperature adjusts to task requirements
- **Specialization**: Delegate tasks to expert agents
- **Opt-In Design**: All features disabled by default for backward compatibility

### Quick Start

```rust
use paladin::application::services::paladin::paladin_builder::PaladinBuilder;
use paladin::core::platform::container::paladin::MaxLoops;
use paladin::core::platform::container::autonomous_config::*;
use std::sync::Arc;

// Create autonomous configuration
let autonomous_config = AutonomousConfig {
    planning: PlanningConfig {
        enabled: true,
        max_subtasks: 15,
    },
    prompt_generation: PromptConfig {
        enabled: true,
        description: Some("Expert data analyst".to_string()),
    },
    dynamic_temperature: TemperatureConfig {
        enabled: true,
        min: 0.2,
        max: 0.85,
    },
    handoffs: HandoffConfig {
        enabled: true,
        strategy: HandoffStrategy::Automatic,
        max_depth: 5,
    },
};

// Build Paladin with autonomous features
let paladin = PaladinBuilder::new(llm_port)
    .name("data-analyst")
    .max_loops(MaxLoops::Auto) // Autonomous planning
    .agent_description("Expert data analyst specializing in financial reports")
    .auto_generate_prompt(true) // Auto-generate system prompt
    .dynamic_temperature(true)  // Adjust temperature dynamically
    .enable_handoffs()          // Enable delegation
    .build()
    .await?;
```

---

## Autonomous Planning Mode

**User Story US-14.1**: Autonomous planning mode allows agents to decompose complex tasks into manageable subtasks automatically.

### Concept

When `MaxLoops::Auto` is set, the Paladin uses an LLM-powered planning service to:
1. Analyze the input task
2. Decompose it into logical subtasks
3. Execute each subtask sequentially
4. Synthesize results into a final answer

This eliminates the need to manually determine iteration counts or break down complex workflows.

### Use Cases

- **Research Tasks**: "Analyze competitor landscape and provide strategic recommendations"
- **Data Analysis**: "Load dataset, clean data, perform statistical analysis, and visualize results"
- **Content Generation**: "Research topic, create outline, write article, add citations"
- **Code Development**: "Design API, implement endpoints, write tests, document usage"

### Configuration

```rust
use paladin::core::platform::container::paladin::MaxLoops;
use paladin::core::platform::container::autonomous_config::PlanningConfig;

// Enable autonomous planning
let paladin = PaladinBuilder::new(llm_port)
    .name("research-agent")
    .max_loops(MaxLoops::Auto) // Enables planning mode
    .build()
    .await?;

// Or configure via AutonomousConfig
let planning_config = PlanningConfig {
    enabled: true,
    max_subtasks: 15, // Maximum subtasks to create (1-100)
};
```

**YAML Configuration**:
```yaml
autonomous:
  planning:
    enabled: true
    max_subtasks: 15
```

**CLI Flag**:
```bash
paladin agent run --config agent.yaml --input "Research topic" --auto-plan
```

### How It Works

#### 1. Planning Phase
The PlanningService sends a specialized prompt to the LLM:
```
You are a task planner. Decompose the following complex task into 
logical subtasks that can be executed sequentially.

Task: [User input]

Provide a structured plan with:
- Clear subtask descriptions
- Expected outcomes
- Dependencies between subtasks
```

#### 2. Decomposition
The LLM returns a `TaskPlan` structure:
```rust
pub struct TaskPlan {
    pub subtasks: Vec<Subtask>,
    pub estimated_loops: u32,
}

pub struct Subtask {
    pub id: String,
    pub description: String,
    pub expected_outcome: String,
    pub dependencies: Vec<String>,
}
```

#### 3. Execution
Each subtask is executed in sequence:
- Previous subtask results are included in context
- Dependencies are resolved automatically
- Loop count is set to `estimated_loops`

#### 4. Synthesis
Final loop synthesizes all subtask results into a cohesive answer.

### Example Output

**Input**: "Analyze the performance of our web application"

**Generated Plan**:
1. **Identify Metrics**: Define key performance indicators (response time, throughput, error rate)
2. **Collect Data**: Gather performance logs and metrics from monitoring systems
3. **Analyze Trends**: Identify patterns, bottlenecks, and anomalies in the data
4. **Generate Recommendations**: Provide actionable suggestions for optimization
5. **Summarize Findings**: Create executive summary with key insights

**Execution**: Each subtask runs sequentially, final output synthesizes all results.

---

## Auto-Generate System Prompts

**User Story US-14.2**: Automatically generate contextual system prompts based on agent description.

### Concept

Instead of manually writing system prompts, provide a high-level description of the agent's role and capabilities. The PromptGenerationService uses an LLM to create an optimized system prompt.

### Benefits

- **Consistency**: All agents have well-structured prompts
- **Expertise**: Leverage LLM's knowledge of effective prompt patterns
- **Time Savings**: No manual prompt engineering required
- **Adaptability**: Prompts optimized for specific agent roles

### Configuration

```rust
// Enable auto-generation in builder
let paladin = PaladinBuilder::new(llm_port)
    .name("code-reviewer")
    .agent_description("Expert code reviewer specializing in Rust, security, and performance")
    .auto_generate_prompt(true) // Enable auto-generation
    .build()
    .await?;

// Manual system prompt takes precedence
let paladin_manual = PaladinBuilder::new(llm_port)
    .name("custom-agent")
    .system_prompt("Custom prompt...") // Manual override
    .agent_description("Description used only if prompt not set")
    .auto_generate_prompt(true)
    .build()
    .await?;
```

**YAML Configuration**:
```yaml
autonomous:
  prompt_generation:
    enabled: true
    description: "Expert code reviewer specializing in Rust, security, and performance"
```

**CLI Flag**:
```bash
paladin agent run --config agent.yaml --input "Review this code" --auto-prompt
```

### How It Works

#### 1. Generation Request
The PromptGenerationService sends a meta-prompt:
```
Create an effective system prompt for an AI agent with the following role:

Agent Name: code-reviewer
Description: Expert code reviewer specializing in Rust, security, and performance

The prompt should:
- Clearly define the agent's expertise and responsibilities
- Establish appropriate tone and communication style
- Include relevant guidelines and best practices
- Be concise yet comprehensive (2-4 paragraphs)
```

#### 2. LLM Response
The LLM generates a contextual system prompt:
```
You are an expert code reviewer with deep expertise in Rust programming, 
security analysis, and performance optimization. Your role is to provide 
thorough, constructive code reviews that identify issues and suggest 
improvements.

When reviewing code:
1. Check for security vulnerabilities (unsafe code, input validation, etc.)
2. Analyze performance implications (algorithmic complexity, allocations)
3. Ensure idiomatic Rust patterns (ownership, borrowing, error handling)
4. Verify code clarity and maintainability

Provide specific, actionable feedback with code examples where helpful.
```

#### 3. Caching
Generated prompts are cached using a deterministic hash:
```rust
let cache_key = format!("{}:{}", agent_name, description_hash);
```

This prevents redundant LLM calls for identical agent configurations.

### Regeneration

```rust
// Clear cache and regenerate
let prompt_service = PromptGenerationService::new(llm_port);
prompt_service.invalidate_cache("agent-name", "description-hash").await?;

let new_prompt = prompt_service.generate_prompt("agent-name", "Updated description").await?;
```

### Manual Override Pattern

```rust
// Provide fallback but allow override
let paladin = PaladinBuilder::new(llm_port)
    .name("analyst")
    .agent_description("Financial data analyst") // Used if no manual prompt
    .auto_generate_prompt(true)
    .build()
    .await?;

// Check if prompt was generated
if paladin.data().system_prompt.is_empty() {
    eprintln!("Warning: No system prompt generated or provided");
}
```

---

## Dynamic Temperature Adjustment

**User Story US-14.3**: Automatically adjust LLM temperature based on task type (factual vs. creative).

### Concept

Different tasks require different levels of creativity:
- **Factual tasks** (calculations, data retrieval) → Low temperature (0.1-0.3)
- **Analytical tasks** (analysis, reasoning) → Medium temperature (0.5-0.7)
- **Creative tasks** (writing, brainstorming) → High temperature (0.7-0.9)

The TemperatureService classifies tasks and recommends optimal temperature.

### Task Type Classification

| Task Type | Temperature Range | Examples |
|-----------|------------------|----------|
| **Factual** | 0.1 - 0.3 | Math calculations, data lookups, API calls |
| **Analytical** | 0.4 - 0.6 | Code review, debugging, data analysis |
| **Conversational** | 0.6 - 0.7 | Chat, Q&A, general assistance |
| **Creative** | 0.7 - 0.9 | Writing, brainstorming, design |

### Configuration

```rust
// Enable dynamic temperature
let paladin = PaladinBuilder::new(llm_port)
    .name("versatile-agent")
    .agent_description("Multi-purpose agent for varied tasks")
    .dynamic_temperature(true) // Enable dynamic adjustment
    .temperature_bounds(0.2, 0.85) // Optional: set bounds
    .build()
    .await?;

// Or via AutonomousConfig
let temp_config = TemperatureConfig {
    enabled: true,
    min: 0.2,
    max: 0.85,
};
```

**YAML Configuration**:
```yaml
autonomous:
  dynamic_temperature:
    enabled: true
    min: 0.2
    max: 0.85
```

**CLI Flag**:
```bash
paladin agent run --config agent.yaml --input "Task" --dynamic-temp
```

### Classification Heuristics

The TemperatureService uses keyword analysis and LLM classification:

#### Keyword-Based (Fast)
```rust
// Factual indicators
if task.contains_any(&["calculate", "compute", "count", "sum"]) {
    return TaskType::Factual;
}

// Creative indicators
if task.contains_any(&["write", "create", "imagine", "design"]) {
    return TaskType::Creative;
}
```

#### LLM-Based (Accurate)
For ambiguous tasks, the service queries the LLM:
```
Classify this task as: Factual, Analytical, Conversational, or Creative

Task: [User input]

Consider:
- Does it require precise, deterministic output? (Factual)
- Does it involve reasoning and analysis? (Analytical)
- Is it general conversation? (Conversational)
- Does it benefit from creative variation? (Creative)

Respond with only the classification.
```

### How It Works

#### 1. Task Analysis
```rust
let task_type = temperature_service
    .detect_task_type_with_llm(task_description)
    .await?;
```

#### 2. Temperature Calculation
```rust
let temperature = match task_type {
    TaskType::Factual => config.min.max(0.2),
    TaskType::Analytical => (config.min + config.max) / 2.0,
    TaskType::Conversational => (config.min + config.max) / 2.0 + 0.1,
    TaskType::Creative => config.max.min(0.85),
};
```

#### 3. Application
Temperature is applied before LLM request:
```rust
let request = LlmRequest {
    model: "gpt-4",
    temperature, // Dynamically calculated
    messages: vec![...],
};
```

### Example

**Input**: "Calculate the compound interest on $10,000 at 5% for 10 years"
- **Classification**: Factual
- **Temperature**: 0.2 (deterministic, precise)

**Input**: "Write a creative short story about a time traveler"
- **Classification**: Creative
- **Temperature**: 0.85 (varied, imaginative)

---

## Agent Handoff Infrastructure

**User Story US-14.4**: Enable agents to delegate tasks to specialist agents.

### Concept

A general-purpose agent can recognize when a task requires specialized expertise and hand it off to a specialist agent. The specialist executes the task and returns results to the original agent.

### Delegation Patterns

1. **Automatic**: Agent decides when to delegate based on context
2. **Explicit**: Developer specifies handoff points programmatically
3. **Threshold-Based**: Delegate when confidence drops below threshold

### Configuration

```rust
use paladin::core::platform::container::handoff::*;

// Build agent with handoff support
let main_agent = PaladinBuilder::new(llm_port)
    .name("general-assistant")
    .enable_handoffs() // Enable handoff infrastructure
    .handoff_strategy(HandoffStrategy::Automatic)
    .max_handoff_depth(5) // Prevent infinite delegation chains
    .build()
    .await?;

// Register specialist agents
let handoff_service = HandoffService::new(llm_port);
handoff_service.register_specialist(
    "code-expert",
    "Rust programming expert for code generation and debugging"
).await?;

handoff_service.register_specialist(
    "data-analyst",
    "Expert in data analysis, statistics, and visualization"
).await?;
```

**YAML Configuration**:
```yaml
autonomous:
  handoffs:
    enabled: true
    strategy: "automatic"  # or "explicit" or {"threshold": 0.8}
    max_depth: 5
```

**CLI Flag**:
```bash
paladin agent run --config agent.yaml --input "Task" --enable-handoffs
```

### HandoffStrategy Options

#### 1. Automatic
Agent decides when to delegate based on task complexity and expertise:
```rust
HandoffStrategy::Automatic
```

#### 2. Explicit
Developer controls handoffs programmatically:
```rust
HandoffStrategy::Explicit
```

#### 3. Threshold
Delegate when confidence drops below threshold:
```rust
HandoffStrategy::threshold(0.75) // Delegate if confidence < 75%
```

### Circular Handoff Prevention

The HandoffService prevents infinite delegation loops:

```rust
// Validation in should_handoff()
if handoff_chain.contains(&target_agent) {
    return Err(HandoffError::CircularHandoff {
        chain: handoff_chain.clone(),
        attempted_target: target_agent.to_string(),
    });
}
```

**Example**:
- Agent A → Agent B → Agent C ✅ Valid
- Agent A → Agent B → Agent A ❌ Circular (rejected)

### Max Depth Configuration

Prevent unbounded delegation chains:

```rust
let handoff_config = HandoffConfig {
    enabled: true,
    strategy: HandoffStrategy::Automatic,
    max_depth: 5, // Maximum 5 hops
};
```

**Example**:
- A → B → C → D → E ✅ Depth 5 (allowed)
- A → B → C → D → E → F ❌ Depth 6 (rejected)

### Context Transfer

When handing off, context is preserved and transferred:

```rust
pub struct HandoffContext {
    pub original_task: String,
    pub accumulated_results: Vec<String>,
    pub handoff_chain: Vec<String>,
    pub depth: u32,
    pub metadata: HashMap<String, String>,
}
```

The specialist receives:
- Original task description
- All previous agent outputs
- Current position in handoff chain
- Any custom metadata

### Decision Process

```rust
// HandoffService determines if handoff is needed
let decision = handoff_service
    .should_handoff(task, current_agent, context)
    .await?;

match decision {
    HandoffDecision::Complete => {
        // Task can be completed by current agent
    }
    HandoffDecision::Handoff { target_agent, reason } => {
        // Delegate to specialist
        let result = execute_handoff(target_agent, task).await?;
    }
}
```

---

## Handoff Tool

**User Story US-14.5**: Mid-execution agent delegation via LLM tool calls.

### Concept

The `handoff_to_agent` tool is automatically registered with agents that have handoffs enabled. During execution, the LLM can invoke this tool to delegate tasks to specialists.

### Tool Schema

```json
{
  "name": "handoff_to_agent",
  "description": "Delegate the current task to a specialist agent when their expertise is needed",
  "parameters": {
    "type": "object",
    "properties": {
      "agent_name": {
        "type": "string",
        "enum": ["code-expert", "data-analyst", "security-specialist"],
        "description": "Name of the specialist agent to hand off to"
      },
      "message": {
        "type": "string",
        "description": "Clear task description for the specialist agent"
      }
    },
    "required": ["agent_name", "message"]
  }
}
```

**Note**: The `enum` values for `agent_name` are dynamically populated based on registered specialists.

### Example LLM Tool Call

When the LLM recognizes specialized expertise is needed:

```json
{
  "tool_calls": [
    {
      "id": "call_abc123",
      "type": "function",
      "function": {
        "name": "handoff_to_agent",
        "arguments": "{\"agent_name\": \"code-expert\", \"message\": \"Review this Rust function for memory safety issues: fn process_data(data: Vec<u8>) { ... }\"}"
      }
    }
  ]
}
```

### Auto-Registration

The HandoffTool is automatically registered when handoffs are enabled:

```rust
// Automatic registration in PaladinBuilder
if self.handoffs_enabled {
    let handoff_tool = HandoffTool::new(
        self.specialist_list.clone(),
        self.handoff_service.clone()
    );
    arsenal.register_tool(Arc::new(handoff_tool)).await?;
}
```

No manual tool registration required!

### Error Scenarios

#### 1. Invalid Agent
```json
{"agent_name": "nonexistent-agent", "message": "..."}
```
**Error**: `HandoffError::InvalidAgent`
```
Error: Agent 'nonexistent-agent' is not registered as a specialist.
Available agents: code-expert, data-analyst, security-specialist
```

#### 2. Circular Handoff
```
general-agent → code-expert → general-agent (attempt)
```
**Error**: `HandoffError::CircularHandoff`
```
Error: Circular handoff detected.
Chain: general-agent → code-expert → general-agent
Cannot hand back to an agent already in the chain.
```

#### 3. Max Depth Exceeded
```
A → B → C → D → E → F (max_depth = 5)
```
**Error**: `HandoffError::MaxDepthExceeded`
```
Error: Maximum handoff depth (5) exceeded.
Current chain: A → B → C → D → E → F
```

### Execution Flow

1. **LLM invokes tool**: `handoff_to_agent(agent_name="code-expert", message="...")`
2. **Validation**: Check agent exists, no circular handoff, depth OK
3. **Context preparation**: Build HandoffContext with chain history
4. **Specialist execution**: Target agent receives task and context
5. **Result return**: Specialist output returned to original agent
6. **Continuation**: Original agent incorporates result and continues

---

## Configuration

Autonomous features can be configured via YAML files, CLI flags, or the Builder API.

### YAML Configuration

Complete example (`config.yml`):

```yaml
autonomous:
  # Autonomous Planning (US-14.1)
  planning:
    enabled: true
    max_subtasks: 15
  
  # Auto-Generate System Prompt (US-14.2)
  prompt_generation:
    enabled: true
    description: "Expert data analyst specializing in financial reports"
  
  # Dynamic Temperature Adjustment (US-14.3)
  dynamic_temperature:
    enabled: true
    min: 0.2
    max: 0.85
  
  # Agent Handoff (US-14.4 & US-14.5)
  handoffs:
    enabled: true
    strategy: "automatic"  # Options: "automatic", "explicit", {"threshold": 0.8}
    max_depth: 5
```

### CLI Flags

```bash
# Enable all autonomous features
paladin agent run \
  --config agent.yaml \
  --input "Complex task" \
  --auto-plan \
  --auto-prompt \
  --dynamic-temp \
  --enable-handoffs

# Enable specific features
paladin agent run \
  --config agent.yaml \
  --input "Calculate compound interest" \
  --dynamic-temp  # Only dynamic temperature
```

### Builder API

```rust
use paladin::application::services::paladin::paladin_builder::PaladinBuilder;
use paladin::core::platform::container::paladin::MaxLoops;
use paladin::core::platform::container::autonomous_config::*;

let autonomous_config = AutonomousConfig {
    planning: PlanningConfig {
        enabled: true,
        max_subtasks: 15,
    },
    prompt_generation: PromptConfig {
        enabled: true,
        description: Some("Financial analyst".to_string()),
    },
    dynamic_temperature: TemperatureConfig {
        enabled: true,
        min: 0.2,
        max: 0.85,
    },
    handoffs: HandoffConfig {
        enabled: true,
        strategy: HandoffStrategy::Automatic,
        max_depth: 5,
    },
};

let paladin = PaladinBuilder::new(llm_port)
    .name("analyst")
    
    // Method 1: Individual feature methods
    .max_loops(MaxLoops::Auto)
    .agent_description("Financial analyst")
    .auto_generate_prompt(true)
    .dynamic_temperature(true)
    .temperature_bounds(0.2, 0.85)
    .enable_handoffs()
    .handoff_strategy(HandoffStrategy::Automatic)
    .max_handoff_depth(5)
    
    // Method 2: Configuration object
    // .with_autonomous_config(autonomous_config)
    
    .build()
    .await?;
```

### Configuration Precedence

When multiple configuration sources are present:

**Precedence Order** (highest to lowest):
1. **CLI Flags**: `--auto-plan`, `--auto-prompt`, etc.
2. **Builder API**: Explicit method calls
3. **YAML Configuration**: `config.yml` file
4. **Defaults**: All features disabled

**Example**:
```rust
// YAML says planning disabled
// Builder says planning enabled
let paladin = PaladinBuilder::new(llm_port)
    .load_config_from_yaml("config.yml")  // planning: enabled: false
    .max_loops(MaxLoops::Auto)            // Builder overrides YAML
    .build().await?;
// Result: Planning is ENABLED (builder takes precedence)
```

### Environment Variables

Override configuration via environment variables:

```bash
# Planning
export APP_AUTONOMOUS_PLANNING_ENABLED=true
export APP_AUTONOMOUS_PLANNING_MAX_SUBTASKS=20

# Prompt Generation
export APP_AUTONOMOUS_PROMPT_GENERATION_ENABLED=true
export APP_AUTONOMOUS_PROMPT_GENERATION_DESCRIPTION="Expert coder"

# Dynamic Temperature
export APP_AUTONOMOUS_DYNAMIC_TEMPERATURE_ENABLED=true
export APP_AUTONOMOUS_DYNAMIC_TEMPERATURE_MIN=0.2
export APP_AUTONOMOUS_DYNAMIC_TEMPERATURE_MAX=0.8

# Handoffs
export APP_AUTONOMOUS_HANDOFFS_ENABLED=true
export APP_AUTONOMOUS_HANDOFFS_STRATEGY=explicit
export APP_AUTONOMOUS_HANDOFFS_MAX_DEPTH=10
```

---

## Best Practices

### When to Use Each Feature

#### Autonomous Planning
✅ **Use when**:
- Task is complex and multi-step
- Breaking down manually is time-consuming
- Workflow is exploratory (research, analysis)

❌ **Avoid when**:
- Task is simple and single-step
- Exact workflow is known and fixed
- Real-time performance is critical (adds planning overhead)

#### Auto-Generate Prompts
✅ **Use when**:
- Creating many agents with similar roles
- Standardizing prompt quality across agents
- Experimenting with new agent configurations

❌ **Avoid when**:
- Highly specialized prompts requiring domain expertise
- Production agents where prompt is carefully tuned
- Prompt generation costs are a concern

#### Dynamic Temperature
✅ **Use when**:
- Agent handles diverse task types
- Task type varies per execution
- Optimal temperature is unknown

❌ **Avoid when**:
- Agent has single, consistent task type
- Temperature is already well-tuned
- Task classification overhead is unacceptable

#### Agent Handoffs
✅ **Use when**:
- Multiple specialized agents exist
- Tasks require varied expertise
- Collaboration improves outcomes

❌ **Avoid when**:
- Single agent can handle all tasks
- Handoff overhead exceeds benefits
- Linear workflow is more efficient

### Performance Considerations

#### Token Usage
- **Planning**: Adds ~500-1500 tokens for plan generation
- **Prompt Generation**: One-time cost of ~300-800 tokens (cached)
- **Temperature Classification**: ~200-400 tokens per classification
- **Handoffs**: ~200 tokens per handoff decision + specialist execution

**Optimization**:
```rust
// Use planning selectively
let use_planning = task_length > 100 || task_complexity > 0.7;
let max_loops = if use_planning {
    MaxLoops::Auto
} else {
    MaxLoops::Fixed(3)
};
```

#### Latency
- **Planning**: +1-3 seconds for plan generation
- **Prompt Generation**: +0.5-2 seconds (only on first execution)
- **Temperature Classification**: +0.3-1 second per task
- **Handoffs**: +LLM latency per hop (2-5 seconds typical)

**Optimization**:
```rust
// Disable features for latency-sensitive tasks
if real_time_required {
    builder.dynamic_temperature(false);
    builder.max_loops(MaxLoops::Fixed(1));
}
```

#### Cost Management
- **Estimate costs**: Calculate token usage for budget planning
- **Cache prompts**: Prompt generation is cached automatically
- **Limit depth**: Set reasonable `max_handoff_depth` (3-5)
- **Monitor usage**: Track autonomous feature LLM calls

### Token Budget Management

```rust
// Calculate estimated token usage
let base_tokens = 1000; // Task input + output
let planning_tokens = planning_enabled ? 1000 : 0;
let prompt_gen_tokens = prompt_gen_enabled && !cached ? 500 : 0;
let temp_tokens = dynamic_temp_enabled ? 300 : 0;
let handoff_tokens = handoffs_enabled ? 200 * max_depth : 0;

let estimated_total = base_tokens + planning_tokens 
                    + prompt_gen_tokens + temp_tokens 
                    + handoff_tokens;

if estimated_total > budget {
    // Disable or reduce features
}
```

### Combining Features Effectively

#### Recommended Combinations

**Research Agent** (Exploration & Analysis):
```rust
.max_loops(MaxLoops::Auto)         // Plan research steps
.dynamic_temperature(true)          // Adapt to analysis vs. writing
.enable_handoffs()                  // Delegate to specialists
```

**Code Generation Agent** (Precision & Expertise):
```rust
.auto_generate_prompt(true)         // Standard prompt template
.dynamic_temperature(true)          // Low temp for code, high for docs
.enable_handoffs()                  // Delegate to security expert
```

**Customer Support Agent** (Conversational & Adaptive):
```rust
.dynamic_temperature(true)          // Conversational tone
.enable_handoffs()                  // Escalate to specialists
```

**Data Analysis Agent** (Structured & Methodical):
```rust
.max_loops(MaxLoops::Auto)         // Break down analysis steps
.auto_generate_prompt(true)         // Role-based prompt
.dynamic_temperature(true)          // Analytical temperature
```

#### Avoid Over-Configuration

❌ **Too much**:
```rust
.max_loops(MaxLoops::Auto)         // Planning
.auto_generate_prompt(true)         // Auto prompt
.dynamic_temperature(true)          // Dynamic temp
.enable_handoffs()                  // Handoffs
.max_handoff_depth(10)              // Deep chains
// Result: Slow, expensive, complex debugging
```

✅ **Balanced**:
```rust
.max_loops(MaxLoops::Fixed(5))     // Fixed loops
.system_prompt("...")               // Manual prompt (tuned)
.dynamic_temperature(true)          // Only dynamic temp
// Result: Fast, cost-effective, predictable
```

---

## Error Handling

Autonomous features have specific error types for different failure modes.

### PlanningError

```rust
pub enum PlanningError {
    /// LLM failed to generate a valid plan
    PlanGenerationFailed(String),
    
    /// Generated plan has no subtasks
    EmptyPlan,
    
    /// Subtask dependencies are circular
    CircularDependencies(Vec<String>),
    
    /// LLM provider error during planning
    LlmError(LlmError),
}
```

**Handling**:
```rust
use paladin::core::platform::container::planning::PlanningError;

match paladin.execute(input).await {
    Err(PaladinError::Planning(PlanningError::PlanGenerationFailed(msg))) => {
        eprintln!("Failed to generate plan: {}", msg);
        // Fallback: Use fixed loop count
        paladin.config_mut().max_loops = MaxLoops::Fixed(5);
        paladin.execute(input).await?
    }
    Err(PaladinError::Planning(PlanningError::EmptyPlan)) => {
        eprintln!("LLM returned empty plan, using default execution");
        // Fallback: Single execution
        paladin.config_mut().max_loops = MaxLoops::Fixed(1);
        paladin.execute(input).await?
    }
    Ok(result) => result,
    Err(e) => return Err(e),
}
```

### PromptError

```rust
pub enum PromptError {
    /// LLM failed to generate a valid prompt
    GenerationFailed(String),
    
    /// Agent description is missing or empty
    MissingDescription,
    
    /// Generated prompt is too short/long
    InvalidLength { length: usize, min: usize, max: usize },
    
    /// LLM provider error during generation
    LlmError(LlmError),
}
```

**Handling**:
```rust
use paladin::core::platform::container::prompt::PromptError;

let builder = PaladinBuilder::new(llm_port)
    .agent_description("Analyst")
    .auto_generate_prompt(true);

match builder.build().await {
    Err(PaladinError::Prompt(PromptError::MissingDescription)) => {
        eprintln!("Agent description required for auto-prompt");
        // Fallback: Use default prompt
        PaladinBuilder::new(llm_port)
            .system_prompt("You are a helpful AI assistant.")
            .build().await?
    }
    Err(PaladinError::Prompt(PromptError::GenerationFailed(msg))) => {
        eprintln!("Prompt generation failed: {}", msg);
        // Fallback: Manual prompt
        PaladinBuilder::new(llm_port)
            .system_prompt("You are an analyst.")
            .build().await?
    }
    Ok(paladin) => paladin,
    Err(e) => return Err(e),
}
```

### HandoffError

```rust
pub enum HandoffError {
    /// Target agent not found in registry
    InvalidAgent(String),
    
    /// Circular handoff detected
    CircularHandoff { chain: Vec<String>, attempted_target: String },
    
    /// Maximum handoff depth exceeded
    MaxDepthExceeded { current_depth: u32, max_depth: u32 },
    
    /// Specialist execution failed
    ExecutionFailed { agent: String, error: String },
    
    /// LLM provider error during handoff
    LlmError(LlmError),
}
```

**Handling**:
```rust
use paladin::core::platform::container::handoff::{HandoffError, HandoffDecision};

match handoff_service.should_handoff(task, agent, context).await {
    Ok(HandoffDecision::Handoff { target_agent, .. }) => {
        match execute_handoff(&target_agent, task).await {
            Ok(result) => result,
            Err(HandoffError::InvalidAgent(name)) => {
                eprintln!("Agent '{}' not found, continuing with current agent", name);
                // Fallback: Current agent completes task
                current_agent.execute(task).await?
            }
            Err(HandoffError::CircularHandoff { chain, attempted_target }) => {
                eprintln!("Circular handoff detected: {:?} -> {}", chain, attempted_target);
                // Fallback: Break chain, current agent completes
                current_agent.execute(task).await?
            }
            Err(HandoffError::MaxDepthExceeded { current_depth, max_depth }) => {
                eprintln!("Max depth {} exceeded (current: {})", max_depth, current_depth);
                // Fallback: No more handoffs, finish with current agent
                current_agent.execute(task).await?
            }
            Err(e) => return Err(e),
        }
    }
    Ok(HandoffDecision::Complete) => {
        // No handoff needed
        current_agent.execute(task).await?
    }
    Err(e) => return Err(e.into()),
}
```

### Graceful Degradation

**Pattern**: Disable feature on error, continue execution

```rust
async fn execute_with_fallback(paladin: &Paladin, input: &str) -> Result<String> {
    // Try with autonomous features
    match paladin.execute(input).await {
        Ok(result) => Ok(result.output),
        
        // Planning failed: retry with fixed loops
        Err(PaladinError::Planning(_)) => {
            eprintln!("Planning failed, using fixed execution");
            let mut config = paladin.config().clone();
            config.max_loops = MaxLoops::Fixed(3);
            paladin.execute_with_config(input, config).await
                .map(|r| r.output)
        }
        
        // Handoff failed: continue without delegation
        Err(PaladinError::Handoff(_)) => {
            eprintln!("Handoff failed, completing task without delegation");
            let mut config = paladin.config().clone();
            config.handoffs_enabled = false;
            paladin.execute_with_config(input, config).await
                .map(|r| r.output)
        }
        
        // Other errors: propagate
        Err(e) => Err(e),
    }
}
```

---

## Troubleshooting

### Common Issues and Solutions

#### Issue: Planning generates too many subtasks

**Symptom**: Plans have 20+ subtasks, execution is slow

**Solution**:
```yaml
autonomous:
  planning:
    max_subtasks: 10  # Reduce limit
```

Or provide more focused input:
```rust
// ❌ Too broad
"Analyze the company's performance"

// ✅ More focused
"Analyze Q4 2025 revenue trends and identify top 3 growth drivers"
```

#### Issue: Generated prompts are too generic

**Symptom**: Auto-generated prompts lack specificity

**Solution**: Provide detailed agent descriptions
```rust
// ❌ Too vague
.agent_description("Analyst")

// ✅ Specific
.agent_description(
    "Senior financial analyst specializing in SaaS companies, \
     with expertise in revenue forecasting, churn analysis, and \
     unit economics. Focus on actionable insights and data-driven \
     recommendations."
)
```

#### Issue: Wrong temperature for task

**Symptom**: Factual tasks get creative outputs, or vice versa

**Solution**: Check classification logic or override manually
```rust
// Option 1: Provide clearer task description
// ❌ Ambiguous
"Tell me about quantum computing"

// ✅ Clear intent
"Calculate the energy levels of a hydrogen atom" // → Factual

// Option 2: Manual override
.temperature(0.2)  // Force low temperature
.dynamic_temperature(false)  // Disable auto-adjustment
```

#### Issue: Circular handoff errors

**Symptom**: `HandoffError::CircularHandoff` errors

**Solution**: Review agent configurations and handoff logic
```rust
// Check specialist capabilities don't overlap
handoff_service.register_specialist(
    "code-expert",
    "Rust code generation and debugging (does NOT do security audits)"
);

handoff_service.register_specialist(
    "security-expert",
    "Security audits and vulnerability analysis (does NOT write code)"
);
```

#### Issue: Max depth exceeded

**Symptom**: `HandoffError::MaxDepthExceeded` errors

**Solution**: Increase `max_depth` or simplify task delegation
```yaml
autonomous:
  handoffs:
    max_depth: 10  # Increase limit
```

Or break complex delegation into separate Paladin executions.

#### Issue: Features not activating

**Symptom**: Autonomous features appear disabled despite configuration

**Solution**: Verify configuration precedence
```rust
// Check 1: Configuration loaded?
println!("Config: {:?}", paladin.config());

// Check 2: Builder methods called?
let paladin = PaladinBuilder::new(llm_port)
    .auto_generate_prompt(true)  // Must be true
    .agent_description("...")     // Must be provided
    .build().await?;

// Check 3: CLI flags passed?
// paladin agent run --auto-prompt  (must include flag)
```

### Debugging Tips

#### Enable Logging

```bash
export RUST_LOG=paladin=debug,paladin::application::services::paladin=trace

# Run with verbose output
paladin agent run --config agent.yaml --input "Task" --verbose
```

**Output**:
```
DEBUG paladin::planning: Generating plan for task: "Analyze data"
DEBUG paladin::planning: Plan generated with 5 subtasks
TRACE paladin::planning: Subtask 1: Load dataset
TRACE paladin::planning: Subtask 2: Clean data
...
```

#### Tracing

Use OpenTelemetry for distributed tracing:

```rust
use tracing::{info, debug, span, Level};

let span = span!(Level::INFO, "autonomous_execution");
let _enter = span.enter();

info!("Starting execution with planning enabled");
debug!(max_subtasks = config.planning.max_subtasks, "Planning configuration");

// Execution...
```

#### Inspect Intermediate Results

```rust
// Enable detailed output
let result = paladin.execute(input).await?;

println!("Execution time: {}ms", result.execution_time_ms);
println!("Loops completed: {}", result.loop_count);
println!("Stop reason: {:?}", result.stop_reason);

// Access plan (if available)
if let Some(plan) = result.plan {
    println!("Generated plan:");
    for subtask in plan.subtasks {
        println!("  - {}: {}", subtask.id, subtask.description);
    }
}

// Access handoff history
for handoff in result.handoff_history {
    println!("Handoff: {} -> {} ({})", 
             handoff.from_agent, 
             handoff.to_agent, 
             handoff.reason);
}
```

### Performance Optimization Tips

#### Reduce Token Usage
```rust
// Disable expensive features for simple tasks
if task.len() < 50 {
    builder
        .max_loops(MaxLoops::Fixed(1))
        .dynamic_temperature(false);
}
```

#### Cache Aggressively
```rust
// Prompt generation caches automatically
// For other expensive operations, implement caching:
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

let task_type_cache: Arc<RwLock<HashMap<String, TaskType>>> = 
    Arc::new(RwLock::new(HashMap::new()));

// Check cache before classification
if let Some(cached_type) = task_type_cache.read().await.get(task) {
    return Ok(*cached_type);
}
```

#### Parallel Execution
```rust
// For independent tasks, use Phalanx (parallel execution)
let phalanx = Phalanx::new(vec![paladin1, paladin2, paladin3]);
let results = phalanx.execute(inputs).await?;
```

#### Optimize Handoff Strategy
```rust
// Use explicit handoffs for predictable workflows
builder.handoff_strategy(HandoffStrategy::Explicit);

// Implement custom decision logic
if task_requires_specialist(&task) {
    execute_handoff("specialist", &task).await?
} else {
    current_agent.execute(&task).await?
}
```

---

## Advanced Usage

### Combining Autonomous Features

#### Example: Research & Analysis Agent

```rust
let research_agent = PaladinBuilder::new(llm_port)
    .name("research-analyst")
    .max_loops(MaxLoops::Auto)  // Plan research steps
    .agent_description(
        "Expert research analyst with skills in literature review, \
         data synthesis, and academic writing"
    )
    .auto_generate_prompt(true)  // Generate researcher prompt
    .dynamic_temperature(true)   // Analytical + creative
    .temperature_bounds(0.3, 0.8)
    .enable_handoffs()           // Delegate to specialists
    .handoff_strategy(HandoffStrategy::threshold(0.7))
    .build()
    .await?;

// Register specialists
handoff_service.register_specialist(
    "statistics-expert",
    "Statistical analysis and data interpretation"
).await?;

handoff_service.register_specialist(
    "writer",
    "Academic and technical writing"
).await?;

// Execute complex research task
let result = research_agent
    .execute("Research the impact of AI on software development productivity")
    .await?;
```

#### Example: Code Generation Agent

```rust
let code_agent = PaladinBuilder::new(llm_port)
    .name("code-generator")
    .agent_description(
        "Expert Rust developer specializing in safe, idiomatic code \
         with comprehensive error handling and documentation"
    )
    .auto_generate_prompt(true)  // Generate coder prompt
    .dynamic_temperature(true)   // Low for code, higher for docs
    .temperature_bounds(0.1, 0.6)
    .enable_handoffs()           // Delegate testing & review
    .build()
    .await?;

// Register specialists
handoff_service.register_specialist(
    "test-engineer",
    "Unit and integration test generation"
).await?;

handoff_service.register_specialist(
    "security-auditor",
    "Security review and vulnerability scanning"
).await?;

// Generate with automatic testing and review
let result = code_agent
    .execute("Create a secure REST API endpoint for user authentication")
    .await?;
```

### Custom Agent Configurations

#### Multi-Stage Pipeline

```rust
// Stage 1: Planning
let planner = PaladinBuilder::new(llm_port)
    .name("planner")
    .max_loops(MaxLoops::Auto)
    .auto_generate_prompt(true)
    .agent_description("Task decomposition specialist")
    .build()
    .await?;

// Stage 2: Execution
let executor = PaladinBuilder::new(llm_port)
    .name("executor")
    .max_loops(MaxLoops::Fixed(1))
    .dynamic_temperature(true)
    .enable_handoffs()
    .build()
    .await?;

// Stage 3: Review
let reviewer = PaladinBuilder::new(llm_port)
    .name("reviewer")
    .max_loops(MaxLoops::Fixed(1))
    .temperature(0.3)  // Analytical
    .system_prompt("Review the output for completeness and accuracy")
    .build()
    .await?;

// Execute pipeline
let plan_result = planner.execute(task).await?;
let exec_result = executor.execute(&plan_result.output).await?;
let final_result = reviewer.execute(&exec_result.output).await?;
```

#### Adaptive Agent

```rust
// Agent that adjusts its configuration based on feedback
struct AdaptiveAgent {
    paladin: Paladin,
    performance_history: Vec<f32>,
}

impl AdaptiveAgent {
    async fn execute_adaptive(&mut self, task: &str) -> Result<String> {
        // Adjust based on historical performance
        let avg_performance = self.performance_history.iter().sum::<f32>() 
                            / self.performance_history.len() as f32;
        
        if avg_performance < 0.7 {
            // Performance is low, enable more features
            self.paladin.config_mut().max_loops = MaxLoops::Auto;
            self.paladin.config_mut().enable_handoffs = true;
        } else {
            // Performance is good, optimize for speed
            self.paladin.config_mut().max_loops = MaxLoops::Fixed(3);
            self.paladin.config_mut().enable_handoffs = false;
        }
        
        let result = self.paladin.execute(task).await?;
        
        // Record performance
        let performance = self.calculate_performance(&result);
        self.performance_history.push(performance);
        
        Ok(result.output)
    }
}
```

### Integration with Battalion Patterns

#### Formation with Autonomous Agents

```rust
use paladin::application::services::battalion::formation_service::FormationService;

// Create autonomous agents
let agent1 = PaladinBuilder::new(llm_port.clone())
    .name("researcher")
    .max_loops(MaxLoops::Auto)
    .auto_generate_prompt(true)
    .agent_description("Research and data gathering specialist")
    .build().await?;

let agent2 = PaladinBuilder::new(llm_port.clone())
    .name("analyst")
    .dynamic_temperature(true)
    .auto_generate_prompt(true)
    .agent_description("Data analysis and insights expert")
    .build().await?;

let agent3 = PaladinBuilder::new(llm_port.clone())
    .name("writer")
    .temperature(0.7)
    .auto_generate_prompt(true)
    .agent_description("Report writing and documentation specialist")
    .build().await?;

// Formation: Sequential execution (output N → input N+1)
let formation = FormationService::new();
let result = formation.execute(
    vec![agent1, agent2, agent3],
    "Analyze market trends in AI industry"
).await?;
```

#### Phalanx with Handoffs

```rust
use paladin::application::services::battalion::phalanx_service::PhalanxService;

// Create agents with handoff capabilities
let agents: Vec<Paladin> = vec![
    PaladinBuilder::new(llm_port.clone())
        .name("competitor-analyzer")
        .enable_handoffs()
        .build().await?,
    PaladinBuilder::new(llm_port.clone())
        .name("market-researcher")
        .enable_handoffs()
        .build().await?,
    PaladinBuilder::new(llm_port.clone())
        .name("trend-analyst")
        .enable_handoffs()
        .build().await?,
];

// Register shared specialists
for agent in &agents {
    handoff_service.register_specialist(
        "data-expert",
        "Statistical analysis and data interpretation"
    ).await?;
}

// Phalanx: Parallel execution
let phalanx = PhalanxService::new();
let results = phalanx.execute(
    agents,
    vec!["Analyze competitor X", "Research market Y", "Identify trend Z"]
).await?;
```

---

## API Reference

### PaladinBuilder Methods

#### Autonomous Planning
```rust
/// Enable autonomous planning mode
pub fn max_loops(mut self, loops: MaxLoops) -> Self

// MaxLoops variants
pub enum MaxLoops {
    Fixed(u32),  // Manual loop count
    Auto,        // Autonomous planning
}
```

#### Prompt Generation
```rust
/// Enable automatic prompt generation
pub fn auto_generate_prompt(mut self, enabled: bool) -> Self

/// Set agent description (required for auto-prompt)
pub fn agent_description(mut self, description: impl Into<String>) -> Self

/// Manual system prompt (overrides auto-generation)
pub fn system_prompt(mut self, prompt: impl Into<String>) -> Self
```

#### Dynamic Temperature
```rust
/// Enable dynamic temperature adjustment
pub fn dynamic_temperature(mut self, enabled: bool) -> Self

/// Set temperature bounds (min, max)
pub fn temperature_bounds(mut self, min: f32, max: f32) -> Self

/// Manual temperature (overrides dynamic)
pub fn temperature(mut self, temp: f32) -> Self
```

#### Agent Handoffs
```rust
/// Enable agent handoff capabilities
pub fn enable_handoffs(mut self) -> Self

/// Set handoff strategy
pub fn handoff_strategy(mut self, strategy: HandoffStrategy) -> Self

/// Set maximum handoff depth
pub fn max_handoff_depth(mut self, depth: u32) -> Self
```

### Configuration Types

#### AutonomousConfig
```rust
pub struct AutonomousConfig {
    pub planning: PlanningConfig,
    pub prompt_generation: PromptConfig,
    pub dynamic_temperature: TemperatureConfig,
    pub handoffs: HandoffConfig,
}

impl AutonomousConfig {
    pub fn new() -> Self;
    pub fn validate(&self) -> Result<(), String>;
}
```

#### PlanningConfig
```rust
pub struct PlanningConfig {
    pub enabled: bool,
    pub max_subtasks: u32,
}

impl PlanningConfig {
    pub fn new(max_subtasks: u32) -> Self;
    pub fn enabled() -> Self;
}
```

#### PromptConfig
```rust
pub struct PromptConfig {
    pub enabled: bool,
    pub description: Option<String>,
}

impl PromptConfig {
    pub fn new(description: String) -> Self;
    pub fn enabled() -> Self;
    pub fn with_description(self, description: String) -> Self;
}
```

#### TemperatureConfig
```rust
pub struct TemperatureConfig {
    pub enabled: bool,
    pub min: f32,
    pub max: f32,
}

impl TemperatureConfig {
    pub fn new(min: f32, max: f32) -> Self;
    pub fn enabled() -> Self;
    pub fn with_bounds(self, min: f32, max: f32) -> Self;
}
```

#### HandoffConfig
```rust
pub struct HandoffConfig {
    pub enabled: bool,
    pub strategy: HandoffStrategy,
    pub max_depth: u32,
}

impl HandoffConfig {
    pub fn new(strategy: HandoffStrategy, max_depth: u32) -> Self;
    pub fn enabled() -> Self;
    pub fn with_strategy(self, strategy: HandoffStrategy) -> Self;
    pub fn with_max_depth(self, max_depth: u32) -> Self;
}
```

### Services

#### PlanningService
```rust
pub struct PlanningService {
    llm_port: Arc<dyn LlmPort>,
}

impl PlanningService {
    pub fn new(llm_port: Arc<dyn LlmPort>) -> Self;
    
    pub async fn generate_plan(
        &self,
        task: &str,
        max_subtasks: u32
    ) -> Result<TaskPlan, PlanningError>;
}
```

#### PromptGenerationService
```rust
pub struct PromptGenerationService {
    llm_port: Arc<dyn LlmPort>,
    cache: Arc<RwLock<HashMap<String, String>>>,
}

impl PromptGenerationService {
    pub fn new(llm_port: Arc<dyn LlmPort>) -> Self;
    
    pub async fn generate_prompt(
        &self,
        agent_name: &str,
        description: &str
    ) -> Result<String, PromptError>;
    
    pub async fn clear_cache(&self);
    
    pub async fn invalidate_cache(&self, agent_name: &str, description: &str);
}
```

#### TemperatureService
```rust
pub struct TemperatureService {
    llm_port: Arc<dyn LlmPort>,
}

impl TemperatureService {
    pub fn new(llm_port: Arc<dyn LlmPort>) -> Self;
    
    pub async fn calculate_optimal_temperature(
        &self,
        task: &str,
        config: Option<&TemperatureConfig>
    ) -> Result<f32, TemperatureError>;
    
    pub async fn detect_task_type_with_llm(
        &self,
        task: &str
    ) -> Result<TaskType, TemperatureError>;
}
```

#### HandoffService
```rust
pub struct HandoffService {
    llm_port: Arc<dyn LlmPort>,
    specialists: Arc<RwLock<HashMap<String, String>>>,
}

impl HandoffService {
    pub fn new(llm_port: Arc<dyn LlmPort>) -> Self;
    
    pub async fn register_specialist(
        &self,
        name: &str,
        description: &str
    ) -> Result<(), HandoffError>;
    
    pub async fn should_handoff(
        &self,
        task: &str,
        current_agent: &str,
        context: &HandoffContext
    ) -> Result<HandoffDecision, HandoffError>;
    
    pub fn get_specialists(&self) -> Vec<String>;
}
```

### Error Types

```rust
pub enum PlanningError {
    PlanGenerationFailed(String),
    EmptyPlan,
    CircularDependencies(Vec<String>),
    LlmError(LlmError),
}

pub enum PromptError {
    GenerationFailed(String),
    MissingDescription,
    InvalidLength { length: usize, min: usize, max: usize },
    LlmError(LlmError),
}

pub enum TemperatureError {
    ClassificationFailed(String),
    InvalidBounds { min: f32, max: f32 },
    LlmError(LlmError),
}

pub enum HandoffError {
    InvalidAgent(String),
    CircularHandoff { chain: Vec<String>, attempted_target: String },
    MaxDepthExceeded { current_depth: u32, max_depth: u32 },
    ExecutionFailed { agent: String, error: String },
    LlmError(LlmError),
}
```

---

## Examples

See the `examples/` directory for complete working examples:

- [`autonomous_planning.rs`](../examples/autonomous_planning.rs) - Autonomous planning mode
- [`autonomous_prompt_generation.rs`](../examples/autonomous_prompt_generation.rs) - Auto-generated prompts
- [`autonomous_temperature.rs`](../examples/autonomous_temperature.rs) - Dynamic temperature
- [`autonomous_handoffs.rs`](../examples/autonomous_handoffs.rs) - Agent delegation
- [`autonomous_complete.rs`](../examples/autonomous_complete.rs) - All features combined

Run examples:
```bash
# Autonomous planning
cargo run --example autonomous_planning

# Auto-generate prompts
cargo run --example autonomous_prompt_generation

# Dynamic temperature
cargo run --example autonomous_temperature

# Agent handoffs
cargo run --example autonomous_handoffs

# All features
cargo run --example autonomous_complete
```

---

## Further Reading

- [Paladin Overview](./README.md)
- [Battalion Orchestration](./BATTALION.md)
- [Arsenal Tool System](./ARSENAL.md)
- [Garrison Memory System](./GARRISON.md)
- [Configuration Guide](./INSTALLATION.md)
- [API Documentation](https://docs.rs/paladin)

---

**Version**: 0.1.0  
**Last Updated**: February 1, 2026  
**Status**: ✅ Stable (Epic 14 Complete)
