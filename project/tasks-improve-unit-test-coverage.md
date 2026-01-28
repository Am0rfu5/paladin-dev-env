## Relevant Files

- `src/main.rs` - Main entry point with 0% coverage
- `src/main.rs` (tests) - Unit tests for main.rs
- `src/infrastructure/web/user_controller.rs` - User controller with 0% coverage
- `src/infrastructure/web/user_controller.rs` (tests) - Unit tests for user controller
- `src/infrastructure/repositories/sqlite_user_repository.rs` - User repository with 0% coverage
- `src/infrastructure/repositories/sqlite_user_repository.rs` (tests) - Unit tests for user repository
- `src/infrastructure/adapters/file_storage/minio.rs` - MinIO adapter with 0% coverage
- `src/infrastructure/adapters/file_storage/minio.rs` (tests) - Unit tests for MinIO adapter
- `src/infrastructure/adapters/llm/openai_adapter.rs` - OpenAI adapter with low coverage
- `src/infrastructure/adapters/llm/openai_adapter.rs` (tests) - Unit tests for OpenAI adapter
- `src/infrastructure/adapters/llm/anthropic_adapter.rs` - Anthropic adapter with low coverage
- `src/infrastructure/adapters/llm/anthropic_adapter.rs` (tests) - Unit tests for Anthropic adapter
- `src/infrastructure/adapters/llm/deepseek_adapter.rs` - DeepSeek adapter with low coverage
- `src/infrastructure/adapters/llm/deepseek_adapter.rs` (tests) - Unit tests for DeepSeek adapter
- `src/application/use_cases/content/content_ingestion_service.rs` - Content ingestion with low coverage
- `src/application/use_cases/content/content_ingestion_service.rs` (tests) - Unit tests for content ingestion
- `src/cli/commands/agent.rs` - Agent command with low coverage
- `src/cli/commands/agent.rs` (tests) - Unit tests for agent command
- `src/cli/commands/arsenal.rs` - Arsenal command with low coverage
- `src/cli/commands/arsenal.rs` (tests) - Unit tests for arsenal command
- `src/cli/commands/battalion.rs` - Battalion command with low coverage
- `src/cli/commands/battalion.rs` (tests) - Unit tests for battalion command
- `src/cli/interactive.rs` - Interactive CLI with low coverage
- `src/cli/interactive.rs` (tests) - Unit tests for interactive CLI
- `src/config/setup/service_runner.rs` - Service runner with low coverage
- `src/config/setup/service_runner.rs` (tests) - Unit tests for service runner
- `src/config/user_config.rs` - User config with 0% coverage
- `src/config/user_config.rs` (tests) - Unit tests for user config
- `src/config/setup/mod.rs` - Setup mod with 0% coverage
- `src/config/setup/mod.rs` (tests) - Unit tests for setup mod
- `src/application/use_cases/content/content_aggregator_service.rs` - Content aggregator with 0% coverage
- `src/application/use_cases/content/content_aggregator_service.rs` (tests) - Unit tests for content aggregator
- `src/cli/user_commands.rs` - User commands with 0% coverage
- `src/cli/user_commands.rs` (tests) - Unit tests for user commands

### Notes

- Unit tests should be placed in `#[cfg(test)]` modules within the same files they are testing, following Rust conventions.
- Use `cargo test` to run tests. Use `cargo llvm-cov` to check coverage.
- Focus on mocking external dependencies (e.g., HTTP clients, databases) to isolate unit tests.
- Leverage existing mock adapters in the codebase where available.
- Do not change the fundamental design of the code you are testing.

### Current Progress

**Overall Coverage**: 69.36% regions / 67.12% lines (Target: >85%)
**Completed Tasks**: 6/28 (21.4%)
- ✅ 0.0 Create feature branch
- ✅ 1.0 Analyze coverage and prioritize files  
- ✅ 2.1 Add unit tests for src/main.rs (3 tests, improved from 0% to ~48.65%)
- ✅ 2.2 Add unit tests for src/infrastructure/web/user_controller.rs (15 tests, improved from 0% to ~78.10%)
- ✅ 2.3 Add unit tests for src/infrastructure/repositories/sqlite_user_repository.rs (14 tests, improved from 0% to ~89.77%)
- ✅ 2.4 Add unit tests for src/config/user_config.rs (3 tests, improved from 0% to ~47.75% regions / 45.21% lines)

**Next Priority**: Continue with 0% coverage files (2.5-2.7) before low coverage files

## Instructions for Completing Tasks

**IMPORTANT:** As you complete each task, you must check it off in this markdown file by changing `- [ ]` to `- [x]`. This helps track progress and ensures you don't skip any steps.

Example:

- `- [ ] 1.1 Read file` → `- [x] 1.1 Read file` (after completing)

Update the file after completing each sub-task, not just after completing an entire parent task.

## Tasks

- [x] 0.0 Create feature branch
  - [x] 0.1 Create and checkout a new branch for this feature (e.g., `git checkout -b feature/improve-unit-test-coverage`)
- [x] 1.0 Analyze current coverage report and identify priority files
  - [x] 1.1 Review the current coverage report from `cargo llvm-cov` output
  - [x] 1.2 Identify all files with 0% coverage (e.g., main.rs, user_controller.rs, sqlite_user_repository.rs)
  - [x] 1.3 Identify files with low coverage (<50%) (e.g., LLM adapters, CLI commands, repositories)
  - [x] 1.4 Identify files with moderate coverage (50-80%) that need improvement (e.g., some use cases, managers)
  - [x] 1.5 Prioritize files based on criticality: core > application > infrastructure
- [ ] 2.0 Add unit tests for files with 0% coverage
  - [x] 2.1 Add unit tests for src/main.rs (main function and basic setup)
  - [x] 2.2 Add unit tests for src/infrastructure/web/user_controller.rs (HTTP handlers and routing)
  - [x] 2.3 Add unit tests for src/infrastructure/repositories/sqlite_user_repository.rs (database operations)
  - [x] 2.4 Add unit tests for src/config/user_config.rs (configuration loading) - **COMPLETED**: Added comprehensive unit tests with mock LogPort and NotificationService, improved coverage from 0% to ~47.75% regions / 45.21% lines
  - [ ] 2.5 Add unit tests for src/config/setup/mod.rs (setup module functions)
  - [ ] 2.6 Add unit tests for src/application/use_cases/content/content_aggregator_service.rs (content aggregation logic)
  - [ ] 2.7 Add unit tests for src/cli/user_commands.rs (CLI command parsing and execution)
- [ ] 3.0 Add unit tests for files with low coverage (<50%)
  - [ ] 3.1 Add unit tests for src/infrastructure/adapters/file_storage/minio.rs (file storage operations)
  - [ ] 3.2 Add unit tests for src/infrastructure/adapters/llm/openai_adapter.rs (LLM API calls and responses)
  - [ ] 3.3 Add unit tests for src/infrastructure/adapters/llm/anthropic_adapter.rs (LLM API calls and responses)
  - [ ] 3.4 Add unit tests for src/infrastructure/adapters/llm/deepseek_adapter.rs (LLM API calls and responses)
  - [ ] 3.5 Add unit tests for src/cli/commands/agent.rs (agent command logic)
  - [ ] 3.6 Add unit tests for src/cli/commands/arsenal.rs (arsenal command logic)
  - [ ] 3.7 Add unit tests for src/cli/commands/battalion.rs (battalion command logic)
  - [ ] 3.8 Add unit tests for src/cli/interactive.rs (interactive CLI functionality)
  - [ ] 3.9 Add unit tests for src/config/setup/service_runner.rs (service initialization)
- [ ] 4.0 Add unit tests for moderate coverage files (50-80%)
  - [ ] 4.1 Add unit tests for src/application/use_cases/content/content_ingestion_service.rs (missing error paths and edge cases)
  - [ ] 4.2 Add unit tests for src/infrastructure/adapters/input/news_api_fetcher.rs (API fetching logic)
  - [ ] 4.3 Add unit tests for src/infrastructure/adapters/input/tensorflow_adapter.rs (ML model integration)
  - [ ] 4.4 Add unit tests for src/core/platform/manager/notification_service.rs (notification handling)
  - [ ] 4.5 Add unit tests for src/core/platform/manager/orchestrator.rs (orchestration logic)
  - [ ] 4.6 Add unit tests for src/core/platform/manager/queue_service.rs (queue operations)
  - [ ] 4.7 Add unit tests for src/core/platform/manager/scheduler.rs (scheduling functionality)
- [ ] 5.0 Ensure critical paths in core functionality are covered
  - [ ] 5.1 Verify unit tests cover all critical paths in Paladin execution (src/core/platform/container/paladin.rs)
  - [ ] 5.2 Verify unit tests cover all critical paths in Battalion orchestration (formation, phalanx, campaign, chain of command)
  - [ ] 5.3 Verify unit tests cover all critical paths in Arsenal tool execution
  - [ ] 5.4 Verify unit tests cover all critical paths in Garrison memory operations
  - [ ] 5.5 Verify unit tests cover all critical paths in Citadel state persistence
  - [ ] 5.6 Add tests for error handling and edge cases in all critical paths
- [ ] 6.0 Run tests and verify coverage improvements
  - [ ] 6.1 Run `cargo test` to ensure all new tests pass
  - [ ] 6.2 Run `cargo llvm-cov` to generate updated coverage report
  - [ ] 6.3 Verify overall coverage exceeds 85%
  - [ ] 6.4 Verify no decrease in coverage for previously well-covered modules
- [ ] 7.0 Address any failing tests or regressions
  - [ ] 7.1 Identify and fix any test failures from new tests
  - [ ] 7.2 Address any regressions in existing functionality
  - [ ] 7.3 Refactor tests if needed for better isolation or maintainability
  - [ ] 7.4 Re-run tests and coverage after fixes
- [ ] 8.0 Final coverage verification and cleanup
  - [ ] 8.1 Run final `cargo test` and `cargo llvm-cov` to confirm >85% coverage
  - [ ] 8.2 Run `cargo fmt` and `cargo clippy` to ensure code quality
  - [ ] 8.3 Remove any temporary debug code or unused test utilities
  - [ ] 8.4 Commit changes with descriptive message following conventional commits