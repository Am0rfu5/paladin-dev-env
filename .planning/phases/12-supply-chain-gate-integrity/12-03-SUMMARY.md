---
phase: 12-supply-chain-gate-integrity
plan: 03
subsystem: infra
tags: [adr, governance, rustsec, cargo-audit, cargo-deny, supply-chain, ci]

# Dependency graph
requires:
  - phase: 12-supply-chain-gate-integrity
    provides: "12-01's checkpoint RESOLUTION selecting option-a (2026-08-09), authorizing this plan to run; 12-02's D-08 guard script and its Makefile/ci.yml wiring, cited as ADR-0036's enforcement mechanism"
provides:
  - "ADR-0036, `Accepted`/`conforms`, promoting `PROMOTION.md` Part B candidate 7 — the audit-suppression single-source topology invariant"
  - "Four dated correction banners on the passages that claimed this promotion was impossible (SUPPLY-03's own block, REQUIREMENTS.md's eleven-candidate framing, PROJECT.md's Out of Scope bullet and Context eleven-candidate passage)"
affects: [12-04-hand-off]

# Tech tracking
tech-stack:
  added: []
  patterns:
    - "Single-physical-line bullets in ADR `## Code Locations`/`## Considered Options`: adr-parser.cjs's splitEntries splits every non-blank line of the section body, not just markdown-bullet lines, so a wrapped multi-line bullet inflates key_files/options_considered by one spurious fragment per wrapped continuation line. Writing each citation/option as one unwrapped physical line keeps the parsed entry count equal to the visible bullet count (verified: 12 bullets -> 12 key_files, 4 bullets -> 4 options_considered), rather than the multi-line style ADR-0031 used (28 key_files against ~10 real citations)."

key-files:
  created:
    - .planning/decisions/0036-audit-suppression-single-source-topology.md
    - .planning/phases/12-supply-chain-gate-integrity/12-03-SUMMARY.md
  modified:
    - .planning/REQUIREMENTS.md
    - .planning/PROJECT.md

key-decisions:
  - "ADR-0036 adopts the source PRD's §8 two-file framing (`.cargo/audit.toml` AND `deny.toml`) over FR-1's narrower single-file framing, and states the divergence explicitly rather than silently picking one — a topology invariant that only watched `.cargo/audit.toml` would leave `deny.toml`'s `cargo deny` invocations free to carry an undetected inline suppression."
  - "Code Locations and Considered Options bullets were rewritten as single unwrapped physical lines (not ADR-0031's wrapped-bullet style) after the first structural-check run showed key_files=29/options_considered=19 against 13/4 actual bullets — confirming the parser's per-physical-line split behavior inflates any wrapped bullet, independent of the trailing-prose defect ADR-0031 already names."
  - "requirements-completed is deliberately empty in this SUMMARY, and `gsd_run query requirements.mark-complete` was NOT run for SUPPLY-03, even though it is this plan's sole `requirements:` frontmatter entry. The plan's own hard prohibitions are explicit: SUPPLY-03's checkbox and traceability row are plan 12-04's to close, not this plan's — flipping it here would reintroduce the premature closure the orchestrator already reverted once this phase (commit `6916c2f`)."

requirements-completed: []

coverage:
  - id: D1
    description: "ADR-0036 exists, is Accepted/conforms, carries all seven required H2 headings in PROMOTION.md's prescribed order with no frontmatter and no Supersedes line, and passes the adr-parser.cjs structural check"
    requirement: "SUPPLY-03"
    verification:
      - kind: other
        ref: "node .claude/gsd-core/bin/lib/adr-parser.cjs --input .planning/decisions/0036-audit-suppression-single-source-topology.md -> status=accepted, key_files=12, options_considered=4, decisions non-empty, unmapped_headers includes 'Code Conformance' and 'Downstream Consumers' (this execution)"
        status: pass
    human_judgment: false
  - id: D2
    description: "Code Locations and Considered Options are genuine one-bullet-per-line lists (no un-bulleted prose, no wrapped multi-line bullets), verified by sed-range grep and by key_files-count-vs-bullet-count proximity"
    verification:
      - kind: other
        ref: "sed -n '/^## Considered Options/,/^## Code Locations/p' and '/^## Code Locations/,/^## Code Conformance/p' -> zero non-bullet/non-blank lines; key_files(12) == bullet count(12), options_considered(4) == bullet count(4)"
        status: pass
    human_judgment: false
  - id: D3
    description: "ADR-0024 is cited by number, never restated or superseded; its file and Status are untouched"
    verification:
      - kind: other
        ref: "grep -qF 'ADR-0024' 0036-*.md (pass); git status --porcelain -- .planning/decisions/0024-rustsec-exception-governance.md -> empty (this execution)"
        status: pass
    human_judgment: false
  - id: D4
    description: "Every ci.yml line citation in ADR-0036 re-derived against the current tree (plan 12-02 shifted line numbers), not copied from a pre-12-02 artefact"
    verification:
      - kind: other
        ref: "grep -n against .github/workflows/ci.yml this execution: security-audit job 61-78, rationale comment 74-76, bare cargo audit 78, D-08 guard step 103-104, cargo deny check 121 — all confirmed by direct Read of the current file"
        status: pass
    human_judgment: false
  - id: D5
    description: "Four dated correction banners (SUPPLY-03's own block, REQUIREMENTS.md eleven-candidate passage, PROJECT.md Out of Scope bullet, PROJECT.md Context eleven-candidate passage), every original retained verbatim, zero deletions"
    requirement: "SUPPLY-03"
    verification:
      - kind: other
        ref: "git diff --numstat -- .planning/REQUIREMENTS.md .planning/PROJECT.md -> 39/0 and 20/0 (insertions/deletions) this execution; grep -c 'Corrected by Phase 12 (plan 12-03), dated 2026-08-09' summed == 4; all four original phrases still grep-match"
        status: pass
    human_judgment: false
  - id: D6
    description: "SUPPLY-03's checkbox and traceability row, and PROMOTION.md, left untouched for plan 12-04; no .rs file modified"
    verification:
      - kind: other
        ref: "grep -q '^| SUPPLY-03 | Phase 12 | Pending |' REQUIREMENTS.md (pass); git status --porcelain -- .planning/decisions/PROMOTION.md -> empty; git diff --name-only -- '*.rs' | wc -l -> 0 (this execution)"
        status: pass
    human_judgment: false

duration: ~20min
completed: 2026-08-09
status: complete
---

# Phase 12 Plan 03: ADR-0036 Authoring Summary

**ADR-0036 promotes `PROMOTION.md` Part B candidate 7 as `Accepted`/`conforms`, restating the
audit-suppression invariant in the source PRD's §8 two-file framing and citing the D-08 guard as
its enforcement mechanism, plus four dated correction banners retiring the stale "promotion is
impossible" claims across `REQUIREMENTS.md` and `PROJECT.md` — with every original passage kept
verbatim and SUPPLY-03's own closure left for plan 12-04.**

## Performance

- **Duration:** ~20 min
- **Started:** 2026-08-09
- **Completed:** 2026-08-09T13:39:41Z
- **Tasks:** 2 of 2
- **Files modified:** 3 (`.planning/decisions/0036-audit-suppression-single-source-topology.md`
  created; `.planning/REQUIREMENTS.md`, `.planning/PROJECT.md` modified)

## Accomplishments

- Verified all three preconditions by direct read before writing anything: 12-01's checkpoint
  RESOLUTION selected `option-a` (2026-08-09); `.planning/decisions/0036-*.md` did not exist;
  `PROMOTION.md:59` still read `Next free ADR number: 0036`.
- Authored `.planning/decisions/0036-audit-suppression-single-source-topology.md` — H1 title, no
  frontmatter, all seven required H2 headings in `PROMOTION.md`'s order, no `## Supersedes` line.
  `## Status` reads `Accepted`, `## Code Conformance` reads `conforms`.
- Adopted the source PRD's §8 two-file framing (`.cargo/audit.toml` **and** `deny.toml`) over FR-1's
  narrower single-file framing, stating the distinction explicitly in `## Context` rather than
  silently picking one.
- Cited ADR-0024 by number in `## Context`/`## Decision` as the sibling governing suppression
  *contents*, scoping ADR-0036 to *topology* only; never restated any of ADR-0024's decisions; left
  ADR-0024's file and `## Status` untouched (verified via `git status --porcelain`).
- Re-derived every `ci.yml` citation against the current tree rather than copying a pre-plan-12-02
  line number: `security-audit:` job `:61-78`, rationale comment `:74-76`, bare `cargo audit` `:78`,
  the D-08 guard's own CI step `:103-104`, `cargo deny check` `:121`.
- Ran the D-08 guard in this execution (`./scripts/check-workflow-suppressions.sh`) and recorded its
  verbatim transcript inside `## Code Conformance`: `6 workflow file(s) scanned, 109 run step(s)
  examined, 1 cargo audit invocation(s) found; no inline advisory-ignore suppression detected`,
  exit `0`. Cross-checked the ignore-family token census by hand (`--ignore-existing` on `mc mb`
  lines, `--ignored` on `cargo test` lines — neither an advisory suppression).
- Discovered and fixed a parser-fragmentation defect before committing (see Deviations below):
  rewrote `## Code Locations` and `## Considered Options` as single unwrapped physical lines per
  bullet so `adr-parser.cjs`'s `key_files`/`options_considered` counts match the visible bullet
  counts (12/12, 4/4) instead of inflating via wrapped-line fragments.
- Wrote four dated correction banners, each citing ADR-0036 and (where the candidate count is at
  issue) ADR-0024 and `PROMOTION.md:185-189`, on: `REQUIREMENTS.md`'s SUPPLY-03 block (the
  does-not-act clause and the two-candidates phrasing, one combined banner), `REQUIREMENTS.md`'s
  eleven-candidate framing (all three stale claims), `PROJECT.md`'s §Out of Scope bullet, and
  `PROJECT.md`'s §Context eleven-candidate passage. All four originals retained verbatim; `git diff
  --numstat` shows `0` deletions across both files.
- Left `SUPPLY-03`'s checkbox `- [ ]` and traceability row `| SUPPLY-03 | Phase 12 | Pending |`
  untouched, and did not touch `PROMOTION.md` — both are plan 12-04's to close, per this plan's
  explicit prohibitions.
- Confirmed zero `.rs` files touched across both commits.

## Task Commits

Each task was committed atomically:

1. **Task 1: Author ADR-0036 — the audit-suppression single-source topology invariant** -
   `931fc29` (docs)
2. **Task 2: Correct the four passages claiming this promotion is impossible** - `e78eaac` (docs)

## Files Created/Modified

- `.planning/decisions/0036-audit-suppression-single-source-topology.md` — new ADR, `Accepted`,
  `conforms`, seven headings, cites ADR-0024 and the D-08 guard, no `## Supersedes` line
- `.planning/REQUIREMENTS.md` — two dated correction banners (SUPPLY-03's own block; the
  eleven-candidate framing), both citing ADR-0036
- `.planning/PROJECT.md` — two dated correction banners (§Out of Scope bullet; §Context
  eleven-candidate passage), both citing ADR-0036

## Decisions Made

- **§8's two-file framing adopted over FR-1's single-file framing**, stated explicitly in `##
  Context` rather than silently chosen — see key-decisions above.
- **Code Locations and Considered Options rewritten as single-physical-line bullets** after the
  first structural-check run exposed the parser's per-physical-line split behavior inflating
  `key_files` to 29 and `options_considered` to 19 against 13/4 real bullets; the rewritten,
  unwrapped form produces 12/12 and 4/4, well inside the "factor of two" acceptance bar and
  deliberately tighter than ADR-0031's own 28-against-~10 ratio.
- **`requirements-completed: []` in this SUMMARY, and no `requirements mark-complete` call for
  SUPPLY-03** — the plan's hard prohibitions explicitly reserve SUPPLY-03's checkbox and
  traceability row for plan 12-04, and this phase already reverted one premature SUPPLY-03 closure
  (commit `6916c2f`, preceding this plan's own commits in the log).
- **No `PROMOTION.md` numbering-line, index row, or Part B closure note added** — all three are
  plan 12-04's, per the "Artifacts this phase produces" table and the plan's own prohibitions
  (`PROMOTION.md` must have exactly one writer, in the final wave).

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 1 - Bug] `## Code Locations`/`## Considered Options` wrapped-bullet style inflated
`adr-parser.cjs`'s parsed entry counts far past what the corresponding bullet count would suggest**
- **Found during:** Task 1, running the plan's own structural self-check against the first-draft
  file (`adr-parser.cjs --input` + the `key_files`/`options_considered` length assertions)
- **Issue:** The first draft followed ADR-0031's own multi-line-wrapped-bullet style (each citation
  or option spanning 2-4 physical lines, matching ADR-0031's own prose rhythm). `adr-parser.cjs`'s
  `splitEntries` splits a section body by literal newline and only strips a leading bullet marker
  where present — every wrapped continuation line becomes its own "entry," independent of whether it
  starts with a bullet character. The first draft measured `key_files=29` against 13 actual bullet
  lines and `options_considered=19` against 4 actual bullets (comparable to ADR-0031's own
  28-key_files-against-~10-real-citations ratio, which `12-PATTERNS.md` and the parser constraints
  in this plan's prompt name as the exact wrinkle to avoid). This is a stricter form of the
  "trailing un-bulleted paragraph" defect the plan's `parser_constraints` section calls out —
  wrapped bullets inflate the count the same way, not only trailing prose.
- **Fix:** Rewrote every bullet in both sections as a single unwrapped physical line (long lines
  accepted in exchange for one bullet == one parsed entry). Re-ran the structural check: `key_files`
  dropped from 29 to 12 (matching the 12 visible bullet lines exactly), `options_considered` from 19
  to 4 (matching the 4 visible bullet lines exactly). Removed one leftover duplicate source-document
  citation line that an intermediate edit pass left behind during the rewrite.
- **Files modified:** `.planning/decisions/0036-audit-suppression-single-source-topology.md`
- **Verification:** `sed -n '/^## Considered Options/,/^## Code Locations/p'` and `sed -n
  '/^## Code Locations/,/^## Code Conformance/p'` both yield only heading/blank/bullet lines (no
  non-bullet content); `key_files` count (12) equals the counted bullet lines (12) exactly, well
  inside the plan's "within a factor of two" acceptance bar.
- **Committed in:** `931fc29` (Task 1 commit) — the fix was applied before the first commit of this
  file, so no separate remediation commit exists; the committed version is the corrected one.

---

**Total deviations:** 1 auto-fixed (1 bug in ADR-authoring mechanics, caught and fixed by the
plan's own structural self-check before commit).
**Impact on plan:** None on scope or content — no citation, option, or decision was added, removed
or reworded; only the physical line-wrapping of existing bullets changed, plus removal of one
accidental duplicate line from an intermediate edit. No prohibited site was touched.

## Issues Encountered

None beyond the deviation above, which the plan's own verify block caught and this execution fixed
before any commit.

## User Setup Required

None — no external service configuration required.

## Next Phase Readiness

- ADR-0036 exists, `Accepted`, `conforms`, structurally valid, citing ADR-0024 and the D-08 guard —
  ready for plan 12-04 to reference in the `PROMOTION.md` index row, the `## Key Decisions` table
  row, and the Phase 13 hand-off block.
- All four correction banners are in place with their originals intact; plan 12-04 can flip
  SUPPLY-03's checkbox and traceability row against this evidence without re-deriving anything.
- No blockers. `PROMOTION.md`, ADR-0024, and SUPPLY-03's own checkbox/row remain exactly as plan
  12-04 expects to find them (untouched by this plan).

---
*Phase: 12-supply-chain-gate-integrity*
*Completed: 2026-08-09*

## Self-Check: PASSED

- FOUND: `.planning/decisions/0036-audit-suppression-single-source-topology.md`
- FOUND: `.planning/phases/12-supply-chain-gate-integrity/12-03-SUMMARY.md`
- FOUND: commit `931fc29` (Task 1)
- FOUND: commit `e78eaac` (Task 2)
- FOUND: commit `692af96` (this SUMMARY)
