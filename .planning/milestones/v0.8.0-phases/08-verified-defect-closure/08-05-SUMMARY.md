---
phase: 08-verified-defect-closure
plan: 05
subsystem: docs
tags: [requirements-traceability, corpus-annotation, debt-01, api-surface]

requires:
  - phase: 08-verified-defect-closure
    provides: DEBT-01 research (08-RESEARCH.md) confirming .project/current-exports.txt exists and the corrected regeneration procedure works
provides:
  - Five .project/ requirement documents annotated with dated D-00c correction banners and inline struck-and-corrected clauses, naming .project/current-exports.txt instead of the stale project/current-exports.txt
  - Five .planning/REQUIREMENTS.md REQ-* traceability rows corrected in place with a dated Phase 8 note, DEBT-01 pointers intact
  - A deferred-items.md record of four additional, out-of-scope propagation sites discovered during verification
affects: [08-09 (DEBT-01 checkbox/status closure), any future .project/ ingest run touching Milestone 8 Epic 7 or Milestone 12]

tech-stack:
  added: []
  patterns: [D-00c dated correction banner + inline struck-and-corrected annotation, applied to .project/ source documents]

key-files:
  created:
    - .planning/phases/08-verified-defect-closure/deferred-items.md
  modified:
    - .project/Milestone_8-Facade-Cleanup-Shim-Resolution/Epic_7/prd-paladin-web-single-framework-axum.md
    - .project/Milestone_12-Web-API/Epic_1/prd-agent-registry-execution-api.md
    - .project/Milestone_12-Web-API/Epic_5/prd-api-security-authorization.md
    - .project/Milestone_12-Web-API/Epic_6/prd-openapi-spec-interactive-docs.md
    - .project/Milestone_12-Web-API/Epic_7/prd-deployment-artifacts-examples-docs.md
    - .planning/REQUIREMENTS.md

key-decisions:
  - "Corrected a second live clause in prd-paladin-web-single-framework-axum.md (§7 Technical Considerations, not just FR-10) beyond the plan's named site, because the plan's own zero-live-occurrence acceptance criterion requires it — the file had two occurrences, not one"
  - "Restructured multi-line struck clauses to strike per-line (~~...~~ on every line) rather than open/close-only, because the plan's mechanical verify grep is line-based and would otherwise false-flag legitimately-struck middle lines"
  - "Discovered four additional stale-path propagation sites outside the plan's five named documents; logged to deferred-items.md and left unfixed per the SCOPE BOUNDARY deviation rule rather than silently expanding the plan's declared file scope"

requirements-completed: [DEBT-01]

coverage:
  - id: D1
    description: "Five .project/ requirement documents (M8 Epic 7, M12 Epic 1/5/6/7) carry a dated DEBT-01 correction banner and inline struck-and-corrected clauses naming .project/current-exports.txt, original text retained"
    requirement: "DEBT-01"
    verification:
      - kind: other
        ref: "grep -rn 'project/current-exports.txt' .project/ --include='prd-*.md' | grep -v '~~' | grep -v '\\.project/current-exports' (zero output for the five target files)"
        status: pass
    human_judgment: false
  - id: D2
    description: "Five REQUIREMENTS.md REQ-* traceability rows corrected in place with a dated Phase 8 note naming .project/current-exports.txt, DEBT-01 pointers intact, no checkbox/status row touched"
    requirement: "DEBT-01"
    verification:
      - kind: other
        ref: "sed -n '826p' and sed -n '3659,3663p' .planning/REQUIREMENTS.md unchanged; git diff --stat confined to 5 hunks"
        status: pass
    human_judgment: false
  - id: D3
    description: "Four additional stale-path propagation sites outside this plan's five named documents, discovered during verification and left unfixed per scope boundary"
    verification: []
    human_judgment: true
    rationale: "A scope/prioritization call for a human or a follow-up plan (recommended: 08-09) — whether to fix these four before DEBT-01's checkbox is ticked, not something this plan's declared file scope covers"

duration: ~20min
completed: 2026-08-07
status: complete
---

# Phase 8 Plan 05: Requirement-Text Half of DEBT-01 Summary

**Annotated five `.project/` PRDs and corrected five REQUIREMENTS.md traceability rows to name `.project/current-exports.txt` instead of the stale `project/current-exports.txt`, using the D-00c dated-banner-plus-struck-and-corrected pattern — and discovered four more propagation sites the corpus's "nine references" count missed.**

## Performance

- **Duration:** ~20min
- **Started:** 2026-08-07T00:03:14Z (STATE.md last_updated at phase start)
- **Completed:** 2026-08-07T00:22:59Z
- **Tasks:** 2
- **Files modified:** 6 (5 `.project/` PRDs + `.planning/REQUIREMENTS.md`), plus 1 file created (`deferred-items.md`)

## Accomplishments

- Annotated all five `.project/` requirement-text sources named in CONTEXT.md D-04 with a dated
  `2026-08-06, DEBT-01` correction banner immediately below the H1, and struck-and-corrected every
  defective clause inline — 7 clauses total, one more than the plan's estimated 6 (see Deviations)
- Corrected all five `REQ-*` traceability rows in `.planning/REQUIREMENTS.md` to name
  `.project/current-exports.txt` with a dated Phase 8 attribution, appending rather than replacing
  existing evidence text, `→ DEBT-01` pointers intact
- Confirmed via re-run evidence that `.project/current-exports.txt` exists (442,369 bytes) and
  `project/current-exports.txt` does not, in every corrected document
- Discovered and recorded four additional stale-path propagation sites the corpus's nine-reference
  count for DEBT-01 did not include, none of them past-tense task-list records

## Task Commits

Each task was committed atomically:

1. **Task 1: Annotate the five .project/ requirement-text sources** - `94814ff` (docs)
2. **Task 2: Correct the five REQUIREMENTS.md traceability rows** - `8897b55` (docs)

**Plan metadata:** committed alongside this SUMMARY (see below)

## Files Created/Modified

- `.project/Milestone_8-Facade-Cleanup-Shim-Resolution/Epic_7/prd-paladin-web-single-framework-axum.md` - banner + FR-10 correction + a second corrected clause (§7 Technical Considerations)
- `.project/Milestone_12-Web-API/Epic_1/prd-agent-registry-execution-api.md` - banner + §7 "API surface" bullet corrected
- `.project/Milestone_12-Web-API/Epic_5/prd-api-security-authorization.md` - banner + §7 "API surface" bullet corrected
- `.project/Milestone_12-Web-API/Epic_6/prd-openapi-spec-interactive-docs.md` - banner + §7 "API surface" bullet corrected, `cross_refs` label-drift noted
- `.project/Milestone_12-Web-API/Epic_7/prd-deployment-artifacts-examples-docs.md` - banner + two corrected clauses (Success Metric 6, §4.6 FR-13)
- `.planning/REQUIREMENTS.md` - five `REQ-*` traceability rows corrected in place
- `.planning/phases/08-verified-defect-closure/deferred-items.md` - new record of four out-of-scope propagation sites

## Evidence: before/after `file:line` for each corrected clause

All `ls` evidence re-run during this task: `ls -la .project/current-exports.txt` → present, 442,369
bytes. `ls project/current-exports.txt` → "No such file or directory".

1. **`prd-paladin-web-single-framework-axum.md` FR-10** — before (original `:98-101`): `10. The
   system **must** regenerate the public API-surface baseline (`./scripts/extract-public-api.sh
   project/current-exports.txt`) ...`. After (now `:110-120`): struck in place, corrected paragraph
   below it names `.project/current-exports.txt` and the corrected command
   `./scripts/extract-public-api.sh .project/current-exports.txt`.
2. **`prd-paladin-web-single-framework-axum.md` §7 Technical Considerations "Crate / layer" bullet
   — not named in the plan's `read_first`, found during this task's own acceptance-criteria run**
   — before (original `:128-131`): "...plus `deny.toml`, `project/current-exports.txt`, and
   `CHANGELOG.md` at the workspace root...". After (now `:147-155`): struck in place, corrected
   paragraph points at the FR-10 correction for evidence. Both occurrences in this file are now
   closed; the file's live-occurrence count went from 2 to 0.
3. **`prd-agent-registry-execution-api.md` §7 "API surface" bullet** — before (original `:285-286`):
   "new public items will change `project/current-exports.txt`; regenerate the baseline...". After
   (now `:298-303`): struck in place, corrected paragraph names `.project/current-exports.txt`.
4. **`prd-api-security-authorization.md` §7 "API surface" bullet** — before (original `:220-222`):
   "new public items (...) will change `project/current-exports.txt` — regenerate...". After (now
   `:233-240`): struck per-line, corrected paragraph names `.project/current-exports.txt`.
5. **`prd-openapi-spec-interactive-docs.md` §7 "API surface" bullet** — before (original
   `:197-198`): "new public items (...) will change `project/current-exports.txt` — regenerate
   (additive expected)." After (now `:213-218`): struck in place, corrected paragraph names
   `.project/current-exports.txt` and records the `cross_refs` label drift REQUIREMENTS.md's
   traceability row carries (the actual clause is a §7 bullet, not a `cross_refs` field).
6. **`prd-deployment-artifacts-examples-docs.md` Success Metric 6** — before (original `:51-52`):
   "All workspace crates are at **0.6.0**; `CHANGELOG.md` and `project/current-exports.txt` reflect
   the release." After (now `:64-70`): struck in place, corrected paragraph names
   `.project/current-exports.txt`.
7. **`prd-deployment-artifacts-examples-docs.md` §4.6 FR-13** — before (original `:128-129`):
   "`CHANGELOG.md` **must** gain a finalized Milestone 12 release summary ... and
   `project/current-exports.txt` **must** be regenerated." After (now `:146-152`): struck in place,
   corrected paragraph names `.project/current-exports.txt`.

## Evidence: before/after text of each of the five REQUIREMENTS.md traceability rows

1. **`REQ-web-api-baseline-changelog` (:3028)** — before: "...**The defect is now written into a
   requirement as well as into the tooling**, so DEBT-01 must correct both". After: same text plus
   appended "**Corrected (Phase 8, dated 2026-08-06):** FR-10 is now annotated in place at source
   (`.project/Milestone_8-Facade-Cleanup-Shim-Resolution/Epic_7/prd-paladin-web-single-framework-axum.md`)
   with a dated D-00c banner and inline struck-and-corrected text naming
   `.project/current-exports.txt`; original text retained, nothing deleted → DEBT-01".
2. **`REQ-agent-registry` (:3245)** — before: "...**§7 names `project/current-exports.txt`** →
   DEBT-01". After: same text plus appended "**Corrected (Phase 8, dated 2026-08-06):** §7's "API
   surface" bullet is now annotated in place at source
   (`.project/Milestone_12-Web-API/Epic_1/prd-agent-registry-execution-api.md`) ... → DEBT-01".
3. **`REQ-per-agent-role-authorization` (:3292)** — before: "Verify → ORCH-01. **§7 names
   `project/current-exports.txt`** → DEBT-01". After: same text plus appended "**Corrected (Phase
   8, dated 2026-08-06):** §7's "API surface" bullet is now annotated in place at source
   (`.project/Milestone_12-Web-API/Epic_5/prd-api-security-authorization.md`) ... → DEBT-01".
4. **`REQ-openapi-drift-guard` (:3303)** — before: "...**`cross_refs` names
   `project/current-exports.txt`** → DEBT-01". After: same text plus appended "**Corrected (Phase 8,
   dated 2026-08-06):** the source clause is in fact §7's "API surface" bullet, not a `cross_refs`
   field ... → DEBT-01".
5. **`REQ-m12-v060-release` (:3313)** — before: "...**FR-4.6 names `project/current-exports.txt`**
   → DEBT-01. Its non-goals are notable...". After: same text plus appended "**Corrected (Phase 8,
   dated 2026-08-06):** both defective clauses (Success Metric 6 and §4.6 FR-13...) are now
   annotated in place at source ... → DEBT-01".

## Historical `.project/` task-list hits deliberately left intact (10 files, 16 lines)

Per the plan's Execution notes, these are past-tense records of an action already completed
against the old path, not forward-looking instructions. Correcting them would rewrite history,
which D-00c forbids (see E-01e in the plan's flagged-assumptions table — this disposition is
recorded as reasoning, not proof, per that entry).

| File | Lines |
|---|---|
| `Milestone_12-Web-API/Epic_1/tasks-agent-registry-execution-api.md` | 16 |
| `Milestone_12-Web-API/Epic_2/tasks-configurable-web-host-server-binary.md` | 87 |
| `Milestone_12-Web-API/Epic_3/tasks-streaming-async-execution.md` | 95 |
| `Milestone_12-Web-API/Epic_4/tasks-api-cross-cutting-concerns.md` | 25, 98 |
| `Milestone_12-Web-API/Epic_5/tasks-api-security-authorization.md` | 22, 82 |
| `Milestone_12-Web-API/Epic_6/tasks-openapi-spec-interactive-docs.md` | 25, 89 |
| `Milestone_12-Web-API/Epic_7/tasks-deployment-artifacts-examples-docs.md` | 22, 85 |
| `Milestone_11-Documentation-Overhaul-Publish/Epic_6/tasks-deployment-topologies-documentation.md` | 53 |
| `Milestone_8-Facade-Cleanup-Shim-Resolution/Epic_7/tasks-paladin-web-single-framework-axum.md` | 22, 104, 107 |
| `Milestone_4-Refactor-Crates-Features/Epic_2/tasks-harden-port-traits-stable-api.md` | 76 |

## Final verification grep (target files only)

```
$ grep -rn 'project/current-exports.txt' .project/ --include='prd-*.md' | grep -v '~~' | grep -v '\.project/current-exports'
.project/Milestone_12-Web-API/Epic_4/prd-api-cross-cutting-concerns.md:226:  `project/current-exports.txt` (paladin-web is optional/non-default in the facade surface, but the
.project/Milestone_11-Documentation-Overhaul-Publish/Epic_6/prd-deployment-topologies-documentation.md:254:(docs scope). Do **not** regenerate `project/current-exports.txt` (no API change).
```

**Not empty** — see Deviations below. Zero output for the five plan-target files specifically;
these two hits are outside this plan's declared scope.

## Decisions Made

- Followed CONTEXT.md D-00c / D-04 and 08-PATTERNS.md's verbatim analog
  (`prd-expand-feature-flags.md`) for the banner and inline-correction shape.
- Restructured struck spans to per-line `~~...~~` (rather than open-at-first-line,
  close-at-last-line) so the plan's own line-based mechanical verify grep does not false-flag
  legitimately-struck middle lines of a multi-line clause.
- Kept correction prose from ever repeating the bare, un-struck stale literal
  (`project/current-exports.txt` without the leading dot) — every mention refers to "the struck
  path above" or is itself wrapped in `~~`, so the automated live-occurrence check only flags
  genuine unstruck instructions, not evidentiary prose.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 1 - Bug] Corrected a second, previously unnamed defective clause in `prd-paladin-web-single-framework-axum.md`**
- **Found during:** Task 1, running the plan's own acceptance-criteria grep
- **Issue:** The plan's `read_first` and numbered site list named only FR-10 (`:99`) in this file.
  A second live occurrence existed at §7 Technical Considerations ("Crate / layer" bullet, then
  `:128-131`), which would have left the file's own zero-live-occurrence check failing even after
  FR-10 was fixed.
- **Fix:** Struck and corrected the second clause using the same D-00c pattern, cross-referencing
  the FR-10 correction for the evidence rather than repeating it.
- **Files modified:** `.project/Milestone_8-Facade-Cleanup-Shim-Resolution/Epic_7/prd-paladin-web-single-framework-axum.md`
- **Verification:** `grep -c 'project/current-exports.txt' <file>` went from 2 (both live) to 8
  (all inside `~~…~~` spans or the corrected `.project/` path); file-scoped live-occurrence check
  returns zero.
- **Committed in:** `94814ff` (Task 1 commit)

**2. [Rule 1 - Bug] Restructured multi-line struck clauses to per-line strikethrough**
- **Found during:** Task 1, first pass at the plan's acceptance-criteria grep
  (`grep -v '~~'`-filtered)
- **Issue:** My first-pass edits opened `~~` at the start of a multi-line clause and closed it at
  the end, which is valid Markdown but meant middle lines containing the literal
  `project/current-exports.txt` had no `~~` on that specific line — the plan's line-based
  mechanical check flagged them as "live" even though they were part of a genuinely struck span.
- **Fix:** Rewrote every multi-line struck clause (5 of the 7) so each line is independently
  wrapped in `~~...~~`. Also rewrote banner and "Corrected" prose to never state the bare stale
  literal outside a struck span — referring to "the struck path above" / "the pre-rename path"
  instead, since the mechanical check cannot distinguish evidentiary prose from a live instruction.
- **Files modified:** all five `.project/` PRDs
- **Verification:** `grep -rn 'project/current-exports.txt' .project/ --include='prd-*.md' | grep -v '~~' | grep -v '\.project/current-exports'` returns zero output for all five target files.
- **Committed in:** `94814ff` (Task 1 commit)

### Discovered But Not Fixed (logged to deferred-items.md, scope boundary)

**3. [Scope boundary] Four additional stale-path propagation sites, pre-existing and outside this plan's five named files**
- **Found during:** Task 1's own repository-wide verification grep, which is not restricted to the
  five target files
- **Issue:** `.project/Milestone_12-Web-API/Epic_4/prd-api-cross-cutting-concerns.md:225-227` and
  `.project/Milestone_11-Documentation-Overhaul-Publish/Epic_6/prd-deployment-topologies-documentation.md:254`
  both match the plan's own `prd-*.md` verification glob and carry the same defect class as the
  five corrected documents. Two more sites — `.project/Milestone_12-Web-API/overview/Milestone-12_Web-API.md:318`
  and `.project/Milestone_8-Facade-Cleanup-Shim-Resolution/Epic_7/Milestone_8-Epic_7-paladin-web-single-framework-axum.md:40`
  — are epic-summary/overview documents (not `prd-*`-named) with the same stale reference. None of
  the four are past-tense task-list records.
- **Why not fixed here:** the plan's `<files>` frontmatter and Task 1's own acceptance criterion
  ("`git status --short .project/` lists exactly the five PRD files") explicitly scope this plan
  to five documents. Per the executor deviation rules' SCOPE BOUNDARY guidance, pre-existing issues
  not caused by this task's changes — even same-class defects — are logged, not silently fixed, when
  fixing them would contradict an explicit plan constraint.
- **Disposition:** recorded in `.planning/phases/08-verified-defect-closure/deferred-items.md` with
  full `file:line` detail. **This means the plan's own must_haves backstop truth ("No sixth
  propagation site exists") is not actually true** — DEBT-01's corpus-recorded count of nine total
  references (five tooling + five requirement-text) understates the real propagation; the true
  count is at least thirteen, with four still open. Recommend routing to plan 08-09 (which owns
  DEBT-01's checkbox/status closure) before that checkbox is ticked.
- **Committed in:** `94814ff` (deferred-items.md created alongside Task 1's commit)

---

**Total deviations:** 2 auto-fixed (both Rule 1, both necessary to satisfy the plan's own mechanical
acceptance criteria), 1 discovered-and-deferred (scope boundary, logged not fixed).
**Impact on plan:** Both auto-fixes were required for the five target files to pass the plan's own
verification; no scope creep on those. The deferred item is a genuine, material finding about
DEBT-01's true closure status and should inform whether 08-09 ticks the DEBT-01 checkbox.

## Issues Encountered

None beyond the deviations above.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- Combined with plan 08-02 (the five tooling references), this plan closes the five requirement-text
  references named in CONTEXT.md D-04. DEBT-01's checkbox and status row remain untouched, as
  instructed — they are plan 08-09's, behind full closure evidence.
- **Blocker for 08-09:** before ticking DEBT-01's checkbox, 08-09 should account for the four
  additional propagation sites recorded in `deferred-items.md` — the "nine references" the corpus
  counted is not the full picture.

---
*Phase: 08-verified-defect-closure*
*Completed: 2026-08-07*

## Self-Check: PASSED

- FOUND: `.project/Milestone_8-Facade-Cleanup-Shim-Resolution/Epic_7/prd-paladin-web-single-framework-axum.md`
- FOUND: `.planning/phases/08-verified-defect-closure/deferred-items.md`
- FOUND: `.planning/phases/08-verified-defect-closure/08-05-SUMMARY.md`
- FOUND commit: `94814ff`
- FOUND commit: `8897b55`
