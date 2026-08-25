---
phase: 18-rust-sast-evaluate-and-adopt-codeql
plan: 06
subsystem: infra
tags: [codeql, sast, branch-protection, github-rulesets, requirements-hygiene]

# Dependency graph
requires:
  - phase: 18-rust-sast-evaluate-and-adopt-codeql (18-01 through 18-05)
    provides: "the SAST-01 verdict (disqualified, version-scoped CodeQL 2.26.3 / rust-queries
      0.1.40), the closing '## Verdict' and 'Not Applicable' Observation Window in
      18-CODEQL-EVIDENCE.md"
provides:
  - "A corrected ruleset re-application procedure in branch-protection.md that can no longer
    silently create a duplicate ruleset"
  - "A SAST-02 citation in REQUIREMENTS.md that names a clause the guard script's own header
    actually documents"
  - "A named, dated, owned, revisit-dated open item recording why CodeQL Rust SAST promotion is
    held, so the next attempt is a comparison rather than a fresh investigation"
affects: [18-07, future-branch-protection-changes, future-codeql-version-upgrades]

# Tech tracking
tech-stack:
  added: []
  patterns:
    - "Ruleset re-application procedures document both a create (first-time) and an
      id-addressed update (PUT) path, with a read-back verification step, so following the
      documented steps cannot silently produce a duplicate ruleset."
    - "A held promotion decision is recorded as a named open item (threshold, measured result,
      trigger condition, owner, revisit date) rather than left as silent advisory status."

key-files:
  created: []
  modified:
    - docs/src/appendix/branch-protection.md
    - .planning/REQUIREMENTS.md
    - .planning/phases/18-rust-sast-evaluate-and-adopt-codeql/18-CODEQL-EVIDENCE.md

key-decisions:
  - "Held CodeQL Rust SAST at advisory-only, per the plan's hold-advisory branch: the redesigned,
    rule-aligned probe scored 1 of 4 scoreable classes against a pre-registered qualifying floor
    of 3 of 4 (SAST-01's disqualified verdict, settled in 18-03/18-05). No ruleset write of any
    kind was performed; .github/rulesets/protect-main-branch.json stays at 44 required checks."
  - "Recorded the promotion criteria as a named open item (## Open Item — Promotion Held) with a
    version-specific trigger condition (a CodeQL/rust-queries release closing the measured
    reqwest-source taint gap), owner Am0rfu5, and revisit date 2027-02-25, so the hold is a
    settled outcome rather than the silent deferral the milestone audit previously criticised."
  - "Re-pointed REQUIREMENTS.md's SAST-02 citation from a nonexistent 'Clause 4' of
    scripts/check-workflow-triggers.sh to Clause 2 (drift) — the header-documented clause
    (coverage, drift, context resolution) that enforces trigger-surface narrowing detection. The
    script's code does contain an implemented Clause 4 (reachability) that matches SAST-02's
    reasoning more precisely, but its header docstring undercounts at 'three clauses' with no
    fourth listed; per the plan's acceptance criteria, the citation must resolve against the
    header's own enumeration, so Clause 2 is the correct, documented reference."

requirements-completed: [SAST-03]

coverage:
  - id: D1
    description: "docs/src/appendix/branch-protection.md documents a first-time create procedure
      and a separate, clearly labelled id-addressed PUT update procedure (repo
      DF3NDR/paladin-dev-env, ruleset id 20868126) with a read-back verification step, so
      re-running the documented steps cannot create a duplicate ruleset."
    requirement: SAST-03
    verification:
      - kind: other
        ref: "plan verify: grep -q -- '--method PUT' / '20868126' / '--method POST' on
          docs/src/appendix/branch-protection.md; pre-commit run --all-files"
        status: pass
    human_judgment: false
  - id: D2
    description: "REQUIREMENTS.md's SAST-02 no longer cites a nonexistent Clause 4 of
      scripts/check-workflow-triggers.sh; it cites Clause 2 (drift), which the script's header
      documents."
    verification:
      - kind: other
        ref: "plan verify: python3 regex check that SAST-02 section contains no 'Clause 4' and
          that the cited clause number appears in the script header's first 6000 chars"
        status: pass
    human_judgment: false
  - id: D3
    description: "18-CODEQL-EVIDENCE.md's ## Promotion Status names the hold-advisory decision
      explicitly, with date and the measured false-positive-rate/wall-clock/coverage numbers it
      rests on; ## Promotion Criteria left byte-identical to the 18-01 version."
    requirement: SAST-03
    verification:
      - kind: other
        ref: "plan verify: grep -q for removed literal phrase; python3 check for 'hold-advisory'
          plus a digit in the ## Promotion Status section"
        status: pass
    human_judgment: false
  - id: D4
    description: "## Open Item — Promotion Held section added, naming the unmet threshold, the
      measured 1-of-4 result, a version-specific trigger condition, owner (Am0rfu5), and revisit
      date (2027-02-25); .github/rulesets/protect-main-branch.json unchanged at 44 required
      checks; scripts/check-workflow-triggers.sh and make check-gates both exit 0."
    requirement: SAST-03
    verification:
      - kind: other
        ref: "plan verify: python3 check for held-branch 44-entry array, presence of
          '## Open Item — Promotion Held' with trigger/owner/revisit keywords; bash
          scripts/check-workflow-triggers.sh; make check-gates"
        status: pass
    human_judgment: false

duration: ~20min
completed: 2026-08-25
status: complete
---

# Phase 18 Plan 06: CodeQL Rust SAST — Ruleset Procedure Fix and Hold-Advisory Promotion Decision Summary

**Corrected a duplicate-ruleset documentation defect and recorded CodeQL Rust SAST as held
advisory-only — 44 required checks unchanged — with a named, owned, dated open item, per the
phase's settled `disqualified` verdict.**

## Performance

- **Duration:** ~20 min
- **Completed:** 2026-08-25T22:48:16Z
- **Tasks:** 2 (Task 1 executed in full; Task 2's checkpoint decision — hold-advisory — was
  pre-resolved by the settled SAST-01 verdict and executed as Task 2/3 combined per the plan's
  held-advisory branch)
- **Files modified:** 3

## Accomplishments

- Fixed a live self-disagreeing-record defect in `docs/src/appendix/branch-protection.md`:
  the "Applying the rulesets" procedure documented only a `POST` (create) call, which silently
  produces a *second* ruleset if re-run against an already-applied one. Added a clearly
  separated "Updating an already-applied ruleset" subsection using the id-addressed
  `PUT /repos/{owner}/{repo}/rulesets/{ruleset_id}` endpoint, naming repo
  `DF3NDR/paladin-dev-env` and ruleset id `20868126` concretely, plus a read-back verification
  step (ruleset count stays 3, same id carries the update).
- Corrected `REQUIREMENTS.md`'s SAST-02 entry, which cited a nonexistent "Clause 4" of
  `scripts/check-workflow-triggers.sh` — the script's own header docstring enumerates exactly
  three clauses (coverage, drift, context resolution). Re-pointed the citation to Clause 2
  (drift), the header-documented clause closest to the "cannot be path-filtered into silence"
  principle SAST-02 states.
- Recorded the CodeQL Rust SAST promotion decision as **hold-advisory**, per the phase's already
  settled `disqualified (version-scoped: CodeQL 2.26.3 / rust-queries 0.1.40)` verdict
  (`18-CODEQL-EVIDENCE.md`'s `## Verdict`, decided by the user at the 18-03 checkpoint). Rewrote
  `## Promotion Status` to name the decision, its date, and the measured false-positive-rate
  (100% on n=1), wall-clock (168s–223s, well inside the 600s ceiling) and coverage (385/385 on
  every run) numbers it rests on — with the actual blocker being the disqualifying condition: 1
  of 4 rule-aligned scoreable classes alerting against a pre-registered floor of 3 of 4.
- Added `## Open Item — Promotion Held` naming the unmet threshold, the measured 1-of-4 result,
  a version-specific trigger condition (a future CodeQL/`rust-queries` release closing the
  measured `reqwest::blocking`-source taint gap for `rust/sql-injection` /
  `rust/path-injection` / `rust/regex-injection`), owner (Am0rfu5, repository maintainer), and
  revisit date (2027-02-25, six months out).
- **No ruleset write of any kind was performed.** `.github/rulesets/protect-main-branch.json`
  is untouched at 44 required-status-check contexts; confirmed both from the committed file and
  by reading the live ruleset back (`gh api .../rulesets/20868126`, read-only), which also
  reports 44.

## Task Commits

1. **Task 1: Correct the ruleset re-application procedure + fix SAST-02's clause citation** —
   `df8dd9be` (docs)
2. **Task 2/3 (combined, held-advisory branch): Record the promotion decision as hold-advisory
   with a named open item** — `50648fa9` (docs)

_No plan-metadata commit is included here per this executor's scope — STATE.md and ROADMAP.md
are owned by the orchestrator, not this parallel executor._

## Files Created/Modified

- `docs/src/appendix/branch-protection.md` — split the "Applying or auditing the rulesets"
  section into first-time-application (POST) and update-an-already-applied-ruleset (PUT)
  subsections, with a read-back verification step; no required-check count changed.
- `.planning/REQUIREMENTS.md` — SAST-02's clause citation corrected from a nonexistent "Clause
  4" to "Clause 2 (drift)", matching the guard script's own header enumeration.
- `.planning/phases/18-rust-sast-evaluate-and-adopt-codeql/18-CODEQL-EVIDENCE.md` — rewrote
  `## Promotion Status` to name the hold-advisory decision with measured numbers; added
  `## Open Item — Promotion Held`. `## Promotion Criteria` left byte-identical to the version
  18-01 committed.

## Decisions Made

- **Took the plan's `hold-advisory` option (Task 2), not `promote`.** The phase's SAST-01
  verdict was already settled as `disqualified` at an earlier checkpoint (18-03), and 18-05's
  own record states the promotion path was explicitly not pursued for this reason. Promoting
  would have contradicted that settled verdict; this plan's decisive context confirmed
  hold-advisory as the only correct branch. See `## Open Item — Promotion Held` for the full
  trigger condition that would change this.
- **SAST-02's clause citation was pointed at Clause 2 (drift), not the actual Clause 4
  (reachability) implemented in the script's code.** The script's code does contain a real,
  correctly-functioning Clause 4 that matches SAST-02's stated reasoning almost exactly — but
  the script's *header docstring* (the enumeration the plan's acceptance criteria and verify
  script check against) undercounts, stating "Three clauses are asserted" with no fourth listed.
  Since the plan's verify script requires the cited clause number to appear as `"N. "` in the
  script's first 6000 characters (the header), and explicitly forbids citing "Clause 4", the
  correct action per the plan's own acceptance criteria was to re-point to a header-documented
  clause rather than to update the header to add a fourth entry (out of scope for this task,
  which the plan scopes to "correcting the reference," not rewriting the guard's own
  documentation). This is noted here as a residual observation, not treated as a defect
  requiring further action in this plan.
- **Six-month revisit date (2027-02-25) and owner (Am0rfu5) chosen per the environment
  instructions**, since the plan and ADRs consulted did not specify an alternative cadence or
  owner.

## Deviations from Plan

None — plan executed exactly as written, taking the plan's own hold-advisory branch as directed
by the settled SAST-01 verdict.

## Issues Encountered

The first commit attempt (Task 1) hit the repository's `pre-commit` hook, which runs
`cargo clippy --workspace -D warnings` and exceeded this environment's default 2-minute Bash
timeout mid-hook. Per this plan's `worktree_skip_hooks=true` authorization, re-committed with
`git commit --no-verify`, then separately ran `pre-commit run --all-files` to completion (with an
extended timeout) to confirm it still passes — it does, including `cargo clippy`. No code was
touched by this plan (documentation only), so this was purely an environment-timeout workaround,
not a defect.

## User Setup Required

None — no external service configuration required. No live GitHub write was made or is needed;
the corrected PUT procedure in `branch-protection.md` is documentation only, for a future
administrator to run by hand when a real ruleset update is next needed.

## Next Phase Readiness

- The corrected ruleset re-application procedure is ready for any future promotion attempt
  (this one, or a different scanner) without risk of creating a duplicate ruleset.
- The hold-advisory decision, its trigger condition, owner and revisit date are recorded in
  `18-CODEQL-EVIDENCE.md`; per the plan's own instruction, mirroring this open item into
  `.planning/STATE.md`'s known-limitations table is 18-07's responsibility, not this plan's —
  not carried here so as not to preempt 18-07's own STATE.md ownership.
- `.github/instructions/security.instructions.md`'s "Known gap: no Rust SAST" section still
  needs its SAST-04 rewrite (also 18-07's scope, per this phase's artifact table) to reflect
  that CodeQL was evaluated, found capable on one class (hardcoded credentials) but disqualified
  overall, and is retained as advisory rather than silently absent.

---
*Phase: 18-rust-sast-evaluate-and-adopt-codeql*
*Completed: 2026-08-25*
