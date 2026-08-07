# Phase 8: Verified Defect Closure - Pattern Map

**Mapped:** 2026-08-06
**Files analyzed:** ~24 (2 new ADRs, 1 new `[features]` section, ~1 optional COVERAGE.md, ~17
modified code/manifest/CI/script files, ~6 modified records)
**Analogs found:** 24 / 24 — this is a defect-closure phase; nearly every "new" file has a mandatory,
exact analog already named in CONTEXT.md/RESEARCH.md, and every code change is a surgical edit to an
existing file with a sibling pattern already in the same file or a neighboring one.

## File Classification

| New/Modified File | Role | Data Flow | Closest Analog | Match Quality |
|---|---|---|---|---|
| `.planning/decisions/0022-*.md` (deprecation withdrawal) | config/record (ADR) | transform (decision → doc) | `.planning/decisions/0019-binary-target-architecture.md`, `0016-port-value-type-ownership.md` | exact |
| `.planning/decisions/0023-*.md` (CLI dependency isolation) | config/record (ADR) | transform | same two ADRs | exact |
| `crates/paladin-herald/Cargo.toml` `[features]` section | config | build-time gating | `crates/paladin-llm/Cargo.toml:17-24` | role-match (closest `[features]` block in workspace: optional deps + a feature enabling them) |
| `.planning/phases/08-verified-defect-closure/COVERAGE.md` | config/record | transform | `.planning/phases/07-workspace-ground-truth-recorded-answers/COVERAGE.md` | exact (same "no external API" declaration shape) — only needed if this phase's coverage-floor re-check surfaces something to record; otherwise skip, do not fabricate a matrix |
| `Cargo.toml` (root) — remove `structopt`, gate `colored`/`comfy-table`, extend `cli` feature, add `required-features` to `paladin` `[[bin]]` | config | build-time gating | same file's own `paladin-cli`/`paladin-server` `[[bin]]` stanzas (`:244-252`) | exact (in-file sibling) |
| `src/main.rs` (structopt → clap v4) | binary entry point / CLI parser | request-response (argv → config) | `src/bin/paladin-cli.rs` (existing clap v4 derive usage) | exact — RESEARCH.md already produced the full verified translation, see below |
| `src/infrastructure/adapters/herald/mod.rs` | module re-export / adapter facade | transform | `src/infrastructure/adapters/mod.rs:4-13` (sibling `content-processing`/`notifications` gating) | exact |
| `src/application/services/herald/herald_registry.rs` | service / registry (CRUD-like register/get) | event-driven (registration) | its own `Default::default()` construction block; gating pattern copied from `infrastructure/adapters/mod.rs` | role-match (gating pattern lives one file over; construction site is in this file) |
| `crates/paladin-core/src/platform/container/token_usage.rs` | model (value type) | transform | `crates/paladin-core/src/platform/container/battalion/mod.rs:496-524` (the richer duplicate being absorbed) | exact (absorb target's own derives/impls) |
| `crates/paladin-core/src/platform/container/battalion/mod.rs` (TokenUsage → re-export) | model (value type) | transform | `crates/paladin-core/src/platform/container/herald.rs:28` (intra-crate `pub use`) | exact |
| `crates/paladin-llm/src/llm_analysis_service.rs` (TokenUsage → re-export) | model (value type) | transform | `crates/paladin-ports/src/output/llm_port.rs:671` (cross-crate `pub use`) | exact |
| `crates/paladin-ports/Cargo.toml` (`[lib] doctest = false` removal) | config | build-time gating | n/a — two-line deletion, no analog needed | trivial |
| `.github/workflows/ci.yml` (`:172,182,187,226`) | CI config | pipeline/batch | in-file: `api-surface` job (`:140-190`) vs. `test` job (`:191-`) — edits stay within each job's own existing step shape | exact (in-file) |
| `scripts/check-api-surface.sh`, `scripts/extract-public-api.sh` (default path literals) | utility / CI script | file-I/O | n/a — one-line default-value edits in the same files | trivial |
| `scripts/check-deprecations.sh` (make it fail; scan `crates` too) | utility / CI script (gate) | file-I/O + batch (grep across tree) | `scripts/check-doc-config.sh` and `scripts/check-api-surface.sh` (both `set -euo pipefail`, meaningful non-zero exit, no swallowed failure) | role-match |
| `.planning/ledgers/milestone-04-06.md` (5 rows amended) | record | transform (in-place amendment) | `.planning/ledgers/milestone-01.md` (own prior in-place amendments, Phase 2/3/4) | exact |
| `.project/` 6 documents (`DEPRECATIONS.md` + 5 requirement sources) | record (annotation) | transform | `.project/Milestone_4-Refactor-Crates-Features/Epic_1/prd-expand-feature-flags.md` (Phase 7's dated correction banner + inline `~~struck~~`/**Corrected** markup) | exact |
| `.planning/decisions/PROMOTION.md`, `PROJECT.md`, `REQUIREMENTS.md`, `CHANGELOG.md` | record | transform | Phase 7's own close-out edits to the same four files (07-13) | role-match |

## Pattern Assignments

### `.planning/decisions/0022-*.md` and `0023-*.md` (ADR, transform)

**Analog:** `.planning/decisions/0016-port-value-type-ownership.md` and
`.planning/decisions/0019-binary-target-architecture.md` (both read in full).

**Skeleton — exactly these seven `##` headings, in this order, no YAML frontmatter:**
```markdown
# ADR-00NN: <Title>

## Status

Accepted

**Date:** 2026-08-06

## Context

<why the question exists; what conflicting sources say; precedence-order framing if relevant>

## Decision

<the ratified answer, stated in the medieval-military ubiquitous language, with a "target, stated
explicitly" paragraph naming file:line for every code site the decision touches>

## Considered Options

- <accepted option> (accepted) — <why>
- <rejected option 1> (rejected) — <why>
- <rejected option 2> (rejected) — <why>

## Code Locations

- `path/to/file.rs:LINE` — <what's there>
- ...

## Code Conformance

must change

<which phase/plan is the named executor, and exactly what "must change" means — no vague "needs
updating">

## Downstream Consumers

- Phase N / ITEM — <what it inherits from this ADR>
- Plan NN-NN — <what ledger/PROMOTION.md bookkeeping it drives>
```

**Concrete opening line format** (copy verbatim shape, substitute number/title):
```markdown
# ADR-0016: Port value-type ownership
```
```markdown
# ADR-0019: Binary-target architecture and per-binary purpose
```

**"Status" block exact shape** (`0019` lines 3-7):
```markdown
## Status

Accepted

**Date:** 2026-08-06
```

**"Code Conformance" — "must change" with a named phase executor** (`0016` lines 100-108):
```markdown
## Code Conformance

must change

Phase 8 / DEBT-05 is the executing requirement. The required change is collapsing the two duplicate
`TokenUsage` definitions (`battalion/mod.rs:497` and `llm_analysis_service.rs:51`) into re-exports of
the canonical `paladin-core` definition (`token_usage.rs:13`).
```
ADR-0022/0023 are new in this corpus: they are the *first* ADRs whose executing phase is their own
(D-22) — so their "Code Conformance" section should read "must change" with **Phase 8 itself** named,
not a future phase, matching the two analogs' structure but pointing inward.

**PROMOTION.md update** — per D-22, after authoring 0022/0023, advance the "next free" line the same
way `0019`'s own Downstream Consumers section describes it happening (`0016` lines 119-121, `0019`
line 140): a ledger/PROMOTION.md row added in the close-out plan, not in the ADR itself.

---

### `crates/paladin-herald/Cargo.toml` `[features]` section (config, build-time gating)

**Analog:** `crates/paladin-llm/Cargo.toml:17-24` (closest existing `[features]` block gating
optional deps behind named features with a default set).

```toml
[features]
default = ["openai", "mock"]
openai = ["dep:reqwest", "dep:rand"]
anthropic = ["dep:reqwest", "dep:rand"]
deepseek = ["dep:reqwest", "dep:rand"]
mock = []
vision = ["openai", "dep:base64"]
openai-embeddings = ["openai"]
```
Pattern to copy: bare feature name maps to a list of `dep:<crate>` entries; a feature can compose
other features (`vision = ["openai", "dep:base64"]`) rather than repeating deps. For
`paladin-herald`, per D-14/Claude's Discretion, the shape is:
```toml
[dependencies]
comfy-table = { version = "7.1", optional = true }
colored = { version = "2.1", optional = true }

[features]
default = []
table = ["dep:comfy-table"]
markdown-color = ["dep:colored"]   # or a single combined name — discretion is explicitly open
```
The root `cli` feature (`Cargo.toml:284`) must then list `paladin-herald/<chosen-name(s)>` so a
`--features cli` build still gets the styled formatters — mirroring how the root `vision` feature
already composes a sub-crate's feature in the `paladin-llm` analog above.

---

### `Cargo.toml` (root) — `[[bin]] paladin` gets `required-features` (config, build-time gating)

**Analog:** the file's own two sibling `[[bin]]` stanzas, `:244-252` (read verbatim, in full).

```toml
[[bin]]
name = "paladin"
path = "src/main.rs"

[[bin]]
name = "paladin-cli"
path = "src/bin/paladin-cli.rs"
required-features = ["cli"]

[[bin]]
name = "paladin-server"
path = "src/bin/paladin-server.rs"
required-features = ["web-server"]
```
Copy the `required-features = ["cli"]` line verbatim into the `paladin` stanza — same key, same
feature name (`cli` already exists and is being extended, not created) — producing:
```toml
[[bin]]
name = "paladin"
path = "src/main.rs"
required-features = ["cli"]
```

---

### `src/main.rs` — `structopt` → `clap` v4 derive migration (binary entry point, request-response)

**Analog:** RESEARCH.md's own verified, ready-to-use translation (`## Code Examples` §"src/main.rs
clap v4 migration"), produced by reading the current 61-line file in full and `src/bin/paladin-cli.rs`
for house clap-derive idiom.

**Current shape (to replace):**
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
```

**Target shape (verified compiling translation, clap v4.5.40 already vendored at `Cargo.toml:122`,
gated by the `cli` feature at `Cargo.toml:284` — no new dependency line needed):**
```rust
use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "smartcontent-aggregator")]
struct Opt {
    #[arg(short, long, default_value = "config.yml")]
    config: String,
}
// ...
let opt = Opt::parse();
```

**Test-fn substitution** (`Opt::from_iter(&[...])` → `Opt::parse_from([...])`):
```rust
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
Mapping table: `#[structopt(...)]` → `#[command(...)]` (container-level) / `#[arg(...)]`
(field-level); `Opt::from_args()` → `Opt::parse()`; `Opt::from_iter(&[...])` → `Opt::parse_from([...])`.

---

### `src/infrastructure/adapters/herald/mod.rs` and `src/application/services/herald/herald_registry.rs` (module gating, transform)

**Analog — the exact template, already shipped in the same sibling file** —
`src/infrastructure/adapters/mod.rs:1-17` (read in full):
```rust
pub mod arsenal;
pub mod auth;
pub mod citadel;
#[cfg(feature = "content-processing")]
pub mod document;
pub mod file_storage;
pub mod garrison;
pub mod herald;
pub mod input;
pub mod llm;
pub mod logs;
#[cfg(feature = "notifications")]
pub mod notifications;
pub mod output;
pub mod queue;
pub mod sanctum;
pub mod scheduling;
```
Note: RESEARCH.md's phrasing ("herald_registry.rs already uses this pattern for its sibling
content-processing and notifications modules") refers to this file — `infrastructure/adapters/mod.rs`
— not literally inside `herald_registry.rs` itself. This is the file to copy the `#[cfg(feature =
"...")]` idiom from.

**Apply the same idiom inside `infrastructure/adapters/herald/mod.rs`**, splitting the currently
unconditional re-export:
```rust
// current (ungated):
pub use paladin_herald::{JsonHerald, MarkdownHerald, TableHerald};

// target shape — JSON stays unconditional (needs no CLI dep), Markdown/Table gated:
pub use paladin_herald::JsonHerald;
#[cfg(feature = "table")]
pub use paladin_herald::TableHerald;
#[cfg(feature = "markdown-color")]  // or whichever feature name(s) chosen in paladin-herald's manifest
pub use paladin_herald::MarkdownHerald;
```

**Apply the same idiom inside `herald_registry.rs`'s `Default::default()` construction** (current
unconditional block, lines ~247-250 read this session):
```rust
// current:
registry.register("json", Arc::new(JsonHerald::new()));
registry.register("markdown", Arc::new(MarkdownHerald::new()));
registry.register("table", Arc::new(TableHerald::default()));

// target shape — json unconditional, markdown/table gated to match:
registry.register("json", Arc::new(JsonHerald::new()));
#[cfg(feature = "markdown-color")]
registry.register("markdown", Arc::new(MarkdownHerald::new()));
#[cfg(feature = "table")]
registry.register("table", Arc::new(TableHerald::default()));
```
(Exact feature name(s) are Claude's Discretion per CONTEXT.md — must match whatever is chosen in
`paladin-herald/Cargo.toml`'s new `[features]` section and threaded through the root `cli` feature.)

---

### `crates/paladin-core/src/platform/container/token_usage.rs` (model, transform — gains derives + inherent impls)

**Analog:** the richer duplicate being absorbed, `crates/paladin-core/src/platform/container/battalion/mod.rs:496-524` (read via RESEARCH.md's direct file read):

```rust
// CANONICAL today — token_usage.rs:12-13 (extend this):
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenUsage { pub prompt_tokens: u32, pub completion_tokens: u32, pub total_tokens: u32 }

// DUPLICATE (battalion/mod.rs:496-524) — absorb these derives + these two inherent methods:
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct TokenUsage { /* same 3 fields */ }
impl TokenUsage {
    pub fn new(prompt_tokens: u32, completion_tokens: u32) -> Self { .. }
    pub fn from_total(total_tokens: u32) -> Self { .. }
}
```
Target: canonical type becomes
`#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]` plus the `new()`/`from_total()`
inherent impl block, copied verbatim from the battalion duplicate before that duplicate is deleted.
Sequence is non-negotiable (D-17): extend canonical first, re-export second — reversing this order
breaks the 11 call sites RESEARCH.md enumerated that depend on `::new()`/`::from_total()`.

---

### `battalion/mod.rs` and `llm_analysis_service.rs` — TokenUsage duplicates become `pub use` re-exports (model, transform)

**Two precedents, both already shipped — copy verbatim, don't invent:**

Precedent 1 — cross-crate, `crates/paladin-ports/src/output/llm_port.rs:671`:
```rust
pub use paladin_core::platform::container::token_usage::TokenUsage;
```

Precedent 2 — intra-crate, `crates/paladin-core/src/platform/container/herald.rs:28`:
```rust
pub use crate::platform::container::token_usage::TokenUsage;
```

Apply:
- `crates/paladin-core/src/platform/container/battalion/mod.rs:497` (intra-crate, same crate as
  `token_usage.rs`) → use **precedent 2's shape**:
  ```rust
  pub use crate::platform::container::token_usage::TokenUsage;
  ```
- `crates/paladin-llm/src/llm_analysis_service.rs:51` (cross-crate; `paladin-llm` already depends on
  `paladin-core` per `Cargo.toml:27`, no new edge) → use **precedent 1's shape**:
  ```rust
  pub use paladin_core::platform::container::token_usage::TokenUsage;
  ```
Both replace the entire `#[derive(...)] pub struct TokenUsage { ... }` block (and any now-redundant
inherent `impl` block) at their site — a `pub use` carries the re-exported type's inherent methods
automatically, so call sites like `battalion::TokenUsage::from_total(...)` keep resolving with zero
call-site changes.

---

### `scripts/check-deprecations.sh` (utility script, must become capable of failing)

**Analogs:** `scripts/check-doc-config.sh` and `scripts/check-api-surface.sh` (both read in full) —
the house shape for a CI gate script that can genuinely fail.

**House shape to copy** (`check-doc-config.sh:1-20`, `check-api-surface.sh:1-19`):
```bash
#!/usr/bin/env bash
# <script purpose>
#
# Usage:  ./scripts/<name>.sh
# Exit:   0 if <condition>, non-zero on failure.

set -euo pipefail

WORKSPACE_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
# ... do the check ...
if <failure condition>; then
    echo "ERROR: ..." >&2
    exit 1
fi
echo "✅ <success message>"
exit 0
```
The current `check-deprecations.sh` (read this session, full text):
```bash
if grep -r "#\[deprecated\]" src/ --include="*.rs" | grep -v "since\|note"; then
    exit 1
fi
echo "✅ All deprecation attributes are properly formatted"
```
already has `set -euo pipefail` at line 4 and a real `exit 1` at line 39 for this final check — the
defect (D-05) is narrower than the script's overall shape: (1) the *primary* and *fallback* branches
above this final check both unconditionally `exit 0` regardless of what they find (lines 27, 30) —
those need to actually gate on their own findings the same way the final check does; (2) the grep
scope is `src/` only and must extend to `src/ crates/`, following `check-api-surface.sh`'s pattern of
running its check across the full relevant path set rather than a subset. Do not add a new
"deprecations must exist" gate — only make the existing exit-code mechanics honest and extend the
malformed-attribute grep's file scope.

---

### `.planning/ledgers/milestone-04-06.md` (record, in-place amendment)

**Analog:** `.planning/ledgers/milestone-01.md` (own prior amendments — Phase 2, 3, 4 sections, read
via grep + targeted reads this session).

**Section-header pattern for a new amendment wave:**
```markdown
## Phase 4 amendments (2026-08-03)

This file is amended in place again, per Phase 2/3's own convention: every row below is additive,
retains any superseded text, and carries a dated note naming its evidence.
```

**Row-amendment markup pattern** (retain original verdict text, wrap it, append dated finding):
```markdown
| REL-05 — <original clause> | **satisfied** (was: deferred with reason → measured, failed) | <original evidence retained>. **(Amended by Phase 4, dated 2026-08-03, citing `04-ci-gate-deferrals.md` §"Second CI execution": <what changed and why, with the new measurement>.)** **(Further amended by Phase 4, dated 2026-08-03, citing ...: <second finding>.)** |
```
Apply this shape to the five rows D-23 names (`:115` `REQ-api-surface-ci`, `:116`
`REQ-deprecation-warnings`, `:157` `REQ-ports-doctest-compilation`, `:160`
`REQ-ports-tests-and-rustdoc`, `:225` `REQ-workspace-ci-upgrade` clause 3): open a
`## Phase 8 amendments (2026-08-06)` section header using the same "amended in place, per Phase N's
own convention" framing, then wrap each row's verdict cell in a bolded new verdict + `(was: <old
verdict>)`, with the amendment prose appended in bold parens citing the exact command/file:line per
D-00e, exactly as the `milestone-01.md` REL-05 rows do above.

---

### `.project/` 6 documents — dated correction banners (record, annotation not rewrite)

**Analog:** `.project/Milestone_4-Refactor-Crates-Features/Epic_1/prd-expand-feature-flags.md`
(Phase 7's own D-00c annotation, read in full — both the document-level banner and an inline
correction site).

**Document-level banner** (placed immediately below the H1, `prd-expand-feature-flags.md:1-11`):
```markdown
# Product Requirements Document: Expand Feature Flags to Gate the Full Optional Surface

> **Correction (dated 2026-08-06, ARCH-05):** This PRD's FR1 and Design Considerations describe
> three feature-flag behaviours the shipped `Cargo.toml` manifest contradicts — the `vision`
> gating of the encryption dependencies, the MCP transport flags, and the `web-server` gating of
> `actix-web`. Applying the `vision` clause literally would break
> `cargo build --no-default-features` for user auth and Citadel encryption. See the Milestone 4-6
> ledger's `diverged` rows and
> [`.planning/decisions/0011-vision-port-surfaces.md`](../../../.planning/decisions/0011-vision-port-surfaces.md)
> for the prior decision that dispositioned this same encryption code. Original text is retained
> below with inline corrections — nothing is deleted.
```

**Inline correction markup** — strike the wrong claim, retain it, append a bolded dated correction
directly beneath it (`prd-expand-feature-flags.md:122-140`):
```markdown
- ~~`vision` — Gates vision pipeline, vision adapters (`openai_vision.rs`, `anthropic_vision.rs`), Sentinel Vision encryption deps (`chacha20poly1305`, `zeroize`), and `VisionPort`/`VisionCapableLlm` trait implementations~~
  **Corrected (dated 2026-08-06, diverged from shipped code):** Shipped `Cargo.toml:274` declares
  `vision = []` — an empty feature that gates **no dependency**. `chacha20poly1305`
  (`Cargo.toml:134`) and `zeroize` (`Cargo.toml:135`) are unconditional root dependencies,
  confirmed by `grep -rn 'chacha20poly1305\|zeroize' Cargo.toml crates/*/Cargo.toml` during this
  task — they serve `src/infrastructure/security/encryption.rs`'s general encryption for user
  auth and Citadel state, not only the vision pipeline.
```
For a claim that is simply confirmed-correct-as-eliminated rather than struck (no literal change
needed, just annotation), the third example in the same file shows the "Confirmed" variant:
```markdown
- ~~`mcp-arsenal`~~ — **ELIMINATED:** Arsenal remains unconditionally compiled
  **Confirmed (dated 2026-08-06, ARCH-05):** No MCP feature flag of any kind exists in the
  shipped manifest — `grep -n mcp Cargo.toml` returns no output, re-run during this task.
```

**Apply this shape to the 6 phase-8 targets**: `DEPRECATIONS.md` (D-07(1), closing its 4 Open
Questions with one-line dispositions each, banner citing ADR-0022) plus the 5 requirement-text
sources for D-04's `project/` → `.project/` path corrections (M8 Epic 7 FR-10, M12 Epic 1 §7, M12
Epic 5 §7, M12 Epic 6 `cross_refs`, M12 Epic 7 FR-4.6) — banner at the top of each document, inline
`~~struck~~` + **Corrected (dated 2026-08-06, DEBT-01)** directly beneath each defective clause,
retaining all original text.

## Shared Patterns

### ADR file shape (no frontmatter, 7 headings)
**Source:** `.planning/decisions/0016-port-value-type-ownership.md`,
`.planning/decisions/0019-binary-target-architecture.md`
**Apply to:** `0022-*.md`, `0023-*.md`
Seven `##` headings in fixed order: `Status` (with `**Date:**` beneath it), `Context`, `Decision`,
`Considered Options`, `Code Locations`, `Code Conformance`, `Downstream Consumers`. No YAML
frontmatter anywhere in the file.

### `#[cfg(feature = "...")]` module/construction gating
**Source:** `src/infrastructure/adapters/mod.rs:4,12` (module declarations)
**Apply to:** `infrastructure/adapters/herald/mod.rs`'s re-exports, `herald_registry.rs`'s
`register(...)` calls, `paladin-herald`'s new `[features]`-gated items
```rust
#[cfg(feature = "content-processing")]
pub mod document;
```

### `set -euo pipefail` + real non-zero exit for CI gate scripts
**Source:** `scripts/check-doc-config.sh:20`, `scripts/check-api-surface.sh:4`
**Apply to:** `scripts/check-deprecations.sh`'s two currently-unconditional `exit 0` branches
```bash
set -euo pipefail
# ...
if <failure>; then
    echo "ERROR: ..." >&2
    exit 1
fi
```

### Ledger in-place amendment (D-00d)
**Source:** `.planning/ledgers/milestone-01.md` §"Phase 4 amendments (2026-08-03)"
**Apply to:** all 5 rows D-23 names in `.planning/ledgers/milestone-04-06.md`
```markdown
| <clause> | **<new verdict>** (was: <old verdict>) | <original evidence>. **(Amended by Phase 8, dated 2026-08-06, citing `<file>`: <finding>.)** |
```

### `.project/` dated correction banner + inline strike-and-append (D-00c)
**Source:** `.project/Milestone_4-Refactor-Crates-Features/Epic_1/prd-expand-feature-flags.md:1-11,122-140`
**Apply to:** `DEPRECATIONS.md` + the 5 D-04 requirement-text sources
```markdown
> **Correction (dated 2026-08-06, <ID>):** <what's wrong> ... Original text is retained below with
> inline corrections — nothing is deleted.
```
```markdown
- ~~<original wrong claim>~~
  **Corrected (dated 2026-08-06, <reason>):** <what's actually true, with file:line evidence>.
```

### `pub use` re-export for a canonical type
**Source:** `crates/paladin-ports/src/output/llm_port.rs:671` (cross-crate),
`crates/paladin-core/src/platform/container/herald.rs:28` (intra-crate)
**Apply to:** `battalion/mod.rs:497` (intra-crate shape), `llm_analysis_service.rs:51` (cross-crate
shape)

## No Analog Found

None. Every file in this phase's scope has a mandatory, named, or discovered analog — this is a
defect-closure phase operating entirely inside established conventions, not new feature construction.

## Metadata

**Analog search scope:** `.planning/decisions/`, `.planning/ledgers/`, `.project/`,
`.planning/phases/07-workspace-ground-truth-recorded-answers/`, `scripts/`, `Cargo.toml` (root +
workspace crates), `src/infrastructure/adapters/`, `src/application/services/herald/`,
`crates/paladin-core/src/platform/container/`, `crates/paladin-llm/src/`, `crates/paladin-ports/src/output/`
**Files scanned:** ~30 (direct reads + targeted greps)
**Pattern extraction date:** 2026-08-06
