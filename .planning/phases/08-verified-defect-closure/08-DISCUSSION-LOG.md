# Phase 8: Verified Defect Closure - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.
> Decisions are captured in CONTEXT.md — this log preserves the alternatives considered.

**Date:** 2026-08-06
**Phase:** 8-verified-defect-closure
**Mode:** `--auto` — all gray areas auto-selected, every question resolved to its recommended
option without human confirmation.
**Areas discussed:** DEBT-01 baseline & fix shape · DEBT-02 implement-or-withdraw ·
DEBT-03 doctest scope · DEBT-04 the two CLI blockers · DEBT-05 consolidation mechanics ·
Cross-cutting (evidence, ADRs, ledger, decomposition)

---

## DEBT-01 — the `api-surface` CI job

### Q1: How should the five stale path references be fixed?

| Option | Description | Selected |
|--------|-------------|----------|
| Correct the five literals | Change `project/` → `.project/` in two scripts and three `ci.yml` lines | ✓ |
| Add fallback resolution | Script tries both paths, or reads an env var | |
| Env-var indirection | Single `API_BASELINE` variable referenced everywhere | |

**Choice:** Correct the five literals (D-01).
**Notes:** A defect that survived three ingest runs because nobody read five literals is not fixed
by creating a sixth place to look. Verified all five in the tree this session.

### Q2: The baseline file `.project/current-exports.txt` is dated 2026-07-06, before Phases 2/3/6 changed code. Regenerate it?

| Option | Description | Selected |
|--------|-------------|----------|
| Regenerate and commit | Run `extract-public-api.sh`, commit the new baseline, record the diff | ✓ |
| Accept the existing baseline | Fix the path only; let the first CI run reveal the drift | |
| Regenerate in a later phase | Hand the refresh to Phase 15's pipeline work | |

**Choice:** Regenerate and commit (D-02).
**Notes:** ROADMAP criterion 1 requires "an unchanged tree makes it pass". A path fix against a stale
baseline produces a job that fails for a different reason — indistinguishable from broken on a CI
dashboard.

### Q3: `cargo public-api` needs nightly plus `cargo install`. What if it cannot run here?

| Option | Description | Selected |
|--------|-------------|----------|
| Land the fix, record a blocker | Path corrected; regeneration procedure documented; closure claim scoped honestly | ✓ |
| Block DEBT-01 entirely | Defer the whole item until tooling is available | |
| Hand-edit the baseline | Approximate the current surface | |

**Choice:** Land the fix, record a blocker (D-03).
**Notes:** The tree builds and tests offline (Phase 7 proved it per-crate), but nothing has proven
`cargo install` works here, and Phase 1's coverage measurement was halted by this class of
constraint. Hand-editing a 442 KB generated artefact was rejected outright.

### Q4: How are the five requirement-text references corrected?

| Option | Description | Selected |
|--------|-------------|----------|
| Both `.project/` annotation and REQUIREMENTS.md | D-00c dated banners at source plus traceability-row updates | ✓ |
| REQUIREMENTS.md only | Correct the planning record, leave the corpus | |
| `.project/` only | Correct the source, let REQUIREMENTS.md derive | |

**Choice:** Both (D-04).
**Notes:** The four M12 clauses were written in June 2026, months after commit `928c6d5` — proof the
defect propagates from the source documents, which is what a future implementer reads.

### Q5: `check-deprecations.sh` — in scope?

| Option | Description | Selected |
|--------|-------------|----------|
| Fix it, narrowly | Make the malformed-attribute check cover `crates` too; make exit status meaningful | ✓ |
| Leave it | DEBT-01 only requires that it be *reached* | |
| Rewrite it as a real gate | Fail when the deprecation policy is violated | |

**Choice:** Fix it, narrowly (D-05).
**Notes:** Verified this session — both branches `exit 0`, so it cannot fail; and its only real check
greps `src/` alone, missing eleven crates. "It gets to execute" is meaningless if it cannot fail.
Rejected the third option because inventing a gate would prejudge DEBT-02.

---

## DEBT-02 — implement or withdraw Milestone 4 Epic 2 FR-8

### Q1: Implement the deprecations or withdraw the requirement?

| Option | Description | Selected |
|--------|-------------|----------|
| Withdraw, recorded in ADR-0022 | Record that the epic's own decisions produce zero deprecations; reconcile the three surfaces | ✓ |
| Implement FR-8 | Add `#[deprecated]` attributes to some set of types | |
| Leave open | Defer to a later phase | |

**Choice:** Withdraw with a recorded reason (D-06). ⚠ Flagged for human review.
**Notes:** The evidence is the epic's own tracking document, not inference. `DEPRECATIONS.md`'s
IMMEDIATE DEPRECATION category — the only one producing a `#[deprecated]` attribute — reads *"None
identified yet"*. Its SOFT DEPRECATION category resolves to `#[doc(hidden)]`, and the tree carries 38
of those, so that half was executed. Its INTERNAL-ONLY category resolves to `pub(crate)`.
Implementing FR-8 would require inventing a candidate list the corpus never produced. Leaving it open
is forbidden by the requirement text's own "no third state" clause.

### Q2: ADR or ledger row?

| Option | Description | Selected |
|--------|-------------|----------|
| ADR-0022 | A withdrawal is a decision with a competing defensible position | ✓ |
| Ledger row only | Treat it as a code-settled divergence | |

**Choice:** ADR-0022 (D-06, D-22).
**Notes:** Phase 7 D-17's rule — contested positions get ADRs, code-settled divergences get ledger
rows. Withdrawing a published API-governance requirement is contested by construction.

### Q3: What happens to `stable-api.md`'s deprecation policy?

| Option | Description | Selected |
|--------|-------------|----------|
| Keep the policy, correct existence claims | Policy survives; any claim that deprecations exist today is corrected | ✓ |
| Delete the deprecation sections | Remove what is now unused | |

**Choice:** Keep the policy (D-07).
**Notes:** The framework will need a deprecation process eventually. The defect is claims of current
state, not the process description.

### Q4: The stale v0.2.0 → v0.3.0 → v1.0.0 timeline?

| Option | Description | Selected |
|--------|-------------|----------|
| Restate against 0.7.0 inside ADR-0022 | Judge the stale artefact, say why, state the replacement | ✓ |
| Drop it silently | Remove the timeline | |
| Preserve it as history | Leave it, marked historical | |

**Choice:** Restate (D-08).
**Notes:** Same treatment ADR-0020 gave the build benchmark and ADR-0006 gave the ~78% coverage
figure. The workspace ships at 0.7.0 (`Cargo.toml:34`, verified), five minor versions past the
timeline's anchor.

---

## DEBT-03 — `paladin-ports` doctests

### Q1: How is the scope determined?

| Option | Description | Selected |
|--------|-------------|----------|
| Measure first | Remove the flag, run the doctests, derive scope from the failure list | ✓ |
| Estimate from the record | Plan against "~25 port traits" from the requirement text | |
| Rewrite all examples defensively | Assume all need updating | |

**Choice:** Measure first (D-09).
**Notes:** **The single biggest finding of this session.** The `doctest = false` comment blames doc
examples referencing the root `paladin::` crate — there are **zero**. All 19 `paladin::`-looking hits
are the module path `paladin_core::platform::container::paladin::Paladin`, and `llm_port.rs:654,671`
already use crate-local paths. The blocker appears to have been fixed and the flag left behind. The
item's true size is somewhere between "already passes" and "187 examples to repair"; guessing wastes
a plan either way.

### Q2: How are failing examples handled?

| Option | Description | Selected |
|--------|-------------|----------|
| Rewrite to compile; `ignore` only for live services, each with a reason | | ✓ |
| Mark failures `ignore` | Fastest path to a green gate | |
| Delete failing examples | Remove rather than repair | |

**Choice:** Rewrite (D-10).
**Notes:** Blanket-`ignore` reproduces the exact failure DEBT-03 exists to close — a documentation
guard configured not to guard. The 87 pre-existing `ignore`/`no_run`/`text` fences are explicitly not
audited here (deferred to Phase 16 / DOCS-03).

### Q3: Sequence the two halves of the guard?

| Option | Description | Selected |
|--------|-------------|----------|
| Same commit | Remove `doctest = false` and `ci.yml:226`'s `--exclude` together | ✓ |
| Separate plans | Manifest first, CI later | |

**Choice:** Same commit (D-11).
**Notes:** Splitting creates a window where the doctests exist and CI still refuses to run them.

### Q4: Block on HARD-07 (Phase 10), which settles the `cargo doc` warning bar?

| Option | Description | Selected |
|--------|-------------|----------|
| Do not block; record the seam | DEBT-03 delivers executing doctests; HARD-07 keeps the warning-bar question | ✓ |
| Sequence Phase 8 behind Phase 10 | Resolve the coupling first | |

**Choice:** Do not block (D-12).
**Notes:** REQUIREMENTS.md says "resolve DEBT-03 together with HARD-07", but the two deliverables are
separable, and sequencing would idle four independent DEBT items behind a question none of them
needs answered.

---

## DEBT-04 — the library-only build

### Q1: `structopt`'s only consumer is the un-gated `paladin` binary (ADR-0019). What is its fate?

| Option | Description | Selected |
|--------|-------------|----------|
| Migrate to clap v4 **and** add `required-features = ["cli"]` | Removes `structopt` entirely; makes all three bins consistent | ✓ |
| Gate only | `required-features` on the bin, keep `structopt` optional | |
| Migrate only | Swap to clap, leave the bin un-gated | |
| Retire `src/main.rs` | Delete the legacy content-aggregator entry point | |

**Choice:** Migrate and gate (D-13). ⚠ Flagged for human review — changes a shipped surface.
**Notes:** Gating alone leaves `structopt` — a crate whose upstream declares itself superseded by
clap 3+ — as an optional dependency nobody intends to keep. Migrating alone leaves `clap`
unconditional and just renames the leak. **Retirement was explicitly rejected:** ADR-0019 has just
documented that binary's purpose; retiring it one phase later is new scope, not defect closure.
**User-visible cost recorded:** after this change `cargo run` no longer builds the `paladin` binary
without `--features cli`. Must land in `CHANGELOG.md` and the ADR.

### Q2: `paladin-herald` re-introduces `colored` and `comfy-table` unconditionally. What now?

| Option | Description | Selected |
|--------|-------------|----------|
| Add a `[features]` section to `paladin-herald`, gate the two formatters | | ✓ |
| Accept and restate criterion 4 as root-manifest-scoped | Record `superseded by shipped code`, as Phase 7's ledger leaned | |
| Make `paladin-herald` an optional root dependency | | |

**Choice:** Feature-gate Herald (D-14). ⚠ Flagged for human review — shrinks a published crate's
default API.
**Notes:** Verified — `crates/paladin-herald/Cargo.toml:22-23` declares both unconditionally with no
`[features]` section at all, and Herald is an unconditional root dependency (`Cargo.toml:22,54`), so
root gating cannot reach them. Phase 7 specifics-3 read this as `superseded by shipped code` at the
ledger level, but ROADMAP criterion 4 names all three crates explicitly and is stricter — the
criterion governs. A documented fallback exists if the planner *proves* infeasibility, but it must be
proven, not assumed.

### Q3: One ADR or two?

| Option | Description | Selected |
|--------|-------------|----------|
| One — ADR-0023, "CLI dependency isolation and the binary/Herald surface" | | ✓ |
| Two separate ADRs | One per site | |

**Choice:** One (D-15).
**Notes:** One question — what a library-only consumer compiles — with two sites. The phase goal's
"no shipped surface is removed without a recorded decision behind it" is aimed precisely here.

### Q4: How is criterion 4 proved?

| Option | Description | Selected |
|--------|-------------|----------|
| Run `cargo tree`, capture the output verbatim | | ✓ |
| Read the manifests | Inspect `Cargo.toml` and reason | |

**Choice:** Run it (D-16).
**Notes:** The D-00e evidence bar. If the flag combination in the requirement text does not exist as
written, record the equivalent invocation used and why — do not silently substitute.

---

## DEBT-05 — one `TokenUsage`

### Q1: The battalion copy has `Default`, `PartialEq`, `new()` and `from_total()` the canonical one lacks. Where do they go?

| Option | Description | Selected |
|--------|-------------|----------|
| Move them onto the canonical type first, then re-export | Purely additive for existing consumers | ✓ |
| Drop them and fix call sites | Keep the canonical type minimal | |
| Keep the battalion copy as canonical | Invert ADR-0016 | |

**Choice:** Extend the canonical type (D-17).
**Notes:** Field sets are identical across all three definitions — verified — so consolidation is not
a pure deletion. Inverting ADR-0016 was not an option: it is a landed decision with `Code Conformance:
must change` naming this phase. Sequence is non-negotiable: extend → re-export battalion →
re-export llm → grep-verify.

### Q2: Re-export or delete-and-reimport?

| Option | Description | Selected |
|--------|-------------|----------|
| `pub use` re-export at both sites | Preserves every existing import path | ✓ |
| Delete duplicates, rewrite importers | Cleaner call graph | |

**Choice:** Re-export (D-18).
**Notes:** Both DEBT-05's done-condition and ADR-0016 say "re-exports". 179 `TokenUsage` references
exist across `src`, `crates`, `tests` and `examples`; a re-export closes the requirement in three
files without touching any of them. `paladin-ports/src/output/llm_port.rs:671` is a shipped instance
of the exact pattern.

### Q3: Is `VisionTokenUsage` in scope?

| Option | Description | Selected |
|--------|-------------|----------|
| No — record it as deferred | Different name, different purpose, not among ADR-0016's five types | ✓ |
| Yes — converge it too | One token-accounting type everywhere | |

**Choice:** Deferred (D-20).
**Notes:** DEBT-05's done-condition greps `pub struct TokenUsage` exactly. Convergence is a real
question for the vision surface (Phase 14 territory), but folding it in here is scope creep.

---

## Cross-cutting

### Q1: Evidence bar for a code phase?

**Choice:** Every closure claim proved by a command run in this environment and recorded verbatim;
prefer `--offline`; anything that genuinely cannot run gets a recorded blocker, never an inferred
pass (D-21).
**Notes:** Phase 7 established the tree builds and tests offline —
`cargo test --offline -p paladin-ports --lib` (98 passed) and
`cargo doc --offline -p paladin-ports --no-deps` under `-D warnings` (0 warnings).

### Q2: ADR allocation?

**Choice:** 0022 (deprecation withdrawal) and 0023 (CLI dependency isolation); `PROMOTION.md`
advances to 0024. No ADR for DEBT-01, DEBT-03 or DEBT-05 (D-22).
**Notes:** These are the first ADRs in this corpus whose executing phase is their own.

### Q3: Ledger handling?

**Choice:** Amend the five affected rows in place, dated, superseded text retained; recount the
verdict distribution rather than adjusting arithmetically (D-23).
**Notes:** The Phase 7 07-13 lesson — counts get recomputed by counting.

### Q4: Decomposition?

**Choice:** ~8 plans across 4 waves; DEBT-04's two manifest changes share one plan because they touch
the same file; ADRs land before the code they authorise (D-24).

---

## Claude's Discretion

- `[features]` names in `paladin-herald` (`pretty`/`table` vs `formatters` vs `styled`).
- Whether ADR-0022 and ADR-0023 are authored in their own plans or fold into the executing plans.
- The `clap` v4 idiom for `src/main.rs` — derive vs builder (derive is the closer analogue).
- Banner wording and inline-correction markup for the `.project/` annotations.
- Whether the DEBT-03 measurement spike publishes its failure list as an artefact or inline.
- Whether the regenerated 442 KB baseline is committed alongside the path fix or separately.

---

## Deferred Ideas

- `VisionTokenUsage` converging on the canonical `TokenUsage` — Phase 14 territory.
- Auditing the 87 pre-existing `ignore`/`no_run`/`text` fences in `paladin-ports` — Phase 16 / DOCS-03.
- Retiring or replacing `src/main.rs`, the legacy content-aggregator entry point — still open after
  D-13 gates and migrates it.
- The `#[structopt(name = "smartcontent-aggregator")]` product-name mismatch — a product decision the
  clap migration will have to touch.
- Which `cargo doc --workspace --no-deps` bar governs — Phase 10 / HARD-07.
- A `cargo tree`-based dependency-allowlist check in CI — Phase 15, from ADR-0015.
- The user-facing binary-architecture mdbook page — Phase 16, from ADR-0019.
- The eight deprecated GitHub Action references, including `ci.yml:148` inside the very job DEBT-01
  fixes — Phase 15 / PIPE-04. Explicitly not bumped opportunistically.
- Nyquist validation for Phases 1-4 — `/gsd-validate-phase 1`…`4`.
- Whether ADRs should be published to the mdbook for framework consumers — Phase 16.
