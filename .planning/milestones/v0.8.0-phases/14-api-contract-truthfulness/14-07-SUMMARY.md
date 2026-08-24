---
phase: 14-api-contract-truthfulness
plan: 07
subsystem: docs
tags: [requirements, ledger, adr-index, closure, provenance, decision-records]

# Dependency graph
requires:
  - phase: 14-api-contract-truthfulness
    provides: "plan 14-01's ADR-0040 forward references (BearerTokenAuthConfig rename), plan 14-04's ADR-0041 forward references (startup warning + fail-closed test), plan 14-05's ADR-0040/0041, plan 14-02's flag fix + sidecar route fix, plan 14-03's reachability rustdoc, plan 14-06's ADR-0042"
provides:
  - "Six ledger rows in .planning/ledgers/milestone-09-12.md amended in place, dated 2026-08-12, citing ADR-0040/0041/0042 and the closing plans/commits/tests"
  - "WEB-01..04 checked in REQUIREMENTS.md with Complete traceability, each behind a named closing plan and citable artefact"
  - "WEB-02's manifest citation corrected at source (D-08) with an explicit basis-for-closure note"
  - "WEB-03's closure carries plan 02-02's --auto provenance qualification (D-00i, D-14) in both the ledger and REQUIREMENTS.md, independently"
  - "Both Phase 13 hand-off items (sidecar route, fail-closed test) closed with dated provenance"
  - "PROMOTION.md's ADR index advanced with rows for 0040/0041/0042 and the next-free-ADR-number line advanced from 0040 to 0043"
affects: ["14-08 (release bookkeeping — version bump, CHANGELOG finalization)"]

# Tech tracking
tech-stack:
  added: []
  patterns:
    - "Dated in-place amendment retaining superseded text (D-00c/D-00d), applied to ledger table rows and REQUIREMENTS.md checkbox items alike — new dated paragraphs inserted around original text, never rewriting it"
    - "Provenance-qualification stated independently in two documents (ledger row + REQUIREMENTS.md checkbox) rather than one canonical location, so neither can read cleaner than the other (T-14-26's mitigation)"

key-files:
  created: []
  modified:
    - .planning/ledgers/milestone-09-12.md
    - .planning/REQUIREMENTS.md
    - .planning/decisions/PROMOTION.md

key-decisions:
  - "REQ-jwt-bearer-auth-v2 amended to a new subclass 'Shipped, superseded (auth vocabulary)' rather than reusing 'Shipped, superseded (route text)' — the legend's 'Shipped, superseded' class permits a descriptive parenthetical, matching the precedent REQ-agent-execute-endpoint's row already set for route text"
  - "REQ-llm-tool-calling-port and REQ-llm-tool-calling-adapters keep 'Verified open' as their capability half's verdict class after amendment, rather than inventing a 'Deferred' class — the capability is still literally absent from the tree (confirmed absent, not a defect), and the legend's eleven classes have no 'deferred with ADR' verdict; ADR-0042's deferral decision is recorded as the citation attached to the existing class, not a new class"
  - "WEB-03's closure paragraph in REQUIREMENTS.md was placed immediately after the requirement's heading line, before the original prose, rather than after the *Derives* line (the position used for WEB-01/WEB-02/WEB-04) — the original 12-line prose block already fills a 12-line-after-heading window, so a closure note placed after *Derives* would fall outside that window and fail the plan's own 'grep -A12 ... auto' acceptance check; placing it first keeps the qualification unmissable by any reader scanning from the top and costs nothing since D-00c requires retention, not position"
  - "PROMOTION.md's Part B inventory (the eleven-candidate list) was left untouched — the plan's own action text scopes this plan's PROMOTION.md edit to the Numbering index table, the next-free-ADR-number line and its dated note; ADR-0041 and ADR-0042 are not promotions of pre-existing Part B ingest candidates (only ADR-0040 corresponds to Part B candidate 10, and updating that row was not named in the plan's action text or acceptance criteria, so it was left as-is to avoid unauthorised scope creep into a section the plan explicitly says stays untouched)"

requirements-completed: [WEB-01, WEB-02, WEB-03, WEB-04]

coverage:
  - id: D1
    description: "Six ledger rows (REQ-opaque-bearer-token-adapter-v1, REQ-jwt-bearer-auth-v2, REQ-fail-closed-auth-posture, REQ-k8s-manifests, REQ-llm-tool-calling-port, REQ-llm-tool-calling-adapters) amended in place, dated, citing ADR-0040/0041/0042, with superseded text retained and the row count unchanged"
    requirement: "WEB-01"
    verification:
      - kind: other
        ref: "grep -c '^| REQ-' .planning/ledgers/milestone-09-12.md -> 120 (unchanged before/after); grep -c 'ADR-0040' -> 2; grep -c 'ADR-0041' -> 2; grep -c 'ADR-0042' -> 4; git diff -- .planning/ledgers/milestone-09-12.md | grep -c '^-| REQ-' -> 6, matching grep -c '^+| REQ-' -> 6 (no row inserted or deleted)"
        status: pass
    human_judgment: false
  - id: D2
    description: "REQ-llm-tool-calling-adapters' flag half carries plan 02-02's --auto provenance qualification, not a bare Complete"
    requirement: "WEB-03"
    verification:
      - kind: other
        ref: "grep 'REQ-llm-tool-calling-adapters' .planning/ledgers/milestone-09-12.md | grep -o 'auto' | wc -l -> 2"
        status: pass
    human_judgment: false
  - id: D3
    description: "All four WEB-01..04 checkboxes checked in REQUIREMENTS.md, traceability rows read Complete"
    requirement: "WEB-01"
    verification:
      - kind: other
        ref: "grep -c '^- \\[x\\] \\*\\*WEB-0[1-4]\\*\\*' .planning/REQUIREMENTS.md -> 4; grep -c '^- \\[ \\] \\*\\*WEB-0[1-4]\\*\\*' -> 0; grep -E '^\\| WEB-0[1-4] ' .planning/REQUIREMENTS.md -> all four rows read Complete"
        status: pass
    human_judgment: false
  - id: D4
    description: "WEB-02's manifest citation corrected at source with a dated banner, original retained, plus an explicit basis-for-closure note"
    requirement: "WEB-02"
    verification:
      - kind: other
        ref: "grep -q 'ADR-0041' .planning/REQUIREMENTS.md; git diff --numstat -- .planning/REQUIREMENTS.md shows only checkbox/traceability-row lines deleted, all WEB-02 correction/closure text is pure addition"
        status: pass
    human_judgment: false
  - id: D5
    description: "WEB-03's REQUIREMENTS.md closure note names plan 02-02's --auto provenance, independent of the ledger row"
    requirement: "WEB-03"
    verification:
      - kind: other
        ref: "grep -A12 '\\*\\*WEB-03\\*\\*' .planning/REQUIREMENTS.md | grep -ci 'auto' -> 2"
        status: pass
    human_judgment: false
  - id: D6
    description: "PROMOTION.md's ADR index carries one row per record (0040, 0041, 0042), next-free-ADR-number advances exactly once to 0043 with a dated note, procedure section untouched"
    requirement: "WEB-04"
    verification:
      - kind: other
        ref: "grep -c 'Next free ADR number: 0043' .planning/decisions/PROMOTION.md -> 1; grep -q '| 0040 |' / '| 0041 |' / '| 0042 |' all pass; test -e on all three ADR files all pass; git diff -- .planning/decisions/PROMOTION.md shows changes only above the 'Required heading set' section"
        status: pass
    human_judgment: false
  - id: D7
    description: "Both Phase 13 hand-off items (sidecar route fix, fail-closed test) closed with dated provenance naming their closing plans"
    verification:
      - kind: other
        ref: "grep -n 'Closed, dated 2026-08-12 (plan 14-02)' and 'Item 5 above (\\`REQ-fail-closed-auth-posture\\`) — closed, dated 2026-08-12 (plan 14-04)' both present in .planning/REQUIREMENTS.md's Phase 14 hand-off block"
        status: pass
    human_judgment: false

# Metrics
duration: ~50min
completed: 2026-08-12
status: complete
---

# Phase 14 Plan 07: Ledger and Requirements Close-Out Summary

**Amended six ledger rows and closed all four WEB requirements at source, citing ADR-0040/0041/0042 with dated provenance — including carrying plan 02-02's `--auto` provenance forward independently in both the ledger and REQUIREMENTS.md — and advanced PROMOTION.md's ADR index from 0040 to 0043.**

## Performance

- **Duration:** ~50 min
- **Started:** 2026-08-12 (worktree dispatch)
- **Completed:** 2026-08-12
- **Tasks:** 2
- **Files modified:** 3 (`.planning/ledgers/milestone-09-12.md`, `.planning/REQUIREMENTS.md`, `.planning/decisions/PROMOTION.md`)

## Accomplishments

- Amended six ledger rows in `.planning/ledgers/milestone-09-12.md` in place, each dated 2026-08-12, superseded text retained:
  - `REQ-opaque-bearer-token-adapter-v1` (`Contract diverges` → `Shipped`, citing ADR-0040 and commits `1f8a1d1`/`51deeeb`)
  - `REQ-jwt-bearer-auth-v2` (`Contract diverges` → `Shipped, superseded (auth vocabulary)`, citing ADR-0040's dissolution of Open Question 4)
  - `REQ-fail-closed-auth-posture` (`Verified open` → `Shipped`, citing plan 14-04's `build_auth_config_fails_closed_when_enabled_with_no_credentials` test; the disabled-mode warning is stated explicitly as still unexercised)
  - `REQ-k8s-manifests` (correctness question answered by ADR-0041; replica count unchanged and explained as correct, not an open gap)
  - `REQ-llm-tool-calling-port` (capability half stays `Verified open`, now citing ADR-0042's deferral with a named trigger and owner)
  - `REQ-llm-tool-calling-adapters` (flag half stays `Shipped → WEB-03` with `--auto` provenance carried forward per D-00i; capability half stays `Verified open`, citing ADR-0042)
- Updated the ledger's forward-scope pointer table (both tool-calling rows) to route a reader to ADR-0042 instead of an open Phase 14 pointer.
- Recomputed the ledger's row total (120, unchanged) and verdict distribution by counting, not assuming unchanged — appended a dated `## Phase 14 close-out amendments (2026-08-12)` section recording the exact counts, the counting commands, and the class-by-class delta from Phase 13's own distribution table.
- Checked all four `WEB-01`..`WEB-04` boxes in `.planning/REQUIREMENTS.md` (WEB-02 and WEB-03 were already checked by plans 14-04/14-02; WEB-01 and WEB-04 checked by this plan) and set all four traceability-table rows to `Complete`.
- Added a dated correction banner (D-08) to WEB-02's own text, naming `k8s/server/deployment.yaml`/`k8s/server/service.yaml` as the correct manifest pair, with the original `k8s/deployment.yaml`/`k8s/service.yaml` citation retained and marked superseded, plus an explicit basis-for-closure note (ROADMAP criterion 2's second clause plus ADR-0041's reasoning, since WEB-02's own literal "done when" exits were not taken).
- Added WEB-03's dated closure paragraph naming plan 02-02's `--auto` provenance independently of the ledger row (T-14-26's mitigation — the acceptance criteria check both locations separately so neither can read cleaner than the other).
- Closed both Phase 13 hand-off items with dated provenance: the sidecar route fix (plan 14-02, naming the outstanding `/gsd-secure-phase 13` re-run for the orchestrator/user) and the fail-closed test (plan 14-04).
- Advanced `PROMOTION.md`'s ADR index: added rows for 0040/0041/0042 matching the form of the 0037-0039 rows, advanced `**Next free ADR number:**` from 0040 to 0043, and added a dated note (2026-08-12, plan 14-07) explaining the three-step advance and naming which plan authored each record. The procedure sections (`Required heading set`, `Supersession mechanism`, `Promotion procedure for existing ADR candidates` including Part A and Part B) are untouched.

## Task Commits

1. **Task 1: Amend the six ledger rows in place, dated, with provenance carried forward** — `1b664ec` (docs)
2. **Task 2: Close WEB-01..04 at source, correct WEB-02's manifest citation, and advance the ADR index** — `5c23958` (docs)

**Plan metadata:** this SUMMARY's commit follows (worktree mode — STATE.md/ROADMAP.md are updated by the orchestrator after all wave agents complete).

## Files Created/Modified

- `.planning/ledgers/milestone-09-12.md` — six rows amended in place; forward-scope pointer table updated; new dated `## Phase 14 close-out amendments (2026-08-12)` section appended with recomputed row total and verdict distribution
- `.planning/REQUIREMENTS.md` — WEB-01/WEB-04 checkboxes checked; WEB-01/WEB-02/WEB-03/WEB-04 dated closure paragraphs added; WEB-02's manifest-citation correction banner added; traceability table rows set to `Complete`; both Phase 13 hand-off items closed with dated provenance
- `.planning/decisions/PROMOTION.md` — three new ADR index rows (0040/0041/0042); next-free-ADR-number advanced to 0043 with a dated explanatory note; procedure sections untouched

## Decisions Made

- `REQ-jwt-bearer-auth-v2`'s amended verdict uses a new descriptive parenthetical (`Shipped, superseded (auth vocabulary)`) under the legend's existing `Shipped, superseded` class, following the precedent `REQ-agent-execute-endpoint`'s row already set for `(route text)` — no sixth verdict class was invented; every verdict used (`Shipped`, `Shipped, superseded`, `Verified open`) appears in the ledger's own legend.
- Both `REQ-llm-tool-calling-port` and `REQ-llm-tool-calling-adapters`' capability halves keep `Verified open` as their verdict class after amendment. The capability is genuinely still absent from the tree — `Verified open`'s own legend definition ("confirmed absent from the tree, checked directly rather than inferred") remains accurate — so the amendment adds ADR-0042's deferral citation to the existing class rather than inventing a "Deferred" class the legend does not define.
- WEB-03's REQUIREMENTS.md closure paragraph was placed immediately after the requirement's heading line rather than after the `*Derives*` line (the position used for WEB-01/WEB-02/WEB-04). WEB-03's original prose is exactly 12 lines long, already filling the plan's own `grep -A12` acceptance-check window; a closure note placed after `*Derives*` would fall outside that window. Placing it first satisfies the check and costs nothing under D-00c/D-00d, which require the original text retained, not any particular position relative to the addition.
- `PROMOTION.md`'s Part B inventory (the eleven-candidate list, including candidate 10's "opaque-bearer-token decision... Owner phase: Phase 14" row) was left untouched. The plan's own action text scopes this plan's `PROMOTION.md` edit to the Numbering index table, the next-free-ADR-number line, and its dated note — it does not instruct a Part B update, and the acceptance criteria do not check for one. Updating Part B candidate 10 to add a "Closed by ADR-0040" note (matching the pattern Phase 13 used for candidates 7 and 8) would be defensible but is out of this plan's stated scope; left for a future phase or explicit instruction rather than assumed.
- Removed a duplicate WEB-03 closure paragraph that was drafted twice during editing (once at the top per the `grep -A12` fix, once at the original end-of-entry position) before committing — kept only the top-of-entry copy, verified via `git diff` that no duplicate text remains in the committed version.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 1 - Bug] WEB-03's closure paragraph placement corrected to satisfy the plan's own `grep -A12` acceptance check**
- **Found during:** Task 2, self-verification of the acceptance criteria after the first draft edit
- **Issue:** The plan's own acceptance criterion — `grep -A12 '**WEB-03**' .planning/REQUIREMENTS.md | grep -ci 'auto'` must be at least 1 — could not be satisfied if the closure paragraph (naming plan 02-02's `--auto` provenance) was appended after the requirement's existing 12-line prose block and its `*Derives*` line, because that already fills the entire 12-line-after-heading window the check inspects.
- **Fix:** Moved the closure paragraph to sit immediately after the WEB-03 heading line, before the original prose, rather than at the end of the entry (the position used for the other three WEB requirements). Original prose is fully retained, unmodified, just no longer the first thing a reader meets.
- **Files modified:** `.planning/REQUIREMENTS.md`
- **Verification:** `grep -A12 '**WEB-03**' .planning/REQUIREMENTS.md | grep -ci 'auto'` → 2 (was 0 before the fix); `git diff --numstat -- .planning/REQUIREMENTS.md` still shows deletions only on checkbox/traceability-row lines.
- **Committed in:** `5c23958` (Task 2 commit)

---

**Total deviations:** 1 auto-fixed (Rule 1 — a self-contradicting first draft, caught and corrected before commit via the plan's own acceptance-criteria check).
**Impact on plan:** No scope change; the fix is a repositioning of already-planned content within the same requirement entry, still purely additive per D-00c/D-00d.

## Issues Encountered

- Two `Edit` string-match failures on the ledger's `REQ-k8s-manifests` row, caused by a smart-quote/plain-quote mismatch between the string typed by hand and the file's actual `"Disabled for testing")` text — resolved by re-reading the exact line via `grep -n` before retrying, not a deviation from the plan, just a mechanical correction to the edit tool invocation.
- WEB-02 and WEB-03 in `.planning/REQUIREMENTS.md` were already checked `[x]` with `Complete` traceability before this plan ran (by sibling plans 14-04 and 14-02 respectively, per this plan's own dispatch context). This plan's job for those two was narrower than for WEB-01/WEB-04: add the missing dated closure/correction content (WEB-02's manifest-citation banner and basis-for-closure note; WEB-03's `--auto` provenance note) rather than flip a checkbox. Confirmed via `git diff` that no checkbox line for WEB-02/WEB-03 appears in this plan's diff — only WEB-01 and WEB-04's checkbox lines changed.

## User Setup Required

None — no external service configuration required.

## Known Stubs

None. This is a planning-record-only plan; no code, test, or UI surface was touched.

## Threat Flags

None beyond the plan's own threat model, which is addressed by construction: T-14-26 (WEB-03's closure record) is mitigated by the `--auto` provenance appearing independently in both the ledger row and the REQUIREMENTS.md closure note, each verified by a separate grep. T-14-27 (WEB-02's checkbox) is mitigated by the explicit basis-for-closure note citing ROADMAP criterion 2's second clause and ADR-0041's reasoning, rather than implying the literal "done when" text was satisfied. T-14-28 (ledger row counts) is mitigated by the counted (not assumed) row total and verdict distribution, recorded with the counting commands in the ledger's own new close-out section. T-14-29 (`PROMOTION.md`'s next-free number) is mitigated by this plan being the phase's single writer of that file, advancing the line exactly once, and every newly-indexed ADR slug resolving on disk via `test -e`.

## Next Phase Readiness

- All four WEB-01..04 requirements are closed at source with named, citable evidence. The six ledger rows this phase's dependency count on are amended, dated, and correctly cite ADR-0040/0041/0042.
- `PROMOTION.md`'s next-free ADR number is 0043, ready for plan 14-08 or any subsequent phase to take the next number without an `ls` sweep.
- Outstanding, not this plan's job: re-running `/gsd-secure-phase 13` to formally record T-13-20 moving from `accept`/`AR-13-01` to `closed` (named as an outstanding action in both `14-02-SUMMARY.md` and this plan's own REQUIREMENTS.md closure note) — owned by the orchestrator/user after this wave merges, since `13-SECURITY.md` is outside every Phase 14 plan's `files_modified` scope.
- No blockers for plan 14-08's release bookkeeping (version bump to 0.8.0, CHANGELOG finalization) — this plan touched no manifest and no `.rs` file.

---
*Phase: 14-api-contract-truthfulness*
*Completed: 2026-08-12*

## Self-Check: PASSED

All three claimed modified files verified present on disk (`test -f`, three calls) plus this
SUMMARY itself. All three commit hashes (`1b664ec`, `5c23958`, `bfb2ec9`) verified present in
`git log --oneline --all`.
