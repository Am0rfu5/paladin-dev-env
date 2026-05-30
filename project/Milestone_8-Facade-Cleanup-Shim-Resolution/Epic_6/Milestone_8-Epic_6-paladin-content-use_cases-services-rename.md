## Epic 6: `paladin-content` — `use_cases` → `services` Rename

**Priority:** High
**Estimated Effort:** Small
**Dependencies:** Epic 4 (`use_cases` → `services` rename in the facade crate)

### Background

During Epic 4, the facade crate's `src/application/use_cases/` directory was renamed to
`src/application/services/`. As part of that rename, the facade re-export bridge at
`src/application/services/content/mod.rs` was updated to reference
`paladin_content::services::*`. However, the `paladin-content` leaf crate was not updated:
its module is still published as `pub mod use_cases` in `crates/paladin-content/src/lib.rs`.

This mismatch causes six `E0432 unresolved import` errors against
`paladin_content::services::*`. The errors are masked in the default `cargo test` run because
all six re-export statements are `#[cfg(feature = "content-processing")]`-gated, so the
broken paths are never compiled in the default feature set.

### Root Cause

The Epic 4 rename scope was defined as the facade crate only (`src/`). The leaf crate
`paladin-content` was not in scope for that Epic. As a result the facade's re-export bridge
was updated to the new path, but `paladin-content` still publishes the old path, leaving the
bridge broken under the `content-processing` feature flag.

### Objective

Rename `use_cases` → `services` inside `crates/paladin-content` to match the naming
convention established by Epic 4, resolve the compile errors in the facade, and verify the
workspace is clean under all relevant feature flag combinations.

---

### Tasks

#### Task 6.1: Rename the `use_cases` Directory in `paladin-content`

**Description:** Use `git mv` to rename the module directory:

```bash
git mv crates/paladin-content/src/use_cases crates/paladin-content/src/services
```

**Deliverables:**
- `crates/paladin-content/src/services/` exists with all service files intact.
- `crates/paladin-content/src/use_cases/` is deleted from the repository.

---

#### Task 6.2: Update `paladin-content/src/lib.rs` Module Declaration

**Description:** In `crates/paladin-content/src/lib.rs`, change the public module
declaration:

```rust
// before
pub mod use_cases;

// after
pub mod services;
```

Also update the crate-level `//!` doc comment if it references `use_cases`.

**Deliverables:**
- `lib.rs` declares `pub mod services;`.
- Doc comment updated if needed.

---

#### Task 6.3: Update Internal `crate::use_cases` References

**Description:** Several files inside `paladin-content` import from `crate::use_cases::*`.
Find and update all of them:

```bash
grep -rn "crate::use_cases" crates/paladin-content/src/ --include="*.rs"
```

Known affected files (as of Epic 6 creation):
- `src/adapters/input/http_content_fetcher.rs`
- `src/adapters/input/file_content_list_fetcher.rs`
- `src/adapters/input/news_api_fetcher.rs`
- `src/use_cases/content_llm_analysis_service.rs` (now `src/services/`)

Replace every occurrence of `crate::use_cases` with `crate::services`.

**Deliverables:**
- Zero `crate::use_cases` references remain in `crates/paladin-content/src/`.
- `cargo build -p paladin-content` succeeds.

---

#### Task 6.4: Update `paladin-content/README.md`

**Description:** `crates/paladin-content/README.md` contains prose and code examples that
reference `use_cases`. Update them to `services`.

Known occurrences:
- Module description line referencing `use_cases`.
- `use paladin_content::use_cases;` import example.
- `type_name` example using `use_cases::`.

**Deliverables:**
- All `use_cases` references in `README.md` updated to `services`.

---

#### Task 6.5: Verify Facade Re-export Bridge Resolves

**Description:** The facade's `src/application/services/content/mod.rs` already references
`paladin_content::services::*` (the correct post-rename path). After completing Tasks 6.1–6.4
this file requires no changes. Confirm the errors are resolved by building with the feature
flag enabled:

```bash
cargo build --workspace --features content-processing 2>&1 | grep -E "error|warning.*use_cases"
```

The build must emit zero errors. Residual warnings are acceptable if pre-existing.

**Deliverables:**
- `cargo build --workspace --features content-processing` exits 0.
- Zero `E0432 unresolved import` errors against `paladin_content::services`.

---

#### Task 6.6: Quality Gate and Commit

**Description:** Run the full quality gate and commit.

```bash
cargo build --workspace
cargo test --workspace
cargo test --workspace --features content-processing
cargo clippy --workspace -- -D warnings
cargo fmt --all -- --check
```

**Deliverables:**
- All five commands exit 0.
- Changes committed with message:
  ```
  fix(m8-e6): rename use_cases -> services in paladin-content
  - git mv src/use_cases -> src/services
  - Updated lib.rs: pub mod services
  - Updated internal crate::use_cases refs in adapter files
  - Updated README.md examples
  - Resolves E0432 unresolved import errors in facade content/mod.rs
  - Closes broken re-export bridge introduced by Epic 4 rename
  ```

---

### Success Criteria

- `crates/paladin-content/src/use_cases/` no longer exists.
- `crates/paladin-content/src/services/` contains all service files.
- `cargo build --workspace` exits 0.
- `cargo build --workspace --features content-processing` exits 0.
- `cargo test --workspace` all pass.
- Zero `crate::use_cases` references remain in `crates/paladin-content/`.
- Naming is consistent: both the `paladin-content` leaf crate and the facade re-export
  bridge use the `services` path.

---

### Files Affected

| File | Change |
|------|--------|
| `crates/paladin-content/src/use_cases/` (directory) | Renamed to `src/services/` |
| `crates/paladin-content/src/lib.rs` | `pub mod use_cases` → `pub mod services` |
| `crates/paladin-content/src/adapters/input/http_content_fetcher.rs` | `crate::use_cases` → `crate::services` |
| `crates/paladin-content/src/adapters/input/file_content_list_fetcher.rs` | `crate::use_cases` → `crate::services` |
| `crates/paladin-content/src/adapters/input/news_api_fetcher.rs` | `crate::use_cases` → `crate::services` |
| `crates/paladin-content/src/services/content_llm_analysis_service.rs` | `crate::use_cases` → `crate::services` (path also updated by directory rename) |
| `crates/paladin-content/README.md` | `use_cases` → `services` in prose and examples |
| `src/application/services/content/mod.rs` | **No change required** — already references `paladin_content::services::*` |
