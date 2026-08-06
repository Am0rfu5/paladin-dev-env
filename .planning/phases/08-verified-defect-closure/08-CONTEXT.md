# Phase 8: Verified Defect Closure - Context

**Gathered:** 2026-08-06
**Status:** Ready for planning
**Mode:** `--auto` — every gray area was auto-resolved to its recommended option. Each decision below
carries the reasoning that produced it; **none was confirmed by a human.** Three decisions change a
shipped surface and are flagged `⚠ HUMAN REVIEW` — read those first if you read nothing else.

<domain>
## Phase Boundary

Fix the five defects that direct code verification proved open, so the guards this project believes
it has actually work — and record a decision behind any shipped surface that changes as a result.

**This is the first code-changing phase in three.** Phases 5, 6 (mostly) and 7 were records-only;
Phase 7's hard boundary was a provably empty `git diff -- '*.rs' 'Cargo.toml' '.github/'`. Phase 8
inverts that: it *must* touch `.rs`, `Cargo.toml` and `.github/workflows/`. Every plan is subject to
the workspace gate (`cargo test` → `cargo fmt --check` → `cargo clippy -- -D warnings`) and to
ADR-0006's 84% workspace line-coverage floor.

**Five deliverable classes:**

1. **DEBT-01 — a working `api-surface` CI job.** Nine stale `project/current-exports.txt`
   references (5 in tooling, 4 FR clauses across 5 requirement documents) corrected, plus a baseline
   that actually matches the tree, so an intentional API change fails CI and an unchanged tree
   passes. `check-deprecations.sh` gets to execute for the first time.
2. **DEBT-02 — the deprecation question answered either way.** Milestone 4 Epic 2 FR-8 is either
   implemented or **withdrawn with a recorded reason**, and `DEPRECATIONS.md`, the mdbook stable-API
   page and the tree end up agreeing. No third state.
3. **DEBT-03 — `paladin-ports` doctests execute.** `[lib] doctest = false` removed, `ci.yml`'s
   `--exclude paladin-ports` dropped, and the ~25 port traits' rustdoc examples compile.
4. **DEBT-04 — a library-only build with zero CLI dependencies.** `structopt`, `colored` and
   `comfy-table` out of a default library build — which, per ADR-0019, requires deciding the fate of
   the un-gated `paladin` binary *and* feature-gating `paladin-herald`'s formatters.
5. **DEBT-05 — one `TokenUsage`.** The two duplicates collapse into re-exports of the canonical
   `paladin-core` definition ADR-0016 named.

**Not in this phase:** the eight deprecated GitHub Action references (Phase 15 / PIPE-04 — DEBT-01
shed them deliberately); the `cargo doc --workspace --no-deps` warning-bar question (Phase 10 /
HARD-07 — DEBT-03 records the seam, does not decide it); building the `cargo tree` dependency-
allowlist check into CI (Phase 15, from ADR-0015); the user-facing binary-architecture mdbook page
(Phase 16, from ADR-0019); making the port traits' examples part of a published documentation
deliverable (Phase 16 / DOCS-03 — DEBT-03 only makes them *executable*); `VisionTokenUsage`
(a differently-named fourth type — see Deferred Ideas); Milestone 7-8 ground truth (Phase 10).

</domain>

<decisions>
## Implementation Decisions

### Inherited from Phases 1, 5 and 7 — locked, not re-litigated

- **D-00a:** ADRs live in `.planning/decisions/`, flat sequential numbering, file shape
  `Status / Context / Decision / Considered Options / Code Locations / Code Conformance /
  Downstream Consumers`, **no frontmatter** (matching 0001-0021, not the `adr-parser.cjs` schema).
  **`PROMOTION.md` records 0022 as next free.** *(Phase 1 D-01/D-03, Phase 7 D-00a/D-00h)*
- **D-00b:** Precedence order is **ADR → shipped tree → `.planning/codebase/` map →
  `intel/code-verification.md` → PRD → DOC → task-list checkbox.** An ADR that contradicts shipped
  code is an instruction to change the code. **Phase 8 is where three such instructions get
  executed** (ADR-0016 `must change`, ADR-0019 `must change`, and whatever D-06 produces).
  *(Phase 1 D-02)*
- **D-00c:** Source corrections under `.project/` are **annotation, not rewriting** — a dated
  correction banner naming what was wrong and pointing at the ADR or requirement, each defective
  claim corrected inline with the original text retained and marked superseded. `.project/` is the
  historical ingest corpus. *(Phase 5 D-08)*
- **D-00d:** Ledgers are **amended in place**, dated, superseded text retained. Never a separate
  corrections file. *(Phase 2 D-02)*
- **D-00e:** Evidence bar (the "D-19 bar"): no claim of closure without the exact command or
  `file:line` that produced it, recorded verbatim. *(Phases 3, 5, 7)*
- **D-00f:** Medieval-military ubiquitous language is mandatory in code, docs and comments.
  *(CLAUDE.md)*

---

### DEBT-01 — the `api-surface` CI job

- **D-01: Correct the five tooling references literally. No resolution cleverness.**
  `scripts/check-api-surface.sh:6` and `scripts/extract-public-api.sh:6` change their `BASELINE` /
  `OUTPUT_FILE` defaults from `project/current-exports.txt` to `.project/current-exports.txt`, and
  `ci.yml:172,182,187` change the same literal. Verified this session — all five are exactly as the
  ledger records, and `.project/current-exports.txt` exists at 442,369 bytes while
  `project/current-exports.txt` does not.
  Chosen over adding fallback logic that tries both paths, or an env-var indirection: a defect that
  survived three ingest runs because nobody looked at five literals is not fixed by adding a sixth
  place to look. This is the cheapest item in the phase and it should stay cheap.

- **D-02: The baseline must be regenerated, and that — not the path edit — is DEBT-01's real
  work.** `.project/current-exports.txt` is dated 2026-07-06, *before* Phases 2, 3 and 6 changed
  code (Phase 3 alone added ~43 tests and refactored `redis.rs`'s private surface; Phase 6 changed
  `route_task`). ROADMAP criterion 1 requires that **an unchanged tree makes the job pass** — a
  path fix against a stale baseline produces a job that fails for a *different* reason, which is
  not closure. The plan must run `./scripts/extract-public-api.sh .project/current-exports.txt`,
  commit the regenerated baseline, and record the diff's size and character in the SUMMARY.
  — **Reversibility:** reversible — the baseline is a generated artefact; regenerating it again is
  one command.

- **D-03: If `cargo public-api` cannot run in this environment, land the path fix and record a
  blocker. Do not fake the baseline.** `cargo public-api` needs a nightly toolchain and
  `cargo install`, i.e. network. The tree *does* build offline (`cargo doc --offline -p
  paladin-ports` and `cargo test --offline -p paladin-ports --lib` both ran during Phase 7), but
  nothing has proven `cargo install cargo-public-api` works here, and Phase 1's coverage
  measurement was halted by exactly this class of constraint. If regeneration is impossible: the
  five references still get fixed, the regeneration command is recorded as the documented
  procedure, and the DEBT-01 closure claim is scoped honestly to "path corrected; baseline
  refresh blocked, procedure recorded" — never "criterion 1 satisfied".

- **D-04: The five requirement-text references get D-00c annotation at source *and* a
  REQUIREMENTS.md correction.** The four FR clauses plus one `cross_refs` field —
  M8 Epic 7 FR-10, M12 Epic 1 §7, M12 Epic 5 §7, M12 Epic 6 `cross_refs`, M12 Epic 7 FR-4.6 — are
  annotated in `.project/` per D-00c, and the corresponding `REQ-*` rows in REQUIREMENTS.md's
  traceability are updated to point at the corrected path. Both sides, because the `.project/`
  documents are what a future implementer reads and REQUIREMENTS.md is what a future *planner*
  reads. Chosen over correcting only REQUIREMENTS.md: the M12 clauses were written in June 2026,
  months after commit `928c6d5`, which is proof the defect propagates from the source documents.

- **D-05: `check-deprecations.sh` is fixed in the same plan, because "it gets to execute" is
  meaningless if it cannot fail.** Verified this session: the script's primary branch pipes to
  `/dev/null` and its fallback branch `exit 0`s on *both* outcomes, so the only way it can fail is
  its final malformed-attribute grep — which scans `src/` only and misses all eleven crates.
  Scope, deliberately narrow: make the malformed-attribute check cover `src` **and** `crates`, and
  make the script's exit status meaningful. **Do not invent a new gate** (e.g. "fail if zero
  deprecations exist") — that would prejudge DEBT-02.

### DEBT-02 — implement or withdraw the deprecation requirement

- **D-06: Withdraw FR-8, with the reason recorded in a new ADR-0022 — do not manufacture
  deprecations to satisfy a grep.** ⚠ **HUMAN REVIEW.**
  The evidence read this session points one way, and it comes from the epic's own tracking document
  rather than from inference:
  - `DEPRECATIONS.md`'s **⚠ IMMEDIATE DEPRECATION** section — the only category that would produce
    a `#[deprecated]` attribute — lists, in full: *"None identified yet - managers are currently
    `pub(crate)` or will be moved to application layer (Epic 3)"*, plus "Migration Path: TBD" and
    "List: TBD based on usage analysis" for its other two sub-categories.
  - Its **SOFT DEPRECATION** category resolves to `#[doc(hidden)]`, not `#[deprecated]` — and the
    tree carries **38** `doc(hidden)` occurrences, i.e. that half was executed.
  - Its **INTERNAL-ONLY** category resolves to `pub(crate)` — no annotation by design.
  - Its stated timeline (**v0.2.0 → v0.3.0 → v1.0.0**) is stale by five minor versions: the
    workspace ships at **0.7.0** (root `Cargo.toml:34`, verified). A `#[deprecated(since = …)]`
    written today cannot honour a removal schedule keyed to v0.3.0.
  The zero grep result is therefore not an unfinished task — it is what the epic's own decisions
  produce. Recording that is the honest closure; inventing deprecation attributes so a grep returns
  non-zero would be the dishonest one.
  Chosen over: (a) implementing FR-8 by deprecating something — there is no named candidate anywhere
  in the corpus, so the implementer would have to invent the list; (b) leaving it open — the
  requirement text explicitly permits withdrawal and forbids the third state.
  — **Reversibility:** costly — withdrawing a published deprecation policy is a documented API-
  governance change; re-instating it later means re-deriving the candidate list Epic 2 never
  produced and re-opening the v0.2.0-era timeline against a 0.7.0 tree.

- **D-07: The three-way reconciliation is the deliverable, not the ADR.** Whichever way D-06 lands,
  DEBT-02's done-condition is that `DEPRECATIONS.md`, `docs/src/api-reference/stable-api.md` and the
  tree agree. Concretely, under D-06:
  1. `.project/.../Epic_2/DEPRECATIONS.md` gets a D-00c dated banner recording the withdrawal, the
     ADR number, and that its "Current Status / Deprecation Log" zeros are the *outcome*, not a gap.
     Its four Open Questions are answered or closed, not left dangling.
  2. `docs/src/api-reference/stable-api.md` — verified this session to carry ~15 deprecation
     clauses including a "Deprecation Lifecycle" section and a "🔴 Deprecated" tier — is updated so
     the *policy* (how a future deprecation will work) survives while any claim that deprecations
     **exist today** is corrected. The policy is not deleted; the framework will need it.
  3. The tree is unchanged — zero `#[deprecated]` is the recorded correct state.

- **D-08: The stale v0.2.0→v0.3.0→v1.0.0 timeline is restated against 0.7.0 inside ADR-0022, not
  silently dropped.** Same treatment ADR-0020 gave the build benchmark: judge the stale artefact,
  say why it is stale, and state what replaces it — here, that the pre-1.0 series absorbs API
  evolution through minor bumps per ADR-0008, so the deprecation policy's version anchors move to
  "one minor version" rather than named releases that already shipped.

### DEBT-03 — `paladin-ports` doctests

- **D-09: Measure before scoping. The recorded justification for `doctest = false` is stale, and
  that changes the size of this item.** Verified this session:
  `grep -rn 'use paladin::' crates/paladin-ports/src` returns **0**, and a wider
  `grep -rnE '(^|[^_a-z])paladin::'` filtered for the root crate returns **0** — the 19 apparent
  hits are all `paladin_core::platform::container::paladin::Paladin`, a *module* path, not the root
  crate. `crates/paladin-ports/src/output/llm_port.rs:671` already reads
  `pub use paladin_core::platform::container::token_usage::TokenUsage;`, and `:654` already
  documents `use paladin_ports::output::llm_port::TokenUsage;`.
  **The circular-dev-dependency problem the `Cargo.toml:14-18` comment describes appears to have
  been fixed already, and nobody removed the flag.** The first task therefore removes
  `doctest = false` and runs `cargo test --offline -p paladin-ports --doc` to produce a *measured*
  failure list; every later task's scope derives from that list, not from an estimate.
  Volume for sizing: **274** fenced blocks across 33 port files, of which **87** carry
  `ignore` / `no_run` / `text` — so roughly **187** executing candidates.

- **D-10: Fix failures by making examples compile. `ignore` is permitted only for examples needing
  a live external service, and each one gets a one-line reason.** Blanket-`ignore`ing a fence to
  turn the gate green reproduces the exact failure DEBT-03 exists to close — a documentation guard
  configured not to guard. The 87 pre-existing `ignore`/`no_run`/`text` fences are **not** audited
  by this phase; they are pre-existing state, and re-litigating them would triple the item.

- **D-11: `ci.yml:226`'s `--exclude paladin-ports` is dropped in the same commit that removes
  `doctest = false`.** Two halves of one guard; splitting them across plans creates a window where
  the crate's doctests exist and CI still refuses to run them. (Note the citation drift the Phase 7
  ledger flagged: the line is **226**, not the `:225` most of this corpus cites.)

- **D-12: Do not block on HARD-07.** REQUIREMENTS.md pairs DEBT-03 with HARD-07 (Phase 10), which
  settles which `cargo doc --workspace --no-deps` warning bar governs. DEBT-03's deliverable is
  *doctests executing*; the warning-bar question is separable and stays with HARD-07. The seam is
  recorded in the SUMMARY and in the ledger row so Phase 10 inherits an accurate state rather than
  a surprise. Chosen over sequencing Phase 8 behind Phase 10 — that would idle four independent
  DEBT items behind a question none of them needs answered.

### DEBT-04 — the library-only build

- **D-13: Migrate `src/main.rs` from `structopt` to `clap` v4, and give the `paladin` binary
  `required-features = ["cli"]`.** ⚠ **HUMAN REVIEW — this changes a shipped surface.**
  ADR-0019 established the constraint and left three doors open (gate it, migrate it, retire it).
  Re-verified this session: `structopt`'s only consumer in the entire tree is `src/main.rs`
  (`grep -rln structopt src/ crates/` → one file), and that file is the `paladin` binary, which has
  **no** `required-features` while `paladin-cli` and `paladin-server` both do.
  Migrate *and* gate, because either alone is insufficient: gating without migrating leaves
  `structopt` — a crate whose upstream declares itself superseded by clap 3+ — as an optional
  dependency nobody intends to keep; migrating without gating leaves `clap` unconditional and just
  renames the leak. Doing both makes all three `[[bin]]` targets consistent with ADR-0019's
  three-binary architecture, removes `structopt` from the manifest entirely, and leaves a default
  library build with zero CLI dependencies from the root manifest.
  **Explicitly rejected: retiring `src/main.rs`.** ADR-0019 has just recorded its purpose (the
  legacy content-aggregator service runner, `#[structopt(name = "smartcontent-aggregator")]`).
  Retiring a binary the previous phase documented as purposeful is new scope, not defect closure.
  **User-visible consequence to weigh:** after this change `cargo run` no longer builds the
  `paladin` binary without `--features cli`. That is the cost of criterion 4, and it must be called
  out in `CHANGELOG.md` and in the ADR — not discovered by a user.
  — **Reversibility:** costly — `required-features` on a default binary changes how every existing
  invocation, Dockerfile stage and CI leg builds it; reverting means re-auditing each consumer.

- **D-14: Feature-gate `paladin-herald`'s `colored` and `comfy-table` formatters. Root-manifest
  gating alone cannot satisfy criterion 4.** ⚠ **HUMAN REVIEW — this shrinks a crate's default
  public API.**
  Re-verified this session: `crates/paladin-herald/Cargo.toml:22-23` declares `comfy-table = "7.1"`
  and `colored = "2.1"` unconditionally, the crate has **no `[features]` section at all**, and
  `paladin-herald` is an **unconditional** root dependency (`Cargo.toml:22,54`). So both crates
  re-enter a library-only build through Herald regardless of what the root manifest does.
  Shape: add a `[features]` section to `paladin-herald` gating `table_herald.rs` (`comfy_table`) and
  the coloured path in `markdown_herald.rs` / `lib.rs` (`colored`); leave `json_herald.rs`
  unconditional; have the root `cli` feature enable them. Phase 7 specifics-3 read this as
  `superseded by shipped code` at the *ledger* level, but ROADMAP criterion 4 is stricter than that
  verdict and names all three crates explicitly — so the criterion governs, and the ledger row gets
  amended rather than the criterion narrowed.
  **If the planner finds this genuinely infeasible** (e.g. `colored` is threaded through Herald's
  trait signatures rather than its bodies), the fallback is to record the infeasibility in
  ADR-0023 with the `cargo tree` evidence and restate criterion 4 as root-manifest-scoped — but
  that fallback must be *proven*, not assumed, and it must be written down.
  — **Reversibility:** costly — a crate's default feature set is part of its published contract;
  Herald ships on crates.io.

- **D-15: Both D-13 and D-14 are recorded in one ADR-0023, "CLI dependency isolation and the
  binary/Herald surface".** They are one question — what a library-only consumer compiles — with
  two sites. The phase goal's own second clause ("no shipped surface is removed without a recorded
  decision behind it") is aimed precisely here. ADR-0023 cites ADR-0019 as its precondition and
  records the `cargo tree` command and output that proves criterion 4.

- **D-16: Criterion 4 is proved by running the command, not by reading the manifest.** The recorded
  proof is the literal `cargo tree --no-default-features` output (workspace-scoped, with the exact
  invocation used) showing none of `structopt`, `colored`, `comfy-table` — captured verbatim in the
  SUMMARY. If the flag combination in the requirement text does not exist as written, the plan
  records the equivalent invocation it used and why, rather than silently substituting.

### DEBT-05 — one `TokenUsage`

- **D-17: The canonical type absorbs the battalion copy's capabilities; it does not lose them.**
  Read this session, all three definitions carry **identical field sets** (`prompt_tokens`,
  `completion_tokens`, `total_tokens`, all `u32`) — but the derives differ:
  | Site | Derives | Inherent impls |
  |---|---|---|
  | `paladin-core/.../token_usage.rs:13` (canonical) | `Debug, Clone, Serialize, Deserialize` | none |
  | `paladin-core/.../battalion/mod.rs:497` | `Debug, Clone, **Default**, Serialize, Deserialize, **PartialEq**` | `new()`, `from_total()` |
  | `paladin-llm/src/llm_analysis_service.rs:51` | `Debug, Clone, Serialize, Deserialize` | none |
  So consolidation is **not** a pure deletion: `Default`, `PartialEq`, `new()` and `from_total()`
  must move onto `token_usage.rs` first, or every battalion call site breaks. Adding derives and
  inherent constructors to the canonical type is purely additive for existing consumers.
  Sequence, non-negotiable: (1) extend the canonical type; (2) replace `battalion/mod.rs:497` with a
  re-export; (3) replace `llm_analysis_service.rs:51` with a re-export; (4) grep-verify.

- **D-18: Both duplicate sites become `pub use` re-exports, preserving their existing import
  paths.** DEBT-05's own done-condition says "the other two sites are re-exports", and ADR-0016
  says the same. `paladin-ports` already demonstrates the exact pattern at
  `llm_port.rs:671`. This keeps every downstream `use` path resolving — no breaking change — while
  `grep -rn 'pub struct TokenUsage' crates src` returns exactly one.
  Chosen over deleting the duplicates and rewriting all importers: 179 `TokenUsage` references exist
  across `src`, `crates`, `tests` and `examples`; a re-export closes the requirement without
  touching any of them.
  — **Reversibility:** one-way — once the two bodies are collapsed, re-splitting means re-dividing a
  public type across two published crates, which is what ADR-0016 exists to prevent.

- **D-19: `paladin-llm` already depends on `paladin-core`, so the re-export is a one-line change
  with no new edge.** Verified: `crates/paladin-llm/Cargo.toml:27` declares
  `paladin-core = { package = "paladin-ai-core", … }`. No dependency-graph consequence, and
  ADR-0015's purity invariant is untouched.

- **D-20: `VisionTokenUsage` is out of scope and is recorded as such, not silently ignored.**
  `crates/paladin-ports/src/output/vision_port.rs:34` defines a fourth token-accounting struct under
  a different name, consumed by `paladin-llm`'s two vision adapters. DEBT-05's done-condition greps
  `pub struct TokenUsage` exactly, and ADR-0016 settles five named types that do not include this
  one. Whether it should converge on the canonical type is a real question — it belongs to the
  vision surface, not to this phase. See Deferred Ideas.

### Cross-cutting

- **D-21: Every closure claim is proved by a command run in this environment and recorded verbatim.**
  The D-00e evidence bar, applied to a code phase. The tree is known to build offline — Phase 7 ran
  `cargo test --offline -p paladin-ports --lib` (98 passed) and
  `cargo doc --offline -p paladin-ports --no-deps` under `RUSTDOCFLAGS="-D warnings"` (0 warnings).
  Prefer `--offline` throughout. Anything that genuinely cannot run here (D-03's `cargo public-api`)
  gets a recorded blocker with its procedure, never an inferred pass.

- **D-22: ADR allocation — 0022 (deprecation withdrawal) and 0023 (CLI dependency isolation).**
  `PROMOTION.md`'s next-free line advances to **0024**. Both carry `Code Conformance: must change`
  with Phase 8 itself named as the executor — the first ADRs in this corpus whose executing phase is
  their own. DEBT-01, DEBT-03 and DEBT-05 get **no** ADR: their answers are already recorded
  (ADR-0016 for DEBT-05) or are plain defect fixes with no competing defensible position — the
  Phase 7 D-17 rule.

- **D-23: The Milestone 4-6 ledger rows are amended in place as each item closes.**
  `.planning/ledgers/milestone-04-06.md` carries three `genuinely outstanding` rows that this phase
  closes — `REQ-api-surface-ci` (:115), `REQ-deprecation-warnings` (:116),
  `REQ-ports-doctest-compilation` (:157) — plus `REQ-ports-tests-and-rustdoc` (:160,
  `present, unproven`) and `REQ-workspace-ci-upgrade` (:225, `deferred with reason`, clause 3),
  whose verdicts change when DEBT-03 lands. Per D-00d: amend in place, dated, retain superseded
  text. The phase's close-out plan recounts the verdict distribution rather than adjusting it
  arithmetically — the Phase 7 07-13 lesson.

- **D-24 [informational]: Suggested decomposition — ~8 plans, 4 waves.** DEBT-05 is unblocked
  (ADR-0016 landed), DEBT-01 and DEBT-03 are fully independent, DEBT-02 and DEBT-04 each need their
  ADR first.
  - **Wave 1 (parallel):** ① DEBT-05 consolidation (D-17→D-19). ② DEBT-01 path fix +
    `check-deprecations.sh` (D-01, D-05) + baseline regeneration attempt (D-02/D-03).
    ③ DEBT-03 measurement spike: remove the flag, run the doctests, publish the failure list (D-09).
  - **Wave 2 (parallel):** ④ ADR-0022 + the DEBT-02 three-way reconciliation (D-06→D-08).
    ⑤ ADR-0023 (D-15) — written before either code change it authorises.
    ⑥ DEBT-03 example repair, scoped by wave 1's measured list (D-10, D-11).
  - **Wave 3 (sequential within, because both touch `Cargo.toml`):** ⑦ DEBT-04 — `src/main.rs`
    clap migration + `required-features` (D-13), then `paladin-herald` feature gating (D-14), then
    the `cargo tree` proof (D-16). Manifest contention makes this one plan, not two.
  - **Wave 4:** ⑧ Close-out — ledger amendments (D-23), REQUIREMENTS.md checkbox flips behind
    evidence, `PROMOTION.md` → 0024, `PROJECT.md` Key Decisions rows, `CHANGELOG.md` entries for
    D-13/D-14's user-visible changes, `COVERAGE.md`, coverage-floor re-check against ADR-0006.
  Plan-file naming is `08-NN-PLAN.md`.

### Claude's Discretion

- Exact `[features]` names in `paladin-herald` (D-14) — `pretty`/`table`, `formatters`, or a single
  `styled`. The constraint is that the root `cli` feature enables them and a default library build
  does not.
- Whether ADR-0022 and ADR-0023 are authored in their own plans or fold into the plans that execute
  them. D-24 suggests separate, so the decision is reviewable before the code lands; a planner with
  a reason to combine may.
- The precise `clap` v4 idiom for `src/main.rs` (derive vs builder). Derive is the closer analogue to
  the `structopt` code being replaced and keeps the diff legible.
- Banner wording and inline-correction markup for the D-04 and D-07 `.project/` annotations
  (D-00c fixes the pattern, not the prose).
- Whether the DEBT-03 measurement spike (wave 1 ③) publishes its failure list as a plan artefact or
  inline in its SUMMARY.
- Whether the regenerated `.project/current-exports.txt` (D-02) is committed in the same commit as
  the path fix or its own — the diff will be large and may read better alone.

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Phase scope and requirements

- `.planning/ROADMAP.md` §"Phase 8: Verified Defect Closure" (from line 572) — the goal, the
  dependency note, and the five success criteria. **Criterion 4 is stricter than Phase 7's ledger
  verdict on the same subject; see D-14.**
- `.planning/REQUIREMENTS.md` lines 821-937 — **DEBT-01 … DEBT-05 in full**, with their *Derives*
  provenance and the run-4 / run-5 extension banners. **This is the authoritative statement of
  scope**, and it is longer than the ROADMAP summary — DEBT-01 alone carries three extension blocks
  and a scope-split note.
- `.planning/REQUIREMENTS.md` lines ~3655-3665 — the DEBT-01…DEBT-05 traceability rows this phase
  flips.
- `.planning/REQUIREMENTS.md` lines ~3745-3762 — the cross-phase coupling table: ARCH-03(c) → DEBT-05
  (satisfied), HARD-07 → DEBT-03 (D-12 declines to block on it), DEBT-03 → DOCS-03 (Phase 16).

### The recorded answers this phase executes

- `.planning/decisions/0016-port-value-type-ownership.md` — **`Code Conformance: must change`, and
  Phase 8 / DEBT-05 is the named executor.** Names the canonical `TokenUsage`
  (`token_usage.rs:13`) and both duplicates. Read before touching DEBT-05.
- `.planning/decisions/0019-binary-target-architecture.md` — the three-binary architecture and the
  `structopt` / `src/main.rs` coupling that re-scopes DEBT-04. **`Code Conformance: must change`.**
- `.planning/decisions/0021-cli-application-layer-placement.md` — the CLI's layer placement, and the
  corrected fact that `src/application/mod.rs:57-59`'s `pub mod cli;` **is** `#[cfg(feature = "cli")]`
  -gated (an earlier Phase 7 research claim that it was un-gated is recorded wrong).
- `.planning/decisions/0008-workspace-version-0-7-0.md` — the version story D-08's timeline restatement
  cites.
- `.planning/decisions/0006-coverage-gate.md` — the **84% workspace line-coverage hard floor** every
  plan in this phase must not regress.
- `.planning/decisions/0015-core-ports-dependency-allowlist.md` — the purity invariant D-19 confirms
  DEBT-05 does not disturb.
- `.planning/decisions/PROMOTION.md` — the numbering index, **next free 0022**, and the five-step
  append procedure. Read before writing ADR-0022 or ADR-0023; update it in the close-out plan
  (→ 0024).
- `.planning/decisions/0001-battalion-config.md` … `0021-…` — the ADR file shape. **0022-0023 must
  match it** (no frontmatter, seven headings, per D-00a).

### Prior-phase context this phase inherits

- `.planning/phases/07-workspace-ground-truth-recorded-answers/07-CONTEXT.md` — D-00a…D-00h source,
  and **specifics 3**, which is the origin of D-13 and D-14. Read for the reasoning.
- `.planning/phases/07-workspace-ground-truth-recorded-answers/07-13-SUMMARY.md` §"Next Phase
  Readiness" — the explicit Phase 8 hand-off and the two flagged planner assumptions.
- `.planning/ledgers/milestone-04-06.md` §"Forward scope" — the two Phase 8 hand-offs stated in the
  ledger's own words.
- `.planning/ledgers/milestone-04-06.md` rows at lines 115, 116, 157, 160, 225 — the five rows D-23
  amends, each already carrying the exact `file:line` evidence this phase acts on.

### Defect sites — verified in this session, 2026-08-06

**DEBT-01 (5 tooling references, all confirmed):**
- `scripts/check-api-surface.sh:6` — `BASELINE="${1:-project/current-exports.txt}"`
- `scripts/extract-public-api.sh:6` — `OUTPUT_FILE="${1:-project/current-exports.txt}"`
- `.github/workflows/ci.yml:172`, `:182`, `:187` — the literal path, three times
- `.github/workflows/ci.yml:140-190` — the whole `api-surface` job, including the
  `check-deprecations.sh` step at `:175` that has never executed
- `.project/current-exports.txt` — exists, 442,369 bytes, **dated 2026-07-06** (D-02's staleness)
- `scripts/check-deprecations.sh` — both branches `exit 0`; the malformed-attribute grep scans
  `src/` only (D-05)

**DEBT-02:**
- `.project/Milestone_4-Refactor-Crates-Features/Epic_2/DEPRECATIONS.md` — §"⚠ IMMEDIATE
  DEPRECATION" (line 81) lists no candidates; §"Current Status" (171), §"Deprecation Log" (190),
  §"Open Questions" (206). **The primary evidence for D-06.**
- `docs/src/api-reference/stable-api.md` — ~15 deprecation clauses; §"Deprecation Lifecycle" (772),
  the "🔴 Deprecated" tier (183), the `check-deprecations.sh` reference (338), the "how long are
  deprecated APIs supported" FAQ (395). **The document D-07(2) reconciles.**
- `grep -rn '#\[deprecated' src crates` → **0**; `grep -rn 'doc(hidden)' src crates` → **38**
- `Cargo.toml:34` — `version = "0.7.0"` (D-08's anchor)

**DEBT-03:**
- `crates/paladin-ports/Cargo.toml:14-18` — `[lib] doctest = false` and the stale "Task 7.0" comment
- `.github/workflows/ci.yml:226` — `cargo test --workspace --doc --exclude paladin-ports`
  (**:226, not :225** — Phase 7 recorded the drift)
- `crates/paladin-ports/src/output/llm_port.rs:654,671` — doc example and `pub use` already on
  `paladin_ports::` / `paladin_core::` paths (D-09's evidence)
- 33 port files under `crates/paladin-ports/src/{input,output}/`; **274** fences, **87**
  `ignore`/`no_run`/`text`

**DEBT-04:**
- `Cargo.toml:93` `structopt = "0.3"`, `:125` `colored = "2.1"`, `:126` `comfy-table = "7.1"` — all
  unconditional
- `Cargo.toml:284` — `cli = ["dep:clap", "dep:dialoguer", "dep:indicatif", "dep:console",
  "dep:serde_yaml"]` (5 of 8)
- `Cargo.toml:22,54` — `paladin-herald` as an **unconditional** root dependency (D-14's mechanism)
- `src/main.rs:1-38` — the only `structopt` consumer; `#[structopt(name = "smartcontent-aggregator")]`
- `crates/paladin-herald/Cargo.toml:22-23` — `comfy-table`, `colored`; **no `[features]` section**
- `crates/paladin-herald/src/{table_herald.rs,markdown_herald.rs,lib.rs}` — the three files using
  them; `json_herald.rs` does not

**DEBT-05:**
- `crates/paladin-core/src/platform/container/token_usage.rs:13` — canonical
- `crates/paladin-core/src/platform/container/battalion/mod.rs:497` — duplicate **with `Default`,
  `PartialEq`, `new()`, `from_total()`** (D-17)
- `crates/paladin-llm/src/llm_analysis_service.rs:51` — duplicate, plain
- `crates/paladin-ports/src/output/llm_port.rs:671` — the re-export pattern to copy (D-18)
- `crates/paladin-llm/Cargo.toml:27` — the existing `paladin-core` edge (D-19)
- `crates/paladin-ports/src/output/vision_port.rs:34` — `VisionTokenUsage`, **out of scope** (D-20)

### Codebase maps and conventions

- `.planning/codebase/STRUCTURE.md` — corrected by Phase 7 to ten library crates plus `doc-examples`
  plus the root `paladin-ai` facade.
- `.planning/codebase/TESTING.md`, `.planning/codebase/CONVENTIONS.md` — the test and style
  conventions every code change here must match.
- `CLAUDE.md` and `.github/instructions/rust.instructions.md` — the workspace gate
  (`cargo test` → `cargo fmt --check` → `cargo clippy -- -D warnings`), the no-`unwrap()`-in-library
  rule, and the medieval-military ubiquitous language requirement.

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets

- **`crates/paladin-ports/src/output/llm_port.rs:671`** — a working, shipped instance of exactly the
  re-export DEBT-05 must produce twice. Copy it; do not invent a pattern.
- **`.planning/decisions/0001`-`0021`** — twenty-one ADRs in the target format. **0016 and 0019 are
  this phase's inputs**; 0006 and 0008 show how a later phase cites an earlier answer instead of
  re-deciding it; 0020 shows how to judge a stale artefact rather than reconcile it (the model for
  D-08).
- **`.planning/ledgers/milestone-04-06.md`** — every row this phase closes already carries its
  `file:line` evidence, freshly re-grepped by Phase 7. The scouting for DEBT-01, DEBT-02 and DEBT-03
  is done; this phase acts on it.
- **`crates/paladin-herald/src/json_herald.rs`** — the formatter that needs *no* CLI dependency, and
  therefore the shape of what a library-only Herald build keeps (D-14).
- **The `cli` feature's existing five-crate shape** (`Cargo.toml:284`) — D-13 and D-14 extend this
  list rather than creating a parallel feature.

### Established Patterns

- **Precedence is the project's core mechanic** (D-00b), and this phase is where it finally *bites*:
  ADR-0016 and ADR-0019 both carry `Code Conformance: must change` with Phase 8 named, so two of the
  five items are executing an instruction rather than choosing an approach.
- **Retain superseded text; amend in place; date every amendment** (D-00c, D-00d) — applies to the
  ledger, to `DEPRECATIONS.md`, and to every `.project/` annotation.
- **Contested positions get ADRs; code-settled defects get ledger rows.** D-22 allocates against it:
  two ADRs for the two decisions, none for the three plain fixes.
- **Documents lie about themselves in both directions.** `Cargo.toml:14-18`'s "Task 7.0" comment
  describes a circular-dependency problem the tree no longer has (D-09); `check-deprecations.sh`
  presents as a gate and cannot fail (D-05); `DEPRECATIONS.md` presents as an unfinished task list
  and is actually a completed decision record (D-06). **Read the artefact and the claim, and trust
  neither alone** — this phase's three biggest findings all came from doing that.
- **The workspace builds and tests offline.** Phase 7 proved it per-crate. Prefer `--offline`; treat
  anything needing `cargo install` or the network as a recorded risk (D-03).

### Integration Points

- **`Cargo.toml` (root)** — touched by D-13 (remove `structopt`, add `clap` to the bin path,
  `required-features` on `[[bin]] paladin`) and D-14 (`colored` / `comfy-table` removal or gating,
  `cli` feature extension). **Two decisions, one file → one plan** (D-24 wave 3).
- **`crates/paladin-herald/Cargo.toml` + its three formatter modules** — gains its first
  `[features]` section (D-14). Herald ships on crates.io; this is a published-contract change.
- **`crates/paladin-ports/Cargo.toml`** and **`.github/workflows/ci.yml:226`** — the two halves of
  the DEBT-03 guard, changed together (D-11).
- **`.github/workflows/ci.yml:140-190`** — the `api-surface` job; DEBT-01 changes three lines inside
  it and makes its second step reachable. **Do not touch `:148`'s `actions-rs/toolchain@v1`** —
  that belongs to Phase 15 / PIPE-04 (DEBT-01 shed it deliberately).
- **`.planning/ledgers/milestone-04-06.md`** — five rows amended in place (D-23).
- **`.project/` (6 documents)** — `DEPRECATIONS.md` plus the five requirement-text sources for
  DEBT-01, all annotated per D-00c, never rewritten.
- **`CHANGELOG.md`** — D-13's `required-features` change and D-14's Herald feature gating are both
  user-visible and must land here.
- **Phase 10 / HARD-07** — receives DEBT-03's post-change state (doctests executing) without the
  `cargo doc` bar being decided (D-12).
- **Phase 16 / DOCS-03** — receives executable port-trait examples as its input.

</code_context>

<specifics>
## Specific Ideas

**Four findings surfaced during this session that neither the ingest record nor Phase 7 contains.**
Each was read from the tree today. Treat them as verified starting points, not hypotheses.

1. **DEBT-03 is probably much smaller than the record implies — its blocker appears already
   fixed.** `crates/paladin-ports/Cargo.toml`'s `doctest = false` comment blames doc examples that
   "reference `paladin::` (root crate)". **There are none.** `grep -rn 'use paladin::'` over
   `crates/paladin-ports/src` returns **0**, and every one of the 19 `paladin::`-looking hits is the
   module path `paladin_core::platform::container::paladin::Paladin`. `llm_port.rs:654` already
   documents `use paladin_ports::output::llm_port::TokenUsage;`. The examples were evidently
   rewritten to crate-local paths at some point and the flag was never removed. **The first task
   should just remove the flag and run the doctests** — the answer may be "they already pass".

2. **DEBT-02's zero is a decision, not a gap — and the epic's own document says so.**
   `DEPRECATIONS.md`'s IMMEDIATE DEPRECATION category, the only one that would produce a
   `#[deprecated]` attribute, reads verbatim: *"None identified yet - managers are currently
   `pub(crate)` or will be moved to application layer (Epic 3)"*. Its other two categories resolve
   to `pub(crate)` and `#[doc(hidden)]` by design — and the tree carries 38 `doc(hidden)`
   occurrences, so that half **was** executed. Anyone reading only the ingest summary ("returns 0
   today") will plan an implementation task for a requirement whose own tracking document already
   answered it "none".

3. **DEBT-01's path fix is the easy half; the baseline is the hard half.**
   `.project/current-exports.txt` is dated **2026-07-06** — before Phases 2, 3 and 6 changed code.
   Correcting five literals against a five-week-old baseline yields a job that fails on a real
   diff, which reads identically to the broken state from a CI dashboard. Criterion 1's *"an
   unchanged tree makes it pass"* clause cannot be satisfied without regenerating it, and
   regeneration needs `cargo public-api` on nightly — the one step in this phase with a genuine
   environment risk.

4. **DEBT-05's consolidation is additive, not subtractive.** All three `TokenUsage` structs have
   identical fields, but the battalion copy carries `Default`, `PartialEq`, `new()` and
   `from_total()` that the canonical one lacks. A planner who reads "collapse into re-exports" as
   "delete two structs" will break every battalion call site. The canonical type must be **extended
   first**. There are 179 `TokenUsage` references across `src`, `crates`, `tests` and `examples`, so
   the re-export approach (D-18) is what keeps the change to three files instead of dozens.

**Scale note for the planner:** five requirements, ~8 plans, and an unusually wide range of task
sizes. DEBT-05 is three files and probably one plan. DEBT-01's path fix is five lines. DEBT-03 is
unmeasurable until wave 1 measures it (between "already passes" and "187 examples to repair").
DEBT-04 is the largest and riskiest — two published-contract changes in one manifest. **Do not size
the phase from the ROADMAP's five-bullet summary**; REQUIREMENTS.md lines 821-937 carry three
extension banners the summary omits.

</specifics>

<deferred>
## Deferred Ideas

- **`VisionTokenUsage` converging on the canonical `TokenUsage`** —
  `crates/paladin-ports/src/output/vision_port.rs:34`, a fourth token-accounting struct under a
  different name, consumed by `paladin-llm`'s OpenAI and Anthropic vision adapters. Out of DEBT-05's
  scope (its grep is `pub struct TokenUsage` exactly; ADR-0016 settles five named types that exclude
  it). A real question for the vision surface — belongs with Phase 14's API-contract work or its own
  item, not here. (D-20)
- **Auditing the 87 pre-existing `ignore` / `no_run` / `text` fences in `paladin-ports`** — DEBT-03
  makes the *executing* examples execute; whether the non-executing ones should be is a
  documentation-quality question owned by Phase 16 / DOCS-03. (D-10)
- **Retiring or replacing `src/main.rs`, the legacy content-aggregator entry point** — carried
  forward unresolved from Phase 7's deferred list. D-13 gates and migrates it; **whether the
  `smartcontent-aggregator` service runner should survive at all is still open**, and ADR-0019
  documented its purpose without endorsing its future.
- **The `#[structopt(name = "smartcontent-aggregator")]` product-name mismatch** — D-13's clap
  migration will have to write *some* name. Renaming the binary's user-facing identity is a product
  decision, not a dependency fix; record whatever is chosen and flag it if it is not a
  like-for-like carry-over.
- **Which `cargo doc --workspace --no-deps` bar governs** — Phase 10 / HARD-07. D-12 explicitly
  declines to decide it here.
- **A `cargo tree`-based dependency-allowlist check in CI** — Phase 15, from ADR-0015. D-16 runs
  `cargo tree` as a one-off proof; nothing enforces it on every build.
- **The user-facing binary-architecture mdbook page** — Phase 16, from ADR-0019 D-21.
- **The eight deprecated GitHub Action references** (including `ci.yml:148` inside the very job
  DEBT-01 fixes) — Phase 15 / PIPE-04, which owns the full sweep. DEBT-01 shed them deliberately;
  do not opportunistically bump them while editing the job.
- **Nyquist validation for Phases 1-4** — carried forward unresolved from Phases 5 and 7.
  Owner: `/gsd-validate-phase 1`…`4`.
- **Whether ADRs should be published to the mdbook for framework consumers** — carried forward
  unanswered from Phases 1, 5 and 7. Belongs with Phase 16's documentation work.

</deferred>

---

*Phase: 8-verified-defect-closure*
*Context gathered: 2026-08-06*
