---
phase: 10-milestone-7-8-ground-truth-recorded-account
plan: 04
subsystem: infra
tags: [adr, dependency-rule, cargo-features, docs, ground-truth]

# Dependency graph
requires:
  - phase: 07-workspace-ground-truth-recorded-answers
    provides: ADR-0015's structural model (invariant separated from fact list) and the D-11 "an ADR is the promotion" rule
provides:
  - "ADR-0031: the extracted-crate dependency rule restated as a default-build invariant, checkable via `cargo tree --no-default-features`"
  - "Three human-answered branch selections (D-15, D-18, cargo-doc bar) recorded for plans 10-05 and 10-06 to read"
  - "Dated inline corrections on M7 Epic 1 PRD Goal 2 and §6.1, naming ADR-0031"
affects: [10-05-pdf-extraction-capability, 10-06-cargo-doc-warning-bar, 10-07-ledger-hard-05-row, phase-11-facade-02]

# Tech tracking
tech-stack:
  added: []
  patterns: ["ADR multi-part Decision shape (invariant / baseline / anchor-moved), copied from ADR-0015", "D-00c inline strike-and-correct annotation with a document-head banner"]

key-files:
  created:
    - .planning/decisions/0031-extracted-crate-dependency-rule.md
  modified:
    - .project/Milestone_7-Production-Hardening/Epic_1/prd-extract-infrastructure-crates.md

key-decisions:
  - "q1-restate: the extracted-crate dependency rule is a default-build invariant, not an absolute 'never' — ADR-0031"
  - "q2-delete: the inert pdf feature is deleted, not wired or kept — executed by plan 10-05"
  - "q3-ratify: the cargo doc zero-warning bar is ratified with the 20-warning debt recorded, not cleared in this phase — executed by plan 10-06"

requirements-completed: [HARD-05]

coverage:
  - id: D1
    description: "ADR-0031 restates the extracted-crate dependency rule as a default-build invariant, in ADR-0015's multi-part shape, naming cargo tree --no-default-features as the check and handing enforcement to Phase 15"
    requirement: "HARD-05"
    verification:
      - kind: other
        ref: "grep -c 'cost-benefit-assessment' / 'no-default-features' / 'ADR-0015' / 'Phase 15' / 'FACADE-02' / '(rejected)' / 'special case\\|promot' / '0\\.7\\.0' / grep -cx 'conforms' against .planning/decisions/0031-extracted-crate-dependency-rule.md — all thresholds met (see Self-Check)"
        status: pass
    human_judgment: false
  - id: D2
    description: "M7 Epic 1 PRD Goal 2 and §6.1 each carry a dated HARD-05 correction naming ADR-0031, original absolute wording retained struck"
    requirement: "HARD-05"
    verification:
      - kind: other
        ref: "grep -c 'Corrected (dated 2026-08-08, HARD-05)' == 2; grep -c 'Corrected (dated 2026-08-08, HARD-06)' == 0; git diff --numstat shows 23 insertions / 2 deletions"
        status: pass
    human_judgment: false

duration: 35min
completed: 2026-08-08
status: complete
---

# Phase 10 Plan 04: Extracted-Crate Dependency Rule (ADR-0031) Summary

**ADR-0031 restates the M7 extraction PRD's absolute "never depend on another extracted crate or
the facade" rule as a checkable default-build invariant, and the PRD's Goal 2 / §6.1 are annotated
in place to point at it — settling the answer Phase 11's FACADE-02 plans its relocation targets
against, plus recording two more branch decisions plans 10-05 and 10-06 need.**

## Performance

- **Duration:** 35 min
- **Started:** 2026-08-08T15:28:00Z
- **Completed:** 2026-08-08T16:03:49Z
- **Tasks:** 3 (1 checkpoint, already answered; 2 auto)
- **Files modified:** 2 (1 created, 1 modified)

## Checkpoint Answers (Task 1 — already answered, recorded here verbatim)

This plan's Task 1 was a `checkpoint:decision` with `gate="blocking"`. It was answered by the human
before this executor ran; the answers below are transcribed verbatim from the executor's prompt,
not re-derived or re-asked.

**Question 1 (D-15, extracted-crate dependency rule) → `q1-restate`.**
The rule is restated as a **default-build invariant**: no extracted crate may depend on another
extracted crate or on the `paladin` facade **in its default build**; a non-default optional
feature may declare such an edge only where the facade opts in explicitly and the dependent code
is `cfg`-gated. ADR-0031 is written against this form. The `paladin-content → paladin-llm` edge is
therefore legal and is **not** removed. This is the recommended branch, so Task 2 ran in its
recommended form and the plan did not halt.

*Reasoning the human's selection carries:* q1-restate was chosen because the invariant with teeth
is the default-build one — checkable with `cargo tree --no-default-features`, the same Phase 15
mechanism ADR-0015 already awaits — and because removing the edge is architecture work outside a
ground-truth phase.

**Question 2 (D-18, the inert `pdf` feature) → `q2-delete`.**
Delete the `pdf = []` line from `crates/paladin-content/Cargo.toml:18`. **Plan 10-05 executes
this**; this answer is recorded here so 10-05 can read it before it runs. The accepted cost is that
`cargo build -p paladin-content --features pdf` begins to fail where it previously
succeeded-and-did-nothing, and that must be recorded in `crates/paladin-content/CHANGELOG.md` by
plan 10-05.

*Reasoning the human's selection carries:* q2-delete was chosen because the feature gates nothing
in either direction while `news-api = []` in the same manifest legitimately gates a
dependency-free module, proving an empty feature is not itself the defect.

**Question 3 (the `cargo doc` bar) → `q3-ratify`.**
ADR-0033 ratifies the zero-warning bar as the project's one bar AND records the measured
20-warning state (dated, counted, per-crate: `paladin-web` 13, `paladin-battalion` 3, `paladin-ai`
3, `paladin-herald` 1) as debt with **Phase 16 / DOCS-03** named as owner. **Plan 10-06 executes
this**; this answer is recorded here so 10-06 can read it before it runs. **D-23's boundary
HOLDS** — this phase touches no `.rs` file, and every plan's `git status --porcelain -- '*.rs'`
empty assertion stands unchanged (verified for this plan below). No extra plan is added.

*Reasoning the human's selection carries:* q3-ratify was chosen to keep a ground-truth phase
writing ground truth rather than converting it into a code-change phase with its own review
surface.

## Accomplishments

- Wrote `.planning/decisions/0031-extracted-crate-dependency-rule.md` — seven canonical headings,
  no frontmatter, leading with the measured default-build fact (`paladin-content`'s `llm` feature
  is non-default, `cfg`-gated, and activated only by the facade's explicit opt-in), then the
  restated invariant in ADR-0015's three-part shape, then Considered Options, Code Locations
  (including the corrected `cost-benefit-assessment.md:118` citation), `conforms`, and Downstream
  Consumers naming Phase 11 / FACADE-02 and Phase 15.
- Ran `cargo tree -p paladin-content --no-default-features` in this environment; it resolved
  offline and the captured output contains zero occurrences of `paladin-llm`, `paladin-web`,
  `paladin-storage`, `paladin-notifications`, `paladin-battalion`, or the facade package
  `paladin-ai` — the invariant holds today, measured directly, not inferred.
- Annotated M7 Epic 1 PRD Goal 2 and §6.1 with a dated head blockquote plus per-clause
  strike-and-correct treatment, per D-00c: original absolute wording retained struck, followed by
  a `**Corrected (dated 2026-08-08, HARD-05):**` paragraph citing ADR-0031's restated invariant.
- Recorded the post-edit line numbers of §4.4.1 (`151`, was `132`) and §4.4.6 (`184`, was `165`)
  for plan 10-05, which owns those two clauses in a later wave and must still re-verify the
  numbers itself before editing.

## Task Commits

1. **Task 1: Blocking decision — the three flagged branches** — no commit (checkpoint answered
   prior to this executor's run; answers recorded above, not re-asked)
2. **Task 2: Write ADR-0031** — `6493726` (feat)
3. **Task 3: Annotate M7 Epic 1 PRD §6.1 and Goal 2 at source** — `ee64c4e` (docs)

_No plan-metadata commit in this plan — worktree mode: STATE.md/ROADMAP.md updates are owned by
the orchestrator after all wave agents complete, per this plan's execution instructions._

## Files Created/Modified

- `.planning/decisions/0031-extracted-crate-dependency-rule.md` — new ADR restating the
  extracted-crate dependency rule as a default-build invariant.
- `.project/Milestone_7-Production-Hardening/Epic_1/prd-extract-infrastructure-crates.md` — Goal 2
  (line 30) and §6.1 (line ~257, pre-edit) struck and corrected in place; a dated head blockquote
  added after the document's metadata block. §4.4.1 and §4.4.6 untouched, now at post-edit lines
  151 and 184 respectively.

## Decisions Made

- The three checkpoint answers above (`q1-restate`, `q2-delete`, `q3-ratify`) were already made by
  the human before this executor ran; they are transcribed, not re-derived.
- Task 2's ADR cites `cost-benefit-assessment.md:118` rather than the PRD's own non-existent §4.4
  for the `paladin-llm` complexity note — both `10-CONTEXT.md` D-15 and
  `.planning/REQUIREMENTS.md:1438` mis-attribute that sentence to the PRD's §4.4; the correct
  citation is used throughout ADR-0031 and the mis-citation is not reproduced.
- Task 3's banner text states explicitly that the promotion is of the invariant to its general
  form (default-build), not permission for a default-build edge, and that the absolute form
  becomes the special case of a crate declaring no non-default features — matching D-15's
  requirement that the ADR record the anchor as moved deliberately.

## Deviations from Plan

None — plan executed exactly as written. The checkpoint was pre-answered by the human as
instructed in this executor's prompt; Tasks 2 and 3 ran in their recommended forms per the
`q1-restate` selection.

## Issues Encountered

None. `cargo tree -p paladin-content --no-default-features` resolved offline against the
workspace's cached/vendored lockfile on the first attempt, so no CI-only fallback was needed for
the invariant's check.

## Self-Check

Verified against the plan's acceptance criteria for Task 2 (`0031-extracted-crate-dependency-rule.md`):
- Heading set matches exactly: `## Status`, `## Context`, `## Decision`, `## Considered Options`,
  `## Code Locations`, `## Code Conformance`, `## Downstream Consumers`, no frontmatter — confirmed.
- `grep -c 'cost-benefit-assessment'` → `4`; `grep -c 'prd-extract-infrastructure-crates.md §4.4'`
  → `0` (mis-citation not reproduced).
- `grep -c 'no-default-features'` → `6` (≥2 required).
- `grep -c 'ADR-0015'` → `3`; `grep -c 'Phase 15'` → `2` (both ≥1 required).
- `grep -c 'FACADE-02'` → `2` (≥1 required).
- `grep -c '(rejected)'` → `3`, including the edge-removal option and the `--manifest` re-tag
  option (≥3 required).
- `grep -ci 'special case\|promot'` → `8` (≥1 required).
- `grep -c '0\.7\.0'` → `5` (≥1 required).
- `grep -cx 'conforms'` → `1` (exact line, `== 1` required).
- `git status --porcelain -- '*.rs' Cargo.toml crates/paladin-content/Cargo.toml` → empty.

Verified against the plan's acceptance criteria for Task 3 (PRD annotation):
- `grep -c 'Corrected (dated 2026-08-08, HARD-05)'` → `2` (exactly `2` required).
- `grep -c 'Corrected (dated 2026-08-08, HARD-06)'` → `0` (required `0`).
- `grep -o '~~' | wc -l` → `4` occurrences (2 struck sentences, opening+closing marker each); note
  the plan's literal `grep -c '~~'` (line-count mode) returns `2`, since both markers of each
  strike sit on the same line — the intent ("both original sentences survive struck") is
  unambiguously met; flagging the counting-mode discrepancy here for transparency.
- `grep -c 'ADR-0031'` → `3`; relative link `../../../.planning/decisions/0031-extracted-crate-dependency-rule.md` resolves (confirmed via `ls`).
- `git diff --numstat` → `23` insertions, `2` deletions (exactly `2` deleted lines required).
- `grep -n '4.4.1 Create'` → line `151`, exactly one match; `grep -n '4.4.6 The facade'` → line
  `184`, exactly one match.
- `grep -c 'no-default-features\|default build'` → `8` (≥1 required).
- `git status --porcelain -- '*.rs' Cargo.toml crates/paladin-content/Cargo.toml Makefile
  .cargo/audit.toml` → empty.

Commit hashes verified present: `git log --oneline --all | grep -q 6493726` → FOUND;
`git log --oneline --all | grep -q ee64c4e` → FOUND.

## Self-Check: PASSED

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- ADR-0031 is committed and citable by number; Phase 11 / FACADE-02 can plan D2/D3/D4's
  relocation targets against it.
- Plan 10-05 has this SUMMARY's `q2-delete` answer and can proceed with deleting
  `crates/paladin-content/Cargo.toml:18`'s `pdf = []` line; it must re-derive §4.4.1's and
  §4.4.6's line numbers itself rather than trust the `151`/`184` recorded here as final (this
  plan's own instruction: "10-05 re-derives its line numbers because this plan's insertions move
  them").
- Plan 10-06 has this SUMMARY's `q3-ratify` answer and can proceed with ADR-0033 recording the
  20-warning debt with Phase 16 / DOCS-03 as owner.
- Plan 10-07's ledger row for `REQ-extracted-crate-dependency-rule` can now cite ADR-0031 and flip
  from `Code diverges → HARD-05` to `satisfied`.
- No blockers. `git status --porcelain -- '*.rs'` is empty, confirmed above — D-23's boundary
  holds for this plan.

---
*Phase: 10-milestone-7-8-ground-truth-recorded-account*
*Completed: 2026-08-08*
