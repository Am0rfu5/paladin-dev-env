# Logging Configuration

Complete guide for configuring and managing logs in Paladin using the `tracing` ecosystem.

## Table of Contents

- [Overview](#overview)
- [Configuration](#configuration)
- [Log Levels](#log-levels)
- [Structured Logging](#structured-logging)
- [Log Aggregation](#log-aggregation)
- [Log Analysis](#log-analysis)
- [Best Practices](#best-practices)

## Overview

Paladin uses the Rust `tracing` crate for structured, async-aware logging with:
- **Structured fields**: JSON-formatted logs
- **Async tracing**: Spans across async boundaries
- **Multiple outputs**: Console, file, and external systems
- **Dynamic filtering**: Runtime log level adjustment

## Configuration

### Environment Variables

```bash
# Set log level
export RUST_LOG=info,paladin=debug

# Detailed format
export RUST_LOG_FORMAT=json

# Enable specific modules
export RUST_LOG=paladin::core=debug,paladin::infrastructure=info
```

### config.yml

```yaml
logging:
  # Global log level
  level: "info"

  # Format: json, pretty, compact
  format: "json"

  # Outputs
  outputs:
    - type: "stdout"
      level: "info"

    - type: "file"
      path: "/app/logs/paladin.log"
      level: "debug"
      rotation:
        max_size: "100MB"
        max_age: "7d"
        max_backups: 10

    - type: "loki"
      url: "http://loki:3100"
      labels:
        app: "paladin"
        environment: "production"

  # Module-specific levels
  modules:
    paladin::core: "debug"
    paladin::infrastructure::adapters: "info"
    paladin::application: "debug"

  # Sampling (for high-volume logs)
  sampling:
    enabled: true
    rate: 0.1  # Log 10% of debug messages
```

## Log Levels

### Level Hierarchy

```
ERROR < WARN < INFO < DEBUG < TRACE
  1      2      3      4       5
```

### Usage Guidelines

| Level | Usage | Example |
|-------|-------|---------|
| **ERROR** | Critical errors requiring immediate attention | Database connection failed, LLM API error |
| **WARN** | Concerning events that don't prevent operation | High latency, rate limit approaching |
| **INFO** | Normal operational messages | Paladin started, request completed |
| **DEBUG** | Detailed diagnostic information | Configuration loaded, intermediate steps |
| **TRACE** | Very verbose, low-level details | Function entry/exit, loop iterations |

### Code Examples

```rust,ignore
use tracing::{error, warn, info, debug, trace};

// ERROR: Critical failures
error!(error = %e, "Failed to connect to LLM provider");

// WARN: Concerning but recoverable
warn!(
    loops_used = paladin.max_loops,
    "Paladin reached max loop limit"
);

// INFO: Normal operations
info!(
    paladin_id = %paladin.id,
    duration_ms = elapsed.as_millis(),
    "Paladin execution completed"
);

// DEBUG: Detailed diagnostics
debug!(
    garrison_entries = garrison.len(),
    max_tokens = garrison.max_tokens,
    "Garrison state after adding entry"
);

// TRACE: Very detailed
trace!("Entering formation execution loop iteration {}", i);
```

## Structured Logging

### Field-Based Logging

```rust,ignore
use tracing::{info, instrument};

#[instrument(
    skip(paladin),
    fields(
        paladin_id = %paladin.id,
        paladin_name = %paladin.data.name,
        model = %paladin.data.model
    )
)]
async fn execute_paladin(paladin: &Paladin, input: &str) -> Result<PaladinResult> {
    info!(input_length = input.len(), "Starting execution");

    let result = paladin.execute(input).await?;

    info!(
        loops_used = result.loops_used,
        output_length = result.content.len(),
        success = true,
        "Execution completed"
    );

    Ok(result)
}
```

### Spans for Context

```rust,ignore
use tracing::info_span;

async fn battalion_execute(battalion: &Battalion, input: &str) -> Result<BattalionResult> {
    let span = info_span!(
        "battalion_execution",
        battalion_id = %battalion.id,
        battalion_type = ?battalion.pattern,
        paladin_count = battalion.paladins.len()
    );

    async {
        info!("Starting battalion execution");

        for (i, paladin) in battalion.paladins.iter().enumerate() {
            let paladin_span = info_span!(
                "paladin_execution",
                paladin_index = i,
                paladin_id = %paladin.id
            );

            paladin_span.in_scope(|| {
                info!("Executing paladin");
            });
        }

        Ok(result)
    }.instrument(span).await
}
```

### Error Logging

```rust,ignore
use tracing::error;
use anyhow::Context;

match llm_port.generate(model, messages, temperature).await {
    Ok(response) => response,
    Err(e) => {
        error!(
            error = %e,
            error_chain = ?e.chain().collect::<Vec<_>>(),
            model = model,
            temperature = temperature,
            "LLM generation failed"
        );
        return Err(e).context("Failed to generate LLM response");
    }
}
```

## Log Aggregation

### Loki Integration

```rust,ignore
// Cargo.toml
[dependencies]
tracing-loki = "0.2"

// src/infrastructure/logging/loki.rs
use tracing_loki::Layer as LokiLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

pub fn init_loki_logging(url: &str) -> Result<()> {
    let (loki_layer, task) = LokiLayer::new(
        url.parse()?,
        vec![
            ("app".to_string(), "paladin".to_string()),
            ("environment".to_string(), std::env::var("ENVIRONMENT")?),
        ],
    )?;

    tracing_subscriber::registry()
        .with(loki_layer)
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Spawn background task for Loki
    tokio::spawn(task);

    Ok(())
}
```

### Elasticsearch/OpenSearch

```rust,ignore
use tracing_elastic::Elastic;

pub fn init_elastic_logging(url: &str, index: &str) -> Result<()> {
    let elastic_layer = Elastic::new(url, index)?;

    tracing_subscriber::registry()
        .with(elastic_layer)
        .with(tracing_subscriber::fmt::layer())
        .init();

    Ok(())
}
```

### Fluentd/Fluent Bit

```ini
# fluent-bit.conf
[SERVICE]
    Flush        5
    Daemon       Off
    Log_Level    info

[INPUT]
    Name             tail
    Path             /app/logs/paladin.log
    Parser           json
    Tag              paladin.*
    Refresh_Interval 5

[FILTER]
    Name    modify
    Match   paladin.*
    Add     app paladin
    Add     environment production

[OUTPUT]
    Name  es
    Match *
    Host  elasticsearch
    Port  9200
    Index paladin
    Type  _doc
```

## Log Analysis

### Common Log Queries

#### Loki (LogQL)

```logql
# All errors in last hour
{app="paladin"} |= "ERROR" | json

# High latency requests
{app="paladin"} | json | duration_ms > 2000

# Specific paladin
{app="paladin"} | json | paladin_id="abc-123"

# Error rate
rate({app="paladin"} |= "ERROR"[5m])

# Top error messages
topk(10, count_over_time({app="paladin"} |= "ERROR" [1h]))
```

#### Elasticsearch (Lucene)

```json
# Errors in production
{
  "query": {
    "bool": {
      "must": [
        { "term": { "level": "ERROR" }},
        { "term": { "environment": "production" }}
      ],
      "filter": {
        "range": {
          "@timestamp": {
            "gte": "now-1h"
          }
        }
      }
    }
  }
}

# Slow requests
{
  "query": {
    "range": {
      "duration_ms": {
        "gte": 2000
      }
    }
  }
}
```

### Log Dashboards

#### Grafana Dashboard (JSON)

```json
{
  "dashboard": {
    "title": "Paladin Logs",
    "panels": [
      {
        "title": "Error Rate",
        "targets": [
          {
            "expr": "rate({app=\"paladin\"} |= \"ERROR\"[5m])",
            "legendFormat": "Errors/sec"
          }
        ]
      },
      {
        "title": "Log Volume by Level",
        "targets": [
          {
            "expr": "sum by (level) (rate({app=\"paladin\"}[5m]))"
          }
        ]
      },
      {
        "title": "Recent Errors",
        "targets": [
          {
            "expr": "{app=\"paladin\"} |= \"ERROR\"",
            "maxLines": 100
          }
        ]
      }
    ]
  }
}
```

## Best Practices

### 1. Consistent Field Names

```rust,ignore
// ✅ Good: Consistent naming
info!(paladin_id = %id, "Starting");
info!(paladin_id = %id, "Completed");

// ❌ Bad: Inconsistent
info!(paladin = %id, "Starting");
info!(id = %id, "Completed");
```

### 2. Structured Over String Interpolation

```rust,ignore
// ✅ Good: Structured fields
info!(
    paladin_id = %paladin.id,
    duration_ms = elapsed.as_millis(),
    success = true,
    "Execution completed"
);

// ❌ Bad: String interpolation
info!("Execution completed for paladin {} in {}ms: success",
    paladin.id, elapsed.as_millis());
```

### 3. Sensitive Data Redaction

```rust,ignore
// ✅ Good: Redact sensitive data
info!(
    api_key = "***REDACTED***",
    endpoint = url,
    "Making API call"
);

// ❌ Bad: Logging secrets
info!(api_key = api_key, "Making API call");
```

### 4. Appropriate Log Levels

```rust,ignore
// ✅ Good: INFO for normal operations
info!("Paladin execution started");

// ❌ Bad: DEBUG for normal operations
debug!("Paladin execution started");
```

### 5. Error Context

```rust,ignore
// ✅ Good: Full error context
error!(
    error = %e,
    paladin_id = %paladin.id,
    input_length = input.len(),
    "Paladin execution failed"
);

// ❌ Bad: Minimal context
error!("Error: {}", e);
```

### 6. Performance Considerations

```rust,ignore
// ✅ Good: Conditional expensive operations
if tracing::enabled!(tracing::Level::DEBUG) {
    let expensive_debug_info = compute_debug_info();
    debug!(info = ?expensive_debug_info, "Debug information");
}

// ❌ Bad: Always compute
let expensive_debug_info = compute_debug_info();
debug!(info = ?expensive_debug_info, "Debug information");
```

### 7. Log Rotation

```toml
# Cargo.toml
[dependencies]
tracing-appender = "0.2"

# src/main.rs
use tracing_appender::rolling::{RollingFileAppender, Rotation};

let file_appender = RollingFileAppender::new(
    Rotation::DAILY,
    "/app/logs",
    "paladin.log"
);
```

### 8. Production Log Level

```yaml
# Production: Reduce log volume
logging:
  level: "warn"  # Only warnings and errors

  # Enable debug for specific modules
  modules:
    paladin::core::platform: "debug"
```

### 9. Correlation IDs

```rust,ignore
use uuid::Uuid;

async fn handle_request(req: Request) -> Response {
    let request_id = Uuid::new_v4();

    let span = info_span!(
        "request",
        request_id = %request_id,
        method = %req.method(),
        path = %req.uri().path()
    );

    async {
        // All logs within this span include request_id
        info!("Processing request");
        // ...
    }.instrument(span).await
}
```

### 10. Sampling for High-Volume Logs

```rust,ignore
use rand::Rng;

// Sample 10% of debug logs
if tracing::enabled!(tracing::Level::DEBUG) && rand::thread_rng().gen_bool(0.1) {
    debug!(details = ?data, "Detailed debug information");
}
```

## Next Steps

- **[Monitoring](monitoring.md)** - Metrics and observability
- **[Troubleshooting](troubleshooting.md)** - Common issues
- **[Performance Tuning](performance-tuning.md)** - Optimization guide
