# Critical-Path Exercisers — QUAL-03 Evidence at the D-19 Bar

This file is raw evidence only: verified `file:line` citations plus the exact Rust identifier of a
named, passing, non-`#[ignore]`d integration test for each of QUAL-03's three critical paths, run
under the default parallel `cargo test --offline` harness — the thread-count-limiting flag that
would force single-threaded execution is never passed anywhere in this record. Per
`.planning/decisions/0006-coverage-gate.md` and `03-CONTEXT.md` D-08, QUAL-03's percentage clause is
superseded; no coverage percentage is recorded anywhere below.

Evidence bar (`01-CONTEXT.md` D-19, `ledgers/milestone-01.md` §Verdict legend): `satisfied` requires
a `file:line` citation **plus** a named passing test, example, or command that exercises it. A
citation with nothing exercising it is `present, unproven`, not `satisfied` — every row below meets
the stricter bar.

Commit measured against: `1ad8be53d1ffd383e5ed45b35b04c9a7ab4abde1` (branch `release/v0.7.0`), same
commit as this plan's exit coverage measurement.

## Paladin execution

**Exerciser:** `test_end_to_end_paladin_execution` — `tests/integration/paladin_integration_test.rs:19`

Drives the complete flow named by QUAL-03 end to end: `PaladinBuilder` constructs a `Paladin` wired
to a `MockLlmAdapter` and a `CircuitBreaker`, then `PaladinExecutionService::execute` is called
directly and its `PaladinResult` (output, loop count, token count) is asserted. No `#[ignore]`
attribute on this test or its enclosing module.

Command:

```
cargo test --offline --test lib integration::paladin_integration_test::test_end_to_end_paladin_execution -- --exact
```

Output:

```
    Finished `test` profile [unoptimized + debuginfo] target(s) in 12.51s
     Running tests/lib.rs (target/debug/deps/lib-dc15202b3e48a7b9)

running 1 test
test integration::paladin_integration_test::test_end_to_end_paladin_execution ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 695 filtered out; finished in 0.00s
```

## Battalion orchestration

**Exerciser 1:** `test_commander_executes_formation_end_to_end` — `tests/integration/commander_integration_tests.rs:150`

Drives `Commander` routing a request through the Formation battalion pattern end to end against a
mock `PaladinPort`, asserting the orchestrated result. No `#[ignore]` attribute.

Command:

```
cargo test --offline --test lib integration::commander_integration_tests::test_commander_executes_formation_end_to_end -- --exact
```

Output:

```
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.54s
     Running tests/lib.rs (target/debug/deps/lib-dc15202b3e48a7b9)

running 1 test
test integration::commander_integration_tests::test_commander_executes_formation_end_to_end ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 695 filtered out; finished in 0.04s
```

**Exerciser 2:** `test_load_formation_50_concurrent_battalions` — `tests/integration/battalion/load_test.rs:102`

Drives 50 concurrent Formation battalions (each with 10 Paladins) through the same orchestration
path under load, proving Battalion orchestration holds under concurrency, not just a single
happy-path call. No `#[ignore]` attribute.

Command:

```
cargo test --offline --test lib integration::battalion::load_test::test_load_formation_50_concurrent_battalions -- --exact
```

Output:

```
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.53s
     Running tests/lib.rs (target/debug/deps/lib-dc15202b3e48a7b9)

running 1 test
test integration::battalion::load_test::test_load_formation_50_concurrent_battalions ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 695 filtered out; finished in 0.12s
```

## Tool invocation

**Exerciser 1:** `function_call_dispatch_still_invokes_arsenal_exactly_once_with_matching_call` — `tests/integration/arsenal_bridge_regression_test.rs:165`

Drives a fake `LlmPort` returning a `function_call`-carrying response through
`PaladinExecutionService`'s Layer-3 dispatch (`function_call -> handle_tool_call -> arsenal.invoke`)
against a spy `ArsenalPort`, asserting the spy's `invoke` was called exactly once with an
`ArmamentCall` whose `tool_name`/`arguments` match the LLM's function call. This is the file the
sibling `arsenal_execution_integration_test.rs`'s own doc comment (lines 23-26) names as the proof
of "the end-to-end LLM-driven dispatch" — that file's own tests deliberately construct
`ArsenalExecutionService` with no MCP client registered and so only prove validation and
`ToolNotFound` routing, not a successful invocation; this file's test is the one that proves the
successful dispatch path. No `#[ignore]` attribute.

Command:

```
cargo test --offline --test lib integration::arsenal_bridge_regression_test::function_call_dispatch_still_invokes_arsenal_exactly_once_with_matching_call -- --exact
```

Output:

```
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.62s
     Running tests/lib.rs (target/debug/deps/lib-dc15202b3e48a7b9)

running 1 test
test integration::arsenal_bridge_regression_test::function_call_dispatch_still_invokes_arsenal_exactly_once_with_matching_call ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 695 filtered out; finished in 0.00s
```

**Exerciser 2:** `streamable_http_round_trip_with_correct_bearer_token_succeeds` — `tests/integration/mcp_streamable_http_test.rs:342`

Drives the full `initialize -> notifications/initialized -> tools/list -> tools/call` round-trip
against a real in-process spec-strict MCP Streamable-HTTP server fixture with a correct bearer
token, proving tool invocation over the wire protocol Paladin ships (not the SSE transport named
in the superseded Milestone-1 PRD — see `ledgers/milestone-01.md`'s divergence row for
`REQ-mcp-sse-transport`, which already cites this same test). No `#[ignore]` attribute.

Command:

```
cargo test --offline --test lib integration::mcp_streamable_http_test::streamable_http_round_trip_with_correct_bearer_token_succeeds -- --exact
```

Output:

```
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.56s
     Running tests/lib.rs (target/debug/deps/lib-dc15202b3e48a7b9)

running 1 test
test integration::mcp_streamable_http_test::streamable_http_round_trip_with_correct_bearer_token_succeeds ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 695 filtered out; finished in 0.07s
```

## Shared citations

None. Each of the five exercisers above is cited under exactly one critical path — no test in this
record is claimed for two paths. `function_call_dispatch_still_invokes_arsenal_exactly_once_with_matching_call`
does construct a `Paladin` via `PaladinBuilder` as scaffolding to reach the dispatch call, but its
own assertions are entirely about the Arsenal `invoke` call, not about `PaladinResult` content — it
is cited under tool invocation only, and `test_end_to_end_paladin_execution` remains the sole,
un-shared Paladin-execution exerciser.

## Path status

All three of QUAL-03's named critical paths are `satisfied` at the D-19 bar — none is
`genuinely outstanding`.

## Percentage clause

QUAL-03's original percentage clause — a numeric integration-coverage threshold expressed as "at or
above" a stated target, raised up from a stated numeric baseline — is recorded **superseded
by ADR-0006** (`.planning/decisions/0006-coverage-gate.md`), which abolished a second coverage
number under a second scope in favor of one workspace-wide line-coverage figure — the same
disposition `03-CONTEXT.md` D-08 records and the same disposition Phase 2's D-04 gave the
structurally identical Epic 2 task 11.5 clause. This phase therefore records no coverage
percentage for QUAL-03 anywhere in this file, or in `03-coverage-measurement.md`'s exit section, or
in any other artifact it writes. QUAL-03's surviving substance — named, passing, non-`#[ignore]`d
exercisers for each of the three critical paths — is satisfied above. **Plan 03-08** is where the
amendment to QUAL-03's text lands at source in `REQUIREMENTS.md`.
