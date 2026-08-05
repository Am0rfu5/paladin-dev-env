# ADR-0013: Grove routing model from configuration

## Status

Accepted

**Date:** 2026-08-05

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

## Code Conformance

conforms

**CLOSE-01** is the requirement this ADR records, and it is satisfied by shipped code: plan 06-01
landed `GroveConfig.routing_model`, the `GroveBuilder` setter, the hard-error guard, and the
config-sourced `LlmRequest.model` construction, all committed and covered by the tests cited
above. `cargo test --workspace`, `cargo fmt --check` and `cargo clippy --workspace --all-targets
-- -D warnings` all ran green at that plan's close (06-01-SUMMARY.md). No further code work is
outstanding against CLOSE-01.

## Downstream Consumers

- `CHANGELOG.md`'s `## [Unreleased]` entry and `.planning/PROJECT.md`'s `## Key Decisions` row —
  both written by this plan's Task 2, both required by D-03 alongside this ADR
- Phase 6 close-out plan 06-07 — marks CLOSE-01 satisfied in `.planning/REQUIREMENTS.md`, citing
  this ADR
- Any operator running a Grove with `RoutingStrategy::LlmRouting` — must set `routing_model` in
  their Grove configuration (YAML/JSON or `GroveBuilder::routing_model(..)`) or routing now
  returns `BattalionError::RoutingError` instead of silently defaulting to `gpt-4`
