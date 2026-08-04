# Phase 5: Milestone 2-3 Ground Truth - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.
> Decisions are captured in CONTEXT.md — this log preserves the alternatives considered.

**Date:** 2026-08-04
**Phase:** 5-milestone-2-3-ground-truth
**Mode:** `--auto` — no human answered any question. Claude selected the recommended option for
every gray area and logged the reasoning. Review before planning if any selection looks wrong.
**Areas discussed:** Ledger depth & evidence bar · VERIFY-02 block verdicts · Release-notes
correction mechanism · Vision surfaces & encryption placement · Coverage answer extension ·
Live-API key semantics · ADR allocation · Plan decomposition

---

## Ledger depth & evidence bar (VERIFY-01)

| Option | Description | Selected |
|--------|-------------|----------|
| Full Phase 1 bar for all 118 rows | Every row needs `file:line` + a named exercising test/example/command to reach `satisfied`; ingest `Shipped` rows get no free pass | ✓ |
| Accept existing `Shipped` rows, full bar only on `Verify` rows | Faster; ~40 rows skip verification | |
| Two-tier bar with a lighter standard for already-shipped rows | Explicit tiering in the verdict vocabulary | |

**Selection:** Full bar for all 118 (auto — recommended). **Rationale:** an ingest `Shipped` verdict
*is* the bare "the code exists" claim that Phase 1's D-19 evidence bar was written to reject.
Accepting those rows would import the exact false-positive class the bar exists for. Triage (D-02)
directs *effort* toward contested rows; it does not lower the standard.

---

## VERIFY-02 — verdict granularity for the three blocks

| Option | Description | Selected |
|--------|-------------|----------|
| Per-block verdict backed by a parent-task cluster table | 3 verdicts; each justified by verifying one distinct capability claim per parent-task heading | ✓ |
| Bare per-block verdict, prose justification only | Cheapest; satisfies the literal requirement | |
| Per-checkbox verification | 155 items verified individually | |

**Selection:** Cluster-table-backed block verdicts (auto — recommended). **Rationale:**
REQUIREMENTS.md forbids "a task list derived from checkbox arithmetic", but CLOSE-02's scope is
*set* by these verdicts — a bare prose verdict would hand Phase 6 a mood instead of a named scope.
Clustering by the task list's own parent-task headings threads both constraints. Per-checkbox
verification is the arithmetic the requirement explicitly rejects.

---

## VERIFY-03 — how the defective release notes get corrected

| Option | Description | Selected |
|--------|-------------|----------|
| Dated banner + inline corrections, superseded text retained, plus ADR-0010 | Two deliverables; `.project/` stays usable as a historical record | ✓ |
| Record the correction only in `.planning/`, leave `.project/` untouched | Preserves history perfectly | |
| Rewrite the release notes with the correct numbering | Cleanest to read | |

**Selection:** Banner + inline + ADR (auto — recommended). **Rationale:** VERIFY-03 asks for two
things — "recorded once and permanently" (the ADR) *and* "the defective source document is corrected
in-repo" (the edit). Option 2 fails the second; option 3 destroys the provenance five ingest runs
were built on. The retain-superseded-text pattern is already this corpus's house style (ROADMAP.md's
own `**Amended by Phase 4, dated…**` blocks, and the Phase 2/3 ledger amendment sections).

---

## VERIFY-04 — vision surfaces, and where encryption-at-rest goes

| Option | Description | Selected |
|--------|-------------|----------|
| Both intended long-term, entry-point guidance recorded, no migration | `VisionPort` = application entry point; `VisionCapableLlm` = adapter-author surface | ✓ |
| Deprecate `VisionCapableLlm` with a migration note | Single surface going forward | |
| Deprecate `VisionPort` | Keeps the Epic 13 lineage | |

**Selection:** Both retained with guidance (auto — recommended). **Rationale:** they sit at different
layers and are reached by different entry points (`PaladinBuilder::enable_vision` vs
`PaladinExecutionService::execute_with_vision`) — that is *why* both ship. REQUIREMENTS.md explicitly
forbids planning a migration on the strength of the PRD conflict alone.

**Encryption sub-question — the premise turned out to be false.** The roadmap flagged this as an
open-ended risk ("new security work with no phase home anywhere in Phases 5-16"). Verified against
the tree during this session:

| Option | Description | Selected |
|--------|-------------|----------|
| Record as *built but unwired*; wiring decision goes to Phase 6 CLOSE-03; no new requirement | Evidence closes the risk | ✓ |
| Create a new requirement / phase for encryption work | What the roadmap anticipated | |
| Record as consciously dropped | The other branch the roadmap named | |

**Selection:** Built-but-unwired (auto — evidence-driven, not a preference). **Rationale:**
`VisionError::EncryptionError` exists (`vision.rs:210-212`); `EncryptionService` with
`encrypt_image_data`/`decrypt_image_data`, `SecureData` (`Zeroize, ZeroizeOnDrop`) and
`DataRetentionPolicy` all ship at `src/infrastructure/security/encryption.rs`; both dependencies are
declared unconditionally at `Cargo.toml:134-135`. **But all three have zero consumers outside their
own module.** Neither "shipped" nor "dropped" was the right answer. No new phase, no new requirement.

---

## VERIFY-05 — how the coverage answer is extended

| Option | Description | Selected |
|--------|-------------|----------|
| Amend ADR-0006 in place | One number in one place; matches the ADR's own ratchet clause | ✓ |
| New ADR that extends/supersedes 0006 | Cleaner authorship boundary | |
| New ADR for module-scoped gates only, 0006 untouched | Separates global from module scope | |

**Selection:** Amend in place (auto — recommended). **Rationale:** RECON-07 exists to eliminate the
"choosing between two numbers" failure; a second coverage ADR recreates it. ADR-0006 already
specifies in-place amendment for its own ratchet, and Phase 2's convention (inherited by Phase 5)
makes amend-in-place the house rule.

**Sub-decisions, all auto-selected:**

| Question | Options | Selected |
|---|---|---|
| Module-scoped gates (Herald ≥ 95%, autonomous ≥ 90%) | hard gate now / recorded target + owner, enforcement to Phase 15 / withdraw | recorded target + owner ✓ |
| Re-measure coverage in Phase 5? | yes / no — reuse 84.79% | no ✓ |
| `paladin-server.rs` at 0.00% | fix now / `deferred with reason`, owner Phase 15, `run()` seam named | deferred with named prerequisite ✓ |
| `minio.rs` feature scope | widen denominator now / record as outside gated scope, decision to Phase 15 | outside gated scope ✓ |
| ~78% Milestone-3 figure | reconcile / judge it (fails the 84% floor, stale) | judge it ✓ |

**Notes:** Phase 5 cannot enforce a gate — there is no CI coverage job to enforce it in (ADR-0006's
own `Code Conformance` records this). Widening the denominator to non-default features without a
measurement would move the 84% floor on no evidence, which ADR-0006 forbids.

---

## VERIFY-06 — live-API-test missing-key behaviour

| Option | Description | Selected |
|--------|-------------|----------|
| Keep the shipped panic; record that the gating supplies the PRDs' skip | Preserves both positions' intent | ✓ |
| Revert to graceful skip per Epic 23 FR-23.4.4 / Epic 24 US-24.7 | What both PRDs specified | |
| Env-var-selectable behaviour | Satisfies everyone, adds a knob | |

**Selection:** Keep the panic (auto — recommended). **Rationale:** the tree supplies a synthesis
neither PRD saw. The suite is double-gated — `tests/integration/mod.rs:34-35` gates the module on
`#[cfg(feature = "live-api-tests")]`, and all 13 tests carry `#[ignore]` — so a CI run with no keys
never compiles or executes it. **The "graceful skip with a clear message" both PRDs require is
supplied by the gating, not by the helper.** The panic fires only after an explicit double opt-in
(`--features live-api-tests -- --ignored`), where a silent skip would be a false pass — precisely
what the post-Epic-24 cleanup reversed it for. Option 3 was rejected as a third position where the
requirement asks for one.

**Notes:** a real defect surfaced. `require_api_key`'s doc comment
(`tests/integration/llm_live_api_tests.rs:63`) says *"Skip test if API key is not present or empty"*
while both match arms `panic!`. The panic messages themselves are already correct. CLOSE-03's
live-API half is therefore a doc-comment correction, not a behavioural change.

---

## ADR allocation

| Option | Description | Selected |
|--------|-------------|----------|
| Three new ADRs (0010-0012) + amend 0006 | One question per ADR, matching 0001-0009 | ✓ |
| One combined "run-2 recorded answers" ADR | Fewer files | |
| No new ADRs — everything as ledger rows | Lightest | |

**Selection:** Three ADRs + one amendment (auto — recommended). **Rationale:** Phase 1's precedent —
contested positions get ADRs, divergences settled by shipped code get ledger rows. VERIFY-03/04/06
are all contested; VERIFY-01/02 produce the ledger. 0010 = epic numbering, 0011 = vision surfaces +
encryption, 0012 = live-API semantics.

---

## Plan decomposition

| Option | Description | Selected |
|--------|-------------|----------|
| Scaffold → epic fan-out (2-3 epics/plan) → 3 block plans → 2 decision plans | ~10-11 plans | ✓ |
| Sequential single-track ledger build | Fewer plans, longer chain | |
| One plan per epic (14) + decisions | Maximum parallelism | |

**Selection:** Scaffold-first fan-out (auto — recommended). **Rationale:** the scaffold fixes the
ledger shape before six plans append into it. The three VERIFY-02 blocks get one plan each because
their verdicts set Phase 6's entire CLOSE-02 scope and must not be diluted. 14 single-epic plans
over-fragments ~8-row units of work. Matches Phase 1's proven shape (12 plans at a third the scale).

---

## Claude's Discretion

Recorded in CONTEXT.md `<decisions>` → *Claude's Discretion*:

- Exact banner wording and inline-correction markup for `RELEASE_NOTES_MILESTONE_3.md`.
- Whether ADR-0011 and ADR-0012 are two files or one combined ADR.
- Formatting of the parent-task cluster tables inside the ledger.
- Whether the `present, unproven` count becomes a reported headline figure.
- Ordering within the epic fan-out (Milestone 2 first or Milestone 3 first).

---

## Deferred Ideas

- **WARN-01** — Herald not reachable from Campaign, Chain of Command or the Commander router.
  Phase 6 must adopt or decline it.
- **Nyquist validation for Phases 1-4** — all four `VALIDATION.md` files read `status: draft`.
  `/gsd-validate-phase 1`…`4`.
- **Uncommitted `.github/workflows/ci.yml` change** — reverts Phase 4's advisory multi-arch
  wall-clock rationale to a hard 300s `::error::`, a budget MILESTONES.md records as never met
  (measured 2946s). Out of scope; flagged so it is not committed silently.
- **`grove_service.rs:537` hardcoded `model: "gpt-4"`** — CLOSE-01, Phase 6.
- **Re-measuring coverage under the Docker-backed `--features integration-tests` scope** —
  Phase 15 / PIPE-02.
- **Enforcing the module-scoped coverage gates in CI** — Phase 15 / PIPE-02.
- **Publishing ADRs to the mdbook** — carried forward unanswered from Phase 1; Phase 16.
