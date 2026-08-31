---
phase: 21
slug: release-artifacts-curated-release-notes-and-attached-distrib
# status lifecycle: draft (seeded by plan-phase) → validated (set by validate-phase §6)
# audit-milestone §5.5 distinguishes NOT-VALIDATED (draft) from PARTIAL (validated + nyquist_compliant: false) (#2117)
status: draft
nyquist_compliant: true
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
| 21-01 T1 (tracer) | 21-01 | 1 | ARTIFACT-01, -03 | T-21-01, T-21-02 | Version via `env:` only; section reaches the API as a file, never inline | script (fixture + `gh` stub) | `bash tests/scripts/extract-changelog-section_test.sh` | ❌ W0 — created by this task | ⬜ pending |
| 21-01 T2 | 21-01 | 1 | ARTIFACT-01 | T-21-02 | Regex metacharacters in a version are escaped, never interpreted | script (fixture) | `bash tests/scripts/extract-changelog-section_test.sh && make test-shell-guards` | ✅ after T1 | ⬜ pending |
| 21-02 T1 (tracer) | 21-02 | 2 | ARTIFACT-02, -05 | T-21-05, T-21-08 | A leg producing no executable fails before archiving | script (fixture) + local cargo build | `bash tests/scripts/package-release-binaries_test.sh && cargo build --bins --features cli,web-server` | ❌ W0 — created by this task | ⬜ pending |
| 21-02 T2 | 21-02 | 2 | ARTIFACT-02 | T-21-08 | Exact-name matching; empty manifest is a hard failure | script (fixture) | `bash tests/scripts/package-release-binaries_test.sh && make test-shell-guards` | ✅ after T1 | ⬜ pending |
| 21-02 T3 | 21-02 | 2 | ARTIFACT-06 | T-21-07, T-21-09 | No archived action; SHA-pinned refs kept in write-permission jobs | source assertion + gate | `grep -c 'upload-release-asset@v1' .github/workflows/release.yml` == 0 and `make check-gates` | ✅ | ⬜ pending |
| 21-03 T1 (tracer) | 21-03 | 3 | ARTIFACT-03, -04 | T-21-10, T-21-12, T-21-13 | Body rebuilt from a marker, never appended; digest from the action's own output | script (`gh` stub) | `bash tests/scripts/finalize-release-body_test.sh` | ❌ W0 — created by this task | ⬜ pending |
| 21-03 T2 | 21-03 | 3 | ARTIFACT-03 | T-21-11, T-21-14 | Literal-string truncation; failed leg's section omitted, never advertised | script (`gh` stub) | `bash tests/scripts/finalize-release-body_test.sh && make test-shell-guards` | ✅ after T1 | ⬜ pending |
| 21-04 T1 (tracer) | 21-04 | 4 | ARTIFACT-03, -05 | T-21-16, T-21-17 | Sums generated from what was actually attached; instructions only when a sums file exists | script (`gh` stub) | `bash tests/scripts/finalize-release-body_test.sh` | ✅ | ⬜ pending |
| 21-04 T2 | 21-04 | 4 | ARTIFACT-05 | T-21-16 | `LC_ALL=C` deterministic ordering; empty input attaches nothing | script (`gh` stub) | `bash tests/scripts/finalize-release-body_test.sh && make test-shell-guards` | ✅ | ⬜ pending |
| 21-05 T1 | 21-05 | 5 | ARTIFACT-01, -02, -05 | T-21-22, T-21-23 | The not-signed / no-attestation negative is stated explicitly | source assertion | `grep -q 'attest-build-provenance' docs/src/appendix/release-automation.md` (+ 4 sibling greps) | ✅ | ⬜ pending |
| 21-05 T2 | 21-05 | 5 | ARTIFACT-05, -06 | T-21-24 | Trigger-policy table untouched | source assertion + gate | `grep -q 'SHA256SUMS' docs/src/appendix/release-checklist.md && make check-doc-config` | ✅ | ⬜ pending |
| 21-06 T1 (checkpoint) | 21-06 | 6 | ARTIFACT-06 | T-21-25 | No prerelease version consumed without human authorisation | blocking human decision | *(none — `checkpoint:decision`)* | n/a | ⬜ pending |
| 21-06 T2 | 21-06 | 6 | ARTIFACT-06 | T-21-25, T-21-28 | Ancestor-of-`main` precondition asserted before the push; fix forward, never re-tag | live e2e | tag existence + `./scripts/check-release-consistency.sh --tag v<version> --sha $(git rev-parse HEAD)` | ✅ | ⬜ pending |
| 21-06 T3 | 21-06 | 6 | ARTIFACT-01…-06 | T-21-26, T-21-29 | Exactly two permitted evidence shapes; silence never reads as a pass | evidence-file assertion | `test -s 21-ARTIFACT-EVIDENCE.md && grep -qi 'what this does and does not prove' …` | ❌ — created by this task | ⬜ pending |

*Status: ⬜ pending · ✅ green · ❌ red · ⚠️ flaky*

---

## Wave 0 Requirements

Every MISSING reference is created by the same task that first needs it — there is no separate
Wave 0 plan, because each new harness is authored inside its own plan's tracer task before any
expansion task depends on it. The glob-driven `make test-shell-guards` loop picks each file up
with no registration step.

- [ ] `tests/scripts/extract-changelog-section_test.sh` — created by 21-01 T1 (ARTIFACT-01)
- [ ] `tests/scripts/package-release-binaries_test.sh` — created by 21-02 T1 (ARTIFACT-02, -05)
- [ ] `tests/scripts/finalize-release-body_test.sh` — created by 21-03 T1 (ARTIFACT-03, -04), extended by 21-04
- [ ] `21-ARTIFACT-EVIDENCE.md` — created by 21-06 T3 (ARTIFACT-06), in both the run and the not-run branch
- [ ] No new framework install — Phase 20's script-test pattern covers this phase

---

## Manual-Only Verifications

| Behavior | Requirement | Why Manual | Test Instructions |
|----------|-------------|------------|-------------------|
| End-to-end throwaway-tag rehearsal (assets download+verify, image pulls by digest, body matches changelog section) | ARTIFACT-06 | Requires a real tag push, live GitHub/ghcr/crates.io runs | Per CONTEXT.md D-14; evidence in `21-ARTIFACT-EVIDENCE.md` |
| aarch64 cross-build of all three binaries | ARTIFACT-02 | No Docker in dev sandbox; only the CI leg proves it | Observed on the rehearsal run's aarch64 leg |

---

## Validation Sign-Off

- [x] All tasks have `<automated>` verify or Wave 0 dependencies — the only exception is 21-06 T1, a `checkpoint:decision`, which by type has no automated verify
- [x] Sampling continuity: no 3 consecutive tasks without automated verify
- [x] Wave 0 covers all MISSING references — each harness is authored by the tracer task that first needs it
- [x] No watch-mode flags
- [x] Feedback latency < 120s — the slowest automated verify is 21-02 T1's debug `cargo build --bins --features cli,web-server` (~47s measured during research); every other command is seconds
- [x] `nyquist_compliant: true` set in frontmatter

**Approval:** seeded by planner 2026-08-31; awaiting execution
