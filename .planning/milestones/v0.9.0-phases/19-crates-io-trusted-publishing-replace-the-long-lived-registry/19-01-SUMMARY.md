---
phase: 19-crates-io-trusted-publishing-replace-the-long-lived-registry
plan: 01
subsystem: infra
tags: [crates-io, release, cargo, publish-order, github-actions, rulesets]

requires:
  - phase: 18-rust-sast-evaluate-and-adopt-codeql
    provides: sealed security-tooling baseline on main; release.yml cache modernization (15.1)
provides:
  - Eleven-crate publish set reconciled across cargo metadata, .crate-names.txt, and release.yml's CRATES array (paladin-herald inserted at the dependency-valid position)
  - All eleven crates live on crates.io at 0.8.1-rc.1, paladin-herald for the first time — the precondition for its Trusted Publishing configuration (19-03)
  - Bootstrap publish evidence recorded with the credential named honestly (standing CARGO_REGISTRY_TOKEN, trustpub_data null on all eleven versions)
affects: [19-02, 19-03, 19-04, 19-05, phase-20-publish-ops]

tech-stack:
  added: []
  patterns: [package include allowlist for root-crate packaging, PR-decomposed release flow under PR-only main ruleset]

key-files:
  created:
    - .planning/phases/19-crates-io-trusted-publishing-replace-the-long-lived-registry/19-PUBLISH-EVIDENCE.md
  modified:
    - .github/workflows/release.yml
    - Cargo.toml
    - CHANGELOG.md
    - crates/paladin-web/openapi.json
    - Makefile
    - .github/workflows/pre-commit.yml

key-decisions:
  - "bootstrap-now (Task 2 one-way door): first publish of paladin-herald through the standing token during the pre-revocation window, per locked D-02/D-03"
  - "make release's direct-push flow is dead under the PR-only main ruleset — release decomposed into branch → PR #36 → merge → admin-bypass tag push; docs/Makefile update deferred to 19-05"
  - "paladin-ai 413 fix: include allowlist in root [package] (2,425 files/>10MiB → 442 files/800KiB); published crate built from main a5f27791 (tag + packaging fix only), tag not moved — recorded plainly in evidence"

patterns-established:
  - "Evidence log entries carry run URLs, dates, actors, credentials, and failed attempts — not just the success"
  - "publish_one's already-published tolerance proved out: ten crates skipped cleanly on re-runs"

duration: ~3h wall clock (spanned a GitHub Actions major outage)
completed: 2026-08-26
---

# Phase 19 Plan 01: Reconcile the publish set and bootstrap-publish eleven crates

## What was built

- **Task 1 (prior session):** `release.yml`'s `CRATES` array reconciled to the eleven
  publishable crates with `paladin-herald` inserted after `paladin-ports` (its
  version-pinned dev-dependency) and before `paladin-ai` (its only dependent); the
  planning docs' proposed insertion point was proven wrong against the manifests.
- **Task 2 (prior session):** one-way-door decision recorded as `bootstrap-now`.
- **Task 3 (this session, human-action checkpoint executed on the owner's explicit
  delegation):** version `0.8.1-rc.1` cut via `cargo release version` + changelog +
  OpenAPI baseline on a branch, merged as PR #36 (`828515b3`), tag `v0.8.1-rc.1`
  pushed via admin bypass. Publishing took three workflow runs (GitHub Actions
  outage; then a deterministic `413 Payload Too Large` on `paladin-ai`, fixed by a
  package include allowlist in PR #37). All eleven crates verified live at
  `0.8.1-rc.1` with `trustpub_data: null`; full history in `19-PUBLISH-EVIDENCE.md`
  §"Bootstrap Publish (old credential)".

## Deviations

- The documented `make release` flow is incompatible with the "Protect main branch"
  ruleset — decomposed into PR flow (evidence Deviation 1); 19-05 must update the docs.
- `make release` omits the OpenAPI baseline regeneration its own gate requires (`make
  openapi` run manually) and `lint-shell` had to learn the pre-commit gate's `.claude/`
  exclusion — both fixed en route (commits `d0585d67`, and pre-commit.yml toolchain
  step `d372baca` earlier the same day).
- `paladin-ai`'s published crate is built from `main` `a5f27791` (tag content + the
  packaging fix), the other ten from the tag commit `828515b3`.

## Open items

- Build Binaries matrix (4 targets) failed in every release run — systematic, not on
  the publish path, undiagnosed; carried in the evidence log's open items.
- `actions/create-release@v1` is not re-run tolerant (release object had to be
  deleted before each re-dispatch).

## Self-Check: PASSED

- All eleven `https://crates.io/api/v1/crates/<name>/0.8.1-rc.1` endpoints: HTTP 200
- `paladin-herald` crate-level endpoint: HTTP 200 (was 404)
- `trustpub_data`: `null` on all eleven versions
- `git ls-remote origin refs/tags/v0.8.1-rc.1`: present (`18ac0996`)
