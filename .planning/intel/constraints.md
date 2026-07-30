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
