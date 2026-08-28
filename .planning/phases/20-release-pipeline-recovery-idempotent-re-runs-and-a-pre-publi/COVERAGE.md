# Phase 20 — External API Coverage Matrix

**Produced:** 2026-08-28 (plan time)
**Detector:** `api-coverage.cjs --json` → `detected: true` (signal: "query the GitHub API for
`ci.yml`'s workflow run(s) on the exact tagged commit SHA")
**Default posture:** Full API coverage by default — `INTEGRATE` unless a one-line reason opts out.

Two external services are in scope this phase: the **GitHub REST API** (reached through `gh api`
with the runner's `GITHUB_TOKEN`) and **crates.io** (the `api/v1` DB endpoint and the
`index.crates.io` sparse index, reached through `curl`). The capability surface below is the set
of operations the phase's deliverables actually need; nothing outside it is exercised.

## GitHub REST API (via `gh api`)

| Capability | Endpoint | Disposition | Plan | Reason (opt-outs only) |
|---|---|---|---|---|
| Get release by tag | `GET repos/{owner}/{repo}/releases/tags/{tag}` | INTEGRATE | 20-03 | — |
| Create release | `POST repos/{owner}/{repo}/releases` | INTEGRATE | 20-03 | — |
| Read `upload_url` from the release object | (response field of both calls above) | INTEGRATE | 20-03 | — |
| List workflow runs for a workflow file, filtered by head SHA | `GET repos/{owner}/{repo}/actions/workflows/ci.yml/runs?head_sha=&status=completed` | INTEGRATE | 20-02 | — |
| Read a run's `conclusion` / `created_at` / `id` | (response fields of the call above) | INTEGRATE | 20-02 | — |
| Update release (`PATCH .../releases/{id}`) | `PATCH repos/{owner}/{repo}/releases/{id}` | OPT-OUT | — | Release **body/notes content** is Phase 21 (`ARTIFACT-*`), explicitly deferred in CONTEXT.md; Phase 20 reuses an existing release unchanged rather than rewriting it. |
| Upload release asset | `POST uploads.github.com/.../releases/{id}/assets` | OPT-OUT | — | Asset plumbing (`actions/upload-release-asset@v1`) is Phase 21; D-02 requires Phase 20 to preserve the existing `upload_url` contract, not to replace the uploader. |
| Delete release | `DELETE repos/{owner}/{repo}/releases/{id}` | OPT-OUT | — | Never used: recovery is complete-forward (D-03/D-13); a release object is reused, never destroyed. |
| Re-run workflow / re-run failed jobs | `POST .../actions/runs/{id}/rerun*` | OPT-OUT | — | D-15 makes the re-run an **operator action in the Actions UI**, not an automated call; automating it would let CI retrigger its own publish path. |
| List workflow-run jobs | `GET .../actions/runs/{id}/jobs` | OPT-OUT | — | The CI-conclusion granularity decision (20-02) is **whole-run**, evidenced against `ci.yml`'s actual job list; per-job resolution is deliberately not adopted, so this endpoint is not needed. |

## crates.io

| Capability | Endpoint | Disposition | Plan | Reason (opt-outs only) |
|---|---|---|---|---|
| Crate-version existence pre-check | `GET https://crates.io/api/v1/crates/{name}/{version}` | INTEGRATE | 20-05 | — |
| Sparse-index visibility poll | `GET https://index.crates.io/{prefix-path}/{name}` | INTEGRATE | 20-05 | — |
| Read `yanked` flag from the index line | (response field of the call above) | INTEGRATE | 20-05 | — |
| Per-crate recovery-state queries (operator) | `GET https://crates.io/api/v1/crates/{name}/{version}` | INTEGRATE | 20-06 (documented in the runbook) | — |
| Publish a crate version | `PUT /api/v1/crates/new` | INTEGRATE (indirectly) | 20-05 | Reached only through `cargo publish -p`; the phase never calls the publish endpoint directly. |
| Yank / unyank a version | `DELETE|PUT /api/v1/crates/{name}/{version}/yank` | OPT-OUT (documented, not automated) | 20-06 | D-13: yanking is a **human act by the crate-owner account**, never CI. The runbook documents `cargo yank` and the yank register; no workflow, script or make target may call it. |
| Owner add/remove | `PUT|DELETE /api/v1/crates/{name}/owners` | OPT-OUT | — | Ownership is settled (Phase 19); no phase deliverable mutates it. |
| Full-text crate search / download stats | `GET /api/v1/crates?q=` etc. | OPT-OUT | — | Not needed for any PUBOPS requirement; the phase only ever asks "does this exact `name@version` exist". |

## Cross-cutting call discipline (applies to every INTEGRATE row above)

- Every crates.io call sends a `User-Agent` header — crates.io answers `403` without one
  (ADR-0026 / `19-PUBLISH-EVIDENCE.md`).
- No credential-bearing call follows redirects (`-L` is not passed) —
  `security.instructions.md` control.
- HTTP status is branched on explicitly (`200` / `404` / `429` / other); a non-2xx that is not a
  recognised status is a hard, named failure, never an implicit "not published".
- Response shape is validated with `jq -e` before use; a malformed body fails loudly.
- Every tag-derived or SHA-derived value reaches a `run:` block through `env:`, never through
  direct `${{ }}` interpolation (repo convention CR-01).
