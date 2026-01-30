## Epic 18: CLI Enhancement & Polish

**Theme:** Developer Experience  
**Duration:** 2 weeks  
**Priority:** Medium  
**Dependencies:** Epics 11-17  

### Description
Enhance the Armory CLI with onboarding wizards, setup verification, feature discovery, and advanced commands for common multi-agent patterns.

### User Stories

#### US-18.1: Onboarding Wizard
**As a** new developer  
**I want** an interactive setup experience  
**So that** I can get started quickly

**Acceptance Criteria:**
- [ ] `paladin onboarding` command
- [ ] Guides through API key configuration
- [ ] Creates initial `.env` file
- [ ] Offers to create sample config files
- [ ] Validates provider connectivity
- [ ] Colorful, friendly CLI output

**Definition of Done:**
```bash
$ paladin onboarding

🎖️  Welcome to Paladin!

Let's set up your environment...

? Which LLM provider will you use primarily?
  ❯ OpenAI
    Anthropic
    DeepSeek

? Enter your OpenAI API key: sk-...

✓ API key validated successfully!

? Create sample configuration files?
  ❯ Yes, create examples
    No, I'll create my own

✓ Created:
  - examples/basic_paladin.yaml
  - examples/formation.yaml
  - .env

🚀 You're all set! Try: paladin agent run --config examples/basic_paladin.yaml
```

---

#### US-18.2: Setup Check Command
**As a** developer  
**I want** to verify my environment  
**So that** I can troubleshoot issues

**Acceptance Criteria:**
- [ ] `paladin setup-check` command
- [ ] Validates API keys for configured providers
- [ ] Checks optional dependencies (Redis, etc.)
- [ ] Reports versions of key components
- [ ] `--verbose` for detailed output
- [ ] Exit codes for CI/CD usage

**Definition of Done:**
```bash
$ paladin setup-check

🔍 Checking Paladin environment...

Core Components:
  ✓ Paladin CLI v0.1.0
  ✓ Rust 1.75.0

LLM Providers:
  ✓ OpenAI: API key valid (gpt-4 accessible)
  ✓ Anthropic: API key valid (claude-3 accessible)
  ✗ DeepSeek: API key not configured

Optional Services:
  ⚠ Redis: Not running (Citadel caching unavailable)
  ✓ Qdrant: Running at localhost:6333

Summary: 4/5 checks passed
```

---

#### US-18.3: Features Discovery Command
**As a** developer  
**I want** to see all available features  
**So that** I can discover capabilities

**Acceptance Criteria:**
- [ ] `paladin features` command
- [ ] Lists all commands with descriptions
- [ ] Groups by category
- [ ] Shows feature flags and optional features
- [ ] Links to documentation

**Definition of Done:**
```bash
$ paladin features

🎖️  Paladin Features

Agent Commands:
  paladin agent new      Create agent configuration template
  paladin agent run      Execute a single agent
  paladin agent validate Validate agent configuration

Battalion Commands:
  paladin battalion new       Create battalion template
  paladin battalion run       Execute multi-agent workflow
  paladin battalion visualize Visualize workflow structure

Orchestration Patterns:
  ✓ Formation      Sequential execution
  ✓ Phalanx        Parallel execution
  ✓ Campaign       DAG/Graph execution
  ✓ ChainOfCommand Hierarchical delegation
  ✓ Conclave       Expert synthesis (MoA)
  ✓ Council        Group discussion
  ✓ Grove          Tree-based routing
  ✓ Maneuver       Flow DSL

Memory Systems:
  ✓ Garrison (Short-term) In-memory, SQLite
  ✓ Sanctum (Long-term)   Qdrant, In-memory

📚 Documentation: https://docs.paladin.dev
```

---

#### US-18.4: Muster Command
**As a** developer  
**I want** automatic battalion generation  
**So that** I can quickly prototype workflows

**Acceptance Criteria:**
- [ ] `paladin muster --task "..."` command
- [ ] Uses LLM to analyze task and suggest campaign
- [ ] Generates appropriate configuration
- [ ] Optionally executes immediately
- [ ] Saves config for future use

**Definition of Done:**
```bash
$ paladin muster --task "Research and write a report on AI trends"

🤖 Analyzing task...

Recommended Pattern: Formation (Sequential)
Suggested Agents:
  1. Researcher - Gather information on AI trends
  2. Analyst - Analyze and categorize findings
  3. Writer - Compose final report

? Execute this campaign now? [Y/n]

Generated config saved to: muster_20260129.yaml
```

---

#### US-18.5: Council Command
**As a** developer  
**I want** quick access to council discussions  
**So that** I can run collaborative sessions

**Acceptance Criteria:**
- [ ] `paladin council --topic "..." --participants 3`
- [ ] Quick setup for group discussions
- [ ] Configurable number of participants
- [ ] Default participant roles if not specified
- [ ] Interactive output of conversation

**Definition of Done:**
```bash
$ paladin council \
    --topic "Should we use microservices or monolith?" \
    --participants 3 \
    --max-rounds 5

🏛️  Council Session: Architecture Decision

Participants:
  - Advocate (Pro-Microservices)
  - Critic (Pro-Monolith)
  - Moderator

Round 1:
  Advocate: "Microservices offer better scalability..."
  Critic: "However, the operational complexity..."
  Moderator: "Both valid points. Let's explore..."

[... conversation continues ...]

Conclusion: The council recommends starting with a modular monolith...
```

---

#### US-18.6: Rich CLI Output
**As a** developer  
**I want** beautiful terminal output  
**So that** the CLI is pleasant to use

**Acceptance Criteria:**
- [ ] Progress indicators for long operations
- [ ] Colored output (respects NO_COLOR)
- [ ] Spinners during API calls
- [ ] Tables for structured data
- [ ] Box drawing for emphasis

**Definition of Done:**
```rust
// Using indicatif for progress
// Using console/colored for colors
// Using comfy-table for tables

// Example execution output:
// ⠋ Executing Paladin: Researcher...
// ✓ Researcher completed (1.2s, 450 tokens)
// ⠋ Executing Paladin: Writer...
// ✓ Writer completed (2.1s, 890 tokens)
//
// ┌──────────────────────────────────────┐
// │ Battalion Execution Summary          │
// ├──────────────────────────────────────┤
// │ Total Time: 3.3s                     │
// │ Total Tokens: 1,340                  │
// │ Status: Success                      │
// └──────────────────────────────────────┘
```

---

### Epic 18 Completion Criteria
- [ ] All 6 user stories completed and tested
- [ ] Onboarding wizard functional
- [ ] Setup check comprehensive
- [ ] Features discovery complete
- [ ] Muster generates valid configs
- [ ] Council command working
- [ ] Rich CLI output throughout
- [ ] Updated CLI documentation

---
