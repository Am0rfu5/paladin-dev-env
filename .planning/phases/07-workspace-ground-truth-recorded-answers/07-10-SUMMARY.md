---
phase: 07-workspace-ground-truth-recorded-answers
plan: 10
subsystem: docs
tags: [records, ledger, adr, paladin-battalion, paladin-llm, workspace-decomposition]

# Dependency graph
requires:
  - phase: 07-workspace-ground-truth-recorded-answers
    provides: "Ledger scaffold (07-01), ADR-0015/0017/0018/0020 (07-06/07-04/07-05/07-11 wave lineage)"
provides:
  - "Milestone 5 Epic 3 (paladin-battalion extraction, 9 rows) fully cited and verdicted"
  - "Milestone 5 Epic 4 (paladin-llm extraction, 11 rows) fully cited and verdicted"
  - "A residual finding on REQ-battalion-facade-shim: the shim mechanism persists today (renamed via M8 Epic 4), narrower than ADR-0018 (iv)'s 'Milestone 6 retired it' framing"
affects: ["07-13"]

# Tech tracking
tech-stack:
  added: []
  patterns:
    - "Two-clause row evidence (REQ-llm-build-validation): build-time sub-target and incremental-improvement target addressed and verdicted separately rather than folded into one row verdict"
    - "diverged verdict used for structurally-matching-but-differently-named implementations (LlmProviderError variant set, ProviderFactoryError visibility)"

key-files:
  created: []
  modified:
    - .planning/ledgers/milestone-04-06.md

key-decisions:
  - "REQ-battalion-facade-shim recorded superseded by shipped code citing ADR-0018 per the plan's prescribed disposition, but the Evidence cell also records a fresh-grep residual finding: the shim itself (not just its use_cases/battalion/ directory) persists today at the M8-renamed src/application/services/battalion/mod.rs, consumed by ~19 files — narrower than ADR-0018 clause (iv)'s literal 'Milestone 6 retired it' claim. Recorded per this plan's own instruction to apply D-06 per-row rather than assume, without reopening the M5→M6 no-new-shim policy question ADR-0018 actually settles."
  - "REQ-llm-provider-error and REQ-llm-provider-factory recorded diverged rather than satisfied: both match their requirement's architectural intent (conversion boundary; feature-gated graceful degradation) but ship different concrete shapes (variant names/types; ProviderFactoryError is pub not private, no From<ProviderFactoryError> for LlmProviderError)"
  - "REQ-llm-mock-adapters recorded present, unproven: MultiStepMockLlmPort matches exactly, but the row's named MockLlmPort type does not exist — the shipped type is MockLlmAdapter"
  - "REQ-llm-test-architecture and REQ-llm-facade-prelude recorded relocated: integration tests and the seven LLM-adapter re-exports exist and are exercised, but at the workspace tests/integration/ path and the paladin:: crate root respectively, not at the crate-local crates/paladin-llm/tests/ or paladin::prelude paths the requirements name"
  - "REQ-battalion-crate-docs recorded present, unproven: cargo doc -p paladin-battalion --no-deps produces 3 warnings (two unresolved intra-doc links, one unclosed HTML tag), against FR-24's explicit zero-warnings bar, despite clean clippy/fmt"

requirements-completed: [ARCH-01, ARCH-03, ARCH-04, ARCH-07]

coverage:
  - id: D1
    description: "9 Milestone 5 Epic 3 (paladin-battalion) ledger rows verdicted with fresh file:line citations and scoped cargo test/tree/doc/clippy/fmt runs"
    requirement: "ARCH-01"
    verification:
      - kind: unit
        ref: "cargo test --offline -p paladin-battalion --lib — 219 passed, 0 failed"
        status: pass
      - kind: unit
        ref: "cargo test --offline --test lib battalion — 177 passed, 0 failed"
        status: pass
      - kind: other
        ref: "sed -n '/Milestone 5 Epic 3/,/Milestone 5 Epic 4/p' .planning/ledgers/milestone-04-06.md | grep -c 'PENDING-VERDICT' == 0"
        status: pass
    human_judgment: false
  - id: D2
    description: "11 Milestone 5 Epic 4 (paladin-llm) ledger rows verdicted with fresh file:line citations and scoped cargo test/tree/doc/clippy/fmt runs across the six-leg feature matrix"
    requirement: "ARCH-01"
    verification:
      - kind: unit
        ref: "cargo test --offline -p paladin-llm --all-features --lib — 80 passed, 0 failed"
        status: pass
      - kind: other
        ref: "cargo build -p paladin-llm run for --no-default-features / --features {openai,anthropic,deepseek,mock} / --all-features — all 6 legs Finished, 0 errors"
        status: pass
      - kind: other
        ref: "sed -n '/Milestone 5 Epic 4/,/Milestone 5 Epic 5/p' .planning/ledgers/milestone-04-06.md | grep -c 'PENDING-VERDICT' == 0"
        status: pass
    human_judgment: false
  - id: D3
    description: "REQ-battalion-facade-shim verdicted superseded by shipped code citing ADR-0018, with the M5→M6 posture flip recorded as history and a residual finding about the shim's actual lifecycle"
    requirement: "ARCH-04"
    verification:
      - kind: other
        ref: "grep -n 'REQ-battalion-facade-shim' .planning/ledgers/milestone-04-06.md — verdict cell reads 'superseded by shipped code', Evidence cites ADR-0018"
        status: pass
    human_judgment: false
  - id: D4
    description: "REQ-paladin-core-dependency-allowlist-v2 verdicted citing ADR-0015, confirming petgraph version alignment between paladin-core and paladin-battalion; REQ-llm-config-bridge-location-v1 verdicted citing ADR-0017; REQ-llm-build-validation's incremental-improvement clause verdicted citing ADR-0020's per-scenario restatement"
    requirement: "ARCH-03"
    verification:
      - kind: other
        ref: "grep -c ADR-0015/ADR-0017/ADR-0020 in the respective epic sections — each >= 1"
        status: pass
    human_judgment: false
  - id: D5
    description: "Ledger row-count invariants hold: 115 total REQ-* rows, PENDING-VERDICT dropped from 61 to 41, 13 milestone headers, no duplicate REQ-* IDs, no *.rs/Cargo.toml/.github/ file touched"
    requirement: "ARCH-01"
    verification:
      - kind: other
        ref: "grep -c '^| REQ-' .planning/ledgers/milestone-04-06.md == 115; grep -c PENDING-VERDICT == 41; grep -c '^### Milestone ' == 13; grep -o '^| REQ-...' | sort | uniq -d prints nothing; git diff --stat HEAD~1 -- '*.rs' 'Cargo.toml' '.github/' empty"
        status: pass
    human_judgment: false

duration: ~55min
completed: 2026-08-06
status: complete
---

# Phase 07 Plan 10: Milestone 5 Epic 3-4 Ledger Rows Summary

**Verdicted all 20 `paladin-battalion` and `paladin-llm` extraction rows in `.planning/ledgers/milestone-04-06.md`, re-grepping every citation fresh against the live tree — including the `crate-isolation` job (`ci.yml:315`, header at `:304`, not the stale `:228`) and the `feature-flags.yml` per-provider legs (`:115,118,141`) — and running 299 scoped `cargo test` assertions (219 battalion + 80 llm) plus the full six-leg `paladin-llm` feature matrix, all passing.**

## Performance

- **Duration:** ~55 min
- **Tasks:** 2
- **Files modified:** 1 (`.planning/ledgers/milestone-04-06.md`)

## Accomplishments

- Filled all 9 Milestone 5 Epic 3 (`paladin-battalion` extraction) ledger rows with `file:line` citations resolved fresh against the tree this task, backed by `cargo test --offline -p paladin-battalion --lib` (219 passed), `cargo tree` isolation checks, `cargo doc`/`clippy`/`fmt` gates, and six named-example compiles.
- Filled all 11 Milestone 5 Epic 4 (`paladin-llm` extraction) ledger rows, backed by `cargo test --offline -p paladin-llm --all-features --lib` (80 passed), all six feature-matrix build legs (`--no-default-features`, `--features {openai,anthropic,deepseek,mock}`, `--all-features`), and `cargo tree`/`clippy`/`fmt`/`doc` gates.
- `REQ-battalion-facade-shim` recorded `superseded by shipped code` citing ADR-0018 per the plan's prescribed disposition, with an additional fresh-grep residual finding recorded in the Evidence cell (see Deviations below).
- `REQ-paladin-core-dependency-allowlist-v2` recorded citing ADR-0015, confirming `petgraph = "0.6"` pinned identically in both `paladin-core/Cargo.toml:27` and `paladin-battalion/Cargo.toml:28` (no duplicate compilation).
- `REQ-llm-config-bridge-location-v1` recorded `superseded by shipped code` citing ADR-0017 and the plan-07-04 FR-31/FR-32 annotation in the source PRD.
- `REQ-llm-build-validation` split into its two named clauses per the plan's instruction: the feature-matrix/quality-gate half verdicted `present, unproven`-contributing pass evidence, and the `≥ 50%` incremental-improvement clause cites ADR-0020's per-scenario restatement (2 of 5 scenarios pass: `paladin-memory` −50.2%, `paladin-battalion` −90.9%).
- Verified the stub arithmetic: `PENDING-VERDICT` count dropped from 61 to exactly 41 (61 − 20 = 41), row count held at 115, no duplicate `REQ-*` IDs, no `.rs`/`Cargo.toml`/`.github/` file touched.

## Task Commits

1. **Task 1 + Task 2 combined (per plan instruction — commit once at the end, not per row):** `23ea295` (docs)

**Plan metadata:** commit for STATE.md/ROADMAP.md is intentionally *not* made by this plan — the orchestrator owns those writes after all worktree agents in the wave complete, per this plan's explicit prompt objective.

## Files Created/Modified

- `.planning/ledgers/milestone-04-06.md` — 20 rows verdicted and cited under `### Milestone 5 Epic 3` and `### Milestone 5 Epic 4`

## Decisions Made

- **`REQ-battalion-facade-shim`:** followed the plan's prescribed `superseded by shipped code` verdict citing ADR-0018, and additionally recorded a fresh-grep finding in the Evidence cell: `src/application/use_cases/battalion/` (the literal FR-14 path) does not exist, matching ADR-0018's own citation — but the shim *mechanism* was not deleted, only carried forward under the unrelated Milestone 8 Epic 4 `use_cases`→`services` rename. It lives today at `src/application/services/battalion/mod.rs`, still a thin `pub use paladin_battalion::<module>;` re-export block, declared in `src/application/services/mod.rs:3`, and consumed by ~19 example/test files. This is recorded as a narrower fact than ADR-0018 clause (iv)'s "Milestone 6 retired it" — not as a contradiction to reopen (the no-*new*-shim policy question ADR-0018 answers is a separate, settled question about the Epic 2/Epic 4 orchestration and CircuitBreaker relocations), but as the D-06 fresh-per-row grep this phase's evidence bar requires. Downstream readers (Phase 11 / FACADE-02 D1) still get the settled no-new-shim policy from ADR-0018; this residual note is scoped narrowly to this one shim's literal lifecycle.
- **`diverged` used for two rows** (`REQ-llm-provider-error`, `REQ-llm-provider-factory`) where the shipped implementation matches the requirement's architectural intent but ships different concrete type shapes than the requirement names — distinguishing this from `present, unproven` (nothing exercises the claim) since both are fully exercised and passing under their own shapes.
- **`relocated` used for two rows** (`REQ-llm-test-architecture`, `REQ-llm-facade-prelude`) where the deliverable exists and is exercised, but at the workspace-level `tests/integration/` path or the `paladin::` crate root rather than the crate-local `crates/paladin-llm/tests/` or `paladin::prelude` paths the requirements name, applying D-02's tie-break.
- **`REQ-llm-build-validation`'s no-default-features "<5 seconds" build-time sub-clause recorded `present, unproven`** rather than pass/fail: no comparable historical baseline exists for this specific figure (`build-benchmarks.md` measures workspace-wide scenarios, not a `paladin-llm --no-default-features` figure), and a cold-vs-warm cargo build timing in this sandboxed environment (14.43s cold, 0.23s warm no-op) is not a meaningful substitute for the CI-cache-warm baseline the original target assumed.

## Deviations from Plan

None requiring the four deviation rules (no bug fixes, no missing-critical-functionality additions, no blocking issues, no architectural changes) — this plan is records-only per its phase character and touched no product code. One judgment call is worth flagging explicitly, documented above under Decisions Made: the `REQ-battalion-facade-shim` Evidence cell records both the plan's prescribed disposition (superseded by shipped code, citing ADR-0018) *and* a fresh-grep residual finding that qualifies ADR-0018 clause (iv)'s literal claim. This is not a deviation from the plan's instructions — the plan's own `<phase_character>` section explicitly directs "re-grep every citation against the live tree" and D-06's per-row corroboration discipline — but it does add detail beyond the plan's minimum prescribed text, so it is called out here for visibility.

## Issues Encountered

None. All scoped `cargo test`/`tree`/`doc`/`clippy`/`fmt` commands ran cleanly to completion within timeout budgets; no auth gates, no missing dependencies, no blocked commands.

## User Setup Required

None — no external service configuration required.

## Next Phase Readiness

- All 20 rows in Milestone 5 Epics 3-4 are cited and verdicted; the ledger's row-count and duplicate-ID invariants hold (115 rows, 41 remaining `PENDING-VERDICT`, 13 section headers).
- Plan 07-13's final ledger closeout (nested outstanding-item count, `PROMOTION.md` ADR index update) can proceed against a ledger with 74/115 rows now verdicted (61 filled by earlier waves + this plan's 20, minus the wave-1 scaffold... — precise running total not recomputed here; verify via `grep -c PENDING-VERDICT` at closeout time rather than trusting this figure forward, per this ledger's own D-01 discipline).
- No blockers. The `REQ-battalion-facade-shim` residual finding is recorded, not escalated — it does not change the no-new-shim policy Phase 11 consumes from ADR-0018 (iv), only the accuracy of one adjacent factual claim in that ADR's Context section, which this phase's `.planning/decisions/*.md` files are not modified by (per this plan's prohibitions).

---
*Phase: 07-workspace-ground-truth-recorded-answers*
*Completed: 2026-08-06*

## Self-Check: PASSED

- FOUND: `.planning/phases/07-workspace-ground-truth-recorded-answers/07-10-SUMMARY.md`
- FOUND: commit `23ea295` (ledger rows)
- FOUND: commit `c41d774` (this SUMMARY)
