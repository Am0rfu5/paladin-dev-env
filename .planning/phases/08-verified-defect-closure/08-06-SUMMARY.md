---
phase: 08-verified-defect-closure
plan: 06
subsystem: docs
tags: [deprecation-policy, adr, mdbook, dot-project-annotation, deprecated-attribute]

# Dependency graph
requires:
  - phase: 08-verified-defect-closure
    provides: "ADR-0022 (plan 08-04, wave 1) — the recorded decision this plan cites throughout"
provides:
  - "DEPRECATIONS.md reads as a completed decision record, not an unfinished task list"
  - "stable-api.md carries no false present-tense deprecation claim while retaining its full forward-looking policy"
  - "Three-way agreement between ADR-0022, DEPRECATIONS.md and stable-api.md, proven by a recorded reading"
affects: ["08-09 (close-out plan — must amend ledger row REQ-deprecation-warnings)"]

# Tech tracking
tech-stack:
  added: []
  patterns: ["D-00c .project/ annotation: dated banner + inline ~~struck~~/Confirmed markup, zero deletion"]

key-files:
  created: []
  modified:
    - ".project/Milestone_4-Refactor-Crates-Features/Epic_2/DEPRECATIONS.md"
    - "docs/src/api-reference/stable-api.md"

key-decisions:
  - "Left check-deprecations.sh's stable-api.md:338 reference text unchanged after verifying it still accurately describes the plan-08-02-repaired script."
  - "Cited ADR-0022/ADR-0008 as plain-text file-path citations rather than markdown hyperlinks in stable-api.md, to keep the link-target set byte-identical (mdbook linkcheck cannot run locally)."

requirements-completed: [DEBT-02]

coverage:
  - id: D1
    description: "grep -rn '#[deprecated' src crates returns 0, recorded as the outcome of ADR-0022's withdrawal rather than an unfinished task"
    requirement: "DEBT-02"
    verification:
      - kind: other
        ref: "grep -rn '#\\[deprecated' src crates | wc -l -> 0"
        status: pass
    human_judgment: false
  - id: D2
    description: "DEPRECATIONS.md, stable-api.md and ADR-0022 tell one story: zero current deprecations, withdrawal named, ADR-0022 cited, no third state"
    requirement: "DEBT-02"
    verification: []
    human_judgment: true
    rationale: "VALIDATION.md marks this human_judgment: true — agreement between prose documents can only be established by a reading, not a grep. Five-point reading recorded below."
  - id: D3
    description: "stable-api.md's forward-looking deprecation policy (Deprecation Lifecycle, 🔴 Deprecated tier, FAQ, illustration) survives unchanged; only the present-tense claim at :875 is corrected"
    requirement: "DEBT-02"
    verification:
      - kind: other
        ref: "grep -c 'Deprecation Lifecycle' docs/src/api-reference/stable-api.md -> 1; grep -c '🔴 Deprecated' -> 1; grep -c 'deprecated(since' -> 4 (unchanged pre/post)"
        status: pass
    human_judgment: false
  - id: D4
    description: "DEPRECATIONS.md's four Open Questions each carry a one-line dated disposition; zero text deleted"
    requirement: "DEBT-02"
    verification:
      - kind: other
        ref: "sed -n '/^## Open Questions/,/^---/p' DEPRECATIONS.md shows four numbered items each with a Resolved/Moot/Closed annotation; all struck lines reappear in the same diff hunk"
        status: pass
    human_judgment: false

duration: ~20min
completed: 2026-08-07
status: complete
---

# Phase 8 Plan 06: Deprecation Three-Way Reconciliation Summary

**Annotated `DEPRECATIONS.md` and corrected `stable-api.md` so both documents and the tree agree with ADR-0022: zero `#[deprecated]` attributes is the withdrawal's recorded outcome, not an unfinished task.**

## Performance

- **Duration:** ~20 min
- **Tasks:** 3/3 completed
- **Files modified:** 2 (plus this SUMMARY)

## Accomplishments

- `DEPRECATIONS.md` gained a dated banner citing ADR-0022, a struck-and-restated stale timeline, a confirmed IMMEDIATE-DEPRECATION section, terminal (not in-progress) Current Status / Deprecation Log annotations, and all four Open Questions closed — with zero original text deleted.
- `stable-api.md:875`'s false "Current and planned deprecations" claim corrected to state the process is documented and nothing is currently active, with a new forward-looking note at the Deprecation Lifecycle section restating the stale version anchors against ADR-0008/ADR-0022 — while the link-target set stayed byte-identical before and after.
- A recorded five-point reading proves the three documents (ADR-0022, `DEPRECATIONS.md`, `stable-api.md`) tell one story, backed by a zero-count grep and a passing `check-deprecations.sh`.

## Task Commits

Each task was committed atomically:

1. **Task 1: Annotate DEPRECATIONS.md** - `c8bd7b1` (docs)
2. **Task 2: Correct stable-api.md's false present-tense claim** - `2e30e89` (docs)
3. **Task 3: Prove the three-way agreement** - this SUMMARY (no code change; verification-only task)

**Plan metadata:** committed separately by the orchestrator after wave completion (per parallel-executor instructions, this agent does not create the plan-metadata commit).

## Files Created/Modified

- `.project/Milestone_4-Refactor-Crates-Features/Epic_2/DEPRECATIONS.md` — dated D-00c banner + inline annotations; four Open Questions closed; zero deletions (all struck lines reappear in the same diff hunk, verified via `git show c8bd7b1 -- <file> | grep '^-'`).
- `docs/src/api-reference/stable-api.md` — one false present-tense claim corrected at `:875` (now `:887` post-edit); forward-looking note added before `### Deprecation Lifecycle`; policy content, tier, FAQ and illustration retained verbatim.

## Evidence Bar (D-00e / D-21)

**Verbatim command outputs, this session:**

```
$ grep -rn '#\[deprecated' src crates | wc -l
0

$ grep -rn 'doc(hidden)' src crates | wc -l
38

$ grep -n '^version' Cargo.toml | head -1
34:version = "0.7.0"

$ bash scripts/check-deprecations.sh
🔍 Checking deprecation warnings...
✅ No deprecation warnings found
Checking for properly formatted deprecation attributes...
✅ All deprecation attributes are properly formatted
$ echo $?
0
```

**`stable-api.md:875` before/after** (original plan-cited line was `:875`; after this plan's earlier
insertion of the Deprecation Lifecycle note, the corrected line now sits at `:887` — content is
what matters):

- **Before:** `- **[Deprecations Tracking](https://github.com/DF3NDR/paladin-dev-env/blob/main/CHANGELOG.md)** - Current and planned deprecations`
- **After:** `- **[Deprecations Tracking](https://github.com/DF3NDR/paladin-dev-env/blob/main/CHANGELOG.md)** - The deprecation process is documented here; no deprecation is currently active (see ADR-0022, \`.planning/decisions/0022-deprecation-requirement-withdrawal.md\`)`

**Link-target set, before vs. after** (`grep -o '](\([^)]*\))' docs/src/api-reference/stable-api.md | sort -u`): **byte-identical** — verified via `diff` producing no output. The two new ADR citations use plain-text file-path references (`.planning/decisions/0022-...md`, `.planning/decisions/0008-...md`), not markdown hyperlinks, specifically to keep the mdbook linkcheck target set unchanged since linkcheck cannot be run locally to prove it (see mdbook note below).

## Open Question dispositions (DEPRECATIONS.md `:254-267` post-edit, originally `:206-211`)

1. **Adapter Visibility Strategy** — **Resolved.** `#[doc(hidden)]` was the approach taken; `grep -rn 'doc(hidden)' src crates | wc -l` → **38**, confirmed tree-wide this session.
2. **Factory Functions** — **Moot** under ADR-0022: no deprecation timeline exists to schedule this against. Whether factory functions are independently worth adding is a live design question, outside DEBT-02's scope.
3. **Prelude Module** — **Closed — answered by shipped code.** `src/prelude.rs` exists and re-exports `Paladin`, `PaladinConfig`, `PaladinData`, `PaladinStatus`, `BattalionConfig`, `BattalionError` for `use paladin::prelude::*`. Verified present this session (`find . -name "prelude.rs"` → `./src/prelude.rs`).
4. **Manager Refactoring** — **Moot** under ADR-0022: no deprecation timeline exists to schedule this against. Manager types' `pub(crate)`/application-layer placement is Epic 3's own concern.

## Task 6.0 export-curation marker

`DEPRECATIONS.md`'s "⏳ Curating explicit exports (Task 6.0)" marker is annotated but explicitly **not adjudicated** by this phase: it is recorded as outside DEBT-02's scope (ADR-0022 withdraws FR-8's *deprecation* requirement only, not this separate export-curation task), with its original state left unchanged per D-00c. A future phase would need to evaluate Task 6.0 on its own merits.

## Five-Point Three-Way Agreement (`human_judgment: true` reading, per VALIDATION.md row 1)

Read end to end: `.planning/decisions/0022-deprecation-requirement-withdrawal.md`, `.project/Milestone_4-Refactor-Crates-Features/Epic_2/DEPRECATIONS.md` (as annotated), `docs/src/api-reference/stable-api.md` (as corrected).

| # | Point | ADR-0022 | DEPRECATIONS.md | stable-api.md |
|---|---|---|---|---|
| 1 | Each states zero `#[deprecated]` attributes exist today | `0022-...md:13` — "Re-run this session: `grep -rn '#\[deprecated' src crates` returns **0**." | banner (top, below H1) — "`grep -rn '#\[deprecated' src crates \| wc -l` → **0**" | `:775` (in the new Deprecation Lifecycle note) — "`grep -rn '#\[deprecated' src crates` returns **0** today" |
| 2 | Each names the FR-8 withdrawal and cites ADR-0022 by number | `0022-...md:39` — "## Decision — Milestone 4 Epic 2 FR-8 is **withdrawn**." | banner — "Milestone 4 Epic 2 FR-8 — the requirement this document tracks — is **withdrawn**... See ADR-0022" | `:774` — "was withdrawn by ADR-0022 ... on 2026-08-06" |
| 3 | Each gives the same reason: the epic's own IMMEDIATE DEPRECATION category named no candidate | `0022-...md:19-21` — quotes `DEPRECATIONS.md:81`'s "None identified yet..." verbatim as the primary evidence | `:115-119` (post-edit, the Confirmed annotation under Manager Services) — "This is the only category in this document that would ever produce a `#[deprecated]` attribute, and it names no candidate; that is the primary evidence ADR-0022 cites" | `:774-776` — "the epic's own tracking document named no candidate for deprecation, so ... returns 0 today, and that is the recorded outcome, not an unfinished task" |
| 4 | `stable-api.md` keeps its forward-looking policy while carrying no current-deprecation claim | `0022-...md:52-54` — "the *policy* ... survives ... only the claim that deprecations **exist today** is withdrawn" | n/a (this point belongs to stable-api.md, which DEPRECATIONS.md does not itself assert) | `### Deprecation Lifecycle` heading (`:784`), 🔴 Deprecated tier (`:183`), FAQ (`:395`), illustration (`:805-820`) all retained verbatim; only `:887`'s link description no longer claims current deprecations |
| 5 | None promises a removal timeline the tree cannot start — stale `v0.2.0→v0.3.0→v1.0.0` anchors restated as "at least one minor version" per ADR-0008 everywhere they appear | `0022-...md:43-50` — "the deprecation policy's version anchors move to **'one minor version'**" per ADR-0008 | Deprecation Timeline (`:15-18` original text, now struck) restated at the correction directly beneath it: "removal window is **'at least one minor version'**... per [ADR-0008]" | `:776-780` (new note) — "ADR-0022 restates the removal window as **'at least one minor version'**... per the pre-1.0 versioning posture recorded in ADR-0008" |

**All five points hold in all three documents. No fix was required before writing this SUMMARY.**

### Remaining `v0.3.0` occurrences — enumerated per acceptance criteria

**`stable-api.md`:**
- `:366`, `:369` — `cargo public-api --diff-git-checkouts v0.3.0 v0.5.0` (Manual API Verification section). **Not applicable** — this is an example command comparing two historical git tags/checkouts, not a claim about a deprecation-removal timeline. Untouched, correctly.
- `:777` (in the new note) — accompanied by the ADR-0022 restatement, per point 5 above.

**`DEPRECATIONS.md`:**
- `:32` (post-edit) — inside the struck Deprecation Timeline span.
- `:110` — "**Status:** Add `#[deprecated]` in v0.2.0, remove in v0.3.0" under Category: Manager Services — accompanied three lines later (`:115-119`) by the Confirmed/ADR-0022 annotation for that same subsection.
- `:184` — inside the "Template for Deprecation Warnings" `rust` code fence, an explicitly hypothetical illustration (parallel to `stable-api.md`'s own `#[deprecated(since = ...)]` example) — not a claim about current state.
- `:247` — inside the "Format:" template block under Deprecation Log, immediately preceded (`:235-238`) by the Confirmed/ADR-0022 annotation — also a hypothetical illustration.

Every occurrence is either struck, hypothetical-illustration, or ADR-0022-accompanied — none is a live, unqualified timeline promise.

## mdbook / mdbook-linkcheck availability

Confirmed unavailable this session: `command -v mdbook` → exit 1 (not found); `command -v mdbook-linkcheck` → not found. Per the plan and `08-RESEARCH.md`, `cargo install mdbook-linkcheck` requires crates.io (HTTP 403 here) and was **not attempted**. VALIDATION.md row 2b is therefore satisfied **structurally, not by execution**: this plan changed no link target (the link-target set is byte-identical before/after, verified above) and added no page, so linkcheck risk is low but unproven locally.

## Ledger row plan 08-09 must amend

`.planning/ledgers/milestone-04-06.md:116` — `REQ-deprecation-warnings`, currently `genuinely outstanding`. The evidence that changes its verdict: `grep -rn '#\[deprecated' src crates` → 0 (unchanged from the ledger's own prior grep), but now corroborated by ADR-0022's withdrawal decision and this plan's three-way reconciliation (DEPRECATIONS.md and stable-api.md both now agree with the tree and cite ADR-0022) — the "genuinely incomplete epic" framing in the ledger row's current text no longer holds; the epic is complete by withdrawal, not incomplete by omission. This plan does **not** amend the ledger row itself (that is plan 08-09's job per D-23).

## Decisions Made

- Left `stable-api.md:338`'s `check-deprecations.sh` description ("Verifies that deprecated items compile with warnings") unchanged after verifying it against the plan-08-02-repaired script — the sentence does not over- or under-claim what the script now genuinely does (report deprecation warnings if present, gate on malformed-attribute formatting across `src`+`crates`).
- Cited ADR-0022 and ADR-0008 in `stable-api.md` as plain-text file-path references rather than markdown hyperlinks, so the link-target set stays byte-identical before/after (required since mdbook linkcheck cannot verify new targets locally).

## Deviations from Plan

None — plan executed exactly as written. One self-correction during execution: an initial draft of the `stable-api.md` Deprecation Lifecycle note used markdown hyperlinks to ADR-0022/ADR-0008, which changed the link-target set; caught by the acceptance-criteria check and rewritten as plain-text citations before commit. No separate commit was needed since this was caught pre-commit.

## Issues Encountered

`bash scripts/check-deprecations.sh` cold-compiles the workspace in this fresh worktree (`workflow.worktree_skip_hooks=true` applies) and exceeded the default 120s tool timeout on first invocation; re-ran with an explicit longer timeout after the background build settled, which then completed with exit 0 (both the deprecation-warning report and the malformed-attribute check passed). No code was at fault — this is expected cold-worktree build latency documented in project memory.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- DEBT-02 is fully reconciled: ADR-0022, `DEPRECATIONS.md` and `stable-api.md` agree; tree unchanged at zero `#[deprecated]` attributes.
- Plan 08-09 (close-out) can now amend `.planning/ledgers/milestone-04-06.md:116`'s `REQ-deprecation-warnings` row using the evidence recorded above, flip the DEBT-02 checkbox in REQUIREMENTS.md, and add ADR-0022's `PROMOTION.md` row.
- No blockers for downstream plans in this phase.

---
*Phase: 08-verified-defect-closure*
*Completed: 2026-08-07*

## Self-Check: PASSED

- FOUND: `.project/Milestone_4-Refactor-Crates-Features/Epic_2/DEPRECATIONS.md`
- FOUND: `docs/src/api-reference/stable-api.md`
- FOUND: `.planning/phases/08-verified-defect-closure/08-06-SUMMARY.md`
- FOUND commit `c8bd7b1` (Task 1)
- FOUND commit `2e30e89` (Task 2)
- FOUND commit `454f528` (Task 3 — this SUMMARY)
