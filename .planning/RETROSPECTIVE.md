# Project Retrospective

*A living document updated after each milestone. Lessons feed forward into future planning.*

## Milestone: v0.7.1 — Milestone 1 close-out

**Shipped:** 2026-08-04
**Phases:** 4 | **Plans:** 38 | **Tasks:** 88 | **Commits:** 255

### What Was Built

- **Nine ADRs (`0001`–`0009`)** settling every contested definition in the corpus, each naming its
  chosen variant and the shipped code it was checked against. `.planning/decisions/` and
  `.planning/ledgers/` were stood up as new document classes to hold them.
- **Code changes applying those decisions** — `ProviderCapabilities.temperature_range`, Formation's
  1-Paladin minimum, the `BattalionCheckpointConfig` rename, and Formation's per-Paladin
  times/tokens/`node_errors` on `BattalionResult` rendered through all three Heralds.
- **A real UTF-8 panic fix** in `TableHerald::truncate_text` plus the two adjacent panic paths that
  shared the same defective helper.
- **An offline coverage pipeline** (`rustc -C instrument-coverage` → `llvm-profdata` → `llvm-cov`)
  that works with no `cargo-llvm-cov`, no network and no Docker — 84.79% at Phase 1, reproduced at
  85.56% / 85.92% in Phase 3.
- **62 previously dead tests activated** — 25 in `tests/unit/llm/`, 37 in `tests/cli/` — none
  deleted, plus four real Commander error-path tests replacing `#[ignore]`d stubs.
- **Release coherence** — twelve manifests on version 0.7.0 / edition 2024, advisory posture
  recorded, quickstart fixed and measured at 15 minutes, gate suite at 2,924 tests / 185 doc tests
  / 47 example targets.

### What Worked

- **Recording a decision before applying it.** Phases 1→2 ran as "ADR first, code second", and the
  audit's highest-value check — do the six ADRs match shipped code? — came back clean on all of
  them. The ADR gave the code change an unambiguous target.
- **Refusing to paper over a missing prerequisite.** Plan 01-08 declined to write a ledger row it
  could not source, and 01-04 halted at its own Task 1 precondition rather than fabricating a
  coverage number. Both were later closed properly by gap-closure plans. The halts cost time and
  bought correctness.
- **Measuring instead of restating.** The "22 runnable examples" figure had propagated through five
  documents from a single Milestone-1 validation report. Phase 4 counted the tree (47 `.rs` files,
  4 declared targets) and replaced the count with a property — "every example target builds" —
  that cannot go stale the same way.
- **Re-verification after gap closure.** Three of four phases entered verification as `gaps_found`
  and were re-verified to `passed` only after the gaps were actually closed. No phase was talked
  into passing.

### What Was Inefficient

- **The coverage question was answered three times.** Plan 01-04 halted on it, 01-09 measured it,
  01-10 wrote the ADR, 01-12 flipped the checkbox — and Phase 3 then re-measured it twice more. A
  single decision consumed five plans across two phases because the measurement tooling was
  established only after the requirement was already in flight.
- **REQUIREMENTS.md bookkeeping drifted from reality repeatedly.** Two separate gap-closure plans
  (02-11, 03-07/03-08) existed solely to reconcile checkboxes and traceability rows against work
  that was already done. The record lagged the code by a whole verification cycle each time.
- **A premature checkbox flip had to be reverted.** RECON-07 was marked satisfied before ADR-0006
  existed (commit `799c53f` reverted it). Caught, but only after it shipped into the file.
- **Nyquist validation never ran at all.** All four `VALIDATION.md` files sat at `status: draft`
  through the entire milestone and nobody noticed until the closing audit.

### Patterns Established

- **ADR-first precedence.** `.planning/decisions/` outranks PRDs and ingested docs; the shipped tree
  outranks both. Wired into PROJECT.md as an explicit precedence order.
- **Machine-parseable decision records.** ADRs are validated by `adr-parser.cjs`, and checkbox flips
  can be gated on that parse succeeding — a decision cannot be cited before it is well-formed.
- **Provenance-standard measurement (D-17).** Every recorded figure carries `rustc -vV`,
  `cargo --version`, `git rev-parse HEAD`, `date -u` and raw pasted stdout — not a restated number.
  This is what made the 84.79% figure auditable months later.
- **Deferrals carry a named owner or an explicit "no owner assigned".** Never a silent drop. Ten
  deferrals left this milestone; every one names where it goes.
- **Superseded work is preserved, not deleted.** The 2026-05-27 benchmark run stayed in place marked
  superseded; plan 01-04 got a disposition record rather than removal.

### Key Lessons

1. **Establish the measurement before writing the requirement that depends on it.** The coverage
   gate cost five plans because the requirement (RECON-07) was written before anyone knew whether
   the number could be produced in this sandbox. Cheap fix: a feasibility spike on the measurement
   tool during discuss-phase.
2. **Bookkeeping is not free — schedule it inside the plan that does the work.** Every phase needed
   a dedicated follow-up plan to reconcile REQUIREMENTS.md. Flipping the checkbox should be the
   last task of the plan that earns it, not a separate cleanup pass.
3. **A count copied from a report is a liability; a property proven by a command is not.** Prefer
   success criteria that re-derive themselves ("every example target builds") over criteria that
   restate a number ("22 examples").
4. **Verification timestamps are load-bearing.** A documentation-only commit added after the fact
   invalidated Phase 1's verification and forced this milestone to close as `override_closeout`.
   If a phase is verified, later edits to its directory need either a re-verify or a conscious
   override.
5. **Workflow defaults can be wrong for the project.** The stock milestone-close step deletes
   REQUIREMENTS.md; here that file carries forward scope for twelve unstarted phases. Read what a
   destructive step actually targets before running it.

### Cost Observations

- Sessions: not tracked this milestone.
- Notable: the offline coverage pipeline was the single highest-leverage artifact produced — it
  unblocked QUAL-01/02/03 and is reusable by Phase 15's PIPE-02 without modification.
- Notable: three phases required a second verification pass. Budgeting one re-verification per
  phase as the expected case, rather than the exception, would have made the schedule honest.

---

## Cross-Milestone Trends

### Process Evolution

| Milestone | Sessions | Phases | Key Change |
|-----------|----------|--------|------------|
| v0.7.1 | — | 4 | First milestone with protected decisions. The corpus had 0 locked ADRs across twelve prior milestones and eighteen months; this one produced 9. |

### Cumulative Quality

| Milestone | Tests | Coverage | Zero-Dep Additions |
|-----------|-------|----------|-------------------|
| v0.7.1 | 2,924 passing (+185 doc tests) | 85.92% (floor 84%) | 0 new dependencies |

### Top Lessons (Verified Across Milestones)

1. *(Awaiting a second milestone to cross-validate.)* The strongest single-milestone candidate:
   record the decision before writing the code, and gate the record on a parser.

---

*Next milestone: Milestone 2-3 close-out (Phases 5-6). Start with `/gsd-new-milestone`.*
