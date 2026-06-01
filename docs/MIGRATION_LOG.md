# Documentation Migration Log

**Epic:** Milestone 11, Epic 2 — MDBook Setup and Structure  
**Date:** 2026-06-01  
**Branch:** `feature/milestone-11-epic-2-mdbook-setup`

This log records every file migration decision made during the transition from the flat `docs/` directory structure to the MDBook chapter hierarchy under `docs/src/`.

---

## Open Questions Resolved

| OQ | Question | Decision |
|----|----------|----------|
| OQ-1 | `docs/README.md` vs fresh `introduction.md` | Migrated `docs/README.md` via `git mv` — content is a well-structured overview of all sections, suitable as the MDBook introduction. |
| OQ-2 | Which file becomes `architecture/overview.md` | Used `docs/architecture/overview.md` (concise, current); `docs/Design/Design_and_Architecture.md` → appendix as the comprehensive design reference. |
| OQ-3 | CHANGELOG.md inclusion in MDBook | Not included — CHANGELOG.md remains at repo root; it is a developer/release artifact, not user-facing documentation. |
| OQ-4 | `gh-pages` branch protection blocking `GITHUB_TOKEN` deploy | Configured workflow to use `GITHUB_TOKEN` with `contents: write` permission; no branch protection rules needed for `gh-pages`. |

---

## Getting Started

| Source | Destination | Method | Reason |
|--------|-------------|--------|--------|
| `docs/INSTALLATION.md` | `docs/src/getting-started/installation.md` | `git mv` | Primary Getting Started chapter |
| `docs/QUICKSTART.md` | `docs/src/getting-started/quickstart.md` | `git mv` | Primary Getting Started chapter |
| `docs/CONFIGURATION.md` | `docs/src/getting-started/configuration.md` | `git mv` | Core configuration reference |

---

## Introduction

| Source | Destination | Method | Reason |
|--------|-------------|--------|--------|
| `docs/README.md` | `docs/src/introduction.md` | `git mv` | OQ-1: migrated existing README; contains well-structured overview of all documentation sections |

---

## User Guides

| Source | Destination | Method | Reason |
|--------|-------------|--------|--------|
| `docs/AUTONOMOUS.md` | `docs/src/user-guides/paladin-agents.md` | `git mv` | Core autonomous agent feature guide |
| `docs/BATTALION.md` | `docs/src/user-guides/battalion-patterns.md` | `git mv` | Core orchestration chapter |
| `docs/COMMANDER.md` | `docs/src/user-guides/orchestration.md` | `git mv` | Orchestration patterns (Commander strategy router) |
| `docs/ARSENAL.md` | `docs/src/user-guides/arsenal-tools.md` | `git mv` | Tool integration (MCP/Arsenal) |
| `docs/GARRISON.md` | `docs/src/user-guides/garrison-memory.md` | `git mv` | Memory management chapter |
| `docs/SANCTUM.md` | `docs/src/user-guides/sanctum-vector-memory.md` | `git mv` | Vector memory chapter |
| `docs/HERALD.md` | `docs/src/user-guides/herald-output.md` | `git mv` | Output formatting chapter |
| `docs/MANEUVER.md` | `docs/src/user-guides/maneuver-flow-dsl.md` | `git mv` | Key 🆕 feature guide (Epic 17 Flow DSL); placed in user-guides rather than appendix due to its current and prominent status |
| `docs/guides/paladin-configuration.md` | `docs/src/user-guides/paladin-configuration.md` | `git mv` | Existing guides/ subdir content fits user-guides chapter |
| `docs/guides/memory-management.md` | `docs/src/user-guides/memory-management.md` | `git mv` | Existing guides/ subdir content fits user-guides chapter |
| `docs/guides/tool-integration.md` | `docs/src/user-guides/tool-integration.md` | `git mv` | Existing guides/ subdir content fits user-guides chapter |
| `docs/guides/output-formatting.md` | `docs/src/user-guides/output-formatting.md` | `git mv` | Existing guides/ subdir content fits user-guides chapter |

---

## Architecture

| Source | Destination | Method | Reason |
|--------|-------------|--------|--------|
| `docs/architecture/overview.md` | `docs/src/architecture/overview.md` | `git mv` | OQ-2: chosen as primary overview (concise, structured) |
| `docs/architecture/hexagonal-design.md` | `docs/src/architecture/hexagonal-design.md` | `git mv` | Core architecture chapter |
| `docs/architecture/domain-model.md` | `docs/src/architecture/domain-model.md` | `git mv` | Core architecture chapter |
| `docs/architecture/design-patterns.md` | `docs/src/architecture/design-patterns.md` | `git mv` | Core architecture chapter |
| `docs/architecture/dependency-flow-diagrams.md` | `docs/src/architecture/crate-map.md` | `git mv` | Renamed to `crate-map.md` — better reflects the content (crate dependency diagrams) |

---

## Deployment

| Source | Destination | Method | Reason |
|--------|-------------|--------|--------|
| `docs/deployment/docker.md` | `docs/src/deployment/docker.md` | `git mv` | Direct chapter mapping |
| `docs/deployment/kubernetes.md` | `docs/src/deployment/kubernetes.md` | `git mv` | Direct chapter mapping |
| `docs/deployment/production-best-practices.md` | `docs/src/deployment/production.md` | `git mv` | Renamed to `production.md` for brevity |
| `docs/deployment/cicd.md` | `docs/src/deployment/cicd.md` | `git mv` | Direct chapter mapping |

---

## Operations

| Source | Destination | Method | Reason |
|--------|-------------|--------|--------|
| `docs/operations/logging.md` | `docs/src/operations/logging.md` | `git mv` | Direct chapter mapping |
| `docs/operations/monitoring.md` | `docs/src/operations/monitoring.md` | `git mv` | Direct chapter mapping |
| `docs/operations/performance-tuning.md` | `docs/src/operations/performance-tuning.md` | `git mv` | Direct chapter mapping |
| `docs/operations/troubleshooting.md` | `docs/src/operations/troubleshooting.md` | `git mv` | Direct chapter mapping |

---

## API Reference

| Source | Destination | Method | Reason |
|--------|-------------|--------|--------|
| `docs/FEATURE_FLAGS.md` | `docs/src/api-reference/feature-flags.md` | `git mv` | Feature flag reference |
| `docs/MIGRATION.md` | `docs/src/api-reference/migration-guide.md` | `git mv` | API migration guide |
| `STABLE_API.md` + `docs/VERSIONING_POLICY.md` | `docs/src/api-reference/stable-api.md` | merge (concat) | Two related docs merged; `STABLE_API.md` is the primary content; `VERSIONING_POLICY.md` appended with an H2 separator (`## Versioning Policy`); both source files removed via `git rm` |

---

## Contributing

| Source | Destination | Method | Reason |
|--------|-------------|--------|--------|
| `CONTRIBUTING.md` (root) | `docs/src/contributing/development-setup.md` | `git mv` | Primary contributor guide at repo root |
| `docs/contributing/testing-guide.md` | `docs/src/contributing/testing-guide.md` | `git mv` | Direct chapter mapping |
| `docs/contributing/adapter-development.md` | `docs/src/contributing/architecture-decisions.md` | `git mv` | Renamed — content covers adapter architecture decisions |
| `docs/CONTRIBUTING_PROVIDERS.md` | `docs/src/contributing/contributing-providers.md` | `git mv` | Provider contribution guide |

---

## Appendix

All files below were placed in `docs/src/appendix/` — they are valid reference material but do not map to a primary chapter (operational, benchmarking, CLI tooling, or historical design docs).

| Source | Destination | Method | Reason for appendix placement |
|--------|-------------|--------|-------------------------------|
| `docs/GROVE.md` | `docs/src/appendix/grove.md` | `git mv` | Specialized pattern; no primary chapter slot |
| `docs/COUNCIL.md` | `docs/src/appendix/council.md` | `git mv` | Specialized orchestration variant; no primary chapter slot |
| `docs/SENTINEL.md` | `docs/src/appendix/sentinel.md` | `git mv` | Specialized component; no primary chapter slot |
| `docs/guides/conclave-pattern.md` | `docs/src/appendix/conclave-pattern.md` | `git mv` | Specialized pattern guide; no primary chapter slot |
| `docs/guides/battalion-patterns.md` | `docs/src/appendix/battalion-patterns-guide.md` | `git mv` | Supplementary guide (main battalion doc already in user-guides) |
| `docs/guides/flow-dsl.md` | `docs/src/appendix/flow-dsl-guide.md` | `git mv` | Supplementary guide (main Maneuver doc already in user-guides) |
| `docs/CLI_USAGE.md` | `docs/src/appendix/cli-usage.md` | `git mv` | CLI reference; no primary chapter slot |
| `docs/USER_SYSTEM.md` | `docs/src/appendix/user-system.md` | `git mv` | User system docs; no primary chapter slot |
| `docs/user_rest_api_usage.md` | `docs/src/appendix/user-rest-api.md` | `git mv` | REST API usage reference; no primary chapter slot |
| `docs/PROVIDER_EXPANSION.md` | `docs/src/appendix/provider-expansion.md` | `git mv` | Provider expansion guide; no primary chapter slot |
| `docs/BATTALION_VISION_SUPPORT.md` | `docs/src/appendix/battalion-vision-support.md` | `git mv` | Feature support matrix; reference material |
| `docs/INTEGRATION_TESTS.md` | `docs/src/appendix/integration-tests.md` | `git mv` | Testing operational docs; appendix reference |
| `docs/SECURITY_SCANNING.md` | `docs/src/appendix/security-scanning.md` | `git mv` | Security tooling docs; operational reference |
| `docs/BRANCH_PROTECTION.md` | `docs/src/appendix/branch-protection.md` | `git mv` | Git operations; operational reference |
| `docs/BUILD_BASELINES.md` | `docs/src/appendix/build-baselines.md` | `git mv` | CI baseline metrics; reference |
| `docs/PERFORMANCE_BASELINE.md` | `docs/src/appendix/performance-baseline.md` | `git mv` | Performance metrics; reference |
| `docs/BATTALION_BENCHMARKS.md` | `docs/src/appendix/battalion-benchmarks.md` | `git mv` | Benchmark results; reference |
| `docs/SANCTUM_BENCHMARKS.md` | `docs/src/appendix/sanctum-benchmarks.md` | `git mv` | Benchmark results; reference |
| `docs/SANCTUM_DEPLOYMENT.md` | `docs/src/appendix/sanctum-deployment.md` | `git mv` | Sanctum-specific deployment; supplementary |
| `docs/SANCTUM_MIGRATION.md` | `docs/src/appendix/sanctum-migration.md` | `git mv` | Sanctum migration guide; supplementary |
| `docs/RELEASE_AUTOMATION.md` | `docs/src/appendix/release-automation.md` | `git mv` | Release tooling docs; operational reference |
| `docs/RELEASE_CHECKLIST.md` | `docs/src/appendix/release-checklist.md` | `git mv` | Release process; operational reference |
| `docs/DOC_COVERAGE_REPORT.md` | `docs/src/appendix/doc-coverage-report.md` | `git mv` | Documentation metrics; reference |
| `docs/port-trait-doc-template.md` | `docs/src/appendix/port-trait-template.md` | `git mv` | Developer template; reference |
| `docs/Design/Design_and_Architecture.md` | `docs/src/appendix/design-and-architecture.md` | `git mv` | OQ-2: comprehensive design doc superseded by `architecture/overview.md` for primary chapter; preserved as full reference in appendix |
| `docs/Design/minio-file-repository-setup.md` | `docs/src/appendix/minio-file-repository-setup.md` | `git mv` | Infrastructure setup notes; no primary chapter slot |
| `docs/Design/redis-queue-adapter-setup.md` | `docs/src/appendix/redis-queue-adapter-setup.md` | `git mv` | Infrastructure setup notes; no primary chapter slot |
| `docs/cli/CONFIGURATION.md` | `docs/src/appendix/cli-configuration.md` | `git mv` | CLI tool docs; reference |
| `docs/cli/COUNCIL.md` | `docs/src/appendix/cli-council.md` | `git mv` | CLI tool docs; reference |
| `docs/cli/MUSTER.md` | `docs/src/appendix/cli-muster.md` | `git mv` | CLI tool docs; reference |
| `docs/cli/ONBOARDING.md` | `docs/src/appendix/cli-onboarding.md` | `git mv` | CLI tool docs; reference |
| `docs/cli/SETUP_CHECK.md` | `docs/src/appendix/cli-setup-check.md` | `git mv` | CLI tool docs; reference |
| `docs/cli/TESTING.md` | `docs/src/appendix/cli-testing.md` | `git mv` | CLI tool docs; reference |
| `docs/contributing/CONTRIBUTING.md` | `docs/src/appendix/contributing-legacy.md` | `git mv` | Legacy contributing file in docs/contributing/; superseded by root CONTRIBUTING.md (now at docs/src/contributing/development-setup.md); kept for historical reference |

---

## Assets

| Source | Destination | Method | Reason |
|--------|-------------|--------|--------|
| `docs/Design/assets/*.svg` (6 files) | `docs/src/assets/` | `git mv` | SVG diagram assets must be under `docs/src/` to be served by MDBook |

---

## Files Not Migrated

| File | Reason |
|------|--------|
| `CHANGELOG.md` | OQ-3: developer/release artifact; remains at repo root |
| `README.md` (root) | Repository README; not a docs page |
| `docs/AI-Centric-Softare-Development/` | Contains `.jsx` files (not documentation); outside migration scope |
| `docs/assets/` | Empty directory; no content to migrate |

---

## Summary Statistics

- **Files migrated via `git mv`:** 68
- **Files merged (concatenated):** 2 → 1 (`STABLE_API.md` + `docs/VERSIONING_POLICY.md`)
- **Files deleted (source removed):** 2 (`STABLE_API.md`, `docs/VERSIONING_POLICY.md` via `git rm`)
- **SVG assets moved:** 6
- **Files not migrated:** 4 (CHANGELOG.md, root README.md, AI-Centric dir, empty assets dir)
- **Total files in `docs/src/`:** 71 markdown files

All 124 files audited in Epic 1 that were not flagged for deletion appear somewhere in the MDBook structure.
