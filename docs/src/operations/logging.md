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

> **Scope note (2026-08-24, D-09/D-12 currency sweep).** The paragraph above and most of the
> code fences on this page describe logging via the `tracing` ecosystem (spans, `#[instrument]`,
> `tracing-subscriber` layers, `tracing-loki`, `tracing-elastic`, `tracing-appender`). That is
> **not** what is shipped. The live logging facade is the `log` crate
> (`Cargo.toml:14`, `log = "0.4.21"`) with `env_logger` for configuration and output
> (`src/main.rs:2,24`; `src/bin/paladin-server.rs:40`), wrapped by an application-layer
> `LogOrchestrator` (`src/application/services/log_orchestrator/mod.rs`) that routes typed
> `LogEntry` values to one of five `LogDestination`s — `System`, `Access`, `Error`, `Security`,
> `Performance`, or `Custom(name)` (`crates/paladin-core/src/platform/container/log.rs:76-89`) —
> through a `LogPort` implemented by `SystemLogAdapter`
> (`src/infrastructure/adapters/logs/system_log_adapter.rs`). `tracing-subscriber` is listed as
> a dependency (`Cargo.toml:119`) but has **zero** call sites anywhere in this workspace
> (`grep -rl 'tracing::\|tracing_subscriber' src/ crates/` returns no files); no crate here calls
> `tracing::info!`, uses `#[instrument]`, or registers a `tracing-subscriber` layer. `rand = "0.8"`
> (used in the Sampling snippet below) is a real dependency (`Cargo.toml:15`); `tracing-loki`,
> `tracing-elastic`, and `tracing-appender` are not dependencies anywhere in this workspace
> (`grep -n 'tracing-loki\|tracing-elastic\|tracing-appender' Cargo.toml` → 0 hits). Where this
> page's claims are independently checkable against the real facade (environment variables,
> `config.yml`), they are corrected in place below. The remaining `tracing`-shaped code fences
> are left as illustrative patterns rather than fully rewritten, per D-12's no-restructure guard —
> this note is the standing correction for all of them.

## Configuration

### Environment Variables

`env_logger` reads the standard `log`-crate directive syntax, so module-level filters work as
shown; `RUST_LOG_FORMAT` below was fabricated (`grep -rn RUST_LOG_FORMAT src/` → 0 hits) and has
been removed. `SYSTEM_LOG_LEVEL` is a real fallback, read only when `RUST_LOG` is unset
(`SystemLogAdapterConfig::from_env`, `src/infrastructure/adapters/logs/system_log_adapter.rs:59-61`).

```bash
# Set log level (read by env_logger, via the `log` crate's directive syntax)
export RUST_LOG=info,paladin=debug

# Enable specific modules
export RUST_LOG=paladin::core=debug,paladin::infrastructure=info

# Fallback used only when RUST_LOG is unset
export SYSTEM_LOG_LEVEL=info
```

### config.yml

**Fabricated — corrected 2026-08-24.** No `logging:` key exists anywhere in the `Settings`
struct or any of its per-domain config types (`grep -n logging src/config/settings.rs` → 0
hits); log configuration is not YAML-driven at all. It is set via the environment variables
above, plus the programmatic `SystemLogAdapterConfig`
(`src/infrastructure/adapters/logs/system_log_adapter.rs:31-41`):

```rust,ignore
pub struct SystemLogAdapterConfig {
    pub log_level: String,  // from RUST_LOG / SYSTEM_LOG_LEVEL
    pub format: LogFormat,  // Text (default) | Json | Structured(String)
    pub target: String,     // defaults to "paladin"
    pub structured: bool,
}
```

There is no Loki output, no rotation config, and no per-module or sampling YAML anywhere in the
tree — the `outputs:`/`modules:`/`sampling:` keys previously shown here did not correspond to any
real config surface. Structured routing by category is done in code via `LogDestination`
(`System`, `Access`, `Error`, `Security`, `Performance`, `Custom(name)` —
`crates/paladin-core/src/platform/container/log.rs:76-89`), not YAML.

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

**Note:** `tracing-loki` and `tracing-elastic` are not dependencies anywhere in this workspace
(`grep -n 'tracing-loki\|tracing-elastic' Cargo.toml` → 0 hits), and no such integration file
exists in this tree. The snippets below are illustrative of a pattern for wiring one up, not
shipped code.

### Loki Integration

```rust,ignore
// Cargo.toml
[dependencies]
tracing-loki = "0.2"

// illustrative only — no such file exists in this tree
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

`tracing-appender` is not a dependency anywhere in this workspace
(`grep -n tracing-appender Cargo.toml` → 0 hits), and the real `src/main.rs` initializes
`env_logger`, not this. `env_logger` itself has no built-in rotation
(`src/infrastructure/adapters/logs/system_log_adapter.rs:412`, "env_logger doesn't support
rotation — would need external log management"). This snippet is illustrative only.

```toml
# Cargo.toml
[dependencies]
tracing-appender = "0.2"

# illustrative only — real src/main.rs uses env_logger, not tracing-appender
use tracing_appender::rolling::{RollingFileAppender, Rotation};

let file_appender = RollingFileAppender::new(
    Rotation::DAILY,
    "/app/logs",
    "paladin.log"
);
```

### 8. Production Log Level

There is no `logging:` YAML key (see the config.yml correction above). The real equivalent is
the `RUST_LOG` directive syntax:

```bash
# Production: reduce log volume, keep debug for one module
export RUST_LOG=warn,paladin::core::platform=debug
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
