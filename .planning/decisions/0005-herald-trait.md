# ADR-0005: Herald trait signature

## Status

Accepted

**Date:** 2026-07-30

## Context

Two documented `Herald` trait shapes exist in the ingested corpus: Epic 8 FR-1 describes an
infallible form (every method returns `String` or `Option<String>` directly); Epic 8 §6.2 describes
a fallible form (`Result<String, HeraldError>` throughout, plus `finalize_stream`, `name()` and
`mime_type()`). Which shape is authoritative for the Herald output-formatting contract that
`PaladinExecutionService`, the CLI, and downstream integrations format against?

## Decision

- `crates/paladin-core/src/platform/container/herald.rs:49` is authoritative for the `Herald` trait
  shape, and it ships the **v2 fallible form** exactly. In declaration order:
  - `format_paladin_result(&self, result: &PaladinResult) -> Result<String, HeraldError>`
  - `format_battalion_result(&self, result: &BattalionResult) -> Result<String, HeraldError>`
  - `format_stream_chunk(&self, chunk: &StreamChunk) -> Result<Option<String>, HeraldError>`
  - `finalize_stream(&self, metadata: &ExecutionMetadata) -> Result<String, HeraldError>`
  - `format_error(&self, error: &PaladinError) -> String`
  - `name(&self) -> &str`
  - `mime_type(&self) -> &str`
- `format_error` is deliberately the one infallible method on the trait — it returns a bare
  `String`, never a `Result`. This asymmetry is intentional, not an inconsistency to be smoothed
  over: it is what makes Epic 8 FR-10's graceful-degradation requirement expressible at all. If
  `format_error` could itself fail, a Herald implementor would have no way to render a best-effort
  error message when the primary formatting path has already failed — the one place in the trait
  that must never error is the one that formats errors.

## Considered Options

- `REQ-herald-trait-v1` (Epic 8 FR-1, infallible `-> String` returns throughout, `Send + Sync`) —
  rejected; not what shipped, and the infallible form cannot express FR-10's graceful-degradation
  contract at the four call sites that do need to fail (malformed results, unsupported metadata
  shapes, streaming errors, battalion-aggregation errors).
- `REQ-herald-type-consolidation` (run 2, placeholder-type consolidation) — rejected; no placeholder
  or `TODO` type exists anywhere in `herald.rs` for this to consolidate. The trait already re-exports
  the real domain types (`BattalionResult`, `PaladinResult`, `PaladinError`, `TokenUsage`) at the top
  of the file, so there is nothing left to consolidate.

## Code Locations

- `crates/paladin-core/src/platform/container/herald.rs:49` — trait declaration, `pub trait Herald: Send + Sync`
- `crates/paladin-core/src/platform/container/herald.rs:70` — `format_paladin_result`
- `crates/paladin-core/src/platform/container/herald.rs:85` — `format_battalion_result`
- `crates/paladin-core/src/platform/container/herald.rs:111` — `format_stream_chunk`
- `crates/paladin-core/src/platform/container/herald.rs:125` — `finalize_stream`
- `crates/paladin-core/src/platform/container/herald.rs:140` — `format_error`
- `crates/paladin-core/src/platform/container/herald.rs:146` — `name`
- `crates/paladin-core/src/platform/container/herald.rs:153` — `mime_type`

## Code Conformance

conforms

## Downstream Consumers

- Phase 2 GAP-07 — no action required; the shipped trait already matches this decision, so no code
  change follows from this ADR.
- Any future `Herald` implementor (new output formats added to `crates/paladin-herald`, or a custom
  formatter built outside the workspace) must implement this exact signature, including the
  `format_error` infallibility.
