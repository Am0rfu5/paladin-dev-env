# Phase 19: crates.io Trusted Publishing — Publish Evidence Log

Phase: 19-crates-io-trusted-publishing-replace-the-long-lived-registry
Requirements: PUB-01, PUB-02, PUB-03, PUB-04, PUB-05

This document is the phase's evidence log, following the shape of Phase 18's
`18-CODEQL-EVIDENCE.md`: measured facts, dated and sourced, not summarized-away.
Every crates.io API call in this document requires a `User-Agent` header —
crates.io answers `403` without one, the same failure mode ADR-0026 recorded as
"crates.io returns HTTP 403 in this environment". Every `curl` call below used
`-H 'User-Agent: paladin-release-check (github.com/DF3NDR/paladin-dev-env)'`.

## Crate-Set Reconciliation

**Date:** 2026-08-26

Three sources claim to describe the publishable crate set. Before this plan, they
disagreed: `cargo metadata` and `.crate-names.txt` both reported eleven names,
`release.yml`'s `CRATES` array reported ten — missing `paladin-herald`.

| Source | Count | Verdict |
|---|---|---|
| `cargo metadata --no-deps --format-version 1` (packages with `publish == null`) | 11 | authoritative — this is what Cargo will actually try to publish |
| `.crate-names.txt` (hand-edited owned-name allow-list) | 11 | agrees with manifests |
| `release.yml`'s `CRATES` array (pre-change, `git show HEAD`) | 10 | **disagreed** — missing `paladin-herald` |
| `release.yml`'s `CRATES` array (post-change, this plan) | 11 | reconciled |

### Eleven-crate name-to-source-directory map

| crates.io package name | Source directory | Note |
|---|---|---|
| `paladin-ai` | workspace root (`Cargo.toml`) | facade package; name diverges from directory |
| `paladin-ai-core` | `crates/paladin-core` | name diverges from directory |
| `paladin-ports` | `crates/paladin-ports` | |
| `paladin-herald` | `crates/paladin-herald` | not yet published — see below |
| `paladin-battalion` | `crates/paladin-battalion` | |
| `paladin-llm` | `crates/paladin-llm` | |
| `paladin-memory` | `crates/paladin-memory` | |
| `paladin-storage` | `crates/paladin-storage` | |
| `paladin-notifications` | `crates/paladin-notifications` | |
| `paladin-content` | `crates/paladin-content` | |
| `paladin-web` | `crates/paladin-web` | |

Recording `paladin-ai-core` → `crates/paladin-core` and `paladin-ai` → workspace
root explicitly here (not just once, in a footnote) is what makes an undocumented
Trusted Publishing configuration expensive to reconstruct — a future operator
searching for "paladin-ai-core" in the tree by directory name would not find it.

The two legitimate non-publishable workspace members are `crates/doc-examples`
(`publish = false`) and `fixtures/codeql-probe` (excluded from the workspace
entirely — Phase 18, D-07). Neither appears in any of the three sources above,
and neither should.

### Dependency-Order Constraints

Three constraints determine where `paladin-herald` may sit in the publish order:

1. `paladin-herald` has a normal `[dependencies]` edge on `paladin-ai-core`
   (`paladin-core = { package = "paladin-ai-core", version = "0.8.0", path = ... }`)
   — must publish **after** `paladin-ai-core`.
2. `paladin-herald` has a version-pinned `[dev-dependencies]` edge on
   `paladin-ports` (`paladin-ports = { version = "0.8.0", path = "../paladin-ports" }`).
   Cargo records a version-carrying dev-dependency in the published manifest, and
   crates.io validates that dependency against the index at publish time — so
   `paladin-ports` must already be on the registry when `paladin-herald` is
   published. This must publish **after** `paladin-ports`.
3. Only the root `paladin-ai` package depends on `paladin-herald`
   (`paladin-herald = { workspace = true }` in the root `[dependencies]`) — must
   publish **before** `paladin-ai`.

**Corrected insertion point:** after `paladin-ai-core`, after `paladin-ports`,
before `paladin-ai` — i.e. immediately following `paladin-ports` in the existing
nine-crate order.

**Superseded predecessor (wrong):** both `19-PATTERNS.md` ("Insert
`paladin-herald` into the `CRATES` array between `paladin-ai-core` and
`paladin-ports`") and `19-RESEARCH.md` ("`paladin-herald` depends on nothing but
`paladin-ai-core`") proposed inserting `paladin-herald` immediately after
`paladin-ai-core` and before `paladin-ports` — a position that satisfies
constraint 1 but violates constraint 2, because neither document accounted for
the version-pinned dev-dependency in `crates/paladin-herald/Cargo.toml`'s
`[dev-dependencies]` table. Reading `crates/paladin-herald/Cargo.toml` directly,
rather than trusting the planning documents' restated reasoning, is what
surfaced the correction.

**Applied order** (`release.yml`'s `CRATES` array, this plan):

```
paladin-ai-core
paladin-ports
paladin-herald   <- corrected insertion point
paladin-battalion
paladin-llm
paladin-memory
paladin-web
paladin-notifications
paladin-content
paladin-storage
paladin-ai
```

Verified topologically valid by the reconciliation one-liner (see PLAN.md Task 1
`<verify>`): `RECONCILED: 11 crates, order topologically valid`. Run against
`git show HEAD:.github/workflows/release.yml` (the pre-change, ten-crate array),
the same one-liner reports `SET MISMATCH: workflow=10 manifests=11
diff=["paladin-herald"]` — the fail-first proof that the gate has teeth.

## Live Registry State (pre-bootstrap)

**Date measured:** 2026-08-26

| crates.io package | `max_version` | HTTP status |
|---|---|---|
| `paladin-ai-core` | 0.5.1 | 200 |
| `paladin-ports` | 0.5.1 | 200 |
| `paladin-battalion` | 0.5.1 | 200 |
| `paladin-llm` | 0.5.1 | 200 |
| `paladin-memory` | 0.5.1 | 200 |
| `paladin-web` | 0.5.1 | 200 |
| `paladin-notifications` | 0.5.1 | 200 |
| `paladin-content` | 0.5.1 | 200 |
| `paladin-storage` | 0.5.1 | 200 |
| `paladin-ai` | 0.5.1 | 200 |
| `paladin-herald` | — (never published) | **404** |

All ten existing crates sit at `0.5.1` — the last real registry publish
(2026-06-04, per CONTEXT.md D-01). The workspace tree is at `0.8.0`; tags
`v0.7.0`/`v0.7.1` were cut but never reached the registry. `paladin-herald`
returns HTTP 404 — it has never been published under any version, which is the
precondition crates.io imposes before a Trusted Publishing configuration can be
created for it at all (D-03).

Reproduction command (per crate, `paladin-herald` shown):

```
curl -sf -H 'User-Agent: paladin-release-check (github.com/DF3NDR/paladin-dev-env)' \
  https://crates.io/api/v1/crates/paladin-herald
```

## Bootstrap Publish (old credential)

### Task 2 decision: the one-way door

**Decision:** `bootstrap-now`
**Date:** 2026-08-26
**Deciding actor:** GSD orchestrator (auto-mode chain, `19-01` plan execution), applying CONTEXT.md's locked decisions D-02 and D-03 rather than introducing new judgment. Auto-mode was active at the orchestrator level for this chain (the plan-local config read inside this executor's worktree showed `workflow._auto_chain_active: false` because the flag was toggled at the orchestrator after this worktree forked — an isolation artifact of worktree execution, not a live re-evaluation).
**Exact version to be published:** `0.8.1-rc.1` (all eleven crates, in lockstep, per `make release VERSION=0.8.1-rc.1`)

**Rationale:** `bootstrap-now` is the option CONTEXT.md already locked, not a fresh choice —
D-02 explicitly chose closing the `paladin-herald` gap in this phase over recording it as a
named exclusion, and 19-RESEARCH.md's Summary settled D-03's open question: crates.io cannot
configure Trusted Publishing for a crate that has never been published, so a first publish
through the still-live `CARGO_REGISTRY_TOKEN` during the proof window is the only viable
route to ever covering `paladin-herald`. Choosing `defer-herald` here would leave `cargo
publish -p paladin-ai` permanently broken against the registry and hand Phase 20's
pre-publish gate the same defect PUB-01 exists to close.

The one-way, irreversible act itself — actually running `make release VERSION=0.8.1-rc.1`
from `main`, which pushes a tag and spends the standing publish credential — remains gated
behind Task 3's `checkpoint:human-action`. That gate is not auto-approved: a human with push
access to `main` and crates.io account ownership must perform it. This Task 2 record settles
*which path* the plan takes; it does not itself publish anything.

### Task 3 execution: the bootstrap publish

**Date:** 2026-08-26
**Actor:** Am0rfu5 (repository owner; workflow dispatches and PR merges performed by
Claude Code operating with the owner's fine-grained PAT, at the owner's explicit
request after the checkpoint was presented)
**Credential used:** the standing `CARGO_REGISTRY_TOKEN` repository secret — token
publish, inside the pre-revocation window D-05 step 2 permits. This run is
deliberately NOT evidence about the OIDC path.
**Release tag:** `v0.8.1-rc.1` → commit `828515b3` (annotated tag object `18ac0996`)

#### Deviation 1: the documented `make release` flow is dead — PR flow substituted

`make release`'s `git push origin HEAD` to `main` is blocked by the "Protect main
branch" ruleset (PR-only, zero bypass actors; `GH013: Changes must be made through
a pull request`). That flow last worked at v0.5.1 (2026-06-04); local-only tags
`v0.7.0`/`v0.7.1` corroborate that pushes stopped reaching the remote after the
ruleset landed. The release was therefore decomposed into the same steps in
rules-compliant order:

1. `cargo release version 0.8.1-rc.1 --execute --no-confirm --workspace` +
   changelog finalization + OpenAPI baseline regeneration (`make openapi` — the
   baseline embeds the crate version; `make release` does not automate this and
   `release-check` fails without it) on branch `chore/release-0.8.1-rc.1`.
2. `make release-check` locally: passed end-to-end (exit 0).
3. PR #36 → merged to `main` as `828515b3`.
4. Tag `v0.8.1-rc.1` created on `828515b3` and pushed directly — permitted by the
   "Protect release tags" ruleset's repository-admin bypass.

Updating `Makefile`/operator docs to match the ruleset reality is 19-05 territory.

#### Deviation 2: three workflow runs, one partial publish, one packaging defect

| Run | Trigger | Outcome |
|---|---|---|
| [32985190305](https://github.com/DF3NDR/paladin-dev-env/actions/runs/32985190305) | tag push | `startup_failure`, 0 jobs — GitHub Actions major outage (githubstatus.com incident, confirmed; unrelated workflows failed identically) |
| [32996942576](https://github.com/DF3NDR/paladin-dev-env/actions/runs/32996942576) | `workflow_dispatch tag=v0.8.1-rc.1` after recovery | `verify-tag-source` ✓, tests ✓; `publish-crates` published **10 of 11** in the committed order, then failed on `paladin-ai` |
| [32998197346](https://github.com/DF3NDR/paladin-dev-env/actions/runs/32998197346) | re-dispatch | ten already-published crates correctly tolerance-skipped (`already exists` branch — the re-runnability truth held); `paladin-ai` failed again: **`413 Payload Too Large: max upload size is 10485760`** |
| [33009214745](https://github.com/DF3NDR/paladin-dev-env/actions/runs/33009214745) | re-dispatch after fix | `publish-crates` **success** — `paladin-ai` published |

The `413` was deterministic, not transient: `paladin-ai`'s package root is the
repository root and the root `[package]` had no `include` list, so `cargo package`
bundled 2,425 files (`docs/` 32MB, `.planning/` 19MB, `.claude/` 9.6MB) — over
crates.io's 10 MiB cap. A local `cargo publish --dry-run` had NOT caught this:
dry-run aborts before the upload where the server enforces size. Fix: `include`
allowlist in the root `Cargo.toml` (PR #37, merged as `a5f27791`) — 442 files,
3.9 MiB raw, 799.7 KiB compressed, package verification build green. The ten
`crates/*` packages were never affected (their package roots are their own
subdirectories).

**Content note, stated plainly:** the published `paladin-ai-0.8.1-rc.1.crate` was
built from `main` at `a5f27791` (dispatch-mode checkout uses `main`), i.e. the tag
content **plus** the packaging-manifest fix. The ten other crates were built from
`828515b3`, byte-identical to the tag. The tag was not moved; this difference is
one `Cargo.toml` `include` list and nothing else.

The `create-release` job (`actions/create-release@v1`) fails on re-dispatch when
the GitHub release already exists; the stale release object was deleted (tag
preserved) before each re-dispatch. A re-runnable release workflow should
tolerate an existing release — 19-05/Phase 20 material, recorded here.

Separately: all four Build Binaries matrix jobs failed in every run — systematic,
still undiagnosed at this writing, and not on the publish path (`publish-crates`
needs only `test` + `create-release`). Tracked as an open item below.

#### Registry verification (all eleven crates)

Measured 2026-08-26 (post-publish), one call per crate:

```
curl -sf -H 'User-Agent: paladin-release-check (github.com/DF3NDR/paladin-dev-env)' \
  https://crates.io/api/v1/crates/<name>/0.8.1-rc.1
```

| crates.io package | HTTP | `trustpub_data` | published (UTC) |
|---|---|---|---|
| `paladin-ai-core` | 200 | `null` | 2026-08-26T18:01:22 |
| `paladin-ports` | 200 | `null` | 2026-08-26T18:01:50 |
| `paladin-herald` | 200 | `null` | 2026-08-26T18:02:13 |
| `paladin-battalion` | 200 | `null` | 2026-08-26T18:02:43 |
| `paladin-llm` | 200 | `null` | 2026-08-26T18:03:17 |
| `paladin-memory` | 200 | `null` | 2026-08-26T18:03:42 |
| `paladin-web` | 200 | `null` | 2026-08-26T18:04:25 |
| `paladin-notifications` | 200 | `null` | 2026-08-26T18:04:48 |
| `paladin-content` | 200 | `null` | 2026-08-26T18:05:19 |
| `paladin-storage` | 200 | `null` | 2026-08-26T18:05:42 |
| `paladin-ai` | 200 | `null` | 2026-08-26T20:25:02 |

`paladin-herald` exists on crates.io for the first time (crate-level endpoint now
HTTP 200, was 404 pre-bootstrap) — the precondition for creating its Trusted
Publishing configuration in 19-03 is met. `trustpub_data` is `null` on all eleven
versions: every one was token-published. That is what makes a non-null
`trustpub_data` in 19-03's proof event mean something.

The timestamps also show the publish order held in practice: the first ten landed
in the committed array order at ~25-35s intervals (run 32996942576), `paladin-ai`
last (run 33009214745). `paladin-herald` published at position 3 with no
dependency-resolution error — the corrected insertion point (after
`paladin-ports`, before `paladin-ai`) is validated by the registry itself.

#### Open items from this bootstrap

- Build Binaries matrix (4 targets): failed in all runs, cause not yet diagnosed;
  does not gate crates publishing.
- `make release` + operator docs contradict the `main` ruleset (Deviation 1) —
  fold into 19-05.
- `actions/create-release@v1` is not re-run tolerant (archived action; also flagged
  deprecated by Node runtime warnings) — candidate for Phase 20 hardening.

## OIDC Proof Event (PUB-03)

*Filled by plan 19-03.*

### Environment Posture

**Date:** 2026-08-27
**Created via:** `gh api` (both calls returned 2xx on the first attempt — no 403,
no human-UI fallback needed)

- **Name:** `crates-io`, created on `DF3NDR/paladin-dev-env`
  (`repos/DF3NDR/paladin-dev-env/environments/crates-io`). This is the literal
  string embedded in the OIDC subject claim
  `repo:DF3NDR/paladin-dev-env:environment:crates-io` that each of the eleven
  crates' Trusted Publishing configurations pins in 19-03 (D-06).
- **Deployment branch policy:** `custom_branch_policies: true`,
  `protected_branches: false`, with exactly one policy entry —
  `{"name": "v*.*.*", "type": "tag"}`. A branch push cannot reach this
  environment; only a ref matching `v*.*.*` typed as a tag can.
- **Reviewer gate:** deliberately absent (D-08). `protection_rules` holds only
  the one `branch_policy` type entry created above — no `required_reviewers`
  entry exists. Tag-push releases stay unattended; the ref restriction is the
  protection, not a human approval step. This is a reversible settings choice
  in both directions — enabling a reviewer gate later is a `gh api` call plus a
  one-line doc update in 19-05's trust table, not a workflow rewrite.
- **Secrets/variables:** none added. `environments/crates-io/secrets`
  `total_count` reads `0`. The environment exists to constrain identity via the
  OIDC subject claim, not to hold credentials — giving it a secret store would
  reintroduce the standing-token pattern this phase removes.

Live-state verification commands and results (2026-08-27):

```
$ gh api repos/DF3NDR/paladin-dev-env/environments --jq '[.environments[].name]'
["crates-io","github-pages"]

$ gh api repos/DF3NDR/paladin-dev-env/environments/crates-io --jq '.deployment_branch_policy'
{"protected_branches":false,"custom_branch_policies":true}

$ gh api repos/DF3NDR/paladin-dev-env/environments/crates-io/deployment-branch-policies --jq '[.branch_policies[]|{name,type}]'
[{"name":"v*.*.*","type":"tag"}]

$ gh api repos/DF3NDR/paladin-dev-env/environments/crates-io --jq '[.protection_rules[].type]'
["branch_policy"]

$ gh api repos/DF3NDR/paladin-dev-env/environments/crates-io/secrets --jq '.total_count'
0
```

### Human Confirmation (Task 3, 19-02 checkpoint)

**Date:** 2026-08-27
**Actor:** Am0rfu5 (repository owner). Checkpoint presented and approved via the GSD
orchestrator's `checkpoint:human-verify` gate.
**Resolution:** "approved" — all four verification points (environment deployment
rule, `environment: crates-io` + job-scoped `id-token: write` in `release.yml`, the
two-outcome mode step with the silent-skip branch deleted, and the D-08
reviewer-gate posture) stand as presented in the checkpoint.
**Reviewer-gate decision:** none — deliberate (D-08). No required-reviewer
protection rule is added to `crates-io`; unattended tag-push releases stay
working. This matches the live state (`protection_rules` holds only the
`branch_policy` entry, no `required_reviewers`) and is the plan's default
posture, not a deviation. Enabling one later remains a `gh api`/settings change
plus a doc-table update in 19-05.

### Trust Link Ledger

**Date:** 2026-08-27
**Actor:** Am0rfu5 (repository owner / crates.io crate-owner). Configurations created
manually through the crates.io web UI — no API or CLI exists for this step (D-13); the
executor presented the checkpoint instructions and paused, the human performed the
eleven UI visits, and the executor recorded this ledger on resume.

**Resolution as reported:** the human's resume signal was the bare word **"linked"** —
no count given, no crate named as unconfigurable. Per the checkpoint contract stated in
19-03-PLAN.md's `<resume-signal>` ("Type 'linked' with the count actually saved, or name
the crates that could not be configured"), a bare "linked" with nothing named as failed
is recorded as **all eleven saved, none reported unconfigurable**. This executor did not
independently re-open crate settings pages to verify — crates.io exposes no public API
for reading back a Trusted Publishing configuration, and settings pages require an
authenticated crate-owner session this agent does not hold. The re-verification named in
the plan's `<verification>` (reopening two or three pages including `paladin-ai-core` and
`paladin-herald`) is therefore a human-reported claim, not an executor-confirmed fact.
Task 2's proof release is the mechanism that will falsify any silently missing or
misconfigured link: a crate whose configuration was not actually saved, or was saved
with the wrong environment string, will fail at the `Authenticate with crates.io` step
or at that crate's publish step rather than passing silently.

Every configuration was instructed to carry the same four values:

| Field | Value |
|---|---|
| GitHub repository owner | `DF3NDR` |
| Repository name | `paladin-dev-env` |
| Workflow filename | `release.yml` |
| Environment | `crates-io` |

The instructions given at the checkpoint stated plainly that the environment field is
optional in the crates.io UI but must be populated on every configuration regardless
(D-06) — a blank environment field would let any workflow in this repository holding
`id-token: write` mint a publish token for that crate, defeating the pinned-environment
protection. The instructions also stated plainly that "Trusted Publishing Only"
enforcement mode must **not** be enabled on any crate in this task, so the old
`CARGO_REGISTRY_TOKEN` credential stays usable as fallback until 19-04's revocation.
Both instructions are recorded here as given; their execution rests on the human's
"linked" confirmation per the paragraph above, not on independent re-verification.

| Package name | Source directory | Workflow | Environment | Link date | Status |
|---|---|---|---|---|---|
| `paladin-ai-core` | `crates/paladin-core` | `release.yml` | `crates-io` | 2026-08-27 | linked (reported) |
| `paladin-ports` | `crates/paladin-ports` | `release.yml` | `crates-io` | 2026-08-27 | linked (reported) |
| `paladin-herald` | `crates/paladin-herald` | `release.yml` | `crates-io` | 2026-08-27 | linked (reported) |
| `paladin-battalion` | `crates/paladin-battalion` | `release.yml` | `crates-io` | 2026-08-27 | linked (reported) |
| `paladin-llm` | `crates/paladin-llm` | `release.yml` | `crates-io` | 2026-08-27 | linked (reported) |
| `paladin-memory` | `crates/paladin-memory` | `release.yml` | `crates-io` | 2026-08-27 | linked (reported) |
| `paladin-web` | `crates/paladin-web` | `release.yml` | `crates-io` | 2026-08-27 | linked (reported) |
| `paladin-notifications` | `crates/paladin-notifications` | `release.yml` | `crates-io` | 2026-08-27 | linked (reported) |
| `paladin-content` | `crates/paladin-content` | `release.yml` | `crates-io` | 2026-08-27 | linked (reported) |
| `paladin-storage` | `crates/paladin-storage` | `release.yml` | `crates-io` | 2026-08-27 | linked (reported) |
| `paladin-ai` | workspace root (`Cargo.toml`) | `release.yml` | `crates-io` | 2026-08-27 | linked (reported) |

Eleven data rows, matching the eleven-crate set reconciled in Task 1 of 19-01. No crate
was named by the human as unconfigurable, so no row is recorded as not-covered and no
interim auth path is stated for any crate. If Task 2's proof release later reveals a
silently missing or misconfigured link for any crate, that crate's status here will be
corrected and the discrepancy recorded rather than left standing.

### Proof Release Run (Task 2)

**Precondition resolution:** Task 2's stated precondition — "the 19-02 commit rewriting
`publish-crates` is contained in `origin/main`" — was unmet when this executor first
reached this task (`b12789a6` was not yet an ancestor of `origin/main`; the whole phase
sat on `chore/19-trusted-publishing`, unmerged). Resolved via option 1 (merge now):
`chore/19-trusted-publishing` → **PR #38** → merge commit `08aa4528` on `main`. Verified:
`git merge-base --is-ancestor b12789a6 origin/main` now succeeds.

**Version:** `0.8.1-rc.2`, bumped in lockstep across all eleven crates plus changelog and
OpenAPI baseline, on branch `chore/release-0.8.1-rc.2`, following the same PR-decomposed
flow Deviation 1 recorded for the `0.8.1-rc.1` bootstrap (the documented `make release`
direct-push flow is dead under the PR-only `main` ruleset).
**Release PR:** **PR #39**, merge commit `40990087`.
**Tag:** `v0.8.1-rc.2`, annotated, created on `40990087` and pushed.
**Actor:** Am0rfu5 (repository owner; PR merges and the tag push performed by Claude Code
operating with the owner's fine-grained PAT, at the owner's explicit delegation — same
posture as the `0.8.1-rc.1` bootstrap record).

**Trigger:** the tag push itself, **not** `workflow_dispatch` — this is what the plan's
Task 2 instructions specifically called for, to avoid resting the proof on RESEARCH.md
assumption A1 (`workflow_dispatch` eligibility for Trusted Publishing), which no source
confirms or denies.

**Run:** [33089177606](https://github.com/DF3NDR/paladin-dev-env/actions/runs/33089177606)
**Date:** 2026-08-27
**Event:** `push` (confirmed via `gh run view 33089177606 --json event` → `push`)
**Overall conclusion:** `failure` — but sourced entirely from the four pre-existing Build
Binaries matrix jobs (`ubuntu-latest`/`macos-latest` × two targets each), the same
undiagnosed, not-on-the-publish-path defect recorded in the bootstrap section's open
items. Every job on the actual publish path succeeded: `Verify Tag From Main`,
`Test Suite`, `Create Release`, and `Publish to crates.io` all report `conclusion:
success`. Judging this run by its overall conclusion rather than by the publish-path
jobs would misread a known, unrelated CI defect as a Trusted Publishing failure —
19-RESEARCH.md's Common Pitfall 3 is exactly this trap.

**`Publish to crates.io` job, step timing** (`gh run view 33089177606 --json jobs`):

| Step | Conclusion | Started | Completed |
|---|---|---|---|
| Determine publish mode | success | 15:48:39Z | 15:48:39Z |
| Authenticate with crates.io | success | 15:48:39Z | 15:48:40Z |
| Publish crates in dependency order | success | 15:48:40Z | 15:55:58Z |

**No repository secret was read.** `grep -c 'secrets.CARGO_REGISTRY_TOKEN'
.github/workflows/release.yml` returns `0` at this HEAD — the job has no path back to the
standing token at all; the credential was minted per-run from the eleven Trusted
Publishing configurations via `rust-lang/crates-io-auth-action@v1`.

**Auth-to-last-publish span:** the `Authenticate with crates.io` step completed at
`15:48:40Z`; the last crate's registry `created_at` (`paladin-ai`, published last in the
committed order) reads `15:55:35.670096Z` — a span of **~6m56s**. Measured against
crates.io's approximately 30-minute Trusted Publishing token lifetime (19-RESEARCH.md),
this run used roughly 23% of the token's life. That margin is comfortable for eleven
crates at this run's per-crate publish pace, but it is recorded as a **residual risk, not
a closed one** — T-19-19 in this plan's threat register accepts it explicitly rather than
restructuring the publish loop, and a slower run (larger crates, registry backpressure, a
future twelfth crate) could still approach the boundary. Re-running the job mints a fresh
token; loop restructuring is out of this phase's scope.

### Registry-Side Provenance (Task 3)

Per-crate query, `https://crates.io/api/v1/crates/<name>/0.8.1-rc.2`, `.version.trustpub_data`,
each independently re-queried by this executor (not taken solely from the workflow's
self-report):

| Package | Version | HTTP | `trustpub_data` verdict | Raw provenance value |
|---|---|---|---|---|
| `paladin-ai-core` | `0.8.1-rc.2` | 200 | **OIDC** | `{provider: github, repository: DF3NDR/paladin-dev-env, run_id: 33089177606, sha: 40990087...}` |
| `paladin-ports` | `0.8.1-rc.2` | 200 | **OIDC** | same provider/repository/run_id/sha |
| `paladin-herald` | `0.8.1-rc.2` | 200 | **OIDC** | same provider/repository/run_id/sha |
| `paladin-battalion` | `0.8.1-rc.2` | 200 | **OIDC** | same provider/repository/run_id/sha |
| `paladin-llm` | `0.8.1-rc.2` | 200 | **OIDC** | same provider/repository/run_id/sha |
| `paladin-memory` | `0.8.1-rc.2` | 200 | **OIDC** | same provider/repository/run_id/sha |
| `paladin-web` | `0.8.1-rc.2` | 200 | **OIDC** | same provider/repository/run_id/sha |
| `paladin-notifications` | `0.8.1-rc.2` | 200 | **OIDC** | same provider/repository/run_id/sha |
| `paladin-content` | `0.8.1-rc.2` | 200 | **OIDC** | same provider/repository/run_id/sha |
| `paladin-storage` | `0.8.1-rc.2` | 200 | **OIDC** | same provider/repository/run_id/sha |
| `paladin-ai` | `0.8.1-rc.2` | 200 | **OIDC** | same provider/repository/run_id/sha |

All eleven carry the identical, non-null provenance object:
`{"provider":"github","repository":"DF3NDR/paladin-dev-env","run_id":"33089177606","sha":"40990087ffe0795740d44c6718b00a5165f8c212"}` —
`run_id` matches the tag-push run above; `sha` matches release-PR merge commit `40990087`.

**The baseline contrast, stated explicitly rather than left implicit:** the same eleven
crates, same registry, eleven days apart:

| Version | Credential path | `trustpub_data` (all eleven) |
|---|---|---|
| `0.8.1-rc.1` (bootstrap, 2026-08-26) | standing `CARGO_REGISTRY_TOKEN` | `null` |
| `0.8.1-rc.2` (this proof, 2026-08-27) | OIDC exchange via `rust-lang/crates-io-auth-action@v1` | non-null, `provider: github` |

Same eleven crates, same registry, same publishing mechanism in the workflow file except
for the credential step — the only variable that changed between the two runs is whether
the credential came from a long-lived secret or a per-run OIDC mint, and the registry's
own provenance field tracks that difference precisely. This is the whole proof.

**Assumption A1 (`workflow_dispatch` eligibility) remains untested.** This proof used a
tag push, per Task 2's instructions, specifically to avoid depending on A1. Whether a
`workflow_dispatch`-triggered run can mint a Trusted Publishing token is still not
established by anything in this phase.

### What This Proof Does Not Establish

- **It does not establish that the old credential is gone.** `CARGO_REGISTRY_TOKEN` was
  not read by this run (verified: zero references in `release.yml` at this HEAD), but the
  secret itself has not been revoked or deleted from the repository — that is 19-04's
  job, and it has not started.
- **It does not establish that a `workflow_dispatch`-triggered run can mint a token.**
  This proof deliberately used a tag push to sidestep RESEARCH.md assumption A1, which no
  authoritative source confirms or denies. A1 stays untested.
- **It does not establish anything about dry runs.** `cargo publish --dry-run` mints no
  credential of any kind by design — a green dry run would be evidence about packaging
  and nothing else, and none of the evidence here is a dry run; every publish above is a
  real, non-dry-run upload with registry-observed provenance.
- **It does not establish the OIDC path's behavior under failure conditions** — an
  expired token mid-loop, a misconfigured Trust Publisher Configuration, or a revoked
  environment policy were not exercised. This run's Trust Link Ledger (Task 1) rests on
  the human's unverified "linked" report; had one of the eleven configurations actually
  been missing or misconfigured, this run's all-success outcome demonstrates that it was
  not, but the *mechanism* for what would happen if one were missing (a per-crate auth
  failure, not a whole-job failure) was not separately tested.

## Credential Revocation (PUB-04)

### Task 1 decision: the one-way door

**Decision:** `revoke-now`
**Date:** 2026-08-27
**Deciding actor:** Am0rfu5 (repository owner and crates.io crate owner), resolved via the
`checkpoint:decision` this plan's Task 1 presented. No concerns were named; the decision
proceeds on D-05 steps 3 and 4 exactly as this plan specifies.

**Evidence the decision rests on** (both conditions this task's `<context>` names as the
prerequisite for `revoke-now`, read directly from this file's `## OIDC Proof Event (PUB-03)`
section rather than re-derived):

- **Eleven Trust Publisher Configurations, populated `environment` field.** The Trust Link
  Ledger above records all eleven crates as `linked (reported)`, each carrying the same
  `crates-io` environment value, with none named by the human operator as unconfigurable.
- **Registry-side, non-dry-run, non-self-reported provenance.** The Registry-Side Provenance
  (Task 3) table above shows all eleven `0.8.1-rc.2` versions carrying the identical non-null
  `trustpub_data` object (`provider: github`, `repository: DF3NDR/paladin-dev-env`,
  `run_id: 33089177606`, `sha: 40990087...`), against the `0.8.1-rc.1` baseline where the same
  eleven crates read `trustpub_data: null` under the standing token. Both conditions hold, so
  per this task's own framing the answer is `revoke-now` rather than `hold`.

**Accepted limits.** The proof's stated boundaries were presented alongside the decision and
accepted as residual, not blocking:

- **`workflow_dispatch` eligibility (assumption A1) remains untested under OIDC.** The proof
  release used a tag push deliberately, to avoid depending on A1; whether a
  `workflow_dispatch`-triggered run can mint a Trusted Publishing token is still not
  established by anything in this phase.
- **Dry-run behavior under OIDC is not established.** `cargo publish --dry-run` mints no
  credential of any kind by design, so nothing in this phase's evidence speaks to dry-run
  behavior specifically — every publish recorded above is a real, non-dry-run upload.
- **No instant fallback after revocation.** Once the crates.io token is revoked, the OIDC path
  is the only publish path; recovery from an OIDC failure after this point means minting a new
  token by hand and temporarily reverting the workflow (T-19-24, accepted in this plan's threat
  register), not falling back to a live standing credential.

Per this task's own acceptance criteria, this record cites the eleven-row provenance table
above as the evidence the decision rests on, and Tasks 2 and 3 now proceed.
