# Task List: Epic 17.5 - CLI Consolidation

Consolidate duplicate CLI directories (`src/cli/` → `src/application/cli/`) to establish consistent architecture, eliminate duplicate functionality, and align with hexagonal architecture principles.

## Relevant Files

### Files to Migrate (from `src/cli/`)
- `src/cli/mod.rs` - Original CLI module root (will be merged)
- `src/cli/commands/agent.rs` - Agent command implementation
- `src/cli/commands/battalion.rs` - Battalion command implementation
- `src/cli/commands/arsenal.rs` - Arsenal command implementation
- `src/cli/commands/maneuver.rs` - Maneuver command implementation
- `src/cli/commands/mod.rs` - Commands module root
- `src/cli/config/loader.rs` - Configuration loader from YAML files
- `src/cli/config/paladin_config.rs` - Paladin configuration structures
- `src/cli/config/battalion_config.rs` - Battalion configuration structures
- `src/cli/config/mod.rs` - Config module root
- `src/cli/output/formatter.rs` - Output formatting utilities (will be replaced by Epic 18 formatters)
- `src/cli/output/errors.rs` - CLI error types (will be merged with application/cli/error.rs)
- `src/cli/output/mod.rs` - Output module root
- `src/cli/templates/paladin_template.rs` - Paladin YAML template generation
- `src/cli/templates/battalion_template.rs` - Battalion YAML template generation
- `src/cli/templates/mod.rs` - Templates module root
- `src/cli/interactive.rs` - Interactive utilities
- `src/cli/user_commands.rs` - User management commands

### Files to Keep and Enhance (from `src/application/cli/`)
- `src/application/cli/mod.rs` - New CLI module root (will be the main module)
- `src/application/cli/error.rs` - Unified CLI error types (will be enhanced)
- `src/application/cli/commands/onboarding.rs` - Onboarding wizard (Epic 18)
- `src/application/cli/commands/setup_check.rs` - Setup checker (Epic 18)
- `src/application/cli/commands/features.rs` - Feature management (Epic 18)
- `src/application/cli/commands/muster.rs` - Muster command (Epic 18)
- `src/application/cli/commands/council.rs` - Council command (Epic 18)
- `src/application/cli/commands/mod.rs` - Commands module root (will be enhanced)
- `src/application/cli/formatters/output.rs` - Rich output formatter (Epic 18)
- `src/application/cli/formatters/table.rs` - Table formatting (Epic 18)
- `src/application/cli/formatters/progress.rs` - Progress indicators (Epic 18)
- `src/application/cli/formatters/mod.rs` - Formatters module root
- `src/application/cli/interactive/prompts.rs` - Interactive prompts (Epic 18)
- `src/application/cli/interactive/wizard.rs` - Wizard framework (Epic 18)
- `src/application/cli/interactive/mod.rs` - Interactive module root
- `src/application/cli/templates/env.rs` - Environment template (Epic 18)
- `src/application/cli/templates/mod.rs` - Templates module root (will be enhanced)

### Files to Update (Import Path Changes)
- `src/bin/paladin-cli.rs` - Binary entry point (update imports)
- `src/lib.rs` - Library root (update module structure)
- `tests/integration/cli_agent_test.rs` - Agent CLI integration tests
- `tests/integration/cli_battalion_test.rs` - Battalion CLI integration tests
- `tests/integration/cli_arsenal_test.rs` - Arsenal CLI integration tests
- `tests/integration/cli_maneuver_test.rs` - Maneuver CLI integration tests
- `tests/functional/cli_workflow_test.rs` - CLI workflow tests
- All files importing from `paladin::cli::*` (50+ files estimated)

### New Files to Create
- `src/application/cli/config/mod.rs` - Config module in new location
- `src/application/cli/config/loader.rs` - Migrated config loader
- `src/application/cli/config/paladin_config.rs` - Migrated Paladin config
- `src/application/cli/config/battalion_config.rs` - Migrated Battalion config
- `src/application/cli/commands/agent.rs` - Migrated agent commands
- `src/application/cli/commands/battalion.rs` - Migrated battalion commands
- `src/application/cli/commands/arsenal.rs` - Migrated arsenal commands
- `src/application/cli/commands/maneuver.rs` - Migrated maneuver commands
- `src/application/cli/commands/user.rs` - Migrated user commands
- `src/application/cli/templates/paladin_template.rs` - Migrated Paladin template
- `src/application/cli/templates/battalion_template.rs` - Migrated Battalion template

### Notes

- This is a **refactoring task**, not a feature addition
- All functionality must remain intact - no behavioral changes
- Focus on **moving files** and **updating imports**, not rewriting logic
- The Epic 18 formatters and wizard framework are **superior** - keep them, discard old formatter
- Two CliError types must be **unified** into one in `src/application/cli/error.rs`
- Use `cargo check` frequently to catch import errors early
- Run `cargo test` after each major migration step
- Follow **Red-Green-Refactor**: tests should pass before and after each step

## Instructions for Completing Tasks

**IMPORTANT:** As you complete each task, you must check it off in this markdown file by changing `- [ ]` to `- [x]`. This helps track progress and ensures you don't skip any steps.

Example:

- `- [ ] 1.1 Read file` → `- [x] 1.1 Read file` (after completing)

Update the file after completing each sub-task, not just after completing an entire parent task.

## Tasks

- [x] 0.0 Create feature branch for CLI consolidation
  - [x] 0.1 Check current git status and ensure working directory is clean
  - [x] 0.2 Create and checkout new branch: `git checkout -b refactor/epic-17-5-consolidate-cli-directories`
  - [x] 0.3 Verify branch creation with `git branch --show-current`

- [x] 1.0 Audit and document current CLI structure
  - [x] 1.1 List all files in `src/cli/` directory with `find src/cli -type f -name "*.rs"`
  - [x] 1.2 List all files in `src/application/cli/` directory with `find src/application/cli -type f -name "*.rs"`
  - [x] 1.3 Search for all imports from `paladin::cli::` with `grep -r "use.*paladin::cli::" src/ tests/ --include="*.rs" | wc -l`
  - [x] 1.4 Search for all imports from `paladin::application::cli::` with `grep -r "use.*paladin::application::cli::" src/ tests/ --include="*.rs" | wc -l`
  - [x] 1.5 Identify files using both CliError types (search for `cli::output::errors::CliError` and `application::cli::error::CliError`)
  - [x] 1.6 Document findings: create a mapping of which files need import updates

- [x] 2.0 Consolidate error handling into unified CliError
  - [x] 2.1 Read `src/cli/output/errors.rs` to understand old CliError structure
  - [x] 2.2 Read `src/application/cli/error.rs` to understand new CliError structure
  - [x] 2.3 Identify all unique error variants from both error types
  - [x] 2.4 Design unified CliError enum in `src/application/cli/error.rs` that includes all variants
  - [x] 2.5 Add conversion traits: `impl From<crate::cli::output::errors::CliError> for CliError`
  - [x] 2.6 Add conversion traits: `impl From<std::io::Error> for CliError`
  - [x] 2.7 Add conversion traits: `impl From<serde_yaml::Error> for CliError`
  - [x] 2.8 Add conversion traits for any other error types used in old CLI
  - [x] 2.9 Update `CliResult<T>` type alias to use unified error
  - [x] 2.10 Write unit tests for error conversions in `src/application/cli/error.rs`
  - [x] 2.11 Run `cargo test` to verify error handling tests pass

- [x] 3.0 Migrate command modules to application/cli/commands/
  - [x] 3.1 Create `src/application/cli/commands/agent.rs` by copying from `src/cli/commands/agent.rs`
  - [x] 3.2 Update imports in `agent.rs` to use `crate::application::cli::error::{CliError, CliResult}`
  - [x] 3.3 Update any formatter imports in `agent.rs` to use `crate::application::cli::formatters`
  - [x] 3.4 Update config imports in `agent.rs` to use `crate::application::cli::config` (will be migrated next)
  - [x] 3.5 Create `src/application/cli/commands/battalion.rs` by copying from `src/cli/commands/battalion.rs`
  - [x] 3.6 Update imports in `battalion.rs` following same pattern as agent.rs
  - [x] 3.7 Create `src/application/cli/commands/arsenal.rs` by copying from `src/cli/commands/arsenal.rs`
  - [x] 3.8 Update imports in `arsenal.rs` following same pattern
  - [x] 3.9 Create `src/application/cli/commands/maneuver.rs` by copying from `src/cli/commands/maneuver.rs`
  - [x] 3.10 Update imports in `maneuver.rs` following same pattern
  - [x] 3.11 Create `src/application/cli/commands/user.rs` by copying from `src/cli/user_commands.rs`
  - [x] 3.12 Update imports in `user.rs` following same pattern
  - [x] 3.13 Update `src/application/cli/commands/mod.rs` to export all new command modules
  - [x] 3.14 Add `pub mod agent;`, `pub mod battalion;`, `pub mod arsenal;`, `pub mod maneuver;`, `pub mod user;`
  - [x] 3.15 Run `cargo check` to verify command modules compile (blocked: needs config and templates modules migrated first)

- [x] 4.0 Migrate configuration modules to application/cli/config/
  - [x] 4.1 Create `src/application/cli/config/` directory
  - [x] 4.2 Create `src/application/cli/config/mod.rs` with module exports
  - [x] 4.3 Create `src/application/cli/config/loader.rs` by copying from `src/cli/config/loader.rs`
  - [x] 4.4 Update imports in `loader.rs` to use unified CliError
  - [x] 4.5 Update any file path handling to use consistent error conversion
  - [x] 4.6 Create `src/application/cli/config/paladin_config.rs` by copying from `src/cli/config/paladin_config.rs`
  - [x] 4.7 Update imports in `paladin_config.rs` to use `crate::application::cli::error`
  - [x] 4.8 Update serialization error handling to use unified CliError
  - [x] 4.9 Create `src/application/cli/config/battalion_config.rs` by copying from `src/cli/config/battalion_config.rs`
  - [x] 4.10 Update imports in `battalion_config.rs` following same pattern
  - [x] 4.11 Update `src/application/cli/config/mod.rs` to export: `pub mod loader;`, `pub mod paladin_config;`, `pub mod battalion_config;`
  - [x] 4.12 Go back to command modules (agent, battalion, arsenal, maneuver) and update config import paths
  - [x] 4.13 Run `cargo check` to verify config modules and commands compile together

- [ ] 5.0 Consolidate formatters and output handling
  - [ ] 5.1 Read `src/cli/output/formatter.rs` to identify any functions not present in Epic 18 formatters
  - [ ] 5.2 Compare with `src/application/cli/formatters/output.rs` to identify gaps
  - [ ] 5.3 If old formatter has unique functions, migrate them to `src/application/cli/formatters/output.rs`
  - [ ] 5.4 Add any missing color formatting functions
  - [ ] 5.5 Add any missing table formatting functions not in `table.rs`
  - [ ] 5.6 Ensure `OutputFormatter` in Epic 18 version has all capabilities of old formatter
  - [ ] 5.7 Update any command files still referencing old formatter to use new `OutputFormatter`
  - [ ] 5.8 Verify all progress indicators use `src/application/cli/formatters/progress.rs`
  - [ ] 5.9 Run `cargo check` to verify formatter consolidation

- [x] 6.0 Migrate templates to application/cli/templates/
  - [x] 6.1 Create `src/application/cli/templates/paladin_template.rs` by copying from `src/cli/templates/paladin_template.rs`
  - [x] 6.2 Update imports in `paladin_template.rs` to use unified CliError
  - [x] 6.3 Update template generation to use `crate::application::cli::config::paladin_config`
  - [x] 6.4 Create `src/application/cli/templates/battalion_template.rs` by copying from `src/cli/templates/battalion_template.rs`
  - [x] 6.5 Update imports in `battalion_template.rs` to use unified CliError
  - [x] 6.6 Update template generation to use `crate::application::cli::config::battalion_config`
  - [x] 6.7 Update `src/application/cli/templates/mod.rs` to add: `pub mod paladin_template;`, `pub mod battalion_template;`
  - [x] 6.8 Verify `env.rs` (Epic 18) remains functional
  - [x] 6.9 Run `cargo check` to verify template modules compile

- [ ] 7.0 Update all import paths throughout codebase
  - [ ] 7.1 Find all files with `use paladin::cli::` imports: `grep -r "use.*paladin::cli::" src/ tests/ --include="*.rs" -l`
  - [ ] 7.2 Create Python script to systematically replace `paladin::cli::` with `paladin::application::cli::`
  - [ ] 7.3 Update imports in `src/bin/paladin-cli.rs` first (critical path)
  - [ ] 7.4 Update imports in all files under `src/application/use_cases/` that reference CLI
  - [ ] 7.5 Update imports in all files under `src/infrastructure/adapters/` that reference CLI
  - [ ] 7.6 Update imports in all test files under `tests/integration/` that test CLI commands
  - [ ] 7.7 Update imports in all test files under `tests/functional/` that test CLI workflows
  - [ ] 7.8 Run `cargo check` after each batch of 10-15 files to catch errors early
  - [ ] 7.9 Fix any remaining import errors identified by compiler
  - [ ] 7.10 Search for any remaining `cli::output::errors::CliError` references and replace with `application::cli::error::CliError`
  - [ ] 7.11 Search for any remaining `cli::output::formatter` references and replace with `application::cli::formatters::output::OutputFormatter`

- [ ] 8.0 Update binary entry point (paladin-cli.rs)
  - [ ] 8.1 Read `src/bin/paladin-cli.rs` to understand current CLI command structure
  - [ ] 8.2 Update all command imports to use `paladin::application::cli::commands::*`
  - [ ] 8.3 Update error handling to use `paladin::application::cli::error::CliError`
  - [ ] 8.4 Update formatter usage to use `paladin::application::cli::formatters::*`
  - [ ] 8.5 Verify all command handlers (agent, battalion, arsenal, maneuver, onboarding, etc.) are properly imported
  - [ ] 8.6 Test binary compilation: `cargo build --bin paladin-cli`
  - [ ] 8.7 Run `cargo check` to verify binary compiles

- [ ] 9.0 Update module structure in src/lib.rs
  - [ ] 9.1 Read `src/lib.rs` to see current module exports
  - [ ] 9.2 Verify `pub mod application;` is present (should already exist)
  - [ ] 9.3 Check if `pub mod cli;` is still exported - if yes, remove it or make it private
  - [ ] 9.4 Add documentation comment explaining CLI is now part of application layer
  - [ ] 9.5 Verify `src/application/mod.rs` exports `pub mod cli;`
  - [ ] 9.6 Run `cargo check` to verify module structure is correct
  - [ ] 9.7 Check for any external crates or examples that might import `paladin::cli::*` directly

- [ ] 10.0 Remove old src/cli/ directory
  - [ ] 10.1 Run `cargo check` one final time to ensure everything compiles
  - [ ] 10.2 Run `cargo test --lib` to ensure unit tests pass
  - [ ] 10.3 Verify no files in the codebase still import from `paladin::cli::` (except possibly deprecated re-exports)
  - [ ] 10.4 Create temporary backup: `cp -r src/cli /tmp/paladin-cli-backup`
  - [ ] 10.5 Remove old CLI directory: `git rm -r src/cli`
  - [ ] 10.6 Run `cargo check` to verify removal didn't break anything
  - [ ] 10.7 If errors occur, restore from backup and investigate remaining dependencies
  - [ ] 10.8 Once clean, remove backup: `rm -rf /tmp/paladin-cli-backup`

- [ ] 11.0 Run full test suite and fix any issues
  - [ ] 11.1 Run unit tests: `cargo test --lib`
  - [ ] 11.2 Fix any failing unit tests related to import paths
  - [ ] 11.3 Fix any failing unit tests related to error type conversions
  - [ ] 11.4 Run integration tests: `cargo test --test '*'`
  - [ ] 11.5 Fix any failing integration tests for CLI commands
  - [ ] 11.6 Run clippy: `cargo clippy --all-targets -- -D warnings`
  - [ ] 11.7 Fix any clippy warnings related to imports or unused code
  - [ ] 11.8 Run formatter: `cargo fmt --all`
  - [ ] 11.9 Run full test suite: `make test-all`
  - [ ] 11.10 Verify all 1400+ tests still pass
  - [ ] 11.11 Run benchmarks if applicable: `cargo bench` (optional)

- [ ] 12.0 Update documentation
  - [ ] 12.1 Update `docs/CLI_USAGE.md` to reflect new import paths in code examples
  - [ ] 12.2 Update any architecture diagrams in `docs/Design/` that show CLI module structure
  - [ ] 12.3 Update `docs/CONTRIBUTING.md` if it mentions CLI development location
  - [ ] 12.4 Add migration note to `CHANGELOG.md` explaining consolidation
  - [ ] 12.5 Update `README.md` if it contains CLI examples with import statements
  - [ ] 12.6 Review `docs/architecture/` for any references to old `src/cli/` structure
  - [ ] 12.7 Update any code comments in `src/application/cli/mod.rs` explaining the consolidation
  - [ ] 12.8 Add rustdoc comments to `src/application/cli/mod.rs` explaining the unified CLI structure
  - [ ] 12.9 Stage documentation changes: `git add docs/ README.md CHANGELOG.md`
  - [ ] 12.10 Commit documentation updates: `git commit -m "docs: update CLI documentation after consolidation"`

- [ ] 13.0 Final verification and commit
  - [ ] 13.1 Run `cargo check` one final time
  - [ ] 13.2 Run `cargo test` one final time
  - [ ] 13.3 Run `cargo clippy -- -D warnings` one final time
  - [ ] 13.4 Run `cargo fmt --check` to verify formatting
  - [ ] 13.5 Review git status: `git status`
  - [ ] 13.6 Stage all changes: `git add -A`
  - [ ] 13.7 Review changes: `git diff --cached --stat`
  - [ ] 13.8 Commit consolidation: `git commit -m "refactor: consolidate CLI directories into application layer" -m "- Migrated src/cli/ modules to src/application/cli/" -m "- Unified CliError types into single error enum" -m "- Updated all import paths throughout codebase" -m "- Consolidated formatters, keeping Epic 18 rich implementation" -m "- Migrated commands: agent, battalion, arsenal, maneuver, user" -m "- Migrated config modules: loader, paladin_config, battalion_config" -m "- Migrated templates: paladin, battalion (kept env from Epic 18)" -m "- Updated binary entry point and module exports" -m "- Removed old src/cli/ directory" -m "- All tests passing (1400+)" -m "Related to Epic 18 CLI Enhancement - architectural cleanup"`
  - [ ] 13.9 Push branch: `git push -u origin refactor/consolidate-cli-directories`
  - [ ] 13.10 Create pull request with summary of consolidation

## Success Criteria

✅ All files from `src/cli/` successfully migrated to `src/application/cli/`
✅ Single unified `CliError` type in `src/application/cli/error.rs`
✅ All import paths updated to use `paladin::application::cli::*`
✅ Old `src/cli/` directory removed from codebase
✅ All tests passing (1400+ unit and integration tests)
✅ No clippy warnings related to consolidation
✅ Code properly formatted with `cargo fmt`
✅ Documentation updated to reflect new structure
✅ Binary entry point (`paladin-cli`) compiles and runs correctly
✅ No behavioral changes - all CLI functionality preserved
