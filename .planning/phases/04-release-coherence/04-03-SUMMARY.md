---
phase: 04-release-coherence
plan: 03
subsystem: infra
tags: [ci, github-actions, docker, kubernetes, kind, cargo, examples, release-gate]

# Dependency graph
requires:
  - phase: 04-release-coherence
    provides: "D-14/D-15 CI configuration gaps identified in 04-CONTEXT.md and the exact analog excerpts in 04-RESEARCH.md/04-PATTERNS.md"
provides:
  - "ci.yml push trigger covering release/** so a push to release/v0.7.0 actually fires CI"
  - "examples job (Example Muster (Feature Matrix)): 4-invocation build matrix covering all 47 example targets with a binary-count assertion"
  - "docker job extended with linux/amd64,linux/arm64 multi-arch build plus hard-failing 500 MB size and 300 s wall-clock budget assertions"
  - "kubernetes-smoke job: kind-based smoke test with a hard-failing 30 s pod-startup budget assertion"
  - "04-ci-gate-deferrals.md: the D-15 deferral register naming what was authored/statically validated vs. executed, with six deferred-with-reason rows and named owners"
  - "COVERAGE.md: reasoned no-external-API-integration declaration for this phase"
affects: [phase-15-pipe, phase-14-web, phase-9-sec-01]

# Tech tracking
tech-stack:
  added: []
  patterns: ["deferred with reason + named owner ledger convention applied to CI gates that cannot execute in this environment"]

key-files:
  created:
    - .planning/phases/04-release-coherence/04-ci-gate-deferrals.md
    - .planning/phases/04-release-coherence/COVERAGE.md
  modified:
    - .github/workflows/ci.yml

key-decisions:
  - "Only ci.yml gets the release/** push trigger; integration-tests.yml and feature-flags.yml carry the identical commented-out stanza but are deliberately left untouched (owner: Phase 15 / PIPE-04)"
  - "The 300s Docker time budget applies to the whole multi-arch buildx invocation, authored to hard-fail rather than softened to a warning, and is expected red on first execution (only measurement in corpus: 112 MB / 5m31s single-arch)"
  - "The Kubernetes smoke job reuses k8s/deployment.yaml's placeholder sleep-3600 shape as-is; real readiness-probe wiring is out of this phase's boundary (owner: Phase 14 / WEB)"
  - "gh is installed (2.96.0) but not authenticated in this execution sandbox, unlike the CONTEXT.md discussion session — recorded honestly in the deferral register rather than fabricating a gh run list result"

patterns-established:
  - "Budget assertions emit ::error:: + exit 1, never ::warning::, when SC5 language says 'inside its budget'"

requirements-completed: [REL-05]

coverage:
  - id: D1
    description: "ci.yml push trigger fires on release/** (a push to release/v0.7.0 now runs CI)"
    requirement: "REL-05"
    verification:
      - kind: other
        ref: "python3 -c \"import yaml; d=yaml.safe_load(open('.github/workflows/ci.yml')); assert 'release/**' in d['on']['push']['branches']\""
        status: pass
    human_judgment: false
  - id: D2
    description: "examples job builds all 47 example targets via a 4-invocation feature matrix and fails if binary count != 47"
    requirement: "REL-05"
    verification:
      - kind: other
        ref: "python3 -c yaml assertion: cargo build occurs >=4 times in examples job steps; job name == 'Example Muster (Feature Matrix)'"
        status: pass
    human_judgment: true
    rationale: "The build matrix and the 47-binary assertion are statically verified (YAML structure, invocation count, gated-example names present), but the job has never actually executed in CI — first execution requires GitHub Actions, which cannot be triggered from this sandbox."
  - id: D3
    description: "docker job builds linux/amd64,linux/arm64 and hard-fails above 500 MB or 300 s"
    requirement: "REL-05"
    verification:
      - kind: other
        ref: "python3 -c yaml assertion: 'linux/amd64,linux/arm64' and 'setup-qemu-action@v3' present; '-gt 500'/'-gt 300'/'exit 1' present in run steps"
        status: pass
    human_judgment: true
    rationale: "Authored and statically validated only (D-15) — docker is absent from this environment, so the budgets have never actually been measured against a built image. Filed as deferred with reason, owner Phase 15 / PIPE."
  - id: D4
    description: "kubernetes-smoke job exists and hard-fails above 30 s or on unreadable timestamps"
    requirement: "REL-05"
    verification:
      - kind: other
        ref: "python3 -c yaml assertion: job name == 'Kubernetes Smoke Test'; '-gt 30'/'exit 1' present in run steps"
        status: pass
    human_judgment: true
    rationale: "Authored and statically validated only (D-15) — kind and kubectl are absent from this environment, so the budget has never actually been measured. Filed as deferred with reason, owner Phase 15 / PIPE. Also measures container scheduling, not app readiness, per the k8s/deployment.yaml placeholder caveat (owner Phase 14 / WEB)."
  - id: D5
    description: "04-ci-gate-deferrals.md records the D-15 deferral register with static-validation results and six named-owner deferred rows"
    requirement: "REL-05"
    verification:
      - kind: other
        ref: "grep -c 'deferred with reason' .planning/phases/04-release-coherence/04-ci-gate-deferrals.md >= 6; grep -c 'Owner: Phase 15 / PIPE' >= 3; grep -c 'Owner: Phase 14 / WEB' >= 1"
        status: pass
    human_judgment: false
  - id: D6
    description: "COVERAGE.md carries the reasoned no-external-API-integration declaration"
    verification:
      - kind: other
        ref: "grep -c '^No external API integration:' .planning/phases/04-release-coherence/COVERAGE.md == 1"
        status: pass
    human_judgment: false

# Metrics
duration: 20min
completed: 2026-08-03
status: complete
---

# Phase 4 Plan 3: CI Gate Repair and Docker/Kubernetes Deferral Register Summary

**Restored `ci.yml`'s `release/**` push trigger, added a 4-invocation examples feature-matrix job with a 47-binary assertion, extended the Docker job with hard-failing multi-arch size/time budgets, added a kind-based Kubernetes smoke job with a hard-failing startup budget, and filed both unexecutable gates in a named-owner deferral register — never claimed green.**

## Performance

- **Duration:** ~20 min
- **Completed:** 2026-08-03T00:21:34Z
- **Tasks:** 3
- **Files modified:** 1 (`.github/workflows/ci.yml`)
- **Files created:** 2 (`04-ci-gate-deferrals.md`, `COVERAGE.md`)

## Accomplishments
- `ci.yml`'s `on.push.branches` now includes `release/**` (plus `main`, `develop`, `feature/**`), so a push to `release/v0.7.0` fires CI — previously the `push:` trigger was entirely commented out and only `pull_request`/`workflow_dispatch` ran.
- New `examples` job (`Example Muster (Feature Matrix)`) runs 4 build invocations (default-feature bulk build + 3 feature-gated invocations for `vision_analysis`/`vision_battalion`, `document_processing`, `http_service_host`) and asserts exactly 47 example binaries are produced, failing the job otherwise — closing the gap where a bare `cargo build --examples` silently skips 4 of 47 targets with exit code 0.
- `docker` job extended: `docker/setup-qemu-action@v3` + `platforms: linux/amd64,linux/arm64`, plus two hard-failing budget assertions (image size ≤ 500 MB, wall-clock ≤ 300 s) using `::error::` + `exit 1` rather than `release.yml`'s non-blocking `::warning::` pattern.
- New `kubernetes-smoke` job reusing `integration-tests.yml`'s working kind/kubectl smoke-test shape, with two deliberate hardenings over the copied source: the paladin-pod wait no longer has `|| true` (a timeout now fails the job), and the startup-time check fails on unreadable timestamps instead of silently skipping the check.
- `04-ci-gate-deferrals.md` records the D-17 provenance block, all three static-validation results (YAML parse, action-reference resolution against sibling workflows, filesystem-path resolution) with raw command output, an honest record of the `gh run list` read attempt (not authenticated in this execution sandbox, unlike the CONTEXT.md discussion session), and six `deferred with reason` rows each with a named owner.
- `COVERAGE.md` states plainly that this phase introduces no new external API surface, so the seal-time re-scan doesn't misread the CI/manifest prose as new integration work.

## Task Commits

Each task was committed atomically:

1. **Task 1: Restore the push trigger for `release/**` and add the examples feature-matrix job** - `8d4ea16` (feat)
2. **Task 2: Add Docker size and time budget assertions and a kind-based Kubernetes smoke job** - `2526fef` (feat)
3. **Task 3: Statically validate the whole file and write the deferral register** - `ceeb2a7` (docs)

_Worktree mode: STATE.md/ROADMAP.md updates are owned by the orchestrator after this wave's worktree agents merge; no plan-metadata commit is made from this worktree._

## Files Created/Modified
- `.github/workflows/ci.yml` - push trigger widened to include `release/**`; new `examples` job; `docker` job extended with multi-arch + budget assertions; new `kubernetes-smoke` job
- `.planning/phases/04-release-coherence/04-ci-gate-deferrals.md` - D-15 deferral register: provenance, static-validation raw output, `gh run list` read attempt, six deferred-with-reason rows with named owners
- `.planning/phases/04-release-coherence/COVERAGE.md` - reasoned no-external-API-integration declaration

## Decisions Made
- Followed the plan's decision to leave `integration-tests.yml` and `feature-flags.yml`'s identical commented-out `push:` stanzas untouched — only `ci.yml` gets the trigger, per D-14's named scope. Recorded as deferred row 6, owner Phase 15 / PIPE-04.
- Followed the plan's decision that the 300 s Docker time budget applies to the whole multi-arch buildx invocation (not per-architecture), authored to hard-fail even though it is expected to be red on first real execution.
- Followed the plan's decision to reuse `k8s/deployment.yaml`'s placeholder `sleep 3600` shape verbatim rather than wiring real readiness probes (new product capability, out of boundary) — recorded as deferred row 3, owner Phase 14 / WEB.
- **Deviation from the plan's assumed `gh` behavior:** `04-CONTEXT.md`'s D-16 verified `gh run list` reading the remote successfully during the phase's discussion session. In this plan's execution sandbox, `gh 2.96.0` is installed but not authenticated (`gh auth status` fails). Recorded this honestly in the deferral register rather than fabricating the "expected empty" output D-16 anticipated — the underlying D-14.1 finding (a `release/**` push previously ran nothing) is independently established by the static validation of the pre-edit `push:` trigger being commented out, not by `gh run list`'s emptiness, so this does not weaken any claim in the register.

## Deviations from Plan

None beyond the `gh` authentication note above, which is a recorded environmental observation, not a deviation from any task's required action — the plan's own D-16 instruction ("read CI state, do not infer it") was followed exactly; the read simply came back as an auth failure rather than an empty result set, and that failure is recorded verbatim rather than smoothed over.

## Issues Encountered
None. All three tasks' acceptance criteria and `<verify>` commands passed on the first attempt.

## User Setup Required
None - no external service configuration required.

## Next Phase Readiness
- `ci.yml` can now prove SC5's format/clippy/test/doc-test/examples clauses on a `release/**` push, and the Docker/Kubernetes clauses are expressed as real, hard-failing gates rather than absent ones.
- First execution of the `docker` and `kubernetes-smoke` jobs requires a Docker-and-kind-capable CI runner — owned by Phase 15 / PIPE per `04-ci-gate-deferrals.md`.
- Real pod-readiness measurement (replacing the `sleep 3600` placeholder) requires `paladin-web` health endpoints — owned by Phase 14 / WEB.
- No blockers for this phase's remaining plans (04-01, 04-02, 04-04 through 04-07); this plan's `ci.yml` changes are additive and do not depend on or conflict with the version/edition/advisory work in sibling plans.

---
*Phase: 04-release-coherence*
*Completed: 2026-08-03*

## Self-Check: PASSED

All claimed files found on disk (`.github/workflows/ci.yml`, `04-ci-gate-deferrals.md`,
`COVERAGE.md`, `04-03-SUMMARY.md`) and all four commit hashes (`8d4ea16`, `2526fef`, `ceeb2a7`,
`4ecda7b`) found in `git log --oneline --all`.
