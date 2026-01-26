# Task List: Herald Output Formatting System

**Epic:** Epic 8  
**Priority:** Low  
**Effort:** 1-2 weeks  
**Dependencies:** Epic 1 (Paladin Domain Foundation)  
**Created:** January 26, 2026

---

## Instructions for Completing Tasks

**IMPORTANT:** As you complete each task, you must check it off in this markdown file by changing `- [ ]` to `- [x]`. This helps track progress and ensures you don't skip any steps.

Example:
- `- [ ] 1.1 Read file` → `- [x] 1.1 Read file` (after completing)

Update the file after completing each sub-task, not just after completing an entire parent task.

---

## Relevant Files

### Core Layer
- `src/core/platform/container/herald.rs` - Herald trait definition and core types
- `src/core/platform/container/herald_error.rs` - Herald error types

### Application Layer
- `src/application/ports/output/herald_port.rs` - Herald port abstraction (if needed)
- `src/application/use_cases/herald/herald_registry.rs` - Herald registry service

### Infrastructure Layer
- `src/infrastructure/adapters/herald/mod.rs` - Herald adapters module
- `src/infrastructure/adapters/herald/json_herald.rs` - JSON formatter implementation
- `src/infrastructure/adapters/herald/markdown_herald.rs` - Markdown formatter implementation
- `src/infrastructure/adapters/herald/table_herald.rs` - Table formatter implementation

### Configuration
- `src/config/application_settings.rs` - Updated with Herald configuration
- `config.yml` - Herald configuration section
- `config.test.yml` - Test configuration for Herald

### Integration Points
- `src/application/use_cases/paladin/paladin_execution_service.rs` - Updated for Herald integration
- `src/application/use_cases/battalion/formation_service.rs` - Updated for Herald integration
- `src/application/use_cases/battalion/phalanx_service.rs` - Updated for Herald integration

### Tests
- `tests/unit/herald_tests.rs` - Unit tests for Herald trait and types
- `tests/unit/json_herald_tests.rs` - Unit tests for JSON formatter
- `tests/unit/markdown_herald_tests.rs` - Unit tests for Markdown formatter
- `tests/unit/table_herald_tests.rs` - Unit tests for Table formatter
- `tests/unit/herald_registry_tests.rs` - Unit tests for registry
- `tests/integration/herald_integration_tests.rs` - End-to-end Herald tests

### Examples
- `examples/herald_custom_formatter.rs` - Custom formatter implementation example
- `examples/herald_json_output.rs` - JSON formatter usage example
- `examples/herald_markdown_output.rs` - Markdown formatter usage example
- `examples/herald_streaming.rs` - Streaming formatter example

### Documentation
- `docs/HERALD.md` - User guide for Herald system

### Notes

- Unit tests in Rust go in `#[cfg(test)]` modules within the same file or in `tests/unit/`
- Integration tests go in `tests/integration/`
- Run tests with `cargo test`, format with `cargo fmt`, lint with `cargo clippy`
- All public items must have rustdoc comments (`///`)
- Follow hexagonal architecture: Core → Application → Infrastructure
- Herald trait must be `Send + Sync` for async compatibility

---

## Tasks

- [x] 0.0 Create feature branch
  - [x] 0.1 Create and checkout new branch `git checkout -b epic-8/herald-output-formatting`

- [x] 1.0 Set up core Herald infrastructure (trait, errors, types)
  - [x] 1.1 Create `src/core/platform/container/herald_error.rs` with HeraldError enum
  - [x] 1.2 Add error variants (SerializationError, TemplateError, InvalidResult, IoError)
  - [x] 1.3 Implement Display and Error traits using thiserror
  - [x] 1.4 Create `src/core/platform/container/herald.rs` with Herald trait definition
  - [x] 1.5 Define Herald trait methods (format_paladin_result, format_battalion_result, format_stream_chunk, finalize_stream, format_error, name, mime_type)
  - [x] 1.6 Add trait bounds (Send + Sync) for async compatibility
  - [x] 1.7 Add comprehensive rustdoc documentation for trait and all methods
  - [x] 1.8 Write unit tests for error type creation and formatting
  - [x] 1.9 Update `src/core/platform/container/mod.rs` to export herald module

- [x] 2.0 Implement Herald Registry system
  - [x] 2.1 Create `src/application/use_cases/herald/` directory
  - [x] 2.2 Create `src/application/use_cases/herald/herald_registry.rs`
  - [x] 2.3 Define HeraldRegistry struct with HashMap<String, Arc<dyn Herald>>
  - [x] 2.4 Implement register method for adding formatters
  - [x] 2.5 Implement get method for retrieving formatters by name
  - [x] 2.6 Implement list method for listing available formatter names
  - [x] 2.7 Add thread-safe concurrent access with RwLock
  - [x] 2.8 Implement Default trait to auto-register built-in formatters
  - [x] 2.9 Add comprehensive rustdoc documentation
  - [x] 2.10 Write unit tests for registry operations (register, get, list)
  - [x] 2.11 Update `src/application/use_cases/mod.rs` to export herald module

- [x] 3.0 Implement JsonHerald formatter
  - [x] 3.1 Add dependencies to Cargo.toml (serde_json already present, verify)
  - [x] 3.2 Create `src/infrastructure/adapters/herald/` directory
  - [x] 3.3 Create `src/infrastructure/adapters/herald/mod.rs`
  - [x] 3.4 Create `src/infrastructure/adapters/herald/json_herald.rs`
  - [x] 3.5 Define JsonHerald struct with configuration (pretty, include_metadata)
  - [x] 3.6 Implement Herald trait for JsonHerald
  - [x] 3.7 Implement format_paladin_result with JSON serialization
  - [x] 3.8 Implement format_battalion_result with nested JSON structure
  - [x] 3.9 Implement format_stream_chunk for streaming JSON (NDJSON or similar)
  - [x] 3.10 Implement finalize_stream to append metadata as JSON
  - [x] 3.11 Implement format_error with JSON error structure
  - [x] 3.12 Implement name() returning "json" and mime_type() returning "application/json"
  - [x] 3.13 Add comprehensive rustdoc with JSON schema examples
  - [x] 3.14 Write unit tests for all Herald trait methods
  - [x] 3.15 Write property test for format → parse → format roundtrip

- [x] 4.0 Implement MarkdownHerald formatter
  - [x] 4.1 Add dependencies to Cargo.toml (colored or ansi_term for colors - optional)
  - [x] 4.2 Create `src/infrastructure/adapters/herald/markdown_herald.rs`
  - [x] 4.3 Define MarkdownHerald struct with configuration (include_colors, heading_level)
  - [x] 4.4 Implement Herald trait for MarkdownHerald
  - [x] 4.5 Implement format_paladin_result with Markdown formatting (headings, bold, lists)
  - [x] 4.6 Add status badges/emojis (✅ Success, ❌ Failed, ⏱️ Timeout)
  - [x] 4.7 Implement format_battalion_result with nested sections
  - [x] 4.8 Implement format_stream_chunk for progressive Markdown output
  - [x] 4.9 Implement finalize_stream to append metadata section
  - [x] 4.10 Implement format_error with formatted error blocks
  - [x] 4.11 Implement name() returning "markdown" and mime_type() returning "text/markdown"
  - [x] 4.12 Add color support detection and configuration
  - [x] 4.13 Add comprehensive rustdoc with Markdown examples
  - [x] 4.14 Write unit tests for all Herald trait methods
  - [x] 4.15 Test color output with and without ANSI support

- [x] 5.0 Implement TableHerald formatter
  - [x] 5.1 Add dependencies to Cargo.toml (comfy-table or tabled)
  - [x] 5.2 Create `src/infrastructure/adapters/herald/table_herald.rs`
  - [x] 5.3 Define TableHerald struct with configuration (max_column_width, border_style)
  - [x] 5.4 Implement Herald trait for TableHerald
  - [x] 5.5 Implement format_paladin_result with ASCII table (columns: Field, Value)
  - [x] 5.6 Implement format_battalion_result with multi-row table for each Paladin
  - [x] 5.7 Implement format_stream_chunk (may return None until complete)
  - [x] 5.8 Implement finalize_stream to create final table with metadata
  - [x] 5.9 Implement format_error with table formatting
  - [x] 5.10 Implement name() returning "table" and mime_type() returning "text/plain"
  - [x] 5.11 Add configurable column widths and border styles
  - [x] 5.12 Add comprehensive rustdoc with table examples
  - [x] 5.13 Write unit tests for all Herald trait methods
  - [x] 5.14 Test with various border styles and column widths

- [x] 6.0 Add configuration integration
  - [x] 6.1 Read current `src/config/application_settings.rs` structure
  - [x] 6.2 Define HeraldConfig struct in application_settings.rs
  - [x] 6.3 Add fields (default_formatter, json, markdown, table configurations)
  - [x] 6.4 Implement Deserialize for all Herald config structs
  - [x] 6.5 Add Herald config to ApplicationSettings
  - [x] 6.6 Update `config.yml` with herald section and defaults
  - [x] 6.7 Update `config.test.yml` with test Herald configuration
  - [x] 6.8 Add environment variable overrides (HERALD_DEFAULT_FORMATTER)
  - [x] 6.9 Write configuration loading tests
  - [x] 6.10 Document configuration options in rustdoc

- [ ] 7.0 Integrate Herald with Paladin/Battalion execution
  - [x] 7.1 Read `src/application/use_cases/paladin/paladin_execution_service.rs`
  - [x] 7.2 Add optional herald parameter to PaladinExecutionService
  - [x] 7.3 Add format_result method to PaladinExecutionService
  - [x] 7.4 Implement global default Herald resolution from config
  - [x] 7.5 Implement runtime override support via Option<Arc<dyn Herald>>
  - [x] 7.6 Update PaladinBuilder to accept with_herald(herald) method
  - [x] 7.7 Read Battalion service files (formation, phalanx)
  - [x] 7.8 Add Herald support to FormationExecutionService
  - [x] 7.9 Add Herald support to PhalanxExecutionService
  - [x] 7.10 Implement format_result for BattalionResult
  - [x] 7.11 Ensure formatted output includes all metadata fields per FR-6 and FR-7
  - [x] 7.12 Write integration tests for Paladin with Herald
  - [ ] 7.13 Write integration tests for Battalion with Herald

- [ ] 8.0 Implement streaming support for formatters
  - [ ] 8.1 Review existing streaming implementation in PaladinExecutionService
  - [ ] 8.2 Define StreamChunk type if not already present
  - [ ] 8.3 Define ExecutionMetadata type for finalize_stream
  - [ ] 8.4 Implement format_stream_chunk in JsonHerald (NDJSON approach)
  - [ ] 8.5 Implement format_stream_chunk in MarkdownHerald (progressive text)
  - [ ] 8.6 Implement format_stream_chunk in TableHerald (buffer until complete)
  - [ ] 8.7 Implement finalize_stream in all formatters to append metadata
  - [ ] 8.8 Update streaming execution to call Herald methods
  - [ ] 8.9 Write streaming tests for each formatter
  - [ ] 8.10 Verify streaming output consistency with complete output

- [ ] 9.0 Add comprehensive testing suite
  - [ ] 9.1 Create `tests/unit/herald_tests.rs` for trait and types
  - [ ] 9.2 Create `tests/unit/json_herald_tests.rs` with ≥80% coverage
  - [ ] 9.3 Create `tests/unit/markdown_herald_tests.rs` with ≥80% coverage
  - [ ] 9.4 Create `tests/unit/table_herald_tests.rs` with ≥80% coverage
  - [ ] 9.5 Create `tests/unit/herald_registry_tests.rs` with full coverage
  - [ ] 9.6 Create `tests/integration/herald_integration_tests.rs`
  - [ ] 9.7 Write end-to-end test: Paladin execution with JSON Herald
  - [ ] 9.8 Write end-to-end test: Paladin execution with Markdown Herald
  - [ ] 9.9 Write end-to-end test: Battalion execution with Herald
  - [ ] 9.10 Write streaming consistency test per Acceptance Testing section
  - [ ] 9.11 Write runtime override test per Acceptance Testing section
  - [ ] 9.12 Create performance benchmark for formatter overhead
  - [ ] 9.13 Verify benchmark: JSON formatter < 1ms for 10KB results
  - [ ] 9.14 Verify benchmark: Markdown formatter < 2ms for 10KB results
  - [ ] 9.15 Run full test suite: `cargo test`
  - [ ] 9.16 Generate coverage report: `cargo llvm-cov` (if available)
  - [ ] 9.17 Verify ≥80% coverage for Herald module

- [ ] 10.0 Write documentation and examples
  - [ ] 10.1 Create `docs/HERALD.md` user guide
  - [ ] 10.2 Document Herald trait and how to implement custom formatters
  - [ ] 10.3 Document all built-in formatters (JSON, Markdown, Table)
  - [ ] 10.4 Document configuration options and defaults
  - [ ] 10.5 Document runtime override pattern
  - [ ] 10.6 Document streaming formatter behavior
  - [ ] 10.7 Create `examples/herald_json_output.rs` demonstrating JSON Herald
  - [ ] 10.8 Create `examples/herald_markdown_output.rs` demonstrating Markdown Herald
  - [ ] 10.9 Create `examples/herald_custom_formatter.rs` with XML or CSV example
  - [ ] 10.10 Create `examples/herald_streaming.rs` demonstrating streaming formatters
  - [ ] 10.11 Update existing examples to use Herald (basic_paladin.rs, etc.)
  - [ ] 10.12 Add Herald section to README.md
  - [ ] 10.13 Generate rustdoc: `cargo doc --open`
  - [ ] 10.14 Review all public API documentation for completeness
  - [ ] 10.15 Add Herald to project glossary and architecture docs

---

## Notes

- This task list follows the implementation phases outlined in the PRD (Phases 1-5)
- All tests must achieve ≥80% coverage per TDD requirements
- Follow hexagonal architecture: Core → Application → Infrastructure
- Use `cargo test` to run unit tests, `make test-all` for full test suite
- All public APIs must have rustdoc documentation
- Code must pass `cargo clippy` with no warnings and `cargo fmt --check`
- Follow completion protocol from process-task-list.md:
  - Mark sub-tasks `[x]` as completed
  - Run `cargo test`, `cargo fmt --check`, `cargo clippy` before committing parent tasks
  - Use conventional commit format with `-m` flags
  - Mark parent task `[x]` only when all sub-tasks complete and tests pass
