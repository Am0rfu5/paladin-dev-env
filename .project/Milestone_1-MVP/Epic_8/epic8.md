## Epic 8: Herald Output Formatting

### Overview

**Priority:** Low  
**Effort:** 1-2 weeks  
**Dependencies:** Epic 1  
**Team:** 1 developer

**Objective:** Implement the Herald formatting system for structured output from Paladins and Battalions.

### Technical Design

**herald.rs - Output Formatters**

```rust
pub trait Herald: Send + Sync {
    fn format_paladin_result(&self, result: &PaladinResult) -> String;
    fn format_battalion_result(&self, result: &BattalionResult) -> String;
}

pub struct MarkdownHerald;
pub struct JsonHerald;
pub struct TableHerald;

impl Herald for MarkdownHerald {
    fn format_paladin_result(&self, result: &PaladinResult) -> String {
        format!(
            "## Paladin: {}\n\n**Status:** {}\n\n### Output\n\n{}\n\n### Metadata\n\n- Loops: {}\n- Tokens: {}\n",
            result.paladin_name,
            result.status,
            result.output,
            result.loop_count,
            result.token_usage.total_tokens
        )
    }
}
```

### Acceptance Criteria

- [ ] Results can be formatted as Markdown, JSON, or tables
- [ ] Metadata included in formatted output
- [ ] Battalion results show individual Paladin contributions

---
