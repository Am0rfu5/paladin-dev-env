---
phase: 2
slug: functional-gap-closure
# status lifecycle: draft (seeded by plan-phase) → validated (set by validate-phase §6)
# audit-milestone §5.5 distinguishes NOT-VALIDATED (draft) from PARTIAL (validated + nyquist_compliant: false) (#2117)
status: draft
nyquist_compliant: false
wave_0_complete: false
created: 2026-07-31
---

# Phase 2 — Validation Strategy

> Per-phase validation contract for feedback sampling during execution.
> Seeded from `02-RESEARCH.md` § Validation Architecture.

---

## Test Infrastructure

| Property | Value |
|----------|-------|
| **Framework** | Native `cargo test` — unit `#[test]`/`#[tokio::test]` in `#[cfg(test)]` modules, integration `[[test]]` binaries, and doctests. No `nextest` or other harness detected. |
| **Config file** | Root `Cargo.toml` `[[test]]` section (`Cargo.toml:171-219`) — no separate test-framework config file |
| **Quick run command** | `cargo test -p <crate> <test_name_or_module>` (per-crate, per-test) |
| **Full suite command** | `cargo test --workspace` |
| **Estimated runtime** | Quick run ~1s (GAP-05 targeted check measured 0.01s); full suite completes within the standard timeout — 2790 passed / 0 failed / 126 ignored across 35 binaries on commit `fb4b942` |

---

## Sampling Rate

- **After every task commit:** Run `cargo test -p <crate> <test_name_or_module>` for the crate touched
- **After every plan wave:** Run `cargo test --workspace`
- **Before `/gsd-verify-work`:** Full suite must be green, with the run output captured as D-01 baseline evidence
- **Max feedback latency:** ~60 seconds for the targeted per-crate run; full-workspace run reserved for wave boundaries

---

## Per-Task Verification Map

> Seeded per-requirement from research. Per-task rows are filled by `/gsd-validate-phase`
> once PLAN.md task IDs exist.

| Task ID | Plan | Wave | Requirement | Threat Ref | Secure Behavior | Test Type | Automated Command | File Exists | Status |
|---------|------|------|-------------|------------|-----------------|-----------|-------------------|-------------|--------|
| TBD | TBD | TBD | GAP-01 | — | N/A | unit + integration | `cargo test -p paladin-battalion chain_of_command` | ✅ existing, verified passing | ⬜ pending |
| TBD | TBD | TBD | GAP-02 | — | N/A | integration | `cargo test --test lib -- integration::battalion::load_test` | ✅ existing, verified passing | ⬜ pending |
| TBD | TBD | TBD | GAP-03 | — | N/A | integration (new) | new test in `tests/integration/` (new file or extend `herald_integration_test.rs`) | ❌ W0 | ⬜ pending |
| TBD | TBD | TBD | GAP-04 | — | Telemetry written only under a configured `metadata_output_dir` | unit | `cargo test -p paladin-battalion commander::tests` | ✅ existing, verified passing | ⬜ pending |
| TBD | TBD | TBD | GAP-05 | — | N/A | unit | `cargo test -p paladin-battalion test_auto_selects` | ✅ existing, verified passing | ⬜ pending |
| TBD | TBD | TBD | GAP-06 | — | N/A | manual review (written document, not a test) | N/A | N/A | ⬜ pending |
| TBD | TBD | TBD | GAP-07 | — | N/A | unit (new + existing) + compile-time | `cargo test -p paladin-core formation`, `cargo test -p paladin-llm capabilities`, `cargo test -p paladin-ports`, `cargo test --workspace` (doctests) | ⚠️ partial — new assertions needed | ⬜ pending |

*Status: ⬜ pending · ✅ green · ❌ red · ⚠️ flaky*

---

## Wave 0 Requirements

- [ ] A Formation-driven, 3-Herald end-to-end test (D-06) — no existing file covers this. Either a new `tests/integration/battalion_herald_end_to_end_test.rs`, or deepen the existing-but-shallow `test_battalion_formation_with_herald` (`tests/integration/herald_integration_test.rs:426-490`).
- [ ] `tests/cli/helpers.rs` (or an equivalent `#[path]` shim) — does not exist; its absence blocks D-09's CLI test cluster (1,895 lines across 5 files in `tests/cli/`, currently commented out of `tests/cli/mod.rs`) from compiling at all.
- [ ] Updated assertions for `Formation::validate`'s new boundary — the existing test asserting the old "at least 2 Paladins" behavior needs its expectation *changed*, not merely supplemented, alongside a new `test_formation_rejects_zero_paladins`.

*GAP-01/02/04/05 test infrastructure is fully in place and green today — no Wave 0 work for those.*

---

## Manual-Only Verifications

| Behavior | Requirement | Why Manual | Test Instructions |
|----------|-------------|------------|-------------------|
| Garrison PRD-acceptance review | GAP-06 | The deliverable is a written review document reconciling Garrison against its PRD acceptance criteria — a judgement recorded in prose, not an assertable runtime behavior | Read the Garrison PRD acceptance criteria and the shipped Garrison implementation; record per-criterion met / not-met / deferred-with-reason in the phase artifact |

---

## Validation Sign-Off

- [ ] All tasks have `<automated>` verify or Wave 0 dependencies
- [ ] Sampling continuity: no 3 consecutive tasks without automated verify
- [ ] Wave 0 covers all MISSING references
- [ ] No watch-mode flags
- [ ] Feedback latency < 60s
- [ ] `nyquist_compliant: true` set in frontmatter

**Approval:** pending
