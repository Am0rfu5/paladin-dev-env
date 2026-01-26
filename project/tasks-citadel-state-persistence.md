# Task List: Citadel State Persistence (Epic 7)

**Epic:** Epic 7 - Citadel State Persistence  
**Priority:** Medium  
**Effort:** 2 weeks  
**Dependencies:** Epic 1 (Paladin Domain), Epic 2 (Garrison Memory)  
**PRD:** [prd-citadel-state-persistence.md](prd-citadel-state-persistence.md)

## Overview

Implement the Citadel persistence layer for automatic saving and restoration of Paladin agents and Battalion orchestrations. This system enables state persistence to the file system as JSON files, supporting autosave after execution completion and full state restoration for resuming workflows.

## Instructions for Completing Tasks

**IMPORTANT:** As you complete each task, you must check it off in this markdown file by changing `- [ ]` to `- [x]`. This helps track progress and ensures you don't skip any steps.

Example:
- `- [ ] 1.1 Read file` → `- [x] 1.1 Read file` (after completing)

Update the file after completing each sub-task, not just after completing an entire parent task.

## Relevant Files

- `src/core/platform/container/citadel.rs` - Domain types for state persistence (PaladinState, BattalionState, StateSummary)
- `src/application/ports/output/citadel_port.rs` - Port trait defining persistence operations
- `src/application/errors/citadel_error.rs` - Error types for Citadel operations
- `src/infrastructure/adapters/citadel/mod.rs` - Citadel adapter module exports
- `src/infrastructure/adapters/citadel/file_citadel.rs` - File system adapter implementation
- `src/application/use_cases/paladin/paladin_builder.rs` - Builder integration for autosave and restoration
- `src/config/application_settings.rs` - Configuration structures for Citadel settings
- `config.yml` - Default configuration values for state persistence
- `tests/integration/citadel_integration_test.rs` - Integration tests for file persistence
- `examples/citadel_autosave.rs` - Example demonstrating automatic state saving
- `examples/citadel_restore.rs` - Example demonstrating state restoration
- `examples/battalion_checkpoint_recovery.rs` - Example demonstrating Battalion resumption

### Notes

- Unit tests are placed inline within source files using `#[cfg(test)]` modules
- Integration tests go in the `tests/integration/` directory
- Run tests with `cargo test`
- Check formatting with `cargo fmt --check` before committing
- Run `cargo clippy` to catch common mistakes and ensure idiomatic code
- All public APIs must have rustdoc comments (`///`)

## Tasks

- [x] 0.0 Create feature branch
  - [x] 0.1 Create and checkout new branch `feature/epic7-citadel-state-persistence` from main
  - [x] 0.2 Verify current branch and clean working directory

- [x] 1.0 Create Citadel domain types in core layer
  - [x] 1.1 Create `src/core/platform/container/citadel.rs` file
  - [x] 1.2 Define `PaladinState` struct with all required fields (FR1.1-FR1.4)
  - [x] 1.3 Add serde Serialize/Deserialize derives to PaladinState
  - [x] 1.4 Define `BattalionState` struct with battalion type, config, paladin states, checkpoint (FR4)
  - [x] 1.5 Add serde derives to BattalionState
  - [x] 1.6 Define `StateSummary` struct for listing saved states
  - [x] 1.7 Define `StateType` enum (Paladin, Battalion)
  - [x] 1.8 Define `CheckpointData` struct for Battalion resumption tracking
  - [x] 1.9 Add schema_version field to both PaladinState and BattalionState (Q1 recommendation)
  - [x] 1.10 Add rustdoc comments to all public types
  - [x] 1.11 Write unit tests for struct creation and basic operations
  - [x] 1.12 Run `cargo test` to verify domain types compile and tests pass
  - [x] 1.13 Run `cargo fmt` and `cargo clippy`
  - [x] 1.14 Commit changes with message describing domain types

- [x] 2.0 Define CitadelPort trait in application layer
  - [x] 2.1 Create `src/application/errors/citadel_error.rs` file
  - [x] 2.2 Define `CitadelError` enum with all error variants (FR7)
  - [x] 2.3 Implement `thiserror::Error` derive for CitadelError
  - [x] 2.4 Add conversion from std::io::Error and serde_json::Error
  - [x] 2.5 Create `src/application/ports/output/citadel_port.rs` file
  - [x] 2.6 Define `CitadelPort` trait with Send + Sync bounds
  - [x] 2.7 Add `save_paladin` async method (FR6.1)
  - [x] 2.8 Add `load_paladin` async method (FR6.2)
  - [x] 2.9 Add `save_battalion` async method (FR6.3)
  - [x] 2.10 Add `load_battalion` async method (FR6.4)
  - [x] 2.11 Add `list_saved` async method (FR6.5)
  - [x] 2.12 Add comprehensive rustdoc comments to trait and all methods
  - [x] 2.13 Export citadel_port from `src/application/ports/output/mod.rs`
  - [x] 2.14 Export citadel_error from `src/application/errors/mod.rs`
  - [x] 2.15 Run `cargo check` to verify trait compiles
  - [x] 2.16 Run `cargo fmt` and `cargo clippy`
  - [x] 2.17 Commit changes with message describing port trait

- [x] 3.0 Implement FileCitadel adapter in infrastructure layer
  - [x] 3.1 Create `src/infrastructure/adapters/citadel/` directory
  - [x] 3.2 Create `src/infrastructure/adapters/citadel/mod.rs` with module exports
  - [x] 3.3 Create `src/infrastructure/adapters/citadel/file_citadel.rs`
  - [x] 3.4 Define `FileCitadel` struct with state_dir PathBuf field
  - [x] 3.5 Implement `FileCitadel::new()` constructor with directory creation (FR9.1)
  - [x] 3.6 Add directory permission validation in constructor (FR9.2)
  - [x] 3.7 Implement private `paladin_path(&self, id: Uuid) -> PathBuf` helper (FR9.4)
  - [x] 3.8 Implement private `battalion_path(&self, id: Uuid) -> PathBuf` helper (FR9.4)
  - [x] 3.9 Implement `CitadelPort::save_paladin` using tokio::fs and serde_json (FR2)
  - [x] 3.10 Add INFO logging for save operations (FR10.1)
  - [x] 3.11 Implement `CitadelPort::load_paladin` with error handling (FR3, FR7)
  - [x] 3.12 Add INFO logging for load operations (FR10.2)
  - [x] 3.13 Implement `CitadelPort::save_battalion` following same pattern
  - [x] 3.14 Implement `CitadelPort::load_battalion` with error handling
  - [x] 3.15 Implement `CitadelPort::list_saved` by scanning directory (FR6.5)
  - [x] 3.16 Add comprehensive error handling for all I/O operations
  - [x] 3.17 Add rustdoc comments to FileCitadel and all methods
  - [x] 3.18 Run `cargo test --lib` to verify adapter compiles
  - [x] 3.19 Run `cargo clippy` and address any warnings
  - [x] 3.20 Run `cargo fmt`
  - [x] 3.21 Commit changes with message describing adapter implementation

- [x] 4.0 Integrate Citadel with PaladinBuilder
  - [x] 4.1 Open `src/application/use_cases/paladin/paladin_builder.rs`
  - [x] 4.2 Add optional `citadel_port: Option<Arc<dyn CitadelPort>>` field to PaladinBuilder
  - [x] 4.3 Add `autosave_enabled: bool` field to builder configuration
  - [x] 4.4 Add `state_dir: Option<String>` field to builder configuration
  - [x] 4.5 Implement `enable_autosave(mut self) -> Self` method (FR8.1)
  - [x] 4.6 Implement `save_state_dir(mut self, path: impl Into<String>) -> Self` method (FR8.2)
  - [x] 4.7 Implement `with_citadel(mut self, citadel: Arc<dyn CitadelPort>) -> Self` method
  - [x] 4.8 Implement `restore_from(mut self, state_id: Uuid) -> Result<Self, PaladinError>` method (FR8.3)
  - [x] 4.9 Add validation in builder to ensure state_dir is writable when autosave enabled (FR8.4)
  - [x] 4.10 Update `build()` method to initialize FileCitadel if needed
  - [x] 4.11 Add logic to trigger save after Paladin execution in execution service (Note: Deferred to execution service implementation)
  - [x] 4.12 Add rustdoc comments to all new builder methods
  - [x] 4.13 Write unit tests for builder methods with citadel integration
  - [x] 4.14 Run `cargo test paladin_builder` to verify tests pass
  - [x] 4.15 Run `cargo clippy` and address warnings
  - [x] 4.16 Run `cargo fmt`
  - [x] 4.17 Commit changes with message describing builder integration

- [ ] 5.0 Add serialization support for domain entities
  - [ ] 5.1 Open `src/core/platform/container/paladin.rs`
  - [ ] 5.2 Verify PaladinData has Serialize/Deserialize derives
  - [ ] 5.3 Open `src/core/platform/container/garrison.rs`
  - [ ] 5.4 Verify GarrisonEntry has Serialize/Deserialize derives
  - [ ] 5.5 Add `ExecutionRecord` struct to citadel.rs if not exists
  - [ ] 5.6 Add Serialize/Deserialize to ExecutionRecord
  - [ ] 5.7 Ensure all nested types in state structs are serializable
  - [ ] 5.8 Write serialization roundtrip tests in citadel.rs unit tests
  - [ ] 5.9 Test JSON output is human-readable with proper formatting
  - [ ] 5.10 Run `cargo test` to verify all serialization tests pass
  - [ ] 5.11 Run `cargo fmt` and `cargo clippy`
  - [ ] 5.12 Commit changes with message describing serialization support

- [ ] 6.0 Write unit tests for domain types and serialization
  - [ ] 6.1 Add `#[cfg(test)]` module to `src/core/platform/container/citadel.rs`
  - [ ] 6.2 Write `test_paladin_state_creation` test
  - [ ] 6.3 Write `test_paladin_state_serialization_roundtrip` test
  - [ ] 6.4 Write `test_battalion_state_creation` test
  - [ ] 6.5 Write `test_battalion_state_serialization_roundtrip` test
  - [ ] 6.6 Write `test_state_summary_creation` test
  - [ ] 6.7 Write `test_checkpoint_data_serialization` test
  - [ ] 6.8 Write `test_schema_version_field_present` test
  - [ ] 6.9 Write `test_json_output_human_readable` test
  - [ ] 6.10 Verify test coverage ≥80% with `cargo llvm-cov` or similar
  - [ ] 6.11 Run `cargo test citadel::tests` to run all unit tests
  - [ ] 6.12 Run `cargo clippy` and address warnings
  - [ ] 6.13 Run `cargo fmt`
  - [ ] 6.14 Commit changes with message describing unit tests

- [ ] 7.0 Write integration tests for file persistence
  - [ ] 7.1 Create `tests/integration/citadel_integration_test.rs` file
  - [ ] 7.2 Add helper function to create temporary test directory
  - [ ] 7.3 Write `test_save_and_load_paladin_state` integration test
  - [ ] 7.4 Write `test_save_overwrites_existing_state` test (FR2.4)
  - [ ] 7.5 Write `test_load_nonexistent_state_returns_none` test
  - [ ] 7.6 Write `test_load_corrupted_json_returns_error` test (FR7.1)
  - [ ] 7.7 Write `test_directory_created_automatically` test (FR9.1)
  - [ ] 7.8 Write `test_list_saved_states` test (FR6.5)
  - [ ] 7.9 Write `test_save_and_load_battalion_state` test
  - [ ] 7.10 Write `test_file_naming_convention` test (FR9.4)
  - [ ] 7.11 Write `test_permission_error_handling` test (FR7.4)
  - [ ] 7.12 Write `test_paladin_restoration_via_builder` test
  - [ ] 7.13 Write `test_garrison_context_restored_correctly` test
  - [ ] 7.14 Add cleanup for temporary test directories in each test
  - [ ] 7.15 Run `cargo test --test citadel_integration_test` to verify all tests pass
  - [ ] 7.16 Run `cargo clippy` on test file
  - [ ] 7.17 Run `cargo fmt`
  - [ ] 7.18 Commit changes with message describing integration tests

- [ ] 8.0 Add configuration support for Citadel
  - [ ] 8.1 Open `src/config/application_settings.rs`
  - [ ] 8.2 Add `CitadelConfig` struct with state_dir and autosave_enabled fields
  - [ ] 8.3 Add serde Deserialize derive to CitadelConfig
  - [ ] 8.4 Add CitadelConfig to ApplicationSettings struct
  - [ ] 8.5 Set default values: state_dir = "./citadel", autosave_enabled = true
  - [ ] 8.6 Open `config.yml` in project root
  - [ ] 8.7 Add citadel configuration section with default values
  - [ ] 8.8 Add `config.test.yml` citadel section for test environment
  - [ ] 8.9 Write unit test to verify config deserialization
  - [ ] 8.10 Update PaladinBuilder to use config values by default
  - [ ] 8.11 Add rustdoc comments to CitadelConfig
  - [ ] 8.12 Run `cargo test` to verify config loading works
  - [ ] 8.13 Run `cargo fmt` and `cargo clippy`
  - [ ] 8.14 Commit changes with message describing configuration support

- [ ] 9.0 Create documentation and example code
  - [ ] 9.1 Create `examples/citadel_autosave.rs`
  - [ ] 9.2 Implement basic Paladin with autosave enabled example
  - [ ] 9.3 Add comments explaining autosave behavior
  - [ ] 9.4 Create `examples/citadel_restore.rs`
  - [ ] 9.5 Implement Paladin restoration from saved state example
  - [ ] 9.6 Add comments explaining restoration process
  - [ ] 9.7 Create `examples/battalion_checkpoint_recovery.rs`
  - [ ] 9.8 Implement Battalion resumption example (placeholder until Epic 4)
  - [ ] 9.9 Add inline documentation to example showing checkpoint behavior
  - [ ] 9.10 Add module-level rustdoc to `citadel.rs` explaining Citadel purpose
  - [ ] 9.11 Add usage examples in rustdoc for CitadelPort trait
  - [ ] 9.12 Add usage examples in rustdoc for PaladinBuilder citadel methods
  - [ ] 9.13 Verify examples compile with `cargo build --examples`
  - [ ] 9.14 Test run examples with `cargo run --example citadel_autosave`
  - [ ] 9.15 Test run `cargo run --example citadel_restore`
  - [ ] 9.16 Generate docs with `cargo doc --no-deps --open` and review
  - [ ] 9.17 Run `cargo fmt` on all example files
  - [ ] 9.18 Commit changes with message describing documentation and examples

- [ ] 10.0 Run final validation and cleanup
  - [ ] 10.1 Run full test suite: `cargo test --all`
  - [ ] 10.2 Verify test coverage meets ≥80% unit, ≥70% integration targets
  - [ ] 10.3 Run `cargo clippy -- -D warnings` to ensure zero warnings
  - [ ] 10.4 Run `cargo fmt --check` to verify formatting
  - [ ] 10.5 Run `cargo audit` to check for security vulnerabilities
  - [ ] 10.6 Build release mode: `cargo build --release`
  - [ ] 10.7 Review all rustdoc comments for completeness
  - [ ] 10.8 Review all error messages for clarity (FR7.5)
  - [ ] 10.9 Verify all acceptance criteria from PRD are met
  - [ ] 10.10 Remove any debug prints, temporary code, or commented-out code
  - [ ] 10.11 Update CHANGELOG.md with Epic 7 changes (if exists)
  - [ ] 10.12 Run git status to verify no uncommitted changes
  - [ ] 10.13 Push branch to remote: `git push origin feature/epic7-citadel-state-persistence`
  - [ ] 10.14 Create PR against main branch with detailed description
  - [ ] 10.15 Link PR to Epic 7 issue/tracking document

---

**Status:** Phase 2 Complete - Detailed sub-tasks generated

All tasks are ready for implementation following TDD principles and hexagonal architecture patterns.
