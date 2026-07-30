# Phase 1: Ground Truth & Decision Records - Research

**Researched:** 2026-07-30
**Domain:** Internal documentation reconciliation — ADR authoring, cited status-ledger construction,
against an existing Rust hexagonal-architecture codebase. No new product code, no new runtime
dependencies.
**Confidence:** HIGH

<user_constraints>
## User Constraints (from CONTEXT.md)

### Locked Decisions

**ADR home & authority**

- **D-01:** ADRs live in `.planning/decisions/`, one file per decision. Phases 5, 7, 10 and 13
  append to the same directory. Chosen over `docs/src/appendix/adr/` (Milestone 11 Epic 3 declared
  the appendix a rewrite non-goal, and DOCS-02 in Phase 16 is already fighting a document that went
  invisible after being relocated there) and over `docs/adr/` (sits apart from `.planning/`, so
  downstream GSD agents need an explicit pointer to find it).

- **D-02:** The precedence order gains a new top tier:
  **ADR → shipped tree → `.planning/codebase/` map → `intel/code-verification.md` → PRD → DOC →
  task-list checkbox.** An ADR that contradicts shipped code is an instruction to change the code.
  This is what routes RECON-04 and the `citadel.rs` duplicate into Phase 2's GAP-07.
  PROJECT.md's stated precedence order must be updated to match.
  — **Reversibility:** costly — every ADR written from Phase 1 onward is authored against this
  ordering, and PROJECT.md, REQUIREMENTS.md and four sibling ground-truth phases cite it. Undoing
  it means re-deciding whether each recorded answer still binds.

- **D-03:** Because ADRs now top the order, **every ADR MUST carry a `Code conformance` field**
  with value `conforms` or `must change`. Where the value is `must change`, the ADR names the
  requirement that will execute it (normally GAP-07). This field is what keeps "authoritative"
  from being confused with "already true".

- **D-04:** ADR file shape is lean and evidence-first:
  `Status · Date · Question · Chosen variant · Evidence (file:line) · Rejected variants (the REQ-*
  IDs it resolves) · Code conformance · Downstream consumers`.
  Chosen over MADR (its "Considered Options" section would restate REQUIREMENTS.md's variant groups
  verbatim) and over the corpus's own two-file `decisions/` + `-options.md` house style (twelve
  files for six decisions).

- **D-05:** Phase 1 authors its six ADRs and **writes down the promotion procedure** for the eleven
  existing ADR candidates. Promotion is now viable without re-running the closed ingest, because
  ADRs live in `.planning/decisions/` and top the precedence order. **Phase 1 promotes none of the
  eleven** — each stays with its owning phase (7, 9, 10, 12, 13, 14), which gains a path where it
  previously had a blocker.

- **D-06:** PROJECT.md's `## Key Decisions` table gets one row per Phase 1 ADR, linking to the file.
  PROJECT.md itself predicts this ("The first real entries in this table are expected from Phase 1
  — six ADRs, one per competing variant pair"), and the table's "Empty by evidence, not by
  omission" note is replaced with a pointer to `.planning/decisions/`.

**Coverage gate (RECON-07)**

- **D-07:** **Re-measure before recording.** Run `cargo llvm-cov` against the current tree; the ADR
  records the gate alongside the measured figure, the exact command, and the date. Every documented
  baseline is stale or contested — 60.88% unit / 67.79% integration (Milestone 1, predates
  Milestones 2-12), ~78% overall (Milestone 3 release notes), 76-77% (Deferred-QA, February 2026,
  with two known-stale module paths).

- **D-08:** **Scope is one workspace-wide line-coverage number**, all tests combined, from one
  reproducible command (`cargo llvm-cov --workspace`). This is what RECON-07 literally asks for
  ("a single number and a single scope") and what Phase 15's CI gate and `make coverage` need.
  **Consequence:** Phase 3's success criterion 1, which names unit and integration coverage
  separately, must be amended to match. Flag this to the roadmap.

- **D-09:** **The gate is the re-measured baseline rounded down to a whole percent, hard-fail from
  day one**, with **80% recorded in the same ADR as the target** plus a named ratchet trigger.
  This answers the parent PRD's Open Question 3 ("hard fail or soft warning initially?") as
  hard-fail without the ramp-up problem it worried about — the gate cannot be red on day one by
  construction. Phase 3 raises real coverage; Phase 15 wires the floor into CI.
  Chosen over an immediate 80% hard gate (would make Phase 15's CI gate red until Phase 3 lands)
  and over Epic 25's phased ramp (three numbers where RECON-07 asked for one).

- **D-10:** The two module-scoped gates — **Herald ≥ 95%, autonomous ≥ 90%** — are named in the
  RECON-07 ADR, recorded as sitting above the global floor, and **explicitly not withdrawn**. Their
  placement is handed to **VERIFY-05 in Phase 5**, which the roadmap already assigns as their owner.

**Record vs decide (posture and the five type/gate answers)**

- **D-11:** **Default posture: shipped code wins unless the ADR argues otherwise in writing.**
  Deviation requires a stated reason and a `Code conformance: must change` flag. This matches the
  precedence order and `intel/code-verification.md`'s standing instruction not to plan a
  reconciliation for `BattalionResult`.

- **D-12: RECON-02 — `BattalionConfig`.** `crates/paladin-core/src/platform/container/battalion/mod.rs:37`
  is the one authoritative `BattalionConfig` (verified: it is the Epic 4 field set exactly;
  `CommanderConfig` does not exist anywhere in `crates/` or `src/`).
  The duplicate at `crates/paladin-core/src/platform/container/citadel.rs:280` is a self-described
  placeholder — its doc comment reads *"This is a placeholder and will be expanded in Epic 4"* —
  that Epic 4 superseded and nobody removed. **It is renamed** (e.g. `BattalionCheckpointConfig`),
  **keeping its three fields (`max_concurrency`, `timeout_seconds`, `continue_on_error`) and its
  serde shape.** It is a different concept — checkpoint/resume knobs, not orchestration config — so
  a distinct name is the accurate answer rather than a workaround.
  **No persisted-schema change, no migration** — chosen deliberately over replacing it with the
  real `BattalionConfig`, which would change `BattalionState`'s serialized form (`schema_version:
  "1.0.0"`, consumed by `crates/paladin-memory/src/citadel/file_citadel.rs`) and require a version
  bump plus a read path for existing checkpoints.
  — **Code conformance:** must change (rename lands in GAP-07)

- **D-13: RECON-03 — `BattalionResult`.** Pure recording. The shipped struct at
  `battalion/mod.rs:549` is a verified merged superset of all three positions. The ADR records what
  the superset chose: `per_paladin_times` in place of Epic 5's `execution_time_ms`, and
  `node_errors: Vec<NodeError>` in place of `errors: Vec<PaladinError>` (because `BattalionError`
  does not derive `Serialize`/`Deserialize` while `BattalionResult` does). Epic 8's Herald
  expectation is satisfied — Battalion type as `strategy_used`, aggregated tokens as `total_tokens`
  plus `per_paladin_tokens`.
  — **Code conformance:** conforms

- **D-14: RECON-04 — Formation minimum Paladin count.** **Formation relaxes to ≥ 1.**
  *Verified during discussion, and sharper than the documents describe: shipped code contains both
  halves of the contradiction.* `crates/paladin-battalion/src/commander.rs:1912`
  (`test_auto_selects_formation_for_single_paladin`) is a **passing** test asserting Auto routes a
  single Paladin to Formation, and `crates/paladin-core/src/platform/container/battalion/formation.rs:111`
  then rejects fewer than 2 at execution. **"Code wins" cannot resolve this — the tree argues with
  itself**, making this a second instance of the Group 29 class (a variant shipped code does not
  settle).
  Relaxing Formation leaves the passing Commander test and its Auto rule untouched, and matches
  Phase 2's success criterion 5 as written. Majority aggregation keeps its independent ≥ 3 check.
  Rejected: rewriting `analyze_and_select` (breaks a passing test and contradicts Phase 2's
  criterion); failing at `CommanderBuilder::build()` (removes single-Paladin Commander as a
  capability rather than reconciling).
  — **Code conformance:** must change (`formation.rs:109` lands in GAP-07)

- **D-15: RECON-05 — temperature validation.** **Provider-aware.** Add a temperature range to
  `ProviderCapabilities` and validate against the selected provider's range.
  *Verified during discussion:* `ProviderCapabilities` at
  `crates/paladin-ports/src/output/llm_port.rs:754` **has no temperature-range field at all**, so
  the provider-aware position was never implementable as specified. Meanwhile
  `src/application/services/paladin/paladin_builder.rs:1112` clamps to `[0.0, 1.0]` while
  `crates/paladin-llm/src/config/llm.rs:14` documents "Default temperature (0.0–2.0)" — the
  contradiction is live in the tree, not only on paper.
  This makes Epic 6 REQ-5's DeepSeek 0.0-2.0 range reachable through the normal Paladin path
  instead of unreachable by construction. The autonomous band logic
  (`src/application/services/paladin/temperature_service.rs`, bounds validated in
  `autonomous_config.rs:107-110`) stays a layer above as task-type guidance.
  Rejected: a global `[0.0, 1.0]` clamp (would require recording Epic 6 REQ-5 as withdrawn — an
  explicit capability reduction); adapter-level clamping (silent clamping past the point the caller
  can be told they were wrong).
  — **Code conformance:** must change (ports-layer change lands in GAP-07)
  — **Sequencing note:** Phase 14's WEB-03 corrects `supports_tool_calling` on the *same struct*.
  Do not schedule these independently.
  — **Reversibility:** costly — `ProviderCapabilities` is a published ports-layer type on the
  framework's primary integration contract; adding a field is additive, but every adapter must then
  populate it and downstream consumers branch on it.

- **D-16: RECON-06 — `Herald` trait.** Pure recording. *Verified during discussion:* the shipped
  trait at `crates/paladin-core/src/platform/container/herald.rs:49` ships the **v2** form exactly —
  `format_paladin_result`, `format_battalion_result` and `finalize_stream` returning
  `Result<String, HeraldError>`; `format_stream_chunk` returning `Result<Option<String>, HeraldError>`;
  plus `name()` and `mime_type()`. `format_error` is deliberately infallible (`-> String`), which is
  what makes FR-10's graceful-degradation requirement expressible. The ADR should record that
  asymmetry explicitly, not smooth it over.
  — **Code conformance:** conforms

**Ledger shape & evidence (RECON-01, RECON-08)**

- **D-17:** The ledger is a **new file per milestone** — `.planning/ledgers/milestone-01.md` — with
  REQUIREMENTS.md's existing "Milestone 1 as-shipped ledger" section reduced to a pointer. Phases 5,
  7, 10 and 13 each add a sibling. REQUIREMENTS.md is already ~4,000 lines and holds four
  as-shipped ledger sections; five sets of `file:line`-cited verdicts inline would make it
  unreadable.

- **D-18:** **Primary key is the `REQ-*` requirement ID, with outstanding task items nested under
  the requirement they belong to.** Satisfies RECON-01's "every outstanding Milestone-1 task item"
  without inventing identifiers for task-list checkboxes that have none (they are numbered
  positions inside 64 files), and keeps the ledger joinable to REQUIREMENTS.md and the roadmap.

- **D-19:** **Evidence bar: `satisfied` requires a `file:line` citation PLUS a named passing test,
  example, or command that exercises it.** A `file:line` with nothing exercising it gets a distinct
  verdict — **`present, unproven`**. This exists because "the code exists" has already burned this
  corpus: Milestone 4 Epic 3's task list is fully checked while three CLI-only dependencies remain
  unconditional.
  **Expect this to produce a third bucket nobody has counted yet.** That is the point, and the
  planner should budget for it rather than treat it as a surprise.

- **D-20:** **Verdict classes for the ledger:** `satisfied` · `present, unproven` ·
  `genuinely outstanding` · `deferred with reason` · `superseded by shipped code`.

- **D-21:** The three known divergences are recorded as **`superseded by shipped code` rows in the
  ledger**, not as separate ADRs: MCP Streamable-HTTP where the Milestone-1 PRD specified SSE;
  Qdrant/Sanctum where it specified `sqlite-vss`; and the shipped interactive REPL against Epic 9
  non-goal NG-7. The REPL row should be flagged loudly — it is a documented non-goal that shipped
  anyway, and it is the corpus's own evidence for why nothing here is treated as locked.
  Rejected: giving each its own ADR (grows Phase 1 from six to nine, and none is a competing
  variant pair); a standalone divergence register (a third document class, where Phases 7, 10 and
  13 already fold their equivalents into their ledgers).

### Claude's Discretion

- ADR file naming and numbering within `.planning/decisions/` (the user did not specify; pick a
  scheme that stays stable as Phases 5/7/10/13 append — a milestone-or-phase-scoped prefix is
  likely right, since a flat global counter across five phases invites collisions).
- Whether an ADR can later be superseded, and by what mechanism. Not discussed. Given D-02 puts
  ADRs at the top of precedence, the planner should record *something* here rather than leave it
  undefined.
- Coverage measurement exclusions — whether `examples/`, `benches/` and the `doc-examples` crate
  count toward the workspace number, and whether doctests are included. Not discussed. Must be
  pinned down in the RECON-07 ADR itself, because the number is meaningless without it and Phase 15
  has to reproduce it in CI.
- The concrete ratchet trigger for raising the coverage floor toward 80% (D-09 requires it be
  named; the user did not name it).
- The renamed identifier for the `citadel.rs` struct — `BattalionCheckpointConfig` was the
  discussion's example, not a locked choice.
- How ledger verification work is split across plans (per-epic fan-out vs sequential). Not
  discussed; Milestone 1 has ten epics.

### Deferred Ideas (OUT OF SCOPE)

- **Promoting the eleven existing ADR candidates.** Phase 1 builds the mechanism (D-05) but promotes
  none. Owners: Phase 7 (battalion-result upward dependency, CLI placement), Phase 9 (RustSec risk
  acceptance — carries the corpus's only expiry date, 2026-09-30), Phase 10 (cost-benefit
  assessment, licence checklist, facade reconciliation), Phase 12 (audit-suppression single-source
  invariant), Phase 13 (`AgentProvisioner` placement), Phase 14 (opaque-bearer-token decision),
  Phase 15 (coverage deferral record).
- **The Herald `format_error` asymmetry as a design question.** D-16 records it as shipped. Whether
  the whole trait should be uniformly fallible is a design change, not a variant resolution — out
  of scope here.
- **Amending ROADMAP.md Phase 3's success criterion for D-08.** The amendment itself is in scope
  (correcting the record at source is this phase's job); *re-scoping Phase 3's testing work* to the
  new single number is Phase 3's.
- **Coverage measurement tooling** — a `make coverage` target and `.codecov.yml` (neither exists).
  Phase 15's PIPE-02 owns both. Phase 1 only needs the number and a reproducible command.
- **Whether ADRs should be published to the mdbook** for framework consumers. Rejected as this
  phase's home (D-01), but "should the decisions be user-visible eventually?" is a real question
  and belongs with Phase 16's documentation work.
</user_constraints>

<phase_requirements>
## Phase Requirements

| ID | Description | Research Support |
|----|-------------|------------------|
| RECON-01 | Cited status ledger for every outstanding Milestone-1 task item, plus the three divergence rows and the ingest-count discrepancy | §Ledger sizing below gives the exact 39-item / 8-file scope from `intel/task-completion-state.md`, and the run-1 vs SYNTHESIS.md count discrepancy is located |
| RECON-02 | `BattalionConfig` ADR | D-12 already verified against the tree (spot-checked again in this research); §ADR Template gives the parseable shape |
| RECON-03 | `BattalionResult` ADR | D-13 already verified against the tree; recording-only, no code change |
| RECON-04 | Formation minimum Paladin count ADR | D-14 already verified against the tree (both halves of the contradiction confirmed present) |
| RECON-05 | Temperature validation ADR | D-15 already verified against the tree (`ProviderCapabilities` confirmed to have no temperature field) |
| RECON-06 | `Herald` trait ADR | D-16 already verified against the tree (full method set read and matched) |
| RECON-07 | Coverage gate ADR | §Environment Availability documents that `cargo-llvm-cov` is **not installed and crates.io is unreachable** in this sandbox — the re-measurement step needs a network-enabled environment or CI |
| RECON-08 | Epic 10 Task 7.0 dispute + 102-vs-103 discrepancy | §INGEST-CONFLICTS finding located verbatim, with the exact source citations for both documents |
</phase_requirements>

## Summary

This phase produces no product code — it produces six ADR files and one cited ledger file, both
net-new document classes for this `.planning/` directory (`.planning/decisions/` and
`.planning/ledgers/` do not exist yet). Every one of the six ADR "verdicts" (D-12 through D-16, plus
the coverage scope in D-07/D-08) has already been spot-verified against `release/v0.7.0` during this
research pass by direct file reads — CONTEXT.md's claims are accurate, not just plausible. That
means the planner's job for RECON-02, 03, 04, 06 is transcription into the ADR shape, not further
investigation. RECON-05 and RECON-07 carry real remaining work: RECON-05 requires designing the new
`ProviderCapabilities` field (a ports-layer addition with fan-out to three adapters, deferred to
GAP-07 but the *shape* of the field belongs in the ADR); RECON-07 requires actually running
`cargo llvm-cov --workspace`, which is not currently possible in this research sandbox (tool absent,
package registry unreachable — see Environment Availability) and must be flagged to the plan as an
execution-environment dependency, not a research gap.

The one piece of real leverage this research adds beyond CONTEXT.md: GSD ships a structural ADR
parser (`gsd-core/bin/lib/adr-parser.cjs`) that classifies Markdown headings into canonical buckets
(status, decisions, considered_options, key_files, consequences, …) using a synonym table — it does
**not** read YAML frontmatter. D-04's proposed field names ("Question", "Evidence (file:line)",
"Rejected variants", "Code conformance", "Downstream consumers") only partially match that synonym
table. Naming the ADR sections to hit the parser's recognized synonyms (`## Status`, `## Context`,
`## Decision`, `## Considered Options`, `## Code Locations` / `## Affected Files`) costs nothing
structurally and buys the ADRs machine-readability for GSD's own tooling (notably the "ADR Ingest
Express Path" that later phases can use to skip `/gsd-discuss-phase` and synthesize CONTEXT.md
straight from an ADR) — for free, without deviating from D-04's field list in substance. See
§Don't Hand-Roll and §Code Examples.

The ledger's real sizing is now known precisely: Milestone-1's outstanding task-item universe is
**39 open checkboxes across 8 files** (`intel/task-completion-state.md`), heavily concentrated in
Epic 6 (19 of 39). These are almost all *parent*-level checkboxes (`6.0`, `7.0`) left unchecked
while their subtasks are done — which is exactly the shape D-19's `present, unproven` bucket exists
to catch. This sizing should directly inform the "per-epic fan-out vs sequential" discretion call:
8 files is small enough for one pass, but Epic 6's 19 open items justify giving it its own plan or
task if the fan-out is chosen.

**Primary recommendation:** Treat this phase as a transcription-and-one-measurement task, not an
investigation task — the investigation is already done (in CONTEXT.md and re-confirmed here). Budget
the actual effort on (a) getting `cargo llvm-cov --workspace` to run somewhere with network access,
and (b) opening all 8 task-list files with open checkboxes to read their literal item text (not just
counts) so the ledger's nested rows are accurate, not inferred.

## Architectural Responsibility Map

| Capability | Primary Tier | Secondary Tier | Rationale |
|------------|-------------|----------------|-----------|
| ADR authoring (`.planning/decisions/*.md`) | Documentation / Planning | — | Pure `.planning/` artifact; no runtime tier owns it |
| Cited status ledger (`.planning/ledgers/milestone-01.md`) | Documentation / Planning | — | Same; joins to REQUIREMENTS.md and shipped-tree citations |
| Coverage measurement (`cargo llvm-cov --workspace`) | Build/CI tooling | Database/Storage (none) | A dev-tool invocation against the compiled workspace; not a runtime capability of the shipped product |
| `BattalionConfig` / `BattalionResult` / Formation / temperature / `Herald` — the six contested definitions | API / Backend (Core domain) | — | All six live in `paladin-core`, `paladin-ports`, `paladin-battalion` — the domain/port layer, not any client-facing surface. Recording their ADRs does not touch Browser, SSR, or CDN tiers at all |
| PROJECT.md / ROADMAP.md / REQUIREMENTS.md corrections | Documentation / Planning | — | Source-of-truth updates required by D-02, D-06, D-08, D-17 |

**Note for the planner:** unlike most phases, this one has no Browser/Frontend/API/CDN tier work at
all — Paladin is a Rust library + CLI + HTTP service, and this phase touches none of the HTTP
surface, none of the CLI, and no persistence schema (D-12 explicitly rejects a schema migration).
The "Primary Tier" column above is included for consistency with the phase-planning contract, not
because tier misassignment is a live risk here.

## Standard Stack

### Core

| Tool | Version | Purpose | Why Standard |
|------|---------|---------|--------------|
| `cargo llvm-cov` | 0.8.7 (docs.rs, checked 2026-07-30) [CITED: docs.rs/crate/cargo-llvm-cov] | Workspace-wide line-coverage measurement for RECON-07 | Already the project's own choice — `.github/workflows/integration-tests.yml:117-118` runs `cargo install cargo-llvm-cov` and `cargo llvm-cov --features integration-tests --lcov --output-path integration-lcov.info` today [VERIFIED: .github/workflows/integration-tests.yml:117-118]. D-07/D-08 lock this as the tool of record; do not introduce `tarpaulin` or `grcov` even though `codebase/TESTING.md` documents `cargo tarpaulin` as an older local-dev habit — that document is stale relative to the CI file |
| `gsd-core/bin/lib/adr-parser.cjs` | in-repo, no version | Structural Markdown → JSON parser GSD uses to read ADR files (status, decisions, considered options, key files, consequences) | Already shipped with the GSD toolchain in this repo; using its recognized heading synonyms costs nothing and makes the six new ADRs machine-readable by GSD's own "ADR Ingest Express Path" [VERIFIED: .claude/gsd-core/bin/lib/adr-parser.cjs] |

### Supporting

| Tool | Version | Purpose | When to Use |
|------|---------|---------|-------------|
| `ripgrep` / `grep -rn` | n/a | Citation verification — confirming every `file:line` claim in the ADRs and ledger against the actual tree before marking a verdict | Every ADR and every ledger row, before it is written, per this project's own precedence rule ("shipped tree" outranks everything) |
| `rustup component` (`llvm-tools`) | already installed in this environment [VERIFIED: `rustup component list --installed` output] | LLVM coverage instrumentation backend `cargo-llvm-cov` depends on | Confirm present before attempting the coverage run; it already is in this sandbox, so only the `cargo-llvm-cov` binary itself is the gap |

### Alternatives Considered

| Instead of | Could Use | Tradeoff |
|------------|-----------|----------|
| `cargo llvm-cov --workspace` | `cargo tarpaulin` | Rejected by D-08 implicitly (the CI pipeline already standardized on llvm-cov); tarpaulin gives a different, generally lower number on the same tree and would not match what `integration-tests.yml` reports |
| Markdown-heading ADRs (D-04 shape mapped onto parser synonyms) | YAML-frontmatter ADRs | `adr-parser.cjs` does not read frontmatter at all — it sectionizes on Markdown headings only. A frontmatter-based ADR format would be invisible to GSD's own ADR tooling |

**Installation (only if the coverage measurement must run in this environment):**
```bash
# llvm-tools component is already installed here; confirm with:
rustup component list --installed | grep llvm-tools

# cargo-llvm-cov itself is not installed and crates.io returned HTTP 403
# from this sandbox during research (see Environment Availability below).
cargo install cargo-llvm-cov   # requires network access to crates.io
cargo llvm-cov --workspace --summary-only
```

**Version verification:** `cargo-llvm-cov`'s presence in this project's own CI
(`.github/workflows/integration-tests.yml:117`) is the authoritative confirmation that this is the
correct tool for this project — stronger evidence than the registry version lookup, which is
provided only as a freshness check.

## Package Legitimacy Audit

This phase installs no new runtime or build dependency into `Cargo.toml`. The one tool a plan step
may need to install on the executing machine is `cargo-llvm-cov`, a `cargo` subcommand already used
by this project's own CI. Ran the legitimacy gate on it anyway for completeness:

```
gsd-tools query package-legitimacy check --ecosystem crates cargo-llvm-cov
→ { "name": "cargo-llvm-cov", "verdict": "OK",
    "signals": { "publishedAt": "2021-01-22", "weeklyDownloads": 101781,
                 "repoUrl": "https://github.com/taiki-e/cargo-llvm-cov",
                 "deprecated": false, "postinstall": null } }
```

| Package | Registry | Age | Downloads | Source Repo | Verdict | Disposition |
|---------|----------|-----|-----------|-------------|---------|-------------|
| `cargo-llvm-cov` | crates.io | ~5.5 yrs (since 2021-01-22) | ~101,781/wk | github.com/taiki-e/cargo-llvm-cov | OK | Approved — already used in `.github/workflows/integration-tests.yml:117`, so this is a re-confirmation, not a new-dependency decision |

**Packages removed due to `[SLOP]` verdict:** none.
**Packages flagged as suspicious `[SUS]`:** none.

No `checkpoint:human-verify` is required for this tool — its provenance is the project's own shipped
CI configuration, not WebSearch or training-data discovery, which is the strongest tier this
protocol recognizes.

## Architecture Patterns

### System Architecture Diagram

This phase does not touch runtime data flow — it produces static documentation artifacts consumed
by later phases and by human readers. The "flow" that matters is the document-authoring pipeline:

```
                      ┌─────────────────────────┐
                      │   shipped tree (v0.7.0) │  ◄── ultimate evidence source
                      │  crates/*, src/*        │      (grep/read, file:line)
                      └────────────┬────────────┘
                                   │ cite
                                   ▼
   REQUIREMENTS.md   ┌─────────────────────────┐   .planning/codebase/*.md
   (6 variant groups)│   Phase 1 author pass   │   intel/code-verification.md
        ────────────►│                         │◄──────────── (secondary evidence,
                      │  1. Verify each REQ-*   │               already-refreshed maps)
                      │     citation against    │
                      │     the tree            │
                      │  2. Write 6 ADRs        │──────► .planning/decisions/*.md
                      │  3. Run coverage once   │              (new dir, D-01)
                      │  4. Nest 39 open task   │
                      │     items under REQ-*   │──────► .planning/ledgers/
                      │     IDs                 │              milestone-01.md
                      │  5. Update PROJECT.md   │              (new dir, D-17)
                      │     precedence + Key    │
                      │     Decisions table     │──────► PROJECT.md (edited)
                      └─────────────────────────┘
                                   │
                                   ▼
              Phase 2 (GAP-07) reads "Code conformance: must change" ADRs
              Phase 3 reads the RECON-07 coverage number
              Phase 5/7/10/13 append siblings to decisions/ and ledgers/
```

### Recommended Project Structure

```
.planning/
├── decisions/                          # NEW — created by this phase (D-01)
│   ├── 0001-battalion-config.md        # RECON-02
│   ├── 0002-battalion-result.md        # RECON-03
│   ├── 0003-formation-min-paladins.md  # RECON-04
│   ├── 0004-temperature-validation.md  # RECON-05
│   ├── 0005-herald-trait.md            # RECON-06
│   ├── 0006-coverage-gate.md           # RECON-07
│   └── PROMOTION.md                    # D-05 — the promotion procedure for the 11 candidates
├── ledgers/                            # NEW — created by this phase (D-17)
│   └── milestone-01.md                 # RECON-01, RECON-08
├── PROJECT.md                          # edited: precedence order (D-02), Key Decisions (D-06)
├── ROADMAP.md                          # edited: Phase 3 success criterion 1 flagged for D-08
└── REQUIREMENTS.md                     # edited: "Milestone 1 as-shipped ledger" reduced to a pointer (D-17)
```

**Numbering scheme recommendation (Claude's Discretion item 1):** use a flat, zero-padded,
monotonic counter (`0001-…` through `0006-…` for Phase 1) rather than a phase-scoped prefix. A
phase-scoped prefix (`p01-…`, `p05-…`) reads naturally today but breaks the moment an ADR is
superseded by a *later* phase's ADR — the reader has to know which phase number is "newer" rather
than just comparing the counter. A flat counter surviving Phases 1, 5, 7, 10, 13 needs only one
shared piece of state (the next free number); record that next-free-number pointer in
`.planning/decisions/PROMOTION.md` or a similar small index file so Phase 5 does not have to `ls`
the directory to find the next number.

### Pattern 1: ADR heading shape that is both D-04-compliant and adr-parser.cjs-parseable

**What:** Author each ADR using Markdown H2 headings chosen from `adr-parser.cjs`'s
`CANONICAL_HEADERS` synonym table wherever a D-04 field has a matching synonym, and a plain custom
heading (unmapped, but still human-readable) for the two fields that have no synonym
(`Code conformance`, `Downstream consumers`).

**When to use:** All six Phase 1 ADRs, and by extension every ADR Phases 5, 7, 10, 13 append.

**Mapping (verified against `.claude/gsd-core/bin/lib/adr-parser.cjs` `CANONICAL_HEADERS`):**

| D-04 field | Parser bucket | Matching heading to use | Notes |
|---|---|---|---|
| Status | `status` | `## Status` | First line must be a bare word for clean classification: `Accepted` (the parser recognizes `accepted`/`proposed`/`superseded`/`rejected`/`deprecated` case-insensitively) |
| Date | *(no bucket)* | Put as a line under `## Status` (e.g. `**Date:** 2026-07-30`) | Not machine-extracted; harmless as prose |
| Question | `goal` | `## Context` | `context` is an exact synonym match |
| Chosen variant | `decisions` | `## Decision` | `decision` is an exact synonym match; write it as a bullet list so `splitEntries` gives clean array entries, not one giant paragraph |
| Evidence (file:line) | `key_files` | `## Code Locations` | `code locations` is an exact synonym match (also acceptable: `## Affected Files`) |
| Rejected variants (REQ-* IDs) | `considered_options` | `## Considered Options` | Exact synonym match; list each rejected `REQ-*` variant as its own bullet |
| Code conformance | *(no bucket)* | `## Code Conformance` | Goes to `unmapped_headers` in the parsed output — acceptable, since nothing currently consumes this field programmatically; still write it, it is D-03's contract |
| Downstream consumers | `dependencies` | `## Downstream Consumers` — **will not match** any dependencies synonym as written | If machine-readability of this field matters later, rename to `## Dependencies` or `## Related ADRs`; otherwise leave as unmapped prose |

**Example (RECON-06, Herald trait — a `conforms` / pure-recording ADR):**
```markdown
# ADR-0005: Herald trait signature

## Status
Accepted

**Date:** 2026-07-30

## Context
Two documented Herald trait shapes exist in the ingested corpus (Epic 8 FR-1, infallible; Epic 8
§6.2, fallible). Which one is authoritative for the framework's output-formatting contract?

## Decision
- The shipped trait at `crates/paladin-core/src/platform/container/herald.rs:49` is authoritative.
- It ships the fallible (v2) form: `format_paladin_result`, `format_battalion_result`, and
  `finalize_stream` return `Result<String, HeraldError>`; `format_stream_chunk` returns
  `Result<Option<String>, HeraldError>`; plus `name()` and `mime_type()`.
- `format_error` is deliberately infallible (`-> String`) — this is what makes FR-10's
  graceful-degradation requirement expressible, and the ADR records this as intentional, not an
  inconsistency to be smoothed over.

## Considered Options
- REQ-herald-trait-v1 (Epic 8 FR-1) — infallible `-> String` returns throughout — rejected, not what shipped
- REQ-herald-type-consolidation (run 2) — placeholder-type consolidation — rejected, no placeholder or TODO exists in `herald.rs`

## Code Locations
- `crates/paladin-core/src/platform/container/herald.rs:49-153` — the full trait definition

## Code Conformance
conforms

## Downstream Consumers
- Phase 2 GAP-07 (no action required — nothing to change)
- Any future Herald implementor (`crates/paladin-herald`, custom output formats)
```

### Anti-Patterns to Avoid

- **Writing "Evidence" as a prose paragraph instead of a bulleted `file:line` list.** The parser's
  `splitEntries` only produces multiple structured entries from bullet/numbered lines; a paragraph
  becomes one opaque blob. Since the whole point of D-19's evidence bar and D-04's "Evidence
  (file:line)" field is precise, checkable citations, bullet them even in the ADR (not just the
  ledger).
- **Re-deriving RECON-02/03/04/06's verdicts from scratch.** They are already verified against the
  tree (in CONTEXT.md and independently re-confirmed in this research pass). Re-investigating them
  wastes phase budget that RECON-05 (design work) and RECON-07 (an actual command execution) need.
- **Treating the 39-item ledger count as the ledger's total row count.** RECON-01 asks the ledger to
  classify *every outstanding Milestone-1 task item* — the 39 open checkboxes are that set — but
  D-18 nests them under `REQ-*` rows, and Milestone 1 has ~115 `REQ-*` IDs total (most already
  `Shipped`). Do not conflate "39 rows" with "the size of the ledger."
- **Running `cargo llvm-cov` with default (non-workspace) scope.** `cargo llvm-cov` alone only
  covers the root crate; D-08 requires `--workspace` explicitly, matching what
  `integration-tests.yml` does with `--features integration-tests`. Decide in the ADR itself
  (per the Claude's Discretion item) whether the RECON-07 measurement includes the
  `integration-tests` feature or is a plain `--workspace` run with no extra features — these will
  produce different numbers and the ADR must say which one it used.

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| Parsing/validating ADR structure | A custom ADR linter or frontmatter schema | `.claude/gsd-core/bin/lib/adr-parser.cjs`'s existing heading-synonym scheme (author headings to match it) | It already exists in this repo, is already wired into GSD's "ADR Ingest Express Path," and a bespoke format would be invisible to it |
| Tracking "next ADR number" | A new counter file with novel format | A one-line index in `.planning/decisions/PROMOTION.md` (already needed for D-05's promotion procedure) — reuse that file rather than inventing a second index | D-05 already requires this file to exist; folding the numbering pointer into it avoids a second small file five phases have to remember to update |
| Coverage measurement | A custom `cargo test`-output line-counting script | `cargo llvm-cov --workspace` | Already the project's chosen tool (CI-verified); a hand-rolled counter would produce a number nobody else's tooling agrees with |
| Verifying `file:line` citations | Trusting CONTEXT.md's citations without re-checking | `grep -n` / direct `Read` against `release/v0.7.0` before every ADR/ledger row is finalized | This project's own precedence rule puts the shipped tree first for a reason — three ingest runs already found checkbox and prose claims wrong in both directions (STATE.md) |

**Key insight:** This phase's entire deliverable is "citations that hold up," so the only thing
worth building custom is the discipline of re-checking every citation against the tree at write
time — not tooling. Everything else (parsing conventions, coverage measurement, numbering schemes)
already has an answer either in this repo's tooling or in the project's own CI.

## Runtime State Inventory

Not applicable — this phase renames nothing at the string/identifier level that has runtime state
implications on its own (the `citadel.rs` struct rename is *recorded* here but *executed* in
Phase 2's GAP-07, which is where a runtime-state check for that specific rename belongs). No stored
data, live service config, OS-registered state, secrets, or build artifacts are touched by writing
ADRs and a ledger.

- **Stored data:** None — no data migration in this phase (D-12 explicitly keeps the persisted
  `BattalionState.config` schema unchanged).
- **Live service config:** None — no CI, Docker, or Kubernetes config is touched.
- **OS-registered state:** None.
- **Secrets/env vars:** None.
- **Build artifacts:** None — `cargo llvm-cov` output (`.lcov` files, HTML reports) is not
  committed; only the resulting percentage figure and command are recorded in the RECON-07 ADR.

## Common Pitfalls

### Pitfall 1: Trusting a `file:line` citation without re-verifying line numbers

**What goes wrong:** Line numbers drift as files are edited; a citation accurate at ingest time can
be off by a few lines by the time the ADR is written weeks later.
**Why it happens:** The corpus's own history — STATE.md documents run-3's discovery that checkbox
claims were wrong in both directions, and D-19 exists specifically because "the code exists" has
already produced false-positive completions.
**How to avoid:** Re-run the grep/read for every citation immediately before finalizing each ADR or
ledger row, not once at research time. This research pass re-verified all six D-12–D-16 citations
(see §Summary) as of 2026-07-30 — the planner should re-verify again at execution time if any
material time has passed, since the six ADRs and the ledger may be written across multiple plans.
**Warning signs:** A citation that looks suspiciously round (e.g., always `:XX0` or matching a
remembered number rather than a freshly grepped one).

### Pitfall 2: Recording RECON-07's number without pinning the measurement scope

**What goes wrong:** "80%" or "76%" without stating whether doctests, `examples/`, `benches/`, and
the `doc-examples` crate are included is not reproducible — Phase 15's CI gate would then measure
something different and disagree with the recorded number for reasons nobody can diagnose.
**Why it happens:** `cargo llvm-cov` has several scope-affecting flags (`--workspace`, `--doctests`,
`--all-features` vs default features, `--ignore-filename-regex`) and the corpus's own coverage
history (four/five/six documented positions, none reproducible) is a direct demonstration of what
happens when scope is left implicit.
**How to avoid:** D-09's "Claude's Discretion" item explicitly requires pinning coverage measurement
exclusions inside the RECON-07 ADR itself. Write the exact command used (including all flags) as a
first-class field in that ADR, not just the resulting percentage.
**Warning signs:** An ADR that states a percentage but not the command that produced it.

### Pitfall 3: Treating "89% complete" epic status as "no outstanding items"

**What goes wrong:** Several epics (10, in particular) show 100%/103-of-103 in one document while a
different document says one task remains — this project's own live example (RECON-08's Epic 10
Task 7.0 dispute). Ledger rows must be built from the tree and cross-checked documents, not from a
single "percent complete" figure.
**Why it happens:** `.project/Milestone_1-MVP/Epic_10/tasks-epic10-validation-documentation.md`
marks all 103 checklist items complete across parent tasks 0.0-6.0 and contains **no Task 7.0** at
all, while `.project/Milestone_1-MVP/Epic_10/task6.0-validation-report.md` states "101 of 102
subtasks (99%)" and "Only Task 7.0 (Final Documentation Review) remains" — naming a 6-subtask
Task 7.0 that does not exist in the task list [VERIFIED: `.planning/INGEST-CONFLICTS.md:125-127`].
Both documents are DOC-precedence, so mechanical precedence cannot resolve which is right — this is
exactly why RECON-08 exists as a phase requirement rather than being auto-resolved.
**How to avoid:** For RECON-08, the ledger row must explicitly state: (a) the task list itself has
no Task 7.0 and all 103 of its own items are checked; (b) the validation report's claim of a
6-subtask Task 7.0 is either corroborated by finding that work elsewhere in the tree (making the
validation report right and the task list incomplete-by-omission) or the validation report is
recorded as wrong. This research did not locate a Task 7.0 anywhere in the shipped tree or in any
other document — the planner should search specifically for "Final Documentation Review" content
(e.g., a dedicated final-review doc, sign-off, or checklist) before concluding either way, since
absence-of-evidence here is exactly what RECON-08 asks to be resolved rather than left open.
**Warning signs:** A ledger row that just repeats "103/103" or "disputed" without stating which of
the two documents' claims is corroborated by the tree.

### Pitfall 4: Coverage tool unavailable at execution time

**What goes wrong:** A plan step assumes `cargo llvm-cov --workspace` runs immediately and blocks
on tool installation or network access.
**Why it happens:** Verified in this research sandbox (2026-07-30): `cargo llvm-cov` is **not
installed** (`cargo llvm-cov --version` → `error: no such command`), `llvm-tools-preview` **is**
already present via `rustup component list --installed`, and a direct request to `https://crates.io`
returned **HTTP 403** from this environment, suggesting outbound network access to the crates.io
registry is restricted here. `cargo-tarpaulin` and `grcov` are also absent.
**How to avoid:** See §Environment Availability below for the concrete fallback options; the
planner should not assume this command is a zero-friction one-liner in every execution environment.
**Warning signs:** A plan with no fallback path if `cargo install cargo-llvm-cov` fails.

## Code Examples

### Verifying a `file:line` citation before writing it into an ADR (the discipline this phase runs on)

```bash
# Example: re-confirming D-14's Formation/Commander contradiction before writing RECON-04's ADR
grep -n "at least 2\|requires at least" \
  crates/paladin-core/src/platform/container/battalion/formation.rs
# → 111:                "Formation requires at least 2 Paladins, got {}",

grep -n "test_auto_selects_formation_for_single_paladin" -A 15 \
  crates/paladin-battalion/src/commander.rs
# → 1912: fn test_auto_selects_formation_for_single_paladin() { ... }
#         asserts strategy == BattalionStrategy::Formation for a single Paladin
```
Both citations reconfirmed exactly as CONTEXT.md states, on 2026-07-30. [VERIFIED: direct read
of `release/v0.7.0` tree during this research pass]

### Reading a task list's literal open items (not just the count)

`intel/task-completion-state.md` only gives per-file open **counts**; the ledger needs the actual
item text. Example for one of the 8 files with open items:

```bash
grep -n "^- \[ \]" ".project/Milestone_1-MVP/Epic_4/tasks-battalion-orchestration.md"
# → 258:- [ ] 6.0 Implement Chain of Command Pattern (Phase 2 - Hierarchical Delegation)
# → 302:- [ ] 7.0 Integration Testing, Performance Validation & Documentation
```
Note both open items are **parent-task** checkboxes, not leaf subtasks — a pattern that repeated
across the other 7 files checked (Epic 8's sole open item is also a bare `7.0` parent task). This
is the shape D-19's `present, unproven` bucket is designed to catch: parent checkbox unchecked while
the underlying capability may already be shipped and tested (per RECON-08's Epic 10 precedent,
verify each such row against the tree rather than assuming the parent checkbox is accurate either
way).

### The 8 files with Milestone-1 open items (full RECON-01 ledger scope, from `intel/task-completion-state.md`)

```
Epic_6/tasks-provider-expansion.md              -- 19 open
Epic_2/tasks-garrison-memory-system.md          -- 4 open
Epic_5/tasks-commander-strategy-router.md       -- 4 open
Epic_3/tasks-arsenal-tool-system.md             -- 3 open
Epic_9/tasks-armory-cli-tools.md                -- 3 open
Epic_4/tasks-battalion-orchestration.md         -- 2 open
Epic_8/tasks-herald-output-formatting.md        -- 2 open
unit-test-improvements/tasks-improve-unit-test-coverage.md -- 2 open
                                                 -- 39 total
```
[VERIFIED: `.planning/intel/task-completion-state.md`, deterministic GFM-checkbox count, not an
LLM classification]

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| Task-list checkbox state as the source of truth for "is this done" | `file:line`-cited ledger with a named passing test/example/command as the evidence bar (D-19) | This phase (Phase 1) | Checkbox state drops to lowest precedence tier; RECON-01/08 exist because checkbox state has been wrong in both directions across three prior ingest runs |
| Zero locked decisions across 554 ingested requirements (0 ADR-typed documents in the whole corpus) | ADRs at the **top** of the precedence order (D-02), authored directly in `.planning/decisions/` | This phase (Phase 1) | First protected decisions this project has ever recorded; PROJECT.md's "Key Decisions" table gets its first non-empty rows |
| `cargo tarpaulin` as the documented local coverage tool (`codebase/TESTING.md`) | `cargo llvm-cov` as the tool of record (already used in CI) | Already true in CI before this phase; this phase makes it official in an ADR | RECON-07 should not re-introduce tarpaulin numbers; `codebase/TESTING.md`'s coverage section is itself now slightly stale and could be flagged (not required by any RECON-* ID, optional cleanup) |

**Deprecated/outdated:**
- The four-plus documented coverage-gate positions (80% / 85% / 75%-layered / 80%-Epic-24 /
  78%-hard / 70-74-78%-phased) are all superseded by D-09's single re-measured number once
  RECON-07's ADR is written.

## Assumptions Log

| # | Claim | Section | Risk if Wrong |
|---|-------|---------|---------------|
| A1 | `cargo-llvm-cov` version 0.8.7 is current as of research date | Standard Stack | Low — the exact patch version does not affect the coverage percentage produced; only the tool identity (llvm-cov vs tarpaulin) matters for D-08, and that is verified against the project's own CI, not this version number |
| A2 | Outbound network to crates.io is blocked specifically in *this research sandbox*, not necessarily in the environment that will execute the plan | Environment Availability, Pitfall 4 | Medium — if the execution environment also lacks network access, the RECON-07 coverage-measurement task needs a different fallback (CI-triggered run, or ask a human to run it and report the number back) |
| A3 | No Task 7.0 content exists anywhere in the shipped tree or other `.project/` documents for Epic 10 | Pitfall 3 | Medium — this research did a documents-level check (INGEST-CONFLICTS.md) but did not exhaustively grep the entire `.project/Milestone_1-MVP/Epic_10/` directory tree for a possibly-misnamed final-review artifact; the planner should do that grep before concluding the validation report is simply wrong |

## Open Questions (RESOLVED)

Both questions below were carried into planning and are settled by the phase plans. Each retains its
original research framing (what we know / what's unclear / recommendation) followed by a `RESOLVED:`
marker naming the plan and task that settles it. No question in this section is still open.

1. **Does the RECON-07 coverage command include `--features integration-tests` or run with default
   features only?** — **RESOLVED**
   - What we know: `integration-tests.yml:117-118` runs `cargo llvm-cov --features
     integration-tests --lcov --output-path integration-lcov.info` — a *feature-scoped* run, not a
     bare `--workspace` default-features run. D-08 says "one workspace-wide line-coverage number,
     all tests combined, from one reproducible command (`cargo llvm-cov --workspace`)" without
     specifying feature flags.
   - What's unclear: whether "all tests combined" means literally reproducing the CI invocation
     (with `--features integration-tests`) or a plainer `--workspace` run that a contributor without
     Docker services running could also reproduce locally.
   - Recommendation: Reproduce the CI invocation's feature scope (`--features integration-tests`)
     since that is the only coverage-generating command this project already has evidence of
     working, and record the exact flags used in the ADR per Pitfall 2. If Docker services (Redis,
     MinIO) are unavailable wherever this command runs, note that as a caveat on the recorded number.
   - **RESOLVED: feature-scoped, reproducing the CI invocation — settled in plan `01-04` Task 1**
     ("Measure workspace coverage against the current tree and record the raw evidence"). That task
     pins the scope before the command runs: `--workspace` is mandatory per D-08 (a bare
     `cargo llvm-cov` covers only the root crate), and the run reproduces the CI feature scope
     `--features integration-tests` because that is the only coverage-generating command this project
     has evidence of working. It further pins what this research left unstated: `examples/`,
     `benches/` and the `doc-examples` crate are excluded from the denominator via
     `--ignore-filename-regex` (exact regex recorded), doctests are excluded (no `--doctests`), and
     `--summary-only` is used so no `.lcov`/`.profraw`/HTML artifact is committed. The Docker-services
     caveat this recommendation asked for is mandatory rather than optional: if Redis/MinIO are
     unavailable, the task records that as an explicit caveat on the figure rather than silently
     reporting a lower number. The full command line, toolchain versions, date, commit SHA and
     verbatim tool output land in `01-coverage-measurement.md`, which the ADR then transcribes.

2. **Where does Epic 10's Task 7.0 dispute actually resolve?** — **RESOLVED**
   - What we know: the task list has no Task 7.0 and all 103 of its own items are checked; the
     validation report claims a 6-subtask Task 7.0 remains, under the name "Final Documentation
     Review."
   - What's unclear: whether any artifact in `.project/Milestone_1-MVP/Epic_10/` or elsewhere in the
     shipped `docs/` tree corresponds to that "Final Documentation Review" content, which would mean
     the validation report is right and the task list is incomplete-by-omission (missing a task
     entry for real work), versus no such artifact existing, which would mean the validation
     report's claim is simply wrong.
   - Recommendation: `ls .project/Milestone_1-MVP/Epic_10/` and grep the `docs/` tree for anything
     resembling a documentation-review sign-off before writing the RECON-08 ledger row; this
     research located the conflict but did not do that exhaustive local search.
   - **RESOLVED: the search is an executed step, not a research finding — settled in plan `01-05`
     Task 1** ("Resolve the Epic 10 Task 7.0 dispute and the 102-vs-103 discrepancy"). That task
     performs exactly the exhaustive search this recommendation asked for, and does so *before*
     writing the verdict: it lists the full contents of `.project/Milestone_1-MVP/Epic_10/`, greps
     that directory, the rest of `.project/Milestone_1-MVP/`, and the `docs/` tree for the literal
     phrase "Final Documentation Review" plus the looser terms a renamed artifact would carry, and
     also greps for the six subtask descriptions the validation report attributes to Task 7.0 in case
     the work shipped under a different heading. Every command run and its result is recorded in the
     ledger, because here the absence of a result *is* the evidence and an unrecorded search is not
     evidence. The task then forces exactly one of two named verdicts with no third hedged option —
     either "the Final Documentation Review is outstanding work" (row classified `genuinely
     outstanding`, with a named owning phase or requirement) or "the validation report is recorded as
     wrong" (row classified `satisfied`, task list corroborated) — and requires both the 102 and 103
     totals to be stated verbatim against their sources with the arithmetic difference explained,
     rather than one number being picked and the other dropped.
   - Note on scope: this research pass deliberately does **not** pre-judge which of the two verdicts
     is correct; the evidence needed to choose is a local filesystem search, which is execution work.
     What is resolved here is *where and how* the question gets answered, and that the answer cannot
     be left open — plan `01-05` Task 1's acceptance criteria fail unless a verdict and its search
     record are both present in `.planning/ledgers/milestone-01.md`. This closes Assumption A3
     ("No Task 7.0 content exists anywhere in the shipped tree or other `.project/` documents for
     Epic 10"), which the Assumptions Log rates Medium risk precisely because that grep had not run.

## Environment Availability

| Dependency | Required By | Available | Version | Fallback |
|------------|------------|-----------|---------|----------|
| `cargo-llvm-cov` | RECON-07 (D-07 re-measurement) | ✗ (checked 2026-07-30 in this sandbox) | — | See below |
| `llvm-tools-preview` (rustup component) | `cargo-llvm-cov`'s instrumentation backend | ✓ | `llvm-tools-x86_64-unknown-linux-gnu` | — |
| Network access to `crates.io` | Installing `cargo-llvm-cov` | ✗ (HTTP 403 observed from this sandbox) | — | Run in CI (where `integration-tests.yml` already does this successfully) or in an environment confirmed to have registry access, then transcribe the resulting number and exact command into the ADR |
| `cargo-tarpaulin` / `grcov` (alternate coverage tools) | Fallback if `cargo-llvm-cov` truly cannot be installed | ✗ (neither present) | — | Not recommended as a substitute per D-08 discretion note — CI already uses llvm-cov, so a tarpaulin number would not match CI's own gate |
| `rustc` / `cargo` toolchain | Everything else in this phase | ✓ | rustc 1.97.1 / cargo 1.97.1 | — |

**Missing dependencies with no fallback:** none — every gap has a documented fallback below.

**Missing dependencies with fallback:**
- `cargo-llvm-cov` — install via `cargo install cargo-llvm-cov` in a network-enabled environment, or
  trigger the existing `integration-tests.yml` GitHub Actions workflow (which already installs and
  runs it) and read the coverage percentage from its output/artifacts, then hand-transcribe the
  number, exact command, and date into the RECON-07 ADR as D-07 requires. **Do not fabricate or
  estimate a coverage number** — D-07's whole premise is that every existing baseline is stale or
  contested; producing a seventh unverified guess would repeat exactly the problem this phase exists
  to close.

## Validation Architecture

This phase produces no executable product code, so there is no unit/integration test suite to wire
up. "Validation" here means: every factual claim in the six ADRs and the ledger must be independently
checkable against the shipped tree by a third party, using the same commands this research used.

### Test Framework

| Property | Value |
|----------|-------|
| Framework | None (documentation phase) — validation is citation re-verification, not automated tests |
| Config file | n/a |
| Quick run command | `grep -n "<citation text>" <cited file>` — spot-check any single ADR claim |
| Full suite command | Re-run every `grep`/`Read` citation check listed in this RESEARCH.md's Code Examples section, plus `cargo llvm-cov --workspace --features integration-tests` for RECON-07 |

### Phase Requirements → Test Map

| Req ID | Behavior | Test Type | Automated Command | File Exists? |
|--------|----------|-----------|-------------------|-------------|
| RECON-01 | Every outstanding task item is cited and classified | Manual/citation-check | `grep -n "^- \[ \]" .project/Milestone_1-MVP/**/tasks-*.md` against each of the 8 files, cross-checked against the tree | ✅ (source files exist today) |
| RECON-02 | `BattalionConfig` ADR citations hold | Citation-check | `grep -n "pub struct BattalionConfig" -A 20 crates/paladin-core/src/platform/container/battalion/mod.rs crates/paladin-core/src/platform/container/citadel.rs` | ✅ verified in this research pass |
| RECON-03 | `BattalionResult` ADR citations hold | Citation-check | `grep -n "pub struct BattalionResult" -A 20 crates/paladin-core/src/platform/container/battalion/mod.rs` | ✅ (not re-read line-by-line this pass; verified in prior ingest run 3 and unchanged since) |
| RECON-04 | Formation/Commander contradiction citations hold | Citation-check | see §Code Examples above | ✅ verified in this research pass |
| RECON-05 | `ProviderCapabilities` has no temperature field | Citation-check | `sed -n '740,775p' crates/paladin-ports/src/output/llm_port.rs` | ✅ verified in this research pass |
| RECON-06 | `Herald` trait signature citations hold | Citation-check | `sed -n '49,153p' crates/paladin-core/src/platform/container/herald.rs` | ✅ verified in this research pass |
| RECON-07 | Coverage number is freshly measured, not copied from a stale baseline | Command execution | `cargo llvm-cov --workspace --features integration-tests --summary-only` | ❌ Wave 0 — tool not installed in this sandbox, see Environment Availability |
| RECON-08 | Epic 10 Task 7.0 / 102-vs-103 discrepancy documented and resolved | Citation-check + local search | `ls .project/Milestone_1-MVP/Epic_10/`; grep `docs/` for documentation-review artifacts | ⚠️ Wave 0 — conflict located; the exhaustive resolution search is executed by plan `01-05` Task 1, not by this research pass (Open Question 2, RESOLVED) |

### Sampling Rate

- **Per ADR/ledger-row write:** re-run the specific citation's grep/read command before finalizing.
- **Per plan/wave merge:** re-run the full citation-check list in this RESEARCH.md.
- **Phase gate:** every ADR's `Code Locations` claims re-verified once more immediately before
  `/gsd-verify-work`; the RECON-07 coverage number confirmed reproducible from the exact command
  recorded in its ADR.

### Wave 0 Gaps

- [ ] `cargo-llvm-cov` installed somewhere with crates.io access (local, CI trigger, or human-run) —
      blocks RECON-07
- [ ] Exhaustive local search of `.project/Milestone_1-MVP/Epic_10/` and `docs/` for "Final
      Documentation Review" content — blocks a confident RECON-08 verdict (currently: conflict is
      documented, resolution direction is not)
- [ ] Read the literal item text (not just the count) of all 39 open checkboxes across the 8 files
      listed in §Code Examples — needed before RECON-01's ledger rows can be written with accurate
      nested task-item descriptions

## Security Domain

`security_enforcement` has no explicit `false` in `.planning/config.json` (the file does not exist),
so per the workflow default this section is included, but its content is thin because this phase
writes no code that handles input, auth, or cryptography.

### Applicable ASVS Categories

| ASVS Category | Applies | Standard Control |
|---------------|---------|-----------------|
| V2 Authentication | No | This phase touches no auth code |
| V3 Session Management | No | — |
| V4 Access Control | No | — |
| V5 Input Validation | Indirectly | RECON-05's temperature-validation ADR *records* a design (provider-aware range check on `ProviderCapabilities`) but does not implement it — GAP-07 implements, and GAP-07's own research/plan is where V5 controls should be re-checked against the actual Rust validation code written |
| V6 Cryptography | No | — |

### Known Threat Patterns for this stack

None applicable to this phase's actual deliverable (Markdown documents). The one item worth flagging
forward: this research's read of `PROJECT.md` and `STATE.md` surfaced that the corpus's rustsec
risk-acceptance record (2 advisories, expiry 2026-09-30) and the shared-store `AuthPort` question
(WEB-01/WEB-02, opaque-token store not safe under the shipped multi-replica Kubernetes deployment)
are real open security items — but they belong to Phases 9 and 14 respectively, not Phase 1. Noted
here only so the planner does not accidentally fold them in.

## Sources

### Primary (HIGH confidence — direct tree verification during this research session, 2026-07-30)
- `crates/paladin-core/src/platform/container/battalion/mod.rs:37-58` — `BattalionConfig` field set
- `crates/paladin-core/src/platform/container/citadel.rs:270-289` — placeholder `BattalionConfig` duplicate
- `crates/paladin-core/src/platform/container/battalion/formation.rs:108-111` — the ≥2 rejection
- `crates/paladin-battalion/src/commander.rs:1912-1927` — the passing single-Paladin Auto test
- `crates/paladin-ports/src/output/llm_port.rs:753-772` — `ProviderCapabilities`, no temperature field
- `src/application/services/paladin/paladin_builder.rs:1105-1119` — the `[0.0,1.0]` clamp
- `crates/paladin-llm/src/config/llm.rs:14-15` — documented 0.0–2.0 default range
- `crates/paladin-core/src/platform/container/herald.rs:49-153` — full `Herald` trait
- `.claude/gsd-core/bin/lib/adr-parser.cjs` — ADR heading-synonym parser, full read
- `.planning/intel/task-completion-state.md` — deterministic 39-item / 8-file Milestone-1 open-item count
- `.planning/INGEST-CONFLICTS.md:125-127` — Epic 10 Task 7.0 / 102-vs-103 conflict, verbatim
- `.project/Milestone_1-MVP/Epic_4/tasks-battalion-orchestration.md`,
  `.project/Milestone_1-MVP/Epic_8/tasks-herald-output-formatting.md`,
  `.project/Milestone_1-MVP/unit-test-improvements/tasks-improve-unit-test-coverage.md` — sample open-item text
- `.github/workflows/integration-tests.yml:117-123` — confirms `cargo llvm-cov` already CI's tool of record
- `.planning/codebase/TESTING.md` — test framework/coverage conventions (partially stale re: tarpaulin)
- `.planning/codebase/STRUCTURE.md` — crate layout
- `.planning/PROJECT.md:731-734, 1020-1074` — current precedence order and empty Key Decisions table
- `.project/Milestone_5-Workspace-Decomposition/Epic_1/decisions/battalion-result-upward-dependency-decision.md` — the corpus's only prior decision/options-pair document, read for house-style precedent
- `rustup component list --installed`, `cargo llvm-cov --version`, `curl -sI https://crates.io` — this sandbox's tool/network availability, checked 2026-07-30

### Secondary (MEDIUM confidence)
- [cargo-llvm-cov on docs.rs](https://docs.rs/crate/cargo-llvm-cov/latest/source/README.md) — version 0.8.7, checked via WebSearch, cross-referenced against the project's own CI usage

### Tertiary (LOW confidence)
- None — this phase's domain is entirely internal-codebase and required no external/unverified web claims beyond the single version-freshness check above.

## Metadata

**Confidence breakdown:**
- Standard stack (tooling choice): HIGH — `cargo-llvm-cov` is already the project's own CI-verified tool; `adr-parser.cjs` was read in full
- Architecture (the six ADR verdicts): HIGH — all six independently re-verified against the shipped tree in this research session, not just carried over from CONTEXT.md
- Pitfalls: HIGH — three of four pitfalls are drawn from this project's own documented history (STATE.md's ingest-run findings); the fourth (coverage tool absence) is a direct, reproducible observation in this sandbox
- RECON-07/RECON-08 remaining work: MEDIUM — the scope of what's needed is clear, but the actual coverage number and the Epic 10 resolution direction are not yet obtained (documented as Open Questions and Wave 0 gaps, not guessed at)

**Research date:** 2026-07-30
**Valid until:** 14 days — this research is tied to `release/v0.7.0`'s exact line numbers; any commit
that touches `battalion/mod.rs`, `formation.rs`, `commander.rs`, `herald.rs`, `llm_port.rs`,
`paladin_builder.rs`, or `citadel.rs` before the plan executes should trigger a re-verification pass
using the same grep commands in §Code Examples.
