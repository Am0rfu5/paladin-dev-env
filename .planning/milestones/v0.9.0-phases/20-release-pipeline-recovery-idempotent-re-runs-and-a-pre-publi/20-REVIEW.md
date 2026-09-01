---
phase: 20-release-pipeline-recovery-idempotent-re-runs-and-a-pre-publi
reviewed: 2026-08-30T19:24:03Z
depth: standard
files_reviewed: 14
files_reviewed_list:
  - .github/workflows/release.yml
  - Makefile
  - release.toml
  - docs/src/SUMMARY.md
  - docs/src/appendix/release-automation.md
  - docs/src/appendix/release-checklist.md
  - docs/src/appendix/release-recovery.md
  - scripts/check-release-consistency.sh
  - scripts/create-or-reuse-release.sh
  - scripts/finalize-crate-changelogs.sh
  - scripts/publish-crates.sh
  - tests/scripts/check-release-consistency_test.sh
  - tests/scripts/create-or-reuse-release_test.sh
  - tests/scripts/finalize-crate-changelogs_test.sh
  - tests/scripts/publish-crates_test.sh
findings:
  critical: 2
  warning: 6
  info: 2
  total: 10
status: issues_found
---

# Phase 20: Code Review Report

**Reviewed:** 2026-08-30T19:24:03Z
**Depth:** standard
**Files Reviewed:** 14
**Status:** issues_found

## Summary

The four new/modified shell scripts (`check-release-consistency.sh`, `create-or-reuse-release.sh`,
`finalize-crate-changelogs.sh`, `publish-crates.sh`) are careful, well-tested, and their CR-01
(indirect-through-`env:`) and no-redirect-following discipline is applied correctly and
consistently — no violations found there, and the four regression harnesses in `tests/scripts/`
genuinely exercise the documented decision tables via network-free stubs.

The gaps this review found are concentrated in two places the rehearsals (v0.8.1-rc.3, v0.8.1-rc.4)
never exercised: the `workflow_dispatch` trigger path in `release.yml` (both rehearsals were tag
pushes, not dispatches), and the local-only `make publish-dry-run` target, which is not wired into
CI at all. Both are broken in ways that were empirically confirmed against this checkout (a live
`git rev-list` invocation and `cargo pkgid` resolution), not merely suspected. The
`workflow_dispatch` path is the only way to run the documented dry-run validation
(`gh workflow run release.yml -f tag=... -f dry_run=true`) and is also the pipeline's own
recovery-adjacent trigger — it currently fails before doing anything.

## Critical Issues

### CR-01: `verify-tag-source`'s SHA resolution is invalid git usage — every `workflow_dispatch` run fails immediately

**File:** `.github/workflows/release.yml:56`
**Issue:** The `Resolve release commit` step, under `set -euo pipefail`, runs:

```bash
SHA=$(git rev-list -n 1 -- "$RELEASE_TAG")
```

`git rev-list -n 1 -- <path>` is a pathspec-only invocation with **no commit-ish given before
`--`**; git rejects this outright with a usage error (exit 129), it does not resolve the ref
named by `$RELEASE_TAG` at all. This was verified directly against this repository's git binary:

```
$ git rev-list -n 1 -- v1.2.3
usage: git rev-list [<options>] <commit>... [--] [<path>...]
...
exit: 129
```

Because the assignment `SHA=$(...)` is a simple command whose exit status is that of the failing
substitution, `set -e` aborts the step immediately — confirmed with the exact snippet under
`set -euo pipefail`. This means **every `workflow_dispatch` invocation of `release.yml` fails at
the very first job**, before `create-release`, `test`, or `check-release-consistency` ever run.
Tag-push events are unaffected (they take the `else` branch and use `github.sha` directly), which
is why this was never caught by the v0.8.1-rc.3/rc.4 rehearsals — both were tag pushes.

This breaks:
- The documented dry-run flow: `gh workflow run release.yml -f tag=v0.4.0-rc.1 -f dry_run=true`
  (`docs/src/appendix/release-automation.md:162`).
- Any future use of `workflow_dispatch` for manual re-triggering.

**Fix:**
```bash
# Resolve the tag to its commit SHA directly (no pathspec separator needed;
# rev-list peels an annotated tag to the commit it points at).
SHA=$(git rev-list -n 1 "$RELEASE_TAG")
```
Or more directly, `SHA=$(git rev-parse "$RELEASE_TAG^{commit}")`. Add a regression check (even a
simple `gh workflow run ... -f dry_run=true` rehearsal, since this job has no shell-script harness
the way the four `scripts/*.sh` gates do) before relying on `workflow_dispatch` again.

### CR-02: `make publish-dry-run` never actually dry-runs 3 of the 11 real crates — wrong package names, masked by `|| true`

**File:** `Makefile:536-549`
**Issue:** `publish-dry-run` invokes `cargo publish --dry-run -p paladin-core` and
`-p paladin`, but neither is a real Cargo package name in this workspace — confirmed directly:

```
$ cargo pkgid -p paladin-core
error: package ID specification `paladin-core` did not match any packages
help: a package with a similar name exists: `paladin-ai-core`

$ cargo pkgid -p paladin
error: package ID specification `paladin` did not match any packages
help: a package with a similar name exists: `plain`
```

The real names are `paladin-ai-core` (`crates/paladin-core`) and `paladin-ai` (workspace root), per
`scripts/publish-crates.sh`'s own `CRATES` array and the canonical order documented in
`docs/src/appendix/release-automation.md` and `docs/src/appendix/release-checklist.md`. The target
also omits `paladin-herald` entirely — the crate whose placement in the publish order required the
most care (see the `paladin-herald` comment in `scripts/publish-crates.sh:134-143`) is the one
local dry-run validation never touches.

Because every line ends in `|| true`, all three of these no-op/failing invocations are silently
swallowed and `make publish-dry-run` always prints "Dry-run publish command sequence completed."
regardless. Net effect: the target that `CLAUDE.md`'s own quick-reference table documents as
`make publish-dry-run # Dependency-first cargo publish --dry-run` only ever validates 7 of the 11
real publishable crates, and gives false confidence about the other 4 — including the foundational
`paladin-ai-core` crate every other crate depends on, and the `paladin-ai` facade crate whose
`package.include` allowlist is precisely the kind of manifest issue a dry run is supposed to catch.

**Fix:**
```makefile
.PHONY: publish-dry-run
publish-dry-run: release-check ## Run dependency-first `cargo publish --dry-run` for all crates
	@echo "$(CYAN)Running dependency-first publish dry-runs...$(NC)"
	@$(CARGO) publish --dry-run -p paladin-ai-core || true
	@$(CARGO) publish --dry-run -p paladin-ports || true
	@$(CARGO) publish --dry-run -p paladin-herald || true
	@$(CARGO) publish --dry-run -p paladin-battalion || true
	@$(CARGO) publish --dry-run -p paladin-llm || true
	@$(CARGO) publish --dry-run -p paladin-memory || true
	@$(CARGO) publish --dry-run -p paladin-web || true
	@$(CARGO) publish --dry-run -p paladin-notifications || true
	@$(CARGO) publish --dry-run -p paladin-content || true
	@$(CARGO) publish --dry-run -p paladin-storage || true
	@$(CARGO) publish --dry-run -p paladin-ai || true
```
Better still, drive this list from `scripts/publish-crates.sh --dry-run` (already correct and
tested) instead of maintaining a second, drifted copy of the crate order in the Makefile.

## Warnings

### WR-01: `publish-crates.sh` dry-run mode unconditionally swallows `cargo publish --dry-run` failures

**File:** `scripts/publish-crates.sh:303-309`
**Issue:**
```bash
if [ "${dry_run}" = "true" ]; then
    echo "::group::Publishing ${name} (dry-run)"
    "${CARGO_BIN}" publish --dry-run -p "${name}" || true
    OUTCOME["${name}"]="skipped"
    echo "::endgroup::"
    return 0
fi
```
`|| true` discards `cargo publish --dry-run`'s exit status entirely, and every crate is recorded
`skipped` regardless of whether the underlying command actually succeeded. `publish_crates_main`'s
exit rule for dry-run mode (`if [ "${DRY_RUN}" = "true" ]; then return 0; fi`) never inspects
`OUTCOME` in this mode, so **a real packaging/manifest defect in any crate (a broken `Cargo.toml`,
a missing file the `include` allowlist misses, etc.) cannot fail this job** — the CI dry-run
(`workflow_dispatch -f dry_run=true`) and `make publish-dry-run` alike would both report success.
This directly undercuts the "Dry-Run Claim Boundary" `docs/src/appendix/release-automation.md`
establishes ("it asserts packaging validity") — packaging-validity failures are exactly the class
this silently discards. (Some `|| true` here is legitimate: a downstream crate's dry run can fail
merely because an upstream sibling was not actually published earlier in the same dry run — that
failure mode is expected and shouldn't abort the loop. The problem is that this and a genuine
packaging defect are currently indistinguishable in the outcome table.)

**Fix:** Capture the exit code and record a distinct outcome (e.g. `skipped (dry-run failed)`)
instead of a bare `skipped`, so the table at least signals that something failed even though the
loop intentionally continues:
```bash
if "${CARGO_BIN}" publish --dry-run -p "${name}"; then
    OUTCOME["${name}"]="skipped"
else
    OUTCOME["${name}"]="skipped (dry-run failed — see log above)"
fi
```

### WR-02: `workflow_dispatch` release creation is not pinned to the resolved release commit

**File:** `.github/workflows/release.yml:114-118`
**Issue:** The `create-release` job's checkout has no `ref:` override:
```yaml
- name: Checkout code
  uses: actions/checkout@v4
  with:
    fetch-depth: 0
```
For a `workflow_dispatch` run this checks out whatever ref the dispatch itself was run against
(typically the default branch tip), not the commit `verify-tag-source` resolves for
`inputs.tag`. The `Generate changelog` step's `git log ... HEAD` / `git describe ... HEAD^` then
computes the changelog from the wrong commit, and `create-or-reuse-release.sh`'s payload
(`scripts/create-or-reuse-release.sh:104-110`) never sets `target_commitish`, so if
`inputs.tag` names a ref that doesn't already exist as a tag, GitHub creates it pointing at the
repository's default branch — not the SHA `verify-tag-source` resolved. This is currently masked
by CR-01 (the job never gets this far today), but will resurface once CR-01 is fixed.
**Fix:** Checkout the resolved SHA explicitly (`ref: ${{ needs.verify-tag-source.outputs.sha }}`
in `create-release`, which already depends on `verify-tag-source`), and pass
`--target-commitish` through to `create-or-reuse-release.sh` so a freshly-created tag is pinned to
the right commit rather than defaulting to the branch tip.

### WR-03: `_cor_gh_call` merges stdout and stderr before parsing the HTTP status line

**File:** `scripts/create-or-reuse-release.sh:68-85`
**Issue:**
```bash
raw=$(printf '%s' "${payload}" | "${GH}" api -i -X "${method}" "${endpoint}" --input - 2>&1) || true
...
status_line=$(printf '%s\n' "${raw}" | head -n1 | tr -d '\r')
HTTP_STATUS=$(printf '%s' "${status_line}" | sed -nE 's#^HTTP/[0-9.]+ ([0-9]{3}).*#\1#p')
```
`2>&1` interleaves anything `gh` writes to stderr (a deprecation notice, an auth hint, a retry
warning) with the `-i`-included HTTP response on stdout, and the status line is read as
whatever line 1 happens to be. If `gh` ever emits diagnostic output ahead of the response, this
parses garbage as the status line, `HTTP_STATUS` falls back to `"000"`, and the script reports a
hard failure ("expected 200 or 404") that is not actually what happened. This fails loud rather
than silently misbehaving (no security impact), but it is a reliability gap in exactly the kind of
in-CI HTTP-status disambiguation this script exists to get right.
**Fix:** Capture stdout and stderr into separate files/variables (as
`scripts/check-release-consistency.sh`'s `_crc_fetch_ci_runs` already does with `err_file`) rather
than merging them, and only parse the stdout stream for the status line.

### WR-04: `steps.meta.outputs.json` interpolated directly into a `run:` block, inconsistent with this file's own CR-01 policy

**File:** `.github/workflows/release.yml:254`
**Issue:**
```bash
IMAGE=$(echo '${{ steps.meta.outputs.json }}' | jq -r '.tags[0]')
```
Every other tainted-value use in this workflow is deliberately indirected through `env:` with an
explanatory CR-01 comment (see e.g. lines 133-140, 429-432, 449-452, 527-530). This is the one
place a computed step output is spliced directly into a `run:` script body via `${{ }}`, inside a
single-quoted string — if `steps.meta.outputs.json` ever contained an embedded single quote (it
currently should not, since `docker/metadata-action`'s tags are computed from ref/semver patterns,
not free text), it would break out of the quoting. Low exploitability today, but it is the one
inconsistency with a security control this file otherwise applies uniformly and calls out by name.
**Fix:** Route through `env:` like every other step in this file:
```yaml
- name: Verify image size
  env:
    META_JSON: ${{ steps.meta.outputs.json }}
  run: |
    IMAGE=$(echo "${META_JSON}" | jq -r '.tags[0]')
```

### WR-05: `actions/upload-release-asset@v1` (archived, unmaintained) is a plausible cause of the documented Build Binaries failures

**File:** `.github/workflows/release.yml:357-375`, `.github/workflows/release.yml:472-480`
**Issue:** `docs/src/appendix/release-automation.md:150-152` records: "The four Build Binaries
matrix jobs...have failed on every release run observed so far, cause undiagnosed." All four
upload steps (two per binary-matrix job, plus the SBOM job) use `actions/upload-release-asset@v1`,
an action GitHub archived years ago and does not maintain. This phase already replaced one
archived-action dependency (`actions/create-release@v1`, per the `create-or-reuse-release.sh`
D-01 comment) specifically because of its lack of upsert/reliability guarantees; the same rationale
applies here and this is a live, recorded, unexplained failure mode in the same workflow.
**Fix:** Replace with `gh release upload <tag> <file> --clobber` (consistent with the `gh`-CLI
house style this phase already established for `create-or-reuse-release.sh`), which also sidesteps
the `upload_url` templated-URL mechanism the archived action depends on.

### WR-06: Duplicated `Get version` step logic between two jobs

**File:** `.github/workflows/release.yml:120-129`, `.github/workflows/release.yml:416-425`
**Issue:** The `create-release` and `check-release-consistency` jobs each carry an identical
6-line `if [ "${{ github.event_name }}" == "workflow_dispatch" ] ... else ... fi` block to derive
`version` from either `inputs.tag` or `GITHUB_REF`. A future change to this logic (e.g., adding
input validation, which would also help WR-02/CR-01) has to be made twice and can silently drift.
**Fix:** Factor into a small reusable workflow (`workflow_call`) or a composite action, or at
minimum have `check-release-consistency` consume `needs.create-release.outputs.version` instead of
re-deriving it (note: this would require reordering `needs`, since `check-release-consistency`
currently only depends on `verify-tag-source`, not `create-release`).

## Info

### IN-01: Dead assertion block in `publish-crates_test.sh`

**File:** `tests/scripts/publish-crates_test.sh:290-293`
**Issue:**
```bash
ASSERTIONS=$((ASSERTIONS + 1))
if [ -s "${DIR11}/curl_call_log" ] && ! grep -qL 'User-Agent' "${DIR11}/curl_call_log" 2>/dev/null; then
    :
fi
```
This increments `ASSERTIONS` and then does nothing (`:` no-op) regardless of the condition —
vestigial from an earlier edit (the real check follows two lines below). It doesn't hide a real
failure (it never touches `FAILED`), but it inflates the reported assertion count with a check
that verifies nothing.
**Fix:** Delete the dead block; the following `if grep -q 'User-Agent' ...` block already covers
the intended assertion.

### IN-02: `release.toml`'s comment documents a 10-crate order, missing `paladin-herald`

**File:** `release.toml:12-14`
**Issue:** The header comment says "cargo-release publishes workspace members in topological
dependency order automatically: paladin-core -> paladin-ports -> leaf tier -> paladin," which
both uses the pre-Trusted-Publishing package names (`paladin-core`/`paladin`) instead of
`paladin-ai-core`/`paladin-ai`, and omits `paladin-herald`'s position — inconsistent with the
canonical eleven-crate order this phase's own docs (`docs/src/appendix/release-automation.md`)
and `scripts/publish-crates.sh` establish. This is a comment only (no functional effect, since
`cargo-release` itself is not what actually publishes crates — CI's tag-triggered pipeline is),
but it is exactly the kind of stale documentation that fed CR-02's Makefile bug.
**Fix:** Update the comment to match the real eleven-crate, correctly-named order, or simply
point at `docs/src/appendix/release-automation.md#canonical-publish-order` as the single source of
truth instead of restating it here.

---

_Reviewed: 2026-08-30T19:24:03Z_
_Reviewer: Claude (gsd-code-reviewer)_
_Depth: standard_
