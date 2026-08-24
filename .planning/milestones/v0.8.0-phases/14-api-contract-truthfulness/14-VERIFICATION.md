---
phase: 14-api-contract-truthfulness
verified: 2026-08-12T21:15:00Z
status: passed
score: 4/4 must-haves verified
behavior_unverified: 0
overrides_applied: 0
re_verification:
  previous_status: gaps_found
  previous_score: 3/4
  gaps_closed:
    - "A developer reading the agent API's authentication documentation, its config keys and its OpenAPI security scheme sees the mechanism the code actually runs (ROADMAP success criterion 1 / WEB-01)"
  gaps_remaining: []
  regressions: []
deferred: []
---

# Phase 14: API Contract Truthfulness Verification Report

**Phase Goal:** Every capability this project advertises through an interface is one it actually
has — so a developer reading the auth contract, deploying the Kubernetes manifests, or branching
on a provider capability flag gets the behaviour the interface promised.
**Verified:** 2026-08-12T21:15:00Z
**Status:** passed
**Re-verification:** Yes — after gap closure (commit `f9f493e`)

## Gap Closure Record

The prior verification pass (2026-08-12T20:00:00Z, status `gaps_found`, score 3/4) found success
criterion 1 / WEB-01 unmet because two documentation surfaces still described the superseded JWT
vocabulary after plan 14-01's rename:

1. `README.md:154` — the "Authentication & authorization" bullet read "**JWT** — send
   `Authorization: Bearer <token>`, verified via the wired `AuthPort`".
2. `.planning/codebase/INTEGRATIONS.md:131-134` — a "JWT Authentication" heading pointing at the
   removed `http.auth.jwt` config key.

Commit `f9f493e` ("docs(14): correct auth mechanism in README and codebase map") rewrote both
sections. This re-verification independently re-read the diff, the resulting prose, the two ADRs
it cites, and the source it describes — it does not take the commit message or SUMMARY at their
word. Finding: **the gap is genuinely closed, and no new problem was introduced.**

- `README.md:154-160` now reads: "**Bearer tokens** — send `Authorization: Bearer <token>`,
  verified via the wired `AuthPort`. These are opaque, server-issued tokens: a random string
  checked against the store's own hashed records, not a signed or self-describing token such as a
  JWT. The shipped store is in-process, so a token issued by one replica does not verify against
  another (see [ADR-0040](...) and [ADR-0041](...))."
- `.planning/codebase/INTEGRATIONS.md:131-138` now reads "**Bearer Token Authentication**", cites
  `http.auth.bearer_token` (the current key), and states the same hashed/in-process/single-replica
  facts with inline ADR-0040/ADR-0041 citations.
- Both files now contain exactly one occurrence of the string "JWT": README's contrastive clause
  "not a signed or self-describing token such as a JWT". This is judged **legitimate and accurate**
  — it correctly names what the mechanism is *not*, immediately after correctly stating what it
  *is*. It is not a residual miss; removing it would make the sentence read as an unfinished
  contrast, not a truer one.
- Cross-checked the prose against `crates/paladin-web/src/agent_auth.rs`'s own module docs
  ("Opaque server-issued bearer token via `Authorization: Bearer <token>`, verified by the injected
  `AuthPort` against the server's own in-process token store — not a signed or self-describing
  token") and against `src/infrastructure/adapters/auth/in_memory_token_auth_adapter.rs`'s
  implementation: `generate_token()` draws 32 random bytes (`rand::thread_rng().fill_bytes`) and
  base64-encodes them (opaque, unsigned); `hash_token()` SHA-256-hashes before storing
  (`store: RwLock<HashMap<String, AuthClaims>>` keyed by hash, not by raw token). The README/
  INTEGRATIONS prose matches this exactly — "a random string checked against the store's own
  hashed records" is a correct plain-English description of that code.
- Cross-checked the ADR-0040 and ADR-0041 citations resolve to real records
  (`.planning/decisions/0040-opaque-bearer-token-mechanism.md`,
  `.planning/decisions/0041-in-process-token-store-single-replica-scope.md`) and that each says
  what the prose attributes to it: ADR-0040 ratifies the opaque-token mechanism and states the
  rename reached "four surfaces" (it does not claim README/INTEGRATIONS among them — those two were
  the phase's own gap, closed separately by this commit, which is consistent, not contradictory).
  ADR-0041 states the in-process store is single-replica scoped and documents the unconditional
  startup warning plus the k8s ConfigMap/README/mdBook mitigations — exactly the "token issued by
  one replica does not verify against another" claim the README bullet makes.

No new anti-pattern, stub, or broken link was introduced by the fix commit (it touches only these
two `.md` files, confirmed via `git show f9f493e --stat`).

## Goal Achievement

### Observable Truths

| # | Truth (ROADMAP success criterion) | Status | Evidence |
|---|---|---|---|
| 1 | A developer reading the agent API's authentication documentation, its config keys and its OpenAPI security scheme sees the mechanism the code actually runs (WEB-01) | ✓ VERIFIED | Rust source, OpenAPI contract, config.example.yml, k8s ConfigMap, `.planning/codebase/ARCHITECTURE.md`, `docs/src/deployment-topologies/http-service-host.md` remain `grep`-clean of `jwt`/`JWT` outside the legitimate contrastive clause. `README.md:154-160` and `.planning/codebase/INTEGRATIONS.md:131-138` (fixed by commit `f9f493e`, re-read directly this session) now correctly describe the opaque, server-issued, hashed, in-process bearer-token mechanism and cite ADR-0040/ADR-0041, both of which resolve and say what the prose claims. Cross-checked against `agent_auth.rs` module docs and `in_memory_token_auth_adapter.rs`'s actual hashing/generation code — matches. See Gap Closure Record above. |
| 2 | A token issued against one server instance either verifies against another, or the deployment artefacts and documentation say it will not (WEB-02) | ✓ VERIFIED | Re-confirmed: `IN_PROCESS_TOKEN_STORE_WARNING` fires unconditionally in `build_auth_config` (`src/bin/paladin-server.rs:148-151,189-191`); `k8s/server/deployment.yaml:14` `replicas: 2` unmodified; `k8s/server/configmap.yaml:39-45` documents the constraint with an ADR-0041 pointer; `k8s/README.md` and `docs/src/deployment-topologies/http-service-host.md` state the limitation. No file touched by the gap-closure commit affects this criterion. |
| 3 | `ProviderCapabilities` reports tool-calling support that matches what the OpenAI, Anthropic and DeepSeek adapters actually do, with a test asserting the correspondence (WEB-03) | ✓ VERIFIED | Re-confirmed: `crates/paladin-llm/src/lib.rs` `test_capabilities_tool_calling_matches_request_surface` still present and unaffected by the docs-only gap-closure commit; prior direct test run recorded as passing. |
| 4 | Asking "does Paladin support LLM tool calling?" returns one recorded answer with reasoning — built or withdrawn — rather than a fourth deferred-register appearance (WEB-04) | ✓ VERIFIED | Re-confirmed: `.planning/decisions/0042-llm-native-tool-calling-deferred.md` still records the deferral; unaffected by the gap-closure commit. |

**Score:** 4/4 truths verified (0 present, behavior-unverified)

### Required Artifacts

| Artifact | Expected | Status | Details |
|---|---|---|---|
| `README.md` | Authentication documentation matches shipped mechanism | ✓ VERIFIED | Fixed by commit `f9f493e`; `:154-160` now describes the opaque bearer-token mechanism accurately, cites ADR-0040/0041, and the one remaining "JWT" occurrence is a legitimate contrastive clause |
| `.planning/codebase/INTEGRATIONS.md` | Same | ✓ VERIFIED | Fixed by commit `f9f493e`; `:131-138` renamed to "Bearer Token Authentication", `http.auth.bearer_token` key, ADR-0040/0041 cited |
| (all other phase-14 artifacts) | — | ✓ VERIFIED | Unchanged since prior pass (see previous VERIFICATION.md revision history in git); the gap-closure commit is docs-only and touches no other tracked artifact |

### Key Link Verification

| From | To | Via | Status | Details |
|---|---|---|---|---|
| `README.md`'s auth section | shipped `AuthPort` mechanism | prose description | ✓ WIRED | Prose now names the ratified mechanism (opaque bearer token) and matches `agent_auth.rs`/`in_memory_token_auth_adapter.rs` behavior |
| `.planning/codebase/INTEGRATIONS.md`'s auth section | shipped `AuthPort` mechanism + `http.auth.bearer_token` key | prose description | ✓ WIRED | Prose and config-key reference both correct |
| (all other phase-14 key links) | — | — | ✓ WIRED | Unchanged since prior pass; unaffected by the docs-only fix commit |

### Requirements Coverage

| Requirement | Source Plan | Description | Status | Evidence |
|---|---|---|---|---|
| WEB-01 | 14-01, 14-05, 14-07 (gap closed by orchestrator, commit `f9f493e`) | Agent API token mechanism has one true answer | ✓ SATISFIED | Code/config/OpenAPI/exports/CHANGELOG surfaces fully renamed (ADR-0040); README.md and INTEGRATIONS.md now also correct, closing the phase's "documentation" clause |
| WEB-02 | 14-04, 14-05, 14-07 | Multi-replica token verification correctness | ✓ SATISFIED | Re-confirmed; unaffected by the fix |
| WEB-03 | 14-02, 14-07 | `ProviderCapabilities` matches adapter reality | ✓ SATISFIED | Re-confirmed; unaffected by the fix |
| WEB-04 | 14-03, 14-06, 14-07 | Tool calling in-scope-with-plan or withdrawn | ✓ SATISFIED | Re-confirmed; unaffected by the fix |

No orphaned requirements: `.planning/REQUIREMENTS.md`'s Phase-14 traceability table lists exactly
WEB-01..04.

Note: `.planning/REQUIREMENTS.md`'s WEB-01 "Closed" annotation (`:2652-2663`) is dated to plan
14-01 and does not mention the later `f9f493e` gap-closure commit. This is a minor bookkeeping
staleness in a planning artifact, not a live developer-facing claim, and does not affect the truth
of WEB-01's closure — it is noted here for completeness, not raised as a gap.

### Anti-Patterns Found

None remaining. The two 🛑 Blocker findings from the prior pass (`README.md:154`,
`.planning/codebase/INTEGRATIONS.md:131-134`) are resolved by commit `f9f493e`, independently
re-verified above.

Full workspace `jwt`/`JWT` sweep re-run this session (excluding `.git`, `target`,
`.planning/decisions|ledgers|phases`, `.project/`, `examples/`, `docs/src/appendix/`,
`tests/cli/table_output_test.rs`, and `.claude/` framework/tooling files, which are unrelated to
Paladin's own auth surface or are historical/generic content — same exclusions the prior pass
applied). Remaining live hits, all judged legitimate:

- `README.md:156` — the one contrastive-clause occurrence (see Gap Closure Record).
- `.planning/codebase/INTEGRATIONS.md:137` — same pattern, inline ADR-0040 citation.
- `CHANGELOG.md:15-22` (`## [0.8.0]`, current/Unreleased entry) — accurately documents the breaking
  rename itself (`jwt` key → `bearer_token` key) and correctly states the mechanism "was always" an
  opaque token, never a signed JWT. This is the changelog *describing the fix*, not making an untrue
  claim.
- `CHANGELOG.md:194-222` (`## [0.6.0]`, a already-released historical version's notes) — describes
  the mechanism as JWT because that is what the M12 vocabulary called it at that point in the
  project's history. Legitimate historical record, consistent with how the rest of the CHANGELOG
  treats superseded terminology in old version sections.
- `.planning/STATE.md:343-347` — an in-progress planning snapshot dated to "Phase 14 execution
  started" (last touched by commit `5b5fca7`, before the phase's later waves and the gap-closure
  commit), recording the pre-fix problem statement that motivated WEB-01. Same category as
  `.planning/PROJECT.md`'s pre-phase problem statement, already judged legitimate in the prior
  pass — both are planning artifacts that get evolved at phase close, not developer-facing product
  documentation, and neither is live at HEAD in a way a developer deploying or integrating against
  Paladin would read as the current contract.
- `.planning/ROADMAP.md`, `.planning/REQUIREMENTS.md`, `.planning/INGEST-CONFLICTS.md`,
  `.planning/intel/*.md`, `.planning/milestones/v0.7.1-*.md` — all quote the pre-phase-14 problem
  statement or ingested historical PRD text inside "Context"/"done when" narrative blocks that are
  explicitly framed as describing the *prior* incoherent state; `REQUIREMENTS.md`'s WEB-01 entry
  carries its own "Closed" annotation directly beneath the historical quote. Same category judged
  legitimate in the prior pass.
- `crates/paladin-web/CHANGELOG.md:16-22` — same pattern as the root CHANGELOG's current entry:
  documents the rename, not an untrue live claim.
- `.claude/agents/*.md`, `.claude/gsd-core/**` — GSD framework tooling/template files with generic
  JWT examples unrelated to Paladin's own auth implementation (same category as the `examples/`
  exclusion already applied in the prior pass).

No `TBD`/`FIXME`/`XXX`/`TODO`/`HACK`/`PLACEHOLDER` markers found in `README.md` or
`.planning/codebase/INTEGRATIONS.md`.

### Human Verification Required

None. Every must-have in this phase is verifiable by source inspection, grep, or a directly-run
test; no visual, real-time, or external-service behavior is involved.

### Gaps Summary

No gaps remain. The single gap from the prior pass (`README.md` and
`.planning/codebase/INTEGRATIONS.md` still describing the superseded JWT vocabulary) was closed by
commit `f9f493e`, independently re-verified in this session against the actual diff, the resulting
prose, the two source files the prose describes (`agent_auth.rs`,
`in_memory_token_auth_adapter.rs`), and the two ADRs it cites (ADR-0040, ADR-0041) — all confirmed
accurate and consistent. The remaining single "JWT" occurrence in each fixed file is a legitimate
contrastive clause, not a residual miss. No new anti-pattern or broken link was introduced (the fix
commit is docs-only, touching exactly the two files it claims to). All four ROADMAP success
criteria (WEB-01 through WEB-04) are now verified true against the tree.

---

_Verified: 2026-08-12T21:15:00Z_
_Verifier: Claude (gsd-verifier)_
