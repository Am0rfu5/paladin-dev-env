# CodeQL Rust SAST — Evidence Log

Phase: 18-rust-sast-evaluate-and-adopt-codeql
Requirements: SAST-01, SAST-02, SAST-03, SAST-04

This document is the phase's evidence document (D-16): the raw log lives here, under
`.planning/`, where the Snyk evaluation's evidence pattern already lives
(`.github/instructions/security.instructions.md` §"Snyk was evaluated and removed
(2026-08-18)"). Only its **conclusions** propagate into `security.instructions.md` later
(SAST-04); this document is the source of record, not the summary.

## Method

What is measured, and how: `.github/workflows/codeql.yml`'s `CodeQL Analysis (Rust)` job is run
against this repository (build-mode `none`, `security-extended` queries, `debug: true`), and two
independent numbers are read off each run — the **analysed-file count** (how many of this
checkout's own `.rs` files CodeQL's internal `rust-analyzer` pass actually archived into its
database, via `scripts/codeql-analysed-files.sh` reading the run's `debug-artifacts` artifact) and
the **finding count** (what the code-scanning alert store reports for that ref). These two numbers
are never collapsed into one verdict: a scanner that analysed nothing and a scanner that analysed
everything and found nothing look identical in a finding count alone — the exact confusion that
disqualified Snyk (`security.instructions.md`'s "Measured, not assumed" section) — and the
analysed-file count is what tells them apart.

## Promotion Criteria

*Written 2026-08-25, before the D-11 probe (fixture crate, plan 18-02) or the D-14/D-15 backfill
observation window have produced a single number, so the threshold cannot be retrofitted to
whatever those numbers turn out to be. Task 1's tracer run below is an end-to-end wiring proof, not
the probe or the observation window — it exists to prove the pipeline runs at all before either of
those depends on it.*

A scanner qualifies for promotion to a required check (SAST-03, D-18) only if **all** of the
following hold over the D-14/D-15 observation window:

1. **False-positive-rate ceiling: ≤ 20%.** At least 4 of every 5 triaged findings must be
   confirmed true positives (or an equivalent aggregate ratio over the whole window). A scanner
   producing more noise than that trades one assurance problem (silent gaps) for another (alert
   fatigue that trains reviewers to dismiss findings unread).
2. **Wall-clock ceiling: ≤ 600 seconds (10 minutes) per run**, warm or cold cache. The Task 1
   tracer run measured 212s (3m32s) cold-cache end to end — well inside this ceiling — but a
   single data point does not establish a distribution; the ceiling is asserted here so a
   regression during the observation window has a named bar to fail against.
3. **Minimum analysed-file count: ≥ 366 of the 385 denominator (≥ 95%)**, per run. The Task 1
   tracer run measured exactly **385/385** (see Run Log below); this 95% floor exists so that a
   later run, on a larger or restructured tree, has explicit headroom for incidental exclusions
   (e.g. a file mid-rename) without silently lowering the bar to whatever a bad run produces.
4. **Disqualifying conditions, either one overrides every number above:**
   - A probe finding count of **exactly 0** across all four planted defect classes (D-08:
     hardcoded credential, `sh -c` command injection, path traversal, SQL injection) disqualifies
     the tool regardless of every other number (D-11). This is the Snyk failure shape recurring
     with a different tool; a scanner that cannot find its own planted defects does not get
     credit for a low false-positive rate elsewhere.
   - An analysed-file count **far below** the 385 denominator (below the 95% floor in item 3) is
     disqualifying even when findings are non-zero (D-13) — non-zero findings from a scan that
     covered a fraction of the tree is exactly the "clean-looking but blind" failure this phase
     exists to catch.

If the D-14/D-15 numbers do not qualify, the phase closes with the scanner advisory (D-06's
posture, already the case — see Promotion Status below) and this criteria section stands as the
named, dated, un-retrofitted trigger condition for revisiting promotion later (D-18).

## Run Log

Only runs whose `conclusion` is `success` or `failure` are recorded with metrics; a `cancelled` run
contributes no metrics and is noted separately if it occurs.

| run_id | ref | commit | event | conclusion | wall_clock_s | cache | analysed_rs_files | alerts_total |
|---|---|---|---|---|---|---|---|---|
| 32868842656 | `refs/heads/codeql-tracer-18-01` | `4b74cae7` | push | success | 212 | cold | 385 | 1 |

**Run 32868842656 (Task 1 tracer, 2026-08-25T15:55:59Z–15:59:35Z):** the first-ever execution of
`codeql.yml` against this repository. Pushed to a disposable branch (`codeql-tracer-18-01`,
deleted after this evidence was captured — see "Tracer branch" below), watched to completion via
`gh run watch 32868842656 --exit-status`, conclusion `success`. Job wall-clock (the
`CodeQL Analysis (Rust)` job itself, excluding queue time): **3m32s (212s)**. `Swatinem/rust-cache`
had nothing to restore (first run ever), so this is a **cold-cache** measurement — the D-15 warm-
cache figure is not yet established and will come from a subsequent run reusing the same cache key.

**SARIF landed and is readable via the API**, confirming the full pipeline end-to-end:
`GET /repos/DF3NDR/paladin-dev-env/code-scanning/analyses?ref=refs/heads/codeql-tracer-18-01`
returned one analysis (`category: /language:rust`, `tool: CodeQL 2.26.3`, `results_count: 1`,
`rules_count: 27`).

**alerts_total=1 is a genuine first-party finding, not the D-11 probe** (the probe fixture does not
exist yet — it is plan 18-02's deliverable). `GET .../code-scanning/alerts?ref=refs/heads/codeql-
tracer-18-01&tool_name=CodeQL` returned exactly one CodeQL-tool alert:

| # | Rule | Severity | Path | State |
|---|------|----------|------|-------|
| 28 | `rust/hard-coded-cryptographic-value` | critical | `src/core/platform/manager/user_service.rs` | open |

Per this phase's explicit out-of-scope boundary (18-CONTEXT.md: "fixing whatever real defects the
scanner finds ... is its own work"), this finding is **recorded here, not remediated in this
plan**. It is left open in the code-scanning alert store for triage in a follow-up plan or phase.
It is also, incidentally, a first data point in the eventual D-14/D-15 true-positive/false-positive
tally, once triaged — but one alert is not a sample, and it is not counted toward the Promotion
Criteria's false-positive-rate ceiling here.

**Tracer branch:** pushed to `refs/heads/codeql-tracer-18-01` on `origin`
(`DF3NDR/paladin-dev-env`), watched to completion, evidence captured (this document, plus
`scripts/codeql-analysed-files.sh`'s output below) — then deleted
(`git push origin --delete codeql-tracer-18-01`) once capture was complete, per this phase's D-09
posture of not leaving standing scan surfaces around. The code-scanning analysis and alert #28
remain queryable by commit SHA (`4b74cae7`) after the branch deletion; GitHub retains code-scanning
history independent of ref lifetime.

## Analysis Coverage

`scripts/codeql-analysed-files.sh 32868842656` output, verbatim:

```
run_id=32868842656
analysed_rs_files=385
denominator=385
difference=0
probe_fixture_entries=0
feature_gated_present=src/infrastructure/web/mod.rs:yes
feature_gated_present=src/application/cli/commands/agent.rs:yes
feature_gated_present=crates/paladin-web/src/lib.rs:yes
src_zip_total_rs_entries=3434
src_zip_checkout_rs_entries=557
src_zip_toolchain_stdlib_rs_entries=2874
src_zip_other_vendored_rs_entries=3
```

**385 of 385 — an exact match against the denominator**, on the first real run. `difference=0`.
`probe_fixture_entries=0` is expected and correct: the probe fixture (`fixtures/codeql-probe/`)
does not exist until plan 18-02.

**D-12's open question — does buildless Rust extraction reach code gated behind non-default cargo
features? — is answered empirically, not assumed, by this run.** All three named feature-gated
probe paths (`src/infrastructure/web/mod.rs`, gated on `web-server`;
`src/application/cli/commands/agent.rs`, gated on `cli`; `crates/paladin-web/src/lib.rs`, the
`paladin-web` crate's own default root) are present in the analysed set. The mechanism is visible
directly in the run's own log
(`rust/log/database-index-files-*.log`):

```
This is codeql database index-files --verbosity=progress++ --include-extension=.rs
  --exclude=**/.git --size-limit=5m --language=rust --working-dir=. ...
[PROGRESS] database index-files> Scanning for files in /home/runner/work/paladin-dev-env/paladin-dev-env...
```

CodeQL's Rust extractor performs a **filesystem walk keyed on the `.rs` extension**, not a
`cargo build`/`cargo check` invocation resolving a feature set. It never asks Cargo which files are
reachable under which feature flags — it indexes every `.rs` file present in the checkout,
independent of Cargo feature gating entirely. This is a stronger and more mechanically-grounded
answer than "the probe found the fifth planted defect" would have been: it explains *why* coverage
is complete (file-extension indexing has no feature-selection step to narrow it), not merely *that*
it happened to be complete on this run. **This does not by itself prove queries reason correctly
about code paths that are only reachable when a feature is enabled** — extraction reaching a file
and analysis producing correct dataflow/taint facts inside it are two different claims — but it
settles the narrower, file-reach question D-12 named as unverified, and settles it in the
qualifying direction.

**Assumption A5 (RESEARCH.md) is corrected, not confirmed.** The debug artifact's `src.zip` is
**not** at the top level of the `debug-artifacts` artifact as RESEARCH.md's Pattern 2 sketched —
it is nested inside `db-<language>.zip` (e.g. `db-rust.zip`) at `<db-basename>/src.zip`. A script
that looked for `src.zip` at the artifact's top level would find nothing and, absent careful
failure handling, could easily misreport "0 analysed files" — which would have been *exactly* the
kind of false-negative-through-mechanism-error this whole evidence chain exists to prevent. This is
recorded here as a correction to the research record, and `scripts/codeql-analysed-files.sh`
implements the corrected path directly.

**`src.zip`'s raw entry count is not 1:1 with the checkout (Open Question 2, RESEARCH.md).** It
also archives the Rust toolchain's own standard-library source (2,874 of 3,434 total `.rs`
entries) and three CodeQL-bundled Rust builtin files, alongside the 557 checkout-relative entries.
`analysed_rs_files` above is scoped to exactly the two globs (`crates/**/*.rs`, root `src/**/*.rs`)
that define the 385 denominator — the raw totals are reported separately by the script
(`src_zip_total_rs_entries`, etc.) specifically so neither number is mistaken for the other.

**Cross-check against `codeql database print-baseline` (Open Question 2, second half):** attempted
against this run's own log artifacts. No `print-baseline` invocation or lines-of-code summary
appears anywhere in the debug artifact's log files (`log/database-init-*.log`,
`rust/log/database-index-files-*.log`, `rust/log/database-trace-command-*.log`,
`rust/log/dataset-import-*.log`, `rust/log/database-finalize-*.log`,
`rust/log/database-run-queries-*.log`, `rust/log/execute-queries-*.log`,
`rust/log/database-interpret-results-*.log`). This corroborating check is therefore **not
available** from a standard `codeql-action` run without invoking the CLI directly against the
downloaded database zip (out of scope for this plan) — no disagreement is recorded because no
second number exists to disagree with the `src.zip`-derived count.

## Verdict

pending — probe not yet run

## Promotion Status

advisory — context not pinned in any ruleset
