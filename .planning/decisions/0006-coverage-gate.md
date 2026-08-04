# ADR-0006: Project-wide test coverage gate

## Status

Accepted

**Date:** 2026-07-31

## Context

The coverage question needs exactly one number and one scope, recorded so a later phase (Phase 3's
QUAL-01) can objectively pass or fail against it rather than choosing between competing targets.
**Six positions on the coverage gate exist across the corpus**, none of them settled by shipped
code (no coverage gate exists in `ci.yml` at all):

1. **80% unit / 70% integration** — `REQ-test-coverage-target-v1`, nine Milestone-1 Epic PRDs
   (Epics 1-8, 10), measured via `cargo-llvm-cov`.
2. **85% overall, functions under 50% must reach 80%** — `REQ-test-coverage-target-v2`, the
   `unit-test-improvements` workstream, stated baseline 67.79%.
3. **75% overall with a layered per-tier table** — the Milestone 3 `Project_Plan` "Cross-Cutting
   Concerns" section (run-2 third position, not a `REQ-*`-carrying entry): core domain ≥ 85%,
   application services ≥ 80%, infrastructure adapters ≥ 70%, CLI commands ≥ 70%, overall ≥ 75%.
4. **80% all modules / 70% integration, re-asserted** — `REQ-epic24-quality-gates` (Epic 24).
5. **78% hard gate** — the Deferred-QA parent PRD, `prd-deferred-qa-completion.md` FR-25.3 item 10:
   "Configure a coverage threshold gate of 78% minimum. PRs dropping below this threshold must
   fail."
6. **A phased 70% → 74% → 78% ramp** — Epic 25, `prd-cicd-pipeline-enhancement.md` FR-25.6 and
   Appendix C, patch target 80% throughout, one `target:` edit per phase.

The parent PRD's own **Open Question 3** — "Should the coverage threshold gate be a hard fail
(block merge) or a soft warning initially? Moving from no gate to 78% hard fail could block
legitimate PRs during ramp-up" — is recorded Open in the corpus. **It is answered here: hard-fail,
from the first run**, by a construction (below) that removes the ramp-up risk the question worried
about.

Every previously measured baseline in the corpus is stale or contested: **60.88% unit / 67.79%
integration** (Milestone 1, predating Milestones 2-12); **~78% overall** (Milestone 3 release
notes); **76-77%** (Deferred-QA, February 2026, with two known-stale module paths). None of the
six positions above was checked against a fresh measurement of the current tree before this ADR.

**Measurement provenance.** Plan 01-04's original measurement path was blocked in this environment
— no `cargo-llvm-cov` installed, crates.io returning HTTP 403, and no Docker available to run the
Redis-/MinIO-backed integration suite. The figure recorded below was produced instead by the
offline `rustc` source-based instrumentation path plan 01-09 proved (`RUSTFLAGS="-C
instrument-coverage"` plus `llvm-profdata`/`llvm-cov` directly, no crates.io fetch required) — this
is why the command below differs in shape from `.github/workflows/integration-tests.yml`'s
`cargo llvm-cov` invocation even though both ultimately drive the same LLVM instrumentation.

## Decision

- **The measured figure.** **84.79%** workspace line coverage — 61,404 first-party lines counted,
  9,340 missed — measured **2026-07-31T14:57:11Z (UTC)** against commit
  `9be788c8e9c744ec3a6aad20b64110fb85925de4`. This figure is transcribed byte-identical from
  `01-coverage-measurement.md`'s TOTAL row, never re-typed or rounded. (Region coverage 87.33%;
  function coverage 77.34% — see the module-scoped gates and the function/line gap note below.)

- **The exact command**, verbatim, including every flag and env var:

  ```
  RUSTFLAGS="-C instrument-coverage" \
  LLVM_PROFILE_FILE="$PWD/target/coverage/paladin-%p-%m.profraw" \
  cargo test --workspace --offline
  ```

  followed by `llvm-profdata merge -sparse target/coverage/*.profraw -o
  target/coverage/paladin.profdata` and

  ```
  llvm-cov report --instr-profile=target/coverage/paladin.profdata \
    --ignore-filename-regex='(^|/)(examples|benches)/|crates/doc-examples/|registry/src/|rustlib/src/' \
    --object=<each of 31 discovered test-binary objects>
  ```

  `--workspace` is mandatory per D-08 — a bare `cargo test` covers only the root crate.
  `--ignore-filename-regex` removes, in words: any `examples/` or `benches/` directory anywhere in
  the tree; the `crates/doc-examples/` crate; third-party dependency source under the cargo
  registry; and the Rust standard library's own bundled source. Only first-party workspace source
  (`crates/*/src`, `src/`, and this workspace's own `tests/`) remains in the denominator. **Doctests
  are excluded** — no `--doctests` pass was run, and object discovery selects only
  `.profile.test == true` unit/integration-test binaries, never the doctest harness.

- **The scope.** Per the human decision at Task 1's checkpoint (option-a): **the locally-measured
  workspace default-feature scope is recorded as binding.** This is one workspace-wide
  line-coverage number from one reproducible command — not per-tier, not per-crate. The
  Docker-backed `--features integration-tests` scope that `.github/workflows/integration-tests.yml`
  runs **could not execute in this environment** — Docker is entirely absent, so the Redis- and
  MinIO-backed suites never ran and that feature was never passed. This narrowing is stated
  plainly, not glossed: the recorded 84.79% is not expected to equal what CI reports today.
  **VERIFY-05** (Phase 5) and **PIPE-02** (Phase 15) are the requirements that must extend this
  scope to the Docker-backed suites; the narrowing is scheduled work, not lost work.
  `examples/`, `benches/` and the `crates/doc-examples` crate stay excluded from the denominator,
  and doctests stay excluded — both confirmed as pinned by the human at the same checkpoint.

- **The relationship to `cargo llvm-cov`.** `.github/workflows/integration-tests.yml:117-118`
  installs and runs `cargo llvm-cov`, which is a wrapper over the same LLVM source-based
  instrumentation this measurement drove directly with `rustc`/`llvm-profdata`/`llvm-cov`. The two
  commands' denominators are expected to agree **only** when the ignore regex, the doctest
  decision, and the feature set all match. They do not fully match today (CI additionally runs
  `--features integration-tests`). **PIPE-02 must either reproduce those three or record why its
  figure differs** — the two commands are never assumed equivalent by silence.

- **The gate (floor): 84%, hard-fail from the first run.** Arithmetic, shown explicitly on the
  actual measured figure: **measured 84.79%, truncated down to a whole percent → floor 84%.** The
  rounding rule is truncation toward zero — explicitly neither round-half-up nor round-half-even —
  and a measurement already on a whole percent floors to itself. The comparison is **at-or-above**:
  a later run measuring exactly 84% **passes**; a run measuring 83.99% **fails**. Because the floor
  is the measured figure truncated downward, the gate cannot be red on the run that set it — this
  is what answers Open Question 3 as hard-fail without the ramp-up problem it worried about.

- **The target: superseded. There is no operative target — this is a deviation from D-09, recorded
  as one.** D-09 specified "80% recorded in the same ADR as the target" alongside the floor. D-09
  was written when every candidate baseline (60.88%, 67.79%, ~76-78%) sat **below** 80%, so
  `floor < target` was meaningful and Phase 3 raising real coverage toward 80% was real work. The
  actual measurement is **84.79%**, so the floor this ADR records (84%) is already **above** 80%.
  Recording 80% as a target alongside an 84% floor would ship a self-contradictory ADR — a target
  the tree has already cleared before the ADR recording it is even written — and would hand Phase
  15 two numbers to reconcile, which is the exact "80 vs 85" failure RECON-07 exists to eliminate.
  **This ADR therefore retires 80% as a superseded historical aspiration**, not a live number: the
  tree has already overtaken it. Exactly one number — **84%** — is binding, and it is what PIPE-02
  wires into CI. This deviation is D-09's *intent* honored (a floor derived from the real
  measurement that cannot be red on day one) with its now-stale literal 80%-as-target clause set
  aside; D-09 did not anticipate a baseline this far above 80%, and this ADR does not pretend it
  did.

- **The ratchet trigger.** The floor is raised to the next whole percent below the then-current
  measured workspace figure whenever a milestone closes with measured coverage two or more whole
  percentage points above the standing floor. The raise is applied by amending this ADR in place
  with the new figure, command, and date. **Because 80% is superseded rather than live, the ratchet
  has no upper stop as of this ADR** — D-09's literal text specified a stop at 80%, but that stop
  was contingent on the floor sitting below the 80% target, which is no longer true (the floor is
  84%, above 80%). The ratchet continues to raise the floor on future qualifying milestone closes
  with no ceiling recorded here.

- **The two module-scoped gates.** **Herald ≥ 95%** and **autonomous ≥ 90%** sit **above** the
  global 84% floor and are **explicitly not withdrawn** by this ADR. Their placement is handed to
  **VERIFY-05 in Phase 5**, which the roadmap already assigns as their owner. Neither gate currently
  has a recorded measurement (`REQ-herald-consolidation-quality-gates`,
  `REQ-autonomous-completion-quality-gates`) — this measurement's per-file rows show
  `crates/paladin-core/src/platform/container/herald.rs` at 80.49% line coverage today, below its
  95% module target, which VERIFY-05 inherits as live gap-closure work rather than a settled number.

- **Function coverage flag for VERIFY-05.** The same TOTAL row that produced 84.79% line coverage
  also reports **77.34% function coverage** (7,546 functions, 1,710 missed) — about seven points
  below the line figure, and region coverage sits at 87.33%. **A coverage gate expressed only in
  lines does not see that ~7-point gap.** This is flagged here as context for VERIFY-05's
  module-scoped gate work and for any future refinement of this gate; it is not itself gated by
  this ADR, which records a line-coverage floor per D-08's scope decision.

- **The ~24-point gap above the stale Milestone-1 baselines is accepted and noted, not explained.**
  84.79% sits roughly 24 points above the stale 60.88% unit / 67.79% integration Milestone-1
  figures. Those baselines predate the workspace migration (Milestones 2-12) and may have counted a
  different set of paths than this run's `--ignore-filename-regex` admits. This ADR does not
  reconcile against the Milestone-1 figures — the delta is recorded as an observation, not
  re-derived or explained away.

- **The tool-of-record note.** `.planning/codebase/TESTING.md:319-322` documents `cargo tarpaulin`
  as a local habit (`cargo tarpaulin --out Html`, `cargo +nightly tarpaulin --exclude-files
  tests`). This is stale relative to `.github/workflows/integration-tests.yml`, which uses
  `cargo llvm-cov`; a tarpaulin-produced figure would not match this gate and should not be compared
  against it.

- **The consequence for the roadmap.** `ROADMAP.md`'s Phase 3 success criterion 1 names unit
  coverage and integration coverage separately ("`cargo llvm-cov` reports unit coverage at or above
  the gate recorded in Phase 1 (baseline 60.88%) and integration coverage at or above 70% (baseline
  67.79%)"). It must be amended to cite this ADR's single workspace-wide 84% floor instead.
  **Plan 01-12** is where that amendment lands.

## Phase 5 amendment (2026-08-04)

**(Amended by Phase 5, dated 2026-08-04, citing `REQUIREMENTS.md`'s VERIFY-05 and
`05-CONTEXT.md` D-12–D-16):** VERIFY-05 extends this same ADR rather than writing a second one —
this ADR's own ratchet clause already specifies in-place amendment, D-00g makes it the house
convention, and RECON-07 exists precisely to eliminate the "choosing between two numbers" failure a
second coverage ADR would recreate. Superseded text elsewhere in this document is retained, not
deleted, per D-00g. **No re-measurement was performed in this phase** — the 84.79% figure measured
2026-07-31T14:57:11Z and recorded above under "The measured figure" stands unchanged (D-16).
**Phase 15 / PIPE-02 remains the enforcement owner** for everything this amendment records: the CI
threshold itself, the two module-scoped gates below, the `run()`-seam prerequisite, and the
`minio.rs` feature-scope question.

The floor arithmetic recorded above under "The gate (floor): 84%, hard-fail from the first run" is
unchanged and is restated here by reference only, not rewritten: the floor is derived by truncating
the measured 84.79% toward zero to a whole percent, the comparison is at-or-above, and the same
worked example already given there (84% clears the floor, 83.99% does not) still holds exactly as
written. No new comparison operator, rounding mode, or precision rule is introduced by this
amendment.

### Module-scoped gate table (D-13)

Both targets below sit **above** the global 84% floor and are **explicitly not withdrawn** — see
"The two module-scoped gates" above, ADR-0006's own standing instruction. Where a module target and
the global floor both apply to the same file, both numbers coexist: the module target is a target
above the floor, never a replacement for it, and **the 84% floor remains the only binding gate**.
This amendment records the number, the scope and the gap for each target — it does not convert
either into a hard CI gate, because Phase 5 has no CI in which to enforce one. Enforcement is
Phase 15 / PIPE-02's.

| Module scope | Target | Measured | Gap | Owner |
|---|---|---|---|---|
| Herald (`REQ-herald-consolidation-quality-gates`) | ≥ 95% | **80.49%** line coverage — `crates/paladin-core/src/platform/container/herald.rs`, transcribed from `01-coverage-measurement.md:317` | ~14.5 points below target | Phase 15 / PIPE-02 |
| Autonomous components (`REQ-autonomous-completion-quality-gates`) | ≥ 90% | **92.80%** line-weighted aggregate across the four autonomous services, transcribed from `01-coverage-measurement.md`: `planning_service.rs:421` 577 lines / 43 missed / 92.55%, `prompt_generation_service.rs:422` 234 lines / 26 missed / 88.89%, `temperature_service.rs:423` 394 lines / 23 missed / 94.16%, `handoff_service.rs:418` 239 lines / 12 missed / 94.98% — aggregate `(1444 - 104) / 1444 = 92.80%` | Aggregate clears the target by ~2.8 points; `prompt_generation_service.rs` individually sits ~1.1 points below it | Phase 15 / PIPE-02 |

### Inherited dispositions from the v0.7.1 close-out (D-14a, D-14b)

- **`src/bin/paladin-server.rs` — deferred with reason.** Measured at **0.00%** line coverage (185
  regions / 13 functions / 145 lines, all missed — transcribed from `01-coverage-measurement.md:426`).
  Closing it requires extracting a testable `run()` seam from `#[tokio::main] async fn main()`, which
  is a code change and therefore outside this phase's boundary — this plan records the disposition
  only and edits no `.rs` file. The seam extraction is named as the concrete prerequisite so
  Phase 15 inherits a task, not a puzzle. A 0%-coverage binary is not allowed to sit silently in the
  denominator without a record. **Owner: Phase 15 / PIPE-02.**

- **`minio.rs` — outside the gated denominator by construction.** ADR-0006's scope stays
  default-feature workspace-wide (see "The scope" above); `minio.rs` sits behind the non-default
  `s3-storage` feature, so it does not appear in the 84.79% measurement's denominator at all — it is
  outside scope by construction, not a 0%-coverage file inside scope. Widening the denominator to
  non-default features in Phase 5 would move the 84% floor with no measurement behind it, which is
  exactly what this ADR forbids. Whether to add a second, feature-scoped measurement is
  **Phase 15 / PIPE-02's** decision — RECON-07's one-number-one-scope rule stays intact until then.

### The ~78% Milestone-3 figure (D-15)

The ~78% overall figure reported in `RELEASE_NOTES_MILESTONE_3.md` **fails** the 84% floor and
**predates** the measurement that set it — a stale historical figure, not a competing current one.
It is accepted and noted here, not explained away or reconciled, in the same shape "The ~24-point gap
above the stale Milestone-1 baselines is accepted and noted, not explained" above already gives the
60.88% / 67.79% Milestone-1 baselines.

## Considered Options

- `REQ-test-coverage-target-v1` (80% unit / 70% integration, nine Milestone-1 PRDs) — rejected; the
  measured 84.79% baseline already clears this figure, and D-08's scope decision replaces the
  unit/integration split with one workspace-wide number.
- `REQ-test-coverage-target-v2` (85% overall, unit-test-improvements workstream) — rejected; 85%
  exceeds the measured 84.79% baseline and would leave the gate red on the run that set it,
  violating the no-red-on-day-one construction this ADR requires.
- Run-2 third position (75% overall, layered per-tier table, Milestone 3 plan) — rejected; RECON-07
  asks for one workspace-wide number, not a per-tier table with four separate targets.
- `REQ-epic24-quality-gates` (80% all modules / 70% integration, re-asserted) — rejected; re-asserts
  the same immediate-80%-hard-gate shape D-09 already rejected (would have left Phase 15's CI gate
  red until Phase 3 landed), and is now moot since measured coverage already exceeds it.
- Deferred-QA parent PRD FR-25.3 item 10 (78% hard gate) — rejected; sits below the measured 84.79%
  baseline, and answering Open Question 3 with an immediate 78% hard gate reintroduces exactly the
  ramp-up risk that question raised.
- Epic 25 FR-25.6 phased 70% → 74% → 78% ramp — rejected, per D-09 explicitly: three numbers where
  RECON-07 asked for one.
- **option-b** (record a CI-produced figure under the full `--features integration-tests` scope) —
  rejected by the human at Task 1's checkpoint; requires an actual CI run to exist or be triggered
  and its output read, which is not reproducible or re-checkable locally in this environment and
  would block phase closure on an external run.
- **option-c** (record both a local number and a CI number) — rejected by the human at Task 1's
  checkpoint; reproduces the precise failure RECON-07 exists to close — two numbers restores the
  "choosing between 80% and 85%" problem the phase goal explicitly rejects.
- An immediate 80% hard gate applied literally as D-09 first framed it — rejected as moot; the
  measured baseline (84.79%) already exceeds 80%, so an 80% floor would under-gate relative to what
  the tree already proves.
- Retaining 80% as a recorded target alongside the 84% floor (D-09's literal text) — rejected as
  this ADR's deviation from D-09; a floor above its own target is self-contradictory and would hand
  Phase 15 two numbers instead of RECON-07's required one. Recorded here rather than silently
  dropped, per the deviation protocol.
- **(Added by Phase 5, dated 2026-08-04, VERIFY-05/D-12)** Milestone 3 plan's layered per-tier table
  (run 2) — 75% overall (core ≥ 85%, application ≥ 80%, infrastructure ≥ 70%, CLI ≥ 70%) — rejected
  against the measured 84.79%: 75% sits ten points below the measured baseline and would under-gate
  what the tree already achieves, and a four-tier table introduces four numbers where RECON-07
  requires one. See the existing rejection above ("Run-2 third position (75% overall, layered
  per-tier table, Milestone 3 plan) — rejected; RECON-07 asks for one workspace-wide number, not a
  per-tier table with four separate targets.") for the RECON-07 argument in full; this bullet adds
  the numeric under-gating comparison against the measured figure.
- **(Added by Phase 5, dated 2026-08-04, VERIFY-05/D-12)** Epic 24's re-assertion of ≥ 80% / ≥ 70%
  (run 2) — rejected against the measured 84.79% for the same under-gating reason as the existing
  rejection above ("`REQ-epic24-quality-gates` (80% all modules / 70% integration, re-asserted) —
  rejected; re-asserts the same immediate-80%-hard-gate shape D-09 already rejected ... and is now
  moot since measured coverage already exceeds it."); cross-referenced rather than restated.

## Code Locations

- `.github/workflows/integration-tests.yml:117-118` — the existing CI coverage invocation
  (`cargo install cargo-llvm-cov` / `cargo llvm-cov --features integration-tests --lcov
  --output-path integration-lcov.info`), the scope this ADR's recorded figure does not reproduce.
- `.planning/phases/01-ground-truth-decision-records/01-coverage-measurement.md` — the measurement
  record: toolchain versions, verbatim commands, the full `llvm-cov report` TOTAL row, and the
  human confirmation this ADR's figure is transcribed from.
- `.planning/codebase/TESTING.md:319-322` — the superseded local `cargo tarpaulin` reference.

## Code Conformance

must change

No coverage gate currently exists in `.github/workflows/ci.yml` — there is no `coverage` job and no
`llvm-cov`/`codecov` reference in it at all (`REQ-ci-combined-coverage-job`, verified open). The
only coverage tooling in the tree is the integration-only, `continue-on-error: true` step in
`integration-tests.yml`, which does not gate anything. This is recorded as pending work, not
smoothed into conformance: **PIPE-02** in Phase 15 is the requirement that wires this ADR's 84%
floor into CI either way.

## Downstream Consumers

- **Phase 3 (QUAL-01)** — raises real coverage against this floor; Phase 3's success criterion 1
  must be amended (plan 01-12) to cite this ADR's single 84% number instead of the separate
  unit/integration figures it currently names.
- **Phase 5 (VERIFY-05)** — owns the two module-scoped gates (Herald ≥ 95%, autonomous ≥ 90%),
  extends this ADR's number and scope across the four earlier corpus positions, and inherits the
  function-vs-line coverage gap (77.34% vs 84.79%) flagged above.
- **Phase 15 (PIPE-02)** — must land the 84% CI threshold on this number, or record why its figure
  differs; must also extend the recorded scope to the Docker-backed `integration-tests.yml` suite
  this ADR could not reach.
- **Phase 15 (PIPE-02) — extended by this Phase 5 amendment.** In addition to the CI threshold and
  Docker-backed-scope extension recorded above, PIPE-02 now also receives: the `run()`-seam
  extraction from `src/bin/paladin-server.rs` named as a concrete prerequisite (D-14a); the
  `minio.rs` feature-scoped-measurement decision (D-14b); and enforcement of both module-scoped
  gates in the table above (D-13).
- **Phase 5 ledger plans 05-11 and 05-12** — cite this amendment's transcribed Herald (80.49%) and
  autonomous-components (92.80% aggregate) figures on the `REQ-herald-consolidation-quality-gates`
  and `REQ-autonomous-completion-quality-gates` ledger rows respectively, rather than restating an
  unmeasured target.
