---
phase: 21-release-artifacts-curated-release-notes-and-attached-distrib
fixed_at: 2026-08-31T22:08:52Z
review_path: .planning/phases/21-release-artifacts-curated-release-notes-and-attached-distrib/21-REVIEW.md
iteration: 1
findings_in_scope: 7
fixed: 6
skipped: 1
status: partial
---

# Phase 21: Code Review Fix Report

**Fixed at:** 2026-08-31T22:08:52Z
**Source review:** .planning/phases/21-release-artifacts-curated-release-notes-and-attached-distrib/21-REVIEW.md
**Iteration:** 1

**Summary:**
- Findings in scope: 7 (2 Critical, 5 Warning — `fix_scope: critical_warning`)
- Fixed: 6
- Skipped: 1

## Fixed Issues

### CR-01: `finalize-release-body.sh` crashes when zero binary archives exist on the release

**Files modified:** `scripts/finalize-release-body.sh`, `tests/scripts/finalize-release-body_test.sh`
**Commit:** `ef4e82b2`
**Applied fix:** Wrapped the `gh release download ... --pattern '*.tar.gz'` call in
`aggregate_checksums` with an `if ! ...; then :; fi` guard so a non-zero exit (real `gh`'s "no
assets match the file pattern" behavior) no longer propagates under `set -euo pipefail` and aborts
the job before reaching the function's own documented zero-archive handling. In the same commit,
fixed the test harness's `gh` stub (case 15's regression coverage) to actually model the real
CLI's failure mode — it previously always `exit 0`d regardless of whether `download_source`
contained anything, masking this exact defect. The stub now exits 1 with "no assets match the file
pattern" and writes nothing to the destination when `download_source` is empty or absent. Full
84-assertion `finalize-release-body_test.sh` suite passes.

### CR-02: `workflow_dispatch` releases fail immediately in `verify-tag-source`

**Files modified:** `.github/workflows/release.yml`
**Commit:** `ebae6cfc`
**Applied fix:** Dropped the `--` in `git rev-list -n 1 -- "$RELEASE_TAG"` → `git rev-list -n 1
"$RELEASE_TAG"`, matching the exact minimal fix specified in the review (`$RELEASE_TAG` now
resolves as a revision, not a pathspec). `set -euo pipefail` already fails the step loudly if the
tag does not exist as a revision, satisfying the "fail loudly when the tag does not exist"
constraint without further restructuring. No job restructuring performed, per instruction. YAML
validated with `python3 -c "import yaml; yaml.safe_load(...)"`; `make test-shell-guards` and `make
check-gates` both pass against the committed state.

### WR-01: `Verify image size` step violates the file's own CR-01 convention

**Files modified:** `.github/workflows/release.yml`
**Commit:** `62887d6f`
**Applied fix:** Added an `env: META_JSON: ${{ steps.meta.outputs.json }}` block to the `Verify
image size` step and changed `echo '${{ steps.meta.outputs.json }}'` to `echo "$META_JSON"`,
matching the CR-01 `env:` indirection convention every other tainted value in this file already
follows.

### WR-02: Stale "Re-dispatching" caveat contradicts the pipeline's actual create-or-reuse behavior

**Files modified:** `docs/src/appendix/release-automation.md`
**Commit:** `cc53f352`
**Applied fix:** Rewrote the caveat bullet to describe the actual `create-or-reuse-release.sh`
behavior (look up by tag, HTTP 200 reuses / 404 creates, no `actions/create-release@v1`
dependency, no upsert-failure mode) and cross-referenced `release-recovery.md`'s "Completing
forward" section for the current re-run playbook, per the review's suggested fix.

### WR-03: Stale "cause undiagnosed" claim about Build Binaries, contradicted by this phase's own D-05 fix

**Files modified:** `docs/src/appendix/release-automation.md`
**Commit:** `9efcbca8`
**Applied fix:** Updated the caveat to state the root cause is now identified and fixed (D-05:
builds silently omitting `paladin-cli`/`paladin-server` when Cargo `required-features` were
unmet), referencing `package-release-binaries.sh`'s expected-binary assertion that now hard-fails
the leg instead of silently shipping an incomplete archive, rather than repeating "cause
undiagnosed" unchanged.

### WR-04: Fragile positional JSON lookup for image size

**Files modified:** `.github/workflows/release.yml`
**Commit:** `beab9060`
**Applied fix:** Replaced `jq -r 'to_entries[2].value // empty'` with `jq -r '.image_size_mb //
empty'`, removing the dependency on `toJSON(needs.build-docker.outputs)` preserving field
declaration order. Updated the accompanying comment to explain the ordering-fragility rationale.

## Skipped Issues

### WR-05: Duplicated tag/version resolution logic across two jobs

**File:** `.github/workflows/release.yml:119-128` (job `create-release`) and `:415-424` (job
`check-release-consistency`)
**Reason:** Skipped per the explicit "risk of destabilizing the just-rehearsed pipeline" carve-out
in this run's instructions. The suggested fix requires adding a new `version` output to the shared
upstream `verify-tag-source` job and repointing two downstream jobs (`create-release`,
`check-release-consistency`) to consume `needs.verify-tag-source.outputs.version` instead of
re-deriving it — a job-graph/output restructuring across the exact pipeline that was just
rehearsed end-to-end (run 33436573814, all 12 jobs green). This finding is pure drift-risk
(duplication, no functional bug), there is no automated harness in this repo that exercises the
GitHub Actions job-output wiring itself (only the shell-script logic inside individual steps), so
verifying the refactor's correctness would require another live workflow rehearsal, which is out
of scope for this fixer run. Left as-is; recommend addressing in a follow-up phase with a
dedicated rehearsal run to confirm the new output wiring.
**Original issue:** The `workflow_dispatch` vs. tag-push version-resolution `if`/`else` block is
copy-pasted verbatim in both jobs' `Get version` steps, creating drift risk if one copy is edited
(e.g. to handle a new ref shape) without mirroring the other.

---

_Fixed: 2026-08-31T22:08:52Z_
_Fixer: Claude (gsd-code-fixer)_
_Iteration: 1_
