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

## Credential Revocation (PUB-04)

*Filled by plan 19-04.*
