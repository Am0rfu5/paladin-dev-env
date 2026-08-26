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

## OIDC Proof Event (PUB-03)

*Filled by plan 19-03.*

## Credential Revocation (PUB-04)

*Filled by plan 19-04.*
