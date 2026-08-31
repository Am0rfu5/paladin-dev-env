---
status: testing
phase: 21-release-artifacts-curated-release-notes-and-attached-distrib
source: [21-VERIFICATION.md]
started: 2026-08-31T22:30:00Z
updated: 2026-08-31T22:30:00Z
---

## Current Test

number: 1
name: Docker pull by immutable digest, from outside CI
expected: |
  From a machine with working `docker` (or a `packages:read`-scoped credential) outside CI, run:
  `docker pull ghcr.io/df3ndr/paladin-dev-env@sha256:9e6d22d7bc01c459447719cf4f7753c1fc18095d5aff5c3d5d5fa44d1517f1c2`
  The image pulls cleanly by the immutable digest the v0.8.1-rc.5 release body names — the literal
  ARTIFACT-06 acceptance clause ("whose image pulls by the digest the release names").
awaiting: user response

## Tests

### 1. Docker pull by immutable digest, from outside CI
expected: |
  `docker pull ghcr.io/df3ndr/paladin-dev-env@sha256:9e6d22d7bc01c459447719cf4f7753c1fc18095d5aff5c3d5d5fa44d1517f1c2`
  succeeds from outside the CI run that produced it. Why human: the verification sandbox has no
  `docker`, anonymous ghcr tokens were refused (401), and the available fine-grained PAT lacks
  `packages:read` (403/404). The two CI-internal corroborating readings (build step's self-reported
  digest; a pull by *tag* later in the same job) are not the literal out-of-band pull-by-digest.
  Note: `COVERAGE.md`'s "Pull an image by immutable digest … 21-06 (rehearsal proof)" row overstates
  this — correct that row or close this gap.
result: [pending]

### 2. Execute paladin-cli from the released archive
expected: |
  On a host with glibc >= 2.39 (or the CI runner's own environment), extract
  `paladin-linux-amd64.tar.gz` from release v0.8.1-rc.5 and run `./paladin-cli --help` (or
  `--version`). The binary executes and produces real output, matching the clean runs already
  observed for `paladin` and `paladin-server` from the same archive. Why human: the verification
  sandbox's Debian 12 glibc (< 2.38) cannot load it; it was confirmed a well-formed, correctly
  linked ELF with a verified checksum, but no process has ever been run from it.
result: [pending]

## Summary

total: 2
passed: 0
issues: 0
pending: 2
skipped: 0
blocked: 0

## Gaps
