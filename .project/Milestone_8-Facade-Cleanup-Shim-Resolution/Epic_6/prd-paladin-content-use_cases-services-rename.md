# PRD: `paladin-content` — Rename `use_cases` → `services` (Milestone 8, Epic 6)

**Project:** Paladin Framework
**Milestone:** 8 — Facade Cleanup, Shim Resolution, and Directory Stabilization
**Epic:** 6 — `paladin-content` `use_cases` → `services` Rename
**Version Target:** post-v0.2.0 patch
**Status:** Ready for Implementation
**Created:** 2026-05-30
**Author:** AI Coding Agent (GitHub Copilot)

---

## 1. Introduction / Overview

Epic 4 renamed the facade crate's `src/application/use_cases/` directory to
`src/application/services/`. As part of that rename, the facade's re-export bridge at
`src/application/services/content/mod.rs` was updated to reference
`paladin_content::services::*`. However, the `paladin-content` leaf crate itself was not in
scope for Epic 4: its public module is still declared as `pub mod use_cases` in
`crates/paladin-content/src/lib.rs`.

This mismatch leaves six `E0432 unresolved import` errors latent inside the facade bridge.
The errors are masked in the default `cargo test` run because all six re-export statements
carry `#[cfg(feature = "content-processing")]` gates — the broken paths are never compiled
unless that feature flag is explicitly enabled.

Epic 6 completes the rename inside `paladin-content` to match the naming convention
established by Epic 4, resolves the compile errors, and verifies the workspace is clean
under both the default feature set and the `content-processing` feature flag.

**Scope:** This Epic is limited to the `paladin-content` leaf crate. No other leaf crates
are audited or modified. Auditing other leaf crates for equivalent `use_cases` modules is
explicitly out of scope and deferred to a future Epic.

---

## 2. Goals

1. `crates/paladin-content/src/services/` exists with all service files intact; the
   `src/use_cases/` directory is deleted from the repository.
2. `crates/paladin-content/src/lib.rs` declares `pub mod services;` with an accurate
   doc comment.
3. Zero `crate::use_cases` references remain inside `crates/paladin-content/`.
4. `crates/paladin-content/README.md` uses `services` everywhere.
5. `cargo build --workspace --features content-processing` exits 0 (six latent `E0432`
   errors are resolved).
6. All quality gates pass under both the default feature set and the `content-processing`
   feature flag.
7. A `fix:` CHANGELOG entry is added to the `## [Unreleased]` block.

---

## 3. Non-Goals

- **No changes to `src/application/services/content/mod.rs`** — the facade re-export bridge
  already references `paladin_content::services::*` and requires zero modifications.
- **No changes to `src/application/services/content/content_ingestion_service.rs`** — this
  file implements logic unique to the facade (RSS/web/API ingestion, scheduler, source
  management) and is not part of `paladin-content`.
- **No audit of other leaf crates** — only `paladin-content` is in scope for this Epic.
- **No semver-breaking-change treatment** — `paladin-content` is a workspace-internal crate
  with no independent crates.io release. The rename does not break any external public API.

---

## 4. User Stories

- **As a contributor** building the workspace with `--features content-processing`, I want the
  build to succeed so I can work on content-processing functionality without encountering
  unresolved-import errors that block compilation.

- **As a maintainer** reviewing the naming conventions established by Epic 4, I want the
  `paladin-content` leaf crate to use `services` (not `use_cases`) consistently with every
  other module in the workspace.

- **As a future contributor** adding a new content service to `paladin-content`, I want to
  place it in `src/services/` and import it via `crate::services::*` without encountering
  conflicting module names that suggest the old convention.

---

## 5. Functional Requirements

### 5.1 — Directory Rename

**FR-1.** The directory `crates/paladin-content/src/use_cases/` must be renamed to
`crates/paladin-content/src/services/` using `git mv` so the rename is tracked by Git:

```bash
git mv crates/paladin-content/src/use_cases crates/paladin-content/src/services
```

All files currently in `use_cases/` must be present in `services/` after the rename. The
`use_cases/` directory must not exist in the repository after the rename.

**FR-2.** The files that must be present in `crates/paladin-content/src/services/` after the
rename (no additions, no deletions):
- `content_aggregator_service.rs`
- `content_analysis_service.rs`
- `content_delivery_service.rs`
- `content_fetching_service.rs`
- `content_filtering_service.rs`
- `content_list_fetching_service.rs`
- `content_list_ingestion_service.rs`
- `content_list_service.rs`
- `content_llm_analysis_service.rs`
- `content_ml_analysis_service.rs`
- `content_nlp_analysis_service.rs`
- `content_summarizer_service.rs`
- `mod.rs`

### 5.2 — `lib.rs` Module Declaration Update

**FR-3.** In `crates/paladin-content/src/lib.rs`, the module declaration must change from:

```rust
pub mod use_cases;
```

to:

```rust
pub mod services;
```

**FR-4.** The crate-level `//!` doc comment in `lib.rs` must not reference `use_cases`. The
current line:

```
//! Content processing adapters and use-case services for the Paladin framework.
```

must be updated to use `services` terminology, for example:

```
//! Content processing adapters and application services for the Paladin framework.
```

### 5.3 — Internal `crate::use_cases` Reference Updates

**FR-5.** Every occurrence of `crate::use_cases` inside `crates/paladin-content/src/` must be
replaced with `crate::services`. The following files are known to contain such references
(confirmed by `grep -rn "crate::use_cases" crates/paladin-content/src/ --include="*.rs"`):

| File | Reference count |
|------|----------------|
| `src/adapters/input/http_content_fetcher.rs` | 1 |
| `src/adapters/input/file_content_list_fetcher.rs` | 1 |
| `src/adapters/input/news_api_fetcher.rs` | 2 |
| `src/services/content_llm_analysis_service.rs` (renamed) | 1 |

Total: 5 occurrences across 4 files.

**FR-6.** After the updates, the following command must return zero results:

```bash
grep -rn "crate::use_cases" crates/paladin-content/src/ --include="*.rs"
```

### 5.4 — `README.md` Updates

**FR-7.** `crates/paladin-content/README.md` must be updated so that all references to
`use_cases` in prose, module descriptions, import examples, and type-name examples are
replaced with `services`.

Known occurrences to update:
- Module description line referencing `use_cases`.
- `use paladin_content::use_cases;` import example → `use paladin_content::services;`
- Any `use_cases::` path prefix in code examples → `services::`

### 5.5 — CHANGELOG Entry

**FR-8.** A `fix:` entry must be added to the `## [Unreleased]` section of `CHANGELOG.md`
documenting this patch. The entry must appear under a `### Fixed` sub-section and must
describe:
- The rename of `use_cases` → `services` inside `paladin-content`.
- Resolution of six `E0432 unresolved import` errors in the facade's
  `content/mod.rs` re-export bridge.
- That the errors were previously masked by the `content-processing` feature gate.

---

## 6. Non-Functional Requirements

**NFR-1.** `cargo build -p paladin-content` must exit 0 after Task 6.3 (internal ref
updates) and before any workspace-level build.

**NFR-2.** `cargo build --workspace` must exit 0 under the default feature set.

**NFR-3.** `cargo build --workspace --features content-processing` must exit 0, confirming
the six previously-broken `E0432` errors are resolved.

**NFR-4.** `cargo test --workspace` must exit 0 under the default feature set.

**NFR-5.** `cargo test --workspace --features content-processing` must exit 0.

**NFR-6.** `cargo clippy --workspace -- -D warnings` must exit 0 (no new warnings
introduced by the rename).

**NFR-7.** `cargo fmt --all -- --check` must exit 0 (no formatting drift).

---

## 7. Technical Design

### 7.1 Change Summary

The entire change is a module rename with no behavioural differences. The public API
of `paladin-content` changes only in the module path:

| Before | After |
|--------|-------|
| `paladin_content::use_cases::*` | `paladin_content::services::*` |

Because `paladin-content` is workspace-internal and has no independent crates.io release,
this rename is **not** a semver-breaking change. No downstream crates outside the workspace
depend on `paladin_content::use_cases`.

### 7.2 Files Affected

| File | Change |
|------|--------|
| `crates/paladin-content/src/use_cases/` (directory) | `git mv` → `src/services/` |
| `crates/paladin-content/src/lib.rs` | `pub mod use_cases` → `pub mod services`; update `//!` doc |
| `crates/paladin-content/src/adapters/input/http_content_fetcher.rs` | `crate::use_cases` → `crate::services` (1 ref) |
| `crates/paladin-content/src/adapters/input/file_content_list_fetcher.rs` | `crate::use_cases` → `crate::services` (1 ref) |
| `crates/paladin-content/src/adapters/input/news_api_fetcher.rs` | `crate::use_cases` → `crate::services` (2 refs) |
| `crates/paladin-content/src/services/content_llm_analysis_service.rs` | `crate::use_cases` → `crate::services` (1 ref; path also updated by directory rename) |
| `crates/paladin-content/README.md` | `use_cases` → `services` in prose and code examples |
| `CHANGELOG.md` | Add `fix:` entry under `## [Unreleased]` → `### Fixed` |
| `src/application/services/content/mod.rs` | **No change required** — already references `paladin_content::services::*` |

### 7.3 Verification Command Sequence

```bash
# Step 1: Isolated crate build (after lib.rs + internal refs updated)
cargo build -p paladin-content

# Step 2: Full workspace default build
cargo build --workspace

# Step 3: Full workspace with content-processing flag (the key regression test)
cargo build --workspace --features content-processing 2>&1 | grep -E "^error"
# Must produce zero output

# Step 4: Test suites
cargo test --workspace
cargo test --workspace --features content-processing

# Step 5: Lint + format
cargo clippy --workspace -- -D warnings
cargo fmt --all -- --check
```

---

## 8. Implementation Tasks (High-Level)

| # | Task | Deliverable |
|---|------|-------------|
| 6.1 | `git mv` `use_cases/` → `services/` | Directory renamed; `use_cases/` absent |
| 6.2 | Update `lib.rs` module declaration and `//!` doc | `pub mod services;`; no `use_cases` in doc |
| 6.3 | Replace all `crate::use_cases` refs (5 occurrences, 4 files) | Zero grep hits |
| 6.4 | Update `README.md` | No `use_cases` references remain |
| 6.5 | Verify `--features content-processing` build exits 0 | Zero `E0432` errors |
| 6.6 | Add CHANGELOG entry | `## [Unreleased]` → `### Fixed` entry present |
| 6.7 | Full quality gate + commit | All 7 commands exit 0; commit on branch |

---

## 9. Success Criteria

- [ ] `crates/paladin-content/src/use_cases/` does **not** exist in the repository.
- [ ] `crates/paladin-content/src/services/` contains all 13 files listed in FR-2.
- [ ] `grep -rn "use_cases" crates/paladin-content/` returns zero results.
- [ ] `cargo build --workspace` exits 0.
- [ ] `cargo build --workspace --features content-processing` exits 0.
- [ ] `cargo test --workspace` exits 0.
- [ ] `cargo test --workspace --features content-processing` exits 0.
- [ ] `cargo clippy --workspace -- -D warnings` exits 0.
- [ ] `cargo fmt --all -- --check` exits 0.
- [ ] `CHANGELOG.md` contains a `### Fixed` entry under `## [Unreleased]` describing this patch.

---

## 10. Out of Scope

- Auditing other leaf crates (`paladin-core`, `paladin-llm`, `paladin-memory`, etc.) for
  surviving `use_cases` modules.
- Any changes to the facade crate's `src/application/services/content/mod.rs` or
  `content_ingestion_service.rs`.
- Updating `STABLE_API.md` — `paladin-content` module paths are workspace-internal and
  not part of the stable public API catalog.
- Generating a new `api_surface_current.txt` or `final-api.txt` — the facade's public
  surface is unchanged by this Epic.

---

## 11. Dependencies

| Dependency | Status |
|------------|--------|
| Epic 4 (`use_cases` → `services` rename in facade) | Completed ✅ |
| Epic 5 (facade documentation and v0.2.0 finalization) | Completed ✅ |

---

## 12. Risks and Mitigations

| Risk | Likelihood | Mitigation |
|------|------------|------------|
| Additional `crate::use_cases` refs missed by the known-file list | Low | Run `grep -rn "crate::use_cases" crates/paladin-content/` after FR-5 edits and confirm zero hits before committing |
| `git mv` not tracked correctly (directory rename split into delete + add) | Low | Verify with `git status` — must show `renamed:` not `deleted:` + `untracked:` |
| New clippy warning introduced by renamed module | Low | Run `cargo clippy --workspace -- -D warnings` as part of the quality gate before committing |

---

## 13. Commit Message

```
fix(m8-e6): rename use_cases -> services in paladin-content

- git mv src/use_cases -> src/services
- Updated lib.rs: pub mod services; updated crate doc comment
- Updated internal crate::use_cases refs in adapter files (5 occurrences, 4 files)
- Updated README.md examples and prose
- Resolves E0432 unresolved import errors in facade content/mod.rs
- Closes broken re-export bridge introduced by Epic 4 rename
- Errors were previously masked by content-processing feature gate
```
