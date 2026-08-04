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
