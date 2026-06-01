# Paladin Configuration Guide

This guide covers how to configure Paladin agents for optimal performance, from basic setup to advanced tuning.

## Table of Contents

- [Basic Configuration](#basic-configuration)
- [System Prompt Best Practices](#system-prompt-best-practices)
- [Model Selection](#model-selection)
- [Temperature and Sampling](#temperature-and-sampling)
- [Stop Words and Termination](#stop-words-and-termination)
- [Timeout and Retry Settings](#timeout-and-retry-settings)
- [Advanced Configuration](#advanced-configuration)

## Basic Configuration

### Minimal Setup

```rust
use paladin::prelude::*;

let paladin = PaladinBuilder::new(llm_adapter)
    .name("Assistant")
    .system_prompt("You are a helpful assistant.")
    .build()?;
```

### Common Configuration

```rust
let paladin = PaladinBuilder::new(llm_adapter)
    .name("DataAnalyst")
    .system_prompt("You are an expert data analyst. Provide clear, data-driven insights.")
    .model("gpt-4")
    .temperature(0.7)
    .max_loops(5)
    .timeout(Duration::from_secs(120))
    .build()?;
```

### Full Configuration

```rust
let paladin = PaladinBuilder::new(llm_adapter)
    .name("ResearchAssistant")
    .system_prompt("You are a research assistant specializing in academic papers.")
    .user_name("Researcher")
    .model("gpt-4-turbo")
    .temperature(0.8)
    .max_loops(10)
    .stop_words(vec!["END", "STOP", "FINAL_ANSWER"])
    .timeout(Duration::from_secs(300))
    .retry_attempts(3)
    .retry_delay(Duration::from_secs(5))
    .with_garrison(garrison)
    .add_armament(search_tool)
    .add_armament(calculator_tool)
    .build()?;
```

## System Prompt Best Practices

The system prompt defines your Paladin's behavior and capabilities. Follow these best practices:

### 1. Be Specific About Role

**❌ Vague:**
```rust
.system_prompt("You are helpful.")
```

**✅ Specific:**
```rust
.system_prompt("You are a senior software engineer specializing in Rust. \
                You provide code reviews focused on safety, performance, and idiomatic patterns.")
```

### 2. Define Output Format

```rust
.system_prompt("You are a JSON API. Always respond with valid JSON. \
                Structure: {\"status\": \"success|error\", \"data\": {...}, \"message\": \"...\"}  \
                Never include markdown code blocks or explanations outside the JSON.")
```

### 3. Set Boundaries

```rust
.system_prompt("You are a customer support agent for TechCorp. \
                - Only answer questions about our products and services \
                - Escalate billing questions to the finance team \
                - Do not provide medical, legal, or financial advice \
                - Be polite and professional at all times")
```

### 4. Include Examples (Few-Shot)

```rust
.system_prompt("You categorize customer feedback as: FEATURE_REQUEST, BUG_REPORT, or PRAISE. \
                \
                Examples: \
                Input: 'The app crashes when I upload large files' \
                Output: BUG_REPORT \
                \
                Input: 'It would be great to have dark mode' \
                Output: FEATURE_REQUEST \
                \
                Input: 'Love the new design!' \
                Output: PRAISE")
```

### 5. Specify Tone and Style

```rust
.system_prompt("You are a technical writer creating documentation for developers. \
                - Use clear, concise language \
                - Prefer active voice \
                - Include code examples \
                - Target audience: junior to mid-level developers \
                - Avoid jargon unless necessary")
```

## Model Selection

Choose the right model for your use case:

### OpenAI Models

```rust
// GPT-4 Turbo - Best for complex reasoning
.model("gpt-4-turbo")  // Latest turbo model
.model("gpt-4")        // Standard GPT-4

// GPT-3.5 - Fast and cost-effective
.model("gpt-3.5-turbo")  // Recommended for most tasks
```

**When to use:**
- **GPT-4**: Complex reasoning, code generation, detailed analysis
- **GPT-3.5**: Simple queries, classification, summarization

### DeepSeek Models

```rust
// DeepSeek Chat - Strong coding capabilities
.model("deepseek-chat")

// DeepSeek Coder - Specialized for code
.model("deepseek-coder")
```

**When to use:**
- **deepseek-chat**: General purpose, good for multi-turn conversations
- **deepseek-coder**: Code generation, technical documentation

### Anthropic Models

```rust
// Claude 3 Family
.model("claude-3-opus")    // Most capable
.model("claude-3-sonnet")  // Balanced
.model("claude-3-haiku")   // Fastest
```

**When to use:**
- **Opus**: Complex analysis, long documents, creative writing
- **Sonnet**: General purpose, good balance of speed and quality
- **Haiku**: Fast responses, simple queries, high throughput

### Model Comparison

| Model | Speed | Cost | Quality | Max Tokens | Best For |
|-------|-------|------|---------|------------|----------|
| GPT-4 Turbo | Medium | High | Excellent | 128K | Complex reasoning |
| GPT-3.5 Turbo | Fast | Low | Good | 16K | Simple tasks |
| Claude 3 Opus | Medium | High | Excellent | 200K | Long documents |
| Claude 3 Sonnet | Fast | Medium | Very Good | 200K | General purpose |
| Claude 3 Haiku | Very Fast | Low | Good | 200K | High throughput |
| DeepSeek Chat | Fast | Very Low | Good | 64K | Cost-sensitive |
| DeepSeek Coder | Fast | Very Low | Very Good | 64K | Code generation |

## Temperature and Sampling

Temperature controls randomness in responses:

### Temperature Scale

```rust
// 0.0 - Deterministic, focused (best for factual tasks)
.temperature(0.0)

// 0.3-0.5 - Slightly varied (good for classification)
.temperature(0.4)

// 0.7 - Balanced (general purpose)
.temperature(0.7)

// 0.9-1.0 - Creative, diverse (brainstorming, creative writing)
.temperature(0.9)

// >1.0 - Very random (experimental, not recommended)
.temperature(1.2)
```

### Use Cases by Temperature

| Temperature | Use Case | Example |
|-------------|----------|---------|
| 0.0 - 0.3 | Factual, deterministic | Math, code review, data extraction |
| 0.4 - 0.6 | Balanced, consistent | Customer support, Q&A, summarization |
| 0.7 - 0.8 | Creative, natural | Content generation, conversation |
| 0.9 - 1.0 | Highly creative | Brainstorming, storytelling, poetry |

### Example: Task-Specific Configuration

```rust
// Code Review - Deterministic
let code_reviewer = PaladinBuilder::new(llm_adapter)
    .system_prompt("Review Rust code for safety and best practices.")
    .temperature(0.2)
    .build()?;

// Content Writer - Creative
let writer = PaladinBuilder::new(llm_adapter)
    .system_prompt("Write engaging blog posts about technology.")
    .temperature(0.9)
    .build()?;

// Customer Support - Balanced
let support = PaladinBuilder::new(llm_adapter)
    .system_prompt("Help customers with product questions.")
    .temperature(0.7)
    .build()?;
```

## Stop Words and Termination

Control when a Paladin stops generating:

### Basic Stop Words

```rust
let paladin = PaladinBuilder::new(llm_adapter)
    .stop_words(vec!["END", "STOP", "###"])
    .build()?;
```

### Use Cases

#### 1. Structured Output

```rust
// Stop at delimiter for parsing
.system_prompt("Generate a list of items. End with '---'")
.stop_words(vec!["---"])
```

#### 2. Multi-Step Reasoning

```rust
// Stop when final answer is reached
.system_prompt("Think step by step. When done, output FINAL_ANSWER: <answer>")
.stop_words(vec!["FINAL_ANSWER:"])
```

#### 3. Dialog Systems

```rust
// Stop at turn boundaries
.system_prompt("You are user A in a conversation. End each turn with [END_TURN]")
.stop_words(vec!["[END_TURN]"])
```

### Max Loops

Prevent infinite reasoning loops:

```rust
// Default: 3 loops
.max_loops(3)

// For simple tasks: 1 loop
.max_loops(1)

// For complex reasoning: 10+ loops
.max_loops(15)
```

**What is a loop?**
A loop is one reasoning cycle: prompt → LLM → response → (optional tool calls) → repeat.

## Timeout and Retry Settings

### Timeout Configuration

```rust
use std::time::Duration;

let paladin = PaladinBuilder::new(llm_adapter)
    .timeout(Duration::from_secs(60))  // 60 second timeout
    .build()?;
```

**Recommended Timeouts:**
- Simple queries: 30 seconds
- Complex reasoning: 120 seconds
- With tool calls: 300 seconds

### Retry Configuration

```rust
let paladin = PaladinBuilder::new(llm_adapter)
    .retry_attempts(3)                        // Retry up to 3 times
    .retry_delay(Duration::from_secs(5))      // Wait 5 seconds between retries
    .build()?;
```

### Error Handling

```rust
match paladin.execute(input).await {
    Ok(response) => println!("Success: {}", response.content),
    Err(PaladinError::Timeout(secs)) => {
        eprintln!("Request timed out after {} seconds", secs);
        // Increase timeout or simplify prompt
    }
    Err(PaladinError::LlmError(msg)) => {
        eprintln!("LLM error: {}", msg);
        // Check API key, rate limits, model availability
    }
    Err(PaladinError::MaxLoopsExceeded) => {
        eprintln!("Max reasoning loops exceeded");
        // Increase max_loops or refine system prompt
    }
    Err(e) => eprintln!("Other error: {}", e),
}
```

## Advanced Configuration

### Configuration from File

```rust
use paladin::config::ApplicationSettings;

let config = ApplicationSettings::load_from("config.yml")?;
let paladin = PaladinBuilder::from_config(&config.paladin)?;
```

`config.yml`:
```yaml
paladin:
  name: "Assistant"
  system_prompt: "You are a helpful assistant."
  model: "gpt-4"
  temperature: 0.7
  max_loops: 5
  timeout_seconds: 120
  retry_attempts: 3
  stop_words:
    - "END"
    - "STOP"
```

### Environment-Based Configuration

```rust
let model = std::env::var("PALADIN_MODEL").unwrap_or("gpt-3.5-turbo".to_string());
let temperature = std::env::var("PALADIN_TEMPERATURE")
    .ok()
    .and_then(|s| s.parse::<f32>().ok())
    .unwrap_or(0.7);

let paladin = PaladinBuilder::new(llm_adapter)
    .model(&model)
    .temperature(temperature)
    .build()?;
```

### Dynamic Configuration

```rust
struct PaladinFactory;

impl PaladinFactory {
    fn create_for_task(task_type: &str, llm_adapter: Arc<dyn LlmPort>) -> Result<Paladin> {
        match task_type {
            "code_review" => Self::create_code_reviewer(llm_adapter),
            "creative_writing" => Self::create_writer(llm_adapter),
            "data_analysis" => Self::create_analyst(llm_adapter),
            _ => Self::create_default(llm_adapter),
        }
    }

    fn create_code_reviewer(llm_adapter: Arc<dyn LlmPort>) -> Result<Paladin> {
        PaladinBuilder::new(llm_adapter)
            .system_prompt("Expert Rust code reviewer")
            .temperature(0.2)
            .model("gpt-4")
            .build()
    }

    // ... other factory methods
}
```

### Configuration Validation

```rust
let paladin = PaladinBuilder::new(llm_adapter)
    .temperature(0.7)
    .build()?;  // Validates configuration

// Manual validation
if let Err(e) = paladin.validate() {
    eprintln!("Invalid configuration: {}", e);
}
```

## Configuration Checklist

Before deploying a Paladin, verify:

- [ ] System prompt is clear and specific
- [ ] Appropriate model selected for task
- [ ] Temperature suitable for use case (0.2 for factual, 0.9 for creative)
- [ ] Max loops set appropriately (1-3 for simple, 10+ for complex)
- [ ] Timeout configured (30-300 seconds)
- [ ] Retry logic in place for production
- [ ] Stop words defined if needed
- [ ] Error handling implemented
- [ ] Configuration tested with sample inputs

## Performance Tuning

### For Throughput

```rust
// Fast model, simple prompts
let paladin = PaladinBuilder::new(llm_adapter)
    .model("gpt-3.5-turbo")
    .temperature(0.7)
    .max_loops(1)
    .timeout(Duration::from_secs(30))
    .build()?;
```

### For Quality

```rust
// Best model, detailed prompts
let paladin = PaladinBuilder::new(llm_adapter)
    .model("gpt-4")
    .temperature(0.5)
    .max_loops(10)
    .timeout(Duration::from_secs(300))
    .build()?;
```

### For Cost Efficiency

```rust
// Cheaper model, efficient prompts
let paladin = PaladinBuilder::new(llm_adapter)
    .model("deepseek-chat")
    .temperature(0.7)
    .max_loops(3)
    .build()?;
```

## Next Steps

- **[Battalion Patterns](battalion-patterns.md)** - Multi-agent orchestration
- **[Tool Integration](tool-integration.md)** - Add capabilities with Arsenal
- **[Memory Management](memory-management.md)** - Use Garrison for context
- **[Examples](../../examples/)** - See configuration in action

## Related Documentation

- [Quickstart Guide](../QUICKSTART.md)
- [API Reference](https://docs.rs/paladin)
- [Performance Tuning](../operations/performance-tuning.md)
