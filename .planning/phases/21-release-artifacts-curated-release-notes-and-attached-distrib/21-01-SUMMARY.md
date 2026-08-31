---
phase: 21-release-artifacts-curated-release-notes-and-attached-distrib
plan: 01
subsystem: infra
tags: [github-actions, release, changelog, bash, python3, shellcheck]

# Dependency graph
requires:
  - phase: 20-release-pipeline-hardening-tag-consistency-and-publish-recovery
    provides: create-or-reuse-release.sh's --body-file contract and its upload_url= stdout output
provides:
  - scripts/extract-changelog-section.sh — extracts one version's curated CHANGELOG.md section, byte-for-byte, for use as a GitHub release body
  - A committed regression harness proving the extraction boundary/adjacency/escaping/encoding contract with no network access
  - create-release job in .github/workflows/release.yml now sources its release body from the curated changelog section instead of a commit-log dump plus a hardcoded, never-pushed :latest pull instruction
affects: [21-02, 21-03, 21-04, 21-06]

# Tech tracking
tech-stack:
  added: []
  patterns:
    - "python3 -<<'PY' heredoc for text-processing boundary scans, using re.escape on operator-supplied input before building a match pattern (mirrors check-release-consistency.sh's Clause 2)"
    - "Sourcing seam via a *_LIB_ONLY env var so a script's main function can be exercised directly from its regression harness without executing side effects"

key-files:
  created:
    - scripts/extract-changelog-section.sh
    - tests/scripts/extract-changelog-section_test.sh
  modified:
    - .github/workflows/release.yml

key-decisions:
  - "Boundary regex is a byte-exact mirror of check-release-consistency.sh's Clause 2 heading_re, cross-referenced by file:line comment in both places, so the two implementations can never silently diverge on where a section starts."
  - "No alternate body source exists in the script or the calling workflow step — a missing ## [X.Y.Z] section is a hard, named failure (D-01), never a silent fallback to a commit-log summary."
  - "The extracted section reaches the GitHub API only as a --body-file path, never $GITHUB_OUTPUT-interpolated or command-line-concatenated, closing the injection surface a changelog line matching a heredoc delimiter would otherwise open."

patterns-established:
  - "Scratch-fixture regression testing: edge-case coverage (adjacency, escaping, EOF, UTF-8) is built entirely from mktemp-scratch CHANGELOG.md fixtures, never by mutating the real in-tree CHANGELOG.md, with a git status --porcelain baseline/after comparison closing every test run."

requirements-completed: [ARTIFACT-01, ARTIFACT-03]

coverage:
  - id: D1
    description: "extract-changelog-section.sh extracts a version's curated section from CHANGELOG.md byte-for-byte, exits 0 for a heading-only (empty) section, and fails loudly with a named remedy message for a missing version"
    requirement: "ARTIFACT-01"
    verification:
      - kind: unit
        ref: "tests/scripts/extract-changelog-section_test.sh (assertions 1-4, real-tree extraction/heading-only/missing-version/end-to-end payload)"
        status: pass
    human_judgment: false
  - id: D2
    description: "Version matching is boundary-correct and metacharacter-safe: adjacency (0.8.1 vs 0.8.1-rc.2, 0.8.1 vs 0.8.10), a trailing heading date suffix, regex-escaping of . and -, EOF-terminated last section, no-gap heading adjacency, Unreleased exclusion, and byte-for-byte UTF-8 survival"
    requirement: "ARTIFACT-01"
    verification:
      - kind: unit
        ref: "tests/scripts/extract-changelog-section_test.sh (Task 2 boundary/adjacency/escaping/encoding matrix, 12 assertions)"
        status: pass
    human_judgment: false
  - id: D3
    description: "release.yml's create-release job body is sourced from the curated changelog section — the commit-log body generation and the hardcoded :latest pull instruction are removed"
    requirement: "ARTIFACT-03"
    verification:
      - kind: other
        ref: "grep -c 'git log --pretty' .github/workflows/release.yml == 0; grep -c ':latest' .github/workflows/release.yml == 0; grep -c 'extract-changelog-section.sh' .github/workflows/release.yml == 1"
        status: pass
    human_judgment: false
  - id: D4
    description: "On a live tag push, the created release's body renders as the curated section with no commit-subject list and no unreachable pull instruction"
    human_judgment: true
    rationale: "Backstop truth per plan frontmatter — nothing local proves the rendered body on a real GitHub release; only the D-14 rehearsal in plan 21-06 provides that evidence."

duration: 50min
completed: 2026-08-31
status: complete
---

# Phase 21 Plan 01: Curated CHANGELOG.md Release Body Summary

**`scripts/extract-changelog-section.sh` extracts one version's curated `CHANGELOG.md` section byte-for-byte as the GitHub release body, replacing a 1,014-commit-subject dump and a never-pushed `:latest` pull instruction, with a 16-assertion local regression harness covering adjacency, escaping, EOF, and UTF-8 edge cases.**

## Performance

- **Duration:** 50 min (Task 1 commit 13:01:20 UTC → Task 2 commit 13:51:08 UTC)
- **Tasks:** 2
- **Files modified:** 3 (2 created, 1 modified)

## Accomplishments
- `scripts/extract-changelog-section.sh` extracts a version's `## [X.Y.Z]` section from the root `CHANGELOG.md` verbatim, mirroring `check-release-consistency.sh`'s Clause 2 boundary regex so the two implementations can never silently diverge
- A missing section is a hard, named failure with a remedy message (`run make release VERSION=X.Y.Z ... before tagging`) — there is no alternate body source
- `.github/workflows/release.yml`'s `create-release` job now writes the extracted section to `${RUNNER_TEMP}/release-body.md` and passes it to `create-or-reuse-release.sh --body-file`, with the old `Generate changelog` step (commit-log body + hardcoded `:latest` pull block) removed entirely
- A 16-assertion regression harness (`tests/scripts/extract-changelog-section_test.sh`) proves the contract locally with no network access: real-tree extraction, heading-only emptiness (D-02), missing-version failure, end-to-end payload fidelity against a stubbed `gh`, and a full boundary/adjacency/escaping/encoding edge matrix

## Task Commits

Each task was committed atomically:

1. **Task 1: End-to-end curated body — CHANGELOG.md section reaches the release payload** - `3504ece9` (feat) — completed by a prior executor, verified present and its test still passing before this continuation began
2. **Task 2: Boundary and version-matching edge matrix** - `8e8b5cdb` (test)

## Files Created/Modified
- `scripts/extract-changelog-section.sh` - Extracts a version's curated section from `CHANGELOG.md`, byte-for-byte, via a `python3` heredoc boundary scan (Task 1)
- `tests/scripts/extract-changelog-section_test.sh` - Regression harness: 4 Task 1 assertions (real-tree extraction, heading-only emptiness, missing-version failure, end-to-end payload fidelity) plus 12 Task 2 assertions (adjacency ×2, dated heading, escaping, EOF-terminated section, no-gap adjacency, Unreleased exclusion, UTF-8 byte-for-byte survival) — 16 total
- `.github/workflows/release.yml` - `create-release` job's `Generate changelog` step replaced with `Extract changelog section`, keeping `id: changelog` so the downstream `Create or reuse release` step's `RELEASE_BODY_FILE` reference stays valid (Task 1)

## Decisions Made
None beyond what the plan specified — Task 2 executed exactly as written, and every new assertion passed against the Task 1 implementation on the first run with no fix required.

## Deviations from Plan

None - plan executed exactly as written.

## Issues Encountered

**Continuation context:** This plan was resumed after a prior executor completed Task 1 and crashed mid-Task-2. Task 1's commit (`3504ece9`) and its regression test were verified present and green before this continuation wrote any code; the crashed agent's uncommitted Task 2 edits had already been discarded and were rewritten fresh from the plan's `<action>` rather than reconstructed.

**Pre-commit hook duration:** The repo's pre-commit hook runs `cargo clippy` over the full workspace, which took roughly 2 minutes on the Task 2 commit — a routine slow-hook wait, not a failure, retried once with an extended timeout after the first attempt hit the default 2-minute Bash tool timeout.

## Next Phase Readiness
ARTIFACT-01 is closed at source and ARTIFACT-03's body half is closed. `create-or-reuse-release.sh`'s `--body-file` and `upload_url=` stdout contracts are unchanged, so plans 21-02/21-03/21-04 (which build the artifact-attachment steps) can rely on this plan's `Extract changelog section` step output without further changes here. The live-tag rendering truth (D4 above) remains a backstop pending the D-14 rehearsal in plan 21-06.

---
*Phase: 21-release-artifacts-curated-release-notes-and-attached-distrib*
*Completed: 2026-08-31*

## Self-Check: PASSED

- FOUND: .planning/phases/21-release-artifacts-curated-release-notes-and-attached-distrib/21-01-SUMMARY.md
- FOUND: scripts/extract-changelog-section.sh
- FOUND: tests/scripts/extract-changelog-section_test.sh
- FOUND: commit 3504ece9 (Task 1)
- FOUND: commit 8e8b5cdb (Task 2)
