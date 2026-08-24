# Phase 16: Documentation Currency & the Architecture Gap - Research

**Researched:** 2026-08-24
**Domain:** Rust workspace documentation (mdBook + rustdoc), CI doc-gates, devcontainer tooling provisioning, terminal-demo recording
**Confidence:** HIGH (all load-bearing claims below were reproduced against this tree this session — commands and outputs are quoted, not inferred)

<user_constraints>
## User Constraints (from CONTEXT.md)

### Locked Decisions

**DOCS-02 — the architecture document**
- **D-01:** Archive `docs/src/appendix/design-and-architecture.md` — recorded as historical,
  superseded by `docs/src/architecture/` plus `docs/src/appendix/sentinel.md`. Add a header banner
  saying so and pointing there; stop tracking FR-26.1 against this file. Re-anchor FR-26.1's success
  metric to the live chapter. Reversibility: costly.
- **D-02:** The substance closes by giving Sentinel a home in the live chapter. Metric restatement:
  FR-26.1's "8 of 15+ → 15+ of 15+" becomes **18 of 19 → 19 of 19** against the live chapter.
- **D-03:** FR-26.1's four-Mermaid-diagram clause is withdrawn, with the reason and a mapping to the
  six existing SVGs in `docs/src/assets/` plus the existing mermaid block in
  `docs/src/architecture/crate-map.md`, recorded. Do not author diagrams into a file being archived.
- **D-04:** One ADR-0047, following the ADR-0022 pattern (restate the stale premise; write
  re-instatement down as an instruction, not a mechanism). Update `PROMOTION.md`'s next-free line to
  0048.

**DOCS-03 — the `cargo doc` bar and the `# Examples` requirement**
- **D-05:** "Public API entry point" = the 79 items FR-26.3 names — 11 builders + 35 `*Port` traits +
  33 `*Service` structs. Enumerate them in the phase record. Baseline: 47 of 77 resolvable
  entry-point files already carry an example block; ~30 do not. Zero-example crates:
  `paladin-llm`, `paladin-storage`, `paladin-web`, `paladin-content`, `paladin-notifications`.
- **D-06:** Accept both `# Example`/`# Examples` headings tree-wide; normalise only the 79 enumerated
  entry points to `# Examples`. Record the rule in `.planning/codebase/CONVENTIONS.md`.
- **D-07:** Remove `paladin-herald`'s `#![allow(missing_docs)]` (measured zero new warnings), amend
  ADR-0033 in place. Also disposition `crates/doc-examples`, which carries neither attribute.
- **D-08:** Clear the 20 warnings; leave the gate mechanism untouched. Prove green by running
  `.github/workflows/ci.yml:63`'s exact command verbatim. Do NOT switch to `RUSTDOCFLAGS='-D
  warnings'`; do NOT add an mdbook/linkcheck gate (Phase 15.1 already pinned `Build MDBook` as
  required).

**DOCS-01 — the fourteen files**
- **D-09:** Deliverable is a per-file currency verdict record (one artifact, not 26 tasks): file,
  signals checked, exact command/`file:line` per finding, verdict `current` or `updated → commit`.
- **D-10:** Install the doc toolchain locally, review the real linkcheck output (verbatim), and make
  the install survive a devcontainer rebuild (user-added requirement).
- **D-11:** Both `.devcontainer/Dockerfile.dev` and `.devcontainer/Dockerfile` get
  `mdbook 0.4.40`, `mdbook-mermaid 0.13.0`, `mdbook-linkcheck 0.7.7`, `--locked --version`, matching
  `docs.yml:44-54`.
- **D-12:** Mechanical signals checked exhaustively across all 10,337 lines; prose read for
  contradictions; no style rewrite (Milestone 11 Epic 3 already rewrote this corpus once).

**DOCS-04 — the demos**
- **D-13:** Record them — the credential blocker is measured false (M-08). Record in the phase that
  DOCS-04's live-key premise was measured false, amending the requirement text in place.
- **D-14:** VHS, with checked-in `.tape` scripts. Emit `.gif` for embedding and `.cast` for FR-26.4's
  artifact shape, under `docs/assets/recordings/`. Provision the recorder in both devcontainer images
  alongside the mdbook tooling (D-11).
- **D-15:** `docs/DEMOS.md` is the index; README gets one link to it.
- **D-16:** Four scenarios fixed: Basic Paladin Execution → `examples/basic_paladin.rs`; Battalion
  Formation → `examples/formation_sequential.rs`; Council Discussion →
  `examples/council_discussion.rs`; Grove Routing → `examples/grove_routing.rs`.

### Claude's Discretion
- Executable doctests for the ~30 new examples — compile-and-run where possible, `no_run` where it
  needs live I/O; record the split.
- `crates/doc-examples` disposition under D-07.
- Plan splitting for the fourteen files (one sweep or several) — verdict record stays single per D-09.
- Where the D-09 verdict record lives (phase artifact, ledger amendment, or both).
- Whether `.tape` scripts get a CI regeneration check.
- Whether `docs/assets/recordings/` commits binaries to git, and their size budget.

### Deferred Ideas (OUT OF SCOPE)
- `Armory` naming drift (0 occurrences in tree) — code ubiquitous-language issue, not documentation.
- `cargo-llvm-cov` missing from devcontainer images — testing-area concern, declined for this phase.
- Refreshing all seven `.planning/codebase/` maps — `/gsd-map-codebase` work, not this phase (D-06
  touches only `CONVENTIONS.md`).
- Auditing the five live architecture-chapter pages for currency the way D-12 audits the fourteen —
  offered under D-02, declined; those five are in no requirement.
- Hardening the doc gate to `RUSTDOCFLAGS='-D warnings'` — offered under D-08, declined.
- Whether the four withdrawn Mermaid diagrams are ever authored into the live chapter — written down
  as a future-ADR instruction, not scheduled.
</user_constraints>

<phase_requirements>
## Phase Requirements

| ID | Description | Research Support |
|----|-------------|------------------|
| DOCS-01 | Fourteen user-guide/deployment/operations pages settled by content, with linkcheck reviewed | Reproduced doc-toolchain install path, exact verification commands, and a worked example of a genuine content-currency defect (`cicd.md`'s fabricated `ci.yml` block) below |
| DOCS-02 | `design-and-architecture.md` gets a recorded disposition (archive vs. live) | Verified 311-line count, 0 subsystem mentions, 0 mermaid blocks, and the live chapter's 18/19 coverage; ADR-0022 pattern confirmed as the template for ADR-0047 |
| DOCS-03 | One `cargo doc` bar, applied; public API documented to it | Re-derived and verified all 20 warnings by class and `file:line` this session (fresh `cargo doc` run); confirmed no stable-Rust tooling exists to assert "has an example" mechanically |
| DOCS-04 | Demos get a decision; `docs/assets/` stops implying work in flight | Verified `docs/assets/` absence; verified VHS's native `Output` formats via its own README — **does not include `.cast`**, a load-bearing gap in D-14 as written (see Landmines) |
</phase_requirements>

## Summary

This is a documentation-currency and doc-tooling phase, not a feature phase — there is very little
"standard stack" to select; the stack is already fixed (mdBook + rustdoc + the CI gates that already
exist). The research value here is entirely mechanical: reproducing the exact commands an executor
will run, re-deriving stale citations rather than trusting prior ADR text, and surfacing one genuine
technical gap in a locked decision (D-14) that the planner needs to resolve before writing tasks.

Four findings matter most for planning:

1. **All 20 `cargo doc` warnings were re-reproduced this session** with fresh `file:line` citations
   (not copied from ADR-0033, which has drifted by one line on `agent_auth.rs` and describes the
   three facade private-link warnings by their *target* item where the current warning text names
   the *enclosing* item — cosmetic, but worth re-deriving per the phase's own D-00e evidence bar).
2. **There is no stable-Rust mechanism to assert "every public entry point has an `# Examples`
   block."** `rustdoc::missing_doc_code_examples` exists only behind `#![feature(...)]` on nightly;
   this project pins `dtolnay/rust-toolchain@stable`. The honest fallback is a small grep/awk script
   against the 79-item D-05 enumeration, run in CI or as a phase-gate check — not a rustdoc lint.
3. **VHS cannot natively emit `.cast`.** Its `Output` command supports `.gif`, `.mp4`, `.webm`, and a
   PNG-frame directory only (confirmed from the tool's own README, `Output` section). D-14 asks for
   both `.gif` and `.cast` from VHS. The planner must resolve this — most likely by pairing VHS
   (`.tape` → `.gif`, the primary regenerable artifact) with a scripted, non-interactive
   `asciinema rec -c "<command>"` for the `.cast` (still regenerable — it is the same commands
   executed unattended, not a hand-performed take — so it does not undermine D-14's rationale).
4. **`docs/src/deployment/cicd.md` contains verifiably fabricated CI content**, not merely stale
   version strings: its quoted `ci.yml` code block has `on: push: branches: [main, develop]` and a
   job named `check`/`Check`; the real `ci.yml` triggers on `push: branches: ['**']`, has no
   `develop` branch (retired in plan 15.1-09), and its job is `lint`/`Code Quality`. The file's own
   workflow-structure diagram lists `integration-tests.yml`, which was deleted and absorbed into
   `ci.yml` by commit `2cf9919` (`feat(15.1-05): absorb integration-tests.yml into ci.yml and delete
   it`). This is offered as a concrete worked example of what "checked against the current tree"
   must catch — a defect no version-string grep would find.

**Primary recommendation:** Treat DOCS-01 as a per-file diff-against-source-of-truth exercise (one
verdict row per file, D-09), treat DOCS-03 as "clear 20 known warnings + normalise 79 known
headings + flip one crate's lint attribute," treat DOCS-02 as "write ADR-0047 in the ADR-0022 shape,
touch three files (banner, architecture chapter, `SUMMARY.md`)," and resolve the VHS/`.cast` gap in
DOCS-04 explicitly before task-writing rather than discovering it mid-execution.

## Architectural Responsibility Map

| Capability | Primary Tier | Secondary Tier | Rationale |
|------------|-------------|----------------|-----------|
| Doc-content currency verdicts (DOCS-01) | Documentation source (`docs/src/**/*.md`) | CI (`docs.yml` linkcheck) | Content lives in the mdBook source tree; the linkcheck gate is the automated cross-check, not the source of truth |
| Architecture-doc disposition (DOCS-02) | Documentation source + ADR record | mdBook TOC (`SUMMARY.md`) | The decision is recorded in an ADR and enacted as a banner + TOC edit; no code changes |
| `cargo doc` warning bar + `# Examples` (DOCS-03) | Rust source (`.rs` doc comments) | CI (`ci.yml:63` gate) | The fix is entirely `.rs` doc-comment edits; CI already gates it and is explicitly not to be modified (D-08) |
| Demo recordings (DOCS-04) | Devcontainer/CI tooling (VHS, ffmpeg, ttyd) | Documentation source (`docs/DEMOS.md`, `README.md`) | Recording is a build-tool concern; indexing/linking is a doc-source concern |
| Doc-toolchain provisioning (D-10/D-11) | Devcontainer images (`Dockerfile`, `Dockerfile.dev`) | CI (`docs.yml:44-54`, the version source of truth) | CI already pins exact versions; devcontainer provisioning must mirror, not diverge from, that pin |

## Standard Stack

### Core (already fixed by prior phases — not up for reselection)
| Tool | Pinned version | Purpose | Why fixed |
|------|---------|---------|--------------|
| `mdbook` | 0.4.40 | Renders `docs/src/` to the published book | Pinned at `.github/workflows/docs.yml:44` `--locked`; D-11 mirrors it |
| `mdbook-mermaid` | 0.13.0 | Renders the one existing mermaid block (`crate-map.md`) and any future ones | Pinned at `docs.yml:49` `--locked` |
| `mdbook-linkcheck` | 0.7.7 | The linkcheck DOCS-01/D-10 requires be "reviewed", not just passed | Pinned at `docs.yml:54` `--locked`; `docs/book.toml` sets `warning-policy = "error"`, `follow-web-links = false` |
| rustdoc (via `cargo doc`) | stable toolchain (`dtolnay/rust-toolchain@stable`) | The DOCS-03 warning bar | `ci.yml:63` is the exact gate command; do not change |

### New for this phase
| Tool | Version | Purpose | Provenance |
|------|---------|---------|--------------|
| `vhs` (charmbracelet) | latest stable release at plan time | Drives the four `.tape`-scripted terminal recordings (D-14) | `[CITED: github.com/charmbracelet/vhs/blob/main/README.md]` — not on crates.io/npm/PyPI, distributed via Homebrew, an APT/YUM repo (`repo.charm.sh`), or `go install` |
| `ttyd` | latest GitHub release | VHS's terminal-server dependency — VHS's own README states it must be on `$PATH` before VHS will run | `[CITED: same README]`; no APT package in the VHS repo instructions — binary release download from `github.com/tsl0922/ttyd/releases` |
| `ffmpeg` | Debian `bookworm` repo version | VHS's GIF/video encoder dependency | Standard Debian package; devcontainer base is `rust:1.97.1-slim-bookworm` (`.devcontainer/Dockerfile.dev:8`), so `apt-get install ffmpeg` is the natural path, no separate pin needed |
| `asciinema` | Debian/PyPI package | **Only if** the planner accepts the VHS+asciinema pairing to satisfy D-14's `.cast` clause (see Landmines) | `[ASSUMED]` — not verified this session; standard `apt install asciinema` or `pipx install asciinema` |

### Alternatives Considered
| Instead of | Could Use | Tradeoff |
|------------|-----------|----------|
| VHS for `.gif` | `asciinema` + `agg` (asciinema-gif-generator) for both `.cast` and `.gif` from one source | Rejected by the user's own D-14 (locked): VHS's `.tape` is a more explicit, diffable script than a live-recorded `.cast`; the tradeoff is exactly the `.cast`-output gap this research flags |
| Devcontainer `apt` install of `mdbook*` | `cargo install --locked --version` (chosen, D-11) | `apt` has no `mdbook-linkcheck`/`mdbook-mermaid` packages in Debian; `cargo install` is also what CI already does, so local and CI stay on identical code paths |

**Installation (mirrors `docs.yml:44-54`, verified against this tree's CI file this session):**
```bash
cargo install mdbook --version 0.4.40 --locked
cargo install mdbook-mermaid --version 0.13.0 --locked
cargo install mdbook-linkcheck --version 0.7.7 --locked
mdbook-mermaid install docs/
```

**Version verification (reproduced this session):**
```bash
$ grep -n "cargo install mdbook" .github/workflows/docs.yml
44:          cargo install mdbook --version 0.4.40 --locked
48:          cargo install mdbook-mermaid --version 0.13.0 --locked
52:          cargo install mdbook-linkcheck --version 0.7.7 --locked
```
None of `mdbook`, `mdbook-linkcheck`, `mdbook-mermaid`, `asciinema`, `vhs`, `ttyd`, `ffmpeg`, or `go`
is present in this research sandbox (`which` returns nothing for all seven). This confirms M-10
first-hand. **This research session could not fully verify an end-to-end local install**:
`crates.io` returned `HTTP 403` and `repo.charm.sh` returned `HTTP 429` from this sandbox's egress
(`static.rust-lang.org` and `github.com` both returned `200`). The executor must re-check network
egress from inside the actual devcontainer/CI runner — do not assume this sandbox's 403/429 carries
over; it is very likely a research-session-specific egress restriction, not a permanent block.

## Package Legitimacy Audit

| Package | Registry | Age | Downloads | Source Repo | Verdict | Disposition |
|---------|----------|-----|-----------|-------------|---------|-------------|
| `mdbook` | crates.io | ~11 yrs (first publish 2015-08-06) | 63,270/wk | `github.com/rust-lang/mdBook` | OK | Approved — already CI-pinned, D-11 mirrors it |
| `mdbook-linkcheck` | crates.io | ~8.5 yrs (2018-01-25) | 1,811/wk | `github.com/Michael-F-Bryan/mdbook-linkcheck` | OK | Approved — already CI-pinned |
| `mdbook-mermaid` | crates.io | ~7 yrs (2019-07-10) | 5,741/wk | `github.com/badboy/mdbook-mermaid` | OK | Approved — already CI-pinned |
| `vhs` | N/A — Go binary via Homebrew/APT/`go install`, not a crates.io/npm/PyPI package | public since 2022 | N/A (no registry download counter) | `github.com/charmbracelet/vhs` | *outside package-legitimacy seam's ecosystem coverage* | **Flagged for `checkpoint:human-verify`** — new tool addition to the supply chain (APT third-party repo with a `curl \| gpg --dearmor` key-install step), from a well-known publisher (Charm — also makers of Glow, Gum, Bubble Tea) but not verifiable through the automated npm/PyPI/crates gate |
| `ttyd` | N/A — GitHub release binary, no package registry at all | — | — | `github.com/tsl0922/ttyd` | *outside seam coverage* | **Flagged for `checkpoint:human-verify`** — binary download from GitHub releases is the only documented install path; verify checksum/signature if the release page provides one |
| `ffmpeg` | Debian `bookworm` APT repo (base image already Debian) | decades | N/A | `ffmpeg.org` | *outside seam coverage, but standard distro package* | Approved — ordinary `apt-get install ffmpeg`, same trust boundary as the rest of the devcontainer's existing `apt-get install` list |

**Packages removed due to [SLOP] verdict:** none.
**Packages flagged as suspicious [SUS]:** none via the automated gate (`vhs`/`ttyd` are flagged only
because they fall outside the gate's ecosystem coverage, not because a check returned SUS).

*`vhs` and `ttyd` were discovered via WebSearch/official-README fetch, not the automated
package-legitimacy seam (no npm/PyPI/crates registry exists for either). Tag both `[ASSUMED
install path — CITED tool identity]` and gate their devcontainer install behind
`checkpoint:human-verify`, consistent with the protocol's instruction for packages the gate cannot
evaluate.*

## Architecture Patterns

### System Architecture Diagram

```
                     ┌─────────────────────────────┐
                     │   docs/src/**/*.md source    │
                     │  (14 DOCS-01 files, appendix,│
                     │   architecture chapter)      │
                     └───────────┬──────────────────┘
                                 │  mdbook build
                                 ▼
                     ┌─────────────────────────────┐
                     │  mdbook + mdbook-mermaid +   │
                     │  mdbook-linkcheck            │──► docs/book (HTML) + linkcheck report
                     │  (D-10/D-11: local + CI,     │        (D-10: report REVIEWED, not
                     │   identical pinned versions) │         merely "passed")
                     └───────────┬──────────────────┘
                                 │  required check
                                 ▼
                     ┌─────────────────────────────┐
                     │ .github/workflows/docs.yml   │
                     │ "Build MDBook" (required)    │
                     └─────────────────────────────┘

  ┌───────────────────────────┐        ┌───────────────────────────┐
  │  .rs doc comments          │        │  examples/*.rs (4 named   │
  │  (79 D-05 entry points +   │        │  in D-16, mock-backed)    │
  │   the 20 known warnings)   │        │        │                  │
  └───────────┬────────────────┘        │        ▼                  │
              │ cargo doc                │  .tape script → VHS      │
              ▼                          │  → ttyd + ffmpeg          │
  ┌───────────────────────────┐         │  → .gif (+ .cast via      │
  │ ci.yml:63 "Check           │         │     paired asciinema,     │
  │ documentation" (required,  │         │     see Landmines)        │
  │ zero-warning grep gate —   │         └───────────┬───────────────┘
  │ DO NOT MODIFY, D-08)       │                     ▼
  └───────────────────────────┘         ┌───────────────────────────┐
                                         │ docs/assets/recordings/    │
                                         │ docs/DEMOS.md (index)      │
                                         │ README.md (one link, D-15) │
                                         └───────────────────────────┘
```

### Recommended Task Grouping (not a project-structure recommendation — this phase adds no `src/`)
```
docs/src/user-guides/      # DOCS-01 task 6.0, 6 files, 4,782 lines
docs/src/deployment/       # DOCS-01 task 7.0 (part), 4 files, 2,933 lines
docs/src/operations/       # DOCS-01 task 7.0 (part), 4 files, 2,092 lines
docs/src/appendix/design-and-architecture.md   # DOCS-02, banner + archive
docs/src/architecture/     # DOCS-02, Sentinel cross-link/section added
crates/*/src/lib.rs        # DOCS-03, herald allow(missing_docs) removed
crates/paladin-web/src/*.rs, src/infrastructure/**/*.rs,
crates/paladin-battalion/src/in_memory_registry.rs,
crates/paladin-herald/src/lib.rs                # DOCS-03, the 20 warnings
docs/assets/recordings/*.tape, *.gif, *.cast    # DOCS-04
docs/DEMOS.md, README.md                        # DOCS-04
.devcontainer/Dockerfile{,.dev}                 # D-11/D-14 tooling pins
.planning/decisions/0047-*.md                   # DOCS-02 ADR
.planning/decisions/0033-cargo-doc-warning-bar.md   # amended in place (D-07)
.planning/codebase/CONVENTIONS.md               # D-06 rule recorded
```

### Pattern 1: Per-file currency verdict, evidence-first (D-09)
**What:** One row per DOCS-01 file recording exactly which signal classes were checked, the command
or `file:line` that produced each finding, and a verdict.
**When to use:** Any "update in place" requirement where file-existence is not evidence (this phase's
explicit framing).
**Example (worked, from this session's own verification — usable as the row template):**
```markdown
| File | Signals checked | Findings (command / file:line) | Verdict |
|---|---|---|---|
| docs/src/deployment/cicd.md | version strings, workflow names, job names, `develop` branch refs | `grep -n "branches: \[ main, develop \]" docs/src/deployment/cicd.md` → line 47 (fabricated, vs. actual `ci.yml:14 branches: ['**']`); `grep -n "check:" docs/src/deployment/cicd.md` → job named `check`/`Check` (actual job is `lint`/`Code Quality`, `ci.yml:41`); workflow-structure diagram lists `integration-tests.yml`, deleted by commit `2cf9919` | updated → commit |
```

### Pattern 2: Warning-class-to-fix mapping for the 20 `cargo doc` warnings (DOCS-03)
**What:** Four distinct rustdoc lint classes appear in the 20-warning residue; each has a mechanical,
different fix. Re-derived this session by running `cargo doc --workspace --no-deps` fresh (do not
trust ADR-0033's file:line list verbatim — see Common Pitfalls).
**Example:**
```rust
// Class 1: unresolved intra-doc link (14 of 20) — target lacks an import path or isn't in scope.
// Fix: either import the type so the link resolves, or de-link it (escape the brackets) if it
// names a concept, not an item — e.g. crates/paladin-battalion/src/in_memory_registry.rs:9
// links [`paladin-core`] and [`paladin-ports`] as if they were items; they are crate names.
//! Because this type depends only on `paladin-core` and `paladin-ports` it carries   // fixed
//! Because this type depends only on [`paladin-core`] and [`paladin-ports`] it carries // warns

// Class 2: private-item link from public docs (3 of 20) — public doc references a private fn.
// Fix: either make the target `pub(crate)`-visible-enough (not always desirable) or drop the
// link and name it as plain text — e.g. src/infrastructure/web/agent_host.rs:216
/// checked when the provider is actually created in `build_agent`.               // fixed
/// checked when the provider is actually created in [`build_agent`].             // warns

// Class 3: redundant explicit link target (2 of 20) — label already resolves without the path.
// Fix: drop the explicit target — e.g. crates/paladin-web/src/app.rs:69
/// [`agent_router`], merged in. It is the                                        // fixed
/// [`agent_router`](crate::agent_controller::agent_router), merged in. It is the // warns

// Class 4: unclosed HTML tag (1 of 20) — a bare generic type read as an HTML tag.
// Fix: wrap in a code span — e.g. crates/paladin-battalion/src/in_memory_registry.rs:65
/// Internal storage: Paladin ID -> `Arc<Paladin>`                                // fixed
/// Internal storage: Paladin ID -> Arc<Paladin>                                  // warns
```

### Pattern 3: Mechanical "has an example" check (no rustdoc lint exists on stable — DOCS-03's honest fallback)
**What:** `rustdoc::missing_doc_code_examples` is nightly-only, feature-gated
(`#![feature(rustdoc_missing_doc_code_examples)]`) — confirmed via web search this session, and this
project pins `dtolnay/rust-toolchain@stable` in every workflow. There is no stable-Rust mechanism
that fails a build when a public item lacks an example.
**When to use:** As the phase-gate check for the D-05 79-item enumeration, in place of a rustdoc lint
that does not exist for this toolchain.
**Example (fallback pattern, not a specific script to lift verbatim):**
```bash
# For each of the 79 enumerated entry points (builders, *Port traits, *Service structs), assert the
# preceding doc block contains a `# Examples` heading before the item's own line:
awk '/^\/\/\/ # Examples$/{found=1} /^pub (struct|trait|fn) /{ if (!found) print FILENAME":"FNR" MISSING"; found=0 }' \
  crates/*/src/**/*.rs src/**/*.rs
```
Record this as a phase-authored script (e.g. `scripts/check-public-api-examples.sh`), not a rustdoc
feature — and record explicitly in the plan that no stable-Rust built-in exists, so a future reader
does not go looking for one.

### Anti-Patterns to Avoid
- **Trusting ADR-0033's file:line citations verbatim for the 20 warnings:** they have drifted by one
  line (`agent_auth.rs:7` in the ADR vs. `:8` in the tree today) and by naming convention (ADR cites
  the link *target*; current rustdoc output names the *enclosing* item). Re-run `cargo doc
  --workspace --no-deps` and re-derive, per CONTEXT.md's own instruction and D-00e.
- **A naive `paladin-*` crate-name sweep across the fourteen files (M-06's explicit warning):** most
  `paladin-*` tokens in `kubernetes.md` are Kubernetes object/Secret/ConfigMap names
  (`paladin-data`, `paladin-secrets`, `paladin-logs`), not crate names. Confirmed this session: only
  `docs/src/user-guides/maneuver-flow-dsl.md:55` (`paladin-battalion = { version = "0.5.0" }`) and
  the 11 `v0.4.3` occurrences are genuine version-currency defects among the version/crate-name class
  of signal.
- **Rewriting prose for tone/structure in the fourteen files (D-12):** Milestone 11 Epic 3 already
  rewrote this corpus once; DOCS-01 asks for "update in-place." A rewrite makes a currency fix
  indistinguishable from a style change in review.
- **Authoring new Mermaid diagrams into `design-and-architecture.md` before archiving it (D-03):**
  the file is being archived in the same phase; diagram authorship belongs in the live chapter if it
  ever happens, per the ADR-0047 "instruction, not mechanism" pattern.

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|--------------|-----|
| Terminal-session recording | A custom `script`/`ttyrec` wrapper | VHS (`.tape` scripts) — already locked by D-14 | Deterministic replay, checked-in source, no manual "take" to go stale |
| Doc-example compile verification | A hand-rolled markdown-code-block extractor | The existing `scripts/check-doc-examples.sh` + `crates/doc-examples` compile-checked crate (already in the tree, unrelated to but adjacent to DOCS-03's rustdoc `# Examples` work) | Already does exactly this for the mdBook-embedded examples; don't build a second mechanism for a related-but-distinct concern (mdBook examples vs. rustdoc `# Examples` on the 79 entry points are two different systems and should stay that way) |
| Broken-link detection | A custom link-crawler for `docs/src/` | `mdbook-linkcheck` (already pinned, already gates CI) | `docs/book.toml` already configures it `warning-policy = "error"`, offline (`follow-web-links = false`) — exactly the deterministic, reviewable report D-10 asks for |

**Key insight:** every mechanical need this phase has (link checking, example compilation, doc-build
gating) is already served by an existing, pinned tool in this tree. The only genuinely missing
mechanism is "assert every one of 79 enumerated entry points has an `# Examples` heading" — which
has no off-the-shelf tool on stable Rust and needs a small phase-authored script (Pattern 3 above).

## Common Pitfalls

### Pitfall 1: Citing ADR-0033's warning list instead of re-running `cargo doc`
**What goes wrong:** A task written from ADR-0033's file:line table alone will cite
`agent_auth.rs:7`; the tree today has the link at line 8. A fix applied to the wrong line either
silently fails (edits the wrong doc comment) or worse, edits code adjacent to the real target.
**Why it happens:** The file has had unrelated one-line edits since ADR-0033 was written
(2026-08-08); rustdoc's own warning output for links inside multi-line `//!` blocks carries no
`file:line` span at all for 11 of the 20 (a documented rustdoc behaviour), so even the ADR's own
citations for those eleven were *re-derived by grep*, not emitted by the compiler — a second
opportunity for drift.
**How to avoid:** Run `cargo doc --workspace --no-deps 2>&1 | tee <path>` fresh at plan- and
execute-time; grep the bracketed identifiers from the warning text against the source tree rather
than trusting any prior document's line numbers.
**Warning signs:** A warning's rustdoc output has no `-->` file span — this happens for every link
inside a `//!` module-doc comment (all 11 `paladin-web` `broken_intra_doc_links` this session showed
this pattern); the fix location must be found by grepping the linked identifier's exact bracketed
text.

### Pitfall 2: Treating `docs/assets/` (empty/absent) and `docs/src/assets/` (six SVGs) as the same path
**What goes wrong:** DOCS-04 targets `docs/assets/recordings/`; DOCS-02/D-03 targets the *unrelated*
`docs/src/assets/` for its SVG-to-diagram-clause mapping. A task that greps "assets" without the
`src/` distinction will conflate the two.
**Why it happens:** Both directories exist, one level apart, with overlapping "assets" naming; only
one currently has content (`docs/src/assets/`, six SVGs — `ArchitectureOverview`, `LayerArchitecture`,
`ComponentInteractionFlow`, `ContentProcessingPipeline`, `DeploymentArchitecture`, `data-flow`).
`docs/assets/` was confirmed absent entirely this session (`ls docs/assets` → no such directory).
**How to avoid:** Always qualify with the full path in task text; never write "assets/" unqualified.
**Warning signs:** A task description that says "the assets directory" without a full path.

### Pitfall 3: `--locked` cargo installs of mdbook tooling may be slow/impossible in a constrained sandbox
**What goes wrong:** This research session's own attempt to reach `crates.io` returned `HTTP 403`
and `repo.charm.sh` returned `HTTP 429`. If the execution environment has similar restrictions,
`cargo install mdbook --locked` (a from-source build, historically several minutes for mdbook alone)
will fail or hang.
**Why it happens:** Sandboxed dev/CI environments frequently restrict or rate-limit outbound network
access differently from a full devcontainer or GitHub-hosted runner.
**How to avoid:** The executor must verify egress from the *actual* devcontainer/CI runner before
committing to D-10's "install locally and review the real report" — if genuinely blocked, the
documented fallback is: cite CI's `docs.yml` linkcheck job output instead, with an explicit note that
D-10's "review the real report, not just CI's pass/fail signal" requirement could not be met and why.
**Warning signs:** `cargo install` hangs past a minute or two with no registry-index progress output,
or immediately errors with a connection/TLS failure.

### Pitfall 4: Assuming VHS produces `.cast`
**What goes wrong:** D-14 says "Emit `.gif` for embedding and `.cast` for FR-26.4's artifact shape."
A task that assumes `vhs demo.tape` alone produces both will fail at execution — VHS's `Output`
command only recognizes `.gif`, `.mp4`, `.webm`, and a PNG-frame directory (confirmed from the tool's
own README `### Output` section, fetched and grepped this session — no `.cast` case exists).
**Why it happens:** `.cast` is asciinema's native format; VHS is a different, unrelated tool that
happens to solve an adjacent problem (deterministic terminal recording via a script), and D-14's own
rationale ("regenerable... from `.tape` sources") does not by itself imply VHS emits every needed
format.
**How to avoid:** Plan for a two-tool pairing: VHS renders the primary `.gif` from each `.tape`; a
scripted, non-interactive `asciinema rec -c "<the same command the .tape drives>" out.cast` produces
the `.cast` from the identical underlying command — still scripted and regenerable, not a
hand-performed take, so it does not violate D-14's stated reasoning. Record this pairing decision
explicitly rather than silently dropping the `.cast` requirement.
**Warning signs:** A `.tape` script with an `Output demo.cast` line — VHS will not error usefully on
an unrecognized extension in every version; verify the actual produced file before trusting it.

### Pitfall 5: Mistaking "file exists and mtime is recent" for "content is current" (DOCS-01's core warning)
**What goes wrong:** All 14 files exist and 12 were touched 2026-06-02/03/06; a naive check ("was it
edited recently?") would mark them current. They still carry 11 occurrences of `v0.4.3` against a
shipped `0.8.0`.
**Why it happens:** The Milestone 11 in-place update targeted v0.4.3-era content and was never
re-run against the 0.8.0 tree.
**How to avoid:** Check each signal class explicitly (version strings, dependency pins, crate names,
module paths, `make` targets, workflow/job names, error types, feature flags) with its own command,
per D-09 — mtime is not a signal class.
**Warning signs:** A verdict row with no accompanying command or `file:line`.

## Code Examples

### Reproducing the exact CI doc gate (D-08's proof requirement)
```bash
# Source: .github/workflows/ci.yml:63, quoted verbatim, reproduced this session
cargo doc --workspace --no-deps 2>&1 | tee /tmp/doc-output.txt && ! grep -q "warning:" /tmp/doc-output.txt
```
This session's run against the unmodified tree: **exit 1**, 20 warnings across `paladin-web` (13),
`paladin-ai`/facade (3), `paladin-battalion` (3), `paladin-herald` (1) — identical split to
ADR-0033's 2026-08-08 measurement; the residue has not moved.

### The 20 warnings, re-derived this session with exact `file:line`
```
paladin-web (13):
  crates/paladin-web/src/agent_auth.rs:8            unresolved link `AuthPort`
  crates/paladin-web/src/agent_registry.rs:5         unresolved link `Paladin`
  crates/paladin-web/src/agent_registry.rs:5         unresolved link `PaladinExecutorPort`
  crates/paladin-web/src/agent_registry.rs:10        unresolved link `PaladinExecutorPort`
  crates/paladin-web/src/agent_registry.rs:10        unresolved link `Paladin`
  crates/paladin-web/src/delivery_controller.rs:8    unresolved link `deliver_content`
  crates/paladin-web/src/delivery_controller.rs:9    unresolved link `get_delivery_status`
  crates/paladin-web/src/delivery_controller.rs:10   unresolved link `get_delivery_stats`
  crates/paladin-web/src/delivery_controller.rs:12   unresolved link `create_delivery_routes`
  crates/paladin-web/src/openapi.rs:5                unresolved link `build_openapi`
  crates/paladin-web/src/openapi.rs:6                unresolved link `docs_router`
  crates/paladin-web/src/agent_controller.rs:651:45  redundant explicit link `JobRecord`
  crates/paladin-web/src/app.rs:69:22                redundant explicit link `agent_router`

paladin-battalion (3):
  crates/paladin-battalion/src/in_memory_registry.rs:9   unresolved link `paladin-core`
  crates/paladin-battalion/src/in_memory_registry.rs:9   unresolved link `paladin-ports`
  crates/paladin-battalion/src/in_memory_registry.rs:65:44  unclosed HTML tag `Paladin` (Arc<Paladin>)

paladin-herald (1):
  crates/paladin-herald/src/lib.rs:14:9              unresolved link `TableHerald`

paladin-ai / facade (3):
  src/infrastructure/adapters/arsenal/mcp_streamable_http_adapter.rs:18:16
      private-item link (enclosing item `mcp_streamable_http_adapter`, target `BearerToken::expose_secret`)
  src/infrastructure/web/agent_host.rs:216:56
      private-item link (enclosing item `validate_config`, target `build_agent`)
  src/infrastructure/web/agent_host.rs:268:7
      private-item link (enclosing item `build_agent_registry`, target `build_agent`)
```
*Source: `cargo doc --workspace --no-deps`, run in full this session, `grep -n " --> "` +
`grep -rn "<linked identifier>"` used to recover file:line for the 11 warnings rustdoc emits with no
span (links inside multi-line `//!` blocks) — see Pitfall 1.*

### The version-drift signal, exactly as it appears in the tree
```
$ grep -rn "v0\.4\.3" docs/src/deployment/{docker,kubernetes}.md docs/src/operations/performance-tuning.md | wc -l
11
$ grep -n 'paladin-battalion = { version' docs/src/user-guides/maneuver-flow-dsl.md
55:paladin-battalion = { version = "0.5.0", path = "crates/paladin-battalion" }
$ grep -n '^version' Cargo.toml
34:version = "0.8.0"
```
All 11 crate manifests ship at `0.8.0` in lockstep (`grep -n '^version' crates/*/Cargo.toml` — all
eleven, including `doc-examples`, report `0.8.0`).

### The `integration-tests.yml` deletion (evidence for the `cicd.md` fabrication finding)
```
$ git log --oneline --all -- .github/workflows/integration-tests.yml | head -1
2cf9919 feat(15.1-05): absorb integration-tests.yml into ci.yml and delete it
$ ls .github/workflows/
benchmarks.yml  ci.yml  docs.yml  feature-flags.yml  pre-commit.yml  release.yml
```
`docs/src/deployment/cicd.md`'s "Workflow Structure" diagram still lists `integration-tests.yml` as
a live file, and its quoted `ci.yml` code sample shows `on: push: branches: [main, develop]` and a
job named `check` — neither matches the tree (`ci.yml:14` is `branches: [ '**' ]`; the job is `lint`
/ `Code Quality`, `ci.yml:41`; `develop` was retired per plan 15.1-09, confirmed in `STATE.md`).

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| M8 Epic 5 FR-19 "warnings acceptable; must not fail" on `cargo doc` | Zero-warning bar, CI-enforced | ADR-0033, 2026-08-08 | DOCS-03 applies an already-ratified bar; does not re-litigate it (D-00t) |
| `integration-tests.yml` as a separate workflow | Absorbed into `ci.yml` | commit `2cf9919`, Phase 15.1 | `cicd.md`'s workflow-structure diagram and quoted `ci.yml` sample are both stale; a genuine DOCS-01 finding |
| `develop` branch in CI triggers | Retired; `push: branches: ['**']` | plan 15.1-09 | `cicd.md`'s quoted trigger block is stale in the same finding above |
| Two broken mdBook links (`docker.md:118`, `tool-integration.md:324`) | Fixed | Phase 15.1, commit `d87d11e` | `mdbook build` is green today (M-05); Phase 16 inherits a green book, does not need to fix it |

**Deprecated/outdated:**
- The `docs/Design/Design_and_Architecture.md` path — relocated by Milestone 11 Epic 2 to
  `docs/src/appendix/design-and-architecture.md`; any remaining reference to the old path in the
  fourteen DOCS-01 files (none found this session) would itself be a DOCS-01 finding.

## Assumptions Log

| # | Claim | Section | Risk if Wrong |
|---|-------|---------|---------------|
| A1 | `vhs`/`ttyd` are safe to install via the documented Charm APT repo / GitHub-release binary path | Package Legitimacy Audit, Standard Stack | Low-moderate — both flagged for `checkpoint:human-verify`; a compromised third-party APT repo or release binary is a real supply-chain vector, mitigated by gating the install behind human confirmation before it lands in both devcontainer Dockerfiles |
| A2 | `asciinema` is an acceptable pairing tool to satisfy D-14's `.cast` clause alongside VHS | Landmines / Pitfall 4 | Moderate — this is a planning recommendation to resolve a real gap in a locked decision, not a verified fact; the planner (or a return to `/gsd-discuss-phase`) should confirm this resolution rather than treat it as settled, since it adds a second tool dependency D-14 did not name |
| A3 | This sandbox's `crates.io` 403 / `repo.charm.sh` 429 do not reflect the real devcontainer/CI egress | Standard Stack, Pitfall 3 | Low — flagged explicitly as an environment-specific caveat needing re-verification, not treated as blocking |

**If this table is empty:** N/A — three assumptions recorded above.

## Open Questions

1. **How does D-14's `.cast` requirement get satisfied given VHS's confirmed output-format gap?**
   - What we know: VHS natively emits `.gif`/`.mp4`/`.webm`/PNG-frames only (verified from its own
     README this session). D-14 locks VHS and asks for both `.gif` and `.cast`.
   - What's unclear: Whether the user intended VHS alone (in which case `.cast` may need to be
     dropped with a recorded reason, similar to D-15's "adapted, not dropped" treatment of the README
     clause) or intended a VHS+asciinema pairing (not discussed in CONTEXT.md).
   - Recommendation: The planner should record an explicit resolution (most likely the pairing in
     Pitfall 4) rather than silently choosing one; if genuinely ambiguous, this is a candidate for a
     `checkpoint:human-verify` or a short return to `/gsd-discuss-phase` on this one point only —
     everything else in DOCS-04 is otherwise fully specified.

2. **Does the devcontainer's network egress support `cargo install --locked` for the mdbook toolchain
   and the VHS APT-repo install?**
   - What we know: This research sandbox could not reach `crates.io` (403) or `repo.charm.sh` (429);
     `github.com` and `static.rust-lang.org` were reachable (200).
   - What's unclear: Whether the actual execution environment (devcontainer rebuild, or CI) has the
     same restriction — this sandbox's network posture is not guaranteed to match.
   - Recommendation: First task in the DOCS-01/D-10 plan should be a direct network probe
     (`cargo install mdbook --version 0.4.40 --locked --dry-run` is not a real flag; use a plain
     `curl -sI https://crates.io` reachability check first) before committing later tasks to a full
     toolchain install.

## Environment Availability

| Dependency | Required By | Available | Version | Fallback |
|------------|------------|-----------|---------|----------|
| `mdbook` | D-10 local build/review, D-11 devcontainer pin | ✗ (this sandbox) | — | Cite CI's `docs.yml` output if local install is genuinely blocked in the real environment (documented deviation, Pitfall 3) |
| `mdbook-linkcheck` | D-10 linkcheck report review | ✗ (this sandbox) | — | Same as above |
| `mdbook-mermaid` | D-10/D-11 | ✗ (this sandbox) | — | Same as above |
| `vhs` | D-14 demo recording | ✗ (this sandbox) | — | No fallback for the primary tool — DOCS-04's demo-recording tasks cannot execute without it; must be installed in the real execution environment |
| `ttyd` | D-14 (VHS dependency) | ✗ (this sandbox) | — | Same as `vhs` — no fallback |
| `ffmpeg` | D-14 (VHS dependency) | ✗ (this sandbox) | — | Standard `apt-get install ffmpeg`, low risk |
| `asciinema` | Only if the VHS/`.cast` pairing (Open Question 1) is adopted | ✗ (this sandbox) | — | If the `.cast` requirement is instead withdrawn with a recorded reason, this dependency drops out entirely |
| `cargo doc` (rustdoc, stable) | DOCS-03 | ✓ | via `rustc`/`cargo` already installed in this session | — |
| `git` | all D-09 evidence gathering | ✓ | — | — |

**Missing dependencies with no fallback:**
- `vhs`, `ttyd` — DOCS-04's core deliverable cannot execute without them; install in the real
  devcontainer/CI runner, not assumed present.

**Missing dependencies with fallback:**
- `mdbook`, `mdbook-linkcheck`, `mdbook-mermaid` — CI already runs the equivalent build; if the local
  install genuinely cannot be completed, cite CI's output with an explicit, recorded caveat that
  D-10's "review the real local report" requirement was not fully met and why.
- `ffmpeg` — trivial standard-package fallback, effectively no risk.

## Validation Architecture

### Test Framework
| Property | Value |
|----------|-------|
| Framework | `cargo test --workspace --doc` (rustdoc doctests) + `mdbook build` (linkcheck) + phase-authored scripts (no unit-test framework applies — this is a documentation-content phase) |
| Config file | `docs/book.toml` (linkcheck settings); no dedicated test config for the doc-currency checks |
| Quick run command | `cargo doc --workspace --no-deps 2>&1 \| tee /tmp/doc-output.txt && ! grep -q "warning:" /tmp/doc-output.txt` |
| Full suite command | `cargo test --workspace --doc && mdbook build docs/ && ./scripts/check-doc-examples.sh` |

### Phase Requirements → Test Map
| Req ID | Behavior | Test Type | Automated Command | File Exists? |
|--------|----------|-----------|-------------------|-------------|
| DOCS-01 | Fourteen files current against 0.8.0 tree | content-diff (manual, evidence-recorded) | no single automatable command — per-signal greps recorded in the D-09 verdict record | ❌ Wave 0 — the verdict-record artifact itself doesn't exist yet |
| DOCS-01 | Linkcheck report reviewed | integration (mdbook) | `mdbook build docs/` (offline, `warning-policy = "error"`) | ✅ — `docs/book.toml` already configured |
| DOCS-02 | Architecture disposition recorded | manual (ADR review) | n/a — ADR authorship, not a test | ❌ Wave 0 — ADR-0047 doesn't exist yet |
| DOCS-03 | Zero `cargo doc` warnings | smoke (compiler diagnostic) | `cargo doc --workspace --no-deps 2>&1 \| tee /tmp/doc-output.txt && ! grep -q "warning:" /tmp/doc-output.txt` | ✅ — `ci.yml:63`, exact command exists today |
| DOCS-03 | 79 entry points carry `# Examples` | unit (phase-authored script) | `scripts/check-public-api-examples.sh` (Pattern 3 above) — **does not exist yet** | ❌ Wave 0 |
| DOCS-03 | `paladin-herald` lint uniformity | smoke | `cargo doc -p paladin-herald --no-deps 2>&1 \| grep -c warning` (expect 0, per M-07's measurement) | ✅ |
| DOCS-04 | Four demos recorded, offline, no credentials | smoke (manual run + recording) | `cargo run --example basic_paladin` (verified exit 0 this session per M-08) | ✅ — all four examples already exist |

### Sampling Rate
- **Per task commit:** `cargo doc --workspace --no-deps 2>&1 | tee /tmp/doc-output.txt && ! grep -q "warning:" /tmp/doc-output.txt` for any `.rs` doc-comment edit; `mdbook build docs/` for any `docs/src/**` edit.
- **Per wave merge:** `cargo test --workspace --doc && mdbook build docs/ && ./scripts/check-doc-examples.sh`.
- **Phase gate:** Full suite green before `/gsd-verify-work`, plus the D-09 verdict record complete for all fourteen files and the D-05 79-item `# Examples` check passing.

### Wave 0 Gaps
- [ ] `scripts/check-public-api-examples.sh` — the phase-authored script asserting all 79 D-05 entry
      points carry `# Examples` (no stable-Rust built-in exists — Pattern 3, Open Questions).
- [ ] The D-09 per-file verdict record artifact itself (a new `.md` under the phase directory, or a
      ledger amendment — Claude's Discretion item).
- [ ] `docs/DEMOS.md` — does not exist (M-09, confirmed again this session: `ls docs/DEMOS.md` →
      no such file).
- [ ] `docs/assets/recordings/` — does not exist (`docs/assets/` itself is absent).
- [ ] `.tape` scripts for the four demos — none exist yet under any path.

## Security Domain

### Applicable ASVS Categories

| ASVS Category | Applies | Standard Control |
|---------------|---------|-----------------|
| V2 Authentication | no | phase touches no auth code |
| V3 Session Management | no | phase touches no session code |
| V4 Access Control | no | phase touches no access-control code |
| V5 Input Validation | no | no new user-facing input surface |
| V6 Cryptography | no | phase touches no crypto code |
| V14 Configuration / Dependency Management | **yes** | pinned, `--locked` installs (`cargo install --locked --version X`) for every new devcontainer tool, matching CI's own pin exactly — this is the phase's only real supply-chain-adjacent surface |

### Known Threat Patterns for this stack

| Pattern | STRIDE | Standard Mitigation |
|---------|--------|---------------------|
| Unpinned `cargo install` / third-party APT repo drifting to a malicious release | Tampering | Pin every version explicitly (`--locked --version`), matching `docs.yml`'s pins exactly; gate the two non-crates.io tools (`vhs`, `ttyd`) behind `checkpoint:human-verify` since they cannot be verified through the automated package-legitimacy gate |
| `curl \| gpg --dearmor` key-install pattern (VHS's documented APT setup) | Tampering, Spoofing | Verify the GPG key fingerprint against Charm's published key out-of-band before trusting the repo; do not silently script this step without a human checkpoint on first install |
| Committing binary demo artifacts (`.gif`/`.cast`) to git | (not a STRIDE threat — a repo-hygiene concern) | Size-budget the recordings (Claude's Discretion item) before committing; consider `git-lfs` or CI-artifact hosting if the `.gif`s are large, rather than bloating the git history unbounded |

## Sources

### Primary (HIGH confidence — reproduced against this tree this session)
- `cargo doc --workspace --no-deps` — full output captured and grepped this session; all 20 warnings'
  `file:line` citations independently re-derived.
- `.github/workflows/ci.yml`, `.github/workflows/docs.yml` — read in full this session.
- `.devcontainer/Dockerfile`, `.devcontainer/Dockerfile.dev`, `.devcontainer/docker-compose.yml` —
  read this session; confirmed the existing pinned-install pattern (`cargo-release 1.1.2`,
  `cargo-deny 0.19.8`, `cargo-cyclonedx 0.5.9`) and that `docker-compose.yml:8` builds
  `Dockerfile.dev`.
- `docs/src/deployment/cicd.md` vs. actual `.github/workflows/*.yml` and `git log` — the fabricated-
  content finding.
- `git log --oneline --all -- .github/workflows/integration-tests.yml` — commit `2cf9919`.
- `.planning/decisions/0033-cargo-doc-warning-bar.md`, `.planning/decisions/0022-deprecation-requirement-withdrawal.md`, `.planning/decisions/PROMOTION.md` — read in full.
- `.planning/codebase/CONVENTIONS.md` (Comments section) — read; confirms `# Example` singular is the
  currently-documented (stale) convention D-06 must update.
- `gsd-tools query package-legitimacy check --ecosystem crates mdbook mdbook-linkcheck mdbook-mermaid`
  — all three `OK`.
- Line counts for all 14 DOCS-01 files (summed to exactly 10,337, cross-checking CONTEXT.md's own
  figure) and the architecture/appendix files.

### Secondary (MEDIUM confidence — official source fetched, not independently re-verified by a second source)
- `github.com/charmbracelet/vhs/blob/main/README.md` (`### Output`, `### Require`, install sections)
  — fetched directly via `curl` this session; confirms the `.cast` output gap.
- Web search on `rustdoc::missing_doc_code_examples` stability — confirms nightly-only,
  feature-gated status as of 2026.

### Tertiary (LOW confidence — not independently verified this session)
- `asciinema` as the pairing tool for `.cast` (Assumption A2) — a planning recommendation, not a
  verified requirement; flagged in Open Questions for explicit resolution.
- Charm APT/GitHub-release install commands quoted from a WebFetch summary of the VHS README, not
  independently executed (no network access to `repo.charm.sh` from this sandbox — 429 observed).

## Metadata

**Confidence breakdown:**
- Standard stack (mdbook toolchain): HIGH — versions and pins directly read from `docs.yml`, cross-
  checked against ADR-0033/CONTEXT.md, and the package-legitimacy gate returned `OK` for all three.
- Standard stack (VHS/ttyd/ffmpeg): MEDIUM — tool identity and output-format constraints verified
  from the official README; install commands not independently executed in this sandbox.
- Architecture/pitfalls (DOCS-01/DOCS-03 mechanics): HIGH — every warning, every line count, and the
  `cicd.md` fabrication finding were reproduced against the live tree this session, not inferred.
- DOCS-04 landmine (VHS `.cast` gap): HIGH confidence that the gap exists (primary-source-verified);
  MEDIUM confidence in the recommended resolution (pairing with asciinema), which needs planner or
  human confirmation.

**Research date:** 2026-08-24
**Valid until:** 30 days for the toolchain-version findings (pins may move if CI's `docs.yml` is
re-pinned); the `cargo doc` 20-warning residue and the `cicd.md` fabrication finding are valid until
the next commit touches those files — re-verify immediately before executing any DOCS-01/DOCS-03 task
if significant time has passed since this research.
