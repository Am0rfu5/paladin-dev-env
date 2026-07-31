---
phase: 01-ground-truth-decision-records
plan: 03
subsystem: docs
tags: [adr, formation, battalion, commander, temperature, provider-capabilities, llm]

# Dependency graph
requires:
  - phase: 01-ground-truth-decision-records (plan 01)
    provides: PROMOTION.md ADR conventions and the 0005-herald-trait.md reference shape
provides:
  - ".planning/decisions/0003-formation-min-paladins.md — RECON-04 decision record"
  - ".planning/decisions/0004-temperature-validation.md — RECON-05 decision record"
affects: [phase-02-functional-gap-closure, phase-14-api-contract-truthfulness]

# Tech tracking
tech-stack:
  added: []
  patterns:
    - "ADR records a live in-tree contradiction (both halves shipped and passing/enforced simultaneously) as a distinct case from a document-vs-code contradiction — Group-29 class, second instance"
    - "ADR specifies a not-yet-implemented struct field (identifier, Rust type, endpoint inclusivity, absent-value fallback, validation ordering) precisely enough that a later implementing plan has exactly one interpretation to build"

key-files:
  created:
    - .planning/decisions/0003-formation-min-paladins.md
    - .planning/decisions/0004-temperature-validation.md
  modified: []

key-decisions:
  - "D-14: Formation relaxes its minimum to 1 Paladin (integer count, no rounding); 0 stays rejected, 1 now accepted, 2 unchanged; the passing Commander Auto test is left untouched; Majority aggregation's independent minimum of 3 is unchanged."
  - "D-15: Temperature validation becomes provider-aware via a new `ProviderCapabilities.temperature_range: Option<(f32, f32)>` field, both endpoints inclusive, falling back to the existing global [0.0, 1.0] clamp when a provider declares no range, with the provider range checked before the autonomous task-type bands narrow within it."
  - "Both ADRs: Code Conformance = must change, GAP-07 (Phase 2) is the executing requirement; ADR-0004 additionally carries a sequencing note that Phase 14's WEB-03 touches the same ProviderCapabilities struct and must not be scheduled independently of GAP-07's temperature-range addition."

patterns-established:
  - "Pattern: when shipped code contains both halves of a contradiction (a passing test and a runtime rejection that would break it), the ADR relaxes the runtime check to match the test's expectation rather than rewriting the test, provided doing so doesn't break an independent, differently-reasoned check elsewhere (Majority's minimum of 3 stays untouched)."

requirements-completed: [RECON-04, RECON-05]

coverage:
  - id: D1
    description: "ADR-0003 records Formation's minimum Paladin count as 1, names both halves of the live contradiction (commander.rs:1911-1927 passing test, formation.rs:109-111 rejection), states behavior at 0/1/2 Paladins, preserves Majority's independent minimum of 3, and hands the code change to GAP-07."
    requirement: "RECON-04"
    verification:
      - kind: other
        ref: "node .claude/gsd-core/bin/lib/adr-parser.cjs --input .planning/decisions/0003-formation-min-paladins.md (status=accepted, non-empty decisions/options_considered/key_files)"
        status: pass
      - kind: other
        ref: "grep -q test_auto_selects_formation_for_single_paladin crates/paladin-battalion/src/commander.rs"
        status: pass
      - kind: other
        ref: "grep -qi 'at least 2' crates/paladin-core/src/platform/container/battalion/formation.rs"
        status: pass
    human_judgment: false
  - id: D2
    description: "ADR-0004 records temperature validation as provider-aware, specifies the new ProviderCapabilities.temperature_range field (identifier, Rust type, inclusive endpoints, absent-range fallback, provider-before-band validation order), records the WEB-03 sequencing note, and hands the ports-layer change to GAP-07."
    requirement: "RECON-05"
    verification:
      - kind: other
        ref: "node .claude/gsd-core/bin/lib/adr-parser.cjs --input .planning/decisions/0004-temperature-validation.md (status=accepted, non-empty decisions/options_considered/key_files)"
        status: pass
      - kind: other
        ref: "grep -q 'struct ProviderCapabilities' crates/paladin-ports/src/output/llm_port.rs && awk struct-body scan finds no 'temperature' field"
        status: pass
    human_judgment: false

duration: 12min
completed: 2026-07-31
status: complete
---

# Phase 1 Plan 3: RECON-04 and RECON-05 Decision Records Summary

**ADR-0003 relaxes Formation to a 1-Paladin minimum to reconcile a live in-tree contradiction; ADR-0004 specifies a provider-aware `temperature_range` field for `ProviderCapabilities` that does not exist yet, both handing their code changes to Phase 2's GAP-07**

## Performance

- **Duration:** 12 min
- **Started:** 2026-07-31T00:35:33Z
- **Completed:** 2026-07-31T00:47:10Z
- **Tasks:** 2
- **Files modified:** 2 (both new files)

## Accomplishments
- ADR-0003 records that shipped code contains both halves of the Formation-minimum contradiction — the passing `test_auto_selects_formation_for_single_paladin` at `commander.rs:1911-1927` and the `>= 2` rejection at `formation.rs:109-111` — and resolves it by relaxing Formation to accept 1 or more Paladins, stating behavior at 0, 1 and 2 explicitly, and leaving Majority aggregation's independent minimum of 3 (`phalanx.rs:141-146`) untouched.
- ADR-0004 records that `ProviderCapabilities` (`llm_port.rs:754`) has no temperature-range field today, so the "provider-aware" position in REQUIREMENTS.md was never implementable as specified, and that the shipped `[0.0, 1.0]` clamp (`paladin_builder.rs:1112-1117`) already contradicts the documented `0.0-2.0` default (`llm/config/llm.rs:14`) — both re-verified by direct read before citing.
- ADR-0004 specifies the new field precisely enough for one implementation: `temperature_range: Option<(f32, f32)>`, both endpoints inclusive, `[0.0, 1.0]` fallback when a provider declares no range, and the provider range checked before the autonomous task-type bands (`temperature_service.rs`, `autonomous_config.rs:106-116`) narrow within it.
- Both ADRs carry `Code Conformance: must change` and name **GAP-07** (Phase 2) as the executing requirement; ADR-0004 additionally records that Phase 14's **WEB-03** modifies the same `ProviderCapabilities` struct and the two must not be scheduled independently.

## Task Commits

Each task was committed atomically:

1. **Task 1: ADR-0003 — Formation relaxes to one Paladin** - `43aec54` (docs)
2. **Task 2: ADR-0004 — provider-aware temperature validation and the ProviderCapabilities field shape** - `fcb690b` (docs)

_Note: worktree mode — commits used `--no-verify` per orchestrator authorization (`workflow.worktree_skip_hooks=true`); the orchestrator runs the pre-commit hook once as the wave backstop._

## Files Created/Modified
- `.planning/decisions/0003-formation-min-paladins.md` - RECON-04 decision record: Formation minimum relaxes to 1 Paladin
- `.planning/decisions/0004-temperature-validation.md` - RECON-05 decision record: provider-aware temperature validation and the new `ProviderCapabilities` field shape

## Decisions Made
- **D-14** (ADR-0003): Formation's minimum Paladin count relaxes to 1, recorded as an integer count with no rounding; the passing Commander Auto test and `analyze_and_select` are left untouched; Majority aggregation's independent minimum of 3 is unchanged.
- **D-15** (ADR-0004): Temperature validation becomes provider-aware via `ProviderCapabilities.temperature_range: Option<(f32, f32)>`, both endpoints inclusive; falls back to the existing global `[0.0, 1.0]` clamp when a provider declares no range; provider range is checked before the autonomous task-type bands narrow within it; reversibility rated `costly` because every LLM adapter must populate the field and downstream consumers branch on it.

## Deviations from Plan

None - plan executed exactly as written. All `<read_first>` citations were re-verified by direct file read before being written into either ADR (per the plan's threat-model mitigation for T-01-02), and both citations matched the plan's expectations exactly (no drift found).

## Issues Encountered
None.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness
- RECON-04 and RECON-05 each have exactly one recorded, evidence-cited answer, satisfying Phase 1's success criteria for this plan.
- Phase 2's GAP-07 now has two concretely specified code changes to implement: relaxing `Formation::validate`'s bound at `formation.rs:109-111`, and adding `ProviderCapabilities.temperature_range` at `llm_port.rs:754` plus populating it in all three LLM adapters.
- Phase 14's WEB-03 planning must account for the sequencing note in ADR-0004 — it touches the same `ProviderCapabilities` struct as GAP-07's temperature-range addition.
- No Rust source file was modified by this plan, consistent with the plan's success criteria.

---
*Phase: 01-ground-truth-decision-records*
*Completed: 2026-07-31*
