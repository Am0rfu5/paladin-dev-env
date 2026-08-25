---
phase: 11
slug: facade-residue-deferred-register-disposition
# status lifecycle: draft (seeded by plan-phase) → validated (set by validate-phase §6)
# audit-milestone §5.5 distinguishes NOT-VALIDATED (draft) from PARTIAL (validated + nyquist_compliant: false) (#2117)
status: draft
nyquist_compliant: false
wave_0_complete: false
created: 2026-08-08
---

# Phase 11 — Validation Strategy

> Per-phase validation contract for feedback sampling during execution.
>
> **Domain note (from 11-RESEARCH.md § Validation Architecture):** this phase's validation applies to
> **records**, not code. A "test" is a shell command that proves a citation resolves, a row count
> matches, or an annotation exists at a named path — the same shape as Phase 10's precedent. No Rust
> is written, so no Rust test harness is added.

---

## Test Infrastructure

| Property | Value |
|----------|-------|
| **Framework** | None — direct shell verification (`grep`, `ls`, `find`, `git log`, `git show`) |
| **Config file** | none — no framework to install |
| **Quick run command** | Per-claim `grep -n "<pattern>" <file>` or `[ -e <path> ] && echo EXISTS` |
| **Full suite command** | Re-run every command in the Per-Task Verification Map below, confirming no drift since 2026-08-08 |
| **Estimated runtime** | ~5 seconds (grep/ls over a working tree; no compilation) |

---

## Sampling Rate

- **After every task commit:** Re-run the specific `grep`/`ls`/`git log` command that the task's own
  ADR row or register row cites, before marking that disposition's evidence cell complete.
- **After every plan wave:** Re-run the FACADE-01 17-occurrence count and the FACADE-04 20-row
  existence table against the in-progress register file, to confirm no row was silently dropped or
  miscounted during parallel fan-out.
- **Before `/gsd-verify-work`:** Full suite green — re-run this document's map verbatim, since it
  measures a mutable working tree rather than a fixed historical document.
- **Max feedback latency:** 10 seconds

---

## Per-Task Verification Map

Task IDs are assigned by `/gsd-plan-phase` when PLAN.md files are written; this seed maps
**requirement → automated command**, and the planner binds each row to the task that satisfies it.

| Task ID | Plan | Wave | Requirement | Threat Ref | Secure Behavior | Test Type | Automated Command | File Exists | Status |
|---------|------|------|-------------|------------|-----------------|-----------|-------------------|-------------|--------|
| TBD | TBD | TBD | FACADE-01 | — | N/A | grep-count | `grep -rn "println!\|eprintln!\|dbg!" src/application/services/ src/infrastructure/ \| wc -l` → 17 | ✅ | ⬜ pending |
| TBD | TBD | TBD | FACADE-01 | — | N/A | grep-filter | `grep -rn "println!\|eprintln!\|dbg!" src/application/services/ src/infrastructure/ \| grep -v '///' \| grep -v '//!'` → 0 | ✅ | ⬜ pending |
| TBD | TBD | TBD | FACADE-02 (D1) | — | N/A | find + grep-count | `find src/core -name "*.rs" \| wc -l` → 6; `grep -rl "crate::core::" src/ \| wc -l` → 49 | ✅ | ⬜ pending |
| TBD | TBD | TBD | FACADE-02 (D2) | — | N/A | file-exists | `ls src/core/platform/manager/` lists the three files D2 names | ✅ | ⬜ pending |
| TBD | TBD | TBD | FACADE-02 (D1–D4) | — | N/A | grep | Each of D1–D4 has a decision verb (`do`/`defer`/`withdraw`) + named owner in the register — not a rating | ❌ W0 | ⬜ pending |
| TBD | TBD | TBD | FACADE-03 | — | N/A | git-show | `git log -1 --format="%H %ai %s" 3d48768` resolves; both removals in that one commit | ✅ | ⬜ pending |
| TBD | TBD | TBD | FACADE-03 | — | N/A | find | `find crates/paladin-ports -iname "*ml_port*"` → `crates/paladin-ports/src/input/ml_port.rs` | ✅ | ⬜ pending |
| TBD | TBD | TBD | FACADE-03 | — | N/A | grep | The `paladin user register` answer is reachable from `.planning/` — 1,065 LOC declared-never-dispatched + named restore commit | ❌ W0 | ⬜ pending |
| TBD | TBD | TBD | FACADE-04 | — | N/A | file-exists (×20) | Each of the 20 candidate rows resolves to done / not-a-candidate / still-open with a live-tree check | ❌ W0 | ⬜ pending |
| TBD | TBD | TBD | FACADE-04 | — | N/A | ls | `ls crates/` → 11 entries; neither `paladin-arsenal` nor `paladin-sanctum` present | ✅ | ⬜ pending |
| TBD | TBD | TBD | cross-cutting | — | N/A | grep | `grep -n "Next free ADR number" .planning/decisions/PROMOTION.md` → matches the number this phase's new ADRs claim | ✅ | ⬜ pending |

*Status: ⬜ pending · ✅ green · ❌ red · ⚠️ flaky*
*File Exists: ✅ = the command's target exists today; ❌ W0 = the target is an artifact this phase creates.*

---

## Wave 0 Requirements

- [ ] No test framework to install — `grep`/`ls`/`find`/`git` are already available in this environment.
- [ ] The `❌ W0` rows above target **records this phase authors** (the disposition register, the
      FACADE-03 answer, the FACADE-04 triage). Their commands become runnable as soon as the
      corresponding artifact is written; they are not missing infrastructure.

*Existing infrastructure covers all phase requirements — there is no test suite to scaffold.*

---

## Manual-Only Verifications

| Behavior | Requirement | Why Manual | Test Instructions |
|----------|-------------|------------|-------------------|
| Each D1–D4 disposition is a *decision*, not a restated rating | FACADE-02 | Judgment: a grep can confirm a verb is present but not that the reasoning is sound | Read each disposition; confirm it names an owner and, if deferred, a concrete trigger condition |
| "Nothing is planned twice" across D2 / run-3 v2 / Deferred-QA Epic 28 | FACADE-02 | Cross-document reasoning over three separate registers | Read D2, the run-3 v2 item, and Epic 28; confirm the register states which owns the split and which owns the tests |
| The ML reintroduction condition survives outside a single DOC | FACADE-03 | Durability is a property of where the statement lives, not of its text | Confirm the `paladin-ml` leaf-crate + feature-flag-on-that-crate condition is recorded in a promoted decision, not only in a doc file |

---

## Validation Sign-Off

- [ ] All tasks have `<automated>` verify or Wave 0 dependencies
- [ ] Sampling continuity: no 3 consecutive tasks without automated verify
- [ ] Wave 0 covers all MISSING references
- [ ] No watch-mode flags
- [ ] Feedback latency < 10s
- [ ] `nyquist_compliant: true` set in frontmatter

**Approval:** pending
