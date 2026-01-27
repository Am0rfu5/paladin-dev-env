# Output Formatting Guide

This guide covers the Herald system for formatting and controlling Paladin output in various formats and styles.

## Table of Contents

- [Overview](#overview)
- [Herald Architecture](#herald-architecture)
- [Built-in Formatters](#built-in-formatters)
- [Custom Formatters](#custom-formatters)
- [Streaming Output](#streaming-output)
- [Multi-Format Output](#multi-format-output)
- [Post-Processing](#post-processing)
- [Best Practices](#best-practices)
- [Advanced Patterns](#advanced-patterns)
- [Troubleshooting](#troubleshooting)

## Overview

The Herald system controls how Paladin output is formatted and presented to users.

**Key Capabilities:**
- **Format Transformation**: Convert LLM output to JSON, Markdown, HTML, etc.
- **Streaming**: Real-time output delivery for better UX
- **Validation**: Ensure output meets schema requirements
- **Post-Processing**: Clean, enhance, or transform responses
- **Multi-Channel**: Different formats for different output destinations

**Key Concepts:**
- **Herald**: Output formatting system
- **Formatter**: Converts raw LLM output to specific format
- **OutputFormat**: Target format specification (JSON, Markdown, Plain, etc.)
- **StreamHandler**: Processes output chunks in real-time

## Herald Architecture

### Core Components

```rust
// Output format types
pub enum OutputFormat {
    Plain,         // Raw LLM output
    Markdown,      // Markdown-formatted
    Json,          // Structured JSON
    Html,          // HTML rendering
    Custom(String), // Custom format name
}

// Herald interface
#[async_trait]
pub trait Herald: Send + Sync {
    /// Format complete output
    async fn format(&self, content: &str) -> Result<String, HeraldError>;
    
    /// Format streaming chunk
    async fn format_chunk(&self, chunk: &str) -> Result<String, HeraldError>;
    
    /// Validate output against format requirements
    fn validate(&self, content: &str) -> Result<(), HeraldError>;
    
    /// Get format metadata
    fn metadata(&self) -> FormatMetadata;
}

// Format metadata
pub struct FormatMetadata {
    pub format_name: String,
    pub mime_type: String,
    pub file_extension: String,
    pub supports_streaming: bool,
}
```

### Integration with Paladin

```rust
let paladin = PaladinBuilder::new(llm_adapter)
    .name("Assistant")
    .system_prompt("You are a helpful assistant.")
    .output_format(OutputFormat::Markdown)
    .with_herald(Arc::new(MarkdownHerald::default()))
    .build()?;

let response = paladin.execute("Explain async/await").await?;
// response.content is formatted as Markdown
```

## Built-in Formatters

### Plain Text Herald

No formatting, returns raw LLM output.

```rust
use paladin::herald::*;

let herald = Arc::new(PlainHerald::default());

let paladin = PaladinBuilder::new(llm_adapter)
    .with_herald(herald)
    .build()?;

let response = paladin.execute("Hello").await?;
println!("{}", response.content);  // Raw output
```

### Markdown Herald

Formats output as Markdown with proper structure.

```rust
use paladin::herald::*;

let herald = Arc::new(MarkdownHerald::new()
    .with_code_highlighting(true)
    .with_header_ids(true)
    .with_table_of_contents(true)
);

let paladin = PaladinBuilder::new(llm_adapter)
    .system_prompt("Format all responses as Markdown with proper headers and code blocks.")
    .with_herald(herald)
    .build()?;

let response = paladin.execute("Explain Rust ownership").await?;
println!("{}", response.content);
```

**Output example:**
```markdown
# Rust Ownership

Ownership is a core concept in Rust that ensures memory safety.

## Key Rules

1. Each value has a single owner
2. When the owner goes out of scope, the value is dropped
3. Values can be borrowed immutably or mutably

## Example

```rust
fn main() {
    let s1 = String::from("hello");
    let s2 = s1;  // s1 is moved
    // println!("{}", s1);  // Error: s1 is no longer valid
}
```

## Benefits

- Memory safety without garbage collection
- No data races at compile time
- Zero-cost abstractions
```

### JSON Herald

Formats output as structured JSON.

```rust
use paladin::herald::*;
use serde_json::json;

let herald = Arc::new(JsonHerald::new()
    .with_schema(json!({
        "type": "object",
        "properties": {
            "summary": {"type": "string"},
            "key_points": {
                "type": "array",
                "items": {"type": "string"}
            },
            "confidence": {"type": "number"}
        },
        "required": ["summary", "key_points"]
    }))
    .validate_output(true)
);

let paladin = PaladinBuilder::new(llm_adapter)
    .system_prompt("Always respond in JSON format matching this schema: \
                    {summary: string, key_points: string[], confidence: number}")
    .with_herald(herald)
    .build()?;

let response = paladin.execute("Analyze sentiment of: 'This product is amazing!'").await?;

// Parse structured output
let json: serde_json::Value = serde_json::from_str(&response.content)?;
println!("Summary: {}", json["summary"]);
println!("Key points: {:?}", json["key_points"]);
```

**Output example:**
```json
{
  "summary": "Highly positive sentiment expressing enthusiasm",
  "key_points": [
    "Strong positive emotion indicated by 'amazing'",
    "Exclamation mark reinforces enthusiasm",
    "No negative indicators present"
  ],
  "confidence": 0.95
}
```

### HTML Herald

Formats output as styled HTML.

```rust
use paladin::herald::*;

let herald = Arc::new(HtmlHerald::new()
    .with_css_framework(CssFramework::Tailwind)
    .with_syntax_highlighting(true)
    .with_responsive_design(true)
);

let paladin = PaladinBuilder::new(llm_adapter)
    .with_herald(herald)
    .build()?;

let response = paladin.execute("Create a todo list").await?;

// Serve as web page
let html = format!(r#"
<!DOCTYPE html>
<html>
<head>
    <meta charset="UTF-8">
    <title>Paladin Response</title>
    <link href="https://cdn.jsdelivr.net/npm/tailwindcss@2/dist/tailwind.min.css" rel="stylesheet">
</head>
<body class="bg-gray-100 p-8">
    {}
</body>
</html>
"#, response.content);
```

### Code Herald

Specialized for code generation with syntax validation.

```rust
use paladin::herald::*;

let herald = Arc::new(CodeHerald::new()
    .language("rust")
    .with_syntax_check(true)
    .with_formatting(true)
    .with_linting(true)
);

let paladin = PaladinBuilder::new(llm_adapter)
    .system_prompt("You are a Rust code generator. Return ONLY valid Rust code.")
    .with_herald(herald)
    .build()?;

let response = paladin.execute("Write a function to reverse a string").await?;

// Output is validated, formatted Rust code
println!("{}", response.content);
```

**Output:**
```rust
pub fn reverse_string(s: &str) -> String {
    s.chars().rev().collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_reverse_string() {
        assert_eq!(reverse_string("hello"), "olleh");
        assert_eq!(reverse_string(""), "");
    }
}
```

## Custom Formatters

Create custom heralds for specialized output formats.

### Simple Custom Herald

```rust
use paladin::herald::*;
use async_trait::async_trait;

pub struct UppercaseHerald;

#[async_trait]
impl Herald for UppercaseHerald {
    async fn format(&self, content: &str) -> Result<String, HeraldError> {
        Ok(content.to_uppercase())
    }
    
    async fn format_chunk(&self, chunk: &str) -> Result<String, HeraldError> {
        Ok(chunk.to_uppercase())
    }
    
    fn validate(&self, _content: &str) -> Result<(), HeraldError> {
        Ok(())  // No validation needed
    }
    
    fn metadata(&self) -> FormatMetadata {
        FormatMetadata {
            format_name: "uppercase".to_string(),
            mime_type: "text/plain".to_string(),
            file_extension: "txt".to_string(),
            supports_streaming: true,
        }
    }
}

// Usage
let herald = Arc::new(UppercaseHerald);
let paladin = PaladinBuilder::new(llm_adapter)
    .with_herald(herald)
    .build()?;
```

### XML Herald

```rust
use paladin::herald::*;
use quick_xml::Writer;
use std::io::Cursor;

pub struct XmlHerald {
    root_element: String,
}

impl XmlHerald {
    pub fn new(root_element: &str) -> Self {
        Self {
            root_element: root_element.to_string(),
        }
    }
}

#[async_trait]
impl Herald for XmlHerald {
    async fn format(&self, content: &str) -> Result<String, HeraldError> {
        let mut writer = Writer::new(Cursor::new(Vec::new()));
        
        // Write XML declaration
        writer.write_event(quick_xml::events::Event::Decl(
            quick_xml::events::BytesDecl::new("1.0", Some("UTF-8"), None)
        ))?;
        
        // Parse content as structured data
        let data: serde_json::Value = serde_json::from_str(content)
            .map_err(|e| HeraldError::FormatError(e.to_string()))?;
        
        // Convert to XML
        self.json_to_xml(&mut writer, &self.root_element, &data)?;
        
        let xml_bytes = writer.into_inner().into_inner();
        Ok(String::from_utf8(xml_bytes)?)
    }
    
    fn validate(&self, content: &str) -> Result<(), HeraldError> {
        // Validate JSON structure
        serde_json::from_str::<serde_json::Value>(content)
            .map(|_| ())
            .map_err(|e| HeraldError::ValidationError(e.to_string()))
    }
    
    fn metadata(&self) -> FormatMetadata {
        FormatMetadata {
            format_name: "xml".to_string(),
            mime_type: "application/xml".to_string(),
            file_extension: "xml".to_string(),
            supports_streaming: false,
        }
    }
}

// Usage
let herald = Arc::new(XmlHerald::new("response"));

let paladin = PaladinBuilder::new(llm_adapter)
    .system_prompt("Return JSON that will be converted to XML")
    .with_herald(herald)
    .build()?;
```

### CSV Herald

```rust
use paladin::herald::*;
use csv::Writer;

pub struct CsvHerald {
    headers: Vec<String>,
    delimiter: u8,
}

impl CsvHerald {
    pub fn new(headers: Vec<String>) -> Self {
        Self {
            headers,
            delimiter: b',',
        }
    }
    
    pub fn with_delimiter(mut self, delimiter: u8) -> Self {
        self.delimiter = delimiter;
        self
    }
}

#[async_trait]
impl Herald for CsvHerald {
    async fn format(&self, content: &str) -> Result<String, HeraldError> {
        // Parse JSON array
        let rows: Vec<serde_json::Value> = serde_json::from_str(content)
            .map_err(|e| HeraldError::FormatError(e.to_string()))?;
        
        let mut wtr = Writer::from_writer(vec![]);
        
        // Write headers
        wtr.write_record(&self.headers)?;
        
        // Write data rows
        for row in rows {
            let record: Vec<String> = self.headers.iter()
                .map(|h| {
                    row.get(h)
                        .map(|v| v.to_string())
                        .unwrap_or_default()
                })
                .collect();
            
            wtr.write_record(&record)?;
        }
        
        wtr.flush()?;
        let csv_bytes = wtr.into_inner()?;
        Ok(String::from_utf8(csv_bytes)?)
    }
    
    fn validate(&self, content: &str) -> Result<(), HeraldError> {
        // Validate JSON array structure
        let _: Vec<serde_json::Value> = serde_json::from_str(content)
            .map_err(|e| HeraldError::ValidationError(e.to_string()))?;
        Ok(())
    }
    
    fn metadata(&self) -> FormatMetadata {
        FormatMetadata {
            format_name: "csv".to_string(),
            mime_type: "text/csv".to_string(),
            file_extension: "csv".to_string(),
            supports_streaming: false,
        }
    }
}

// Usage
let herald = Arc::new(CsvHerald::new(vec![
    "name".to_string(),
    "age".to_string(),
    "city".to_string(),
]));

let paladin = PaladinBuilder::new(llm_adapter)
    .system_prompt("Return data as JSON array of objects with name, age, city fields")
    .with_herald(herald)
    .build()?;

let response = paladin.execute("Generate 5 sample user records").await?;
// Output is formatted CSV
```

## Streaming Output

Process and format output in real-time for better user experience.

### Basic Streaming

```rust
use paladin::herald::*;
use futures::StreamExt;

let herald = Arc::new(MarkdownHerald::default());

let paladin = PaladinBuilder::new(llm_adapter)
    .with_herald(herald.clone())
    .build()?;

// Execute with streaming
let mut stream = paladin.execute_stream("Write a story").await?;

while let Some(chunk) = stream.next().await {
    let chunk = chunk?;
    
    // Format chunk
    let formatted = herald.format_chunk(&chunk.content).await?;
    
    // Print in real-time
    print!("{}", formatted);
    std::io::stdout().flush()?;
}
println!();
```

### Streaming with Accumulation

```rust
pub struct StreamAccumulator {
    herald: Arc<dyn Herald>,
    buffer: String,
}

impl StreamAccumulator {
    pub fn new(herald: Arc<dyn Herald>) -> Self {
        Self {
            herald,
            buffer: String::new(),
        }
    }
    
    pub async fn process_chunk(&mut self, chunk: &str) -> Result<String, HeraldError> {
        self.buffer.push_str(chunk);
        
        // Format accumulated content
        self.herald.format(&self.buffer).await
    }
    
    pub fn buffer(&self) -> &str {
        &self.buffer
    }
}

// Usage
let mut accumulator = StreamAccumulator::new(herald);
let mut stream = paladin.execute_stream("Explain quantum computing").await?;

while let Some(chunk) = stream.next().await {
    let chunk = chunk?;
    let formatted_so_far = accumulator.process_chunk(&chunk.content).await?;
    
    // Update UI with fully formatted content
    update_ui(&formatted_so_far);
}
```

### Progress Indicators

```rust
pub struct ProgressHerald {
    inner: Arc<dyn Herald>,
    show_progress: bool,
}

#[async_trait]
impl Herald for ProgressHerald {
    async fn format_chunk(&self, chunk: &str) -> Result<String, HeraldError> {
        let formatted = self.inner.format_chunk(chunk).await?;
        
        if self.show_progress {
            // Add visual progress indicator
            Ok(format!("{} .", formatted))
        } else {
            Ok(formatted)
        }
    }
    
    async fn format(&self, content: &str) -> Result<String, HeraldError> {
        self.inner.format(content).await
    }
    
    fn validate(&self, content: &str) -> Result<(), HeraldError> {
        self.inner.validate(content)
    }
    
    fn metadata(&self) -> FormatMetadata {
        self.inner.metadata()
    }
}
```

## Multi-Format Output

Generate output in multiple formats simultaneously.

### Multi-Format Herald

```rust
pub struct MultiFormatHerald {
    heralds: HashMap<String, Arc<dyn Herald>>,
}

impl MultiFormatHerald {
    pub fn new() -> Self {
        Self {
            heralds: HashMap::new(),
        }
    }
    
    pub fn add_format(mut self, name: &str, herald: Arc<dyn Herald>) -> Self {
        self.heralds.insert(name.to_string(), herald);
        self
    }
    
    pub async fn format_all(&self, content: &str) -> Result<HashMap<String, String>, HeraldError> {
        let mut results = HashMap::new();
        
        for (name, herald) in &self.heralds {
            let formatted = herald.format(content).await?;
            results.insert(name.clone(), formatted);
        }
        
        Ok(results)
    }
}

// Usage
let multi_herald = MultiFormatHerald::new()
    .add_format("json", Arc::new(JsonHerald::default()))
    .add_format("markdown", Arc::new(MarkdownHerald::default()))
    .add_format("html", Arc::new(HtmlHerald::default()));

let paladin = PaladinBuilder::new(llm_adapter).build()?;
let response = paladin.execute("Summarize Rust features").await?;

// Generate all formats
let all_formats = multi_herald.format_all(&response.content).await?;

// Save or serve each format
std::fs::write("output.json", &all_formats["json"])?;
std::fs::write("output.md", &all_formats["markdown"])?;
std::fs::write("output.html", &all_formats["html"])?;
```

### Adaptive Format Selection

```rust
pub struct AdaptiveHerald {
    formats: HashMap<String, Arc<dyn Herald>>,
    default: Arc<dyn Herald>,
}

impl AdaptiveHerald {
    pub async fn format_for_context(
        &self,
        content: &str,
        context: &OutputContext,
    ) -> Result<String, HeraldError> {
        let herald = self.select_herald(context);
        herald.format(content).await
    }
    
    fn select_herald(&self, context: &OutputContext) -> &Arc<dyn Herald> {
        match context.channel {
            OutputChannel::Web => self.formats.get("html").unwrap_or(&self.default),
            OutputChannel::Api => self.formats.get("json").unwrap_or(&self.default),
            OutputChannel::Terminal => self.formats.get("markdown").unwrap_or(&self.default),
            OutputChannel::File(ref ext) => {
                self.formats.get(ext.as_str()).unwrap_or(&self.default)
            }
        }
    }
}

pub struct OutputContext {
    pub channel: OutputChannel,
    pub user_preferences: HashMap<String, String>,
}

pub enum OutputChannel {
    Web,
    Api,
    Terminal,
    File(String),
}

// Usage
let adaptive = AdaptiveHerald::new()
    .with_format("html", Arc::new(HtmlHerald::default()))
    .with_format("json", Arc::new(JsonHerald::default()))
    .with_format("markdown", Arc::new(MarkdownHerald::default()))
    .with_default(Arc::new(PlainHerald::default()));

// Format based on context
let web_output = adaptive.format_for_context(
    &content,
    &OutputContext {
        channel: OutputChannel::Web,
        user_preferences: HashMap::new(),
    }
).await?;

let api_output = adaptive.format_for_context(
    &content,
    &OutputContext {
        channel: OutputChannel::Api,
        user_preferences: HashMap::new(),
    }
).await?;
```

## Post-Processing

Transform or enhance output after formatting.

### Sanitization Herald

```rust
pub struct SanitizingHerald {
    inner: Arc<dyn Herald>,
    remove_patterns: Vec<regex::Regex>,
}

impl SanitizingHerald {
    pub fn new(inner: Arc<dyn Herald>) -> Self {
        Self {
            inner,
            remove_patterns: vec![
                // Remove potential PII
                regex::Regex::new(r"\b\d{3}-\d{2}-\d{4}\b").unwrap(),  // SSN
                regex::Regex::new(r"\b[\w\.-]+@[\w\.-]+\.\w+\b").unwrap(),  // Email
                regex::Regex::new(r"\b\d{3}-\d{3}-\d{4}\b").unwrap(),  // Phone
            ],
        }
    }
}

#[async_trait]
impl Herald for SanitizingHerald {
    async fn format(&self, content: &str) -> Result<String, HeraldError> {
        let formatted = self.inner.format(content).await?;
        
        // Remove sensitive patterns
        let mut sanitized = formatted;
        for pattern in &self.remove_patterns {
            sanitized = pattern.replace_all(&sanitized, "[REDACTED]").to_string();
        }
        
        Ok(sanitized)
    }
    
    // Implement other methods...
}
```

### Enhancement Herald

```rust
pub struct EnhancingHerald {
    inner: Arc<dyn Herald>,
}

#[async_trait]
impl Herald for EnhancingHerald {
    async fn format(&self, content: &str) -> Result<String, HeraldError> {
        let formatted = self.inner.format(content).await?;
        
        // Add enhancements
        let enhanced = self.add_table_of_contents(&formatted);
        let enhanced = self.add_footnotes(&enhanced);
        let enhanced = self.add_timestamps(&enhanced);
        
        Ok(enhanced)
    }
    
    fn add_table_of_contents(&self, content: &str) -> String {
        // Extract headers and generate TOC
        let headers = self.extract_headers(content);
        
        if headers.is_empty() {
            return content.to_string();
        }
        
        let toc = headers.iter()
            .map(|(level, text, id)| {
                let indent = "  ".repeat(*level - 1);
                format!("{}* [{}](#{})", indent, text, id)
            })
            .collect::<Vec<_>>()
            .join("\n");
        
        format!("## Table of Contents\n\n{}\n\n{}", toc, content)
    }
    
    fn add_footnotes(&self, content: &str) -> String {
        // Process [^1] style footnote references
        // Implementation...
        content.to_string()
    }
    
    fn add_timestamps(&self, content: &str) -> String {
        format!("Generated at: {}\n\n{}", chrono::Utc::now().to_rfc3339(), content)
    }
}
```

### Caching Herald

```rust
use std::collections::HashMap;
use std::sync::RwLock;

pub struct CachingHerald {
    inner: Arc<dyn Herald>,
    cache: RwLock<HashMap<String, String>>,
    max_cache_size: usize,
}

#[async_trait]
impl Herald for CachingHerald {
    async fn format(&self, content: &str) -> Result<String, HeraldError> {
        // Check cache
        {
            let cache = self.cache.read().unwrap();
            if let Some(cached) = cache.get(content) {
                return Ok(cached.clone());
            }
        }
        
        // Format
        let formatted = self.inner.format(content).await?;
        
        // Store in cache
        {
            let mut cache = self.cache.write().unwrap();
            
            // Evict oldest if at capacity
            if cache.len() >= self.max_cache_size {
                if let Some(key) = cache.keys().next().cloned() {
                    cache.remove(&key);
                }
            }
            
            cache.insert(content.to_string(), formatted.clone());
        }
        
        Ok(formatted)
    }
    
    // Implement other methods...
}
```

## Best Practices

### 1. Match Format to Use Case

```rust
// ✅ API endpoints - use JSON
let api_herald = Arc::new(JsonHerald::new()
    .with_schema(api_schema)
    .validate_output(true)
);

// ✅ Documentation - use Markdown
let docs_herald = Arc::new(MarkdownHerald::new()
    .with_table_of_contents(true)
    .with_code_highlighting(true)
);

// ✅ Web display - use HTML
let web_herald = Arc::new(HtmlHerald::new()
    .with_css_framework(CssFramework::Bootstrap)
    .with_responsive_design(true)
);

// ✅ Data export - use CSV
let export_herald = Arc::new(CsvHerald::new(headers));
```

### 2. Validate Structured Output

```rust
let herald = Arc::new(JsonHerald::new()
    .with_schema(schema)
    .validate_output(true)  // Validate against schema
);

// Paladin will retry if output doesn't match schema
let paladin = PaladinBuilder::new(llm_adapter)
    .system_prompt("CRITICAL: Output MUST be valid JSON matching the schema")
    .with_herald(herald)
    .max_retries(3)  // Retry on validation failures
    .build()?;
```

### 3. Use Streaming for Long Responses

```rust
// ❌ Bad: Wait for complete response
let response = paladin.execute(long_prompt).await?;
println!("{}", response.content);  // User waits 30 seconds

// ✅ Good: Stream for immediate feedback
let mut stream = paladin.execute_stream(long_prompt).await?;
while let Some(chunk) = stream.next().await {
    let chunk = chunk?;
    print!("{}", chunk.content);  // Immediate output
    std::io::stdout().flush()?;
}
```

### 4. Layer Heralds for Composability

```rust
// Layer: Base -> Enhancement -> Sanitization -> Caching
let herald = Arc::new(
    CachingHerald::new(
        Arc::new(SanitizingHerald::new(
            Arc::new(EnhancingHerald::new(
                Arc::new(MarkdownHerald::default())
            ))
        )),
        100,  // cache size
    )
);
```

### 5. Provide Format Guidance in System Prompt

```rust
// ✅ Explicit format instructions
let paladin = PaladinBuilder::new(llm_adapter)
    .system_prompt(
        "You MUST respond in valid JSON format:\n\
         {\n\
           \"answer\": \"your response\",\n\
           \"confidence\": 0.0 to 1.0,\n\
           \"sources\": [\"source1\", \"source2\"]\n\
         }\n\
         Do NOT include any text outside this JSON structure."
    )
    .with_herald(Arc::new(JsonHerald::default()))
    .build()?;
```

## Advanced Patterns

### Template-Based Herald

```rust
use handlebars::Handlebars;

pub struct TemplateHerald {
    handlebars: Handlebars<'static>,
    template_name: String,
}

impl TemplateHerald {
    pub fn new(template: &str, template_name: &str) -> Result<Self, HeraldError> {
        let mut handlebars = Handlebars::new();
        handlebars.register_template_string(template_name, template)?;
        
        Ok(Self {
            handlebars,
            template_name: template_name.to_string(),
        })
    }
}

#[async_trait]
impl Herald for TemplateHerald {
    async fn format(&self, content: &str) -> Result<String, HeraldError> {
        // Parse content as JSON
        let data: serde_json::Value = serde_json::from_str(content)?;
        
        // Render template
        let rendered = self.handlebars.render(&self.template_name, &data)?;
        
        Ok(rendered)
    }
    
    // Implement other methods...
}

// Usage
let template = r#"
# {{title}}

**Summary:** {{summary}}

## Details

{{#each items}}
- {{this}}
{{/each}}

*Generated: {{timestamp}}*
"#;

let herald = Arc::new(TemplateHerald::new(template, "report")?);

let paladin = PaladinBuilder::new(llm_adapter)
    .system_prompt("Return JSON: {title, summary, items: [], timestamp}")
    .with_herald(herald)
    .build()?;
```

### Diff Herald

```rust
pub struct DiffHerald {
    previous_content: RwLock<Option<String>>,
}

#[async_trait]
impl Herald for DiffHerald {
    async fn format(&self, content: &str) -> Result<String, HeraldError> {
        let previous = self.previous_content.read().unwrap().clone();
        
        let formatted = if let Some(prev) = previous {
            // Generate diff
            self.generate_diff(&prev, content)
        } else {
            // First time, show all
            content.to_string()
        };
        
        // Update previous content
        *self.previous_content.write().unwrap() = Some(content.to_string());
        
        Ok(formatted)
    }
    
    fn generate_diff(&self, old: &str, new: &str) -> String {
        // Use diff algorithm
        // Implementation...
        format!("--- Old\n+++ New\n{}", new)
    }
}
```

## Troubleshooting

### Invalid JSON Output

**Problem**: JSON Herald fails to parse LLM output.

**Solutions**:
1. Strengthen system prompt with explicit JSON instructions
2. Add JSON schema to prompt
3. Enable output validation with retries
4. Use JSON mode in LLM if supported

```rust
let paladin = PaladinBuilder::new(llm_adapter)
    .system_prompt(
        "CRITICAL INSTRUCTION: You MUST respond with ONLY valid JSON. \
         No additional text before or after. No markdown code blocks. \
         Just pure JSON.\n\n\
         Schema: {\"result\": string, \"confidence\": number}"
    )
    .output_format(OutputFormat::Json)  // Some LLMs support JSON mode
    .max_retries(3)
    .build()?;
```

### Streaming Format Inconsistency

**Problem**: Streamed chunks don't format correctly.

**Solutions**:
1. Use accumulation pattern
2. Implement chunk boundary detection
3. Buffer until complete format units

```rust
pub struct BufferedStreamHerald {
    buffer: RwLock<String>,
    delimiter: String,
}

impl BufferedStreamHerald {
    async fn format_chunk(&self, chunk: &str) -> Result<String, HeraldError> {
        let mut buffer = self.buffer.write().unwrap();
        buffer.push_str(chunk);
        
        // Check for complete units (e.g., sentences, paragraphs)
        if buffer.ends_with(&self.delimiter) {
            let complete = buffer.clone();
            buffer.clear();
            Ok(complete)
        } else {
            Ok(String::new())  // Not ready yet
        }
    }
}
```

### Performance Issues with Complex Formatting

**Problem**: Formatting is slow for large outputs.

**Solutions**:
1. Implement caching
2. Use lazy formatting (format on demand)
3. Optimize regex patterns
4. Consider parallel processing

```rust
// Lazy formatting
pub struct LazyHerald {
    inner: Arc<dyn Herald>,
    cached_result: RwLock<Option<String>>,
}

impl LazyHerald {
    pub async fn get_formatted(&self, content: &str) -> Result<String, HeraldError> {
        // Check cache
        if let Some(cached) = self.cached_result.read().unwrap().as_ref() {
            return Ok(cached.clone());
        }
        
        // Format and cache
        let formatted = self.inner.format(content).await?;
        *self.cached_result.write().unwrap() = Some(formatted.clone());
        
        Ok(formatted)
    }
}
```

## Testing

### Unit Testing Heralds

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_json_herald_formats_correctly() {
        let herald = JsonHerald::default();
        
        let input = r#"{"name": "Alice", "age": 30}"#;
        let formatted = herald.format(input).await.unwrap();
        
        // Verify valid JSON
        let parsed: serde_json::Value = serde_json::from_str(&formatted).unwrap();
        assert_eq!(parsed["name"], "Alice");
        assert_eq!(parsed["age"], 30);
    }
    
    #[tokio::test]
    async fn test_json_herald_validates_schema() {
        let schema = json!({
            "type": "object",
            "properties": {
                "name": {"type": "string"}
            },
            "required": ["name"]
        });
        
        let herald = JsonHerald::new().with_schema(schema);
        
        // Valid
        assert!(herald.validate(r#"{"name": "Bob"}"#).is_ok());
        
        // Invalid - missing required field
        assert!(herald.validate(r#"{"age": 25}"#).is_err());
    }
}
```

## Examples

See working examples:
- `examples/herald_markdown_output.rs` - Markdown formatting
- `examples/herald_json_output.rs` - Structured JSON output
- `examples/herald_streaming.rs` - Real-time streaming
- `examples/herald_custom_formatter.rs` - Custom herald implementation

## Next Steps

- **[Tool Integration](tool-integration.md)** - Format tool results
- **[Battalion Patterns](battalion-patterns.md)** - Format multi-agent outputs
- **[API Reference](https://docs.rs/paladin)** - Herald API documentation

## Related Resources

- [Handlebars Templates](https://handlebarsjs.com/)
- [JSON Schema](https://json-schema.org/)
- [Markdown Specification](https://commonmark.org/)
