# Phase 13: Milestone 9-12 Ground Truth & Recorded Account - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.
> Decisions are captured in CONTEXT.md — this log preserves the alternatives considered.

**Date:** 2026-08-10
**Phase:** 13-milestone-9-12-ground-truth-recorded-account
**Mode:** `--auto` — all ten gray areas auto-selected; every question resolved to its recommended
option **without human confirmation**. Two decisions carry `⚠ HUMAN REVIEW` in CONTEXT.md (D-14,
D-15) and plan ⑧ is gated on a blocking `checkpoint:decision` because of them.
**Areas discussed:** Ledger home & shape (ORCH-01), The new verdict class (ORCH-01), Ledger
arithmetic (ORCH-01), Checkbox verdicts & the corpus pattern (ORCH-02), Agent route surface
(ORCH-03a), Stale paths (ORCH-03 b-e), `AgentProvisioner` placement (ORCH-04a), Garrison & Arsenal on
HTTP-served agents (ORCH-04b), Version trajectory & numbering prediction (ORCH-05), Code-change
boundary and ADR allocation (cross-cutting).

---

## Ledger home & shape (ORCH-01)

| Option | Description | Selected |
|--------|-------------|----------|
| New `.planning/ledgers/milestone-09-12.md`; REQUIREMENTS.md §3607-3931 → pointer | Fifth sibling in the series; all four existing ledgers already name this filename | ✓ |
| Grow the REQUIREMENTS.md section in place | Keeps everything in one file; contradicts four recorded head notes | |
| Ledger plus a REQUIREMENTS.md copy | Two diverging records of the same 120 rows | |

**Choice:** New sibling file, REQUIREMENTS.md section reduced to a pointer (D-01).
**Notes:** Not really a judgement call — `milestone-01.md:5`, `milestone-02-03.md:5`,
`milestone-04-06.md:5` and `milestone-07-08.md:5` each name `milestone-09-12.md`, and
`REQUIREMENTS.md:3552/:3565/:3581` repeat it. Phase 12 deliberately left the file uncreated (its
D-09) so this phase's scope would not be silently constrained. Vocabulary: keep the eleven-class
run-5 status key already written at `REQUIREMENTS.md:3634-3637` and map it onto the series' seven
classes in the head note, rather than re-keying 120 rows (D-02). `Verify` is retired — it is a
to-do marker, not a verdict.

---

## The new verdict class (ORCH-01)

| Option | Description | Selected |
|--------|-------------|----------|
| Carry both halves — the failure and the 2026-08-08 fix, each dated | Phase 12's hand-off names this as ORCH-01's own deliverable | ✓ |
| Record the failure only | Accurate for Milestone 10's own record, false about the tree today | |
| Record the fix only | Erases the corpus's only instance of the verdict class | |

**Choice:** Both halves (D-05).
**Notes:** Milestone 10 is 100% complete, ships everything, and failed M10 Epic 2 §8 — and no longer
does, since Phase 9 plan 09-06 / commit `cb75b2b`, promoted to ADR-0036 by Phase 12 with
`scripts/check-workflow-suppressions.sh` behind it. Verified independently this session: `grep -n
"cargo audit --ignore" .github/workflows/ci.yml` returns nothing. Provenance travels with the
closure (D-06, D-00i): Phase 9's D-07 re-scope was an unratified `--auto` decision, and Phase 12's
D-01/D-08 were ratified only at plan 12-01's blocking checkpoint.

---

## Ledger arithmetic (ORCH-01)

| Option | Description | Selected |
|--------|-------------|----------|
| Correct the 16/104 figure at source and record the measured 35/53/32 split | The sixteen `settled-by` entries are variant-register entries, not ledger rows | ✓ |
| Use ORCH-01's figure as written | Budgets 104 rows against a population of 120 | |
| Note the discrepancy in the ledger only | Leaves the defective figure live in the requirement | |

**Choice:** Correct at source (D-04).
**Notes:** Fresh finding. `intel/SYNTHESIS.md:546` puts the sixteen under the variants section and
`:335` defines the mechanism as applying to variants; `grep "settled-by"` over
`REQUIREMENTS.md:3607-3931` returns **0**. Measured split: 35 bare `Verify`, 53 bare `Shipped`, 32
already richer. Same error class as Phase 10's D-05 (a "14-row table" holding 13), and again sitting
inside the requirement that exists to retire it.

---

## Checkbox verdicts & the corpus pattern (ORCH-02)

| Option | Description | Selected |
|--------|-------------|----------|
| Five verdicts + the five-run pattern in the ledger head note, no ADR | The ledger is what a planner opens; one place stops a sixth rediscovery | ✓ |
| Write the pattern into a new dedicated document | A sixth place for a fact that keeps being rediscovered | |
| ADR for the corpus position | It describes the corpus; it is not a contested position (D-00g) | |

**Choice:** Ledger head note, no ADR (D-10).
**Notes:** All five corroborated this session against `intel/task-completion-state.md` and
`code-verification.md:622-659`. M11's 26 open is the only genuine count in run 5 and the only one of
542 that survives verification — carried to DOCS-01 (Phase 16), not settled here. None of the five
becomes a task.

---

## Agent route surface (ORCH-03a)

| Option | Description | Selected |
|--------|-------------|----------|
| `/v1` confirmed against `openapi.json`; Epics 1/3/4/5 → superseded provenance; ADR-0037 | The committed drift-guard baseline locks in what shipped | ✓ |
| Treat the disagreement as still unsettled | Contradicted by the committed baseline | |
| Rewrite the four Epics' route text | Violates D-00c — annotate, never rewrite | |

**Choice:** `/v1`, with ADR-0037 (D-11) — plus one live documentation fix (D-12).
**Notes:** All six agent paths in `crates/paladin-web/openapi.json` are `/v1`-prefixed. Fresh
finding: the only *published* unprefixed reference anywhere in `docs/src/`, `examples/` or
`README.md` is `docs/src/deployment-topologies/sidecar.md:29`. That is a live contract, not
provenance, and it lands inside ORCH-03's own done-when. One-line correction.

---

## Stale paths (ORCH-03 b-e)

| Option | Description | Selected |
|--------|-------------|----------|
| Ledger rows + D-00c source annotations, no ADR | Code-settled relocations, not contested positions | ✓ |
| An ADR per relocation | Four ADRs for four facts nobody disputes | |
| Ledger rows only, no source annotation | Leaves the non-existent paths live in requirement text | |

**Choice:** Rows plus annotations (D-13).
**Notes:** All four verified — each old path absent, each new path present. (d) splits into two
facts: `Shipped (relocated)` for the move and `Verified open` for the 311-line rewrite that the M11
Epic 3 appendix exemption froze — the rewrite is DOCS-02's, Phase 16's. A related fresh finding
(D-09) was deliberately **not** made a sixth ORCH-03 item: `scripts/check-api-surface.sh:6` now
defaults to `.project/current-exports.txt`, which exists, so run-5 finding 8's "the api-surface job
fails on every run" is closed — but four M12 requirements still name the undotted path. Recorded as
ledger rows and handed to Phase 15 rather than growing ORCH-03.

---

## `AgentProvisioner` placement (ORCH-04a) — ⚠ HUMAN REVIEW

| Option | Description | Selected |
|--------|-------------|----------|
| Keep in `paladin-web`, because `AgentSpec` is an OpenAPI-annotated HTTP DTO; ADR-0038 | Retires the default by denying its premise; the reason survives a second consumer | ✓ |
| Promote to `paladin-ports` now for reuse by future topologies | Would put `utoipa` in the core-tier ports crate, against ADR-0015(i) | |
| Keep the default as written ("promote if a second consumer appears") | Leaves a load-bearing seam as a default, which ORCH-04 exists to end | |

**Choice:** Keep in `paladin-web`, with the reason restated (D-14). **Reversibility: costly.**
**Notes:** Fresh finding — Epic 1 §7's claim that "either placement is clean since it references
`Paladin` + `PaladinExecutorPort`" omits `AgentSpec`, which derives `utoipa::ToSchema`
(`agent_registry.rs:55`) and is doc-commented "Sent in the body of `POST /agents`".
`paladin-ports` carries no `utoipa`; ADR-0015(i) bars web-framework dependencies from it. The
default's escape clause would also have fired on a false signal: a second impl already exists
(`src/infrastructure/web/facade_provisioner.rs:70`) but it is `#[cfg(feature = "web-server")]`-gated,
i.e. the HTTP composition root, not a new topology. `queue-worker.md:55` says each worker "is itself
an embedded agent host", and neither `queue-worker.md` nor `sidecar.md` mentions provisioning from a
spec at all. **Fallback if overturned:** split `AgentSpec` into a domain spec and an HTTP DTO with a
`From` conversion, move `ProvisionError`/`ProvisionedAgent`, deprecate the `paladin-web` re-export —
architecture work across two published crates, Phase 14, not here.

---

## Garrison & Arsenal on HTTP-served agents (ORCH-04b) — ⚠ HUMAN REVIEW

| Option | Description | Selected |
|--------|-------------|----------|
| A property of the shipped topology, stated in the decision matrix; ADR-0039 | Honest, and the routing story already supports it | ✓ |
| Planned scope with a target | Would commit an unplanned milestone; no schedule exists | |
| Leave it in the Epic 2 non-goal | The exact non-answer ORCH-04 rejects | |

**Choice:** Property of the topology, stated in `overview.md`, with the `http-service-host.md:54`
correction (D-15). **Reversibility: costly.**
**Notes:** Fresh finding — ORCH-04(b) frames this as under-surfaced, but it is a contradiction:
`http-service-host.md:54`'s sequence diagram reads `Service->>Agent: run (LLM + tools + memory)`,
using the same words `embedded-library.md:31-32` uses for the topology that actually has them, while
`overview.md` — M11 Epic 6 FR-8's "single source of routing" — says nothing. `grep -in
"garrison\|arsenal"` over the whole directory hits only `embedded-library.md`. Chose "property"
because `AgentSpec` has no memory/tool fields and adding them is genuine API design (MCP identity,
credentials, lifetimes) that no milestone scheduled; and because the routing answer already works —
a consumer needing Garrison or Arsenal uses the embedded topology, which `queue-worker.md:55` already
says a worker is. ADR-0039 names WEB-04 (Phase 14) downstream: WEB-04's own text asks for a "stated
relationship" between Arsenal/MCP and LLM tool calling, and this is half of it.

---

## Version trajectory & numbering prediction (ORCH-05)

| Option | Description | Selected |
|--------|-------------|----------|
| Append four rows to ADR-0029; cite ADR-0030 for the prediction; correct the stale figures | One unbroken line across three ADRs, as ADR-0029's own Downstream Consumers requires | ✓ |
| Write a new version ADR for v0.3.0-v0.6.0 | Explicitly prohibited by Phase 10's hand-off (item 3) | |
| Re-close the numbering prediction here | ADR-0030:79-84 already closed it, deliberately | |

**Choice:** Append and cite (D-16, D-17), plus source corrections (D-18).
**Notes:** Two fresh findings. (1) ORCH-05's second half is already discharged —
`0030-milestone-7-self-numbering.md:79-84` records the fifth-instance prediction closed "so no later
phase inherits a standing prediction to check", Phase 10's D-14, dated 2026-08-08. ORCH-05 shrinks to
citing it plus the one check it owns: confirming run-5 provenance keys resolve against directory
numbering. If that finds a fifth collision, ADR-0030 is amended in place, never rivalled. (2)
ORCH-05's current-state clause is two releases stale — it says `Cargo.toml` `0.6.0` / latest tag
`v0.5.1`; measured, `Cargo.toml:34` is `0.7.0` and `v0.7.1`/`v0.7.0` both exist. **This is the same
sentence Phase 10's D-11 corrected in HARD-03, regrown one requirement later.** REL-01 stays closed.

---

## Code-change boundary and ADR allocation (cross-cutting)

| Option | Description | Selected |
|--------|-------------|----------|
| Record-writing + a three-file `docs/src/` surface; no `.rs`; ADRs 0037-0039 | Mirrors Phase 10's D-23 with the surface moved from config to docs | ✓ |
| Record-only, zero in-tree changes | Leaves two published documentation defects live | |
| Include the `AgentProvisioner` move | `.rs` work across two published crates; outside a ground-truth phase | |

**Choice:** Three documentation lines, no `.rs`, three ADRs (D-19, D-20).
**Notes:** Permitted surface is exactly `sidecar.md:29`, `http-service-host.md:54` (plus its stated
limitation) and `overview.md`. Boundary is checkable, not asserted:
`git diff --name-only <base>..HEAD -- '*.rs' | wc -l` → `0` at close-out, the same command Phase 10
used. `PROMOTION.md:60` confirms next free is **0037**; advance to **0040**. ORCH-01, ORCH-02 and
ORCH-05 get no ADR (D-00g). Decomposition: ~11 plans in 4 waves (D-23), with plan ⑧ gated on a
blocking `checkpoint:decision` because D-14 and D-15 are both flagged and both rated `costly`. Three
forward hand-off blocks owed — Phase 14, Phase 15, Phase 16 (D-22) — and this is the last
ground-truth phase, so they are the corpus's final forward-work signal.

---

## Claude's Discretion

Recorded in CONTEXT.md `<decisions>` § *Claude's Discretion*:

- Placement of the two head-note class tables (head, foot, or both).
- Whether ADR-0038 and ADR-0039 are two files or one (two recommended; both must stay separately
  citable for Phase 14).
- Where ADR-0039's limitation lands in `overview.md` — the `Avoid when` cell, a note under the table,
  or both.
- Inline-per-row vs cross-reference-block presentation of run 5's 37 verified-shipped claims.
- Banner wording and inline-correction markup for `.project/` annotations.
- Whether ORCH-05's provenance-key confirmation rides in plan ⑩ or the close-out.
- Ordering of the Deferred-QA section (epic number vs the register's recommended implementation
  order, 25 → 28 → 29 → 26/27).
- Whether D-08's corrected 15-job `ci.yml` list is written into the ledger head note, PIPE-01's text,
  or both.

## Deferred Ideas

Nineteen items recorded in CONTEXT.md `<deferred>`. New this phase:

- **Wiring Garrison and Arsenal into HTTP-served agents** — the option D-15 declined; a milestone, not
  a clause, if overturned.
- **Promoting `AgentProvisioner` to `paladin-ports`** — the option D-14 declined, with the shape the
  move would have to take recorded so a later reader does not re-derive it.
- **Owner and expiry fields for the 13 `deny.toml` ignores that have neither** — run-5 finding 2's
  governance gap; no phase owns it, candidate for the milestone close-out.
- **A third clause for Phase 15's `cargo tree` allowlist check** — that `paladin-ports` acquires no
  OpenAPI or web dependency (from D-14).

Carried forward unresolved from earlier phases: the Deferred-QA coverage-threshold variant (78% hard
gate vs 70→74→78 ramp, Phase 15); the eight deprecated GitHub Action references (Phase 15 / PIPE-04);
the 311-line architecture rewrite and the empty `docs/assets/` (Phase 16); `scraper`/`rss`/
`tiktoken-rs`; the seven `doctest = false` crates; the `cargo tree` allowlist check; the unapplied
GitHub rulesets (owner-only); stray root artefacts; `dotenv` → `dotenvy`; a `SECURITY.md`; retiring
`src/main.rs`; Nyquist validation for Phases 1-4; and **whether ADRs should be published to the
mdbook — now unanswered across seven phases, and this phase adds three more ADRs to the unpublished
set.**
</content>
