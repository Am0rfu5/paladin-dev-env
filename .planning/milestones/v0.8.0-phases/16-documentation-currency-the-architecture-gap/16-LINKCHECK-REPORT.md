# 16-01: Local `mdbook build docs/` — Linkcheck Report (D-10)

**Date:** 2026-08-24T12:19:20Z – 2026-08-24T12:20:33Z
**Working directory:** `/workspace/.claude/worktrees/agent-a13b41d049eb8d646` (repo root)
**Command:** `mdbook build docs/`
**Toolchain (all three at CI's exact pins, `.github/workflows/docs.yml:44-54`):**

```
$ mdbook --version
mdbook v0.4.40
$ mdbook-mermaid --version
mdbook-mermaid 0.13.0
$ mdbook-linkcheck --version
mdbook-linkcheck 0.7.7
```

`docs/mermaid.min.js` and `docs/mermaid-init.js` were missing before this run (both are
`.gitignore`d generated assets, `.gitignore:20-22`), so `mdbook-mermaid install docs/` was run
once, first, per the plan's read-once-if-missing rule. `docs/book.toml` was diffed immediately
after and is byte-identical to its pre-install state (`git diff --exit-code docs/book.toml`
exits 0) — the preprocessor and `additional-js` entries it needs were already present.

## Run 1 — first `mdbook build docs/`, FAILED (exit 101)

This is the first time this exact command has been run against the pinned toolchain. It found
a real, pre-existing broken link that no CI run had caught, because `docs.yml`'s `Build MDBook`
job runs the identical command and would have failed identically had a docs-touching PR ever
exercised it against this file. The fix is recorded below and re-verified in Run 2.

```
2026-08-24 12:19:20 [INFO] (mdbook::book): Book building has started
Warning: The mdbook-mermaid preprocessor was built against version 0.4.36 of mdbook, but we're being called from version 0.4.40
2026-08-24 12:19:20 [INFO] (mdbook::book): Running the html backend
2026-08-24 12:19:22 [INFO] (mdbook::book): Running the linkcheck backend
2026-08-24 12:19:22 [INFO] (mdbook::renderer): Invoking the "linkcheck" renderer
[2026-08-24T12:19:22Z INFO  mdbook_linkcheck] Started the link checker
[2026-08-24T12:19:22Z INFO  mdbook_linkcheck] Scanning book for links
[2026-08-24T12:19:22Z INFO  mdbook_linkcheck] Found 896 links (0 incomplete links)
[2026-08-24T12:19:22Z WARN  linkcheck::validation::filesystem] Not checking that the "overview" section exists in "/workspace/.claude/worktrees/agent-a13b41d049eb8d646/docs/src/user-guides/tool-integration.md" because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "embedding-in-your-own-app" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "embedding-in-your-own-app" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "overview" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "configuration" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "log-levels" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "structured-logging" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "log-aggregation" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "log-analysis" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "best-practices" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "overview" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "metrics-collection" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "prometheus-setup" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "grafana-dashboards" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "alerting" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "key-metrics" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "distributed-tracing" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "health-checks" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "performance-baselines" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "benchmarking" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "llm-optimization" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "memory-optimization" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "concurrency-tuning" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "database-optimization" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "network-optimization" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "resource-allocation" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "diagnostic-tools" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "common-issues" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "performance-issues" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "configuration-issues" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "deployment-issues" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "integration-issues" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "getting-help" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "overview" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "quick-start" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "routing-strategies" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "expertise-definition" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "fallback-behavior" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "configuration" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "examples" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "best-practices" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "api-reference" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "quick-start" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "overview" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "quick-start" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "turn-taking-strategies" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "termination-conditions" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "garrison-integration" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "configuration" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "examples" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "best-practices" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "api-reference" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "quick-start" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "introduction" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "getting-started" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "vision-content-types" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "supported-providers" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "paladin-vision-api" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "document-processing" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "cli-usage" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "yaml-configuration" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "security" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "battalion-integration" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "error-handling" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "performance-considerations" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "troubleshooting" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "overview" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "quick-start" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "configuration" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "programmatic-api" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "yaml-configuration" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "cli-usage" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "use-cases" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "error-handling" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "observability" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "best-practices" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "troubleshooting" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "overview" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "formation-sequential" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "phalanx-parallel" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "campaign-graphdag" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "chain-of-command-hierarchical" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "pattern-selection-guide" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "common-pitfalls" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "performance-considerations" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "introduction" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "motivation" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "quick-start" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "syntax-reference" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "error-handling-strategies" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "visualization" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "best-practices" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "troubleshooting" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "performance-considerations" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "examples" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "quick-start" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "installation" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "environment-setup" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "getting-started" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "paladin-onboarding" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "paladin-setup-check" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "paladin-features" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "commands-reference" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "paladin-agent" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "paladin-battalion" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "paladin-muster" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "paladin-council" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "paladin-maneuver" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "paladin-arsenal" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "configuration-files" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "examples" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "troubleshooting" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "overview" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "provider-comparison" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "configuration-guide" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "use-case-recommendations" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "migration-guide" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "performance-characteristics" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "prerequisites" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "local-development" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "docker-compose" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "kubernetes" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "cloud-deployments" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "production-best-practices" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "monitoring" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "backup-and-recovery" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "migration-scenarios" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "inmemory-to-qdrant-migration" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "qdrant-version-upgrades" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "changing-vector-dimensions" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "zero-downtime-migration" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "rollback-procedures" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "data-validation" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "troubleshooting" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "executive-summary" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "architecture-overview" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "design-principles" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "system-architecture" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "core-components" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "data-flow" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "implementation-guidelines" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "security-considerations" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "deployment-architecture" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "future-considerations" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "5-use-cases-initial" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "overview" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "configuration-file-structure" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "garrison-configuration-memory" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "arsenal-configuration-tools" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "scheduler-configuration" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "complete-configuration-examples" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "environment-variables" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "troubleshooting" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "overview" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "quick-start" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "command-syntax" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "agent-roles" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "discussion-modes" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "output-options" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "best-practices" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "examples" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "troubleshooting" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "overview" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "quick-start" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "command-syntax" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "generation-workflow" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "configuration-options" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "output-formats" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "best-practices" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "examples" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "troubleshooting" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "cli-snapshot-testing" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "code-of-conduct" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "getting-started" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "development-workflow" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "architecture-guidelines" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "testing-requirements" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "documentation-standards" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "pull-request-process" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "community" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "overview" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "prerequisites" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "quick-start" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "docker-images" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "configuration" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "environment-variables" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "volumes-and-persistence" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "networking" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "multi-container-setup" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "multi-architecture-support" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "image-versioning" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "health-checks" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "resource-limits" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "production-deployment" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "troubleshooting" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "overview" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "prerequisites" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "quick-start" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "architecture" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "kubernetes-manifests" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "configmaps-and-secrets" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "helm-chart" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "resource-management" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "high-availability" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "horizontal-scaling" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "storage" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "networking" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "monitoring" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "security" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "troubleshooting" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "pre-deployment-checklist" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "security" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "performance" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "reliability" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "monitoring" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "disaster-recovery" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "cost-optimization" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "maintenance" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "overview" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "github-actions-workflows" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "ci-pipeline" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "docker-build-pipeline" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "release-pipeline" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "integration-testing" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "security-scanning" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "deployment-automation" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "best-practices" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "quick-start" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "paladinbuilder-api" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "execution-model" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "paladinresult-fields" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "stopreason-variants" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "autonomous-features" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "memory--garrison" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "tools--arsenal" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "output-formatting--herald" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "configuration-reference" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "error-handling" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "best-practices" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "overview" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "quick-start" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "the-eight-patterns" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "formation--sequential" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "phalanx--concurrent" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "campaign--graphdag" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "chain-of-command--hierarchical" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "conclave--mixture-of-experts" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "council--collaborative-discussion" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "grove--semantic-routing" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "maneuver--flow-dsl" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "commander--strategy-router" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "error-handling" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "performance-notes" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "best-practices" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "workflow-patterns-overview" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "formation--sequential" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "phalanx--parallel" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "campaign--graph--dag" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "chain-of-command--hierarchical" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "commander--dynamic-strategy-routing" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "job-scheduling" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "event-and-trigger-system" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "configuration-reference" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "see-also" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "configuration-reference" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation::filesystem] Not checking that the "paladin-content" section exists in "/workspace/.claude/worktrees/agent-a13b41d049eb8d646/docs/src/api-reference/crate-map.md" because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "content-ingestion-sources" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "aggregation-and-the-processing-pipeline" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "content--agent-bridge" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "content-delivery" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "capabilities-and-limitations" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "see-also" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "capabilities-and-limitations" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation::filesystem] Not checking that the "use-case-recipes" section exists in "/workspace/.claude/worktrees/agent-a13b41d049eb8d646/docs/src/user-guides/agent-orchestrator-bridge.md" because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation::filesystem] Not checking that the "paladin-content" section exists in "/workspace/.claude/worktrees/agent-a13b41d049eb8d646/docs/src/api-reference/crate-map.md" because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "agents-triggering-orchestration" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "orchestration-invoking-agents" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "configuring-the-bridge" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "use-case-recipes" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "see-also" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation::filesystem] Not checking that the "configuration-reference" section exists in "/workspace/.claude/worktrees/agent-a13b41d049eb8d646/docs/src/user-guides/orchestration.md" because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation::filesystem] Not checking that the "job-scheduling" section exists in "/workspace/.claude/worktrees/agent-a13b41d049eb8d646/docs/src/user-guides/orchestration.md" because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation::filesystem] Not checking that the "event-and-trigger-system" section exists in "/workspace/.claude/worktrees/agent-a13b41d049eb8d646/docs/src/user-guides/orchestration.md" because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation::filesystem] Not checking that the "paladin-ports" section exists in "/workspace/.claude/worktrees/agent-a13b41d049eb8d646/docs/src/api-reference/crate-map.md" because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "concepts" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "quick-start--stdio-server" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "streamable-http-server-configuration" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "configyml-reference" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "arsenalport-trait" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "arsenalregistry-trait" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "attaching-arsenal-to-a-paladin" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "custom-armaments-direct-rust-tools" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "handoff-tool" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "error-handling" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "best-practices" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "concepts" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "quick-start" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "garrison-adapters" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "garrisonport-trait" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "garrisonconfig" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "conversation-roles" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "attaching-to-a-paladin" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "long-term-memory-with-embeddings" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "configyml-reference" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "error-handling" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "best-practices" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "sanctum-vs-garrison" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "quick-start" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "sanctum-adapters" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "sanctumport-trait" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "sanctumentry-and-memory-types" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "searching-with-sanctumquery" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "rag--retrieval-augmented-generation" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "attaching-to-a-paladin" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "docker-setup-qdrant" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "configyml-reference" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "error-handling" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "best-practices" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "overview" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "available-heralds" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "herald-trait" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "attaching-to-a-paladin" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "attaching-to-a-battalion-service" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "custom-herald-implementation" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "streaming-output" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "error-handling" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "overview" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "quick-start" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "flow-dsl-syntax" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "execution-patterns" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "configuration" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "cli-commands" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "visualization" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "error-handling" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "performance" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "best-practices" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "api-reference" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "troubleshooting" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "basic-configuration" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "system-prompt-best-practices" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "model-selection" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "temperature-and-sampling" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "stop-words-and-termination" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "timeout-and-retry-settings" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "advanced-configuration" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "overview" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "garrison-architecture" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "in-memory-garrison" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "persistent-garrison" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "memory-windowing" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "semantic-search" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "memory-types" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "best-practices" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "advanced-patterns" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "troubleshooting" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "overview" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "arsenal-architecture" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "mcp-protocol" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "stdio-tool-servers" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "streamable-http-tool-servers" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "custom-tool-development" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "tool-result-handling" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "best-practices" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "troubleshooting" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "overview" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "herald-architecture" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "built-in-formatters" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "custom-formatters" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "streaming-output" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "multi-format-output" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "post-processing" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "best-practices" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "advanced-patterns" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "troubleshooting" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "overview" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "available-feature-flags" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "default-configuration" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "usage-examples" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "build-comparison" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "feature-dependencies" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "best-practices" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "migrating-to-v050-from-v04x" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "migrating-to-v04x-from-v03x" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "migrating-to-v020-from-v01x" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "migrating-to-v010-feature-flag-reorganization" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "migration-scenarios" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "testing-your-migration" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "introduction" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "api-stability-guarantee" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "versioning-policy" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "stability-tiers" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "per-crate-api-surface-and-stability" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "stable-public-api-catalog" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "port-traits-output-ports" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "port-traits-input-ports" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "domain-entities" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "builder-types" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "configuration-types" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "error-types" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "base-types" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "internal-implementation-details-not-stable" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "api-change-process" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "migration-guide-for-breaking-changes" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "tracking-api-changes" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "frequently-asked-questions" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "questions-and-support" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "versioning-policy" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "api-change-process" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "code-of-conduct" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "getting-started" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "git-hooks-pre-commit" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "development-workflow" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "testing-guidelines" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "code-quality-standards" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "documentation" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "releasing" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "adding-a-new-dependency" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "api-change-process" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "pull-request-process" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "community" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "rust-coding-conventions" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "testing-guidelines" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "quick-reference-test-commands" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "testing-philosophy" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "test-organization" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "unit-testing" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "integration-testing" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "functional-testing" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "test-coverage" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "mocking-and-fixtures" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "ci-integration" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "testing-best-practices" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "next-steps" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "test-coverage" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "overview" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "port-architecture" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "llm-adapter-development" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "garrison-adapter-development" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "arsenal-adapter-development" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "citadel-adapter-development" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "testing-adapters" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "publishing-adapters" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "prerequisites" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "implementation-steps" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "adapter-template" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "testing-requirements" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "documentation-requirements" in the current file because fragment resolution isn't implemented
[2026-08-24T12:19:22Z WARN  linkcheck::validation] Not checking "submission-guidelines" in the current file because fragment resolution isn't implemented
error: Linking outside of the "root" directory is forbidden
    ┌─ getting-started/configuration.md:176:11
    │
176 │ above, in [`.env.example`](../../../.env.example) at the repository root. Copy it to
    │           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ Linking outside of the "root" directory is forbidden

[2026-08-24T12:19:22Z INFO  mdbook_linkcheck] 1 broken links found
Error: One or more incorrect links
2026-08-24 12:19:22 [ERROR] (mdbook::renderer): Renderer exited with non-zero return code.
2026-08-24 12:19:22 [ERROR] (mdbook::utils): Error: Rendering failed
2026-08-24 12:19:22 [ERROR] (mdbook::utils): 	Caused By: The "linkcheck" renderer failed
```

**Finding (Rule 1 — bug, auto-fixed):** `mdbook-linkcheck` reported
`error: Linking outside of the "root" directory is forbidden` at
`docs/src/getting-started/configuration.md:176`. The book's root is `docs/src/`
(`docs/book.toml:5`, `src = "src"`); the line linked
`` [`.env.example`](../../../.env.example) ``, a real, existing file
(`ls .env.example` — present at the repository root), but three levels of `../` walk out of
the book's root, which `mdbook-linkcheck`'s default `traverse-parent-directories = false`
forbids by design (not a stale-target problem — the file exists; a book-root-boundary
problem). Fixed by dropping the markdown-link wrapper and keeping the existing inline-code
mention (`` `.env.example` at the repository root``), a one-line content edit with no
`docs/book.toml` change, matching D-12 (mechanical fix, no style rewrite) and preserving the
acceptance criterion that `docs/book.toml` stays byte-identical
(`git diff --exit-code docs/book.toml`).

## Run 2 — `mdbook build docs/` after the fix, PASSED (exit 0)

```
2026-08-24 12:20:31 [INFO] (mdbook::book): Book building has started
Warning: The mdbook-mermaid preprocessor was built against version 0.4.36 of mdbook, but we're being called from version 0.4.40
2026-08-24 12:20:31 [INFO] (mdbook::book): Running the html backend
2026-08-24 12:20:33 [INFO] (mdbook::book): Running the linkcheck backend
2026-08-24 12:20:33 [INFO] (mdbook::renderer): Invoking the "linkcheck" renderer
[2026-08-24T12:20:33Z INFO  mdbook_linkcheck] Started the link checker
[2026-08-24T12:20:33Z INFO  mdbook_linkcheck] Scanning book for links
[2026-08-24T12:20:33Z INFO  mdbook_linkcheck] Found 895 links (0 incomplete links)
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "overview" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "configuration" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "log-levels" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "structured-logging" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "log-aggregation" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "log-analysis" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "best-practices" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "overview" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "metrics-collection" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "prometheus-setup" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "grafana-dashboards" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "alerting" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "key-metrics" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "distributed-tracing" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "health-checks" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "performance-baselines" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "benchmarking" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "llm-optimization" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "memory-optimization" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "concurrency-tuning" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "database-optimization" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "network-optimization" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "resource-allocation" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "diagnostic-tools" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "common-issues" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "performance-issues" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "configuration-issues" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "deployment-issues" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "integration-issues" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "getting-help" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation::filesystem] Not checking that the "overview" section exists in "/workspace/.claude/worktrees/agent-a13b41d049eb8d646/docs/src/user-guides/tool-integration.md" because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "code-of-conduct" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "getting-started" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "git-hooks-pre-commit" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "development-workflow" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "testing-guidelines" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "code-quality-standards" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "documentation" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "releasing" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "adding-a-new-dependency" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "api-change-process" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "pull-request-process" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "community" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "rust-coding-conventions" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "testing-guidelines" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "quick-reference-test-commands" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "testing-philosophy" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "test-organization" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "unit-testing" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "integration-testing" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "functional-testing" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "test-coverage" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "mocking-and-fixtures" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "ci-integration" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "testing-best-practices" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "next-steps" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "test-coverage" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "overview" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "port-architecture" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "llm-adapter-development" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "garrison-adapter-development" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "arsenal-adapter-development" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "citadel-adapter-development" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "testing-adapters" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "publishing-adapters" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "prerequisites" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "implementation-steps" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "adapter-template" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "testing-requirements" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "documentation-requirements" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "submission-guidelines" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "overview" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "quick-start" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "routing-strategies" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "expertise-definition" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "fallback-behavior" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "configuration" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "examples" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "best-practices" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "api-reference" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "quick-start" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "overview" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "quick-start" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "turn-taking-strategies" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "termination-conditions" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "garrison-integration" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "configuration" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "examples" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "best-practices" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "api-reference" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "quick-start" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "introduction" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "getting-started" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "vision-content-types" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "supported-providers" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "paladin-vision-api" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "document-processing" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "cli-usage" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "yaml-configuration" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "security" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "battalion-integration" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "error-handling" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "performance-considerations" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "troubleshooting" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "overview" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "quick-start" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "configuration" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "programmatic-api" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "yaml-configuration" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "cli-usage" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "use-cases" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "error-handling" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "observability" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "best-practices" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "troubleshooting" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "overview" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "formation-sequential" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "phalanx-parallel" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "campaign-graphdag" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "chain-of-command-hierarchical" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "pattern-selection-guide" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "common-pitfalls" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "performance-considerations" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "introduction" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "motivation" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "quick-start" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "syntax-reference" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "error-handling-strategies" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "visualization" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "best-practices" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "troubleshooting" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "performance-considerations" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "examples" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "quick-start" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "installation" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "environment-setup" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "getting-started" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "paladin-onboarding" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "paladin-setup-check" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "paladin-features" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "commands-reference" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "paladin-agent" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "paladin-battalion" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "paladin-muster" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "paladin-council" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "paladin-maneuver" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "paladin-arsenal" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "configuration-files" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "examples" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "troubleshooting" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "overview" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "provider-comparison" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "configuration-guide" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "use-case-recommendations" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "migration-guide" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "performance-characteristics" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "prerequisites" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "local-development" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "docker-compose" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "kubernetes" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "cloud-deployments" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "production-best-practices" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "monitoring" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "backup-and-recovery" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "migration-scenarios" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "inmemory-to-qdrant-migration" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "qdrant-version-upgrades" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "changing-vector-dimensions" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "zero-downtime-migration" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "rollback-procedures" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "data-validation" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "troubleshooting" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "executive-summary" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "architecture-overview" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "design-principles" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "system-architecture" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "core-components" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "data-flow" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "implementation-guidelines" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "security-considerations" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "deployment-architecture" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "future-considerations" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "5-use-cases-initial" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "overview" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "configuration-file-structure" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "garrison-configuration-memory" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "arsenal-configuration-tools" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "scheduler-configuration" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "complete-configuration-examples" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "environment-variables" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "troubleshooting" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "overview" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "quick-start" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "command-syntax" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "agent-roles" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "discussion-modes" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "output-options" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "best-practices" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "examples" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "troubleshooting" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "overview" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "quick-start" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "command-syntax" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "generation-workflow" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "configuration-options" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "output-formats" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "best-practices" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "examples" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "troubleshooting" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "cli-snapshot-testing" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "code-of-conduct" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "getting-started" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "development-workflow" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "architecture-guidelines" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "testing-requirements" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "documentation-standards" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "pull-request-process" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "community" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "embedding-in-your-own-app" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "embedding-in-your-own-app" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "overview" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "available-feature-flags" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "default-configuration" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "usage-examples" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "build-comparison" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "feature-dependencies" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "best-practices" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "migrating-to-v050-from-v04x" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "migrating-to-v04x-from-v03x" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "migrating-to-v020-from-v01x" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "migrating-to-v010-feature-flag-reorganization" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "migration-scenarios" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "testing-your-migration" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "introduction" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "api-stability-guarantee" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "versioning-policy" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "stability-tiers" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "per-crate-api-surface-and-stability" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "stable-public-api-catalog" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "port-traits-output-ports" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "port-traits-input-ports" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "domain-entities" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "builder-types" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "configuration-types" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "error-types" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "base-types" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "internal-implementation-details-not-stable" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "api-change-process" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "migration-guide-for-breaking-changes" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "tracking-api-changes" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "frequently-asked-questions" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "questions-and-support" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "versioning-policy" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "api-change-process" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "quick-start" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "paladinbuilder-api" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "execution-model" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "paladinresult-fields" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "stopreason-variants" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "autonomous-features" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "memory--garrison" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "tools--arsenal" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "output-formatting--herald" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "configuration-reference" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "error-handling" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "best-practices" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "overview" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "quick-start" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "the-eight-patterns" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "formation--sequential" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "phalanx--concurrent" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "campaign--graphdag" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "chain-of-command--hierarchical" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "conclave--mixture-of-experts" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "council--collaborative-discussion" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "grove--semantic-routing" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "maneuver--flow-dsl" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "commander--strategy-router" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "error-handling" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "performance-notes" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "best-practices" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "workflow-patterns-overview" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "formation--sequential" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "phalanx--parallel" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "campaign--graph--dag" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "chain-of-command--hierarchical" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "commander--dynamic-strategy-routing" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "job-scheduling" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "event-and-trigger-system" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "configuration-reference" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "see-also" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "configuration-reference" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation::filesystem] Not checking that the "paladin-content" section exists in "/workspace/.claude/worktrees/agent-a13b41d049eb8d646/docs/src/api-reference/crate-map.md" because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "content-ingestion-sources" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "aggregation-and-the-processing-pipeline" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "content--agent-bridge" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "content-delivery" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "capabilities-and-limitations" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "see-also" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "capabilities-and-limitations" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation::filesystem] Not checking that the "use-case-recipes" section exists in "/workspace/.claude/worktrees/agent-a13b41d049eb8d646/docs/src/user-guides/agent-orchestrator-bridge.md" because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation::filesystem] Not checking that the "paladin-content" section exists in "/workspace/.claude/worktrees/agent-a13b41d049eb8d646/docs/src/api-reference/crate-map.md" because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "agents-triggering-orchestration" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "orchestration-invoking-agents" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "configuring-the-bridge" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "use-case-recipes" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "see-also" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation::filesystem] Not checking that the "configuration-reference" section exists in "/workspace/.claude/worktrees/agent-a13b41d049eb8d646/docs/src/user-guides/orchestration.md" because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation::filesystem] Not checking that the "job-scheduling" section exists in "/workspace/.claude/worktrees/agent-a13b41d049eb8d646/docs/src/user-guides/orchestration.md" because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation::filesystem] Not checking that the "event-and-trigger-system" section exists in "/workspace/.claude/worktrees/agent-a13b41d049eb8d646/docs/src/user-guides/orchestration.md" because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation::filesystem] Not checking that the "paladin-ports" section exists in "/workspace/.claude/worktrees/agent-a13b41d049eb8d646/docs/src/api-reference/crate-map.md" because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "concepts" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "quick-start--stdio-server" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "streamable-http-server-configuration" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "configyml-reference" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "arsenalport-trait" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "arsenalregistry-trait" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "attaching-arsenal-to-a-paladin" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "custom-armaments-direct-rust-tools" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "handoff-tool" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "error-handling" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "best-practices" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "concepts" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "quick-start" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "garrison-adapters" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "garrisonport-trait" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "garrisonconfig" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "conversation-roles" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "attaching-to-a-paladin" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "long-term-memory-with-embeddings" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "configyml-reference" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "error-handling" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "best-practices" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "sanctum-vs-garrison" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "quick-start" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "sanctum-adapters" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "sanctumport-trait" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "sanctumentry-and-memory-types" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "searching-with-sanctumquery" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "rag--retrieval-augmented-generation" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "attaching-to-a-paladin" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "docker-setup-qdrant" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "configyml-reference" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "error-handling" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "best-practices" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "overview" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "available-heralds" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "herald-trait" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "attaching-to-a-paladin" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "attaching-to-a-battalion-service" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "custom-herald-implementation" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "streaming-output" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "error-handling" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "overview" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "quick-start" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "flow-dsl-syntax" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "execution-patterns" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "configuration" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "cli-commands" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "visualization" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "error-handling" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "performance" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "best-practices" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "api-reference" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "troubleshooting" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "basic-configuration" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "system-prompt-best-practices" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "model-selection" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "temperature-and-sampling" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "stop-words-and-termination" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "timeout-and-retry-settings" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "advanced-configuration" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "overview" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "garrison-architecture" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "in-memory-garrison" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "persistent-garrison" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "memory-windowing" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "semantic-search" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "memory-types" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "best-practices" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "advanced-patterns" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "troubleshooting" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "overview" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "arsenal-architecture" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "mcp-protocol" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "stdio-tool-servers" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "streamable-http-tool-servers" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "custom-tool-development" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "tool-result-handling" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "best-practices" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "troubleshooting" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "overview" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "herald-architecture" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "built-in-formatters" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "custom-formatters" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "streaming-output" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "multi-format-output" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "post-processing" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "best-practices" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "advanced-patterns" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "troubleshooting" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "overview" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "prerequisites" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "quick-start" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "docker-images" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "configuration" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "environment-variables" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "volumes-and-persistence" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "networking" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "multi-container-setup" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "multi-architecture-support" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "image-versioning" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "health-checks" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "resource-limits" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "production-deployment" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "troubleshooting" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "overview" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "prerequisites" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "quick-start" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "architecture" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "kubernetes-manifests" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "configmaps-and-secrets" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "helm-chart" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "resource-management" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "high-availability" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "horizontal-scaling" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "storage" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "networking" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "monitoring" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "security" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "troubleshooting" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "pre-deployment-checklist" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "security" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "performance" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "reliability" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "monitoring" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "disaster-recovery" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "cost-optimization" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "maintenance" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "overview" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "github-actions-workflows" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "ci-pipeline" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "docker-build-pipeline" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "release-pipeline" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "integration-testing" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "security-scanning" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "deployment-automation" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z WARN  linkcheck::validation] Not checking "best-practices" in the current file because fragment resolution isn't implemented
[2026-08-24T12:20:33Z INFO  mdbook_linkcheck] No broken links found
```

## Review (D-10 requires the report be reviewed, not that CI's pass/fail signal be cited)

Run 2's linkcheck section reports, verbatim: `Found 895 links (0 incomplete links)` (one fewer
than Run 1's 896 — the fixed line no longer emits a markdown link, so the link-count delta is
itself evidence the edit landed) followed immediately by `No broken links found`. All links are
local intra-book references — no `follow-web-links` activity is possible because
`docs/book.toml:25` sets `follow-web-links = false`, so this run never made an outbound HTTP
request; the check is entirely filesystem/anchor based. `warning-policy = "error"`
(`docs/book.toml:26`) means any warning that reached that policy would have escalated to a
build failure exactly like the one Run 1 hit — Run 2 has zero such escalations. Every
"Not checking ... because fragment resolution isn't implemented" line (448 of them in Run 2,
one fewer set than Run 1) is `mdbook-linkcheck` 0.7.7's own documented limitation: it verifies
the *target file* of an in-book anchor link exists, but does not parse the target's Markdown to
confirm the named heading/anchor itself exists inside it. These are informational `WARN`-level
lines, not build-failing warnings — the toolchain's behavior did not change between runs, only
the one link this task fixed did.

`git diff --exit-code docs/book.toml` and `git diff --exit-code .github/workflows/` both exit
0 after Run 2 — the build (Run 1, Run 2, and the one `mdbook-mermaid install docs/` call before
either) did not rewrite the book config, and D-08 (no CI workflow file touched) holds.

## Run 3 — `mdbook build docs/` after 16-01's `cicd.md` content fix (Task 2), PASSED (exit 0)

Re-run per Task 2's action step, to prove the `cicd.md` currency edit did not break the book.
Command, working directory and toolchain versions are unchanged from Runs 1-2 above.

```
2026-08-24 12:25:41 [INFO] (mdbook::book): Book building has started
Warning: The mdbook-mermaid preprocessor was built against version 0.4.36 of mdbook, but we're being called from version 0.4.40
2026-08-24 12:25:42 [INFO] (mdbook::book): Running the html backend
2026-08-24 12:25:45 [INFO] (mdbook::book): Running the linkcheck backend
2026-08-24 12:25:45 [INFO] (mdbook::renderer): Invoking the "linkcheck" renderer
[2026-08-24T12:25:45Z INFO  mdbook_linkcheck] Started the link checker
[2026-08-24T12:25:45Z INFO  mdbook_linkcheck] Scanning book for links
[2026-08-24T12:25:45Z INFO  mdbook_linkcheck] Found 895 links (0 incomplete links)
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "embedding-in-your-own-app" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "embedding-in-your-own-app" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "overview" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "quick-start" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "routing-strategies" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "expertise-definition" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "fallback-behavior" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "configuration" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "examples" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "best-practices" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "api-reference" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "quick-start" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "overview" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "quick-start" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "turn-taking-strategies" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "termination-conditions" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "garrison-integration" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "configuration" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "examples" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "best-practices" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "api-reference" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "quick-start" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "introduction" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "getting-started" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "vision-content-types" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "supported-providers" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "paladin-vision-api" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "document-processing" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "cli-usage" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "yaml-configuration" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "security" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "battalion-integration" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "error-handling" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "performance-considerations" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "troubleshooting" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "overview" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "quick-start" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "configuration" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "programmatic-api" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "yaml-configuration" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "cli-usage" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "use-cases" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "error-handling" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "observability" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "best-practices" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "troubleshooting" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "overview" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "formation-sequential" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "phalanx-parallel" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "campaign-graphdag" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "chain-of-command-hierarchical" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "pattern-selection-guide" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "common-pitfalls" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "performance-considerations" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "introduction" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "motivation" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "quick-start" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "syntax-reference" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "error-handling-strategies" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "visualization" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "best-practices" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "troubleshooting" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "performance-considerations" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "examples" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "quick-start" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "installation" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "environment-setup" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "getting-started" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "paladin-onboarding" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "paladin-setup-check" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "paladin-features" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "commands-reference" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "paladin-agent" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "paladin-battalion" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "paladin-muster" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "paladin-council" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "paladin-maneuver" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "paladin-arsenal" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "configuration-files" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "examples" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "troubleshooting" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "overview" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "provider-comparison" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "configuration-guide" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "use-case-recommendations" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "migration-guide" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "performance-characteristics" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "prerequisites" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "local-development" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "docker-compose" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "kubernetes" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "cloud-deployments" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "production-best-practices" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "monitoring" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "backup-and-recovery" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "migration-scenarios" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "inmemory-to-qdrant-migration" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "qdrant-version-upgrades" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "changing-vector-dimensions" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "zero-downtime-migration" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "rollback-procedures" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "data-validation" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "troubleshooting" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "executive-summary" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "architecture-overview" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "design-principles" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "system-architecture" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "core-components" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "data-flow" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "implementation-guidelines" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "security-considerations" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "deployment-architecture" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "future-considerations" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "5-use-cases-initial" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "overview" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "configuration-file-structure" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "garrison-configuration-memory" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "arsenal-configuration-tools" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "scheduler-configuration" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "complete-configuration-examples" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "environment-variables" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "troubleshooting" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "overview" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "quick-start" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "command-syntax" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "agent-roles" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "discussion-modes" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "output-options" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "best-practices" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "examples" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "troubleshooting" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "overview" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "quick-start" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "command-syntax" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "generation-workflow" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "configuration-options" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "output-formats" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "best-practices" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "examples" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "troubleshooting" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "cli-snapshot-testing" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "code-of-conduct" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "getting-started" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "development-workflow" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "architecture-guidelines" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "testing-requirements" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "documentation-standards" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "pull-request-process" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "community" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "overview" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "prerequisites" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "quick-start" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "docker-images" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "configuration" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "environment-variables" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "volumes-and-persistence" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "networking" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "multi-container-setup" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "multi-architecture-support" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "image-versioning" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "health-checks" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "resource-limits" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "production-deployment" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "troubleshooting" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "overview" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "prerequisites" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "quick-start" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "architecture" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "kubernetes-manifests" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "configmaps-and-secrets" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "helm-chart" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "resource-management" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "high-availability" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "horizontal-scaling" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "storage" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "networking" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "monitoring" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "security" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "troubleshooting" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "pre-deployment-checklist" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "security" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "performance" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "reliability" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "monitoring" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "disaster-recovery" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "cost-optimization" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "maintenance" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "overview" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "github-actions-workflows" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "ci-pipeline" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "docker-build-pipeline" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "release-pipeline" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "integration-testing" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "security-scanning" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "deployment-automation" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "best-practices" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "quick-start" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "paladinbuilder-api" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "execution-model" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "paladinresult-fields" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "stopreason-variants" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "autonomous-features" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "memory--garrison" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "tools--arsenal" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "output-formatting--herald" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "configuration-reference" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "error-handling" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "best-practices" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "overview" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "quick-start" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "the-eight-patterns" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "formation--sequential" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "phalanx--concurrent" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "campaign--graphdag" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "chain-of-command--hierarchical" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "conclave--mixture-of-experts" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "council--collaborative-discussion" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "grove--semantic-routing" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "maneuver--flow-dsl" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "commander--strategy-router" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "error-handling" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "performance-notes" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "best-practices" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "workflow-patterns-overview" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "formation--sequential" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "phalanx--parallel" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "campaign--graph--dag" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "chain-of-command--hierarchical" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "commander--dynamic-strategy-routing" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "job-scheduling" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "event-and-trigger-system" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "configuration-reference" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "see-also" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "configuration-reference" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation::filesystem] Not checking that the "paladin-content" section exists in "/workspace/.claude/worktrees/agent-a13b41d049eb8d646/docs/src/api-reference/crate-map.md" because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "content-ingestion-sources" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "aggregation-and-the-processing-pipeline" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "content--agent-bridge" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "content-delivery" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "capabilities-and-limitations" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "see-also" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "capabilities-and-limitations" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation::filesystem] Not checking that the "use-case-recipes" section exists in "/workspace/.claude/worktrees/agent-a13b41d049eb8d646/docs/src/user-guides/agent-orchestrator-bridge.md" because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation::filesystem] Not checking that the "paladin-content" section exists in "/workspace/.claude/worktrees/agent-a13b41d049eb8d646/docs/src/api-reference/crate-map.md" because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "agents-triggering-orchestration" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "orchestration-invoking-agents" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "configuring-the-bridge" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "use-case-recipes" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "see-also" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation::filesystem] Not checking that the "configuration-reference" section exists in "/workspace/.claude/worktrees/agent-a13b41d049eb8d646/docs/src/user-guides/orchestration.md" because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation::filesystem] Not checking that the "job-scheduling" section exists in "/workspace/.claude/worktrees/agent-a13b41d049eb8d646/docs/src/user-guides/orchestration.md" because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation::filesystem] Not checking that the "event-and-trigger-system" section exists in "/workspace/.claude/worktrees/agent-a13b41d049eb8d646/docs/src/user-guides/orchestration.md" because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation::filesystem] Not checking that the "paladin-ports" section exists in "/workspace/.claude/worktrees/agent-a13b41d049eb8d646/docs/src/api-reference/crate-map.md" because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "concepts" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "quick-start--stdio-server" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "streamable-http-server-configuration" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "configyml-reference" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "arsenalport-trait" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "arsenalregistry-trait" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "attaching-arsenal-to-a-paladin" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "custom-armaments-direct-rust-tools" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "handoff-tool" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "error-handling" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "best-practices" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "concepts" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "quick-start" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "garrison-adapters" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "garrisonport-trait" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "garrisonconfig" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "conversation-roles" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "attaching-to-a-paladin" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "long-term-memory-with-embeddings" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "configyml-reference" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "error-handling" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "best-practices" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "sanctum-vs-garrison" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "quick-start" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "sanctum-adapters" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "sanctumport-trait" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "sanctumentry-and-memory-types" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "searching-with-sanctumquery" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "rag--retrieval-augmented-generation" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "attaching-to-a-paladin" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "docker-setup-qdrant" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "configyml-reference" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "error-handling" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "best-practices" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "overview" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "available-heralds" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "herald-trait" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "attaching-to-a-paladin" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "attaching-to-a-battalion-service" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "custom-herald-implementation" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "streaming-output" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "error-handling" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "overview" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "quick-start" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "flow-dsl-syntax" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "execution-patterns" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "configuration" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "cli-commands" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "visualization" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "error-handling" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "performance" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "best-practices" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "api-reference" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "troubleshooting" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "basic-configuration" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "system-prompt-best-practices" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "model-selection" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "temperature-and-sampling" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "stop-words-and-termination" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "timeout-and-retry-settings" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "advanced-configuration" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "overview" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "garrison-architecture" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "in-memory-garrison" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "persistent-garrison" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "memory-windowing" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "semantic-search" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "memory-types" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "best-practices" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "advanced-patterns" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "troubleshooting" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "overview" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "arsenal-architecture" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "mcp-protocol" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "stdio-tool-servers" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "streamable-http-tool-servers" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "custom-tool-development" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "tool-result-handling" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "best-practices" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "troubleshooting" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "overview" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "herald-architecture" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "built-in-formatters" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "custom-formatters" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "streaming-output" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "multi-format-output" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "post-processing" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "best-practices" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "advanced-patterns" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "troubleshooting" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation::filesystem] Not checking that the "overview" section exists in "/workspace/.claude/worktrees/agent-a13b41d049eb8d646/docs/src/user-guides/tool-integration.md" because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "code-of-conduct" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "getting-started" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "git-hooks-pre-commit" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "development-workflow" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "testing-guidelines" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "code-quality-standards" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "documentation" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "releasing" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "adding-a-new-dependency" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "api-change-process" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "pull-request-process" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "community" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "rust-coding-conventions" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "testing-guidelines" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "quick-reference-test-commands" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "testing-philosophy" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "test-organization" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "unit-testing" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "integration-testing" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "functional-testing" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "test-coverage" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "mocking-and-fixtures" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "ci-integration" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "testing-best-practices" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "next-steps" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "test-coverage" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "overview" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "port-architecture" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "llm-adapter-development" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "garrison-adapter-development" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "arsenal-adapter-development" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "citadel-adapter-development" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "testing-adapters" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "publishing-adapters" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "prerequisites" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "implementation-steps" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "adapter-template" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "testing-requirements" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "documentation-requirements" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "submission-guidelines" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "overview" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "configuration" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "log-levels" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "structured-logging" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "log-aggregation" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "log-analysis" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "best-practices" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "overview" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "metrics-collection" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "prometheus-setup" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "grafana-dashboards" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "alerting" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "key-metrics" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "distributed-tracing" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "health-checks" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "performance-baselines" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "benchmarking" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "llm-optimization" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "memory-optimization" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "concurrency-tuning" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "database-optimization" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "network-optimization" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "resource-allocation" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "diagnostic-tools" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "common-issues" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "performance-issues" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "configuration-issues" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "deployment-issues" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "integration-issues" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "getting-help" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "overview" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "available-feature-flags" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "default-configuration" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "usage-examples" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "build-comparison" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "feature-dependencies" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "best-practices" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "migrating-to-v050-from-v04x" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "migrating-to-v04x-from-v03x" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "migrating-to-v020-from-v01x" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "migrating-to-v010-feature-flag-reorganization" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "migration-scenarios" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "testing-your-migration" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "introduction" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "api-stability-guarantee" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "versioning-policy" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "stability-tiers" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "per-crate-api-surface-and-stability" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "stable-public-api-catalog" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "port-traits-output-ports" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "port-traits-input-ports" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "domain-entities" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "builder-types" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "configuration-types" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "error-types" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "base-types" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "internal-implementation-details-not-stable" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "api-change-process" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "migration-guide-for-breaking-changes" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "tracking-api-changes" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "frequently-asked-questions" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "questions-and-support" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "versioning-policy" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z WARN  linkcheck::validation] Not checking "api-change-process" in the current file because fragment resolution isn't implemented
[2026-08-24T12:25:45Z INFO  mdbook_linkcheck] No broken links found
```

Same result as Run 2 — `Found 895 links (0 incomplete links)` / `No broken links found` — the
`cicd.md` currency edit changed prose and code-fence content only, not the link graph, so the
link count is unchanged from Run 2 (both are post-mermaid-install, pre-`cicd.md`-edit vs.
post-`cicd.md`-edit; the edit added no new links and removed none).

## Run 4 — 16-05's closing `mdbook build docs/`, end-of-phase state, PASSED (exit 0)

**Date:** 2026-08-24T14:23:54Z – 2026-08-24T14:23:57Z
**Working directory:** `/workspace/.claude/worktrees/agent-a65150f4cec31a493` (repo root)
**Command:** `mdbook build docs/`

This is the DOCS-01 closing run, captured after the last content edit of the phase (16-05 Task 2,
commit `059523f0`, which cleared the last stale `v0.4.3` tag and settled the final two of the
fourteen verdict rows). Milestone 11's task 1.2 asks for the linkcheck report to be reviewed, not
just cited pass/fail — Run 2/Run 3 (16-01) captured the opening state; this run captures the end
state and the Review section below compares the two.

```
2026-08-24 14:23:54 [INFO] (mdbook::book): Book building has started
Warning: The mdbook-mermaid preprocessor was built against version 0.4.36 of mdbook, but we're being called from version 0.4.40
2026-08-24 14:23:54 [INFO] (mdbook::book): Running the html backend
2026-08-24 14:23:56 [INFO] (mdbook::book): Running the linkcheck backend
2026-08-24 14:23:56 [INFO] (mdbook::renderer): Invoking the "linkcheck" renderer
[2026-08-24T14:23:57Z INFO  mdbook_linkcheck] Started the link checker
[2026-08-24T14:23:57Z INFO  mdbook_linkcheck] Scanning book for links
[2026-08-24T14:23:57Z INFO  mdbook_linkcheck] Found 905 links (0 incomplete links)
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "overview" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "configuration" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "log-levels" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "structured-logging" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "log-aggregation" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "log-analysis" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "best-practices" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "overview" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "metrics-collection" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "prometheus-setup" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "grafana-dashboards" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "alerting" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "key-metrics" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "distributed-tracing" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "health-checks" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "performance-baselines" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "benchmarking" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "llm-optimization" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "memory-optimization" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "concurrency-tuning" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "database-optimization" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "network-optimization" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "resource-allocation" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "diagnostic-tools" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "common-issues" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "performance-issues" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "configuration-issues" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "deployment-issues" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "integration-issues" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "getting-help" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "overview" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "available-feature-flags" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "default-configuration" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "usage-examples" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "build-comparison" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "feature-dependencies" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "best-practices" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "migrating-to-v050-from-v04x" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "migrating-to-v04x-from-v03x" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "migrating-to-v020-from-v01x" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "migrating-to-v010-feature-flag-reorganization" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "migration-scenarios" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "testing-your-migration" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "introduction" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "api-stability-guarantee" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "versioning-policy" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "stability-tiers" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "per-crate-api-surface-and-stability" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "stable-public-api-catalog" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "port-traits-output-ports" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "port-traits-input-ports" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "domain-entities" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "builder-types" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "configuration-types" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "error-types" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "base-types" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "internal-implementation-details-not-stable" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "api-change-process" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "migration-guide-for-breaking-changes" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "tracking-api-changes" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "frequently-asked-questions" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "questions-and-support" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "versioning-policy" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "api-change-process" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "overview" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "prerequisites" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "quick-start" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "docker-images" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "configuration" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "environment-variables" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "volumes-and-persistence" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "networking" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "multi-container-setup" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "multi-architecture-support" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "image-versioning" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "health-checks" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "resource-limits" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "production-deployment" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "troubleshooting" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "environment-variables" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "multi-container-setup" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "health-check-failing" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "overview" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "prerequisites" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "quick-start" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "architecture" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "kubernetes-manifests" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "configmaps-and-secrets" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "helm-chart" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "resource-management" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "high-availability" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "horizontal-scaling" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "storage" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "networking" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "monitoring" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "security" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "troubleshooting" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "pre-deployment-checklist" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "security" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "performance" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "reliability" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "monitoring" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "disaster-recovery" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "cost-optimization" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "maintenance" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation::filesystem] Not checking that the "production-deployment" section exists in "/workspace/.claude/worktrees/agent-a65150f4cec31a493/docs/src/deployment/docker.md" because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "overview" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "github-actions-workflows" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "ci-pipeline" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "docker-build-pipeline" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "release-pipeline" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "integration-testing" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "security-scanning" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "deployment-automation" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "best-practices" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "quick-start" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "paladinbuilder-api" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "execution-model" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "paladinresult-fields" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "stopreason-variants" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "autonomous-features" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "memory--garrison" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "tools--arsenal" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "output-formatting--herald" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "configuration-reference" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "error-handling" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "best-practices" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "overview" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "quick-start" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "the-eight-patterns" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "formation--sequential" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "phalanx--concurrent" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "campaign--graphdag" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "chain-of-command--hierarchical" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "conclave--mixture-of-experts" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "council--collaborative-discussion" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "grove--semantic-routing" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "maneuver--flow-dsl" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "commander--strategy-router" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "error-handling" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "performance-notes" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "best-practices" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "workflow-patterns-overview" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "formation--sequential" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "phalanx--parallel" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "campaign--graph--dag" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "chain-of-command--hierarchical" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "commander--dynamic-strategy-routing" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "job-scheduling" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "event-and-trigger-system" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "configuration-reference" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "see-also" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "configuration-reference" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "phalanx--parallel" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation::filesystem] Not checking that the "paladin-content" section exists in "/workspace/.claude/worktrees/agent-a65150f4cec31a493/docs/src/api-reference/crate-map.md" because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "content-ingestion-sources" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "aggregation-and-the-processing-pipeline" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "content--agent-bridge" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "content-delivery" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "capabilities-and-limitations" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "see-also" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "capabilities-and-limitations" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation::filesystem] Not checking that the "use-case-recipes" section exists in "/workspace/.claude/worktrees/agent-a65150f4cec31a493/docs/src/user-guides/agent-orchestrator-bridge.md" because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation::filesystem] Not checking that the "paladin-content" section exists in "/workspace/.claude/worktrees/agent-a65150f4cec31a493/docs/src/api-reference/crate-map.md" because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "agents-triggering-orchestration" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "orchestration-invoking-agents" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "configuring-the-bridge" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "use-case-recipes" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "see-also" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation::filesystem] Not checking that the "configuration-reference" section exists in "/workspace/.claude/worktrees/agent-a65150f4cec31a493/docs/src/user-guides/orchestration.md" because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation::filesystem] Not checking that the "job-scheduling" section exists in "/workspace/.claude/worktrees/agent-a65150f4cec31a493/docs/src/user-guides/orchestration.md" because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation::filesystem] Not checking that the "event-and-trigger-system" section exists in "/workspace/.claude/worktrees/agent-a65150f4cec31a493/docs/src/user-guides/orchestration.md" because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation::filesystem] Not checking that the "paladin-ports" section exists in "/workspace/.claude/worktrees/agent-a65150f4cec31a493/docs/src/api-reference/crate-map.md" because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "concepts" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "quick-start--stdio-server" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "streamable-http-server-configuration" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "configyml-reference" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "arsenalport-trait" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "arsenalregistry-trait" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "attaching-arsenal-to-a-paladin" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "custom-armaments-direct-rust-tools" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "handoff-tool" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "error-handling" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "best-practices" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "concepts" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "quick-start" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "garrison-adapters" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "garrisonport-trait" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "garrisonconfig" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "conversation-roles" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "attaching-to-a-paladin" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "long-term-memory-with-embeddings" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "configyml-reference" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "error-handling" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "best-practices" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "sanctum-vs-garrison" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "quick-start" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "sanctum-adapters" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "sanctumport-trait" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "sanctumentry-and-memory-types" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "searching-with-sanctumquery" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "rag--retrieval-augmented-generation" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "attaching-to-a-paladin" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "docker-setup-qdrant" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "configyml-reference" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "error-handling" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "best-practices" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "overview" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "available-heralds" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "herald-trait" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "attaching-to-a-paladin" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "attaching-to-a-battalion-service" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "custom-herald-implementation" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "streaming-output" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "error-handling" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "overview" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "quick-start" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "flow-dsl-syntax" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "execution-patterns" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "configuration" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "cli-commands" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "visualization" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "error-handling" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "performance" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "best-practices" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "api-reference" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "troubleshooting" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "basic-configuration" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "system-prompt-best-practices" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "model-selection" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "temperature-and-sampling" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "stop-words-and-termination" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "timeout-and-retry-settings" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "advanced-configuration" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation::filesystem] Not checking that the "custom-tool-development" section exists in "/workspace/.claude/worktrees/agent-a65150f4cec31a493/docs/src/user-guides/tool-integration.md" because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "overview" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "garrison-architecture" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "in-memory-garrison" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "persistent-garrison" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "memory-windowing" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "semantic-search" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "memory-types" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "best-practices" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "advanced-patterns" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "troubleshooting" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "scoping-by-paladin" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "semantic-search" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "overview" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "arsenal-architecture" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "mcp-protocol" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "stdio-tool-servers" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "streamable-http-tool-servers" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "custom-tool-development" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "tool-result-handling" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "best-practices" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "troubleshooting" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "overview" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "herald-architecture" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "built-in-formatters" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "custom-formatters" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "streaming-output" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "multi-format-output" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "post-processing" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "best-practices" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "advanced-patterns" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "troubleshooting" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "embedding-in-your-own-app" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "embedding-in-your-own-app" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "code-of-conduct" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "getting-started" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "git-hooks-pre-commit" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "development-workflow" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "testing-guidelines" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "code-quality-standards" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "documentation" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "releasing" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "adding-a-new-dependency" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "api-change-process" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "pull-request-process" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "community" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "rust-coding-conventions" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "testing-guidelines" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "quick-reference-test-commands" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "testing-philosophy" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "test-organization" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "unit-testing" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "integration-testing" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "functional-testing" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "test-coverage" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "mocking-and-fixtures" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "ci-integration" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "testing-best-practices" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "next-steps" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "test-coverage" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "overview" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "port-architecture" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "llm-adapter-development" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "garrison-adapter-development" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "arsenal-adapter-development" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "citadel-adapter-development" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "testing-adapters" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "publishing-adapters" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "prerequisites" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "implementation-steps" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "adapter-template" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "testing-requirements" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "documentation-requirements" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "submission-guidelines" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "overview" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "quick-start" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "routing-strategies" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "expertise-definition" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "fallback-behavior" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "configuration" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "examples" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "best-practices" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "api-reference" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "quick-start" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "overview" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "quick-start" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "turn-taking-strategies" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "termination-conditions" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "garrison-integration" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "configuration" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "examples" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "best-practices" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "api-reference" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "quick-start" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "introduction" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "getting-started" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "vision-content-types" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "supported-providers" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "paladin-vision-api" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "document-processing" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "cli-usage" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "yaml-configuration" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "security" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "battalion-integration" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "error-handling" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "performance-considerations" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "troubleshooting" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "overview" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "quick-start" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "configuration" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "programmatic-api" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "yaml-configuration" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "cli-usage" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "use-cases" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "error-handling" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "observability" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "best-practices" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "troubleshooting" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "overview" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "formation-sequential" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "phalanx-parallel" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "campaign-graphdag" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "chain-of-command-hierarchical" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "pattern-selection-guide" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "common-pitfalls" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "performance-considerations" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "introduction" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "motivation" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "quick-start" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "syntax-reference" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "error-handling-strategies" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "visualization" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "best-practices" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "troubleshooting" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "performance-considerations" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "examples" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "quick-start" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "installation" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "environment-setup" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "getting-started" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "paladin-onboarding" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "paladin-setup-check" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "paladin-features" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "commands-reference" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "paladin-agent" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "paladin-battalion" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "paladin-muster" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "paladin-council" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "paladin-maneuver" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "paladin-arsenal" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "configuration-files" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "examples" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "troubleshooting" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "overview" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "provider-comparison" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "configuration-guide" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "use-case-recommendations" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "migration-guide" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "performance-characteristics" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "prerequisites" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "local-development" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "docker-compose" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "kubernetes" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "cloud-deployments" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "production-best-practices" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "monitoring" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "backup-and-recovery" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "migration-scenarios" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "inmemory-to-qdrant-migration" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "qdrant-version-upgrades" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "changing-vector-dimensions" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "zero-downtime-migration" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "rollback-procedures" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "data-validation" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "troubleshooting" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "executive-summary" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "architecture-overview" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "design-principles" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "system-architecture" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "core-components" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "data-flow" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "implementation-guidelines" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "security-considerations" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "deployment-architecture" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "future-considerations" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "5-use-cases-initial" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "overview" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "configuration-file-structure" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "garrison-configuration-memory" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "arsenal-configuration-tools" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "scheduler-configuration" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "complete-configuration-examples" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "environment-variables" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "troubleshooting" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "overview" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "quick-start" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "command-syntax" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "agent-roles" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "discussion-modes" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "output-options" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "best-practices" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "examples" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "troubleshooting" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "overview" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "quick-start" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "command-syntax" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "generation-workflow" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "configuration-options" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "output-formats" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "best-practices" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "examples" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "troubleshooting" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "cli-snapshot-testing" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "code-of-conduct" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "getting-started" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "development-workflow" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "architecture-guidelines" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "testing-requirements" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "documentation-standards" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "pull-request-process" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation] Not checking "community" in the current file because fragment resolution isn't implemented
[2026-08-24T14:23:57Z WARN  linkcheck::validation::filesystem] Not checking that the "overview" section exists in "/workspace/.claude/worktrees/agent-a65150f4cec31a493/docs/src/user-guides/tool-integration.md" because fragment resolution isn't implemented
[2026-08-24T14:23:57Z INFO  mdbook_linkcheck] No broken links found
```

## Review — closing run vs. opening run (D-10, Milestone 11 task 1.2)

Run 4 reports, verbatim: `Found 905 links (0 incomplete links)` followed immediately by
`No broken links found`. Zero broken links, zero incomplete links, exit 0 — the same clean
result as Run 2/Run 3. `warning-policy = "error"` (`docs/book.toml:26`) means any warning
promoted to that policy would have failed the build exactly like Run 1 did at the start of the
phase; Run 4 has zero such escalations. As in every prior run, all links are local intra-book
references — `follow-web-links = false` (`docs/book.toml:25`) means no outbound HTTP request was
ever made; the check is entirely filesystem/anchor based. The "Not checking ... because fragment
resolution isn't implemented" lines (456 of them in Run 4) are `mdbook-linkcheck` 0.7.7's own
documented limitation (verifies the target *file* of an in-book anchor link, not the anchor
itself) — informational `WARN`-level lines, not build-failing warnings, unchanged in kind from
Run 1 through Run 4.

**Link count, compared honestly rather than merely cited:** Run 3 (16-01, captured immediately
after that plan's own `cicd.md` fix) found 895 links. Run 4 finds 905 — ten more. This delta is
**not** attributable to 16-05's own edits: a `git diff` of every 16-05 commit
(`103249e3`, `059523f0`) against the four operations pages contains zero added or removed
markdown link syntax (`[text](url)`) — every citation this plan added is a backtick code span
(e.g. `` `crates/paladin-core/src/platform/container/log.rs:76-89` ``), which mdbook does not
parse as a link. The ten-link delta is fully explained by the six other plans that landed in this
worktree's git history between Run 3 (captured mid-16-01) and this run: 16-02, 16-03, 16-04,
16-09, and 16-10 each authored or corrected substantial new prose across `docs/src/user-guides/`,
`docs/src/deployment/`, and elsewhere, including new cross-reference and "See also" links (`git
log --oneline` shows `f3d3ef33` 16-03, `4274713b` 16-04, `44fd29ff`/`865df54f` 16-09/16-10, all
between Run 3 and Run 4). 16-05's own contribution to the link graph is a net zero — expected,
since the plan's corrections replaced fabricated content in place rather than adding new
cross-references.

**Conclusion:** the book is green at the phase's end state exactly as it was at the phase's
start (Run 1's single pre-existing broken link, fixed by 16-01 before any of these runs), and it
stayed green through every one of the sixteen-plus edits that landed across 16-01 through 16-05.
`git diff --exit-code docs/book.toml` and `git diff --exit-code .github/workflows/` both exit 0
after Run 4 — no plan in this phase touched the book config or a CI workflow file, and D-08
holds through the phase's close.
