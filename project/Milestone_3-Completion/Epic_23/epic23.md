## Epic 23: CLI, Config & Infrastructure Completion

**Theme:** Wire remaining CLI configuration, complete infrastructure stubs  
**Duration:** 1–2 weeks  
**Priority:** Medium  
**Dependencies:** Epics 19–22  
**Origin:** Epics 9, 10, 18 deferred tasks and infrastructure TODOs

### Description

The CLI agent command has TODOs for wiring garrison and arsenal configuration from YAML files. Several CLI integration tests were deferred because they require mock provider support. The API content deliverer has a scheduler stub. This epic completes all infrastructure and CLI wiring.

### User Stories

#### US-23.1: CLI Garrison Configuration

**As a** developer  
**I want** the CLI `muster` command to configure garrison from YAML  
**So that** agents launched from config files have proper memory

**Acceptance Criteria:**
- [ ] Parse garrison config from YAML (type: `in_memory` | `sqlite`, path, max_entries)
- [ ] Instantiate appropriate garrison adapter based on config
- [ ] Pass configured garrison to `PaladinBuilder`
- [ ] Unit test with sample config
- [ ] Error handling for invalid garrison config

**Source Files:**
- `src/application/cli/commands/agent.rs` — line 293 (Task 5.8)

---

#### US-23.2: CLI Arsenal/MCP Configuration

**As a** developer  
**I want** the CLI `muster` command to configure arsenal from YAML  
**So that** agents launched from config files have tool access

**Acceptance Criteria:**
- [ ] Parse MCP server config from YAML (name, type: `stdio` | `sse`, command, args, url)
- [ ] Instantiate MCP adapters based on config
- [ ] Register tools in arsenal registry
- [ ] Pass configured arsenal to `PaladinBuilder`
- [ ] Unit test with sample config
- [ ] Error handling for invalid arsenal config

**Source Files:**
- `src/application/cli/commands/agent.rs` — line 296 (Task 5.9)

---

#### US-23.3: CLI Integration Tests with Mock Provider

**As a** developer  
**I want** CLI integration tests that use mock LLM providers  
**So that** end-to-end CLI workflows are validated without API keys

**Acceptance Criteria:**
- [ ] Implement mock LLM provider support for CLI testing
- [ ] Test: run Paladin from config with mock LLM adapter
- [ ] Test: run Formation with multiple mock Paladins
- [ ] Test: run Phalanx with parallel execution
- [ ] Tests run in CI without external dependencies

**Source — Deferred Tasks:**
- Epic 10, Task 13.4–13.6: CLI integration tests requiring mock provider

---

#### US-23.4: CLI End-to-End & Environment Testing

**As a** developer  
**I want** CLI tested across real environments and terminal types  
**So that** the CLI works reliably in production

**Acceptance Criteria:**
- [ ] Test full user journey: onboarding → first agent run (with mock provider)
- [ ] Test `setup-check` with real services (Redis, Qdrant, MinIO) — Docker-gated
- [ ] Test `muster` command with real LLM providers — env-var-gated
- [ ] Test `council` command end-to-end — env-var-gated
- [ ] Test all commands in non-interactive mode (CI/CD)
- [ ] Test CLI with `NO_COLOR` environment variable
- [ ] Test CLI with different terminal types

**Source — Deferred Tasks:**
- Epic 18, Task 9.1–9.7: End-to-end CLI testing

---

#### US-23.5: API Content Deliverer Scheduler Integration

**As a** developer  
**I want** scheduled content delivery to use a real scheduler  
**So that** time-based delivery works in production

**Acceptance Criteria:**
- [ ] Integrate `tokio-cron-scheduler` or equivalent for scheduled delivery
- [ ] `schedule_delivery()` creates real scheduled jobs
- [ ] Cancellation support for pending scheduled deliveries
- [ ] Unit tests with mock scheduler
- [ ] Integration test verifying scheduled execution

**Source Files:**
- `src/infrastructure/adapters/output/api_content_deliverer.rs` — line 297

---

### Epic 23 Completion Criteria

- [ ] CLI garrison and arsenal config wired from YAML
- [ ] CLI integration tests with mock provider passing
- [ ] End-to-end CLI tests documented and gated appropriately
- [ ] Scheduler integration completed
- [ ] All tests pass; `cargo clippy` clean

---
