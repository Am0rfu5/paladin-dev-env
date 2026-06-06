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

- [x] 2.0 Prepare the doc-example & config-check harness for the new examples (FR-10, FR-12)
  - [x] 2.1 Resolved OQ-6: the harness crate is `crates/doc-examples` (package
        `paladin-doc-examples`). Examples live as `// ANCHOR: name` regions in
        `src/*.rs`, compiled by `cargo check -p paladin-doc-examples` and pulled into guides
        via mdBook `{{#include ...:anchor}}`. Inline fenced blocks are only syntax-scanned
        (and skipped if they import external crates) — so **compilable** topology examples
        must be `{{#include}}`-d from this crate, not written inline.
  - [x] 2.2 Added to `crates/doc-examples/Cargo.toml`: `axum = "0.8.4"` (matches `paladin-web`),
        `reqwest` (workspace), `serde` (workspace, derive — for request/response structs), and
        `paladin-storage` with the `redis-queue` feature. `tokio` was already present. No
        shipped-library crate was modified. (`web-server` is an umbrella feature on
        `paladin-ai`, not on `paladin-web`, which is depended on directly when needed — the
        HTTP-host example composes its own router, so no `paladin-web` dep was required.)
  - [x] 2.3 Read `scripts/check-doc-config.sh`: the config gate is **syntactic only** (PyYAML
        `safe_load_all`), not deep schema validation (deep validation is a tracked follow-up,
        OQ-7). YAML snippets must parse as valid YAML; I will still use realistic keys for
        accuracy, but the gate enforces syntax, not schema.
  - [x] 2.4 Smoke-tested with a throwaway `src/_smoke.rs` importing `axum::Router`,
        `reqwest::Client`, and `paladin_storage::redis::{RedisQueueAdapter, RedisQueueConfig}`:
        `cargo check -p paladin-doc-examples` compiled, confirming all three resolve. Removed
        the throwaway file; `make check-doc-examples` passes (0 failed).

- [x] 3.0 Author the Embedded Library topology page (FR-3)
  - [x] 3.1 Verified anchors against source: `PaladinBuilder::new(Arc<dyn LlmPort>)` +
        `.build().await`; `PaladinExecutionService::new(llm, circuit_breaker, garrison,
        arsenal)` + `.execute(&paladin, input).await`; `MockLlmAdapter::new()`;
        `CircuitBreaker::new(failures, successes, Duration)`; prelude exports `Paladin`,
        `LlmPort`, `PaladinResult`, `PaladinBuilder`. The single-agent pattern already exists,
        compiled, as `readme.rs:quickstart`.
  - [x] 3.2 Wrote the intro: single-process model, `paladin-ai` (lib `paladin`) as composition
        root, when to choose it / when to look elsewhere.
  - [x] 3.3 Added the single-agent example by reusing the compiled `readme.rs:quickstart`
        anchor via `{{#include}}` (DRY — no duplicate source to drift).
  - [x] 3.4 Added the "multiple distinct agents in one process" section with a **new compiled**
        `AgentRegistry` example (`crates/doc-examples/src/deployment_topologies.rs`,
        `embedded_registry` anchor) — `HashMap<String, (Paladin, Arc<PaladinExecutionService>)>`
        — and a forward link to the HTTP-host page.
  - [x] 3.5 Added cross-links: → `user-guides/paladin-agents.md`, →
        `battalion-orchestration.md`, → `http-service-host.md`, ← landing page.
  - [x] 3.6 Verified: `make check-doc-examples` passes (registry compiles via
        `cargo check -p paladin-doc-examples`); `mdbook build` exits 0, no broken links; both
        `{{#include}}` directives confirmed rendered into the built HTML.

- [x] 4.0 Author the Battalion Orchestration topology page (FR-4)
  - [x] 4.1 Verified the reused anchor against source: `orchestration.rs:phalanx` uses
        `PhalanxExecutionService::new(paladin_port)` + `.execute(&phalanx, input)`,
        `Phalanx::new(vec![...], BattalionConfig)`, `AggregationStrategy::CollectAll`,
        `with_max_concurrency`; `BattalionResult.final_output` confirmed. Already compiled.
  - [x] 4.2 Wrote the intro: "many agents collaborating in one runtime" and when it beats a
        hand-rolled registry (agents form a *workflow*, not independent endpoints).
  - [x] 4.3 Added the compilable example by reusing the existing `orchestration.rs:phalanx`
        anchor (≥2 agents, `BattalionConfig`, concurrency cap) — no duplicate source.
  - [x] 4.4 Added the intent→pattern table (Formation / Phalanx / Campaign / Chain of Command /
        Commander, with the service type for each).
  - [x] 4.5 Added cross-links: → `user-guides/orchestration.md`,
        `user-guides/battalion-patterns.md`, ← landing; plus pointers to worker/http-host as
        composition. No duplication of the pattern guides.
  - [x] 4.6 Verified: `make check-doc-examples` passes; `mdbook build` exits 0, no broken
        links; the Phalanx `{{#include}}` confirmed rendered into the built HTML.

- [x] 5.0 Author the HTTP Service Host topology page (FR-5)
  - [x] 5.1 Verified against source: read `crates/paladin-web/src/app.rs` —
        `create_app_router(user_service, auth_port)` wires only `/users/...` register/login/CRUD
        (user-management), **no agent endpoint**. Axum is `0.8.x` (route param syntax `{id}`).
        Honesty callout #1 is accurate.
  - [x] 5.2 Wrote the intro + **honesty callout**: Paladin ships no agent-execution endpoint;
        this page composes an `axum` handler around `PaladinExecutionService`.
  - [x] 5.3 Added the **compilable** example (`http_service_host.rs:http_host`): `axum` Router
        with `POST /agents/{id}/execute` over shared `AppState` (Arc agent registry), handler
        calling `.execute()` and returning JSON. The real `axum::serve` bind compiles in full
        (cargo check never runs it, so no port is bound) — no hiding needed.
  - [x] 5.4 Added a `config.yml` host/agent snippet (validates — `check-doc-config` passes).
  - [x] 5.5 Added a `sequenceDiagram` (client → handler → execution service → agent).
  - [x] 5.6 Added cross-links: → `api-reference/crate-map.md` (paladin-web user/auth routes),
        → `sidecar.md`, ← landing; embedded-library referenced for the registry.
  - [x] 5.7 Verified: `make check-doc-examples` (0 failed) + `make check-doc-config` (156 YAML
        blocks, 0 failed) pass; `mdbook build` exits 0, no broken links; include rendered.

- [x] 6.0 Author the Queue / Worker (distributed) topology page (FR-6)
  - [x] 6.1 Verified against source: `RedisQueueAdapter::new(RedisQueueConfig, Option<Arc<dyn
        LogPort>>)`; `RedisQueueConfig` fields (host/port/db/timeout/key_prefix/max_retries);
        `QueuePort::{create_queue, enqueue, dequeue, start_processing, complete_processing}`;
        `QueueItem::new(queue_name, Message, Option<QueueItemConfig>)`; `redis-queue` feature.
        **Drift caught:** the queue-port doc-comment example (`rust,ignore`) uses a stale
        `Message::new(task, source_str, Location::Local)` — the real signature is
        `Message::new(source: Location, destination: Location, message: T)` and `Location` has
        no `Local` variant (used `Location::service(..)`). Used the verified API.
  - [x] 6.2 Wrote the intro: producer/worker model, backpressure, retries, scale-out, isolation.
  - [x] 6.3 Added the **compilable** example (`queue_worker.rs:queue`, `redis-queue`-gated):
        producer builds `RedisQueueConfig` + `RedisQueueAdapter` and enqueues a typed
        `AgentJob`; worker dequeues (generic JSON), reads the input via `message.payload()`,
        runs `PaladinExecutionService`, marks complete. Redis `.await`s compile but never run
        under `cargo check`, so no live Redis is needed.
  - [x] 6.4 Added a `config.yml` Redis-queue snippet (validates — `check-doc-config` passes).
  - [x] 6.5 Added the `> **Prerequisites:** make dev + redis-queue feature` callout.
  - [x] 6.6 Added cross-links: → `appendix/redis-queue-adapter-setup.md`, →
        `embedded-library.md`, → `battalion-orchestration.md`, ← landing.
  - [x] 6.7 Verified: `make check-doc-examples` (0 failed) + `make check-doc-config` (157 YAML
        blocks, 0 failed) pass; `mdbook build` exits 0, no broken links; include rendered.

- [x] 7.0 Author the Sidecar (separate-process) topology page (FR-7)
  - [x] 7.1 Re-confirmed: `grep` for `tonic|grpc|rpc|sidecar|prost` across all crate manifests
        found nothing (only my own doc-examples comment); `reqwest` is used solely for
        LLM/content fetching, not inter-process agent calls. No IPC/RPC transport ships.
  - [x] 7.2 Wrote the intro: agent in its own process, called over the network; when
        isolation justifies the overhead vs. the one-process HTTP host.
  - [x] 7.3 Added the **honesty callout** (mandate #2): no first-class sidecar transport ships;
        the wire contract is consumer-owned.
  - [x] 7.4 Added the **compilable** caller-side example (`sidecar.rs:sidecar_client`): a
        `reqwest` POST to `POST /agents/{id}/execute` with request/response structs mirroring
        the host. Compiles in full; the network call never runs under `cargo check`. Server
        side cross-linked to the HTTP-host page.
  - [x] 7.5 Added the "what a first-class sidecar would need" subsection (transport port trait
        + serialization contract) as a documented limitation / future direction (OQ-3).
  - [x] 7.6 Added cross-links: → `http-service-host.md`, → `embedded-library.md`, ← landing.
  - [x] 7.7 Verified: `make check-doc-examples` passes (0 failed); `mdbook build` exits 0, no
        broken links; the `{{#include}}` rendered into the built HTML.

- [x] 8.0 Cross-link, register, and verify the full book build (FR-9, FR-11, FR-12, FR-13)
  - [x] 8.1 Confirmed all six pages registered under the Deployment Topologies section in
        `docs/src/SUMMARY.md`; no stub markers remain; no "not in SUMMARY" warnings.
  - [x] 8.2 Verified the cross-link map (PRD §6): landing ↔ each topology (overview links to
        all 5; each links back); embedded-library → paladin-agents; battalion →
        orchestration + battalion-patterns; http-host → crate-map + sidecar; queue-worker →
        redis-queue-adapter-setup; sidecar → http-host. All resolve under linkcheck.
  - [x] 8.3 Added inbound cross-links from `user-guides/paladin-agents.md`,
        `user-guides/orchestration.md`, `appendix/redis-queue-adapter-setup.md`, and
        `architecture/overview.md` pointing into the new section.
  - [x] 8.4 Full gates pass: `make check-doc-examples` (0 failed), `make check-doc-config`
        (157 YAML blocks, 0 failed), `mdbook build` exit 0 with linkcheck enforcing — "No
        broken links found" (remaining `fragment resolution` warnings are pre-existing and
        non-fatal).
  - [x] 8.5 `cargo test --workspace --lib --bins` passes — 0 failed across all crates
        (no regressions from the Task 2.0 manifest change).

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
