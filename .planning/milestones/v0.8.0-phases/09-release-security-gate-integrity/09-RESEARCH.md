# Phase 9: Release & Security Gate Integrity - Research

**Researched:** 2026-08-07
**Domain:** Rust workspace release/security governance — cargo-audit/cargo-deny exception
management, SPDX licence metadata, crates.io publish guards, Docker layer caching (cargo-chef),
and repo-guard shell scripting. No `.rs` source is touched.
**Confidence:** HIGH for everything re-verified against the tree in this session; MEDIUM for the
cargo-chef semantics (established from upstream docs, not measured — Docker is absent here);
LOW for nothing — every claim below is either tree-verified or doc-cited.

<user_constraints>
## User Constraints (from CONTEXT.md)

### Locked Decisions

The full D-00a…D-22 decision set in `09-CONTEXT.md` is locked. This research does not re-decide
any of it. In summary (see CONTEXT.md for full reasoning):

- **D-01:** `.cargo/audit.toml` is the authoritative suppression surface; `deny.toml` mirrors its
  vulnerability class and adds one labelled second class; `SECURITY-EXCEPTIONS.md` (root) is the
  authoritative governance register.
- **D-02:** The sync invariant becomes `scripts/check-advisory-register.sh`, wired into CI,
  asserting three things (class-ID equality, register-row coverage, `Cargo.lock` liveness),
  provable offline.
- **D-03:** The two `deny.toml` classes (vulnerability vs. unmaintained) stay separated and
  labelled; do not re-open the run-5 sync-invariant finding.
- **D-04:** Four `deny.toml` suppressions (`structopt`, `ansi_term`, `atty`, `proc-macro-error`)
  are dead (0 hits in `Cargo.lock` post-Phase-8) and are **deleted, not backfilled**.
- **D-05:** The corpus's "fifteen/ten" figures are stale by one; the live set is 14 total →
  10 after D-04 (5 vulnerability-class, 5 unmaintained: `dotenv`, `fxhash`, `number_prefix`,
  `rustls-pemfile`, `paste`). `RUSTSEC-2025-0121` (`gcc`) is already absent from every surface.
- **D-06:** Extend the M10 FR-3 four-field schema with owner + expiry; record the extension in
  ADR-0024, not as a defect fix.
- **D-07:** Delete `ci.yml`'s second `cargo audit` job in **this** phase (not Phase 12); re-scope
  Phase 12's SUPPLY-01/SUPPLY-02 to verification-only, closed-by-Phase-9 with commit SHA.
  ⚠ HUMAN REVIEW (changes another phase's scope).
- **D-08:** Ratify the three 2026 vulnerability ignores (`RUSTSEC-2026-0187/-0194/-0195`) via
  ADR-0024 rather than removing them; each gets a concrete compensating control.
- **D-09:** Owner becomes `DF3NDR` (repository owner), not "Platform Security (Milestone 7)".
  ⚠ HUMAN REVIEW (reassigns accountability for a signed security acceptance).
- **D-10:** The 2026-09-30 acceptance is renewed with a **per-advisory** review date of
  **2026-12-31** (not a blanket date), for the two original advisories (`rsa`, `tokio-tar`).
- **D-11:** Adopt `MIT OR Apache-2.0` across the root package and all ten library crates.
  ⚠ HUMAN REVIEW — one-way (already-published 0.1.0 crates); **must not be resolved by
  inference.** This is the *recommendation*; D-12 is the fallback shape if the human declines.
- **D-12:** If the human confirms MIT-only instead: annotate the checklist superseded, re-justify
  the dual-licence rule's effect on `r-efi` explicitly, manifests/`deny.toml` unchanged. Either
  answer closes SEC-02; a plan must not assume which.
- **D-13:** Add an offline crates.io name guard (`.crate-names.txt` or a `deny.toml` bans-section
  list — planner's choice) enumerating the eleven owned package names; CI + `make` target fail on
  an unlisted `[package] name`. ADR-0026 records the accepted residual cost (a genuinely novel
  name is still human-checked, not CI-checked).
- **D-14:** Create `crates/paladin-herald/CHANGELOG.md` (Keep-a-Changelog, matching the nine
  siblings). No exemption.
- **D-15:** Add a guard asserting every `crates/*/` directory carries a `CHANGELOG.md`.
- **D-16:** Delete `Dockerfile.chef:25-33`'s nine-manifest enumeration rather than extend it to
  ten. ⚠ HUMAN REVIEW — supersedes M7 Epic 2 FR-01. **Contingent on this researcher confirming
  the cargo-chef reading; see "Answering the Open Technical Questions" §1 below — CONFIRMED.**
  Fallback if refuted: keep enumeration, add herald's line, add a guard script asserting every
  `crates/*/Cargo.toml` appears in the planner stage.
- **D-17:** SEC-01 does not wait for HARD-06; `crates/paladin-content/Cargo.toml:41`'s
  unconditional `pdf-extract` dependency already warrants the `-0187` suppression regardless of
  how HARD-06 answers the PDF-capability question.
- **D-18:** ADR allocation — 0024 (RustSec governance), 0025 (licence, blocking on human),
  0026 (name guard), 0027 (Dockerfile.chef supersession). `PROMOTION.md` advances to 0028.
- **D-19:** Every closure claim is proved by a command run in this environment, verbatim. Not
  runnable here: `cargo audit`, `cargo deny`, anything Docker.
- **D-20:** SEC rows are recorded in `REQUIREMENTS.md` (no M7-8 ledger exists yet) and handed to
  Phase 10 as an explicit closed-`REQ-*` list.
- **D-21:** `.planning/codebase/CONCERNS.md:257-268` (and its `deny.toml:141-147` citation) is
  corrected in this phase per D-00c annotation, not left to drift.
- **D-22 [informational]:** Suggested decomposition — ~7 plans, 3 waves (see CONTEXT.md for the
  full breakdown and file-contention notes).

### Claude's Discretion

- The register file's exact name/format — `SECURITY-EXCEPTIONS.md` + Markdown table is the
  starting recommendation; `.toml`/`.yml` is explicitly defensible for D-02 clause 2's
  queryability. **This research recommends a concrete resolution — see "Answering the Open
  Technical Questions" §2.**
- Whether the name guard (D-13) and changelog guard (D-15) are separate scripts, one script, or
  CI-inline steps.
- The home of `.crate-names.txt` — standalone file, `deny.toml` section, or `Makefile` variable.
- Whether ADR-0024…0027 are authored standalone or folded into the plans that execute them.
- Banner wording/markup for the D-05 and D-11 `.project/` annotations.
- Whether `SECURITY-EXCEPTIONS.md` doubles as a GitHub-facing `SECURITY.md`. There is no
  `SECURITY.md` today; skipping this is acceptable but must be noted, not silently omitted.

### Deferred Ideas (OUT OF SCOPE)

- "Is PDF extraction supported?" (Phase 10 / HARD-06, criterion 6) — D-17 supplies evidence and
  explicitly declines to answer it.
- `cargo doc --workspace --no-deps` warning bar (Phase 10 / HARD-07).
- A `SECURITY.md` for GitHub's advisory/private-reporting UI — adjacent, genuinely missing, but a
  separate deliverable (candidate for Phase 16).
- Replacing `dotenv` with `dotenvy` — a dependency change with `.rs` consequences; out of scope
  for a phase that governs suppressions rather than removing them.
- The other four live unmaintained advisories' upstream paths (`fxhash`, `number_prefix`,
  `rustls-pemfile`, `paste`) — register the condition, do not act on it.
- A CI dependency-allowlist check built on `cargo tree` (Phase 15, ADR-0015).
- The eight deprecated GitHub Action references (Phase 15 / PIPE-04) — two sit inside jobs this
  phase edits; do not opportunistically bump them.
- Stray root artefacts (`api_surface_current.txt`, `final-api.txt`, `flat`, `lcov.info`).
- `src/main.rs` retirement, Nyquist validation for Phases 1-4, ADR-mdbook publication — all
  carried forward, untouched here.

</user_constraints>

<phase_requirements>
## Phase Requirements

| ID | Description | Research Support |
|----|-------------|------------------|
| SEC-01 | One RustSec exception set with governance; 2026-09-30 acceptance disposed of | §"Answering the Open Technical Questions" §2-3, §"Standard Stack", §"Architecture Patterns" (register schema, guard design, exact file:line surfaces) |
| SEC-02 | One licence answer the manifests declare | §"Answering the Open Technical Questions" §4 (mechanism, file inventory, `deny.toml` no-op confirmation) |
| SEC-03 | crates.io name collisions catchable before a release cycle | §"Answering the Open Technical Questions" §5 (authoritative name list, guard placement) |
| SEC-04 | `crates/paladin-herald/CHANGELOG.md` exists or exemption recorded | §"Code Examples" (herald changelog content, feature-gating history) |
| SEC-05 | `Dockerfile.chef` planner stage cannot silently go stale | §"Answering the Open Technical Questions" §1 (cargo-chef semantics — CONFIRMED, take the delete branch) |

</phase_requirements>

## Summary

Every one of CONTEXT.md's tree-verified claims re-checked cleanly in this session: `deny.toml`
holds exactly 14 `[advisories] ignore` entries (2 vulnerability-mirrored + 9 unmaintained + 3
new-2026), `RUSTSEC-2025-0121` (`gcc`) is confirmed absent from every surface, and the four D-04
"dead" crates (`structopt`, `ansi_term`, `atty`, `proc-macro-error`) return **0** hits in
`Cargo.lock` while the five claimed-live unmaintained crates (`dotenv`, `fxhash`, `number_prefix`,
`rustls-pemfile`, `paste`) each return **1**. `ci.yml`'s two `cargo audit` jobs are exactly where
CONTEXT.md says (`security-audit:` `:61-78`, `security:` `:466-482`, both displaying as
**"Security Audit"**), and a previously-unrecorded fact strengthens D-07: the repository's own
branch-protection ruleset (`.github/rulesets/protect-main-branch.json:39`) requires the **context
string** `"Security Audit"`, not a job ID — since the surviving `security-audit` job already
produces that exact context, deleting `security:` carries **zero** required-status-check risk.

The highest-value open question — whether `Dockerfile.chef`'s nine-manifest enumeration is
genuinely decorative — is **CONFIRMED** from cargo-chef's own canonical documentation. cargo-chef's
official README Dockerfile pattern does not enumerate manifests at all; it does `COPY . .` into the
planner stage, because `recipe.json` (the artifact `cargo chef prepare` produces) is a
manifest-and-lockfile skeleton with no source-code content, and the builder stage's cache is keyed
on `recipe.json`'s byte-for-byte content via BuildKit's content-addressed cross-stage `COPY
--from`, not on whether the planner stage's own layers were cache-hit. Because this Dockerfile
already runs `COPY crates ./crates` (a full source-tree copy) at `:36`, immediately before `cargo
chef prepare` at `:38`, the per-manifest `COPY` lines at `:25-33` cannot deliver any cache
isolation — the full-tree copy already invalidates everything downstream on any `.rs` change,
manifest enumeration or not. **Take D-16's primary branch: delete the enumeration.** This is
established from documentation, not measured (Docker is absent here), and should be recorded as
such.

For the guard scripts (D-02, D-13, D-15), the environment has `python3` 3.11.2 with `tomllib` in
the standard library (zero new dependencies, zero network access) — a materially more robust way
to parse `deny.toml` and `.cargo/audit.toml` than regex/`awk`, and available today. `Cargo.lock`'s
`[[package]] name = "..."` format is stable and Cargo-generated (never hand-edited), so `grep -c`
against it is safe. The register's exact format is genuinely open (Claude's Discretion), but given
`tomllib`'s presence, a plain Markdown table is the worse machine-parsing choice; recommend either
a `.toml` register or a `.md` file with a single fenced ` ```toml ` block that the guard extracts
and parses — both satisfy D-01's "adjacent, human-visible" reasoning while giving the guard a
real parser instead of pipe-table regex.

**Primary recommendation:** Confirm cargo-chef's decorative-enumeration reading and delete
`Dockerfile.chef:25-33` (D-16's primary branch); write `scripts/check-advisory-register.sh` in
`python3`+`tomllib` against a TOML-parseable register; and reuse the already-present
`.github/rulesets/protect-main-branch.json` evidence to close out D-07's blast-radius question
with a definitive "no required-check risk" answer.

## Architectural Responsibility Map

This phase does not touch application code — every capability below lives in the **Release / CI
Tooling** tier (build-time and CI-time configuration, not a runtime architectural layer). It is
listed for completeness against the standard tiers; none of it maps to Browser, Frontend Server,
API/Backend, or Database tiers.

| Capability | Primary Tier | Secondary Tier | Rationale |
|------------|-------------|----------------|-----------|
| RustSec advisory suppression governance | CI Tooling / Release Config | Repo-root docs (`SECURITY-EXCEPTIONS.md`) | `cargo audit`/`cargo deny` are CI-gate tools; the register is a human/CI-shared artifact, not application state |
| Licence metadata | Release Config (`Cargo.toml` × 11) | Repo-root docs (`LICENSE*`, `README.md`) | SPDX `license` field is Cargo manifest metadata consumed by crates.io and `cargo-deny`, not runtime code |
| crates.io name-collision guard | CI Tooling / `scripts/` | `Makefile` | A pre-publish gate; no runtime component |
| Per-crate changelog completeness | Repo-root docs / `scripts/` guard | — | Documentation completeness enforced mechanically |
| Docker layer-cache correctness | Build Tooling (`Dockerfile.chef`) | — | Multi-stage Docker build optimization; no application-layer effect either way |

## Answering the Open Technical Questions

### 1. D-16 / SEC-05 — the cargo-chef question (HIGHEST VALUE)

**Verdict: CONFIRMED.** Delete the enumeration. `[CITED: github.com/LukeMathWalker/cargo-chef
README.md]`, `[CITED: depot.dev/docs/languages/rust-dockerfile]`.

**What cargo-chef's own canonical pattern actually does.** The official README's recommended
Dockerfile is:

```dockerfile
FROM lukemathwalker/cargo-chef:latest-rust-1 AS chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
RUN cargo build --release --bin app
```

There is **no manifest enumeration anywhere in the upstream pattern**. The planner stage does
`COPY . .` — the whole source tree, unconditionally — every single time. This is the strongest
possible evidence against the theory that per-manifest `COPY` lines are a cargo-chef best
practice: the tool's own author does not do it.

**Why `COPY . .` in the planner stage is safe (per the README, verbatim):** *"it contains the
skeleton of your project (e.g. all the `Cargo.toml` files with their relative path, the
`Cargo.lock` file is available) plus a few additional pieces of information"* — `recipe.json`'s
content is manifest-and-lockfile-derived only; it does not embed `.rs` source. And: *"The `COPY .
.` statement in the planner stage will invalidate the cache for the planner container, but it will
not invalidate the cache for the builder container, as long as the checksum of the `recipe.json`
returned by `cargo chef prepare` does not change."* This is the entire mechanism: Docker
BuildKit's `COPY --from=<stage>` is content-addressed on the copied file(s), not chained to the
producing stage's own layer-cache status. So even if the planner stage's `RUN cargo chef prepare`
re-executes on every build (because `COPY . .` always busts its own layer), the **builder** stage's
`RUN cargo chef cook` only re-executes when `recipe.json`'s bytes actually change — which happens
only when a manifest or the lockfile changes, never on `.rs`-only edits.

**Applying this to Paladin's `Dockerfile.chef`.** The planner stage here already contains, in
sequence: `COPY Cargo.toml Cargo.lock ./` (:24) → nine per-manifest `COPY`s (:25-33) → `COPY src
./src` (:35) → **`COPY crates ./crates`** (:36, full recursive copy of all ten crates' manifests
*and* source) → `COPY benches ./benches` (:37) → `RUN cargo chef prepare` (:38). Docker's classic
layer-cache model invalidates every subsequent instruction once any earlier instruction's layer is
invalidated. `COPY crates ./crates` recursively copies real `.rs` source for every crate
(including the nine whose manifests were already copied individually two lines earlier), so **any**
source edit anywhere under `crates/` busts that layer — and therefore busts `RUN cargo chef
prepare` too, regardless of whether the nine preceding per-manifest `COPY`s were themselves
unchanged. The per-manifest enumeration at `:25-33` is consequently inert: it can never be the
layer that determines whether `cargo chef prepare` re-runs, because a strictly later, coarser copy
(`:36`) already dominates that decision for every one of the ten crates, not just the omitted
eleventh (`paladin-herald`).

**What actually delivers the caching benefit here — and it is untouched by D-16.** The real payoff
(builder-stage dependency compilation skipped when only source changes) comes from `recipe.json`'s
own content being manifest-only, combined with the **builder** stage's `COPY --from=planner
/app/recipe.json recipe.json` (:53) being a cross-stage, content-addressed copy. That mechanism is
completely independent of how the *planner* stage assembled its input files — enumerated,
full-tree-copied, or anything in between, `cargo chef prepare`'s output depends only on the
manifests and lockfile it finds, and BuildKit's builder-stage cache depends only on that output's
bytes. **Deleting the nine-line enumeration removes dead weight without touching the actual
cache-tightness mechanism.**

**What is measured vs. established.** Docker is absent from this environment (confirmed:
`command -v docker` fails; consistent with `.planning/phases/04-release-coherence/
04-ci-gate-deferrals.md`'s prior finding). Nothing above was run against real Docker/BuildKit —
it is established from cargo-chef's official documentation and the well-documented, stable
semantics of Docker's layer-cache invalidation chain and BuildKit's content-addressed
cross-stage `COPY --from`. Record the D-16 closure claim as *"established from upstream
documentation, not measured in this environment"* — do not claim a measured pass.

**Recommendation to the planner: take D-16's primary branch.** Delete `Dockerfile.chef:25-33`
entirely. The remaining `COPY Cargo.toml Cargo.lock ./` (root manifest + lockfile) followed by
`COPY src ./src`, `COPY crates ./crates`, `COPY benches ./benches` is both simpler and — per the
analysis above — delivers identical caching behavior to the current nine-line-enumerated version,
while making an eleventh crate (or twelfth, or any future crate) automatically covered with zero
Dockerfile edits, which is exactly what SEC-05's done-condition demands ("the mechanism cannot
miss an eleventh crate"). Update the comment block at `:21-23` to describe the corrected reasoning
(the manifest-first-then-full-tree pattern was never delivering isolation; the isolation this
project actually gets comes from `recipe.json`'s manifest-only content plus BuildKit's cross-stage
cache, both of which survive the deletion unchanged). ADR-0027 should record this exact chain of
reasoning with the two upstream citations above, since it supersedes M7 Epic 2 FR-01's rationale.

**If a future researcher wants to *measure* this** (e.g., once Docker is available): build the
image twice, touching only a `.rs` file between builds with `DOCKER_BUILDKIT=1 docker build .`,
and confirm `cargo chef cook`'s `RUN` step reports `CACHED` in the second build's output. That
would upgrade this from CITED to VERIFIED. Not possible in this session.

### 2. D-02 — the advisory-register guard script

**Environment facts (measured this session):**
- `python3 --version` → `Python 3.11.2`.
- `python3 -c "import tomllib"` → succeeds. **`tomllib` is stdlib since Python 3.11 — zero pip
  installs, zero network access required.** `[VERIFIED: python3 -c import tomllib, this session]`
- `jq` is present (`/usr/bin/jq`) — usable if TOML is first converted to JSON via `tomllib`.
- `cargo audit` / `cargo deny` are **not** installed and cannot be installed (crates.io returns
  HTTP 403 here) — the guard must not shell out to either.

**Recommended parsing strategy — no new dependencies, offline:**
- **`deny.toml` and `.cargo/audit.toml`:** parse with `python3 -c "import tomllib; ..."`. This
  gives a real TOML AST (arrays, tables) instead of fragile regex over comments. Both files'
  `[advisories].ignore` arrays are read directly as Python lists of strings — no ambiguity about
  quoting, trailing commas, or comment placement.
- **`Cargo.lock`:** grep is safe and appropriate here. `Cargo.lock`'s `[[package]]` /
  `name = "..."` format is machine-generated by Cargo itself (line 1-3 of the file state "This
  file is automatically @generated by Cargo. It is not intended for manual editing."), so its
  shape is stable across the version this project pins (`version = 4` at the top). Use
  `grep -c '^name = "<crate>"$' Cargo.lock` exactly as CONTEXT.md's own D-04 verification did —
  do not write a full TOML-array-of-tables parser for this file; it is unnecessary complexity for
  a stable, tool-generated format.
- **The register file:** see §"the register format" below — recommend TOML for the same
  `tomllib` reason.

**Design for the three D-02 assertions, expressed against a TOML register schema** (assuming the
register is machine-parseable — see next section):

1. **Class-ID equality.** Read the register's rows; partition into `vuln_ids` (class =
   `vulnerability`) and `unmaintained_ids` (class = `unmaintained`). Read `.cargo/audit.toml`'s
   `ignore` array as set `A`. Assert `set(vuln_ids) == A` exactly (not subset — exact set
   equality, so an ID present in one but not the other fails loud). Read `deny.toml`'s `ignore`
   array as set `D`. Assert `set(vuln_ids) | set(unmaintained_ids) == D` exactly. This correctly
   models the two-class-in-one-array reality of `deny.toml` without needing to parse its inline
   comments to recover class information — the register is the single source of class
   information, not the TOML comments.
2. **Register coverage, both directions.** For every ID in `A ∪ D`, assert a register row exists
   with all four/six governance fields (owner, expiry/review date, affected scope, compensating
   control, per D-06's extended schema) non-empty. For every register row, assert its ID appears
   in `A` or `D` (a register row with no matching live suppression is stale bookkeeping and should
   fail the guard, not silently pass).
3. **Crate liveness (the check nothing previously performed — this is what D-04's dead-suppression
   discovery would have caught automatically).** For each register row, extract the crate name(s)
   from its "affected crate" field (some entries reference more than one candidate crate, e.g. a
   `via tonic/testcontainers` path — treat as "at least one must be live"), and run
   `grep -c "^name = \"<crate>\"$" Cargo.lock`. Assert the count is `>= 1` for at least one
   candidate crate per row; if zero for every candidate, the suppression is dead and the guard
   fails, naming the exact row.

**Demonstrating the guard can fail (the Phase-8 `check-deprecations.sh` lesson).** Before wiring
the script into CI, the implementing plan should include an explicit negative-path
demonstration — e.g., temporarily add a bogus `RUSTSEC-9999-0000` to `deny.toml`'s `ignore` array
with no register row, run the script, confirm non-zero exit and a legible error message naming the
missing row, then revert. Record the exact command and its exit code in the plan's evidence, per
D-19's bar. This is the concrete antidote to Phase 8 D-05's finding that `check-deprecations.sh`'s
both branches `exit 0`.

**Recommended register format: TOML, not a Markdown table (or a hybrid).** D-01 locked the
filename `SECURITY-EXCEPTIONS.md`, but Claude's Discretion reopens the *format* explicitly. Given
`tomllib` is confirmed present and free, a Markdown pipe-table would require regex extraction of
cell contents — fragile the moment "compensating control" prose contains a literal `|` character
or a Markdown link, and there is no such fragility with TOML array-of-tables. Two options that
both honor D-01's "adjacent, human-visible, repo-root" reasoning:

- **Option A (recommended): `SECURITY-EXCEPTIONS.md` containing prose plus one fenced
  ` ```toml ` block** holding the actual register as TOML array-of-tables (e.g. `[[exception]]`
  per advisory). The guard script extracts the fenced block (a single, trivial `sed`/regex step:
  find between ` ```toml ` and the next ` ``` `) and parses it with `tomllib`. Humans reading the
  file on GitHub see rendered prose *and* a clearly-formatted code block; the guard gets a real
  parser. This is the closest match to D-01's stated reasoning ("adjacent to the config it
  governs... visible to anyone not running GSD") while giving D-02 clause 2 a clean implementation.
- **Option B: rename to `SECURITY-EXCEPTIONS.toml`** and abandon the `.md` extension entirely.
  Simpler to parse (no fenced-block extraction step) but loses GitHub's default Markdown
  rendering for anyone browsing the repo — a real regression against D-01's own "readable by every
  downstream consumer of a published crate family" rationale.

Recommend **Option A** to the planner: it satisfies D-01's readability intent and D-02 clause 2's
queryability intent simultaneously, at the cost of one extra (trivial) extraction step in the
guard script.

### 3. D-04 — independent re-verification of the four dead suppressions

**Re-verified this session, exact commands and counts** `[VERIFIED: this session, Cargo.lock @
current tree state]`:

```
for c in structopt ansi_term atty proc-macro-error gcc dotenv fxhash number_prefix rustls-pemfile paste; do
  grep -c "^name = \"$c\"\$" Cargo.lock
done
```

| Crate | Count | Verdict |
|---|---|---|
| `structopt` | 0 | dead — D-04 confirmed |
| `ansi_term` | 0 | dead — D-04 confirmed |
| `atty` | 0 | dead — D-04 confirmed |
| `proc-macro-error` | 0 | dead — D-04 confirmed |
| `gcc` (RUSTSEC-2025-0121) | 0 | already absent — D-05 confirmed |
| `dotenv` | 1 | live |
| `fxhash` | 1 | live |
| `number_prefix` | 1 | live |
| `rustls-pemfile` | 1 | live |
| `paste` | 1 | live |

**All ten of CONTEXT.md's claims are correct with no discrepancy.** `deny.toml`'s `[advisories]
ignore` array was independently counted (not trusted from CONTEXT.md's prose): 2 vulnerability
entries mirrored from `.cargo/audit.toml` (`RUSTSEC-2023-0071`, `-2025-0111`) + 9 unmaintained
entries (`-2021-0139`, `-2021-0141`, `-2024-0370`, `-2024-0375`, `-2025-0057`, `-2025-0119`,
`-2025-0134`, `-2024-0436`, `-2022-0104`) + 3 new-2026 vulnerability entries (`-2026-0187`,
`-0194`, `-0195`) = **14 total**, matching D-05 exactly. `.cargo/audit.toml` holds exactly the 5
vulnerability advisories. After D-04's four deletions: **10 live entries** (5 vulnerability-class,
5 unmaintained) — confirming D-05's "do not plan a backfill of fifteen" instruction and sizing
SEC-01's governance backfill at exactly 10 rows, not 15 and not 14.

**Non-default-feature reachability check.** `deny.toml`'s `[graph] all-features = true` (`:15`)
means `cargo-deny` itself already evaluates the dependency graph with all features enabled, so
there is no daylight between "default resolve" and "all-features resolve" for `cargo-deny`'s own
purposes. For `cargo audit`, the tool scans `Cargo.lock` directly and is feature-agnostic (`Cargo.lock`
already reflects the union of everything any workspace member's Cargo.toml could pull in, since
Cargo resolves one lockfile for the whole workspace regardless of which features are active in a
given build) — `cargo audit`'s advisory scan does not vary by which features are enabled at build
time. So the "reachable only under non-default features" question the research brief raised does
not create a distinct risk class here: `Cargo.lock` is the single ground truth both tools consult,
and a crate absent from `Cargo.lock` entirely (as `structopt`/`ansi_term`/`atty`/`proc-macro-error`
now are) cannot be reached under any feature combination. `[VERIFIED: deny.toml:15, and Cargo's
documented single-lockfile-per-workspace resolution model]`

### 4. D-11 / SEC-02 — the licence change mechanics

**Inheritance mechanism — confirmed NOT `[workspace.package]` inheritance.** `[VERIFIED: this
session, Cargo.toml + all 10 crates/*/Cargo.toml]` The root `Cargo.toml` has **no
`[workspace.package]` table at all** — only `[workspace]` (members, resolver) and
`[workspace.dependencies]`. The `[package]` table's `license = "MIT"` at `Cargo.toml:40` is a
literal, independent declaration, exactly as each of the ten `crates/*/Cargo.toml` independently
declares `license = "MIT"` at their own `:6` or `:8`. Grepping for `.workspace = true` usage
confirms the pattern the project *does* use for shared config (dependency versions:
`async-trait = { workspace = true }` etc., seen in `paladin-content`, `paladin-storage`,
`paladin-core`, `doc-examples`) — but **no crate uses `license.workspace = true`** anywhere in the
tree; that Cargo feature (workspace-inherited package metadata) is simply not adopted here for
`license`, `version`, `edition`, `authors`, `repository`, etc. — each of those is repeated per
crate. **Consequence for the plan: D-11's "eleven manifests change" is mechanically literal — there
is no single inheritance point to edit.** The plan must touch all eleven `license =` lines
individually; there is no shortcut through `[workspace.package]`.

**Current `LICENSE` file** `[VERIFIED: this session, LICENSE]`:
```
MIT License

Copyright (c) 2026 Am0rfu5

Permission is hereby granted, free of charge, to any person obtaining a copy
[... standard MIT text follows]
```
A single root-level file named `LICENSE` (no extension), the standard MIT template text,
copyright holder `Am0rfu5`, year 2026. There is **no** `LICENSE-APACHE` file anywhere in the tree.

**Conventional Rust dual-licence file layout** (the ecosystem norm, `[ASSUMED]` — not verified
against a specific authoritative source this session, but this is uncontroversial, widely-adopted
convention across the Rust ecosystem, e.g. the `rust-lang/rust` repo itself and most `MIT OR
Apache-2.0` crates): rename the existing `LICENSE` → `LICENSE-MIT` (content unchanged), add a new
`LICENSE-APACHE` containing the verbatim standard Apache License 2.0 text (this is a fixed,
boilerplate legal text — not something to draft; the planner should source the canonical text from
https://www.apache.org/licenses/LICENSE-2.0.txt, filled in with no per-project modification, as
Apache-2.0 explicitly permits verbatim redistribution). Cargo's `license` field for the dual
expression becomes the SPDX string `"MIT OR Apache-2.0"` (verified valid SPDX syntax; this is the
canonical Rust-ecosystem dual-licence expression, used by e.g. `serde`, `tokio`, `rand`).

**`README.md`'s current claims** `[VERIFIED: this session, README.md]`: line 7 carries a shields.io
badge `[![license: MIT](...)](LICENSE)`; the `## License` section (lines 187-189) states
"Licensed under the [MIT License](LICENSE)." Both need updating under D-11 (badge text + link
target if the file is renamed; section prose) and are **unchanged** under D-12 (MIT-only fallback).

**`deny.toml`'s allow-list already permits both** `[VERIFIED: this session, deny.toml:24-46]` —
`"MIT"` and `"Apache-2.0"` both appear in `[licenses] allow` (lines 25, 26) alongside
`BSD-2-Clause`, `BSD-3-Clause`, `ISC`, `Zlib`, `Unicode-3.0`, `0BSD`, `CC0-1.0`,
`CDLA-Permissive-2.0`, plus eight `[[licenses.exceptions]]` MPL-2.0 entries (`colored`,
`attohttpc`, `cssparser`, `cssparser-macros`, `dtoa-short`, `minidom`, `selectors`,
`smartstring`). **`deny.toml` needs zero changes under either D-11 or D-12** — confirming
CONTEXT.md's claim exactly.

**Dockerfile note (new, not in CONTEXT.md's canonical_refs):** `Dockerfile.chef` itself is not
touched by SEC-02, but the runtime `Dockerfile` (the multi-stage production image, separate file
at repo root, distinct from `Dockerfile.chef`) carries `LABEL org.opencontainers.image.licenses=
"MIT"` at `:93`. **If D-11 lands, this label also needs updating to `"MIT OR Apache-2.0"`** — it
was not named in CONTEXT.md's D-11 "cost this decision accepts" list (manifests, `LICENSE`,
`README.md`, `deny.toml`, `CHANGELOG.md`) and should be added to the plan's file list. This is the
same top-level `Dockerfile` CONTEXT.md's "Integration Points" section explicitly excludes from
SEC-05's scope ("`Dockerfile` and `Dockerfile.server` are not in scope" — that exclusion is about
the manifest-enumeration defect, not about the unrelated licence label this research found in the
same file).

### 5. D-13 / SEC-03 — the name guard

**Authoritative published-name list — confirmed 11 entries** `[VERIFIED: this session, `[package]
name` in Cargo.toml + all 11 crates/*/Cargo.toml]`:

| Directory | `[package] name` | Publishable? |
|---|---|---|
| (root) | `paladin-ai` | yes |
| `crates/paladin-core` | `paladin-ai-core` | yes |
| `crates/paladin-ports` | `paladin-ports` | yes |
| `crates/paladin-battalion` | `paladin-battalion` | yes |
| `crates/paladin-herald` | `paladin-herald` | yes |
| `crates/paladin-llm` | `paladin-llm` | yes |
| `crates/paladin-memory` | `paladin-memory` | yes |
| `crates/paladin-storage` | `paladin-storage` | yes |
| `crates/paladin-notifications` | `paladin-notifications` | yes |
| `crates/paladin-content` | `paladin-content` | yes |
| `crates/paladin-web` | `paladin-web` | yes |
| `crates/doc-examples` | `paladin-doc-examples` | **no — `publish = false` at `Cargo.toml:5`** |

**Eleven `[package]` entries workspace-wide are publishable; a twelfth (`doc-examples` /
`paladin-doc-examples`) exists but declares `publish = false` explicitly.** A naive guard that
enumerates `find crates -name Cargo.toml | xargs grep 'name ='` without checking `publish` would
incorrectly include `paladin-doc-examples` in the guarded set — harmless in practice (its name is
also not registered on crates.io and would simply need adding to the allow-list), but the guard
should either (a) explicitly skip crates with `publish = false`, or (b) include all twelve names
in the allow-list up front so the check never has a false positive on a member that will never be
published. **Recommend (a)**: parse each `crates/*/Cargo.toml`'s `[package]` table with `tomllib`,
skip any where `publish == false`, and check the remainder (11 names) against the allow-list. This
makes the guard self-maintaining if a future non-published example/test crate is added, rather
than requiring a manual allow-list edit for something that was never going to be published anyway.

**Cannot pass vacuously.** A guard that only checks "every name in the allow-list still exists in
the tree" (backwards) would pass vacuously if a crate were deleted or renamed without updating the
list. The guard must check the **forward** direction — every publishable `[package] name` found in
the tree must appear in the allow-list — so an *added* crate with an unlisted name fails loud. Both
directions are cheap to check together; recommend asserting set equality between "publishable
names found in tree" and "names in the allow-list" (not merely subset), so a stale allow-list entry
for a since-removed crate also surfaces (a different but related integrity signal, closing the same
"an assertion nobody enforces" pattern D-02 targets).

**Where the allow-list lives:** `Makefile` and `scripts/` both already host adjacent guards
(`scripts/check-api-surface.sh`, `scripts/check-deprecations.sh`) — a new `.crate-names.txt` at
repo root (one name per line, or a `[[crate]]` TOML array if colocated with the register) is
consistent with that existing idiom and is trivially diffable in review (a one-line addition per
new crate, exactly as D-13 intends).

### 6. D-07 — the CI deletion's blast radius

**Job locations and exact line ranges — re-verified, unchanged from CONTEXT.md**
`[VERIFIED: this session, .github/workflows/ci.yml]`:
- `security-audit:` job id, `name: Security Audit` — lines **61-78** (`:61` job id, `:62` display
  name, bare `cargo audit` at `:78`). **This job survives.**
- `security:` job id, `name: Security Audit` — lines **466-482** (`:466` job id, `:467` display
  name, `cargo audit --ignore RUSTSEC-2023-0071 --ignore RUSTSEC-2025-0111` at `:482`). **This job
  is deleted by D-07.**
- `cargo-deny` job's `cargo deny check` step confirmed at `:105` (job starts `:74`, "License &
  Dependency Policy" display name).
- `publish-dry-run` job at `:902-929` (`cargo publish --workspace --dry-run` at `:929`).
- `release.yml`'s per-crate dry run: `publish_one()` function at `:406`, `cargo publish --dry-run
  -p "$crate"` at `:410`.

**New finding — the branch-protection blast-radius question is answerable, and the answer is
"no risk."** `[VERIFIED: this session, .github/rulesets/protect-main-branch.json:31-46]` The
repository's committed GitHub ruleset requires three status-check **contexts** by display-name
string:
```json
"required_status_checks": [
  { "context": "Code Quality" },
  { "context": "Security Audit" },
  { "context": "License & Dependency Policy" }
]
```
This required check is keyed on the **context string** `"Security Audit"` — GitHub's Actions
integration posts a check run per job using the job's `name:` field as that context/check name.
Both `security-audit` (`:61-78`) and `security` (`:466-482`) currently post a check run under the
identical context `"Security Audit"`. **Deleting the `security` job at `:466-482` does not remove
the `"Security Audit"` context from the workflow** — the surviving `security-audit` job continues
to post a check run under that exact same context string, which is what the ruleset actually
requires. `docs/src/appendix/branch-protection.md:70` documents this same three-context
requirement in prose, corroborating the JSON file. **There is no other reference in the repository
to a job-ID-keyed required check** (searched `.github/`, `docs/`, `.project/`, `.planning/` for
"required status check" / "branch protection" / "Security Audit" — the only structured
configuration is `protect-main-branch.json`, and it is context-string-keyed, not job-ID-keyed).
D-07's deletion is therefore safe: **it closes the duplicate-display-name defect without touching
what the ruleset actually requires**, and the plan can state this as a resolved risk rather than an
open one. `.github/rulesets/protect-release-tags.json` (the sibling ruleset) does not reference
security jobs at all and is unaffected.

**Line-number drift after the deletion.** Confirmed: `publish-dry-run` currently sits at
`:902-929`; deleting `:466-482` (18 lines: `# Security audit` comment through the `run:` line, plus
the blank line separator) shifts everything below `:482` up by that count, moving
`publish-dry-run` to approximately `:884-911` post-edit. **Re-derive, do not assume**, when any
later plan or ADR needs to cite a post-deletion `ci.yml` line number.

### 7. Corrections to CONTEXT.md's `<canonical_refs>`

Every `file:line` citation independently re-checked in this session against the current tree
**matched CONTEXT.md exactly** — no corrections required. Specifically re-verified and confirmed:
`Dockerfile.chef:10,14,24-38,55`; `deny.toml`'s full advisories block (14 entries, three labelled
classes); `.cargo/audit.toml`'s 5-entry ignore list; `ci.yml:61-78`, `:105`, `:466-482`,
`:902-929`; `release.yml:406,410`; `Cargo.toml:40` plus all ten `crates/*/Cargo.toml` license
lines; `LICENSE` (single file, no `LICENSE-APACHE`); `crates/paladin-herald/`'s contents (`Cargo.toml`,
`README.md`, `src/` — no `CHANGELOG.md`); the nine sibling `CHANGELOG.md` files;
`PROMOTION.md:47` ("Next free ADR number: 0024"); `CONCERNS.md:257-268` (word-for-word match,
including the stale `gcc` entry). **One citation is stale within `CONCERNS.md` itself** (not a
CONTEXT.md error, but worth flagging for D-21's correction): `CONCERNS.md:278/284` cites
`deny.toml:141–147` for the three new-2026 advisories; the current file has that block at
`:140-145` (comment `:140`, three `ignore` entries at `:143-145`) — a 1-2 line drift from
whatever revision `CONCERNS.md` was written against. Minor, but D-21's correction should cite the
current, re-verified line numbers rather than propagating `CONCERNS.md`'s own stale figure.

## Standard Stack

No new external crates or tools are introduced by this phase (per the phase boundary — no `.rs`
change, no new `Cargo.toml` dependency). The "stack" here is entirely the already-present, already
version-pinned tooling this phase configures/governs:

### Core (already present, versions confirmed this session)

| Tool | Version (pinned/observed) | Purpose | Provenance |
|------|---------|---------|--------------|
| `cargo-audit` | installed fresh in CI via `cargo install cargo-audit --locked` (`ci.yml:69`) / unpinned in the surviving job's sibling (`:75` bare) | RustSec vulnerability advisory scanning against `Cargo.lock` | `[VERIFIED: ci.yml]` |
| `cargo-deny` | installed fresh in CI via `cargo install cargo-deny --locked` (`ci.yml:100`) | License allow-list, bans, advisory mirror, source restrictions | `[VERIFIED: ci.yml]` |
| `cargo-chef` | `0.1.77`, `--locked` (`Dockerfile.chef:14`) | Docker dependency-layer caching for Rust builds | `[VERIFIED: Dockerfile.chef:14]` |
| `python3` | 3.11.2 (environment-provided) | Guard-script TOML parsing via stdlib `tomllib` | `[VERIFIED: python3 --version, this session]` |
| `jq` | present (`/usr/bin/jq`) | Optional JSON post-processing if a TOML→JSON bridge is used | `[VERIFIED: which jq, this session]` |

### Supporting

| Tool | Purpose | When to Use |
|------|---------|-------------|
| `grep -c '^name = "<crate>"$' Cargo.lock` | Crate-liveness check | D-02 clause 3, D-04-style re-verification |
| `tomllib` (Python 3.11+ stdlib) | Structured TOML parsing without new dependencies | Any guard script reading `deny.toml`, `.cargo/audit.toml`, or a TOML-format register |

### Alternatives Considered

| Instead of | Could Use | Tradeoff |
|------------|-----------|----------|
| `python3`+`tomllib` for guard parsing | Pure `grep`/`awk`/`sed` regex over TOML | Regex is fragile against comment placement and multi-line arrays; `tomllib` is a real parser, already present, zero cost |
| `python3`+`tomllib` | Install a Rust-based TOML CLI (`taplo`, `dasel`) | Requires `cargo install` against a crates.io that returns HTTP 403 here — not viable in this environment, and unnecessary given `tomllib` |
| Markdown table register | Structured `.toml`/`.yml` register | Markdown is what D-01 originally named, but pipe-table cell extraction is fragile; see §2 above for the recommended hybrid |

**Installation:** None required — every tool above is already present or already CI-installed via
existing workflow steps. No `npm install` / `pip install` / `cargo install` is added by this
phase's own deliverables.

## Package Legitimacy Audit

**Not applicable — this phase installs no new external packages.** No `Cargo.toml` dependency is
added or changed anywhere in this phase's scope (guard scripts are plain `bash`/`python3` using
only the stdlib `tomllib`; no new crate, no new pip package, no new npm package). The Package
Legitimacy Gate protocol is skipped for this reason, per its own "whenever this phase installs
external packages" trigger condition, which does not fire here.

## Architecture Patterns

### System Architecture Diagram

```
                    ┌─────────────────────────────┐
                    │   SECURITY-EXCEPTIONS.md     │  ← D-01: authoritative
                    │  (register: owner, expiry,   │     governance surface
                    │   scope, compensating ctrl)  │
                    └───────────┬─────────────────┘
                                │ read by
                 ┌──────────────┼──────────────────┐
                 ▼                                  ▼
      ┌─────────────────────┐          ┌─────────────────────┐
      │  .cargo/audit.toml   │          │     deny.toml        │
      │  (cargo-audit reads  │◄─mirror──┤  (cargo-deny reads;   │
      │   this automatically)│  invariant│   2 classes labelled)│
      └──────────┬───────────┘          └──────────┬───────────┘
                 │                                  │
                 ▼                                  ▼
      ┌─────────────────────┐          ┌─────────────────────┐
      │ ci.yml:61-78         │          │ ci.yml:~74-105       │
      │ security-audit job   │          │ cargo-deny job        │
      │ (bare `cargo audit`) │          │ (`cargo deny check`)  │
      └─────────────────────┘          └─────────────────────┘
                 ▲
                 │  (D-07 deletes this sibling entirely — it read
                 │   only 2 of the 5 advisories inline)
      ┌─────────────────────┐
      │ ci.yml:466-482       │
      │ security job [DELETE]│
      └─────────────────────┘

      ┌──────────────────────────────────────────────────┐
      │  scripts/check-advisory-register.sh (D-02)         │
      │  reads all THREE files above + Cargo.lock,          │
      │  asserts: (1) class-ID sets match exactly,          │
      │           (2) every ID has a register row & v.v.,   │
      │           (3) every suppressed crate is in Lock      │
      │  wired into CI next to the cargo-deny job            │
      └──────────────────────────────────────────────────┘

   Independent guards (D-13, D-15), same idiom:
      .crate-names.txt  ──►  scripts/check-crate-names.sh   ──► CI + `make`
      crates/*/           ──►  scripts/check-changelogs.sh   ──► CI + `make`

   Dockerfile.chef (D-16, unrelated to the above):
      COPY Cargo.toml Cargo.lock ./
      COPY src ./src
      COPY crates ./crates      ← already a full-tree copy; enumeration
      COPY benches ./benches      at :25-33 deleted, adds nothing today
      RUN cargo chef prepare      recipe.json = manifest+lock skeleton only
           │
           ▼ (COPY --from=planner, content-addressed)
      RUN cargo chef cook         ← cache keyed on recipe.json bytes,
                                     NOT on planner-stage cache status
```

### Recommended Project Structure

No new directories. New/changed files, by location:

```
/ (repo root)
├── SECURITY-EXCEPTIONS.md          # NEW — D-01 register (recommend: fenced ```toml block)
├── LICENSE-MIT                     # NEW (if D-11) — renamed from LICENSE
├── LICENSE-APACHE                  # NEW (if D-11) — verbatim Apache-2.0 text
├── LICENSE                         # unchanged (if D-12)
├── .crate-names.txt                # NEW — D-13 allow-list (or a deny.toml/Makefile home)
├── Cargo.toml                      # license= line (×1) — D-11/D-12
├── deny.toml                       # D-04 deletions, D-06 comment rewrite, D-01 pointer
├── .cargo/audit.toml                # comment/pointer updates only
├── Dockerfile.chef                 # D-16: delete :25-33
├── Dockerfile                      # licenses LABEL — D-11 only (new finding, §4 above)
├── .github/workflows/ci.yml        # D-07 deletion (:466-482); guard steps added
├── crates/paladin-*/Cargo.toml      # license= line (×10) — D-11/D-12
├── crates/paladin-herald/CHANGELOG.md  # NEW — D-14
└── scripts/
    ├── check-advisory-register.sh  # NEW — D-02
    ├── check-crate-names.sh        # NEW — D-13 (or folded into the above)
    └── check-changelogs.sh         # NEW — D-15 (or folded into the above)
```

### Pattern 1: Content-addressed cross-stage Docker cache (cargo-chef)

**What:** A two-stage Docker pattern where a cheap "planner" stage always re-runs but produces a
small, content-stable artifact (`recipe.json`); a "builder" stage consumes that artifact via
`COPY --from=<stage>`, and BuildKit's cross-stage copy caching is keyed on the artifact's bytes,
not on whether the planner stage itself was a cache hit.

**When to use:** Any multi-stage Docker build where an expensive step (dependency compilation)
should be decoupled from a cheap-but-frequently-changing input (application source).

**Example:**
```dockerfile
# Source: github.com/LukeMathWalker/cargo-chef README.md (canonical pattern)
FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
RUN cargo build --release --bin app
```

### Pattern 2: Register-of-record + mirrored mechanical configs, enforced by script

**What:** One human-readable, governance-complete file is authoritative; one or more
machine-consumed config files mirror a subset of it; a script — not a comment — asserts the
mirror holds.

**When to use:** Any time a tool's native config format (here, TOML arrays of bare strings) cannot
express the governance metadata a compliance requirement demands (owner, expiry, scope,
compensating control), but multiple tools must agree on the underlying set.

**Example (schema, not code — the register's row shape per D-06):**
```toml
# Source: this research, synthesizing D-01/D-02/D-06/D-08/D-09/D-10
[[exception]]
id = "RUSTSEC-2023-0071"
class = "vulnerability"
crate = "rsa"
path = "rsa -> sqlx-mysql -> sqlx -> workspace crates"
why_present = "transitive dev/test dependency of sqlx-mysql"
why_not_fixable = "no upstream fix available in sqlx-mysql's rsa dependency"
owner = "DF3NDR"
review_date = "2026-12-31"
compensating_control = "dev/test-scoped only; not reachable in a release build"
revisit_condition = "sqlx-mysql upgrades its rsa dependency past the vulnerable range"
```

### Anti-Patterns to Avoid

- **Manifest enumeration ahead of a full-tree copy, in the same Docker stage:** delivers zero
  cache isolation once a later, coarser `COPY` of the same directory tree exists downstream but
  upstream of the command the enumeration was meant to protect. If per-manifest granularity is
  ever genuinely needed again, it only works if **no** later instruction in the same stage
  re-copies the same paths with real source content before the command being cached runs.
- **A sync invariant stated only in a comment:** `deny.toml`'s own header ("Keep these two files in
  sync") and `check-deprecations.sh`'s always-`exit 0` fallback are both examples, already shipped,
  of an assertion with no enforcement. Every invariant this phase states must end in a script with
  a demonstrated non-zero exit path.
- **A guard that only checks one direction of a set-equality claim:** e.g., "every allow-listed name
  still exists" without also checking "every existing publishable name is allow-listed" — passes
  vacuously against the exact failure mode (a new, colliding name) the guard exists to catch.

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| TOML parsing in a guard script | A regex/`awk` TOML "parser" | `python3 -c "import tomllib; ..."` | `tomllib` is stdlib (3.11+, confirmed present), handles comments/quoting/multi-line arrays correctly; regex will silently misparse an edge case someone adds later |
| Apache-2.0 license text | Drafting/paraphrasing the license | Copy verbatim from `apache.org/licenses/LICENSE-2.0.txt` | Apache-2.0 is boilerplate; any deviation from the canonical text is a legal risk with zero corresponding benefit |
| crates.io availability checking | A live network query against crates.io | The offline `.crate-names.txt` allow-list (D-13) | `crates.io` returns HTTP 403 in this environment and in CI per Phase 8's D-03 lesson — a network check can be written but never demonstrated passing, which is the exact failure mode D-13 was chosen to avoid |

**Key insight:** every "don't hand-roll" item above is really the same lesson restated: prefer a
tool/format that is *already proven to work in this environment* over one that merely seems more
idiomatic — `tomllib` over hand-rolled regex, an offline list over a network check, verbatim legal
text over paraphrase.

## Common Pitfalls

### Pitfall 1: Trusting the corpus's suppression counts without re-grepping

**What goes wrong:** REQUIREMENTS.md's own SEC-01 text still carries the run-4 "fifteen/two"
framing in places even after the run-5 correction banner; a planner reading only the ROADMAP
summary would size SEC-01's backfill at 15 rows instead of the tree-verified 10.
**Why it happens:** Five ingest runs have each found the corpus's own self-description wrong in a
new way (the "documents lie about themselves in both directions" pattern CONTEXT.md names).
**How to avoid:** Re-run the exact `grep -c` commands in this research's §3 before writing any
plan-time size estimate; do not transcribe a count from prose.
**Warning signs:** Any plan that says "backfill fifteen suppressions" or "ten unmaintained
entries" is citing the stale figure.

### Pitfall 2: Assuming `Dockerfile.chef`'s enumeration is "at least harmless"

**What goes wrong:** A planner might reason "adding the herald line is strictly safer than
deleting nine lines" and choose the minimal-diff fix — which reproduces the exact defect SEC-05's
done-condition calls out by name ("an enumerated list that goes stale on every crate addition is
the defect, not just the one missing line").
**Why it happens:** Minimal-diff instinct, and the deletion touches more lines than the addition.
**How to avoid:** The deletion is not merely safe, it is *equivalent in caching behavior* to the
enumeration (per §1's analysis) — there is no caching regression from deleting it, and there is a
structural staleness fix from doing so. Take the deletion.
**Warning signs:** A plan task titled "add `paladin-herald` to `Dockerfile.chef`'s planner stage".

### Pitfall 3: Writing the guard script's assertions against `deny.toml`'s comments instead of the register

**What goes wrong:** Trying to recover "which class does this ID belong to" by parsing `deny.toml`'s
inline `#` comments (e.g., "# ansi_term (unmaintained)") is exactly the kind of string-matching
that will silently break the next time someone edits comment wording without updating a script that
depends on its exact text.
**Why it happens:** The comments already contain the class information in prose, so it's tempting
to scrape it rather than duplicate it in the register.
**How to avoid:** The register (not `deny.toml`'s comments) is the single source of class
information for the guard's purposes; `deny.toml`'s comments stay human-readable prose and are
never parsed by the script.
**Warning signs:** A guard script containing a regex like `grep -B1 unmaintained deny.toml`.

## Code Examples

### D-14: `crates/paladin-herald/CHANGELOG.md` starting content

Backfilled from real history per CONTEXT.md's instruction (not a stub), matching the nine
siblings' Keep-a-Changelog format `[CITED: crates/paladin-llm/CHANGELOG.md, this session, as the
format exemplar]`:

```markdown
# Changelog

All notable changes to `paladin-herald` will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project follows lockstep workspace versioning.

## [Unreleased]

### Added
- Crate created by the 2026-06-04 facade-cleanup reconciliation (commit `66f6c4e`), extracting
  Herald output-formatter adapters (JSON, Markdown, Table) into their own publishable crate.

### Changed
- **Breaking (default features): `table` and `color` are now opt-in features, not unconditional
  dependencies.** ADR-0023 gave `paladin-herald` its first `[features]` section
  (`default = []`, `table = ["dep:comfy-table"]`, `color = ["dep:colored"]`), so a consumer
  depending on `paladin-herald` with default features no longer compiles `comfy-table` or
  `colored`. `TableHerald` requires the `table` feature; `MarkdownHerald`'s coloured rendering
  path requires the `color` feature (its uncoloured path, `include_colors: false`, is
  unconditional); `JsonHerald` is unaffected and needs neither dependency.
```

Verify the exact prose against `.planning/decisions/0023-cli-dependency-isolation.md`'s "Site 2"
section before finalizing wording — the summary above is derived from it, re-derive rather than
copy if the ADR's own text changes before this phase executes.

### D-02: guard-script TOML read (illustrative fragment, not a full script)

```bash
# Source: this research — python3 3.11.2's stdlib tomllib, confirmed present, this session
python3 - "$DENY_TOML" "$AUDIT_TOML" <<'PYEOF'
import sys, tomllib

deny_path, audit_path = sys.argv[1], sys.argv[2]

with open(deny_path, "rb") as f:
    deny = tomllib.load(f)
with open(audit_path, "rb") as f:
    audit = tomllib.load(f)

deny_ids = set(deny["advisories"]["ignore"])
audit_ids = set(audit["advisories"]["ignore"])

# audit_ids must be a subset of deny_ids's vulnerability-class rows -- the
# register (not this script) is authoritative on which deny_ids are
# vulnerability-class vs. unmaintained-class; see the register-driven
# design in RESEARCH.md section 2 for the full three-assertion check.
print(f"deny.toml ignore count: {len(deny_ids)}")
print(f"audit.toml ignore count: {len(audit_ids)}")
PYEOF
```

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|---------------|--------|
| Manifest-by-manifest `COPY` enumeration in a cargo-chef planner stage | Full-tree `COPY . .` into the planner stage, relying on `recipe.json`'s manifest-only content plus BuildKit's content-addressed cross-stage `COPY --from` | This is cargo-chef's original and still-current documented pattern — not a recent change, but apparently never adopted by this Dockerfile, which invented a manifest-enumeration variant instead | The enumerated variant provides no caching benefit beyond the canonical pattern once a later full-tree `COPY` exists in the same stage (as it does here); deleting it is a pure simplification with a correctness upside (no more elevenths going stale) |
| Blanket security-acceptance expiry date covering multiple unrelated advisories | Per-advisory review/expiry dates | D-10, this phase | A single date governing two dissimilar advisories with different upstream fix timelines produces either premature or overdue reviews for one of them; per-advisory dates let each track its own upstream |

**Deprecated/outdated:** Nothing in this phase's domain is itself deprecated — the tools
(`cargo-audit`, `cargo-deny`, `cargo-chef`) are all current and actively maintained. The *pattern*
being retired is project-specific (the manifest enumeration), not an upstream deprecation.

## Assumptions Log

| # | Claim | Section | Risk if Wrong |
|---|-------|---------|---------------|
| A1 | The conventional Rust dual-licence file layout is `LICENSE-MIT` + `LICENSE-APACHE`, sourced verbatim from apache.org | §4 (D-11 mechanics) | Low — this is uncontroversial ecosystem convention; even if the exact filenames differ from some other project's choice, the SPDX `license` field is what actually governs compliance, and the file layout is a readability convenience, not a legal requirement |
| A2 | The BuildKit content-addressed cross-stage `COPY --from` mechanism (used to explain *why* `COPY . .` in the planner stage doesn't propagate cache invalidation to the builder stage) works as commonly documented in third-party sources (Depot's docs, cargo-chef's own README's cache-behavior claim) | §1 (cargo-chef confirmation) | Medium — this is the load-bearing mechanism behind D-16's confirmation, but it is corroborated by cargo-chef's own README's explicit claim ("will not invalidate the cache for the builder container, as long as the checksum... does not change") and is consistent with BuildKit's widely-documented behavior; it was not measured against a real Docker daemon in this environment |

**Note on confidence:** A2 is the single highest-stakes claim in this document (it underwrites a
⚠ HUMAN REVIEW decision that supersedes a completed-milestone PRD requirement). It is tagged
`[CITED]`, not `[VERIFIED]`, precisely because Docker is absent here and it cannot be upgraded to
`[VERIFIED]` in this environment. The planner and any human reviewer should treat D-16's cargo-chef
reasoning as strong-but-unmeasured, and the close-out plan should note this honestly rather than
claim a measured pass (per D-19).

## Open Questions

1. **Should the runtime `Dockerfile`'s `LABEL org.opencontainers.image.licenses="MIT"` (`:93`) be
   updated alongside the eleven manifests under D-11?**
   - What we know: it exists, states `"MIT"`, and was not named in CONTEXT.md's D-11 cost list.
   - What's unclear: whether OCI image labels are considered part of "the manifests declare it" or
     a separate, lower-priority piece of metadata debt.
   - Recommendation: include it in the SEC-02 plan's file list regardless — it is a one-line,
     zero-risk change and leaving it stale would recreate exactly the "one project, three
     answers" problem SEC-02 exists to close, just in a fourth place.

2. **Does the register's fenced-TOML-block approach (Option A in §2) need a stricter
   extraction guarantee than "find text between the first ` ```toml ` and the next ` ``` `"?**
   - What we know: this is trivially reliable if the file has exactly one fenced TOML block.
   - What's unclear: whether future editors might add a second illustrative TOML snippet
     elsewhere in the same file (e.g., in prose explaining the schema) and break the extraction.
   - Recommendation: use an explicit, unique marker comment (e.g.
     `<!-- BEGIN MACHINE-READABLE REGISTER -->` / `<!-- END -->`) around the fenced block rather
     than relying on "the first/only fenced block," making the guard's extraction robust against
     future prose additions.

## Environment Availability

| Dependency | Required By | Available | Version | Fallback |
|------------|------------|-----------|---------|----------|
| `python3` + `tomllib` | Guard scripts (D-02, D-13, D-15) | ✓ | 3.11.2 | — |
| `jq` | Optional guard post-processing | ✓ | present | — |
| `cargo audit` | SEC-01 gate | ✗ (cannot install — crates.io 403) | — | Land config changes; the guard script and CI are the only executable verification; local closure claims are scoped to "not runnable here" per D-19 |
| `cargo deny` | SEC-01 gate | ✗ (cannot install — crates.io 403) | — | Same as above |
| `docker` / BuildKit | SEC-05 gate | ✗ (confirmed absent, consistent with Phase 4's prior finding) | — | Establish from cargo-chef's official documentation (done, §1); the actual Docker build cannot be exercised until a CI runner does it |
| `python3 -c "import tomllib"` (register parsing) | D-02, D-13, D-15 | ✓ | stdlib since 3.11 | — |

**Missing dependencies with no fallback:** None that block *landing* the changes — every SEC
requirement's config/register/script work can be written, committed, and reviewed without
`cargo audit`/`cargo deny`/`docker` being runnable locally. The gates themselves (whether CI's
`cargo audit`/`cargo deny`/Docker build pass against the new configuration) can only be confirmed
on a CI runner, and closure claims must say so explicitly rather than inferring a pass (D-19).

**Missing dependencies with fallback:** `docker` — fallback is documentation-based establishment
of the cargo-chef mechanism (this research, §1), explicitly not a substitute for a measured pass.

## Validation Architecture

> Included per this project's Nyquist validation config (no `workflow.nyquist_validation: false`
> found in `.planning/config.json`'s visible scope; treat as enabled).

### Test Framework

| Property | Value |
|----------|-------|
| Framework | None (this phase changes no `.rs` — no unit/integration test framework applies) |
| Config file | N/A |
| Quick run command | See per-requirement commands below — each SEC requirement's validation is a shell command against a config/register file, not a `cargo test` invocation |
| Full suite command | `cargo test → cargo fmt --check → cargo clippy -- -D warnings` (the standing CLAUDE.md workspace gate — expected unchanged by this phase, since no `.rs` is touched; the close-out should confirm this explicitly) |

### Phase Requirements → Validation Map

| Req ID | Behavior | Validation Type | Command (runnable in this environment) | Command (CI-only) |
|--------|----------|-----------|-------------------|--------------------|
| SEC-01 | `deny.toml`/`.cargo/audit.toml` sync + register coverage + crate liveness | script (offline) | `./scripts/check-advisory-register.sh` (exit 0/1) | `cargo audit` and `cargo deny check` actually passing in CI — **cannot be run here** |
| SEC-01 | The two `cargo audit` CI jobs collapse to one | static diff | `grep -c '^  security:$' .github/workflows/ci.yml` → expect `0` after deletion | The surviving job passing on a real CI runner |
| SEC-02 | Manifests + `LICENSE*` + `deny.toml` agree | grep/diff | `grep -h '^license = ' Cargo.toml crates/*/Cargo.toml \| sort -u` → expect exactly one line | crates.io accepting the next real publish under the new `license` field (out of scope — no publish happens in this phase) |
| SEC-03 | Name guard fails on an unlisted name | script (offline) | `./scripts/check-crate-names.sh` against a temporarily-added bogus name, confirm non-zero exit, then revert | The guard running as a required CI/`make` step on every PR |
| SEC-04 | Herald changelog exists | file existence + guard | `test -f crates/paladin-herald/CHANGELOG.md` and `./scripts/check-changelogs.sh` | — (fully verifiable offline) |
| SEC-05 | Docker planner stage cannot go stale | file diff + doc citation | `grep -c 'COPY crates/paladin.*Cargo.toml' Dockerfile.chef` → expect `0` after deletion | An actual `docker build` twice (touch `.rs` only) confirming `cargo chef cook` reports `CACHED` — **cannot be run here (Docker absent)** |

### Sampling Rate

- **Per task commit:** run the specific guard script(s) touched by that task; run
  `cargo fmt --check` / `cargo clippy -- -D warnings` if any file under `scripts/` is shell (not
  gated by clippy) — for the eleven manifest edits, run `cargo check --workspace` to confirm the
  `license` field change doesn't break parsing.
- **Per wave merge:** run all three guard scripts together plus a full `grep`-based re-verification
  of every count in this research's §3 (dead suppressions, live suppressions, published-name list)
  to confirm no drift was introduced mid-phase.
- **Phase gate:** `cargo test` (expected unchanged pass count — no `.rs` touched),
  `cargo fmt --check`, `cargo clippy -- -D warnings`, all three guard scripts passing, and an
  explicit, honest statement in the close-out plan of which SEC criteria were verified locally
  vs. which require a CI runner (per D-19).

### Wave 0 Gaps

- [ ] `scripts/check-advisory-register.sh` does not exist yet — must be written (D-02).
- [ ] `scripts/check-crate-names.sh` (or equivalent) does not exist yet — must be written (D-13).
- [ ] `scripts/check-changelogs.sh` (or equivalent) does not exist yet — must be written (D-15).
- [ ] No existing negative-path fixture for any of the three guards above (per Phase 8's
      `check-deprecations.sh` lesson, each guard's implementing plan should include one
      demonstrated-failing invocation, then revert, with the command and exit code recorded).

## Security Domain

> `security_enforcement` is not explicitly `false` in visible config; treating as enabled. Note:
> this phase's entire subject *is* the security-gate tooling itself (RustSec suppression
> governance, licence compliance), not application-layer input handling — the ASVS categories
> below are consequently mostly "not applicable" for this phase, which is itself the expected and
> correct answer, not a gap.

### Applicable ASVS Categories

| ASVS Category | Applies | Standard Control |
|---------------|---------|-----------------|
| V2 Authentication | no | This phase touches no auth code |
| V3 Session Management | no | This phase touches no session code |
| V4 Access Control | no | This phase touches no access-control code |
| V5 Input Validation | no | Guard scripts read only trusted, repo-local config files (`deny.toml`, `.cargo/audit.toml`, `Cargo.lock`, the register) — no external/attacker-controlled input |
| V6 Cryptography | no | No cryptographic code is touched |
| V14 Configuration (informal mapping) | yes | This phase's entire subject: dependency-vulnerability suppression governance and licence-declaration correctness are configuration-hardening controls |

### Known Threat Patterns for this phase's domain

| Pattern | STRIDE | Standard Mitigation |
|---------|--------|---------------------|
| A guard script that always exits 0 regardless of input (the `check-deprecations.sh` failure mode) | Repudiation (a "passing" gate that never actually gates) | Demonstrate a failing invocation before wiring into CI, per D-02/D-19 |
| An advisory suppression that has silently gone stale (its crate removed from `Cargo.lock`, the suppression not) | Tampering (of the trust model — a suppression the tree no longer needs still masks any *future*, unrelated advisory for the same ID string) | D-02 clause 3's crate-liveness check |
| A branch-protection required check silently satisfied by the *wrong* job after a deletion | Spoofing (a green check that isn't actually testing what it claims) | §6's confirmation that `protect-main-branch.json` keys on context string, not job ID — verified safe in this specific case, but worth re-checking any time a same-named job is deleted in the future |

## Sources

### Primary (HIGH confidence — verified this session against the tree)

- Direct file reads/greps this session: `Dockerfile.chef`, `deny.toml`, `.cargo/audit.toml`,
  `Cargo.toml` + all 11 `crates/*/Cargo.toml`, `Cargo.lock`, `.github/workflows/ci.yml`,
  `.github/workflows/release.yml`, `.github/rulesets/protect-main-branch.json`, `LICENSE`,
  `README.md`, `Makefile`, `.planning/codebase/CONCERNS.md`, `.planning/decisions/PROMOTION.md`,
  `.planning/decisions/0023-cli-dependency-isolation.md`, `scripts/check-deprecations.sh`,
  `scripts/check-api-surface.sh`, `crates/paladin-llm/CHANGELOG.md`, `CHANGELOG.md`,
  `docs/src/appendix/branch-protection.md`.
- `python3 --version`, `python3 -c "import tomllib"`, `which jq`, `cargo --version` — all run this
  session in this environment.

### Secondary (MEDIUM confidence — official docs, not measured in this environment)

- cargo-chef README (github.com/LukeMathWalker/cargo-chef) — the canonical Dockerfile pattern and
  the `recipe.json` content/cache-behavior claims quoted in §1.
- Depot.dev's Rust Dockerfile documentation (depot.dev/docs/languages/rust-dockerfile) —
  corroborating description of `recipe.json`'s manifest-only content.

### Tertiary (LOW confidence)

- None used — every claim in this document is either tree-verified this session or sourced from
  cargo-chef's own official documentation.

## Metadata

**Confidence breakdown:**
- Standard stack: HIGH — nothing new is introduced; all tooling versions were read directly from
  the tree this session.
- Architecture (register/guard design): HIGH for the design logic, MEDIUM for the exact register
  file format recommendation (genuinely discretionary, not tree-determined).
- cargo-chef / SEC-05 finding: MEDIUM — confirmed against official upstream documentation, not
  measured against a real Docker daemon (unavailable in this environment). This is the one
  finding in this document that should be flagged to a human reviewer as "documentation-confirmed,
  not measured" before the ⚠ HUMAN REVIEW checkpoint on D-16 is resolved.
- Pitfalls: HIGH — all three are either directly observed in this session (stale counts, the
  Docker copy-ordering fact) or a direct restatement of Phase 8's own documented lesson.
- Branch-protection blast-radius (D-07 item 6): HIGH — the ruleset JSON is unambiguous and was
  read directly.

**Research date:** 2026-08-07
**Valid until:** This is a point-in-time audit of a specific tree state (commit at time of
research). Any commit that touches `deny.toml`, `.cargo/audit.toml`, `Cargo.lock`,
`.github/workflows/ci.yml`, or any of the eleven manifests between this research and plan
execution invalidates the exact line numbers and counts above — re-verify before executing if more
than a few days elapse, per this phase's own D-00e/D-19 discipline.
