# Phase 20: Release Pipeline Recovery — Idempotent Re-Runs and a Pre-Publish Gate - Research

**Researched:** 2026-08-28
**Domain:** GitHub Actions release-pipeline idempotency, crates.io registry-state polling, GitHub API workflow-conclusion resolution, cargo-release changelog tooling
**Confidence:** HIGH (workflow/API mechanics, verified against this repo and official API docs) / MEDIUM (cargo publish --workspace behavior, no maintainer-confirmed resume semantics found)

<user_constraints>
## User Constraints (from CONTEXT.md)

### Locked Decisions

**Create-release idempotency (PUBOPS-03, criterion 1)**
- **D-01:** `actions/create-release@v1` (archived since 2021) is replaced with `gh` CLI / `gh api` create-or-reuse logic: if a release for the tag exists, reuse it; if not, create it. No new third-party action is introduced.
- **D-02:** The job's outputs contract is preserved: it continues to emit `upload_url` and `version` (fetched via `gh api repos/{owner}/{repo}/releases/tags/{tag}` on the reuse path), because `build-binaries` and `sbom` consume `needs.create-release.outputs.upload_url` via `actions/upload-release-asset@v1` today. Phase 21 owns removing that plumbing; Phase 20 must not break it.
- **D-03:** Every job on the tag→publish path must be safe to run twice on the same tag. Both re-run shapes are supported: "Re-run failed jobs" (primary recovery) and "Re-run all jobs" (must not fail on already-done work).

**Already-published detection and index wait (PUBOPS-03, criterion 2)**
- **D-04:** "Already published" is determined from registry state, never from matched error prose: the crates.io API (or sparse index) is queried for `crate@version` before attempting `cargo publish`, and a crate already at the tagged version is skipped and recorded as `already-at-this-version`. The `grep -qiE` tolerance is deleted. All crates.io API calls send a `User-Agent` header.
- **D-05:** The fixed `sleep 20` between crates is replaced by polling the registry until the just-published version is visible, with a bounded timeout — a check, not a guess.
- **D-06:** Researcher must evaluate `cargo publish --workspace` as the carrier for D-04/D-05. Adoption is conditional on verifying (a) the pinned toolchain supports it for real publishes, (b) it tolerates a partially-published workspace, and (c) per-crate outcomes (PUBOPS-04) remain derivable. If any of the three fails, keep the explicit per-crate loop with API pre-checks. Either way the detection principle in D-04 is binding.

**Pre-publish gate (PUBOPS-01)**
- **D-07:** The gate is a repo script (e.g. `scripts/check-release-consistency.sh` — exact name at planner discretion) invoked by a new job in `release.yml` that `publish-crates` `needs`, and runnable locally (make target). It runs before the first `cargo publish`; whether `create-release` also needs it is planner discretion.
- **D-08:** The gate checks that ALL of the following agree, and reports every mismatch found, not the first: the tag version; the eleven publishable manifests' `version` fields (read from `cargo metadata`, not a hardcoded list); a `## [X.Y.Z]` section in the root `CHANGELOG.md`; and a `## [X.Y.Z]` section in each of the ten crate changelogs. It also performs the PUBOPS-02 CI-conclusion check (D-10) or depends on the job that does.
- **D-09:** Prerelease tags get the same gate as stable tags — no exempted path. The release tooling (`make release` / cargo-release configuration) is extended in this phase to finalize the ten crate changelogs mechanically alongside the root one. Researcher verifies cargo-release's per-crate changelog/replacement support; fallback is a small finalize script.

**CI-conclusion verification (PUBOPS-02)**
- **D-10:** This phase takes "resolve the recorded CI conclusion for the tagged SHA": query the GitHub API for `ci.yml`'s workflow run(s) on the exact tagged commit SHA and require a recorded successful conclusion; refuse to publish without one.
- **D-11:** `ci.yml` triggers are not touched (no tag trigger added). The failure message when no successful run exists tells the operator exactly what to do.

**Per-crate outcome reporting (PUBOPS-04)**
- **D-12:** The publish job emits a per-crate outcome table (written to `$GITHUB_STEP_SUMMARY` and echoed in the log): each of the eleven crates gets exactly one of `published-now` / `already-at-this-version` / `skipped` / `failed`. A run in which no crate reached `published-now` fails with a distinct, self-diagnosing message.

**Runbook and yank policy (PUBOPS-05)**
- **D-13:** The runbook is a new `docs/src/appendix/release-recovery.md`, beside `release-automation.md` and `release-checklist.md`, cross-linked from both. Content: how to establish which crates reached crates.io; default recovery is complete forward by re-running the same tag's workflow run (D-03); a published version is never deleted or re-uploaded — correct via new patch + `cargo yank`, never a retry of the same version; who may yank (the crate-owner account); every yank recorded in a "Yank register" table (version, crates, reason, owner, date), kept out of `SECURITY-EXCEPTIONS.md`.

**Recovery rehearsal (PUBOPS-05 / criterion 7)**
- **D-14:** The rehearsal induces a real partial failure on a throwaway prerelease version (the next rc on the current line): let the publish loop land some crates, stop it mid-loop, verify the half-published state via the registry, then perform the documented recovery — re-running the same tag-push workflow run — and verify it completes the set. `cargo publish --dry-run` is explicitly not acceptable evidence. Evidence recorded in a phase evidence file (`19-PUBLISH-EVIDENCE.md` style). If not run, the runbook is labelled **untested**.
- **D-15:** Recovery is designed around re-running the existing tag-push run, not `workflow_dispatch` (Phase 19's assumption A1 is untested). If the rehearsal happens to prove dispatch works too, record it; do not depend on it.

### Claude's Discretion
- Exact gate script name, make target name, and the new gate job's name/position in the `needs` graph (binding constraint: before the first real `cargo publish`).
- Whether `cargo publish --workspace` or the explicit loop carries D-04/D-05 (per D-06, after researcher verification — **this research resolves it below: keep the explicit loop**).
- The precise induced-failure mechanism in the rehearsal (cancellation vs injected fault), and the rc version string used.
- Whether the CI-conclusion check lives inside the gate script or as its own job/step.
- Runbook prose structure, provided the D-13 content list is fully answered.

### Deferred Ideas (OUT OF SCOPE)
- Release body from curated `CHANGELOG.md` section, attached binaries/digest/SBOM fixes, `upload_url` plumbing removal, `upload-release-asset@v1` replacement — Phase 21 (`ARTIFACT-*`).
- The real stable catch-up release (registry at prerelease rc's; last stable is 0.5.1) — an operator act once this phase's machinery exists.
- Environment required-reviewer gate on `crates-io` — deferred by Phase 19 D-08 until this runbook defines who approves releases; the runbook names who may yank, whether that person also becomes a required reviewer is a follow-up settings decision, not taken silently here.
- `workflow_dispatch` eligibility under Trusted Publishing (A1) — remains untested unless the rehearsal incidentally proves it.
</user_constraints>

<phase_requirements>
## Phase Requirements

| ID | Description | Research Support |
|----|-------------|------------------|
| PUBOPS-01 | No crate published until tag, manifests, changelogs agree; gate reports every mismatch | `## Architecture Patterns` Pattern 2 (the gate script), `## Code Examples` §1, D-08 enumeration via `cargo metadata`, existing `check-changelogs.sh`/`check-crate-names.sh` precedent scripts |
| PUBOPS-02 | CI conclusion for tagged SHA verified via API, not inferred | `## Code Examples` §4 (`gh api .../actions/workflows/{file}/runs?head_sha=`), `## Common Pitfalls` Pitfall 3 |
| PUBOPS-03 | Re-running the same tag is idempotent end-to-end; already-published from registry state | `## Architecture Patterns` Pattern 1 (create-or-reuse release), Pattern 3 (registry pre-check + index-wait poll), `cargo publish --workspace` evaluation (D-06 verdict below) |
| PUBOPS-04 | Per-crate outcome table; no-crate-moved run fails | `## Code Examples` §3, `## Architecture Patterns` Pattern 3 |
| PUBOPS-05 | Runbook + yank policy + rehearsal | `## Common Pitfalls`, `## State of the Art`, runbook content checklist under Pattern 4 |
</phase_requirements>

## Project Constraints (from CLAUDE.md)

- TDD / Red-Green-Refactor is the stated working agreement, but this phase's primary deliverables are **bash scripts and a GitHub Actions workflow**, not Rust library code — the 82% workspace line-coverage floor (ADR-0006) does not apply to `.sh`/`.yml` files, and `cargo llvm-cov` cannot instrument them. The applicable test discipline is the repo's own established shell-guard pattern: `tests/scripts/<name>_test.sh`, fixture-lifecycle via `mktemp -d` + `trap cleanup EXIT`, wired into `make test-shell-guards`. New scripts this phase adds MUST follow this pattern, not a Rust unit-test pattern.
- Before committing a parent task: run `cargo fmt --check` / `cargo clippy` are **not applicable** to shell/YAML changes; substitute `shellcheck` (already used implicitly by this repo's script style — verify no existing `make shellcheck` target and add one if establishing a new script family) and `actionlint` or GitHub's own workflow syntax validation. `make security` (cargo-audit + cargo-deny) is still relevant only if this phase changes `Cargo.toml`/`Cargo.lock` — it should not.
- Ubiquitous language: no Medieval Military renaming applies to this phase's artifacts (`release.yml`, `crates-io` environment, publish job names are already established proper nouns from Phase 19 and are out of scope to rename).
- Security: `make security` and the manual credential-handling review apply directly — this phase touches `id-token: write` scoped jobs, `GITHUB_TOKEN` usage via `gh`/`gh api`, and crates.io API calls. No response body redaction concern here (no LLM API keys in play), but the HTTP-client-following-redirects concern from `security.instructions.md` is relevant to any new `curl`/`gh api` call this phase adds — see `## Security Domain` below.
- Conventional commits and the parent-task stop-and-wait protocol govern how this phase's plan should be executed once written.

## Summary

Phase 20 turns a release pipeline that can neither be safely re-run nor verified before it
publishes into one that supports both. The codebase evidence (read directly, not inferred) shows
`release.yml` already has the right *shape* for most of the fix: `verify-tag-source` → `test` /
`create-release` → `build-docker` / `build-binaries` / `sbom` / `publish-crates`, with
`publish-crates` already gated `needs: [test, create-release]`. The two defects the phase must
close are narrow and mechanical: (1) `create-release`'s `actions/create-release@v1` step 422s on a
tag that already has a GitHub release, which — because `publish-crates` needs `create-release` —
makes the one existing recovery tolerance in `publish_one()` (a `grep` over `cargo publish`
output) unreachable on exactly the run that needs it; and (2) nothing checks tag/manifest/changelog
agreement or the tagged commit's CI conclusion before that first `cargo publish` fires.

The central technical question this research had to resolve — whether native `cargo publish
--workspace` (stabilized in Rust 1.90, well below the pinned 1.97.1 toolchain) can replace the
existing `publish_one()` loop — resolves **against** adoption. Cargo's own workspace-publish
feature is documented as "non-atomic," with no discovered maintainer statement that a re-run
skips crates already at the target version, and the surrounding tooling ecosystem (cargo-release's
own open `--skip-published` feature request, and three independent third-party wrapper crates —
`cargo-workspaces`, `cargo-publish-all`, `shipper` — that exist specifically to add this behavior)
is itself the evidence that stock `cargo publish --workspace` does not solve resumability. The
existing `publish_one()` loop, dependency order and all, stays the carrier; only its detection
(registry API pre-check replacing the `grep`) and its wait (registry poll replacing `sleep 20`)
change.

The GitHub Actions mechanics this phase depends on are all directly confirmed against this
repository's live state or official API docs: `gh api repos/{owner}/{repo}/releases/tags/{tag}`
returns the release object (including `upload_url`) for the create-or-reuse pattern; the
`actions/runs`-by-workflow-file endpoint supports a `head_sha` query parameter, which is the exact
mechanism PUBOPS-02 needs to resolve `ci.yml`'s conclusion for the tagged commit without adding a
tag trigger to `ci.yml` itself; and GitHub's documented re-run semantics ("Re-run failed jobs"
reuses the original `GITHUB_SHA`/`GITHUB_REF`) mean a re-run of the tag-push event keeps the
`v*.*.*` ref the `crates-io` environment's deployment policy and OIDC subject claim require —
directly supporting D-15's built-in design choice.

**Primary recommendation:** keep `publish_one()`'s per-crate bash loop as the publish carrier;
replace its detection (`grep` on stdout → `curl` against `https://crates.io/api/v1/crates/<name>/<version>`
with the required `User-Agent`) and its wait (`sleep 20` → poll the same endpoint, or the sparse
index at `https://index.crates.io/<prefix>/<name>`, with a bounded timeout); replace
`actions/create-release@v1` with a `gh api` create-or-reuse step that preserves the `upload_url`/
`version` outputs; add a new gate job/script that reads `cargo metadata` for the eleven manifest
versions, greps each changelog for a `## [X.Y.Z]` heading, and resolves `ci.yml`'s conclusion for
the tagged SHA via `gh api .../actions/workflows/ci.yml/runs?head_sha=<sha>`, reporting every
mismatch before failing; and extend `cargo-release`/the finalize step to also stamp the ten crate
changelogs (cargo-release's `pre-release-replacements` is package-scoped, not workspace-scoped —
this phase's fallback finalize script, not a `release.toml` change, is the practical path).

## Architectural Responsibility Map

| Capability | Primary Tier | Secondary Tier | Rationale |
|------------|-------------|----------------|-----------|
| Release-object create-or-reuse | CI / Release workflow (`release.yml` `create-release` job) | GitHub REST API (via `gh api`) | The GitHub release object is server-side state owned by GitHub; the workflow job is the only actor that should mutate it, and reuse-detection must happen server-side (a 404 on `releases/tags/{tag}` vs 200) rather than being inferred locally |
| Already-published detection | CI / Release workflow (`publish-crates` job, pre-check step) | crates.io registry (source of truth) | The registry, not the workflow's own memory of what it did last run, is the only thing that can answer "is this crate@version already published" correctly across separate workflow runs |
| Index-visibility wait | CI / Release workflow (`publish-crates` job, poll step) | crates.io sparse index | The index (not the api/v1 DB record) is what `cargo`'s dependency resolver for the *next* crate in the loop actually reads; polling the index is the more faithful "is it safe to publish the next dependent" check than polling the api/v1 endpoint alone |
| Pre-publish consistency gate | Repo script (`scripts/check-release-consistency.sh`) + CI job | `cargo metadata` (manifest source), git tree (changelog files) | Must be runnable locally per D-07, so it cannot live only in the workflow; the script is the single source of truth, the CI job is a thin invocation |
| CI-conclusion resolution | Repo script or dedicated CI job | GitHub REST API (`actions/workflows/{file}/runs`) | Same "must be inspectable/runnable outside the workflow" argument as the gate; a `gh api` call is portable between local and CI execution |
| Per-crate outcome reporting | CI / Release workflow (`publish-crates` job) | `$GITHUB_STEP_SUMMARY` | Reporting is a property of the one job that performs the publish loop; no other tier has the per-iteration state needed to emit it |
| Runbook / yank policy | Documentation (`docs/src/appendix/release-recovery.md`) | crates.io account UI (yank execution) | The policy is documentation; the mechanism it documents (`cargo yank`) is executed by a human with crates.io ownership, never by CI |

## Standard Stack

### Core

| Tool | Version (verified) | Purpose | Why Standard |
|------|---------------------|---------|---------------|
| `gh` CLI / `gh api` | 2.97.0 (devcontainer, `[VERIFIED: gh --version]`) | Create-or-reuse GitHub release; resolve workflow-run conclusion for a SHA | Already authenticated by `GITHUB_TOKEN` on every runner; D-01 explicitly forbids introducing a new third-party marketplace action |
| `curl` | 7.88.1 (devcontainer, `[VERIFIED]`) | crates.io API / sparse-index polling | Already the tool used throughout `19-PUBLISH-EVIDENCE.md`'s registry checks; zero new dependency |
| `jq` | 1.6 (devcontainer, `[VERIFIED]`) | Parsing `gh api`/`curl` JSON responses in bash | Already used elsewhere in `release.yml` (`docker/metadata-action` JSON parsing) |
| `python3` | 3.11.2 (devcontainer, `[VERIFIED]`) + `tomllib` | Gate script's manifest/changelog parsing | Established local pattern: `check-changelogs.sh` and `check-workflow-triggers.sh` both use `python3 -c` heredocs with `tomllib`/`yaml` for structured parsing rather than regex-only bash |
| `cargo metadata --no-deps --format-version 1` | ships with pinned toolchain 1.97.1 `[VERIFIED: cargo --version]` | Enumerate the eleven publishable crates and their manifest versions | D-08 explicitly requires reading versions from `cargo metadata`, not a hardcoded list — this is also exactly how `19-PUBLISH-EVIDENCE.md`'s Task 1 reconciliation was performed |

### Supporting

| Tool | Version | Purpose | When to Use |
|------|---------|---------|-------------|
| `cargo publish -p <crate>` (existing) | 1.97.1 | Actual per-crate publish, unchanged from Phase 19 | Retained as-is; only its wrapping (`publish_one()`) changes |
| `cargo-release` | version pinned by `make release`'s `command -v cargo-release` check (no exact version pinned in repo — `[ASSUMED]`, verify locally with `cargo-release --version` before relying on `pre-release-replacements` behavior) | Lockstep version bump | Already the chosen tool (`release.toml`); D-09's changelog-finalization extension should NOT attempt a workspace-level `pre-release-replacements` — see `## State of the Art` |

### Alternatives Considered

| Instead of | Could Use | Tradeoff |
|------------|-----------|----------|
| Explicit `publish_one()` loop (recommended) | `cargo publish --workspace` (native, stable since Rust 1.90) | Native command is one line and gets dependency ordering for free, but (per D-06's own conditional test) is documented as non-atomic with no confirmed already-published-skip-on-rerun behavior, and gives no native per-crate outcome hook — fails D-06 conditions (b) and (c). Rejected as carrier; see `## State of the Art`. |
| `gh api` create-or-reuse (recommended) | A maintained community action (e.g. a fork/successor of `actions/create-release`) | D-01 explicitly rejects introducing new third-party action surface; Phase 21 rewrites this job's body/assets anyway |
| crates.io `api/v1/crates/<name>/<version>` polling (recommended for D-04 pre-check) | Sparse index (`index.crates.io`) polling only | The api/v1 endpoint is simpler for a single-version existence check (`200`/`404`); the sparse index is better for D-05's *index-visibility* wait since it is literally what cargo's own resolver reads for the next crate's dependency check — recommend using **both**, each for its specific purpose (see Pattern 3) |
| Gate as a repo script (recommended, D-07) | Gate logic embedded only in workflow YAML | D-07 explicitly requires local runnability (`make` target) — a script is the only way to satisfy that without duplicating logic in two places |

**Installation:** No new package installs. Every tool above is already present in this
devcontainer/CI image (`gh`, `curl`, `jq`, `python3`, `cargo`), or is the existing pinned
`cargo-release` binary the release process already requires.

**Version verification:** `gh --version`, `curl --version`, `jq --version`, `python3 --version`,
`cargo --version` were all run directly in this environment on 2026-08-28 and are recorded above
`[VERIFIED]`. `cargo-release`'s installed version was not checked in this session — the plan should
run `cargo-release --version` before depending on any version-specific `pre-release-replacements`
behavior.

## Package Legitimacy Audit

**Not applicable — this phase adds no new external package dependency.** No `Cargo.toml` /
`Cargo.lock` changes are in scope; no new GitHub Action is introduced (D-01 explicitly forbids one);
all CLI tools used (`gh`, `curl`, `jq`, `python3`, `cargo`) are pre-existing, already-vetted parts
of the devcontainer/CI image. The Package Legitimacy Gate protocol is skipped for this phase.

**Packages removed due to [SLOP] verdict:** none (none evaluated — none proposed).
**Packages flagged as suspicious [SUS]:** none.

## Architecture Patterns

### System Architecture Diagram

```
                    git push (tag v*.*.*)  OR  Actions "Re-run failed/all jobs"
                                  │
                                  ▼
                    ┌─────────────────────────┐
                    │   verify-tag-source      │  (unchanged — ancestor-of-main check)
                    └────────────┬─────────────┘
                                  │ needs
                    ┌─────────────┴─────────────┐
                    ▼                             ▼
        ┌───────────────────────┐     ┌─────────────────────────┐
        │        test            │     │      create-release      │
        │ cargo test --workspace │     │  gh api create-or-reuse  │◄── D-01/D-02: reuse on
        └───────────┬─────────────┘     │  (tag_name lookup first) │    422/already-exists,
                    │                    └────────────┬─────────────┘    preserve upload_url +
                    │                                  │ needs             version outputs
                    │                                  │
                    │                    ┌─────────────┴──────────────┐
                    │                    ▼                             ▼
                    │        ┌─────────────────────┐        (build-docker, build-binaries,
                    │        │  check-release-      │         sbom — unchanged, still
                    │        │  consistency (NEW)    │         needs: create-release only)
                    │        │  D-07/D-08:           │
                    │        │  - cargo metadata     │
                    │        │    vs tag version      │
                    │        │  - 11 changelogs vs    │
                    │        │    tag version          │
                    │        │  - ci.yml conclusion   │
                    │        │    for tagged SHA       │
                    │        │    (D-10, gh api        │
                    │        │    head_sha= query)     │
                    │        │  reports ALL mismatches │
                    │        └────────────┬────────────┘
                    │                      │ needs
                    └──────────┬───────────┘
                                ▼
                    ┌─────────────────────────────┐
                    │       publish-crates          │
                    │  OIDC auth (Phase 19, unchanged)│
                    │  for each of 11 crates:        │
                    │   1. GET crates.io/api/v1/     │◄── D-04: registry-state
                    │      crates/<name>/<ver>        │    pre-check, not grep
                    │      200 → already-at-version   │
                    │      404 → attempt publish      │
                    │   2. cargo publish -p <name>    │
                    │      fail → record 'failed'     │
                    │      succeed → poll index/api    │◄── D-05: bounded poll,
                    │      until visible → 'published- │    not sleep 20
                    │      now'                        │
                    │  emit per-crate outcome table    │◄── D-12: to $GITHUB_STEP_SUMMARY
                    │  0 published-now → job FAILS     │    + log; self-diagnosing msg
                    └─────────────────────────────────┘
```

### Recommended Project Structure

```
scripts/
├── check-release-consistency.sh   # NEW (D-07/D-08): tag/manifest/changelog/CI-conclusion gate
├── check-changelogs.sh            # EXISTING, unchanged — checks CHANGELOG.md *exists*, not
│                                   #   that it has a versioned section (the new gate's job)
├── finalize-crate-changelogs.sh   # NEW (D-09 fallback), OR extend `make release`'s existing
│                                   #   perl one-liner into a loop over crates/*/CHANGELOG.md
.github/workflows/
└── release.yml                    # MODIFIED: create-release (D-01/D-02), new gate job (D-07),
                                    #   publish-crates detection/wait/reporting (D-04/D-05/D-12)
docs/src/appendix/
├── release-recovery.md            # NEW (D-13): runbook + yank policy + yank register
├── release-automation.md          # MODIFIED: cross-link to release-recovery.md
└── release-checklist.md           # MODIFIED: reference the gate and runbook
tests/scripts/
└── check-release-consistency_test.sh   # NEW, following the check-workflow-triggers_test.sh
                                          #   fixture-lifecycle pattern (mktemp -d + trap cleanup)
.planning/phases/20-.../
└── 20-RECOVERY-EVIDENCE.md        # NEW (D-14): rehearsal evidence, 19-PUBLISH-EVIDENCE.md style
```

### Pattern 1: Create-or-reuse GitHub release via `gh api`

**What:** Replace `actions/create-release@v1` with two `gh api` calls: attempt `GET
releases/tags/{tag}`; if 404, `POST releases`; either way, surface `upload_url` and the version
as step outputs.
**When to use:** The `create-release` job, unconditionally (D-01).
**Example:**
```bash
# Source: GitHub REST API docs (releases) — verified endpoint shape, 2026-08-28
set -euo pipefail
TAG="${VERSION}"
if RELEASE_JSON=$(gh api "repos/${GITHUB_REPOSITORY}/releases/tags/${TAG}" 2>/dev/null); then
  echo "::notice::Release for ${TAG} already exists — reusing (idempotent re-run)."
else
  RELEASE_JSON=$(gh api "repos/${GITHUB_REPOSITORY}/releases" \
    -f tag_name="${TAG}" \
    -f name="Release ${TAG}" \
    -f body="${CHANGELOG_BODY}" \
    -F draft=false \
    -F "prerelease=${IS_PRERELEASE}")
fi
UPLOAD_URL=$(jq -r '.upload_url' <<<"${RELEASE_JSON}")
echo "upload_url=${UPLOAD_URL}" >> "$GITHUB_OUTPUT"
echo "version=${TAG}" >> "$GITHUB_OUTPUT"
```
Note: `gh api` returns non-zero on a 404, which is why the reuse path is the `if` branch's
success case for `gh api ... 2>/dev/null` failing — the snippet above inverts that; a real
implementation should check the actual HTTP status (`gh api --include` or `-i`) rather than relying
on exit code alone, since a 404 and a network failure both produce non-zero.

### Pattern 2: Pre-publish consistency gate — collect-then-report

**What:** A script that gathers every mismatch across tag, 11 manifests, 11 changelogs (root +
10 crate) and the CI-conclusion check, then reports the full list and exits non-zero if any
mismatch exists — never fails fast on the first (D-08).
**When to use:** `scripts/check-release-consistency.sh`, invoked by a new `release.yml` job that
`publish-crates` `needs`.
**Example:**
```python
# Source: modeled on this repo's own scripts/check-workflow-triggers.sh "accumulate into a
# shared failure list, never stop at the first" house style — verified 2026-08-28
failures = []

tag_version = "0.9.0"  # parsed from the tag ref

manifest_versions = {}  # name -> version, from `cargo metadata --no-deps --format-version 1`
for name, version in manifest_versions.items():
    if version != tag_version:
        failures.append(f"manifest {name} is at {version}, tag says {tag_version}")

changelogs = ["CHANGELOG.md"] + [f"crates/{c}/CHANGELOG.md" for c in publishable_crate_dirs]
for path in changelogs:
    if f"## [{tag_version}]" not in read(path):
        failures.append(f"{path} has no '## [{tag_version}]' section")

if ci_conclusion != "success":
    failures.append(f"ci.yml conclusion for tagged SHA is '{ci_conclusion}', not 'success'")

if failures:
    for f in failures:
        print(f"MISMATCH: {f}")
    raise SystemExit(1)
```

### Pattern 3: Registry-state pre-check + bounded index-visibility poll

**What:** Before `cargo publish -p <crate>`, `curl` the crate/version endpoint; if `200`, record
`already-at-this-version` and skip. After a successful publish, poll (not sleep) until the version
is visible, bounded by a timeout, before moving to the next dependent crate.
**When to use:** Inside `publish_one()` in the `publish-crates` job (D-04/D-05).
**Example:**
```bash
# Source: pattern verified against this repo's own 19-PUBLISH-EVIDENCE.md registry-check calls
# (User-Agent requirement) and the official crates.io sparse-index format (index.crates.io)
UA='User-Agent: paladin-release-check (github.com/DF3NDR/paladin-dev-env)'

crate_version_exists() {
  local name="$1" version="$2"
  local status
  status=$(curl -s -o /dev/null -w '%{http_code}' -H "$UA" \
    "https://crates.io/api/v1/crates/${name}/${version}")
  [ "$status" = "200" ]
}

wait_for_index_visibility() {
  local name="$1" version="$2" timeout_s="${3:-120}" waited=0
  while [ "$waited" -lt "$timeout_s" ]; do
    if crate_version_exists "$name" "$version"; then
      echo "::notice::${name}@${version} visible after ${waited}s."
      return 0
    fi
    sleep 5
    waited=$((waited + 5))
  done
  echo "::error::${name}@${version} not visible after ${timeout_s}s — index-wait timed out."
  return 1
}
```

### Pattern 4: Per-crate outcome table with an honest zero-published-now failure

**What:** Accumulate an associative array of outcomes across the loop; write a Markdown table to
`$GITHUB_STEP_SUMMARY`; fail the job explicitly if no crate reached `published-now` (D-12).
**When to use:** End of the `publish-crates` job's publish step.
**Example:**
```bash
# Source: modeled on this phase's D-12 requirement text and the Phase 12/18/19 "no green on
# measuring/doing nothing" house convention (security.instructions.md honesty posture)
declare -A OUTCOME
# OUTCOME[paladin-ai-core]="published-now" | "already-at-this-version" | "skipped" | "failed"

{
  echo "## Publish outcome — ${VERSION}"
  echo ""
  echo "| Crate | Outcome |"
  echo "|---|---|"
  PUBLISHED_NOW_COUNT=0
  for c in "${CRATES[@]}"; do
    echo "| ${c} | ${OUTCOME[$c]} |"
    [ "${OUTCOME[$c]}" = "published-now" ] && PUBLISHED_NOW_COUNT=$((PUBLISHED_NOW_COUNT + 1))
  done
} | tee -a "$GITHUB_STEP_SUMMARY"

if [ "$PUBLISHED_NOW_COUNT" -eq 0 ]; then
  echo "::error::all crates already at ${VERSION} — this tag appears fully published; if this \
was a recovery re-run, nothing was left to recover. See docs/src/appendix/release-recovery.md."
  exit 1
fi
```

### Anti-Patterns to Avoid
- **String-matching `cargo publish`'s stderr/stdout for recovery logic:** the exact defect D-04
  exists to remove — crates.io's error wording is not a stable API contract.
- **A fixed `sleep N` anywhere in the publish path:** replaced everywhere by a bounded poll
  (D-05); a longer fixed sleep is not a fix, it is the same guess with a bigger number.
- **Treating `cargo publish --workspace`'s single exit code as the per-crate outcome signal:** it
  cannot distinguish "3 already published, 8 newly published" from "8 newly published, 3 failed" —
  this is exactly why D-06 condition (c) fails for the native command.
- **Embedding the gate logic only in workflow YAML with no standalone script:** violates D-07's
  local-runnability requirement and duplicates logic the moment someone wants a `make` target.

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|--------------|-----|
| GitHub release create-or-reuse | A custom idempotency-tracking file/artifact in the repo | `gh api` querying the release object directly by tag | GitHub's release-by-tag lookup is already the authoritative idempotency check; a local tracking file can drift from server truth across re-runs |
| YAML/TOML manifest parsing in bash | Regex over `Cargo.toml` text | `cargo metadata --format-version 1` (JSON) piped to `jq`/`python3 -c json.load` | `cargo metadata` is Cargo's own structured, versioned output format — the same source `19-PUBLISH-EVIDENCE.md`'s crate-set reconciliation already used, and immune to manifest formatting variance a regex would break on |
| Cross-run workflow state ("did the last run get here") | A custom marker file committed to a branch, or a cache artifact | Registry state itself (crates.io) as the sole source of truth (D-04) | Any local state artifact is itself something that can desync from the registry after a partial failure — the registry cannot lie about what it actually published |
| GitHub Actions retry/backoff for API polling | A hand-rolled exponential-backoff loop with jitter | A simple bounded linear poll (fixed interval, fixed timeout) — crates.io index propagation is typically fast (seconds), and this loop runs at most 11 times per release | Simplicity matches the actual problem shape; a backoff library or complex retry policy is disproportionate machinery for an operation this repo's Phase 19 evidence shows completing in tens of seconds per crate |

**Key insight:** every "don't hand-roll" item above reduces to the same principle threaded through
D-04/D-05/D-08: **query the authoritative external source of truth (registry, GitHub API, `cargo
metadata`) instead of maintaining a parallel local approximation of it.** The pre-Phase-20 defects
this phase closes are all instances of a local approximation (error-message pattern, fixed sleep,
absent cross-check) standing in for a query that was always available.

## Common Pitfalls

### Pitfall 1: `gh api`'s exit code conflates "not found" with "network/auth failure"
**What goes wrong:** A create-or-reuse check written as `if gh api releases/tags/$TAG; then reuse;
else create; fi` will also "create" (and then fail with a real 422) on a transient network error,
because both a 404 and a curl-level failure produce non-zero exit.
**Why it happens:** `gh api`'s plain exit code doesn't distinguish HTTP status classes by default.
**How to avoid:** Use `gh api --include` (or check `gh api -i ... | head -1` for the status line)
and branch explicitly on `404` vs other non-2xx codes; treat anything else as a hard failure rather
than silently proceeding to "create."
**Warning signs:** A re-run that intermittently attempts to create a release that already exists,
producing the exact 422 this phase exists to eliminate, on an otherwise-healthy re-run.

### Pitfall 2: Polling the crates.io `api/v1` endpoint for index-visibility instead of the sparse index
**What goes wrong:** The `api/v1/crates/<name>/<version>` endpoint reflects the database record
(created essentially at upload time), which can be visible *before* the crate is actually
resolvable by `cargo` for a dependent crate's build — polling this endpoint alone for D-05's
"visible" wait can under-wait relative to what the next crate's `cargo publish` verification build
actually needs.
**Why it happens:** The DB write and the index write are two different systems with historically
observed propagation lag between them (this is why `cargo`'s own publish command, per the Cargo
book, blocks on index visibility rather than the API record — see PR #11062, "Block until it is in
index").
**How to avoid:** For the D-05 wait specifically, poll `https://index.crates.io/<2-char-prefix
path>/<name>` (the sparse index) and check the newline-delimited JSON for the target `"vers"` value
with `"yanked":false`, not just the api/v1 endpoint. Use the api/v1 endpoint for D-04's simpler
existence pre-check, where DB-record visibility is exactly the right question ("did we already
publish this").
**Warning signs:** A dependent crate's `cargo publish` fails with a dependency-resolution error
immediately after the dependency's api/v1 check reported success.

### Pitfall 3: Judging release health by the workflow's overall conclusion
**What goes wrong:** `19-PUBLISH-EVIDENCE.md` records this exactly: the proof-release run
(33089177606) has overall `conclusion: failure`, sourced entirely from the four unrelated,
long-standing Build Binaries matrix failures — every job on the actual publish path (`verify-tag-
source`, `test`, `create-release`, `publish-crates`) succeeded. A CI-conclusion check for PUBOPS-02
that resolves `ci.yml`'s conclusion this same naive way for the *tagged commit* on `main` would
have the identical trap: `ci.yml` is an 18-job suite, and one flaky/unrelated job failing there
must not be conflated with "the tagged commit didn't pass its real checks," or the gate becomes
unusably strict and gets bypassed by operators under pressure.
**Why it happens:** GitHub's run-level `conclusion` field is a single boolean-ish rollup over every
job, with no built-in notion of "the jobs that matter for this decision."
**How to avoid:** Decide explicitly (and document in the gate) whether PUBOPS-02 means "the whole
`ci.yml` run succeeded" or "the specific jobs `release.yml` cares about succeeded" — this repo's
own Build Binaries flakiness (both in `release.yml` and structurally similar to jobs in `ci.yml`)
makes the all-or-nothing interpretation a real operational risk, not a hypothetical one. D-10's
text says "a recorded successful conclusion" for the run — the plan should decide and record which
granularity satisfies that, given this repo's own history of unrelated job flakiness.
**Warning signs:** A gate that blocks every release the week a known-flaky, non-publish-critical CI
job (e.g. Build Binaries) has a bad run.

### Pitfall 4: Multiple `ci.yml` runs for the same SHA (retries, re-runs, cron)
**What goes wrong:** The `actions/workflows/{file}/runs?head_sha=<sha>` query can return more than
one run for the same SHA (a manual re-run creates a new attempt but the same run object in some
cases, a re-triggered push in others). Naively taking `.workflow_runs[0]` without sorting can pick
a stale or in-progress run.
**Why it happens:** GitHub does not guarantee at most one workflow run per SHA per workflow file.
**How to avoid:** Sort by `created_at` descending (or use the `status=completed` filter combined
with taking the most recent), and explicitly decide whether "any successful run ever" or "the most
recent run must be successful" satisfies D-10 — recommend the latter (most recent completed run),
since an old success followed by a newer failure on the same SHA (e.g. after a force-push
re-triggered the same commit) should not be read as still-passing.
**Warning signs:** A gate that passes despite the most recent `ci.yml` run on that SHA having
failed, because an older run on the same SHA happened to succeed.

### Pitfall 5: cargo-release `pre-release-replacements` at workspace scope
**What goes wrong:** Attempting to configure the ten per-crate `CHANGELOG.md` finalizations via a
single workspace-level `pre-release-replacements` entry in the root `release.toml` will either be
silently ignored or applied once (to the wrong file) rather than once-per-crate, per the
`pre-release-replacements` package-scoping documented in cargo-release's own open issues.
**Why it happens:** `pre-release-replacements` is documented as package-level configuration that
workspace config is inherited *into*, not something expressible once at the workspace root for a
list of ten distinct target files.
**How to avoid:** Per D-09's own stated fallback, use a small finalize script (extending the
existing `make release` perl one-liner into a loop over `crates/*/CHANGELOG.md`) rather than
fighting cargo-release's per-package replacement model. This is directly consistent with this
repo's own `release.toml` comment explaining why the *root* changelog itself is finalized by
`make release`'s perl step rather than `pre-release-replacements`, for the identical
one-heading-would-duplicate-per-crate reason.
**Warning signs:** Ten `pre-release-replacements` blocks added to `release.toml`'s top level with
no effect observed on a dry-run of `cargo release version`.

## Code Examples

### 1. Enumerate publishable crates and versions from `cargo metadata` (D-08)
```bash
# Source: modeled directly on 19-PUBLISH-EVIDENCE.md's Task 1 crate-set reconciliation method
cargo metadata --no-deps --format-version 1 | jq -r '
  .packages[]
  | select(.publish == null)          # publish == null means publishable (Cargo default)
  | "\(.name) \(.version)"'
```

### 2. Resolve `ci.yml`'s conclusion for the tagged SHA (D-10/PUBOPS-02)
```bash
# Source: GitHub REST API docs — "List workflow runs for a workflow", head_sha query param
# verified 2026-08-28
SHA="<tagged commit sha>"
CONCLUSION=$(gh api "repos/${GITHUB_REPOSITORY}/actions/workflows/ci.yml/runs" \
  -f head_sha="${SHA}" -f status=completed --paginate \
  --jq '[.workflow_runs[]] | sort_by(.created_at) | last | .conclusion // "none"')

if [ "$CONCLUSION" != "success" ]; then
  echo "::error::No successful ci.yml run found for commit ${SHA}. Re-run CI on main at that \
SHA, or fix and re-tag. See docs/src/appendix/release-recovery.md."
  exit 1
fi
```

### 3. Create-or-reuse GitHub release preserving `upload_url`/`version` outputs (D-01/D-02)
See `## Architecture Patterns` Pattern 1 above.

### 4. crates.io sparse-index visibility check (D-05, complementing Pattern 3's api/v1 pre-check)
```bash
# Source: official crates.io sparse index format (index.crates.io/<2-char>/<name> or
# index.crates.io/<2-char>/<2-char>/<name> for longer names — mirrors Cargo's own index
# path convention), newline-delimited JSON, one line per published version
index_path_for() {
  local name="$1" len=${#1}
  case "$len" in
    1) echo "1/${name}" ;;
    2) echo "2/${name}" ;;
    3) echo "3/${name:0:1}/${name}" ;;
    *) echo "${name:0:2}/${name:2:2}/${name}" ;;
  esac
}
version_in_index() {
  local name="$1" version="$2" path
  path=$(index_path_for "$name")
  curl -sf -H "$UA" "https://index.crates.io/${path}" \
    | jq -e --arg v "$version" 'select(.vers == $v and .yanked == false)' >/dev/null 2>&1
}
```

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|-------------------|---------------|--------|
| `actions/create-release@v1` + `actions/upload-release-asset@v1` | `gh` CLI / `gh api` direct calls | Both archived by GitHub, upstream, since 2021 `[CITED: repeated in PUB requirements text and this repo's own release.yml comments]` | No upsert behavior on create-release is the direct cause of PUBOPS-03's blocking defect; this phase replaces create-release only, Phase 21 replaces upload-release-asset |
| Manual per-crate `cargo publish -p <crate>` loop, hand-rolled ordering | Native `cargo publish --workspace` / `cargo publish -p a -p b` (multi-package) | Stabilized Rust 1.90.0, 2025-09-18 `[VERIFIED: blog.rust-lang.org/2025/09/18/Rust-1.90.0/, WebSearch confirmed]` | Available given the pinned 1.97.1 toolchain, but **not adopted this phase** — see D-06 verdict in Summary; cargo's own workspace-publish is documented non-atomic with no confirmed resumable-skip behavior on re-run |
| `cargo publish --workspace --dry-run` colliding with already-published versions | Local registry given precedence over remote in dry-run mode | Fixed by rust-lang/cargo PR #14847, shipped Rust 1.85.0 `[VERIFIED: GitHub PR + WebSearch, 2026-08-28]` | This fix is dry-run-specific; it does not establish or imply equivalent behavior for a *real* (non-dry-run) re-run against already-published crates — do not extrapolate dry-run tolerance to real-publish tolerance |
| Error-message grep for "already published" | Registry-state pre-check (D-04) | This phase | Removes dependency on crates.io's error wording remaining stable |
| Fixed `sleep 20` | Bounded registry/index poll (D-05) | This phase | Removes both false-negative (index not yet visible after exactly 20s under load) and false-positive-cost (wasting 20s when the index updates in 2s) failure modes |

**Deprecated/outdated:**
- `actions/create-release@v1`, `actions/upload-release-asset@v1`: both archived, no longer
  receiving updates; `actions/upload-release-asset@v1` stays in scope for Phase 21, not this phase.
- Third-party workspace-publish wrapper crates (`cargo-workspaces`, `cargo-publish-all`,
  `shipper`) were surveyed as alternatives to the hand-rolled loop; none is adopted — each would
  introduce a new external dependency into the release pipeline for a problem the existing
  `publish_one()` loop (with the D-04/D-05 fixes) already solves without one, and D-01's "minimize
  action/tooling surface" posture argues against adding one here.

## Assumptions Log

| # | Claim | Section | Risk if Wrong |
|---|-------|---------|----------------|
| A1 | crates.io enforces roughly 1 request/second (or equivalent) rate-limit etiquette on the `api/v1` endpoint for unauthenticated polling, and prefers the sparse index for bulk/repeated checks | Architecture Patterns Pattern 3, Common Pitfalls Pitfall 2 | If the actual limit is stricter, an 11-crate loop with pre-check + poll could get rate-limited (429) mid-release, turning a routine publish into a new failure mode the gate itself introduces. The plan should add explicit backoff-on-429 handling to the poll/pre-check functions regardless of the exact limit, as a defensive measure. |
| A2 | `gh api repos/{owner}/{repo}/actions/workflows/ci.yml/runs -f head_sha=<sha>` accepts the workflow **filename** (not just numeric ID) in the path, as GitHub's docs generally allow for workflow-scoped endpoints | Code Examples §2 | If the endpoint requires a numeric workflow ID for this repository, the gate script needs an extra `gh api .../actions/workflows` lookup step first to resolve `ci.yml` → ID. Low risk (this is documented GitHub API behavior for `workflow_id` parameters generally accepting `owner/repo/workflow.yml` as a string), but not independently exercised against this specific repository in this research session. |
| A3 | The `crates-io` GitHub Environment's OIDC subject-claim / deployment-branch-policy (restricted to `v*.*.*` tag refs, Phase 19 D-06/D-07) is preserved unchanged by GitHub's "Re-run failed jobs" / "Re-run all jobs" mechanics, since GitHub's docs state a re-run reuses the original `GITHUB_SHA`/`GITHUB_REF` | Summary, D-15 support | If GitHub's re-run implementation evaluates the environment's deployment branch policy against something other than the *original* triggering ref (e.g. re-evaluates against the re-run actor's current context), a re-run recovery could unexpectedly fail the OIDC exchange — this is exactly Phase 19's untested assumption A1 territory but for re-runs specifically rather than `workflow_dispatch`; the D-14 rehearsal is the first real test of this and should explicitly capture whether the environment gate passes on the re-run. |
| A4 | `cargo-release`'s currently-installed version in this repo's CI/dev environment does not materially change the `pre-release-replacements` package-scoping behavior described in `## Common Pitfalls` Pitfall 5 | Common Pitfalls Pitfall 5, Standard Stack Supporting | Low risk — this scoping behavior has been stable across cargo-release's issue history — but the plan should run `cargo-release --version` and spot-check the installed version's changelog before finalizing the fallback-script decision, since D-09 explicitly asks the researcher/planner to verify rather than assume. |

## Open Questions

1. **Does "CI conclusion for the tagged SHA" (D-10) mean the whole `ci.yml` run, or a named
   subset of its 18 jobs?**
   - What we know: `ci.yml`'s overall `conclusion` can be `failure` for reasons unrelated to
     release-readiness (Pitfall 3, directly observed in `19-PUBLISH-EVIDENCE.md`'s own proof run).
   - What's unclear: D-10's text says "a recorded successful conclusion" without specifying
     granularity, and this phase is explicitly barred from adding a tag trigger to `ci.yml` or
     duplicating its 18 jobs inside `release.yml` (D-11).
   - Recommendation: the plan should decide explicitly — either require the *whole run* to be
     `success` (simplest, but brittle against known-flaky non-critical jobs like Build Binaries in
     `release.yml` itself, which is a different workflow but demonstrates the flakiness class
     exists in this repo's CI surface) or enumerate the specific job names (`lint`, `test`, etc.)
     that must each be `success` within the run. Given D-11's explicit no-duplication constraint
     and the honesty posture this phase must uphold, **whole-run success is the simpler, more
     honest default** unless a specific known-flaky-and-irrelevant job in `ci.yml` itself (not
     `release.yml`) is identified — this was not confirmed in this research session and should be
     checked against `ci.yml`'s actual job list before the plan commits to a granularity.

2. **Should the gate job (`check-release-consistency`) also gate `create-release`, or only
   `publish-crates`?**
   - What we know: D-07 leaves this to planner discretion explicitly ("whether `create-release`
     also needs it is planner discretion").
   - What's unclear: gating `create-release` on the same check would mean a mismatched
     tag/manifest/changelog state never even gets a GitHub release object created — arguably
     cleaner — versus the current design where `create-release`, `build-docker`, `build-binaries`,
     and `sbom` are deliberately *not* gated on `test` (WR-05's documented asymmetry, preserved by
     this phase).
   - Recommendation: keep the gate scoped to `publish-crates` only, consistent with WR-05's
     existing documented asymmetry (release/docker/binaries/SBOM proceed independent of the
     crates.io publish path) — extending the gate to `create-release` would be a scope expansion
     beyond what PUBOPS-01's criterion text ("no crate is published until…") actually requires.

3. **Exact bounded-timeout value for the D-05 index-visibility poll.**
   - What we know: `19-PUBLISH-EVIDENCE.md` observed real publish-to-visible timings around
     25-35 seconds per crate in the committed order, and the whole 11-crate OIDC proof run
     completed its publish loop in ~7 minutes end-to-end (~40s/crate average including the actual
     publish call, not just wait).
   - What's unclear: no documented crates.io SLA for index-propagation worst-case latency was
     found in this research session.
   - Recommendation: use a per-crate timeout in the 2-3 minute range (comfortably above the
     observed ~30s typical case) with a 5-10 second poll interval, and treat a timeout as a
     `failed` outcome for that crate (D-12) rather than a hard job abort, so the loop's honest
     per-crate reporting still applies.

## Environment Availability

| Dependency | Required By | Available | Version | Fallback |
|------------|--------------|-----------|---------|----------|
| `gh` CLI | Create-or-reuse release, CI-conclusion resolution | ✓ | 2.97.0 | `gh` is also present on every GitHub-hosted runner by default; no fallback needed |
| `curl` | crates.io API/index polling | ✓ | 7.88.1 | — |
| `jq` | JSON parsing throughout | ✓ | 1.6 | — |
| `python3` (+ `tomllib`) | Gate script manifest parsing | ✓ | 3.11.2 | — |
| `cargo` (pinned toolchain) | `cargo metadata`, `cargo publish` | ✓ | 1.97.1 (pins ≥1.90 for workspace-publish, not adopted) | — |
| `cargo-release` | Version bump / changelog finalize tooling this phase extends | not independently verified this session | unknown — verify via `cargo-release --version` before relying on any version-specific behavior | `make release`'s existing `command -v cargo-release` preflight check already fails loudly if absent |
| crates.io API / sparse index (network) | D-04/D-05 registry checks | requires outbound network access from the CI runner (standard GitHub-hosted runner has this; devcontainer network access not exercised in this research session) | — | none — this is a hard dependency of PUBOPS-03; no offline fallback is meaningful for "is this already published" |
| GitHub Actions `head_sha`-filtered runs API | PUBOPS-02 | requires the tagged SHA to actually have triggered a `ci.yml` run on some branch (true today: `ci.yml` triggers on `push: branches: ['**']`, so every commit that reached `main` has a run) | — | If a commit somehow reached `main` without a recorded `ci.yml` run (e.g. an old commit predating the workflow, or an admin bypass), the gate correctly refuses to publish — this is the intended fail-closed behavior, not a gap to work around |

**Missing dependencies with no fallback:** none blocking — every tool is already present.

**Missing dependencies with fallback:** none applicable this phase.

## Validation Architecture

### Test Framework

| Property | Value |
|----------|-------|
| Framework | Bash regression-harness pattern already established in `tests/scripts/` (no third-party test framework — hand-rolled `mktemp -d` + `trap cleanup EXIT` + assertion counters, e.g. `tests/scripts/check-workflow-triggers_test.sh`) |
| Config file | none — each `*_test.sh` is self-contained and invoked directly |
| Quick run command | `./tests/scripts/check-release-consistency_test.sh` (new file this phase adds) |
| Full suite command | `make test-shell-guards` (existing target; this phase's new test file should be added to its invocation list alongside `check-workflow-triggers_test.sh` etc.) |

Rust-side validation (`cargo test`) is not the primary mechanism for this phase's deliverables,
since the changes are shell scripts and a GitHub Actions workflow, not library code. Where this
phase's changes are exercised, it is via (a) the new shell-guard regression test file, and (b) the
D-14 rehearsal itself — a real, recorded execution against a throwaway prerelease tag, which is the
phase's actual integration test and cannot be simulated by a unit test.

### Phase Requirements → Test Map

| Req ID | Behavior | Test Type | Automated Command | File Exists? |
|--------|----------|-----------|---------------------|--------------|
| PUBOPS-01 | Gate reports every mismatch, not the first, across tag/manifests/changelogs | unit (fixture-driven) | `./tests/scripts/check-release-consistency_test.sh` | ❌ Wave 0 |
| PUBOPS-02 | CI-conclusion resolution refuses publish without a recorded success for the tagged SHA | unit (fixture-driven, mocked `gh api` response) | same file, additional assertions | ❌ Wave 0 |
| PUBOPS-03 | Re-run reaches publish step; already-published detected from registry state | integration / manual-only (requires live crates.io + live GitHub Actions re-run) | D-14 rehearsal, recorded in `20-RECOVERY-EVIDENCE.md` | ❌ Wave 0 (the rehearsal itself is the test artifact) |
| PUBOPS-04 | Per-crate outcome table; zero-published-now fails the job | unit (fixture-driven bash function test, e.g. sourcing `publish_one`/outcome-table logic with a stubbed `curl`/`cargo publish`) | new or extended shell-guard test | ❌ Wave 0 |
| PUBOPS-05 | Runbook content complete; rehearsal exercised | manual-only (documentation review) + D-14 rehearsal | N/A (doc review) + `20-RECOVERY-EVIDENCE.md` | ❌ Wave 0 |

### Sampling Rate
- **Per task commit:** `./tests/scripts/check-release-consistency_test.sh` (or whichever new test
  file the plan creates) — fast, offline, no network.
- **Per wave merge:** `make test-shell-guards` (runs all shell-guard regression tests together).
- **Phase gate:** the D-14 rehearsal (a real workflow run against a throwaway prerelease tag) is
  the phase's actual gate for PUBOPS-03/04/05 — no unit test can substitute for it, per D-14's own
  explicit prohibition on `cargo publish --dry-run` as evidence.

### Wave 0 Gaps
- [ ] `tests/scripts/check-release-consistency_test.sh` — covers PUBOPS-01, PUBOPS-02 (fixture-
      driven: fake `cargo metadata` JSON, fake changelog files under a `mktemp -d` scratch dir,
      stubbed `gh api` response for the CI-conclusion check)
- [ ] A stubbable seam for `publish_one()`'s outcome-table logic (extract into a sourceable
      function so it can be unit-tested with stubbed `curl`/`cargo publish` outputs, covering
      PUBOPS-04's zero-published-now failure path without touching the real registry)
- [ ] `.planning/phases/20-.../20-RECOVERY-EVIDENCE.md` — the D-14 rehearsal record itself,
      created during phase execution, not before

## Security Domain

### Applicable ASVS Categories

| ASVS Category | Applies | Standard Control |
|----------------|---------|--------------------|
| V2 Authentication | no | This phase does not touch user-facing authentication; it touches CI-to-registry authentication (OIDC), already covered by Phase 19 |
| V3 Session Management | no | N/A |
| V4 Access Control | yes | The `crates-io` environment's deployment-branch-policy (`v*.*.*` tags only, no reviewer gate — Phase 19 D-08) is the access-control boundary this phase must not weaken; the new gate job and create-release rewrite must not introduce a path that mints an OIDC token or performs a publish outside that environment's job |
| V5 Input Validation | yes | Tag-derived version strings, SHA values, and crate names flowing into `gh api`/`curl` calls must be validated/quoted defensively — see Known Threat Patterns below |
| V6 Cryptography | no | No new cryptographic material is introduced by this phase (OIDC token minting is unchanged, owned by Phase 19's `rust-lang/crates-io-auth-action@v1`) |

### Known Threat Patterns for this stack

| Pattern | STRIDE | Standard Mitigation |
|---------|--------|-----------------------|
| Shell injection via tainted `${{ ... }}` expression interpolated directly into a `run:` block (e.g. a crafted tag name, or a `head_sha` value) | Tampering | Indirect through `env:` blocks rather than direct `${{ }}` interpolation inside `run:` — this repo already applies this pattern deliberately (see `release.yml`'s `RELEASE_VERSION: ${{ needs.create-release.outputs.version }}` env indirection, commented "CR-01"); the new gate script and `gh api` calls this phase adds must follow the same convention for any tag-derived or SHA-derived value |
| `gh api` calls following an unexpected redirect to an attacker-influenced host (credential-header leak) | Information Disclosure | `security.instructions.md`'s stated control ("HTTP clients sending a credential header do not follow redirects") applies directly to any raw `curl` call this phase adds against crates.io — verify `curl` calls either omit `-L` (no redirect following, the default) or explicitly pin `--proto '=https' --location-trusted` only if redirects are genuinely required; the `gh api` calls use `GITHUB_TOKEN`/`gh`'s own credential handling, not a raw header, and are lower risk |
| A gate script or publish loop over-trusting a crates.io response body without validating expected shape before use | Tampering / Denial of Service | `jq -e` (exit non-zero on missing/null field) rather than blind field access; a malformed or unexpected JSON shape from a registry response should fail the check loudly, not silently pass or crash the workflow uninformatively |
| Environment deployment-policy bypass via re-run mechanics (re-run somehow evaluated against a different ref than the original tag) | Elevation of Privilege | This is exactly Assumption A3 above — the D-14 rehearsal is the concrete verification step; no code-level mitigation exists beyond confirming GitHub's documented re-run-preserves-ref behavior holds for this specific environment-gated job |

## Sources

### Primary (HIGH confidence)
- This repository, read directly, 2026-08-28: `.github/workflows/release.yml`,
  `.github/workflows/ci.yml` (publish-dry-run job), `release.toml`, `rust-toolchain.toml`,
  `Makefile` (release/release-check/publish-dry-run/check-gates/test-shell-guards targets),
  `Cargo.toml` (workspace root, `include` allowlist), `CHANGELOG.md` + `crates/*/CHANGELOG.md`,
  `scripts/check-workflow-triggers.sh`, `scripts/check-changelogs.sh`,
  `tests/scripts/check-workflow-triggers_test.sh`, `docs/src/appendix/release-automation.md`,
  `docs/src/appendix/release-checklist.md`, `SECURITY-EXCEPTIONS.md`
- `.planning/phases/19-.../19-PUBLISH-EVIDENCE.md` — eleven-crate set/order, User-Agent
  requirement, OIDC proof timings, known Build Binaries flakiness, create-release non-reuse defect
  observed directly in production
- GitHub REST API docs, `docs.github.com/en/rest/actions/workflow-runs` and
  `docs.github.com/en/rest/releases/releases` — `head_sha` query parameter, releases-by-tag
  endpoint shape, fetched 2026-08-28
- `blog.rust-lang.org/2025/09/18/Rust-1.90.0/` — `cargo publish --workspace` stabilization date
  and "non-atomic" characterization
- Direct environment probes (`gh --version`, `curl --version`, `jq --version`, `python3 --version`,
  `cargo --version`, `gh auth status`) run in this session, 2026-08-28

### Secondary (MEDIUM confidence)
- `doc.rust-lang.org/cargo/commands/cargo-publish.html` — confirms `--workspace` flag exists but
  does not document partial-failure/resume semantics
- GitHub issues: `rust-lang/cargo#14347` (closed, not planned — feature request predates
  stabilization), `rust-lang/cargo#14789`/PR `#14847` (dry-run version-collision fix, 1.85.0),
  `crate-ci/cargo-release#303` (open `--skip-published` feature request, evidence cargo-release
  itself lacks this)
- `tweag.io/blog/2025-07-10-cargo-package-workspace/` — implementation-level explanation of
  registry-overlay approach, confirms nightly-first rollout timeline

### Tertiary (LOW confidence)
- crates.io rate-limit etiquette (Assumption A1) — general WebSearch results referenced 403-without-
  User-Agent behavior (consistent with this repo's own verified experience) but did not surface an
  authoritative numeric rate-limit policy in this session; treat as unverified until confirmed
  against crates.io's own current policy page or observed empirically during the D-14 rehearsal
- `gh api` workflow-filename-vs-ID path acceptance (Assumption A2) — based on general GitHub API
  documentation conventions, not independently exercised against this specific repository's
  `ci.yml` in this research session

## Metadata

**Confidence breakdown:**
- Standard stack / tooling availability: HIGH — every tool version was directly probed in this
  environment, not assumed
- `cargo publish --workspace` adoption verdict (D-06): MEDIUM — the stabilization facts are
  HIGH-confidence (official Rust blog, PR references), but the specific "does a re-run skip
  already-published crates" behavior was not found stated authoritatively anywhere; the verdict
  rests on the absence of documented resume behavior plus strong circumstantial evidence
  (cargo-release's own open feature request, three independent wrapper tools solving exactly this
  gap) rather than a single definitive source
- GitHub Actions/API mechanics (create-or-reuse, head_sha filtering, re-run ref preservation):
  HIGH — official GitHub docs, directly fetched
- crates.io registry-polling mechanics: HIGH for the endpoint shapes and User-Agent requirement
  (verified against this repo's own prior production evidence); MEDIUM for rate-limit specifics
  (Assumption A1)
- Architecture patterns / don't-hand-roll / pitfalls: HIGH — derived directly from this
  repository's existing code and Phase 19's recorded production incidents (413 payload error, four
  Build Binaries failures, create-release non-reuse failure), not speculative

**Research date:** 2026-08-28
**Valid until:** 30 days for the GitHub Actions/API mechanics (stable surface); re-verify the
`cargo publish --workspace` verdict if a future cargo release changelog explicitly documents
resume/skip-already-published semantics, since that would change the D-06 recommendation
