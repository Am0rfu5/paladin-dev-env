# Phase 21 — External API Coverage Matrix

**Produced:** 2026-08-31 (plan time)
**Detector:** `api-coverage.cjs --json` over ROADMAP §Phase 21 → `detected: false`. Authored anyway
because the seal-time gate (`check api-coverage.verify-pre`) re-scans phase scope including the
PLAN.md bodies, which do reference the GitHub Releases API, the ghcr.io registry and Docker
content digests. Follows the Phase 20 precedent
(`.planning/phases/20-release-pipeline-recovery-idempotent-re-runs-and-a-pre-publi/COVERAGE.md`).
**Default posture:** Full API coverage by default — `INTEGRATE` unless a one-line reason opts out.

Two external services are in scope this phase: the **GitHub REST API** (reached through the `gh`
CLI with the runner's `GITHUB_TOKEN`) and **ghcr.io** (reached through `docker/build-push-action`
and `docker pull`). crates.io is reached only incidentally, by the D-14 rehearsal tag triggering
the unmodified `publish-crates` job — its capability surface is Phase 20's and is not re-decided
here.

## GitHub REST API (via `gh`)

| Capability | Endpoint / command | Disposition | Plan | Reason (opt-outs only) |
|---|---|---|---|---|
| Get release by tag | `GET repos/{owner}/{repo}/releases/tags/{tag}` (`create-or-reuse-release.sh`) | INTEGRATE | 21-01 | — |
| Create release with a body | `POST repos/{owner}/{repo}/releases` (`create-or-reuse-release.sh`, `--body-file`) | INTEGRATE | 21-01 | — |
| Read current release body | `gh release view <tag> --json body -q .body` | INTEGRATE | 21-03 | — |
| Edit release notes | `gh release edit <tag> --notes-file <path>` (`PATCH .../releases/{id}`) | INTEGRATE | 21-03 | — |
| Upload release asset | `gh release upload <tag> <file> --clobber` | INTEGRATE | 21-02, 21-04 | — |
| Download release assets | `gh release download <tag> --pattern '*.tar.gz'` | INTEGRATE | 21-04 | — |
| Read `upload_url` from the release object | (response field) | OPT-OUT | — | D-07 deletes the `upload_url` plumbing from the workflow; `gh release upload` resolves the release by tag itself, so the field is never consumed by a workflow job again. |
| Delete release | `DELETE repos/{owner}/{repo}/releases/{id}` | OPT-OUT | — | Never used: a release object is reused and rewritten in place (Phase 20 D-03 recovery posture), never destroyed. |
| Delete release asset | `DELETE .../releases/assets/{id}` | OPT-OUT | — | `--clobber` performs the replace internally; hand-rolling delete-then-upload would reintroduce the interrupted-upload data-loss window (RESEARCH.md "Don't Hand-Roll"). |
| List releases | `GET repos/{owner}/{repo}/releases` | OPT-OUT | — | Every call in this phase addresses one known tag; no listing or pagination is needed. |
| Publish/unpublish (draft, prerelease toggling) | `PATCH .../releases/{id}` with `draft`/`prerelease` | OPT-OUT | — | Prerelease flagging is already decided by `create-or-reuse-release.sh`'s hyphen rule (Phase 20); this phase only rewrites `body`. |
| Re-run workflow / re-run failed jobs | `POST .../actions/runs/{id}/rerun*` | OPT-OUT | — | Phase 20 D-15 keeps the re-run an operator action in the Actions UI; automating it would let CI retrigger its own publish path. |
| Release reactions / discussions | `POST .../releases/{id}/reactions`, `discussion_category_name` | OPT-OUT | — | Not needed by any ARTIFACT requirement; the release is a distribution surface here, not a discussion surface. |

## ghcr.io (via Docker)

| Capability | Endpoint / command | Disposition | Plan | Reason (opt-outs only) |
|---|---|---|---|---|
| Push multi-arch image + read the returned content digest | `docker/build-push-action@v5` → `steps.build.outputs.digest` | INTEGRATE | 21-03 | — |
| Read the exact pushed tag list | `docker/metadata-action@v5` → `steps.meta.outputs.json` | INTEGRATE | 21-03 | — |
| Pull an image by tag and inspect its size | `docker pull` + `docker image inspect --format='{{.Size}}'` | INTEGRATE | 21-03 | — |
| Pull an image by immutable digest | `docker pull <image>@sha256:<hex>` | INTEGRATE | 21-06 (rehearsal proof) | — |
| Registry `_catalog` / tag listing | `GET /v2/{name}/tags/list` | OPT-OUT | — | The pushed tag list already comes from `metadata-action`'s own output; querying the registry would be a second, divergent source of truth for the same fact. |
| Manifest delete / untag | `DELETE /v2/{name}/manifests/{ref}` | OPT-OUT | — | No phase deliverable removes an image; the `:latest` change (D-09) is a body-text deletion, not a registry mutation. |
| Package visibility / retention settings | `PATCH /user/packages/container/{name}` | OPT-OUT | — | Registry administration is an operator concern, out of scope per CONTEXT.md. |

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
