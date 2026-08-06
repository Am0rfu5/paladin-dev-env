# ADR-0017: LLM configuration ownership and the bridge location

## Status

Accepted

**Date:** 2026-08-06

## Context

`.project/Milestone_5-Workspace-Decomposition/Epic_4/prd-paladin-llm-extraction.md` FR-31
(`:197`) forbids `paladin-llm` from importing `crate::config::application_settings` or any
equivalent root-crate path, on the stated ground that doing so "would create a circular
dependency." FR-32 (`:199`) assigns the conversion responsibility to the root `paladin` crate:
"the root `paladin` crate is solely responsible for reading `ApplicationSettings.llm.*` fields and
converting them into the appropriate `paladin-llm` `*Config` struct," naming
`src/infrastructure/adapters/llm/config_bridge.rs` as that conversion code's intended home —
explicitly **not** inside `paladin-llm`.

`.project/Milestone_6-Architectural-Refinements/Epic_1/prd-decompose-application-settings.md`
then decomposed `application_settings.rs` and moved the LLM and vision configuration types
themselves **down** into `paladin-llm`: its file-move table (`:74-75`) sends `LlmProviderConfig`,
`LlmConfig`, `VisionRetryConfig`, `VisionProviderConfig` and `VisionConfig` to
`crates/paladin-llm/src/config/llm.rs` and `crates/paladin-llm/src/config/vision.rs`, and its own
`## Context` (`:261`) names the consequence directly: "`src/infrastructure/adapters/llm/config_bridge.rs`
bridges `ApplicationSettings` LLM config types to the adapter's own config types. After moving
`LlmProviderConfig` and `VisionConfig` to `paladin-llm`, this bridge's imports must be updated."

The shipped bridge is `crates/paladin-llm/src/config/bridge.rs` — inside `paladin-llm`, the
location FR-31/FR-32 explicitly forbade. Its own doc comment (`:1-7`) states why that placement is
now correct rather than a violation: "Placing the impls here keeps all `paladin-llm` types
together and avoids the Rust orphan rule restriction (both source and target types are local to
this crate)." Both the source type (`LlmProviderConfig`, `crate::config::llm::LlmProviderConfig`)
and the target type (`OpenAIConfig`, `crate::openai::adapter::OpenAIConfig`, and its Anthropic /
DeepSeek / vision counterparts) now live inside `paladin-llm` itself.

## Decision

The shipped bridge location, `crates/paladin-llm/src/config/bridge.rs`, is accepted as the answer
to ARCH-03(d).

FR-31's circular-dependency concern was real at the time it was written and does not describe the
shipped tree. FR-31 assumed the configuration types (`LlmProviderConfig`, `VisionConfig`) would
stay in the root `paladin` crate, in which case a bridge living inside `paladin-llm` would indeed
require `paladin-llm` to import from the root crate — the cycle FR-31 forbids. Milestone 6 Epic 1
did not relocate the bridge to satisfy that constraint; it relocated the *configuration types* the
bridge converts between, moving them down into `paladin-llm` alongside the adapter-specific
`*Config` structs FR-32 already placed there. With both the `From` impl's source type and target
type local to `paladin-llm`, the bridge has no reason to reach up into the root crate at all, and
the cycle FR-31 feared has no path to exist.

Milestone 6 did not break the boundary Epic 4 established — it removed the need for it. Epic 4's
FR-31/FR-32 correctly separated "configuration ownership" from "adapter configuration" at the time
they were written, when the root crate owned `ApplicationSettings` in full; Milestone 6's
decomposition changed which crate owns the configuration types the bridge sits between, and the
bridge's correct location moved with them. The concern was sound; the resolution was structural,
not a reversal of the concern.

## Considered Options

- Accept the shipped location and record FR-31/FR-32's concern as real but mis-sited — accepted.
  This matches what actually happened: the types moved, not the constraint, and the shipped
  bridge's own doc comment states the orphan-rule reasoning that makes co-location correct now.
- Relocate the bridge to `src/infrastructure/adapters/llm/config_bridge.rs` to satisfy FR-31/FR-32
  as literally written — rejected. This phase does not change product code (see this plan's
  prohibitions), and relocating the bridge would fight the Rust orphan rule the shipped doc comment
  already identifies — `From<&LlmProviderConfig> for OpenAIConfig` cannot be implemented outside
  `paladin-llm` once both types live inside it, without a newtype wrapper neither FR-31 nor FR-32
  ever asked for. There is also no live cycle to guard against; relocating would be a change
  against a risk that no longer exists.
- Record Epic 4 as simply mistaken — rejected. FR-31's concern was correct given Milestone 5's
  configuration ownership at the time it was written; Milestone 6 changed the ownership the
  concern was reasoning about. Recording it as a mistake would misstate a sound piece of reasoning
  that a later, unrelated decision (the Epic 1 config decomposition) happened to make moot.

## Code Locations

- `crates/paladin-llm/src/config/bridge.rs:1-7` — the shipped bridge's doc comment, stating the
  orphan-rule reasoning for co-locating the `From` impls inside `paladin-llm`
- `crates/paladin-llm/src/config/bridge.rs:9-19` — the `LlmProviderConfig`/`OpenAIConfig`/
  `AnthropicConfig`/`DeepSeekConfig` imports, all local to `paladin-llm`
- `crates/paladin-llm/src/config/llm.rs` and `crates/paladin-llm/src/config/vision.rs` — the
  configuration modules Milestone 6 Epic 1 moved down into `paladin-llm`
- `.project/Milestone_5-Workspace-Decomposition/Epic_4/prd-paladin-llm-extraction.md:197` — FR-31,
  the circular-dependency prohibition
- `.project/Milestone_5-Workspace-Decomposition/Epic_4/prd-paladin-llm-extraction.md:199` — FR-32,
  naming `src/infrastructure/adapters/llm/config_bridge.rs` as the bridge's intended root-crate
  home
- `.project/Milestone_6-Architectural-Refinements/Epic_1/prd-decompose-application-settings.md:74-75`
  — the file-move table sending `LlmProviderConfig`/`LlmConfig`/`VisionConfig` and friends into
  `crates/paladin-llm/src/config/`
- `.project/Milestone_6-Architectural-Refinements/Epic_1/prd-decompose-application-settings.md:261`
  — the clause naming the bridge's import update as the direct consequence of the config-type move

## Code Conformance

conforms

`crates/paladin-llm/src/config/bridge.rs` already implements this decision. No code change is
made by this ADR or by this phase.

## Downstream Consumers

- Plan 07-08 and plan 07-10 — the ledger rows for `REQ-llm-config-bridge-location-v1` and
  `REQ-llm-config-bridge-location-v2` cite this ADR as the recorded answer to ARCH-03(d).
- Plan 07-13 — adds this ADR's row to `.planning/decisions/PROMOTION.md`'s numbering index and
  advances the "Next free ADR number" line past 0017.
