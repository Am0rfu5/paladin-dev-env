# ADR-0009: Workspace Rust edition is 2024

## Status

Accepted

**Date:** 2026-08-03

## Context

Ten of twelve edition-carrying manifests (the root `paladin-ai` package plus nine member crates)
already declared `edition = "2024"` before this phase began; exactly two, `crates/paladin-ports`
and `crates/paladin-notifications`, declared `edition = "2021"`. `rust-toolchain.toml` pins
`channel = "1.97.1"`, and Rust 2024 has been stable since Rust 1.85 — the pinned toolchain is
twelve minor releases past that stabilization point.

Neither the code nor the record was self-consistent before this phase closed the split. On the
record side: Milestone 5 Epics 1-4 require `edition = "2021"` while Epic 5 and the Milestone 5
overview require `edition = "2024"` — the same milestone disagreeing with itself across its own
epics. **These documents are amended by this decision, not merely overridden**: the corpus's
precedence order (ADR → shipped tree → `.planning/codebase/` map → PRD → DOC) means a later ADR
settling the question is recorded as the authoritative answer, and the conflicting Milestone 5
documents are left in place with their disagreement stated rather than silently discarded.

On the code side: `.planning/codebase/CONCERNS.md`'s "Edition 2024 in Project Manifests" finding
(lines 7-25) claimed `edition = "2024"` "does not exist in Rust's stable channel. Rust only defines
editions 2015, 2018, and 2021," and recommended standardizing on 2021. **That claim is factually
wrong at the pinned toolchain** — Rust 2024 stabilized in Rust 1.85, and this workspace's pinned
`1.97.1` toolchain builds the ten already-2024 manifests today without any "lenient parsing"
hypothesis needed. Task 2 of this plan corrects that finding at source with dated provenance
(`CONCERNS.md` §"Edition 2024 in Project Manifests"); this ADR is the authority Task 2's correction
cites.

`ARCH-03(a)` (Phase 7) is the requirement that would otherwise adjudicate this question — "Each of
the four run-3 competing variant pairs has exactly one recorded answer... (a) Rust edition (group
17) — the answer feeds REL-02, which is the code fix." REL-02's own requirement text states:
"Whichever of Phase 4 / Phase 7 executes first records the answer, the other applies it." Phase 4
runs first, so Phase 4 records the edition answer here rather than waiting for Phase 7 — the
mirror-image convention of ADR-0008's version decision, which also runs first in Phase 4.

## Decision

**The workspace Rust edition is `2024`, uniformly.** The two stragglers — `crates/paladin-ports`
and `crates/paladin-notifications` — were bumped from `edition = "2021"` to `edition = "2024"` by
plan 04-01, closing the split in the same direction ten of twelve manifests already used.

The reasoning is a direct comparison of blast radius: bumping the two remaining manifests forward
is a two-line change against a 38-file compiled surface (`cargo fix --edition` produced zero source
rewrites for either crate — the two mechanically-detectable hazard classes,
`unsafe_op_in_unsafe_fn`/`static_mut_refs`, and the `gen`-keyword collision, were verified absent).
Moving the other nine crates and the root package **backward** to 2021 would instead be a
workspace-wide regression that would also have to reconcile any 2024-only syntax those nine crates
may already contain, for no compensating benefit — the toolchain already supports 2024, and there
is no forward-compatibility reason to retreat from it.

Both required build-proof legs pass on the unified tree: `cargo build --workspace --offline` and
`cargo build --workspace --no-default-features --offline` both exit 0 (plan 04-01's measurement,
`04-release-measurement.md`). The `--no-default-features` leg is separately noted as a structural
no-op for the root `paladin-ai` package's own feature resolution (traced to
`crates/doc-examples/Cargo.toml:15`'s `paladin-ai` dependency declaration) — a pre-existing
workspace feature-graph fact unrelated to the edition bump, which does not weaken this ADR's proof
because neither `paladin-ports` nor `paladin-notifications` declares a `default` feature of its
own.

## Considered Options

- **`2024`** (chosen) — two manifests forward against a 38-file surface; the toolchain already
  supports it; ten of twelve manifests already used it, so this closes the split in the direction
  requiring the smallest, best-supported change.
- **`2021`** — rejected. This is what `.planning/codebase/CONCERNS.md:25` recommended, but that
  recommendation rests on a factually incorrect premise (that Rust 2024 "does not exist in Rust's
  stable channel"), corrected at source by this phase's Task 2. Moving nine crates plus the root
  package backward would also require reconciling any 2024-only syntax already present in those
  nine crates, for no forward-compatibility benefit.
- **Leave the split in place** — rejected. SC2 (Phase 4's roadmap success criterion 2) forbids an
  unresolved per-crate edition split; `cargo build --workspace` must succeed under one consistent,
  valid edition across every manifest, not a mix.

## Code Locations

- `crates/paladin-ports/Cargo.toml` and `crates/paladin-notifications/Cargo.toml` — the two
  manifests bumped from `edition = "2021"` to `edition = "2024"`, no other key touched (plan
  04-01).
- `rust-toolchain.toml` — `channel = "1.97.1"`, the toolchain proof that Rust 2024 (stable since
  1.85) is fully supported here.
- `.planning/phases/04-release-coherence/04-release-measurement.md`, § "Entry measurement — edition
  2024 on paladin-ports" and § "Entry measurement — edition 2024 on paladin-notifications (workspace
  now uniform)" — plan 04-01's two build-proof measurement sections, including the verbatim
  `cargo fix --edition` output (zero source rewrites) and both `cargo build --workspace [--offline]
  [--no-default-features]` invocations exiting 0.

## Code Conformance

must change

**REL-02** is the executing requirement, and it has already executed this decision within this
phase: both stragglers were bumped by plan 04-01, and both required build legs are proven green
with full D-17 provenance. The verdict is therefore `satisfied` at the time of writing, not
pending.

This ADR does **not** touch `crates/paladin-ports/Cargo.toml`'s separate `doctest = false` key. That
is **DEBT-03** (Phase 8) — re-enabling `paladin-ports`'s doctests — governed by **HARD-07** (Phase
10), which decides which `cargo doc` bar applies. The `edition` key and the `doctest` key are
orthogonal Cargo manifest settings; this ADR's scope is the former only.

## Downstream Consumers

- **Phase 7's ARCH-03(a)** — the Rust edition half of ARCH-03's four competing-variant pairs.
  ARCH-03(a)'s requirement text is amended (plan 04-07, Task 2) to cite this ADR instead of
  re-adjudicating; its remaining scope becomes citing and applying this answer, not deciding it.
