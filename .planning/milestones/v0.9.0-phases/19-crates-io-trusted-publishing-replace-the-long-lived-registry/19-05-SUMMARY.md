---
phase: 19-crates-io-trusted-publishing-replace-the-long-lived-registry
plan: 05
subsystem: docs
tags: [crates-io, trusted-publishing, oidc, release-automation, changelog, documentation]

requires:
  - phase: 19-crates-io-trusted-publishing-replace-the-long-lived-registry
    provides: "The completed evidence log (19-PUBLISH-EVIDENCE.md) covering crate-set reconciliation, the OIDC proof event, and the credential revocation ledger, which this plan's docs copy from rather than re-derive"
provides:
  - "A rewritten docs/src/appendix/release-automation.md carrying a `## Trusted Publishing` section: mechanism, environment/protection posture, an eleven-row per-crate trust table, a four-row credential history, dry-run claim boundary, break-glass recovery, and known limits"
  - "A corrected `## Canonical Publish Order` (eleven crates, real order, paladin-herald's insertion point explained, no paladin-cli reference) and an Operator Guide rewritten to document the actual PR-merge + direct-tag-push release flow (make release's push to main is dead)"
  - "docs/src/appendix/release-checklist.md and docs/src/contributing/development-setup.md reconciled to the same eleven-crate order and Trusted Publishing credential path, with both `Required secret` sections that described the deleted repository-secret behavior removed"
  - "A `### Security` entry under `## [Unreleased]` in CHANGELOG.md recording the credential migration"
  - "PUB-01 marked complete in REQUIREMENTS.md (checkbox + traceability table) — the last of this plan's three frontmatter requirements not already closed by an earlier 19-0x plan"
affects: [20]

tech-stack:
  added: []
  patterns:
    - "Amend-at-source for historical records: the tooling-comparison table's 'Required secrets / infra' row is not rewritten to match the later decision — a dated note is added beneath it instead, preserving what was true when it was written"
    - "One source of truth, two pointers: release-checklist.md and development-setup.md point at release-automation.md's trust table and credential history via anchor links rather than each carrying their own drifting copy"

key-files:
  created:
    - .planning/phases/19-crates-io-trusted-publishing-replace-the-long-lived-registry/19-05-SUMMARY.md
  modified:
    - docs/src/appendix/release-automation.md
    - docs/src/appendix/release-checklist.md
    - docs/src/contributing/development-setup.md
    - CHANGELOG.md
    - .planning/REQUIREMENTS.md

key-decisions:
  - "The Operator Guide's 'Cutting a Release' section was rewritten beyond what Task 1's action text explicitly named, to document the real PR-merge + direct-tag-push flow rather than the dead make-release-pushes-to-main flow it previously described. This was flagged as a CRITICAL DOC CORRECTION in the executor's session context and treated as a Rule 2 deviation (missing critical functionality/correctness) — leaving the guide teaching a flow that has been dead since v0.5.1 (2026-06-04) directly contradicts the plan's own criterion that no document may claim a behavior that does not exist."
  - "PUB-01 was marked complete in REQUIREMENTS.md by this plan, not an earlier one. The mechanical crate-set reconciliation (eleven-crate release.yml array matching cargo metadata) landed in 19-01, before any trust link was created in 19-03 — satisfying the requirement's ordering constraint — but no prior 19-0x SUMMARY's requirements-completed field listed PUB-01 (19-02 closed PUB-02/PUB-05, 19-03 closed PUB-03, 19-04 closed PUB-04). This plan's documentation is what makes the reconciliation an operator-visible, written fact rather than an implementation detail, which is the remaining half PUB-01 names."

patterns-established:
  - "Trust-table equality by crates.io package name, never by directory name: both divergent rows (paladin-ai-core / crates/paladin-core, paladin-ai / workspace root) carry their own source-directory column rather than a footnote, on every table that lists the eleven crates."

requirements-completed: [PUB-01]

coverage:
  - id: D1
    description: "release-automation.md carries a '## Trusted Publishing' section (mechanism, environment/protection posture, eleven-row per-crate trust table, four-row credential history, dry-run claim boundary, break-glass recovery, known limits) replacing the deleted '### Required Secret' section"
    requirement: "PUB-05"
    verification:
      - kind: other
        ref: "Task 1 <verify> one-liner re-run directly: all heading-presence, row-count, and grep assertions pass; bash scripts/check-doc-config.sh exits 0 (150 YAML blocks checked, 0 failed)"
        status: pass
    human_judgment: false
  - id: D2
    description: "## Canonical Publish Order corrected to the real eleven-crate order with paladin-herald's insertion point explained and no paladin-cli reference; release-checklist.md and development-setup.md reconciled to the same order and credential path"
    requirement: "PUB-01"
    verification:
      - kind: other
        ref: "Task 2 <verify> one-liner re-run directly: all grep assertions pass, git diff --exit-code docs/src/SUMMARY.md succeeds (unchanged), bash scripts/check-doc-config.sh exits 0"
        status: pass
    human_judgment: false
  - id: D3
    description: "CHANGELOG.md carries a ### Security entry under ## [Unreleased] naming the credential migration and pointing at the credential history rather than restating it"
    requirement: "PUB-04"
    verification:
      - kind: other
        ref: "Task 3 <verify> one-liner re-run directly: Security heading present after Unreleased and before first dated version heading; entry contains 'Trusted Publishing', 'crates-io', 'paladin-herald', and a pointer to release-automation.md; dated 0.8.1-rc.1/rc.2 sections byte-unchanged (git diff shows only additions); bash scripts/check-changelogs.sh exits 0"
        status: pass
    human_judgment: false
  - id: D4
    description: "PUB-01 marked complete in REQUIREMENTS.md (checkbox + traceability row)"
    requirement: "PUB-01"
    verification:
      - kind: other
        ref: "gsd-tools query requirements.mark-complete PUB-01 -> {updated: true, marked_complete: [PUB-01]}; git diff .planning/REQUIREMENTS.md confirms both the checkbox and traceability-table rows flipped"
        status: pass
    human_judgment: false
  - id: D5
    description: "Full-book mdbook build with linkcheck succeeds against the new/changed anchor links (release-automation.md#trusted-publishing, #canonical-publish-order, #credential-history)"
    verification:
      - kind: other
        ref: "mdbook build docs/ (after mdbook-mermaid install docs/ to restore missing, gitignored asset files) -> 'No broken links found'"
        status: pass
    human_judgment: false

duration: "~20 min executor-side"
completed: 2026-08-27
status: complete
---

# Phase 19 Plan 05: Write the trust configuration and credential history down where the next operator will look

**Rewrote `docs/src/appendix/release-automation.md` around crates.io Trusted Publishing — an
eleven-row per-crate trust table, a four-row credential history, the corrected eleven-crate
publish order, and an Operator Guide documenting the real PR-merge release flow — then reconciled
the release checklist, contributor setup guide, and changelog to match.**

## Performance

- **Duration:** ~20 min executor-side
- **Completed:** 2026-08-27
- **Tasks:** 3 (all `type="auto"`), plus a requirements-marking step
- **Files modified:** 5 (4 plan-scoped + `.planning/REQUIREMENTS.md`)

## Accomplishments

- **Task 1:** Replaced `docs/src/appendix/release-automation.md`'s `### Required Secret` section
  (which documented exactly the behavior 19-02 deleted — an absent secret causing a skipped,
  green publish) with a `## Trusted Publishing` section: the OIDC mechanism paragraph; environment
  and protection posture (`crates-io`, `v*.*.*` tag-only deployment policy, no reviewer gate per
  D-08, no environment secrets); the eleven-row `### Per-Crate Trust Configuration` table with
  source directory on every row, including both divergent pairs (`paladin-ai-core` /
  `crates/paladin-core`, `paladin-ai` / workspace root); the four-row `### Credential History`
  table (bootstrap publish, OIDC proof, crates.io revocation, secret deletion), each with a date
  and named actor, and a stated reason it lives here rather than in `SECURITY-EXCEPTIONS.md`
  (that register is mechanically parsed by `scripts/check-advisory-register.sh` against a
  different schema); `### Dry-Run Claim Boundary`; `### Break-Glass Recovery`; and `### Known
  Limits` (token-lifetime margin, Trusted-Publishing-Only mode deliberately not enabled,
  `workflow_dispatch` eligibility untested). Also corrected `## Canonical Publish Order` to the
  real eleven-crate order and removed the non-existent `paladin-cli` reference, and — beyond the
  task's literal text, as a Rule 2 deviation — rewrote the Operator Guide to document the actual
  PR-merge + direct-tag-push release flow, since `make release`'s push to `main` has been dead
  since v0.5.1.
- **Task 2:** Reconciled `docs/src/appendix/release-checklist.md` sections 5 and 6 to the real
  eleven-crate order (package names, including `paladin-herald`) and pointed section 6 at the new
  trust table instead of restating it. Replaced `docs/src/contributing/development-setup.md`'s
  `### Required secret` block (which stated both a now-false claim: that a secret is required, and
  that an absent one causes a silent skip) with a `### Publish credential` block pointing at
  Trusted Publishing, and pointed the publish-order parenthetical at the canonical order rather
  than restating a second, drifting copy. `docs/src/SUMMARY.md` verified byte-unchanged.
- **Task 3:** Added a `### Security` entry under `## [Unreleased]` in `CHANGELOG.md` summarizing
  the migration (no more long-lived repository secret; per-run OIDC mint under the `crates-io`
  environment; both destruction events dated; the silent-skip branch removed, not reworded;
  `paladin-herald` now the eleventh crate; all eleven crates trust-linked), pointing at the
  credential history rather than repeating it. The dated `0.8.1-rc.1`/`0.8.1-rc.2` sections are
  unmodified (verified via `git diff`, additions only).
- Marked `PUB-01` complete in `.planning/REQUIREMENTS.md` (checkbox and traceability row) — the
  one requirement in this plan's frontmatter (`[PUB-05, PUB-01, PUB-04]`) not already closed by an
  earlier 19-0x plan's `requirements-completed`.
- Plan-level verification re-run and passing: `bash scripts/check-doc-config.sh` (150 YAML blocks,
  0 failed), `bash scripts/check-changelogs.sh` (10 crates, all have `CHANGELOG.md`),
  `make check-gates` (all six offline release-gate guards green), `mdbook build docs/` with the
  linkcheck backend (`No broken links found`, after restoring the missing, gitignored
  `mermaid.min.js`/`mermaid-init.js` assets via `mdbook-mermaid install docs/` — a pre-existing
  environment gap unrelated to this plan's content, fixed to unblock verification per Rule 3).

## Task Commits

Each task was committed atomically:

1. **Task 1: Rewrite the release-automation appendix around Trusted Publishing** — `d3faa79b` (docs)
2. **Task 2: Reconcile the release checklist and the contributor setup guide** — `4abacae0` (docs)
3. **Task 3: Record the migration in the changelog** — `55dde8cd` (docs)
4. **Mark PUB-01 complete in REQUIREMENTS.md** — `587854ee` (docs)
5. **Fix: stop duplicating the dead make-release push flow in development-setup.md** — `c30853a0` (docs, Rule 1 deviation, found during plan-level verification)

_No separate plan-metadata commit — this SUMMARY commit is the final commit for this plan._

## Files Created/Modified

- `docs/src/appendix/release-automation.md` — replaced `### Required Secret` with `## Trusted
  Publishing` (environment/protection posture, per-crate trust table, credential history,
  dry-run claim boundary, break-glass recovery, known limits); corrected the canonical publish
  order; rewrote the Operator Guide around the real PR-merge release flow; added a dated note
  under the tooling-comparison table
- `docs/src/appendix/release-checklist.md` — sections 5/6 reconciled to the eleven-crate order,
  pointed at the trust table
- `docs/src/contributing/development-setup.md` — replaced `### Required secret` with
  `### Publish credential`, corrected the publish-order pointer, and (Rule 1 fix, `c30853a0`)
  replaced the duplicated dead push-based "Cutting a release" flow with a pointer to the
  corrected procedure in release-automation.md
- `CHANGELOG.md` — added `### Security` entry under `## [Unreleased]`
- `.planning/REQUIREMENTS.md` — `PUB-01` checkbox and traceability row marked Complete
- `.planning/phases/19-crates-io-trusted-publishing-replace-the-long-lived-registry/19-05-SUMMARY.md` — this file

## Decisions Made

- **The Operator Guide rewrite went beyond Task 1's literal action text**, driven by the
  executor's session context flagging this as a CRITICAL DOC CORRECTION: `make release`'s
  documented single-command flow (bump, tag, push) has been dead since v0.5.1 because the
  "Protect main branch" ruleset blocks a direct push. Leaving the guide as-is would have left a
  published document teaching a release procedure that cannot work, which directly contradicts
  this plan's own criterion 7 (no document may claim a behavior that does not exist). Treated as a
  Rule 2 deviation (missing critical functionality) rather than out-of-scope, since it is the same
  class of defect Task 1 was already fixing for the credential section.
- **PUB-01 closure was attributed to this plan, not 19-01.** 19-01 did the mechanical
  reconciliation (the `release.yml` `CRATES` array now matches `cargo metadata`, done before any
  trust link existed), but no prior plan's `requirements-completed` field named PUB-01. Since the
  requirement's remaining half is exactly what this plan produces — a written, operator-visible
  record of the reconciliation and per-crate trust configuration — this plan is the correct place
  to mark it complete.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 2 - Missing Critical] Operator Guide rewritten to document the real PR-merge release flow**
- **Found during:** Task 1 (release-automation appendix rewrite)
- **Issue:** The pre-existing "Operator Guide: Cutting a Release" section described `make release`
  pushing directly to `main`, which the "Protect main branch" ruleset has blocked since v0.5.1
  (2026-06-04). Both `0.8.1-rc.1` and `0.8.1-rc.2` in this phase were cut through a
  version-bump-branch → PR → merge → direct tag push flow instead — a fact recorded in
  `19-PUBLISH-EVIDENCE.md`'s "Deviation 1" but not yet reflected in the operator-facing doc.
- **Fix:** Rewrote the Operator Guide to document the actual eight-step flow, including the
  `make openapi` baseline-regeneration step `make release` omits, plus known operational caveats
  (the `actions/create-release@v1` re-dispatch requirement to delete a stale release object first,
  and the Build Binaries matrix's systematic, undiagnosed, non-publish-gating failures).
- **Files modified:** `docs/src/appendix/release-automation.md`
- **Verification:** `mdbook build docs/` with linkcheck passes; `bash scripts/check-doc-config.sh`
  exits 0; the section's content matches `19-PUBLISH-EVIDENCE.md`'s "Deviation 1" and "Deviation 2"
  records.
- **Committed in:** `d3faa79b` (Task 1 commit)

**2. [Rule 3 - Blocking] Missing mermaid.min.js/mermaid-init.js assets restored to run mdbook build**
- **Found during:** Plan-level verification (`mdbook build docs/`)
- **Issue:** `mdbook build docs/` failed with "Unable to copy ... mermaid.min.js ... No such file or
  directory." These files are gitignored, generated assets that `.github/workflows/docs.yml`
  restores via `mdbook-mermaid install docs/` before building in CI — a pre-existing environment
  setup gap in this worktree, unrelated to any content this plan changed.
- **Fix:** Ran `mdbook-mermaid install docs/` (the same idempotent step CI runs), which regenerated
  the two gitignored asset files. No source files were affected; `git status --short docs/` showed
  no changes after the install.
- **Files modified:** none (gitignored, generated assets only)
- **Verification:** `mdbook build docs/` → `No broken links found`
- **Committed in:** not committed (gitignored files, no tracked change)

**3. [Rule 1 - Bug] Second copy of the dead make-release push flow in development-setup.md**
- **Found during:** Plan-level verification, after Task 2's commit
- **Issue:** `docs/src/contributing/development-setup.md`'s "Cutting a release" section (outside
  the `### Required secret` block Task 2's action explicitly named) still described `make
  release` pushing directly to `main` in a single command, and restated the same five-step flow
  Task 1 already corrected in `release-automation.md`'s Operator Guide. Left as-is, the two files
  would have disagreed with each other about a currently-dead procedure.
- **Fix:** Replaced the duplicated flow with a pointer to the corrected, single procedure in
  `release-automation.md`'s Operator Guide, following the same "one source of truth, one pointer"
  pattern already used elsewhere in this plan.
- **Files modified:** `docs/src/contributing/development-setup.md`
- **Verification:** `bash scripts/check-doc-config.sh` exits 0 (150 blocks, 0 failed); `mdbook
  build docs/` with linkcheck → `No broken links found` (confirms the new anchor link resolves)
- **Committed in:** `c30853a0`

---

**Total deviations:** 3 auto-fixed (1 missing-critical, 1 blocking, 1 bug)
**Impact on plan:** No scope creep — all three fixes remove false or dead claims from
operator-facing documentation (or unblock verification); none changes what the plan's three tasks
were already required to produce.

## Issues Encountered

None beyond the two deviations recorded above.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- **PUB-01, PUB-04, and PUB-05 (this plan's frontmatter requirements) are all satisfied.** PUB-04
  and PUB-05 were already closed by 19-04 and 19-02 respectively; PUB-01 is closed by this plan.
  Phase 19's full requirement set (PUB-01 through PUB-05) is now complete.
- **The documentation half of the phase's success criterion is met**: an operator who has never
  seen this phase can open `docs/src/appendix/release-automation.md` and learn, without opening
  crates.io, which eleven crates are trust-linked, from which directories, under which workflow
  and environment, when each link was made, when the old credential died and who killed it, and
  the three things this configuration does not guarantee — all without crates.io UI access.
- **No document under `docs/src/` still instructs an operator to configure a crates.io publish
  secret**, and none still claims an absent secret causes the publish job to be skipped while the
  release succeeds — verified by grep across all three edited files plus a full-book `mdbook
  build` with linkcheck.
- **Residual, not addressed by this plan** (carried forward as recorded facts, not blockers): the
  Build Binaries matrix's systematic failure remains undiagnosed; `workflow_dispatch` eligibility
  for Trusted Publishing remains untested; whether any publish-scoped crates.io token exists on
  the account besides the revoked "Paladin" token remains unattested (T-19-21, from 19-04).

---
*Phase: 19-crates-io-trusted-publishing-replace-the-long-lived-registry*
*Completed: 2026-08-27*

## Self-Check: PASSED

- FOUND: `docs/src/appendix/release-automation.md`
- FOUND: `docs/src/appendix/release-checklist.md`
- FOUND: `docs/src/contributing/development-setup.md`
- FOUND: `CHANGELOG.md`
- FOUND: `.planning/phases/19-crates-io-trusted-publishing-replace-the-long-lived-registry/19-05-SUMMARY.md`
- FOUND commit: `d3faa79b` (Task 1)
- FOUND commit: `4abacae0` (Task 2)
- FOUND commit: `55dde8cd` (Task 3)
- FOUND commit: `587854ee` (REQUIREMENTS.md)
- FOUND commit: `c30853a0` (Rule 1 fix)
- Task 1 `<verify>` one-liner: re-run directly, all assertions pass
- Task 2 `<verify>` one-liner: re-run directly, all assertions pass
- Task 3 `<verify>` one-liner: re-run directly, all assertions pass
- `bash scripts/check-doc-config.sh`: 150 YAML block(s) checked, 0 failed (re-run after `c30853a0`)
- `bash scripts/check-changelogs.sh`: 10 publishable crate(s) checked, all have a CHANGELOG.md
- `make check-gates`: all six offline release-gate guards green
- `mdbook build docs/` (linkcheck backend): No broken links found (re-run after `c30853a0`)
- `pre-commit run --all-files`: all 10 hooks Passed (includes `cargo fmt --check` and
  `cargo clippy --workspace -D warnings` over the whole workspace)
