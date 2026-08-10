# ADR-0038: `AgentProvisioner` placement — stays in `paladin-web`

## Status

Accepted

**Date:** 2026-08-10

## Context

`Milestone_12/Epic_1/prd-agent-registry-execution-api.md` §7 Open Question 2 records the
`AgentProvisioner` trait's placement as a default rather than a decision, with an explicit escape
clause: promote the trait from `crates/paladin-web` to `paladin-ports` "only if a second consumer
appears." §7 justifies calling the choice a coin-flip on the grounds that the trait references only
`Paladin` and `PaladinExecutorPort` — both already `paladin-core`/`paladin-ports` types — so nothing
in the trait signature itself pins it to the web crate.

That justification omits `AgentSpec`, the type that actually decides the question. Re-read this
session at `crates/paladin-web/src/agent_registry.rs:103-110`, the trait signature is:

```rust
async fn provision(&self, spec: &AgentSpec) -> Result<ProvisionedAgent, ProvisionError>;
```

`AgentSpec` (`:56-79`) derives `#[derive(Debug, Clone, PartialEq, Serialize, Deserialize,
utoipa::ToSchema)]` and its doc comment (`:48`) states plainly: "Sent in the body of `POST
/agents`." Its `allowed_roles` field (`:75-78`) additionally carries `#[schema(value_type =
Vec<String>)]`, an OpenAPI-schema annotation with no meaning outside an HTTP API description.
`crates/paladin-ports/Cargo.toml` (`:14-25`, re-read this session) carries eleven dependencies —
`paladin_core`, `async-trait`, `serde`, `thiserror`, `uuid`, `chrono`, `tokio`, `serde_json`,
`futures`, `md5`, `mime_guess` — and no `utoipa` entry. ADR-0015 §Decision (i)
(`.planning/decisions/0015-core-ports-dependency-allowlist.md`) states the enforceable invariant
this promotion would violate: "`paladin-core` and `paladin-ports` may carry no provider SDK … no
transport client … no storage driver, and no web framework (e.g. `axum`, `actix-web`)." `utoipa` is
an OpenAPI-schema-generation crate whose only purpose is documenting a web API surface — promoting
`AgentProvisioner` (and therefore `AgentSpec`) to `paladin-ports` would be the first `paladin-ports`
dependency whose entire reason to exist is a web framework's documentation tooling, which is exactly
the class of dependency ADR-0015(i) bars from that crate.

The escape clause's premise — "promote only if a second consumer appears" — would have fired on a
false signal. `src/infrastructure/web/facade_provisioner.rs:70` implements `AgentProvisioner` for
`FacadeProvisioner`, re-read this session; the module is gated `#[cfg(feature = "web-server")]` at
the crate level (the whole `infrastructure::web` module tree ships only under that feature) and
`src/bin/paladin-server.rs:60` is its sole construction site. `FacadeProvisioner` is the HTTP
composition root wiring the `PaladinBuilder` + `LlmProviderFactory` path behind the same
`POST /agents` route `agent_registry.rs` describes — not an independent second topology. The shipped
topology pages confirm this: `docs/src/deployment-topologies/queue-worker.md`, re-grepped this
session for `provision` and `AgentSpec`, returns no match — the queue/worker topology's worker
dequeues a typed `AgentJob` and runs it through `PaladinExecutionService` directly; it does not call
`AgentProvisioner::provision` at all. No shipped topology page describes a second, non-HTTP consumer
of spec-driven provisioning.

## Decision

`AgentProvisioner` stays in `crates/paladin-web`, because its parameter type — `AgentSpec` — is an
OpenAPI-annotated HTTP request DTO, and promoting the trait would move that schema (and its
`utoipa::ToSchema` derive) into the core-tier ports crate, directly against ADR-0015(i)'s allowlist.
The correct shape for a future non-HTTP topology that also wants spec-driven provisioning is its own
spec type over the same `PaladinBuilder` path `FacadeProvisioner` already uses — not a shared trait
whose signature is pinned to an HTTP wire format.

## Considered Options

- Keep `AgentProvisioner` in `paladin-web` (chosen) — the trait's own parameter type is an
  OpenAPI-annotated HTTP request DTO; no code change required, and the placement matches where the
  trait already lives.
- Promote `AgentProvisioner` to `paladin-ports` for reuse (rejected) — drags `utoipa` into a crate
  ADR-0015(i) explicitly forbids a web framework's tooling from entering, and requires splitting
  `AgentSpec` into a domain spec and an HTTP DTO with a `From` conversion before the trait could move.
- Leave the placement as an unratified default (rejected) — ORCH-04's done-when requires a recorded
  placement with reasoning, not a restated coin-flip.

## Code Locations

- `crates/paladin-web/src/agent_registry.rs:103-110` — the `AgentProvisioner` trait signature, re-read this session.
- `crates/paladin-web/src/agent_registry.rs:56-79` — `AgentSpec`, its `utoipa::ToSchema` derive and its `#[schema(value_type = Vec<String>)]` field, re-read this session.
- `crates/paladin-web/src/agent_registry.rs:48` — `AgentSpec`'s doc comment naming it a `POST /agents` request body.
- `crates/paladin-ports/Cargo.toml:14-25` — the `[dependencies]` block, eleven entries, no `utoipa`, re-read this session.
- `.planning/decisions/0015-core-ports-dependency-allowlist.md` §Decision (i) — the invariant this decision turns on.
- `src/infrastructure/web/facade_provisioner.rs:70` — `FacadeProvisioner`'s `impl AgentProvisioner`, gated `#[cfg(feature = "web-server")]`, the apparent "second consumer" that is actually the HTTP composition root.
- `src/bin/paladin-server.rs:60` — `FacadeProvisioner`'s sole construction site.
- `docs/src/deployment-topologies/queue-worker.md` — re-grepped this session for `provision` and `AgentSpec`; no match, confirming no second topology calls this trait.
- `.project/Milestone_12/Epic_1/prd-agent-registry-execution-api.md` §7 Open Question 2 — the source document this ADR promotes, `PROMOTION.md` Part B candidate 8.

## Code Conformance

conforms

Re-run this session: `grep -n "pub trait AgentProvisioner" crates/paladin-web/src/agent_registry.rs`
returns `crates/paladin-web/src/agent_registry.rs:103:pub trait AgentProvisioner: Send + Sync {`,
confirming the trait is where this decision says it is. This ADR changes no file; it ratifies where
`AgentProvisioner` already lives.

## Downstream Consumers

- **Phase 14 / WEB-03, WEB-04** — already opens `crates/paladin-ports/src/output/llm_port.rs` and
  `crates/paladin-web/src/agent_auth.rs`, and is the phase that would execute a move of
  `AgentProvisioner` if a human later overturns this decision.
- A future non-HTTP consumer needing spec-driven provisioning either takes a dependency on
  `paladin-web` and `axum` to reuse this trait, or writes its own spec type and trait over the same
  `PaladinBuilder` path — and the second is correct, since the first would pull a web framework into
  a non-HTTP topology. This is stated explicitly so a later reader does not re-derive the question.
