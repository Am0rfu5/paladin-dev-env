# Task Completion State (deterministic)

Extracted 2026-07-30 by counting GFM checkboxes in all 75 `tasks-*.md` files under
`.project/`. NOT produced by an LLM classifier -- these are literal `- [x]` / `- [ ]`
counts. Source of truth for what the historical plan considered done.

**Caveat:** checkbox state reflects what the task author marked at the time. Ingest run 1
already found two Milestone-1 items marked open that are in fact implemented in shipped
code (Chain of Command, Herald wiring). Treat open counts as *claims to verify*, not as
confirmed remaining work.

## Milestone_1-MVP

- Task lists: 11  |  done: 1817  |  open: 39  |  97.9% complete
- Open items by list (descending):
  - `.project/Milestone_1-MVP/Epic_6/tasks-provider-expansion.md` -- 19 open
  - `.project/Milestone_1-MVP/Epic_2/tasks-garrison-memory-system.md` -- 4 open
  - `.project/Milestone_1-MVP/Epic_5/tasks-commander-strategy-router.md` -- 4 open
  - `.project/Milestone_1-MVP/Epic_3/tasks-arsenal-tool-system.md` -- 3 open
  - `.project/Milestone_1-MVP/Epic_9/tasks-armory-cli-tools.md` -- 3 open
  - `.project/Milestone_1-MVP/Epic_4/tasks-battalion-orchestration.md` -- 2 open
  - `.project/Milestone_1-MVP/Epic_8/tasks-herald-output-formatting.md` -- 2 open
  - `.project/Milestone_1-MVP/unit-test-improvements/tasks-improve-unit-test-coverage.md` -- 2 open

## Milestone_10-CI-Hardening-Release-Automation

- Task lists: 5  |  done: 173  |  open: 0  |  100.0% complete
- No open items.

## Milestone_11-Documentation-Overhaul-Publish

- Task lists: 5  |  done: 301  |  open: 26  |  92.0% complete
- Open items by list (descending):
  - `.project/Milestone_11-Documentation-Overhaul-Publish/Epic_3/tasks-content-rewrite.md` -- 26 open

## Milestone_12-Web-API

- Task lists: 7  |  done: 290  |  open: 3  |  99.0% complete
- Open items by list (descending):
  - `.project/Milestone_12-Web-API/Epic_5/tasks-api-security-authorization.md` -- 3 open

## Milestone_2-Missing_features

- Task lists: 9  |  done: 1867  |  open: 298  |  86.2% complete
- Open items by list (descending):
  - `.project/Milestone_2-Missing_features/Epic_15/tasks-conclave-mixture-of-agents.md` -- 129 open
  - `.project/Milestone_2-Missing_features/Epic_11/tasks-sanctum-memory-foundation.md` -- 111 open
  - `.project/Milestone_2-Missing_features/Epic_14/tasks-autonomous-agent-features.md` -- 45 open
  - `.project/Milestone_2-Missing_features/Epic_18/tasks-epic-18-cli-enhancement.md` -- 12 open
  - `.project/Milestone_2-Missing_features/Epic_17.5/tasks-cli-consolidation.md` -- 1 open

## Milestone_3-Completion

- Task lists: 7  |  done: 1188  |  open: 132  |  90.0% complete
- Open items by list (descending):
  - `.project/Milestone_3-Completion/Epic_22/tasks-epic22-battalion-commander-hardening.md` -- 81 open
  - `.project/Milestone_3-Completion/Epic_24/tasks-test-hardening-benchmarks-qa.md` -- 29 open
  - `.project/Milestone_3-Completion/Epic_21/tasks-autonomous-agent-completion.md` -- 12 open
  - `.project/Milestone_3-Completion/Epic_20/tasks-vision-pipeline-completion.md` -- 5 open
  - `.project/Milestone_3-Completion/Epic_19/tasks-epic19-herald-consolidation.md` -- 4 open
  - `.project/Milestone_3-Completion/Epic_23/tasks-task46-arsenal-tool-integration-tests.md` -- 1 open

## Milestone_4-Refactor-Crates-Features

- Task lists: 3  |  done: 276  |  open: 20  |  93.2% complete
- Open items by list (descending):
  - `.project/Milestone_4-Refactor-Crates-Features/Epic_2/tasks-harden-port-traits-stable-api.md` -- 20 open

## Milestone_5-Workspace-Decomposition

- Task lists: 6  |  done: 459  |  open: 17  |  96.4% complete
- Open items by list (descending):
  - `.project/Milestone_5-Workspace-Decomposition/Epic_6/tasks-workspace-finalization-epic-6.md` -- 7 open
  - `.project/Milestone_5-Workspace-Decomposition/Epic_1/tasks-workspace-initialization-and-paladin-core-extraction.md` -- 5 open
  - `.project/Milestone_5-Workspace-Decomposition/Epic_3/tasks-paladin-battalion-extraction.md` -- 4 open
  - `.project/Milestone_5-Workspace-Decomposition/Epic_4/tasks-paladin-llm-extraction.md` -- 1 open

## Milestone_6-Architectural-Refinements

- Task lists: 4  |  done: 300  |  open: 0  |  100.0% complete
- No open items.

## Milestone_7-Production-Hardening

- Task lists: 4  |  done: 255  |  open: 3  |  98.8% complete
- Open items by list (descending):
  - `.project/Milestone_7-Production-Hardening/Epic_2/tasks-production-build-infra-adaptation.md` -- 3 open

## Milestone_8-Facade-Cleanup-Shim-Resolution

- Task lists: 7  |  done: 345  |  open: 3  |  99.1% complete
- Open items by list (descending):
  - `.project/Milestone_8-Facade-Cleanup-Shim-Resolution/Epic_2/tasks-remove-dead-shims-empty-modules.md` -- 2 open
  - `.project/Milestone_8-Facade-Cleanup-Shim-Resolution/Epic_3/tasks-relocate-remaining-misplaced-modules.md` -- 1 open

## Milestone_9-Classic-Orchestrator-Completion

- Task lists: 6  |  done: 240  |  open: 0  |  100.0% complete
- No open items.

## project-management

- Task lists: 1  |  done: 0  |  open: 1  |  0.0% complete
- Open items by list (descending):
  - `.project/project-management/tasks-project-management-setup.md` -- 1 open

## Totals

- done: 7511  |  open: 542  |  93.3% complete across 75 task lists

Largest open concentrations (candidate forward work, pending verification):
- 129 open -- `.project/Milestone_2-Missing_features/Epic_15/tasks-conclave-mixture-of-agents.md`
- 111 open -- `.project/Milestone_2-Missing_features/Epic_11/tasks-sanctum-memory-foundation.md`
- 81 open -- `.project/Milestone_3-Completion/Epic_22/tasks-epic22-battalion-commander-hardening.md`
- 45 open -- `.project/Milestone_2-Missing_features/Epic_14/tasks-autonomous-agent-features.md`
- 29 open -- `.project/Milestone_3-Completion/Epic_24/tasks-test-hardening-benchmarks-qa.md`
- 26 open -- `.project/Milestone_11-Documentation-Overhaul-Publish/Epic_3/tasks-content-rewrite.md`
- 20 open -- `.project/Milestone_4-Refactor-Crates-Features/Epic_2/tasks-harden-port-traits-stable-api.md`
- 19 open -- `.project/Milestone_1-MVP/Epic_6/tasks-provider-expansion.md`
- 12 open -- `.project/Milestone_2-Missing_features/Epic_18/tasks-epic-18-cli-enhancement.md`
- 12 open -- `.project/Milestone_3-Completion/Epic_21/tasks-autonomous-agent-completion.md`
