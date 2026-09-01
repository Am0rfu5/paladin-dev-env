---
phase: 18
slug: rust-sast-evaluate-and-adopt-codeql
# status lifecycle: draft (seeded by plan-phase) → validated (set by validate-phase §6)
# audit-milestone §5.5 distinguishes NOT-VALIDATED (draft) from PARTIAL (validated + nyquist_compliant: false) (#2117)
status: draft
nyquist_compliant: false
wave_0_complete: false
created: 2026-08-25
---

# Phase 18 — Validation Strategy

> Per-phase validation contract for feedback sampling during execution.

**Domain note (read first).** This phase's verification unit is **a CI workflow run's measured
outcome** — alert count, analysed-file count, wall-clock — not a `cargo test` assertion. No
application code is being added to the workspace, so the usual unit-test sampling model applies
only to the guard scripts. This is recorded plainly rather than forced into a test-shaped frame it
does not fit.

---

## Test Infrastructure

| Property | Value |
|----------|-------|
| **Framework** | None in the traditional sense. Verification surface = CodeQL run outcomes (SARIF, code-scanning alerts, `src.zip` debug artifact) + existing guard scripts + documentation content |
| **Config file** | `.github/workflows/codeql.yml` is itself the "test config" (does not exist yet — Wave 0) |
| **Quick run command** | `bash scripts/check-workflow-triggers.sh` (offline, seconds) |
| **Full suite command** | `pre-commit run --all-files` plus a real dispatched/pushed CodeQL run read for its actual outcome |
| **Estimated runtime** | Guard scripts ~seconds; a CodeQL Rust run on this tree is **unmeasured** — establishing that number is itself a deliverable (SAST-03 / D-15) |

---

## Sampling Rate

- **After every task commit:** `bash scripts/check-workflow-triggers.sh` once `codeql.yml` exists
  (it is the only automated assertion that directly covers this phase's wiring), plus
  `pre-commit run --all-files` for any commit touching the probe fixture.
- **After every plan wave:** a real CodeQL run — dispatched or triggered by a pushed branch — read
  for its actual outcome, never inferred from a green checkmark.
- **Before `/gsd-verify-work`:** the D-16 evidence document complete and committed, and
  `scripts/check-workflow-triggers.sh` passing.
- **Max feedback latency:** guard scripts < 30s; CodeQL run latency to be measured (D-15).

---

## Per-Task Verification Map

*Task IDs are assigned by the planner; this table is seeded `draft` and completed by
`/gsd-validate-phase` once PLAN.md files exist. Requirement-level mapping below is final.*

| Task ID | Plan | Wave | Requirement | Threat Ref | Secure Behavior | Test Type | Automated Command | File Exists | Status |
|---------|------|------|-------------|------------|-----------------|-----------|-------------------|-------------|--------|
| TBD | TBD | 0 | SAST-01 | — | Probe finds, or provably fails to find, all 4 planted defect classes; count recorded either way | manual (SARIF/alerts read by a human or agent) | `gh run view <run-id>` + code-scanning alerts API for the probe branch | ❌ W0 — probe fixture does not exist | ⬜ pending |
| TBD | TBD | 1 | SAST-02 | — | Workflow triggers on `pull_request` (no path filter), `push: ['**']`, `schedule`, `workflow_dispatch`; every pinned context resolves | automated | `bash scripts/check-workflow-triggers.sh` | ✓ script exists / ❌ W0 workflow file | ⬜ pending |
| TBD | TBD | 1 | SAST-01 / D-13 | — | Analysed-file count is produced and recorded against the 385 denominator | automated-ish | `debug: true` on `codeql-action/init` → download `src.zip` artifact, count `.rs` entries | ❌ W0 | ⬜ pending |
| TBD | TBD | 2 | SAST-03 | — | FP rate + wall-clock measured over a recorded window before any gate | manual (observation-window procedure) | D-14/D-15 procedure defined by the plan | ❌ W0 | ⬜ pending |
| TBD | TBD | 3 | SAST-03 / D-19 | — | Promotion updates all four places in one change; no place disagrees | automated + review | `bash scripts/check-workflow-triggers.sh` (Clause 3) + `grep -c '\b44\b\|\b45\b' docs/src/appendix/branch-protection.md` | ✓ targets exist | ⬜ pending |
| TBD | TBD | 3 | SAST-04 | — | "Known gap" section matches the measured outcome; no doc claims uncovered coverage | manual review | `grep -n "taint analysis\|Known gap" .github/instructions/security.instructions.md` | ✓ target exists | ⬜ pending |

*Status: ⬜ pending · ✅ green · ❌ red · ⚠️ flaky*

---

## Wave 0 Requirements

- [ ] Probe fixture crate — must live **outside `crates/*`** (that glob auto-includes anything
      placed there) and be excluded from the workspace, per research finding.
- [ ] `.github/workflows/codeql.yml` — plus its row in `docs/src/contributing/branching-model.md`'s
      trigger-policy table **in the same commit** (Clause 1 fails a workflow with no row).
- [ ] Alert-triage register (D-17) — new file or section, planner's call.
- [ ] Evidence document under `.planning/` (D-16).
- [ ] A `.gitleaks.toml` allowlist entry for the planted credential, carrying a stated reason —
      not a `--no-verify` bypass (D-10).

*No `cargo test`-shaped gap exists: this phase adds no application code to the workspace.*

---

## Manual-Only Verifications

| Behavior | Requirement | Why Manual | Test Instructions |
|----------|-------------|------------|-------------------|
| Probe finding count and its comparability to the Snyk baseline | SAST-01 | The judgement "did the scanner actually analyse Rust" cannot be asserted by a test; it requires reading the SARIF against the known-planted defects | Run the probe branch; list code-scanning alerts; compare against the 4 planted classes and against Snyk's recorded 0-in-Rust / 3-in-JavaScript |
| False-positive rate | SAST-03 | Requires human triage of each alert as true/false positive | Follow the D-14 backfill + live-window procedure; record per D-15 |
| Feature-gated coverage (the phase's central open risk) | SAST-01 / D-12 | **No official documentation answers whether buildless Rust extraction reaches non-default-feature code.** Must be established empirically | Plant a 5th defect behind a non-default feature; observe whether it is reported |
| "Known gap" section accuracy | SAST-04 | Documentation-content judgement | Read the rewritten section against the recorded evidence; confirm it neither overstates coverage nor deletes an unresolved gap |

---

## Validation Sign-Off

- [ ] All tasks have `<automated>` verify or Wave 0 dependencies
- [ ] Sampling continuity: no 3 consecutive tasks without automated verify
- [ ] Wave 0 covers all MISSING references
- [ ] No watch-mode flags
- [ ] Feedback latency < 30s for guard scripts; CodeQL run latency measured, not assumed
- [ ] `nyquist_compliant: true` set in frontmatter

**Approval:** pending
