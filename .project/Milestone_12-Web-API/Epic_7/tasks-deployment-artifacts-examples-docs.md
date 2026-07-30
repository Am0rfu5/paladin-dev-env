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

- [x] 0.0 Create feature branch
  - [x] 0.1 Branched `feature/m12-epic7-deployment-artifacts` from `main` (Epics 1–6 merged).
  - [x] 0.2 Baseline confirmed: `cargo build --features web-server` + `cargo test -p paladin-web` (117) green before changes.

- [x] 1.0 Container image + docker-compose service for `paladin-server`
  - [x] 1.1 Wrote `Dockerfile.server`: `rust:1.93-slim-bookworm` builder (`cargo build --release --bin paladin-server --features web-server`, stripped) → `debian:12-slim` runtime (`ca-certificates`/`libssl3`, non-root uid 65532, binary + baked `/etc/paladin/config.yml`, `ENV PALADIN_CONFIG`, `EXPOSE 8080`, `HEALTHCHECK NONE`, entrypoint).
  - [x] 1.2 `config.example.yml` now binds `0.0.0.0:8080`.
  - [x] 1.3 Added `docker/docker-compose.server.yml` — standalone `paladin-server` service (builds `Dockerfile.server`, maps `8080`, env-sourced `OPENAI_API_KEY`/`PALADIN_API_KEY_*`, optional config volume).
  - [x] 1.4 Added `make docker-build-server`. **Docker is unavailable in this dev environment**, so the image build wasn't executed here; the server's `/health` behavior is verified in-process by the smoke + e2e suites (5.0).
  - [x] 1.5 Build/run documented in `Dockerfile.server`/compose headers; full deployment docs in task 3.0.

- [x] 2.0 Kubernetes Deployment/Service/ConfigMap with `/health` + `/ready` probes
  - [x] 2.1 Added `k8s/server/deployment.yaml`: `paladin-server` Deployment (namespace `paladin`), container port `8080`, **liveness `/health`** + **readiness `/ready`** probes, config mounted from the ConfigMap, secrets via `secretKeyRef`, requests/limits, non-root + read-only-root securityContext. Standalone (no Redis/MinIO init containers).
  - [x] 2.2 Added `k8s/server/service.yaml` (ClusterIP 80 → `http`/8080), `k8s/server/configmap.yaml` (embedded `config.yml` — auth + docs + a sample agent), and `k8s/server/secret.yaml.example` (provider + API-key keys). Real `secret.yaml` is now gitignored.
  - [x] 2.3 `kubectl`/`kubeconform` unavailable in this env; validated all manifests + the embedded `config.yml` as well-formed YAML (Python). Updated `k8s/README.md` with a `paladin-server` section.

- [x] 3.0 Rewrite the HTTP service-host deployment docs + topology overview table
  - [x] 3.1 Rewrote `http-service-host.md` around the shipped `paladin-server`: a route table (`/v1/agents…`, `/health`, `/ready`, `/openapi.json`, `/docs`), the auth posture (fail-closed, API key + JWT, `allowed_roles`, admin gate), the `config.yml` shape, run instructions (binary / Docker / k8s), and a versioning note — removing the "ships no agent-execution endpoint / compose your own" framing (kept an "Embedding in your own app" section).
  - [x] 3.2 Updated the `overview.md` table row to "**`paladin-server` (ships out of the box)**".
  - [x] 3.3 `mdbook build` succeeds (only pre-existing mermaid/fragment warnings); the page + `{{#include}}` anchor resolve. (The included example is refreshed in task 4.3.)

- [x] 4.0 Runnable `examples/` program + compile-tested `doc-examples` snippet
  - [x] 4.1 Added `examples/http_service_host.rs`: assembles the app like the binary (`/v1` router + `docs_router` + `with_http_layers`, auth on with a sample admin key, `MockLlmAdapter`), serves on an ephemeral port, then lists agents / runs a buffered + a streamed call / reads the OpenAPI title via `reqwest`, printing each; graceful shutdown.
  - [x] 4.2 Wired the `[[example]]` entry in the root `Cargo.toml` (`required-features = ["web-server"]`).
  - [x] 4.3 Rewrote `crates/doc-examples/src/http_service_host.rs` to embed the **shipped** API (`AgentApiState` → `agent_router` → `with_http_layers`), replacing the old "compose your own handler" snippet; added `features = ["web-server"]` to the doc-examples `paladin-ai` dep.
  - [x] 4.4 `cargo run --example http_service_host --features web-server` prints a successful agent call (mock output, 1 stream chunk, OpenAPI title); `cargo check -p paladin-doc-examples` + `mdbook build` succeed. fmt/clippy clean.

- [x] 5.0 In-process end-to-end integration test suite (hermetic)
  - [x] 5.1 Added `tests/web_server_e2e.rs` (`#![cfg(feature = "web-server")]`): a `serve()` helper assembling the app like the binary (`agent_router` + `docs_router(build_openapi(state))` + `with_http_layers`, auth enabled with admin + user keys, an open agent + an admin-only agent) on an ephemeral port.
  - [x] 5.2 `auth_is_enforced`: no credential → `401`; admin key → `200`; non-admin `POST /v1/agents` → `403`; user invoking the admin-only agent → `403` (`forbidden`).
  - [x] 5.3 `execution_buffered_streaming_and_jobs`: buffered `execute` → `200` + output; streaming → `chunk`…`done`; async job → poll → `completed`.
  - [x] 5.4 `ops_health_errors_and_docs`: `/health` + `/ready` → `200` (no credential); unknown agent → `404` nested envelope; `/openapi.json` (paths + `api_key` scheme) + `/docs/` → `200`.
  - [x] 5.5 Hermetic (`MockLlmAdapter`, no network/keys); 3 tests pass; fmt/clippy clean.

- [x] 6.0 Bump all workspace crates to `0.6.0` (+ inter-crate path-dep pins)
  - [x] 6.1 Bumped every crate's `[package].version` and the root `paladin-ai` version `0.5.1 → 0.6.0` (blanket replace across all `Cargo.toml`; verified all `0.5.1` occurrences were Paladin-only, no external deps).
  - [x] 6.2 Updated all inter-crate pins (`version = "0.5.1"` and the exact `=0.5.1` pin in `paladin-ports`) → `0.6.0`.
  - [x] 6.3 `cargo build --workspace --features web-server` resolves cleanly; `Cargo.lock` refreshed to `0.6.0` (caught the `=0.5.1` exact pin on the first build).
  - [x] 6.4 README "Current version" → `0.6.0`; the new k8s server manifests already carry `version: v0.6.0` labels.

- [x] 7.0 Finalize: CHANGELOG `[0.6.0]`, regenerate API baseline, full gate + image build
  - [x] 7.1 Added a `CHANGELOG.md [0.6.0]` section (Milestone 12 summary + an Epic 7 entry); `[Unreleased]` left empty/forward-pointing (tagging left to the release process).
  - [x] 7.2 Regenerated `project/current-exports.txt` — version-header bump only, **no removals** (the example/e2e/doc-example add no public items).
  - [x] 7.3 Full gate green: `cargo test --features web-server` (20 test binaries, 0 failures), `cargo fmt --check`, `cargo clippy --workspace --all-targets --features web-server -- -D warnings`, `make deny`, `make audit`. No debug prints.
  - [x] 7.4 `make docker-build-server` target added; **Docker is unavailable in this environment**, so the image build wasn't executed here — the server's `/health` is verified in-process by the e2e + smoke suites.
  - [x] 7.5 Committed; all parent tasks complete.
