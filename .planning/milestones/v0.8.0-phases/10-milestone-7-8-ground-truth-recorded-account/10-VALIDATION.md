---
phase: 10
slug: milestone-7-8-ground-truth-recorded-account
# status lifecycle: draft (seeded by plan-phase) → validated (set by validate-phase §6)
# audit-milestone §5.5 distinguishes NOT-VALIDATED (draft) from PARTIAL (validated + nyquist_compliant: false) (#2117)
status: draft
nyquist_compliant: false
wave_0_complete: false
created: 2026-08-08
---

# Phase 10 — Validation Strategy

> Per-phase validation contract for feedback sampling during execution.

**This phase validates records, not code.** A "test" here is a shell command that proves a citation
resolves, a row count matches, or an annotation banner exists at a named path. Source:
`10-RESEARCH.md` §"Validation Architecture".

---

## Test Infrastructure

| Property | Value |
|----------|-------|
| **Framework** | None — direct shell verification (`grep`, `sed -n`, `awk`, `git log`, `cargo doc`, `cargo test --doc`) |
| **Config file** | none — no test framework is installed or needed |
| **Quick run command** | Per-claim `grep -n "<pattern>" <file>` / `sed -n '<range>p' <file>` — the exact command the ledger row or ADR cites |
| **Full suite command** | Re-run every row of the Per-Task Verification Map below (the close-out plan owns this) |
| **Estimated runtime** | ~2 seconds for the grep/sed rows; ~90 seconds if the two `cargo` rows are included |

---

## Sampling Rate

- **After every task commit:** Re-run the specific `grep`/`sed`/`git log` command that task's ledger
  row or ADR cites, before marking its evidence cell complete. This is D-00e's bar applied per row.
- **After every plan wave:** Re-run the 86-row count and the 13-row supersession count against the
  in-progress ledger, to confirm the parallel Wave 2 fan-out dropped or duplicated nothing.
- **Before `/gsd-verify-work`:** Re-run this whole table verbatim. Two rows (`cargo doc` and the
  doctest behaviour) are facts about a mutable working tree, not about fixed historical documents.
- **Max feedback latency:** 2 seconds for record assertions; ~90 seconds when a `cargo` row runs.

---

## Per-Task Verification Map

| Task ID | Plan | Wave | Requirement | Threat Ref | Secure Behavior | Test Type | Automated Command | File Exists | Status |
|---------|------|------|-------------|------------|-----------------|-----------|-------------------|-------------|--------|
| 10-01-* | 01 | 1 | HARD-01 | — | N/A | row-count | `grep -c '^\| REQ-' .planning/ledgers/milestone-07-08.md` → `86` | ✅ | ⬜ pending |
| 10-01-* | 01 | 1 | HARD-01 | — | N/A | row-count | `sed -n '365,381p' .planning/intel/code-verification.md \| grep -c '^\| '` → `14` (1 header + **13** data rows, per D-05) | ✅ | ⬜ pending |
| 10-02..05-* | 02-05 | 2 | HARD-01 | — | N/A | citation-resolves | For each row: the cited `file:line` is re-read and the quoted text still matches | ✅ | ⬜ pending |
| 10-06-* | 06 | 3 | HARD-02 | — | N/A | file-exists + grep | `find .project -iname '*RECONCILIATION*' -o -iname '*facade-audit*' -o -iname '*infrastructure-adapter-disposition*'` → all three; each carries a dated D-00c banner after the task | ✅ | ⬜ pending |
| 10-06-* | 06 | 3 | HARD-02 | — | N/A | grep | `grep -rn "use_cases" src/ crates/ tests/ examples/ benches/ --include='*.rs'` → **zero** (Epic 6 complete) | ✅ | ⬜ pending |
| 10-07-* | 07 | 3 | HARD-03 | — | N/A | grep + git | `grep -n '^version' Cargo.toml` → `0.7.0`; `git tag --sort=-v:refname \| head -2` → `v0.7.1`, `v0.7.0` | ✅ | ⬜ pending |
| 10-07-* | 07 | 3 | HARD-03 | — | N/A | grep | `grep -n 'REL-01' .planning/REQUIREMENTS.md \| head -1` shows `[x]` — REL-01 is not re-opened | ✅ | ⬜ pending |
| 10-07-* | 07 | 3 | HARD-04 | — | N/A | grep | The M7 overview's self-title line carries a dated banner pointing at ADR-0030 | ✅ | ⬜ pending |
| 10-08-* | 08 | 3 | HARD-05 | — | N/A | manifest-grep | `grep -n 'paladin-llm' crates/paladin-content/Cargo.toml` → optional, non-default; `grep -rn 'cfg(feature = "llm")' crates/paladin-content/src/` → gated | ✅ | ⬜ pending |
| 10-08-* | 08 | 3 | HARD-06 | — | N/A | manifest-grep | `grep -n '^pdf' crates/paladin-content/Cargo.toml` → **zero matches** after the D-18 deletion | ✅ | ⬜ pending |
| 10-08-* | 08 | 3 | HARD-06 | — | V14 (config) | The corrected `.cargo/audit.toml` comment states the actual reachability path | grep | `sed -n '20,35p' .cargo/audit.toml` contains no `optional \`content-processing\`` claim about the `pdf` feature | ✅ | ⬜ pending |
| 10-08-* | 08 | 3 | HARD-07 | — | N/A | build-command | `cargo doc --workspace --no-deps 2>&1 \| tee /tmp/d.txt && ! grep -q "warning:" /tmp/d.txt` | ⚠️ **currently exits 1 — 20 warnings across 4 crates** (see Manual-Only, below) | ⬜ pending |
| 10-08-* | 08 | 3 | HARD-07 | — | N/A | makefile-grep | `grep -n 'exclude paladin-ports' Makefile` → **zero matches** after the D-21 deletion | ✅ | ⬜ pending |
| 10-08-* | 08 | 3 | HARD-07 | — | N/A | manifest-grep | `grep -c doctest crates/*/Cargo.toml` → the seven-crate list ADR-0033 records is accurate at write time | ✅ | ⬜ pending |
| 10-09-* | 09 | 4 | all | — | N/A | checkbox + count | HARD-01…HARD-07 are `[x]` in REQUIREMENTS.md; `grep -n 'Next free ADR number' .planning/decisions/PROMOTION.md` → `0034` | ✅ | ⬜ pending |
| 10-09-* | 09 | 4 | all | — | N/A | coverage-floor | ADR-0006's 84% workspace line-coverage floor is unmoved — expected, since no `.rs` file is modified | ⚠️ CI-only (`cargo-llvm-cov` not installable here) | ⬜ pending |

*Status: ⬜ pending · ✅ green · ❌ red · ⚠️ flaky*

---

## Wave 0 Requirements

*None — there is no test suite to scaffold.* The "tests" are the shell commands in the map above,
and every one of them already runs in this environment. One of them (`cargo doc`) currently returns
a **failing** result, which the phase must **record honestly**, not silently pass.

---

## Manual-Only Verifications

| Behavior | Requirement | Why Manual | Test Instructions |
|----------|-------------|------------|-------------------|
| The `cargo doc --workspace --no-deps` zero-warning bar actually holds | HARD-07 | **The gate fails today.** `10-RESEARCH.md` reproduced the exact CI command twice: exit 1, **20 rustdoc warnings** across `paladin-web` (13), `paladin-ai` (3), `paladin-battalion` (3), `paladin-herald` (1). Clearing them requires `.rs` doc-comment edits, which CONTEXT.md D-23 puts outside this phase's boundary. **A human must decide** whether ADR-0033 ratifies the bar and records the 20-warning debt against an owning phase, or whether D-23's boundary widens to cover the doc-comment fixes. | Run `cargo doc --workspace --no-deps 2>&1 \| tee /tmp/doc.txt; grep -c "^warning:" /tmp/doc.txt`, then choose: (a) ADR-0033 ratifies the bar and names an owner for the 20 warnings, or (b) this phase fixes them and D-23 is amended in the plan. Record which branch was taken. |
| `cargo audit` / `cargo deny check` pass against the reconciled config | HARD-06 (D-19's audit.toml edit) | Neither tool is installable here — `crates.io` returns HTTP 403 (Phase 9 D-19, unchanged) | CI-only. Land the comment correction, record the exact command a runner executes, and scope the claim honestly. Never infer a pass. |
| The ADR-0006 84% coverage floor is unmoved | cross-cutting (D-23) | `cargo-llvm-cov` is not installable in this environment (Phase 1 finding, unchanged) | CI-only. A phase that modifies no `.rs` file cannot move line coverage; state that as the reasoning rather than as a measurement. |

---

## Validation Sign-Off

- [ ] Every ledger row's evidence cell carries the exact command or `file:line` that produced it (D-00e)
- [ ] Sampling continuity: no ledger fan-out plan commits without re-running its own row count
- [ ] Wave 0 covers all MISSING references — N/A, no test infrastructure needed
- [ ] No watch-mode flags
- [ ] Feedback latency < 2s for record assertions, < 90s including the `cargo` rows
- [ ] The three Manual-Only rows are each recorded with an explicit disposition, never inferred as passing
- [ ] `nyquist_compliant: true` set in frontmatter

**Approval:** pending
