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

## Phase 8 amendment (2026-08-06)

**(Amended by Phase 8, dated 2026-08-06, citing plan 08-09's SUMMARY):** the 84% floor is
re-measured against the Phase 8 close-out tree using this ADR's own pipeline, verbatim, with the
absolute rustup LLVM tool paths (`cargo-llvm-cov` remains uninstalled and crates.io still returns
HTTP 403 in this environment; **not** attempted, matching this ADR's own tool-of-record note ruling
out `cargo tarpaulin`):

```
RUSTFLAGS="-C instrument-coverage" \
LLVM_PROFILE_FILE="$PWD/target/coverage/paladin-%p-%m.profraw" \
cargo test --workspace --offline
```

Result: exit 0, 3013 passed / 0 failed across every `test result: ok.` line in the run (matching the
figure carried forward from plan 08-08's SUMMARY at the same tree state).

```
llvm-profdata merge -sparse target/coverage/*.profraw -o target/coverage/paladin.profdata
```

Exit 0, `target/coverage/paladin.profdata` created from 2,321 `.profraw` files.

```
RUSTFLAGS="-C instrument-coverage" cargo test --workspace --no-run --message-format=json --offline \
  | jq -r 'select(.profile.test == true) | .filenames[]' \
  | grep -v '\.dSYM' \
  | sort -u
```

Result: **30** unique test-binary object paths (one fewer than the 31 this ADR's original
measurement discovered — `paladin-herald`'s `table_herald` test module no longer produces a
default-feature test binary of its own significance here; see the accounting below).

```
llvm-cov report --instr-profile=target/coverage/paladin.profdata \
  --ignore-filename-regex='(^|/)(examples|benches)/|crates/doc-examples/|registry/src/|rustlib/src/' \
  --object=<each of the 30 discovered test-binary objects>
```

TOTAL row, transcribed byte-identical:

```
TOTAL   97193   11610   88.05%   7799   1677   78.50%   63999   9059   85.85%   0   0   -
```

(regions / missed regions / region-cover / functions / missed functions / function-cover / lines /
missed lines / line-cover / branches / missed branches / branch-cover, matching this ADR's own
column order.)

**The measured figure: 85.85% workspace line coverage** — 63,999 first-party lines counted, 9,059
missed. **This clears the 84.00% floor by 1.85 points.** Region coverage 88.05%; function coverage
78.50% (the same ~7-9-point function/line gap this ADR already flagged for VERIFY-05, unchanged in
kind).

**Delta from the last recorded measurement:** 85.92% at HEAD `1ad8be5` (Phase 3, plan 07) → 85.85%
now, a **−0.07 point** move — effectively flat, well inside measurement noise, and nowhere near the
floor.

**Plan 08-07's `cli`-gating change (D-16/D-00e — this phase's one plausible regression vector,
addressed explicitly rather than assumed neutral):** gating `paladin-herald`'s `table_herald` module
behind the `table` feature removed **30** `#[test]` functions from the default-feature run (3 in the
root `paladin-ai` crate, 27 inside `paladin-herald`'s own `table_herald` test module) — ten times the
3 the plan anticipated (`08-07-SUMMARY.md`, "Issues Encountered"). Crucially, the *source* module
(`crates/paladin-herald/src/table_herald.rs`) is gated the same way as its tests: this llvm-cov run's
per-file rows contain **no `table_herald.rs` row at all** (confirmed:
`grep 'table_herald' <report-output>` returns only `markdown_herald.rs`, which stays compiled by
default with only its *coloured* rendering path split out). Both the removed tests and the source
lines they exercised are gone from **both** the numerator and the denominator symmetrically — this
is a **feature-gating removal, not a coverage regression**: no line that used to count as covered
now counts as missed. All 30 tests still run and pass under `--features cli` / `--features
table,color` (`08-07-SUMMARY.md`), so no test coverage is lost, only excluded from this scope's
denominator by the same construction `minio.rs` already sits outside it under (see "`minio.rs` —
outside the gated denominator by construction" above).

**No coverage claim is degraded by this phase's changes.** The floor arithmetic recorded above under
"The gate (floor): 84%, hard-fail from the first run" is unchanged and restated here by reference
only: the floor is derived by truncating the measured figure toward zero to a whole percent, the
comparison is at-or-above, and 85.85% floors to 85%, still above the standing 84% floor — this run
does **not** trigger the ratchet (which requires ≥ 2 whole points above the standing floor at a
*milestone* close, not a phase close).

## Phase 15 amendment (2026-08-13)

**(Amended by Phase 15, dated 2026-08-13, citing plan 15-01's `coverage` CI job and plan
15-03's checkpoint):** PIPE-02 extends this same ADR rather than writing a second one — the
ratchet clause specifies in-place amendment, D-00g/D-00l make it the house convention, and
RECON-07 exists precisely to eliminate the "choosing between two numbers" failure a second
coverage ADR would recreate. Superseded text elsewhere in this document is retained, not
deleted, per D-00d.

### One — the measurement and its provenance

**Run:** GitHub Actions run **31723620732** (`CI/CD Pipeline`, event `push`), job
**94526445416** (`Coverage`), url
`https://github.com/DF3NDR/paladin-dev-env/actions/runs/31723620732/job/94526445416`,
started `2026-08-13T17:01:27Z`, completed `2026-08-13T17:07:12Z`, conclusion **success**,
against commit `c33b0800f6dfb3d1d0c681c6102f71d88972388c` on `release/v0.7.0`.

**The exact command**, verbatim, as landed by plan 15-01 (`.github/workflows/ci.yml`'s
`coverage` job, "Measure coverage" step):

```
cargo llvm-cov --workspace --features integration-tests --lcov --output-path lcov.info -- --test-threads=1
```

with `USE_EXTERNAL_TEST_SERVICES=true` and live Redis (`localhost:6380`) / MinIO
(`localhost:9010`) service containers, per D-01.

**Ignored tests:** the command does not pass `--include-ignored`, so `#[ignore]`-gated tests
are outside both the numerator and the denominator — standard `cargo test` default behaviour,
stated explicitly per D-00e's reproducibility bar.

**A data-integrity finding, discovered and corrected during this amendment (Rule 1
auto-fix, T-15-08's mitigation exercised in practice).** The run's `lcov.info` and
`coverage-summary.txt` artifacts were downloaded (`gh run download 31723620732 -n
coverage-summary`) to transcribe the figure byte-identically per D-00e. Doing so surfaced
that the job's **`Coverage summary` step** (`cargo llvm-cov report --summary-only`, no
`--workspace` flag) reports coverage for **only the root `paladin-ai` package** (everything
under `src/`), not the full workspace `--lcov` output already sitting beside it. The two are
provably different scopes from the same run:

| Scope | Source | Lines | Line % | Functions | Function % |
|---|---|---|---|---|---|
| Root package only (`src/**`, no `crates/*`) — what `Coverage summary` printed | `coverage-summary.txt` TOTAL row, `cargo llvm-cov report --summary-only` (no `--workspace`) | 14018 total / 2217 missed | **84.18%** | 1848 total / 407 missed | **77.98%** | 
| Full workspace (`src/**` + all 12 `crates/*` members, 211 files) — what `--fail-under-lines` on the `--workspace` measure step actually gates | Summed directly from `lcov.info`'s `LF:`/`LH:`/`FNF:`/`FNH:` across every `SF:` record | 47618 total / 8385 missed | **82.39%** | 6115 total / 1511 missed | **75.29%** |

Verified two ways: (a) summing `LF:`/`LH:` only for `SF:` records under `/src/` and excluding
`/crates/` reproduces the root-package TOTAL row exactly — 14018/11801/84.18% lines,
1848/1441/77.98% functions, byte-identical to `coverage-summary.txt`; (b) summing across all
211 `SF:` records (root plus all twelve `crates/*` members: `paladin-core`, `paladin-ports`,
`paladin-battalion`, `paladin-herald`, `paladin-llm`, `paladin-memory`, `paladin-storage`,
`paladin-notifications`, `paladin-content`, `paladin-web`, `doc-examples`, plus the root
`src/`) gives the workspace total above. The root-package TOTAL row was what plan 15-03's
checkpoint captured and supplied for this amendment — it is retained here, marked, per D-00c,
because it is real data from the cited run and D-00e forbids silently dropping a captured
figure — but it is **not the workspace figure D-01 mandates and not what the armed gate
checks**, so it is **not** the figure this amendment floors against.

**The measured figure this ADR binds to: 82.39% workspace line coverage** — 47,618 first-party
lines counted (root package plus all workspace member crates), 8,385 missed, transcribed from
`lcov.info`'s per-file `LF:`/`LH:` records summed across all 211 `SF:` entries, reproducible by
anyone who downloads the same run's `coverage-summary` artifact and reruns the same summation.
Function coverage: **75.29%** (6,115 functions, 1,511 missed) — a ~7-point line/function gap,
consistent in kind with the ADR's original 84.79%/77.34% gap and the Phase 8 85.85%/78.50% gap.
Region coverage under the full workspace scope is **not** independently derivable from
`lcov.info` — lcov's line/function/branch format carries no per-region granularity, and a
region total requires `cargo llvm-cov report --summary-only --workspace` (the corrected form
of the step this amendment fixes — see "Fix landed" below). The next CI run under the
corrected step will print the true workspace TOTAL row, including regions, and can be
cross-checked against the 82.39%/75.29% figures recorded here.

**Fix landed alongside this amendment (Rule 1 — bug, no architectural change; same commit as
Task 3's `--fail-under-lines`):** `.github/workflows/ci.yml`'s `Coverage summary` step gains
`--workspace` (`cargo llvm-cov report --summary-only --workspace`), so its printed job-summary
figure and artifact match the scope the gate actually enforces on every future run. Before this
fix, the job summary a human reads would show a materially rosier number (84.18%) than the true
gated figure (82.39%) — exactly the "gate that silently does not measure what it reports"
failure shape T-15-09 already names for a different mechanism (Codecov), now closed for this
one too.

### Two — the scope extension, and why the earlier rejection does not contradict it

The measurement is `--workspace --features integration-tests` with Redis and MinIO running
(D-01), which is the extension this ADR itself names as scheduled work. ADR-0006 rejected
"record a CI-produced figure" at Phase 1 *because no CI gate existed to produce one*; plan
15-01 landed that gate, and this phase is what changes the premise. `--all-features` was
rejected (D-01): `qdrant` requires a live Qdrant service and the vision/embedding suites
require real provider API keys, so that code would enter the denominator with nothing in CI
able to exercise it. Default-feature-only was rejected because it leaves this ADR's own
extension instruction unfulfilled.

**A further scope difference from the original 84.79% pipeline, observed and recorded rather
than glossed:** the original `rustc`/`llvm-profdata` pipeline passed an explicit
`--ignore-filename-regex` excluding `examples/`, `benches/`, and `crates/doc-examples/`. The
`cargo llvm-cov` command plan 15-01 landed carries no equivalent flag. `examples/` and
`benches/` remain absent from this run's denominator regardless (`cargo test` does not compile
`[[example]]`/`[[bench]]` targets without `--examples`/`--benches`), but **`crates/doc-examples/`
is now included** — 9 files, part of the 211 `SF:` records the workspace total above sums.
Doctests remain excluded (no `--doctests` flag passed). Still one number, one scope: the figure
above is the one this ADR binds to, and the difference from the original pipeline's exclusions
is recorded rather than silently absorbed.

### Three — the re-derived floor

Applying this ADR's own arithmetic to the actual workspace figure: **measured 82.39%, truncated
toward zero to a whole percent → floor 82%** — explicitly neither round-half-up nor
round-half-even; a measurement already on a whole percent floors to itself (not exercised here,
since 82.39% is not itself whole). The comparison is **at-or-above**: a later run measuring
exactly 82% **passes**; a run measuring 81.99% **fails**. Because the floor is the measured
figure truncated downward, **the gate cannot be red on the run that sets it** — this is the
construction the ratchet clause depends on, and it is why the corrected 82.39% figure (not the
root-package-only 84.18%) has to be the one this floor derives from: flooring at 84 from the
uncorrected figure would have set a gate the very same run's true workspace measurement
(82.39%) fails, breaking this ADR's own no-red-on-day-one guarantee on day one.

**Relationship to the standing 84% floor and the Phase 8 85.85% figure — a scope change, not a
regression.** The two commands' denominators agree only when the ignore regex, the doctest
decision and the feature set all match (this ADR's own words, restated in section Two above),
and here they deliberately do not: this run adds `--features integration-tests` (wider —
exercises Redis/MinIO-backed code the default-feature runs never reached) and drops the
`doc-examples` exclusion (also wider — adds files with no dedicated coverage push behind
them). The lower resulting percentage (82.39% vs. 84.79%/85.85%) is the arithmetic result of a
wider denominator, not fewer passing tests against the same code — nothing that used to count
as covered now counts as missed; the denominator itself grew. `PIPE-02`'s own instruction was
"reproduce those three or record why its figure differs" — recorded here.

**`--fail-under-lines 82`** is the literal flag and value Task 3 arms in both
`.github/workflows/ci.yml` and the `Makefile`.

### Four — the two module-scoped gates, re-measured and recorded, not enforced

Extracted directly from this same run's `lcov.info` (same commit, same job, same artifact —
not a separate local measurement), summing each file's `LF:`/`LH:`/`FNF:`/`FNH:` records:

| Module scope | Target | Measured (this run) | Gap | Owner |
|---|---|---|---|---|
| Herald (`REQ-herald-consolidation-quality-gates`) — `crates/paladin-core/src/platform/container/herald.rs` | ≥ 95% | **80.49%** line coverage (246 lines, 198 hit, 48 missed); function coverage 63.04% (46 functions, 29 hit) | ~14.51 points below target | Phase 15 / PIPE-02 (recorded, not closed — Herald's climb remains named forward work, no owner beyond this record) |
| Autonomous components (`REQ-autonomous-completion-quality-gates`) — `planning_service.rs` 578/534/92.39%, `prompt_generation_service.rs` 235/208/88.51%, `temperature_service.rs` 395/371/93.92%, `handoff_service.rs` 239/228/95.40% (lines total/hit/line-%) | ≥ 90% | **92.67%** line-weighted aggregate — `(1447 - 106) / 1447 = 92.67%` | Aggregate clears the target by ~2.67 points; `prompt_generation_service.rs` individually sits ~1.49 points below it | Phase 15 / PIPE-02 |

The single workspace floor (82%, above) remains the **only binding gate** — this ADR's own
words are that module targets are "never a replacement for it." Herald's figure is essentially
unchanged from the Phase 5 amendment's 80.49% (transcribed there from
`01-coverage-measurement.md:317`) — the same file, independently re-measured under a materially
different scope (`--features integration-tests` vs. default-feature-only), landing at the same
percentage to two decimal places. The autonomous aggregate moved from 92.80% (1444 lines, 104
missed, Phase 5 amendment) to 92.67% (1447 lines, 106 missed) here — three more lines in the
denominator, two more missed, consistent with ordinary code change between phases rather than a
scope effect (the four files are not feature-gated). A per-module no-regression ratchet was
considered and rejected, as D-05 specifies: it reintroduces the multi-number failure RECON-07
exists to prevent, in a smaller form.

### Five — the three binaries, and a correction to this ADR's own D-06 premise for one of them

`Cargo.toml:239-252`'s three `[[bin]]` targets: `paladin` and `paladin-cli` both carry
`required-features = ["cli"]`; `paladin-server` carries `required-features = ["web-server"]`.
D-06's premise was that under D-01's `--features integration-tests` scope **none** of the three
compiles, so none enters the denominator — the same treatment this ADR already gives
`minio.rs`. That premise holds for two of the three and is corrected for the third:

- **`paladin` (`src/main.rs`) and `paladin-cli` (`src/bin/paladin-cli.rs`) — correctly absent.**
  Neither file appears anywhere among this run's 211 `SF:` records in `lcov.info`. `cli` is
  never activated for the root package under this measurement, so both stay outside the gated
  denominator by construction, matching D-06 and the `minio.rs` precedent exactly.

- **`paladin-server` (`src/bin/paladin-server.rs`) — unexpectedly present, 43.20% line
  coverage.** `lcov.info` carries an `SF:` record for it: 206 lines, 89 hit, 117 missed —
  **43.20%** — a real, in-scope measurement already folded into the 82.39% workspace total
  above, not a scope exclusion. Traced to source: `crates/doc-examples/Cargo.toml:15` declares
  `paladin-ai = { path = "../..", features = ["web-server"] }` — a normal (non-dev) dependency
  of the `doc-examples` workspace member on the root package, requesting `web-server`
  explicitly. Because `crates/doc-examples` is a workspace member (`members = [".",
  "crates/*"]`, `Cargo.toml:2`) built in the same `cargo test --workspace` invocation as the
  root package, Cargo's workspace feature resolution activates `web-server` for the root
  package for the whole build — the root package and the `doc-examples`-requested dependency on
  it are the same package instance, so its activated feature set is unified across the build
  graph. This is **corrected here per D-00d**: the original text above ("Under D-01's feature
  set none of them compiles") is superseded for `paladin-server` specifically and retained
  for the two `cli`-gated binaries. `.codecov.yml`'s `src/bin/**` ignore entry (D-06) still
  applies for Codecov's own report, unaffected by this correction — it is a reporting-only
  exclusion and does not change what `--fail-under-lines` gates.

### Six — two inherited dispositions that close by observation

D-14a named "extracting a testable `run()` seam from `#[tokio::main] async fn main()`" as the
concrete prerequisite Phase 15 inherits. Confirmed by direct read: `src/bin/paladin-server.rs:34`
is `async fn main()`, `:49` is `async fn run() -> Result<(), Box<dyn std::error::Error>>` — the
seam already exists. The recorded 0.00% figure for that file (D-14a, transcribed from
`01-coverage-measurement.md:426`) is stale in two ways now: a `#[cfg(test)] mod tests` sits at
`:256` (confirmed by direct read — Phase 14's D-15b addition), and per section Five above the
file is not even 0%-covered under this phase's scope (43.20%, via the `doc-examples` feature
unification). Both original claims are marked superseded here with the original text retained,
per D-00c/D-00d. Neither requires code — both close by observation.

### Seven — the tool-of-record note's premise has changed

The raw `rustc`/`llvm-profdata` pipeline this ADR used through Phase 8 was forced by an HTTP 403
to crates.io. `cargo-llvm-cov@0.8.7`, pinned via `taiki-e/install-action@v2` in plan 15-01's
`coverage` job, installed and ran successfully in the cited run (job conclusion: success) — the
403 that forced the raw pipeline is gone, `cargo llvm-cov` is the tool-of-record for this and
future measurements. Docker remains absent from every local authoring environment (confirmed
2026-08-12/13, unchanged since Phase 15's context-gathering), so the service-backed
(`--features integration-tests`) figure necessarily still comes from CI, never a local run —
this is the entire reason D-04's two-step (measure, then gate) landing exists.

Do not author a new ADR and do not touch `PROMOTION.md` (D-00l).

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
