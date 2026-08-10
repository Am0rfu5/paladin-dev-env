---
phase: 13
slug: milestone-9-12-ground-truth-recorded-account
status: verified
# threats_open = count of OPEN threats at or above workflow.security_block_on severity (the blocking gate)
threats_open: 0
asvs_level: 1
created: 2026-08-10
---

# Phase 13 — Security

> Per-phase security contract: threat register, accepted risks, and audit trail.

**Phase character.** Phase 13 is a documentation ground-truth phase. It changed **zero `.rs`
files**, zero `Cargo.toml` files and zero `Cargo.lock` entries (independently re-measured by the
auditor: `git diff --name-only e12f18306ca9a80b1c3301e6afca31602e7c41ec..HEAD -- '*.rs'` → `0`).
Its STRIDE register therefore applies the six categories to **documentary integrity** rather than
to application code — the asset under protection is the corpus's own truth claims. "Spoofing" means
a published route string a reader would call; "Repudiation" means destroying provenance by deleting
a prior dated correction; "Tampering" means a stale `file:line` citation; "Elevation of privilege"
means scope creep past a plan's declared file boundary.

---

## Trust Boundaries

| Boundary | Description | Data Crossing |
|----------|-------------|---------------|
| `.project/` PRD → published mdbook page | one is provenance nobody executes against; the other is a live contract a reader calls | route strings, module paths, capability claims |
| ADR → precedence order | an ADR sits at the top of the precedence order by construction, so a wrong one instructs a code change | ratified decisions (D-14, D-15) |
| requirement text → downstream quotation | a document that misstates itself is quoted verbatim by a later requirement | counts, version figures, job lists |
| markdown source → rendered mdbook output | `{{#include}}` pulls external file content into a published page; the source file does not contain the included text | code examples, route literals |

The fourth boundary was **added by this audit** — it is the boundary T-13-20 crossed undetected
(see the Accepted Risks Log and the method note below).

---

## Threat Register

Register authored at plan time (`register_authored_at_plan_time: true`) across all 13 PLAN.md
`<threat_model>` blocks. 41 unique numeric ids plus a recurring `T-13-SC` supply-chain acceptance
row; `T-13-11` names two distinct threats in plans 13-04 and 13-06 (id collision, recorded below),
so 43 register entries in total.

**42 of 43 closed.** Every `mitigate` threat was independently re-verified against the live tree by
the auditor — cited evidence was re-run or re-derived rather than taken from SUMMARY self-reports.

| Threat ID | Category | Component | Severity | Disposition | Mitigation | Status |
|-----------|----------|-----------|----------|-------------|------------|--------|
| T-13-01 | Tampering | ledger row counts / test figures | high | mitigate | `auth_rbac` 5/5, `event_trigger_pipeline` 5/5, `orchestrator_bridge::` 10/10 reproduced; ledger `grep -c '^| REQ-'` → 120/120 | closed |
| T-13-02 | Tampering | stale `file:line` in ADR `## Code Locations` | high | mitigate | `agent_registry.rs:55-79`, `k8s/server/deployment.yaml:8-85`, `agent_controller.rs:723`, `openapi.rs:103`, `listener.rs:141`, `llm_port.rs` all read directly and matched | closed |
| T-13-03 | Repudiation | ADR-0029 placeholder row | high | mitigate | superseded in place, original text retained (D-00d); `.project/` zero-deletion diff | closed |
| T-13-04 | Elevation of privilege | ADR numbering index | medium | mitigate | `.planning/decisions/` = exactly 40 files; only 0037/0038/0039 new; PROMOTION.md advanced once | closed |
| T-13-05 | Denial of service | ledger integrity under 13 parallel writers | medium | mitigate | 120/120 rows intact, zero contention overlaps across the phase's commits | closed |
| T-13-06 | Repudiation | auth requirement verdicts | medium | mitigate | `REQ-jwt-bearer-auth-v2` / `REQ-opaque-bearer-token-adapter-v1` rows verified against live `agent_auth.rs` and manifests | closed |
| T-13-07 | Spoofing | cited CI run | high | mitigate | `gh run list` reproduced run `31320378772` (2026-08-09, success), newer than `30861568499` (2026-08-03, failure) | closed |
| T-13-08 | Tampering | inherited exclusion inventory | medium | mitigate | 13-03/13-11 cite Phase 12's inventory; not re-run per D-07 | closed |
| T-13-09 | Elevation of privilege | GitHub API access | high | mitigate | no `-X POST/PUT/DELETE` call anywhere in phase transcripts; `gh api` reads only | closed |
| T-13-10 | Repudiation | `mdbook build` result after edits | high | mitigate | reproduced identical exit `101` with the same two pre-existing errors; no new error naming an edited page | closed |
| T-13-11 (13-04) | Information disclosure | M11 26-open count | medium | mitigate | recorded "Verified open", corroborated in ledger + DOCS-01 hand-off | closed |
| T-13-11 (13-06) | Repudiation | vacuous-checkbox verdict | medium | mitigate | Task 0.0 text quoted verbatim in ledger `:523` | closed |
| T-13-12 | Elevation of privilege | premature D-14/D-15 statement | high | mitigate | 13-04/05/06 summaries carry no premature statement; 13-09's checkpoint was genuinely blocking | closed |
| T-13-13 | Tampering | 13-05 scope | medium | mitigate | `git show --stat`: zero `.project/` files touched | closed |
| T-13-14 | Repudiation | M12 Epic docs outside 13-11 scope | medium | mitigate | outside `files_modified`; references stay ledger rows | closed |
| T-13-15 | Spoofing | JWT capability claim | high | mitigate | `jsonwebtoken` absent from all manifests; `InMemoryTokenAuthAdapter` is what `paladin-server` wires; ledger records "Contract diverges" honestly | closed |
| T-13-16 | Information disclosure | multi-replica token store | high | mitigate | `k8s/server/deployment.yaml` `replicas: 2` with live probes; ledger discloses the shared-store gap, owner WEB-02 | closed |
| T-13-17 | Repudiation | coverage tooling claim | high | mitigate | no `coverage:` job in `ci.yml`, no `.codecov.yml` at root | closed |
| T-13-18 | Tampering | relocation vs content gap | medium | mitigate | ledger `:589` keeps them as two separately labelled facts (D-00f) | closed |
| T-13-19 | Elevation of privilege | coverage-threshold variant | medium | mitigate | ledger `:578` records both sides; OQ-3 still Open, resolved by neither | closed |
| **T-13-20** | **Spoofing** | **the published route in `sidecar.md`** | **high** | **accept** (was `mitigate`) | **see Accepted Risks Log — residue survives in included content** | **accepted 2026-08-10** |
| T-13-21 | Repudiation | `.project/` annotation destroying provenance | high | mitigate | `git diff -- '.project/' \| grep -c '^-[^-]'` → `0` across the whole phase | closed |
| T-13-22 | Elevation of privilege | scope creep past D-11's four Epics | medium | mitigate | 13-08 recorded extra route sites under "Route Sites Found Outside D-11's Four Named Epics" rather than editing them | closed |
| T-13-23 | Tampering | duplicate ADR number | medium | mitigate | PROMOTION.md "Next free ADR number: 0040" advanced exactly once | closed |
| T-13-24 | Elevation of privilege | auto-approving the D-14/D-15 checkpoint | high | mitigate | 13-09 Checkpoint Status shows genuine `AskUserQuestion` provenance, auto-mode `false`; confirmed by 13-VERIFICATION.md #7 | closed |
| T-13-25 | Information disclosure | undisclosed topology limitation | high | mitigate | `http-service-host.md` limitation prose confirmed live | closed |
| T-13-26 | Spoofing | "tools + memory" capability claim | high | mitigate | claim replaced; `AgentSpec` has no Garrison/Arsenal field | closed |
| T-13-27 | Repudiation | silent candidate-9 disposition | high | mitigate | PROMOTION.md candidate-9 entry carries an explicit disposition | closed |
| T-13-28 | Tampering | `.rs` edit under a docs phase | high | mitigate | `git diff --name-only -- '*.rs'` → `0` | closed |
| T-13-29 | Denial of service | ledger destroyed by REQUIREMENTS reduction | high | mitigate | reduction to a pointer occurred only after asserted preconditions; ledger holds 120/120 | closed |
| T-13-30 | Tampering | orphaned `REQ-` rows | high | mitigate | `0` `REQ-` rows remain in REQUIREMENTS.md; 120 live only in the ledger | closed |
| T-13-31 | Elevation of privilege | rewriting historical lockstep gates | medium | mitigate | "historical lockstep gates (v0.3.0..v0.6.0) left untouched" | closed |
| T-13-32 | Spoofing | four relocated module/doc paths | high | mitigate | all four confirmed present and correct in the tree | closed |
| T-13-33 | Tampering | double-correcting an existing correction | medium | mitigate | `REQ-llm-tool-calling-port` left unchanged where a correction already existed | closed |
| T-13-34 | Tampering | ADR-0029 trajectory table | high | mitigate | placeholder superseded in place, four new rows ascending, endpoints intact; tags live-verified | closed |
| T-13-35 | Repudiation | rival numbering ADR | high | mitigate | only ADR-0030 exists, untouched | closed |
| T-13-36 | Elevation of privilege | extra version ADR | high | mitigate | no additional version ADR file (40 total, accounted for) | closed |
| T-13-37 | Repudiation | uncited requirement closure | high | mitigate | all five ORCH-01…05 `[x]` entries carry cited evidence blocks | closed |
| T-13-38 | Tampering | `.rs`/manifest edit | high | mitigate | `git diff --name-only -- '*.rs'` → `0`, `-- '*Cargo.toml'` → `0` | closed |
| T-13-39 | Spoofing | dangling `ADR-NNNN` citations | medium | mitigate | all 9 ledger ADR citations resolve to existing files | closed |
| T-13-40 | Tampering | unreproducible CI-only claims | high | mitigate | ledger's "Claims that are CI-only" section gives exact commands and reasoning (curl 403, no llvm-cov binary, no docker) | closed |
| T-13-41 | Elevation of privilege | absorbing deferred scope | medium | mitigate | three named hand-off blocks (Phases 14/15/16) hold deferred items; nothing absorbed | closed |
| T-13-SC | Tampering | supply chain | low | accept | zero `.rs` / `Cargo.toml` / `Cargo.lock` changes across the phase — rationale independently confirmed true | accepted 2026-08-10 |

*Status: open · closed · accepted*
*Severity: critical > high > medium > low — only open threats at or above `workflow.security_block_on` (`high`) count toward `threats_open`*
*Disposition: mitigate (implementation required) · accept (documented risk) · transfer (third-party)*

---

## Accepted Risks Log

| Risk ID | Threat Ref | Rationale | Accepted By | Date |
|---------|------------|-----------|-------------|------|
| AR-13-01 | T-13-20 | See full rationale below | human operator (via `/gsd-verify-work 13`, AskUserQuestion) | 2026-08-10 |
| AR-13-02 | T-13-SC | Phase changed zero `.rs`, `Cargo.toml` and `Cargo.lock` files; no dependency entered the tree, so the supply-chain surface is unchanged by this phase. Independently re-measured by the auditor against base `e12f18306ca9a80b1c3301e6afca31602e7c41ec`. | human operator (via `/gsd-verify-work 13`) | 2026-08-10 |

### AR-13-01 — T-13-20, unprefixed agent route in `sidecar.md`'s included example

**The residue.** `crates/doc-examples/src/sidecar.rs:34` builds `{base_url}/agents/{agent}/execute`
— the unprefixed route — and its `:25` doc comment names the same path.
`docs/src/deployment-topologies/sidecar.md:34` embeds that file verbatim via
`{{#include ../../../crates/doc-examples/src/sidecar.rs:sidecar_client}}`, and the page claims the
example "matches the current API". The page's own prose at lines 29-30 correctly states
`POST /v1/agents/{id}/execute`. The live server mounts the agent API only under
`API_V1_PREFIX = "/v1"` (`crates/paladin-web/src/agent_controller.rs:723`, asserted by
`spec_paths_are_versioned_under_v1` at `crates/paladin-web/src/openapi.rs:103`), so a reader who
copies the rendered example writes a client that receives `404 Not Found`.

**Why the original mitigation did not catch it.** The acceptance criterion that closed this row was
`grep -c 'POST /agents/{id}/execute' sidecar.md` → `0`, run against the **raw markdown source**.
That source contains only the `{{#include}}` directive, never the literal route string, so the grep
was *structurally incapable* of detecting the residue however often it ran. The mitigation's stated
sweep (`docs/src/`, `examples/`, `README.md`) is literally satisfied — `crates/doc-examples/src/`
lies outside all three — but the threat is named for what `sidecar.md` **publishes**, and the
include makes the stale route part of that published output. The check was performed in the wrong
layer.

**Why it is accepted rather than mitigated.** The fix requires editing a `.rs` file. Phase 13's D-19
boundary admits no `.rs` change, and the phase's close-out assertion — an independently re-measured
zero-`.rs` diff against base `e12f18306ca9a80b1c3301e6afca31602e7c41ec` — depends on that holding.
Breaching the boundary to fix one string would invalidate an assertion the whole phase was built to
support. The residue is disclosed, not hidden: exact `file:line`, exact fix, and named owner are
recorded in `13-REVIEW.md` CR-01 and in `.planning/REQUIREMENTS.md`'s Phase-14 hand-off item 6.

**Acceptance provenance.** Obtained interactively from the human operator during the
`/gsd-verify-work 13` session on 2026-08-10 via the runtime's AskUserQuestion prompt, after the
auditor returned `## OPEN_THREATS` with this threat blocking. The operator was shown the auditor's
full reasoning — including the wrong-layer verification finding — alongside two alternatives ("fix
the `.rs` now, close it properly" and "leave blocked until Phase 14") and selected "accept as
documented residual risk". A relayed human decision, not an agent inference; auto-mode was off.

**Residual risk owner and closure condition.** Phase 14. Change
`crates/doc-examples/src/sidecar.rs:34` and the `:25` doc comment to
`{base_url}/v1/agents/{agent}/execute`, ideally paired with an assertion tying the literal back to
`paladin_web::agent_controller::API_V1_PREFIX` — `cargo check` cannot catch this, since the URL is
an opaque string literal. Re-run `/gsd-secure-phase 13` once Phase 14 lands the fix to move
T-13-20 from `accept` to `closed`.

---

## Method Note — verification-layer defect (carried to Phase 14)

**Class of defect:** an acceptance criterion that greps a markdown *source* file cannot see content
mdBook pulls in via `{{#include}}`. Any assertion of the form "zero occurrences of X remain in
`page.md`" is unsound for a page that includes external files, because the string it is hunting
never appears in the file it greps.

**Rule for future route/content assertions:** sweep the included files as well as the including
page — resolve `{{#include}}` targets and grep those too, or assert against the rendered
`docs/book/` output rather than `docs/src/`. Scoping a sweep to `docs/src/` silently exempts every
`{{#include}}` source, and `crates/doc-examples/src/` is exactly such a source.

This note is recorded here and in `.planning/REQUIREMENTS.md`'s Phase-14 hand-off item 6 so the
phase that owns the fix does not redraw the same boundary.

---

## Security Audit Trail

| Audit Date | Threats Total | Closed | Open | Run By |
|------------|---------------|--------|------|--------|
| 2026-08-10 | 43 | 42 | 1 → 0 (T-13-20 accepted) | gsd-security-auditor (ASVS L1; L2/L3 depth applied to all high-severity items and to T-13-20) |

---

## Informational — not counted toward `threats_open`

- **`REQ-fail-closed-auth-posture` — unregistered surface.** Surfaced during plan 13-13's close-out
  integrity re-check: the fail-closed auth code path exists and matches its requirement's shape, but
  no test drives the `Err` branch or observes a real refusal, so D-03's evidence bar refuses to mark
  it `Shipped`. Not a Phase 13 mitigation gap (this phase wrote no auth code), but genuine
  security-relevant surface with no `T-13-NN` mapping. Carried to Phase 14 as hand-off item 5.
- **`T-13-11` id collision.** The same numeric id names two unrelated threats across plans 13-04
  (Information disclosure / M11 26-open count) and 13-06 (Repudiation / vacuous-checkbox verdict).
  A register-authoring defect, not a security gap; both instances verified closed independently.
- **No `## Threat Flags` sections.** None of the 13 SUMMARY.md files carries the `## Threat Flags`
  section the audit pipeline reads to pick up threats discovered during execution (confirmed by
  grep across all 13). This is why `REQ-fail-closed-auth-posture` surfaced only via 13-13's
  close-out rather than through the standard mechanism.

---

## Sign-Off

- [x] All threats have a disposition (mitigate / accept / transfer)
- [x] Accepted risks documented in Accepted Risks Log
- [x] `threats_open: 0` confirmed
- [x] `status: verified` set in frontmatter

**Approval:** verified 2026-08-10
