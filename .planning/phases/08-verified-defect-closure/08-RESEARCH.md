# Phase 8: Verified Defect Closure - Research

**Researched:** 2026-08-06
**Domain:** Rust workspace CI/tooling defect closure (API-surface gate, deprecation policy, doctest
execution, CLI dependency isolation, value-type consolidation)
**Confidence:** HIGH — every claim below was produced by a command run in this checkout today, or is
a direct file:line read. Two claims are `[ASSUMED]` and are called out inline.

---

<user_constraints>
## User Constraints (from CONTEXT.md)

**Full CONTEXT.md is 24 numbered decisions (D-00a…D-24) plus Claude's Discretion and Deferred
Ideas; it is not re-typed verbatim here to avoid duplicating a 620-line document the planner reads
directly. This section indexes it precisely and states only what this research adds, corrects, or
sharpens.** The planner MUST read `.planning/phases/08-verified-defect-closure/08-CONTEXT.md` in
full before planning — every D-01…D-24 decision there is locked, not re-litigated by this document.

### Locked decisions (by ID, one-line index — see CONTEXT.md for full reasoning)

- **D-00a…D-00f** — inherited house rules: ADR shape (no frontmatter, 7 headings), precedence order
  (ADR → shipped tree → codebase map → intel → PRD → DOC → checkbox), `.project/` annotation not
  rewrite, ledger amend-in-place, D-00e evidence bar (command or file:line, verbatim), medieval
  ubiquitous language mandatory.
- **D-01…D-05 (DEBT-01)** — fix 5 tooling references literally (`project/` → `.project/`);
  regenerate the baseline (this is the item's real work, not the path fix); if regeneration is
  blocked, record a blocker rather than fake it; annotate 5 requirement-text references; fix
  `check-deprecations.sh` so it can fail (malformed-attribute grep must cover `src` **and**
  `crates`, not `src` alone).
- **D-06…D-08 (DEBT-02)** — withdraw M4 Epic 2 FR-8 via new ADR-0022 (⚠ HUMAN REVIEW), do not
  manufacture deprecations; reconcile `DEPRECATIONS.md` + `stable-api.md` + tree to agree (three-way
  reconciliation is the deliverable, not the ADR); restate the stale v0.2.0→v0.3.0→v1.0.0 timeline
  against 0.7.0 inside ADR-0022.
- **D-09…D-12 (DEBT-03)** — measure before scoping (remove `doctest = false`, run the doctests,
  scope every later task from the measured list); fix failures by making examples compile, `ignore`
  only for live-external-service examples with a one-line reason; drop `ci.yml:226`'s
  `--exclude paladin-ports` in the same commit; do not block on HARD-07 (Phase 10).
- **D-13…D-16 (DEBT-04)** — ⚠ HUMAN REVIEW ×2: migrate `src/main.rs` `structopt`→`clap` v4 **and**
  add `required-features = ["cli"]` to the `paladin` `[[bin]]`; feature-gate `paladin-herald`'s
  `colored`/`comfy-table` formatters (root-manifest gating alone is insufficient); both land in one
  ADR-0023; criterion 4 is proved by running `cargo tree`, not by reading the manifest.
- **D-17…D-20 (DEBT-05)** — canonical type absorbs the battalion copy's capabilities (extend first,
  never delete-then-recreate); both duplicate sites become `pub use` re-exports preserving import
  paths; `paladin-llm` already depends on `paladin-core`, no new edge; `VisionTokenUsage` is
  explicitly out of scope.
- **D-21…D-24 (cross-cutting)** — every closure claim proved by a command run in this environment;
  ADR allocation 0022 (deprecation withdrawal) + 0023 (CLI isolation), `PROMOTION.md` → 0024; the
  Milestone 4-6 ledger rows at lines 115, 116, 157, 160, 225 are amended in place; suggested
  decomposition ~8 plans / 4 waves (see CONTEXT.md D-24 verbatim — this research's own
  `## Wave/Ordering Constraints` section below confirms, corrects, and sharpens it with fresh file
  evidence).

### Claude's Discretion (from CONTEXT.md, unchanged)

- Exact `[features]` name(s) in `paladin-herald` (D-14) — this research's finding below (Herald
  consumer sites) **narrows** this discretion: whatever name is chosen must also be applied to two
  root-facade consumer files, not just `paladin-herald` itself (see `## DEBT-04` below).
- Whether ADR-0022/0023 are their own plans or fold into executing plans.
- The precise `clap` v4 idiom for `src/main.rs` (derive vs builder) — this research provides the
  exact derive translation below.
- Banner wording for D-04/D-07 `.project/` annotations.
- Whether the DEBT-03 measurement spike publishes its failure list as an artefact or inline —
  **this research already performed that measurement; see `## DEBT-03` below. There is no failure
  list to publish because there were no failures.**
- Whether the regenerated `.project/current-exports.txt` (D-02) commits with the path fix or alone.

### Deferred Ideas (from CONTEXT.md, out of scope — unchanged)

`VisionTokenUsage` convergence; auditing the 87 pre-existing `ignore`/`no_run`/`text` fences;
retiring/replacing `src/main.rs`; the `smartcontent-aggregator` product-name mismatch; which
`cargo doc --workspace --no-deps` bar governs (Phase 10/HARD-07); a `cargo tree` allowlist check in
CI (Phase 15); the user-facing binary-architecture mdbook page (Phase 16); the eight deprecated
GitHub Action references including `ci.yml:148` (Phase 15/PIPE-04 — **do not touch this line**, see
`## Prohibited Edit` below); Nyquist validation for Phases 1-4; ADRs-to-mdbook publication.

</user_constraints>

<phase_requirements>
## Phase Requirements

| ID | Description | Research Support |
|----|-------------|------------------|
| DEBT-01 | `api-surface` CI job works: 9 stale path references fixed, baseline regenerated, unchanged tree passes, intentional change fails | `## DEBT-01` — baseline regeneration **proven achievable in this environment today** (contradicts CONTEXT.md D-03's blocker assumption; see `## Corrections to 08-CONTEXT.md`) |
| DEBT-02 | Deprecation requirement answered either way, no third state | `## DEBT-02` — exact `stable-api.md` correction site found; `DEPRECATIONS.md` Open Questions enumerated for closure |
| DEBT-03 | `paladin-ports` doctests execute | `## DEBT-03` — **measured this session**: 96 passed, 0 failed, 94 ignored. DEBT-03 is a two-line change, not an examples-repair project. |
| DEBT-04 | Library-only build, zero CLI dependencies | `## DEBT-04` — clap v4 translation provided; full Dockerfile/CI/docs enumeration; Herald consumer-site gap found (extends D-14's scope) |
| DEBT-05 | One `TokenUsage` | `## DEBT-05` — all three derive sets confirmed, all call sites enumerated, one glob re-export found and confirmed non-colliding |

</phase_requirements>

---

## Summary

All five defects were re-verified against the live tree this session, and three produced findings
that materially change how the planner should scope work relative to `08-CONTEXT.md`. Most
important: **DEBT-01's baseline regeneration, which CONTEXT.md's D-03 treats as an at-risk fallback
path, was run successfully in this exact environment** — `cargo-public-api` 0.52.0 is already
installed, and a nightly toolchain installs cleanly via `rustup` because `static.rust-lang.org` is
reachable even though `crates.io` returns HTTP 403. The regenerated baseline is deterministic, diffs
cleanly against the stale one, and `check-api-surface.sh` correctly reports "unchanged" against it.
**DEBT-03's blocker is confirmed already fixed**: removing `doctest = false` and running
`cargo test --offline -p paladin-ports --doc` produces **96 passed, 0 failed, 94 ignored** — DEBT-03
is a two-line diff (drop the manifest flag, drop the CI exclusion), not an examples-repair project.
**DEBT-04 has one gap CONTEXT.md's D-14 does not name**: two root-facade files
(`src/infrastructure/adapters/herald/mod.rs`, `src/application/services/herald/herald_registry.rs`)
unconditionally construct `TableHerald`/`MarkdownHerald`, so feature-gating `paladin-herald` alone
would break the default library build — these two files need matching `#[cfg(feature = ...)]` splits.
DEBT-04 also surfaces a live Docker/CI break surface: two Dockerfiles, one CI step, and one docs page
build the un-gated `paladin` binary literally by name and will fail once `required-features = ["cli"]`
lands, unless updated in the same plan. DEBT-05's consolidation is confirmed additive-safe: one glob
re-export (`src/application/services/analysis/llm_analysis_service.rs:6`) picks up the canonical type
transparently, with no collision.

**Primary recommendation:** Do not treat DEBT-01's baseline regeneration as an at-risk fallback item
— run it as a first-class task; it is proven to work here. Do not treat DEBT-03 as a spike-then-repair
sequence — the measurement is done, and it found zero failures, so DEBT-03 collapses to a single
small task. Reserve the real planning attention for DEBT-04, which is genuinely the largest and
riskiest item, and is larger than CONTEXT.md scoped it (Herald's default-build breakage, plus five
build/CI/docs surfaces that assume the `paladin` binary builds unconditionally).

## Architectural Responsibility Map

| Capability | Primary Tier | Secondary Tier | Rationale |
|------------|-------------|----------------|-----------|
| API-surface diffing (DEBT-01) | CI / tooling (`scripts/*.sh`, `ci.yml`) | — | Shell scripts + GitHub Actions job; no application-layer code involved |
| Deprecation policy (DEBT-02) | Documentation / governance | Core (future `#[deprecated]` attrs, if ever used) | Today resolves entirely in docs/ADR; no code changes |
| Doctest execution (DEBT-03) | Core / Ports (rustdoc examples embedded in `paladin-ports` source) | CI (`ci.yml` doc-test job) | The examples live in the port trait definitions themselves |
| CLI dependency isolation (DEBT-04) | Application (binary entry points, `src/main.rs`) | Infrastructure/adapter (`paladin-herald` formatters) | Binary-target gating is an application-layer concern; Herald's `colored`/`comfy-table` split is an infrastructure-adapter concern |
| `TokenUsage` consolidation (DEBT-05) | Core (`paladin-core`, canonical owner) | Ports, Battalion, LLM (all three consume via re-export) | ADR-0016 already settled core ownership; this phase performs the mechanical re-export |

## Standard Stack

No new external stack is introduced by this phase. `clap` v4.5.40 is **already** a workspace
dependency (`Cargo.toml:122`, `optional = true`, part of the `cli` feature) — DEBT-04 does not add a
new package, it re-points `src/main.rs` at a dependency the tree already vendors.
`cargo-public-api` 0.52.0 is **already installed** in this environment (confirmed via
`which cargo-public-api` → `/usr/local/cargo/bin/cargo-public-api`); DEBT-01 does not need
`cargo install`.

| Tool | Version | Status | Evidence |
|------|---------|--------|----------|
| `clap` | 4.5.40 | Already a workspace dep, `optional = true`, in the `cli` feature list | `Cargo.toml:122`, `:284` |
| `cargo-public-api` | 0.52.0 | Already installed | `cargo public-api --version` → `cargo-public-api 0.52.0` |
| nightly toolchain | 1.99.0-nightly (7608eb7b0 2026-08-05) | Installable via `rustup toolchain install nightly` — **succeeded this session** | see `## DEBT-01` |

## Package Legitimacy Audit

**Not applicable.** No new external package is introduced by any DEBT-01…DEBT-05 task. `clap` is an
existing, already-vetted workspace dependency; the migration is a re-point of one binary's
dependency, not a new install. `cargo-public-api` is CI/dev tooling, not a workspace dependency
(`cargo install`, not `Cargo.toml`), and is already present in this environment.

---

## DEBT-01 — the `api-surface` CI job

### The five tooling references (D-01) — re-confirmed unchanged

```
scripts/check-api-surface.sh:6    BASELINE="${1:-project/current-exports.txt}"
scripts/extract-public-api.sh:6   OUTPUT_FILE="${1:-project/current-exports.txt}"
.github/workflows/ci.yml:172      run: ./scripts/check-api-surface.sh project/current-exports.txt
.github/workflows/ci.yml:182      (diff line, same literal)
.github/workflows/ci.yml:187      (diff line, same literal)
```
`[VERIFIED: direct file read this session]` — `.project/current-exports.txt` exists;
`project/current-exports.txt` does not (`ls`: "No such file or directory").

### THE HEADLINE FINDING: baseline regeneration works in this environment, right now

CONTEXT.md's D-03 says: *"If `cargo public-api` cannot run in this environment, land the path fix
and record a blocker... nothing has proven `cargo install cargo-public-api` works here."* **This
research disproves that premise for this checkout.** Sequence run this session, verbatim:

```
$ which cargo-public-api && cargo public-api --version
/usr/local/cargo/bin/cargo-public-api
cargo-public-api 0.52.0
```
`cargo-public-api` is **already installed** — no `cargo install` (and therefore no crates.io
network) is required at all.

```
$ rustup toolchain list
stable-x86_64-unknown-linux-gnu
1.97.1-x86_64-unknown-linux-gnu (active, default)
```
No nightly present initially — this matches CONTEXT.md's concern.

```
$ cargo public-api --simplified -p paladin-ai-core
error: toolchain 'nightly-x86_64-unknown-linux-gnu' is not installed
```
Confirms `cargo public-api` needs nightly, as CONTEXT.md states.

```
$ curl -sI https://crates.io          → HTTP/2 403   (blocked)
$ curl -sI https://static.rust-lang.org → HTTP/2 200   (reachable)
```
**The critical distinction CONTEXT.md's D-03 did not make:** `cargo install` needs crates.io
(blocked), but `rustup toolchain install nightly` fetches from `static.rust-lang.org` (reachable) —
these are different network dependencies with different outcomes in this environment.

```
$ rustup toolchain install nightly --profile minimal
info: syncing channel updates for nightly-x86_64-unknown-linux-gnu
info: latest update on 2026-08-06 for version 1.99.0-nightly (7608eb7b0 2026-08-05)
  nightly-x86_64-unknown-linux-gnu installed - rustc 1.99.0-nightly (7608eb7b0 2026-08-05)
```
**Succeeded.** (First attempt with a 15s timeout looked hung and was killed — it just needed more
than 15s; a 90s timeout completed cleanly. The planner's task should not use a short timeout here.)

```
$ bash scripts/extract-public-api.sh /tmp/.../test-exports.txt
Extracting public API surface using cargo-public-api...
✅ API surface extracted to /tmp/.../test-exports.txt (1968 items)
```
**The script runs end to end, unmodified, producing a real baseline: 1968 public items, 3547 lines.**

**Determinism check** — ran extraction twice, diffed the two outputs (excluding the timestamp
header): **0 lines differ.** Reproducible.

**"Unchanged tree passes" check** — ran `check-api-surface.sh` against the just-generated baseline:
```
✅ API surface unchanged
exit: 0
```
This directly proves ROADMAP criterion 1's "an unchanged tree makes it pass" clause is achievable.

**Diff against the stale `.project/current-exports.txt` (dated 2026-07-06)** — 53 diff lines,
real and substantive: methods added/removed on `ArsenalExecutionService`, a field on
`MCPServerConfig` (`auth_token_env`), the `paladin_builder::PaladinBuilder::add_mcp_sse` method, an
`MCPTransport` trait plus `mcp_sse_adapter`/`mcp_streamable_http_adapter` module churn. This confirms
D-02's premise — the baseline is genuinely stale and a path-only fix would produce a job that fails
on real drift, not "no baseline found."

**Conclusion for the planner:** D-02 (baseline regeneration) is not a stretch goal contingent on
environment luck — it is a proven, reproducible, ~1-2 minute local operation
(`rustup toolchain install nightly` is the only slow step, and it's a one-time cost). **Plan it as a
first-class task, not a "record a blocker" fallback.** The fallback in D-03 should be retained in
the plan text as a documented contingency (CI's own runner may differ from this devcontainer), but
the default path is: run it, it will work, commit the real baseline.

### `check-deprecations.sh` (D-05) — re-confirmed broken exactly as recorded

Both branches of the primary/fallback structure `exit 0` unconditionally except the final grep,
which scans `src/` only:
```
if grep -r "#\[deprecated\]" src/ --include="*.rs" | grep -v "since\|note"; then
    exit 1
fi
echo "✅ All deprecation attributes are properly formatted"
```
`[VERIFIED: direct file read]` — `scripts/check-deprecations.sh`, full text read this session. D-05's
fix (extend the grep to `src` **and** `crates`, make the script's own exit meaningful) is confirmed
minimal and correctly scoped — do not add an "N deprecations must exist" gate (that would prejudge
DEBT-02).

## DEBT-02 — the deprecation question (withdraw vs. implement)

### `stable-api.md`'s one concrete "current state" overclaim (the D-07(2) correction site)

`grep -n "deprecat" docs/src/api-reference/stable-api.md` (40 hits) shows the document is
overwhelmingly **policy/process** prose ("Types/methods being removed will be deprecated for at
least one minor version before removal", `:79`; the `#[deprecated(since=…)]` example at `:798-827`
is explicitly a hypothetical "Version 0.2.0" illustration, not a claim about today's tree) — this
policy language does **not** need correction; D-07(2) says the policy survives.

**Exactly one line makes a present-tense claim that is false today:**
```
stable-api.md:875   - **[Deprecations Tracking](.../CHANGELOG.md)** - Current and planned deprecations
```
`grep -rn '#\[deprecated' src crates` → **0 matches** (re-confirmed this session). "Current...
deprecations" is a false present-tense claim. **This is the one line D-07(2) must correct** — e.g.
to "Planned deprecation process (none currently active — see ADR-0022)". No other line in the file
asserts current deprecations exist.

### `DEPRECATIONS.md` Open Questions (D-07(1) — "answered or closed, not left dangling")

`.project/Milestone_4-Refactor-Crates-Features/Epic_2/DEPRECATIONS.md:206-211`, read this session:
```
## Open Questions
1. Adapter Visibility Strategy: Confirm #[doc(hidden)] approach vs. full deprecation
2. Factory Functions: Should we provide factory functions in v0.2.0 or wait for Epic 3?
3. Prelude Module: Should we add `paladin::prelude::*` for common imports?
4. Manager Refactoring: Wait for Epic 3 (Tier 3) before deprecating manager types?
```
Under D-06 (withdraw FR-8), all four become moot and should be closed with a one-line disposition
each, e.g.: (1) `#[doc(hidden)]` was the approach taken — 38 occurrences confirmed tree-wide,
resolved; (2)-(4) moot — no deprecation timeline exists to schedule factory functions or manager
types against, per ADR-0022's withdrawal. This is a closure annotation, not new investigation —
D-00c banner pattern applies.

### mdbook linkcheck — confirmed present, low risk for this edit class

`[VERIFIED: docs/book.toml]` — `[output.linkcheck]` section exists at `book.toml:24`;
`.github/workflows/docs.yml:46-48` installs `mdbook-linkcheck` 0.7.7 via `cargo install` (a
network-dependent CI step, unrelated to this phase). The DEBT-02 edits are prose corrections to
existing pages (no new pages, no new internal links) plus a `.project/` annotation banner (not part
of the mdbook source tree) — low risk of breaking linkcheck. No action needed beyond normal review.

## DEBT-03 — `paladin-ports` doctests

### THE MEASUREMENT (the single most important number in this document)

Per D-09, `doctest = false` was temporarily removed from `crates/paladin-ports/Cargo.toml` (the
stale "Task 7.0" comment removed with it) and
`cargo test --offline -p paladin-ports --doc` was run in full. **The file was restored to its
committed state immediately after** — confirmed via `git status --short` (empty) both immediately
after restoration and again at the end of this research session.

```
$ git diff crates/paladin-ports/Cargo.toml   # before restoring, for the record:
-[lib]
-# Doctests in copied port files reference `paladin::` (root crate) which would
-# require a circular dev-dependency.  Re-enable in Task 7.0 after rewriting
-# examples to use `paladin_ports::` / `paladin_core::` paths.
-doctest = false
+[lib]
+# TEMP-RESEARCH-EDIT: doctest = false removed for phase-8 research measurement; restored after.

$ cargo test --offline -p paladin-ports --doc
...
test result: ok. 96 passed; 0 failed; 94 ignored; 0 measured; 0 filtered out; finished in 0.02s

all doctests ran in 1.96s; merged doctests compilation took 1.94s

$ git checkout -- crates/paladin-ports/Cargo.toml
$ git status --short
(empty)
```

**96 passed. 0 failed. 94 ignored. Zero failure classes to report, because there were zero
failures.** This is empirical confirmation of CONTEXT.md's own specifics-1 finding ("the answer may
be 'they already pass'") — it does. The `use paladin::` circular-dependency problem the stale
`Cargo.toml` comment blames has already been fixed by whoever rewrote the examples to
`paladin_ports::`/`paladin_core::` paths; nobody removed the flag afterward.

**Scale reconciliation against CONTEXT.md's estimate:** CONTEXT.md's D-09 estimated ~187 "executing
candidates" from 274 total fenced blocks minus 87 `ignore`/`no_run`/`text`. The measured total is
190 collected doctests (96 run + 94 ignored) — close to the estimate; the small delta is explained by
`text`-fenced blocks not being collected as doctests at all (they're prose, not Rust) and by
`no_run` fences compiling-but-not-running, which show as `ok` (in the 96), not `ignored`.

**What this means for the plan:** DEBT-03 is **not** an examples-repair project. It is a two-line
change:
1. Remove `crates/paladin-ports/Cargo.toml:14-18`'s `doctest = false` block (and its stale comment).
2. Drop `.github/workflows/ci.yml:226`'s `--exclude paladin-ports` from
   `cargo test --workspace --doc --exclude paladin-ports`.

Both in one commit per D-11. No example-fixing task is needed. The plan should still include a
verification task that reruns the exact measurement above as its evidence, per D-00e/D-21's bar —
but there is no repair work to scope.

### `ci.yml:226`, not `:225` (D-11's citation-drift note) — re-confirmed

```
$ grep -n "cargo test --workspace --doc" .github/workflows/ci.yml
226:      run: cargo test --workspace --doc --exclude paladin-ports
```
`[VERIFIED]` — line 226, exactly as D-11/the ledger record. This is inside the `test` job (job header
at `ci.yml:191`), a **different job block** from the `api-surface` job (`ci.yml:140-190`) that
DEBT-01/DEBT-02 touch — see `## Wave/Ordering Constraints` below for why this means DEBT-03 has no
real line-contention with DEBT-01/DEBT-02.

## DEBT-04 — the library-only build

### `src/main.rs` full content (61 lines, read in full) and its clap v4 translation

Current `structopt` shape:
```rust
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "smartcontent-aggregator")]
struct Opt {
    #[structopt(short, long, default_value = "config.yml")]
    config: String,
}
// ...
let opt = Opt::from_args();
// ...
#[cfg(test)]
mod tests {
    // Opt::from_iter(&["test"]) / Opt::from_iter(&["test", "--config", "custom.yml"]) / ...-s"
}
```
`clap` v4.5.40 is already present as an optional workspace dependency
(`Cargo.toml:122`: `clap = { version = "4.5.40", features = ["derive", "cargo", "env"], optional = true }`),
gated by the `cli` feature (`Cargo.toml:284`). Once `required-features = ["cli"]` is added to the
`paladin` `[[bin]]` (D-13), `clap` is available to `src/main.rs` with **no new manifest dependency
line** — only the `structopt = "0.3"` line (`Cargo.toml:93`) is removed. Derive translation:

```rust
// Source: clap v4.5 derive API (already vendored; no new dependency)
use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "smartcontent-aggregator")]
struct Opt {
    #[arg(short, long, default_value = "config.yml")]
    config: String,
}
// ...
let opt = Opt::parse();
// ...
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_opt_default_config() {
        let opt = Opt::parse_from(["test"]);   // was: Opt::from_iter(&["test"])
        assert_eq!(opt.config, "config.yml");
    }
    // same substitution for the other two test fns: from_iter(&[..]) -> parse_from([..])
}
```
Mapping: `#[structopt(...)]` → `#[command(...)]` (container) / `#[arg(...)]` (field);
`Opt::from_args()` → `Opt::parse()`; `Opt::from_iter(&[...])` → `Opt::parse_from([...])`. This is a
mechanical, low-risk translation — the struct has exactly one field.

### Full enumeration: everything that assumes the `paladin` binary builds by default

This is the enumeration the additional-context flagged as high-risk ("a missed Dockerfile stage is
how this change breaks the release pipeline"). Every hit found by
`grep -rn "bin.*paladin\b|--bin paladin\b|bin/paladin\b" Dockerfile* Makefile docker/ k8s/ docs/
.github/workflows/*.yml` this session, triaged by whether it will actually break:

| # | Location | What it does | Breaks under `required-features=["cli"]`? |
|---|----------|---------------|---------------------------------------------|
| 1 | `Dockerfile:33` | `RUN cargo build --release --workspace --bin paladin` (no `--features cli`) | **Yes — build fails**, must add `--features cli` |
| 2 | `Dockerfile:52,73` | `COPY .../paladin ...` / `ENTRYPOINT [".../paladin"]` | Downstream of #1; no change needed to these lines themselves |
| 3 | `Dockerfile.chef:74` | Same `cargo build --release --workspace --bin paladin`, no `--features cli` | **Yes — build fails**, must add `--features cli` |
| 4 | `Dockerfile.chef:94,113,116` | `COPY`, `HEALTHCHECK CMD [".../paladin","health"]`, `ENTRYPOINT` | Downstream of #3. **Separate pre-existing defect, independent of DEBT-04**: the `Opt` struct has no `health` subcommand — `structopt`/`clap` would reject the literal arg `"health"` today already. Flag, do not fix (out of DEBT-04's literal scope; likely a stale healthcheck copied from `paladin-server`'s pattern) |
| 5 | `.github/workflows/feature-flags.yml:144` | Step **literally named** "Verify paladin binary builds **without** cli feature" — `cargo build --bin paladin` | **Yes — this step's entire assertion inverts.** Must be updated/removed/repurposed in the same plan, or CI fails immediately after D-13 lands |
| 6 | `docker/testserver/Dockerfile` (production stage) | `RUN cargo build --release` (no `--bin`, no `--features cli`), `COPY .../paladin`, `CMD ["./paladin"]` | Would break if built, **but confirmed dead**: `docker-compose.test.yml`'s `integration-tests` service targets the `test` stage (`cargo test`), not `production` — low priority, optional fix for hygiene |
| 7 | `docker/redis/Dockerfile:107` | `CMD ["./paladin"]` | Confirmed **orphaned** — zero references anywhere in Makefile/workflows/compose files. Out of scope, note only |
| 8 | `docs/src/deployment/docker.md:135,146,156` | Documents the exact `Dockerfile` build/copy/entrypoint commands as source-of-truth prose | **Yes — must update alongside #1** or docs silently go stale (this file is mdbook *source*; `docs/book/html/deployment/docker.html` is a **generated artefact** and needs no manual edit, it regenerates on `mdbook build`) |
| 9 | `k8s/deployment.yaml:65,68` | `image: paladin:test`, `args: ["-c", "echo 'Paladin started' && sleep 3600"]  # Placeholder for testing` | Explicitly commented "Placeholder for testing" — low priority; would need the image rebuilt with `--features cli` if this manifest is ever actually deployed, but it is not wired to any real command |
| — | `k8s/server/deployment.yaml`, all other `k8s/*.yaml` | Reference `paladin-server:*` image, not `paladin` | **Confirmed zero other references to the bare `paladin` binary in `k8s/`** |
| — | `Makefile:307` `build-docker` target | `docker build -f docker/Dockerfile ...` | **Pre-existing, unrelated defect**: `docker/Dockerfile` does not exist anywhere in the tree (`find -iname Dockerfile*` finds only `/Dockerfile`, `/Dockerfile.chef`, `/Dockerfile.server`, `.devcontainer/Dockerfile*`) — this Make target is already broken today, independent of DEBT-04. Flag, do not fix |
| — | `docker/docker-compose.dev.yml:36` | `command: cargo run` (bare, no `--bin`) | **Pre-existing, unrelated defect**: no `default-run` key in `Cargo.toml` `[package]` (confirmed absent), and three `[[bin]]` targets exist — `cargo run` bare is already ambiguous/broken today. Flag, do not fix |

**Planner action:** items #1, #3, #5, #8 are real, will-break-on-merge consequences of D-13 and
belong in the same plan/commit as the `src/main.rs` migration and the `[[bin]]` gate — this is the
concrete list the additional-context asked for. Items #4, #6, #7, and the two Makefile/compose
findings are pre-existing, unrelated defects surfaced as a side effect of this search; recording them
(per this corpus's own house style) is correct, but fixing them is new scope, not DEBT-04.

### `paladin-herald`'s `colored`/`comfy-table`: function-body-only, not in public signatures — but two ROOT-FACADE consumers are ungated (extends D-14)

**Signature check (confirms D-14's premise is achievable as a clean cfg-gate):**
```
table_herald.rs:41   pub struct TableHeraldConfig { pub max_column_width: usize, pub border_style: String }
table_herald.rs:63   pub struct TableHerald { config: TableHeraldConfig }   // private field
table_herald.rs:69   pub fn new(config: TableHeraldConfig) -> Self
markdown_herald.rs:27  pub struct MarkdownHeraldConfig { pub include_colors: bool, pub heading_level: u8 }
markdown_herald.rs:105 pub struct MarkdownHerald { ... }
markdown_herald.rs:115,136  pub fn new() -> Self / pub fn with_config(config: MarkdownHeraldConfig) -> Self
```
`[VERIFIED: direct file read]` — no `comfy_table::*` or `colored::*` type appears in any `pub fn`
signature or `pub` struct field. Both config structs use only plain types (`usize`, `String`, `bool`,
`u8`). `comfy_table`/`colored` are used **only inside function bodies** (rendering logic). **D-14's
premise holds: this is a clean additive `#[cfg(feature = ...)]` gate, not a breaking API split.**

**The gap CONTEXT.md's D-14 does not name:** two files in the root facade construct
`TableHerald`/`MarkdownHerald` **unconditionally**, with no feature gate anywhere in their module
declarations:
```
src/infrastructure/adapters/mod.rs:8       pub mod herald;              // no #[cfg(...)] — contrast
                                                                          // with :4 "content-processing"
                                                                          // and :12 "notifications",
                                                                          // both of which ARE gated
src/infrastructure/adapters/herald/mod.rs:9   pub use paladin_herald::{JsonHerald, MarkdownHerald, TableHerald};
src/application/services/mod.rs:5          pub mod herald;              // also ungated
src/application/services/herald/herald_registry.rs:248-250
    registry.register("json", Arc::new(JsonHerald::new()));
    registry.register("markdown", Arc::new(MarkdownHerald::new()));
    registry.register("table", Arc::new(TableHerald::default()));
```
`[VERIFIED: direct file read + grep this session]` — **if `paladin-herald`'s Cargo.toml gates
`table_herald`/`markdown_herald` behind a feature the root `--no-default-features` build does not
enable, these two files will fail to compile** (`TableHerald`/`MarkdownHerald` types would not
exist), breaking the *default library build itself*, not merely leaving CLI deps in the dependency
tree. **D-14's scope must extend to these two files**: split the `pub use` in
`infrastructure/adapters/herald/mod.rs` (JSON unconditional, Markdown/Table gated) and gate the two
`registry.register(...)` calls in `herald_registry.rs` (keep `"json"` unconditional). This is a small,
mechanical addition to D-14's task list, not a new architectural question — but it is not optional;
skipping it means criterion 4's `cargo tree --lib --no-default-features` command wouldn't even get to
run because the library wouldn't compile.

**What consumes Herald's formatters** (`grep -rln "JsonHerald|MarkdownHerald|TableHerald"`):
`herald_registry.rs`, `config/herald.rs` (config structs only, no formatter construction),
`infrastructure/adapters/herald/mod.rs`, `application/services/paladin/{paladin_builder,
paladin_execution_service}.rs`, plus 6+ files under `examples/` and `tests/integration/`. The
`examples/`/`tests/` consumers are not part of the default library build's dependency graph (examples
compile as separate targets and already declare their own feature requirements where needed) and are
out of scope for criterion 4, which is `--lib`-scoped.

## DEBT-05 — one `TokenUsage`

### The three definitions, derives and impls (re-confirmed against ADR-0016's table)

```rust
// CANONICAL — crates/paladin-core/src/platform/container/token_usage.rs:12-13
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenUsage { pub prompt_tokens: u32, pub completion_tokens: u32, pub total_tokens: u32 }
// no inherent impl block

// DUPLICATE 1 — crates/paladin-core/src/platform/container/battalion/mod.rs:496-524
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct TokenUsage { /* same 3 fields */ }
impl TokenUsage {
    pub fn new(prompt_tokens: u32, completion_tokens: u32) -> Self { .. }
    pub fn from_total(total_tokens: u32) -> Self { .. }
}

// DUPLICATE 2 — crates/paladin-llm/src/llm_analysis_service.rs:50-55
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenUsage { /* same 3 fields */ }
// no inherent impl block
```
`[VERIFIED: direct file read, all three]`. Matches ADR-0016's table exactly. **D-17's sequencing is
confirmed correct and necessary**: the canonical type must gain `Default`, `PartialEq`, `new()`,
`from_total()` before the battalion copy can become a re-export, or the call sites below break.

### Every call site depending on the battalion copy's extra capability (D-17's risk surface)

```
$ grep -rn "TokenUsage::new(|TokenUsage::from_total(" src crates tests examples
```
11 call sites, all resolve correctly once the canonical type gains the two constructors:
```
battalion/mod.rs:1135,1143,1159,1172,1173,1201,1209,1210,1211,1267   (11 sites, all ::new/::from_total, in-crate test code)
paladin-herald/src/table_herald.rs:403        paladin_core::...::battalion::TokenUsage::from_total(token_count)
paladin-battalion/src/phalanx_service.rs:279  TokenUsage::from_total(result.token_count)
paladin-herald/src/json_herald.rs:391,392     TokenUsage::from_total(137) / (263)   [doc-example/test code]
paladin-herald/src/markdown_herald.rs:510,514 paladin_core::...::battalion::TokenUsage::from_total(...)
paladin-battalion/src/formation_service.rs:231 TokenUsage::from_total(result.token_count)
```
**Two Herald call sites (`table_herald.rs:403`, `markdown_herald.rs:510,514`) explicitly reference
the type via the `battalion::TokenUsage` path and call `::from_total()` on it** — this confirms D-18's
`pub use` (not a type alias without methods) is the right mechanism: a `pub use` re-export carries the
type's inherent methods with it automatically, so once `new`/`from_total` live on the canonical
struct, `battalion::TokenUsage::from_total(...)` continues to resolve with zero call-site changes.

**`Default`/`PartialEq` usage on the battalion copy:**
```
battalion/mod.rs:1151   let usage = TokenUsage::default();   // one site, test code
```
`grep` for `TokenUsage ==` / `assert_eq!(...TokenUsage...)` beyond this found no additional
`PartialEq`-dependent comparisons. One `Default::default()` call, one `PartialEq` derive with no
found direct `==` usage outside the derive itself — both trivially safe to add to the canonical type
(adding derives is additive; nothing currently depends on their *absence*).

### Reference count and the one glob re-export (D-18's collision check)

```
$ grep -rc "TokenUsage" src crates tests examples | awk -F: '{s+=$2} END{print s}'
182
```
Close to CONTEXT.md's cited ~179 (small growth since Phase 7, expected). **One glob re-export found:**
```
src/application/services/analysis/llm_analysis_service.rs:6
    pub use paladin_llm::llm_analysis_service::*;
```
**Confirmed non-colliding.** This glob re-export currently picks up `paladin_llm::llm_analysis_service::TokenUsage`
(duplicate 2). Once that module's `TokenUsage` becomes `pub use paladin_core::...::TokenUsage;`
(D-18), the glob re-export transparently follows the new re-export — Rust does not error on a glob
re-exporting a re-exported item, and no other identifier in that module collides with the name
`TokenUsage`. **No `pub use ... as ...` rename exists anywhere for `TokenUsage`** — confirmed via
`grep -rn "TokenUsage as"` (0 hits). `grep -rn "TokenUsage" src/lib.rs src/prelude.rs
src/application/services/mod.rs` → 0 hits — the root facade does not re-export `TokenUsage` directly
by either path today, so there is no existing dual-path ambiguity to resolve.

**One pre-existing re-export precedent found inside `paladin-core` itself** (beyond the
`paladin-ports` precedent CONTEXT.md already cites):
```
crates/paladin-core/src/platform/container/herald.rs:28
    pub use crate::platform::container::token_usage::TokenUsage;
```
Confirms the `pub use` pattern is already idiomatic within `paladin-core`, not just at the
`paladin-ports` boundary — one more precedent to copy, not invent.

---

## Wave/Ordering Constraints (file-contention analysis)

Re-verified against the live `ci.yml` and manifests this session:

- **`api-surface` job is `ci.yml:140-190`; the `test` job (with the doctest exclusion at `:226`) is
  a *separate* job block starting at `ci.yml:191`.** DEBT-01 touches lines within 140-190
  (`:172,182,187`, all inside the `api-surface` job). DEBT-03 touches line 226, inside the `test`
  job. **These do not overlap — DEBT-01 and DEBT-03 have zero real line-contention in `ci.yml`**,
  contrary to a literal reading that groups them as one contended region. They can run as fully
  parallel plans.
- **DEBT-02 does not touch `ci.yml` at all.** Its D-06/D-07/D-08 scope is `DEPRECATIONS.md`,
  `stable-api.md`, and a new ADR-0022 — no line in `ci.yml`'s `api-surface` job changes for DEBT-02.
  (D-05's `check-deprecations.sh` fix is scoped to DEBT-01, not DEBT-02, per CONTEXT.md's own
  decision text — the script file itself is touched by DEBT-01's plan, not DEBT-02's.) **DEBT-01 and
  DEBT-02 have no file contention at all** and can run fully in parallel; CONTEXT.md D-24's
  sequencing of DEBT-02 into wave 2 is because it needs ADR-0022 authored first (a content
  dependency), not a file-contention dependency — worth stating plainly to the planner since the
  additional-context's framing implied a shared-file reason that this research did not confirm.
- **DEBT-04's two halves (D-13, D-14) both touch the root `Cargo.toml`** — D-13 removes
  `structopt = "0.3"` (`:93`) and adds `required-features` to the `paladin` `[[bin]]` (`:241-242`);
  D-14 either marks `colored`/`comfy-table` (`:125-126`) `optional = true` or removes them from the
  root manifest entirely, and extends the `cli` feature list (`:284`). **Confirmed real, single-file
  contention — this must stay one plan** (D-24's wave-3 sequencing is correct and necessary).
  Additionally, D-14 now also touches `crates/paladin-herald/Cargo.toml` (new `[features]` section)
  **and** two additional root-facade `.rs` files this research found (`infrastructure/adapters/herald/mod.rs`,
  `application/services/herald/herald_registry.rs`) — none of these overlap with DEBT-01/02/03/05's
  files, so DEBT-04 as a whole remains parallel-safe against the other four items, just internally
  sequential (D-13 then D-14 then the `cargo tree` proof, all in one plan, as D-24 already specifies).
- **DEBT-05 touches only `paladin-core/.../token_usage.rs`,
  `paladin-core/.../battalion/mod.rs`, and `paladin-llm/src/llm_analysis_service.rs`** — zero overlap
  with any other DEBT item's files. Fully parallel with everything.
- **Net correction to the additional-context's framing:** the only *real* single-file contention in
  this phase is DEBT-04's two halves on root `Cargo.toml`. DEBT-01/DEBT-02/DEBT-03/DEBT-05 can all
  run as fully independent, fully parallel plans with zero file overlap between them or with DEBT-04.
  CONTEXT.md D-24's suggested 4-wave shape is still reasonable (ADR-authoring-before-executing is a
  legitimate *content* dependency, not a file dependency), but the planner should know the underlying
  constraint is narrower than "DEBT-01 and DEBT-02 share a CI region" — they do not.

## ADR File Shape (confirmed) and `PROMOTION.md` Bookkeeping

Confirmed by reading ADR-0016, ADR-0019, ADR-0021, ADR-0006 in full this session: **no frontmatter**,
exactly these seven `##` headings in order — `Status` (with a `**Date:**` line under it),
`Context`, `Decision`, `Considered Options`, `Code Locations`, `Code Conformance`,
`Downstream Consumers`. ADR-0022 and ADR-0023 must match this shape exactly.

`.planning/decisions/PROMOTION.md:45` — `**Next free ADR number: 0022**` (confirmed this session).
Per D-22, this phase allocates 0022 (DEBT-02's withdrawal ADR) and 0023 (DEBT-04's CLI-isolation
ADR); the close-out plan (wave 4, per D-24) must advance this line to **0024**.

## Prohibited Edit — `ci.yml:148`

```
$ grep -n "actions-rs/toolchain@v1" .github/workflows/ci.yml
148:        uses: actions-rs/toolchain@v1
393:        uses: actions-rs/toolchain@v1
792:        uses: actions-rs/toolchain@v1
```
`[VERIFIED]` — line 148 is confirmed inside the `api-surface` job (job header `ci.yml:140`, this line
sits under the "Install Rust toolchain" step at `:146-150`), which DEBT-01 edits three other lines
of. **Do not touch line 148** — it belongs to Phase 15 / PIPE-04's full eight-reference
action-modernization sweep. The planner's DEBT-01 task should edit `:172,182,187` only and explicitly
leave `:148` alone even though it sits in the same job block.

---

## Corrections to 08-CONTEXT.md

Per the D-00e/D-21 evidence bar and this session's own working discipline ("documents lie about
themselves in both directions"), two findings materially correct or extend CONTEXT.md's recorded
positions. Both are corrections of *scope/risk assessment*, not of any locked decision — D-02, D-03,
and D-14 all still stand; what changes is which branch of each is the expected path.

1. **D-03's fallback framing is the wrong default for this environment.** CONTEXT.md D-03 states:
   *"If `cargo public-api` cannot run in this environment, land the path fix and record a
   blocker... nothing has proven `cargo install cargo-public-api` works here."* **This session
   proved the opposite**: `cargo-public-api` is already installed (no `cargo install` needed at
   all), and `rustup toolchain install nightly` succeeds because it fetches from
   `static.rust-lang.org`, which is reachable, even though `crates.io` (needed only for
   `cargo install`, which turned out to be unnecessary) returns HTTP 403. Baseline regeneration was
   run to completion, is deterministic, and correctly reports "unchanged" against itself. **The
   planner should scope DEBT-01's D-02 baseline regeneration as the default, proven path — not a
   best-effort attempt with a pre-written excuse.** D-03's blocker text should be kept in the plan
   only as a documented contingency (in case CI's actual runner differs from this devcontainer), not
   as the expected outcome.

2. **D-14's scope is missing two files that will break the default build, not just leave CLI deps in
   the tree.** CONTEXT.md's D-14 scopes the Herald feature-gate to `paladin-herald`'s own
   `Cargo.toml` + `table_herald.rs`/`markdown_herald.rs`/`lib.rs`. This session found that
   `src/infrastructure/adapters/herald/mod.rs` (`pub use paladin_herald::{JsonHerald, MarkdownHerald,
   TableHerald};`, ungated) and `src/application/services/herald/herald_registry.rs`
   (unconditionally calls `TableHerald::default()`/`MarkdownHerald::new()`, ungated — contrast with
   the *sibling* modules `content-processing` and `notifications` in the same file, which *are*
   `#[cfg(feature = ...)]`-gated) will fail to **compile** under `--no-default-features` once
   `paladin-herald` itself gates those types behind a feature the default build doesn't enable. This
   is not a new architectural question (D-14's "genuinely infeasible" fallback does not apply — the
   fix is a clean, additive `#[cfg(feature = ...)]` split, matching the pattern the file already uses
   for its sibling modules) — it is two additional files the plan's task list must include, or
   criterion 4 will never get to run `cargo tree` because the library won't build.

No other CONTEXT.md decision, evidence citation, or line number was found incorrect this session.
All D-01, D-05, D-09 through D-12, D-13, D-15 through D-20 evidence was re-verified and matches
exactly.

---

## Common Pitfalls

### Pitfall 1: Treating DEBT-03 as an examples-repair project
**What goes wrong:** A planner reads "~187 executing candidates" and schedules multiple
example-fixing tasks/waves.
**Why it happens:** CONTEXT.md's sizing note (correctly) says DEBT-03 is unmeasurable until measured
— but it has now been measured, with a zero-failure result.
**How to avoid:** Cite this document's `## DEBT-03` measurement directly; scope DEBT-03 as a single
two-line task plus a verification task that reruns the same command.
**Warning signs:** Any DEBT-03 task list longer than ~2 tasks.

### Pitfall 2: Gating `paladin-herald`'s Cargo.toml without gating its two facade consumers
**What goes wrong:** `cargo tree --lib --no-default-features` never gets evaluated because
`cargo build --lib --no-default-features` fails first, with an unhelpful "cannot find type
`TableHerald`" error far from the actual manifest change.
**Why it happens:** The feature-gate lives in `paladin-herald`'s manifest; the breakage surfaces in
the root package's `herald_registry.rs`/`infrastructure/adapters/herald/mod.rs`, two directories
away.
**How to avoid:** Include both files in DEBT-04's task list explicitly (see `## DEBT-04` above);
verify with `cargo check --lib --no-default-features` (not just `cargo tree`) before trusting the
`cargo tree` proof.
**Warning signs:** `cargo tree --lib --no-default-features` command errors instead of printing a
tree.

### Pitfall 3: Adding `required-features = ["cli"]` to the `paladin` binary without updating the build surfaces that assume it builds unconditionally
**What goes wrong:** `docker build -f Dockerfile .`, `docker build -f Dockerfile.chef .`, and the
`feature-flags.yml` CI job all break in the same PR that "only" touched `src/main.rs` and
`Cargo.toml`.
**Why it happens:** The binary-gate change is a manifest+source edit; its blast radius is Docker
build commands and a CI step name, none of which a `cargo test`/`cargo clippy` run will catch.
**How to avoid:** Use the enumerated list in `## DEBT-04` (`Dockerfile:33`, `Dockerfile.chef:74`,
`feature-flags.yml:144`, `docs/src/deployment/docker.md:135,146,156`) as the DEBT-04 task's literal
checklist; each of these four needs a one-line `--features cli` addition (or, for the CI step, a
rename/removed-assertion decision).
**Warning signs:** A green `cargo test --workspace` with a red Docker build or a red
`feature-flags.yml` run.

### Pitfall 4: Assuming `cargo public-api` cannot run here and skipping baseline regeneration
**What goes wrong:** DEBT-01 ships with the path fix only, against a stale baseline, and criterion 1
("unchanged tree passes") is never actually proven true.
**Why it happens:** CONTEXT.md's own D-03 primes this expectation; without re-checking, a planner
would reasonably default to the documented-blocker path.
**How to avoid:** Follow this document's `## DEBT-01` sequence — it is proven to work, start to
finish, in this exact checkout.
**Warning signs:** A DEBT-01 SUMMARY that says "baseline refresh blocked" without having actually
attempted `rustup toolchain install nightly` with a timeout longer than ~20 seconds.

## Code Examples

### `src/main.rs` clap v4 migration (full, verified translation)
```rust
// Source: this file's own current structopt shape, translated to clap v4.5 derive API
// (clap already vendored: Cargo.toml:122, gated by the `cli` feature: Cargo.toml:284)
use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "smartcontent-aggregator")]
struct Opt {
    #[arg(short, long, default_value = "config.yml")]
    config: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(debug_assertions)]
    {
        let _ = dotenv::dotenv();
    }
    env_logger::Builder::from_env(Env::default().default_filter_or("debug")).init();

    let opt = Opt::parse();
    let config = match Settings::load_from_file(&opt.config) {
        Ok(cfg) => cfg,
        Err(e) => {
            eprintln!("Failed to load configuration: {:?}", e);
            std::process::exit(1);
        }
    };
    info!("Loaded configuration: {:?}", config);
    setup_and_run(config).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_opt_default_config() {
        let opt = Opt::parse_from(["test"]);
        assert_eq!(opt.config, "config.yml");
    }

    #[test]
    fn test_opt_custom_config() {
        let opt = Opt::parse_from(["test", "--config", "custom.yml"]);
        assert_eq!(opt.config, "custom.yml");
    }

    #[test]
    fn test_opt_short_config() {
        let opt = Opt::parse_from(["test", "-c", "short.yml"]);
        assert_eq!(opt.config, "short.yml");
    }
}
```

### The `pub use` re-export pattern to copy for DEBT-05 (two precedents, not one)
```rust
// Precedent 1 — crates/paladin-ports/src/output/llm_port.rs:671 (cross-crate)
pub use paladin_core::platform::container::token_usage::TokenUsage;

// Precedent 2 — crates/paladin-core/src/platform/container/herald.rs:28 (intra-crate)
pub use crate::platform::container::token_usage::TokenUsage;
```
DEBT-05's two targets: `battalion/mod.rs:497` becomes
`pub use crate::platform::container::token_usage::TokenUsage;` (intra-crate, like precedent 2);
`llm_analysis_service.rs:51` becomes
`pub use paladin_core::platform::container::token_usage::TokenUsage;` (cross-crate, like precedent
1, since `paladin-llm` already depends on `paladin-core` per `Cargo.toml:27`).

## Assumptions Log

| # | Claim | Section | Risk if Wrong |
|---|-------|---------|---------------|
| A1 | `docker/testserver/Dockerfile`'s `production` stage is genuinely unreferenced by any CI trigger beyond `docker-compose.test.yml` (checked Makefile, workflows, and compose files only) `[ASSUMED — checked but not exhaustively; a manual `docker build --target production` invocation outside this repo's tracked automation would not appear in this grep]` | DEBT-04, item 6 | Low — if wrong, a manual/undocumented build path breaks; no tracked automation is affected |
| A2 | `k8s/deployment.yaml`'s `paladin:test` placeholder is not deployed by any tracked CI/CD pipeline `[ASSUMED — confirmed the file says "Placeholder for testing" and found no workflow reference to `kubectl apply` against this specific manifest, but did not exhaustively search all possible deployment automation]` | DEBT-04, item 9 | Low — explicitly labeled placeholder |

**All other claims in this document were verified via a command or direct file read in this exact
checkout this session — see the inline `[VERIFIED: ...]` tags throughout.**

## Open Questions

1. **Should DEBT-04's plan also fix the four pre-existing, unrelated defects this research
   surfaced** (`Dockerfile.chef`'s `health`/`run` CMD mismatch with the actual `Opt` CLI surface,
   `Makefile:307`'s reference to a nonexistent `docker/Dockerfile`, `docker-compose.dev.yml`'s
   already-ambiguous bare `cargo run`, `docker/redis/Dockerfile`'s orphaned status)?
   - What we know: none of the four is caused by or required for DEBT-04's own success criteria.
   - What's unclear: whether "no shipped surface is removed without a recorded decision behind it"
     (the phase's second goal clause) implies these should at least be *recorded* even if not fixed.
   - Recommendation: record each with a one-line note in the DEBT-04 plan's SUMMARY (consistent with
     this corpus's house style of naming defects even when out of scope), fix none of them — new
     scope belongs to a future phase or ledger note, not this one.
2. **Exact `[features]` name for `paladin-herald`'s gate** (Claude's Discretion, per CONTEXT.md) —
   this research found no reason to prefer one name over another; whichever is chosen must be
   threaded through three files now (paladin-herald's own manifest, plus the two root-facade files
   found here), not one.

## Environment Availability

| Dependency | Required By | Available | Version | Fallback |
|------------|------------|-----------|---------|----------|
| `cargo-public-api` | DEBT-01 baseline regeneration | ✓ (pre-installed) | 0.52.0 | — |
| nightly Rust toolchain | DEBT-01 (`cargo public-api` requirement) | ✓ (installable via `rustup`, confirmed this session) | 1.99.0-nightly (7608eb7b0 2026-08-05) | — |
| `clap` v4 | DEBT-04 `src/main.rs` migration | ✓ (already a workspace dep) | 4.5.40 | — |
| crates.io network access | Not required by any DEBT item (no new `cargo install`/new dependency) | ✗ (HTTP 403, confirmed) | — | N/A — nothing in this phase needs it |
| `static.rust-lang.org` network access | `rustup toolchain install nightly` | ✓ (HTTP 200, confirmed) | — | — |

**Missing dependencies with no fallback:** none — every dependency this phase needs is either already
present or was proven installable this session.

## Validation Architecture

### Test Framework
| Property | Value |
|----------|-------|
| Framework | `cargo test` (unit/integration/doc), shell scripts for CI gating, `cargo public-api`/`cargo tree` for surface proofs |
| Config file | `.github/workflows/ci.yml` (job definitions); `crates/paladin-ports/Cargo.toml` (`[lib] doctest`) |
| Quick run command | Per-item, see table below |
| Full suite command | `cargo test --workspace --offline` (workspace gate, per CLAUDE.md) then `cargo fmt --check` then `cargo clippy -- -D warnings` |

### Phase Requirements → Test Map (per success criterion)

| # | Success Criterion | Observable | Command | Sampling Point |
|---|---|---|---|---|
| 1 | An intentional API change fails CI; an unchanged tree passes; all 9 stale path refs gone | `check-api-surface.sh`'s exit code and stdout, against the regenerated `.project/current-exports.txt` | `bash scripts/extract-public-api.sh /tmp/x.txt && bash scripts/check-api-surface.sh .project/current-exports.txt` (expect exit 0, "unchanged"); then temporarily add a `pub fn` somewhere and rerun (expect exit 1, "changed") | Per-task commit (regeneration task); phase gate (both directions proven, diff reverted) |
| 2 | `#[deprecated]` either implemented with replacement+version, or withdrawn with recorded reason, no third state | `grep -rn '#\[deprecated' src crates` count, cross-checked against `DEPRECATIONS.md`/`stable-api.md`/ADR-0022 agreement | `grep -rn '#\[deprecated' src crates` (expect 0, matching ADR-0022's withdrawal) plus a manual read confirming all three documents (`DEPRECATIONS.md`, `stable-api.md`, ADR-0022) state the same zero-and-why | Phase gate (DEBT-02 close-out) |
| 3 | `cargo test --workspace --doc` runs `paladin-ports`; port trait examples compile | The doctest job's own pass/fail count for the `paladin-ports` crate specifically | `cargo test --offline -p paladin-ports --doc` (expect the measured 96 passed / 0 failed / 94 ignored, or better if any `ignore` fences are later un-ignored) then `cargo test --offline --workspace --doc` (expect no `--exclude`, full workspace doc-test count includes `paladin-ports`) | Per-task commit; phase gate (full workspace run) |
| 4 | `cargo tree --lib --no-default-features` shows none of `structopt`, `colored`, `comfy-table` | The literal `cargo tree` output | `cargo build --offline --lib --no-default-features` (must succeed — proves the Herald-consumer gating didn't break compilation) then `cargo tree --offline --no-default-features \| grep -E 'structopt|colored|comfy-table'` (expect no output) | Per-task commit (after D-13, again after D-14); phase gate (final combined proof, captured verbatim in the SUMMARY per D-16) |
| 5 | `grep -rn 'pub struct TokenUsage' crates src` returns exactly one result | The literal grep count | `grep -rn 'pub struct TokenUsage' crates src \| wc -l` (expect 1) plus `cargo test --offline --workspace --lib` (expect the same or higher pass count as pre-change, proving the re-exports didn't break any of the 182 reference sites) | Per-task commit; phase gate |

### Sampling Rate
- **Per task commit:** the specific command from the table above for that DEBT item.
- **Per wave merge:** `cargo test --workspace --offline`, `cargo fmt --check`, `cargo clippy --workspace -- -D warnings` (the CLAUDE.md workspace gate).
- **Phase gate:** all five commands above run in sequence, plus `cargo test --workspace --offline` for the 84% ADR-0006 coverage floor (measure via the ADR-0006-recorded `rustc`/`llvm-cov` pipeline, not `cargo tarpaulin` — see ADR-0006's "tool-of-record note").

### Wave 0 Gaps
None — every command in the table above already runs against existing, committed test/CI
infrastructure. No new test file or fixture is needed before this phase's tasks can execute.

## Security Domain

No new dependency, network surface, credential, or user input handling is introduced by any of the
five DEBT items — this is a CI-tooling, documentation, and dependency-hygiene phase. `V6
Cryptography`/`V5 Input Validation`-class categories are not applicable. One adjacent note: DEBT-04's
`clap` migration replaces `structopt` (whose upstream has publicly stated it is superseded by
clap 3+) with a maintained, actively-patched dependency — a net risk reduction, not a new surface.

## Sources

### Primary (HIGH confidence — commands run in this checkout this session)
- `cargo public-api --version`, `rustup toolchain list`, `rustup toolchain install nightly`,
  `bash scripts/extract-public-api.sh`, `bash scripts/check-api-surface.sh` — DEBT-01 baseline
  regeneration proof.
- `cargo test --offline -p paladin-ports --doc` (with `doctest = false` temporarily removed and
  restored) — DEBT-03 measurement.
- `grep`/direct file reads of `src/main.rs`, `Cargo.toml` (root and `paladin-herald`), all three
  `TokenUsage` definitions, `herald_registry.rs`, `infrastructure/adapters/herald/mod.rs`,
  `table_herald.rs`, `markdown_herald.rs`, all Dockerfiles, `ci.yml`, `Makefile`, `k8s/*.yaml`,
  `docs/src/deployment/docker.md`.
- Direct reads of `.planning/decisions/{0006,0016,0019,0021}-*.md`, `.planning/ledgers/milestone-04-06.md`
  (rows at lines 115, 116, 157, 160, 225), `.planning/REQUIREMENTS.md:821-937`, `.planning/ROADMAP.md`
  Phase 8 section, `.planning/STATE.md`, `.planning/decisions/PROMOTION.md`.

### Secondary (MEDIUM confidence)
- None — every claim in this document traces to a primary-source command or file read.

### Tertiary (LOW confidence)
- A1, A2 in the Assumptions Log above — scoped and flagged explicitly.

## Metadata

**Confidence breakdown:**
- DEBT-01 (baseline regeneration achievability): HIGH — reproduced end-to-end this session.
- DEBT-02 (deprecation reconciliation): HIGH — exact correction site and open-question disposition found.
- DEBT-03 (doctest measurement): HIGH — the actual measurement, not an estimate.
- DEBT-04 (CLI isolation): HIGH for the technical shape (signatures checked, clap translation
  verified against existing dependency); MEDIUM for the completeness of the "everything that assumes
  paladin builds by default" enumeration (A1/A2 flag the edges of that search).
- DEBT-05 (TokenUsage consolidation): HIGH — every call site and the one glob re-export enumerated
  and checked for collision.

**Research date:** 2026-08-06
**Valid until:** This phase is the first to touch `.rs`/`Cargo.toml`/`.github/workflows/` after three
records-only phases — treat this research as valid until Phase 8's plans are written and executed
(expect days, not weeks); a second `cargo test --offline -p paladin-ports --lib` or `cargo tree`
re-run immediately before executing each plan is cheap insurance against drift.
