---
phase: 21
slug: release-artifacts-curated-release-notes-and-attached-distrib
status: verified
# threats_open = count of OPEN threats at or above workflow.security_block_on severity (the blocking gate)
threats_open: 0
asvs_level: 1
created: 2026-09-01
---

# Phase 21 — Security

> Per-phase security contract: threat register, accepted risks, and audit trail.
> Register authored at plan time across all six PLAN.md `<threat_model>` blocks; classified at L1
> (grep-level) per the short-circuit rule, corroborated by plan acceptance greps at execution, the
> independent verifier pass, the post-rehearsal code-review fix cycle (21-REVIEW-FIX.md — the one
> live CR-01 violation found was fixed as WR-01), and the green v0.8.1-rc.5 rehearsal
> (run 33436573814).

---

## Trust Boundaries

| Boundary | Description | Data Crossing |
|----------|-------------|---------------|
| CHANGELOG.md / tag input → release body | Author-written and tag-derived text reaching the GitHub API | untrusted text (CR-01 class) |
| workflow → GitHub Releases API (`gh`) | Release create/edit/upload/download with `GITHUB_TOKEN` | credential-bearing API calls |
| workflow → ghcr.io | Multi-arch image push; digest read-back | credential-bearing push, content digests |
| operator → `origin` (tag push) | A push that triggers a publishing pipeline with registry write access | release trigger |
| release workflow → crates.io | OIDC-minted short-lived publish token (Phase 19 controls, unchanged) | publish credential |
| downloaded release assets → consumer/operator | Artifacts fetched from the internet and executed during verification | binaries, checksums |

---

## Threat Register

| Threat ID | Category | Component | Severity | Disposition | Mitigation | Status |
|-----------|----------|-----------|----------|-------------|------------|--------|
| T-21-01 | Tampering | `create-release` extract step | high | mitigate | CR-01: version via `env:`, section via file; no inline `${{ }}` in `run:` (verified grep) | closed |
| T-21-02 | Tampering | extraction output → release body | high | mitigate | Regex metacharacters escaped; exact-heading anchoring shared with gate Clause 2; 16-assertion harness | closed |
| T-21-03 | Information Disclosure | `gh api` calls in `create-or-reuse-release.sh` | medium | accept | See Accepted Risks R-21-03 | closed |
| T-21-04 | Denial of Service | release body size | low | accept | See Accepted Risks R-21-04 | closed |
| T-21-05 | Tampering | `Package release binaries` step | high | mitigate | Logic in `package-release-binaries.sh` (shellcheck-clean, `*_LIB_ONLY` seam); tainted values via `env:` | closed |
| T-21-06 | Spoofing | published binary archives | medium | mitigate | Per-asset `.sha256` + aggregated `SHA256SUMS`; verification instructions in the body | closed |
| T-21-07 | Elevation of Privilege | `build-binaries`/`sbom` with `contents: write` | high | mitigate | WR-02: `dtolnay/rust-toolchain` pinned to commit SHA (verified grep); no new marketplace actions — `gh` CLI only | closed |
| T-21-08 | Repudiation | a leg producing no artifact ending green | high | mitigate | Existence assert per target before tar (`expected_binaries_for_target`); missing binary fails the leg; proven by rehearsal | closed |
| T-21-09 | Information Disclosure | `gh release upload` following a credential-bearing redirect | medium | mitigate | No `-L`/`--location` anywhere in release scripts (verified grep = 0) | closed |
| T-21-10 | Tampering | `finalize-release-body` step | high | mitigate | CR-01 discipline; body via `--notes-file`, never inline | closed |
| T-21-11 | Tampering | truncate-and-rebuild over maintainer-editable body | medium | mitigate | Literal-string marker truncation (no regex interpretation of body text); 84-assertion harness | closed |
| T-21-12 | Denial of Service | body growth across re-runs | high | mitigate | Marker-based idempotent rebuild — re-run converges, never appends (harness case + rehearsal re-run semantics) | closed |
| T-21-13 | Spoofing | consumer pinning to a mutable tag | medium | mitigate | Body pins `docker pull …@sha256:…`; digest from `build-push-action`'s own output; `:latest` instruction deleted | closed |
| T-21-14 | Repudiation | a failed artifact leg presented as complete | high | mitigate | Finalize job writes sections only from succeeded legs' outputs; failed leg's section omitted (harness case) | closed |
| T-21-15 | Information Disclosure | `gh` calls in the finalize job | medium | mitigate | Same no-redirect + `env:` discipline; no token echo (verified grep = 0) | closed |
| T-21-16 | Tampering | asset substitution between build and consumer | high | mitigate | `SHA256SUMS` generated from actually-attached assets; one-command verification (`sha256sum -c`) proven in UAT | closed |
| T-21-17 | Spoofing | instructions naming a file the run did not produce | high | mitigate | Instructions emitted only when the sums file exists; asset list from real uploads (ARTIFACT-03) | closed |
| T-21-18 | Tampering | asset names rendered into markdown | medium | mitigate | Names sourced from `gh release view` JSON, rendered as code spans; body assembled via file | closed |
| T-21-19 | Information Disclosure | `gh release download`/`upload` redirects | medium | mitigate | Same no-redirect control (grep = 0); GitHub asset redirects handled internally by `gh` without leaking the token header cross-host | closed |
| T-21-20 | Repudiation | SBOM presented as covering more than it does | medium | mitigate | Body identifies the CycloneDX SBOM as covering the root `paladin-ai` package only (ARTIFACT-05) | closed |
| T-21-21 | Denial of Service | unbounded asset download in finalize job | low | accept | See Accepted Risks R-21-21 | closed |
| T-21-22 | Repudiation | signing/provenance posture stated ambiguously | high | mitigate | Explicit deferral with reasoning in `release-automation.md`, naming `actions/attest-build-provenance` (verified grep in 21-05) | closed |
| T-21-23 | Spoofing | documented verification commands not matching the pipeline | medium | mitigate | Docs quote commands verbatim from `finalize-release-body.sh`; `make check-doc-config` green | closed |
| T-21-24 | Tampering | documentation drift from the trigger policy | medium | mitigate | `scripts/check-workflow-triggers.sh` green; `branching-model.md` untouched (asserted in 21-05) | closed |
| T-21-25 | Elevation of Privilege | rehearsal tag triggering `publish-crates` | high | mitigate | Phase 19 controls unchanged (crates-io environment, `v*.*.*`-only, OIDC short-lived token); `verify-tag-source` ancestor-of-main check; blocking human checkpoint authorized the version consumption (option-a, 2026-08-31) | closed |
| T-21-26 | Repudiation | a rehearsal reported as run when it was not | high | mitigate | Evidence file has exactly two permitted shapes; the run branch is populated with run URLs and measured outputs (21-ARTIFACT-EVIDENCE.md) | closed |
| T-21-27 | Tampering | executing downloaded binaries during verification | medium | mitigate | Checksums verified before execution; `--help`/`--version` read-only invocations; no elevated privileges (UAT test 2 followed this) | closed |
| T-21-28 | Denial of Service | a failed rehearsal leg tempting a re-tag | medium | mitigate | Fix-forward + same-tag re-run policy in plan and runbook; no re-tag occurred (rehearsal green first run) | closed |
| T-21-29 | Information Disclosure | run logs/evidence capturing token material | medium | mitigate | Evidence transcribes outcomes/digests/sizes only; no token echo (verified grep = 0); evidence reviewed before commit | closed |
| T-21-SC | Tampering | package installs | low | accept | See Accepted Risks R-21-SC | closed |

*Status: open · closed · open — below high threshold (non-blocking)*
*Severity: critical > high > medium > low — only open threats at or above workflow.security_block_on count toward threats_open*
*Disposition: mitigate (implementation required) · accept (documented risk) · transfer (third-party)*

---

## Accepted Risks Log

| Risk ID | Threat Ref | Rationale | Accepted By | Date |
|---------|------------|-----------|-------------|------|
| R-21-03 | T-21-03 | `gh api` calls in `create-or-reuse-release.sh` use the runner's `GITHUB_TOKEN` with repo-scoped permissions; response bodies are release metadata, not secrets — accepted at plan time (21-01 threat model) | plan 21-01 register, ratified by phase execution | 2026-08-31 |
| R-21-04 | T-21-04 | Release body size bounded in practice by the curated changelog section; GitHub's ~125k char cap is the hard stop and the finalize job's truncate-and-rebuild prevents unbounded growth | plan 21-01 register | 2026-08-31 |
| R-21-21 | T-21-21 | Finalize-job asset download is bounded to this repo's own release assets (`--pattern '*.tar.gz'`), produced by the same run — no untrusted source can inflate it | plan 21-04 register | 2026-08-31 |
| R-21-SC | T-21-SC | No new packages and no new Actions introduced by this phase; `cargo-release` is a pre-existing required local tool the `make release` target checks for rather than installs (RESEARCH.md Package Legitimacy Audit) | all six plan registers | 2026-08-31 |

*Accepted risks do not resurface in future audit runs.*

---

## Security Audit Trail

| Audit Date | Threats Total | Closed | Open | Run By |
|------------|---------------|--------|------|--------|
| 2026-09-01 | 30 (29 unique + T-21-SC) | 30 | 0 | /gsd-secure-phase orchestrator (L1 short-circuit; grep checks 1–8 + verifier/code-review/rehearsal corroboration) |

---

## Sign-Off

- [x] All threats have a disposition (mitigate / accept / transfer)
- [x] Accepted risks documented in Accepted Risks Log
- [x] `threats_open: 0` confirmed
