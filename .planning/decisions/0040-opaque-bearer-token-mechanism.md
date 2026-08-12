# ADR-0040: Opaque server-issued bearer tokens ratified as the agent API's token mechanism

## Status

Accepted

**Date:** 2026-08-12

## Context

Two Milestone PRDs specify incompatible token mechanisms for the same `AuthPort` seam, and
neither document references the other. Milestone 9 Epic 5 §6.1
(`.project/Milestone_9-Classic-Orchestrator-Completion/Epic_5/prd-user-admin-system-completion.md:160-169`)
records: "**Chosen: opaque, randomly-generated bearer tokens with a server-side hashed store.**
Rationale: avoids adding a JWT dependency (`jsonwebtoken`) and a signing-key management story;
supports immediate **revocation** (logout) which stateless JWTs cannot; trivially deterministic
to unit test. … no new dependencies are required." Its own §5 non-goal
(`:149-151`) states plainly: "JWT/OIDC/OAuth or any external identity provider integration."
Milestone 12 Epic 5 FR-2 (`.project/Milestone_12-Web-API/Epic_5/prd-api-security-authorization.md:110`)
then required "a **JWT** via `Authorization: Bearer <token>`, verified through the existing
`AuthPort::verify_token`," layered over the same port without changing the implementation, and
carried the vocabulary into `http.auth.jwt.enabled`, `JwtAuthConfig`, and
`AgentAuthConfig { enabled, api_keys, jwt: Option<Arc<dyn AuthPort>> }`.

This is variant group 29 — recorded in `.planning/REQUIREMENTS.md`'s WEB-01 entry as "the only
variant in five verification runs shipped code could not settle" — because the tree carried both
the Milestone 12 vocabulary and the Milestone 9 mechanism at once: `agent_auth.rs` implemented
the v2 shape (a `jwt` field, a `MockJwt` test double, "bearer JWT checked first" in its own
module docs) while the only concrete implementation behind it was v1's opaque, in-process,
hashed-token store.

Two facts re-verified this session settle which mechanism the tree actually ships. First,
`grep -rn "jsonwebtoken" Cargo.toml crates/*/Cargo.toml` across all twelve workspace manifests
returns no matches (exit 1) — no JWT-signing dependency exists anywhere in the audited graph.
Second, the only implementation of `AuthPort` (`crates/paladin-ports/src/output/auth_port.rs`)
in the workspace is `src/infrastructure/adapters/auth/in_memory_token_auth_adapter.rs`. That
module's own doc comment states its security properties directly: "Tokens are opaque random
strings (not JWTs). Only a SHA-256 hash of each token is stored alongside its claims, so a leak
of the in-memory store does not reveal usable tokens. Tokens carry a configurable time-to-live
and can be explicitly revoked." Reading the module confirms the claim: `hash_token` (`:49-54`)
hashes with `Sha256`; `generate_token` (`:57-61`) draws 32 bytes from
`rand::thread_rng().fill_bytes` before base64-encoding them; `DEFAULT_TTL` (`:24`) is 24 hours;
and `revoke_token` (`:118-126`) removes the stored hash immediately, making a revoked token
invalid on the very next `verify_token` call. That immediate-revocation property is exactly what
Milestone 9's own rationale names as the reason a stateless JWT was rejected — a signed token
cannot be un-signed by a server-side action alone.

## Decision

**Opaque server-issued bearer tokens are the mechanism (D-01).** Milestone 9 Epic 5 §6.1's choice
is ratified, and the Milestone 12 vocabulary is brought into line with it — option (a) of WEB-01's
own "done when" text, not option (b) (adding a real JWT `AuthPort` implementation and answering
Open Question 4).

The correction reached four surfaces, landed by plan 14-01 and re-verified against the tree this
session: the YAML key `http.auth.bearer_token.enabled` (renamed from `http.auth.jwt.enabled`);
`paladin::config::agents::BearerTokenAuthConfig` (renamed from `JwtAuthConfig`) with
`AuthConfig.bearer_token` (renamed from `.jwt`); `paladin_web::AgentAuthConfig.token_verifier`
(renamed from `.jwt`); and `paladin_web::openapi::SEC_BEARER_TOKEN = "bearer_token"` (renamed
from `SEC_JWT = "jwt"`), with the `.bearer_format("JWT")` hint removed entirely because an opaque
token has no registered format to declare.

**Milestone 12 Epic 5's Open Question 4 — "which concrete `AuthPort` impl does `paladin-server`
wire, and what does it need (signing secret/algorithm) from config/env?"
(`.project/Milestone_12-Web-API/Epic_5/prd-api-security-authorization.md:290-292`) — is
dissolved, not answered.** An opaque hashed store has neither a signing secret nor an algorithm
to configure; the question presupposes a signed-token verifier that the shipped adapter is not,
so it has no referent for the code that exists. This record states that in those terms rather
than leaving OQ-4 recorded as open against an adapter that was never going to answer it.

The rename is a clean break: no `#[serde(alias = "jwt")]` was added, so a deployed `config.yml`
naming the superseded `jwt` key fails to deserialize rather than being silently accepted. Both
`CHANGELOG.md:12-21` and `crates/paladin-web/CHANGELOG.md:15-23` carry `BREAKING` entries under
`## [Unreleased]` naming the consumer-visible breaks (`BearerTokenAuthConfig`, `.bearer_token`,
`.token_verifier`, the OpenAPI scheme id) and pointing at this record.

## Considered Options

- keep opaque tokens and correct the Milestone 12 vocabulary, config keys, module documentation and the OpenAPI security scheme (chosen) — matches what the tree already ships (no `jsonwebtoken` dependency, one `AuthPort` implementation) and requires no new cryptographic surface
- add a real signed-token `AuthPort` implementation and answer OQ-4 (rejected — the reversal is costly: a `jsonwebtoken` dependency would enter the audited graph that `cargo audit` and `cargo deny` gate, a signing-key management and rotation story would be needed in config, and the immediate-revocation property the opaque store provides today would be lost, since a stateless signed token cannot be un-signed by a server-side action alone)
- add a compatibility `#[serde(alias = "jwt")]` for the superseded config key (rejected under D-02 — it would leave the untrue word in the accepted input surface with nobody owning its removal)
- bundle this decision with WEB-02's into one auth-mechanism ADR (rejected under D-16 despite the coupling, citing ADR-0034's supersession-granularity cost — a future phase revisiting one verdict would have to supersede a record carrying the other)

## Code Locations

- `crates/paladin-web/src/openapi.rs:27` — `pub const SEC_BEARER_TOKEN: &str = "bearer_token";`, and `:49-52` where the scheme is registered with no `bearer_format` call, re-verified this session
- `crates/paladin-web/src/agent_auth.rs:63` — `pub token_verifier: Option<Arc<dyn AuthPort>>,` on `AgentAuthConfig`, re-verified this session
- `crates/paladin-web/src/agent_auth.rs:203` — `struct MockTokenVerifier {`, the renamed test double, re-verified this session
- `src/config/agents.rs:92` — `pub struct BearerTokenAuthConfig`, and `:112` `pub bearer_token: BearerTokenAuthConfig,` on `AuthConfig`, re-verified this session
- `src/bin/paladin-server.rs:159` — `fn build_auth_config(cfg: &AuthConfig)`, the wiring seam where the renamed config field becomes the web layer's auth state, re-verified this session
- `src/infrastructure/adapters/auth/in_memory_token_auth_adapter.rs` — the sole `AuthPort` implementation in the workspace, whose module doc and hashing/generation functions this record's Context quotes directly
- `crates/paladin-web/openapi.json` and `.project/current-exports.txt` — the two regenerated machine baselines; `.project/current-exports.txt:1801` confirms `pub struct paladin::config::agents::BearerTokenAuthConfig` in the public API surface

## Code Conformance

conforms

The code was changed to match this decision by plan 14-01, with the commands that prove it
recorded verbatim: `grep -rn "jsonwebtoken" Cargo.toml crates/*/Cargo.toml` (no matches, re-run
this session); `UPDATE_OPENAPI=1 cargo test -p paladin-web openapi_matches_committed_baseline`
(the drift guard, run twice and diffed byte-identical per `14-01-SUMMARY.md`);
`./scripts/check-api-surface.sh` (the `api-surface` guard); `cargo test -p paladin-ai --lib
config::agents` (7 tests, pass); and `cargo test -p paladin-web` (117 unit + 5 integration tests
pass, including `openapi::tests::openapi_matches_committed_baseline`).

## Downstream Consumers

- **ADR-0041** — scopes the store this mechanism uses (single-replica serving today, a
  shared-store swap deferred with a named trigger).
- **The `REQ-opaque-bearer-token-adapter-v1` and `REQ-jwt-bearer-auth-v2` ledger rows** in
  `.planning/ledgers/milestone-09-12.md` — plan 14-07 amends them against this record.
- **Generated OpenAPI clients** keying their security requirements off the `bearer_token` scheme
  id published in `crates/paladin-web/openapi.json`.
- **Any deployed configuration file** carrying the superseded `http.auth.jwt.*` key — it now
  fails to deserialize rather than being silently accepted.
