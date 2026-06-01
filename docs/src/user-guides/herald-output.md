# Herald Output Formatting System

The **Herald** is Paladin's pluggable output formatting system that transforms Paladin and Battalion execution results into human-readable formats. It provides multiple built-in formatters (JSON, Markdown, Table) and supports custom formatters through a simple trait-based interface.

## Table of Contents

- [Overview](#overview)
- [Quick Start](#quick-start)
- [Built-in Formatters](#built-in-formatters)
  - [JSON Herald](#json-herald)
  - [Markdown Herald](#markdown-herald)
  - [Table Herald](#table-herald)
- [Configuration](#configuration)
- [Usage Patterns](#usage-patterns)
  - [Paladin Execution](#paladin-execution)
  - [Battalion Execution](#battalion-execution)
  - [Runtime Override](#runtime-override)
- [Streaming Support](#streaming-support)
- [Custom Formatters](#custom-formatters)
- [Performance](#performance)
- [API Reference](#api-reference)

---

## Overview

The Herald system follows the **Hexagonal Architecture** pattern:

```
Core (Domain)                   Application (Ports)              Infrastructure (Adapters)
┌─────────────────┐            ┌──────────────────┐            ┌─────────────────────┐
│ Herald Trait    │────────────│ Herald Port      │────────────│ JsonHerald          │
│ PaladinResult   │            │ HeraldRegistry   │            │ MarkdownHerald      │
│ BattalionResult │            │                  │            │ TableHerald         │
│ StreamChunk     │            │                  │            │ (Your Custom Herald)│
└─────────────────┘            └──────────────────┘            └─────────────────────┘
```

**Key Features:**
- 🎨 **Multiple Formats**: JSON, Markdown, and Table formatters included
- ⚡ **High Performance**: <1ms for 10KB results (tested at 0.0095ms)
- 🔌 **Pluggable**: Easy to add custom formatters
- 📡 **Streaming Support**: Progressive output for long-running tasks
- ⚙️ **Configurable**: YAML-based configuration with runtime overrides
- 🏗️ **Type-Safe**: Strong typing with comprehensive error handling

---

## Quick Start

### 1. Configure Herald in `config.yml`

```yaml
herald:
  default_formatter: "json"  # or "markdown", "table"
  include_metadata: true

  # JSON-specific options
  json:
    pretty_print: true
    include_timestamps: true

  # Markdown-specific options
  markdown:
    use_colors: true
    heading_level: 2

  # Table-specific options
  table:
    max_column_width: 60
    border_style: "rounded"  # or "ascii", "modern", "none"
```

### 2. Use Herald with Paladin

```rust
use paladin::application::services::paladin::paladin_builder::PaladinBuilder;
use paladin::application::services::paladin::paladin_execution_service::PaladinExecutionService;
use paladin::config::Settings;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load settings
    let settings = Settings::new()?;

    // Create Herald from config
    let herald = settings.create_default_herald()?;

    // Create LLM port (example with OpenAI)
    let config = OpenAIConfig::from_env()?;
    let llm_port = Arc::new(OpenAIAdapter::new(config)?);

    // Build Paladin
    let paladin = PaladinBuilder::new(llm_port.clone())
        .system_prompt("You are a helpful assistant")
        .name("MyPaladin")
        .build()?;

    // Create execution service with Herald
    let service = PaladinExecutionService::new(
        llm_port,
        circuit_breaker,
        None,  // garrison
        None,  // arsenal
    ).with_herald(herald);

    // Execute and format
    let result = service.execute(&paladin, "Hello!").await?;
    if let Some(formatted) = service.format_result(&result, &paladin)? {
        println!("{}", formatted);
    }

    Ok(())
}
```

---

## Built-in Formatters

### JSON Herald

**Best for:** API integrations, structured logging, machine parsing

**Format:** Pretty-printed JSON with full metadata

```json
{
  "paladin_id": "paladin-123",
  "paladin_name": "DataAnalyst",
  "status": "completed",
  "output": "Analysis results here...",
  "metadata": {
    "execution_time_ms": 1245,
    "total_tokens": 523,
    "timestamp": "2026-01-26T10:30:45Z"
  }
}
```

**Features:**
- Pretty-printed by default (configurable)
- Optional timestamps
- NDJSON streaming (newline-delimited JSON)
- Metadata in separate object

**Usage:**
```rust
use paladin::infrastructure::adapters::herald::JsonHerald;

let herald = Arc::new(JsonHerald::new());
// or with custom config
let herald = Arc::new(JsonHerald::new().with_config(JsonHeraldConfig {
    pretty_print: false,
    include_timestamps: true,
}));
```

### Markdown Herald

**Best for:** Human-readable reports, documentation, CLI output

**Format:** Structured Markdown with colors and formatting

```markdown
## ✅ Paladin: DataAnalyst

**Status:** completed
**Output:**
Analysis results here...

---
*Execution Time: 1.25s | Tokens: 523 | Timestamp: 2026-01-26 10:30:45*
```

**Features:**
- Color-coded status badges (✅ ❌ ⏱️)
- Configurable heading levels
- Progressive streaming (immediate text output)
- Optional ANSI colors

**Usage:**
```rust
use paladin::infrastructure::adapters::herald::MarkdownHerald;

let herald = Arc::new(MarkdownHerald::new());
// or with custom config
let herald = Arc::new(MarkdownHerald::new().with_config(MarkdownHeraldConfig {
    use_colors: true,
    heading_level: 3,
}));
```

### Table Herald

**Best for:** Terminal dashboards, side-by-side comparisons, compact summaries

**Format:** ASCII/Unicode tables with borders

```
┌────────────┬───────────┬──────────────────────┐
│ Field      │ Value     │ Details              │
├────────────┼───────────┼──────────────────────┤
│ Paladin    │ DataAna…  │ Status: completed    │
│ Output     │ Analysis… │ (truncated to 60ch)  │
│ Time       │ 1.25s     │ Tokens: 523          │
└────────────┴───────────┴──────────────────────┘
```

**Features:**
- Multiple border styles (rounded, ascii, modern, none)
- Automatic text truncation (configurable)
- Buffered streaming (renders complete table at end)
- Compact representation

**Usage:**
```rust
use paladin::infrastructure::adapters::herald::TableHerald;

let herald = Arc::new(TableHerald::default());
// or with custom config
let herald = Arc::new(TableHerald::new().with_config(TableHeraldConfig {
    max_column_width: 80,
    border_style: "modern".to_string(),
}));
```

---

## Configuration

### YAML Configuration

All Herald settings are defined in `config.yml`:

```yaml
herald:
  # Global settings
  default_formatter: "json"        # Default formatter to use
  include_metadata: true            # Include execution metadata

  # JSON formatter configuration
  json:
    pretty_print: true              # Pretty-print JSON (vs compact)
    include_timestamps: true        # Add ISO 8601 timestamps

  # Markdown formatter configuration
  markdown:
    use_colors: true                # Use ANSI colors in output
    heading_level: 2                # Heading level (1-6)

  # Table formatter configuration
  table:
    max_column_width: 60            # Max chars per column
    border_style: "rounded"         # rounded|ascii|modern|none
```

### Environment Variable Overrides

```bash
# Override default formatter
export PALADIN_HERALD__DEFAULT_FORMATTER=markdown

# Override JSON settings
export PALADIN_HERALD__JSON__PRETTY_PRINT=false

# Override table settings
export PALADIN_HERALD__TABLE__MAX_COLUMN_WIDTH=100
```

### Validation Rules

- `default_formatter`: Must be "json", "markdown", or "table"
- `heading_level`: Must be 1-6
- `max_column_width`: Must be > 0
- `border_style`: Must be "rounded", "ascii", "modern", or "none"

Invalid configurations will return a `HeraldError::ConfigurationError`.

---

## Usage Patterns

### Paladin Execution

```rust
use paladin::application::services::paladin::paladin_execution_service::PaladinExecutionService;

// Create service with Herald
let service = PaladinExecutionService::new(llm_port, cb, None, None)
    .with_herald(herald);

// Execute
let result = service.execute(&paladin, "input").await?;

// Format result
match service.format_result(&result, &paladin)? {
    Some(formatted) => println!("{}", formatted),
    None => println!("No Herald configured"),
}
```

### Battalion Execution

**Formation (Sequential):**
```rust
use paladin::application::services::battalion::formation_service::FormationExecutionService;

let service = FormationExecutionService::new(llm_port, cb, None, None)
    .with_herald(herald);

let result = service.execute(&formation, "input").await?;

// Format all Paladin results with enumeration
if let Some(formatted) = service.format_result(&result)? {
    println!("{}", formatted);
}
```

**Phalanx (Concurrent):**
```rust
use paladin::application::services::battalion::phalanx_service::PhalanxExecutionService;

let service = PhalanxExecutionService::new(
    llm_port,
    cb,
    aggregation_strategy,
    None,
    None,
).with_herald(herald);

let result = service.execute(&phalanx, "input").await?;

if let Some(formatted) = service.format_result(&result)? {
    println!("{}", formatted);
}
```

### Runtime Override

Override the Herald at runtime without changing configuration:

```rust
// Load default Herald from config
let default_herald = settings.create_default_herald()?;

// Create service with default
let mut service = PaladinExecutionService::new(llm_port, cb, None, None)
    .with_herald(default_herald);

// Execute with JSON
let result1 = service.execute(&paladin, "task1").await?;
let json_output = service.format_result(&result1, &paladin)?;

// Override to Markdown for specific task
let markdown_herald = Arc::new(MarkdownHerald::new());
service = service.with_herald(markdown_herald);

let result2 = service.execute(&paladin, "task2").await?;
let markdown_output = service.format_result(&result2, &paladin)?;

// Override to Table
let table_herald = Arc::new(TableHerald::default());
service = service.with_herald(table_herald);

let result3 = service.execute(&paladin, "task3").await?;
let table_output = service.format_result(&result3, &paladin)?;
```

---

## Streaming Support

Herald supports progressive output for long-running tasks through streaming:

### Streaming Architecture

```rust
pub struct StreamChunk {
    pub content: String,
    pub is_final: bool,
}

pub struct ExecutionMetadata {
    pub execution_time_ms: u64,
    pub total_tokens: u32,
}

pub trait Herald: Send + Sync {
    fn format_stream_chunk(&self, chunk: &StreamChunk) -> Result<Option<String>>;
    fn finalize_stream(&self, metadata: &ExecutionMetadata) -> Result<String>;
}
```

### Streaming Strategies

**JSON Herald (NDJSON):**
- Each chunk is a separate JSON object on its own line
- Newline-delimited JSON (NDJSON) format
- Can be parsed line-by-line as it streams

```json
{"content":"First chunk","is_final":false}
{"content":"Second chunk","is_final":false}
{"content":"Final chunk","is_final":true}
{"type":"metadata","execution_time_ms":1000,"total_tokens":500}
```

**Markdown Herald (Progressive):**
- Chunks append directly to output as text
- Immediate visibility for users
- Metadata added as footer section when finalized

```markdown
First chunk Second chunk Final chunk
---
*Execution Time: 1.00s | Tokens: 500*
```

**Table Herald (Buffered):**
- All chunks return `None` (buffered internally)
- Complete table rendered only in `finalize_stream()`
- Ensures proper table formatting

```
(nothing until finalize)
┌────────────┬──────────────────┐
│ Field      │ Value            │
├────────────┼──────────────────┤
│ Output     │ Complete content │
│ Time       │ 1.00s            │
└────────────┴──────────────────┘
```

### Streaming Example

```rust
// Create Herald
let herald = Arc::new(JsonHerald::new());

// Process stream
let mut output = String::new();

for chunk in stream {
    if let Some(formatted) = herald.format_stream_chunk(&chunk)? {
        output.push_str(&formatted);
        print!("{}", formatted);  // Progressive output
    }
}

// Finalize
let metadata = ExecutionMetadata {
    execution_time_ms: 1000,
    total_tokens: 500,
};
let final_line = herald.finalize_stream(&metadata)?;
output.push_str(&final_line);
println!("{}", final_line);
```

---

## Custom Formatters

Implement the `Herald` trait to create custom formatters:

### Example: XML Herald

```rust
use paladin::core::platform::container::herald::{
    Herald, PaladinResult, BattalionResult, StreamChunk, ExecutionMetadata, HeraldError,
};
use async_trait::async_trait;

pub struct XmlHerald;

impl Herald for XmlHerald {
    fn name(&self) -> &str {
        "xml"
    }

    fn mime_type(&self) -> &str {
        "application/xml"
    }

    fn format_paladin_result(&self, result: &PaladinResult) -> Result<String, HeraldError> {
        Ok(format!(
            r#"<?xml version="1.0" encoding="UTF-8"?>
<paladin_result>
    <paladin_id>{}</paladin_id>
    <paladin_name>{}</paladin_name>
    <status>{}</status>
    <output>{}</output>
</paladin_result>"#,
            result.paladin_id,
            result.paladin_name,
            result.status,
            xml_escape(&result.output),
        ))
    }

    fn format_battalion_result(&self, result: &BattalionResult) -> Result<String, HeraldError> {
        let mut xml = format!(
            r#"<?xml version="1.0" encoding="UTF-8"?>
<battalion_result>
    <battalion_id>{}</battalion_id>
    <battalion_name>{}</battalion_name>
    <status>{}</status>
    <paladins>"#,
            result.battalion_id,
            result.battalion_name,
            result.status,
        );

        for paladin in &result.results {
            xml.push_str(&format!(
                r#"
        <paladin id="{}">
            <name>{}</name>
            <status>{}</status>
            <output>{}</output>
        </paladin>"#,
                paladin.paladin_id,
                paladin.paladin_name,
                paladin.status,
                xml_escape(&paladin.output),
            ));
        }

        xml.push_str("\n    </paladins>\n</battalion_result>");
        Ok(xml)
    }

    fn format_error(&self, error: &str) -> Result<String, HeraldError> {
        Ok(format!(
            r#"<?xml version="1.0" encoding="UTF-8"?>
<error>{}</error>"#,
            xml_escape(error)
        ))
    }

    fn format_stream_chunk(&self, chunk: &StreamChunk) -> Result<Option<String>, HeraldError> {
        // XML streaming: wrap each chunk
        Ok(Some(format!(
            r#"<chunk is_final="{}">{}</chunk>"#,
            chunk.is_final,
            xml_escape(&chunk.content)
        )))
    }

    fn finalize_stream(&self, metadata: &ExecutionMetadata) -> Result<String, HeraldError> {
        Ok(format!(
            r#"<metadata execution_time_ms="{}" total_tokens="{}"/>"#,
            metadata.execution_time_ms,
            metadata.total_tokens
        ))
    }
}

fn xml_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}
```

### Register Custom Herald

```rust
use paladin::application::services::herald::herald_registry::HeraldRegistry;

// Create registry
let mut registry = HeraldRegistry::default();

// Register custom herald
let xml_herald = Arc::new(XmlHerald);
registry.register("xml".to_string(), xml_herald);

// Get when needed
let herald = registry.get("xml").expect("Herald not found");
```

---

## Performance

Herald formatters are designed for **minimal overhead**:

### Benchmark Results

| Formatter | Data Size | Time | vs Target |
|-----------|-----------|------|-----------|
| JSON | 1 KB | 2.0 µs | - |
| JSON | 5 KB | 5.4 µs | - |
| JSON | 10 KB | **9.5 µs** | **105x faster** than 1ms target |
| JSON | 50 KB | 42.8 µs | 23x faster |
| Markdown | 10 KB | ~10 µs | ~200x faster than 2ms target |
| Table | 10 KB | ~10 µs | ~200x faster than 2ms target |

**Key Takeaways:**
- All formatters process 10KB results in **under 10 microseconds**
- Performance exceeds requirements by **orders of magnitude**
- Zero-copy operations where possible
- Efficient string building with pre-allocation

### Performance Tips

1. **Use appropriate formatter**: JSON for APIs, Markdown for humans, Table for dashboards
2. **Disable pretty-printing**: Set `pretty_print: false` for JSON in production
3. **Limit output size**: Truncate large outputs before formatting
4. **Buffer streaming**: Use Table Herald's buffering for UI consistency

---

## API Reference

### Core Types

```rust
/// Main Herald trait for output formatting
pub trait Herald: Send + Sync {
    fn name(&self) -> &str;
    fn mime_type(&self) -> &str;
    fn format_paladin_result(&self, result: &PaladinResult) -> Result<String, HeraldError>;
    fn format_battalion_result(&self, result: &BattalionResult) -> Result<String, HeraldError>;
    fn format_error(&self, error: &str) -> Result<String, HeraldError>;
    fn format_stream_chunk(&self, chunk: &StreamChunk) -> Result<Option<String>, HeraldError>;
    fn finalize_stream(&self, metadata: &ExecutionMetadata) -> Result<String, HeraldError>;
}

/// Paladin execution result
pub struct PaladinResult {
    pub paladin_id: String,
    pub paladin_name: String,
    pub status: String,
    pub output: String,
}

/// Battalion execution result
pub struct BattalionResult {
    pub battalion_id: String,
    pub battalion_name: String,
    pub status: String,
    pub results: Vec<PaladinResult>,
}

/// Stream chunk for progressive output
pub struct StreamChunk {
    pub content: String,
    pub is_final: bool,
}

/// Execution metadata for stream finalization
pub struct ExecutionMetadata {
    pub execution_time_ms: u64,
    pub total_tokens: u32,
}
```

### Error Types

```rust
#[derive(Debug, thiserror::Error)]
pub enum HeraldError {
    #[error("Configuration error: {0}")]
    ConfigurationError(String),

    #[error("Formatting error: {0}")]
    FormattingError(String),

    #[error("Invalid result: {0}")]
    InvalidResult(String),

    #[error("Serialization error: {0}")]
    SerializationError(String),
}
```

### Configuration Types

```rust
/// JSON Herald configuration
pub struct JsonHeraldConfig {
    pub pretty_print: bool,
    pub include_timestamps: bool,
}

/// Markdown Herald configuration
pub struct MarkdownHeraldConfig {
    pub use_colors: bool,
    pub heading_level: u8,
}

/// Table Herald configuration
pub struct TableHeraldConfig {
    pub max_column_width: usize,
    pub border_style: String,
}
```

---

## Best Practices

### 1. Choose the Right Formatter

- **JSON**: APIs, logging systems, structured data stores
- **Markdown**: Human-readable reports, CLI tools, documentation
- **Table**: Terminal dashboards, comparison views, compact summaries

### 2. Configure Appropriately

```yaml
# Development: Pretty and colorful
herald:
  default_formatter: "markdown"
  markdown:
    use_colors: true

# Production: Compact and structured
herald:
  default_formatter: "json"
  json:
    pretty_print: false
```

### 3. Handle Errors Gracefully

```rust
match service.format_result(&result, &paladin) {
    Ok(Some(formatted)) => println!("{}", formatted),
    Ok(None) => println!("Raw output: {}", result.output),
    Err(e) => eprintln!("Formatting error: {}", e),
}
```

### 4. Use Runtime Overrides Sparingly

```rust
// Good: Configure once
let herald = settings.create_default_herald()?;
let service = PaladinExecutionService::new(...).with_herald(herald);

// Avoid: Changing formatter for every request
// (unless truly needed for different output destinations)
```

### 5. Test Custom Formatters

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_custom_formatter() {
        let herald = XmlHerald;
        let result = PaladinResult {
            paladin_id: "test-1".to_string(),
            paladin_name: "TestPaladin".to_string(),
            status: "completed".to_string(),
            output: "Test output".to_string(),
        };

        let formatted = herald.format_paladin_result(&result).unwrap();
        assert!(formatted.contains("<paladin_result>"));
        assert!(formatted.contains("test-1"));
    }
}
```

---

## Troubleshooting

### Issue: Herald not formatting output

**Symptoms:** `format_result()` returns `None`

**Solutions:**
1. Verify Herald is configured: `service.with_herald(herald)`
2. Check that Herald is Some, not None
3. Ensure configuration is valid

### Issue: Formatting fails with error

**Symptoms:** `HeraldError::FormattingError`

**Solutions:**
1. Check result data is valid (no null/empty required fields)
2. Verify custom formatter implementation handles edge cases
3. Review error message for specific cause

### Issue: Colors not showing in Markdown

**Symptoms:** ANSI codes visible as text

**Solutions:**
1. Ensure terminal supports ANSI colors
2. Check `use_colors` is set to `true` in config
3. Use a color-capable terminal emulator

### Issue: Table borders not displaying correctly

**Symptoms:** Broken box characters

**Solutions:**
1. Use UTF-8 compatible terminal
2. Switch to `border_style: "ascii"` for compatibility
3. Set `border_style: "none"` to disable borders

---

## Examples

See the `examples/` directory for complete working examples:

- [`herald_json_output.rs`](../examples/herald_json_output.rs) - JSON formatting
- [`herald_markdown_output.rs`](../examples/herald_markdown_output.rs) - Markdown formatting
- [`herald_custom_formatter.rs`](../examples/herald_custom_formatter.rs) - XML/CSV custom formatters
- [`herald_streaming.rs`](../examples/herald_streaming.rs) - Streaming formatters
- [`basic_paladin.rs`](../examples/basic_paladin.rs) - Updated with Herald usage

---

## Further Reading

- [Design and Architecture](Design/Design_and_Architecture.md) - Overall system architecture
- [Battalion Documentation](BATTALION.md) - Multi-agent orchestration
- [Garrison Documentation](GARRISON.md) - Memory system
- [Arsenal Documentation](ARSENAL.md) - Tool integration
- [Hexagonal Architecture](../notes/hexagonal-arch.md) - Architectural pattern

---

**Questions or Issues?** See [CONTRIBUTING.md](../CONTRIBUTING.md) or open an issue on GitHub.
