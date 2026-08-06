---
phase: 7
slug: workspace-ground-truth-recorded-answers
# status lifecycle: draft (seeded by plan-phase) → validated (set by validate-phase §6)
# audit-milestone §5.5 distinguishes NOT-VALIDATED (draft) from PARTIAL (validated + nyquist_compliant: false) (#2117)
status: draft
nyquist_compliant: false
wave_0_complete: false
created: 2026-08-06
---

# Phase 7 — Validation Strategy

> Per-phase validation contract for feedback sampling during execution.

**This is a records phase.** It writes markdown into `.planning/` and annotates historical
documents under `.project/`; it changes no product code. "Validation" therefore means *how a later
reader or verifier confirms a ledger row is honest, an ADR is well-formed, and a `.project/`
annotation is present* — not a test suite. Every check below is a `grep`/`sed`/`git diff`
one-liner against files this phase itself creates, so there is no Wave 0 test-infrastructure debt.

Source: `07-RESEARCH.md` § Validation Architecture.

---

## Test Infrastructure

| Property | Value |
|----------|-------|
| **Framework** | none — `grep`/`sed`/`git` assertions over markdown artefacts |
| **Config file** | none required |
| **Quick run command** | `grep -c '^\| REQ-' .planning/ledgers/milestone-04-06.md` |
| **Full suite command** | the greppable-checks table below, run in order |
| **Estimated runtime** | < 5 seconds |

**Product-code guard.** The workspace test suite (`cargo test`) is *not* a validation instrument
for this phase — nothing it exercises can change. It is instead a **negative** control: the phase
boundary requires `git diff --stat -- '*.rs' 'Cargo.toml' '.github/'` to stay empty across the
whole phase. A non-empty diff there is a boundary violation, not a test failure.

---

## Sampling Rate

- **After every task commit:** the plan's own automated `<verify>` block — row counts, heading
  counts, duplicate-ID check. Same shape as Phase 5's `05-01-PLAN.md`, reusable near-verbatim.
- **After every plan wave:** re-run the row-count and duplicate-ID checks against the ledger, so a
  fan-out plan that clobbers a sibling's rows is caught at the wave boundary rather than at close.
- **Before `/gsd-verify-work`:** the full greppable-checks table must pass.
- **Max feedback latency:** ~5 seconds.

---

## Per-Task Verification Map

Populated by the planner from the final PLAN.md set. Every task in this phase produces or edits a
markdown artefact, so each row's automated command is an assertion over that artefact.

| Task ID | Plan | Wave | Requirement | Threat Ref | Secure Behavior | Test Type | Automated Command | File Exists | Status |
|---------|------|------|-------------|------------|-----------------|-----------|-------------------|-------------|--------|
| 07-01-01 | 01 | 1 | ARCH-01 | — | N/A | artefact | `test $(grep -c '^\| REQ-' .planning/ledgers/milestone-04-06.md) -eq 115` | ❌ W0 | ⬜ pending |

*Status: ⬜ pending · ✅ green · ❌ red · ⚠️ flaky*

---

## Phase-Gate Checks (run at close)

| Check | Command | What it confirms |
|-------|---------|-------------------|
| Ledger row count matches requirement count | `grep -c '^\| REQ-' .planning/ledgers/milestone-04-06.md` equals `115` | No row silently dropped |
| No duplicate `REQ-*` IDs | `grep -o '^\| REQ-[a-z0-9-]*' .planning/ledgers/milestone-04-06.md \| sort \| uniq -d` prints nothing | Primary-key integrity (D-00e) |
| No stub rows survive | `grep -c 'PENDING-VERDICT' .planning/ledgers/milestone-04-06.md` equals `0` | All 115 rows actually verdicted |
| Every ADR carries a Conformance verdict | `grep -L 'conforms\|must change' .planning/decisions/00{14,15,16,17,18,19,20}*.md` prints nothing | D-00c compliance |
| Every ADR uses the required heading set | each of the new ADRs shows the 7 canonical `## ` headings | `adr-parser.cjs` compatibility (D-00h) |
| `PROMOTION.md` next-free line advanced | `grep -c 'Next free ADR number: 002[12]' .planning/decisions/PROMOTION.md` equals `1` | D-25 / D-25a bookkeeping complete |
| REQUIREMENTS.md's old ledger section reduced to a pointer | that section contains zero `^\| REQ-` rows | D-26 complete, no diverging second copy |
| `.project/` corrections are additive | `git diff --stat` over the correction commits shows insertions, no large deletion blocks | D-00g provenance preservation |
| **No product code touched** | `git diff --stat <phase-range> -- '*.rs' 'Cargo.toml' '.github/'` is empty | The phase's own hard boundary |
| `STRUCTURE.md` correction lands in the prose section | the "Directory Purposes" section lists all ten library crates plus `doc-examples` | D-05 targets prose, not just the ASCII tree |

---

## Wave 0 Requirements

*Existing infrastructure covers all phase requirements.* No framework install, no fixture, no test
file — every check is a shell one-liner over artefacts the phase creates. The only ordering
constraint is that the ledger scaffold must exist before any row-count assertion can pass, which
the plan ordering already guarantees.

---

## Manual-Only Verifications

| Behavior | Requirement | Why Manual | Test Instructions |
|----------|-------------|------------|-------------------|
| A ledger verdict is *honest* — the cited `file:line` actually supports the verdict | ARCH-01 | No mechanical check can confirm that a citation supports the claim it is attached to; only reading both can | Spot-check a random sample of rows per fan-out plan: open the cited `file:line` and confirm it states what the row says it states. Phase 5 precedent: sample rather than re-verify all. |
| An ADR's recorded answer matches the shipped code it cites | ARCH-03, ARCH-04, ARCH-06 | Same reason — citation-supports-claim is a reading task | For each of ADR-0014…0020, open every path in `## Code Locations` and confirm it exists and says what the ADR claims |
| The ≥50% restatement is faithful to the source report | ARCH-07 | Transcription fidelity | Diff ADR-0020's five figures against `build-benchmarks.md`'s summary table |

---

## Validation Sign-Off

- [ ] All tasks have an automated `<verify>` block or a named manual verification above
- [ ] Sampling continuity: no 3 consecutive tasks without an automated verify
- [ ] Wave 0 covers all MISSING references *(N/A — none)*
- [ ] No watch-mode flags
- [ ] Feedback latency < 5s
- [ ] `nyquist_compliant: true` set in frontmatter

**Approval:** pending
