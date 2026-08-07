# PRD: Deployment Artifacts, Examples & Documentation (Milestone 12, Epic 7)

> **Correction (dated 2026-08-06, DEBT-01):** This document instructs a future implementer to
> write/regenerate the public-API surface baseline at the pre-rename `project/` path in two places
> (Success Metric 6 and §4.6 FR-13, both struck below) — a path that has not existed since commit
> `928c6d5` renamed `project/` to `.project/`. The baseline lives at `.project/current-exports.txt`,
> confirmed present at 442,369 bytes via `ls -la .project/current-exports.txt`, re-run during this
> task; the pre-rename path is confirmed absent via `ls` on it, which returns "No such file or
> directory", also re-run during this task. This document was created 2026-06-09,
> months after commit `928c6d5` renamed the directory, so the defect it names propagates forward
> rather than decaying. This is one of five requirement documents Phase 8 / DEBT-01 corrects on
> the requirement-text side; the corresponding tooling (`scripts/check-api-surface.sh`,
> `scripts/extract-public-api.sh`, `.github/workflows/ci.yml`) was corrected separately in plan
> 08-02. Original text is retained below with inline corrections — nothing is deleted.

**Project:** Paladin Framework
**Milestone:** 12 — Web API / HTTP Service Host Topology, Out of the Box
**Epic:** 7 — Deployment Artifacts, Examples & Documentation (final epic)
**Version Target:** **v0.6.0** (this epic performs the bump)
**Status:** Ready for Implementation
**Created:** 2026-06-09
**Author:** AI Coding Agent (Claude Code)
**Depends on:** Milestone 12 Epics 1–6 (runnable `paladin-server` with a stable, documented,
secured, versioned API surface)

---

## 1. Introduction / Overview

Epics 1–6 built and hardened the `paladin-server` HTTP API — execution, streaming, async jobs,
discovery, runtime registration, health/readiness, a unified error envelope, authentication &
authorization, and a versioned (`/v1`) OpenAPI-documented surface. **What's missing is everything a
user needs to actually run and adopt it:** a container image, orchestration manifests, accurate
deployment docs, a worked example, end-to-end tests, and the **v0.6.0** version bump that releases
the milestone.

**This Epic makes the shipped server deployable and discoverable** and closes Milestone 12.

### Scope decisions (from PRD clarification)

- **Version bump:** bump **all workspace crates to `0.6.0`** (and their inter-crate path-dep
  versions) in one coordinated move.
- **Container base:** mirror the existing CLI image — `rust`-slim builder → **`debian:12-slim`**
  runtime (glibc + CA certs, proven for the reqwest/TLS LLM calls).
- **Artifacts:** **dedicated** server artifacts (`Dockerfile.server`, a compose service, `k8s/`
  server manifests) — the existing CLI `Dockerfile`/k8s assets are left untouched.
- **E2E tests:** **in-process** `axum::serve` on an ephemeral port (like the smoke test), driven with
  `reqwest` — fast and hermetic.

---

## 2. Goals

1. A `Dockerfile.server` builds a minimal `debian:12-slim` image that runs `paladin-server` and
   responds on `/health`.
2. A docker-compose service and `k8s/` Deployment+Service run the server with config + secrets
   injected and liveness/readiness probes pointing at `/health` / `/ready`.
3. The deployment-topology docs accurately describe the **shipped** API + server binary (replacing
   the old "compose your own endpoint" framing), and the topology overview table is updated.
4. A runnable `examples/` program (and a compile-tested `doc-examples` snippet) boots the server from
   a sample config and exercises an agent.
5. An end-to-end test suite boots the real assembled server and asserts auth, buffered + streaming
   execution, async jobs, health/readiness, the error envelope, and the served spec/UI.
6. ~~All workspace crates are at **0.6.0**; `CHANGELOG.md` and `project/current-exports.txt` reflect the
   release.~~
   **Corrected (dated 2026-08-06, DEBT-01):** The correct baseline path is
   `.project/current-exports.txt`, not the pre-rename path struck above — the directory was
   renamed by commit `928c6d5`. Confirmed via `ls -la .project/current-exports.txt` (442,369 bytes
   present) and `ls` on the struck path ("No such file or directory"), both re-run during this
   task. The version and `CHANGELOG.md` clauses are unaffected.
7. New code compiles warning-free; `cargo test`, `fmt`, `clippy -D warnings`, `make deny`/`make audit`
   pass; the container image builds.

---

## 3. User Stories

- **As an operator**, I want a container image and k8s manifests so I can deploy `paladin-server`
  with health/readiness probes wired, without assembling them myself.
- **As a developer evaluating Paladin**, I want a `docker compose up` (or a runnable example) that
  starts the API and shows an agent call end-to-end.
- **As an integrator**, I want deployment docs that match the shipped binary and routes (`/v1`,
  `/health`, `/openapi.json`, `/docs`, auth), not an outdated "build it yourself" page.
- **As a maintainer**, I want an e2e suite that exercises the whole server so regressions in wiring
  (auth, layers, streaming, docs) are caught.
- **As a release manager**, I want the workspace coherently bumped to v0.6.0 with an accurate
  CHANGELOG and API baseline.

---

## 4. Functional Requirements

### 4.1 Container image & compose

1. A **`Dockerfile.server`** (multi-stage) **must** build `paladin-server` in a `rust`-slim builder
   (`cargo build --release --bin paladin-server --features web-server`) and run it from a
   `debian:12-slim` runtime with `ca-certificates` installed, as a **non-root** user.
2. The image **must** `EXPOSE` the HTTP port, set a sensible `ENTRYPOINT`/`CMD`, and read its config
   via `PALADIN_CONFIG` (with secrets — LLM + API keys — supplied as environment variables, never
   baked in). It **should** declare a `HEALTHCHECK` against `/health`.
3. A **docker-compose service** **must** run the server image, mounting an example `config.yml` and
   sourcing secrets from the environment, suitable for `docker compose up`.

### 4.2 Kubernetes manifests

4. A `k8s/` **Deployment** + **Service** for `paladin-server` **must** exist, consistent with the
   existing `k8s/` assets (namespace/labels), with **liveness** (`/health`) and **readiness**
   (`/ready`) probes and the HTTP port exposed.
5. Config **must** come from a **ConfigMap** (the `config.yml`) and secrets from a **Secret**
   (LLM/API keys), referenced by the Deployment — mirroring the existing secret/configmap pattern.

### 4.3 Documentation

6. [`deployment-topologies/http-service-host.md`](../../../docs/src/deployment-topologies/http-service-host.md)
   **must** be rewritten to document the shipped API and `paladin-server`: the routes (`/v1/agents…`,
   `/health`, `/ready`, `/openapi.json`, `/docs`), auth posture, config (`http`, `agents`), timeouts,
   and how to run it (binary, Docker, k8s) — replacing the "Paladin ships no agent-execution
   endpoint / compose your own" framing.
7. The topology [overview](../../../docs/src/deployment-topologies/overview.md) table **must** be
   updated so the HTTP service-host row reflects "ships out of the box".

### 4.4 Example & doc-example

8. A runnable **`examples/http_service_host.rs`** **must** boot the server (in-process) from a sample
   config / state and exercise an agent (buffered, and ideally a streamed call), hermetic where
   possible (mock LLM) or clearly documenting any required key.
9. A compile-tested **`doc-examples/src/http_service_host.rs`** snippet **must** show the minimal
   "build state → serve / call" path used in the docs.

### 4.5 End-to-end integration tests

10. An **in-process** e2e suite **must** boot the fully-assembled app (auth enabled + docs +
    `with_http_layers`) on an ephemeral port and assert, over real HTTP:
    - auth: `401` without a credential, `200` with a valid `X-API-Key`, `403` for a disallowed role /
      non-admin register;
    - execution: buffered `execute` (`200` + output), streaming (`chunk`…`done` SSE), and async jobs
      (`202` → poll → `completed`);
    - ops: `/health` + `/ready`, the nested error envelope on a bad request, and `/openapi.json` +
      `/docs` reachable.
11. The suite **must** be hermetic (mock LLM, no network/keys) so it runs in normal CI.

### 4.6 Version bump & finalize

12. **All workspace crates** **must** be bumped to **`0.6.0`**, including inter-crate path-dependency
    `version = "0.5.1"` pins updated to `0.6.0`.
13. ~~`CHANGELOG.md` **must** gain a finalized Milestone 12 release summary (and/or a `[0.6.0]` section)
    and `project/current-exports.txt` **must** be regenerated.~~
    **Corrected (dated 2026-08-06, DEBT-01):** The correct baseline path is
    `.project/current-exports.txt`, not the pre-rename path struck above — the directory was
    renamed by commit `928c6d5`. Confirmed via `ls -la .project/current-exports.txt` (442,369
    bytes present) and `ls` on the struck path ("No such file or directory"), both re-run during
    this task. The `CHANGELOG.md` clause is unaffected.
14. The full gate (`cargo test`/`--features web-server`, `fmt`, `clippy`, `make deny`, `make audit`)
    **must** pass and the server image **must** build.

---

## 5. Non-Goals (Out of Scope)

- **Publishing** images to a registry or **crates.io** release (CI/CD/release automation is its own
  milestone) — this epic produces the artifacts, not the publish.
- **Helm charts / Kustomize overlays** — plain manifests only.
- **TLS termination / ingress** — a proxy/ingress concern; documented as out-of-scope, not implemented.
- **Multi-arch / static-musl images** — single `debian:12-slim` amd64 image (multi-arch deferred).
- **The spawn-the-binary e2e mode** — in-process only this epic.
- **New API features** — Epic 7 is artifacts/docs/tests/release only; no behavior changes to the API.

---

## 6. Design Considerations

### Dockerfile.server (sketch)

```dockerfile
FROM rust:1-slim-bookworm AS builder
WORKDIR /app
COPY . .
RUN cargo build --release --bin paladin-server --features web-server

FROM debian:12-slim
RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates \
    && rm -rf /var/lib/apt/lists/* && useradd -r -u 10001 paladin
COPY --from=builder /app/target/release/paladin-server /usr/local/bin/paladin-server
COPY config.example.yml /etc/paladin/config.yml
USER paladin
EXPOSE 8080
ENV PALADIN_CONFIG=/etc/paladin/config.yml
HEALTHCHECK CMD ["/usr/local/bin/paladin-server", "--health"]  # or a curl-less probe
ENTRYPOINT ["/usr/local/bin/paladin-server"]
```

### k8s probes (sketch)

```yaml
livenessProbe:  { httpGet: { path: /health, port: http }, initialDelaySeconds: 5 }
readinessProbe: { httpGet: { path: /ready,  port: http }, initialDelaySeconds: 5 }
```

### Sample config

Reuse `config.example.yml`; for the hermetic example/e2e use auth + a known key and a mock LLM so no
real provider key is required.

---

## 7. Technical Considerations

- **No new crate code** is strictly required; the e2e suite and example live in `tests/` and
  `examples/`, reusing the public `paladin-web` surface (`AgentApiState`, `agent_router`,
  `with_http_layers`, `openapi::{build_openapi, docs_router}`, `AgentAuthConfig`, `Principal`).
  Hermetic agents use the existing `MockLlmAdapter` (as the smoke test does).
- **Build target:** the image builds `--features web-server` (the feature that compiles
  `paladin-server`). Note the swagger-ui assets are embedded in the binary, so the runtime image
  needs no extra static files — but the binary is larger and links are heavier (watch build
  disk/memory).
- **HEALTHCHECK without curl:** `debian:12-slim` has no curl; either add a tiny `--health` subcommand
  to `paladin-server`, install `curl`, or rely on the k8s `httpGet` probe and omit the Docker
  `HEALTHCHECK`. (See Open Question 2.)
- **Version bump mechanics:** every `crates/*/Cargo.toml` `version` and every inter-crate dependency
  `{ version = "0.5.1", path = … }` → `0.6.0`; the root/workspace package version too. Verify the
  lockfile and that `cargo build` still resolves. This touches many files — do it as one mechanical
  pass and rely on `cargo build` to catch mismatches.
- **Docs build:** if the repo builds `docs/` (mdBook) and `doc-examples` in CI, the new snippet must
  compile there; keep it minimal and hermetic.
- **API surface:** the version bump changes the `current-exports.txt` header/versions; the e2e/example
  add no public items. Regenerate and confirm the diff is the bump (+ any incidental).

---

## 8. Success Metrics

1. `docker build -f Dockerfile.server .` produces an image that, run with a sample config, serves
   `GET /health` → `200`.
2. `docker compose up` (server service) brings up a reachable API; `kubectl apply` of the `k8s/`
   server manifests yields a Deployment whose pods become Ready via the `/ready` probe.
3. The deployment-topology page and overview table describe the shipped API (no "compose your own"
   language remains).
4. `cargo run --example http_service_host` boots the server and prints a successful agent call.
5. The e2e suite passes hermetically in CI, covering auth, buffered/streaming/jobs, health, errors,
   and docs.
6. All crates report `0.6.0`; `CHANGELOG.md` + `current-exports.txt` are updated; `cargo test
   --features web-server`, `fmt`, `clippy --workspace --all-targets`, `make deny`, `make audit` are
   green.

---

## 9. Open Questions

1. **HTTP port default:** standardize on `8080` (matches the existing CLI image `EXPOSE 8080`) for the
   image/compose/k8s and the example config? (Default: yes — `server.port: 8080`.)
2. **Docker `HEALTHCHECK` strategy:** add a lightweight `paladin-server --health` self-check
   subcommand (no curl dependency), or omit the Docker `HEALTHCHECK` and rely on the k8s `httpGet`
   probes? (Default: omit Docker `HEALTHCHECK`, rely on k8s probes; revisit if a self-check is cheap.)
3. **Example hermeticity:** make `examples/http_service_host.rs` fully hermetic with `MockLlmAdapter`
   (no key, always runnable), or wire a real provider behind an env key? (Default: hermetic mock, with
   a comment on swapping in a real provider.)
4. **CHANGELOG shape:** convert `[Unreleased]` into a dated `[0.6.0]` section now, or keep
   `[Unreleased]` until the actual tag/release? (Default: add a `[0.6.0]` heading capturing Milestone
   12; leave tagging to the release process.)
5. **Compose backing services:** does the server compose service need Redis/MinIO (Epics 1–6 agents
   are LLM + prompt only, no garrison/arsenal), or is it standalone? (Default: standalone — no backing
   services; document how to add them.)

---

*Next step: run `/generate-tasks` against this PRD to produce
`tasks-deployment-artifacts-examples-docs.md` in this `Epic_7/` folder.*
