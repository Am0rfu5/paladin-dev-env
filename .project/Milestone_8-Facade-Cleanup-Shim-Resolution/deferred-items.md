# Milestone 8 — Deliberately Deferred Items

**Date:** 2026-06-07
**Status:** Record of intentional non-goals (not bugs / not oversights)
**Related:** [`facade-cleanup-RECONCILIATION-2026-06-04.md`](./facade-cleanup-RECONCILIATION-2026-06-04.md),
Epic 7 [`deferred-features.md`](./Epic_7/deferred-features.md)

This document records the facade-cleanup items that were **consciously left in place** after
Milestone 8's relocation/cleanup work (PR #14) and Epic 7 (PR #17). They are deferred on purpose —
each is either low-ROI/high-churn, an unresolved architecture decision, or out of the milestone's
scope — and are captured here so a future milestone can pick them up deliberately rather than
rediscovering them.

> **Correction (dated 2026-08-08, FACADE-01):** The `### D5` section below (`- **Effort / risk:**
> low / low.` and its `- **Recommendation:**` line) and the `## Suggested grouping` section's
> `- **Quick wins:** D5 (println residue).` line are all premised on D5's 17
> `println!`/`eprintln!`/`dbg!` occurrences being runtime diagnostic residue in library code that
> needs case-by-case conversion judgment. **They are not.** Re-verified 2026-08-08:
> `grep -rn "println!\|eprintln!\|dbg!" src/application/services/ src/infrastructure/` still
> returns exactly **17** occurrences across exactly **6** files (this document's count was and
> remains exact), and every one of the 17 is a `///` or `//!` doc-comment line inside a fenced
> `rust`/`rust,ignore` code block — a rustdoc example, not runtime stdout. Filtering the same grep
> to non-doc-comment lines returns **0**. There is no conversion to perform and no quick win to
> bank. Full per-occurrence disposition:
> `.planning/registers/facade-01-rustdoc-stdout-disposition.md`. This is the corpus's first
> measured case of this document's own count being exact while its characterisation is wrong —
> struck below, corrected inline, nothing deleted.

> Verified against `main` on 2026-06-07.

---

## Already done (for context — NOT deferred)

So readers don't mistake completed work for a gap, the following from the reconciliation plan are
**done** and merged:

- Orphaned dead files deleted; unused `file_content_repository` deleted.
- Half-built `user` CLI command + TensorFlow ML stub + `ml` flag removed (documented in
  `Epic_7/deferred-features.md`).
- `HashMapPaladinRegistry` → `paladin-battalion`; `FileCitadel` → `paladin-memory`.
- Herald formatters → new `paladin-herald` crate.
- MinIO/S3, **Redis**, and SQLite repositories → `paladin-storage` (now non-optional);
  facade re-exports them.
- **Notification** adapters → `paladin-notifications` (facade keeps a re-export only).
- Facade `infrastructure/web/user_controller.rs` **removed** — the facade now re-exports
  `paladin_web::*`; the controller lives solely in `paladin-web`.
- actix-web removed from `paladin-web` (Epic 7); cargo-deny ban added.
- `println!`/`eprintln!` hygiene: reduced from ~435 occurrences to **17 across 6 files**
  in `services/` + `infrastructure/`.

---

## Deferred items

### D1 — `src/core/` re-export shims (KEEP, by decision)

- **What:** `src/core/mod.rs` and `src/core/platform/mod.rs` re-export `paladin_core::*` (plus the
  `platform/mod.rs` battalion `maneuver`/`parser` path injection) so facade-internal code can use
  `crate::core::…` paths.
- **Current state:** present; **~49 facade files** still import via `crate::core::…`.
- **Why deferred:** removing the shims means rewriting those ~49 files to `paladin_core::` /
  `paladin_battalion::` paths — high churn, low functional value, and it would not change behavior
  or the public API. The reconciliation doc's recommendation was **KEEP**.
- **Recommendation:** keep, unless a future milestone wants strict "no facade-internal re-export
  aliases." If pursued: mechanical path rewrite + delete the shims; verify `platform/mod.rs`'s
  maneuver/parser injection is preserved (it carries real logic, not just re-exports).
- **Effort / risk:** medium churn / low risk (mechanical, compiler-checked).

> **Correction (dated 2026-08-08, FACADE-02, plan 11-02):** This rating is superseded by a
> disposition with a named owner. **Verdict: DEFER.** Trigger: a facade-wide no-alias sweep.
> Owner: unassigned-pending that sweep — named as unassigned rather than left blank. ADR-0018
> (the Milestone 6 no-shim posture) does **not** settle this item on its own: it retired old
> paths for *relocated types*, a different construct from `src/core/`'s surviving re-export
> layer, which still fronts live code for 49 importers today. Full record:
> `.planning/decisions/0034-d1-d4-facade-relocation-disposition.md`.

### D2 — `src/core/platform/manager/` services are mis-layered

- **What:** `content_service.rs`, `event_manager.rs`, `user_service.rs` live under
  `core/platform/manager/` but are application/domain services, not facade composition glue.
- **Current state:** all three still in the facade under `core/platform/manager/`.
- **Why deferred:** these are placement/architecture corrections, not slop removal, and
  `user_service` in particular needs a trait/impl split decision. Out of scope for the
  dead-code/relocation pass.
- **Recommendation (from the Epic 1 audit):**
  - `content_service.rs` (`ContentItemService`, pure domain) → `paladin-core`.
  - `event_manager.rs` (`EventService`) → `paladin-core` or a facade app-service module.
  - `user_service.rs` → **split**: trait + DTOs → `paladin-core`/`paladin-ports`; concrete impl
    (depends on repo/log/notification ports + argon2) → a facade app-service.
- **Effort / risk:** medium / medium (touches consumers across the facade + tests).

> **Correction (dated 2026-08-08, FACADE-02, plan 11-02):** This rating is superseded by three
> independent verdicts, one per file. **`user_service.rs`: the split above is WITHDRAWN.**
> Reconciliation commit `6704807` already found no split was needed for the controller case
> (`UserServiceTrait` + DTOs already live in `paladin-core`); the full relocation is owned by
> the run-3 v2 tech-debt item; the tests are owned by DEFER-02 / Phase 15. Three owners, no
> overlap. **`content_service.rs` and `event_manager.rs`: each DEFERS independently**, trigger:
> the architecture-pass milestone named in `## Suggested grouping` below; owner:
> unassigned-pending that milestone. Full record:
> `.planning/decisions/0034-d1-d4-facade-relocation-disposition.md`.

### D3 — Entangled Paladin use-case services (KEEP for now)

- **What:** `src/application/services/paladin/{planning_service, prompt_generation_service,
  temperature_service, handoff_service}.rs` (~2,750 LOC).
- **Current state:** in the facade, tightly coupled to `paladin_builder.rs` and
  `paladin_execution_service.rs`.
- **Why deferred:** candidates to move to `paladin-battalion` (planning/handoff) and `paladin-llm`
  (prompt/temperature), but they are heavily entangled with the builder/execution flow; the
  original audit recommended **keep**. Moving them safely needs the builder/execution coupling
  untangled first.
- **Recommendation:** revisit only alongside a builder/execution refactor; not worth a standalone
  move.
- **Effort / risk:** high / high.

> **Correction (dated 2026-08-08, FACADE-02, plan 11-02):** This rating is superseded by a
> disposition with a named owner. **Verdict: DEFER.** Trigger: the broader builder/execution
> refactor named above. Owner: unassigned-pending that refactor. HARD-05 is answered — ADR-0031
> restated the extracted-crate dependency rule, so the `paladin-battalion`/`paladin-llm` targets
> are legal on the same terms `paladin-content`'s existing `llm` feature already satisfies; the
> remaining question is per-edge (non-default, facade-gated, `cfg`-scoped), answered when the
> trigger fires, not here. Full record:
> `.planning/decisions/0034-d1-d4-facade-relocation-disposition.md`.

### D4 — `content_ingestion_service.rs` placement

- **What:** `src/application/services/content/content_ingestion_service.rs` (~1,211 LOC) —
  content-pipeline domain logic.
- **Current state:** in the facade.
- **Why deferred:** arguably belongs in `paladin-content`, but it orchestrates across several
  facade services; a move needs a dependency-coupling review first.
- **Recommendation:** evaluate moving to `paladin-content` if/when the content pipeline is
  consolidated there.
- **Effort / risk:** medium / medium.

> **Correction (dated 2026-08-08, FACADE-02, plan 11-02):** This rating is superseded by a
> disposition with a named owner. **Verdict: DEFER.** Trigger: the dependency-coupling review
> named above. Owner: unassigned-pending that review. Legal under ADR-0031 on the same terms as
> D3. Full record: `.planning/decisions/0034-d1-d4-facade-relocation-disposition.md`.

### D5 — Residual `println!`/`eprintln!` in services/infrastructure

- **What:** 17 `println!`/`eprintln!`/`dbg!` occurrences across 6 files in
  `src/application/services/` + `src/infrastructure/` (down from ~435).
- **Current state:** small residue after the main hygiene sweep.
- **Why deferred:** the bulk was converted; the remainder needs case-by-case judgment (some may be
  intentional user-facing output rather than diagnostics).
- ~~**Recommendation:** review the 6 files; convert genuine diagnostics to `log::*`, keep
  intentional stdout output.~~
  **Corrected (dated 2026-08-08, FACADE-01):** No conversion review is needed. Re-verified this
  date: all 17 occurrences are `///`/`//!` rustdoc-example lines, zero are runtime diagnostics —
  the filtered grep (`grep -v '///' | grep -v '//!'`) returns 0. Each of the 17 carries a recorded
  per-occurrence disposition in
  `.planning/registers/facade-01-rustdoc-stdout-disposition.md` naming it deliberate rustdoc
  stdout. See `FACADE-01`.
- ~~**Effort / risk:** low / low.~~
  **Corrected (dated 2026-08-08, FACADE-01):** Not applicable — there is no conversion work to
  rate, low or otherwise. This item closes by recorded disposition, not by execution.

---

## Suggested grouping for a future milestone

- ~~**Quick wins:** D5 (println residue).~~
  **Corrected (dated 2026-08-08, FACADE-01):** D5 is not a quick win to bank — it is not
  executable work at all. All 17 occurrences are rustdoc example lines with a recorded disposition
  (`.planning/registers/facade-01-rustdoc-stdout-disposition.md`); nothing is converted or
  reviewed. Struck rather than deleted so the original grouping advice remains legible.
- **Architecture pass (one focused milestone):** D2 (manager services) + optionally D4
  (content_ingestion), since both are layer/placement corrections.
- **Only with a broader refactor:** D3 (entangled Paladin services), and D1 (`core` shims) if a
  "no re-export aliases" policy is adopted.

> **Correction (dated 2026-08-08, FACADE-02, plan 11-02):** The two groupings above are
> superseded by four individual verdicts, each with a named owner and (where deferred) a
> concrete trigger — see `.planning/decisions/0034-d1-d4-facade-relocation-disposition.md`.
> D2's `content_service.rs`/`event_manager.rs` DEFER to the architecture-pass milestone named
> here; `user_service.rs`'s split is WITHDRAWN, out of this grouping's scope. D3 DEFERs to the
> broader refactor named here; D1 DEFERs to a facade-wide no-alias sweep — the sweep is itself
> the trigger, not conditioned on whether the policy is "adopted." The grouping's own text above
> is retained as the input that informed these triggers, not as a live disposition.
