---
phase: 13-milestone-9-12-ground-truth-recorded-account
plan: 08
subsystem: docs
tags: [adr, openapi, versioning, agent-api, documentation]

# Dependency graph
requires:
  - phase: 13-milestone-9-12-ground-truth-recorded-account
    provides: "13-01's ADR conventions/promotion mechanism, 13-05/13-06's ledger rows for REQ-agent-execute-endpoint / REQ-api-v1-versioning / REQ-openapi-drift-guard"
provides:
  - "ADR-0037: the agent route surface is `/v1`, confirmed against the committed openapi.json drift-guard baseline"
  - "docs/src/deployment-topologies/sidecar.md corrected to name the live /v1-prefixed route"
  - "Seven Milestone 12 Epic documents (Epic 1 prd+tasks, Epic 3 prd+tasks, Epic 4 prd+tasks, Epic 5 prd) dated-banner-annotated as superseded route provenance"
affects: [14-web-01, 15-pipe-01]

# Tech tracking
tech-stack:
  added: []
  patterns: ["ADR-per-contested-position (D-00g)", "dated correction banner + inline superseded-note annotation (D-00c), never rewriting a source document"]

key-files:
  created:
    - .planning/decisions/0037-agent-route-surface-v1.md
  modified:
    - docs/src/deployment-topologies/sidecar.md
    - .project/Milestone_12-Web-API/Epic_1/prd-agent-registry-execution-api.md
    - .project/Milestone_12-Web-API/Epic_1/tasks-agent-registry-execution-api.md
    - .project/Milestone_12-Web-API/Epic_3/prd-streaming-async-execution.md
    - .project/Milestone_12-Web-API/Epic_3/tasks-streaming-async-execution.md
    - .project/Milestone_12-Web-API/Epic_4/prd-api-cross-cutting-concerns.md
    - .project/Milestone_12-Web-API/Epic_4/tasks-api-cross-cutting-concerns.md
    - .project/Milestone_12-Web-API/Epic_5/prd-api-security-authorization.md

key-decisions:
  - "The agent API is served under /v1; Milestone 12 Epics 1, 3, 4 and 5's unprefixed route text is superseded provenance, not a live contract (ADR-0037)"
  - "Inline annotation used a new-line-per-occurrence pattern (never modifying an existing line) rather than Phase 8's strikethrough-in-place pattern, so that git diff shows zero deleted lines across .project/Milestone_12-Web-API/ per the plan's acceptance criteria"
  - "Route sites in Epic 2's prd/tasks and the milestone overview document, found outside D-11's four named Epics, are recorded here rather than absorbed into this plan's edit set"

requirements-completed: [ORCH-03]

coverage:
  - id: D1
    description: "ADR-0037 exists with the corpus's exact seven-heading ADR shape, no frontmatter, /v1 as the recorded answer with openapi.json citation, a must-change verdict naming this plan's task 2, and Phase 14/Phase 15 as downstream consumers"
    requirement: "ORCH-03"
    verification:
      - kind: other
        ref: "grep -c '^## ' .planning/decisions/0037-*.md -> 7; grep -c '/v1/agents' -> 11; awk Code Conformance block grep -c 'must change' -> 2; awk Downstream Consumers block grep -c 'Phase 14' -> 1"
        status: pass
    human_judgment: false
  - id: D2
    description: "docs/src/deployment-topologies/sidecar.md's live route reference corrected from unprefixed to /v1-prefixed, matching http-service-host.md and openapi.json exactly; mdbook build result unchanged from the pre-phase baseline"
    requirement: "ORCH-03"
    verification:
      - kind: other
        ref: "grep -c 'POST /v1/agents/{id}/execute' sidecar.md -> 1; grep -c 'POST /agents/{id}/execute' -> 0; git diff --numstat -> 1 line changed; mdbook build docs/ -> exit 101, same two pre-existing errors (deployment/docker.md:118, user-guides/tool-integration.md:324), no new error naming sidecar.md"
        status: pass
    human_judgment: false
  - id: D3
    description: "All seven Milestone 12 Epic 1/3/4/5 documents carry a dated ADR-0037 correction banner with every superseded route string retained inline and marked superseded, zero original lines deleted"
    requirement: "ORCH-03"
    verification:
      - kind: other
        ref: "grep -rl 'ADR-0037' .project/Milestone_12-Web-API/ | wc -l -> 7; grep -rl '0037-agent-route-surface-v1.md' -> 7; git diff -- .project/Milestone_12-Web-API/ | grep -c '^-[^-]' -> 0; git diff --name-only -- '.project/*' | wc -l -> 7; git diff --name-only -- '*.rs' | wc -l -> 0"
        status: pass
    human_judgment: false

duration: 20min
completed: 2026-08-10
status: complete
---

# Phase 13 Plan 08: Agent Route Surface ADR Summary

**ADR-0037 records the agent route surface as `/v1`-prefixed against the committed `openapi.json` baseline, corrects the one live doc defect in `sidecar.md`, and annotates four Milestone 12 Epic documents as superseded provenance without deleting a line of their original text.**

## Performance

- **Duration:** 20 min
- **Started:** 2026-08-10T20:18:00Z (approx.)
- **Completed:** 2026-08-10T20:38:31Z
- **Tasks:** 3
- **Files modified:** 9 (1 created, 8 modified)

## Accomplishments

- Authored `.planning/decisions/0037-agent-route-surface-v1.md` with the corpus's exact seven-heading ADR shape (no frontmatter), citing `crates/paladin-web/openapi.json`'s six `/v1`-prefixed agent paths as the settling artefact, a `must change` verdict naming this plan's own task 2 as executor, and Phase 14 / WEB-01 and Phase 15 / PIPE-01 as downstream consumers.
- Corrected `docs/src/deployment-topologies/sidecar.md`'s one live route defect — `POST /agents/{id}/execute` → `POST /v1/agents/{id}/execute` — a one-line diff, character-identical to `http-service-host.md` and `openapi.json`. Re-ran `mdbook build docs/` before and after: exit `101` both times, the same two pre-existing errors (`deployment/docker.md:118`, `user-guides/tool-integration.md:324`), no new error naming `sidecar.md`.
- Annotated all seven Milestone 12 Epic 1/3/4/5 documents (prd + tasks where applicable) with a dated `ADR-0037` correction banner and a superseded-note appended after every unprefixed route occurrence, using a pure line-insertion technique so `git diff` shows zero deleted lines across the whole `.project/Milestone_12-Web-API/` directory.
- Recorded route sites found outside D-11's four named Epics (below), per the plan's instruction not to absorb them into this edit set.

## Task Commits

Each task was committed atomically:

1. **Task 1: Author ADR-0037 — the agent route surface is /v1** - `6d3710f` (docs)
2. **Task 2: Correct the one live route defect in sidecar.md** - `64446bd` (docs)
3. **Task 3: Annotate the four Milestone 12 Epic documents as superseded provenance** - `c7f85d4` (docs)

**Plan metadata:** (this commit, made after this SUMMARY)

## Files Created/Modified

- `.planning/decisions/0037-agent-route-surface-v1.md` - New ADR recording `/v1` as the agent route surface answer
- `docs/src/deployment-topologies/sidecar.md` - One-line route correction, `POST /agents/{id}/execute` → `POST /v1/agents/{id}/execute`
- `.project/Milestone_12-Web-API/Epic_1/prd-agent-registry-execution-api.md` - Dated banner + 18 inline superseded-notes (19 sites)
- `.project/Milestone_12-Web-API/Epic_1/tasks-agent-registry-execution-api.md` - Dated banner + 8 inline superseded-notes
- `.project/Milestone_12-Web-API/Epic_3/prd-streaming-async-execution.md` - Dated banner + 8 inline superseded-notes
- `.project/Milestone_12-Web-API/Epic_3/tasks-streaming-async-execution.md` - Dated banner + 4 inline superseded-notes
- `.project/Milestone_12-Web-API/Epic_4/prd-api-cross-cutting-concerns.md` - Dated banner + 1 inline superseded-note
- `.project/Milestone_12-Web-API/Epic_4/tasks-api-cross-cutting-concerns.md` - Dated banner + 3 inline superseded-notes
- `.project/Milestone_12-Web-API/Epic_5/prd-api-security-authorization.md` - Dated banner (appended below Phase 8's existing banner) + 4 inline superseded-notes

## Decisions Made

- **Additive-only annotation, not in-place strikethrough.** Phase 8's precedent for this corpus (the `current-exports.txt` DEBT-01 correction) used `~~struck~~` + a replacement paragraph in place, which git records as a line modification (deletion + insertion). This plan's acceptance criteria requires `git diff -- .project/Milestone_12-Web-API/ | grep -c '^-[^-]'` → `0` across all seven files combined. Because Epics 1/3/4/5 have far denser route-text repetition than Phase 8's single-occurrence DEBT-01 case (up to 19 sites in one document), an in-place strikethrough pattern would have produced dozens of "deleted" lines and failed that gate. Instead, every annotation is a brand-new line inserted immediately after the original (never touching the original line's bytes), which git correctly reports as pure addition. The original text stays completely untouched — an even stronger form of "retained" than strikethrough-wrapping it.
- **A document that already carries a Phase 8 banner gets the ADR-0037 banner appended below it, dated separately** — applied to Epic 1's prd and Epic 5's prd, per D-00c/D-00f. Neither banner was merged into the other.
- **Where the same requirement's route text appears in both a `prd-` and a `tasks-` file, each file got its own banner** (Epic 1, Epic 3, Epic 4) — never a shared banner, per D-00f.

## Deviations from Plan

None — plan executed exactly as written. The ADR content, the sidecar.md fix, and the annotation pattern all followed the plan's `<action>` instructions; the only implementation choice made at the plan's stated discretion ("exact banner wording and inline markup are at your discretion") was the additive-only (new-line) annotation mechanic described above, chosen specifically to satisfy the plan's own zero-deletion acceptance criterion.

## Route Sites Found Outside D-11's Four Named Epics

Per task 3's instruction, the following unprefixed-route sites were located by the same grep but are **not** part of this plan's edit set — recorded here rather than silently absorbed:

- **`.project/Milestone_12-Web-API/Epic_2/prd-configurable-web-host-server-binary.md`** — lines 26, 42, 56, 76, 120, 150, 209, 256, 257, 271. Genuine additional unprefixed-route defect sites, not Epic 6's winning position (Epic 2 predates Epic 6's `/v1` versioning decision, same as Epics 1/3/4/5).
- **`.project/Milestone_12-Web-API/Epic_2/tasks-configurable-web-host-server-binary.md`** — lines 19, 80. Same disposition as above.
- **`.project/Milestone_12-Web-API/overview/Milestone-12_Web-API.md`** — lines 45, 99, 170, 176. Same disposition — the milestone overview document also predates and disagrees with Epic 6's versioning decision.
- **`.project/Milestone_12-Web-API/Epic_6/prd-openapi-spec-interactive-docs.md:115,250` and `Epic_6/tasks-openapi-spec-interactive-docs.md:65,83`** — these are **not** defects. Epic 6 already uses the `/v1`-prefixed form throughout (`/v1/agents/{id}/execute[/stream]`, `/v1/agents/{id}/jobs[/{job_id}]`) — this is the winning position being correctly quoted, matching what shipped.

These Epic 2 and overview-document sites are candidates for a future phase's ORCH-03-shaped cleanup; ORCH-03 itself is bounded to the five items D-09 named and is not grown by this observation (D-09).

## Issues Encountered

None. `mdbook`, `mdbook-mermaid`, and `mdbook-linkcheck` were pre-installed in this environment at the versions `.github/workflows/docs.yml` pins; running `mdbook-mermaid install docs/` before `mdbook build docs/` (matching the CI workflow's own step order) was needed once per build since the mermaid asset files are gitignored and regenerated at build time, not committed.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- The agent route surface question is closed with one recorded, cited answer (ADR-0037) that Phase 14 (`WEB-01`) and Phase 15 (`PIPE-01`) can cite directly rather than re-deriving.
- The one live documentation defect this phase's own `<verification>` block cared about is closed and re-verified against the pre-phase `mdbook build` baseline.
- No blockers for downstream plans in this wave (13-09, which touches `http-service-host.md`/`overview.md` behind its own blocking human checkpoint, is unaffected by this plan's edits — this plan explicitly did not touch either file).

---
*Phase: 13-milestone-9-12-ground-truth-recorded-account*
*Completed: 2026-08-10*
