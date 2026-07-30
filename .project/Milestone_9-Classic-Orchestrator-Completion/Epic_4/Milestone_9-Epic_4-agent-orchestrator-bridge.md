# Milestone 9 — Epic 4: Agent → Orchestrator Bridge

**Project:** Paladin Framework
**Milestone:** 9 — Classic Orchestrator, Content Pipeline, and Agent-Orchestrator Bridge
**Epic:** 4 of 6
**Priority:** High
**Estimated Effort:** Large
**Dependencies:** Epic 1 (Orchestrator must be functional)
**Status:** Planning

---

## Objective

Enable Paladin AI agents, during execution, to trigger orchestration actions: schedule a job,
enqueue content processing, fire an event, or send a notification. This completes the bidirectional
integration between the AI agent system and the classic orchestration system.

## Background

Currently, a Paladin agent can use tools via the Arsenal (MCP servers, skills, prompts). But it
cannot interact with the orchestration system. An agent analyzing a document should be able to say
"schedule a follow-up analysis for tomorrow" or "queue this content for further processing" or
"notify the admin that a critical finding was detected."

This Epic establishes the *agent → orchestrator* direction, complementing Epic 3's *content → agent*
direction. Together they close the loop between the two halves of the platform.

## Scope

**In scope:**
- A design decision between a port-based bridge and an Arsenal-tool-based bridge.
- Implementation of the chosen bridge.
- Wiring the bridge into the Paladin execution context.
- Integration test proving an agent can trigger an orchestration action.

**Out of scope:**
- The content → agent bridge (Epic 3).
- New orchestration capabilities beyond those already exposed by the `Orchestrator`.

---

## Tasks

### Task 4.1: Design the Agent → Orchestrator Interface

**Description:** Evaluate two approaches:

**Option A — `OrchestratorPort` trait in `paladin-ports`:** Define `OrchestratorPort` with methods
like `schedule_job()`, `queue_item()`, `fire_event()`, `send_notification()`. Inject into
`PaladinExecutionService` alongside `LlmPort` and `ArsenalPort`. The agent accesses orchestrator
capabilities through the port.

**Option B — Arsenal tool (Armament) wrapping the Orchestrator:** Create an `OrchestratorArmament`
that registers as a tool in the agent's Arsenal. The agent invokes it via natural language tool
calls (e.g., "Use the scheduler tool to create a follow-up job"). This leverages the existing
MCP/tool-use infrastructure.

**Decision criteria to document:**
- Discoverability by the LLM (tool schemas are self-describing; ports are not).
- Safety/authorization boundaries (which approach makes it easier to gate which actions an agent may
  take).
- Testability and coupling (port keeps the core decoupled; tool reuses existing Arsenal plumbing).
- Consistency with the existing Arsenal/MCP tool-use model.

**Deliverables:**
- Design document evaluating both options with trade-offs.
- Selected approach.
- Interface definition (trait signature or tool schema).

**Acceptance criteria:**
- A written design with an explicit decision and rationale exists in this Epic directory.
- The interface (trait or tool schema) is fully specified before implementation begins.

---

### Task 4.2: Implement the Selected Bridge

**Description:** Implement the bridge per the design decision in Task 4.1. Wire the Orchestrator (or
a subset of its capabilities) into the Paladin execution context.

**Implementation notes:**
- Keep the agent-facing surface minimal and safe: expose only the orchestration actions that are
  intended to be agent-triggerable.
- Apply authorization/guardrails so an agent cannot schedule unbounded work or fire arbitrary events
  without constraints.
- Depend on abstractions (port/trait) so the bridge is unit-testable with a mock orchestrator.

**Deliverables:**
- Bridge implementation.
- `PaladinExecutionService` or `PaladinBuilder` updated to accept the bridge.
- Unit tests with mock orchestrator.

**Acceptance criteria:**
- An agent execution context can be constructed with the bridge attached.
- Each exposed action invokes the corresponding `Orchestrator` capability (verified with a mock).
- Guardrails reject out-of-policy actions.

---

### Task 4.3: Integration Test — Agent Triggers Orchestration

**Description:** Write an integration test: create a Paladin agent with orchestrator access →
execute the agent with a prompt that requires scheduling a follow-up → verify the job appears in the
scheduler/queue.

**Deliverables:**
- Integration test demonstrating the bidirectional flow.
- Uses mock LLM adapter with a predetermined tool-call response.

**Acceptance criteria:**
- The test deterministically drives the agent to issue an orchestration action via a scripted
  LLM/tool-call response.
- The resulting job/event/notification is observable in the orchestrator state.

---

## Definition of Done

- A design decision (Option A or B) is documented with rationale.
- The selected bridge is implemented, wired into the execution context, and unit tested.
- An integration test proves an agent can trigger an orchestration action.
- `cargo build`, `cargo test`, `cargo clippy -- -D warnings`, and `cargo fmt --check` all pass.
