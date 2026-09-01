---
phase: 19-crates-io-trusted-publishing-replace-the-long-lived-registry
verified: 2026-08-27T19:19:05Z
status: passed
score: 6/7 must-haves verified
behavior_unverified: 1
overrides_applied: 0
human_verification:

  - test: "Independently confirm at crates.io (Account Settings -> API Tokens, DF3NDR account) that the publish-scoped token named \"Paladin\" (the one that backed CARGO_REGISTRY_TOKEN) shows as revoked, and that no other live publish-scoped token exists on the account."
    expected: "The 'Paladin' token is listed as revoked (or absent), and no other publish-scoped token remains that could still authenticate a `cargo publish` for any paladin-* crate."
    why_human: "crates.io exposes no API or CLI to read back a token's existence, last-used timestamp, or revocation state (confirmed during this verification — there is no `gh`-equivalent for crates.io token introspection). The only record is the operator's bare attestation ('revoked', token name 'Paladin' in a follow-up) captured in 19-PUBLISH-EVIDENCE.md's Revocation Ledger, which itself states three requested fields (last-used timestamp, revocation timestamp, other-token sweep) as 'not reported by operator.' This is a plan-declared `verification: backstop` truth (19-04-PLAN.md) — no codebase or registry-API evidence can close it; only a human with crates.io account access can confirm the load-bearing half of PUB-04 firsthand."
---

# Phase 19: crates.io Trusted Publishing — Replace the Long-Lived Registry Token Verification Report

**Phase Goal:** The ability to publish `paladin-*` to crates.io stops living in a long-lived
repository secret. crates.io Trusted Publishing replaces `CARGO_REGISTRY_TOKEN` with a token
minted per run from GitHub OIDC. The phase is not finished when the OIDC path works; it is
finished when the old secret no longer exists.
**Verified:** 2026-08-27T19:19:05Z
**Status:** human_needed
**Re-verification:** No — initial verification

## Goal Achievement

All verification below was performed independently against the live codebase (branch
`chore/19-trusted-publishing`, working tree clean), the live GitHub repository/environment API,
and the live crates.io registry API — not read from SUMMARY.md or the evidence log's self-report,
though those documents' claims were cross-checked against this independent evidence and found to
match in every case except the one item routed to human verification below.

### Observable Truths

Truths are the seven ROADMAP.md Success Criteria for Phase 19 (the merged must-haves from the
five plans map onto these one-to-one; PUB-01..PUB-05 are cited per truth).

| # | Truth | Status | Evidence |
|---|-------|--------|----------|
| 1 | The publishable crate set is enumerated from `Cargo.toml` and reconciled with `release.yml`'s publish order before any trust link is created (PUB-01). | VERIFIED | `cargo metadata --no-deps` reports 11 publishable packages (`publish == null`); `.crate-names.txt` lists the same 11; `.github/workflows/release.yml`'s `CRATES` array (lines 434-446) lists the same 11 including `paladin-herald`, in a dependency-valid order (`paladin-ai-core` -> `paladin-ports` -> `paladin-herald` -> ... -> `paladin-ai`). `bash scripts/check-crate-names.sh` -> "11 publishable crate(s) checked, all match the allow-list exactly." `paladin-herald` crate-level endpoint independently re-queried: HTTP 200 (was 404 pre-bootstrap per the evidence log). |
| 2 | Publishing authenticates through an OIDC exchange rather than a stored secret; `id-token: write` is granted on the `publish-crates` job only (PUB-02). | VERIFIED | `.github/workflows/release.yml` line 407: `uses: rust-lang/crates-io-auth-action@v1`; line 382: `environment: crates-io`; lines 383-385: job-scoped `permissions: {contents: read, id-token: write}`; no `permissions:` key exists at workflow (top) level (grep confirms zero matches for `^permissions:`). `grep -rn "secrets.CARGO_REGISTRY_TOKEN" .github/workflows/` returns zero matches across all 7 workflow files. Live `gh api .../environments/crates-io` confirms the environment exists, restricted to `v*.*.*` tags typed as a tag rule, with `secrets.total_count: 0`. |
| 3 | The new path is proven to publish before the old credential is destroyed, and the proof is not a dry run (PUB-03). | VERIFIED | All eleven crates independently re-queried at `https://crates.io/api/v1/crates/<name>/0.8.1-rc.2`: every one carries non-null `trustpub_data` (`{"provider":"github","repository":"DF3NDR/paladin-dev-env","run_id":"33089177606","sha":"40990087..."}`), contrasted against the `0.8.1-rc.1` bootstrap baseline where the same eleven crates read `trustpub_data: null` under the standing token (also independently re-verified in the earlier bootstrap section of the evidence log). This is registry-side provenance, not a workflow self-report or a dry run. |
| 4 | The long-lived credential is revoked at crates.io **and** deleted from repository secrets, and both actions are recorded with a date and an actor (PUB-04). | ⚠️ PRESENT_BEHAVIOR_UNVERIFIED (secret-deletion half VERIFIED; crates.io-revocation half is a plan-declared backstop truth) | GitHub secret half independently confirmed live: `gh api repos/DF3NDR/paladin-dev-env/actions/secrets` -> `{"total_count":0,"secrets":[]}`; `grep -rl 'secrets.CARGO_REGISTRY_TOKEN' .github/workflows/` -> no files; the one remaining bare `CARGO_REGISTRY_TOKEN` occurrence (release.yml line 411) is the OIDC-output env var, not a secret expression. Commit timestamps independently confirm ratchet ordering: decision `5b5866ea` 18:06:28Z -> crates.io-revocation record `39574004` 18:16:46Z -> secret-deletion+sweep `bb818e88` 18:22:36Z. **The crates.io-side revocation itself cannot be verified from this side**: crates.io exposes no API or CLI to read back a token's existence or revocation state, so there is no registry-side check equivalent to Truth 3's `trustpub_data` query. `19-04-PLAN.md`'s own frontmatter marks this exact truth `verification: backstop` for that reason. The evidence log records the operator's confirmation as the bare word "revoked" (token name "Paladin" in a follow-up), with three requested corroborating fields — last-used timestamp, revocation timestamp, a sweep for a second forgotten token (T-19-21) — explicitly recorded as "not reported by operator" rather than invented. Routed to human verification below. |
| 5 | No release can silently skip publishing while reporting success (PUB-05). | VERIFIED | `grep -c "dry_run=skip"` and `grep -c "continue-on-error"` on `release.yml`: both zero. The `Determine publish mode` step (lines 391-399) produces exactly `dry_run=true` or `dry_run=false` — no third branch. The publish loop runs under `set -euo pipefail` and its only tolerance branch is the pre-existing already-published regex match, which still `exit 1`s on any other failure. |
| 6 | The per-crate trust configuration is documented (PUB-05, PUB-01). | VERIFIED | `docs/src/appendix/release-automation.md` `### Per-Crate Trust Configuration` (lines 194-224) carries an 11-row table (crate name, source directory, workflow filename, environment name, link date, status), with both divergent-name rows (`paladin-ai-core`/`crates/paladin-core`, `paladin-ai`/workspace root) on their own rows. `### Credential History` (lines 226-240) carries all 4 events (bootstrap publish, OIDC proof, crates.io revocation, secret deletion) each with a date, actor and evidence pointer. `docs/src/appendix/release-checklist.md` and `docs/src/contributing/development-setup.md` independently confirmed reconciled to the same 11-crate order and Trusted Publishing credential path (no residual "Required secret" language; `grep` confirms `CARGO_REGISTRY_TOKEN` absent from both). |
| 7 | Nothing in this phase claims a protection it did not establish (PUB-05, PUB-01). | VERIFIED | The trust table's "linked (reported)" status column, and the accompanying prose in both `release-automation.md` and `19-PUBLISH-EVIDENCE.md`, state plainly that the Trust Link Ledger rests on the human operator's unverified report (crates.io exposes no read-back API), not on independent re-verification, and name the proof release as the corroborating (not conclusive) mechanism. No crate is silently folded into a "protected" count without qualification; no document claims workflow_dispatch eligibility, dry-run-under-OIDC behavior, or a second-token sweep that was not actually established. |

**Score:** 6/7 truths verified (1 present, behavior-unverified — the crates.io-side token revocation, a plan-declared backstop truth no codebase or registry-API evidence can close)

### Required Artifacts

| Artifact | Expected | Status | Details |
|----------|----------|--------|---------|
| `.github/workflows/release.yml` | Eleven-crate dependency-first publish order, OIDC auth step, job-scoped `id-token: write`, `crates-io` environment, no silent-skip branch | VERIFIED | All elements present and wired; confirmed by direct grep/read (see Truths 1, 2, 5). |
| `.planning/phases/19-.../19-PUBLISH-EVIDENCE.md` | Crate-set reconciliation, bootstrap run, OIDC proof, revocation record | VERIFIED | Contains `## Crate-Set Reconciliation`, `## OIDC Proof Event (PUB-03)`, `## Credential Revocation (PUB-04)`; every claim in it that is independently checkable (registry state, secret state, environment state, commit timestamps) matched live evidence. |
| `docs/src/appendix/release-automation.md` | `## Trusted Publishing` section, per-crate trust table, credential history, dry-run boundary | VERIFIED | Section present at line 171 with all named subsections (`### Environment and Protection Posture`, `### Per-Crate Trust Configuration`, `### Credential History`, `### Dry-Run Claim Boundary`, `### Break-Glass Recovery`, `### Known Limits`). |
| `docs/src/appendix/release-checklist.md` | Reconciled to eleven crates, contains `paladin-herald` | VERIFIED | `paladin-herald` present in the dry-run order sections; no stale "ten crate" or missing-secret language found. |
| `docs/src/contributing/development-setup.md` | Points at Trusted Publishing instead of a repository secret | VERIFIED | `### Publish credential` section (line 662) replaces the old `### Required secret` block; no residual dead single-push release flow (a Rule-1 fix in 19-05 removed a duplicate copy). |
| `CHANGELOG.md` | `### Security` entry recording the migration | VERIFIED | Present under `## [Unreleased]`, names Trusted Publishing, the `crates-io` environment, and `paladin-herald`. |
| `.crate-names.txt` / `Cargo.toml` manifests | Eleven-crate owned-name set | VERIFIED | `bash scripts/check-crate-names.sh` passes; independently cross-checked against `cargo metadata`. |

### Key Link Verification

| From | To | Via | Status | Details |
|------|-----|-----|--------|---------|
| `.github/workflows/release.yml` | `Cargo.toml` (workspace manifests) | `CRATES` array is a topological order of the publishable package set `cargo metadata` reports | WIRED | Independently confirmed identical 11-name sets, and topological validity of the committed order (herald after `paladin-ports`, before `paladin-ai`). |
| `.github/workflows/release.yml` | `.crate-names.txt` | Both enumerate the eleven owned crates.io names; `scripts/check-crate-names.sh` gates the allow-list against manifests | WIRED | Script passes; sets match. |
| `.github/workflows/release.yml` (auth step) | `.github/workflows/release.yml` (publish step) | `CARGO_REGISTRY_TOKEN` env var sourced from `steps.auth.outputs.token` | WIRED | Line 411 confirmed. |
| `.github/workflows/release.yml`'s `environment:` key | Each crate's crates.io Trust Publisher Configuration | Environment name embedded in OIDC subject claim must match the crates.io config's environment field | WIRED (registry-proven) | The proof run (33089177606) succeeded end-to-end and produced non-null `trustpub_data` on all eleven crates — a mismatch here would have failed the mint or the publish, not passed silently. |
| `docs/src/appendix/release-automation.md` | `.planning/phases/19-.../19-PUBLISH-EVIDENCE.md` | Credential-history rows are copied from and cite the evidence ledger | WIRED | Confirmed by direct read — both tables carry matching dates/actors/run IDs and the doc cites the evidence file path. |
| `docs/src/appendix/release-automation.md`'s trust table | `.github/workflows/release.yml`'s `environment:` key | Workflow-filename/environment-name columns must match the job's actual values | WIRED | Table says `release.yml` / `crates-io` on every row; matches the live job. |

### Requirements Coverage

| Requirement | Source Plan | Description | Status | Evidence |
|-------------|-------------|--------------|--------|----------|
| PUB-01 | 19-01, 19-05 | Publishable crate set enumerated from manifests and reconciled with `release.yml` before any trust link | SATISFIED | Truth 1; `.crate-names.txt`, `CRATES` array, `cargo metadata` all agree at 11; documented in release-automation.md's Canonical Publish Order. |
| PUB-02 | 19-02 | Publishing authenticates via OIDC exchange, job-scoped `id-token: write` | SATISFIED | Truth 2; live `gh api` environment check + workflow grep. |
| PUB-03 | 19-03 | New path proven to publish (non-dry-run) before old credential destroyed | SATISFIED | Truth 3; independent registry re-query of all 11 crates' `trustpub_data`. |
| PUB-04 | 19-04, 19-05 | Long-lived credential revoked at crates.io AND deleted from repo secrets, both recorded with date/actor | SATISFIED for the recording obligation and the GitHub-secret half; the crates.io-revocation half rests on operator attestation only, per the plan's own `verification: backstop` designation — see Truth 4 and Human Verification below | GitHub secret absence independently confirmed; revocation ledger present with date/actor; crates.io-side state not independently checkable by any tool available to this verifier. |
| PUB-05 | 19-02, 19-05 | No release silently skips publishing while reporting success; per-crate trust config documented | SATISFIED | Truths 5, 6; silent-skip branch absent, trust table present and complete. |

No orphaned requirements: all five plans' `requirements:` frontmatter fields together cover PUB-01 through PUB-05 exactly, matching `.planning/REQUIREMENTS.md`'s Phase 19 section and its traceability table (all five rows already marked Complete, consistent with this verification's findings).

### Anti-Patterns Found

None. Scanned `.github/workflows/release.yml` and all five modified docs files for `TBD`/`FIXME`/`XXX`/`TODO`/`HACK`/`PLACEHOLDER`/"not yet implemented"/empty-return patterns: zero matches (the one `RUSTSEC-XXXX-XXXX` hit in `development-setup.md` is a pre-existing documentation template example, not a debt marker, and is outside this phase's edited sections). `cargo fmt --check` passes. `bash scripts/check-doc-config.sh`, `bash scripts/check-changelogs.sh`, and `bash scripts/check-workflow-triggers.sh` all pass cleanly against the current tree.

### Behavioral Spot-Checks

| Behavior | Command | Result | Status |
|----------|---------|--------|--------|
| No workflow references the old secret | `grep -rn "secrets.CARGO_REGISTRY_TOKEN" .github/workflows/` | no matches | PASS |
| GitHub repository secret is gone | `gh api repos/DF3NDR/paladin-dev-env/actions/secrets` | `{"total_count":0,"secrets":[]}` | PASS |
| `crates-io` environment restricted to release tags only | `gh api .../environments/crates-io/deployment-branch-policies` | `[{"name":"v*.*.*","type":"tag"}]` | PASS |
| `crates-io` environment holds no secrets | `gh api .../environments/crates-io/secrets --jq .total_count` | `0` | PASS |
| All eleven `0.8.1-rc.2` crate versions carry OIDC provenance | per-crate `curl` to crates.io API, checked `trustpub_data` | non-null on all 11, identical provider/run_id/sha | PASS |
| `paladin-herald` exists on crates.io | `curl` crate-level endpoint | HTTP 200 | PASS |
| Crate-name allow-list matches manifests | `bash scripts/check-crate-names.sh` | "11 publishable crate(s) checked, all match" | PASS |
| Workflow trigger-surface policy intact | `bash scripts/check-workflow-triggers.sh` | "7 workflow file(s) scanned ... all pass" | PASS |

### Probe Execution

Not applicable — this phase has no `scripts/*/tests/probe-*.sh` and neither PLAN nor SUMMARY declares one. Skipped.

### Human Verification Required

### 1. Confirm crates.io-side token revocation independently

**Test:** Log into the crates.io account that owns the `paladin-*` crates (Account Settings ->
API Tokens) and confirm the token named "Paladin" is revoked/absent, and that no other
publish-scoped token exists on the account.
**Expected:** The "Paladin" token shows as revoked (or is gone from the active token list), and no
other live publish-scoped token remains.
**Why human:** crates.io provides no API or CLI to read back a token's existence, last-used time,
or revocation state — this verifier confirmed that gap directly (there is no equivalent to the
`trustpub_data` registry check used for Truth 3). `19-04-PLAN.md` itself marks this exact truth
`verification: backstop` for that reason, and the evidence log records the operator's confirmation
as a bare, minimally-detailed attestation ("revoked" / "Paladin"), explicitly leaving three
requested corroborating fields — last-used timestamp, revocation timestamp, a sweep for a second
forgotten token (open item T-19-21) — as "not reported by operator." This is not a defect in the
phase's execution (the phase did everything a non-account-holding agent can do, and recorded the
gap honestly rather than papering over it); it is a structural limit of the registry that only a
human with account access can close.

### Gaps Summary

No gaps. Every artifact, key link, and codebase-checkable truth verified cleanly against live
state (GitHub API, crates.io registry API, and the workspace tree), independent of SUMMARY.md's
claims — and in every case the SUMMARY/evidence-log claims matched what was independently
observed. The sole open item is not a phase defect: it is the load-bearing half of PUB-04 (the
crates.io token revocation) resting on human attestation because crates.io's registry has no API
to prove a token's revocation state, exactly as the plan itself anticipated by marking that one
truth `verification: backstop`. The phase's own artifacts are honest about this limit rather than
concealing it — the Revocation Ledger explicitly marks the unreported fields as such instead of
inventing them, and names the still-open T-19-21 risk (a possible second forgotten token) rather
than silently closing it.

Two additional items are named here for completeness, not as gaps (already flagged as accepted,
out-of-scope, or deferred by the phase's own evidence and not disputed by this verification):

- The Build Binaries matrix (4 targets) fails systematically on every release run; pre-existing,
  undiagnosed, and does not gate `publish-crates` (`needs: [test, create-release]` only).

- `workflow_dispatch` eligibility for Trusted Publishing (RESEARCH.md assumption A1) remains
  untested; the proof deliberately used a tag push to avoid depending on it, which is sufficient
  to satisfy PUB-03's "real publish, not a dry run" requirement.

Separately, worth noting for ship-readiness rather than as a phase gap: `.github/workflows/release.yml`
itself (the 19-01/19-02 functional change) is already merged to `origin/main` via PR #38, but the
phase's remaining commits (19-03 evidence, 19-04 revocation record, 19-05 documentation, and the
`REQUIREMENTS.md`/`ROADMAP.md` completion marks) still sit on `chore/19-trusted-publishing` and
have not yet been merged to `main`. This verification was performed against that branch (the
current checkout) as instructed, and confirmed the working tree is clean and up to date with its
own remote — but the documentation and evidence-log half of the phase is not yet visible on `main`
until this branch ships.

---

_Verified: 2026-08-27T19:19:05Z_
_Verifier: Claude (gsd-verifier)_
