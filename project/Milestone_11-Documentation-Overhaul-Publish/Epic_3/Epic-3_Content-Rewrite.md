# Epic 3: Content Rewrite

**Project:** Paladin Framework
**Milestone:** 11 — Documentation Review, Reorganization, MDBook Publish
**Epic:** 3 — Content Rewrite
**Version Target:** v0.5.0
**Status:** Not Started
**Created:** 2026-05-29

---

## Milestone Context

The project has ~40 markdown documentation files accumulated across eight milestones of development and refactoring. Many reference pre-refactoring paths (`application_settings.rs`, old import paths), deprecated configuration patterns, or contain placeholder content. This Epic rewrites all stale documentation identified in Epic 1's audit so that every code example compiles, every path is current, and every configuration example matches the current schema.

### Milestone Success Criteria (for reference)

- Every existing doc file is audited: current, stale, or delete.
- MDBook builds locally and via CI with zero warnings.
- Documentation published to GitHub Pages (or equivalent).
- All code examples in docs compile against the current workspace.
- New documentation covers: orchestration guide, content processing, crate map, agent↔orchestrator bridge.
- The main `paladin-dev-env` monorepo includes the docs as a subdirectory.

---

## Parallel Execution Context

**Epic 3 has dependencies and should not begin until:**

1. **Epic 1 (Content Audit)** is complete — the audit output determines which files need rewriting and what specific issues must be fixed.
2. **Epic 2 (MDBook Setup)** is complete — rewrites go directly into the MDBook `docs/src/` structure.
3. **Milestone 8** is complete — final directory structure must be stable so all paths are correct.
4. **Milestone 9 Epics 1–3** are complete — the orchestrator and bridge APIs must be stable before documenting them.

Epic 4 (New Documentation) can run in parallel with Epic 3 once the above prerequisites are met.

---

## Epic Overview

**Priority:** High
**Estimated Effort:** Large
**Dependencies:** Epic 1, Epic 2, Milestone 8, Milestone 9 Epics 1–3

### Objective

Rewrite all stale documentation identified in the Epic 1 audit. Every code example must compile against the current workspace. Every import path must be current. Every configuration example must match the current `config.yml` schema.

### Quality Bar

Every rewritten document must satisfy:

- All `use` statements and import paths resolve against the current workspace.
- All code examples pass `cargo check` (at minimum) or `cargo test --doc` where applicable.
- All `config.yml` snippets contain only keys present in the current configuration schema.
- No references to deleted files, removed modules, or deprecated APIs.
- All cross-references between documents use relative links valid within the MDBook structure.

---

## Tasks

### Task 3.1: Rewrite Getting Started Guides

**Description:**

Rewrite the three foundational guides that new users encounter first. These must be accurate, complete, and use working examples.

**Files:**

- `docs/src/getting-started/installation.md`
- `docs/src/getting-started/quickstart.md`
- `docs/src/getting-started/configuration.md`

**Content Requirements:**

**`installation.md`** must cover:
- Current Rust toolchain version requirement (from `rust-toolchain.toml` or `Cargo.toml`).
- All system prerequisites (system libraries, Docker for services, etc.).
- Workspace crate installation (add to `Cargo.toml` with current crate names and versions).
- Feature flag profiles for common use cases ("minimal", "full", "with-redis", etc.).
- Verification step: a minimal code snippet that compiles against the installed crates.

**`quickstart.md`** must cover:
- End-to-end "hello world" Paladin agent example (compiles and runs).
- Service startup (Docker Compose or equivalent) if required.
- Expected output.
- Pointer to full configuration reference.

**`configuration.md`** must cover:
- Complete `config.yml` schema reflecting the post-Milestone 6 decomposed configuration.
- All top-level sections: `paladin`, `garrison`, `arsenal`, `llm`, and any others present.
- Every configuration key with its type, default, and description.
- Environment variable overrides.
- Multi-environment configuration patterns.

**Deliverables:**

- Three rewritten guide files.
- All code examples verified to pass `cargo check`.

---

### Task 3.2: Rewrite Architecture Documentation

**Description:**

Update architecture docs to reflect the final workspace structure established by Milestone 8.

**Files:**

- `docs/src/architecture/overview.md`
- `docs/src/architecture/hexagonal-design.md`
- `docs/src/architecture/domain-model.md`
- `docs/src/architecture/crate-map.md` *(new — see also Epic 4 Task 4.4)*
- `docs/src/architecture/design-patterns.md`

**Content Requirements:**

**`overview.md`** must cover:
- The three-layer hexagonal architecture (Core, Application, Infrastructure).
- Dependency flow rule: inward only (Core ← Application ← Infrastructure).
- High-level diagram (Mermaid or ASCII) of the layer relationships.
- How the workspace crates map to the layers.

**`hexagonal-design.md`** must cover:
- Ports and Adapters pattern as used in Paladin.
- How to add a new adapter (step-by-step, referencing current module paths).
- Current port trait locations under `crates/paladin-ports/`.
- Current adapter locations under `crates/*/`.

**`domain-model.md`** must cover:
- All domain entities (Paladin, Battalion, Garrison, Arsenal, Citadel, Herald, etc.) with current module paths.
- The `Node<T>` pattern for domain entities.
- Aggregate roots and bounded contexts.
- Medieval Military naming convention table.

**`crate-map.md`** must cover (see also Epic 4 Task 4.4 for the feature-flag dimension):
- Every workspace crate, its layer (Core / Application / Infrastructure), its purpose.
- Crate dependency graph (Mermaid diagram).
- Which crate to depend on for each capability.

**`design-patterns.md`** must cover:
- Builder pattern (with current `PaladinBuilder` example).
- Error handling pattern (with current error enum examples).
- Port trait pattern (`async_trait`, `Send + Sync`).
- Service composition pattern.

**Deliverables:**

- All five architecture files rewritten with current content.
- Mermaid diagrams render correctly in MDBook.

---

### Task 3.3: Rewrite User Guides

**Description:**

Update each user guide with current import paths, current API signatures, working examples, and cross-references to related guides.

**Files:**

- `docs/src/user-guides/paladin-agents.md`
- `docs/src/user-guides/battalion-patterns.md`
- `docs/src/user-guides/arsenal-tools.md`
- `docs/src/user-guides/garrison-memory.md`
- `docs/src/user-guides/sanctum-vector-memory.md`
- `docs/src/user-guides/herald-output.md`

**Content Requirements per Guide:**

**`paladin-agents.md`** must cover:
- `PaladinBuilder` fluent API with current method signatures.
- `system_prompt`, `name`, `model`, `temperature`, `max_loops`, `stop_words` configuration.
- Attaching a `GarrisonPort` for memory.
- Attaching armaments via `add_armament`.
- Execution via `PaladinExecutionService`.
- `PaladinStatus` lifecycle.
- Working end-to-end example.

**`battalion-patterns.md`** must cover:
- Formation (sequential, output N → input N+1).
- Phalanx (concurrent, parallel processing).
- Campaign (DAG/graph-based orchestration).
- Chain of Command (hierarchical delegation).
- Commander (dynamic strategy router).
- When to use each pattern.
- Working example for each pattern.
- Current module paths for all types.

**`arsenal-tools.md`** must cover:
- MCP STDIO adapter (command-line tools).
- MCP SSE adapter (web services).
- `ArsenalPort` trait.
- Tool discovery and invocation lifecycle.
- Configuration in `config.yml` (`arsenal.mcp_servers`).
- Working example using a real MCP server.

**`garrison-memory.md`** must cover:
- In-memory garrison adapter.
- SQLite garrison adapter.
- `GarrisonPort` trait methods.
- Memory lifecycle (store, retrieve, clear).
- Configuration in `config.yml` (`garrison` section).
- Working example.

**`sanctum-vector-memory.md`** must cover:
- Sanctum vector store overview.
- Current adapter implementation.
- Configuration.
- Semantic search usage.
- Working example.

**`herald-output.md`** must cover:
- Output formatting system.
- Available formatters.
- Custom formatter implementation.
- Working example.

**Deliverables:**

- Six rewritten user guide files.
- All code examples verified to pass `cargo check`.
- All cross-references between guides use valid relative links.

---

### Task 3.4: Rewrite Deployment and Operations Docs

**Description:**

Update deployment and operations documentation for the workspace build structure, current configuration schema, and current feature flags.

**Files:**

- `docs/src/deployment/docker.md`
- `docs/src/deployment/kubernetes.md`
- `docs/src/deployment/production.md`
- `docs/src/operations/logging.md`
- `docs/src/operations/monitoring.md`
- `docs/src/operations/performance-tuning.md`

**Content Requirements:**

**`docker.md`** must cover:
- `docker-compose.dev.yml` services (Redis, MinIO, MySQL).
- `Dockerfile` and `Dockerfile.chef` build stages.
- Environment variable configuration.
- `make dev` and `make services-up` commands.
- Health checks.

**`kubernetes.md`** must cover:
- Kubernetes manifests in `k8s/`.
- Deployment, Service, ConfigMap, and Secret resources.
- Scaling considerations for Battalion workloads.

**`production.md`** must cover:
- Production configuration checklist.
- Secret management.
- TLS configuration.
- Resource limits and tuning.

**`logging.md`** must cover:
- Logging framework in use (tracing/log).
- Log level configuration.
- Structured log format.
- Log aggregation recommendations.

**`monitoring.md`** must cover:
- Metrics exposed (if any).
- Sentinel integration.
- Health check endpoints.
- Alerting recommendations.

**`performance-tuning.md`** must cover:
- Benchmark results from `benches/`.
- Tokio runtime tuning.
- Phalanx concurrency limits.
- Database and queue connection pooling.

**Deliverables:**

- Six rewritten deployment/operations files.
- All shell commands verified to work in the current dev container environment.

---

## Deliverables Summary

| Artifact | Description |
|----------|-------------|
| `docs/src/getting-started/*.md` | 3 rewritten guides |
| `docs/src/architecture/*.md` | 5 rewritten architecture docs |
| `docs/src/user-guides/*.md` | 6 rewritten user guides |
| `docs/src/deployment/*.md` | 3 rewritten deployment docs |
| `docs/src/operations/*.md` | 3 rewritten operations docs |

---

## Definition of Done

- [ ] All stale files identified in Epic 1 audit have been rewritten.
- [ ] Every fenced Rust code block passes `cargo check` (or `cargo test --doc` where applicable).
- [ ] Every configuration example matches the current `config.yml` schema.
- [ ] Every import path resolves in the current workspace.
- [ ] No broken internal MDBook links.
- [ ] `mdbook build` succeeds with zero warnings after all rewrites.

---

## Schedule Reference

| Phase | This Epic | Duration | Predecessors |
|-------|-----------|----------|-------------|
| Phase 2 | Epic 3: Content Rewrite | 2–3 sprints | Milestones 8, 9; Epics 1, 2 |

Runs in parallel with Epic 4 (New Documentation) once prerequisites are met.
