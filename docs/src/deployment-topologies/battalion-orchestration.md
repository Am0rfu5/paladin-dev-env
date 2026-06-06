# Battalion Orchestration (Many Agents, One Runtime)

When several agents should **collaborate on one task** — rather than serve independent
requests — use a **Battalion**. It runs many Paladins in a single `tokio` runtime with a
coordination pattern built in, so you express the *relationship* between agents instead of
hand-rolling the concurrency.

> The example below is compiled code pulled from the `paladin-doc-examples` crate via mdBook
> `{{#include}}`, so it matches the current API.

## When to choose it

- **Choose it when** the agents form a *workflow*: a pipeline, a fan-out/fan-in, a DAG, or a
  lead delegating to specialists. The Battalion owns ordering, concurrency limits, and error
  strategy for you.
- **Look elsewhere when** the agents are *independent* request handlers — a plain
  [agent registry](embedded-library.md) (optionally behind an
  [HTTP host](http-service-host.md)) fits better than an orchestration pattern.

This is still a **single-process** topology — it composes naturally with the others: a
[worker](queue-worker.md) or an [HTTP host](http-service-host.md) can run a Battalion as the
unit of work it executes.

## Example: parallel agents (Phalanx)

A **Phalanx** fans the same input out to several Paladins concurrently and aggregates the
results — the most direct "many agents, one runtime" pattern. Note the `with_max_concurrency`
cap and the `BattalionConfig`:

```rust
{{#include ../../../crates/doc-examples/src/orchestration.rs:phalanx}}
```

## Picking a pattern

| Your agents should… | Pattern | Service type |
|---|---|---|
| Run in a fixed order, each feeding the next | Formation (sequential) | `FormationExecutionService` |
| Run together on the same input, then aggregate | Phalanx (parallel) | `PhalanxExecutionService` |
| Follow explicit dependencies / branches | Campaign (DAG) | `CampaignExecutionService` |
| Have a lead delegate to specialists | Chain of Command | `ChainOfCommandExecutionService` |
| Use a pattern chosen per-request | Commander (auto-route) | `CommanderBuilder` |

The full guides cover every pattern with a worked, compiled example, plus Conclave, Council,
Grove, and the Maneuver flow DSL:

- [Orchestration Patterns](../user-guides/orchestration.md) — the comprehensive reference.
- [Battalion Orchestration Patterns](../user-guides/battalion-patterns.md) — pattern-by-pattern
  walkthrough.

---

← Back to [Choosing a topology](overview.md)
