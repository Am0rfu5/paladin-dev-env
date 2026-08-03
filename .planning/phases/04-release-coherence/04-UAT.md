---
status: complete
phase: 04-release-coherence
source: [04-01-SUMMARY.md, 04-02-SUMMARY.md, 04-03-SUMMARY.md, 04-04-SUMMARY.md, 04-05-SUMMARY.md, 04-06-SUMMARY.md, 04-07-SUMMARY.md]
started: 2026-08-03
updated: 2026-08-03
---

## Current Test

[testing complete]

## Tests

<!-- 22 deliverables auto-passed via `uat classify-coverage`: deterministically covered
     by passing tests cited in their `verification` refs. Not presented as checkpoints. -->

### 1. Edition 2024 on paladin-ports
expected: paladin-ports declares edition 2024; workspace builds
result: pass
source: automated
coverage_id: 04-01/D1

### 2. Edition 2024 on paladin-notifications; split closed
expected: zero manifests on 2021, twelve on 2024
result: pass
source: automated
coverage_id: 04-01/D2

### 3. Both D-06 build legs green
expected: `cargo build --workspace` and `--no-default-features` both exit 0
result: pass
source: automated
coverage_id: 04-01/D3

### 4. Stale advisory suppression removed
expected: `cargo deny check` emits no `advisory-not-detected` warning
result: pass
source: automated
coverage_id: 04-02/D1

### 5. cargo audit clean
expected: 0 vulnerabilities, advisory-DB snapshot recorded
result: pass
source: automated
coverage_id: 04-02/D2

### 6. cargo deny check clean
expected: advisories ok, bans ok, licenses ok, sources ok
result: pass
source: automated
coverage_id: 04-02/D3

### 7. Migration/review notes completed
expected: 12 of 14 surviving suppressions carry a migration or review note
result: pass
source: automated
coverage_id: 04-02/D4

### 8. Four newly-surfaced advisories recorded, not suppressed
expected: recorded as a dated finding with owners; added to neither ignore list
result: pass
source: automated
coverage_id: 04-02/D5

### 9. release/** push trigger restored
expected: `on.push.branches` includes `release/**`
result: pass
source: automated
coverage_id: 04-03/D1

### 10. ci.yml parses as YAML after every edit
expected: yaml.safe_load succeeds
result: pass
source: automated
coverage_id: 04-03/D5

### 11. Pre-existing CI jobs byte-unchanged
expected: security, api-surface and all other jobs untouched
result: pass
source: automated
coverage_id: 04-03/D6

### 12. Local gate suite — fmt
expected: `cargo fmt --all -- --check` clean
result: pass
source: automated
coverage_id: 04-04/D1

### 13. Local gate suite — clippy, tests, doc tests
expected: clippy 0 warnings under -D warnings; 2,924 tests pass; 185 doc tests pass
result: pass
source: automated
coverage_id: 04-04/D2

### 14. "22 examples" amended at source
expected: five restatements amended with dated provenance; gate becomes "every example target builds"
result: pass
source: automated
coverage_id: 04-04/D3

### 15. Twelve manifests + every internal pin at 0.7.0
expected: 12 × 0.7.0; exact pin `=0.7.0`; tiktoken-rs unmoved
result: pass
source: automated
coverage_id: 04-05/D1

### 16. CHANGELOG finalized
expected: `## [0.7.0] - 2026-08-03`; `## [0.6.0] - 2026-06-10`
result: pass
source: automated
coverage_id: 04-05/D2

### 17. QUICKSTART sample repaired and proven to compile
expected: imports resolve against shipped tree; sample compiles offline
result: pass
source: automated
coverage_id: 04-06/D1

### 18. QUICKSTART timing recorded pass or fail
expected: first-ever measurement, labelled "measured under stated conditions"
result: pass
source: automated
coverage_id: 04-06/D2

### 19. ADR-0008 and ADR-0009 authored
expected: both carry `Code conformance` and name downstream consumers; PROMOTION index updated
result: pass
source: automated
coverage_id: 04-07/D1

### 20. CONCERNS.md edition claim corrected at source
expected: original text retained, dated amendment appended
result: pass
source: automated
coverage_id: 04-07/D2

### 21. REL-01..REL-05 ledger rows written
expected: REL-05 split into facets; every deferral carries a named owner
result: pass
source: automated
coverage_id: 04-07/D3

### 22. No row claims a Docker or Kubernetes gate was proven
expected: zero blended `satisfied` verdicts across the ledger
result: pass
source: automated
coverage_id: 04-07/D4

<!-- Orchestrator pre-check, derived from the phase goal's opening clause
     ("A developer can clone the release tag, build it ... follow the quickstart") -->

### 23. Clone the release tag and get a coherent tree
expected: |
  The `v0.7.0` tag points at a commit whose test suite passes and whose QUICKSTART
  is the repaired one — i.e. a developer cloning the tag gets the release the phase
  claims to have produced.
result: issue
reported: "Orchestrator pre-check: tag v0.7.0 pointed at 648e7a4, 17 commits behind HEAD. At that commit crates/paladin-web/openapi.json still read 0.6.0 while its manifest read 0.7.0, so `cargo test --workspace` FAILED at the tag; and docs/src/getting-started/quickstart.md was still the un-repaired version citing paladin-ai-core 0.5.0 and the non-existent llm-openai feature. ADR-0008 and ADR-0009 were also absent. A developer cloning the tag would get a failing build and a broken quickstart — directly falsifying two clauses of the phase goal."
severity: blocker
status: resolved
resolution: |
  Root cause: orchestration ordering, not plan content. The tag was created immediately
  after Wave 3's merge because plan 04-05 sat in Wave 3, but Waves 4 (QUICKSTART repair),
  5 (ADRs, ledger) and the OpenAPI baseline fix all landed afterward. A release tag must
  be the last artifact created in a release phase, after every content change.

  Fixed by deleting and recreating the local annotated tag at HEAD (35535d2):
  `git tag -d v0.7.0 && git tag -a v0.7.0 -m "Release 0.7.0" HEAD`

  Verified after the move:
  - `git log --oneline v0.7.0..HEAD` returns 0 commits
  - `git show v0.7.0:crates/paladin-web/openapi.json` → "version": "0.7.0"
  - `git show v0.7.0:docs/src/getting-started/quickstart.md` → `paladin-ai = "0.7.0"`,
    `use paladin::application::services::paladin::paladin_builder::PaladinBuilder;`
  - `git ls-remote --tags origin` → still absent from origin (never pushed)

  Safe because the tag is local and unpushed; no crates.io state was ever affected.

<!-- Human-judgment checkpoints from `uat classify-coverage` (4 of 26) -->

### 24. CI examples job — first execution never run
expected: |
  The `examples` job builds all 47 example targets via a 4-invocation feature matrix
  and hard-fails if the binary count is not 47. Statically verified (YAML structure,
  invocation count, gated-example names present) — but never executed, because GitHub
  Actions cannot be triggered from this sandbox.
result: pass
reason: "User accepted the deferral — authored + statically validated is the correct disposition; first execution owned by Phase 15 / PIPE. Superseded in part by the branch push below, which lets CI execute this job for real."

### 25. Docker multi-arch build and its 500 MB / 300 s budgets
expected: |
  Authored and statically validated only. `docker` is absent here, so neither budget has
  ever been measured against a built image. Filed `deferred with reason`, owner Phase 15 /
  PIPE. Note the 300 s gate is expected RED on first real execution: the only Docker
  figure in the corpus (PROJECT.md:767) is 5m31s single-arch, already over budget.
result: pass
reason: "User accepted the deferral. The 300 s budget is recorded as expected-RED on first execution; a red there is the measurement REL-05 has never taken, not a regression."

### 26. Kubernetes smoke job and its 30 s pod-startup budget
expected: |
  Authored and statically validated only. `kind` and `kubectl` are absent, so the budget
  has never been measured. Additionally, k8s/deployment.yaml runs a placeholder
  `sleep 3600` with readiness probes commented out — so even once executed, the figure
  measures container scheduling, not application readiness. Owners: Phase 15 / PIPE
  (first execution), Phase 14 / WEB (real probe wiring).
result: pass
reason: "User accepted the deferral. Owners confirmed: Phase 15 / PIPE for first execution, Phase 14 / WEB for real readiness-probe wiring."

### 27. The tag/push/publish human gate
expected: |
  The annotated tag `v0.7.0` exists locally and is absent from origin. The push+publish
  sequence is documented and unexecuted.
result: pass
reason: "User directed: push the BRANCH only, not the tag. release/v0.7.0 pushed to origin so CI exercises the examples/Docker/Kubernetes jobs for real. The tag stays local; crates.io is untouched, since release.yml fires on tag push only."

## Summary

total: 27
passed: 26
issues: 1
resolved: 1
pending: 0
skipped: 0
blocked: 0

## Gaps

- gap_id: G-04-23
  truth: "A developer can clone the release tag and get a tree that builds and whose quickstart works"
  status: resolved
  reason: "Orchestrator pre-check: tag pointed 17 commits behind HEAD — failing test suite and un-repaired QUICKSTART at the tagged commit"
  severity: blocker
  test: 23
  root_cause: "Tag created after Wave 3's merge instead of after all content changes; Waves 4-5 and the OpenAPI fix landed afterward"
  artifacts:
    - path: ".git/refs/tags/v0.7.0"
      issue: "pointed at 648e7a4 rather than the final phase commit"
  missing:
    - "Recreate the local annotated tag at HEAD once all content changes have landed"
  resolved_by: "orchestrator, in-session"
  resolved_at: 2026-08-03
