---
phase: 15
slug: coverage-ci-quality-gates
# status lifecycle: draft (seeded by plan-phase) → validated (set by validate-phase §6)
# audit-milestone §5.5 distinguishes NOT-VALIDATED (draft) from PARTIAL (validated + nyquist_compliant: false) (#2117)
status: draft
nyquist_compliant: false
wave_0_complete: false
created: 2026-08-13
---

# Phase 15 — Validation Strategy

> Per-phase validation contract for feedback sampling during execution.
> Seeded from `15-RESEARCH.md` § Validation Architecture.

---

## Test Infrastructure

| Property | Value |
|----------|-------|
| **Framework** | `cargo test` (built-in) + `insta` for CLI snapshot assertions |
| **Config file** | none dedicated — feature-gated `[[test]]` targets in `Cargo.toml` (`:191-217`) |
| **Quick run command** | `cargo test --workspace --lib --bins` (existing `make test`) |
| **Full suite command** | `cargo test --workspace --features integration-tests -- --test-threads=1` (requires Redis + MinIO via `make services-up`) |
| **Estimated runtime** | quick ~60s (warm) · full suite service-backed, several minutes |

---

## Sampling Rate

- **After every task commit:** Run `cargo test --workspace --lib --bins`
- **After every plan wave:** Run `cargo llvm-cov --workspace --features integration-tests` (needs `make services-up`) plus `cargo test -p paladin-ai --features cli --test cli`
- **Before `/gsd-verify-work`:** Full suite green — `make ci-test` extended with `test-cli`, plus the new `coverage` job's threshold
- **Max feedback latency:** 120 seconds for the per-task quick run

---

## Per-Task Verification Map

> Task IDs are assigned when PLAN.md files are written. `/gsd-validate-phase` fills this table
> against the finalized plans; the requirement→command mapping below is already fixed by research.

| Task ID | Plan | Wave | Requirement | Threat Ref | Secure Behavior | Test Type | Automated Command | File Exists | Status |
|---------|------|------|-------------|------------|-----------------|-----------|-------------------|-------------|--------|
| TBD | TBD | TBD | PIPE-01 | — | N/A | snapshot (insta) | `cargo test -p paladin-ai --features cli --test cli` | ✅ (86 snapshots / 97 tests) | ⬜ pending |
| TBD | TBD | TBD | PIPE-01 | — | N/A | compile-check | `cargo bench --workspace --no-run` | ✅ `[[bench]] config_benchmarks` (`Cargo.toml:254-256`) | ⬜ pending |
| TBD | TBD | TBD | PIPE-02 | — | N/A | CI job (service-backed) | `cargo llvm-cov --workspace --features integration-tests --fail-under-lines <floor>` | ❌ W0 | ⬜ pending |
| TBD | TBD | TBD | PIPE-03 | — | N/A | manual/local | `make coverage` | ❌ W0 | ⬜ pending |
| TBD | TBD | TBD | PIPE-04 | — | N/A | static analysis | `actionlint .github/workflows/*.yml` | ❌ W0 | ⬜ pending |
| TBD | TBD | TBD | PIPE-05 | — | N/A | see plan | see plan | TBD | ⬜ pending |
| TBD | TBD | TBD | DEFER-01 | — | N/A | unit (shared mocks) | `cargo test --workspace --lib` | ❌ W0 (`src/test_support/`) | ⬜ pending |
| TBD | TBD | TBD | DEFER-02 | T-15-01 | password-hashing paths keep their existing guarantees under new tests | unit (`#[tokio::test]`) | module-filtered `cargo llvm-cov report` | ✅ `user_service.rs:467-583` (5 tests) | ⬜ pending |
| TBD | TBD | TBD | DEFER-03 | — | N/A | unit + concurrency | `cargo llvm-cov` module-targeted; `tokio::test(flavor = "multi_thread")` | ✅ `listener.rs:400,471,488,514` (3 tests) | ⬜ pending |

*Status: ⬜ pending · ✅ green · ❌ red · ⚠️ flaky*

---

## Wave 0 Requirements

- [ ] `.github/workflows/ci.yml` — new `cli-tests`, `bench-check`, `coverage` job definitions
- [ ] `Makefile` — new Coverage section (`coverage`, `coverage-html`), `test-cli`, `bench-check` targets, `ci-test` extended, new `ci-full` target
- [ ] `.codecov.yml` — new file at repo root
- [ ] `src/test_support/` — new module (planner-named) for D-08's shared `Send + Sync` mocks
- [ ] `actionlint` invocation — not currently run anywhere in CI or locally; needs a CI job or a documented local/pre-commit check
- [ ] `llvm-tools-preview` component added to the `coverage` job's toolchain step (`cargo-llvm-cov` fails without it)

*No test-infrastructure gap for the coverage-target modules themselves — both `user_service.rs` and
`listener.rs` already have working `#[cfg(test)]` / `#[tokio::test]` scaffolding to extend.*

---

## Manual-Only Verifications

| Behavior | Requirement | Why Manual | Test Instructions |
|----------|-------------|------------|-------------------|
| `make coverage` reproduces the CI number locally | PIPE-03 | Requires a developer machine with Redis + MinIO up; the point of the requirement is human reproducibility | `make services-up && make coverage`, compare the reported line-coverage % against the CI `coverage` job's output for the same commit |

---

## Validation Sign-Off

- [ ] All tasks have `<automated>` verify or Wave 0 dependencies
- [ ] Sampling continuity: no 3 consecutive tasks without automated verify
- [ ] Wave 0 covers all MISSING references
- [ ] No watch-mode flags
- [ ] Feedback latency < 120s
- [ ] `nyquist_compliant: true` set in frontmatter

**Approval:** pending
