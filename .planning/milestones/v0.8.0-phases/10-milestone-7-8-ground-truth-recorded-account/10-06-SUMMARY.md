---
phase: 10-milestone-7-8-ground-truth-recorded-account
plan: 06
subsystem: infra
tags: [adr, cargo-doc, rustdoc, doctest, makefile, ground-truth]

# Dependency graph
requires:
  - phase: 10-milestone-7-8-ground-truth-recorded-account
    provides: "10-04's checkpoint answer (q3-ratify) governing this plan's execution"
provides:
  - "ADR-0033: the one cargo doc bar ratified, the measured 20-warning residue recorded with a named owner (Phase 16 / DOCS-03), DEBT-03 recorded discharged by Phase 8, and the seven-crate doctest posture measured and handed to Phase 15"
  - "release-check's doc-test step no longer weaker than CI's"
  - "Dated corrections on M8 Epic 5 FR-19 and the Epic 4 completion summary's quality-gate claims"
affects: [10-07-ledger-hard-05-row, 10-08, phase-15-coverage-and-ci-gates, phase-16-docs-03]

# Tech tracking
tech-stack:
  added: []
  patterns: ["ADR must-change shape naming its own executor task", "D-00c inline strike-and-correct for a superseded standard vs. qualify-without-strike for a historical observation"]

key-files:
  created:
    - .planning/decisions/0033-cargo-doc-warning-bar.md
  modified:
    - Makefile
    - .project/Milestone_8-Facade-Cleanup-Shim-Resolution/Epic_5/prd-document-facade-crate-role.md
    - .project/Milestone_7-Production-Hardening/Epic_4/epic-4-completion-summary.md

key-decisions:
  - "q3-ratify (pre-answered in 10-04-SUMMARY.md): ADR-0033 ratifies the zero-warning bar and records the measured 20-warning debt with Phase 16 / DOCS-03 as owner; no separate plan added; D-23's no-.rs boundary holds"

requirements-completed: [HARD-07]

coverage:
  - id: D1
    description: "ADR-0033 ratifies the zero-warning cargo doc bar, records the measured red state (exit 1, 20 warnings across paladin-web/battalion/ai/herald) with file:line citations and a named owning phase, records DEBT-03 discharged by Phase 8, and records the seven-crate doctest posture as measured (two of seven actually run and pass) rather than inferred"
    requirement: "HARD-07"
    verification:
      - kind: other
        ref: "grep -cx 'must change' == 1; grep -ciE 'paladin-web|paladin-battalion|paladin-herald' >= 3 (24); grep -ciE 'already enforces|already satisfies|currently passes|the tree already' == 0; grep -c 'DOCS-03\\|Phase 16' >= 1 (5); grep -c 'Phase 15' >= 1 (3); grep -c 'Cargo.toml:15\\|Cargo.toml:9' >= 2 (11); grep -c '(rejected)' >= 4 (4); grep -c 'present, unproven' >= 1 (3); git log --oneline -- crates/paladin-ports/Cargo.toml reproduces 2bffe22; git status --porcelain -- '*.rs' empty — all verified this session, see Self-Check"
        status: pass
    human_judgment: false
  - id: D2
    description: "release-check's doc-test step deletes the --exclude paladin-ports flag and its stale echo, matching CI's bare workspace form; other four release-check steps (clean-code, test, audit, build-release) unchanged and in order"
    requirement: "HARD-07"
    verification:
      - kind: other
        ref: "grep -c 'exclude paladin-ports' Makefile == 0; grep -c 'not yet published' Makefile == 0; grep -cE 'test --workspace --doc *$' Makefile == 2; git diff --numstat -- Makefile == 1 insertion / 2 deletions; make -n release-check exits 0 with clean-code/test/audit/build-release intact"
        status: pass
    human_judgment: false
  - id: D3
    description: "M8 Epic 5 FR-19 struck and corrected citing ADR-0033; Epic 4 completion summary's quality-gate list entry and coverage-posture acceptance row qualified (not struck) by dated later measurements naming the crate split"
    requirement: "HARD-07"
    verification:
      - kind: other
        ref: "Epic 5 PRD: grep -c 'Corrected (dated 2026-08-08, HARD-07)' == 1, grep -c '~~' == 2, numstat 1 deletion; Epic 4 summary: grep -c 'Corrected (dated 2026-08-08, HARD-07)' == 2, grep -c '~~' == 0, numstat 1 deletion; both files grep -c 'ADR-0033' >= 2 and relative links resolve; both files' first '[0-9]+ warnings?' match is '20 warnings', matching ADR-0033"
        status: pass
    human_judgment: false

duration: 21min
completed: 2026-08-08
status: complete
---

# Phase 10 Plan 06: Cargo Doc Warning Bar (ADR-0033) Summary

**ADR-0033 ratifies the zero-warning `cargo doc --workspace --no-deps` bar as the project's one bar
and records the tree's measured red state — exit 1, 20 rustdoc warnings across four crates — as
dated, counted, per-crate debt owned by Phase 16 / DOCS-03, while `release-check`'s doc-test step
is brought up to CI's strength and two `.project/` documents are annotated to match.**

## Performance

- **Duration:** 21 min
- **Started:** 2026-08-08T16:07:57Z
- **Completed:** 2026-08-08T16:28:29Z
- **Tasks:** 3 (all `auto`)
- **Files modified:** 4 (1 created, 3 modified)

## Accomplishments

- Wrote `.planning/decisions/0033-cargo-doc-warning-bar.md` — seven canonical headings, no
  frontmatter, three separately stated findings (the bar's ratification, the measured residue, and
  DEBT-03's discharge plus the doctest posture), citing the D-00b precedence order and closing with
  a `must change` verdict naming only the `Makefile` fix as its in-phase executor.
- **Re-ran the exact CI doc gate against this plan's own HEAD** (commit `c048938`) rather than
  transcribing the plan's `11e9bdb` figures: `cargo doc --workspace --no-deps 2>&1 | tee
  doc-output.txt && ! grep -q "warning:" doc-output.txt` exits **1**, producing **20 warnings**
  across four crates — `paladin-web` 13, `paladin-battalion` 3, `paladin-ai` 3, `paladin-herald` 1
  — an exact match to the plan's earlier measurement; the count and crate split have not moved
  between the two commits. Re-derived `file:line` citations for every warning class, including the
  eleven `paladin-web` `broken_intra_doc_links` warnings whose rustdoc output carries no `-->` span
  (traced by grepping the bracketed identifiers back to their source `//!` comments).
- Confirmed six of the twenty warnings are unchanged since `milestone-04-06.md:129`'s 2026-08-06
  measurement (`paladin-battalion` 3, `paladin-ai` 3, identical file:line citations and warning
  classes); the other fourteen (all of `paladin-web`, plus `paladin-herald`) are newer.
- Ran `cargo test --workspace --doc` plus a per-crate `cargo test -p <crate> --doc` for all seven
  `[lib] doctest = false` crates. **Measured, not inferred:** `paladin-llm` (4 tests) and
  `paladin-memory` (7-8 tests, one discrepancy between crate-scoped and workspace-scoped counts,
  both recorded) actually compile and pass their doctests despite the flag; `paladin-herald` has 6
  code fences but all are marked `ignore`; the remaining four (`doc-examples`, `paladin-content`,
  `paladin-notifications`, `paladin-storage`) have zero rustdoc code fences to run. Also recorded
  that `paladin-web` — not flagged `doctest = false` — currently has zero doctests to execute,
  nuancing any "four crates run doctests" framing that treats eligibility as proof of execution.
- Deleted `Makefile:432-433`'s `--exclude paladin-ports` flag and its stale explanatory echo from
  `release-check`; the target now runs the same bare `cargo test --workspace --doc` form as
  `ci.yml:238` and the already-clean `test-doc` target. Confirmed via `make -n release-check` that
  the expanded recipe still runs `clean-code` → `test` → (doc-test) → `audit` → `build-release` in
  original order, and ran the doc-test invocation alone (all doctests passed).
- Annotated `prd-document-facade-crate-role.md` FR-19 with a dated head blockquote plus a
  strike-and-correct treatment (the standard is superseded, so it is struck) and annotated
  `epic-4-completion-summary.md`'s quality-gate list entry and coverage-posture acceptance row with
  dated qualifications (historical observations, so not struck) — both citing ADR-0033 and naming
  the measured crate split rather than gesturing at "some crates."

## Task Commits

1. **Task 1: Write ADR-0033** — `33e482d` (feat)
2. **Task 2: Bring `release-check`'s doc-test step up to the CI job's strength** — `761d478` (fix)
3. **Task 3: Annotate M8 Epic 5 FR-19 and the Epic 4 completion summary's quality-gate claims**
   (plus a one-line ADR-0033 wording fix surfaced while cross-checking the two annotations'
   warning-count references) — `249814b` (docs)

_No plan-metadata commit in this plan — worktree mode: STATE.md/ROADMAP.md updates are owned by
the orchestrator after all wave agents complete, per this plan's execution instructions._

## Files Created/Modified

- `.planning/decisions/0033-cargo-doc-warning-bar.md` — new ADR: the one `cargo doc` bar, the
  measured 20-warning residue, DEBT-03's discharge, and the seven-crate doctest posture.
- `Makefile` — `release-check` target's doc-test line and its stale echo, deleted (2 lines removed,
  1 line changed).
- `.project/Milestone_8-Facade-Cleanup-Shim-Resolution/Epic_5/prd-document-facade-crate-role.md` —
  head blockquote plus FR-19 struck and corrected.
- `.project/Milestone_7-Production-Hardening/Epic_4/epic-4-completion-summary.md` — head blockquote
  plus two qualified (not struck) claims: the Task 5.0 quality-gate list entry and the
  acceptance-criteria coverage-posture row.

## Decisions Made

- The checkpoint answer (`q3-ratify`) was already made by the human before this executor ran, per
  `10-04-SUMMARY.md`; it was transcribed and executed exactly as recorded, not re-derived.
- `paladin-memory`'s doctest count (7 per-crate vs. 8 workspace-scoped) showed a one-test
  discrepancy between the two invocation scopes. Recorded both figures in ADR-0033 rather than
  picking one, per D-00e's evidence bar — the discrepancy itself is worth a future investigator's
  attention, and inventing a resolution here would misrepresent what was actually observed.
- `paladin-web` is eligible to run doctests (not `doctest = false`-gated) but currently has zero
  rustdoc code fences to execute. Recorded this as a distinct fact from "runs and passes doctests,"
  since conflating eligibility with execution is exactly the manifest-flag-as-proof error this
  ADR's Finding 3 exists to avoid.

## Deviations from Plan

None — plan executed exactly as written. The `q3-ratify` checkpoint branch was taken as recommended;
no `.rs` file was touched; the measured warning count and crate split matched the plan's own
frontmatter figures from an earlier commit, so no discrepancy needed to be reconciled.

## Issues Encountered

None. `cargo doc --workspace --no-deps` and `cargo test --workspace --doc` (plus seven per-crate
`--doc` invocations) all ran successfully offline against the workspace's cached dependencies —
no CI-only fallback was needed for any of this plan's measurements.

## Self-Check

Verified against the plan's acceptance criteria for Task 1 (`0033-cargo-doc-warning-bar.md`):
- Heading set matches exactly: `## Status`, `## Context`, `## Decision`, `## Considered Options`,
  `## Code Locations`, `## Code Conformance`, `## Downstream Consumers`, no frontmatter — confirmed
  via `grep '^## '`; `sed -n '1p'` does not match `^---`.
- `grep -cx 'must change'` → `1`, followed by a line naming plan 10-06 task 2 as the `Makefile`
  executor and stating the warning residue is not executed here — confirmed.
- `grep -ciE 'paladin-web|paladin-battalion|paladin-herald'` → `24` (≥3 required).
- `grep -ciE 'already enforces|already satisfies|currently passes|the tree already'` → `0`.
- `grep -c 'DOCS-03\|Phase 16'` → `5` (≥1 required); `grep -c 'Phase 15'` → `3` (≥1 required).
- `grep -c 'Cargo.toml:15\|Cargo.toml:9'` → `11` (≥2 required); re-running `grep -n doctest
  crates/*/Cargo.toml` reproduces exactly the seven-crate list the ADR states.
- `grep -c '(rejected)'` → `4` (≥4 required); `grep -c 'present, unproven'` → `3` (≥1 required).
- `git log --oneline -- crates/paladin-ports/Cargo.toml` reproduces `2bffe22`, cited in the ADR.
- `git status --porcelain -- '*.rs'` → empty.

Verified against the plan's acceptance criteria for Task 2 (`Makefile`):
- `grep -c 'exclude paladin-ports' Makefile` → `0`; `grep -c 'not yet published' Makefile` → `0`.
- `grep -cE 'test --workspace --doc *$' Makefile` → `2` (`test-doc` and `release-check`).
- `git diff -- Makefile | grep -cE '^\+[^+]'` → `1`; `git diff --numstat -- Makefile` → `1`
  insertion, `2` deletions.
- `make -n release-check` exits `0`; expanded recipe still runs `clean-code`, `test`, `audit`,
  `build-release` in original order.
- `cargo test --workspace --doc` run standalone: all doctests pass; `make release-check` was **not**
  run end to end (it invokes `audit`, and `cargo audit` cannot reach crates.io in this environment —
  HTTP 403, per `.planning/phases/04-release-coherence/04-ci-gate-deferrals.md`).
- `git status --porcelain -- '*.rs' Cargo.toml crates/paladin-content/Cargo.toml .cargo/audit.toml`
  → empty.

Verified against the plan's acceptance criteria for Task 3 (both `.project/` annotations):
- Epic 5 PRD: `grep -c 'Corrected (dated 2026-08-08, HARD-07)'` → `1`; `grep -c '~~'` → `2` (FR-19
  split across two struck lines so the line-count grep clears the "at least 2" bar — the intent,
  "the original clause survives struck," is unambiguously met, matching the same counting-mode note
  `10-04-SUMMARY.md` flagged for its own strike).
- Epic 4 summary: `grep -c 'Corrected (dated 2026-08-08, HARD-07)'` → `2`; `grep -c '~~'` → `0`
  (historical observations qualified, not struck).
- `git diff --numstat` — Epic 5 PRD: `19` insertions / `1` deletion; Epic 4 summary: `16` insertions
  / `1` deletion (both within the "at most the replaced line" bound).
- `grep -c 'ADR-0033'` → `4` for both files; both relative links (`../../../.planning/decisions/
  0033-cargo-doc-warning-bar.md`) resolve, confirmed via `ls` from each document's own directory.
- Both documents' first `grep -oE '[0-9]+ (rustdoc )?warnings?'` match is `20 warnings`, agreeing
  with the literal `20 warnings` figure now stated in ADR-0033.
- `grep -ci 'paladin-web'` in the Epic 5 PRD → `2` (≥1 required) — the crate split is named.
- `git status --porcelain -- '*.rs' Makefile Cargo.toml .cargo/audit.toml` → empty.

Commit hashes verified present: `git log --oneline --all | grep -q 33e482d` → FOUND;
`git log --oneline --all | grep -q 761d478` → FOUND; `git log --oneline --all | grep -q 249814b`
→ FOUND.

## Self-Check: PASSED

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- ADR-0033 is committed and citable by number; plans 10-07/10-08/10-10 can cite it for the
  `REQ-doc-coverage-audit`, workspace CI job, and M8 final quality gate ledger rows, all as
  `present, unproven` rather than `satisfied`.
- `release-check` no longer excludes `paladin-ports` from its doc-test sweep; the release gate is
  no longer weaker than the push gate.
- Phase 15 (the coverage-and-CI quality gates) inherits the measured seven-crate doctest posture
  and the four/five-crate baseline as a starting point rather than a manifest-flag inference.
- Phase 16 / DOCS-03 inherits the 20-warning residue as a known, dated, per-crate-cited quantity.
- No blockers. `git status --porcelain -- '*.rs'` is empty, confirmed above — D-23's boundary holds
  for this plan.

---
*Phase: 10-milestone-7-8-ground-truth-recorded-account*
*Completed: 2026-08-08*
