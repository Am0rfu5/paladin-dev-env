---
phase: 05-milestone-2-3-ground-truth
plan: 12
subsystem: docs
tags: [ledger, requirements-traceability, autonomous-agents, cli, scheduler, config, adr-cited, fan-out-close]

# Dependency graph
requires:
  - phase: 05-milestone-2-3-ground-truth
    provides: "05-01's ledger scaffold (head notes, verdict legend, 118 PENDING-VERDICT stub rows) and its D-01 evidence bar; 05-03's ADR-0012 (Tier-3 live-API skip semantics); 05-04's ADR-0006 Phase 5 amendment (autonomous-components 92.80%/90% coverage figures); 05-06's Epic 14 findings (REQ-handoff-tool-v1 present/unproven and the competing handoff_to_specialist v2 shape, REQ-autonomous-configuration's CLI-flag dead-code finding, MaxLoops::Auto disconnection); 05-11's precedent for row shape and evidence bar"
provides:
  - "Epic 21 (Autonomous Agent Completion, 7 IDs) fully cited in .planning/ledgers/milestone-02-03.md"
  - "Epic 23 (CLI, Config & Infrastructure Completion, 10 IDs) fully cited, with an epic-level note on its lowest-open-item-count status"
  - "The ledger's last 17 rows filled — all 118 rows now carry a verdict; the fan-out this phase built closes here"
  - "REQ-handoff-tool-v2 resolves variant group 13 alongside Epic 14's REQ-handoff-tool-v1: this IS the wired dispatch shape"
  - "A new cross-cutting finding: 4 tests/cli/ test files (66 test functions) are commented out of tests/cli/mod.rs and never compiled into any test target — affects REQ-cli-garrison-configuration, REQ-cli-arsenal-configuration and REQ-cli-tiered-environment-testing"
  - "A new finding: two independent, non-duplicate TokioCronSchedulerAdapter implementations ship simultaneously (paladin-storage's feature-gated copy and src/infrastructure's unconditionally-compiled copy)"
affects: [05-13, "06 (Verified Gap Closure)"]

# Tech tracking
tech-stack:
  added: []
  patterns:
    - "Orphaned-test detection: cross-referencing a [[test]] target's own mod.rs against the files physically present in its directory surfaces test code that exists, compiles standalone, but is never linked into any binary Cargo actually runs — a distinct failure mode from a stale checkbox or a missing test, and one plain file-existence citation would have missed entirely"
    - "unimplemented!() re-derivation via per-file #[cfg(test)] boundary comparison: rather than trusting a bare occurrence count, each of the 27 tree-wide occurrences was checked against its own file's #[cfg(test)] line number to classify test-scaffold vs. production-path, closing the exact gap the plan's own action flagged as a run-2 risk"
    - "Grove-defect cross-reference pattern (continuing 05-06/05-blocks): REQ-autonomous-configurable-model and REQ-autonomous-completion-quality-gates both point at the same grove_service.rs:537 TODO already recorded genuinely outstanding under Epic 22's REQ-grove-llm-routing, rather than re-deriving or duplicating the finding"

key-files:
  created: []
  modified:
    - .planning/ledgers/milestone-02-03.md

key-decisions:
  - "REQ-handoff-tool-v2 verdicted satisfied, resolving variant group 13. PaladinBuilder::generate_handoff_tool() builds the handoff_to_specialist tool with specialist_name/task_description exactly as Epic 21/23 name it, auto-registered idempotently at build() time, and this IS the shape PaladinExecutionService::is_handoff_tool_call() actually dispatches on -- confirming from the other side what Epic 14's REQ-handoff-tool-v1 row (plan 05-06) already recorded about the wired path bypassing the Epic-14 HandoffTool struct."
  - "REQ-autonomous-configurable-model verdicted satisfied, with caveat. PlanningService and PromptGenerationService both take model as a parameter fed from the Paladin's actual configured model (not hardcoded 'gpt-4'), closing the named defect class. Two caveats recorded: no model-compatibility-validation-with-fallback exists at this layer (a test comment says that's the LlmPort's job), and subtask expected_output is still a hardcoded placeholder string carrying its own open TODO at planning_service.rs:430-433 -- a second, previously unrecorded TODO. Grove's survival of the same defect class is cross-referenced to Epic 22's REQ-grove-llm-routing rather than duplicated."
  - "REQ-paladin-result-autonomous-metadata and REQ-handoff-execution-integration both verdicted superseded by shipped code. PaladinResult.plan/.handoff_history default and deserialize correctly, but TaskPlan has no goal/created_at fields and HandoffRecord has no specialist_name/task_description/timestamp fields (uses from_agent/to_agent/task/depth:u32 instead) -- and MessagePack support and the 100-record cap/eviction do not exist anywhere in the tree. Retry policy and cycle detection ship with the PRD's exact defaults and are fully tested, but no circuit-breaker integration exists and the E-HANDOFF-00x error-code taxonomy is entirely absent (shipped HandoffError variants are named differently, e.g. InvalidAgent not SpecialistNotFound)."
  - "REQ-autonomous-orchestration-layers verdicted satisfied, with caveat. Three of the four layers (planning, prompts, dynamic temperature) are each gated by their own named bool field on PaladinData; the fourth (handoffs) is instead implicitly gated by whether the auto-registered handoff tool is present in the arsenal, not a named handoffs: bool field. No integration test exercises all four layers enabled together in combination."
  - "REQ-autonomous-completion-config-schema verdicted superseded by shipped code as the later position on Epic 14's REQ-autonomous-configuration. AutonomousConfig ships opt-in, validated at build time, but HandoffConfig has no concurrent, history.{max_records,eviction}, on_specialist_unavailable or inline specialists-list fields, and PlanningConfig has no validate_at field (though its effect -- validation always happens at build time -- is achieved anyway)."
  - "REQ-autonomous-completion-quality-gates verdicted deferred with reason, transcribing the amended ADR-0006's autonomous-components figure (92.80% aggregate against a >=90% target, owner Phase 15/PIPE-02) byte-identical, with zero re-measurement. The 'zero remaining TODO' clause is recorded contradicted twice: once via the already-tracked Grove TODO (cross-referenced to Epic 22), and once via a newly-found TODO inside this epic's own scope (planning_service.rs:433)."
  - "REQ-cli-garrison-configuration and REQ-cli-arsenal-configuration both verdicted present, unproven -- not because the implementation is missing (instantiate_garrison()/instantiate_arsenal() are real, and the old TODOs at agent.rs are gone) but because their only test coverage (tests/cli/garrison_config_test.rs's 8 tests, tests/cli/arsenal_config_test.rs's 10 tests) is commented out of tests/cli/mod.rs and never compiled into the cli [[test]] binary or any other target. instantiate_arsenal()'s streamable_http error paths are a partial exception: 3 inline loader.rs tests do exercise them and do pass."
  - "REQ-cli-tiered-environment-testing verdicted present, unproven for the same disconnected-test reason, extended: tests/cli/environment_tests.rs (45 tests) and tests/cli/integration_tests.rs (3 tests) are also commented out, and environment_tests.rs is the ONLY place NO_COLOR-detection and non-interactive-mode-detection are tested at all -- error_handling_test.rs (which IS wired) covers empty-input/large-input/malformed-YAML but not those two. Tier 3's contested skip semantics are settled by citing ADR-0012 rather than restating the deadlock; Tier 2's Docker-gated suite exists and is properly #[ignore]-gated."
  - "REQ-scheduler-port verdicted superseded by shipped code. Shipped tokio-cron-scheduler is 0.13.0 (Cargo.lock), not the PRD's pinned 0.9. New finding: two independent, non-duplicate TokioCronSchedulerAdapter structs ship simultaneously in two different crates -- crates/paladin-storage/src/scheduler.rs (feature-gated) and src/infrastructure/adapters/scheduling/tokio_cron_adapter.rs (unconditionally compiled, at the PRD's exact named path) -- both real, both independently tested (16/16 and 14 tests respectively)."
  - "REQ-content-deliverer-scheduling verdicted satisfied. The named unimplemented!() stub is gone, real scheduler-backed job creation/cancellation ships and is tested (5/5 integration tests). Re-derived, not transcribed: all 27 unimplemented!() occurrences across crates/ and src/ were individually checked against each file's own #[cfg(test)] boundary line -- every one sits inside a test module or a doc-comment example, zero in production code paths anywhere in the tree, directly answering the plan's explicit re-derivation instruction."
  - "REQ-cli-error-types verdicted superseded by shipped code. CliError has zero #[from] attributes anywhere (not the PRD's three typed conversions); GarrisonConfigError/ArsenalConfigError are string-message variants populated by manual .map_err() calls, and no SchedulerError-carrying variant exists at all."
  - "REQ-mock-arsenal-port and REQ-tool-call-loop-tests both verdicted satisfied cleanly. MockArsenalPort lives exactly at the PRD's named path and is reused (not duplicated) by the cli [[test]] target via a #[path] re-export shim. tool_integration_test.rs's 8 test functions match the PRD's 8 named tests verbatim, one-for-one, all passing."
  - "REQ-mcp-gated-integration-tests verdicted superseded by shipped code. tests/integration/tool_integration_mcp_test.rs (the PRD's named file) does not exist; the equivalent capability ships at tests/integration/mcp_stdio_test.rs instead, unconditionally compiled with zero #[ignore] attributes and no Python-availability skip check -- the PRD's gating requirement is unmet even though the functional capability (9 tests including exact calculator-tool assertions matching the PRD's own worked example) is real and passing."

requirements-completed: []  # VERIFY-01/VERIFY-02 span all of plans 05-01..05-13; not individually completable until 05-13 closes the ledger out

coverage:
  - id: D1
    description: "Epic 21's 7 REQ-* rows filled to the D-01 evidence bar, with the three previously-uninspected rows (REQ-paladin-result-autonomous-metadata, REQ-autonomous-orchestration-layers, REQ-handoff-execution-integration) recording actual findings and the coverage-gate row transcribing ADR-0006 without re-measuring"
    verification:
      - kind: unit
        ref: "cargo test --offline -p paladin-ai --lib -- handoff_tool -- 5/5 passed"
      - kind: unit
        ref: "cargo test --offline -p paladin-ai application::services::paladin::planning_service application::services::paladin::prompt_generation_service -- 19/19 passed"
        status: pass
      - kind: other
        ref: "grep -n 'model: \"gpt-4\"' crates/paladin-battalion/src/grove_service.rs -- confirms grove_service.rs:537 hardcoded model survives, cross-referenced not duplicated"
      - kind: other
        ref: "grep -rn 'rmp_serde|MessagePack' --include=*.rs . -- zero matches, confirms MessagePack absent"
      - kind: other
        ref: "grep -n 'CircuitBreaker|circuit_breaker' handoff_service.rs -- zero matches, confirms no circuit-breaker integration"
      - kind: other
        ref: "grep -rn 'E-HANDOFF|E-PLAN-|E-PROMPT-|E-CONFIG-' --include=*.rs . -- zero matches, confirms error-code taxonomy absent"
    human_judgment: true
    rationale: "Requires a human to confirm the REQ-handoff-tool-v2/REQ-handoff-tool-v1 pairing is not contradictory, that the REQ-autonomous-configurable-model caveat (validate/fallback + hardcoded-placeholder findings) is correctly scoped as a caveat rather than overriding the satisfied verdict, and that the transcribed 92.80%/90%/PIPE-02 coverage figures are byte-identical to ADR-0006's amendment with no re-measurement (same class of manual check as sibling plans 05-06/05-11)."
  - id: D2
    description: "Epic 23's 10 REQ-* rows filled to the D-01 evidence bar, with the six previously-uninspected/contested rows recording measured findings, the ADR-0012 citation resolving the Tier-3 skip semantics, and the ledger's last PENDING-VERDICT row closed -- all 118 rows now filled"
    verification:
      - kind: unit
        ref: "cargo test --offline --features cli --lib -- application::cli::config -- 28/28 passed"
      - kind: other
        ref: "cargo test --offline --features cli --test cli -- 106/106 passed"
      - kind: unit
        ref: "cargo test --offline -p paladin-storage --features scheduler scheduler:: -- 7/7 passed, then -- --ignored -- 9/9 passed (16 total)"
      - kind: integration
        ref: "cargo test --offline -p paladin-ai --test lib --features web-server -- scheduler_full_lifecycle scheduler_job_info deliverer_schedule deliverer_cancel deliverer_without_scheduler -- 5/5 passed"
      - kind: integration
        ref: "cargo test --offline -p paladin-ai --test lib -- integration::mcp_stdio_test -- 10/10 passed"
      - kind: other
        ref: "grep -c 'fn test_' tests/cli/tool_integration_test.rs -- exactly 8, matching the PRD's 8 named tests"
      - kind: other
        ref: "grep -rn 'unimplemented!' crates/ src/ -- 27 occurrences, each individually checked against its file's #[cfg(test)] boundary -- zero in production paths"
    human_judgment: true
    rationale: "Requires a human to confirm the orphaned-test-file finding (66 test functions across 4 files never compiled into any target) is correctly attributed to REQ-cli-garrison-configuration/REQ-cli-arsenal-configuration/REQ-cli-tiered-environment-testing without overstating or understating it, that the duplicate-TokioCronSchedulerAdapter finding on REQ-scheduler-port is accurately described as two genuinely independent implementations rather than a re-export, and that the epic-level note's transcribed 1-open-item count is correctly cited from task-completion-state.md (same class of manual check as sibling plans 05-06/05-11)."
  - id: D3
    description: "Ledger integrity preserved: exactly 118 REQ-* rows, 14 epic sections, no row inserted/deleted/reordered, no .rs/Cargo.toml/.github file touched -- the fan-out closes with all 118 rows filled"
    verification:
      - kind: other
        ref: "grep -c '^| REQ-' .planning/ledgers/milestone-02-03.md equals 118; grep -c 'PENDING-VERDICT' equals 0; git diff --stat -- '*.rs' 'Cargo.toml' '.github/' empty for both commits; git log -1 --name-only shows only the ledger file for both commits"
        status: pass
    human_judgment: false

duration: ~150min
completed: 2026-08-04
status: complete
---

# Phase 5 Plan 12: Epic 21 Autonomous Agent Completion and Epic 23 CLI/Config/Infrastructure ledger rows Summary

**Filled Epic 21's 7 rows and Epic 23's 10 rows in the Milestone 2-3 ledger, resolving the handoff-tool variant group from the wired-code side, finding a hardcoded-placeholder subtask-output TODO that widens Epic 21's coverage-gate row, and discovering that four `tests/cli/` files (66 test functions) plus a second, independently-implemented `TokioCronSchedulerAdapter` are disconnected from or duplicated across the currently-compiled test/build surface — closing all 118 ledger rows.**

## Performance

- **Duration:** ~150 min (dominated by cold-then-warming compiles across `paladin-ai`, `paladin-storage --features scheduler`, and `paladin-ai --features web-server`, plus a full per-file `#[cfg(test)]`-boundary re-derivation of 27 `unimplemented!()` occurrences)
- **Tasks:** 2
- **Files modified:** 1 (`.planning/ledgers/milestone-02-03.md`)

## Accomplishments

- Filled all 7 Epic 21 rows: `REQ-handoff-tool-v2`, `REQ-autonomous-configurable-model`, `REQ-paladin-result-autonomous-metadata`, `REQ-autonomous-orchestration-layers`, `REQ-handoff-execution-integration`, `REQ-autonomous-completion-config-schema`, `REQ-autonomous-completion-quality-gates`.
- Filled all 10 Epic 23 rows: `REQ-cli-garrison-configuration`, `REQ-cli-arsenal-configuration`, `REQ-mock-llm-adapter`, `REQ-cli-tiered-environment-testing`, `REQ-scheduler-port`, `REQ-content-deliverer-scheduling`, `REQ-cli-error-types`, `REQ-mock-arsenal-port`, `REQ-tool-call-loop-tests`, `REQ-mcp-gated-integration-tests`, plus an epic-level note transcribing Epic 23's 1-open-item count from `task-completion-state.md`.
- **Resolved variant group 13 from the wired-code side.** `REQ-handoff-tool-v2` verdicted `satisfied`: `PaladinBuilder::generate_handoff_tool()` builds exactly the `handoff_to_specialist`/`specialist_name`/`task_description` shape this ID names, and this IS what `PaladinExecutionService::is_handoff_tool_call()` actually dispatches on — confirming, from the other side, what Epic 14's `REQ-handoff-tool-v1` row (plan 05-06) already found about the wired path bypassing the Epic-14 `HandoffTool` struct.
- **Found a second, previously unrecorded TODO in Epic 21's own scope.** `planning_service.rs:430-433` still generates subtask `expected_output` as a hardcoded placeholder string with an open `// TODO: Ask LLM for expected output` — directly matching the PRD's own named acceptance criterion for `REQ-autonomous-configurable-model`, and widening `REQ-autonomous-completion-quality-gates`'s "zero remaining TODO" contradiction beyond the already-tracked Grove defect.
- **Traced `PaladinResult`'s shipped field shapes against the PRD precisely.** `TaskPlan` has no `goal`/`created_at` fields (ships `original_task`/`subtasks`/`dependencies`/`max_subtasks` instead); `HandoffRecord` has no `specialist_name`/`task_description`/`timestamp` fields (ships `from_agent`/`to_agent`/`task`/`depth: u32` instead); MessagePack support and the 100-record cap/eviction do not exist anywhere in the tree.
- **Found the retry-and-cycle-detection half of handoff execution fully real and tested, but circuit-breaker integration and the `E-HANDOFF-00x` error-code taxonomy entirely absent** — `HandoffError`'s shipped variants (`InvalidAgent`, `CircularHandoff`, `MaxDepthExceeded`, `ExecutionFailed`) match by behavior, not by name or code string.
- **Discovered a cross-cutting orphaned-test-file pattern spanning three Epic 23 rows.** `tests/cli/mod.rs` comments out four modules — `arsenal_config_test` (10 tests), `environment_tests` (45 tests), `garrison_config_test` (8 tests), `integration_tests` (3 tests) — 66 test functions total, none compiled into the `cli` `[[test]]` binary or any other target. `environment_tests.rs` is the sole location testing NO_COLOR-detection and non-interactive-mode-detection at all.
- **Re-derived the tree-wide `unimplemented!()` sweep per the plan's explicit instruction, not transcribed from run-2.** All 27 occurrences across `crates/` and `src/` were individually checked against each file's own `#[cfg(test)]` boundary line; every one sits inside a test module or a doc-comment example. Zero in production code paths anywhere in the tree — `REQ-content-deliverer-scheduling` verdicted `satisfied` on this basis.
- **Found a previously unrecorded architectural duplication.** Two independent, non-duplicate `TokioCronSchedulerAdapter` structs ship simultaneously: `crates/paladin-storage/src/scheduler.rs` (feature-gated, `try_new()`/`engine`/`registrations`) and `src/infrastructure/adapters/scheduling/tokio_cron_adapter.rs` (unconditionally compiled at the PRD's exact named path, `new()`/`inner`/`jobs`) — both real `impl SchedulerPort` blocks, both independently tested (16 and 14 tests respectively).
- Ran 9 distinct scoped `cargo test` commands during this task, all passing where expected: 5 handoff-tool tests, 19 planning/prompt-service tests, 28 CLI-config `--lib` tests, 106 full `cli` `[[test]]` target tests, 16 scheduler storage-layer tests, 5 scheduler-integration tests (`--features web-server`), 10 MCP STDIO tests — 179 total test passes, cited by name in the rows that rely on them.

## Task Commits

1. **Task 1: Fill Epic 21's 7 rows** — `f2de1e4` (docs)
2. **Task 2: Fill Epic 23's 10 rows, close all 118** — `98e7f3a` (docs)

`f2de1e4` — `docs(05-12): fill Epic 21 autonomous agent completion ledger rows`
`98e7f3a` — `docs(05-12): fill Epic 23 CLI/config/infrastructure ledger rows, close all 118`

_No separate plan-metadata commit — SUMMARY.md is committed by this same worktree per the parallel-execution instructions; STATE.md/ROADMAP.md updates are owned by the orchestrator after the wave merges._

**Worktree hook policy note:** this repo's pre-commit hooks (`cargo fmt`, `cargo clippy --workspace --all-targets --all-features`, both `always_run: true`) would cold-compile the entire 12-crate workspace on every commit including markdown-only ones. Per `workflow.worktree_skip_hooks=true`, `--no-verify` was used for both commits, matching plans 05-01/05-05/05-06/05-07/05-08/05-09/05-10/05-11's precedent in this phase. Task 1's row content was committed immediately after its own verification passed rather than deferred to a single end-of-plan commit, following this executor's parallel-execution pacing instructions (commit early and often) over the plan's own "commit once, after Task 2" phrasing — the same documented override sibling plan 05-11 applied and recorded as a deviation.

## Files Created/Modified

- `.planning/ledgers/milestone-02-03.md` — Epic 21 section (7 rows) and Epic 23 section (10 rows plus one epic-level note): replaced all 17 remaining `PENDING-VERDICT` stub rows with cited verdicts. No other epic section touched; row count (118) and section count (14) both verified unchanged outside Epics 21/23. **All 118 ledger rows now carry a filled verdict.**

## Decisions Made

See `key-decisions` in the frontmatter for the full, citation-bearing list. Summarized:
- `REQ-handoff-tool-v2` → `satisfied`, resolving variant group 13 from the wired-code side.
- `REQ-autonomous-configurable-model` → `satisfied, with caveat`; defect closed in two files, survives in Grove (cross-referenced), plus a newly-found hardcoded-placeholder TODO.
- `REQ-paladin-result-autonomous-metadata` and `REQ-handoff-execution-integration` → `superseded by shipped code`; both real capabilities with materially different field/error shapes than the PRD.
- `REQ-autonomous-orchestration-layers` → `satisfied, with caveat`; three of four layers have named bool toggles, the fourth is implicit; no all-features-combined test exists.
- `REQ-autonomous-completion-config-schema` → `superseded by shipped code`, the later position on Epic 14's `REQ-autonomous-configuration`.
- `REQ-autonomous-completion-quality-gates` → `deferred with reason`, transcribing ADR-0006's 92.80%/90% figures verbatim, with the "zero TODO" clause doubly contradicted.
- `REQ-cli-garrison-configuration`/`REQ-cli-arsenal-configuration`/`REQ-cli-tiered-environment-testing` → `present, unproven`, driven by the orphaned-test-file finding.
- `REQ-scheduler-port` → `superseded by shipped code`; version divergence plus the duplicate-adapter finding.
- `REQ-content-deliverer-scheduling` → `satisfied`, backed by the full re-derived `unimplemented!()` sweep.
- `REQ-cli-error-types` → `superseded by shipped code`; zero `#[from]` attributes anywhere, no `SchedulerError` variant.
- `REQ-mock-arsenal-port` and `REQ-tool-call-loop-tests` → `satisfied` cleanly.
- `REQ-mcp-gated-integration-tests` → `superseded by shipped code`; real capability, unmet gating requirement.

## Deviations from Plan

**1. [Process] Committed each task immediately after its own verification passed, rather than a single end-of-plan commit.** Task 1's action says "Do not commit yet — this plan commits once, after Task 2," and Task 2's action says to commit once at the end with a 300000ms timeout, without `--no-verify`. This executor's parallel-execution instructions explicitly direct committing early and often, and separately authorize `--no-verify` for every commit given `workflow.worktree_skip_hooks=true`. Followed the more specific, risk-mitigating runtime instruction — committed Epic 21's rows (`f2de1e4`) after Task 1's verification passed, then Epic 23's rows (`98e7f3a`) after Task 2's verification passed, both with `--no-verify`, matching the identical override already documented and applied by sibling plans 05-01, 05-05 through 05-11 in this same phase. Neither the row content, the verdicts, nor the verification results differ from what a single end-of-plan commit would have produced.

Neither deviation changed the ledger's content, verdicts, or evidence — this is a process-only accommodation to this worktree's execution environment, consistent with prior plans in this phase.

## Issues Encountered

- **Distinguishing the two `TokioCronSchedulerAdapter` implementations required reading both files' internal field names and constructors**, not just their file paths, to confirm they are genuinely independent implementations rather than one re-exporting the other (as `api_content_deliverer.rs` turned out to be, via `src/infrastructure/adapters/output/mod.rs`'s `pub use paladin_web::adapters::api_content_deliverer;`).
- **The `unimplemented!()` re-derivation required checking each of 27 occurrences individually against its own file's `#[cfg(test)]` boundary line** rather than trusting a single tree-wide count, since a naive occurrence count would have looked like a large regression when in fact all 27 are test-scaffold or doc-comment examples.
- **The orphaned `tests/cli/` files' own comment** (`tests/cli/mod.rs:6-9`) attributes their exclusion to a prior milestone's Phase 2 scope decision (`.planning/phases/02-functional-gap-closure/02-CONTEXT.md D-09`) that no longer exists in this milestone's `.planning/` tree (archived at a prior milestone close) — the comment's provenance could not be independently re-verified, but the disconnection fact itself (checked directly against `Cargo.toml`'s `[[test]]` target and `tests/cli/mod.rs`'s active `mod` declarations) is not in doubt.

## User Setup Required

None — no external service configuration required.

## Next Phase Readiness

- Epic 21's and Epic 23's ledger sections are complete: 17 cited `REQ-*` rows, one epic-level note. **Every one of the 118 ledger rows in `.planning/ledgers/milestone-02-03.md` now carries a filled verdict — the fan-out this phase built (plans 05-01 through 05-12) closes here.**
- Plan 05-13 is positioned to close the ledger out: count the final verdict distribution across all 118 rows, and adjudicate the three block verdicts this phase records (Epic 14 `partially outstanding`, Epic 22 `satisfied by shipped code`).
- Several named, concrete gaps are available for Phase 6 to scope if prioritized (not raised as new CLOSE-0x requirements here, since this plan's `must_haves` scope only the ledger rows themselves): (1) re-wiring the four orphaned `tests/cli/` modules (66 test functions) into `tests/cli/mod.rs`; (2) the `planning_service.rs:433` hardcoded subtask-output TODO; (3) circuit-breaker integration for handoff retries; (4) the two independent `TokioCronSchedulerAdapter` implementations, which will eventually need reconciling or an explicit "both stay, here's why" decision; (5) `CliError`'s missing `SchedulerError` variant.
- No blockers for the next wave.

## Self-Check: PASSED

- FOUND: `.planning/ledgers/milestone-02-03.md` (modified, Epic 21 and Epic 23 sections)
- FOUND: `.planning/phases/05-milestone-2-3-ground-truth/05-12-SUMMARY.md`
- FOUND: commit `f2de1e4` (Task 1, ledger file only)
- FOUND: commit `98e7f3a` (Task 2, ledger file only)

---
*Phase: 05-milestone-2-3-ground-truth*
*Completed: 2026-08-04*
