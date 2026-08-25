---
phase: 11-facade-residue-deferred-register-disposition
plan: 05
subsystem: docs
tags: [adr-governance, ledger-amendment, requirements-correction, facade-cleanup, close-out]

# Dependency graph
requires:
  - phase: 11-facade-residue-deferred-register-disposition (plans 01-04)
    provides: "facade-01 rustdoc register, ADR-0034 (D1-D4 disposition), facade-03 removed-features register + ADR-0035, facade-04 M9 candidate triage — all four artefacts this plan indexes into the ledger and REQUIREMENTS.md"
provides:
  - "Five amended REQ-* ledger rows (REQ-m8-deferred-items-register, REQ-deferred-cli-user-commands, REQ-deferred-tensorflow-ml-adapter-v3, REQ-adapter-disposition-record, REQ-m8-epic3-no-extractions) each citing this phase's registers/ADRs by path, row count unchanged at 86"
  - "REQ-m8-deferred-items-register's D1-D4 pending marker resolved — all five deferred items (D1-D5) now carry a disposition"
  - "REQUIREMENTS.md FACADE-01 and FACADE-03(a) corrected at source, both originals retained"
  - "PROMOTION.md numbering index advanced to 0036 with two new contiguous rows (0034, 0035) and a dated note"
  - "PROJECT.md Key Decisions table gains one row per new ADR (29 -> 31 rows)"
affects: []

# Tech tracking
tech-stack:
  added: []
  patterns:
    - "Dated in-place amendment appended to an existing Evidence/Verdict cell, never replacing the superseded prose (D-00d)"
    - "PROMOTION.md six-step promotion procedure, step 5 (next-free advance) performed last, in its own commit, no other Phase-11 artefact bundled"

key-files:
  created: []
  modified:
    - .planning/ledgers/milestone-07-08.md
    - .planning/REQUIREMENTS.md
    - .planning/decisions/PROMOTION.md
    - .planning/PROJECT.md

key-decisions:
  - "REQ-m8-deferred-items-register's D1-D4 pending marker resolved by citing ADR-0034's four verdicts in one clause each, without restating the marker's literal wording anywhere in the replacement prose"
  - "REQ-deferred-cli-user-commands amended to carry the runnable SHA recovery pointer (git show 3d48768^:...) instead of only the register's own branch-named text, with the measured branch state (no local branch, remote-tracking ref does resolve) recorded rather than assumed"
  - "FACADE-01's REQUIREMENTS.md 'Done when' clause corrected in place: the conversion-to-log::* branch is never available because all 17 occurrences are rustdoc-example doc-comment lines, matching the ROADMAP amendment and the facade-01 register word for word"
  - "FACADE-03(a)'s REQUIREMENTS.md recovery sentence corrected in place: one commit (3d48768) removed both the CLI and ML features, so one immutable-SHA pointer replaces the branch-vs-commit split; the branch state is recorded as actually measured (no local branch, but the remote-tracking ref does resolve) rather than the stronger 'absent as local or remote ref' claim D-10 carried"
  - "ADR numbering advanced by two, from 0034 to 0036, as the phase's last act (D-14) — no Part B candidate closed by either ADR"

patterns-established: []

requirements-completed: [FACADE-01, FACADE-02, FACADE-03, FACADE-04]

coverage:
  - id: D1
    description: "Five ledger rows amended in place citing this phase's four artefacts by path, row count unchanged at 86, D1-D4 pending marker resolved"
    requirement: "FACADE-01"
    verification:
      - kind: other
        ref: "grep -c '^| REQ-' milestone-07-08.md (86, unchanged); grep -c 'plan 11-05' (5); grep -c 'pending — plan 11-05' (0); grep -c 'plan 11-01' (1, retained); per-artefact citation greps each non-zero"
        status: pass
    human_judgment: false
  - id: D2
    description: "REQUIREMENTS.md FACADE-01 and FACADE-03(a) corrected in place, purely additive, both originals retained"
    requirement: "FACADE-03"
    verification:
      - kind: other
        ref: "grep -qF 'chore/facade-cleanup-m8-finish' (retained); grep -qF 'converted to `log::*`' (retained); grep -c 'Corrected 2026-08-08 (plan 11-05)' (2); git diff --numstat shows 0 deletions across both edits (24 insertions, 0 deletions)"
        status: pass
    human_judgment: false
  - id: D3
    description: "PROMOTION.md numbering index advanced to 0036 with two contiguous rows and a dated note; PROJECT.md Key Decisions gains one row per ADR"
    requirement: "FACADE-02"
    verification:
      - kind: other
        ref: "grep -c '^| 00' PROMOTION.md (35); awk sort -c on index column (ascending, contiguous); grep 'Next free ADR number: 0036'; PROJECT.md Key Decisions row count 29 -> 31; git diff shows rows 0001-0033 byte-identical"
        status: pass
    human_judgment: false
  - id: D4
    description: "Zero .rs files touched across all three task commits (D-13)"
    requirement: "FACADE-04"
    verification:
      - kind: other
        ref: "git diff --name-only HEAD~1 HEAD | grep -c 'rs$' == 0, checked after each of the three task commits"
        status: pass
    human_judgment: false

duration: ~35min
completed: 2026-08-09
status: complete
---

# Phase 11 Plan 05: Facade Residue Close-Out — Ledger Amendment, REQUIREMENTS.md Correction, ADR Numbering Advance Summary

**Closed Phase 11 by amending five `REQ-*` ledger rows to cite the phase's four registers/ADRs by path, correcting FACADE-01's and FACADE-03(a)'s defective claims in REQUIREMENTS.md at source, and advancing `PROMOTION.md`'s ADR numbering index to 0036 as the phase's last act.**

## Performance

- **Duration:** ~35 min
- **Completed:** 2026-08-09
- **Tasks:** 3
- **Files modified:** 4

## Accomplishments

- Amended `REQ-m8-deferred-items-register` in `.planning/ledgers/milestone-07-08.md`, resolving the
  `D1–D4: pending — plan 11-05` marker plan 11-01 left on this row (that literal marker string no
  longer appears anywhere in the file). The replacement prose names each of ADR-0034's four verdicts
  in one clause: D1 defers to a facade-wide no-alias sweep; D2 splits into three independent
  verdicts (`user_service.rs` split withdrawn to a three-owner split, `content_service.rs` and
  `event_manager.rs` each deferred independently); D3 defers to the broader builder/execution
  refactor with HARD-05 answered; D4 defers to the dependency-coupling review. Plan 11-01's own
  earlier D5 amendment text is retained unchanged beneath the new passage.
- Amended `REQ-deferred-cli-user-commands`, citing `.planning/registers/facade-03-removed-features.md`
  §1 and replacing the row's branch-named recovery instruction with the runnable SHA form
  `git show 3d48768^:src/application/cli/commands/user.rs`, recording that a branch ref — local or
  remote-tracking — is mutable and deletable while a commit SHA is immutable, and that no local
  branch exists today but the remote-tracking ref does resolve.
- Amended `REQ-deferred-tensorflow-ml-adapter-v3`, citing the facade-03 register §2 and ADR-0035,
  recording that the `paladin-ml` leaf-crate placement condition is now promoted out of DOC
  precedence into an ADR and restating the asymmetric non-goal split (overridden for
  `paladin-herald`, still holding for `paladin-ml`) in one clause.
- Amended `REQ-adapter-disposition-record` and `REQ-m8-epic3-no-extractions`, both citing
  `.planning/registers/facade-04-m9-candidate-triage.md`'s 14/6/0 tally and its finding that
  `paladin-arsenal`/`paladin-sanctum` are artefacts of a mis-written table, and confirming no
  relocation inside ADR-0028's executed range is re-planned by this phase. Confirmed the
  head-of-file summary-table entry for `REQ-m8-epic3-no-extractions` (line 135) still states
  something true after the section-row amendment — no edit needed there.
- Re-ran `grep -c '^| REQ-' milestone-07-08.md` before and after: both **86**. No row inserted,
  deleted or reordered — the commit's own `--numstat` shows added lines equal to deleted lines (5/5).
- Corrected REQUIREMENTS.md's FACADE-01 "Done when" clause in place: appended a dated
  `**Corrected 2026-08-08 (plan 11-05):**` passage stating the re-measured finding (17 occurrences
  across 6 files, all `///`/`//!` doc-comment lines, 0 non-doc-comment matches) so the conversion
  branch is never taken, using the same two commands and the same 17/0 figures the amended ROADMAP
  criterion 1 and the facade-01 register carry. The original clause is retained above it, unedited.
- Corrected REQUIREMENTS.md's FACADE-03(a) recovery sentence in place: appended a dated correction
  stating that one commit (`3d48768`) removed both the CLI and ML features, so the branch-for-CLI /
  commit-for-ML split attributes one event to two different pointer kinds; the durable pointer is
  the immutable SHA, not the branch; and the branch state is recorded as actually measured (no
  local branch exists, but the remote-tracking ref `refs/remotes/origin/chore/facade-cleanup-m8-finish`
  does resolve and is an ancestor of `3d48768`) rather than asserting the stronger "absent as local
  or remote ref" claim `11-CONTEXT.md`'s D-10 carried. Both edits are purely additive
  (`git diff --numstat`: 24 insertions, 0 deletions across the commit).
- Advanced `PROMOTION.md`'s ADR numbering index: added two contiguous rows (0034, 0035) after row
  0033, added a dated note recording the two-number advance, the `ls .planning/decisions/0034-*.md
  .planning/decisions/0035-*.md` proof both files exist, that neither closes a Part B candidate
  (`deferred-features.md` is not among the eleven listed), and that no existing index row was
  renumbered — then updated `**Next free ADR number: 0036**` last, in the same commit as the two
  index rows and the dated note, with no ledger/REQUIREMENTS.md change bundled alongside it.
- Added two `## Key Decisions` rows to `.planning/PROJECT.md` (ADR-0034, ADR-0035), both `conforms`,
  raising the table from 29 to 31 rows.
- Confirmed via `ls .planning/decisions/0034-*.md .planning/decisions/0035-*.md` (precondition) that
  both ADR files exist before writing any index row, per plan 11-01's resolved checkpoint (option-a).
- Zero `.rs` files touched across all three commits — checked and confirmed after each (D-13).

## Task Commits

Each task was committed atomically:

1. **Task 1: Amend the five ledger rows in place** - `8a55f70` (docs)
2. **Task 2: Correct FACADE-01's and FACADE-03(a)'s defective claims in REQUIREMENTS.md at source** - `d9fc295` (docs)
3. **Task 3: Advance the ADR numbering index and add the Key Decisions rows — the phase's last act** - `0aa2166` (docs)

_No TDD tasks in this plan — D-13 forbids executable `.rs` changes; all three tasks are
documentation-only._

## Files Created/Modified

- `.planning/ledgers/milestone-07-08.md` - Five `REQ-*` rows amended in place (Evidence cells only),
  row count unchanged at 86, D1-D4 pending marker resolved
- `.planning/REQUIREMENTS.md` - FACADE-01 and FACADE-03(a) corrected in place, both originals
  retained, purely additive
- `.planning/decisions/PROMOTION.md` - Two Numbering-index rows added (0034, 0035), a dated note,
  and the Next free ADR number advanced to 0036
- `.planning/PROJECT.md` - Two `## Key Decisions` rows added (ADR-0034, ADR-0035), both `conforms`

## Decisions Made

- **The D1-D4 pending marker is resolved by naming each of ADR-0034's four verdicts rather than by
  a single summary sentence** — the plan's own acceptance criterion requires the row be readable
  without opening the ADR, so each of D1, D2 (three sub-verdicts), D3 and D4 gets its own clause.
- **The recovery pointer on `REQ-deferred-cli-user-commands` is stated in fully runnable form**
  (`git show 3d48768^:src/application/cli/commands/user.rs`) rather than only naming the SHA, so a
  future reader can copy-paste it directly.
- **The FACADE-03(a) correction states the measured branch result exactly as re-verified this
  session** (`git rev-parse --verify refs/remotes/origin/chore/facade-cleanup-m8-finish` resolves to
  `4bf6745…`) rather than repeating the stronger "absent as local or remote ref" claim carried in
  `11-CONTEXT.md`'s D-10 — per the plan's explicit prohibition against asserting an absence unless
  the command actually fails.
- **PROMOTION.md's `Next free ADR number` line was advanced only after both ledger amendments and
  both REQUIREMENTS.md corrections were already committed** (Task 3 runs last), and within Task 3
  the advance is bundled with the two index rows and the dated note in a single commit that touches
  no other Phase-11 artefact — matching D-14's "last act" instruction and the task's own acceptance
  criterion about commit isolation.
- **No Part B candidate is closed by ADR-0034 or ADR-0035** — `deferred-features.md` is not among
  the eleven listed candidates in `PROMOTION.md`'s Part B inventory, confirmed before writing the
  dated note rather than assumed.

## Deviations from Plan

None in substance. One pre-existing, non-introduced discrepancy noted rather than fixed, per this
phase's own precedent (plan 11-02's SUMMARY records an analogous case):

**Note on `grep -c 'Next free ADR number'` count.** Task 3's acceptance criteria state this grep
should return `1` after the task. It returns `2`, both before and after this plan's edit — the
second match is the pre-existing prose "Updating the `Next free ADR number` line in this file." in
`PROMOTION.md`'s `### Part A — the procedure` section (step 5), unrelated to the actual index line
this task amends. Confirmed via `git show 7027570:.planning/decisions/PROMOTION.md | grep -c 'Next
free ADR number'` — the count was already `2` before this plan touched the file. This plan's edit
did not introduce the second match and could not remove it without rewriting Part A's procedure
prose, which is out of this task's scope. The underlying invariant the criterion protects — the
actual index line reads `**Next free ADR number: 0036**` — holds and is separately verified; the
plan's own automated `<verify>` block checks for that exact string's presence, not the grep count,
and passes cleanly.

## Issues Encountered

None beyond the one pre-existing discrepancy noted above, which does not affect this plan's
substance or its automated verification.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- **Phase 11 is fully closed.** Every artefact the phase produced (two registers from wave 1/2, two
  ADRs, one triage register) is now reachable from a `REQ-*` ledger row by path, per this plan's
  own success criterion.
- **REQUIREMENTS.md's FACADE-01 and FACADE-03(a) defective claims are corrected at source**, both
  with their originals intact for a future auditor to compare against.
- **`PROMOTION.md`'s ADR numbering line reads 0036** — the next phase to author an ADR reads this
  line and takes 0036 without needing to `ls` the directory.
- **The unratified ADR-0031 dependency is unresolved by this plan and remains so** — ADR-0034 and
  ADR-0035 both cite it, and neither this plan nor its predecessors ratify it. Any future phase
  executing a D3/D4 relocation on ADR-0031's authority should confirm it with a human first
  (carried forward from plan 11-01's checkpoint resolution).
- **No blocker.** Phase 11's five requirements (FACADE-01 through FACADE-04, closed by this plan's
  amendments) are complete; nothing further is owed to any of them.

## Self-Check: PASSED

- FOUND: `.planning/ledgers/milestone-07-08.md` (5 rows amended, 86 total, verified)
- FOUND: `.planning/REQUIREMENTS.md` (both corrections present, verified)
- FOUND: `.planning/decisions/PROMOTION.md` (35 index rows, next-free 0036, verified)
- FOUND: `.planning/PROJECT.md` (31 Key Decisions rows, verified)
- FOUND commit `8a55f70` (Task 1: ledger amendments)
- FOUND commit `d9fc295` (Task 2: REQUIREMENTS.md corrections)
- FOUND commit `0aa2166` (Task 3: PROMOTION.md advance + PROJECT.md rows)

---
*Phase: 11-facade-residue-deferred-register-disposition*
*Completed: 2026-08-09*
