# Product Requirements Document: Herald Output Formatting System

**Epic:** Epic 8  
**Priority:** Low  
**Effort:** 1-2 weeks  
**Dependencies:** Epic 1 (Paladin Domain Foundation)  
**Team:** 1 developer  
**Version:** 1.0  
**Date:** January 26, 2026

---

## 1. Introduction/Overview

The Herald Output Formatting system provides a flexible, extensible mechanism for formatting Paladin and Battalion execution results into multiple output formats. As Paladin is designed for diverse use cases (CLI tools, APIs, logging systems, external integrations), the ability to transform execution results into appropriate formats is critical for system usability and integration.

**Problem Statement:**  
Currently, Paladin execution results are returned as raw Rust structs. Different consumers need different representations:
- Developers need human-readable terminal output during development
- API consumers need machine-readable JSON responses
- Documentation systems need structured Markdown
- Monitoring systems need tabular/structured data

**Solution:**  
Implement a Herald trait-based formatting system with built-in formatters (JSON, Markdown, Table) and support for custom user-defined formatters. The system will support both complete result formatting and streaming output formatting, with configurable defaults and runtime overrides.

---

## 2. Goals

1. **Multi-Format Support:** Enable Paladin results to be formatted as JSON, Markdown, and structured tables
2. **Extensibility:** Allow users to implement custom Herald formatters via trait implementation
3. **Streaming Compatibility:** Support both complete result formatting and progressive streaming output
4. **Configuration Flexibility:** Provide global defaults with per-execution runtime overrides
5. **Metadata Preservation:** Ensure all execution metadata (tokens, loops, timing, errors) is included in formatted output
6. **Zero Overhead:** Formatting should not impact core execution performance (lazy formatting)
7. **Type Safety:** Leverage Rust's type system to ensure correct formatter usage at compile time

---

## 3. User Stories

### US-1: Developer CLI Output
**As a** developer using Paladin via CLI,  
**I want** to see formatted Markdown output in my terminal,  
**So that** I can easily read and understand Paladin execution results during development.

**Acceptance Criteria:**
- Paladin results display with proper headings, status indicators, and metadata
- Output is color-coded when terminal supports ANSI colors
- Metadata sections are clearly separated from content

### US-2: API JSON Response
**As an** API consumer,  
**I want** Paladin results returned as structured JSON,  
**So that** I can programmatically process results in my application.

**Acceptance Criteria:**
- JSON output includes all fields (output, status, metadata, errors)
- JSON schema is consistent and well-documented
- Optional pretty-printing for debugging

### US-3: Custom Formatter Implementation
**As a** Paladin integrator,  
**I want** to implement a custom Herald formatter (e.g., XML, CSV, Protocol Buffers),  
**So that** I can integrate Paladin with legacy systems requiring specific formats.

**Acceptance Criteria:**
- Herald trait is public and well-documented
- Examples demonstrate custom formatter implementation
- Custom formatters work seamlessly with existing infrastructure

### US-4: Streaming Output Formatting
**As a** user of streaming Paladin responses,  
**I want** output formatted progressively as tokens arrive,  
**So that** I can display real-time updates to end users.

**Acceptance Criteria:**
- Streaming formatters handle partial results
- Format consistency maintained between streaming and complete modes
- Errors during streaming are handled gracefully

### US-5: Runtime Format Selection
**As a** developer,  
**I want** to override the default formatter at execution time,  
**So that** I can use different formats for different use cases without reconfiguring.

**Acceptance Criteria:**
- Execution methods accept optional formatter parameter
- Global default is used when no override specified
- Override does not affect global configuration

### US-6: Battalion Result Aggregation
**As a** user of Battalion multi-agent orchestration,  
**I want** formatted output showing contributions from each Paladin,  
**So that** I can understand how different agents contributed to the final result.

**Acceptance Criteria:**
- Battalion results show individual Paladin outputs
- Execution order/parallelism is reflected in output
- Aggregated metadata (total tokens, total time) is included

---

## 4. Functional Requirements

### FR-1: Herald Trait Definition
The system **must** define a `Herald` trait with the following methods:
- `format_paladin_result(&self, result: &PaladinResult) -> String`
- `format_battalion_result(&self, result: &BattalionResult) -> String`
- `format_paladin_stream(&self, chunk: &StreamChunk) -> Option<String>` (for streaming)
- `format_error(&self, error: &PaladinError) -> String`

The trait **must** be `Send + Sync` for async compatibility.

### FR-2: Built-in Formatters
The system **must** provide three built-in Herald implementations:

**JsonHerald:**
- Serializes results to JSON using `serde_json`
- Supports optional pretty-printing via configuration
- Includes all metadata fields
- Schema documented in rustdoc

**MarkdownHerald:**
- Formats results as structured Markdown
- Includes headings, code blocks, and status badges
- Preserves whitespace and formatting from Paladin output
- Suitable for documentation generation

**TableHerald:**
- Formats results as ASCII tables using `comfy-table` or similar
- Supports both single-result and multi-result (Battalion) tables
- Includes column headers for metadata fields
- Configurable column widths

### FR-3: Streaming Support
The system **must** support both output modes:

**Complete Mode:**
- Formats entire `PaladinResult` after execution completes
- All metadata available

**Streaming Mode:**
- Formats partial output as tokens arrive from LLM
- Progressive formatting maintains consistency
- Final metadata appended when stream completes
- Errors during streaming include partial output

### FR-4: Configuration Integration
The system **must** integrate with existing configuration system:

```yaml
# config.yml
herald:
  default_formatter: "json"  # "json", "markdown", "table"
  json:
    pretty: true
    include_metadata: true
  markdown:
    include_colors: true
    heading_level: 2
  table:
    max_column_width: 80
    border_style: "rounded"
```

### FR-5: Global Default with Runtime Override
The system **must** support both configuration patterns:

**Global Default:**
- Set in `config.yml` or environment variable
- Applied to all executions unless overridden
- Can be changed at application startup

**Runtime Override:**
- Execution methods accept `Option<Arc<dyn Herald>>`
- Override takes precedence over global default
- Does not mutate global configuration

### FR-6: Paladin Result Formatting
For `PaladinResult`, formatted output **must** include:
- Paladin name and ID
- Execution status (Success, Failed, Timeout)
- Output text/content
- Metadata:
  - Loop count
  - Token usage (input, output, total)
  - Execution time
  - Tool calls made (if any)
  - Stop reason (stop word, max loops, completion)
- Error details (if failed)
- Timestamp

### FR-7: Battalion Result Formatting
For `BattalionResult`, formatted output **must** include:
- Battalion name, ID, and type (Formation, Phalanx, Campaign, Chain of Command)
- Overall status
- Individual Paladin results (formatted according to selected Herald)
- Execution order/graph (for Campaign)
- Aggregated metadata:
  - Total execution time
  - Total token usage across all Paladins
  - Success/failure counts
- Errors and partial results

### FR-8: Herald Registry
The system **must** provide a `HeraldRegistry` for managing formatters:
- Register built-in formatters by name
- Register custom formatters
- Retrieve formatters by name
- List available formatters
- Thread-safe concurrent access

### FR-9: Builder Pattern Integration
The system **must** integrate with `PaladinBuilder`:

```rust
PaladinBuilder::new(llm_port)
    .name("ResearchAgent")
    .with_herald(Arc::new(MarkdownHerald))
    .build()?
```

### FR-10: Error Handling
The system **must** handle errors gracefully:
- Formatting errors do not prevent result retrieval
- Fallback to basic string representation on formatter failure
- Errors include context (which formatter, which result)
- Partial formatting results preserved when possible

---

## 5. Non-Goals (Out of Scope)

The following are explicitly **not** included in this epic:

1. **Binary Formats:** Protocol Buffers, MessagePack, or other binary serialization formats (future enhancement)
2. **Template Engines:** External templating systems (Handlebars, Tera) - custom formatters can implement these
3. **Output Storage:** Automatic saving of formatted output to files/databases (separate concern)
4. **Format Conversion:** Converting between formats (e.g., Markdown → HTML) - use external tools
5. **Syntax Highlighting:** Code syntax highlighting in formatted output (terminal-specific feature)
6. **i18n/Localization:** Multi-language support for metadata labels (future enhancement)
7. **Output Compression:** Compressing formatted output (separate concern)
8. **Schema Validation:** Validating JSON output against external schemas (user responsibility)

---

## 6. Design Considerations

### 6.1 Architecture

Following Paladin's hexagonal architecture:

**Core Layer (`src/core/platform/container/herald.rs`):**
- `Herald` trait definition
- Core formatting types and enums
- Error types

**Application Layer (`src/application/ports/output/herald_port.rs`):**
- Herald port abstraction (if needed for dependency injection)
- Herald registry service

**Infrastructure Layer (`src/infrastructure/adapters/herald/`):**
- `json_herald.rs` - JSON formatter implementation
- `markdown_herald.rs` - Markdown formatter implementation
- `table_herald.rs` - Table formatter implementation

### 6.2 Trait Design

```rust
/// Herald trait for formatting Paladin execution results
#[async_trait]
pub trait Herald: Send + Sync {
    /// Format complete Paladin result
    fn format_paladin_result(&self, result: &PaladinResult) -> Result<String, HeraldError>;

    /// Format complete Battalion result
    fn format_battalion_result(&self, result: &BattalionResult) -> Result<String, HeraldError>;

    /// Format streaming chunk (progressive output)
    fn format_stream_chunk(&self, chunk: &StreamChunk) -> Result<Option<String>, HeraldError>;

    /// Finalize streaming output (append metadata)
    fn finalize_stream(&self, metadata: &ExecutionMetadata) -> Result<String, HeraldError>;

    /// Format error for display
    fn format_error(&self, error: &PaladinError) -> String;

    /// Get formatter name/identifier
    fn name(&self) -> &str;

    /// Get formatter MIME type (e.g., "application/json", "text/markdown")
    fn mime_type(&self) -> &str;
}
```

### 6.3 Example Usage

```rust
// Global default from config
let paladin = PaladinBuilder::new(llm_port)
    .name("Analyst")
    .build()?;

let result = paladin.execute("Analyze this data").await?;
let formatted = paladin.format_result(&result)?; // Uses config default

// Runtime override
let json_herald = Arc::new(JsonHerald::new());
let formatted_json = json_herald.format_paladin_result(&result)?;

// Streaming with formatting
let mut stream = paladin.execute_stream("Write a story").await?;
let markdown_herald = Arc::new(MarkdownHerald::default());

while let Some(chunk) = stream.next().await {
    if let Some(formatted) = markdown_herald.format_stream_chunk(&chunk)? {
        print!("{}", formatted); // Progressive output
    }
}

let final_metadata = markdown_herald.finalize_stream(&stream.metadata())?;
println!("{}", final_metadata);
```

### 6.4 JSON Schema Example

```json
{
  "paladin_id": "uuid-here",
  "paladin_name": "ResearchAgent",
  "status": "success",
  "output": "The analysis shows...",
  "metadata": {
    "loop_count": 2,
    "token_usage": {
      "input_tokens": 150,
      "output_tokens": 500,
      "total_tokens": 650
    },
    "execution_time_ms": 1234,
    "tool_calls": [
      {
        "tool_name": "web_search",
        "arguments": { "query": "AI trends 2026" },
        "result": "..."
      }
    ],
    "stop_reason": "completion",
    "timestamp": "2026-01-26T10:30:00Z"
  },
  "errors": []
}
```

### 6.5 Markdown Example

```markdown
## Paladin: ResearchAgent

**Status:** ✅ Success  
**Execution Time:** 1.23s  
**Tokens Used:** 650 (150 in, 500 out)

### Output

The analysis shows...

### Metadata

- **Loops:** 2
- **Stop Reason:** completion
- **Timestamp:** 2026-01-26T10:30:00Z

#### Tool Calls

1. **web_search**
   - Query: "AI trends 2026"
   - Result: ...
```

---

## 7. Technical Considerations

### 7.1 Dependencies

**Required crates:**
- `serde` + `serde_json` - JSON serialization (already in project)
- `comfy-table` or `tabled` - ASCII table rendering
- `ansi_term` or `colored` - Terminal color support (optional)

### 7.2 Performance

- **Lazy Formatting:** Results should not be formatted unless explicitly requested
- **Zero-Copy:** Use references in trait methods to avoid cloning large results
- **Streaming Efficiency:** Minimize buffering during streaming format operations
- **Allocation:** Prefer `String` over `Vec<u8>` for formatter output (UTF-8 guaranteed)

### 7.3 Testing

- **Unit Tests:** Each formatter tested independently with mock results
- **Integration Tests:** End-to-end formatting with real Paladin executions
- **Property Tests:** Format → Parse → Format roundtrip for JSON
- **Benchmark Tests:** Formatter performance with large results (>100KB output)

### 7.4 Documentation

- **Rustdoc:** Comprehensive documentation for Herald trait and all implementations
- **Examples:** `examples/herald_custom_formatter.rs` demonstrating custom implementation
- **User Guide:** Documentation in `docs/HERALD.md` with usage patterns

### 7.5 Error Handling

```rust
#[derive(Debug, thiserror::Error)]
pub enum HeraldError {
    #[error("Serialization error: {0}")]
    SerializationError(String),

    #[error("Template error: {0}")]
    TemplateError(String),

    #[error("Invalid result structure: {0}")]
    InvalidResult(String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}
```

---

## 8. Success Metrics

The Herald system will be considered successful when:

1. **Adoption:** All example code and CLI tools use Herald formatters (100% coverage)
2. **Test Coverage:** Herald module achieves ≥ 80% unit test coverage
3. **Performance:** Formatting overhead < 5% of total execution time for typical results
4. **Extensibility:** At least one community-contributed custom formatter created
5. **Documentation:** Herald documentation receives positive feedback (no confusion issues)
6. **Streaming:** Streaming formatters maintain consistent output with complete formatters
7. **Zero Regressions:** No existing functionality broken by Herald introduction

**Quantitative Metrics:**
- JSON formatter performance: < 1ms for results up to 10KB
- Markdown formatter performance: < 2ms for results up to 10KB
- Zero formatter-related crashes in production use
- API response times unchanged after Herald integration

---

## 9. Open Questions

1. **Color Support Strategy:**  
   Should color support be automatic (detect terminal capability) or explicit configuration?
   - **Recommendation:** Auto-detect with config override

2. **Streaming Buffer Size:**  
   What buffer size should streaming formatters use for partial output?
   - **Recommendation:** 1KB default, configurable

3. **Battalion Formatting Depth:**  
   Should nested Battalions (Battalion containing Battalions) show full recursion or summarize?
   - **Recommendation:** Configurable depth limit (default: 2 levels)

4. **Custom Formatter Registration:**  
   Should custom formatters be registered via config file or only in code?
   - **Recommendation:** Code registration only for v1, config in future

5. **Format Negotiation:**  
   Should the system support HTTP-style content negotiation (Accept headers)?
   - **Recommendation:** Out of scope for v1, evaluate for API layer

6. **Async Formatting:**  
   Should Herald trait methods be async (for future formatters that make network calls)?
   - **Recommendation:** Start with sync, consider async in v2 if needed

---

## 10. Implementation Phases

### Phase 1: Core Infrastructure (Days 1-3)
- Define Herald trait in core layer
- Implement HeraldError types
- Create HeraldRegistry
- Add configuration schema
- Write trait documentation

### Phase 2: Built-in Formatters (Days 4-6)
- Implement JsonHerald
- Implement MarkdownHerald
- Implement TableHerald
- Unit tests for each formatter

### Phase 3: Integration (Days 7-8)
- Integrate with PaladinExecutionService
- Add Battalion formatting support
- Configuration loading
- Builder pattern integration

### Phase 4: Streaming Support (Days 9-10)
- Implement streaming trait methods
- Add streaming tests
- Update existing streaming examples

### Phase 5: Testing & Documentation (Days 11-12)
- Integration tests
- Performance benchmarks
- Write HERALD.md documentation
- Create custom formatter example
- Update existing examples to use Herald

---

## 11. Acceptance Testing

### Test Scenario 1: Basic Formatting
```rust
#[tokio::test]
async fn test_json_herald_formats_paladin_result() {
    let herald = JsonHerald::new();
    let result = create_mock_paladin_result();
    let formatted = herald.format_paladin_result(&result).unwrap();
    let parsed: serde_json::Value = serde_json::from_str(&formatted).unwrap();

    assert_eq!(parsed["status"], "success");
    assert_eq!(parsed["paladin_name"], "TestPaladin");
}
```

### Test Scenario 2: Runtime Override
```rust
#[tokio::test]
async fn test_runtime_herald_override() {
    let paladin = PaladinBuilder::new(llm_port)
        .name("Agent")
        .build()?;

    let result = paladin.execute("test").await?;

    // Default formatter (from config)
    let default_output = paladin.format_result(&result)?;

    // Override with Markdown
    let markdown_herald = Arc::new(MarkdownHerald::default());
    let markdown_output = markdown_herald.format_paladin_result(&result)?;

    assert!(markdown_output.contains("##")); // Markdown heading
    assert_ne!(default_output, markdown_output);
}
```

### Test Scenario 3: Streaming Consistency
```rust
#[tokio::test]
async fn test_streaming_output_matches_complete() {
    let herald = JsonHerald::new();
    let mut stream = create_mock_stream();

    let mut streamed_parts = Vec::new();
    while let Some(chunk) = stream.next().await {
        if let Some(formatted) = herald.format_stream_chunk(&chunk)? {
            streamed_parts.push(formatted);
        }
    }

    let complete_result = stream.into_result();
    let complete_formatted = herald.format_paladin_result(&complete_result)?;

    // Verify streaming assembled output matches complete formatting
    assert_output_semantically_equal(&streamed_parts.join(""), &complete_formatted);
}
```

---

## Appendix A: Related Documents

- [Paladin Project Completion Plan](./Milestone_1-MVP/Paladin Project Completion Plan.md)
- [Epic 1: Paladin Domain Foundation](./Milestone_1-MVP/Epic_1/)
- [Hexagonal Architecture Guide](../notes/hexagonal-arch.md)
- [Design and Architecture](../docs/Design/Design_and_Architecture.md)

## Appendix B: Glossary

- **Herald:** A formatter that transforms Paladin execution results into specific output formats
- **Stream Chunk:** A partial piece of output received during streaming execution
- **MIME Type:** Media type identifier (e.g., "application/json") for formatted content
- **Progressive Formatting:** Formatting output incrementally as it arrives, rather than waiting for completion
- **Format Negotiation:** Automatic selection of formatter based on client preferences (future feature)

---

**Document Status:** Ready for Review  
**Next Steps:**  
1. Technical review by senior developer
2. Approval from product owner
3. Create implementation tasks
4. Begin Phase 1 development
