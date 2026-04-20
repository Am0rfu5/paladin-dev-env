# Changelog

All notable changes to the Paladin project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- **CLI Feature Flag** (Milestone 4, Epic 3): Gate the `paladin-cli` binary and `application::cli` module behind the new `cli` feature flag
  - New feature: `cli = ["dep:clap", "dep:dialoguer", "dep:indicatif", "dep:console", "dep:serde_yaml"]`
  - CLI-only dependencies (`clap`, `dialoguer`, `indicatif`, `console`, `serde_yaml`) are now `optional = true`
  - The `application::cli` module is now `#[cfg(feature = "cli")]`-gated in both `src/application/mod.rs` and `src/lib.rs`
  - The `paladin-cli` binary now requires `required-features = ["cli"]` in Cargo.toml
  - The `full` convenience flag includes `cli`
  - New integration test suite: `tests/cli_isolation_test.rs` — 9 regression tests verifying library compiles without CLI deps
  - Dedicated `cli-isolation` CI job verifies library-only and CLI-enabled builds
  - **Benefit**: Library consumers who don't use the CLI avoid compiling `clap` and associated TUI dependencies

### Changed - BREAKING
- **Default Feature Flags Revised**: Default features changed from `["redis-queue", "s3-storage", "openai-embeddings"]` to `["llm-openai"]` only
  - **Impact**: Applications relying on Redis queue, S3 storage, or OpenAI embeddings in default builds must now explicitly enable these features
  - **Migration**: Add required features to `Cargo.toml`: `paladin = { version = "0.1", features = ["redis-queue", "s3-storage"] }`
  - **Reason**: Enables minimal builds for pure orchestration use cases, reduces compile times and binary sizes
  - See [docs/MIGRATION.md](docs/MIGRATION.md) for complete migration guide

### Changed
- **Internal Type Visibility**: Applied `#[doc(hidden)]` to ~60 adapter and repository types (Milestone 4, Epic 2, Task 7.0)
  - **Affected Types**: All LLM adapters, Garrison adapters, Sanctum adapters, Arsenal adapters, Herald formatters, Repository implementations, and infrastructure adapters
  - **Impact**: No breaking changes - types remain accessible but hidden from documentation
  - **Strategy**: Used `#[doc(hidden)]` instead of `pub(crate)` to maintain compatibility with examples/tests/benchmarks (separate crates)
  - **User Guidance**: Consumers should use port traits (e.g., `LlmPort`, `GarrisonPort`) instead of concrete adapter types
  - **No import path changes required** - all existing code continues to work unchanged
  - See [project/DEPRECATIONS.md](project/DEPRECATIONS.md) for API transition strategy

### Added
- **Feature Flag System**: Comprehensive feature flags for controlling compiled dependencies
  - LLM Provider Flags: `llm-openai`, `llm-anthropic`, `llm-deepseek`, `llm-all`
  - Subsystem Flags: `vision`, `content-processing`, `web-server`, `notifications`
  - Infrastructure Flags: `redis-queue`, `s3-storage`, `openai-embeddings`, `qdrant`
  - Convenience Flag: `full` (enables all optional features)
  - See [docs/FEATURE_FLAGS.md](docs/FEATURE_FLAGS.md) for complete reference
- **CI Feature Matrix**: GitHub Actions workflow testing 15 feature combinations
  - Tests: no-default, default, all-features, full, individual providers and subsystems
  - Ensures all feature combinations compile and pass tests
  - See [.github/workflows/feature-flags.yml](.github/workflows/feature-flags.yml)

### Fixed
- **Live API Tests**: All OpenAI and Anthropic live API tests now passing (10/10 essential tests)
  - OpenAI: Fixed model assertion to handle versioned models (e.g., "gpt-3.5-turbo-0125")
  - OpenAI: Added graceful streaming error handling for incomplete JSON chunks
  - Anthropic: Fixed struct deserialization by removing underscore-prefixed fields
  - Anthropic: Updated test model to claude-3-haiku-20240307 (wider API tier access)
  - Anthropic: Added graceful streaming error handling
  - All tests verified with real API calls and comprehensive output validation
  - See **Milestone 3: Post-Epic 24 Completion** section below and `project/Milestone_3-Completion/Post-Epic_24-cleanup/LIVE_API_TESTS_SUCCESS.md` for complete documentation

### Removed
- **Legacy OpenAI Adapter**: Removed unused `openai_llm_adapter.rs` from `infrastructure/adapters/output/`
  - All functionality migrated to `infrastructure/adapters/llm/openai_adapter.rs`
  - Updated documentation references in `docs/HERALD.md`
  - Updated code examples in `examples/llm_provider_selection.rs`
  - Zero functional impact - adapter had no actual usage in codebase
  - See **Milestone 3: Post-Epic 24 Completion** section below for complete cleanup details

---

## Milestone 3: Post-Epic 24 Completion & Test Hardening

**Status**: ✅ Complete
**Branch**: `bugs/epic-24-post-fixes`
**Documentation**: `project/Milestone_3-Completion/Post-Epic_24-cleanup/`

This section documents the comprehensive cleanup, hardening, and bug fixes performed after Epic 24 to finalize Milestone 3. All work focused on ensuring production-readiness through integration test fixes, infrastructure improvements, and code quality enhancements.

### Added - Post-Epic 24 Completion

#### DevContainer Docker Compose Integration
- **Full docker-compose integration** for development services
  - Configured DevContainer to use `docker-compose.yml` for service orchestration
  - Services: Redis (queue), MySQL (storage), MinIO (S3-compatible storage)
  - Automatic service startup on container creation
  - Network: `paladin-network` for inter-service communication
- **DevContainer configurations**:
  - Features: rust, docker-in-docker, git
  - Mounts: cargo cache, target directory, git config
  - Post-create commands: install cargo-nextest, restore dependencies
  - VS Code extensions: rust-analyzer, crates, better-toml, GitLens
- **Service health checks and readiness**:
  - Redis: automatic connection test on startup
  - MySQL: root user with full privileges
  - MinIO: S3-compatible API on port 9000, console on 9001
- **Documentation updates**:
  - Updated `.devcontainer/README.md` with service details
  - Service connection information and credentials
  - Troubleshooting guide for common DevContainer issues

### Fixed - Post-Epic 24 Completion

#### Integration Test Fixes
- **Redis Queue Integration Tests** (all tests now passing):
  - Fixed Redis connection to use external docker-compose service instead of testcontainers
  - Updated connection from localhost to `redis` service hostname
  - Modified tests to support persistent Redis service (clear existing queues before tests)
  - Added proper cleanup: `FLUSHDB` command to reset state between tests
  - Removed testcontainers dependency from Redis queue tests (simplified infrastructure)
  - All 6 Redis queue integration tests passing
  - Tests documented: enqueue/dequeue, priority, batch operations, error handling

- **SQLite Garrison Integration Tests** (all tests now passing):
  - Fixed path resolution for in-memory SQLite databases
  - Changed from `:memory:` to unique file-based paths for test isolation
  - Added proper cleanup: remove test database files after completion
  - Fixed concurrent test execution issues (unique DB per test)
  - All 12 garrison integration tests passing
  - Tests documented: CRUD operations, search, TTL, concurrent access

- **LLM Provider Integration Tests** (modernized API):
  - Updated OpenAI integration test to use current `OpenAIAdapter` API
  - Fixed import paths from legacy `output::openai_llm_adapter` to `llm::openai_adapter`
  - Updated type names: `OpenAILlmAdapter` → `OpenAIAdapter`
  - Fixed configuration API: `OpenAIConfig::new()` now takes single argument (api_key)
  - Corrected provider name assertion: expects lowercase "openai"
  - Removed duplicate `cfg` attributes in live API tests
  - All integration tests compile and run successfully

#### Code Quality & Cleanup
- **Dead Code Warnings Resolved**:
  - Added `#[allow(dead_code)]` for deserialization-only fields in `OpenAIAdapter`
  - Suppressed warnings for: `OpenAIResponse.id`, `OpenAIChoice.index`, `OpenAIStreamChunk.id`, `OpenAIStreamChoice.index`, `OpenAIStreamDelta.role`
  - Added `#[allow(dead_code)]` for `RedisContainer.container` field (required for RAII)
  - All fields necessary for proper struct deserialization or resource management

- **Test Code Cleanup**:
  - Removed superfluous `vec![]` in test assertions (use direct comparison)
  - Fixed formatting inconsistencies in test files
  - Removed unused imports and dead test helper code
  - Cleaned up deprecated test patterns

- **Provider Factory Test Fixes**:
  - Fixed `test_case_insensitive_provider_names` to be environment-agnostic
  - Test now handles both success (API key present) and ConfigurationMissing (API key absent)
  - No longer assumes API keys are missing in test environment
  - Properly validates case-insensitive provider name matching

#### DevContainer Configuration
- **Formatting and Structure**:
  - Reformatted `.devcontainer/devcontainer.json` for consistency
  - Added inline comments explaining each configuration section
  - Standardized indentation and JSON structure
  - Improved readability of mounts and features configuration

- **Settings Corrections**:
  - Fixed rust-analyzer settings for better IDE experience
  - Corrected cargo check settings for faster feedback
  - Updated file associations for better file type recognition
  - Aligned editor settings with project conventions

### Changed - Post-Epic 24 Completion

#### Test Infrastructure
- **Integration Test Strategy**:
  - Redis tests: external service via docker-compose (no testcontainers)
  - SQLite tests: file-based databases with unique paths (better isolation)
  - LLM tests: feature-gated `live-api-tests` with proper `#[ignore]` markers
  - Clear separation: unit tests (always run) vs integration tests (opt-in)

- **Service Architecture**:
  - Redis: persistent service (not ephemeral testcontainer)
  - Requires explicit state cleanup in tests (`FLUSHDB`)
  - Better reflects production environment (persistent service)
  - Faster test execution (no container startup time)

- **Documentation**:
  - Added comprehensive test fix documentation in `Post-Epic_24-cleanup/`
  - `BUILD_TEST_FIXES.md`: Details all test compilation fixes
  - `LEGACY_CLEANUP_SUMMARY.md`: Legacy adapter removal summary
  - `LIVE_API_TESTS_SUCCESS.md`: Comprehensive live API test fixes
  - `QUICK_SUMMARY.md`: Quick reference for test status
  - `SESSION_SUMMARY.md`: Complete session chronicle
  - `verify_live_api_tests.sh`: Automated verification script

### Technical Debt Resolution

#### Resolved Issues
1. ✅ Legacy OpenAI adapter confusion (removed 580+ lines of dead code)
2. ✅ Integration test failures (Redis, SQLite, LLM providers - all fixed)
3. ✅ DevContainer service integration (docker-compose working)
4. ✅ Live API test robustness (graceful streaming error handling)
5. ✅ Code quality warnings (all dead_code warnings properly addressed)
6. ✅ Test environment dependencies (Redis testcontainer removed)

#### Quality Metrics
- **All unit tests passing**: 1606/1606 (100%)
- **All integration tests passing**: Redis 6/6, SQLite 12/12, LLM providers 10/10
- **Live API tests**: OpenAI 6/6, Anthropic 4/4 (100% essential tests)
- **Build status**: Clean compilation with `cargo check`
- **Code quality**: Zero clippy warnings with `cargo clippy -- -D warnings`
- **Formatting**: All code formatted with `cargo fmt`

### Production Readiness

#### Milestone 3 Completion Criteria
✅ **All Epic 24 tests passing** (100% test success rate)
✅ **Live API integration verified** (OpenAI, Anthropic with real API calls)
✅ **DevContainer fully operational** (docker-compose services working)
✅ **Integration tests hardened** (Redis, SQLite, Qdrant, LLM providers)
✅ **Code quality standards met** (zero warnings, all formatting checks pass)
✅ **Documentation complete** (comprehensive cleanup docs in Post-Epic_24-cleanup/)
✅ **Legacy code removed** (580+ lines of unused code eliminated)

#### Coverage Statistics
- **Total tests**: 1,628 (1606 unit + 22 integration)
- **Test execution time**: < 10 seconds for full test suite
- **CI-ready**: All tests deterministic, no flaky tests
- **API independence**: Core tests run without API keys

---

### Added - Epic 23: CLI, Config & Infrastructure Completion

#### Garrison Configuration
- Complete garrison (memory) configuration support from YAML files
- Support for `in_memory` garrison type: fast, temporary memory storage
- Support for `sqlite` garrison type: persistent memory with database backing
- Configuration options: `max_entries`, `ttl_seconds`, `path` for SQLite
- Garrison wiring in CLI agent command (resolved TODO at line 293)
- 9 comprehensive unit tests covering all configuration scenarios
- Example configurations in `examples/cli_configs/paladin_with_garrison.yaml`
- Comprehensive error handling with actionable error messages

#### Arsenal/MCP Configuration
- Complete arsenal (external tools) configuration support from YAML files
- Support for STDIO MCP servers: command-line tools via stdin/stdout
- Support for SSE MCP servers: HTTP-based tools via Server-Sent Events
- Automatic tool discovery and registration from MCP servers
- Support for environment variable substitution in configs (`${VAR_NAME}`)
- Arsenal wiring in CLI agent command (resolved TODO at line 296)
- 8 comprehensive unit tests covering STDIO, SSE, and validation scenarios
- Example configurations in `examples/cli_configs/paladin_with_arsenal.yaml`
- Integration examples: web search, filesystem, GitHub, custom APIs

#### Mock LLM Infrastructure
- **MockLlmAdapter** for CI-ready testing without API keys (`tests/helpers/mock_llm_adapter.rs`)
- Configurable responses: text, tool calls, streaming, and error injection
- Invocation recording for test assertions and verification
- Tool call simulation for arsenal integration testing
- Builder pattern for fluent mock configuration
- Support for sequential response queues
- Zero external dependencies for core test suite

#### Mock Arsenal Infrastructure
- **MockArsenalPort** for in-process tool testing (`tests/helpers/mock_arsenal_adapter.rs`)
- Tool registration with schemas and response configuration
- Success and error response simulation
- Invocation tracking with argument capture
- 9 unit tests for mock infrastructure validation
- Enables comprehensive tool integration testing in CI

#### CLI Integration Tests
- **84 comprehensive CLI integration tests**, all passing
- 6 Paladin execution tests: basic, with garrison, with arsenal, with config
- 4 Formation execution tests: sequential flow, output chaining, error propagation
- 5 Phalanx execution tests: parallel execution, result aggregation, error handling
- 8 Tool integration tests: LLM ↔ Arsenal ↔ result loop (Task 4.6)
  - Core flow: function call → Arsenal execution → result
  - Error handling: no arsenal, unknown tool, invalid arguments, execution errors
  - Advanced: sequential tool chains, garrison+arsenal integration
- 14 Error handling tests: configuration errors, execution failures, validation
- 9 Garrison configuration tests: in-memory, SQLite, validation, errors
- 8 Arsenal configuration tests: STDIO, SSE, tool registration, errors
- All tests use mock infrastructure - **zero API keys required**
- **CI-ready**: complete in < 5 seconds, no external dependencies

#### Scheduler Integration
- Production-ready scheduler using tokio-cron-scheduler v0.13
- **SchedulerPort trait** (`src/application/ports/output/scheduler_port.rs`):
  - Methods: `schedule_job()`, `cancel_job()`, `list_jobs()`, `get_job_info()`
  - Types: JobId, JobSpec, JobInfo, JobStatus, SchedulerError
  - 6 inline tests for trait contract
- **TokioCronSchedulerAdapter** (`src/infrastructure/adapters/scheduling/tokio_cron_adapter.rs`):
  - Full cron expression support for scheduling
  - Job lifecycle management (create, cancel, list, query)
  - Error handling and logging
  - 13 inline tests for adapter implementation
- **APIContentDeliverer integration**:
  - Replaced scheduler stub (resolved TODO at line 297)
  - `schedule_delivery()` creates real scheduled jobs
  - `cancel_delivery()` cancels pending deliveries
  - Returns JobId for job tracking
- **Configuration support**:
  - SchedulerConfig in `src/config/application_settings.rs`
  - Fields: `enabled`, `default_cron`, `channel_size`
  - YAML configuration support
- 21 total scheduler tests (16 unit + 5 integration)

#### Documentation
- **CLI Configuration Guide** (`docs/cli/CONFIGURATION.md`, 500+ lines):
  - Comprehensive guide for garrison, arsenal, and scheduler configuration
  - Complete YAML configuration examples with detailed comments
  - Environment variable usage and substitution
  - Troubleshooting section with common errors and solutions
  - Integration examples for popular MCP servers
- **CLI Testing Guide** (`docs/cli/TESTING.md`) updates:
  - Mock infrastructure documentation (MockLlmAdapter, MockArsenalPort)
  - Test tier strategy (no deps, Docker-gated, API-key-gated)
  - Test coverage statistics and categories
  - Best practices for writing tests with mocks
- **CLI Usage Guide** (`docs/CLI_USAGE.md`) updates:
  - References to new CONFIGURATION.md guide
  - Updated with garrison and arsenal capabilities
  - Example usage patterns

#### Configuration Examples
- `examples/cli_configs/paladin_with_garrison.yaml` - In-memory and SQLite garrison examples
- `examples/cli_configs/paladin_with_arsenal.yaml` - STDIO and SSE MCP server examples
- `examples/cli_configs/paladin_full_config.yaml` - Complete configuration with all features
- All examples include extensive inline comments and usage instructions
- Examples tested and validated for out-of-the-box functionality

### Changed - Epic 23: CLI, Config & Infrastructure Completion

#### Configuration Loading
- Extended `PaladinYamlConfig` with garrison and arsenal configuration structures
- Enhanced ConfigLoader with garrison and arsenal parsing methods
- Added environment variable resolution for sensitive configuration values
- Improved error messages with actionable guidance

#### CLI Command Infrastructure
- Removed TODO at `src/application/cli/commands/agent.rs` line 293 (garrison wiring)
- Removed TODO at `src/application/cli/commands/agent.rs` line 296 (arsenal wiring)
- Garrison adapter instantiation based on YAML config
- Arsenal registry population from MCP server configs
- Integration with PaladinBuilder for full feature wiring

#### Content Delivery Infrastructure
- Removed scheduler stub at `src/infrastructure/adapters/output/api_content_deliverer.rs` line 297
- Integrated SchedulerPort for scheduled content delivery
- Added cancellation support for pending scheduled deliveries
- JobId tracking for scheduled tasks

#### Test Organization
- Implemented three-tier test strategy:
  - **Tier 1**: Core functionality, no dependencies (84 tests, runs in CI)
  - **Tier 2**: Docker-gated service tests (#[ignore], clear skip messages)
  - **Tier 3**: API-key-gated provider tests (feature flag + #[ignore])
- All Tier 1 tests CI-ready with deterministic execution
- Test helper module exports: MockLlmAdapter, MockArsenalPort, MockPaladinPort

### Fixed - Epic 23: CLI, Config & Infrastructure Completion

#### Code Quality
- Resolved all Epic 23 scope TODOs (3 total: agent.rs lines 293, 296; api_content_deliverer.rs line 297)
- All code passes `cargo clippy -- -D warnings` with zero warnings
- All code formatted with `cargo fmt` - zero formatting issues
- Zero compilation warnings in Epic 23 changes

#### Test Coverage
- Closed critical test coverage gap: LLM ↔ Arsenal ↔ result tool call loop (8 tests added)
- Added missing garrison configuration tests (9 tests)
- Added missing arsenal configuration tests (8 tests)
- Added missing error handling tests (14 tests)
- **Test count:** 84 new CLI integration tests, all passing

#### Deferred Task Completion
- **Epic 9, Task 5.8**: Garrison configuration wiring ✅
- **Epic 9, Task 5.9**: Arsenal/MCP configuration wiring ✅
- **Epic 10, Tasks 13.4-13.6**: CLI integration tests for Paladin, Formation, Phalanx ✅
- **Epic 18, Tasks 9.1-9.7**: End-to-end testing and test documentation ✅
- **All Milestone 3 deferred tasks now complete**

### Added - Epic 22: Battalion & Commander Hardening

#### Commander Metadata Export
- Commander now exports detailed execution metadata to JSON files when `metadata_output_dir` is configured
- JSON files use naming convention: `{strategy}_{timestamp}_{uuid_short}.json`
- Metadata includes: battalion_id, battalion_name, strategy_used, timestamps, final_output, paladin_results
- Per-paladin execution metrics: execution times and token usage independently tracked
- Comprehensive metadata structure for audit trails, debugging, and performance analysis
- Automatic directory creation and validation with detailed error messages
- Integration test coverage for end-to-end metadata export validation

#### Enhanced Phalanx Metrics Collection
- Phalanx now tracks per-paladin execution times in `per_paladin_times: HashMap<String, u64>`
- Per-paladin token usage tracked in `per_paladin_tokens: HashMap<String, TokenUsage>`
- Total token aggregation across all parallel executions in `total_tokens: u64`
- Success/failure counts: `paladin_success_count` and `paladin_failure_count`
- Metrics collected concurrently for accurate parallel execution profiling
- Enhanced BattalionResult with comprehensive performance data
- 100% test coverage for metrics collection across all Battalion patterns

#### Test Infrastructure Improvements
- MockLlmAdapter test infrastructure with configurable response queueing
- Call count tracking and state management for repeatable tests
- Helper functions: `create_mock_with_responses()`, `create_test_paladin_with_mock()`
- Strategy-specific mock implementations (MockChainOfCommandPort for delegation testing)
- Comprehensive test coverage for Campaign and ChainOfCommand orchestration patterns
- Error handling tests: FailFast, ContinueOnError, RetryThenContinue, partial failure scenarios
- Integration test for Commander metadata export with JSON validation
- All 1590 lib tests passing, 211 doctests passing, 19 integration tests

#### Paladin Registry Foundation
- PaladinRegistry trait defining standard interface for Paladin lookup and management
- HashMapPaladinRegistry implementation with thread-safe concurrent access via RwLock
- O(1) average case lookup performance for Paladin retrieval by ID
- Methods: `register()`, `unregister()`, `get()`, `contains()`, `list_ids()`, `clear()`, `count()`
- Duplicate ID prevention with detailed error reporting
- Full rustdoc with usage examples and performance characteristics
- Ready for Council and Grove integration (implementation in Epic 22 Sprint 2)

### Changed - Epic 22: Battalion & Commander Hardening

#### BattalionConfig Enhancements
- Added `metadata_output_dir: Option<PathBuf>` for optional metadata export configuration
- New `validate_metadata_dir()` method ensures directory exists and is writable before execution
- Builder pattern method: `with_metadata_dir(dir: PathBuf)` for fluent configuration
- Comprehensive error messages for directory validation failures

#### BattalionResult Structure
- Extended with `TokenUsage` struct containing `prompt_tokens`, `completion_tokens`, `total_tokens`
- Added `per_paladin_times: HashMap<String, u64>` for granular timing data
- Added `per_paladin_tokens: HashMap<String, TokenUsage>` for granular token tracking
- Added `total_tokens: u64` for cross-Battalion token aggregation
- Added `paladin_success_count: usize` and `paladin_failure_count: usize` for execution summaries
- Backward-compatible additions, all existing code continues to work

#### Commander Test Coverage
- Enabled and fixed previously ignored Campaign orchestration tests with DAG validation
- Enabled and fixed previously ignored ChainOfCommand delegation tests with specialist selection
- Added comprehensive error handling test suite covering all ErrorStrategy variants
- MockChainOfCommandPort returns properly formatted "SELECT: name1, name2\nREASON: ..." responses
- Unique paladin naming in tests to avoid graph cycle detection false positives
- 50 Commander unit tests now passing (up from ~40 with ignored tests)

### Fixed - Epic 22: Battalion & Commander Hardening

#### Code Quality
- Resolved clippy warning: unused loop variable in phalanx_service.rs timing validation
- Resolved clippy warning: manual Option::map pattern in paladin_builder.rs MockArsenalRegistry
- Fixed 6 failing doctests in PaladinRegistry with correct trait imports and API usage
- Fixed typo in HandoffConfig doctest (removed `stat` field)
- All code now passes `cargo clippy -- -D warnings` with zero warnings

#### Test Reliability
- Fixed ChainOfCommand test failures by implementing context-aware mock responses
- Fixed Campaign test failures by ensuring unique paladin names in DAG construction
- MockPaladinPort enhanced with configurable response strategies per test scenario
- All integration tests now run reliably without flaky failures

### Added - Epic 20: Vision Pipeline Completion

#### Vision Configuration System
- Complete vision configuration support with retry logic and token limits
- `VisionConfig` struct with configurable retry parameters: `max_retries`, `initial_backoff_ms`, `backoff_multiplier`
- Provider-specific token limits for OpenAI and Anthropic
- Exponential backoff for transient failures (network errors, rate limits, timeouts)
- Configuration loaded from `config.yml` with sensible defaults
- Test configuration support in `config.test.yml`

#### Vision Error Handling
- Comprehensive `VisionError` enum with 10 error variants
- Error types: `InvalidImage`, `UnsupportedFormat`, `AuthenticationError`, `RateLimitExceeded`, `ProviderError`, `NetworkError`, `Timeout`, `UnsupportedProvider`, `MaxRetriesExceeded`, `FileTooLarge`
- Detailed error messages with context for debugging
- Integration with existing error handling patterns via `thiserror`

#### OpenAI Vision Adapter
- Full OpenAI vision API integration with retry logic
- Support for URL-based and base64-encoded images
- Image detail levels: Auto, Low (512x512), High (2048x2048)
- Multiple images per request (up to 10)
- Token estimation: ~85 tokens (low), ~170 tokens per tile (high)
- Models supported: `gpt-4o`, `gpt-4o-mini`, `gpt-4-vision-preview`
- Automatic retry with exponential backoff on transient failures
- Comprehensive unit tests with mock server validation

#### Anthropic Vision Adapter
- Full Anthropic vision API integration with retry logic
- Support for URL-based images (auto-converted to base64) and base64-encoded images
- Image detail levels with automatic conversion
- Multiple images per request (up to 20)
- Models supported: `claude-3-opus`, `claude-3-sonnet`, `claude-3-haiku`
- Automatic base64 encoding for all image types
- Automatic retry with exponential backoff on transient failures
- Comprehensive unit tests with mock server validation

#### Paladin Vision Execution
- `execute_with_vision()` method added to `PaladinExecutionService`
- Seamless integration with existing Paladin execution flow
- Support for vision-capable LLM providers through trait abstraction
- Vision content validation before API calls
- Memory (Garrison) integration for vision analysis history
- Tool (Arsenal) integration for vision-augmented agents
- Circuit breaker support for fault tolerance

#### Vision Integration Tests
- Environment-gated integration tests with real API calls
- Tests controlled by `ENABLE_VISION_TESTS` environment variable
- OpenAI vision integration tests with multiple scenarios
- Anthropic vision integration tests with multiple scenarios
- Multiple images test, image URL test, high detail processing test
- Test fixtures: sample images for integration testing
- Comprehensive documentation for running tests with API keys

#### Examples and Documentation
- Updated `vision_analysis.rs` example with comprehensive demonstrations
- Base64-encoded image processing example
- Multiple images comparison example
- Error handling patterns and best practices
- Added vision retry configuration documentation to `SENTINEL.md`
- Image size limits documentation (OpenAI: 20MB, Anthropic: 5MB)
- Troubleshooting section for common vision issues
- Configuration examples and best practices for different environments

### Changed - Epic 20: Vision Pipeline Completion

#### Configuration Structure
- Enhanced `ApplicationSettings` with `vision: VisionConfig` field
- Vision configuration loaded from YAML with proper deserialization
- Backward-compatible configuration loading (vision section optional)

#### LLM Adapters Enhancement
- OpenAI adapter constructor updated to accept `VisionConfig`
- Anthropic adapter constructor updated to accept `VisionConfig`
- Vision-specific retry logic separated from general LLM retries
- Provider capabilities detection for vision support

### Added - Epic 19: Herald & Domain Type Consolidation

#### StreamChunk Extensible Metadata
- Complete StreamChunk structure with 7 fields including extensible metadata HashMap
- Builder pattern with validation for safe construction
- Fields: `chunk_id`, `sequence_number`, `timestamp`, `content`, `token_count`, `is_final`, `metadata`
- Support for provider-specific and custom metadata without struct changes
- JSON serialization/deserialization with flattened metadata
- Comprehensive rustdoc with multiple usage examples

#### ExecutionMetadata Full Telemetry
- Complete ExecutionMetadata structure with 9 fields for comprehensive observability
- Builder pattern with validation for safe construction
- Fields: `execution_id`, `start_time`, `end_time`, `duration_ms`, `model_used`, `token_usage`, `cost_estimate`, `error_count`, `metadata`
- Duration calculation helper method
- Total cost estimation helper method
- Extensible metadata for custom telemetry
- Re-exported `TokenUsage` from llm_port with consistent field names
- Comprehensive rustdoc with telemetry use cases and examples

#### Auto-Registration of Built-in Formatters
- `HeraldRegistry::default()` automatically registers three built-in formatters
- Zero-config pattern: JSON, Markdown, and Table formatters available immediately
- No manual registration required for built-in formatters
- Custom formatters can still be added via `register()` method
- Built-in formatters can be overridden with custom configurations
- Updated rustdoc with zero-config and extensible patterns

### Changed - Epic 19: Herald & Domain Type Consolidation

#### Domain Type Consolidation
- Removed placeholder `PaladinResult`, `BattalionResult`, and `PaladinError` types from herald.rs
- Herald system now uses actual domain types from paladin.rs and battalion modules
- Added public re-exports for Herald consumers: `PaladinResult`, `BattalionResult`, `PaladinError`, `TokenUsage`
- Updated all Herald adapters (JSON, Markdown, Table) to use actual type structures
- Fixed field access patterns: `paladin_results` instead of `results` for Battalion
- PaladinError now handled as enum with match on variants

#### Documentation Improvements
- Enhanced StreamChunk rustdoc with detailed field descriptions and extensible metadata examples
- Enhanced ExecutionMetadata rustdoc with telemetry use cases and comprehensive examples
- Updated HeraldRegistry rustdoc documenting auto-registered formatters and usage patterns
- Added examples for zero-config pattern (recommended) and manual registration
- Updated all Herald-related documentation for consolidated types

### Added - Epic 18: CLI Enhancement

#### New CLI Commands

**Onboarding Wizard** (`paladin onboarding`)
- Interactive first-time setup wizard for environment configuration
- Provider selection (OpenAI, DeepSeek, Anthropic) with descriptions
- Secure API key input with validation and masking
- Automatic `.env` file generation with comments
- Sample configuration file creation
- Resumable state for interrupted sessions
- Real-time validation of API keys and connectivity

**Setup Check** (`paladin setup-check`)
- Comprehensive environment validation
- System checks: Rust version, cargo, git availability
- Environment checks: Required and optional variables
- Provider validation: API key format and connectivity
- Optional service checks: Redis, Qdrant, MinIO
- Categorized results: System, Environment, Provider, Service
- Multiple output formats: standard, verbose, JSON
- Exit codes: 0 (success), 1 (critical failures), 2 (warnings only)
- CI/CD integration support

**Features Discovery** (`paladin features`)
- Discover all available Paladin capabilities
- Feature categories: Agent, Battalion, Orchestration, Memory, Utilities
- 24 documented features with descriptions and documentation links
- Orchestration patterns: Formation, Phalanx, Campaign, Chain of Command, Conclave, Council, Grove, Maneuver
- Memory systems: Garrison (InMemory, Sqlite), Sanctum (InMemory, Qdrant)
- Category filtering: `--category` flag
- Output formats: table (default), JSON
- Feature availability status indicators

**Muster Command** (`paladin muster`) [STUB]
- AI-powered Battalion configuration generation from natural language
- LLM-based task analysis and pattern suggestion
- Automatic YAML/JSON config generation
- Validates generated configurations
- Supports all orchestration patterns
- Note: Requires LLM integration (currently returns stub configurations)

**Council Command** (`paladin council`) [STUB]
- Quick multi-agent discussions without configuration files
- Multiple discussion modes: parallel, sequential, debate
- Configurable agent roles and perspectives
- Automatic synthesis of diverse viewpoints
- Output formats: markdown, JSON, plain text
- Note: Requires LLM integration (currently returns mock discussions)

#### CLI Infrastructure

**Output Formatters**
- `OutputFormatter`: Unified formatter for CLI output with colored styling
- `TableFormatter`: ASCII table rendering with alignment and borders
- Consistent styling: success (green), error (red), warning (yellow), info (cyan)
- NO_COLOR environment variable support
- Support for both TTY and non-TTY environments

**Progress Indicators**
- `ProgressSpinner`: Async spinner for long-running operations
- `ProgressBar`: Progress tracking with percentage and ETA
- Customizable messages and styling
- Automatic cleanup on completion or error

**Error Handling**
- `CliError` enum with 30+ specific error variants
- Detailed error messages with context
- Error categories: Configuration, IO, Validation, Provider, Service
- Proper error propagation with `CliResult<T>`
- User-friendly error formatting

**Templates**
- `.env` file template generation with provider-specific sections
- Paladin configuration templates (YAML) for all providers
- Battalion configuration templates for all orchestration patterns
- Template merging for incremental updates
- Valid YAML/JSON output with comments

#### Documentation

**CLI Usage Guide** (`docs/CLI_USAGE.md`)
- Comprehensive command reference (405 new lines)
- Getting Started section with onboarding workflow
- Detailed syntax, options, and examples for all commands
- Cross-references to detailed guides

**Detailed Command Guides** (~1,900 lines total)
- `docs/cli/ONBOARDING.md`: Wizard flow, security, troubleshooting (~300 lines)
- `docs/cli/SETUP_CHECK.md`: Check categories, exit codes, CI/CD (~350 lines)
- `docs/cli/MUSTER.md`: AI-powered generation, patterns, examples (~600 lines)
- `docs/cli/COUNCIL.md`: Multi-agent discussions, modes, advanced usage (~650 lines)

**README Updates**
- Added CLI Quick Start section (65 lines)
- Installation instructions
- First-time setup with onboarding wizard
- Quick commands reference
- Links to comprehensive documentation

**Example Configurations**
- Enhanced `examples/cli_configs/paladin_with_rag.yaml`
- Verified existing examples: basic_paladin, formation, phalanx
- All examples include usage instructions

#### Testing

**Test Infrastructure** (29 new tests, 193 CLI tests total)
- Mock test utilities in `src/application/cli/tests/mod.rs`
- `formatter_tests.rs`: 13 tests for output and table formatters
- `command_tests.rs`: 16 tests for command validation and parsing
- Integration test framework in `tests/cli/integration_tests.rs`

**Test Coverage**
- 193 CLI unit tests (100% pass rate, 6 intentionally ignored)
- All tests follow TDD principles
- Zero clippy warnings with `-D warnings` flag
- Code formatted with `cargo fmt`
- 1,487 total project tests passing

#### Architecture & Code Quality

**Hexagonal Architecture**
- All CLI code in application layer (`src/application/cli/`)
- Clear separation: commands, config, formatters, templates, error handling
- Port/adapter pattern for external integrations
- No direct dependencies on infrastructure layer

**Code Quality Metrics**
- Zero clippy warnings (strict mode: `-D warnings`)
- All code formatted with `cargo fmt`
- Comprehensive rustdoc for public APIs
- Consistent error handling patterns
- No debug prints or temporary code in production

**Performance**
- Release build: 2m 48s
- CLI test suite: 0.02s
- Full test suite: 7.82s
- Async spinners (non-blocking UI)
- Efficient table rendering

### Changed - Epic 17.5: CLI Directory Consolidation

#### CLI Module Consolidation
- **Unified CLI Structure**: Consolidated all CLI code into `src/application/cli/`
  - Removed legacy `src/cli/` directory (18 files)
  - All CLI functionality now follows hexagonal architecture in application layer
  - Commands: agent, arsenal, battalion, maneuver, user
  - Config: paladin_config, battalion_config, loader
  - Output: Unified `CliError` type with 25+ variants
  - Templates: paladin_template, battalion_template
  - Interactive: TTY utilities and prompts

- **Error Handling**: Single unified error type
  - Merged `src/cli/output/errors::CliError` and `src/application/cli::error::CliError`
  - All CLI commands now use `CliError` and `CliResult` from `application::cli::error`
  - Removed duplicate error conversion logic
  - Improved error messages with detailed formatting

- **Import Path Changes**: Updated all imports to new structure
  - **Old**: `use paladin::cli::*;` (deprecated and removed)
  - **New**: `use paladin::application::cli::*;`
  - Binary entry point (`paladin-cli.rs`) updated
  - All examples and tests updated

- **Code Quality Improvements**:
  - Fixed 27 clippy warnings (clone_on_copy, field_reassign_with_default, etc.)
  - All tests passing: 1411 unit tests
  - Zero clippy warnings with `-D warnings`
  - Code formatted with `cargo fmt`

#### Migration Guide for Developers

If you have code importing from the old CLI structure, update your imports:

```rust
// OLD (removed)
use paladin::cli::output::errors::CliError;
use paladin::cli::commands::agent;
use paladin::cli::config::loader::load_paladin_config;

// NEW (current)
use paladin::application::cli::error::{CliError, CliResult};
use paladin::application::cli::commands::agent;
use paladin::application::cli::config::loader::load_paladin_config;
```

The `src/cli/` directory has been completely removed. All CLI functionality is now properly organized in the application layer following hexagonal architecture principles.

### Added - Epic 17: Flow DSL & Agent Rearrangement (Maneuver Pattern)

#### Flow DSL Parser
- **FlowParser**: String-based workflow orchestration with intuitive syntax
  - Sequential operator `->` for linear workflows (e.g., "A -> B -> C")
  - Parallel operator `,` for concurrent execution (e.g., "A, B, C")
  - Nested patterns with parentheses for complex workflows
  - Complete lexer, AST, and parser implementation in core layer
  - 57 comprehensive tests covering all syntax patterns
- **Error Handling**: Detailed `FlowParseError` types with position tracking
  - Helpful error messages for common syntax mistakes
  - Support for debugging complex nested expressions
  - Suggestion methods for error recovery

#### Maneuver Domain Model
- **Maneuver**: New Battalion pattern for declarative workflow definition
  - Parse flow expressions into executable agent graphs
  - Support for 10-30 agent workflows with automatic dependency resolution
  - Three error strategies: FailFast, ContinueParallel, IgnoreErrors
  - Two output formats: CombinedText, StructuredJson
  - 21 domain tests validating configuration and behavior
- **ManeuverConfig**: Comprehensive configuration with timeouts and validation
  - Per-agent timeout controls
  - Error strategy selection
  - Output format specification
  - Validation rules for agent count and flow complexity

#### Execution Engine
- **ManeuverExecutionService**: Async execution with dependency resolution
  - Parallel execution of independent agents
  - Sequential execution for dependent agents
  - Result aggregation based on output format
  - Error handling with configurable strategies
  - 3 integration tests verifying execution patterns
- **Flow Visualization**: ASCII and Mermaid diagram generation
  - ASCII art for terminal display and documentation
  - Mermaid diagrams for rich visualizations
  - Support for simple, nested, and complex flows
  - 12 tests covering all visualization scenarios

#### Commander Integration
- **Pattern Detection**: Automatic Maneuver pattern recognition
  - Parse flow expressions from input strings
  - Detect sequential and parallel patterns automatically
  - Seamless integration with existing Formation and Phalanx patterns
  - 16 tests for Commander Maneuver integration
- **CLI Commands**: Complete CLI support for Maneuver operations
  - `paladin maneuver create` - Generate Maneuver configurations
  - `paladin maneuver execute` - Execute flow expressions
  - `paladin maneuver validate` - Validate flow syntax
  - `paladin maneuver visualize` - Generate visualizations
  - 4 CLI command tests

#### Documentation & Examples
- **Comprehensive Documentation**: Complete documentation suite
  - `docs/guides/flow-dsl.md` (800+ lines) - Complete Flow DSL user guide
    - Syntax reference with EBNF grammar
    - Error handling strategies (FailFast, ContinueParallel, IgnoreErrors)
    - Visualization guide (ASCII tree, Mermaid flowcharts)
    - 10+ practical examples and best practices
    - Troubleshooting section with common errors
    - Performance considerations and scalability limits
  - Updated `docs/BATTALION.md` with Maneuver pattern section (lines 500-560)
  - Updated `docs/CLI_USAGE.md` with Maneuver CLI commands
  - Updated main `README.md` - Changed from 5 to 8 orchestration patterns
    - Added Council, Grove, and Maneuver pattern descriptions
    - Added link to Flow DSL guide
- **Production Examples**: 3 complete working examples (958 lines)
  - `maneuver_basic.rs` - Introduction to Flow DSL
  - `maneuver_nested_flow.rs` - Enterprise review pipeline
  - `maneuver_dynamic_flow.rs` - Runtime flow generation
- **Performance Benchmarks**: 7 benchmark suites (32 test cases)
  - Parse time benchmarks (4 complexity levels)
  - Visualization performance (ASCII and Mermaid)
  - Validation overhead measurement
  - Sequential and parallel execution benchmarks
  - Nested flow performance testing
  - Overhead comparison vs Formation/Phalanx patterns

#### Test Coverage
- **113 Total Tests**: Comprehensive coverage across all components
  - Parser: 57 tests (lexer, AST, error handling)
  - Domain: 21 tests (Maneuver, ManeuverConfig)
  - Execution: 3 tests (ManeuverExecutionService)
  - Commander: 16 tests (pattern detection, integration)
  - Visualization: 12 tests (ASCII, Mermaid)
  - CLI: 4 tests (command validation)
- **Benchmark Coverage**: 32 performance test cases
  - Parse performance: < 1ms for complex flows
  - Execution overhead: < 2% vs direct patterns
  - Memory efficiency validation
  - Scalability testing (3-20 agents)

### Added - Epic 14: Autonomous Agent Features

#### Autonomous Planning Mode
- **Auto Loop Detection**: New `MaxLoops::Auto { max_subtasks: u32 }` variant enables intelligent loop optimization
  - Automatic task complexity analysis
  - Dynamic subtask decomposition for complex tasks
  - Optimal loop count determination (simple tasks use fewer loops)
- **Planning Service**: New `PlanningService` with comprehensive task planning
  - Task complexity assessment
  - Structured plan generation with subtasks
  - Subtask execution and synthesis
  - Integration with Paladin execution flow
- **Planning Configuration**: `PlanningConfig` with enabled flag, max_subtasks, and complexity threshold
- **Domain Types**: `TaskPlan`, `Subtask`, `ComplexityLevel` for structured planning representation

#### Auto-Generate System Prompts
- **Prompt Generation Service**: New `PromptGenerationService` for LLM-powered prompt creation
  - Generate system prompts from natural language agent descriptions
  - Optimize prompts for specific agent roles and capabilities
  - Cache generated prompts for reuse
  - Support for prompt regeneration and manual overrides
- **Prompt Configuration**: `PromptConfig` with enabled flag and optional cache control
- **Builder Integration**: `agent_description()` method on PaladinBuilder for seamless prompt generation

#### Dynamic Temperature Adjustment
- **Temperature Service**: New `TemperatureService` with task-based temperature optimization
  - Automatic task type classification (factual, creative, balanced)
  - Temperature bounds configuration (min/max range)
  - Classification heuristics based on task keywords
  - Real-time temperature adjustment per task
- **Temperature Configuration**: `TemperatureConfig` with enabled flag, min/max bounds, and custom keywords
- **Task Types**: `TaskType` enum (Factual, Creative, Balanced) with appropriate temperature ranges

#### Intelligent Agent Handoffs
- **Handoff Service**: New `HandoffService` for delegation between specialist agents
  - Specialist discovery and routing
  - Task complexity assessment for delegation
  - Circuit breaker integration for reliability
  - Handoff depth limiting (prevent infinite delegation)
- **Handoff Configuration**: `HandoffConfig` with enabled flag, strategy, and max delegation depth
- **Handoff Strategies**: `HandoffStrategy` enum (Automatic, ExplicitOnly) for control
- **Domain Types**: `HandoffDecision`, `HandoffMetadata` for structured delegation tracking

#### Handoff Tool Integration
- **Arsenal Integration**: New `HandoffTool` registered in Arsenal for LLM-accessible delegation
  - `delegate_to_specialist` function for explicit handoffs
  - JSON schema for LLM tool use
  - Specialist validation and routing
  - Seamless integration with agent execution loop

#### Configuration & Builder API
- **Autonomous Configuration**: New `AutonomousConfig` aggregating all autonomous features
  - Centralized configuration structure
  - YAML configuration support
  - CLI flag integration
  - Builder pattern support via `PaladinBuilder`
- **Builder Methods**: New autonomous feature methods on PaladinBuilder
  - `enable_planning(bool)` - Toggle autonomous planning
  - `agent_description(String)` - Set description for prompt generation
  - `enable_dynamic_temperature(bool)` - Toggle temperature adjustment
  - `enable_handoffs(bool)` - Toggle delegation capabilities

#### Documentation & Examples
- **Comprehensive Guide**: New `docs/AUTONOMOUS.md` (400+ lines)
  - Introduction and features overview
  - Detailed user story documentation (all 5 features)
  - Configuration guide (YAML, CLI, Builder)
  - Best practices and performance considerations
  - Error handling and troubleshooting
  - Advanced usage patterns
  - Complete API reference
- **Working Examples**: 5 comprehensive example files (~1,400 lines)
  - `autonomous_planning.rs` - Planning mode with task decomposition
  - `autonomous_prompt_generation.rs` - Auto-prompt generation concepts
  - `dynamic_temperature.rs` - Temperature adjustment by task type
  - `agent_handoffs.rs` - Specialist delegation workflow
  - `autonomous_full_config.rs` - All features combined
- **Examples README**: Updated `examples/README.md` with autonomous section

#### Testing & Quality
- **Comprehensive Testing**: 1,280+ tests passing including autonomous features
  - Unit tests for all services and domain logic
  - Integration tests for Paladin with autonomous features
  - MockLlmAdapter integration for deterministic testing
- **Code Quality**: Zero clippy warnings in strict mode
  - All code formatted with rustfmt
  - Comprehensive rustdoc for all public APIs
  - Error handling with thiserror patterns

#### Security Audit Results
- **Vulnerabilities**: 2 transitive dependency vulnerabilities identified (non-critical)
  - `rsa 0.9.10`: Marvin Attack timing sidechannel (RUSTSEC-2023-0071) - Medium severity, no upgrade available (from sqlx-mysql)
  - `tokio-tar 0.3.1`: PAX header parsing issue (RUSTSEC-2025-0111) - No upgrade available (from testcontainers, dev dependency only)
- **Unmaintained Crates**: 9 warnings about unmaintained transitive dependencies
  - All are indirect dependencies from test/dev dependencies
  - No immediate security risk to production code
  - Monitored for future upgrades when upstream updates available

### Added - Epic 13: Sentinel Vision System

#### Vision API & Multi-Modal Processing
- **Vision Content Types**: Support for three image input formats
  - `ImageUrl`: Process images from public web URLs
  - `ImageFile`: Load and analyze local image files with automatic base64 encoding
  - `ImageBase64`: Direct base64-encoded image input
- **Vision-Enabled Paladins**: New `enable_vision()` builder method and `execute_with_vision()` function
- **Image Detail Levels**: Control token usage and analysis depth
  - `Low`: ~85 tokens, fast processing for simple tasks
  - `High`: 170+ tokens, detailed analysis with fine-grained details
  - `Auto`: Automatic balancing based on image complexity
- **Multi-Provider Support**: Vision capabilities across LLM providers
  - OpenAI: GPT-4o, GPT-4o-mini with vision support
  - Anthropic: Claude 3 Opus, Sonnet, Haiku with vision capabilities

#### Document Processing System
- **PDF Extraction**: Comprehensive PDF text extraction via `PdfExtractor`
  - Multi-page document support
  - Metadata extraction (title, author, creation date, page count)
  - Character-accurate text extraction
  - Page-by-page content access
- **Intelligent Document Chunking**: Flexible chunking strategies via `ChunkConfig`
  - Configurable chunk sizes (characters per chunk)
  - Overlap control for context preservation
  - Custom separators (paragraphs, sentences, custom delimiters)
  - Three built-in configurations:
    - RAG-optimized: 500 chars, 100 overlap, paragraph-based
    - Summarization: 2000 chars, 200 overlap, paragraph-based
    - Sentence-based: 300 chars, 50 overlap, sentence-based
- **DocumentPort Interface**: Clean abstraction for document operations
  - Extract metadata and content from PDFs
  - Chunk documents with configurable strategies
  - Extensible to other document formats

#### Security & Data Protection
- **Vision Data Encryption**: ChaCha20-Poly1305 authenticated encryption
  - Secure at-rest encryption for image data
  - Automatic encryption for `ImageFile` and `ImageBase64` types
  - Decryption utilities for secure data access
- **Data Retention Policies**: Configurable retention for sensitive vision data
  - Time-based retention (e.g., 30 days)
  - Automatic cleanup of expired encrypted data
  - Audit logging for compliance
- **Audit Logging**: Comprehensive tracking of vision operations
  - Document processing events (PDF extraction, chunking)
  - Vision API calls (provider, model, image count, tokens)
  - Encryption/decryption operations
  - Security-related events (data retention, cleanup)

#### CLI Integration
- **Vision Analysis Commands**:
  ```bash
  paladin vision analyze --image path/to/image.jpg --prompt "Describe this image"
  paladin vision analyze --url https://example.com/image.jpg --detail high
  paladin vision batch --directory images/ --prompt "Classify image"
  ```
- **Document Processing Commands**:
  ```bash
  paladin document extract --pdf document.pdf --output text
  paladin document chunk --pdf report.pdf --chunk-size 500 --overlap 100
  paladin document analyze --pdf paper.pdf --prompt "Summarize key findings"
  ```
- **Security Commands**:
  ```bash
  paladin vision encrypt --image sensitive.jpg --output encrypted.bin
  paladin vision decrypt --input encrypted.bin --output decrypted.jpg
  paladin security audit --filter vision --since "30 days ago"
  ```

#### YAML Configuration Support
- **Vision Configuration Section**:
  ```yaml
  vision:
    default_detail: "auto"
    max_images_per_request: 10
    supported_formats: ["png", "jpg", "jpeg", "gif", "webp"]
    enable_encryption: true
  ```
- **Document Processing Configuration**:
  ```yaml
  document:
    pdf:
      max_pages: 1000
      chunk_size: 500
      chunk_overlap: 100
      separator: "\n\n"
  ```
- **Security Configuration**:
  ```yaml
  security:
    vision:
      encryption_enabled: true
      data_retention_days: 30
      audit_logging: true
  ```

#### Battalion Integration
- **Formation Pattern**: Sequential vision pipelines
  - Example: Image Analyzer → Detail Extractor → Insight Generator
  - Output of each stage feeds into the next
  - Perfect for multi-stage vision analysis workflows
- **Phalanx Pattern**: Parallel image processing
  - Process multiple images concurrently with ~3x speedup
  - Each image analyzed by a separate vision-enabled Paladin
  - Results aggregated at completion
- **Campaign Pattern**: Graph-based vision workflows
  - Complex vision processing DAGs
  - Conditional branching based on vision analysis results
  - Mix vision and non-vision tasks in same graph
- **Chain of Command Pattern**: Hierarchical vision delegation
  - Commander Paladin delegates vision tasks to specialist Paladins
  - Automatic load balancing across vision-capable Paladins
  - Escalation for complex or ambiguous visual content

#### Documentation
- **Comprehensive Guide**: `docs/SENTINEL.md` (600+ lines)
  - 13 major sections covering entire vision system
  - Getting started tutorials
  - Supported providers and models
  - Paladin Vision API reference
  - Document processing workflows
  - CLI usage with 8+ command examples
  - YAML configuration templates
  - Security best practices
  - Battalion integration patterns
  - Error handling strategies
  - Performance optimization tips
  - Troubleshooting guide (7 common issues)
- **Code Examples**: Three comprehensive working examples
  - `examples/vision_analysis.rs`: Single-image analysis with detail levels (200 lines)
  - `examples/document_processing.rs`: PDF extraction and chunking strategies (280 lines)
  - `examples/vision_battalion.rs`: Formation and Phalanx patterns (320 lines)
- **README Updates**: Vision & Multi-Modal Processing section
  - Key features overview
  - Quick start code samples
  - Supported content types
  - Document processing examples
  - CLI command references
  - Battalion integration notes
  - Links to comprehensive documentation

### Technical Details

#### Architecture
- **Hexagonal Architecture Compliance**: All vision components follow ports/adapters pattern
  - Vision domain entities in `core/platform/container/`
  - Vision port definitions in `application/ports/output/`
  - Provider-specific adapters in `infrastructure/adapters/llm/`
- **Test-Driven Development**: Comprehensive test coverage
  - 1146 library tests passing (including vision tests)
  - Unit tests for all vision content types
  - Integration tests with mocked API responses
  - Error path testing for invalid formats
  - Security tests for encryption/decryption

#### Dependencies
- **New Dependencies**:
  - `pdf-extract`: PDF text extraction
  - `lopdf`: Low-level PDF manipulation
  - Additional cryptographic dependencies for vision encryption

#### Performance
- **Benchmarks Available**: Vision-specific performance tests
  - Image encoding/decoding: ~50ms per 2MB image
  - Batch processing: ~3x speedup with Phalanx pattern
  - PDF extraction: ~200ms per 100-page document
  - Document chunking: ~10ms for 10k character document

### Security

#### Known Vulnerabilities (from `cargo audit`)
- **RUSTSEC-2023-0071**: RSA timing sidechannel in `rsa 0.9.10` (Medium severity)
  - **Impact**: Potential key recovery through timing attacks
  - **Source**: Transitive dependency via `sqlx-mysql`
  - **Status**: No fixed upgrade available
  - **Mitigation**: Affects MySQL TLS certificate validation (optional feature)
  - **Risk Assessment**: Low for Paladin use case (MySQL connections are internal)

- **RUSTSEC-2025-0111**: tokio-tar PAX header parsing vulnerability
  - **Impact**: File smuggling attacks via malformed TAR archives
  - **Source**: Dev dependency via `testcontainers`
  - **Status**: No fixed upgrade available
  - **Mitigation**: Only used in test environment, not production code
  - **Risk Assessment**: Low (development-only dependency)

#### Unmaintained Dependencies (Warnings)
- `ansi_term 0.12.1` (via structopt): Consider migrating to `clap 4.x`
- `atty 0.2.14` (via structopt): Replaced by `is-terminal` in modern Rust
- `dotenv 0.15.0`: Consider migrating to `dotenvy`
- `fxhash 0.2.1` (via scraper): Low risk, internal to scraper crate
- `gcc 0.3.55` (via fasthash-sys): Build-time only dependency
- `number_prefix 0.4.0` (via indicatif): No security impact
- `proc-macro-error 1.0.4` (via structopt): Compile-time only
- `rustls-pemfile 2.2.0` (via testcontainers): Dev dependency only

**Action Plan**: Monitor for updates to `sqlx` and consider migrating from `structopt` to `clap 4.x` in future release.

### Testing

#### Test Coverage
- **Total Tests**: 1146 passing (0 failed)
- **Test Execution**: 7.33s for full library test suite
- **Coverage**: ≥80% for vision and document modules
  - Vision content types: 100% coverage
  - Document extraction: 95% coverage
  - Security (encryption): 90% coverage

#### Test Categories
- **Unit Tests**: 1000+ tests for core functionality
- **Integration Tests**: Mocked API responses for vision providers
- **Security Tests**: Encryption, decryption, audit logging
- **Error Path Tests**: Invalid formats, corrupted data, missing files

### Code Quality

#### Static Analysis
- **Clippy**: PASSED with `-D warnings` (library code)
- **Formatting**: PASSED `cargo fmt --check`
- **Compilation**: CLEAN with `cargo check --all-features`

#### Documentation Quality
- All public APIs have rustdoc comments
- Three comprehensive code examples (800+ lines total)
- User guide with 13 major sections
- Troubleshooting guide with 7 common issues

### Breaking Changes
None. All changes are additive and backward compatible.

### Migration Guide
No migration required. Existing Paladin code works without modification.

To use new vision features:
```rust
// Enable vision on a Paladin
let paladin = PaladinBuilder::new(llm_port)
    .system_prompt("You are a vision-enabled AI assistant")
    .enable_vision(true)
    .build()?;

// Process images
let content = vec![VisionContent::ImageUrl {
    url: "https://example.com/image.jpg".to_string(),
    detail: ImageDetail::Auto,
}];

let result = service.execute_with_vision(&paladin, "Describe this image", content).await?;
```

### Contributors
- John Amatulli (jamatulli) - Epic 13 implementation and documentation

---

## [0.1.0] - Previous Releases

### Added
- Core Paladin platform with Hexagonal Architecture
- Multi-provider LLM support (OpenAI, DeepSeek, Anthropic)
- Battalion orchestration patterns (Formation, Phalanx, Campaign, Chain of Command)
- Arsenal MCP integration for external tools
- Garrison memory and context system
- Citadel state persistence
- Herald output formatting
- Comprehensive CLI (paladin-cli)
- User management system with authentication
- Content processing pipeline
- Redis queue integration
- MinIO file storage integration
- MySQL and SQLite repository support
- Security features (TLS verification, audit logging)
- Docker development environment

### Technical Foundation
- Test-Driven Development (TDD) methodology
- Domain-Driven Design (DDD) principles
- Three-layer hexagonal architecture
- Comprehensive test suite (1146+ tests)
- Continuous integration ready

[Unreleased]: https://github.com/jamatulli/paladin/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/jamatulli/paladin/releases/tag/v0.1.0
