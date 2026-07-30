# Product Requirements Document: Armory CLI Tools

## Introduction/Overview

The Armory CLI (`paladin-cli`) is a command-line interface tool that enables developers to rapidly develop, test, and deploy Paladin agents and Battalion orchestrations from their local workstations. It solves the problem of having to write boilerplate Rust code for every Paladin experiment or test scenario by providing a declarative YAML-based configuration approach with immediate execution capabilities.

**Goal:** Provide a developer-friendly CLI that makes working with Paladins and Battalions as simple as running a single command, reducing the friction from idea to execution from hours to minutes.

## Goals

1. **Rapid Prototyping**: Enable developers to create and test Paladin agents in under 5 minutes using YAML configurations
2. **Zero Boilerplate**: Allow Paladin execution without writing any Rust code for common use cases
3. **Developer Experience**: Provide helpful error messages, validation, and interactive prompts for missing configuration
4. **Tool Discovery**: Make Arsenal (MCP tools) easy to discover, test, and integrate with Paladins
5. **Workflow Automation**: Support Battalion orchestration patterns (Formation, Phalanx, Campaign, Chain of Command) via CLI
6. **Configuration Validation**: Catch configuration errors before execution with clear, actionable feedback

## User Stories

### Primary User Stories

1. **As a developer**, I want to run a Paladin from a YAML configuration file so that I can test agent behavior without writing Rust code.

2. **As a developer**, I want to provide input to a Paladin via command-line flags so that I can script and automate Paladin execution.

3. **As a developer**, I want to create a new Paladin configuration template so that I have a starting point with documented options.

4. **As a developer**, I want to execute a Formation (sequential multi-agent workflow) from configuration so that I can orchestrate complex tasks.

5. **As a developer**, I want to list available MCP tools so that I can discover what capabilities are available to my Paladins.

6. **As a developer**, I want to test an MCP server connection so that I can verify tool availability before adding it to a Paladin.

7. **As a developer**, I want interactive prompts for missing required configuration so that I don't have to memorize all CLI flags.

8. **As a developer**, I want configuration validation with helpful error messages so that I can fix issues quickly.

### Secondary User Stories

9. **As a developer**, I want to run a Phalanx (parallel multi-agent execution) so that I can process multiple inputs concurrently.

10. **As a developer**, I want to execute a Campaign (DAG-based workflow) so that I can orchestrate complex multi-step processes.

11. **As a developer**, I want to save Paladin execution output to a file so that I can review results later or use them in other tools.

## Functional Requirements

### Core CLI Structure

**FR-1**: The CLI MUST be named `paladin-cli` and invocable via `paladin` when installed.

**FR-2**: The CLI MUST follow a subcommand structure: `paladin <resource> <action> [options]`

**FR-3**: The CLI MUST support these top-level resource groups:
- `agent` - Single Paladin operations
- `battalion` - Multi-Paladin orchestration operations  
- `arsenal` - Tool and MCP server operations

### Paladin Agent Commands

**FR-4**: The CLI MUST support `paladin agent run` to execute a single Paladin with the following options:
- `--config <path>` or `-c <path>`: Path to YAML configuration file (required)
- `--input <text>` or `-i <text>`: Input text for the Paladin (optional, prompts if missing)
- `--output <path>` or `-o <path>`: Path to save output (optional, prints to stdout if missing)
- `--verbose` or `-v`: Enable verbose logging (optional)

**FR-5**: The CLI MUST support `paladin agent new` to generate a new Paladin configuration template with:
- `--name <name>` or `-n <name>`: Name for the Paladin (required)
- `--output <path>` or `-o <path>`: Where to save the template (required)
- `--provider <provider>`: LLM provider (openai, deepseek, anthropic) (optional, defaults to openai)

**FR-6**: When `--input` is not provided to `paladin agent run`, the CLI MUST prompt the user interactively with: "Enter input for Paladin: "

**FR-7**: The generated YAML template from `paladin agent new` MUST include:
- Commented examples for all configuration options
- System prompt placeholder with guidance
- LLM provider configuration section
- Garrison (memory) configuration example
- Arsenal (tools) configuration example
- All available configuration parameters documented

### Battalion Commands

**FR-8**: The CLI MUST support `paladin battalion run` to execute multi-Paladin workflows with:
- `--config <path>` or `-c <path>`: Path to Battalion YAML configuration (required)
- `--type <type>` or `-t <type>`: Battalion type (formation, phalanx, campaign, chain-of-command) (required)
- `--output <path>` or `-o <path>`: Path to save output (optional)
- `--verbose` or `-v`: Enable verbose logging (optional)

**FR-9**: The CLI MUST validate that the Battalion type matches the structure defined in the configuration file.

**FR-10**: The CLI MUST support `paladin battalion new` to generate Battalion templates with:
- `--name <name>` or `-n <name>`: Battalion name (required)
- `--type <type>` or `-t <type>`: Battalion type (formation, phalanx, campaign, chain-of-command) (required)
- `--output <path>` or `-o <path>`: Where to save template (required)

### Arsenal Commands

**FR-11**: The CLI MUST support `paladin arsenal list` to display all configured MCP tools with:
- Tool name
- Tool description
- Tool type (stdio, sse)
- Connection status

**FR-12**: The CLI MUST support `paladin arsenal test` to verify MCP server connectivity with:
- `--mcp-stdio <command>`: Test STDIO-based MCP server (mutually exclusive with --mcp-sse)
- `--mcp-sse <endpoint>`: Test SSE-based MCP server (mutually exclusive with --mcp-stdio)

**FR-13**: The `arsenal test` command MUST output:
- Connection status (success/failure)
- List of available tools from the server
- Basic tool schema information
- Connection time/latency

### Configuration Format

**FR-14**: All Paladin and Battalion configurations MUST be in YAML format only.

**FR-15**: Paladin YAML configuration MUST support these fields:
```yaml
name: string (required)
system_prompt: string (required)
model: string (required)
temperature: float (optional, default: 0.7)
max_loops: integer (optional, default: 3)
timeout_seconds: integer (optional, default: 300)
stop_words: list of strings (optional)
provider:
  type: string (openai|deepseek|anthropic) (required)
  # API key loaded from environment variable
garrison:
  type: string (in_memory|sqlite) (optional)
  config: object (optional)
arsenal:
  mcp_servers: list (optional)
    - name: string
      type: string (stdio|sse)
      command: string (for stdio)
      args: list of strings (for stdio)
      endpoint: string (for sse)
```

**FR-16**: Battalion YAML configuration MUST support:
- Type-specific structure (formation, phalanx, campaign, chain-of-command)
- Reference to Paladin configuration files or inline Paladin definitions
- Execution parameters specific to the Battalion type

### Environment Variables

**FR-17**: The CLI MUST load LLM provider API keys from these environment variables:
- `OPENAI_API_KEY` for OpenAI
- `DEEPSEEK_API_KEY` for DeepSeek  
- `ANTHROPIC_API_KEY` for Anthropic

**FR-18**: If a required API key is missing, the CLI MUST fail with a clear error message: "Missing API key: <KEY_NAME>. Please set the environment variable."

### Validation and Error Handling

**FR-19**: The CLI MUST validate configuration files before execution and report specific errors:
- Invalid YAML syntax with line/column numbers
- Missing required fields with field names
- Invalid field values with expected format
- File not found errors with file paths

**FR-20**: All error messages MUST be actionable and include:
- What went wrong
- Why it's a problem
- How to fix it (when possible)

**FR-21**: The CLI MUST return appropriate exit codes:
- `0` for success
- `1` for user errors (invalid config, missing required args)
- `2` for runtime errors (LLM failures, tool failures)
- `130` for SIGINT (Ctrl+C)

### Output Formatting

**FR-22**: The CLI MUST output execution results in a human-readable format including:
- Paladin name
- Input provided
- Final output/response
- Execution time
- Token usage (if available)

**FR-23**: When `--output` is specified, the CLI MUST save results to the file in structured format (JSON) containing:
- All execution metadata
- Full conversation history
- Tool calls and results (if any)
- Timestamps

**FR-24**: The CLI MUST support `--verbose` mode which outputs:
- Each reasoning loop iteration
- Tool calls and results
- LLM requests and responses
- Timing information for each step

### Interactive Mode

**FR-25**: When required arguments are missing, the CLI MUST:
- Prompt the user interactively for the missing value
- Display helpful context about what's being requested
- Allow the user to cancel with Ctrl+C

**FR-26**: Interactive prompts MUST validate user input and re-prompt on invalid input with guidance.

## Non-Goals (Out of Scope)

The following are explicitly OUT OF SCOPE for this feature:

**NG-1**: **GUI or Web Interface** - This is a CLI-only tool; no graphical interface will be provided.

**NG-2**: **Configuration File Encryption** - Sensitive data (API keys) must be in environment variables only; no encrypted config file support.

**NG-3**: **Real-time Monitoring Dashboard** - No built-in dashboard or UI for monitoring running Paladins.

**NG-4**: **Multi-format Configuration** - Only YAML is supported; no JSON or TOML configuration files.

**NG-5**: **Cloud Deployment** - The CLI is for local development only; cloud deployment features are not included.

**NG-6**: **Built-in Package Manager** - No built-in installation via apt/brew/chocolatey; users must use `cargo install`.

**NG-7**: **REPL Mode** - No interactive shell or REPL; each command is a discrete execution.

**NG-8**: **Configuration Migration Tools** - No tools to migrate configs from other agent frameworks.

**NG-9**: **Distributed Execution** - All execution is local; no remote or distributed Battalion execution.

**NG-10**: **Built-in Credential Management** - No keychain/secret manager integration; environment variables only.

## Design Considerations

### CLI Framework

- Use `clap` (v4+) for argument parsing with derive macros for clean, maintainable code
- Follow standard Unix CLI conventions (flags, options, subcommands)
- Support `--help` at every level with comprehensive documentation

### User Experience

- Default to sensible behavior (e.g., print to stdout if no output file specified)
- Make common workflows as few keystrokes as possible
- Provide progress indicators for long-running operations
- Use colors for terminal output (errors in red, success in green, info in blue) when TTY is detected

### YAML Configuration Examples

The CLI should generate templates that look like:

```yaml
# Paladin Configuration
name: "research_analyst"
system_prompt: |
  You are a research analyst who provides detailed, factual analysis.
  Always cite sources and explain your reasoning.

model: "gpt-4"
temperature: 0.7
max_loops: 3
timeout_seconds: 300

provider:
  type: openai
  # API key loaded from OPENAI_API_KEY environment variable

garrison:
  type: in_memory
  config:
    max_entries: 100

arsenal:
  mcp_servers:
    - name: web_search
      type: stdio
      command: uvx
      args: ["mcp-web-search"]
```

## Technical Considerations

### Dependencies

- **clap** (v4+): CLI argument parsing
- **serde** + **serde_yaml**: YAML configuration parsing
- **tokio**: Async runtime for Paladin execution
- **colored** or **owo-colors**: Terminal color output
- **indicatif**: Progress bars for long operations

### Architecture

The CLI binary (`src/bin/paladin-cli.rs`) should:

1. **Parse arguments** using clap derive macros
2. **Load configuration** from YAML files using serde_yaml
3. **Validate configuration** using domain validation logic from `core/`
4. **Construct domain entities** (Paladin, Battalion) using builders from `application/use_cases/`
5. **Execute** using appropriate services from `application/`
6. **Format output** using Herald formatter (Epic 8)
7. **Handle errors** gracefully with actionable messages

### Integration Points

- **Epic 1 (Paladin Foundation)**: Use `PaladinBuilder` to construct agents from config
- **Epic 2 (Garrison)**: Configure memory systems from YAML
- **Epic 3 (Arsenal)**: Connect MCP servers specified in config
- **Epic 4 (Battalion)**: Execute Formation, Phalanx, Campaign, Chain of Command patterns
- **Epic 6 (Provider Expansion)**: Support multiple LLM providers via config
- **Epic 8 (Herald)**: Use Herald for output formatting

### File Locations

- Binary: `src/bin/paladin-cli.rs`
- Subcommand modules: `src/cli/commands/` (agent.rs, battalion.rs, arsenal.rs)
- Config parsing: `src/cli/config/` (paladin_config.rs, battalion_config.rs)
- Template generation: `src/cli/templates/`

### Configuration Loading Order

1. Load YAML file specified by `--config`
2. Parse and deserialize into domain configuration types
3. Load environment variables for API keys
4. Validate all configuration before execution
5. Fail fast with clear errors if validation fails

### Error Handling Strategy

All errors should follow this format:

```
Error: <Short description>

Details: <Longer explanation>

Suggestion: <How to fix it>

Example: <If applicable, show correct usage>
```

For example:
```
Error: Missing required field 'system_prompt'

Details: The Paladin configuration at 'config.yaml' is missing the required 'system_prompt' field.

Suggestion: Add a system_prompt field to your configuration:

system_prompt: |
  You are a helpful assistant...
```

## Success Metrics

The Armory CLI will be considered successful when:

1. **Time to First Paladin**: A developer can go from `cargo install paladin-cli` to executing their first Paladin in under 5 minutes

2. **Adoption Rate**: >80% of Paladin project contributors use the CLI for local testing within 2 weeks of release

3. **Error Resolution**: >90% of configuration errors are resolved by users without consulting documentation (measured by error message clarity)

4. **Template Usage**: >70% of new Paladin configurations start from CLI-generated templates

5. **Battalion Execution**: Developers can successfully run all four Battalion patterns (Formation, Phalanx, Campaign, Chain of Command) via CLI

6. **Tool Discovery**: 100% of configured MCP tools are discoverable via `paladin arsenal list`

7. **Zero Code Testing**: Developers can test agent behavior changes without writing/recompiling Rust code (measured by iteration time reduction)

## Open Questions

1. **Streaming Output**: Should the CLI support streaming output for long-running Paladin responses? (Related to Epic 8 Herald streaming support)

2. **Config Schema Validation**: Should we provide a JSON schema or similar for YAML validation in IDEs?

3. **Template Repository**: Should templates be embedded in the binary or downloaded from a repository to allow updates without CLI updates?

4. **Dry Run Mode**: Should we support `--dry-run` to validate configuration without executing?

5. **Watch Mode**: Should we support `--watch` to auto-reload and re-execute when config files change?

6. **Exit on Error**: For Battalions, should execution stop on first Paladin failure or continue with best effort?

7. **Logging Configuration**: Should logging levels be configurable via CLI flags or config file?

8. **Checkpoint Files**: Should the CLI support Battalion checkpoint/resume from specific steps?

---

## Appendix: Example Usage Scenarios

### Scenario 1: Quick Paladin Test

```bash
# Generate template
paladin agent new --name analyst --output analyst.yaml

# Edit analyst.yaml with your favorite editor

# Run it
paladin agent run --config analyst.yaml --input "What are the trends in AI for 2026?"
```

### Scenario 2: Formation Workflow

```bash
# Generate Formation template
paladin battalion new --name research-workflow --type formation --output workflow.yaml

# Run the workflow
paladin battalion run --config workflow.yaml --type formation
```

### Scenario 3: MCP Tool Testing

```bash
# Test MCP server connectivity
paladin arsenal test --mcp-stdio "uvx mcp-web-search"

# List all available tools
paladin arsenal list
```

### Scenario 4: Automated Testing

```bash
#!/bin/bash
# test-paladin.sh

export OPENAI_API_KEY="sk-..."

for test_case in test-cases/*.yaml; do
  echo "Running $test_case..."
  paladin agent run --config config/paladin.yaml --input "$(cat $test_case)" \
    --output "results/$(basename $test_case .yaml).json"
done
```

---

**Document Version**: 1.0  
**Last Updated**: January 26, 2026  
**Epic**: Epic 9 - Armory CLI Tools  
**Priority**: Medium  
**Estimated Effort**: 2-3 weeks
