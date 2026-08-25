# Phase 14: API Contract Truthfulness - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.
> Decisions are captured in CONTEXT.md — this log preserves the alternatives considered.

**Date:** 2026-08-11
**Phase:** 14-api-contract-truthfulness
**Areas discussed:** Token mechanism (WEB-01), Multi-replica store (WEB-02), Tool calling (WEB-03/WEB-04), Phase boundary & blast radius

---

## Area selection

| Option | Description | Selected |
|--------|-------------|----------|
| Token mechanism (WEB-01) | Opaque vs JWT, and how far the rename travels | ✓ |
| Multi-replica store (WEB-02) | Pin replicas, build a shared store, or gate the unsafe combination | ✓ |
| Tool calling (WEB-03/04) | Build or withdraw Epic 27; scope of the capability-flag fix | ✓ |
| Phase boundary & blast radius | Inherited hand-off items, ADR allocation, baselines, release bookkeeping | ✓ |

**User's choice:** all four (multiSelect).

---

## Token mechanism (WEB-01)

### Q1 — Which mechanism is the answer?

| Option | Description | Selected |
|--------|-------------|----------|
| Keep opaque, fix the words (Recommended) | Ratify M9 §6.1's opaque hashed-token store; correct the M12 vocabulary. No `jsonwebtoken` dependency exists; JWT/OIDC was an explicit M9 non-goal; dissolves M12 OQ-4 | ✓ |
| Add a real JWT AuthPort | Implement a `jsonwebtoken`-backed AuthPort so code matches the M12 docs; needs a signing-key story, gives up immediate revocation | |
| Support both, configured | Opaque by default, JWT as an alternative AuthPort; two auth paths, and "one recorded answer" becomes two | |

**User's choice:** Keep opaque, fix the words.
**Notes:** → D-01.

### Q2 — How is the config-key break handled?

| Option | Description | Selected |
|--------|-------------|----------|
| Clean rename, no alias (Recommended) | Rename the key and `JwtAuthConfig`, record the break; follows M8's `use_cases` → `services` clean-break precedent | ✓ |
| Rename with a serde alias | `#[serde(alias = "jwt")]` keeps v0.6.0 configs loading; leaves the untrue word in the accepted surface | |
| Leave the key, fix prose only | Zero operator impact; leaves WEB-01 half-closed by its own criterion | |

**User's choice:** Clean rename, no alias.
**Notes:** → D-02.

### Q3 — How far does the OpenAPI correction go?

| Option | Description | Selected |
|--------|-------------|----------|
| Rename scheme + drop bearerFormat (Recommended) | New scheme id, `bearerFormat: "JWT"` removed, handler annotations follow, committed `openapi.json` regenerated in the same commit | ✓ |
| Drop bearerFormat, keep the id | Smaller diff; the scheme id still says jwt | |
| Document it as opaque explicitly | Rename + drop + a scheme description stating the mechanism; largest diff | |

**User's choice:** Rename scheme + drop bearerFormat.
**Notes:** → D-03, D-18, D-20.

### Q4 — How is the public Rust surface handled?

| Option | Description | Selected |
|--------|-------------|----------|
| Rename the field, record the break (Recommended) | Rename `AgentAuthConfig.jwt`, fix `Principal`/module docs, CHANGELOG entry, regenerate `.project/current-exports.txt` | ✓ |
| Rename + `#[deprecated]` shim | Downstream keeps compiling with a warning; DEBT-02 (zero `#[deprecated]` in the tree) is Phase 8's scope, not this one | |
| Docs only, keep the field name | No API break; a Rust consumer still reads the field as JWT | |

**User's choice:** Rename the field, record the break.
**Notes:** → D-04, D-20.

**Continue?** Next area.

---

## Multi-replica store (WEB-02)

### Q1 — Which of WEB-02's two exits?

| Option | Description | Selected |
|--------|-------------|----------|
| Neither as stated — gate the unsafe combo (Recommended) | The shipped ConfigMap sets `jwt.enabled: false` and uses Secret-sourced API keys, so `replicas: 2` is correct; scope the shared-store requirement to the AuthPort path and make the unsafe combination loud | ✓ |
| Pin replicas: 1 and document | WEB-02's literal cheap exit; caps the shipped topology to guard a disabled code path | |
| Build the shared store | Persistent-store AuthPort + cross-instance test; feature work, not a truthfulness fix | |

**User's choice:** Neither as stated — gate the unsafe combo.
**Notes:** Deviates from the requirement's own "done when", so ADR-0041 must carry the reasoning. → D-06.

### Q2 — How is "loud" delivered?

| Option | Description | Selected |
|--------|-------------|----------|
| Warn on startup + docs + manifest comment (Recommended) | Unconditional WARN when the in-process store is wired (a pod cannot read its own replica count), plus ConfigMap comment, `k8s/README.md` and the deployment-topology pages | ✓ |
| Refuse to start without acknowledgement | New opt-in flag; impossible to miss, breaks existing configs that had the store on | |
| Documentation and manifests only | Meets ROADMAP criterion 2 exactly; no runtime feedback | |

**User's choice:** Warn on startup + docs + manifest comment.
**Notes:** → D-07.

### Q3 — WEB-02 cites the wrong manifests. What now?

| Option | Description | Selected |
|--------|-------------|----------|
| Correct the citation, leave the placeholder (Recommended) | Dated banner naming `k8s/server/*`, original text retained; the root manifest sleeps and exposes no API | ✓ |
| Correct it and annotate the placeholder too | Same, plus a pointer comment in `k8s/deployment.yaml` | |
| Act on the right files, say nothing | Smallest footprint; stale citation survives | |

**User's choice:** Correct the citation, leave the placeholder.
**Notes:** → D-08; the placeholder annotation moved to Deferred Ideas.

### Q4 — What stops the shared store being lost?

| Option | Description | Selected |
|--------|-------------|----------|
| Deferred with a named trigger (Recommended) | Reintroduction condition recorded, citing M9 §6.2; follows the ADR-0035 precedent | ✓ |
| Permanent property of the topology | The ADR-0039 treatment; strongest closure, needs superseding if ever wanted | |
| Backlog item for a future milestone | Competes at the next milestone boundary; the mechanism that produced three prior deferrals | |

**User's choice:** Deferred with a named trigger.
**Notes:** → D-09.

**Continue?** Next area.

---

## Tool calling (WEB-03 / WEB-04)

### Q1 — Does Paladin support LLM tool calling?

| Option | Description | Selected |
|--------|-------------|----------|
| Withdraw Epic 27, record the reason (Recommended) | Arsenal/MCP already provides tool execution; breaking `LlmPort` change; both open questions unanswered | |
| Build it, phased | The PRD's own four-stage plan; the most expensive item in the register | |
| Withdraw with a reintroduction condition | Same withdrawal with a named trigger recorded | |

**User's choice:** free text — *"I'm really not sure on this one. We want to maximize the capabilities so this sounds like a future feature improvement. This is listed under 'deferred'. Some of these Epics in this Milestone may have been completed already but this is the source of some potential future version improvements not any current functionality. This should be recorded as such and everything should properly reflect current functionality. Make your decision based on this perspective."*
**Notes:** Resolved to the third option's shape — out of current scope, recorded as a future capability improvement with a named trigger and an owner. The answer was read as a general principle for the phase, not only a verdict on Epic 27, and is quoted in CONTEXT.md `<specifics>` finding 5. → D-10, D-11, D-13.

### Q2 — How far does WEB-03's honesty fix reach?

| Option | Description | Selected |
|--------|-------------|----------|
| Flag + record the reachability finding (Recommended) | Flip OpenAI's `supports_function_calling`, extend the correspondence test to both flags, and record that no shipped adapter emits a `FunctionCall` | ✓ |
| Just the flag and the test | Closes WEB-03 by its own wording; leaves the reachability finding undocumented | |
| Leave `supports_function_calling` as-is | Treat the flag as vendor capability; contradicts the rationale already committed in the adapters | |

**User's choice:** Flag + record the reachability finding.
**Notes:** → D-12, D-13.

### Q3 — Where does the reachability limitation get stated?

| Option | Description | Selected |
|--------|-------------|----------|
| Code docs + the user-facing tool guide (Recommended) | Rustdoc plus `docs/src/user-guides/tool-integration.md` | |
| Everywhere it is implied | The above plus `architecture/overview.md`, `architecture/domain-model.md`, `contributing-providers.md` | ✓ |
| Planning record only | ADR and ledger; leaves the shipped docs making the claim | |

**User's choice:** Everywhere it is implied.
**Notes:** Scope guard added — the sweep covers tool-call reachability claims only; DOCS-01 (Phase 16) still owns general content currency. → D-13.

### Q4 — What makes this recording different from three prior deferrals?

| Option | Description | Selected |
|--------|-------------|----------|
| ADR + dated banner at the Epic 27 source (Recommended) | ADR 0040-series with reason, trigger and owner, plus a D-00c banner on the `.project/` Epic 27 documents | ✓ |
| ADR only | Lighter; the next reader of Epic 27 finds no pointer | |
| Requirement amendment only | No ADR; but build-vs-defer with a breaking port change is contested (D-00g) | |

**User's choice:** ADR + dated banner at the Epic 27 source.
**Notes:** → D-11, D-16.

**Continue?** Next area.

---

## Phase boundary & blast radius

### Q1 — Phase 13's two hand-off items: in or out?

| Option | Description | Selected |
|--------|-------------|----------|
| Both in (Recommended) | The `sidecar.rs:25,34` route fix with an `API_V1_PREFIX` assertion, and `REQ-fail-closed-auth-posture`'s missing Err-branch test | ✓ |
| Sidecar fix only | It carries a live security disposition; leaves the fail-closed test unowned | |
| Neither — keep to the four requirements | Cleanest boundary; nothing downstream picks them up | |

**User's choice:** Both in.
**Notes:** → D-15. Landing the sidecar fix moves T-13-20 from AR-13-01 to closed; re-run `/gsd-secure-phase 13` after.

### Q2 — How many ADRs from 0040?

| Option | Description | Selected |
|--------|-------------|----------|
| Three: 0040/0041/0042 (Recommended) | One each for WEB-01, WEB-02, WEB-04; finest supersession unit, per Phase 11's recorded ADR-0034 lesson | ✓ |
| Two: auth + tool calling | Bundle WEB-01 + WEB-02; fewer records, coarser supersession unit | |
| One phase ADR | Smallest record set; three future changes would all supersede one document | |

**User's choice:** Three: 0040/0041/0042.
**Notes:** WEB-03 gets none — code-settled defect under D-00g. → D-16.

### Q3 — How much release bookkeeping belongs here?

| Option | Description | Selected |
|--------|-------------|----------|
| CHANGELOG under Unreleased, no bump (Recommended) | BREAKING entries + baseline regeneration; leave versioning to the release process | |
| Bump the lockstep version too | Version the breaking change the moment it lands; touches all twelve manifests | ✓ |
| Baselines only | CI stays green, no consumer-visible record | |

**User's choice:** Bump the lockstep version too.
**Notes:** The cost flagged when the option was offered — `paladin-herald` having no CHANGELOG (SEC-04) — was then checked and no longer applies: `crates/paladin-herald/CHANGELOG.md` exists in the tree. → D-17.

### Q4 — Bump to what?

| Option | Description | Selected |
|--------|-------------|----------|
| 0.8.0 (Recommended) | SemVer for 0.x: a breaking change bumps the minor; two consumer-visible renames land here | ✓ |
| 0.7.2 | Matches the milestone's close-out numbering; a patch version carrying two breaking renames | |
| 0.7.1 → next patch, decided at close-out | Defers the number to the milestone; but the bump was asked for in this phase | |

**User's choice:** 0.8.0.
**Notes:** `openapi.json`'s `info.version` comes from `CARGO_PKG_VERSION`, so the bump moves the drift baseline a second time — bump and regeneration must land together or be sequenced. → D-17, D-18.

**Continue?** Ready for context.

---

## Claude's Discretion

- Exact replacement identifiers for `SEC_JWT`, `AgentAuthConfig.jwt`, `JwtAuthConfig` and the `http.auth.jwt.*` key.
- Whether the shipped `crates/paladin-llm/src/mock.rs` should be able to emit a `FunctionCall`.
- Plan decomposition and wave structure.
- Ledger and requirement amendment mechanics (which rows carry closure versus pointer).
- Whether ADR-0040 also records M12 Epic 5's OQ-4 as dissolved in the OQ table's own location.

## Deferred Ideas

- A shared-store `AuthPort` implementation — deferred with a named trigger (D-09, ADR-0041).
- LLM-native tool calling / Deferred-QA Epic 27 — deferred with a trigger and an owner (D-10, ADR-0042).
- Letting the shipped mock adapter emit a `FunctionCall` so the tool path is demonstrable.
- Annotating the root `k8s/deployment.yaml` as a placeholder — offered and declined under D-08.
- A JWT `AuthPort` implementation — not chosen; ADR-0040 records what reversing D-01 would cost.
