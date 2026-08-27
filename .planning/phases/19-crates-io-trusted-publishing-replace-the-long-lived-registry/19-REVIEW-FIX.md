---
phase: 19-crates-io-trusted-publishing-replace-the-long-lived-registry
fixed_at: 2026-08-27T21:17:06Z
review_path: .planning/phases/19-crates-io-trusted-publishing-replace-the-long-lived-registry/19-REVIEW.md
iteration: 1
findings_in_scope: 6
fixed: 6
skipped: 0
status: all_fixed
---

# Phase 19: Code Review Fix Report

**Fixed at:** 2026-08-27T21:17:06Z
**Source review:** .planning/phases/19-crates-io-trusted-publishing-replace-the-long-lived-registry/19-REVIEW.md
**Iteration:** 1

**Summary:**
- Findings in scope: 6 (1 Critical, 5 Warning — Info findings IN-01, IN-02, IN-03 are out of scope per `fix_scope: critical_warning`)
- Fixed: 6
- Skipped: 0

All fixes were applied to `.github/workflows/release.yml` only, in the isolated worktree
`gsd-reviewfix/19-*` branched from `chore/19-trusted-publishing`, one commit per finding. Each
commit was validated with `actionlint` (clean) and `scripts/check-workflow-triggers.sh` (pass)
before being made, in addition to a YAML parse check and a re-read of the changed section.

## Fixed Issues

### CR-01: Unsanitized `workflow_dispatch` input interpolated into shell commands (script injection)

**Files modified:** `.github/workflows/release.yml`
**Commit:** `71e7b733`
**Applied fix:** Added `env:` indirection for the tainted values instead of direct `${{ }}`
interpolation in `run:`/`with:` blocks, in all three locations the review identified:
- `verify-tag-source` / `Resolve release commit`: `inputs.tag` now flows through
  `env.RELEASE_TAG`, referenced as `"$RELEASE_TAG"` in `git rev-list -n 1 -- "$RELEASE_TAG"`
  (added `--` before the ref as an extra defensive measure from the review's example).
- `create-release` / `Get version`: same pattern, `echo "version=$RELEASE_TAG"`.
- `sbom` job: added a job-level `env: RELEASE_VERSION: ${{ needs.create-release.outputs.version }}`
  and switched the `cp` shell command to `"${RELEASE_VERSION}"`, and the `asset_path`/`asset_name`
  `with:` fields on the "Upload SBOM to release" step to `${{ env.RELEASE_VERSION }}`, matching the
  review's fix guidance to apply the same indirection to all three `cp`/`asset_path`/`asset_name`
  usages.

Did not add the review's *optional* second-layer `^v[0-9]+\.[0-9]+\.[0-9]+` regex validation of
`inputs.tag` — the finding marked it optional and the task's constraint was to preserve behavior
exactly beyond the findings themselves.

### WR-01: Jobs missing an explicit `permissions:` block

**Files modified:** `.github/workflows/release.yml`
**Commit:** `9feb3f34`
**Applied fix:** Added `permissions: contents: read` explicitly to `verify-tag-source` and `test`,
matching the scoped-permissions discipline already used by every other job in the file.

### WR-02: Mutable `@master` action ref used in a job with `contents: write`

**Files modified:** `.github/workflows/release.yml`
**Commit:** `6934e504`
**Applied fix:** Resolved `dtolnay/rust-toolchain@master`'s current HEAD via
`git ls-remote https://github.com/dtolnay/rust-toolchain master` (`6c977a6c...`) and pinned to
that commit SHA with a dated comment (`# master, as of 2026-08-27`), preserving the `master`
action-ref semantics (required for the job's `toolchain:` input mechanism) while removing the
mutability. Added a comment clarifying that only the ref is pinned, not the toolchain-selection
design the existing comment already documents (rust-toolchain.toml as single source of truth).

### WR-03: `cargo install cross` is unpinned

**Files modified:** `.github/workflows/release.yml`
**Commit:** `2abfc767`
**Applied fix:** Looked up `cross`'s current `max_stable_version` on crates.io (`0.2.5`) and pinned
`cargo install cross --locked --version 0.2.5`, matching the reproducibility discipline already
used for `cargo-release` and `cargo-cyclonedx` in the same file.

### WR-04: `GITHUB_OUTPUT` heredoc uses a collidable literal delimiter

**Files modified:** `.github/workflows/release.yml`
**Commit:** `e3557a12`
**Applied fix:** Generate a per-run random delimiter (`delim="EOF_$(openssl rand -hex 8)"`) and use
it in place of both `EOF` occurrences in the changelog heredoc, per the review's exact suggested
fix. `openssl` is present on `ubuntu-latest` runners.

### WR-05: Docker image and binary artifacts are not gated on the test suite

**Files modified:** `.github/workflows/release.yml`
**Commit:** `ad0d3587`
**Applied fix:** The review offered two alternative fixes: add `test` to `build-docker`'s and
`build-binaries`'s `needs:` list, or document the asymmetry explicitly. Chose the
documentation-only option to avoid changing job dependency topology/timing in a release workflow
that was just proven working end-to-end (run `33089177606`) — adding `needs: test` would delay
both artifact jobs behind the test suite, a real (if arguably desirable) behavior change beyond
what this fix pass was scoped to introduce. Expanded the `test` job's comment to state explicitly
that `build-docker`/`build-binaries` are not test-gated, and added a one-line cross-reference
comment at each of those two jobs pointing back to it, so the asymmetry reads as a documented
choice rather than a latent surprise, with zero change to execution behavior.

## Skipped Issues

None — all six in-scope findings (CR-01, WR-01 through WR-05) were fixed.

---

_Fixed: 2026-08-27T21:17:06Z_
_Fixer: Claude (gsd-code-fixer)_
_Iteration: 1_
