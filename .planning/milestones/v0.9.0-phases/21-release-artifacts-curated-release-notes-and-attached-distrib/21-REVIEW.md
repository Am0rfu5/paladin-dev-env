---
phase: 21-release-artifacts-curated-release-notes-and-attached-distrib
reviewed: 2026-08-31T00:00:00Z
depth: standard
files_reviewed: 10
files_reviewed_list:
  - .github/workflows/release.yml
  - docs/src/appendix/release-automation.md
  - docs/src/appendix/release-checklist.md
  - docs/src/appendix/release-recovery.md
  - scripts/extract-changelog-section.sh
  - scripts/finalize-release-body.sh
  - scripts/package-release-binaries.sh
  - tests/scripts/extract-changelog-section_test.sh
  - tests/scripts/finalize-release-body_test.sh
  - tests/scripts/package-release-binaries_test.sh
findings:
  critical: 2
  warning: 5
  info: 0
  total: 7
status: issues_found
---

# Phase 21: Code Review Report

**Reviewed:** 2026-08-31T00:00:00Z
**Depth:** standard
**Files Reviewed:** 10
**Status:** issues_found

## Summary

Reviewed the full release-artifacts pipeline (workflow, three new/rewired scripts, their
regression harnesses, and the three docs files describing them) at standard depth. The scripts
themselves (`extract-changelog-section.sh`, `package-release-binaries.sh`) are well-built —
careful boundary handling, exact-name matching, deterministic ordering, portable checksums, and
thorough test matrices that genuinely exercise the documented edge cases.

Two confirmed, evidence-backed BLOCKERs were found, both verified by direct command execution
rather than by inspection alone:

1. `scripts/finalize-release-body.sh`'s `aggregate_checksums` crashes the whole
   `finalize-release-body` job the moment zero `.tar.gz` archives exist on a release — directly
   contradicting the function's own documented "zero archives is a normal, non-failing outcome"
   contract, and directly contradicting `docs/src/appendix/release-automation.md`'s "Assembly
   order" and "aggregate_checksums's own doc comment" claims. This is not a hypothetical: the
   docs in this very phase's file list state the four `build-binaries` matrix legs "have failed
   on every release run observed so far" as of the pre-Phase-21 baseline — exactly the condition
   that triggers this bug.
2. `.github/workflows/release.yml`'s `verify-tag-source` job fails immediately on every
   `workflow_dispatch` invocation (including the documented dry-run flow) due to a `git rev-list`
   pathspec/revision confusion. This is pre-existing code (not touched by this phase's diff) but
   is squarely within this review's file scope, and it directly contradicts the "Dry Run" section
   of `release-automation.md`, which this phase's diff left standing next to newly-added,
   heavily-cross-referenced content in the same file.

The cited rehearsal (run 33436573814, tag push, all 12 jobs green) does not cover either finding:
it is a `push` event (not `workflow_dispatch`, so finding 2 was never exercised), and it evidently
had at least one `.tar.gz` archive land on the release before `finalize-release-body` ran (so
finding 1's zero-archive branch was never exercised either). Both are live, reachable defects, not
theoretical ones.

Five WARNING-level findings round out the review: one real (if low-exploitability) violation of
the project's own CR-01 "no inline `${{ }}` in run: bodies" rule, two stale documentation claims
left uncorrected in a heavily-edited file, one fragile positional JSON lookup, and one case of
duplicated tag-resolution logic across two jobs.

## Critical Issues

### CR-01: `finalize-release-body.sh` crashes when zero binary archives exist on the release

**File:** `scripts/finalize-release-body.sh:165-193` (function `aggregate_checksums`), invoked
unconditionally from `.github/workflows/release.yml:559` (`--aggregate-checksums`) and from
`finalize_release_body_main` at `scripts/finalize-release-body.sh:463`.

**Issue:** `aggregate_checksums` calls `gh release download "${tag}" --pattern '*.tar.gz' --dir
"${assets_dir}" --clobber` with no exit-status handling, under the script's top-level
`set -euo pipefail`. When the release has zero assets matching `*.tar.gz` — e.g. every
`build-binaries` matrix leg failed, or `finalize-release-body` runs before any binary has been
uploaded — the real `gh` CLI exits non-zero and prints `no assets match the file pattern`.
Verified live:

```
$ gh release download v2.97.0 --repo cli/cli --pattern '*.definitely-not-a-real-extension-xyz' --dir /tmp/ghtest
no assets match the file pattern
$ echo $?
1
```

Under `set -euo pipefail`, this non-zero exit propagates immediately, aborting
`aggregate_checksums`, `finalize_release_body_main`, and the whole script — before the function
ever reaches its own "zero archives is a normal, non-failing outcome" logic a few lines below
(the `archives=()` count check). This directly contradicts:
- The function's own doc comment: "Zero archives downloaded is a normal, non-failing outcome: no
  sums file is written and no upload is attempted."
- `docs/src/appendix/release-recovery.md`'s "Re-running the body-finalizing job" section, which
  documents this job as always safe to re-run regardless of what upstream legs succeeded.
- `docs/src/appendix/release-automation.md`'s "Assembly order" section, which claims a failed or
  skipped leg simply "contributes no section" rather than failing the whole job.

**Why the test suite didn't catch this:** `tests/scripts/finalize-release-body_test.sh` case 15
("zero downloaded archives") exercises this exact scenario, but against a *stubbed* `gh` whose
`release download` branch unconditionally `exit 0`s regardless of whether anything matched:

```bash
elif [ "${1:-}" = "release" ] && [ "${2:-}" = "download" ]; then
    echo "DOWNLOAD" >> "${CALL_LOG}"
    DEST_DIR="$(_stub_flag_value --dir "$@")"
    mkdir -p "${DEST_DIR}"
    if [ -d "${SCRATCH_DIR}/download_source" ]; then
        find "${SCRATCH_DIR}/download_source" -maxdepth 1 -type f -exec cp {} "${DEST_DIR}/" \;
    fi
    exit 0
```

The stub never replicates the real CLI's "no assets match" failure mode, so the regression suite
passes while masking a defect in exactly the scenario it claims to prove is handled.

**Impact:** Any release run where `finalize-release-body` executes before at least one `.tar.gz`
archive is visible on the release (all `build-binaries` legs failed — the documented historical
norm for this pipeline — or the job runs early in a partial re-run) fails the job outright,
leaving the release body un-finalized (no container-image section, no downloads section, no SBOM
section) even though `create-release`, `build-docker`, and `sbom` may all have succeeded.

**Fix:** Do not let a "no matches" exit from `gh release download` propagate as a hard failure;
let the existing zero-archive check downstream be the actual signal, matching the function's own
documented intent:

```bash
if ! "${gh}" release download "${tag}" --pattern '*.tar.gz' --dir "${assets_dir}" --clobber; then
    : # no assets matched *.tar.gz -- fall through to the archives=() check below,
      # which already treats zero archives as a normal, non-failing outcome.
fi
```

And update the case-15 stub to actually model the real CLI's behavior (exit 1, no files written,
when `download_source` is empty or absent) so the regression suite proves the real failure mode
is handled, not just that the stub was told to succeed.

### CR-02: `workflow_dispatch` releases fail immediately in `verify-tag-source`

**File:** `.github/workflows/release.yml:45-62` (step `Resolve release commit`, job
`verify-tag-source`)

**Issue:**

```bash
if [ "${{ github.event_name }}" = "workflow_dispatch" ]; then
  git fetch --tags --force --quiet origin
  SHA=$(git rev-list -n 1 -- "$RELEASE_TAG")
else
```

`git rev-list -n 1 -- "$RELEASE_TAG"` places `$RELEASE_TAG` after `--`, which makes git treat it
as a **pathspec**, not a revision. `git rev-list` requires at least one `<commit>` argument before
`--`; with none given, it prints its usage message and exits 129. Verified live:

```
$ git rev-list -n 1 -- v0.4.1
usage: git rev-list [<options>] <commit>... [--] [<path>...]
...
$ echo $?
129

$ git rev-list -n 1 v0.4.1        # without `--`, correct form
e49d9d719306474156b877be230693462fe4108d
$ echo $?
0
```

Reproduced inside the same `set -euo pipefail` shape the step actually uses:

```
$ bash -c 'set -euo pipefail; SHA=$(git rev-list -n 1 -- v0.4.1); echo "$SHA"'
usage: git rev-list [<options>] <commit>... [--] [<path>...]
...
(script exit code: 129)
```

**Impact:** every `workflow_dispatch` trigger of `release.yml` fails in the very first job, before
`create-release`, `test`, or anything else runs. This breaks:
- The documented dry-run flow in `release-automation.md`'s "Dry Run (no live publish)" section
  (`gh workflow run release.yml -f tag=v0.4.0-rc.1 -f dry_run=true`).
- `release-recovery.md`'s "Re-dispatching" recovery guidance for `workflow_dispatch`-triggered
  attempts.

This code is not part of this phase's diff (`git diff d55e75e8..HEAD -- .github/workflows/release.yml`
shows no touch to this block), but it is squarely in the reviewed file's scope, and it directly
undermines claims made by content this phase *did* add/keep in `release-automation.md` about the
dry-run path working end to end. The cited green rehearsal (run 33436573814) was a tag-push
event (`github.event_name == push`), so it never exercised this branch — the defect is live and
undetected by that evidence.

**Fix:** drop the `--`:

```bash
SHA=$(git rev-list -n 1 "$RELEASE_TAG")
```

## Warnings

### WR-01: `Verify image size` step violates the file's own CR-01 convention

**File:** `.github/workflows/release.yml:231-249` (step `Verify image size`, job `build-docker`)

**Issue:** Every other tainted value in this file (tag, version, SHA, digest, JSON blobs) reaches
its `run:` body through `env:` indirection, exactly per the project's stated CR-01 convention
("tag/version/digest... never inline `${{ }}` interpolation"). This one step is the sole
exception:

```bash
IMAGE=$(echo '${{ steps.meta.outputs.json }}' | jq -r '.tags[0]')
```

`steps.meta.outputs.json` is docker/metadata-action's tag-derived JSON blob — exactly the class of
value CR-01 exists to protect. It is inlined directly into the `run:` body rather than routed
through `env:`. Practical exploitability is low here (git ref/tag names cannot contain most shell
metacharacters, and creating a tag at all requires the "Protect release tags" ruleset's
admin-bypass), but it is a real, provable inconsistency against a hard rule this same file
otherwise follows without exception. Pre-existing (predates this phase's diff), not newly
introduced, but still present in the reviewed file.

**Fix:**

```yaml
      - name: Verify image size
        id: size
        env:
          META_JSON: ${{ steps.meta.outputs.json }}
        run: |
          IMAGE=$(echo "$META_JSON" | jq -r '.tags[0]')
          echo "Inspecting image: $IMAGE"
          docker pull "$IMAGE"
          ...
```

### WR-02: Stale "Re-dispatching" caveat contradicts the pipeline's actual create-or-reuse behavior

**File:** `docs/src/appendix/release-automation.md:144-149` ("Known operational caveats")

**Issue:**

> **Re-dispatching a release fails outright if the GitHub release object already exists.**
> `actions/create-release@v1` has no upsert behavior; a `workflow_dispatch` re-run after a failed
> attempt requires deleting the stale release object first...

This is contradicted by the `create-release` job's actual, current behavior — documented a few
lines earlier in this exact file section and in `.github/workflows/release.yml`'s own comment on
the `Create or reuse release` step: it looks the release up by tag first (HTTP 200 reuses it, 404
creates it) via `scripts/create-or-reuse-release.sh`, with no `actions/create-release@v1`
dependency at all. An operator following this stale caveat mid-incident could unnecessarily delete
a real, already-published release object.

**Fix:** Remove or correct this bullet to reflect the create-or-reuse behavior, or cross-reference
`release-recovery.md`'s "Completing forward" section, which already documents the correct,
current re-run behavior.

### WR-03: Stale "cause undiagnosed" claim about Build Binaries, contradicted by this phase's own D-05 fix

**File:** `docs/src/appendix/release-automation.md:150-153` ("Known operational caveats")

**Issue:**

> **The four Build Binaries matrix jobs... have failed on every release run observed so far**,
> cause undiagnosed.

`.github/workflows/release.yml`'s `Package release binaries` step comment (added by this phase)
identifies and fixes exactly the kind of root cause this caveat calls "undiagnosed": builds
silently omitting `paladin-cli`/`paladin-server` because `required-features` were unmet (D-05),
now hard-failing instead via `scripts/package-release-binaries.sh`'s expected-binary assertion.
Leaving "cause undiagnosed" standing in the same file this phase substantially reworked
misrepresents the current state to an operator reading this section during an incident.

**Fix:** Update this caveat to reference the D-05 fix, or remove it if the underlying failure mode
is now believed resolved; if it is not yet confirmed resolved, say so explicitly rather than
repeating "cause undiagnosed" unchanged.

### WR-04: Fragile positional JSON lookup for image size

**File:** `.github/workflows/release.yml:543-549` (step `Finalize release body with artifact
sections`, job `finalize-release-body`)

**Issue:**

```bash
# Positional, not by field name: this is the sole other reference
# to build-docker's size output in this file...
IMAGE_SIZE_MB=$(echo "$DOCKER_OUTPUTS_JSON" | jq -r 'to_entries[2].value // empty')
```

This relies on `toJSON(needs.build-docker.outputs)` preserving field order exactly matching
`build-docker`'s `outputs:` declaration order (`digest`, `tags_json`, `image_size_mb`). JSON
objects are not ordering-guaranteed by spec; GitHub Actions' current behavior happens to preserve
insertion order, but nothing enforces this contract at the point where it would break. If a future
edit reorders or inserts a field in `build-docker`'s `outputs:` block, this silently starts
reading the wrong value (or an empty one) — no error, just a wrong or missing number/section in
the published release body.

**Fix:** Use a named-field jq lookup instead of a positional index — this achieves the same "don't
re-declare the field name a second time" goal the comment cites just as well as positional
indexing does, without the ordering fragility:

```bash
IMAGE_SIZE_MB=$(echo "$DOCKER_OUTPUTS_JSON" | jq -r '.image_size_mb // empty')
```

### WR-05: Duplicated tag/version resolution logic across two jobs

**File:** `.github/workflows/release.yml:119-128` (job `create-release`, step `Get version`) and
`:415-424` (job `check-release-consistency`, step `Get version`)

**Issue:** The `workflow_dispatch` vs. tag-push version-resolution logic is copy-pasted verbatim
between these two jobs:

```bash
if [ "${{ github.event_name }}" == "workflow_dispatch" ]; then
  echo "version=$RELEASE_TAG" >> "$GITHUB_OUTPUT"
else
  echo "version=${GITHUB_REF#refs/tags/}" >> "$GITHUB_OUTPUT"
fi
```

`verify-tag-source` (a shared upstream job both of these depend on, directly or transitively)
already performs an equivalent resolution for its own `sha` output. Two independent copies of the
same parsing logic create a drift risk: a future edit to one (e.g. to handle a new ref shape) that
isn't mirrored in the other could cause the release body's version and the pre-publish consistency
gate's version to silently disagree.

**Fix:** Compute `version` once in `verify-tag-source` alongside its existing `sha` output, and
have `create-release` and `check-release-consistency` both consume
`needs.verify-tag-source.outputs.version` instead of re-deriving it.

---

_Reviewed: 2026-08-31T00:00:00Z_
_Reviewer: Claude (gsd-code-reviewer)_
_Depth: standard_
