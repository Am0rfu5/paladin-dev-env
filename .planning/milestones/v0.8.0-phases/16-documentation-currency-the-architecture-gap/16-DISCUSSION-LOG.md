# Phase 16: Documentation Currency & the Architecture Gap - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.
> Decisions are captured in CONTEXT.md — this log preserves the alternatives considered.

**Date:** 2026-08-24
**Phase:** 16-Documentation Currency & the Architecture Gap
**Areas discussed:** Architecture doc (archive or rebuild), the `# Examples` bar, depth and evidence for the fourteen, demos (record or withdraw)

---

## Pre-discussion: todo cross-reference

| Option | Description | Selected |
|--------|-------------|----------|
| Leave it pending | Targets `docs/src/contributing/testing-guide.md` — not one of DOCS-01's fourteen — and needs Docker + `cargo-llvm-cov`, neither installed. Its own text says it should outlive its phase. | ✓ |
| Fold it in | Treat the testing guide's Code Coverage section as a fifteenth currency target. | |

**User's choice:** Leave it pending
**Notes:** Recorded in CONTEXT.md under Reviewed Todos so future phases know it was considered.

---

## Architecture doc: archive or rebuild

### Q1 — disposition of `docs/src/appendix/design-and-architecture.md`

| Option | Description | Selected |
|--------|-------------|----------|
| Archive it, signpost to the live chapter | Record an ADR: historical, superseded by `docs/src/architecture/` (1,216 lines, 6 of 7 subsystems) plus `appendix/sentinel.md`. Banner + pointer; stop tracking FR-26.1 against this file; re-anchor the metric. | ✓ |
| Rebuild it in place as the live deliverable | Expand to ~600-800 lines with all 15+ components and four Mermaid diagrams, move out of the exempt appendix chapter. | |
| Merge: fold anything unique in, then archive | Migrate Security Considerations / Deployment Architecture / config.toml sections into the live chapter first, then archive the husk. | |

**User's choice:** Archive it, signpost to the live chapter
**Notes:** The option set was reshaped by a measurement taken during discussion — the seven "missing" subsystems are covered in the live `docs/src/architecture/` chapter and `appendix/sentinel.md`, so the 311-line file is a relocated pre-rewrite artifact rather than the project's architecture documentation.

### Q2 — what closes DOCS-02's substance

| Option | Description | Selected |
|--------|-------------|----------|
| Archive + close the Sentinel gap in the live chapter | Re-anchor FR-26.1's metric to the live chapter and document Sentinel there, giving 19 of 19. | ✓ |
| Archive only — the decision IS the deliverable | DOCS-02's "Done when" offers archive-and-stop-tracking as a complete branch. | |
| Archive + audit the whole live chapter for currency | Check all five architecture pages the way DOCS-01 checks its fourteen. | |

**User's choice:** Archive + close the Sentinel gap in the live chapter
**Notes:** Measured 18 of 19 shipped ubiquitous-language components covered; Sentinel is the only absentee. The metric restatement "8 of 15+ → 15+ of 15+" becomes "18 of 19 → 19 of 19".

### Q3 — FR-26.1's four-Mermaid-diagram clause

| Option | Description | Selected |
|--------|-------------|----------|
| Withdraw the clause with the reason recorded | Map the six existing SVGs to the four named diagrams, withdraw any genuinely unanswered one; don't author diagrams into a file being archived. | ✓ |
| Author the four into the live architecture chapter | Move the clause with the metric to the chapter readers are being sent to. | |
| Author only the ones the SVGs don't cover | Audit first, author the gaps. | |

**User's choice:** Withdraw the clause with the reason recorded
**Notes:** `mdbook-mermaid` is not installed locally, so rendering could only be proven in CI — a secondary factor, not the deciding one.

### Q4 — recording mechanism

| Option | Description | Selected |
|--------|-------------|----------|
| One ADR-0047 following the ADR-0022 pattern | All three sub-decisions in one ADR; restate the stale premise; write re-instatement as an instruction; advance PROMOTION.md to 0048. | ✓ |
| ADR-0047 plus a separate withdrawal ADR-0048 | Split archive from withdrawal for cleaner supersession targets. | |
| Ledger row only, no ADR | Dated amendment against `REQ-arch-doc-modernization`. | |

**User's choice:** One ADR-0047 following the ADR-0022 pattern
**Notes:** ADR-0022 was read during discussion and confirmed as a direct structural precedent for a requirement withdrawn with the reason recorded.

---

## What the `# Examples` bar actually means

### Q1 — binding definition of "entry point"

| Option | Description | Selected |
|--------|-------------|----------|
| The 79 named in FR-26.3's own wording | 11 builders + 35 port traits + 33 service structs — exactly what the text enumerates. 47 already comply. | ✓ |
| Every public item in the exports baseline | All 1,971 items in `.project/current-exports.txt`. | |
| The 79 plus every public constructor | Adds the 204 `pub fn new(` sites. | |

**User's choice:** The 79 named in FR-26.3's own wording
**Notes:** A prior measurement established that `missing_docs` is already clean workspace-wide, so FR-26.3's "enumerate undocumented pub items" half is effectively closed and only the examples half is open.

### Q2 — `# Example` vs `# Examples`

| Option | Description | Selected |
|--------|-------------|----------|
| Accept either; normalise only the 79 entry points | Rustdoc renders both identically; normalise where the requirement's wording must be grep-satisfiable, record the rule in the CONVENTIONS map. | ✓ |
| Normalise all 364 sites | One spelling everywhere via a sed sweep. | |
| Leave both, treat spelling as out of scope | Read "# Examples" as "an examples section". | |

**User's choice:** Accept either; normalise only the 79 entry points
**Notes:** Tree carries `# Example` 212 times and `# Examples` 152 times. The repo's own CONVENTIONS map uses the singular in its worked example, which is why the first coverage measurement under-counted and had to be redone.

### Q3 — `paladin-herald`'s `#![allow(missing_docs)]`

| Option | Description | Selected |
|--------|-------------|----------|
| Remove it and correct ADR-0033 in place | Flip to `warn` (measured: zero new warnings), amend ADR-0033 per D-00d, disposition `doc-examples` too. | ✓ |
| Remove it, no ADR amendment | Fix the code, record in SUMMARY and a ledger row. | |
| Leave the opt-out, amend ADR-0033 to match reality | Ratify the exemption. | |

**User's choice:** Remove it and correct ADR-0033 in place
**Notes:** Measured directly during discussion by flipping the attribute and rebuilding — zero additional warnings. The working tree was restored afterwards and verified clean.

### Q4 — does the gate mechanism change

| Option | Description | Selected |
|--------|-------------|----------|
| Clear the 20, leave the gate mechanism alone | Fix the tree, prove green with `ci.yml:63`'s exact command per D-00e; record that the "add the CI gate" clause was already satisfied. | ✓ |
| Clear the 20 and harden to `RUSTDOCFLAGS='-D warnings'` | Replace grep-on-`tee` with rustdoc's deny mode. | |
| Clear the 20 and also gate mdbook + linkcheck | Extend hardening to the book build. | |

**User's choice:** Clear the 20, leave the gate mechanism alone
**Notes:** The hardening idea is preserved as a deferred item — the current grep can match per-crate summary lines as well as real warnings, but changing it would need its own ADR since ADR-0033 ratified the gate in its present form.

---

## Depth and evidence for the fourteen

### Q1 — the deliverable that closes DOCS-01

| Option | Description | Selected |
|--------|-------------|----------|
| A per-file currency verdict record, plus the edits it finds | Fourteen rows: file, signals checked, producing command or `file:line`, verdict current-or-updated. One artifact, not 26 tasks. | ✓ |
| Edits only — the git diff is the evidence | Sweep and fix; a file with no diff is implicitly current. | |
| Verdict record with a re-runnable check script | Adds mechanical regression-guarding for the checkable signals. | |

**User's choice:** A per-file currency verdict record, plus the edits it finds
**Notes:** DOCS-01 forbids both converting the count into 26 tasks and dismissing it; a single record satisfies both constraints while meeting D-00e.

### Q2 — evidence surface for the linkcheck report

| Option | Description | Selected |
|--------|-------------|----------|
| Install the tooling locally and review the real report | `cargo install` mdbook + linkcheck + mermaid, run `mdbook build docs/`, record output verbatim. | ✓ |
| Accept CI's Build MDBook check as the evidence | Cite Phase 15.1's verified run at commit `d87d11e`. | |
| Install locally AND cite the CI run | Belt and braces. | |

**User's choice:** Install the tooling locally and review the real report — **"and make sure that they will be installed next time the devcontainer is rebuilt"**
**Notes:** The user added a requirement not present in any option: the install must survive a devcontainer rebuild. This produced Q3.

### Q3 — which images get the install

| Option | Description | Selected |
|--------|-------------|----------|
| Both Dockerfiles, pinned to CI's exact versions | mdbook 0.4.40, mdbook-mermaid 0.13.0, mdbook-linkcheck 0.7.7 with `--locked --version` in both `Dockerfile.dev` and `Dockerfile`. | ✓ |
| Only `Dockerfile.dev`, the image actually built | Smaller diff; leaves the other image a trap. | |
| Both, plus add `cargo-llvm-cov` while we're here | Would also unblock the pending coverage todo. | |

**User's choice:** Both Dockerfiles, pinned to CI's exact versions
**Notes:** Versions taken from `.github/workflows/docs.yml:44-54` so local and CI tooling cannot disagree. `cargo-llvm-cov` was declined here and preserved as a deferred item.

### Q4 — depth of the content pass

| Option | Description | Selected |
|--------|-------------|----------|
| Mechanical signals in full, prose read for contradictions | Check every checkable signal exhaustively; read prose for statements the 0.8.0 tree contradicts; no style rewriting. | ✓ |
| Full editorial rewrite of each page | Re-author for accuracy, structure and tone. | |
| Mechanical signals only | Fix greppable drift and stop. | |

**User's choice:** Mechanical signals in full, prose read for contradictions
**Notes:** 10,337 lines across the fourteen. Measurements taken during discussion: 11 `v0.4.3` strings against a shipped 0.8.0, a `0.5.0` dependency pin at `maneuver-flow-dsl.md:55`, all 7 cited source paths still resolving, and most `paladin-*` tokens being Kubernetes object names rather than crate names.

---

## Demos: record or withdraw

### Q1 — record or withdraw

| Option | Description | Selected |
|--------|-------------|----------|
| Record them — the blocker was false | Four mock-backed examples run offline at exit 0; provision a recorder, record to `docs/assets/recordings/`, add `docs/DEMOS.md`. | ✓ |
| Withdraw with the corrected reason recorded | Withdraw per DOCS-04's second branch, citing the README's changed shape rather than the false credential claim. | |
| Record two, defer two | Record the two shortest, defer the rest. | |

**User's choice:** Record them — the blocker was false
**Notes:** DOCS-04 states "recordings also require live LLM API keys, which puts them outside any offline gate." Measured false during discussion: all four scenarios use mock adapters, and `cargo run --example basic_paladin` completed at exit 0 with no credentials.

### Q2 — recording format (DOCS-04's Open Question 4)

| Option | Description | Selected |
|--------|-------------|----------|
| VHS — scripted, reproducible, emits `.gif` and `.cast` | Checked-in `.tape` sources make demos regenerable rather than hand-performed. | ✓ |
| asciinema — exactly what FR-26.4 names | `.cast` files, literal compliance, but each take is manual and unregenerable. | |
| Plain GIFs | Simplest to embed, drops the `.cast` artifact. | |

**User's choice:** VHS — scripted, reproducible, emits `.gif` and `.cast`
**Notes:** Open Question 4 (asciinema vs VHS vs Terminalizer vs plain GIFs) had been recorded as unanswered since the requirement was written. This settles it.

### Q3 — where the demos surface

| Option | Description | Selected |
|--------|-------------|----------|
| `docs/DEMOS.md` as the index, one README link to it | Honours the embedding clause without re-inflating the M11 Epic 5 landing page. | ✓ |
| `docs/DEMOS.md` plus a book chapter | Also surface inside the mdbook. | |
| Restore a demos section in the README | Literal reading of FR-26.4; reverses a deliberate M11 decision. | |

**User's choice:** `docs/DEMOS.md` as the index, one README link to it
**Notes:** Recorded as the clause being adapted rather than dropped — the README it targeted changed shape under Milestone 11 Epic 5.

---

## Claude's Discretion

Surfaced during discussion but not put to the user; the planner decides and records its reasoning:

- Whether the ~30 new `# Examples` blocks must compile as executable doctests or may be `no_run`/`ignore`.
- Disposition of `crates/doc-examples`, the eleventh crate, which carries neither `warn` nor `allow(missing_docs)`.
- Whether the fourteen files are swept in one plan or split by directory.
- Where the D-09 verdict record lives — phase artifact, ledger amendment, or both.
- Whether the `.tape` scripts get a CI regeneration check.
- Whether `docs/assets/recordings/` commits binary artifacts to git, and their size budget.

## Deferred Ideas

- `Armory` appears 0 times in the tree — ubiquitous-language drift in code, not docs.
- `cargo-llvm-cov` is missing from both devcontainer images — the reason the coverage-reproduction todo has never been walked.
- The `.planning/codebase/` maps are all dated 2026-07-30 and predate Phases 12-17.
- Auditing the five live architecture-chapter pages for currency (offered under DOCS-02 Q2, declined).
- Hardening the doc gate to `RUSTDOCFLAGS='-D warnings'` (offered under DOCS-03 Q4, declined).
- Whether the four withdrawn Mermaid diagrams are ever authored into the live chapter.
