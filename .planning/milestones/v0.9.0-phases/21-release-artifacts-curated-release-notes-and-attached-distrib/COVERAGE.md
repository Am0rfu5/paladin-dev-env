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

## Capability matrix (GitHub REST API via `gh`, and ghcr.io via Docker)

| capability | decision | reason |
|---|---|---|
| gh: get release by tag | INTEGRATE | `GET repos/{owner}/{repo}/releases/tags/{tag}` via `create-or-reuse-release.sh` (plan 21-01) |
| gh: create release with body | INTEGRATE | `POST repos/{owner}/{repo}/releases` via `create-or-reuse-release.sh` `--body-file` (plan 21-01) |
| gh: read release body | INTEGRATE | `gh release view <tag> --json body -q .body` (plan 21-03) |
| gh: edit release notes | INTEGRATE | `gh release edit <tag> --notes-file <path>` (`PATCH .../releases/{id}`, plan 21-03) |
| gh: upload release asset | INTEGRATE | `gh release upload <tag> <file> --clobber` (plans 21-02, 21-04) |
| gh: download release assets | INTEGRATE | `gh release download <tag> --pattern '*.tar.gz'` (plan 21-04) |
| gh: read upload_url field | OPT-OUT | D-07 deletes the `upload_url` plumbing; `gh release upload` resolves the release by tag itself, so no workflow job consumes the field again |
| gh: delete release | OPT-OUT | never used: a release object is reused and rewritten in place (Phase 20 D-03 recovery posture), never destroyed |
| gh: delete release asset | OPT-OUT | `--clobber` performs the replace internally; hand-rolling delete-then-upload reintroduces the interrupted-upload data-loss window (RESEARCH.md "Don't Hand-Roll") |
| gh: list releases | OPT-OUT | every call in this phase addresses one known tag; no listing or pagination is needed |
| gh: toggle draft/prerelease | OPT-OUT | prerelease flagging is already decided by `create-or-reuse-release.sh`'s hyphen rule (Phase 20); this phase only rewrites `body` |
| gh: re-run workflow/jobs | OPT-OUT | Phase 20 D-15 keeps re-runs an operator action in the Actions UI; automating it would let CI retrigger its own publish path |
| gh: reactions/discussions | OPT-OUT | not needed by any ARTIFACT requirement; the release is a distribution surface here, not a discussion surface |
| ghcr: push image, read digest | INTEGRATE | `docker/build-push-action@v5` `steps.build.outputs.digest` (plan 21-03) |
| ghcr: read pushed tag list | INTEGRATE | `docker/metadata-action@v5` `steps.meta.outputs.json` (plan 21-03) |
| ghcr: pull by tag, inspect size | INTEGRATE | `docker pull` + `docker image inspect --format='{{.Size}}'` (plan 21-03) |
| ghcr: pull by immutable digest | INTEGRATE | `docker pull <image>@sha256:<hex>` (plan 21-06); in-CI corroboration only — out-of-band pull pending human confirmation, UAT item 1 in `21-UAT.md` |
| ghcr: registry tag listing | OPT-OUT | the pushed tag list already comes from `metadata-action`'s own output; querying the registry would be a second, divergent source of truth for the same fact |
| ghcr: manifest delete/untag | OPT-OUT | no phase deliverable removes an image; the `:latest` change (D-09) is a body-text deletion, not a registry mutation |
| ghcr: package visibility/retention | OPT-OUT | registry administration is an operator concern, out of scope per CONTEXT.md |

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
