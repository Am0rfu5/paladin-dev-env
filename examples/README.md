# Paladin Examples Gallery

This directory contains comprehensive examples demonstrating Paladin's capabilities. Each example is a fully-functional Rust program that you can run and modify.

## Table of Contents

- [Getting Started](#getting-started)
- [Basic Paladin Examples](#basic-paladin-examples)
- [Memory & Garrison Examples](#memory--garrison-examples)
- [Tool Integration Examples](#tool-integration-examples)
- [Battalion Orchestration Examples](#battalion-orchestration-examples)
- [Output Formatting Examples](#output-formatting-examples)
- [State Management Examples](#state-management-examples)
- [Configuration Examples](#configuration-examples)
- [Advanced Examples](#advanced-examples)
- [Running Examples](#running-examples)

## Getting Started

All examples require:
- Rust 1.70 or later
- API keys for LLM providers (OpenAI, DeepSeek, or Anthropic)
- Docker (for examples using Redis/MinIO)

Set your API key:
```bash
export OPENAI_API_KEY="your-api-key-here"
# or
export DEEPSEEK_API_KEY="your-api-key-here"
# or
export ANTHROPIC_API_KEY="your-api-key-here"
```

## Basic Paladin Examples

### [basic_paladin.rs](basic_paladin.rs)
**Demonstrates:** Creating and executing a simple Paladin agent

The most basic Paladin usage - create an agent with a system prompt and execute a query.

```bash
cargo run --example basic_paladin
```

**Key concepts:**
- PaladinBuilder fluent API
- System prompt configuration
- Simple execution

**Code snippet:**
```rust
let paladin = PaladinBuilder::new(llm_adapter)
    .name("Assistant")
    .system_prompt("You are a helpful AI assistant.")
    .build()?;

let response = paladin.execute("Hello!").await?;
println!("{}", response.content);
```

### [paladin_with_config.rs](paladin_with_config.rs)
**Demonstrates:** Advanced Paladin configuration

Shows how to configure temperature, max loops, stop words, and other parameters.

```bash
cargo run --example paladin_with_config
```

**Key concepts:**
- Temperature tuning
- Maximum loop control
- Stop word configuration
- Timeout settings
- Retry logic

**Code snippet:**
```rust
let paladin = PaladinBuilder::new(llm_adapter)
    .name("ConfiguredAgent")
    .system_prompt("You are a precise assistant.")
    .temperature(0.3)
    .max_loops(5)
    .stop_words(vec!["DONE", "END"])
    .timeout(Duration::from_secs(60))
    .build()?;
```

### [llm_provider_selection.rs](llm_provider_selection.rs)
**Demonstrates:** Using different LLM providers

Shows how to switch between OpenAI, DeepSeek, and Anthropic providers.

```bash
cargo run --example llm_provider_selection
```

**Key concepts:**
- Provider adapters
- Model selection
- API configuration
- Multi-provider support

## Memory & Garrison Examples

### [garrison_in_memory.rs](garrison_in_memory.rs)
**Demonstrates:** In-memory conversation history

Shows basic memory management with the in-memory Garrison.

```bash
cargo run --example garrison_in_memory
```

**Key concepts:**
- Conversation context
- Memory windowing
- Token management
- Multi-turn dialogue

**Code snippet:**
```rust
let garrison = Arc::new(InMemoryGarrison::new(
    GarrisonConfig::default()
        .with_max_entries(100)
        .with_max_tokens(4000)
));

let paladin = PaladinBuilder::new(llm_adapter)
    .with_garrison(garrison)
    .build()?;

// First turn
paladin.execute("My name is Alice").await?;

// Second turn - remembers context
paladin.execute("What's my name?").await?;
```

### [garrison_persistent.rs](garrison_persistent.rs)
**Demonstrates:** SQLite-backed persistent memory

Shows how to save and restore conversation history across sessions.

```bash
cargo run --example garrison_persistent
```

**Key concepts:**
- SQLite storage
- Session management
- Cross-session persistence
- Database migrations

**Code snippet:**
```rust
let garrison = Arc::new(
    SqliteGarrison::new("garrison.db")
        .await?
        .with_session_id(session_id)
);

// History persists across restarts
```

### [garrison_semantic_search.rs](garrison_semantic_search.rs)
**Demonstrates:** Vector embeddings and semantic search

Shows long-term memory with semantic retrieval.

```bash
cargo run --example garrison_semantic_search
```

**Key concepts:**
- Embedding generation
- Vector similarity search
- RAG (Retrieval-Augmented Generation)
- Long-term knowledge

## Tool Integration Examples

### [arsenal_stdio_tools.rs](arsenal_stdio_tools.rs)
**Demonstrates:** MCP STDIO tool servers

Shows how to connect command-line tool servers via MCP protocol.

```bash
# Install MCP server first
uvx mcp-server-fetch

cargo run --example arsenal_stdio_tools
```

**Key concepts:**
- STDIO communication
- MCP protocol
- Tool discovery
- Function calling

**Code snippet:**
```rust
let web_search = MCPStdioAdapter::new()
    .command("uvx")
    .args(vec!["mcp-server-fetch"])
    .build()
    .await?;

let paladin = PaladinBuilder::new(llm_adapter)
    .add_armament(Arc::new(web_search))
    .build()?;

// Paladin automatically uses tools when needed
```

### [arsenal_sse_tools.rs](arsenal_sse_tools.rs)
**Demonstrates:** MCP SSE tool servers

Shows how to connect web-based tool servers via Server-Sent Events.

```bash
cargo run --example arsenal_sse_tools
```

**Key concepts:**
- HTTP/SSE communication
- Authentication
- API integration
- Remote tools

**Code snippet:**
```rust
let api_tools = MCPSseAdapter::new()
    .endpoint("https://api.example.com/mcp")
    .api_key(api_key)
    .build()
    .await?;
```

## Battalion Orchestration Examples

### [formation_sequential.rs](formation_sequential.rs)
**Demonstrates:** Sequential multi-agent execution

Shows Formation pattern where Paladins execute in sequence, passing output.

```bash
cargo run --example formation_sequential
```

**Key concepts:**
- Sequential execution
- Output passing
- Pipeline patterns
- Multi-stage processing

**Code snippet:**
```rust
let researcher = PaladinBuilder::new(llm_adapter.clone())
    .name("Researcher")
    .system_prompt("Research the topic and provide facts.")
    .build()?;

let analyst = PaladinBuilder::new(llm_adapter.clone())
    .name("Analyst")
    .system_prompt("Analyze the research and identify key insights.")
    .build()?;

let writer = PaladinBuilder::new(llm_adapter)
    .name("Writer")
    .system_prompt("Write a clear summary based on the analysis.")
    .build()?;

let formation = Formation::new("ResearchPipeline", vec![
    researcher,
    analyst,
    writer,
]);

let result = formation.execute("Explain quantum computing").await?;
```

### [phalanx_parallel.rs](phalanx_parallel.rs)
**Demonstrates:** Concurrent multi-agent execution

Shows Phalanx pattern where multiple Paladins process the same input in parallel.

```bash
cargo run --example phalanx_parallel
```

**Key concepts:**
- Parallel execution
- Result aggregation
- Concurrent processing
- Multiple perspectives

**Code snippet:**
```rust
let technical = PaladinBuilder::new(llm_adapter.clone())
    .name("TechnicalReviewer")
    .system_prompt("Review for technical accuracy.")
    .build()?;

let security = PaladinBuilder::new(llm_adapter.clone())
    .name("SecurityReviewer")
    .system_prompt("Review for security issues.")
    .build()?;

let ux = PaladinBuilder::new(llm_adapter)
    .name("UXReviewer")
    .system_prompt("Review for user experience.")
    .build()?;

let phalanx = Phalanx::new("CodeReview", vec![
    technical,
    security,
    ux,
]).with_aggregation(AggregationStrategy::Concatenate);

let result = phalanx.execute("Review this code: ...").await?;
```

### [campaign_workflow.rs](campaign_workflow.rs)
**Demonstrates:** Graph-based agent orchestration

Shows Campaign pattern with conditional routing and DAG execution.

```bash
cargo run --example campaign_workflow
```

**Key concepts:**
- Directed Acyclic Graph (DAG)
- Conditional edges
- Branching logic
- Complex workflows

**Code snippet:**
```rust
let mut campaign = Campaign::new("DataProcessing");

// Add nodes
let classifier_idx = campaign.add_paladin(classifier);
let technical_idx = campaign.add_paladin(technical_analyst);
let business_idx = campaign.add_paladin(business_analyst);

// Add conditional edges
campaign.add_edge(
    classifier_idx,
    technical_idx,
    CampaignEdge::new()
        .with_condition(EdgeCondition::OutputContains("technical"))
);

campaign.add_edge(
    classifier_idx,
    business_idx,
    CampaignEdge::new()
        .with_condition(EdgeCondition::OutputContains("business"))
);

let result = campaign.execute("Classify and analyze this document").await?;
```

### [chain_of_command_delegation.rs](chain_of_command_delegation.rs)
**Demonstrates:** Hierarchical agent delegation

Shows Chain of Command pattern with leader and specialist Paladins.

```bash
cargo run --example chain_of_command_delegation
```

**Key concepts:**
- Hierarchical structure
- Dynamic delegation
- Specialist routing
- Result synthesis

**Code snippet:**
```rust
let commander = PaladinBuilder::new(llm_adapter.clone())
    .name("Commander")
    .system_prompt("You coordinate specialists. Analyze tasks and delegate.")
    .build()?;

let specialists = vec![
    database_specialist,
    api_specialist,
    frontend_specialist,
];

let chain = ChainOfCommand::new(
    "DevelopmentTeam",
    commander,
    specialists,
).with_delegation_strategy(DelegationStrategy::CommanderChoice);

let result = chain.execute("Implement user authentication").await?;
```

### [commander_basic.rs](commander_basic.rs)
**Demonstrates:** Basic Commander usage

Shows dynamic Battalion strategy selection based on task.

```bash
cargo run --example commander_basic
```

**Key concepts:**
- Strategy routing
- Task analysis
- Dynamic selection
- Battalion types

### [commander_auto.rs](commander_auto.rs)
**Demonstrates:** Automatic strategy selection

Shows Commander with LLM-based strategy decision.

```bash
cargo run --example commander_auto
```

**Key concepts:**
- Intelligent routing
- LLM-based decisions
- Auto-optimization
- Adaptive orchestration

### [commander_full_config.rs](commander_full_config.rs)
**Demonstrates:** Complete Commander configuration

Shows all Commander features with full customization.

```bash
cargo run --example commander_full_config
```

**Key concepts:**
- Custom routing logic
- Multiple strategies
- Fallback handling
- Advanced configuration

## Output Formatting Examples

### [herald_markdown_output.rs](herald_markdown_output.rs)
**Demonstrates:** Markdown formatting

Shows how to format Paladin output as Markdown.

```bash
cargo run --example herald_markdown_output
```

**Key concepts:**
- Markdown herald
- Code blocks
- Headers and structure
- Documentation generation

**Code snippet:**
```rust
let herald = Arc::new(MarkdownHerald::new()
    .with_code_highlighting(true)
    .with_table_of_contents(true)
);

let paladin = PaladinBuilder::new(llm_adapter)
    .with_herald(herald)
    .build()?;

let response = paladin.execute("Explain Rust ownership").await?;
// Output is formatted Markdown
```

### [herald_json_output.rs](herald_json_output.rs)
**Demonstrates:** Structured JSON output

Shows JSON formatting with schema validation.

```bash
cargo run --example herald_json_output
```

**Key concepts:**
- JSON schema
- Validation
- Structured data
- API responses

**Code snippet:**
```rust
let herald = Arc::new(JsonHerald::new()
    .with_schema(schema)
    .validate_output(true)
);

let paladin = PaladinBuilder::new(llm_adapter)
    .system_prompt("Respond only in JSON: {result: string, confidence: number}")
    .with_herald(herald)
    .build()?;
```

### [herald_streaming.rs](herald_streaming.rs)
**Demonstrates:** Real-time streaming output

Shows streaming with live formatting.

```bash
cargo run --example herald_streaming
```

**Key concepts:**
- Streaming responses
- Real-time formatting
- Progress indicators
- User experience

**Code snippet:**
```rust
let mut stream = paladin.execute_stream("Write a story").await?;

while let Some(chunk) = stream.next().await {
    let chunk = chunk?;
    let formatted = herald.format_chunk(&chunk.content).await?;
    print!("{}", formatted);
    std::io::stdout().flush()?;
}
```

### [herald_custom_formatter.rs](herald_custom_formatter.rs)
**Demonstrates:** Custom Herald implementation

Shows how to create custom output formatters.

```bash
cargo run --example herald_custom_formatter
```

**Key concepts:**
- Herald trait
- Custom formatting
- Format validation
- Extensibility

## State Management Examples

### [citadel_autosave.rs](citadel_autosave.rs)
**Demonstrates:** Automatic state persistence

Shows Citadel autosave functionality for recovery.

```bash
cargo run --example citadel_autosave
```

**Key concepts:**
- State snapshots
- Automatic saving
- Checkpoint creation
- Failure recovery

**Code snippet:**
```rust
let citadel = Arc::new(
    FileCitadel::new("checkpoints/")
        .with_autosave(true)
        .with_interval(Duration::from_secs(30))
);

let paladin = PaladinBuilder::new(llm_adapter)
    .with_citadel(citadel)
    .build()?;
```

### [citadel_restore.rs](citadel_restore.rs)
**Demonstrates:** State restoration after failure

Shows how to recover Paladin state from checkpoints.

```bash
cargo run --example citadel_restore
```

**Key concepts:**
- Checkpoint restoration
- State recovery
- Resume execution
- Fault tolerance

### [battalion_checkpoint_recovery.rs](battalion_checkpoint_recovery.rs)
**Demonstrates:** Battalion state management

Shows checkpoint/recovery for multi-agent orchestration.

```bash
cargo run --example battalion_checkpoint_recovery
```

**Key concepts:**
- Battalion state
- Multi-agent recovery
- Partial execution
- Fault handling

## Configuration Examples

The `cli_configs/` directory contains YAML configuration examples:

### [basic_paladin.yaml](cli_configs/basic_paladin.yaml)
Basic Paladin configuration for CLI usage.

### [advanced_paladin.yaml](cli_configs/advanced_paladin.yaml)
Advanced Paladin with all options configured.

### [formation.yaml](cli_configs/formation.yaml)
Formation Battalion configuration.

### [phalanx.yaml](cli_configs/phalanx.yaml)
Phalanx Battalion configuration.

### [campaign.yaml](cli_configs/campaign.yaml)
Campaign Battalion with graph structure.

### [chain_of_command.yaml](cli_configs/chain_of_command.yaml)
Chain of Command configuration.

## Advanced Examples

### Error Handling Patterns

Most examples include robust error handling:

```rust
use paladin::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load configuration
    let config = load_config()?;
    
    // Initialize adapters with fallback
    let llm_adapter = create_llm_adapter(&config)
        .or_else(|_| create_fallback_adapter())?;
    
    // Create Paladin with retries
    let paladin = PaladinBuilder::new(llm_adapter)
        .max_retries(3)
        .retry_delay(Duration::from_secs(1))
        .build()?;
    
    // Execute with timeout
    let result = tokio::time::timeout(
        Duration::from_secs(60),
        paladin.execute(input)
    ).await??;
    
    Ok(())
}
```

### Logging and Observability

Examples include tracing integration:

```rust
use tracing::{info, warn, error};
use tracing_subscriber;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();
    
    info!("Starting Paladin execution");
    
    let response = paladin.execute(input).await?;
    
    info!(
        tokens = response.token_usage.total_tokens,
        duration = ?response.execution_time,
        "Execution completed"
    );
    
    Ok(())
}
```

## Running Examples

### Run a Specific Example

```bash
cargo run --example basic_paladin
```

### Run with Specific Features

```bash
# With Redis queue support
cargo run --example basic_paladin --features redis-queue

# With S3 storage
cargo run --example basic_paladin --features s3-storage

# All features
cargo run --example basic_paladin --all-features
```

### Run in Release Mode

```bash
cargo run --release --example basic_paladin
```

### Set Log Level

```bash
RUST_LOG=debug cargo run --example basic_paladin
```

### With Custom Config

```bash
cargo run --example basic_paladin -- --config my-config.yml
```

## Example Dependencies

Some examples require external services:

### Start Docker Services

```bash
# All services (Redis, MinIO)
make dev

# Or individually
docker-compose -f docker/docker-compose.yml up redis -d
docker-compose -f docker/docker-compose.yml up minio -d
```

### Install MCP Servers

```bash
# Web search
uvx mcp-server-fetch

# File system
uvx mcp-server-filesystem

# Calculator
uvx mcp-server-calculator
```

## Building a Custom Example

Create a new example in `examples/my_example.rs`:

```rust
use paladin::prelude::*;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load API key
    let api_key = std::env::var("OPENAI_API_KEY")?;
    
    // Create LLM adapter
    let llm_adapter = Arc::new(
        OpenAiAdapter::new()
            .api_key(&api_key)
            .model("gpt-4")
            .build()?
    );
    
    // Create Paladin
    let paladin = PaladinBuilder::new(llm_adapter)
        .name("MyPaladin")
        .system_prompt("You are a helpful assistant.")
        .build()?;
    
    // Execute
    let response = paladin.execute("Hello!").await?;
    println!("{}", response.content);
    
    Ok(())
}
```

Run it:
```bash
cargo run --example my_example
```

## Troubleshooting

### Example Won't Compile

```bash
# Clean and rebuild
cargo clean
cargo build --examples

# Check specific example
cargo check --example basic_paladin
```

### Missing API Key

```bash
# Set in environment
export OPENAI_API_KEY="sk-..."

# Or create .env file
echo "OPENAI_API_KEY=sk-..." > .env

# Then run with dotenv
cargo run --example basic_paladin
```

### Service Connection Errors

```bash
# Check services are running
make health

# Restart services
make dev-restart

# View logs
docker-compose -f docker/docker-compose.yml logs
```

## Learning Path

Recommended order for learning:

1. **Start Simple**
   - `basic_paladin.rs` - Understand core concepts
   - `paladin_with_config.rs` - Learn configuration

2. **Add Memory**
   - `garrison_in_memory.rs` - Basic memory
   - `garrison_persistent.rs` - Persistence

3. **Add Tools**
   - `arsenal_stdio_tools.rs` - Tool integration
   - Custom tool creation

4. **Multi-Agent**
   - `formation_sequential.rs` - Sequential
   - `phalanx_parallel.rs` - Parallel
   - `campaign_workflow.rs` - Advanced

5. **Production Features**
   - `herald_json_output.rs` - Structured output
   - `citadel_autosave.rs` - State management
   - `commander_auto.rs` - Dynamic routing

## Contributing Examples

Want to add an example? See [CONTRIBUTING.md](../docs/contributing/CONTRIBUTING.md) for guidelines.

Good example characteristics:
- Self-contained (single file)
- Well-commented
- Demonstrates one clear concept
- Includes error handling
- Has descriptive output

## Next Steps

- **[Quickstart Guide](../docs/QUICKSTART.md)** - Get started with Paladin
- **[User Guides](../docs/guides/)** - In-depth documentation
- **[API Reference](https://docs.rs/paladin)** - Complete API docs
- **[Architecture](../docs/architecture/)** - System design details

## Questions?

- Check [Documentation](../docs/)
- Open an [Issue](https://github.com/your-org/paladin/issues)
- Join [Discord](https://discord.gg/paladin) (if available)
