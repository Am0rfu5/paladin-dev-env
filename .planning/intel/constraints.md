# Constraints (from SPEC-typed docs)

Ingest run 1 of 14 — source set: `.project/Milestone_1-MVP` (36 docs).

**No SPEC-typed documents were present in this ingest run.**

Classification breakdown for this run: 11 PRD, 25 DOC, 0 ADR, 0 SPEC.

No constraint entries are recorded. Note that api-contract-shaped and
schema-shaped material (port trait signatures, SQLite DDL, YAML config
schemas, JSON state schemas, CLI command grammar) does exist in the source
set, but every such document was manifest-typed as PRD or DOC — not SPEC.
That material is therefore captured as requirement acceptance criteria in
`requirements.md` or as context in `context.md`, with its original source
path preserved.

Classifier notes on 9 of the `epic*.md` DOCs explicitly recorded SPEC-like
content signals (Rust type/trait contracts) that were overridden by
`MANIFEST_TYPE=DOC`. If SPEC-level precedence is wanted for that material,
re-tag those docs via `--manifest` and re-run ingest.

---

## Ingest run 2 of 5 — `.project/Milestone_2-Missing_features` + `.project/Milestone_3-Completion` (45 docs)

**No SPEC-typed documents were present in this ingest run either.**

Classification breakdown for run 2: 15 PRD, 30 DOC, 0 ADR, 0 SPEC.

No constraint entries are recorded. As in run 1, constraint-shaped material does exist
in the source set but every carrier document was manifest-typed PRD or DOC, so the
material lives in `requirements.md` acceptance criteria or `context.md` instead, with
its original source path preserved. In run 2 that material includes:

- **api-contract shaped:** `EmbeddingPort`, `SanctumPort`, `DocumentPort`, `VisionPort`,
  `VisionCapableLlm`, `PaladinRegistry`, `SchedulerPort` and `ArsenalPort` trait
  signatures; the Grove LLM routing JSON contract
  (`{"tree_name","agent_id","confidence","reasoning"}`); the `handoff_to_agent` /
  handoff tool JSON schema; the OpenAI and Anthropic vision request/response shapes
  including Anthropic's `{type:"image",source:{type,media_type,data}}` content block.
- **schema shaped:** the Qdrant collection schema (1536-dim vectors, Cosine distance,
  indexed `paladin_id`/`memory_type`/`created_at`/`importance`); the Commander metadata
  export JSON document; the CLI YAML schemas for garrison, arsenal/MCP, Conclave,
  Council, Grove, Maneuver and autonomous features; the `paladin features --format json`
  output shape.
- **nfr shaped:** Sanctum search latency (<500ms at 100K vectors on Qdrant, <100ms at
  10K in-memory), RAG retrieval <500ms p95 and extraction <3s p95, Grove routing <3s,
  Maneuver parse <1ms and orchestration overhead <10ms, vision single-image <5s, PDF
  extraction <2s small / <10s large, registry lookup <1ms, metadata export <50ms,
  Phalanx metrics overhead <1%, and the several competing test-coverage gates.
- **protocol shaped:** MCP STDIO and SSE transport configuration; SSE streaming chunk
  handling for LLM providers.

If any of this should bind at SPEC precedence, re-tag the carrier documents via
`--manifest` and re-run ingest.
