# Phase 21 — External API Coverage Matrix

> Full coverage by default. Opt-outs are explicit, reasoned decisions.

**Produced:** 2026-08-31 (plan time); reformatted to the canonical 3-column matrix 2026-09-01
(the seal-time parser reads `| capability | decision | reason |` — the original 5-column layout
made it misread the endpoint column as the decision).
**Detector:** `api-coverage.cjs --json` over ROADMAP §Phase 21 → `detected: false`. Authored anyway
because the seal-time gate re-scans phase scope including the PLAN.md bodies, which reference the
GitHub Releases API, the ghcr.io registry and Docker content digests. Follows the Phase 20
precedent (`.planning/phases/20-release-pipeline-recovery-idempotent-re-runs-and-a-pre-publi/COVERAGE.md`).

Two external services are in scope this phase: the **GitHub REST API** (reached through the `gh`
CLI with the runner's `GITHUB_TOKEN`) and **ghcr.io** (reached through `docker/build-push-action`
and `docker pull`). crates.io is reached only incidentally, by the D-14 rehearsal tag triggering
the unmodified `publish-crates` job — its capability surface is Phase 20's and is not re-decided
here.

## GitHub REST API (via `gh`)

| capability | decision | reason |
|---|---|---|
| Get release by tag — `GET repos/{owner}/{repo}/releases/tags/{tag}` via `create-or-reuse-release.sh` (plan 21-01) | INTEGRATE | |
| Create release with a body — `POST repos/{owner}/{repo}/releases` via `create-or-reuse-release.sh` `--body-file` (plan 21-01) | INTEGRATE | |
| Read current release body — `gh release view <tag> --json body -q .body` (plan 21-03) | INTEGRATE | |
| Edit release notes — `gh release edit <tag> --notes-file <path>` (`PATCH .../releases/{id}`, plan 21-03) | INTEGRATE | |
| Upload release asset — `gh release upload <tag> <file> --clobber` (plans 21-02, 21-04) | INTEGRATE | |
| Download release assets — `gh release download <tag> --pattern '*.tar.gz'` (plan 21-04) | INTEGRATE | |
| Read `upload_url` from the release object (response field) | OPT-OUT | D-07 deletes the `upload_url` plumbing; `gh release upload` resolves the release by tag itself, so no workflow job consumes the field again |
| Delete release — `DELETE repos/{owner}/{repo}/releases/{id}` | OPT-OUT | never used: a release object is reused and rewritten in place (Phase 20 D-03 recovery posture), never destroyed |
| Delete release asset — `DELETE .../releases/assets/{id}` | OPT-OUT | `--clobber` performs the replace internally; hand-rolling delete-then-upload reintroduces the interrupted-upload data-loss window (RESEARCH.md "Don't Hand-Roll") |
| List releases — `GET repos/{owner}/{repo}/releases` | OPT-OUT | every call in this phase addresses one known tag; no listing or pagination is needed |
| Publish/unpublish toggling — `PATCH .../releases/{id}` with `draft`/`prerelease` | OPT-OUT | prerelease flagging is already decided by `create-or-reuse-release.sh`'s hyphen rule (Phase 20); this phase only rewrites `body` |
| Re-run workflow / failed jobs — `POST .../actions/runs/{id}/rerun*` | OPT-OUT | Phase 20 D-15 keeps re-runs an operator action in the Actions UI; automating it would let CI retrigger its own publish path |
| Release reactions / discussions — `POST .../releases/{id}/reactions`, `discussion_category_name` | OPT-OUT | not needed by any ARTIFACT requirement; the release is a distribution surface here, not a discussion surface |

## ghcr.io (via Docker)

| capability | decision | reason |
|---|---|---|
| Push multi-arch image and read the returned content digest — `docker/build-push-action@v5` `steps.build.outputs.digest` (plan 21-03) | INTEGRATE | |
| Read the exact pushed tag list — `docker/metadata-action@v5` `steps.meta.outputs.json` (plan 21-03) | INTEGRATE | |
| Pull an image by tag and inspect its size — `docker pull` + `docker image inspect --format='{{.Size}}'` (plan 21-03) | INTEGRATE | |
| Pull an image by immutable digest — `docker pull <image>@sha256:<hex>` (plan 21-06; in-CI corroboration only — the out-of-band pull-by-digest is pending human confirmation, UAT item 1 in `21-UAT.md`; 21-VERIFICATION.md holds this at human_needed) | INTEGRATE | |
| Registry tag listing — `GET /v2/{name}/tags/list` | OPT-OUT | the pushed tag list already comes from `metadata-action`'s own output; querying the registry would be a second, divergent source of truth for the same fact |
| Manifest delete / untag — `DELETE /v2/{name}/manifests/{ref}` | OPT-OUT | no phase deliverable removes an image; the `:latest` change (D-09) is a body-text deletion, not a registry mutation |
| Package visibility / retention settings — `PATCH /user/packages/container/{name}` | OPT-OUT | registry administration is an operator concern, out of scope per CONTEXT.md |

## Cross-cutting call discipline (applies to every INTEGRATE row above)

- No credential-bearing call follows redirects — `-L`/`--location` is never passed to `gh` or
  `curl` (`security.instructions.md`; the same control `check-release-consistency.sh` documents).
- Every tag- or version-derived value reaches a `run:` block through `env:`, never through direct
  `${{ }}` interpolation (repo convention CR-01).
- Every multi-line or author-written body reaches the API through a file (`--body-file`,
  `--notes-file`) or `jq -n --arg`, never string-concatenated into a shell command.
- A non-2xx from any `gh` invocation is a hard, named failure; no call falls through to a
  "probably fine" default.
- The image reference for any digest-pinned pull is read from `metadata-action`'s JSON output,
  never reconstructed by hand-lowercasing `github.repository`.
