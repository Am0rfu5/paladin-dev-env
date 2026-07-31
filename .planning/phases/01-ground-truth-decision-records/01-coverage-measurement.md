# Coverage Measurement — Raw Evidence Record

This file is raw evidence only: verbatim commands, verbatim tool output, toolchain versions,
commit SHA, branch, and UTC dates. It carries no gate, floor, or target value — those are recorded
in `.planning/decisions/0006-coverage-gate.md` (a later plan), which transcribes from here.

## Tracer — pipeline proof on one crate (`paladin-ai-core`)

**Purpose:** prove the fully offline `-C instrument-coverage` → `llvm-profdata merge` → `llvm-cov
report` chain works in this environment, with no network access and no new crate installed, before
spending a full workspace instrumented build on it.

### Environment probes (verbatim)

Command: `rustc -vV`

```
rustc 1.97.1 (8bab26f4f 2026-07-14)
binary: rustc
commit-hash: 8bab26f4f68e0e26f0bb7960be334d5b520ea452
commit-date: 2026-07-14
host: x86_64-unknown-linux-gnu
release: 1.97.1
LLVM version: 22.1.6
```

Command: `cargo --version`

```
cargo 1.97.1 (c980f4866 2026-06-30)
```

Command: `cargo llvm-cov --version`

```
error: no such command: `llvm-cov`

help: view all installed commands with `cargo --list`
help: find a package to install `llvm-cov` with `cargo search cargo-llvm-cov`
```

Exit status: 101 (command does not exist). This failure is itself evidence, recorded rather than
hidden — it is exactly what plan 01-04 hit when it correctly halted (crates.io returned HTTP 403
in that sandbox, so `cargo install cargo-llvm-cov` could not complete). This plan does not attempt
that install at all.

Command: `command -v docker`

```
(no output)
```

Exit status: 1 (absent). Docker is not installed in this environment. This is expected and
confirmed per the environment facts supplied to this plan.

**LLVM tool directory resolution.** Resolved portably as
`$(rustc --print sysroot)/lib/rustlib/$(rustc -vV | sed -n 's/^host: //p')/bin` rather than any
hardcoded toolchain version or absolute path. On this run that resolved to:

```
/usr/local/rustup/toolchains/1.97.1-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/bin
```

Both `llvm-profdata` and `llvm-cov` are present in that directory (confirmed via `ls`), alongside
`llc`, `llvm-ar`, `llvm-as`, `llvm-cov`, `llvm-dis`, `llvm-nm`, `llvm-objcopy`, `llvm-objdump`,
`llvm-profdata`, `llvm-readobj`, `llvm-size`, `llvm-strip`, `opt`. The `llvm-tools` rustup
component is installed.

**`jq` availability.** `jq-1.6` present at `/usr/bin/jq`, used below for object-file discovery.

### Package-name correction (Rule 3 auto-fix, not a fabrication)

The plan's read_first / action text names the crate `paladin-core`, matching the directory
`crates/paladin-core/`. The Cargo *package* name is actually `paladin-ai-core` — the workspace
root `Cargo.toml` aliases it: `paladin-core = { package = "paladin-ai-core", version = "0.6.0",
path = "crates/paladin-core" }`. `cargo test -p paladin-core --offline` fails with
`error: package ID specification 'paladin-core' did not match any packages` (suggesting the
correct name). All commands below use the correct package spec, `-p paladin-ai-core`. This is a
mechanical package-ID correction, not a scope or evidence substitution.

### Pipeline commands (verbatim)

`target/coverage/` did not exist in this worktree prior to this run (a plain `ls target` returned
"No such file or directory" — the worktree has no warm build of its own, unlike the shared
checkout the environment facts describe), so there was no stale `.profraw`/`.profdata` to delete.
The repo-root `lcov.info` (dated May 31, predating the workspace migration) was not read, opened,
or used as an input anywhere in this pipeline.

Command 1 — instrumented test run:

```
RUSTFLAGS="-C instrument-coverage" \
LLVM_PROFILE_FILE="$PWD/target/coverage/paladin-%p-%m.profraw" \
cargo test -p paladin-ai-core --offline
```

Result (tail of output):

```
test result: ok. 49 passed; 0 failed; 37 ignored; 0 measured; 0 filtered out; finished in 0.02s

all doctests ran in 0.75s; merged doctests compilation took 0.73s
```

The unittest binary run reported (grep of full log): `test result: ok.` for the
`paladin_core-01c7b4d9b8b89386` unittest target, plus additional passing library/doctest results.
No test failures. This produced 17 `.profraw` files under `target/coverage/`.

Command 2 — profile merge:

```
llvm-profdata merge -sparse target/coverage/*.profraw -o target/coverage/paladin.profdata
```

(invoked as `/usr/local/rustup/toolchains/1.97.1-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/bin/llvm-profdata merge -sparse target/coverage/*.profraw -o target/coverage/paladin.profdata`,
resolved via the portable sysroot path above)

Result: exit 0, no output — `target/coverage/paladin.profdata` created.

Command 3 — object discovery:

```
RUSTFLAGS="-C instrument-coverage" cargo test -p paladin-ai-core --no-run --message-format=json --offline \
  | jq -r 'select(.profile.test == true) | .filenames[]' \
  | grep -v '\.dSYM'
```

Result (one line, the unittest binary — this crate's doctests are compiled in a separate,
transient harness that does not surface a stable `--object` path, consistent with Task 2's
explicit doctest-exclusion decision):

```
/workspace/.claude/worktrees/agent-aaaa928ffbc589613/target/debug/deps/paladin_core-01c7b4d9b8b89386
```

Command 4 — report:

```
llvm-cov report --instr-profile=target/coverage/paladin.profdata \
  --object /workspace/.claude/worktrees/agent-aaaa928ffbc589613/target/debug/deps/paladin_core-01c7b4d9b8b89386
```

TOTAL row, pasted exactly as printed (this tracer applies no `--ignore-filename-regex`, so — as
expected for a proof-of-pipeline pass — the row aggregates every source file linked into the test
binary, including third-party dependency crates under `usr/local/cargo/registry/`, not
`paladin-ai-core` alone; the properly scoped, first-party-only figure is produced in the
measurement-of-record section below using `--ignore-filename-regex`):

```
TOTAL   182325   152570   16.32%   20184   17165   14.96%   129522   107982   16.63%   0   0   -
```

Column header for reference: `Regions  Missed Regions  Cover  Functions  Missed Functions  Executed  Lines  Missed Lines  Cover  Branches  Missed Branches  Cover`.

**Tracer outcome:** a real `llvm-cov report` printed a real TOTAL row with a line-coverage
percentage, produced entirely offline with no network access and no new crate installed. This
proves instrumentation, profile merge, object discovery, and reporting all work in this
environment. This raw, unfiltered total is **not** the workspace figure and is not used as one —
it is pipeline proof only.

No `.profraw`, `.profdata`, `.lcov`, or coverage HTML file was staged into git at this point
(`target/` is not a tracked path in this repository).

## Measurement of record — workspace-wide line coverage

**Toolchain and repository state** (verbatim):

- `rustc -vV`: `rustc 1.97.1 (8bab26f4f 2026-07-14)`, `host: x86_64-unknown-linux-gnu`,
  `release: 1.97.1`, `LLVM version: 22.1.6` (identical to the tracer probe above; re-confirmed
  before this run).
- `cargo --version`: `cargo 1.97.1 (c980f4866 2026-06-30)`
- `git rev-parse HEAD`: `9be788c8e9c744ec3a6aad20b64110fb85925de4`
- `git rev-parse --abbrev-ref HEAD`: `worktree-agent-aaaa928ffbc589613` (a per-agent worktree
  branch created off `release/v0.7.0` for this plan's execution; the commit above is this plan's
  own Task 1 tracer commit, made against the tree this measurement then ran on)
- `date -u`: `2026-07-31T14:57:11Z`

**Scope decision (stated before running, per D-08 and the plan's scope-pinning requirement):**

- `--workspace` is used — mandatory per D-08; a bare `cargo test` covers only the root crate.
- The measured scope is **workspace default-feature test targets only**. The Docker-backed
  `--features integration-tests` scope that `.github/workflows/integration-tests.yml:110-123` runs
  **could not execute here** — `command -v docker` returned nothing (exit 1) in the tracer probe
  above, so Redis and MinIO are unavailable and that feature was never passed. The recorded figure
  is therefore expected to differ from the CI figure, which additionally exercises the
  `integration-tests` feature's test paths.
- `--ignore-filename-regex='(^|/)(examples|benches)/|crates/doc-examples/|registry/src/|rustlib/src/'`
  excludes, in words: any `examples/` or `benches/` directory anywhere in the tree (the workspace
  root's `examples/`/`benches/` and any crate-local equivalents); the `crates/doc-examples/` crate;
  third-party dependency source under the cargo registry (`.../registry/src/...`, matched by
  `registry/src/`); and the Rust standard library's own source (matched by `rustlib/src/`, the
  path segment under which `rustc`'s bundled `library/` sources are vendored in this toolchain).
  Only first-party workspace source (`crates/*/src`, `src/`, and this workspace's own `tests/`)
  remains in the denominator.
- Doctests are excluded — no `--doctests` pass was run, and the object-discovery command below
  intentionally selects only `.profile.test == true` unit/integration-test binaries, not the
  doctest harness. Doctests are not part of this figure.

**Command 1 — instrumented workspace test run:**

```
RUSTFLAGS="-C instrument-coverage" \
LLVM_PROFILE_FILE="$PWD/target/coverage/paladin-%p-%m.profraw" \
cargo test --workspace --offline
```

`target/coverage/` was deleted again immediately before this run (`rm -rf target/coverage`), so no
artifact from the Task 1 tracer carried into this measurement. Result: exit 0. Every one of the 35
`test result: ok.` lines in the run's output reports `0 failed`; no compile errors; no test
failures anywhere in the workspace (`grep -c 'test result: ok' -> 35`, `grep -iE '^error|error\['`
-> no matches). This produced 2,341 `.profraw` files under `target/coverage/`.

**Command 2 — profile merge:**

```
llvm-profdata merge -sparse target/coverage/*.profraw -o target/coverage/paladin.profdata
```

(resolved via the same portable sysroot path as the tracer). Result: exit 0,
`target/coverage/paladin.profdata` created.

**Command 3 — object discovery (workspace scope):**

```
RUSTFLAGS="-C instrument-coverage" cargo test --workspace --no-run --message-format=json --offline \
  | jq -r 'select(.profile.test == true) | .filenames[]' \
  | grep -v '\.dSYM' \
  | sort -u
```

Result: 31 unique test-binary object paths (matching the 31 `Running unittests`/`Running tests`
harness invocations counted in the Command 1 log), one per `[[test]]`/unit-test target across the
workspace's crates and the root package. Full list captured in this run's working notes; every
path is under `target/debug/deps/` inside this worktree.

**Command 4 — report:**

```
llvm-cov report --instr-profile=target/coverage/paladin.profdata \
  --ignore-filename-regex='(^|/)(examples|benches)/|crates/doc-examples/|registry/src/|rustlib/src/' \
  --object=<obj1> --object=<obj2> ... --object=<obj31>
```

(all 31 discovered object paths from Command 3, each passed as a separate `--object` argument, per
the plan's instruction).

Full `llvm-cov report` stdout, pasted verbatim (per-file rows for every first-party source file
that survived the `--ignore-filename-regex` exclusion, `crates/*/src`, `src/`, and this workspace's
own `tests/`):

```
warning: 33 functions have mismatched data
Filename                                                                     Regions    Missed Regions     Cover   Functions  Missed Functions  Executed       Lines      Missed Lines     Cover    Branches   Missed Branches     Cover
----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
crates/paladin-battalion/src/campaign_service.rs                                 272                39    85.66%          18                 7    61.11%         180                36    80.00%           0                 0         -
crates/paladin-battalion/src/chain_of_command_service.rs                         335                31    90.75%          32                 6    81.25%         259                40    84.56%           0                 0         -
crates/paladin-battalion/src/commander.rs                                       2846               339    88.09%         135                21    84.44%        1878               267    85.78%           0                 0         -
crates/paladin-battalion/src/conclave_execution_service.rs                       665                79    88.12%          37                 3    91.89%         446                53    88.12%           0                 0         -
crates/paladin-battalion/src/council_service.rs                                  630               126    80.00%          41                10    75.61%         471               101    78.56%           0                 0         -
crates/paladin-battalion/src/error_aggregation.rs                                247                 1    99.60%          20                 0   100.00%         147                 1    99.32%           0                 0         -
crates/paladin-battalion/src/formation_service.rs                                463                22    95.25%          35                 3    91.43%         306                22    92.81%           0                 0         -
crates/paladin-battalion/src/grove_service.rs                                    933               184    80.28%         102                40    60.78%         715               181    74.69%           0                 0         -
crates/paladin-battalion/src/in_memory_registry.rs                               362                 8    97.79%          21                 1    95.24%         179                 5    97.21%           0                 0         -
crates/paladin-battalion/src/maneuver/mod.rs                                     285                 4    98.60%          31                 1    96.77%         217                 3    98.62%           0                 0         -
crates/paladin-battalion/src/maneuver/parser/ast.rs                              178                 0   100.00%          18                 0   100.00%         112                 0   100.00%           0                 0         -
crates/paladin-battalion/src/maneuver/parser/error.rs                            102                17    83.33%           8                 0   100.00%          83                15    81.93%           0                 0         -
crates/paladin-battalion/src/maneuver/parser/lexer.rs                            281                14    95.02%          15                 0   100.00%         163                13    92.02%           0                 0         -
crates/paladin-battalion/src/maneuver/parser/mod.rs                              170                 9    94.71%          11                 0   100.00%         103                 2    98.06%           0                 0         -
crates/paladin-battalion/src/maneuver/service.rs                                 990                88    91.11%          59                10    83.05%         563               105    81.35%           0                 0         -
crates/paladin-battalion/src/maneuver/visualizer.rs                              743                 5    99.33%          34                 0   100.00%         356                 3    99.16%           0                 0         -
crates/paladin-battalion/src/phalanx_service.rs                                 1048                56    94.66%          67                 4    94.03%         573                46    91.97%           0                 0         -
crates/paladin-battalion/src/retry.rs                                            160                 0   100.00%          13                 0   100.00%         122                 0   100.00%           0                 0         -
crates/paladin-content/src/adapters/document/document_adapter.rs                 504                52    89.68%          53                 8    84.91%         267                36    86.52%           0                 0         -
crates/paladin-content/src/adapters/document/pdf_extractor.rs                    350                23    93.43%          27                 0   100.00%         195                 9    95.38%           0                 0         -
crates/paladin-content/src/adapters/input/file_content_fetcher.rs                453                71    84.33%          26                 7    73.08%         186                18    90.32%           0                 0         -
crates/paladin-content/src/adapters/input/file_content_list_fetcher.rs           301                57    81.06%          16                 7    56.25%         133                27    79.70%           0                 0         -
crates/paladin-content/src/adapters/input/http_content_fetcher.rs                194                12    93.81%          17                 4    76.47%         108                 5    95.37%           0                 0         -
crates/paladin-content/src/adapters/input/news_api_fetcher.rs                    587               116    80.24%          29                 7    75.86%         349                60    82.81%           0                 0         -
crates/paladin-content/src/services/content_aggregator_service.rs                215                 6    97.21%          14                 2    85.71%         100                 6    94.00%           0                 0         -
crates/paladin-content/src/services/content_analysis_service.rs                   42                 0   100.00%           4                 0   100.00%          21                 0   100.00%           0                 0         -
crates/paladin-content/src/services/content_delivery_service.rs                   56                20    64.29%          10                 6    40.00%          81                42    48.15%           0                 0         -
crates/paladin-content/src/services/content_fetching_service.rs                  269                25    90.71%          18                 9    50.00%         123                11    91.06%           0                 0         -
crates/paladin-content/src/services/content_list_fetching_service.rs             103                 2    98.06%           8                 1    87.50%          50                 1    98.00%           0                 0         -
crates/paladin-content/src/services/content_llm_analysis_service.rs              447                60    86.58%          30                10    66.67%         326                50    84.66%           0                 0         -
crates/paladin-content/src/services/content_summarizer_service.rs                513                90    82.46%          31                 4    87.10%         302                35    88.41%           0                 0         -
crates/paladin-core/src/base/component/action.rs                                 377                20    94.69%          31                 4    87.10%         284                15    94.72%           0                 0         -
crates/paladin-core/src/base/component/event.rs                                  178                 0   100.00%          14                 0   100.00%         138                 0   100.00%           0                 0         -
crates/paladin-core/src/base/entity/collection.rs                                 38                32    15.79%          10                 8    20.00%          38                32    15.79%           0                 0         -
crates/paladin-core/src/base/entity/field.rs                                       4                 0   100.00%           1                 0   100.00%           7                 0   100.00%           0                 0         -
crates/paladin-core/src/base/entity/message.rs                                   241                62    74.27%          29                 9    68.97%         202                62    69.31%           0                 0         -
crates/paladin-core/src/base/entity/node.rs                                      154                15    90.26%          13                 2    84.62%         114                15    86.84%           0                 0         -
crates/paladin-core/src/base/service/collection_versioning_service.rs            878                74    91.57%          53                12    77.36%         739                31    95.81%           0                 0         -
crates/paladin-core/src/base/service/field_version_service.rs                    426               204    52.11%          41                26    36.59%         351               166    52.71%           0                 0         -
crates/paladin-core/src/base/service/message_service.rs                          357               116    67.51%          43                13    69.77%         261                80    69.35%           0                 0         -
crates/paladin-core/src/base/service/node_version_service.rs                     423               107    74.70%          37                18    51.35%         338                59    82.54%           0                 0         -
crates/paladin-core/src/platform/container/arsenal/core.rs                        15                 0   100.00%           3                 0   100.00%          25                 0   100.00%           0                 0         -
crates/paladin-core/src/platform/container/arsenal/handoff_error.rs              152                 0   100.00%          12                 0   100.00%          92                 0   100.00%           0                 0         -
crates/paladin-core/src/platform/container/arsenal/handoff_tool.rs                90                 0   100.00%          12                 0   100.00%          74                 0   100.00%           0                 0         -
crates/paladin-core/src/platform/container/autonomous_config.rs                  310                13    95.81%          41                 2    95.12%         271                15    94.46%           0                 0         -
crates/paladin-core/src/platform/container/battalion/campaign.rs                 244                 3    98.77%          24                 1    95.83%         163                 3    98.16%           0                 0         -
crates/paladin-core/src/platform/container/battalion/chain_of_command.rs         213                14    93.43%          19                 3    84.21%         124                12    90.32%           0                 0         -
crates/paladin-core/src/platform/container/battalion/conclave.rs                 331                19    94.26%          27                 1    96.30%         233                18    92.27%           0                 0         -
crates/paladin-core/src/platform/container/battalion/council.rs                  285                16    94.39%          30                 2    93.33%         222                14    93.69%           0                 0         -
crates/paladin-core/src/platform/container/battalion/formation.rs                153                 0   100.00%          14                 0   100.00%          89                 0   100.00%           0                 0         -
crates/paladin-core/src/platform/container/battalion/grove.rs                    512                24    95.31%          35                 2    94.29%         345                21    93.91%           0                 0         -
crates/paladin-core/src/platform/container/battalion/mod.rs                      760                44    94.21%          57                 8    85.96%         517                41    92.07%           0                 0         -
crates/paladin-core/src/platform/container/battalion/phalanx.rs                  198                 4    97.98%          16                 0   100.00%         120                 4    96.67%           0                 0         -
crates/paladin-core/src/platform/container/citadel.rs                            449                 3    99.33%          25                 1    96.00%         282                 3    98.94%           0                 0         -
crates/paladin-core/src/platform/container/citadel_error.rs                      103                 2    98.06%          13                 0   100.00%          65                 0   100.00%           0                 0         -
crates/paladin-core/src/platform/container/content.rs                            401                28    93.02%          42                 1    97.62%         255                 6    97.65%           0                 0         -
crates/paladin-core/src/platform/container/content_list.rs                       437               160    63.39%          52                29    44.23%         261               124    52.49%           0                 0         -
crates/paladin-core/src/platform/container/document.rs                           171                 0   100.00%          17                 0   100.00%          99                 0   100.00%           0                 0         -
crates/paladin-core/src/platform/container/execution_result.rs                    32                 0   100.00%           7                 0   100.00%          43                 0   100.00%           0                 0         -
crates/paladin-core/src/platform/container/garrison.rs                           399                31    92.23%          32                 5    84.38%         263                35    86.69%           0                 0         -
crates/paladin-core/src/platform/container/garrison_error.rs                      14                 0   100.00%           2                 0   100.00%           8                 0   100.00%           0                 0         -
crates/paladin-core/src/platform/container/handoff.rs                            318                 1    99.69%          28                 0   100.00%         177                 1    99.44%           0                 0         -
crates/paladin-core/src/platform/container/herald.rs                             310                72    76.77%          46                17    63.04%         246                48    80.49%           0                 0         -
crates/paladin-core/src/platform/container/herald_error.rs                        37                 0   100.00%           6                 0   100.00%          23                 0   100.00%           0                 0         -
crates/paladin-core/src/platform/container/job.rs                                804               180    77.61%          58                14    75.86%         547               118    78.43%           0                 0         -
crates/paladin-core/src/platform/container/log.rs                                397               119    70.03%          38                13    65.79%         267                81    69.66%           0                 0         -
crates/paladin-core/src/platform/container/notification.rs                       559               283    49.37%          42                21    50.00%         486               250    48.56%           0                 0         -
crates/paladin-core/src/platform/container/orchestration_context.rs               22                 1    95.45%           3                 0   100.00%          23                 0   100.00%           0                 0         -
crates/paladin-core/src/platform/container/paladin.rs                            133                18    86.47%          17                 1    94.12%         104                10    90.38%           0                 0         -
crates/paladin-core/src/platform/container/paladin_config.rs                     142                 0   100.00%          16                 0   100.00%         117                 0   100.00%           0                 0         -
crates/paladin-core/src/platform/container/paladin_error.rs                       58                 1    98.28%           7                 0   100.00%          33                 0   100.00%           0                 0         -
crates/paladin-core/src/platform/container/planning.rs                           316                 3    99.05%          19                 0   100.00%         154                 9    94.16%           0                 0         -
crates/paladin-core/src/platform/container/prompt.rs                             268                93    65.30%          20                 3    85.00%         201                53    73.63%           0                 0         -
crates/paladin-core/src/platform/container/queue_config.rs                        40                 0   100.00%           4                 0   100.00%          52                 0   100.00%           0                 0         -
crates/paladin-core/src/platform/container/queue_item.rs                         331                69    79.15%          25                 7    72.00%         228                50    78.07%           0                 0         -
crates/paladin-core/src/platform/container/sanctum.rs                            107                14    86.92%          15                 2    86.67%          94                14    85.11%           0                 0         -
crates/paladin-core/src/platform/container/schedule.rs                            24                 0   100.00%           2                 0   100.00%          22                 0   100.00%           0                 0         -
crates/paladin-core/src/platform/container/task.rs                               685                70    89.78%          83                17    79.52%         442                51    88.46%           0                 0         -
crates/paladin-core/src/platform/container/trigger.rs                            477               133    72.12%          27                10    62.96%         359               106    70.47%           0                 0         -
crates/paladin-core/src/platform/container/user.rs                               539                31    94.25%          38                 3    92.11%         308                15    95.13%           0                 0         -
crates/paladin-core/src/platform/container/vision.rs                             320                19    94.06%          15                 1    93.33%         254                18    92.91%           0                 0         -
crates/paladin-herald/src/json_herald.rs                                         668                13    98.05%          41                 5    87.80%         399                11    97.24%           0                 0         -
crates/paladin-herald/src/markdown_herald.rs                                     702                38    94.59%          46                 5    89.13%         429                17    96.04%           0                 0         -
crates/paladin-herald/src/table_herald.rs                                        642                22    96.57%          36                 3    91.67%         382                15    96.07%           0                 0         -
crates/paladin-llm/src/anthropic/adapter.rs                                      666               241    63.81%          58                24    58.62%         524               228    56.49%           0                 0         -
crates/paladin-llm/src/anthropic/vision.rs                                       458               242    47.16%          45                25    44.44%         362               178    50.83%           0                 0         -
crates/paladin-llm/src/config/bridge.rs                                           51                37    27.45%           9                 7    22.22%          54                43    20.37%           0                 0         -
crates/paladin-llm/src/config/llm.rs                                             216                10    95.37%          12                 0   100.00%         191                 6    96.86%           0                 0         -
crates/paladin-llm/src/config/vision.rs                                           99                 3    96.97%          12                 0   100.00%          83                 0   100.00%           0                 0         -
crates/paladin-llm/src/deepseek/adapter.rs                                       497               286    42.45%          46                28    39.13%         389               256    34.19%           0                 0         -
crates/paladin-llm/src/error.rs                                                   19                19     0.00%           1                 1     0.00%          13                13     0.00%           0                 0         -
crates/paladin-llm/src/llm_analysis_service.rs                                    92                27    70.65%           8                 2    75.00%          99                36    63.64%           0                 0         -
crates/paladin-llm/src/mock.rs                                                   276                27    90.22%          44                10    77.27%         216                42    80.56%           0                 0         -
crates/paladin-llm/src/openai/adapter.rs                                         493               367    25.56%          45                33    26.67%         392               297    24.23%           0                 0         -
crates/paladin-llm/src/openai/vision.rs                                          322               245    23.91%          38                28    26.32%         254               200    21.26%           0                 0         -
crates/paladin-llm/src/provider_factory.rs                                       125                70    44.00%          14                 8    42.86%         104                67    35.58%           0                 0         -
crates/paladin-memory/src/citadel/file_citadel.rs                                518                57    89.00%          44                 8    81.82%         294                66    77.55%           0                 0         -
crates/paladin-memory/src/config/garrison.rs                                     113                 4    96.46%          11                 0   100.00%         110                 0   100.00%           0                 0         -
crates/paladin-memory/src/config/rag.rs                                           90                 2    97.78%          12                 0   100.00%          99                 5    94.95%           0                 0         -
crates/paladin-memory/src/config/sanctum.rs                                      120                12    90.00%          12                 2    83.33%         122                17    86.07%           0                 0         -
crates/paladin-memory/src/garrison/in_memory_garrison.rs                         323                10    96.90%          34                 6    82.35%         215                 9    95.81%           0                 0         -
crates/paladin-memory/src/garrison/sqlite_garrison.rs                            441                93    78.91%          67                34    49.25%         372                92    75.27%           0                 0         -
crates/paladin-memory/src/sanctum/in_memory_adapter.rs                           250                29    88.40%          27                 3    88.89%         157                20    87.26%           0                 0         -
crates/paladin-memory/src/services/memory_extraction_service.rs                  559                61    89.09%          52                16    69.23%         434                46    89.40%           0                 0         -
crates/paladin-memory/src/services/rag_retrieval_service.rs                      394                52    86.80%          39                12    69.23%         267                46    82.77%           0                 0         -
crates/paladin-ports/src/input/document_port.rs                                  135                 3    97.78%          13                 0   100.00%         117                 3    97.44%           0                 0         -
crates/paladin-ports/src/input/ml_port.rs                                         91                52    42.86%           7                 4    42.86%          66                44    33.33%           0                 0         -
crates/paladin-ports/src/output/arsenal_port.rs                                    2                 2     0.00%           1                 1     0.00%           2                 2     0.00%           0                 0         -
crates/paladin-ports/src/output/auth_port.rs                                      30                 0   100.00%           2                 0   100.00%          20                 0   100.00%           0                 0         -
crates/paladin-ports/src/output/battalion_port.rs                                 97                 2    97.94%          15                 0   100.00%          69                 2    97.10%           0                 0         -
crates/paladin-ports/src/output/citadel_port.rs                                   75                 0   100.00%           8                 0   100.00%          41                 0   100.00%           0                 0         -
crates/paladin-ports/src/output/content_delivery_port.rs                          38                 0   100.00%           2                 0   100.00%          35                 0   100.00%           0                 0         -
crates/paladin-ports/src/output/file_storage_port.rs                             117               117     0.00%          19                19     0.00%         104               104     0.00%           0                 0         -
crates/paladin-ports/src/output/garrison_port.rs                                  35                 0   100.00%           3                 0   100.00%          29                 0   100.00%           0                 0         -
crates/paladin-ports/src/output/llm_port.rs                                       54                 0   100.00%           5                 0   100.00%          72                 0   100.00%           0                 0         -
crates/paladin-ports/src/output/log_port.rs                                      131                21    83.97%          16                 6    62.50%          98                18    81.63%           0                 0         -
crates/paladin-ports/src/output/notification_port.rs                              43                14    67.44%           7                 4    42.86%          34                10    70.59%           0                 0         -
crates/paladin-ports/src/output/orchestrator_port.rs                             153                 0   100.00%          12                 0   100.00%          89                 0   100.00%           0                 0         -
crates/paladin-ports/src/output/paladin_port.rs                                  289                 0   100.00%          13                 0   100.00%         199                 0   100.00%           0                 0         -
crates/paladin-ports/src/output/paladin_registry.rs                               21                 2    90.48%           3                 1    66.67%          11                 1    90.91%           0                 0         -
crates/paladin-ports/src/output/queue_port.rs                                     62                 0   100.00%           4                 0   100.00%          54                 0   100.00%           0                 0         -
crates/paladin-ports/src/output/sanctum_port.rs                                   89                 0   100.00%          14                 0   100.00%          66                 0   100.00%           0                 0         -
crates/paladin-ports/src/output/scheduler_port.rs                                124                 3    97.58%          15                 1    93.33%          82                 3    96.34%           0                 0         -
crates/paladin-ports/src/output/vision_llm_port.rs                                57                18    68.42%          17                 8    52.94%          46                20    56.52%           0                 0         -
crates/paladin-ports/src/output/workflow_repository_port.rs                       30                 6    80.00%           4                 0   100.00%          30                 5    83.33%           0                 0         -
crates/paladin-storage/src/redis.rs                                              361               361     0.00%         108               108     0.00%         350               350     0.00%           0                 0         -
crates/paladin-storage/src/sqlite_content_repository.rs                          544               454    16.54%         107                96    10.28%         392               309    21.17%           0                 0         -
crates/paladin-storage/src/sqlite_user_repository.rs                             824                68    91.75%          85                32    62.35%         448                55    87.72%           0                 0         -
crates/paladin-storage/src/sqlite_workflow_repository.rs                         258                58    77.52%          32                17    46.88%         178                51    71.35%           0                 0         -
crates/paladin-web/src/adapters/api_content_deliverer.rs                         618               231    62.62%          54                22    59.26%         483               190    60.66%           0                 0         -
crates/paladin-web/src/agent_auth.rs                                             339                10    97.05%          38                 3    92.11%         264                 7    97.35%           0                 0         -
crates/paladin-web/src/agent_controller.rs                                      1509                52    96.55%         132                 4    96.97%        1044                28    97.32%           0                 0         -
crates/paladin-web/src/agent_registry.rs                                         365                16    95.62%          34                 8    76.47%         218                10    95.41%           0                 0         -
crates/paladin-web/src/app.rs                                                     60                 9    85.00%           2                 1    50.00%          34                 8    76.47%           0                 0         -
crates/paladin-web/src/auth_middleware.rs                                        228                 5    97.81%          32                 2    93.75%         133                 4    96.99%           0                 0         -
crates/paladin-web/src/delivery_controller.rs                                    172                 7    95.93%          23                 0   100.00%         131                 3    97.71%           0                 0         -
crates/paladin-web/src/error.rs                                                  144                 0   100.00%          21                 0   100.00%         132                 0   100.00%           0                 0         -
crates/paladin-web/src/health.rs                                                  51                 0   100.00%           9                 0   100.00%          33                 0   100.00%           0                 0         -
crates/paladin-web/src/http_layers.rs                                            326                28    91.41%          38                 5    86.84%         238                19    92.02%           0                 0         -
crates/paladin-web/src/job_store.rs                                              225                12    94.67%          19                 5    73.68%         106                 8    92.45%           0                 0         -
crates/paladin-web/src/openapi.rs                                                182                14    92.31%          12                 0   100.00%         125                 5    96.00%           0                 0         -
crates/paladin-web/src/request_log.rs                                            115                 1    99.13%          16                 0   100.00%          72                 0   100.00%           0                 0         -
crates/paladin-web/src/timeout.rs                                                 74                 0   100.00%           8                 0   100.00%          54                 0   100.00%           0                 0         -
crates/paladin-web/src/user_controller.rs                                        748               168    77.54%          75                22    70.67%         518               142    72.59%           0                 0         -
crates/paladin-web/tests/auth_rbac.rs                                            195                26    86.67%          30                13    56.67%         122                26    78.69%           0                 0         -
src/application/errors/planning_error.rs                                          94                 0   100.00%          11                 0   100.00%          63                 0   100.00%           0                 0         -
src/application/errors/prompt_error.rs                                           112                 0   100.00%          10                 0   100.00%          66                 0   100.00%           0                 0         -
src/application/services/arsenal/arsenal_execution_service.rs                    310                24    92.26%          32                 4    87.50%         215                21    90.23%           0                 0         -
src/application/services/arsenal/arsenal_registry_service.rs                     101                 0   100.00%          17                 0   100.00%          59                 0   100.00%           0                 0         -
src/application/services/content/content_ingestion_service.rs                    812               176    78.33%          82                33    59.76%         550               115    79.09%           0                 0         -
src/application/services/herald/herald_registry.rs                               405                18    95.56%          32                 6    81.25%         190                21    88.95%           0                 0         -
src/application/services/log_orchestrator/mod.rs                                1142               104    90.89%         105                31    70.48%         797                71    91.09%           0                 0         -
src/application/services/log_orchestrator/types.rs                                43                13    69.77%           5                 1    80.00%          46                 8    82.61%           0                 0         -
src/application/services/notification_orchestrator/mod.rs                        693               313    54.83%          60                23    61.67%         513               255    50.29%           0                 0         -
src/application/services/notification_orchestrator/types.rs                       42                 0   100.00%           1                 0   100.00%          22                 0   100.00%           0                 0         -
src/application/services/orchestration/listener.rs                               452               196    56.64%          55                28    49.09%         313               137    56.23%           0                 0         -
src/application/services/orchestration/mod.rs                                   2189               168    92.33%         164                20    87.80%        1269               114    91.02%           0                 0         -
src/application/services/orchestration/orchestrator_bridge.rs                    361                35    90.30%          44                10    77.27%         295                34    88.47%           0                 0         -
src/application/services/orchestration/processors/battalion_processor.rs         225                35    84.44%          22                 9    59.09%         131                32    75.57%           0                 0         -
src/application/services/orchestration/processors/mod.rs                          40                11    72.50%           6                 2    66.67%          34                 9    73.53%           0                 0         -
src/application/services/orchestration/processors/paladin_processor.rs           234                13    94.44%          21                 3    85.71%         127                 9    92.91%           0                 0         -
src/application/services/orchestration/scheduler.rs                             1211               146    87.94%          84                12    85.71%         710                83    88.31%           0                 0         -
src/application/services/orchestration/types.rs                                   61                 5    91.80%          14                 0   100.00%          67                 4    94.03%           0                 0         -
src/application/services/paladin/handoff_service.rs                              298                17    94.30%          27                 2    92.59%         239                12    94.98%           0                 0         -
src/application/services/paladin/paladin_builder.rs                             1136               166    85.39%         123                38    69.11%         938               141    84.97%           0                 0         -
src/application/services/paladin/paladin_execution_service.rs                   1246               457    63.32%          77                27    64.94%         883               346    60.82%           0                 0         -
src/application/services/paladin/planning_service.rs                             732                70    90.44%          52                11    78.85%         577                43    92.55%           0                 0         -
src/application/services/paladin/prompt_generation_service.rs                    342                20    94.15%          31                 7    77.42%         234                26    88.89%           0                 0         -
src/application/services/paladin/temperature_service.rs                          533                20    96.25%          55                 6    89.09%         394                23    94.16%           0                 0         -
src/application/services/queue_orchestrator/mod.rs                               586                41    93.00%          61                 8    86.89%         371                20    94.61%           0                 0         -
src/application/services/queue_orchestrator/types.rs                             226                56    75.22%          13                 3    76.92%         177                33    81.36%           0                 0         -
src/bin/paladin-server.rs                                                        185               185     0.00%          13                13     0.00%         145               145     0.00%           0                 0         -
src/config/agents.rs                                                             185                 0   100.00%          19                 0   100.00%         123                 0   100.00%           0                 0         -
src/config/arsenal.rs                                                              3                 0   100.00%           1                 0   100.00%           7                 0   100.00%           0                 0         -
src/config/citadel.rs                                                            143                 6    95.80%          10                 0   100.00%         151                 1    99.34%           0                 0         -
src/config/env_utils.rs                                                          119                 4    96.64%          15                 0   100.00%          87                 1    98.85%           0                 0         -
src/config/file_storage.rs                                                       186                13    93.01%           5                 0   100.00%         150                 1    99.33%           0                 0         -
src/config/herald.rs                                                             215                 0   100.00%          15                 0   100.00%         231                 0   100.00%           0                 0         -
src/config/queue.rs                                                               59                44    25.42%           3                 1    66.67%          54                36    33.33%           0                 0         -
src/config/scheduler.rs                                                           14                11    21.43%           2                 1    50.00%          16                 9    43.75%           0                 0         -
src/config/settings.rs                                                           267                54    79.78%          11                 2    81.82%         203                25    87.68%           0                 0         -
src/config/setup/mod.rs                                                           39                 5    87.18%           6                 0   100.00%          20                 1    95.00%           0                 0         -
src/config/setup/service_runner.rs                                               381               146    61.68%          42                14    66.67%         284                99    65.14%           0                 0         -
src/config/user_config.rs                                                        183                70    61.75%          35                25    28.57%         185                65    64.86%           0                 0         -
src/core/platform/manager/content_service.rs                                     340                22    93.53%          20                 2    90.00%         255                20    92.16%           0                 0         -
src/core/platform/manager/event_manager.rs                                       335                40    88.06%          33                10    69.70%         206                27    86.89%           0                 0         -
src/core/platform/manager/user_service.rs                                        313                41    86.90%          41                12    70.73%         246                46    81.30%           0                 0         -
src/infrastructure/adapters/arsenal/mcp_protocol.rs                              375                16    95.73%          43                 2    95.35%         253                11    95.65%           0                 0         -
src/infrastructure/adapters/arsenal/mcp_stdio_adapter.rs                          34                 6    82.35%           7                 2    71.43%          20                 4    80.00%           0                 0         -
src/infrastructure/adapters/arsenal/mcp_streamable_http_adapter.rs               107                14    86.92%          14                 2    85.71%          79                12    84.81%           0                 0         -
src/infrastructure/adapters/arsenal/resource_controls.rs                         196                 6    96.94%          28                 1    96.43%         135                 5    96.30%           0                 0         -
src/infrastructure/adapters/arsenal/tool_result_formatter.rs                     382                34    91.10%          19                 3    84.21%         205                18    91.22%           0                 0         -
src/infrastructure/adapters/auth/in_memory_token_auth_adapter.rs                 146                11    92.47%          23                 5    78.26%          81                 7    91.36%           0                 0         -
src/infrastructure/adapters/llm/config_bridge.rs                                  40                 0   100.00%           4                 0   100.00%          36                 0   100.00%           0                 0         -
src/infrastructure/adapters/logs/system_log_adapter.rs                           482                94    80.50%          55                18    67.27%         344                66    80.81%           0                 0         -
src/infrastructure/adapters/scheduling/tokio_cron_adapter.rs                     411                21    94.89%          49                 4    91.84%         211                11    94.79%           0                 0         -
src/infrastructure/resilience/circuit_breaker.rs                                 155                14    90.97%          12                 0   100.00%         123                 8    93.50%           0                 0         -
src/infrastructure/security/audit.rs                                             275                 8    97.09%          22                 1    95.45%         215                 5    97.67%           0                 0         -
src/infrastructure/security/encryption.rs                                        354                11    96.89%          31                 2    93.55%         186                 6    96.77%           0                 0         -
src/infrastructure/security/tls_verification.rs                                   39                 0   100.00%           5                 0   100.00%          29                 0   100.00%           0                 0         -
src/infrastructure/web/agent_host.rs                                             410                48    88.29%          35                 2    94.29%         261                24    90.80%           0                 0         -
src/infrastructure/web/facade_provisioner.rs                                      79                 8    89.87%           9                 1    88.89%          66                 4    93.94%           0                 0         -
src/main.rs                                                                       38                20    47.37%           5                 2    60.00%          29                17    41.38%           0                 0         -
target/debug/build/utoipa-swagger-ui-10aed8599aeed486/out/embed.rs                 1                 1     0.00%           1                 1     0.00%           1                 1     0.00%           0                 0         -
tests/agent_orchestrator_bridge.rs                                                90                 2    97.78%           5                 0   100.00%          56                 1    98.21%           0                 0         -
tests/cli_isolation_test.rs                                                       96                 0   100.00%           9                 0   100.00%          72                 0   100.00%           0                 0         -
tests/content_agent_bridge.rs                                                    153                 2    98.69%          10                 0   100.00%         102                 1    99.02%           0                 0         -
tests/event_trigger_pipeline.rs                                                  230                11    95.22%          24                 4    83.33%         200                11    94.50%           0                 0         -
tests/functional/content_lifecycle_test.rs                                       344                 7    97.97%           6                 0   100.00%         200                 2    99.00%           0                 0         -
tests/functional/paladin_tool_invocation_test.rs                                 542                45    91.70%          31                12    61.29%         530                36    93.21%           0                 0         -
tests/helpers/mock_arsenal_adapter.rs                                            335                23    93.13%          30                 2    93.33%         196                15    92.35%           0                 0         -
tests/helpers/mock_llm_adapter.rs                                                478                55    88.49%          46                11    76.09%         324                52    83.95%           0                 0         -
tests/helpers/mock_paladin_port.rs                                                17                17     0.00%           4                 4     0.00%          15                15     0.00%           0                 0         -
tests/integration/arsenal_bridge_regression_test.rs                              122                23    81.15%          16                 7    56.25%         108                23    78.70%           0                 0         -
tests/integration/arsenal_execution_integration_test.rs                          498                 5    99.00%          34                 0   100.00%         260                 4    98.46%           0                 0         -
tests/integration/arsenal_registry_integration_test.rs                           520                 0   100.00%          40                 0   100.00%         312                 0   100.00%           0                 0         -
tests/integration/autonomous_planning_test.rs                                    166                13    92.17%          13                 5    61.54%         173                21    87.86%           0                 0         -
tests/integration/battalion/campaign_integration_test.rs                         778                 5    99.36%          33                 2    93.94%         373                 5    98.66%           0                 0         -
tests/integration/battalion/chain_of_command_integration_test.rs                 380                 5    98.68%          22                 2    90.91%         180                 5    97.22%           0                 0         -
tests/integration/battalion/council_integration_test.rs                          414                 5    98.79%          20                 2    90.00%         264                 5    98.11%           0                 0         -
tests/integration/battalion/formation_integration_test.rs                        493                 8    98.38%          27                 2    92.59%         242                 7    97.11%           0                 0         -
tests/integration/battalion/grove_integration_test.rs                            806                27    96.65%          34                10    70.59%         398                27    93.22%           0                 0         -
tests/integration/battalion/load_test.rs                                         424                11    97.41%          30                 2    93.33%         278                 8    97.12%           0                 0         -
tests/integration/battalion/phalanx_integration_test.rs                          681                14    97.94%          44                 2    95.45%         329                10    96.96%           0                 0         -
tests/integration/battalion_campaign_integration_test.rs                         624                 7    98.88%          30                 2    93.33%         274                 9    96.72%           0                 0         -
tests/integration/battalion_chain_of_command_integration_test.rs                 586                 7    98.81%          33                 2    93.94%         307                 9    97.07%           0                 0         -
tests/integration/citadel_integration_test.rs                                    520                15    97.12%          32                 6    81.25%         333                14    95.80%           0                 0         -
tests/integration/commander_integration_tests.rs                                1211                33    97.27%          50                 4    92.00%         609                24    96.06%           0                 0         -
tests/integration/context_injection_test.rs                                      352                40    88.64%          26                11    57.69%         309                35    88.67%           0                 0         -
tests/integration/herald_integration_test.rs                                     613                21    96.57%          28                 7    75.00%         300                17    94.33%           0                 0         -
tests/integration/in_memory_sanctum_tests.rs                                     808                 2    99.75%          42                 0   100.00%         470                 2    99.57%           0                 0         -
tests/integration/mcp_stdio_test.rs                                              434                 7    98.39%          28                 1    96.43%         253                 3    98.81%           0                 0         -
tests/integration/mcp_streamable_http_live_test.rs                                23                23     0.00%           3                 3     0.00%          23                23     0.00%           0                 0         -
tests/integration/mcp_streamable_http_test.rs                                    235                10    95.74%          24                 0   100.00%         170                 9    94.71%           0                 0         -
tests/integration/mod.rs                                                         151               151     0.00%          23                23     0.00%         115               115     0.00%           0                 0         -
tests/integration/openai_content_analysis_integration_test.rs                    161               100    37.89%          11                 4    63.64%         172               128    25.58%           0                 0         -
tests/integration/orchestrator_workflow_lifecycle_test.rs                        156                 0   100.00%           8                 0   100.00%          89                 0   100.00%           0                 0         -
tests/integration/paladin_garrison_integration_test.rs                           691                34    95.08%          45                14    68.89%         394                33    91.62%           0                 0         -
tests/integration/paladin_integration_test.rs                                    596                10    98.32%          25                 0   100.00%         335                 4    98.81%           0                 0         -
tests/integration/scheduler_integration_test.rs                                  274                19    93.07%          23                 8    65.22%         151                14    90.73%           0                 0         -
tests/integration/sqlite_garrison_integration_test.rs                            542                 1    99.82%          19                 0   100.00%         344                 0   100.00%           0                 0         -
tests/integration/system_log_integration_test.rs                                 382                 4    98.95%          17                 0   100.00%         266                 0   100.00%           0                 0         -
tests/lib.rs                                                                     177               177     0.00%          28                28     0.00%         163               163     0.00%           0                 0         -
tests/paladin_server_smoke.rs                                                    509                 9    98.23%          16                 1    93.75%         266                 6    97.74%           0                 0         -
tests/queue_port_contract.rs                                                     213                17    92.02%          28                 8    71.43%         130                 9    93.08%           0                 0         -
tests/unit/arsenal/handoff_tool_test.rs                                          289                14    95.16%          19                 6    68.42%         148                14    90.54%           0                 0         -
tests/unit/arsenal_config_test.rs                                                191                 0   100.00%          10                 0   100.00%         169                 0   100.00%           0                 0         -
tests/unit/arsenal_domain_test.rs                                                128                 0   100.00%           5                 0   100.00%          71                 0   100.00%           0                 0         -
tests/unit/arsenal_port_test.rs                                                  278                 2    99.28%          16                 0   100.00%         147                 1    99.32%           0                 0         -
tests/unit/battalion/campaign_service_tests.rs                                   947                 5    99.47%          39                 2    94.87%         388                 5    98.71%           0                 0         -
tests/unit/battalion/campaign_tests.rs                                           630                11    98.25%          22                 0   100.00%         257                 8    96.89%           0                 0         -
tests/unit/battalion/chain_of_command_service_tests.rs                           799                 5    99.37%          39                 2    94.87%         339                 5    98.53%           0                 0         -
tests/unit/battalion/chain_of_command_tests.rs                                   388                11    97.16%          20                 0   100.00%         176                 3    98.30%           0                 0         -
tests/unit/battalion/formation_tests.rs                                          231                 2    99.13%          11                 0   100.00%         107                 2    98.13%           0                 0         -
tests/unit/battalion/phalanx_tests.rs                                            244                 3    98.77%          13                 0   100.00%         114                 0   100.00%           0                 0         -
tests/unit/circuit_breaker_test.rs                                               187                11    94.12%          12                 0   100.00%         100                 2    98.00%           0                 0         -
tests/unit/embedding_port_tests.rs                                                90                 0   100.00%          12                 0   100.00%          73                 0   100.00%           0                 0         -
tests/unit/handoff_service_test.rs                                               835                 5    99.40%          48                 0   100.00%         486                 0   100.00%           0                 0         -
tests/unit/herald_consolidation_test.rs                                          343               300    12.54%          14                11    21.43%         253               208    17.79%           0                 0         -
tests/unit/maneuver_domain_tests.rs                                              529                 3    99.43%          23                 0   100.00%         224                 3    98.66%           0                 0         -
tests/unit/mcp_protocol_test.rs                                                  108                 6    94.44%          12                 0   100.00%          75                 3    96.00%           0                 0         -
tests/unit/mock_llm_adapter_test.rs                                              381                 3    99.21%          32                 0   100.00%         231                 0   100.00%           0                 0         -
tests/unit/paladin_builder_arsenal_test.rs                                       163                20    87.73%          19                 9    52.63%         116                20    82.76%           0                 0         -
tests/unit/paladin_builder_test.rs                                               310                17    94.52%          22                 6    72.73%         228                17    92.54%           0                 0         -
tests/unit/paladin_config_test.rs                                                 88                 0   100.00%           6                 0   100.00%          70                 0   100.00%           0                 0         -
tests/unit/paladin_entity_test.rs                                                145                 0   100.00%           5                 0   100.00%          94                 0   100.00%           0                 0         -
tests/unit/paladin_error_test.rs                                                 123                 0   100.00%           6                 0   100.00%          61                 0   100.00%           0                 0         -
tests/unit/paladin_execution_service_test.rs                                     820                16    98.05%          49                 5    89.80%         451                14    96.90%           0                 0         -
tests/unit/parser_tests.rs                                                       634                37    94.16%          57                 0   100.00%         323                23    92.88%           0                 0         -
tests/unit/prompt_generation_service_test.rs                                     154                 1    99.35%          11                 0   100.00%          83                 1    98.80%           0                 0         -
tests/unit/sanctum_domain_tests.rs                                               410                 0   100.00%          21                 0   100.00%         193                 0   100.00%           0                 0         -
tests/unit/sanctum_port_tests.rs                                                 278                 0   100.00%          22                 0   100.00%         134                 0   100.00%           0                 0         -
tests/unit/scheduler_tests.rs                                                    379                 8    97.89%          45                 1    97.78%         193                 1    99.48%           0                 0         -
tests/unit/settings_config_test.rs                                               316                 2    99.37%          17                 0   100.00%         301                 1    99.67%           0                 0         -
tests/web_server_e2e.rs                                                          353                 3    99.15%          16                 0   100.00%         223                 2    99.10%           0                 0         -
----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
TOTAL                                                                          93834             11888    87.33%        7546              1710    77.34%       61404              9340    84.79%           0                 0         -
```

**Extracted figure.** Column layout per the header row: `Filename | Regions | Missed Regions |
Cover(region) | Functions | Missed Functions | Executed(function-cover) | Lines | Missed Lines |
Cover(line) | Branches | Missed Branches | Cover(branch)`. The TOTAL row's Lines block reads
`61404  9340  84.79%`.

**The single workspace line-coverage percentage is `84.79%`** — 61,404 first-party lines counted,
9,340 missed, transcribed character-for-character from the pasted `llvm-cov report` TOTAL row
above, not re-typed or rounded.

**Caveats:**

- `llvm-cov` printed `warning: 33 functions have mismatched data` before the table. This is a known
  `llvm-cov`/`llvm-profdata` warning class that occurs when instrumentation counters for a small
  subset of functions (typically generic/monomorphized or duplicate-symbol functions linked into
  more than one of the 31 test binaries) do not perfectly reconcile across the merged profile. It
  did not prevent the report from completing and every file still received region/function/line
  figures; it is recorded here rather than hidden, per this plan's integrity requirement, and is a
  caveat on precision at the margins, not a reason to distrust the TOTAL row's order of magnitude.
- The Docker-backed `--features integration-tests` CI scope (Redis- and MinIO-dependent test paths
  in `.github/workflows/integration-tests.yml:110-123`) was not exercised — Docker is entirely
  absent from this environment. The recorded 84.79% is a default-feature-workspace figure, not the
  CI figure, and the two are not expected to match.
- One target file, `target/debug/build/utoipa-swagger-ui-10aed8599aeed486/out/embed.rs`, appears in
  the table at 0.00% — this is a build-script-generated file under `target/`, not first-party
  source; it contributes a single missed region/function/line to the TOTAL row's denominator (an
  immaterial rounding-level effect at this scale) and is noted here rather than silently included.
- This 84.79% figure is a fresh measurement produced by the commands above against commit
  `9be788c8e9c744ec3a6aad20b64110fb85925de4` on `worktree-agent-aaaa928ffbc589613`. It is **not**
  any of the four documented stale/contested baselines this project has previously recorded —
  60.88% unit / 67.79% integration (Milestone 1), ~78% overall (Milestone 3 release notes), or
  76-77% (Deferred-QA, February 2026) — and no coincidence with any of them was found or is claimed.

No `.profraw`, `.profdata`, `.lcov`, or coverage HTML artifact was staged into git for this
measurement (`target/` remains gitignored throughout).
