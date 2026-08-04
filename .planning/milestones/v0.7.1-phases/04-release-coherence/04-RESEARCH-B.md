# Phase 4 Research — Part B: CI Gates, Examples & Quickstart

**Researched:** 2026-08-02
**Domain:** GitHub Actions CI configuration, Cargo example/feature matrix, QUICKSTART timing
**Confidence:** HIGH (all repo claims verified live against HEAD `68ba809`; no external-docs claims required)

## Scope

Covers D-10 through D-17 of `.planning/phases/04-release-coherence/04-CONTEXT.md`: the gate-suite
proof, the three missing SC5 CI jobs, the example-build gate, and the QUICKSTART timing
measurement. All findings below are `[VERIFIED: local repo / cargo]` unless marked otherwise — this
research required no package installs and no external documentation lookups, so the
`research-plan`/provider seam was not invoked; every claim traces to a file read or a command run
in this sandbox.

## Q1. The three missing SC5 CI jobs

### (a) Trigger stanza

`ci.yml:3-12` currently:
```yaml
on:
  # push:
  #   branches: [ main, develop, 'feature/**' ]
  pull_request:
    branches: [ main, develop ]
  workflow_dispatch:
```
`[VERIFIED: ci.yml:3-12]` A push to `release/v0.7.0` matches none of `pull_request`'s target
branches and isn't a manual dispatch, so **nothing runs**. Uncommenting the original line is
insufficient per D-14.1 — it lists `main, develop, feature/**`, never `release/**`.

**Recommended replacement** (adds `release/**` without dropping the existing PR-only-avoid-double-run
convention documented in the comment):
```yaml
on:
  push:
    branches: [ main, develop, 'feature/**', 'release/**' ]
  pull_request:
    branches: [ main, develop ]
  workflow_dispatch:
```
This is the smallest change that makes a `release/v0.7.0` push trigger CI. Two sibling workflows
(`integration-tests.yml:3-16`, `feature-flags.yml:3-9`) carry the identical commented-out `push:`
stanza with the same `main, develop, feature/**` list and the same avoid-double-run comment —
**all three files share one convention**; whether to add `release/**` to those two as well is a
discretion call (SC5 only names `ci.yml`; the other two aren't in scope here since PIPE governs
`integration-tests.yml`'s deprecated-actions per the deferred list, and `feature-flags.yml` isn't
named in D-14 at all). Recorded here as a **plan-level open question**, not resolved by this
research.

### (b) Examples-build job

Model this job on `feature-flags.yml`'s `feature-matrix` job (`feature-flags.yml:19-118`)
`[VERIFIED: feature-flags.yml:19-118]`, not on `ci.yml`'s `lint`/`test` job (those don't need a
feature matrix). `feature-flags.yml` already encodes the exact caching pattern (`actions/cache@v4`
for registry/index/build, three separate cache steps, `dtolnay/rust-toolchain@master`,
`actions/checkout@v5` per `ci.yml`'s convention vs. `checkout@v4` in `feature-flags.yml` — note
`ci.yml` itself is already on `checkout@v5`, one version ahead of the other three workflow files;
follow `ci.yml`'s own convention since the new job lands there).

**Required-features audit** `[VERIFIED: Cargo.toml:220-238]` — exactly 4 of the 47 example files are
declared `[[example]]` targets, and every one of them gates on non-default features:

| Example | `required-features` |
|---|---|
| `vision_analysis` | `["vision", "llm-openai"]` |
| `vision_battalion` | `["vision", "llm-openai"]` |
| `document_processing` | `["content-processing"]` |
| `http_service_host` | `["web-server"]` |

`[features] default = ["llm-openai"]` `[VERIFIED: Cargo.toml:259]` — `llm-openai` is on by default;
`vision`, `content-processing`, `web-server` are **not**.

**`cargo build --examples --offline` under default features is NOT sufficient.** Verified live: ran
`cargo build --examples --offline` after deleting the 4 required-features binaries from
`target/debug/examples/` — the bulk `--examples` invocation silently **skips** all 4 (no error, no
warning printed to the tail of stdout; cargo's default behavior for unmet `required-features` on a
bulk target selector is silent omission). Confirmed independently: `cargo build --example
document_processing --offline` (without the feature) hard-errors:
```
error: target `document_processing` in package `paladin-ai` requires the features: `content-processing`
Consider enabling them by passing, e.g., `--features="content-processing"`
```
Same hard error reproduced for `vision_analysis` (needs `vision`, `llm-openai`) and
`http_service_host` (needs `web-server`).

**A feature matrix of (at minimum) 4 build invocations is required** to cover all 47 targets:
1. `cargo build --examples --offline` (default features — covers the 43 auto-discovered examples)
2. `cargo build --example vision_analysis --example vision_battalion --features "vision,llm-openai" --offline`
3. `cargo build --example document_processing --features "content-processing" --offline`
4. `cargo build --example http_service_host --features "web-server" --offline`

All 4 verified to **succeed** in this sandbox `[VERIFIED: cargo build, this session]`:
- (1) default: `Finished ... in 20.29s` (incremental; cold estimate not measured — see D-17 caveat)
- (2) vision: `Finished ... in 19.50s`
- (3) content-processing: pulled in `bollard`, `mockito`, `wiremock`, both `reqwest` versions,
  `pdf-extract`, `sqlx-sqlite`, `rmcp`, `testcontainers*`, plus 6 workspace crates —
  `Finished ... in 1m 03s` (cold path for that feature; longest of the four)
- (4) web-server: `Finished ... in 30.07s`

**Conclusion for D-12/D-13:** "every example target builds" is a **claim the plan can record as
green today**, not a work item — but only if the CI job uses the 4-invocation matrix above.
A single `cargo build --examples` step (no feature flags) would make CI silently under-cover 4 of
47 examples with no failure signal, which is exactly the kind of unproven-gate risk D-15 warns
about for Docker/K8s. **Do not let the plan write a one-line `cargo build --examples` step and
call SC5's example gate closed.**

### (c) Docker budget assertions

`ci.yml`'s `docker` job (`:409-434`) `[VERIFIED]` builds **single-platform** (no `platforms:` key —
defaults to the runner's native arch), asserts **no size or time budget**, and does not push.
Existing action versions in that job: `docker/setup-buildx-action@v4` (`:418`),
`docker/build-push-action@v6` (`:421`).

**Multi-arch config is missing entirely.** `release.yml`'s `build-docker` job (`:160-220`)
`[VERIFIED: release.yml:160-220]` already does near-identical work for the release pipeline and is
the closest existing model in this repo — but note it runs **older pinned action versions**:
`docker/setup-qemu-action@v3`, `docker/setup-buildx-action@v3`, `docker/build-push-action@v5`. Per
the instruction to cite versions already in use rather than invent new ones, and because `ci.yml`
itself is already one step ahead (`buildx-action@v4`, `build-push-action@v6`), the new `ci.yml` job
should stay on `ci.yml`'s own already-used `@v4`/`@v6` pair and add `docker/setup-qemu-action@v3`
(the only piece `ci.yml` doesn't already have — `release.yml` is the only in-repo precedent for its
version, so `@v3` is what's "already used in this repo" for that specific action).

`release.yml:196-220` already contains a **size-check pattern to reuse directly**:
```yaml
      - name: Build and push
        uses: docker/build-push-action@v5
        with:
          context: .
          platforms: linux/amd64,linux/arm64
          push: true
          tags: ${{ steps.meta.outputs.tags }}
          cache-from: type=gha
          cache-to: type=gha,mode=max

      - name: Verify image size
        run: |
          IMAGE=$(echo '${{ steps.meta.outputs.json }}' | jq -r '.tags[0]')
          docker pull "$IMAGE"
          SIZE=$(docker image inspect "$IMAGE" --format='{{.Size}}')
          SIZE_MB=$((SIZE / 1024 / 1024))
          echo "Image size: ${SIZE_MB} MB"
          if [ $SIZE_MB -gt 500 ]; then
            echo "::warning::Image size (${SIZE_MB} MB) exceeds 500 MB target"
          fi
```
Two adaptations needed for the `ci.yml` job (which does not push to a registry, unlike
`release.yml`'s):
1. `push: false` in CI (as it is today) means there's no registry pull to inspect — instead use
   `docker image inspect paladin:test --format='{{.Size}}'` directly against the locally-built
   image, the same tag the job already assigns at `ci.yml:426`.
2. `release.yml`'s check only emits `::warning::` (non-blocking) — SC5's "inside its budget" language
   implies the CI gate should **fail**, not warn. Change `echo "::warning..."` to `exit 1` (or keep
   as `::error::` + `exit 1`) for the new `ci.yml` job specifically; `release.yml`'s existing
   non-blocking warning is out of scope to change (not named in D-14/D-15).

**Time budget has no existing precedent in this repo** — neither `ci.yml` nor `release.yml` times a
Docker build today. The `kubernetes-smoke-test` job in `integration-tests.yml:238-249` (see below)
is the closest in-repo pattern for a wall-clock budget check (epoch-diff, `::warning::` on
overage). Model the Docker time budget the same way: capture `date +%s` before the
`build-push-action` step, capture it again after, diff, and either warn or fail if `> 300` (5 min).
Multi-arch (`linux/amd64,linux/arm64`) via QEMU emulation is markedly slower than native-arch
builds — the plan should budget for this when deciding whether the 5-minute figure includes both
architectures or is measured per-arch (D-14/D-15 don't specify; **flag as an open question for the
planner**, this research found no PROJECT.md/ROADMAP clarification of which).

### (d) kind-based Kubernetes smoke job

`k8s/` contains `[VERIFIED: find k8s -type f]`:
```
k8s/namespace.yaml
k8s/configmap.yaml
k8s/secret.yaml.example
k8s/redis.yaml
k8s/minio.yaml
k8s/deployment.yaml
k8s/service.yaml
k8s/README.md
k8s/server/{configmap,deployment,service}.yaml, k8s/server/secret.yaml.example
```

**`integration-tests.yml` already has a complete, working `kubernetes-smoke-test` job**
(`integration-tests.yml:171-264`) `[VERIFIED]` — this is the setup to reuse, not reinvent, per the
research brief's explicit instruction. It:
1. Spins up kind via `helm/kind-action@v1` (`version: v0.20.0`, `cluster_name: paladin-test`)
2. `kubectl create namespace paladin`
3. Creates a `paladin-secrets` Secret with dummy LLM keys + MinIO creds
4. `kubectl apply -f k8s/configmap.yaml`, `k8s/redis.yaml`, `k8s/minio.yaml`, then `sleep 20`
5. `kubectl wait --for=condition=ready pod -l app=redis -n paladin --timeout=120s` (and same for
   minio)
6. `docker build -t paladin:test .` then `kind load docker-image paladin:test --name paladin-test`
7. `sed 's/paladin:latest/paladin:test/g' k8s/deployment.yaml | kubectl apply -f -`, then
   `k8s/service.yaml`
8. `kubectl wait --for=condition=ready pod -l app=paladin -n paladin --timeout=180s || true`
   (note the `|| true` — this wait does **not** fail the job even on timeout today)
9. **The startup-time budget check already exists** at `:238-249`:
```bash
START_TIME=$(kubectl get pod -l app=paladin -n paladin -o jsonpath='{.items[0].status.startTime}')
READY_TIME=$(kubectl get pod -l app=paladin -n paladin -o jsonpath='{.items[0].status.conditions[?(@.type=="Ready")].lastTransitionTime}')
STARTUP_TIME=$((READY_EPOCH - START_EPOCH))
if [ $STARTUP_TIME -gt 30 ]; then
  echo "::warning::Startup time (${STARTUP_TIME}s) exceeds 30 second target"
fi
```

**Important caveat that changes the plan's job:** `k8s/deployment.yaml:66-68` `[VERIFIED]` currently
runs a **placeholder command**, not the real binary:
```yaml
command: ["/bin/sh"]
args: ["-c", "echo 'Paladin started' && sleep 3600"]  # Placeholder for testing
```
and all three probes (liveness/readiness/startup, `:137-174`) are **commented out** with the note
"needs HTTP server endpoint." This means the existing `kubernetes-smoke-test` job measures
*container scheduling and shell-startup time*, not actual application readiness — the pod becomes
"Ready" the instant the container's `RollingUpdate` default readiness (no probe = ready as soon as
process starts) is satisfied, which for a placeholder `sleep 3600` is nearly instantaneous. **The
< 30s pod-startup budget is trivially satisfied today because there is nothing real being started.**
This is a genuine gap the plan should record: either (a) accept the placeholder-based smoke test as
the SC5 gate as currently scoped (it does prove kind/kubectl orchestration works, which is what
D-14.3 literally asks for), or (b) note that a true readiness-probe-based budget requires
`paladin-web`'s `/health`/`/ready` endpoints to be wired into `k8s/deployment.yaml`'s commented-out
probes first — which is arguably new capability, out of this phase's "no new product capability"
boundary. **Recommend (a)**: reuse the existing job's placeholder shape verbatim inside `ci.yml`
(or invoke/duplicate the `kubernetes-smoke-test` job definition), and record the probe-wiring gap as
a named deferral, not attempt it inside Phase 4.

Since `docker`, `kind`, `kubectl` are all absent in this sandbox `[see already_established #6]`,
this job (like the Docker budget job) can only be **authored and statically validated** here (YAML
syntax, `k8s/*.yaml` file references resolve, action version exists) — never executed. This is
exactly D-15's boundary.

## Q2. Which of the 47 examples build today, and at what cost

**Target count:** `cargo build --examples` produces exactly 47 distinct example binaries when the
right feature flags are supplied across the 4-invocation matrix in Q1(b) — one per `.rs` file in
`examples/`. `[VERIFIED: find examples -name '*.rs' | wc -l` → 47; confirmed all 47 file basenames
appear as `target/debug/examples/<name>` binaries after running the full matrix.]`

**Under default features alone** (`cargo build --examples --offline`, `default = ["llm-openai"]`):
- **43 of 47** compile successfully, silently.
- **4 of 47 are skipped, not failed**: `vision_analysis`, `vision_battalion`,
  `document_processing`, `http_service_host` — all 4 have `required-features` unmet by the default
  set. Cargo's bulk `--examples` selector omits them without any non-zero exit code or visible
  warning in the summary output — **a CI job that runs bare `cargo build --examples` would report
  success while covering only 43/47 targets**, and nothing in the job would flag the gap.

**Feature matrix outcome (this session, `--offline` throughout, per D-17):**
| Invocation | Examples covered | Result | Wall time (this run) |
|---|---|---|---|
| `cargo build --examples --offline` | 43 (default) | pass | 20.29s (incremental) |
| `--example vision_analysis --example vision_battalion --features vision,llm-openai` | 2 | pass | 19.50s |
| `--example document_processing --features content-processing` | 1 | pass | 1m 03s (new deps: bollard, mockito, wiremock, dual reqwest, pdf-extract, sqlx-sqlite, rmcp, testcontainers*) |
| `--example http_service_host --features web-server` | 1 | pass | 30.07s |

None of the 43 auto-discovered (un-gated) examples failed under default features — they compile
cleanly with only `llm-openai` enabled, meaning none of them silently depends on a non-default
feature without declaring `required-features` (a state that would otherwise produce a hard compile
error, not a silent skip, so this was self-verifying: the bulk build either fails loudly for
ungated examples needing missing features, or succeeds).

**Recommendation for the plan:** record "every example target builds" as a **green, verified claim**
for D-12, conditioned explicitly on the CI job implementing the 4-invocation feature matrix from
Q1(b) rather than a single bare `cargo build --examples`. This is a documentation/CI-authoring task,
not a code-fix task — no example source needs changing.

## Q3. QUICKSTART: step sequence, offline reachability, and staleness

### Step sequence (`docs/src/getting-started/quickstart.md`, 127 lines, read whole)

1. Prerequisite: complete `installation.md`, `export OPENAI_API_KEY=...` (`:5-11`)
2. `cargo new my-paladin-agent && cd my-paladin-agent` (`:15-18`)
3. Hand-add 4 dependency lines to the new project's `Cargo.toml`: `paladin-ai-core = "0.5.0"`,
   `paladin-ports = "0.5.0"`, `paladin-llm = { version = "0.5.0", features = ["llm-openai"] }`,
   `tokio = { version = "1", features = ["full"] }` (`:20-28`)
4. Replace `src/main.rs` with a ~35-line program that builds a `PaladinBuilder`, wraps it in a
   `PaladinExecutionService`, and calls `.execute(...)` (`:32-69`)
5. `cargo run` (`:73-75`), expect specific stdout (`:79-83`)
6. Optional: clone the full workspace, `make services-up`, `cargo run --example basic_paladin`,
   `cargo run --example formation_sequential`, (commented) `cargo run --example phalanx_concurrent`
   (`:85-105`)
7. Reference table for `PaladinResult` fields (`:107-117`) — no executable step
8. "What's next" links (`:119-127`) — no executable step

### Steps requiring network or an LLM key (not timeable in this sandbox)

- **Step 2-3 (network):** `cargo new` itself is offline-safe, but the moment `cargo build`/`cargo
  run` is invoked against a fresh project depending on crates.io-hosted `paladin-ai-core`,
  `paladin-ports`, `paladin-llm`, cargo must resolve and fetch those crates from crates.io.
  **crates.io returns HTTP 403 in this sandbox** `[already_established #6]` — this entire path
  cannot be exercised here, `--offline` cannot substitute (there is no local registry mirror of
  these not-yet-published 0.7.0 crates, and `paladin-ai-core` isn't even published under that
  exact version today per D-01's version-state finding).
- **Step 5 (LLM key):** `cargo run` executes `OpenAIAdapter::from_env()?` then makes a live OpenAI
  API call. **No LLM API key is present in this environment** `[already_established #6]` — even if
  step 2-4 could be made to compile, execution would fail at the API call, not produce the
  documented "Hello!" output.
- **Step 6 (partially offline-safe):** cloning the workspace is redundant here (already checked
  out); `make services-up` needs Docker (absent); `cargo run --example basic_paladin` **compiles
  offline fine** (verified as part of the Q2 default-feature build) but **executing** it still
  calls out to an LLM and needs a key, so it fails at the same point step 5 does.

### Can the happy path reach "a working agent" offline at all?

**No — and independent of the network/key blockers, the primary code sample does not match the
shipped tree's crate layout.** Verified live:
- `PaladinBuilder` and `PaladinExecutionService` are defined in
  `/workspace/src/application/services/paladin/{paladin_builder.rs,paladin_execution_service.rs}`
  `[VERIFIED: grep -rl PaladinExecutionService]` — i.e., inside the **root workspace package**
  `paladin-ai` (`Cargo.toml:33`), whose `[lib] name = "paladin"` (`Cargo.toml:45-46`).
- `crates/paladin-core/src/` (the actual `paladin-ai-core` package, `crates/paladin-core/Cargo.toml`)
  contains only `base/` and `platform/` — the `Node<T>` primitive and `Paladin =
  Node<PaladinData>` type alias (`platform/container/paladin.rs:229`). **It has no
  `application::services` module at all.**
- Quickstart's `Cargo.toml` block (`:22-28`) never lists `paladin-ai` (crate `paladin`) as a
  dependency — only `paladin-ai-core`, `paladin-ports`, `paladin-llm`.
- Therefore the `use paladin_ai_core::application::services::paladin::paladin_builder::
  PaladinBuilder;` import in the code sample (`:36`) references a module path that **does not
  exist in the `paladin-ai-core` crate on this tree**, regardless of version number. This is a
  structural staleness defect, not just a version-number staleness defect — pasting the sample
  code into a project with the documented dependency list would fail to compile with an unresolved
  import, even with network access and an API key.

**Largest honestly-measurable prefix in this sandbox:** steps 2 (`cargo new`) trivially succeeds
offline (no dependency resolution needed for the bare scaffold), but step 3 onward requires either
network (crates.io) or a restructured sample pointing at the in-tree workspace via a `path =`
dependency — neither of which the current page supports. **The only prefix of the *documented* page
that is both offline-reachable and produces a real, running artifact is Step 6's `cargo build
--example basic_paladin`** (compile-only, using this session's already-checked-out workspace and
warm local cargo cache) — running it further needs the absent API key. Recommend the plan record
the QUICKSTART measurement as: *"measured through compile of the in-workspace example set; the
documented new-project happy path (steps 2-5) cannot be measured in this environment due to
crates.io 403 and absent LLM key, and separately cannot be measured on ANY machine as currently
written due to the `paladin_ai_core::application::services` import-path defect — this defect should
be fixed as part of REL-04/D-11's edit to `quickstart.md`, not merely the version numbers."*

### Other staleness found relative to the shipped tree

| Location | Claim | Shipped-tree reality | Verified |
|---|---|---|---|
| `quickstart.md:3` | "under five minutes" | Contradicts `introduction.md:9`'s "15 minutes"; D-11 settles on 15 min | already-established #5 |
| `quickstart.md:24-26` | `paladin-ai-core = "0.5.0"`, `paladin-ports = "0.5.0"`, `paladin-llm = "0.5.0"` | Workspace is at `0.6.0` today, converging to `0.7.0` under D-01/D-03 | `Cargo.toml:34`, member manifests |
| `quickstart.md:36-37` | imports from `paladin_ai_core::application::services::paladin::*` | That module tree does not exist in `paladin-ai-core`; it exists only in the root `paladin` lib crate, which quickstart never declares as a dependency | `grep -rl PaladinExecutionService`, `crates/paladin-core/Cargo.toml`, `Cargo.toml:45-46` |
| `quickstart.md:104` | (commented) `cargo run --example phalanx_concurrent` | File is `examples/phalanx_parallel.rs` — no `phalanx_concurrent.rs` exists | `ls examples/ | grep phalanx` |
| `quickstart.md:98,101` | `cargo run --example basic_paladin`, `formation_sequential` | Both files exist and both compile under default features (verified in Q2's matrix run) | `find examples -name 'basic_paladin.rs' -o -name 'formation_sequential.rs'` |

## Validation Architecture

`.planning/config.json` `[VERIFIED: grep]` has no `workflow.nyquist_validation` key — absent means
enabled; this section is required.

### Test framework
| Property | Value |
|---|---|
| Framework | `cargo test` (workspace built-in), no external test runner |
| Config file | none dedicated — behavior driven by `Cargo.toml` `[[test]]` entries (`:172-218`) and `ci.yml` job definitions |
| Quick run (fmt/lint) | `cargo fmt --all -- --check && cargo clippy --workspace --all-targets --all-features -- -D warnings` |
| Full suite | `cargo test --workspace` (unit + integration; Phase 2 recorded 2864 passed / 0 failed at this tree — re-run for this phase, do not cite per D-12) |

### Phase Requirements -> Validation Map
| SC | Behavior | Validation type | Command | Executable here? |
|---|---|---|---|---|
| SC1 (version) | Manifests, tag, CHANGELOG, release notes agree | grep/diff across 12 files | `grep -h '^version' Cargo.toml crates/*/Cargo.toml`, `git tag --list`, `head CHANGELOG.md` | Yes — fully local |
| SC2 (edition) | One edition workspace-wide; `cargo build --workspace` (both default and `--no-default-features`) succeed | build | `cargo build --workspace --offline` (verified this session: 15.95s, clean); `cargo build --workspace --no-default-features --offline` (not yet run — planner should schedule as a task, not assume) | Yes |
| SC3 (advisories) | `cargo audit` 0 vulnerabilities; `cargo deny check` clean; every ignore has rationale + migration/review note | tool run + manual doc audit | `cargo audit`; `cargo deny check` | Yes — both tools work despite crates.io 403 (advisory DB is a GitHub repo; `[already_established #6]`) |
| SC4 (QUICKSTART) | Timed against 15-min target, pass or fail | manual/scripted timing | wall-clock of the documented step sequence | **Partially** — see Q3; full happy path blocked by crates.io 403 + no LLM key + the import-path defect. Measure the offline-reachable prefix only, record the rest as `deferred with reason` |
| SC5 (gate suite) | fmt, clippy `-D warnings`, workspace tests, doc tests, all 47 examples, multi-arch Docker inside budget, K8s smoke inside budget | mixed | `cargo fmt --all -- --check`; `cargo clippy --workspace --all-targets --all-features -- -D warnings`; `cargo test --workspace`; `cargo test --workspace --doc --exclude paladin-ports` (mirrors `ci.yml:225`'s existing doctest exclusion of `paladin-ports`, whose `doctest = false` is DEBT-03/out of scope here); the 4-invocation example matrix from Q1(b); Docker/K8s jobs | **fmt/clippy/tests/doctests/examples: yes, all executable here.** **Docker multi-arch build and K8s kind smoke: no — `docker`, `kind`, `kubectl` all absent** `[already_established #6]`. Per D-15, these two are authored + statically validated (YAML parses, action refs resolve, `Dockerfile`/`k8s/*.yaml` references resolve) but **never claimed green** in this environment. |

### Sampling rate
- **Per task commit:** `cargo fmt --all -- --check` (fast, already verified clean at HEAD)
- **Per wave merge:** `cargo clippy --workspace --all-targets --all-features -- -D warnings` +
  `cargo build --workspace --offline` + the 4-invocation example matrix
- **Phase gate:** `cargo test --workspace` full suite + `cargo audit` + `cargo deny check`, all
  green, before `/gsd-verify-work`. Docker/K8s jobs are gated by YAML-lint + static-reference-check
  only, per D-15 — **do not require them green as a phase-gate condition**, since they cannot
  execute here.

### Wave 0 gaps
None — no new test files or fixtures are needed. This phase edits manifests, `CHANGELOG.md`,
`ci.yml`, `deny.toml`, `docs/src/getting-started/quickstart.md`, and `.planning/` records; it adds
no product code requiring new unit/integration tests. The "tests" this phase produces are CI job
definitions themselves, validated by YAML syntax + static reference checks (see SC5 row above), not
by `pytest`/`cargo test`-style fixtures.

### What explicitly cannot be validated in this environment, and why

1. **Docker multi-arch build (`linux/amd64,linux/arm64`) and its < 500 MB / < 5 min budget** —
   `docker` binary absent from this sandbox. `[already_established #6]`
2. **kind-based Kubernetes smoke test and its < 30 s pod-startup budget** — `kind`, `kubectl`
   absent. `[already_established #6]`
3. **The QUICKSTART's "clean machine, cold registry" claim** — crates.io returns HTTP 403 here, so a
   cold dependency fetch cannot be timed; the local cargo registry/build cache is already warm from
   this and prior sessions, and Docker (needed for `make services-up`) is absent. Per D-11.2, record
   what *is* measurable (offline-reachable steps only) under stated conditions, not a clean-machine
   figure.
4. **The QUICKSTART's LLM call** (`OpenAIAdapter::from_env()?` executing against the real OpenAI
   API) — no LLM API key present. `[already_established #6]`
5. **Triggering the repaired CI workflow itself** — `gh` is available for *reading* workflow-run
   history (D-16), but dispatching a run, pushing the `release/**` trigger change, or opening a PR
   is an outward-facing action gated the same way as D-03's tag push. This research/plan can author
   and statically validate the YAML; only a live GitHub Actions runner (reachable after a human
   pushes) can prove SC5's CI-driven claims execute.
6. **`cargo build --workspace --no-default-features --offline`** — not run in this research session
   (budget discipline); the planner should schedule it as an explicit verification task per D-06's
   proof obligation ("both must succeed"), not assume it from the default-features build's success.

**Authoring CI configuration is not the same as proving a gate (D-15).** Every row above that says
"authored + statically validated" must be recorded as such in the plan's verification section —
never as "SC5 met."

## Package Legitimacy Audit

Not applicable. This phase installs no new external packages — it edits existing CI YAML, Cargo
manifests, `CHANGELOG.md`, `deny.toml`, and `docs/src/getting-started/quickstart.md`. All action
references cited above (`docker/setup-buildx-action@v4`, `docker/build-push-action@v6`,
`docker/setup-qemu-action@v3`, `helm/kind-action@v1`, `actions/checkout@v5`, `actions/cache@v4`)
are versions **already present and running** in this repository's own workflow files, not newly
introduced — cited with `file:line` above, not sourced from WebSearch or training data.

## Open gaps

1. **Docker build time budget (< 5 min) has no existing precedent anywhere in this repo.** Neither
   `ci.yml` nor `release.yml` times a Docker build today. The plan must author this from scratch
   (epoch-diff pattern modeled on the K8s startup-time check), and must decide whether the 5-minute
   figure applies per-architecture or to the whole multi-arch `linux/amd64,linux/arm64` build
   (QEMU-emulated arm64 on an `ubuntu-latest` amd64 runner is markedly slower than native) — this
   research found no clarifying source (`PROJECT.md`, `ROADMAP.md`, `REQUIREMENTS.md`) that
   disambiguates. **Flagging for the planner or a discuss-phase follow-up, not resolving here.**
2. **`k8s/deployment.yaml` runs a placeholder command (`sleep 3600`) with all readiness/liveness
   probes commented out.** The existing `kubernetes-smoke-test` job's < 30s pod-startup measurement
   is real (kind/kubectl orchestration genuinely executes), but it measures container-scheduling
   time against a trivial placeholder, not application-readiness time against a real HTTP health
   endpoint. Whether SC5's "Kubernetes smoke test inside its startup budget" is satisfied by the
   placeholder-based job as-is, or requires wiring `paladin-web`'s health endpoints into the
   commented-out probes first (arguably new capability, outside this phase's "no new product
   capability" boundary), is unresolved. Recommended in Q1(d): accept the placeholder-based
   reuse and record the probe-wiring gap as a named deferral (owner TBD by the planner — not
   assigned in `04-CONTEXT.md`'s deferred list).
3. **Whether `release/**` should also be added to `integration-tests.yml` and `feature-flags.yml`'s
   commented-out `push:` stanzas** (both carry the identical convention as `ci.yml`'s pre-fix
   stanza) is unresolved — D-14 only names `ci.yml`. Left as a plan-level question in Q1(a).
4. **`cargo build --workspace --no-default-features --offline` was not run in this research
   session** (budget discipline, see Validation Architecture item 6). The default-features build
   was verified clean; the no-default-features leg (part of D-06's proof obligation) must be run by
   the plan/execution phase, not assumed from this research.
5. **Cold-build wall-clock time for `cargo build --examples --offline` under default features was
   not independently measured from a fully clean target directory** — this session's cargo caches
   were already warm from repeated builds. The 20.29s figure reflects incremental recompilation
   after a `touch` of all example source files, not a from-scratch build. If the plan needs a true
   cold-build time budget for the examples job, it should be measured freshly (e.g., `cargo clean`
   first) rather than cited from this research.
