# Phase 21: Release Artifacts — Curated Release Notes and Attached Distributables - Pattern Map

**Mapped:** 2026-08-31
**Files analyzed:** 9 (1 modified workflow + 2 new scripts + 2 new tests + 2 docs + 1 evidence file
+ 1 modified script/job set counted together)
**Analogs found:** 9 / 9

## File Classification

| New/Modified File | Role | Data Flow | Closest Analog | Match Quality |
|---|---|---|---|---|
| `scripts/extract-changelog-section.sh` (NEW) | utility (CI script) | transform (text-in, text-out) | `scripts/check-release-consistency.sh` (Clause 2 heading-match logic) | role-match (script-in-repo, text processing over `CHANGELOG.md`) — exact for the regex/anchoring discipline |
| `scripts/finalize-release-body.sh` (NEW, or inline — see discretion note) | utility (CI script) | transform + idempotent read-modify-write | `scripts/create-or-reuse-release.sh` (gh API wrapper, `--body-file` discipline, sourcing seam) | role-match — same "logic-in-scripts, gh CLI, tainted-value-via-file" shape |
| `.github/workflows/release.yml` — `create-release` job (MODIFIED) | CI workflow job | request-response (gh API call) | itself, prior revision (Phase 20's `create_or_reuse_release` step) | exact — same job, changing only what feeds `--body-file` |
| `.github/workflows/release.yml` — `build-binaries` job (MODIFIED) | CI workflow job | batch (matrix build) | itself, prior revision | exact — same job; feature flags, strip guard, upload mechanism change |
| `.github/workflows/release.yml` — `build-docker` job (MODIFIED) | CI workflow job | batch + request-response | itself, prior revision (`Verify image size` step's `steps.meta.outputs.json` idiom) | exact |
| `.github/workflows/release.yml` — `sbom` job (MODIFIED) | CI workflow job | batch + request-response | itself, prior revision | exact |
| `.github/workflows/release.yml` — `finalize-release-body` job (NEW) | CI workflow job | event-driven (terminal, `needs` fan-in) | `publish-crates` job (thin invocation of a repo script, terminal-ish `needs` fan-in on multiple upstream jobs) | role-match |
| `tests/scripts/extract-changelog-section_test.sh` (NEW) | test | batch (fixture harness) | `tests/scripts/check-release-consistency_test.sh` | exact — same fixture-lifecycle pattern (scratch dir, mktemp, trap cleanup, no network) |
| `tests/scripts/finalize-release-body_test.sh` (NEW, if factored into script) | test | batch (fixture harness) | `tests/scripts/create-or-reuse-release_test.sh` | exact — same `gh` CLI stub-on-PATH pattern, call-log assertions |
| `docs/src/appendix/release-automation.md` (MODIFIED) | config/docs | — | itself, prior revision | exact |
| `docs/src/appendix/release-checklist.md` (MODIFIED) | config/docs | — | itself, prior revision | exact |
| `21-ARTIFACT-EVIDENCE.md` (NEW, phase dir) | test (evidence record) | batch (manual/e2e) | `20-RECOVERY-EVIDENCE.md`, `19-PUBLISH-EVIDENCE.md` | exact |

## Pattern Assignments

### `scripts/extract-changelog-section.sh` (utility, transform)

**Analog:** `scripts/check-release-consistency.sh` (Clause 2, lines ~395-430) and
`scripts/create-or-reuse-release.sh` (header/sourcing-seam conventions, lines 1-50)

**Header/contract comment pattern** (`scripts/create-or-reuse-release.sh` lines 1-50):
```bash
#!/usr/bin/env bash
# create-or-reuse-release.sh
#
# Makes the `create-release` job safe to run twice on the same tag (PUBOPS-03).
# ...
# Sourcing seam: set CREATE_OR_REUSE_RELEASE_LIB_ONLY=1 before sourcing this
# file to load the create_or_reuse_release_main function (and its helpers)
# without executing it -- this file's own regression harness uses this to
# exercise the function directly.
#
# Usage:  ./scripts/create-or-reuse-release.sh --tag <vX.Y.Z> ...
# Output: on success, prints `upload_url=<value>` and `version=<tag>` to
#         stdout, and appends the same two lines to $GITHUB_OUTPUT ...
# Exit:   0 on success; non-zero for any HTTP-status failure, a malformed
#         response, or a usage error.

set -euo pipefail
```
Adopt the identical structure: a purpose comment naming the requirement ID (ARTIFACT-01), a
`*_LIB_ONLY` sourcing seam env var so the test harness can call the function directly, an explicit
`Usage`/`Output`/`Exit` contract block, and `set -euo pipefail`.

**Core heading-match pattern to reuse verbatim, not reinvent** (`scripts/check-release-consistency.sh`
lines 401-430):
```python
heading_re = re.compile(r"^##\s*\[" + re.escape(tag_version) + r"\](\s|$)")
...
found = False
try:
    with open(changelog_path, "r", encoding="utf-8") as fh:
        for line in fh:
            if heading_re.match(line):
                found = True
                break
```
RESEARCH.md's own "Don't Hand-Roll" table is explicit: the extractor's start-boundary regex must
match this exact anchoring (`(\s|$)` lookahead after the bracketed version, so `0.8.1` never
matches `0.8.1-rc.2` and `0.8.10` never matches `0.8.1`) — either duplicate with a cross-reference
comment (this repo's established precedent, see `finalize-crate-changelogs.sh`'s own comment about
parallel-but-documented implementations) or factor into a tiny shared lib. New behavior beyond the
analog: a **stop boundary** (next `^##\s*\[` line or EOF) the analog does not need, since Clause 2
only checks *presence*, not *content*.

**Tag normalization** (`scripts/check-release-consistency.sh`, referenced in its own header
comment): strip at most one leading `v` via `${TAG#v}` — apply the identical strip, fed the same
input `check-release-consistency.sh` receives (not a second independent parse of `GITHUB_REF`).

**Error/failure-message pattern** (house style, from `check-release-consistency.sh`'s own header
and `scripts/finalize-crate-changelogs.sh`'s disposition-3 case): a named, actionable `::error::`
message, never a bare stack trace or silent fallback. Use the exact wording specified in
CONTEXT.md's `<specifics>`: `"no ## [X.Y.Z] section in CHANGELOG.md -- run make release
VERSION=X.Y.Z (finalizes changelogs) before tagging"`.

**Output-to-file pattern (CR-01)** — never inline the extracted section into a `run:` body or
`$GITHUB_OUTPUT` (a changelog line matching a heredoc delimiter could inject). Follow
`release.yml`'s current changelog-generation step (soon deleted, but its file-output discipline is
exactly right):
```bash
CHANGELOG_FILE="${RUNNER_TEMP}/release-changelog.md"
{ ... } > "$CHANGELOG_FILE"
echo "changelog_file=$CHANGELOG_FILE" >> "$GITHUB_OUTPUT"
```

---

### `scripts/finalize-release-body.sh` (utility, idempotent read-modify-write) — or inline, planner discretion

**Analog:** `scripts/create-or-reuse-release.sh` (full file) — same "thin `gh` CLI wrapper, JSON
built structurally via `jq -n --arg`, tainted text only ever reaches the API via a file or
`--input -`" shape.

**gh API call + status introspection pattern** (`scripts/create-or-reuse-release.sh` lines 61-83):
```bash
_cor_gh_call() {
    local method="$1" endpoint="$2" payload="${3:-}"
    local raw
    if [ -n "${payload}" ]; then
        raw=$(printf '%s' "${payload}" | "${GH}" api -i -X "${method}" "${endpoint}" --input - 2>&1) || true
    else
        raw=$("${GH}" api -i -X "${method}" "${endpoint}" 2>&1) || true
    fi
    local status_line
    status_line=$(printf '%s\n' "${raw}" | head -n1 | tr -d '\r')
    HTTP_STATUS=$(printf '%s' "${status_line}" | sed -nE 's#^HTTP/[0-9.]+ ([0-9]{3}).*#\1#p')
    ...
}
```
For the finalize job, `gh release edit --notes-file` (per RESEARCH.md Pattern 1) is sufficient —
no status-code introspection is needed because a non-2xx exit is already a hard failure — so this
script can be simpler than `create-or-reuse-release.sh`, but should still resolve `gh` through a
`GH_BIN`-style override env var (same testability seam) so the test harness can stub it.

**Idempotent marker-based truncate-and-rebuild** (RESEARCH.md Pattern 1, novel to this phase — no
existing analog does read-modify-write on a release body; compose from first principles per
RESEARCH.md):
```bash
MARKER='<!-- gsd:release-artifacts -->'
CURRENT_BODY=$(gh release view "$TAG" --json body -q .body)
CURATED_SECTION="${CURRENT_BODY%%"$MARKER"*}"
{
  printf '%s\n' "$CURATED_SECTION"
  printf '%s\n' "$MARKER"
  printf '\n---\n\n## Release Artifacts\n\n'
  printf '%s\n' "$DIGEST_LINE" "$ASSET_LIST" "$SBOM_LINE" "$SIZE_LINE"
} > "$RUNNER_TEMP/final-body.md"
gh release edit "$TAG" --notes-file "$RUNNER_TEMP/final-body.md"
```

**Sourcing seam / `*_LIB_ONLY` pattern** — replicate `create-or-reuse-release.sh`'s
`CREATE_OR_REUSE_RELEASE_LIB_ONLY=1` convention so the test harness exercises the composition
function without hitting `gh` for real.

---

### `.github/workflows/release.yml` — job modifications (CI workflow, request-response/batch)

**Analog:** itself, prior revision — every modified job keeps its existing shape; only specific
steps change.

**CR-01 tainted-value-through-env pattern** (already used throughout `release.yml`, e.g.
`create-release`'s `Get version` / `Generate changelog` steps, lines ~110-165):
```yaml
      - name: Create or reuse release
        id: create_or_reuse_release
        env:
          RELEASE_TAG: ${{ steps.get_version.outputs.version }}
          RELEASE_REPOSITORY: ${{ github.repository }}
          RELEASE_BODY_FILE: ${{ steps.changelog.outputs.changelog_file }}
          GH_TOKEN: ${{ secrets.GITHUB_TOKEN }}
        run: |
          ./scripts/create-or-reuse-release.sh --tag "$RELEASE_TAG" --repo "$RELEASE_REPOSITORY" --body-file "$RELEASE_BODY_FILE"
```
Every new step this phase adds (extraction invocation, digest capture, `finalize-release-body`
job) must route tainted values through `env:` the same way, never interpolate directly into
`run:`.

**Job-output plumbing pattern** (`create-release`'s `outputs:` block, to be edited — `upload_url`
removed, `version` kept):
```yaml
    outputs:
      upload_url: ${{ steps.create_or_reuse_release.outputs.upload_url }}   # DELETE (D-07)
      version: ${{ steps.get_version.outputs.version }}                     # KEEP
```
New outputs for `build-docker` (D-09), added the same way:
```yaml
      - name: Build and push
        id: build
        uses: docker/build-push-action@v5
        with: # ...unchanged...
    outputs:
      digest: ${{ steps.build.outputs.digest }}
      tags_json: ${{ steps.meta.outputs.json }}
```

**Image-reference reconstruction pattern to reuse, never hand-roll** (`Verify image size` step,
existing, lines ~275-290):
```bash
IMAGE=$(echo '${{ steps.meta.outputs.json }}' | jq -r '.tags[0]')
```
The new digest pull-line composer in `finalize-release-body` must use this exact idiom
(`steps.meta.outputs.json` via `jq -r '.tags[0]'`), never hand-lowercase `github.repository`.

**Existence-assert-before-archive pattern** (new to this phase, D-06 — no existing analog asserts
binary presence; compose per RESEARCH.md Pattern 2):
```bash
BINARIES=(paladin paladin-cli paladin-server)
MISSING=()
for b in "${BINARIES[@]}"; do
  [ -f "target/${{ matrix.target }}/release/$b" ] || MISSING+=("$b")
done
if [ "${#MISSING[@]}" -gt 0 ]; then
  echo "::error::expected binaries not built for ${{ matrix.target }}: ${MISSING[*]}"
  exit 1
fi
```

**Upload-mechanism replacement pattern** — `actions/upload-release-asset@v1` (3 occurrences: 2 in
`build-binaries`, 1 in `sbom`) replaced with `gh release upload --clobber`, following D-07's
`gh`-CLI-first posture already established by `create-or-reuse-release.sh`:
```yaml
      - name: Upload release asset
        env:
          GH_TOKEN: ${{ secrets.GITHUB_TOKEN }}
        run: gh release upload "${{ needs.create-release.outputs.version }}" \
               target/${{ matrix.target }}/release/${{ matrix.artifact_name }}.tar.gz --clobber
```

**OS-portable checksum pattern** (RESEARCH.md Pattern 3, newly-surfaced — no existing analog in
this repo does OS-conditional hashing):
```bash
sha256_cmd() {
  if command -v sha256sum >/dev/null 2>&1; then sha256sum "$@"; else shasum -a 256 "$@"; fi
}
```

---

### `tests/scripts/extract-changelog-section_test.sh` (test, fixture harness)

**Analog:** `tests/scripts/check-release-consistency_test.sh` (fixture-lifecycle pattern) — read
its full structure before writing; also `tests/scripts/create-or-reuse-release_test.sh` for the
stub-on-PATH pattern if the extractor needs no `gh` stubbing (it should not — pure text
processing, no network).

**Scratch-dir + trap-cleanup + accumulate-failures pattern**
(`tests/scripts/create-or-reuse-release_test.sh` lines 1-40):
```bash
set -uo pipefail
WORKSPACE_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
GUARD="${WORKSPACE_ROOT}/scripts/create-or-reuse-release.sh"
SCRATCH="$(mktemp -d "${TMPDIR:-/tmp}/create-or-reuse-release-test.XXXXXX")"
cleanup() { rm -rf "${SCRATCH}"; }
trap cleanup EXIT
FAILED=0
ASSERTIONS=0
```
Every assertion increments `ASSERTIONS`, and failures accumulate into `FAILED` rather than
exiting on the first mismatch (house style, "report everything"). Mirror this exactly for
`extract-changelog-section_test.sh`, with fixture `CHANGELOG.md` files covering: normal section,
empty/heading-only section (D-02, the live `0.8.1-rc.4` case), missing section (failure path),
`-rc.N` boundary correctness (`0.8.1` must not match `0.8.1-rc.2`), and dated-suffix tolerance
(`## [1.2.3] - 2026-01-01`).

---

### `tests/scripts/finalize-release-body_test.sh` (test, fixture harness, if factored into a script)

**Analog:** `tests/scripts/create-or-reuse-release_test.sh` (`gh` stub-on-PATH mechanism, lines
40-90) — write a scripted `gh` stand-in exposing `release view --json body` and `release edit
--notes-file` behaviors, with a call log. Reuse the same `GH_BIN` env-var seam
`create-or-reuse-release.sh` established, applied to the new script's own `GH_BIN`.

---

### `21-ARTIFACT-EVIDENCE.md` (evidence record)

**Analog:** `.planning/phases/20-release-pipeline-recovery-idempotent-re-runs-and-a-pre-publi/20-RECOVERY-EVIDENCE.md`
and `.planning/phases/19-crates-io-trusted-publishing-replace-the-long-lived-registry/19-PUBLISH-EVIDENCE.md`

Shape to copy: dated, measured, run-URL-sourced entries per acceptance criterion — never
"re-reading the workflow" cited as evidence (the phase's own honesty rule, ARTIFACT-02's defect
"survived every prior reading"). If the D-14 rehearsal is not run, the file must explicitly record
the artifact path as **unverified**, not silently omitted.

---

## Shared Patterns

### CR-01 — tainted value indirection
**Source:** `.github/workflows/release.yml` (used throughout `create-release`, `build-docker`,
`check-release-consistency`)
**Apply to:** every new/modified step in `release.yml`, and every new script invocation
```yaml
env:
  RELEASE_TAG: ${{ steps.get_version.outputs.version }}
run: ./scripts/some-script.sh --tag "$RELEASE_TAG"
```

### `gh` CLI as the sole release-API surface (no new marketplace actions)
**Source:** `scripts/create-or-reuse-release.sh`, `scripts/publish-crates.sh`
**Apply to:** `finalize-release-body.sh`, the `build-binaries`/`sbom` upload-step replacements,
SHA256SUMS aggregation (`gh release download` / `gh release upload --clobber`)

### Script sourcing seam for testability (`*_LIB_ONLY`)
**Source:** `scripts/create-or-reuse-release.sh` header comment; consumed by
`tests/scripts/create-or-reuse-release_test.sh`
**Apply to:** `extract-changelog-section.sh`, `finalize-release-body.sh` — both must expose a
`_main` function callable without executing, via a `*_LIB_ONLY=1` env-var guard.

### Fixture-harness test structure (no third-party framework)
**Source:** `tests/scripts/check-release-consistency_test.sh`,
`tests/scripts/create-or-reuse-release_test.sh`
**Apply to:** all new `tests/scripts/*_test.sh` files — `mktemp -d` scratch dir, `trap cleanup
EXIT`, real-tree mutation baseline check via `git status --porcelain`, accumulate-into-`$FAILED`
assertions, no network.

### "Don't duplicate the heading regex silently" cross-reference discipline
**Source:** `scripts/check-release-consistency.sh` Clause 2 comment;
`scripts/finalize-crate-changelogs.sh` header comment (documents its own parallel-implementation
precedent)
**Apply to:** `extract-changelog-section.sh` — must either source a shared regex or carry an
explicit comment cross-referencing `check-release-consistency.sh`'s line, per this repo's
established "documented parallel implementation over premature DRY" convention.

### Image-reference reconstruction via `metadata-action` JSON, never hand-built
**Source:** `.github/workflows/release.yml` `Verify image size` step's own comment and code
**Apply to:** the new digest pull-line composer in `finalize-release-body`

## No Analog Found

| File | Role | Data Flow | Reason |
|---|---|---|---|
| Marker-based truncate-and-rebuild body composition logic | utility (embedded in `finalize-release-body.sh`) | idempotent read-modify-write | No existing script in this repo performs a read-modify-write cycle against a live GitHub release body; RESEARCH.md's Pattern 1 (Code Examples section) is the closest thing to an analog and should be treated as the primary reference for this specific piece, not a codebase file. |
| SHA256SUMS aggregation across matrix legs | utility (embedded in `finalize-release-body.sh` per RESEARCH.md's recommendation) | batch (fan-in over `gh release download`) | No existing script aggregates artifacts across isolated matrix runners; RESEARCH.md's Code Examples section (`gh release download --pattern '*.tar.gz'; sha256sum *.tar.gz > SHA256SUMS`) is the reference. |
| Docker digest capture as a job output | CI workflow step | request-response | `build-docker` has never exposed `digest` as an output before; RESEARCH.md's Code Examples section is the reference, not an in-repo analog. |

## Metadata

**Analog search scope:** `scripts/`, `tests/scripts/`, `.github/workflows/release.yml`,
`.planning/phases/19-*`, `.planning/phases/20-*`
**Files scanned:** `scripts/create-or-reuse-release.sh`, `scripts/check-release-consistency.sh`,
`scripts/finalize-crate-changelogs.sh`, `scripts/publish-crates.sh`,
`tests/scripts/create-or-reuse-release_test.sh`, `tests/scripts/check-release-consistency_test.sh`,
`.github/workflows/release.yml` (full file, all jobs), `19-PUBLISH-EVIDENCE.md`,
`20-RECOVERY-EVIDENCE.md` (referenced by shape, not re-quoted here)
**Pattern extraction date:** 2026-08-31
