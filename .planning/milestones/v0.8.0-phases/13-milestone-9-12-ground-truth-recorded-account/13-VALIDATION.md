---
phase: 13
slug: milestone-9-12-ground-truth-recorded-account
# status lifecycle: draft (seeded by plan-phase) → validated (set by validate-phase §6)
# audit-milestone §5.5 distinguishes NOT-VALIDATED (draft) from PARTIAL (validated + nyquist_compliant: false) (#2117)
status: draft
nyquist_compliant: false
wave_0_complete: false
created: 2026-08-10
---

# Phase 13 — Validation Strategy

> Per-phase validation contract for feedback sampling during execution.
> Derived from `13-RESEARCH.md` §Validation Architecture (lines 729-779), verified 2026-08-10.

**This is a records phase.** Its code-change boundary (`13-CONTEXT.md` D-19) permits zero `.rs`
changes, so there is no test suite to sample. Validation here is the project's own D-00e evidence
bar: **no claim of closure without the exact command or `file:line` that produced it, recorded
verbatim.** Every "test" below is a shell command that already runs in this environment.

---

## Test Infrastructure

| Property | Value |
|----------|-------|
| **Framework** | None — direct shell verification (`grep`, `sed`, `awk`, `git log`, `git diff`, `gh run`/`gh api`, `mdbook build`), following `10-VALIDATION.md`'s precedent verbatim |
| **Config file** | none |
| **Quick run command** | Per-claim `grep -n "<pattern>" <file>` / `sed -n '<range>p' <file>` — the exact command the ledger row or ADR cites |
| **Full suite command** | Re-run every row of the Per-Task Verification Map below |
| **Estimated runtime** | ~5 seconds for the grep/git rows; ~3 minutes if `mdbook build docs/` is included |

---

## Sampling Rate

- **After every task commit:** Re-run the specific `grep` / `sed` / `git log` / `gh` command that the
  task's ledger row or ADR cites, before marking its cell or section complete.
- **After every plan wave:** Re-run the 120-row count and the per-section row counts against the
  in-progress ledger, to confirm the parallel wave-2 fan-out dropped or duplicated nothing:
  ```bash
  grep -c '^| REQ-' .planning/ledgers/milestone-09-12.md   # → 120
  git diff --numstat -- .planning/ledgers/milestone-09-12.md  # added == deleted for in-place edits
  ```
  This mirrors `10-11-SUMMARY.md`'s own wave-3 integrity check, scaled from 86 rows to 120.
- **Before `/gsd-verify-work`:** the full map below re-runs green, plus the two boundary assertions.
- **Max feedback latency:** < 10 seconds for every row except `mdbook build`.

---

## Per-Task Verification Map

| Task ID | Plan | Wave | Requirement | Threat Ref | Secure Behavior | Test Type | Automated Command | File Exists | Status |
|---------|------|------|-------------|------------|-----------------|-----------|-------------------|-------------|--------|
| ledger-exists | 01 | 1 | ORCH-01 | — | N/A | row-count | `grep -c '^\| REQ-' .planning/ledgers/milestone-09-12.md` → `120` | ❌ W1 | ⬜ pending |
| no-bare-verify | 02-06 | 2 | ORCH-01 | — | N/A | grep | trailing-marker check for a bare `Verify` verdict → `0` | ❌ W2 | ⬜ pending |
| requirements-pointer | 01 | 1 | ORCH-01 | — | N/A | grep | `grep -n 'Milestone 9-12 as-shipped ledger' .planning/REQUIREMENTS.md` — body reduced to a one-line pointer, not a duplicate | ❌ W1 | ⬜ pending |
| arithmetic-corrected | 01 | 1 | ORCH-01 (D-04) | — | N/A | grep | ORCH-01's `:2210-2212` sentence carries a dated correction naming the measured 35/53/32 split | ❌ W1 | ⬜ pending |
| supply01-trigger | 03 | 2 | ORCH-01 (D-06) | — | N/A | gh | `gh run list --workflow=ci.yml --limit 3 --branch release/v0.7.0` → newest newer than `2026-08-03`, conclusion `success` | ✅ run `31320378772` | ⬜ pending |
| both-halves-row | 03 | 2 | ORCH-01 (D-05) | — | N/A | grep | the M10 Epic 2 row carries **both** the criterion-false verdict and the 2026-08-08 fix with commit `cb75b2b` | ❌ W2 | ⬜ pending |
| ci-job-list | 03 | 2 | ORCH-01 (D-08) | — | N/A | grep | measured 15-job list recorded; PIPE-01's stale 14-job copy corrected at source | ❌ W2 | ⬜ pending |
| five-verdicts | 01 | 1 | ORCH-02 | — | N/A | grep | all five checkbox verdicts + the five-run pattern present in the ledger head note, in exactly one place | ❌ W1 | ⬜ pending |
| route-surface | 07 | 3 | ORCH-03(a) | — | N/A | grep | `grep -c '"/v1/agents' crates/paladin-web/openapi.json` → `≥1` (re-confirm, unchanged) | ✅ | ⬜ pending |
| sidecar-route-fix | 07 | 3 | ORCH-03(a′) / D-12 | — | N/A | grep | `grep -n 'POST /v1/agents/{id}/execute' docs/src/deployment-topologies/sidecar.md` → hit; unprefixed form → no hit | ❌ W3 | ⬜ pending |
| stale-paths | 09 | 3 | ORCH-03(b)-(e) | — | N/A | file+grep | per path: old path `ls` fails, new path exists, `.project/` dated banner present | ❌ W3 | ⬜ pending |
| adr-0037 | 07 | 3 | ORCH-03(a) | — | N/A | file+grep | `ls .planning/decisions/0037-*.md`; seven required H2 headings present | ❌ W3 | ⬜ pending |
| adr-0038 | 08 | 3 | ORCH-04(a) | — | N/A | file+grep | `ls .planning/decisions/0038-*.md`; `Code Conformance` → `conforms`; `Downstream Consumers` names Phase 14 | ❌ W3 | ⬜ pending |
| adr-0039 | 08 | 3 | ORCH-04(b) | T-13-01 | Capability limitation stated where a reader chooses a topology, not only in a non-goal | file+grep | `ls .planning/decisions/0039-*.md`; `grep -n 'tools.*memory' docs/src/deployment-topologies/http-service-host.md` → no hit; `grep -in 'garrison\|arsenal' docs/src/deployment-topologies/overview.md` → `≥1` | ❌ W3 | ⬜ pending |
| trajectory-rows | 10 | 4 | ORCH-05 | — | N/A | grep | `grep -cE '^\| v0\.[3-6]\.0 ' .planning/decisions/0029-version-trajectory-history.md` → `4`, ascending, existing rows unmoved | ❌ W4 | ⬜ pending |
| prediction-cited | 10 | 4 | ORCH-05 (D-17) | — | N/A | grep | ADR-0030 cited as the closure; **no rival version or numbering ADR created** (`ls .planning/decisions/` shows 0037-0039 only) | ❌ W4 | ⬜ pending |
| version-figures | 10 | 4 | ORCH-05 (D-18) | — | N/A | grep | ORCH-05's `:2300-2301` current-state clause carries a dated correction to `0.7.0` / `v0.7.1` | ❌ W4 | ⬜ pending |
| rs-boundary | 11 | 4 | D-19 | — | Zero source-code surface for a records phase | git diff | `git diff --name-only <base>..HEAD -- '*.rs' \| wc -l` → `0` | ❌ close-out | ⬜ pending |
| mdbook-baseline | 11 | 4 | D-19 | — | N/A | build | `mdbook build docs/` — exit code recorded verbatim and compared against the **pre-phase baseline established 2026-08-10 (exit 101, two pre-existing broken links in files this phase does not touch)** | ✅ baseline established | ⬜ pending |
| promotion-advance | 11 | 4 | D-20 | — | N/A | grep | `grep -n 'Next free ADR number' .planning/decisions/PROMOTION.md` → `0040`; advancing note explains 0037-0039 and states Part B candidate 9's disposition | ❌ close-out | ⬜ pending |
| no-dangling-adrs | 11 | 4 | ORCH-01 | — | N/A | grep | every `ADR-\d{4}` cited in the ledger resolves to a file in `.planning/decisions/` | ❌ close-out | ⬜ pending |
| no-pending-stubs | 11 | 4 | ORCH-01 | — | N/A | grep | zero unfilled row stubs / TBD cells remain in `milestone-09-12.md` | ❌ close-out | ⬜ pending |

*Status: ⬜ pending · ✅ green · ❌ red · ⚠️ flaky*

*Plan IDs above follow `13-CONTEXT.md` D-23's proposed decomposition and are indicative — the planner
owns the final numbering. The Requirement column is authoritative.*

---

## Wave 0 Requirements

**None — there is no test suite to scaffold.** Every command in the map above already runs in this
environment; the research session of 2026-08-10 is itself the Wave 0 proof, having executed each one
and recorded its result.

Two environment facts the plans must respect rather than re-discover:

- **`mdbook build docs/` currently FAILS (exit 101)** with two pre-existing broken-link errors in
  files this phase does not touch. The baseline predates any Phase 13 edit. The close-out must record
  this honestly — neither reinterpreting it as a pass nor attributing it to this phase's three-line
  documentation edit.
- **Not runnable here** (unchanged from Phases 9, 10 and 12): `cargo audit`, `cargo deny`,
  `cargo llvm-cov`, and anything Docker — `crates.io` returns HTTP 403 and `docker` is absent
  (`.planning/phases/04-release-coherence/04-ci-gate-deferrals.md`). Claims resting on these are
  scoped CI-only with their exact command recorded, never inferred as passing.

---

## Manual-Only Verifications

| Behavior | Requirement | Why Manual | Test Instructions |
|----------|-------------|------------|-------------------|
| `AgentProvisioner` stays in `paladin-web` is the right call | ORCH-04(a) / D-14 | ⚠ HUMAN REVIEW — an architectural placement decided under `--auto`, rated `costly`. No command can validate a judgement; only a human can ratify it. | Read ADR-0038's `Considered Options`. Confirm the `utoipa::ToSchema` evidence at `crates/paladin-web/src/agent_registry.rs:55` and the absence of `utoipa` from `crates/paladin-ports/Cargo.toml`. Select `option-a` at the wave-3 checkpoint to ratify, or reject to send the move to Phase 14. |
| Recording Garrison/Arsenal absence as a topology property rather than planned scope | ORCH-04(b) / D-15 | ⚠ HUMAN REVIEW — writes a capability limitation into published user documentation and declines to schedule the capability. Rated `costly`. | Read ADR-0039. Confirm `http-service-host.md:54`'s current claim and `overview.md`'s silence. Select `option-a` to ratify, or reject to make it planned scope with a target. |
| PROMOTION.md Part B candidate 9's non-promotion | D-20 | A gap the researcher found between `PROMOTION.md`'s own inventory (which assigns candidate 9 — the M9 Epic 4 agent/orchestrator bridge decision — to Phase 13) and D-20's locked three-ADR allocation. Resolving it is a scope judgement, not a check. | Decide at the wave-3 checkpoint alongside D-14/D-15: promote it as ADR-0040 (widening D-20), or record it explicitly deferred with an owner in the close-out's advancing note. Either way the disposition must be **stated**, not silent. |

---

## Validation Sign-Off

- [ ] Every ledger row's verdict cites the command or `file:line` that produced it (D-00e)
- [ ] Wave-merge integrity check re-run after each fan-out plan (120 rows, added == deleted)
- [ ] Sampling continuity: no ledger section marked complete without its own re-run
- [ ] `mdbook build` before/after comparison recorded, with the pre-existing failure named
- [ ] `git diff --name-only … -- '*.rs' | wc -l` → `0` asserted at close-out
- [ ] All three Manual-Only rows dispositioned at the wave-3 checkpoint (none left silent)
- [ ] `nyquist_compliant: true` set in frontmatter

**Approval:** pending
</content>
