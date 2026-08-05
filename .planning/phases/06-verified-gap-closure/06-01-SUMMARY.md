---
phase: 06-verified-gap-closure
plan: 01
subsystem: battalion
tags: [grove, routing, llm, rust, tdd, hexagonal]

# Dependency graph
requires:
  - phase: 05-milestone-2-3-ground-truth
    provides: the verified CLOSE-01 defect (grove_service.rs:537 hardcoded model literal) and ROADMAP success criteria 1/2
provides:
  - "GroveConfig.routing_model: Option<String> — operator-configured LLM routing model, additive to the serde surface"
  - "GroveBuilder.routing_model(..) fluent setter"
  - "Grove LLM routing hard-errors (BattalionError::RoutingError) with no fallback of any kind when routing_model is absent or blank, under RoutingStrategy::LlmRouting"
  - "RecordingLlmMock test convention in grove_service.rs proving a non-OpenAI model reaches LlmPort::generate"
affects: [06-06 (ADR-0013 authoring cites this plan's checkpoint outcome and rustdoc)]

# Tech tracking
tech-stack:
  added: []
  patterns: [config-sourced model never a literal, .ok_or_else guard idiom for missing-config hard errors]

key-files:
  created: []
  modified:
    - crates/paladin-core/src/platform/container/battalion/grove.rs
    - crates/paladin-battalion/src/grove_service.rs
    - examples/commander_grove.rs
    - examples/grove_routing.rs
    - tests/integration/battalion/grove_integration_test.rs

key-decisions:
  - "Task 1 checkpoint (D-02 reversibility gate): user selected proceed-as-locked on 2026-08-05 — D-02 ships as locked, a missing routing_model under LlmRouting is a hard BattalionError::RoutingError with no fallback of any kind."
  - "Whitespace-only routing_model (Some(\"\") / Some(\"   \")) resolves to the same RoutingError as None — planner-resolved edge per the plan's own reasoning that a blank model name is a configuration error by the same logic D-02 applies to an absent one."

patterns-established:
  - "Config-sourced model, never a literal: LlmRequest.model is sourced from a guarded config value (grove.node.config.routing_model), never an inline literal — mirrors the Epic 21 precedent in planning_service.rs."

requirements-completed: [CLOSE-01]

coverage:
  - id: D1
    description: "GroveConfig.routing_model threaded through GroveBuilder, serde-additive, with rustdoc stating it is required under LlmRouting and errors when absent"
    requirement: "CLOSE-01"
    verification:
      - kind: unit
        ref: "crates/paladin-core/src/platform/container/battalion/grove.rs#test_grove_builder_sets_routing_model"
        status: pass
      - kind: unit
        ref: "crates/paladin-core/src/platform/container/battalion/grove.rs#test_grove_config_default_routing_model_is_none"
        status: pass
      - kind: unit
        ref: "crates/paladin-core/src/platform/container/battalion/grove.rs#test_grove_config_omits_routing_model_when_none"
        status: pass
      - kind: unit
        ref: "crates/paladin-core/src/platform/container/battalion/grove.rs#test_grove_config_deserializes_without_routing_model_key"
        status: pass
      - kind: unit
        ref: "crates/paladin-core/src/platform/container/battalion/grove.rs#test_grove_config_is_send_and_sync"
        status: pass
    human_judgment: false
  - id: D2
    description: "A configured non-OpenAI model (deepseek-chat) reaches LlmPort::generate via Grove LLM routing"
    requirement: "CLOSE-01"
    verification:
      - kind: unit
        ref: "crates/paladin-battalion/src/grove_service.rs#test_llm_routing_uses_configured_routing_model"
        status: pass
    human_judgment: false
  - id: D3
    description: "Grove LLM routing hard-errors with no fallback of any kind when routing_model is absent, blank, or the fallback would otherwise succeed via keyword matching"
    requirement: "CLOSE-01"
    verification:
      - kind: unit
        ref: "crates/paladin-battalion/src/grove_service.rs#test_llm_routing_errors_when_routing_model_absent"
        status: pass
      - kind: unit
        ref: "crates/paladin-battalion/src/grove_service.rs#test_llm_routing_errors_when_routing_model_empty"
        status: pass
      - kind: unit
        ref: "crates/paladin-battalion/src/grove_service.rs#test_llm_routing_missing_model_error_precedes_keyword_fallback"
        status: pass
    human_judgment: false
  - id: D4
    description: "Two concurrently-executing Groves with different routing_model values each cause their own configured model to reach the LLM call; GroveConfig/Grove remain Send + Sync"
    requirement: "CLOSE-01"
    verification:
      - kind: unit
        ref: "crates/paladin-battalion/src/grove_service.rs#test_concurrent_groves_use_their_own_routing_model"
        status: pass
    human_judgment: false
  - id: D5
    description: "No OpenAI model literal or deferral comment remains in grove_service.rs's production region; workspace build/test/fmt/clippy all green"
    requirement: "CLOSE-01"
    verification:
      - kind: other
        ref: "awk '/^#\\[cfg\\(test\\)\\]/{exit} {print}' crates/paladin-battalion/src/grove_service.rs | grep -c 'gpt-4' => 0"
        status: pass
      - kind: other
        ref: "grep -rn 'TODO' crates/paladin-battalion/src/ | grep -c 'grove_service.rs' => 0"
        status: pass
      - kind: integration
        ref: "cargo test --workspace"
        status: pass
      - kind: other
        ref: "cargo fmt --check && cargo clippy --workspace --all-targets -- -D warnings"
        status: pass
    human_judgment: false

duration: ~55min
completed: 2026-08-05
status: complete
---

# Phase 6 Plan 01: Grove LLM Routing Model Configuration Summary

**`GroveConfig.routing_model: Option<String>` threaded through `GroveBuilder` into `grove_service.rs`'s `route_by_llm`, replacing the hardcoded `model: "gpt-4"` literal with a hard `BattalionError::RoutingError` (no fallback) when the operator has not configured a routing model.**

## Performance

- **Duration:** ~55 min
- **Started:** 2026-08-05 (continuation dispatch, resumed at Task 2)
- **Completed:** 2026-08-05
- **Tasks:** 3 (1 checkpoint:decision already resolved by human before this dispatch; 2 code tasks executed in this session)
- **Files modified:** 5 (2 declared in the plan's `files_modified`, 3 additional per the documented deviation below)

## Task 1 Checkpoint Resolution (recorded per plan acceptance criteria)

**Decision:** `proceed-as-locked`
**Date:** 2026-08-05
**Resolved by:** human user, via the orchestrator, before this continuation dispatch began.

D-02 ships as locked: a missing `routing_model` under `RoutingStrategy::LlmRouting` is a hard `BattalionError::RoutingError` with no fallback of any kind — no serde default, no warning-plus-default, no retained model literal. The prior executor instance that reached this gate made zero commits and touched zero files before returning it to the orchestrator; this instance started from a clean worktree at the wave base and executed Tasks 2 and 3 exactly as written, per the checkpoint resolution. This outcome is cited by plan 06-06's ADR-0013.

## Accomplishments

- `GroveConfig.routing_model: Option<String>` added, serde-additive (`#[serde(skip_serializing_if = "Option::is_none")]`), with rustdoc stating it is required under `RoutingStrategy::LlmRouting` and that routing errors when absent.
- `GroveBuilder.routing_model(..)` fluent setter, threaded through `GroveBuilder::config()` and `GroveBuilder::build()`.
- `route_by_llm` in `grove_service.rs` no longer hardcodes `"gpt-4"`: a guard (same `.ok_or_else` idiom as the pre-existing `llm_port` guard) treats `None` and blank-after-trim as unconfigured and returns `BattalionError::RoutingError` naming `routing_model`, before the prompt is built and before any call to `llm_port.generate`. The guard consults no fallback of any kind — not `routing_fallback`, not `handle_routing_failure`, not the LLM port's available-models query.
- `RecordingLlmMock` added to `grove_service.rs`'s existing in-file mock convention, capturing every `LlmRequest.model` it receives.
- Ten new tests across the two files proving: the configured-model path, the missing/absent path, the empty/whitespace path, fallback-precedence, serde round-trip stability, and `Send + Sync`.

## Task Commits

Each task was committed atomically (Task 2 followed RED/GREEN TDD per its `tdd="true"` attribute; Task 3 landed as a single test commit since its cases were either new failure-mode tests or witnesses of behavior Task 2's guard already provided):

1. **Task 1: Confirm the one-way runtime break in Grove LLM routing (D-02)** — checkpoint:decision, resolved `proceed-as-locked` by the human user prior to this dispatch (no commit; zero files touched by that prior instance).
2. **Task 2: End-to-end "a configured non-OpenAI model reaches the LLM call" — one path only** (tracer, tdd=true):
   - `9b3a571` (test) — RED: `GroveConfig.routing_model` + builder wiring + two direct grove.rs tests, plus `RecordingLlmMock` and two failing grove_service.rs tests. Confirmed failing for the expected reason before commit.
   - `0ea5502` (feat) — GREEN: guard + config-sourced model in `route_by_llm`; updated four pre-existing `route_by_llm` tests that would otherwise have hit the new guard before exercising their intended behavior.
3. **Task 3: Prove the edges — empty, fallback precedence, serde stability, concurrency**:
   - `05ee6b4` (test) — six edge-case tests across both files, plus the auto-fix deviation to three files outside the plan's declared scope (see Deviations below).

_Tracer feedback gate: Task 2's `<verify>` (`cargo test -p paladin-ai-core --lib grove:: && cargo test -p paladin-battalion --lib grove_service::`) was re-run and passed before Task 3 began, per the autonomous-run tracer gate (this is a non-interactive worktree wave dispatch continuing from an already-resolved checkpoint)._

**Plan metadata:** this commit (docs: complete plan) — see Final Commit below.

## Files Created/Modified

- `crates/paladin-core/src/platform/container/battalion/grove.rs` — `GroveConfig.routing_model` field + rustdoc, `Default` wiring, `GroveBuilder.routing_model` field + setter + `config()`/`build()` wiring, five new tests.
- `crates/paladin-battalion/src/grove_service.rs` — `route_by_llm` guard + config-sourced `LlmRequest.model`, `RecordingLlmMock`, eight new tests, four pre-existing tests updated to set `routing_model`.
- `examples/commander_grove.rs` — three struct-literal `GroveConfig` construction sites updated (deviation, see below).
- `examples/grove_routing.rs` — one struct-literal `GroveConfig` construction site updated (deviation, see below).
- `tests/integration/battalion/grove_integration_test.rs` — `.routing_model("gpt-4")` added to `test_grove_llm_routing_end_to_end`'s builder chain (deviation, see below).

## Decisions Made

- Task 1 checkpoint: `proceed-as-locked` (recorded above, cited by plan 06-06's ADR-0013).
- Whitespace-only `routing_model` (`Some("")` / `Some("   ")`) treated identically to `None` — a blank model name is a configuration error by the same reasoning D-02 applies to an absent one, and passing it through would send a blank model string to a provider. Trim-then-check is implemented in the guard itself, so this required no separate implementation path beyond the guard already written for Task 2.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Fixed three call sites outside `files_modified` that broke `cargo test --workspace` as a direct, mechanical consequence of `GroveConfig` gaining a required struct field**
- **Found during:** Task 3 (`cargo test --workspace` acceptance check)
- **Issue:** `GroveConfig` has no `Default`-based struct-update pattern at three struct-literal construction sites; adding `routing_model` broke their compilation, and one integration test (`test_grove_llm_routing_end_to_end`) that builds a `RoutingStrategy::LlmRouting` Grove with a real `llm_port` started failing its assertions once the new guard fired (it never set `routing_model`).
- **Fix:**
  - `examples/commander_grove.rs`: added `routing_model: None` to the `KeywordMatch` and `SemanticSimilarity` example configs; added `routing_model: Some("gpt-4".to_string())` to the `LlmRouting` example config (matches its own printed "LLM Model: gpt-4" description, and demonstrates correct configured usage).
  - `examples/grove_routing.rs`: added `routing_model: None` to its `KeywordMatch` config.
  - `tests/integration/battalion/grove_integration_test.rs`: added `.routing_model("gpt-4")` to `test_grove_llm_routing_end_to_end`'s `GroveBuilder` chain. `test_grove_llm_routing` (the other `LlmRouting` integration test) was unaffected — its `llm_port` is `None`, so it hits the pre-existing `llm_port` guard before reaching the new `routing_model` guard, and falls back to keyword matching exactly as before.
- **Files modified:** `examples/commander_grove.rs`, `examples/grove_routing.rs`, `tests/integration/battalion/grove_integration_test.rs`
- **Verification:** `cargo test --workspace` — 0 failed (was 3 example-compile errors + 1 integration test failure before the fix).
- **Committed in:** `05ee6b4` (Task 3 commit)

**2. [Rule 1 - Bug] Updated four pre-existing `route_by_llm` tests that the new guard broke by intercepting before their intended behavior**
- **Found during:** Task 2 (GREEN phase `cargo test` run)
- **Issue:** `test_route_with_llm_successful`, `test_route_with_llm_low_confidence`, `test_route_with_llm_invalid_json`, and `test_route_with_llm_fallback_to_keyword` all call `route_by_llm` directly on a `create_test_grove()` Grove without setting `routing_model`. The new guard fires first, so three failed outright and the fourth (`invalid_json`) would have incidentally passed for the wrong reason (guard error instead of JSON-parse error), silently losing coverage of its named behavior.
- **Fix:** Added `grove.node.config.routing_model = Some("mock-model".to_string());` to each of the four test Groves so they continue exercising the behavior each is named for.
- **Files modified:** `crates/paladin-battalion/src/grove_service.rs`
- **Verification:** `cargo test -p paladin-battalion --lib grove_service::` — 17/17 pass (this fix's commit); 20/20 pass after Task 3's additions.
- **Committed in:** `0ea5502` (Task 2 GREEN commit)

**3. [Rule 3 - Blocking] Reworded a guard comment to avoid a literal grep match**
- **Found during:** Task 3 acceptance-criteria check
- **Issue:** The guard's explanatory comment textually contained the string `get_available_models()`, which the plan's own acceptance criterion (`awk ... | grep -c 'get_available_models'` must output `0`) treats as a literal match regardless of comment-vs-code context.
- **Fix:** Reworded the comment to describe the same invariant ("must not query the LLM port for its available models") without the literal substring.
- **Files modified:** `crates/paladin-battalion/src/grove_service.rs`
- **Verification:** `grep -c 'get_available_models'` on the production region now outputs `0`.
- **Committed in:** `0ea5502` (Task 2 GREEN commit)

---

**Total deviations:** 3 auto-fixed (2 Rule 3 - blocking, 1 Rule 1 - bug)
**Impact on plan:** All three were mechanical, necessary consequences of the plan's own intended change (a new required config field + a new hard-error guard). None expands scope beyond CLOSE-01; all were required to satisfy the plan's own `<verification>` section (`cargo test --workspace` green).

## Issues Encountered

None beyond the deviations documented above.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- CLOSE-01 is closed in code: ROADMAP success criteria 1 and 2 are satisfied (hardcoded literal gone, model comes from configuration, `grove_service.rs`'s only TODO resolved).
- D-01, D-02, D-04 fully implemented; D-03's rustdoc half is in place on `GroveConfig.routing_model`. The ADR-0013/CHANGELOG halves of D-03 are plan 06-06's responsibility, which can now cite this plan's checkpoint outcome and the shipped rustdoc verbatim.
- No blockers for downstream plans in this phase (06-02 through 06-07 do not depend on this plan's `files_modified`, per this plan's `depends_on: []` and the phase's wave structure).

---
*Phase: 06-verified-gap-closure*
*Completed: 2026-08-05*

## Self-Check: PASSED

- FOUND: `crates/paladin-core/src/platform/container/battalion/grove.rs`
- FOUND: `crates/paladin-battalion/src/grove_service.rs`
- FOUND: `examples/commander_grove.rs`
- FOUND: `examples/grove_routing.rs`
- FOUND: `tests/integration/battalion/grove_integration_test.rs`
- FOUND: commit `9b3a571` (test — RED)
- FOUND: commit `0ea5502` (feat — GREEN)
- FOUND: commit `05ee6b4` (test — Task 3 edges)
