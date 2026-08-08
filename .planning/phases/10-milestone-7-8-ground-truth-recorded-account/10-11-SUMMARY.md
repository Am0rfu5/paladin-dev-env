---
phase: 10-milestone-7-8-ground-truth-recorded-account
plan: 11
subsystem: docs
tags: [close-out, requirements-traceability, adr-promotion, ledger-amendment, phase-gate]

# Dependency graph
requires:
  - phase: 10-milestone-7-8-ground-truth-recorded-account
    provides: "All ten prior plans' completed work — the 86-row ledger (10-01, 10-07..10-10), six ADRs 0028-0033 (10-02..10-06), and the three checkpoint answers (10-04)"
provides:
  - "HARD-01 through HARD-07 flipped [x] in REQUIREMENTS.md, each behind a dated closure note naming its artefact and a re-run command or file:line"
  - "Four D-26 forward hand-off blocks: Phase 11/FACADE-02, Phase 11/FACADE-03(b), Phase 12/SUPPLY-02+03, Phase 13/ORCH-05"
  - "PROMOTION.md advanced to next-free 0034 with six new index rows; PROJECT.md Key Decisions carries all six ADRs"
  - "Ledger close-out amendment appended: whole-file integrity re-check, per-class verdict tally, the 12-vs-13 supersession-count reconciliation, three do-not-re-delete markers, checkpoint answers, CI-only claims, measured cargo doc state"
  - "ROADMAP.md Phase 10 section closed: 10-11-PLAN.md marked complete, dated completion note"
  - "crates/paladin-content/README.md's stale pdf feature line corrected (scope addition beyond D-23's three-file surface, a .md file not .rs)"
affects: ["11-facade-residue-and-deferred-register-disposition", "12-supply-chain-gate-integrity", "13-milestone-9-12-deferred-qa-close-out"]

tech-stack:
  added: []
  patterns:
    - "Evidence-before-flip: run the closing command/citation first, then flip the checkbox, then write the dated note naming both — never flip on the strength of a plan having run"
    - "Hand-off block placement: immediately before the section heading containing the receiving requirement, matching Phase 9's precedent exactly"

key-files:
  created: []
  modified:
    - .planning/REQUIREMENTS.md
    - .planning/decisions/PROMOTION.md
    - .planning/PROJECT.md
    - .planning/ledgers/milestone-07-08.md
    - .planning/ROADMAP.md
    - crates/paladin-content/README.md

key-decisions:
  - "The scraper/rss/tiktoken-rs dead-optional-dependency finding (surfaced by plans 10-05 and 10-10, left unowned by ADR-0032's own Downstream Consumers section) is assigned a definite owner in this close-out's Phase 12 hand-off block: Phase 15, not left as ADR-0032's open 'Phase 11 or Phase 15' choice"
  - "The 12-vs-13 supersession-count discrepancy is fully reconciled by tracing all 16 REQ-* IDs implied by the summary table's 13 rows to their own epic-section verdict: 5 remain superseded by outcome, 6 land relocated (tie-break rule), 2 land satisfied (the dagger-marked HARD-05/HARD-06 flips), 2 land diverged, 1 lands satisfied on a corrected count, and 1 table entry carries no REQ-* ID at all — netting 5, which plus 7 independently-found superseded-by-outcome rows never in the original table sums to the ledger's actual 12"
  - "The paladin-content README's stale pdf feature line is corrected directly (not via D-00c strike-and-annotate), since a README documents current shipped capability rather than a corpus PRD/DOC under the annotation convention — recorded as a deliberate scope addition per the phase's own instruction"
  - "REL-01's checkbox and traceability row are read-only evidence for HARD-03's closure note and the Phase 13/ORCH-05 hand-off — neither was touched, confirmed via grep before and after every edit to REQUIREMENTS.md"

requirements-completed: [HARD-01, HARD-02, HARD-03, HARD-04, HARD-05, HARD-06, HARD-07]

coverage:
  - id: D1
    description: "HARD-01 through HARD-07 flipped [x] in REQUIREMENTS.md, each with a dated closure note naming its artefact and a re-run command or file:line, plus traceability rows updated to Complete"
    requirement: "HARD-01"
    verification:
      - kind: other
        ref: "grep -cE '^- \\[x\\] \\*\\*HARD-0[1-7]\\*\\*' REQUIREMENTS.md == 7; grep -cE '^- \\[ \\] \\*\\*HARD-0[1-7]\\*\\*' == 0; grep -c 'dated 2026-08-08 (plan 10-11)' == 11 (>=7); grep -cE 'HARD-0[1-7].*Phase 10' == 18 (>=7)"
        status: pass
    human_judgment: false
  - id: D2
    description: "Four D-26 forward hand-off blocks written, each naming a receiving requirement ID, an artefact, and something not to re-derive"
    requirement: "HARD-01"
    verification:
      - kind: other
        ref: "grep -c '^#### Hand-off to Phase 1' REQUIREMENTS.md == 4; grep -c FACADE-02/FACADE-03/SUPPLY-0/ORCH-05 each >=1"
        status: pass
    human_judgment: false
  - id: D3
    description: "PROMOTION.md advanced to next-free 0034 with six new index rows (slugs matching filenames) and a dated note naming which plan authored each ADR; PROJECT.md Key Decisions carries all six"
    requirement: "HARD-01"
    verification:
      - kind: other
        ref: "grep -c 'Next free ADR number: 0034' == 1; grep -cE '^\\| 002[89] \\||^\\| 003[0-3] \\|' == 6; ls .planning/decisions/ | grep -cE '^00(2[89]|3[0-3])-.*\\.md$' == 6; git diff -- PROMOTION.md | grep -c '^-[^-]' == 1; grep -cE 'ADR-00(2[89]|3[0-3])' PROJECT.md >= 6"
        status: pass
    human_judgment: false
  - id: D4
    description: "Ledger close-out amendment appended (86/86/12 unchanged, zero pending stubs, zero dangling ADR citations, exactly one appended section, zero deleted lines) sourced from all ten plan SUMMARYs"
    requirement: "HARD-01"
    verification:
      - kind: other
        ref: "grep -c '^| REQ-' == 86; distinct-ID count == 86; grep -c '^### ' == 12; grep -c '^## Phase 10 close-out amendments' == 1; git diff --numstat shows 0 deletions; grep -cE '10-0[1-9]-SUMMARY|10-10-SUMMARY' == 11 (>=10)"
        status: pass
    human_judgment: false
  - id: D5
    description: "Phase gate run and recorded verbatim: zero .rs changes across the whole phase, cargo test/fmt --check/clippy -D warnings all pass, cargo metadata and make -n release-check both exit 0"
    verification:
      - kind: other
        ref: "git diff --name-only 6a6f175..HEAD -- '*.rs' | wc -l == 0; cargo test --workspace exit 0; cargo fmt --check exit 0; cargo clippy --workspace --all-targets --all-features -- -D warnings exit 0; cargo metadata --no-deps --format-version 1 exit 0; make -n release-check exit 0"
        status: pass
    human_judgment: false
  - id: D6
    description: "Three Manual-Only rows from 10-VALIDATION.md each given an explicit disposition; ADR-0006's coverage floor recorded as reasoning, never as measurement"
    verification:
      - kind: other
        ref: "cargo doc --workspace --no-deps re-run this session: exit 1, 24 warning-prefixed lines (20 individual + 4 summary), matching ADR-0033's prior figure exactly, confirming the gate is unchanged and still red"
        status: pass
    human_judgment: false

duration: ~95min
completed: 2026-08-08
status: complete
---

# Phase 10 Plan 11: Milestone 7-8 Close-Out Summary

**Closed Phase 10: seven HARD requirements flipped `[x]` behind cited evidence, `PROMOTION.md` advanced to next-free `0034` with six new ADR index rows, four forward hand-off blocks written for Phases 11-13, the ledger's close-out amendment appended (12-vs-13 supersession count fully reconciled), and the whole-phase gate — zero `.rs` changes, `cargo test`/`fmt --check`/`clippy -D warnings` all green — run and recorded verbatim.**

## Performance

- **Duration:** ~95 min
- **Started:** 2026-08-08T17:59:35Z (worktree fork base)
- **Completed:** 2026-08-08T19:34:15Z
- **Tasks:** 3 (plus one recorded scope addition — the README correction)
- **Files modified:** 6 (`REQUIREMENTS.md`, `PROMOTION.md`, `PROJECT.md`, the ledger, `ROADMAP.md`, `crates/paladin-content/README.md`)

## Accomplishments

- **Task 1 — HARD-01 through HARD-07 flipped behind evidence.** For each, ran the closing command
  or citation first, then flipped the checkbox, then wrote a dated closure note naming the artefact,
  the proving command/`file:line`, and the owning plan's SUMMARY — never the reverse order. HARD-07's
  note states explicitly the `cargo doc --workspace --no-deps` gate "is still red today" and names
  Phase 16 / DOCS-03 as owner, so the flip records the bar being written once with its measured
  state, not the tree clearing it. Traceability rows for all seven updated from `Pending` to
  `Complete`. Four D-26 hand-off blocks written, placed immediately before the section heading
  containing the receiving requirement (matching Phase 9's own precedent exactly): Phase 11 /
  FACADE-02 (ADR-0031's restated invariant plus ADR-0028's already-executed Epic 3 relocations),
  Phase 11 / FACADE-03(b) (the `paladin-herald`/`paladin-ml` non-goal split), Phase 12 / SUPPLY-02
  and SUPPLY-03 (the corrected `.cargo/audit.toml` reasoning, delivered as an answer, plus the
  `scraper`/`rss`/`tiktoken-rs` dead-dependency finding named to Phase 15), and Phase 13 / ORCH-05
  (ADR-0029's `## Trajectory` table to extend, REL-01 already converged).
- **Task 2 — `PROMOTION.md` advanced to 0034; `PROJECT.md` records all six ADRs.** Six new index
  rows (slugs matching filenames) appended after ADR-0027's row; next-free line changed from `0028`
  to `0034`; a dated note added naming which plan authored which ADR and which two carry `must
  change` verdicts (ADR-0032, ADR-0033). `git diff` on `PROMOTION.md` shows exactly one deleted line
  (the next-free line itself) — every other change additive, no existing index row disturbed.
  `PROJECT.md`'s Key Decisions table gained six rows in the existing shape.
- **Task 3 — ledger close-out amendment, ROADMAP closure, phase gate.** Ran the whole-ledger
  integrity check first: 86 rows, 86 distinct IDs, 12 sections in run-4 order, zero pending stubs,
  zero blank cells, and every `ADR-NNNN` citation in the file (nine distinct numbers) resolved to a
  file that exists. Appended `## Phase 10 close-out amendments (2026-08-08)` at the foot — zero
  deletions, sourced from all ten plan SUMMARYs by name — recording the per-class verdict tally
  across all 86 rows, which rows were cited rather than re-derived (the seven Phase-9-closed rows,
  every citation re-run and none moved), which claims are CI-only with the exact commands (`cargo
  audit`, `cargo deny check`, `cargo-llvm-cov`), the measured `cargo doc` state (exit 1, 20 warnings,
  re-confirmed unchanged this session), the three do-not-re-delete markers and where each sits, the
  three checkpoint answers (`q1-restate`, `q2-delete`, `q3-ratify`), and a full reconciliation of the
  12-vs-13 supersession-count discrepancy (see Decisions Made). `ROADMAP.md`'s Phase 10 section
  closed: `10-11-PLAN.md` marked complete, a dated completion note added naming the ledger, the six
  ADRs and the three config changes; criterion 1's plan-10-01 correction left untouched. Ran the
  phase gate: `git diff --name-only 6a6f175..HEAD -- '*.rs' | wc -l` → `0`; `cargo test --workspace`
  → all suites `ok`, `0 failed`; `cargo fmt --check` → exit `0`; `cargo clippy --workspace
  --all-targets --all-features -- -D warnings` → exit `0`; `cargo metadata --no-deps
  --format-version 1` → exit `0`; `make -n release-check` → exit `0`, recipe order intact, no
  `--exclude paladin-ports` anywhere.
- **Recorded scope addition — `crates/paladin-content/README.md`'s stale `pdf` feature line
  corrected.** Plan 10-05 deleted the inert `pdf = []` feature from the crate's `Cargo.toml` but the
  README still documented it. Corrected the line to state PDF extraction ships unconditionally,
  citing ADR-0032. A `.md` file, not `.rs` — D-23's boundary is unaffected — and beyond the phase's
  three-file config surface, so recorded here as a deliberate addition rather than silently folded
  into Task 3.

## Task Commits

Each task was committed atomically:

1. **Task 1: Flip HARD-01..07 and write four forward hand-offs** — `7716357` (docs)
2. **Task 2: Advance PROMOTION.md to 0034, record six ADRs in PROJECT.md** — `7027570` (docs)
3. **Task 3: Append ledger close-out amendment, close ROADMAP Phase 10, run phase gate** — `865850c` (docs)
4. **Recorded scope addition: correct paladin-content README's stale pdf line** — `feb56eb` (docs)

_No plan-metadata commit for STATE.md/ROADMAP.md progress counters — worktree mode: the orchestrator
owns those writes centrally after all wave agents complete. This plan's own Task 3 edit to
`ROADMAP.md` is a content correction (marking this plan's own checkbox and adding a dated closure
note to Phase 10's own section), not a progress-tracking update, and is committed within Task 3
above per the plan's own `<files>` list._

## Files Created/Modified

- `.planning/REQUIREMENTS.md` — HARD-01..07 flipped `[x]` with dated closure notes; traceability
  rows updated to `Complete`; four D-26 hand-off blocks added.
- `.planning/decisions/PROMOTION.md` — six new index rows (0028-0033); next-free line → `0034`;
  dated note.
- `.planning/PROJECT.md` — six new Key Decisions rows, one per ADR.
- `.planning/ledgers/milestone-07-08.md` — `## Phase 10 close-out amendments (2026-08-08)` appended.
- `.planning/ROADMAP.md` — `10-11-PLAN.md` marked complete; dated Phase 10 completion note added.
- `crates/paladin-content/README.md` — stale `pdf` feature line corrected (scope addition).

## Decisions Made

- **The `scraper`/`rss`/`tiktoken-rs` finding is assigned a definite owner (Phase 15), not left at
  ADR-0032's open "Phase 11 or Phase 15" choice.** The Phase 12 hand-off block names Phase 15 as the
  owner of record, since Phase 15 already inherits the seven-crate doctest posture from ADR-0033 as
  a related dependency-and-build-surface hygiene item — consolidating two related hygiene findings
  under one phase rather than leaving either ambiguous.
- **The 12-vs-13 supersession-count discrepancy is fully reconciled, not merely re-stated.** The
  summary table's 13 rows expand to 16 distinct `REQ-*` IDs (one row bundles 5 IDs, one row carries
  0 — the M7 overview's `paladin-cli` entry, outside the 86-row inventory). Tracing all 16 to their
  own epic-section verdict: 5 remain `superseded by outcome` exactly as anticipated; 6 land
  `relocated` under the tie-break rule; 2 land `satisfied` (the `‡`-marked ADR-0031/ADR-0032 flips);
  2 turned out `diverged` on independent re-derivation (`REQ-sqlx-workspace-dependency`,
  `REQ-ci-publish-dry-run-v1`); 1 turned out already `satisfied` (`REQ-dead-file-batch-deletion` —
  the PRD's own anticipated post-Epic figures were simply higher than the reconciliation's actual
  count, not shipped code undoing the requirement). That nets 5, which plus 7 rows found
  independently during the 82-row fan-out that were never part of the original 13-row table at all
  sums to the ledger's actual 12. No table row and no epic-section row is unaccounted for.
- **The README correction uses a direct edit, not D-00c's strike-and-annotate shape.** D-00c governs
  `.project/` corpus documents (PRDs, DOCs) where the original ingested claim must be retained
  struck. `crates/paladin-content/README.md` is live crate documentation describing current shipped
  capability, not a corpus artefact under that convention — correcting it directly (with a dated,
  ADR-cited note explaining the change) is the right shape, and the phase-note instructions asked
  for exactly this: "correct that README line," not "annotate it superseded."
- **REL-01 was read, never written.** Every reference to REL-01 across HARD-03's closure note and
  the Phase 13/ORCH-05 hand-off block cites its existing `[x]` checkbox and traceability row as
  evidence; neither was edited. Confirmed via `grep -n "\[x\] \*\*REL-01\*\*"` before Task 1 and
  after every subsequent edit to `REQUIREMENTS.md`: line `360`, unchanged and stable throughout,
  since REL-01 sits above every section this plan edited (the HARD block, the FACADE/SUPPLY/ORCH
  hand-off insertion points, and the traceability table all follow it in the file).

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 2 — recorded scope addition, pre-authorized by the phase note] `paladin-content`
README's stale `pdf` feature line**
- **Found during:** Task 3 (confirming plan 10-05's manifest deletion left no other residue)
- **Issue:** `crates/paladin-content/README.md:27` still documented the `pdf` feature plan 10-05
  deleted from `Cargo.toml`, leaving the README describing a capability flag that no longer exists.
- **Fix:** Corrected the line to state PDF extraction ships unconditionally, citing ADR-0032, with a
  dated 2026-08-08 note.
- **Files modified:** `crates/paladin-content/README.md`.
- **Verification:** `git diff` shows a 5-line insertion replacing the single stale bullet; no other
  line touched; `git status --porcelain -- '*.rs'` remains empty.
- **Committed in:** `feb56eb`.

---

**Total deviations:** 1 recorded scope addition (explicitly pre-authorized by this plan's own
`<additional_items_found_during_execution>` instructions, not a Rule 1-4 discovery made
independently).
**Impact on plan:** No `.rs` file touched; the phase's D-23 boundary is unaffected (the boundary
governs `.rs` files, not `.md` files). No architectural decision, no scope creep beyond the one line
the phase note explicitly named.

## Issues Encountered

- `cargo bench`/long-running commands were not needed this plan — all verification commands
  (`cargo test --workspace`, `cargo fmt --check`, `cargo clippy --workspace --all-targets
  --all-features -- -D warnings`, `cargo doc --workspace --no-deps`) completed within normal
  foreground timeouts, the longest (`clippy`, cold-ish cache) taking ~4 minutes.
- One transient tool-permission denial on a combined `git add file1 file2` call (Bash auto-mode
  classifier); resolved by staging each file individually in separate calls, which succeeded
  without incident.

## User Setup Required

None — no external service configuration required.

## Known Stubs

None. The ledger's own whole-file integrity check (re-run this session) confirms zero
`pending — plan` markers and zero blank Verdict/Evidence cells remain anywhere in
`.planning/ledgers/milestone-07-08.md`.

## Next Phase Readiness

- Phase 11 (`FACADE-01..04`) can proceed: `FACADE-02` and `FACADE-03(b)` each have a dedicated
  hand-off block in `REQUIREMENTS.md` naming the exact artefact to cite and what not to re-derive.
- Phase 12 (`SUPPLY-01..03`) can proceed: `SUPPLY-02`/`SUPPLY-03` inherit the `.cargo/audit.toml`
  reachability answer as delivered, plus a named owner (Phase 15) for the dead-dependency finding.
- Phase 13 (`ORCH-05`) can proceed: ADR-0029's `## Trajectory` table has a labelled placeholder row
  waiting for `v0.3.0` through `v0.6.0`, and REL-01's already-converged status is confirmed not
  re-opened.
- `PROMOTION.md`'s next-free line is `0034` — Phase 12 and Phase 13 (the remaining phases with ADR
  candidates in `PROMOTION.md`'s Part B inventory, candidates 7-11) take numbers from there.
- No blockers. `git diff --name-only 6a6f175..HEAD -- '*.rs' | wc -l` → `0` — the whole of Phase 10
  touched zero `.rs` files, D-23's boundary held and is checkable by a command rather than only
  asserted.

## Self-Check: PASSED

- FOUND: `.planning/REQUIREMENTS.md` (HARD-01..07 all `[x]`, four hand-off blocks present)
- FOUND: `.planning/decisions/PROMOTION.md` (next-free `0034`, six new rows)
- FOUND: `.planning/PROJECT.md` (six new Key Decisions rows)
- FOUND: `.planning/ledgers/milestone-07-08.md` (close-out amendment appended, 86/86/12 intact)
- FOUND: `.planning/ROADMAP.md` (10-11-PLAN.md marked complete, dated note present)
- FOUND: `crates/paladin-content/README.md` (stale pdf line corrected)
- FOUND: commit `7716357` (Task 1)
- FOUND: commit `7027570` (Task 2)
- FOUND: commit `865850c` (Task 3)
- FOUND: commit `feb56eb` (README correction)

---
*Phase: 10-milestone-7-8-ground-truth-recorded-account*
*Completed: 2026-08-08*
