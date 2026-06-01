# Commander Strategy Router

**Unified interface for intelligent Battalion orchestration with automatic strategy selection**

## Table of Contents

- [Overview](#overview)
- [Quick Start](#quick-start)
- [Strategy Selection](#strategy-selection)
  - [Auto Mode](#auto-mode)
  - [Explicit Strategy Selection](#explicit-strategy-selection)
- [Metadata Export](#metadata-export)
  - [Enabling Metadata Export](#enabling-metadata-export)
  - [JSON Structure](#json-structure)
  - [Use Cases](#use-cases)
- [Configuration](#configuration)
  - [BattalionConfig](#battalionconfig)
  - [Error Handling Strategies](#error-handling-strategies)
  - [Retry Policies](#retry-policies)
- [Telemetry & Monitoring](#telemetry--monitoring)
- [Best Practices](#best-practices)
- [Troubleshooting](#troubleshooting)

## Overview

The Commander is a high-level abstraction that simplifies Battalion usage by providing:

1. **Auto Mode**: Automatically selects the optimal orchestration strategy based on input analysis
2. **Unified API**: Single interface for all Battalion patterns (Formation, Phalanx, Campaign, ChainOfCommand, Maneuver)
3. **Simplified Configuration**: Smart defaults with comprehensive customization options
4. **Enhanced Telemetry**: Strategy selection reasoning, detailed timing, and metadata export

### When to Use Commander

- **Auto Mode**: When strategy may vary per request (e.g., user-driven workflows)
- **Explicit Mode**: When strategy is known and fixed (e.g., production pipelines)
- **Metadata Export**: When audit trails, cost tracking, or performance analysis needed

### Architecture

```
┌──────────────────────────────────────────────────────────────┐
│                      Commander                                 │
├──────────────────────────────────────────────────────────────┤
│  Strategy Selection Logic (Auto Mode)                        │
│    ↓                                                          │
│  ┌─────────┬─────────┬──────────┬───────────┬─────────┐    │
│  │Formation│ Phalanx │ Campaign │ChainOfCmd │ Maneuver│    │
│  └─────────┴─────────┴──────────┴───────────┴─────────┘    │
├──────────────────────────────────────────────────────────────┤
│  Telemetry & Metadata Collection                             │
│  - Execution times per Paladin                               │
│  - Token usage breakdown                                     │
│  - Strategy selection reasoning                              │
│  - Optional JSON export                                      │
└──────────────────────────────────────────────────────────────┘
```

## Quick Start

### Auto Mode (Recommended for Dynamic Workflows)

```rust
use paladin::application::services::battalion::commander::CommanderBuilder;
use paladin::core::platform::container::battalion::BattalionStrategy;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let paladin_port = Arc::new(/* your PaladinPort implementation */);

    let paladins = vec![
        create_paladin("Analyzer", "data analysis"),
        create_paladin("Processor", "data processing"),
        create_paladin("Synthesizer", "report generation"),
    ];

    // Commander automatically selects best strategy
    let commander = CommanderBuilder::new(paladin_port)
        .strategy(BattalionStrategy::Auto)
        .paladins(paladins)
        .build()?;

    let result = commander.execute("Analyze this data").await?;

    println!("Strategy Selected: {:?}", result.strategy_used);
    if let Some(reasoning) = &result.strategy_selection_reasoning {
        println!("Reasoning: {}", reasoning);
    }

    Ok(())
}
```

### Explicit Strategy (Recommended for Production Pipelines)

```rust
let commander = CommanderBuilder::new(paladin_port)
    .strategy(BattalionStrategy::Formation)  // Explicit strategy
    .paladins(pipeline_paladins)
    .build()?;

let result = commander.execute(input).await?;
```

## Strategy Selection

### Auto Mode

Commander analyzes input and Paladin configuration to select the optimal strategy.

#### Selection Logic

Commander evaluates multiple factors:

1. **Input Keyword Analysis**:
   - **Maneuver** (highest priority): "flow", "dynamic flow", "->", "," (DSL operators)
   - **Formation**: "sequential", "pipeline", "step by step", "one after", "first then"
   - **Phalanx**: "parallel", "concurrent", "all at once", "simultaneously"
   - **Campaign**: "workflow", "graph", "conditional", "if-then", "depends on"
   - **ChainOfCommand**: "delegate", "hierarchy", "specialist", "expert"

2. **Paladin Count Heuristics**:
   - **1-3 Paladins**: Formation (sequential) by default
   - **4+ Paladins**: Analyzes for parallelism indicators
   - **Many similar Paladins**: Prefers Phalanx (parallel execution)
   - **Mixed specialist Paladins**: Considers ChainOfCommand (delegation)

3. **Fallback Logic**:
   - If no clear indicators: **Formation** (safest default)
   - Selection typically completes in 0-5ms
   - Reasoning explanation included in result metadata

#### Example: Auto Mode with Analysis

```rust
let commander = CommanderBuilder::new(paladin_port)
    .strategy(BattalionStrategy::Auto)
    .paladins(vec![
        create_paladin("Worker1", "analysis"),
        create_paladin("Worker2", "analysis"),
        create_paladin("Worker3", "analysis"),
    ])
    .build()?;

// Input suggests parallel execution
let result = commander.execute("Process all items in parallel").await?;

assert_eq!(result.strategy_used, BattalionStrategy::Phalanx);
assert!(result.strategy_selection_reasoning.is_some());
println!("Selected: {:?} because {}",
    result.strategy_used,
    result.strategy_selection_reasoning.unwrap()
);
```

### Explicit Strategy Selection

When the orchestration pattern is known, use explicit strategy:

```rust
// Sequential processing pipeline
let commander = CommanderBuilder::new(paladin_port)
    .strategy(BattalionStrategy::Formation)
    .paladins(vec![analyzer, enhancer, reviewer])
    .build()?;

// Parallel batch processing
let commander = CommanderBuilder::new(paladin_port)
    .strategy(BattalionStrategy::Phalanx)
    .paladins(parallel_workers)
    .build()?;

// Conditional routing
let commander = CommanderBuilder::new(paladin_port)
    .strategy(BattalionStrategy::Campaign)
    .paladins(workflow_paladins)
    .build()?;
```

## Metadata Export

Commander can export comprehensive execution metadata to JSON files for audit trails, performance analysis, and cost tracking.

### Enabling Metadata Export

```rust
use std::path::PathBuf;
use paladin::core::platform::container::battalion::BattalionConfig;

let config = BattalionConfig::new("audited_battalion")
    .with_metadata_dir(PathBuf::from("./battalion_metadata"));

let commander = CommanderBuilder::new(paladin_port)
    .strategy(BattalionStrategy::Auto)
    .paladins(paladins)
    .config(config)
    .build()?;

let result = commander.execute(input).await?;

// Metadata automatically written to:
// ./battalion_metadata/{strategy}_{timestamp}_{uuid}.json
```

### File Naming Convention

Metadata files are named using a consistent pattern:

```
{strategy}_{timestamp}_{uuid}.json
```

**Components**:
- `strategy`: Battalion strategy executed (Formation, Phalanx, Campaign, etc.)
- `timestamp`: ISO 8601 format without separators (YYYYMMDD_HHMMSS)
- `uuid`: First 8 characters of the Battalion execution UUID

**Examples**:
```
Formation_20240315_143022_a1b2c3d4.json
Phalanx_20240315_150815_f5e6d7c8.json
Campaign_20240315_162341_9a8b7c6d.json
```

### JSON Structure

The metadata JSON file contains comprehensive execution information:

```json
{
  "battalion_id": "a1b2c3d4-e5f6-7890-abcd-ef1234567890",
  "battalion_name": "audited_battalion",
  "strategy_used": "Formation",
  "started_at": "2024-03-15T14:30:22.123456Z",
  "completed_at": "2024-03-15T14:31:15.789012Z",
  "duration_ms": 53666,
  "status": "Completed",
  "paladin_success_count": 3,
  "paladin_failure_count": 0,
  "total_tokens": 1520,
  "paladin_results": [
    {
      "paladin_name": "Analyzer",
      "status": "Success",
      "output": "Analysis complete: 15 insights identified",
      "execution_time_ms": 1500,
      "token_count": 450,
      "loop_count": 1,
      "stop_reason": "Completed"
    },
    {
      "paladin_name": "Enhancer",
      "status": "Success",
      "output": "Enhanced analysis with 8 recommendations",
      "execution_time_ms": 1800,
      "token_count": 620,
      "loop_count": 1,
      "stop_reason": "Completed"
    },
    {
      "paladin_name": "Reviewer",
      "status": "Success",
      "output": "Final review: High quality, approved",
      "execution_time_ms": 1200,
      "token_count": 450,
      "loop_count": 1,
      "stop_reason": "Completed"
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
    },
    "Enhancer": {
      "prompt_tokens": 220,
      "completion_tokens": 400,
      "total_tokens": 620
    },
    "Reviewer": {
      "prompt_tokens": 150,
      "completion_tokens": 300,
      "total_tokens": 450
    }
  },
  "strategy_selection_reasoning": "Input contains 'sequential' keyword",
  "strategy_selection_time_ms": 2,
  "final_output": "Complete analysis with recommendations and review",
  "errors": []
}
```

#### Field Reference

| Field | Type | Description |
|-------|------|-------------|
| `battalion_id` | UUID | Unique identifier for this execution |
| `battalion_name` | String | Configuration name from BattalionConfig |
| `strategy_used` | String | Actual strategy executed (may differ from requested in Auto mode) |
| `started_at` | ISO 8601 | Execution start timestamp with microsecond precision |
| `completed_at` | ISO 8601 | Execution completion timestamp |
| `duration_ms` | Integer | Total execution time in milliseconds |
| `status` | String | "Completed", "Failed", "PartialSuccess", "Timeout" |
| `paladin_success_count` | Integer | Number of Paladins that completed successfully |
| `paladin_failure_count` | Integer | Number of Paladins that failed |
| `total_tokens` | Integer | Sum of all token usage across all Paladins |
| `paladin_results` | Array | Detailed results for each Paladin execution |
| `per_paladin_times` | Object | Execution time (ms) per Paladin by name |
| `per_paladin_tokens` | Object | Token breakdown per Paladin (prompt, completion, total) |
| `strategy_selection_reasoning` | String\|null | Auto mode decision explanation (null for explicit strategies) |
| `strategy_selection_time_ms` | Integer | Overhead for strategy selection (0 for explicit strategies) |
| `final_output` | String | Aggregated or final output from Battalion execution |
| `errors` | Array | Error details if any Paladins failed |

### Use Cases

#### 1. Performance Analysis

```rust
let config = BattalionConfig::new("performance_profiling")
    .with_metadata_dir(PathBuf::from("./profiling_data"));

let result = commander.execute(input).await?;

// Analyze metadata to identify bottlenecks
// Find slow Paladins: Check per_paladin_times
// Optimize token usage: Review per_paladin_tokens
```

#### 2. Cost Tracking

```rust
let config = BattalionConfig::new("cost_tracking")
    .with_metadata_dir(PathBuf::from("./billing_data"));

// Parse metadata files to calculate costs
// Cost = total_tokens * model_cost_per_token
// Per-Paladin cost breakdown available
```

#### 3. Audit Trails & Compliance

```rust
let config = BattalionConfig::new("production_api_handler")
    .with_metadata_dir(PathBuf::from("/var/log/battalion"));

// Every execution fully documented
// Tamper-evident JSON with timestamps
// Track who executed what and when
```

#### 4. Debugging & Troubleshooting

```rust
let config = BattalionConfig::new("debug_session")
    .with_metadata_dir(PathBuf::from("./debug_logs"));

// Capture execution state before failures
// Per-Paladin outputs for inspection
// Strategy selection reasoning for unexpected results
```

### Configuration via YAML

```yaml
# config.yml
battalion:
  metadata_output_dir: "./battalion_metadata"
  default_timeout: 300
  error_strategy: "RetryThenContinue"
```

```rust
use config::Config;

let settings = Config::builder()
    .add_source(config::File::with_name("config.yml"))
    .build()?;

let metadata_dir = settings.get_string("battalion.metadata_output_dir")?;
let config = BattalionConfig::new("from_config")
    .with_metadata_dir(PathBuf::from(metadata_dir));
```

### Performance Impact

- **File I/O**: Asynchronous, non-blocking
- **Overhead**: <1ms for typical payloads
- **Disk Usage**: ~1-5KB per execution (depends on Paladin count and output size)
- **Production Ready**: Zero performance impact on critical path

## Configuration

### BattalionConfig

Comprehensive configuration for Commander behavior:

```rust
use paladin::core::platform::container::battalion::{
    BattalionConfig, ErrorStrategy, RetryPolicy
};
use std::path::PathBuf;

let config = BattalionConfig::new("my_battalion")
    .with_description("Processes critical data pipeline")
    .with_timeout(300)  // 5 minutes
    .with_error_strategy(ErrorStrategy::RetryThenContinue)
    .with_retry_policy(RetryPolicy {
        max_attempts: 3,
        initial_delay_ms: 1000,
        max_delay_ms: 30000,
        backoff_multiplier: 2.0,
    })
    .with_metadata_dir(PathBuf::from("./checkpoints"));
```

#### Configuration Fields

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `name` | String | "default_commander_battalion" | Battalion identifier |
| `description` | Option<String> | None | Human-readable description |
| `timeout_seconds` | u64 | 300 | Maximum execution time |
| `error_strategy` | ErrorStrategy | FailFast | How to handle Paladin failures |
| `retry_policy` | RetryPolicy | 3 attempts | Retry configuration |
| `metadata_output_dir` | Option<PathBuf> | None | Directory for metadata JSON export |

### Error Handling Strategies

#### FailFast (Default)

Stops execution immediately on first Paladin failure.

```rust
let config = BattalionConfig::new("fail_fast")
    .with_error_strategy(ErrorStrategy::FailFast);
```

**When to Use**:
- All Paladins must succeed for valid result
- Failures indicate fundamental issues (bad input, configuration errors)
- Want fast failure feedback for debugging

#### ContinueOnError

Continues executing remaining Paladins despite failures.

```rust
let config = BattalionConfig::new("continue_on_error")
    .with_error_strategy(ErrorStrategy::ContinueOnError);
```

**When to Use**:
- Partial results are valuable (e.g., batch processing)
- Independent tasks where some failures acceptable
- Need complete execution report for analysis

#### RetryThenContinue (Recommended for Production)

Retries failed Paladins up to `max_attempts`, then continues with remaining Paladins.

```rust
let config = BattalionConfig::new("production")
    .with_error_strategy(ErrorStrategy::RetryThenContinue)
    .with_retry_policy(RetryPolicy {
        max_attempts: 3,
        initial_delay_ms: 1000,
        max_delay_ms: 30000,
        backoff_multiplier: 2.0,
    });
```

**When to Use**:
- Transient failures possible (network issues, rate limits, temporary unavailability)
- Production environments requiring resilience
- Want to maximize success rate without blocking entire workflow

### Retry Policies

```rust
pub struct RetryPolicy {
    pub max_attempts: u32,        // Total attempts (including initial)
    pub initial_delay_ms: u64,    // First retry delay
    pub max_delay_ms: u64,        // Cap on delay
    pub backoff_multiplier: f64,  // Exponential backoff factor
}
```

**Default Retry Policy**:
```rust
RetryPolicy {
    max_attempts: 3,          // 3 total attempts
    initial_delay_ms: 1000,   // 1 second first retry
    max_delay_ms: 30000,      // 30 second cap
    backoff_multiplier: 2.0,  // Double delay each retry
}
```

**Retry Timing Example**:
- Attempt 1: Immediate
- Attempt 2: After 1 second
- Attempt 3: After 2 seconds
- If max_attempts = 4, Attempt 4: After 4 seconds

## Telemetry & Monitoring

### BattalionResult Telemetry

```rust
pub struct BattalionResult {
    pub battalion_id: Uuid,
    pub battalion_name: String,
    pub started_at: DateTime<Utc>,
    pub completed_at: DateTime<Utc>,
    pub status: BattalionStatus,
    pub strategy_used: BattalionStrategy,
    pub strategy_selection_reasoning: Option<String>,
    pub strategy_selection_time_ms: u64,
    pub final_output: String,
    pub paladin_success_count: usize,
    pub paladin_failure_count: usize,
    pub total_tokens: usize,
    pub per_paladin_times: HashMap<String, u64>,
    pub per_paladin_tokens: HashMap<String, TokenUsage>,
    // ... additional fields
}
```

### Monitoring Examples

#### Execution Duration

```rust
let result = commander.execute(input).await?;

let duration = result.completed_at
    .signed_duration_since(result.started_at)
    .num_milliseconds();

println!("Execution time: {}ms", duration);
```

#### Success Rate

```rust
let success_rate = result.paladin_success_count as f64
    / (result.paladin_success_count + result.paladin_failure_count) as f64
    * 100.0;

println!("Success rate: {:.1}%", success_rate);
```

#### Per-Paladin Metrics

```rust
for (name, time_ms) in &result.per_paladin_times {
    let tokens = result.per_paladin_tokens
        .get(name)
        .map(|t| t.total_tokens)
        .unwrap_or(0);

    println!("{}: {}ms, {} tokens", name, time_ms, tokens);
}
```

#### Integration with Metrics Systems

```rust
// Prometheus-style metrics
metrics.record_battalion_duration(
    result.battalion_name.as_str(),
    duration as f64
);

metrics.record_strategy_selection(
    result.strategy_used,
    result.strategy_selection_time_ms
);

metrics.record_paladin_counts(
    result.paladin_success_count,
    result.paladin_failure_count
);
```

## Best Practices

### 1. Use Auto Mode for User-Driven Workflows

```rust
// Good: Flexibility for unpredictable inputs
let commander = CommanderBuilder::new(paladin_port)
    .strategy(BattalionStrategy::Auto)
    .paladins(general_purpose_paladins)
    .build()?;
```

### 2. Use Explicit Strategies for Production Pipelines

```rust
// Good: Predictability and performance
let commander = CommanderBuilder::new(paladin_port)
    .strategy(BattalionStrategy::Formation)  // Known pattern
    .paladins(pipeline_paladins)
    .build()?;
```

### 3. Configure Appropriate Timeouts

```rust
// Good: Realistic timeout with buffer
let config = BattalionConfig::new("batch_processing")
    .with_timeout(600);  // 10 minutes for batch job
```

**Consider**:
- LLM response times (typically 1-30 seconds per request)
- Number of Paladins and strategy (sequential vs. parallel)
- Network latency and retries
- Add 20-30% buffer for safety

### 4. Use RetryThenContinue in Production

```rust
// Best practice for production
let config = BattalionConfig::new("production")
    .with_error_strategy(ErrorStrategy::RetryThenContinue)
    .with_retry_policy(RetryPolicy {
        max_attempts: 3,
        initial_delay_ms: 1000,
        max_delay_ms: 30000,
        backoff_multiplier: 2.0,
    });
```

### 5. Enable Metadata Export for Critical Systems

```rust
// Good: Audit trail for compliance
let config = BattalionConfig::new("critical_system")
    .with_metadata_dir(PathBuf::from("/var/log/battalion"));
```

### 6. Monitor Telemetry Regularly

```rust
let result = commander.execute(input).await?;

// Log key metrics
log::info!(
    "Battalion {} completed in {}ms ({} success, {} failed)",
    result.battalion_name,
    result.completed_at.signed_duration_since(result.started_at).num_milliseconds(),
    result.paladin_success_count,
    result.paladin_failure_count
);
```

### 7. Handle Errors Gracefully

```rust
match commander.execute(input).await {
    Ok(result) => {
        if result.paladin_failure_count > 0 {
            log::warn!(
                "Completed with {} failures",
                result.paladin_failure_count
            );
        }
        process_result(result);
    }
    Err(e) => {
        log::error!("Battalion execution failed: {}", e);
        handle_failure(e);
    }
}
```

## Troubleshooting

### Issue: Strategy Selection Takes Too Long

**Symptoms**: High `strategy_selection_time_ms` (>10ms)

**Solutions**:
1. Use explicit strategy instead of Auto mode
2. Simplify input (avoid very long strings in keyword analysis)
3. Consider caching strategy decisions for similar inputs

```rust
// If Auto mode adds too much overhead:
let commander = CommanderBuilder::new(paladin_port)
    .strategy(BattalionStrategy::Formation)  // Explicit, 0ms overhead
    .paladins(paladins)
    .build()?;
```

### Issue: Metadata Files Not Created

**Possible Causes**:
1. Directory doesn't exist or lacks write permissions
2. `metadata_output_dir` not set in configuration
3. Execution failed before metadata write

**Solutions**:

```rust
use std::fs;

// Ensure directory exists with correct permissions
let metadata_dir = PathBuf::from("./battalion_metadata");
fs::create_dir_all(&metadata_dir)?;

let config = BattalionConfig::new("battalion")
    .with_metadata_dir(metadata_dir);

// Verify after execution
let result = commander.execute(input).await?;
println!("Battalion ID: {}", result.battalion_id);
// Look for: {strategy}_{timestamp}_{first_8_chars_of_uuid}.json
```

### Issue: Unexpected Strategy Selected

**Symptoms**: Auto mode selects different strategy than expected

**Diagnosis**:

```rust
let result = commander.execute(input).await?;

println!("Expected: X, Got: {:?}", result.strategy_used);
if let Some(reasoning) = &result.strategy_selection_reasoning {
    println!("Reasoning: {}", reasoning);
}
```

**Solutions**:
1. Review input for keyword conflicts
2. Use explicit strategy if behavior must be deterministic
3. Check Paladin count (affects heuristics)

### Issue: High Token Usage

**Symptoms**: `total_tokens` higher than expected

**Diagnosis**:

```rust
let result = commander.execute(input).await?;

println!("Total tokens: {}", result.total_tokens);
for (name, tokens) in &result.per_paladin_tokens {
    println!("  {}: {} tokens", name, tokens.total_tokens);
}

// Check for surprisingly high token usage
let max_tokens = result.per_paladin_tokens.values()
    .map(|t| t.total_tokens)
    .max()
    .unwrap_or(0);

if max_tokens > expected_threshold {
    println!("WARNING: High token usage detected");
}
```

**Solutions**:
1. Optimize Paladin system prompts (reduce verbosity)
2. Trim input context before passing to Paladins
3. Use smaller models for simple tasks
4. Consider token limits in Paladin configuration

### Issue: Timeouts

**Symptoms**: BattalionStatus::Timeout in result

**Diagnosis**:

```rust
let result = commander.execute(input).await;

if let Ok(r) = result {
    if r.status == BattalionStatus::Timeout {
        println!("Timeout after {}s", config.timeout_seconds);

        // Check which Paladins completed
        println!("Completed: {}", r.paladin_success_count);
        println!("Failed: {}", r.paladin_failure_count);
    }
}
```

**Solutions**:
1. Increase timeout appropriately
2. Check per-Paladin execution times for bottlenecks
3. Consider using Phalanx (parallel) instead of Formation (sequential)
4. Optimize slow Paladins

```rust
// Increase timeout
let config = BattalionConfig::new("battalion")
    .with_timeout(600);  // 10 minutes instead of 5

// Or switch to parallel execution
let commander = CommanderBuilder::new(paladin_port)
    .strategy(BattalionStrategy::Phalanx)  // Parallel = faster
    .paladins(paladins)
    .build()?;
```

### Issue: Partial Failures

**Symptoms**: `paladin_failure_count > 0` but execution completes

**This is expected behavior with**:
- `ErrorStrategy::ContinueOnError`
- `ErrorStrategy::RetryThenContinue` (after retries exhausted)

**Handling**:

```rust
let result = commander.execute(input).await?;

if result.paladin_failure_count > 0 {
    log::warn!(
        "Partial success: {} of {} Paladins failed",
        result.paladin_failure_count,
        result.paladin_success_count + result.paladin_failure_count
    );

    // Check metadata for detailed error information
    if let Some(metadata_dir) = config.metadata_output_dir {
        println!("See metadata in: {}", metadata_dir.display());
    }
}
```

## See Also

- [Battalion Documentation](BATTALION.md) - Detailed orchestration pattern documentation
- [Paladin](../README.md#paladin) - Individual agent configuration
- [Configuration Guide](../README.md#configuration) - System-wide configuration
- [Examples](../examples/) - Runnable code examples

---

**Version**: 0.1.0  
**Last Updated**: 2024-03-15
