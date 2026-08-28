---
phase: 19
slug: crates-io-trusted-publishing-replace-the-long-lived-registry
status: secured
threats_open: 0
asvs_level: 1
created: 2026-08-28
---

# Phase 19 — Security

> Per-phase security contract: threat register, accepted risks, and audit trail.
> Audit performed 2026-08-28 by gsd-security-auditor against live state (crates.io API,
> GitHub API, gate scripts re-run) — not document trust. Full per-threat evidence is in
> the audit return recorded below; the registry source is each PLAN.md's `<threat_model>`.

---

## Trust Boundaries

| Boundary | Description | Data Crossing |
|----------|-------------|---------------|
| workspace manifests → release.yml publish order | Hand-maintained CRATES array restating what cargo metadata computes | Publish set/order (integrity-critical) |
| CI job → crates.io registry | Per-run OIDC-minted token crosses to a public, append-only registry | Ephemeral publish credential |
| GitHub OIDC → crates.io Trusted Publishing | Identity claims (repo, workflow, environment) exchanged for a ~30-min token | Workflow identity claims |
| Planning documents → executor edits | Docs proposed an insertion point the manifests contradicted | Publish-order instructions |
| Operator attestation → evidence log | crates.io token state has no read-back API; ledger rows rest on human report | Credential lifecycle facts |

---

## Threat Register

All 36 threats (T-19-01 … T-19-31 plus five plan-scoped T-19-SC supply-chain rows) are
**closed**. Register rows, severities, dispositions and mitigation plans live in each
plan's `<threat_model>` table (19-01 … 19-05-PLAN.md); the 2026-08-28 audit verified
every mitigation in the implementation or evidence, re-querying live state where an API
exists. Highlights:

| Threat ID | Component | Severity | Disposition | Status |
|-----------|-----------|----------|-------------|--------|
| T-19-01/02 | Publish-set reconciliation gates | high | mitigate | closed (re-ran gates: RECONCILED 11, allow-list byte-unchanged) |
| T-19-07/08 | crates-io environment tag policy | high | mitigate | closed (live gh api: v*.*.* tag rule, env exists, 0 secrets) |
| T-19-09/10/12 | release.yml job scoping/triggers | high | mitigate | closed (no dry_run=skip, no top-level permissions, triggers 7/7) |
| T-19-14/15 | Trust configs + OIDC proof | high | mitigate | closed (trustpub_data non-null ×11 re-queried; docs.yml disjoint) |
| T-19-16/20 | D-05 ratchet ordering | high | mitigate | closed (dependency + ledger ordering verified) |
| T-19-21/23 | Registry-side revocation + second-token sweep | high | mitigate | closed (19-UAT.md test 1 passed 2026-08-28: token "Paladin" absent, none other) |
| T-19-22 | Workflow-directory secret sweep | high | mitigate | closed (single hit = OIDC output env var, not a secret read) |
| T-19-25/30 | Advisory-register integrity | medium | mitigate | closed (SECURITY-EXCEPTIONS.md byte-unchanged) |
| T-19-26..29/31 | Documentation truthfulness | high/med | mitigate | closed (stale-secret headings absent; attestation-honest tables) |
| T-19-SC ×5 | Supply chain per plan | high | mitigate | closed (sole third-party artifact: rust-lang/crates-io-auth-action@v1, audited) |

*Only open threats at or above workflow.security_block_on (`high`) count toward threats_open.*

---

## Accepted Risks Log

| Risk ID | Threat Ref | Rationale | Accepted By | Date |
|---------|------------|-----------|-------------|------|
| AR-19-1 | T-19-06 | Prerelease of already-public source discloses nothing new; prereleases never win default resolution | plan 19-01 (executed) | 2026-08-26 |
| AR-19-2 | T-19-13 | Single OIDC mint covers the 11-crate loop (~7 min vs ~30-min lifetime); named in release.yml job comment | plan 19-02 (executed) | 2026-08-27 |
| AR-19-3 | T-19-19 | Measured auth-to-last-publish ~6m56s (~23% of lifetime); re-run mints fresh token; loop restructuring is Phase 20 | plan 19-03 (executed) | 2026-08-27 |
| AR-19-4 | T-19-24 | OIDC-failure-after-revocation accepted with documented break-glass path (mint new token + temporary workflow revert) in release-automation.md | Am0rfu5 (revoke-now decision) | 2026-08-27 |

---

## Security Audit Trail

| Audit Date | Threats Total | Closed | Open | Run By |
|------------|---------------|--------|------|--------|
| 2026-08-28 | 36 | 36 | 0 | gsd-security-auditor (sonnet), verify-mitigations mode, ASVS L1 |

Audit-adjacent finding (not a register gap): pre-existing `workflow_dispatch` tag-input
shell injection (CR-01, predates Phase 19) was found by this phase's code review and fully
remediated in commits 71e7b733..ad0d3587, together with WR-01..WR-05 hardening.

Operational note from the audit: the review-fix commits live on `chore/19-trusted-publishing`
and take effect on real release runs only once the phase's finalization PR merges to `main`.

---

## Sign-Off

- [x] All threats have a disposition (mitigate / accept / transfer)
- [x] Accepted risks documented in Accepted Risks Log
- [x] `threats_open: 0` confirmed
