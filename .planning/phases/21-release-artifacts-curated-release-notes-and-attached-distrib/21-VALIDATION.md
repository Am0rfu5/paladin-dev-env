---
phase: 21
slug: release-artifacts-curated-release-notes-and-attached-distrib
# status lifecycle: draft (seeded by plan-phase) → validated (set by validate-phase §6)
# audit-milestone §5.5 distinguishes NOT-VALIDATED (draft) from PARTIAL (validated + nyquist_compliant: false) (#2117)
status: draft
nyquist_compliant: false
wave_0_complete: false
created: 2026-08-31
---

# Phase 21 — Validation Strategy

> Per-phase validation contract for feedback sampling during execution.

---

## Test Infrastructure

| Property | Value |
|----------|-------|
| **Framework** | bash script tests (`tests/scripts/*_test.sh`, Phase 20 pattern) + `cargo test` |
| **Config file** | none — plain executable test scripts |
| **Quick run command** | `bash tests/scripts/<script>_test.sh` (per changed script) |
| **Full suite command** | `for t in tests/scripts/*_test.sh; do bash "$t"; done && cargo test --workspace` |
| **Estimated runtime** | ~60 seconds (script tests seconds; cargo test dominates) |

---

## Sampling Rate

- **After every task commit:** Run the changed script's `tests/scripts/*_test.sh`
- **After every plan wave:** Run all script tests
- **Before `/gsd-verify-work`:** Full suite must be green
- **Max feedback latency:** 120 seconds

---

## Per-Task Verification Map

| Task ID | Plan | Wave | Requirement | Threat Ref | Secure Behavior | Test Type | Automated Command | File Exists | Status |
|---------|------|------|-------------|------------|-----------------|-----------|-------------------|-------------|--------|
| *(seeded by planner)* | | | ARTIFACT-01..06 | | | script | `tests/scripts/*_test.sh` | ❌ W0 | ⬜ pending |

*Status: ⬜ pending · ✅ green · ❌ red · ⚠️ flaky*

---

## Wave 0 Requirements

- [ ] `tests/scripts/` test files for any new/changed release scripts (extraction, body finalize) — stubs for ARTIFACT-01, ARTIFACT-03
- [ ] No new framework install — Phase 20's script-test pattern covers this phase

---

## Manual-Only Verifications

| Behavior | Requirement | Why Manual | Test Instructions |
|----------|-------------|------------|-------------------|
| End-to-end throwaway-tag rehearsal (assets download+verify, image pulls by digest, body matches changelog section) | ARTIFACT-06 | Requires a real tag push, live GitHub/ghcr/crates.io runs | Per CONTEXT.md D-14; evidence in `21-ARTIFACT-EVIDENCE.md` |
| aarch64 cross-build of all three binaries | ARTIFACT-02 | No Docker in dev sandbox; only the CI leg proves it | Observed on the rehearsal run's aarch64 leg |

---

## Validation Sign-Off

- [ ] All tasks have `<automated>` verify or Wave 0 dependencies
- [ ] Sampling continuity: no 3 consecutive tasks without automated verify
- [ ] Wave 0 covers all MISSING references
- [ ] No watch-mode flags
- [ ] Feedback latency < 120s
- [ ] `nyquist_compliant: true` set in frontmatter

**Approval:** pending
