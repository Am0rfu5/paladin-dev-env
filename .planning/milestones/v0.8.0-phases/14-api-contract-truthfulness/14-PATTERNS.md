# Phase 14: API Contract Truthfulness - Pattern Map

**Mapped:** 2026-08-12
**Files analyzed:** 16
**Analogs found:** 16 / 16 (this is a rename/correction phase — every "analog" is the file's own
current state, since there is no comparable second implementation elsewhere in the tree; the pattern
to copy is the project's existing rename/warning/ADR/correspondence-test conventions, demonstrated in
these same files or their closest siblings).

## File Classification

| New/Modified File | Role | Data Flow | Closest Analog | Match Quality |
|---|---|---|---|---|
| `crates/paladin-web/src/agent_auth.rs` | middleware | request-response | itself (`Principal`/`AgentAuthConfig` already well-tested, `ct_eq` pattern at `:82-91`) | exact — rename in place |
| `crates/paladin-web/src/openapi.rs` | config/service | request-response | itself (`decorate()` + drift guard `:120-125`) | exact — rename in place |
| `crates/paladin-web/openapi.json` | config (generated artifact) | batch | itself, regenerated via `UPDATE_OPENAPI=1 cargo test` | exact |
| `crates/paladin-web/src/agent_controller.rs` | controller | request-response | `openapi.rs`'s `spec_paths_are_versioned_under_v1` (`:103`) for the assertion-against-constant pattern; handler `security(...)` annotations follow `SEC_JWT`/`SEC_API_KEY` naming | role-match |
| `src/config/agents.rs` | config | CRUD (deserialize) | itself (`JwtAuthConfig`/`AuthConfig`, tests `:306-334`) | exact — rename in place |
| `src/bin/paladin-server.rs` | config/service (binary wiring) | request-response | itself (`build_auth_config`, existing disabled-auth `warn!` at `:146-155`) | exact — D-07's new warning is a sibling of an existing one |
| `crates/paladin-llm/src/openai/adapter.rs` | service (LLM adapter) | request-response | Anthropic/DeepSeek adapters' `get_capabilities()` (both already declare `false` correctly) | exact — same struct literal shape, three siblings agree |
| `crates/paladin-llm/src/lib.rs` (`capability_invariants` mod) | test | transform (correspondence check) | itself, `test_capabilities_tool_calling_matches_request_surface` (`:98-136`) | exact — extend, don't replace |
| `crates/paladin-ports/src/output/llm_port.rs` | model (port DTO) | request-response | itself — existing rustdoc voice on `temperature_range` (`:832-835`ish) and `FunctionCall`/`LlmResponse.function_call` (`:618-620`) | exact — add rustdoc only |
| `crates/doc-examples/src/sidecar.rs` | test/utility (compiled doc example) | request-response | `openapi.rs`'s `spec_paths_are_versioned_under_v1` (`:103`) as the "assert literal against constant" precedent | role-match |
| `.planning/decisions/0040-*.md` (WEB-01) | config (ADR) | batch (document) | `.planning/decisions/0039-http-topology-no-garrison-no-arsenal.md` (most recent ADR, full section shape) | exact |
| `.planning/decisions/0041-*.md` (WEB-02) | config (ADR) | batch (document) | `.planning/decisions/0035-paladin-ml-leaf-crate-placement.md` (reintroduction-condition-without-building precedent, explicitly cited by D-09) | exact |
| `.planning/decisions/0042-*.md` (WEB-04) | config (ADR) | batch (document) | `.planning/decisions/0035-...md` (same deferred-with-trigger shape) + `0039-...md` (Status/Context/Decision/Considered Options/Code Locations/Code Conformance shape) | exact |
| `.project/Deferred-QA-CICD-Completion/prd-deferred-qa-completion.md` | config (source doc) | transform (annotation) | any prior D-00c banner precedent elsewhere in `.project/` (pattern only — dated banner, original retained) | role-match |
| `.planning/ledgers/milestone-09-12.md` (rows `REQ-llm-tool-calling-port`, `REQ-llm-tool-calling-adapters`) | config (ledger) | transform (amend in place) | itself — existing amended rows elsewhere in the same ledger (D-00d pattern) | exact |
| `Cargo.toml` + 11 `crates/*/Cargo.toml`, `CHANGELOG.md` files | config (release) | batch | root `CHANGELOG.md`'s existing `BREAKING` entries (lines 173, 231, 824 per RESEARCH.md) | exact |
| `k8s/server/configmap.yaml`, `k8s/README.md`, `docs/src/deployment-topologies/*` | config/docs | batch | itself — inline comment + doc-limitation pattern already used elsewhere in `k8s/README.md` | role-match |

## Pattern Assignments

### `crates/paladin-web/src/agent_auth.rs` (middleware, request-response)

**Analog:** itself — current shipped file is the analog for what to preserve while renaming.

**Current state to rename** (verified 2026-08-12):
```rust
// Module doc, lines 1-17
//! Two credential types are accepted when auth is enabled:
//! - **API key** via the `X-API-Key` header, ...
//! - **JWT** via `Authorization: Bearer <token>`, verified by the injected [`AuthPort`].

// Principal.id doc, lines 34-37
/// An authenticated caller: an identifier plus the role used for authorization.
pub struct Principal {
    /// Stable identifier (API-key name or JWT subject).
    pub id: String,

// AgentAuthConfig field, lines ~54-60
pub struct AgentAuthConfig {
    pub enabled: bool,
    pub api_keys: HashMap<String, Principal>,
    /// Optional JWT verifier (the `AuthPort` implementation is injected by the binary).
    pub jwt: Option<Arc<dyn AuthPort>>,
```

**Rename target (D-04, exact identifier is Claude's Discretion — CONTEXT.md's own illustrative
example is `token_verifier`):** rename `jwt` field to `token_verifier`, update `Principal.id` doc to
drop "JWT subject" in favor of "opaque bearer-token subject", update module doc's "JWT" bullet to
describe an opaque server-issued bearer token verified by `AuthPort`, and rename the `MockJwt` test
double (search for it — not shown in the excerpt above, present later in the file per D-05's citation
at `:200`).

**Test module to extend, not replace:** the file already has a "complete unit + router-level test
module (constant-time key match, 401 shapes, a redaction test ... health-exempt routing)" per
RESEARCH.md — rename `MockJwt` and its construction sites there; do not add a parallel test module.

---

### `crates/paladin-web/src/openapi.rs` (config/service, request-response)

**Analog:** itself.

**Current state** (verified 2026-08-12, lines 1-58):
```rust
pub const SEC_JWT: &str = "jwt";
...
fn decorate(api: &mut OpenApi) {
    api.info.title = "Paladin Agent API".to_string();
    api.info.version = env!("CARGO_PKG_VERSION").to_string();
    ...
    components.add_security_scheme(
        SEC_JWT,
        SecurityScheme::Http(
            HttpBuilder::new()
                .scheme(HttpAuthScheme::Bearer)
                .bearer_format("JWT")
                .build(),
        ),
    );
}
```

**Rename target (D-03):** `SEC_JWT` → a name describing an opaque bearer token (e.g.
`SEC_BEARER_TOKEN`); drop `.bearer_format("JWT")` entirely (an opaque token has no registered
format). Also update the module doc's "bearer JWT" phrase at `:6`.

**Regeneration command (must run in the same commit, D-03/D-20, then AGAIN after the version bump
per D-18):**
```bash
UPDATE_OPENAPI=1 cargo test -p paladin-web openapi_matches_committed_baseline
# or: make openapi
```

**Existing assertion precedent to reuse for D-15a** (`:103`, `spec_paths_are_versioned_under_v1`) —
the shape D-15a's `sidecar_example_route_matches_api_v1_prefix` test should copy: assert a literal
against a named constant, not against a live-generated value.

---

### `src/config/agents.rs` (config, CRUD/deserialize)

**Analog:** itself.

**Current state** (verified 2026-08-12, lines 89-124):
```rust
/// JWT authentication settings for the agent API.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct JwtAuthConfig {
    /// Whether to accept `Authorization: Bearer` tokens via the wired `AuthPort`.
    #[serde(default)]
    pub enabled: bool,
}

pub struct AuthConfig {
    #[serde(default = "default_auth_enabled")]
    pub enabled: bool,
    #[serde(default)]
    pub api_keys: Vec<ApiKeyConfig>,
    /// JWT bearer-token settings.
    #[serde(default)]
    pub jwt: JwtAuthConfig,
}
```

**Rename target (D-02):** `JwtAuthConfig` → e.g. `TokenAuthConfig`; field `jwt` on `AuthConfig` →
e.g. `token`. **No `#[serde(alias = "jwt")]`** — D-02 explicitly rejects it. Three tests at
`:306-334` (not yet read in this session — re-grep before editing, per D-05's own instruction) need
the same rename.

---

### `src/bin/paladin-server.rs` (config/service, request-response)

**Analog:** itself — the file's own existing disabled-auth warning is the voice for D-07's new one.

**Current state** (verified 2026-08-12, lines 145-199):
```rust
fn build_auth_config(cfg: &AuthConfig) -> Result<AgentAuthConfig, Box<dyn std::error::Error>> {
    if !cfg.enabled {
        warn!(
            "agent API authentication is DISABLED (http.auth.enabled = false) — all agent routes are open"
        );
        return Ok(AgentAuthConfig { enabled: false, api_keys: HashMap::new(), jwt: None });
    }
    let api_keys: HashMap<String, Principal> = cfg.api_keys.iter().map(|k| (
        k.key.clone(), Principal { id: k.name.clone(), role: k.role },
    )).collect();

    // The JWT path reuses the existing AuthPort. ...
    let jwt: Option<Arc<dyn AuthPort>> = if cfg.jwt.enabled {
        Some(Arc::new(InMemoryTokenAuthAdapter::new()))
    } else {
        None
    };

    let auth = AgentAuthConfig { enabled: true, api_keys, jwt };

    if !auth.has_credentials() {
        return Err(
            "authentication is enabled but no credentials are configured: set \
             http.auth.api_keys and/or http.auth.jwt.enabled, or set http.auth.enabled = false"
                .into(),
        );
    }

    info!(
        "agent API authentication ENABLED ({} API key(s){})",
        auth.api_keys.len(),
        if cfg.jwt.enabled { " + JWT" } else { "" }
    );
    Ok(auth)
}
```

**Pattern to add (D-07) — sibling warning, unconditional on the store being wired, matching the
existing warning's voice:**
```rust
let token_verifier: Option<Arc<dyn AuthPort>> = if cfg.token.enabled {
    warn!(
        "in-process bearer-token store enabled — tokens verify only on the issuing process; \
         do not scale past one replica while this store is wired (see ADR-0041)"
    );
    Some(Arc::new(InMemoryTokenAuthAdapter::new()))
} else {
    None
};
```
Also rename the `" + JWT"` log suffix (D-05) and the `cfg.jwt.enabled` branch throughout to the
renamed field, and the comment at `:171-175`.

**Fail-closed test target (D-15b, `REQ-fail-closed-auth-posture`) — no existing test drives this
branch.** Build one against the `Err` arm shown above:
```rust
#[test]
fn build_auth_config_fails_closed_when_enabled_with_no_credentials() {
    let cfg = AuthConfig { enabled: true, api_keys: vec![], token: TokenAuthConfig { enabled: false } };
    assert!(build_auth_config(&cfg).is_err());
}
```
(`build_auth_config` is a private fn in a `src/bin/*.rs` — check whether it needs `#[cfg(test)]`
module placement in the same file, following any existing `#[cfg(test)] mod tests` in
`paladin-server.rs` if present; none was located in the excerpt read this session — re-grep before
writing.)

---

### `crates/paladin-llm/src/openai/adapter.rs` (service/LLM adapter, request-response)

**Analog:** Anthropic/DeepSeek adapters in the same crate, which already declare the flag correctly.

**Current defect** (verified 2026-08-12):
```rust
// :553
function_call: None,
// :642-650
fn get_capabilities(&self) -> ProviderCapabilities {
    ProviderCapabilities {
        ...
        supports_tool_calling: false,
        supports_function_calling: true,   // <- lies; flip to false (D-12)
        ...
    }
}
// :703-710 existing test to update
fn test_get_capabilities() {
    let caps = adapter.get_capabilities();
    ...
    assert!(!caps.supports_tool_calling);
    // add: assert!(!caps.supports_function_calling);
}
```

**Fix:** flip `supports_function_calling: true` → `false` with an inline rationale comment matching
the existing style already used for `supports_tool_calling` elsewhere in this file (per RESEARCH.md's
citation of "The flag describes what this adapter does, not what the vendor's API offers (WEB-03,
D-14)" already committed in the adapters).

---

### `crates/paladin-llm/src/lib.rs` — `capability_invariants` module (test, transform)

**Analog:** itself, `test_capabilities_tool_calling_matches_request_surface` (verified 2026-08-12,
lines 98-136 read in full — reproduced above under Code Context). **Extend this exact function/module,
do not add a parallel one** (D-12 says "extended," RESEARCH.md's Don't-Hand-Roll table repeats this).

**Pattern to add:**
```rust
const RESPONSE_SURFACE_SUPPORTS_FUNCTION_CALLING: bool = false;
// ... inside the same for-loop, add declared_function_calling and a second assert_eq! per adapter,
// following the exact shape of the existing declared/assert_eq! pair for tool_calling.
```

---

### `crates/paladin-ports/src/output/llm_port.rs` (model/port DTO, request-response)

**Analog:** itself — `LlmResponse.function_call` and `FunctionCall` (verified 2026-08-12, lines
600-632, reproduced above under Code Context — no rustdoc currently on `function_call`, just
`/// Function call details if the model requested a tool invocation`).

**Pattern to add (D-13) — reachability rustdoc, matching the project's existing detailed-doc voice
seen elsewhere in this same file (e.g. `FinishReason`'s variant-by-variant doc block just below):**
```rust
/// Function call details if the model requested a tool invocation.
///
/// As of this writing, no shipped adapter (OpenAI, Anthropic, DeepSeek, or the bundled mock)
/// ever populates this field — `generate()` always returns `None` here. The reasoning loop's
/// tool-invocation branch (`paladin_execution_service.rs`) is reachable only through a
/// consumer-supplied `LlmPort` implementation that parses tool calls itself. See ADR-0042 for
/// the tracked status of LLM-native tool calling.
pub function_call: Option<FunctionCall>,
```
Same treatment applies to `ProviderCapabilities`'s doc block (cited at `llm_port.rs:814-837` in
RESEARCH.md — not re-read this session; re-grep before editing).

---

### `crates/doc-examples/src/sidecar.rs` (test/utility, request-response)

**Analog:** `openapi.rs:103`'s `spec_paths_are_versioned_under_v1` — the existing precedent for
pinning a literal against a named constant.

**Current state** (verified 2026-08-12, lines 1-38):
```rust
//! Compiled example for `docs/src/deployment-topologies/sidecar.md` ...
//! Pulled into the page via mdBook `{{#include}}`. ...
pub async fn call_sidecar_agent(
    base_url: &str,
    agent: &str,
    input: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let resp: ExecuteResponse = client
        .post(format!("{base_url}/agents/{agent}/execute"))   // <- unversioned, D-15a fix target
        .json(&ExecuteRequest { input: input.to_string() })
        .send()
        .await?
        .error_for_status()?
```
Doc comment at `:25` currently reads `POST /agents/{id}/execute` — must match the corrected literal.

**Fix (D-15a):** change the literal to `{base_url}/v1/agents/{agent}/execute`, update the `:25` doc
comment to match, and add:
```rust
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
**Check first:** whether `crates/doc-examples/Cargo.toml` already depends on `paladin-web`
(dev-dependency) before adding the import — an in-workspace dev-dependency is not a new external
package and does not require the Package Legitimacy Gate (per RESEARCH.md).

---

### `.planning/decisions/0040-*.md` (WEB-01 ADR)

**Analog:** `.planning/decisions/0039-http-topology-no-garrison-no-arsenal.md` — most recently landed
ADR in this series, same section shape (verified 2026-08-12, lines 1-20+ read): `# ADR-NNNN: Title` /
`## Status` (`Accepted`, `**Date:**`) / `## Context` (prose citing specific `file:line` evidence,
verified-this-session grep output inline) / `## Decision` / `## Considered Options` / `## Code
Locations` / `## Code Conformance` / `## Downstream Consumers`. No frontmatter (D-00a).

**Must state explicitly (D-01):** OQ-4 dissolution — an opaque hashed store needs no signing
secret/algorithm, so the open question is answered by being dissolved, not answered.

---

### `.planning/decisions/0041-*.md` (WEB-02 ADR)

**Analog:** `.planning/decisions/0035-paladin-ml-leaf-crate-placement.md` — the explicit precedent
D-09 cites: "a reintroduction condition promoted into a decision record without building the thing."
Verified shape (lines 1-40 read): `## Status` / `## Context` with numbered measured-facts sub-points
and literal `$ command` / output blocks proving the tree state, in the same voice as D-06's own
"the deviation is the decision" framing.

**Must state explicitly (D-06):** the departure from WEB-02's literal "done when" text (pin
`replicas: 1` or build the shared store) — neither is taken; the shared-store requirement attaches to
the `AuthPort` credential path, not replica count, because `k8s/server/configmap.yaml` ships
`jwt.enabled: false`.

---

### `.planning/decisions/0042-*.md` (WEB-04 ADR)

**Analog:** same two ADRs above — `0035` for the deferred-with-trigger shape, `0039` for the
Status/Context/Decision/Considered-Options/Code-Locations/Code-Conformance section list. RESEARCH.md
already sketches the exact shape to use (§"Pattern 3"):
```markdown
# ADR-0042: LLM-native tool calling — deferred with a named trigger, not built
## Status
Accepted
## Context
[Deferred-QA Epic 27, OQ-1 DeepSeek support, OQ-5 canonical schema, breaking LlmPort change, ADR-0039]
## Decision
[D-10's verbatim user framing]
## Considered Options
- Build Epic 27 now (rejected)
- Withdraw entirely (rejected)
- Record as deferred-with-trigger (chosen — ADR-0035 precedent)
## Code Locations
[.project/.../prd-deferred-qa-completion.md Epic 27 sites, D-11's banner targets]
## Code Conformance
conforms — no code change; the ADR records intent, D-12/D-13 correct current-state claims
```

---

### `.project/Deferred-QA-CICD-Completion/prd-deferred-qa-completion.md` (source doc, annotation)

**Analog:** pattern only (D-00c) — no specific prior banner was read this session; follow the rule
literally: dated banner naming what was wrong and pointing at ADR-0042, original text retained and
marked superseded, applied inline at Epic 27 (`:124,250-298`), the phased-approach note (`:492`), the
priority row (`:557`), and OQ-1/OQ-5 (`:570,574`).

---

### `.planning/ledgers/milestone-09-12.md` rows (ledger, transform)

**Analog:** itself — D-00d's in-place amendment convention, applied to
`REQ-llm-tool-calling-port` / `REQ-llm-tool-calling-adapters`. Amend in place, dated, citing commit
`a2cc1c5` and its `--auto` provenance (D-00i/D-14) — do not create a new row or a separate
corrections file.

---

## Shared Patterns

### Dated in-place amendment, original text retained (D-00c/D-00d)
**Source:** project convention, no single file citation — applies uniformly to `.project/` banners
and `.planning/ledgers/*` rows.
**Apply to:** the Deferred-QA PRD banner, the milestone-09-12 ledger rows, `PROMOTION.md:63`'s
next-free-ADR-number line (update to `0043` once 0040-0042 land).

### Unconditional startup warning naming a scope limitation (D-07)
**Source:** `src/bin/paladin-server.rs:146-155` (existing disabled-auth warning).
**Apply to:** `build_auth_config`'s new token-verifier-enabled warning.

### Assert-literal-against-named-constant (route/version drift guard)
**Source:** `crates/paladin-web/src/openapi.rs:103` (`spec_paths_are_versioned_under_v1`).
**Apply to:** `crates/doc-examples/src/sidecar.rs`'s new `sidecar_example_route_matches_api_v1_prefix`
test (D-15a).

### Extend an existing correspondence test, never fork a parallel one (D-12)
**Source:** `crates/paladin-llm/src/lib.rs:98-136`
(`test_capabilities_tool_calling_matches_request_surface`).
**Apply to:** the new `supports_function_calling` assertion — add a sibling constant and a second
`assert_eq!` inside the same loop/function, not a new test.

### Clean-break rename, no compatibility alias (D-02/D-03/D-04)
**Source:** M8 Epics 4/6 precedent (`use_cases` → `services`, cited in CONTEXT.md D-02, not re-read
this session).
**Apply to:** `http.auth.jwt.*` config key, `JwtAuthConfig`/`AuthConfig.jwt`, `AgentAuthConfig.jwt`,
`SEC_JWT` — every rename in WEB-01 is one-way, no `#[serde(alias = ...)]`.

### Regenerate machine baselines in the same commit that moves them (D-20), twice for OpenAPI (D-18)
**Source:** `crates/paladin-web/src/openapi.rs:120-125` (drift guard), `.project/current-exports.txt`
+ `scripts/check-api-surface.sh`.
**Apply to:** every commit that changes `SEC_JWT`, `AgentAuthConfig.jwt`, or bumps the workspace
version — run `UPDATE_OPENAPI=1 cargo test -p paladin-web openapi_matches_committed_baseline` and
`./scripts/extract-public-api.sh .project/current-exports.txt` in that same commit.

### ADR + dated banner, not a fourth register entry (D-11/D-16)
**Source:** `.planning/decisions/0035-paladin-ml-leaf-crate-placement.md`,
`.planning/decisions/0039-http-topology-no-garrison-no-arsenal.md`.
**Apply to:** ADR-0040/0041/0042 and the Deferred-QA PRD banner.

## No Analog Found

None — every file in this phase's scope is a modification of an existing, already-shipped file, or a
new document (ADR) with two directly-cited precedent ADRs in the same corpus. There is no file in this
phase's scope with zero prior art to model against.

## Metadata

**Analog search scope:** `crates/paladin-web/src`, `crates/paladin-llm/src`,
`crates/paladin-ports/src/output`, `src/config`, `src/bin`, `crates/doc-examples/src`,
`.planning/decisions`, `.planning/ledgers`, `.project/Deferred-QA-CICD-Completion`.
**Files scanned:** 16 target files read directly this session (see line ranges cited inline above),
plus `.planning/decisions/0035-*.md` and `0039-*.md` read for ADR-shape precedent.
**Pattern extraction date:** 2026-08-12
