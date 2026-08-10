---
phase: 13-milestone-9-12-ground-truth-recorded-account
plan: 06
subsystem: docs
tags: [ledger, requirements-traceability, jwt, auth, openapi, kubernetes, adr]

# Dependency graph
requires:
  - phase: 13-milestone-9-12-ground-truth-recorded-account
    provides: "13-01's ledger scaffold (head notes, vocabulary, 120 row stubs) that this plan appends to"
provides:
  - "15 cited Verdict cells for Milestone 12 Epics 5-7 (API Security & Authorization, OpenAPI & Interactive Docs, Deployment Artefacts) in .planning/ledgers/milestone-09-12.md"
  - "The JWT-vs-opaque-token contradiction recorded as Contract diverges, handed to Phase 14 / WEB-01 unresolved"
  - "The multi-replica in-process-token-store correctness problem recorded as Verified open, handed to Phase 14 / WEB-02"
  - "The vacuous three-open-checkbox evidence for Milestone 12 (D-10), quoted verbatim from the task file"
  - "A fresh finding: no v0.6.0 git tag exists despite the lockstep version-bump commits and CHANGELOG entry"
affects: [phase-14-api-contract-truthfulness, plan-13-08, plan-13-09, plan-13-12]

# Tech tracking
tech-stack:
  added: []
  patterns: ["D-00e evidence bar applied to all 15 rows: exact command or file:line re-derived this session, never carried forward from an ingest status word"]

key-files:
  created: []
  modified:
    - .planning/ledgers/milestone-09-12.md

key-decisions:
  - "REQ-jwt-bearer-auth-v2 recorded as Contract diverges -> Phase 14 / WEB-01: agent_auth.rs documents JWT throughout but the only shipped AuthPort implementation is the opaque in-memory token adapter; no jsonwebtoken dependency exists anywhere in the workspace. Neither variant is picked."
  - "REQ-k8s-manifests recorded as Shipped with the multi-replica correctness question Verified open, handed to Phase 14 / WEB-02: k8s/server/deployment.yaml ships replicas: 2 with live health/ready probes against an in-process token store that cannot be shared across pods."
  - "REQ-fail-closed-auth-posture recorded as Verified open, not Shipped: the refusal-to-start code path exists in src/bin/paladin-server.rs but has zero test coverage (it is a bin with no test module)."
  - "Distinguished the real M12 Epic 7 artefact (k8s/server/) from a pre-existing, unrelated top-level k8s/deployment.yaml placeholder manifest (probes commented out, testing-only image) that an earlier reading could have conflated with the shipped deliverable."
  - "Surfaced that no v0.6.0 git tag exists despite the CHANGELOG entry and lockstep-bump commits -- recorded as a fact for plan 13-12 to cite accurately in ADR-0029's trajectory table, not resolved here."

requirements-completed: [ORCH-01, ORCH-02]

coverage:
  - id: D1
    description: "All 6 Milestone 12 Epic 5 (API Security & Authorization) ledger rows carry a cited verdict; REQ-jwt-bearer-auth-v2 is recorded Contract diverges -> WEB-01 and the vacuous-checkbox evidence is quoted verbatim"
    requirement: "ORCH-01"
    verification:
      - kind: other
        ref: "awk '/^### Milestone 12 Epic 5/,/^### Milestone 12 Epic 6/' .planning/ledgers/milestone-09-12.md | grep -c '^| REQ-' -> 6; grep -c 'pending — plan' -> 0"
        status: pass
    human_judgment: false
  - id: D2
    description: "All 9 Milestone 12 Epic 6-7 ledger rows carry a cited verdict; REQ-k8s-manifests hands the multi-replica question to WEB-02 and REQ-deployment-topology-doc-update records the sidecar.md/http-service-host.md pointers without pre-empting plans 13-08/13-09"
    requirement: "ORCH-01"
    verification:
      - kind: other
        ref: "awk '/^### Milestone 12 Epic 6/,/^### Deferred-QA Epic 25/' .planning/ledgers/milestone-09-12.md | grep -c '^| REQ-' -> 9; grep -c 'k8s/deployment.yaml' -> 2; grep -c 'sidecar.md:29' -> 1"
        status: pass
    human_judgment: false
  - id: D3
    description: "Ledger remains at exactly 120 rows and no .rs/.project/ file was modified"
    requirement: "ORCH-01"
    verification:
      - kind: other
        ref: "grep -c '^| REQ-' .planning/ledgers/milestone-09-12.md -> 120; git diff --name-only -- '*.rs' 'docs/*' '.project/*' | wc -l -> 0"
        status: pass
    human_judgment: false

duration: ~50min
completed: 2026-08-10
status: complete
---

# Phase 13 Plan 06: Milestone 12 Epics 5-7 Ledger Derivation Summary

**Re-derived 15 Milestone 12 ledger rows (API Security, OpenAPI Docs, Deployment Artefacts) with fresh file:line evidence, recording the JWT-vs-opaque contradiction and the multi-replica token-store gap as unresolved hand-offs to Phase 14.**

## Performance

- **Duration:** ~50 min
- **Completed:** 2026-08-10T17:54:48Z
- **Tasks:** 2
- **Files modified:** 1 (`.planning/ledgers/milestone-09-12.md`)

## Accomplishments

- Derived all 6 Milestone 12 Epic 5 (API Security & Authorization) rows at the D-00e evidence bar — every claim re-run this session, none carried forward from the run-5 transcription or left as `pending`.
- Recorded `REQ-jwt-bearer-auth-v2` as `Contract diverges → Phase 14 / WEB-01`: `crates/paladin-web/src/agent_auth.rs` documents its verifier as JWT throughout, but `grep -rn 'jsonwebtoken' Cargo.toml crates/*/Cargo.toml` returns nothing and the only shipped `AuthPort` implementation (`src/infrastructure/adapters/auth/in_memory_token_auth_adapter.rs`, wired by `src/bin/paladin-server.rs:174-175`) is the opaque, in-process, hashed-token store Milestone 9 chose specifically to avoid a `jsonwebtoken` dependency. Epic 5's own Open Question 4 is recorded unanswerable for the shipped adapter. Neither mechanism is picked; the row hands the decision to WEB-01.
- Recorded the vacuous three-open-checkbox evidence (D-10) in the Epic 5 header note, quoting all three unchecked items verbatim from `.project/Milestone_12-Web-API/Epic_5/tasks-api-security-authorization.md:43-45` and confirming every other item in the file is `[x]`.
- Recorded `REQ-fail-closed-auth-posture` as `Verified open` rather than `Shipped`: the refusal-to-start branch in `src/bin/paladin-server.rs:145-192` `build_auth_config` exists and matches the requirement's shape, but no test exercises it — `paladin-server` is a `bin` with no `#[cfg(test)]` module, and the "spawn-the-binary" e2e mode is Epic 7's own stated non-goal.
- Derived all 9 Milestone 12 Epic 6-7 rows. `REQ-api-v1-versioning`/`REQ-openapi-drift-guard` re-confirmed all six agent paths in `crates/paladin-web/openapi.json` carry the `/v1` prefix, naming ADR-0037 (plan 13-08) as the recorded answer.
- **Distinguished two different Kubernetes manifests that could otherwise be conflated:** the top-level `k8s/deployment.yaml` is a pre-existing, unrelated placeholder (image `paladin:test`, all probes commented out "Disabled for testing"), while the real Milestone 12 Epic 7 artefact is `k8s/server/deployment.yaml` (`replicas: 2`, live `/health`/`/ready` probes). Recorded `REQ-k8s-manifests` as `Shipped, correctness question open → Phase 14 / WEB-02`: multiple replicas each run their own in-process token store, so a token issued by one pod will not verify on another — a correctness question, not a scaling optimization, handed to WEB-02 without proposing a fix or a `replicas: 1` pin.
- Recorded `REQ-deployment-topology-doc-update`'s two live documentation defects (`sidecar.md:29`'s unprefixed route, `http-service-host.md:54`'s tools/memory promise) as pointers only, naming plans 13-08 and 13-09 respectively without stating either plan's answer — 13-09's is gated on a blocking human checkpoint.
- **Fresh finding on `REQ-m12-v060-release`:** re-ran `git tag --list 'v0.6*'` and found no `v0.6.0` tag exists in the tree, despite the lockstep version-bump commits (`90ca591`, `67b6207`, `23b187b`) and the `CHANGELOG.md:139-164` `[0.6.0]` entry — contrasted against `v0.4.0`-`v0.4.3` and `v0.5.0`/`v0.5.1`, which do exist. Recorded as evidence for plan 13-12 to cite accurately (commits, not a tag) in ADR-0029's trajectory row; not resolved or corrected here.

## Task Commits

Each task was committed atomically:

1. **Task 1: Derive Milestone 12 Epic 5 (6 rows) with the WEB-01 contradiction and the vacuous-checkbox evidence** - `4b26a8f` (docs)
2. **Task 2: Derive Milestone 12 Epics 6-7 (9 rows) with the WEB-02 hand-off** - `7e9b1ec` (docs)

_No TDD tasks in this plan (record-writing only, D-19)._

## Files Created/Modified

- `.planning/ledgers/milestone-09-12.md` - Verdict cells for 15 Milestone 12 requirement IDs (Epics 5, 6, 7), plus the Epic 5 header note's vacuous-checkbox evidence

## Decisions Made

- `REQ-jwt-bearer-auth-v2` verdict class is `Contract diverges`, cross-referencing `REQ-opaque-bearer-token-adapter-v1` as a separate row per D-00f, with Phase 14 / WEB-01 as owner — no mechanism is chosen.
- `REQ-fail-closed-auth-posture` verdict class is `Verified open`, not `Shipped`, because the D-03 evidence bar requires a test for a security posture claim and none exists for the refusal-to-start branch specifically (the default-enabled config value and the middleware's 401 behavior are separately tested and separately cited).
- `REQ-k8s-manifests` cites `k8s/server/deployment.yaml` (the real M12 artefact) rather than the top-level `k8s/deployment.yaml` (an older, unrelated placeholder), and hands the multi-replica token-store question to Phase 14 / WEB-02 as `Verified open` without proposing a fix.
- `REQ-m12-v060-release` records the missing `v0.6.0` git tag as a fresh finding rather than silently transcribing the ingest-era "terminal release gate" language, since the requirement's own acceptance criteria do not name a tag explicitly and no acceptance criterion is thereby failed — but a downstream reader of ADR-0029 needs the fact.

## Deviations from Plan

None - plan executed exactly as written. The "fresh finding" on the missing `v0.6.0` tag is evidence-gathering within the plan's own D-00e/D-21 mandate ("every closure claim is proved by a command run in this environment"), not a deviation from planned scope — it is recorded in the row this plan owns and handed to plan 13-12 without touching ADR-0029.

## Issues Encountered

None. All cited commands (`cargo test -p paladin-web --lib` → 117 passed; `cargo test --test web_server_e2e --features web-server` → 3 passed; `cargo test -p paladin-web --lib openapi::` → 5 passed; the various `grep`/`git tag`/`git log` re-runs) were executed successfully in this session and matched or extended the existing ledger head note's own re-derived facts.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- All 15 Milestone 12 Epic 5-7 rows are cited and closed; the ledger's Milestone 12 section (Epics 1-7, 34 IDs total) is now fully derived across plans 13-05 and 13-06.
- Phase 14 inherits two concrete, cited, unresolved questions: WEB-01 (JWT vs. opaque token mechanism) and WEB-02 (multi-replica token-store correctness) — both recorded with named owners and no fabricated answers, ready for that phase's planning.
- Plans 13-08 and 13-09 (wave 3, parallel) can proceed independently — this plan's rows point at their defects (`sidecar.md:29`, `http-service-host.md:54`) without pre-empting either plan's fix or 13-09's blocking checkpoint.
- Plan 13-12 (ORCH-05) has the evidence it needs for the `v0.6.0` trajectory row: the three lockstep-bump commits, with an explicit note that no tag exists to cite instead.
- No blockers.

---
*Phase: 13-milestone-9-12-ground-truth-recorded-account*
*Completed: 2026-08-10*
