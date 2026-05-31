# Product Requirements Document: Herald & Domain Type Consolidation

## Document Information

- **Epic:** 19 - Herald & Domain Type Consolidation
- **Version:** 1.0
- **Created:** February 9, 2026
- **Status:** Draft
- **Duration:** 1-2 weeks
- **Priority:** Critical
- **Dependencies:** None

---

## Introduction/Overview

During Epic 8 (Herald Output Formatting), several domain types in `herald.rs` were defined as placeholder structs with `TODO` comments. These placeholders were created because the actual domain types from Epics 1 (Paladin) and 4 (Battalion) were not yet implemented. Now that the full multi-agent system is complete, these placeholder types must be consolidated with the actual domain entities to eliminate duplication and ensure type consistency across the framework.

Additionally, the Herald registry requires completion to auto-register built-in formatters (JSON, Markdown, Table), making the output formatting system immediately usable without manual configuration.

**Problem Statement:** The codebase currently contains duplicate type definitions for `PaladinResult`, `BattalionResult`, and `PaladinError`. Incomplete structures (`StreamChunk`, `ExecutionMetadata`) lack full telemetry capabilities. The Herald system requires manual formatter registration, creating friction for developers.

**Solution:** Replace all placeholder types with imports from actual domain modules, complete the streaming and metadata structures with extensible designs, and implement auto-registration of built-in formatters.

---

## Goals

1. **Eliminate Type Duplication:** Remove all placeholder types from `herald.rs` and establish single source of truth for domain types
2. **Complete Streaming Infrastructure:** Finish `StreamChunk` and `ExecutionMetadata` structures with full telemetry fields
3. **Enable Zero-Config Herald Usage:** Auto-register built-in formatters so developers can use Herald immediately
4. **Maintain Type Safety:** Ensure all type consolidations preserve compile-time safety and error handling
5. **Design for Extensibility:** Structure streaming and metadata types to accommodate future telemetry additions
6. **Achieve 100% Test Coverage:** Follow TDD methodology for all changes with comprehensive test suites

---

## User Stories

### US-19.1: Consolidate Herald Domain Types

**As a** framework developer  
**I want** Herald to use the actual domain result and error types  
**So that** there are no duplicate or placeholder structs in the codebase

**Acceptance Criteria:**
- [ ] Remove placeholder `PaladinResult` from `src/core/platform/container/herald.rs` (line 147)
- [ ] Remove placeholder `BattalionResult` from `src/core/platform/container/herald.rs` (line 158)
- [ ] Remove placeholder `PaladinError` from `src/core/platform/container/herald.rs` (line 187)
- [ ] Replace with imports from actual domain types:
  - `PaladinResult` from `src/core/platform/container/paladin.rs`
  - `BattalionResult` from `src/core/platform/container/battalion/mod.rs`
  - `PaladinError` from `src/core/platform/container/paladin.rs`
- [ ] Complete `StreamChunk` structure with full streaming metadata (line 169):
  - `chunk_id`: Unique identifier for the chunk
  - `sequence_number`: Order in stream
  - `timestamp`: When chunk was generated
  - `token_count`: Approximate tokens in chunk
  - `is_final`: Whether this is the last chunk
  - `metadata`: Extensible map for future fields
- [ ] Complete `ExecutionMetadata` structure with full telemetry fields (line 178):
  - `execution_id`: Unique identifier for execution
  - `start_time`: Execution start timestamp
  - `end_time`: Execution completion timestamp
  - `duration_ms`: Total execution time
  - `model_used`: LLM model identifier
  - `token_usage`: Input/output token counts
  - `cost_estimate`: Estimated API cost
  - `error_count`: Number of retries/errors
  - `metadata`: Extensible map for provider-specific data
- [ ] Update all Herald trait implementations to use consolidated types
- [ ] Update `HeraldPort` trait in `src/application/ports/output/herald_port.rs`
- [ ] Update all Herald adapters (`JsonHerald`, `MarkdownHerald`, `TableHerald`)
- [ ] All existing Herald tests continue to pass
- [ ] No duplicate type definitions remain (verified via grep/search)

**Source Files:**
- `src/core/platform/container/herald.rs` — lines 147, 158, 169, 178, 187
- `src/core/platform/container/paladin.rs` — actual domain types
- `src/core/platform/container/battalion/mod.rs` — actual domain types

---

### US-19.2: Register Built-in Herald Formatters

**As a** developer  
**I want** built-in formatters auto-registered in the Herald registry  
**So that** JSON, Markdown, and Table formatting work out of the box without manual configuration

**Acceptance Criteria:**
- [ ] `HeraldRegistry::default()` auto-registers three built-in formatters:
  - `JsonHerald` with key "json"
  - `MarkdownHerald` with key "markdown"
  - `TableHerald` with key "table"
- [ ] Formatters are immediately retrievable via `registry.get("json")`, etc.
- [ ] Registry supports additional custom formatter registration
- [ ] Unit tests verify:
  - All three formatters present after `HeraldRegistry::default()`
  - Each formatter is retrievable by correct key
  - Custom formatters can still be added
  - Duplicate keys are handled appropriately (error or overwrite)
- [ ] Documentation updated with zero-config usage example

**Source Files:**
- `src/application/use_cases/herald/herald_registry.rs` — line 186

---

## Functional Requirements

### FR-1: Type Consolidation
1. **FR-1.1:** Remove all placeholder type definitions from `herald.rs`
2. **FR-1.2:** Add imports for actual types from `paladin.rs` and `battalion/mod.rs`
3. **FR-1.3:** Ensure no compilation errors after type replacement
4. **FR-1.4:** Verify all trait implementations use consolidated types
5. **FR-1.5:** Update trait bounds and generic constraints as needed

### FR-2: StreamChunk Completion
1. **FR-2.1:** Define complete `StreamChunk` struct with all required fields
2. **FR-2.2:** Implement `Debug`, `Clone`, `Serialize`, `Deserialize` traits
3. **FR-2.3:** Add `metadata: HashMap<String, Value>` for extensibility
4. **FR-2.4:** Provide builder pattern for constructing chunks
5. **FR-2.5:** Add validation for required fields

### FR-3: ExecutionMetadata Completion
1. **FR-3.1:** Define complete `ExecutionMetadata` struct with all telemetry fields
2. **FR-3.2:** Implement `Debug`, `Clone`, `Serialize`, `Deserialize` traits
3. **FR-3.3:** Add `metadata: HashMap<String, Value>` for provider-specific data
4. **FR-3.4:** Provide builder pattern for constructing metadata
5. **FR-3.5:** Calculate `duration_ms` automatically from start/end times
6. **FR-3.6:** Add helper methods for token usage analysis

### FR-4: Formatter Auto-Registration
1. **FR-4.1:** Implement `Default` trait for `HeraldRegistry`
2. **FR-4.2:** Auto-register `JsonHerald` in default constructor
3. **FR-4.3:** Auto-register `MarkdownHerald` in default constructor
4. **FR-4.4:** Auto-register `TableHerald` in default constructor
5. **FR-4.5:** Maintain existing manual registration API
6. **FR-4.6:** Document formatter keys in rustdoc

### FR-5: Testing (TDD Approach)
1. **FR-5.1:** Write failing tests before implementing each change
2. **FR-5.2:** Unit tests for type consolidation (compilation tests)
3. **FR-5.3:** Unit tests for `StreamChunk` with all fields
4. **FR-5.4:** Unit tests for `ExecutionMetadata` with all fields
5. **FR-5.5:** Unit tests for default registry with auto-registration
6. **FR-5.6:** Integration tests for full Herald pipeline with consolidated types
7. **FR-5.7:** Serialization/deserialization tests for extensible metadata fields
8. **FR-5.8:** Tests for builder patterns and validation

---

## Non-Goals (Out of Scope)

1. **Performance Optimization:** This epic focuses on correctness and consolidation, not performance improvements
2. **Additional Formatters:** No new formatter types (XML, YAML, etc.) will be added
3. **Streaming Protocol Changes:** The streaming mechanism itself is not being redesigned
4. **Herald Configuration System:** No new configuration options beyond auto-registration
5. **Backward Compatibility Shims:** Since this is internal refactoring, no compatibility layer is needed
6. **External API Changes:** Public-facing Herald APIs remain unchanged (internal refactor only)
7. **Database Schema Changes:** No changes to how results are persisted
8. **UI/Frontend Changes:** This is purely backend consolidation

---

## Design Considerations

### Type Consolidation Pattern
```rust
// BEFORE (herald.rs) - REMOVE THESE
pub struct PaladinResult { /* ... */ }  // Line 147
pub struct BattalionResult { /* ... */ } // Line 158
pub struct PaladinError { /* ... */ }    // Line 187

// AFTER - USE ACTUAL TYPES
use crate::core::platform::container::paladin::{PaladinResult, PaladinError};
use crate::core::platform::container::battalion::BattalionResult;
```

### StreamChunk Design (Extensible)
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamChunk {
    /// Unique identifier for this chunk
    pub chunk_id: Uuid,
    /// Sequence number in the stream (0-indexed)
    pub sequence_number: u64,
    /// Timestamp when chunk was generated
    pub timestamp: DateTime<Utc>,
    /// Content of this chunk
    pub content: String,
    /// Approximate token count in this chunk
    pub token_count: Option<u32>,
    /// Whether this is the final chunk in the stream
    pub is_final: bool,
    /// Extensible metadata for future fields
    #[serde(flatten)]
    pub metadata: HashMap<String, serde_json::Value>,
}

impl StreamChunk {
    pub fn builder() -> StreamChunkBuilder { /* ... */ }
}
```

### ExecutionMetadata Design (Extensible)
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionMetadata {
    /// Unique identifier for this execution
    pub execution_id: Uuid,
    /// Execution start time
    pub start_time: DateTime<Utc>,
    /// Execution end time
    pub end_time: Option<DateTime<Utc>>,
    /// Total duration in milliseconds
    pub duration_ms: Option<u64>,
    /// LLM model used
    pub model_used: String,
    /// Token usage statistics
    pub token_usage: TokenUsage,
    /// Estimated API cost in USD
    pub cost_estimate: Option<f64>,
    /// Number of errors/retries during execution
    pub error_count: u32,
    /// Provider-specific extensible metadata
    #[serde(flatten)]
    pub metadata: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenUsage {
    pub input_tokens: u32,
    pub output_tokens: u32,
    pub total_tokens: u32,
}

impl ExecutionMetadata {
    pub fn builder() -> ExecutionMetadataBuilder { /* ... */ }

    /// Calculate duration from start/end times
    pub fn calculate_duration(&mut self) {
        if let Some(end) = self.end_time {
            self.duration_ms = Some((end - self.start_time).num_milliseconds() as u64);
        }
    }
}
```

### Auto-Registration Pattern
```rust
impl Default for HeraldRegistry {
    fn default() -> Self {
        let mut registry = Self::new();

        // Auto-register built-in formatters
        registry.register("json", Arc::new(JsonHerald::new()));
        registry.register("markdown", Arc::new(MarkdownHerald::new()));
        registry.register("table", Arc::new(TableHerald::new()));

        registry
    }
}
```

---

## Technical Considerations

### Hexagonal Architecture Compliance
- **Core Layer:** `herald.rs` remains in `core/platform/container/` (pure domain logic)
- **Application Layer:** `herald_registry.rs` and formatters in `application/use_cases/herald/`
- **No Infrastructure Dependencies:** Herald uses only domain types, no external adapters

### Type Safety
- All type replacements must maintain strong typing
- Generic trait bounds may need adjustment for consolidated types
- Ensure `Send + Sync` bounds preserved for async compatibility

### Serialization
- `StreamChunk` and `ExecutionMetadata` must be serializable for:
  - JSON formatting output
  - Citadel state persistence
  - Event streaming to external systems
- Use `#[serde(flatten)]` for extensible metadata maps

### Error Handling
- Consolidated `PaladinError` may have different error variants
- Ensure Herald error handling covers all variants
- Consider adding `HeraldError` if new failure modes emerge

### Testing Strategy (TDD)
1. **Write failing test** for each TODO item
2. **Implement minimal code** to pass the test
3. **Refactor** while keeping tests green
4. **Add edge case tests** after main functionality works

### Migration Notes
- This is **internal refactoring** — breaking changes acceptable
- No external API surface affected
- Developers using Herald API see no changes
- Only internal Herald implementation consumers need updates

---

## Success Metrics

### Immediate Success Criteria
1. **Zero Duplicate Types:** No remaining placeholder types in codebase (verified via `grep`)
2. **All Tests Pass:** 100% test suite passing with consolidated types
3. **Compilation Success:** `cargo build` succeeds with no warnings
4. **Clippy Clean:** `cargo clippy` reports zero warnings
5. **Auto-Registration Works:** `HeraldRegistry::default()` contains 3 formatters

### Code Quality Metrics
1. **Test Coverage:** ≥95% coverage for modified Herald modules
2. **Documentation Coverage:** 100% of public APIs have rustdoc
3. **Lines of Code Removed:** Net reduction due to placeholder removal
4. **Cyclomatic Complexity:** No increase in complexity metrics

### Functional Validation
1. **Existing Examples Run:** All Herald examples in `examples/herald_*.rs` execute successfully
2. **Serialization Round-Trip:** All types serialize/deserialize without data loss
3. **Extensible Metadata:** Can add custom fields to metadata maps without code changes
4. **Zero-Config Usage:** Developer can use Herald with 3 lines of code:
   ```rust
   let registry = HeraldRegistry::default();
   let formatter = registry.get("json").unwrap();
   let output = formatter.format(&result)?;
   ```

---

## Open Questions

### Resolved by User Selections
- ✅ **Testing Approach:** TDD with comprehensive coverage (Selection: 1D)
- ✅ **Backward Compatibility:** Breaking changes acceptable (Selection: 2A)
- ✅ **Implementation Order:** Flexible, either user story can go first (Selection: 3D)
- ✅ **Documentation Level:** Rustdoc updates and basic examples (Selection: 4B)
- ✅ **Extensibility Design:** Yes, design for future telemetry additions (Selection: 5A)

### Outstanding Questions
1. **Should `ExecutionMetadata` include memory usage metrics?** (Can be added via extensible metadata if needed later)
2. **Do we need a versioning scheme for serialized metadata?** (Recommendation: Add `version: u32` field)
3. **Should formatter registry support priorities/ordering?** (Current scope: No, defer to future epic)
4. **Is cost estimation provider-specific or should we standardize?** (Recommendation: Use extensible metadata for provider-specific calculations)

---

## Implementation Checklist

### Phase 1: Setup and Test Infrastructure (Week 1, Days 1-2)
- [ ] Create feature branch `feature/epic-19-herald-consolidation`
- [ ] Review all TODO locations in source files
- [ ] Write comprehensive test suite (TDD — failing tests first):
  - [ ] Type consolidation compilation tests
  - [ ] StreamChunk serialization tests
  - [ ] ExecutionMetadata calculation tests
  - [ ] Auto-registration tests
  - [ ] Integration tests for full Herald pipeline

### Phase 2: Type Consolidation (Week 1, Days 3-4)
- [ ] Remove placeholder `PaladinResult` from herald.rs (line 147)
- [ ] Remove placeholder `BattalionResult` from herald.rs (line 158)
- [ ] Remove placeholder `PaladinError` from herald.rs (line 187)
- [ ] Add imports for actual domain types
- [ ] Update Herald trait implementations
- [ ] Run tests (should now pass)
- [ ] Verify no duplicate types remain (`grep -r "struct PaladinResult"`)

### Phase 3: Complete StreamChunk (Week 1, Days 4-5)
- [ ] Define complete `StreamChunk` struct with all fields
- [ ] Implement required traits (Debug, Clone, Serialize, Deserialize)
- [ ] Add builder pattern
- [ ] Add extensible metadata field
- [ ] Update all StreamChunk usage sites
- [ ] Run tests (should pass)

### Phase 4: Complete ExecutionMetadata (Week 1, Days 5-Week 2, Day 1)
- [ ] Define complete `ExecutionMetadata` struct with all telemetry fields
- [ ] Implement required traits
- [ ] Add builder pattern
- [ ] Add `calculate_duration()` helper method
- [ ] Add extensible metadata field
- [ ] Update all ExecutionMetadata usage sites
- [ ] Run tests (should pass)

### Phase 5: Auto-Registration (Week 2, Days 1-2)
- [ ] Implement `Default` trait for `HeraldRegistry`
- [ ] Auto-register JsonHerald
- [ ] Auto-register MarkdownHerald
- [ ] Auto-register TableHerald
- [ ] Update documentation with zero-config example
- [ ] Run tests (should pass)

### Phase 6: Documentation and Examples (Week 2, Days 2-3)
- [ ] Update rustdoc for all modified types
- [ ] Add usage examples to rustdoc
- [ ] Update existing Herald examples if needed
- [ ] Document extensible metadata patterns
- [ ] Update CHANGELOG.md

### Phase 7: Quality Assurance (Week 2, Days 3-4)
- [ ] Run full test suite: `cargo test`
- [ ] Run integration tests: `make test-all`
- [ ] Check formatting: `cargo fmt --check`
- [ ] Run linter: `cargo clippy -- -D warnings`
- [ ] Verify test coverage ≥95% for Herald modules
- [ ] Run benchmarks if available
- [ ] Manual testing of Herald examples

### Phase 8: Completion (Week 2, Day 4-5)
- [ ] Address any remaining clippy warnings
- [ ] Final code review
- [ ] Merge to main branch
- [ ] Mark Epic 19 as complete in project plan

---

## Dependencies

### Required Before Starting
- None — Epic 19 has no dependencies on other epics

### Enables Future Work
- **Epic 20:** Vision Pipeline uses consolidated `PaladinResult`
- **Epic 21:** Autonomous Agent uses `ExecutionMetadata` for planning context
- **Epic 22:** Battalion patterns use consolidated `BattalionResult`

---

## Risk Assessment

| Risk | Likelihood | Impact | Mitigation |
|------|-----------|--------|------------|
| Breaking existing Herald usage | Low | Medium | Comprehensive test suite, TDD approach |
| Metadata field conflicts | Low | Low | Use namespaced keys in extensible metadata |
| Performance regression from extensible maps | Low | Low | Benchmark before/after, metadata is optional |
| Incomplete type migration | Low | High | Grep verification step in checklist |

---

## Definition of Done

Epic 19 is complete when:
1. ✅ All placeholder types removed from `herald.rs`
2. ✅ Actual domain types imported and used throughout
3. ✅ `StreamChunk` fully defined with extensible metadata
4. ✅ `ExecutionMetadata` fully defined with extensible metadata
5. ✅ Built-in formatters auto-registered in `HeraldRegistry::default()`
6. ✅ All tests pass (`cargo test`)
7. ✅ Code formatted (`cargo fmt`)
8. ✅ No clippy warnings (`cargo clippy -- -D warnings`)
9. ✅ Test coverage ≥95% for Herald modules
10. ✅ Documentation updated with rustdoc and examples
11. ✅ All Herald examples run successfully
12. ✅ Code merged to main branch
13. ✅ Epic 19 marked complete in project plan

---

## References

- **Project Plan:** `/project/Milestone_3-Completion/Project_Plan_Milestone_3.md`
- **Epic Definition:** `/project/Milestone_3-Completion/Epic_19/epic19.md`
- **Herald Implementation:** `src/core/platform/container/herald.rs`
- **Herald Registry:** `src/application/use_cases/herald/herald_registry.rs`
- **Domain Types:** `src/core/platform/container/paladin.rs`, `src/core/platform/container/battalion/mod.rs`
- **Hexagonal Architecture Guide:** `notes/hexagonal-arch.md`
- **Medieval Naming Convention:** `.github/copilot-instructions.md`
