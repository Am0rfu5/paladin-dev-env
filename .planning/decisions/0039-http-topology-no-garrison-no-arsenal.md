# ADR-0039: HTTP-served agents carry no Garrison and no Arsenal — a permanent property of the topology

## Status

Accepted

**Date:** 2026-08-10

## Context

Milestone 12 Epic 2's non-goal and Epic 3's restatement both exclude Garrison (memory) and Arsenal
(tools/MCP) wiring for agents served through the HTTP service-host topology — the resident-agent
registry `crates/paladin-web/src/agent_registry.rs` backs and `src/bin/paladin-server.rs` runs.
Neither `AgentSpec` nor `AgentEntry` (`agent_registry.rs:33-79`) carries a field for either
subsystem.

The situation is worse than a documentation omission. `docs/src/deployment-topologies/
http-service-host.md`'s request-flow sequence diagram, re-read this session at line 54, reads:

```
Service->>Agent: run (LLM + tools + memory)
```

This is the identical capability phrase `embedded-library.md` uses for the one topology that
actually has Garrison and Arsenal — re-read this session, `embedded-library.md:32` points a reader
at "tools (Arsenal)" and "memory (Garrison)" in the full builder-API guide. `docs/src/
deployment-topologies/overview.md` — the page M11 Epic 6 FR-8 designates "the single source of
routing" — says nothing about the capability difference either way in its comparison table or its
routing flowchart, re-read this session. A reader who compares the two pages, or who is routed by
`overview.md`'s flowchart into the HTTP host without reading `http-service-host.md`'s prose at all,
is actively misinformed about what the topology they were just routed to can do — not merely left to
guess.

Re-grepped this session, `grep -lirE "garrison|arsenal" docs/src/deployment-topologies/*.md` returns
exactly one file: `embedded-library.md`. No other page under `deployment-topologies/` — including
`http-service-host.md` before this phase's correction, `queue-worker.md`, `sidecar.md`, or
`battalion-orchestration.md` — mentions either subsystem. `queue-worker.md`, re-read this session,
states in its own "See also" section: "Each worker is itself an embedded agent host" — the
queue/worker topology's capability set is inherited from the embedded-library topology it wraps, not
independently stated.

## Decision

The absence of Garrison and Arsenal on HTTP-served agents is a permanent property of the shipped
topology, stated in the decision matrix a reader consults before choosing — not planned scope with a
target. A consumer needing Garrison or Arsenal for an agent it also wants to expose over HTTP uses
the embedded-library topology (optionally wrapped in its own HTTP layer, which is what
`http-service-host.md`'s own "Embedding in your own app" section already documents as a supported
pattern using the same `AgentRegistry` type). Each queue/worker deployment is itself an embedded
agent host per `queue-worker.md`'s own text, so the routing story stays coherent without any new
work: the embedded-library topology is the one answer to "where do I get Garrison and Arsenal,"
reachable whether the caller is in-process, over HTTP, or via a queue.

## Considered Options

- State the absence as a permanent property of the topology, in the routing matrix (chosen) — matches what the tree already ships, requires no new API surface, and closes the misinformation gap by correcting the diagram and adding the routing-page statement.
- Make Garrison/Arsenal for HTTP-served agents planned scope with a target (rejected) — `AgentSpec` has no fields for memory or tools, and expressing an MCP server's identity, credentials and lifetime inside a JSON `POST /agents` request body is genuine API design that no milestone has scheduled; committing to it here would be an unplanned milestone smuggled into a ground-truth phase.
- Leave the limitation stated only in the Epic 2 non-goal (rejected) — one line in a non-goal is not enough surface for a reader choosing a topology to see, and `http-service-host.md`'s diagram actively contradicts it.

## Code Locations

- `crates/paladin-web/src/agent_registry.rs:33-79` — `AgentEntry` and `AgentSpec`, re-read this session; neither carries a Garrison or Arsenal field.
- `docs/src/deployment-topologies/http-service-host.md:54` — the sequence-diagram line this ADR's `## Code Conformance` names for correction (before this phase's edit: `Service->>Agent: run (LLM + tools + memory)`).
- `docs/src/deployment-topologies/embedded-library.md:32` — the correct Garrison/Arsenal advertisement this ADR routes a reader to, re-read this session.
- `docs/src/deployment-topologies/overview.md` — M11 Epic 6 FR-8's single source of routing, re-read this session; states nothing about the capability difference before this phase's edit.
- `docs/src/deployment-topologies/queue-worker.md` — "Each worker is itself an embedded agent host," re-read this session; the statement this decision's routing story depends on.
- `.project/Milestone_12/Epic_2/` and `.project/Milestone_12/Epic_3/` non-goal text — the source documents recording the exclusion this ADR promotes into a decision.

## Code Conformance

must change

Plan 13-09 task 3 is the executor. `docs/src/deployment-topologies/http-service-host.md`'s
sequence-diagram line changes from `Service->>Agent: run (LLM + tools + memory)` to
`Service->>Agent: run (LLM + prompt)`, with a new prose sentence stating the limitation and pointing
at `embedded-library.md`. `docs/src/deployment-topologies/overview.md` gains a stated limitation
where a reader chooses a topology (the HTTP-service-host row's `Avoid when` cell, a note under the
comparison table, or both) linking to `embedded-library.md`. `embedded-library.md` itself is not
touched — its existing advertisement is correct and this ADR cross-references it rather than
restating it.

## Downstream Consumers

- **Phase 14 / WEB-04** — its own text states that Arsenal/MCP and LLM tool calling "would need a
  stated relationship" for HTTP-served agents; this ADR is half of that relationship (the "not now,
  and here is why" half). A future phase building HTTP-side tool-calling support supersedes this
  ADR rather than silently contradicting it.
