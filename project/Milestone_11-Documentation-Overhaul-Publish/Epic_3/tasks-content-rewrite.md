## Relevant Files

- `docs/src/introduction.md` — Fix 14 broken links to old flat paths (FR-33)
- `docs/book.toml` — Re-enable `[output.linkcheck]` after all links are fixed (FR-1)
- `scripts/check-doc-examples.sh` — New script: extracts fenced Rust code blocks and runs `cargo check` via temp crate (FR-2, FR-3)
- `Makefile` — Add `check-doc-examples` convenience target (FR-2)
- `.pre-commit-config.yaml` — Add `check-doc-examples` as a `pre-push` stage hook (FR-2)
- `.github/workflows/docs.yml` — Add code-block check step to `build` job (FR-3)
- `docs/src/getting-started/installation.md` — Full rewrite (FR-4)
- `docs/src/getting-started/quickstart.md` — Full rewrite (FR-5)
- `docs/src/getting-started/configuration.md` — Full rewrite (FR-6)
- `docs/src/architecture/overview.md` — In-place update (FR-7)
- `docs/src/architecture/hexagonal-design.md` — In-place update (FR-8)
- `docs/src/architecture/domain-model.md` — In-place update (FR-9)
- `docs/src/architecture/crate-map.md` — In-place update (FR-10)
- `docs/src/architecture/design-patterns.md` — In-place update (FR-11)
- `docs/src/user-guides/paladin-agents.md` — Full rewrite (FR-12)
- `docs/src/user-guides/battalion-patterns.md` — Full rewrite (FR-13)
- `docs/src/user-guides/arsenal-tools.md` — Full rewrite (FR-14)
- `docs/src/user-guides/garrison-memory.md` — Full rewrite (FR-15)
- `docs/src/user-guides/sanctum-vector-memory.md` — Full rewrite, requires Qdrant from dev container (FR-16)
- `docs/src/user-guides/herald-output.md` — Full rewrite (FR-17)
- `docs/src/user-guides/maneuver-flow-dsl.md` — In-place update
- `docs/src/user-guides/memory-management.md` — In-place update
- `docs/src/user-guides/orchestration.md` — In-place update
- `docs/src/user-guides/output-formatting.md` — In-place update
- `docs/src/user-guides/paladin-configuration.md` — In-place update
- `docs/src/user-guides/tool-integration.md` — In-place update
- `docs/src/deployment/docker.md` — In-place update (FR-18)
- `docs/src/deployment/kubernetes.md` — In-place update (FR-19)
- `docs/src/deployment/production.md` — In-place update (FR-20)
- `docs/src/deployment/cicd.md` — In-place update (FR-21)
- `docs/src/operations/logging.md` — In-place update (FR-22)
- `docs/src/operations/monitoring.md` — In-place update (FR-23)
- `docs/src/operations/performance-tuning.md` — In-place update (FR-24)
- `docs/src/operations/troubleshooting.md` — In-place update (FR-25)
- `docs/src/api-reference/stable-api.md` — Full rewrite (FR-26)
- `docs/src/api-reference/feature-flags.md` — In-place update (FR-27)
- `docs/src/api-reference/migration-guide.md` — In-place update (FR-28)
- `docs/src/contributing/development-setup.md` — Full rewrite (FR-29)
- `docs/src/contributing/testing-guide.md` — In-place update (FR-30)
- `docs/src/contributing/architecture-decisions.md` — In-place update (FR-31)
- `docs/src/contributing/contributing-providers.md` — In-place update (FR-32)

### Notes

- This Epic modifies **no Rust source code**. Do not edit `*.rs` files or `Cargo.toml`.
- All file edits are in `docs/src/` except for infrastructure changes (`scripts/`, `Makefile`, `.pre-commit-config.yaml`, `.github/workflows/docs.yml`, `docs/book.toml`).
- Tasks must be executed in order — links must be fixed before content is rewritten, and the code-check infrastructure must exist before any code examples are added.
- After completing all tasks, `mdbook build docs/` (run from inside `docs/`) must exit 0 with zero warnings and zero linkcheck errors.
- Use `make dev` to start all services (Redis, MinIO, MySQL, Qdrant) before testing any example that requires a running service.
- All fenced Rust code blocks must use mdBook hiding-line syntax (`# use ...;`) for imports and `# #[allow(unused)]` where needed to suppress dead-code warnings.

## Instructions for Completing Tasks

**IMPORTANT:** As you complete each task, you must check it off in this markdown file by changing `- [ ]` to `- [x]`. This helps track progress and ensures you don't skip any steps.

Example:

- `- [ ] 1.1 Read file` → `- [x] 1.1 Read file` (after completing)

Update the file after completing each sub-task, not just after completing an entire parent task.

## Tasks

- [x] 0.0 Create feature branch
  - [x] 0.1 Create and checkout a new branch: `git checkout -b feature/milestone-11-epic-3-content-rewrite`

- [x] 1.0 Fix all 227 broken internal cross-reference links and re-enable linkcheck (FR-1, FR-33)
  - [x] 1.1 Temporarily enable linkcheck in `docs/book.toml` with `warning-policy = "warn"` (not "error") to capture the full broken-link list without failing the build
  - [ ] 1.2 Run `cd /workspace/docs && mdbook build 2>&1 | grep -i "warning\|error" > /tmp/linkcheck-report.txt` and review the full list
  - [x] 1.3 Fix the 14 broken links in `docs/src/introduction.md` (e.g., `QUICKSTART.md` → `getting-started/quickstart.md`, `INSTALLATION.md` → `getting-started/installation.md`, etc.) — FR-33
  - [x] 1.4 Fix broken links in all `docs/src/getting-started/` files
  - [x] 1.5 Fix broken links in all `docs/src/user-guides/` files
  - [x] 1.6 Fix broken links in all `docs/src/architecture/` files
  - [x] 1.7 Fix broken links in all `docs/src/deployment/` and `docs/src/operations/` files
  - [x] 1.8 Fix broken links in all `docs/src/api-reference/` and `docs/src/contributing/` files
  - [x] 1.9 Fix any broken links in `docs/src/appendix/` files identified by the linkcheck report
  - [x] 1.10 Update `docs/book.toml`: replace the commented-out `[output.linkcheck]` block with the active configuration (`follow-web-links = false`, `warning-policy = "error"`)
  - [x] 1.11 Run `cd /workspace/docs && mdbook build` and confirm it exits 0 with zero linkcheck errors or warnings
  - [x] 1.12 Commit: `git commit -m "fix(docs): fix all 227 broken internal cross-reference links; re-enable linkcheck"`

- [x] 2.0 Add code-block check infrastructure (FR-2, FR-3)
  - [x] 2. Create `scripts/check-doc-examples.sh`: script that (a) extracts each fenced ` ```rust ` block from `docs/src/**/*.md` using `awk`, (b) creates a temp crate at `/tmp/paladin-doc-check-$$` with a `Cargo.toml` pointing at the workspace root, (c) writes each block as `src/main.rs` and runs `cargo check`, (d) reports the source file name and block number on failure, (e) cleans up via `trap` on exit
  - [x] 2. Make the script executable: `chmod +x scripts/check-doc-examples.sh`
  - [x] 2. Add a `check-doc-examples` target to the `Makefile` that runs `./scripts/check-doc-examples.sh`
  - [x] 2. Add the following hook to the `local` repo section of `.pre-commit-config.yaml` as a `pre-push` stage hook:
        ```yaml
        - id: check-doc-examples
          name: cargo check (doc code examples)
          entry: ./scripts/check-doc-examples.sh
          language: system
          stages: [pre-push]
          pass_filenames: false
          always_run: true
        ```
  - [x] 2. Add the following step to the `build` job in `.github/workflows/docs.yml` (after the `Build docs` step):
        ```yaml
        - name: Check doc code examples
          run: ./scripts/check-doc-examples.sh
        ```
  - [x] 2. Run `make check-doc-examples` locally to confirm the script runs and passes against current doc files (no examples yet — should exit 0)
  - [x] 2. Validate `.pre-commit-config.yaml` and `.github/workflows/docs.yml` YAML syntax
  - [x] 2. Commit: `git commit -m "feat(docs): add doc code-example check script, pre-push hook, and CI step"`

- [x] 3.0 Rewrite Getting Started guides (FR-4, FR-5, FR-6)
  - [x] 3.1 Read `Cargo.toml` (root) for workspace crate names, versions, and feature flags; read `config.yml` for full configuration schema
  - [x] 3.2 Rewrite `docs/src/getting-started/installation.md`: Rust ≥ 1.85 requirement, system prerequisites (Docker), all v0.4.3 crate names/versions, feature flag profiles, verification snippet
  - [x] 3.3 Run `make check-doc-examples` to confirm `installation.md` code blocks pass `cargo check`
  - [x] 3.4 Read `examples/basic_paladin.rs` (and related examples) for current `PaladinBuilder` usage patterns
  - [x] 3.5 Rewrite `docs/src/getting-started/quickstart.md`: end-to-end "hello world" `PaladinBuilder` example, `make dev` service startup, expected output, pointer to `configuration.md`
  - [x] 3.6 Run `make check-doc-examples` to confirm `quickstart.md` code blocks pass `cargo check`
  - [x] 3.7 Read `config.yml` and `config.test.yml` to enumerate every top-level section and key
  - [x] 3.8 Rewrite `docs/src/getting-started/configuration.md`: complete schema for `paladin`, `garrison`, `arsenal`, `llm` sections; every key with type/default/description; env var overrides; multi-environment patterns
  - [x] 3.9 Run `make check-doc-examples` to confirm `configuration.md` code blocks pass `cargo check`
  - [x] 3.10 Run `cd /workspace/docs && mdbook build` — confirm zero errors and zero linkcheck warnings
  - [x] 3.11 Commit: `git commit -m "docs(getting-started): full rewrite of installation, quickstart, and configuration guides"`

- [ ] 4.0 Update Architecture documentation in-place (FR-7, FR-8, FR-9, FR-10, FR-11)
  - [ ] 4.1 Read each crate's `Cargo.toml` and `src/lib.rs` to capture module structure, feature flags, and re-exports; read `.github/copilot-instructions.md` for the authoritative naming convention table
  - [ ] 4.2 Update `docs/src/architecture/overview.md`: three-layer hexagonal diagram (Mermaid), correct crate-to-layer mapping for all 9 workspace crates, inward-only dependency flow rule
  - [ ] 4.3 Update `docs/src/architecture/hexagonal-design.md`: current port trait locations under `crates/paladin-ports/`, current adapter locations per crate, step-by-step guide for adding a new adapter
  - [ ] 4.4 Update `docs/src/architecture/domain-model.md`: all domain entities with current module paths, `Node<T>` pattern, complete Medieval Military naming convention table
  - [ ] 4.5 Update `docs/src/architecture/crate-map.md`: every workspace crate with layer/purpose/feature flags, Mermaid crate dependency graph
  - [ ] 4.6 Update `docs/src/architecture/design-patterns.md`: `PaladinBuilder` with current method signatures, `thiserror` error enum examples, port trait pattern (`async_trait`, `Send + Sync`), service composition pattern
  - [ ] 4.7 Run `make check-doc-examples` to confirm all architecture code examples pass `cargo check`
  - [ ] 4.8 Run `cd /workspace/docs && mdbook build` — confirm Mermaid diagrams render and zero linkcheck warnings
  - [ ] 4.9 Commit: `git commit -m "docs(architecture): update all architecture docs for v0.4.3 workspace structure"`

- [ ] 5.0 Rewrite User Guides — full rewrites (FR-12, FR-13, FR-14, FR-15, FR-16, FR-17)
  - [ ] 5.1 Read `crates/paladin-core/src/` and `crates/paladin-ports/src/` for `PaladinBuilder`, `PaladinExecutionService`, and `PaladinStatus` current signatures
  - [ ] 5.2 Rewrite `docs/src/user-guides/paladin-agents.md`: `PaladinBuilder` fluent API, `PaladinExecutionService`, `PaladinStatus` lifecycle, working end-to-end example
  - [ ] 5.3 Verify `paladin-agents.md` examples pass `make check-doc-examples`
  - [ ] 5.4 Read `crates/paladin-battalion/src/` for Formation, Phalanx, Campaign, ChainOfCommand, and Commander current module paths and type signatures
  - [ ] 5.5 Rewrite `docs/src/user-guides/battalion-patterns.md`: all five patterns with current module paths, when-to-use guide, working example per pattern
  - [ ] 5.6 Verify `battalion-patterns.md` examples pass `make check-doc-examples`
  - [ ] 5.7 Read `crates/paladin-ports/src/` for `ArsenalPort` trait and MCP adapter source for tool discovery lifecycle
  - [ ] 5.8 Rewrite `docs/src/user-guides/arsenal-tools.md`: MCP STDIO and SSE adapters, `ArsenalPort` trait, tool discovery lifecycle, `config.yml` `arsenal.mcp_servers` section, working example
  - [ ] 5.9 Verify `arsenal-tools.md` examples pass `make check-doc-examples`
  - [ ] 5.10 Read `crates/paladin-memory/src/` for `GarrisonPort`, in-memory and SQLite adapter implementations
  - [ ] 5.11 Rewrite `docs/src/user-guides/garrison-memory.md`: both garrison adapters, `GarrisonPort` trait methods, memory lifecycle, `config.yml` `garrison` section, working example
  - [ ] 5.12 Verify `garrison-memory.md` examples pass `make check-doc-examples`
  - [ ] 5.13 Read `docker/docker-compose.dev.yml` to confirm Qdrant service name and port; read Sanctum adapter source in `crates/paladin-memory/src/`
  - [ ] 5.14 Rewrite `docs/src/user-guides/sanctum-vector-memory.md`: Sanctum vector store overview, current adapter, configuration, semantic search usage, working example with `make dev` prerequisite callout
  - [ ] 5.15 Verify `sanctum-vector-memory.md` examples pass `make check-doc-examples`
  - [ ] 5.16 Read Herald output formatting source in `crates/paladin-core/src/` or wherever `Herald` is implemented
  - [ ] 5.17 Rewrite `docs/src/user-guides/herald-output.md`: output formatting system, available formatters, custom formatter example, working example
  - [ ] 5.18 Verify `herald-output.md` examples pass `make check-doc-examples`
  - [ ] 5.19 Run `cd /workspace/docs && mdbook build` — zero errors and zero linkcheck warnings
  - [ ] 5.20 Commit: `git commit -m "docs(user-guides): full rewrite of paladin-agents, battalion-patterns, arsenal-tools, garrison-memory, sanctum, herald"`

- [ ] 6.0 Update remaining User Guide files in-place
  - [ ] 6.1 Update `docs/src/user-guides/orchestration.md` — current Commander routing and orchestration patterns
  - [ ] 6.2 Update `docs/src/user-guides/maneuver-flow-dsl.md` — current flow DSL syntax and usage
  - [ ] 6.3 Update `docs/src/user-guides/memory-management.md` — current memory management patterns and lifecycle
  - [ ] 6.4 Update `docs/src/user-guides/tool-integration.md` — current tool integration patterns
  - [ ] 6.5 Update `docs/src/user-guides/paladin-configuration.md` — current configuration options and patterns
  - [ ] 6.6 Update `docs/src/user-guides/output-formatting.md` — current output formatting options
  - [ ] 6.7 Run `make check-doc-examples` to verify all updated user guide examples pass `cargo check`
  - [ ] 6.8 Run `cd /workspace/docs && mdbook build` — zero errors and zero linkcheck warnings
  - [ ] 6.9 Commit: `git commit -m "docs(user-guides): in-place updates for orchestration, maneuver, memory-management, tool-integration, paladin-configuration, output-formatting"`

- [ ] 7.0 Update Deployment and Operations documentation in-place (FR-18–FR-25)
  - [ ] 7.1 Read `docker/docker-compose.dev.yml`, `Dockerfile`, and `Dockerfile.chef` for current service names, ports, and build stages
  - [ ] 7.2 Update `docs/src/deployment/docker.md`: `docker/docker-compose.dev.yml` services, `Dockerfile`/`Dockerfile.chef` build stages, env var config, `make dev`/`make services-up`, health checks
  - [ ] 7.3 Read `k8s/` directory for current manifest files
  - [ ] 7.4 Update `docs/src/deployment/kubernetes.md`: current manifests, Deployment/Service/ConfigMap/Secret resources, Battalion workload scaling considerations
  - [ ] 7.5 Update `docs/src/deployment/production.md`: production configuration checklist, secret management, TLS, resource limits
  - [ ] 7.6 Update `docs/src/deployment/cicd.md`: reflect current `.github/workflows/docs.yml` and all other CI workflows present under `.github/workflows/`
  - [ ] 7.7 Update `docs/src/operations/logging.md`: `tracing`/`log` setup, log level configuration, structured log format, aggregation recommendations
  - [ ] 7.8 Update `docs/src/operations/monitoring.md`: Sentinel integration, health check endpoints, alerting recommendations
  - [ ] 7.9 Read `benches/` for current benchmark files and results
  - [ ] 7.10 Update `docs/src/operations/performance-tuning.md`: benchmark results, Tokio runtime tuning, Phalanx concurrency limits, DB/queue connection pooling
  - [ ] 7.11 Update `docs/src/operations/troubleshooting.md`: common error scenarios with current error types, recovery steps
  - [ ] 7.12 Run `make check-doc-examples` to verify all deployment/operations code examples pass `cargo check`
  - [ ] 7.13 Run `cd /workspace/docs && mdbook build` — zero errors and zero linkcheck warnings
  - [ ] 7.14 Commit: `git commit -m "docs(deployment,operations): in-place updates for docker, k8s, production, cicd, logging, monitoring, perf-tuning, troubleshooting"`

- [ ] 8.0 Update API Reference and Contributing documentation (FR-26–FR-32)
  - [ ] 8.1 Read root `Cargo.toml` and all `crates/*/Cargo.toml` for all current feature flag definitions and defaults
  - [ ] 8.2 Rewrite `docs/src/api-reference/stable-api.md`: current stability guarantees, stability tiers, versioning policy for v0.4.3
  - [ ] 8.3 Update `docs/src/api-reference/feature-flags.md`: every feature flag sourced from `Cargo.toml`, with default, what it enables, and any mutually exclusive flags
  - [ ] 8.4 Update `docs/src/api-reference/migration-guide.md`: add a migration section for every breaking change since the last stable release; consult `CHANGELOG.md`
  - [ ] 8.5 Rewrite `docs/src/contributing/development-setup.md`: dev container setup, current `make` targets (from `Makefile`), Clippy `-D warnings` requirement, pre-commit hooks (`make hooks`), workflow for first PR
  - [ ] 8.6 Update `docs/src/contributing/testing-guide.md`: unit tests (`cargo test --workspace --lib`), integration tests (`make test-all`, `make test-integration-docker`), doc tests (`cargo test --doc`)
  - [ ] 8.7 Update `docs/src/contributing/architecture-decisions.md`: current adapter locations per crate, port trait contracts in `crates/paladin-ports/`
  - [ ] 8.8 Update `docs/src/contributing/contributing-providers.md`: current LLM provider adapter structure under `crates/paladin-llm/`, steps to add a new provider
  - [ ] 8.9 Run `make check-doc-examples` to verify all API reference and contributing code examples pass `cargo check`
  - [ ] 8.10 Run `cd /workspace/docs && mdbook build` — zero errors and zero linkcheck warnings
  - [ ] 8.11 Commit: `git commit -m "docs(api-reference,contributing): rewrite stable-api; update feature-flags, migration-guide, dev-setup, testing, architecture-decisions, contributing-providers"`

- [ ] 9.0 Final verification and commit
  - [ ] 9.1 Run `cd /workspace/docs && mdbook build` — must exit 0 with **zero errors and zero warnings** (linkcheck active with `warning-policy = "error"`)
  - [ ] 9.2 Run `make check-doc-examples` — all fenced Rust code blocks across all 33 files must pass `cargo check`
  - [ ] 9.3 Run `cargo test` to confirm no Rust regressions were introduced
  - [ ] 9.4 Run `cargo fmt --check` and `cargo clippy --workspace --all-targets --all-features -- -D warnings`
  - [ ] 9.5 Confirm `docs/book.toml` has `[output.linkcheck]` active with `warning-policy = "error"` (not commented out)
  - [ ] 9.6 Confirm `.github/workflows/docs.yml` includes the `Check doc code examples` step
  - [ ] 9.7 Stage all remaining changes: `git add .`
  - [ ] 9.8 Commit: `git commit -m "feat(milestone-11/epic-3): documentation content rewrite complete — all links, code examples, and content current for v0.4.3"` with body summarising all files changed
