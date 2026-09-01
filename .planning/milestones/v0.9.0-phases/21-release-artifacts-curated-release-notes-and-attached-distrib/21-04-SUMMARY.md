---
phase: 21-release-artifacts-curated-release-notes-and-attached-distrib
plan: 04
subsystem: infra
tags: [github-actions, release, bash, gh-cli, checksums, sbom, shellcheck]

# Dependency graph
requires:
  - phase: 21-03
    provides: scripts/finalize-release-body.sh's marker-based truncate-and-rebuild composer, the FINALIZE_RELEASE_BODY_LIB_ONLY sourcing seam, and the terminal finalize-release-body workflow job this plan extends in place
provides:
  - scripts/finalize-release-body.sh's aggregate_checksums function -- downloads every already-uploaded *.tar.gz release asset back from the release and writes one SHA256SUMS file covering exactly what's visible, uploaded with --clobber
  - compose_release_body's two new sections (Downloads and verification, SBOM) in the fixed declared order: container image, downloads and verification, SBOM, image size
  - five new script flags (--aggregate-checksums, --assets-dir, --assets-file, --sums-name, --sbom-asset)
  - .github/workflows/release.yml's finalize-release-body job invoking the script with --aggregate-checksums always and --sbom-asset only when the sbom job succeeded
  - tests/scripts/finalize-release-body_test.sh's extended gh stub (release download, release view --json assets, an asset-registry-tracking release upload) and 84 total assertions (44 inherited + 40 new)
affects: [21-05, 21-06]

# Tech tracking
tech-stack:
  added: []
  patterns:
    - "Checksum aggregation reads the release back after upload, never from a build-time list -- SHA256SUMS attests to what a consumer will actually receive"
    - "Asset inventory always read from gh release view --json assets (never assembled from expectations), so a failed leg's asset is never advertised"
    - "Aggregation runs before the asset list is read within the same invocation, so the sums file it just uploaded appears in its own run's published inventory"
    - "A verification block is gated on this invocation actually having attached a sums file, not merely on a sums-name flag being set -- naming a file the run didn't attach is worse than no instructions"
    - "Downloads-section rendering deduplicates by name via an associative-array 'seen' set, independent of whether the asset list itself already came pre-sorted or pre-deduplicated"

key-files:
  created: []
  modified:
    - scripts/finalize-release-body.sh
    - tests/scripts/finalize-release-body_test.sh
    - .github/workflows/release.yml

key-decisions:
  - "compose_release_body's signature grew from 5 to 8 positional args (CURATED DIGEST IMAGE_REF SIZE_MB ASSET_LIST SUMS_NAME SBOM_ASSET OUTPUT) rather than a flags-object or associative array -- kept consistent with the existing positional-argument style the plan 21-03 function already established, and the test harness's compose_to helper defaults the three new trailing params to empty strings so every pre-existing case1-13 call site kept working unmodified."
  - "ASSET_LIST and the verification-gating SUMS_NAME are passed as separate parameters rather than folded together -- separating 'what assets exist' from 'was a sums file attached in this run' is what lets the downloads section render assets with no verification block (case 25) and is the concrete mechanism behind the plan's prohibition against naming a file the run didn't attach."
  - "The asset list is populated via gh release view --json assets on every invocation, not only when --aggregate-checksums is set -- ARTIFACT-03's inventory half (an asset a failed leg never uploaded is never listed) benefits every finalize run, not just checksum-aggregating ones; --assets-file bypasses this call entirely for the test harness and uses its content verbatim, unsorted, per the plan's own 'unless --assets-file was supplied' wording."
  - "aggregate_checksums takes GH as an explicit first parameter rather than re-resolving ${GH_BIN:-gh} internally -- keeps the function's only side-effect-producing dependency explicit and matches compose_release_body's pattern of taking every external input as a parameter."
  - "_frb_build_downloads_section deduplicates asset names with a bash associative array ('seen' set) rather than relying on the caller to pre-deduplicate -- proven directly by case 24 (an asset list containing the sums file name twice renders it once)."
  - "No shared sha256_cmd helper was extracted between package-release-binaries.sh and finalize-release-body.sh -- added a private, identically-shaped _frb_sha256_cmd instead, matching the plan's 'existing sha256_cmd-style availability check' wording (style match, not literal code sharing) and avoiding a cross-script dependency neither file currently has."
  - "Split the single implementation pass into two atomic commits matching the plan's two tasks by reconstructing the test file's intermediate state (cases 1-18) for the Task 1 commit, then restoring the full file (cases 1-25) for the Task 2 commit -- both states were independently run through the full harness, shellcheck, and make check-gates/test-shell-guards before their respective commit."

requirements-completed: [ARTIFACT-03, ARTIFACT-05]

coverage:
  - id: D1
    description: "aggregate_checksums downloads every already-uploaded *.tar.gz release asset back from the release via gh release download, writes one SHA256SUMS file (bare filenames, LC_ALL=C sorted, equal digests never collapse to one entry) when one or more archives were found, and uploads it with --clobber; zero downloaded archives is a handled non-failure -- no file written, no upload attempted, no verification section composed"
    requirement: "ARTIFACT-05"
    verification:
      - kind: unit
        ref: "tests/scripts/finalize-release-body_test.sh -- cases 14 (3-archive round trip, sorted lines, --clobber upload), 15 (zero-archive non-failure), 19 (equal-content archives stay as two lines), 20 (single-archive one-line file), 22 (re-run byte-identical, second --clobber upload not a failure), 23 (stray non-*.tar.gz file contributes no line)"
        status: pass
    human_judgment: false
  - id: D2
    description: "The composed body's downloads section lists exactly the asset names gh release view --json assets reports for the release, in LC_ALL=C sorted order, with duplicates collapsed to one entry -- so a leg that never uploaded an asset is never listed (ARTIFACT-03's inventory half)"
    requirement: "ARTIFACT-03"
    verification:
      - kind: unit
        ref: "tests/scripts/finalize-release-body_test.sh -- cases 16 (LC_ALL=C sorted order from an out-of-order registry fixture), 21 (hyphen+digit filenames sort under LC_ALL=C regardless of creation/download order, cmp byte-identical), 24 (a duplicated asset name in the list renders once, not twice), 25 (assets exist but no sums file attached -> list renders, no verification block)"
        status: pass
    human_judgment: false
  - id: D3
    description: "The one-command verification block (Linux sha256sum -c / macOS shasum -a 256 -c) is emitted if and only if this invocation actually attached a sums file -- never merely because a sums-name flag was passed -- and the sums file itself appears in the asset list because aggregation runs before the list is read"
    requirement: "ARTIFACT-05"
    verification:
      - kind: unit
        ref: "tests/scripts/finalize-release-body_test.sh -- case 14 (SHA256SUMS appears in the published asset list, both verification-command forms present), case 15 (no verification section when zero archives were downloaded), case 25 (existing assets with no sums attached in this run -> no verification block)"
        status: pass
    human_judgment: false
  - id: D4
    description: "The SBOM section, emitted only when --sbom-asset names a document, states the attached document is a CycloneDX SBOM for the root paladin-ai package only, not the eleven-crate workspace (D-12); the per-crate changelogs are never inlined or linked in the body (D-03)"
    requirement: "ARTIFACT-05"
    verification:
      - kind: unit
        ref: "tests/scripts/finalize-release-body_test.sh -- cases 14 and 17 (SBOM section names the attached document and states root paladin-ai package scope, never claims workspace-wide coverage; per-crate CHANGELOG text never appears)"
        status: pass
    human_judgment: false
  - id: D5
    description: "The .github/workflows/release.yml finalize-release-body job invokes the script with --aggregate-checksums unconditionally and --sbom-asset only when needs.sbom.result == 'success', built to match the sbom job's own uploaded filename exactly (paladin-<version>.cdx.json), with no new Action and no permission change beyond the job's existing contents: write"
    requirement: "ARTIFACT-03"
    verification:
      - kind: other
        ref: "grep -c 'aggregate-checksums' .github/workflows/release.yml == 1; grep -c 'sbom-asset' .github/workflows/release.yml == 1; grep -c 'gh release upload' .github/workflows/release.yml == 2 (unchanged -- aggregation's own upload happens inside the script); ./scripts/check-workflow-triggers.sh and make check-gates both exit 0 -- all verified locally"
        status: pass
      - kind: other
        ref: "Live branching (a real sbom-job failure on an actual GitHub Actions run genuinely omitting the SBOM section, and a real consumer downloading assets and running sha256sum -c against a live release) is untestable offline -- explicitly the D-14 rehearsal's job (plan 21-06), consistent with this phase's backstop-verification item"
        status: unknown
    human_judgment: true
    rationale: "This plan proves the finalize job's local aggregation/inventory/SBOM-scope logic exhaustively (84 assertions) and confirms the workflow YAML's flag-passing and success-gating are wired as specified. Whether a real live release run actually produces a downloadable, verifiable asset set end-to-end is the D-14 rehearsal's responsibility (plan 21-06) -- this plan's own must_haves.truths entry for that exact statement is explicitly marked 'backstop' verification, not proven here."

duration: ~50min
completed: 2026-08-31
status: complete
---

# Phase 21 Plan 04: Make Attached Artifacts Verifiable in One Command Summary

**`scripts/finalize-release-body.sh` gains `aggregate_checksums` (a single `SHA256SUMS` covering every archive actually visible on the release, downloaded back and re-uploaded via `--clobber`), a `### Downloads and verification` section sourced from the release's real asset list, and a `### SBOM` section stating the attached document covers only the root `paladin-ai` package -- closing ARTIFACT-05 and the asset-inventory half of ARTIFACT-03.**

## Performance

- **Duration:** ~50 min
- **Started:** 2026-08-31T14:52:00Z
- **Completed:** 2026-08-31T15:12:00Z
- **Tasks:** 2 (1 tracer, 1 auto)
- **Files modified:** 3

## Accomplishments

- `aggregate_checksums` downloads every already-uploaded `*.tar.gz` release asset back from the release itself (never from a build-time list), computes a `sha256sum`/`shasum -a 256` digest per archive with bare filenames, sorts under `LC_ALL=C` by filename so re-runs are byte-stable and equal digests never collapse, and uploads the result as `SHA256SUMS` via `--clobber`. Zero downloaded archives is a handled, non-failing outcome -- no sums file written, no upload attempted, no verification section composed.
- The composed body's new `### Downloads and verification` section always reads the real asset list from `gh release view --json assets` (`--assets-file` bypasses this for the test harness), sorted under `LC_ALL=C` with duplicate names collapsed to a single entry -- so an asset a failed leg never uploaded is never advertised. The one-command verification block (`sha256sum -c SHA256SUMS` / macOS `shasum -a 256 -c SHA256SUMS`) appears only when *this invocation* actually attached a sums file, never merely because a name was configured.
- Aggregation runs before the asset list is read within `finalize_release_body_main`, so `SHA256SUMS` itself shows up in its own run's published inventory -- proven directly by the test harness's persistent per-scratch-dir asset registry, not merely asserted by reading the source.
- A new `### SBOM` section, emitted only when `--sbom-asset` names a document, states in one sentence that it is a CycloneDX SBOM for the root `paladin-ai` package only, not the eleven-crate workspace (D-12). `compose_release_body`'s section order is now fixed as container image, downloads and verification, SBOM, image size.
- `.github/workflows/release.yml`'s `finalize-release-body` job now passes `--aggregate-checksums` unconditionally and `--sbom-asset` built to match the `sbom` job's own uploaded filename exactly, supplied only when that job succeeded -- no new Action, no permission change (rides the job's existing `contents: write`).
- `tests/scripts/finalize-release-body_test.sh` grew from 44 to 84 assertions: an extended `gh` stub answering `release download`, `release view --json assets`, and an asset-registry-tracking `release upload`; 5 new cases (14-18) proving the full aggregation round trip, the zero-archive non-failure path, stable asset ordering, SBOM content, and double-run byte-identity across aggregation + assets + SBOM together; and 7 new edge-matrix cases (19-25) proving equal-digest handling, single-archive output, `LC_ALL=C` filename ordering independent of download order, re-run idempotency, stray-file filtering, list deduplication, and the no-verification-without-attachment rule.

## Task Commits

Each task was committed atomically:

1. **Task 1: End-to-end verifiable download -- aggregate, attach, and publish the instructions** (tracer) - `fea0bd33` (feat)
2. **Task 2: Aggregation edge matrix -- empty input, equal digests, re-run stability** - `04df2994` (test)

**Plan metadata:** committed alongside this SUMMARY (worktree mode -- STATE.md/ROADMAP.md updates deferred to the orchestrator).

## Files Created/Modified

- `scripts/finalize-release-body.sh` - Adds `aggregate_checksums`, `_frb_sha256_cmd`, `_frb_build_downloads_section`, `_frb_build_sbom_section`; extends `compose_release_body`'s signature and `finalize_release_body_main`'s flag parsing with the five new flags
- `tests/scripts/finalize-release-body_test.sh` - Extends `compose_to` and `write_gh_stub`; adds 12 new cases (14-25, 40 new assertions) covering both plan tasks' full `<behavior>` matrix
- `.github/workflows/release.yml` - `finalize-release-body` job invokes the script with `--aggregate-checksums` and a conditionally-populated `--sbom-asset`

## Decisions Made

- `compose_release_body`'s positional signature grew from 5 to 8 args rather than switching to a flags object -- consistent with the existing style, and `compose_to`'s three new trailing parameters default to empty strings so all of plan 21-03's `case1`-`case13` calls kept working unmodified.
- Asset-list population via `gh release view --json assets` runs on every invocation (not gated on `--aggregate-checksums`) -- the inventory-accuracy benefit (ARTIFACT-03) applies to every finalize run, not only checksum-aggregating ones.
- The verification block is gated on `sums_attached_name` (set only when *this run's own* aggregation call actually wrote the file), not on the `--sums-name` flag's mere presence -- the concrete mechanism satisfying the plan's prohibition against publishing instructions for a file the run didn't attach.
- Split the single implementation into two atomic commits by reconstructing the test file's intermediate state: staged the file at the point covering only Task 1's cases (1-18), ran the full harness/shellcheck/`make check-gates` against that state, committed, then restored the full file (cases 1-25) and re-verified before the Task 2 commit. No production-code change was needed between the two commits -- Task 1's `aggregate_checksums`/`_frb_build_downloads_section` already satisfied every Task 2 edge-case assertion.

## Deviations from Plan

None -- plan executed exactly as written, aside from the implementation-detail decisions recorded above (all necessary to satisfy the plan's own literal acceptance criteria and task-commit structure, not functional changes to what was asked).

## Issues Encountered

- The `gh` stub's `release upload` handler initially read the uploaded file path from `$3` (matching a mental model of `release upload PATH --clobber`), but the real invocation is `release upload TAG PATH --clobber` -- `$3` is the tag, `$4` the path. This made two upload-count assertions read `0` instead of the expected count on the first harness run. Fixed the stub to read `$4`; both assertions (and 82 others) passed on re-run.
- `git commit` on Task 1 was killed by the harness's default 2-minute Bash timeout mid-way through the pre-commit hook's `cargo clippy` pass (exit 143, no commit landed -- confirmed via `git log`/`git status` before retrying). Re-ran the identical commit with a 600000ms timeout per this executor's own instructions; it completed cleanly the second time.
- `make test-shell-guards` failed once with `check-workflow-suppressions_test.sh`'s "no mutation" assertion firing on `.github/workflows/release.yml` -- not a regression from this plan's work, but that guard's own precondition (`git status --porcelain -- .github/workflows/` must already be empty when the harness runs) being violated by this plan's own legitimate, still-uncommitted `release.yml` edit. Resolved by committing Task 1 first (restoring a clean `.github/workflows/` tree) before running the full `make test-shell-guards` suite for Task 2's acceptance criteria.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- ARTIFACT-05 is closed: checksums are verifiable in one command, the instructions are only published when actually attachable, and the SBOM's real scope is stated. ARTIFACT-03's inventory half is closed: the asset list is read from the release itself, so nothing a failed leg failed to upload is ever advertised.
- The live-branching and consumer-facing halves of this plan's own `must_haves.truths` -- a real tag push producing a downloadable archive set plus `SHA256SUMS` that a consumer's `sha256sum -c` actually passes against -- remain explicitly unproven outside local script-level tests, consistent with the plan's own `backstop`-verification flag and coverage item D5 above. This is the D-14 rehearsal's job (plan 21-06).
- `scripts/finalize-release-body.sh`'s section-builder pattern (one `_frb_build_*_section` helper plus one `has_*` gate per section) made adding the two new sections additive, not a rewrite -- the same pattern remains available for any future section plan 21-05/21-06 might need.
- No blockers.

---
*Phase: 21-release-artifacts-curated-release-notes-and-attached-distrib*
*Completed: 2026-08-31*
