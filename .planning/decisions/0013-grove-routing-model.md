# ADR-0013: Grove routing model from configuration

## Status

Accepted

**Date:** 2026-08-05

**Amended (Phase 6, 2026-08-05):** `06-VERIFICATION.md` found D-02's guard correct in isolation —
`route_by_llm`'s missing-`routing_model` check genuinely returned `BattalionError::RoutingError`
with no fallback consulted — but **unreachable from `GroveExecutionService::execute()`**, the only
public entry point a real operator uses: `route_task`'s pre-existing blanket `match result { Err(e)
=> .. }` arm caught the guard's error identically to any transient routing failure and silently
substituted `fallback_tree` or the first agent in the first tree, so no caller of `execute()` ever
observed the D-02 guarantee. This made this ADR's original `## Downstream Consumers` operator bullet
untrue for every real caller — an operator running a misconfigured Grove saw a silent fallback, not
the documented error. Plan 06-08 closed the gap by resolving `routing_model` via a shared
`GroveExecutionService::resolve_routing_model` function called **before** strategy dispatch inside
`route_task`, so the configuration error propagates through `?` and out of `execute()` before it can
ever reach the fallback arm. The recorded decision itself is unchanged: D-02 ships exactly as the
human user approved it at plan 06-01's `checkpoint:decision` on 2026-08-05 (`proceed-as-locked`);
see `## Code Locations` and `## Code Conformance` below for the shipped mechanism and its
exercisers.

## Context

Phase 5 verified the one defect in run-2 scope still open against the tree:
`crates/paladin-battalion/src/grove_service.rs:537` built its routing `LlmRequest` with a
hardcoded provider model literal in production code —

```rust
model: "gpt-4".to_string(), // TODO: Make configurable
```

— with the file's `#[cfg(test)]` module beginning well below it (at line 732, pre-06-01), so a
Grove configured for Anthropic or DeepSeek silently routed its LLM-based selection through OpenAI
regardless of what the operator had configured elsewhere. This was the **only** defect in run-2
scope verified open against the tree (`REQUIREMENTS.md` CLOSE-01), and it is the same defect
class Epic 21 already removed from `planning_service.rs` and `prompt_generation_service.rs`. It
also meant Epic 22's own completion criterion — quoted verbatim from `REQUIREMENTS.md`'s CLOSE-01
entry — **"all inline TODOs in Battalion and Commander files resolved"** — was not met.

Two sibling occurrences of the same string exist in the crate, both re-derived against the tree
for this ADR rather than transcribed from CONTEXT.md: `council_service.rs:816`
(`model: "gpt-4".to_string()`) and `conclave_execution_service.rs:600`
(`model: "gpt-4o".to_string()`). Both sit inside `#[cfg(test)]` blocks that begin at line 521 and
line 512 of their respective files — confirmed by direct grep, not assumed — so both are test
fixtures, not production defects, and are out of scope for CLOSE-01. `grove_service.rs:537` is the
only production occurrence in the crate.

## Decision

Two halves.

**First — the config surface (D-01).** The routing model comes from a new
`routing_model: Option<String>` field on `GroveConfig`
(`crates/paladin-core/src/platform/container/battalion/grove.rs:221`), threaded through
`GroveBuilder`. It sits alongside the five knobs already there (`routing_strategy`,
`fallback_tree`, `similarity_threshold`, `routing_fallback`, `min_confidence`), so it is
config-shaped like every other Grove setting and additive to the YAML/JSON surface (`GroveConfig`
is `Serialize + Deserialize`), using `#[serde(skip_serializing_if = "Option::is_none")]` to match
the existing `fallback_tree` convention.
— **Reversibility: costly** — `GroveConfig` is a public, serialized type; removing the field later
breaks every YAML config that sets it and every builder call that passes it.

**Second — the hard-error guard (D-02).** When `routing_model` is `None` (or blank/whitespace-only
after trimming) and `routing_strategy` is `RoutingStrategy::LlmRouting`, routing returns
`BattalionError::RoutingError` naming the missing configuration, with **no fallback of any kind**:
`routing_fallback` is not consulted, `get_available_models()` is not called, and no per-provider
default is guessed. State the deliberate consequence plainly: **this is a runtime behaviour break
for any Grove already using LLM routing.** A Grove using `RoutingStrategy::LlmRouting` today works
(silently, against `gpt-4`) and starts returning `BattalionError::RoutingError` after this change
until its configuration names a model. It is scoped by the fact that `RoutingStrategy::default()`
is `KeywordMatch`, so a default-constructed Grove is unaffected — only Groves that explicitly
select LLM routing are affected.
— **Reversibility: one-way** — this is a deliberate runtime behaviour break, already shipped by
the time it could be softened.

**Checkpoint outcome.** This one-way break was presented to the human user as a
`checkpoint:decision` reversibility gate before any code was written (plan 06-01, Task 1). The
user selected **`proceed-as-locked`** on 2026-08-05: D-02 ships as locked, a missing
`routing_model` under `RoutingStrategy::LlmRouting` is a hard error with no fallback of any kind.
That approval is recorded here as a fact, not re-opened.

## Considered Options

- A `String` field with a `"gpt-4"` serde default, rather than `Option<String>` with a hard-error
  guard — rejected. This moves the vendor literal into the config default rather than eliminating
  it, leaving CLOSE-01's own criterion — the hardcoded model gone, with a test proving a
  non-OpenAI model reaches the LLM call — half-met: the literal would still silently govern every
  Grove that never sets the field.
- A constructor argument on `GroveExecutionService::new`, rather than a `GroveConfig` field —
  rejected. Breaking signature, and it puts a routing knob where no other Grove knob lives,
  invisible to YAML/JSON configuration entirely.
- Resolving the model from `llm_port.get_available_models()` when `routing_model` is absent,
  rather than hard-erroring — rejected. Async, fallible, and returns providers in arbitrary
  ordering, so the "resolved" model would be non-deterministic and would require a network
  round-trip on every misconfigured routing call.
- A per-provider default-model table inside `paladin-battalion`, rather than requiring explicit
  configuration — rejected. Goes stale as providers rename or retire models, and it puts provider
  knowledge in the orchestration crate, which owns no provider adapters.

## Code Locations

- `crates/paladin-core/src/platform/container/battalion/grove.rs:221` — `GroveConfig.routing_model:
  Option<String>` field, with rustdoc stating it is required under `RoutingStrategy::LlmRouting`
  and that routing errors when absent
- `crates/paladin-core/src/platform/container/battalion/grove.rs:254-264` — `impl Default for
  GroveConfig`, initialising `routing_model: None`
- `crates/paladin-core/src/platform/container/battalion/grove.rs:306-410` — `GroveBuilder`'s
  `routing_model` field (`:310`) and fluent `routing_model(..)` setter (`:406-409`)
- `crates/paladin-battalion/src/grove_service.rs:493-510` — the missing-model guard in
  `route_by_llm`: trims and checks `grove.node.config.routing_model`, returning
  `BattalionError::RoutingError` naming `routing_model` when absent or blank, consulting no
  fallback of any kind
- `crates/paladin-battalion/src/grove_service.rs:554-561` — the config-sourced `LlmRequest`
  construction, `model: routing_model.to_string()`, replacing the former `"gpt-4"` literal
- `crates/paladin-battalion/src/grove_service.rs#test_llm_routing_uses_configured_routing_model`
  (`:1663`) — proves a configured non-OpenAI model (`deepseek-chat`) reaches `LlmPort::generate`
  via a `RecordingLlmMock`, clearing this project's D-19 evidence bar (a citation plus a named
  passing test)
- `crates/paladin-battalion/src/grove_service.rs#test_llm_routing_errors_when_routing_model_absent`
  (`:1692`) and `#test_llm_routing_errors_when_routing_model_empty` (`:1732`) — prove the hard-error
  path for the `None` and blank/whitespace cases
- `crates/paladin-battalion/src/grove_service.rs#test_llm_routing_missing_model_error_precedes_keyword_fallback`
  (`:~1780`) — proves the missing-`routing_model` error fires before any keyword-fallback attempt,
  i.e. no fallback of any kind
- `crates/paladin-battalion/src/grove_service.rs#test_concurrent_groves_use_their_own_routing_model`
  (`:1818`) — proves two concurrently-executing Groves with different `routing_model` values each
  cause their own configured model to reach the LLM call

**Added by the Phase 6 amendment above — the `execute()`-reachable half of D-02:**

- `crates/paladin-battalion/src/grove_service.rs:64` — `MISSING_ROUTING_MODEL_ERROR`, the single
  error-message constant shared by both call sites below
- `crates/paladin-battalion/src/grove_service.rs:252-260` — `GroveExecutionService::resolve_routing_model`,
  the single definition of the D-02 configuration check, called from both `route_task`'s
  pre-dispatch resolution and `route_by_llm`'s in-strategy guard, so the two checks cannot drift
  apart
- `crates/paladin-battalion/src/grove_service.rs:301-303` — `route_task`'s pre-dispatch resolution
  (`if matches!(strategy, RoutingStrategy::LlmRouting) { Self::resolve_routing_model(grove)?; }`),
  sitting **above** the `match result { .. Err(e) => .. }` fallback arm that begins at `:306`, so
  the `?` operator propagates the configuration error out of `GroveExecutionService::execute()`
  before that fallback arm can ever intercept it
- `tests/integration/battalion/grove_integration_test.rs#test_grove_llm_routing_errors_when_routing_model_absent_through_execute`
  (`:389`) — the primary `execute()`-level exerciser: builds an `LlmRouting` Grove with a
  **configured** `llm_port` and no `routing_model`, drives it through
  `GroveExecutionService::execute()` (not `route_by_llm` directly), asserts
  `Err(BattalionError::RoutingError(..))` naming `routing_model`, and asserts zero LLM calls were
  recorded — clearing this project's D-19 evidence bar (a `file:line` citation plus a named
  passing exerciser) for the no-fallback half of D-02
- `tests/integration/battalion/grove_integration_test.rs#test_grove_llm_routing` (`:235`) — the
  standing counter-example `06-VERIFICATION.md` cited by name (it previously asserted
  `result.is_ok()` for this exact misconfiguration through `execute()`); inverted by plan 06-08 to
  assert the D-02 error instead, so the regression this ADR corrects cannot silently return
- `crates/paladin-battalion/src/grove_service.rs#test_execute_errors_when_routing_model_absent`
  (`:1919`), `#test_execute_errors_when_routing_model_blank` (`:1957`), and
  `#test_execute_errors_despite_fallback_tree_when_routing_model_absent` (`:1999`) — three further
  `execute()`-level exercisers proving, respectively, the plain `None` case, blank/whitespace-only
  `routing_model`, and that a **configured and resolvable** `fallback_tree` is still declined for
  this specific configuration error
- `tests/integration/battalion/grove_integration_test.rs#test_grove_llm_routing_falls_back_when_llm_port_absent_but_routing_model_set`
  (`:457`) — the scope-boundary control: the hard error above covers the missing/blank
  `routing_model` configuration case only. An absent `llm_port` under `LlmRouting` with
  `routing_model` set is a *different* failure mode and deliberately keeps its pre-existing
  fallback behaviour through `GroveExecutionService::execute()` — this test proves that fallback
  still succeeds, so no reader infers a broader break than shipped

## Code Conformance

conforms

**CLOSE-01** is the requirement this ADR records, and it is satisfied by shipped code: plan 06-01
landed `GroveConfig.routing_model`, the `GroveBuilder` setter, the hard-error guard, and the
config-sourced `LlmRequest.model` construction, all committed and covered by the tests cited
above. `cargo test --workspace`, `cargo fmt --check` and `cargo clippy --workspace --all-targets
-- -D warnings` all ran green at that plan's close (06-01-SUMMARY.md). No further code work is
outstanding against CLOSE-01.

**Correction (Phase 6, 2026-08-05):** the paragraph above was wrong when it was written. "No
further code work is outstanding against CLOSE-01" overstated what plan 06-01 shipped:
`route_by_llm`'s guard was correct in isolation, but `route_task`'s pre-existing blanket fallback
arm made the guard's no-fallback guarantee unreachable from `GroveExecutionService::execute()`, the
only public entry point — `06-VERIFICATION.md` truth 3 found this and scored CLOSE-01 half-closed.
Plan 06-08 added the missing code work: the pre-dispatch `resolve_routing_model` call in
`route_task` cited above, plus the five `execute()`-level tests proving it. `cargo test
--workspace`, `cargo fmt --check`, and `cargo clippy --workspace --all-targets -- -D warnings` all
ran green again at plan 06-08's close, HEAD `d83cd36` (06-08-SUMMARY.md). The `conforms` verdict
above now holds at the level a real caller of `execute()` observes it; it did not hold at that
level when plan 06-01 first wrote it.

## Downstream Consumers

- `CHANGELOG.md`'s `## [Unreleased]` entry and `.planning/PROJECT.md`'s `## Key Decisions` row —
  both written by this plan's Task 2, both required by D-03 alongside this ADR
- Phase 6 close-out plan 06-07 — marks CLOSE-01 satisfied in `.planning/REQUIREMENTS.md`, citing
  this ADR
- Any operator running a Grove with `RoutingStrategy::LlmRouting` — must set `routing_model` in
  their Grove configuration (YAML/JSON or `GroveBuilder::routing_model(..)`) or
  `GroveExecutionService::execute()` — the entry point the error is actually returned from — now
  returns `BattalionError::RoutingError` instead of silently defaulting to `gpt-4`. **Scope
  boundary:** this applies only to the missing/blank `routing_model` configuration case; every
  other Grove routing failure (a transient LLM call failure, unparseable JSON, a below-threshold
  confidence, or an absent `llm_port`) keeps its existing `fallback_tree`/default-agent fallback
  behaviour unchanged
