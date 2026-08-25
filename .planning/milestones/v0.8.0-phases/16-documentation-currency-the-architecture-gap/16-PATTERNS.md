# Phase 16: Documentation Currency & the Architecture Gap - Pattern Map

**Mapped:** 2026-08-24
**Files analyzed:** ~24 discrete artifacts (14 DOCS-01 pages as one class, plus per-file items below)
**Analogs found:** 6 strong / 3 partial / 4 "no analog — new shape"

**Calibration note:** this is a documentation-content phase. There is almost no new Rust; the
"role/data-flow" table below is repurposed as "artifact kind / change kind" since the standard
controller/service/CRUD taxonomy does not fit prose, ADRs, or shell scripts.

## File Classification

| New/Modified File | Kind | Change Kind | Closest Analog | Match Quality |
|---|---|---|---|---|
| `.planning/decisions/0047-*.md` (ADR) | ADR | new (archive + metric + diagram-withdrawal decisions) | `.planning/decisions/0022-deprecation-requirement-withdrawal.md` | exact (explicitly named by CONTEXT D-04) |
| `.planning/decisions/0033-cargo-doc-warning-bar.md` | ADR | amend in place (D-07: herald claim correction) | itself (in-place amendment, no external analog needed) | exact |
| `.planning/decisions/PROMOTION.md` | ledger | amend one line (next-free-number 0047→0048) | itself | exact |
| `scripts/check-public-api-examples.sh` | shell script | new | `scripts/check-doc-examples.sh` (style/shebang), `scripts/extract-public-api.sh` (arg/exit convention) | role-match |
| `crates/*/src/**.rs` doc comments (20 warning sites) | rustdoc comment | fix in place | `crates/paladin-web/src/*.rs`, `crates/paladin-battalion/src/in_memory_registry.rs`, `crates/paladin-herald/src/lib.rs`, facade files — fixes are self-contained, see Pattern classes below | exact (fix classes re-derived by RESEARCH.md, not this agent) |
| ~30 new `# Examples` blocks on builders/`*Port`/`*Service` | rustdoc doc-comment | new | `src/application/services/paladin/paladin_builder.rs` (builder), `crates/paladin-ports/src/output/citadel_port.rs` (Port trait doc shape) | exact for builders; role-match for services (no `*Service` example was read this pass — see below) |
| `crates/paladin-herald/src/lib.rs:20` | crate attribute | flip `allow`→`warn` | the other 9 crates' `#![warn(missing_docs)]` lines (already-uniform sibling crates) | exact |
| `docs/src/appendix/design-and-architecture.md` | mdBook page | modify (add archive banner) | no direct precedent for an archive-banner page in this tree; closest shape is the "(Legacy)" naming pattern already used at `docs/src/SUMMARY.md:109` (`Contributing (Legacy)`) | partial |
| `docs/src/architecture/*.md` (Sentinel addition) | mdBook page | modify (add Sentinel coverage or cross-link) | `docs/src/appendix/sentinel.md` (existing Sentinel content to cross-link/lift from) | exact |
| `docs/src/SUMMARY.md` | mdBook TOC | modify (one line's context, not new content) | itself — existing TOC entries at lines 76-109 show the flat list-item shape | exact |
| 14 DOCS-01 pages (`user-guides/*.md`, `deployment/*.md`, `operations/*.md`) | mdBook page | content-currency edit | `docs/src/deployment/cicd.md` — the worked fabrication example (see excerpt below) | exact (this is the model defect, not a stylistic analog) |
| `docs/assets/recordings/*.tape` | VHS script | new | **no analog — new shape** | none |
| `docs/assets/recordings/*.gif` / `*.cast` | binary/recording artifact | new | **no analog — new shape** | none |
| `docs/DEMOS.md` | mdBook/root index page | new | nearest shape analog: any existing `docs/src/**` index-with-embedded-media page — none of the 14 DOCS-01 pages or the architecture chapter embeds media the way this must; closest structural cousin is `docs/src/SUMMARY.md`'s flat list style for the link, not the page itself | none (say so) |
| `README.md` | root doc | modify (one link line, D-15) | itself — existing README structure after M11 Epic 5's condensation | exact |
| `.devcontainer/Dockerfile.dev`, `.devcontainer/Dockerfile` | container build script | modify (add pinned tool installs) | the existing `cargo-release`/`cargo-deny`/`cargo-cyclonedx` pinned-install block at `Dockerfile.dev:127-134` | exact |
| `.planning/codebase/CONVENTIONS.md` §Comments | project convention doc | modify (D-06 heading rule) | itself — existing "Comments" section documents `# Example` (singular); this is a direct amend-in-place | exact |

## Pattern Assignments

### `.planning/decisions/0047-*.md` (new ADR)

**Analog:** `.planning/decisions/0022-deprecation-requirement-withdrawal.md` (full file read)

**Structure to copy** (no frontmatter, exact heading set):
```markdown
# ADR-00NN: <title>

## Status

Accepted

**Date:** YYYY-MM-DD

## Context
<restate the stale premise rather than dropping it; cite exact grep/file:line evidence>

## Decision
<the decision, with the reversibility/re-instatement clause written as an instruction:
"Re-instatement is possible but is written down as an instruction, not mechanised: any future
ADR that wants to bring X back must explicitly supersede this one.">

## Considered Options
- **<accepted option>** (accepted) — reasoning
- **<rejected option>** (rejected) — reasoning
- **<rejected option>** (rejected) — reasoning

## Code Locations
- `path:line` — description

## Code Conformance
<not shown in the excerpt above but present later in 0022 — confirm shape by reading past line 80
before drafting the plan's action section>

## Downstream Consumers
<same — confirm shape from the tail of 0022 or another recent ADR (e.g. ADR-0033) before drafting>
```

**Key phrasing pattern to copy verbatim in spirit** (0022's re-instatement clause):
```
Re-instatement is possible but is written down as an instruction, not mechanised: any future ADR
that wants to bring FR-8 back must explicitly supersede this one. This corpus's `PROMOTION.md`
defines a supersession mechanism (superseded ADR's `## Status` becomes `Superseded` with a pointer;
the superseding ADR carries a `## Supersedes` line)...
```
ADR-0047 must use this exact pattern for D-03's "diagram re-instatement is an instruction, not a
mechanism" requirement.

**ADR-0033 is also directly on-topic** (D-00t: it ratified the `cargo doc` bar) — pull its "Finding
1 / Finding 2 / Finding 3" visually-distinct-findings structure if ADR-0047 similarly needs to keep
its three sub-decisions (archive / metric / diagram-withdrawal) visually separate rather than merged
into one narrative:
```
This ADR settles HARD-07 with three separate findings, kept visually distinct because they have
different owners and different dispositions. They are not merged into one narrative.
```

---

### `.planning/decisions/0033-cargo-doc-warning-bar.md` (amend in place, D-07)

**Analog:** itself. Read lines 1-60 this pass (Context section, Finding 1 and the start of Finding
2). The amendment per D-00d must be a **dated note appended**, not a silent edit — follow the same
"restate the stale premise, then correct it" pattern seen in 0022's Context section (which restates
Milestone 4's stale v0.2.0→v1.0.0 timeline before superseding it) and in this phase's own
CONTEXT.md M-07 language: *"amend ADR-0033 in place with a dated note recording that its 'all ten
library crates' claim was inaccurate when written."*

Exact text to correct: ADR-0033's Finding 1 says *"All ten library crates plus the facade carry
`#![warn(missing_docs)]`"* — `paladin-herald` did not (measured M-07), so this line needs a dated
correction note, not a silent rewrite.

---

### `scripts/check-public-api-examples.sh` (new)

**Analog:** `scripts/check-doc-examples.sh` (full file read) for style/shebang/structure;
`scripts/extract-public-api.sh` (partial read) for exit-code and tool-missing-fallback convention.

**Shebang + strict-mode pattern** (from `check-doc-examples.sh` lines 1-21):
```bash
#!/usr/bin/env bash
# check-doc-examples.sh
#
# <purpose comment block, numbered layers if there is more than one check>
#
# Usage:  ./scripts/check-doc-examples.sh
# Exit:   0 if all examples compile and all inline blocks are valid.

set -euo pipefail

WORKSPACE_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
DOCS_SRC="${WORKSPACE_ROOT}/docs/src"
```
Copy this exact `WORKSPACE_ROOT` derivation and `set -euo pipefail` header — the `.pre-commit-config.yaml`
runs `shellcheck`, and this pattern is already shellcheck-clean in the existing script.

**Failure-message pattern** (same file, lines 27-30):
```bash
if ! cargo check --quiet --manifest-path "${WORKSPACE_ROOT}/crates/doc-examples/Cargo.toml"; then
    echo "ERROR: documentation examples failed to compile." >&2
    echo "Fix the example in crates/doc-examples/src/ that no longer matches the API." >&2
    exit 1
fi
```
Use this `ERROR: ... >&2` + actionable-next-step + `exit 1` shape for `check-public-api-examples.sh`'s
failure path (RESEARCH.md Pattern 3 gives the awk logic; wrap it in this script skeleton, not a bare
awk one-liner).

**Tool-missing fallback pattern** (from `extract-public-api.sh` lines 9-19):
```bash
if ! command -v cargo-public-api &> /dev/null; then
    echo "❌ cargo-public-api not found. Installing..."
    cargo install cargo-public-api || {
        echo "❌ Failed to install cargo-public-api"
        exit 1
    }
fi
```
Not directly needed (the new script only needs `awk`/`grep`, both always present), but this is the
project's established "check tool present, fail loud with remediation" convention if any dependency
check is added.

---

### `crates/paladin-web/src/*.rs` etc. — the 20 `cargo doc` warning fixes (DOCS-03)

**Analog:** self-contained; RESEARCH.md Pattern 2 already gives the exact four fix classes with
before/after code. Reproduced here for the planner's direct use (verbatim from 16-RESEARCH.md):

```rust
// Class 1: unresolved intra-doc link (14 of 20) — de-link crate-name mentions.
//! Because this type depends only on `paladin-core` and `paladin-ports` it carries   // fixed
//! Because this type depends only on [`paladin-core`] and [`paladin-ports`] it carries // warns

// Class 2: private-item link from public docs (3 of 20) — drop the link, keep the name as text.
/// checked when the provider is actually created in `build_agent`.               // fixed
/// checked when the provider is actually created in [`build_agent`].             // warns

// Class 3: redundant explicit link target (2 of 20) — drop the explicit target.
/// [`agent_router`], merged in. It is the                                        // fixed
/// [`agent_router`](crate::agent_controller::agent_router), merged in. It is the // warns

// Class 4: unclosed HTML tag (1 of 20) — wrap generic type in a code span.
/// Internal storage: Paladin ID -> `Arc<Paladin>`                                // fixed
/// Internal storage: Paladin ID -> Arc<Paladin>                                  // warns
```

**Do not** trust ADR-0033's file:line citations for these — re-run `cargo doc --workspace --no-deps`
fresh per RESEARCH.md Pitfall 1; this agent did not re-derive them again (RESEARCH.md already did,
this session, with fresh citations reproduced in its Code Examples section).

---

### New `# Examples` blocks on builders (~11 of the 79 D-05 entry points)

**Analog:** `src/application/services/paladin/paladin_builder.rs` lines 1-28 (module-doc `# Examples`
block, full pattern read):
```rust
//! PaladinBuilder - Fluent builder for creating Paladin instances with validation
//!
//! This module provides a builder pattern implementation for constructing Paladin entities
//! with compile-time safety and runtime validation of configuration parameters.
//!
//! # Examples
//!
//! ```rust,no_run
//! use paladin::application::services::paladin::paladin_builder::PaladinBuilder;
//! use paladin_ports::output::llm_port::LlmPort;
//! use paladin::core::platform::container::paladin_config::OutputFormat;
//! use std::sync::Arc;
//!
//! # async fn example(llm_port: Arc<dyn LlmPort>) -> Result<(), Box<dyn std::error::Error>> {
//! let paladin = PaladinBuilder::new(llm_port)
//!     .system_prompt("You are a helpful coding assistant")
//!     .name("CodePaladin")
//!     .user_name("Developer")
//!     .model("gpt-4")
//!     .temperature(0.7)
//!     .max_loops(5)
//!     .add_stop_word("STOP")
//!     .retry_attempts(3)
//!     .timeout_seconds(300)
//!     .enable_planning(true)
//!     .output_format(OutputFormat::Json)
//!     .build().await?;
//! # Ok(())
//! # }
//! ```
```
This is a **`no_run`** disposition (async, needs a real `LlmPort` implementation) — one worked
example of the "needs live I/O → `no_run`" half of Claude's Discretion item on doctests. Copy the
`# async fn example(...) -> Result<(), Box<dyn std::error::Error>> { ... }` hidden-line wrapper
pattern (`#` prefix hides the line from rendered docs but keeps it compiling) for any new builder
example needing async setup.

### New `# Examples` blocks on `*Port` traits (~35 of the 79)

**Analog:** `crates/paladin-ports/src/output/citadel_port.rs` lines 1-24 (module-doc header read).
This file's doc comment is a **narrative/architecture-diagram** style (`## Purpose`, `## Hexagonal
Architecture Context` with an ASCII box diagram) rather than a runnable code example — useful for
`*Port` trait *module*-level docs, but note this specific file was **not yet confirmed to carry a
runnable `# Examples` code block** in the portion read. Before treating it as the `*Port` analog for
D-05's literal requirement, the executor should grep the full file for `# Examples` /
` ```rust ``` ` and, if absent here, fall back to `embedding_port.rs` or `arsenal_port.rs` (both
returned by the same grep as carrying `# Examples`, per this session's search) as the concrete
runnable-example analog instead.

### New `# Examples` blocks on `*Service` structs (~33 of the 79)

**No analog confirmed this pass** — no `*Service` struct's doc comment was read for its `# Examples`
shape. Recommend the planner grep `grep -rln '^/// # Examples' crates/*/src src` filtered to
`*_service.rs` / `*Service` matches and pick one of the 47-already-documented entry points D-05
counts, rather than inventing a new shape.

---

### `crates/paladin-herald/src/lib.rs:20` (flip `allow`→`warn`)

**Analog:** the other 9 library crates' `lib.rs` top-of-file attribute, already
`#![warn(missing_docs)]` per ADR-0033 Finding 1. This is a one-line, zero-ambiguity change — no
excerpt needed beyond: change `#![allow(missing_docs)]` to `#![warn(missing_docs)]` at that exact
line, matching the sibling crates' existing line verbatim in form.

---

### 14 DOCS-01 pages — the worked "what drift looks like" analog

**Analog:** `docs/src/deployment/cicd.md` (partial read, lines 1-60) is the concrete example of a
genuine content-currency defect, not a style analog. Quoted for the planner to replicate the
*checking method* against the other 13 files:

```yaml
# docs/src/deployment/cicd.md's quoted (stale/fabricated) ci.yml excerpt:
name: CI

on:
  push:
    branches: [ main, develop ]
  pull_request:
    branches: [ main, develop ]
```
versus the actual tree (`ci.yml:14`): `branches: ['**']`, no `develop` branch (retired plan
15.1-09); the actual job is `lint` / `Code Quality` (`ci.yml:41`), not `check`/`Check`; and the
page's "Workflow Structure" ASCII tree at line ~29-36 still lists `integration-tests.yml`, deleted
by commit `2cf9919` and absorbed into `ci.yml`.

**Per-file verdict-record row shape** (D-09, from RESEARCH.md Pattern 1, this is the template every
one of the 14 rows must follow):
```markdown
| File | Signals checked | Findings (command / file:line) | Verdict |
|---|---|---|---|
| docs/src/deployment/cicd.md | version strings, workflow names, job names, `develop` branch refs | `grep -n "branches: \[ main, develop \]" docs/src/deployment/cicd.md` → line 47 (fabricated, vs. actual `ci.yml:14 branches: ['**']`); `grep -n "check:" docs/src/deployment/cicd.md` → job named `check`/`Check` (actual job is `lint`/`Code Quality`, `ci.yml:41`); workflow-structure diagram lists `integration-tests.yml`, deleted by commit `2cf9919` | updated → commit |
```

---

### `docs/src/appendix/design-and-architecture.md` (archive banner, D-01/D-02/D-03)

**No strong precedent for a banner block exists in this tree.** The closest shape signal is
`docs/src/SUMMARY.md:109`'s `Contributing (Legacy)` naming — the project's only other "this is the
superseded one" marker, but it is a TOC label, not a banner. **Say so plainly: no analog — new
shape.** Constrain the banner by DOCS-02's own wording (quoted in CONTEXT.md specifics): it "finds a
clear statement that the architecture appendix is historical **and where to look instead**" —
i.e. the banner must name `docs/src/architecture/` and `docs/src/appendix/sentinel.md` as the
live targets, not just say "this file is outdated."

---

### `docs/src/architecture/*.md` — Sentinel cross-link (D-02)

**Analog:** `docs/src/appendix/sentinel.md` — the existing Sentinel content itself is the source
material to link to or lift a summary from; read this file directly when drafting the cross-link
before deciding whether D-02 is satisfied by a link or by moved/duplicated content.

---

### `.devcontainer/Dockerfile.dev` + `.devcontainer/Dockerfile` (D-11, D-14 tool pins)

**Analog:** `.devcontainer/Dockerfile.dev` lines 127-134 (read directly):
```dockerfile
# Install cargo-release (pinned) — drives `make release` (lockstep version bump,
RUN cargo install --locked --version 1.1.2 cargo-release
RUN cargo install --locked --version 0.19.8 cargo-deny
RUN cargo install --locked --version 0.5.9 cargo-cyclonedx
```
Copy this exact `cargo install --locked --version <N> <crate>` shape for `mdbook 0.4.40`,
`mdbook-mermaid 0.13.0`, `mdbook-linkcheck 0.7.7` in **both** Dockerfiles (D-11 requires both — the
main `Dockerfile` uses `bullseye` not `bookworm`, confirmed by this session's read; the tool pins are
identical, only the base image line differs, so the RUN block itself is copy-identical across both
files). `vhs`/`ttyd`/`asciinema` are **not** `cargo install` — they need their own install shape
(APT repo add + key, or GitHub-release binary download); no existing analog in either Dockerfile
installs from a non-cargo, non-apt-standard-repo source, so **that half is no analog — new shape**,
flagged in RESEARCH.md for `checkpoint:human-verify`.

---

## Shared Patterns

### ADR shape and in-place amendment
**Source:** `.planning/decisions/0022-deprecation-requirement-withdrawal.md` (structure),
`.planning/decisions/0033-cargo-doc-warning-bar.md` (amendment target)
**Apply to:** ADR-0047 (new) and the ADR-0033 dated-correction note (amend).
No frontmatter; heading set is `Status / Context / Decision / Considered Options / Code Locations /
Code Conformance / Downstream Consumers` per CONTEXT.md D-00a — confirm the last two headings' exact
content shape by reading past line 80 of 0022 before drafting, since only the first three sections
were read this pass.

### Evidence-first verdict rows (D-00e / D-09)
**Source:** RESEARCH.md Pattern 1 (already a worked template, reproduced above under the DOCS-01
section) — every claim of "current" or "updated" needs the producing `grep`/`file:line` inline in
the row, not a bare assertion.

### Pinned devcontainer tool install
**Source:** `.devcontainer/Dockerfile.dev:127-134`
**Apply to:** All D-11/D-14 tool provisioning — `cargo install --locked --version N <crate>`, one
`RUN` line per tool, matching CI's `docs.yml:44-54` pins exactly.

### Shell script skeleton (shebang, strict mode, path derivation, error output)
**Source:** `scripts/check-doc-examples.sh` lines 1-21, 27-30
**Apply to:** `scripts/check-public-api-examples.sh` (new).

## No Analog Found

| File | Kind | Reason |
|---|---|---|
| `docs/assets/recordings/*.tape` | VHS script | No `.tape` file exists anywhere in the tree; genuinely new tool/format for this repo (D-14) |
| `docs/assets/recordings/*.gif`, `*.cast` | recording artifact | No recorded demo of any kind exists in the tree; `docs/assets/` itself does not exist (M-09) |
| `docs/DEMOS.md` | mdBook/root index page | No existing page in `docs/src/**` embeds media as its primary content; nearest structural cousin is `SUMMARY.md`'s flat link-list style for the README's one added link, not the page's own body |
| Archive banner text for `design-and-architecture.md` | mdBook page fragment | No "this file is historical, go here instead" banner exists anywhere in `docs/src/`; constrained instead by DOCS-02's own wording (quoted above) rather than a codebase precedent |
| `vhs`/`ttyd`/`asciinema` install block in the Dockerfiles | container build script fragment | No existing Dockerfile RUN line installs from a GitHub-release binary or third-party APT repo with a `curl \| gpg --dearmor` key step — every existing tool install is `apt-get` (Debian repo) or `cargo install` (crates.io) |

## Metadata

**Analog search scope:** `.planning/decisions/`, `scripts/`, `.devcontainer/`,
`docs/src/{deployment,appendix,architecture}/`, `docs/src/SUMMARY.md`,
`src/application/services/paladin/paladin_builder.rs`, `crates/paladin-ports/src/output/*.rs`,
`crates/paladin-web/src/*.rs` (for warning-fix context only).
**Files scanned (Read/Grep/Bash):** ~15 files fully or partially read, plus 3 directory listings.
**Pattern extraction date:** 2026-08-24
