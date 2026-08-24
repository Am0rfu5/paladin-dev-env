---
phase: 13-milestone-9-12-ground-truth-recorded-account
plan: 03
subsystem: docs
tags: [ledger, requirements-traceability, ci-hardening, release-automation, supply-chain, branch-protection]

# Dependency graph
requires:
  - phase: 13-milestone-9-12-ground-truth-recorded-account
    provides: 13-01's ledger scaffold (.planning/ledgers/milestone-09-12.md), contention table fixing this plan's section range to Milestone 10 Epics 1-5
  - phase: 12-supply-chain-gate-integrity
    provides: SUPPLY-01/02/03 closures, the both-halves verdict class definition, the D-00i --auto provenance convention
provides:
  - Milestone 10 Epics 1-5's 23 requirement rows fully derived to the D-00e evidence bar (real file:line + named-consumer citations, gh api/gh run commands actually run this session)
  - The corpus's only Shipped, one acceptance criterion false row (REQ-audit-toml-single-source), both halves dated, with SUPPLY-01's previously-pending CI-run trigger clause resolved via a live gh run citation
  - REQ-advisory-exception-process closed via SUPPLY-02 with D-00i --auto provenance (Phase 9 D-07 unratified; Phase 12 D-01/D-08 ratified at plan 12-01's 2026-08-09 checkpoint) recorded in the Epic 2 header note
  - REQ-github-rulesets recorded owner-only with a live gh api confirmation that main is unprotected and no ruleset is applied
affects: [13-13]

# Tech tracking
tech-stack:
  added: []
  patterns: [ledger row-replacement-in-place (Phase 13's own contention protocol), re-run-every-citation evidence bar (D-00e/D-03), live gh CLI verification for time-varying GitHub state]

key-files:
  created: []
  modified: [.planning/ledgers/milestone-09-12.md]

key-decisions:
  - "Every citation was re-run against the tree this session rather than trusting the inherited transcription — this caught five stale line-number citations (Makefile:282/261/264/439/424/413, all drifted since the source ledger was written) and corrected them in place"
  - "REQ-audit-toml-single-source carries both halves dated (D-05), plus a new addition beyond the plan's inherited highlight table: the live gh run citation resolving SUPPLY-01's previously-pending 'first real CI run after the deletion' trigger clause (run 31320378772, 2026-08-09, success), superseding the stale 30861568499 reference"
  - "REQ-advisory-exception-process is recorded as closed via SUPPLY-02 and cited per D-06/D-00i rather than re-verified from scratch, with the --auto provenance chain (Phase 9's D-07 re-scope, never ratified; Phase 12's D-01/D-08, ratified only at plan 12-01's blocking checkpoint on 2026-08-09) written into the Epic 2 section header rather than only in the commit message"
  - "REQ-github-rulesets and the SUPPLY-01 CI-run trigger were both verified live via gh CLI this session (gh run list, gh run view --json jobs, gh api repos/:owner/:repo/rulesets, gh api repos/:owner/:repo/branches/main/protection) rather than trusting the corpus's prior transcription of these time-varying facts"
  - "REQ-workspace-publish-order's paladin-herald omission (previously flagged only in the transcribed doc citation) was confirmed to also be present in the executable release.yml publish-crates CRATES array — recorded as a limitation of the Shipped verdict, not a downgrade, since the order itself is correct and internally consistent"

requirements-completed: [ORCH-01]

coverage:
  - id: D1
    description: "All 12 rows in Milestone 10 Epic 1 (pre-commit/pre-push hooks) and Epic 2 (dependency security & licence compliance) carry a cited, re-run verdict; REQ-audit-toml-single-source carries both halves dated with a live gh run citation resolving SUPPLY-01's pending trigger"
    requirement: "ORCH-01"
    verification:
      - kind: other
        ref: "awk '/^### Milestone 10 Epic 1/,/^### Milestone 10 Epic 3/' .planning/ledgers/milestone-09-12.md | grep -c '^| REQ-' -> 12; same range grep -c 'pending — plan' -> 0; grep -c 'run-5 input (not yet re-derived)' -> 0"
        status: pass
      - kind: other
        ref: "grep -c cb75b2b -> 5 (>=2 required); grep -c 2026-08-08 -> 3 (>=1); grep -c ADR-0036 -> 3 (>=1); grep -cE 'run [0-9]{8,}|31320378772' -> 1 (>=1, live gh run 31320378772); grep -c 12-01 -> 4 (>=1)"
        status: pass
      - kind: other
        ref: "gh run list --workflow=ci.yml --limit 5 --branch release/v0.7.0 (newest: 31320378772, 2026-08-09, success); gh run view 31320378772 --json jobs (Security Audit: success); gh api repos/:owner/:repo/rulesets ([]); gh api repos/:owner/:repo/branches/main/protection (404)"
        status: pass
    human_judgment: false
  - id: D2
    description: "All 11 rows in Milestone 10 Epics 3-5 (release automation, v0.4.0 release, tag-source enforcement) carry a cited, re-run verdict; the ruleset finding is recorded owner-only with nothing applied; ledger integrity preserved at exactly 120 rows"
    requirement: "ORCH-01"
    verification:
      - kind: other
        ref: "awk '/^### Milestone 10 Epic 3/,/^### Milestone 11 Epics 1-2/' .planning/ledgers/milestone-09-12.md | grep -c '^| REQ-' -> 11; same range grep -c 'pending — plan' -> 0; grep -c 'run-5 input (not yet re-derived)' -> 0"
        status: pass
      - kind: other
        ref: "grep -c rulesets -> 2 (>=1); grep -c REQ-lockstep-versioning -> 2 (>=1); grep -c '^| REQ-' .planning/ledgers/milestone-09-12.md -> 120"
        status: pass
      - kind: other
        ref: "git diff --numstat -- .planning/ledgers/milestone-09-12.md -> 14 insertions / 14 deletions (true balance; the plan's own awk one-liner compares deletions-count to the filename string and always prints 'unbalanced' regardless of content — see Deviations)"
        status: pass
    human_judgment: false
  - id: D3
    description: "Ledger-file contention respected: no row outside Milestone 10 Epics 1-5 touched, no row inserted/deleted/reordered, zero .rs files modified across the whole plan"
    requirement: "ORCH-01"
    verification:
      - kind: other
        ref: "git diff --name-only 668fc44113ac114c8361a83af85cb3f3695d5886..HEAD -- '*.rs' | wc -l -> 0; git status --short shows only .planning/ledgers/milestone-09-12.md modified"
        status: pass
    human_judgment: false

# Metrics
duration: ~90min
completed: 2026-08-10
status: complete
---

# Phase 13 Plan 03: Milestone 10 Epics 1-5 Ledger Derivation Summary

**Derived real, cited verdicts for all 23 requirement rows in Milestone 10 (CI hardening & release automation), wrote the corpus's only `Shipped, one acceptance criterion false` row with both halves dated and a live `gh run` citation resolving SUPPLY-01's last pending clause, and confirmed via live `gh api` calls that the committed GitHub rulesets remain unapplied and `main` unprotected.**

## Performance

- **Duration:** ~90 min
- **Started:** 2026-08-10 (session start)
- **Completed:** 2026-08-10T17:01:13Z
- **Tasks:** 2
- **Files modified:** 1 (`.planning/ledgers/milestone-09-12.md`)

## Accomplishments

- All 12 rows in Milestone 10 Epic 1 (Pre-Commit & Pre-Push Hooks) and Epic 2 (Dependency Security &
  Licence Compliance) carry a `file:line` citation plus a named consumer, re-run against this tree
  this session — five stale line-number citations were caught and corrected (`Makefile:282→304-305`,
  `:261→283-284`, `:264→286-294`, `:439→461-505`, `:424→446`, `:413→436`)
- `REQ-audit-toml-single-source` carries the corpus's only `Shipped, one acceptance criterion false`
  verdict, both halves dated: the duplicate `security` job's failure at pre-deletion `ci.yml:465-482`,
  and the fix (commit `cb75b2b`, ADR-0036, `scripts/check-workflow-suppressions.sh`), re-confirmed
  with a zero-match `grep -n "cargo audit --ignore" .github/workflows/ci.yml` this session. SUPPLY-01's
  previously-pending "first real CI run after the deletion" trigger clause is resolved with a live
  citation: `gh run list` shows run `31320378772` (2026-08-09, `success`) as newest, superseding the
  stale `30861568499` reference, and `gh run view --json jobs` confirms the single (non-duplicated)
  `Security Audit` job concluded `success`
- `REQ-advisory-exception-process` is recorded closed via SUPPLY-02, cited per D-06/D-00i rather than
  re-verified, with the full `--auto` provenance chain (Phase 9's D-07 re-scope, never ratified by a
  human; Phase 12's own D-01/D-08, ratified only at plan 12-01's blocking checkpoint dated 2026-08-09)
  written into the Epic 2 section header note
- `REQ-github-rulesets` recorded owner-only (D-06) with two live confirmations this session:
  `gh api repos/:owner/:repo/rulesets` → `[]` (unapplied) and
  `gh api repos/:owner/:repo/branches/main/protection` → HTTP `404` "Branch not protected" — nothing
  applied by this phase
- All 11 rows in Milestone 10 Epics 3-5 (Release Automation, v0.4.0 Release, Tag-Source Enforcement)
  carry cited verdicts; `REQ-workspace-publish-order`'s `paladin-herald` omission (previously flagged
  only in the doc citation) is confirmed present in the executable `release.yml` `CRATES=(...)` array
  too — recorded as a limitation, not a downgrade
- `REQ-lockstep-versioning`'s mechanism (`Makefile:498`, `cargo release version ... --workspace`) is
  confirmed at the `v0.4.0` tag via `git show v0.4.0:Cargo.toml`, ready for plan 13-12 to cite rather
  than re-derive a commit hash per tag
- Ledger integrity preserved throughout: `grep -c '^| REQ-'` reads `120` before and after this plan's
  edits; zero rows inserted, deleted or reordered; zero `.rs` files modified (`git diff --name-only
  <base>..HEAD -- '*.rs' | wc -l` → `0`)

## Task Commits

1. **Task 1: Derive Milestone 10 Epics 1-2 (12 rows)** - `147017b` (docs), with a self-caught D-00i provenance addition in `f261d9b` (docs)
2. **Task 2: Derive Milestone 10 Epics 3-5 (11 rows)** - `5d85a82` (docs)

## Files Created/Modified

- `.planning/ledgers/milestone-09-12.md` - Verdict cells replaced in place for the 23 requirement rows
  in Milestone 10 Epics 1-5; two section header notes (Epic 1, Epic 2) updated once fully derived; no
  row inserted, deleted, or reordered

## Decisions Made

- Verified every row by re-running its citation against this tree this session (`grep`, `sed`, direct
  file reads, `gh run`/`gh api` calls) rather than trusting the inherited "run-5 input (not yet
  re-derived)" transcription or the bare `pending` marker — this caught five stale Makefile line
  numbers that had drifted since the source ledger was written
- `REQ-audit-toml-single-source`'s verdict cell echoes the ledger's own head-of-file highlight table
  rather than restating the full both-halves account, and adds the live `gh run` resolution of
  SUPPLY-01's pending trigger clause, which the highlight table (written by plan 13-01, before this
  session's `gh` calls) did not yet carry
- `REQ-advisory-exception-process`'s D-00i provenance (Phase 9 D-07 unratified, Phase 12 D-01/D-08
  ratified at plan 12-01's 2026-08-09 checkpoint) was initially only summarized in the task 1 commit
  message and not actually written into the ledger text — caught via self-verification against the
  plan's own must_have before moving to Task 2, and added as a dedicated paragraph in the Epic 2
  section header (commit `f261d9b`)
- `REQ-github-rulesets` and the SUPPLY-01 CI-run trigger were resolved with live `gh` CLI calls in this
  environment (`gh auth status` confirmed an authenticated session) rather than assuming the prior
  transcription of these time-varying GitHub facts still held

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 1 - Bug] D-00i SUPPLY closure provenance was summarized in a commit message but not written into the ledger**
- **Found during:** Self-verification of Task 1's acceptance criteria before proceeding to Task 2
- **Issue:** The plan's `must_have` requires the ledger to record that Phase 9's D-07 re-scope was
  never ratified and that Phase 12's D-01/D-08 were human-selected at plan 12-01's checkpoint dated
  2026-08-09. This provenance was described in the Task 1 commit message but the actual ledger prose
  only cited the SUPPLY-02 closure's evidence transcripts, not the `--auto` ratification chain.
- **Fix:** Added a dedicated paragraph to the Milestone 10 Epic 2 section header, citing
  `REQUIREMENTS.md:2084-2176` (Phase 12's hand-off block, dated 2026-08-09, plan 12-04) verbatim for
  the D-07/D-01/D-08 chain.
- **Files modified:** `.planning/ledgers/milestone-09-12.md`
- **Verification:** `grep -c '12-01' .planning/ledgers/milestone-09-12.md` → `4`; row count and pending-marker checks for the Epic 1-2 range unaffected (still `12`/`0`/`0`).
- **Committed in:** `f261d9b`

**2. [Rule 1 - Bug] Five stale `Makefile` line-number citations, inherited from the source ledger, no longer matched the tree**
- **Found during:** Task 1 and Task 2, while re-deriving each row's citation against the current tree
- **Issue:** The inherited transcriptions cited `Makefile:282` (`hooks`), `:261` (`security`), `:264`
  (`sbom`), `:439` (`release`), `:424` (`publish-dry-run`), and `:413` (`release-check`) — all of which
  have drifted since the source ledger's own citations were written (the `Makefile` has grown/reordered
  targets in the intervening phases). Re-reading `Makefile` directly this session found the true
  current lines: `304-305`, `283-284`, `286-294`, `461-505`, `446`, `436` respectively.
- **Fix:** Corrected each citation in place, noting the drift explicitly in the row text ("not `:NNN`
  — line drift, corrected here") per the phase's own "re-read every `file:line` before repeating it"
  instruction.
- **Files modified:** `.planning/ledgers/milestone-09-12.md`
- **Verification:** Each corrected line number was independently confirmed via `grep -n` /
  `Read` against the current `Makefile` before being written into the ledger.
- **Committed in:** `147017b`, `5d85a82`

### Documented, not auto-fixed: a bug in the plan's own acceptance-criteria wording

**Task 2's `git diff --numstat ... | awk '{print ($2==$3)?"balanced":"unbalanced"}'` check can never print `balanced`.** `git diff --numstat`'s three whitespace-separated fields are insertions ($1), deletions ($2), and filename ($3, a string). The check as literally written compares the deletions count to the filename string — these can never be equal, so the command prints `unbalanced` regardless of what the diff contains (verified: this plan's actual diff is `14	14	.planning/ledgers/milestone-09-12.md`, a true balance, and the check still printed `unbalanced`). The evident intent — stated in the check's own parenthetical, "in-place cell replacement, no rows added or removed" — is `$1==$2` (insertions equal deletions), which this plan's diff satisfies exactly (`14` and `14`). This is the same class of self-referential arithmetic-drift bug this phase's own D-04/D-08/D-09/D-17/D-18 findings catch elsewhere in the corpus, now found inside a plan's acceptance criteria rather than the corpus it audits. **Not auto-fixed** because there is no ledger defect to fix — the row count is genuinely unchanged (`grep -c '^| REQ-'` → `120` before and after, and no row was inserted, deleted, or reordered, confirmed by direct inspection of the diff hunks) — flagging the check's own wording here is the correct action per the evidence bar this plan itself applies to the corpus.

---

**Total deviations:** 2 auto-fixed (both Rule 1 — a missing provenance paragraph and five stale line citations, both caught by this plan's own self-verification before committing), plus one documented-not-fixed discrepancy in the plan's own acceptance-criteria wording (no ledger defect underlies it).
**Impact on plan:** Both auto-fixes were caught and corrected inline before their respective task commits; no scope creep, no downstream plan needs to redo work.

## Issues Encountered

None beyond the two self-caught issues documented above. `gh` CLI calls (`gh run list`, `gh run view`,
`gh api .../rulesets`, `gh api .../branches/main/protection`) all succeeded on the first attempt —
`gh auth status` confirmed an authenticated session at the start of Task 1, so no auth-gate handling
was needed.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- Plan 13-13 (close-out) can rely on Milestone 10's ledger section being fully derived: all 23 rows in
  Epics 1-5 leave zero `Verify`, `pending`, or `run-5 input (not yet re-derived)` markers.
- The corpus's only `Shipped, one acceptance criterion false` row now carries the live `gh run
  31320378772` citation resolving SUPPLY-01's previously-pending trigger clause — plan 13-13's
  close-out should cite this row rather than re-running `gh run list` again, since the trigger has now
  fired and been recorded.
- `REQ-github-rulesets`'s owner-only finding (nothing applied, `main` unprotected) is ready for the
  milestone close-out to surface to the repository owner as an action item; this phase applied nothing
  by design (D-06).
- `REQ-lockstep-versioning`'s `Makefile:498` citation is ready for plan 13-12 to cite directly when
  appending the four ADR-0029 trajectory rows, rather than re-deriving the mechanism.
- No `.rs` file was touched by this plan (`git diff --name-only <base>..HEAD -- '*.rs' | wc -l` → `0`),
  consistent with the phase's D-19 boundary.
- Flag for a later reader of this ledger: the acceptance-criteria wording bug documented above
  (`$2==$3` in the plan's own numstat check) is worth correcting in `13-03-PLAN.md` itself if that file
  is ever revised, though this plan does not modify `PLAN.md` files as part of its own scope.

## Self-Check: PASSED

- `.planning/phases/13-milestone-9-12-ground-truth-recorded-account/13-03-SUMMARY.md` — FOUND
- `.planning/ledgers/milestone-09-12.md` — FOUND
- Commit `147017b` (Task 1) — FOUND
- Commit `f261d9b` (Task 1 self-fix) — FOUND
- Commit `5d85a82` (Task 2) — FOUND

---
*Phase: 13-milestone-9-12-ground-truth-recorded-account*
*Completed: 2026-08-10*
