---
phase: 11-facade-residue-deferred-register-disposition
verified: 2026-08-09T00:39:13Z
status: human_needed
score: 9/9 must-haves verified (record-only domain; behavior_unverified: 0 — no state-transition/cancellation-invariant truths in this phase)
overrides_applied: 0
requirements_coverage:
  - id: FACADE-01
    status: satisfied
  - id: FACADE-02
    status: satisfied
  - id: FACADE-03
    status: satisfied
  - id: FACADE-04
    status: satisfied
human_verification:
  - test: "Confirm the FACADE-02 cross-register backstop truth: that user_service.rs resolves to exactly three non-overlapping owners (split withdrawn/no owner, full relocation owned by the run-3 v2 tech-debt item, tests owned by DEFER-02/Phase 15) with no file left proposing two incompatible next actions."
    expected: "A human reads ADR-0034's D2 sub-decision, the run-3 v2 tech-debt item, and Deferred-QA Epic 28/DEFER-02's entry side by side and confirms the three owners are distinct and non-overlapping."
    why_human: "must_haves.truths marks this `verification: backstop` and 11-VALIDATION.md independently classes it manual-only — no single command proves a cross-document non-collision. This verifier found strong supporting evidence (ADR-0034 lines 106-118 name all three owners explicitly, and REQUIREMENTS.md lines 2314-2339, 3255-3281, 4076 corroborate the collision this withdrawal resolves) but per the backstop-truth protocol this abstains to human_needed rather than silently passing on inference."
  - test: "Confirm the ADR-allocation checkpoint (plan 11-01, Task 2) was an acceptable resolution."
    expected: "A human reviews option-a (two ADRs: 0034 for D1-D4, 0035 for paladin-ml placement) and confirms it was the right call, since the plan's own coverage table flags this item `human_judgment: true`."
    why_human: "The `checkpoint:decision` gate was auto-selected by the orchestrator under AUTO_MODE=true rather than independently reasoned by a human reviewer (11-01-SUMMARY.md's own D5 coverage entry states this explicitly). The selection is transparently recorded and evidence-backed, but was not human-confirmed at the time it was made."
  - test: "Spot-check the 8 judgment-tier prohibitions across the 5 plans (all authored descriptor-less, no check_* wiring) for any silent violation this verifier's greps could have missed."
    expected: "Each prohibition (`must not delete or rewrite the text it corrects`, `ADR-0031's unratified status must not be laundered`, `the CLI reintroduction record must not imply purely mechanical re-wiring`, `the ROADMAP amendment must remain falsifiable, not retro-fitted`) holds under closer human reading of the full annotated documents, not just the grep/diff evidence this verifier collected."
    why_human: "Per the prohibition-handling protocol, judgment-tier prohibitions with no wired check are NON-AUTHORITATIVE when confirmed only by an LLM judge (this verifier). This verifier's own LLM-judge disposition for all 8 is PASS (see below), backed by `git diff --numstat` zero-deletion evidence and targeted greps, but the protocol requires this be flagged for human review rather than silently folded into a passed verdict."
---

# Phase 11: Facade Residue & Deferred Register Disposition Verification Report

**Phase Goal:** Everything Milestone 8 deliberately left behind has a decision rather than a rating — the five deferred items, the two removed features and their reintroduction conditions — and the Milestone 9 candidate list is triaged so nobody re-plans relocations that already happened.
**Verified:** 2026-08-09T00:39:13Z
**Status:** human_needed
**Re-verification:** No — initial verification

**Domain note:** This phase deliberately touched zero `.rs` files (D-13). `git diff --name-only b417326..HEAD | grep -c '\.rs$'` returns `0`, confirmed independently below. Verification here is a re-run of the shell assertions the plans and 11-VALIDATION.md specify (grep counts, `ls`, `git log`/`git show`, row counts) against the live tree, not a code-behavior test.

## Goal Achievement

### Observable Truths

| # | Truth | Status | Evidence |
|---|-------|--------|----------|
| 1 | FACADE-01: all 17 rustdoc-stdout occurrences carry a per-occurrence disposition, `deferred-items.md` D5 and ROADMAP criterion 1 corrected in place, originals retained | ✓ VERIFIED | `grep -rn "println!\|eprintln!\|dbg!" src/application/services/ src/infrastructure/ \| wc -l` → 17; filtered → 0. All 17 `file:line` citations present in the register (17/17 checked, none missing). `deferred-items.md`: `Effort / risk` still occurs 6× (5 clauses + 1 in the correction banner quoting it), `2026-08-08` present. ROADMAP.md:726 retains the original criterion-1 sentence with the dated amendment and "Original criterion text retained above." appended. |
| 2 | FACADE-02: D1-D4 each carry a verb + named owner (+ trigger where deferred) in ADR-0034; `content_service.rs`/`event_manager.rs` get independent verdicts; ADR-0031's unratified status is carried forward, not laundered; no relocation executes | ✓ VERIFIED | ADR-0034 has the 7 required headings in order, no frontmatter (`head -1` is the title line). `user_service.rs` (18×), `content_service.rs` (9×), `event_manager.rs` (10×) all cited with independent verdicts. `6704807`, `run-3 v2`, `DEFER-02` all present in the D2 sub-decision. `⚠ HUMAN REVIEW`/`never been human-ratified` present at lines 66 and 254. `find src/core -name "*.rs" | wc -l` → 6, `grep -rl "crate::core::" src/ | wc -l` → 49, matching the ADR's cited figures. `deferred-items.md`'s D1-D4 clauses all annotated (Shape A), D5 section untouched by plan 11-02's commit (confirmed via `git show a14db07` diff — the D5 heading appears only as unchanged context). |
| 3 | FACADE-03: the `paladin user register` answer and the ML placement condition are both reachable from `.planning/` with an immutable SHA recovery pointer; one commit removed both features; the corpus's branch-vs-commit attribution is corrected on honestly re-measured evidence | ✓ VERIFIED | `find crates/paladin-ports -iname "*ml_port*"` → `crates/paladin-ports/src/input/ml_port.rs`. Register cites `git show 3d48768^:src/application/cli/commands/user.rs` in runnable form, all 8 subcommands named, `1,065`/`636` LOC present, security note present ("not a purely mechanical re-wiring exercise from a security standpoint"). ADR-0035 has the 7 required headings, no frontmatter, reproduces the placement condition verbatim (`leaf crate`, `never back into the facade`, `MlPort`, `stays in the workspace` all present), asymmetric non-goal split stated both directions. **WR-01 (git-ancestry-direction bug flagged in 11-REVIEW.md) is fixed in commit `0134374`**: independently re-verified `git merge-base --is-ancestor 3d48768 refs/remotes/origin/chore/facade-cleanup-m8-finish` exits 0 (true) and the reverse exits 1 (false); `git rev-list 3d48768..refs/remotes/origin/chore/facade-cleanup-m8-finish --count` returns 14 — the corrected banner text in `deferred-features.md:20` ("3d48768 is an ancestor of that ref... 14 commits past it") matches exactly. No local branch of that name exists (`git branch --list '*facade-cleanup-m8-finish*'` returns empty), matching the corrected text. |
| 4 | FACADE-04: every surviving row of the 20-row M9 candidate table is triaged `done`/`not a candidate`/`still open` against ADR-0028's range and the live tree, not the record's own claims; `paladin-arsenal`/`paladin-sanctum` recorded as artefacts; zero-still-open stated as a finding; rows 1, 14, 17, 19 carry their specific corrections | ✓ VERIFIED | Triage register has exactly 20 numbered rows; disposition-word tally counted live: 14 `done`, 6 `not a candidate`, 0 `still open` (matches the register's own stated `## Tally`). `ls crates/` → 11 entries, `paladin-arsenal`/`paladin-sanctum` both absent (`grep -c` on the `ls crates/` output → 0). Row 1/19 each cite `prd-relocate-remaining-misplaced-modules` (4 total citations, ≥2 required). Row 14 names both `error_log_adapter`/`system_log_adapter`. Row 17 carries "do not re-delete" + `ca7e4e8`. `## Zero rows are still open` and `## The two crate names are artefacts` sections both present. Source document `infrastructure-adapter-disposition.md` carries exactly one banner sentence (`grep -c` → 1) with the triage pointer added, and the 20-row table/`Date: 2025-01` header untouched. |
| 5 | Phase close-out: all five named ledger rows amended in place citing this phase's artefacts (row count unchanged at 86), REQUIREMENTS.md's FACADE-01/FACADE-03(a) corrected at source (purely additive), PROMOTION.md advanced to 0036 as the phase's last act with a dated note, PROJECT.md gains 2 Key Decisions rows | ✓ VERIFIED | `grep -c '^| REQ-' milestone-07-08.md` → 86 (unchanged). All 5 named rows carry `plan 11-05` (5 matches), `pending — plan 11-05` marker is gone (0 matches), plan 11-01's earlier amendment retained (1 match). Recovery pointer present in ledger in runnable form. Verdict-cell vocabulary for the 5 amended rows all legend words (`deferred with register` ×3, `superseded by outcome` ×2). REQUIREMENTS.md: both original clauses retained (`chore/facade-cleanup-m8-finish` and `` converted to `log::*` `` both still present), 2 dated `Corrected 2026-08-08 (plan 11-05)` passages, `git diff --numstat` on the correcting commit shows 24 insertions / 0 deletions. PROMOTION.md: 35 index rows, ascending/contiguous, `Next free ADR number: 0036`, dated note present with the `ls` proof and the Part-B non-closure statement, rows 0001-0033 byte-identical. PROJECT.md `## Key Decisions` table: 31 rows (was 29), both new rows link to the correct ADR files with `conforms` outcomes matching each ADR's own Code Conformance verdict. |
| 6 | No Rust source file modified anywhere in the phase (D-13) | ✓ VERIFIED | `git diff --name-only b417326..HEAD | grep -c '\.rs$'` → 0. Full file list for the phase's entire commit range (23 commits) touches only `.planning/`, `.project/` and phase-tracking files. |
| 7 | FACADE-02 backstop truth: `user_service.rs` resolves to three non-overlapping owners across ADR-0034, the run-3 v2 item and Deferred-QA Epic 28, with no file left proposing two incompatible next actions | ⚠ evidence found, not independently confirmable by command — routed to human_needed | `verification: backstop`. Strong supporting evidence collected (ADR-0034 D2 sub-decision names all three owners explicitly at lines 106-118; REQUIREMENTS.md's DEFER-02 entry at 2314-2339 and the forward-coupling table at 4076 describe the same collision this withdrawal resolves) but per the backstop-truth protocol and 11-VALIDATION.md's own manual-only classification, this abstains from a silent VERIFIED and is routed to human verification below. |
| 8 | Judgment-tier prohibitions (8 total across the 5 plans, all descriptor-less / no `check_*` wiring) hold — no correction deleted or rewrote the text it corrects; ADR-0031's status is not laundered; the CLI reintroduction record is not presented as purely mechanical | LLM-judge: PASS (non-authoritative) — routed to human_needed | Every "no deletion" prohibition independently confirmed via `git diff --numstat` (0 deletions on every annotating commit checked: `deferred-items.md` D5+D1-D4, `deferred-features.md`, `infrastructure-adapter-disposition.md`, `REQUIREMENTS.md`) and targeted greps for original text survival. ADR-0031 non-laundering confirmed at ADR-0034 lines 66/254. CLI security note confirmed present in the facade-03 register. Per protocol, an unwired judgment-tier prohibition disposition from this verifier alone is non-authoritative and must be flagged for human review rather than silently counted toward a `passed` verdict. |
| 9 | The one unresolved FACADE-03 edge (`unclassified` probe row) stays surfaced as an explicit flagged assumption, not silently auto-backstopped or dropped | ✓ VERIFIED | `11-03-PLAN.md`'s "## Flagged assumptions carried into this plan" section states the `unclassified` probe result and the recorded-deferral-branch choice explicitly (D-09), and the resulting register (`facade-03-removed-features.md`) closes with the stated `## Disposition` line — recorded deferral, no crate created, no feature reintroduced. |

**Score:** 7/9 truths cleanly VERIFIED by command; 2/9 (truths 7 and 8) have strong supporting evidence but are protocol-routed to human_needed rather than auto-passed — no truth FAILED.

### Required Artifacts

| Artifact | Expected | Status | Details |
|----------|----------|--------|---------|
| `.planning/registers/facade-01-rustdoc-stdout-disposition.md` | Per-occurrence D5 disposition register, Phase 15 hand-off, corpus finding | ✓ VERIFIED | Exists, 17/17 citations, Phase 15 named 4×, `## Corpus-level finding` section present. |
| `.planning/decisions/0034-d1-d4-facade-relocation-disposition.md` | ADR: 4 verdicts with owners/triggers | ✓ VERIFIED | 7 headings in order, no frontmatter, all four items verbed and owned. |
| `.planning/registers/facade-03-removed-features.md` | Both removed features, SHA pointer, ADR citation | ✓ VERIFIED | All required strings present (3d48768, runnable git show command, ml_port, paladin-ml, leaf crate, 1,065, 636). |
| `.planning/decisions/0035-paladin-ml-leaf-crate-placement.md` | ADR: paladin-ml placement condition, promoted | ✓ VERIFIED | 7 headings, no frontmatter, condition reproduced verbatim, `conforms` verdict. |
| `.planning/registers/facade-04-m9-candidate-triage.md` | 20-row triage, tally, artefact finding | ✓ VERIFIED | 20 rows, 14/6/0 tally matches live recount, both crate names recorded as artefacts. |
| `.planning/decisions/PROMOTION.md` | 2 index rows, dated note, next-free → 0036 | ✓ VERIFIED | 35 rows, ascending, `Next free ADR number: 0036`. |
| `.planning/ledgers/milestone-07-08.md` | 5 amended rows, 86 unchanged | ✓ VERIFIED | Row count 86 before/after, 5 rows carry `plan 11-05`. |
| `.planning/REQUIREMENTS.md` | FACADE-01/FACADE-03(a) corrected, originals retained | ✓ VERIFIED | 0 deletions, originals present, 2 dated corrections. |
| `.planning/PROJECT.md` | 2 new Key Decisions rows | ✓ VERIFIED | 29 → 31 rows, both `conforms`. |
| `.project/.../deferred-items.md` | D5 (11-01) + D1-D4 (11-02) annotated, Shape A | ✓ VERIFIED | All clauses annotated; D5/D1-D4 boundaries respected (no cross-plan overlap). |
| `.project/.../deferred-features.md` | Shape-B banner, both `.planning/` homes pointed to, WR-01 fixed | ✓ VERIFIED | Exactly one banner; ancestry direction independently re-verified correct post-fix. |
| `.project/.../infrastructure-adapter-disposition.md` | Existing Phase-10 banner extended, never duplicated | ✓ VERIFIED | Exactly one banner sentence; 20-row table and `Date: 2025-01` header untouched. |

### Key Link Verification

| From | To | Via | Status | Details |
|------|----|----|--------|---------|
| `facade-01-rustdoc-stdout-disposition.md` | `deferred-items.md` | cross-citation | ✓ WIRED | Register cites the D5 clause; `deferred-items.md` points back at the register path (≥1 occurrence). |
| `0034-...disposition.md` | `0031-extracted-crate-dependency-rule.md` | Code Locations citation, unratified status carried | ✓ WIRED | `0031-extracted-crate-dependency-rule` cited, `HUMAN REVIEW`/`never been human-ratified` both present. |
| `facade-03-removed-features.md` | `0035-paladin-ml-leaf-crate-placement.md` | register cites ADR as authority | ✓ WIRED | `0035-paladin-ml-leaf-crate-placement` cited in the register. |
| `0035-...placement.md` | `crates/paladin-ports/src/input/ml_port.rs` | Code Locations citation | ✓ WIRED | `ml_port.rs` cited in `## Code Locations`, confirmed the file exists at that path. |
| `facade-04-m9-candidate-triage.md` | `0028-m8-reconciliation-authoritative.md` | `e5b2011` range citation on `done` rows | ✓ WIRED | `e5b2011` present ≥1×. |
| `infrastructure-adapter-disposition.md` | `facade-04-m9-candidate-triage.md` | extended banner pointer | ✓ WIRED | `facade-04-m9-candidate-triage` cited in the extended banner. |
| `milestone-07-08.md` (5 rows) | all 5 phase artefacts | Evidence-cell citations | ✓ WIRED | Each artefact path non-zero-cited in its owning row. |

### Behavioral Spot-Checks

Not applicable — this phase produces records, not runnable code (no test suite, no service, no CLI change). Skipped per the domain note in the verification brief; all "behavioral" evidence for this phase is the re-run shell assertions documented above (equivalent to Step 7b's evidence bar for a record-producing phase).

### Requirements Coverage

| Requirement | Source Plan | Description | Status | Evidence |
|--------------|------------|--------------|--------|----------|
| FACADE-01 | 11-01 | D5 closed via per-occurrence disposition | ✓ SATISFIED | Register + ledger row + REQUIREMENTS.md + ROADMAP all agree on 17/0 and cross-cite. |
| FACADE-02 | 11-02 | D1-D4 disposition with owners | ✓ SATISFIED | ADR-0034; one cross-document truth routed to human review (see above), does not block satisfaction. |
| FACADE-03 | 11-03 | Removed features recorded, reintroduction conditions intact | ✓ SATISFIED | Register + ADR-0035 + corrected ledger/REQUIREMENTS.md; WR-01 fixed post-review. |
| FACADE-04 | 11-04 | M9 candidate list triaged | ✓ SATISFIED | 20-row triage, tally verified, artefact finding recorded. |

**Note (informational, not a gap):** `REQUIREMENTS.md`'s top-level "Requirement Coverage" status table (around line 3990-3993) still lists FACADE-01 through FACADE-04 as "Pending", matching the pattern of every other not-yet-milestone-closed phase in that same table (this status column appears to be updated by a separate close-out/milestone workflow, not by phase-execution plans themselves — no plan in this phase names that table row in its `files_modified` or `must_haves`). Not counted as a gap since it is outside every plan's declared scope.

### Anti-Patterns Found

None. No `TBD`/`FIXME`/`XXX`/`HACK`/`PLACEHOLDER` markers in any of the 5 new/modified `.planning/` artefacts. No unreferenced debt markers. `git diff --numstat` on every annotating commit shows 0 deletions where the plans required purely-additive edits, confirmed independently rather than trusting the SUMMARYs' claims.

### Human Verification Required

### 1. FACADE-02 cross-register backstop truth (user_service.rs three-owner split)

**Test:** Read `.planning/decisions/0034-d1-d4-facade-relocation-disposition.md`'s D2 sub-decision, the run-3 v2 tech-debt item, and Deferred-QA Epic 28 / DEFER-02's entry in `.planning/REQUIREMENTS.md` together.
**Expected:** All three owners (split=withdrawn/nobody, full relocation=run-3 v2, tests=DEFER-02/Phase 15) are distinct and non-overlapping, and no other document proposes a fourth, conflicting action on `user_service.rs`.
**Why human:** Declared `verification: backstop` in the plan's own must_haves and independently classed manual-only by 11-VALIDATION.md — cross-document semantic non-collision cannot be proven by a single command. This verifier found strong, consistent supporting evidence but the protocol requires abstaining from a silent pass.

### 2. ADR-allocation checkpoint (plan 11-01, Task 2) — auto-selected under AUTO_MODE

**Test:** Review option-a's rationale (two ADRs: 0034 for D1-D4, 0035 for `paladin-ml` placement) as recorded in 11-01-SUMMARY.md's "Checkpoint Resolution — Task 2" section.
**Expected:** Confirm the coarser-supersession trade-off accepted by option-a, and the still-unratified ADR-0031 dependency it inherits, are acceptable.
**Why human:** This was a `checkpoint:decision gate="blocking"` task whose own plan text states "The ADR allocation is confirmed by a human before wave 2 burns a number" — but it was resolved by orchestrator auto-selection under `AUTO_MODE=true`, not independent human reasoning. The plan's own coverage table (11-01-SUMMARY.md, coverage id D5) explicitly flags `human_judgment: true` for this reason.

### 3. Judgment-tier prohibitions (8 total, descriptor-less)

**Test:** Spot-check a sample of the 8 "must not delete/rewrite" and "must not launder/understate" prohibitions across the 5 plans against the full annotated `.project/` and `.planning/` documents (not just the grep/diff evidence collected here).
**Expected:** All 8 hold under closer reading — no original text silently lost, ADR-0031's status not presented as settled, the CLI reintroduction record not presented as purely mechanical.
**Why human:** All 8 prohibitions were authored without `check_*` descriptors, so by design they dispose flagged-unverified. This verifier's LLM-judge disposition for all 8 is PASS, backed by `git diff --numstat` zero-deletion evidence and targeted greps, but per the prohibition-handling protocol an unwired judgment-tier disposition from an LLM verifier alone is non-authoritative and must surface for human confirmation rather than being folded silently into a `passed` verdict.

### Gaps Summary

No gaps. Every must-have truth, artifact and key link this verifier could confirm by command checks out exactly as the plans and SUMMARYs claim — independently re-measured, not read off the SUMMARYs. The one pre-existing defect flagged by code review (WR-01, git-ancestry direction reversed in `deferred-features.md`'s banner) was fixed in a follow-up commit (`0134374`) and independently re-verified correct here. The phase's status is `human_needed` rather than `passed` solely because of the record-domain's own designed escalation points — a backstop truth requiring cross-document judgment, an AUTO_MODE-resolved blocking checkpoint, and 8 unwired judgment-tier prohibitions — none of which produced any adverse finding, but all of which the verification protocol requires to be surfaced for a human rather than silently passed.

---

_Verified: 2026-08-09T00:39:13Z_
_Verifier: Claude (gsd-verifier)_
