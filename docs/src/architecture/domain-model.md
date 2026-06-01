# Paladin Domain Model

This document describes the core domain entities, their relationships, and business rules using Domain-Driven Design (DDD) principles.

## Table of Contents

- [Overview](#overview)
- [Ubiquitous Language](#ubiquitous-language)
- [Bounded Contexts](#bounded-contexts)
- [Domain Entities](#domain-entities)
- [Entity Relationships](#entity-relationships)
- [Aggregates](#aggregates)
- [Value Objects](#value-objects)
- [Domain Events](#domain-events)
- [Business Rules](#business-rules)

## Overview

Paladin's domain model follows Domain-Driven Design principles with a clear **Ubiquitous Language** based on Medieval Military terminology. This creates a consistent vocabulary shared by developers, documentation, and code.

**Core Philosophy**:
- **Rich domain model**: Business logic lives in entities, not services
- **Aggregates**: Clear ownership and transactional boundaries
- **Value objects**: Immutable, validated data structures
- **Domain events**: Capture important state changes

## Ubiquitous Language

### Medieval Military Theme

| Term | Domain Meaning | Code Location |
|------|----------------|---------------|
| **Paladin** | An autonomous AI agent capable of reasoning and action | `core/platform/container/paladin.rs` |
| **Battalion** | A coordinated group of Paladins working together | `core/platform/container/battalion/` |
| **Formation** | Sequential Paladin execution pattern (output N → input N+1) | `battalion/formation.rs` |
| **Phalanx** | Concurrent Paladin execution pattern (parallel processing) | `battalion/phalanx.rs` |
| **Campaign** | Graph/DAG-based Paladin orchestration with conditional routing | `battalion/campaign.rs` |
| **Chain of Command** | Hierarchical Paladin delegation pattern (leader → specialists) | `battalion/chain_of_command.rs` |
| **Commander** | Dynamic Battalion strategy router | `services/battalion/commander.rs` |
| **Garrison** | Paladin memory and conversation context storage | `core/platform/container/garrison.rs` |
| **Arsenal** | Tool and capability registry | `core/platform/container/arsenal.rs` |
| **Armament** | A single tool or capability within the Arsenal | Part of Arsenal |
| **Citadel** | State persistence and checkpoint system | `core/platform/container/citadel.rs` |
| **Herald** | Output formatting and presentation system | `core/platform/container/herald.rs` |
| **Quest** | A task or mission assigned to Paladins | Runtime concept |

## Bounded Contexts

Paladin is organized into distinct bounded contexts with clear boundaries:

```
┌────────────────────────────────────────────────────────────┐
│                 Paladin System                              │
│                                                             │
│  ┌────────────────┐  ┌────────────────┐                   │
│  │  Agent Context │  │  Memory Context │                   │
│  │   (Paladin)    │  │   (Garrison)    │                   │
│  └────────────────┘  └────────────────┘                   │
│                                                             │
│  ┌────────────────┐  ┌────────────────┐                   │
│  │  Tool Context  │  │Orchestration   │                   │
│  │   (Arsenal)    │  │   (Battalion)   │                   │
│  └────────────────┘  └────────────────┘                   │
│                                                             │
│  ┌────────────────┐  ┌────────────────┐                   │
│  │  State Context │  │ Output Context  │                   │
│  │   (Citadel)    │  │   (Herald)      │                   │
│  └────────────────┘  └────────────────┘                   │
└────────────────────────────────────────────────────────────┘
```

### 1. Agent Context (Paladin)

**Responsibility**: Autonomous AI agent execution and lifecycle

**Key Concepts**:
- Paladin configuration and state
- Execution loop management
- Stop conditions and max loops
- Temperature and model settings

### 2. Memory Context (Garrison)

**Responsibility**: Conversation history and knowledge storage

**Key Concepts**:
- Conversation entries (user, assistant, system, tool)
- Memory windowing
- Token management
- Semantic search

### 3. Tool Context (Arsenal)

**Responsibility**: External tool integration and execution

**Key Concepts**:
- Tool definitions (Armament)
- Tool invocation (ArmamentCall)
- Tool results (ArmamentResult)
- MCP protocol integration

### 4. Orchestration Context (Battalion)

**Responsibility**: Multi-agent coordination patterns

**Key Concepts**:
- Formation (sequential)
- Phalanx (concurrent)
- Campaign (graph)
- Chain of Command (hierarchical)

### 5. State Context (Citadel)

**Responsibility**: Checkpoint and recovery management

**Key Concepts**:
- State snapshots
- Autosave functionality
- Recovery points
- Rollback capabilities

### 6. Output Context (Herald)

**Responsibility**: Output formatting and presentation

**Key Concepts**:
- Format types (JSON, Markdown, HTML, etc.)
- Streaming output
- Validation
- Post-processing

## Domain Entities

### Paladin

The central entity representing an autonomous AI agent.

```rust
/// Paladin data payload
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaladinData {
    /// System prompt defining Paladin behavior
    pub system_prompt: String,

    /// Human-readable name for the Paladin
    pub name: String,

    /// User name for personalization
    pub user_name: String,

    /// LLM model to use (e.g., "gpt-4", "claude-3-opus")
    pub model: String,

    /// Sampling temperature (0.0 - 2.0)
    pub temperature: f32,

    /// Maximum reasoning loops before stopping
    pub max_loops: u32,

    /// Words that trigger immediate stop
    pub stop_words: Vec<String>,

    /// Current execution status
    pub status: PaladinStatus,
}

/// Paladin entity using Node pattern
pub type Paladin = Node<PaladinData>;

/// Paladin execution states
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PaladinStatus {
    /// Not currently executing
    Idle,

    /// Actively reasoning
    Running,

    /// Successfully completed
    Complete,

    /// Stopped due to condition (max_loops, stop_word)
    Stopped(StopReason),

    /// Encountered an error
    Failed(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StopReason {
    MaxLoops,
    StopWord(String),
    Timeout,
    UserInterrupt,
}
```

**Invariants**:
- `system_prompt` must not be empty
- `temperature` must be between 0.0 and 2.0
- `max_loops` must be > 0
- `name` must not be empty

**Behavior**:
```rust
impl PaladinData {
    /// Validate Paladin configuration
    pub fn validate(&self) -> Result<(), PaladinError> {
        if self.system_prompt.is_empty() {
            return Err(PaladinError::ConfigurationError(
                "System prompt is required".into()
            ));
        }

        if !(0.0..=2.0).contains(&self.temperature) {
            return Err(PaladinError::ConfigurationError(
                format!("Temperature {} must be between 0.0 and 2.0", self.temperature)
            ));
        }

        if self.max_loops == 0 {
            return Err(PaladinError::ConfigurationError(
                "max_loops must be greater than 0".into()
            ));
        }

        Ok(())
    }

    /// Check if stop word is present in text
    pub fn has_stop_word(&self, text: &str) -> Option<String> {
        self.stop_words.iter()
            .find(|word| text.contains(word.as_str()))
            .cloned()
    }
}
```

### Battalion

Abstract base for multi-Paladin orchestration.

```rust
/// Battalion configuration
#[derive(Debug, Clone, Builder, Serialize, Deserialize)]
pub struct BattalionConfig {
    pub name: String,
    pub description: String,
    pub error_strategy: ErrorStrategy,
    pub max_retries: u32,
    pub timeout: Option<Duration>,
}

/// Battalion execution result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BattalionResult {
    pub battalion_id: Uuid,
    pub name: String,
    pub final_output: String,
    pub individual_results: Vec<PaladinResult>,
    pub execution_time: Duration,
    pub status: BattalionStatus,
}

/// Error handling strategies
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ErrorStrategy {
    /// Stop immediately on first error
    FailFast,

    /// Continue executing remaining Paladins
    Continue,

    /// Retry failed Paladin before continuing
    RetryThenContinue,
}
```

**Subtypes**:

#### Formation (Sequential)

```rust
/// Sequential multi-Paladin execution
#[derive(Debug, Clone)]
pub struct Formation {
    pub id: Uuid,
    pub name: String,
    pub paladins: Vec<Paladin>,
    pub shared_context: Option<String>,
}

impl Formation {
    /// Create new Formation
    pub fn new(name: &str, paladins: Vec<Paladin>) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: name.to_string(),
            paladins,
            shared_context: None,
        }
    }

    /// Add shared context prepended to each Paladin
    pub fn with_shared_context(mut self, context: &str) -> Self {
        self.shared_context = Some(context.to_string());
        self
    }

    /// Validate Formation configuration
    pub fn validate(&self) -> Result<(), BattalionError> {
        if self.paladins.is_empty() {
            return Err(BattalionError::ConfigurationError(
                "Formation must have at least one Paladin".into()
            ));
        }

        for paladin in &self.paladins {
            paladin.data.validate()
                .map_err(|e| BattalionError::PaladinError(e))?;
        }

        Ok(())
    }
}
```

#### Phalanx (Concurrent)

```rust
/// Concurrent multi-Paladin execution
#[derive(Debug, Clone)]
pub struct Phalanx {
    pub id: Uuid,
    pub name: String,
    pub paladins: Vec<Paladin>,
    pub aggregation: AggregationStrategy,
}

/// Result aggregation strategies
#[derive(Debug, Clone)]
pub enum AggregationStrategy {
    /// Return all results as list
    All,

    /// Concatenate all outputs
    Concatenate,

    /// Take first successful result
    FirstSuccess,

    /// Use voting/consensus
    Consensus,

    /// Custom aggregation function
    Custom(Arc<dyn Fn(Vec<PaladinResult>) -> String + Send + Sync>),
}
```

#### Campaign (Graph)

```rust
/// Graph-based multi-Paladin orchestration
#[derive(Debug)]
pub struct Campaign {
    pub id: Uuid,
    pub name: String,
    pub graph: DiGraph<Paladin, CampaignEdge>,
    pub entry_points: Vec<NodeIndex>,
}

/// Edge with conditional execution
#[derive(Debug, Clone)]
pub struct CampaignEdge {
    pub condition: Option<EdgeCondition>,
    pub transform: Option<Arc<dyn Fn(&str) -> String + Send + Sync>>,
}

/// Edge execution conditions
#[derive(Debug, Clone)]
pub enum EdgeCondition {
    Always,
    OutputContains(String),
    OutputMatches(regex::Regex),
    Custom(Arc<dyn Fn(&str) -> bool + Send + Sync>),
}

impl Campaign {
    /// Validate Campaign is a valid DAG
    pub fn validate(&self) -> Result<(), CampaignError> {
        // Check for cycles
        if !petgraph::algo::is_cyclic_directed(&self.graph) {
            return Err(CampaignError::InvalidGraph(
                "Campaign contains cycles (must be DAG)".into()
            ));
        }

        // Check entry points exist
        for &node_idx in &self.entry_points {
            if self.graph.node_weight(node_idx).is_none() {
                return Err(CampaignError::InvalidGraph(
                    format!("Entry point {:?} does not exist", node_idx)
                ));
            }
        }

        Ok(())
    }
}
```

#### Chain of Command

```rust
/// Hierarchical delegation pattern
#[derive(Debug)]
pub struct ChainOfCommand {
    pub id: Uuid,
    pub name: String,
    pub commander: Paladin,
    pub specialists: Vec<Paladin>,
    pub delegation_strategy: DelegationStrategy,
}

/// Delegation strategies
#[derive(Debug, Clone)]
pub enum DelegationStrategy {
    /// Commander analyzes and chooses specialists
    CommanderChoice,

    /// Delegate to all specialists
    Broadcast,

    /// Round-robin distribution
    RoundRobin,

    /// Custom logic
    Custom(Arc<dyn Fn(&str, &[Paladin]) -> Vec<usize> + Send + Sync>),
}
```

### Garrison

Memory storage for Paladin conversations.

```rust
/// Single memory entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GarrisonEntry {
    pub id: Uuid,
    pub role: ConversationRole,
    pub content: String,
    pub timestamp: DateTime<Utc>,
    pub metadata: HashMap<String, String>,
    pub token_count: Option<u32>,
}

/// Conversation roles
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConversationRole {
    System,    // System prompts
    User,      // User messages
    Assistant, // Paladin responses
    Tool,      // Tool execution results
}

/// Conversation history with windowing
#[derive(Debug, Clone)]
pub struct ConversationHistory {
    entries: VecDeque<GarrisonEntry>,
    max_entries: usize,
    max_tokens: Option<u32>,
}

impl ConversationHistory {
    /// Add entry, respecting limits
    pub fn add(&mut self, entry: GarrisonEntry) {
        if self.entries.len() >= self.max_entries {
            self.entries.pop_front();
        }

        self.entries.push_back(entry);
    }

    /// Get entries within token window
    pub fn get_window(&self, max_tokens: u32) -> Vec<GarrisonEntry> {
        let mut result = Vec::new();
        let mut token_sum = 0u32;

        for entry in self.entries.iter().rev() {
            let entry_tokens = entry.token_count.unwrap_or(0);

            if token_sum + entry_tokens > max_tokens {
                break;
            }

            token_sum += entry_tokens;
            result.push(entry.clone());
        }

        result.reverse();
        result
    }
}
```

### Arsenal

Tool registry and execution system.

```rust
/// Tool definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Armament {
    pub name: String,
    pub description: String,
    pub schema: ToolSchema,
    pub required_params: Vec<String>,
}

/// Tool invocation request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArmamentCall {
    pub tool_name: String,
    pub parameters: HashMap<String, Value>,
    pub call_id: Uuid,
}

/// Tool execution result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArmamentResult {
    pub call_id: Uuid,
    pub success: bool,
    pub output: String,
    pub error: Option<String>,
    pub execution_time_ms: u64,
}

/// Tool parameter schema
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolSchema {
    pub parameters: Vec<ToolParameter>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolParameter {
    pub name: String,
    pub param_type: ParamType,
    pub description: String,
    pub required: bool,
}
```

### Citadel

State checkpoint and recovery system.

```rust
/// State checkpoint
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Checkpoint {
    pub id: Uuid,
    pub timestamp: DateTime<Utc>,
    pub paladin_state: PaladinState,
    pub garrison_snapshot: Vec<GarrisonEntry>,
    pub metadata: HashMap<String, String>,
}

/// Recoverable Paladin state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaladinState {
    pub paladin_id: Uuid,
    pub loop_count: u32,
    pub last_input: String,
    pub last_output: String,
    pub status: PaladinStatus,
}
```

## Entity Relationships

```
┌──────────────────────────────────────────────────────────────┐
│                   Entity Relationships                        │
└──────────────────────────────────────────────────────────────┘

                        ┌──────────────┐
                        │   Paladin    │
                        └──────┬───────┘
                               │
                ┌──────────────┼──────────────┐
                │              │              │
         ┌──────▼──────┐ ┌────▼────┐   ┌────▼─────┐
         │  Garrison   │ │ Arsenal │   │ Citadel  │
         │  (memory)   │ │ (tools) │   │ (state)  │
         └─────────────┘ └─────────┘   └──────────┘

                        ┌──────────────┐
                        │   Battalion  │
                        └──────┬───────┘
                               │
                     ┌─────────┴─────────┐
                     │   contains 1..N    │
                     ▼                    ▼
              ┌──────────────┐    ┌──────────────┐
              │   Paladin    │    │   Paladin    │
              └──────────────┘    └──────────────┘
```

**Relationships**:

1. **Paladin ↔ Garrison** (1:0..1)
   - Paladin may have a Garrison for memory
   - Garrison belongs to one Paladin

2. **Paladin ↔ Arsenal** (1:0..N)
   - Paladin may have access to multiple Armaments
   - Armaments can be shared across Paladins

3. **Paladin ↔ Citadel** (1:0..1)
   - Paladin may have a Citadel for state persistence
   - Citadel stores checkpoints for one Paladin

4. **Battalion ↔ Paladin** (1:N)
   - Battalion coordinates multiple Paladins
   - Paladins can be part of multiple Battalions

5. **GarrisonEntry ↔ ArmamentResult** (0..1:0..1)
   - Tool results are stored as Garrison entries
   - Linked by metadata

## Aggregates

### Paladin Aggregate

**Aggregate Root**: `Paladin`

**Entities**:
- `PaladinData` (root)
- `PaladinConfig`

**Value Objects**:
- `Temperature`
- `Model`
- `StopWords`

**Invariants**:
- System prompt must not be empty
- Temperature within valid range
- Max loops > 0

**Transactional Boundary**:
- All Paladin configuration changes are atomic
- Configuration validation happens before persistence

### Battalion Aggregate

**Aggregate Root**: `Battalion` (Formation, Phalanx, Campaign, ChainOfCommand)

**Entities**:
- `Battalion` (root)
- `BattalionConfig`

**References** (not owned):
- Collection of `Paladin` references

**Invariants**:
- Must have at least one Paladin
- All referenced Paladins must be valid
- Graph must be acyclic (for Campaign)

### Garrison Aggregate

**Aggregate Root**: `Garrison`

**Entities**:
- `ConversationHistory` (root)

**Value Objects**:
- Collection of `GarrisonEntry`

**Invariants**:
- Entries ordered chronologically
- Total tokens ≤ max_tokens (if set)
- Entry count ≤ max_entries

## Value Objects

### Temperature

```rust
/// Temperature value object
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Temperature(f32);

impl Temperature {
    pub fn new(value: f32) -> Result<Self, ValidationError> {
        if !(0.0..=2.0).contains(&value) {
            return Err(ValidationError::OutOfRange {
                field: "temperature",
                min: 0.0,
                max: 2.0,
                actual: value,
            });
        }
        Ok(Self(value))
    }

    pub fn value(&self) -> f32 {
        self.0
    }
}
```

### TokenCount

```rust
/// Token count value object
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct TokenCount(u32);

impl TokenCount {
    pub fn new(count: u32) -> Self {
        Self(count)
    }

    pub fn value(&self) -> u32 {
        self.0
    }
}

impl std::ops::Add for TokenCount {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self(self.0 + other.0)
    }
}
```

### Model

```rust
/// LLM model identifier
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Model(String);

impl Model {
    pub fn new(name: impl Into<String>) -> Self {
        Self(name.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Check if model supports function calling
    pub fn supports_tools(&self) -> bool {
        matches!(
            self.0.as_str(),
            "gpt-4" | "gpt-4-turbo" | "gpt-3.5-turbo" | "claude-3-opus" | "claude-3-sonnet"
        )
    }
}
```

## Domain Events

Events that capture important state changes:

```rust
/// Domain events
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PaladinEvent {
    /// Paladin was created
    Created {
        paladin_id: Uuid,
        name: String,
        timestamp: DateTime<Utc>,
    },

    /// Paladin started executing
    ExecutionStarted {
        paladin_id: Uuid,
        input: String,
        timestamp: DateTime<Utc>,
    },

    /// Paladin completed execution
    ExecutionCompleted {
        paladin_id: Uuid,
        output: String,
        loops_used: u32,
        timestamp: DateTime<Utc>,
    },

    /// Paladin invoked a tool
    ToolInvoked {
        paladin_id: Uuid,
        tool_name: String,
        parameters: HashMap<String, Value>,
        timestamp: DateTime<Utc>,
    },

    /// Paladin stopped due to condition
    Stopped {
        paladin_id: Uuid,
        reason: StopReason,
        timestamp: DateTime<Utc>,
    },

    /// Paladin encountered error
    Failed {
        paladin_id: Uuid,
        error: String,
        timestamp: DateTime<Utc>,
    },
}
```

**Event Publishing**:
```rust
pub trait EventPublisher: Send + Sync {
    fn publish(&self, event: PaladinEvent);
}

// Example usage in service
impl PaladinExecutionService {
    pub async fn execute(&self, paladin: &Paladin, input: &str) -> Result<PaladinResult> {
        self.event_publisher.publish(PaladinEvent::ExecutionStarted {
            paladin_id: paladin.id,
            input: input.to_string(),
            timestamp: Utc::now(),
        });

        // ... execution logic

        self.event_publisher.publish(PaladinEvent::ExecutionCompleted {
            paladin_id: paladin.id,
            output: result.content.clone(),
            loops_used: result.loops_used,
            timestamp: Utc::now(),
        });

        Ok(result)
    }
}
```

## Business Rules

### Paladin Rules

1. **System Prompt Required**
   ```rust
   if paladin.system_prompt.is_empty() {
       return Err(PaladinError::InvalidConfiguration("System prompt required"));
   }
   ```

2. **Temperature Bounds**
   ```rust
   if !(0.0..=2.0).contains(&paladin.temperature) {
       return Err(PaladinError::InvalidConfiguration("Temperature must be 0.0-2.0"));
   }
   ```

3. **Max Loops Enforcement**
   ```rust
   if loop_count >= paladin.max_loops {
       return Err(PaladinError::MaxLoopsReached(paladin.max_loops));
   }
   ```

4. **Stop Word Detection**
   ```rust
   if let Some(stop_word) = paladin.has_stop_word(&output) {
       return Ok(PaladinResult::stopped(output, StopReason::StopWord(stop_word)));
   }
   ```

### Battalion Rules

1. **Minimum Paladin Count**
   ```rust
   if battalion.paladins.is_empty() {
       return Err(BattalionError::InvalidConfiguration("At least one Paladin required"));
   }
   ```

2. **Campaign Must Be DAG**
   ```rust
   if petgraph::algo::is_cyclic_directed(&campaign.graph) {
       return Err(CampaignError::CyclicGraph);
   }
   ```

3. **Error Strategy Enforcement**
   ```rust
   match battalion.config.error_strategy {
       ErrorStrategy::FailFast => {
           if result.is_err() {
               return result; // Stop immediately
           }
       }
       ErrorStrategy::Continue => {
           // Log error and continue
       }
       ErrorStrategy::RetryThenContinue => {
           // Retry up to max_retries
       }
   }
   ```

### Garrison Rules

1. **Token Limit Enforcement**
   ```rust
   while total_tokens > garrison.max_tokens {
       garrison.evict_oldest();
   }
   ```

2. **Entry Ordering**
   ```rust
   // Entries must be chronologically ordered
   assert!(entries.windows(2).all(|w| w[0].timestamp <= w[1].timestamp));
   ```

### Arsenal Rules

1. **Required Parameters**
   ```rust
   for param in &armament.required_params {
       if !call.parameters.contains_key(param) {
           return Err(ArsenalError::MissingParameter(param.clone()));
       }
   }
   ```

2. **Tool Validation**
   ```rust
   if !registry.has_tool(&call.tool_name) {
       return Err(ArsenalError::ToolNotFound(call.tool_name));
   }
   ```

## Next Steps

- **[Design Patterns](design-patterns.md)** - Patterns used in Paladin
- **[Hexagonal Design](hexagonal-design.md)** - Port/adapter implementation
- **[Adapter Development](../contributing/adapter-development.md)** - Create custom adapters
