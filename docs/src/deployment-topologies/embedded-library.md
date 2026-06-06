# Embedded Library (Single Process)

The simplest topology: depend on `paladin-ai` (library name `paladin`) and build your
agents directly in your own binary. Paladin is designed for this — the root crate is a
**composition root**, not a framework that owns your process — so "embed it as a library and
build each agent's behaviour in your app" is the grain of the design, not a workaround.

> The code blocks below are compiled examples pulled from the `paladin-doc-examples` crate
> via mdBook `{{#include}}`, so they are guaranteed to match the current API.

## When to choose it

- **Choose it when** you control invocation in-code, all agents share one process, and you
  want the least moving parts. It is the right starting point for almost every project.
- **Look elsewhere when** an external client needs to call your agents
  ([HTTP service host](http-service-host.md)), you need scale-out or backpressure
  ([queue / worker](queue-worker.md)), or the agents collaborate on a single task
  ([Battalion orchestration](battalion-orchestration.md)).

## One agent

Build an agent with the fluent `PaladinBuilder`, then run it through a
`PaladinExecutionService`. The mock LLM keeps the example offline; swap in
`OpenAIAdapter::from_env()?` (or another adapter) for real use.

```rust
{{#include ../../../crates/doc-examples/src/readme.rs:quickstart}}
```

See the [Paladin Agents](../user-guides/paladin-agents.md) guide for the full builder API —
system prompt, model, temperature, loops, stop words, vision, memory (Garrison), and tools
(Arsenal).

## Multiple distinct agents in one process

Because Paladins are `Send + Sync` and everything runs on `tokio`, you can keep many
different agents resident in one process and route to them. A small **agent registry** —
a map from a name to an agent plus its execution service — is all you need:

```rust
{{#include ../../../crates/doc-examples/src/deployment_topologies.rs:embedded_registry}}
```

Each entry can differ by system prompt, model, tools, or memory — that is what makes them
"different agents." Calls are independent and run concurrently on the runtime, so several
`run(..)` futures can be in flight at once.

This registry is also the foundation of the next topology: the
[HTTP service host](http-service-host.md) wraps exactly this map behind an HTTP handler so
an external client can invoke each agent. When the agents instead collaborate on one task,
reach for [Battalion orchestration](battalion-orchestration.md).

---

← Back to [Choosing a topology](overview.md)
