---
phase: 14-api-contract-truthfulness
plan: 05
subsystem: auth
tags: [adr, documentation, auth, kubernetes, decision-records]

# Dependency graph
requires:
  - phase: 14-api-contract-truthfulness
    provides: "plan 14-01's shipped token-vocabulary rename (BearerTokenAuthConfig, .bearer_token, .token_verifier, SEC_BEARER_TOKEN) and plan 14-04's shipped startup warning + deployment-artefact call-outs (IN_PROCESS_TOKEN_STORE_WARNING, k8s/server/configmap.yaml, k8s/README.md, docs/src/deployment-topologies/http-service-host.md)"
provides:
  - "ADR-0040: opaque server-issued bearer tokens ratified as the agent API's token mechanism, M12 Epic 5 Open Question 4 dissolved (not answered), the reversal cost to a signed-token AuthPort priced"
  - "ADR-0041: the shared-store requirement scoped to the AuthPort credential path (not the replica count), WEB-02's deviation from its own two literal exits stated and justified, the manifest citation corrected, the shared store deferred with a named ADR-0035-style trigger"
affects: [14-06, 14-07, 14-08]

# Tech tracking
tech-stack:
  added: []
  patterns:
    - "One-decision-per-ADR discipline (D-16), following the ADR-0035/ADR-0042 numbered-measured-facts Context style"

key-files:
  created:
    - .planning/decisions/0040-opaque-bearer-token-mechanism.md
    - .planning/decisions/0041-in-process-token-store-single-replica-scope.md
  modified: []

key-decisions:
  - "ADR-0040 states OQ-4 is dissolved, not answered, in those exact terms — an opaque hashed store has no signing secret or algorithm to configure, so the question has no referent for the shipped adapter."
  - "ADR-0041 takes neither of WEB-02's two literal 'done when' exits (pin replicas to 1, or build the shared store) and states that departure as a deliberate deviation with tree-level reasoning, per the plan's explicit prohibition against presenting it as requirement-authorised."

patterns-established:
  - "Numbered measured facts in an ADR's Context section, each citing the exact command, file:line, or PRD section that produced it (ADR-0035/ADR-0042 style), used again here for both records."

requirements-completed: [WEB-01, WEB-02]

coverage:
  - id: D1
    description: "ADR-0040 records the WEB-01 decision: opaque server-issued bearer tokens ratified, OQ-4 dissolved in those terms, the reversal cost priced"
    requirement: "WEB-01"
    verification:
      - kind: other
        ref: "grep -c '^## ' .planning/decisions/0040-opaque-bearer-token-mechanism.md == 7; grep -ci dissolved == 1; grep -c 'Open Question 4' == 2; grep -c SEC_BEARER_TOKEN == 2; grep -c BearerTokenAuthConfig == 4; grep -c conforms == 1"
        status: pass
      - kind: other
        ref: "every Code Locations path verified present via individual test -e calls (7/7 resolve)"
        status: pass
    human_judgment: false
  - id: D2
    description: "ADR-0041 records the WEB-02 decision: verification scope, the deviation from WEB-02's literal exits named and justified, the manifest citation corrected, the shared store deferred with a named trigger"
    requirement: "WEB-02"
    verification:
      - kind: other
        ref: "grep -c '^## ' .planning/decisions/0041-in-process-token-store-single-replica-scope.md == 7; grep -c ADR-0035 == 3; grep -c k8s/server/deployment.yaml == 6; grep -c k8s/server/service.yaml == 1; grep -c IN_PROCESS_TOKEN_STORE_WARNING == 1"
        status: pass
      - kind: other
        ref: "every Code Locations path verified present via individual test -e calls (9/9 resolve); git diff --exit-code -- k8s/server/deployment.yaml k8s/deployment.yaml is clean"
        status: pass
    human_judgment: false

# Metrics
duration: ~35min
completed: 2026-08-12
status: complete
---

# Phase 14 Plan 05: Auth-surface decision records (ADR-0040, ADR-0041) Summary

**Authored ADR-0040 (opaque bearer-token mechanism ratified, OQ-4 dissolved) and ADR-0041 (verification scope, WEB-02's literal-exit deviation justified, shared store deferred with an ADR-0035-style trigger), closing the two forward references plans 14-01 and 14-04 left pointing at this plan.**

## Performance

- **Duration:** ~35 min
- **Started:** 2026-08-12T16:50Z (approx, inferred from worktree dispatch context)
- **Completed:** 2026-08-12T17:27Z
- **Tasks:** 2 (Task 1: ADR-0040; Task 2: ADR-0041)
- **Files modified:** 2 (both new files)

## Accomplishments

- **ADR-0040** ratifies opaque server-issued bearer tokens as the agent API's token mechanism (D-01), built from two facts re-verified this session against the post-14-01 tree: `grep -rn "jsonwebtoken" Cargo.toml crates/*/Cargo.toml` across all twelve workspace manifests returns no matches, and the only `AuthPort` implementation in the workspace is `src/infrastructure/adapters/auth/in_memory_token_auth_adapter.rs` (SHA-256 hashing, 32 cryptographically-random bytes per token, 24h default TTL, explicit revocation). States Milestone 12 Epic 5's Open Question 4 as **dissolved, not answered** — an opaque hashed store has no signing secret or algorithm to configure — and prices the reversal cost (a `jsonwebtoken` dependency entering the audited graph, a signing-key rotation story, loss of immediate revocation).
- **ADR-0041** scopes the shared-store requirement to the `AuthPort` credential path rather than the replica count (D-06), and states that departure from WEB-02's own two literal "done when" exits explicitly as a deviation with the tree-level reasoning: the shipped `k8s/server/configmap.yaml` disables `bearer_token` verification and authenticates with byte-identical static API keys, so pinning `replicas: 1` would degrade a working two-replica deployment to guard a code path the shipped configuration has already turned off. Corrects WEB-02's own manifest citation (D-08) — the real Milestone 12 Epic 7 artefacts are `k8s/server/deployment.yaml` and `k8s/server/service.yaml`, not the root placeholder. Defers the shared-store `AuthPort` with a named trigger (D-09) following the ADR-0035 precedent: the first deployment needing more than one replica serving `AuthPort`-issued tokens.
- Both records cite only paths that resolve in the post-14-01/14-04 tree — every path in both `## Code Locations` sections was individually verified with `test -e` (7/7 for ADR-0040, 9/9 for ADR-0041) rather than carried forward from CONTEXT.md unchecked, per D-00b's precedence-order warning that an ADR contradicting shipped code is an instruction to change the code.
- Both parser-sensitive sections (`## Code Locations`, `## Considered Options`) were written as single physical lines per bullet and self-checked with `awk`/`grep -c '^- '` against the section boundaries: ADR-0040 (7 Code Locations, 4 Considered Options), ADR-0041 (9 Code Locations, 5 Considered Options) — both match the literal bullet counts exactly.

## Task Commits

1. **Task 1: ADR-0040 — opaque server-issued bearer tokens as the ratified mechanism** — `070477c` (docs)
2. **Task 2: ADR-0041 — verification scope, the deliberate deviation, and the deferred shared store** — `1fc8547` (docs)

**Plan metadata:** this SUMMARY's commit follows (worktree mode — STATE.md/ROADMAP.md are updated by the orchestrator after all wave agents complete).

## Files Created/Modified

- `.planning/decisions/0040-opaque-bearer-token-mechanism.md` — the WEB-01 decision record
- `.planning/decisions/0041-in-process-token-store-single-replica-scope.md` — the WEB-02 decision record

## Verification Evidence

**ADR parser self-check (bullet counts in the two parser-sensitive sections):**

| File | `## Code Locations` bullets | `## Considered Options` bullets |
|---|---|---|
| ADR-0040 | 7 | 4 |
| ADR-0041 | 9 | 5 |

Counted via `awk '/^## Code Locations/,/^## Code Conformance/' <file> | grep -c '^- '` and the equivalent for `## Considered Options`, confirming each bullet occupies exactly one physical line (no wrapped continuation inflating the count).

**`test -e` sweep over every path cited in `## Code Locations`:**

- ADR-0040 (7/7 resolve): `crates/paladin-web/src/openapi.rs`, `crates/paladin-web/src/agent_auth.rs`, `src/config/agents.rs`, `src/bin/paladin-server.rs`, `src/infrastructure/adapters/auth/in_memory_token_auth_adapter.rs`, `crates/paladin-web/openapi.json`, `.project/current-exports.txt`
- ADR-0041 (9/9 resolve): `src/bin/paladin-server.rs` (four separate line-range citations), `k8s/server/configmap.yaml`, `k8s/server/deployment.yaml`, `k8s/README.md`, `docs/src/deployment-topologies/http-service-host.md`, `src/infrastructure/adapters/auth/in_memory_token_auth_adapter.rs`

**Acceptance-criteria greps, both files:** `head -1` matches `^# ADR-00NN:`; `grep -c '^## '` is 7 for both; neither file opens with `---` (no frontmatter); `grep -ci dissolved` and `grep -c 'Open Question 4'` on ADR-0040 are 1 and 2 respectively; `grep -c SEC_BEARER_TOKEN`/`BearerTokenAuthConfig` on ADR-0040 are 2/4; `grep -c conforms` is 1 on both files; `grep -c ADR-0035` on ADR-0041 is 3; `grep -c k8s/server/deployment.yaml`/`k8s/server/service.yaml` on ADR-0041 are 6/1; `grep -c IN_PROCESS_TOKEN_STORE_WARNING` on ADR-0041 is 1.

**Scope guard:** `git diff --name-only <worktree-base>..HEAD` lists exactly the two new ADR files — no manifest, no code, no other planning document touched. `git diff --exit-code -- k8s/server/deployment.yaml k8s/deployment.yaml .planning/decisions/PROMOTION.md` is clean.

## Decisions Made

- Cited both PRD source documents directly (`.project/Milestone_9-.../prd-user-admin-system-completion.md` §6.1/§5/§6.2, `.project/Milestone_12-.../prd-api-security-authorization.md` FR-2/§9 OQ-4) with line numbers re-derived this session, rather than relying solely on `.planning/REQUIREMENTS.md`'s already-quoted excerpts, so a reader can trace each quoted sentence to its origin file.
- Did not edit `.planning/decisions/PROMOTION.md`'s next-free-number line — per the plan's explicit instruction, plan 14-07 is this phase's single writer of that file.
- Left `.planning/REQUIREMENTS.md`'s WEB-02 manifest-citation text untouched — ADR-0041 records the correction (D-08) but the dated correction banner on WEB-02's own text is plan 14-07's job, not this plan's `files_modified` scope.

## Deviations from Plan

None — plan executed exactly as written. Both tasks' `<action>` instructions were followed directly; no Rule 1/2/3 auto-fixes were needed since this plan only creates two new documentation files with no code or build surface to break.

## Issues Encountered

None.

## User Setup Required

None — no external service configuration required.

## Known Stubs

None. Both ADR files are complete records with no placeholder sections, TODOs, or forward references left unresolved within this plan's own scope.

## Threat Flags

None beyond the plan's own threat model, which is fully addressed by construction: T-14-18 (ADR precedence over shipped code) is mitigated by the `test -e` sweep over every cited path and re-derivation of every line number against the post-14-01/14-04 tree rather than copying from CONTEXT.md. T-14-19 (unexplained deviation reading as an oversight) is mitigated by ADR-0041's Decision section stating the departure from WEB-02's literal exits explicitly, with the tree-level reasoning for declining each. T-14-20 (Secret-value disclosure in quoted tree state) is mitigated — both records quote configuration key names and flags only (`bearer_token.enabled`, `PALADIN_API_KEY_CI`/`PALADIN_API_KEY_APP` as variable names, never a value). T-14-21 (ADR numbering collision) is mitigated — both filenames and first-line headings match the reserved numbers 0040/0041 exactly, matching `PROMOTION.md`'s recorded next-free-number line, unedited by this plan.

## Next Phase Readiness

- Both forward references left by plans 14-01 and 14-04 are now resolved: the two CHANGELOG `BREAKING` entries (`CHANGELOG.md`, `crates/paladin-web/CHANGELOG.md`) that point at ADR-0040, and the four files (`src/bin/paladin-server.rs`, `k8s/server/configmap.yaml`, `k8s/README.md`, `docs/src/deployment-topologies/http-service-host.md`) that point at ADR-0041, now resolve to real, complete decision records rather than forward pointers.
- Plan 14-07 depends on both ADRs existing to amend the `REQ-opaque-bearer-token-adapter-v1`, `REQ-jwt-bearer-auth-v2`, `REQ-k8s-manifests`, and `REQ-fail-closed-auth-posture` ledger rows against them, to advance `PROMOTION.md`'s next-free-ADR-number line past 0042, and to apply WEB-02's own dated manifest-citation correction banner (D-08) — none of that is done by this plan.
- No blockers for 14-06 through 14-08 specific to this plan's changes.

---
*Phase: 14-api-contract-truthfulness*
*Completed: 2026-08-12*

## Self-Check: PASSED

Both claimed files verified present on disk (`test -f`, two calls). Both commit hashes
(`070477c`, `1fc8547`) verified present in `git log --oneline --all`.
