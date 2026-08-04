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

## Success-criterion re-proof

Four named commands, one per requirement (GAP-05, GAP-01, GAP-02, GAP-04), run against the same
unmodified tree and commit as the workspace baseline above.

### GAP-05 / ROADMAP SC1 — Auto keyword routing

Command:

```
cargo test -p paladin-battalion test_auto_selects
```

Verbatim output:

```
     Running unittests src/lib.rs (target/debug/deps/paladin_battalion-792fc9c333130695)

running 7 tests
test commander::tests::test_auto_selects_formation_for_single_paladin ... ok
test commander::tests::test_auto_selects_council_for_discussion_keywords ... ok
test commander::tests::test_auto_selects_formation_for_sequential_keywords ... ok
test commander::tests::test_auto_selects_campaign_for_workflow_keywords ... ok
test commander::tests::test_auto_selects_chain_for_delegate_keywords ... ok
test commander::tests::test_auto_selects_grove_for_routing_keywords ... ok
test commander::tests::test_auto_selects_phalanx_for_parallel_keywords ... ok

test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 201 filtered out; finished in 0.00s
```

`test_auto_selects_campaign_for_workflow_keywords` — the ROADMAP's named "fails today" test —
passes: `... ok`.

**7 distinct `test_auto_selects_*` tests ran**, one per Auto-routing outcome, enumerating the "all
four keyword families" claim by name rather than by assertion (Formation appears twice — single-
Paladin and sequential-keyword routing are two separate tests both landing on Formation):

1. `test_auto_selects_formation_for_single_paladin`
2. `test_auto_selects_council_for_discussion_keywords`
3. `test_auto_selects_formation_for_sequential_keywords`
4. `test_auto_selects_campaign_for_workflow_keywords`
5. `test_auto_selects_chain_for_delegate_keywords`
6. `test_auto_selects_grove_for_routing_keywords`
7. `test_auto_selects_phalanx_for_parallel_keywords`

All 7 pass, 0 fail, 0 ignored.

Agrees with ledger row `REQ-commander-auto-selection` (`milestone-01.md:316`) —
"satisfied ... `test_auto_selects_campaign_for_workflow_keywords` passing with 0 failures", and
with the nested item at `milestone-01.md:317` recording the ROADMAP's "(FAILING - needs fix)"
premise as stale. This run corroborates both by direct re-execution rather than citation.

### GAP-01 / ROADMAP SC2 — Chain of Command

Command 1:

```
cargo test -p paladin-battalion chain_of_command
```

Verbatim output:

```
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.34s
     Running unittests src/lib.rs (target/debug/deps/paladin_battalion-792fc9c333130695)

running 2 tests
test chain_of_command_service::tests::test_service_construction ... ok
test chain_of_command_service::tests::test_validate_valid_chain ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 206 filtered out; finished in 0.00s
```

Command 2:

```
cargo test --test unit -- battalion::chain_of_command
```

Verbatim output:

```
     Running tests/unit/mod.rs (target/debug/deps/unit-72cc53f5fe0db3b9)

running 37 tests
test battalion::chain_of_command_service_tests::automatic_delegation_tests::test_automatic_delegation_with_invalid_selection ... ok
test battalion::chain_of_command_service_tests::automatic_delegation_tests::test_automatic_delegation_selects_single_specialist ... ok
test battalion::chain_of_command_service_tests::automatic_delegation_tests::test_automatic_delegation_commander_formats_context ... ok
test battalion::chain_of_command_service_tests::broadcast_delegation_tests::test_broadcast_executes_all_specialists ... ok
test battalion::chain_of_command_service_tests::broadcast_delegation_tests::test_broadcast_executes_concurrently ... ok
test battalion::chain_of_command_service_tests::delegation_result_tests::test_delegation_result_creation ... ok
test battalion::chain_of_command_service_tests::custom_delegation_tests::test_custom_delegation_with_logic_string ... ok
test battalion::chain_of_command_service_tests::broadcast_delegation_tests::test_broadcast_with_single_specialist ... ok
test battalion::chain_of_command_service_tests::automatic_delegation_tests::test_automatic_delegation_selects_multiple_specialists ... ok
test battalion::chain_of_command_service_tests::delegation_result_tests::test_delegation_result_with_multiple_specialists ... ok
test battalion::chain_of_command_service_tests::service_construction_tests::test_service_new_stores_port ... ok
test battalion::chain_of_command_service_tests::service_construction_tests::test_service_new_with_valid_port ... ok
test battalion::chain_of_command_service_tests::custom_delegation_tests::test_custom_delegation_includes_logic_in_reasoning ... ok
test battalion::chain_of_command_tests::chain_of_command_builder_tests::test_with_strategy_automatic ... ok
test battalion::chain_of_command_service_tests::round_robin_delegation_tests::test_round_robin_with_single_specialist ... ok
test battalion::chain_of_command_service_tests::round_robin_delegation_tests::test_round_robin_reasoning_includes_rotation ... ok
test battalion::chain_of_command_service_tests::round_robin_delegation_tests::test_round_robin_cycles_through_specialists ... ok
test battalion::chain_of_command_service_tests::service_validation_tests::test_validate_empty_chain_fails ... ok
test battalion::chain_of_command_tests::chain_of_command_builder_tests::test_with_strategy_broadcast ... ok
test battalion::chain_of_command_service_tests::service_validation_tests::test_validate_chain_of_command ... ok
test battalion::chain_of_command_tests::chain_of_command_builder_tests::test_with_strategy_custom ... ok
test battalion::chain_of_command_tests::chain_of_command_builder_tests::test_with_strategy_round_robin ... ok
test battalion::chain_of_command_tests::chain_of_command_construction_tests::test_chain_of_command_new_with_no_specialists_fails ... ok
test battalion::chain_of_command_tests::chain_of_command_construction_tests::test_chain_of_command_new_with_valid_setup ... ok
test battalion::chain_of_command_tests::chain_of_command_construction_tests::test_chain_of_command_with_single_specialist_succeeds ... ok
test battalion::chain_of_command_tests::chain_of_command_default_strategy_tests::test_default_strategy_is_automatic ... ok
test battalion::chain_of_command_tests::chain_of_command_validation_tests::test_commander_access ... ok
test battalion::chain_of_command_tests::chain_of_command_validation_tests::test_specialists_access ... ok
test battalion::chain_of_command_tests::chain_of_command_validation_tests::test_specialist_count ... ok
test battalion::chain_of_command_tests::chain_of_command_validation_tests::test_validate_with_multiple_specialists_succeeds ... ok
test battalion::chain_of_command_tests::delegation_strategy_tests::test_delegation_strategy_automatic ... ok
test battalion::chain_of_command_tests::chain_of_command_validation_tests::test_validate_with_valid_chain_succeeds ... ok
test battalion::chain_of_command_tests::delegation_strategy_tests::test_delegation_strategy_broadcast ... ok
test battalion::chain_of_command_tests::delegation_strategy_tests::test_delegation_strategy_clone ... ok
test battalion::chain_of_command_tests::delegation_strategy_tests::test_delegation_strategy_custom ... ok
test battalion::chain_of_command_tests::delegation_strategy_tests::test_delegation_strategy_debug ... ok
test battalion::chain_of_command_tests::delegation_strategy_tests::test_delegation_strategy_round_robin ... ok

test result: ok. 37 passed; 0 failed; 0 ignored; 0 measured; 368 filtered out; finished in 0.00s
```

**The four delegation-strategy test modules named in the output**, exactly as GAP-01 requires them
enumerated:

1. `automatic_delegation_tests`
2. `broadcast_delegation_tests`
3. `round_robin_delegation_tests`
4. `custom_delegation_tests`

Command 3 — the "runnable from an example" half of SC2:

```
ls examples/chain_of_command_delegation.rs
```

Verbatim output:

```
examples/chain_of_command_delegation.rs
```

The file exists at the named path.

Agrees with ledger row `REQ-chain-of-command-execution` (`milestone-01.md:294`) —
"satisfied ... a full run of `cargo test --test lib chain_of_command` on 2026-07-31 shows 54
passed, 0 failed, 0 ignored" — and with the nested item at `milestone-01.md:299` recording the
stale Task 6.0 parent checkbox as satisfied. This run's own totals (2 + 37 = 39 unit-level tests
across the two commands run here, a narrower slice than the ledger's full `--test lib
chain_of_command` sweep which also includes the integration-level tests) are consistent with that
row: 0 failures across both commands, all four delegation strategies present by name, and the
example file confirmed present.

### GAP-02 — Battalion integration/performance tests

Command 1:

```
cargo test --test lib -- integration::battalion::load_test
```

Verbatim output:

```
     Running tests/lib.rs (target/debug/deps/lib-dc15202b3e48a7b9)

running 5 tests
test integration::battalion::load_test::test_performance_orchestration_overhead ... ok
test integration::battalion::load_test::test_stress_high_concurrency_limit ... ok
test integration::battalion::load_test::test_load_phalanx_concurrent_execution ... ok
test integration::battalion::load_test::test_load_formation_50_concurrent_battalions ... ok
test integration::battalion::load_test::test_memory_efficiency_under_load ... ok

test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 646 filtered out; finished in 0.38s
```

Command 2:

```
cargo test --test lib -- integration::battalion
```

Verbatim output:

```
     Running tests/lib.rs (target/debug/deps/lib-dc15202b3e48a7b9)

running 74 tests
test integration::battalion::campaign_integration_test::test_empty_campaign_validation ... ok
test integration::battalion::campaign_integration_test::test_cycle_detection_prevents_execution ... ok
test integration::battalion::campaign_integration_test::test_linear_graph_chain_execution ... ok
test integration::battalion::campaign_integration_test::test_branching_graph_failure_path ... ok
test integration::battalion::campaign_integration_test::test_multiple_independent_branches ... ok
test integration::battalion::campaign_integration_test::test_complex_dag_with_fan_out_fan_in ... ok
test integration::battalion::campaign_integration_test::test_self_loop_detection ... ok
test integration::battalion::campaign_integration_test::test_branching_graph_conditional_routing ... ok
test integration::battalion::chain_of_command_integration_test::test_custom_delegation_end_to_end ... ok
test integration::battalion::chain_of_command_integration_test::test_automatic_delegation_with_multiple_specialists_selected ... ok
test integration::battalion::chain_of_command_integration_test::test_automatic_delegation_end_to_end ... ok
test integration::battalion::chain_of_command_integration_test::test_round_robin_cycling_end_to_end ... ok
test integration::battalion::council_integration_test::test_council_error_handling ... ok
test integration::battalion::council_integration_test::test_council_moderator_directed_strategy ... ok
test integration::battalion::chain_of_command_integration_test::test_broadcast_delegation_end_to_end ... ok
test integration::battalion::council_integration_test::test_council_roundrobin_three_paladins_two_rounds ... ok
test integration::battalion::council_integration_test::test_council_consensus_termination ... ok
test integration::battalion::campaign_integration_test::test_regex_pattern_routing ... ok
test integration::battalion::formation_integration_test::test_formation_failfast_error_handling ... ok
test integration::battalion::formation_integration_test::test_formation_continue_on_error ... ok
test integration::battalion::formation_integration_test::test_formation_end_to_end_success ... ok
test integration::battalion::grove_integration_test::test_grove_error_handling ... ok
test integration::battalion::formation_integration_test::test_formation_output_chaining ... ok
test integration::battalion::grove_integration_test::test_grove_fallback_behavior ... ok
test integration::battalion::formation_integration_test::test_formation_with_shared_context ... ok
test integration::battalion::formation_integration_test::test_formation_multiple_failures_continue_on_error ... ok
test integration::battalion::grove_integration_test::test_grove_llm_routing ... ok
test integration::battalion::grove_integration_test::test_grove_keyword_match_routing ... ok
test integration::battalion::grove_integration_test::test_grove_no_fallback_default_behavior ... ok
test integration::battalion::grove_integration_test::test_grove_semantic_similarity_routing ... ok
test integration::battalion::grove_integration_test::test_grove_llm_routing_end_to_end ... ok
test integration::battalion::formation_integration_test::test_formation_retry_then_continue ... ok
test integration::battalion::council_integration_test::test_council_timeout_enforcement ... ok
test integration::battalion::grove_integration_test::test_grove_multiple_trees ... ok
test integration::battalion::load_test::test_performance_orchestration_overhead ... ok
test integration::battalion::formation_integration_test::test_formation_large_pipeline ... ok
test integration::battalion::phalanx_integration_test::test_phalanx_collect_all_aggregation ... ok
test integration::battalion::load_test::test_load_phalanx_concurrent_execution ... ok
test integration::battalion::phalanx_integration_test::test_phalanx_concurrent_execution_with_10_paladins ... ok
test integration::battalion::phalanx_integration_test::test_phalanx_first_success_aggregation ... ok
test integration::battalion::phalanx_integration_test::test_phalanx_majority_aggregation_with_consensus ... ok
test integration::battalion::load_test::test_stress_high_concurrency_limit ... ok
test integration::battalion::phalanx_integration_test::test_phalanx_majority_no_consensus_fails ... ok
test integration::battalion::phalanx_integration_test::test_phalanx_mixed_success_and_failure ... ok
test integration::battalion::phalanx_integration_test::test_phalanx_performance_overhead ... ok
test integration::battalion::phalanx_integration_test::test_phalanx_partial_failures_continue_on_error ... ok
test integration::battalion_campaign_integration_test::test_branching_campaign_fan_out ... ok
test integration::battalion_campaign_integration_test::test_campaign_execution_timeout ... ok
test integration::battalion_campaign_integration_test::test_campaign_service_creation ... ok
test integration::battalion_campaign_integration_test::test_campaign_with_edge_transform ... ok
test integration::battalion_campaign_integration_test::test_complex_workflow ... ok
test integration::battalion_campaign_integration_test::test_diamond_graph_campaign ... ok
test integration::battalion_campaign_integration_test::test_empty_campaign_validation ... ok
test integration::battalion_campaign_integration_test::test_linear_campaign_execution ... ok
test integration::battalion_campaign_integration_test::test_multiple_entry_points ... ok
test integration::battalion_campaign_integration_test::test_single_paladin_campaign ... ok
test integration::battalion_chain_of_command_integration_test::test_automatic_delegation_with_specialist_selection ... ok
test integration::battalion_chain_of_command_integration_test::test_broadcast_delegation ... ok
test integration::battalion_chain_of_command_integration_test::test_chain_of_command_service_creation ... ok
test integration::battalion_chain_of_command_integration_test::test_chain_with_config_timeout ... ok
test integration::battalion_chain_of_command_integration_test::test_concurrent_broadcasts ... ok
test integration::battalion_chain_of_command_integration_test::test_delegation_result_structure ... ok
test integration::battalion_chain_of_command_integration_test::test_different_delegation_strategies_same_chain ... ok
test integration::battalion_chain_of_command_integration_test::test_multi_specialist_automatic_selection ... ok
test integration::battalion_chain_of_command_integration_test::test_round_robin_delegation ... ok
test integration::battalion_chain_of_command_integration_test::test_simple_delegation_single_specialist ... ok
test integration::battalion_chain_of_command_integration_test::test_validation_no_specialists ... ok
test integration::battalion::phalanx_integration_test::test_phalanx_large_scale_concurrent_execution ... ok
test integration::battalion::load_test::test_load_formation_50_concurrent_battalions ... ok
test integration::battalion::phalanx_integration_test::test_phalanx_cancellation_support ... ok
test integration::battalion::phalanx_integration_test::test_phalanx_concurrency_limiting ... ok
test integration::battalion::load_test::test_memory_efficiency_under_load ... ok
test integration::battalion::formation_integration_test::test_formation_timeout_enforcement ... ok
test integration::battalion::phalanx_integration_test::test_phalanx_timeout_enforcement ... ok

test result: ok. 74 passed; 0 failed; 0 ignored; 0 measured; 577 filtered out; finished in 1.18s
```

**Naming the two performance claims by test name, as required:**

- **The ≥ 10 concurrent Paladins claim** is exercised by
  `integration::battalion::phalanx_integration_test::test_phalanx_concurrent_execution_with_10_paladins`
  — `... ok` in the Command 2 output above. `test_load_phalanx_concurrent_execution` and
  `test_stress_high_concurrency_limit` (both in `load_test`) additionally exercise concurrency
  under load, but the "≥ 10" figure specifically is what the `_with_10_paladins`-named test
  targets.
- **The < 1 s orchestration-overhead claim** is exercised by
  `integration::battalion::load_test::test_performance_orchestration_overhead` — `... ok` in both
  commands above.

Neither claim is left to inference — both are named, both pass.

**Existence bar per D-07 — one non-`#[ignore]`d integration exerciser per Battalion pattern, with
ignored count:**

| Pattern | Exercising file | Non-ignored? | Ignored count in this run |
|---|---|---|---|
| Formation | `tests/integration/battalion/formation_integration_test.rs` (module `integration::battalion::formation_integration_test`) | Yes — 8 tests, all `... ok` | 0 |
| Phalanx | `tests/integration/battalion/phalanx_integration_test.rs` (module `integration::battalion::phalanx_integration_test`) | Yes — 12 tests, all `... ok`, including the `_with_10_paladins` test above | 0 |
| Campaign | `tests/integration/battalion/campaign_integration_test.rs` (module `integration::battalion::campaign_integration_test`) and the legacy `tests/integration/battalion_campaign_integration_test.rs` (module `integration::battalion_campaign_integration_test`) — both compile and run | Yes — 8 + 10 tests, all `... ok` | 0 |
| Chain of Command | `tests/integration/battalion/chain_of_command_integration_test.rs` (module `integration::battalion::chain_of_command_integration_test`) and the legacy `tests/integration/battalion_chain_of_command_integration_test.rs` (module `integration::battalion_chain_of_command_integration_test`) — both compile and run | Yes — 4 + 11 tests, all `... ok` | 0 |

All 74 tests in Command 2's output read `... ok`; the run's own summary line confirms
`0 ignored` for this filtered slice. Every one of the four Battalion patterns meets the D-07
existence bar — none is a pattern whose only exerciser is behind `#[ignore]`.

**Restating D-07's boundary explicitly, as this subsection is required to do:** existence is
Phase 2's bar, proven above. Depth — raising coverage (QUAL-01/QUAL-02), un-ignoring the four
empty-bodied Commander error tests at `commander.rs:2181,2189,2197,2205` (QUAL-04), and MCP
failure-mode tests — is Phase 3's, and the shared `Send + Sync` failing-mock harness those tests
need should be built there as a shared asset, because Phase 15's DEFER register names the same
prerequisite (DEFER-01).

Agrees with ledger row `REQ-phalanx-concurrency` (`milestone-01.md:288`) —
"satisfied ... validated under real load by `test_load_phalanx_concurrent_execution`
(`load_test.rs:192`) and `test_stress_high_concurrency_limit` (`load_test.rs:273`)" — and the
Epic 4 nested item at `milestone-01.md:300` recording task 7.0 (Integration Testing, Performance
Validation) as satisfied with the same five `load_test.rs` tests. This run's five-test and
seventy-four-test slices both corroborate those rows directly.

### GAP-04 / ROADMAP SC4 — Commander result normalization and telemetry

Command:

```
cargo test -p paladin-battalion commander
```

Verbatim output:

```
     Running unittests src/lib.rs (target/debug/deps/paladin_battalion-792fc9c333130695)

running 54 tests
test commander::tests::test_auto_defaults_to_formation_when_uncertain ... ok
test commander::tests::test_auto_selects_campaign_for_workflow_keywords ... ok
test commander::tests::test_auto_prioritizes_keywords_over_count ... ok
test commander::tests::test_auto_selection_is_case_insensitive ... ok
test commander::tests::test_auto_selects_chain_for_delegate_keywords ... ok
test commander::tests::test_auto_selects_formation_for_single_paladin ... ok
test commander::tests::test_auto_selects_council_for_discussion_keywords ... ok
test commander::tests::test_auto_selects_grove_for_routing_keywords ... ok
test commander::tests::test_auto_selects_formation_for_sequential_keywords ... ok
test commander::tests::test_auto_selects_phalanx_for_parallel_keywords ... ok
test commander::tests::test_auto_strategy_does_not_select_maneuver ... ok
test commander::tests::test_commander_builder_empty_paladins ... ok
test commander::tests::test_commander_builder_invalid_config ... ok
test commander::tests::test_commander_builder_missing_paladins ... ok
test commander::tests::test_commander_builder_missing_strategy ... ok
test commander::tests::test_commander_builder_success ... ok
test commander::tests::test_commander_builder_with_error_strategy ... ok
test commander::tests::test_commander_builder_with_flow_expression ... ok
test commander::tests::test_continue_on_error_collects_all_errors ... ignored
test commander::tests::test_commander_build_without_metadata_dir ... ok
test commander::tests::test_commander_builder_with_maneuver_config ... ok
test commander::tests::test_council_requires_multiple_paladins ... ok
test commander::tests::test_config_passthrough_to_services ... ok
test commander::tests::test_council_and_grove_keywords_are_case_insensitive ... ok
test commander::tests::test_default_config_generation ... ok
test commander::tests::test_commander_all_strategies ... ok
test commander::tests::test_commander_build_with_valid_metadata_dir ... ok
test commander::tests::test_error_handling_continue_on_error ... ok
test commander::tests::test_error_handling_retry_then_continue ... ok
test commander::tests::test_error_handling_fail_fast ... ok
test commander::tests::test_fail_fast_stops_on_first_error ... ignored
test commander::tests::test_grove_requires_multiple_paladins ... ok
test commander::tests::test_maneuver_requires_at_least_one_paladin ... ok
test commander::tests::test_maneuver_with_invalid_flow_expression_fails ... ok
test commander::tests::test_execute_resolves_auto_strategy ... ok
test commander::tests::test_execute_routes_to_chain_service ... ok
test commander::tests::test_execute_routes_to_phalanx_service ... ok
test commander::tests::test_execute_routes_to_campaign_service ... ok
test commander::tests::test_maneuver_execution_through_commander ... ok
test commander::tests::test_maneuver_without_flow_expression_fails ... ok
test commander::tests::test_maneuver_with_nested_pattern ... ok
test commander::tests::test_maneuver_strategy_explicit ... ok
test commander::tests::test_partial_results_returned_with_errors ... ignored
test commander::tests::test_metadata_export_no_dir_configured ... ok
test commander::tests::test_maneuver_with_parallel_pattern ... ok
test commander::tests::test_partial_failure_handling ... ok
test commander::tests::test_result_contains_strategy_used ... ok
test commander::tests::test_retry_then_continue_retries_failed_paladins ... ignored
test commander::tests::test_result_contains_selection_reasoning ... ok
test commander::tests::test_metadata_export_creates_file ... ok
test commander::tests::test_result_contains_telemetry_metadata ... ok
test commander::tests::test_metadata_export_correct_naming ... ok
test commander::tests::test_metadata_export_json_structure ... ok
test commander::tests::test_timeout_enforcement ... ok

test result: ok. 50 passed; 0 failed; 4 ignored; 0 measured; 154 filtered out; finished in 1.01s
```

50 passed, 0 failed, 4 ignored — the 4 ignored are the same empty-bodied edge-case tests the
ledger already names (`test_fail_fast_stops_on_first_error`,
`test_continue_on_error_collects_all_errors`, `test_retry_then_continue_retries_failed_paladins`,
`test_partial_results_returned_with_errors`), owned by Phase 3's QUAL-04, not this phase.

**Source-site citations for the two GAP-04 halves, read from the current tree (not copied from
research):**

- **Post-dispatch enrichment of `strategy_used` / `strategy_selection_reasoning` /
  `strategy_selection_time_ms`:** `crates/paladin-battalion/src/commander.rs:847-849`
  ```rust
  result.strategy_used = effective_strategy.clone();
  result.strategy_selection_reasoning = selection_reason.clone();
  result.strategy_selection_time_ms = selection_time_ms;
  ```
- **`export_metadata` writing to `metadata_output_dir`:**
  `crates/paladin-battalion/src/commander.rs:880-881`
  ```rust
  fn export_metadata(&self, result: &BattalionResult) {
      let Some(dir) = &self.config.metadata_output_dir else {
  ```
  (function continues past line 881; the `let Some(dir) = &self.config.metadata_output_dir else`
  guard at line 881 is the exact site gating the write on whether the field is configured.)

Agrees with ledger row `REQ-commander-result-normalization`
(`milestone-01.md:319-322`) — and with `REQ-commander-telemetry` (`milestone-01.md:326`) — both
"satisfied", with the same `commander.rs:880` `export_metadata` citation this run's live grep
confirms at `:880-881` (the ledger's own note already records this exact one-line drift from an
earlier `:870` citation as "a small drift consistent with intervening commits, not a
contradiction"). `REQ-commander-error-strategy` (`milestone-01.md:323`) is also corroborated: base
capability satisfied, the same 4 edge-case tests still `#[ignore]`d with empty bodies, forward
note QUAL-04 unchanged.

## GAP-05 finding, restated at source

SC1 names `test_auto_selects_campaign_for_workflow_keywords` as failing today. It does not — the
GAP-05 re-proof above shows it passing live, on this commit, alongside all 6 sibling
`test_auto_selects_*` tests. The ledger already records this at `milestone-01.md:316-317` as
`satisfied`, with the January task list's own line 99 (`tasks-commander-strategy-router.md:99`,
"(FAILING - needs fix)") flagged as a stale checkbox rather than a live defect. Whatever caused
the original January failure has left no trace in the current tree. Plan 02-09 is the forward
owner for amending ROADMAP.md's Phase 2 SC1 wording to drop the stale "fails today" premise (D-02).

## Flagged open assumption — GAP-04 edge-probe classification (unresolved, carried forward)

The edge probe that ran ahead of this phase classified GAP-04 as `unclassified`, and it remains
unresolved after this re-proof. This run's four-command GAP-04 re-proof above establishes that the
*base* capability — result normalization and telemetry export — is `satisfied` by a named,
passing exerciser. What it does **not** establish, and what no artifact in this planning corpus
derives, is any of the following predicates for Commander result normalization:

- A **boundary** predicate — e.g. what happens at zero strategies considered, or at the maximum
  representable `strategy_selection_time_ms`.
- An **ordering** predicate — when two candidate strategies would produce equally valid results,
  which one `strategy_used` reports, and whether that choice is stable across runs.
- An **empty-input** predicate — what `analyze_and_select` and the subsequent normalization do
  when given no Paladins and no keywords at all (distinct from the `test_commander_builder_empty_paladins`
  *construction-time* rejection test above, which fires before `execute()` — and thus before
  result normalization — is ever reached).
- A **precision** predicate — whether `strategy_selection_time_ms`'s millisecond rounding is
  specified behavior or an implementation accident.

None of these was derived by the edge probe, and none is derived by this plan (D-01 scopes this
plan to re-proof by execution, not to deriving new edge-case predicates). This assumption is
recorded here rather than silently dropped, per this plan's own action instructions. **Phase 3's
QUAL work is the natural candidate owner**, alongside the other Commander depth work D-07 already
routes there (raising coverage, un-ignoring the four empty-bodied error tests, MCP failure-mode
tests) — but no phase has formally claimed it, and this record does not claim one on its behalf.
