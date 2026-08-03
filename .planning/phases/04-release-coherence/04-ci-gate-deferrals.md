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

---

## First real CI execution — 2026-08-03

**Added by the phase orchestrator after `release/v0.7.0` was pushed to origin.** The user
supplied a `GH_TOKEN`, so run results became readable for the first time; the deferral rows
above were written when `gh` was unauthenticated and CI results were genuinely unknown.

### What the push proved

Run `30824230947` (`CI/CD Pipeline`, push, `release/v0.7.0`, 11m57s):

- **The `release/**` push trigger fires.** This is REL-05's core repair, and it is now
  proven by execution rather than by YAML inspection. Before this phase, `ci.yml`'s `push:`
  trigger was commented out and a push to this branch ran nothing.
- **`Unit Tests (stable)`, `Unit Tests (beta)` and `Integration Tests` all passed on clean
  CI runners** — independent corroboration of plan 04-04's locally-measured suite, from
  machines that are not this sandbox.
- **`OSV Scanner` passed.**

### What it did not prove — the three Phase 4 jobs were SKIPPED, not run

`Example Muster (Feature Matrix)`, `Docker Build` and `Kubernetes Smoke Test` all show
`skipped`: their `needs:` dependencies failed, so they never executed. **The deferral rows
above are unchanged and remain accurate.** No Docker image was built, no image size or build
time was measured, no kind cluster started, no pod-startup figure was taken.

### The blocker, and why it is not this phase's regression

14 jobs failed at `Install Rust toolchain` in 4-7 seconds each:

```
toolchain:
'toolchain' is a required input
##[error]Process completed with exit code 1.
```

`dtolnay/rust-toolchain` is referenced at the moving `@master` ref. Upstream made
`toolchain` a required input sometime after this branch's last green run (2026-07-06); the
repository's pinned-action hygiene did not catch it because the ref is not pinned.

**Verified pre-existing, not introduced here.** At this phase's base commit `68ba809` there
were **8** usages of `dtolnay/rust-toolchain@master` of which only **1** passed a
`toolchain:` input — seven were already latent-broken. Phase 4 added exactly one line
(`uses: dtolnay/rust-toolchain@master` in the new `examples` job, copying the convention of
the analog jobs it was instructed to reuse) and modified **zero** existing toolchain steps.
The next push to this branch was going to fail regardless of its content; Phase 4 was simply
the first push since the upstream change.

### The fix applied

Eight input-less usages switched to the action's documented no-input form,
`dtolnay/rust-toolchain@stable` (lines 29, 69, 89, 239, 336, 474, 782, 868). The one usage
that already passes an explicit input — line 209, `toolchain: ${{ matrix.rust-version }}`,
the stable/beta matrix, and **the only job of the fourteen that passed** — is deliberately
left on `@master` and unmodified.

`rust-toolchain.toml` continues to pin the effective toolchain at **1.97.1** and, by its own
documented contract, overrides whatever version the action installs. The effective toolchain
in CI is therefore unchanged by this fix; only the action's required-input error is resolved.

### Still open after this fix

- `API Surface Tracking` fails for an unrelated, pre-existing reason — **DEBT-01, Phase 8**.
  It is not one of SC5's named gates and is not addressed here.
- Pinning `dtolnay/rust-toolchain` to a commit SHA so this class of silent upstream drift
  cannot recur was considered and **not** done: it would touch the one working job and
  duplicate the version across nine places, against `rust-toolchain.toml`'s single-source-of-
  truth contract. **Owner: Phase 15 / PIPE-04**, alongside the deprecated-actions sweep.

---

## Second CI execution — 2026-08-03, after the toolchain fix

Run `30833088746` (`CI/CD Pipeline`, push, `release/v0.7.0`). The toolchain fix cleared the
blockage; 14 previously-failing jobs passed and two of the three Phase 4 jobs finally executed.

### `Example Muster (Feature Matrix)` — **SUCCESS. No longer deferred.**

The examples job executed on a clean GitHub runner and passed. This is REL-05's examples
clause **proven by execution**, not by YAML inspection. It is the clause carrying the trap
that a bare `cargo build --examples` silently covers only 43 of 47 targets; the four-invocation
feature matrix plus the 47-binary assertion both held in CI. **Deferral row 1 above is
superseded — this gate is satisfied.**

### `Docker Build` — **EXECUTED AND FAILED. First real measurement.**

```
Docker build wall-clock time: 2946s
##[error]Docker multi-arch build took 2946s, exceeding the 300s budget
```

Job duration 49m43s. **The image itself built successfully** — `Build Docker image` passed;
the failure is the wall-clock assertion alone.

**This is the measurement REL-05 had never taken, and it says the budget is mis-specified
rather than merely unmet.** SC5 asserts a *multi-arch* build inside a 500 MB / 5-minute
budget, but those two figures trace to `PROJECT.md:767` — "112 MB built in 5m31s" — which was
a **single-arch** measurement from Milestone 1. Building `linux/arm64` under QEMU emulation on
a stock `ubuntu-latest` runner is roughly an order of magnitude slower than native amd64, so
2946s is not an anomaly to tune away: it is what multi-arch-under-emulation costs. As written,
SC5's time budget is unreachable for the build SC5 itself demands.

**The image-size budget remains unmeasured.** `Assert image size budget (<= 500 MB)` is the
step immediately after the failing one and never ran. Nothing here establishes whether the
multi-arch image is over or under 500 MB.

**No budget was relaxed to make this pass.** Changing a success criterion's figure to fit a
measurement is the inversion this milestone exists to end; the number is recorded as-is and
the disposition is left to a human. Candidate resolutions, none adopted here:
1. Scope the existing budget to single-arch and give multi-arch its own, larger one.
2. Replace QEMU emulation with native `arm64` runners or cross-compilation, then re-measure.
3. Keep the multi-arch build but make the time assertion non-blocking, with the size assertion
   still hard — accepting that build *duration* is a runner property, not a code property.

### `Kubernetes Smoke Test` — **still never executed.** `skipped`; its `needs:` chain includes
`Docker Build`, which failed. The 30 s pod-startup budget remains unmeasured, and the
`k8s/deployment.yaml` placeholder caveat (`sleep 3600`, readiness probes commented out) is
still untested. **Deferral row 2 stands unchanged. Owner: Phase 15 / PIPE.**

### `API Surface Tracking` — still failing, unrelated and pre-existing. **DEBT-01, Phase 8.**
Not one of SC5's named gates.

### Standing note
`docker/build-push-action@v6` and `docker/setup-qemu-action@v3` raised Node.js 20 deprecation
annotations on this run. Not addressed here — **Owner: Phase 15 / PIPE-04**, with the
deprecated-actions sweep.
