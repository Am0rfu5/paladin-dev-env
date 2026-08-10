# ADR-0029: Version trajectory — `v0.1.0-rc.1` as closed history

## Status

Accepted

**Date:** 2026-08-08

## Context

Three documents anchor the first publishable release at a different figure than what shipped, and
HARD-03's own text is stale about what the current tree looks like.

**What the documents anchored.** M7 Epic 4's PRD
(`.project/Milestone_7-Production-Hardening/Epic_4/prd-api-stabilization-pre-release-preparation.md`,
Appendix C "Versioning Strategy", "Phase 1: Lockstep Versioning (Initial)") and the Milestone 7
overview's own Appendix C
(`.project/Milestone_7-Production-Hardening/overview/Milestone_7-Tier_4-Production_Hardening.md:616-625`)
both anchor the first publishable release at a lockstep `0.2.0`:

```toml
[workspace.package]
version = "0.2.0"
```

**What actually shipped.** All ten crates published at `0.1.0`, tagged `v0.1.0-rc.1` at commit
`a9530fc` on 2026-05-28
(`.project/Milestone_7-Production-Hardening/Epic_4/post-release-verification-v0.1.0-rc.1.md:1-4`,
`git log -1 --format=%H%n%cs a9530fc` run this session confirms both the hash and the date). The
`release-readiness-audit-report.md` (dated 2026-05-28) records every gate `PASS` — `cargo test
--workspace`, `cargo clippy --workspace -- -D warnings`, `cargo fmt --all -- --check`, `cargo doc
--workspace --no-deps`, `cargo publish --dry-run` for every public crate, `cargo audit`, and
license compatibility — closing with **`Recommendation: GO for release candidate tagging`**.
`post-release-verification-v0.1.0-rc.1.md` confirms all ten crates resolve on docs.rs, including
the package/lib split visible as `docs.rs/paladin-ai-core/latest/paladin_core/` (the package was
renamed to avoid a crates.io collision, per the audit's "Blocker Closure Notes") alongside
`docs.rs/paladin-ai/latest/paladin/`, plus an external smoke project compiling successfully against
`paladin-ai = "0.1.0"` (`SMOKE_OK: paladin-ai dependency resolved and compiled`).

**The sharper finding: HARD-03's forward coupling has already fired.** REL-01 is `[x]` at
`REQUIREMENTS.md:358` ("Version metadata agrees everywhere") with a `Phase 4 | Complete`
traceability row at `REQUIREMENTS.md:3913`, converged by Phase 4 on `0.7.0` via ADR-0008
(`.planning/decisions/0008-workspace-version-0-7-0.md`) and commit `c2e20a1`. HARD-03's own text
(`REQUIREMENTS.md:1405-1420`) still reads "**Feeds REL-01**, which converges the three-way version
disagreement" — a hand-off framing that is now backwards: REL-01 ran and closed before this phase
did.

**HARD-03's current-state clause is also two releases stale.** HARD-03's text states "Current tree:
`Cargo.toml` `0.6.0`, branch `release/v0.7.0`, latest tag `v0.5.1`." Measured this session:

- `Cargo.toml:34` — `version = "0.7.0"` (not `0.6.0`).
- `git tag --sort=-v:refname | head -3` — `v0.7.1`, `v0.7.0`, `v0.5.1` (not `v0.5.1` as latest;
  Milestone 1's close-out shipped `v0.7.1` on 2026-08-04, after `v0.7.0`).
- Branch: `worktree-agent-ad46b65d60e342b48`, forked from `release/v0.7.0` at the base commit this
  phase started from — the underlying release branch name is unchanged.

Plan 10-01 corrected HARD-03's current-state clause at source in `REQUIREMENTS.md`; this ADR
records the trajectory these figures describe as history, and states the current tree only to
prove the figures below are measured, not transcribed from a stale ingest.

## Decision

Under the D-00b precedence order (ADR → shipped tree → `.planning/codebase/` map →
`intel/code-verification.md` → PRD → DOC → checkbox — an ADR that contradicts shipped code is an
instruction to change the code), the verdict is: **the `v0.1.0-rc.1` trajectory is recorded as
closed history, no rc.1 artefact is treated as current state, and REL-01 is not re-opened by this
ADR because Phase 4 already converged it.**

**(i) REL-01's status is confirmed, not re-decided.** REL-01 did not converge on any rc.1 figure —
it converged on `0.7.0` via ADR-0008. A planner who treats REL-01 as open, on the strength of
HARD-03's own "Feeds REL-01" hand-off language, will re-plan a closed requirement. ADR-0008 is
cited here as the decision that owns the landing at `0.7.0`; this ADR does not compete with it.

**(ii) This ADR is the single home for the whole trajectory.** Phase 13 / ORCH-05 extends the
`## Trajectory` table below rather than writing a second version ADR. HARD-03 covers `rc.1` →
`v0.2.0`; ORCH-05 covers `v0.3.0` → `v0.6.0`; REL-01 (Phase 4, done) covers the landing at `0.7.0`.
Three ADRs for one unbroken line would guarantee the third contradicts the first.

## Trajectory

Rows are added in **ascending version order**, and existing rows are never re-sorted or re-keyed —
a row is keyed by the tag or target figure, not by the milestone that carried it, so a milestone
that targeted two figures gets two rows.

| Version / tag | Status in the tree | Date | What it was | Evidence |
|---|---|---|---|---|
| `0.2.0` (lockstep target) | targeted, not shipped | — | M7 Epic 4 PRD Appendix C and the overview's own Appendix C both anchored the first publishable release here; no tag `v0.2.0` exists in the tree | `prd-api-stabilization-pre-release-preparation.md` Appendix C; `Milestone_7-Tier_4-Production_Hardening.md:616-625` |
| `0.1.0` / `v0.1.0-rc.1` | shipped | 2026-05-28 | All ten crates first published at `0.1.0`; GO sign-off; docs.rs verified for all ten including the `paladin-ai-core`/`paladin_core` package-lib split; external smoke project compiled against `paladin-ai = "0.1.0"` | `release-readiness-audit-report.md`; `post-release-verification-v0.1.0-rc.1.md`; `git log -1 --format=%H%n%cs a9530fc` → `a9530fc239627e1a1381cdc901e6f1e3d6c21d0f`, `2026-05-28` |
| `v0.2.0` | shipped | — | Milestone 8 targeted `v0.2.0` throughout its own documents; its Epic 7, written 2026-06-06, targets "post-v0.5.1 (Unreleased)" | `REQUIREMENTS.md:1409-1412` |
| SUPERSEDED — `v0.3.0` … `v0.5.1` (former placeholder) | **Superseded in place, 2026-08-10 (plan 13-12).** The four rows below now carry this range; this row's original text is retained per D-00d. Original text: "shipped between Epic 5 and Epic 7 — **Owned by Phase 13 / ORCH-05 — to be appended.** The M8-11 dependency graph's `v0.2.0` → `v0.3.0` → `v0.4.0` → `v0.5.0` sequence completed in this span; ORCH-05 supplies the per-tag rows" | — | superseded — see rows below | `REQUIREMENTS.md:1411-1412`; `git tag --sort=-v:refname \| head -8` re-run 2026-08-10 → `v0.7.1`, `v0.7.0`, `v0.5.1`, `v0.5.0`, `v0.4.3`, `v0.4.2`, `v0.4.1`, `v0.4.0` (and `v0.3.0-rc.1` immediately below the head-8 window, confirmed via `git tag --sort=-v:refname \| grep -E '^v0\.[3-6]'`) |
| `v0.3.0` (Milestone 9) | shipped as release candidate | 2026-05-31 | Milestone 9's finalization Epic (Epic 6) cut the quality-gate release candidate; `CHANGELOG.md:596` `## [0.3.0] - 2026-05-31`, grouped by feature area (Orchestration, Scheduler & Queue, Content Pipeline, Agent–Orchestrator Bridge, User/Admin System & Security). Lockstep bump per `REQ-lockstep-versioning`'s mechanism (`Makefile:498` `cargo release version --execute --workspace`, `release.toml:18` `shared-version = true`) | ledger row `REQ-m9-quality-gate-v030` (`.planning/ledgers/milestone-09-12.md:323`); tag `v0.3.0-rc.1` confirmed present via `git tag --sort=-v:refname \| grep -E '^v0\.[3-6]'` re-run 2026-08-10; `git show v0.3.0-rc.1:Cargo.toml` confirms lockstep `version = "0.3.0"` at that tag |
| `v0.4.0` (Milestone 10) | shipped | 2026-05-31 | Milestone 10's finalization Epic cut the CI-hardening/release-automation bump; `CHANGELOG.md:521` `## [0.4.0] - 2026-05-31`. Lockstep bump per `REQ-lockstep-versioning`'s mechanism | ledger row `REQ-m10-v040-release` (`.planning/ledgers/milestone-09-12.md:395`); tag `v0.4.0` confirmed present via `git tag --sort=-v:refname \| head -8` re-run 2026-08-10 (`v0.4.3, v0.4.2, v0.4.1, v0.4.0` all present); `git show v0.4.0:Cargo.toml` confirms lockstep `version = "0.4.0"` at that tag |
| `v0.5.0` (Milestone 11) | shipped | 2026-06-03 | Milestone 11's finalization Epic cut the documentation-overhaul/publish bump; `CHANGELOG.md:444` `## [0.5.0] - 2026-06-03`, recording Milestones 8-11 consolidated. Lockstep bump per `REQ-lockstep-versioning`'s mechanism | ledger row `REQ-m11-v050-release` (`.planning/ledgers/milestone-09-12.md:467`); tag `v0.5.0` confirmed present via `git tag --sort=-v:refname \| head -8` re-run 2026-08-10; `git show v0.5.0:Cargo.toml` confirms lockstep `version = "0.5.0"` at that tag |
| `v0.6.0` (Milestone 12) | shipped, no tag cut | 2026-06-10 | Milestone 12's finalization Epic (Epic 7) bumped every workspace crate to `0.6.0`; `CHANGELOG.md:139-164` `## [0.6.0] - 2026-06-10`, `:164` "Version: all workspace crates bumped to 0.6.0." Lockstep bump per `REQ-lockstep-versioning`'s mechanism, confirmed by the finalization commits rather than a tag — `git tag --list 'v0.6*'` re-run 2026-08-10 returns nothing, a gap the reader needs to know about (not a failed acceptance criterion; the requirement's own criteria do not name a tag) | ledger row `REQ-m12-v060-release` (`.planning/ledgers/milestone-09-12.md:563`); `git log --oneline` confirms `90ca591 chore(release): bump all workspace crates to 0.6.0 (M12 E7, task 6.0)`, `67b6207 docs(release): finalize CHANGELOG [0.6.0] + regen API baseline (M12 E7, task 7.0)`, `23b187b chore(release): regenerate OpenAPI baseline for 0.6.0 (M12 E7)`, all re-confirmed present this session |
| `0.7.0` / `v0.7.0`, `v0.7.1` | shipped | 2026-08-03 (`v0.7.0`) | Phase 4 converged every manifest and internal pin on `0.7.0` via ADR-0008; `v0.7.1` followed as Milestone 1's close-out tag on 2026-08-04 | `.planning/decisions/0008-workspace-version-0-7-0.md`; `Cargo.toml:34` → `version = "0.7.0"` (re-grepped this session); `git tag --sort=-v:refname \| head -3` → `v0.7.1`, `v0.7.0`, `v0.5.1` (re-run this session) |

## Considered Options

- **One trajectory ADR that later phases append to** (accepted) — a single, extensible home for an
  unbroken version line, extended by whichever phase reaches the next segment.
- **A separate ADR per version range** (rejected) — three ADRs for one unbroken line guarantee the
  third contradicts the first; there is exactly one trajectory, not three competing ones.
- **Re-opening REL-01 to converge with the whole trajectory in view** (rejected) — REL-01 is `[x]`
  and Phase 4's ADR-0008 already owns the landing at `0.7.0`; HARD-03 confirms rather than
  converges, and re-opening a closed requirement to re-litigate an already-settled landing point
  serves no one.
- **Recording the trajectory only as a ledger row** (rejected) — the `0.2.0`-versus-`0.1.0`
  disagreement is a contested position between two documents and the tree (D-00g routes contested
  positions to an ADR, not a ledger row that a later reader could take as merely descriptive).

## Code Locations

- `Cargo.toml:34` — `version = "0.7.0"`, re-grepped this session.
- `REQUIREMENTS.md:358` — REL-01's checkbox, `[x]`.
- `REQUIREMENTS.md:3913` — REL-01's traceability row, `Phase 4 | Complete`.
- `.planning/decisions/0008-workspace-version-0-7-0.md` — Phase 4's convergence on `0.7.0`, the
  landing point this ADR does not re-decide.
- `.project/Milestone_7-Production-Hardening/Epic_4/prd-api-stabilization-pre-release-preparation.md`
  Appendix C, and `.project/Milestone_7-Production-Hardening/overview/Milestone_7-Tier_4-Production_Hardening.md:616-625`
  — both carry the `0.2.0` lockstep target.
- `.project/Milestone_7-Production-Hardening/Epic_4/release-readiness-audit-report.md` — every gate
  `PASS`, the `GO` recommendation, and the crates.io package-rename blocker-closure notes.
- `.project/Milestone_7-Production-Hardening/Epic_4/post-release-verification-v0.1.0-rc.1.md` — the
  docs.rs verification of all ten crates and the external smoke-project compile.
- `REQUIREMENTS.md:1405-1420` — HARD-03 in full, including the current-state clause plan 10-01
  corrected in place.
- `REQUIREMENTS.md:4036` — the cross-phase coupling table's row naming HARD-03 → ORCH-05 and
  stating that ORCH-05 completes the chain from `v0.3.0` through `v0.6.0`.
- `git tag --sort=-v:refname | head -6` (run this session) — `v0.7.1`, `v0.7.0`, `v0.5.1`, `v0.5.0`,
  `v0.4.3`, `v0.4.2`.
- `git log -1 --format=%H%n%cs a9530fc` (run this session) —
  `a9530fc239627e1a1381cdc901e6f1e3d6c21d0f`, `2026-05-28`.

## Code Conformance

conforms

No `.rs` file, `Cargo.toml`, or CI workflow is touched by this ADR. It records history and confirms
an already-executed convergence; the tree already reflects `0.7.0` via Phase 4's ADR-0008 and plan
04-05's execution.

## Downstream Consumers

- **Phase 13 / ORCH-05** — appends `v0.3.0` through `v0.6.0` rows to the `## Trajectory` table
  above, in ascending order, without re-sorting or re-keying the existing rows, and inherits the
  note that REL-01 is already converged (this ADR's decision (i)) so ORCH-05 applies the trajectory
  rather than re-deciding it.
- **Phase 10 / HARD-01** — the ledger rows for `REQ-versioning-policy`, `REQ-release-readiness-audit`
  and `REQ-changelog-v020-cut` cite this ADR, written by plans 10-08 and 10-10.
- **ORCH-05's numbering half** is discharged by `.planning/decisions/0030-milestone-7-self-numbering.md`
  lines 79-84 (the Roadmap Extension Protocol's fifth-instance prediction, already closed there); the
  run-5 provenance-key re-confirmation (plan 13-12, 2026-08-10) found no fifth collision and is not
  re-recorded here — see `0030-milestone-7-self-numbering.md` for the record.
