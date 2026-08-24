---
phase: 09-release-security-gate-integrity
plan: 05
subsystem: infra
tags: [licensing, spdx, cargo-manifest, compliance, adr]

# Dependency graph
requires:
  - phase: 09-release-security-gate-integrity (plan 03)
    provides: "Dockerfile.chef's nine-line deletion that moved the OCI licenses LABEL from ~:93 to :87"
provides:
  - "One licence expression (MIT OR Apache-2.0) declared identically across the root package and all ten library crates"
  - "LICENSE-MIT (renamed via git mv, history preserved) and a new verbatim LICENSE-APACHE"
  - "README.md badge and License section updated for dual licensing"
  - "Dockerfile.chef OCI image licenses label updated to match"
  - "ADR-0025 recording the licence decision, approver, date, and both branches fairly"
  - "The signed compatibility checklist and the M7 Epic 4 PRD/overview annotated in place, original text retained"
affects: [09-07, "Phase 10 / HARD-01 ledger"]

# Tech tracking
tech-stack:
  added: []
  patterns: ["dual-licence SPDX expression (MIT OR Apache-2.0) as the Rust ecosystem norm"]

key-files:
  created:
    - LICENSE-APACHE
    - .planning/decisions/0025-licence-posture.md
  modified:
    - Cargo.toml
    - crates/paladin-core/Cargo.toml
    - crates/paladin-ports/Cargo.toml
    - crates/paladin-battalion/Cargo.toml
    - crates/paladin-herald/Cargo.toml
    - crates/paladin-llm/Cargo.toml
    - crates/paladin-memory/Cargo.toml
    - crates/paladin-storage/Cargo.toml
    - crates/paladin-notifications/Cargo.toml
    - crates/paladin-content/Cargo.toml
    - crates/paladin-web/Cargo.toml
    - LICENSE (renamed to LICENSE-MIT)
    - README.md
    - Dockerfile.chef
    - CHANGELOG.md
    - .project/Milestone_7-Production-Hardening/Epic_4/license-compatibility-decision-checklist.md
    - .project/Milestone_7-Production-Hardening/Epic_4/prd-api-stabilization-pre-release-preparation.md
    - .project/Milestone_7-Production-Hardening/overview/Milestone_7-Tier_4-Production_Hardening.md

key-decisions:
  - "Checkpoint answer (verbatim, recorded 2026-08-08 by the repository owner via an interactive orchestrator checkpoint): option-a — the dual expression MIT OR Apache-2.0."
  - "ADR-0025 written recording the decision, approver DF3NDR, date 2026-08-08, and both branches fairly."

requirements-completed: [SEC-02]

coverage:
  - id: D1
    description: "All eleven manifests (root + ten library crates) declare license = \"MIT OR Apache-2.0\", collapsing to one distinct expression under a sort -u count"
    requirement: "SEC-02"
    verification:
      - kind: unit
        ref: "grep -h '^license = ' Cargo.toml crates/*/Cargo.toml (scoped to the eleven governed manifests) | sort -u | wc -l"
        status: pass
      - kind: integration
        ref: "cargo metadata --offline --no-deps --format-version 1 && cargo check --offline --workspace"
        status: pass
    human_judgment: false
  - id: D2
    description: "LICENSE renamed to LICENSE-MIT (git mv, history preserved) and verbatim LICENSE-APACHE added; README badge/section and Dockerfile.chef OCI label updated to match"
    requirement: "SEC-02"
    verification:
      - kind: unit
        ref: "test -f LICENSE-MIT && test -f LICENSE-APACHE && test ! -e LICENSE; git log --follow --oneline -1 -- LICENSE-MIT; grep -c 'Apache License' LICENSE-APACHE; wc -l < LICENSE-APACHE"
        status: pass
    human_judgment: false
  - id: D3
    description: "ADR-0025 written in the ADR-0022/0023 seven-heading shape recording the decision, approver, date, and both branches fairly; source documents annotated in place with original text retained"
    requirement: "SEC-02"
    verification:
      - kind: unit
        ref: "head -1 .planning/decisions/0025-licence-posture.md; grep '^## ' matches the seven-heading template; grep -c '(rejected)'; grep -cE '^(must change|no change needed)$'; git diff -- checklist/PRD/overview shows only additions (0 removed lines)"
        status: pass
    human_judgment: false

duration: ~10min (Task 2 + Task 3 execution)
completed: 2026-08-08
status: complete
---

# Phase 9 Plan 05: Licence Posture — MIT OR Apache-2.0 Summary

**Relicensed the root package and all ten library crates from `MIT` to `MIT OR Apache-2.0`, confirming the signed 551-package compatibility checklist over the PRD's single-licence claim, per a blocking human checkpoint.**

## Checkpoint Answer (Task 1)

**Task 1 was a `checkpoint:decision` with `gate="blocking"`.** The orchestrator put the decision to
the repository owner interactively, presenting both options (`option-a` dual expression / `option-b`
single expression) with their full pros, cons, and file-level consequences as written in the plan.

**Response, received 2026-08-08, from the repository owner (via an interactive orchestrator
checkpoint):**

> **`option-a` — the dual expression `MIT OR Apache-2.0`.**

Task 1's other acceptance criterion — that no file under `Cargo.toml`, `crates/*/Cargo.toml`,
`LICENSE*`, `README.md` or `Dockerfile.chef` was modified before the checkpoint returned — was
satisfied: this worktree forked from a clean base and none of those files had been touched prior to
the checkpoint answer arriving in this plan's prompt.

**Branch executed: option-a (dual expression).**

## Performance

- **Duration:** ~10 min (Task 2 + Task 3 execution; Task 1 was pre-answered before this agent spawned)
- **Started:** 2026-08-08T03:2x:xxZ (worktree spawn)
- **Completed:** 2026-08-08T03:29:14Z
- **Tasks:** 3 (Task 1 pre-answered; Task 2 and Task 3 executed by this agent)
- **Files modified:** 20 (16 in Task 2's commit, 4 in Task 3's commit)

## Accomplishments

- All eleven manifests (root `Cargo.toml` + ten `crates/*/Cargo.toml`) now declare
  `license = "MIT OR Apache-2.0"` — verified with `grep -h '^license = ' ... | sort -u | wc -l` = 1,
  and each manifest individually confirmed to carry exactly one `license` line.
- `LICENSE` renamed to `LICENSE-MIT` via `git mv` (history preserved, confirmed with
  `git log --follow`); a new `LICENSE-APACHE` added carrying the canonical Apache License,
  Version 2.0 text verbatim (201 lines, sourced from the local Rust toolchain's own
  `share/doc/cargo/LICENSE-APACHE` — an authoritative, unmodified copy of the ASF text — with the
  boilerplate appendix left unfilled exactly as the canonical template specifies).
- `README.md`'s licence badge and `## License` section updated to name both licences and the
  reader's choice between them, with no remaining link to the old `LICENSE` filename.
- `Dockerfile.chef:87`'s `LABEL org.opencontainers.image.licenses` updated to
  `"MIT OR Apache-2.0"` — re-derived at line 87 (not the stale `:93` `09-RESEARCH.md` recorded; Plan
  09-03's earlier nine-line deletion moved it).
- `CHANGELOG.md` records the relicensing under `## [Unreleased]` → `### Changed`, naming the
  approver, the date, and stating explicitly that it is an additional grant to existing consumers,
  not a restriction.
- `deny.toml` confirmed to need no change: its `[licenses] allow` list already contains both `MIT`
  and `Apache-2.0` (`deny.toml:25-26`); `git diff -- deny.toml` is empty.
- `cargo metadata --offline --no-deps --format-version 1` and `cargo check --offline --workspace`
  both exit 0 against the edited manifests (full cold workspace compile succeeded).
- ADR-0025 written in the exact ADR-0022/0023 seven-heading shape (no frontmatter): `## Status`,
  `## Context`, `## Decision`, `## Considered Options`, `## Code Locations`, `## Code Conformance`,
  `## Downstream Consumers`. Records the approver (`DF3NDR`), the date (2026-08-08), and both
  branches fairly — the rejected branch (option-b) is written out in full, not as a strawman.
- The signed compatibility checklist annotated in place as **confirmed and now declared**, original
  text retained below the banner, no original line removed.
- The M7 Epic 4 PRD §4.7.7 and the M7 overview Acceptance Criterion 1 each annotated in place as
  **superseded by ADR-0025**, original text retained.

## Task Commits

1. **Task 2: Execute the selected branch across every site that states a licence** —
   `6bf860f` (feat) — all eleven manifest edits, `LICENSE` → `LICENSE-MIT` rename, new
   `LICENSE-APACHE`, `README.md`, `Dockerfile.chef`, `CHANGELOG.md`.
2. **Task 3: Annotate the source documents and write ADR-0025** —
   `74a05fe` (docs) — `.planning/decisions/0025-licence-posture.md`, checklist annotation, PRD
   annotation, overview annotation.

**Plan metadata:** SUMMARY.md commit follows this summary (per worktree protocol, committed
immediately after this file is written).

_Note: Task 1 (`checkpoint:decision`) was answered by the repository owner before this agent was
spawned; no commit was produced by Task 1 itself since it changed no files._

## Files Created/Modified

- `Cargo.toml` — root package `license` field → `"MIT OR Apache-2.0"`.
- `crates/{paladin-core,paladin-ports,paladin-battalion,paladin-herald,paladin-llm,paladin-memory,paladin-storage,paladin-notifications,paladin-content,paladin-web}/Cargo.toml` — same field, all ten.
- `LICENSE-MIT` — the former `LICENSE`, renamed via `git mv`, content unchanged.
- `LICENSE-APACHE` — new, verbatim Apache License 2.0 text (201 lines).
- `README.md` — licence badge (line 7) and `## License` section updated for dual licensing.
- `Dockerfile.chef` — OCI `licenses` label (line 87) updated.
- `CHANGELOG.md` — `## [Unreleased]` → `### Changed` entry added.
- `.planning/decisions/0025-licence-posture.md` — new ADR.
- `.project/Milestone_7-Production-Hardening/Epic_4/license-compatibility-decision-checklist.md` — annotated confirmed/declared.
- `.project/Milestone_7-Production-Hardening/Epic_4/prd-api-stabilization-pre-release-preparation.md` — §4.7.7 annotated superseded.
- `.project/Milestone_7-Production-Hardening/overview/Milestone_7-Tier_4-Production_Hardening.md` — Acceptance Criterion 1 annotated superseded.

## Decisions Made

- **Selected `option-a` (dual expression)** per the checkpoint answer recorded above — matches the
  signed 551-package compatibility checklist, is the only safe direction on already-published
  `0.1.0` crates, and preserves the dual-licence approval rule's rationale for `r-efi`'s
  `MIT OR Apache-2.0 OR LGPL-2.1-or-later` expression.
- **Sourced the canonical Apache-2.0 text from the local Rust toolchain distribution**
  (`/usr/local/rustup/toolchains/stable-x86_64-unknown-linux-gnu/share/doc/cargo/LICENSE-APACHE`)
  rather than fetching from the network, because the plan's own environment notes record
  `crates.io` returning HTTP 403 in this session. This file is the ASF-authored canonical text
  shipped by `cargo` itself (a Rust Foundation project), byte-for-byte the standard form used
  across the Rust ecosystem's dual-licensed crates, confirmed against `/usr/share/common-licenses/Apache-2.0`
  (Debian's copy) as semantically identical (same nine sections, same appendix, differing only in
  leading whitespace/http-vs-https in the header). The bracketed appendix fields were left unfilled
  exactly as the canonical template specifies — not populated with project-specific text — per the
  plan's explicit instruction that any deviation from verbatim text is a legal risk with no
  corresponding benefit.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking, scope-only] Plan's automated `<verify>` glob for Task 2 is broader than its own acceptance criteria**
- **Found during:** Task 2 verification
- **Issue:** The plan's `<verify>` block for Task 2 runs `grep -c "^license = " "$f"` over the glob
  `crates/*/Cargo.toml`, which also matches `crates/doc-examples/Cargo.toml` — a `publish = false`,
  internal-only crate that has never carried a `license` field and is **not** one of the plan's
  "eleven manifests" (root + ten library crates named explicitly in `files_modified` and the
  acceptance criteria). Running the literal script fails on `doc-examples` with `BAD ...Cargo.toml=0`.
- **Fix:** No code change made — `doc-examples/Cargo.toml` is out of this plan's scope (not in
  `files_modified`, `publish = false`, pre-existing state unrelated to the licence posture). Ran the
  verification scoped exactly to the eleven manifests the plan's acceptance criteria name instead of
  the plan's broader literal glob; this scoped run passes (`DISTINCT=1`, `ALL ELEVEN OK`,
  `MANIFESTS PARSE`).
- **Files modified:** None (verification-only).
- **Verification:** `bash -c '...'` scoped to the eleven files — see the transcript in this plan's
  execution; also confirmed by the full-workspace `cargo check --offline --workspace` succeeding.
- **Committed in:** N/A (no code change; documented here per the scope-boundary rule for
  deferred/out-of-scope items — this is not tracked in `deferred-items.md` since it requires no
  future action, just a documented interpretation of the plan's own verify-script glob).

**2. [Rule 2 - Missing declared scope] Two annotation targets required by Task 3's action/acceptance criteria were absent from the plan's frontmatter `files_modified` list**
- **Found during:** Task 3
- **Issue:** The plan's frontmatter `files_modified` list omits
  `.project/Milestone_7-Production-Hardening/Epic_4/prd-api-stabilization-pre-release-preparation.md`
  and `.project/Milestone_7-Production-Hardening/overview/Milestone_7-Tier_4-Production_Hardening.md`,
  yet Task 3's `<action>` explicitly instructs annotating "the M7 Epic 4 PRD §4.7.7 and the M7
  overview AC 1 single-licence claims as superseded by ADR-0025" under option-a, and Task 3's
  `<acceptance_criteria>` explicitly checks for superseded markers in both files.
- **Fix:** Annotated both files in place per D-00c (banner + original text retained, nothing
  rewritten or deleted), matching the same pattern used for the checklist annotation that **is**
  in `files_modified`.
- **Files modified:**
  `.project/Milestone_7-Production-Hardening/Epic_4/prd-api-stabilization-pre-release-preparation.md`,
  `.project/Milestone_7-Production-Hardening/overview/Milestone_7-Tier_4-Production_Hardening.md`.
- **Verification:** `git diff -- <file> | grep -cE '^\-[^-]'` returns `0` for both (additive-only);
  `grep -ci "superseded" <file>` returns `1` for both.
- **Committed in:** `74a05fe` (Task 3 commit).

---

**Total deviations:** 2 (1 scope-boundary documentation note requiring no code change, 1 auto-add of
missing declared scope). **Impact on plan:** Both are necessary to satisfy the plan's own explicit
acceptance criteria; no unrequested scope creep occurred, and no source outside the plan's stated
intent was touched.

## Issues Encountered

None beyond the two deviations documented above. The full-workspace `cargo check --offline --workspace`
cold build succeeded (per the memory note on worktree cold-build cost, this was expected and budgeted
for since this plan touches all eleven `Cargo.toml` manifests).

## User Setup Required

None — no external service configuration required.

## Next Phase Readiness

- SEC-02 is closed: the project's licence posture has one answer, declared identically by the root
  package, all ten library crates, `deny.toml`, `LICENSE-MIT`/`LICENSE-APACHE`, `README.md`, and
  `Dockerfile.chef`'s OCI label.
- Phase 9 / Plan 07 (close-out) is the next consumer: it should advance
  `.planning/decisions/PROMOTION.md`'s ADR-numbering index to include 0025 (left untouched by this
  plan, per instruction) and cite ADR-0025 in the Milestone 7-8 ledger's licence sign-off row
  (HARD-01), replacing direct citations of the now-annotated checklist and PRD claims.
- No publish occurred in this phase (crates.io returns HTTP 403 in this environment; publishing a
  new version under the dual expression is out of Phase 9's scope and remains CI-only / a future
  release-cycle action, recorded as such in ADR-0025's Downstream Consumers section rather than
  inferred as done).
- No blockers for downstream plans in this wave.

## Self-Check: PASSED

- FOUND: LICENSE-MIT
- FOUND: LICENSE-APACHE
- FOUND: .planning/decisions/0025-licence-posture.md
- FOUND: .planning/phases/09-release-security-gate-integrity/09-05-SUMMARY.md
- FOUND commit: 6bf860f (Task 2)
- FOUND commit: 74a05fe (Task 3)
- FOUND commit: 6e05f9d (SUMMARY.md)

---
*Phase: 09-release-security-gate-integrity*
*Completed: 2026-08-08*
