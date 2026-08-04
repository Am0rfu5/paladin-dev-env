---
phase: 01-ground-truth-decision-records
verified: 2026-07-31T16:46:51Z
status: passed
score: 5/5 must-haves verified
behavior_unverified: 0
overrides_applied: 0
re_verification:
  previous_status: gaps_found
  previous_score: 3/5
  gaps_closed:
    - "Six ADRs exist, one per competing variant pair (BattalionConfig, BattalionResult, Formation minimum Paladin count, temperature range, Herald trait signature, coverage gate), each naming the chosen variant and the shipped code it was checked against."
    - "The coverage question has one number and one scope, so Phase 3 can objectively pass or fail against it instead of choosing between 80% and 85%."
  gaps_remaining: []
  regressions: []
deferred: []
---

# Phase 1: Ground Truth & Decision Records Verification Report

**Phase Goal:** `.planning/` describes the v0.7.0 code as it actually is, and each of the six contested definitions has exactly one recorded, evidence-cited answer that later milestones can build on.
**Verified:** 2026-07-31T16:46:51Z
**Status:** passed
**Re-verification:** Yes — after gap closure (plans 01-09, 01-10, 01-11, 01-12)

## Goal Achievement

### Observable Truths (ROADMAP.md Success Criteria)

| # | Truth | Status | Evidence |
|---|-------|--------|----------|
| 1 | A developer can open one status ledger and see, for every outstanding Milestone-1 task item, whether shipped code already satisfies it — each verdict carrying a `file:line` citation rather than a task-list checkbox. | ✓ VERIFIED (unchanged) | `.planning/ledgers/milestone-01.md` still holds sections for all 10 epics + unit-test-improvements, its `## Outstanding item reconciliation` still states 39/39 (independently re-summed: 19+4+4+3+3+2+2+2+2(garrison correction unaffected)=39, confirmed at the ledger's own per-file table lines 472-491). Plan 01-11's edits (the new `REQ-battalion-result-v1` row and the verdict-distribution correction) did not touch this section — no regression. |
| 2 | Six ADRs exist, one per competing variant pair (`BattalionConfig`, `BattalionResult`, Formation minimum Paladin count, temperature range, `Herald` trait signature, coverage gate), each naming the chosen variant and the shipped code it was checked against. | ✓ VERIFIED | All six now exist: `0001`..`0006`. Re-ran `node .claude/gsd-core/bin/lib/adr-parser.cjs --input <file>` against all six — every one returns `status: accepted` with non-empty `decisions`/`options_considered`/`key_files` (0001: 21/13/10, 0002: 25/17/7, 0003: 20/17/4, 0004: 38/23/5, 0005: 15/8/8, 0006: 98/30/7). `0006-coverage-gate.md` names its chosen variant (84% hard-fail floor, workspace default-feature scope) and the shipped code it was checked against (`.github/workflows/integration-tests.yml:117-118`'s `cargo llvm-cov` invocation, named as the not-yet-reproduced Docker-backed scope). |
| 3 | The ledger records the places where shipped code has already superseded an ingested requirement — MCP Streamable-HTTP in place of SSE, Sanctum/Qdrant in place of `sqlite-vss`, the interactive REPL that Epic 9 declared a non-goal — so no later phase mistakes divergence for a defect. | ✓ VERIFIED (unchanged) | All three rows still present and untouched in `## Divergences` (`milestone-01.md` lines 40-42) — confirmed byte-for-byte unaffected by plans 01-09..01-12. |
| 4 | The coverage question has one number and one scope, so Phase 3 can objectively pass or fail against it instead of choosing between 80% and 85%. | ✓ VERIFIED | ADR-0006 records exactly one number (84% floor, from a measured 84.79%) and one explicit scope (workspace default-feature, Docker-backed integration-tests scope named as not-yet-reached rather than silently merged in). `grep -n "60.88\|67.79" .planning/ROADMAP.md` returns **zero** matches — the stale two-baseline literals are gone. ROADMAP.md Phase 3 success criterion 1 now reads "at or above the hard-fail floor recorded in [ADR-0006](.planning/decisions/0006-coverage-gate.md)" and cites RECON-07/D-08 by name. The 80%-vs-85% dispute is explicitly resolved as "80% retired as a superseded historical aspiration; exactly one number (84%) is binding" — a deliberate, disclosed deviation from D-09 rather than a silent drop (see Deviation Assessment below). |
| 5 | The Epic 10 Task 7.0 dispute is answered — either the Final Documentation Review is outstanding work with an owner, or the validation report is recorded as wrong — and the 102-vs-103 subtask discrepancy is explained. | ✓ VERIFIED (unchanged) | `## Epic 10 Task 7.0 — dispute resolution (RECON-08)` section untouched by plans 01-09..01-12; re-confirmed present with its verdict and arithmetic explanation. |

**Score:** 5/5 ROADMAP success criteria verified (0 present-but-behavior-unverified)

### The measurement's honesty (highest-value check — verified in full)

- **Raw pasted stdout, not prose-only.** `01-coverage-measurement.md`'s "Measurement of record" section (lines 164-541) contains the full, unedited `llvm-cov report` per-file table (line 253 onward) ending in a `TOTAL` row at line 531: `TOTAL ... 93834 11888 87.33% 7546 1710 77.34% 61404 9340 84.79% 0 0 -`. This is pasted tool output, not a restated figure.
- **Arithmetic self-consistency confirmed independently.** `(61404 − 9340) / 61404 = 0.847936... → 84.79%`, exact match to the stated percentage.
- **Full provenance present.** `rustc -vV` (`rustc 1.97.1`, host `x86_64-unknown-linux-gnu`, LLVM 22.1.6), `cargo --version` (`cargo 1.97.1`), `git rev-parse HEAD` (`9be788c8e9c744ec3a6aad20b64110fb85925de4`), and `date -u` (`2026-07-31T14:57:11Z`) are all recorded verbatim immediately before the measurement commands (lines 166-176).
- **Measured source tree confirmed byte-identical to the phase's pre-gap-closure baseline.** `git show --stat 9be788c` (the commit the measurement cites as HEAD) shows exactly one file changed: `01-coverage-measurement.md`, 162 insertions, 0 deletions. `git diff --stat d2714b7 9be788c` confirms the same — the only change between the initial-verification baseline commit and the measurement commit is the measurement documentation itself. No Rust source changed between the two, so the 84.79% figure is valid for the tree the initial verification also inspected.
- **Human checkpoint genuinely gated, not auto-approved.** Task 3 of plan 01-09 was a `checkpoint:human-verify gate="blocking-human"` that the plan's own instructions say must never be auto-approved even under active auto-mode — confirmed halted and resumed only after an explicit "approved" resume signal, recorded with approver, UTC timestamp and scope restatement at `01-coverage-measurement.md:569-582`.
- **A premature/incorrect completion was caught and reverted, not silently shipped.** Commit `799c53f` ("revert premature RECON-07 completion — ADR-0006 does not exist yet") shows that at one point during plan 01-09's execution REQUIREMENTS.md's RECON-07 checkbox and Traceability row were flipped to Complete before ADR-0006 existed; this was caught and reverted 2 lines / 2 places, restoring `[ ]`/`Pending` until plan 01-12 correctly re-flipped it after ADR-0006 was authored and parsed. This is evidence the process self-corrects rather than a defect that survived to the current state.

**Verdict: the 84.79%/84% figure is real, reproducible-on-paper, and traceable end to end. Not fabricated, not estimated.**

### Deviation Assessment: D-09's 80%-as-target clause retired

D-09 specified "80% recorded in the same ADR as the target" alongside the floor. Because the actual measurement (84.79%) already exceeds 80%, a human directed (outside a plan-internal decision) that 80% be retired as a superseded historical aspiration rather than recorded as a live number — avoiding a self-contradictory ADR (a floor above its own target) and avoiding handing Phase 15 two numbers. ADR-0006 documents this explicitly, by name, in its own Decision section ("The target: superseded... this is a deviation from D-09, recorded as one") and in its Considered Options ("Retaining 80% as a recorded target alongside the 84% floor (D-09's literal text) — rejected as this ADR's deviation from D-09"). This meets the phase's honesty standard: the deviation is named, reasoned, and dated rather than silently absorbed. **Assessed as an honest, adequately-documented deviation, not a gap.**

### Requirement IDs Cross-Reference (RECON-01..08)

| Requirement | REQUIREMENTS.md checkbox | Status | Evidence |
|---|---|---|---|
| RECON-01 | `[x]` | ✓ SATISFIED | Ledger + divergences + bookkeeping; `REQ-battalion-result-v1` gap (noted in prior verification) closed by plan 01-11 — row now present at `milestone-01.md:280`, sourced verbatim from ADR-0002. |
| RECON-02 | `[x]` | ✓ SATISFIED | ADR-0001 parses, unchanged. |
| RECON-03 | `[x]` | ✓ SATISFIED | ADR-0002 parses, unchanged. |
| RECON-04 | `[x]` | ✓ SATISFIED | ADR-0003 parses, unchanged. |
| RECON-05 | `[x]` | ✓ SATISFIED | ADR-0004 parses, unchanged. |
| RECON-06 | `[x]` | ✓ SATISFIED | ADR-0005 parses, unchanged. |
| RECON-07 | `[x]` | ✓ SATISFIED | ADR-0006 exists, parses (`status: accepted`), and both REQUIREMENTS.md surfaces (checkbox at line 186, Traceability table at line 3801) now read satisfied/Complete — flip gated on `adr-parser.cjs` exiting 0, per plan 01-12's own recorded discipline. |
| RECON-08 | `[x]` | ✓ SATISFIED | Unchanged, re-confirmed present. |

**8 of 8 RECON requirement IDs are satisfied** — a clean improvement from the prior 7/8.

### Required Artifacts

| Artifact | Expected | Status | Details |
|----------|----------|--------|---------|
| `.planning/decisions/0001-battalion-config.md` .. `0005-herald-trait.md` | RECON-02..06 ADRs | ✓ VERIFIED (unchanged) | Re-parsed clean, all `status: accepted`. |
| `.planning/decisions/0006-coverage-gate.md` | RECON-07 ADR | ✓ VERIFIED (new) | Exists, parses `status: accepted`, 98 decisions / 30 options / 7 key_files. Names the chosen variant (84% floor, no operative target) and the shipped code checked against (`ci.yml` / `integration-tests.yml`, both confirmed to carry no coverage gate today — `Code Conformance: must change`). |
| `.planning/phases/01-ground-truth-decision-records/01-coverage-measurement.md` | Raw coverage evidence | ✓ VERIFIED (new) | 609 lines; tracer section, measurement-of-record section with full pasted `llvm-cov report` stdout, and a Human-confirmation section. See "The measurement's honesty" above. |
| `.planning/ledgers/milestone-01.md` | Milestone 1 cited status ledger | ✓ VERIFIED | `REQ-battalion-result-v1` row now present (line 280); verdict-distribution table's arithmetic re-verified: 100+23+11+20+1 = 155, matching its own stated total (113 REQ rows + 39 nested items + 3 Divergences rows). unit-test-improvements scope note refreshed to cite ADR-0006 by path. |
| `.planning/PROJECT.md` | Precedence order + Key Decisions table | ✓ VERIFIED | Key Decisions table now holds all six real ADR-linked rows — no "Pending" placeholder remains (confirmed via grep). ADR-0006 row cites the measured 84.79%/84%-floor figure and names `must change (PIPE-02...)` as Outcome, matching the ADR's own Code Conformance verdict. |
| `.planning/REQUIREMENTS.md` | Milestone 1 ledger body reduced to pointer | ✓ VERIFIED | Body reduced to a 9-line pointer at `milestone-01.md` (confirmed lines 2520-2529). Independently re-ran the subset check against the pre-reduction table (recovered via `git show 4a8dee3^:.planning/REQUIREMENTS.md`): 112 source `REQ-*` row IDs extracted, all 112 present in the current ledger, **zero missing** — matches the plan's own claimed 112/112 exactly. |
| `.planning/ROADMAP.md` | Phase 3 criterion amended to one coverage number | ✓ VERIFIED | Criterion 1 amended exactly as claimed; `git diff d2714b7 HEAD -- .planning/ROADMAP.md` shows only the Wave 6-8 checkbox flips and the single criterion-1 line replaced — no collateral changes. All 16 phase headings and Phase 3's other 4 criteria confirmed intact. |
| `.planning/WINDOWS.md` | Broken-windows register | ✓ VERIFIED | `open_count: 0`, item 1 `status: fixed` with a `resolved_at` timestamp and an empty `reason` (a repair, not a waiver). |

### Key Link Verification

| From | To | Via | Status | Details |
|------|-----|-----|--------|---------|
| `.planning/PROJECT.md` Key Decisions | `.planning/decisions/0001-...0006-*.md` | Markdown link per row | ✓ WIRED | All 6 links present and resolve; ADR-0006's Outcome cell matches its own Code Conformance verdict. |
| `.planning/ROADMAP.md` Phase 3 SC1 | `.planning/decisions/0006-coverage-gate.md` | Amendment citing the ADR | ✓ WIRED | Criterion 1 now links to the ADR by relative path and names RECON-07/D-08 as the amending authority. |
| `.planning/ledgers/milestone-01.md` | `.planning/REQUIREMENTS.md` (subset of `REQ-*` IDs) | "supersedes" relationship (D-17) | ✓ WIRED | Independently re-confirmed strict superset: 112/112 source IDs present in the destination (see Required Artifacts row above), closing the prior `⚠️ PARTIAL` finding. |
| `01-coverage-measurement.md` | `.planning/decisions/0006-coverage-gate.md` | Transcription of the measured figure | ✓ WIRED | `grep -c '84.79%' .planning/decisions/0006-coverage-gate.md` → 11 occurrences, all byte-identical to the measurement record's TOTAL row. |
| `.planning/REQUIREMENTS.md` RECON-07 (both surfaces) | `.planning/decisions/0006-coverage-gate.md` | Status flip gated on ADR existing and parsing | ✓ WIRED | Commit `404e3b6` flips only the checkbox (line 186) and Traceability row (line 3801); the ADR existed and parsed cleanly before this commit (`2635601`, `480b553` precede it). |

### Requirements Coverage

| Requirement | Source Plan | Description | Status | Evidence |
|---|---|---|---|---|
| RECON-01 | 01-01, 01-05, 01-06, 01-07, 01-11 | Cited status ledger + divergences + bookkeeping | ✓ SATISFIED | `REQ-battalion-result-v1` gap closed; no open items remain. |
| RECON-02..06 | 01-01, 01-02, 01-03 | BattalionConfig/Result, Formation min count, temperature, Herald trait ADRs | ✓ SATISFIED | Unchanged, re-confirmed. |
| RECON-07 | 01-09, 01-10, 01-12 (supersedes halted 01-04) | Coverage gate ADR | ✓ SATISFIED | ADR-0006 authored from a real offline measurement; REQUIREMENTS.md flipped in both surfaces. |
| RECON-08 | 01-05 | Epic 10 Task 7.0 dispute | ✓ SATISFIED | Unchanged, re-confirmed. |

**8 of 8 RECON requirement IDs satisfied.** No orphaned requirements found.

### Anti-Patterns Found

| File | Line | Pattern | Severity | Impact |
|------|------|---------|----------|--------|
| — | — | — | — | No `TBD`/`FIXME`/`XXX`/`TODO`/`HACK`/unreferenced-placeholder markers found in any file this gap-closure round modified (`.planning/decisions/0006-coverage-gate.md`, `01-coverage-measurement.md`, `milestone-01.md`, `PROJECT.md`, `ROADMAP.md`, `REQUIREMENTS.md`, `WINDOWS.md`). `ROADMAP.md`'s pre-existing `Plans: TBD` markers for Phases 2-16 are unrelated to Phase 1's scope and predate this gap-closure round — not new debt. |

No blockers found. The phase's process itself surfaced and corrected one internal error during execution (the premature RECON-07 flip in plan 01-09, caught and reverted in commit `799c53f` before this verification ran) — evidence the phase's own discipline works, not a defect surviving to the final state.

### Probe Execution

No `scripts/*/tests/probe-*.sh` files exist and none are declared in any Phase 1 PLAN/SUMMARY. SKIPPED (no probes) — unchanged from initial verification.

### Behavioral Spot-Checks

| Behavior | Command | Result | Status |
|---|---|---|---|
| All 6 ADRs parse via GSD's own ADR parser | `node .claude/gsd-core/bin/lib/adr-parser.cjs --input <file>` for each of 0001-0006 | All 6 exit with `"status": "accepted"` and non-empty `decisions`/`options_considered`/`key_files` | ✓ PASS |
| Coverage measurement TOTAL row arithmetic is exact | Manual computation `(61404-9340)/61404` | `0.847936... → 84.79%`, exact match | ✓ PASS |
| Measurement commit touches only documentation | `git show --stat 9be788c` and `git diff --stat d2714b7 9be788c` | Both show exactly 1 file changed (`01-coverage-measurement.md`, +162/-0); no Rust source touched | ✓ PASS |
| `REQ-battalion-result-v1` now present in ledger | `grep -c "REQ-battalion-result-v1" .planning/ledgers/milestone-01.md` | 1 (was 0 in initial verification) | ✓ PASS |
| Independent re-run of the REQUIREMENTS.md→ledger subset check | Extracted 112 source `REQ-*` row IDs from the pre-reduction table (recovered via `git show`), diffed against the current ledger's `REQ-*` IDs | 0 missing — 112/112 present | ✓ PASS |
| Stale 60.88%/67.79% baselines removed from ROADMAP.md | `grep -n "60.88\|67.79" .planning/ROADMAP.md` | No matches | ✓ PASS |
| ROADMAP.md diff scoped exactly as claimed | `git diff --stat d2714b7 HEAD -- .planning/ROADMAP.md` and full diff | Only Wave 6-8 checkbox flips + Phase 3 criterion 1 replaced; all 16 phase headings and other 4 criteria untouched | ✓ PASS |
| Verdict-distribution arithmetic in the ledger | Manual sum `100+23+11+20+1` | 155, matches ledger's own stated total (113 REQ rows + 39 nested + 3 Divergences) | ✓ PASS |

## Gaps Summary

None. All five ROADMAP.md success criteria are now met. The two gaps recorded in the initial verification (2026-07-31T13:31:33Z) — ADR-0006/coverage-measurement missing (blocking criteria 2 and 4), and the `REQ-battalion-result-v1` ledger completeness gap — are both closed, with evidence independently re-derived in this pass rather than accepted from SUMMARY.md claims alone:

1. **RECON-07 / ADR-0006** — closed by plans 01-09 (real offline measurement, human-approved 84.79% figure with full command/toolchain/commit provenance) and 01-10 (ADR-0006 authored from that measurement, with an honestly-disclosed deviation from D-09's literal 80%-target clause).
2. **`REQ-battalion-result-v1` ledger gap** — closed by plan 01-11 (row added, subset check independently re-verified 112/112, REQUIREMENTS.md pointer reduction completed, WINDOWS.md item repaired).
3. **Downstream wiring** — closed by plan 01-12 (PROJECT.md, ROADMAP.md, REQUIREMENTS.md's two RECON-07 surfaces, and the ledger's scope note all now point at ADR-0006 consistently).

Plan 01-04 remains formally unexecuted (no SUMMARY) but is honestly annotated at `ROADMAP.md:177` as superseded by plans 01-09/01-10, which used an offline measurement path 01-04 did not have available. This is a disclosed supersession, not a silently dropped plan, and does not constitute a gap.

**Phase 1's goal is achieved:** `.planning/` now describes the v0.7.0 code as it actually is, and all six contested definitions (`BattalionConfig`, `BattalionResult`, Formation minimum Paladin count, temperature range, `Herald` trait signature, coverage gate) each have exactly one recorded, evidence-cited answer. Ready to proceed to Phase 2.

---

*Verified: 2026-07-31T16:46:51Z*
*Verifier: Claude (gsd-verifier)*
