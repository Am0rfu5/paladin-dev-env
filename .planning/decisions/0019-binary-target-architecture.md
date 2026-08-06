# ADR-0019: Binary-target architecture and per-binary purpose

## Status

Accepted

**Date:** 2026-08-06

## Context

Milestone 4 Epic 3's `prd-cli-isolation.md` Q1 ("Binary Target Architecture Decision") offered four
options for the relationship between `src/main.rs` (the default `paladin` binary) and
`src/bin/paladin-cli.rs`, and recorded "**Status:** User selected Option D — requires architecture
review" (`prd-cli-isolation.md:446`), naming that review as a blocker for "Task 3.3: Update Binary
Entry Points" (`:555`) and a project risk ("Binary target architecture decision delayed", `:573`). No
architecture-review record exists anywhere in the ingest set. FR3.3 asked each binary target to have
its relationship and intended use case documented (`:134`), and FR9.3 asked for that documentation
deliverable "after architecture review" (`:204`) — neither was produced.

The tree answers the question de facto. `Cargo.toml:240-252` declares three `[[bin]]` stanzas, which
is Option A ("keep `src/main.rs` as-is, `paladin-cli` as a separate tool") extended by a third
target. This ADR ratifies that shipped shape and states the purpose FR3.3/FR9.3 asked for, closing
the architecture review Milestone 4 made a blocker and never produced.

## Decision

**The three binary targets are ratified, in `Cargo.toml` declaration order, each with a distinct
stated purpose:**

- **`paladin`** (`src/main.rs`, no `required-features`) — the honest purpose read from the source
  during this task: it is the **pre-Paladin content-aggregation service runner**, not an
  agent-orchestration binary. It declares `#[structopt(name = "smartcontent-aggregator")]`
  (`src/main.rs:8`), loads `config.yml` through `Settings::load_from_file` (`src/main.rs:27`), and
  calls `paladin::config::setup::setup_and_run` (`src/main.rs:37`). The stale application name is
  recorded as a finding, not tidied away — this binary predates the Paladin agent-orchestration
  domain and its `structopt` self-identification says so plainly.
- **`paladin-cli`** (`src/bin/paladin-cli.rs`, `required-features = ["cli"]`) — the Armory developer
  CLI: agent, battalion, arsenal, and maneuver subcommands built on `clap`.
- **`paladin-server`** (`src/bin/paladin-server.rs`, `required-features = ["web-server"]`) — the Axum
  HTTP API server, serving the agent-execution API over HTTP with graceful shutdown.

The declaration order in `Cargo.toml` is the recorded order, and the three stanzas are distinct
targets with distinct feature gates, so no two ever share a purpose.

**This answer also exposes a coupling that re-scopes Phase 8's CLI-isolation work.** Verified this
task: `grep -rn structopt src/ crates/` returns three hits, all confined to one file —
`src/main.rs:5,8,10`. `structopt`'s only consumer in the entire tree is the un-gated `paladin`
binary; because that binary carries no `required-features` gate, `structopt` cannot be marked
`optional = true` in `Cargo.toml` without first deciding `src/main.rs`'s fate — gate it, migrate it
to `clap`, or retire it. The recorded "three-line fix" for CLI dependency isolation (mark
`structopt`, `colored`, and `comfy-table` optional and add them to the `cli` feature —
`INGEST-CONFLICTS.md:321`) is therefore **wrong for one of its three lines**: this ADR's answer is
its precondition.

The second half of the same finding, confirmed by reading the manifest during this task:
`crates/paladin-herald/Cargo.toml:22-23` declares `comfy-table = "7.1"` and `colored = "2.1"`
unconditionally, and the file has **no `[features]` section at all**. `paladin-herald` is itself a
required, non-optional workspace dependency of the root package (`Cargo.toml:22,54`), so these two
crates always compile into the root package regardless of the root manifest's own `cli` feature —
even though the root `Cargo.toml` also carries them unconditionally today (`Cargo.toml:125-126`,
neither marked `optional = true`). Gating the root manifest's copies alone cannot satisfy FR5.4's
"zero CLI code or dependencies in `cargo tree --lib --no-default-features`" criterion
(`prd-cli-isolation.md:158,403`) while `paladin-herald` re-introduces both unconditionally. The M4
`dependency-matrix.md` audit classified both as CLI-only and was correct *at the time*;
`paladin-herald` did not exist until it was extracted from that surface later, in Milestone 8. The
honest ledger verdict for this residue is `superseded by shipped code`, not `genuinely outstanding`.

Separately confirmed this task, and recorded so the finding above is not overstated: the CLI
*module* surface itself is already correctly gated — `src/application/mod.rs:57-59` gates
`pub mod cli;` behind `#[cfg(feature = "cli")]`, and `src/lib.rs:155-156` gates the corresponding
`pub use application::cli;` re-export the same way. The unresolved surface is the **dependency
declaration layer** (`structopt`, `colored`, `comfy-table` as unconditional `Cargo.toml` entries),
not the module-gating layer — FR1 (module gating) is satisfied; FR2/FR5 (dependency isolation) is
not.

Whether `src/main.rs` should survive as the legacy content aggregator is new scope this ADR exposes
but does not own — it belongs to Phase 8's CLI-isolation work or its own phase.

## Considered Options

- Ratify Option A extended — three binary targets, each with a stated purpose (accepted). This is
  what the shipped tree already implements; it closes the architecture review Q1 made a blocker and
  Milestone 4 never produced.
- Collapse to a single binary with subcommands (rejected) — a code change this phase is forbidden to
  make, against a shape three shipped `[[bin]]` stanzas already establish.
- Record the question as still formally open, pending the never-produced architecture review
  (rejected) — the review has been owed since Milestone 4 and the tree has already answered it; a
  Phase 7 whose whole point is recording de facto answers cannot leave this one un-recorded.
- Invent a forward-looking, tidier purpose for `paladin` (rejected explicitly) — the honest answer is
  the legacy content aggregator, and the stale `smartcontent-aggregator` name is part of the record,
  not a defect to smooth over.

## Code Locations

- `Cargo.toml:240-242` — `[[bin]] name = "paladin"`, `path = "src/main.rs"`, no `required-features`.
- `Cargo.toml:244-247` — `[[bin]] name = "paladin-cli"`, `required-features = ["cli"]`.
- `Cargo.toml:249-252` — `[[bin]] name = "paladin-server"`, `required-features = ["web-server"]`.
- `src/main.rs:8` — `#[structopt(name = "smartcontent-aggregator")]`, the stale application name.
- `src/main.rs:27` — `Settings::load_from_file(&opt.config)`, the settings-load call.
- `src/main.rs:37` — `setup_and_run(config).await`, the setup-call.
- `src/bin/paladin-cli.rs:1-17` — the Armory CLI's `clap`-based entry point and subcommand set.
- `src/bin/paladin-server.rs:1-14` — the Axum HTTP API server's module doc and entry point.
- `Cargo.toml:284` — `cli = ["dep:clap", "dep:dialoguer", "dep:indicatif", "dep:console", "dep:serde_yaml"]`.
- `Cargo.toml:276` — `web-server = ["dep:paladin-web", "dep:axum"]`.
- `Cargo.toml:93` — `structopt = "0.3"`, unconditional root dependency.
- `Cargo.toml:125-126` — `colored = "2.1"` and `comfy-table = "7.1"`, unconditional root dependencies.
- `crates/paladin-herald/Cargo.toml:22-23` — `comfy-table` and `colored`, unconditional, no
  `[features]` section anywhere in the file.
- `src/application/mod.rs:57-59` and `src/lib.rs:155-156` — the CLI module and its re-export, both
  correctly feature-gated behind `cli`.
- `.project/Milestone_4-Refactor-Crates-Features/Epic_3/prd-cli-isolation.md:112` — FR2.1.
- `.project/Milestone_4-Refactor-Crates-Features/Epic_3/prd-cli-isolation.md:158,396-403` — FR5.4 and
  §8.3.
- `.project/Milestone_4-Refactor-Crates-Features/Epic_3/prd-cli-isolation.md:430,446,555` — Q1, its
  "Option D" status, and the Task 3.3 blocker.
- `.project/Milestone_4-Refactor-Crates-Features/Epic_3/prd-cli-isolation.md:127-137,204` — FR3 and
  FR9.3.

## Code Conformance

must change

The user-facing binary-architecture mdbook page FR9.3 asks for does not exist and is still owed.
This ADR plus its ledger row satisfy FR9.3's *record* — the decision now exists, cited and dated —
but not its *user-facing deliverable*. **Phase 16's documentation-currency work** is the executor of
that deliverable. No code changes result from this ADR: `src/main.rs`'s fate, `structopt`'s
optionality, and `paladin-herald`'s feature-gating are all outside this phase's record-only boundary.

## Downstream Consumers

- **Phase 16** — writes the user-facing binary-architecture mdbook page FR9.3 asked for; this ADR is
  its source record, not a substitute for it.
- **Phase 8's CLI-isolation requirement** — receives this ADR's two-part finding by name: the
  `structopt`/`src/main.rs` precondition (structopt cannot be gated until `src/main.rs`'s fate is
  decided) and the `paladin-herald` hole (gating the root manifest's `colored`/`comfy-table` entries
  alone cannot satisfy FR5.4 while `paladin-herald` re-introduces both unconditionally). Phase 8's
  recorded "three-line fix" must be re-scoped accordingly.
- **Plan 07-12** — the ledger rows for `REQ-binary-target-config`, `REQ-cli-docs`,
  `REQ-cli-dependency-isolation`, and `REQ-library-only-build` cite this ADR for their verdicts.
- **Plan 07-13** — adds this ADR's row to `.planning/decisions/PROMOTION.md`'s numbering index.
