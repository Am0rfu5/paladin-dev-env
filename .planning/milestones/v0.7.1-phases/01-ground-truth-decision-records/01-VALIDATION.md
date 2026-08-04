---
phase: 1
slug: ground-truth-decision-records
# status lifecycle: draft (seeded by plan-phase) → validated (set by validate-phase §6)
# audit-milestone §5.5 distinguishes NOT-VALIDATED (draft) from PARTIAL (validated + nyquist_compliant: false) (#2117)
status: draft
nyquist_compliant: false
wave_0_complete: false
created: 2026-07-30
---

# Phase 1 — Validation Strategy

> Per-phase validation contract for feedback sampling during execution.
>
> **Phase shape:** this phase produces documentation artifacts (a status ledger + six ADRs), not
> executable product code. "Validation" here means every factual claim must be independently
> re-checkable against the shipped `release/v0.7.0` tree by a third party, using the exact command
> recorded alongside the claim. Source: `01-RESEARCH.md` § Validation Architecture.

---

## Test Infrastructure

| Property | Value |
|----------|-------|
| **Framework** | None (documentation phase) — validation is citation re-verification, not automated tests |
| **Config file** | none — n/a for a documentation phase |
| **Quick run command** | `grep -n "<citation text>" <cited file>` — spot-check any single ADR/ledger claim |
| **Full suite command** | Re-run every `grep`/`Read` citation check listed in `01-RESEARCH.md` § Code Examples, plus `cargo llvm-cov --workspace --features integration-tests --summary-only` for RECON-07 |
| **Estimated runtime** | ~30 seconds for citation checks; coverage run is minutes and needs crates.io access |

---

## Sampling Rate

- **After every task commit:** re-run the specific citation's `grep`/`sed` command for every claim written by that task
- **After every plan wave:** re-run the full citation-check list from `01-RESEARCH.md` § Code Examples
- **Before `/gsd-verify-work`:** every ADR's `## Code Locations` claim re-verified once more; the RECON-07 coverage number confirmed reproducible from the exact command recorded in its ADR
- **Max feedback latency:** ~30 seconds (single citation re-check)

---

## Per-Task Verification Map

Task IDs are assigned when PLAN.md files are written. Seeded here at requirement granularity from
`01-RESEARCH.md` § Phase Requirements → Test Map; `/gsd-validate-phase` expands these rows to
per-task granularity once plans exist.

| Task ID | Plan | Wave | Requirement | Threat Ref | Secure Behavior | Test Type | Automated Command | File Exists | Status |
|---------|------|------|-------------|------------|-----------------|-----------|-------------------|-------------|--------|
| TBD (plan-assigned) | TBD | TBD | RECON-01 | — | N/A (docs) | citation-check | `grep -n "^- \[ \]" .project/Milestone_1-MVP/**/tasks-*.md` across the 8 source files | ✅ | ⬜ pending |
| TBD (plan-assigned) | TBD | TBD | RECON-02 | — | N/A (docs) | citation-check | `grep -n "pub struct BattalionConfig" -A 20 crates/paladin-core/src/platform/container/battalion/mod.rs crates/paladin-core/src/platform/container/citadel.rs` | ✅ | ⬜ pending |
| TBD (plan-assigned) | TBD | TBD | RECON-03 | — | N/A (docs) | citation-check | `grep -n "pub struct BattalionResult" -A 20 crates/paladin-core/src/platform/container/battalion/mod.rs` | ✅ | ⬜ pending |
| TBD (plan-assigned) | TBD | TBD | RECON-04 | — | N/A (docs) | citation-check | Formation/Commander citations per `01-RESEARCH.md` § Code Examples | ✅ | ⬜ pending |
| TBD (plan-assigned) | TBD | TBD | RECON-05 | — | N/A (docs) | citation-check | `sed -n '740,775p' crates/paladin-ports/src/output/llm_port.rs` | ✅ | ⬜ pending |
| TBD (plan-assigned) | TBD | TBD | RECON-06 | — | N/A (docs) | citation-check | `sed -n '49,153p' crates/paladin-core/src/platform/container/herald.rs` | ✅ | ⬜ pending |
| TBD (plan-assigned) | TBD | TBD | RECON-07 | — | N/A (docs) | command execution | `cargo llvm-cov --workspace --features integration-tests --summary-only` | ❌ W0 | ⬜ pending |
| TBD (plan-assigned) | TBD | TBD | RECON-08 | — | N/A (docs) | citation-check + local search | `ls .project/Milestone_1-MVP/Epic_10/`; grep `docs/` for documentation-review artifacts | ⚠️ W0 | ⬜ pending |

*Status: ⬜ pending · ✅ green · ❌ red · ⚠️ flaky*

---

## Wave 0 Requirements

- [ ] `cargo-llvm-cov` available on a machine with crates.io access (local, CI trigger, or human-run) — blocks RECON-07's fresh coverage measurement
- [ ] Exhaustive local search of `.project/Milestone_1-MVP/Epic_10/` and `docs/` for "Final Documentation Review" content — blocks a confident RECON-08 verdict (the conflict is located at `INGEST-CONFLICTS.md:125-127`; the resolution direction is not yet established)
- [ ] Read the literal item text (not just the count) of all 39 open checkboxes across the 8 task files listed in `01-RESEARCH.md` § Code Examples — needed before RECON-01's ledger rows can carry accurate task-item descriptions

---

## Manual-Only Verifications

| Behavior | Requirement | Why Manual | Test Instructions |
|----------|-------------|------------|-------------------|
| Every ledger verdict carries a resolvable `file:line` citation | RECON-01 | No automated linker exists for prose→code citations in this repo | For each ledger row, run the recorded `grep -n`/`sed -n` command and confirm the cited line still contains the quoted text |
| Each ADR names exactly one chosen variant and the shipped code it was checked against | RECON-02…RECON-06 | Requires human judgement that the chosen variant matches the cited evidence | Open each ADR; confirm `## Decision` names one variant and every `## Code Locations` entry resolves |
| Coverage question has one number and one scope | RECON-07 | The scope choice (plain `--workspace` vs `--features integration-tests`) is a decision, not a measurement | Confirm the ADR records both the number and the exact command that produced it |

---

## Validation Sign-Off

- [ ] All tasks have `<automated>` verify (citation command) or Wave 0 dependencies
- [ ] Sampling continuity: no 3 consecutive tasks without a recorded verification command
- [ ] Wave 0 covers all MISSING references (llvm-cov availability, Epic 10 search, 39-checkbox read)
- [ ] No watch-mode flags
- [ ] Feedback latency < 30s for citation checks
- [ ] `nyquist_compliant: true` set in frontmatter

**Approval:** pending
