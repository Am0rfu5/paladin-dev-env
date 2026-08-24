# Phase 11: Facade Residue & Deferred Register Disposition - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.
> Decisions are captured in CONTEXT.md — this log preserves the alternatives considered.

**Date:** 2026-08-08
**Phase:** 11-facade-residue-deferred-register-disposition
**Areas discussed:** println disposition rule (FACADE-01) · D1–D4 posture (FACADE-02) ·
the `user_service.rs` collision (D2 ↔ DEFER-02) · recording home and ADR allocation ·
D5 framing correction (round 2) · ignored doctests (round 2)
**Mode:** interactive, two rounds. Round 2 was triggered by a verification finding that
overturned round 1's premise for FACADE-01.

---

## Round 1

### println disposition rule (FACADE-01)

Framing given to the user: FACADE-01 is the only code change in the phase — 17 occurrences across
6 files; Phase 10's boundary was explicitly "no `.rs` file is touched"; CLAUDE.md forbids
`println!`/`panic!` in library code.

| Option | Description | Selected |
|--------|-------------|----------|
| Default to log, annotate exceptions | Convert every occurrence to the project's logging macro unless a specific one is provably deliberate stdout, in which case annotate it inline with its reason. Per-file disposition recorded either way. | ✓ |
| Convert all 17, no exceptions | Every occurrence becomes a log call. Zero judgement calls, but risks breaking an intentional stdout path. | |
| Annotate only — no `.rs` changes | Record dispositions in `.planning/`, hand conversion to a later phase. Preserves Phase 10's boundary but closes FACADE-01 on the weaker half. | |

**User's choice:** Default to log, annotate exceptions.
**Notes:** ⚠ **The framing I supplied with this question was partly wrong.** I asserted that "4 of
the 6 files are infrastructure/resilience or arsenal adapters where stdout is almost certainly
accidental." Verification immediately afterward showed all 17 occurrences are rustdoc example lines
inside fenced blocks, and none is runtime code. The rule the user selected still resolves the case
correctly — doc-example `println!` is *provably* deliberate stdout, so the exception branch fires
for all 17 — but the outcome is the opposite of what the question implied. Corrected to the user
before round 2. Captured as CONTEXT.md **D-01**.

---

### D1–D4 posture (FACADE-02)

Scale given: "do" on D1 means a mechanical path rewrite of 49 files plus preserving
`platform/mod.rs`'s maneuver-parser path injection, which carries real logic rather than re-exports.

| Option | Description | Selected |
|--------|-------------|----------|
| Decide each on merit, relocations deferred | Each of D1–D4 gets a real verdict with an owner and, where deferred, a concrete trigger — but no relocation executes in this phase. | ✓ |
| Defer all four with triggers | Uniform posture, cheapest and most predictable, but risks reading as a rating by another name if a trigger is vague. | |
| Decide on merit and execute what is cheap | Same verdicts, but low-risk items actually land. Larger blast radius; D1 and D3 would still defer. | |

**User's choice:** Decide each on merit, relocations deferred.
**Notes:** Captured as CONTEXT.md **D-04**, with per-item verdicts at **D-05** (D1), **D-06** (D2),
**D-07** (D3), **D-08** (D4). D-05 records a distinction the register glosses: ADR-0018 retired old
paths for *relocated types*, which is not the same construct as `src/core/`'s surviving re-export
layer — so ARCH-04 does not settle D1 on its own.

---

### The `user_service.rs` collision (D2 ↔ DEFER-02, Phase 15)

Framing: M8's register plans to **split** `user_service.rs`; Deferred-QA Epic 28 plans to **test**
it to ≥ 80%. REQUIREMENTS.md says "do not schedule independently." Narrowing facts supplied:
reconciliation commit `6704807` already found no user-service split was needed for the controller
case because `UserServiceTrait` and the DTOs already live in `paladin-core`; the full relocation is
already a run-3 v2 tech-debt item.

| Option | Description | Selected |
|--------|-------------|----------|
| Withdraw D2's split half, record why | The split is arguably planned twice and partly obsolete. Withdrawing frees Phase 15 to test the file as-shipped and removes the collision rather than sequencing it. | ✓ |
| Sequence split-before-test, hand to Phase 15 | Keep the split, state it lands before Epic 28's tests, record that this changes Epic 28's estimate and mock set. Honours the register but couples two phases. | |
| Record the coupling, let Phase 15 decide | Document both positions, take no verdict on ordering. Safest, but leaves D2 as a rating — the exact thing FACADE-02 exists to stop. | |

**User's choice:** Withdraw D2's split half, record why.
**Notes:** Captured as CONTEXT.md **D-06**, rated `costly` for reversibility — Phase 15 will size
DEFER-02 against the unsplit file, so reinstating the split later invalidates that sizing.
`content_service.rs` and `event_manager.rs` remain genuine D2 items and keep their own verdicts;
only the `user_service` axis is withdrawn.

---

### Recording home and ADR allocation

Framing: FACADE-03 requires reintroduction conditions live in `.planning/` "rather than only in a
DOC"; FACADE-04 needs each row marked done / not-a-candidate / still-open. Phase 10's D-00g says
contested → ADR, code-settled → ledger row; D-00c says `.project/` sources are annotated in place.

| Option | Description | Selected |
|--------|-------------|----------|
| ADRs for contested, register file for the rest | New ADRs from 0034 for genuinely contested positions, plus a `.planning/` register carrying the removed features and the FACADE-04 triage table. `.project/` gets dated banners per D-00c. | ✓ |
| One ADR per requirement (0034–0037) | Uniform and easy to cite, but manufactures ADRs for things nobody disputes, against D-00g. | |
| Extend the milestone-07-08 ledger, minimal new ADRs | Fewest new files; risks burying the FACADE-03 reintroduction conditions the requirement wants findable. | |

**User's choice:** ADRs for contested, register file for the rest.
**Notes:** Captured as CONTEXT.md **D-14**. Contested → ADR: the D1–D4 disposition set, and the
`paladin-ml` leaf-crate placement condition (the surviving half of the M8 Epic 3 non-goal that
`paladin-herald` overrode, carried only by a DOC today). Not contested → register/ledger: the 17
rustdoc dispositions, the CLI surface's status, the FACADE-04 triage table. Exact numbering left to
the planner as Claude's Discretion.

---

## Round 2 — triggered by a verification finding

Between rounds, direct verification established that all 17 FACADE-01 occurrences are `///` or `//!`
doc-comment lines inside fenced ```` ```rust ```` / ```` ```rust,ignore ```` blocks. Filtering the
grep to non-doc-comment lines returns nothing. This was surfaced to the user as a correction before
asking round 2.

### D5 framing correction

| Option | Description | Selected |
|--------|-------------|----------|
| Correct register + ROADMAP criterion | Annotate all 17 as deliberate rustdoc-example stdout; dated banner on the M8 register per D-00c, plus an amendment to ROADMAP criterion 1 so it no longer implies a conversion that should not happen. | ✓ |
| Correct the register only | Banner on the register; leave ROADMAP criterion 1 and satisfy it via the "annotated with its reason" branch. Less churn, but the ROADMAP keeps implying runtime calls exist. | |
| Record the disposition, correct nothing | Write dispositions into the phase's artefacts and close FACADE-01. Cheapest, but both sources stay misleading. | |

**User's choice:** Correct register + ROADMAP criterion.
**Notes:** Captured as CONTEXT.md **D-02**, rated `costly` — amending the phase's own governing
success criterion mid-milestone means a later audit reads the amended text, so the superseded
original must stay visible per D-00c/D-00d. Note the corpus-level significance recorded in
CONTEXT.md `<specifics>`: `deferred-items.md` is described across the corpus as its highest-fidelity
document, and this is the first measured case of it being misleading — right about the count, wrong
about the kind.

---

### Ignored doctests

| Option | Description | Selected |
|--------|-------------|----------|
| Note it, hand to Phase 15 | Record the four `rust,ignore` fences with Phase 15 as owner, consistent with Phase 10 having already routed doctest posture there. | ✓ |
| Un-ignore them in this phase | Make the four `herald_registry` examples real doctests now. Genuine quality win, but collides with Phase 15's ownership. | |
| Out of scope, do not record | FACADE-01 asks only for a disposition per occurrence. Least scope, but discards a real finding. | |

**User's choice:** Note it, hand to Phase 15.
**Notes:** Captured as CONTEXT.md **D-03** with the `file:line` list
(`herald_registry.rs:163,182,195,208`).

---

## Claude's Discretion

Recorded in CONTEXT.md `<decisions>` → `### Claude's Discretion`:

- Exact ADR count and numbering within 0034+ — D-14 fixes the rule, the planner assigns numbers.
- The `.planning/` home and filename for the FACADE-03 register and the FACADE-04 triage table
  (one file or two; which directory). Constraint: findable from `.planning/` without reading
  `.project/`.
- Plan decomposition and wave assignment — all four requirements are mutually independent since
  D-04 defers execution.
- Whether the FACADE-01 per-file disposition is additionally recorded as a non-executable source
  comment; `.planning/` is the required home either way.

## Deferred Ideas

- Un-ignoring the four `rust,ignore` doctests in `herald_registry.rs` → Phase 15.
- Executing any D1–D4 relocation → architecture work with its own phase.
- Rewriting the 49 `crate::core::` importers → D1's "do" branch, trigger = a facade-wide no-alias sweep.
- Reintroducing the `paladin user …` CLI surface → recorded deferral with an intact recovery path
  (`git show 3d48768^:src/application/cli/commands/user.rs`).
- Creating `paladin-ml` → placement condition recorded; crate explicitly out of scope per PROJECT.md.
- Confirming ADR-0031 with a human → flagged as CONTEXT.md **D-00m**. Phase 10 ran `--auto` and none
  of its nine decisions were human-confirmed; ADR-0031 was one of two it flagged `⚠ HUMAN REVIEW`.
  Not blocking Phase 11 because no relocation executes, but any future phase executing a D3/D4 edge
  should confirm it first.
