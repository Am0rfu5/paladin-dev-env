# API Coverage — Phase 8: Verified Defect Closure

No external API integration: this phase closes five verified code-and-records defects
(`DEBT-01`…`DEBT-05`) — a CI job's stale path literals, a withdrawn deprecation requirement, a
doctest re-enablement, a CLI dependency-isolation gate, and a duplicate-struct consolidation — plus
its own close-out bookkeeping (this file, five amended ledger rows, five flipped checkboxes,
`PROMOTION.md`, `PROJECT.md`). It integrates, wraps, or consumes no external API, SDK, transport, or
service.

**Why the detector may fire.** This phase's own scope vocabulary is thick with the words "API" and
"CI," and a keyword detector reading it out of context will match every one of the following: the
`api-surface` CI job and its `cargo public-api`-based baseline diff (`DEBT-01`); the requirement IDs
`REQ-api-surface-ci` and `REQ-api-surface-reduction-target` this phase's ledger amendments cite;
`REQ-web-api-baseline-changelog` and `REQ-openapi-drift-guard`, named in the DEBT-01 closure note as
the requirement-text sites plan 08-05 corrected; and the `paladin-web` HTTP surface referenced
in-passing by several Milestone 4-6 ledger rows this phase's Task 1 also amends (`REQ-crate-isolation-ci`,
`REQ-workspace-ci-upgrade`). Every one of those refers to this project's own already-shipped Rust
surface, or to a CI job name and a static-analysis tool (`cargo public-api`, run entirely offline
against the local git tree) that compares two local text snapshots — not to a network call, provider
client, or transport this phase adds or exercises.

This phase modifies zero new network-calling code. `git diff Cargo.lock` across the phase's commit
range shows **removals only** — 14 packages removed (`structopt`, `structopt-derive`, `clap` v2.34.0
and their unique transitive dependencies), zero added, per `08-07-SUMMARY.md`'s verbatim
`git diff Cargo.lock | grep -c '^+\[\[package\]\]'` → `0` / `git diff Cargo.lock | grep -c '^-\[\[package\]\]'`
→ `14`. The only network access any task in this phase performs is `rustup toolchain install
nightly` against `static.rust-lang.org`, already-existing tooling for the pre-installed
`cargo public-api` baseline extraction (`DEBT-01`, plan 08-02) — not a call this phase's own code
makes at runtime. This phase handles no credential and no user input.
