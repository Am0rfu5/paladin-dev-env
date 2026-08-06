---
phase: 07-workspace-ground-truth-recorded-answers
plan: 12
subsystem: docs
tags: [ledger, cargo-features, cli-isolation, api-surface, ADR-0019, cargo-tree]

# Dependency graph
requires:
  - phase: 07-workspace-ground-truth-recorded-answers
    provides: "07-01 ledger scaffold (115 REQ-* stubs); 07-07 ADR-0019/ADR-0021; 07-09 Epic 1 PRD corrections (diverged rows source)"
provides:
  - "Milestone 4 Epics 1, 2, 3 fully verdicted (25 rows) in .planning/ledgers/milestone-04-06.md"
  - "Ledger closed at 115/115 verdicted rows, zero PENDING-VERDICT tokens"
affects: [phase-08-verified-defect-closure, phase-16-documentation-currency, 07-13-summary-bookkeeping]

# Tech tracking
tech-stack:
  added: []
  patterns: ["manifest carve-out citation (Cargo.toml line + consuming CI job)", "diverged vs superseded-by-shipped-code verdict discipline per D-20"]

key-files:
  created: []
  modified:
    - .planning/ledgers/milestone-04-06.md

key-decisions:
  - "REQ-cli-dependency-isolation and REQ-library-only-build verdicted superseded by shipped code (not genuinely outstanding) per D-20/ADR-0019's corrected two-part scope: structopt's ungated src/main.rs consumer and paladin-herald's unconditional, featureless colored/comfy-table re-entry."
  - "REQ-api-surface-ci and REQ-deprecation-warnings verdicted genuinely outstanding, pointing at Phase 8 / DEBT-01 and DEBT-02, with the stale project/current-exports.txt baseline path (5 references across 2 scripts + 3 workflow lines) and the zero #[deprecated] grep result recorded fresh."
  - "REQ-feature-flag-matrix carries a third divergence found fresh this task (not previously recorded anywhere in the corpus): Cargo.toml:55 hardcodes paladin-llm's own openai/anthropic/deepseek/vision features on unconditionally, decoupled from root's llm-* flags, so provider adapter code always compiles inside paladin-llm regardless of which root LLM flag is set."
  - "REQ-visibility-hardening verdicted diverged: FR-2's pub(crate) mechanism was not applied anywhere meaningful (a single pub(crate) match project-wide); CHANGELOG.md:788-794 claims #[doc(hidden)] was applied to ~60 adapter/repository types but zero such annotations exist in the current paladin-llm source — a self-contradicting document, consistent with 07-CONTEXT.md's 'documents lie about themselves' pattern."
  - "REQ-cli-docs verdicted present, unproven (not satisfied): CONTRIBUTING.md does not exist anywhere in the repository, and cargo doc --workspace --no-deps is not clean, so FR9's full documentation/quality-gate bar is not cleared despite substantial README/migration-guide/CHANGELOG coverage."

requirements-completed: [ARCH-01, ARCH-05, ARCH-06]

coverage:
  - id: D1
    description: "Milestone 4 Epic 1 (Feature Flag Expansion) — 7 rows verdicted with fresh Cargo.toml/feature-flags.yml citations"
    requirement: "ARCH-01"
    verification:
      - kind: other
        ref: "sed -n '/### Milestone 4 Epic 1 /,/### Milestone 4 Epic 2 /p' .planning/ledgers/milestone-04-06.md | grep -c '^| REQ-' → 7; grep -c PENDING-VERDICT → 0"
        status: pass
    human_judgment: false
  - id: D2
    description: "Milestone 4 Epic 2 (Port Trait Hardening & Stable API) — 9 rows verdicted, 3 DEBT defects recorded outstanding pointing at Phase 8"
    requirement: "ARCH-01"
    verification:
      - kind: other
        ref: "sed -n '/### Milestone 4 Epic 2 /,/### Milestone 4 Epic 3 /p' .planning/ledgers/milestone-04-06.md | grep -c '^| REQ-' → 9; grep -c PENDING-VERDICT → 0; grep -c DEBT-0[123] → 1 each"
        status: pass
    human_judgment: false
  - id: D3
    description: "Milestone 4 Epic 3 (CLI Isolation) — 9 rows verdicted citing ADR-0019's corrected scope; ledger closed at 115/115"
    requirement: "ARCH-05, ARCH-06"
    verification:
      - kind: other
        ref: "grep -c PENDING-VERDICT .planning/ledgers/milestone-04-06.md → 0; grep -c '^| REQ-' → 115; grep -c '^### Milestone ' → 13; grep -o '^| REQ-[a-z0-9-]*' | sort | uniq -d → empty"
        status: pass
      - kind: other
        ref: "git diff --stat HEAD~1 -- '*.rs' 'Cargo.toml' '.github/' → empty (no product code touched)"
        status: pass
    human_judgment: true
    rationale: "Row-level file:line accuracy across 25 rows citing live cargo/grep output benefits from a human spot-check per 07-VALIDATION.md's Manual-Only Verifications row 1, even though the automated row/count/duplicate-ID checks all pass deterministically."

# Metrics
duration: 55min
completed: 2026-08-06
status: complete
---

# Phase 7 Plan 12: Milestone 4 Epics 1-3 Ledger Close-out Summary

**Verdicted the last 25 Milestone 4-6 ledger rows (Feature Flag Expansion, Port Trait Hardening, CLI Isolation) with fresh cargo/grep evidence, closing the ledger at 115/115 with zero PENDING-VERDICT tokens.**

## Performance

- **Duration:** 55 min
- **Started:** 2026-08-06T19:19:00Z
- **Completed:** 2026-08-06T20:14:17Z
- **Tasks:** 3
- **Files modified:** 1

## Accomplishments

- Verdicted all 7 Milestone 4 Epic 1 rows, recording two `diverged` positions (feature-flag-matrix's `web-server`/MCP corrections plus a freshly-found third divergence in `paladin-llm`'s hardcoded provider features at `Cargo.toml:55`; vision-feature-gating's empty `vision = []` flag) and one `relocated` row (feature-flag-docs → mdbook chapters).
- Verdicted all 9 Milestone 4 Epic 2 rows, treating the epic as genuinely incomplete per D-06: `REQ-api-surface-ci` and `REQ-deprecation-warnings` recorded `genuinely outstanding` pointing at Phase 8 / DEBT-01 and DEBT-02 with fresh evidence (stale `project/current-exports.txt` baseline path across 5 locations; zero `#[deprecated]` attributes anywhere in the tree); `REQ-port-trait-rustdoc` recorded `present, unproven` pointing at DEBT-03; `REQ-stable-api-doc` recorded `relocated`.
- Verdicted all 9 Milestone 4 Epic 3 rows, treating the epic's fully-checked checkbox list as the corpus's one case where checked boxes overstate completion: `REQ-cli-dependency-isolation` and `REQ-library-only-build` verdicted `superseded by shipped code` citing ADR-0019's corrected two-part scope (structopt's ungated `src/main.rs` consumer; `colored`/`comfy-table` re-entering via `paladin-herald`'s unconditional, featureless manifest), confirmed fresh via `cargo tree --offline --no-default-features`.
- Closed the ledger: `grep -c PENDING-VERDICT` returns 0, `grep -c '^| REQ-'` returns 115, `grep -c '^### Milestone '` returns 13, no duplicate `REQ-*` IDs.

## Task Commits

All 25 rows were written and committed in a single ledger-writing commit (per the plan's Task 3 instruction: "Commit the ledger once at the end of the plan"):

1. **Tasks 1-3: Milestone 4 Epics 1, 2, 3 — 25 rows verdicted, ledger closed at 115/115** — `db656bb` (docs)

## Files Created/Modified

- `.planning/ledgers/milestone-04-06.md` — 25 rows filled under `### Milestone 4 Epic 1`, `### Milestone 4 Epic 2`, `### Milestone 4 Epic 3`; ledger reaches 115 of 115 verdicted rows.

## Live Grep/Command Results (recorded per the plan's `<output>` spec)

- `grep -rn '#\[deprecated' src crates` → **0 matches** (workspace-wide).
- `grep -rn structopt src/ crates/` → **3 hits, all in `src/main.rs`** (lines 5, 8, 10).
- `cargo tree --offline --no-default-features` → lists `structopt v0.3.26`, `colored v2.2.0`/`v3.1.1`, `comfy-table v7.2.2` (confirming the ADR-0019 CLI-dependency-isolation gap).
- `Cargo.toml` lines cited: `:55` (paladin-llm forced provider features), `:93` (structopt), `:125-126` (colored/comfy-table), `:134-135` (chacha20poly1305/zeroize), `:240-252` (three `[[bin]]` targets), `:259` (default), `:268-287` (feature declarations).
- `crates/paladin-herald/Cargo.toml:22-23` — `comfy-table`/`colored` unconditional, no `[features]` section (re-confirmed).
- `.github/workflows/feature-flags.yml` lines cited: `:26-82` (15 matrix legs), `:112,115,118` (check/build/test triad), `:138,141,144,147` (cli-isolation job steps).
- `.github/workflows/ci.yml:172,182,187` + `scripts/check-api-surface.sh:6` + `scripts/extract-public-api.sh:6` — 5 references to the stale `project/current-exports.txt` path; actual file lives at `.project/current-exports.txt` (442,369 bytes); `.public-api-baseline.txt` never created.
- `cargo check --offline --workspace --lib --no-default-features` — `Finished` in 2m27s, 0 errors.
- `cargo check --offline --workspace --all-features` — `Finished` in 2m52s, 0 errors.
- `cargo build --offline --bin paladin` — `Finished`, 0 errors; `cargo build --offline --bin paladin-cli --features cli` — `Finished` in 25.13s, 0 errors.
- `cargo test --offline --test cli_isolation` — 9 passed, 0 failed.
- `cargo test --offline --lib --features cli application::cli::` — 192 passed, 0 failed, 6 ignored.
- `cargo doc --offline --no-deps --workspace` — `Finished` in 16.58s, **6 warnings** (3 in `paladin-battalion`, 3 in `paladin-ai`).
- Ledger closure: `grep -c PENDING-VERDICT` = 0, `grep -c '^| REQ-'` = 115, `grep -c '^### Milestone '` = 13, `grep -o '^| REQ-[a-z0-9-]*' | sort | uniq -d` = empty.

## Decisions Made

- **Third divergence in `REQ-feature-flag-matrix`, found fresh this task:** `Cargo.toml:55` hardcodes `paladin-llm`'s own `features = ["openai", "anthropic", "deepseek", "mock", "vision"]` on the dependency declaration, decoupled entirely from root's `llm-openai`/`llm-anthropic`/`llm-deepseek` flags — provider adapter code compiles inside `paladin-llm` regardless of which root LLM flag is enabled; only the root crate's re-export is conditionally gated. This is new information not previously recorded anywhere in the corpus and is now cited in the ledger row.
- **`REQ-visibility-hardening` verdicted `diverged` rather than `genuinely outstanding`:** the shipped tree supplants FR-2's `pub(crate)` mechanism with two different mechanisms — curated re-export lists (FR-1) and feature-gating (Epic 3) — and `CHANGELOG.md:788-794`'s claim that `#[doc(hidden)]` was applied to ~60 types is contradicted by a fresh `grep -rn 'doc(hidden)' crates/paladin-llm/src/` returning no output, an instance of the corpus's "documents lie about themselves" pattern.
- **`REQ-cli-dependency-isolation` and `REQ-library-only-build` both verdicted `superseded by shipped code`, not `genuinely outstanding`,** per D-20's explicit instruction to use the verdict class the evidence supports: the M4 `dependency-matrix.md` classification of `structopt`/`colored`/`comfy-table` as CLI-only was correct *at the time*; `paladin-herald` did not exist until its Milestone 8 extraction, so the "gap" is a later-tree fact, not an unfulfilled original requirement.
- **`REQ-cli-docs` verdicted `present, unproven`, not `satisfied`:** `CONTRIBUTING.md` does not exist anywhere in the repository (confirmed via `find . -maxdepth 1 -iname CONTRIBUTING.md` returning empty) and `cargo doc --workspace --no-deps` is not warning-clean, so FR9's full bar is not cleared despite substantial README/migration-guide/CHANGELOG coverage.

## Deviations from Plan

None — plan executed exactly as written. All prescribed dispositions (D-17 `diverged` rows, D-06 `genuinely outstanding` Epic 2 rows, D-20/ADR-0019 `superseded by shipped code` Epic 3 rows, D-18 `relocated` doc rows) were applied as instructed. One piece of new evidence was surfaced beyond the plan's explicit instructions (the `paladin-llm` hardcoded-features finding at `Cargo.toml:55`) and recorded inline per Rule 2 (auto-add missing critical evidence) rather than treated as an architectural change — it strengthens an already-prescribed `diverged` verdict rather than changing the verdict class.

## Known Stubs

None — this is a records-only plan; no product code was touched (`git diff --stat HEAD~1 -- '*.rs' 'Cargo.toml' '.github/'` is empty) and no stub tokens remain in the ledger (`grep -c PENDING-VERDICT` = 0).

## Issues Encountered

None. The worktree had no pre-existing `target/` directory, so the first `cargo check` was a cold build (~2m27s); subsequent commands reused the warm cache and completed in seconds to low minutes.

## Next Phase Readiness

- The Milestone 4-6 ledger is complete at 115/115 verdicted rows — ready for plan 07-13's summary/bookkeeping pass (PROMOTION.md next-free-line update, ledger `## Summary` section).
- Phase 8's DEBT-01 (api-surface CI), DEBT-02 (deprecation warnings), DEBT-03 (paladin-ports doctests), and DEBT-04 (CLI dependency isolation) all now have this ledger's fresh, corrected-scope evidence to plan against — DEBT-04 in particular must plan against ADR-0019's two-part `structopt`/`paladin-herald` scope, not the original three-line fix.
- Phase 16 receives the FR9.3 user-facing binary-architecture mdbook page as owed work (ADR-0019 plus this ledger's `REQ-binary-target-config`/`REQ-cli-docs` rows are the source record, not the deliverable itself).

---
*Phase: 07-workspace-ground-truth-recorded-answers*
*Completed: 2026-08-06*
