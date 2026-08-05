---
phase: 06-verified-gap-closure
reviewed: 2026-08-05T20:05:15Z
depth: standard
files_reviewed: 19
files_reviewed_list:
  - crates/paladin-battalion/benches/battalion_benchmarks.rs
  - crates/paladin-battalion/src/campaign_service.rs
  - crates/paladin-battalion/src/chain_of_command_service.rs
  - crates/paladin-battalion/src/commander.rs
  - crates/paladin-battalion/src/grove_service.rs
  - crates/paladin-core/src/platform/container/battalion/grove.rs
  - crates/paladin-ports/src/output/vision_llm_port.rs
  - crates/paladin-ports/src/output/vision_port.rs
  - docs/src/appendix/performance-baseline.md
  - examples/commander_grove.rs
  - examples/grove_routing.rs
  - src/application/cli/commands/agent.rs
  - src/application/cli/config/paladin_config.rs
  - src/application/cli/templates/paladin_template.rs
  - src/infrastructure/security/encryption.rs
  - tests/integration/battalion/grove_integration_test.rs
  - tests/integration/battalion_chain_of_command_herald_test.rs
  - tests/integration/llm_live_api_tests.rs
  - tests/integration/mod.rs
findings:
  critical: 2
  warning: 2
  info: 3
  total: 7
status: issues_found
---

# Phase 06: Code Review Report

**Reviewed:** 2026-08-05T20:05:15Z
**Depth:** standard
**Files Reviewed:** 19
**Status:** issues_found

## Summary

The bulk of this phase's changes — the `routing_model` guard inside `route_by_llm`, the
Herald `with_herald`/`format_result`/`to_battalion_result` triad on
`ChainOfCommandExecutionService`/`CampaignExecutionService`/`Commander`, the additive-only
autonomous-flag override logic in `agent.rs`, and the doc-only edits to `vision_port.rs`,
`vision_llm_port.rs`, `encryption.rs`, `llm_live_api_tests.rs`, and `tests/integration/mod.rs`
— are implemented correctly and match their stated intent. The `routing_model` guard itself
(`grove_service.rs:493-510`) is correct in isolation: it rejects `None` and
empty/whitespace-only values before touching `routing_fallback` or the LLM port, exactly as
specified.

However, that guard's effect is nullified one layer up. `GroveExecutionService::execute()` —
the only public entry point external callers use — routes every strategy through `route_task()`,
which catches *any* `Err` from the selected strategy (including the deliberate,
no-fallback-by-design `RoutingError` for an unconfigured `routing_model`) and silently
substitutes the Grove's `fallback_tree` or first tree instead of propagating the error. This is
not a hypothetical: an existing, currently-green integration test
(`grove_integration_test.rs::test_grove_llm_routing`) builds a Grove with `LlmRouting` and no
`routing_model` set, calls `execute()`, and asserts `Ok(_)` — with a comment acknowledging "will
use keyword fallback since we don't have real LLM." That is the exact behavior the phase intent
says must not happen. See CR-01.

A second, unrelated defect was found in the CLI: `PaladinYamlConfig`'s `vision_enabled` /
`images` / `documents` YAML fields are validated (file existence, format) but never read by
`handle_agent_run` — only the separate `--image`/`--document` CLI flags actually drive vision
processing. A config that passes validation with `vision_enabled: true` silently runs without
vision. See CR-02.

Two further, lower-severity issues are noted below (a conditional-edge / fan-in aggregation gap
in `campaign_service.rs`, and a couple of documentation/naming drift items).

## Critical Issues

### CR-01: Grove's `execute()` silently swallows the deliberate no-fallback `routing_model` guard

**File:** `crates/paladin-battalion/src/grove_service.rs:171-175` (entry point), `240-290`
(`route_task`'s catch-all fallback), `493-510` (the correctly-implemented guard inside
`route_by_llm`)

**Issue:** The phase intent is explicit: when `RoutingStrategy::LlmRouting` is selected and
`GroveConfig.routing_model` is absent or blank, routing must return
`BattalionError::RoutingError` with **no fallback of any kind** — no keyword fallback, no
`routing_fallback` consultation, no querying the LLM port. `route_by_llm` implements this
correctly (`grove_service.rs:499-510`): it returns `Err` before ever calling
`llm_port.generate(...)`.

The problem is one call frame up. `GroveExecutionService::execute()` (the only method external
callers use — `grove_service.rs:171`) calls `self.route_task(grove, task).await?` at line 175.
`route_task` (`grove_service.rs:240-290`) wraps the strategy dispatch in a blanket
`match result { Ok(decision) => Ok(decision), Err(e) => { /* try fallback_tree, else first
tree */ } }`. This catch has no knowledge of *why* the strategy failed — it treats the
deliberate, no-fallback `RoutingError` from the missing-`routing_model` guard identically to a
transient/expected routing miss, and falls back to `fallback_tree` (if configured) or the
Grove's first tree (line 277, `grove.node.trees.first()`) unconditionally. Because
`GroveBuilder::build()` requires at least one non-empty tree, `grove.node.trees.first()` is
*always* `Some` for any successfully-built Grove — so this fallback path can never itself fail,
meaning `execute()` can never surface the "no `routing_model` configured" error to a caller who
only goes through the public `execute()` API. The guard is only observable to code that calls
the crate-private `route_by_llm` directly (which is exactly what every unit test in this file
that exercises the guard does — see `test_llm_routing_errors_when_routing_model_absent` etc. —
none of them call `execute()`).

This is independently confirmed by an existing, currently-passing integration test that this
same phase's file list includes:

```rust
// tests/integration/battalion/grove_integration_test.rs:234-274
let grove = GroveBuilder::new()
    .name("LlmRoutingGrove")
    .add_tree(tree1)
    .add_tree(tree2)
    .routing_strategy(RoutingStrategy::LlmRouting)
    .build()
    .expect("Grove build should succeed");
    // no .routing_model(...) call -- routing_model is None
...
let service = GroveExecutionService::new(paladin_port, None, Some... /* actually None */, Arc::new(registry));
// Execute task - will use keyword fallback since we don't have real LLM
let result = service.execute(&grove, "Fix the login bug").await;
assert!(result.is_ok(), "Execution should succeed");
```

`test_grove_llm_routing` builds a Grove with `LlmRouting` and no `routing_model`, calls the
public `execute()`, and asserts success — the exact "silently substituting a model the operator
did not choose" behavior D-01/D-02 says must not happen, reproduced by a green test.

**Fix:** Distinguish "no-fallback-by-design" routing errors from ordinary routing misses so
`route_task`'s catch-all cannot re-absorb them, e.g. a dedicated error variant/marker that
`route_task` checks before attempting `fallback_tree`/first-tree substitution:

```rust
// in route_by_llm's guard:
.ok_or_else(|| BattalionError::RoutingConfigurationError(
    "routing_model not configured for LLM-based routing".to_string(),
))?;

// in route_task:
match result {
    Ok(decision) => Ok(decision),
    Err(BattalionError::RoutingConfigurationError(msg)) => {
        // Configuration errors are not eligible for any fallback -- propagate as-is.
        Err(BattalionError::RoutingConfigurationError(msg))
    }
    Err(e) => { /* existing fallback_tree / first-tree logic */ }
}
```
and update `test_grove_llm_routing` (and any other test asserting `Ok` for an unconfigured
`routing_model` under `LlmRouting`) to assert the error instead.

### CR-02: YAML `vision_enabled`/`images`/`documents` are validated but never applied

**File:** `src/application/cli/config/paladin_config.rs:113-123` (fields), `331-395`
(`validate()` requiring file existence/format); `src/application/cli/commands/agent.rs`
(`handle_agent_run`, entire function — no reference to `config.vision_enabled`, `config.images`,
or `config.documents` anywhere in the file)

**Issue:** `PaladinYamlConfig` documents (module doc, top of `paladin_config.rs`) and validates a
`vision_enabled: bool` plus `images: Vec<String>` / `documents: Vec<String>` YAML schema:
`validate()` requires at least one image/document when `vision_enabled` is `true`
(`paladin_config.rs:332-340`), checks each image/document path exists
(`paladin_config.rs:344-368`), and checks supported extensions. A user who writes a config with
`vision_enabled: true` and `images: ["diagram.png"]` gets a config that passes `validate()`
cleanly.

`handle_agent_run` in `agent.rs`, however, never reads `config.vision_enabled`,
`config.images`, or `config.documents` at any point — confirmed by exhaustive grep across
`src/`. The *only* code path that calls `builder.enable_vision(true)` is gated on
`!args.images.is_empty()` (`agent.rs:364-365`), where `args.images` is the separate `--image`
CLI flag (`agent.rs:73-75`), not the YAML config. Similarly, document ingestion is gated on
`args.document` (the `--document` CLI flag), never `config.documents`. The result: a Paladin
configured for vision entirely through its YAML file — the documented, validated way to do it —
silently executes with vision disabled and never loads the configured images, with no error,
warning, or any other signal to the operator that their `vision_enabled`/`images` settings were
ignored.

**Fix:** In `handle_agent_run`, seed `args.images`/`args.document`/vision-enablement from
`config.vision_enabled` / `config.images` / `config.documents` when the corresponding CLI flags
are absent (mirroring the "YAML baseline, CLI flag as additive override" pattern this same phase
already established for the `autonomous` section in `apply_autonomous_config`), e.g.:

```rust
let vision_enabled_effective = config.vision_enabled || !args.images.is_empty();
let images_effective: Vec<PathBuf> = if !args.images.is_empty() {
    args.images.clone()
} else {
    config.images.iter().map(PathBuf::from).collect()
};
// ... use vision_enabled_effective / images_effective in place of args.images below
```
and thread `config.documents` similarly for the document path.

## Warnings

### WR-01: Conditional fan-in aggregation ignores per-edge condition results

**File:** `crates/paladin-battalion/src/campaign_service.rs:318-321` (readiness check),
`342-375` (`aggregate_inputs_for_node`), `401-421` (`are_dependencies_satisfied`)

**Issue:** `are_dependencies_satisfied` (line 402) only checks that every incoming edge's
*source node has executed* (`executed_nodes.contains(&source_id)`) — it does not check whether
that specific edge's `EdgeCondition` evaluated `true`. Likewise, `aggregate_inputs_for_node`
(line 356-364) pulls an input from every incoming edge whose source appears in `node_outputs`
(i.e. the source ran at all), again without checking whether that edge's own condition was
satisfied.

Concretely: a node `D` with two incoming edges, `B -> D` (condition true) and `C -> D`
(condition false), becomes "ready" via the `B -> D` traversal at line 319 as soon as `C` has
also executed *for any reason* (e.g. via an unrelated edge to a different node), because
`are_dependencies_satisfied` only checks that `C` ran, not that `C -> D`'s condition held. When
`D` then executes, `aggregate_inputs_for_node` concatenates `C`'s output into `D`'s input
(`campaign_service.rs:373`, the fan-in `join`) even though the `C -> D` edge condition was
`false` and that edge should never have been "traversed." This silently breaks the combination
of two features this module's own doc comment (lines 10-14) advertises together — Conditional
Routing and Fan-Out/Fan-In — whenever a node has more than one conditional incoming edge.

**Fix:** Track *per-edge* satisfaction (e.g. a `HashSet<(source_id, target_id)>` or a
`HashMap<Uuid, HashSet<EdgeIndex>>` of edges whose condition evaluated `true`) rather than only
per-node execution state, and use that set both in `are_dependencies_satisfied` (require every
incoming edge to be individually satisfied, not merely its source executed) and in
`aggregate_inputs_for_node` (only fold in outputs from edges present in that set).

### WR-02: `execute_agent`'s doc comment describes parameters that no longer exist

**File:** `crates/paladin-battalion/src/grove_service.rs:712-724`

**Issue:** The doc comment reads:

```
/// * `agent_id` - ID of the agent to execute (matches index in paladins vec, e.g., "agent_0")
/// * `paladins` - Slice of available Paladins
/// * `task` - Task input string
```

but the actual signature (line 725-729) is
`async fn execute_agent(&self, paladin: &Paladin, task: &str) -> Result<String, BattalionError>`
— there is no `agent_id` parameter and no `paladins` slice; the function takes an
already-resolved `&Paladin`. This is stale documentation left over from an earlier signature
(the registry-based resolution now happens in `execute()` before `execute_agent` is called) and
will mislead anyone reading the rustdoc.

**Fix:** Update the doc comment to match the current signature, e.g. replace the `agent_id`/
`paladins` bullets with a single `* \`paladin\` - The already-resolved Paladin to execute`.

## Info

### IN-01: Herald-formatting error mapped to a validation-flavored error variant in `Commander`

**File:** `crates/paladin-battalion/src/commander.rs:289-302`

**Issue:** `Commander::format_result` maps a Herald formatting failure to
`BattalionError::CommanderValidation(format!("Herald formatting error: {}", e))`. The sibling
services use error variants that actually name the failing operation
(`ChainOfCommandExecutionService::format_result` → `BattalionError::ChainOfCommandError`,
`CampaignExecutionService::format_result` → `BattalionError::CampaignError`). `CommanderValidation`
elsewhere in this same file is used exclusively for builder/config validation failures (empty
Paladins, missing strategy, bad timeout, etc.), so a caller matching on
`BattalionError::CommanderValidation` to detect a bad `CommanderBuilder` configuration will also
incorrectly catch a runtime Herald-formatting failure with unrelated semantics.

**Fix:** Introduce (or reuse an existing) `BattalionError::CommanderError` (or similarly-named)
variant for this call site, consistent with the naming convention the sibling services use.

### IN-02: Example binaries construct Grove routing configurations but never execute the router

**File:** `examples/commander_grove.rs` (all three `_grove1`/`_grove2`/`_grove3` bindings),
`examples/grove_routing.rs` (the `grove` binding)

**Issue:** Both examples build fully valid `Grove` instances via `GroveBuilder` and then print
hand-authored "Expected Routing" narrative text for each example task rather than actually
constructing a `GroveExecutionService` and calling `execute()`. The built `Grove` values are
bound to `_grove1`/`_grove2`/`_grove3` (underscore-prefixed, confirming they are intentionally
unused beyond construction) or `grove` (used only for field introspection at the end of
`grove_routing.rs`, e.g. printing `grove.node.trees.len()`). This is clearly labeled as
"Expected" rather than actual output, so it is not misleading in a strict sense, but it means
neither example actually demonstrates or exercises the routing algorithms it claims to showcase,
and the printed numbers (confidence percentages, similarity scores) are invented for narrative
purposes rather than produced by the code.

**Fix:** Optional improvement — wire a `GroveExecutionService` (with a mock/no-op
`PaladinPort`) into at least one of these examples so the printed routing decisions come from
`RoutingDecision` values the router actually produced, rather than from hardcoded strings.

### IN-03: `performance-baseline.md`'s new-run `sample.json` paths use an underscore-joined
convention that does not match criterion's actual on-disk directory structure

**File:** `docs/src/appendix/performance-baseline.md:155-157` (and, for context, the
pre-existing `848-849` lines from the earlier run using the same convention)

**Issue:** The three new `battalion/chain_of_command_*` benchmark IDs
(`c.bench_function("battalion/chain_of_command_2_levels_3_subordinates", ...)` in
`battalion_benchmarks.rs:175`) contain a `/`, which criterion treats as a directory separator
when writing `target/criterion/<id>/new/sample.json`. The document's jq-filter output lines
instead show `target/criterion/battalion_chain_of_command_2_levels_3_subordinates/new/sample.json`
(underscore-joined, no nested `battalion/` directory). This is consistent with the same
convention already used in the 2026-08-02 run earlier in this same document (e.g.
`target/criterion/battalion_formation_3_agents/new/sample.json` for a bench id of
`battalion/formation_3_agents`), so it is not a new inconsistency introduced by this phase, but
it means the paths as written cannot be copy-pasted to actually locate the files on disk.

**Fix:** None required for this phase (pre-existing documentation convention); if this document
is ever revised more broadly, consider correcting the path convention to match criterion's real
nested-directory output.

---

_Reviewed: 2026-08-05T20:05:15Z_
_Reviewer: Claude (gsd-code-reviewer)_
_Depth: standard_
