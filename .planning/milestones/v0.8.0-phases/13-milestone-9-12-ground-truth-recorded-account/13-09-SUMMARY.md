---
phase: 13-milestone-9-12-ground-truth-recorded-account
plan: 09
subsystem: docs
tags: [adr, agent-provisioner, garrison, arsenal, deployment-topologies, mdbook]

# Dependency graph
requires:
  - phase: 13-milestone-9-12-ground-truth-recorded-account
    provides: "13-01 (context), 13-04 and 13-05 (the requirement and validation groundwork the ORCH-04 done-when criteria cite)"
provides:
  - "ADR-0038: AgentProvisioner placement ratified (stays in paladin-web), conforms"
  - "ADR-0039: absence of Garrison/Arsenal on HTTP-served agents ratified as permanent, must change (executed same plan)"
  - "http-service-host.md corrected: no longer promises tools+memory, states the limitation in prose"
  - "overview.md states the same limitation where a reader chooses a topology, links to embedded-library.md"
  - "PROMOTION.md Part B candidate 9 disposed explicitly: not promoted this phase, redirected to Phase 14"
affects: [13-13-close-out]

# Tech tracking
tech-stack:
  added: []
  patterns:
    - "ADR promotion following ADR-0015/ADR-0031/ADR-0036's three-sub-decision shape (enforceable invariant / measured baseline / conformance verdict), applied here to two Milestone 12 seams"

key-files:
  created:
    - .planning/decisions/0038-agent-provisioner-placement.md
    - .planning/decisions/0039-http-topology-no-garrison-no-arsenal.md
    - .planning/phases/13-milestone-9-12-ground-truth-recorded-account/13-09-SUMMARY.md
  modified:
    - docs/src/deployment-topologies/http-service-host.md
    - docs/src/deployment-topologies/overview.md

key-decisions:
  - "D-14 ratified as written: AgentProvisioner stays in paladin-web because AgentSpec is an OpenAPI-annotated HTTP request DTO (utoipa::ToSchema) and paladin-ports carries no utoipa, against ADR-0015 Decision (i)"
  - "D-15 ratified as permanent: absence of Garrison/Arsenal on HTTP-served agents is a permanent property of the shipped topology, published in the routing matrix, capability not scheduled"
  - "PROMOTION.md Part B candidate 9 (the M9 Epic 4 agent/orchestrator bridge decision) is NOT promoted this phase; its 'Owner phase: Phase 13' assignment is redirected to Phase 14, per 13-RESEARCH.md's own recommendation (WEB-01/WEB-02 already sit in the same M9 Epic 4/5 neighborhood as candidate 10, itself owned by Phase 14)"
  - "All three dispositions obtained from a human operator via the runtime's AskUserQuestion interactive mechanism during the orchestrator session, relayed to this continuation agent as a resolved checkpoint — recorded in full under Checkpoint Status below, per Phase 12's plan 12-01 provenance convention (D-00i)"

requirements-completed: [ORCH-04]

coverage:
  - id: D1
    description: "Blocking checkpoint (D-14, D-15, PROMOTION.md candidate 9) resolved by a human before any ADR was authored, with the outcome and how it was obtained recorded verbatim in this summary"
    requirement: "ORCH-04"
    verification:
      - kind: other
        ref: "Checkpoint Status section below, recording option, date and provenance mechanism"
        status: pass
    human_judgment: false
  - id: D2
    description: "ADR-0038 and ADR-0039 exist with the seven required H2 headings, no frontmatter, bulleted Code Locations/Considered Options, correct Code Conformance verdicts"
    requirement: "ORCH-04"
    verification:
      - kind: other
        ref: "grep -c '^## ' .planning/decisions/0038-*.md .planning/decisions/0039-*.md -> 7 and 7; awk-scoped bullet checks -> 0 prose lines both; conforms/must change verdicts confirmed"
        status: pass
    human_judgment: false
  - id: D3
    description: "http-service-host.md no longer promises tools+memory and states the limitation in prose; overview.md states the same limitation where a reader chooses a topology and links to embedded-library.md"
    requirement: "ORCH-04"
    verification:
      - kind: other
        ref: "grep -cE 'tools \\+ memory|memory \\+ tools' http-service-host.md -> 0; grep -icE 'garrison|arsenal' both files -> >=1; grep -c 'embedded-library.md' overview.md -> >=1"
        status: pass
    human_judgment: false
  - id: D4
    description: "mdbook build docs/ re-run and matches the recorded pre-phase baseline exactly (exit 101, same two pre-existing errors, no new error naming either edited file); zero .rs files touched"
    requirement: "ORCH-04"
    verification:
      - kind: other
        ref: "mdbook build docs/ (this execution, after mdbook-mermaid install docs/ regenerated the gitignored mermaid assets) -> exit 101, errors at deployment/docker.md:118 and user-guides/tool-integration.md:324 only; git diff --name-only -- '*.rs' | wc -l -> 0"
        status: pass
    human_judgment: false

duration: ~35min
completed: 2026-08-10
status: complete
---

# Phase 13 Plan 09: Milestone 12 Seam Ratification (AgentProvisioner Placement & HTTP-Topology Capability Truth) Summary

**ADR-0038 ratifies `AgentProvisioner` staying in `paladin-web` (its parameter type is an OpenAPI
request DTO, not a portable core type), ADR-0039 ratifies the absence of Garrison/Arsenal on
HTTP-served agents as a permanent topology property rather than a gap, and the routing page a reader
is sent to now tells the truth about what the HTTP host can and cannot do.**

## Performance

- **Duration:** ~35 min (continuation agent; the checkpoint itself was resolved by a prior
  orchestrator session before this agent was spawned)
- **Started:** 2026-08-10
- **Completed:** 2026-08-10
- **Tasks:** 3 of 3 (Task 1 is the blocking checkpoint, resolved before this agent started; Tasks 2
  and 3 executed and committed by this agent)
- **Files modified:** 4 (2 created — the two ADRs; 2 modified — the two doc pages)

## Accomplishments

- Recorded the human's disposition of the blocking checkpoint (D-14, D-15, PROMOTION.md Part B
  candidate 9) under Checkpoint Status below, with the provenance mechanism named per D-00i.
- Authored `.planning/decisions/0038-agent-provisioner-placement.md` — `AgentProvisioner` stays in
  `crates/paladin-web`, re-reading the trait signature, `AgentSpec`'s `utoipa::ToSchema` derive,
  `paladin-ports/Cargo.toml`'s eleven dependencies (no `utoipa`), ADR-0015 §Decision (i), and
  `FacadeProvisioner`'s `#[cfg(feature = "web-server")]` gate confirming it is the HTTP composition
  root, not a second topology. Verdict: `conforms`.
- Authored `.planning/decisions/0039-http-topology-no-garrison-no-arsenal.md` — the absence of
  Garrison and Arsenal on HTTP-served agents is a permanent property of the shipped topology, not
  planned scope. Verdict: `must change`, naming this plan's own Task 3 as executor.
- Corrected `docs/src/deployment-topologies/http-service-host.md`: the sequence diagram no longer
  reads `run (LLM + tools + memory)` (now `run (LLM + prompt)`), and a new prose callout states the
  limitation and points to `embedded-library.md`.
- Stated the same limitation in `docs/src/deployment-topologies/overview.md`, both in the
  HTTP-service-host row's `Avoid when` cell and in a note directly under the comparison table, using
  the same Garrison/Arsenal terms as `embedded-library.md` and ADR-0039 (D-00h).
- Left `embedded-library.md` untouched, per its correct existing advertisement.
- Regenerated the gitignored mermaid assets (`mdbook-mermaid install docs/`, pinned version 0.13.0
  matching `.github/workflows/docs.yml`) since this worktree started without them, then re-ran
  `mdbook build docs/` before and after the doc edits: both runs exit `101` with exactly the same two
  pre-existing errors (`deployment/docker.md:118`, `user-guides/tool-integration.md:324`) and no new
  error naming either edited file.
- Zero `.rs` files touched by this plan (`git diff --name-only -- '*.rs' | wc -l` → `0`).

## Task Commits

Each task was committed atomically:

1. **Task 1: checkpoint:decision (gate="blocking")** — resolved by a human before this agent was
   spawned; no commit at this task (the checkpoint produces no file by itself). Its outcome is
   recorded under Checkpoint Status below and folded into this SUMMARY's own commit.
2. **Task 2: Author ADR-0038 and ADR-0039** — `a1dd46a` (docs)
3. **Task 3: Correct http-service-host.md and state the limitation in overview.md** — `98f3eda` (docs)

**Plan metadata:** committed alongside this SUMMARY (worktree mode — STATE.md/ROADMAP.md excluded,
owned by the orchestrator after merge).

## Files Created/Modified

- `.planning/decisions/0038-agent-provisioner-placement.md` — new ADR, `AgentProvisioner` placement, `conforms`
- `.planning/decisions/0039-http-topology-no-garrison-no-arsenal.md` — new ADR, Garrison/Arsenal absence, `must change`
- `docs/src/deployment-topologies/http-service-host.md` — sequence-diagram capability line corrected, prose limitation added
- `docs/src/deployment-topologies/overview.md` — limitation stated in the comparison table row and a note under the table

## Decisions Made

See `key-decisions` in frontmatter, and the full Checkpoint Status record below for D-14, D-15, and
PROMOTION.md Part B candidate 9.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Regenerated gitignored `mdbook-mermaid` assets before the baseline build could run**
- **Found during:** Task 3, pre-edit baseline `mdbook build docs/` run
- **Issue:** `docs/mermaid.min.js` and `docs/mermaid-init.js` are gitignored, generated-at-build-time
  files (`.gitignore:19-23`: "MDBook mermaid generated assets (re-generated at build time via
  `mdbook-mermaid install`)"). This worktree had neither file, so the first `mdbook build docs/`
  attempt failed immediately with "Unable to copy... No such file or directory" — a different
  failure from the two-broken-link baseline 13-RESEARCH.md recorded, and one that would have
  produced a false "new error" reading if left unaddressed.
- **Fix:** Ran `mdbook-mermaid install docs/` (binary present at the exact pinned version
  `.github/workflows/docs.yml` uses, `0.13.0`) to regenerate the two gitignored asset files. This
  produced no `git status` change (both files are gitignored) and is exactly the "install it offline
  at `.github/workflows/docs.yml`'s pinned versions" escape hatch this plan's own Task 3 action names
  for a missing/broken toolchain.
- **Files modified:** none tracked (`docs/mermaid.min.js`, `docs/mermaid-init.js` are gitignored,
  confirmed via `git status --short docs/` showing no output after the install).
- **Verification:** re-ran `mdbook build docs/` after the install; it now reaches the linkcheck
  renderer and reproduces exactly the two pre-existing errors 13-RESEARCH.md recorded, confirmed
  identical before and after the doc edits.
- **Committed in:** not applicable (no tracked file changed by this fix).

---

**Total deviations:** 1 auto-fixed (1 blocking — a missing generated-asset toolchain step, not a
content or scope change). No `.rs` file touched, no prohibited documentation page touched, no fourth
ADR authored.
**Impact on plan:** None on scope or content. The fix was necessary to obtain a comparable mdbook
baseline at all; without it, the pre-edit and post-edit builds could not have been compared honestly.

## Issues Encountered

None beyond the mermaid-asset regeneration documented above.

## User Setup Required

None - no external service configuration required.

## Checkpoint Status

**RESOLVED before this agent was spawned.** Task 1 (`type="checkpoint:decision" gate="blocking"`)
opens this plan. A prior executor reached it and returned without committing anything (its worktree
was reclaimed); this continuation agent was spawned fresh with the human's decision already in hand,
per this plan's checkpoint-resolution handoff.

**Decision: `option-a` — the recommended package — was selected for all three items.**

1. **D-14 — RATIFIED AS WRITTEN.** `AgentProvisioner` stays in `crates/paladin-web`. The "promote to
   `paladin-ports` when a second consumer appears" default is retired by denying its premise:
   `AgentSpec` derives `utoipa::ToSchema` and is documented as sent in the body of `POST /agents`;
   `paladin-ports` carries no `utoipa`; ADR-0015 §Decision (i) bars web-framework dependencies from
   `paladin-core`/`paladin-ports`; and `FacadeProvisioner` is `#[cfg(feature = "web-server")]`-gated
   — the HTTP composition root, not a second topology.
2. **D-15 — RATIFIED AS PERMANENT.** The absence of Garrison and Arsenal on HTTP-served agents is a
   permanent property of the shipped topology, not planned scope. The capability limitation is
   written into published documentation (this plan's Task 3) and the capability is explicitly NOT
   scheduled.
3. **PROMOTION.md Part B candidate 9 — NOT PROMOTED this phase, with a named future owner.** The
   Milestone 9 Epic 4 agent/orchestrator bridge decision
   (`Milestone_9/Epic_4/prd-agent-orchestrator-bridge.md` §6.1) is not authored as a fourth ADR this
   phase — D-20's locked three-ADR allocation (0037, 0038, 0039) stays intact, and no ORCH
   requirement's `Derives` list reaches this PRD section, so there is no requirement-level mandate to
   promote it here. PROMOTION.md's own "Owner phase: Phase 13" assignment is redirected to **Phase
   14**, per 13-RESEARCH.md's own recommendation: WEB-01 and WEB-02 (Phase 14's opaque-token and
   shared-store work) already sit in the same Milestone 9 Epic 4/5 neighborhood as PROMOTION.md
   candidate 10 (the opaque-bearer-token decision, itself owned by Phase 14), so grouping candidate 9
   with candidate 10 under the same owner phase is the natural fit rather than a redirection with no
   rationale. **This disposition is recorded here explicitly, per this plan's own resume-signal
   requirement and T-13-27's mitigation** — plan 13-13's advancing note is expected to cite this
   record when it states candidate 9's disposition in `PROMOTION.md` itself (D-20). This plan does
   **not** edit `PROMOTION.md` — that file is out of this plan's declared `files_modified` and is
   explicitly reserved to plan 13-13.

**Options shown to the human (retained verbatim as the audit record):**

- **option-a — Ratify D-14 and D-15 as written; record candidate 9 as not promoted this phase with a
  named future owner (recommended, selected).** Pros: matches the locked decisions and the evidence;
  keeps the phase's edit surface at three documentation lines and zero `.rs`; candidate 9's
  disposition is stated rather than silent. Cons: two costly decisions are ratified on documentary
  evidence rather than a design review; candidate 9's "Owner phase: Phase 13" assignment is
  redirected rather than honoured.
- **option-b — Overturn D-14 — promote `AgentProvisioner` to `paladin-ports`.** Pros: makes the trait
  reusable by a non-HTTP topology without depending on the HTTP adapter crate. Cons: requires
  splitting `AgentSpec`, moving two more types and deprecating a public re-export across two
  published crates — `.rs` work outside this phase's D-19 boundary.
- **option-c — Overturn D-15 — make Garrison/Arsenal for HTTP-served agents planned scope with a
  target.** Pros: closes the capability gap rather than publishing it. Cons: `AgentSpec` has no
  fields for memory or tools; expressing an MCP server, its credentials and its lifetime in a JSON
  request body is genuine API design no milestone has scheduled.
- **option-d — Promote PROMOTION.md Part B candidate 9 as a fourth ADR (0040) this phase.** Pros:
  honours `PROMOTION.md`'s own "Owner phase: Phase 13" assignment and closes the inventory entry.
  Cons: widens D-20's locked three-ADR allocation; no ORCH requirement's `Derives` list reaches the
  PRD section, so the promotion has no requirement behind it.

**How the decision was obtained (provenance, per D-00i):** obtained interactively from the human
operator during the `/gsd-execute-phase 13` orchestrator session on 2026-08-10, using the runtime's
`AskUserQuestion` interactive prompt. The three items were presented to the operator as three
separate single-select questions (D-14; D-15; PROMOTION.md Part B candidate 9), each carrying the
executor's evidence and the trade-offs of both dispositions listed above. The operator selected the
recommended option on each of the three. This is a **relayed human decision**, not an agent inference
and not an auto-approval: auto-mode was off for that orchestrator run (`workflow._auto_chain_active`
= `false`; `check auto-mode --pick active` = `false`), and the orchestrator was explicitly instructed
not to self-approve this gate. **This bullet exists because a record of the mechanism, not merely the
outcome, is what lets a future reader tell a real ratification from an asserted one without trusting
prose** — the same reasoning Phase 12's plan 12-01 recorded for its own checkpoint resolution
(`.planning/phases/12-supply-chain-gate-integrity/12-01-SUMMARY.md` §Checkpoint Status).

**What was not at risk either way:** the plan's `files_modified` boundary (the two ADRs and the two
doc pages), the D-19 zero-`.rs` boundary, and `embedded-library.md`'s untouched state — none of that
depended on this decision.

**Awaiting:** nothing further from this checkpoint. Tasks 2 and 3 executed against `option-a` as
written, in this plan's own execution.

## Next Phase Readiness

- ORCH-04's done-when criteria are satisfied: both ADRs exist with the correct shape and verdicts,
  both doc pages are corrected/updated, and PROMOTION.md Part B candidate 9's disposition is recorded
  (in this SUMMARY) for plan 13-13's close-out to cite.
- Plan 13-13 (the close-out) is unblocked to advance `PROMOTION.md`'s next-free line to `0040` and
  write its own dated advancing note citing `0037-agent-route-surface-v1.md`, `0038-*`, `0039-*`, and
  explicitly stating candidate 9's redirection to Phase 14, per this record.
- No blockers for sibling wave-3 plans; this plan's `files_modified` boundary was held exactly
  (`.planning/decisions/0038-*.md`, `.planning/decisions/0039-*.md`,
  `docs/src/deployment-topologies/http-service-host.md`,
  `docs/src/deployment-topologies/overview.md` — nothing else touched).

---
*Phase: 13-milestone-9-12-ground-truth-recorded-account*
*Completed: 2026-08-10*
