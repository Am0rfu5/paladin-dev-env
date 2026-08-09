# FACADE-04 — Milestone 9 Candidate Triage

**Date:** 2026-08-09
**Subject:** FACADE-04 / `infrastructure-adapter-disposition.md`'s 20-row Milestone 9 candidate
list, triaged into `done` / `not a candidate` / `still open`
**Ledger keys:** `REQ-adapter-disposition-record`, `REQ-m8-epic3-no-extractions`
(`.planning/ledgers/milestone-07-08.md:280,283`)
**Status:** Closed by disposition — zero executable code changes (D-13)

## Triage baseline (D-12)

Every disposition below is derived against **ADR-0028's executed commit range,
`e5b2011~1..a1e4901`, and the live tree — not against
`infrastructure-adapter-disposition.md`'s own "Stays / No change" claims.** Per D-00j, any row
whose relocation falls inside that range is `done` by outcome regardless of what the source
record says about it. This is the mechanism that stops this triage re-planning a relocation that
already happened, which is the exact failure mode FACADE-04 exists to prevent.

## Row identity

The unit of triage is the source table's twenty rows, in the source document's own order 1
through 20. The `input/*` fetcher group is not collapsed: `infrastructure-adapter-disposition.md`
already lists its five named files as five separate numbered rows (7-11), and this triage
preserves that — no grouped source cell is collapsed into one fact here. Twenty source rows,
twenty triage rows, never re-sorted by disposition.

## Re-verification this session (D-00e evidence bar)

```
$ ls crates/
doc-examples  paladin-battalion  paladin-content  paladin-core  paladin-herald  paladin-llm
paladin-memory  paladin-notifications  paladin-ports  paladin-storage  paladin-web
$ ls crates/ | wc -l
11
$ ls crates/ | grep -cE 'paladin-(arsenal|sanctum)'
0
```

Eleven entries, neither disputed name present — re-confirmed independently of ADR-0028's own
`## Decision (iii)` restatement of the same command.

## Triage table

| # | Adapter path (as the source row names it) | Live-tree status (command + result) | Disposition | Reason and authority |
|---|---|---|---|---|
| 1 | `adapters/arsenal/` (source: 5 files — `mcp_protocol.rs`, `mcp_sse_adapter.rs`, `mcp_stdio_adapter.rs`, `resource_controls.rs`, `tool_result_formatter.rs`, `mod.rs`) | `ls src/infrastructure/adapters/arsenal/` → `mcp_protocol.rs mcp_stdio_adapter.rs mcp_streamable_http_adapter.rs mod.rs resource_controls.rs tool_result_formatter.rs` — EXISTS, 6 files, still in the facade (`mcp_sse_adapter.rs` renamed/replaced by `mcp_streamable_http_adapter.rs` since the source table was written; not a relocation, no `paladin-arsenal` involved) | not a candidate | Two verdicts disagree on this row and both are recorded rather than one silently picked (D-00b). The **row's own verdict**: "Yes (List A)" targeting future `paladin-arsenal`. The **governing PRD's verdict**, `prd-relocate-remaining-misplaced-modules.md:154` (table row: `arsenal/` — "Stays in facade" — "MCP wiring is facade composition-root responsibility" — **No**). This triage follows the PRD. Never acted on either way inside `e5b2011~1..a1e4901` — not superseded by outcome, decided on the PRD's authority. |
| 2 | `adapters/citadel/file_citadel.rs` | `ls src/infrastructure/adapters/citadel/` → `mod.rs` — GONE, directory holds only `mod.rs` (re-export shim) | done | Relocated to `paladin-memory` inside `e5b2011~1..a1e4901` (ADR-0028 `## Decision (iii)`); facade keeps a stability re-export. Row's own verdict ("Stays", List B → `paladin-memory` M9) is superseded by outcome — the M9 move already happened in M8. |
| 3 | `adapters/document/` (`document_adapter.rs`, `pdf_extractor.rs`) | `ls src/infrastructure/adapters/document/` → `mod.rs` — GONE, directory holds only `mod.rs` | done | Deleted as an orphaned duplicate, ADR-0028 Category 1, commit `e5b2011` (`git log -1 --format="%H %ai %s" e5b2011` → `e5b2011ef6c17f38b9aa212db276dfbd77722b70 2026-06-04 18:14:25 +0000 chore(facade): delete orphaned dead adapter files (M8 Phase 1)`); the live code already lived in `paladin-content`. |
| 4 | `adapters/file_storage/minio.rs` | `ls src/infrastructure/adapters/file_storage/` → `mod.rs` — GONE, directory holds only `mod.rs` | done | Relocated to `paladin-storage` inside `e5b2011~1..a1e4901`; facade re-exports only. |
| 5 | `adapters/garrison/mod.rs` | `cat src/infrastructure/adapters/garrison/mod.rs` → `pub use paladin_memory::garrison::{InMemoryGarrison, SqliteGarrison, ...}` plus backward-compatible sub-modules — EXISTS as a re-export shim | not a candidate | Kept deliberately. `REQ-garrison-sanctum-bridges-kept` is already `satisfied` in `.planning/ledgers/milestone-07-08.md:282` with named multi-file consumer evidence (`cli/config/loader.rs`, `infrastructure/mod.rs`, 5 examples, 5 integration tests). Matches the row's own "Stays" verdict. |
| 6 | `adapters/herald/` (`json_herald.rs`, `markdown_herald.rs`, `table_herald.rs`) | `ls src/infrastructure/adapters/herald/` → `mod.rs` — GONE as separate files, directory holds only `mod.rs` | done | **Contradicts the row's own "No" verdict.** Extracted to a brand-new `paladin-herald` crate by commit `66f6c4e` (`git log -1 --format="%H %ai %s" 66f6c4e` → `66f6c4e2a804a609ba3c06dfd7fd357cbada5371 2026-06-04 22:59:24 +0000 refactor(herald): extract Herald formatters into new paladin-herald crate (M8 Phase 3)`), inside `e5b2011~1..a1e4901`, despite the M8 Epic 3 §5 non-goal explicitly naming `paladin-herald` as out of scope (ADR-0028 `## Decision (iv)`, the asymmetric non-goal split). |
| 7 | `adapters/input/file_content_fetcher.rs` | `ls src/infrastructure/adapters/input/` → `mod.rs` only — GONE | done | Deleted as an orphaned duplicate, ADR-0028 Category 1, commit `e5b2011`; `paladin-content` already owned the live code. |
| 8 | `adapters/input/file_content_list_fetcher.rs` | `ls src/infrastructure/adapters/input/` → `mod.rs` only — GONE | done | Same authority as row 7 — orphaned duplicate, commit `e5b2011`. |
| 9 | `adapters/input/http_content_fetcher.rs` | `ls src/infrastructure/adapters/input/` → `mod.rs` only — GONE | done | Same authority as row 7 — orphaned duplicate, commit `e5b2011`. |
| 10 | `adapters/input/local_file_fetcher.rs` | `ls src/infrastructure/adapters/input/` → `mod.rs` only — GONE | done | Same authority as row 7 — orphaned duplicate, commit `e5b2011`. |
| 11 | `adapters/input/news_api_fetcher.rs` | `ls src/infrastructure/adapters/input/` → `mod.rs` only — GONE | done | Same authority as row 7 — orphaned duplicate, commit `e5b2011`. Rows 7-11 are each triaged individually per the row-identity definition above, not collapsed into one fact. |
| 12 | `adapters/input/tensorflow_adapter.rs` | `test -e src/infrastructure/adapters/input/tensorflow_adapter.rs` → GONE, entirely | done | **Resolved differently from what the row's own "Action in Epic 3" cell describes.** Deleted outright by commit `3d48768` ("chore(facade): remove half-built user CLI + tensorflow ML stub (M8)", 2026-06-04), not feature-gated behind `cfg(feature = "ml")` as the row claims. This row's subject is FACADE-03(b)'s, not a Milestone 9 relocation — see `.planning/registers/facade-03-removed-features.md` for the reintroduction condition. |
| 13 | `adapters/llm/` (`config_bridge.rs`) | `ls src/infrastructure/adapters/llm/` → `config_bridge.rs mod.rs` — EXISTS | not a candidate | Stays; matches the row's own verdict. Config mapping is a facade-level composition-root concern. |
| 14 | `adapters/logs/` (`error_log_adapter.rs`, `system_log_adapter.rs`) | `ls src/infrastructure/adapters/logs/` → `mod.rs system_log_adapter.rs` — PARTIAL: `system_log_adapter.rs` EXISTS, `error_log_adapter.rs` GONE | not a candidate | **With an inventory correction.** Logging stays facade-level, matching the row's original "No" M9-candidate verdict. But the two-file inventory the row describes (`error_log_adapter.rs`, `system_log_adapter.rs`) is now one file: `error_log_adapter.rs` was orphaned and deleted inside `e5b2011~1..a1e4901` (ADR-0028 Category 1), not "stays" as the row claims. The correction is stated on the row rather than absorbed silently. |
| 15 | `adapters/notifications/` (`email_notification_adapter.rs`, `system_notification_adapter.rs`) | `ls src/infrastructure/adapters/notifications/` → `mod.rs` — GONE as separate files, directory holds only `mod.rs` | done | Relocated to `paladin-notifications`; facade keeps a re-export only. |
| 16 | `adapters/output/api_content_deliverer.rs` | `ls src/infrastructure/adapters/output/` → `mod.rs` — GONE, directory holds only `mod.rs` | done | Deleted as an orphaned duplicate, ADR-0028 Category 1, commit `e5b2011`. The ledger's own correction of its LOC stands: **724 LOC, not 629** — 629 LOC belongs to `tensorflow_adapter.rs` (`.planning/ledgers/milestone-07-08.md:282`, `intel/requirements.md:4513`'s corrected figure). `paladin-web` already re-exports the live equivalent. |
| 17 | `adapters/paladin_registry.rs` | `test -e src/infrastructure/adapters/paladin_registry.rs` → GONE, entirely | done | **Contradicts the row's own "No" verdict.** Consolidated into `paladin-battalion` by commit `ca7e4e8` ("refactor(battalion): consolidate HashMapPaladinRegistry into paladin-battalion (M8 Phase 3)", 2026-06-04) — the richer 418-LOC facade implementation replaced battalion's thinner 67-LOC copy (re-confirmed this session: `grep -n "HashMapPaladinRegistry" crates/paladin-battalion/src/in_memory_registry.rs` → `pub struct HashMapPaladinRegistry` at `:64`). **DO NOT RE-DELETE — the do-not-re-delete marker carries forward**: this was not an orphan deletion, and its logic is now battalion's own registry implementation (`.planning/ledgers/milestone-07-08.md:282` carries the same marker). |
| 18 | `adapters/queue/redis.rs` | `ls src/infrastructure/adapters/queue/` → `mod.rs` — GONE, directory holds only `mod.rs` | done | Relocated to `paladin-storage`; facade re-exports only. |
| 19 | `adapters/sanctum/mod.rs` | `cat src/infrastructure/adapters/sanctum/mod.rs` → `pub use paladin_memory::sanctum::InMemorySanctum;` plus `#[cfg(feature = "qdrant")] pub use paladin_memory::sanctum::QdrantSanctumAdapter;` — EXISTS as a re-export shim | not a candidate | **Its target name is an artefact.** Kept deliberately (`REQ-garrison-sanctum-bridges-kept` `satisfied`, same evidence class as row 5). Two verdicts disagree on the target crate and both are recorded (D-00b), as with row 1: the **row's own verdict** names "future `paladin-sanctum` (M9)"; the **governing PRD's verdict**, `prd-relocate-remaining-misplaced-modules.md:310-316` (§8, resolved decision 2, "`sanctum/mod.rs` in infra adapters — RESOLVED: Stays; not a deletion candidate"), folds sanctum into "the Milestone 9 `paladin-memory` extraction" — a crate that already exists and already owns it. No `paladin-sanctum` crate is named anywhere in the PRD. This triage follows the PRD. |
| 20 | `adapters/scheduling/tokio_cron_adapter.rs` | `ls src/infrastructure/adapters/scheduling/` → `mod.rs tokio_cron_adapter.rs` — EXISTS | not a candidate | Stays; matches the row's own verdict. Single concrete scheduler implementation. |

No row's disposition is blank, and the table is not blanket-marked superseded (D-11): rows 1, 5,
13, 19 and 20 are live, deliberate decisions, and rows 1 and 19 were never acted on either way —
they are decided on the governing PRD's authority, not written off as obsolete.

## Tally

Counted directly from the table above, under the row-identity definition stated at the top of this
document (twenty rows, the `input/*` group already individually numbered by the source):

- **`done`:** rows 2, 3, 4, 6, 7, 8, 9, 10, 11, 12, 15, 16, 17, 18 — **14 rows**
- **`not a candidate`:** rows 1, 5, 13, 14, 19, 20 — **6 rows**
- **`still open`:** **0 rows**

**14 + 6 + 0 = 20.** The sum agrees with the rows above it.

`11-RESEARCH.md`'s `## FACADE-04 Verification Table` section carries two internal figures, both
**superseded by this count**: its Summary paragraph states "11 of the 20", and its own tally
paragraph immediately below states "13 rows" while listing thirteen row numbers (2, 3, 4, 6, 7, 8,
9, 10, 11, 15, 16, 17, 18) that omit row 12 — whose own cell in that same document reads `done`.
Neither figure is the resolved count. The count that reconciles with the rows in this document,
under this document's explicit row-identity definition, is **14 `done` / 6 `not a candidate` / 0
`still open`**, and row 12 is included in the `done` bucket here (it resolved by outcome — deletion
by commit `3d48768` — even though its subject belongs to FACADE-03(b), not to a relocation).

## Zero rows are still open — a finding, not an omission

FACADE-04's own third bucket lands empty. That is a true and useful result, not an incomplete
triage. Every one of the twenty rows is settled either by what already shipped inside ADR-0028's
`e5b2011~1..a1e4901` range (fourteen rows, several — 6, 12, 17 — resolving in a way that
contradicts the row's own original verdict) or by a deliberate decision to keep the code in the
facade (six rows: 1, 5, 13, 14, 19, 20, four of which match the row's own verdict and two of which
— 1, 19 — override the row's stated M9 target with the governing PRD's contrary verdict). There is
no row left in a state that requires a future decision. A reader who expected some rows to remain
open — because the ROADMAP criterion's own phrasing lists three possible buckets — should read this
section rather than assume the triage stopped early or skipped rows.

## The two crate names are artefacts

`paladin-arsenal` (named in `infrastructure-adapter-disposition.md` at lines 10, 36 and 81) and
`paladin-sanctum` (lines 10 and 54) are recorded here as **artefacts of a mis-written table**, not
as future crates, on three grounds:

1. **Neither appears in `ls crates/`.** Re-run this session: eleven entries — `doc-examples`,
   `paladin-battalion`, `paladin-content`, `paladin-core`, `paladin-herald`, `paladin-llm`,
   `paladin-memory`, `paladin-notifications`, `paladin-ports`, `paladin-storage`, `paladin-web` —
   and `ls crates/ | grep -cE 'paladin-(arsenal|sanctum)'` returns `0`.
2. **Each disagrees with the governing PRD on the same row that names it.** `arsenal/` is "Stays in
   facade… **No**" at `prd-relocate-remaining-misplaced-modules.md:154`; `sanctum/mod.rs` folds
   into "the Milestone 9 `paladin-memory` extraction" per §8's resolved decision 2
   (`prd-relocate-remaining-misplaced-modules.md:310-316`) — a crate that already exists and already
   owns it, not a new `paladin-sanctum`.
3. **PROJECT.md's `### Out of Scope` already records exactly this.** `.planning/PROJECT.md`'s
   *Out of Scope* section states: "Building `paladin-arsenal`, `paladin-sanctum` or `paladin-ml`" —
   "none exists. The first two are named only by a superseded disposition record that contradicts
   its own governing PRD (FACADE-04 triages the list)."

This record creates no crate and authorises none. Rows 1 and 19 of the triage table above are the
two rows these names appear on; both follow the PRD's verdict rather than the source row's.

## What this triage does not do

No relocation is planned or executed here. Every row inside ADR-0028's `e5b2011~1..a1e4901` range
is treated as `done` by outcome regardless of what `infrastructure-adapter-disposition.md` says
about it (D-12) — none is re-planned as a Milestone 9 candidate, none is scheduled, and no target
crate is created or authorised by this document.

Separately, the source document's `Date: 2025-01` header
(`infrastructure-adapter-disposition.md:17`) remains inconsistent with every other Milestone 8
document — noted here as residue, not corrected, since correcting it at source would be a rewrite
rather than an annotation (D-00c forbids rewriting `.project/` source documents).
