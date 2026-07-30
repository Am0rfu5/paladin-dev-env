# Decisions (from ADR-typed docs)

Ingest run 1 of 14 — source set: `.project/Milestone_1-MVP` (36 docs).

**No ADR-typed documents were present in this ingest run.**

Classification breakdown for this run: 11 PRD, 25 DOC, 0 ADR, 0 SPEC.

No decision entries are recorded. Nothing in the source set carried an ADR
status field, a Decision/Consequences structure, or a `locked: true` flag, so
no decision statements are asserted here. Several `epic*.md` DOCs contain
technical design blocks that read like decisions (Rust type and trait
contracts); per the precedence rules these are recorded as context, not as
decisions — see `context.md`.

Locked decisions: 0.

Subsequent ingest runs (Milestones 2-12, Deferred-QA-CICD-Completion,
project-management) may add ADR-typed docs; this file is expected to be
appended to in merge mode.

---

## Ingest run 2 of 5 — `.project/Milestone_2-Missing_features` + `.project/Milestone_3-Completion` (45 docs)

**No ADR-typed documents were present in this ingest run either.**

Classification breakdown for run 2: 15 PRD, 30 DOC, 0 ADR, 0 SPEC. Every
classification carried `locked: false` and `precedence: null`.

Cumulative across runs 1-2: 81 documents ingested, 26 PRD, 55 DOC, **0 ADR, 0 SPEC,
0 locked decisions**. No LOCKED-vs-LOCKED contradiction is possible and none of the
technical positions recorded in `requirements.md` or `context.md` is protected from
being overridden by a future ADR.

Decision-shaped material found in run 2 but NOT recorded as a decision (it sits at
DOC or PRD precedence, not ADR):

- `.project/Milestone_2-Missing_features/Epic_17.5/epic17-5.md` — "Recommended
  Consolidation" chooses `src/application/cli` over `src/cli` on the stated rationale
  that "CLI is an input adapter in the application layer, not infrastructure", and
  directs deletion of the entire `src/cli` tree plus removal of `pub mod cli;` from
  `lib.rs`. This is the only module-ownership decision in the run-2 corpus. It has no
  ADR status field, no Consequences section and no `locked` flag, so it is recorded as
  context. **Strongest ADR candidate in the corpus so far** — see
  `.planning/INGEST-CONFLICTS.md` INFO.
- `.project/Milestone_3-Completion/Epic_19/prd-epic19-herald-consolidation.md` —
  establishes a single source of truth for `PaladinResult`, `BattalionResult` and
  `PaladinError` (Herald imports the real domain types). Decision-shaped, but carried
  by a PRD, so recorded as `REQ-herald-type-consolidation`.
- `.project/Milestone_3-Completion/Post-Epic_24-cleanup/LEGACY_CODE_CLEANUP_PLAN.md` —
  establishes that `adapters/llm/` is the canonical location for LLM adapters and
  `adapters/output/` is legacy. Recorded as context.

Locked decisions: 0.

Runs 3-5 (Milestones 4-12, Deferred-QA-CICD-Completion, project-management) may add
ADR-typed docs; this file remains append-only in merge mode.
