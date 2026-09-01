# Phase 19: crates.io Trusted Publishing — Replace the Long-Lived Registry Token - Research

**Researched:** 2026-08-26
**Domain:** GitHub Actions OIDC → crates.io Trusted Publishing (CI/CD credential migration)
**Confidence:** MEDIUM-HIGH (mechanism is well-documented by rust-lang and GitHub; a few operational edges — first-publish support, multi-crate token lifetime, workflow_dispatch eligibility — are confirmed by official sources but not exercised in this repository yet, which is exactly what PUB-03's proof event is for)

<user_constraints>
## User Constraints (from CONTEXT.md)

### Locked Decisions

**Crate set and the paladin-herald gap (PUB-01)**
- **D-01:** The publishable set is enumerated from `Cargo.toml` manifests, and it is **eleven**
  crates: the ten in `release.yml`'s `CRATES` array plus `paladin-herald`.
  `crates/doc-examples` (`publish = false`) and `fixtures/codeql-probe` (workspace-excluded) are
  the only non-publishable members. Verified live against crates.io on 2026-08-26: all ten
  `CRATES` entries exist on the registry at `max_version 0.5.1`; `paladin-herald` returns 404 —
  it has never been published. The tree is at 0.8.0 and tags v0.7.0/v0.7.1 exist but were never
  published to the registry — the last real publish was 0.5.1 (2026-06-04).
- **D-02:** Close the herald gap in this phase (add `paladin-herald` to the publish order, in
  dependency order, before `paladin-ai`, which depends on it as `version = "0.8.0", path = ...`).
  Reversible until the first real publish lands; one-way afterward.
- **D-03:** `paladin-herald`'s first publish happens during the proof event (PUB-03), while the
  old token is still valid, OR via the OIDC path directly if crates.io now supports Trusted
  Publishing for not-yet-existing crates (researcher must verify current new-crate support;
  historically a trust link could only be configured on an existing crate). Until
  `paladin-herald` exists on crates.io and its trust link is created, the trust table names it
  explicitly as *not covered* with its interim auth path stated.

**Proof event design (PUB-03)**
- **D-04:** The proof is a real prerelease publish through `release.yml`'s actual publish path
  (e.g. `v*.*.*-rc.1` tag or `workflow_dispatch` with `dry_run=false`), evidenced by run URL plus
  version visible on crates.io. Not a dry run, not an out-of-band manual `cargo publish`. The
  0.5.1 → 0.8.0 backlog is NOT cleared as the proof vehicle — a prerelease minimizes blast radius.
- **D-05:** Ordering is a ratchet, each step evidenced before the next: (1) trust links created
  on crates.io for the existing ten crates; (2) proof publish succeeds via OIDC (run URL +
  crates.io listing recorded); (3) token revoked at crates.io; (4) `CARGO_REGISTRY_TOKEN` deleted
  from repository secrets. Steps 3 and 4 never precede step 2. One-way at step 3.

**Environment and permission shape (PUB-02)**
- **D-06:** One GitHub Environment named **`crates-io`**, attached to the `publish-crates` job.
  The environment name is pinned in every crate's crates.io trust configuration. Costly to
  reverse — recorded in eleven per-crate trust configs plus the trust table doc.
- **D-07:** `id-token: write` is granted on the `publish-crates` job only, added alongside its
  existing `contents: read`. No workflow-level block. `docs.yml`'s grant is at *workflow* level
  (line 28-30) — proves the OIDC mechanism works in this repo, but its permission placement is
  NOT copied.
- **D-08:** Environment protection: deployment branch/tag policy restricted to `v*.*.*` tags; no
  required-reviewer gate initially. Reversible — both directions are settings changes.

**Failure-honesty rewrite of the skip branch (PUB-05)**
- **D-09:** The token-presence check and the `dry_run=skip` branch (`release.yml:391-395`) are
  deleted, not rewritten: mode becomes exactly `dry_run=true|false` from the dispatch input (tag
  pushes are always `false`). In real mode the `rust-lang/crates-io-auth-action` step runs
  unconditionally and its failure fails the job — no `continue-on-error` anywhere on the publish
  path.
- **D-10:** Dry-run mode skips the OIDC mint entirely — `cargo publish --dry-run` needs no
  credential. Docs state this boundary explicitly.

**Documentation and recording (PUB-04, PUB-05)**
- **D-11:** The per-crate trust table lives in `docs/src/appendix/release-automation.md`.
  Columns: crate name, source directory, workflow filename, environment name, link date, status
  (`linked` / `not covered — interim path: …`). `docs/src/appendix/release-checklist.md` is
  updated in the same change wherever it references the token.
- **D-12:** The revocation record (PUB-04) follows the Phase 9/12 convention — a named owner, a
  date, an actor — but NOT the `SECURITY-EXCEPTIONS.md` register file (scoped to RustSec advisory
  suppressions, mechanically checked). The revocation entry goes in a "Credential history"
  subsection of `release-automation.md` beside the trust table, plus a `CHANGELOG.md` entry.
- **D-13:** Human-in-the-loop steps are explicit plan checkpoints. Creating trust configurations
  and revoking the token happen in the crates.io UI under the crate-owner account (the user); no
  CI job or agent can perform them. GitHub Environment creation and repository secret deletion
  can be automated (`gh api`) with confirmation. Plans must sequence these as
  checkpoint/human-action tasks with exact instructions.

### Claude's Discretion

- Exact prerelease version string and whether the proof runs via prerelease tag or
  `workflow_dispatch` — pick whatever exercises the real publish path with least ceremony.
- Whether trust links for all ten existing crates are created before the proof or a single pilot
  crate is linked and proven first, then the rest.
- Wording and placement details inside `release-automation.md`, provided the table columns and
  credential-history record match D-11/D-12.

### Deferred Ideas (OUT OF SCOPE)

- Full registry catch-up release (0.5.1 → current) — Phase 20+ territory, after idempotency and
  pre-publish gate exist.
- Environment required-reviewer gate — deliberately not enabled now (D-08).
- `create-release@v1` replacement, index-wait fix, yank policy — Phase 20 (`PUBOPS-*`).
</user_constraints>

<phase_requirements>
## Phase Requirements

| ID | Description | Research Support |
|----|-------------|------------------|
| PUB-01 | Enumerate publishable crate set from `Cargo.toml`, reconcile with `CRATES` array before any trust link | Confirmed: eleven publishable crates (`Cargo.toml` diligence below); `paladin-herald` has no `publish = false`, is a path+version dependency of root `paladin-ai` only, and depends only on `paladin-ai-core` — see Dependency Order section |
| PUB-02 | OIDC exchange via `rust-lang/crates-io-auth-action`, `id-token: write` on `publish-crates` job only, protected GitHub Environment | Confirmed exact action usage, output variable, permissions shape, and environment mechanics — see Standard Stack, Architecture Patterns, Code Examples |
| PUB-03 | Prove new path publishes before old credential is destroyed; proof is not a dry run | Confirmed `--dry-run` needs no credential (official cargo/crates.io behavior); confirmed evidence shape (run URL + crates.io listing with "via GitHub" provenance label) — see Common Pitfalls, Code Examples |
| PUB-04 | Revoke at crates.io AND delete repo secret; both recorded with date + actor | Confirmed `gh secret delete` command shape; crates.io revocation is a UI-only action (human, per D-13) — see Environment Availability, Don't Hand-Roll |
| PUB-05 | Remove `dry_run=skip` silent-success branch; document per-crate trust config as a table | Confirmed exact lines to delete in `release.yml`; trust-config field set confirmed (owner/repo/workflow — required; environment — optional but D-06 pins it) — see Architecture Patterns, Standard Stack |
</phase_requirements>

## Summary

crates.io Trusted Publishing is a real, shipped feature (RFC 3691, merged and live since mid-2025,
with GitLab CI/CD support and a "Trusted Publishing Only" enforcement mode added by January 2026).
The mechanism is exactly what CONTEXT.md's D-01–D-13 already assume: a GitHub Actions job with
`id-token: write` calls `rust-lang/crates-io-auth-action@v1`, which exchanges the job's GitHub
OIDC identity token for a crates.io access token that expires roughly 30 minutes after mint and is
also explicitly revoked by the action's own `post` step at job end. `cargo publish` then reads that
token from `CARGO_REGISTRY_TOKEN` exactly as it does today for the static secret — no cargo version
bump, no publish-command changes.

The one load-bearing fact this research had to nail down before planning could proceed is D-03's
open question: **crates.io Trusted Publishing cannot be configured for a crate that has never been
published.** Every authoritative source (the official docs, the RFC, the July 2025 dev-update blog
post) states the same thing in the same words: the first release of any crate must be published
with a traditional API token; only after that first publish exists can a maintainer create a Trust
Publisher Configuration linking future releases to a GitHub repo/workflow/environment. crates.io's
own roadmap lists a PyPI-style "pending publisher" (configure trust before the first publish)
as a **future possibility**, not a shipped feature. This makes D-03's first branch ("during the
proof event, while the old token is still valid") the *only* viable path — the OIDC-direct
alternative D-03 hedges on does not exist yet. `paladin-herald` must be published with the
old `CARGO_REGISTRY_TOKEN` (which is precisely why D-05's ratchet places the proof publish *before*
revocation), and its trust link created immediately afterward, before the standing token is torn
down.

The publish-order mechanics also resolve cleanly: `paladin-herald` depends on nothing but
`paladin-ai-core`, and nothing except the root `paladin-ai` package depends on `paladin-herald` —
so it can be inserted anywhere after `paladin-ai-core` and before `paladin-ai` in the `CRATES`
array without perturbing the other nine crates' relative order.

The remaining operational risk this research surfaces (not covered explicitly by D-01–D-13) is
**token lifetime versus loop duration**: the action's token is good for ~30 minutes and is minted
once per job (not per `cargo publish` call), so a single mint must outlive the entire eleven-crate
sequential loop, including its `sleep 20` index-wait between crates. At current pacing (11 × ~20s
sleeps ≈ 4 minutes, plus per-crate `cargo publish` network/build time) this is very unlikely to
exceed 30 minutes, but it is a real, named risk the plan should record rather than silently assume
away — see Common Pitfalls.

**Primary recommendation:** Mint the OIDC token once per job (one `crates-io-auth-action` step,
job-scoped `env: CARGO_REGISTRY_TOKEN: ${{ steps.auth.outputs.token }}`), keep the existing
sequential-loop publish shape (that redesign is explicitly Phase 20's), delete the `dry_run=skip`
branch entirely rather than rewrite it, and treat `paladin-herald`'s first publish as a one-time,
manually-tokened bootstrap step inside the PUB-03 proof event — never described as "covered by
Trusted Publishing" until its trust link exists.

## Architectural Responsibility Map

| Capability | Primary Tier | Secondary Tier | Rationale |
|------------|-------------|----------------|-----------|
| OIDC identity token issuance | CI/CD platform (GitHub Actions runtime) | — | GitHub mints the workflow's OIDC ID token; not something the job's own code produces |
| OIDC → registry-token exchange | CI/CD job (via `rust-lang/crates-io-auth-action`) | External registry (crates.io) | The action running inside the `publish-crates` job calls crates.io's OIDC endpoint; crates.io validates and issues the short-lived token |
| Trust configuration (which repo/workflow/environment may mint a token for crate X) | External registry (crates.io, human-configured) | — | Lives entirely in crates.io's per-crate settings; no in-repo artifact enforces it except the documentation table (D-11) |
| Environment/branch-tag restriction (who can even reach the job that requests a token) | CI/CD platform (GitHub Environment protection rules) | — | GitHub, not crates.io, enforces "only `v*.*.*` tags may target the `crates-io` environment" |
| Token consumption for the actual publish | CI/CD job (`cargo publish` reading `CARGO_REGISTRY_TOKEN`) | — | Unchanged from today's static-secret flow; cargo does not know or care whether the token is static or OIDC-derived |
| Revocation of the standing credential | External registry (crates.io UI, human) + repo secret store (GitHub, `gh api`/`gh secret`) | — | Two independent systems must both drop the old token (D-04/PUB-04); neither alone is sufficient |

## Standard Stack

### Core
| Component | Version | Purpose | Why Standard |
|---------|---------|---------|--------------|
| `rust-lang/crates-io-auth-action` | `@v1` (major-version tag, official `rust-lang` org action) [VERIFIED: GitHub API — `owner.login == "rust-lang"`, `owner.type == "Organization"`, repo `rust-lang/crates-io-auth-action`, description "Get a crates.io temporary access token"] | Exchanges the job's GitHub OIDC token for a short-lived crates.io publish token | This is *the* action crates.io's own documentation and RFC point to; there is no alternative implementation to evaluate — it is maintained by the same org that runs crates.io [CITED: github.com/rust-lang/crates-io-auth-action] |
| GitHub Actions OIDC (`id-token: write` permission) | n/a (platform feature) | Issues the workflow-identity JWT the action exchanges | Already proven in this repo by `docs.yml`'s Pages deploy job [CITED: `.github/workflows/docs.yml:28-30,85-87`] |
| GitHub Environments | n/a (platform feature) | Scopes which refs/branches can reach the `id-token: write` job, and is itself embedded in the OIDC subject claim (`repo:org/repo:environment:NAME`) when the job is pinned to one | Confirmed by GitHub's own OIDC reference: the subject claim format changes to include `environment:<name>` specifically because the job declares `environment:` [CITED: docs.github.com/en/actions/reference/security/oidc via GitHub Changelog on granular OIDC claims] |

### Supporting
| Component | Version | Purpose | When to Use |
|---------|---------|---------|-------------|
| `gh api` (REST: `PUT /repos/{owner}/{repo}/environments/{name}`, `POST .../deployment-branch-policies`) | gh CLI 2.97.0 present in this devcontainer [VERIFIED: `gh --version` → `2.97.0`, `gh auth status` → authenticated as `Am0rfu5`] | Automate GitHub Environment creation + tag-restriction policy (D-13 says this half can be automated) | Creating the `crates-io` environment and its `v*.*.*` tag policy |
| `gh secret delete CARGO_REGISTRY_TOKEN` | gh CLI 2.97.0 | Delete the repository secret (PUB-04's non-crates.io half) | After PUB-03's proof succeeds and crates.io revocation is confirmed done by the human (D-05 step 3 before step 4) |

### Alternatives Considered
| Instead of | Could Use | Tradeoff |
|------------|-----------|----------|
| GitHub-native OIDC exchange via `crates-io-auth-action` | A hand-rolled `curl`/`jq` call against crates.io's OIDC token endpoint | Don't hand-roll this — the action already implements the exact request/response contract crates.io expects, including the `post`-step revocation. A hand-rolled version reimplements a security-sensitive protocol for no benefit (see Don't Hand-Roll). |
| `rust-lang/crates-io-auth-action@v1` (major-version tag) | Pinning to an exact commit SHA | The major-version tag is the convention crates.io's own docs and every example use; SHA-pinning is defensible defense-in-depth but is Claude's Discretion territory, not a locked decision — not required by any PUB-* requirement. |

**Installation:** No package install — `uses: rust-lang/crates-io-auth-action@v1` in the workflow YAML is a GitHub Action reference, not a Cargo/npm dependency. Nothing is added to `Cargo.toml` or `Cargo.lock`.

**Version verification:** `rust-lang/crates-io-auth-action` is referenced by major-version tag `@v1` in every official example (crates.io docs, RFC 3691, blog.rust-lang.org's 2025-07-11 dev update) [CITED: blog.rust-lang.org/2025/07/11/crates-io-development-update-2025-07]. Direct GitHub API lookup on 2026-08-26 confirms the repository is live, owned by the `rust-lang` GitHub organization, and undisputedly the canonical action [VERIFIED: GitHub API `repos/rust-lang/crates-io-auth-action`].

## Package Legitimacy Audit

No Cargo/npm/PyPI package is installed by this phase — `rust-lang/crates-io-auth-action` is a
GitHub Action referenced by `uses:` in workflow YAML, not a registry dependency, so the
`package-legitimacy check` seam (which targets npm/PyPI/crates registries) does not apply. The
equivalent diligence performed instead:

| Item | Type | Owner | Verdict | Disposition |
|------|------|-------|---------|-------------|
| `rust-lang/crates-io-auth-action` | GitHub Action | `rust-lang` org (GitHub API confirms `owner.type: "Organization"`, `owner.login: "rust-lang"`) [VERIFIED: GitHub API] | OK — canonical, official, single-purpose action maintained by the crates.io team itself | Approved, pin to `@v1` |

**Packages removed due to [SLOP] verdict:** none (not applicable — no registry packages).
**Packages flagged as suspicious [SUS]:** none.

## Architecture Patterns

### System Architecture Diagram

```
Tag push (v*.*.*) or workflow_dispatch(dry_run=false)
        │
        ▼
┌───────────────────────┐
│ verify-tag-source      │  (unchanged — already enforces tag descends from main)
└──────────┬─────────────┘
           ▼
┌───────────────────────┐        ┌────────────────────────────┐
│ test                   │        │ create-release              │
└──────────┬─────────────┘        └──────────────┬───────────────┘
           │                                       │
           └───────────────┬───────────────────────┘
                            ▼
              ┌─────────────────────────────┐
              │ publish-crates job           │
              │ permissions:                 │
              │   contents: read              │
              │   id-token: write   ◄── NEW   │
              │ environment: crates-io ◄── NEW│  ← GitHub Environment gates which
              └──────────────┬───────────────┘    refs/tags may even reach this job
                             │
                 dry_run == true?
                 ┌───────────┴────────────┐
                 │ yes                    │ no
                 ▼                        ▼
      cargo publish --dry-run    ┌─────────────────────────────┐
      (no credential needed,     │ rust-lang/                   │
       no OIDC mint)             │ crates-io-auth-action@v1      │──► GitHub issues OIDC ID token
                                 │   (id: auth)                  │      (subject includes
                                 └──────────────┬────────────────┘       repo:org/repo:environment:
                                                │                         crates-io)
                                                ▼
                                  crates.io validates the OIDC
                                  token against the crate's Trust
                                  Publisher Configuration (owner/
                                  repo/workflow/environment match)
                                                │
                                                ▼
                                  crates.io issues short-lived
                                  (~30 min) publish-scoped token
                                                │
                                                ▼
                          CARGO_REGISTRY_TOKEN = steps.auth.outputs.token
                                                │
                                                ▼
                          for each of 11 crates (dependency order):
                            cargo publish -p <crate>
                            (already-published tolerance unchanged —
                             Phase 20 owns making this a real check)
                                                │
                                                ▼
                          job ends → action's `post` step revokes
                          the minted token at crates.io automatically
```

### Recommended Project Structure

No new files/directories — this phase edits in place:

```
.github/workflows/release.yml     # publish-crates job: permissions, environment, auth step, CRATES array, skip-branch deletion
docs/src/appendix/release-automation.md   # trust table + credential-history subsection (D-11, D-12)
docs/src/appendix/release-checklist.md    # token references updated (D-11)
CHANGELOG.md                              # revocation record entry (D-12)
```

### Pattern 1: Job-scoped OIDC permission + protected Environment
**What:** Grant `id-token: write` only on the job that needs it, and pin that same job to a named
GitHub Environment whose deployment policy restricts eligible refs.
**When to use:** Any job that authenticates to an external OIDC relying party (crates.io, cloud
providers, container registries) — this is the general pattern GitHub itself documents for
"secure cloud deployments," and it is already proven in this repo by `docs.yml`.
**Example:**
```yaml
# Source: .github/workflows/docs.yml:27-30,80-87 (existing pattern in this repo)
# — permission is at WORKFLOW level here; PUB-02/D-07 require JOB level instead, shown below.
permissions:
  contents: read
  pages: write
  id-token: write
jobs:
  deploy:
    needs: build
    environment:
      name: github-pages
      url: ${{ steps.deployment.outputs.page_url }}
    steps:
      - uses: actions/deploy-pages@v4
```
```yaml
# Adapted shape for publish-crates (job-level permissions + environment, per D-06/D-07):
publish-crates:
  name: Publish to crates.io
  runs-on: ubuntu-latest
  needs: [test, create-release]
  environment: crates-io          # NEW — D-06
  permissions:
    contents: read
    id-token: write               # NEW — D-07, job-scoped, not workflow-scoped
  steps:
    - uses: actions/checkout@v4
    - uses: dtolnay/rust-toolchain@stable
    - name: Determine publish mode
      id: mode
      run: |
        if [ "${{ github.event.inputs.dry_run }}" = "true" ]; then
          echo "dry_run=true" >> "$GITHUB_OUTPUT"
        else
          echo "dry_run=false" >> "$GITHUB_OUTPUT"
        fi
    - name: Authenticate with crates.io
      id: auth
      if: steps.mode.outputs.dry_run != 'true'
      uses: rust-lang/crates-io-auth-action@v1
    - name: Publish crates in dependency order
      env:
        CARGO_REGISTRY_TOKEN: ${{ steps.auth.outputs.token }}
        DRY_RUN: ${{ steps.mode.outputs.dry_run }}
      run: |
        # ... unchanged loop body, CRATES array gains paladin-herald ...
```
*(Confirmed action usage shape — `id: auth`, `uses: rust-lang/crates-io-auth-action@v1`, and
`CARGO_REGISTRY_TOKEN: ${{ steps.auth.outputs.token }}` — is the exact pattern shown by the
official crates.io docs, the RFC's example workflow, and an independent third-party verification
article [CITED: blog.rust-lang.org/2025/07/11/crates-io-development-update-2025-07; CITED:
github.com/rust-lang/rfcs/blob/master/text/3691-trusted-publishing-cratesio.md; secondary
confirmation: zenn.dev/kesin11 write-up of a real migration].)*

### Pattern 2: Environment tag-restriction via `gh api`
**What:** Create a GitHub Environment and restrict which tags may deploy to it, entirely via the
REST API (no web UI click-through required).
**When to use:** D-13 designates this as the automatable half of the human/automation split.
**Example:**
```bash
# Source: docs.github.com/en/rest/deployments/branch-policies (GitHub REST API reference)
# Step 1 — create/update the environment, enabling custom deployment policies:
gh api --method PUT "repos/{owner}/{repo}/environments/crates-io" \
  -f "deployment_branch_policy[protected_branches]=false" \
  -f "deployment_branch_policy[custom_branch_policies]=true"

# Step 2 — add the tag-pattern policy (note "type": "tag", not "branch"):
gh api --method POST "repos/{owner}/{repo}/environments/crates-io/deployment-branch-policies" \
  -f "name=v*.*.*" -f "type=tag"
```
*(Endpoint shapes and the `type: branch|tag` field [CITED: docs.github.com/en/rest/deployments/branch-policies].)*

### Anti-Patterns to Avoid
- **Minting a fresh OIDC token per `cargo publish -p <crate>` call inside the bash loop:** the
  action can only run once per step invocation in a static workflow; looping the auth step would
  require restructuring the job into a matrix (out of scope — Phase 20 owns loop redesign). Mint
  once, reuse the env var for the whole loop, and treat lifetime as a documented risk instead
  (see Common Pitfalls).
- **Leaving the `environment` trust-config field blank on crates.io:** D-06 is explicit that an
  empty environment field lets *any* branch/workflow with `id-token: write` in the trusted repo
  mint a token for that crate — pinning `crates-io` is the actual protection, not a nicety.
- **`continue-on-error: true` on the auth or publish step:** violates D-09's honesty rule
  directly; a failed mint or failed publish must fail the job.

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| Exchanging a GitHub OIDC token for a crates.io publish token | A `curl`/`jq` script hitting crates.io's OIDC token endpoint directly | `rust-lang/crates-io-auth-action@v1` | The action is maintained by the same team that runs crates.io's OIDC endpoint; it already implements the exact request signing, the audience claim, and — critically — the `post`-step revocation that cleans up the minted token at job end. A hand-rolled version has to reimplement that revocation step correctly or leaves a live token dangling past job completion. |
| Restricting which refs can request a publish-scoped token | A bash guard inside the job (`if [[ "$GITHUB_REF" == refs/tags/v* ]]`) | GitHub Environment deployment branch/tag policy (D-08) | A bash guard runs *after* the job has already started and the OIDC token has already been issued to the runner — it protects nothing at the identity layer. The Environment policy prevents the job from running at all on a non-matching ref, which is what actually shows up in the OIDC subject claim crates.io validates against. |
| Detecting whether a crate version is already published | Regex-matching `cargo publish`'s combined stdout/stderr (`grep -qiE "already (exists|uploaded)|..."`, the current `release.yml:428`) | *(Out of scope for this phase — Phase 20/PUBOPS-02 owns replacing this with a registry-state check)* | Named here only so the planner does not accidentally "fix" this while touching the same loop for PUB-05; it is explicitly PUBOPS-02's job, not PUB-05's. |

**Key insight:** Every piece of this phase that touches *identity and credential issuance*
(OIDC exchange, environment gating) has an official, narrowly-scoped tool. The only thing this
phase legitimately hand-writes is the bash control flow deciding *when* to call that tool
(dry-run vs real) — and D-09 already specifies exactly what that control flow must look like.

## Common Pitfalls

### Pitfall 1: Assuming Trusted Publishing works for a crate that has never been published
**What goes wrong:** A plan attempts to create a crates.io Trust Publisher Configuration for
`paladin-herald` before it has ever been published, and the crates.io UI has no such option to
offer (there is no crate settings page for a crate that doesn't exist yet).
**Why it happens:** The mental model "OIDC replaces the token everywhere" over-generalizes; PyPI's
"pending publisher" feature (configure trust *before* the first publish) does not have a crates.io
equivalent yet — it is listed as a future possibility, not shipped.
**How to avoid:** Treat `paladin-herald`'s first publish as a manual, `CARGO_REGISTRY_TOKEN`-backed
`cargo publish -p paladin-herald` executed *inside* the PUB-03 proof window, immediately followed
by creating its trust link. Never place it in the OIDC-only loop until that link exists.
**Warning signs:** Any plan task that says "configure Trusted Publishing for paladin-herald" as a
prerequisite to publishing it for the first time is inverted — order it the other way.
[CITED: crates.io/docs/trusted-publishing (via aggregated WebSearch of the live page); CITED:
blog.rust-lang.org/2025/07/11/crates-io-development-update-2025-07 — "To get started with Trusted
Publishing, you'll need to publish your first release manually"]

### Pitfall 2: Token lifetime vs. eleven-crate sequential loop duration
**What goes wrong:** The action mints one ~30-minute token at job start; if the eleven-crate loop
(each with a network round-trip, package build/verify, and the existing `sleep 20` index-wait)
runs long — e.g. due to a slow crate, a registry hiccup, or CI runner contention — the token could
expire before the last crate(s) publish, and `cargo publish` fails with an auth error that looks
like a workflow bug rather than an expected-and-handled condition.
**Why it happens:** The token is minted once per job, not once per `cargo publish` invocation
[CITED: rust-lang/crates-io-auth-action README — token output is read once and reused across
subsequent steps; RFC 3691 leaves exact lifetime as "long enough initially," and the ~30-minute
figure is corroborated by multiple independent secondary sources].
**How to avoid:** At current pacing (11 × 20s sleeps ≈ 4 minutes plus build/upload time per crate)
this is unlikely to bite, but the plan should record it as a known, accepted risk rather than
silently assume it away — e.g. a one-line note in `release-automation.md`'s credential-history
section, or a follow-up flag for Phase 20 if the loop redesign changes crate count or pacing.
**Warning signs:** A publish failure on one of the later crates in the `CRATES` array with an
auth/403 error, especially on a slow CI day.

### Pitfall 3: Treating `cargo publish --dry-run` as evidence of OIDC working
**What goes wrong:** A plan (or a future person re-reading `release.yml`) points at a green
dry-run workflow run as proof the OIDC path works.
**Why it happens:** `cargo publish --dry-run` performs packaging/manifest validation only and
never calls the registry's authenticated publish endpoint — it needs no credential of any kind,
static or OIDC-derived.
**How to avoid:** D-04 already forbids this explicitly; this pitfall entry exists so the plan's
verification steps check for it directly (e.g., grep the proof evidence for "dry-run" and fail the
checkpoint if found).
**Warning signs:** Evidence for PUB-03 that only cites a `dry_run=true` workflow run.
[CITED: doc.rust-lang.org/cargo/reference/publishing.html — `--dry-run` "Performs all checks
without uploading."]

### Pitfall 4: Confusing `workflow_dispatch` eligibility with blocked trigger types
**What goes wrong:** Assuming `workflow_dispatch`-triggered runs cannot mint an OIDC token because
some triggers are blocked from Trusted Publishing.
**Why it happens:** crates.io does block `pull_request_target` and `workflow_run` from Trusted
Publishing specifically because those triggers have a documented history of GitHub Actions
security incidents (an attacker-controlled fork can cause them to run with elevated context).
`workflow_dispatch` is not in that blocked list in any source found during this research — it is a
manually-invoked, repo-collaborator-gated trigger with no equivalent confused-deputy risk.
**How to avoid:** D-04's `workflow_dispatch` proof-event option should work; this is corroborated
but not exercised in *this* repository yet — which is exactly what the proof event is for. If the
proof run fails specifically at the OIDC-mint step under `workflow_dispatch`, that is new,
repo-specific information the plan's checkpoint should surface immediately (do not silently fall
back to a tag-push proof without recording why).
**Warning signs:** An auth-action failure specifically correlated with `event_name ==
'workflow_dispatch'` and not with tag pushes.
[CITED: multiple secondary sources aggregate the same fact — `pull_request_target` and
`workflow_run` are explicitly blocked; no source found blocks `workflow_dispatch`. This is
MEDIUM confidence, not HIGH — the official docs page itself could not be fetched in full during
this research (JS-rendered), so this rests on WebSearch aggregation of the page's cached content
plus RFC 3691's discussion, not a direct read of the current live page.]

### Pitfall 5: Deleting the repo secret before crates.io revocation is confirmed
**What goes wrong:** `gh secret delete CARGO_REGISTRY_TOKEN` runs first because it's the
automatable half (D-13), giving a false sense that PUB-04 is "mostly done," while the actual
publish-scoped token is still live at crates.io (pasted into some other CI system, a local
`~/.cargo/credentials`, etc.).
**Why it happens:** The GitHub-side action is scriptable and satisfying to automate; the
crates.io-side revocation is a manual UI click a human must perform, so it's tempting to
sequence the easy part first.
**How to avoid:** D-04/PUB-04 is explicit that crates.io revocation is "the load-bearing half."
Sequence: human revokes at crates.io UI first (with owner+date recorded) → THEN `gh secret delete`
runs, in the same session, so there's no window where secret-deletion looks complete but the live
credential isn't actually dead. This is a plan-ordering discipline, not a technical constraint.
**Warning signs:** A `release-automation.md` credential-history entry with a secret-deletion date
but no accompanying crates.io-revocation date, or vice versa with a large gap between them.

## Code Examples

Verified patterns from official/CITED sources:

### Minimal Trusted Publishing workflow shape
```yaml
# Source: aggregated from blog.rust-lang.org/2025/07/11/crates-io-development-update-2025-07
# and github.com/rust-lang/rfcs/blob/master/text/3691-trusted-publishing-cratesio.md
permissions:
  id-token: write     # job-scoped in this repo's adaptation, per D-07
jobs:
  publish:
    steps:
      - uses: actions/checkout@v4
      - uses: rust-lang/crates-io-auth-action@v1
        id: auth
      - run: cargo publish
        env:
          CARGO_REGISTRY_TOKEN: ${{ steps.auth.outputs.token }}
```

### Environment tag-restriction via REST API
```bash
# Source: docs.github.com/en/rest/deployments/branch-policies
curl -L -X POST \
  https://api.github.com/repos/OWNER/REPO/environments/crates-io/deployment-branch-policies \
  -H "Authorization: Bearer $GH_TOKEN" \
  -H "Accept: application/vnd.github+json" \
  -H "X-GitHub-Api-Version: 2026-03-10" \
  -d '{"name": "v*.*.*", "type": "tag"}'
```

### Repo secret deletion
```bash
# Source: cli.github.com/manual/gh_secret_delete
gh secret delete CARGO_REGISTRY_TOKEN --repo DF3NDR/paladin-dev-env
```

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|---------------|--------|
| Long-lived `CARGO_REGISTRY_TOKEN` repo secret, manually rotated | crates.io Trusted Publishing — per-run OIDC-derived token, ~30 min lifetime, auto-revoked at job end | RFC 3691 shipped mid-2025; GitLab CI/CD support and a "Trusted Publishing Only" enforcement mode added by the January 2026 crates.io dev update | Removes the standing-credential attack surface this phase exists to close; matches the OIDC pattern already proven in this repo by `docs.yml`'s GitHub Pages deploy |

**Deprecated/outdated:**
- Static, indefinitely-lived registry tokens for CI publishing: not deprecated by crates.io (still
  supported, and required for the very-first publish of any crate), but no longer the recommended
  posture for a repo that publishes regularly — which is the entire premise of PUB-04.

## Assumptions Log

| # | Claim | Section | Risk if Wrong |
|---|-------|---------|---------------|
| A1 | `workflow_dispatch`-triggered runs are eligible to mint a Trusted Publishing token (only `pull_request_target` and `workflow_run` are explicitly blocked) | Common Pitfalls #4, D-04 support | If wrong, the `workflow_dispatch` branch of D-04's proof-event choice fails at the OIDC-mint step and the plan must fall back to a tag-push (`v*.*.*-rc.1`) proof instead — low cost since Claude's Discretion already leaves this choice open and the proof event itself will surface the failure immediately, before revocation |
| A2 | A single minted token (~30 min) comfortably covers the full eleven-crate sequential publish loop at current pacing (~4 min of sleeps plus build/upload overhead per crate) | Common Pitfalls #2 | If wrong on a slow CI run, one or more later crates fail to publish mid-loop with an auth error; recoverable by re-running the job (mints a fresh token), but produces a confusing half-published state that looks like a different failure class — Phase 20 owns the general half-published recovery story, but this phase's plan should not be surprised by it |
| A3 | The exact fields of a crates.io Trust Publisher Configuration are: required (GitHub owner, repo name, workflow filename) + optional (GitHub Actions environment name) | Standard Stack, D-06 | If crates.io's actual UI has additional required fields (e.g., branch pattern) not captured here, the human doing the D-13 checkpoint (creating trust links) will discover this directly in the UI — low risk since this is a human-executed step with the UI as ground truth, not a scripted assumption |
| A4 | Token lifetime is ~30 minutes | Summary, Common Pitfalls #2 | This figure is repeated across multiple secondary sources but the RFC itself only commits to "long enough initially" without a hard number; if actual lifetime differs materially (e.g., 5 min vs 60 min), Pitfall #2's risk calculus shifts — the proof event (PUB-03) will surface the real number empirically the first time it's exercised |

**None of these blocks planning** — each is either resolved by the proof event itself (A1, A4) or
low-consequence and independently recoverable (A2, A3). None represents a compliance, security
policy, or irreversible-decision risk requiring a human confirmation *before* execution beyond
what D-13 already mandates.

## Open Questions

1. **Exact UI mechanics of creating a crates.io Trust Publisher Configuration**
   - What we know: required fields are GitHub owner, repo, workflow filename; environment is
     optional but D-06 pins it to `crates-io`. This is a per-crate settings-page action.
   - What's unclear: whether crates.io's UI supports bulk-linking multiple crates at once, or
     requires ten (then eleven) separate visits to ten (then eleven) separate crate settings pages.
   - Recommendation: Plan for ten-to-eleven separate manual linking actions (D-13 checkpoint),
     not a bulk operation, until proven otherwise during execution.

2. **Whether a `-rc.1` prerelease tag or `workflow_dispatch` is the lower-ceremony proof vehicle**
   - What we know: both satisfy D-04's "real publish, not dry-run" requirement.
   - What's unclear: whether `workflow_dispatch` on this repo's `release.yml` currently accepts
     dispatch from non-default branches, and whether the `verify-tag-source` job's tag-resolution
     logic (`git rev-list -n 1 "${{ inputs.tag }}"`) requires the tag to already exist before
     dispatch — which would mean the tag must be pushed (even if unpublished as a GitHub release)
     before the `workflow_dispatch` proof can run.
   - Recommendation: Left to Claude's Discretion per CONTEXT.md; the planner should pick whichever
     requires fewer coordinated steps given `verify-tag-source`'s existing tag-must-exist logic
     (`release.yml:38-53`) — likely favoring a real prerelease tag push over `workflow_dispatch`,
     since the latter still needs the tag to pre-exist for `verify-tag-source` to resolve it.

## Environment Availability

| Dependency | Required By | Available | Version | Fallback |
|------------|------------|-----------|---------|----------|
| `gh` CLI | Environment creation, secret deletion (D-13 automatable half) | ✓ | 2.97.0 | — |
| `gh auth` (GH_TOKEN scope) | Same — `gh api` writes (PUT environment, POST branch policy) and `gh secret delete` both require write-level repo scope, not just read | ✓ (authenticated as `Am0rfu5`) | — | If the active token turns out to be read-only for these specific write endpoints (per memory note on read-only-token failure modes), the fallback is the GitHub web UI — both environment creation and secret deletion are directly clickable there with no automation dependency |
| `cargo` / `rustc` | Publishing itself (unchanged by this phase) | ✓ | cargo 1.97.1, rustc 1.97.1 | — |
| crates.io UI access under the crate-owner account | D-13's human-only half: creating trust links, revoking the old token | Not verifiable from this environment — human-operated per D-13 | — | None — this is inherently human-gated and cannot be automated or worked around |

**Missing dependencies with no fallback:**
- None that block starting the phase. The crates.io UI actions are human-gated by design (D-13),
  not missing tooling.

**Missing dependencies with fallback:**
- `gh api`/`gh secret` write scope, if the current token proves read-only for these specific
  operations: fall back to the GitHub web UI for environment creation and secret deletion.

## Validation Architecture

This phase modifies a GitHub Actions workflow (`release.yml`) and documentation
(`release-automation.md`, `release-checklist.md`, `CHANGELOG.md`) rather than first-party Rust
library code. There is no `cargo test` surface to extend — the correctness of the change is
proven by the PUB-03 proof event itself (a real workflow run against the real registry), which is
a stronger form of validation than a unit test could provide for an authentication protocol this
phase does not implement, only wires up.

### Test Framework
| Property | Value |
|----------|-------|
| Framework | N/A — no Rust code changes; validation is the live workflow run (PUB-03) |
| Config file | `.github/workflows/release.yml` (the artifact under test) |
| Quick run command | `gh workflow run release.yml -f tag=<prerelease-tag> -f dry_run=true` (packaging-only sanity check before the real proof) |
| Full suite command | `gh workflow run release.yml -f tag=<prerelease-tag> -f dry_run=false` (the actual PUB-03 proof event) — or a real `v*.*.*-rc.1` tag push |

### Phase Requirements → Test Map
| Req ID | Behavior | Test Type | Automated Command | File Exists? |
|--------|----------|-----------|-------------------|-------------|
| PUB-01 | Eleven crates enumerated, `CRATES` array matches | manual/diligence | `cargo metadata --no-deps --format-version 1 \| jq -r '.packages[] \| select(.publish == null) \| .name'` cross-checked against `CRATES` array | ✅ command runnable today, no new file needed |
| PUB-02 | `id-token: write` present on `publish-crates` job only; `environment: crates-io` set | static check | `grep -A3 "^  publish-crates:" .github/workflows/release.yml` | ✅ |
| PUB-03 | Real (non-dry-run) publish succeeds via OIDC | live smoke test | `gh workflow run release.yml -f tag=<rc-tag> -f dry_run=false` then check crates.io shows "via GitHub" provenance on the new version | ❌ — this is the proof event itself, not a pre-existing test; Wave 0 gap is "run this once, capture the run URL + crates.io screenshot/API response as evidence" |
| PUB-04 | Secret deleted + token revoked, both recorded | manual verification | `gh secret list \| grep -v CARGO_REGISTRY_TOKEN` (expect no match) + crates.io token list UI showing the old token revoked/absent | ❌ — human-executed, recorded in `release-automation.md` |
| PUB-05 | `dry_run=skip` branch absent; job fails loud on auth/publish failure | static check + negative test | `grep -c "dry_run=skip" .github/workflows/release.yml` (expect 0) | ✅ |

### Sampling Rate
- **Per task commit:** static greps/diffs against `release.yml` (fast, no live workflow run needed)
- **Per wave merge:** none required until the proof-event wave
- **Phase gate:** the PUB-03 proof event must have run and produced recorded evidence before PUB-04's revocation steps execute — this *is* the phase's Nyquist-equivalent gate, enforced by D-05's ratchet ordering, not by a CI job

### Wave 0 Gaps
- No test-framework gap — this phase's "test" is a real, once-per-crate-set workflow execution
  against the live registry, which cannot be pre-built as a repeatable automated suite without
  publishing extra crate versions each time. Record it as `None — validation is the live PUB-03
  proof event by design; no pre-existing test infrastructure gap applies.`

## Security Domain

### Applicable ASVS Categories

| ASVS Category | Applies | Standard Control |
|---------------|---------|-----------------|
| V2 Authentication | yes | OIDC token exchange replaces static bearer credential — the entire point of this phase |
| V3 Session Management | yes (adapted — "session" here is the ~30-min minted publish token) | Token lifetime is short and bound to a single job execution; explicit revocation via the action's `post` step, not implicit expiry alone |
| V4 Access Control | yes | GitHub Environment deployment branch/tag policy (D-08) restricts which refs can even request a token; crates.io's per-crate Trust Publisher Configuration (owner/repo/workflow/environment) restricts which identity may redeem one |
| V5 Input Validation | no | Not applicable — this phase does not process untrusted external input; it configures a trust relationship between two already-trusted parties (this repo's CI and this project's crates.io account) |
| V6 Cryptography | no (delegated) | OIDC token signing/verification is entirely GitHub's and crates.io's responsibility; this phase's code never touches key material directly |

### Known Threat Patterns for GitHub Actions OIDC Publishing

| Pattern | STRIDE | Standard Mitigation |
|---------|--------|---------------------|
| Confused-deputy via `pull_request_target`/`workflow_run` triggers minting a token on attacker-influenced context | Spoofing / Elevation of Privilege | crates.io blocks these trigger types from Trusted Publishing entirely (confirmed via multiple secondary sources) — this repo's `publish-crates` job is only reachable from `push: tags: v*.*.*` and `workflow_dispatch`, neither of which is in the blocked set |
| Standing credential left live after "migration" (the exact failure this phase targets) | Elevation of Privilege / Repudiation | D-04/PUB-04 — dual revocation (crates.io + repo secret) with recorded owner+date, sequenced revoke-first-then-delete-secret per Pitfall #5 |
| Untrusted branch/tag reaching a job with `id-token: write` | Elevation of Privilege | GitHub Environment deployment tag policy restricted to `v*.*.*` (D-08); `verify-tag-source` job already independently enforces the tag descends from `main` before any downstream job runs |
| Overly-broad trust configuration (empty environment field on crates.io, matching any workflow in the repo) | Elevation of Privilege | D-06 — environment field on every crate's trust config is explicitly pinned to `crates-io`, never left blank |

## Sources

### Primary (HIGH confidence)
- GitHub REST API `repos/rust-lang/crates-io-auth-action` — direct API call confirming action
  ownership/organization on 2026-08-26 [VERIFIED]
- `.github/workflows/docs.yml` (this repository) — the working, in-repo OIDC precedent (lines
  27-30 workflow-level `id-token: write`, 80-87 `environment: github-pages`) [VERIFIED — direct file read]
- `.github/workflows/release.yml` (this repository) — the exact lines to change: `:381-396`
  (mode-selection + `dry_run=skip` branch), `:407-418` (`CRATES` array), `:368-373` (job
  permissions) [VERIFIED — direct file read]
- `Cargo.toml` (workspace root) + `crates/paladin-herald/Cargo.toml` — dependency-graph
  confirmation that `paladin-herald` depends only on `paladin-ai-core`, and only the root
  `paladin-ai` depends on `paladin-herald` [VERIFIED — direct file read + grep]

### Secondary (MEDIUM confidence)
- `github.com/rust-lang/rfcs` — RFC 3691 (Trusted Publishing on crates.io), the accepted design
  document [CITED]
- `blog.rust-lang.org/2025/07/11/crates-io-development-update-2025-07` — official crates.io team
  blog post confirming shipped state, first-publish-must-be-manual limitation, example workflow
  [CITED]
- `docs.github.com` (OIDC reference, deployments/branch-policies REST reference) — official
  GitHub documentation on OIDC subject claims and environment branch/tag policy API [CITED]
- `github.com/rust-lang/crates-io-auth-action` README — action inputs/outputs/behavior [CITED]
- `crates.io/docs/trusted-publishing` — official docs page; content aggregated via WebSearch
  because direct WebFetch returned only the page title (JS-rendered SPA) — treated as CITED but
  flagged lower-confidence than a direct fetch would be [CITED, partially indirect]

### Tertiary (LOW confidence)
- `socket.dev/blog/crates-launches-trusted-publishing`, `alpha-omega.dev` blog,
  `zenn.dev/kesin11` migration write-up, `magazine.ediary.site` — third-party summaries used only
  to corroborate details already found in primary/secondary sources (e.g., the ~30-minute token
  lifetime figure, the "VIA GITHUB" provenance label on published versions); no claim in this
  document rests solely on a tertiary source

## Metadata

**Confidence breakdown:**
- Standard stack (action usage, permissions shape, environment mechanics): HIGH — corroborated
  by official RFC + official blog + this repo's own working `docs.yml` precedent
- First-publish-requires-manual-token limitation (D-03's core question): HIGH — stated identically
  and unambiguously across every official source found
- Multi-crate loop token-lifetime interaction: MEDIUM — the ~30-minute figure and single-mint-per-job
  behavior are corroborated but not drawn from a single authoritative numeric spec; treated as a
  named risk (Pitfall #2), not a blocker
- `workflow_dispatch` trigger eligibility for Trusted Publishing: MEDIUM — inferred from an
  explicit blocklist (`pull_request_target`, `workflow_run`) that does not name
  `workflow_dispatch`, rather than from an explicit allowlist statement

**Research date:** 2026-08-26
**Valid until:** ~30 days (crates.io Trusted Publishing is an actively evolving feature — GitLab
support and "Trusted Publishing Only" mode both shipped within the last year; re-verify the
first-publish limitation and any new pending-publisher feature before relying on this document
past 2026-09-25)
