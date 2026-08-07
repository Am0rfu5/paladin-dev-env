# ADR-0023: CLI dependency isolation and the binary/Herald surface

## Status

Accepted

**Date:** 2026-08-06

## Context

ROADMAP criterion 4 requires that a downstream project depending on `paladin` as a library compile
no CLI crates: `cargo tree --lib --no-default-features` must show none of `structopt`, `colored` or
`comfy-table`. Today the shipped `cli` feature (`Cargo.toml:284`) isolates 5 of the 8 dependencies
the Epic 3 PRD classifies as CLI-only, while `structopt = "0.3"` (`Cargo.toml:93`),
`colored = "2.1"` (`Cargo.toml:125`) and `comfy-table = "7.1"` (`Cargo.toml:126`) remain
unconditional root dependencies.

Two independent mechanisms keep them in a library-only build, and both must be addressed:

1. The `paladin` `[[bin]]` (`Cargo.toml:240-242`) carries **no** `required-features`, while its two
   siblings — `paladin-cli` (`Cargo.toml:244-247`) and `paladin-server` (`Cargo.toml:249-252`) — both
   do. `src/main.rs`, the sole `structopt` consumer in the entire tree, is therefore compiled by
   default.
2. `paladin-herald` is an **unconditional** root dependency (`Cargo.toml:22,54`) and declares
   `comfy-table`/`colored` unconditionally in its own manifest with **no `[features]` section at
   all**, so both crates re-enter a library-only build through Herald regardless of what the root
   manifest does.

Phase 7's ledger read the Herald half of this finding as `superseded by shipped code` at the ledger
level, but ROADMAP criterion 4 is stricter than that verdict and names all three crates explicitly —
so the criterion governs, and the ledger row gets amended rather than the criterion narrowed.

[ADR-0019](0019-binary-target-architecture.md) is this decision's precondition: it ratified the
three-binary architecture, named `paladin`'s honest purpose as the legacy content-aggregation
service runner, and found that `structopt`'s only consumer in the tree is `src/main.rs` — so
`structopt` cannot be marked `optional = true` without first deciding `src/main.rs`'s fate (gate it,
migrate it, or retire it). [ADR-0021](0021-cli-application-layer-placement.md) already confirms the
CLI *module* surface is correctly gated: `src/application/mod.rs:57-59`'s `pub mod cli;` **is**
`#[cfg(feature = "cli")]`-gated. The unresolved surface this ADR addresses is the **dependency
declaration layer**, not the module-gating layer.

## Decision

Per D-15, this is **one** ADR recording **one** question — what a library-only consumer compiles —
with two sites.

**Site 1 (the Armory's legacy entry point).** Migrate `src/main.rs` from `structopt` to `clap` v4
derive, and add `required-features = ["cli"]` to the `paladin` `[[bin]]`. Migrate *and* gate, because
either alone is insufficient: gating without migrating leaves `structopt` — a crate whose upstream
declares itself superseded by clap 3+ — as an optional dependency nobody intends to keep; migrating
without gating leaves `clap` unconditional and merely renames the leak. `clap` 4.5.40 is already a
workspace dependency (`Cargo.toml:122`) gated by `cli` (`Cargo.toml:284`), so no new dependency line
is added — only `structopt = "0.3"` (`Cargo.toml:93`) is removed. Stated explicitly, the user-visible
consequence: after this change **`cargo run` no longer builds the `paladin` binary without
`--features cli`.** That is the cost of criterion 4, recorded here and not only in `CHANGELOG.md`.

**Site 2 (Herald's formatters).** Give `paladin-herald` its first `[features]` section, making
`comfy-table` and `colored` optional. The split: the `TableHerald` formatter — whose entire rendering
is `comfy_table` (`crates/paladin-herald/src/table_herald.rs:31`,
`use comfy_table::{Attribute, Cell, CellAlignment, Color, ContentArrangement, Table, presets};`) —
moves behind a feature; the `MarkdownHerald` type stays unconditional, with only its *coloured*
rendering path (`crates/paladin-herald/src/markdown_herald.rs:17`, `use colored::*;`) behind a
feature, because `MarkdownHeraldConfig.include_colors` already exists as a runtime switch and the
uncoloured path is the existing `include_colors: false` behaviour; `JsonHerald` stays unconditional,
needing neither dependency. This is a clean additive split, not a breaking API change: no
`comfy_table::*` or `colored::*` type appears in any `pub fn` signature or `pub` struct field — both
`TableHeraldConfig` and `MarkdownHeraldConfig` use only `usize`, `String`, `bool` and `u8`, and both
dependencies are used only inside function bodies. The root `cli` feature enables both
`paladin-herald` features and marks `colored`/`comfy-table` `optional = true` at the root manifest
(they are genuinely needed by `src/application/cli/`, which is already `cli`-gated per ADR-0021 — so
they become optional, not removed). The root-facade consumer sites gate on the root's own
`feature = "cli"`, since that is the feature DEBT-04's requirement text designates.

**Scope statement — three root-facade consumer sites, not two.** Feature-gating `paladin-herald`
alone is not sufficient: three sites in the root facade must gate to match, or the default library
build fails to *compile* before criterion 4's `cargo tree` command ever runs.
`src/infrastructure/adapters/herald/mod.rs:9-10`
(`pub use paladin_herald::{JsonHerald, MarkdownHerald, TableHerald};` /
`pub use paladin_herald::{json_herald, markdown_herald, table_herald};`) and
`src/application/services/herald/herald_registry.rs:248-250`
(`registry.register("json", …); registry.register("markdown", …); registry.register("table", …);`)
were both found by research; the third — found during planning, absent from `08-RESEARCH.md` — is
`src/config/settings.rs:214,235,244`, whose `Settings::create_default_herald()` unconditionally
imports `TableHerald`/`MarkdownHerald` (`:214`), constructs `MarkdownHerald::with_config(..)`
(`:235`) and constructs `TableHerald::new(..)` (`:244`). Without all three, the default library build
fails to compile and criterion 4 never reaches `cargo tree`. The behavioural consequence of gating
the third site: a configuration naming `herald.default_formatter = "table"` in a build without `cli`
now returns the existing `Unknown formatter '{other}'. Valid options: json, markdown, table` error
path rather than a table Herald — the error the `other =>` match arm already produces for any
unrecognised name, not a new failure mode.

**Criterion-4 proof command, recorded verbatim** —
`cargo build --offline --lib --no-default-features` followed by
`cargo tree --offline --no-default-features | grep -E 'structopt|colored|comfy-table'` —
its output slot is left explicitly unfilled below, in `## Code Locations`, and is not restated here.
Plan 08-08 fills that slot with the captured output once the two sites above are implemented (D-16:
criterion 4 is proved by running the command, not by reading the manifest).

## Considered Options

- **Migrate `src/main.rs` to `clap` v4 *and* gate the `paladin` `[[bin]]`, plus the Herald feature
  split across all three root-facade consumer sites** (accepted) — the only option that satisfies
  criterion 4 without leaving an unmaintained dependency in the tree or breaking the default library
  build's compilation.
- **Gate `src/main.rs` without migrating** (rejected) — keeps `structopt`, whose upstream publicly
  states it is superseded by clap 3+, as an optional dependency nobody intends to keep; a
  half-measure that trades one leak for a maintenance liability.
- **Migrate `src/main.rs` without gating** (rejected) — leaves `clap` unconditional; this merely
  renames the leak from `structopt` to `clap` without ever reaching zero CLI dependencies in a
  library-only build.
- **Retire `src/main.rs`** (rejected) — [ADR-0019](0019-binary-target-architecture.md) has just
  recorded `src/main.rs`'s purpose as the legacy content-aggregator service runner
  (`#[structopt(name = "smartcontent-aggregator")]`). Retiring a binary the previous phase documented
  as purposeful is new scope, not defect closure; whether the `smartcontent-aggregator` service
  runner should survive at all remains an explicitly deferred, still-open question.
- **Root-manifest gating alone, with criterion 4 restated as root-scoped** (rejected) — this was
  D-14's stated fallback if Herald's `colored`/`comfy_table` usage proved threaded through public
  trait signatures rather than confined to function bodies. Research **disproved that precondition**:
  the signature check found no `comfy_table::*`/`colored::*` type in any `pub fn` signature or `pub`
  struct field, so the clean additive split is feasible and this fallback does not apply.

## Code Locations

- `Cargo.toml:93` — `structopt = "0.3"`, removed by this decision.
- `Cargo.toml:122` — `clap = { version = "4.5.40", features = ["derive", "cargo", "env"], optional = true }`, already vendored.
- `Cargo.toml:125` — `colored = "2.1"`, marked `optional = true` by this decision.
- `Cargo.toml:126` — `comfy-table = "7.1"`, marked `optional = true` by this decision.
- `Cargo.toml:22` — `paladin-herald = { version = "0.7.0", path = "crates/paladin-herald" }` in
  `[workspace.dependencies]`.
- `Cargo.toml:54` — `paladin-herald = { workspace = true }`, the unconditional root dependency edge.
- `Cargo.toml:240-242` — `[[bin]] name = "paladin"`, `path = "src/main.rs"`, no `required-features`
  today; gains `required-features = ["cli"]`.
- `Cargo.toml:244-247` — `[[bin]] name = "paladin-cli"`, `required-features = ["cli"]` (existing
  pattern this decision copies).
- `Cargo.toml:249-252` — `[[bin]] name = "paladin-server"`, `required-features = ["web-server"]`
  (existing sibling pattern).
- `Cargo.toml:284` — `cli = ["dep:clap", "dep:dialoguer", "dep:indicatif", "dep:console",
  "dep:serde_yaml"]`, extended to add `dep:colored`, `dep:comfy-table`, and the chosen
  `paladin-herald` feature name(s).
- `src/main.rs:5` — `use structopt::StructOpt;`, replaced with `use clap::Parser;`.
- `src/main.rs:7-12` — the `#[derive(StructOpt, Debug)] #[structopt(name = "smartcontent-aggregator")]
  struct Opt { #[structopt(short, long, default_value = "config.yml")] config: String, }` block,
  translated to `clap` v4 derive (`#[derive(Parser, Debug)] #[command(...)]` / `#[arg(...)]`).
- `src/main.rs:26` — `let opt = Opt::from_args();`, becomes `Opt::parse()`.
- `src/main.rs:46,52,58` — the three `Opt::from_iter(&[...])` test-fn calls, become
  `Opt::parse_from([...])`.
- `crates/paladin-herald/Cargo.toml` `[dependencies]` — `comfy-table = "7.1"`, `colored = "2.1"`,
  unconditional; the file has **no `[features]` section** today. This decision adds one.
- `crates/paladin-herald/src/lib.rs:19-25` — `pub mod json_herald; pub mod markdown_herald;
  pub mod table_herald;` plus the three `pub use …::{Json,Markdown,Table}Herald;` re-exports, split
  to match the new feature gates.
- `crates/paladin-herald/src/table_herald.rs:31` —
  `use comfy_table::{Attribute, Cell, CellAlignment, Color, ContentArrangement, Table, presets};`,
  the entirety of `TableHerald`'s rendering dependency.
- `crates/paladin-herald/src/markdown_herald.rs:17` — `use colored::*;`, `MarkdownHerald`'s coloured
  rendering path.
- `src/infrastructure/adapters/herald/mod.rs:9-10` —
  `pub use paladin_herald::{JsonHerald, MarkdownHerald, TableHerald};` /
  `pub use paladin_herald::{json_herald, markdown_herald, table_herald};`, both currently ungated;
  split so `JsonHerald` stays unconditional and `MarkdownHerald`/`TableHerald` gate on the chosen
  feature name(s).
- `src/application/services/herald/herald_registry.rs:248-250` —
  `registry.register("json", Arc::new(JsonHerald::new()));
  registry.register("markdown", Arc::new(MarkdownHerald::new()));
  registry.register("table", Arc::new(TableHerald::default()));` inside `Default::default()`,
  currently ungated; `"json"` stays unconditional, `"markdown"`/`"table"` gate to match.
- `src/config/settings.rs:214` —
  `use crate::infrastructure::adapters::herald::{JsonHerald, MarkdownHerald, TableHerald};` inside
  `Settings::create_default_herald()`, ungated — the third facade consumer site, found during
  planning.
- `src/config/settings.rs:235` — `let herald = MarkdownHerald::with_config(markdown_config);` inside
  the `"markdown" =>` match arm.
- `src/config/settings.rs:244` — `let herald = TableHerald::new(table_config);` inside the
  `"table" =>` match arm.
- `src/application/mod.rs:57-59` — `#[cfg(feature = "cli")] pub mod cli;`, already correctly gated
  (ADR-0021) and unaffected by this decision; cited as the precondition this ADR does not disturb.
- `Dockerfile:33` — `RUN cargo build --release --workspace --bin paladin`, no `--features cli`; must
  gain the flag or the image build breaks.
- `Dockerfile.chef:74` — the same `cargo build --release --workspace --bin paladin` invocation in
  the chef-cache build stage; same fix required.
- `.github/workflows/feature-flags.yml:144` — the step literally named "Verify paladin binary builds
  without cli feature" (`run: cargo build --bin paladin`), whose assertion inverts once
  `required-features = ["cli"]` lands.
- `docs/src/deployment/docker.md:135,146,156` — the mdbook source documenting the same
  `cargo build --release --workspace --bin paladin` / `COPY … /usr/local/bin/paladin` /
  `CMD ["/usr/local/bin/paladin"]` sequence as source-of-truth prose; must move with the Dockerfile
  changes or go stale silently.

Criterion-4 proof, recorded verbatim:

```
cargo build --offline --lib --no-default-features
cargo tree --offline --no-default-features | grep -E 'structopt|colored|comfy-table'
```

Output: PENDING — filled by plan 08-08 once the code lands.

## Code Conformance

must change

**Phase 8 itself** is the named executor. Plan 08-07 performs both sites' manifest and source
changes: the `src/main.rs` `clap` migration and the `paladin` `[[bin]]` gate (Site 1), and
`paladin-herald`'s new `[features]` section plus the three root-facade consumer-site gates (Site 2).
Plan 08-08 performs the downstream build-surface sweep named in `## Code Locations`
(`Dockerfile:33`, `Dockerfile.chef:74`, `.github/workflows/feature-flags.yml:144`,
`docs/src/deployment/docker.md:135,146,156`), the `CHANGELOG.md` entries for both user-visible
changes, and the criterion-4 proof — replacing this ADR's unfilled output slot with the literal
`cargo tree` output.

## Downstream Consumers

- Phase 8 / plan 08-07 — performs both manifest/source sites this ADR's `must change` requires.
- Phase 8 / plan 08-08 — performs the downstream build-surface sweep, the `CHANGELOG.md` entries, and
  fills this ADR's pending criterion-4 proof line.
- Phase 8 / plan 08-09 — adds this ADR's row to `.planning/decisions/PROMOTION.md`'s numbering index
  and advances its "Next free ADR number" line to 0024, adds the corresponding row to `PROJECT.md`'s
  Key Decisions table, flips the DEBT-04 checkbox in `REQUIREMENTS.md`, and amends the Milestone 4-6
  ledger row Phase 7 recorded as `superseded by shipped code` on the Herald half — this decision
  amends that row rather than narrowing ROADMAP criterion 4 to match it.
- Phase 15 — the `cargo tree`-based dependency-allowlist check in CI, from ADR-0015. This ADR runs
  `cargo tree` once as proof; nothing enforces it on every build until Phase 15 lands.
- Phase 16 — the user-facing binary-architecture mdbook page, from ADR-0019.
