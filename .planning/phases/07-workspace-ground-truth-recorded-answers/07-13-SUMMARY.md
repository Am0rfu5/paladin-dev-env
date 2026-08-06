---
phase: 07-workspace-ground-truth-recorded-answers
plan: 13
subsystem: planning-records
tags: [adr-bookkeeping, ledger-close-out, promotion-numbering, coverage-declaration, phase-boundary]

requires:
  - phase: 07-02..07-12
    provides: all 115 ledger rows verdicted, eight ADR files (0014-0021) authored, source corrections landed
provides:
  - "milestone-04-06.md's counted `## Summary` section (verdict distribution, per-milestone roll-up, forward-scope handoffs, citation resolution)"
  - "REQUIREMENTS.md Milestone 4-6 pointer finalised with the real nested-item count (0)"
  - "PROMOTION.md advanced to next-free 0022 with all eight new ADR index rows and both Part B candidates closed"
  - "PROJECT.md Key Decisions rows for ADR-0014 through ADR-0021"
  - "COVERAGE.md reasoned no-external-API declaration for the phase"
  - "ADR-0018 clause (iv) amended in place to narrow the battalion-shim historical claim"
affects: [phase-08-verified-defect-closure, phase-11-facade-cleanup, phase-15, phase-16]

tech-stack:
  added: []
  patterns: ["D-00f in-place ledger amendment", "D-00g dated .project/ annotation", "ADR clause-level amendment retaining superseded text"]

key-files:
  created:
    - .planning/phases/07-workspace-ground-truth-recorded-answers/COVERAGE.md
    - .planning/phases/07-workspace-ground-truth-recorded-answers/07-13-SUMMARY.md
  modified:
    - .planning/ledgers/milestone-04-06.md
    - .planning/REQUIREMENTS.md
    - .planning/decisions/PROMOTION.md
    - .planning/PROJECT.md
    - .planning/decisions/0018-m6-facade-reexport-policy.md

key-decisions:
  - "Verdict distribution counted fresh from the finished 115-row ledger, not transcribed from any earlier plan's running total: satisfied 71, present-unproven 15, genuinely-outstanding 3, deferred-with-reason 1, superseded-by-shipped-code 12, relocated 5, diverged 8"
  - "Nested outstanding-item count is 0 — no row uses the blank-first-two-column nested-row format; every finding is folded inline into its host row's Evidence cell"
  - "PROMOTION.md next-free line advances to 0022, not 0021, because D-25a allocated an eighth ADR (0021, closing Part B candidate 2) beyond D-25's original seven"
  - "ADR-0018 clause (iv) amended in place (dated, superseded text retained): the battalion facade shim's literal directory (src/application/use_cases/battalion/) is gone, but the re-export mechanism itself survives at src/application/services/battalion/mod.rs under a Milestone 8 rename, consumed by 36 files — this narrows one historical claim without reopening the no-new-shim policy clause (i) settles"

requirements-completed: [ARCH-01, ARCH-02, ARCH-03, ARCH-04, ARCH-05, ARCH-06, ARCH-07]

coverage:
  - id: D1
    description: "Ledger ## Summary section with counted verdict distribution, per-milestone roll-up, nested-outstanding-item count, forward-scope handoffs, and citation resolution"
    requirement: "ARCH-01"
    verification:
      - kind: other
        ref: "grep -c '^## Summary' .planning/ledgers/milestone-04-06.md == 1; grep -c '^| REQ-' == 115; verdict counts sum to 115 by direct grep -oP recount"
        status: pass
    human_judgment: false
  - id: D2
    description: "PROMOTION.md advanced to next-free 0022 with eight new index rows and both Part B candidates closed"
    requirement: "ARCH-02"
    verification:
      - kind: other
        ref: "grep -c 'Next free ADR number: 0022' == 1; grep -c 'Next free ADR number: 0014' == 0; all eight | 00NN | rows present"
        status: pass
    human_judgment: false
  - id: D3
    description: "PROJECT.md Key Decisions gains one row per new ADR (0014-0021), existing evidence note untouched"
    requirement: "ARCH-02"
    verification:
      - kind: other
        ref: "git diff --numstat -- .planning/PROJECT.md: 8 insertions, 0 deletions"
        status: pass
    human_judgment: false
  - id: D4
    description: "COVERAGE.md reasoned no-external-API declaration, no fabricated capability matrix"
    verification:
      - kind: other
        ref: "test -f COVERAGE.md; grep -ci 'No external API integration' >= 1; grep -c '|' == 0; wc -l >= 6"
        status: pass
    human_judgment: false
  - id: D5
    description: "Whole-phase restricted git diff over *.rs, Cargo.toml, .github/ is empty (the phase's hard boundary)"
    verification:
      - kind: other
        ref: "git diff --stat 2835b1a..HEAD -- '*.rs' 'Cargo.toml' '.github/' — empty"
        status: pass
    human_judgment: false
  - id: D6
    description: "ADR-0018 clause (iv) amended in place to match the verified battalion-shim state, orchestrator-directed reconciliation not in the original plan text"
    verification:
      - kind: other
        ref: "grep -c '^## ' 0018-m6-facade-reexport-policy.md == 7 (unchanged); grep -rln 'application::services::battalion' src/ tests/ examples/ crates/ — 36 files, re-verified"
        status: pass
    human_judgment: false

duration: ~50min
completed: 2026-08-06
status: complete
---

# Phase 7 Plan 13: Close-out — Ledger Summary, ADR Bookkeeping, and Coverage Declaration Summary

**Counted (not transcribed) the finished 115-row ledger's verdict distribution, advanced PROMOTION.md's ADR index to next-free 0022 with both Phase-7-owned candidates closed, added eight PROJECT.md Key Decisions rows, wrote the phase's COVERAGE.md declaration, and amended ADR-0018 clause (iv) to narrow an overstated historical claim about the battalion facade shim.**

## Performance

- **Duration:** ~50 min
- **Completed:** 2026-08-06
- **Tasks:** 3 (plus one orchestrator-directed reconciliation folded into Task 3)
- **Files modified:** 6 (5 planned + ADR-0018, per the orchestrator's explicit reconciliation instruction)

## Accomplishments

- Appended `.planning/ledgers/milestone-04-06.md`'s `## Summary` section: a counted 115-row verdict
  distribution, a per-milestone roll-up explaining how each milestone's own checkbox claim held up,
  a 0-count nested-outstanding-items sub-section, a `### Forward scope` sub-section naming all five
  code consequences by owning phase/requirement/ADR, and a `### Citation resolution` sub-section
  recording the confirmed `crate-isolation` job citation drift and the `REQ-battalion-facade-shim`
  residual finding this plan then resolved at source.
- Finalised REQUIREMENTS.md's Milestone 4-6 pointer with the real nested-item count (0), replacing
  plan 07-01's placeholder sentence.
- Advanced `PROMOTION.md`'s ADR numbering index with eight new rows (0014-0021), moved the
  next-free line to 0022 with a dated note explaining the D-25a jump, and closed both Phase-7-owned
  Part B candidates (candidate 1 by ADR-0016, candidate 2 by ADR-0021).
- Added eight `PROJECT.md` Key Decisions rows, one per new ADR, leaving the existing zero-locked-
  decisions evidence note untouched (0 deletions in the diff).
- Wrote `COVERAGE.md` with a reasoned no-external-API declaration, matching the shape of the Phase 5
  and Phase 6 declarations (no fabricated capability matrix — 0 pipe characters in the file).
- **Amended ADR-0018 clause (iv) in place** (dated, superseded text retained per D-00f) to correct
  its overstated "Milestone 6 retired it" claim: the literal `src/application/use_cases/battalion/`
  directory is gone, but the re-export shim mechanism survives at
  `src/application/services/battalion/mod.rs` under an unrelated Milestone 8 rename, consumed by 36
  files (5 `src/`, 18 `tests/`, 13 `examples/`). This does not reopen the no-new-shim policy clause
  (i) settles.
- Confirmed the whole-phase records-only boundary: `git diff --stat 2835b1a..HEAD -- '*.rs'
  'Cargo.toml' '.github/'` is empty across every commit from `e0b793a` (07-01) through this plan's
  own commit.

## Task Commits

Tasks 1-3 (plus the orchestrator-directed ADR-0018 reconciliation) are committed together per the
plan's own instruction ("Commit this plan's five files in a single commit at the end of the plan"),
extended to six files to include the ADR-0018 amendment the orchestrator's phase_character block
required:

1. **Tasks 1-3 + ADR-0018 reconciliation** - see commit hash below (docs)

**Plan metadata:** see final commit hash (docs: complete plan)

## Files Created/Modified

- `.planning/ledgers/milestone-04-06.md` - counted `## Summary` section appended
- `.planning/REQUIREMENTS.md` - Milestone 4-6 pointer finalised with real nested-item count
- `.planning/decisions/PROMOTION.md` - eight new index rows, next-free 0022, Part B candidates closed
- `.planning/PROJECT.md` - eight new Key Decisions rows
- `.planning/decisions/0018-m6-facade-reexport-policy.md` - clause (iv) amended in place
- `.planning/phases/07-workspace-ground-truth-recorded-answers/COVERAGE.md` - new, no-external-API declaration

## Decisions Made

- **Verdict distribution, counted fresh:** `satisfied` 71, `present, unproven` 15,
  `genuinely outstanding` 3, `deferred with reason` 1, `superseded by shipped code` 12,
  `relocated` 5, `diverged` 8 — sums to 115, matching `grep -c '^| REQ-'` exactly.
- **Per-milestone breakdown:** M4 (25 IDs) — satisfied 9, present-unproven 6, genuinely-outstanding
  2, superseded 2, relocated 2, diverged 4. M5 (56 IDs) — satisfied 37, present-unproven 6,
  genuinely-outstanding 1, deferred-with-reason 1, superseded 7, relocated 2, diverged 2. M6 (34
  IDs) — satisfied 25, present-unproven 3, superseded 3, relocated 1, diverged 2, 0 genuinely
  outstanding, 0 deferred.
- **Nested outstanding items: 0.** Confirmed via `grep -n '^| *|' .planning/ledgers/milestone-04-06.md`
  printing nothing — no row uses the blank-first-two-column nested-row format the primary-key
  convention (D-00e) permits.
- **PROMOTION.md next-free line is 0022, not 0021**, because D-25a allocated an eighth ADR beyond
  D-25's seven — recorded with a dated note in `PROMOTION.md` itself so a future reader does not
  mistake the jump for a skipped number.
- **ADR-0018 clause (iv) amendment scope:** narrowed, not reversed. The no-new-shim policy (clause
  i) is unaffected — Non-Goal 7 and FR-4.11 name the orchestration-service and CircuitBreaker
  relocations specifically, neither of which this finding touches. Only the historical claim about
  what Milestone 6 removed is corrected: it retired the directory and the going-forward posture,
  not the pre-existing Milestone-5-era shim mechanism itself, which survived a later unrelated
  rename.
- **`Cargo.toml:55`'s hardcoded `paladin-llm` feature set** (a third `REQ-feature-flag-matrix`
  divergence, found by plan 07-12 and re-confirmed this task) is already recorded inline in that
  row's own Evidence cell — noted in the ledger's Citation resolution sub-section for visibility,
  not duplicated as a separate finding, and not fixed, per the phase's record-only boundary.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 2 - Missing Critical] ADR-0018 clause (iv) amendment, directed by the orchestrator's
phase_character block rather than by 07-13-PLAN.md's own `<tasks>`**
- **Found during:** Task 3 preparation (re-reading plan 07-10's SUMMARY.md, per the orchestrator's
  explicit instruction)
- **Issue:** `07-13-PLAN.md`'s own three tasks do not mention ADR-0018 at all. The orchestrator's
  `<phase_character>` block, however, explicitly requires this reconciliation as part of the
  phase's close-out, and the orchestrator's `<success_criteria>` checklist lists "ADR-0018 clause
  (iv) amended in place to match the verified shim state" as a required outcome. Plan 07-10 had
  already found and recorded the residual finding in the ledger but deliberately did not touch
  `.planning/decisions/*.md` (per its own prohibitions), leaving the ADR amendment for this
  close-out plan.
- **Fix:** Re-verified the finding fresh this task (`test -d src/application/use_cases` fails;
  `src/application/services/battalion/mod.rs` exists, self-describes as "a thin shim", declared in
  `src/application/services/mod.rs`; `grep -rln 'application::services::battalion' src/ tests/
  examples/ crates/` returns 36 files: 5/18/13/0), then amended ADR-0018 clause (iv) in place per
  D-00f — dated amendment, original clause text retained verbatim above the amendment, narrowing
  the claim rather than reversing the ADR's policy conclusion.
- **Files modified:** `.planning/decisions/0018-m6-facade-reexport-policy.md`
- **Verification:** `grep -c '^## '` still returns 7 (no heading added or removed);
  `grep -c 'conforms\|must change'` still returns 1 (conformance verdict intact); the amendment is
  additive text under the existing `## Decision` heading, not a rewrite.
- **Committed in:** single close-out commit (see hash below), alongside this plan's five planned
  files — the plan's own instruction to commit "this plan's five files in a single commit" is
  extended to six files to include this orchestrator-directed addition, documented here rather
  than silently expanding `files_modified`.

---

**Total deviations:** 1 auto-fixed (Rule 2 — missing critical functionality, orchestrator-directed)
**Impact on plan:** The ADR-0018 amendment is additive and scoped to one clause; it does not touch
any of the plan's own five files' required content, and every one of Task 1-3's own acceptance
criteria still passes unchanged. No scope creep beyond the orchestrator's explicit instruction.

## Issues Encountered

- Writing "PENDING-VERDICT" as a literal string inside the ledger's own `## Summary` prose (to
  state that the count is zero) tripped the task's own `grep -c 'PENDING-VERDICT'` verify check,
  since the check counts raw string occurrences regardless of context. Reworded the sentence to
  avoid the literal token while preserving the same claim (rows are unverdicted-token-free); the
  check now correctly returns 0.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- **Phase 8** can now plan DEBT-05 (ADR-0016 names the canonical `TokenUsage`) and its
  CLI-isolation requirement (ADR-0019's `structopt`/`paladin-herald` re-scoping, both named in the
  ledger's Forward scope section and in this plan's PROJECT.md rows).
- **Phase 11 / FACADE-02 D1** can apply ADR-0018's no-shim posture directly; its clause (iv) now
  accurately describes what Milestone 6 did and did not remove.
- **Phase 15** has ADR-0015's `cargo tree`-based allowlist-enforcement candidate to build against.
- **Phase 16** has ADR-0019's user-facing binary-architecture mdbook page to write.
- **Phase 7 is closed.** All 115 ledger rows carry a verdict with a counted, published distribution;
  all eight ADRs (0014-0021) exist with conformance verdicts and the required seven-heading shape;
  `PROMOTION.md` is consistent with the eight files on disk; `PROJECT.md` records all eight;
  `COVERAGE.md` exists; and the whole-phase `git diff --stat 2835b1a..HEAD -- '*.rs' 'Cargo.toml'
  '.github/'` is empty, proving the records-only boundary held for every one of this phase's 13
  plans.
- **Two flagged planner assumptions, carried forward for the phase verifier per this plan's own
  `<output>` instruction:**
  - **ARCH-05 (plan 07-09):** the plan executed on the assumption that ARCH-05's five positions are
    exactly five, closed, and each settled by the shipped tree with no competing defensible
    position — so a source correction plus a `diverged` ledger row is the whole remedy and no ADR
    was warranted for any of them. If a sixth literal-application hazard exists in the Milestone 4-6
    corpus that this enumeration misses, it is not corrected by this phase.
  - **ARCH-07 (plan 07-07):** `07-CONTEXT.md` D-25a's research described `src/application/mod.rs:59`
    (`pub mod cli;`) as un-gated; plan 07-07 re-grepped the live tree and found that claim stale —
    the declaration is `#[cfg(feature = "cli")]`-gated, as is its `src/lib.rs` re-export.
    ADR-0021 records the corrected fact rather than repeating the stale premise. This does not
    weaken the `structopt`/`src/main.rs` and `paladin-herald` findings, which stand independently
    verified.

---
*Phase: 07-workspace-ground-truth-recorded-answers*
*Completed: 2026-08-06*
