---
phase: 01-ground-truth-decision-records
plan: 02
subsystem: docs
tags: [adr, battalion, citadel, herald, decision-record]

# Dependency graph
requires:
  - phase: 01-ground-truth-decision-records (plan 01)
    provides: ".planning/decisions/PROMOTION.md" conventions and the "0005-herald-trait.md" worked ADR shape
provides:
  - "ADR-0001: BattalionConfig field set — battalion/mod.rs:37 named authoritative; citadel.rs:280 placeholder duplicate renamed BattalionCheckpointConfig; must change with GAP-07 named"
  - "ADR-0002: BattalionResult field set — battalion/mod.rs:549 merged-superset named authoritative with every field substitution recorded; conforms, no GAP-07 action"
affects: [phase-02-gap-closure, phase-07-milestone-4-6-closeout]

# Tech tracking
tech-stack:
  added: []
  patterns: ["ADR authoring against PROMOTION.md's required heading set and adr-parser.cjs's bullet-list constraint"]

key-files:
  created:
    - .planning/decisions/0001-battalion-config.md
    - .planning/decisions/0002-battalion-result.md
  modified: []

key-decisions:
  - "BattalionConfig at battalion/mod.rs:37 is the one authoritative definition; the citadel.rs:280 struct is a self-described placeholder renamed BattalionCheckpointConfig with an unchanged serde shape (no migration)"
  - "BattalionResult at battalion/mod.rs:549 is a merged superset of Epic 4, Epic 5 and Epic 8 positions; per_paladin_times displaced execution_time_ms and node_errors displaced errors: Vec<PaladinError> because BattalionError does not derive Serialize/Deserialize"
  - "Type-ownership question (PaladinResult/StopReason/TokenUsage/RegistryError/HandoffError location) is explicitly left to Phase 7's ARCH-03(c) — ADR-0002 does not contradict the Milestone 5 Epic 1 decision document"

patterns-established: []

requirements-completed: [RECON-02, RECON-03]

coverage:
  - id: D1
    description: "ADR-0001 records BattalionConfig's authoritative definition and the citadel.rs:280 duplicate's disposition (renamed, must-change, GAP-07 named)"
    requirement: "RECON-02"
    verification:
      - kind: other
        ref: "node .claude/gsd-core/bin/lib/adr-parser.cjs --input .planning/decisions/0001-battalion-config.md (status=accepted, non-empty decisions/options_considered/key_files) + grep checks for BattalionCheckpointConfig, GAP-07, REQ-commander-config-metadata-dir-v3, citadel.rs:280, and the three field names against citadel.rs"
        status: pass
    human_judgment: false
  - id: D2
    description: "ADR-0002 records BattalionResult's authoritative definition and every field substitution the merged superset made"
    requirement: "RECON-03"
    verification:
      - kind: other
        ref: "node .claude/gsd-core/bin/lib/adr-parser.cjs --input .planning/decisions/0002-battalion-result.md (status=accepted, non-empty decisions/options_considered/key_files) + grep checks for per_paladin_times, execution_time_ms, node_errors, strategy_used, total_tokens, per_paladin_tokens, REQ-battalion-metadata-extension against the ADR and the shipped struct"
        status: pass
    human_judgment: false

duration: 25min
completed: 2026-07-31
status: complete
---

# Phase 1 Plan 2: Battalion Decision Records Summary

**Authored ADR-0001 (BattalionConfig) and ADR-0002 (BattalionResult), settling RECON-02 and RECON-03 with re-verified `file:line` citations and every rejected variant named.**

## Performance

- **Duration:** ~25 min
- **Completed:** 2026-07-31T00:46:50Z
- **Tasks:** 2
- **Files modified:** 2 (both created)

## Accomplishments
- `.planning/decisions/0001-battalion-config.md` names `battalion/mod.rs:37` as the one authoritative `BattalionConfig`, quotes the `citadel.rs:280` placeholder's own doc comment, renames it `BattalionCheckpointConfig` with an explicitly unchanged serde shape, and flags `must change` with GAP-07 as the executing requirement.
- `.planning/decisions/0002-battalion-result.md` names `battalion/mod.rs:549` as the authoritative merged-superset `BattalionResult`, records which field each of the three source positions contributed or lost (`per_paladin_times` over `execution_time_ms`; `node_errors: Vec<NodeError>` over `errors: Vec<PaladinError>`, with the `BattalionError` non-`Serialize` reason verified against the tree), and flags `conforms`.
- Both ADRs list every rejected `REQ-*` variant in REQUIREMENTS.md group order, including `REQ-commander-config-metadata-dir-v3` (never built) and `REQ-battalion-metadata-extension` (satisfied but with a module-path/type mismatch noted rather than silently smoothed over).
- ADR-0002 explicitly records that it does not decide the Milestone 5 Epic 1 type-ownership question, naming Phase 7 as owner, so it does not contradict `battalion-result-upward-dependency-decision.md`.

## Task Commits

Each task was committed atomically:

1. **Task 1: ADR-0001 — the authoritative BattalionConfig and the citadel.rs duplicate** - `6c50118` (docs)
2. **Task 2: ADR-0002 — the merged-superset BattalionResult** - `cae7455` (docs)

**Plan metadata:** SUMMARY.md commit (this file) — see final commit below.

## Files Created/Modified
- `.planning/decisions/0001-battalion-config.md` - RECON-02 decision record: BattalionConfig authoritative definition, citadel.rs:280 duplicate disposition
- `.planning/decisions/0002-battalion-result.md` - RECON-03 decision record: BattalionResult merged-superset field-substitution record

## Decisions Made
- **`BattalionCheckpointConfig` as the citadel.rs:280 rename.** CONTEXT.md left the exact identifier to the planner's discretion; this ADR fixes it. Chosen to reflect that the struct's fields (`max_concurrency`, `timeout_seconds`, `continue_on_error`) are checkpoint/resume knobs distinct from orchestration `BattalionConfig`.
- **No persisted-schema migration for the rename.** `BattalionState.config: BattalionConfig` at `citadel.rs:233` with `schema_version: "1.0.0"` is consumed by `file_citadel.rs`; replacing the placeholder with the real `BattalionConfig` would force a schema version bump and a legacy-checkpoint read path. A rename-in-place avoids both, so ADR-0001 records this as the reason no migration is needed.
- **`REQ-battalion-metadata-extension` recorded as satisfied-with-caveats rather than fully matched.** Its `battalion/battalion_result.rs` module path and `Vec<u64>` typing for `per_paladin_times` don't match what shipped (`battalion/mod.rs`, `HashMap<String, u64>`), but the field's presence and name are what the ADR credits it for — consistent with the plan's instruction to name substitutions rather than assert satisfaction in the abstract.

## Deviations from Plan

None - plan executed exactly as written. All re-read citations (`battalion/mod.rs:37`, `:549`, `:497`; `citadel.rs:233`, `:280`; `file_citadel.rs:507`, `:541`; `herald.rs:49`; `BattalionError`'s derive list at `:759`) were independently verified against the tree on 2026-07-31 before being written into either ADR, and both ADRs' automated `<verify>` commands passed on first run.

## Issues Encountered
None.

## User Setup Required
None - no external service configuration required.

## Next Phase Readiness
- RECON-02 and RECON-03 are both closed with a single recorded, evidence-cited answer each.
- The `citadel.rs:280` duplicate now has a named replacement identifier (`BattalionCheckpointConfig`), an explicitly unchanged serde shape, and a named executing requirement (Phase 2 GAP-07) — GAP-07 can proceed against a concrete target name rather than an open question.
- No Rust source file was modified by this plan; the codebase is unchanged pending Phase 2.

---
*Phase: 01-ground-truth-decision-records*
*Completed: 2026-07-31*
