---
status: complete
phase: 21-release-artifacts-curated-release-notes-and-attached-distrib
source: [21-VERIFICATION.md]
started: 2026-08-31T22:30:00Z
updated: 2026-09-01T00:30:00Z
---

## Current Test

[testing complete]

## Tests

### 1. Docker pull by immutable digest, from outside CI
expected: |
  `docker pull ghcr.io/df3ndr/paladin-dev-env@sha256:9e6d22d7bc01c459447719cf4f7753c1fc18095d5aff5c3d5d5fa44d1517f1c2`
  succeeds from outside the CI run that produced it. Why human: the verification sandbox has no
  `docker`, anonymous ghcr tokens were refused (401), and the available fine-grained PAT lacks
  `packages:read` (403/404). The two CI-internal corroborating readings (build step's self-reported
  digest; a pull by *tag* later in the same job) are not the literal out-of-band pull-by-digest.
  Note: COVERAGE.md's digest-pull row was corrected 2026-09-01 to state the pending status.
result: pass
evidence: |
  2026-09-01, user (repo admin) from a docker-capable host outside CI, gh token refreshed with
  read:packages: `docker pull ghcr.io/df3ndr/paladin-dev-env@sha256:9e6d22d7bc01c459447719cf4f7753c1fc18095d5aff5c3d5d5fa44d1517f1c2`
  → all layers pulled, "Digest: sha256:9e6d22d7…f1c2", "Status: Downloaded newer image".
  Observed failure shapes on the way (recorded for the runbook): without docker auth ghcr answers
  "unauthorized"; with auth but missing read:packages scope ghcr answers "manifest unknown"
  (a 404-shaped response masking a scope problem — same class as Phase 20 Finding 5).

### 2. Execute paladin-cli from the released archive
expected: |
  On a host with glibc >= 2.39 (or the CI runner's own environment), extract
  `paladin-linux-amd64.tar.gz` from release v0.8.1-rc.5 and run `./paladin-cli --help` (or
  `--version`). The binary executes and produces real output, matching the clean runs already
  observed for `paladin` and `paladin-server` from the same archive. Why human: the verification
  sandbox's Debian 12 glibc (< 2.38) cannot load it; it was confirmed a well-formed, correctly
  linked ELF with a verified checksum, but no process has ever been run from it.
result: pass
evidence: |
  2026-09-01, user's host: direct execution failed with `GLIBC_2.38'/'GLIBC_2.39' not found`
  (host glibc too old — same as the sandbox), then executed inside `ubuntu:24.04` (glibc 2.39,
  matching the ubuntu-latest builder): `docker run --rm -v "$PWD:/w" -w /w ubuntu:24.04
  ./paladin-cli --help` printed the full CLI surface (agent, battalion, arsenal, maneuver,
  onboarding, setup-check, features, muster, council; --quiet/--verbose/-h/-V). Real process
  execution of the released artifact confirmed.

## Summary

total: 2
passed: 2
issues: 0
pending: 0
skipped: 0
blocked: 0

## Gaps

## Deferred Follow-Ups

- test: 2
  idea: "Released linux-amd64 binaries are dynamically linked against glibc 2.39 (ubuntu-latest 24.04 builder), so Debian 12 / Ubuntu 22.04-era hosts cannot run them (observed live on two machines). Consider musl/static builds or an older build baseline in a future phase."
  deferred_at: 2026-09-01
