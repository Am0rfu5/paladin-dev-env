---
phase: 06-verified-gap-closure
plan: 03
subsystem: cli
tags: [clap, serde-yaml, paladin-builder, autonomous-agents, cli-config]

# Dependency graph
requires:
  - phase: 06-verified-gap-closure
    provides: "Phase 5's verified verdict that Epic 14 cluster 8.0 (YAML & CLI Configuration Support) is the only genuinely outstanding block behind CLOSE-02's autonomous-flags scope"
provides:
  - "PaladinYamlConfig.autonomous: Option<AutonomousConfig>, reusing paladin-core's AutonomousConfig directly (D-06), with schema-bound validation wired into PaladinYamlConfig::validate"
  - "apply_autonomous_config: a shared YAML-baseline-then-flag-override composition in handle_agent_run, covering all four autonomous features (planning, prompt generation, dynamic temperature, handoffs)"
  - "effective_handoffs: the pure merge function computing the HandoffConfig value handed to PaladinBuilder's handoff setter, preserving YAML-set fields when a flag forces enabled on"
  - "Corrected #[arg] doc comments on the four autonomous flags stating force-on-only semantics (D-07)"
  - "A commented-out autonomous: example in the generated agent template"
affects: [cli, autonomous-agents]

# Tech tracking
tech-stack:
  added: []
  patterns:
    - "YAML-baseline-then-flag-override composition function, shared between production code and tests, so tests exercise the exact code path the CLI uses"
    - "Additive-only CLI flag override: a present flag forces a feature on, never off, with no --no-* counterparts"

key-files:
  created: []
  modified:
    - src/application/cli/config/paladin_config.rs
    - src/application/cli/commands/agent.rs
    - src/application/cli/templates/paladin_template.rs

key-decisions:
  - "Reused paladin-core's AutonomousConfig directly on PaladinYamlConfig rather than a CLI-local mirror type, per D-06"
  - "Extracted apply_autonomous_config and effective_handoffs as private module-level functions so tests drive the identical composition handle_agent_run uses, rather than duplicating builder calls in test code"
  - "Tested the handoff merge logic via effective_handoffs directly rather than via PaladinData, because PaladinData carries no handoff field and PaladinBuilder exposes no getter for its private handoff_config field (D-08 forbids touching paladin_builder.rs to add one)"

patterns-established:
  - "Additive-only CLI flag override on top of a YAML baseline (D-05/D-07): apply the config file first, then force-on flags second, order stated explicitly at the call site even though the result is order-independent for a single feature"

requirements-completed: [CLOSE-02]

coverage:
  - id: D1
    description: "PaladinYamlConfig gains an autonomous: Option<AutonomousConfig> section that deserializes, round-trips (omitted when None), and is bounds-validated via AutonomousConfig::validate"
    requirement: "CLOSE-02"
    verification:
      - kind: unit
        ref: "src/application/cli/config/paladin_config.rs#test_load_paladin_config_with_autonomous_section"
        status: pass
      - kind: unit
        ref: "src/application/cli/config/paladin_config.rs#test_load_paladin_config_without_autonomous_section"
        status: pass
      - kind: unit
        ref: "src/application/cli/config/paladin_config.rs#test_paladin_yaml_config_omits_autonomous_when_none"
        status: pass
      - kind: unit
        ref: "src/application/cli/config/paladin_config.rs#test_validate_rejects_out_of_bounds_autonomous_config"
        status: pass
      - kind: unit
        ref: "src/application/cli/config/paladin_config.rs#test_validate_accepts_valid_autonomous_config"
        status: pass
    human_judgment: false
  - id: D2
    description: "The autonomous planning feature reaches PaladinData.autonomous_planning from both the YAML autonomous.planning.enabled baseline and the --auto-plan flag"
    requirement: "CLOSE-02"
    verification:
      - kind: unit
        ref: "src/application/cli/commands/agent.rs#test_autonomous_planning_from_yaml_reaches_paladin_data"
        status: pass
      - kind: unit
        ref: "src/application/cli/commands/agent.rs#test_auto_plan_flag_forces_planning_on"
        status: pass
    human_judgment: false
  - id: D3
    description: "Prompt generation, dynamic temperature, and handoffs are wired the same way as planning, with the full YAML-value x flag-presence matrix (6 combinations per feature) proven, idempotency and cross-feature independence proven, the no-op baseline proven, and proof that a YAML-enabled feature cannot be disabled from the CLI"
    requirement: "CLOSE-02"
    verification:
      - kind: unit
        ref: "src/application/cli/commands/agent.rs#test_autonomous_prompts_yaml_and_flag"
        status: pass
      - kind: unit
        ref: "src/application/cli/commands/agent.rs#test_dynamic_temperature_yaml_and_flag"
        status: pass
      - kind: unit
        ref: "src/application/cli/commands/agent.rs#test_handoffs_yaml_and_flag"
        status: pass
      - kind: unit
        ref: "src/application/cli/commands/agent.rs#test_autonomous_flag_application_is_idempotent_and_independent"
        status: pass
      - kind: unit
        ref: "src/application/cli/commands/agent.rs#test_no_autonomous_section_and_no_flags_is_a_no_op"
        status: pass
      - kind: unit
        ref: "src/application/cli/commands/agent.rs#test_yaml_enabled_feature_cannot_be_disabled_from_cli"
        status: pass
    human_judgment: false
  - id: D4
    description: "The generated agent template (paladin agent new) documents the autonomous: section as a commented-out example that still parses as valid YAML"
    requirement: "CLOSE-02"
    verification:
      - kind: unit
        ref: "src/application/cli/templates/paladin_template.rs#test_generate_template_documents_autonomous_section"
        status: pass
      - kind: unit
        ref: "src/application/cli/templates/paladin_template.rs#test_generate_template_autonomous_section_is_commented_out"
        status: pass
      - kind: unit
        ref: "src/application/cli/templates/paladin_template.rs#test_template_is_valid_yaml"
        status: pass
    human_judgment: false

duration: ~55min
completed: 2026-08-05
status: complete
---

# Phase 6 Plan 03: Autonomous CLI Flags and YAML Configuration Summary

**Closed Epic 14 cluster 8.0 by wiring `PaladinYamlConfig.autonomous` and all four `--auto-plan`/`--auto-prompt`/`--dynamic-temp`/`--enable-handoffs` flags as additive overrides in `handle_agent_run`, plus a validated schema and a documented template example.**

## Performance

- **Duration:** ~55 min
- **Completed:** 2026-08-05
- **Tasks:** 3 planned tasks (4 commits — a 4th commit closes a Rule 2 validation gap found during Task 1)
- **Files modified:** 3

## Accomplishments

- `PaladinYamlConfig.autonomous: Option<AutonomousConfig>` deserializes, round-trips, and is bounds-validated, reusing `paladin-core`'s `AutonomousConfig` directly with no CLI-local mirror type (D-06)
- `handle_agent_run` applies the YAML `autonomous` baseline to the `PaladinBuilder`, then layers all four CLI flags on top as additive-only overrides, through one shared `apply_autonomous_config` function that both production code and tests drive identically
- The full YAML-value x flag-presence adjacency matrix (6 combinations) is tested for planning, prompt generation, and dynamic temperature; the handoff merge (a non-boolean feature) is tested via its own pure function, `effective_handoffs`
- Proved the deliberate D-07 consequence: a YAML-enabled autonomous feature cannot be turned off by any combination of the four CLI flags (16-case test)
- Corrected all four flags' `#[arg]` doc comments to state force-on-only semantics, including the `--enable-handoffs` note that specialist agents are supplied via the library's `PaladinBuilder::with_handoffs`, not this CLI
- `paladin agent new` now documents a commented-out `autonomous:` example with real field names and real `paladin-core` defaults

## Task Commits

Each task was committed atomically:

1. **Task 1: End-to-end "autonomous planning configured in YAML reaches the built Paladin"** - `94d093a` (feat)
2. **Task 2: Expand to the remaining three features and prove the full override matrix** - `2d359ef` (feat)
3. **Task 3: Emit a commented-out autonomous section in the generated agent template** - `3b4857d` (docs)

**Deviation fix:** `2d20ebc` (fix) — wires `AutonomousConfig::validate` into `PaladinYamlConfig::validate` (Rule 2, see Deviations below).

_Note: this plan's tasks did not use the RED/GREEN/REFACTOR TDD gate as separate commits — each task's tests and implementation landed together in one commit per task, per the plan's own task-level granularity._

## Files Created/Modified

- `src/application/cli/config/paladin_config.rs` - Added `PaladinYamlConfig.autonomous`, schema doc example, and bounds validation wiring
- `src/application/cli/commands/agent.rs` - `apply_autonomous_config`, `effective_handoffs`, `autonomous_feature_summary`, corrected flag doc comments, 13 new tests
- `src/application/cli/templates/paladin_template.rs` - Commented-out `autonomous:` example in the generated template, 2 new tests

## Decisions Made

- **Extracted `apply_autonomous_config` as a shared, testable composition function** rather than inlining the baseline/override blocks only inside `handle_agent_run`. This is what let every test drive the exact code path the CLI uses (a `must_haves.truths` requirement), and kept the `enable_autonomous_planning`/`enable_autonomous_prompts`/`enable_dynamic_temperature`/`handoff_config` call-site counts at exactly 2 each (baseline + override), matching the plan's acceptance criteria literally.
- **Tested the handoff merge via `effective_handoffs` directly, not via `PaladinData`.** `PaladinData` (the domain type `PaladinBuilder::build()` produces) carries no handoff field at all — the builder's private `handoff_config` field is written but never read anywhere in `PaladinBuilder::build()`, and there is no getter. D-08 forbids touching `paladin_builder.rs` to add one. `effective_handoffs` is the pure function that computes the exact `HandoffConfig` value `apply_autonomous_config` hands to the builder's handoff setter in both the baseline and override blocks, so testing it directly is a faithful, complete proof of the D-05/D-07 handoff semantics without needing builder introspection.
- **`test_load_paladin_config_with_autonomous_section` uses hand-written YAML, not a serialize-then-deserialize round trip.** A round trip hit a known `serde_yaml` 0.9 (deprecated) limitation deserializing `MaxLoops`'s untagged-enum representation back out of a document that also carries other nested enums (here, `HandoffStrategy`) — unrelated to the `autonomous` wiring under test, and pre-existing in the dependency, not something this plan introduced or should attempt to fix.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 2 - Missing Critical] Wired `AutonomousConfig::validate` into `PaladinYamlConfig::validate`**
- **Found during:** Task 1, while re-reading the plan's threat register before implementation
- **Issue:** The plan's threat register disposes T-06-03-01 (Elevation of Privilege) and T-06-03-02 (Denial of Service) as `mitigate`, both naming `AutonomousConfig::validate` — which bounds `max_subtasks` (<=100), the temperature range, and handoff `max_depth` (<=20) — as the control. `PaladinYamlConfig::validate` (the CLI-level `Validate` trait impl that `load_paladin_config` actually calls) never invoked it. An out-of-range `autonomous` section (e.g. `max_subtasks: 99999`) would deserialize and pass CLI validation unchecked; the stated mitigation was not actually reachable from the CLI's real load path.
- **Fix:** Added a call to `self.autonomous.validate()` inside `PaladinYamlConfig::validate`, mapping its `Result<(), String>` to `CliError::InvalidFieldValue { field: "autonomous", .. }`, matching the file's existing validation-error idiom.
- **Files modified:** `src/application/cli/config/paladin_config.rs`
- **Verification:** `test_validate_rejects_out_of_bounds_autonomous_config` (a `max_subtasks: 150` config is rejected) and `test_validate_accepts_valid_autonomous_config` (a bounded config passes) both pass; `cargo test --workspace` green.
- **Committed in:** `2d20ebc`

---

**Total deviations:** 1 auto-fixed (Rule 2 — missing critical input validation)
**Impact on plan:** Closes a real gap between the plan's own threat-model claim and the implementation as first written. No scope creep — the fix is confined to the one file the threat register named, and reuses a domain-level `validate()` method that already existed and was already tested (D-08 scope: no domain-level work redone).

## Open Observation (not a deviation — recorded per plan instruction)

Task 1's action text asked this to be recorded rather than fixed: the `--auto-plan` flag's doc comment previously referenced `MaxLoops::Auto`. Read `PaladinBuilder::build()` and `MaxLoops` directly — `enable_autonomous_planning(true)` sets only `PaladinData.autonomous_planning`; it does **not** set `PaladinData.max_loops` to `MaxLoops::Auto`. `max_loops` is set independently, only via `PaladinBuilder::max_loops(u32)`, which always constructs `MaxLoops::Fixed(n)`. So `MaxLoops::Auto` is **not** derived from `--auto-plan` or from `autonomous.planning.enabled` anywhere in the current codebase — a user relying on the flag's old doc text to get `MaxLoops::Auto` behavior would not get it. The corrected doc comment (this plan) now states the flag's actual effect and explicitly notes it does not set `MaxLoops::Auto`, rather than repeating the stale claim. No behavior was changed and no `MaxLoops` code was touched, per D-05/D-07/D-08 scope. This is worth a future look (open item, no phase currently owns it).

## Issues Encountered

- `serde_yaml`'s known limitation round-tripping `MaxLoops` through a document also carrying `HandoffConfig`'s nested `HandoffStrategy` enum (see Decisions above) — worked around with a hand-written YAML fixture for the one affected test; no library or production code change needed.
- Threading `&args` through to `apply_autonomous_config` after `args.input` had already been partially moved out earlier in `handle_agent_run` required changing that extraction from `if let Some(input_text) = args.input { .. }` to `.clone()` on the `Option<String>` — a minimal, targeted fix (Rule 3, blocking) to keep `args` usable by reference at the call site the plan specifies.

## Known Stubs

None. Every wired feature (planning, prompt generation, dynamic temperature, handoffs) reaches a real `PaladinBuilder` setter or, for handoffs, a real value handed to the builder's handoff setter — no hardcoded empty values, no placeholder text, no unwired data paths.

## Threat Flags

None beyond what the plan's own threat register already names (T-06-03-01 through T-06-03-04, T-06-03-SC). The Rule 2 fix above closes the one gap between the register's stated `mitigate` disposition and the code as first written; no new surface was introduced.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- CLOSE-02's Epic 14 cluster `8.0` is closed: the `autonomous` YAML section exists, deserializes, validates, and the four declared flags are read and applied additively on top of it per D-05/D-06/D-07/D-08.
- `cargo test --workspace`, `cargo fmt --check`, and `cargo clippy --workspace --all-targets -- -D warnings` are all green at HEAD (`2d20ebc`).
- No blockers for sibling plans in this wave; this plan touched only `src/application/cli/` (declared scope), leaving `crates/paladin-battalion/` and `crates/paladin-core/` untouched for the parallel agents working those areas.

---
*Phase: 06-verified-gap-closure*
*Completed: 2026-08-05*
