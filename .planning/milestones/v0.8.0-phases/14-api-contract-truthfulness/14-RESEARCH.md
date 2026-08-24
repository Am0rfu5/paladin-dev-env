# Phase 14: API Contract Truthfulness - Research

**Researched:** 2026-08-12
**Domain:** Rust API surface truthfulness — HTTP auth (opaque bearer tokens vs. documented JWT), Kubernetes multi-replica correctness, LLM provider capability flags, ADR/CHANGELOG/release bookkeeping
**Confidence:** HIGH

<user_constraints>
## User Constraints (from CONTEXT.md)

### Locked Decisions

**Inherited, not re-litigated (D-00a…D-00k):** ADRs live in `.planning/decisions/`, flat sequential
numbering, no frontmatter, next free number **0040**. Precedence order: ADR → shipped tree →
`.planning/codebase/` map → `intel/code-verification.md` → PRD → DOC → checkbox. Source corrections
under `.project/` are dated annotation banners, original text retained, never rewritten. Ledgers
amended in place, dated, superseded text retained. Primary key is the `REQ-*` ID. Contested positions
get ADRs; code-settled defects get ledger rows, no ADR. Medieval-military ubiquitous language is
mandatory. Provenance of `--auto` decisions is carried forward, not laundered — WEB-03's substance
shipped under `--auto` in Phase 2 plan 02-02 (see D-11/D-14). ADR-0039 already supplies the HTTP half
of the Arsenal/LlmPort relationship WEB-04 needs. ADR-0037 fixes the agent route surface at `/v1`,
single source `API_V1_PREFIX` (`crates/paladin-web/src/agent_controller.rs:723`).

**WEB-01 — token mechanism:**
- **D-01:** Opaque server-issued tokens are the ratified mechanism (M9 Epic 5 §6.1). M12 Epic 5's Open
  Question 4 (signing secret/algorithm) is **dissolved**, not answered — an opaque hashed store needs
  neither.
- **D-02:** The config key (`http.auth.jwt.*` / `JwtAuthConfig`) is renamed clean, **no serde alias**.
  Old config files fail to load; this break is recorded in the CHANGELOG. No `#[serde(alias = "jwt")]`.
- **D-03:** The OpenAPI security scheme `SEC_JWT` is renamed and `bearerFormat: "JWT"` dropped. The
  committed `crates/paladin-web/openapi.json` baseline is regenerated in the **same commit**.
- **D-04:** The public Rust field `AgentAuthConfig.jwt: Option<Arc<dyn AuthPort>>` is renamed to
  something true (e.g. `token_verifier`); module docs, `Principal.id` doc, the "bearer JWT checked
  first" comment, and the `MockJwt` test double all follow. `paladin-web`'s CHANGELOG gets a
  `BREAKING` entry; `.project/current-exports.txt` is regenerated.
- **D-05:** The correction must reach every verified site (re-grep before acting, don't trust cached
  line numbers): `agent_auth.rs`, `openapi.rs:6,26-27,49-57`, `agent_controller.rs` handler
  `security(...)` annotations, `src/config/agents.rs:90-112` (incl. 3 tests at `:306-334`),
  `src/bin/paladin-server.rs:171-199`, `config.example.yml:58-59`, `k8s/server/configmap.yaml`,
  deployment-topology docs.

**WEB-02 — multi-replica store:**
- **D-06:** The shared-store requirement attaches to the `AuthPort` credential path, **not** to
  replica count — replicas stay as shipped (`replicas: 2`). The shipped ConfigMap sets
  `jwt.enabled: false` and uses static API keys (identical across pods), so today's default is
  correct. ADR-0041 must state this reasoning explicitly — it departs from WEB-02's own "done when"
  text (which offers only "pin replicas: 1" or "build the shared store").
- **D-07:** `build_auth_config` (`src/bin/paladin-server.rs:145-199`) logs a WARN whenever the
  `AuthPort` verifier is wired, naming the constraint (tokens verify only on the issuing process; do
  not scale past one replica with this store) — unconditional on the store being wired, not
  conditional on replica count (a pod can't read its own `spec.replicas`). Paired with an inline
  comment in `k8s/server/configmap.yaml`, a note in `k8s/README.md`, and a stated limitation on the
  deployment-topology docs. **Rejected:** a new opt-in "refuse to start" flag.
- **D-08:** WEB-02's own citation names the wrong manifests (`k8s/deployment.yaml` /
  `k8s/service.yaml`); the real M12 Epic 7 artefacts are `k8s/server/deployment.yaml` /
  `k8s/server/service.yaml`. Corrected via a dated banner on WEB-02 per D-00c/D-00d. The root
  `k8s/deployment.yaml` placeholder (image `paladin:test`, `sleep 3600`, probes commented out) is
  **left alone** — out of scope.
- **D-09:** The shared store is deferred with a named trigger — the first deployment needing more
  than one replica serving `AuthPort`-issued tokens — recorded in ADR-0041 (ADR-0035 precedent: a
  reintroduction condition without building the thing).

**WEB-03 / WEB-04 — capability flags and tool calling:**
- **D-10:** LLM-native tool calling (Deferred-QA Epic 27) is recorded as a **future capability
  improvement, not built**. Verbatim user framing: *"We want to maximize the capabilities so this
  sounds like a future feature improvement... this is the source of some potential future version
  improvements not any current functionality. This should be recorded as such and everything should
  properly reflect current functionality."* ADR-0042 carries a named reintroduction trigger and owner.
- **D-11:** WEB-04's record must not be a fourth deferred-register entry. ADR-0042 is the record; a
  dated correction banner goes on `.project/Deferred-QA-CICD-Completion/prd-deferred-qa-completion.md`
  Epic 27 (`:124,250-298`, `:492`, `:557`, `:570,574`), pointing at the ADR.
- **D-12:** `supports_function_calling` is flipped to `false` on the OpenAI adapter (only asymmetric
  flag: declares `true` at `:650` while hardcoding `function_call: None` at `:553`). The existing
  correspondence test `test_capabilities_tool_calling_matches_request_surface`
  (`crates/paladin-llm/src/lib.rs:98-136`) is **extended**, not replaced, to pin **both** flags.
- **D-13:** The reachability limitation (no shipped adapter ever emits a `FunctionCall`; the tool
  branch at `paladin_execution_service.rs:799`/`:1414` fires only for a consumer's own `LlmPort`) is
  stated in rustdoc on `ProviderCapabilities`/`LlmResponse.function_call`
  (`llm_port.rs:620,815-837`) and on `docs/src/user-guides/tool-integration.md`,
  `docs/src/architecture/overview.md`, `docs/src/architecture/domain-model.md`,
  `docs/src/contributing/contributing-providers.md`. **Scope guard:** reachability claims only, not
  general content currency (that's DOCS-01, Phase 16).
- **D-14:** WEB-03's already-shipped half (commit `a2cc1c5`, plan 02-02, `--auto`) is closed with
  provenance recorded, not re-implemented. Ledger rows `REQ-llm-tool-calling-port` /
  `REQ-llm-tool-calling-adapters` amended in place per D-00d, citing the commit and the `--auto`
  provenance per D-00i.

**Cross-cutting:**
- **D-15:** Both Phase 13 hand-off items are in scope: (a) `crates/doc-examples/src/sidecar.rs:25,34`
  — unversioned route → `{base_url}/v1/agents/{agent}/execute`, paired with an assertion tying the
  literal to `paladin_web::agent_controller::API_V1_PREFIX` (closes T-13-20/AR-13-01; re-run
  `/gsd-secure-phase 13` after). (b) `REQ-fail-closed-auth-posture` gets a test driving the `Err`
  branch of `build_auth_config`. **Method note, do not redraw:** a "zero occurrences of X" assertion
  is unsound against a page using mdBook `{{#include}}` — sweep include targets too, or assert against
  rendered `docs/book/` output.
- **D-16:** Three ADRs — 0040 (WEB-01), 0041 (WEB-02), 0042 (WEB-04). One decision per record (Phase
  11's ADR-0034 bundling cost as the precedent to avoid). WEB-03 gets **no ADR** (code-settled defect,
  D-00g).
- **D-17:** Release bookkeeping lands in this phase — `BREAKING` CHANGELOG entries and a lockstep bump
  to **0.8.0** across all **twelve** manifests (root `Cargo.toml` + 11 crate manifests, `release.toml`
  `shared-version = true`). 0.8.0 not 0.7.2: under SemVer 0.x, a breaking change bumps the minor.
- **D-18: Ordering constraint.** `openapi.rs:37` sets `api.info.version` from
  `env!("CARGO_PKG_VERSION")`, so the 0.8.0 bump moves the committed `openapi.json` baseline a
  **second** time (after D-03's scheme rename already moved it once). Land the bump and the
  regeneration in the same commit, or regenerate once more after the bump — a plan that regenerates
  only after D-03 leaves the drift guard red.
- **D-19:** This phase changes `.rs` in published crates (the opposite of Phase 13's zero-`.rs`
  boundary). Every `.rs` change goes through `cargo test` → `cargo fmt --check` →
  `cargo clippy -- -D warnings` before commit.
- **D-20:** Two machine baselines move and are regenerated in the commit that moves them:
  `crates/paladin-web/openapi.json` (drift guard) and `.project/current-exports.txt` (api-surface
  guard).

### Claude's Discretion
- Exact replacement identifiers for `SEC_JWT`, `AgentAuthConfig.jwt`, `JwtAuthConfig`,
  `http.auth.jwt.*` — pick one vocabulary, use it in all four places, prefer a term surviving a future
  shared-store swap (mechanism = "opaque server-issued bearer token", component = "verifier"). D-00h
  applies only where a domain noun is being coined, not to HTTP/OpenAPI terms of art.
- Whether `crates/paladin-llm/src/mock.rs` should gain the ability to emit a `FunctionCall` (currently
  it declares both flags `false` and never emits one) — out of scope if it grows past a constructor
  option.
- Plan decomposition and wave structure — WEB-03's residue (D-12) and D-15's two items are independent
  of WEB-01 and can run in parallel; D-17/D-18's bump must land last or be sequenced with the final
  regeneration.
- Ledger and requirement amendment mechanics — which rows carry the closure vs. the pointer.
- Whether ADR-0040 also records M12 Epic 5's OQ-4 as dissolved in the OQ table's own location, or only
  in the ADR.

### Deferred Ideas (OUT OF SCOPE)
- A shared-store `AuthPort` implementation (SQLite via `paladin-storage`, or Redis) — deferred by D-09
  with a named trigger, recorded in ADR-0041.
- LLM-native tool calling (Deferred-QA Epic 27) — deferred by D-10 with a trigger and owner, recorded
  in ADR-0042 and banner-linked from the source PRD by D-11.
- Letting `crates/paladin-llm/src/mock.rs` emit a `FunctionCall` — left to planner discretion, out of
  scope if it grows past a constructor option.
- Annotating the root `k8s/deployment.yaml` as a placeholder — declined under D-08.
- A JWT `AuthPort` implementation — not chosen; D-01 ratifies opaque tokens.
- Implementing LLM-native tool calling itself, building the shared-store `AuthPort`, general
  documentation content-currency (Milestone 11's fourteen files are DOCS-01/Phase 16), annotating the
  root `k8s/deployment.yaml`, and CI/coverage gates (PIPE-01…05/Phase 15) are all explicitly **not in
  this phase**.
</user_constraints>

<phase_requirements>
## Phase Requirements

| ID | Description | Research Support |
|----|-------------|------------------|
| WEB-01 | The agent API's token mechanism has one answer, matching the implementation (opaque tokens, JWT vocabulary removed) | `## Architecture Patterns` Pattern 1 (rename mechanics), `## Code Examples` (openapi scheme rename, config rename), `## Runtime State Inventory`, verified code sites in `## Architectural Responsibility Map` |
| WEB-02 | Multi-replica token verification is correct, or the deployment says it is not supported | `## Architecture Patterns` Pattern 2 (startup warning, doc limitation), `## Common Pitfalls` (replica/config-flag mismatch), verified `k8s/server/*` manifest contents below |
| WEB-03 | `ProviderCapabilities` reports the capability the adapters actually have, with a correspondence test | `## Code Examples` (extending `test_capabilities_tool_calling_matches_request_surface`), verified adapter capability blocks below |
| WEB-04 | LLM tool calling is either in scope with a plan, or withdrawn with a reason, recorded once (not a fourth deferred entry) | `## Architecture Patterns` Pattern 3 (ADR + banner pattern, ADR-0035/ADR-0039 precedent) |
</phase_requirements>

## Summary

Phase 14 is not a "build new technology" phase — it is a **truthfulness-correction phase** across an
already-shipped Rust workspace. All four requirements are find-and-fix work: rename a documented-but-
unimplemented JWT mechanism to the opaque bearer-token scheme that actually ships (WEB-01), state a
multi-replica correctness limitation that the shipped ConfigMap currently makes moot but the shipped
Deployment doesn't document (WEB-02), fix one asymmetric capability flag and extend an existing
correspondence test (WEB-03), and record a single authoritative answer on LLM-native tool calling
instead of a fourth "deferred" mention (WEB-04). No new external dependency is introduced anywhere in
this phase — confirmed by `grep -rn "jsonwebtoken" Cargo.toml crates/*/Cargo.toml` returning nothing,
and by the CONTEXT.md decisions explicitly ratifying opaque tokens over adding `jsonwebtoken`.

The technical risk in this phase is not "which library to use" but **ordering and completeness**:
two committed machine baselines (`crates/paladin-web/openapi.json`, `.project/current-exports.txt`)
must be regenerated in the exact commits that move them (D-20), the OpenAPI baseline moves **twice**
(scheme rename D-03, then version bump D-18) and a plan that regenerates only once leaves the drift
guard red, the CHANGELOG/version bump spans **twelve** manifests via `release.toml`'s
`shared-version = true`, and the rename must reach every verified call site (D-05) rather than stop
at the first plausible one. This project's own established patterns — a drift guard comparing
generated OpenAPI against a committed baseline (`openapi.rs:120-125`), a capability-vs-request-surface
correspondence test (`lib.rs:98-136`), and dated in-place ledger/ADR amendment (D-00c/D-00d) — are the
tools to reuse; nothing new needs inventing.

**Primary recommendation:** Treat this phase as a coordinated rename-and-record exercise using the
project's existing verification mechanisms (drift guard, api-surface script, correspondence test,
`cargo clippy -- -D warnings`) as the acceptance criteria, sequence the version bump last per D-18,
and write the ADRs (0040/0041/0042) as the artifacts that carry the *reasoning* the code changes alone
cannot — especially D-06's deliberate departure from WEB-02's literal "done when" text.

## Architectural Responsibility Map

| Capability | Primary Tier | Secondary Tier | Rationale |
|------------|-------------|----------------|-----------|
| Token issuance/verification (opaque bearer tokens) | API / Backend | — | `InMemoryTokenAuthAdapter` (`src/infrastructure/adapters/auth/`) implements `AuthPort`; SHA-256-hashed in-process store, no client/browser role |
| Config → auth wiring (`http.auth.*`) | API / Backend (binary) | — | `build_auth_config` in `src/bin/paladin-server.rs` is the single seam translating `AuthConfig` into `AgentAuthConfig` |
| Published API contract (OpenAPI security scheme) | API / Backend | CDN / Static (served `/docs`, `/openapi.json`) | `crates/paladin-web/src/openapi.rs` generates the spec from handler annotations; served unauthenticated so consumers can discover the contract before holding credentials |
| Multi-replica request routing | Database / Storage boundary (the in-process store *is* the storage tier here) | API / Backend | The correctness question is entirely about whether the store backing `AuthPort` is shared across replicas, not about the Kubernetes Service/Deployment tier itself |
| LLM provider capability declaration | API / Backend (library surface, `paladin-llm`) | — | `ProviderCapabilities` is a `paladin-ports` trait-level type; adapters in `paladin-llm` are the sole authors of the declared value |
| Tool-call reachability (Arsenal invocation) | API / Backend (`src/application/services/paladin/paladin_execution_service.rs`) | — | Gated entirely on `LlmResponse.function_call`, which no shipped adapter populates; Arsenal itself (`paladin-core`) is unaffected — only the LLM-port seam is truthfulness-defective |
| Documentation / OpenAPI truthfulness | Docs / Static artifact | — | `docs/src/**`, `.project/**` banners, `crates/doc-examples/src/sidecar.rs` (compiled but not the runtime API surface) |

## Standard Stack

### Core (already in the workspace — no new dependencies)

| Library | Version | Purpose | Why Standard |
|---------|---------|---------|--------------|
| `utoipa` | 5 (`crates/paladin-web/Cargo.toml:35`) | OpenAPI spec generation from `#[utoipa::path]` handler annotations and `ToSchema` DTOs | Already the sole OpenAPI generator in the workspace; `SecurityScheme::Http(HttpBuilder::new().scheme(HttpAuthScheme::Bearer)...)` is the exact API surface WEB-01/D-03 must edit |
| `utoipa-swagger-ui` | 9 | Serves the interactive `/docs` UI from the generated spec | Already wired via `docs_router` in `openapi.rs` |
| `sha2` | workspace-pinned | SHA-256 hashing of opaque tokens before storage | Already used by `InMemoryTokenAuthAdapter::hash_token` — no change needed for WEB-01/02, cited here because the ADR (0040) should state it as the mechanism's crypto primitive |
| `rand` | workspace-pinned | 32 cryptographically random bytes per issued token | Already used by `InMemoryTokenAuthAdapter::generate_token` |
| `axum` | workspace-pinned | HTTP framework; `AgentAuthConfig` wires into `axum::middleware::Next` | Unaffected by this phase's renames beyond field/type names |
| `serde` / `thiserror` | workspace-pinned | Config deserialization, error types | `JwtAuthConfig`'s rename is a `#[derive(Serialize, Deserialize)]` struct rename — standard serde mechanics, no new crate |
| `cargo-release` | external tool, not a crate dep | Drives `make release VERSION=x.y.z`; `release.toml` sets `shared-version = true`, so a single version bump command updates the workspace's 12 manifests together | Already the project's chosen release tool (`docs/RELEASE_AUTOMATION.md` referenced in `release.toml`) — do not hand-roll a version-bump script |

### Alternatives Considered

| Instead of | Could Use | Tradeoff |
|------------|-----------|----------|
| Opaque server-issued bearer tokens (D-01, ratified) | Real JWT `AuthPort` implementation | Rejected by the user's decision: adds a `jsonwebtoken` dependency, a signing-key management/rotation story, and gives up immediate revocation (a stateless JWT can't be revoked without a blocklist, which reintroduces server-side state anyway) |
| `#[serde(default)]` field-level opt-in per pod for the store-scope warning (D-07) | A new "refuse to start unless shared-store confirmed" flag | Rejected — "a knob whose only job is to be typed once" |
| Extending the existing correspondence test (D-12) | Writing a new, separate test module for `supports_function_calling` | Rejected implicitly by D-12's own text — the two flags must be pinned by the **same** test so they cannot drift apart independently again |

**No installation step is required for this phase** — every crate referenced above is already a
workspace dependency at its current pinned version.

## Package Legitimacy Audit

**Not applicable.** This phase introduces no new external package to any manifest. Verified:
`grep -rn "jsonwebtoken" Cargo.toml crates/*/Cargo.toml` → no matches (2026-08-12, re-confirmed
against CONTEXT.md D-01's identical finding). All twelve manifests are workspace members already
building against pinned dependencies (`utoipa`, `utoipa-swagger-ui`, `sha2`, `rand`, `axum`, `serde`,
`thiserror`) — no `Cargo.toml` gains a new `[dependencies]` entry as part of this phase's scope.

**Packages removed due to [SLOP] verdict:** none.
**Packages flagged as suspicious [SUS]:** none.

## Architecture Patterns

### System Architecture Diagram

```
Config layer                    Web layer (paladin-web)              Published contract
─────────────                   ────────────────────────             ───────────────────
config.yml /                    build_auth_config()                  openapi.rs::decorate()
k8s ConfigMap                   (src/bin/paladin-server.rs)          ── adds security schemes
  http.auth.jwt.enabled  ──────►   reads AuthConfig.jwt.enabled  ────►    SEC_JWT / api_key
  (renamed in D-02)                    │                                    (renamed in D-03)
                                        │ if enabled:                        │
                                        ▼                                    ▼
                                 Arc<dyn AuthPort>                   crates/paladin-web/openapi.json
                                 = InMemoryTokenAuthAdapter                (drift-guard baseline,
                                   (SHA-256-hashed opaque              regenerated in same commit
                                    token store, in-process)              as D-03 AND again for D-18)
                                        │
                                        ▼
                                 AgentAuthConfig.jwt                  agent_auth.rs middleware
                                 (renamed to e.g.                     ── verifies bearer token via
                                  token_verifier, D-04)                  AuthPort::verify_token
                                                                          (rename in Principal doc,
                                                                           comment, MockJwt double)

Multi-replica question (WEB-02):
  k8s/server/deployment.yaml (replicas: 2)
        │
        ▼
  Pod A issues token ──X──► Pod B verifies token   (X = fails: separate in-process stores)
        │
        └── only reachable when k8s/server/configmap.yaml sets jwt.enabled: true
            (shipped default: false — static API keys from Secret, identical per pod, safe)
  Fix: WARN at startup when AuthPort verifier wired (D-07) + doc limitation, not a replica pin (D-06)

LLM capability truthfulness (WEB-03/04):
  Adapter.get_capabilities() ──declares──► ProviderCapabilities{supports_tool_calling,
                                                                  supports_function_calling}
        │                                          │
        │ must match                               │ (D-12: extend correspondence test to pin both)
        ▼                                          ▼
  Adapter.generate() response.function_call   always None on all 3 shipped adapters + mock.rs
        │
        ▼
  paladin_execution_service.rs:799 tool branch  ── unreachable via any shipped provider
                                                     (reachable only via a consumer's own LlmPort)
```

### Recommended Project Structure

No new files/directories are needed. Touched files stay in their existing locations:

```
crates/paladin-web/src/
├── agent_auth.rs         # D-04: field/doc/comment/test-double rename
├── openapi.rs            # D-03: security-scheme rename, bearer_format removal
├── agent_controller.rs   # D-05: handler security(...) annotations
└── openapi.json           # D-03 + D-18: regenerated baseline (moves TWICE)

src/config/agents.rs       # D-02: JwtAuthConfig -> renamed type, key rename
src/bin/paladin-server.rs  # D-02/D-07: wiring comment, cfg branch, warn log

crates/paladin-llm/src/
├── openai/adapter.rs      # D-12: supports_function_calling -> false
└── lib.rs                 # D-12: extend capability_invariants test module

crates/paladin-ports/src/output/llm_port.rs  # D-13: rustdoc on ProviderCapabilities/function_call

crates/doc-examples/src/sidecar.rs           # D-15a: unversioned route -> /v1/...

.planning/decisions/
├── 0040-*.md   # WEB-01
├── 0041-*.md   # WEB-02
└── 0042-*.md   # WEB-04

.project/Deferred-QA-CICD-Completion/prd-deferred-qa-completion.md  # D-11: dated banner
```

### Pattern 1: Clean-break rename with regenerated baselines (WEB-01)

**What:** Rename a public identifier/config-key/OpenAPI-contract-id with no compatibility alias, and
regenerate every machine-checked baseline that encodes the old name in the same commit.
**When to use:** WEB-01's config key, Rust field, and OpenAPI security scheme — all three are
explicitly "one-way" breaks per D-02/D-03/D-04's own reversibility notes.
**Example (OpenAPI scheme rename, current shape at `openapi.rs:49-57`):**
```rust
// Source: crates/paladin-web/src/openapi.rs (verified 2026-08-12)
// BEFORE:
components.add_security_scheme(
    SEC_JWT,
    SecurityScheme::Http(
        HttpBuilder::new()
            .scheme(HttpAuthScheme::Bearer)
            .bearer_format("JWT")   // an opaque token has no format -> drop (D-03)
            .build(),
    ),
);

// AFTER (illustrative — exact identifier is Claude's Discretion, D-00h applies):
components.add_security_scheme(
    SEC_BEARER_TOKEN,  // renamed from SEC_JWT
    SecurityScheme::Http(
        HttpBuilder::new()
            .scheme(HttpAuthScheme::Bearer)
            // no .bearer_format(..) call — the token has no registered format
            .build(),
    ),
);
```
Then regenerate the baseline **in the same commit**:
```bash
UPDATE_OPENAPI=1 cargo test -p paladin-web openapi_matches_committed_baseline
# or: make openapi
```

### Pattern 2: Unconditional startup warning naming a scope limitation (WEB-02)

**What:** Log a `WARN` whenever a code path with a known multi-instance limitation is enabled, rather
than gating the warning on information the process cannot observe (its own replica count).
**When to use:** D-07's `build_auth_config` warning.
**Example (existing pattern to extend, `src/bin/paladin-server.rs:145-155`):**
```rust
// Source: src/bin/paladin-server.rs (verified 2026-08-12) — existing disabled-auth warning,
// the voice D-07's new warning should match:
if !cfg.enabled {
    warn!(
        "agent API authentication is DISABLED (http.auth.enabled = false) — all agent routes are open"
    );
    ...
}

// D-07 adds a sibling warning on the *enabled* branch, unconditional on replica count:
let jwt: Option<Arc<dyn AuthPort>> = if cfg.jwt.enabled {
    warn!(
        "in-process bearer-token store enabled — tokens verify only on the issuing process; \
         do not scale past one replica while this store is wired (see ADR-0041)"
    );
    Some(Arc::new(InMemoryTokenAuthAdapter::new()))
} else {
    None
};
```

### Pattern 3: ADR + dated banner replacing a register entry (WEB-04)

**What:** When a capability has been proposed and deferred multiple times across a project's document
corpus, the fix is not another deferred-register line — it's a single ADR the register points at.
**When to use:** WEB-04, following the exact precedent already in this repo at ADR-0035
(`paladin-ml-leaf-crate-placement`) — a reintroduction condition promoted into a decision record
**without building the thing**.
**Example shape (from `.planning/decisions/0039-*.md`, the most recent ADR in this series):**
```markdown
# ADR-0042: LLM-native tool calling — deferred with a named trigger, not built

## Status
Accepted

## Context
[Cite Deferred-QA Epic 27, its two open questions (OQ-1 DeepSeek support, OQ-5 canonical schema),
 the breaking LlmPort change it requires, and ADR-0039's HTTP-topology half of the relationship.]

## Decision
[D-10's verbatim user framing goes here as the recorded rationale.]

## Considered Options
- Build Epic 27 now (rejected — breaking LlmPort change across 3 adapters + mock, 2 unanswered OQs)
- Withdraw entirely (rejected — user explicitly wants it recorded as a future capability, not deleted)
- Record as deferred-with-trigger (chosen — ADR-0035 precedent)

## Code Locations
[.project/.../prd-deferred-qa-completion.md Epic 27 sites the dated banner (D-11) points at here]

## Code Conformance
conforms — no code change; the ADR records intent, D-12/D-13 correct the current-state claims
```

### Anti-Patterns to Avoid

- **Compatibility aliasing a renamed config key:** `#[serde(alias = "jwt")]` was explicitly considered
  and rejected in D-02 — it would leave the untrue word in the accepted input surface with nobody
  owning its removal. Do not add one during implementation even if it seems like a kindness to
  operators; the CHANGELOG `BREAKING` entry is the intended mechanism for that communication instead.
- **Gating the WEB-02 warning on replica count:** a pod cannot read its own `spec.replicas` without
  Kubernetes API access (D-07). Do not add a Kubernetes API client dependency to make the warning
  "smarter" — that is out of scope and contradicts the decision's own reasoning.
- **Treating WEB-02's literal "done when" text as binding over the verified tree state:** D-06 is a
  deliberate departure — the ROADMAP criterion offers only "pin `replicas: 1`" or "build the shared
  store," but the tree shows the correctness edge is gated by a config flag the shipped ConfigMap
  already turns off. ADR-0041 must state this reasoning; don't silently pin replicas to satisfy the
  literal text without documenting the deviation.
- **Regenerating the OpenAPI baseline only once:** D-18 — the version bump moves `api.info.version`
  independently of the scheme rename. A single regeneration after only one of the two changes leaves
  the drift guard red for whichever change landed second.

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| Detecting drift between the generated OpenAPI spec and the published contract | A new script/CI job comparing spec output | The existing `openapi_matches_committed_baseline` test (`openapi.rs:120-125`) + `UPDATE_OPENAPI=1 cargo test` / `make openapi` | Already wired, already gates CI implicitly via `cargo test`; a parallel mechanism would be a second source of truth |
| Pinning a declared capability flag to actual behavior | A new assertion framework or runtime capability probe | Extend `test_capabilities_tool_calling_matches_request_surface` (`lib.rs:98-136`) with a second `REQUEST_SURFACE_SUPPORTS_FUNCTION_CALLING` constant and a second assert loop, following the exact existing shape | The pattern already exists, is named for exactly this purpose, and D-12 explicitly says "extended," not replaced |
| Detecting public-API-surface breaking changes | Manually diffing exported symbols, or introducing `cargo-semver-checks` (not currently in the toolchain) | The existing `scripts/extract-public-api.sh` / `scripts/check-api-surface.sh` pair driven by `cargo-public-api`, regenerating `.project/current-exports.txt` | Already the project's chosen tool for this; introducing a second tool (`cargo-semver-checks`) is out of this phase's decision scope (not listed under Claude's Discretion) |
| Bumping 12 manifest versions in lockstep | A custom `sed`/`find` version-bump script | `cargo release version 0.8.0` (or `make release VERSION=0.8.0`) per `release.toml`'s `shared-version = true` | Already the project's release tool; a hand-rolled script risks missing a manifest or the `workspace.dependencies` pins at `Cargo.toml:20-28,55` |
| Detecting the `{{#include}}` mdBook drift hazard | A grep against `docs/src/*.md` raw source for the old route string | Sweep the include *targets* (`crates/doc-examples/src/sidecar.rs`) directly, or assert against rendered `docs/book/` output | D-15's carried-forward method note: the raw markdown source of an mdBook page using `{{#include}}` never contains the literal string — a source-level grep is *structurally incapable* of catching this class of drift, proven by T-13-20's original (unsound) acceptance criterion |

**Key insight:** every "don't hand-roll" item in this phase already has a working, tested mechanism
in the tree. The work is *using* those mechanisms correctly and completely (D-05's "reach every
verified surface," D-20's "regenerate baselines in the same commit"), not building new ones.

## Runtime State Inventory

> Rename phase — WEB-01 renames a config key, a public Rust field, and an OpenAPI security-scheme
> identifier. Each category below was checked explicitly against the shipped tree on 2026-08-12.

| Category | Items Found | Action Required |
|----------|-------------|------------------|
| **Stored data** | The `InMemoryTokenAuthAdapter`'s token store (`RwLock<HashMap<String, AuthClaims>>`) is **in-process, ephemeral memory only** — no database, no persisted collection, no on-disk file. Confirmed by reading `src/infrastructure/adapters/auth/in_memory_token_auth_adapter.rs` in full: no `sqlx`, `redis`, or file-write calls anywhere in the module. | None — a config-key or field rename touches zero stored records because nothing is stored outside process memory, and that memory is discarded on restart. |
| **Live service config** | `k8s/server/configmap.yaml`'s `http.auth.jwt.enabled: false` is the **only** live-service config instance of the string `jwt` for this surface, and it is checked into git (not a UI-managed external config). `config.example.yml:58-59` is also in git. | Code edit — both are plain YAML in the repo, edited directly as part of D-02/D-05's sweep. No out-of-band service (n8n, Datadog, etc.) is involved. |
| **OS-registered state** | None applicable — this is a stateless HTTP server process; no Windows Task Scheduler, systemd unit, or pm2 process name embeds `jwt`. | None — verified by domain reasoning (no OS-level registration exists for this project's deployment model, which is a container image run under a Kubernetes Deployment). |
| **Secrets/env vars** | `${PALADIN_API_KEY_CI}` / `${PALADIN_API_KEY_APP}` (referenced in `k8s/server/configmap.yaml`, sourced from `k8s/server/secret.yaml.example`) name **API keys**, not the JWT/token-verifier config — unaffected by D-02's rename. No secret key name anywhere contains the string `jwt`. | None — the renamed config key (`http.auth.jwt.*` → new name) is a **boolean feature flag with no accompanying secret value** (an opaque store needs no signing secret, per D-01's OQ-4 dissolution) — nothing in the Secret changes. |
| **Build artifacts / installed packages** | `.project/current-exports.txt` (api-surface baseline) and `crates/paladin-web/openapi.json` (OpenAPI drift baseline) both encode the pre-rename public field/scheme names and will go stale the moment D-03/D-04 land, **before** regeneration. | Regenerate both in the **same commit** as the code change that moves them (D-20) — `UPDATE_OPENAPI=1 cargo test -p paladin-web openapi_matches_committed_baseline` and `./scripts/extract-public-api.sh .project/current-exports.txt`. |

**Nothing found in category "OS-registered state" and no persisted token records exist to migrate** —
this is a code-and-documentation rename, not a data migration. The only artifacts requiring
regeneration (not migration) are the two machine-checked baselines listed above.

## Common Pitfalls

### Pitfall 1: Regenerating the OpenAPI baseline once when it needs to move twice
**What goes wrong:** A plan lands D-03's security-scheme rename, regenerates `openapi.json`, and
considers the baseline "done." Later, D-17's version bump to 0.8.0 changes `api.info.version` (sourced
from `env!("CARGO_PKG_VERSION")` at `openapi.rs:37`), silently re-diverging the baseline.
**Why it happens:** The two changes (scheme rename, version bump) are naturally sequenced into
different plans/waves per the Claude's Discretion note that the bump "must land last or be sequenced
with the final regeneration" — but "last" is easy to interpret as "after everything, including
regeneration" rather than "with one more regeneration after it."
**How to avoid:** Either land D-17/D-18 in the same commit as the final `UPDATE_OPENAPI=1` run, or
make the last plan in the phase run the regeneration command a second time regardless of what else
changed in that plan, and verify with `cargo test -p paladin-web openapi_matches_committed_baseline`
(no `UPDATE_OPENAPI` set) before considering the phase closed.
**Warning signs:** `openapi_matches_committed_baseline` fails in CI/local test run after what looked
like an unrelated version-only commit.

### Pitfall 2: Treating the WEB-02 "done when" text as the actual acceptance bar
**What goes wrong:** WEB-02's own requirement text in REQUIREMENTS.md offers exactly two literal
exits — pin `replicas: 1`, or build a shared-store `AuthPort`. Neither is what D-06 does. A plan
written straight off the requirement text (rather than off CONTEXT.md's D-06) will produce the wrong
artifact — either degrading a working `replicas: 2` deployment, or building the deferred shared store.
**Why it happens:** ROADMAP/REQUIREMENTS success-criteria text is written before the tree-level
finding (D-06's finding 3: the correctness edge is gated by `jwt.enabled: false` in the shipped
ConfigMap, not by replica count) was verified.
**How to avoid:** Plan WEB-02 directly off CONTEXT.md D-06/D-07/D-08/D-09, and have ADR-0041 state
explicitly why it departs from the literal requirement text — this is D-06's own instruction, not
optional documentation flourish.
**Warning signs:** A plan step that edits `k8s/server/deployment.yaml`'s `replicas:` value, or that
schedules building a shared `AuthPort` implementation inside this phase.

### Pitfall 3: Missing a verified-but-easy-to-forget rename site
**What goes wrong:** D-05 lists eight-plus concrete sites across `agent_auth.rs`, `openapi.rs`,
`agent_controller.rs`, `agents.rs` (incl. 3 tests), `paladin-server.rs`, `config.example.yml`,
`k8s/server/configmap.yaml`, and deployment-topology docs. A plan that greps for the old identifier
once at the start and stops there will miss sites where the identifier's *prose meaning* ("JWT")
persists even after the *code* identifier changes — e.g. `Principal.id`'s doc comment
("API-key name or JWT subject," `agent_auth.rs:36-37`) or `.planning/codebase/ARCHITECTURE.md`'s own
"Web API: X-API-Key header or JWT token" (`:318`) and "Auth middleware validates X-API-Key or JWT"
(`:120`), both confirmed still present as of 2026-08-12.
**Why it happens:** Grepping for a Rust identifier (`SEC_JWT`, `jwt:`) misses free-text prose that
says "JWT" without using the identifier.
**How to avoid:** Two separate sweeps — one for the code identifier (compiler-enforced, `cargo build`
will catch most misses), one for the case-insensitive string `jwt` across `docs/`, `.planning/`, and
crate-level rustdoc, which the compiler cannot catch.
**Warning signs:** `cargo build` passes but `grep -ril "jwt" docs/ .planning/codebase/` still returns
hits after the phase's commits land.

### Pitfall 4: Cold `pre-commit` build in an isolated worktree
**What goes wrong:** If phase execution runs in a fresh git worktree (per this project's execution
model), the first `cargo test`/`cargo clippy` invoked by a pre-commit hook compiles the entire
workspace from scratch, which can appear to hang or time out on a plan's first commit.
**Why it happens:** A new worktree has no populated `target/` directory; every dependency and crate
recompiles.
**How to avoid:** Surface `worktree_skip_hooks` (already `true` in `.planning/config.json`'s
`workflow` block) to the executor, or budget extra time for the first commit's `cargo test` /
`cargo fmt --check` / `cargo clippy -- -D warnings` cycle in each new worktree this phase's plans run
in.
**Warning signs:** The first commit of a plan appears to stall on `cargo clippy` far longer than
subsequent commits in the same worktree.

### Pitfall 5: A published crate-family CHANGELOG entry that undercounts the version delta
**What goes wrong:** D-17 requires `BREAKING` entries in `paladin-web`'s CHANGELOG (D-02's config-key
break, D-04's field break) — but the workspace CHANGELOG format follows Keep a Changelog with an
`## [Unreleased]` heading that `make release` moves under a dated version section. If a plan writes
the `BREAKING` bullet directly under a `## [0.8.0]` heading instead of `## [Unreleased]`, the release
tooling's heading-move step (per `release.toml`'s own comment: "the bump is driven by
`cargo release version`... and the changelog/commit/tag/push are orchestrated by the Makefile") may
not pick it up correctly.
**How to avoid:** Land all `BREAKING` CHANGELOG bullets under `## [Unreleased]` first; let
`make release VERSION=0.8.0` (or the equivalent Makefile target) do the heading rename, per the
project's own documented workflow in `release.toml`'s comments.
**Warning signs:** `crates/paladin-web/CHANGELOG.md` has two `## [0.8.0]`-shaped headings, or the
`BREAKING` bullet sits under a heading that predates D-17's actual bump commit.

## Code Examples

### Extending the correspondence test for both capability flags (WEB-03, D-12)

```rust
// Source: crates/paladin-llm/src/lib.rs:98-136 (verified 2026-08-12) — existing test to extend,
// not replace. Add a sibling constant and a second assertion loop in the same function or an
// adjacent #[test] in the same `capability_invariants` module.
#[test]
fn test_capabilities_tool_calling_matches_request_surface() {
    const REQUEST_SURFACE_SUPPORTS_TOOL_CALLING: bool = false;
    // NEW (D-12): the response surface never carries a FunctionCall from any shipped adapter —
    // grep -rn "function_call: Some" --include=*.rs . returns only test doubles (verified 2026-08-12:
    // tests/helpers/mock_llm_adapter.rs:213 and 3 integration/functional test files).
    const RESPONSE_SURFACE_SUPPORTS_FUNCTION_CALLING: bool = false;

    let openai = OpenAIAdapter::new(OpenAIConfig::new("test-key".to_string())).unwrap();
    // ... anthropic, deepseek constructed as today ...

    for (name, declared_tool_calling, declared_function_calling) in [
        ("openai", openai.get_capabilities().supports_tool_calling,
                   openai.get_capabilities().supports_function_calling),
        // ... anthropic, deepseek ...
    ] {
        assert_eq!(declared_tool_calling, REQUEST_SURFACE_SUPPORTS_TOOL_CALLING, /* ... */);
        assert_eq!(declared_function_calling, RESPONSE_SURFACE_SUPPORTS_FUNCTION_CALLING,
            "{name}'s declared supports_function_calling ({declared_function_calling}) must match \
             whether this adapter's generate() ever returns Some(FunctionCall) \
             ({RESPONSE_SURFACE_SUPPORTS_FUNCTION_CALLING})");
    }
}
```

The single-line fix this test then pins in place, at `crates/paladin-llm/src/openai/adapter.rs:650`:
```rust
// Source: crates/paladin-llm/src/openai/adapter.rs:645-651 (verified 2026-08-12)
supports_tool_calling: false,
supports_function_calling: true,  // -> false (D-12): OpenAIAdapter::generate() hardcodes
                                   // function_call: None at :553; this flag currently lies.
```

### Rustdoc reachability statement (WEB-04, D-13)

```rust
// Source: crates/paladin-ports/src/output/llm_port.rs:620 area (verified 2026-08-12) — add a
// rustdoc note to the existing FunctionCall-bearing field, in the project's established
// documentation voice (see the existing temperature_range doc at :832-835 for the pattern of
// stating a precise behavioral guarantee inline).
/// Function call details if the model requested a tool invocation.
///
/// As of this writing, no shipped adapter (OpenAI, Anthropic, DeepSeek, or the bundled mock)
/// ever populates this field — `generate()` always returns `None` here. The reasoning loop's
/// tool-invocation branch (`paladin_execution_service.rs`) is reachable only through a
/// consumer-supplied `LlmPort` implementation that parses tool calls itself. See ADR-0042 for
/// the tracked status of LLM-native tool calling.
pub function_call: Option<FunctionCall>,
```

### Route-literal drift guard (D-15a, Phase 13 hand-off item)

```rust
// Source: crates/doc-examples/src/sidecar.rs (verified 2026-08-12) — add an assertion tying the
// compiled example's literal route to the single source of truth, following the existing
// spec_paths_are_versioned_under_v1 pattern at crates/paladin-web/src/openapi.rs:103.
#[cfg(test)]
mod tests {
    #[test]
    fn sidecar_example_route_matches_api_v1_prefix() {
        assert!(
            "/v1/agents/{agent}/execute".starts_with(paladin_web::agent_controller::API_V1_PREFIX),
            "sidecar.rs's compiled example must stay in sync with API_V1_PREFIX"
        );
    }
}
```
Note: `doc-examples` will need a dev-dependency on `paladin-web` to import `API_V1_PREFIX` if it does
not already have one — check `crates/doc-examples/Cargo.toml` before writing this test; if the
dependency is absent, adding a `[dev-dependencies]` entry for an in-workspace crate is not a new
external package and does not require the Package Legitimacy Gate.

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|---------------|--------|
| M9 Epic 5 §6.1: opaque bearer tokens (chosen, shipped) | M12 Epic 5 FR-2: documented JWT vocabulary layered on top without changing the implementation | M12, prior to this phase | This phase (WEB-01) reconciles the two by keeping M9's implementation and correcting M12's vocabulary — the "current approach" going forward is opaque tokens, documented as such everywhere |
| SemVer 0.x convention: breaking changes may technically bump either patch or minor pre-1.0 | This project's own precedent (D-02, D-04 reversibility notes; D-17): breaking changes bump the **minor** under 0.x, patch is reserved for non-breaking fixes | Established by this phase's own D-17 decision, consistent with the SemVer spec's guidance for `0.y.z` | Plans in this phase and beyond should default to a minor bump for any `.rs` change with a `BREAKING` CHANGELOG entry |

**Deprecated/outdated:** The M12 Epic 5 "JWT" framing (config key `http.auth.jwt.*`, field
`AgentAuthConfig.jwt`, OpenAPI scheme `SEC_JWT`, `bearerFormat: "JWT"`) is deprecated by this phase's
D-01/D-02/D-03/D-04 in favor of vocabulary describing an opaque server-issued bearer token.

## Assumptions Log

| # | Claim | Section | Risk if Wrong |
|---|-------|---------|---------------|
| A1 | SemVer convention that a breaking change under `0.x` bumps the minor (not just the patch) is treated as settled community practice, cited to semver.org's spec language on `0.y.z` releases rather than fetched fresh this session | `## State of the Art`, `## Standard Stack` | Low — D-17 already locks this as a project decision independent of external convention; this is background justification only, not a load-bearing claim for planning |

**Note:** every other substantive claim in this document was verified directly against the shipped
tree in this session (file reads and greps executed 2026-08-12, cited inline with `file:line`) or
copied verbatim from CONTEXT.md's own already-verified findings — there is unusually little
`[ASSUMED]` content in this research because the phase's CONTEXT.md was produced from an exceptionally
thorough tree-scouted discussion.

## Open Questions

1. **Exact replacement identifier vocabulary for the WEB-01 rename**
   - What we know: the mechanism is "opaque server-issued bearer token," verified by a token, not a
     JWT structure; D-00h says medieval-military naming applies only where a domain noun is coined,
     not to HTTP/OpenAPI terms of art.
   - What's unclear: whether the planner should coin a project-specific noun (e.g. a "Warrant" or
     "Writ" in the medieval register) for the token type, or use a plain descriptive name like
     `token_verifier` / `SEC_BEARER_TOKEN` as CONTEXT.md's own illustrative example does.
   - Recommendation: default to the plain descriptive names CONTEXT.md itself illustrates
     (`token_verifier`, a scheme id describing "opaque bearer token" rather than a coined noun) unless
     a later discussion session prefers a themed name — this is explicitly Claude's Discretion, not a
     planning blocker.

2. **Whether `mock.rs` should gain `FunctionCall`-emitting capability**
   - What we know: raised and explicitly left undecided in CONTEXT.md; currently `mock.rs` declares
     both capability flags `false` and never emits a `FunctionCall`, which is already internally
     consistent (no correction needed for WEB-03/04 truthfulness).
   - What's unclear: whether adding this as a constructor option would help demonstrate the tool path
     without a custom adapter, and whether that value is worth touching a shipped test double's
     behavior.
   - Recommendation: leave `mock.rs` unchanged for this phase's WEB-03/04 closure (it is already
     truthful); note the option in ADR-0042 as a possible future step rather than implementing it,
     consistent with D-10's "not built" framing for the tool-calling capability generally.

## Environment Availability

| Dependency | Required By | Available | Version | Fallback |
|------------|------------|-----------|---------|----------|
| `cargo` / `rustc` (edition 2024) | All `.rs` changes | ✓ | cargo 1.97.1, rustc 1.97.1 | — |
| `cargo fmt`, `cargo clippy` | D-19's pre-commit gate | ✓ (bundled with the toolchain) | — | — |
| `cargo-public-api` | `.project/current-exports.txt` regeneration (D-20) | Not directly probed this session; `scripts/extract-public-api.sh` self-installs via `cargo install cargo-public-api` if missing | — | Script's own auto-install path; no additional fallback needed |
| `make` | `make openapi`, `make release`, `make clean-code` targets referenced throughout this phase | Assumed present (standard on the project's CI and dev images; not re-verified this session) | — | Direct `cargo test`/`cargo release` invocations if `make` is unavailable |
| `kubectl` | Only needed to *deploy* `k8s/server/*` manifests for manual verification, not to edit them | Not required for this phase's actual work (manifest edits are plain YAML text changes) | — | — |
| Snyk code scan | CLAUDE.md mandates a scan on new/modified first-party code; this phase modifies the auth surface | Not probed this session — depends on the execution environment's Snyk integration | — | If unavailable, flag for human review before considering the auth-surface changes closed, per `.github/instructions/snyk_rules.instructions.md` |

**Missing dependencies with no fallback:** none identified — every tool this phase needs is either
already present in the verified toolchain or self-installs via the project's existing scripts.

**Missing dependencies with fallback:** `cargo-public-api` — self-installing via
`scripts/extract-public-api.sh`'s own `cargo install cargo-public-api` step if absent.

## Validation Architecture

### Test Framework

| Property | Value |
|----------|-------|
| Framework | Rust built-in `#[test]` / `cargo test`, workspace-wide |
| Config file | none — standard `Cargo.toml` per crate; no `nextest.toml` or custom harness detected |
| Quick run command | `cargo test -p paladin-web`, `cargo test -p paladin-llm` (crate-scoped, fast) |
| Full suite command | `cargo test` (workspace) → `cargo fmt --check` → `cargo clippy -- -D warnings` per CLAUDE.md's mandated pre-commit sequence |

### Phase Requirements → Test Map

| Req ID | Behavior | Test Type | Automated Command | File Exists? |
|--------|----------|-----------|-------------------|-------------|
| WEB-01 | Config key rename doesn't silently accept the old key (no alias) | unit | `cargo test -p paladin --lib config::agents` (the 3 tests at `src/config/agents.rs:306-334`, updated for the new key name) | ✅ existing tests to update |
| WEB-01 | OpenAPI security scheme matches the committed baseline post-rename | integration (drift guard) | `cargo test -p paladin-web openapi_matches_committed_baseline` | ✅ `crates/paladin-web/src/openapi.rs:120-125` |
| WEB-01 | `.project/current-exports.txt` matches the post-rename public surface | integration (CI script) | `./scripts/check-api-surface.sh` | ✅ `scripts/check-api-surface.sh` |
| WEB-02 | Startup warning fires when the in-process store is wired | unit | new test around `build_auth_config` asserting the log line (or, if logging isn't directly assertable, a smoke test on the returned `AgentAuthConfig` plus a manual `grep` check of captured output) | ❌ Wave 0 — no existing test asserts on `build_auth_config`'s log output |
| WEB-02 (D-15b) | Fail-closed posture — `build_auth_config` returns `Err` when enabled with no credentials | unit | new test constructing `AuthConfig { enabled: true, api_keys: vec![], jwt: JwtAuthConfig { enabled: false } }` and asserting `build_auth_config(&cfg).is_err()` | ❌ Wave 0 — `REQ-fail-closed-auth-posture`'s own hand-off text confirms no test currently drives this branch |
| WEB-03 | `supports_function_calling` matches actual reachability, pinned against drift | unit | `cargo test -p paladin-llm test_capabilities_tool_calling_matches_request_surface` (extended) | ✅ existing test to extend, `crates/paladin-llm/src/lib.rs:98-136` |
| D-15a | Sidecar doc-example route matches `API_V1_PREFIX` | unit | `cargo test -p doc-examples sidecar_example_route_matches_api_v1_prefix` (new) | ❌ Wave 0 — no such assertion exists today per the Phase 13 hand-off |

### Sampling Rate
- **Per task commit:** crate-scoped `cargo test -p <crate>` for the crate(s) touched, plus
  `cargo fmt --check` and `cargo clippy -- -D warnings` per CLAUDE.md.
- **Per wave merge:** full `cargo test` (workspace), `./scripts/check-api-surface.sh`,
  `cargo test -p paladin-web openapi_matches_committed_baseline`.
- **Phase gate:** full suite green, both baselines regenerated and matching, before `/gsd-verify-work`.

### Wave 0 Gaps
- [ ] A unit test asserting `build_auth_config`'s WARN fires when `cfg.jwt.enabled` (or its renamed
      equivalent) is `true` — covers WEB-02/D-07.
- [ ] A unit test driving `build_auth_config`'s `Err` branch (enabled, no credentials) — covers
      `REQ-fail-closed-auth-posture` (D-15b).
- [ ] `crates/doc-examples` test asserting the sidecar route literal against `API_V1_PREFIX` — covers
      D-15a. Check whether `crates/doc-examples/Cargo.toml` already has `paladin-web` as a
      dependency/dev-dependency before adding the import.
- [ ] Framework install: none — `cargo test` is already the workspace's test runner; no new framework
      needed.

## Security Domain

### Applicable ASVS Categories

| ASVS Category | Applies | Standard Control |
|---------------|---------|-------------------|
| V2 Authentication | yes | Opaque bearer tokens (D-01), SHA-256-hashed at rest (`InMemoryTokenAuthAdapter::hash_token`), 32 bytes of `rand::thread_rng()` entropy per token, configurable TTL — this phase renames documentation/config around this mechanism but does not change the crypto primitives themselves |
| V3 Session Management | yes (partial — tokens are bearer credentials, not cookie sessions) | Token TTL (default 24h, `DEFAULT_TTL`), explicit revocation via `revoke_token` — unaffected by this phase's renames; verify the ADR (0040) states these properties as the mechanism's security posture |
| V4 Access Control | yes | `Principal.role` (`UserRole`) drives per-agent `allowed_roles` and the admin gate in `agent_auth.rs` — unaffected by renames, but D-04's rename of `Principal.id`'s doc comment must not accidentally weaken or misdescribe this |
| V5 Input Validation | yes | Config deserialization via `serde` with `#[serde(default)]` fallbacks (`agents.rs`) — the renamed `JwtAuthConfig` type keeps the same validation shape, just a new name; no new validation surface introduced |
| V6 Cryptography | yes | SHA-256 (`sha2` crate) for at-rest token hashing, `rand::thread_rng()` (CSPRNG) for token generation — both already correct per the project's own module doc ("a leak of the in-memory store does not reveal usable tokens"); this phase's ADR-0040 should cite this as the reason opaque tokens satisfy V6 without a signing-key story |

### Known Threat Patterns for this stack

| Pattern | STRIDE | Standard Mitigation |
|---------|--------|----------------------|
| Multi-replica token verification failure (a token issued by pod A doesn't verify on pod B) | Denial of Service (non-deterministic auth failures) — arguably also an availability/correctness issue rather than a classic spoofing vector | WEB-02's D-07 startup warning + documented limitation; long-term mitigation is the shared-store `AuthPort` swap (deferred, D-09) |
| Documentation/contract lying about the auth mechanism (JWT documented, opaque tokens shipped) | Spoofing (a consumer building against the documented JWT contract, e.g. expecting stateless verification or a signing public key, builds an insecure or non-functional integration) | WEB-01's full-surface rename (D-01…D-05) — the standard mitigation for a truthfulness defect is truthful documentation, not a code change to match the (weaker) documented mechanism |
| Route-literal drift in compiled doc examples (T-13-20, already identified by Phase 13's security audit) | Spoofing (a reader copying a published example writes a client against a path that doesn't exist, `404`, and may retry against an unversioned/legacy-shaped endpoint if one ever existed) | D-15a's literal-to-`API_V1_PREFIX` assertion — closes T-13-20 from `accept` to `closed`; re-run `/gsd-secure-phase 13` afterward per CONTEXT.md's explicit instruction |
| Capability-flag over-reporting causing a consumer to silently take an insecure or unimplemented code path (a consumer branches on `supports_function_calling: true`, tries to send tool definitions that are silently dropped) | Tampering / Information Disclosure adjacent — not a classic STRIDE auth threat, but a correctness-as-security issue: a consumer trusting a false capability flag may build a security control (e.g. tool-call allowlisting) around a path that never actually executes | WEB-03's D-12 fix + correspondence test — the standard mitigation is making the flag match reality, verified by an automated test that fails on future drift |

## Sources

### Primary (HIGH confidence — verified directly against the shipped tree, 2026-08-12)
- `crates/paladin-web/src/agent_auth.rs` — module docs, `AgentAuthConfig`, `Principal`, `MockJwt`
- `crates/paladin-web/src/openapi.rs` — `decorate()`, `SEC_JWT`, `SEC_API_KEY`, drift-guard test module
- `crates/paladin-web/Cargo.toml` — `utoipa = { version = "5", ... }`, `utoipa-swagger-ui = "9"`
- `src/config/agents.rs` — `JwtAuthConfig`, `AuthConfig`, `default_auth_enabled`
- `src/bin/paladin-server.rs` — `build_auth_config`
- `src/infrastructure/adapters/auth/in_memory_token_auth_adapter.rs` — full module read (SHA-256, `rand`, TTL)
- `crates/paladin-ports/src/output/llm_port.rs` — `ProviderCapabilities`, `LlmResponse.function_call`, `FunctionCall`
- `crates/paladin-llm/src/{openai,anthropic,deepseek}/adapter.rs`, `crates/paladin-llm/src/mock.rs` — all `get_capabilities()` bodies
- `crates/paladin-llm/src/lib.rs:85-136` — `capability_invariants` test module
- `crates/doc-examples/src/sidecar.rs` — full file, line-numbered
- `k8s/server/{configmap,deployment}.yaml`, `k8s/README.md`
- `.planning/decisions/0039-http-topology-no-garrison-no-arsenal.md`, `.planning/decisions/PROMOTION.md`
- `Cargo.toml`, all 11 `crates/*/Cargo.toml`, `release.toml`, `.project/current-exports.txt`,
  `scripts/{check-api-surface,extract-public-api}.sh`
- `crates/paladin-web/CHANGELOG.md`, root `CHANGELOG.md` (`BREAKING` entry precedent at line 173, 231, 824)
- `.planning/REQUIREMENTS.md` — WEB-01…04 full text (lines 2587-2680+) and the Phase-13→14 hand-off block (lines 2417-2507)
- `.planning/codebase/ARCHITECTURE.md` — stale "JWT" mentions at `:120,318`

### Secondary (MEDIUM confidence)
- `.planning/phases/14-api-contract-truthfulness/14-CONTEXT.md` — the phase's own upstream research/discussion output, itself built from a tree-scouted session on 2026-08-11; treated as authoritative user-decision input per this agent's role, not independently re-verified where it cites its own greps (spot-checked several and found consistent)

### Tertiary (LOW confidence)
- SemVer 0.x minor-bump-for-breaking-change convention — general community knowledge, not fetched fresh this session (see Assumptions Log A1); low risk since D-17 already locks the actual project decision independent of this citation

## Metadata

**Confidence breakdown:**
- Standard stack: HIGH — no new dependencies; every library cited was read directly from the shipped `Cargo.toml`/source in this session
- Architecture: HIGH — every pattern and code example is quoted or adapted from code read this session, with `file:line` citations
- Pitfalls: HIGH — derived from the phase's own CONTEXT.md decisions (D-06, D-18, D-20) plus one general project-operational note (worktree cold-build) carried from persistent memory

**Research date:** 2026-08-12
**Valid until:** 30 days (stable, code-verified findings against a tree that changes only through this phase's own planned commits) — re-verify line numbers before executing if significant time passes or if any other phase lands first, per D-05's own "re-grep before acting" instruction.
