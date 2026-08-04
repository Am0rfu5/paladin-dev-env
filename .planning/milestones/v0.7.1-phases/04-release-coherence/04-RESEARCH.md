# Phase 4: Release Coherence — Research

**Researched:** 2026-08-02
**Phase:** 4-release-coherence
**Requirements:** REL-01, REL-02, REL-03, REL-04, REL-05
**Consumed by:** gsd-planner (via `04-CONTEXT.md`'s 17 locked decisions D-01..D-17)

> Produced by two parallel `gsd-phase-researcher` runs, merged verbatim. Part A covers the
> manifest and release-mechanics half (D-01..D-09, D-17); Part B covers CI gates, examples,
> the QUICKSTART measurement, and the Validation Architecture (D-10..D-17).
> The per-part source files `04-RESEARCH-A.md` and `04-RESEARCH-B.md` are retained alongside
> this merge.

---

## Headline findings the planner must not miss

1. **`cargo-release 1.1.2` IS installed** — a non-mutating dry-run proves `cargo release version
   0.7.0 --workspace` rewrites all eleven `[package] version` fields *and* every internal
   path-pin including the exact pin at `crates/paladin-ports/Cargo.toml:21`. The
   hand-edit-eleven-manifests fallback is not needed.
2. **`make release` is unsafe to invoke this phase**, for two independent reasons: its branch
   guard (`Makefile:456-466`) requires `main` and we are on `release/v0.7.0`; and even bypassed,
   `Makefile:484-485` runs raw `git push` calls that `release.toml`'s `publish = false` /
   `push = false` do **not** protect against — those settings govern cargo-release's own logic,
   not the Makefile's hand-written push lines. This confirms D-03's human gate.
3. **The edition-2024 migration is low-risk.** Four of seven textbook hazards are provably absent
   by grep across all 38 files; both crates use `async_trait` (57 occurrences) rather than native
   RPIT, so the capture-rule change is unlikely to bite. Two semantic hazards remain and are only
   discoverable by actually running the migration.
4. **The examples gate needs a 4-invocation feature matrix, not a bare `cargo build --examples`.**
   Verified live: 43/47 build under default features; the other 4 are **silently skipped, not
   failed**, because their `required-features` (`vision`, `content-processing`, `web-server`)
   are not in the default set. All 47 were verified to build across the four invocations.
5. **Two CI patterns already exist and should be reused, not invented**: `release.yml:160-220`
   has a multi-arch buildx + image-size check; `integration-tests.yml:171-264` has a complete
   kind/kubectl smoke job with a < 30 s startup check. **No time-budget precedent exists
   anywhere in the repo** and must be authored from scratch.
6. **`k8s/deployment.yaml` runs a placeholder `sleep 3600` with all readiness probes commented
   out** — so the existing startup-budget check measures container scheduling, not real app
   readiness. The planner must decide whether SC5's Kubernetes clause is satisfied by the
   placeholder or requires a real probe.
7. **QUICKSTART's primary code sample is structurally broken, not merely stale**:
   `PaladinBuilder` / `PaladinExecutionService` live in the root `paladin` lib crate
   (`src/application/services/paladin/`), not in `paladin-ai-core` as the doc's imports claim,
   and the doc's `Cargo.toml` block never lists `paladin-ai` as a dependency. **Not fixable by
   network or API-key access** — this is a real REL-04 work item, not an environment limitation.
8. **All 15 advisory suppressions carry written rationale; 4 (borderline 6) unmaintained entries
   lack a specific migration/review note.** A live re-run confirms `RUSTSEC-2025-0121`
   (`deny.toml:136`) is the *only* stale/non-matching entry.

---

# Phase 4 Research — Part A: Manifests & Release Mechanics

**Researched:** 2026-08-02
**Scope:** D-01 … D-09, D-17 (version bump, edition unification, CHANGELOG finalize, advisory-note
audit). Docs/QUICKSTART/CI (D-10 … D-16) are Part B's concern, not researched here.
**Confidence:** HIGH — every claim below is either a direct file read, a live `grep`, or a live
`cargo release`/`cargo deny` dry-run against this tree at HEAD `68ba809`. No WebSearch was needed;
this is entirely a "read the repo" phase-half.

---

## Q1 — Edition-2024 migration for `paladin-ports` and `paladin-notifications` (D-04)

**Crate sizes:** `paladin-ports/src` = 33 files, 13,837 lines. `paladin-notifications/src` = 5
files, 1,099 lines. Combined 38 files, matching the CONTEXT's figure. [VERIFIED: `find | xargs
wc -l` on this tree]

**Manifest state:** `crates/paladin-ports/Cargo.toml:4` and `crates/paladin-notifications/Cargo.toml:4`
both read `edition = "2021"`. Neither file contains any pre-existing `#![allow(...)]` or
edition-2024-lint suppression — `grep -rn "edition2024|rust_2024|unsafe_op_in_unsafe_fn|static_mut_refs"`
across both crates and the root `Cargo.toml` returns nothing. [VERIFIED]

### Hazard-by-hazard findings

| Hazard | Found in these two crates? | Evidence |
|---|---|---|
| `unsafe_op_in_unsafe_fn` deny-by-default | **No `unsafe` blocks/fns at all** — `grep -rn "unsafe" crates/paladin-ports/src crates/paladin-notifications/src` returns zero matches. Not applicable. | [VERIFIED] |
| `static_mut_refs` | **No `static mut` anywhere** in either crate. Not applicable. | [VERIFIED] |
| RPIT / `impl Trait` lifetime-capture (`+ use<>`) | **Not applicable to first order.** `grep -rn "-> impl "` in both crates returns zero hits. These are trait-heavy port crates, but every trait uses `#[async_trait]` (57 occurrences, `grep -rn "#\[async_trait\]"`), which desugars to `Box<dyn Future<...>>` at macro-expansion time, not native RPITIT. The 2024 edition's `impl Trait` lifetime-capture default change specifically affects *native* `-> impl Trait` return-position syntax; async-trait's generated code is unaffected because it never emits bare RPIT. **[general]** — this is training-knowledge reasoning about how `async_trait` desugars, not verified against the macro's expanded output in this session. | grep-verified absence; desugaring claim is `[general]` |
| `gen` as a reserved keyword | **No identifier named `gen`** — `grep -rnE '\bgen\b'` (excluding `generic`/`generate*` matches) returns nothing in either crate. Not applicable. | [VERIFIED] |
| `if let` temporary-scope / tail-expression temporary-drop-order change | **Not directly greppable** — this is a semantic change (temporaries in the scrutinee of `if let`/`while let` now drop at the end of the `if`/`while` block rather than living through the `else` branch; tail-expression temporary lifetimes changed similarly). Both crates use `if let` (found in `paladin-notifications/src/email_notification_adapter.rs` and elsewhere; 86 combined `if let`/`match`/`matches!` occurrences across both crates via `grep -c`). **The only reliable check is compiling under 2024 and watching for new drop-order-related borrow errors** — this cannot be predicted from source inspection alone. `[general]` risk, mitigated entirely by the build-proof step below. | grep counts `[VERIFIED]`; risk assessment `[general]` |
| Match-ergonomics tightening | Same category as above — a semantic tightening (default binding modes), not syntactically greppable. No specific pattern search performed; treat as a build-time-discoverable risk only. `[general]` | Not independently verified |
| `unsafe` attributes (`#[no_mangle]` → `#[unsafe(no_mangle)]`) | **No `no_mangle` anywhere** — `grep -rn "no_mangle"` returns zero hits in either crate. Not applicable (this hazard requires `unsafe` FFI/linkage code, which these port-trait/adapter crates do not contain). | [VERIFIED] |

**Bottom line:** four of the seven textbook 2024-migration hazards are provably absent by grep
(`unsafe_op_in_unsafe_fn`, `static_mut_refs`, `gen` keyword, `#[unsafe(no_mangle)]`). RPIT capture
is very unlikely to bite because both crates use `async_trait` exclusively rather than native RPIT.
The two genuinely open risks — `if let`/tail-expression drop-order and match-ergonomics tightening —
are semantic, not syntactic, and can only be surfaced by actually compiling under the new edition.
**This is a low-risk migration** but "low-risk" must still be proven by a real build, not asserted.

### Concrete migration route

1. **Manifest edit first, then `cargo fix`.** `cargo fix --edition` operates by compiling the crate
   under the *old* edition with migration lints enabled and rewriting flagged constructs — but the
   idiomatic and documented cargo workflow for a *two-edition-in-one-manifest-line* bump (2021→2024
   directly, skipping the interim `--edition 2021` step since these crates are already on 2021) is:
   edit the manifest's `edition` field to `"2024"` first, then run `cargo fix` against the new
   edition to auto-migrate any construct that changed meaning, then hand-fix anything `cargo fix`
   flags but cannot rewrite automatically. `[general]` — this is standard `cargo fix --edition`
   guidance from the Rust Edition Guide, not verified against this specific tree by executing the
   command (doing so would mutate the tree; not run in this research pass per the no-tree-mutation
   discipline of this session).
2. **Exact invocation, single workspace member, offline:**
   ```bash
   cargo fix --edition --offline -p paladin-ports --allow-dirty --lib
   cargo fix --edition --offline -p paladin-notifications --allow-dirty --lib
   ```
   Add `--tests` / `--examples` if either crate's test or example targets need the same pass (check
   after the `--lib` pass whether `cargo build -p <crate> --tests --offline` still compiles clean).
   `--allow-dirty` is required because the manifest edit (`edition = "2024"`) must land in the same
   commit or a prior uncommitted state — `cargo fix` refuses to run against a dirty tree otherwise.
3. **No hand-edits are predicted** by the grep sweep above, since the four mechanically-detectable
   hazards are absent. If `cargo fix --edition` or the subsequent `cargo build` surfaces an `if
   let`/drop-order or match-ergonomics diagnostic, the plan should budget for isolated hand-fixes at
   whatever `file:line` the compiler names — this cannot be pre-enumerated without running the
   compiler.

### `doctest = false` interaction — plainly: none

`crates/paladin-ports/Cargo.toml:14-18` sets `doctest = false` under `[lib]`, with a comment citing
a "Task 7.0" that RECON-08 already proved never existed (D-10's territory, not this one). **The
`edition` field and `doctest` are orthogonal Cargo manifest keys** — `edition` governs language
semantics for compiling ordinary code; `doctest` governs whether `cargo test`/`cargo doc --test`
attempts to compile and run the crate's doc comments as standalone test binaries. Bumping `edition`
to `"2024"` does **not** re-enable doctests, does not require touching the `doctest = false` line,
and does not interact with DEBT-03/Phase 8 in either direction. State this in the plan so no task
is tempted to "clean up" the doctest flag while touching this manifest line. [VERIFIED: manifest
read + Cargo semantics]

### D-06's proof obligation — what must be re-verified

Per D-06, both of the following must succeed **after** the edition bump on both manifests, each
recorded to the D-17 provenance standard (`rustc -vV`, `cargo --version`, `git rev-parse HEAD`,
`date -u`, raw pasted stdout):

```bash
cargo build --workspace --offline
cargo build --workspace --no-default-features --offline
```

Both commands exercise the *whole* workspace, not just the two bumped crates — this is intentional:
an edition bump on `paladin-ports` can, in principle, change trait-object inference or macro
expansion visible to every downstream crate that depends on it (all nine other crates path-depend on
`paladin-ports`). The `--no-default-features` pass specifically catches any feature-gated code path
in a *dependent* crate that only compiles under the old edition's rules. Neither command was run in
this research pass (would require the manifest edit to already exist); the planner should schedule
this as the verification step immediately following the two `cargo fix --edition` invocations, in
the same plan/wave as D-04's manifest edits.

---

## Q2 — Version-bump mechanics for 0.7.0 (D-01, D-03)

### `cargo-release` is installed

```
$ command -v cargo-release
/usr/local/cargo/bin/cargo-release
$ cargo release --version
cargo-release 1.1.2
```
[VERIFIED: live check, this session] — this **supersedes** any plan branch that assumes a hand-edit
route is necessary. The environment has the tool; use it.

### Internal path-pins ARE updated by `cargo release version`, verified live via dry-run

A **non-mutating dry-run** (no `--execute` flag — `cargo-release`'s default mode only prints its
plan) was run against the live tree:

```bash
$ cargo release version 0.7.0 --workspace
```

Output (excerpted) confirms `cargo-release` walks the full dependency graph and rewrites **every**
internal path-pin, including the one exact-pin (`=0.6.0` → `=0.7.0`) at
`crates/paladin-ports/Cargo.toml:21`:

```
   Upgrading paladin-ai-core from 0.6.0 to 0.7.0
    Updating workspace's dependency from 0.6.0 to 0.7.0
    ...
    Updating paladin-ports's dependency from =0.6.0 to =0.7.0
    ...
   Upgrading paladin-ports from 0.6.0 to 0.7.0
    ...
```
Every one of the eleven `[package] version = "0.6.0"` lines is upgraded, and every one of the
internal `path =` dependency version requirements listed under `<already_established>` fact-set /
the earlier grep sweep is rewritten to `0.7.0` (or `=0.7.0` for the one exact pin). **No hand-edit
list is needed** — this makes the earlier grep-derived "every `0.6.0` occurrence" enumeration
informative for audit purposes but not the execution mechanism.

**One false-positive to flag for the planner:** the earlier `grep -rn '0\.6\.0' --include=Cargo.toml`
sweep also matched `tiktoken-rs = { version = "0.6.0", optional = true }` at
`crates/paladin-memory/Cargo.toml:37` and `crates/paladin-content/Cargo.toml:44` — this is an
**external crates.io dependency** whose version happens to coincide with the workspace's current
version. `cargo-release`'s dry-run output does **not** touch either `tiktoken-rs` line (confirmed:
absent from the plan output), correctly distinguishing it from the internal pins. **Do not let a
plan task naively bump every grep hit — only path-dependency version fields move.**

**Pre-flight warning surfaced by the dry-run:** `cargo-release` printed `warning: uncommitted
changes detected, please resolve before release: .planning/config.json (Status(WT_MODIFIED))`. This
is a pre-existing dirty-tree condition (visible in this session's initial `git status`), not
something this research introduced. The plan should note that `cargo release version --execute`
will need either a clean tree or `--no-confirm`/`--allow-dirty`-equivalent handling; the Makefile's
`release` target already passes `--no-confirm` (see below) but that flag silences the *confirmation
prompt*, not the *dirty-tree warning* — worth a `checkpoint:human-verify` if the plan wants a
guaranteed-clean commit boundary before the bump.

### `release.toml` settings governing safety

`release.toml:29` — `publish = false`. `release.toml:32` — `push = false`. Both apply to the
`cargo release` *tool itself* — i.e., a bare `cargo release --execute` (the full release subcommand,
not just `version`) would still not push or publish because these settings are baked into the config
file it reads. `release.toml:17` — `shared-version = true`. `release.toml:23-24` —
`consolidate-commits = true`, `pre-release-commit-message = "chore(release): version {{version}}"`.
`release.toml:34-39` — an explicit comment stating the CHANGELOG finalize is deliberately done by
`make release`'s Makefile logic, not by `cargo-release`'s `pre-release-replacements` (because a
root-level replacement in a lockstep multi-crate workspace would run once per crate and duplicate
the heading).

### Non-pushing, non-publishing command sequence

```bash
cargo release version 0.7.0 --execute --no-confirm --workspace --offline
```
This is exactly the command the Makefile's `release` target runs at `Makefile:475`
(`@$(CARGO) release version "$(VERSION)" --execute --no-confirm --workspace`, no `--offline` flag in
the Makefile version — add it per D-17). This single command **bumps all eleven manifests and every
internal path-pin** and stops — it does not commit, tag, or push. Confirmed non-destructive to the
push/publish boundary by `release.toml:29,32` (`publish = false`, `push = false` apply to the
`cargo-release` binary's own release-orchestration path, and `version` is a narrower subcommand that
never invokes publish/push logic regardless).

### `make release` vs. the safe subset — quantified

`Makefile:439-486` is the full `release` target. It does **five** things in sequence, three of which
are outward-facing (must NOT be run this phase):

| Step | Makefile line(s) | Safe for Phase 4? |
|---|---|---|
| Validate VERSION arg is semver | `444-447` | Safe, informational |
| Require `cargo-release` installed | `448-451` | Safe, informational |
| **Require branch == `main`** (or `RELEASE_ALLOW_ANY_BRANCH=1`) | `456-466` | **Blocks Phase 4 outright on `release/v0.7.0`** — current branch is not `main`, and this check runs unconditionally unless the override env var is set. The plan must NOT invoke `make release` as-is; either use the override (still doesn't fix the next problem) or bypass the Makefile target entirely and run the constituent safe commands by hand. |
| Fetch/compare against `origin/main`, bump versions via `cargo release version ... --execute --no-confirm --workspace` | `467-476` | **The version-bump line (`476`) is safe in isolation** — it is the exact command quoted above. |
| CHANGELOG finalize via `perl -0pi` | `477-479` | Safe — pure local file edit, see Q3. |
| **`git commit` (`482`), `git tag -a` (`483`), `git push origin HEAD` (`484`), `git push origin "v$(VERSION)"` (`485`)** | `482-485` | **Not safe — two of these four lines push to `origin`.** This is the exact D-03 boundary: commit and local tag are `reversible`; the two `git push` lines are the `one-way` step that triggers `.github/workflows/release.yml`'s crates.io publish. |

**Quantified difference from `release.toml`:** `release.toml` sets `publish = false` / `push = false`
as *tool-level* policy that `cargo-release`'s own `--execute` path honors. The Makefile's `release`
target does not consult those flags for its own `git push` calls at all — lines 484-485 are **raw
shell `git push` invocations added by the Makefile author, entirely outside `cargo-release`'s
config-driven safety net.** This is why `release.toml`'s `publish = false`/`push = false` protects
`cargo release version` (used alone) but provides **zero** protection against `make release`, which
pushes unconditionally once it reaches line 484 regardless of what `release.toml` says. **The plan
must call the constituent commands directly (`cargo release version ...`, then the CHANGELOG perl
line, then `git tag -a` locally) and must never invoke `make release` this phase.**

**Also note:** the branch guard at `Makefile:461` would fail this run anyway (`release/v0.7.0` ≠
`main`), which is a second, independent reason `make release` cannot be the plan's release
mechanism this phase — it isn't just about push/publish, the target won't even reach the bump step
without `RELEASE_ALLOW_ANY_BRANCH=1`.

---

## Q3 — CHANGELOG finalize mechanics (D-03)

### Exact heading transformation

`Makefile:477-479`:
```make
@DATE=$$(date +%Y-%m-%d); \
    perl -0pi -e "s/## \\[Unreleased\\]/## [Unreleased]\n\n## [$(VERSION)] - $$DATE/" CHANGELOG.md
```
This is a single non-greedy substitution: it finds the **first** literal line `## [Unreleased]` and
replaces it with two lines — the same `## [Unreleased]` heading (now empty, since everything that
followed the original heading now falls under the newly-inserted dated heading immediately below
it) and a new `## [0.7.0] - <YYYY-MM-DD>` heading. **All existing body content that was under
`## [Unreleased]` (the "Phase 12.1" section, `### Added` / `### Changed` / `### Removed` etc.)
automatically becomes the content of the new `## [0.7.0]` section**, because it physically follows
the inserted heading and precedes the next `## [` heading (`## [0.6.0]` at line 63). No content is
moved by hand; the perl one-liner's placement does the "move" implicitly. This is safe to reproduce
by hand (edit the single line, or re-run the identical `perl` invocation standalone, without
invoking `make release`).

### The second required edit: `## [0.6.0]`'s missing date

`CHANGELOG.md:63` reads `## [0.6.0]` with **no trailing ` - <date>`** — confirmed the only heading in
the file lacking one (`grep -n '^## \['` shows every other heading, from `## [0.5.1] - 2026-06-04`
down through `## [0.1.0] - Previous Releases`, carries a date or date-like suffix). D-03 requires
this be corrected in the same pass; there is no tooling for this — it is a one-line hand edit. The
date to use is not determinable from the tree (v0.6.0 was never tagged, so there is no tag-creation
timestamp to source it from); the plan should either use the last commit date touching that section
or flag it for a `checkpoint:human-verify` if the exact historical date matters. This is a gap — see
Open Gaps.

### Date format precedent

Every dated heading uses **`YYYY-MM-DD`** (e.g., `## [0.5.1] - 2026-06-04`, `## [0.5.0] -
2026-06-03`, `## [0.4.3] - 2026-06-01`). The Makefile's own `perl` line generates dates via `date
+%Y-%m-%d`, matching this convention exactly — no reconciliation needed for the *new* `## [0.7.0]`
heading; only the retroactive `## [0.6.0]` date needs a human-supplied or inferred value.

### The "Phase 12.1" heading

`CHANGELOG.md:10` — `### Phase 12.1 — Complete the Paladin Arsenal MCP client (dogfood)`, directly
under `## [Unreleased]`. This numbering (`Phase 12.1`) is old `.project/`-era milestone/epic
numbering, **not** a GSD `.planning/phases/` phase number (this project's GSD phases are numbered
1, 2, 3, 4… with no decimal sub-phases). A reader encountering this inside `.planning/`-era
provenance will misread it as referring to GSD Phase 12 (SUPPLY-01/SUPPLY-02, per this very
CONTEXT's canonical refs). Per CONTEXT's Claude's Discretion, renaming is cosmetic and optional;
if the plan leaves it, it should at minimum add a one-line provenance note (e.g., an HTML comment
or a parenthetical) disambiguating "Phase 12.1" from a GSD phase reference, consistent with the
"amend at source with dated provenance" convention this corpus uses everywhere else.

---

## Q4 — Advisory-suppression migration-note audit (D-09)

### Full suppression inventory

`deny.toml`'s `[advisories] ignore` block runs `deny.toml:112-147` (opening `ignore = [` at `112`,
closing `]` at `147`). `.cargo/audit.toml`'s `[advisories] ignore` block runs `audit.toml:28-34`.
**15 total distinct advisory IDs** across both files, matching the CONTEXT's count exactly.

| Advisory ID | `deny.toml` line | `.cargo/audit.toml` line | Class | Written rationale? | Migration/review note? |
|---|---|---|---|---|---|
| RUSTSEC-2023-0071 | 117 (comment 114-116) | 29 (comment 5-7) | vulnerability | Yes — RSA timing side-channel, transitive via sqlx-mysql | **Yes** — "revisit when sqlx upgrades rsa" (both files) |
| RUSTSEC-2025-0111 | 121 (comment 118-120) | 30 (comment 9-11) | vulnerability | Yes — tokio-tar path-traversal, transitive via testcontainers | **Yes** — "revisit when testcontainers upgrades" (both files) |
| RUSTSEC-2021-0139 | 130 | — (deny-only) | unmaintained | Yes — "ansi_term (unmaintained) — transitive" | **Blanket only** — no per-entry named parent; relies solely on the group header's generic "revisit when the respective parent crates upgrade" (`deny.toml:127-128`), with no crate named to watch |
| RUSTSEC-2021-0141 | 131 | — (deny-only) | unmaintained | Yes — "dotenv (unmaintained)" | **Yes** — "replace with dotenvy later" (specific, inline) |
| RUSTSEC-2024-0370 | 132 | — (deny-only) | unmaintained | Yes — "proc-macro-error (unmaintained) — via structopt" | Partial — names the parent (structopt) but no explicit "revisit when" verb; structopt itself has its own migration note (see 2022-0104) |
| RUSTSEC-2024-0375 | 133 | — (deny-only) | unmaintained | Yes — "atty (unmaintained) — transitive" | **Blanket only** — no named parent, no specific action |
| RUSTSEC-2025-0057 | 134 | — (deny-only) | unmaintained | Yes — "fxhash (unmaintained) — transitive" | **Blanket only** — no named parent, no specific action |
| RUSTSEC-2025-0119 | 135 | — (deny-only) | unmaintained | Yes — "number_prefix (unmaintained) — transitive" | **Blanket only** — no named parent, no specific action |
| RUSTSEC-2025-0121 | 136 | — (deny-only) | unmaintained | Yes — "gcc (unmaintained) — transitive build dep" | **Blanket only, AND stale** — see removal note below |
| RUSTSEC-2025-0134 | 137 | — (deny-only) | unmaintained | Yes — "rustls-pemfile (unmaintained) — via tonic/testcontainers" | Partial — names two parents but no explicit "revisit when" verb |
| RUSTSEC-2024-0436 | 138 | — (deny-only) | unmaintained | Yes — "paste (unmaintained) — via utoipa (M12 E6); no drop-in replacement upstream" | **Yes** — explicitly states no path exists yet (a legitimate "review note": nothing to do until upstream acts) |
| RUSTSEC-2022-0104 | 139 | — (deny-only) | unmaintained (maintenance mode) | Yes — "structopt (maintenance mode) — CLI arg parsing" | **Yes** — "migrate to clap-derive later" (specific, inline) |
| RUSTSEC-2026-0187 | 144 | 31 (comment 19-22) | unsound (DoS, lopdf) | Yes — full prose in both files | **Yes** — audit.toml: "Revisit when pdf-extract ships lopdf >= 0.42 without new advisories" |
| RUSTSEC-2026-0194 | 145 | 32 (comment 24-27) | unsound (DoS, quick-xml) | Yes — full prose in both files | **Yes** — audit.toml: "Revisit when rust-s3 bumps quick-xml" |
| RUSTSEC-2026-0195 | 146 | 33 (comment 24-27, shared with -0194) | unsound (DoS, quick-xml) | Yes — same shared prose | **Yes** — same "revisit when rust-s3 bumps quick-xml" note (shared) |

**Entries genuinely missing a migration/review note (D-09's edit surface):** RUSTSEC-2021-0139
(ansi_term), RUSTSEC-2024-0375 (atty), RUSTSEC-2025-0057 (fxhash), RUSTSEC-2025-0119
(number_prefix) — **four** entries with only the generic group-header blanket statement and no
crate-specific "revisit when X upgrades" hook. RUSTSEC-2025-0121 (gcc) is a fifth entry in the same
position but is being **removed outright** (see below), so it does not need a note added.
RUSTSEC-2024-0370 and RUSTSEC-2025-0134 are borderline — they name a parent but not an explicit
verb; the plan should treat these as needing a one-clause addition too ("revisit when
{structopt/tonic|testcontainers} upgrades") for consistency with the other unmaintained entries that
already have one.

All 15 entries have a **written rationale** (D-09's first bar is already met everywhere, confirming
the CONTEXT's claim). The gap is specifically the *migration/review note* on 4-6 of the 15 unmaintained
entries.

### Stale/removable entries — single re-run confirms only one

```bash
$ cargo deny check advisories 2>&1 | tail -8
warning[advisory-not-detected]: advisory was not encountered
    ┌─ /workspace/deny.toml:136:6
    │
136 │     "RUSTSEC-2025-0121", # gcc (unmaintained) — transitive build dep
    │      ━━━━━━━━━━━━━━━━━ no crate matched advisory criteria

advisories ok
```
[VERIFIED: live re-run, this session, single allowed call per the research questions' instruction]
**Only `RUSTSEC-2025-0121` triggers `advisory-not-detected`.** No other one of the fourteen remaining
entries is in the same stale position — `cargo deny check` prints exactly one such warning and
otherwise reports `advisories ok`. This confirms fact 4 in the CONTEXT is complete, not partial: D-08's
removal work is scoped to this single entry (`deny.toml:136`), and no companion removal is hiding
elsewhere in the 15-entry list.

### Explicitly out of scope (per this phase's boundary — restated, not re-litigated)

No owner or expiry field is proposed anywhere in this research. The four newly-surfaced advisories
(`RUSTSEC-2021-0145`, `RUSTSEC-2026-0221`, `RUSTSEC-2026-0205`, yanked `spin 0.9.8`) are **not**
added to either ignore list here — they are recorded as a dated finding per D-09 and handed to
SEC-01/SUPPLY-02, which is a Part B / ledger-writing concern, not a manifest edit this research
covers.

---

## Open Gaps

1. **The historical date for the retroactive `## [0.6.0]` CHANGELOG heading (Q3) cannot be derived
   from the tree with certainty.** `v0.6.0` was never tagged, so there is no tag timestamp. The
   most defensible proxy is the git log date of the commit that introduced the `## [0.6.0]` heading
   text — this research did not run `git log --follow -p CHANGELOG.md` to locate it (out of budget
   scope for this research pass; recommend the planner add a small `git log -S'## [0.6.0]'
   CHANGELOG.md` task, or route it through a `checkpoint:human-verify` if precision matters more
   than a reasonable approximation).
2. **The `if let`/tail-expression drop-order and match-ergonomics hazards for `paladin-ports` and
   `paladin-notifications` (Q1) cannot be fully ruled out by static grep** — they are semantic, not
   syntactic, changes. This research established that the four *mechanically detectable* hazards are
   absent, but the only way to fully close this question is to actually run `cargo fix --edition`
   and `cargo build --workspace` per the D-06 proof obligation, which is plan/execution work, not
   research work (running it now would mutate the tree ahead of planning).
3. **Whether `cargo release version 0.7.0 --execute` will proceed cleanly given the pre-existing
   `.planning/config.json` uncommitted-change warning** was not tested to completion (only the
   non-mutating dry-run was run). The planner should treat "confirm a clean working tree before the
   bump" as an implicit precondition task.
4. **`async_trait`'s macro-expansion behavior under edition 2024** (the RPIT-capture reasoning in
   Q1) is asserted from general Rust knowledge about how the `async-trait` crate desugars, not
   verified by inspecting its expanded output in this sandbox (`cargo expand` was not run — not
   installed/verified present, and not in the tool-availability list already established for this
   phase).


---

# Phase 4 Research — Part B: CI Gates, Examples & Quickstart

**Researched:** 2026-08-02
**Domain:** GitHub Actions CI configuration, Cargo example/feature matrix, QUICKSTART timing
**Confidence:** HIGH (all repo claims verified live against HEAD `68ba809`; no external-docs claims required)

## Scope

Covers D-10 through D-17 of `.planning/phases/04-release-coherence/04-CONTEXT.md`: the gate-suite
proof, the three missing SC5 CI jobs, the example-build gate, and the QUICKSTART timing
measurement. All findings below are `[VERIFIED: local repo / cargo]` unless marked otherwise — this
research required no package installs and no external documentation lookups, so the
`research-plan`/provider seam was not invoked; every claim traces to a file read or a command run
in this sandbox.

## Q1. The three missing SC5 CI jobs

### (a) Trigger stanza

`ci.yml:3-12` currently:
```yaml
on:
  # push:
  #   branches: [ main, develop, 'feature/**' ]
  pull_request:
    branches: [ main, develop ]
  workflow_dispatch:
```
`[VERIFIED: ci.yml:3-12]` A push to `release/v0.7.0` matches none of `pull_request`'s target
branches and isn't a manual dispatch, so **nothing runs**. Uncommenting the original line is
insufficient per D-14.1 — it lists `main, develop, feature/**`, never `release/**`.

**Recommended replacement** (adds `release/**` without dropping the existing PR-only-avoid-double-run
convention documented in the comment):
```yaml
on:
  push:
    branches: [ main, develop, 'feature/**', 'release/**' ]
  pull_request:
    branches: [ main, develop ]
  workflow_dispatch:
```
This is the smallest change that makes a `release/v0.7.0` push trigger CI. Two sibling workflows
(`integration-tests.yml:3-16`, `feature-flags.yml:3-9`) carry the identical commented-out `push:`
stanza with the same `main, develop, feature/**` list and the same avoid-double-run comment —
**all three files share one convention**; whether to add `release/**` to those two as well is a
discretion call (SC5 only names `ci.yml`; the other two aren't in scope here since PIPE governs
`integration-tests.yml`'s deprecated-actions per the deferred list, and `feature-flags.yml` isn't
named in D-14 at all). Recorded here as a **plan-level open question**, not resolved by this
research.

### (b) Examples-build job

Model this job on `feature-flags.yml`'s `feature-matrix` job (`feature-flags.yml:19-118`)
`[VERIFIED: feature-flags.yml:19-118]`, not on `ci.yml`'s `lint`/`test` job (those don't need a
feature matrix). `feature-flags.yml` already encodes the exact caching pattern (`actions/cache@v4`
for registry/index/build, three separate cache steps, `dtolnay/rust-toolchain@master`,
`actions/checkout@v5` per `ci.yml`'s convention vs. `checkout@v4` in `feature-flags.yml` — note
`ci.yml` itself is already on `checkout@v5`, one version ahead of the other three workflow files;
follow `ci.yml`'s own convention since the new job lands there).

**Required-features audit** `[VERIFIED: Cargo.toml:220-238]` — exactly 4 of the 47 example files are
declared `[[example]]` targets, and every one of them gates on non-default features:

| Example | `required-features` |
|---|---|
| `vision_analysis` | `["vision", "llm-openai"]` |
| `vision_battalion` | `["vision", "llm-openai"]` |
| `document_processing` | `["content-processing"]` |
| `http_service_host` | `["web-server"]` |

`[features] default = ["llm-openai"]` `[VERIFIED: Cargo.toml:259]` — `llm-openai` is on by default;
`vision`, `content-processing`, `web-server` are **not**.

**`cargo build --examples --offline` under default features is NOT sufficient.** Verified live: ran
`cargo build --examples --offline` after deleting the 4 required-features binaries from
`target/debug/examples/` — the bulk `--examples` invocation silently **skips** all 4 (no error, no
warning printed to the tail of stdout; cargo's default behavior for unmet `required-features` on a
bulk target selector is silent omission). Confirmed independently: `cargo build --example
document_processing --offline` (without the feature) hard-errors:
```
error: target `document_processing` in package `paladin-ai` requires the features: `content-processing`
Consider enabling them by passing, e.g., `--features="content-processing"`
```
Same hard error reproduced for `vision_analysis` (needs `vision`, `llm-openai`) and
`http_service_host` (needs `web-server`).

**A feature matrix of (at minimum) 4 build invocations is required** to cover all 47 targets:
1. `cargo build --examples --offline` (default features — covers the 43 auto-discovered examples)
2. `cargo build --example vision_analysis --example vision_battalion --features "vision,llm-openai" --offline`
3. `cargo build --example document_processing --features "content-processing" --offline`
4. `cargo build --example http_service_host --features "web-server" --offline`

All 4 verified to **succeed** in this sandbox `[VERIFIED: cargo build, this session]`:
- (1) default: `Finished ... in 20.29s` (incremental; cold estimate not measured — see D-17 caveat)
- (2) vision: `Finished ... in 19.50s`
- (3) content-processing: pulled in `bollard`, `mockito`, `wiremock`, both `reqwest` versions,
  `pdf-extract`, `sqlx-sqlite`, `rmcp`, `testcontainers*`, plus 6 workspace crates —
  `Finished ... in 1m 03s` (cold path for that feature; longest of the four)
- (4) web-server: `Finished ... in 30.07s`

**Conclusion for D-12/D-13:** "every example target builds" is a **claim the plan can record as
green today**, not a work item — but only if the CI job uses the 4-invocation matrix above.
A single `cargo build --examples` step (no feature flags) would make CI silently under-cover 4 of
47 examples with no failure signal, which is exactly the kind of unproven-gate risk D-15 warns
about for Docker/K8s. **Do not let the plan write a one-line `cargo build --examples` step and
call SC5's example gate closed.**

### (c) Docker budget assertions

`ci.yml`'s `docker` job (`:409-434`) `[VERIFIED]` builds **single-platform** (no `platforms:` key —
defaults to the runner's native arch), asserts **no size or time budget**, and does not push.
Existing action versions in that job: `docker/setup-buildx-action@v4` (`:418`),
`docker/build-push-action@v6` (`:421`).

**Multi-arch config is missing entirely.** `release.yml`'s `build-docker` job (`:160-220`)
`[VERIFIED: release.yml:160-220]` already does near-identical work for the release pipeline and is
the closest existing model in this repo — but note it runs **older pinned action versions**:
`docker/setup-qemu-action@v3`, `docker/setup-buildx-action@v3`, `docker/build-push-action@v5`. Per
the instruction to cite versions already in use rather than invent new ones, and because `ci.yml`
itself is already one step ahead (`buildx-action@v4`, `build-push-action@v6`), the new `ci.yml` job
should stay on `ci.yml`'s own already-used `@v4`/`@v6` pair and add `docker/setup-qemu-action@v3`
(the only piece `ci.yml` doesn't already have — `release.yml` is the only in-repo precedent for its
version, so `@v3` is what's "already used in this repo" for that specific action).

`release.yml:196-220` already contains a **size-check pattern to reuse directly**:
```yaml
      - name: Build and push
        uses: docker/build-push-action@v5
        with:
          context: .
          platforms: linux/amd64,linux/arm64
          push: true
          tags: ${{ steps.meta.outputs.tags }}
          cache-from: type=gha
          cache-to: type=gha,mode=max

      - name: Verify image size
        run: |
          IMAGE=$(echo '${{ steps.meta.outputs.json }}' | jq -r '.tags[0]')
          docker pull "$IMAGE"
          SIZE=$(docker image inspect "$IMAGE" --format='{{.Size}}')
          SIZE_MB=$((SIZE / 1024 / 1024))
          echo "Image size: ${SIZE_MB} MB"
          if [ $SIZE_MB -gt 500 ]; then
            echo "::warning::Image size (${SIZE_MB} MB) exceeds 500 MB target"
          fi
```
Two adaptations needed for the `ci.yml` job (which does not push to a registry, unlike
`release.yml`'s):
1. `push: false` in CI (as it is today) means there's no registry pull to inspect — instead use
   `docker image inspect paladin:test --format='{{.Size}}'` directly against the locally-built
   image, the same tag the job already assigns at `ci.yml:426`.
2. `release.yml`'s check only emits `::warning::` (non-blocking) — SC5's "inside its budget" language
   implies the CI gate should **fail**, not warn. Change `echo "::warning..."` to `exit 1` (or keep
   as `::error::` + `exit 1`) for the new `ci.yml` job specifically; `release.yml`'s existing
   non-blocking warning is out of scope to change (not named in D-14/D-15).

**Time budget has no existing precedent in this repo** — neither `ci.yml` nor `release.yml` times a
Docker build today. The `kubernetes-smoke-test` job in `integration-tests.yml:238-249` (see below)
is the closest in-repo pattern for a wall-clock budget check (epoch-diff, `::warning::` on
overage). Model the Docker time budget the same way: capture `date +%s` before the
`build-push-action` step, capture it again after, diff, and either warn or fail if `> 300` (5 min).
Multi-arch (`linux/amd64,linux/arm64`) via QEMU emulation is markedly slower than native-arch
builds — the plan should budget for this when deciding whether the 5-minute figure includes both
architectures or is measured per-arch (D-14/D-15 don't specify; **flag as an open question for the
planner**, this research found no PROJECT.md/ROADMAP clarification of which).

### (d) kind-based Kubernetes smoke job

`k8s/` contains `[VERIFIED: find k8s -type f]`:
```
k8s/namespace.yaml
k8s/configmap.yaml
k8s/secret.yaml.example
k8s/redis.yaml
k8s/minio.yaml
k8s/deployment.yaml
k8s/service.yaml
k8s/README.md
k8s/server/{configmap,deployment,service}.yaml, k8s/server/secret.yaml.example
```

**`integration-tests.yml` already has a complete, working `kubernetes-smoke-test` job**
(`integration-tests.yml:171-264`) `[VERIFIED]` — this is the setup to reuse, not reinvent, per the
research brief's explicit instruction. It:
1. Spins up kind via `helm/kind-action@v1` (`version: v0.20.0`, `cluster_name: paladin-test`)
2. `kubectl create namespace paladin`
3. Creates a `paladin-secrets` Secret with dummy LLM keys + MinIO creds
4. `kubectl apply -f k8s/configmap.yaml`, `k8s/redis.yaml`, `k8s/minio.yaml`, then `sleep 20`
5. `kubectl wait --for=condition=ready pod -l app=redis -n paladin --timeout=120s` (and same for
   minio)
6. `docker build -t paladin:test .` then `kind load docker-image paladin:test --name paladin-test`
7. `sed 's/paladin:latest/paladin:test/g' k8s/deployment.yaml | kubectl apply -f -`, then
   `k8s/service.yaml`
8. `kubectl wait --for=condition=ready pod -l app=paladin -n paladin --timeout=180s || true`
   (note the `|| true` — this wait does **not** fail the job even on timeout today)
9. **The startup-time budget check already exists** at `:238-249`:
```bash
START_TIME=$(kubectl get pod -l app=paladin -n paladin -o jsonpath='{.items[0].status.startTime}')
READY_TIME=$(kubectl get pod -l app=paladin -n paladin -o jsonpath='{.items[0].status.conditions[?(@.type=="Ready")].lastTransitionTime}')
STARTUP_TIME=$((READY_EPOCH - START_EPOCH))
if [ $STARTUP_TIME -gt 30 ]; then
  echo "::warning::Startup time (${STARTUP_TIME}s) exceeds 30 second target"
fi
```

**Important caveat that changes the plan's job:** `k8s/deployment.yaml:66-68` `[VERIFIED]` currently
runs a **placeholder command**, not the real binary:
```yaml
command: ["/bin/sh"]
args: ["-c", "echo 'Paladin started' && sleep 3600"]  # Placeholder for testing
```
and all three probes (liveness/readiness/startup, `:137-174`) are **commented out** with the note
"needs HTTP server endpoint." This means the existing `kubernetes-smoke-test` job measures
*container scheduling and shell-startup time*, not actual application readiness — the pod becomes
"Ready" the instant the container's `RollingUpdate` default readiness (no probe = ready as soon as
process starts) is satisfied, which for a placeholder `sleep 3600` is nearly instantaneous. **The
< 30s pod-startup budget is trivially satisfied today because there is nothing real being started.**
This is a genuine gap the plan should record: either (a) accept the placeholder-based smoke test as
the SC5 gate as currently scoped (it does prove kind/kubectl orchestration works, which is what
D-14.3 literally asks for), or (b) note that a true readiness-probe-based budget requires
`paladin-web`'s `/health`/`/ready` endpoints to be wired into `k8s/deployment.yaml`'s commented-out
probes first — which is arguably new capability, out of this phase's "no new product capability"
boundary. **Recommend (a)**: reuse the existing job's placeholder shape verbatim inside `ci.yml`
(or invoke/duplicate the `kubernetes-smoke-test` job definition), and record the probe-wiring gap as
a named deferral, not attempt it inside Phase 4.

Since `docker`, `kind`, `kubectl` are all absent in this sandbox `[see already_established #6]`,
this job (like the Docker budget job) can only be **authored and statically validated** here (YAML
syntax, `k8s/*.yaml` file references resolve, action version exists) — never executed. This is
exactly D-15's boundary.

## Q2. Which of the 47 examples build today, and at what cost

**Target count:** `cargo build --examples` produces exactly 47 distinct example binaries when the
right feature flags are supplied across the 4-invocation matrix in Q1(b) — one per `.rs` file in
`examples/`. `[VERIFIED: find examples -name '*.rs' | wc -l` → 47; confirmed all 47 file basenames
appear as `target/debug/examples/<name>` binaries after running the full matrix.]`

**Under default features alone** (`cargo build --examples --offline`, `default = ["llm-openai"]`):
- **43 of 47** compile successfully, silently.
- **4 of 47 are skipped, not failed**: `vision_analysis`, `vision_battalion`,
  `document_processing`, `http_service_host` — all 4 have `required-features` unmet by the default
  set. Cargo's bulk `--examples` selector omits them without any non-zero exit code or visible
  warning in the summary output — **a CI job that runs bare `cargo build --examples` would report
  success while covering only 43/47 targets**, and nothing in the job would flag the gap.

**Feature matrix outcome (this session, `--offline` throughout, per D-17):**
| Invocation | Examples covered | Result | Wall time (this run) |
|---|---|---|---|
| `cargo build --examples --offline` | 43 (default) | pass | 20.29s (incremental) |
| `--example vision_analysis --example vision_battalion --features vision,llm-openai` | 2 | pass | 19.50s |
| `--example document_processing --features content-processing` | 1 | pass | 1m 03s (new deps: bollard, mockito, wiremock, dual reqwest, pdf-extract, sqlx-sqlite, rmcp, testcontainers*) |
| `--example http_service_host --features web-server` | 1 | pass | 30.07s |

None of the 43 auto-discovered (un-gated) examples failed under default features — they compile
cleanly with only `llm-openai` enabled, meaning none of them silently depends on a non-default
feature without declaring `required-features` (a state that would otherwise produce a hard compile
error, not a silent skip, so this was self-verifying: the bulk build either fails loudly for
ungated examples needing missing features, or succeeds).

**Recommendation for the plan:** record "every example target builds" as a **green, verified claim**
for D-12, conditioned explicitly on the CI job implementing the 4-invocation feature matrix from
Q1(b) rather than a single bare `cargo build --examples`. This is a documentation/CI-authoring task,
not a code-fix task — no example source needs changing.

## Q3. QUICKSTART: step sequence, offline reachability, and staleness

### Step sequence (`docs/src/getting-started/quickstart.md`, 127 lines, read whole)

1. Prerequisite: complete `installation.md`, `export OPENAI_API_KEY=...` (`:5-11`)
2. `cargo new my-paladin-agent && cd my-paladin-agent` (`:15-18`)
3. Hand-add 4 dependency lines to the new project's `Cargo.toml`: `paladin-ai-core = "0.5.0"`,
   `paladin-ports = "0.5.0"`, `paladin-llm = { version = "0.5.0", features = ["llm-openai"] }`,
   `tokio = { version = "1", features = ["full"] }` (`:20-28`)
4. Replace `src/main.rs` with a ~35-line program that builds a `PaladinBuilder`, wraps it in a
   `PaladinExecutionService`, and calls `.execute(...)` (`:32-69`)
5. `cargo run` (`:73-75`), expect specific stdout (`:79-83`)
6. Optional: clone the full workspace, `make services-up`, `cargo run --example basic_paladin`,
   `cargo run --example formation_sequential`, (commented) `cargo run --example phalanx_concurrent`
   (`:85-105`)
7. Reference table for `PaladinResult` fields (`:107-117`) — no executable step
8. "What's next" links (`:119-127`) — no executable step

### Steps requiring network or an LLM key (not timeable in this sandbox)

- **Step 2-3 (network):** `cargo new` itself is offline-safe, but the moment `cargo build`/`cargo
  run` is invoked against a fresh project depending on crates.io-hosted `paladin-ai-core`,
  `paladin-ports`, `paladin-llm`, cargo must resolve and fetch those crates from crates.io.
  **crates.io returns HTTP 403 in this sandbox** `[already_established #6]` — this entire path
  cannot be exercised here, `--offline` cannot substitute (there is no local registry mirror of
  these not-yet-published 0.7.0 crates, and `paladin-ai-core` isn't even published under that
  exact version today per D-01's version-state finding).
- **Step 5 (LLM key):** `cargo run` executes `OpenAIAdapter::from_env()?` then makes a live OpenAI
  API call. **No LLM API key is present in this environment** `[already_established #6]` — even if
  step 2-4 could be made to compile, execution would fail at the API call, not produce the
  documented "Hello!" output.
- **Step 6 (partially offline-safe):** cloning the workspace is redundant here (already checked
  out); `make services-up` needs Docker (absent); `cargo run --example basic_paladin` **compiles
  offline fine** (verified as part of the Q2 default-feature build) but **executing** it still
  calls out to an LLM and needs a key, so it fails at the same point step 5 does.

### Can the happy path reach "a working agent" offline at all?

**No — and independent of the network/key blockers, the primary code sample does not match the
shipped tree's crate layout.** Verified live:
- `PaladinBuilder` and `PaladinExecutionService` are defined in
  `/workspace/src/application/services/paladin/{paladin_builder.rs,paladin_execution_service.rs}`
  `[VERIFIED: grep -rl PaladinExecutionService]` — i.e., inside the **root workspace package**
  `paladin-ai` (`Cargo.toml:33`), whose `[lib] name = "paladin"` (`Cargo.toml:45-46`).
- `crates/paladin-core/src/` (the actual `paladin-ai-core` package, `crates/paladin-core/Cargo.toml`)
  contains only `base/` and `platform/` — the `Node<T>` primitive and `Paladin =
  Node<PaladinData>` type alias (`platform/container/paladin.rs:229`). **It has no
  `application::services` module at all.**
- Quickstart's `Cargo.toml` block (`:22-28`) never lists `paladin-ai` (crate `paladin`) as a
  dependency — only `paladin-ai-core`, `paladin-ports`, `paladin-llm`.
- Therefore the `use paladin_ai_core::application::services::paladin::paladin_builder::
  PaladinBuilder;` import in the code sample (`:36`) references a module path that **does not
  exist in the `paladin-ai-core` crate on this tree**, regardless of version number. This is a
  structural staleness defect, not just a version-number staleness defect — pasting the sample
  code into a project with the documented dependency list would fail to compile with an unresolved
  import, even with network access and an API key.

**Largest honestly-measurable prefix in this sandbox:** steps 2 (`cargo new`) trivially succeeds
offline (no dependency resolution needed for the bare scaffold), but step 3 onward requires either
network (crates.io) or a restructured sample pointing at the in-tree workspace via a `path =`
dependency — neither of which the current page supports. **The only prefix of the *documented* page
that is both offline-reachable and produces a real, running artifact is Step 6's `cargo build
--example basic_paladin`** (compile-only, using this session's already-checked-out workspace and
warm local cargo cache) — running it further needs the absent API key. Recommend the plan record
the QUICKSTART measurement as: *"measured through compile of the in-workspace example set; the
documented new-project happy path (steps 2-5) cannot be measured in this environment due to
crates.io 403 and absent LLM key, and separately cannot be measured on ANY machine as currently
written due to the `paladin_ai_core::application::services` import-path defect — this defect should
be fixed as part of REL-04/D-11's edit to `quickstart.md`, not merely the version numbers."*

### Other staleness found relative to the shipped tree

| Location | Claim | Shipped-tree reality | Verified |
|---|---|---|---|
| `quickstart.md:3` | "under five minutes" | Contradicts `introduction.md:9`'s "15 minutes"; D-11 settles on 15 min | already-established #5 |
| `quickstart.md:24-26` | `paladin-ai-core = "0.5.0"`, `paladin-ports = "0.5.0"`, `paladin-llm = "0.5.0"` | Workspace is at `0.6.0` today, converging to `0.7.0` under D-01/D-03 | `Cargo.toml:34`, member manifests |
| `quickstart.md:36-37` | imports from `paladin_ai_core::application::services::paladin::*` | That module tree does not exist in `paladin-ai-core`; it exists only in the root `paladin` lib crate, which quickstart never declares as a dependency | `grep -rl PaladinExecutionService`, `crates/paladin-core/Cargo.toml`, `Cargo.toml:45-46` |
| `quickstart.md:104` | (commented) `cargo run --example phalanx_concurrent` | File is `examples/phalanx_parallel.rs` — no `phalanx_concurrent.rs` exists | `ls examples/ | grep phalanx` |
| `quickstart.md:98,101` | `cargo run --example basic_paladin`, `formation_sequential` | Both files exist and both compile under default features (verified in Q2's matrix run) | `find examples -name 'basic_paladin.rs' -o -name 'formation_sequential.rs'` |

## Validation Architecture

`.planning/config.json` `[VERIFIED: grep]` has no `workflow.nyquist_validation` key — absent means
enabled; this section is required.

### Test framework
| Property | Value |
|---|---|
| Framework | `cargo test` (workspace built-in), no external test runner |
| Config file | none dedicated — behavior driven by `Cargo.toml` `[[test]]` entries (`:172-218`) and `ci.yml` job definitions |
| Quick run (fmt/lint) | `cargo fmt --all -- --check && cargo clippy --workspace --all-targets --all-features -- -D warnings` |
| Full suite | `cargo test --workspace` (unit + integration; Phase 2 recorded 2864 passed / 0 failed at this tree — re-run for this phase, do not cite per D-12) |

### Phase Requirements -> Validation Map
| SC | Behavior | Validation type | Command | Executable here? |
|---|---|---|---|---|
| SC1 (version) | Manifests, tag, CHANGELOG, release notes agree | grep/diff across 12 files | `grep -h '^version' Cargo.toml crates/*/Cargo.toml`, `git tag --list`, `head CHANGELOG.md` | Yes — fully local |
| SC2 (edition) | One edition workspace-wide; `cargo build --workspace` (both default and `--no-default-features`) succeed | build | `cargo build --workspace --offline` (verified this session: 15.95s, clean); `cargo build --workspace --no-default-features --offline` (not yet run — planner should schedule as a task, not assume) | Yes |
| SC3 (advisories) | `cargo audit` 0 vulnerabilities; `cargo deny check` clean; every ignore has rationale + migration/review note | tool run + manual doc audit | `cargo audit`; `cargo deny check` | Yes — both tools work despite crates.io 403 (advisory DB is a GitHub repo; `[already_established #6]`) |
| SC4 (QUICKSTART) | Timed against 15-min target, pass or fail | manual/scripted timing | wall-clock of the documented step sequence | **Partially** — see Q3; full happy path blocked by crates.io 403 + no LLM key + the import-path defect. Measure the offline-reachable prefix only, record the rest as `deferred with reason` |
| SC5 (gate suite) | fmt, clippy `-D warnings`, workspace tests, doc tests, all 47 examples, multi-arch Docker inside budget, K8s smoke inside budget | mixed | `cargo fmt --all -- --check`; `cargo clippy --workspace --all-targets --all-features -- -D warnings`; `cargo test --workspace`; `cargo test --workspace --doc --exclude paladin-ports` (mirrors `ci.yml:225`'s existing doctest exclusion of `paladin-ports`, whose `doctest = false` is DEBT-03/out of scope here); the 4-invocation example matrix from Q1(b); Docker/K8s jobs | **fmt/clippy/tests/doctests/examples: yes, all executable here.** **Docker multi-arch build and K8s kind smoke: no — `docker`, `kind`, `kubectl` all absent** `[already_established #6]`. Per D-15, these two are authored + statically validated (YAML parses, action refs resolve, `Dockerfile`/`k8s/*.yaml` references resolve) but **never claimed green** in this environment. |

### Sampling rate
- **Per task commit:** `cargo fmt --all -- --check` (fast, already verified clean at HEAD)
- **Per wave merge:** `cargo clippy --workspace --all-targets --all-features -- -D warnings` +
  `cargo build --workspace --offline` + the 4-invocation example matrix
- **Phase gate:** `cargo test --workspace` full suite + `cargo audit` + `cargo deny check`, all
  green, before `/gsd-verify-work`. Docker/K8s jobs are gated by YAML-lint + static-reference-check
  only, per D-15 — **do not require them green as a phase-gate condition**, since they cannot
  execute here.

### Wave 0 gaps
None — no new test files or fixtures are needed. This phase edits manifests, `CHANGELOG.md`,
`ci.yml`, `deny.toml`, `docs/src/getting-started/quickstart.md`, and `.planning/` records; it adds
no product code requiring new unit/integration tests. The "tests" this phase produces are CI job
definitions themselves, validated by YAML syntax + static reference checks (see SC5 row above), not
by `pytest`/`cargo test`-style fixtures.

### What explicitly cannot be validated in this environment, and why

1. **Docker multi-arch build (`linux/amd64,linux/arm64`) and its < 500 MB / < 5 min budget** —
   `docker` binary absent from this sandbox. `[already_established #6]`
2. **kind-based Kubernetes smoke test and its < 30 s pod-startup budget** — `kind`, `kubectl`
   absent. `[already_established #6]`
3. **The QUICKSTART's "clean machine, cold registry" claim** — crates.io returns HTTP 403 here, so a
   cold dependency fetch cannot be timed; the local cargo registry/build cache is already warm from
   this and prior sessions, and Docker (needed for `make services-up`) is absent. Per D-11.2, record
   what *is* measurable (offline-reachable steps only) under stated conditions, not a clean-machine
   figure.
4. **The QUICKSTART's LLM call** (`OpenAIAdapter::from_env()?` executing against the real OpenAI
   API) — no LLM API key present. `[already_established #6]`
5. **Triggering the repaired CI workflow itself** — `gh` is available for *reading* workflow-run
   history (D-16), but dispatching a run, pushing the `release/**` trigger change, or opening a PR
   is an outward-facing action gated the same way as D-03's tag push. This research/plan can author
   and statically validate the YAML; only a live GitHub Actions runner (reachable after a human
   pushes) can prove SC5's CI-driven claims execute.
6. **`cargo build --workspace --no-default-features --offline`** — not run in this research session
   (budget discipline); the planner should schedule it as an explicit verification task per D-06's
   proof obligation ("both must succeed"), not assume it from the default-features build's success.

**Authoring CI configuration is not the same as proving a gate (D-15).** Every row above that says
"authored + statically validated" must be recorded as such in the plan's verification section —
never as "SC5 met."

## Package Legitimacy Audit

Not applicable. This phase installs no new external packages — it edits existing CI YAML, Cargo
manifests, `CHANGELOG.md`, `deny.toml`, and `docs/src/getting-started/quickstart.md`. All action
references cited above (`docker/setup-buildx-action@v4`, `docker/build-push-action@v6`,
`docker/setup-qemu-action@v3`, `helm/kind-action@v1`, `actions/checkout@v5`, `actions/cache@v4`)
are versions **already present and running** in this repository's own workflow files, not newly
introduced — cited with `file:line` above, not sourced from WebSearch or training data.

## Open gaps

1. **Docker build time budget (< 5 min) has no existing precedent anywhere in this repo.** Neither
   `ci.yml` nor `release.yml` times a Docker build today. The plan must author this from scratch
   (epoch-diff pattern modeled on the K8s startup-time check), and must decide whether the 5-minute
   figure applies per-architecture or to the whole multi-arch `linux/amd64,linux/arm64` build
   (QEMU-emulated arm64 on an `ubuntu-latest` amd64 runner is markedly slower than native) — this
   research found no clarifying source (`PROJECT.md`, `ROADMAP.md`, `REQUIREMENTS.md`) that
   disambiguates. **Flagging for the planner or a discuss-phase follow-up, not resolving here.**
2. **`k8s/deployment.yaml` runs a placeholder command (`sleep 3600`) with all readiness/liveness
   probes commented out.** The existing `kubernetes-smoke-test` job's < 30s pod-startup measurement
   is real (kind/kubectl orchestration genuinely executes), but it measures container-scheduling
   time against a trivial placeholder, not application-readiness time against a real HTTP health
   endpoint. Whether SC5's "Kubernetes smoke test inside its startup budget" is satisfied by the
   placeholder-based job as-is, or requires wiring `paladin-web`'s health endpoints into the
   commented-out probes first (arguably new capability, outside this phase's "no new product
   capability" boundary), is unresolved. Recommended in Q1(d): accept the placeholder-based
   reuse and record the probe-wiring gap as a named deferral (owner TBD by the planner — not
   assigned in `04-CONTEXT.md`'s deferred list).
3. **Whether `release/**` should also be added to `integration-tests.yml` and `feature-flags.yml`'s
   commented-out `push:` stanzas** (both carry the identical convention as `ci.yml`'s pre-fix
   stanza) is unresolved — D-14 only names `ci.yml`. Left as a plan-level question in Q1(a).
4. **`cargo build --workspace --no-default-features --offline` was not run in this research
   session** (budget discipline, see Validation Architecture item 6). The default-features build
   was verified clean; the no-default-features leg (part of D-06's proof obligation) must be run by
   the plan/execution phase, not assumed from this research.
5. **Cold-build wall-clock time for `cargo build --examples --offline` under default features was
   not independently measured from a fully clean target directory** — this session's cargo caches
   were already warm from repeated builds. The 20.29s figure reflects incremental recompilation
   after a `touch` of all example source files, not a from-scratch build. If the plan needs a true
   cold-build time budget for the examples job, it should be measured freshly (e.g., `cargo clean`
   first) rather than cited from this research.
