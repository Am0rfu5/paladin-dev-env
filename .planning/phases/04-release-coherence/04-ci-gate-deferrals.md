# Phase 4 Plan 3 — CI Gate Deferral Register (D-15)

**Authoring CI configuration is not proving a gate.** Nothing in this file is evidence that SC5's
Docker or Kubernetes clauses are met. `docker`, `kind` and `kubectl` are all absent from the
environment that authored `.github/workflows/ci.yml`'s `docker` and `kubernetes-smoke` jobs, so
those two jobs have been **authored and statically validated only** — never executed — and every
claim below about them is scoped to that distinction on purpose.

## Provenance block (D-17)

Command: `rustc -vV`
```
rustc 1.97.1 (8bab26f4f 2026-07-14)
binary: rustc
commit-hash: 8bab26f4f68e0e26f0bb7960be334d5b520ea452
commit-date: 2026-07-14
host: x86_64-unknown-linux-gnu
release: 1.97.1
LLVM version: 22.1.6
```

Command: `cargo --version`
```
cargo 1.97.1 (c980f4866 2026-06-30)
```

Command: `git rev-parse HEAD`
```
2526fefef605fa92fd8a9f26ef497ebcccfba2e6
```

Command: `git rev-parse --abbrev-ref HEAD`
```
worktree-agent-afefb774f71edc3aa
```
This is the per-agent worktree branch this plan executed on (parallel-executor isolation), not
`release/v0.7.0` itself — the orchestrator merges this branch's commits back into the phase's target
branch after all wave agents complete. It does not change any of the findings below.

Command: `git status --porcelain`
```
(no output — clean working tree at the moment this record was captured, after committing Tasks 1
and 2's ci.yml changes and before writing this file and COVERAGE.md)
```

Command: `date -u`
```
Mon Aug  3 00:18:42 UTC 2026
```

## Static validation (the only kind of proof available for the Docker and Kubernetes jobs)

Three checks, all runnable in this environment. All three ran against the full file after Tasks 1
and 2, not only against the new jobs.

### 1. YAML parses

Command: `python3 -c "import yaml,sys; yaml.safe_load(open('.github/workflows/ci.yml'))"`
```
(exits 0, no output — yaml.safe_load raised no exception)
```

### 2. Every action reference introduced by the new/extended jobs already runs somewhere in this repository's own workflow files

Command: `python3 -c "import yaml; d = yaml.safe_load(open('.github/workflows/ci.yml')); [print(s['uses']) for j in ('examples','docker','kubernetes-smoke') for s in d['jobs'][j]['steps'] if isinstance(s, dict) and 'uses' in s]"`
```
actions/checkout@v5
dtolnay/rust-toolchain@master
actions/cache@v4
actions/cache@v4
actions/cache@v4
actions/checkout@v5
docker/setup-qemu-action@v3
docker/setup-buildx-action@v4
docker/build-push-action@v6
actions/checkout@v5
helm/kind-action@v1
```
`actions/checkout@v5`, `dtolnay/rust-toolchain@master`, `actions/cache@v4`,
`docker/setup-buildx-action@v4` and `docker/build-push-action@v6` were already used elsewhere in
`ci.yml` itself before this plan's edits. The two references this plan actually introduces to
`ci.yml` are:

Command: `grep -rn "docker/setup-qemu-action@v3" .github/workflows/*.yml`
```
.github/workflows/ci.yml:507:        uses: docker/setup-qemu-action@v3
.github/workflows/release.yml:172:        uses: docker/setup-qemu-action@v3
```

Command: `grep -rn "helm/kind-action@v1" .github/workflows/*.yml`
```
.github/workflows/integration-tests.yml:179:        uses: helm/kind-action@v1
.github/workflows/ci.yml:580:        uses: helm/kind-action@v1
```
Both already ran in a sibling workflow (`release.yml:172`, `integration-tests.yml:179`) before this
plan added them to `ci.yml` — no new marketplace action was introduced, satisfying T-04-10.

### 3. Every filesystem path the new jobs reference resolves to a file that exists

Command: `grep -oE 'k8s/[a-z-]+\.yaml' .github/workflows/ci.yml | sort -u`
```
k8s/configmap.yaml
k8s/deployment.yaml
k8s/minio.yaml
k8s/redis.yaml
k8s/service.yaml
```
Each of the five confirmed present with `test -f`: `k8s/configmap.yaml`, `k8s/deployment.yaml`,
`k8s/minio.yaml`, `k8s/redis.yaml`, `k8s/service.yaml` all exist. `test -f Dockerfile` also
succeeds — the `docker` job's `context: .` / `file: Dockerfile` reference resolves.

## Reading recorded CI state (D-16 — read-only)

Command: `gh --version`
```
gh version 2.96.0 (2026-07-02)
https://github.com/cli/cli/releases/tag/v2.96.0
```

Command: `git remote get-url origin`
```
https://github.com/DF3NDR/paladin-dev-env.git
```

Command: `gh run list --branch release/v0.7.0 --limit 10`
```
To get started with GitHub CLI, please run:  gh auth login
Alternatively, populate the GH_TOKEN environment variable with a GitHub API authentication token.
(exit code 4)
```
**Recorded honestly, not smoothed over:** `04-CONTEXT.md`'s D-16 verified `gh 2.96.0` reading the
remote successfully during the phase's discussion session. In *this* execution sandbox `gh` is
installed at the same version but is not authenticated (`gh auth status` also fails: "You are not
logged into any GitHub hosts"). This is an environment difference between the discussion sandbox and
this plan's execution sandbox, not a reversal of D-16's finding — `gh` remains read-only-authorized
per D-16 regardless of which sandbox runs it, and no `gh auth login`, `gh workflow run`, `gh pr
create`, or `git push` was attempted to work around the missing credential. The evidentiary point
D-14.1 needs (a push to `release/v0.7.0` has never fired this workflow) is independently established
by Static validation check 1 above having found the `push:` trigger commented out until Task 1 of
this plan restored it — that fact, not `gh run list`'s emptiness, is D-14.1's evidence base. First
execution of `gh run list --branch release/v0.7.0` against a real result set is folded into the same
first-execution deferral as the CI jobs themselves (row 4 below).

## Deferred with reason

Every row below cannot be executed in the environment that authored it and is filed with a named
owner, per the corpus's `deferred with reason` verdict class
(`.planning/ledgers/milestone-01.md` §"Verdict legend").

| # | Item | Verdict | Evidence / reason | Owner |
|---|------|---------|--------------------|-------|
| 1 | Multi-arch Docker build (`linux/amd64,linux/arm64`) and its 500 MB / 300 s budgets | deferred with reason | authored and statically validated (YAML parses, both budget assertions hard-fail via `::error::` + `exit 1`, action refs and `Dockerfile` path resolve — see Static validation 1-3 above). `docker` is absent from this environment (verified: no `docker` binary on `PATH`), so the job has never been executed and neither budget has ever actually been measured against a built image. **Owner: Phase 15 / PIPE.** |
| 2 | Kubernetes kind smoke test and its 30 s pod-startup budget | deferred with reason | authored and statically validated (YAML parses, the startup assertion hard-fails on overage and on unreadable timestamps, every `k8s/*.yaml` path resolves — see Static validation 1-3 above). `kind` and `kubectl` are both absent from this environment, so the job has never been executed and the budget has never actually been measured. **Owner: Phase 15 / PIPE.** |
| 3 | Real readiness-probe-based startup measurement | deferred with reason | `k8s/deployment.yaml:66-68` runs a placeholder `command: ["/bin/sh"]` / `args: ["-c", "echo 'Paladin started' && sleep 3600"]`, and all three probes (liveness/readiness/startup) are commented out at `k8s/deployment.yaml:137-174` with the note "needs HTTP server endpoint." Even once the `kubernetes-smoke` job is first executed (row 2), its 30 s figure will measure container scheduling, not application readiness, until real probes are wired. Wiring `paladin-web`'s health endpoints into the Deployment's probes is new product capability, outside this measurement/coherence phase's boundary. **Owner: Phase 14 / WEB.** |
| 4 | CI actually running on a `release/**` push | deferred with reason | Task 1 of this plan restored the `push:` trigger so `release/**` is now covered, but nothing in this sandbox can push a commit or dispatch a workflow run to observe it fire — `gh` is read-only-authorized here (D-16) and is not even authenticated in this execution sandbox (see the `gh run list` output above), and `git push` / `gh workflow run` / `gh pr create` are all outward-facing actions prohibited by this plan. First observation of the trigger actually firing happens the next time a commit lands on `release/v0.7.0` and is pushed. **Owner: the human gate that owns the tag push (D-03).** |
| 5 | The 300 s multi-arch time budget's plausibility | deferred with reason | The only Docker build-time measurement anywhere in this corpus is `PROJECT.md:767`'s "112 MB built in 5m31s" — a **single-arch** build already over five minutes. This job's multi-arch build adds `linux/arm64` via QEMU emulation on top of that, which is markedly slower than native-arch. The 300 s gate is authored to hard-fail anyway rather than softened to a warning (per this plan's prohibition against weakening budgets to make them look achievable); it is expected to be red on first real execution, and that red is the measurement REL-05 has never taken. Revising the figure with evidence, or optimizing the build to fit it, is downstream work. **Owner: Phase 15 / PIPE.** |
| 6 | Whether `integration-tests.yml` and `feature-flags.yml` should also gain a `release/**` trigger | deferred with reason | Both carry the identical commented-out `push:` stanza with the same `main, develop, feature/**` list that `ci.yml` had before Task 1. D-14 names `ci.yml` alone; `integration-tests.yml`'s modernization (including its trigger) belongs to PIPE-04's deprecated-actions sweep, and `feature-flags.yml` is not named in D-14 at all. Deliberately left untouched by this plan so a reader does not mistake the omission for an oversight. **Owner: Phase 15 / PIPE-04.** |

## What this register is not

This register is not a report that SC5's Docker or Kubernetes clauses are met, green, passing, or
proven. It records that the configuration for those two clauses now exists, parses, references only
action versions and file paths already resolvable in this repository, and has never been executed.
First execution — and the only event that can turn any of the six rows above from `deferred with
reason` into `satisfied` — requires a CI runner with Docker (rows 1, 2, 5), `paladin-web` health
endpoints (row 3), or the human-gated tag push (row 4, and by extension row 6's disposition).
