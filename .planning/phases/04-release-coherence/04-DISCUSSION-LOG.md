# Phase 4: Release Coherence - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.
> Decisions are captured in CONTEXT.md — this log preserves the alternatives considered.

**Date:** 2026-08-02
**Phase:** 4-release-coherence
**Mode:** `--auto` — all gray areas auto-selected, every question answered with the recommended
option. No `AskUserQuestion` prompts were issued.
**Areas discussed:** Version convergence, Rust edition, Advisory posture, QUICKSTART measurement,
Gate-suite proof, Documentation final review

`[--auto] Selected all gray areas: Version convergence, Rust edition, Advisory posture, QUICKSTART
measurement, Gate-suite proof, Documentation final review.`

---

## Version convergence (REL-01, SC1)

**Q: What version does Phase 4 converge on?**

| Option | Description | Selected |
|--------|-------------|----------|
| `0.7.0` (recommended) | Matches the branch name; next lockstep minor after the M9→0.3.0 … M12→0.6.0 chain; not an rc figure, which HARD-03 forbids | ✓ |
| `1.0.0` | Treats Milestone 6's facade removal as the breaking change that forces a major bump (ARCH-04's open question) | |
| `0.6.0` | Converge downward — tag the version the manifests already declare | |

`[auto] Version convergence — Q: "What version does Phase 4 converge on?" → Selected: "0.7.0" (recommended default)`

**Notes:** The M6 facade removal *was* breaking, but it already shipped inside the 0.x series —
under SemVer a pre-1.0 project expresses breaking changes as minor bumps. `1.0.0` would additionally
assert an API-stability guarantee the corpus has not established, in a project still resolving port
ownership (ARCH-03(c)). Captured as **D-01**.

**Q: Phase 4 runs before ARCH-04 (Phase 7), which the coupling table names as the recording owner. Who records?**

| Option | Description | Selected |
|--------|-------------|----------|
| Phase 4 records it as an ADR (recommended) | Same "whichever executes first records, the other applies" convention REL-02's own text already states for the edition | ✓ |
| Block on Phase 7 | Wait for ARCH-04 before touching versions | |
| Record informally in the ledger | Capture the figure without ADR protection | |

`[auto] Version convergence — Q: "Who records the major-bump answer?" → Selected: "Phase 4 records it as an ADR" (recommended default)`

**Notes:** Blocking creates a circular wait (ARCH-04 → REL-01 → ARCH-04) nine phases deep, in a
phase whose entire purpose is ending three-way disagreements. An informal record sits at DOC
precedence and is auto-overridable by the next document that mentions a version — the exact failure
PROJECT.md's "not one protected decision in twelve milestones" finding names. Captured as **D-02**.

**Q: How far toward an actual release does Phase 4 go?**

| Option | Description | Selected |
|--------|-------------|----------|
| Bump + changelog + local unpushed tag (recommended) | Makes the tree coherent and verifiable; leaves the irreversible step with a human | ✓ |
| Full `make release` including push | Push triggers `release.yml` → publishes ten crates to crates.io | |
| Manifests + changelog only, no tag | Discharge SC1's "git tag" clause with a documented command instead | |

`[auto] Version convergence — Q: "How far toward an actual release?" → Selected: "Bump + changelog + local unpushed tag" (recommended default)`

**Notes:** Publishing ten crates at a lockstep version cannot be undone, only yanked. `release.toml`
already sets `publish = false` / `push = false`, so `cargo release version` is safe; `make release`
is not, because it pushes. Captured as **D-03**; the third option is preserved as a discretion item.

---

## Rust edition (REL-02, SC2)

**Q: Which direction closes the 2024/2021 split?**

| Option | Description | Selected |
|--------|-------------|----------|
| Standardize on 2024 (recommended) | Bump the two stragglers, `paladin-ports` and `paladin-notifications` | ✓ |
| Standardize on 2021 | What `codebase/CONCERNS.md:25` recommends — downgrade the nine 2024 crates | |
| Leave the split, document it | SC2 forbids this | |

`[auto] Rust edition — Q: "Which direction closes the split?" → Selected: "Standardize on 2024" (recommended default)`

**Notes:** Two manifests forward versus nine backward. The CONCERNS.md recommendation rests on a
claim verified false during this discussion — see the next question. Captured as **D-04**.

**Q: `codebase/CONCERNS.md` says edition 2024 "does not exist in Rust's stable channel". What do we do with that?**

| Option | Description | Selected |
|--------|-------------|----------|
| Amend at source with dated provenance (recommended) | Rust 2024 stabilized in 1.85; the toolchain is pinned at 1.97.1 — the claim is wrong and the map loses to the tree | ✓ |
| Silently ignore it | Make the decision without addressing the contradicting map | |
| Treat the map as authoritative | Follow the precedence order literally in the wrong direction | |

`[auto] Rust edition — Q: "What do we do with the stale CONCERNS.md claim?" → Selected: "Amend at source with dated provenance" (recommended default)`

**Notes:** This is the project's precedence order at full strength (shipped tree beats
`.planning/codebase/` map) and follows the in-place, dated amendment convention Phases 1-3
established. Silently ignoring it leaves a future reader unable to tell correction from drift —
Phase 3's D-03 rationale, verbatim. Captured as **D-05**; the ADR is **D-06**.

---

## Advisory posture (REL-03, SC3)

**Q: SC3 says "no high or critical advisories". Both gates were run live and both pass. What is the deliverable?**

| Option | Description | Selected |
|--------|-------------|----------|
| Record the measurement to the provenance standard (recommended) | The gate is already green; the deliverable is a dated, reproducible record | ✓ |
| Treat REQUIREMENTS.md's "Current:" line as five open defects | Remediate the 2 medium + 3 feature-gated advisories | |
| Upgrade the transitive dependencies | Chase upstream fixes that do not exist | |

`[auto] Advisory posture — Q: "What is SC3's deliverable?" → Selected: "Record the measurement" (recommended default)`

**Notes:** `cargo audit` → 0 vulnerabilities, 13 allowed warnings; `cargo deny check` → all four
checks ok. REL-03's "Current:" line describes a *suppression inventory*, not a failing gate.
Captured as **D-07**.

**Q: How much of the suppression governance does Phase 4 take?**

| Option | Description | Selected |
|--------|-------------|----------|
| Rationale + migration notes only; hand off owner/expiry (recommended) | SC3's own words; SEC-01 and SUPPLY-02 own the schema and the 2026-09-30 expiry | ✓ |
| Take the whole governance surface | Add owner and expiry fields here | |
| Take nothing | Leave SC3's second half unaddressed | |

`[auto] Advisory posture — Q: "How much governance does Phase 4 take?" → Selected: "Rationale + migration notes only" (recommended default)`

**Notes:** Inventing a second owner/expiry schema ahead of SUPPLY-02 would create the exact
duplicate-governance problem this corpus keeps closing. Captured as **D-09**.

**Q: What about the two concrete `deny.toml` / `ci.yml` findings?**

| Option | Description | Selected |
|--------|-------------|----------|
| Remove the stale ignore; measure and hand off the duplicate CI job (recommended) | `RUSTSEC-2025-0121` matches nothing; the duplicate audit job is SUPPLY-01's | ✓ |
| Fix both here | Pull SUPPLY-01's 18-line deletion forward | |
| Fix neither | Leave a suppression that cannot fire | |

`[auto] Advisory posture — Q: "What about the deny.toml and ci.yml findings?" → Selected: "Remove the stale ignore; measure and hand off the duplicate job" (recommended default)`

**Notes:** Verified live that the duplicate job's inline `--ignore` flags *augment* rather than
replace `.cargo/audit.toml`, so it exits `0` and does not block SC5 — which is what makes the
hand-off safe. Deleting it is Phase 12's payoff (a Milestone 10 acceptance criterion becomes true).
Captured as **D-08**. The four newly-surfaced advisories were recorded and handed off rather than
suppressed (**D-09**).

---

## QUICKSTART measurement (REL-04, SC4)

**Q: The documented target is contested — 15 minutes vs "under five minutes". Which governs?**

| Option | Description | Selected |
|--------|-------------|----------|
| 15 minutes, reconcile the page after measuring (recommended) | The figure REL-04, ROADMAP SC4 and `introduction.md:9` all carry | ✓ |
| Five minutes | Hold the quickstart page's own tighter claim as the gate | |
| Leave both | Two numbers, which is what this milestone exists to end | |

`[auto] QUICKSTART measurement — Q: "Which target governs?" → Selected: "15 minutes, reconcile after measuring" (recommended default)`

**Notes:** Two of three doc references support 15. If the measurement comes in under five, the page
keeps its claim and the measurement becomes its evidence — decide after measuring, not before.
Captured as **D-11.1**.

**Q: "Clean machine" is unreachable here (warm registry, crates.io 403, no Docker). How is SC4 discharged?**

| Option | Description | Selected |
|--------|-------------|----------|
| Measure under stated conditions; defer the clean-machine figure (recommended) | Phase 3's provenance discipline applied to a timing rather than a coverage number | ✓ |
| Skip the measurement | Leave SC4 unmeasured a fourth time | |
| Report the constrained figure as a clean-machine result | Fabrication | |

`[auto] QUICKSTART measurement — Q: "How is SC4 discharged?" → Selected: "Measure under stated conditions; defer the clean-machine figure" (recommended default)`

**Notes:** ROADMAP SC4 says "measured for the first time, **pass or fail**" — a fail is a legitimate
recordable outcome, and the QUICKSTART must not be tuned to hit a number. Captured as **D-11.2/.3**.

---

## Gate-suite proof (REL-05, SC5)

**Q: CI cannot run here, and three of SC5's named gates have no CI job at all. What does Phase 4 do?**

| Option | Description | Selected |
|--------|-------------|----------|
| Run locally what can run; repair CI so it *can* prove the rest (recommended) | SC5 requires CI to prove these; jobs that do not exist cannot prove anything | ✓ |
| Run locally only | Leave CI unable to prove SC5 on the release branch | |
| Author CI only | Skip the local proof | |

`[auto] Gate-suite proof — Q: "What does Phase 4 do about CI?" → Selected: "Run locally what can run; repair CI" (recommended default)`

**Notes:** Three verified gaps: `ci.yml`'s `push` trigger is commented out (line 9) so a push to
`release/v0.7.0` runs nothing; no job builds the examples; no Kubernetes smoke job exists and the
docker job asserts no size/time budget. Adding these three is SC5's own scope and is distinct from
Phase 15's PIPE register (`cli-tests`, `bench-check`, `coverage`, `.codecov.yml`, deprecated
actions), which stays untouched. Captured as **D-12, D-14**.

**Q: The new Docker and Kubernetes jobs cannot be executed here. How are they reported?**

| Option | Description | Selected |
|--------|-------------|----------|
| Statically validate, then defer with reason and an owner (recommended) | Configuration authored is not a gate proven | ✓ |
| Report SC5 met on the strength of the configuration | The phase's largest honesty risk | |
| Do not author them | Leaves SC5 permanently unprovable | |

`[auto] Gate-suite proof — Q: "How are the unexecutable jobs reported?" → Selected: "Statically validate, then defer with reason and an owner" (recommended default)`

**Notes:** `docker`, `kind` and `kubectl` are all absent from this environment. Same discipline
Phase 3 applied to unmeasurable coverage. Captured as **D-15**; `gh` usage is read-only per **D-16**.

**Q: SC5 says "all 22 examples", but the tree has 47. Which is the gate?**

| Option | Description | Selected |
|--------|-------------|----------|
| Gate on "every example target builds"; amend the count at source (recommended) | Phase 3's D-03 pattern — an ingested count loses to a measurement of the tree | ✓ |
| Gate on 22 | Take the stale figure literally | |
| Substitute 47 silently | Leaves a reader unable to tell correction from drift | |

`[auto] Gate-suite proof — Q: "Which examples figure is the gate?" → Selected: "Gate on every example target building; amend at source" (recommended default)`

**Notes:** The "22" traces to a Milestone 1 Epic 10 validation report and has been restated in five
places across ROADMAP.md, PROJECT.md and REQUIREMENTS.md. Captured as **D-13**.

---

## Documentation final review (REL-04, first clause)

**Q: What does "documentation final review is complete per the RECON-08 answer" require?**

| Option | Description | Selected |
|--------|-------------|----------|
| Nothing — cite the verdict and move on (recommended) | RECON-08 recorded that no Task 7.0 exists; Epic 10 is `satisfied` with no owner needed | ✓ |
| Conduct a documentation review | Invent the deliverable the validation report claimed | |
| Re-open the dispute | Explicitly forbidden by the ledger verdict | |

`[auto] Documentation final review — Q: "What does REL-04's first clause require?" → Selected: "Nothing — cite the verdict and move on" (recommended default)`

**Notes:** The ledger's resolution is unusually well-evidenced: the task list is 103/103 with no
Task 7.0 heading, independently corroborated by `intel/task-completion-state.md`, and "Final
Documentation Review" appears in exactly two lines of one document across all 263 corpus files and
zero times in `docs/`. The absence is the evidence. This removes a whole invented deliverable from
the phase — the live half of REL-04 is the QUICKSTART measurement and nothing else. Captured as
**D-10**.

---

## Claude's Discretion

Auto mode resolved every gray area, so nothing was deferred to Claude by a user. The following were
recorded as genuinely open for the planner (see CONTEXT.md §"Claude's Discretion"):

- Plan decomposition and count.
- Where the ROADMAP / REQUIREMENTS / PROJECT / CONCERNS amendments physically land (Phase 2's route
  vs Phase 3's).
- Whether the `v0.7.0` tag is created locally at all, or SC1 is discharged by manifests + CHANGELOG
  + a documented tag command.
- Whether the dual `reqwest` 0.12.28 / 0.13.4 exposure gets a written note in this phase.
- Whether `## [Unreleased]`'s "Phase 12.1" heading — old `.project/` milestone numbering, not a GSD
  phase — is renamed during the CHANGELOG finalize.

## Deferred Ideas

Recorded in full in CONTEXT.md `<deferred>`. Summary of owners:

- **SUPPLY-01 (Phase 12)** — delete the duplicate `Security Audit` job, `ci.yml:389-406`.
- **SEC-01 (Phase 9) / SUPPLY-02 (Phase 12)** — advisory owner/expiry schema, the 2026-09-30
  expiry, the three unratified 2026 ignores, and the four newly-surfaced advisories found here.
- **SEC-02 (Phase 9)** — the licence three-way (MIT vs `MIT OR Apache-2.0`).
- **DEBT-01 (Phase 8)** — the permanently-red `api-surface` job.
- **DEBT-03 (Phase 8) / HARD-07 (Phase 10)** — `paladin-ports` doctests and the governing
  `cargo doc` bar.
- **PIPE-01 … PIPE-04 (Phase 15)** — `cli-tests` / `bench-check` / `coverage` jobs, `.codecov.yml`,
  Makefile coverage targets, deprecated actions, the 84% floor in CI, first execution of the new
  Docker/Kubernetes jobs, and the clean-machine QUICKSTART timing.
- **Human, gated** — pushing `v0.7.0` and publishing ten crates to crates.io.
