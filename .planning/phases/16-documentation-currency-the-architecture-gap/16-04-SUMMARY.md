---
phase: 16-documentation-currency-the-architecture-gap
plan: 04
subsystem: docs
tags: [mdbook, docs-currency, docker, kubernetes, deployment, security-tooling]

# Dependency graph
requires:
  - phase: 16-01
    provides: "Pinned mdbook toolchain, the D-09 verdict record seeded with all fourteen files, and the eight-class signal battery proven on cicd.md"
  - phase: 16-03
    provides: "Six of fourteen D-09 files settled by content (the whole docs/src/user-guides/ group), the reusable verdict-row shape"
provides:
  - "Three more of fourteen D-09 files settled by content: docker.md, kubernetes.md, production.md — closing the four-file docs/src/deployment/ group (Milestone 11 task 7.0) alongside cicd.md (16-01)"
  - "docker.md's fabricated config.yml (paladin: section, garrison.type, arsenal.mcp_servers[].type, storage:/queue: field names), Environment Variables (missing APP_ prefix, LOG_LEVEL, nonexistent SERVER_HOST/PORT/DEFAULT_* overrides) and /health response corrected against the real Settings struct and health.rs"
  - "kubernetes.md's shipped k8s/ directory documented as a local/CI test fixture (not production-ready), no-shipped-Helm-chart scope note, five manifest path comments corrected to real files, eight corrected to explicit illustrative markers"
  - "production.md's Snyk/docker-scan recommendation replaced with make audit/deny/security/sbom, fabricated OAuth2/Auth0/three-role RBAC replaced with the real bearer-token/x-api-key/two-role mechanism, fabricated circuit_breaker/backoff crate imports replaced with the real first-party CircuitBreaker/RetryPolicy APIs"
affects: [16-05]

# Tech tracking
tech-stack:
  added: []
  patterns:
    - "Reuse the 16-01 D-09 verdict-row shape and eight-class signal battery verbatim across plans"
    - "For deployment docs specifically, extend the battery with an explicit docker/ and k8s/ asset comparison (compose service names, ports, env vars, manifest filenames) alongside the eight mechanical signal classes"
    - "When the shipped reference asset itself (e.g. k8s/deployment.yaml) is a test fixture rather than production content, add one prominent scope note documenting that distinction rather than silently treating the fixture as ground truth"
    - "Classify every paladin-* token as crate-name vs. Kubernetes/Docker object name BEFORE editing (M-06) — in this plan's files, ALL such tokens were object names, zero were crate names"

key-files:
  created: []
  modified:
    - docs/src/deployment/docker.md
    - docs/src/deployment/kubernetes.md
    - docs/src/deployment/production.md
    - .planning/phases/16-documentation-currency-the-architecture-gap/16-DOCS-01-VERDICTS.md

key-decisions:
  - "kubernetes.md's fabrication was comprehensive enough (13 of 13 numbered k8s/NN-name.yaml path comments matched no real file; the entire ~140-line Helm Chart section describes a chart that was never authored) that a full rewrite of every manifest type was judged disproportionate to D-12. Added one prominent scope note under Overview documenting the real k8s/ directory (a local/CI test fixture — paladin:test image, imagePullPolicy: Never, probes disabled) and the absence of any shipped Helm chart; corrected the five manifest path comments with real 1:1 file analogs; marked the eight without a shipped analog as explicitly illustrative rather than fabricating a path."
  - "production.md's Monitoring section (Key Metrics + Alerting Rules) describes an entire Prometheus pipeline that does not exist anywhere in this codebase (no metrics crate dependency, no /metrics route). Rather than deleting the section, added a scope note stating this explicitly and dropped the one line (go_goroutines) that couldn't even apply to a Rust binary, following the kubernetes.md-row precedent for illustrative content describing infrastructure not yet built."
  - "Where a doc's fabricated Rust API had a real, narrower first-party equivalent (circuit_breaker crate -> src/infrastructure/resilience/circuit_breaker.rs; backoff crate -> paladin-battalion's RetryPolicy/retry.rs; RedisConfig/MinioConfig -> QueueConfig/FileStorageConfig), rewrote the snippet to the real API rather than just flagging the fabrication, since a working example was straightforward to produce from the real signatures."

requirements-completed: [DOCS-01]

coverage:
  - id: D1
    description: "docker.md settled by content: stale v0.4.3 tag replaced with v0.8.0 (7 sites), fabricated config.yml/Environment Variables sections corrected against the real Settings struct, fabricated /health response and Docker Health Check section corrected, missing third Dockerfile (Dockerfile.server) documented, docker-publish.yml citation fixed to the real ci.yml/release.yml jobs"
    requirement: "DOCS-01"
    verification:
      - kind: other
        ref: "grep -c 'v0\\.4\\.3' docs/src/deployment/docker.md == 0; grep -c '0\\.8\\.0' docs/src/deployment/docker.md >= 1; mdbook build docs/ exit 0"
        status: pass
    human_judgment: false
  - id: D2
    description: "kubernetes.md settled by content: stale v0.4.3 tag replaced with v0.8.0 (2 sites), every paladin-* token classified before editing (all six are Kubernetes/Helm object names, none are crate names, none touched), a scope note added documenting the shipped k8s/ dir is a CI test fixture and no Helm chart exists, five manifest path comments corrected to real files, readinessProbe path and metrics port corrected"
    requirement: "DOCS-01"
    verification:
      - kind: other
        ref: "grep -c 'v0\\.4\\.3' docs/src/deployment/kubernetes.md == 0; grep -c 'k8s/[0-9]' docs/src/deployment/kubernetes.md == 0; mdbook build docs/ exit 0"
        status: pass
    human_judgment: false
  - id: D3
    description: "production.md settled by content: the removed Snyk scanner and retired docker scan replaced with make audit/deny/security/sbom, fabricated OAuth2/Auth0/three-role RBAC replaced with the real bearer-token/x-api-key/two-role mechanism, fabricated /health/live and /health/ready probe paths corrected to /health and /ready, fabricated circuit_breaker/backoff crate imports replaced with the real first-party APIs; closes the four-file docs/src/deployment/ group"
    requirement: "DOCS-01"
    verification:
      - kind: other
        ref: "grep -c 'snyk\\|docker scan' docs/src/deployment/production.md == 0; grep -c '/health/live\\|/health/ready' docs/src/deployment/production.md == 0; mdbook build docs/ exit 0"
        status: pass
    human_judgment: false
  - id: D4
    description: "Three evidence-bearing verdict rows appended to 16-DOCS-01-VERDICTS.md in the declared row order, replacing three pending rows; four of fourteen remain explicitly pending for 16-05"
    requirement: "DOCS-01"
    verification:
      - kind: other
        ref: "grep -c 'pending — not yet checked' 16-DOCS-01-VERDICTS.md == 5 after Task 1, 4 after Task 2; grep -c '^| docs/src/' == 14 throughout"
        status: pass
    human_judgment: false

duration: ~55min
completed: 2026-08-24
status: complete
---

# Phase 16 Plan 04: Deployment Currency Sweep (docker.md + kubernetes.md + production.md) Summary

**Closed the four-file `docs/src/deployment/` group against the live 0.8.0 tree, finding the phase's densest structural fabrication yet — a shipped `k8s/` directory that is actually a local/CI test fixture (not production content) misrepresented as a deployable manifest set, an entire unauthored Helm chart, a still-present recommendation for the Snyk scanner this repo explicitly evaluated and removed, and multiple non-existent Rust crate imports (`circuit_breaker`, `backoff`, `axum::Server`) — all corrected against the real tree with inline evidence.**

## Performance

- **Duration:** ~55 min
- **Started:** 2026-08-24 (approx, first tool call after reading 16-01/16-03/16-DOCS-01-VERDICTS.md)
- **Completed:** 2026-08-24
- **Tasks:** 2
- **Files modified:** 4 (3 doc pages, 1 verdict record)

## Accomplishments

- Ran the full eight-class D-09 signal battery plus the plan's mandated `docker/`/`k8s/` asset comparison and end-to-end prose review against `docker.md`, `kubernetes.md`, and `production.md`, recording every producing command and result
- Replaced the stale pre-0.8.0 release tag `v0.4.3` with `v0.8.0` across both files (7 sites in `docker.md`, 2 in `kubernetes.md`)
- Classified all `paladin-*` tokens in `kubernetes.md` (6 distinct: `paladin-chart`, `paladin-config`, `paladin-data`, `paladin-quota`, `paladin-secrets`, `paladin-tls`) and `docker.md` (6 distinct) *before* editing per M-06 — every one is a Kubernetes/Docker object name, zero are crate names; none touched
- `docker.md`: corrected the fabricated `config.yml` example (no `paladin:` top-level section exists; `garrison.type`→`garrison_type`; `arsenal.mcp_servers[].type`→`server_type`; `storage:`→`file_storage:`; `queue:`→ real `redis_host`/`redis_port` fields) against the live `Settings` struct; corrected the Environment Variables section (missing required `APP_` prefix on Garrison vars, `LOG_LEVEL`→`RUST_LOG`, removed nonexistent `SERVER_HOST`/`SERVER_PORT`/`DEFAULT_MODEL`/`DEFAULT_TEMPERATURE`/`DEFAULT_MAX_LOOPS` overrides — none has any config path); corrected the fabricated `/health` JSON response (real route returns only `{"status":"ok"}`, with a separate undocumented `/ready` route); documented the missing third Dockerfile (`Dockerfile.server`, builds `paladin-server`); fixed the nonexistent `.github/workflows/docker-publish.yml` citation to the real `ci.yml`/`release.yml` jobs; fixed the metrics port (8081→9090, matching the live Dockerfile/compose) with a note that no `/metrics` handler is wired up yet
- `kubernetes.md`: discovered the shipped `k8s/deployment.yaml` is a **local/CI test fixture** (`image: paladin:test`, `imagePullPolicy: Never`, a `sleep 3600` placeholder command, probes commented out "Disabled for testing"), not production content — added a prominent scope note documenting this and the fact that no Helm chart exists anywhere in the repository (the entire "Helm Chart" section and "Using Helm" quick-start block describe an unauthored chart); corrected the five manifest path comments that have real 1:1 shipped analogs (`k8s/namespace.yaml`, `k8s/deployment.yaml`, `k8s/service.yaml`, `k8s/configmap.yaml`, `k8s/secret.yaml.example`) and marked the eight without a shipped analog (Ingress/ResourceQuota/PDB/HPA/PVC/NetworkPolicy/ServiceMonitor/RBAC) as explicitly illustrative rather than citing a fabricated path; applied the same `Settings`-struct field-name fixes to the ConfigMap example; fixed the readinessProbe path (`/health/ready`→`/ready`) and metrics ports
- `production.md`: removed the recommendation for `docker scan` (retired from the Docker CLI) and `snyk container test` (the exact scanner `.github/instructions/security.instructions.md` records as evaluated and removed on 2026-08-18 for zero Rust coverage), replaced with `make audit`/`make deny`/`make security`/`make sbom`; replaced a fabricated OAuth2/Auth0 + three-role RBAC YAML block with the real bearer-token/`x-api-key`/two-role (`Admin`/`User`) mechanism in `crates/paladin-web/src/agent_auth.rs`; fixed fabricated `/health/live`/`/health/ready` probe paths to the real `/health`/`/ready`; replaced fabricated `circuit_breaker`/`backoff` external-crate imports (neither is a dependency anywhere in this workspace) with the real first-party `CircuitBreaker`/`RetryPolicy` APIs; fixed `axum::Server::bind` (removed in Axum 0.7+; this workspace pins 0.8.4) to the live `axum::serve` pattern; fixed `RedisConfig`/`MinioConfig` to the real `QueueConfig`/`FileStorageConfig`; added a scope note on the Monitoring section documenting that no Prometheus metrics pipeline exists in this codebase at all
- Wrote three evidence-bearing verdict rows into `16-DOCS-01-VERDICTS.md`, replacing `pending` rows in place, preserving the declared row order; four of fourteen rows remain explicitly pending for 16-05 — this closes the four-file `docs/src/deployment/` group (Milestone 11 task 7.0, deployment half)
- `mdbook build docs/` exits 0 after every task (`mdbook-mermaid install docs/` was needed once, this being a fresh worktree with no gitignored mermaid assets yet; `docs/book.toml` confirmed unchanged)

## Task Commits

1. **Task 1: Sweep docker.md and kubernetes.md, clearing the stale release tag and classifying every paladin-* token** - `5b71bf64` (fix)
2. **Task 2: Sweep production.md against the 0.8.0 tree** - `98b81e7e` (fix)

**Plan metadata:** (this commit)

## Files Created/Modified

- `docs/src/deployment/docker.md` - Fixed stale version tags, fabricated config.yml/env vars, fabricated /health response, missing third Dockerfile, fabricated docker-publish.yml workflow citation
- `docs/src/deployment/kubernetes.md` - Fixed stale version tags, added the CI-test-fixture/no-Helm-chart scope note, corrected 5 manifest path comments to real files and marked 8 as explicitly illustrative, fixed the ConfigMap example, readinessProbe path, and metrics ports
- `docs/src/deployment/production.md` - Fixed the Snyk/docker-scan security-tooling recommendation, fabricated OAuth2/Auth0/RBAC auth section, fabricated health-probe paths, fabricated circuit_breaker/backoff crate imports, axum::Server API, RedisConfig/MinioConfig, and added a Monitoring scope note
- `.planning/phases/16-documentation-currency-the-architecture-gap/16-DOCS-01-VERDICTS.md` - Appended three evidence-bearing verdict rows (modified)

## Decisions Made

- `kubernetes.md`'s fabrication was comprehensive enough (13 of 13 numbered `k8s/NN-name.yaml` path comments matched no real file; the entire ~140-line Helm Chart section describes a chart never authored) that a full rewrite of every manifest type was judged disproportionate to D-12. Added one prominent scope note under Overview documenting the real `k8s/` directory (a local/CI test fixture) and the absence of any shipped Helm chart; corrected the five manifest path comments with real 1:1 file analogs; marked the eight without a shipped analog as explicitly illustrative rather than fabricating a path.
- `production.md`'s Monitoring section (Key Metrics + Alerting Rules) describes an entire Prometheus pipeline that does not exist anywhere in this codebase. Rather than deleting the section, added a scope note stating this explicitly and dropped the one line (`go_goroutines`) that couldn't even apply to a Rust binary, following the kubernetes.md-row precedent for illustrative content describing infrastructure not yet built.
- Where a doc's fabricated Rust API had a real, narrower first-party equivalent (`circuit_breaker` crate → `src/infrastructure/resilience/circuit_breaker.rs`; `backoff` crate → `paladin-battalion`'s `RetryPolicy`/`retry.rs`; `RedisConfig`/`MinioConfig` → `QueueConfig`/`FileStorageConfig`), rewrote the snippet to the real API rather than just flagging the fabrication, since a working example was straightforward to produce from the real signatures.

## Deviations from Plan

None beyond the plan's own explicitly-scoped auto-fix mandate — every correction above is a Rule 1 (bug: doc content that doesn't match the live tree) fix within the plan's own `<action>` instruction to run the signal battery, the mandated `docker/`/`k8s/` asset comparison, and (for `production.md`) the mandated security-tooling and runtime-surface checks, correcting only what the checks found. No architectural changes, no new dependencies, no files touched outside the three named pages and the verdict record.

## Issues Encountered

- The sandboxed Bash tool rejected several multi-statement / loop-containing commands as "too complex to verify [they stay] inside the worktree," consistent with 16-02/16-03's notes — worked around by splitting into individual single-purpose commands (e.g. per-target `grep -E '^<target>:' Makefile` calls instead of a loop).
- This worktree had no gitignored `docs/mermaid.min.js`/`docs/mermaid-init.js` yet (fresh checkout); ran `mdbook-mermaid install docs/` once before the first build and confirmed `docs/book.toml` was unchanged, per the plan's own guidance.
- `kubernetes.md`'s divergence from the real `k8s/` tree was deeper than a naive path-rename: the *shape* of the real reference asset (a CI test fixture, not production content) undermined the doc's basic premise that `kubectl apply -f k8s/` deploys a working service. This required a structural scope note rather than a line-by-line fix, resolved per the scope-boundary decision documented above.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- The D-09 verdict record now carries nine settled rows (`cicd.md` from 16-01; six `user-guides/` files from 16-02/16-03; `docker.md`, `kubernetes.md`, `production.md` from this plan) — the whole `docs/src/deployment/` group (Milestone 11 task 7.0) is closed. Four still-`pending` rows remain for 16-05 (`docs/src/operations/`: `logging.md`, `monitoring.md`, `performance-tuning.md`, `troubleshooting.md`).
- `kubernetes.md`'s scope note (real `k8s/` = CI test fixture, no shipped Helm chart) and `production.md`'s Monitoring scope note (no Prometheus pipeline implemented) are documented in this SUMMARY and in the verdict rows' Findings cells — any future plan revisiting either file, or implementing real Kubernetes production manifests / a metrics pipeline, should check there first.
- No blockers for 16-05.

---
*Phase: 16-documentation-currency-the-architecture-gap*
*Completed: 2026-08-24*

## Self-Check: PASSED

All 4 claimed files verified present on disk (`docs/src/deployment/docker.md`,
`docs/src/deployment/kubernetes.md`, `docs/src/deployment/production.md`, this SUMMARY). Both
commit hashes (`5b71bf64`, `98b81e7e`) verified present in `git log --oneline --all`.
