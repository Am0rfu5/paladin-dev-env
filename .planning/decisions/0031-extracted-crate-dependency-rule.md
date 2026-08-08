# ADR-0031: The extracted-crate dependency rule as a default-build invariant

## Status

Accepted

**Date:** 2026-08-08

## Context

The measured fact this ADR is written against: **the default build of `paladin-content` carries
no leaf-to-leaf edge at all.** Four citations, each re-read this session:

- `crates/paladin-content/Cargo.toml:23` — `llm = ["dep:paladin-llm"]`. This feature is **not** a
  member of any default feature set; `cargo tree -p paladin-content --no-default-features`,
  attempted and recorded below, shows no `paladin-llm` node in the resulting tree.
- `crates/paladin-content/Cargo.toml:28` — `paladin-llm = { version = "0.7.0", path =
  "../paladin-llm", optional = true }`. The dependency is `optional = true`, so it is absent
  unless the `llm` feature is enabled. Note the version pin reads **`0.7.0`** today, not the
  `0.6.0` that `.planning/REQUIREMENTS.md:1438` and `10-CONTEXT.md` D-15's text quote — Phase 4
  plan 04-05 (commit `c2e20a1`) converged every manifest and internal pin on `0.7.0`, after those
  two documents' text was written.
- `crates/paladin-content/src/services/mod.rs:7` — `#[cfg(feature = "llm")]` gates exactly one
  module, `content_llm_analysis_service`. `crates/paladin-content/src/services/content_llm_analysis_service.rs:8`
  is that module's only `paladin_llm` consumer in the crate — `use
  paladin_llm::llm_analysis_service::{LlmAnalysisConfig, LlmAnalysisInput, LlmAnalysisService};`.
- Root `Cargo.toml:275` — the facade's `content-processing` feature list includes
  `"paladin-content/llm"` explicitly, alongside `web-scraping`, `rss`, `news-api` and `tiktoken`.
  The opt-in that activates the edge is therefore explicit and lives one level up, in the facade,
  not inside `paladin-content` itself.

Only against that measured baseline does the PRD's absolute form matter. M7 Epic 1 PRD Goal 2
(`.project/Milestone_7-Production-Hardening/Epic_1/prd-extract-infrastructure-crates.md:30`)
states: "Each new crate must depend only on `paladin-core`, `paladin-ports`, and workspace-shared
dependencies — never on other new infrastructure crates or on the facade." The same PRD's §6.1
(`:246`) states the general rule the same way: "No extracted crate may depend on another extracted
crate or on the `paladin` facade." Neither clause carries a condition.

The PRD's own sibling document anticipated this exact case without amending either clause.
`.project/Milestone_7-Production-Hardening/Epic_1/cost-benefit-assessment.md:118`, the
`paladin-content` candidate's Extraction complexity cell, reads in full: "**HIGH** — this
extraction is the most complex because: (a) it includes application-layer use-case services (13
services in `use_cases/content/`), not just infrastructure adapters; (b) use-case services depend
on `paladin-llm` for LLM analysis, creating an inter-crate dependency that must be handled
carefully; (c) `tensorflow_adapter.rs` must remain in the facade and be excluded explicitly; (d)
the 5 feature sub-flags (`pdf`, `web-scraping`, `rss`, `news-api`, `tiktoken`) require careful
conditional compilation across a large file tree." Clause (b) names the exact edge this ADR
restates a rule around, at the planning stage, before a single line of `paladin-content` existed.

**Both `10-CONTEXT.md` D-15 and `.planning/REQUIREMENTS.md:1438` attribute that sentence to "the
same PRD's §4.4."** Re-read directly this session: `prd-extract-infrastructure-crates.md` has no
§4.4 containing this sentence — its own §4.4 numbering runs through cost-benefit gating
instructions, not the complexity table itself, and the complexity table lives in the sibling
`cost-benefit-assessment.md` document, at line 118, cited above. That citation does not resolve
and is not reproduced here; `cost-benefit-assessment.md:118` is the correct one, and every
downstream reader of this ADR should use it rather than the stale §4.4 reference.

## Decision

Under the D-00b precedence order — ADR → shipped tree → `.planning/codebase/` map →
`intel/code-verification.md` → PRD → DOC → checkbox; an ADR that contradicts shipped code is an
instruction to change the code — this ADR restates the rule rather than instructing a code change,
because the tree already conforms to the restatement below.

**The restated rule, as a single quotable sentence:** *no extracted crate may depend on another
extracted crate or on the facade in its default build; a non-default optional feature may declare
such an edge only where the facade opts in explicitly and the dependent code is `cfg`-gated.*

Three sub-decisions, in ADR-0015's shape, kept visually separate:

**(i) The enforceable invariant**, stated independent of any specific crate or fact list: an
extracted crate's `cargo tree --no-default-features` output must show no other extracted crate and
no facade package. This is checkable mechanically, per crate, on every build — the same mechanism
`.planning/decisions/0015-core-ports-dependency-allowlist.md` already names for the
`paladin-core`/`paladin-ports` allowlist, and Phase 15 is the queued owner of building the check
for both.

**(ii) The measured current state, accepted as the baseline**: `paladin-content`'s `llm` feature
and its single `cfg`-gated module (`content_llm_analysis_service`) satisfy the invariant today,
because the feature is non-default and the facade's opt-in is explicit and lives one level up, in
`content-processing`. No code change is required.

**(iii) The anchor moved deliberately.** This is recorded in its own sub-decision so a later reader
sees a promotion rather than an exception bolted onto an unchanged rule: the enforceable invariant
is **promoted** from "no leaf-to-leaf edge at all" to "no leaf-to-leaf edge in the default build."
The absolute form is not abandoned — it becomes the special case of a crate that declares no
non-default features at all, and is recoverable from the general form by observing that a crate
with zero non-default features has, trivially, no non-default-feature edge to declare. The general
form was chosen as primary because it is the one a command (`cargo tree --no-default-features`)
can check directly; the absolute form alone gives no way to distinguish a compliant optional edge
from a violating default one.

**An ADR is the promotion.** HARD-05's own text proposes re-tagging
`prd-extract-infrastructure-crates.md` via `--manifest` and re-running the ingest classifier to
promote this restatement above PRD precedence. That is not done here. Phase 7's D-11
(`07-CONTEXT.md`) settled this pattern for the identical situation with ADR-0016: an ADR restates
the position inside `.planning/decisions/`, which sits at the top of the precedence order by
construction, and cites the PRD as provenance, rather than re-typing a `.project/` document.
`.planning/STATE.md:399,731` records "there is no run 6" — the ingest that would process a
re-tagged manifest is closed, and `.planning/decisions/PROMOTION.md`'s own "Why this is viable
now" note confirms promotion no longer requires it: ADRs are their own document class,
independent of the ingest manifest.

## Considered Options

- **Restate the rule as a default-build invariant, with the `cfg`-gate and explicit-facade-opt-in
  conditions stated tightly** (accepted) — the invariant that has teeth is the default-build one;
  it is checkable by `cargo tree --no-default-features`, matches ADR-0015's structural model
  exactly, and describes what the tree does rather than excusing a violation. Milestone 8's
  reconciliation kept this exact edge while deleting ~10,250 net LOC of everything else — it was
  looked at and left, not overlooked.
- **Keep the absolute "never" and remove the `paladin-content` → `paladin-llm` edge** (rejected) —
  not a record change but architecture work: it requires either deleting a shipped,
  facade-exposed capability (`content-processing`'s LLM-analysis service) or inverting the
  dependency through a port, either of which is real design and real `.rs` churn, outside a
  ground-truth phase's D-23 boundary. Recorded here as deferred to its own phase should a human
  later overturn this ADR's answer.
- **Re-tag `prd-extract-infrastructure-crates.md` via `--manifest` and re-run ingest to promote it
  above PRD precedence** (rejected) — no ingest run 6 exists (`.planning/STATE.md:399,731`), and
  re-typing a completed `.project/` document changes how five already-completed ingest runs
  classified the corpus, for an outcome an ADR achieves natively per Phase 7's D-11 precedent.
- **Leave the rule stated in two contradictory forms** (rejected) — HARD-05 exists precisely
  because the PRD states "never" absolutely while the PRD's own cost-benefit sibling anticipated
  and accepted the violation in the same breath; Phase 11's FACADE-02 cannot plan D2/D3/D4's
  leaf-to-leaf relocation targets against an invariant that is ambiguous about whether it permits
  what the tree already does.

## Code Locations

- `crates/paladin-content/Cargo.toml:23` — `llm = ["dep:paladin-llm"]`, non-default.
- `crates/paladin-content/Cargo.toml:28` — `paladin-llm = { version = "0.7.0", path =
  "../paladin-llm", optional = true }`. Version pin re-verified this session as `0.7.0`.
- `crates/paladin-content/src/services/mod.rs:7` — `#[cfg(feature = "llm")]`.
- `crates/paladin-content/src/services/content_llm_analysis_service.rs:8` — the single
  `paladin_llm` consumer this feature gates.
- Root `Cargo.toml:275` — the facade's `content-processing` feature, whose list includes
  `"paladin-content/llm"` explicitly.
- `.project/Milestone_7-Production-Hardening/Epic_1/prd-extract-infrastructure-crates.md:30` — Goal
  2, the absolute form.
- `.project/Milestone_7-Production-Hardening/Epic_1/prd-extract-infrastructure-crates.md:246` —
  §6.1, the absolute form.
- `.project/Milestone_7-Production-Hardening/Epic_1/cost-benefit-assessment.md:118` — the
  Extraction complexity cell anticipating the edge. **This is the correct citation for the
  `paladin-llm` complexity note; `prd-extract-infrastructure-crates.md`'s own §4.4 does not
  contain it.**
- `.planning/decisions/0015-core-ports-dependency-allowlist.md` — the structural model this ADR
  imitates: an enforceable invariant stated independent of any crate-count fact list, plus the
  measured current state accepted as baseline.
- `.planning/REQUIREMENTS.md:1438` — HARD-05's text, carrying both the stale `0.6.0` version quote
  and the `§4.4` mis-citation this ADR corrects; plan 10-01 corrects the line in place.

Attempted this session: `cargo tree -p paladin-content --no-default-features`. It resolved
offline against the vendored/cached workspace lockfile and produced a full dependency tree with
**zero** occurrences of `paladin-llm`, `paladin-web`, `paladin-storage`, `paladin-notifications`,
`paladin-battalion`, or the facade package `paladin-ai` — confirmed by `grep -n
'paladin-llm\|paladin-web\|paladin-storage\|paladin-notifications\|paladin-battalion\|paladin-ai '`
against the captured output, which returned no matches. The invariant this ADR states holds today,
measured directly, not inferred.

## Code Conformance

conforms

The tree already satisfies the restated invariant: `paladin-content`'s only leaf-to-leaf edge
(`paladin-llm`) is behind a non-default optional feature, `cfg`-gated at the single consuming
module, and activated only by the facade's explicit opt-in. This ADR instructs no code change.

## Downstream Consumers

- **Phase 11 / FACADE-02** — D2/D3/D4's leaf-to-leaf relocation targets are chosen against this
  restated invariant; a target that declares a leaf-to-leaf edge is legal only under the two
  stated conditions (non-default optional feature, explicit facade opt-in with `cfg`-gated
  dependent code), not under the PRD's original unconditional "never."
- **Phase 15** — the `cargo tree --no-default-features` check joins the `cargo tree`-based
  dependency-allowlist check ADR-0015 already queued there, as a second clause of the same
  mechanism rather than a separate check.
- **Phase 10 / HARD-01** — the Milestone 7-8 ledger row for `REQ-extracted-crate-dependency-rule`,
  currently reading `Code diverges → HARD-05` (`.planning/REQUIREMENTS.md:3159`), is upgraded by
  plan 10-07 to `satisfied`, citing this ADR. The verdict flip is explained, not bare: the
  divergence was always in the rule's **wording**, restated here against the default build, and
  never in the code — the tree has satisfied this invariant since the extraction shipped.
