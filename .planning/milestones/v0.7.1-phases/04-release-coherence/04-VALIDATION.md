---
phase: 4
slug: release-coherence
# status lifecycle: draft (seeded by plan-phase) → validated (set by validate-phase §6)
# audit-milestone §5.5 distinguishes NOT-VALIDATED (draft) from PARTIAL (validated + nyquist_compliant: false) (#2117)
status: draft
nyquist_compliant: false
wave_0_complete: false
created: 2026-08-02
---

# Phase 4 — Validation Strategy

> Per-phase validation contract for feedback sampling during execution.
> Seeded from `04-RESEARCH.md` §"Validation Architecture". The Per-Task Verification Map is
> filled once plans exist.

---

## Test Infrastructure

| Property | Value |
|----------|-------|
| **Framework** | `cargo test` (workspace built-in) — no external test runner |
| **Config file** | none dedicated; behavior driven by `Cargo.toml` `[[test]]` entries (`:172-218`) and `.github/workflows/ci.yml` job definitions |
| **Quick run command** | `cargo fmt --all -- --check` |
| **Full suite command** | `cargo test --workspace` |
| **Estimated runtime** | quick ~5 s; full suite several minutes (Phase 2 recorded 2864 passed / 0 failed on this tree — **re-run, do not cite**, per D-12) |

**Supplementary gate commands** (this phase's deliverables are manifests, CI config and docs, so
the gates below carry as much weight as `cargo test`):

| Gate | Command | Executable here? |
|------|---------|------------------|
| Lint | `cargo clippy --workspace --all-targets --all-features -- -D warnings` | Yes |
| Build | `cargo build --workspace --offline` | Yes — verified 15.95 s clean this session |
| Build (no-default) | `cargo build --workspace --no-default-features --offline` | Yes — **not yet run; schedule as an explicit task** per D-06 |
| Doc tests | `cargo test --workspace --doc --exclude paladin-ports` | Yes — mirrors `ci.yml:225`'s existing exclusion; `paladin-ports`' `doctest = false` is DEBT-03 / Phase 8, out of scope |
| Examples | 4-invocation feature matrix (default + `vision` + `content-processing` + `web-server`) — see `04-RESEARCH.md` Q1(b) | Yes — all 47 verified building across the four invocations |
| Advisories | `cargo audit` and `cargo deny check` | Yes — both work despite crates.io 403; the advisory DB is a GitHub repo |
| Docker multi-arch + budgets | `docker buildx` | **No — `docker` absent** |
| Kubernetes smoke + budget | `kind` / `kubectl` | **No — both absent** |

---

## Sampling Rate

- **After every task commit:** `cargo fmt --all -- --check`
- **After every plan wave:** `cargo clippy --workspace --all-targets --all-features -- -D warnings`
  + `cargo build --workspace --offline` + the 4-invocation example matrix
- **Before `/gsd-verify-work`:** `cargo test --workspace` + `cargo audit` + `cargo deny check` all green
- **Max feedback latency:** ~60 s for the per-task quick run

**Explicit non-gate:** the Docker and Kubernetes jobs are validated by YAML parse + static
reference resolution only (per D-15). **They must NOT be required green as a phase-gate
condition** — they cannot execute in this environment, and authoring CI configuration is not the
same as proving a gate.

---

## Per-Task Verification Map

| Task ID | Plan | Wave | Requirement | Threat Ref | Secure Behavior | Test Type | Automated Command | File Exists | Status |
|---------|------|------|-------------|------------|-----------------|-----------|-------------------|-------------|--------|
| _pending_ | — | — | REL-01…REL-05 | — | N/A | — | — | — | ⬜ pending |

*Filled once `04-*-PLAN.md` files exist. Status: ⬜ pending · ✅ green · ❌ red · ⚠️ flaky*

---

## Wave 0 Requirements

Existing infrastructure covers all phase requirements — **no Wave 0 work needed.**

This phase edits manifests, `CHANGELOG.md`, `.github/workflows/ci.yml`, `deny.toml`,
`.cargo/audit.toml`, `docs/src/getting-started/quickstart.md` and `.planning/` records. It adds no
product code, so no new test files or fixtures are required. The "tests" this phase produces are
CI job definitions, validated by YAML syntax and static reference checks rather than by
`cargo test`-style fixtures.

---

## Manual-Only Verifications

| Behavior | Requirement | Why Manual | Test Instructions |
|----------|-------------|------------|-------------------|
| Multi-arch Docker build inside < 500 MB / < 5 min budget | REL-05 | `docker` binary absent from this environment | Authored + statically validated only. First execution requires a Docker-capable CI runner. Owner: Phase 15 / PIPE |
| Kubernetes smoke test inside < 30 s pod-startup budget | REL-05 | `kind` and `kubectl` absent | Same. Note `k8s/deployment.yaml` runs a placeholder `sleep 3600` with readiness probes commented out — the budget check measures scheduling, not app readiness |
| CI actually running on a `release/**` push | REL-05 | Cannot trigger GitHub Actions from here; pushing is an outward-facing action gated like D-03's tag push | `gh` may **read** workflow-run history (D-16). Dispatching, pushing, or opening a PR stays behind the human gate |
| QUICKSTART timing on a clean machine with a cold registry | REL-04 | crates.io returns HTTP 403; the local registry and build cache are already warm; `make services-up` needs Docker | Measure the offline-reachable prefix under stated conditions (D-11.2) and record the clean-machine figure as `deferred with reason` |
| QUICKSTART's live LLM call (`OpenAIAdapter::from_env()`) | REL-04 | No LLM API key present | Not timeable here. **Note:** the sample's import paths are also structurally wrong (`PaladinBuilder` / `PaladinExecutionService` live in the root `paladin` crate, not `paladin-ai-core`), which is a real REL-04 work item, not an environment limitation |
| Pushing tag `v0.7.0` and publishing ten crates to crates.io | REL-01 | Irreversible, outward-facing | Human action behind an explicit gate (D-03). `make release` is unsafe: its branch guard (`Makefile:456-466`) requires `main`, and `Makefile:484-485` runs raw `git push` that `release.toml`'s `push = false` does not protect against |

---

## Validation Sign-Off

- [ ] All tasks have `<automated>` verify or a documented manual-only entry above
- [ ] Sampling continuity: no 3 consecutive tasks without automated verify
- [ ] Wave 0 covers all MISSING references (N/A — no Wave 0 work)
- [ ] No watch-mode flags
- [ ] Feedback latency < 60 s
- [ ] Every "authored + statically validated" item is recorded as such in the plan's verification
      section — never as "SC5 met"
- [ ] `nyquist_compliant: true` set in frontmatter

**Approval:** pending
