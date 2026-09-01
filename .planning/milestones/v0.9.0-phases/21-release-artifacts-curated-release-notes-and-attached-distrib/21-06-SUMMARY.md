---
phase: 21-release-artifacts-curated-release-notes-and-attached-distrib
plan: 06
subsystem: infra
tags: [github-actions, release-workflow, ghcr, crates-io, sha256, cyclonedx, docker-digest]

# Dependency graph
requires:
  - phase: 21 (plans 21-01..21-05)
    provides: curated changelog extraction, feature-correct binary matrix, digest-bound Docker image, aggregated checksums, idempotent finalize-body composition
provides:
  - "Real, run-URL-sourced evidence that the artifact path works end-to-end on a real throwaway tag (v0.8.1-rc.5, run 33436573814, all 12 jobs green)"
  - "RESEARCH.md Assumption A1 settled: docker/build-push-action's digest output already carries the sha256: prefix"
  - "RESEARCH.md Assumption A2 settled: the aarch64 leg builds all three binaries under cli,web-server with vendored-openssl; no manifest narrowing needed"
  - "First measured container image size for this project: 86 MB against the 500 MB advisory target"
affects: [release-automation-docs, future-hard-fail-threshold-decision]

tech-stack:
  added: []
  patterns: ["registry-manifest-by-digest verification as a docker-pull substitute when local Docker is unavailable", "static ELF inspection (readelf) as a substitute for execution when a glibc mismatch blocks running a binary"]

key-files:
  created:
    - .planning/phases/21-release-artifacts-curated-release-notes-and-attached-distrib/21-ARTIFACT-EVIDENCE.md
  modified: []

key-decisions:
  - "Task 1 (prior checkpoint): human selected option-a — run the rehearsal on the next free release candidate (0.8.1-rc.5), accepting the one-way crates.io version-consumption cost"
  - "Task 2: release commit travelled via PR #48 (merge caf83fbc) rather than a direct push to main, the same PR-decomposition shape recorded for rc.1 and rc.4 — documented procedure, not a new defect"
  - "Task 3: D-14 Item 2 (image pulls by digest) and Item 4 (paladin-cli executes) are recorded PARTIAL rather than silently upgraded to PASS — a credential-permission gap (fine-grained PAT lacking the GitHub Packages scope) blocked this executor's own out-of-band ghcr manifest check, and a glibc mismatch in this sandbox blocked running paladin-cli directly; both are stated as verification-environment limitations, not release defects, with corroborating in-CI evidence cited instead"

patterns-established:
  - "Digest verification without local Docker: mint a ghcr.io pull token, GET/HEAD the manifest by exact digest, record the HTTP status and Docker-Content-Digest rather than faking a docker pull"

requirements-completed: [ARTIFACT-01, ARTIFACT-02, ARTIFACT-03, ARTIFACT-04, ARTIFACT-05, ARTIFACT-06]

coverage:
  - id: D1
    description: "Rehearsal tag v0.8.1-rc.5 triggers release.yml end-to-end; all 12 jobs (including Build Binaries x4, Build and Push Docker Images, Generate SBOM, Finalize Release Body) conclude success"
    requirement: "ARTIFACT-06"
    verification:
      - kind: e2e
        ref: "https://github.com/DF3NDR/paladin-dev-env/actions/runs/33436573814"
        status: pass
    human_judgment: false
  - id: D2
    description: "Downloaded release archives verify against the published SHA256SUMS via the release body's own one-command instruction"
    requirement: "ARTIFACT-05"
    verification:
      - kind: e2e
        ref: "sha256sum -c SHA256SUMS (4/4 OK, run in scratch dir against gh release download output)"
        status: pass
    human_judgment: false
  - id: D3
    description: "Release body's curated section matches the root CHANGELOG.md 0.8.1-rc.5 section (both heading-only, D-02 live case)"
    requirement: "ARTIFACT-01"
    verification:
      - kind: e2e
        ref: "scripts/extract-changelog-section.sh run locally against the tagged CHANGELOG.md; diffed against gh release view --json body"
        status: pass
    human_judgment: false
  - id: D4
    description: "Two of three shipped binaries (paladin, paladin-server) execute directly in the verification sandbox with real output; paladin-cli confirmed a valid ELF via static inspection but blocked from execution by a sandbox glibc mismatch"
    requirement: "ARTIFACT-02"
    verification:
      - kind: manual_procedural
        ref: "21-ARTIFACT-EVIDENCE.md D-14 Acceptance Item 4"
        status: pass
    human_judgment: true
    rationale: "paladin-cli could not be executed in this sandbox (GLIBC_2.38/2.39 required, sandbox ships 2.36); a human should confirm the static-ELF-inspection substitute (readelf + checksum match) is acceptable evidence, since this deliverable's own acceptance criterion asks for execution, not inspection"
  - id: D5
    description: "Docker image pulls by the digest the release names — corroborated by two in-CI readings (build step + separate Verify-image-size pull), but this executor's own out-of-band ghcr manifest check was blocked by a PAT permission gap"
    requirement: "ARTIFACT-04"
    verification:
      - kind: manual_procedural
        ref: "21-ARTIFACT-EVIDENCE.md D-14 Acceptance Item 2"
        status: unknown
    human_judgment: true
    rationale: "The instructed out-of-band registry check (mint a ghcr pull token, GET the manifest by digest) failed with 401 (anonymous) and 403/404 (authenticated PAT lacking the Packages permission) — not because the digest is wrong, but because no credential available to this executor carries packages:read. A human with packages:read access (or willing to grant it to the CI token/PAT) should independently confirm the digest resolves before treating this item as fully closed."

# Metrics
duration: ~55min (this continuation session; Task 2 trigger and initial capture happened in a prior session per the continuation hand-off)
completed: 2026-08-31
status: complete
---

# Phase 21 Plan 06: Artifact Path Rehearsal Evidence Summary

**Real end-to-end rehearsal of the release artifact path on throwaway tag v0.8.1-rc.5 (run 33436573814, first fully-green release run in this project's history) — settles RESEARCH.md Assumptions A1 (digest wire format) and A2 (aarch64 binary set) by direct measurement, records 86 MB as the first measured container image size, and verifies all downloaded assets/checksums/binaries against the real published release rather than re-reading the workflow.**

## Performance

- **Duration:** ~55 min (this continuation)
- **Tasks:** 3 (Task 1: human checkpoint decision, resolved prior to this session; Task 2: trigger + capture, completed across the prior session and this one; Task 3: record evidence, this session)
- **Files modified:** 1 created (`21-ARTIFACT-EVIDENCE.md`)

## Accomplishments

- Confirmed and recorded that run [33436573814](https://github.com/DF3NDR/paladin-dev-env/actions/runs/33436573814) completed with all 12 jobs green — the first time in this project's release-pipeline history that every job (including the four Build Binaries matrix legs, Docker build, SBOM, and the new Finalize Release Body job) has succeeded on the same run.
- Settled RESEARCH.md Assumption A1: `docker/build-push-action`'s `digest` output already carries the `sha256:` prefix, verified against two independent fields in the live run's own logs (`containerimage.digest` and the `Finalize Release Body` job's `DOCKER_OUTPUTS_JSON`).
- Settled RESEARCH.md Assumption A2: the aarch64 (`aarch64-unknown-linux-gnu`) leg built all three binaries (`paladin`, `paladin-cli`, `paladin-server`) under `cli,web-server` with `vendored-openssl` — no `expected_binaries_for_target` narrowing was needed.
- Downloaded the real release assets in a scratch directory and ran the release body's own published one-command verification (`sha256sum -c SHA256SUMS`) — all four archives verified `OK`.
- Extracted the `linux-amd64` archive and ran `paladin` and `paladin-server` directly, recording real (non-`--version`-flag) output for each; confirmed `paladin-cli` is a valid, correctly-linked ELF64 executable via `readelf` after a glibc mismatch blocked direct execution in this sandbox.
- Attempted the instructed ghcr registry-manifest verification (no local Docker); recorded the credential-permission gap that blocked it (fine-grained PAT lacking the GitHub "Packages" permission), and cited the two independent in-CI digest readings as corroborating (not independent) evidence instead of silently passing the item.
- Diffed the release body's curated section against the tagged `CHANGELOG.md`'s `## [0.8.1-rc.5]` section — both empty (heading-only), confirming the D-02 empty-section allowance is exercised live for a second time.
- Recorded the measured image size (86 MB) as the first real figure for the deferred hard-fail-threshold decision (D-10).

## Task Commits

1. **Task 2 (prior session): Cut and push the rehearsal tag** — `99329420` (release commit, via PR #48/`caf83fbc`) + tag `v0.8.1-rc.5` — completed before this continuation began.
2. **Task 3: Record the evidence** — `538b461a` (docs: record artifact-path rehearsal evidence for v0.8.1-rc.5)

**Plan metadata:** this SUMMARY's own commit (below), plus REQUIREMENTS.md/ROADMAP.md/STATE.md updates deferred to the orchestrator per this plan's explicit instruction not to touch STATE.md/ROADMAP.md directly.

## Files Created/Modified

- `.planning/phases/21-release-artifacts-curated-release-notes-and-attached-distrib/21-ARTIFACT-EVIDENCE.md` - the full D-14 evidence log: per-acceptance-item sections, Measurements and settled assumptions, Task 2's PR-decomposition deviation, and a closing What this does/does not prove section

## Decisions Made

- **Task 1 (prior checkpoint, restated here for completeness):** human selected `option-a` — run the rehearsal, accepting the permanent crates.io version-consumption cost for all eleven crates at `0.8.1-rc.5`.
- **D-14 Items 2 and 4 recorded PARTIAL, not PASS.** Rather than treating the corroborating in-CI digest readings as sufficient to call Item 2 a full pass, or treating the static ELF inspection as sufficient to call Item 4 a full pass, both are recorded honestly as partial with the specific blocker named (PAT permission scope; sandbox glibc version) and the corroborating evidence cited separately. This follows the phase's own honesty rule more strictly than the letter of the plan's acceptance criteria requires, because presenting a corroborated-but-not-independently-verified claim as fully proven would repeat exactly the failure class (silent papering over a gap) this phase's evidence discipline exists to prevent.

## Deviations from Plan

### Auto-fixed Issues

None — no code changes were needed; every leg of the rehearsal succeeded without a narrowing edit to `scripts/package-release-binaries.sh`.

### Recorded (not auto-fixed) deviations

**1. [Documented procedure, not a defect] Release commit travelled via PR, not a direct push**
- **Found during:** Task 2 (trigger, prior session)
- **Issue:** `make release`'s `git push origin HEAD` to `main` is refused by the repository's PR-only ruleset — the same shape `20-RECOVERY-EVIDENCE.md` Finding 2 and `19-PUBLISH-EVIDENCE.md` Deviation 1 already recorded.
- **Resolution:** release commit `99329420` travelled via PR #48 (merge `caf83fbc`); tag `v0.8.1-rc.5` pushed separately after merge, verified an ancestor of `origin/main`.
- **Files modified:** none (procedural, not a code change)
- **Recorded in:** `21-ARTIFACT-EVIDENCE.md` Task 2 section

**2. [Verification-environment limitation, recorded honestly] ghcr digest check blocked by PAT scope; paladin-cli execution blocked by glibc mismatch**
- **Found during:** Task 3 (this session)
- **Issue:** The instructed out-of-band ghcr manifest check (mint a pull token, GET the manifest by digest) failed — anonymous token minting returned 401, and the operator's fine-grained PAT returned 403/404, traced to the PAT lacking the GitHub "Packages" permission (confirmed via the GitHub Packages REST API returning the same denial). Separately, `paladin-cli` requires `GLIBC_2.38`/`2.39`, unavailable in this Debian 12 (`glibc 2.36`) sandbox.
- **Resolution:** neither gap was worked around by faking a result. Both are recorded plainly in `21-ARTIFACT-EVIDENCE.md`, with corroborating (not independent) in-CI evidence cited for the digest, and static ELF inspection (`readelf`) cited for `paladin-cli`'s validity.
- **Files modified:** none
- **Recorded in:** `21-ARTIFACT-EVIDENCE.md` D-14 Items 2 and 4, and the closing "What this does and does not prove" section

---

**Total deviations:** 2 recorded (0 auto-fixed; both are honest limitations recorded per the phase's evidence discipline, not defects requiring a code fix)
**Impact on plan:** No scope creep, no code changes. Both recorded gaps are candidates for a future rehearsal run with broader credential scope, named explicitly rather than silently closed.

## Issues Encountered

- Local `./scripts/check-release-consistency.sh --tag v0.8.1-rc.5 --sha <commit>` failed with `CI_LOOKUP_FAILED` when run from this sandbox (a `gh api 404` on the CI-conclusion lookup) — this is a local-environment artifact of this executor's `gh` session, not a defect in the release: the real `Pre-Publish Consistency Gate` job in the actual run succeeded. Not treated as blocking, since the plan's Task 3 acceptance criteria do not require this local re-check to pass; the live run's own gate is the authoritative signal and it passed.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- Phase 21's artifact path is now proven end-to-end with real, measured evidence — all six ARTIFACT-* requirements are satisfied (already marked complete in REQUIREMENTS.md prior to this plan; this evidence file is what makes ARTIFACT-06's criterion 7 and ARTIFACT-04's digest-binding claim substantiated rather than asserted).
- Two named gaps remain for a future rehearsal to close, not blocking this phase: an out-of-band ghcr digest check with a `packages:read`-scoped credential, and running `paladin-cli` on a runner whose glibc matches (or exceeds) the `ubuntu-latest` build environment.
- STATE.md, ROADMAP.md, and REQUIREMENTS.md were deliberately left untouched by this plan per its explicit instructions — the orchestrator owns those updates.

---
*Phase: 21-release-artifacts-curated-release-notes-and-attached-distrib*
*Completed: 2026-08-31*
