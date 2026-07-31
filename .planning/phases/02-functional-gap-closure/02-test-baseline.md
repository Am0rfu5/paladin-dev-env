# Test Baseline — Raw Evidence Record (D-01)

This file is raw evidence only: verbatim commands, verbatim tool output, toolchain versions,
commit SHA, branch, and UTC dates. It carries no gate, floor, or target value — no coverage
figure is created or implied here (D-04 forbids a second coverage number in this phase).
Follows the shape of `01-coverage-measurement.md`.

## Environment and commit provenance

Five probes run immediately before the workspace test command, in this order.

Command: `date -u`

```
Fri Jul 31 21:35:46 UTC 2026
```

Command: `git rev-parse HEAD`

```
7e55655ccf58e7f010f5fbbf9c2504f30a0ea29c
```

Command: `git branch --show-current`

```
worktree-agent-a780d29731d59c594
```

(This is a per-agent worktree branch created off `release/v0.7.0` for this plan's execution,
identical in kind to the worktree branch Phase 1's coverage-measurement record used. The commit
above is the tip of that branch at the moment this run started — a clean base commit with no
uncommitted changes, `git status --short` empty immediately before the run.)

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

## Workspace baseline

Command:

```
cargo test --workspace
```

The full stdout is very large (thousands of individual `test ... ok` lines across 35 binaries and
doctest groups). Per this task's elision rule, the per-test lines are elided below because **no
test failed and no panic occurred anywhere in the run** — confirmed by `grep -c 'FAILED\|^error\|panicked at'`
against the captured output, which returns `0`. What is preserved, in order and verbatim, is every
`Running`/`Doc-tests` header line (naming the exact binary or doctest group) immediately followed
by its own `test result:` line — the arithmetic a reader re-derives the totals from — plus the
final lines of the run.

This command was run twice against the identical, unmodified working tree (the second run to
capture the exact process exit code alongside the result); both runs produced byte-identical
`test result:` lines in the same order. The pasted output below is the second run.

```
     Running unittests src/lib.rs (target/debug/deps/paladin-f1f83a2724a87af8)
test result: ok. 416 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 2.60s

     Running unittests src/main.rs (target/debug/deps/paladin-7c129fd34dcd4812)
test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/bin/paladin-server.rs (target/debug/deps/paladin_server-c23b29ba48c6b327)
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/agent_orchestrator_bridge.rs (target/debug/deps/agent_orchestrator_bridge-96d428820fbeaff6)
test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/integration/citadel_integration_test.rs (target/debug/deps/citadel_integration-5b0d3ff5b558c6d6)
test result: ok. 11 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

     Running tests/cli_isolation_test.rs (target/debug/deps/cli_isolation-050e2c48ea6f65e3)
test result: ok. 9 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/content_agent_bridge.rs (target/debug/deps/content_agent_bridge-ad6cb525cc9e43af)
test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/content_ingestion_pipeline.rs (target/debug/deps/content_ingestion_pipeline-cd761494fa7c0e94)
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/event_trigger_pipeline.rs (target/debug/deps/event_trigger_pipeline-229bd4f60a1b8c4c)
test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/functional.rs (target/debug/deps/functional-d81863342cf2d4ee)
test result: ok. 10 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.10s

     Running tests/integration/in_memory_sanctum_tests.rs (target/debug/deps/in_memory_sanctum_integration-0ada312f0eae37b0)
test result: ok. 17 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

     Running tests/lib.rs (target/debug/deps/lib-17252f0af9b90793)
test result: ok. 642 passed; 0 failed; 14 ignored; 0 measured; 0 filtered out; finished in 2.02s

     Running tests/integration/paladin_garrison_integration_test.rs (target/debug/deps/paladin_garrison_integration-6805188ef3031e54)
test result: ok. 12 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.31s

     Running tests/paladin_server_smoke.rs (target/debug/deps/paladin_server_smoke-8802dd5979e15ad8)
test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.29s

     Running tests/queue_port_contract.rs (target/debug/deps/queue_port_contract-ec2b58a58a3e640c)
test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/repository.rs (target/debug/deps/repository-a3d15147e598d3b2)
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/integration/system_log_integration_test.rs (target/debug/deps/system_log_integration-fcaeb0ef9e59d822)
test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.06s

     Running tests/unit/mod.rs (target/debug/deps/unit-529a249f56db9f06)
test result: ok. 394 passed; 0 failed; 11 ignored; 0 measured; 0 filtered out; finished in 0.38s

     Running tests/web_server_e2e.rs (target/debug/deps/web_server_e2e-4aa33b50787496e7)
test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.29s

     Running unittests src/lib.rs (target/debug/deps/paladin_core-a45399df545fc132)
test result: ok. 359 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.12s

     Running unittests src/lib.rs (target/debug/deps/paladin_battalion-b14273a07c32cca2)
test result: ok. 204 passed; 0 failed; 4 ignored; 0 measured; 0 filtered out; finished in 3.03s

     Running unittests src/lib.rs (target/debug/deps/paladin_content-dfd98a1827ef79c2)
test result: ok. 96 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.16s

     Running unittests src/lib.rs (target/debug/deps/paladin_doc_examples-9d678ff83ef7d1b2)
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/lib.rs (target/debug/deps/paladin_herald-08d3c4d2ab334eee)
test result: ok. 59 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

     Running unittests src/lib.rs (target/debug/deps/paladin_llm-586b6c3af4ff6414)
test result: ok. 65 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.92s

     Running unittests src/lib.rs (target/debug/deps/paladin_memory-ffead8d545d932da)
test result: ok. 76 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.86s

     Running unittests src/lib.rs (target/debug/deps/paladin_notifications-822968d0b7dec3d6)
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/lib.rs (target/debug/deps/paladin_ports-6c2e8249f6cbf402)
test result: ok. 76 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

     Running unittests src/lib.rs (target/debug/deps/paladin_storage-dfee8c3dbad29fc0)
test result: ok. 21 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.06s

     Running unittests src/lib.rs (target/debug/deps/paladin_web-d61e33f3a058f2d3)
test result: ok. 117 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 2.55s

     Running tests/auth_rbac.rs (target/debug/deps/auth_rbac-b2d96a1aa6f35d1a)
test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.34s

   Doc-tests paladin
test result: ok. 95 passed; 0 failed; 17 ignored; 0 measured; 0 filtered out; finished in 0.05s

   Doc-tests paladin_core
test result: ok. 49 passed; 0 failed; 37 ignored; 0 measured; 0 filtered out; finished in 0.03s

   Doc-tests paladin_battalion
test result: ok. 28 passed; 0 failed; 43 ignored; 0 measured; 0 filtered out; finished in 0.02s

   Doc-tests paladin_web
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

These are the final lines of the run — the `paladin_web` doctest group (0 doctests, alphabetically
last of the crates carrying any doctest group) is the last thing `cargo test --workspace` printed
before returning control to the shell.

**Exit status:** `0` — captured directly from the shell's `$?` immediately after the command
completed (`cargo test --workspace; echo "EXIT_CODE:$?"` printed `EXIT_CODE:0`), on the second of
the two identical runs described above.

### Arithmetic — total passed, failed, ignored, and binary/doctest-group count

35 `test result:` lines above, one per test binary/doctest group. Summing each column down the
list, in the order pasted:

**Passed:** 416 + 3 + 0 + 2 + 11 + 9 + 3 + 0 + 5 + 10 + 17 + 642 + 12 + 3 + 2 + 0 + 8 + 394 + 3 +
359 + 204 + 96 + 0 + 59 + 65 + 76 + 0 + 76 + 21 + 117 + 5 + 95 + 49 + 28 + 0 = **2790**

**Failed:** every one of the 35 lines reads `0 failed` → **0**

**Ignored:** 0+0+0+0+0+0+0+0+0+0+0+14+0+0+0+0+0+11+0+0+4+0+0+0+0+0+0+0+0+0+0+17+37+43+0 = **126**

**Binaries/doctest groups:** 35 (31 unittest/integration-test binaries + 4 doctest groups —
`paladin`, `paladin_core`, `paladin_battalion`, `paladin_web`; the other seven library crates in
the workspace produced no doctest group in this run, i.e. zero `///` doc-comment code examples
compiled into a runnable doctest for them, or `doctest = false` in their manifest)

**Total: 2790 passed / 0 failed / 126 ignored across 35 binaries/doctest-groups, on commit
`7e55655ccf58e7f010f5fbbf9c2504f30a0ea29c`.**

### Agreement with the prior recorded figures

`02-VALIDATION.md` § Test Infrastructure records: "2790 passed / 0 failed / 126 ignored across 35
binaries on commit `fb4b942`". This run's totals — **2790 / 0 / 126 across 35** — are numerically
identical to that prior figure, on a different (later) commit. This is not a coincidence flagged
as suspicious: `fb4b942` to `7e55655` spans only planning-document commits (Phase 1 close-out and
Phase 2 planning artifacts under `.planning/`), and this task's own acceptance criterion requires
`git diff --name-only` to show no file outside `.planning/` touched by this task — so no Rust
source changed between the two measurements, and an identical test-count outcome is exactly what
that predicts. **This run agrees with the prior recorded figure**, run live rather than trusted by
citation, satisfying D-01.
