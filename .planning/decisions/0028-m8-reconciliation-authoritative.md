# ADR-0028: Milestone 8's authoritative account — the 2026-06-04 reconciliation

## Status

Accepted

**Date:** 2026-08-08

## Context

Two ingested Milestone 8 documents and one independent reconciliation disagree about what
happened in Milestone 8, and the disagreement is factual, not a matter of one document being
newer.

`Epic_1/facade-audit.md:13-21` records the Epic 1 audit's own totals: "Total files audited:
189", "Files staying (List C): 151", "Files to move (List B): 13", "Files to delete (List A):
25". Its List C intro line, `facade-audit.md:118`, names the 151-file "stays" set as containing
"active bridge shims". Concretely, `facade-audit.md:171` and `:175` place
`input/file_content_fetcher.rs`, `input/file_content_list_fetcher.rs`,
`input/http_content_fetcher.rs`, `input/local_file_fetcher.rs`, `input/news_api_fetcher.rs` and
`logs/error_log_adapter.rs` in that "stays" List C.

`Epic_3/infrastructure-adapter-disposition.md:24-44` runs a 20-row table in which **every** row
reads "**Stays**". Row 3 (`:27`, `adapters/document/`) reads "Active bridge; referenced from
content pipeline examples"; rows 7-11 (`:31-35`, the `adapters/input/*` fetchers) each read
"Active bridge" or "Same as #7"; row 16 (`:40`, `adapters/output/api_content_deliverer.rs`) reads
"Active bridge; consumed by `paladin-web` API layer via facade path".

`facade-cleanup-RECONCILIATION-2026-06-04.md:52-54` records what the tree actually showed on a
fresh file-by-file audit: "Verification performed: `rg \"mod <name>\"` across `src/` returns
nothing for each; the `mod.rs` in each directory only does `pub use paladin_<crate>::...`; the
leaf crate file exists." Its §4 Category 1 table (`:59-73`) lists exactly the files named above —
`document_adapter.rs`, `pdf_extractor.rs`, the five `input/*` fetchers,
`output/api_content_deliverer.rs`, `logs/error_log_adapter.rs`, and
`repositories/mysql_content_repository.rs` — with a **~4,465 LOC subtotal** (`:73`), and states
plainly (`:19-22`): "The original Epic 1 audit and the Epic 3 disposition record contain **factual
errors**: they describe ~4,400 LOC of *orphaned, uncompiled duplicate files* as 'active bridges
that stay.' They are not bridges; they are dead corpses left behind when the real code was copied
into leaf crates."

**The load-bearing fact, stated plainly:** `Epic_1/facade-audit.md` and
`Epic_3/infrastructure-adapter-disposition.md` describe approximately 4,400 LOC of **orphaned,
uncompiled duplicate files** as "active bridges that stay". That is a factual mis-classification —
the files were never compiled at all, because no `mod <name>` declaration for them exists anywhere
under `src/` — not a difference of opinion, and not a matter of one document being newer. The
reconciliation's own header (`facade-cleanup-RECONCILIATION-2026-06-04.md:5`) names both documents
by path: "**Supersedes (corrects):** `Epic_1/facade-audit.md` and
`Epic_3/infrastructure-adapter-disposition.md`."

A second, independent defect in `infrastructure-adapter-disposition.md`: two of its 20 rows
disagree with the PRD that governs it. Row 1 (`:25`, `adapters/arsenal/`) marks the group an M9
extraction candidate targeting a future `paladin-arsenal`, but the governing
`Epic_3/prd-relocate-remaining-misplaced-modules.md:145` states the opposite for the same group:
"`arsenal/` | Stays in facade | MCP wiring is facade composition-root responsibility | **No**".
Row 19 (`:43`, `adapters/sanctum/mod.rs`) invents a target crate "future `paladin-sanctum` (M9)",
but the same PRD's §8 Q2 (`prd-relocate-remaining-misplaced-modules.md:293-299`) resolves sanctum
as folding into "the Milestone 9 `paladin-memory` extraction" — no `paladin-sanctum` crate is ever
named there. Neither `paladin-arsenal` nor `paladin-sanctum` exists in the tree today (`ls crates/`
run this session: `doc-examples`, `paladin-battalion`, `paladin-content`, `paladin-core`,
`paladin-herald`, `paladin-llm`, `paladin-memory`, `paladin-notifications`, `paladin-ports`,
`paladin-storage`, `paladin-web` — ten library crates plus `doc-examples`, neither name present).

## Decision

Under the D-00b precedence order (ADR → shipped tree → `.planning/codebase/` map →
`intel/code-verification.md` → PRD → DOC → task-list checkbox — an ADR that contradicts shipped
code is an instruction to change the code), `facade-cleanup-RECONCILIATION-2026-06-04.md` is the
**authoritative account of Milestone 8**, and both `Epic_1/facade-audit.md` and
`Epic_3/infrastructure-adapter-disposition.md` are **superseded by it**.

**(i) The reproducible orphan test.** The reconciliation's verification method is preserved
verbatim as a runnable, reusable, three-step procedure — not as prose about a procedure:

1. `rg "mod <candidate_file_stem>" src/` — for each of the files in question, this returns
   **zero matches**. No `mod <name>;` declaration exists anywhere under `src/` naming the file, so
   the Rust compiler never includes it in the crate — it is not compiled, regardless of what any
   audit document claims about it.
2. `cat src/<containing_directory>/mod.rs` — the containing directory's `mod.rs` declares only a
   `pub use paladin_<crate>::...` re-export chain, with no `mod <name>;` line for the file in
   question.
3. `test -f crates/paladin-<crate>/src/<equivalent path>` — the destination leaf crate already
   contains an equivalent, live, compiled file.

When all three hold for a given facade file, that file is dead, duplicate weight, not an "active
bridge" — deleting it is zero-risk because it was never part of any compiled artifact. This is the
highest-fidelity verification procedure in the run-4 corpus, and it is reusable by any later phase
auditing facade residue (Phase 11 / FACADE-04 in particular, for the `paladin-arsenal` /
`paladin-sanctum` mentions this ADR's Context section flags).

**(ii) The three in-execution corrections — do not re-delete.** The reconciliation's own
execution log (`facade-cleanup-RECONCILIATION-2026-06-04.md:169-175`) records that the orphan test
was not applied blindly, and three specific files were corrected mid-execution rather than
deleted on the original audit's say-so:

- **`paladin_registry.rs` was not a duplicate.** The facade's 418-LOC implementation
  (`facade-audit.md:371`) was richer than battalion's 67-LOC `pub(crate)` copy, so the richer
  implementation was **consolidated into `paladin-battalion`** (commit `ca7e4e8`,
  "consolidate `HashMapPaladinRegistry` into `paladin-battalion`"), not deleted blindly. **Do not
  re-delete `paladin_registry.rs`'s logic as if it were orphaned** — it is now battalion's own
  registry implementation.
- **The `sqlite_*_repository.rs` files were not redundant.** They were the **active
  default-build implementation**, resolved by making `paladin-storage` non-optional in commit
  `897e77e` ("make `paladin-storage` non-optional, drop facade sqlite fallbacks"), which is why
  that single commit alone removed 1,089 lines net. **Do not re-introduce a facade-local sqlite
  fallback** on the theory that the leaf crate's sqlite support is optional — it no longer is.
- **The remainder genuinely were orphaned** — `mysql_content_repository.rs`, the `input/*`
  fetchers (`file_content_fetcher.rs`, `file_content_list_fetcher.rs`, `http_content_fetcher.rs`,
  `local_file_fetcher.rs`, `news_api_fetcher.rs`), `document/*` (`document_adapter.rs`,
  `pdf_extractor.rs`), `output/api_content_deliverer.rs`, and `logs/error_log_adapter.rs`. These
  were deleted outright in commit `e5b2011` (Tier 1, ~4,465 LOC removed) with zero build or test
  impact, confirming the orphan test's verdict.

**(iii) Epic 3 is complete in substance; Epic 6 is complete despite its own record.**

Epic 3 executed the relocations its own governing PRD (`prd-relocate-remaining-misplaced-modules.md`
§5, `:201-203`) explicitly deferred to Milestone 9. Re-derived this session, independently of the
reconciliation's own prose: `git log --oneline e5b2011~1..a1e4901` returns exactly **15 commits**
(`e5b2011`, `2edc031`, `3d48768`, `ca7e4e8`, `8bd7073`, `66f6c4e`, `897e77e`, `ff829e2`, `5a7c901`,
`cf17559`, `6bfcdb7`, `6704807`, `4c7857e`, `74ddf11`, `a1e4901`) — matching the reconciliation's
own "Final tally: 15 commits" (`facade-cleanup-RECONCILIATION-2026-06-04.md:197`) exactly, even
though the reconciliation's own §7 table only names 14 of the 15 by number (its two tables at
`:157-167` and `:181-188` list 10 plus 4); the unlisted 15th, `6bfcdb7`
("docs(m8): record facade-cleanup execution log and outstanding items"), is the commit that first
wrote the reconciliation document itself into the tree, confirmed via
`git show --stat 6bfcdb7`. `git diff --shortstat e5b2011~1 a1e4901` returns
"70 files changed, 1010 insertions(+), 11262 deletions(-)" — **net 10,252 lines removed**,
matching the reconciliation's stated "~10,250 net LOC removed" (`:197`) to within rounding. Both
figures are **measured**, not merely transcribed from the reconciliation's prose. One new leaf
crate, `paladin-herald`, was created in this range (commit `66f6c4e`). Do not plan Epic 3's
relocations as Milestone 9 candidates — they already executed here.

Epic 6 is complete despite the reconciliation's own §2 table rating it "Not verified; low
priority" (`facade-cleanup-RECONCILIATION-2026-06-04.md:37`) and `deferred-items.md` omitting it
entirely. Verified this session: `crates/paladin-content/src/lib.rs:14` declares
`pub mod services;`, `crates/paladin-content/src/services/` exists on disk
(`test -d crates/paladin-content/src/services` exits 0), and a workspace-wide search —
`grep -rn 'use_cases' src/ crates/ tests/ examples/ benches/ --include='*.rs'` — returns **zero
matches**, run this session and reproducible by any later reader. The `use_cases` → `services`
rename is complete; do not plan Epic 6 as outstanding.

**(iv) The Epic 3 §5 non-goal is split.** The clause at
`prd-relocate-remaining-misplaced-modules.md:211` — "No new crates created. `paladin-herald`,
`paladin-ml`, etc. are not in scope." — names the exact crate, `paladin-herald`, that was then
created in this same milestone (commit `66f6c4e`, above). This ADR records the non-goal
**overridden for `paladin-herald`** (which exists, at `crates/paladin-herald/`, confirmed this
session) **and still holding for `paladin-ml`** (which does not exist, confirmed this session by
`test -d crates/paladin-ml` exiting 1). This split is what **FACADE-03(b)** depends on: Phase 11
must not treat `paladin-herald`'s existence as re-opening the non-goal, and must not treat
`paladin-ml`'s absence as license to create it without its own decision.

## Considered Options

- **Name the reconciliation authoritative and annotate both superseded documents at source**
  (accepted) — the reconciliation's own verification method is reproducible and its verdict
  matches what the tree shows today; annotating at source means a reader who lands on either
  superseded document via search sees the correction inline rather than a live-looking wrong
  answer.
- **Leave the contradiction to a ledger row alone, with no ADR** (rejected) — D-00g reserves
  ledger-only treatment for code-settled defects where no two documents actively disagree; here two
  ingested documents assert the opposite of what the tree shows and of each other's mis-classified
  files, which is a contested position by definition.
- **Rewrite `facade-audit.md` and `infrastructure-adapter-disposition.md` to match the tree**
  (rejected) — D-00c forbids rewriting `.project/` source documents; the mis-classification itself
  is the evidence a later reader needs to understand why the orphan test exists and why it matters,
  and rewriting would erase that evidence.
- **Record the Epic 3 §5 non-goal as wholly overridden, since a new crate was in fact created**
  (rejected) — it still holds for `paladin-ml`, which was never created and has no plan to be;
  collapsing the split into "overridden" would be exactly what FACADE-03(b) would then have to
  re-derive from the git history this ADR already re-derived once.
- **Treat the 15-commit / ~10,250-LOC figures as reconciliation-stated without re-measuring**
  (rejected) — the range resolved cleanly in this checkout (all fifteen commit hashes exist and
  are reachable), so D-00e's evidence bar requires measuring rather than transcribing; the
  discrepancy between the reconciliation's own 14-named-commits table and its 15-commit tally was
  only found by measuring.

## Code Locations

- `Epic_1/facade-audit.md:13-21` — the 189/151/13/25 classification totals.
- `Epic_1/facade-audit.md:118` — the "active bridge shims" framing for the 151-file "stays" List C.
- `Epic_1/facade-audit.md:171,175` — the specific orphaned files (`input/*` fetchers,
  `logs/error_log_adapter.rs`) this document placed in "stays".
- `Epic_1/facade-audit.md:371` — `paladin_registry.rs`'s "stays" disposition, corrected per §(ii).
- `Epic_3/infrastructure-adapter-disposition.md:24-44` — the 20-row all-"Stays" table.
- `Epic_3/infrastructure-adapter-disposition.md:25` — row 1, the `paladin-arsenal` mention
  disagreeing with `prd-relocate-remaining-misplaced-modules.md:145`.
- `Epic_3/infrastructure-adapter-disposition.md:27,31-35,40` — the "active bridge" language for
  `document/`, `input/*`, and `output/api_content_deliverer.rs`.
- `Epic_3/infrastructure-adapter-disposition.md:43` — row 19, the `paladin-sanctum` mention
  disagreeing with `prd-relocate-remaining-misplaced-modules.md:293-299` (§8 Q2).
- `Epic_3/prd-relocate-remaining-misplaced-modules.md:145` — the PRD's own "No" extraction verdict
  for `arsenal/`, contradicted by disposition row 1.
- `Epic_3/prd-relocate-remaining-misplaced-modules.md:211` — the §5 non-goal clause naming
  `paladin-herald` and `paladin-ml`.
- `Epic_3/prd-relocate-remaining-misplaced-modules.md:293-299` — §8 Q2, sanctum folding into
  `paladin-memory`, contradicted by disposition row 19's `paladin-sanctum` invention.
- `facade-cleanup-RECONCILIATION-2026-06-04.md:5` — the reconciliation's own header naming both
  superseded documents by path.
- `facade-cleanup-RECONCILIATION-2026-06-04.md:19-22` — the factual-error statement, ~4,400 LOC of
  orphaned files described as "active bridges that stay".
- `facade-cleanup-RECONCILIATION-2026-06-04.md:37` — the Epic 6 "Not verified; low priority" line
  this ADR overrides with measured evidence.
- `facade-cleanup-RECONCILIATION-2026-06-04.md:52-54` — the orphan-test verification method,
  preserved as the three-step procedure in `## Decision` (i).
- `facade-cleanup-RECONCILIATION-2026-06-04.md:59-73` — the Category 1 orphaned-file table and its
  ~4,465 LOC subtotal.
- `facade-cleanup-RECONCILIATION-2026-06-04.md:157-167,181-188` — the 10-plus-4 named commits.
- `facade-cleanup-RECONCILIATION-2026-06-04.md:169-175` — the three in-execution corrections.
- `facade-cleanup-RECONCILIATION-2026-06-04.md:197` — the "15 commits, ~10,250 net LOC removed, one
  new leaf crate" final tally, independently re-measured in `## Decision` (iii).
- `crates/paladin-content/src/lib.rs:14` — `pub mod services;`, Epic 6's completion evidence.
- `crates/paladin-herald/` — the crate the Epic 3 §5 non-goal named and that was then created;
  confirmed present this session.
- `.planning/intel/code-verification.md:383-394` — the two favourable-direction contradictions
  (Epic 6 and Epic 3) this ADR formalizes into a recorded decision.
- Re-derivation commands run this session: `git log --oneline e5b2011~1..a1e4901 | wc -l` → `15`;
  `git diff --shortstat e5b2011~1 a1e4901` → `70 files changed, 1010 insertions(+), 11262
  deletions(-)`; `grep -rn 'use_cases' src/ crates/ tests/ examples/ benches/ --include='*.rs' | wc
  -l` → `0`; `ls crates/` → ten library crates plus `doc-examples`, `paladin-herald` present,
  `paladin-ml` absent.

## Code Conformance

conforms

## Downstream Consumers

- **Phase 11 / FACADE-02** — the Epic 3 relocations already executed (§(iii) above); FACADE-02's
  Milestone 9 candidate list must not re-plan `minio.rs`, `redis.rs`, `file_citadel.rs`, the
  notification adapters, or `user_controller.rs` as pending relocations.
- **Phase 11 / FACADE-03(b)** — the non-goal split, §(iv): overridden for `paladin-herald`, still
  holding for `paladin-ml`.
- **Phase 11 / FACADE-04** — `Epic_3/infrastructure-adapter-disposition.md` is superseded, and it
  is FACADE-04's subject for the `paladin-arsenal` / `paladin-sanctum` mentions this ADR's Context
  section flags as disagreeing with the governing PRD and naming crates that do not exist.
- **Phase 10 / HARD-01** — the `.planning/ledgers/milestone-07-08.md` rows written by plans 10-09
  and 10-10 for the Milestone 8 Epic 1, Epic 3, and Epic 6 requirement IDs cite this ADR as the
  record that resolves their verdicts, rather than re-deriving the orphan test or the commit
  figures independently.
