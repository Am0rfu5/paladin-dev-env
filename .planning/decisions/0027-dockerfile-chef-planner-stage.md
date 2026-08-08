# ADR-0027: Dockerfile.chef planner-stage supersession

## Status

Accepted

**Date:** 2026-08-08

## Context

`Milestone_7-Production-Hardening/Epic_2/prd-production-build-infra-adaptation.md` FR-01
(`:66-69`) requires the `Dockerfile.chef` planner stage to copy the root manifest and lockfile,
**all** `crates/*/Cargo.toml` files, and all source trees. The same PRD's §6 "Design
Considerations" (`:208-233`) states the purpose directly: "the dependency layer only invalidates
when a `Cargo.toml` changes" (`:37`), and its own recommended COPY ordering (`:212-231`) puts
`RUN cargo chef prepare` **before** the full-source `COPY src ./src` / `COPY crates ./crates`
lines, with an explicit note to "keep the dependency cache layer tight" (`:233`).

The shipped `Dockerfile.chef` satisfies FR-01's letter only partially and violates §6's own
ordering. Verified this session, `Dockerfile.chef:24-38` (pre-edit): the planner stage enumerated
**nine** crate manifests by name at `:25-33` — `paladin-core`, `paladin-ports`,
`paladin-battalion`, `paladin-llm`, `paladin-memory`, `paladin-storage`, `paladin-notifications`,
`paladin-content`, `paladin-web` — omitting `crates/paladin-herald/Cargo.toml`, because
`paladin-herald` was created after this milestone's FR-01 shipped (per the 2026-06-04
reconciliation record cited in `.planning/STATE.md`). REQUIREMENTS.md tracks this omission as
SEC-05, whose done-condition is explicit: "an enumerated list that goes stale on every crate
addition is the defect, not just the one missing line."

**The sharper finding, from this session's research (`09-RESEARCH.md` §1):** the enumeration could
never have delivered the stated per-manifest isolation for **any** of the nine crates it did name,
regardless of herald. `Dockerfile.chef:36` ran `COPY crates ./crates` — a full recursive copy of
every crate's manifest *and* `.rs` source — **before** `RUN cargo chef prepare` at `:38`, directly
inverting the ordering the PRD's own §6 example prescribes (recipe prepared first, full source
copied after, specifically "to keep the dependency cache layer tight"). Docker's classic
layer-cache model invalidates every subsequent instruction once an earlier one is invalidated, so
this coarser, strictly later copy already dominated whether `cargo chef prepare` re-ran, for all
ten crates, on every build where source under `crates/` changed at all. The nine-line enumeration
at `:25-33` was consequently inert dead weight, not a working-but-incomplete mechanism.

What actually delivers the cache-tightness benefit, and is untouched by this decision: `cargo chef
prepare` distils `recipe.json` from the manifests and lockfile only — no `.rs` content — and the
**builder** stage's `COPY --from=planner /app/recipe.json recipe.json` (`Dockerfile.chef:53`) is a
content-addressed cross-stage copy. BuildKit keys that copy on `recipe.json`'s bytes, independent
of whether the *planner* stage's own layers were a cache hit. `RUN cargo chef cook` in the builder
stage therefore only re-runs when a manifest or the lockfile actually changes — which is the
isolation FR-01 wanted, delivered by a mechanism the planner-stage enumeration never touched.

Confirmed from cargo-chef's own canonical documentation
(`[CITED: github.com/LukeMathWalker/cargo-chef README.md]`): the upstream reference Dockerfile
does not enumerate manifests at all — its planner stage is a single `COPY . .` of the whole
project — because `recipe.json`'s content already carries only the manifest/lockfile skeleton, and
the README states plainly that a full-tree `COPY . .` in the planner stage "will invalidate the
cache for the planner container, but it will not invalidate the cache for the builder container, as
long as the checksum of the `recipe.json` … does not change." A second source corroborates the
same reading for Rust-in-Docker builds generally
(`[CITED: depot.dev/docs/languages/rust-dockerfile]`).

**What is measured vs. established.** Docker is absent from this environment
(`.planning/phases/04-release-coherence/04-ci-gate-deferrals.md` records this standing constraint;
`command -v docker` fails here too). Nothing in this ADR was run against a real Docker daemon or
BuildKit — the reasoning above is established from cargo-chef's documented recipe semantics and
Docker's publicly documented layer-cache/content-addressed-copy behaviour, not from a build
observed in this session. The outstanding measurement that would upgrade this from
documentation-cited to verified — building the image twice with only a `.rs` source edit between
runs and confirming the builder stage's `cargo chef cook` layer reports `CACHED` on the second
build — cannot be performed here and is recorded below as CI-only.

## Decision

Under the D-00b precedence order (ADR → shipped tree → `.planning/codebase/` map →
`intel/code-verification.md` → PRD → DOC → checkbox — an ADR that contradicts shipped code is an
instruction to change the code, and a PRD requirement contradicted by the tool's own semantics is
superseded), **M7 Epic 2 FR-01's enumeration requirement is superseded.** The nine per-crate
manifest `COPY` lines at `Dockerfile.chef:25-33` are **deleted**, not extended to ten.

This is **D-16's primary branch, not its fallback.** D-16 named two branches going into this
phase's research: delete the enumeration if cargo-chef's own semantics confirmed it was
decorative, or keep the enumeration and add a guard script asserting every `crates/*/Cargo.toml`
appears in the planner stage if that reading was refuted. The researcher **confirmed** the
cargo-chef reading from its own canonical documentation rather than refuting it, so the primary
branch is taken.

The four surviving planner-stage instructions (`COPY Cargo.toml Cargo.lock ./`, `COPY src ./src`,
`COPY crates ./crates`, `COPY benches ./benches`) and the `RUN cargo chef prepare` step keep their
existing relative order — the fix is subtraction only, no instruction is added, reordered, or
consolidated into the upstream `COPY . .` form. The explanatory comment at `Dockerfile.chef:21-23`
is rewritten to describe the mechanism that actually operates (recipe.json's manifest-only content
plus the builder stage's cross-stage content-addressed copy) rather than the per-manifest isolation
the deleted lines never delivered.

## Considered Options

- **Add `crates/paladin-herald/Cargo.toml` as a tenth enumerated line** (rejected) — satisfies
  FR-01's letter and SEC-05's literal ask, but reproduces exactly the defect SEC-05's done-condition
  names: "an enumerated list that goes stale on every crate addition is the defect, not just the
  one missing line." The next new crate would reintroduce the same gap.
- **Keep the nine/ten-line enumeration and add a guard script asserting every
  `crates/*/Cargo.toml` appears in the planner stage** (rejected) — the genuinely considered D-16
  fallback, not an afterthought: this would also close SEC-05 structurally, by making omission
  mechanically detectable rather than eliminating the list. It would have been the correct choice
  **had** the cargo-chef reading been refuted (i.e., had the enumeration turned out to deliver real
  isolation this project depends on). Rejected only because the primary branch's precondition —
  confirmation, not refutation — held.
- **Adopt cargo-chef's upstream whole-tree planner form verbatim (`COPY . .`)** (rejected) — the
  surviving four-instruction form already achieves identical caching behaviour (per Context above)
  while touching fewer lines and preserving this Dockerfile's existing separation of `src`,
  `crates`, and `benches` copies; adopting the literal upstream form would be a larger diff for no
  behavioural gain.
- **Delete the nine-line enumeration, keep the four surviving instructions and their order,
  rewrite the comment** (accepted) — eliminates the defect structurally (crate count is no longer
  encoded anywhere in the planner stage, so an eleventh crate needs no edit here), is equivalent in
  caching behaviour to the enumeration per the dominance analysis above, and is the smallest change
  that satisfies SEC-05's done-condition rather than its letter.

## Code Locations

- `Dockerfile.chef:21-27` (post-edit) — the planner stage's rewritten six-line comment and the
  surviving `COPY Cargo.toml Cargo.lock ./`; the deleted range was `:25-33` (nine `COPY
  crates/paladin-*/Cargo.toml …` lines) in the pre-edit file.
- `Dockerfile.chef:28-31` (post-edit) — the three surviving directory copies, in their original
  relative order: `COPY src ./src`, `COPY crates ./crates`, `COPY benches ./benches`.
- `Dockerfile.chef:32` (post-edit) — `RUN cargo chef prepare --recipe-path recipe.json`, untouched.
- `Dockerfile.chef:53` (unaffected by this decision, pre- and post-edit line number unchanged) —
  `COPY --from=planner /app/recipe.json recipe.json` in the builder stage, the content-addressed
  cross-stage copy that is the actual cache-tightness mechanism.
- `Dockerfile.chef:55` (unaffected) — `RUN cargo chef cook --release --workspace --recipe-path
  recipe.json`, the instruction whose cache-hit behaviour this decision reasons about but does not
  change.
- `Dockerfile.chef:~84` (post-edit; was `:93` pre-edit, shifted up nine lines by this deletion) —
  `LABEL org.opencontainers.image.licenses="MIT"`, confirmed byte-unchanged by this plan; owned by
  plan 09-05 (SEC-02) in wave 2, which must re-derive this line number rather than trust the one
  recorded in `09-RESEARCH.md` §4 (which misattributed the label to the runtime `Dockerfile`).
- `Milestone_7-Production-Hardening/Epic_2/prd-production-build-infra-adaptation.md:66-69` — FR-01,
  the superseded requirement.
- `Milestone_7-Production-Hardening/Epic_2/prd-production-build-infra-adaptation.md:37,208-233` —
  §6's stated purpose and its own recommended COPY ordering, which the shipped file inverted (full
  source copy before `cargo chef prepare`, not after).
- **Upstream source 1:** `[CITED: github.com/LukeMathWalker/cargo-chef README.md]` — the canonical
  planner-stage pattern (`COPY . .`, no manifest enumeration) and the stated cache-invalidation
  behaviour of that pattern.
- **Upstream source 2:** `[CITED: depot.dev/docs/languages/rust-dockerfile]` — corroborating
  third-party documentation of the same Rust-in-Docker cargo-chef caching pattern.
- `.planning/phases/04-release-coherence/04-ci-gate-deferrals.md` — the standing record that Docker
  is absent from this environment, cited here rather than re-asserted without evidence.

**Outstanding, CI-only measurement (not performed here):** build `Dockerfile.chef` twice with
`DOCKER_BUILDKIT=1 docker build .`, touching only a `.rs` file between the two builds, and confirm
the builder stage's `cargo chef cook` step reports `CACHED` in the second build's output. This
would upgrade the caching claim above from documentation-established to measured; it requires a
Docker daemon this environment does not have.

## Code Conformance

must change

Plan 09-03 task 1 is the executor: it performs the deletion at `Dockerfile.chef:25-33`, rewrites
the `:21-23` comment, and confirms `Dockerfile` and `Dockerfile.server` carry no equivalent
per-crate enumeration (neither did before this decision, and neither does after — this decision
touches only `Dockerfile.chef`).

## Downstream Consumers

- **Phase 10 / HARD-01** — the Milestone 7-8 as-shipped ledger's `REQ-docker-workspace-build` row
  currently reads "Shipped, defect → SEC-05"; this ADR is the record HARD-01 cites when it upgrades
  that row to a per-criterion verdict, and the row's disposition becomes "Superseded by outcome"
  for FR-01's enumeration clause specifically, not "Shipped" or "Verify".
- **The first CI runner to build this image** — whichever workflow first runs `docker build -f
  Dockerfile.chef .` after this decision lands is the first opportunity to perform the outstanding
  CI-only measurement recorded above (two builds, one `.rs`-only edit between them, confirm
  `cargo chef cook` reports `CACHED`) and thereby upgrade this ADR's evidence from
  documentation-established to measured.
