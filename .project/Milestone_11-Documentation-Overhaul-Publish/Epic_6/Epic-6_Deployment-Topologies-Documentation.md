# Epic 6 — Deployment Topologies & Running Multiple Agents

**Project:** Paladin Framework
**Milestone:** 11 — Documentation Review, Reorganization, MDBook Publish
**Epic:** 6 — Deployment Topologies & Running Multiple Agents
**Version Target:** v0.5.0 (documentation written against the current workspace)
**Status:** Not Started
**Created:** 2026-06-06
**Author:** Paladin Framework Contributors

---

## Recap

Epics 2–5 stood up the MDBook site, rewrote stale content, wrote net-new subsystem
guides, and published. What is still missing is a single, decision-oriented answer to a
question consumers actually ask: **"I want to build a number of different agents — how do I
run them?"** Today the answer is scattered across `getting-started/quickstart.md`,
`user-guides/battalion-patterns.md`, an architecture note that merely *names*
`paladin-web`, and an appendix Redis setup page. There is no page that frames the choice
between **deployment topologies**, and two topologies (HTTP service host, sidecar) are not
actionable anywhere.

## Scope

Add a **new top-level "Deployment Topologies" section** to the book with a decision-matrix
landing page plus one page per topology:

1. **Embedded library** — single process, agents constructed in your own `main`.
2. **Battalion orchestration** — many agents collaborating in one runtime.
3. **HTTP service host** — agents resident behind an HTTP API (compose Axum + execution service).
4. **Queue / worker (distributed)** — agents executed by workers pulling from a Redis queue.
5. **Sidecar (separate process)** — an agent hosted in its own process, called over HTTP.

Embedded-library and Battalion topologies get fresh, focused topology pages that
**cross-link** (not duplicate) the existing `paladin-agents.md` / `orchestration.md`
guides. Every fenced Rust example compiles against the current workspace through the Epic 3
`check-doc-examples` gate; every `config.yml` snippet validates through the Epic 4
config-check gate.

## Definition of Done

- One landing page + five topology pages authored and registered in `SUMMARY.md` under a
  new top-level **Deployment Topologies** section.
- Every fenced Rust example passes `cargo check` (zero failures); every YAML/config snippet
  validates against the current schema.
- `mdbook build` succeeds with **zero** warnings (linkcheck enforcing).
- A `CHANGELOG.md` `[Unreleased]` entry records the new section.
- Honesty requirements honored: the HTTP-host and sidecar pages document **composition of
  existing APIs** and explicitly flag that no dedicated agent-HTTP endpoint or IPC/RPC
  abstraction ships today (recorded as open questions, **not** invented APIs).

## Non-Goals

- No Rust source/API changes (docs + doc-example test harness deps + check tooling only).
- No new agent-execution HTTP endpoint, gRPC/RPC layer, or sidecar abstraction in the
  framework — if a topology needs glue the framework does not provide, it is documented as
  consumer-owned code and flagged, not implemented.
- No public-API baseline (`current-exports.txt`) regeneration — this Epic is docs-only.

See `prd-deployment-topologies-documentation.md` for the full PRD and
`tasks-deployment-topologies-documentation.md` for the task breakdown.
