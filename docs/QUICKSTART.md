# Quickstart Guide

Get your first Paladin agent running in 15 minutes! This guide will walk you through creating a simple Paladin agent that can answer questions using an LLM.

## Prerequisites

- **Rust**: 1.70 or later ([installation guide](https://rustup.rs/))
- **LLM API Key**: OpenAI, DeepSeek, or Anthropic account
- **Basic Rust knowledge**: Understanding of `cargo` and async/await

## Step 1: Installation

Add Paladin to your `Cargo.toml`:

```toml
[dependencies]
paladin = "0.1"
tokio = { version = "1", features = ["full"] }
```

Or create a new project:

```bash
cargo new my-paladin-agent
cd my-paladin-agent
cargo add paladin
cargo add tokio --features full
```

## Step 2: Set Up Your Environment

Create a `.env` file in your project root:

```bash
# OpenAI
OPENAI_API_KEY=sk-your-api-key-here

# Or DeepSeek
DEEPSEEK_API_KEY=your-deepseek-key

# Or Anthropic
ANTHROPIC_API_KEY=your-anthropic-key
```

**Security Note**: Never commit API keys to version control. Add `.env` to your `.gitignore`.

## Step 3: Create Your First Paladin

Create or edit `src/main.rs`:

```rust
use paladin::prelude::*;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables
    dotenv::dotenv().ok();
    
    // Create an LLM adapter (OpenAI in this example)
    let llm_adapter = Arc::new(
        OpenAiAdapter::new()
            .with_api_key(std::env::var("OPENAI_API_KEY")?)
            .with_model("gpt-4")
            .build()?
    );
    
    // Build a Paladin agent
    let paladin = PaladinBuilder::new(llm_adapter)
        .name("Assistant")
        .system_prompt("You are a helpful AI assistant. Be concise and accurate.")
        .temperature(0.7)
        .max_loops(3)
        .build()?;
    
    // Execute a query
    let response = paladin.execute("What is the capital of France?").await?;
    
    println!("Paladin: {}", response.content);
    
    Ok(())
}
```

## Step 4: Run Your Agent

```bash
cargo run
```

**Expected Output:**

```
Paladin: The capital of France is Paris.
```

## Next Steps

Congratulations! You've created your first Paladin agent. Here's what to explore next:

### 1. Add Memory (Garrison)

Enable conversation context:

```rust
let garrison = Arc::new(InMemoryGarrison::new());

let paladin = PaladinBuilder::new(llm_adapter)
    .name("Assistant")
    .system_prompt("You are a helpful assistant.")
    .with_garrison(garrison)
    .build()?;

// Now the Paladin remembers previous interactions
paladin.execute("My name is Alice").await?;
paladin.execute("What is my name?").await?; // "Your name is Alice"
```

### 2. Add Tools (Arsenal)

Give your Paladin capabilities:

```rust
use paladin::arsenal::*;

// Connect an MCP tool server
let web_search = MCPStdioAdapter::new("uvx", vec!["mcp-web-search"]).await?;

let paladin = PaladinBuilder::new(llm_adapter)
    .name("Research Assistant")
    .system_prompt("You can search the web to answer questions.")
    .add_armament(Arc::new(web_search))
    .build()?;

paladin.execute("What's the latest Rust release?").await?;
```

### 3. Multi-Agent Orchestration (Battalion)

Coordinate multiple Paladins:

```rust
use paladin::battalion::*;

// Sequential execution (Formation)
let analyst = /* create analyst Paladin */;
let writer = /* create writer Paladin */;

let formation = Formation::new()
    .add_paladin(analyst)
    .add_paladin(writer)
    .build()?;

let result = formation.execute("Analyze market trends and write a summary").await?;
```

### 4. Stream Responses

Get real-time output:

```rust
let mut stream = paladin.execute_stream("Tell me a story").await?;

while let Some(chunk) = stream.next().await {
    print!("{}", chunk?);
}
```

## Common Patterns

### Configuration from File

```rust
use paladin::config::ApplicationSettings;

let config = ApplicationSettings::load()?;
let paladin = PaladinBuilder::from_config(&config.paladin)?;
```

### Error Handling

```rust
match paladin.execute(input).await {
    Ok(response) => println!("Success: {}", response.content),
    Err(PaladinError::Timeout(secs)) => {
        eprintln!("Timed out after {} seconds", secs);
    }
    Err(PaladinError::LlmError(msg)) => {
        eprintln!("LLM error: {}", msg);
    }
    Err(e) => eprintln!("Error: {}", e),
}
```

### Async Context

Always run Paladins in an async context:

```rust
#[tokio::main]
async fn main() {
    // Your Paladin code here
}
```

## Troubleshooting

### "API key not found"

Ensure your `.env` file is in the project root and contains the correct variable name:

```bash
OPENAI_API_KEY=sk-...
```

### "Connection timeout"

Check your network connection and API endpoint:

```rust
let llm_adapter = OpenAiAdapter::new()
    .with_timeout(Duration::from_secs(60)) // Increase timeout
    .build()?;
```

### "Rate limit exceeded"

Implement retry logic or use a rate limiter:

```rust
let config = PaladinConfig::default()
    .with_retry_attempts(3)
    .with_retry_delay(Duration::from_secs(5));
```

## Example Projects

Check out complete examples in the [examples/](../examples/) directory:

- `basic_paladin.rs` - Simple question answering
- `garrison_in_memory.rs` - Conversation with memory
- `arsenal_stdio_tools.rs` - Tool integration
- `formation_sequential.rs` - Multi-agent workflows
- `phalanx_parallel.rs` - Concurrent processing

## Learn More

- **[Installation Guide](INSTALLATION.md)** - Detailed setup for all platforms
- **[Configuration Guide](guides/paladin-configuration.md)** - Advanced Paladin options
- **[Battalion Patterns](guides/battalion-patterns.md)** - Multi-agent orchestration
- **[API Reference](https://docs.rs/paladin)** - Complete API documentation

## Get Help

- **Documentation**: [https://github.com/DF3NDR/paladin-dev-env/tree/main/docs](../README.md)
- **Examples**: [https://github.com/DF3NDR/paladin-dev-env/tree/main/examples](../examples/)
- **Issues**: [https://github.com/DF3NDR/paladin-dev-env/issues](https://github.com/DF3NDR/paladin-dev-env/issues)

Happy building with Paladin! 🏰
