---
phase: 20-release-pipeline-recovery-idempotent-re-runs-and-a-pre-publi
plan: 06
subsystem: infra
tags: [documentation, mdbook, release, crates-io, yank, runbook]

# Dependency graph
requires:
  - phase: 20-03
    provides: "scripts/check-release-consistency.sh's status tokens and remediation shape (the gate failure messages the runbook's §6 quotes verbatim)"
  - phase: 20-05
    provides: "scripts/publish-crates.sh's four outcome states, the no-crate-moved failure message and its exact pointer text (docs/src/appendix/release-recovery.md), consumed verbatim by the new runbook"
provides:
  - "docs/src/appendix/release-recovery.md -- the stuck-halfway runbook: establishing what reached crates.io (User-Agent-scoped registry query loop), reading the outcome table's four states and the deliberate no-crate-moved failure, completing forward via a same-tag re-run (never workflow_dispatch), never deleting/re-uploading a published version, who may yank (crate-owner account, never CI) and the yank register, and a remedy subsection for every check-release-consistency.sh failure token"
  - "docs/src/SUMMARY.md Release Recovery entry, positioned immediately after Release Checklist -- the page is now part of the built mdBook"
  - "reciprocal cross-link callouts in release-checklist.md and release-automation.md pointing at release-recovery.md, satisfying D-13's bidirectional cross-link requirement"
affects: [20-07]

# Tech tracking
tech-stack:
  added: []
  patterns:
    - "Runbook prose quotes the exact status tokens and failure strings the scripts emit (published-now, already-at-this-version, MISMATCH, CI_LOOKUP_FAILED, etc.) rather than paraphrasing them, so an operator can search the page for the text they are already looking at in a job log"
    - "An explicit 'Status: untested' line immediately under the cross-reference callout, to be replaced (not appended to) once plan 20-07's rehearsal runs -- the Phase 18/19 honesty convention applied to documentation rather than a script's exit code"

key-files:
  created:
    - docs/src/appendix/release-recovery.md
  modified:
    - docs/src/SUMMARY.md
    - docs/src/appendix/release-checklist.md
    - docs/src/appendix/release-automation.md

key-decisions:
  - "The yank register table ships with header and separator rows only, no empty data row -- an empty-but-present data row would technically satisfy a loose reading of 'no data rows yet' while still rendering as a stray blank table row in the built book"
  - "Followed release-checklist.md's existing blockquote-callout style for both the new runbook's own top-of-file callout and the reciprocal callouts added to release-checklist.md and release-automation.md, rather than inventing a new convention"
  - "release-automation.md's callout was placed immediately after its opening paragraph and before the Tooling Evaluation section, leaving every historical-record note (the 2026-08 credential-history callout, the Known Limits section, the Trust Configuration table) byte-for-byte untouched per the task's explicit instruction"

patterns-established: []

requirements-completed: [PUBOPS-05]

coverage:
  - id: D1
    description: "docs/src/appendix/release-recovery.md exists, covers all six required sections (establishing registry state, reading the outcome table, completing forward, when forward isn't enough, who may yank + yank register, gate failure remedies), and passes the plan's own content assertion"
    requirement: "PUBOPS-05"
    verification:
      - kind: other
        ref: "python3 content assertion (12 required needles, yank-register header regex, untested-status regex, zero broken relative links) -> 'runbook content OK'"
        status: pass
      - kind: other
        ref: "grep -rnE 'cargo[[:space:]]+yank|/yank' scripts .github/workflows Makefile -> zero matches (no yank automation exists)"
        status: pass
    human_judgment: false
  - id: D2
    description: "The runbook is registered in docs/src/SUMMARY.md and cross-linked bidirectionally from release-checklist.md and release-automation.md; make check-doc-config passes"
    requirement: "PUBOPS-05"
    verification:
      - kind: other
        ref: "python3 cross-link assertion (SUMMARY entry position, bidirectional links, zero broken relative links across all three appendix files) -> 'cross-links OK'"
        status: pass
      - kind: integration
        ref: "make check-doc-config -> 150 YAML block(s) checked, 0 failed"
        status: pass
    human_judgment: false

# Metrics
duration: ~35min (includes one cold cargo-clippy pre-commit-hook cache warm-up, background-executed, not authoring time)
completed: 2026-08-28
status: complete
---

# Phase 20 Plan 06: Release Recovery Runbook Summary

**`docs/src/appendix/release-recovery.md` -- the stuck-halfway runbook answering PUBOPS-05: how to find what actually reached crates.io, read the per-crate outcome table, complete forward by re-running the same tag (never `workflow_dispatch`), never retry a published version, who may yank and how it's recorded, and a remedy for every pre-publish gate failure mode -- registered in the book and cross-linked bidirectionally from both existing release documents.**

## Performance

- **Duration:** ~35 min (2026-08-28T16:41Z – 2026-08-28T16:47Z UTC, wall clock), most of which was a single cold `cargo clippy --workspace --all-targets --all-features -- -D warnings` pre-commit-hook cache warm-up run in the background (same accommodation 20-03 and 20-05's SUMMARYs recorded), not authoring or debugging time
- **Started:** 2026-08-28T16:15:00Z (approx, context load)
- **Completed:** 2026-08-28T16:47:23Z
- **Tasks:** 2
- **Files modified:** 4 (1 created, 3 modified)

## Accomplishments
- `docs/src/appendix/release-recovery.md` created: opens with a cross-reference callout to `release-automation.md` and `release-checklist.md`, then a `Status: untested` line (to be replaced by plan 20-07's rehearsal, per D-14). Six numbered sections in the order the objective specifies: (1) a copy-pasteable `curl` loop over all eleven crates at the tag version, using the required `User-Agent` header and the exact `https://crates.io/api/v1/crates/<name>/<version>` endpoint `scripts/publish-crates.sh` itself queries, plus a pointer to the sparse index as the second place to look when a dependent's publish fails on resolution; (2) how to read the outcome table's four states (`published-now` / `already-at-this-version` / `skipped` / `failed`), why a zero-`published-now` real run fails deliberately (tag already fully published, nothing to recover), and that the workflow's overall red/green is not the authority on publish health (Build Binaries fails on every observed run and does not gate `publish-crates`); (3) completing forward via "Re-run failed jobs" then "Re-run all jobs", why the documented path is a same-tag re-run and not `workflow_dispatch` (the `crates-io` environment's tag-ref deployment policy, the OIDC subject claim, and Phase 19's untested dispatch-eligibility assumption), and the concurrent-run hazard (registry pre-check race, fix: let the in-flight run finish or cancel it); (4) why a published version is never deleted or re-uploaded, and that a bad publish is corrected by a new patch version plus a yank; (5) who may yank (the crate-owner account, never CI -- no workflow/script/Makefile target performs one), the `cargo yank --version <X.Y.Z> <crate-name>` command shape, and a Yank register table (header + separator rows only, columns Version/Crates/Reason/Owner/Date) with an explicit note that entries live here rather than in `SECURITY-EXCEPTIONS.md`, whose mechanically-checked schema is scoped to RustSec advisory suppressions; (6) one subsection per `scripts/check-release-consistency.sh` failure token (`MISMATCH`, `ZERO_PACKAGES`, `MISSING_TAG`, both `CHANGELOG_MISMATCH` variants, `CI_MISMATCH`, `CI_LOOKUP_FAILED`, `MISSING_SHA`, and the four combined tokens), each naming the exact status string and the concrete fix -- including, for `CI_MISMATCH`/no-recorded-run, the two permitted remedies (re-run CI on `main` at that SHA, or fix and re-tag) and an explicit statement that no tag trigger is being added to `ci.yml`.
- `docs/src/SUMMARY.md` gained a `Release Recovery` entry immediately after `Release Checklist` in the `# Appendix` list, so the page is part of the built mdBook.
- `release-checklist.md` gained a second blockquote callout (beside the existing automation callout) pointing at the runbook for a stalled release or gate failure, and its changelog-finalization step now notes the per-crate changelogs are stamped by release tooling rather than hand-edited.
- `release-automation.md` gained a reciprocal callout immediately after its opening paragraph, pointing at the runbook for the failure path; every historical-record note, table and section below it (Trusted Publishing, Credential History, Known Limits) is byte-for-byte unchanged -- confirmed directly against `git diff`, which shows only the four added lines.

## Task Commits

Each task was committed atomically:

1. **Task 1: Write the release-recovery runbook** - `bacb9bed` (docs) -- `docs/src/appendix/release-recovery.md` created; content assertion passes (`runbook content OK`), no yank automation exists in `scripts/`, `.github/workflows/` or `Makefile`
2. **Task 2: Register the runbook in the book and cross-link it from both release documents** - `e8b434d9` (docs) -- `docs/src/SUMMARY.md`, `docs/src/appendix/release-checklist.md`, `docs/src/appendix/release-automation.md`; cross-link assertion passes (`cross-links OK`), `make check-doc-config` passes (150 YAML blocks, 0 failed)

**Plan metadata:** pending (this commit, `docs(20-06): complete plan`, made after this SUMMARY)

## Files Created/Modified
- `docs/src/appendix/release-recovery.md` - the stuck-halfway runbook: registry-state query loop, outcome-table reading guide, forward-recovery procedure, never-retry-a-published-version rule, yank authority and register, and per-failure-mode gate remedies
- `docs/src/SUMMARY.md` - added `Release Recovery` entry immediately after `Release Checklist` in the Appendix list
- `docs/src/appendix/release-checklist.md` - added a recovery callout beside the existing automation callout; extended the changelog-finalization step to mention tooling now stamps per-crate changelogs
- `docs/src/appendix/release-automation.md` - added a reciprocal recovery callout after the opening paragraph; no other line changed

## Decisions Made
- The yank register table ships with header and separator rows only -- no placeholder data row -- so "no data rows yet" is literally true rather than approximately true, and the first real yank has an unambiguous place to append without deleting a placeholder first.
- Followed `release-checklist.md`'s existing blockquote-callout style verbatim for all three callouts added in this plan (the runbook's own top-of-file callout, and the reciprocal callouts in `release-checklist.md` and `release-automation.md`), rather than introducing a new visual convention for cross-referencing between appendix pages.
- Placed `release-automation.md`'s callout immediately after its opening paragraph and before `## Tooling Evaluation`, confirmed by `git diff` to be the only change to that file -- the historical-record notes the task explicitly protects (the 2026-08 credential-history callout, Trust Configuration table, Known Limits section) are untouched.

## Deviations from Plan

None - plan executed exactly as written. The only adjustment was a self-correction before commit: an initial draft of the yank register table included an empty data row (`| | | | | |`) beneath the header and separator; re-reading the acceptance criteria ("a separator row and no data rows") caught this before the Task 1 commit, and the row was removed. This was caught and fixed during authoring, before any commit landed, so it produced no separate fix commit and is not a Rule 1-4 deviation in the plan's sense -- noted here only for completeness.

## Issues Encountered
- The pre-commit hook's `cargo clippy --workspace --all-targets --all-features -- -D warnings` step exceeded the Bash tool's default 2-minute timeout on a cold build cache during the first commit attempt (Task 1), matching the identical tooling/timing accommodation 20-03's and 20-05's SUMMARYs recorded. Resolved the same way: ran `cargo clippy` to completion in the background first (warm-up completed successfully, exit code 0), then retried the commit against the warm cache, which completed in well under a minute; Task 2's commit then also completed quickly against the now-warm cache. No code change was needed -- this is a documentation-only plan with no non-doc source files, so the clippy gate itself found nothing to flag.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness
- `docs/src/appendix/release-recovery.md`'s `Status: untested` line is the explicit hook plan 20-07's rehearsal must replace (not append to) once the induced-failure recovery rehearsal runs and produces `20-RECOVERY-EVIDENCE.md`-style evidence.
- The runbook's §1 registry-query loop, §2 outcome-table reading guide and §6 gate-failure remedies are all written against the exact scripts and messages `scripts/publish-crates.sh` (20-05) and `scripts/check-release-consistency.sh` (20-03) already emit -- no drift to reconcile before the rehearsal.
- No blockers identified for plan 20-07.

---
*Phase: 20-release-pipeline-recovery-idempotent-re-runs-and-a-pre-publi*
*Completed: 2026-08-28*

## Self-Check: PASSED

- FOUND: docs/src/appendix/release-recovery.md
- FOUND: docs/src/SUMMARY.md (Release Recovery entry)
- FOUND: bacb9bed (Task 1 commit)
- FOUND: e8b434d9 (Task 2 commit)
