# ADR-0037: The agent route surface is `/v1`

## Status

Accepted

**Date:** 2026-08-10

## Context

Five Milestone 12 Epics disagree, in requirement text, about the agent route surface. Epics 1, 3,
4 and 5 write acceptance criteria, test assertions, and examples against **unprefixed** paths
(`POST /agents/{id}/execute`, `POST /agents`, `DELETE /agents/{id}`, `POST /agents/{id}/execute/stream`,
`POST /agents/{id}/jobs`, `GET /agents/{id}/jobs/{job_id}`). Epic 6 §4.3 (`prd-openapi-spec-interactive-docs.md:112-116`)
requires the opposite: "The agent API routes **must** be served under a `/v1` prefix
(`/v1/agents`, `/v1/agents/{id}/...`, `/v1/agents/{id}/execute[/stream]`,
`/v1/agents/{id}/jobs[/{job_id}]`)... `/health`, `/ready`, `/openapi.json`, and `/docs` **must**
remain unversioned."

The committed drift-guard baseline settles this by construction, not by argument. Re-run this
session: `grep -n '"/v1/agents' crates/paladin-web/openapi.json` returns all six agent paths,
every one `/v1`-prefixed — `/v1/agents` (`:17`), `/v1/agents/{id}` (`:148`),
`/v1/agents/{id}/execute` (`:271`), `/v1/agents/{id}/execute/stream` (`:382`),
`/v1/agents/{id}/jobs` (`:489`), `/v1/agents/{id}/jobs/{job_id}` (`:580`). This baseline is not a
snapshot a human could have drifted from the code — `openapi.rs:120-149`'s
`openapi_matches_committed_baseline` test regenerates the spec in-process on every `cargo test`
run and asserts byte-for-byte equality with this file, so the committed JSON and the served API
cannot diverge without a failing test. `openapi.rs:103`'s `spec_paths_are_versioned_under_v1`
asserts the `/v1` prefix live, not just against the committed copy.

The one live consequence of this disagreement is `docs/src/deployment-topologies/sidecar.md`
(`:29`), which tells a reader to call `POST /agents/{id}/execute` — the unprefixed form — against
a server that in fact serves the `/v1`-prefixed path. Re-run this session:
`grep -rcE '\(`?POST /agents' docs/src/ examples/ README.md 2>/dev/null | grep -v ':0$'` returns
exactly one hit, `docs/src/deployment-topologies/sidecar.md`. It is the only unprefixed agent
route reference anywhere under `docs/src/`, `examples/`, or `README.md` — every other published
page, including `docs/src/deployment-topologies/http-service-host.md` (`:29`, the server side
`sidecar.md` itself points a reader at), already uses the `/v1`-prefixed form.

ORCH-03(a)'s instruction to preserve the competing route surface as a run-5 unsettled position in
the variants register still holds: this ADR records the answer for anyone applying a run-5
requirement literally; the register (`.planning/INGEST-CONFLICTS.md`) keeps the disagreement as a
historical artefact of the ingest. This ADR does not edit the variants register.

## Decision

The agent API is served under a **`/v1`** version prefix; operational endpoints (`/health`,
`/ready`) and docs endpoints (`/openapi.json`, `/docs`) remain unversioned. Milestone 12 Epic 1,
3, 4 and 5's unprefixed route text is **superseded provenance, not a live contract** — it
described an intended shape before Epic 6 fixed the versioning policy, and the shipped code never
implemented the unprefixed form. `docs/src/deployment-topologies/sidecar.md`'s route reference is
corrected to the prefixed form, since it is the one place this disagreement reached a published,
executable contract a reader would act on.

## Considered Options

- **Adopt Epic 6's `/v1` prefix as the answer** (chosen) — the committed `openapi.json` baseline locks in whichever form actually shipped, verified live by `spec_paths_are_versioned_under_v1` and enforced against drift by `openapi_matches_committed_baseline`; a compiled, tested artefact is more authoritative than five PRDs' acceptance-criteria prose, four of which predate Epic 6's versioning decision.
- **Treat the unprefixed form as the contract and change the server** (rejected) — this would break the shipped `openapi.json` drift guard, `spec_paths_are_versioned_under_v1`, `tests/web_server_e2e.rs`'s `/v1/agents/researcher/execute` assertions, and every deployed client already calling the `/v1`-prefixed routes; there is no correctness or security reason to prefer the unprefixed form.
- **Record the disagreement without answering it** (rejected) — ORCH-03's done-when requires that anyone applying a run-5 requirement literally cannot write to a path that does not exist; leaving the question open would leave exactly that trap in place for a future reader of Epics 1, 3, 4, or 5.

## Code Locations

- `crates/paladin-web/openapi.json:17,148,271,382,489,580` — the six agent paths, all `/v1`-prefixed, the committed drift-guard baseline this ADR's decision rests on.
- `.project/Milestone_12-Web-API/Epic_6/prd-openapi-spec-interactive-docs.md:112-116` — §4.3 Versioning, the position that shipped and that this ADR ratifies.
- `docs/src/deployment-topologies/http-service-host.md:29` — the server-side page's existing correct usage of `POST /v1/agents/{id}/execute`, the form `sidecar.md`'s correction must match character-for-character.
- `docs/src/deployment-topologies/sidecar.md:29` — the one live defect site, corrected by plan 13-08 task 2 to name the same prefixed route.
- `.project/Milestone_12-Web-API/Epic_1/prd-agent-registry-execution-api.md` and `Epic_1/tasks-agent-registry-execution-api.md` — one of four superseded-provenance source documents, annotated by plan 13-08 task 3.
- `.project/Milestone_12-Web-API/Epic_3/prd-streaming-async-execution.md` and `Epic_3/tasks-streaming-async-execution.md` — a second superseded-provenance pair, annotated by plan 13-08 task 3.
- `.project/Milestone_12-Web-API/Epic_4/prd-api-cross-cutting-concerns.md` and `Epic_4/tasks-api-cross-cutting-concerns.md` — a third superseded-provenance pair, annotated by plan 13-08 task 3.
- `.project/Milestone_12-Web-API/Epic_5/prd-api-security-authorization.md` — the fourth superseded-provenance document, annotated by plan 13-08 task 3.

## Code Conformance

must change

The one live consequence this ADR instructs is a single-line correction, executed by **plan
13-08, task 2**, in this same phase. In `docs/src/deployment-topologies/sidecar.md`, the "Server
side" bullet's parenthetical is changed from the unprefixed before-string
`POST /agents/{id}/execute` to the prefixed after-string `POST /v1/agents/{id}/execute` — the
identical string already used by `http-service-host.md:29` and by
`crates/paladin-web/openapi.json:271`'s `/v1/agents/{id}/execute` path key. The verifying command,
re-run after the edit: `grep -c 'POST /v1/agents/{id}/execute' docs/src/deployment-topologies/sidecar.md`
→ `1`, and `grep -c 'POST /agents/{id}/execute' docs/src/deployment-topologies/sidecar.md` → `0`.
This mirrors ADR-0032's precedent of a `must change` ADR whose consequence is executed by the same
phase that writes it, rather than deferred to a later phase.

## Downstream Consumers

- **Phase 14 / WEB-01** — inherits the settled route surface (`/v1`-prefixed agent routes,
  unversioned operational/docs routes) so the auth work does not re-derive or re-litigate it.
- **Phase 15 / PIPE-01** — inherits the versioned surface as the contract any future CI route
  check gates against, and the drift-guard tests (`openapi_matches_committed_baseline`,
  `spec_paths_are_versioned_under_v1`) as the mechanism that already enforces it.
