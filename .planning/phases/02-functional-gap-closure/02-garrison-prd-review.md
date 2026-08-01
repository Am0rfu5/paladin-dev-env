# Epic 2 Garrison Memory System — PRD-Acceptance Review

**Closes:** Task 11.6 (`.project/Milestone_1-MVP/Epic_2/tasks-garrison-memory-system.md:252`), the
second of `REQ-garrison-testing`'s two nested outstanding items
(`.planning/ledgers/milestone-01.md:246`). GAP-06's disposition for Task 11.5 (the coverage check)
is recorded separately below, per D-04.

**Source PRD:** `.project/Milestone_1-MVP/Epic_2/prd-garrison-memory-system.md` — five user stories
(each with an explicit Acceptance Criteria block) and ten functional requirements, FR1-FR10. Both
sets are reviewed below, one row per criterion, at the same evidence bar the Phase 1 ledger uses.

## Evidence bar and how "passing" was confirmed

Every `satisfied` row below carries a `file:line` citation read from the current tree plus a named
test. Per D-19, a citation with nothing exercising it is `present, unproven`, not upgraded.

**How the exercisers were confirmed passing, without re-running `cargo test` in this worktree:**
this plan's own `<build_environment_notes>` restrict additional cargo builds in this worktree
beyond what the pre-commit hook performs, because disk is tight. This plan modifies no Rust
source, so the tree's Garrison code is byte-identical to what Phase 2 Plan 01 already measured in
`02-test-baseline.md`: a full `cargo test --workspace` run showing 2790 passed, 0 failed, 126
ignored, run twice for exact-exit-code confirmation. Two further checks corroborate that every test
cited below sits inside that passing set and was not among the 126 ignored: (1) a direct read of
every Garrison source and test file confirms zero `#[ignore]` attributes —
`grep -rn '#\[ignore' crates/paladin-memory/src/garrison/*.rs
crates/paladin-core/src/platform/container/garrison*.rs
crates/paladin-ports/src/output/garrison_port.rs tests/integration/*garrison*.rs
tests/unit/settings_config_test.rs` returns nothing; (2) every test named below was located by its
exact function name in the current tree, at the cited line, inside a file with no
`#[cfg(feature = ...)]` gate that the workspace-default build would exclude. This is corroborating
evidence from a measured baseline on this same tree, not a fresh run performed in this session —
recorded plainly rather than implied.

---

## Part 1 — User Story acceptance criteria

| ID | Criterion (abbreviated) | Verdict | Evidence |
|---|---|---|---|
| S1.1 | Story 1: Paladin can access previous messages in the current conversation | satisfied | `GarrisonPort::recall_recent` (`crates/paladin-ports/src/output/garrison_port.rs:430`); exercised by `test_remember_and_recall` (`crates/paladin-memory/src/garrison/in_memory_garrison.rs:229`) |
| S1.2 | Story 1: Conversation history is automatically included in LLM prompts | satisfied | `build_prompt_with_custom_system` injects `conversation_history` into the prompt string (`src/application/services/paladin/paladin_execution_service.rs:1032-1046`); exercised by `test_paladin_with_garrison_stores_conversation` (`tests/integration/paladin_garrison_integration_test.rs:82`) |
| S1.3 | Story 1: Memory window prevents context overflow | satisfied | `ConversationHistory::apply_windowing` (`crates/paladin-core/src/platform/container/garrison.rs:344-357`), mirrored in `InMemoryGarrison::apply_windowing` (`crates/paladin-memory/src/garrison/in_memory_garrison.rs:87-99`); exercised by `test_conversation_history_windowing_by_count` (`garrison.rs:490`) and `test_windowing_by_count` (`in_memory_garrison.rs:242`) |
| S2.1 | Story 2: Token counts calculated using LLM-specific tokenizers | satisfied | `TiktokenCounter::count_tokens` (`crates/paladin-memory/src/garrison/token_counter.rs:102`); exercised by `test_count_tokens_simple` (`token_counter.rs:203`) |
| S2.2 | Story 2: Oldest messages intelligently evicted when limits are reached | satisfied | `evict_importance_based` (`crates/paladin-core/src/platform/container/garrison.rs:376-404`); exercised by `test_garrison_token_limit_enforcement` (`tests/integration/paladin_garrison_integration_test.rs:292`) |
| S2.3 | Story 2: System prompts and recent messages are prioritized | satisfied | Protected-set logic — `System` role and `preserve_recent_count` both checked before eviction (`garrison.rs:382-384`); exercised by `test_importance_based_eviction_preserves_system` (`garrison.rs:526`) and `test_garrison_importance_based_eviction` (`tests/integration/paladin_garrison_integration_test.rs:329`) |
| S3.1 | Story 3: Conversation history can be saved to SQLite database | satisfied | `SqliteGarrison::remember` (`crates/paladin-memory/src/garrison/sqlite_garrison.rs:281`); exercised by `test_sqlite_remember_and_recall` (`sqlite_garrison.rs:521`) |
| S3.2 | Story 3: Garrison can be restored with full conversation context | satisfied | `SqliteGarrison::connect` / `recall_recent` (`sqlite_garrison.rs:73`, `:312`); exercised by `test_garrison_recovery_after_restart` (`tests/integration/sqlite_garrison_integration_test.rs:233`), which reconnects across three simulated application restarts and asserts exact entry counts each time |
| S3.3 | Story 3: No data loss occurs during persistence operations | satisfied | Same code path as S3.1/S3.2; exercised by `test_sqlite_garrison_persistence` (`tests/integration/sqlite_garrison_integration_test.rs:20`), which asserts exact content survives a full disconnect/reconnect cycle |
| S4.1 | Story 4: Garrison entries can be stored with vector embeddings | superseded by shipped code | See `.planning/ledgers/milestone-01.md:42` (Divergences, `REQ-garrison-longterm-port`/`REQ-garrison-sqlite`). `LongTermGarrisonPort::remember_with_embedding` (`crates/paladin-ports/src/output/garrison_port.rs:684`) has zero implementations in the tree — `grep -rn "impl LongTermGarrisonPort" crates src` returns nothing. Semantic storage ships instead as Sanctum/Qdrant. Not re-decided here. |
| S4.2 | Story 4: Semantic search returns entries ranked by relevance | superseded by shipped code | Same reasoning as S4.1; `search_similar` (`garrison_port.rs:717`) is unimplemented for any Garrison adapter — ranked similarity search ships via `QdrantSanctumAdapter` (`crates/paladin-memory/src/sanctum/qdrant_adapter.rs:59`), outside Epic 2's own port. |
| S4.3 | Story 4: Search results can be limited to top-K most similar entries | superseded by shipped code | Same reasoning as S4.1/S4.2 — the `limit` parameter exists on the unimplemented `search_similar` signature only; the shipped top-K behavior lives in Sanctum. |
| S5.1 | Story 5: Paladins can be built without Garrison attachment | satisfied | `PaladinBuilder.garrison: Option<Arc<dyn GarrisonPort>>` defaults to `None` (`src/application/services/paladin/paladin_builder.rs:127`); exercised by `test_paladin_without_garrison_single_turn` (`tests/integration/paladin_garrison_integration_test.rs:143`) |
| S5.2 | Story 5: Single-turn execution works without memory | satisfied | Same citation as S5.1; `test_paladin_without_garrison_single_turn` asserts `result.is_ok()` with garrison `None` throughout |
| S5.3 | Story 5: Multi-turn conversations require Garrison and fail gracefully if missing | genuinely outstanding | `PaladinError::GarrisonRequired` is defined (`crates/paladin-core/src/platform/container/paladin_error.rs:54`) and pattern-matched in `is_terminal()` (`:78`), but `grep -rn "GarrisonRequired" src crates` shows it is never *constructed* anywhere — the only other reference is a `match` arm in `crates/paladin-battalion/src/conclave_execution_service.rs:364` that routes an existing value, never produces one. No code path in `paladin_execution_service.rs` detects "multi-turn attempted without Garrison"; execution without a garrison simply proceeds with an empty `conversation_history` (see S1.3/S5.1 evidence — `paladin_execution_service.rs:734-755`, the `else { vec![] }` branch at `:754`). The PRD's "fail gracefully if missing" behavior does not exist in shipped code. |

## Part 2 — Functional Requirements FR1-FR10

| ID | Criterion (abbreviated) | Verdict | Evidence |
|---|---|---|---|
| FR1.1 | `GarrisonEntry` struct with id/role/content/timestamp/metadata/token_count | satisfied | `crates/paladin-core/src/platform/container/garrison.rs:41-54`; exercised by `test_garrison_entry_creation` (`garrison.rs:422`) |
| FR1.2 | Validate required fields before storing | satisfied | `GarrisonEntry::validate` (`garrison.rs:119`), called from `InMemoryGarrison::remember` (`crates/paladin-memory/src/garrison/in_memory_garrison.rs:158`) and `SqliteGarrison::remember` (`crates/paladin-memory/src/garrison/sqlite_garrison.rs:283`); exercised by `test_garrison_entry_validation` (`garrison.rs:432`) |
| FR1.3 | Serialization/deserialization for persistence | satisfied | `#[derive(Serialize, Deserialize)]` on `GarrisonEntry` (`garrison.rs:40`); exercised by `test_garrison_entry_serialization` (`garrison.rs:452`) |
| FR2.1 | `ConversationHistory`: chronological order, max-entry limit, max-token limit, recent-N retrieval | satisfied | `crates/paladin-core/src/platform/container/garrison.rs:270-343`; exercised by `test_conversation_history_add_and_get` (`garrison.rs:471`) and `test_conversation_history_windowing_by_count` (`:490`) |
| FR2.2 | Importance-based eviction: preserve system prompts, preserve recent N, evict oldest middle entries, never evict within limits | satisfied | `evict_importance_based` (`garrison.rs:376-404`); exercised by `test_importance_based_eviction_preserves_system` (`garrison.rs:526`). See "Boundary, empty-input and tie-break notes" below for the exact-threshold and tie-break sub-findings this criterion's wording implies. |
| FR2.3 | Token counts via LLM-specific tokenizers; pluggable; cached | satisfied | `TiktokenCounter` (`crates/paladin-memory/src/garrison/token_counter.rs:48-129`), `TokenCounter` trait for pluggability (`:12`), `cache: RwLock<HashMap<String, u32>>` (`:51`); exercised by `test_count_tokens_caching` (`token_counter.rs:218`) |
| FR3.1 | `GarrisonPort` trait: remember/recall_recent/search/forget_all/stats | satisfied | `crates/paladin-ports/src/output/garrison_port.rs:380-491`; exercised through the trait object by `test_remember_and_recall` (`in_memory_garrison.rs:229`) |
| FR3.2 | All trait methods `Send + Sync` | satisfied | `pub trait GarrisonPort: Send + Sync` (`garrison_port.rs:380`); exercised by every `Arc<dyn GarrisonPort>` held across an `.await` point in `tests/integration/paladin_garrison_integration_test.rs:85` and throughout that file — code that would not compile if the bound were absent |
| FR3.3 | All trait methods return `Result<T, GarrisonError>` | satisfied | Visible on every method signature, `garrison_port.rs:405,430,454,474,491`; exercised by `test_remember_and_recall`'s `.await.unwrap()` calls (`in_memory_garrison.rs:234,236`) |
| FR4.1 | `LongTermGarrisonPort`: `remember_with_embedding`/`search_similar` | superseded by shipped code | See Divergences row cited at S4.1; `garrison_port.rs:684,717`; zero implementations in the tree |
| FR4.2 | Embeddings as `Vec<f32>` | superseded by shipped code | Type signature exists (`garrison_port.rs:687`) but nothing in the tree constructs a `LongTermGarrisonPort` to exercise it |
| FR4.3 | Semantic search ranked by cosine similarity | superseded by shipped code | Ships as `QdrantSanctumAdapter`/`InMemorySanctum` (see Divergences row), not via this Garrison-scoped trait |
| FR5.1 | `InMemoryGarrison`: `RwLock<VecDeque>`, all `GarrisonPort` ops, ephemeral | satisfied | `crates/paladin-memory/src/garrison/in_memory_garrison.rs:58-61` (struct), `:154-221` (trait impl); exercised by `test_remember_and_recall` (`:229`) |
| FR5.2 | `InMemoryGarrison` is the default implementation for quick prototyping | satisfied | `GarrisonSettings::default().garrison_type == "in_memory"` (`crates/paladin-memory/src/config/garrison.rs:31`); exercised by `test_garrison_config_defaults` (`tests/unit/settings_config_test.rs:99-111`) |
| FR6.1 | `SqliteGarrison` adapter: persistent, `garrison_entries` table, all ops, survives restart | satisfied | `crates/paladin-memory/src/garrison/sqlite_garrison.rs:52-103` (struct + `connect`); exercised by `test_sqlite_garrison_persistence` (`tests/integration/sqlite_garrison_integration_test.rs:20`) |
| FR6.2 | Database schema with the specified columns | satisfied, with a structural note | `migrations/001_create_garrison_tables.sql:7-17` carries every mandated column on `garrison_entries` (id, paladin_id, role, content, timestamp, token_count, metadata); exercised implicitly — every passing `SqliteGarrison` test requires this schema to apply cleanly. Structural divergence: the PRD's own schema (prd lines 152-165) places `embedding BLOB` as a column on `garrison_entries` itself; the shipped schema instead uses a separate `garrison_embeddings` table (`migrations/001_create_garrison_tables.sql:63-70`) — a normalization choice, not a missing field, and consistent with embeddings shipping via Sanctum rather than this port. |
| FR6.3 | Connection pooling via sqlx | satisfied | `SqlitePoolOptions::new().max_connections(5)` (`sqlite_garrison.rs:89-91`); exercised by `test_sqlite_connection_pooling` (`tests/integration/sqlite_garrison_integration_test.rs:134`), which spawns 10 concurrent write tasks and 5 concurrent read tasks against a shared pool |
| FR6.4 | Vector search via SQLite-vss extension | superseded by shipped code | See Divergences row cited at S4.1; no `sqlite-vss` dependency or usage anywhere in the tree — `migrations/001_create_garrison_tables.sql:61-62` records this explicitly in a comment ("sqlite-vss would be used ... if enabled") |
| FR7.1 | `PaladinBuilder::with_garrison(port)` | satisfied | `src/application/services/paladin/paladin_builder.rs:658-661`; exercised by every test in `paladin_garrison_integration_test.rs` calling `.with_garrison(...)`, e.g. `:95` |
| FR7.2 | Paladins without Garrison execute single-turn successfully | satisfied | `test_paladin_without_garrison_single_turn` (`tests/integration/paladin_garrison_integration_test.rs:143`), asserting `result.is_ok()` |
| FR7.3 | Paladins without Garrison return `PaladinError::GarrisonRequired` on multi-turn attempts | genuinely outstanding | Same finding as S5.3 above — this is the FR-numbered restatement of the same PRD requirement. `PaladinError::GarrisonRequired` exists but is never returned by any execution path. |
| FR7.4 | `PaladinExecutionService` auto-stores input, retrieves history, injects into prompt, stores response | satisfied | Store user input + retrieve history: `src/application/services/paladin/paladin_execution_service.rs:734-756`; inject into prompt: `:1032-1046`; store response: `:927-931`; exercised end-to-end by `test_paladin_with_garrison_stores_conversation` (`tests/integration/paladin_garrison_integration_test.rs:82`) |
| FR8.1 | `config.yml` schema: type/path/max_entries/max_tokens/tokenizer/eviction_strategy | satisfied | `GarrisonSettings` (`crates/paladin-memory/src/config/garrison.rs:11-26`) matches the PRD's field list exactly; exercised by `test_garrison_config_with_overrides` (`tests/unit/settings_config_test.rs:116-131`) |
| FR8.2 | Configuration loaded via `ApplicationSettings` | satisfied, with a naming note | The successor config struct is named `Settings` (`src/config/settings.rs:27`), not literally `ApplicationSettings` — that struct was deleted workspace-wide in Milestone 6 (recorded in `.planning/PROJECT.md`'s Milestone 4-6 section, not re-decided here). `Settings::get_garrison_config()` (`settings.rs:115`) is the shipped equivalent; exercised by `test_garrison_config_env_overrides` (`tests/unit/settings_config_test.rs:142-160`), which proves both the file-default and the environment-variable override paths |
| FR8.3 | Invalid configuration results in a `GarrisonError::Configuration`-class error | present, unproven | `GarrisonSettings::validate()` exists and is directly unit-tested (`crates/paladin-memory/src/config/garrison.rs:46-80`, exercised by `test_garrison_settings_validation_invalid_type` at `:113-120`), but it returns `Result<(), String>`, not `Result<(), GarrisonError>` — and `grep -rn "\.validate()" src` shows no call site anywhere in the `Settings`-loading path. Nothing wires this validation into config load; an actually-invalid `garrison:` block in `config.yml` currently produces no `GarrisonError::ConfigurationError` at load time. The unit-level check exists and passes; the literal FR8.3 end-to-end behavior does not. |
| FR9.1 | `GarrisonError` enum: StorageError/SerializationError/TokenizationError/NotFound/ConfigurationError | satisfied | `crates/paladin-core/src/platform/container/garrison_error.rs:8-44` (plus one additional `Custom` variant beyond the PRD's five); exercised by `test_storage_error_display` (`:51`) and `test_not_found_display` (`:57`) |
| FR9.2 | Errors implement `std::error::Error` and `Display` via thiserror | satisfied | `#[derive(Error)]` from `thiserror` on `GarrisonError` (`garrison_error.rs:7`); exercised via `.to_string()` in the same two tests above |
| FR10.1 | Unit test coverage at or above the PRD's per-module threshold | superseded by shipped code | See the dedicated "Task 11.5 — coverage disposition" section below. No Garrison-scoped coverage figure of any kind is produced by this review. |
| FR10.2a | Named integration test: `test_sqlite_garrison_persistence` | satisfied | Exact name exists and passes — `tests/integration/sqlite_garrison_integration_test.rs:20` |
| FR10.2b | Named integration test: `test_paladin_with_garrison_context` | satisfied | No identically-named test exists, but the described behavior ("verify multi-turn conversations") is exercised by `test_paladin_multi_turn_conversation` (`tests/integration/paladin_garrison_integration_test.rs:169`), which runs two sequential turns and confirms context carries across them |
| FR10.2c | Named integration test: `test_garrison_recovery_after_restart` | satisfied | Exact name exists and passes — `tests/integration/sqlite_garrison_integration_test.rs:233` |
| FR10.2d | Named integration test: `test_token_limit_enforcement` | satisfied | Described behavior exercised by `test_garrison_token_limit_enforcement` (`tests/integration/paladin_garrison_integration_test.rs:292`) and `test_windowing_by_tokens` (`in_memory_garrison.rs:257`) |
| FR10.2e | Named integration test: `test_semantic_search_accuracy` | superseded by shipped code | Semantic/vector search never shipped as a Garrison capability (see FR4/FR6.4 above); it ships as Sanctum. The nearest Sanctum-side exerciser, `test_store_and_retrieve` (`tests/integration/in_memory_sanctum_tests.rs:38`), sits outside Epic 2's scope and is not claimed here. |
| FR10.3 | All public APIs have rustdoc documentation | present, unproven | Extensive `///` doc comments were observed on every public item read during this review across `garrison.rs`, `garrison_port.rs`, `in_memory_garrison.rs`, `sqlite_garrison.rs`, `token_counter.rs` and `config/garrison.rs`, but no automated exerciser proves *all* public items in the Garrison surface are covered. DOCS-03 (Phase 16) is the open item that would add a `cargo doc` bar with a CI gate; until it lands, this criterion is asserted by inspection, not proven by a passing check. |
| FR10.4 | `cargo clippy` passes with no warnings | satisfied | `.pre-commit-config.yaml`'s `cargo clippy --workspace --all-targets --all-features -- -D warnings` hook gates every commit, including every commit in this plan; corroborated by the Phase 2 Plan 01 measured baseline (`.planning/phases/02-functional-gap-closure/02-test-baseline.md`), which recorded a clean, zero-failure `cargo test --workspace` run against this same tree |

## Boundary, empty-input and tie-break notes

Three properties of Garrison's windowing and eviction behavior are worth stating explicitly rather
than leaving implicit inside the FR2.2 row above:

1. **Exact-threshold behavior.** Both windowing implementations use a strict greater-than
   comparison to decide whether to evict: `while self.entries.len() > self.config.max_entries`
   and `while self.total_tokens() > max_tokens` (`crates/paladin-core/src/platform/container/garrison.rs:347,353`),
   mirrored in `InMemoryGarrison::apply_windowing` (`crates/paladin-memory/src/garrison/in_memory_garrison.rs:89,95`).
   A conversation sitting at *exactly* the configured maximum entry count or *exactly* the
   configured maximum token budget therefore triggers no eviction — read directly from the
   comparison operator, and consistent with `test_garrison_token_limit_enforcement`'s assertion
   that `stats.total_tokens` never exceeds the configured limit of 100 (a value that is never
   itself evicted away once reached, only once exceeded).
2. **Empty and single-entry conversations.** An empty `ConversationHistory` is given an explicit,
   passing verdict rather than being skipped as trivially true: `test_empty_history_operations`
   (`garrison.rs:579`) asserts `len() == 0`, `total_tokens() == 0`, and that `get_recent`/`get_all`
   return empty vectors without panicking. `test_garrison_stats_accuracy`
   (`tests/integration/paladin_garrison_integration_test.rs:600`) separately confirms a freshly
   constructed `InMemoryGarrison`'s `stats()` reports `entry_count == 0` before any entry is
   stored. A single-entry conversation is exercised by `test_remember_and_recall`
   (`in_memory_garrison.rs:229`), which stores exactly one entry and recalls exactly one.
3. **Tie-breaking on equal importance and equal recency.** Among multiple non-recent,
   non-system eviction candidates, `evict_importance_based` always removes the lowest-index
   (oldest) match first — the loop `for i in 0..recent_start_idx { if ... { entries.remove(i);
   return; } }` (`garrison.rs:387-392`) returns on the first hit scanning oldest-to-newest. This
   ordering is **specified by the shipped implementation**, not emergent, for the in-memory
   adapter. `SqliteGarrison`'s eviction and retrieval, by contrast, rely on
   `ORDER BY timestamp DESC` with no secondary sort key (`sqlite_garrison.rs:167,189,211,318`);
   SQLite does not guarantee a stable order among rows with identical `timestamp` values absent an
   explicit tiebreaker column, so among exact-timestamp ties in the SQLite adapter specifically,
   ordering is **SQLite-engine-emergent, not specified by this codebase**. This is a genuine,
   narrow divergence between the two adapters' tie-break guarantees, recorded here rather than
   smoothed into one claim.

## Task 11.5 — coverage disposition

Per D-04 (`.planning/phases/02-functional-gap-closure/02-CONTEXT.md:76-85`), the outstanding item
`- [ ] 11.5 Verify test coverage >= [PRD threshold] using cargo llvm-cov`
(`tasks-garrison-memory-system.md:251`) is dispositioned **superseded by shipped code**.

ADR-0006 (`.planning/decisions/0006-coverage-gate.md`) recorded a single workspace-wide hard-fail
coverage floor, derived from one measured figure against the whole tree, and explicitly retired
every per-module and per-tier position that predates it — including the PRD's own per-module
target this task item was written against. Re-measuring Garrison alone, as task 11.5 literally
asks, would reintroduce exactly the second scope ADR-0006 exists to eliminate: a project would
again be choosing between two coverage numbers instead of holding to the one workspace-wide floor.

**This review produces no coverage figure of any kind for Garrison — advisory, measured, or
estimated.** The forward owner for all coverage-raising work, workspace-wide, is **QUAL-01**
(Phase 3), which ADR-0006 already names as the consumer of its floor. Task 11.5 closes here with
that disposition; it is not re-measured, re-estimated, or partially answered above.

## Summary of verdicts

50 criteria reviewed: 15 User Story acceptance-criteria bullets (Stories 1-5, three bullets each)
plus 35 Functional Requirement sub-items (FR1-FR10, split into their PRD-numbered sub-clauses
where a criterion bundles more than one distinct claim).

| Verdict | Count |
|---|---|
| `satisfied` | 37 |
| `superseded by shipped code` | 9 |
| `genuinely outstanding` | 2 |
| `present, unproven` | 2 |
| `deferred with reason` | 0 |
| **Total** | **50** |

Arithmetic check: Part 1 (User Stories) — 11 `satisfied`, 3 `superseded by shipped code`, 1
`genuinely outstanding`, 0 `present, unproven` = 15. Part 2 (Functional Requirements) — 26
`satisfied`, 6 `superseded by shipped code`, 1 `genuinely outstanding`, 2 `present, unproven` = 35.
11 + 26 = 37 satisfied; 3 + 6 = 9 superseded; 1 + 1 = 2 genuinely outstanding; 0 + 2 = 2 present,
unproven. 37 + 9 + 2 + 2 = 50, matching the table row count exactly.

**The two `genuinely outstanding` rows (S5.3, FR7.3) are the same underlying finding**, stated
once in the PRD's user-story acceptance criteria and once in its functional requirements: Garrison
non-attachment is not enforced against multi-turn use, and `PaladinError::GarrisonRequired` is
dead code — defined, pattern-matched once, never constructed. This is a real gap this review
surfaces rather than smooths over, consistent with the phase's evidence bar.

**The two `present, unproven` rows (FR8.3, FR10.3)** both describe capability that exists and is
directly, narrowly tested, but is not proven to fulfil the PRD's literal end-to-end claim:
`GarrisonSettings::validate()` is unit-tested but disconnected from the config-loading path and
returns the wrong error type; the rustdoc claim is visually true wherever this review looked but
has no automated, exhaustive check behind it.

## What this review does not cover

- **A Garrison-scoped coverage figure of any kind** — explicitly out of scope by D-04 and this
  review's own construction. Owner: **QUAL-01** (Phase 3), via ADR-0006's workspace floor.
- **The two divergence-table criteria** (S4.1-S4.3, FR4.1-FR4.3, FR6.4) are referenced to
  `.planning/ledgers/milestone-01.md`'s existing Divergences row, not re-decided here, per this
  plan's own instruction.
- **Forward ownership for the two newly-surfaced gaps** (FR7.3/S5.3's unenforced
  `GarrisonRequired`, and FR8.3's disconnected `validate()`) is not assigned by this review. Both
  are genuine findings a review of this kind exists to surface, but assigning an owner is a ledger
  decision, not a review-drafting one. **Plan 02-09**, which amends `REQ-garrison-testing` and its
  nested items from this review, is the next place that decision belongs.
- **The module-scoped Herald (>= its own target) and autonomous (>= its own target) coverage
  gates** — unrelated to Epic 2, owned by **VERIFY-05** (Phase 5) per ADR-0006.
- **Re-running the cited exercisers in this session** — see "Evidence bar and how 'passing' was
  confirmed" above; this plan's environment guidance restricts additional cargo builds in this
  worktree, so passing status is corroborated from Phase 2 Plan 01's already-measured baseline on
  this identical, unmodified tree rather than re-executed here.
