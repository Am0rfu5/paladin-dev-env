# Phase 1: Ground Truth & Decision Records - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.
> Decisions are captured in CONTEXT.md — this log preserves the alternatives considered.

**Date:** 2026-07-30
**Phase:** 1-ground-truth-decision-records
**Areas discussed:** ADR home & authority, Coverage gate number, Record vs decide, Ledger shape & evidence

---

## Area selection

| Option | Description | Selected |
|--------|-------------|----------|
| ADR home & authority | Where the six ADRs live, what format, what makes them binding, and what happens to the eleven existing candidates | ✓ |
| Coverage gate number | RECON-07 — one number, one scope; feeds VERIFY-05, QUAL-01 and PIPE-02 | ✓ |
| Record vs decide | Whether each ADR ratifies shipped code or decides on merit; sets Phase 2's GAP-07 workload | ✓ |
| Ledger shape & evidence | How RECON-01's ledger is keyed, where it lives, and what clears the evidence bar | ✓ |

**User's choice:** All four.

---

## ADR home & authority

### Q1 — Where should the six Phase 1 ADRs live?

| Option | Description | Selected |
|--------|-------------|----------|
| `.planning/decisions/` | GSD-native, adjacent to PROJECT.md Key Decisions, no mdbook linkcheck gate; Phases 5/7/10/13 append | ✓ |
| `docs/src/appendix/adr/` | Published mdbook chapter — but Milestone 11 Epic 3 made the appendix a rewrite non-goal, and DOCS-02 is already fighting a doc that went invisible there | |
| `docs/adr/` at repo root | Conventional MADR/Nygard location, outside the mdbook — but sits apart from `.planning/` | |

**User's choice:** `.planning/decisions/`

### Q2 — Where do ADRs slot into the precedence order?

| Option | Description | Selected |
|--------|-------------|----------|
| ADRs top the order | ADR → shipped tree → codebase map → code-verification → PRD → DOC → checkbox. An ADR contradicting code is an instruction to change the code | ✓ |
| Above PRD, below the tree | Top documentary authority, tree stays final arbiter — but cannot express "the code is wrong" | |
| No precedence change | ADRs are just files — reproduces the exact failure PROJECT.md names (a PRD outranking an Approved decision) | |

**User's choice:** ADRs top the order.
**Notes:** Consequence recorded — every ADR must carry a `Code conformance` field, since "authoritative" must not be read as "already true". PROJECT.md's stated precedence order needs updating.

### Q3 — What happens to the eleven pre-existing ADR candidates?

| Option | Description | Selected |
|--------|-------------|----------|
| Define mechanism, promote none | Phase 1 authors its six and writes the promotion procedure; the eleven stay with their owning phases, now unblocked | ✓ |
| Also promote battalion-result now | Adds the corpus's only decision/options pair, adjacent to RECON-03 | |
| Promote the two with live cost | battalion-result plus the RustSec acceptance (expiry 2026-09-30) | |
| Six only, no mechanism | Smallest scope, but every later phase re-derives how to promote | |

**User's choice:** Define mechanism, promote none.

### Q4 — What shape should each ADR file take?

| Option | Description | Selected |
|--------|-------------|----------|
| Lean, evidence-first | Status · Date · Question · Chosen variant · Evidence (file:line) · Rejected variants · Code conformance · Downstream consumers | ✓ |
| MADR standard | Widely recognised, tool-friendly — but its Considered Options section restates REQUIREMENTS.md's variant groups verbatim | |
| Mirror the corpus's example | Milestone_5 house style: decision file + `-options.md` sibling; two files per decision, twelve for the phase | |

**User's choice:** Lean, evidence-first.

---

## Coverage gate number

### Q1 — Re-measure before recording?

| Option | Description | Selected |
|--------|-------------|----------|
| Re-measure first | Run `cargo llvm-cov`; the ADR records the gate with the measured figure, command and date | ✓ |
| Use the most recent figure | 76-77% from the Deferred-QA register — newest documented, but Feb 2026 with two stale module paths | |
| Record gate without a baseline | Phase 1 decides, Phase 3 measures — but the number is chosen blind | |

**User's choice:** Re-measure first.
**Notes:** All three documented baselines disagree — 60.88% unit / 67.79% integration (predates Milestones 2-12), ~78% overall, 76-77%.

### Q2 — What scope does the gate cover?

| Option | Description | Selected |
|--------|-------------|----------|
| One workspace-wide number | Line coverage, all tests, one command. What RECON-07 literally asks for and what a CI gate needs | ✓ |
| Split: unit + integration | Matches nine M1 PRDs and Phase 3's criterion as written — but two numbers where RECON-07 asked for one | |
| Overall + per-layer floors | Keeps the hexagonal signal (core ≥ 85, app ≥ 80, adapters ≥ 70) — more to maintain | |

**User's choice:** One workspace-wide number.
**Notes:** Requires amending Phase 3's success criterion 1, which names unit and integration separately.

### Q3 — What rule sets the number?

| Option | Description | Selected |
|--------|-------------|----------|
| No-regression floor + target | Gate = re-measured baseline rounded down, hard-fail from day one; 80% recorded as target with a ratchet trigger | ✓ |
| 80% hard gate now | Most-asserted position (nine M1 PRDs + Epic 24) — but Phase 15's CI gate stays red until Phase 3 lands | |
| Phased ramp | Epic 25's shape — three numbers where RECON-07 asked for one; each step needs an owner or it stalls | |

**User's choice:** No-regression floor + recorded target.
**Notes:** This answers the parent PRD's Open Question 3 (hard fail vs soft warning) as hard-fail, without the ramp-up problem it flagged.

### Q4 — Who settles the module-scoped gates (Herald ≥ 95%, autonomous ≥ 90%)?

| Option | Description | Selected |
|--------|-------------|----------|
| Record, hand off to Phase 5 | RECON-07 names both, states they sit above the floor and are not withdrawn; VERIFY-05 places them | ✓ |
| Settle them in Phase 1 | One ADR covers everything — but decides Milestone 2-3 scope without run-2 verification | |
| Withdraw them | One gate, no exceptions — but loses real signal from Herald's cheap-to-cover surface | |

**User's choice:** Record, hand off to Phase 5.

---

## Record vs decide

### Q1 — Default posture when code and documents disagree?

| Option | Description | Selected |
|--------|-------------|----------|
| Code wins unless argued | Deviation requires a written reason and a "code must change" flag routing to GAP-07 | ✓ |
| Decide on merit, code follows | Cleanest decisions — but inverts the precedence order just established and grows Phase 2 unpredictably | |
| Case by case, no default | Most honest per-decision — but a reader cannot tell reasoned from defaulted | |

**User's choice:** Code wins unless argued.

### Q2 — RECON-04, Formation minimum Paladin count

| Option | Description | Selected |
|--------|-------------|----------|
| Formation accepts 1 | Relax `formation.rs:109` to ≥ 1; leaves the passing Commander test untouched; matches Phase 2's criterion | ✓ |
| Commander stops routing 1 to Formation | Architecturally cleaner — but breaks the passing test at `commander.rs:1912` and contradicts Phase 2's criterion | |
| Reject at build time | Fails fast at `CommanderBuilder::build()` — but removes single-Paladin Commander as a capability | |

**User's choice:** Formation accepts 1.
**Notes:** Verified live during discussion — shipped code contains *both* halves of the contradiction, including a passing test asserting the Auto routing. "Code wins" cannot resolve it; this is a second Group-29-class variant.

### Q3 — RECON-05, temperature validation

| Option | Description | Selected |
|--------|-------------|----------|
| Provider-aware via capabilities | Add a temperature range to `ProviderCapabilities`; makes DeepSeek's 0.0-2.0 reachable | ✓ |
| Global [0.0, 1.0] clamp | Zero code change — but requires recording Epic 6 REQ-5 as withdrawn | |
| Clamp at the adapter | No ports change, full range per provider — but silent clamping past the point the caller can be told | |

**User's choice:** Provider-aware via capabilities.
**Notes:** Verified during discussion — `ProviderCapabilities` (`llm_port.rs:754`) has no temperature-range field at all, so the provider-aware position was never implementable as specified. Sequence with Phase 14's WEB-03, which corrects the same struct.

### Q4 — RECON-02, the `citadel.rs` `BattalionConfig` duplicate

| Option | Description | Selected |
|--------|-------------|----------|
| Rename it, keep the shape | Becomes e.g. `BattalionCheckpointConfig`; same fields, same serde shape, no migration | ✓ |
| Replace it with the real one | One type exactly as RECON-02 asks — but changes the persisted schema and needs a version bump plus a read path | |
| Delete it, inline the fields | Fewest types — migration risk without option 2's benefit, and loses the grouping | |

**User's choice:** Rename it, keep the shape.
**Notes:** Verified during discussion — the struct's own doc comment reads "This is a placeholder and will be expanded in Epic 4." Epic 4 expanded it elsewhere and the placeholder survived. It is `Serialize`/`Deserialize` inside `BattalionState` (`schema_version: "1.0.0"`), which is the constraint that rules out option 2.

---

## Ledger shape & evidence

### Q1 — New file, or extend REQUIREMENTS.md?

| Option | Description | Selected |
|--------|-------------|----------|
| New file per milestone | `.planning/ledgers/milestone-01.md`; REQUIREMENTS.md's section becomes a pointer; four siblings follow | ✓ |
| Extend REQUIREMENTS.md in place | One file, no drift — but compounds a ~4,000-line file, four more times | |
| One file per epic | Finest granularity — but ten files where SC1 asks for one openable ledger | |

**User's choice:** New file per milestone.

### Q2 — What's the ledger's primary key?

| Option | Description | Selected |
|--------|-------------|----------|
| `REQ-*` ID, task items nested | Stable keys, joinable to REQUIREMENTS.md and the roadmap; satisfies RECON-01 without inventing checkbox IDs | ✓ |
| Task item | Most literal reading — but task items have no stable identifiers | |
| Epic | Compact, matches Phase 5's wording — but too coarse to carry a file:line per item | |

**User's choice:** `REQ-*` ID with task items nested.

### Q3 — What clears the bar for "satisfied"?

| Option | Description | Selected |
|--------|-------------|----------|
| Cited + proven to run | file:line **plus** a named passing test/example/command; otherwise "present, unproven" | ✓ |
| file:line citation is enough | Cheapest across five ledgers — but cannot distinguish shipped-and-working from shipped-and-dead | |
| Tiered, no single bar | Maximally transparent — but "inferred" becomes the path of least resistance | |

**User's choice:** Cited + proven to run.
**Notes:** Precedent cited — Milestone 4 Epic 3's task list is fully checked while three CLI-only dependencies remain unconditional. Expect a third bucket nobody has counted.

### Q4 — Where do the three divergences land?

| Option | Description | Selected |
|--------|-------------|----------|
| A verdict class in the ledger | "Superseded by shipped code" alongside satisfied / outstanding / deferred | ✓ |
| Their own ADRs | Each is a real unrecorded technical decision — but grows Phase 1 from six ADRs to nine | |
| A separate divergence register | One standing artefact across all five ground-truth phases — but a third document class | |

**User's choice:** A verdict class in the ledger.
**Notes:** The interactive REPL row should be flagged loudly — a documented non-goal (Epic 9 NG-7) that shipped anyway.

---

## Claude's Discretion

- ADR file naming and numbering within `.planning/decisions/` (must stay stable as four more phases append).
- Whether an ADR can be superseded, and by what mechanism — undefined, and now consequential given ADRs top precedence.
- Coverage measurement exclusions (`examples/`, `benches/`, `doc-examples`, doctests) — must be pinned in the RECON-07 ADR itself.
- The concrete ratchet trigger for raising the coverage floor toward 80%.
- The renamed identifier for the `citadel.rs` struct (`BattalionCheckpointConfig` was an example, not a locked choice).
- How ledger verification splits across plans — per-epic fan-out vs sequential, across ten epics.

## Deferred Ideas

- Promoting the eleven existing ADR candidates — mechanism built here, promotion owned by Phases 7, 9, 10, 12, 13, 14 and 15.
- The Herald `format_error` fallible/infallible asymmetry as a design question (recorded as shipped; redesign is out of scope).
- Re-scoping Phase 3's testing work to the single coverage number (the roadmap amendment is in scope here; the re-scope is Phase 3's).
- Coverage tooling — `make coverage` and `.codecov.yml`, neither of which exists. Phase 15's PIPE-02.
- Publishing ADRs to the mdbook for framework consumers — rejected as this phase's home; belongs with Phase 16's documentation work.
