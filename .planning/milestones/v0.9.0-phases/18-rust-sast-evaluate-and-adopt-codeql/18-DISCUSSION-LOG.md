# Phase 18: Rust SAST — Evaluate and Adopt CodeQL - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.
> Decisions are captured in CONTEXT.md — this log preserves the alternatives considered.

**Date:** 2026-08-25
**Phase:** 18-rust-sast-evaluate-and-adopt-codeql
**Areas discussed:** Scanner setup mode, Workflow placement & trigger shape, Probe fixture
lifecycle, Feature-gated analysis coverage, Observation-window method, Alert-triage governance,
Promotion execution, Semgrep scope, SAST-04 documentation blast radius

**Mode:** `--auto`. No questions were put to the user; the recommended option was taken on every
area and the reasoning recorded. Every row below marked ✓ is an auto-selection, not a user choice.

---

## Scanner setup mode

| Option | Description | Selected |
|--------|-------------|----------|
| Advanced setup (committed workflow) | Query suite, build mode and feature set reviewable in a PR; job name declared in-tree | ✓ |
| Default setup (repo settings) | Zero-config, but declares no job in any workflow file | |

**Choice:** Advanced setup → **D-01**.
**Notes:** Not a preference. `scripts/check-workflow-triggers.sh` Clause 3 requires every pinned
required-status-check context to resolve to a job display name declared in a workflow file.
Default setup declares none, so `SAST-03`'s promotion would fail Clause 3 at the moment it fires.

---

## Job naming

| Option | Description | Selected |
|--------|-------------|----------|
| Single literal name `CodeQL Analysis (Rust)` | Resolves exactly under Clause 3 | ✓ |
| `strategy.matrix` on language | Clause 3 prefix-matches matrix jobs; works but looser | |

**Choice:** Single literal → **D-02**.
**Notes:** A single-language repository gains nothing from a matrix. The name becomes a pinned
contract on promotion.

---

## Workflow placement and trigger shape

| Option | Description | Selected |
|--------|-------------|----------|
| New `.github/workflows/codeql.yml` | Own triggers including `schedule`; own register row | ✓ |
| Job inside `ci.yml` + add `schedule:` there | Would fire the whole hour-plus pipeline weekly | |

**Choice:** New workflow file → **D-03**.
**Notes:** `branching-model.md`'s `ci.yml` row records, explicitly, why `ci.yml` carries no
`schedule:` key. Adding one for CodeQL's convenience would overturn a recorded decision.

| Option | Description | Selected |
|--------|-------------|----------|
| `push: ['**']` | Conforms to Clause 2 unchanged; superset of `main` | ✓ |
| `push: [main]` + third guard exception | Matches `SAST-02`'s literal wording; requires editing `EXCEPTION_FILES` in the guard | |

**Choice:** `['**']` → **D-04**.
**Notes:** The tension is real and worth recording. `SAST-02` and PROJECT.md both say "push on
`main`", but Clause 2 (line 309, `EXCEPTION_FILES` at line 118) fails any workflow whose push
filter is a list other than `['**']` outside `docs.yml`/`release.yml`. Weakening a guard inside
the phase whose purpose is not weakening assurance was rejected; over-coverage satisfies the
requirement without an exemption. Duplicate push+PR runs are absorbed by `ci.yml`'s `concurrency`
pattern.

---

## Non-blocking posture

| Option | Description | Selected |
|--------|-------------|----------|
| Context simply not yet in the ruleset | Job fails honestly; it just does not gate a merge | ✓ |
| `continue-on-error: true` on the scan step | Mirrors the existing `osv-scanner` job | |

**Choice:** Not-yet-pinned → **D-06**.
**Notes:** Success Criterion 6 forbids leaving a `continue-on-error` step reporting success on a
job that did not run. The existing `osv-scanner` job stacks `continue-on-error` on both the scan
and the SARIF upload; that pattern was deliberately not copied.

---

## Probe fixture lifecycle

| Option | Description | Selected |
|--------|-------------|----------|
| In-tree crate, excluded from workspace, scanned on demand | Evidence reproducible; no standing alerts | ✓ |
| Throwaway fixture on a scratch branch, deleted at close | Cheapest; nobody can re-derive the number later | |
| Permanent fixture inside the steady-state PR scan | Four standing alerts forever, needing standing dismissals | |

**Choice:** Excluded crate, dispatch-scanned → **D-07**, **D-09**.
**Notes:** Reproducibility is the point — an unreproducible number repeats the weakness the Snyk
episode exposed. But permanently scanning planted vulnerabilities manufactures exactly the
dismissal habit that erodes a gate's meaning.

| Option | Description | Selected |
|--------|-------------|----------|
| Reuse Snyk's four classes verbatim | Finding counts directly comparable to the 0-vs-3 baseline | ✓ |
| Design a broader/better probe | Produces a number comparable to nothing | |

**Choice:** Verbatim reuse → **D-08**.

---

## Feature-gated analysis coverage

| Option | Description | Selected |
|--------|-------------|----------|
| Measure and record analysed-file count; reach feature-gated code | Separates "found nothing" from "analysed nothing" | ✓ |
| Accept whatever the default configuration analyses | Silent coverage hole; Snyk's failure shape via a different mechanism | |

**Choice:** Measure it → **D-12**, **D-13**.
**Notes:** Denominator settled at **385** and verified against the tree: `crates/**/*.rs` (246) +
`src/**/*.rs` (139) = 385, matching the figure `SAST-03` and PROJECT.md already cite. The full
tree is 575 `.rs` files / ~196k lines including `tests/`, `examples/`, `benches/` and
`doc-examples`; 385 / ~142k is the correct first-party denominator.

---

## Observation-window method

| Option | Description | Selected |
|--------|-------------|----------|
| Backfill over recent merged PR head commits + short live advisory period | Real numbers on real diffs without waiting on calendar time | ✓ |
| Fixed 14-day live window | Idles the phase; invites "recorded as open" instead of settled | |
| Single run on `main` | Produces one data point, no false-positive rate | |

**Choice:** Backfill + short live period → **D-14**, **D-15**.
**Notes:** Metrics fixed as alerts raised / true-positive / false-positive / FP rate / wall-clock
cold and warm / analysed-file count. Evidence lands in `.planning/`; only the conclusion
propagates into `security.instructions.md`.

---

## Alert-triage governance

| Option | Description | Selected |
|--------|-------------|----------|
| Governed register modelled on `SECURITY-EXCEPTIONS.md` | Owner, review date, scope, compensating control per dismissal | ✓ |
| Dismiss in the GitHub UI with a free-text reason | Unaudited escape hatch; gate decays into theatre | |

**Choice:** Governed register → **D-17**.
**Notes:** File-vs-section and whether a `check-*.sh` guard enforces it left to planning;
ungoverned dismissal ruled out.

---

## Promotion execution

| Option | Description | Selected |
|--------|-------------|----------|
| Promote within this phase if the numbers qualify | Settles the question; falls back to a *named* open item with a trigger condition | ✓ |
| Always defer promotion to follow-up work | Reproduces the "open item" pattern the v0.8.0 audit criticised | |
| Promote immediately without the window | Directly violates `SAST-03` | |

**Choice:** Conditional in-phase promotion → **D-18**, **D-19**.
**Notes:** Four-place update enumerated, including the detail that `branch-protection.md` carries
the count `44` in prose at lines 85, 117 and 180 — not only in the context table. Moving the table
alone leaves the document contradicting itself.

---

## Semgrep scope

| Option | Description | Selected |
|--------|-------------|----------|
| Contingency only — evaluate if CodeQL fails the probe | Keeps the phase bounded | ✓ |
| Evaluate both up front as complements | Doubles the phase for a tool already judged secondary | |

**Choice:** Contingency → **D-20**.
**Notes:** PROJECT.md already records Semgrep as pattern matching rather than interprocedural
taint analysis, with thin Rust rule coverage being "the same failure shape as Snyk".

---

## SAST-04 documentation blast radius

| Option | Description | Selected |
|--------|-------------|----------|
| Rewrite every place that asserts the gap, in one change | No document left contradicting the measured outcome | ✓ |
| Rewrite only the named `security.instructions.md` section | Leaves line 26's taint claim and `CLAUDE.md` stale | |

**Choice:** All assertions of the gap → **D-21**, **D-22**.
**Notes:** The Snyk prohibition section stays — it is a standing rule, not stale text. The section
is narrowed by evidence, never deleted.

---

## Claude's Discretion

Left open for research and planning: CodeQL build mode and query suite (`security-extended` vs
default), schedule cadence, cache strategy, probe crate file layout, register file-vs-section, and
backfill sample size.

## Deferred Ideas

- Remediating real findings the scanner surfaces — triaged here, fixed in follow-up work.
- A GitHub-facing `SECURITY.md` — `SECURITY-EXCEPTIONS.md` already records it as a separate
  deliverable for a different audience.
- Semgrep as a standing complement alongside a qualified CodeQL.
- Refreshing `.planning/codebase/INTEGRATIONS.md`, whose "CI Pipeline: Not integrated in this
  codebase" claim is stale by six workflows.
- **Reviewed todo, not folded:** "Verify local `make coverage` reproduces CI's 82.39% figure"
  (score 0.60). Matched on generic keyword overlap; "coverage" there means test-line coverage, an
  unrelated sense. The scope guardrail outranks the `--auto` fold threshold.
