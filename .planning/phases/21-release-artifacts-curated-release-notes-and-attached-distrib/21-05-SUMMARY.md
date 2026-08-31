---
phase: 21-release-artifacts-curated-release-notes-and-attached-distrib
plan: 05
subsystem: docs
tags: [mdbook, release-automation, documentation, signing, provenance]

# Dependency graph
requires:
  - phase: 21-01
    provides: extract-changelog-section.sh (the curated CHANGELOG.md body source this plan documents)
  - phase: 21-02
    provides: package-release-binaries.sh's expected_binaries_for_target manifest (three binaries, feature set) this plan's artifact inventory cites
  - phase: 21-03
    provides: finalize-release-body.sh's container-image and image-size sections whose exact prose this plan quotes
  - phase: 21-04
    provides: finalize-release-body.sh's SHA256SUMS aggregation and verification-block prose this plan quotes word-for-word
provides:
  - "## Release Notes and Attached Artifacts" section in docs/src/appendix/release-automation.md (body source, artifact inventory, verification, assembly order, signing/provenance deferral)
  - strengthened changelog-finalization prerequisite and concrete post-release verification steps in docs/src/appendix/release-checklist.md
  - body-finalizing job rebuild-not-append re-run subsection, plus a cross-reference between the two CHANGELOG-missing failure paths, in docs/src/appendix/release-recovery.md
  - COVERAGE.md re-verified against the shipped tree (no drift found)
affects: [21-06]

# Tech tracking
tech-stack:
  added: []
  patterns:
    - "Consumer-facing documentation quotes verification commands word-for-word from the script that composes them (finalize-release-body.sh), rather than restating them independently, so the two cannot drift apart"
    - "Signing/provenance deferral stated as a plain negative in consumer docs (checksums attached, no signature, no build attestation) rather than silence -- same failure shape as the project's own advisory-only-SAST framing"

key-files:
  created: []
  modified:
    - docs/src/appendix/release-automation.md
    - docs/src/appendix/release-checklist.md
    - docs/src/appendix/release-recovery.md

key-decisions:
  - "Used the existing docs/src/deployment/docker.md convention (ghcr.io/your-org/paladin placeholder) for the digest-pinned pull example rather than hardcoding this repository's actual GHCR path, for consistency with the rest of the book."
  - "Cross-referenced release-automation.md's new #body-source and #artifact-inventory anchors from release-checklist.md instead of restating the inventory table or the body-source rule a second time, per the plan's explicit 'cross-reference rather than duplicate' instruction."
  - "Added the release-creation-path / pre-publish-gate CHANGELOG failure cross-reference as prose inside the existing 'no section for this version' entry rather than a new heading, keeping the CHANGELOG_MISMATCH grep count at exactly 2 (unchanged) as the plan's acceptance criteria require."

requirements-completed: [ARTIFACT-01, ARTIFACT-02, ARTIFACT-05, ARTIFACT-06]

coverage:
  - id: D1
    description: "release-automation.md states the release body is the root CHANGELOG.md section for the tag's version, names the exact remedy printed on a missing section, and states there is no alternate body source"
    requirement: "ARTIFACT-01"
    verification:
      - kind: other
        ref: "grep -q 'extract-changelog-section.sh' docs/src/appendix/release-automation.md; grep -q 'make release VERSION=X.Y.Z' docs/src/appendix/release-automation.md -- both verified locally"
        status: pass
    human_judgment: false
  - id: D2
    description: "Artifact inventory table names all three binaries, the cli,web-server feature set (plus vendored-openssl on aarch64), and the four matrix targets, cross-referencing expected_binaries_for_target as the single narrowing point"
    requirement: "ARTIFACT-02"
    verification:
      - kind: other
        ref: "grep -c 'paladin-cli' and grep -c 'paladin-server' docs/src/appendix/release-automation.md both >= 1; grep -q 'cli,web-server' -- verified locally"
        status: pass
    human_judgment: false
  - id: D3
    description: "Verification instructions (aggregated SHA256SUMS one-command check, macOS shasum variant, digest-pinned image pull) appear in the docs, quoted word-for-word from finalize-release-body.sh's composed text; release-checklist.md's post-release step names the same checks plus the three-binaries-per-target reminder"
    requirement: "ARTIFACT-05"
    verification:
      - kind: other
        ref: "grep -q 'sha256sum -c SHA256SUMS' and grep -q 'shasum -a 256 -c' in release-automation.md; grep -q 'SHA256SUMS' in release-checklist.md -- all verified locally"
        status: pass
    human_judgment: false
  - id: D4
    description: "Signing/provenance decision recorded in consumer-facing docs with reasoning, naming actions/attest-build-provenance as the candidate, stating plainly that artifacts carry checksums but no signature and no build attestation (T-21-22)"
    requirement: "ARTIFACT-05"
    verification:
      - kind: other
        ref: "grep -q 'attest-build-provenance' and a sentence containing 'no signature and no build attestation' in docs/src/appendix/release-automation.md -- verified locally"
        status: pass
    human_judgment: false
  - id: D5
    description: "release-checklist.md reflects every operator-visible change: root changelog section as a hard release-run prerequisite, and concrete post-release verification steps replacing manual eyeballing"
    requirement: "ARTIFACT-01"
    verification:
      - kind: other
        ref: "docs/src/appendix/release-checklist.md sections 2 and 8 read and verified locally against the plan's <action> text"
        status: pass
    human_judgment: false
  - id: D6
    description: "release-recovery.md records the body-finalizing job's rebuild-not-append re-run semantics (safe to re-run, picks up a previously-succeeded leg's section, idempotent asset uploads) and cross-references the sibling CHANGELOG failure on the release-creation path, without renumbering or changing any existing failure code"
    requirement: "ARTIFACT-06"
    verification:
      - kind: other
        ref: "grep -qi 'finalize' docs/src/appendix/release-recovery.md; grep -c 'CHANGELOG_MISMATCH' docs/src/appendix/release-recovery.md == 2 (unchanged) -- both verified locally"
        status: pass
    human_judgment: false
  - id: D7
    description: "COVERAGE.md's INTEGRATE rows name only capabilities the shipped tree actually exercises, and no OPT-OUT row names one it does"
    verification:
      - kind: other
        ref: "Cross-checked every INTEGRATE row (gh release view/edit/upload/download, extract-changelog-section.sh, docker build-push-action/metadata-action/pull/inspect) against .github/workflows/release.yml and scripts/*.sh; cross-checked every OPT-OUT row (upload_url consumption, release delete, release list, re-run, reactions, registry catalog/manifest-delete/visibility) via targeted grep -- no drift found, no correction needed"
        status: pass
    human_judgment: false
  - id: D8
    description: "The trigger-policy table in branching-model.md is untouched, and this plan's diff touches only the three appendix files"
    verification:
      - kind: other
        ref: "git diff --name-only <base>..HEAD returns exactly docs/src/appendix/release-automation.md, docs/src/appendix/release-checklist.md, docs/src/appendix/release-recovery.md -- verified locally, no branching-model.md entry"
        status: pass
    human_judgment: false

duration: ~25min
completed: 2026-08-31
status: complete
---

# Phase 21 Plan 05: Release Automation, Checklist and Recovery Documentation Summary

**Documented the release pipeline's real behavior in consumer-facing mdbook chapters — the curated `CHANGELOG.md` body source with its named failure remedy, the three-binary artifact inventory, the verification commands quoted word-for-word from `finalize-release-body.sh`, and a plainly-stated signing/provenance deferral naming `actions/attest-build-provenance` as the future candidate.**

## Performance

- **Duration:** ~25 min
- **Completed:** 2026-08-31
- **Tasks:** 2 (both auto)
- **Files modified:** 3

## Accomplishments

- Added a `## Release Notes and Attached Artifacts` section to `docs/src/appendix/release-automation.md`, placed after the operator guide and before Trusted Publishing, covering body source, artifact inventory, verification, assembly order, and the signing/provenance deferral.
- Stated plainly, in a consumer's own terms, that attached artifacts carry checksums but no signature and no build attestation — closing ARTIFACT-05's "decided and recorded with its reasoning" clause in documentation a consumer actually reads, not only in `.planning/`.
- Strengthened `release-checklist.md`'s changelog-finalization step (root section is now a hard release-run prerequisite) and replaced manual post-release eyeballing with the concrete checks a consumer or operator can run: `SHA256SUMS` verification, digest-pinned image pull, body-vs-changelog confirmation, and a three-binaries-per-target reminder.
- Added a `release-recovery.md` subsection recording the body-finalizing job's rebuild-not-append re-run semantics, and cross-referenced the sibling `CHANGELOG` failure the extraction step raises on the release-creation path from the existing gate-failure entry — without renumbering or changing any existing failure code.
- Re-verified `COVERAGE.md` against the shipped tree: every `INTEGRATE` row corresponds to a real invocation in `.github/workflows/release.yml` or `scripts/*.sh`, and every `OPT-OUT` row names a capability genuinely unused (confirmed `upload_url` is parsed internally by `create-or-reuse-release.sh` but never consumed by any workflow job — the OPT-OUT reasoning holds). No drift found; no correction needed.

## Task Commits

Each task was committed atomically:

1. **Task 1: Release automation — body source, artifact inventory, verification, signing decision** - `13eedb56` (docs)
2. **Task 2: Operator checklist and recovery notes** - `fca1daf1` (docs)

**Plan metadata:** committed alongside this SUMMARY (worktree mode — STATE.md/ROADMAP.md updates deferred to the orchestrator).

## Files Created/Modified

- `docs/src/appendix/release-automation.md` — New `## Release Notes and Attached Artifacts` section (body source, artifact inventory table, verification commands, assembly order, signing/provenance deferral)
- `docs/src/appendix/release-checklist.md` — §2 changelog-finalization prerequisite strengthened; §8 post-release verification replaced with concrete checks and a three-binaries-per-target reminder
- `docs/src/appendix/release-recovery.md` — New "Re-running the body-finalizing job" subsection under §3; existing `CHANGELOG_MISMATCH` "no section" entry extended with a cross-reference to the sibling release-creation-path failure

## Decisions Made

- Used the existing `docs/src/deployment/docker.md` placeholder convention (`ghcr.io/your-org/paladin`) for the digest-pinned pull example, for consistency with the rest of the book rather than hardcoding this repository's actual GHCR path.
- Cross-referenced `release-automation.md`'s new `#body-source` and `#artifact-inventory` anchors from `release-checklist.md` instead of restating either the inventory table or the body-source rule, per the plan's explicit instruction.
- Added the release-creation-path / pre-publish-gate `CHANGELOG` failure cross-reference as prose inside the existing "no section for this version" entry rather than a new heading, keeping the `CHANGELOG_MISMATCH` grep count at exactly 2 (unchanged), matching the plan's acceptance criteria.

## Deviations from Plan

None — plan executed exactly as written. `COVERAGE.md` verification (required by Task 1's `<action>`) found no drift and required no correction.

## Issues Encountered

- The first `git commit` for Task 1 was killed at the default 2-minute Bash timeout mid-way through the pre-commit hook's `cargo clippy` pass (no commit landed — confirmed via `git log`/`git status`). Re-ran the identical commit with a 600000ms timeout per this executor's own instructions; it completed cleanly.

## User Setup Required

None — no external service configuration required.

## Next Phase Readiness

- ARTIFACT-05's "decided and recorded with its reasoning" clause is now satisfied in consumer-facing documentation, matching the shipped pipeline from plans 21-01 through 21-04.
- Plan 21-06 (the D-14 end-to-end rehearsal) is the remaining backstop-verification item this phase's prior SUMMARYs (21-03 D5, 21-04 D5) explicitly deferred to — this plan's documentation work does not touch or depend on that rehearsal, and none of its content will need revision regardless of the rehearsal's outcome (the docs describe the pipeline's designed behavior, already proven at the script level by 84 assertions in `tests/scripts/finalize-release-body_test.sh`).
- No blockers.

---
*Phase: 21-release-artifacts-curated-release-notes-and-attached-distrib*
*Completed: 2026-08-31*

## Self-Check: PASSED

- FOUND: docs/src/appendix/release-automation.md
- FOUND: .planning/phases/21-release-artifacts-curated-release-notes-and-attached-distrib/21-05-SUMMARY.md
- FOUND commit: 13eedb56 (Task 1)
- FOUND commit: fca1daf1 (Task 2)
