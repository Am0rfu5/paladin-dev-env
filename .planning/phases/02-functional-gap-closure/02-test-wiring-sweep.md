# D-12 Test-Wiring Sweep

**Produced:** 2026-08-01, plan 02-09, Phase 2 — Functional Gap Closure
**Method:** Every `.rs` file under `tests/`, `benches/` and `examples/` was enumerated
(`find tests benches examples -name '*.rs'`) and cross-checked against two independent sources of
"what actually compiled": the `[[test]]`/`[[bench]]`/`[[example]]` target declarations in `Cargo.toml`
(and each crate's own `Cargo.toml` for `benches/`), and every barrel `mod.rs`/`#[path]` declaration
reachable from a compiled root. Where a file is a direct child of `tests/`, `benches/` or `examples/`
with no explicit `[[…]]` entry, Cargo's default autodiscovery (`autotests`/`autobenches`/`autoexamples`
all unset, so all default to `true` — confirmed: no `autotests`/`autobenches`/`autoexamples` key
appears anywhere in the root `Cargo.toml`) makes it its own compiled target automatically, named after
its filename. `cargo test --workspace -- --list` gives the authoritative "what actually compiled"
side under the default feature set (`default = ["llm-openai"]`); `cargo test --features cli --test
cli -- --list` supplies the `cli`-gated slice the default run excludes.

Per D-12/CONTEXT.md's scope: fixing anything found beyond the two already-closed instances (the LLM
unit module, the five CLI execution/error/tool suites) is a separate decision, not automatic. This
record reports; it does not repair.

## Raw evidence — `cargo test --workspace -- --list`, summarised by target

Run against the tree at the commit this phase's plans have been building on
(`.planning/phases/02-functional-gap-closure/02-test-baseline.md`'s baseline commit
`7e55655ccf58e7f010f5fbbf9c2504f30a0ea29c`, unchanged by any Rust-source edit since — every plan from
02-02 onward recorded a clean `cargo test --workspace` at its own commit, and this sweep's Task 1
makes no source change either). **35 binaries/doctest-groups**, matching plan 02-01's baseline count
exactly:

```
  418  unittests src/lib.rs (paladin)
    3  unittests src/main.rs (paladin)
    0  unittests src/bin/paladin-server.rs (paladin_server)
    2  tests/agent_orchestrator_bridge.rs (agent_orchestrator_bridge)
   11  tests/integration/citadel_integration_test.rs (citadel_integration)
    9  tests/cli_isolation_test.rs (cli_isolation)
    3  tests/content_agent_bridge.rs (content_agent_bridge)
    0  tests/content_ingestion_pipeline.rs (content_ingestion_pipeline)
    5  tests/event_trigger_pipeline.rs (event_trigger_pipeline)
   10  tests/functional.rs (functional)
   17  tests/integration/in_memory_sanctum_tests.rs (in_memory_sanctum_integration)
  685  tests/lib.rs (lib)
   12  tests/integration/paladin_garrison_integration_test.rs (paladin_garrison_integration)
    3  tests/paladin_server_smoke.rs (paladin_server_smoke)
    2  tests/queue_port_contract.rs (queue_port_contract)
    0  tests/repository.rs (repository)               [storage-mysql gated; 0 under default features]
    8  tests/integration/system_log_integration_test.rs (system_log_integration)
  430  tests/unit/mod.rs (unit)
    3  tests/web_server_e2e.rs (web_server_e2e)
  361  unittests src/lib.rs (paladin_core)
  210  unittests src/lib.rs (paladin_battalion)
   96  unittests src/lib.rs (paladin_content)
    0  unittests src/lib.rs (paladin_doc_examples)
   66  unittests src/lib.rs (paladin_herald)
   67  unittests src/lib.rs (paladin_llm)
   76  unittests src/lib.rs (paladin_memory)
    0  unittests src/lib.rs (paladin_notifications)
   77  unittests src/lib.rs (paladin_ports)
   21  unittests src/lib.rs (paladin_storage)
  117  unittests src/lib.rs (paladin_web)
  112  Doc-tests paladin
   86  Doc-tests paladin_core
   71  Doc-tests paladin_battalion
    0  Doc-tests paladin_web
    5  tests/auth_rbac.rs (crates/paladin-web's own tests/ dir — see Scope note below)
------
 2986  TOTAL test functions across 35 groups
```

Plus, separately (requires `--features cli`, not part of the default-feature run above):

```
$ cargo test --features cli --test cli -- --list
99 tests, 0 benchmarks
```
(37 from the five suites plan 02-07 reactivated + 43 from four pre-existing CLI snapshot suites + 19
from `tests/helpers/`'s own `#[cfg(test)]` unit tests, pulled in transitively by the `cli` target's
helper shim.)

## Sweep table

| Path | How reached | Status |
|---|---|---|
| **`tests/unit/` (root, 25 files)** | Each declared by a `pub mod <name>;` line in `tests/unit/mod.rs`, which is itself the `[[test]] name = "unit"` target (`Cargo.toml:171-173`) | reachable — all 25 |
| `tests/unit/{cli_agent,cli_arsenal,cli_battalion}_commands_test.rs` | `pub mod` lines gated `#[cfg(feature = "cli")]` in `tests/unit/mod.rs` | reachable, conditionally (compiles under `--features cli`) |
| `tests/unit/arsenal/handoff_tool_test.rs` | `pub mod handoff_tool_test;` in `tests/unit/arsenal/mod.rs`, reached via `pub mod arsenal;` in `tests/unit/mod.rs` | reachable |
| `tests/unit/battalion/{campaign_service,campaign,chain_of_command_service,chain_of_command,formation,phalanx}_tests.rs` (6 files) | Each declared in `tests/unit/battalion/mod.rs`, reached via `pub mod battalion;` in `tests/unit/mod.rs` | reachable — all 6 |
| `tests/unit/llm/{anthropic_adapter_test,deepseek_adapter_test,provider_factory_test}.rs` (3 files, 25 test fns) | Declared in `tests/unit/llm/mod.rs`, reached via `pub mod llm;` in `tests/unit/mod.rs` | **reachable — closed this phase.** `tests/unit/mod.rs` was missing `pub mod llm;` until plan **02-06** added it (D-10/D-11); all 25 functions compile and pass today |
| **`tests/integration/` (root, 36 files)** | Each declared by a `pub mod <name>;` line in `tests/integration/mod.rs`, which supplies the crate root for 8 separately-declared `[[test]]` targets (`system_log_integration`, `paladin_garrison_integration`, `citadel_integration`, `openai_embedding_integration`, `in_memory_sanctum_integration`, `qdrant_sanctum_integration`, `rag_integration`, `vision_integration`) plus, for every other module, the `tests/lib.rs`-rooted `lib` autodiscovered target (`tests/lib.rs`'s `pub mod helpers;` does not itself reach `tests/integration/`, but `tests/integration/mod.rs` is `pub mod`-included as one of the many autodiscovered-root modules compiled inside the same `lib` binary — confirmed by `lib`'s 685-test count, which includes the non-featured integration files) | reachable — all 36; 15 are `#[cfg(feature = "…")]`-gated (`llm-anthropic`, `cli` ×3, `llm-deepseek`, `s3-storage`, `live-api-tests`, `llm-openai` ×2, `openai-embeddings`, `qdrant` ×2, `redis-queue`, `vision`+`llm-openai`+`llm-anthropic`) and correctly compile out under default features rather than being orphaned |
| `tests/integration/battalion/{campaign_integration,chain_of_command_integration,council_integration,formation_integration,grove_integration,load_test,phalanx_integration}.rs` (7 files) | Declared in `tests/integration/battalion/mod.rs`, reached via `pub mod battalion;` in `tests/integration/mod.rs` | reachable — all 7 |
| `tests/integration/battalion_herald_end_to_end_test.rs` | `pub mod battalion_herald_end_to_end_test;` in `tests/integration/mod.rs` | reachable — added by plan **02-05** (D-06, closes Epic 8 task 7.13) |
| `tests/integration/provider_switching_test.rs` | `pub mod provider_switching_test;` in `tests/integration/mod.rs` | reachable — added by plan **02-06** (closes Epic 6 task 7.10) |
| **`tests/cli/` (root, 14 content files + `mod.rs`)** | `mod helpers;` plus five in-scope suites (`error_handling_test`, `formation_execution_test`, `paladin_execution_test`, `phalanx_execution_test`, `tool_integration_test`) and four pre-existing snapshot suites (`error_output_test`, `help_output_test`, `progress_output_test`, `table_output_test`) declared in `tests/cli/mod.rs`, itself the `[[test]] name = "cli"` target (`Cargo.toml:211-214`, `required-features = ["cli"]`) | reachable — 10 files, all under `--features cli` |
| `tests/cli/helpers.rs` | `mod helpers;` in `tests/cli/mod.rs`; internally a `#[path = "../helpers/mod.rs"] mod shared;` re-export shim | reachable — added by plan **02-07** |
| `tests/cli/{error_handling,formation_execution,paladin_execution,phalanx_execution,tool_integration}_test.rs` (5 files, 37 test fns) | Declared (uncommented) in `tests/cli/mod.rs` | **reachable — closed this phase.** These five suites — 1,895 lines of pre-existing, unmodified test source — sat commented out in `tests/cli/mod.rs` because the helper module they import (`tests/cli/helpers.rs`) did not exist. Plan **02-07** created the shim and uncommented them (D-09); all 37 tests passed with zero repairs needed, correcting the Phase 1 ledger's "the test itself was never written" finding for Epic 9 tasks 13.4-13.6 |
| `tests/cli/{arsenal_config_test,environment_tests,garrison_config_test,integration_tests}.rs` (4 files, 66 test fns: 10+45+8+3) | **`mod` lines present in `tests/cli/mod.rs` but commented out**, with a note pointing here | **never compiled — new finding, reported not fixed.** See "Findings requiring a decision" below |
| **`tests/functional/` (4 files)** | Each declared by a `#[path = "functional/<name>.rs"] mod <name>;` line in `tests/functional.rs`, which is itself a direct child of `tests/` and therefore an autodiscovered root target (`functional`, confirmed compiled: 10 tests) | reachable — all 4 |
| **`tests/helpers/` (3 files)** | Declared in `tests/helpers/mod.rs`; reached three ways — `tests/lib.rs`'s top-level `pub mod helpers;` (the autodiscovered `lib` target), `tests/cli/helpers.rs`'s `#[path]` shim (the `cli` target), and `tests/unit/mod.rs` does **not** re-declare it (the `unit` target has its own separate mock needs) | reachable |
| **`tests/repository/mysql_content_repository_test.rs`** | `#[path = "repository/mysql_content_repository_test.rs"] mod mysql_content_repository_test;` in `tests/repository.rs`, gated `#[cfg(feature = "storage-mysql")]`, itself an autodiscovered root target (`repository`, 0 tests under default features — confirmed) | reachable, conditionally (compiles under `--features storage-mysql`, never exercised by the default `cargo test --workspace` run) |
| **Other direct children of `tests/`** — `agent_orchestrator_bridge.rs`, `cli_isolation_test.rs` (also has an explicit `[[test]] name = "cli_isolation"` entry, `Cargo.toml:216-218`), `content_agent_bridge.rs`, `content_ingestion_pipeline.rs`, `event_trigger_pipeline.rs`, `lib.rs`, `paladin_server_smoke.rs`, `queue_port_contract.rs`, `web_server_e2e.rs` (9 files) | Autodiscovered — each is a direct child of `tests/` with no `[[test]]` override (except `cli_isolation_test.rs`, which has one; both the autodiscovered name and the declared name resolve to the same single compiled binary since Cargo does not double-compile a path already covered by an explicit target) | reachable — all 9; two (`content_ingestion_pipeline.rs`, 0 tests) compile to empty bodies under default features — this is feature-gating, not orphaning, confirmed by reading each file's own `#[cfg(feature = …)]` guards |

## Scope note — outside D-12's named `tests/` root

`crates/paladin-web/tests/auth_rbac.rs` (5 tests, compiled and passing — visible in the raw evidence
above as its own `Running tests/auth_rbac.rs` group) is a **per-crate** integration-test directory,
auto-discovered by Cargo per the ordinary single-crate rule and requiring no barrel or `[[test]]`
declaration of its own. D-12 and CONTEXT.md both scope the sweep to the root workspace `tests/`
directory tree (the one the root `Cargo.toml`'s `[[test]]` table and this ledger's Epic-level rows
address); per-crate `tests/` directories are a structurally different, self-contained mechanism with
no wiring failure mode to sweep for (a crate's own `tests/*.rs` files cannot silently fail to compile
into that crate's test run the way a root-`tests/`-subdirectory file can fail to be `pub mod`-declared
from a shared barrel). Noted for completeness; not swept further.

## `benches/`

| Path | Declared? | Status |
|---|---|---|
| `benches/config_benchmarks.rs` | `[[bench]] name = "config_benchmarks", harness = false` (root `Cargo.toml:254-256`) | declared |
| `crates/paladin-memory/benches/{sanctum,garrison}_benchmarks.rs` | `[[bench]]` entries in `crates/paladin-memory/Cargo.toml:44-50` | declared — both |
| `crates/paladin-battalion/benches/battalion_benchmarks.rs` | `[[bench]]` entry in `crates/paladin-battalion/Cargo.toml:34-36` | declared |
| `crates/paladin-llm/benches/llm_serialization_benchmarks.rs` | `[[bench]]` entry in `crates/paladin-llm/Cargo.toml:46-48` | declared |

All 5 bench files across the workspace are explicitly declared with matching `[[bench]]` entries in
their owning crate's `Cargo.toml` (root or per-crate). None is orphaned. This matches
`PROJECT.md`'s "benchmark migration" note (Milestone 7 Epic 3: all five suites relocated into their
owning crates, zero `.disabled` files remaining) — re-confirmed here by direct declaration cross-check
rather than by citation.

## `examples/`

**46 example files found; `.planning/ROADMAP.md`'s Phase 4 success criterion 5 states "all 22
examples".** `find examples -maxdepth 1 -name '*.rs' | wc -l` → 46. 4 carry explicit `[[example]]`
entries with `required-features` (`vision_analysis`, `vision_battalion` — both `["vision",
"llm-openai"]`; `document_processing` — `["content-processing"]`; `http_service_host` —
`["web-server"]`); the remaining 42 are plain autodiscovered examples with no feature gate (Cargo's
`autoexamples` default is `true`, and no `autoexamples` key appears anywhere in the root
`Cargo.toml`, confirmed by grep). `scripts/check-all-examples.sh` — the script Phase 4's own CI gate
would run — enumerates `examples/*.rs` dynamically (`for example in "$EXAMPLES_DIR"/*.rs`), not
against a hardcoded list or count, so it already covers all 46 today; **only the ROADMAP prose's
"22" figure is stale**, not the CI mechanism. This is a Phase 4 concern (its own success criterion
wording), recorded here per D-12's discretionary extension and left to Phase 4 to correct at source —
not amended by this plan, whose ROADMAP-editing scope (Task 3) is confined to the Phase 2 section.

## Already-closed instances (not new findings — recorded per the plan's own instruction)

1. **The LLM unit module**, `tests/unit/llm/` (3 files, 25 test functions) — closed by plan **02-06**:
   added the single missing `pub mod llm;` line to `tests/unit/mod.rs` (D-10/D-11), repaired mechanical
   construction-API drift, and got all 25 passing with zero deletions.
2. **The five CLI execution/error/tool suites**, `tests/cli/{error_handling_test,
   formation_execution_test,paladin_execution_test,phalanx_execution_test,tool_integration_test}.rs`
   (37 test functions) — closed by plan **02-07**: created `tests/cli/helpers.rs` as a path-attribute
   shim into `tests/helpers/`, uncommented the five `mod` lines in `tests/cli/mod.rs`, and all 37
   passed with zero repairs, closing Epic 9 tasks 13.4-13.6.

## The `LlmProviderError` dead-conversion-path finding (discretion item, D-12 theme)

**Verified against the current tree, 2026-08-01.** `crates/paladin-llm/src/error.rs:16` defines
`pub enum LlmProviderError` (nine variants), and `crates/paladin-llm/src/error.rs:54` implements
`impl From<LlmProviderError> for LlmError`. The type's own doc comment
(`crates/paladin-llm/src/error.rs:1-6`) states it exists so "errors propagate cleanly across the port
boundary" via that `From` impl. But:

```
$ grep -rn "LlmProviderError" crates/paladin-llm/src/ | grep -v "src/error.rs"
(no output — zero matches)
```

No adapter anywhere in `paladin-llm` ever constructs an `LlmProviderError`. The ledger's existing
`REQ-provider-error-mapping` row (`milestone-01.md:343`) already records the corollary fact — each
adapter (e.g. `crates/paladin-llm/src/deepseek/adapter.rs:343-350`) maps HTTP status codes directly
to `LlmError` variants at the call site, bypassing `LlmProviderError` and its conversion entirely.
The only other workspace hits for the literal string `LlmProviderError` are the CLI's own, unrelated
`CliError::LlmProviderError { message: String }` variant (`src/application/cli/error.rs:71` and six
call sites in `src/application/cli/commands/{agent.rs,battalion.rs}`) — a same-named but structurally
distinct enum variant on a different type, not a reference to `paladin_llm::error::LlmProviderError`.

**The documented conversion path is dead code.** The type and its `From` impl compile, and nothing
outside their own definition site ever exercises them. **Forward owner: Phase 3**, per CONTEXT.md's
own discretion note grouping this with D-12's dead-code theme; not claimed as QUAL-01/02/03/04/05
specifically, since none of those five requirements names dead-code removal as their own subject —
Phase 3 is named as the candidate destination for a future decision (delete `LlmProviderError` and
its `From` impl, or wire it in as the mapping path adapters actually use), not as a mandate.

## Findings requiring a decision

| Finding | What it is | Proposed forward owner |
|---|---|---|
| `tests/cli/{arsenal_config_test,environment_tests,garrison_config_test,integration_tests}.rs` — 66 test functions (10 + 45 + 8 + 3) across 1,806 lines, still commented out in `tests/cli/mod.rs` | Four pre-existing CLI test files testing arsenal/garrison YAML config instantiation, environment-variable/non-interactive-mode behaviour, and broader CLI-command integration scenarios. Explicitly out of Phase 2's D-09 scope (which named only Epic 9 tasks 13.4-13.6 and Epic 6 tasks 7.10-7.12 as in-scope), and plan 02-07's SUMMARY explicitly deferred reporting on them to this sweep. A spot-check of their imports (`instantiate_arsenal`, `instantiate_garrison`, `ArsenalConfig`, `McpServerConfig`, `GarrisonConfig`, `GarrisonTypeConfig`, `load_paladin_config`, `FeatureCategory`) confirms every referenced symbol still exists in the current tree at its expected path — unlike the LLM module before D-10, there is no obvious sign of mechanical API drift, so whether they compile and pass today is genuinely unknown until someone uncomments them and runs `cargo test --features cli --test cli`. | **Phase 3 (candidate, not claimed).** No existing GAP-*/QUAL-* requirement names this file set; Phase 3 already owns the shared test-completeness and mock-harness work (D-07's forward-owner note for the four `#[ignore]`d Commander tests uses the same "candidate, not claimed" language) and is the nearest natural continuation of CLI test-depth work. This record does not assign the work; it names where a future planner should look. |
| `LlmProviderError` dead conversion path | See dedicated section above | Phase 3 (candidate) |
| ROADMAP Phase 4 criterion 5's stale "22 examples" figure | 46 examples exist today; the CI script already covers all of them dynamically | Phase 4 (the phase whose own criterion this is; not amended here — Task 3's ROADMAP edit is scoped to the Phase 2 section only) |

No other new orphaned or never-compiled test source was found. Every file under `tests/unit/`,
`tests/integration/`, `tests/cli/` (beyond the four named above), `tests/functional/`,
`tests/helpers/`, every other direct child of `tests/`, and every file under `benches/` is declared
and reachable from a compiled target, confirmed against the raw `cargo test --workspace -- --list`
and `cargo test --features cli --test cli -- --list` evidence above. **No source file was modified
by this task** — `git diff --name-only` at the end of Task 1 lists nothing outside `.planning/`.
