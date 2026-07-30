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
