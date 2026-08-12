# ADR-0042: LLM-native tool calling deferred as a future capability, with a named trigger and owner

## Status

Accepted

**Date:** 2026-08-12

## Context

Deferred-QA Epic 27 (`.project/Deferred-QA-CICD-Completion/prd-deferred-qa-completion.md:124-131,250-304`)
proposes LLM-native tool calling: a `tools: Option<Vec<ToolDefinition>>` field on `LlmRequest`,
per-adapter tool-call sending and response parsing for OpenAI, Anthropic and DeepSeek, and live-API
tests gated behind `#[cfg(feature = "live-api-tests")]` (FR-27.1 through FR-27.7). This is a
contested position under D-00g's "WEB-03 gets no ADR, WEB-04 does" split — WEB-04's own text asks
whether tool calling belongs in scope at all, given Arsenal/MCP already provides tool execution —
so it is ADR material.

Re-verified this session, the epic is entirely unbuilt:

- `LlmRequest` (`crates/paladin-ports/src/output/llm_port.rs:524-538`) carries no field a tool
  definition could travel through — `id`, `model`, `prompt`, `attachments`, `stream`, `metadata`
  only. `grep -n "pub struct LlmRequest" -A 15 crates/paladin-ports/src/output/llm_port.rs`,
  re-run this session.
- No `ToolDefinition` or `ToolCall` type exists anywhere in `paladin-ports` or `paladin-llm`:
  `grep -rn "struct ToolDefinition\|struct ToolCall\b" crates/paladin-ports crates/paladin-llm`
  returns no matches (exit 1), re-run this session.
- Every producer of a populated `function_call` in the workspace is a test double under `tests/`:
  `grep -rln "function_call: Some" --include=*.rs .` returns exactly four files —
  `tests/helpers/mock_llm_adapter.rs`, `tests/integration/context_injection_test.rs`,
  `tests/integration/arsenal_bridge_regression_test.rs`,
  `tests/functional/paladin_tool_invocation_test.rs` — none of them a shipped adapter, re-run this
  session.

The epic's own PRD (`:490-496`) flags the cost this record adopts: implementing it modifies the
`LlmPort` trait — a breaking change to a published port interface — requiring all three shipped
adapters (OpenAI, Anthropic, DeepSeek) and the bundled mock to change together, and proposes a
four-phase approach precisely because of that cost.

Both of Epic 27's open questions (`:570,574`) are still unanswered: **OQ-1** — does DeepSeek's API
support tool calling at all, since its adapter currently declares `supports_tool_calling: false`;
and **OQ-5** — should `LlmRequest.tools` use a vendor's JSON Schema format as canonical, or a
provider-agnostic schema of this project's own design. Neither is resolved by this record; both are
named below as part of why the capability stays deferred.

WEB-04 asks this record to state the Arsenal relationship, pairing with ADR-0039's already-recorded
HTTP half (D-00j). ADR-0039 records the absence of Garrison and Arsenal on HTTP-served agents as a
permanent property of that topology. Arsenal and MCP themselves already provide tool execution
through a working seam — the Arsenal tool system, its ports, registry, MCP client and transports all
shipped in Milestone 1 Epic 3 and are invocable today by a consumer supplying their own `LlmPort`
implementation that parses tool calls. What is missing is not tool execution but the LLM-initiated
entry into it: the reasoning loop's tool-invocation branch
(`src/application/services/paladin/paladin_execution_service.rs:799`), including the
`handoff_to_specialist` path (`:1414`), reads `response.function_call` — and no shipped adapter ever
populates that field, so the branch fires only for a consumer's own `LlmPort`.

## Decision

**LLM-native tool calling is recorded as a future capability improvement, not built (D-10).**

The user's framing is the load-bearing rationale for this decision and for the whole phase, recorded
verbatim:

> "We want to maximize the capabilities so this sounds like a future feature improvement. This is
> listed under 'deferred'. Some of these Epics in this Milestone may have been completed already
> but this is the source of some potential future version improvements not any current
> functionality. This should be recorded as such and everything should properly reflect current
> functionality. Make your decision based on this perspective."

**Trigger:** the first consumer that needs a shipped adapter — rather than its own `LlmPort`
implementation — to initiate a tool call, **conditioned on both open questions being answered
first**: OQ-5 (canonical vs. provider-agnostic schema) determines the type design `ToolDefinition`/
`ToolCall` would take, and OQ-1 (DeepSeek support) determines whether the reintroduction spans all
three adapters or two.

**Owner:** the `LlmPort` trait in `paladin-ports` (`crates/paladin-ports/src/output/llm_port.rs`),
plus its three shipped adapters and the mock in `paladin-llm`
(`crates/paladin-llm/src/{openai/adapter,anthropic/adapter,deepseek/adapter,mock}.rs`) — the concrete
surface a future phase changes together, per the epic's own breaking-change note.

Nothing is deleted. The trigger is what brings the capability back — this is a **deferral**, not a
withdrawal, and not the permanent-property treatment ADR-0039 gives a different question (the
absence of Garrison/Arsenal on HTTP-served agents is stated as never changing; LLM-native tool
calling is stated as not-yet, pending the trigger).

**The bundled mock is left unchanged.** `crates/paladin-llm/src/mock.rs` already declares both
`supports_tool_calling` and `supports_function_calling` as `false` (`:267-268`, `:379-380`) and never
populates `function_call` (`:224`, `:346`) — it is already truthful under WEB-03/WEB-04 and needs no
correction. CONTEXT.md raised, as Claude's Discretion, the option of letting the mock emit a
`FunctionCall` so the tool path is demonstrable without a custom adapter; this record closes that
discretion item for this phase by declining it, and notes it as a possible future step — not
planned, not required by the trigger above, available to whichever phase picks the trigger up.

## Considered Options

- build Epic 27 now following the PRD's own four-step phased approach (`:493-496`) (rejected — a breaking change across three shipped adapters and the mock, gated on two open questions that determine the type design, is not a fix scoped to a truthfulness phase)
- withdraw the capability entirely, deleting the deferred-register entries and the port's `function_call`/`FinishReason::FunctionCall` surface (rejected — the user explicitly wants it recorded as a future improvement, not deleted, per D-10's verbatim framing)
- record it as deferred with a named trigger and an owner, following the ADR-0035 `paladin-ml` precedent (chosen — promotes the condition above DOC precedence without building anything the corpus has not authorised)
- add a fourth entry to a deferred register alongside the three that already exist (rejected under D-11 — WEB-04's own text names exactly this as the failure mode it must not repeat)

## Code Locations

- `crates/paladin-ports/src/output/llm_port.rs:524-538` — `LlmRequest`, re-verified this session to carry no field a tool definition could travel through (`id`, `model`, `prompt`, `attachments`, `stream`, `metadata` only)
- `crates/paladin-ports/src/output/llm_port.rs:619-632` — `LlmResponse.function_call`'s reachability rustdoc, added by plan 14-03, pointing at this record
- `crates/paladin-ports/src/output/llm_port.rs:807-867` — `ProviderCapabilities`'s "Tool-call reachability" doc section and its `supports_tool_calling`/`supports_function_calling` field docs, added by plan 14-03
- `docs/src/user-guides/tool-integration.md` — the reachability callout plan 14-03 added after the Overview section
- `docs/src/architecture/overview.md` — the footnote plan 14-03 added under the reasoning-loop state diagram
- `docs/src/architecture/domain-model.md` — the sentence plan 14-03 added distinguishing Arsenal's two seams
- `docs/src/contributing/contributing-providers.md` — the honest `get_capabilities()` template and correspondence-test pointer plan 14-03 added
- `crates/paladin-llm/src/lib.rs:104-158` — `test_capabilities_tool_calling_matches_request_surface`, extended by plan 14-02 to pin both `supports_tool_calling` and `supports_function_calling` against the request/response surface
- `crates/paladin-llm/src/mock.rs:224,267-268,346,379-380` — the bundled mock, re-verified this session to already declare both capability flags `false` and emit no `function_call`
- `src/application/services/paladin/paladin_execution_service.rs:799` — the reasoning loop's tool-invocation branch, reachable only through a consumer-supplied `LlmPort` implementation
- `src/application/services/paladin/paladin_execution_service.rs:1414` — `is_handoff_tool_call`, the specialist-handoff path gated behind the same unreachable `function_call`
- `.project/Deferred-QA-CICD-Completion/prd-deferred-qa-completion.md:124-131,250-304,490-496,557,570,574` — the five Epic 27 sites Task 2 annotates: the user-story block, the functional-requirements section, the breaking-change/phased-approach risk note, the epic-priority ordering row, and both open questions

## Code Conformance

conforms

No code change is made by this record. The current-functionality corrections that make the tree
match it landed by plans 14-02 (`OpenAIAdapter::get_capabilities().supports_function_calling`
flipped `true` → `false` and pinned alongside `supports_tool_calling` in one extended correspondence
test, commits `3ccf2d0`/`8ad9908`) and 14-03 (the reachability rustdoc on `LlmResponse.function_call`
and `ProviderCapabilities`, plus the four documentation pages, commits `498adb7`/`f3e5f82`), both
re-verified against the tree this session.

## Downstream Consumers

- **The `REQ-llm-tool-calling-port` and `REQ-llm-tool-calling-adapters` ledger rows** in
  `.planning/ledgers/milestone-09-12.md` — plan 14-07 amends them against this record.
- **The five Epic 27 sites** in `.project/Deferred-QA-CICD-Completion/prd-deferred-qa-completion.md`
  that Task 2 of this plan annotates with a dated banner pointing here.
- **ADR-0039** — this record supplies the other half of the Arsenal/`LlmPort` relationship WEB-04
  asks for; a future phase building HTTP-side tool-calling support supersedes both rather than
  silently contradicting either.
- **Any future phase picking up the reintroduction trigger** — inherits the owner surface
  (`LlmPort` in `paladin-ports`, its three adapters and the mock in `paladin-llm`) and the
  precondition that both open questions (OQ-1 DeepSeek support, OQ-5 canonical schema) are answered
  first.
