---
phase: 02-functional-gap-closure
plan: 04
subsystem: battalion-herald
tags: [formation, phalanx-port, battalion-result, json-herald, markdown-herald, table-herald, gap-03]

# Dependency graph
requires:
  - phase: 02-functional-gap-closure
    provides: "02-01's measured cargo test --workspace baseline (2790 passed / 0 failed / 126
      ignored on commit 7e55655) — the pre-change tree this plan's own full-suite runs are compared
      against; 02-02's ProviderCapabilities tracer, already merged into this plan's base commit"
provides:
  - "Formation's `BattalionResult` now carries real per-Paladin execution times, per-Paladin token
    usage (keyed by Paladin name) and a non-zero aggregate `total_tokens` — ported from
    `PhalanxExecutionService::execute_internal`, the phase's central analog"
  - "A Formation run under `ContinueOnError`/`RetryThenContinue` records each failed Paladin as a
    structured `NodeError { node_name, error }`, and `paladin_success_count`/`paladin_failure_count`
    now derive from `paladin_results.len()`/`node_errors.len()` (matching Phalanx) rather than a
    stop_reason filter that could never observe a Paladin that failed outright"
  - "All three Heralds (JSON, Markdown, Table) render the Battalion's name, id, strategy, ordered
    per-Paladin results, aggregate token total and failure detail — the JSON and Markdown Heralds
    additively extended, the Table Herald's stub replaced with a renderer that reads its argument"
  - "The Table Herald's own test is now input-dependent: rendering two different `BattalionResult`
    values produces two different strings, closing the exact gap RESEARCH.md Pitfall 5 identified
    (a Herald test whose assertions would pass against a formatter that ignores its input)"
  - "Direct-code-reading finding for plan 02-09: the ledger's `satisfied` verdict for
    `REQ-herald-battalion-result-fields` (`milestone-01.md:401`) is contradicted — the cited tests
    (`test_format_battalion_result_success`/`_includes_metadata`) never exercised the Table Herald,
    and the Table Herald's own pre-existing test asserted only its own hardcoded header strings
    against an empty `paladin_results` input"
affects: [02-05-battalion-herald-e2e, 02-09-amend-ledger]

# Tech tracking
tech-stack:
  added: []
  patterns:
    - "Per-Paladin aggregation built inline in Formation's existing sequential loop (simpler than
      Phalanx's after-the-fact successful_names reconciliation, since Formation's loop already
      conditionally continues vs. fails per ErrorStrategy) — captured before each successful
      PaladinResult is moved into paladin_results.push(result)"
    - "BattalionResult constructed as a struct literal (mirroring Phalanx) rather than through the
      plain BattalionResult::new(..) constructor, whenever a call site needs to populate the
      aggregation fields the constructor defaults to empty/zero"
    - "Table Herald name resolution: since PaladinResult carries no name field, a Paladin's real
      name is recovered by consuming an entry from a (name, time, tokens) pool built from the
      name-keyed per_paladin_times/per_paladin_tokens maps, matched on the exact
      execution_time_ms/token_count pair each map entry copies from its PaladinResult — falling
      back to a positional 'Paladin N' label only when no match is found"

key-files:
  created: []
  modified:
    - crates/paladin-battalion/src/formation_service.rs
    - crates/paladin-core/src/platform/container/battalion/mod.rs
    - crates/paladin-herald/src/json_herald.rs
    - crates/paladin-herald/src/markdown_herald.rs
    - crates/paladin-herald/src/table_herald.rs

key-decisions:
  - "paladin_success_count/paladin_failure_count on Formation's result now derive from
    paladin_results.len()/node_errors.len() (matching Phalanx exactly), not the prior
    stop_reason-based filter inherited from BattalionResult::new — the old derivation could only
    ever see 0 failures under ContinueOnError, since a Paladin that fails outright never produces a
    PaladinResult entry to filter over. No existing test locked in the old behavior (verified by
    grep across tests/ and commander.rs); commander_integration_tests.rs's all-success Formation
    case (3/0) is unaffected by the change."
  - "Table Herald name resolution matches by the (execution_time_ms, token_count) pair rather than
    by map iteration order, because HashMap has no defined iteration order and PaladinResult itself
    carries no name field — matching on the exact values each aggregation map entry copies from its
    source PaladinResult is the only correlating data available, with a documented positional
    fallback for the case a match cannot be found."
  - "Left TableHerald::format_paladin_result untouched — its placeholder rows are a separate,
    out-of-scope stub; this plan's files/tasks name only format_battalion_result."

requirements-completed: [GAP-03]

coverage:
  - id: D1
    description: "Formation populates per_paladin_times, per_paladin_tokens and total_tokens on the
      BattalionResult it builds, ported from PhalanxExecutionService's aggregation loop"
    requirement: "GAP-03"
    verification:
      - kind: unit
        ref: "cargo test -p paladin-battalion formation_service::tests::test_formation_aggregates_per_paladin_times_and_tokens"
        status: pass
      - kind: unit
        ref: "cargo test -p paladin-battalion phalanx (analog untouched, still green)"
        status: pass
    human_judgment: false
  - id: D2
    description: "Formation records a structured NodeError per Paladin that fails under
      ContinueOnError/RetryThenContinue, naming the failing Paladin and its error text"
    requirement: "GAP-03"
    verification:
      - kind: unit
        ref: "cargo test -p paladin-battalion formation_service::tests::test_formation_records_node_errors_on_continue_on_error"
        status: pass
    human_judgment: false
  - id: D3
    description: "JSON and Markdown Heralds render strategy_used, total_tokens, per_paladin_tokens
      and node_errors alongside what they rendered before, proven with distinct non-round token
      counts so the assertions cannot pass against output that ignores the input"
    requirement: "GAP-03"
    verification:
      - kind: unit
        ref: "cargo test -p paladin-herald json_herald::tests::test_json_herald_battalion_includes_strategy_and_total_tokens"
        status: pass
      - kind: unit
        ref: "cargo test -p paladin-herald markdown_herald::tests::test_markdown_herald_battalion_includes_strategy_and_total_tokens"
        status: pass
      - kind: unit
        ref: "cargo test -p paladin-herald json_herald::tests::test_json_herald_battalion_empty_paladin_results_still_valid"
        status: pass
    human_judgment: false
  - id: D4
    description: "Table Herald reads its result argument: real Paladin names/counts/order, the
      Battalion's identity and strategy, aggregate tokens, and failure detail — with the litmus
      test that two different BattalionResult values render as two different strings"
    requirement: "GAP-03"
    verification:
      - kind: unit
        ref: "cargo test -p paladin-herald table_herald::tests::test_table_herald_renders_actual_paladin_names"
        status: pass
      - kind: unit
        ref: "cargo test -p paladin-herald table_herald::tests::test_table_herald_renders_empty_paladin_results"
        status: pass
      - kind: unit
        ref: "cargo test -p paladin-herald table_herald::tests::test_table_herald_renders_multibyte_paladin_name"
        status: pass
      - kind: unit
        ref: "cargo test -p paladin-herald table_herald::tests::test_table_herald_surfaces_node_error_names"
        status: pass
    human_judgment: false
  - id: D5
    description: "Full workspace suite, clippy and fmt stay green after all three tasks"
    verification:
      - kind: other
        ref: "cargo test --workspace (run after each of the three task commits; every test result:
          line reported 0 failed all three times)"
        status: pass
      - kind: other
        ref: "cargo clippy --workspace --all-targets --all-features -- -D warnings"
        status: pass
      - kind: other
        ref: "cargo fmt --all -- --check"
        status: pass
    human_judgment: false

duration: ~35min
completed: 2026-08-01
status: complete
---

# Phase 2 Plan 04: Battalion Result Aggregation & Herald Rendering Summary

**Formation now populates per-Paladin times/tokens/total_tokens and structured node_errors on its `BattalionResult` (ported from Phalanx), and all three Heralds (JSON, Markdown, Table) render the Battalion's strategy, aggregate tokens and failure detail — closing GAP-03, the phase's largest real code-change item and the one requirement whose `satisfied` ledger verdict does not survive direct code reading**

## Performance

- **Duration:** ~35 min
- **Started:** 2026-08-01T01:15:00Z (approx, base commit `91b3033`)
- **Completed:** 2026-08-01T01:34:04Z (Task 3 commit `eea94bf`)
- **Tasks:** 3 (all `type="auto"`, `tdd="true"`)
- **Files modified:** 5 distinct files across three tasks

## Accomplishments

- Ported Phalanx's per-Paladin aggregation pattern into Formation's `execute_internal`: the
  sequential loop now builds `per_paladin_times`, `per_paladin_tokens` and `total_tokens` inline as
  results are produced, and records a `NodeError { node_name, error }` for each Paladin that fails
  under `ContinueOnError`/`RetryThenContinue`. The closing `BattalionResult::new(..)` call is
  replaced with a struct literal that carries the aggregation fields and sets `strategy_used`
  explicitly to `Formation`.
- Corrected `paladin_success_count`/`paladin_failure_count` derivation to match Phalanx
  (`paladin_results.len()`/`node_errors.len()`) rather than the prior stop_reason-based filter,
  which structurally could never see a failure under `ContinueOnError` (a Paladin that fails
  outright never produces a `PaladinResult` to filter over) — verified against the one existing
  integration assertion on Formation's success/failure counts (`commander_integration_tests.rs:608`,
  an all-success 3/0 case, unaffected by the change).
- Extended the JSON Herald's `battalion_result_to_json` and the Markdown Herald's
  `format_battalion_result` additively with `strategy_used`, `total_tokens`, `per_paladin_tokens`
  and `node_errors` — proven with a `BattalionResult` carrying two Paladins with distinct, non-round
  token counts and one `NodeError`, so the new tests cannot pass against output that ignores the
  input.
- Replaced the Table Herald's `format_battalion_result` stub — which rendered two hardcoded
  placeholder rows ("paladin_1"/"paladin_2", fixed durations and token counts) regardless of what
  Battalion actually ran — with a renderer that reads `result.paladin_results` in order, resolves
  each row's real Paladin name from the name-keyed aggregation maps, and renders the Battalion's
  identity, strategy, aggregate tokens and failure detail. Rewrote the Table Herald's own test
  (which previously asserted only the formatter's hardcoded header strings against an empty
  `paladin_results` input — RESEARCH.md's identified Pitfall 5) into four content-asserting tests,
  including the litmus test that two different `BattalionResult` values render as two different
  strings.
- `cargo test --workspace` stayed green (0 failed) after all three task commits, and `cargo clippy
  --workspace --all-targets --all-features -- -D warnings` / `cargo fmt --all -- --check` were clean
  at every commit point.

## Task Commits

Each task was committed atomically:

1. **Task 1: Populate Formation's per-Paladin aggregation, porting the shipped Phalanx pattern** - `7f22d49` (feat)
2. **Task 2: Render strategy, aggregate tokens and failure detail in the JSON and Markdown Heralds** - `fbbf33e` (feat)
3. **Task 3: Replace the Table Herald stub with a real renderer and rewrite its self-confirming test** - `eea94bf` (feat)

**Plan metadata:** (this SUMMARY's own commit, made immediately after this file)

## Files Created/Modified

- `crates/paladin-battalion/src/formation_service.rs` - `execute_internal` now builds
  `per_paladin_times`/`per_paladin_tokens`/`total_tokens` inline in its execution loop, records
  `NodeError` entries for continue-past-failure strategies, and constructs the closing
  `BattalionResult` as a struct literal with `strategy_used: BattalionStrategy::Formation` set
  explicitly. Added `test_formation_aggregates_per_paladin_times_and_tokens` and
  `test_formation_records_node_errors_on_continue_on_error`.
- `crates/paladin-core/src/platform/container/battalion/mod.rs` - Updated the `NodeError`/
  `node_errors` doc comments: the field is no longer described as Phalanx-only, it now documents
  carrying per-node failure detail for any strategy that continues past a failure (Formation and
  Phalanx both).
- `crates/paladin-herald/src/json_herald.rs` - `battalion_result_to_json`'s object literal gains
  `strategy_used`, `total_tokens`, `per_paladin_tokens` and `node_errors` (additive; existing keys
  and the `include_metadata` branch unchanged). Added
  `test_json_herald_battalion_includes_strategy_and_total_tokens` and
  `test_json_herald_battalion_empty_paladin_results_still_valid`.
- `crates/paladin-herald/src/markdown_herald.rs` - `format_battalion_result` renders Strategy and
  Total Tokens alongside the existing summary fields (same `format_field` helper), and a Failures
  section naming each node error's name/text when non-empty. Added
  `test_markdown_herald_battalion_includes_strategy_and_total_tokens` and
  `test_markdown_herald_battalion_no_failures_section_when_no_node_errors`; extended (not replaced)
  `test_format_battalion_result_structure` with assertions for the new fields.
- `crates/paladin-herald/src/table_herald.rs` - `format_battalion_result` rewritten to read its
  `result` argument: a header block (name/id/strategy/total tokens), one table row per
  `paladin_results` entry with the real Paladin name resolved from the aggregation maps, and a
  Failures section. Deleted the two hardcoded placeholder rows. Replaced the single
  self-confirming `test_format_battalion_result` with
  `test_table_herald_renders_actual_paladin_names` (includes the differing-output litmus test),
  `test_table_herald_renders_empty_paladin_results`, `test_table_herald_renders_multibyte_paladin_name`
  and `test_table_herald_surfaces_node_error_names`.

## Ledger Finding (for plan 02-09)

`.planning/ledgers/milestone-01.md:401` records `REQ-herald-battalion-result-fields` as
`satisfied`, citing `test_format_battalion_result_success`/`_includes_metadata` in
`json_herald.rs:354,369`. This plan's execution confirms RESEARCH.md's finding: those two tests
never exercised the Table Herald at all, and the Table Herald's own pre-existing test
(`table_herald.rs:308-343`, now replaced) asserted only its own hardcoded header strings against an
empty `paladin_results` input — a test that would have passed against the stub implementation
regardless of what it rendered. **The `satisfied` verdict was contradicted by direct code
reading.** Plan 02-09 amends this ledger row in place per D-02, citing this plan's four new
content-asserting Table Herald tests and the two extended JSON/Markdown tests as the corrected
evidence.

## Decisions Made

- Ported Phalanx's aggregation pattern exactly rather than inventing a second implementation
  (per the plan's own instruction and `02-PATTERNS.md`'s port guidance) — built inline in
  Formation's existing loop rather than Phalanx's after-the-fact `successful_names` reconciliation,
  since Formation's loop already conditionally continues vs. fails per `ErrorStrategy`.
- Changed `paladin_success_count`/`paladin_failure_count` derivation from the prior
  stop_reason-based filter (inherited from `BattalionResult::new`, which structurally could not
  observe a Paladin that failed outright under `ContinueOnError`) to `paladin_results.len()`/
  `node_errors.len()`, matching Phalanx exactly — required by the plan's own `<behavior>` spec
  (`paladin_failure_count` is 1 for one failing Paladin), verified against the workspace's one
  existing integration assertion on Formation's counts (all-success case, unaffected).
- Table Herald name resolution matches Paladin rows to real names via a consumable
  `(name, time, tokens)` pool built from `per_paladin_times`/`per_paladin_tokens`, since
  `PaladinResult` carries no name field and `HashMap` iteration order is undefined — the exact
  `execution_time_ms`/`token_count` pair each map entry copies from its source `PaladinResult` is
  the only correlating data available. Falls back to a positional `"Paladin N"` label only when no
  match is found (e.g., a `BattalionResult` built by hand without populated aggregation maps).
- Left `TableHerald::format_paladin_result` untouched — its own placeholder rows are a separate,
  out-of-scope stub; this plan's `<files>` and task scope name only `format_battalion_result`.

## Deviations from Plan

None - plan executed exactly as written. All three tasks' `<action>` sections, `<acceptance_criteria>`
greps, and `<verify>` commands passed on the first attempt after implementation; no auto-fixes under
Rules 1-3 were needed and no architectural questions arose under Rule 4.

## Issues Encountered

None.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- **Plan 02-05** (the end-to-end proof tying Formation's aggregation to all three Heralds) can now
  build against a Formation result that actually carries per-Paladin times/tokens/node_errors, and
  against Heralds that actually read them — this plan closes both the producer half and the
  renderer half of GAP-03.
- **Plan 02-09** can amend `.planning/ledgers/milestone-01.md:401`'s `REQ-herald-battalion-result-fields`
  row using the "Ledger Finding" section above as source evidence.
- No blockers for sibling wave-2 plans: this plan touched only the five files declared in its
  `files_modified` frontmatter, and the workspace test suite, clippy and fmt were all green at
  every commit point.

---
*Phase: 02-functional-gap-closure*
*Completed: 2026-08-01*

## Self-Check: PASSED

- FOUND: `crates/paladin-battalion/src/formation_service.rs`
- FOUND: `crates/paladin-core/src/platform/container/battalion/mod.rs`
- FOUND: `crates/paladin-herald/src/json_herald.rs`
- FOUND: `crates/paladin-herald/src/markdown_herald.rs`
- FOUND: `crates/paladin-herald/src/table_herald.rs`
- FOUND: `.planning/phases/02-functional-gap-closure/02-04-SUMMARY.md`
- FOUND: commit `7f22d49` (Task 1)
- FOUND: commit `fbbf33e` (Task 2)
- FOUND: commit `eea94bf` (Task 3)
- FOUND: commit `45d3438` (this SUMMARY)
