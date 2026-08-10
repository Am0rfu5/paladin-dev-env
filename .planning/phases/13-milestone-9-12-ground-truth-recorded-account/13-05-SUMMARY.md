---
phase: 13-milestone-9-12-ground-truth-recorded-account
plan: 05
subsystem: docs
tags: [ledger, agent-registry, agent-provisioner, sse-streaming, async-jobs, api-error-envelope, rate-limiting, openapi-v1]

# Dependency graph
requires:
  - phase: 13-milestone-9-12-ground-truth-recorded-account
    provides: "13-01's ledger scaffold — head notes, legend, ledger-file contention table fixing this plan's disjoint 19-row section range (Milestone 12 Epics 1-4)"
provides:
  - "19 cited, re-derived Verdict cells in .planning/ledgers/milestone-09-12.md (Milestone 12 Epics 1-4)"
  - "The route-surface fact set ADR-0037 (plan 13-08) will cite: /v1 prefix confirmed against openapi.json and openapi.rs's own spec_paths_are_versioned_under_v1 test, Epic 1/3's unprefixed route text marked superseded provenance"
  - "The AgentProvisioner placement fact set ADR-0038 (plan 13-09) will cite: AgentSpec's utoipa::ToSchema derive, the sole FacadeProvisioner implementation, the missing utoipa dependency in paladin-ports — recorded without stating the placement answer"
  - "REQ-registry-from-config-builder's non-goal pointed at ADR-0039 (plan 13-09) without stating the Garrison/Arsenal answer"
  - "The check-api-surface.sh:6 baseline-path finding closed at REQ-agent-registry, documentation-only residue handed to Phase 15"
affects: [13-06, 13-08, 13-09, 13-13]

# Tech tracking
tech-stack:
  added: []
  patterns: [re-derivation with a command actually run per row (D-00e), cross-plan pointer without stating a gated ADR's answer, single-line owner-note rewording to keep git-diff line-count balance]

key-files:
  created: []
  modified: [.planning/ledgers/milestone-09-12.md]

key-decisions:
  - "All 19 rows re-derived with fresh file:line citations plus a named exerciser — no ingest-era bare-Shipped status word carried forward, per D-03; every Epic 4 row in particular arrived bare and was fully re-derived"
  - "REQ-agent-execute-endpoint and REQ-agent-discovery-endpoints and REQ-sse-streaming-endpoint record Epic 1/3's unprefixed route text as 'Shipped, superseded (route text)' — the /v1 prefix is confirmed live against both crates/paladin-web/openapi.json and the shipped openapi.rs test spec_paths_are_versioned_under_v1 — with ADR-0037 named as the recorded answer, not restated here"
  - "REQ-agent-provisioner-port, REQ-concrete-agent-provisioner and REQ-registry-from-config-builder record shipped facts only (AgentSpec's utoipa::ToSchema derive at agent_registry.rs:56, the sole FacadeProvisioner implementation gated behind the web-server feature, the absent utoipa dependency in paladin-ports) and point at ADR-0038/ADR-0039 as the phase's recorded answers without stating either — both are gated on plan 13-09's blocking human checkpoint (D-14, D-15)"
  - "REQ-health-ready-endpoints cross-references REQ-k8s-manifests (plan 13-06's row) for the multi-replica in-process token-store problem, handing that correctness question to WEB-02/Phase 14 rather than resolving it here"
  - "Every row's exerciser is a command actually run this session, not merely cited: cargo test -p paladin-web --lib (117 passed), cargo test --lib --features web-server agent_host:: (10 passed), cargo test --test web_server_e2e --features web-server (3 passed) — the last of these drives a real assembled HTTP server end-to-end for execute/stream/jobs/health/auth/error-envelope in one test file"

requirements-completed: [ORCH-01]

coverage:
  - id: D1
    description: "Milestone 12 Epic 1 (Agent Registry & Execution API, 6 rows) and Epic 2 (Configurable Web Host & Server Binary, 4 rows) re-derived with fresh file:line citations, exercisers, and the /v1 route-surface + AgentProvisioner-placement fact sets ADR-0037/0038/0039 will cite"
    requirement: "ORCH-01"
    verification:
      - kind: other
        ref: "awk '/^### Milestone 12 Epic 1/,/^### Milestone 12 Epic 3/' .planning/ledgers/milestone-09-12.md | grep -c '^| REQ-' → 10; same range grep -c 'pending — plan' → 0; grep -c 'run-5 input (not yet re-derived)' → 0"
        status: pass
      - kind: unit
        ref: "cargo test -p paladin-web --lib → 117 passed; 0 failed"
        status: pass
    human_judgment: false
  - id: D2
    description: "Milestone 12 Epic 3 (Streaming & Async Jobs, 4 rows) and Epic 4 (Operational Hardening, 5 rows) re-derived; every Epic 4 row converted from a bare ingest-era Shipped status word to a cited, exercised verdict"
    requirement: "ORCH-01"
    verification:
      - kind: other
        ref: "awk '/^### Milestone 12 Epic 3/,/^### Milestone 12 Epic 5/' .planning/ledgers/milestone-09-12.md | grep -c '^| REQ-' → 9; grep -cE '\\.rs:[0-9]+' → 9"
        status: pass
      - kind: e2e
        ref: "cargo test --test web_server_e2e --features web-server → 3 passed; 0 failed (real HTTP execute/stream/jobs/health/auth/error-envelope)"
        status: pass
    human_judgment: false
  - id: D3
    description: "Ledger integrity preserved: 120 total rows unchanged, no .rs or .project/ file touched, only Milestone 12 Epics 1-4's section range modified"
    requirement: "ORCH-01"
    verification:
      - kind: other
        ref: "grep -c '^| REQ-' .planning/ledgers/milestone-09-12.md → 120; git diff --name-only -- '*.rs' '.project/*' | wc -l → 0; git diff 778a71a..HEAD -- .planning/ledgers/milestone-09-12.md hunks confined to lines 468-520"
        status: pass
    human_judgment: false

# Metrics
duration: ~50min
completed: 2026-08-10
status: complete
---

# Phase 13 Plan 05: Milestone 12 Epics 1-4 Ledger Derivation Summary

**Re-derived all 19 Milestone 12 Web-API requirement rows (Agent Registry, Web Host/Server Binary, Streaming/Async Jobs, Operational Hardening) with fresh citations, a real end-to-end HTTP test run, and the fact sets ADR-0037/0038/0039 will cite.**

## Performance

- **Duration:** ~50 min
- **Started:** 2026-08-10 (session start)
- **Completed:** 2026-08-10T17:35:51Z
- **Tasks:** 2
- **Files modified:** 1 (`.planning/ledgers/milestone-09-12.md`)

## Accomplishments

- All 10 rows in Milestone 12 Epics 1-2 re-derived: `REQ-agent-registry` through `REQ-paladin-web-no-facade-dep` (Epic 1) and `REQ-host-agents-config-schema` through `REQ-paladin-server-binary` (Epic 2), each with a fresh `file:line` citation and a named exerciser — no ingest-era transcription remains
- The `/v1` route-surface fact confirmed twice this session — against `crates/paladin-web/openapi.json`'s six agent paths and against the shipped `openapi.rs:103` test `spec_paths_are_versioned_under_v1` — with Epic 1/3's unprefixed route text recorded as `Shipped, superseded (route text)` and ADR-0037 (plan 13-08) named as the recorded answer, never restated
- The `AgentProvisioner` placement fact set recorded across three rows without stating ADR-0038's gated answer: `AgentSpec`'s `utoipa::ToSchema` derive (`agent_registry.rs:56`), its own "Sent in the body of `POST /agents`" doc comment (`:48`), the sole `FacadeProvisioner` implementation (`src/infrastructure/web/facade_provisioner.rs:70`) gated `#[cfg(feature = "web-server")]`, and the confirmed absence of `utoipa` in `paladin-ports`'s manifest
- The `check-api-surface.sh:6` finding closed at `REQ-agent-registry`: the dotted baseline path is read by the script and the baseline file exists (446 KB), so the CI-failure consequence is closed; the documentation-only residue across four Milestone 12 requirement texts is handed to Phase 15 with D-08's job list, not grown into a sixth ORCH-03 item
- All 9 rows in Milestone 12 Epics 3-4 re-derived: every Epic 4 row (error envelope, health/ready, request logging, CORS/body-limit/timeout, rate limiting) converted from a bare ingest-era `Shipped` word to a cited, exercised verdict, per D-03
- `REQ-health-ready-endpoints` cross-references `REQ-k8s-manifests` (plan 13-06's row) for the multi-replica in-process token-store problem, handing it to WEB-02/Phase 14 rather than resolving it here
- A real end-to-end HTTP test suite, `tests/web_server_e2e.rs`, actually run this session (`cargo test --test web_server_e2e --features web-server` → `3 passed; 0 failed`), assembling the exact application the `paladin-server` binary assembles and driving execute/stream/jobs/health/auth/error-envelope over real HTTP against an ephemeral port — the single strongest exerciser across all 19 rows

## Task Commits

1. **Task 1: Derive Milestone 12 Epics 1-2 (10 rows) with the route-surface and provisioner pointers** - `dcdc1c9` (docs)
2. **Task 2: Derive Milestone 12 Epics 3-4 (9 rows)** - `1af8b36` (docs)

## Files Created/Modified

- `.planning/ledgers/milestone-09-12.md` - 19 Verdict cells replaced in place across Milestone 12 Epics 1-4 (lines 469-517); no row inserted, deleted, or reordered

## Decisions Made

- All 19 rows re-derived with fresh `file:line` citations plus a named exerciser — no ingest-era bare-`Shipped` status word carried forward (D-03), including all 5 Epic 4 rows which arrived bare
- Route text in Epic 1 (`REQ-agent-execute-endpoint`, `REQ-agent-discovery-endpoints`) and Epic 3 (`REQ-sse-streaming-endpoint`) recorded as `Shipped, superseded (route text)` — the `/v1` prefix is what shipped (confirmed against `openapi.json` and `openapi.rs`'s own drift-guard test), and ADR-0037 (plan 13-08) is named as the recorded answer, never restated in this ledger
- The three provisioner-adjacent rows (`REQ-agent-provisioner-port`, `REQ-concrete-agent-provisioner`, `REQ-registry-from-config-builder`) record shipped facts only and point at ADR-0038/ADR-0039 without stating either answer — both are gated on plan 13-09's blocking human checkpoint (D-14, D-15); pre-empting either would launder an unratified decision into the ledger
- `REQ-health-ready-endpoints` cross-references `REQ-k8s-manifests` rather than re-deriving the multi-replica token problem, keeping WEB-02 as the single owner of that open question
- Every exerciser cited is a command actually run this session, not merely a static reference: `cargo test -p paladin-web --lib` (117 passed), `cargo test --lib --features web-server agent_host::` (10 passed), `cargo test --test web_server_e2e --features web-server` (3 passed)

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 1 - Bug] The Milestone 12 Epic 1 owner note was a stale false-positive against this plan's own `run-5 input` grep check**
- **Found during:** Task 1 (self-verification of the acceptance criterion `grep -c 'run-5 input (not yet re-derived)'` → `0` for the Epic 1-2 range)
- **Issue:** The section's owner note read "Rows not yet re-derived carry the source's own text prefixed `run-5 input (not yet re-derived):`" — a description of the *convention*, not a transcribed row, but its literal text matched the plan's own bare-string acceptance check, inflating the count from the true `0` to `1` even after all six Epic 1 rows were fully derived. Same defect class 13-01's and 13-02's summaries documented (a prose sentence describing a marker, not using it).
- **Fix:** Reworded the note to state the section is fully derived this session, removing the literal marker phrase while preserving the meaning.
- **Files modified:** `.planning/ledgers/milestone-09-12.md`
- **Verification:** `awk '/^### Milestone 12 Epic 1/,/^### Milestone 12 Epic 3/' .planning/ledgers/milestone-09-12.md | grep -c 'run-5 input (not yet re-derived)'` → `0`
- **Committed in:** `dcdc1c9` (part of the task 1 commit — fixed before committing)

**2. [Rule 1 - Bug] The Milestone 12 Epic 4 owner note initially grew the file's added/removed line-count imbalance**
- **Found during:** Task 2 (self-verification of the acceptance criterion comparing `git diff --numstat`'s added/removed counts)
- **Issue:** The first rewording of the Epic 4 owner note (marking it fully derived, same fix as issue 1 above) spanned three lines replacing the original single line, adding two net lines to the file and breaking the intended added==removed balance that proves this plan performed cell-replacement only, never row insertion.
- **Fix:** Condensed the reworded note to a single line, restoring exact line-count balance (10 added / 10 removed for the task 2 diff) while keeping the same substantive content.
- **Files modified:** `.planning/ledgers/milestone-09-12.md`
- **Verification:** `git diff --numstat -- .planning/ledgers/milestone-09-12.md` → `10\t10\t...` before committing task 2.
- **Committed in:** `1af8b36` (part of the task 2 commit — fixed before committing)

---

**Total deviations:** 2 auto-fixed (both Rule 1 — prose-counting bugs caught by this plan's own self-verification against its acceptance criteria before each commit)
**Impact on plan:** Both were caught and corrected inline before their respective task commits; no scope creep, no downstream plan needs to redo work.

### Documented, not auto-fixed: a bug in the plan's own acceptance-criteria wording

**Task 2's `git diff --numstat` balance check compares the wrong fields.** The plan's acceptance criterion reads `git diff --numstat -- .planning/ledgers/milestone-09-12.md | awk '{print ($2==$3)?"balanced":"unbalanced"}'` → `balanced`. `git diff --numstat` emits `<added>\t<removed>\t<path>` — so `$2` is the removed-line count and `$3` is the file path string; comparing a number to a path string can never be numerically true and the check as literally written always prints `unbalanced`, regardless of whether the underlying diff is actually balanced. Re-running the same command directly against this plan's task-2 diff shows the real added/removed counts are `10`/`10` — genuinely balanced, matching the plan's own intent (cell replacement only, no row insertion). This is the same class of self-referential arithmetic slip this phase's own D-04/D-08/D-09/D-17/D-18 findings catch elsewhere in the corpus, and the same class of plan-acceptance-criteria discrepancy 13-01's and 13-03's summaries documented rather than silently "fixing" by reinterpreting the check. **Not auto-fixed** because there is no ledger defect underlying it — the true `$1`/`$2` comparison passes; flagging the check's own field-index bug here is the correct action per this plan's own evidence-bar mandate, not a change to the ledger.

## Issues Encountered

Cold-compile latency: the workspace's first `cargo test` invocation this session took ~2m50s (full dependency graph recompilation, consistent with the known worktree pre-commit cold-build behavior). All test commands were re-run to completion and their output recorded verbatim in the ledger rows rather than inferred from a partial or timed-out run.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- Plan 13-06 (Milestone 12 Epics 5-7) can proceed independently — its section range (lines 518+) was not touched by this plan, and `REQ-k8s-manifests` (its own row) is cross-referenced from this plan's `REQ-health-ready-endpoints` row rather than duplicated.
- Plan 13-08 (ADR-0037 + the four M12 Epic route-text `.project/` annotations + the `sidecar.md:29` fix) has its full evidence base recorded in this plan's rows: the `/v1` prefix confirmed against both `openapi.json` and the shipped `openapi.rs` drift-guard test, plus the exact superseded-provenance framing for Epic 1 and Epic 3's route text.
- Plan 13-09 (ADR-0038 + ADR-0039, gated on a blocking `checkpoint:decision`) has its full fact base recorded across three rows (`REQ-agent-provisioner-port`, `REQ-concrete-agent-provisioner`, `REQ-registry-from-config-builder`) without either placement answer being stated — the checkpoint remains the sole place either decision is made.
- No blockers. All 19 rows carry a `file:line` citation plus a command actually run this session; `grep -c '^| REQ-'` on the whole ledger remains `120` before and after, and no `.rs` or `.project/` file was touched.

## Self-Check: PASSED

- `test -f .planning/phases/13-milestone-9-12-ground-truth-recorded-account/13-05-SUMMARY.md` → FOUND
- `git log --oneline --all | grep dcdc1c9` → FOUND (Task 1 commit)
- `git log --oneline --all | grep 1af8b36` → FOUND (Task 2 commit)

---
*Phase: 13-milestone-9-12-ground-truth-recorded-account*
*Completed: 2026-08-10*
