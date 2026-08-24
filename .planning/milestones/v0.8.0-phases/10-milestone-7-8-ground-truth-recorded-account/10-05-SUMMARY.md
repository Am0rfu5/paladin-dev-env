---
phase: 10-milestone-7-8-ground-truth-recorded-account
plan: 05
subsystem: infra
tags: [adr, cargo-features, security-governance, docs, ground-truth, pdf]

# Dependency graph
requires:
  - phase: 10-milestone-7-8-ground-truth-recorded-account
    provides: "10-04's recorded checkpoint answer (q2-delete) for the inert pdf feature's disposition"
provides:
  - "ADR-0032: PDF extraction is unconditional in every build of paladin-content; the inert pdf feature is deleted"
  - "The corrected RUSTSEC-2026-0187 reachability path in .cargo/audit.toml, positively stated"
  - "The consumer-visible cost of the feature removal, recorded in crates/paladin-content/CHANGELOG.md"
  - "Dated inline corrections on M7 Epic 1 PRD §4.4.1 and §4.4.6, naming ADR-0032"
affects: [10-07-hard-06-ledger-row, 10-10-close-out, phase-12-supply-02, phase-11-facade-residue]

# Tech tracking
tech-stack:
  added: []
  patterns: ["ADR must-change shape with deletion-only fix (ADR-0027 analog)", "D-00c inline strike-and-correct annotation, second dated block in a document already annotated by a sibling plan"]

key-files:
  created:
    - .planning/decisions/0032-pdf-extraction-capability.md
  modified:
    - crates/paladin-content/Cargo.toml
    - crates/paladin-content/CHANGELOG.md
    - .cargo/audit.toml
    - .project/Milestone_7-Production-Hardening/Epic_1/prd-extract-infrastructure-crates.md

key-decisions:
  - "q2-delete executed: the inert pdf = [] feature is deleted from crates/paladin-content/Cargo.toml, not wired or kept — per 10-04's recorded checkpoint answer"
  - "The RUSTSEC-2026-0187 comment is corrected to state the actual reachability path positively (pdf-extract unconditional in paladin-content; paladin-content optional in the facade), not merely negating the old wrong parenthetical"
  - "The scraper/rss/tiktoken-rs dead-optional-dependency finding is recorded in ADR-0032 as an adjacent, out-of-scope finding for Phase 11 or Phase 15, not fixed in this plan"

requirements-completed: [HARD-06]

coverage:
  - id: D1
    description: "ADR-0032 answers HARD-06 from source-level evidence (zero cfg(feature = \"pdf\") matches, unconditional pdf-extract dependency, news-api comparator), records the q2-delete disposition and its accepted cost, and corrects the RUSTSEC-2026-0187 reachability reasoning"
    requirement: "HARD-06"
    verification:
      - kind: other
        ref: "grep -c 'must change'==1, grep -c '(rejected)'==3, grep -c 'news-api'==8, grep -c 'document_adapter'==4, grep -ci 'HTTP 403|cannot be run'==2, grep -c 'SUPPLY-0'==1, grep -ci scraper==3 against .planning/decisions/0032-pdf-extraction-capability.md — all thresholds met (see Self-Check)"
        status: pass
    human_judgment: false
  - id: D2
    description: "The [features] block in crates/paladin-content/Cargo.toml drops the inert pdf entry (pure subtraction, five entries survive in order), CHANGELOG.md records the consumer-visible build cost, and .cargo/audit.toml's RUSTSEC-2026-0187 comment states the true reachability path while the suppression itself is untouched"
    requirement: "HARD-06"
    verification:
      - kind: other
        ref: "grep -cE '^pdf +='==0, grep -cE '^news-api +='==1, git diff --numstat shows 0 added/1 deleted for Cargo.toml, cargo metadata --no-deps --offline exits 0, grep -c RUSTSEC-2026-0187==1 unchanged, grep -c 'optional `content-processing`'==0, grep -c ADR-0032 (CHANGELOG)>=1"
        status: pass
    human_judgment: false
  - id: D3
    description: "M7 Epic 1 PRD §4.4.1 and §4.4.6 each carry a dated inline correction naming ADR-0032, original wording retained struck, plan 10-04's HARD-05 corrections in the same file untouched"
    requirement: "HARD-06"
    verification:
      - kind: other
        ref: "grep -c 'Corrected (dated 2026-08-08, HARD-06)'==2, grep -c 'Corrected (dated 2026-08-08, HARD-05)'==2 (unchanged), grep -c ADR-0032>=3, git diff --numstat shows 19 insertions/2 deletions"
        status: pass
    human_judgment: false

duration: 12min
completed: 2026-08-08
status: complete
---

# Phase 10 Plan 05: PDF Extraction Capability (ADR-0032) Summary

**ADR-0032 settles HARD-06 from source-level evidence (zero `cfg(feature = "pdf")` matches in
`paladin-content/src/`, an unconditional `pdf-extract` dependency, and the `news-api` comparator
proving an empty feature is not itself the defect): PDF extraction ships, always, in every build,
and the inert `pdf = []` feature — per 10-04's recorded `q2-delete` answer — is deleted, its
consumer-visible cost recorded in `CHANGELOG.md`, and `.cargo/audit.toml`'s `RUSTSEC-2026-0187`
comment corrected to state the true reachability path.**

## Performance

- **Duration:** 12 min
- **Started:** 2026-08-08T16:10:00Z
- **Completed:** 2026-08-08T16:22:00Z
- **Tasks:** 3 (all auto)
- **Files modified:** 5 (1 created, 4 modified)

## Accomplishments

- Wrote `.planning/decisions/0032-pdf-extraction-capability.md` — seven canonical headings, no
  frontmatter, `must change`, naming plan 10-05 tasks 2 and 3 as executor. Context section cites
  the three surface-level facts, the fourth source-level fact that settles them (zero
  `cfg(feature = "pdf")` matches; `document_adapter.rs:22,29,123,132`'s ungated `PdfExtractor`
  field/constructor/call-sites; `news-api`'s byte-identical but legitimate comparator), the
  corrected advisory reasoning, an explicit "measured vs. established" scope for `cargo
  audit`/`cargo deny` (unrunnable here, HTTP 403), and the adjacent `scraper`/`rss`/`tiktoken-rs`
  dead-dependency finding (confirmed zero source references this session, handed to Phase 11 or
  15, not fixed here).
- Deleted `pdf = []` from `crates/paladin-content/Cargo.toml:18` — pure subtraction, zero added
  lines, five surviving `[features]` entries in unchanged relative order, `news-api = []` left
  untouched as the legitimate comparator. `cargo metadata --no-deps --format-version 1 --offline`
  exits 0 after the edit — the manifest still parses.
- Recorded the consumer-visible cost in `crates/paladin-content/CHANGELOG.md` under a new
  `### Removed` heading: the feature gated no dependency and no code, PDF extraction is
  unaffected, `cargo build -p paladin-content --features pdf` begins to fail where it previously
  succeeded-and-did-nothing, cited to ADR-0032.
- Corrected `.cargo/audit.toml:26-29`'s `RUSTSEC-2026-0187` comment: replaced the parenthetical
  attributing `pdf-extract`'s optionality to the facade's `content-processing` feature with a
  positive statement of the actual path — `pdf-extract` is unconditional inside
  `paladin-content`; reachability is gated one level up by whether the facade's optional
  `paladin-content` dependency is enabled. The `ignore` array, the `-0194`/`-0195` entry, and every
  other line of the file are untouched; `SECURITY-EXCEPTIONS.md` and `deny.toml` are untouched
  (`git status --porcelain` confirms).
- Annotated M7 Epic 1 PRD §4.4.1 and §4.4.6 with a second dated head blockquote (placed beneath,
  not merged into, plan 10-04's existing HARD-05 blockquote) plus inline strike-and-correct
  treatment on each clause: §4.4.1's "`pdf` (gates `pdf-extract`)" phrase struck and corrected
  with the exact `grep` command and its zero-match result; §4.4.6's "with all capability features
  enabled" phrase struck and corrected, noting the facade's list omits `pdf` harmlessly since it
  gates nothing.

## Task Commits

1. **Task 1: Write ADR-0032** — `63717a4` (feat)
2. **Task 2: Execute the two config changes and record the consumer cost** — `a1559f3` (fix)
3. **Task 3: Annotate M7 Epic 1 PRD §4.4.1 and §4.4.6** — `2bc1e15` (docs)

_No plan-metadata commit in this plan — worktree mode: STATE.md/ROADMAP.md updates are owned by
the orchestrator after all wave agents complete, per this plan's execution instructions. This
SUMMARY.md is committed separately below, per the parallel-executor protocol._

## Files Created/Modified

- `.planning/decisions/0032-pdf-extraction-capability.md` — new ADR settling HARD-06: PDF
  extraction is unconditional; the inert `pdf` feature is deleted (`q2-delete`); the
  `RUSTSEC-2026-0187` reachability path is corrected.
- `crates/paladin-content/Cargo.toml` — `[features]` block: `pdf = []` deleted (line 18,
  pre-edit); five entries survive (`web-scraping`, `rss`, `news-api`, `tiktoken`, `llm`) in
  unchanged relative order. `pdf-extract` dependency (`:41`, post-edit `:40`) untouched.
- `crates/paladin-content/CHANGELOG.md` — new `### Removed` entry under `## [Unreleased]`
  recording the feature name, that it gated nothing, and the consumer-visible build cost, citing
  ADR-0032.
- `.cargo/audit.toml` — `RUSTSEC-2026-0187` comment block corrected to state the true reachability
  path; suppression (`ignore` array) unchanged.
- `.project/Milestone_7-Production-Hardening/Epic_1/prd-extract-infrastructure-crates.md` — §4.4.1
  (line 151, pre-edit) and §4.4.6 (line 184, pre-edit) struck and corrected in place; a second
  dated head blockquote added beneath plan 10-04's existing one.

**Pre-edit `[features]` block, recorded verbatim (per this plan's `<output>` instruction):**
```toml
[features]
pdf          = []
web-scraping = ["dep:scraper"]
rss          = ["dep:rss"]
news-api     = []
tiktoken     = ["dep:tiktoken-rs"]
llm          = ["dep:paladin-llm"]
```

**CI command for the audit tooling, and confirmation no local pass was observed (per this plan's
`<output>` instruction):** `cargo audit --config .cargo/audit.toml` and `cargo deny check
advisories` are the two commands a CI runner executes against the reconciled configuration.
Neither tool could run in this environment — `crates.io` returns HTTP 403 here, unchanged from
Phase 9 (`.planning/phases/04-release-coherence/04-ci-gate-deferrals.md`) — so no local pass was
observed for either, and none is claimed. The suppression list itself (`ignore = [...]`) is
unmodified by this plan, so both commands are expected to behave identically to before this
change once they do run in CI.

## Decisions Made

- Executed `q2-delete` exactly as recorded in `10-04-SUMMARY.md`: deleted `pdf = []` rather than
  wiring it or keeping it as a documentation marker. The accepted cost (`cargo build -p
  paladin-content --features pdf` begins failing where it previously silently succeeded) is
  recorded in `CHANGELOG.md` and in ADR-0032's `## Decision` section, as required.
- Struck only the defective sub-phrase in each PRD clause (`` `pdf` (gates `pdf-extract`), `` in
  §4.4.1; `(with all capability features enabled)` in §4.4.6) rather than the entire sentence,
  since the surrounding text in both clauses remains true (the other four feature flags in
  §4.4.1 are correctly described; the rest of §4.4.6's redefinition claim is accurate). This
  keeps the annotation maximally precise while still satisfying the "wrap the clause... word for
  word" instruction and the numstat acceptance criterion (each clause's single source line
  replaced in place, 1 deletion each, 2 total).
- Recorded the `scraper`/`rss`/`tiktoken-rs` dead-optional-dependency finding in ADR-0032's
  Context and Downstream Consumers sections (confirmed via `grep -rn "scraper::\|tiktoken_rs\|::rss::"
  crates/paladin-content/src/` returning zero matches this session) rather than fixing it — it is
  explicitly out of HARD-06's scope per `10-CONTEXT.md`'s Phase Boundary section, and D-23's
  three-file config-surface boundary does not include these dependency lines.

## Deviations from Plan

None — plan executed exactly as written. The checkpoint (10-04's blocking decision) was answered
by the human before this executor ran; task 2's precondition (`10-04-SUMMARY.md` records
`q2-delete`, not `q2-wire`) was met, so the task ran in its recommended form without halting.

## Issues Encountered

None. All `file:line` citations recorded in `10-CONTEXT.md`, `10-PATTERNS.md`, and
`10-RESEARCH.md` (Cargo.toml:18/21/41, document_adapter.rs:22/29/123/132, input/mod.rs:5,
audit.toml:26-29, PRD §4.4.1 at line 151 and §4.4.6 at line 184) were re-verified this session and
matched exactly — no drift since 10-04 or since the research session.

## Self-Check

Verified against the plan's acceptance criteria for Task 1 (`0032-pdf-extraction-capability.md`):
- Heading set matches exactly: `## Status`, `## Context`, `## Decision`, `## Considered Options`,
  `## Code Locations`, `## Code Conformance`, `## Downstream Consumers`, no frontmatter, first
  line `# ADR-0032:...` (not `---`) — confirmed via `diff`.
- `grep -cx 'must change'` → `1`, followed by a line naming plan 10-05 tasks 2 and 3.
- `grep -c '(rejected)'` → `3` (all three alternative dispositions plus change-nothing tagged).
- `grep -c 'news-api'` → `8` (≥1 required).
- `grep -c 'document_adapter'` → `4` (≥1 required).
- `grep -ci 'HTTP 403\|cannot be run in this environment\|not run in this environment'` → `2`
  (≥1 required).
- `grep -c 'SUPPLY-0'` → `1` (≥1 required).
- `grep -ci 'scraper'` → `3` (≥1 required).
- ADR names the selected branch (`q2-delete`), matching `10-04-SUMMARY.md`.
- `git status --porcelain -- '*.rs'` → empty.

Verified against the plan's acceptance criteria for Task 2 (the three config edits):
- `grep -cE '^pdf +='` → `0`; `grep -cE '^pdf-extract'` → `1` (untouched).
- `grep -cE '^news-api +='` → `1`.
- `[features]` block holds `5` entries, same relative order as pre-edit (confirmed by direct
  comparison against the pre-edit block recorded above).
- `git diff -- crates/paladin-content/Cargo.toml | grep -c '^+[^+]'` → `0`.
- `git diff --numstat -- crates/paladin-content/Cargo.toml` → `0` added, `1` deleted.
- `grep -c '"RUSTSEC-2026-0187"'` → `1` (unchanged).
- `grep -c 'optional \`content-processing\`'` (audit.toml) → `0`; `grep -c 'unconditional'` → `1`;
  `grep -c 'ADR-0032'` (audit.toml) → `1`.
- `grep -c 'RUSTSEC-2026-0194'` → `2`, `grep -c 'RUSTSEC-2026-0195'` → `1` — both unchanged from
  pre-edit values; `git diff` touches no line inside their entry.
- `git status --porcelain -- SECURITY-EXCEPTIONS.md deny.toml` → empty.
- `grep -c 'ADR-0032'` (CHANGELOG.md) → `1`, entry states the build consequence.
- `cargo metadata --no-deps --format-version 1 --offline` → exit `0`.
- `git status --porcelain -- '*.rs'` → empty.

Verified against the plan's acceptance criteria for Task 3 (PRD annotation):
- `grep -c 'Corrected (dated 2026-08-08, HARD-06)'` → `2` (exactly `2` required).
- `grep -c 'Corrected (dated 2026-08-08, HARD-05)'` → `2` (unchanged, plan 10-04's corrections
  intact).
- `grep -c 'ADR-0032'` (PRD file) → `3`; relative link `../../../.planning/decisions/0032-pdf-extraction-capability.md`
  resolves (confirmed via `ls`).
- `grep -c 'ADR-0031'` (PRD file) → `3` (unchanged from post-10-04 value).
- `git diff --numstat` → `19` insertions, `2` deletions (exactly `2` deleted lines required).
- `grep -c 'SUPERSEDED BY\|Correction (dated'` → `2` — two separate dated head blocks, neither
  merged into the other.
- `git status --porcelain -- '*.rs' crates/paladin-content/Cargo.toml .cargo/audit.toml Makefile`
  → empty (those files already committed in Task 2, unmodified by Task 3).

Commit hashes verified present: `git log --oneline --all | grep -q 63717a4` → FOUND;
`git log --oneline --all | grep -q a1559f3` → FOUND; `git log --oneline --all | grep -q 2bc1e15`
→ FOUND.

## Self-Check: PASSED

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- ADR-0032 is committed and citable by number; Phase 12 / SUPPLY-02 and SUPPLY-03 inherit the
  `pdf-extract` reachability answer rather than re-deriving it.
- Plan 10-07's ledger row for `REQ-content-processing-build-gate` can now cite ADR-0032 and record
  the `pdf`/`pdf-extract` contradiction as resolved.
- The `scraper`/`rss`/`tiktoken-rs` dead-optional-dependency finding is recorded but unowned by
  any specific phase yet — a candidate for Phase 11's facade residue work or a Phase 15
  dependency-hygiene item, named in ADR-0032's Downstream Consumers.
- No blockers. `git status --porcelain -- '*.rs'` is empty, confirmed above — D-23's boundary
  holds for this plan; only the three permitted config-surface files plus one `.project/`
  annotation were touched.

---
*Phase: 10-milestone-7-8-ground-truth-recorded-account*
*Completed: 2026-08-08*
