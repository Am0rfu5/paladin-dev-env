# Deprecation Tracking - Epic 2: API Hardening

**Created:** 2026-04-15  
**Epic:** Epic 2 - Milestone 4, Tier 1  
**Purpose:** Track all deprecation warnings for API transition

---

## Deprecation Strategy

This document tracks types being deprecated as we transition from glob re-exports to a curated public API surface.

### Deprecation Timeline

- **v0.1.0** (current): All types exported via glob
- **v0.2.0** (this Epic): Add deprecation warnings, curated exports begin
- **v0.3.0** (future): Remove deprecated exports, finalize stable API
- **v1.0.0** (future): Stable public API guarantee

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
- ⏳ Adding deprecation warnings (Task 3.0)
- ⏳ Curating explicit exports (Task 6.0)

**Deprecated Items:** 0 (none yet)

**Restricted Items:** 0 (to be done in Task 6.0)

---

## Deprecation Log

*No deprecations added yet.*

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
2. **Factory Functions:** Should we provide factory functions in v0.2.0 or wait for Epic 3?
3. **Prelude Module:** Should we add `paladin::prelude::*` for common imports?
4. **Manager Refactoring:** Wait for Epic 3 (Tier 3) before deprecating manager types?

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
