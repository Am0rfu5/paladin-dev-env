# Tasks: Epic 6 — Deployment Topologies & Running Multiple Agents

**Project:** Paladin Framework
**Milestone:** 11 — Documentation Review, Reorganization, MDBook Publish
**Epic:** 6 — Deployment Topologies & Running Multiple Agents
**Version Target:** v0.5.0 (documentation written against the current workspace)
**Status:** Not Started
**Created:** 2026-06-06
**Source PRD:** `prd-deployment-topologies-documentation.md`

---

## Relevant Files

### New Files (documentation)
- `docs/src/deployment-topologies/overview.md` — landing / decision-matrix page (FR-2).
- `docs/src/deployment-topologies/embedded-library.md` — single-process topology (FR-3).
- `docs/src/deployment-topologies/battalion-orchestration.md` — many-agents-one-runtime (FR-4).
- `docs/src/deployment-topologies/http-service-host.md` — Axum agent-host topology (FR-5).
- `docs/src/deployment-topologies/queue-worker.md` — Redis queue/worker topology (FR-6).
- `docs/src/deployment-topologies/sidecar.md` — separate-process sidecar topology (FR-7).

### Files Updated (In-Place)
- `docs/src/SUMMARY.md` — new top-level **Deployment Topologies** section + six entries (FR-1, FR-11).
- `CHANGELOG.md` — `[Unreleased]` entry for the new section (FR-14).
- `docs/src/user-guides/paladin-agents.md` — inbound cross-link from the embedded-library page (optional).
- `docs/src/user-guides/orchestration.md` — inbound cross-link from the Battalion page (optional).
- `docs/src/appendix/redis-queue-adapter-setup.md` — inbound cross-link from the queue/worker page (optional).
- `docs/src/architecture/overview.md` — optional pointer to the new section.

### Tooling (only if required so compilable examples pass — FR-10)
- doc-examples harness manifest (confirm exact path, e.g. `crates/doc-examples/Cargo.toml` /
  `paladin-doc-examples`) — add `axum`, `reqwest`, `tokio` dev-deps, a `paladin-web` dep, and
  activate the `redis-queue` feature so the host/worker/sidecar blocks compile. Do **not** add
  these as runtime deps to any shipped library crate.

### Notes
- This is a **docs-only** Epic. No `*.rs` library/API changes; the only non-markdown edits are
  the doc-examples harness manifest (FR-10) and, if needed, check tooling.
- Reuse the Epic 3 code gate (`scripts/check-doc-examples.sh`, `make check-doc-examples`) and
  the Epic 4 config gate (`scripts/check-doc-config.sh`, `make check-doc-config`); both glob
  `docs/src/**/*.md`, so new files are picked up automatically.
- Compilable-example conventions (inherited from Epics 3–4): prefer `MockLlmAdapter`; use
  `# `-hidden lines for boilerplate `use` statements and for `.await`ed bind/network calls
  (`axum::serve`, `reqwest ... .send().await`, Redis enqueue/dequeue) so blocks **compile**
  under `cargo check` without running a server/service.
- **Honesty mandate (PRD §4):** the HTTP-host page must state there is no shipped
  agent-execution endpoint (`create_app_router` is user-management only); the sidecar page must
  state there is no IPC/gRPC/RPC transport. Document composition, never invent APIs.
- Verify every type/method/path/feature flag against source before publishing (PRD §7 anchors).
- **Completion checks for this Epic are docs-focused:** `mdbook build` (zero warnings,
  linkcheck enforcing) + `make check-doc-examples` + `make check-doc-config` + `cargo test`,
  plus a `CHANGELOG` `[Unreleased]` entry. **Skip `project/current-exports.txt` / public-API
  baseline regeneration** — there is no API change.

## Instructions for Completing Tasks
- Mark each sub-task `[x]` as soon as it is finished.
- When all sub-tasks under a parent are done, run the docs gates, then mark the parent `[x]`.
- Stop after each parent task and wait for the go-ahead before starting the next.

## Tasks

- [x] 0.0 Create feature branch
  - [x] 0.1 Ensure `main` is up to date: `git checkout main && git pull`. *(Fetch showed no new commits — `main` already current; skipped `pull` to avoid disturbing pre-existing uncommitted changes unrelated to this Epic.)*
  - [x] 0.2 Create and checkout the branch: `git checkout -b feature/milestone-11-epic-6-deployment-topologies-documentation`.
  - [x] 0.3 Confirm a clean working tree: `git status`. *(Pre-existing unrelated changes noted — `.claude/settings.json`, `README.md`, milestone-file move — will NOT be committed by this Epic.)*

- [x] 1.0 Scaffold the Deployment Topologies section & landing decision-matrix page (FR-1, FR-2, FR-8)
  - [x] 1.1 Confirm the book's section-landing convention (resolve OQ-5): inspected
        `docs/src/SUMMARY.md` — sections are mdBook `# Part` headers followed by a flat page
        list; there is **no separate parent/landing page** per section. Resolution: the landing
        page is the first list item, named `overview.md` (matching `architecture/overview.md`).
  - [x] 1.2 Created `docs/src/deployment-topologies/` and the landing page
        `docs/src/deployment-topologies/overview.md`.
  - [x] 1.3 Added the **Deployment Topologies** part header to `docs/src/SUMMARY.md`
        immediately before **Deployment** (after User Guides), with the landing page + five
        child entries; created the five child stub files so there are no dangling links.
  - [x] 1.4 Wrote the landing page body: "topology vs packaging" framing, a comparison table
        (topology, process model, concurrency, when to use, when to avoid, key crates/features),
        and the "two are documented in depth — see their pages" note.
  - [x] 1.5 Added the Mermaid `flowchart TD` decision tree routing to each of the five topologies.
  - [x] 1.6 Verified: `mdbook build` exits 0 with "No broken links found"; no warnings
        reference the new pages (remaining `fragment resolution` warnings are pre-existing in
        unrelated files and non-fatal).

- [ ] 2.0 Prepare the doc-example & config-check harness for the new examples (FR-10, FR-12)
  - [ ] 2.1 Locate the doc-examples test harness manifest (resolve OQ-6): find the crate that
        `scripts/check-doc-examples.sh` compiles extracted blocks against, and read its
        `Cargo.toml` to see how features/deps are declared.
  - [ ] 2.2 Add the dev-dependencies the host/worker/sidecar examples need — `axum`,
        `reqwest`, `tokio` — and a dependency on `paladin-web`; activate the `redis-queue`
        feature (and `web-server` if it gates `paladin-web`). Do **not** add these as runtime
        deps to any shipped library crate.
  - [ ] 2.3 Confirm the config-loader entry point used by `scripts/check-doc-config.sh` (the
        struct `config.yml` deserializes into) so the host/queue YAML snippets use real keys.
  - [ ] 2.4 Smoke-test the harness with a throwaway compilable block that imports `axum`,
        `reqwest`, and the `redis-queue`-gated `RedisQueueAdapter`: run
        `make check-doc-examples` (or `scripts/check-doc-examples.sh`) and confirm it
        compiles, then remove the throwaway block.

- [ ] 3.0 Author the Embedded Library topology page (FR-3)
  - [ ] 3.1 Verify the embedded-library API anchors against source (PRD §7): `PaladinBuilder`,
        `PaladinExecutionService::new(..)`/`execute(..)`, `MockLlmAdapter::new()`,
        `CircuitBreaker`, and the prelude re-exports.
  - [ ] 3.2 Write the page intro: the single-process model, `paladin-ai` (lib `paladin`) as
        composition root; when to choose it (simplest, in-process, you control invocation).
  - [ ] 3.3 Add the compilable example: build one agent with `PaladinBuilder` + `MockLlmAdapter`,
        construct `PaladinExecutionService`, call `.execute(&paladin, input)`. Use `# `-hidden
        boilerplate; no API key/network required.
  - [ ] 3.4 Add the "multiple distinct agents in one process" note introducing the agent-registry
        pattern (`HashMap<AgentId, (Paladin, Arc<PaladinExecutionService>)>`) and a forward
        link to the HTTP-host page where the registry is served.
  - [ ] 3.5 Add cross-links: → `user-guides/paladin-agents.md` (full builder API) and ← back to
        the landing page; do not duplicate the builder reference.
  - [ ] 3.6 Verify: `make check-doc-examples` passes for this file and `mdbook build docs/` is
        warning-free.

- [ ] 4.0 Author the Battalion Orchestration topology page (FR-4)
  - [ ] 4.1 Verify Battalion anchors against source (PRD §7): the chosen service type
        (`PhalanxExecutionService` or `FormationExecutionService`), its `::new(paladin_port)`
        and `execute(..)` signatures, `BattalionConfig`, `ErrorStrategy`, `AggregationStrategy`.
  - [ ] 4.2 Write the page intro: "many agents collaborating in one runtime" and when it beats
        a hand-rolled registry (the agents form a *workflow*, not independent endpoints).
  - [ ] 4.3 Add the compilable example: a Battalion service with ≥2 agents (mock LLM port) and a
        `BattalionConfig`, executed end-to-end.
  - [ ] 4.4 Add the intent→pattern table (sequential→Formation, parallel→Phalanx, DAG→Campaign,
        hierarchical→Chain of Command, dynamic→Commander).
  - [ ] 4.5 Add cross-links: → `user-guides/orchestration.md` and
        `user-guides/battalion-patterns.md` for per-pattern depth; ← landing page. No duplication.
  - [ ] 4.6 Verify: `make check-doc-examples` passes for this file; `mdbook build docs/` clean.

- [ ] 5.0 Author the HTTP Service Host topology page (FR-5)
  - [ ] 5.1 Verify anchors against source (PRD §7): `axum` version/API, that
        `paladin-web::create_app_router(user_service, auth_port)` is **user-management only**,
        and the `web-server` feature flag. Confirm no agent-execution endpoint ships.
  - [ ] 5.2 Write the page intro + **honesty callout** (mandate #1): the framework ships no
        agent-execution endpoint; this page shows how to compose your own.
  - [ ] 5.3 Add the compilable example: an Axum `Router` with `POST /agents/{id}/execute` over
        shared state (`Arc` agent registry + `Arc<PaladinExecutionService>`), the handler
        calling `.execute()` and returning JSON. Show the `axum::serve(..)` bind line as a
        `# `-hidden / `no_run`-style line so the block compiles without binding a port.
  - [ ] 5.4 Add a `config.yml` snippet for host/agent configuration (must validate — FR-12).
  - [ ] 5.5 Optionally add a `sequenceDiagram` (client → handler → execution service → agent).
  - [ ] 5.6 Add cross-links: → `api-reference/crate-map.md` (paladin-web bundled user/auth
        routes), → `sidecar.md`, ← landing page.
  - [ ] 5.7 Verify: `make check-doc-examples` + `make check-doc-config` pass for this file;
        `mdbook build docs/` clean.

- [ ] 6.0 Author the Queue / Worker (distributed) topology page (FR-6)
  - [ ] 6.1 Verify anchors against source (PRD §7): `RedisQueueAdapter::new(config, log_port)`,
        `RedisQueueConfig` fields, the queue port traits
        (`QueuePort`/`BatchQueuePort`/`FullQueuePort`), `QueueItem<T>`, and the `redis-queue`
        feature flag.
  - [ ] 6.2 Write the page intro: producer/worker model — enqueue agent jobs, worker pool
        dequeues and executes; backpressure, retries, scale-out, fault isolation.
  - [ ] 6.3 Add the compilable example (feature-gated `redis-queue`): producer builds
        `RedisQueueConfig` + `RedisQueueAdapter` and enqueues a job payload; worker dequeues,
        deserializes, runs a `PaladinExecutionService`, marks complete. Use `no_run`-style
        hiding for the `.await`ed Redis/network calls so it compiles without a live Redis, but
        exercise the real types/signatures.
  - [ ] 6.4 Add a `config.yml` snippet for the Redis queue (must validate — FR-12).
  - [ ] 6.5 Add the `> **Prerequisites:** Run \`make dev\` (Redis) first; enable the
        \`redis-queue\` feature.` callout.
  - [ ] 6.6 Add cross-links: → `appendix/redis-queue-adapter-setup.md` (infra setup), ← landing.
  - [ ] 6.7 Verify: `make check-doc-examples` + `make check-doc-config` pass for this file;
        `mdbook build docs/` clean.

- [ ] 7.0 Author the Sidecar (separate-process) topology page (FR-7)
  - [ ] 7.1 Re-confirm (PRD §7 / OQ-3) there is no IPC/gRPC/tonic/RPC client in the workspace,
        so the page is honest about composing HTTP host + HTTP client.
  - [ ] 7.2 Write the page intro: agent in its own process, called over the network; when
        process/security/deploy isolation justifies the overhead vs. the one-process HTTP host.
  - [ ] 7.3 Add the **honesty callout** (mandate #2): no first-class sidecar transport ships;
        the wire contract is consumer-owned.
  - [ ] 7.4 Add the compilable caller-side example: a `reqwest` client call to the FR-5
        `POST /agents/{id}/execute` contract with request/response structs; `no_run`-style so
        it compiles without a running server. Cross-link the server side to the HTTP-host page.
  - [ ] 7.5 Add the "what a first-class sidecar would need" subsection (transport port trait +
        serialization) as a documented limitation / future direction, linked to OQ-3.
  - [ ] 7.6 Add cross-links: → `http-service-host.md`, ← landing page.
  - [ ] 7.7 Verify: `make check-doc-examples` passes for this file; `mdbook build docs/` clean.

- [ ] 8.0 Cross-link, register, and verify the full book build (FR-9, FR-11, FR-12, FR-13)
  - [ ] 8.1 Confirm all six pages are registered under the Deployment Topologies section in
        `docs/src/SUMMARY.md` (no stubs left; no "not in SUMMARY" warnings).
  - [ ] 8.2 Verify the cross-link map (PRD §6): landing ↔ each topology; embedded-library →
        paladin-agents; battalion → orchestration/battalion-patterns; http-host →
        crate-map + sidecar; queue-worker → redis-queue-adapter-setup; sidecar → http-host.
  - [ ] 8.3 Add optional inbound cross-links from existing pages where natural
        (`user-guides/paladin-agents.md`, `user-guides/orchestration.md`,
        `appendix/redis-queue-adapter-setup.md`, `architecture/overview.md`).
  - [ ] 8.4 Run the full gates: `make check-doc-examples` (0 failures), `make check-doc-config`
        (0 failures), and `mdbook build docs/` with `[output.linkcheck]` enforcing (**0
        warnings**, every internal link resolves).
  - [ ] 8.5 Run `cargo test` to confirm no regressions from the harness manifest change (2.0).

- [ ] 9.0 CHANGELOG, final checks, commit & PR (docs-focused completion — FR-14)
  - [ ] 9.1 Add a `## [Unreleased]` entry to `CHANGELOG.md` recording the new Deployment
        Topologies section (docs scope).
  - [ ] 9.2 Confirm scope hygiene: no `*.rs` library changes; **do not** regenerate
        `project/current-exports.txt` (docs-only Epic — no API change). Review `git diff` to
        ensure only docs, `SUMMARY.md`, `CHANGELOG.md`, and the doc-examples harness manifest
        changed.
  - [ ] 9.3 Run the final check sweep: `cargo fmt --check`, `cargo clippy --workspace
        --all-targets -- -D warnings` (the harness manifest change must not introduce
        warnings), `make check-doc-examples`, `make check-doc-config`, `mdbook build docs/`.
  - [ ] 9.4 Remove any throwaway/placeholder content left from scaffolding (1.2/1.3 stubs,
        2.4 smoke-test block).
  - [ ] 9.5 Stage and commit with a conventional message and co-author trailer, e.g.
        `git commit -m "docs(milestone-11/epic-6): add Deployment Topologies section" -m "- Landing decision matrix + 5 topology pages (embedded, battalion, http-host, queue-worker, sidecar)" -m "- Compilable examples gated by check-doc-examples; config snippets validated" -m "- Honest gaps flagged: no agent-HTTP endpoint, no IPC/sidecar transport" -m "Implements Epic 6 of Milestone 11"`.
  - [ ] 9.6 Push the branch and open a PR to `main`: title
        `docs(M11/E6): Deployment Topologies & Running Multiple Agents`; body summarizing the
        new section, the compilable-example/config gates, and the documented gaps (OQ-2/OQ-3).
        Confirm CI (`docs.yml` mdbook build + doc gates) is green.
