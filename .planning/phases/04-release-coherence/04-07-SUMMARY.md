---
phase: 04-release-coherence
plan: 07
subsystem: infra
tags: [adr, governance, ledger, requirements-traceability, provenance, semver, rust-edition]

# Dependency graph
requires:
  - phase: 04-release-coherence
    provides: "Plans 04-01 through 04-06's six measurement records and SUMMARYs — every figure
      this plan's ADRs and ledger rows cite (edition uniformity, advisory posture, CI gate
      repair, the SC5 gate suite, version convergence, QUICKSTART repair/timing)"
provides:
  - "ADR-0008 (workspace version 0.7.0) and ADR-0009 (workspace Rust edition 2024), both with
    Code Conformance = must change naming REL-01/REL-02 as already-executed and Downstream
    Consumers naming ARCH-04/HARD-03 and ARCH-03(a)"
  - "PROMOTION.md's numbering index and next-free-ADR-number line updated (0008, 0009 → next
    free 0010)"
  - "CONCERNS.md's false 'edition 2024 does not exist in stable' claim corrected at source with
    dated provenance, original text retained"
  - "ARCH-03(a) and ARCH-04 requirement text amended to cite the two ADRs instead of
    re-adjudicating; PROJECT.md's Key Decisions table gains its first Phase-4-authored rows"
  - "REL-01..REL-05 ledger verdicts in milestone-01.md, REL-05 split into facet rows (five
    satisfied local gates, two deferred-with-reason Docker/Kubernetes clauses), every Phase 4
    deferral consolidated with a bold named owner"
  - "REQUIREMENTS.md's five REL requirements checked off and their traceability rows moved from
    Pending to Complete"
affects: [07-architecture-truth, 09-security-review, 10-hardening, 12-supply-chain-governance,
  14-web-api, 15-pipeline-completion]

# Tech tracking
tech-stack:
  added: []
  patterns:
    - "ADR house structure (seven fixed H2 headings, bulleted Considered Options/Code Locations)
      applied to two new record-only ADRs, mirroring 0006/0007's worked examples"
    - "Amend-at-source with dated parenthetical, copied verbatim from ROADMAP.md:274's shape,
      applied to CONCERNS.md's false edition claim and ARCH-03(a)/ARCH-04's requirement text"
    - "Ledger verdict split into facet rows (REL-05) so five proven-green gates carry `satisfied`
      while two never-executed gates carry `deferred with reason` with a named owner, rather
      than one blended row overstating the whole requirement"

key-files:
  created:
    - .planning/decisions/0008-workspace-version-0-7-0.md
    - .planning/decisions/0009-workspace-rust-edition-2024.md
  modified:
    - .planning/decisions/PROMOTION.md
    - .planning/codebase/CONCERNS.md
    - .planning/REQUIREMENTS.md
    - .planning/PROJECT.md
    - .planning/ledgers/milestone-01.md

key-decisions:
  - "REL-01 was NOT recorded as one blended satisfied row. The manifest/CHANGELOG convergence
    (fully executed) is satisfied; the tag creation (deferred to the orchestrator's post-merge
    step, a worktree-safety correctness fix from plan 04-05) and the push/publish sequence
    (deferred to the human release gate, D-03) are recorded as a separate deferred-with-reason
    row with named owners, so REL-01's own 'the git tag... tell one story' clause is not
    overclaimed as already true inside this worktree."
  - "PROJECT.md's Key Decisions table already held six rows (ADR-0001..0006) from Phases 1 and 3
    by the time this plan ran, contradicting the plan's own framing that ADR-0008/0009 would be
    'the first entries this table has ever held.' Rather than write a literally false claim, the
    evidence-note update states plainly that Phase 4 entered its two rows ahead of the phases
    originally forecast to own them, preserving the historical 'not one protected decision'
    finding without fabricating a contradicted 'first ever' assertion."
  - "REQUIREMENTS.md's REL-01..REL-05 traceability rows were moved to 'Complete' (not a new
    status word), matching QUAL-02's own precedent — a Complete row can still carry deferred
    sub-items owned by later phases; the table itself only ever uses Complete/Pending, no
    partial-status word exists to borrow instead."
  - "Consolidated more Phase 4 deferrals into the ledger than the plan's action text names by
    ID count alone (9 named-owner rows beyond the two ADR-backed REL-01/REL-02 rows), because
    04-02/04-03/04-06's own deferral registers name several distinct facts (four advisories,
    the owner/expiry schema gap, the duplicate CI job, the reqwest disposition, Docker+K8s
    execution, the 300s budget's plausibility, the readiness-probe wiring, the release-push
    trigger's first observation, the sibling-workflow-trigger question) that would otherwise be
    silently absent from the one document later phases read as ground truth."

requirements-completed: [REL-01, REL-02, REL-03, REL-04, REL-05]

coverage:
  - id: D1
    description: "ADR-0008 and ADR-0009 exist with the seven required H2 headings in fixed
      order, bulleted Considered Options/Code Locations, must-change Code Conformance naming
      REL-01/REL-02, and Downstream Consumers naming ARCH-04/HARD-03 and ARCH-03(a);
      PROMOTION.md's index and next-free line updated; no existing ADR (0001-0007) modified"
    requirement: "REL-01"
    verification:
      - kind: other
        ref: "grep '^## ' on both ADR files equals the exact seven-heading sequence; grep -c 'Next free ADR number: 0010' PROMOTION.md == 1; grep -cE '^\\| 000[89] \\|' PROMOTION.md == 2; git diff --name-only .planning/decisions/ excludes 0001-0007"
        status: pass
    human_judgment: false
  - id: D2
    description: "CONCERNS.md's edition finding corrected at source with dated provenance citing
      ADR-0009 and the 1.85/1.97.1 toolchain proof; original 'does not exist in Rust' claim
      retained, not deleted"
    requirement: "REL-02"
    verification:
      - kind: other
        ref: "grep -c 'Amended by Phase 4' CONCERNS.md >= 1; grep -c '0009-workspace-rust-edition-2024' >= 1; grep -ci 'does not exist in Rust' >= 1; grep -c '1.85' >= 1; grep -c '1.97.1' >= 1"
        status: pass
    human_judgment: false
  - id: D3
    description: "ARCH-03(a) and ARCH-04 requirement text amended to cite ADR-0009/ADR-0008
      respectively; PROJECT.md's Key Decisions table gains two rows linking the new ADRs and
      its evidence note is updated (not deleted)"
    requirement: "REL-01"
    verification:
      - kind: other
        ref: "grep -c '0009-workspace-rust-edition-2024' REQUIREMENTS.md >= 1; grep -c '0008-workspace-version-0-7-0' REQUIREMENTS.md >= 1; grep -cE '\\.planning/decisions/000[89]-' PROJECT.md >= 2; grep -qi 'not one protected decision' PROJECT.md"
        status: pass
    human_judgment: false
  - id: D4
    description: "REL-01..REL-05 each carry a cited ledger row at the file's own five-verdict
      legend; REL-05 split into five satisfied facet rows and two deferred-with-reason
      Docker/Kubernetes facet rows, both owned Phase 15 / PIPE; the four pre-existing
      forward-owner rows (REQ-api-documentation, REQ-user-documentation,
      REQ-deployment-artifacts, REQ-epic10-quality-gates) amended in place, not duplicated;
      every Phase 4 deferral (advisories, duplicate CI job, readiness-probe wiring, QUICKSTART
      clean-machine/live-LLM legs, reqwest, release-push observation, sibling-trigger question)
      carries a bold named owner; no REL traceability row left Pending"
    requirement: "REL-03"
    verification:
      - kind: other
        ref: "for r in REL-01..REL-05: grep -q \"| $r\" milestone-01.md; grep -c 'Owner: Phase 15 / PIPE' >= 2 (actual 9); grep -q 'Owner: Phase 14 / WEB'; grep -q SUPPLY-01; grep -c REQ-user-documentation == 1 (and same for the other 3 amended rows); grep -c 'Phase 4 amendments' == 1; sed range REL-01..REL-05 in REQUIREMENTS.md has 0 'Pending'"
        status: pass
    human_judgment: false

# Metrics
duration: 25min
completed: 2026-08-03
status: complete
---

# Phase 4 Plan 07: ADRs, Source Correction, and Ledger Verdicts Summary

**Authored ADR-0008 (workspace version 0.7.0) and ADR-0009 (workspace Rust edition 2024), corrected `CONCERNS.md`'s false edition claim at source, wired both ADRs into Phase 7's ARCH-03(a)/ARCH-04 requirement text, and wrote every REL-01..REL-05 ledger verdict with REL-05 split into honest facet rows and every Phase 4 deferral consolidated under a named owner.**

## Performance

- **Duration:** ~25 min
- **Started:** 2026-08-03T13:30:00Z (approx.)
- **Completed:** 2026-08-03T13:52:23Z
- **Tasks:** 3
- **Files modified:** 7 (2 created, 5 modified)

## Accomplishments

- **ADR-0008** records the workspace version answer (`0.7.0`): the Milestone 6 facade change was
  breaking but shipped inside the pre-1.0 series, so SemVer's pre-1.0 convention expresses it as a
  minor bump; `Code Conformance: must change` names REL-01 as already-executed; `Downstream
  Consumers` names Phase 7's ARCH-04 and Phase 10's HARD-03.
- **ADR-0009** records the workspace edition answer (`2024`): ten of twelve manifests already
  agreed, the toolchain (pinned `1.97.1`) has supported 2024 since Rust 1.85, and the two
  Milestone 5 epics disagreeing internally are stated as amended, not silently overridden;
  `Code Conformance: must change` names REL-02; `Downstream Consumers` names Phase 7's ARCH-03(a)
  and states the doctest-flag orthogonality with DEBT-03/HARD-07.
- `PROMOTION.md`'s numbering index gains rows `0008`/`0009`; next free ADR number advances to
  `0010`. No existing ADR (0001-0007) was modified.
- `CONCERNS.md`'s "edition 2024 does not exist in Rust's stable channel" claim (§"Edition 2024 in
  Project Manifests") is corrected at source with three dated parentheticals (Issue, Impact, Fix
  approach), citing ADR-0009 and the `rustc -vV`/`rust-toolchain.toml` proof; the original claim's
  text is retained in full, per the amend-at-source convention.
- `ARCH-03(a)` and `ARCH-04` (`REQUIREMENTS.md`) now cite ADR-0009 and ADR-0008 respectively
  instead of re-adjudicating; each requirement's remaining, un-answered scope is preserved
  verbatim.
- `PROJECT.md`'s Key Decisions table gains two rows (ADR-0008, ADR-0009); the evidence note
  beneath the table is updated (not deleted) to record that Phase 4 entered these two decisions
  ahead of the phases originally forecast to own them.
- `milestone-01.md` gains a new "Phase 4 amendments" section: REL-01..REL-05 each carry at least
  one cited row at the file's own five-verdict legend. REL-05 is split into five `satisfied`
  facet rows (fmt, clippy, workspace tests, doc tests, every example target) and two `deferred
  with reason` facet rows (multi-arch Docker build, kind-based Kubernetes smoke test), both owned
  **Phase 15 / PIPE** — never blended into one overstated `satisfied` row. Nine further Phase 4
  deferrals (four advisories, the advisory owner/expiry schema gap, the duplicate CI job, the
  reqwest dual-version disposition, the QUICKSTART clean-machine and live-LLM legs, the
  readiness-probe wiring, the release-push first-observation, and the sibling-workflow-trigger
  question) are consolidated with bold named owners drawn from plans 04-02/04-03/04-06 rather
  than re-derived.
- The four pre-existing forward-owner rows (`REQ-api-documentation`, `REQ-user-documentation`,
  `REQ-deployment-artifacts`, `REQ-epic10-quality-gates`) are amended in place — none duplicated —
  to cite the new REL-* verdicts.
- `REQUIREMENTS.md`'s five REL requirement checkboxes are checked (`[x]`) and the traceability
  table's REL-01/02/04/05 rows move from `Pending` to `Complete`, matching REL-03's and QUAL-02's
  own precedent (a Complete row may still carry deferred sub-items owned by later phases).

## Task Commits

Each task was committed atomically:

1. **Task 1: Author ADR-0008 (version) and ADR-0009 (edition), and update the numbering index** — `e984582` (docs)
2. **Task 2: Correct the CONCERNS.md edition finding at source, wire the ADRs into Phase 7's requirement text, and open the Key Decisions table** — `33fe807` (docs)
3. **Task 3: Write the REL-01..REL-05 ledger verdicts and every Phase 4 deferral with a named owner** — `7a02582` (docs)

_Worktree mode: STATE.md and ROADMAP.md's plan-progress tracking are owned by the orchestrator
after this wave's worktree agents merge; no plan-metadata commit is made from this worktree. This
plan's edits to `.planning/codebase/CONCERNS.md`, `.planning/PROJECT.md`, `.planning/
REQUIREMENTS.md` and `.planning/ledgers/milestone-01.md` are the objective's explicit exception —
its source-correction and ledger deliverables, not the forbidden plan-progress checkboxes._

## Files Created/Modified

- `.planning/decisions/0008-workspace-version-0-7-0.md` — new ADR recording the version answer
- `.planning/decisions/0009-workspace-rust-edition-2024.md` — new ADR recording the edition answer
- `.planning/decisions/PROMOTION.md` — numbering index gains rows 0008/0009; next-free line → 0010
- `.planning/codebase/CONCERNS.md` — edition finding's Issue/Impact/Fix-approach lines each gain a
  dated amendment parenthetical; original text retained
- `.planning/REQUIREMENTS.md` — ARCH-03(a) and ARCH-04 amended to cite the ADRs; REL-01..REL-05
  checkboxes checked; traceability table rows moved to Complete
- `.planning/PROJECT.md` — Key Decisions table gains two rows; evidence note updated
- `.planning/ledgers/milestone-01.md` — new "Phase 4 amendments" section with REL-01..REL-05 rows
  and nine consolidated deferral rows; four pre-existing forward-owner rows amended in place

## Decisions Made

- **REL-01 was not recorded as one blended `satisfied` row.** The manifest/`CHANGELOG.md`
  convergence (fully executed by plan 04-05) is `satisfied`; the tag creation (deferred to the
  orchestrator's post-merge step, per plan 04-05's own worktree-safety correction) and the
  push/publish sequence (deferred to the human release gate, D-03) are a separate `deferred with
  reason` row with named owners, so REL-01's "the git tag... tell one story" clause is not
  overclaimed as already true inside this worktree.
- **The Key Decisions table's evidence note was corrected, not left contradicted.** The plan's own
  action text framed ADR-0008/0009 as "the first entries this table has ever held" — but by
  execution time, Phases 1 and 3 had already entered six rows (ADR-0001..0006). Rather than write
  a literally false "first ever" claim to satisfy the plan's framing, the evidence-note addition
  states accurately that Phase 4 entered its two rows ahead of the phases the note's own forecast
  paragraph originally named as their owners.
- **REL-01..REL-05's traceability status was set to `Complete`, not a new word.** The table only
  ever uses `Complete`/`Pending` — no partial-status word exists anywhere in it to borrow instead,
  and `QUAL-02`'s own row (which also carries deferred sub-items) already establishes that
  `Complete` coexists with named forward deferrals in this table's convention.
- **Consolidated more named-owner deferral rows than the plan's action text enumerates by category
  count**, because 04-02/04-03/04-06's own deferral registers each name several distinct facts
  (e.g., the D-15 register alone has six rows) that would otherwise be silently absent from the
  one document later phases read as ground truth if only summarized at the category level.

## Deviations from Plan

None — plan executed as written. The two items under "Decisions Made" above (REL-01's tag/push
split, and the Key Decisions evidence-note wording) are executor judgment calls made to satisfy
the plan's own prohibitions (never record a `satisfied` verdict for unexecuted work; never write
a fabricated claim) where the plan's action text and the tree's actual state diverged slightly —
not deviations from any task's required action, acceptance criteria, or `<verify>` command, all of
which pass as written.

## Issues Encountered

None. All three tasks' acceptance criteria and `<verify>` commands passed on the first attempt.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- The version and edition answers are now protected at the top of the precedence order (ADR),
  not sitting at DOC precedence where the next document mentioning either would auto-override
  them — the exact failure mode `PROJECT.md`'s "not one protected decision" finding names.
- Phase 7 can read ARCH-03(a) and ARCH-04 and inherit the edition and version-bump answers
  directly, rather than re-deciding either.
- `milestone-01.md` now carries a single, cited, ground-truth record of every REL-01..REL-05
  verdict and every Phase 4 deferral with a named owner — Phases 9, 12, 14 and 15 can each find
  their inherited work by owner tag without re-deriving it from the six prior plans' raw
  measurement records.
- This is Phase 4's final plan. No blockers for Phase 5 or any later phase; every deferral this
  phase created carries a named forward owner, and no ledger row claims a Docker or Kubernetes
  gate was proven rather than authored.

---
*Phase: 04-release-coherence*
*Completed: 2026-08-03*

## Self-Check: PASSED

- FOUND: `.planning/decisions/0008-workspace-version-0-7-0.md`
- FOUND: `.planning/decisions/0009-workspace-rust-edition-2024.md`
- FOUND: `.planning/phases/04-release-coherence/04-07-SUMMARY.md`
- FOUND: commit `e984582` (Task 1)
- FOUND: commit `33fe807` (Task 2)
- FOUND: commit `7a02582` (Task 3)
