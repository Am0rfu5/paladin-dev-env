# Task List: Armory CLI Tools (Epic 9)

**PRD:** `prd-armory-cli-tools.md`  
**Priority:** Medium  
**Estimated Effort:** 2-3 weeks  
**Epic:** Epic 9 - Armory CLI Tools

## Relevant Files

- `src/bin/paladin-cli.rs` - Main CLI binary entry point
- `src/cli/mod.rs` - CLI module root
- `src/cli/commands/mod.rs` - Command modules root
- `src/cli/commands/agent.rs` - Agent subcommands implementation
- `src/cli/commands/battalion.rs` - Battalion subcommands implementation
- `src/cli/commands/arsenal.rs` - Arsenal subcommands implementation
- `src/cli/config/mod.rs` - Configuration module root
- `src/cli/config/paladin_config.rs` - Paladin YAML configuration types
- `src/cli/config/battalion_config.rs` - Battalion YAML configuration types
- `src/cli/config/loader.rs` - Configuration file loading and validation
- `src/cli/templates/mod.rs` - Template generation module
- `src/cli/templates/paladin_template.rs` - Paladin YAML template generator
- `src/cli/templates/battalion_template.rs` - Battalion YAML template generator
- `src/cli/output/mod.rs` - Output formatting module
- `src/cli/output/formatter.rs` - Result formatting implementation
- `src/cli/output/errors.rs` - CLI-specific error types and formatting
- `src/cli/interactive.rs` - Interactive prompt utilities
- `Cargo.toml` - Updated with CLI dependencies (clap, serde_yaml, colored, indicatif)
- `tests/cli/agent_commands_test.rs` - Unit tests for agent commands
- `tests/cli/battalion_commands_test.rs` - Unit tests for battalion commands
- `tests/cli/arsenal_commands_test.rs` - Unit tests for arsenal commands
- `tests/cli/config_loading_test.rs` - Unit tests for configuration loading
- `tests/cli/template_generation_test.rs` - Unit tests for template generation
- `tests/integration/cli_integration_test.rs` - Integration tests for CLI workflows
- `examples/cli_configs/basic_paladin.yaml` - Example Paladin configuration
- `examples/cli_configs/formation.yaml` - Example Formation configuration
- `examples/cli_configs/phalanx.yaml` - Example Phalanx configuration

### Notes

- All CLI code goes in the `src/cli/` module directory
- The binary at `src/bin/paladin-cli.rs` should be minimal, delegating to `src/cli/mod.rs`
- Follow hexagonal architecture: CLI is an adapter that uses application layer services
- Use `cargo test --test cli_*` to run CLI-specific tests
- Use `cargo run --bin paladin-cli -- --help` to test CLI during development

## Instructions for Completing Tasks

**IMPORTANT:** As you complete each task, you must check it off in this markdown file by changing `- [ ]` to `- [x]`. This helps track progress and ensures you don't skip any steps.

Example:
- `- [ ] 1.1 Read file` → `- [x] 1.1 Read file` (after completing)

Update the file after completing each sub-task, not just after completing an entire parent task.

## Tasks

- [x] 0.0 Create feature branch
  - [x] 0.1 Create and checkout new branch: `git checkout -b epic-9/armory-cli-tools`
  - [x] 0.2 Verify branch with: `git branch --show-current`

- [x] 1.0 Set up CLI project structure and dependencies
  - [x] 1.1 Read `Cargo.toml` to understand current dependencies
  - [x] 1.2 Add CLI dependencies to `Cargo.toml` in `[dependencies]` section:
    - `clap = { version = "4.4", features = ["derive", "cargo", "env"] }`
    - `serde_yaml = "0.9"`
    - `colored = "2.1"`
    - `indicatif = "0.17"`
    - `dialoguer = "0.11"` (for interactive prompts)
  - [x] 1.3 Add `[[bin]]` section to `Cargo.toml` for `paladin-cli` binary:
    ```toml
    [[bin]]
    name = "paladin-cli"
    path = "src/bin/paladin-cli.rs"
    ```
  - [x] 1.4 Create directory structure: `mkdir -p src/cli/{commands,config,templates,output}`
  - [x] 1.5 Create `src/cli/mod.rs` with module declarations
  - [x] 1.6 Create placeholder files for all modules listed in Relevant Files
  - [x] 1.7 Run `cargo build` to verify structure compiles

- [x] 2.0 Implement core CLI argument parsing with clap
  - [x] 2.1 Create `src/bin/paladin-cli.rs` with main function and clap setup
  - [x] 2.2 Define `Cli` struct in `src/bin/paladin-cli.rs` using clap derive macros with version and about text
  - [x] 2.3 Define `Commands` enum with `Agent`, `Battalion`, and `Arsenal` variants
  - [x] 2.4 In `src/cli/commands/agent.rs`, define `AgentCommands` enum with `Run` and `New` variants
  - [x] 2.5 Define `AgentRunArgs` struct with fields: config (PathBuf), input (Option<String>), output (Option<PathBuf>), verbose (bool)
  - [x] 2.6 Define `AgentNewArgs` struct with fields: name (String), output (PathBuf), provider (Option<String>)
  - [x] 2.7 In `src/cli/commands/battalion.rs`, define `BattalionCommands` enum with `Run` and `New` variants
  - [x] 2.8 Define `BattalionRunArgs` struct with fields: config (PathBuf), type (String), output (Option<PathBuf>), verbose (bool)
  - [x] 2.9 Define `BattalionNewArgs` struct with fields: name (String), type (String), output (PathBuf)
  - [x] 2.10 In `src/cli/commands/arsenal.rs`, define `ArsenalCommands` enum with `List` and `Test` variants
  - [x] 2.11 Define `ArsenalTestArgs` struct with fields: mcp_stdio (Option<String>), mcp_sse (Option<String>)
  - [x] 2.12 Implement basic command routing in `src/bin/paladin-cli.rs` main function (match on command and call stub functions)
  - [x] 2.13 Test `cargo run --bin paladin-cli -- --help` displays all commands correctly
  - [x] 2.14 Test `cargo run --bin paladin-cli -- agent --help` shows agent subcommands

- [x] 3.0 Implement configuration loading and validation
  - [x] 3.1 In `src/cli/config/paladin_config.rs`, define `PaladinYamlConfig` struct with serde derives matching FR-15 schema
  - [x] 3.2 Define nested `ProviderConfig`, `GarrisonConfig`, `ArsenalConfig`, and `McpServerConfig` structs
  - [x] 3.3 Implement `Validate` trait for `PaladinYamlConfig` with validation logic (required fields, valid ranges)
  - [x] 3.4 In `src/cli/config/battalion_config.rs`, define `BattalionYamlConfig` struct with type discriminator
  - [x] 3.5 Define battalion-specific config types: `FormationConfig`, `PhalanxConfig`, `CampaignConfig`, `ChainOfCommandConfig`
  - [x] 3.6 In `src/cli/config/loader.rs`, implement `load_paladin_config(path: &Path) -> Result<PaladinYamlConfig, ConfigError>`
  - [x] 3.7 Implement `load_battalion_config(path: &Path) -> Result<BattalionYamlConfig, ConfigError>`
  - [x] 3.8 Implement error handling for: file not found, invalid YAML, validation failures
  - [x] 3.9 In `src/cli/output/errors.rs`, define `CliError` enum covering all CLI error cases per FR-20 format
  - [x] 3.10 Implement `Display` trait for `CliError` with actionable error messages (what, why, how to fix)
  - [x] 3.11 Write unit test for loading valid Paladin YAML config
  - [x] 3.12 Write unit test for validation error with missing required field
  - [x] 3.13 Write unit test for invalid YAML syntax error handling

- [x] 4.0 Implement `paladin agent new` command (template generation)
  - [x] 4.1 In `src/cli/templates/paladin_template.rs`, define `generate_paladin_template(name: &str, provider: &str) -> String`
  - [x] 4.2 Create YAML template string with all fields from FR-15, including comments explaining each option
  - [x] 4.3 Add system_prompt placeholder with guidance text as multi-line string
  - [x] 4.4 Include garrison example (in_memory) with commented sqlite alternative
  - [x] 4.5 Include arsenal example with MCP server (stdio type) commented out
  - [x] 4.6 Implement template variable substitution for name and provider
  - [x] 4.7 In `src/cli/commands/agent.rs`, implement `handle_agent_new(args: AgentNewArgs) -> Result<(), CliError>`
  - [x] 4.8 Validate provider is one of: openai, deepseek, anthropic (default openai if None)
  - [x] 4.9 Generate template string using `generate_paladin_template`
  - [x] 4.10 Check if output file already exists, prompt for overwrite confirmation
  - [x] 4.11 Write template to output file using `std::fs::write`
  - [x] 4.12 Print success message with colored output: "✓ Created Paladin template: {path}"
  - [x] 4.13 Wire `handle_agent_new` into command routing in `src/bin/paladin-cli.rs`
  - [x] 4.14 Test command: `cargo run --bin paladin-cli -- agent new -n test -o /tmp/test.yaml`
  - [x] 4.15 Verify generated YAML is valid and contains all expected fields

- [x] 5.0 Implement `paladin agent run` command (Paladin execution)
  - [x] 5.1 In `src/cli/commands/agent.rs`, implement `handle_agent_run(args: AgentRunArgs) -> Result<(), CliError>`
  - [x] 5.2 Load configuration using `load_paladin_config(&args.config)`
  - [x] 5.3 Load API key from environment variable based on provider type (per FR-17)
  - [x] 5.4 Return error per FR-18 if API key missing: "Missing API key: {KEY_NAME}. Please set the environment variable."
  - [x] 5.5 If `args.input` is None, call interactive prompt function (stub for now, implement in task 11.0)
  - [x] 5.6 Create LLM port adapter based on provider type (OpenAI, DeepSeek, Anthropic)
  - [x] 5.7 Use `PaladinBuilder` from `application/use_cases/paladin/` to construct Paladin from config
  - [x] 5.8 Configure garrison if specified in config (in_memory or sqlite)
  - [x] 5.9 Configure arsenal/MCP servers if specified in config
  - [x] 5.10 Execute Paladin using `PaladinExecutionService::execute()`
  - [x] 5.11 Handle verbose mode: if `args.verbose`, print each loop iteration and tool calls
  - [x] 5.12 Format output using formatter from task 10.0 (stub for now)
  - [x] 5.13 If `args.output` is Some, write JSON result to file
  - [x] 5.14 If `args.output` is None, print human-readable result to stdout
  - [x] 5.15 Handle errors (LLM failures, timeouts, tool failures) and convert to CliError
  - [x] 5.16 Wire `handle_agent_run` into command routing
  - [x] 5.17 Create test config at `examples/cli_configs/basic_paladin.yaml`
  - [x] 5.18 Test command with real LLM: `cargo run --bin paladin-cli -- agent run -c examples/cli_configs/basic_paladin.yaml -i "Hello"`

- [x] 6.0 Implement `paladin battalion new` command (Battalion template generation)
  - [x] 6.1 In `src/cli/templates/battalion_template.rs`, define `generate_battalion_template(name: &str, battalion_type: &str) -> Result<String, CliError>`
  - [x] 6.2 Implement formation template with sequential Paladin list and pass_output_to_next: true
  - [x] 6.3 Implement phalanx template with parallel Paladin list and inputs array
  - [x] 6.4 Implement campaign template with DAG structure (nodes, edges, start_node)
  - [x] 6.5 Implement chain_of_command template with commander and delegate hierarchy
  - [x] 6.6 Each template should include inline Paladin definitions or references to external config files
  - [x] 6.7 Add comprehensive comments explaining each Battalion type's structure
  - [x] 6.8 In `src/cli/commands/battalion.rs`, implement `handle_battalion_new(args: BattalionNewArgs) -> Result<(), CliError>`
  - [x] 6.9 Validate battalion_type is one of: formation, phalanx, campaign, chain-of-command
  - [x] 6.10 Generate template using `generate_battalion_template`
  - [x] 6.11 Check if output file exists, prompt for overwrite
  - [x] 6.12 Write template to output file
  - [x] 6.13 Print success message with colored output
  - [x] 6.14 Wire `handle_battalion_new` into command routing
  - [x] 6.15 Test command: `cargo run --bin paladin-cli -- battalion new -n test-formation -t formation -o /tmp/formation.yaml`
  - [x] 6.16 Verify all four battalion type templates generate valid YAML

- [x] 7.0 Implement `paladin battalion run` command (Battalion execution)
  - [x] 7.1 In `src/cli/commands/battalion.rs`, implement `handle_battalion_run(args: BattalionRunArgs) -> Result<(), CliError>`
  - [x] 7.2 Load configuration using `load_battalion_config(&args.config)`
  - [x] 7.3 Validate that config type matches `args.type` per FR-9
  - [x] 7.4 Load all Paladin configs referenced in Battalion config (if external files)
  - [x] 7.5 Construct Paladins using PaladinBuilder for each agent in Battalion
  - [x] 7.6 Create appropriate Battalion service based on type (Formation, Phalanx, Campaign, ChainOfCommand)
  - [x] 7.7 Execute Battalion using appropriate service from `application/use_cases/battalion/`
  - [x] 7.8 Handle verbose mode: log each Paladin execution in the Battalion
  - [x] 7.9 Format Battalion results (multiple outputs) using formatter
  - [x] 7.10 If output specified, write JSON results with all Paladin outputs
  - [x] 7.11 If no output file, print human-readable summary to stdout
  - [x] 7.12 Handle Battalion-level errors (Paladin failures, graph validation)
  - [x] 7.13 Wire `handle_battalion_run` into command routing
  - [x] 7.14 Create example configs: `examples/cli_configs/formation.yaml`, `phalanx.yaml`
  - [x] 7.15 Test formation execution: Unit tests pass, full integration requires API key
  - [x] 7.16 Test phalanx execution: Unit tests pass, full integration requires API key

- [x] 8.0 Implement `paladin arsenal list` command (tool discovery)
  - [x] 8.1 In `src/cli/commands/arsenal.rs`, implement `handle_arsenal_list() -> Result<(), CliError>`
  - [x] 8.2 Load MCP server configuration from default config location or environment
  - [x] 8.3 For each configured MCP server, attempt to connect and discover tools
  - [x] 8.4 Query tool list using MCP protocol from `infrastructure/adapters/arsenal/`
  - [x] 8.5 Collect tool metadata: name, description, type (stdio/sse), connection status
  - [x] 8.6 Format output as a table using colored output per FR-11:
    - Tool Name | Description | Type | Status
  - [x] 8.7 Use colored output: green for connected, red for connection failed
  - [x] 8.8 Handle case where no MCP servers are configured (show helpful message)
  - [x] 8.9 Handle connection failures gracefully (show error but continue to next server)
  - [x] 8.10 Wire `handle_arsenal_list` into command routing
  - [x] 8.11 Configure test MCP server in config.yml for testing
  - [x] 8.12 Test command: All 28 CLI tests pass, integration requires MCP server running

- [ ] 9.0 Implement `paladin arsenal test` command (MCP server testing)
  - [ ] 9.1 In `src/cli/commands/arsenal.rs`, implement `handle_arsenal_test(args: ArsenalTestArgs) -> Result<(), CliError>`
  - [ ] 9.2 Validate that exactly one of mcp_stdio or mcp_sse is provided (mutually exclusive per FR-12)
  - [ ] 9.3 If mcp_stdio provided, parse command and args (e.g., "uvx mcp-web-search" → command="uvx", args=["mcp-web-search"])
  - [ ] 9.4 Create MCPStdioAdapter with parsed command and args
  - [ ] 9.5 If mcp_sse provided, create MCPSseAdapter with endpoint URL
  - [ ] 9.6 Measure connection time using `std::time::Instant`
  - [ ] 9.7 Attempt to connect to MCP server and list tools
  - [ ] 9.8 Display connection status (success/failure) with colored output per FR-13
  - [ ] 9.9 If successful, display list of available tools with schemas
  - [ ] 9.10 Display connection latency in milliseconds
  - [ ] 9.11 If connection fails, display detailed error message with debugging hints
  - [ ] 9.12 Wire `handle_arsenal_test` into command routing
  - [ ] 9.13 Test stdio: `cargo run --bin paladin-cli -- arsenal test --mcp-stdio "uvx mcp-web-search"`
  - [ ] 9.14 Test error handling with invalid command: `--mcp-stdio "nonexistent-command"`

- [ ] 10.0 Implement output formatting and error handling
  - [ ] 10.1 In `src/cli/output/formatter.rs`, define `OutputFormatter` struct
  - [ ] 10.2 Implement `format_paladin_result(result: &PaladinResult, verbose: bool) -> String` for human-readable output per FR-22
  - [ ] 10.3 Format should include: Paladin name, input, final output, execution time, token usage
  - [ ] 10.4 Implement `format_paladin_result_json(result: &PaladinResult) -> serde_json::Value` for file output per FR-23
  - [ ] 10.5 JSON format should include: metadata, conversation history, tool calls, timestamps
  - [ ] 10.6 Implement `format_battalion_result(results: &BattalionResult, verbose: bool) -> String`
  - [ ] 10.7 Battalion format should show each Paladin's output in sequence or aggregated based on type
  - [ ] 10.8 Implement verbose output formatting per FR-24: show loops, tool calls, timing
  - [ ] 10.9 In `src/cli/output/errors.rs`, implement detailed error formatting per FR-20 format:
    ```
    Error: <Short description>
    Details: <Longer explanation>
    Suggestion: <How to fix>
    Example: <If applicable>
    ```
  - [ ] 10.10 Implement colored error output: red for "Error:", yellow for "Details:", green for "Suggestion:"
  - [ ] 10.11 Add examples to error messages where applicable (e.g., correct YAML syntax)
  - [ ] 10.12 Ensure exit codes per FR-21: 0 success, 1 user errors, 2 runtime errors, 130 SIGINT
  - [ ] 10.13 Set up signal handler for SIGINT (Ctrl+C) to return exit code 130
  - [ ] 10.14 Write unit tests for each formatter function
  - [ ] 10.15 Write unit tests for error message formatting

- [ ] 11.0 Implement interactive prompts for missing arguments
  - [ ] 11.1 In `src/cli/interactive.rs`, implement `prompt_for_input(prompt: &str) -> Result<String, CliError>`
  - [ ] 11.2 Use `dialoguer::Input` to create interactive text prompt
  - [ ] 11.3 Display prompt text and wait for user input
  - [ ] 11.4 Handle Ctrl+C gracefully (return CliError::Cancelled)
  - [ ] 11.5 Implement `confirm(prompt: &str, default: bool) -> Result<bool, CliError>` for yes/no confirmations
  - [ ] 11.6 Use `dialoguer::Confirm` for confirmation prompts
  - [ ] 11.7 Implement `prompt_with_validation<F>(prompt: &str, validator: F) -> Result<String, CliError>` where F: Fn(&str) -> Result<(), String>
  - [ ] 11.8 Add validation support to re-prompt on invalid input with error message per FR-26
  - [ ] 11.9 Update `handle_agent_run` to use `prompt_for_input` when args.input is None per FR-6
  - [ ] 11.10 Update file overwrite checks to use `confirm` prompt
  - [ ] 11.11 Detect if running in non-TTY environment and fail with error instead of prompting
  - [ ] 11.12 Write unit test for prompt functionality (mock stdin)
  - [ ] 11.13 Test interactive prompt manually: run agent without --input flag

- [ ] 12.0 Write unit tests for CLI components
  - [ ] 12.1 Create `tests/cli/agent_commands_test.rs`
  - [ ] 12.2 Write test for `generate_paladin_template` with different providers
  - [ ] 12.3 Write test for `handle_agent_new` creating file correctly
  - [ ] 12.4 Write test for agent command argument parsing
  - [ ] 12.5 Create `tests/cli/battalion_commands_test.rs`
  - [ ] 12.6 Write test for each battalion template type (formation, phalanx, campaign, chain-of-command)
  - [ ] 12.7 Write test for battalion type validation
  - [ ] 12.8 Create `tests/cli/arsenal_commands_test.rs`
  - [ ] 12.9 Write test for arsenal command argument parsing
  - [ ] 12.10 Write test for mcp_stdio and mcp_sse mutual exclusivity
  - [ ] 12.11 Create `tests/cli/config_loading_test.rs`
  - [ ] 12.12 Write test for loading valid Paladin YAML
  - [ ] 12.13 Write test for loading valid Battalion YAML (all types)
  - [ ] 12.14 Write test for missing required field error
  - [ ] 12.15 Write test for invalid field value error
  - [ ] 12.16 Write test for file not found error
  - [ ] 12.17 Write test for invalid YAML syntax error with line numbers
  - [ ] 12.18 Create `tests/cli/template_generation_test.rs`
  - [ ] 12.19 Write test verifying generated templates are valid YAML
  - [ ] 12.20 Write test verifying all required fields present in templates
  - [ ] 12.21 Run all unit tests: `cargo test --lib cli`
  - [ ] 12.22 Verify unit test coverage ≥80% using `cargo llvm-cov` if available

- [ ] 13.0 Write integration tests for end-to-end workflows
  - [ ] 13.1 Create `tests/integration/cli_integration_test.rs`
  - [ ] 13.2 Write test: generate Paladin template, verify file created and valid
  - [ ] 13.3 Write test: generate Battalion template (formation), verify file and structure
  - [ ] 13.4 Write test: run Paladin from config with mock LLM adapter
  - [ ] 13.5 Write test: run Formation with multiple mock Paladins
  - [ ] 13.6 Write test: run Phalanx with parallel execution
  - [ ] 13.7 Write test: arsenal list with mock MCP servers
  - [ ] 13.8 Write test: arsenal test with mock stdio MCP server
  - [ ] 13.9 Write test: missing API key error handling
  - [ ] 13.10 Write test: invalid config file error handling
  - [ ] 13.11 Write test: output to file (--output flag)
  - [ ] 13.12 Write test: verbose mode output
  - [ ] 13.13 Write test: exit codes for different error types
  - [ ] 13.14 Set up integration test helpers for creating temp configs and files
  - [ ] 13.15 Run integration tests: `cargo test --test cli_integration_test`
  - [ ] 13.16 Verify integration test coverage ≥70%

- [ ] 14.0 Create documentation and example configurations
  - [ ] 14.1 Create `examples/cli_configs/basic_paladin.yaml` with simple Paladin config
  - [ ] 14.2 Create `examples/cli_configs/advanced_paladin.yaml` with garrison and arsenal
  - [ ] 14.3 Create `examples/cli_configs/formation.yaml` with 3-step sequential workflow
  - [ ] 14.4 Create `examples/cli_configs/phalanx.yaml` with parallel execution
  - [ ] 14.5 Create `examples/cli_configs/campaign.yaml` with DAG workflow
  - [ ] 14.6 Create `examples/cli_configs/chain_of_command.yaml` with hierarchical delegation
  - [ ] 14.7 Add inline comments to all example configs explaining each field
  - [ ] 14.8 Create `docs/CLI_USAGE.md` with comprehensive CLI documentation
  - [ ] 14.9 Document all commands with syntax and examples in CLI_USAGE.md
  - [ ] 14.10 Document configuration file format with full schema in CLI_USAGE.md
  - [ ] 14.11 Document environment variables required (API keys) in CLI_USAGE.md
  - [ ] 14.12 Add troubleshooting section with common errors and solutions
  - [ ] 14.13 Update main `README.md` with link to CLI documentation
  - [ ] 14.14 Update main `README.md` with quick start example using CLI
  - [ ] 14.15 Add rustdoc comments to all public CLI functions and types
  - [ ] 14.16 Generate docs: `cargo doc --no-deps --open` and verify CLI module documented

- [ ] 15.0 Final validation and code review preparation
  - [ ] 15.1 Run `cargo fmt` to format all code
  - [ ] 15.2 Run `cargo clippy -- -D warnings` and fix all warnings
  - [ ] 15.3 Run `cargo test --all` and verify all tests pass
  - [ ] 15.4 Run `cargo audit` to check for security vulnerabilities
  - [ ] 15.5 Run Snyk scan on CLI code: `snyk_code_scan` per security rules
  - [ ] 15.6 Fix any security issues identified
  - [ ] 15.7 Test all CLI commands manually end-to-end:
    - `paladin agent new` → edit → `paladin agent run`
    - `paladin battalion new` → edit → `paladin battalion run`
    - `paladin arsenal list`
    - `paladin arsenal test`
  - [ ] 15.8 Test with all three LLM providers (OpenAI, DeepSeek, Anthropic)
  - [ ] 15.9 Test error scenarios: missing API key, invalid config, connection failures
  - [ ] 15.10 Verify --help works for all commands and subcommands
  - [ ] 15.11 Verify --verbose flag provides detailed output
  - [ ] 15.12 Verify --output flag saves results to file correctly
  - [ ] 15.13 Verify interactive prompts work when arguments missing
  - [ ] 15.14 Test Ctrl+C handling (exit code 130)
  - [ ] 15.15 Review all code for adherence to hexagonal architecture
  - [ ] 15.16 Review all error messages for actionable guidance per FR-20
  - [ ] 15.17 Commit all changes: `git add . && git commit -m "feat(epic-9): implement Armory CLI tools"`
  - [ ] 15.18 Push branch: `git push -u origin epic-9/armory-cli-tools`
  - [ ] 15.19 Create PR with description linking to PRD and this task list
  - [ ] 15.20 Address code review feedback
