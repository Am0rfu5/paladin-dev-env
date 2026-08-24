---
phase: 9
slug: release-security-gate-integrity
# status lifecycle: draft (seeded by plan-phase) → validated (set by validate-phase §6)
# audit-milestone §5.5 distinguishes NOT-VALIDATED (draft) from PARTIAL (validated + nyquist_compliant: false) (#2117)
status: draft
nyquist_compliant: false
wave_0_complete: false
created: 2026-08-07
---

# Phase 9 — Validation Strategy

> Per-phase validation contract for feedback sampling during execution.
> Derived from `09-RESEARCH.md` §"Validation Architecture".
>
> **This phase changes no `.rs` source.** No unit/integration test framework applies to its
> deliverables. Validation is therefore *guard-script* driven: each SEC requirement closes with a
> shell command against a config or register file. The standing `cargo` gate still runs, and is
> expected to be **unchanged** — proving that is itself a phase deliverable.

---

## Test Infrastructure

| Property | Value |
|----------|-------|
| **Framework** | None applicable — no `.rs` changes. Guards are POSIX shell + `python3` 3.11.2 stdlib `tomllib` (confirmed present, zero new dependencies) |
| **Config file** | N/A |
| **Quick run command** | `./scripts/check-advisory-register.sh && ./scripts/check-crate-names.sh && ./scripts/check-changelogs.sh` |
| **Full suite command** | `cargo test --offline --workspace && cargo fmt --check && cargo clippy --offline -- -D warnings` |
| **Estimated runtime** | Guards ~2s; full `cargo` gate several minutes (cold) |

---

## Sampling Rate

- **After every task commit:** Run the guard script(s) that task touches. For the eleven manifest
  edits (D-11), run `cargo check --offline --workspace` to confirm the `license` field still parses.
- **After every plan wave:** Run all three guards together, plus a `grep`-based re-verification of
  every count in `09-RESEARCH.md` §3 (dead suppressions, live suppressions, published-name list) to
  confirm no drift was introduced mid-phase.
- **Before `/gsd-verify-work`:** Full `cargo` gate green **and** the close-out's honest statement of
  which SEC criteria were verified locally vs. which require a CI runner (D-19).
- **Max feedback latency:** ~5 seconds for guards; the `cargo` gate is a wave-boundary check, not a
  per-task one.

---

## Per-Task Verification Map

> Task IDs are placeholders until `gsd-planner` assigns them; the requirement → command mapping
> below is the contract each task must land on.

| Task ID | Plan | Wave | Requirement | Threat Ref | Secure Behavior | Test Type | Automated Command | File Exists | Status |
|---------|------|------|-------------|------------|-----------------|-----------|-------------------|-------------|--------|
| 09-XX-XX | TBD | 1 | SEC-01 | — | Advisory suppression sets cannot silently diverge across `deny.toml`, `.cargo/audit.toml` and the register | script | `./scripts/check-advisory-register.sh` | ❌ W0 | ⬜ pending |
| 09-XX-XX | TBD | 1 | SEC-01 | — | Every live suppression carries owner + expiry + scope + compensating control | script | `./scripts/check-advisory-register.sh` (register-coverage clause) | ❌ W0 | ⬜ pending |
| 09-XX-XX | TBD | 1 | SEC-01 | — | No suppression outlives the dependency it suppresses | script | `./scripts/check-advisory-register.sh` (`Cargo.lock` liveness clause) | ❌ W0 | ⬜ pending |
| 09-XX-XX | TBD | 2 | SEC-01 | — | Exactly one `cargo audit` invocation in CI; no inline `--ignore` | source assertion | `grep -c '^  security:$' .github/workflows/ci.yml` → `0`; `grep -c 'cargo audit --ignore' .github/workflows/ci.yml` → `0` | ✅ | ⬜ pending |
| 09-XX-XX | TBD | 2 | SEC-02 | — | One licence expression across root + ten crates | source assertion | `grep -h '^license = ' Cargo.toml crates/*/Cargo.toml \| sort -u \| wc -l` → `1` | ✅ | ⬜ pending |
| 09-XX-XX | TBD | 2 | SEC-02 | — | `deny.toml` allows the declared expression; manifests still parse | build | `cargo check --offline --workspace` exits 0 | ✅ | ⬜ pending |
| 09-XX-XX | TBD | 1 | SEC-03 | — | An unlisted package name fails the guard | script (negative path) | `./scripts/check-crate-names.sh` with a bogus name added → non-zero exit, then revert | ❌ W0 | ⬜ pending |
| 09-XX-XX | TBD | 1 | SEC-04 | — | Herald changelog exists | file existence | `test -f crates/paladin-herald/CHANGELOG.md` | ✅ | ⬜ pending |
| 09-XX-XX | TBD | 1 | SEC-04 | — | A crate without a changelog fails the guard | script (negative path) | `./scripts/check-changelogs.sh` with one changelog temporarily renamed → non-zero exit, then revert | ❌ W0 | ⬜ pending |
| 09-XX-XX | TBD | 1 | SEC-05 | — | Planner stage carries no per-crate manifest enumeration to go stale | source assertion | `grep -c 'COPY crates/paladin.*Cargo.toml' Dockerfile.chef` → `0` | ✅ | ⬜ pending |
| 09-XX-XX | TBD | 3 | all | — | The `.rs` tree is untouched and the gate is unmoved | build + coverage | `git diff --stat <phase-base>..HEAD -- '*.rs'` empty; `cargo test --offline --workspace` pass count unchanged | ✅ | ⬜ pending |

*Status: ⬜ pending · ✅ green · ❌ red · ⚠️ flaky*

---

## Wave 0 Requirements

- [ ] `scripts/check-advisory-register.sh` — does not exist; written by SEC-01's plan (D-02)
- [ ] `scripts/check-crate-names.sh` — does not exist; written by SEC-03's plan (D-13)
- [ ] `scripts/check-changelogs.sh` — does not exist; written by SEC-04's plan (D-15)
- [ ] A demonstrated **negative path** for each of the three guards above

**The negative path is not optional.** Phase 8 (D-05) found `scripts/check-deprecations.sh` had
both branches `exit 0` — it presented as a gate and could not fail. Every guard this phase adds must
be recorded failing as well as passing, with the exact invocation and exit code captured in the
plan's SUMMARY. A guard that has only ever been observed passing has not been validated.

---

## Manual-Only Verifications

| Behavior | Requirement | Why Manual | Test Instructions |
|----------|-------------|------------|-------------------|
| `cargo audit` and `cargo deny check` actually pass against the reconciled configuration | SEC-01 | Neither tool is installed and neither can be installed — `crates.io` returns HTTP 403 in this environment | Read the surviving `security-audit` job's run on the first CI execution after merge; confirm it exits 0 against `.cargo/audit.toml`'s five entries |
| `cargo chef cook` reports `CACHED` when only `.rs` changes | SEC-05 | Docker is absent from this environment (recorded at `.planning/phases/04-release-coherence/04-ci-gate-deferrals.md`) | Build `Dockerfile.chef` twice with only a `.rs` edit between runs; confirm the `cargo chef cook` layer is `CACHED`. **Until then, D-16 rests on cargo-chef's documented recipe semantics, not on measurement — the close-out must say so** |
| crates.io accepts the new `license` expression | SEC-02 | No publish happens in this phase | Deferred to the next real release cycle; the guard here is `cargo check` parsing, not registry acceptance |
| The `Security Audit` required status check still resolves after the duplicate job is deleted | SEC-01 | Branch protection is evaluated by GitHub, not locally | `09-RESEARCH.md` established that `.github/rulesets/protect-main-branch.json:39` requires the **context string** `"Security Audit"`, which the surviving `security-audit` job also posts — so the risk is assessed as zero. Confirm on the first post-merge run |

---

## Validation Sign-Off

- [ ] All tasks have an automated verify command or a Wave 0 dependency
- [ ] Sampling continuity: no 3 consecutive tasks without an automated verify
- [ ] Wave 0 covers all three MISSING guard scripts **and their negative paths**
- [ ] No watch-mode flags
- [ ] Feedback latency < 5s for guards
- [ ] Every "manual-only" row above is restated in the close-out as an explicitly *unverified-here*
      claim, per D-19 — never inferred as passing
- [ ] `nyquist_compliant: true` set in frontmatter

**Approval:** pending
