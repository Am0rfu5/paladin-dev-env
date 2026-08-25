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
| 32884197028 | `refs/heads/eval/codeql-probe` | `f9bc44cb` | workflow_dispatch | success | 181 | warm (5th-ever run) | 385 | 2 |
| 32889890607 | `refs/heads/eval/codeql-probe` | `c7e3bc84` | push | success | 168 | warm (6th-ever run) | 385 | 1 |
| 32890183115 | `refs/heads/eval/codeql-probe` | `c7e3bc84` | workflow_dispatch | success | 188 | warm (7th-ever run) | 385 | 2 |
| 32894118236 | `refs/heads/eval/codeql-probe` | `70efbe23` | workflow_dispatch | success | 209 | warm (9th-ever run) | 386 | 2 |

**Run 32893827932** (8th-ever run, push-triggered by the confound experiment's push) was
cancelled by the same concurrency mechanism as every prior push-triggered run on this branch —
`gh run view` reports conclusion `cancelled`, "Canceling since a higher priority waiting
request ... exists," after 3m25s. Its partial code-scanning analysis (id `1671479428`,
`results_count: 1`, `rules_count: 27`, created `2026-08-25T20:13:27Z`) is notably more complete
than prior cancelled-run debris (which showed `results_count: 0, rules_count: 0` with an
explicit `error` field) — this one may have completed its SARIF upload before the cancellation
reached the post-analysis cleanup steps. Per this document's own convention, a `cancelled`
conclusion contributes no metrics regardless of how far it progressed, and it does not appear
as a data row above; run `32894118236` (the following `workflow_dispatch`) is the authoritative
result for this iteration.

**Run 32894118236 (18-03 continuation, workspace-member confound test,
2026-08-25T20:13:39Z–20:17:08Z):** dispatched via `gh workflow run codeql.yml --ref
eval/codeql-probe -f scan_probe_fixture=true` after pushing the confound experiment (commit
`70efbe23`, containing both the unchanged excluded fixture AND the new workspace-member
confound file `src/codeql_workspace_probe.rs`) to `eval/codeql-probe`. Watched to completion,
conclusion `success`, job wall-clock **3m29s (209s)**. `analysed_rs_files=386` — one more than
the 385 denominator, `difference=-1` — confirming the confound file was counted within the
denominator-scoped `src/**/*.rs` glob (385 original first-party files + 1 new confound file).
`alerts_total=2` — see `## Workspace-Member Confound Test Result` below for the per-class
breakdown.

**`eval/codeql-probe` deleted again after this iteration's evidence capture**, per the same
D-09 posture applied after every prior probe run on this branch. Every analysis and alert
referenced in this section remains queryable by commit SHA (`70efbe23`) independent of the
branch's lifetime — though per the hard safety invariant, that commit itself was never part of
this plan's own mergeable branch history (see `## Workspace-Member Confound Test Result` for
the exact cleanup steps taken).

**Run 32889890607** is the automatic push-triggered steady-state run (`scan_probe_fixture`
unset, defaulting `false`) that fired when the diagnostic fixture variant was pushed. Unlike
both prior pushes to this branch, it completed successfully (168s) rather than being cancelled
— the following `workflow_dispatch` run (`32890183115`) was not dispatched until after it had
already finished, so the two never collided in the concurrency group. Its `alerts_total=1` is
the pre-existing corrected alert #28 only, consistent with every other steady-state result on
this branch; not otherwise analysed further, since Task 2's steady-state exclusion was already
proven independently and re-verifying it was not part of this diagnostic's scope.

**Run 32890183115 (18-03 continuation, diagnostic iteration, 2026-08-25T19:32:52Z–19:36:00Z):**
dispatched via `gh workflow run codeql.yml --ref eval/codeql-probe -f
scan_probe_fixture=true` after pushing the diagnostic fixture variant (commit `c7e3bc84`) to
`eval/codeql-probe`. Watched to completion, conclusion `success`, job wall-clock **3m8s
(188s)**. `alerts_total=2` — see `## Diagnostic Iteration Result` below for the per-class
breakdown.

**`eval/codeql-probe` deleted again after this iteration's evidence capture** (`git push
origin --delete eval/codeql-probe`), per the same D-09 posture applied after every prior probe
run on this branch. Every analysis and alert referenced in this section remains queryable by
commit SHA (`c7e3bc84`) independent of the branch's lifetime.

**Run 32884197028 (18-03 continuation, re-probe against the redesigned fixture,
2026-08-25T18:30:54Z–18:35:55Z):** dispatched via `gh workflow run codeql.yml --ref
eval/codeql-probe -f scan_probe_fixture=true` after pushing the redesigned fixture (commit
`f9bc44cb`) to the same `eval/codeql-probe` branch. Watched to completion, conclusion
`success`, job wall-clock **3m1s (181s)**. `alerts_total=2` — see `## Re-Probe Result` below
for the per-class breakdown.

**A duplicate push-triggered run was cancelled by the same concurrency mechanism as the first
probe.** Pushing the redesigned fixture to `eval/codeql-probe` triggered an automatic
`push`-event run (`32883958722`), cancelled 3m21s in when the following `workflow_dispatch`
run (`32884197028`) started. Its partial SARIF upload is visible in the code-scanning analyses
list as analysis id `1670898889` (`error: "unsuccessful execution, exit code: 0"`,
`results_count: 0`, `rules_count: 0`, created `2026-08-25T18:31:06Z`) — recorded here as
debris from the cancelled run, not a real zero-result analysis, and not counted anywhere in
this document.

**`eval/codeql-probe` deleted after evidence capture** (`git push origin --delete
eval/codeql-probe`), per this phase's D-09 posture of not leaving standing scan surfaces
around — matching the same cleanup already applied to the `codeql-tracer-18-01` branch in
18-01. Every code-scanning analysis and alert referenced in this section (`32884197028`'s
results, alert #29) remains queryable by commit SHA (`f9bc44cb`) after the branch's deletion;
GitHub retains code-scanning history independent of ref lifetime. The fixture source itself is
unaffected — it lives in this plan's own commits on the worktree branch, not on the deleted
evaluation branch.

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

**alerts_total=1 is a real CodeQL alert instance, not a D-11 probe finding** (the probe fixture
does not exist yet — it is plan 18-02's deliverable). `GET .../code-scanning/alerts?ref=refs/heads/codeql-
tracer-18-01&tool_name=CodeQL` returned exactly one CodeQL-tool alert:

| # | Rule | Severity | Path | State |
|---|------|----------|------|-------|
| 28 | `rust/hard-coded-cryptographic-value` | critical | `src/core/platform/manager/user_service.rs` | open |

Per this phase's explicit out-of-scope boundary (18-CONTEXT.md: "fixing whatever real defects the
scanner finds ... is its own work"), this finding is **recorded here, not remediated in this
plan**. It is left open in the code-scanning alert store. **Correction (18-03 continuation,
2026-08-25):** this alert was originally described here as a "genuine first-party finding"
requiring later triage; triage is now complete and it is a **test-code false positive** — see the
full reframe in the Verdict section → "Alert #28 correction" below. It is the first (and so far only)
data point toward the eventual D-14/D-15 true-positive/false-positive tally: **1 triaged, 1 false
positive**, recorded there rather than counted toward the Promotion Criteria's false-positive-rate
ceiling in this section, since one alert is not a sample.

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
`src/core/platform/manager/user_service.rs`) — the same alert (a test-code false positive on
triage, not a "genuine first-party finding" as first characterized — see the full reframe in
the Verdict section → "Alert #28 correction" below) the 18-01 tracer run first surfaced, at a path
entirely outside `fixtures/codeql-probe/`. It is not a probe finding and is excluded from the
table above and from every count in this section. No other mechanism was needed to separate the
two: the alerts endpoint returned exactly one alert total, and its location path does not match
any fixture file, so there was nothing to filter out beyond confirming that one path directly.

**Correction (18-03 continuation, 2026-08-25): this zero-finding result is instrument-invalid, not
disqualifying.** All five classes tabled above were structurally incapable of triggering any wired
CodeQL Rust query — three lacked a recognized taint source under the default `remote` threat
model, and two (the command-injection classes) target a CWE with no upstream Rust query at all.
This was established from the query source code itself, not inferred from the zero result. The
full per-class analysis, citations, and the resulting `instrument-invalid` verdict are in
the Verdict section below; this section is retained as an accurate historical record of what the first
probe run actually measured.

## Baseline Comparison

**Correction (18-03 continuation, 2026-08-25): the "equal-or-better than Snyk" and "disqualifies
CodeQL" conclusions below do not hold — the underlying probe was instrument-invalid (see
the Verdict section).** This section's factual claims about what was measured (0 findings, coverage
numbers, alert #28's existence) are accurate and left as recorded; its *interpretation* of those
numbers as meaningful evidence about CodeQL's Rust detection capability is not, because none of
the five planted classes could have alerted regardless of how well CodeQL actually performs. The
comparison below is retained for the historical record, not as the basis for the verdict.

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
and this very same run's SARIF surfaced an unrelated alert — alert #28,
`rust/hard-coded-cryptographic-value` — proving the extraction→taint→alert pipeline runs end to
end against this repository's real code; on triage this alert is a test-code false positive, not
the genuine production-credential finding first claimed here — see the Verdict section → "Alert #28
correction"). No JavaScript control exists for CodeQL — it is a
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

**SAST-01 verdict: `disqualified` (version-scoped: CodeQL `2.26.3` / `rust-queries` `0.1.40`).
`codeql.yml` is retained, advisory-only, not promoted to a required check. Decided by the user
at the fourth and final checkpoint on this plan (2026-08-25), closing the evaluation.**

**Decision basis.** Across four independent measurements — the original fixture (bare-parameter
source), the redesigned fixture (`reqwest::blocking::get(...).text()` source, `?` operator,
`format!`), the diagnostic iteration (`.unwrap()`/`.unwrap_or_default()`, string concatenation),
and the workspace-member confound test (identical diagnostic shapes planted in the real,
already-scanned `paladin` crate) — SQL injection, path traversal and regex injection built from
a `reqwest` remote source **never fired, under any tested condition**. The null result held
independent of probe design (fixture vs. workspace member), the `?` operator (removed and ruled
out), the diagnosed `format!`-macro-expansion defect (removed for the SQL class specifically,
result unchanged), and workspace membership (the confound file was semantically analysed —
`analysed_rs_files=386`, no `semantic analyzer unavailable` message, using the crate's own
already-resolved `sqlx`/`reqwest`/`regex` dependencies — and the relevant queries evaluated
non-empty, `rust/sql-injection` interpreted in `12ms`, yet still returned nothing). This is a
**genuine detection gap at this CodeQL/rust-queries version, not a measurement artifact** — the
full diagnostic chain exists specifically to rule out the artifact explanations, and it did.

**What CodeQL DID deliver, recorded honestly rather than omitted because the overall verdict is
disqualifying:**
- The hardcoded-credential class (`rust/hard-coded-cryptographic-value`, a local heuristic
  parameter-name sink) fires **reliably** — reproduced identically across the redesigned,
  diagnostic, and confound runs (alert #29).
- **The extraction → semantic-analysis → alert pipeline is proven end-to-end** against this
  repository's real code, independent of the probe (alerts #28 and #29 both real, both
  investigated).
- **385/385 first-party file coverage on every run without exception** — the specific Snyk
  failure shape this whole evaluation exists to catch ("analysed 0 files" masquerading as "found
  0 issues") **cannot recur**: coverage is proven, separately from findings, every time.
- **But the one class that does fire carries a real false-positive cost**: alert #28, this
  evaluation's only hit against genuinely pre-existing repository code (not a planted probe),
  was a **test-code false positive** on triage (`## Verdict` → "Alert #28 correction" above) —
  a literal test-fixture string flagged as a hardcoded credential, not a leaked secret. The
  working class's real-world signal, on the one sample available, is 1 false positive out of 1
  triaged alert.

**Disposition: `codeql.yml` remains a NON-REQUIRED, advisory scan.** It stays wired
(`.github/workflows/codeql.yml`, `security-extended` queries, `debug: true`), runs on every
push/PR/schedule, and is **not** pinned in `.github/rulesets/protect-main-branch.json` or any
other ruleset — it catches the credential class and benefits automatically from any future
`rust-queries` rule improvements, at zero merge-gate risk, since a false-positive-prone,
gap-ridden scanner is never allowed to block a merge.

**This verdict is version-scoped and should be revisited on any CodeQL or `rust-queries`
upgrade.** The disqualifying evidence is tied to `CodeQL 2.26.3` / `rust-queries 0.1.40`
specifically (recorded throughout this document's run log); a future version that adds
`rust/sql-injection`/`rust/path-injection`/`rust/regex-injection` source recognition for
`reqwest::blocking` responses, or closes the CWE-078 gap, would warrant re-running this
evaluation's fixture (kept, per D-09, precisely for this reproducibility) rather than assuming
the disqualification still holds.

**Forward pointer, not acted on here: the Semgrep contingency's trigger condition (D-20 — "no
qualifying Rust SAST promoted") is now met.** This is recorded for whichever future plan takes
up that contingency; no Semgrep evaluation work is performed as part of this plan.

**Downstream note for plan 18-06:** because CodeQL is not promoted, 18-06's held-verdict branch
applies — no ruleset write (`.github/rulesets/protect-main-branch.json`, 44 → 45) occurs, no
`docs/src/appendix/branch-protection.md` count update occurs, and `scripts/check-workflow-triggers.sh`
Clause 3 has no new required context to resolve. 18-06's remaining scope, if any, is limited to
correcting the ruleset re-application procedure documentation itself (its other named
deliverable), not promotion.

### Per-class impossibility table

| Class | Fixture File | Why it could not fire | Citation |
|---|---|---|---|
| Hardcoded credential | `credential.rs` | `rust/hard-coded-cryptographic-value`'s sinks are crypto-API arguments (modeled `credentials-{password,key,iv,nonce,salt}` sinks) plus a heuristic for call arguments whose **parameter name** is literally `password`/`iv`/`nonce`/`salt`. `format!("Bearer {…}")` building an `Authorization` header string matches neither shape — the fixture's own header comment claiming this was "the sink shape CodeQL looks for" was incorrect. | `rust/ql/lib/codeql/rust/security/HardcodedCryptographicValueExtensions.qll` |
| Shell command injection via `sh -c` | `command_injection.rs` | No CWE-078 (OS command injection) query exists in `rust-queries` 0.1.40, and none exists upstream on `github/codeql` `main` as of this evaluation. Untestable by construction — no rule was ever capable of firing, regardless of fixture design. | `rust/ql/src/queries/security` (no `CWE-078` directory) |
| Path traversal | `path_traversal.rs` | `rust/path-injection` is a taint query whose `Source` is `ActiveThreatModelSource` **only**. A `pub fn` parameter (`caller_input: &str`) is never itself a source — the fixture contained zero source nodes for this query to start from. | `TaintedPathExtensions.qll` |
| SQL injection | `sql_injection.rs` | The **sink** was correctly modeled (`sqlx_core::query_as::query_as, Argument[0]`), but the fixture had the same missing-source problem as path traversal: CodeQL's own test for this query fires only from sources like `std::env::args()` or `reqwest::blocking::get(...)`, not a bare function parameter. | `rust/ql/lib/codeql/rust/frameworks/sqlx.model.yml`; `rust/ql/test/query-tests/security/CWE-089/sqlx.rs` |
| Feature-gated command injection (D-12 probe) | `feature_gated.rs` | Same CWE-078 gap as the unconditional command-injection class — untestable by construction, independent of feature gating. This left D-12's *finding-status* signal vacuous; only the file-reach signal (extraction, not detection) was ever informative for this class. | Same as command injection above |

**Root mechanism, common to three of the five classes:** CodeQL's default active threat model is
`remote` only (`shared/threat-models/ext/threat-model-grouping.model.yml`); `env`/`args`/`stdin`/
`file`-derived sources are grouped under `local`, which is **off by default**. The fixture's
`pub fn(caller_input: &str)` shape treated the function parameter itself as the taint origin,
which none of the wired taint queries recognize as a source under the default threat model. This
is a probe-design defect, not a scanner-capability finding.

**The query suite was not the problem.** `security-extended` was already selected
(`.github/workflows/codeql.yml`'s `queries: security-extended`); the probe run's `rules_count: 27`
matches upstream's full extended-suite rule count for Rust. Broadening the query suite would not
have helped — the gap is in what the fixture's shapes could ever trigger, not in which rules were
loaded.

### Alert #28 correction

**Alert #28 (`rust/hard-coded-cryptographic-value`, `src/core/platform/manager/user_service.rs:1582`)
was recorded in the 18-01 tracer run's evidence, and repeated in this plan's `## Run Log`,
`## Probe Result` and `## Baseline Comparison` sections above, as a "genuine first-party finding."
That characterization is corrected here.** Direct inspection of the source (confirmed against
this worktree's tree) shows the alert fires on the literal `"any-password"` passed as the first
argument to `service.verify_password("any-password", "not-a-valid-phc-hash")` at line 1582,
inside `#[tokio::test] async fn
verify_password_against_a_malformed_hash_returns_a_hash_error()`, itself inside the
`#[cfg(test)] mod tests` block that opens at line 491. **This is a test-code false positive, not
a leaked production credential**: the rule's heuristic password-parameter sink (a literal passed
to an argument named `password`) fired exactly as specified, on test fixture data, not on a real
secret.

This correction does **not** retract everything the earlier record drew from alert #28. It still
directly demonstrates that CodeQL's extraction → taint-tracking → alert pipeline runs end-to-end
against this repository's real code — the alert would not exist if the pipeline were broken.
What it no longer supports is the earlier framing that this was evidence of the
credential-detection rule "firing on real code ... not only in theory" in the sense of catching a
genuine defect — on triage, it is a false positive.

**Recorded false-positive-rate data point:** 1 alert triaged, 1 false positive → **1/1 = 100% FP
rate**, against the Promotion Criteria's ≤20% ceiling. This is an **n=1 sample** — nowhere near
sufficient to evaluate the ≤20% criterion on its own — but the single available data point points
in the unfavorable direction, and is recorded here rather than omitted. It does not, by itself,
change the "instrument-invalid" verdict above (which rests on the query-source impossibility
analysis, not on this FP rate), but it is relevant context for the D-14/D-15 observation window
this phase's later plans still depend on.

Every earlier place in this document that referred to alert #28 as a "genuine first-party
finding" (the `## Run Log`'s tracer-run and probe-run paragraphs, `## Probe Result`,
`## Baseline Comparison`) should be read with this correction applied. Those sections are left as
an accurate historical record of what was actually measured and are not rewritten line-by-line;
this correction is authoritative for how to interpret them.

## Re-Probe Criteria (pre-registered)

*Pre-registered 2026-08-25, before the redesigned fixture exists and before any re-probe run —
per the same T-18-13 discipline the original `## Promotion Criteria` followed ("threshold
retrofitted to the numbers" is the threat this guards against). Committed in the same commit as
the `## Verdict` correction above, strictly before any fixture file is touched or any dispatch
happens.*

### New rule-aligned, source-wired classes

All five classes below are redesigned so that taint originates from a **default-threat-model
`remote` source** — `reqwest::blocking::get(...).text()` — matching the shape CodeQL's own
upstream test suite uses for these exact queries, rather than treating a bare function parameter
as a source (the defect that invalidated the first probe).

1. **SQL injection:** `reqwest` response body → `format!`-interpolated query string →
   `sqlx::query_as` (the already-modeled sink, `sqlx_core::query_as::query_as, Argument[0]`).
2. **Path traversal:** `reqwest` response body → `PathBuf::join` → `std::fs::read_to_string`.
3. **Hardcoded credential:** a string literal `const` → passed as the argument to a local
   function whose parameter is named `password` (the rule's heuristic sink). The synthetic,
   low-entropy value property (D-10) is kept; gitleaks must still pass unmodified.
4. **Regex injection (`rust/regex-injection`):** `reqwest` response body →
   `regex::Regex::new(...)`. This replaces one of the two untestable command-injection slots with
   a rule that actually exists upstream.
5. **D-12 feature-gated variant:** identical shape to class 1 (SQL injection — now a genuinely
   detectable class), planted behind the `probe-feature-gated` cargo feature, exactly as the
   original fifth defect was structured.

### Known-gap register row (recorded, not scored)

**Command injection (CWE-078) has no upstream Rust query as of this evaluation.**
`command_injection.rs` (the original `sh -c` shell command injection fixture) is **kept in the
crate as documentation of this gap** — it remains a real, plausible Rust vulnerability shape this
evaluation cannot test, because CodeQL cannot test it, not because the fixture is malformed. It
is **not scored** in the qualification arithmetic below; its continued 0-finding result is
expected and uninformative, not evidence of anything.

### Scoring (pre-registered, before any new number exists)

- **Disqualifying:** fewer than 2 of the 4 ungated, scoreable classes (SQL injection, path
  traversal, hardcoded credential, regex injection) alert.
- **Qualifying:** 3 or more of the 4 ungated, scoreable classes alert (any miss is documented
  per-rule, naming which class and why).
- **Hard disqualifier, unchanged from the original criteria:** 0 findings across all four
  scoreable classes disqualifies the tool outright (D-11's shape recurring) — this floor is not
  relaxed by the redesign.
- **Carried over unchanged from the original `## Promotion Criteria`:** wall-clock ceiling ≤600s
  per run; minimum analysed-file count ≥95% of the 385 denominator.
- **Additional evidence goal:** any fixture finding on this redesigned probe also proves the
  workspace-excluded, standalone `fixtures/codeql-probe` crate is **semantically analysed** (real
  taint-tracking facts produced inside it), not merely **archived** into the extraction database —
  closing the archiving≠analysis caveat recorded in `## Analysis Coverage` above (the original
  probe's `probe_fixture_entries=6` proved presence in the source set, never proved semantic
  analysis, since nothing in it could ever alert).

## Re-Probe Result

**Run `32884197028`** (`eval/codeql-probe`, commit `f9bc44cb`, `workflow_dispatch` with
`scan_probe_fixture=true`, conclusion `success`, 181s). Alerts read via
`gh api "/repos/DF3NDR/paladin-dev-env/code-scanning/alerts?ref=refs/heads/eval/codeql-probe&tool_name=CodeQL&per_page=100"`
(HTTP 200), cross-checked directly against the run's own raw SARIF (`rust.sarif` inside the
`debug-artifacts` artifact) — both report exactly **2 results**, so nothing was lost to
pagination or alert-store dedup.

**Coverage established before interpreting the result:** `scripts/codeql-analysed-files.sh
32884197028` reports `probe_fixture_entries=7` (all seven fixture files — `lib.rs`,
`sql_injection.rs`, `path_traversal.rs`, `credential.rs`, `regex_injection.rs`,
`command_injection.rs`, `feature_gated.rs`) confirmed present in the extracted source set,
`analysed_rs_files=385`/`difference=0` unchanged. Full output:

```
run_id=32884197028
analysed_rs_files=385
denominator=385
difference=0
probe_fixture_entries=7
feature_gated_present=src/infrastructure/web/mod.rs:yes
feature_gated_present=src/application/cli/commands/agent.rs:yes
feature_gated_present=crates/paladin-web/src/lib.rs:yes
src_zip_total_rs_entries=3441
src_zip_checkout_rs_entries=564
src_zip_toolchain_stdlib_rs_entries=2874
src_zip_other_vendored_rs_entries=3
```

### Per-class result

| Class | Fixture File | Alert Raised | Rule ID | Scored? |
|---|---|---|---|---|
| SQL injection | `sql_injection.rs` | No | — | Yes |
| Path traversal | `path_traversal.rs` | No | — | Yes |
| Hardcoded credential | `credential.rs` | **Yes** — alert #29, `fixtures/codeql-probe/src/credential.rs:25` | `rust/hard-coded-cryptographic-value` | Yes |
| Regex injection | `regex_injection.rs` | No | — | Yes |
| Shell command injection (known gap) | `command_injection.rs` | No | — | **No — unscored** (no upstream CWE-078 query) |
| D-12 feature-gated (SQL-injection variant) | `feature_gated.rs` | No | — | Yes (D-12 signal only, see below) |

**1 of 4 scoreable classes alerted.** Alert #29 (`rust/hard-coded-cryptographic-value`,
`credential.rs:25`) is a genuine detection of the redesigned heuristic-sink shape (a hardcoded
literal passed to a locally-defined function parameter named `password`) — not a false
positive, not a pre-existing alert; it did not exist before this run. Alert #28
(`user_service.rs:1582`, the corrected test-code false positive from the original probe) is
also present on this ref, as expected, and is excluded from this table and from the
per-class count by path.

### Extraction-mechanism findings (from the run's own debug logs, not inferred)

Before scoring the three non-firing classes as a capability finding, the run's own
`rust/log/database-index-files-*.log` was inspected directly for extraction-level anomalies —
the same discipline that surfaced the first probe's instrument-invalidity, applied again here
rather than assumed absent:

1. **SQL injection (`sql_injection.rs:29`) has a diagnosed extraction failure, not merely a
   silent non-detection.** The log records: `WARN
   .../fixtures/codeql-probe/src/sql_injection.rs:29:9: macro expansion failed for 'format'` —
   line 29 is exactly the fixture's `format!("SELECT id, name FROM users WHERE name =
   '{untrusted_username}'")` call, the taint step this class's entire sink shape depends on. If
   the macro's expansion is unresolved, the dataflow edge from the tainted `reqwest` response
   through the interpolated string into `sqlx::query_as`'s argument may never be constructed,
   independent of whether the underlying `rust/sql-injection`-family query can otherwise detect
   this pattern.
   - **This is not isolated to the redesigned fixture.** The same warning, at the same kind of
     `format!` call, appears for `command_injection.rs:27` (`let shell_command = format!("echo
     {caller_input}")`) — a file this continuation did **not** modify. Both fixture files use
     Rust's inline-captured-identifier interpolation syntax (`format!("...{ident}...")` rather
     than positional `format!("...{}...", ident)`).
   - **This warning is systemic across the whole checkout, not fixture-specific**: it occurs
     **889 times** in this run's log, all under first-party checkout paths (not just the
     fixture). Whether it reflects a genuine, repository-wide CodeQL Rust extractor limitation
     around `format!` macro expansion, or a narrower interaction specific to the
     inline-capture syntax, was not further isolated in this continuation — that would require
     an additional, targeted fixture variant (e.g. positional-args `format!`) and is not
     pre-registered scope here.
2. **Path traversal (`path_traversal.rs`) and regex injection (`regex_injection.rs`) extracted
   cleanly — no warnings, no errors, in either file's `LoadSource`/`Parse`/`Extract` log
   entries.** Their non-firing has no diagnosed extraction-level cause on this evidence; it may
   reflect a genuine gap in how `rust/path-injection` / `rust/regex-injection` recognize
   `reqwest::blocking::get(...).text()` specifically (as opposed to some other remote-source
   shape), or something else not yet isolated. This is a materially different, more legitimate
   kind of non-detection than SQL injection's diagnosed macro-expansion failure — it is not
   itself proven to be instrument-invalid, but it is also not proven to be a clean capability
   finding; no further isolation was performed within this continuation's scope.
3. **D-12's feature-gated variant (`feature_gated.rs`) was archived but explicitly NOT
   semantically analysed on this run**, per the log's own words: `INFO
   .../fixtures/codeql-probe/src/feature_gated.rs:1:1: semantic analyzer unavailable (not
   included as a module): macro expansion will be skipped.` This refines D-12's answer beyond
   what either the original probe or 18-01's tracer run established. **File-reach and semantic
   analysis are two different claims, and this run separates them directly**: `feature_gated.rs`
   IS present in the extracted source set (one of the 7 `probe_fixture_entries`, same
   file-extension-based archiving mechanism as always) — but rust-analyzer's own
   module-inclusion pass, which determines what actually gets semantically analysed for taint
   facts, explicitly excludes it when `probe-feature-gated` is not the active feature set for
   this scan. **This is the closest empirical answer yet to whether buildless CodeQL genuinely
   analyses this workspace's many feature-gated subsystems (`vision`, `web-server`,
   `llm-openai`/`anthropic`/`deepseek`, `redis-queue`, `s3-storage`, `storage-mysql`, `qdrant`,
   `cli`, `notifications`, etc.) under any single scan run: extraction reaches them (the file is
   archived), but semantic analysis of a `#[cfg(feature = "...")]`-gated module is skipped
   unless that specific feature happens to be active during that particular invocation** — and
   `codeql.yml` runs with exactly one default feature set per invocation, not a matrix over
   every feature combination.

### Scoring against the pre-registered `## Re-Probe Criteria`

Applying the criteria literally, as written before this run existed: **1 of 4 scoreable classes
alerted**, which is fewer than the pre-registered qualifying floor of 3 and meets the
pre-registered disqualifying condition of fewer than 2. On the literal text of the
pre-registered scoring, this result is **disqualifying**.

**This literal scoring is presented alongside the mechanism findings above, not instead of
them, because one of the three non-firing classes has a diagnosed extraction-level cause
(SQL injection's `format!`-macro-expansion failure) rather than a clean non-detection — the
same category of evidence (a run-log-diagnosed mechanism defect, not an inference from the
zero result) that justified correcting the first probe's verdict from "disqualified" to
"instrument-invalid."** Whether that one diagnosed defect is enough to again call the
instrument invalid, whether the two cleanly-extracted-but-non-firing classes (path traversal,
regex injection) should be treated as genuine capability findings or as still-unexplored gaps,
and whether the D-12 semantic-analysis-exclusion finding changes the shape of any eventual
qualification, are **not decided here**. Wall-clock (181s) and analysed-file coverage
(385/385, 100%) both comfortably clear their carried-over ceilings and are not disqualifying on
their own.

## Diagnostic Iteration (pre-registered)

*Pre-registered 2026-08-25T19:20:57Z, before the diagnostic fixture variant exists and before
any dispatch — same T-18-13 discipline as the original `## Promotion Criteria` and the
`## Re-Probe Criteria` above. The user, at the second checkpoint, authorized exactly one more
diagnostic iteration to isolate why 3 of 4 scoreable classes missed on the redesigned fixture
(`## Re-Probe Result`), then a final verdict — not an open-ended retry loop.*

### Orchestrator-verified tool-side facts (not re-derived here, cited as given)

For path traversal and regex injection specifically, the source, propagation and sink models
are all confirmed present at the deployed CodeQL CLI version (`v2.26.3`):
`blocking::get`'s `Ok` return field and `blocking::Response::text`'s summary as sources;
`Path::join`'s taint summary (`stdlib/fs.model.yml` lines 64-65) as propagation;
`std::fs::read_to_string Argument[0]` (`path-injection` sink) and
`<regex::regex::string::Regex>::new` (`NewSink`) as sinks. **Both misses are therefore
tool-side, with an undiagnosed cause** — not a missing model, which rules out the simplest
explanation and motivates this diagnostic.

### Two single-cause hypotheses, pre-stated before this run

Two hypotheses each fully explain the observed 1/4 pattern from `## Re-Probe Result` on their
own:

- **(a) Taint does not survive the `?` operator.** All three missing classes (SQL injection,
  path traversal, regex injection) unwrap `Result`s via `?` on the taint path
  (`reqwest::blocking::get(lookup_url)?.text()?`, and again on the sink call). CodeQL's own
  passing upstream tests for these queries use `.unwrap()` / `.unwrap_or(...)` instead. The one
  class that *did* fire (hardcoded credential) involves no `Result`/`?` at all on its taint
  path.
- **(b) External-crate call resolution fails inside this nested, workspace-excluded crate.**
  All three misses need a `reqwest`/`sqlx`/`regex` canonical path resolved by CodeQL's Rust
  extractor to match the model library's qualified names; the one firing class needs only a
  local, same-crate function call, no external-crate resolution at all.

The diagnosed `format!`-macro-expansion failure (`sql_injection.rs:29`, 889× checkout-wide,
recorded in `## Re-Probe Result`) is treated as an **additional, compounding defect specific to
the SQL class only** — it does not explain path traversal's or regex injection's misses, which
involve no `format!` call.

### What changes in this diagnostic variant

The three missing classes, plus the feature-gated variant (which mirrors SQL injection), are
rewritten to match CodeQL's own upstream test idioms **exactly**, isolating hypothesis (a) as
the single changed variable per class:

- Every `?` on the taint path is replaced with `.unwrap()` (on `reqwest::blocking::get(...)`)
  and `.unwrap_or_default()` (on `.text()`) — matching the unwrap/unwrap_or shape CodeQL's own
  passing tests use, rather than early-return error propagation.
- **`sql_injection.rs` additionally replaces `format!` with string concatenation**
  (`String::from("SELECT id, name FROM users WHERE name = '") + &untrusted + "'"` — the
  `unsafe_query_3` shape from CodeQL's own upstream sqlx test), so the SQL class isolates the
  `?`-operator variable from the already-diagnosed `format!`-macro-expansion defect. If SQL
  injection still misses on this variant, the miss cannot be blamed on `format!` — it must be
  hypothesis (a), (b), or both.
- `path_traversal.rs` and `regex_injection.rs` keep their existing (non-`format!`) sink shapes;
  only the `?` unwrapping changes.
- `feature_gated.rs` mirrors the new SQL-injection shape (concatenation + unwrap), unchanged in
  its feature-gating.

### Interpretation, pre-stated before any number exists

- **A class flips from missing to firing** → implicates hypothesis (a) for that class (taint
  broke on `?`-style error handling), and for SQL specifically also confirms the `format!`
  defect as compounding rather than solely causal (since removing it, alongside removing `?`,
  is what let it fire).
- **A class still misses on this variant** → implicates hypothesis (b) (external-crate
  resolution) or a deeper, still-undiagnosed extraction failure — `?` is no longer a live
  explanation for that class once removed.
- **No fixture-surgery beyond this single change-set is authorized.** If a class still misses,
  the debug-artifact logs are grepped for unresolved-path / type-inference / semantic-analyzer
  warnings naming the fixture files or `reqwest`/`sqlx`/`regex` resolution, and whatever is
  found is recorded as (b)-hypothesis evidence — not chased with a further fixture rewrite or a
  workspace-membership variant. One iteration, then the final checkpoint.

### Scoring, pre-registered: this run's table supersedes, both are recorded

**The diagnostic run's per-class results supersede the second probe's (`## Re-Probe Result`)
for scoring against the `## Re-Probe Criteria` thresholds** (unchanged: fewer than 2 of 4
scoreable classes alerting is disqualifying, 3 or more is qualifying, 0 is a hard
disqualifier) — because the unwrap/unwrap_or idiom is realistic vulnerable Rust (not a
contrivance) and matches CodeQL's own test corpus shape more closely than the `?`-based
redesign did. **Both runs' per-class tables are kept side by side in the record; the earlier
table is not deleted or overwritten.**

**If the diagnostic qualifies on unwrap-shapes, the `?`-operator sensitivity discovered here is
recorded as a first-class false-negative limitation, not silently absorbed into a passing
score.** The `?` operator is the dominant Rust error-handling idiom in real code — including
this codebase's own production paths — so a scanner that only detects taint through
`.unwrap()`/`.unwrap_or(...)` chains has a materially narrower real-world detection surface
than a qualifying score alone would suggest. This limitation is carried forward to the final
checkpoint for the user to weigh explicitly, regardless of which way the literal score lands.

## Diagnostic Iteration Result

**Run `32890183115`** (`eval/codeql-probe`, commit `c7e3bc84`, `workflow_dispatch` with
`scan_probe_fixture=true`, conclusion `success`, 188s). Alerts read via
`gh api "/repos/DF3NDR/paladin-dev-env/code-scanning/alerts?ref=refs/heads/eval/codeql-probe&tool_name=CodeQL&per_page=100"`
(HTTP 200), cross-checked directly against the run's own raw SARIF (`rust.sarif`) — both
report exactly **2 results**, identical in rule, path and line to the second probe's alerts:
alert #29 (`credential.rs:25`) and the pre-existing corrected alert #28.

**Coverage established before interpreting the result:** `scripts/codeql-analysed-files.sh
32890183115` reports `probe_fixture_entries=7` (unchanged), `analysed_rs_files=385/385`
(unchanged).

### Per-class result: second-probe table vs. diagnostic table, side by side

| Class | Second Probe (`?`, `format!`, run `32884197028`) | Diagnostic (`.unwrap()`/concatenation, run `32890183115`) |
|---|---|---|
| SQL injection | No | **No — unchanged** |
| Path traversal | No | **No — unchanged** |
| Hardcoded credential | **Yes** (#29) | **Yes — unchanged** (#29, same location) |
| Regex injection | No | **No — unchanged** |
| Command injection (known gap) | No (unscored) | No (unscored) — file untouched |
| D-12 feature-gated variant | No | **No — unchanged** |

**1 of 4 scoreable classes alerted on this run too — identical to the second probe's result.**
No class flipped from missing to firing when every `?` on the taint path was replaced with
`.unwrap()`/`.unwrap_or_default()` and `sql_injection.rs`'s `format!` was replaced with string
concatenation.

### Diagnosis against the pre-registered hypotheses

**Hypothesis (a) — taint does not survive the `?` operator — is ruled out.** Every `?` on
every missing class's taint path was removed in this variant, matching CodeQL's own upstream
`.unwrap()`/`.unwrap_or(...)` test idiom exactly, and the result did not change. `?`-style
error handling is not the (or not the sole) cause of these three classes missing.

**The `format!`-macro-expansion defect is confirmed resolved for the SQL class, independent of
the miss.** This run's `rust/log/database-index-files-*.log` shows the `macro expansion
failed for 'format'` warning **only** for the untouched `command_injection.rs:27` — it no
longer appears for `sql_injection.rs`, which now uses string concatenation. SQL injection
still misses despite this defect being removed, so the `format!` defect was compounding, not
solely causal, exactly as `## Diagnostic Iteration (pre-registered)` anticipated for that
outcome.

**No diagnosable extraction-level cause was found in the logs for any of the three still-missing
classes** — no `WARN`/`ERROR` line anywhere in `database-index-files`, `database-trace-command`,
or `dataset-import` mentions `sql_injection.rs`, `path_traversal.rs`, `regex_injection.rs`, or
any `reqwest`/`sqlx`/`regex` resolution failure. All three files extracted cleanly
(`LoadSource`/`Parse`/`Extract` steps present, no warnings), same as the second probe's
path-traversal and regex-injection results. `feature_gated.rs` again shows the `semantic
analyzer unavailable (not included as a module): macro expansion will be skipped` message,
confirming the D-12 semantic-analysis-exclusion finding holds unchanged in this run too.

**One additional, sub-warning-level signal from `database-interpret-results-*.log`:** all three
missing queries ran to completion and produced legitimately empty result sets (not crashes, not
timeouts) — `rust/regex-injection` and `rust/path-injection` both interpreted in `0ms`
(essentially instantaneous, consistent with an empty relation), while `rust/sql-injection` took
`13ms` (measurably nonzero, unlike the two 0ms queries, though still far short of producing an
alert). This is not itself a diagnosis — it is recorded as the most granular evidence this
iteration's log inspection could surface, consistent with hypothesis (b) (external-crate call
resolution not matching the model library's qualified names for these specific call shapes) or
an unidentified, deeper dataflow-modeling gap, without being conclusive proof of either.

**Per the pre-registered protocol, no further fixture surgery or workspace-membership variant
was attempted.** This is the single authorized diagnostic iteration; its result — hypothesis
(a) ruled out, hypothesis (b) or a deeper gap unconfirmed but not ruled out, `format!` confirmed
compounding-not-causal for SQL — is reported as-is.

### Scoring against the pre-registered criteria (this table supersedes the second probe's, per `## Diagnostic Iteration (pre-registered)`)

**1 of 4 scoreable classes alerted, same as the second probe.** Per the pre-registered scoring
(fewer than 2 of 4 is disqualifying, 3 or more is qualifying), this **literally scores as
disqualifying**, identically to the second probe. Wall-clock (188s) and analysed-file coverage
(385/385) both clear their ceilings and are not disqualifying on their own.

**What is different after this iteration, compared to before it:** the `?`-operator hypothesis
that could have explained the misses as an artifact of unrealistic-for-CodeQL-testing fixture
code is now ruled out — the misses persist under the tool's own preferred test idiom. This
removes one candidate benign explanation and leaves the miss pattern **more**, not less,
consistent with a genuine detection gap (or an unconfirmed external-crate-resolution defect)
rather than a fixture-authoring artifact. The one class that fires (hardcoded credential)
continues to demonstrate the extraction → semantic-analysis → alert pipeline works end-to-end
on this exact fixture crate, under the `.unwrap()` idiom, ruling out "the whole crate is
somehow invisible to analysis" as an alternative explanation for the other three.

## Workspace-Member Confound Test (pre-registered)

*Pre-registered 2026-08-25T19:49:41Z, before the workspace-member experimental code exists and
before any dispatch. The user, at the third checkpoint, explicitly authorized this one further
diagnostic and explicitly lifted the prior "no workspace-membership variant" scope guard for
this purpose only.*

### The question this settles

Are the SQL-injection / path-traversal / regex-injection misses (unchanged across both the
redesigned and diagnostic fixture variants — `## Re-Probe Result`, `## Diagnostic Iteration
Result`) a **genuine CodeQL tool gap** for these source/sink shapes, or an **artifact of the
probe fixture being a nested, workspace-EXCLUDED, standalone Cargo crate** whose external-crate
canonical paths (`reqwest`/`sqlx`/`regex`/`std::fs`) may not resolve identically to how they
resolve inside an ordinary, in-workspace member during buildless extraction? The `0ms`
empty-relation interpretation of `rust/path-injection` and `rust/regex-injection` in the
diagnostic run (`## Diagnostic Iteration Result`) is consistent with their sinks never being
recognized at all — which an external-crate resolution failure specific to the excluded
fixture's nested-workspace shape would explain. The one class that fires in the fixture
(hardcoded credential) needs only a **local** function call, no external-crate sink resolution
— a structurally different requirement from the three that miss.

### What: identical shapes, planted in an ordinary workspace member

The three still-missing classes are planted as functions inside the root `paladin` crate
(`src/`), an **ordinary, already-scanned workspace member** — not a new crate, not excluded
from the workspace, not workspace-excluded in any way. This crate already depends on all three
needed externals for real, shipped reasons, so no new dependency is added and every canonical
path the planted code calls is identical to what steady-state code in this crate already
resolves:

- `reqwest` — root `Cargo.toml` line 71: `reqwest = { workspace = true, features =
  ["blocking", "stream"] }` (blocking feature already present, same as the fixture uses).
- `sqlx` — root `Cargo.toml` line 117: `sqlx = { workspace = true }` (workspace default
  features include `sqlite`, same backend the fixture uses).
- `regex` — root `Cargo.toml` line 115: `regex = "1.11.1"` (identical version the fixture
  declares).
- `std::fs` — no dependency needed, standard library.

Each planted function reuses the exact diagnostic-idiom shape from `## Diagnostic Iteration
(pre-registered)` (`.unwrap()`/`.unwrap_or_default()` instead of `?`, string concatenation
instead of `format!` for the SQL class) — the only changed variable is workspace membership,
not source/sink shape, not error-handling idiom, not query version. This is the correct
single-variable experimental design: everything that could confound the result except
workspace membership is held constant against the diagnostic run.

### Hard safety invariant (binding on execution, not just this record)

The vulnerable workspace-member code exists **only** on the throwaway `eval/codeql-probe`
branch. It is:
- **Never** committed to this plan's mergeable worktree branch history.
- **Never** merged toward `chore/18-rust-sast-codeql` or any branch that reaches `main`.
- **Never** referenced from real/shipped code paths — no `mod` declaration reachable from any
  production entry point survives past the experiment.
- Confined by committing it on a separate, throwaway local branch (not the worktree's own
  `worktree-agent-*` branch), pushing only that throwaway branch's tip to
  `refs/heads/eval/codeql-probe`, then discarding the local throwaway branch entirely once
  evidence is captured — so the mergeable branch's own commit history never contains the
  vulnerable code at any point, not even transiently.
- The remote `eval/codeql-probe` branch is deleted after evidence capture, exactly as after
  every prior probe run on this branch.
- This document's own record of what was done (the exact steps taken, the local branch name,
  confirmation commands run) is committed separately, on the mergeable branch, and contains no
  vulnerable code itself — only its description and results.

### Interpretation, pre-stated before any number exists

- **A class FIRES as a workspace member but MISSED as the excluded fixture** → the miss was a
  workspace-exclusion / external-crate-resolution artifact, not a tool capability gap, for that
  class. Evidence leans toward CodeQL genuinely detecting this class in real, in-workspace Rust
  code — the fixture's own workspace-exclusion design (itself required by D-07/D-11 to keep
  deliberately-vulnerable code out of the real build graph) would then be understood as having
  introduced its own measurement artifact for these three classes.
- **A class MISSES in BOTH** the workspace-member and the excluded-fixture form → a genuine
  tool gap for that specific source/sink shape at this CodeQL version, independent of workspace
  membership. The workspace-exclusion hypothesis is ruled out for that class specifically.

### Scoring (pre-registered): deciding for the confound, not for SAST-01

**This is the deciding measurement for the workspace-exclusion confound** — it settles which of
the two explanations (tool gap vs. measurement artifact) applies to each of the three classes.
**It does not, by itself, set the final SAST-01 verdict** — that remains the user's call at the
next checkpoint, informed by this result alongside every prior run's evidence. The record here
is: how many of the 3 classes (SQL injection, path traversal, regex injection) fire as workspace
members, named individually, not collapsed into a single pass/fail number.

## Workspace-Member Confound Test Result

### Execution record (hard safety invariant compliance)

The experimental workspace-member code was created, committed, pushed, scanned and discarded
exactly as pre-registered:

1. Created `src/codeql_workspace_probe.rs` (three functions: SQL injection, path traversal,
   regex injection — identical shapes to the diagnostic fixture variant, using the root
   `paladin` crate's own real `reqwest`/`sqlx`/`regex` dependencies) and a temporary `mod
   codeql_workspace_probe;` declaration in `src/lib.rs`.
2. **Both were committed on a separate local branch, `scratch-eval-workspace-member-throwaway`,
   created from this plan's own mergeable branch tip (`d27d4c33`) — never on the mergeable
   branch itself.** One fix was needed and applied on that same scratch branch (amended, not a
   new commit, since the branch is entirely throwaway): `String::from(...) + &untrusted_username`
   does not resolve under this workspace's full `--all-features` clippy invocation the way it
   does in the fixture crate's isolated dependency graph (compiles standalone; fails under the
   full workspace feature union, per a `cargo clippy --workspace --all-targets --all-features`
   run against the pre-push hook's exact flags) — switched to `.as_str()` for an unambiguous
   `&str` conversion. Final scratch commit: `70efbe23`.
3. **Only that scratch branch's tip was pushed to `refs/heads/eval/codeql-probe`** — `git push
   origin HEAD:refs/heads/eval/codeql-probe` executed while `HEAD` was the scratch branch, not
   the mergeable one.
4. `codeql.yml` dispatched against `eval/codeql-probe` with `scan_probe_fixture=true`, watched
   to completion (run `32894118236`), evidence captured (below).
5. **Remote `eval/codeql-probe` deleted** (`git push origin --delete eval/codeql-probe`).
6. **`git checkout` back to the mergeable `worktree-agent-a328ac09cefd2c593` branch** — this
   branch's own history was never touched by any of steps 1–3; its tip remained `d27d4c33`
   throughout.
7. **Local scratch branch deleted** (`git branch -D scratch-eval-workspace-member-throwaway`),
   removing the last local ref pointing at the vulnerable commit.
8. **Verification, run and recorded here:**
   - `git status --short` on the mergeable branch: clean, no output.
   - `test -f src/codeql_workspace_probe.rs`: absent from the working tree.
   - `grep -rn "codeql_workspace_probe" src/lib.rs`: no match (exit 1).
   - `grep -rn` for all three confound function names
     (`confound_sql_injection_from_remote_lookup`, `confound_path_traversal_from_remote_lookup`,
     `confound_regex_injection_from_remote_lookup`) across the entire repository tree: no match
     (exit 1) — the code does not exist anywhere in the checked-out mergeable tree.
   - `git log --all --oneline | grep -i THROWAWAY`: no match (exit 1) — no ref reachable from
     this worktree's local repository retains the throwaway commit.
   - `git log --oneline -10` on the mergeable branch: tip is `d27d4c33` (this section's own
     pre-registration commit), confirming the branch's history is exactly what it was before
     the experiment began, plus this results commit.

### Per-class result

| Class | Excluded Fixture (both variants) | Workspace Member (run `32894118236`) |
|---|---|---|
| SQL injection | No (both `?` and `.unwrap()` variants) | **No — unchanged** |
| Path traversal | No (both variants) | **No — unchanged** |
| Regex injection | No (both variants) | **No — unchanged** |
| (Hardcoded credential, for reference — not part of this confound) | Yes (#29) | n/a, not replanted here |

**0 of 3 classes fired as workspace members.** Alerts on this run: alert #29
(`fixtures/codeql-probe/src/credential.rs:25`, the excluded fixture's still-present, unmodified
credential class) and the pre-existing corrected alert #28. Cross-checked directly against the
run's own raw SARIF — exactly 2 results, identical rule/path/line to every prior run, confirming
no pagination or dedup loss. Nothing on `src/codeql_workspace_probe.rs` anywhere in the alert
store or the raw SARIF.

### Coverage and extraction evidence for the confound file specifically

`scripts/codeql-analysed-files.sh 32894118236`: `analysed_rs_files=386` (385 + the new confound
file), `probe_fixture_entries=7` (unchanged). The run's own `rust/log/database-index-files-*.log`
shows `src/codeql_workspace_probe.rs` extracted cleanly:
`LoadSource` → `archived` → `Parse` → `creating trap file` → `Extract`, all at `ms: 0`, with
**no** `macro expansion failed` warning (expected — no `format!` in this file) and **no**
`semantic analyzer unavailable (not included as a module)` message (unlike `feature_gated.rs`
in every prior run) — this file **was** included as a real, always-compiled module and
semantically analysed, not skipped. `rust/log/database-interpret-results-*.log` shows the same
non-zero-but-empty pattern as the diagnostic run: `rust/sql-injection` interpreted in `12ms`,
`rust/path-injection` and `rust/regex-injection` each in `1ms` — all completed, all zero
results. No `WARN`/`ERROR` line anywhere in the extraction logs names
`codeql_workspace_probe.rs` or reports a `reqwest`/`sqlx`/`regex` resolution failure.

### Diagnosis against the pre-registered interpretation

**Per the pre-registered interpretation: 0 of 3 classes flipped from missing to firing when
replanted as an ordinary, already-scanned workspace member using the crate's own real
dependencies.** All three classes **miss in both** the workspace-member and the excluded-fixture
form. Per the pre-registered reading, this result is consistent with **a genuine tool gap for
these specific source/sink shapes at this CodeQL version, independent of workspace membership**
— the workspace-exclusion / external-crate-resolution hypothesis (b) is not supported by this
measurement for any of the three classes. The confound file's clean extraction, its confirmed
semantic-module inclusion (unlike the fixture's `feature_gated.rs`), and its use of the exact
same already-resolved `reqwest`/`sqlx`/`regex` canonical paths that steady-state code in this
crate already relies on, together rule out "the confound file itself was invisible or
unresolved to CodeQL" as an alternative explanation for the null result.

**What remains outside this measurement's power to settle:** whether a *different* remote-source
shape (not `reqwest::blocking::get(...).text()`), a *different* library version, or a future
CodeQL Rust release would change this outcome. This confound test rules out workspace membership
and external-crate path resolution as the explanation for these three specific misses on this
specific CodeQL version (`2.26.3`, `rust-queries` `0.1.40`) — it does not, and was not
pre-registered to, make a claim beyond that scope.

## Promotion Status

**Decision: `hold-advisory`, recorded 2026-08-25.** `CodeQL Analysis (Rust)` is **not** promoted
to a required status check on `main`. `.github/rulesets/protect-main-branch.json` keeps its
44-entry `required_status_checks` array unchanged, the live ruleset (id `20868126`) is not
re-applied, and `docs/src/appendix/branch-protection.md`'s required-check count and context list
are untouched — there is nothing to reconcile across those places because none of them changed.

This is not a deferral: it is the outcome the `## Promotion Criteria` (written 2026-08-25, before
any measurement existed) compares against, and the measured numbers do not clear it. `codeql.yml`
runs on every push/PR/schedule and continues to surface findings (the credential class, and any
future `rust-queries` rule improvements) in the code-scanning UI, but never blocks a merge.

**Measured numbers this decision rests on** (full detail in `## Verdict` and its subsections
above):

- **False-positive rate: 1 triaged alert, 1 false positive → 100% FP rate** (alert #28, a
  test-fixture literal misclassified as `rust/hard-coded-cryptographic-value`; see `## Verdict` →
  "Alert #28 correction"), against the Promotion Criteria's ≤20% ceiling — an n=1 sample, but the
  only real-code data point available, and it is on the wrong side of the ceiling.
- **Wall-clock: every recorded run landed between 168s and 223s** (cold-cache tracer run
  `32868842656`: 212s; steady-state run `32889890607`: 168s; warm-cache probe runs ranged 181s–223s
  — see `## Run Log`), comfortably inside the ≤600s ceiling. Wall-clock was never the blocker.
- **Analysed-file coverage: 385/385 (100%) on every run without exception** (`## Analysis
  Coverage`, `## Re-Probe Result`, `## Diagnostic Iteration Result`), comfortably inside the ≥95%
  floor. Coverage was never the blocker.
- **Disqualifying condition (D-11) triggered: the redesigned, rule-aligned probe scored 1 of 4
  scoreable classes** (SQL injection, path traversal, hardcoded credential, regex injection —
  `## Re-Probe Result`, reconfirmed unchanged by the diagnostic iteration and the workspace-member
  confound test), against the pre-registered qualifying floor of 3 of 4 and the disqualifying
  ceiling of fewer than 2 of 4. This is the actual blocker: the false-positive-rate and wall-clock
  ceilings are secondary to a scanner that only detects 1 of 4 rule-aligned, source-wired classes
  at this CodeQL/`rust-queries` version, per the `disqualified (version-scoped: CodeQL 2.26.3 /
  rust-queries 0.1.40)` verdict.

See `## Open Item — Promotion Held` below for the trigger condition, owner, and revisit date that
make this a settled outcome rather than a silent deferral.

## Open Item — Promotion Held

**Threshold not met:** the `## Re-Probe Criteria`'s qualifying floor — at least 3 of the 4
rule-aligned, source-wired scoreable classes (SQL injection, path traversal, hardcoded credential,
regex injection) must alert for the redesigned probe to qualify.

**Measured result: 1 of 4.** Only the hardcoded-credential class fired (alert #29). This result
was reconfirmed unchanged across three independent measurements after the initial redesigned
probe: the diagnostic iteration (`.unwrap()`/string-concatenation idiom, ruling out the `?`
operator as the cause), and the workspace-member confound test (identical shapes planted in the
real, already-scanned `paladin` crate, ruling out fixture workspace-exclusion as the cause). See
`## Re-Probe Result`, `## Diagnostic Iteration Result`, and `## Workspace-Member Confound Test
Result` for the full per-class detail.

**Trigger condition:** re-run this evaluation's fixture (retained per D-09, specifically for
reproducibility) against a future CodeQL engine / `rust-queries` release whose release notes report
added or improved `rust/sql-injection`, `rust/path-injection`, or `rust/regex-injection` source
recognition for `reqwest::blocking` response bodies, or that otherwise states it closes the
detection gap measured here at CodeQL `2.26.3` / `rust-queries` `0.1.40` (see `## Verdict`). A bare
CodeQL version bump with no stated change to these query families' source modeling is not
sufficient on its own to re-open this item — the trigger is a specific capability claim in the
upstream changelog, not the passage of time or a version number alone.

**Owner:** Am0rfu5 (repository maintainer).

**Revisit date:** 2027-02-25 (six months from this decision, 2026-08-25), or immediately upon the
trigger condition above being met, whichever comes first.

## Observation Window

### Not Applicable — Promotion Not Pursued

The D-14/D-15 observation-window measurement — a backfill table over sampled historical commits,
a live advisory period over this phase's own pull requests, and the complete D-15 metric block
(false-positive rate, cold/warm-cache wall-clock, analysed-file coverage) — is **NOT APPLICABLE**
and was **not performed**.

**Why.** `18-05-PLAN.md`'s Task 1 carries an explicit precondition: `18-CODEQL-EVIDENCE.md`'s
`## Verdict` section must record `qualified` or `qualified-with-coverage-gap` before the backfill
work begins. The `## Verdict` section above records the opposite: **`disqualified`
(version-scoped: CodeQL `2.26.3` / `rust-queries` `0.1.40`)**, with `codeql.yml` retained
advisory-only and explicitly **not** promoted to a required check (see `## Verdict` and
`## Promotion Status` above). The precondition is unmet, and it is unmet by the settled outcome
of the evaluation this phase exists to perform — not by an oversight or a blocked prerequisite
that could be satisfied by re-running something.

This plan's entire purpose was to produce the noise and latency numbers a *required check* would
be pinned on before promotion — its own objective states "SAST-03 forbids promoting an unmeasured
scanner; the promotion in 18-06 reads only what this plan records." With SAST-01 disqualified and
`18-06`'s promotion path not taken (per `## Verdict`'s "Downstream note for plan 18-06"), there is
no required check to baseline. Measuring backfill noise and CI wall-clock cost for a check that
will never gate a merge would produce numbers nobody reads and that inform no decision — the
measurement is moot, not merely postponed.

**What was not done, explicitly.** No historical commits were sampled or backfilled; no
`tmp/codeql-backfill-*` branches were pushed or deleted; no live advisory period was recorded; no
new alerts were triaged beyond the ones already dispositioned in `## Verdict`'s "Alert #28
correction"; no entries were added to `CODEQL-DISMISSALS.md` under this plan; no `git log`
sampling command was run. None of Task 1's or Task 2's acceptance criteria were attempted.

**Authorization.** This was decided by the user, not inferred by the executor, at the `18-03`
verdict checkpoint on 2026-08-25 — the same checkpoint that settled the `disqualified` verdict
recorded in `## Verdict` above. The instruction was to record this plan as resolved /
not-applicable rather than to halt on the unmet precondition as a blocking checkpoint, and rather
than to fabricate or proceed with a measurement that the settled verdict has already made moot.

**Cross-reference.** See `## Verdict` (the disqualification and its version-scoping) and
`## Promotion Status` ("this is not an interim state pending 18-06 promotion work — it is this
plan's closing disposition") above for the full basis of this determination.
