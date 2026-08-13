---
phase: 15-coverage-ci-quality-gates
plan: 04
subsystem: testing
tags: [coverage, cargo-llvm-cov, adr-0006, ci, documentation]

# Dependency graph
requires:
  - phase: 15-coverage-ci-quality-gates (plan 15-03)
    provides: "the `coverage` CI job with `--fail-under-lines 82` armed, and ADR-0006's Phase 15 amendment recording the 82% floor"
provides:
  - "a rewritten Code Coverage section in docs/src/contributing/testing-guide.md a contributor can follow end-to-end to reproduce the CI coverage number"
  - "CLAUDE.md, .github/copilot-instructions.md and .planning/codebase/TESTING.md corrected to cite ADR-0006's single 82% floor instead of the rejected 80%/70% two-number position"
affects: [16-docs-update]

# Tech tracking
tech-stack:
  added: []
  patterns:
    - "D-00c/D-00d annotation: coverage-claim corrections in source documents are dated correction notes citing ADR-0006, not silent rewrites"

key-files:
  created: []
  modified:
    - docs/src/contributing/testing-guide.md
    - CLAUDE.md
    - .github/copilot-instructions.md
    - .planning/codebase/TESTING.md

key-decisions:
  - "The retained-original-text requirement (D-00c/D-00d) is satisfied via prose description of the superseded 80%/70% claim rather than literal reproduction, because the plan's own acceptance grep (`Unit tests?:? *(≥|>=) *80|Integration tests?:? *(≥|>=) *70` must return 0 across all three files) would otherwise be tripped by a literal quote or strikethrough of the original text — the two requirements are only jointly satisfiable if the retained text does not reproduce the exact matched shape."
  - "development-setup.md was left untouched despite carrying the same stale 80%/70% coverage claims (lines 201-202) — out of this plan's files_modified scope (D-14's scope is coverage-number claims in the three named instruction files plus the testing guide only); flagged as a Known Stub / deferred item for DOCS-01 (Phase 16)."
  - "mdbook build docs fails in this environment on a pre-existing, unrelated defect (docs/mermaid.min.js is never committed to the repo, so book.toml's additional-js reference cannot resolve) — not caused by this plan's edits. Fell back to the acceptance criterion's alternate check (balanced code fences), which passes."

requirements-completed: [PIPE-03, PIPE-05]  # Marked complete on checkpoint resolution — see "Checkpoint Resolution" below. PIPE-03's local-runnability half and PIPE-05's local-reproduction half are deferred, not verified; tracked in .planning/todos/pending/2026-08-13-verify-local-coverage-reproduction.md per the user's recorded decision.

coverage:
  - id: D1
    description: "Rewritten Code Coverage section in testing-guide.md (prerequisites, local generation, scope, threshold policy, reading output, Codecov behaviour, troubleshooting) reproducing the CI coverage number's derivation"
    requirement: "PIPE-05"
    verification:
      - kind: other
        ref: "plan 15-04-PLAN.md Task 1 automated <verify> scripts (floor-match, no-rejected-claim grep, ToC-sync) — all three run and passed during execution"
        status: pass
    human_judgment: false
  - id: D2
    description: "CLAUDE.md, .github/copilot-instructions.md and .planning/codebase/TESTING.md corrected to state ADR-0006's single 82% floor, citing ADR-0006 and cargo-llvm-cov as tool of record, with the rejected two-number claim removed"
    requirement: "PIPE-05"
    verification:
      - kind: other
        ref: "plan 15-04-PLAN.md Task 2 automated <verify> scripts (ADR-0006/floor/tool citation check, rejected-claim grep, enforced-in-CI truth check) — all three run and passed during execution"
        status: pass
    human_judgment: false
  - id: D3
    description: "The documented 82% threshold and its floor arithmetic (truncate toward zero, at-or-above) are confirmed against a live CI coverage measurement, and the `coverage`/`cli-tests`/`bench-check`/unit-test CI jobs pass"
    requirement: "PIPE-03, PIPE-05"
    verification:
      - kind: other
        ref: "GitHub Actions run 31727496744 (ci.yml), commit e9e3267f9ae6d8483be3ee52c04ffe6a763cbb37 — job Coverage: Lines 39233/47618 = 82.39% (truncates to 82, passes --fail-under-lines 82); job CLI Snapshot Tests: success; job Benchmark Compile Check: success; job Unit Tests (stable): success"
        status: pass
    human_judgment: false
  - id: D4
    description: "A developer reproduces the CI coverage number locally via the documented procedure end-to-end, on a machine with Docker, confirming no undocumented prerequisite step exists"
    requirement: "PIPE-03, PIPE-05"
    verification: []
    human_judgment: true
    rationale: "No authoring environment in Phase 15 has Docker or cargo-llvm-cov installed, so the local `make services-up` / `make coverage` walkthrough could not run here. Deferred per the user's recorded decision at the Task 3 checkpoint (\"Accept, track local check\"); tracked in .planning/todos/pending/2026-08-13-verify-local-coverage-reproduction.md, owner: repo maintainer, no resolves_phase tag so it outlives Phase 15."

duration: ~35min (Tasks 1-2); Task 3 resolved by deferral, not executed locally
completed: 2026-08-13
status: complete
---

# Phase 15 Plan 04: Coverage Documentation and Instruction-File Correction Summary

**Rewrote the contributor Code Coverage section to reproduce ADR-0006's 82% CI figure, and corrected the rejected 80%/70% coverage claim in CLAUDE.md, copilot-instructions.md and TESTING.md — Task 3's local-reproduction checkpoint resolved by user-accepted deferral, tracked as a todo.**

## Performance

- **Duration:** ~35 min for Tasks 1-2, plus checkpoint resolution
- **Started:** 2026-08-13
- **Tasks:** 3 of 3 resolved (Task 3's local Docker walkthrough deferred by recorded user decision, not executed)
- **Files modified:** 4 (Tasks 1-2) + 1 todo created + REQUIREMENTS.md

## Accomplishments

- `docs/src/contributing/testing-guide.md`'s `## Test Coverage` section rewritten end-to-end: prerequisites (`llvm-tools-preview`, `cargo-llvm-cov --locked`, `cargo binstall` pointer), the two-step local procedure (`make services-up` → `make coverage` / `make coverage-html`) with the full underlying `cargo llvm-cov` invocation shown, the `--features integration-tests` scope rationale (not `--all-features`; the two `cli`-gated binaries excluded by construction), the threshold policy (82% floor, truncate-toward-zero, at-or-above comparison, ratchet-only-rises), how to read region/function/line output with an explicit "only line is gated" warning, Codecov's non-gating behaviour and why (`CODECOV_TOKEN` absence risk), and a four-mode troubleshooting subsection.
- The `### Coverage Requirements` subsection under `## Testing Philosophy` now states ADR-0006's single 82% floor instead of the separate ≥90%/≥80% unit and ≥80%/≥70% integration targets.
- Table of Contents brought into sync with all `## ` headings in the file (three pre-existing headings — Quick Reference, Testing Best Practices, Next Steps — were already missing from the ToC before this plan; fixed as required by the task's own verify script).
- `CLAUDE.md`'s TDD working-agreement bullet, `.github/copilot-instructions.md`'s Coverage Requirements subsection, and `.planning/codebase/TESTING.md`'s Coverage section all replaced the rejected 80%/70% two-number position with ADR-0006's single 82% floor, citing ADR-0006 and `cargo-llvm-cov` as tool of record.
- `TESTING.md` additionally: `cargo tarpaulin` retained only as a labelled non-comparable informal alternative (ptrace vs. LLVM instrumentation), and the "Enforced in CI pipeline" claim made precise (names the `coverage` job and `--fail-under-lines`).

## Task Commits

1. **Task 1: The Code Coverage section in the contributor testing guide** - `b6baf96` (docs)
2. **Task 2: Correct the rejected coverage figure in the three instruction files** - `b7c74b2` (docs)
3. **Task 3: Confirm a developer can reproduce the CI number locally** - RESOLVED BY DEFERRAL (see "Checkpoint Resolution" below); todo `2026-08-13-verify-local-coverage-reproduction.md` created and committed

**Plan metadata:** this SUMMARY commit (below)

## Checkpoint Resolution

**Task 3 (`checkpoint:human-verify`, `gate="blocking"`) was resolved via the plan's own sanctioned
second path — "recorded as an open item with an owner" — rather than by a local walkthrough.**

**Decision and provenance.** The orchestrator presented the checkpoint to the human user via an
interactive `AskUserQuestion` prompt. The user selected, verbatim, **"Accept, track local check."**
This is a recorded human decision relayed by the orchestrator — not an agent-invented approval and
not an auto-approval (auto-mode was off for the original checkpoint). The local-reproduction step
was deferred rather than performed, by the user's explicit choice.

**What IS confirmed (from CI, gathered by the orchestrator, not re-run here):** GitHub Actions run
`31727496744` (workflow `ci.yml`), on commit `e9e3267f9ae6d8483be3ee52c04ffe6a763cbb37` — the exact
base this plan's worktree branched from:
- `Scope: --workspace --features integration-tests (the gated measurement)`
- `Lines:     39233/47618 = 82.39%`
- `Functions: 4604/6115 = 75.29%`
- Floor arithmetic per ADR-0006: 82.39% truncates toward zero to 82; comparison is at-or-above;
  `--fail-under-lines 82` → **gate passes**.
- Job `Coverage` → success
- Job `CLI Snapshot Tests` (equivalent to `make test-cli`) → success
- Job `Benchmark Compile Check` (equivalent to `make bench-check`) → success
- Job `Unit Tests (stable)` (the `ci-test` half of `make ci-full`) → success

Two jobs in that same run failed, both outside this plan's scope: `License & Dependency Policy` and
`Example Muster (Feature Matrix)`. The `Example Muster` `--offline` breakage is already known and
scoped to Phase 15.1. Observed, not investigated further here.

**What is NOT confirmed:** the local `make coverage` figure, and the human walkthrough of
`docs/src/contributing/testing-guide.md`'s Code Coverage section end to end. This environment has
no Docker and no `cargo-llvm-cov` installed, so `make services-up` → `make coverage` could not run
here, and no authoring environment in Phase 15 has had either. This is the specific item deferred.

**Tracking.** `.planning/todos/pending/2026-08-13-verify-local-coverage-reproduction.md` records the
open item: walk the testing guide on a Docker-capable machine, run `make services-up` then
`make coverage`, confirm the local figure agrees with CI's 82.39% at whole-percent precision, and
fix any missing/wrong prerequisite step found. Owner: repo maintainer (the user). Deliberately
carries no `resolves_phase` tag, so it is not auto-closed when Phase 15 completes — it is expected
to outlive this phase.

**Requirements.** PIPE-03 and PIPE-05 are marked complete in `REQUIREMENTS.md` on the strength of
the CI evidence above plus the user's accept-and-track decision — their local-runnability /
local-reproduction halves are deferred, not verified, and remain tracked by the todo.

## Files Created/Modified

- `docs/src/contributing/testing-guide.md` - Rewritten `## Test Coverage` section (prerequisites, local generation, scope, threshold policy, reading output, Codecov behaviour, troubleshooting); corrected `### Coverage Requirements`; synced Table of Contents
- `CLAUDE.md` - TDD working-agreement bullet now cites ADR-0006's 82% floor instead of unit≥80%/integration≥70%
- `.github/copilot-instructions.md` - Coverage Requirements subsection corrected with a dated D-00c note and the single 82% floor
- `.planning/codebase/TESTING.md` - Coverage section corrected with a dated D-00d note (ledger-class annotation), tarpaulin relabelled as a non-comparable informal alternative, "Enforced in CI pipeline" made precise
- `.planning/todos/pending/2026-08-13-verify-local-coverage-reproduction.md` - New: tracks the deferred local-reproduction walkthrough, owner repo maintainer, no `resolves_phase` tag
- `.planning/REQUIREMENTS.md` - PIPE-03 and PIPE-05 checkboxes and traceability rows marked complete

## Decisions Made

- **Prose annotation over literal-text retention.** D-00c/D-00d instruct retaining the original superseded text, marked. The plan's own acceptance grep (`grep -cE 'Unit tests?:? *(≥|>=) *80|Integration tests?:? *(≥|>=) *70'` must return 0 across all three files) would be tripped by a literal quote or `~~strikethrough~~` of the original "Unit tests: ≥ 80% coverage" / "Integration tests: ≥ 70% coverage" lines, since markdown strikethrough markers don't remove the matched substring. Resolved by describing the superseded claim in prose (spelled-out numbers, no `≥`/`>=` symbol adjacent to "Unit test(s)"/"Integration test(s)") — the audit trail is preserved in substance (what was wrong, what it said, why it was wrong, pointer to ADR-0006) without reproducing the exact machine-matched shape.
- **`development-setup.md` left untouched.** It carries the same stale 80%/70% coverage claims (lines 201-202) but is not in this plan's `files_modified` list, and D-14's scope is explicitly the three named instruction files plus the testing guide — general content currency is DOCS-01 in Phase 16. Recorded as a Known Stub below.
- **`mdbook build docs` acceptance fallback used.** The build fails in this environment on a pre-existing, unrelated defect: `docs/mermaid.min.js` referenced by `book.toml`'s `additional-js` is never committed to the repository (confirmed via `git log` — zero history for that path), so `mdbook build` cannot copy it regardless of markdown content. This predates and is unrelated to this plan's edits. Used the acceptance criterion's documented fallback instead — `docs/src/contributing/testing-guide.md`'s code fences are balanced (verified: even count of `` ``` `` markers).

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 1 - Bug] Table of Contents was missing three pre-existing headings**
- **Found during:** Task 1 verification (the plan's own ToC-sync `<automated>` script)
- **Issue:** `docs/src/contributing/testing-guide.md`'s `## Table of Contents` list was already missing "Quick Reference: Test Commands", "Testing Best Practices" and "Next Steps" before this plan touched the file — a pre-existing defect, not introduced by this plan's edits, but caught by Task 1's own verify script which checks every `## ` heading in the (now-modified) file against the ToC.
- **Fix:** Added the three missing entries to the Table of Contents list.
- **Files modified:** `docs/src/contributing/testing-guide.md`
- **Verification:** Re-ran the plan's ToC-sync verify script; passed ("toc in sync").
- **Committed in:** `b6baf96` (Task 1 commit)

**2. [Rule 1 - Bug] Strikethrough annotation in copilot-instructions.md still matched the rejected-claim grep**
- **Found during:** Task 2, self-review before commit
- **Issue:** An initial draft retained the original "Unit tests: ≥ 80% coverage" / "Integration tests: ≥ 70% coverage" lines wrapped in `~~strikethrough~~` markers to satisfy D-00c's "retain original text" instruction. `grep -cE` treats strikethrough markers as ordinary characters, so the literal matched substring survived and the required "returns 0" acceptance check would fail.
- **Fix:** Replaced the strikethrough retention with a prose correction note (see "Decisions Made" above) that conveys the same audit-trail information without reproducing the exact matched pattern.
- **Files modified:** `.github/copilot-instructions.md`
- **Verification:** Re-ran the task's rejected-claim grep across all three files; all returned 0.
- **Committed in:** `b7c74b2` (Task 2 commit)

---

**Total deviations:** 2 auto-fixed (2 bugs, both caught by the plan's own verify scripts before commit)
**Impact on plan:** Both were required to satisfy this plan's own stated acceptance criteria. No scope creep — both fixes stayed inside the already-in-scope file (`testing-guide.md`) or the already-in-scope subsection (`copilot-instructions.md`'s Coverage Requirements).

## Issues Encountered

- `mdbook build docs` fails on a pre-existing, unrelated missing-asset defect (`docs/mermaid.min.js` never committed). Not fixed here — out of this plan's coverage-claim scope, and the acceptance criterion provides an explicit fallback (balanced code fences) which was used instead. Recorded for awareness in case Phase 16 (DOCS-01) needs it.

## Known Stubs

- `docs/src/contributing/development-setup.md:201-202` still states "Unit tests: ≥ 80% coverage for new code" / "Integration tests: ≥ 70% coverage for public APIs" — the same rejected ADR-0006 position 1 claim this plan corrected elsewhere, but this file is outside this plan's `files_modified` scope (D-14 scopes to CLAUDE.md, copilot-instructions.md, TESTING.md and testing-guide.md only). Owner: DOCS-01, Phase 16.

## User Setup Required

None - no external service configuration required. The deferred local-verification item
(`.planning/todos/pending/2026-08-13-verify-local-coverage-reproduction.md`) requires a
Docker-capable machine, but that is tracked follow-up work, not setup blocking this plan.

## Next Phase Readiness

**This plan is complete.** Tasks 1 and 2 are committed and pass every `<automated>` verify script
and acceptance criterion. Task 3 is resolved via the plan's own sanctioned "recorded as an open
item with an owner" path, per the user's recorded "Accept, track local check" decision — see
"Checkpoint Resolution" above. PIPE-03 and PIPE-05 are marked complete in `REQUIREMENTS.md`.

The one open follow-up is the deferred local walkthrough tracked in
`.planning/todos/pending/2026-08-13-verify-local-coverage-reproduction.md` — not a blocker for
this plan or Phase 15, but real work a repo maintainer should still do on a Docker-capable machine
to close the loop the checkpoint originally asked for.

---
*Phase: 15-coverage-ci-quality-gates*
*Completed: 2026-08-13 (Task 3 resolved by recorded user decision to defer local verification)*
