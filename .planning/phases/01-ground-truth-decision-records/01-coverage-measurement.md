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
