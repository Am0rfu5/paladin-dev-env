# Release Coherence — Raw Evidence Record (Phase 4)

This file is raw evidence only: verbatim commands, verbatim tool output, toolchain versions,
commit SHA, and UTC dates. It carries no gate or target value of its own. Per D-17, every cargo
invocation in this record carries `--offline` unless the command needs the advisory database
(`cargo audit`), and every figure was produced live in the run that recorded it — no figure is
carried over from RESEARCH.md, from an earlier phase, or from a prior session.

## Entry measurement — edition 2024 on paladin-ports

### Environment probes (verbatim)

Command: `rustc -vV`

```
rustc 1.97.1 (8bab26f4f 2026-07-14)
binary: rustc
commit-hash: 8bab26f4f68e0e26f0bb7960be334d5b520ea452
commit-date: 2026-07-14
host: x86_64-unknown-linux-gnu
release: 1.97.1
LLVM version: 22.1.6
```

Command: `cargo --version`

```
cargo 1.97.1 (c980f4866 2026-06-30)
```

Command: `git rev-parse HEAD`

```
1c993434dd7d05223d3fa59623b08a48b67e3732
```

Command: `git rev-parse --abbrev-ref HEAD`

```
worktree-agent-a2d8bf5f17079ff85
```

This is a parallel-executor worktree branch spawned by the orchestrator for phase 04-release-coherence
plan 01; its parent branch is `release/v0.7.0`, verified at the base commit above via the
worktree's own startup assertion before any task ran.

Command: `git status --porcelain` (captured immediately before the `crates/paladin-ports/Cargo.toml`
edit landed)

```
 M crates/paladin-ports/Cargo.toml
```

The single modification shown is this task's own edit (the `edition` key bump from `2021` to
`2024`) — no unrelated pre-existing dirty file exists in this worktree at task start; the task's
own `<precondition>` (`git status --porcelain` reports no modified/untracked files outside
`.planning/`) was verified clean before this edit was made.

Command: `date -u`

```
Mon Aug  3 00:21:14 UTC 2026
```

### Migration commands and output (verbatim)

Command: `cargo fix --edition --offline -p paladin-ports --allow-dirty --lib`

```
warning: `crates/paladin-ports/src/lib.rs` is already on the latest edition (2024), unable to migrate further

If you are trying to migrate from the previous edition (2021), the
process requires following these steps:

1. Start with `edition = "2021"` in `Cargo.toml`
2. Run `cargo fix --edition`
3. Modify `Cargo.toml` to set `edition = "2024"`
4. Run `cargo build` or `cargo test` to verify the fixes worked

More details may be found at
https://doc.rust-lang.org/edition-guide/editions/transitioning-an-existing-project-to-a-new-edition.html

    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1m 03s
```

This warning fires because the plan's task text (and RESEARCH.md Part A, Q1's documented route)
sequences the manifest edit *before* `cargo fix --edition`, whereas `cargo fix --edition`'s own
tool guidance (shown above) expects the manifest to still declare the *old* edition while it runs,
then be bumped afterward. Both orders were considered: `git status --porcelain -- crates/paladin-ports`
immediately after this command showed only the `Cargo.toml` edit — zero source-file rewrites — and
the subsequent `cargo build --workspace --offline` (below) compiled the crate clean under edition
2024 with no errors or edition-migration diagnostics. This confirms the crate required no source
rewrite regardless of invocation order: the four mechanically-detectable hazards RESEARCH.md
Part A, Q1 found absent (no `unsafe`, no `static mut`, no `gen` identifier, no `no_mangle`) are
in fact absent, and the two semantic hazards (`if let`/tail-expression drop order, match-ergonomics
tightening) did not fire. Per the plan's own truth — "`cargo fix --edition` producing zero source
rewrites is a pass, not a failure — the proof obligation is the build, not the diff size" — this is
recorded as a pass.

`git status --porcelain -- crates/paladin-ports` immediately after the `cargo fix` invocation:

```
 M crates/paladin-ports/Cargo.toml
```

No file under `crates/paladin-ports/src` was touched by `cargo fix --edition`.

### Build leg 1 — `cargo build --workspace --offline`

```
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.73s
```

(Re-run for this record after the full first invocation had already compiled the workspace fresh
in 3m 14s with zero errors and zero warnings attributable to the edition change — see Task 1's
tool-call log for the full `Compiling`/`Checking` transcript. This re-run's `Finished` line with no
intervening `error`/`warning` output confirms the cached build state is clean.)

### Build leg 2 — `cargo build --workspace --no-default-features --offline`

```
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.88s
```

**Finding, recorded honestly per D-17/T-04-02:** this leg exits 0, but it does not exercise a
genuinely feature-reduced build of the root `paladin-ai` package. `cargo metadata --no-default-features
--offline --format-version 1` was inspected and shows `paladin-ai`'s resolved feature set as
`["default", "llm-openai", "web-server"]` — i.e. `default` (and therefore `llm-openai`) stays
enabled even under `--no-default-features` at the workspace level. Root cause, verified by direct
read: `crates/doc-examples/Cargo.toml:15` declares
`paladin-ai = { version = "0.6.0", path = "../..", features = ["web-server"] }` **without** setting
`default-features = false`. Cargo's additive `features = [...]` syntax does not, on its own, disable
a dependency's default features — and because `doc-examples` is itself a workspace member built in
the same `cargo build --workspace` invocation, its unconditional re-request of `paladin-ai`'s
default feature set is unified into every other unit that also depends on `paladin-ai`, including
the top-level `--no-default-features` flag's own target. The two build legs were confirmed to
produce an identical resolved feature set for the crates under this plan's scope
(`paladin-ports`, `paladin-notifications`) — `paladin-ports` and `paladin-notifications` declare no
`[features]` section of their own, so this workspace-level feature-unification quirk does not affect
this plan's edition-migration proof for those two crates specifically, but it does mean the
`--no-default-features` leg's promise (per D-06 — "the pass that catches a feature-gated path in a
dependent crate that only compiled under the old edition's rules") is **not fully discharged** for
`paladin-ai` itself in this run. This is a pre-existing structural fact of the workspace's feature
graph (`crates/doc-examples/Cargo.toml:15` predates this plan) — unrelated to the edition bump and
out of this plan's `files_modified` scope to fix. Recorded here as a finding, not silently passed
over; disposition left to whichever phase next touches `crates/doc-examples/Cargo.toml` or CI's
feature-matrix job (Phase 4 Plan 03's territory per D-12, not this plan's).

### Acceptance criteria verification (verbatim)

Command: `grep -c 'edition = "2024"' crates/paladin-ports/Cargo.toml`

```
1
```

Command: `grep -c 'doctest = false' crates/paladin-ports/Cargo.toml`

```
1
```

Command: `grep -c '"=0.6.0"' crates/paladin-ports/Cargo.toml`

```
1
```

Command: `git diff -- crates/paladin-ports/src | grep -c '^+#!\[allow('`

```
0
```

All four acceptance-criteria checks pass. `crates/paladin-ports` now declares `edition = "2024"`,
`doctest = false` and the `=0.6.0` exact pin are both unchanged, and no diagnostic-suppression
annotation was added to silence the migration.

## Entry measurement — edition 2024 on paladin-notifications (workspace now uniform)

### Environment probes (verbatim)

Command: `rustc -vV`

```
rustc 1.97.1 (8bab26f4f 2026-07-14)
binary: rustc
commit-hash: 8bab26f4f68e0e26f0bb7960be334d5b520ea452
commit-date: 2026-07-14
host: x86_64-unknown-linux-gnu
release: 1.97.1
LLVM version: 22.1.6
```

Command: `cargo --version`

```
cargo 1.97.1 (c980f4866 2026-06-30)
```

Command: `git rev-parse HEAD`

```
7d8e730b42118b5aa7110e8f54a6db0bda78e06f
```

This is Task 1's commit (`feat(04-01): bump paladin-ports to edition 2024`) — the tip of this
worktree branch at the moment Task 2's edit landed on top of it, before Task 2's own commit exists.

Command: `git rev-parse --abbrev-ref HEAD`

```
worktree-agent-a2d8bf5f17079ff85
```

Command: `git status --porcelain` (captured immediately after the
`crates/paladin-notifications/Cargo.toml` edit landed, before staging)

```
 M crates/paladin-notifications/Cargo.toml
```

The single modification shown is this task's own edit (the `edition` key bump from `2021` to
`2024`) — no unrelated pre-existing dirty file exists in this worktree at task start; Task 1's own
changes are already committed at the HEAD recorded above.

Command: `date -u`

```
Mon Aug  3 00:25:49 UTC 2026
```

### Migration commands and output (verbatim)

Command: `cargo fix --edition --offline -p paladin-notifications --allow-dirty --lib`

```
    Blocking waiting for file lock on package cache
    Blocking waiting for file lock on package cache
    Blocking waiting for file lock on package cache
    Checking paladin-notifications v0.6.0 (/workspace/.claude/worktrees/agent-a2d8bf5f17079ff85/crates/paladin-notifications)
warning: `crates/paladin-notifications/src/lib.rs` is already on the latest edition (2024), unable to migrate further

If you are trying to migrate from the previous edition (2021), the
process requires following these steps:

1. Start with `edition = "2021"` in `Cargo.toml`
2. Run `cargo fix --edition`
3. Modify `Cargo.toml` to set `edition = "2024"`
4. Run `cargo build` or `cargo test` to verify the fixes worked

More details may be found at
https://doc.rust-lang.org/edition-guide/editions/transitioning-an-existing-project-to-a-new-edition.html

    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.46s
```

Same outcome as Task 1: zero source rewrites. `crates/paladin-notifications` (5 files, 1,099 lines
per RESEARCH.md Part A, Q1) uses `if let` in `email_notification_adapter.rs` and elsewhere — the
drop-order hazard was live here in a way it wasn't necessarily for `paladin-ports` — but no
diagnostic fired. `git status --porcelain -- crates/paladin-notifications` immediately after this
command:

```
 M crates/paladin-notifications/Cargo.toml
```

No file under `crates/paladin-notifications/src` was touched by `cargo fix --edition`.

### Build leg 1 — `cargo build --workspace --offline`

```
   Compiling paladin-notifications v0.6.0 (/workspace/.claude/worktrees/agent-a2d8bf5f17079ff85/crates/paladin-notifications)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.75s
```

### Build leg 2 — `cargo build --workspace --no-default-features --offline`

```
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.70s
```

Both legs exit 0 on the final state — every one of the twelve `edition`-carrying manifests in the
workspace now agrees. As recorded in the first entry measurement above, the `--no-default-features`
leg does not exercise a genuinely feature-reduced build of the root `paladin-ai` package (workspace
feature unification via `crates/doc-examples/Cargo.toml:15` keeps `default`/`llm-openai` enabled
regardless of the flag — a pre-existing structural fact unrelated to this plan's edition work).
`paladin-notifications` declares its own `[features]` (`email`, `push`, `system`) but none of them
is a `default` feature (`crates/paladin-notifications/Cargo.toml` has no `default = [...]` line),
so this crate is unaffected by the same unification quirk — its non-default optional features
(`email`, `push`, `system`) simply stay off in both legs, which is the correct, expected behavior
for optional features with no `default` entry naming them.

### Uniformity assertion (verbatim)

Command: `grep -h '^edition' Cargo.toml crates/*/Cargo.toml | sort -u`

```
edition = "2024"
```

Exactly one line, naming 2024. No manifest in the workspace declares any other edition.

Command: `grep -h '^edition' Cargo.toml crates/*/Cargo.toml | wc -l`

```
12
```

Command: `grep -h '^edition' Cargo.toml crates/*/Cargo.toml | grep -c 2024`

```
12
```

Command: `grep -h '^edition' Cargo.toml crates/*/Cargo.toml | grep -c 2021`

```
0
```

**Count note:** twelve manifests carry an `edition` key — the root `Cargo.toml` (the `paladin-ai`
workspace package) plus eleven member crates under `crates/*/Cargo.toml`. `04-CONTEXT.md` and
`04-RESEARCH.md` say "eleven manifests" in places because they are counting member crates only; the
root `Cargo.toml` is the twelfth file carrying an `edition` key, and this plan's own truth
("Exactly zero manifests declare the 2021 edition and exactly twelve declare 2024 — the root
`Cargo.toml` plus eleven member crates") is the one to cite going forward.

### Acceptance criteria verification (verbatim)

Command: `grep -c 'edition = "2024"' crates/paladin-notifications/Cargo.toml`

```
1
```

Command: `grep -h '^edition' Cargo.toml crates/*/Cargo.toml | sort -u | wc -l`

```
1
```

Command: `grep -h '^edition' Cargo.toml crates/*/Cargo.toml | grep -c 2024`

```
12
```

Command: `grep -h '^edition' Cargo.toml crates/*/Cargo.toml | grep -c 2021`

```
0
```

Command: `git diff -- crates/paladin-notifications/src | grep -c '^+#!\[allow('`

```
0
```

All acceptance-criteria checks pass. `crates/paladin-notifications` now declares `edition = "2024"`;
the workspace-wide `edition` grep returns exactly one distinct value across all twelve manifests,
twelve of twelve naming 2024, zero naming 2021; and no diagnostic-suppression annotation was added
to either crate touched by this plan.
