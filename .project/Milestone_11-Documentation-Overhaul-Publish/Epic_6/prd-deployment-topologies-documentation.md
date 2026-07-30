# PRD: Epic 6 — Deployment Topologies & Running Multiple Agents

**Project:** Paladin Framework
**Milestone:** 11 — Documentation Review, Reorganization, MDBook Publish
**Epic:** 6 — Deployment Topologies & Running Multiple Agents
**Version Target:** v0.5.0 (documentation written against the current workspace)
**Status:** Not Started
**Created:** 2026-06-06
**Author:** Paladin Framework Contributors

---

## 1. Introduction / Overview

A recurring consumer question has no good answer in the current documentation: *"I want to
build a number of different agents on top of Paladin — how do I actually run them? One
process? A service I hit? A sidecar? Workers?"*

The framework already supports several answers, but they are scattered:

- **Embedded library** usage is shown in `getting-started/quickstart.md` and
  `user-guides/paladin-agents.md`.
- **Multi-agent orchestration** is documented in `user-guides/battalion-patterns.md` and
  `user-guides/orchestration.md`.
- **HTTP hosting** via `paladin-web` is only *named* in `architecture/overview.md` and
  `api-reference/crate-map.md`; there is no usage guide, and — critically — the shipped
  `create_app_router` is a **user-management** REST API, not an agent-execution endpoint.
- **Queue/worker** execution has an infrastructure setup page
  (`appendix/redis-queue-adapter-setup.md`) but no "run agents as workers" recipe.
- **Sidecar / separate-process** deployment is documented nowhere, and the workspace
  exposes **no IPC/gRPC/RPC abstraction** to support it.

This Epic writes a **new top-level "Deployment Topologies" section** that frames the choice
as a decision and gives each topology a focused, working page. It reuses (cross-links) the
existing embedded-library and Battalion guides rather than duplicating them, and it adds the
net-new HTTP-host, queue/worker, and sidecar pages that don't exist today.

**Key decisions carried into this PRD** (resolved with the maintainer — see §9):

1. **A new top-level section**, `Deployment Topologies`, is added to `SUMMARY.md` between
   **User Guides** and **Deployment**. (The existing `Deployment` section covers
   *packaging* — Docker/K8s/CI-CD — not runtime topology; the two are complementary.)
2. **A landing decision-matrix page + five topology pages** are authored (embedded library,
   Battalion, HTTP host, queue/worker, sidecar). Embedded-library and Battalion pages are
   short and cross-link the existing guides for depth.
3. **All Rust examples are compilable** and gated by the Epic 3 `check-doc-examples.sh`
   harness; all YAML by the Epic 4 config-check gate.
4. **Honesty mandate** (see §4 banner): the HTTP-host and sidecar pages document
   *composition of existing APIs*; where the framework provides no first-class mechanism
   (agent-HTTP endpoint, IPC/RPC), the page says so plainly and records an open question
   instead of inventing an API.
5. **Docs-focused completion checks** — `mdbook build` (zero warnings, linkcheck enforcing)
   + `check-doc-examples` + config-check + `cargo test`, plus a `CHANGELOG` `[Unreleased]`
   entry. **No `current-exports.txt` / public-API baseline regeneration** (docs-only Epic).

---

## 2. Goals

1. **Author a landing decision-matrix page** that lets a reader pick a topology in under a
   minute (comparison table + Mermaid decision flowchart + "when to use / avoid" per
   topology).
2. **Author an Embedded-Library topology page** (single-process) with a compilable example
   building and executing one agent, cross-linking `paladin-agents.md`.
3. **Author a Battalion-Orchestration topology page** (many agents, one runtime) with a
   compilable multi-agent example, cross-linking `orchestration.md` /
   `battalion-patterns.md`.
4. **Author an HTTP-Service-Host topology page** showing how to host an **agent registry**
   behind Axum by composing `axum` + `PaladinExecutionService` — with an honest note that
   `paladin-web::create_app_router` is the user-management API, not an agent endpoint.
5. **Author a Queue/Worker (distributed) topology page** showing a producer enqueuing agent
   jobs and a worker dequeuing and executing them via the Redis queue adapter, cross-linking
   the appendix Redis setup.
6. **Author a Sidecar (separate-process) topology page** composing the HTTP host (server
   side) + an HTTP client (caller side), with an honest "no built-in IPC/RPC" callout and
   guidance on when a sidecar is worth the operational cost.
7. **Register the new section and all six pages in `SUMMARY.md`** so they render in the book
   with no "not in SUMMARY" warnings.
8. **Every fenced Rust example passes `cargo check`** via `scripts/check-doc-examples.sh`;
   **every YAML/config snippet validates** via the config-check gate.
9. **`mdbook build` succeeds with zero warnings** (linkcheck enforcing) and all internal
   cross-links resolve.
10. **Record the new section in `CHANGELOG.md`** under `[Unreleased]`.

---

## 3. User Stories

- **As a developer scoping a new system**, I want one page that compares running agents in a
  single process vs. a hosted service vs. workers vs. a sidecar, with a decision flowchart,
  so I can choose a topology before writing code.
- **As a developer embedding Paladin**, I want a minimal compilable example of constructing
  and running an agent in my own binary, so I can start without reading source.
- **As a developer building a request/response service**, I want to see how to keep several
  distinct agents resident behind an HTTP API and route a request to the right one, so I can
  stand up an agent service.
- **As a developer scaling out**, I want a worker recipe that pulls agent jobs off a queue
  and executes them, so I can add backpressure and horizontal scale.
- **As a developer needing process isolation**, I want to know whether Paladin gives me a
  sidecar/RPC mechanism out of the box, and if not, exactly what I have to write myself, so
  I am not misled into expecting a feature that doesn't exist.
- **As any reader**, I want every example to compile against the version I'm using, so I'm
  never misled by aspirational APIs.

---

## 4. Functional Requirements

> **Naming & honesty mandate (applies to every FR):** The PRD lists API names verified
> against the current workspace, but the author **must re-verify every type, method, module
> path, and feature flag against the source before publishing**. Verified anchors are in §7.
> Two capabilities the Epic *describes* do **not** exist as first-class framework features
> and must be documented as **consumer-composed code with an explicit gap callout**, never
> as an invented API:
> 1. There is **no built-in agent-execution HTTP endpoint**. `paladin-web::create_app_router`
>    serves user-management routes (`/users/...`) and takes a `UserServiceTrait` + `AuthPort`
>    — it does **not** execute agents. The HTTP-host page composes a consumer-owned Axum
>    handler around `PaladinExecutionService`.
> 2. There is **no IPC / gRPC / RPC / sidecar abstraction** in the workspace. The sidecar
>    page composes the HTTP host (server) + an HTTP client (caller); it must state plainly
>    that no first-class sidecar transport ships today.

### Task 6.1 — Section Scaffold & Landing Page

#### FR-1: New top-level section in `SUMMARY.md`
Add a **Deployment Topologies** section header between **User Guides** and **Deployment**,
with six entries: the landing page (parent) and the five topology pages (children). No
existing entries are moved or removed.

#### FR-2: Landing / decision-matrix page (`deployment-topologies/overview.md`)
Must contain:
- A one-paragraph framing of "topology = how you run agents, independent of packaging."
- A **comparison table**: columns = topology, process model, concurrency, when to use,
  when to avoid, key crates/features. Rows = the five topologies.
- A **Mermaid `flowchart TD`** that routes a reader to a topology from questions
  (collaborating-on-one-task? request/response service? scale-out? process isolation?).
- A short "these two are documented in depth elsewhere" note linking the embedded-library
  and Battalion topology pages to their parent guides.

### Task 6.2 — Embedded Library Topology Page

#### FR-3: New file `deployment-topologies/embedded-library.md`
- Explains the single-process model: `paladin-ai` (lib `paladin`) as composition root.
- A **compilable** example: build one agent with `PaladinBuilder`, construct
  `PaladinExecutionService`, run `.execute(&paladin, input)`. Use `MockLlmAdapter` so the
  example compiles and (if run) needs no API key or network.
- A short "multiple distinct agents in one process" note introducing an agent-registry
  pattern (`HashMap<AgentId, (Paladin, Arc<PaladinExecutionService>)>`), pointing forward to
  the HTTP-host page where the registry is served.
- **Cross-link** to `user-guides/paladin-agents.md` for the full builder API rather than
  duplicating it.

### Task 6.3 — Battalion Orchestration Topology Page

#### FR-4: New file `deployment-topologies/battalion-orchestration.md`
- Explains "many agents collaborating in one runtime" and when this beats a hand-rolled
  registry (the agents form a *workflow*, not independent endpoints).
- A **compilable** example using one Battalion service (Phalanx or Formation) with ≥2
  agents and a `BattalionConfig`. Use the mock LLM port.
- A pointer table mapping intent → pattern (sequential→Formation, parallel→Phalanx,
  DAG→Campaign, hierarchical→Chain of Command, dynamic→Commander).
- **Cross-link** to `user-guides/orchestration.md` and `user-guides/battalion-patterns.md`
  for per-pattern depth; do **not** duplicate those guides.

### Task 6.4 — HTTP Service Host Topology Page

#### FR-5: New file `deployment-topologies/http-service-host.md`
- Explains the "one service, many agents resident, concurrent requests" model.
- **Honesty callout (FR mandate #1):** state that the framework ships no agent-execution
  endpoint; `paladin-web::create_app_router` is the user/auth REST surface. This page shows
  how to *compose your own* endpoint.
- A **compilable** example: an Axum `Router` with a `POST /agents/{id}/execute` handler over
  shared state holding an `Arc` agent registry + `Arc<PaladinExecutionService>`, calling
  `.execute()` and returning the result as JSON. The `axum::serve(...)` bind line is shown
  but may be a `# `-hidden / `no_run`-style line so the example compiles without binding a
  port during `cargo check`.
- A **`config.yml`** snippet for host/agent configuration (must validate — FR-12).
- A "see also" link to `paladin-web` for the bundled user/auth routes a real service often
  also needs, and a forward link to the sidecar page.

### Task 6.5 — Queue / Worker (Distributed) Topology Page

#### FR-6: New file `deployment-topologies/queue-worker.md`
- Explains the producer/worker model: enqueue agent jobs, a worker pool dequeues and
  executes — backpressure, retries, scale-out, fault isolation.
- A **compilable** example, feature-gated behind `redis-queue`:
  - Producer: build `RedisQueueConfig`, construct `RedisQueueAdapter::new(...)`, enqueue a
    job payload via the queue port.
  - Worker: dequeue, deserialize the job, run a `PaladinExecutionService`, mark complete.
  - Wrap the runtime-requiring parts so the example **compiles** (`cargo check`) without a
    live Redis — use `no_run`-style hiding for the `.await`ed network calls if needed; the
    types and signatures must still be exercised so the example is real.
- A **`config.yml`** snippet for the Redis queue (validate — FR-12).
- A `> **Prerequisites:** Run \`make dev\` (Redis) first; enable the \`redis-queue\` feature.`
  callout.
- **Cross-link** to `appendix/redis-queue-adapter-setup.md` for infra setup.

### Task 6.6 — Sidecar (Separate Process) Topology Page

#### FR-7: New file `deployment-topologies/sidecar.md`
- Explains the model: an agent hosted in its own process, called by your main app over the
  network; when process/security/deploy isolation justifies the operational overhead (and
  when the HTTP host in one process is the better default).
- **Honesty callout (FR mandate #2):** state plainly that Paladin ships no IPC/gRPC/RPC or
  sidecar transport; the pattern is composed from the HTTP host (server side) + a plain HTTP
  client (caller side), and the wire contract is consumer-owned.
- A **compilable** example for the caller side: a minimal HTTP client call (e.g. `reqwest`)
  to the `POST /agents/{id}/execute` endpoint defined on the HTTP-host page, with
  request/response structs. The server side is a cross-link to FR-5 (not re-listed). The
  network call is `no_run`-style so it compiles without a running server.
- A short "what you would need for a first-class sidecar" subsection (a transport port
  trait + serialization) framed as a documented limitation / future direction, linked to
  OQ-3.

### Cross-cutting Requirements

#### FR-8: Decision matrix is the single source of routing
The comparison table + flowchart on the landing page (FR-2) is the canonical "which
topology" reference; each topology page links **back** to it and **across** to its
neighbors so a reader can navigate without leaving the section.

#### FR-9: Compilable Rust examples (hard gate)
All new fenced ```rust blocks must pass `scripts/check-doc-examples.sh`
(globs `docs/src/**/*.md`; wired into the pre-push hook and `docs.yml`). Examples follow the
Epic 3 conventions: `# `-hidden boilerplate imports, `# #[allow(unused)]` where needed,
`no_run`-style hiding for `.await`ed network/bind calls that must compile but not run.

#### FR-10: Doc-example harness dependencies
The compilable HTTP-host, queue/worker, and sidecar examples require crates the
doc-examples test harness may not currently pull in (`axum`, `reqwest`, `tokio`,
`paladin-web`, and the `redis-queue` feature on the relevant crate). The author must add the
needed dev-dependencies / feature activation to the doc-examples harness manifest (see §7)
so `check-doc-examples` can compile them, **without** adding runtime deps to any shipped
crate. If a dependency cannot be added cleanly, the affected block is documented and the
limitation recorded as an open question — examples are not silently downgraded to
non-compiling.

#### FR-11: SUMMARY.md registration
All six new pages are linked under the new section (FR-1) so `mdbook build` includes them
with no "file not in SUMMARY" warnings.

#### FR-12: Config-snippet validation
Every fenced ```yaml / `config.yml` snippet on the new pages validates against the current
schema via the Epic 4 config-check gate (`scripts/check-doc-config.sh` / `make
check-doc-config`). New files are picked up automatically by the existing glob.

#### FR-13: Zero-warning build
After all content is added, `mdbook build` completes with zero warnings, `[output.linkcheck]`
(`warning-policy = "error"`) enforcing — every internal cross-link from the new pages
resolves.

#### FR-14: CHANGELOG entry
Add a `[Unreleased]` entry to `CHANGELOG.md` recording the new Deployment Topologies section
(docs scope). Do **not** regenerate `project/current-exports.txt` (no API change).

---

## 5. Non-Goals (Out of Scope)

- **Rust source-code / API changes** — This Epic modifies only `docs/src/` markdown,
  `docs/src/SUMMARY.md`, the doc-examples test harness manifest (dev-deps/features for
  FR-10), and check tooling if needed. **No `*.rs` library changes, no new endpoints, no
  gRPC/RPC layer, no sidecar abstraction.** Missing capabilities are documented and flagged,
  not implemented.
- **A first-class agent-HTTP endpoint or sidecar transport** — explicitly out of scope;
  documented as consumer-composed and recorded as open questions (OQ-2, OQ-3).
- **Rewriting existing pages** — `paladin-agents.md`, `orchestration.md`,
  `battalion-patterns.md`, `deployment/*`, and `appendix/redis-queue-adapter-setup.md` are
  untouched except for **new inbound cross-links** where natural.
- **Packaging/deployment ops** — Docker/K8s/CI-CD remain in the existing `Deployment`
  section; this section is about *runtime topology*, and links to packaging rather than
  duplicating it.
- **Publishing / GitHub Pages** — handled by Epic 5's pipeline; this Epic only ensures a
  clean local + CI `mdbook build`.
- **Public-API baseline regeneration** (`current-exports.txt`) — docs-only Epic.
- **External link validation** — `follow-web-links = false`; external URLs are not verified.

---

## 6. Design Considerations

### Suggested authoring order
1. **FR-2 (landing/decision matrix)** — establishes the five-topology vocabulary every other
   page references.
2. **FR-3 (embedded library)** and **FR-4 (Battalion)** — short, mostly cross-links;
   establish the in-process baseline.
3. **FR-5 (HTTP host)** — defines the `POST /agents/{id}/execute` contract the sidecar reuses.
4. **FR-6 (queue/worker)**.
5. **FR-7 (sidecar)** — depends on the HTTP-host contract.
6. **FR-9–FR-13 (gates + SUMMARY + build)** verified continuously, with a final full
   `mdbook build` + `make check-doc-examples` + `make check-doc-config` pass.

### Code-example conventions (inherited from Epics 3–4)
- Prefer `MockLlmAdapter` for agent examples so blocks compile and need no key/network.
- Use `# `-hidden lines for boilerplate `use` statements and for `.await`ed bind/network
  calls (`axum::serve`, `reqwest::...send().await`, Redis enqueue/dequeue) so the example
  *compiles* under `cargo check` without running a server or service.
- Job-queue and host examples carry a `> **Prerequisites:**` callout naming `make dev` and
  the required feature flag (`redis-queue`).

### Mermaid diagrams
- `flowchart TD` for the topology decision flowchart (FR-2).
- Optional `sequenceDiagram` on the HTTP-host and sidecar pages to show request flow
  (client → handler → execution service → agent). All must render under mdbook-mermaid.

### Cross-linking map
- Landing ↔ each topology page (bidirectional).
- `embedded-library.md` → `user-guides/paladin-agents.md`.
- `battalion-orchestration.md` → `user-guides/orchestration.md`,
  `user-guides/battalion-patterns.md`.
- `http-service-host.md` → `api-reference/crate-map.md` (paladin-web), → `sidecar.md`.
- `queue-worker.md` → `appendix/redis-queue-adapter-setup.md`.
- `sidecar.md` → `http-service-host.md`.

---

## 7. Technical Considerations

### Verified source anchors for the new pages
*(Re-verify against source before publishing — naming mandate, §4.)*

- **Embedded library** — `src/lib.rs`, `src/prelude.rs` re-export `PaladinBuilder`,
  `Paladin`, `PaladinData`, `PaladinStatus`, `PaladinConfig`, `LlmPort`.
  - `PaladinBuilder::new(llm_port: Arc<dyn LlmPort>) -> Self`; `async fn build(self) ->
    Result<Paladin, PaladinError>`.
  - `PaladinExecutionService::new(llm_port: Arc<dyn LlmPort>, circuit_breaker:
    Arc<CircuitBreaker>, garrison: Option<Arc<dyn GarrisonPort>>, arsenal: Option<Arc<dyn
    ArsenalPort>>) -> Self`; `async fn execute(&self, paladin: &Paladin, input: &str) ->
    Result<PaladinResult, PaladinError>` (path:
    `crate::application::services::paladin::paladin_execution_service`).
  - `MockLlmAdapter::new() -> Self` (always available); `OpenAIAdapter::new(OpenAIConfig) ->
    Result<Self, String>`, `AnthropicAdapter::new(AnthropicConfig) -> Result<Self,
    LlmError>` (feature-gated).
- **Battalion** — `crates/paladin-battalion/src/`: `FormationExecutionService`,
  `PhalanxExecutionService`, `CampaignExecutionService`, `ChainOfCommandExecutionService`,
  `Commander` / `CommanderBuilder`, each `::new(paladin_port: Arc<dyn PaladinPort>) -> Self`
  with `async fn execute(...)`. Config: `BattalionConfig`, `ErrorStrategy`
  (`FailFast`/`ContinueOnError`/`RetryThenContinue`), `AggregationStrategy`
  (`CollectAll`/`FirstSuccess`/`Majority`/`Custom`), `BattalionResult`, `BattalionError`.
- **HTTP host** — `crates/paladin-web/src/app.rs`:
  `create_app_router(user_service: Arc<dyn UserServiceTrait>, auth_port: Arc<dyn AuthPort>)
  -> axum::Router`. **This is user-management only.** Axum `0.8.x`; no `serve` helper is
  provided — the consumer calls `tokio::net::TcpListener::bind` + `axum::serve`. Feature flag
  gating the crate: `web-server`. The agent endpoint in FR-5 is **consumer-authored**.
- **Queue/worker** — `crates/paladin-storage/src/redis.rs`:
  `RedisQueueAdapter::new(config: RedisQueueConfig, log_port: Option<Arc<dyn LogPort>>) ->
  Result<Self, QueueError>`. `RedisQueueConfig { redis_host, redis_port, redis_password,
  redis_db, connection_timeout, key_prefix, max_retries }` (has `Default`). Ports
  (`paladin_ports::output::queue_port`): `QueuePort`, `BatchQueuePort`, `PriorityQueuePort`,
  `QueueManagementPort`, `FullQueuePort`; `QueueItem<T>`, `QueueStats`, `QueueError`. Feature
  flag: `redis-queue`.
- **Sidecar** — **No IPC/gRPC/tonic/RPC client exists in the workspace** (confirmed by
  search). The compilable caller-side example uses a generic HTTP client (`reqwest`) against
  the FR-5 contract; document the absence as a limitation (OQ-3).

### Doc-examples harness (FR-10)
- `scripts/check-doc-examples.sh` extracts and `cargo check`s every ```rust block under
  `docs/src/**/*.md`; new files are covered automatically.
- The harness crate (the doc-examples test crate, e.g. `crates/doc-examples` /
  `paladin-doc-examples`) must gain dev-dependencies `axum`, `reqwest`, `tokio` and activate
  the `redis-queue` feature (and depend on `paladin-web`) so the host/worker/sidecar blocks
  compile. **Confirm the exact harness manifest path and how it enables features before
  editing.** Do not add these as runtime deps to any shipped library crate.

### Config-snippet validation (FR-12)
- `scripts/check-doc-config.sh` / `make check-doc-config` parses fenced YAML through the
  framework config loader; new files are globbed automatically. Confirm the config struct
  the loader deserializes into so host/queue snippets use real keys.

### Dependencies
- Epic 2 (MDBook), Epic 3 (content rewrite + code/link gates), and Epic 4 (new docs +
  config-check gate) are merged to `main`. All documented APIs exist in the current
  workspace.

---

## 8. Success Metrics

| Metric | Target |
|---|---|
| New pages delivered | 6 (landing + 5 topologies) |
| Topologies with a compilable example | 5 / 5 (sidecar via composed HTTP client) |
| Decision matrix (table + flowchart) on landing page | 1 |
| Honesty callouts present (HTTP-host no-endpoint; sidecar no-IPC) | 2 / 2 |
| Fenced Rust blocks failing `cargo check` | 0 |
| Fenced YAML/config snippets failing schema validation | 0 |
| `mdbook build` warnings (linkcheck enforcing) | 0 |
| New pages registered in `SUMMARY.md` | 6 / 6 |
| CHANGELOG `[Unreleased]` entry added | 1 |
| `current-exports.txt` changes | 0 (docs-only) |

---

## 9. Open Questions

| ID | Question | Resolution |
|---|---|---|
| OQ-1 | This overlaps Epic 4 ("New Documentation"). Fold in or stand alone? | **Stand-alone Epic 6**, a new top-level *Deployment Topologies* section. Epic 4 documented *subsystems*; this documents *runtime topology* as a decision. (Maintainer decision.) |
| OQ-2 | The HTTP host has no shipped agent-execution endpoint — document composition or request a feature? | **Document the consumer-composed Axum handler around `PaladinExecutionService`** and flag the absence. A first-class endpoint is a future framework decision, not this Epic. |
| OQ-3 | The sidecar topology has no IPC/RPC support in the workspace — can it have a compilable example? | **Yes, via composition:** server side = HTTP host (FR-5), caller side = a compilable `reqwest` client (`no_run`). State plainly no first-class sidecar transport ships; record "transport port + serialization" as a documented future direction. |
| OQ-4 | Can the doc-examples harness gain `axum`/`reqwest`/`redis-queue` deps without polluting shipped crates? | **To be confirmed by the author** — add them as harness dev-deps/feature activation only. If a block cannot be made to compile cleanly, document it and log the limitation rather than shipping a non-compiling example. |
| OQ-5 | Landing page filename — `overview.md`, `index.md`, or `README.md`? | **To be confirmed against the book's existing section-landing convention** (match how other sections name their parent page) during scaffold. |
| OQ-6 | Exact doc-examples harness manifest path and config-loader struct for FR-10/FR-12? | **To be confirmed by the author** when wiring the gates; reuse the same entry points Epics 3–4 established. |

---

## Relevant Files

### New Files (documentation)
- `docs/src/deployment-topologies/overview.md` — landing / decision matrix (FR-2)
- `docs/src/deployment-topologies/embedded-library.md` — single-process topology (FR-3)
- `docs/src/deployment-topologies/battalion-orchestration.md` — many-agents-one-runtime (FR-4)
- `docs/src/deployment-topologies/http-service-host.md` — Axum agent host (FR-5)
- `docs/src/deployment-topologies/queue-worker.md` — Redis queue/worker (FR-6)
- `docs/src/deployment-topologies/sidecar.md` — separate-process sidecar (FR-7)

### Files Updated (In-Place)
- `docs/src/SUMMARY.md` — new top-level Deployment Topologies section + six entries (FR-1, FR-11)
- `CHANGELOG.md` — `[Unreleased]` entry for the new section (FR-14)
- `docs/src/user-guides/paladin-agents.md` — inbound cross-link from embedded-library page (optional)
- `docs/src/user-guides/orchestration.md` — inbound cross-link from Battalion page (optional)
- `docs/src/appendix/redis-queue-adapter-setup.md` — inbound cross-link from queue/worker page (optional)
- `docs/src/architecture/overview.md` — optional pointer to the new section

### Tooling (only if required for FR-10)
- doc-examples harness manifest (e.g. `crates/doc-examples/Cargo.toml` /
  `paladin-doc-examples`) — add `axum`, `reqwest`, `tokio` dev-deps + `redis-queue` feature
  + `paladin-web` dep so host/worker/sidecar blocks compile. **Confirm exact path first.**
