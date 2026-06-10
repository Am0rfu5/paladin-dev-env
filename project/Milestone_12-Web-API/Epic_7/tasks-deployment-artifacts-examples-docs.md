# Tasks: Deployment Artifacts, Examples & Documentation (Milestone 12, Epic 7)

**PRD:** [prd-deployment-artifacts-examples-docs.md](prd-deployment-artifacts-examples-docs.md)
**Scope:** packaging/docs/tests/release — reuses the public `paladin-web` surface (no new API code).
**Base:** `main` (Milestone 12 Epics 1–6 merged — PRs #19, #21, #22, #23, #24, #25).
**Status:** Phase 2 — sub-tasks expanded, ready for implementation

---

## Relevant Files

- `Dockerfile.server` - **New.** Multi-stage build of `paladin-server` (`--features web-server`) on a `debian:12-slim` runtime (non-root, CA certs, `EXPOSE 8080`).
- `docker/docker-compose.server.yml` (or a service in an existing compose) - **New.** Run the server image with a mounted `config.yml` and env-sourced secrets.
- `k8s/server/deployment.yaml`, `k8s/server/service.yaml`, `k8s/server/configmap.yaml` - **New.** Server Deployment+Service+ConfigMap with `/health` (liveness) and `/ready` (readiness) probes; reuse the existing `paladin` namespace/labels + Secret pattern.
- `docs/src/deployment-topologies/http-service-host.md` - **Modify.** Rewrite to document the shipped API + `paladin-server` (routes, auth, config, Docker/k8s run).
- `docs/src/deployment-topologies/overview.md` - **Modify.** Update the topology table row.
- `examples/http_service_host.rs` - **New.** Runnable: boot the server in-process from sample state and call an agent (hermetic via `MockLlmAdapter`). Wire an `[[example]]` entry (`required-features = ["web-server"]`) in the root `Cargo.toml`.
- `doc-examples/src/http_service_host.rs` (+ `doc-examples/Cargo.toml`/`lib.rs` wiring) - **New.** Compile-tested minimal "build state → serve" snippet for the docs.
- `tests/web_server_e2e.rs` - **New.** In-process end-to-end suite (auth, buffered/streaming/jobs, health, errors, spec/UI), hermetic.
- `crates/*/Cargo.toml`, root `Cargo.toml` - **Modify.** Bump all workspace crates + inter-crate path-dep pins `0.5.1 → 0.6.0`.
- `config.example.yml` - **Modify (maybe).** Ensure `server.port: 8080` and an auth/docs setup the example/compose can use.
- `CHANGELOG.md` / `project/current-exports.txt` - **Modify.** `[0.6.0]` Milestone 12 summary; regenerate the API baseline.
- `Makefile` / `README.md` - **Modify (maybe).** A `docker-build-server` convenience target; README run/deploy pointers.

### Notes

- **No new crate/library code** is expected — the example and e2e suite consume the existing public
  `paladin-web` API (`AgentApiState`, `agent_router`, `with_http_layers`, `openapi::{build_openapi,
  docs_router}`, `AgentAuthConfig`, `Principal`) and `MockLlmAdapter`.
- Run with `cargo test --features web-server`. Before committing a parent task: `cargo test` →
  `cargo fmt --check` → `cargo clippy -- -D warnings` → `make deny`/`make audit`.
- **Hermetic:** the example and e2e tests use `MockLlmAdapter` — no network, no provider keys — so
  they run in normal CI.
- **debian:12-slim has no curl** — rely on the k8s `httpGet` probes for health; omit a Docker
  `HEALTHCHECK` (or add a cheap `--health` self-check) per PRD Open Q2.
- **Version bump touches many files** — do it as one mechanical pass and let `cargo build` catch any
  missed inter-crate pin.
- **Out of scope:** registry/crates.io publish, Helm, TLS/ingress, multi-arch/musl, the
  spawn-the-binary e2e mode, any API behavior change.

## Tasks

- [ ] 0.0 Create feature branch
  - [ ] 0.1 Update `main` (Epics 1–6 merged) and create/checkout `feature/m12-epic7-deployment-artifacts` from it.
  - [ ] 0.2 Confirm a clean baseline: `cargo build --features web-server` and `cargo test --features web-server` pass before changes.

- [ ] 1.0 Container image + docker-compose service for `paladin-server`
  - [ ] 1.1 Write `Dockerfile.server`: multi-stage — `rust`-slim builder running `cargo build --release --bin paladin-server --features web-server`, then a `debian:12-slim` runtime with `ca-certificates`, a non-root user, the binary at `/usr/local/bin/paladin-server`, a baked default `config.yml` (from `config.example.yml`), `ENV PALADIN_CONFIG`, `EXPOSE 8080`, and an `ENTRYPOINT`.
  - [ ] 1.2 Ensure `config.example.yml` uses `server.port: 8080` and an example-friendly auth/docs setup (so the image is reachable out of the box; secrets come from env).
  - [ ] 1.3 Add `docker/docker-compose.server.yml` with a `paladin-server` service (build from `Dockerfile.server`, port mapping, mounted/inline `config.yml`, env-sourced `OPENAI_API_KEY`/`PALADIN_API_KEY_*`); standalone (no Redis/MinIO — see PRD Open Q5).
  - [ ] 1.4 Add a `make docker-build-server` target (and help entry). Build the image locally (`docker build -f Dockerfile.server`) and verify `GET /health` → `200` from a run container.
  - [ ] 1.5 Document the build/run in the relevant docs (cross-ref task 3.0).

- [ ] 2.0 Kubernetes Deployment/Service/ConfigMap with `/health` + `/ready` probes
  - [ ] 2.1 Add `k8s/server/deployment.yaml`: `paladin-server` Deployment in the `paladin` namespace (existing labels), container port `8080`, **liveness** `httpGet /health` + **readiness** `httpGet /ready`, config from a ConfigMap mount + secrets from a Secret (LLM/API keys), resource requests/limits, non-root securityContext.
  - [ ] 2.2 Add `k8s/server/service.yaml` (ClusterIP exposing `8080`) and `k8s/server/configmap.yaml` (the `config.yml`); reuse the existing `k8s/secret.yaml.example` pattern (document the keys needed).
  - [ ] 2.3 Validate the manifests (`kubectl apply --dry-run=client -f k8s/server/` or `kubeconform` if available); update `k8s/README.md` to list the server manifests.

- [ ] 3.0 Rewrite the HTTP service-host deployment docs + topology overview table
  - [ ] 3.1 Rewrite `docs/src/deployment-topologies/http-service-host.md`: the shipped `paladin-server`, routes (`/v1/agents…`, `/health`, `/ready`, `/openapi.json`, `/docs`), auth posture + per-agent `allowed_roles` + admin gate, the `http`/`agents`/`timeouts` config, and how to run it (binary, Docker, k8s) — removing the "Paladin ships no agent-execution endpoint / compose your own" framing.
  - [ ] 3.2 Update the `docs/src/deployment-topologies/overview.md` table row for the HTTP service host to "ships out of the box".
  - [ ] 3.3 If the docs build in CI (mdBook), confirm the pages render and internal links resolve.

- [ ] 4.0 Runnable `examples/` program + compile-tested `doc-examples` snippet
  - [ ] 4.1 Add `examples/http_service_host.rs`: build an `AgentApiState` with a `MockLlmAdapter`-backed agent, an enabled `AgentAuthConfig` (a sample key), and docs; serve in-process on an ephemeral port, then call `/v1/agents/{id}/execute` (and a streamed call) with `reqwest`, printing the result; shut down cleanly.
  - [ ] 4.2 Wire an `[[example]]` entry for it in the root `Cargo.toml` with `required-features = ["web-server"]`.
  - [ ] 4.3 Add `doc-examples/src/http_service_host.rs` (+ module wiring): a minimal compile-tested "build state → `agent_router` / `with_http_layers` → serve" snippet used by the docs.
  - [ ] 4.4 `cargo run --example http_service_host --features web-server` succeeds and prints a successful agent call; the doc-example compiles.

- [ ] 5.0 In-process end-to-end integration test suite (hermetic)
  - [ ] 5.1 **(Test first)** Add `tests/web_server_e2e.rs` (`#![cfg(feature = "web-server")]`): a helper that assembles the full app exactly as the binary does — `agent_router(state)` + `docs_router(build_openapi(state))` + `with_http_layers(...)`, with auth enabled (sample admin + user keys) — and serves it on an ephemeral port.
  - [ ] 5.2 Auth assertions: no credential → `401`; valid `X-API-Key` → `200`; non-admin `POST /v1/agents` → `403`; disallowed role on a restricted agent → `403`.
  - [ ] 5.3 Execution assertions: buffered `execute` → `200` + output; streaming → `chunk`…`done` SSE; async job → `202` → poll → `completed`.
  - [ ] 5.4 Ops assertions: `/health` + `/ready` → `200` (no credential); a malformed body → `400` nested error envelope; `/openapi.json` (paths + security schemes) + `/docs/` → `200`.
  - [ ] 5.5 Ensure hermetic (mock LLM, no network/keys); fmt/clippy; gates.

- [ ] 6.0 Bump all workspace crates to `0.6.0` (+ inter-crate path-dep pins)
  - [ ] 6.1 Bump every `crates/*/Cargo.toml` `version` and the root/workspace package version `0.5.1 → 0.6.0`.
  - [ ] 6.2 Update every inter-crate dependency pin `{ version = "0.5.1", path = … }` → `0.6.0` (and `package = "paladin-ai-core"` style entries).
  - [ ] 6.3 `cargo build --workspace --features web-server` resolves cleanly; refresh `Cargo.lock`; fix any missed pin.
  - [ ] 6.4 Update any version strings in docs/manifests that should track the release (e.g. k8s labels), if appropriate.

- [ ] 7.0 Finalize: CHANGELOG `[0.6.0]`, regenerate API baseline, full gate + image build
  - [ ] 7.1 Add a `CHANGELOG.md [0.6.0]` section summarizing Milestone 12 (Epics 1–7); keep `[Unreleased]` empty or pointing forward (tagging left to the release process).
  - [ ] 7.2 Regenerate `project/current-exports.txt` (version header bump; no new public items expected) and review the diff.
  - [ ] 7.3 Full gate: `cargo test --features web-server`, `cargo fmt --check`, `cargo clippy --workspace --all-targets --features web-server -- -D warnings`, `make deny`, `make audit`. Remove debug prints.
  - [ ] 7.4 Confirm `make docker-build-server` builds the image and a run container serves `/health`.
  - [ ] 7.5 Commit referencing Milestone 12 / Epic 7; mark parent tasks complete and **stop for go-ahead**.
