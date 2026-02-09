# Product Requirements Document: Epic 18 - CLI Enhancement & Polish

## Document Information
- **Feature Name:** CLI Enhancement & Polish
- **Epic:** Epic 18
- **Version:** 1.0
- **Created:** February 7, 2026
- **Status:** Draft
- **Target Audience:** Junior Developers
- **Dependencies:** Epics 11-17 (Sanctum, Sentinel, Autonomous Agents, Conclave, Advanced Battalion Patterns, Flow DSL)

---

## 1. Introduction/Overview

The Paladin Armory CLI currently provides basic functionality for running agents and battalions, but lacks the developer experience features that make complex multi-agent systems accessible to new users and productive for experienced developers. This epic focuses on enhancing the CLI with onboarding wizards, environment validation, feature discovery tools, and advanced orchestration commands that leverage the newly implemented features from Epics 11-17.

**Problem Statement:**
- New developers struggle to configure the Paladin environment correctly
- Users don't know what features are available or how to use them
- Common multi-agent patterns require verbose YAML configuration
- CLI output is plain and difficult to parse during complex executions
- Troubleshooting environment issues is time-consuming

**Solution:**
Create a polished, user-friendly CLI experience that guides new users through setup, helps all users discover capabilities, provides shortcuts for common patterns, and delivers rich, informative output during execution.

---

## 2. Goals

1. **Reduce Time-to-First-Success:** New users should successfully run their first Paladin agent within 5 minutes of installation
2. **Increase Feature Discoverability:** Users should be able to discover 80% of Paladin capabilities through CLI commands alone
3. **Simplify Common Workflows:** Reduce configuration overhead for common multi-agent patterns by 60%
4. **Improve Troubleshooting:** Users should be able to diagnose 90% of configuration issues using built-in validation commands
5. **Enhance Visual Feedback:** Provide real-time progress indicators and rich formatted output for all long-running operations
6. **Maintain CI/CD Compatibility:** All interactive features must have non-interactive alternatives for automation

---

## 3. User Stories

### US-18.1: Onboarding Wizard
**As a** new developer  
**I want** an interactive setup experience  
**So that** I can get started quickly without reading extensive documentation

**Acceptance Criteria:**
- Interactive wizard runs with `paladin onboarding` command
- Guides user through API key configuration for at least one LLM provider
- Creates `.env` file with proper formatting
- Offers to create sample configuration files (agent, formation, phalanx, etc.)
- Validates provider connectivity in real-time
- Handles existing configurations gracefully (prompts to overwrite, skip, or merge)
- Provides colored, emoji-enhanced output for better readability
- Can be interrupted and resumed
- Generates summary of completed setup steps

### US-18.2: Setup Check Command
**As a** developer  
**I want** to verify my environment configuration  
**So that** I can troubleshoot issues and confirm dependencies are available

**Acceptance Criteria:**
- Runs with `paladin setup-check` command
- Validates API keys for all configured LLM providers (OpenAI, Anthropic, DeepSeek)
- Tests actual API connectivity (not just key format)
- Checks optional service availability (Redis, Qdrant, MinIO)
- Reports versions of Paladin CLI and Rust toolchain
- Displays clear status indicators (✓, ✗, ⚠)
- Provides actionable error messages for failed checks
- Supports `--verbose` flag for detailed diagnostic output
- Returns appropriate exit codes for CI/CD integration (0 = all pass, 1 = critical failure, 2 = warnings)

### US-18.3: Features Discovery Command
**As a** developer  
**I want** to explore available features and commands  
**So that** I can understand what Paladin can do without leaving the terminal

**Acceptance Criteria:**
- Runs with `paladin features` command
- Lists all CLI commands grouped by category (Agent, Battalion, Orchestration, Memory, Utilities)
- Shows brief description for each command
- Indicates which orchestration patterns are available
- Lists memory system options (Garrison types, Sanctum backends)
- Shows feature flags and optional features status
- Provides links to relevant documentation sections
- Supports filtering by category (`paladin features --category battalion`)
- Machine-readable output option (`--format json`)

### US-18.4: Muster Command (LLM-Powered Battalion Generation)
**As a** developer  
**I want** automatic battalion generation from task descriptions  
**So that** I can quickly prototype complex workflows without manual configuration

**Acceptance Criteria:**
- Runs with `paladin muster --task "description"` command
- Uses configured LLM to analyze the task description
- Determines appropriate orchestration pattern (Formation, Phalanx, Campaign, etc.)
- Generates suggested agent roles and responsibilities
- Creates valid YAML configuration file
- Allows user to review and edit before execution
- Supports `--execute` flag to run immediately after generation
- Saves generated config to `muster_<timestamp>.yaml` by default
- Supports `--output <path>` to specify save location
- Provides clear explanation of why specific pattern was chosen
- Handles LLM errors gracefully with fallback to template-based generation

### US-18.5: Council Command (Quick Group Discussions)
**As a** developer  
**I want** quick access to council discussion patterns  
**So that** I can facilitate multi-agent collaborative reasoning without complex setup

**Acceptance Criteria:**
- Runs with `paladin council --topic "..." --participants N` command
- Creates council with specified number of participants (default: 3)
- Assigns default roles if not explicitly specified (Advocate, Critic, Moderator, etc.)
- Supports custom role specification via `--roles "role1,role2,role3"`
- Configurable maximum discussion rounds (`--max-rounds N`, default: 5)
- Streams conversation in real-time to terminal
- Provides formatted output showing speaker, role, and contribution
- Generates summary/conclusion at end of discussion
- Saves full transcript to file (`--save <path>`)
- Supports all standard LLM configuration flags (model, temperature, etc.)

### US-18.6: Rich CLI Output
**As a** developer  
**I want** visually appealing and informative terminal output  
**So that** I can easily track progress and understand results

**Acceptance Criteria:**
- Progress indicators for long-running operations (spinners, progress bars)
- Color-coded output (green for success, red for errors, yellow for warnings, blue for info)
- Respects `NO_COLOR` environment variable for accessibility
- Animated spinners during API calls with status messages
- Structured data displayed in formatted tables
- Box drawing for emphasis and visual hierarchy
- Token usage and timing information displayed clearly
- Real-time streaming output for agent responses
- Summary tables at end of battalion executions
- Error messages highlighted with context and suggestions
- Support for `--quiet` mode (minimal output) and `--verbose` mode (detailed output)

---

## 4. Functional Requirements

### Core CLI Infrastructure

**FR-1:** The CLI must support both interactive and non-interactive modes for all commands

**FR-2:** All commands must support `--help` flag with comprehensive usage information

**FR-3:** The CLI must respect standard environment variables (`NO_COLOR`, `TERM`, etc.)

**FR-4:** All commands must return appropriate exit codes (0 = success, 1 = error, 2 = warning)

**FR-5:** The CLI must load configuration from standard locations (`.env`, `config.yml`, CLI flags)

### Onboarding Wizard (US-18.1)

**FR-6:** The onboarding command must create a `.env` file with the following structure:
```
# LLM Provider Configuration
OPENAI_API_KEY=<key>
ANTHROPIC_API_KEY=<key>
DEEPSEEK_API_KEY=<key>

# Optional Services
REDIS_URL=redis://localhost:6379
QDRANT_URL=http://localhost:6333
MINIO_ENDPOINT=localhost:9000
```

**FR-7:** When existing configuration files are detected, the wizard must:
- Display the conflicting file path
- Offer three options: [O]verwrite, [S]kip, [M]erge
- For merge, intelligently combine configurations without duplicates

**FR-8:** The wizard must validate API keys by making test API calls to each provider

**FR-9:** Sample configurations must be generated for:
- Basic agent (`examples/basic_paladin.yaml`)
- Formation workflow (`examples/formation.yaml`)
- Phalanx workflow (`examples/phalanx.yaml`)
- Agent with RAG (`examples/paladin_with_rag.yaml`)

**FR-10:** The wizard must be resumable if interrupted (track completed steps)

### Setup Check (US-18.2)

**FR-11:** The setup check must validate the following components:
- Paladin CLI version
- Rust toolchain version
- Each configured LLM provider (API key and model access)
- Redis connectivity (if configured)
- Qdrant connectivity (if configured)
- MinIO connectivity (if configured)

**FR-12:** Provider validation must test actual API calls:
- OpenAI: Call `/v1/models` endpoint
- Anthropic: Test message API with minimal request
- DeepSeek: Call available models endpoint

**FR-13:** Status indicators must be:
- `✓` (green) for available/passing
- `✗` (red) for unavailable/failing
- `⚠` (yellow) for available with warnings

**FR-14:** Verbose mode must display:
- Full version strings
- Response times for API calls
- Detailed error messages with stack traces
- Configuration file locations being used

### Features Discovery (US-18.3)

**FR-15:** The features command must group commands into categories:
- **Agent Commands:** `new`, `run`, `validate`
- **Battalion Commands:** `new`, `run`, `visualize`
- **Orchestration Patterns:** Formation, Phalanx, Campaign, ChainOfCommand, Conclave, Council, Grove, Maneuver
- **Memory Systems:** Garrison (In-memory, SQLite), Sanctum (Qdrant, In-memory)
- **Utility Commands:** `onboarding`, `setup-check`, `features`, `muster`, `council`

**FR-16:** Each feature entry must include:
- Command name
- Brief description (1-2 sentences)
- Availability status (✓ available, ✗ requires feature flag)
- Link to documentation

**FR-17:** JSON output format must be:
```json
{
  "categories": [
    {
      "name": "Agent Commands",
      "commands": [
        {
          "name": "paladin agent new",
          "description": "...",
          "available": true,
          "docs_url": "..."
        }
      ]
    }
  ]
}
```

### Muster Command (US-18.4)

**FR-18:** The muster command must accept task description via:
- `--task "description"` flag
- Interactive prompt if flag not provided
- Reading from stdin (`echo "task" | paladin muster`)

**FR-19:** LLM analysis must return structured output including:
- Recommended orchestration pattern with justification
- List of suggested agents (name, role, system prompt)
- Estimated complexity (simple/medium/complex)
- Estimated token usage

**FR-20:** Generated configuration must be valid YAML that can be immediately executed with `paladin battalion run`

**FR-21:** The command must support:
- `--execute` flag to run immediately after generation
- `--output <path>` to specify save location
- `--provider <name>` to select LLM for analysis
- `--model <name>` to specify model for analysis
- `--no-review` to skip review step in non-interactive mode

**FR-22:** If LLM call fails, must fall back to template selection based on keyword matching

### Council Command (US-18.5)

**FR-23:** The council command must support:
- `--topic "description"` for discussion topic
- `--participants N` for number of participants (min: 2, max: 10)
- `--roles "role1,role2,..."` for custom role assignment
- `--max-rounds N` for maximum discussion iterations
- `--save <path>` to save transcript

**FR-24:** Default role assignment for different participant counts:
- 2: Advocate, Critic
- 3: Advocate, Critic, Moderator
- 4: Advocate, Critic, Moderator, Synthesizer
- 5+: Mix of Experts, Advocates, Critics, Moderator

**FR-25:** Real-time output must show:
- Round number
- Speaker role and name
- Contribution text with formatting
- Clear visual separation between turns

**FR-26:** Final summary must include:
- Key points from discussion
- Areas of consensus
- Areas of disagreement
- Recommended action/conclusion

### Rich CLI Output (US-18.6)

**FR-27:** Progress indicators must be used for:
- API calls (spinner with status message)
- File operations (progress bar for large files)
- Battalion execution (progress bar showing completion percentage)
- Embedding generation (progress bar for batches)

**FR-28:** Color scheme:
- Green: Success, completion, positive status
- Red: Errors, failures, critical warnings
- Yellow: Warnings, notices, important info
- Blue: Informational messages, headers
- Cyan: Links, references
- White/Default: Standard output

**FR-29:** Tables must be used for:
- Battalion execution summaries (agent, time, tokens, status)
- Setup check results (component, status, details)
- Feature listings (name, description, availability)

**FR-30:** Box drawing must be used for:
- Section headers
- Important notices
- Final summaries
- Error messages with context

---

## 5. Non-Goals (Out of Scope)

The following are explicitly **not** included in this epic:

**NG-1:** Graphical User Interface (GUI) or web-based dashboard

**NG-2:** Voice-based CLI interaction or speech recognition

**NG-3:** Integration with external project management tools (Jira, GitHub Issues, etc.)

**NG-4:** Multi-language support (internationalization/localization) for CLI output

**NG-5:** Built-in code editor or IDE integration (remain editor-agnostic)

**NG-6:** Automatic billing/cost tracking for LLM API usage

**NG-7:** Built-in agent marketplace or sharing platform

**NG-8:** Windows-specific features (focus on Linux/macOS, Windows via WSL)

**NG-9:** Telemetry or usage analytics collection

**NG-10:** Custom shell completions (bash/zsh/fish) - can be added in future iteration

---

## 6. Design Considerations

### User Experience Flow

**New User Journey:**
1. Install Paladin CLI
2. Run `paladin onboarding`
3. Follow interactive prompts to configure API keys
4. Wizard validates connectivity and creates sample configs
5. User runs suggested first command
6. Success within 5 minutes

**Experienced User Journey:**
1. Run `paladin muster --task "complex workflow"` or `paladin council --topic "architecture"`
2. Review generated configuration or watch live discussion
3. Execute or save for future use
4. Verify with `paladin setup-check` if issues arise

### Visual Design Principles

- **Clarity:** Output should be immediately understandable
- **Consistency:** Use same colors/indicators across all commands
- **Progressive Disclosure:** Show minimal output by default, detailed with `--verbose`
- **Accessibility:** Respect `NO_COLOR`, provide text alternatives to icons
- **Performance:** Spinners/progress indicators must not impact execution time

### CLI Libraries to Use

- **clap:** Command-line argument parsing with derive macros
- **indicatif:** Progress bars and spinners
- **console:** Terminal manipulation and styling (respects NO_COLOR)
- **colored:** Color support with automatic detection
- **comfy-table:** Table formatting with Unicode borders
- **dialoguer:** Interactive prompts and confirmations
- **serde_yaml:** YAML configuration generation

---

## 7. Technical Considerations

### Architecture Integration

**Module Location:**
- Main CLI entry point: `src/bin/paladin-cli.rs`
- Command implementations: `src/application/cli/commands/`
- Output formatters: `src/application/cli/formatters/`
- Interactive utilities: `src/application/cli/interactive/`

**Directory Structure:**
```
src/
├── bin/
│   └── paladin-cli.rs          # Main entry point
└── application/
    └── cli/
        ├── commands/
        │   ├── onboarding.rs   # US-18.1
        │   ├── setup_check.rs  # US-18.2
        │   ├── features.rs     # US-18.3
        │   ├── muster.rs       # US-18.4
        │   └── council.rs      # US-18.5
        ├── formatters/
        │   ├── table.rs
        │   ├── progress.rs
        │   └── output.rs       # US-18.6
        └── interactive/
            ├── prompts.rs
            └── wizard.rs
```

### Dependencies

**New Cargo Dependencies:**
```toml
[dependencies]
# Existing...
clap = { version = "4.5", features = ["derive", "env"] }
indicatif = "0.17"
console = "0.15"
colored = "2.1"
comfy-table = "7.1"
dialoguer = "0.11"
```

### LLM Integration for Muster

**Prompt Template for Task Analysis:**
```
Analyze the following task and recommend a multi-agent orchestration strategy.

Task: {user_task}

Provide your response in the following JSON format:
{
  "pattern": "Formation|Phalanx|Campaign|ChainOfCommand|Conclave|Council|Grove",
  "reasoning": "Why this pattern is appropriate",
  "agents": [
    {
      "name": "Agent name",
      "role": "Brief role description",
      "system_prompt": "Detailed system prompt for this agent"
    }
  ],
  "complexity": "simple|medium|complex",
  "estimated_tokens": 1000
}
```

### Configuration File Generation

**Generated YAML Template:**
```yaml
# Generated by: paladin muster
# Task: {original_task}
# Pattern: {selected_pattern}
# Created: {timestamp}

name: "Generated Battalion"
pattern: "{pattern}"
description: "{task}"

agents:
  - id: "{agent_id}"
    name: "{agent_name}"
    system_prompt: |
      {system_prompt}
    model: "gpt-4"
    temperature: 0.7

# Pattern-specific configuration
{pattern_config}
```

### Error Handling

**Error Categories:**
- **Configuration Errors:** Missing API keys, invalid YAML, etc.
- **Network Errors:** API timeouts, connection failures
- **Validation Errors:** Invalid model names, unsupported features
- **User Errors:** Invalid input, cancelled operations

**Error Display Format:**
```
❌ Error: Configuration Error

Failed to load API key for OpenAI

Cause: OPENAI_API_KEY environment variable not set

Suggestion: Run 'paladin onboarding' to configure API keys
            or set manually in .env file

Documentation: https://docs.paladin.dev/setup
```

### Testing Strategy

**Unit Tests (src/application/cli/tests/):**
- Command parsing and validation
- Output formatting logic
- Configuration file generation
- Error message formatting

**Integration Tests (tests/cli/):**
- Full command execution in test environment
- Mocked LLM responses for muster command
- File system operations (config creation, etc.)
- Environment variable handling

**Snapshot Tests (tests/cli/snapshots/):**
- CLI output formatting for various scenarios
- Table rendering with different data
- Progress indicator sequences
- Error message formatting

**Test Coverage Targets:**
- Unit tests: ≥ 80% line coverage
- Integration tests: All happy paths + major error cases
- Snapshot tests: All user-facing output formats

---

## 8. Success Metrics

### Quantitative Metrics

**M-1: Time-to-First-Success**
- **Target:** 90% of new users successfully run first agent within 5 minutes
- **Measurement:** Track onboarding completion time from user surveys/telemetry (if opted in)

**M-2: Configuration Error Reduction**
- **Target:** 70% reduction in configuration-related support requests
- **Measurement:** Compare GitHub issues tagged "configuration" before/after release

**M-3: Feature Discovery**
- **Target:** 80% of users can discover and list available orchestration patterns without documentation
- **Measurement:** User testing session results, survey responses

**M-4: Command Usage Distribution**
- **Target:** 30% adoption rate for new commands (muster, council) within first month
- **Measurement:** Aggregate command usage from opt-in telemetry

**M-5: Setup Validation Success Rate**
- **Target:** 95% of `setup-check` runs identify actual configuration issues
- **Measurement:** Follow-up validation after users fix reported issues

### Qualitative Metrics

**M-6: User Satisfaction**
- Survey question: "How would you rate the Paladin CLI experience?" (1-5 scale)
- **Target:** Average rating ≥ 4.0

**M-7: Developer Feedback**
- Collect feedback on GitHub discussions
- **Target:** 80% positive sentiment in CLI-related feedback

**M-8: Documentation Clarity**
- Survey question: "Did the CLI provide enough information without needing documentation?"
- **Target:** 75% answer "yes" or "mostly"

---

## 9. Open Questions

### Technical Questions

**Q-1:** Should the onboarding wizard support automatic detection of existing API keys in system keychain (macOS Keychain, Windows Credential Manager)?

**Q-2:** For the muster command, should we implement a feedback loop where the user can iterate on the generated config by providing additional constraints?

**Q-3:** Should council transcripts be saved in a structured format (JSON) in addition to human-readable format for potential analysis?

**Q-4:** Do we need rate limiting / throttling for commands that make multiple LLM calls (e.g., council with many rounds)?

### Product Questions

**Q-5:** Should the onboarding wizard recommend provider selection based on user's use case (e.g., cost-sensitive vs. performance-focused)?

**Q-6:** Should we provide a `paladin doctor` command that runs diagnostics and attempts auto-fixes for common issues?

**Q-7:** Do we want to support configuration profiles (dev, staging, production) that can be switched easily?

**Q-8:** Should there be a `paladin update` command to check for and install new versions of the CLI?

### User Experience Questions

**Q-9:** Should progress indicators show estimated time remaining based on historical execution data?

**Q-10:** Should the CLI support a "tutorial mode" that explains each step in detail for learning purposes?

**Q-11:** For power users, should there be a way to disable interactive prompts globally (beyond `--non-interactive` on each command)?

---

## 10. Implementation Phases

### Phase 1: Foundation (Week 1)
**Priority:** Critical path for other user stories

**Deliverables:**
- Rich CLI output infrastructure (US-18.6)
- Output formatters, progress indicators, color support
- Base command structure and error handling
- Testing infrastructure setup

**Rationale:** All other commands depend on consistent output formatting

### Phase 2: New User Experience (Week 1)
**Priority:** High (impacts adoption)

**Deliverables:**
- Onboarding wizard (US-18.1)
- Setup check command (US-18.2)
- Sample configuration generation

**Rationale:** Addresses primary goal of reducing time-to-first-success

### Phase 3: Feature Discovery (Week 2)
**Priority:** Medium (enables exploration)

**Deliverables:**
- Features command (US-18.3)
- Comprehensive help text for all commands
- Documentation links

**Rationale:** Helps users discover capabilities after initial setup

### Phase 4: Power User Features (Week 2)
**Priority:** Medium (advanced functionality)

**Deliverables:**
- Muster command (US-18.4)
- Council command (US-18.5)
- Advanced CLI shortcuts

**Rationale:** Provides value for experienced users building complex systems

---

## 11. Dependencies and Integration Points

### Internal Dependencies (Paladin Framework)

**D-1:** Requires functional LLM adapters (OpenAI, Anthropic, DeepSeek) from Epic 6

**D-2:** Requires Council orchestration pattern from Epic 15

**D-3:** Requires Battalion execution service for muster command

**D-4:** Requires configuration loading system for setup validation

**D-5:** Requires Sanctum and Garrison implementations for feature discovery

### External Dependencies (Rust Ecosystem)

**D-6:** `clap` v4.5+ for modern CLI parsing with derive macros

**D-7:** `indicatif` v0.17+ for progress indicators and spinners

**D-8:** `dialoguer` v0.11+ for interactive prompts

**D-9:** `console` v0.15+ for terminal manipulation with NO_COLOR support

**D-10:** `comfy-table` v7.1+ for Unicode table rendering

### Integration Points

**I-1:** `.env` file loading must integrate with existing `ApplicationSettings`

**I-2:** API validation must use existing adapter health check methods

**I-3:** Generated configs must match existing YAML schema parsers

**I-4:** Muster command must invoke `PaladinBuilder` and `BattalionService` APIs

**I-5:** Council command must use existing Council orchestration implementation

---

## 12. Acceptance Criteria Summary

This epic is considered complete when:

- [ ] All 6 user stories (US-18.1 through US-18.6) are implemented and tested
- [ ] New users can complete onboarding and run first agent in < 5 minutes
- [ ] `paladin setup-check` successfully validates all configured components
- [ ] `paladin features` provides comprehensive feature listing
- [ ] `paladin muster` generates valid, executable battalion configurations
- [ ] `paladin council` facilitates multi-agent discussions
- [ ] All CLI output uses consistent formatting with colors, tables, and progress indicators
- [ ] Test coverage meets targets (80% unit, full integration, snapshot tests)
- [ ] Documentation updated with new commands and examples
- [ ] No regression in existing CLI functionality
- [ ] Code passes all quality checks (fmt, clippy, audit)

---

## 13. Appendix

### Example Command Outputs

**Onboarding Wizard:**
```
🎖️  Welcome to Paladin!

Let's set up your environment...

? Which LLM provider will you use primarily?
  ❯ OpenAI
    Anthropic  
    DeepSeek

✓ OpenAI selected

? Enter your OpenAI API key: ****************************************

⠋ Validating API key...
✓ API key validated successfully! (gpt-4 accessible)

? Create sample configuration files?
  ❯ Yes, create examples
    No, I'll create my own

✓ Created:
  • examples/basic_paladin.yaml
  • examples/formation.yaml
  • examples/phalanx.yaml
  • examples/paladin_with_rag.yaml
  • .env

🚀 You're all set!

Next steps:
  1. Try: paladin agent run --config examples/basic_paladin.yaml
  2. Explore: paladin features
  3. Learn: paladin --help

Happy agent building! 🛡️
```

**Setup Check:**
```
🔍 Checking Paladin environment...

Core Components:
  ✓ Paladin CLI v0.1.0
  ✓ Rust toolchain 1.75.0

LLM Providers:
  ✓ OpenAI: API key valid (models: gpt-4, gpt-4o, gpt-3.5-turbo)
  ✓ Anthropic: API key valid (models: claude-3-opus, claude-3-sonnet)
  ✗ DeepSeek: API key not configured

Optional Services:
  ⚠ Redis: Not running (Citadel caching unavailable)
  ✓ Qdrant: Running at localhost:6333 (collections: 0)
  ⚠ MinIO: Not configured (file storage unavailable)

════════════════════════════════════════════════════════════════
Summary: 5/8 checks passed, 2 warnings, 1 not configured

Suggestions:
  • Configure DeepSeek: Set DEEPSEEK_API_KEY in .env
  • Start Redis: docker-compose up -d redis
  • Configure MinIO: See docs/SETUP.md#minio
════════════════════════════════════════════════════════════════
```

**Battalion Execution with Rich Output:**
```
🎖️  Executing Battalion: Research and Report

Pattern: Formation (Sequential)
Agents: 3

⠋ [1/3] Executing Researcher...
✓ [1/3] Researcher completed (2.3s, 450 tokens)

⠋ [2/3] Executing Analyst...
✓ [2/3] Analyst completed (3.1s, 680 tokens)

⠋ [3/3] Executing Writer...
✓ [3/3] Writer completed (5.2s, 1,200 tokens)

┌─────────────────────────────────────────────────────────────┐
│                 Battalion Execution Summary                  │
├─────────────────┬──────────┬──────────┬─────────┬──────────┤
│ Agent           │ Status   │ Duration │ Tokens  │ Cost     │
├─────────────────┼──────────┼──────────┼─────────┼──────────┤
│ Researcher      │ ✓ Success│ 2.3s     │ 450     │ $0.009   │
│ Analyst         │ ✓ Success│ 3.1s     │ 680     │ $0.014   │
│ Writer          │ ✓ Success│ 5.2s     │ 1,200   │ $0.024   │
├─────────────────┼──────────┼──────────┼─────────┼──────────┤
│ Total           │ ✓ Success│ 10.6s    │ 2,330   │ $0.047   │
└─────────────────┴──────────┴──────────┴─────────┴──────────┘

✓ Battalion completed successfully!

Output saved to: output/battalion_20260207_143022.md
```

### Related Documentation

- [CLI Usage Guide](../docs/CLI_USAGE.md)
- [Configuration Reference](../docs/guides/configuration.md)
- [Battalion Patterns](../docs/BATTALION.md)
- [Contributing Guide](../docs/contributing/CONTRIBUTING.md)

---

**End of PRD**
