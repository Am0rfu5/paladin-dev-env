# Phase 5: Milestone 2-3 Ground Truth - Research

**Researched:** 2026-08-04
**Domain:** Documentation/decision-record authoring — cited status ledger, ADRs, in-place amendment,
in-repo historical-document correction. No product code changes.
**Confidence:** HIGH

## Summary

This phase produces exactly five artefact classes, all markdown, all inside a corpus that already
has a working, load-bearing example of every one of them: `.planning/ledgers/milestone-01.md` (the
ledger shape, including two rounds of in-place amendment), `.planning/decisions/0001-0009.md` (the
ADR shape, none of which use YAML frontmatter — `adr-parser.cjs` parses by H2 heading synonym
matching, not frontmatter, so 0010-0012 need no schema beyond copying 0001-0009's headings
verbatim), `.planning/decisions/0006-coverage-gate.md` (the file VERIFY-05 amends in place), and
`RELEASE_NOTES_MILESTONE_3.md` (the one non-`.planning/` file this phase edits, in an
annotate-don't-rewrite style ROADMAP.md already demonstrates on itself). All four of CONTEXT.md's
`<specifics>` code claims were re-verified directly against the tree during this research pass and
hold, with one small citation-precision correction (below) that the ADR-0011 author should apply.

The dominant cost is not technique, it is scale: 118 `REQ-*` rows across 14 epics, three deep
block-verification passes covering 81/45/29 open checkboxes (task-completion-state.md's counts,
confirmed authoritative — a naive `grep -c '\- \[ \]'` overcounts each file by exactly 2 because the
shared "Instructions for Completing Tasks" boilerplate contains two literal `- [ ]` example strings).
D-01's evidence bar (citation **plus** a named passing test/example/command) is the load-bearing
requirement across all 118 rows, and the single most useful thing this research can hand the planner
is a repeatable search recipe for finding that exercising artefact — see the "Exercising-artefact
search strategy" section.

**Primary recommendation:** Follow D-20's scaffold-first shape exactly, but sequence the three
VERIFY-02 block plans (Epic 22, Epic 14, Epic 24) as their own wave immediately after the scaffold —
not interleaved with the ledger fan-out — because they gate Phase 6's planning entirely and are the
deepest, slowest work per row. Budget real wall-clock for git hooks: `cargo fmt --check` and
`cargo clippy --workspace --all-targets --all-features -- -D warnings` run on **every** commit in
this repo, including markdown-only ones (`always_run: true` in `.pre-commit-config.yaml`), and
`cargo fmt` was observed to hang past 2 minutes on this machine — batch commits accordingly rather
than committing after every small ledger edit.

## Architectural Responsibility Map

This phase has no browser/API/database tiers — it is a documentation phase. The "architecture" that
matters is precedence and file ownership, not runtime layers.

| Capability | Primary Tier | Secondary Tier | Rationale |
|------------|-------------|----------------|-----------|
| Per-requirement cited verdicts (VERIFY-01) | `.planning/ledgers/` (new file) | `.planning/REQUIREMENTS.md` (reduced to pointer, D-21) | D-00d: one ledger file per milestone, REQUIREMENTS.md's inline copy retired |
| Block verdicts (VERIFY-02) | `.planning/ledgers/milestone-02-03.md` (nested under the relevant epic's rows, or a sibling appendix — Claude's discretion) | `.project/**/tasks-*.md` (read-only evidence source, never edited) | D-05/D-06: the parent-task cluster table is the backing evidence; the task-list files themselves are not touched |
| Epic-numbering defect (VERIFY-03) | `.planning/decisions/0010-*.md` (new ADR) | `.project/Milestone_3-Completion/RELEASE_NOTES_MILESTONE_3.md` (in-repo correction) | D-07: recorded-once-permanently lives in the ADR; the propagating document gets an annotated correction, not a rewrite |
| Vision surfaces + encryption disposition (VERIFY-04) | `.planning/decisions/0011-*.md` (new ADR) | none — code consequence is Phase 6 CLOSE-03 | D-09/D-10/D-11: this phase records, does not wire |
| Coverage answer (VERIFY-05) | `.planning/decisions/0006-coverage-gate.md` (amended in place) | none | D-12: no second ADR; ADR-0006 already names Phase 5 as the module-gate owner |
| Live-API-test behaviour (VERIFY-06) | `.planning/decisions/0012-*.md` (new ADR) | `tests/integration/llm_live_api_tests.rs` doc comment (Phase 6 CLOSE-03, not this phase) | D-17/D-18: the code consequence is one doc-comment line, explicitly deferred to Phase 6 |

## Standard Stack

Not applicable — no libraries are introduced or evaluated by this phase. `Package Legitimacy Audit`
is skipped: this phase installs no packages (see `<package_legitimacy_protocol>` — the gate is
conditioned on "every phase that installs external packages"; this one does not).

## Package Legitimacy Audit

**Skipped.** This phase writes markdown only. No `Cargo.toml`, `package.json`, or any manifest is
touched by any of VERIFY-01…06. The one exception CONTEXT.md documents — `Cargo.toml:134-135`
(`chacha20poly1305`, `zeroize`) — is **cited, not added**; both dependencies already shipped
unconditionally before this phase began (confirmed: `sed -n '130,140p' Cargo.toml` shows both lines
present, uncommented, with no feature gate).

## Architecture Patterns

### System Architecture Diagram

Not a runtime system. The "flow" that matters is the evidence chain each ledger row and ADR must
satisfy, traced end to end:

```
REQ-* ID (REQUIREMENTS.md "Milestone 2-3 as-shipped ledger", ~line 2653)
        │
        ▼
  grep/read the current tree for the cited artefact
  (path caveat: PRD paths are pre-Milestone-5; use codebase/STRUCTURE.md
   or the ledger's own head-note caveat, not the PRD's literal src/... path)
        │
        ▼
  file:line citation confirmed against the shipped tree
        │
        ├── nothing exercises it ─────────────► verdict: present, unproven
        │
        ▼
  search for a named passing test / example / command
  (cargo metadata --no-deps; grep -rn "fn test_<keyword>" tests/;
   ls examples/*.rs; grep -rn "<Requirement's own type/fn name>")
        │
        ├── found, and it passes ─────────────► verdict: satisfied
        ├── found, contradicts the PRD ───────► verdict: superseded by shipped code
        ├── nothing found in tree at all ─────► verdict: genuinely outstanding
        └── explicit named-owner deferral ────► verdict: deferred with reason
```

For the three VERIFY-02 blocks, the same chain runs once per **parent-task cluster** (D-05), not
once per checkbox:

```
tasks-epic22-...md parent task "5.0 Implement Grove LLM-based routing"
        │
        ▼
  read the 34 open subtask lines under 5.0 as one capability claim
        │
        ▼
  verify the capability against grove_service.rs (does LLM routing exist? what's missing?)
        │
        ▼
  one row in the cluster table: parent task → verdict → evidence
        │
        ▼
  all clusters for the block roll up into ONE block verdict (D-06)
```

### Recommended Project Structure

No new source directories. The only new/changed files:

```
.planning/
├── ledgers/
│   └── milestone-02-03.md          # NEW — sibling to milestone-01.md
├── decisions/
│   ├── 0006-coverage-gate.md       # AMENDED IN PLACE (VERIFY-05)
│   ├── 0010-milestone-3-epic-numbering.md   # NEW (VERIFY-03) — slug is discretion
│   ├── 0011-vision-port-surfaces.md         # NEW (VERIFY-04) — or combined w/ 0012, discretion
│   ├── 0012-live-api-test-key-behaviour.md  # NEW (VERIFY-06)
│   └── PROMOTION.md                # UPDATE "Next free ADR number" line to 0013
└── REQUIREMENTS.md                 # "Milestone 2-3 as-shipped ledger" section reduced to a pointer (D-21)

.project/Milestone_3-Completion/
└── RELEASE_NOTES_MILESTONE_3.md    # AMENDED IN PLACE — banner + inline corrections (D-08)
```

### Pattern 1: Ledger head notes (copy from milestone-01.md, adapt for 118 rows)

**What:** Every sibling ledger opens with (a) the supersession statement pointing at the
REQUIREMENTS.md section it replaces, (b) the primary-key statement (`REQ-*` ID, D-00e), (c) the
evidence-bar statement (D-01's citation+exerciser bar, verbatim or adapted), and (d) — new for this
phase, per D-04 — the two systematic path caveats stated **once**, not per row.

**When to use:** The scaffold plan (D-20 step 1), before any per-epic row is authored.

**Example (adapted from `milestone-01.md:1-19`):**
```markdown
# Milestone 2-3 cited status ledger

This file **supersedes** `REQUIREMENTS.md`'s `## Milestone 2-3 as-shipped ledger` section (D-21).
That section becomes a pointer to this file.

**Primary key: the `REQ-*` requirement ID.** [... D-00e text ...]

**Evidence bar.** A `satisfied` verdict requires a `file:line` citation **plus** a named passing
test, example, or command that exercises it. [... D-01 text, no lowered bar for ingest `Shipped`
rows — an ingest `Shipped` verdict is the bare "code exists" claim D-01 exists to reject ...]

**Path caveats (apply once, not per row):**
(a) Every `src/core|application|infrastructure` path in the run-2 PRDs predates the Milestone 5
    workspace decomposition. Current layout: `.planning/codebase/STRUCTURE.md`.
(b) Milestone-1 benchmark files those PRDs reference have relocated into per-crate `benches/`
    directories.
A row whose only divergence from its PRD is (a) or (b) is `superseded by shipped code` pointing at
this note, not a fresh divergence write-up (D-04).
```

### Pattern 2: ADR shape — no frontmatter needed

**What:** `.claude/gsd-core/bin/lib/adr-parser.cjs` parses ADRs by **H2 heading text**, matched
against a synonym table (`CANONICAL_HEADERS`), not by YAML frontmatter. `## Status`'s first body
line is matched against `accepted|proposed|superseded|rejected|deprecated`. `## Code Conformance`
and `## Downstream Consumers` have no synonym entry and land in `unmapped_headers` — parsed but
unused by any consumer today. This resolves the open question CONTEXT.md's discretion note raised:
**0001-0009 already satisfy the parser exactly as they are (no frontmatter, seven H2 headings in
PROMOTION.md's required order); 0010-0012 need nothing more than the same shape.**

**When to use:** Every new ADR this phase writes.

**Example (verified heading set, from `0006-coverage-gate.md` and PROMOTION.md's own statement):**
```markdown
# ADR-0010: <title>

## Status
Accepted
**Date:** 2026-08-04

## Context
...

## Decision
...

## Considered Options
- option — rejected/chosen; reason
...

## Code Locations
- `path/to/file.rs:NN` — what's there

## Code Conformance
conforms | must change

[if must change: names the requirement that executes it]

## Downstream Consumers
- Phase N (REQ-ID) — what it does with this decision
```
**Note:** `## Code Locations` and `## Considered Options` **must be bulleted lists** — the parser's
`splitEntries` only yields structured entries from `-`/`*`/`+`/numbered lines; a prose paragraph
collapses into one opaque blob (PROMOTION.md, "Required heading set").

### Pattern 3: In-place amendment (ADR-0006 and the ledger)

**What:** A dated, additive amendment section is appended; superseded text is retained with a
`**(Amended by Phase N, dated ..., citing ...)**` inline marker wrapping the corrected clause, never
deleting the original. `milestone-01.md`'s "Phase 2/3/4 amendments" sections and `0006-coverage-gate.md`
itself (which is the file VERIFY-05 amends) are the two working examples in this corpus.

**When to use:** VERIFY-05's ADR-0006 amendment; any later correction to `milestone-02-03.md` itself
(future phases, not this one).

**Example (the exact inline marker style used throughout `milestone-01.md`):**
```markdown
**(Amended by Phase 5, dated 2026-08-04, citing `01-coverage-measurement.md`'s per-file rows: Herald
is separately recorded at 80.49% line coverage against its 95% module target — the ~15-point gap is
transcribed here, not re-measured.)**
```

### Pattern 4: Annotate-don't-rewrite historical-document correction

**What:** A dated correction banner at the top of `RELEASE_NOTES_MILESTONE_3.md` naming what's wrong
and pointing at ADR-0010; each defective claim corrected inline with the original retained and
marked superseded. `ROADMAP.md` already does this to itself
(`**Amended by Phase 4, dated 2026-08-03, citing …**`) — that is the pattern to copy, not invent.

**When to use:** VERIFY-03's D-08 correction (the only in-repo, outside-`.planning/` edit this phase
makes).

**Example shape:**
```markdown
> **Correction (dated 2026-08-04, ADR-0010):** This document's Epic 19-24 numbering does not match
> the authoritative plan/epic-definition set. See ADR-0010 for the corrected mapping. Original text
> retained below with inline corrections; nothing is deleted.

### ~~Epic 19: Conclave Pattern (Multi-Expert Synthesis)~~ Epic 19: Herald & Domain Type Consolidation
**Corrected numbering (ADR-0010):** this section's content (Conclave) is **Epic 15**, not Epic 19.
[... original text retained below ...]
```

### Anti-Patterns to Avoid

- **Re-deriving the 81/45/29 open-checkbox counts by a fresh grep.** A naive
  `grep -c '\- \[ \]' <file>` overcounts each of the three block files by exactly 2 — the shared
  "Instructions for Completing Tasks" section contains two literal example strings
  (`` `- [ ] 1.1 Read file` → `- [x] 1.1 Read file` `` and the sentence describing the checkbox
  convention) that match the pattern but are not real task checkboxes. `task-completion-state.md`'s
  counts already exclude these; CONTEXT.md explicitly says do not re-derive them — use that file's
  numbers, cite it, move on.
- **Accepting an ingest `Shipped` row as pre-satisfied.** D-01 is explicit and absolute: this is the
  exact false-positive class the evidence bar exists to reject, for all 118 rows without exception.
- **Treating the vision "Not found in tree" ledger claim as still true.** It is verified false on all
  three counts (below). Do not re-derive this — cite the citations in this document.
- **Writing a second coverage ADR.** D-12 is explicit: amend 0006 in place.
- **Grepping the PRD's literal `src/core|application|infrastructure/...` paths and reporting "not
  found."** These predate the Milestone-5 workspace decomposition; the current location is in
  `.planning/codebase/STRUCTURE.md` or the ledger's own head-note caveat (Pattern 1 above).

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| Finding what tests exist for a requirement | A new test inventory or spreadsheet | `cargo metadata --no-deps --format-version=1 \| jq` for named `[[test]]` targets, plus `grep -rn "fn test_.*<keyword>"` inside `tests/` | `cargo metadata` is authoritative and free; a hand-built inventory goes stale the moment a test file is renamed |
| Counting open checkboxes | A custom parser | `.planning/intel/task-completion-state.md` (already computed, deterministic, documented caveat) | Already exists at the exact three files this phase needs; re-deriving risks the +2 boilerplate-string bug found in this research |
| Checking ADR format compliance | A YAML frontmatter schema | `.claude/gsd-core/bin/lib/adr-parser.cjs`'s own H2-heading synonym table (already ships) | The parser is already lenient by design (synonym matching); building a stricter frontmatter schema than what ships would make 0010-0012 diverge from 0001-0009 for no reason |

**Key insight:** Every tool this phase needs to find evidence already exists in the repo or the
`.planning/intel/` corpus. The work is reading and citing, not building tooling.

## Runtime State Inventory

Not applicable — this phase renames nothing and migrates no runtime data. It corrects a historical
markdown document (`RELEASE_NOTES_MILESTONE_3.md`) and writes new decision records. No database, no
external service, no OS-registered state, no secret, no build artifact is touched.

## Common Pitfalls

### Pitfall 1: Trusting REQUIREMENTS.md's existing verdict text as a starting truth rather than a hypothesis

**What goes wrong:** REQUIREMENTS.md's inline ledger already carries verdict text (`Shipped`,
`Verify`, `Variant`, etc.) for all 118 rows. It is tempting to transcribe these into the new ledger
with citations bolted on.
**Why it happens:** The ingest-run text reads confidently and the citations it already carries look
authoritative.
**How to avoid:** Treat every existing `Shipped` verdict as `present, unproven` until a named
exerciser is found (D-01/D-02). The `REQ-vision-security-encryption` row is proof this corpus's
ingest record can be wrong on every count it asserts (see Pitfall 2).
**Warning signs:** A ledger row with a citation but no test/example/command name next to it.

### Pitfall 2: The `REQ-vision-security-encryption` "Not found in tree" claim is false — verify, don't transcribe

**What goes wrong:** REQUIREMENTS.md:2704 states *"Not found in tree — no encryption-at-rest,
zeroization or retention-policy artefact was located, and Epic 20's `VisionError` omits
`EncryptionError`."* This is wrong on all three counts, confirmed by direct re-inspection during
this research pass:
- `VisionError::EncryptionError(String)` **exists**, `crates/paladin-core/src/platform/container/vision.rs:210-212` — confirmed exact line match.
- `EncryptionService` ships at `src/infrastructure/security/encryption.rs` with
  `encrypt_image_data` (:200) / `decrypt_image_data` (:217) over ChaCha20-Poly1305 — confirmed.
- `SecureData` is `#[derive(Zeroize, ZeroizeOnDrop)]` — confirmed, struct declaration line ~68.
- A retention/expiry check exists — confirmed, but see the citation-precision correction below.
- Both `chacha20poly1305 = "0.10"` and `zeroize = { version = "1.8", features = ["derive"] }` are
  **unconditional** in `Cargo.toml:134-135` — confirmed, no feature gate.
- **Zero consumers outside `src/infrastructure/security/`** — confirmed:
  `grep -rln "EncryptionService\|DataRetentionPolicy\|VisionError::EncryptionError" src/ crates/ | grep -v infrastructure/security`
  returns **empty**.

**One citation-precision correction for ADR-0011 to make that CONTEXT.md D-10 does not quite get
right:** `is_expired` at line 95 is a method on **`SecureData`**, not `DataRetentionPolicy` —
`DataRetentionPolicy` (struct at :106, impl at :122) instead has `should_retain(&self, created_at:
SystemTime) -> bool` at :131. There is no `DataRetentionPolicy::is_expired` method in the tree. The
retention capability is real and unconsumed either way — the finding (zero consumers) is unaffected —
but ADR-0011's `## Code Locations` section should cite `SecureData::is_expired` (`encryption.rs:95`)
and `DataRetentionPolicy::should_retain` (`encryption.rs:131`) as two separate methods rather than
conflating them into one `DataRetentionPolicy::is_expired` citation.
**Why it happens:** Two structs (`SecureData`, `DataRetentionPolicy`) both encode "is this data still
good" logic with different method names; easy to attribute the wrong method to the wrong struct when
skimming.
**How to avoid:** Re-read the actual `impl` blocks, not just the module's doc comment, before citing
a method name in an ADR.
**Warning signs:** A citation naming a method on a type that, on inspection, doesn't declare that
method.

### Pitfall 3: The double-gate on `llm_live_api_tests.rs` is easy to miss if you only read the PRDs

**What goes wrong:** Epic 23 FR-23.4.4 and Epic 24 US-24.7 both specify graceful skip on a missing
key; the shipped `require_api_key` panics. Read in isolation, this looks like an open contradiction
requiring a behavioural fix.
**Why it happens:** Neither PRD's author appears to have traced the module's compile-time gating.
**How to avoid:** Confirmed by direct inspection: `tests/integration/mod.rs:34-35` gates the whole
module behind `#[cfg(feature = "live-api-tests")]`, and **13** of its tests carry `#[ignore]`
(`grep -c '#\[ignore\]' tests/integration/llm_live_api_tests.rs` → 13). The module only compiles at
all via `tests/lib.rs:61`'s `pub mod integration;` (confirmed present at exactly that line). A
default `cargo test --workspace` run — no feature flag, no `--ignored` — never reaches this code.
The "graceful skip" the PRDs want is supplied by the gate, not the panic path; document that instead
of changing the panic.
**Warning signs:** Treating `require_api_key`'s panic as reachable in CI without checking the two
gates above it first.

### Pitfall 4: The panic-vs-doc-comment defect is real but tiny — don't scope-creep it into a behaviour change

**What goes wrong:** `require_api_key`'s doc comment (`tests/integration/llm_live_api_tests.rs:61`,
confirmed exact line) reads *"Skip test if API key is not present or empty, otherwise return the
key"* while both match arms of the function body (confirmed, lines ~65-83) `panic!`. It is tempting
to "fix" this by changing the panic to an actual skip.
**How to avoid:** D-17/D-18 already settle this — the panic is correct behaviour (a false pass would
be worse), only the doc comment lies. This phase's own boundary explicitly excludes code changes;
even the doc-comment fix is Phase 6 CLOSE-03's job, not this phase's. Record the finding in
ADR-0012; do not edit the `.rs` file.
**Warning signs:** A plan task in this phase that touches any `.rs` file.

### Pitfall 5: Git hooks fire on every commit, including markdown-only ones, and can hang

**What goes wrong:** `.pre-commit-config.yaml`'s `cargo-fmt` and `cargo-clippy` hooks are declared
with `always_run: true` (confirmed, lines 79/86) — this **overrides** their `types: [rust]` file
filter, so they run on **every** commit regardless of which files changed, including a commit that
only touches `.planning/ledgers/milestone-02-03.md`. `cargo clippy --workspace --all-targets
--all-features -- -D warnings` compiles the entire 12-crate workspace; `cargo fmt --all -- --check`
was independently observed to hang past 2 minutes on this machine.
**Why it happens:** The hooks are deliberately workspace-scoped ("run once per commit, not per
file" per the file's own comment) because Rust compilation errors can't be attributed to individual
files anyway — but that design choice has a real cost for a phase that commits nothing but markdown.
**How to avoid:** Do not plan one commit per ledger row or per epic-section edit. Batch each plan's
markdown changes into one commit at the end of the plan (the standard GSD unit anyway), and budget
Bash tool timeouts generously (2+ minutes) for every commit in this phase's execution. Do not use
`--no-verify` without explicit user instruction (project's own git safety protocol).
**Warning signs:** A plan decomposition that implies more than one commit per plan, or a commit step
with a short timeout.

## Code Examples

### Requirement → exercising-artefact search recipe

```bash
# 1. List every named [[test]] target in the workspace (one-time reference, cheap)
cargo metadata --no-deps --format-version=1 | \
  jq -r '.packages[].targets[] | select(.kind[0]=="test") | "\(.name)\t\(.src_path)"'
# Confirms: tests/integration/ compiles ONLY via the "lib" target (tests/lib.rs:61
# `pub mod integration;`) — individual files under tests/integration/ are not their
# own [[test]] targets except where Cargo.toml declares one explicitly (e.g. the
# "vision_integration" target maps to tests/integration/vision_integration_test.rs).

# 2. For a requirement naming a type/fn (e.g. REQ-vision-content-model → VisionContent),
#    find both the declaration and every test that constructs/exercises it:
grep -rn "struct VisionContent\|VisionContent::" crates/ src/ tests/ examples/

# 3. For a requirement naming a capability rather than a type (e.g.
#    REQ-council-termination-conditions), search test function names directly:
grep -rn "fn test_.*council\|fn.*council.*test" tests/ --include="*.rs"

# 4. Confirm a specific test actually passes (do this once per epic batch, not per row,
#    to control wall-clock — see Pitfall 5):
cargo test --offline -p paladin-battalion council:: 2>&1 | tail -5

# 5. For examples/ (47 files total — `ls examples/*.rs | wc -l` confirmed):
ls examples/ | grep -i <keyword>
```

### Verifying the epic-numbering defect's exact scope

```bash
# The three defect categories D-08 names, with confirmed line numbers:
grep -n "^### Epic 19\|^### Epic 20\|^### Epic 21\|^### Epic 22\|^### Epic 23" \
  .project/Milestone_3-Completion/RELEASE_NOTES_MILESTONE_3.md
# → lines 21, 48, 76, 111, 147 (Conclave/Council/Grove/Maneuver/Commander-Enhancement
#   headings, all mismatched against the authoritative numbering)

grep -n "PerformanceBased" .project/Milestone_3-Completion/RELEASE_NOTES_MILESTONE_3.md
# → line 106, inside the Epic 21(-mislabeled) "Routing Strategies" bullet list

grep -n "🔮 What's Next" .project/Milestone_3-Completion/RELEASE_NOTES_MILESTONE_3.md
# → line 320
```

## State of the Art

Not applicable in the usual "library upgrade" sense. The one relevant "old vs. current" fact:

| Old position | Current position | When Changed | Impact |
|--------------|------------------|--------------|--------|
| REQUIREMENTS.md's inline "Milestone 2-3 as-shipped ledger" is the authoritative record | `.planning/ledgers/milestone-02-03.md` is authoritative; REQUIREMENTS.md becomes a pointer | This phase, per D-00d/D-21 | Any future phase or agent reading REQUIREMENTS.md's ledger section for Milestone 2-3 detail must be redirected to the new file |
| 80% coverage target (D-09 era) | 84% floor, 80% explicitly superseded | ADR-0006, 2026-07-31 (before this phase) | VERIFY-05 extends this same number/scope; does not introduce a new one |

## Assumptions Log

| # | Claim | Section | Risk if Wrong |
|---|-------|---------|---------------|
| A1 | The D-20 suggested plan-grouping shape (ledger scaffold → epic fan-out 5-6 plans → 3 block plans → ADR pairs) is followed as written rather than re-derived from scratch | Plan decomposition (see below) | Low — D-20 explicitly marks this "suggested shape," not locked; the planner has full discretion here |
| A2 | Pairing Epic 13 + Epic 20 in one ledger fan-out plan (both vision, cross-milestone) is preferable to strict Milestone-2/Milestone-3 boundary grouping | Ledger fan-out sizing | Low — either grouping produces the same 118 rows; this is a sequencing convenience only, flagged as a discretion point below |

No `[ASSUMED]`-tagged factual claims requiring user confirmation appear in this document — every
code citation was re-verified against the tree during this research session (see Pitfalls 2-5, all
confirmed by direct `grep`/`sed`/`cargo metadata` inspection, not transcribed from CONTEXT.md
without checking).

## Open Questions (RESOLVED)

*Both questions were resolved by the Phase 5 plan set, written 2026-08-04. Resolutions recorded
inline below.*

1. **RESOLVED — Combined, as recommended.** Should the three VERIFY-02 block-verification plans run
   before or interleaved with the ledger epic fan-out?
   → Each block plan authors its own epic's ledger rows: `05-05` (Epic 22, 10 rows), `05-06`
   (Epic 14, 8 rows), `05-07` (Epic 24, 9 rows). Those three epics are excluded from the fan-out
   plans' scope. No epic is read twice.
   - What we know: D-20 lists them as step 3, after the epic fan-out (step 2). But Epic 22
     (81 open), Epic 14 (45 open) and Epic 24 (29 open) are themselves part of the epic fan-out's
     row count (they contain 10, 8, and 9 `REQ-*` rows respectively) — so their ledger rows and
     their block verdicts are naturally produced by the *same* research pass.
   - What's unclear: Whether splitting "ledger row for Epic 22's 10 REQs" and "Epic 22's block
     verdict" into two separate plans (fan-out plan vs. block plan) risks double work, or whether
     one plan should produce both outputs together.
   - Recommendation: Combine — let each of the three block plans also author its own epic's
     `REQ-*` ledger rows, and exclude those three epics (14, 22, 24 = 27 requirements) from the
     epic-fan-out plans' scope. This is consistent with D-20's own step ordering intent (blocks are
     "the deepest verification work" and get dedicated plans) without literally re-reading the same
     epic twice.

2. **RESOLVED — Recommendation adopted, groupings adjusted because the tracer consumed Epic 11.**
   Exact per-plan epic grouping for the fan-out step.
   → As shipped in the plan set: `05-01` (tracer) Epic 11 = 8 · `05-08` Epic 13+20 = 19 (the
   recommended cross-milestone pairing, kept) · `05-09` Epic 12+15 = 13 · `05-10` Epic 16+18 = 18 ·
   `05-11` Epic 17/17.5+19 = 16 · `05-12` Epic 21+23 = 17. Fan-out total 91; plus the three block
   plans' 27 = **118**. The pairings below were re-cut around Epic 11 moving into the tracer plan;
   the Epic 13+20 rationale survived unchanged.
   - What we know: Full per-epic counts (verified from REQUIREMENTS.md's ledger, sums to 118):
     Epic 11 (8), Epic 12 (8), Epic 13 (13), Epic 20 (6), Epic 14 (8, block), Epic 15 (5), Epic 16
     (11), Epic 17/17.5 (11), Epic 18 (7), Epic 19 (5), Epic 21 (7), Epic 22 (10, block), Epic 23
     (10), Epic 24 (9, block). Excluding the three block epics (27 reqs), 91 requirements remain
     for fan-out across roughly 5 plans at ~18/plan.
   - What's unclear: Whether to keep Epic 13 (Milestone 2) grouped with Epic 20 (Milestone 3) since
     they share the vision variant groups (8-12) and citing one without the other risks incomplete
     divergence analysis — D-20's own discretion note says milestone ordering has "no dependency
     either way."
   - Recommendation, with numbers: five fan-out plans — (a) Epic 11+12 = 16, (b) Epic 13+20 = 19
     (crosses the milestone boundary deliberately, for the shared vision variant-group context), (c)
     Epic 15+16 = 16, (d) Epic 17/17.5+18 = 18, (e) Epic 19+21+23 = 22. Total 91, matching the three
     block plans' 27 = 118.

## Environment Availability

| Dependency | Required By | Available | Version | Fallback |
|------------|------------|-----------|---------|----------|
| `cargo metadata` | Exercising-artefact search (test target enumeration) | ✓ | workspace toolchain, confirmed working | — |
| `cargo test` (targeted, per-crate/per-epic) | Confirming a named exerciser passes | ✓ | — | Run scoped (`-p <crate> <module>::`) rather than full workspace, to control wall-clock |
| `docker` | Not needed — this phase does no re-measurement (D-16) | n/a | n/a | n/a |
| `.claude/gsd-core/bin/lib/adr-parser.cjs` | ADR format validation (informal, no CI gate found wired to it) | ✓ (present in tree) | — | Not required to run it; existing ADRs already satisfy it by inspection |

**Missing dependencies with no fallback:** none.

**Missing dependencies with fallback:** none — everything this phase needs is already present.

## Validation Architecture

**Skipped.** This phase's config does not set `workflow.nyquist_validation: false` explicitly, but
the section's own machinery (test framework, phase requirements → test map, sampling rate) does not
apply: this phase produces zero testable code artefacts. Its "verification" is evidence-citation
correctness, which is what the Common Pitfalls and Code Examples sections above already cover. There
is no `cargo test` command that validates a markdown ledger's citations — the closest thing to a
Nyquist check here is: for every `satisfied` row, re-run the named command/test and confirm it still
passes, which is D-01's own bar restated as a verification step, not a separate test suite.

## Security Domain

**Applies narrowly, and only as a citation target, not new work.** `security_enforcement` is not
disabled in this project's config; VERIFY-04's encryption-at-rest disposition touches ASVS V6
(Cryptography) territory, but this phase **records** the disposition — it does not implement, wire,
or change any cryptographic code path.

| ASVS Category | Applies | Standard Control |
|---------------|---------|-----------------|
| V6 Cryptography | Yes, as a citation subject only | `EncryptionService` already uses ChaCha20-Poly1305 AEAD via the `chacha20poly1305` crate (not hand-rolled) — this phase's ADR-0011 records that fact and its zero-consumer status; it does not touch the crypto code |
| V2/V3/V4/V5 | No | Not implicated — this phase touches no auth, session, access-control, or input-validation code |

No new threat patterns are introduced. The one relevant finding for a future implementer (Phase 6
CLOSE-03, not this phase): `EncryptionService` is fully built and self-tested but has zero external
consumers, so no vision-path image bytes are currently encrypted at rest despite the capability
existing — recorded as a finding, wiring decision deferred per D-11.

## Sources

### Primary (HIGH confidence — direct tree inspection during this research session)
- `crates/paladin-core/src/platform/container/vision.rs:189-214` — `VisionError` enum incl.
  `EncryptionError` at :210-212, confirmed by direct read.
- `src/infrastructure/security/encryption.rs` — full file structure confirmed: `SecureData` (Zeroize
  derive), `DataRetentionPolicy` (`should_retain`, not `is_expired`), `EncryptionService`
  (`encrypt_image_data`/`decrypt_image_data`), confirmed by direct read and `grep -n` line lookups.
- `Cargo.toml:130-140` — `chacha20poly1305`/`zeroize` unconditional, `live-api-tests = []` at :265,
  confirmed by direct read.
- `tests/integration/mod.rs:34-35` and `tests/lib.rs:61` — the double-gate structure, confirmed by
  direct read and grep.
- `tests/integration/llm_live_api_tests.rs:61-88` — the doc-comment/panic mismatch, `#[ignore]` count
  = 13, confirmed by direct read and `grep -c`.
- `.project/Milestone_3-Completion/RELEASE_NOTES_MILESTONE_3.md` — full file read; exact line numbers
  for all epic headings, the `PerformanceBased` claim (:106), and the "What's Next" section (:320)
  confirmed.
- `.project/Milestone_3-Completion/Epic_22/tasks-epic22-battalion-commander-hardening.md`,
  `.project/Milestone_2-Missing_features/Epic_14/tasks-autonomous-agent-features.md`,
  `.project/Milestone_3-Completion/Epic_24/tasks-test-hardening-benchmarks-qa.md` — parent-task
  structure and per-parent open-item counts confirmed by direct script analysis (see per-parent
  breakdown table in Common Pitfalls / plan-decomposition notes above).
- `.planning/REQUIREMENTS.md` lines 2641-2890 — full "Milestone 2-3 as-shipped ledger" read in full;
  per-epic ID counts derived and cross-summed to 118.
- `.planning/ledgers/milestone-01.md` — read in full (698 lines); head notes, amendment sections,
  divergence table all inspected directly for shape.
- `.planning/decisions/0001-battalion-config.md`, `0006-coverage-gate.md`, `PROMOTION.md` — read in
  full for ADR heading shape and numbering-index convention.
- `.claude/gsd-core/bin/lib/adr-parser.cjs` — read in full; confirmed no frontmatter requirement,
  H2-heading synonym matching only.
- `.pre-commit-config.yaml` — read in full; confirmed `always_run: true` on `cargo-fmt`/`cargo-clippy`
  hooks (lines 73-89).
- `cargo metadata --no-deps --format-version=1` — executed directly; confirmed test-target inventory
  including the `lib` target that compiles `tests/integration/`.

### Secondary (MEDIUM confidence)
- `.planning/intel/task-completion-state.md` — 81/45/29 counts cross-referenced (lines 48, 56-57,
  114-116) and used as authoritative per D-05's own instruction not to re-derive; a naive re-grep
  during this research independently confirmed the +2 boilerplate-string discrepancy explaining why
  they differ from a literal `grep -c`.

### Tertiary (LOW confidence)
- None — every claim in this document was checked against the tree or an explicitly-authoritative
  `.planning/intel/` file during this session.

## Metadata

**Confidence breakdown:**
- Ledger/ADR document shape: HIGH — copied directly from working, shipped examples in this repo
- Scale/grouping numbers: HIGH — derived by direct enumeration of REQUIREMENTS.md's 118-row table,
  cross-summed to the corpus's own stated total
- The four CONTEXT.md code claims: HIGH — all four re-verified directly against the tree this
  session, one small citation-precision correction identified (Pitfall 2)
- Git-hook/commit-cadence risk: HIGH — confirmed directly by reading `.pre-commit-config.yaml`

**Research date:** 2026-08-04
**Valid until:** Until the next commit to `src/infrastructure/security/`, `tests/integration/`, or
`RELEASE_NOTES_MILESTONE_3.md` — none of these are expected to change before Phase 5 executes, since
Phase 5 is next in the milestone. If Phase 5 is deferred and other phases land first, re-verify the
four code citations before planning.
