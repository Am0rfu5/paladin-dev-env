---
phase: 13-milestone-9-12-ground-truth-recorded-account
reviewed: 2026-08-10T21:14:12Z
depth: standard
files_reviewed: 12
files_reviewed_list:
  - .project/Deferred-QA-CICD-Completion/DEFERRED_COVERAGE.md
  - .project/Deferred-QA-CICD-Completion/prd-deferred-qa-completion.md
  - .project/Milestone_12-Web-API/Epic_1/prd-agent-registry-execution-api.md
  - .project/Milestone_12-Web-API/Epic_1/tasks-agent-registry-execution-api.md
  - .project/Milestone_12-Web-API/Epic_3/prd-streaming-async-execution.md
  - .project/Milestone_12-Web-API/Epic_3/tasks-streaming-async-execution.md
  - .project/Milestone_12-Web-API/Epic_4/prd-api-cross-cutting-concerns.md
  - .project/Milestone_12-Web-API/Epic_4/tasks-api-cross-cutting-concerns.md
  - .project/Milestone_12-Web-API/Epic_5/prd-api-security-authorization.md
  - docs/src/deployment-topologies/http-service-host.md
  - docs/src/deployment-topologies/overview.md
  - docs/src/deployment-topologies/sidecar.md
findings:
  critical: 1
  warning: 1
  info: 1
  total: 3
status: issues_found
---

# Phase 13: Code Review Report

**Reviewed:** 2026-08-10T21:14:12Z
**Depth:** standard
**Files Reviewed:** 12
**Status:** issues_found

## Summary

Phase 13 is a documentation ground-truth correction phase; no source files changed. All 12
reviewed Markdown files were checked against the live Rust source (`crates/paladin-web/src/`,
`src/config/`, `crates/doc-examples/src/`) and against each other for the specific defect classes
this phase targets: factual claims contradicted by code, internal contradictions, broken
cross-references, and security-relevant misstatements.

**What verified clean:**
- The `/v1` route-prefix correction is accurate and consistently applied. `crates/paladin-web/src/agent_controller.rs:723` defines `API_V1_PREFIX = "/v1"`, and `openapi.rs`'s `spec_paths_are_versioned_under_v1` test (confirmed present at `crates/paladin-web/src/openapi.rs:103`) asserts `/v1/agents` and `/v1/agents/{id}/execute` in the committed `openapi.json` baseline. Every superseded-route annotation in the five Epic PRD/task docs correctly names the `/v1`-prefixed successor.
- The Garrison/Arsenal-absence correction in `overview.md` and `http-service-host.md` is accurate and mutually consistent: `AgentSpec` (`crates/paladin-web/src/agent_registry.rs:57-77`) has no field for either capability, confirming both pages' claim that this is a permanent topology property, not a gap.
- All four `.project/` relocation corrections (listener service, `llm_port.rs`, `Design_and_Architecture.md`, asciinema/README) check out against the current tree: `src/application/services/orchestration/listener.rs` exists with `ListenerOrchestrator` at line 141; the old path is absent; `crates/paladin-ports/src/output/llm_port.rs` exists and the old `src/application/ports/` directory is absent; `docs/src/appendix/design-and-architecture.md` exists at exactly 311 lines with zero occurrences of the seven newer subsystems; `README.md` is 193 lines with zero "asciinema"/"demo" hits, and `docs/assets/`/`docs/DEMOS.md` are both absent while `docs/src/assets/` exists as claimed.
- `tests/event_trigger_pipeline.rs` has exactly 5 `#[tokio::test]` functions, matching the "5 passing tests" figure cited in both the coverage doc and the PRD.
- The health/readiness, OpenAPI/docs, and auth (`X-API-Key`/JWT, `AuthConfig`/`ApiKeyConfig`) claims in `http-service-host.md` all match the live `paladin-web` implementation (`health.rs`, `openapi.rs`, `src/config/agents.rs`).
- The `sidecar.md` prose itself correctly states the server-side route as `/v1/agents/{id}/execute`.

One genuine defect was found (below) plus two lower-severity gaps.

## Critical Issues

### CR-01: `sidecar.md`'s embedded caller-side code example targets a route that does not exist on the live server

**File:** `docs/src/deployment-topologies/sidecar.md:26-38` (embeds `crates/doc-examples/src/sidecar.rs:34`)

**Issue:** `sidecar.md`'s prose correctly states the server-side contract as `POST /v1/agents/{id}/execute`
(line 30, itself quoting the ADR-0037-corrected `http-service-host.md`). But the very code block
the page includes two lines later — pulled in verbatim via mdBook `{{#include}}` from
`crates/doc-examples/src/sidecar.rs` — builds its request URL as:

```rust
.post(format!("{base_url}/agents/{agent}/execute"))
```

i.e. the **unprefixed** `/agents/{id}/execute`, not `/v1/agents/{id}/execute`. This is an internal
contradiction within the same document: the prose sentence immediately above the code block states
the correct `/v1`-prefixed route, and the compiled example directly below it targets a route the
server does not serve (a real `paladin-server` instance would 404 this request, since
`agent_router`/`API_V1_PREFIX` mount everything under `/v1` — confirmed at
`crates/paladin-web/src/agent_controller.rs:723` and the `spec_paths_are_versioned_under_v1` test
at `crates/paladin-web/src/openapi.rs:103-111`). A reader who copies this "compiled, matches the
current API" example (the page's own claim: *"so it matches the current `reqwest` API"*) will
write a client against a route that returns `404 Not Found`. This is exactly the failure mode the
phase's other corrections were written to prevent, and it was missed in the source file that the
corrected `.md` page embeds.

Note: `crates/doc-examples/src/sidecar.rs` itself is outside this phase's reviewed file list (only
the `.md` page was in scope), but the defect is squarely inside the reviewed document's rendered
content — `sidecar.md` presents this code as part of its own body via `{{#include}}`, and the page
claims it "matches the current API."

**Fix:** Update `crates/doc-examples/src/sidecar.rs:34` to build the URL against the versioned
route:
```rust
.post(format!("{base_url}/v1/agents/{agent}/execute"))
```
`cargo check -p paladin-doc-examples` will not catch this on its own (the URL is just a string), so
this needs a manual fix plus, ideally, an assertion or comment tying the literal back to
`paladin_web::agent_controller::API_V1_PREFIX` so future prefix changes don't silently re-introduce
the drift.

## Warnings

### WR-01: One unprefixed `/agents` route mention in `prd-agent-registry-execution-api.md`'s illustrative code block was not annotated as superseded

**File:** `.project/Milestone_12-Web-API/Epic_1/prd-agent-registry-execution-api.md:289-296`

**Issue:** Every other unprefixed `/agents...` route mention in this document (and its sibling
Epic 1/3/4/5 docs) is systematically followed within a line or two by an
`> *(superseded — ADR-0037: shipped as ...)*` annotation. One instance was missed: the "State
sharing" illustrative code block's inline comment —
```text
provisioner: Option<Arc<dyn AgentProvisioner>>, // injected by Epic 2; None ⇒ POST /agents fails closed
```
— has no corresponding superseded note before the next section (`## 7. Technical Considerations`
begins directly after). This is a low-consequence miss (it's a behavioral comment inside
illustrative pseudocode, not a route declaration in the requirements/table text), but it breaks the
otherwise-complete annotation pattern this phase established, and a careful reader cross-checking
route correctness against this file's own convention would reasonably expect every unprefixed
`/agents` mention to carry the marker.

**Fix:** Add a superseded note after the code block, consistent with the pattern used elsewhere in
this file, e.g.:
```markdown
> *(superseded — ADR-0037: shipped as `POST /v1/agents`)*
```

## Info

### IN-01: `.project/current-exports.txt` byte count in the DEBT-01 correction banners is now stale

**File:** `.project/Milestone_12-Web-API/Epic_1/prd-agent-registry-execution-api.md:1-14` and
`.project/Milestone_12-Web-API/Epic_5/prd-api-security-authorization.md:1-14`

**Issue:** Both DEBT-01 correction banners (dated 2026-08-06) cite the baseline file's size as
"442,369 bytes." The file on disk today is 446,377 bytes. This is expected drift — later Epics
(3 and 4) regenerated the export baseline additively after the DEBT-01 correction was written — and
does not misdirect a reader about the *path*, which is the banner's actual claim and remains
correct. Not a defect in the reviewed documents' logic, just a byte-count that will always be a
point-in-time snapshot.

**Fix:** None required. If these banners are ever revised, consider dropping the specific byte
count (which decays) and keeping only the path-existence claim (which doesn't).

---

_Reviewed: 2026-08-10T21:14:12Z_
_Reviewer: Claude (gsd-code-reviewer)_
_Depth: standard_
