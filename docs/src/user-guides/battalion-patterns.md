# Battalion Orchestration System

**Multi-Paladin coordination framework with eight orchestration patterns**

---

## Table of Contents

1. [Overview](#overview)
2. [Quick Start](#quick-start)
3. [Orchestration Patterns](#orchestration-patterns)
4. [Commander Strategy Router](#commander-strategy-router)
5. [Configuration](#configuration)
6. [Error Handling](#error-handling)
7. [Performance](#performance)
8. [Best Practices](#best-practices)
9. [API Reference](#api-reference)

---

## Overview

The Battalion system enables coordination of multiple Paladin agents through eight distinct orchestration patterns:

| Pattern | Description | Use Case | Complexity |
|---------|-------------|----------|------------|
| **Formation** | Sequential execution (output N → input N+1) | Multi-step pipelines, data transformations | Low |
| **Phalanx** | Concurrent execution with result aggregation | Parallel analysis, consensus building | Medium |
| **Campaign** | Graph/DAG-based conditional routing | Complex workflows, branching logic | High |
| **Chain of Command** | Hierarchical delegation (commander + specialists) | Task routing, load distribution | Medium-High |
| **Conclave** | Multi-expert synthesis (Mixture-of-Agents) | Expert panel decisions, comprehensive analysis | Medium |
| **Council** | Multi-agent deliberation with turn-taking | Collaborative discussion, consensus building | Medium |
| **Grove** | Tree-based intelligent agent routing | Specialist selection, task distribution | Medium |
| **Maneuver** | **Flow DSL declarative orchestration** | **Dynamic workflows, mixed patterns** | **Medium** |

###  Key Features

- **Hexagonal Architecture**: Clean separation of domain, application, and infrastructure layers
- **Error Resilience**: Three strategies (FailFast, ContinueOnError, RetryThenContinue)
- **High Performance**: <1s orchestration overhead, tested with 100+ concurrent Battalions
- **Type Safety**: Full Rust type system guarantees, compile-time validation
- **Async/Await**: Built on tokio for efficient concurrent execution

---

## Quick Start

### Installation

Add to `Cargo.toml`:

```toml
[dependencies]
paladin = "0.1.0"
tokio = { version = "1.0", features = ["full"] }
```

### Basic Formation Example

```rust
use paladin::application::services::battalion::formation_service::FormationExecutionService;
use paladin::core::platform::container::battalion::formation::Formation;
use paladin::core::platform::container::battalion::BattalionConfig;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create Paladins
    let paladins = vec![
        create_paladin("analyzer", "Analyze the input data"),
        create_paladin("processor", "Process the analyzed data"),
        create_paladin("summarizer", "Create a summary"),
    ];

    // Create Formation
    let config = BattalionConfig::default();
    let formation = Formation::new(paladins, config)?;

    // Execute
    let service = FormationExecutionService::new(Arc::new(llm_port));
    let result = service.execute(&formation, "Initial input").await?;

    println!("Result: {:?}", result);
    Ok(())
}
```

---

## Orchestration Patterns

### 1. Formation (Sequential Pipeline)

**Purpose**: Execute Paladins sequentially, passing output from each to the next.

**Architecture**:
```
Input → Paladin₁ → Paladin₂ → Paladin₃ → Output
```

**When to Use**:
- Data transformation pipelines
- Multi-step analysis workflows
- Iterative refinement tasks

**Example**:
```rust
let paladins = vec![
    create_paladin("extractor", "Extract key information"),
    create_paladin("validator", "Validate the extracted data"),
    create_paladin("formatter", "Format as JSON"),
];

let formation = Formation::new(paladins, config)?;
let result = formation_service.execute(&formation, text_input).await?;
```

**Performance**: Linear time complexity O(n), where n = number of Paladins.

---

### 2. Phalanx (Concurrent Execution)

**Purpose**: Execute all Paladins concurrently and aggregate results.

**Architecture**:
```
Input → ┌─ Paladin₁ ─┐
        ├─ Paladin₂ ─┤ → Aggregation → Output
        └─ Paladin₃ ─┘
```

**Aggregation Strategies**:

| Strategy | Description | When to Use |
|----------|-------------|-------------|
| `CollectAll` | Gather all results | Multi-perspective analysis |
| `FirstSuccess` | Return first successful result | Fastest response needed |
| `Majority` | Consensus voting (≥3 Paladins) | Decision-making, validation |
| `Custom` | User-defined aggregation function | Domain-specific logic |

**Example**:
```rust
use paladin::core::platform::container::battalion::phalanx::{Phalanx, AggregationStrategy};

let paladins = vec![
    create_paladin("gpt4", "Expert analyst"),
    create_paladin("claude", "Critical reviewer"),
    create_paladin("gemini", "Creative thinker"),
];

let phalanx = Phalanx::new(paladins, config)?
    .with_aggregation(AggregationStrategy::Majority);

let result = phalanx_service.execute(&phalanx, question).await?;
```

**Per-Paladin Metrics**:

Phalanx provides detailed execution metrics for each Paladin, enabling fine-grained performance analysis:

```rust
let result = phalanx_service.execute(&phalanx, question).await?;

// Access execution times per Paladin by name
println!("Execution Times:");
for (paladin_name, time_ms) in &result.per_paladin_times {
    println!("  {}: {}ms", paladin_name, time_ms);
}

// Access token usage per Paladin
println!("\nToken Usage:");
for (paladin_name, tokens) in &result.per_paladin_tokens {
    println!("  {}: {} tokens (prompt: {}, completion: {})",
        paladin_name,
        tokens.total_tokens,
        tokens.prompt_tokens,
        tokens.completion_tokens
    );
}

// Calculate metrics
let avg_time: u64 = result.per_paladin_times.values().sum::<u64>()
    / result.per_paladin_times.len() as u64;
let max_time = result.per_paladin_times.values().max().unwrap_or(&0);
let total_tokens: usize = result.per_paladin_tokens.values()
    .map(|t| t.total_tokens)
    .sum();

println!("\nAggregate Metrics:");
println!("  Average time: {}ms", avg_time);
println!("  Slowest Paladin: {}ms", max_time);
println!("  Total tokens: {}", total_tokens);
```

**Metrics Use Cases**:

- **Performance Profiling**: Identify slow Paladins for optimization
- **Cost Analysis**: Track token consumption per model/Paladin
- **Load Balancing**: Adjust Paladin assignments based on execution patterns
- **SLA Monitoring**: Verify all Paladins meet latency requirements

**Performance**: Constant time O(1) with respect to Paladin count (concurrent execution).

---

### 3. Campaign (Graph Orchestration)

**Purpose**: Execute Paladins based on a directed acyclic graph (DAG) with conditional routing.

**Architecture**:
```
        ┌─ Paladin₂ ─┐
Input → Paladin₁      ├→ Paladin₄ → Output
        └─ Paladin₃ ─┘
```

**Edge Conditions**:

- `Always`: Unconditional edge
- `Contains(String)`: Route if output contains text
- `Regex(String)`: Route if regex matches
- `Custom(String)`: User-defined condition logic

**Example**:
```rust
use paladin::core::platform::container::battalion::campaign::{Campaign, EdgeCondition};

let mut campaign = Campaign::new(config)?;

// Add Paladins
campaign.add_paladin("classifier", create_paladin("classifier", "Classify input"));
campaign.add_paladin("technical", create_paladin("technical", "Handle technical"));
campaign.add_paladin("general", create_paladin("general", "Handle general"));

// Add conditional edges
campaign.add_edge(
    "classifier",
    "technical",
    EdgeCondition::Contains("technical".into()),
    None // No transformation
)?;

campaign.add_edge(
    "classifier",
    "general",
    EdgeCondition::Always,
    None
)?;

campaign.set_entry_points(vec!["classifier".into()])?;

let result = campaign_service.execute(&campaign, user_input).await?;
```

**Performance**: Depends on graph structure; worst-case O(V + E) where V = vertices, E = edges.

---

### 4. Chain of Command (Hierarchical Delegation)

**Purpose**: Commander Paladin analyzes input and delegates to appropriate specialist Paladin(s).

**Architecture**:
```
                Commander (analyzes + routes)
                     ↓
        ┌────────────┼────────────┐
        ↓            ↓            ↓
   Specialist₁   Specialist₂  Specialist₃
```

**Delegation Strategies**:

| Strategy | Description | Use Case |
|----------|-------------|----------|
| `Automatic` | Commander uses LLM to select specialists | Dynamic routing based on content |
| `Broadcast` | Send to all specialists concurrently | Consensus, validation |
| `RoundRobin` | Rotate through specialists | Load balancing |
| `Custom` | User-defined delegation logic | Business-specific rules |

**Example - Automatic Delegation**:
```rust
use paladin::core::platform::container::battalion::chain_of_command::{
    ChainOfCommand, DelegationStrategy
};

let commander = create_paladin("commander",
    "You are a task router. Analyze the input and select specialists.");

let specialists = vec![
    create_paladin("database", "Database specialist"),
    create_paladin("api", "API integration specialist"),
    create_paladin("analytics", "Data analytics specialist"),
];

let chain = ChainOfCommand::new(commander, specialists, config)?
    .with_strategy(DelegationStrategy::Automatic);

// Commander will analyze "Query user database" and select database specialist
let result = chain_service.execute(&chain, "Query user database").await?;
```

**Performance**: O(1) for delegation decision + O(k) for executing k selected specialists.

---

### 5. Conclave (Multi-Expert Synthesis)

**Purpose**: Multiple specialized Paladins (experts) analyze input in parallel, then an aggregator synthesizes their diverse perspectives into a comprehensive response. Implements the **Mixture-of-Agents** pattern.

**Architecture**:
```
                    ┌──────────────┐
                    │   Input      │
                    └──────┬───────┘
                           │
         ┌─────────────────┼─────────────────┐
         │                 │                 │
         ▼                 ▼                 ▼
  ┌─────────────┐   ┌─────────────┐   ┌─────────────┐
  │  Expert 1   │   │  Expert 2   │   │  Expert 3   │
  │ (Technical) │   │ (Business)  │   │ (Security)  │
  └──────┬──────┘   └──────┬──────┘   └──────┬──────┘
         │                 │                 │
         └─────────────────┼─────────────────┘
                           │
                           ▼
                    ┌─────────────┐
                    │ Aggregator  │
                    │  Synthesis  │
                    └──────┬──────┘
                           │
                           ▼
                    ┌─────────────┐
                    │   Final     │
                    │  Response   │
                    └─────────────┘
```

**When to Use**:
- Decisions benefit from multiple expert perspectives (technical, business, security, etc.)
- Diverse viewpoints must be intelligently synthesized
- Quality improves through multi-perspective analysis
- Different stakeholder concerns must all be addressed

**Key Features**:
- **Parallel Expert Execution**: All experts analyze concurrently
- **Intelligent Synthesis**: Aggregator combines perspectives (not simple concatenation)
- **Resilience**: Continues even if some experts fail (partial success)
- **Retry Logic**: Exponential backoff with jitter for failed experts
- **Token Management**: Optional truncation to prevent context overflow
- **Observability**: Three levels (Minimal, Standard, Verbose)

**Example**:
```rust
use paladin::core::platform::container::battalion::conclave::{Conclave, ConclaveConfig};

// Create 3 experts with different perspectives
let technical = create_paladin("TechnicalExpert",
    "Analyze from a technical architecture perspective");
let business = create_paladin("BusinessExpert",
    "Analyze from a business strategy perspective");
let security = create_paladin("SecurityExpert",
    "Analyze from a security and compliance perspective");

// Create aggregator to synthesize expert outputs
let aggregator = create_paladin("Aggregator",
    "Synthesize the expert analyses into a comprehensive recommendation");

// Configure Conclave
let config = ConclaveConfig::new("expert-panel", BattalionConfig::default())
    .with_timeout(300)
    .with_retry_attempts(2)
    .with_observability(ObservabilityLevel::Standard);

// Build and execute
let conclave = Conclave::new(
    vec![technical, business, security],
    aggregator,
    config
)?;

let result = conclave_service.execute(&conclave,
    "Should we migrate to microservices?"
).await?;

println!("Final Recommendation:\n{}", result.aggregated_output.output);
```

**Performance**: O(1) with respect to expert count (concurrent execution) + O(1) for aggregation.

**Learn More**: See [Conclave Pattern Guide](./guides/conclave-pattern.md) for comprehensive documentation including configuration options, YAML setup, CLI usage, best practices, and troubleshooting.

---

### 6. Council (Deliberative Discussion)

**Purpose**: Enable multi-agent deliberation with structured turn-taking and conversation flow.

**Architecture**:
```
Topic: "Should we implement feature X?"

Round 1:  [Expert1] → [Expert2] → [Expert3]
Round 2:  [Expert1] → [Expert2] → [Expert3]
Round 3:  [Expert1] → [Expert2] → [Expert3]

→ Final Output: Synthesized recommendations
```

**Turn-Taking Strategies**:
- **RoundRobin**: Participants speak in order, cycling through the list
- **ModeratorDirected**: Moderator controls discussion flow, calls on relevant experts

**Termination Conditions**:
- **MaxRounds**: Fixed number of discussion rounds
- **Consensus**: Stops when agreement detected (keyword-based)
- **ModeratorDecision**: Moderator decides when sufficient deliberation
- **Keyword**: Specific keyword triggers termination (e.g., "APPROVED")

**When to Use**:
- Collaborative decision-making requiring discussion
- Consensus building among stakeholders
- Expert panel deliberations
- Structured debate with turn-taking

**Example**:
```rust
use paladin::core::platform::container::battalion::council::{
    CouncilBuilder, TurnStrategy, TerminationCondition
};

let council = CouncilBuilder::new()
    .name("Security Review Council")
    .add_participant(security_expert)
    .add_participant(legal_expert)
    .add_participant(technical_expert)
    .turn_strategy(TurnStrategy::RoundRobin)
    .termination_condition(TerminationCondition::MaxRounds(3))
    .build()?;

let topic = "Should we implement two-factor authentication?";
let result = council_service.convene(&council, topic).await?;
```

**Performance**: O(P × R) where P = participants, R = rounds.

**Learn More**: See [Council Pattern Documentation](COUNCIL.md) for comprehensive guide including moderated discussions, consensus building, and conversation history storage.

---

### 7. Grove (Intelligent Agent Routing)

**Purpose**: Route tasks to specialized agents based on expertise matching.

**Architecture**:
```
Task: "Optimize database queries"
         │
         ▼
   [Routing Engine]
         │
    ┌────┴────┐
    ▼         ▼
[Backend]  [Frontend]
[Tree]     [Tree]
│          │
├─ DB Expert ✓ (87% match)
├─ API Expert
└─ Service Expert
```

**Routing Strategies**:

| Strategy | Speed | Cost | Accuracy | Requirements |
|----------|-------|------|----------|--------------|
| **KeywordMatch** | <10ms | Free | Good | Keywords only |
| **SemanticSimilarity** | ~100ms | Low | Better | Embedding service |
| **LlmRouting** | ~300ms | Medium | Best | LLM service |

**When to Use**:
- Specialized task distribution
- Domain expert selection
- Load balancing across specialists
- Hierarchical agent organization

**Example**:
```rust
use paladin::core::platform::container::battalion::grove::{
    GroveBuilder, Tree, TreeAgent, RoutingStrategy
};

let backend_tree = Tree::new("Backend Specialists")
    .add_agent(TreeAgent::new("DatabaseExpert")
        .with_keywords(vec!["database", "sql", "query", "schema"]))
    .add_agent(TreeAgent::new("ApiExpert")
        .with_keywords(vec!["api", "rest", "graphql", "endpoint"]));

let grove = GroveBuilder::new()
    .name("Tech Support Grove")
    .add_tree(backend_tree)
    .config(GroveConfig {
        routing_strategy: RoutingStrategy::KeywordMatch,
        similarity_threshold: 0.6,
        ..Default::default()
    })
    .build()?;

let result = grove_service.execute(&grove,
    "Optimize database query performance").await?;
```

**Performance**: Routing time varies by strategy (10ms-300ms) + agent execution time.

**Learn More**: See [Grove Pattern Documentation](GROVE.md) for complete guide including semantic routing, LLM-powered routing, and expertise definition strategies.

---

### 8. Maneuver (Flow DSL Orchestration)

**Purpose**: Define complex agent workflows declaratively using a simple text-based DSL.

**Architecture**:
```
Flow DSL: "analyzer -> (summarizer, translator) -> reviewer"

Execution:
Input → analyzer → ┌─ summarizer ─┐
                   └─ translator ─┘ → reviewer → Output
```

**Flow Operators**:
- **Sequential (`->`)**: Execute agents in order, passing output as next input
- **Parallel (`,`)**: Execute agents concurrently with same input
- **Nested (`()`)**: Group agents for precedence and mixed patterns

**When to Use**:
- Complex workflows requiring both sequential and parallel execution
- Dynamic workflow generation from configuration
- Rapid prototyping of multi-agent patterns
- Visual workflow documentation needs

**Key Features**:
- **Declarative Syntax**: Define entire workflow as text expression
- **Mixed Patterns**: Combine sequential and parallel in single flow
- **Visual Feedback**: ASCII tree and Mermaid flowchart generation
- **Compile-Time Validation**: Flow expression parsing with error reporting
- **Commander Integration**: Auto-detected via "flow" keywords or `->`/`,` operators

**Example**:
```rust
use paladin::application::services::battalion::maneuver_service::ManeuverExecutionService;
use paladin::core::platform::container::battalion::maneuver::{Maneuver, ManeuverConfig};
use paladin::core::platform::container::battalion::parser::FlowParser;

// Parse flow expression
let flow = FlowParser::parse("intake -> (technical, business, security) -> synthesis")?;

// Create Paladins matching flow agent names
let mut agents = HashMap::new();
agents.insert("intake", create_paladin("intake", "Initial processing"));
agents.insert("technical", create_paladin("technical", "Technical analysis"));
agents.insert("business", create_paladin("business", "Business perspective"));
agents.insert("security", create_paladin("security", "Security review"));
agents.insert("synthesis", create_paladin("synthesis", "Combine perspectives"));

// Create Maneuver
let maneuver = Maneuver::new(
    "review-workflow",
    agents,
    flow,
    ManeuverConfig::default()
)?;

// Execute
let result = maneuver_service.execute(&maneuver, "Proposal document").await?;
```

**CLI Visualization**:
```bash
# Visualize flow structure
paladin maneuver visualize -c workflow.yaml --format ascii

# Output:
# └─> intake
#     ├─> [PARALLEL]
#     │   ├─> technical
#     │   ├─> business
#     │   └─> security
#     └─> synthesis

# Generate Mermaid flowchart
paladin maneuver visualize -c workflow.yaml --format mermaid
```

**Performance**: Parsing overhead <1ms, execution time depends on flow structure (sequential = O(n), parallel = O(1) per stage).

**Learn More**: See [Maneuver Pattern Documentation](MANEUVER.md) for complete guide including Flow DSL syntax reference, configuration options, error handling, visualization formats, and troubleshooting.

---

## Commander Strategy Router

**Unified interface for intelligent Battalion orchestration**

### Overview

The Commander is a high-level abstraction that simplifies Battalion usage by:

1. **Auto Mode**: Automatically selecting the optimal strategy based on input analysis
2. **Unified API**: Single interface for all five Battalion patterns
3. **Simplified Configuration**: Smart defaults with optional customization
4. **Enhanced Telemetry**: Strategy selection reasoning and detailed timing metadata

### Quick Start with Commander

```rust
use paladin::application::services::battalion::commander::CommanderBuilder;
use paladin::core::platform::container::battalion::BattalionStrategy;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Auto mode - Commander selects best strategy
    let commander = CommanderBuilder::new(paladin_port)
        .strategy(BattalionStrategy::Auto)
        .paladins(vec![paladin1, paladin2, paladin3])
        .build()?; // Uses smart defaults

    let result = commander.execute("Analyze this data in parallel").await?;

    // See what strategy was selected
    println!("Strategy: {:?}", result.strategy_used);
    if let Some(reasoning) = &result.strategy_selection_reasoning {
        println!("Because: {}", reasoning);
    }

    Ok(())
}
```

### Auto Mode Strategy Selection

When using `BattalionStrategy::Auto`, the Commander analyzes:

#### 1. **Input Keywords**

- **Maneuver**: "flow", "dynamic flow", "->", "," (DSL operators in input) **[Highest Priority]**
- **Formation**: "sequential", "pipeline", "step by step", "one after", "first then"
- **Phalanx**: "parallel", "concurrent", "all at once", "simultaneously"
- **Campaign**: "workflow", "graph", "conditional", "if-then", "depends on"
- **ChainOfCommand**: "delegate", "hierarchy", "specialist", "expert"

#### 2. **Paladin Count Heuristics**

- **1-3 Paladins**: Defaults to Formation (sequential)
- **4+ Paladins**: Analyzes for parallelism or specialization
- **Many similar Paladins**: Prefers Phalanx (parallel)
- **Mixed specialist Paladins**: Considers ChainOfCommand

#### 3. **Fallback Logic**

- If no clear indicators: Formation (safe default)
- Strategy selection takes 0-5ms typically
- Selection reasoning included in result metadata

### Examples by Strategy

#### Explicit Formation

```rust
let commander = CommanderBuilder::new(paladin_port)
    .strategy(BattalionStrategy::Formation)
    .paladins(vec![analyzer, enhancer, reviewer])
    .config(BattalionConfig::new("review_pipeline").with_timeout(60))
    .build()?;

let result = commander.execute("Review this document").await?;
```

#### Auto Mode with Telemetry

```rust
let commander = CommanderBuilder::new(paladin_port)
    .strategy(BattalionStrategy::Auto)
    .paladins(workers)
    .build()?;

let result = commander.execute("Process these items in parallel").await?;

println!("Selected: {:?} in {}ms",
    result.strategy_used,
    result.strategy_selection_time_ms);
println!("Executed in {}ms",
    result.completed_at.signed_duration_since(result.started_at)
        .num_milliseconds());
```

#### Production Configuration

```rust
use paladin::core::platform::container::battalion::{ErrorStrategy, RetryPolicy};
use std::path::PathBuf;

let config = BattalionConfig::new("production_battalion")
    .with_description("Critical data processing pipeline")
    .with_timeout(300) // 5 minutes
    .with_error_strategy(ErrorStrategy::RetryThenContinue)
    .with_retry_policy(RetryPolicy {
        max_attempts: 3,
        ..Default::default()
    })
    .with_metadata_dir(PathBuf::from("./checkpoints"));

let commander = CommanderBuilder::new(paladin_port)
    .strategy(BattalionStrategy::Formation)
    .paladins(critical_paladins)
    .config(config)
    .build()?;

match commander.execute("Critical task").await {
    Ok(result) => println!("Success: {} succeeded, {} failed",
        result.paladin_success_count,
        result.paladin_failure_count),
    Err(e) => eprintln!("Failed: {}", e),
}
```

### Configuration Options

#### Required Fields

- **strategy**: BattalionStrategy (Formation, Phalanx, Campaign, ChainOfCommand, Auto)
- **paladins**: Vec<Paladin> (must contain at least 1 Paladin)

#### Optional Fields (with defaults)

- **config**: BattalionConfig (default: 300s timeout, FailFast, 3 retries)
  - `name`: Battalion identifier (default: "default_commander_battalion")
  - `timeout_seconds`: Max execution time (default: 300)
  - `error_strategy`: How to handle failures (default: FailFast)
  - `retry_policy`: Retry configuration (default: 3 attempts with backoff)
  - `metadata_output_dir`: Checkpoint directory (default: None)

### Error Handling Strategies

#### FailFast (Default)

Stops execution immediately on first Paladin failure.

**Use When:**
- All Paladins must succeed for valid result
- Failures indicate fundamental issues
- Want fast failure feedback

```rust
.with_error_strategy(ErrorStrategy::FailFast)
```

#### ContinueOnError

Continues executing remaining Paladins despite failures, collects all errors.

**Use When:**
- Partial results are valuable
- Independent tasks where some failures acceptable
- Need complete execution report

```rust
.with_error_strategy(ErrorStrategy::ContinueOnError)
```

#### RetryThenContinue (Recommended for Production)

Retries failed Paladins up to `max_attempts`, then continues with remaining Paladins.

**Use When:**
- Transient failures are possible (network, rate limits)
- Want resilience without blocking entire workflow
- Production environments

```rust
.with_error_strategy(ErrorStrategy::RetryThenContinue)
.with_retry_policy(RetryPolicy {
    max_attempts: 3,
    ..Default::default()
})
```

### Telemetry & Metadata

Commander results include comprehensive metadata:

```rust
pub struct BattalionResult {
    pub battalion_id: Uuid,
    pub battalion_name: String,
    pub started_at: DateTime<Utc>,
    pub completed_at: DateTime<Utc>,
    pub status: BattalionStatus,
    pub strategy_used: BattalionStrategy,         // Actual strategy executed
    pub strategy_selection_reasoning: Option<String>, // Auto mode explanation
    pub strategy_selection_time_ms: u64,          // Selection overhead
    pub final_output: String,
    pub paladin_success_count: usize,
    pub paladin_failure_count: usize,
    pub per_paladin_times: Vec<u64>,              // Individual timing
    // ... additional fields
}
```

**Key Metrics:**

- `strategy_selection_time_ms`: Overhead for Auto mode (typically 0-5ms)
- `paladin_success_count` / `paladin_failure_count`: Execution statistics
- `per_paladin_times`: Individual Paladin execution times for each Paladin by name
- `per_paladin_tokens`: Token usage breakdown (prompt_tokens, completion_tokens, total_tokens) per Paladin
- `strategy_selection_reasoning`: Transparency for Auto mode decisions

#### Metadata Export (JSON Files)

Commander can automatically export comprehensive execution metadata to JSON files for:

- **Performance Analysis**: Track execution times, token usage, and bottlenecks
- **Audit Trails**: Complete execution history for compliance and debugging
- **Cost Tracking**: Per-Paladin token consumption for billing and optimization
- **Troubleshooting**: Detailed error context and failure analysis

**Enable Metadata Export:**

```rust
use std::path::PathBuf;

let config = BattalionConfig::new("audited_battalion")
    .with_metadata_dir(PathBuf::from("./battalion_metadata"));

let commander = CommanderBuilder::new(paladin_port)
    .strategy(BattalionStrategy::Auto)
    .paladins(paladins)
    .config(config)
    .build()?;

let result = commander.execute(input).await?;
// Metadata automatically written to: ./battalion_metadata/{strategy}_{timestamp}_{uuid}.json
```

**Metadata File Naming Convention:**

- Format: `{strategy}_{timestamp}_{uuid}.json`
- Example: `Formation_20240315_143022_a1b2c3d4.json`
- Components:
  - `strategy`: Battalion strategy used (Formation, Phalanx, Campaign, etc.)
  - `timestamp`: ISO 8601 format (YYYYMMDD_HHMMSS)
  - `uuid`: Unique identifier (first 8 characters of Battalion ID)

**JSON Structure:**

```json
{
  "battalion_id": "a1b2c3d4-e5f6-7890-abcd-ef1234567890",
  "battalion_name": "audited_battalion",
  "strategy_used": "Formation",
  "started_at": "2024-03-15T14:30:22.123Z",
  "completed_at": "2024-03-15T14:31:15.456Z",
  "duration_ms": 53333,
  "status": "Completed",
  "paladin_success_count": 3,
  "paladin_failure_count": 0,
  "total_tokens": 1520,
  "paladin_results": [
    {
      "paladin_name": "Analyzer",
      "status": "Success",
      "output": "Analysis complete: ...",
      "execution_time_ms": 1500,
      "token_count": 450,
      "loop_count": 1
    }
  ],
  "per_paladin_times": {
    "Analyzer": 1500,
    "Enhancer": 1800,
    "Reviewer": 1200
  },
  "per_paladin_tokens": {
    "Analyzer": {
      "prompt_tokens": 150,
      "completion_tokens": 300,
      "total_tokens": 450
    }
  },
  "strategy_selection_reasoning": "Input contains 'sequential' keyword",
  "strategy_selection_time_ms": 2
}
```

**Field Descriptions:**

| Field | Type | Description |
|-------|------|-------------|
| `battalion_id` | UUID | Unique identifier for this execution |
| `battalion_name` | String | Configuration name from BattalionConfig |
| `strategy_used` | String | Actual strategy executed (may differ from requested in Auto mode) |
| `started_at` / `completed_at` | ISO 8601 | Execution timestamps with millisecond precision |
| `duration_ms` | Integer | Total execution time in milliseconds |
| `status` | String | "Completed", "Failed", "PartialSuccess", "Timeout" |
| `paladin_success_count` | Integer | Number of Paladins that completed successfully |
| `paladin_failure_count` | Integer | Number of Paladins that failed |
| `total_tokens` | Integer | Sum of all token usage across all Paladins |
| `paladin_results` | Array | Detailed results for each Paladin execution |
| `per_paladin_times` | Object | Execution time (ms) per Paladin by name |
| `per_paladin_tokens` | Object | Token breakdown per Paladin (prompt, completion, total) |
| `strategy_selection_reasoning` | String | Auto mode decision explanation (null for explicit strategies) |
| `strategy_selection_time_ms` | Integer | Overhead for strategy selection (0 for explicit) |

**Use Cases:**

```rust
// Production audit trail
let config = BattalionConfig::new("production_api_handler")
    .with_metadata_dir(PathBuf::from("/var/log/battalion"))
    .with_timeout(60);

// Cost optimization analysis
let config = BattalionConfig::new("cost_tracking")
    .with_metadata_dir(PathBuf::from("./cost_analysis"));

// Performance profiling
let config = BattalionConfig::new("profiling_run")
    .with_metadata_dir(PathBuf::from("./performance_data"));
```

**Configuration via YAML:**

```yaml
battalion:
  metadata_output_dir: "./battalion_metadata"
  default_timeout: 300
  error_strategy: "RetryThenContinue"
```

**Benefits:**

- ✅ **Zero Performance Impact**: Async file I/O, non-blocking
- ✅ **Complete Audit Trail**: Every execution fully documented
- ✅ **Cost Transparency**: Per-Paladin token tracking for billing
- ✅ **Debugging Aid**: Capture execution state before failures
- ✅ **Compliance Ready**: Tamper-evident JSON with timestamps

### Best Practices

#### Use Auto Mode for Flexibility

```rust
// Good: Let Commander optimize
let commander = CommanderBuilder::new(paladin_port)
    .strategy(BattalionStrategy::Auto)
    .paladins(paladins)
    .build()?;
```

#### Use Explicit Strategies for Predictability

```rust
// Good: Known pattern, explicit selection
let commander = CommanderBuilder::new(paladin_port)
    .strategy(BattalionStrategy::Formation)
    .paladins(pipeline_paladins)
    .build()?;
```

#### Configure Timeouts Appropriately

```rust
// Good: Realistic timeout with buffer
let config = BattalionConfig::new("batch_job")
    .with_timeout(600); // 10 minutes for batch processing
```

#### Use RetryThenContinue in Production

```rust
// Best for production
let config = BattalionConfig::new("production")
    .with_error_strategy(ErrorStrategy::RetryThenContinue)
    .with_retry_policy(RetryPolicy { max_attempts: 3, ..Default::default() });
```

#### Monitor Telemetry

```rust
let result = commander.execute(input).await?;
metrics.record_execution_time(
    result.completed_at.signed_duration_since(result.started_at).num_milliseconds()
);
metrics.record_success_rate(
    result.paladin_success_count,
    result.paladin_failure_count
);
```

### Performance Characteristics

- **Auto Mode Overhead**: 0-5ms for strategy selection
- **Timeout Enforcement**: Tokio-based, minimal overhead
- **Telemetry Collection**: <1ms overhead
- **Builder Validation**: Compile-time + runtime validation
- **Strategy Delegation**: Zero-cost abstraction after selection

---

## Configuration

### BattalionConfig

```rust
use paladin::core::platform::container::battalion::{BattalionConfig, ErrorStrategy, RetryPolicy};

let config = BattalionConfig {
    name: "research_battalion".to_string(),
    description: Some("Research and analysis workflow".to_string()),
    timeout_seconds: 300, // 5 minute timeout
    error_strategy: ErrorStrategy::RetryThenContinue,
    retry_policy: RetryPolicy {
        max_attempts: 3,
        exponential_backoff: true,
        jitter: true,
        base_delay: Duration::from_millis(100),
        max_delay: Duration::from_secs(10),
    },
    metadata_output_dir: Some(PathBuf::from("./battalion_metadata")),
};
```

### Configuration Options

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `name` | `String` | Auto-generated UUID | Battalion identifier |
| `description` | `Option<String>` | `None` | Human-readable description |
| `timeout_seconds` | `u64` | `300` | Maximum execution time |
| `error_strategy` | `ErrorStrategy` | `FailFast` | How to handle Paladin failures |
| `retry_policy` | `RetryPolicy` | See below | Retry configuration |
| `metadata_output_dir` | `Option<PathBuf>` | `None` | Where to save execution metadata |

---

## Error Handling

### Error Strategies

**1. FailFast (Default)**
- Stop execution on first Paladin failure
- Return error immediately
- **Use when**: Each step is critical, failures are unacceptable

```rust
let config = BattalionConfig {
    error_strategy: ErrorStrategy::FailFast,
    ..Default::default()
};
```

**2. ContinueOnError**
- Continue executing even if some Paladins fail
- Collect all errors, return at end
- **Use when**: Partial results are valuable

```rust
let config = BattalionConfig {
    error_strategy: ErrorStrategy::ContinueOnError,
    ..Default::default()
};
```

**3. RetryThenContinue**
- Retry failed Paladin up to `max_attempts`
- If still fails, continue to next
- **Use when**: Transient failures expected (network issues, API rate limits)

```rust
let config = BattalionConfig {
    error_strategy: ErrorStrategy::RetryThenContinue,
    retry_policy: RetryPolicy {
        max_attempts: 3,
        exponential_backoff: true,
        jitter: true,
        base_delay: Duration::from_millis(100),
        max_delay: Duration::from_secs(10),
    },
    ..Default::default()
};
```

### Retry Policy

**Exponential Backoff Formula**:
```
delay = min(base_delay * 2^attempt, max_delay)
```

**With Jitter** (recommended to prevent thundering herd):
```
actual_delay = random(0.5 * delay, delay)
```

**Example Retry Sequence**:
```
Attempt 1: 100ms
Attempt 2: 200ms
Attempt 3: 400ms (with jitter: 200-400ms)
```

---

## Performance

### Benchmarks

Tested on: Intel i7, 32GB RAM, Rust 1.93

| Metric | Value | Notes |
|--------|-------|-------|
| **Orchestration Overhead** | <10ms | Per Battalion, with fast mock Paladins |
| **Formation (10 Paladins)** | ~110ms | Sequential, 10ms per Paladin |
| **Phalanx (10 Paladins)** | ~50ms | Concurrent execution |
| **Concurrent Battalions** | 100+ | Tested with Formation and Phalanx |
| **Memory Footprint** | ~1MB | Per Battalion instance |
| **Throughput** | 1000+ | Small Formations per second |

### Performance Tips

1. **Use Phalanx for Independent Tasks**: 10x speedup vs Formation for parallelizable work
2. **Limit Concurrency**: Default semaphore allows 10 concurrent Paladins in Phalanx
3. **Tune Timeouts**: Set realistic timeouts based on LLM latency (typically 1-10s per call)
4. **Batch Processing**: Process multiple inputs with same Battalion configuration
5. **Monitor Token Usage**: Track PaladinResult.token_count to manage LLM costs

### Scaling Limits

- **Formation**: Tested up to 100 Paladins sequentially
- **Phalanx**: Tested up to 50 concurrent Paladins
- **Campaign**: Tested graphs with 20 nodes, 30 edges
- **Chain of Command**: Tested 1 commander + 10 specialists

---

## Best Practices

### 1. Choose the Right Pattern

```
┌─────────────────────────────────────────────────────────────┐
│ Decision Tree                                                │
├─────────────────────────────────────────────────────────────┤
│ Need sequential processing?                                 │
│   → Yes: Formation                                           │
│   → No: Continue...                                          │
│                                                              │
│ Tasks independent and parallelizable?                       │
│   → Yes: Phalanx                                             │
│   → No: Continue...                                          │
│                                                              │
│ Need conditional routing/branching?                         │
│   → Yes: Campaign                                            │
│   → No: Continue...                                          │
│                                                              │
│ Need intelligent task delegation?                           │
│   → Yes: Chain of Command                                    │
└─────────────────────────────────────────────────────────────┘
```

### 2. Design Paladin System Prompts

**Formation**: Make each Paladin aware it's in a pipeline
```rust
create_paladin("step2",
    "You are step 2 in a 3-step pipeline. \
     Input is from step 1 (data extractor). \
     Your output goes to step 3 (summarizer).")
```

**Phalanx**: Ensure consistent output format for aggregation
```rust
create_paladin("analyst1",
    "Provide your analysis in format: VERDICT: [approve|reject], REASON: [text]")
```

**Campaign**: Include routing hints in prompts
```rust
create_paladin("classifier",
    "Classify input as 'technical' or 'general'. \
     Output ONLY the classification word.")
```

**Chain of Command**: Train commander to output specialist names
```rust
create_paladin("commander",
    "Available specialists: database_expert, api_specialist, analytics_pro. \
     Output format: SELECT: [specialist_name(s)], REASON: [why]")
```

### 3. Error Handling Strategy

```rust
// Critical pipeline - fail fast
let critical_formation = Formation::new(paladins, BattalionConfig {
    error_strategy: ErrorStrategy::FailFast,
    ..Default::default()
})?;

// Research task - collect all perspectives
let research_phalanx = Phalanx::new(paladins, BattalionConfig {
    error_strategy: ErrorStrategy::ContinueOnError,
    ..Default::default()
})?;

// External API calls - retry transient failures
let api_campaign = Campaign::new(BattalionConfig {
    error_strategy: ErrorStrategy::RetryThenContinue,
    retry_policy: RetryPolicy {
        max_attempts: 3,
        exponential_backoff: true,
        jitter: true,
        base_delay: Duration::from_millis(500),
        max_delay: Duration::from_secs(5),
    },
    ..Default::default()
})?;
```

### 4. Testing

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use paladin::paladin_ports::output::paladin_port::PaladinPort;

    // Create mock PaladinPort for testing
    struct MockPort;

    #[async_trait]
    impl PaladinPort for MockPort {
        async fn execute(&self, paladin: &Paladin, input: &str)
            -> Result<PaladinResult, PaladinError>
        {
            Ok(PaladinResult {
                output: format!("Mock: {}", input),
                token_count: 10,
                execution_time_ms: 5,
                loop_count: 1,
                stop_reason: StopReason::Completed,
            })
        }

        // ... implement other required methods
    }

    #[tokio::test]
    async fn test_formation_pipeline() {
        let mock_port = Arc::new(MockPort);
        let service = FormationExecutionService::new(mock_port);

        // Test your Battalion logic
    }
}
```

---

## API Reference

### Core Types

```rust
// Domain layer (src/core/platform/container/battalion/)
pub struct Formation { /* ... */ }
pub struct Phalanx { /* ... */ }
pub struct Campaign { /* ... */ }
pub struct ChainOfCommand { /* ... */ }

pub struct BattalionConfig { /* ... */ }
pub enum ErrorStrategy { FailFast, ContinueOnError, RetryThenContinue }
pub struct RetryPolicy { /* ... */ }
pub enum BattalionStatus { Idle, Running, Paused, Completed, Failed, Cancelled }
pub struct BattalionResult { /* ... */ }
pub enum BattalionError { /* ... */ }

// Application layer (src/application/services/battalion/)
pub struct FormationExecutionService { /* ... */ }
pub struct PhalanxExecutionService { /* ... */ }
pub struct CampaignExecutionService { /* ... */ }
pub struct ChainOfCommandExecutionService { /* ... */ }
```

### Key Methods

#### Formation
```rust
impl Formation {
    pub fn new(paladins: Vec<Paladin>, config: BattalionConfig) -> Result<Self, BattalionError>;
    pub fn validate(&self) -> Result<(), BattalionError>;
}

impl FormationExecutionService {
    pub fn new(paladin_port: Arc<dyn PaladinPort>) -> Self;
    pub async fn execute(&self, formation: &Formation, input: &str) -> Result<BattalionResult, BattalionError>;
}
```

#### Phalanx
```rust
impl Phalanx {
    pub fn new(paladins: Vec<Paladin>, config: BattalionConfig) -> Result<Self, BattalionError>;
    pub fn with_aggregation(self, strategy: AggregationStrategy) -> Self;
}

impl PhalanxExecutionService {
    pub fn new(paladin_port: Arc<dyn PaladinPort>) -> Self;
    pub async fn execute(&self, phalanx: &Phalanx, input: &str) -> Result<BattalionResult, BattalionError>;
}
```

#### Campaign
```rust
impl Campaign {
    pub fn new(config: BattalionConfig) -> Result<Self, BattalionError>;
    pub fn add_paladin(&mut self, name: impl Into<String>, paladin: Paladin) -> Result<(), BattalionError>;
    pub fn add_edge(&mut self, from: impl Into<String>, to: impl Into<String>, condition: EdgeCondition, transform: Option<String>) -> Result<(), BattalionError>;
    pub fn set_entry_points(&mut self, entry_points: Vec<String>) -> Result<(), BattalionError>;
    pub fn validate(&self) -> Result<(), BattalionError>;
}
```

#### Chain of Command
```rust
impl ChainOfCommand {
    pub fn new(commander: Paladin, specialists: Vec<Paladin>, config: BattalionConfig) -> Result<Self, BattalionError>;
    pub fn with_strategy(self, strategy: DelegationStrategy) -> Self;
}
```

---

## Examples

See the `examples/` directory for complete runnable examples:

- `examples/formation_sequential.rs` - Multi-step analysis pipeline
- `examples/phalanx_parallel.rs` - Concurrent analysis with majority voting
- `examples/campaign_workflow.rs` - Complex conditional routing DAG
- `examples/chain_of_command_delegation.rs` - All 4 delegation strategies

Run examples:
```bash
cargo run --example formation_sequential
cargo run --example phalanx_parallel
cargo run --example campaign_workflow
cargo run --example chain_of_command_delegation
```

---

## Troubleshooting

### Common Issues

**1. "Formation requires at least 2 Paladins"**
- Solution: Add more Paladins to your Formation

**2. "Cycle detected in Campaign graph"**
- Solution: Use `campaign.validate()` to check for cycles before execution
- Campaigns must be DAGs (directed acyclic graphs)

**3. "Phalanx majority requires ≥3 Paladins"**
- Solution: Use `AggregationStrategy::CollectAll` or add more Paladins

**4. "Timeout exceeded"**
- Solution: Increase `timeout_seconds` in BattalionConfig or optimize Paladin prompts

**5. "No entry points defined for Campaign"**
- Solution: Call `campaign.set_entry_points(vec!["start_node"])?` before execution

---

## Architecture Notes

### Hexagonal Architecture Layers

```
┌──────────────────────────────────────────────┐
│ Infrastructure Layer (Adapters)              │
│ - LLM adapters (OpenAI, DeepSeek, Anthropic) │
│ - Garrison (memory) adapters                 │
│ - Arsenal (tool) adapters                    │
└─────────────────┬────────────────────────────┘
                  │
┌─────────────────┴────────────────────────────┐
│ Application Layer (Ports & Services)         │
│ - BattalionPort trait                        │
│ - *ExecutionService implementations          │
│ - Retry logic, error aggregation utilities   │
└─────────────────┬────────────────────────────┘
                  │
┌─────────────────┴────────────────────────────┐
│ Core Domain Layer (Pure Business Logic)     │
│ - Formation, Phalanx, Campaign, Chain types  │
│ - BattalionConfig, Error types               │
│ - No external dependencies                   │
└──────────────────────────────────────────────┘
```

**Dependency Rule**: Dependencies point inward only. Domain has zero external deps.

---

## Contributing

When adding new Battalion patterns:

1. **Domain Layer**: Define entity in `src/core/platform/container/battalion/`
2. **Application Layer**: Create service in `src/application/services/battalion/`
3. **Tests**: Write unit tests (TDD), integration tests, examples
4. **Documentation**: Update this file, add rustdoc
5. **Performance**: Add load test, verify <1s overhead

---

## License

Same as Paladin project license.

---

## Support

- GitHub Issues: [paladin/issues](https://github.com/your-org/paladin/issues)
- Documentation: [docs/](../docs/)
- Examples: [examples/](../examples/)

---

**Version**: 0.1.0  
**Last Updated**: January 2026  
**Maintainers**: Paladin Core Team
