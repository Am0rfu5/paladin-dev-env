# ADR-0032: PDF extraction is unconditional; the inert `pdf` feature and the -0187 reachability path

## Status

Accepted

**Date:** 2026-08-08

## Context

HARD-06 lists three facts pointing in two directions, each re-read this session at its current
`file:line`:

- `crates/paladin-content/Cargo.toml:18` — `pdf = []`, an empty feature declaration that gates
  nothing.
- `crates/paladin-content/Cargo.toml:41` — `pdf-extract = { version = "0.7" }`, declared with **no
  `optional = true`** — an unconditional dependency of every build of the crate.
- Root `Cargo.toml:275` — the facade's `content-processing` feature list enables
  `paladin-content/web-scraping`, `paladin-content/rss`, `paladin-content/news-api`,
  `paladin-content/tiktoken`, and `paladin-content/llm` — five of the crate's six declared
  features. `pdf` is the one name missing from that list.

Read at face value those three facts point in two directions: an empty feature suggests PDF
support is either unimplemented or feature-gated, and the facade's list omitting `pdf` looks like a
capability gap in the published surface.

**The fourth fact, source-level rather than manifest-level, settles it.** A workspace search this
session:

```
$ grep -rn 'cfg(feature = "pdf")' crates/paladin-content/src/
```

returns **zero matches**. `crates/paladin-content/src/adapters/document/mod.rs:1-2` declares `pub
mod pdf_extractor;` and `pub use pdf_extractor::PdfExtractor;`, both unconditional — no `#[cfg]`
attribute of any kind. `crates/paladin-content/src/adapters/document/document_adapter.rs:22` holds
`pdf_extractor: PdfExtractor` as a plain, ungated struct field on `DocumentAdapter`; `:29`
constructs it unconditionally in `DocumentAdapter::new()`; `:123` and `:132` are its two call
sites, `self.pdf_extractor.extract(&path)` and `self.pdf_extractor.extract_bytes(&data)`, reached
whenever `DocumentFormat::Pdf` is matched, again with no feature gate on either arm.

The comparator that shows the shape is not the defect: `crates/paladin-content/Cargo.toml:21` —
`news-api = []` — is a **byte-identical** empty feature declaration, and it legitimately gates real
code: `crates/paladin-content/src/adapters/input/mod.rs:5` reads `#[cfg(feature = "news-api")]`
immediately above `pub mod news_api_fetcher;`. An empty `= []` value is not itself the defect —
`news-api` proves an empty feature can validly gate a dependency-free module purely through source
`#[cfg]` attributes, no `Cargo.toml` dependency line required. `pdf` is the only feature in the
crate that is inert in **both** directions: it gates no dependency (`pdf-extract` has no
`optional = true`) and no code (zero `#[cfg(feature = "pdf")]` sites exist).

**Conclusion:** PDF extraction ships, always, in every build of `paladin-content`. The facade's
five-of-six `content-processing` list is not a capability gap — the sixth feature name gates
nothing, so its absence from the list changes no build's behavior. `pdf` is the only feature in
this crate that is inert in both directions.

**The advisory half.** `.cargo/audit.toml:26-29`'s `RUSTSEC-2026-0187` entry is **right about
reachability and wrong about the mechanism**. Its current comment reads:

```
# RUSTSEC-2026-0187: stack overflow in lopdf via deeply nested PDF objects.
#   lopdf is transitive via `pdf-extract` (optional `content-processing`). The fix requires
#   `pdf-extract` >= 0.12 (a breaking jump that also pulls a fresh `ttf-parser` advisory);
#   deferred. Revisit when `pdf-extract` ships lopdf >= 0.42 without new advisories.
```

The parenthetical `(optional content-processing)` attributes `pdf-extract`'s optionality to the
facade's `content-processing` feature — but `content-processing` does not gate `pdf-extract` at
all; it gates `paladin-content` itself (`Cargo.toml:59` — `paladin-content = { workspace = true,
optional = true }`), and once `paladin-content` is in the build, `pdf-extract` is unconditional
inside it, per this session's Cargo.toml read above. The actual optionality lives one level up
from where the comment says it does. The suppression stands — `SECURITY-EXCEPTIONS.md`'s
`RUSTSEC-2026-0187` row (`why_present`, `path = "lopdf -> pdf-extract -> paladin-content"`) and
ADR-0024's governance are untouched by this ADR — and only the comment's stated reasoning is
corrected.

**What is measured vs. established.** `cargo audit` and `cargo deny check` cannot be run in this
environment — `crates.io` returns HTTP 403, unchanged from Phase 9
(`.planning/phases/04-release-coherence/04-ci-gate-deferrals.md`). Nothing in this ADR was measured
against either tool; the reachability conclusion above rests entirely on the direct `grep`/file
reads recorded here, not on a tool run. The outstanding CI-only verification is: `cargo audit
--config .cargo/audit.toml` and `cargo deny check advisories`, both expected to pass unchanged
(the suppression list is not modified by this decision), run by the first CI job with network
access to crates.io after this ADR lands.

**Adjacent finding, not fixed by this ADR.** Three further optional dependencies in the same
manifest — `scraper` (behind `web-scraping = ["dep:scraper"]`), `rss` (behind `rss =
["dep:rss"]`), and `tiktoken-rs` (behind `tiktoken = ["dep:tiktoken-rs"]`) — are declared and
gated correctly at the manifest level but consumed by no code in the crate. Confirmed this
session:

```
$ grep -rn "scraper::\|tiktoken_rs\|::rss::" crates/paladin-content/src/
```

returns zero matches, and none of `web-scraping`, `rss`, or `tiktoken` appears in any `#[cfg]`
attribute anywhere under `crates/paladin-content/src/`. This is the mirror image of this ADR's
defect: where `pdf` is a feature that gates nothing, these three are features that gate a
dependency correctly but the dependency itself has no first-party consumer left in the crate. It
is outside HARD-06's scope (HARD-06 is specifically the `pdf`/`pdf-extract` contradiction) and is
recorded here as a deferred finding rather than fixed in this ground-truth phase — candidate for
Phase 11's facade residue work or a Phase 15 dependency-hygiene item.

## Decision

Under the D-00b precedence order (ADR → shipped tree → `.planning/codebase/` map →
`intel/code-verification.md` → PRD → DOC → checkbox — an ADR that contradicts shipped code is an
instruction to change the code), the one-sentence answer: **PDF extraction is a supported
capability of `paladin-content`, unconditionally, in every build.**

**(i) The disposition of the inert feature.** `10-04-SUMMARY.md`'s recorded checkpoint answer to
Question 2 selected **`q2-delete`**: delete the `pdf = []` line from
`crates/paladin-content/Cargo.toml:18`. The reasoning the human's selection carries, quoted: "q2-delete
was chosen because the feature gates nothing in either direction while `news-api = []` in the same
manifest legitimately gates a dependency-free module, proving an empty feature is not itself the
defect." **The accepted cost:** `cargo build -p paladin-content --features pdf` begins to fail
where it previously succeeded-and-did-nothing — a minor public-contract change on a pre-1.0 crate
family, recorded in `crates/paladin-content/CHANGELOG.md` by this same plan. The alternative branch,
`q2-keep` (keep `pdf = []`, add it to the facade's `content-processing` list for literal §4.4.6
compliance, record it in this ADR as a documentation marker with no gating effect), lost because it
leaves a manifest that lies quietly — the exact defect class this milestone close-out exists to
retire. `q2-wire` (cfg-gate the struct field, constructor, and two call sites) was not selected;
see Considered Options below for why it was never a live option for this plan.

**(ii) The corrected reachability path, stated positively.** `pdf-extract` is an unconditional
dependency of `paladin-content` — nothing inside that crate gates it. Reachability is determined
one level up: whether the facade's optional `paladin-content` dependency is enabled
(`Cargo.toml:59`). This is the sentence `.cargo/audit.toml`'s `RUSTSEC-2026-0187` comment now
carries, and it is the sentence `SECURITY-EXCEPTIONS.md`'s compensating-control row for that
advisory rests on.

## Considered Options

- **Delete the inert `pdf = []` declaration** (accepted) — the manifest stops declaring a feature
  that gates nothing; `--features pdf` begins failing where it previously silently succeeded, an
  accepted, recorded, minor cost on a pre-1.0 crate.
- **Keep `pdf = []` and add it to the facade's `content-processing` list, recording it as a
  documentation marker with no gating effect** (rejected) — achieves literal §4.4.6 compliance but
  leaves the manifest lying quietly: a reader sees `pdf` in the feature list and reasonably infers
  it controls something, when it does not and never will under this branch.
- **Wire `pdf` to actually gate `pdf-extract` and the `PdfExtractor` field/constructor/call sites**
  (rejected) — not a no-op. `DocumentAdapter` holds `PdfExtractor` as an ungated struct field;
  making it optional requires `cfg`-gating that field, its constructor, and both call sites — a
  `.rs` change outside this ground-truth phase's D-23 boundary — and it would turn PDF extraction
  into an **opt-out** capability for every existing consumer of the published `paladin-content
  0.1.0`+, a behavior change far larger than a manifest correction.
- **Record the answer and change nothing** (rejected) — leaves a manifest that lies quietly, which
  is the defect class this whole milestone close-out exists to retire; recording an inert feature
  as "known and fine" without removing it does not make the published `cargo build --features pdf`
  invocation stop silently doing nothing.

## Code Locations

- `crates/paladin-content/Cargo.toml:18` (pre-edit) — `pdf = []`, the inert feature declaration
  deleted by this decision.
- `crates/paladin-content/Cargo.toml:21` — `news-api = []`, the byte-identical comparator that
  legitimately gates code and is left untouched.
- `crates/paladin-content/Cargo.toml:41` — `pdf-extract = { version = "0.7" }`, unconditional, no
  `optional = true`.
- `crates/paladin-content/src/adapters/document/mod.rs:1-2` — `pub mod pdf_extractor;` and `pub use
  pdf_extractor::PdfExtractor;`, both unconditional.
- `crates/paladin-content/src/adapters/document/document_adapter.rs:22` — `pdf_extractor:
  PdfExtractor`, the ungated struct field.
- `crates/paladin-content/src/adapters/document/document_adapter.rs:29` — construction in
  `DocumentAdapter::new()`.
- `crates/paladin-content/src/adapters/document/document_adapter.rs:123,132` — the two call sites,
  `self.pdf_extractor.extract(&path)` and `self.pdf_extractor.extract_bytes(&data)`.
- `crates/paladin-content/src/adapters/input/mod.rs:5` — `#[cfg(feature = "news-api")]`, the proof
  an empty feature can legitimately gate code.
- Root `Cargo.toml:59` — `paladin-content = { workspace = true, optional = true }`.
- Root `Cargo.toml:275` — `content-processing = ["dep:paladin-content",
  "paladin-content/web-scraping", "paladin-content/rss", "paladin-content/news-api",
  "paladin-content/tiktoken", "paladin-content/llm", "paladin-memory/content-processing"]` — omits
  `pdf`, harmlessly.
- `.cargo/audit.toml:26-29` — the `RUSTSEC-2026-0187` comment block, corrected by this decision; the
  suppression's `ignore` entry at `:38` is unchanged.
- `SECURITY-EXCEPTIONS.md:79-85` — the `RUSTSEC-2026-0187` register row whose `why_present` field
  ("Transitive dependency of pdf-extract, which crates/paladin-content/Cargo.toml:41 declares as an
  unconditional (non-optional) dependency") already states the corrected path; this ADR is the
  citable record the `.cargo/audit.toml` comment now points back to.
- `.project/Milestone_7-Production-Hardening/Epic_1/prd-extract-infrastructure-crates.md:151` —
  §4.4.1, requiring `pdf` to gate `pdf-extract`; superseded by outcome, annotated by plan 10-05
  task 3.
- `.project/Milestone_7-Production-Hardening/Epic_1/prd-extract-infrastructure-crates.md:184` —
  §4.4.6, requiring "all capability features enabled"; superseded by outcome, annotated by plan
  10-05 task 3. (Both line numbers re-derived this session after plan 10-04's insertions moved
  them from their original `132`/`165`.)

## Code Conformance

must change

Plan 10-05 task 2 is the executor for the manifest deletion, the `CHANGELOG.md` entry, and the
`.cargo/audit.toml` comment correction. Plan 10-05 task 3 is the executor for the M7 Epic 1 PRD
§4.4.1 and §4.4.6 annotations.

## Downstream Consumers

- **Phase 12 / SUPPLY-02 and SUPPLY-03** — the `pdf-extract` reachability question, delivered as an
  answer rather than deferred: `pdf-extract` is unconditional in `paladin-content`, reachability is
  gated one level up by the facade's optional `paladin-content` dependency, and `RUSTSEC-2026-0187`
  is warranted regardless of the `pdf` feature's disposition. Phase 12 does not re-derive this.
- **Phase 10 / HARD-01** — the ledger row for `REQ-content-processing-build-gate`, written by plan
  10-10, cites this ADR to resolve the `pdf`/`pdf-extract` contradiction its evidence previously
  only flagged.
- **Phase 11 or Phase 15** — the three consumed-by-nothing optional dependencies (`scraper`, `rss`,
  `tiktoken-rs`), recorded here as an adjacent finding with no owner assigned yet by this phase;
  candidate for Phase 11's facade residue work or a Phase 15 dependency-hygiene item.
