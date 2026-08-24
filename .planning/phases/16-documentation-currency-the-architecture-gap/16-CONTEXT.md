# Phase 16: Documentation Currency & the Architecture Gap - Context

**Gathered:** 2026-08-24
**Status:** Ready for planning

<domain>
## Phase Boundary

The documentation describes the system that exists. Four requirements, one theme:

- **DOCS-01** — the fourteen user-guide / deployment / operations pages Milestone 11 left open are
  checked against the 0.8.0 tree and marked current or updated, with the linkcheck report reviewed.
  This is **the only open checkbox count in all 542 that survives verification**, and it is settled
  by content, never by file existence or mtime.
- **DOCS-02** — `docs/src/appendix/design-and-architecture.md` gets a recorded disposition: archive
  material, or live deliverable. It cannot stay both.
- **DOCS-03** — one `cargo doc` bar, applied, with the public API documented to it.
- **DOCS-04** — the demos get a decision, and `docs/assets/` stops implying work in flight.

**Not in this phase:** anything that changes the coverage floor, the CI gate mechanism, the
provider adapters, or the code's ubiquitous-language naming. Documentation currency only.

</domain>

<decisions>
## Implementation Decisions

### Inherited from Phases 1, 5, 7, 8, 9, 10, 11, 12, 13, 14, 15, 15.1 and 17 — locked, not re-litigated

- **D-00a [informational]:** ADRs live in `.planning/decisions/`, flat sequential numbering, file
  shape `Status / Context / Decision / Considered Options / Code Locations / Code Conformance /
  Downstream Consumers`, **no frontmatter**. **Next free number is 0047** — confirmed at
  `.planning/decisions/PROMOTION.md:68`. This phase authors ADR-0047 and advances that line to 0048.
- **D-00b [informational]:** Precedence order is **ADR → shipped tree → `.planning/codebase/` map →
  `intel/code-verification.md` → PRD → DOC → task-list checkbox.** An ADR that contradicts shipped
  code is an instruction to change the code. *(Phase 1 D-02.)*
- **D-00d:** Ledgers, ADRs and requirement texts are **amended in place**, dated, superseded text
  retained. Never a separate corrections file. *(Phase 2 D-02.)* **D-07 and D-10 apply this
  directly.**
- **D-00e:** Evidence bar — no claim of closure without the exact command or `file:line` that
  produced it, recorded verbatim. *(Phases 3, 5, 7, 8, 9, 10, 12, 13, 14, 15, 15.1.)* **This phase
  leans on it harder than most: DOCS-01 is defined as unsettleable by inference.**
- **D-00g:** Contested positions get ADRs; code-settled defects get ledger rows and no ADR.
  *(Phase 7 D-17.)*
- **D-00h [informational]:** Medieval-military ubiquitous language is mandatory where a domain noun
  is coined, not for terms of art. *(CLAUDE.md; scope as clarified by Phase 15.1.)*
- **D-00o [informational] — `workflow.worktree_skip_hooks: true`** in `.planning/config.json`.
  Surface this in executor prompts or every commit cold-compiles the workspace.
- **D-00p [informational] — the coverage floor is 82% and the gate is live and required.**
  ADR-0006 is the binding record. **This phase does not touch the floor.**
- **D-00t [informational] — ADR-0033 already ratified the `cargo doc` bar.** Zero warnings on
  `cargo doc --workspace --no-deps`. HARD-07 picked it in Phase 10; **DOCS-03 applies it and does
  not re-litigate whether a bar should exist.** M8 Epic 5 FR-19's "warnings acceptable" position is
  already recorded superseded by outcome.
- **D-00u [informational] — the CI gate already exists.** `.github/workflows/ci.yml:63` runs the
  zero-warning form in the required `lint` job. DOCS-03's "adds the CI gate" clause is **already
  satisfied**; the gate is red only because the tree is. Do not add a second gate.
- **D-00v [informational] — DEBT-03 is discharged.** Phase 8 commit `2bffe22` re-enabled
  `paladin-ports` doctests; `ci.yml` carries no `--exclude` of any crate. Phase 16 inherits
  executable port-trait examples as *input*, not as a dependency to wait on.

---

### Measured this session — first-hand, supersedes inherited premises where they disagree

These were re-measured during discussion on 2026-08-23/24. Where they contradict REQUIREMENTS.md
or the Phase 13 hand-off block, **the measurement governs** per D-00b, and the stale text is
amended in place per D-00d rather than silently dropped.

- **M-01 — the 20-warning residue has not moved.** `cargo doc --workspace --no-deps` run this
  session: **20 warnings**, gate red. Split identical to ADR-0033's 2026-08-08 figure —
  `paladin-web` 13, `paladin-ai` 3, `paladin-battalion` 3, `paladin-herald` 1. Classes: 14
  unresolved links, 3 private-item links, 2 redundant explicit links, 1 unclosed HTML tag.
  *Minor drift:* the facade's three private-link warnings now cite `build_agent_registry`,
  `mcp_streamable_http_adapter` and `validate_config`, where ADR-0033 cited `build_agent` ×2 and
  `BearerToken::expose_secret`. Re-derive the citations rather than trusting the ADR's list.
- **M-02 — `missing_docs` is already clean.** Zero missing-documentation warnings workspace-wide.
  **FR-26.3's "enumerate every `pub` item in `src/` lacking `///`" is effectively already closed.**
  The open half is the `# Examples` requirement, not the `///` requirement.
- **M-03 — the architecture gap is one component, not seven.** `docs/src/architecture/` (5 pages,
  1,216 lines) covers Commander 6, Sanctum 11, Maneuver 4, Council 2, Conclave 2, Grove 2, and
  Sentinel has its own page at `docs/src/appendix/sentinel.md`. Against the 19 shipped
  ubiquitous-language components, the live chapter covers **18 of 19**; only **Sentinel** is absent
  from it.
- **M-04 — the appendix file is unchanged.** `docs/src/appendix/design-and-architecture.md` is
  **exactly 311 lines**, with Commander/Council/Conclave/Grove/Maneuver/Sanctum/Sentinel at **0
  mentions each** and **0 mermaid blocks**.
- **M-05 — the mdbook build failure is already fixed, and no longer unowned.** Phase 13 hand-off
  item 4 recorded two broken links (`deployment/docker.md:118`,
  `user-guides/tool-integration.md:324`) as owned by no phase. **Phase 15.1 fixed them** — commit
  `d87d11e`, "repair mdbook links breaking the Build MDBook required check", verified `mdbook
  build` → "No broken links found", exit 0. Phase 16 inherits a green book. **Amend the hand-off
  block in place.**
- **M-06 — the fourteen are stale by version, not by module path.** All 14 files exist; **12 were
  last touched 2026-06-02/03/06** by "in-place updates for **v0.4.3**". They carry **11 occurrences
  of `v0.4.3`** against a shipped **0.8.0**, plus a `paladin-battalion = { version = "0.5.0" }`
  dependency pin at `docs/src/user-guides/maneuver-flow-dsl.md:55`. All **7** cited
  `crates/…/*.rs` source paths still resolve — module-path drift is minimal. **Caution:** most
  `paladin-*` tokens in these files (`paladin-data`, `paladin-secrets`, `paladin-logs`, …) are
  Kubernetes object names in `kubernetes.md`, **not crate names** — a naive crate-name sweep is
  mostly false positives.
- **M-07 — `paladin-herald` opts out of the bar, and closing it costs nothing.**
  `crates/paladin-herald/src/lib.rs:20` carries **`#![allow(missing_docs)]`**, contradicting
  ADR-0033's "all ten library crates carry `#![warn(missing_docs)]`". Measured by flipping it and
  rebuilding: **zero** additional warnings. (The working tree was restored; this measurement left
  no diff.) `crates/doc-examples` carries neither attribute.
- **M-08 — DOCS-04's credential blocker is false.** All four named scenarios have matching
  examples that run on **mock adapters**: `MockLlmAdapter` in `examples/basic_paladin.rs` and
  `examples/council_discussion.rs`; inline mock `PaladinPort` impls in
  `examples/formation_sequential.rs` and `examples/grove_routing.rs`. Verified end-to-end:
  `cargo run --example basic_paladin` → **exit 0, offline, no credentials**. The requirement's
  "recordings also require live LLM API keys, which puts them outside any offline gate" is
  **measured false** for these four.
- **M-09 — `docs/assets/` does not exist at all** (not "exists and is empty" as DOCS-04's text
  says); `docs/DEMOS.md` does not exist. This confirms Phase 13 hand-off item 3 and contradicts
  the requirement text. `docs/src/assets/` is a **different, unrelated path** holding six
  architecture SVGs.
- **M-10 — the doc toolchain is absent locally.** `mdbook`, `mdbook-linkcheck`, `mdbook-mermaid`
  and `asciinema` are all missing. CI pins exact versions at `.github/workflows/docs.yml:44-54`:
  **mdbook 0.4.40, mdbook-mermaid 0.13.0, mdbook-linkcheck 0.7.7**, all `--locked`.

---

### DOCS-02 — the architecture document

- **D-01: Archive it, and signpost to the live chapter.** `docs/src/appendix/design-and-architecture.md`
  is recorded as **historical, superseded by `docs/src/architecture/`** plus
  `docs/src/appendix/sentinel.md`. Add a header banner saying so and pointing there; stop tracking
  FR-26.1 against this file. Re-anchor FR-26.1's success metric to the live chapter.
  *Rationale (M-03): the seven subsystems are not undocumented — they are undocumented in this one
  relocated pre-rewrite artifact. Rebuilding it would create a second architecture document
  competing with the live chapter, which is the exact duplication that produced this gap.*
  — **Reversibility:** costly — un-archiving means re-deriving the 311 lines' disposition and
  reversing a published ADR; the file itself is retained, so the content is not lost.
- **D-02: The substance closes by giving Sentinel a home in the live chapter.** Archiving settles
  the file's fate; **the metric closes at 19 of 19** by documenting Sentinel in
  `docs/src/architecture/` or cross-linking `docs/src/appendix/sentinel.md` from it. Recorded
  metric restatement: FR-26.1's "8 of 15+ → 15+ of 15+" becomes **18 of 19 → 19 of 19** against the
  live chapter, with the re-anchoring stated explicitly.
- **D-03: FR-26.1's four-Mermaid-diagram clause is withdrawn, with the reason and a mapping
  recorded.** The four named diagrams are overall hexagonal system architecture; Battalion
  orchestration patterns; data flow through a Paladin execution cycle; Arsenal/MCP tool integration
  flow. Record which of the six existing SVGs in `docs/src/assets/`
  (`ArchitectureOverview`, `LayerArchitecture`, `ComponentInteractionFlow`,
  `ContentProcessingPipeline`, `DeploymentArchitecture`, `data-flow`) answers each, plus the
  existing mermaid block in `docs/src/architecture/crate-map.md`, and withdraw any genuinely
  unanswered one with its reason. **Do not author diagrams into a file being archived.**
  — **Reversibility:** reversible — re-instatement is a later ADR authoring the diagrams into the
  live chapter.
- **D-04: One ADR-0047, following the ADR-0022 pattern.** A single ADR carries all three
  sub-decisions (archive; metric re-anchoring naming Sentinel; diagram-clause withdrawal with the
  SVG mapping). Per ADR-0022's precedent: **restate the stale premise rather than drop it**, and
  **write re-instatement down as an instruction, not a mechanism**. Update
  `.planning/decisions/PROMOTION.md`'s next-free line to **0048**.
  — **Reversibility:** costly — superseding a published ADR requires a successor ADR with a
  `## Supersedes` line per PROMOTION.md's supersession mechanism.

### DOCS-03 — the `cargo doc` bar and the `# Examples` requirement

- **D-05: "Public API entry point" is bound to the 79 items FR-26.3's own wording names** —
  **11 builders + 35 `*Port` traits + 33 `*Service` structs**. Enumerate them in the phase record
  so the definition can never drift again. Rejected: all 1,971 items in
  `.project/current-exports.txt` (mostly impl lines and re-exports for which an example is
  meaningless), and the 79-plus-204-`pub fn new` reading.
  **Measured baseline: 47 of 77 resolvable entry-point files already carry an example block; ~30
  do not.** The five crates with zero examples anywhere are `paladin-llm`, `paladin-storage`,
  `paladin-web`, `paladin-content`, `paladin-notifications`.
  — **Reversibility:** reversible — widening the definition later is additive.
- **D-06: Accept both heading spellings; normalise only the 79.** The tree carries `# Example`
  (212 sites) and `# Examples` (152). Rustdoc renders both identically and neither is a warning, so
  this is house style, not correctness. Normalise to **`# Examples`** on the 79 enumerated entry
  points so FR-26.3's wording is literally grep-satisfiable there; leave the other ~285 sites
  alone. Record the rule in the `.planning/codebase/CONVENTIONS.md` map so new code stops splitting.
  *Rejected: a 364-site sweep — hundreds of files touched for zero rendered-output change.*
- **D-07: Remove `paladin-herald`'s `#![allow(missing_docs)]`, and amend ADR-0033 in place.** Flip
  `crates/paladin-herald/src/lib.rs:20` to `#![warn(missing_docs)]` — measured at **zero** new
  warnings (M-07) — so the bar is genuinely uniform. Then amend ADR-0033 per D-00d with a dated
  note recording that its "all ten library crates" claim was inaccurate when written. **Also
  disposition `crates/doc-examples`**, the eleventh crate, which carries neither attribute.
  — **Reversibility:** reversible — the attribute is one line.
- **D-08: Clear the 20 warnings; leave the gate mechanism untouched.** Fix the tree, then prove
  green by running `.github/workflows/ci.yml:63`'s exact command and recording the output verbatim
  per D-00e. **Do not** switch to `RUSTDOCFLAGS='-D warnings'`, and **do not** add an mdbook or
  linkcheck gate — Phase 15.1 already pinned `Build MDBook` as a required check. Record explicitly
  that DOCS-03's "adds the CI gate" clause was already satisfied by `ci.yml:63` (D-00u) rather than
  delivered here.

### DOCS-01 — the fourteen files

- **D-09: The deliverable is a per-file currency verdict record, plus the edits it finds.** Fourteen
  rows, each carrying: the file; which signals were checked (version strings, dependency pins,
  crate names, module paths, `make` targets, workflow and job names, error types, feature flags);
  the exact command or `file:line` that produced each finding; and a verdict of **current** or
  **updated → commit**. This satisfies D-00e, makes "checked" auditable rather than asserted, and
  is **one artifact — not 26 tasks**, which DOCS-01 explicitly forbids. A "current" verdict must be
  worded so it cannot be mistaken for "unchecked".
  — **Reversibility:** reversible.
- **D-10: Install the doc toolchain locally, review the real linkcheck report, and make the install
  survive a devcontainer rebuild.** Run `mdbook build docs/` locally and read the actual linkcheck
  output, recording it verbatim — `docs/book.toml` sets `warning-policy = "error"` with
  `follow-web-links = false`, so the run is offline and deterministic. **Citing CI's pass/fail
  signal is not sufficient**: task 1.2 asks for the report to be *reviewed*.
  **User-added requirement:** the tooling must persist across devcontainer rebuilds.
- **D-11: Both Dockerfiles get the install, pinned to CI's exact versions.** Add **mdbook 0.4.40,
  mdbook-mermaid 0.13.0, mdbook-linkcheck 0.7.7** with `--locked --version` to **both**
  `.devcontainer/Dockerfile.dev` (the image `docker-compose.yml:8` actually builds) **and**
  `.devcontainer/Dockerfile`, matching the pinned convention already used there for `cargo-release`,
  `cargo-deny` and `cargo-cyclonedx`. Same versions as `docs.yml:44-54`, so a local report and a CI
  report cannot disagree on tooling. Updating only the active image would leave the other silently
  divergent.
  — **Reversibility:** reversible.
- **D-12: Mechanical signals checked exhaustively; prose read for contradictions; no style rewrite.**
  Check every checkable signal across all 10,337 lines, and read the prose for statements the 0.8.0
  tree contradicts — but do **not** re-author for structure or tone. Milestone 11 Epic 3 already
  rewrote this corpus once; DOCS-01 asks for "update in-place"; and a rewrite makes a currency fix
  indistinguishable from a style change in review. *Rejected: mechanical-only — DOCS-01 says content
  currency is not settleable by mechanical inference, and v0.4.3-era prose would survive untouched.*

### DOCS-04 — the demos

- **D-13: Record them — the stated blocker is measured false.** Per M-08, four mock-backed examples
  run offline at exit 0 with no credentials. Record the four demos, and **record in the phase that
  DOCS-04's live-key premise was measured false**, amending the requirement text in place per
  D-00d. This closes DOCS-04 on its own terms rather than withdrawing on a premise that is untrue.
- **D-14: VHS, with checked-in `.tape` scripts.** Settles Open Question 4 (asciinema vs VHS vs
  Terminalizer vs plain GIFs). VHS drives recordings from a `.tape` source, so a demo is
  **regenerable** rather than a hand-performed take that goes stale the moment example output
  changes — which matters in a phase whose entire theme is documentation that stays current. Emit
  **`.gif`** for embedding and **`.cast`** for FR-26.4's artifact shape, under
  `docs/assets/recordings/`. Provision the recorder in both devcontainer images alongside the
  mdbook tooling (D-11).
  — **Reversibility:** reversible — re-recording in another format is mechanical from the `.tape`
  sources.
- **D-15: `docs/DEMOS.md` is the index; the README gets one link to it.** Create `docs/DEMOS.md`
  with the four demos embedded, and add a **single line** to the README pointing at it. This
  honours FR-26.4's embedding clause without re-inflating the landing page Milestone 11 Epic 5
  deliberately made concise. **Record that the clause was adapted, not dropped** — the README it
  targeted changed shape under M11 Epic 5.
- **D-16: The four scenarios and their sources are fixed.** Basic Paladin Execution (30-60 s) →
  `examples/basic_paladin.rs`; Battalion Formation (45-90 s) → `examples/formation_sequential.rs`;
  Council Discussion (60-120 s) → `examples/council_discussion.rs`; Grove Routing (45-90 s) →
  `examples/grove_routing.rs`.

### Claude's Discretion

These were surfaced during discussion but not put to the user. The planner decides, and records the
choice with its reasoning:

- **Executable doctests for the ~30 new examples.** Whether each new `# Examples` block must
  compile and run as a doctest, or may be `no_run`/`ignore`. *Recommended default:* compile-and-run
  where the entry point permits it, `no_run` where it needs live I/O — and record the split, since
  ADR-0033 Finding 3 already notes 87 pre-existing `ignore`/`no_run`/`text` fences that DEBT-03
  deliberately left un-audited (D-10 of that phase).
- **`crates/doc-examples` disposition** under D-07 — it carries neither `warn` nor `allow`.
- **Plan splitting for the fourteen** — one sweep plan or several. 10,337 lines across three
  directories (`user-guides/` 6, `deployment/` 4, `operations/` 4) suggests a natural split, but
  the verdict record must remain a single artifact per D-09.
- **Where the D-09 verdict record lives** — phase directory artifact, ledger amendment, or both.
- **Whether the `.tape` scripts get a CI regeneration check** under D-14.
- **Whether `docs/assets/recordings/` commits binary artifacts to git**, and if so their size
  budget.

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Phase scope and requirements
- `.planning/ROADMAP.md` §"Phase 16: Documentation Currency & the Architecture Gap" — goal, the
  four success criteria, and the HARD-07 / DEBT-03 dependency notes.
- `.planning/REQUIREMENTS.md` — **DOCS-01 … DOCS-04** full text (the four `- [ ] **DOCS-0N**`
  blocks). Several of their stated premises are superseded by the measurements above; amend in
  place per D-00d rather than working around them.
- `.planning/REQUIREMENTS.md` §"Hand-off to Phase 16 / DOCS-01 … DOCS-04 — dated 2026-08-10 (plan
  13-13)" — the final ground-truth word on what this phase inherits. **Item 4 is superseded by
  M-05** and must be amended in place.

### Decisions this phase applies, amends, or must not re-open
- `.planning/decisions/0033-cargo-doc-warning-bar.md` — **the binding `cargo doc` bar.** Ratifies
  zero warnings on `cargo doc --workspace --no-deps`; records the 20-warning residue with Phase 16
  / DOCS-03 as named owner. **Amended by D-07** (the herald `allow(missing_docs)` claim).
- `.planning/decisions/0022-deprecation-requirement-withdrawal.md` — **the pattern D-04 follows.**
  A requirement withdrawn with the reason recorded, the stale premise restated rather than dropped,
  re-instatement written down as an instruction.
- `.planning/decisions/PROMOTION.md` — ADR conventions, required headings, the supersession
  mechanism, and the **next-free-number line (0047)** this phase consumes and advances to 0048.
- `.planning/decisions/0006-coverage-gate.md` — the 82% floor. Referenced so it is **not** touched.
- `.planning/decisions/0008-workspace-version-0-7-0.md` — pre-1.0 versioning semantics, relevant to
  how D-12 judges stale version strings.

### The documentation this phase changes
- `docs/src/user-guides/{orchestration,maneuver-flow-dsl,memory-management,tool-integration,paladin-configuration,output-formatting}.md`
  — DOCS-01 task 6.0, six files.
- `docs/src/deployment/{docker,kubernetes,production,cicd}.md` and
  `docs/src/operations/{logging,monitoring,performance-tuning,troubleshooting}.md` — DOCS-01 task
  7.0, eight files.
- `docs/src/appendix/design-and-architecture.md` — the 311-line file D-01 archives.
- `docs/src/architecture/{overview,hexagonal-design,domain-model,design-patterns,crate-map}.md` —
  the live chapter D-01 signposts to and D-02 closes the Sentinel gap in.
- `docs/src/appendix/sentinel.md` — Sentinel's existing page, the cross-link target for D-02.
- `docs/src/assets/` — the six architecture SVGs D-03 maps against the four named diagrams.
- `docs/book.toml` — linkcheck config: `warning-policy = "error"`, `follow-web-links = false`.
- `docs/src/SUMMARY.md` — the book's table of contents; the appendix entry's fate under D-01.
- `README.md` — the M11 Epic 5 landing page that gets one link under D-15.

### Code and tooling this phase touches
- `crates/paladin-herald/src/lib.rs:20` — the `#![allow(missing_docs)]` D-07 removes.
- `crates/paladin-web/src/{agent_auth,agent_registry,delivery_controller,openapi,agent_controller,app}.rs`
  — 13 of the 20 warnings. **Re-derive the citations; ADR-0033's list has drifted (M-01).**
- `src/infrastructure/adapters/arsenal/mcp_streamable_http_adapter.rs`,
  `src/infrastructure/web/agent_host.rs` — the facade's 3 warnings.
- `crates/paladin-battalion/src/in_memory_registry.rs` — 3 warnings.
- `.github/workflows/ci.yml:63` — the required zero-warning `lint` gate. **Read, run, do not
  modify** (D-08).
- `.github/workflows/docs.yml:44-54` — the pinned mdbook toolchain versions D-11 mirrors.
- `.devcontainer/Dockerfile.dev`, `.devcontainer/Dockerfile` — both get the pinned installs (D-11).
- `.devcontainer/docker-compose.yml:8` — establishes that `Dockerfile.dev` is the built image.
- `.project/current-exports.txt` — the 1,971-item public API baseline (regenerated 2026-08-17).
- `scripts/extract-public-api.sh`, `scripts/check-api-surface.sh` — how that baseline is
  regenerated and diffed, if D-05's enumeration needs it.
- `examples/{basic_paladin,formation_sequential,council_discussion,grove_routing}.rs` — the four
  demo sources (D-16).

### Project conventions
- `CLAUDE.md` and `.github/instructions/rust.instructions.md` — rustdoc conventions for all public
  items; the `///` / `//!` and `# Examples` house style D-06 normalises.
- `.planning/codebase/CONVENTIONS.md` §"Comments" — the existing documented rustdoc convention,
  which uses `# Example` (singular) in its own worked example. **D-06 updates this file.**
  *(Note: the `.planning/codebase/` maps are dated 2026-07-30 and predate Phases 12-17.)*

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets
- **`docs/src/architecture/` (5 pages, 1,216 lines)** — already the project's real architecture
  documentation, covering 18 of 19 shipped components. D-01's archive signpost points here; it is
  not a document that needs writing.
- **The six SVGs in `docs/src/assets/`** — `ArchitectureOverview`, `LayerArchitecture`,
  `ComponentInteractionFlow`, `ContentProcessingPipeline`, `DeploymentArchitecture`, `data-flow`.
  D-03 maps these against FR-26.1's four named diagrams instead of authoring new ones.
- **47 existing `# Example`/`# Examples` blocks on entry points** — the pattern to copy for the ~30
  missing ones. The worked reference is `PaladinBuilder::new` in
  `src/application/services/paladin/paladin_builder.rs`, quoted in the CONVENTIONS map.
- **Four ready-to-record examples** running offline on mock adapters (M-08) — no demo needs
  authoring, only recording.
- **`.project/current-exports.txt`** — an existing, regenerable enumeration of the public surface,
  if D-05's 79-item list needs cross-checking.

### Established Patterns
- **Pinned `cargo install --locked --version` in the devcontainer images** — `cargo-release 1.1.2`,
  `cargo-deny 0.19.8`, `cargo-cyclonedx 0.5.9` already follow this. D-11 adds three more in the
  same shape rather than inventing a convention.
- **`#![warn(missing_docs)]` on nine crates plus the facade** — D-07 makes it uniform.
- **ADR shape and in-place amendment (D-00a, D-00d)** — ADR-0022 is the closest structural
  precedent for what ADR-0047 must look like.
- **Evidence-first ledger rows (D-00e)** — every verdict in D-09's record needs its producing
  command or `file:line`.

### Integration Points
- **`ci.yml:63`** — the doc gate flips from red to green when D-08's 20 warnings clear. This is the
  phase's most visible externally-observable outcome.
- **`Build MDBook` required check** (pinned by Phase 15.1) — must stay green through every
  documentation edit in this phase; the local mdbook install from D-11 is how that is checked
  before pushing.
- **`docs/src/SUMMARY.md`** — any new or relocated page (Sentinel under D-02, `docs/DEMOS.md`
  surfacing under D-15) needs a decision about its table-of-contents entry.
- **`.planning/decisions/PROMOTION.md`** — the next-free-number line is shared mutable state;
  advance it to 0048 in the same commit that adds ADR-0047.
- **`README.md`** — one added line under D-15, and nothing else.

</code_context>

<specifics>
## Specific Ideas

- **"Make sure they will be installed next time the devcontainer is rebuilt."** The user's explicit
  addition to D-10. A local `cargo install` that vanishes on rebuild is not an acceptable outcome —
  the provisioning change in D-11 is part of the deliverable, not a convenience.
- **The archive banner should say where to look instead**, not merely that the file is historical.
  DOCS-02's own wording: a developer "finds a clear statement that the architecture appendix is
  historical **and where to look instead**".
- **Demos must be regenerable, not performed.** The reasoning behind D-14's VHS choice: in a phase
  about documentation that goes stale, a hand-recorded take is stale documentation with extra steps.
- **Do not convert DOCS-01 into 26 tasks, and do not dismiss it.** The requirement text says this
  explicitly; D-09's single verdict record is the shape that satisfies both halves.
- **Where a measurement contradicts the requirement text, amend the text in place** (D-00d) and say
  what was measured — do not quietly plan around it. This applies to M-05, M-08 and M-09 at minimum.

</specifics>

<deferred>
## Deferred Ideas

- **`Armory` appears 0 times in the tree.** CLAUDE.md's ubiquitous-language table defines Armory as
  the CLI tooling (`bin/paladin-cli.rs`), but no code uses the term. This is ubiquitous-language
  drift **in the code**, not in the documentation — out of scope for a documentation-currency
  phase, and it would need its own decision about renaming a shipped CLI surface.
- **`cargo-llvm-cov` is also missing from both devcontainer images.** This is precisely why the
  pending coverage-reproduction todo has never been walked. Fixing it alongside D-11 was offered
  and declined as a testing-area concern; it remains a one-line addition whenever someone picks up
  that todo.
- **Refresh the `.planning/codebase/` maps.** All seven are dated **2026-07-30** and predate Phases
  12 through 17. D-06 touches `CONVENTIONS.md` for one rule; a full refresh is `/gsd-map-codebase`
  work, not this phase.
- **Audit the five live architecture-chapter pages for currency** the way D-12 audits the fourteen.
  Offered under D-02 and declined — those five are in no requirement, and the phase's open-count is
  defined as exactly fourteen.
- **Harden the doc gate to `RUSTDOCFLAGS='-D warnings'`.** Offered under D-08 and declined. The
  current grep-on-`tee` check can match the per-crate summary lines as well as real warnings; a
  future phase may want rustdoc's own deny mode, which would need its own ADR since ADR-0033
  ratified the gate in its present form.
- **Whether the four withdrawn Mermaid diagrams are ever authored into the live chapter.** D-03
  withdraws the clause; ADR-0047 writes re-instatement down as an instruction per ADR-0022's
  pattern.

### Reviewed Todos (not folded)

- **"Verify local `make coverage` reproduces CI's 82.39% figure"**
  (`.planning/todos/pending/2026-08-13-verify-local-coverage-reproduction.md`, area `testing`,
  match score 0.6) — **reviewed, not folded.** It targets
  `docs/src/contributing/testing-guide.md`, which is not one of DOCS-01's fourteen, and it requires
  Docker plus `cargo-llvm-cov`, neither installed here, in a phase that is otherwise
  offline-checkable. Its own text states it should outlive its phase and not be silently closed —
  it deliberately carries no `resolves_phase` tag, and its owner is the repo maintainer.

</deferred>

---

*Phase: 16-Documentation Currency & the Architecture Gap*
*Context gathered: 2026-08-24*
