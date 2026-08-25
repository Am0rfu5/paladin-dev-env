# Phase 14: API Contract Truthfulness - Context

**Gathered:** 2026-08-11
**Status:** Ready for planning
**Mode:** interactive — every decision below was selected by a human, except where marked
*Claude's Discretion*. One answer (**D-10**) was given as a principle rather than a menu choice and is
recorded verbatim in `<specifics>` finding 5.

**Four gray areas were presented and all four selected:** the token mechanism (WEB-01) · the
multi-replica store (WEB-02) · tool calling (WEB-03/WEB-04) · phase boundary and blast radius.

**Five findings from this session's tree scout change the shape of the phase.** They are numbered in
`<specifics>`, each verified on 2026-08-11. In summary: WEB-03's substance already shipped in Phase 2
and only its residue and its record are open (1); one capability flag is still asymmetric and the
defect behind it is larger than the flag — **no shipped adapter ever emits a `FunctionCall`**, so the
reasoning loop's entire tool branch is unreachable through the three shipped providers (2); WEB-02's
correctness edge is gated by a config flag the shipped ConfigMap turns **off**, not by the replica
count (3); WEB-02's own citation names the wrong pair of manifests (4); and the user's answer on
WEB-04 sets a general principle for this phase, not just a verdict on Epic 27 (5).

<domain>
## Phase Boundary

Make every capability this project advertises through an interface one it actually has — so a
developer reading the auth contract, deploying the Kubernetes manifests, or branching on a provider
capability flag gets the behaviour the interface promised. Four requirements, WEB-01 … WEB-04, plus
two items Phase 13's hand-off assigned to this phase by name.

**Five deliverable classes:**

1. **One recorded token mechanism, applied across every surface that names it** (WEB-01) — opaque
   server-issued tokens ratified; the word "JWT" removed from the config key, the public Rust field,
   the OpenAPI security scheme, the module documentation and the shipped example configs, with the
   two machine baselines it moves regenerated in the same commits.
2. **A multi-replica answer that matches the shipped topology** (WEB-02) — the shared-store
   requirement scoped to the `AuthPort` credential path, a startup warning when the in-process store
   is wired, the limitation stated in the deployment artefacts and docs, and the shared store itself
   deferred with a named trigger.
3. **Honest capability flags and an honest reachability statement** (WEB-03) — `supports_function_calling`
   brought into line with `supports_tool_calling`, the correspondence test extended to cover both, and
   the fact that LLM-initiated tool calling requires a consumer's own `LlmPort` implementation stated
   everywhere the documentation currently implies otherwise.
4. **Three ADRs — 0040, 0041, 0042** (WEB-01, WEB-02, WEB-04), plus a dated correction banner on the
   Deferred-QA Epic 27 source under `.project/`. WEB-03 gets no ADR: it is a code-settled defect
   (D-00g).
5. **Release bookkeeping for a breaking change** — `BREAKING` CHANGELOG entries and a lockstep
   version bump to **0.8.0** across all twelve manifests.

**Also in this phase, by Phase 13's hand-off (D-15):**

- `crates/doc-examples/src/sidecar.rs:25,34` — the unversioned agent route a rendered mdbook page
  teaches, with an assertion tying the literal to `paladin_web::agent_controller::API_V1_PREFIX`.
  Landing it moves threat **T-13-20** from accepted residual risk **AR-13-01** to closed.
- `REQ-fail-closed-auth-posture` — the missing test driving the `Err` branch of the server's
  fail-closed startup check.

**Not in this phase:**

- **Implementing LLM-native tool calling (Deferred-QA Epic 27).** D-10 records it as a future
  capability improvement with a trigger and an owner. Building it is a breaking `LlmPort` change
  across three adapters and the mock, and belongs to a milestone that schedules it.
- **Building the shared-store `AuthPort`.** D-09 records the deferral and its trigger. M9 §6.2 states
  the port was designed to permit the swap, so the future work is an adapter, not a redesign.
- **General documentation content-currency.** The doc sweep in D-13 is scoped to *tool-call
  reachability claims only*. Milestone 11's fourteen content-currency files are DOCS-01, Phase 16.
- **Annotating the root `k8s/deployment.yaml` placeholder.** D-08 leaves it alone; it runs
  `image: paladin:test` under `sleep 3600` with its probes commented out and serves no API.
- **CI and coverage gates.** PIPE-01 … PIPE-05 are Phase 15.

</domain>

<decisions>
## Implementation Decisions

### Inherited from Phases 1, 5, 7, 8, 9, 10, 12 and 13 — locked, not re-litigated

- **D-00a:** ADRs live in `.planning/decisions/`, flat sequential numbering, file shape
  `Status / Context / Decision / Considered Options / Code Locations / Code Conformance /
  Downstream Consumers`, **no frontmatter** (matching 0001-0039). **`PROMOTION.md:63` records 0040 as
  next free** — verified this session. Update that line when the ADRs land.
- **D-00b:** Precedence order is **ADR → shipped tree → `.planning/codebase/` map →
  `intel/code-verification.md` → PRD → DOC → task-list checkbox.** An ADR that contradicts shipped
  code is an instruction to change the code. *(Phase 1 D-02.)* **This phase is the first since Phase 4
  where that last clause bites** — three ADRs here instruct `.rs` changes.
- **D-00c:** Source corrections under `.project/` are **annotation, not rewriting** — a dated
  correction banner naming what was wrong and pointing at the ADR or requirement, each defective claim
  corrected inline with the original text retained and marked superseded. *(Phase 5 D-08.)*
- **D-00d:** Ledgers are **amended in place**, dated, superseded text retained. Never a separate
  corrections file. *(Phase 2 D-02.)*
- **D-00e:** Evidence bar: no claim of closure without the exact command or `file:line` that produced
  it, recorded verbatim. *(Phases 3, 5, 7, 8, 9, 10, 12, 13.)*
- **D-00f:** Primary key is the `REQ-*` ID; outstanding task items nest under the requirement they
  belong to rather than getting invented identifiers.
- **D-00g:** Contested positions get ADRs; code-settled defects get ledger rows and no ADR.
  *(Phase 7 D-17.)*
- **D-00h [informational]:** Medieval-military ubiquitous language is mandatory in code, docs and
  comments. *(CLAUDE.md.)*
- **D-00i:** Provenance of `--auto` decisions is carried forward rather than laundered. *(Phase 12
  hand-off item 6.)* **Applies directly here** — WEB-03's substance landed in Phase 2 plan 02-02 under
  `--auto`; see D-11.
- **D-00j:** ADR-0039 already ratifies the absence of Garrison and Arsenal on HTTP-served agents as a
  **permanent property of the shipped topology**. It supplies the HTTP half of the Arsenal/LlmPort
  relationship WEB-04 requires; this phase supplies the other half (D-10).
- **D-00k:** ADR-0037 fixes the agent route surface at `/v1`; `API_V1_PREFIX` is the single source
  (`crates/paladin-web/src/agent_controller.rs:723`), asserted by `spec_paths_are_versioned_under_v1`
  (`crates/paladin-web/src/openapi.rs:103`). D-15's sidecar fix conforms to it rather than re-deciding it.

---

### WEB-01 — the token mechanism and how far the correction travels

- **D-01: Opaque server-issued tokens are the mechanism.** M9 Epic 5 §6.1's choice is ratified and
  the Milestone 12 vocabulary is brought into line with it — option (a) of WEB-01's own "done when".
  The tree backs this unambiguously: `grep -rn "jsonwebtoken" Cargo.toml crates/*/Cargo.toml` returns
  nothing, the only `AuthPort` implementation is
  `src/infrastructure/adapters/auth/in_memory_token_auth_adapter.rs`, and M9 §5 lists JWT/OIDC/OAuth
  as an explicit non-goal with immediate revocation as the stated reason a stateless JWT could not
  provide. **M12 Epic 5's Open Question 4** ("which concrete `AuthPort` impl does `paladin-server`
  wire, and what does it need — signing secret/algorithm — from config/env?") is answered by being
  **dissolved**: an opaque hashed store needs neither, and the ADR must say so in those terms rather
  than leave OQ-4 recorded Open.
  — **Reversibility:** costly — reversing means adding a `jsonwebtoken` dependency to the audited
  graph, a signing-key management and rotation story in config, and giving up the immediate-revocation
  property; the port itself does not have to change.

- **D-02: The config key is renamed clean, with no serde alias.** `http.auth.jwt.*` and the
  `JwtAuthConfig` type get names that describe an opaque bearer-token verifier; existing v0.6.0/v0.7.0
  config files that set the old key will fail to load, and that break is recorded in the CHANGELOG.
  Follows this project's own precedent — Milestone 8 Epics 4 and 6 renamed `use_cases` → `services` as
  an explicit clean break with no compatibility alias, and a workspace grep for the old name returns
  zero. A `#[serde(alias = "jwt")]` was considered and rejected: it would leave the untrue word in the
  accepted input surface with nobody owning its removal.
  — **Reversibility:** one-way — it breaks every deployed `config.yml` that names the old key; undoing
  it is a second break or the alias this decision declined.

- **D-03:** The OpenAPI security scheme is renamed and `bearerFormat: "JWT"` is dropped. `SEC_JWT`
  (`crates/paladin-web/src/openapi.rs:27`) becomes a name describing an opaque bearer token, the
  `.bearer_format("JWT")` hint at `:54` is removed (an opaque token has no format), and every handler
  `security(...)` annotation that names the scheme follows. **The committed baseline
  `crates/paladin-web/openapi.json` is regenerated in the same commit** so the drift guard at
  `crates/paladin-web/src/openapi.rs:120-125` stays green.
  — **Reversibility:** one-way — the scheme id is a published contract identifier that generated
  clients key their security requirements off.

- **D-04: The public Rust field is renamed and the break is recorded.**
  `AgentAuthConfig.jwt: Option<Arc<dyn AuthPort>>` (`crates/paladin-web/src/agent_auth.rs:60`) is
  renamed to something true (e.g. `token_verifier`), the module documentation at `:1-18`, the
  `Principal.id` doc ("API-key name or JWT subject", `:36-37`), the `bearer JWT checked first` comment
  at `:121`, and the `MockJwt` test double at `:200` all follow. `paladin-web`'s CHANGELOG gets a
  `BREAKING` entry, and **`.project/current-exports.txt` is regenerated** so the `api-surface` guard
  passes.
  — **Reversibility:** one-way — a semver-visible break in a published crate.

- **D-05:** The correction reaches every surface that names the mechanism, and the plan must
  enumerate them. Verified sites as of 2026-08-11: `crates/paladin-web/src/agent_auth.rs` (module docs, field,
  `Principal` doc, comment, test double), `crates/paladin-web/src/openapi.rs:6,26-27,49-57`,
  `crates/paladin-web/src/agent_controller.rs` (handler `security(...)` annotations),
  `src/config/agents.rs:90-112` (`JwtAuthConfig`, `AuthConfig.jwt`, and the three tests at `:306-334`),
  `src/bin/paladin-server.rs:171-199` (the wiring comment, the `cfg.jwt.enabled` branch and the
  `" + JWT"` log suffix), `config.example.yml:58-59`, `k8s/server/configmap.yaml`, and the
  deployment-topology pages under `docs/src/`. Re-grep before acting rather than trusting these line
  numbers (D-00e's sibling rule from Phase 13's hand-off).

---

### WEB-02 — the multi-replica store

- **D-06:** The shared-store requirement attaches to the `AuthPort` credential path, not to the
  replica count — replicas stay as shipped. Neither of WEB-02's two literal exits is taken, and **the
  deviation is the decision**: `k8s/server/configmap.yaml` sets `jwt.enabled: false` and authenticates
  with static API keys sourced from a Secret, which are byte-identical in every pod, so
  `k8s/server/deployment.yaml:14`'s `replicas: 2` is correct today. Pinning `replicas: 1` would degrade
  a working deployment to guard a code path the shipped configuration has turned off. ADR-0041 must
  state this reasoning explicitly, because it departs from the requirement's own "done when" text.
  — **Reversibility:** reversible — pinning replicas later is a one-line manifest change.

- **D-07: When the in-process store is wired, the process says so at startup.** `build_auth_config`
  (`src/bin/paladin-server.rs:145-199`) logs a WARN whenever the `AuthPort` verifier is enabled, naming
  the constraint — tokens verify only on the issuing process; do not scale past one replica with this
  store. A pod cannot read its own `spec.replicas` without Kubernetes API access, so the warning is
  unconditional on the store being wired rather than conditional on replica count. Paired with an
  inline comment where `k8s/server/configmap.yaml` turns it off, a note in `k8s/README.md`, and the
  limitation stated on the deployment-topology pages — which is what satisfies ROADMAP criterion 2's
  "the deployment artefacts and documentation say it will not". Refusing to start behind a new opt-in
  flag was considered and rejected as a knob whose only job is to be typed once.
  — **Reversibility:** reversible.

- **D-08: WEB-02's manifest citation is corrected at source; the root placeholder is left alone.**
  WEB-02 names `k8s/deployment.yaml` and `k8s/service.yaml`; the Milestone 12 Epic 7 artefacts it
  describes are `k8s/server/deployment.yaml` and `k8s/server/service.yaml`. A dated correction banner
  goes on WEB-02 per D-00c/D-00d with the original text retained. The root `k8s/deployment.yaml` gets
  nothing: it runs `image: paladin:test` with `args: ["-c", "echo 'Paladin started' && sleep 3600"]`
  and its liveness/readiness probes are commented out (`:139-167`), so nothing in it can issue or
  verify a token. *(Annotating it as a placeholder was offered and declined as out of scope.)*
  — **Reversibility:** reversible.

- **D-09: The shared store is deferred with a named trigger.** The reintroduction condition is **the
  first deployment that needs more than one replica serving `AuthPort`-issued tokens**, citing M9 §6.2's
  statement that the port was designed to permit exactly this swap. Recorded in ADR-0041 following the
  ADR-0035 precedent — a reintroduction condition promoted into a decision record without building the
  thing. Deliberately *not* recorded as a permanent property of the topology (the ADR-0039 treatment),
  because the capability is wanted, just not now.
  — **Reversibility:** reversible.

---

### WEB-03 / WEB-04 — capability flags and tool calling

- **D-10:** LLM-native tool calling (Deferred-QA Epic 27) is recorded as a future capability
  improvement, not built. The user's answer sets the frame for the whole phase and is recorded
  verbatim in `<specifics>` finding 5: *"We want to maximize the capabilities so this sounds like a
  future feature improvement. This is listed under 'deferred'. … this is the source of some potential
  future version improvements not any current functionality. This should be recorded as such and
  everything should properly reflect current functionality."* So: **Epic 27 is out of current scope,
  and every surface describing today's behaviour is corrected to describe today's behaviour.** The
  reasons on the record are Arsenal/MCP already providing tool execution through a different seam
  (ADR-0039 supplying the HTTP half of that relationship, D-00j), the breaking `LlmPort` change the
  PRD itself flags, and both of Epic 27's open questions still unanswered (OQ-1 DeepSeek support,
  OQ-5 canonical schema, `prd-deferred-qa-completion.md:570,574`). ADR-0042 carries a named
  reintroduction trigger and an owner.
  — **Reversibility:** reversible — nothing is deleted; the trigger is what brings it back.

- **D-11: WEB-04's recording must not be a fourth register entry, and the record says so.** ADR-0042
  is the record; a **dated correction banner** goes on the Deferred-QA Epic 27 source
  (`.project/Deferred-QA-CICD-Completion/prd-deferred-qa-completion.md`, Epic 27 at `:124,250-298`,
  the phased-approach note at `:492`, the priority row at `:557`, and OQ-1/OQ-5 at `:570,574`) pointing
  at it, per D-00c with the original text retained. WEB-04's own text warns it is "not done by leaving
  it as a deferred register entry for a fourth time" — the banner is what makes the difference: the
  next reader of Epic 27 meets the decision instead of an unbuilt epic.

- **D-12:** `supports_function_calling` is brought into line, and the correspondence test covers both
  flags. OpenAI declares `supports_function_calling: true`
  (`crates/paladin-llm/src/openai/adapter.rs:650`) while hardcoding `function_call: None` at `:553`;
  Anthropic (`:551`) and DeepSeek (`:808`) declare `false`. The flag is flipped to `false` and the
  existing correspondence test `test_capabilities_tool_calling_matches_request_surface`
  (`crates/paladin-llm/src/lib.rs:98-136`) is extended to pin **both** flags to the request/response
  surface, so the two cannot drift apart again. This follows the rationale already committed in the
  adapters: *"The flag describes what this adapter does, not what the vendor's API offers (WEB-03,
  D-14)"*. Leaving it `true` as a vendor-capability statement was considered and rejected — it would
  put two opposite conventions in one struct.
  — **Reversibility:** reversible.

- **D-13: The reachability limitation is stated everywhere it is currently implied.** `function_call:
  Some(...)` appears **only in test doubles** (`tests/helpers/mock_llm_adapter.rs:213`,
  `tests/functional/paladin_tool_invocation_test.rs`, `tests/integration/context_injection_test.rs`,
  `tests/integration/arsenal_bridge_regression_test.rs`). No shipped adapter — OpenAI, Anthropic,
  DeepSeek or `crates/paladin-llm/src/mock.rs` — ever returns one, so the reasoning loop's tool branch
  at `src/application/services/paladin/paladin_execution_service.rs:799` (Arsenal invocation *and* the
  `handoff_to_specialist` path at `:1414`) fires only for a consumer supplying their own `LlmPort`.
  The limitation is stated in rustdoc on `ProviderCapabilities` / `LlmResponse.function_call`
  (`crates/paladin-ports/src/output/llm_port.rs:620,815-837`) **and** on the pages that imply
  otherwise: `docs/src/user-guides/tool-integration.md`, `docs/src/architecture/overview.md`,
  `docs/src/architecture/domain-model.md`, `docs/src/contributing/contributing-providers.md`.
  **Scope guard:** this sweep is limited to tool-call reachability claims. General content currency of
  Milestone 11 pages is DOCS-01, Phase 16 — do not widen into it.
  — **Reversibility:** reversible.

- **D-14: WEB-03's already-shipped half is closed with its provenance, not re-implemented.** Commit
  `a2cc1c5` ("feat(02-02): flip supports_tool_calling honest and declare OpenAI/Anthropic ranges")
  already set `supports_tool_calling: false` on all three adapters with an inline `WEB-03, D-14`
  rationale and added the correspondence test. The requirement checkbox and the
  `.planning/ledgers/milestone-09-12.md` rows `REQ-llm-tool-calling-port` /
  `REQ-llm-tool-calling-adapters` still read `Verified open → WEB-03/WEB-04`. They are amended in place
  per D-00d, dated, citing the commit — and per D-00i the closure records that plan 02-02 ran under
  `--auto`, rather than presenting it as a bare `Complete`.

---

### Cross-cutting — boundary, records and release

- **D-15: Both of Phase 13's hand-off items are in scope.** (a) `crates/doc-examples/src/sidecar.rs:34`
  becomes `{base_url}/v1/agents/{agent}/execute` with the `:25` doc comment matching, paired with an
  assertion tying the literal back to `paladin_web::agent_controller::API_V1_PREFIX` — `cargo check`
  cannot catch an opaque string literal, and without the assertion the drift silently returns on the
  next prefix change. Landing it moves threat **T-13-20** from accepted residual risk **AR-13-01** to
  closed; re-run `/gsd-secure-phase 13` afterwards to record that. (b) `REQ-fail-closed-auth-posture`
  gets the test that drives the `Err` branch of `build_auth_config`
  (`src/bin/paladin-server.rs:145-199`) and observes a real refusal — the code path exists and matches
  its requirement's shape, but nothing exercises it, so under D-03 it cannot be marked `Shipped`.
  **Method note carried forward from Phase 13, do not redraw:** an assertion of the form "zero
  occurrences of X remain in `page.md`" is unsound for a page that uses mdBook `{{#include}}`. Sweep
  the include targets as well as the including page, or assert against rendered `docs/book/` output.

- **D-16: Three ADRs — 0040 (WEB-01), 0041 (WEB-02), 0042 (WEB-04).** One decision per record, the
  finest supersession unit. Phase 11 recorded the cost of the alternative: bundling D1-D4 into ADR-0034
  means a future phase revisiting one verdict must supersede a record carrying three others. WEB-03
  gets **no ADR** — a code-settled defect (D-00g). Bundling WEB-01 and WEB-02 into a single
  auth-mechanism ADR was considered and declined despite their coupling.

- **D-17:** Release bookkeeping lands in this phase — `BREAKING` CHANGELOG entries and a lockstep
  bump to 0.8.0. All twelve manifests currently read `0.7.0`; `release.toml:17` sets `shared-version = true`,
  so the bump moves them together. **0.8.0, not 0.7.2:** under SemVer for `0.x` a breaking change bumps
  the minor, and D-02 and D-04 each break consumers — shipping two renames under a patch version would
  be the same class of untruth this phase exists to close. *(The cost flagged at decision time —
  `paladin-herald` having no CHANGELOG, SEC-04 — no longer applies: `crates/paladin-herald/CHANGELOG.md`
  exists in the tree as of this session.)* Phases 15 and 16 then ship under 0.8.0.
  — **Reversibility:** costly — a bump is cheap to revert before a tag exists, but `release.toml`'s
  tag-triggered publishing and lockstep rule mean an accidental release is not revertible.

- **D-18: Ordering constraint — the version bump and the OpenAPI regeneration are coupled.**
  `crates/paladin-web/src/openapi.rs:37` sets `api.info.version` from `env!("CARGO_PKG_VERSION")`, so
  the 0.8.0 bump moves the committed `openapi.json` baseline a **second** time, after D-03's scheme
  rename already moved it. Either land the bump and the regeneration in the same commit, or sequence
  the bump last and regenerate once more after it. A plan that regenerates only after D-03 will leave
  the drift guard red.

- **D-19: This phase changes `.rs` in published crates — the opposite of Phase 13's boundary.** Phases
  5-13 held a zero-`.rs` boundary; that rule does **not** carry forward. Expected surfaces:
  `crates/paladin-web/src/{agent_auth,openapi,agent_controller}.rs`, `src/config/agents.rs`,
  `src/bin/paladin-server.rs`, `crates/paladin-llm/src/{openai/adapter,lib}.rs`,
  `crates/paladin-ports/src/output/llm_port.rs` (rustdoc), `crates/doc-examples/src/sidecar.rs`, plus
  `config.example.yml`, `k8s/server/*`, `k8s/README.md`, `docs/src/**`, `.project/` banners and
  `.planning/` records. Every `.rs` change goes through `cargo test` → `cargo fmt --check` →
  `cargo clippy -- -D warnings` per CLAUDE.md before commit.

- **D-20: Two machine baselines move and both are regenerated in the commit that moves them** —
  `crates/paladin-web/openapi.json` (drift guard, `crates/paladin-web/src/openapi.rs:120-125`) and
  `.project/current-exports.txt` (the `api-surface` guard read by `scripts/check-api-surface.sh`). A
  commit that changes the public surface without its baseline leaves CI red for every later plan in
  the phase.

### Claude's Discretion

- **Exact replacement identifiers.** The new names for `SEC_JWT`, `AgentAuthConfig.jwt`,
  `JwtAuthConfig` and the `http.auth.jwt.*` key are unspecified. Pick one vocabulary and use it in all
  four places; prefer a term that survives a future shared-store swap (the mechanism is "opaque
  server-issued bearer token", the component is a verifier). D-00h applies — the medieval-military
  register is mandatory where a domain noun is being coined, not where an HTTP/OpenAPI term of art is.
- **Whether the shipped `crates/paladin-llm/src/mock.rs` should be able to emit a `FunctionCall`.**
  Raised and not decided. It would make the tool path demonstrable without a custom adapter; it also
  changes a shipped test double's behaviour. Planner's call, and out of scope if it grows past a
  constructor option.
- **Plan decomposition and wave structure.** WEB-03's residue (D-12) and D-15's two items are
  independent of WEB-01 and can run in parallel; D-17/D-18's bump must land last or be sequenced with
  the final regeneration.
- **Ledger and requirement amendment mechanics.** Which rows in
  `.planning/ledgers/milestone-09-12.md` carry the closure versus the pointer, within D-00d/D-00f.
- **Whether ADR-0040 also records M12 Epic 5's OQ-4 as dissolved in the OQ table's own location**, or
  only in the ADR.

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Phase scope and requirements

- `.planning/ROADMAP.md` — Phase 14 entry: goal, dependencies, the four success criteria. **Read the
  criteria literally; D-06 deviates from WEB-02's "done when" and the ADR must justify it.**
- `.planning/REQUIREMENTS.md` §"API contract truthfulness (WEB)" (WEB-01 … WEB-04) — the requirement
  text this phase closes and, per D-08, amends at source.
- `.planning/REQUIREMENTS.md` §"Hand-off to Phase 14 / WEB-01 … WEB-04 — dated 2026-08-10 (plan 13-13)"
  — six numbered items; items 5 and 6 are what D-15 takes on, including the T-13-20 / AR-13-01
  security disposition and the `{{#include}}` method note.
- `.planning/PROJECT.md` §Active — this phase's place in the Milestone 9-12 + Deferred-QA close-out.

### Decisions this phase applies but does not re-open

- `.planning/decisions/0037-agent-route-surface-v1.md` — `/v1` is the agent route surface; D-15's
  sidecar fix conforms to it.
- `.planning/decisions/0038-agent-provisioner-placement.md` — `AgentProvisioner` stays in
  `paladin-web`; its `## Downstream Consumers` names the accepted cost. Cite it if the seam is revisited.
- `.planning/decisions/0039-http-topology-no-garrison-no-arsenal.md` — the absence of Garrison and
  Arsenal on HTTP-served agents is a permanent property of the topology; supplies the HTTP half of
  WEB-04's required Arsenal/LlmPort relationship (D-00j).
- `.planning/decisions/0035-paladin-ml-leaf-crate-placement.md` — the precedent D-09 follows: a
  reintroduction condition recorded without building the thing.
- `.planning/decisions/0034-d1-d4-facade-relocation-disposition.md` — the precedent D-16 avoids: a
  bundled ADR and its coarser supersession unit.
- `.planning/decisions/PROMOTION.md:63` — **next free ADR number: 0040.** Update this line when
  0040-0042 land; the file's own procedure is at `:209-218`.

### Evidence and ledger

- `.planning/ledgers/milestone-09-12.md` — rows `REQ-opaque-bearer-token-adapter-v1`,
  `REQ-jwt-bearer-auth-v2`, `REQ-k8s-manifests`, `REQ-health-ready-endpoints`,
  `REQ-fail-closed-auth-posture`, `REQ-llm-tool-calling-port`, `REQ-llm-tool-calling-adapters`. Amended
  in place per D-00d.
- `.planning/phases/13-milestone-9-12-ground-truth-recorded-account/13-REVIEW.md` CR-01 — the sidecar
  residue's origin.
- `.planning/phases/13-milestone-9-12-ground-truth-recorded-account/13-SECURITY.md` — threat T-13-20 and
  accepted residual risk AR-13-01.

### Source documents this phase corrects

- `.project/Deferred-QA-CICD-Completion/prd-deferred-qa-completion.md` — Epic 27 at `:124` and
  `:250-298`; the breaking-change/phased-approach note at `:492`; the priority row at `:557`; OQ-1 and
  OQ-5 at `:570,574`. Receives D-11's dated banner.
- M12 Epic 5's requirement text (`REQ-jwt-bearer-auth-v2` provenance) — the JWT specification D-01
  supersedes, and the home of Open Question 4.

### Code sites — all verified 2026-08-11

**Auth surface (WEB-01, WEB-02, D-15b):**
- `crates/paladin-web/src/agent_auth.rs` — module docs `:1-18`, `AgentAuthConfig.jwt` `:60`,
  `Principal.id` doc `:36-37`, `bearer JWT checked first` `:121`, `MockJwt` `:200`.
- `crates/paladin-web/src/openapi.rs` — `SEC_JWT` `:27`, `bearer_format("JWT")` `:54`,
  `api.info.version` from `CARGO_PKG_VERSION` `:37`, drift guard `:120-125`,
  `spec_paths_are_versioned_under_v1` `:103`.
- `crates/paladin-web/openapi.json` — the committed drift baseline (D-03, D-18, D-20).
- `src/config/agents.rs` — `JwtAuthConfig` `:90-97`, `AuthConfig` `:98-124`, tests `:306-334`.
- `src/bin/paladin-server.rs` — `build_auth_config` `:145-199`, the wiring comment `:171-175`,
  the `" + JWT"` log suffix `:195-198`.
- `src/infrastructure/adapters/auth/in_memory_token_auth_adapter.rs` — the only `AuthPort`
  implementation in the workspace.
- `crates/paladin-ports/src/output/auth_port.rs` — `AuthPort`, `AuthClaims`, `AuthToken`, `AuthError`.
- `crates/paladin-web/tests/auth_rbac.rs` — the existing auth integration test surface.
- `config.example.yml:47-59`, `k8s/server/configmap.yaml`, `k8s/server/deployment.yaml:14`,
  `k8s/README.md`.

**Capability flags and tool reachability (WEB-03, WEB-04):**
- `crates/paladin-ports/src/output/llm_port.rs` — `ProviderCapabilities` `:814-837`,
  `LlmResponse.function_call` `:620`, `FunctionCall` `:628`.
- `crates/paladin-llm/src/openai/adapter.rs:553,650` · `anthropic/adapter.rs:292,551` ·
  `deepseek/adapter.rs:691,808` · `mock.rs:224,267-268,346,379-380`.
- `crates/paladin-llm/src/lib.rs:98-136` — `test_capabilities_tool_calling_matches_request_surface`,
  extended by D-12.
- `src/application/services/paladin/paladin_execution_service.rs:799` (tool branch), `:1414`
  (`is_handoff_tool_call`).
- Test doubles that are the only producers of a `FunctionCall`:
  `tests/helpers/mock_llm_adapter.rs:213`, `tests/functional/paladin_tool_invocation_test.rs`,
  `tests/integration/context_injection_test.rs`, `tests/integration/arsenal_bridge_regression_test.rs`.

**Documentation surface (D-13, scoped to reachability claims only):**
- `docs/src/user-guides/tool-integration.md` · `docs/src/architecture/overview.md` ·
  `docs/src/architecture/domain-model.md` · `docs/src/contributing/contributing-providers.md`.
- `crates/doc-examples/src/sidecar.rs:25,34` and `docs/src/deployment-topologies/sidecar.md` (D-15a).

**Release surface (D-17, D-18, D-20):**
- `Cargo.toml:34` and `crates/*/Cargo.toml:3` — twelve manifests at `0.7.0`.
- `release.toml:16-21` — `shared-version = true`, tag-triggered publishing.
- `.project/current-exports.txt` and `scripts/check-api-surface.sh` — the `api-surface` guard.
- `crates/paladin-web/CHANGELOG.md` and siblings.

### Conventions

- `CLAUDE.md` and `.github/instructions/rust.instructions.md` — TDD, no `unwrap()`/`panic!` in library
  code, `cargo test` → `cargo fmt --check` → `cargo clippy -- -D warnings` before commit, rustdoc on
  every public item.
- `.github/instructions/snyk_rules.instructions.md` — Snyk scan on new/modified first-party code. **This
  phase modifies the authentication surface; the scan is not optional here.**
- `.planning/codebase/ARCHITECTURE.md` — note its §"Authentication" ("Web API: X-API-Key header or JWT
  token") and its §"Data Flow" step 2 ("Auth middleware validates X-API-Key or JWT") are both made
  false by D-01 and need updating with the rest of D-05's sweep.

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets

- **`crates/paladin-llm/src/lib.rs:98-136`** — an existing correspondence test that pins a declared
  capability to the actual request surface, with `REQUEST_SURFACE_SUPPORTS_TOOL_CALLING` as a named
  single source of truth. D-12 extends this rather than writing a new pattern.
- **`crates/paladin-web/src/openapi.rs:120-125`** — an existing drift guard comparing the generated
  spec against a committed baseline. D-03/D-18/D-20 work with it; nothing new is needed.
- **`crates/paladin-web/src/openapi.rs:103`** — `spec_paths_are_versioned_under_v1`, the existing
  precedent for asserting a route invariant against `API_V1_PREFIX`. D-15a's assertion follows its shape.
- **`crates/paladin-web/src/agent_auth.rs:190-404`** — a complete unit + router-level test module
  (constant-time key match, 401 shapes, a redaction test proving the credential never appears in the
  error body, health-exempt routing). The renames in D-04 land inside a well-tested module.
- **`crates/paladin-web/tests/auth_rbac.rs`** — the integration-level auth surface D-15b's fail-closed
  test can extend or sit beside.

### Established Patterns

- **Constant-time credential comparison** (`ct_eq`, `agent_auth.rs:82-91`) and a uniform 401 message
  that never echoes the presented credential (`:117-139`). Preserve both through the rename.
- **Capability flags describe the adapter, not the vendor** — committed rationale at
  `openai/adapter.rs:645-650`. D-12 applies the same rule to the second flag.
- **Fail-closed by default with a loud disabled state** — `AuthConfig::enabled` defaults to `true`
  (`src/config/agents.rs:115-118`), the library `AgentAuthConfig::default()` is permissive for embedded
  use, and the binary warns when auth is off (`paladin-server.rs:146-155`). D-07's warning follows the
  same voice.
- **Dated in-place amendment with original text retained** (D-00c/D-00d) — every planning and
  `.project/` correction in this phase.

### Integration Points

- `build_auth_config` (`src/bin/paladin-server.rs:145-199`) is the single seam where config becomes the
  web layer's auth state — D-02's key rename, D-07's warning and D-15b's fail-closed test all meet here.
- `decorate()` (`crates/paladin-web/src/openapi.rs:35-58`) is the single seam where the security schemes
  enter the published contract — D-03 acts here, and the baseline regeneration follows.
- `AgentApiState::with_auth` (`agent_controller.rs`) wires `AgentAuthConfig` into the router; the field
  rename in D-04 propagates through every construction site, including the tests at
  `agent_auth.rs:225-239,296-302,356-359`.
- `paladin_execution_service.rs:799` is where the tool branch reads `response.function_call` — the
  reachability statement in D-13 describes this seam and nothing changes it.

</code_context>

<specifics>
## Specific Ideas

Five findings verified against the tree on 2026-08-11, each of which changes what the phase has to do.

1. **WEB-03's substance already shipped — in Phase 2.** Commit `a2cc1c5` ("feat(02-02): flip
   supports_tool_calling honest and declare OpenAI/Anthropic ranges") set `supports_tool_calling: false`
   on all three adapters with an inline `WEB-03, D-14` rationale and added the correspondence test at
   `crates/paladin-llm/src/lib.rs:98-136`. The requirement checkbox and the ledger rows still read
   `Verified open`. **WEB-03 is a residue-and-record task, not a build task** (D-12, D-14).

2. **The remaining flag asymmetry is smaller than the defect underneath it.** OpenAI alone declares
   `supports_function_calling: true` (`:650`) while hardcoding `function_call: None` (`:553`). But
   `grep -rn "function_call: Some" --include=*.rs .` returns **only test files** — no shipped adapter,
   including `crates/paladin-llm/src/mock.rs`, ever emits a `FunctionCall`. So
   `paladin_execution_service.rs:799`'s tool branch — Arsenal invocation **and** the
   `handoff_to_specialist` path — is unreachable through any provider Paladin ships. The Arsenal
   capability is real and reachable via a consumer's own `LlmPort`; the documentation implies it is
   automatic. This is the same defect class as WEB-01, found in the LLM port instead of the auth
   surface (D-12, D-13).

3. **WEB-02's correctness edge is gated by a config flag, not by the replica count.**
   `k8s/server/deployment.yaml:14` ships `replicas: 2`, but `k8s/server/configmap.yaml` sets
   `jwt.enabled: false` and authenticates with `${PALADIN_API_KEY_CI}` / `${PALADIN_API_KEY_APP}` from a
   Secret — identical in every pod. The in-process store only becomes reachable when an operator flips
   the flag. The shipped default is safe; the footgun is the flag (D-06, D-07).

4. **WEB-02 cites the wrong manifests.** It names `k8s/deployment.yaml` and `k8s/service.yaml`; the
   Milestone 12 Epic 7 artefacts with liveness/readiness probes are `k8s/server/*`. The root
   `k8s/deployment.yaml` runs `image: paladin:test` under `sleep 3600` with its probes commented out —
   a Milestone-1-era placeholder that serves no API, at `replicas: 3` (D-08).

5. **The user's WEB-04 answer is a principle, not just a verdict.** Recorded verbatim: *"I'm really not
   sure on this one. We want to maximize the capabilities so this sounds like a future feature
   improvement. This is listed under 'deferred'. Some of these Epics in this Milestone may have been
   completed already but this is the source of some potential future version improvements not any
   current functionality. This should be recorded as such and everything should properly reflect
   current functionality. Make your decision based on this perspective."* Two instructions follow from
   it, and both are wider than Epic 27: **deferred-register content is a source of future improvements,
   not a description of current functionality** — and **every surface describing current functionality
   must actually describe it.** D-10 and D-13 implement the second one.

</specifics>

<deferred>
## Deferred Ideas

- **A shared-store `AuthPort` implementation** (SQLite via `paladin-storage`, or Redis) so a token
  issued by one instance verifies on another. Deferred with a named trigger by D-09: the first
  deployment needing more than one replica serving `AuthPort`-issued tokens. M9 §6.2 states the port was
  designed for exactly this swap, so it is an adapter, not a redesign. Recorded in ADR-0041.
- **LLM-native tool calling (Deferred-QA Epic 27)** — `tools` on `LlmRequest`, tool-call parsing,
  per-adapter sending, live-API tests. Deferred with a trigger and an owner by D-10, recorded in
  ADR-0042 and banner-linked from the source PRD by D-11. A breaking `LlmPort` change; needs its own
  phase, and its two open questions (DeepSeek support, canonical schema) answered first.
- **Letting `crates/paladin-llm/src/mock.rs` emit a `FunctionCall`** so the tool path is demonstrable
  without a custom adapter. Raised under D-12/D-13 and left to the planner's discretion; out of scope if
  it grows past a constructor option.
- **Annotating the root `k8s/deployment.yaml` as a placeholder** so a reader scanning `k8s/` does not
  deploy the sleeping pod expecting an agent API. Offered and declined under D-08 — arguably the same
  truthfulness defect class, but outside WEB-02's scope. A future infrastructure phase's call.
- **A JWT `AuthPort` implementation.** Not lost, just not chosen: D-01 ratifies opaque tokens, and
  ADR-0040 should record what reversing it would cost (a `jsonwebtoken` dependency in the audited graph,
  a signing-key management and rotation story, and the loss of immediate revocation).

</deferred>

---

*Phase: 14-api-contract-truthfulness*
*Context gathered: 2026-08-11*
