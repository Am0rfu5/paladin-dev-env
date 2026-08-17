# ADR-0046: Facade LLM feature-flag wiring, with the default build preserved

## Status

Accepted

**Date:** 2026-08-17

**Provenance:** D-11 (`17-CONTEXT.md`) was first recorded during the 2026-08-16
`/gsd-discuss-phase 17` session, accepting a `BREAKING` default-build narrowing (option-a).
**Amended 2026-08-17** during the `/gsd-plan-phase 17` session: the human rejected that
consequence in response to a direct challenge to whether the shipped OpenAI, Anthropic and
DeepSeek adapters keep functioning under this phase, and selected **option-b** — wire the flags
for real, but widen `default` so the compiled provider set does not change — via the runtime's
interactive question mechanism (`AskUserQuestion` → *"Lock option-b now"*). Recorded by the
plan-phase orchestrator, not by a subagent. Task 1 of plan 17-06 carried the resolved decision
into execution without reopening it.

## Context

Root `Cargo.toml` declared three empty stub flags —

```toml
llm-openai = []
llm-anthropic = []
llm-deepseek = []
llm-all = ["llm-openai", "llm-anthropic", "llm-deepseek"]
```

— while its `paladin-llm` dependency line (`Cargo.toml:55`, pre-this-plan) pulled the crate in
unconditionally with `features = ["openai", "anthropic", "deepseek", "mock", "vision"]`. Every
build compiled all three provider adapters regardless of which `llm-*` flags were set, so
PROJECT.md's documented contract — "unavailable adapters must fail at compile time, never at
runtime" — was not what shipped. A `--no-default-features` build of `paladin-ai` still compiled
OpenAI's, Anthropic's and DeepSeek's HTTP client construction, credential handling and response
parsing: attack surface an operator who set that flag believed they had excluded (T-17-32).

Phase 17 added five more curated presets (Kimi, Qwen, Grok, Ollama, the generic
`openai-compatible` provider) plus the bespoke Gemini adapter, each already correctly
`cfg`-gated inside `crates/paladin-llm/Cargo.toml` (plans 17-01/17-03/17-04/17-05). Extending the
facade's flag pattern to the five new providers while leaving the original three as inert stubs
was rejected by D-11 itself: two classes of flag in the same `[features]` block — three that do
nothing and six that gate correctly — is harder for an operator to reason about than either
uniform end state.

Pre-1.0 versioning (ADR-0008, D-00n) established that `release.toml`'s `shared-version = true`
absorbs a breaking change as a minor bump. That mechanism is noted here only as the fallback that
*would* have applied had option-a (the originally recorded `BREAKING` position) been taken; under
the amended decision below no breaking change is shipped, so it is not invoked.

## Decision

**The nine facade `llm-*` flags all forward into the `paladin-llm` feature of the same name; the
`paladin-llm` dependency line no longer hardcodes any provider feature.**

```toml
llm-openai = ["paladin-llm/openai"]
llm-anthropic = ["paladin-llm/anthropic"]
llm-deepseek = ["paladin-llm/deepseek"]
llm-kimi = ["paladin-llm/kimi"]
llm-qwen = ["paladin-llm/qwen"]
llm-grok = ["paladin-llm/grok"]
llm-ollama = ["paladin-llm/ollama"]
llm-gemini = ["paladin-llm/gemini"]
llm-openai-compatible = ["paladin-llm/openai-compatible"]
llm-all = ["llm-openai", "llm-anthropic", "llm-deepseek", "llm-kimi", "llm-qwen", "llm-grok", "llm-ollama", "llm-gemini", "llm-openai-compatible"]
```

`full` already included `llm-all`, so it picks up the six new providers with no further edit.
`vision` is rewired from an unconditional `[]` stub to `vision = ["paladin-llm/vision"]`, because
`paladin-llm/vision` itself depends on `paladin-llm/openai` — leaving it unconditional on the
dependency line would have silently re-established exactly the defect this ADR fixes (T-17-36).

**`default = ["llm-openai", "llm-anthropic", "llm-deepseek"]` — the exact provider set a default
build compiled before this plan.** This is the amended (option-b) position: the observable default
is preserved because **Phase 17 is an expansion of the existing adapters, not a replacement**, and
no existing consumer should observe a change. The repair is therefore a `Fixed` CHANGELOG entry,
not a `BREAKING` one. A regression test
(`tests/unit/llm/provider_factory_test.rs::default_features_still_resolve_openai_anthropic_and_deepseek`)
proves `LlmProviderFactory::create("openai")`, `.create("anthropic")` and `.create("deepseek")` all
still resolve (i.e. never return `UnknownProvider`) under whatever features the test binary itself
was compiled with — the executable form of the constraint that a `cargo tree` assertion alone
cannot prove.

**Accepted cost, deferred deliberately, not resolved here:** the default build still compiles
three providers while PROJECT.md's stated posture is opt-in adapters, so the "unavailable adapters
must fail at compile time" contract stays half-true by accident of the default feature set. That
posture question — whether `default` should eventually narrow to zero or one provider — is left
open, owned by a later phase. This ADR fixes the flag-wiring defect; it does not settle the
opt-in-posture question as a side effect.

## Considered Options

- ship the break: keep `default = ["llm-openai"]`, wire the flags, record `BREAKING` (option-a) — rejected 2026-08-17: a consumer building with default features who calls `create("anthropic")` would get `UnknownProvider` where they previously got a working adapter; the human rejected taking that break for an expansion phase
- wire the flags but widen `default` to `["llm-openai", "llm-anthropic", "llm-deepseek"]` (option-b, chosen) — fixes the inert-flag defect PROJECT.md documents with zero observable change for any existing consumer; accepted cost is that the default set still contradicts the project's opt-in posture, deferred to a later phase
- ship the break and additionally emit a compile-time diagnostic naming the remedy (option-c) — rejected: extra machinery the locked decisions did not ask for, and a `compile_error!` cannot fire on the *absence* of a feature without a sentinel, so it would realistically become an enriched runtime error message, out of this plan's scope
- leave the existing three flags as inert stubs and wire only the five new providers — rejected by D-11 itself before this amendment: two classes of flag (some real, some inert) in the same `[features]` block is harder to explain than either uniform end state

## Code Locations

- `Cargo.toml` `[features]` — the nine `llm-*` flags, `llm-all`, `full`, `default`, and the rewired `vision` flag
- `Cargo.toml` `[dependencies].paladin-llm` — no longer carries a hardcoded provider feature list, only `features = ["mock"]`
- `crates/paladin-llm/Cargo.toml` `[features]` — the nine already-`cfg`-gated provider features this plan's facade flags forward into (unchanged by this plan; wired by plans 17-01/17-03/17-04/17-05)
- `crates/paladin-llm/src/provider_factory.rs` — the table-driven registry (D-10) whose `create()`/`provider_names()` the regression test exercises
- `tests/unit/llm/provider_factory_test.rs` — `default_features_still_resolve_openai_anthropic_and_deepseek`, this ADR's own regression guard

## Code Conformance

conforms

`cargo metadata --no-deps --format-version 1` parsed and checked programmatically: every one of
`llm-openai`, `llm-anthropic`, `llm-deepseek`, `llm-kimi`, `llm-qwen`, `llm-grok`, `llm-ollama`,
`llm-gemini`, `llm-openai-compatible` forwards into a `paladin-llm/*` feature of the same name —
`all nine flags forward into paladin-llm`.

`cargo tree -p paladin-ai --no-default-features -e features` shows exactly one `paladin-llm`
feature edge (`mock`) and zero occurrences of `anthropic`, `deepseek`, `openai`, or any other
provider name — confirming the previously-unconditional feature list at the old `Cargo.toml:55`
was actually removed, not merely relabeled (17-RESEARCH.md Pitfall 2's named warning sign).

The plan's own literal `cargo tree -p paladin-ai --features llm-all -e features | grep -c gemini`
returns `0` in this workspace's cargo (1.97.1) — a display quirk of forward-direction `cargo tree
-e features`, which only prints a feature edge originating from a package's *directly declared*
dependency-feature list, not one activated purely through feature-graph unification (the facade's
`llm-gemini = ["paladin-llm/gemini"]` shape). The inverted query proves the wiring instead:
`cargo tree -p paladin-ai --features llm-all -e features -i paladin-llm` lists a
`paladin-llm feature "gemini"` node whose parent chain is
`paladin-ai feature "llm-gemini" → paladin-ai feature "llm-all" (command-line)` — and the same
inverted query lists all nine provider features with the same shape. Recorded here so a future
reader does not treat the forward-direction command as a working regression check.

`cargo check -p paladin-ai --no-default-features` and `cargo check -p paladin-ai` (default
features) both exit `0` (D-00o: run once, at plan close, per the plan's own `<verification>`
block).

## Downstream Consumers

- **PROV-03** — this ADR is the flag-wiring half of PROV-03's "adapters are feature-gated and
  additive" requirement; the config-surface half is `LlmConfig`'s extension to nine providers,
  recorded alongside this ADR in the same plan (17-06).
- **The `CHANGELOG.md` `Fixed` entry** under `## [Unreleased]` — names the repair and states
  explicitly that the default build is unchanged, so no consumer action is required.
- **Any CI or deployment workflow building `paladin-ai` with a non-default feature set** — a build
  that previously (accidentally) got Anthropic/DeepSeek compiled in under
  `--no-default-features` now genuinely does not; such a workflow must add the matching `llm-*`
  flag explicitly if it needs a provider outside the new default three.
- **A later phase owning the deferred opt-in-posture question** — whether `default` should
  eventually narrow, per PROJECT.md's stated posture, is explicitly left open by this record.
