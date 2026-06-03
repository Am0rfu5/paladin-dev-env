# PRD: Epic 3 — Documentation Content Rewrite

**Project:** Paladin Framework
**Milestone:** 11 — Documentation Review, Reorganization, MDBook Publish
**Epic:** 3 — Content Rewrite
**Version Target:** v0.5.0
**Status:** Not Started
**Created:** 2026-06-01
**Author:** Paladin Framework Contributors

---

## 1. Introduction / Overview

The Paladin MDBook site went live in Epic 2 with all 71 documentation files migrated into the `docs/src/` chapter hierarchy. However, the content itself was not updated during migration: import paths reference pre-refactoring module locations, code examples use deprecated API signatures, configuration snippets reference keys that no longer exist, and `[output.linkcheck]` is **disabled** because 227 internal cross-reference links still point to old flat-file paths (e.g., `QUICKSTART.md` instead of `getting-started/quickstart.md`).

This Epic rewrites all stale content so that:
- Every Rust code block compiles against the current v0.4.3 workspace.
- Every configuration snippet matches the current `config.yml` schema.
- Every internal link resolves correctly within the MDBook structure.
- `[output.linkcheck]` is re-enabled with `warning-policy = "error"` as a hard CI gate.

The result is a documentation site that a developer can trust and follow without being misled by outdated examples.

---

## 2. Goals

1. **Fix all 227 broken internal cross-reference links** — replace stale flat-file paths with correct MDBook-relative paths, then re-enable `[output.linkcheck]` in `docs/book.toml`.
2. **Rewrite Getting Started guides from scratch** — provide a working end-to-end new-user experience with compilable examples.
3. **Update Architecture docs in-place** — correct module paths, crate names, and diagrams to reflect the final Milestone 8 workspace structure.
4. **Rewrite User Guides from scratch** — provide accurate API signatures, current import paths, and working examples for all major subsystems.
5. **Update Deployment and Operations docs in-place** — verify all shell commands, Docker Compose references, and configuration snippets against the current dev environment.
6. **Rewrite API Reference and Contributing docs from scratch** — bring `stable-api.md`, `feature-flags.md`, `migration-guide.md`, and `development-setup.md` in line with v0.4.3.
7. **Add a local pre-commit code-check and a CI code-check step** — extract fenced Rust code blocks from docs and run `cargo check` so regressions are caught before merge.
8. **`mdbook build` passes with zero errors and zero warnings** with `[output.linkcheck]` re-enabled.

---

## 3. User Stories

- **As a new user**, I want the Getting Started guide to give me a working "hello world" Paladin agent so I can evaluate the framework without guessing at deprecated APIs.
- **As a developer integrating a Battalion pattern**, I want the User Guide to show me current import paths and method signatures so I don't waste time chasing compiler errors from stale examples.
- **As a DevOps engineer**, I want the Deployment docs to reference the correct Docker Compose files and `make` commands so I can spin up services without consulting source code.
- **As a contributor**, I want the Contributing guide to reflect the actual current dev-container setup, test commands, and Clippy requirements so I can pass CI on my first PR.
- **As any reader following a cross-reference link**, I want every link in the docs to resolve correctly so I'm never sent to a 404.

---

## 4. Functional Requirements

### FR-1: Fix all internal cross-reference links
The 227 broken internal links (stale flat-file paths) must be resolved to correct MDBook-relative paths. After fixing, `[output.linkcheck]` must be re-enabled in `docs/book.toml` with the following configuration:
```toml
[output.linkcheck]
follow-web-links = false
warning-policy = "error"
```
`mdbook build` must pass with zero linkcheck errors.

### FR-2: Local pre-commit code-block check
A new hook must be added to `.pre-commit-config.yaml` (under the existing `local` repo section, as a `pre-push` stage hook — matching the pattern of `cargo-build-push` and `cargo-test-lib-push`) that runs `scripts/check-doc-examples.sh`. This ensures consistent enforcement for every developer without any manual opt-in. A `make check-doc-examples` Makefile target must also exist as a convenience alias for running it manually.

### FR-3: CI code-block check in `docs.yml`
The `.github/workflows/docs.yml` `build` job must include a step that extracts and `cargo check`s all fenced Rust code blocks in `docs/src/**/*.md`. This step runs on every PR touching `docs/**` and fails the build if any example does not compile.

### FR-4: Rewrite Getting Started — `installation.md`
Full rewrite. Must cover:
- Minimum supported Rust version: the current stable release (no `rust-toolchain.toml` is present; document the version in use at the time of writing and note that `edition = "2024"` requires Rust ≥ 1.85).
- System prerequisites (Docker for services, system libraries).
- Workspace crate names and versions (`paladin-ai-core`, `paladin-ports`, `paladin-battalion`, etc. at v0.4.3).
- Feature flag profiles for common use cases.
- Verification snippet that compiles.

### FR-5: Rewrite Getting Started — `quickstart.md`
Full rewrite. Must cover:
- End-to-end "hello world" `PaladinBuilder` example that compiles and runs.
- Required service startup via `make dev` or `docker compose`.
- Expected terminal output.
- Pointer to `configuration.md`.

### FR-6: Rewrite Getting Started — `configuration.md`
Full rewrite. Must cover:
- Complete `config.yml` schema (all top-level sections: `paladin`, `garrison`, `arsenal`, `llm`, and any others present).
- Every key with its type, default value, and a one-line description.
- Environment variable override syntax.
- Multi-environment configuration patterns.

### FR-7: Update Architecture — `overview.md`
In-place update. Must cover:
- Three-layer hexagonal architecture (Core, Application, Infrastructure).
- Correct workspace crate-to-layer mapping (`paladin-core`, `paladin-ports`, `paladin-battalion`, `paladin-llm`, `paladin-memory`, `paladin-storage`, `paladin-content`, `paladin-web`, `paladin-notifications`).
- Dependency flow rule (inward only).
- High-level Mermaid diagram of layer relationships.

### FR-8: Update Architecture — `hexagonal-design.md`
In-place update. Must cover:
- Current port trait locations under `crates/paladin-ports/`.
- Current adapter locations under each `crates/paladin-*/` crate.
- Step-by-step guide for adding a new adapter using current module paths.

### FR-9: Update Architecture — `domain-model.md`
In-place update. Must cover:
- All domain entities with current module paths.
- The `Node<T>` pattern for domain entities.
- Medieval Military naming convention table (matching `copilot-instructions.md`).

### FR-10: Update Architecture — `crate-map.md`
In-place update. Must cover:
- Every workspace crate, its layer, and its purpose.
- Crate dependency graph as a Mermaid diagram.
- Feature flags per crate.

### FR-11: Update Architecture — `design-patterns.md`
In-place update. Must cover:
- Builder pattern (`PaladinBuilder`) with current method signatures.
- Error handling pattern (`thiserror`-based enums) with current examples.
- Port trait pattern (`async_trait`, `Send + Sync`).
- Service composition pattern.

### FR-12: Rewrite User Guide — `paladin-agents.md`
Full rewrite. Must cover the current `PaladinBuilder` fluent API, `PaladinExecutionService`, `PaladinStatus` lifecycle, and a working end-to-end example.

### FR-13: Rewrite User Guide — `battalion-patterns.md`
Full rewrite. Must cover Formation, Phalanx, Campaign, Chain of Command, and Commander — each with current module paths and a working example.

### FR-14: Rewrite User Guide — `arsenal-tools.md`
Full rewrite. Must cover MCP STDIO and SSE adapters, `ArsenalPort` trait, tool discovery lifecycle, `config.yml` configuration, and a working example.

### FR-15: Rewrite User Guide — `garrison-memory.md`
Full rewrite. Must cover in-memory and SQLite garrison adapters, `GarrisonPort` trait methods, memory lifecycle, and a working example.

### FR-16: Rewrite User Guide — `sanctum-vector-memory.md`
Full rewrite. Must cover the Sanctum vector store (qdrant-client v1.14), current adapter, configuration, semantic search usage, and a working example. A running Qdrant instance is required for the example — use the instance provided by the dev container (confirm the service name and port from `docker/docker-compose.dev.yml`). The example must include a `> **Prerequisites:** Run \`make dev\` first (starts Qdrant alongside Redis, MinIO, MySQL).` callout.

### FR-17: Rewrite User Guide — `herald-output.md`
Full rewrite. Must cover the output formatting system, available formatters, and a working example.

### FR-18: Update Deployment — `docker.md`
In-place update. Must reference `docker/docker-compose.dev.yml`, `Dockerfile`, `Dockerfile.chef`, `make dev`, `make services-up`, and current health checks.

### FR-19: Update Deployment — `kubernetes.md`
In-place update. Must reference manifests in `k8s/` and address Battalion workload scaling.

### FR-20: Update Deployment — `production.md`
In-place update. Must include production configuration checklist, secret management, TLS, and resource tuning.

### FR-21: Update Deployment — `cicd.md`
In-place update. Must reflect the current `.github/workflows/docs.yml` and any other CI workflows present.

### FR-22: Update Operations — `logging.md`
In-place update. Must cover the `tracing` / `log` setup, log level configuration, and structured log format.

### FR-23: Update Operations — `monitoring.md`
In-place update. Must cover Sentinel integration and health check endpoints.

### FR-24: Update Operations — `performance-tuning.md`
In-place update. Must reference benchmark results from `benches/`, Tokio runtime tuning, and Phalanx concurrency limits.

### FR-25: Update Operations — `troubleshooting.md`
In-place update. Must cover common error scenarios with current error types and recovery steps.

### FR-26: Rewrite API Reference — `stable-api.md`
Full rewrite. The current file is a merge of `STABLE_API.md` and `VERSIONING_POLICY.md` from before v0.4.3. Must reflect current stability guarantees and versioning policy.

### FR-27: Update API Reference — `feature-flags.md`
In-place update. Must list all current Cargo feature flags with their defaults and what they enable, sourced directly from `Cargo.toml`.

### FR-28: Update API Reference — `migration-guide.md`
In-place update. Must include a migration section for every breaking change since the last stable release.

### FR-29: Rewrite Contributing — `development-setup.md`
Full rewrite (was root `CONTRIBUTING.md` pre-workspace-restructuring). Must reflect the dev container, current `make` targets, Clippy requirements (`-D warnings`), and pre-commit hooks.

### FR-30: Update Contributing — `testing-guide.md`
In-place update. Must cover unit tests, integration tests (`make test-all`, `make test-integration-docker`), and doc tests.

### FR-31: Update Contributing — `architecture-decisions.md`
In-place update (was `adapter-development.md`). Must reflect current adapter locations and port trait contracts.

### FR-32: Update Contributing — `contributing-providers.md`
In-place update. Must reflect current LLM provider adapter structure under `crates/paladin-llm/`.

### FR-33: Update `introduction.md`
In-place update. Fix the 14 broken links to old flat paths (e.g., `QUICKSTART.md` → `getting-started/quickstart.md`).

---

## 5. Non-Goals (Out of Scope)

- **Appendix files** — The 35 appendix files are reference/archive material and are not rewritten in this Epic.
- **New documentation** — New chapters (orchestration guide, content processing, crate map additions) are Epic 4.
- **Rust source code changes** — This Epic modifies only `docs/src/` markdown files, `docs/book.toml`, `.github/workflows/docs.yml`, and the `Makefile` (for the `check-doc-examples` target). No `*.rs` or `Cargo.toml` changes.
- **External link validation** — `follow-web-links = false` in linkcheck; external URLs are not verified.
- **`mdbook serve` / visual review tooling** — Out of scope; developers use `mdbook serve docs/` locally.

---

## 6. Design Considerations

### Execution Order (Sequential)
Tasks must be completed in this order to avoid reworking cross-references:

1. **FR-1** — Fix all 227 broken links and re-enable linkcheck first. Subsequent rewrites write correct links from the start.
2. **FR-2 / FR-3** — Add pre-commit and CI code-check infrastructure before any code examples are written.
3. **FR-4 → FR-6** — Getting Started (full rewrite). These are the highest-risk pages — any compilation failure here blocks new users.
4. **FR-7 → FR-11** — Architecture (in-place). Establishes correct module paths that User Guides depend on.
5. **FR-12 → FR-17** — User Guides (full rewrite). Reference architecture docs for import paths.
6. **FR-18 → FR-25** — Deployment and Operations (in-place).
7. **FR-26 → FR-32** — API Reference and Contributing (mixed).
8. **FR-33** — Introduction link fixes (can run in parallel with any step).

### Code Example Conventions
- All fenced Rust blocks must be self-contained or use `# use paladin_core::...;` hiding lines (mdBook syntax) for brevity.
- Use `# #[allow(unused)]` hiding lines where needed to prevent dead-code `cargo check` warnings.
- Examples that require a running service must be annotated with a `> **Prerequisites:** Run \`make dev\` first.` callout.

### Mermaid Diagrams
- Use Mermaid `graph TD` or `flowchart TD` for architecture diagrams; `sequenceDiagram` for lifecycle flows.
- All diagrams must render in the current mdbook-mermaid v0.13.0 build.

---

## 7. Technical Considerations

### Workspace Crates (v0.4.3)
| Crate | Package Name | Layer |
|---|---|---|
| `crates/paladin-core` | `paladin-ai-core` | Core |
| `crates/paladin-ports` | `paladin-ports` | Application |
| `crates/paladin-battalion` | `paladin-battalion` | Application |
| `crates/paladin-llm` | `paladin-llm` | Infrastructure |
| `crates/paladin-memory` | `paladin-memory` | Infrastructure |
| `crates/paladin-storage` | `paladin-storage` | Infrastructure |
| `crates/paladin-content` | `paladin-content` | Infrastructure |
| `crates/paladin-notifications` | `paladin-notifications` | Infrastructure |
| `crates/paladin-web` | `paladin-web` | Infrastructure |

### Code Block Extraction Script
The CI step (FR-3) and pre-commit hook (FR-2) use a script at `scripts/check-doc-examples.sh` that:
1. Uses `grep`/`awk` to extract all fenced ` ```rust ` blocks from `docs/src/**/*.md`.
2. Creates a temporary scratch crate directory (e.g., `/tmp/paladin-doc-check-$$`) with a `Cargo.toml` that depends on the workspace, writes each extracted block as a `src/main.rs`, and runs `cargo check`.
3. Deletes the temp directory on exit (both success and failure) via a `trap` cleanup.
4. Reports which source file and which block number failed.

### `docs/book.toml` changes
After FR-1 is complete, uncomment and activate:
```toml
[output.linkcheck]
follow-web-links = false
warning-policy = "error"
```

### Dependencies
- **Milestone 9 Epics 1–3 must be complete** before FR-12 through FR-17 (User Guides) are written — the orchestrator and bridge APIs must be stable.
- All other FRs can begin once Epic 2 is merged to `main` (already done).

---

## 8. Success Metrics

| Metric | Target |
|---|---|
| Broken internal links | 0 (linkcheck passes with `warning-policy = "error"`) |
| Fenced Rust code blocks failing `cargo check` | 0 |
| `mdbook build` warnings | 0 |
| Getting Started guides: end-to-end example compiles and runs | Yes |
| Files rewritten or updated | 33 (all `docs/src/` non-appendix files) |
| CI `docs.yml` build step: code-check passes | Yes |

---

## 9. Open Questions

All open questions have been resolved.

| ID | Question | Resolution |
|---|---|---|
| OQ-1 | Minimum supported Rust version for `installation.md`? | No `rust-toolchain.toml` exists. Document the current stable Rust version at time of writing. Note that `edition = "2024"` requires Rust ≥ 1.85. See FR-4. |
| OQ-2 | Are Milestone 9 APIs stable enough to begin User Guide rewrites? | Yes — Milestone 9 Epics 1–3 are complete. FR-12 through FR-17 are unblocked. |
| OQ-3 | Permanent scratch crate or temp directory for code-check script? | Temp directory (`/tmp/paladin-doc-check-$$`), created and deleted by the script via `trap` cleanup. See FR-2 / Technical Considerations. |
| OQ-4 | Is a running Qdrant instance required for `sanctum-vector-memory.md`? | Yes — use the Qdrant instance in the dev container (confirm port from `docker/docker-compose.dev.yml`). See FR-16. |
| OQ-5 | Pre-commit hook via `.pre-commit-config.yaml` or `Makefile` only? | Both: add as a `pre-push` stage hook in `.pre-commit-config.yaml` (consistent with existing `cargo-build-push` pattern) plus a `make check-doc-examples` convenience target. See FR-2. |

---

## Relevant Files

### Files to Rewrite (Full Rewrite)
- `docs/src/getting-started/installation.md`
- `docs/src/getting-started/quickstart.md`
- `docs/src/getting-started/configuration.md`
- `docs/src/user-guides/paladin-agents.md`
- `docs/src/user-guides/battalion-patterns.md`
- `docs/src/user-guides/arsenal-tools.md`
- `docs/src/user-guides/garrison-memory.md`
- `docs/src/user-guides/sanctum-vector-memory.md`
- `docs/src/user-guides/herald-output.md`
- `docs/src/api-reference/stable-api.md`
- `docs/src/contributing/development-setup.md`

### Files to Update (In-Place)
- `docs/src/introduction.md` — fix 14 broken links
- `docs/src/architecture/overview.md`
- `docs/src/architecture/hexagonal-design.md`
- `docs/src/architecture/domain-model.md`
- `docs/src/architecture/crate-map.md`
- `docs/src/architecture/design-patterns.md`
- `docs/src/user-guides/maneuver-flow-dsl.md`
- `docs/src/user-guides/memory-management.md`
- `docs/src/user-guides/orchestration.md`
- `docs/src/user-guides/output-formatting.md`
- `docs/src/user-guides/paladin-configuration.md`
- `docs/src/user-guides/tool-integration.md`
- `docs/src/deployment/docker.md`
- `docs/src/deployment/kubernetes.md`
- `docs/src/deployment/production.md`
- `docs/src/deployment/cicd.md`
- `docs/src/operations/logging.md`
- `docs/src/operations/monitoring.md`
- `docs/src/operations/performance-tuning.md`
- `docs/src/operations/troubleshooting.md`
- `docs/src/api-reference/feature-flags.md`
- `docs/src/api-reference/migration-guide.md`
- `docs/src/contributing/testing-guide.md`
- `docs/src/contributing/architecture-decisions.md`
- `docs/src/contributing/contributing-providers.md`

### Infrastructure Files Modified
- `docs/book.toml` — re-enable `[output.linkcheck]`
- `.github/workflows/docs.yml` — add code-block check step
- `Makefile` — add `check-doc-examples` target
- `scripts/check-doc-examples.sh` — new script (FR-2, FR-3)
