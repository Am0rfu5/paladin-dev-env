# Sidecar (Separate Process)

Run an agent in its **own process**, deployed and scaled independently, and have your main
application call it over the network. Use this when you need hard process, security, or
deploy isolation per agent — at the cost of more operational moving parts.

> The caller-side example below is compiled code pulled from the `paladin-doc-examples` crate
> via mdBook `{{#include}}`, so it matches the current `reqwest` API.

> **Paladin ships no IPC / gRPC / RPC / sidecar transport.** There is no first-class
> "call an agent in another process" mechanism in the workspace. The sidecar pattern is
> *composed*: the agent runs behind an [HTTP service host](http-service-host.md) (the server
> side), and your app calls it with a plain HTTP client (the caller side, below). **You own
> the wire contract** — the URL shape and the request/response types.

## When to choose it

- **Choose it when** an agent needs its own process boundary: independent deployment or
  scaling, a different security context, language/runtime isolation, or blast-radius
  containment.
- **Look elsewhere when** in-process hosting gives the same benefit more cheaply — a single
  [HTTP service host](http-service-host.md) keeps many agents resident in one process and
  avoids the network hop and extra deployment entirely. Prefer it unless isolation is a hard
  requirement.

## The two sides

- **Server side** — the agent runs behind the [HTTP service host](http-service-host.md)
  exactly as documented there (`POST /v1/agents/{id}/execute`), deployed as its own process or
  container.
- **Caller side** — your app makes an HTTP request to that endpoint:

```rust
{{#include ../../../crates/doc-examples/src/sidecar.rs:sidecar_client}}
```

The request/response structs here must mirror the host's contract. Because that contract is
consumer-owned, keep the two sides in sync yourself (e.g. a shared crate of DTOs).

## What a first-class sidecar would need

Paladin does not provide this today; documented here as a limitation and a possible future
direction. A first-class sidecar transport would add:

- A **transport port trait** (e.g. a `RemoteAgentPort`) abstracting "execute this agent in
  another process," with HTTP and/or gRPC adapters.
- A **serialization contract** for agent requests/results across the process boundary, so the
  wire types are defined and versioned by the framework rather than hand-rolled per consumer.

Until then, the composed HTTP host + client above is the supported approach.

## See also

- The server side of this pattern — [HTTP service host](http-service-host.md).
- Keeping agents in one process instead — [Embedded library](embedded-library.md).

---

← Back to [Choosing a topology](overview.md)
