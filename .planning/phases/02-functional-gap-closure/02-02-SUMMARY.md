---
phase: 02-functional-gap-closure
plan: 02
subsystem: api
tags: [llm-port, provider-capabilities, temperature-validation, tool-calling, adr-0004, web-03]

# Dependency graph
requires:
  - phase: 02-functional-gap-closure
    provides: "02-01's measured cargo test --workspace baseline (2790 passed / 0 failed / 126
      ignored on commit 7e55655) — the pre-change tree this plan's own full-suite run is compared
      against"
provides:
  - "`temperature_range: Option<(f32, f32)>` on `ProviderCapabilities` (paladin-ports), landed as
    the phase's tracer per D-16 — the widest-blast-radius change in Phase 2, proven end-to-end
    before any expansion task"
  - "DeepSeek's documented 0.0-2.0 temperature range reachable through the normal
    `PaladinBuilder` path for the first time (ADR-0004's motivating case, GAP-07)"
  - "Provider-aware temperature validation in `PaladinBuilder::validate` — provider range checked
    first, inclusive at both endpoints, `[0.0, 1.0]` named fallback when a provider declares none,
    never clamps"
  - "`supports_tool_calling: false` on all three shipped adapters (OpenAI, Anthropic, DeepSeek),
    replacing OpenAI/Anthropic's over-reported `true` (WEB-03, D-14) — pulled forward from Phase 14
    per D-13"
  - "A correspondence test (`test_capabilities_tool_calling_matches_request_surface`) that ties the
    tool-calling flag to whether `LlmRequest` actually carries a tools field, satisfying WEB-03's
    own success criterion 3 literally"
  - "The corrected, compiler-verified list of every `ProviderCapabilities` exhaustive construction
    site in the workspace (see 'Construction-site list' below) — CONTEXT.md D-15's list, compiled
    by grep, missed the OpenAI and Anthropic adapters themselves"
affects: [02-09-amend-ledger, phase-14-web-03]

# Tech tracking
tech-stack:
  added: []
  patterns:
    - "Provider-aware capability validation: read `self.llm_port.get_capabilities().<field>` from
      the `Arc<dyn LlmPort>` a builder already holds, rather than a hardcoded constant"
    - "Crate-level cross-adapter test module (`capability_invariants` in a leaf crate's `lib.rs`)
      for invariants that need every provider adapter simultaneously in scope, gated on all
      relevant provider features being enabled together"

key-files:
  created: []
  modified:
    - crates/paladin-ports/src/output/llm_port.rs
    - crates/paladin-llm/src/deepseek/adapter.rs
    - crates/paladin-llm/src/openai/adapter.rs
    - crates/paladin-llm/src/anthropic/adapter.rs
    - crates/paladin-llm/src/mock.rs
    - crates/paladin-llm/src/lib.rs
    - crates/paladin-battalion/src/grove_service.rs
    - src/application/services/paladin/paladin_builder.rs
    - src/application/services/paladin/temperature_service.rs
    - src/application/services/paladin/planning_service.rs
    - src/application/services/paladin/prompt_generation_service.rs
    - src/application/services/paladin/paladin_execution_service.rs
    - tests/integration/autonomous_planning_test.rs
    - tests/helpers/mock_llm_adapter.rs

key-decisions:
  - "Split the plan's two tasks into two atomic commits (fff7a80 Task 1 tracer, a2cc1c5 Task 2
    expansion) exactly as the plan structured them, rather than combining."
  - "Placed the two cross-adapter tests (test_capabilities_tool_calling_matches_request_surface,
    test_every_adapter_declares_a_temperature_range) in a new capability_invariants module in
    crates/paladin-llm/src/lib.rs rather than inside any single adapter's own #[cfg(test)] module,
    because no single adapter file's test module can see its siblings under default feature
    resolution — the module is gated on #[cfg(all(test, feature = \"openai\", feature =
    \"anthropic\", feature = \"deepseek\"))], the combination the root paladin-ai package already
    requests, so `cargo test --workspace` exercises both tests without extra flags."
  - "Corrected CONTEXT.md D-15's construction-site enumeration in code rather than in prose: its
    grep-derived list omitted the OpenAI and Anthropic adapters' own get_capabilities literals
    (naming them separately as receiving None in Task 1 and real ranges in Task 2, which is what
    happened) but also undercounted nothing else — the compiler-verified final list matches D-15's
    list plus those two adapters, with vision_llm_port.rs correctly excluded (it already used
    ..Default::default())."

requirements-completed: [GAP-07]

coverage:
  - id: D1
    description: "ProviderCapabilities gains temperature_range: Option<(f32, f32)>, Eq-free derive
      list, Default sets it to None; DeepSeek declares Some((0.0, 2.0)); PaladinBuilder::validate
      checks the provider's range first (inclusive both ends, no epsilon, no clamping), falling
      back to [0.0, 1.0] when a provider declares none"
    requirement: "GAP-07"
    verification:
      - kind: unit
        ref: "cargo test -p paladin-ports test_provider_capabilities_default_temperature_range_is_none"
        status: pass
      - kind: unit
        ref: "cargo test --lib paladin_builder::tests -- test_deepseek_temperature_range_accepts_two_point_zero test_temperature_rejected_above_provider_max"
        status: pass
      - kind: unit
        ref: "cargo test -p paladin-llm --features deepseek test_deepseek_provider_capabilities"
        status: pass
    human_judgment: false
  - id: D2
    description: "supports_tool_calling is false on all three shipped adapters (OpenAI, Anthropic,
      DeepSeek), pinned by a correspondence test tying the flag to whether LlmRequest carries a
      tools field (WEB-03 success criterion 3), and an invariant test asserting every adapter
      declares a temperature range"
    requirement: "GAP-07"
    verification:
      - kind: unit
        ref: "cargo test -p paladin-llm --features openai,anthropic,deepseek test_capabilities_tool_calling_matches_request_surface"
        status: pass
      - kind: unit
        ref: "cargo test -p paladin-llm --features openai,anthropic,deepseek test_every_adapter_declares_a_temperature_range"
        status: pass
      - kind: other
        ref: "grep -c 'supports_tool_calling: true' crates/paladin-llm/src/{openai,anthropic,deepseek}/adapter.rs sums to 0"
        status: pass
    human_judgment: false
  - id: D3
    description: "Full workspace suite stays green after both tasks: cargo test --workspace, cargo
      clippy --workspace --all-targets -- -D warnings, cargo fmt --all -- --check"
    verification:
      - kind: other
        ref: "cargo test --workspace (run after Task 1 and again after Task 2; every `test result:`
          line reported 0 failed both times)"
        status: pass
      - kind: other
        ref: "cargo clippy --workspace --all-targets -- -D warnings"
        status: pass
      - kind: other
        ref: "cargo fmt --all -- --check"
        status: pass
    human_judgment: false

duration: 45min
completed: 2026-08-01
status: complete
---

# Phase 2 Plan 02: Provider-Aware Temperature Validation & Honest Tool-Calling Flag Summary

**`ProviderCapabilities` gains `temperature_range: Option<(f32, f32)>` (DeepSeek 0.0-2.0 now reachable through `PaladinBuilder`) and an honest `supports_tool_calling: false` across all three adapters, both changes landed in the same tracer-then-expand sequence to avoid touching the published struct twice**

## Performance

- **Duration:** ~45 min
- **Started:** 2026-08-01T00:10:33Z (base commit `a637288`)
- **Completed:** 2026-08-01T00:53:32Z (Task 2 commit `a2cc1c5`)
- **Tasks:** 2 (both `type="auto"`, `tdd="true"`)
- **Files modified:** 14 distinct files across both tasks (13 in Task 1, 3 in Task 2, `lib.rs`
  counted once)

## Accomplishments

- Landed the phase's tracer (D-16): `ProviderCapabilities` — the published ports-layer type with
  the widest blast radius in Phase 2 — gained `temperature_range: Option<(f32, f32)>`, dropped
  `Eq` from its derive list (required, since `f32` does not implement `Eq`), and every one of the
  struct's exhaustive construction sites in the workspace was updated to compile against the new
  field.
- A DeepSeek-backed `Paladin` now builds successfully at temperature `1.8` and at the inclusive
  boundary `2.0`, and is rejected at `2.1` with a typed `PaladinError::ConfigurationError` naming
  the rejected value — closing the exact gap ADR-0004 exists to end (DeepSeek's documented
  `0.0-2.0` range was previously unreachable through the normal Paladin path).
- `PaladinBuilder::validate` reads the selected provider's declared range via
  `self.llm_port.get_capabilities().temperature_range`, falling back to today's `[0.0, 1.0]`
  behaviour when a provider declares none — verified for both the DeepSeek case and the
  no-declared-range fallback case in the same test.
- `supports_tool_calling` is now `false` on all three shipped adapters (OpenAI and Anthropic
  flipped from an over-reported `true`; DeepSeek was already `false` and is unchanged), satisfying
  WEB-03 (pulled forward from Phase 14 per D-13) with a correspondence test that expresses the
  tie between the flag and `LlmRequest`'s actual field set rather than hardcoding `false` three
  times.
- `cargo test --workspace` stayed green (0 failed) after both tasks, and `cargo clippy
  --workspace --all-targets -- -D warnings` / `cargo fmt --all -- --check` were clean at both
  commit points.

## Task Commits

Each task was committed atomically:

1. **Task 1: End-to-end provider-aware temperature — one provider, one path** - `fff7a80` (feat)
2. **Task 2: Expand to OpenAI and Anthropic, and make the tool-calling flag honest (WEB-03)** - `a2cc1c5` (feat)

**Plan metadata:** (this SUMMARY's own commit, made immediately after this file)

## Files Created/Modified

- `crates/paladin-ports/src/output/llm_port.rs` - Added `temperature_range: Option<(f32, f32)>` to
  `ProviderCapabilities`, dropped `Eq` from the derive list, set the field to `None` in `Default`,
  updated both doc examples and all four in-file test literals, added
  `test_provider_capabilities_default_temperature_range_is_none`.
- `crates/paladin-llm/src/deepseek/adapter.rs` - Declares `temperature_range: Some((0.0, 2.0))`;
  extended `test_deepseek_provider_capabilities` to assert it.
- `crates/paladin-llm/src/openai/adapter.rs` - Task 1: `temperature_range: None`. Task 2:
  `temperature_range: Some((0.0, 1.0))`, `supports_tool_calling: false` (was `true`); inverted
  `test_get_capabilities`'s assertion and added the temperature-range assertion.
- `crates/paladin-llm/src/anthropic/adapter.rs` - Same shape as OpenAI: Task 1 `None`, Task 2
  `Some((0.0, 1.0))` and `supports_tool_calling: false`; inverted
  `test_anthropic_provider_capabilities`'s assertion and added the range assertion.
- `crates/paladin-llm/src/mock.rs` - Both `get_capabilities` impls (`MockLlmAdapter`,
  `MultiStepMockLlmPort`) take `temperature_range: None`.
- `crates/paladin-llm/src/lib.rs` - New `capability_invariants` test module (gated on all three
  provider features) housing `test_capabilities_tool_calling_matches_request_surface` (WEB-03's
  own success criterion 3) and `test_every_adapter_declares_a_temperature_range` (the
  assumption-delta invariant test named in the plan's `<assumption_delta_decision>`).
- `crates/paladin-battalion/src/grove_service.rs` - The one exhaustive `ProviderCapabilities`
  literal in its test module (line 1108) takes `temperature_range: None`; the other four
  `get_capabilities` fns in this file already delegated to `::default()` and needed no edit.
- `src/application/services/paladin/paladin_builder.rs` - `validate()`'s temperature check now
  reads the provider's declared range (inclusive both ends, no epsilon, no clamping) with a
  `[0.0, 1.0]` fallback; doc comments updated to describe the provider-aware contract; added
  `test_deepseek_temperature_range_accepts_two_point_zero` and
  `test_temperature_rejected_above_provider_max` plus a `deepseek_llm_port()` test helper building
  a real `DeepSeekAdapter`.
- `src/application/services/paladin/temperature_service.rs`,
  `src/application/services/paladin/planning_service.rs`,
  `src/application/services/paladin/prompt_generation_service.rs`,
  `src/application/services/paladin/paladin_execution_service.rs` (3 sites) - Test-fixture
  `ProviderCapabilities` literals take `temperature_range: None`.
- `tests/integration/autonomous_planning_test.rs`,
  `tests/helpers/mock_llm_adapter.rs` - Same mechanical addition.

## Construction-site list (for plan 02-09)

Verified against the compiler after both tasks landed. Every exhaustive `ProviderCapabilities {`
literal in the workspace, cross-referenced against `grep -rn "ProviderCapabilities\s*{"
--include="*.rs" .` with `::default()`-only sites excluded:

- `crates/paladin-ports/src/output/llm_port.rs` — struct definition, `Default` impl, 2 doc
  examples, 4 in-file `#[cfg(test)]` literals (8 sites total, matching D-15's enumeration).
- `crates/paladin-llm/src/deepseek/adapter.rs`, `openai/adapter.rs`, `anthropic/adapter.rs` — the
  three shipped adapters (1 site each). **CONTEXT.md D-15's own list, compiled by grep during
  discussion, omitted these two adapters (OpenAI, Anthropic) from its "every other construction
  site takes `None`" enumeration** — the plan's own `<action>` text for Task 1 already named this
  as "a correction to CONTEXT.md D-15's list ... plus `vision_llm_port.rs` and the `examples/
  herald_*.rs` files, which already use `Default` and need no edit," and this run confirms both
  halves of that correction: the two adapters needed edits (`None` in Task 1, real ranges in Task
  2), and `vision_llm_port.rs` (line 201, `..Default::default()` spread) plus the two
  `examples/herald_*.rs` files needed none.
- `crates/paladin-llm/src/mock.rs` — 2 sites (`MockLlmAdapter`, `MultiStepMockLlmPort`).
- `crates/paladin-battalion/src/grove_service.rs` — 1 site (line 1108); the other four
  `get_capabilities` fns in the same file (lines 1195, 1298, 1399, 1498) delegate to `::default()`
  and needed no edit.
- `src/application/services/paladin/temperature_service.rs`, `planning_service.rs`,
  `prompt_generation_service.rs` — 1 site each.
- `src/application/services/paladin/paladin_execution_service.rs` — 3 sites (all three vision-test
  fixtures share the identical literal).
- `tests/integration/autonomous_planning_test.rs`, `tests/helpers/mock_llm_adapter.rs` — 1 site
  each.

**Confirmed excluded (already `::default()` or `..Default::default()`, no edit needed):**
`examples/herald_json_output.rs`, `examples/herald_markdown_output.rs`,
`examples/herald_custom_formatter.rs`, `crates/paladin-ports/src/output/vision_llm_port.rs`,
`crates/paladin-battalion/src/grove_service.rs` (4 of its 5 `get_capabilities` fns), and every
`tests/{integration,unit,functional}/*.rs` file not listed above (17 additional files verified —
`paladin_garrison_integration_test.rs`, `citadel_integration_test.rs`,
`arsenal_bridge_regression_test.rs`, `herald_integration_test.rs`, `context_injection_test.rs`,
`paladin_builder_arsenal_test.rs`, `paladin_execution_service_test.rs`, `paladin_builder_test.rs`,
`content_llm_analysis_pipeline_test.rs`, `paladin_tool_invocation_test.rs`,
`battalion/grove_integration_test.rs`, `arsenal/handoff_tool_test.rs`, `paladin_builder.rs`'s own
mock in its test module (line 1842), `content_llm_analysis_service.rs`,
`memory_extraction_service.rs`, `paladin_execution_service.rs`'s first `get_capabilities` at line
1994).

## Decisions Made

- Followed D-16 exactly: the ports change landed first as a tracer (Task 1, commit `fff7a80`),
  proven end-to-end through one real provider (DeepSeek) and the builder before Task 2 (commit
  `a2cc1c5`) expanded to the remaining two adapters and the tool-calling flag.
- Placed the two cross-adapter Task 2 tests in a new `capability_invariants` module in
  `crates/paladin-llm/src/lib.rs` rather than in any single adapter's own test module, because no
  adapter's `#[cfg(test)] mod tests { use super::*; }` can see its siblings under default feature
  resolution (each adapter module is itself feature-gated). The new module is gated on
  `#[cfg(all(test, feature = "openai", feature = "anthropic", feature = "deepseek"))]` — the exact
  combination the root `paladin-ai` package's dependency declaration already requests, so `cargo
  test --workspace` exercises both tests with no extra flags, and `cargo test -p paladin-llm`
  alone (default features `["openai", "mock"]`) correctly skips them rather than failing to
  compile.
- Expressed the tool-calling correspondence explicitly (a named
  `REQUEST_SURFACE_SUPPORTS_TOOL_CALLING` constant compared against each adapter's declared flag)
  rather than three hardcoded `assert!(!caps.supports_tool_calling)` calls, per the plan's
  instruction that a future phase adding a tool-carrying field to `LlmRequest` should make this
  test name what changed rather than silently keep passing.

## Deviations from Plan

None - plan executed exactly as written. Both tasks' `<action>` sections, `<acceptance_criteria>`
greps, and `<verify>` commands passed on the first attempt after implementation; no auto-fixes
under Rules 1-3 were needed and no architectural questions arose under Rule 4.

## Issues Encountered

- The Bash tool's worktree-isolation guard rejected a multi-command shell invocation (a `for` loop
  reading 20 files via `sed`) as "too complex to verify." Resolved by using the `Read` tool with
  explicit offsets for each file instead — no functional impact, more tool calls.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- **Plan 02-09** can now amend `.planning/ledgers/milestone-01.md` and correct
  `.planning/CONTEXT.md`'s D-15 enumeration using the compiler-verified construction-site list
  above (D-15's grep-derived list omitted the OpenAI and Anthropic adapters).
- **Phase 14's WEB-03** is satisfied by this plan per D-13/D-14 — Phase 14 should record it as
  such rather than re-plan it. WEB-04 (whether LLM tool calling gets built at all) remains
  entirely open and untouched by this plan.
- `ProviderCapabilities`'s published surface changed additively (new field, `Eq` dropped from the
  derive list); every downstream construction site in this workspace already compiles against it.
  Any out-of-tree consumer constructing `ProviderCapabilities` with an exhaustive literal (rather
  than `..Default::default()`) will need the same one-line addition — this is the accepted,
  recorded cost of T-02-08 in the plan's threat model (disposition: accept, per D-15).
- No blockers for the remaining waves of Phase 2: this plan touched only the files listed above,
  and the workspace test suite, clippy, and fmt all stayed green at both commit points.

---
*Phase: 02-functional-gap-closure*
*Completed: 2026-08-01*
