# Deprecation Tracking - Epic 2: API Hardening

> **Correction (dated 2026-08-06, DEBT-02 / ADR-0022):** Milestone 4 Epic 2 FR-8 — the requirement
> this document tracks — is **withdrawn**. See
> [ADR-0022](../../../../.planning/decisions/0022-deprecation-requirement-withdrawal.md) for the
> full record. This document's zeros (§"Current Status"'s `Deprecated Items: 0 (none yet)`, §"Deprecation
> Log"'s `*No deprecations added yet.*`) are the **outcome** of this epic's own decisions, not a gap:
> its own §"⚠️ IMMEDIATE DEPRECATION" category — the only category that would ever produce a
> `#[deprecated]` attribute — lists, in its own words, "None identified yet." Re-run this session:
> `grep -rn '#\[deprecated' src crates | wc -l` → **0**; `grep -rn 'doc(hidden)' src crates | wc -l` →
> **38**, confirming the §"🔒 SOFT DEPRECATION" category *was* executed via `#[doc(hidden)]` even
> though this category was not. §"⚠️ IMMEDIATE DEPRECATION" is annotated below as **Confirmed** (its
> text is correct as written, not struck); the stale Deprecation Timeline is struck and restated;
> §"Current Status" and §"Deprecation Log" are annotated as terminal, not in-progress; all four
> §"Open Questions" are closed. Original text is retained below with inline annotations — nothing is
> deleted.

**Created:** 2026-04-15
**Epic:** Epic 2 - Milestone 4, Tier 1
**Purpose:** Track all deprecation warnings for API transition

---

## Deprecation Strategy

This document tracks types being deprecated as we transition from glob re-exports to a curated public API surface.

### Deprecation Timeline

~~- **v0.1.0** (current): All types exported via glob~~
~~- **v0.2.0** (this Epic): Add deprecation warnings, curated exports begin~~
~~- **v0.3.0** (future): Remove deprecated exports, finalize stable API~~
~~- **v1.0.0** (future): Stable public API guarantee~~

**Corrected (dated 2026-08-06, DEBT-02 / ADR-0022):** This schedule is stale by five minor
versions — the workspace ships at **0.7.0** (root `Cargo.toml:34`, re-verified this session:
`version = "0.7.0"`), five minor versions past the `v0.2.0` this schedule anchors on. Per
[ADR-0008](../../../../.planning/decisions/0008-workspace-version-0-7-0.md), the pre-1.0 series
absorbs API evolution through minor bumps rather than a named-release removal schedule, so a
future deprecation's removal window is **"at least one minor version"** rather than a named
release that has already shipped and passed. This timeline is superseded by that restatement, not
deleted — it remains above as the epic's original v0.2.0-era plan.

### Deprecation Categories

1. **Immediate Deprecation** - Types with clear migration paths
2. **Soft Deprecation** - `#[doc(hidden)]` but still accessible for advanced use
3. **Internal-Only** - Change to `pub(crate)` (no deprecation needed)

---

## Deprecation Decisions

### ✅ KEEPING PUBLIC (No Deprecation)

These types remain part of the stable public API:

#### Stable API Surface
- **Port Traits** (25): Primary abstraction layer - always public
- **Domain Entities** (50-60): Core types used by port trait signatures
- **Builders** (9): Fluent API for construction
- **Configuration** (5-10): User-facing configuration
- **Error Types** (15-20): Proper error handling requires public errors

#### Practical Utilities (Documented as Stable)
- **`CircuitBreaker`** - Used in examples, valuable utility pattern
- **`PaladinExecutionService`** - Needed for direct execution scenarios
- **Battalion execution services** - Used in examples (Formation, Phalanx, Campaign services)
- **Vision types** (VisionContent, ImageDetail, VisionError) - Domain entities

**Rationale:** These types have proven valuable in real usage (examples/tests) and don't expose unwanted implementation details.

---

### 🔒 SOFT DEPRECATION (Advanced/Unstable API)

Types that remain accessible but are not part of the stable API contract.

#### Adapter Implementations
**Status:** Mark as `#[doc(hidden)]` but keep public

**Types:**
- `OpenAIAdapter`, `AnthropicAdapter`, `DeepSeekAdapter`
- `InMemoryGarrison`, `SqliteGarrison`
- `QdrantSanctum`, `InMemorySanctum`
- `RedisAdapter` (Queue)
- `MinioAdapter` (File Storage)
- All other adapter implementations

**Migration Path:**
- **Preferred:** Use port traits (`LlmPort`, `GarrisonPort`, etc.)
- **Alternative:** Use factory functions (to be added in Epic 3)
- **Advanced:** Direct instantiation still works but undocumented

**Deprecation Annotation:** NO - Keep accessible but hide from documentation

**Rationale:**
- Examples need direct adapter instantiation for simplicity
- Testing requires direct instantiation
- Advanced users may need access for custom implementations
- Port traits are the stable API, adapters are implementation details

---

### ⚠️ IMMEDIATE DEPRECATION

Types with clear alternatives that should be deprecated now.

#### Category: Manager Services
**Status:** Add `#[deprecated]` in v0.2.0, remove in v0.3.0

**List:**
None identified yet - managers are currently pub(crate) or will be moved to application layer (Epic 3)

**Confirmed (dated 2026-08-06, DEBT-02 / ADR-0022):** This "None identified yet" answer is correct
as written and needs no strike — it is now the **final** answer, not a pending one. This is the
only category in this document that would ever produce a `#[deprecated]` attribute, and it names no
candidate; that is the primary evidence ADR-0022 cites for withdrawing Milestone 4 Epic 2 FR-8.
`grep -rn '#\[deprecated' src crates | wc -l` → **0**, re-confirmed this session.

**Migration Path:** TBD based on Epic 3 refactoring

---

#### Category: Repository Implementations
**Status:** Change to `pub(crate)` (internal-only, no deprecation needed)

**List:**
- All MySQL repository implementations
- All SQLite repository implementations
- Repository connection managers
- Repository query builders

**Migration Path:** None needed - these were never intended as public API

---

#### Category: Internal Utilities
**Status:** Evaluate case-by-case

**List:**
- TBD based on usage analysis

---

### 🚫 INTERNAL-ONLY (No Deprecation - Just Restrict)

Types that should never have been public.

#### CLI Modules
**Status:** Change to `pub(crate)` immediately

**Types:**
- All CLI command handlers
- All CLI formatters
- All CLI utilities
- Progress bars, prompts, etc.

**Migration Path:** None - CLI is binary-only, not library API

---

#### Web Server Modules
**Status:** Change to `pub(crate)` and feature-gate

**Types:**
- All route handlers
- All middleware
- WebSocket infrastructure

**Migration Path:** None - web server is optional feature, not library API

---

## Deprecation Annotations

### Template for Deprecation Warnings

```rust
#[deprecated(
    since = "0.2.0",
    note = "Use `PreferredAlternative` instead. \
            See MIGRATION.md for migration guide. \
            This type will be removed in v0.3.0."
)]
pub struct DeprecatedType { /* ... */ }
```

### Guidelines

1. **Always provide `since` field** with version number
2. **Always provide `note` field** with:
   - What to use instead
   - Link to migration documentation
   - Removal timeline
3. **Test that deprecation warnings compile correctly**
4. **Document all deprecations in CHANGELOG.md**

---

## Current Status

### v0.2.0 (Epic 2 - In Progress)

**Completed:**
- ✅ API audit classified all types
- ✅ Deprecation strategy defined
- ✅ DEPRECATIONS.md created

**In Progress:**
- ~~⏳ Adding deprecation warnings (Task 3.0)~~
  **Closed (dated 2026-08-06, DEBT-02 / ADR-0022):** This item is closed by withdrawal, not
  completion. Milestone 4 Epic 2 FR-8 is withdrawn per ADR-0022 — there is no deprecation-warnings
  task left in progress; the epic's own §"⚠️ IMMEDIATE DEPRECATION" category named no candidate to
  warn about.
- ⏳ Curating explicit exports (Task 6.0)
  **Annotated (dated 2026-08-06, DEBT-02):** This item is **outside DEBT-02's scope** — ADR-0022
  withdraws FR-8's *deprecation* requirement only, not this epic's separate export-curation task.
  Its state is recorded here truthfully as unchanged, not absorbed into this withdrawal. Phase 8
  did not adjudicate it.

**Deprecated Items:** 0 (none yet)

**Confirmed (dated 2026-08-06, DEBT-02 / ADR-0022):** This zero is the terminal state for the 0.7.0
tree, not an in-progress count. See ADR-0022.

**Restricted Items:** 0 (to be done in Task 6.0)

---

## Deprecation Log

*No deprecations added yet.*

**Confirmed (dated 2026-08-06, DEBT-02 / ADR-0022):** This is accurate and is now the **terminal**
state for the 0.7.0 tree, not a pending one — Milestone 4 Epic 2 FR-8 is withdrawn per ADR-0022.
The format block below is retained unchanged for use if a future ADR explicitly supersedes ADR-0022
and re-instates a deprecation requirement.

### Format:
```
- **Type:** `TypeName`
- **Module:** `path::to::module`
- **Deprecated In:** v0.2.0
- **Reason:** Brief explanation
- **Migration:** Use `Alternative` instead
- **Removal:** v0.3.0
```

---

## Open Questions

1. **Adapter Visibility Strategy:** Confirm `#[doc(hidden)]` approach vs. full deprecation
   **Resolved (dated 2026-08-06, DEBT-02 / ADR-0022):** `#[doc(hidden)]` was the approach taken —
   `grep -rn 'doc(hidden)' src crates | wc -l` → **38** occurrences confirmed tree-wide this session.
2. **Factory Functions:** Should we provide factory functions in v0.2.0 or wait for Epic 3?
   **Moot (dated 2026-08-06, DEBT-02 / ADR-0022):** No deprecation timeline exists to schedule this
   against — Milestone 4 Epic 2 FR-8 is withdrawn per ADR-0022. Whether factory functions are
   independently worth adding is a live design question outside DEBT-02's scope.
3. **Prelude Module:** Should we add `paladin::prelude::*` for common imports?
   **Closed — answered by shipped code (dated 2026-08-06, DEBT-02):** Yes, and it already exists.
   `src/prelude.rs` re-exports `Paladin`, `PaladinConfig`, `PaladinData`, `PaladinStatus`,
   `BattalionConfig` and `BattalionError` for `use paladin::prelude::*`. Verified this session —
   file present at that path.
4. **Manager Refactoring:** Wait for Epic 3 (Tier 3) before deprecating manager types?
   **Moot (dated 2026-08-06, DEBT-02 / ADR-0022):** No deprecation timeline exists to schedule this
   against — Milestone 4 Epic 2 FR-8 is withdrawn per ADR-0022. Manager types' `pub(crate)` /
   application-layer placement is Epic 3's own concern, unaffected by this withdrawal.

---

## Related Documents

- [API Audit](api-audit.md) - Full classification of current API surface
- [STABLE_API.md](../STABLE_API.md) - Definition of stable public API contract
- [MIGRATION.md](../docs/MIGRATION.md) - Migration guides (to be created)
- [CHANGELOG.md](../CHANGELOG.md) - Version history

---

**Next Steps:**
1. Review and confirm deprecation strategy with team
2. Add deprecation warnings to selected types
3. Test that deprecations compile correctly
4. Document in CHANGELOG.md
5. Update examples to use preferred patterns (where applicable)
