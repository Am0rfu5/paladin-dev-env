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
| 32877178870 | `refs/heads/eval/codeql-probe` | `04328647` | workflow_dispatch | success | 223 | warm (2nd-ever run) | 385 | 1 |
| 32877627856 | `refs/heads/eval/codeql-probe` | `04328647` | workflow_dispatch | success | 220 | warm (3rd-ever run) | 385 | 1 |

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

**Run 32877178870 (Task 1 probe run, 2026-08-25T17:19:11Z–17:22:54Z):** the D-09 dedicated
evaluation branch `eval/codeql-probe` was pushed to `origin` at the current tree (commit
`04328647`, the tip after wave 2's merge — plans 18-01 and 18-02 both landed), then
`codeql.yml` was dispatched via `gh workflow run codeql.yml --ref eval/codeql-probe -f
scan_probe_fixture=true`, selecting `.github/codeql/codeql-config-probe.yml` (fixture included
in scope). Watched to completion via `gh run watch 32877178870 --exit-status`, conclusion
`success`, job wall-clock **3m43s (223s)**.

**A duplicate run was cancelled by the workflow's own concurrency group, exactly as designed.**
Pushing `eval/codeql-probe` itself triggered a second, automatic `push`-event run
(`32877168387`) under the identical concurrency group (`codeql-${{ github.head_ref ||
github.ref }}`, `cancel-in-progress: true` off `main`) as the immediately-following
`workflow_dispatch` run. GitHub cancelled the older push-triggered run 24 seconds in
(`Canceling since a higher priority waiting request for
codeql-refs/heads/eval/codeql-probe exists`). Per this task's own instruction, a `cancelled`
conclusion contributes no metrics and is recorded here as cancelled rather than counted; it does
not appear as a data row in the Run Log table above. This run happened to have been a
steady-state-config run (no `workflow_dispatch` input, so `scan_probe_fixture` defaulted false)
— its cancellation is why Task 2 required its own separate `workflow_dispatch` run
(`32877627856`, see `## Steady-State Exclusion` below) rather than reusing this one.

## Probe Result

Read via `gh api "/repos/DF3NDR/paladin-dev-env/code-scanning/alerts?ref=refs/heads/eval/codeql-probe&tool_name=CodeQL&per_page=100"`
(HTTP 200) against run `32877178870`. Cross-checked against
`/repos/DF3NDR/paladin-dev-env/code-scanning/analyses?ref=refs/heads/eval/codeql-probe`: analysis
id `1670508215`, category `/language:rust`, created `2026-08-25T17:22:27Z` (matching this run's
window), `results_count: 1`, `rules_count: 27` — the same 27-rule `security-extended` query pack
as the 18-01 tracer run, so rule selection did not narrow between runs. The alerts endpoint and
the analyses endpoint's `results_count` agree exactly (1 each), so no page-2 result exists that a
`per_page=100` single-page read could have missed (T-18-15).

**Coverage established before interpreting the finding count (T-18-11):**
`scripts/codeql-analysed-files.sh 32877178870` reports `probe_fixture_entries=6` — greater than
zero. All six of the fixture's own files were directly confirmed present in the extracted
`src.zip` source set (verified by unzipping the nested archive and filtering for
`fixtures/codeql-probe`):

```
fixtures/codeql-probe/src/command_injection.rs
fixtures/codeql-probe/src/credential.rs
fixtures/codeql-probe/src/feature_gated.rs
fixtures/codeql-probe/src/lib.rs
fixtures/codeql-probe/src/path_traversal.rs
fixtures/codeql-probe/src/sql_injection.rs
```

This is "analysed nothing" ruled out directly: the fixture crate's entire five-defect surface
(plus `lib.rs`) was extracted into CodeQL's database. Against that confirmed six-file extraction,
the finding count below is genuinely "found nothing," not an artifact of the fixture never having
been scanned.

| Class | Fixture File | Alert Raised | Rule ID | Severity |
|---|---|---|---|---|
| Hardcoded credential (D-08 #1) | `fixtures/codeql-probe/src/credential.rs` | No | — | — |
| Shell command injection via `sh -c` (D-08 #2) | `fixtures/codeql-probe/src/command_injection.rs` | No | — | — |
| Path traversal (D-08 #3) | `fixtures/codeql-probe/src/path_traversal.rs` | No | — | — |
| SQL injection (D-08 #4) | `fixtures/codeql-probe/src/sql_injection.rs` | No | — | — |
| Feature-gated command injection, D-12's coverage probe | `fixtures/codeql-probe/src/feature_gated.rs` | No | — | — |

**Zero alerts were raised for all five planted classes.** The run's single reported alert
(`results_count: 1`) is alert #28 (`rust/hard-coded-cryptographic-value`,
`src/core/platform/manager/user_service.rs`) — the same genuine first-party finding the 18-01
tracer run first surfaced, at a path entirely outside `fixtures/codeql-probe/`. It is not a probe
finding and is excluded from the table above and from every count in this section. No other
mechanism was needed to separate the two: the alerts endpoint returned exactly one alert total,
and its location path does not match any fixture file, so there was nothing to filter out beyond
confirming that one path directly.

## Baseline Comparison

**CodeQL's probe result: 0 findings across all five planted classes**, against confirmed coverage
of all 6 fixture files (extraction proven directly, see above) and 27 executed `security-extended`
Rust rules (per this run's own `rules_count: 27`).

**Recorded Snyk baseline** (`.github/instructions/security.instructions.md`, "Snyk was evaluated
and removed"): Snyk Code returned **0 findings** against the identical four-class Rust fixture
(hardcoded credential, `sh -c` command injection, path traversal, SQL injection), and **3
findings** (HIGH/MEDIUM/LOW) against the same four classes ported to JavaScript — proving the
scanner and credentials worked and the Rust analysis specifically did not.

**Comparison, stated plainly:** CodeQL's raw finding count on this Rust probe (0) is numerically
**equal** to Snyk's raw finding count on the same four classes (0). Both scanners report zero
Rust findings. But the two zeros rest on different evidence. Snyk's 0 was shown by its own
JavaScript control to mean "this scanner carries no meaningful Rust rule coverage," not "this
scanner analysed the code and found it clean" — Snyk Code ingests `.rs` files but applies no
Rust-specific taint rules to them. CodeQL's 0 here is accompanied by direct, independent proof
of both extraction (all 6 fixture files confirmed present in the analysed source set) and rule
execution against this exact tree (27 Rust `security-extended` rules ran against this checkout,
and this very same run's SARIF surfaced a genuine, unrelated first-party finding — alert #28,
`rust/hard-coded-cryptographic-value` — proving the credential-detection rule class fires on real
code in this repository, not only in theory). No JavaScript control exists for CodeQL — it is a
genuine multi-language SAST product, not a single-purpose probe target the way the Snyk
evaluation was — so the "equal-or-better" judgment here rests on coverage evidence rather than a
cross-language control the way Snyk's did.

On raw count alone, this result is **equal** to Snyk (0 = 0). On coverage evidence, it is
**better**: proven extraction and proven rule execution stand behind CodeQL's zero, where no such
evidence was ever established for Snyk's zero on this repository. Per D-11, **this coverage
distinction does not exempt the tool from the disqualifying threshold** — a zero-finding result
across all four (here, five) planted classes disqualifies CodeQL regardless of how well-evidenced
the zero is. The coverage evidence explains why this particular zero can be trusted as a genuine
"found nothing" rather than a mechanism failure; it does not change what D-11 says a genuine zero
means for adoption.

## Analysis Coverage

### Probe Run Coverage (32877178870, Task 1)

`scripts/codeql-analysed-files.sh 32877178870` output, verbatim:

```
run_id=32877178870
analysed_rs_files=385
denominator=385
difference=0
probe_fixture_entries=6
feature_gated_present=src/infrastructure/web/mod.rs:yes
feature_gated_present=src/application/cli/commands/agent.rs:yes
feature_gated_present=crates/paladin-web/src/lib.rs:yes
src_zip_total_rs_entries=3440
src_zip_checkout_rs_entries=563
src_zip_toolchain_stdlib_rs_entries=2874
src_zip_other_vendored_rs_entries=3
```

**385 of 385 — the denominator-scoped count is unchanged from the 18-01 tracer run.** `difference=0`.
The fixture's own files (`fixtures/codeql-probe/**`) are outside the `crates/**/*.rs` and root
`src/**/*.rs` globs that define the 385 denominator by design, so they do not inflate
`analysed_rs_files` — `probe_fixture_entries=6` is reported as its own separate field for exactly
this reason (D-13's "neither number is mistaken for the other").

**D-12's two-signal answer, this time from both the first-party paths and the probe fixture
itself:**

- **Signal 1 (first-party feature-gated file reach, reconfirmed):** all three
  `feature_gated_present` lines report `yes` on this run too
  (`src/infrastructure/web/mod.rs`, `src/application/cli/commands/agent.rs`,
  `crates/paladin-web/src/lib.rs`) — identical to the 18-01 tracer run's result. Buildless
  extraction reaches feature-gated first-party code regardless of which cargo features are
  active at scan time, reconfirmed on a second, independent run.
- **Signal 2 (the probe fixture's own fifth defect):** `fixtures/codeql-probe/src/feature_gated.rs`
  — gated behind the non-default `probe-feature-gated` cargo feature, never enabled anywhere in
  this run — is directly confirmed present in the extracted source set (one of the 6
  `probe_fixture_entries`, listed by name in `## Probe Result` above). Extraction reached it
  exactly as it reached its four unconditional siblings in the same crate.

**Both signals agree in the qualifying direction on the narrow, mechanistically-grounded question
D-12 actually asks** — does buildless extraction reach code gated behind a non-default cargo
feature at all — and no disagreement is recorded, because none exists: extraction reach is
independently confirmed by both the first-party paths (Signal 1) and the probe fixture's own
gated file (Signal 2). The *report status* half of Signal 2 (does the planted defect inside that
gated file actually get flagged) is uninformative on its own here, because this run's finding
count is 0 for every one of the five planted classes, gated or not — `feature_gated.rs`'s
non-report cannot be distinguished from `command_injection.rs`'s non-report by report status
alone. The file-reach evidence (both signals) is what answers D-12; the report-status evidence is
subsumed by the `## Probe Result` disqualifying finding above and is not treated as a second,
independent data point for D-12 specifically.

**Cross-check against `codeql database print-baseline` (same check re-attempted on this run):**
this run's debug artifact logs were checked for a `print-baseline` invocation or
lines-of-code summary using the same log-file list the 18-01 tracer run checked. None appears
here either — the absence is consistent across both runs, not a one-off omission, and no
disagreement is recorded because no second number exists on either run to disagree with the
`src.zip`-derived count.

### Original Tracer Run Coverage (32868842656, 18-01)

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

## Steady-State Exclusion

**Steady-state run: `32877627856`** (`refs/heads/eval/codeql-probe`, commit `04328647`,
`workflow_dispatch` with `scan_probe_fixture` left `false` — the default path, selecting
`.github/codeql/codeql-config.yml`, the config that carries `paths-ignore: [fixtures/codeql-probe]`).
Watched to completion via `gh run watch 32877627856 --exit-status`, conclusion `success`, job
wall-clock **3m40s (220s)**.

This run was dispatched separately from Task 1's probe run rather than reusing the automatic
push-triggered run (`32877168387`) that fired when `eval/codeql-probe` was first pushed, because
that push-triggered run was itself cancelled by the workflow's own concurrency group when Task 1's
`workflow_dispatch` run started moments later (see `## Run Log` above) — a cancelled run carries
no metrics to read.

**Two independent numbers, both confirming the exclusion held:**

1. **`scripts/codeql-analysed-files.sh 32877627856` reports `probe_fixture_entries=0`.** Full
   output:

   ```
   run_id=32877627856
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

   Zero fixture entries were extracted at all on the steady-state config — the `paths-ignore`
   analysis-scope exclusion (18-02) kept the fixture out of CodeQL's database entirely, not merely
   out of the alert store. (`analysed_rs_files=385`/`difference=0` and the three
   `feature_gated_present=...:yes` lines are unchanged from every other run — the exclusion is
   scoped precisely to `fixtures/codeql-probe/` and does not touch the first-party denominator or
   the first-party feature-gated paths.)

2. **Zero code-scanning alerts on this ref have a location path under `fixtures/codeql-probe/`.**
   Read via `gh api "/repos/DF3NDR/paladin-dev-env/code-scanning/alerts?ref=refs/heads/eval/codeql-probe&per_page=100"`
   (HTTP 200, 16 open alerts total on this ref across all tools — the repository's existing 15
   `osv-scanner` alerts plus the one pre-existing CodeQL alert #28, all present on this branch
   because it shares full history with the tip it was branched from). None of the 16 alert
   locations start with `fixtures/codeql-probe/`. Cross-checked against the analyses endpoint:
   this run's own CodeQL Rust analysis (id `1670538979`, created `2026-08-25T17:27:52Z`,
   `results_count: 1`) reports the same single result as every other run on this tree — alert #28
   — confirming the zero-fixture-alert count is not an artifact of a truncated read.

**Both signals agree: the exclusion held with no surviving fixture alert to disposition.** Neither
number is non-zero, so no alert requires the 18-04 governed-register disposition path, and
`.github/codeql/codeql-config.yml` required no adjustment — its existing
`paths-ignore: [fixtures/codeql-probe]` entry (18-02) is sufficient as written.

`.github/workflows/codeql.yml` still declares no `paths`/`paths-ignore` key under any trigger
(confirmed unchanged by this plan — `git diff` against this plan's own commits touches only this
evidence document), and `bash scripts/check-workflow-triggers.sh` exits 0:

```
🔍 Checking workflow trigger surfaces against the recorded policy table ...
✅ 7 workflow file(s) scanned, 7 policy-table row(s) read; coverage, drift, context and reachability clauses all pass.
```

## Verdict

pending — probe not yet run

## Promotion Status

advisory — context not pinned in any ruleset
