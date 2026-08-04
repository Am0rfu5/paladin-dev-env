# ADR-0011: Vision port surfaces and the encryption-at-rest disposition

## Status

Accepted

**Date:** 2026-08-04

## Context

The ingest record treats `VisionCapableLlm` (`crates/paladin-ports/src/output/vision_llm_port.rs`,
Epic 13 lineage) and `VisionPort` (`crates/paladin-ports/src/output/vision_port.rs`, Epic 20
lineage) as a PRD conflict requiring a migration decision — as if only one of the two surfaces
were meant to survive. They sit at different layers and are reached by different entry points:
one is the trait an adapter author implements to add a vision-capable provider, the other is the
surface application code calls through the execution service. Both ship deliberately.

Separately, `REQUIREMENTS.md`'s `REQ-vision-security-encryption` row records Epic 13's
encryption-at-rest requirement as absent from the tree on three counts — no encryption-at-rest
artefact, no zeroization, no retention policy, and no `VisionError::EncryptionError` variant. All
three counts are verified false by direct inspection during this task. Running
`grep -rln "EncryptionService\|DataRetentionPolicy\|VisionError::EncryptionError" src/ crates/ | grep -v "infrastructure/security"`
returns **no output** — the capability exists, compiles, and is self-tested, but nothing outside
`src/infrastructure/security/` consumes it. That zero-consumer fact is the finding the ingest
record actually missed; the "not found in tree" claim is simply wrong.

## Decision

Both vision surfaces are intended long-term. Neither is legacy, and no migration between them is
planned or recommended.

Entry-point guidance, stated explicitly rather than left to inference: `VisionPort`
(`crates/paladin-ports/src/output/vision_port.rs:47`) is the recommended entry point for
application code, reached via `PaladinExecutionService::execute_with_vision`
(`src/application/services/paladin/paladin_execution_service.rs:517`). `VisionCapableLlm: LlmPort`
(`crates/paladin-ports/src/output/vision_llm_port.rs:52`) is the adapter-author surface,
implemented when adding a vision-capable provider, reached via
`PaladinBuilder::enable_vision` (`src/application/services/paladin/paladin_builder.rs:517`).

The encryption disposition is a third verdict, neither shipped-and-wired nor dropped: **built,
self-tested, and never wired**. `EncryptionService`, `DataRetentionPolicy` and
`VisionError::EncryptionError` all exist in the tree and compile, but the zero-consumer grep
above — re-executed for this task, not transcribed from an earlier pass — confirms nothing
outside `src/infrastructure/security/` calls any of them. Concretely: no vision-path image bytes
are encrypted at rest today, regardless of what the module's own doc comment or unit tests claim.

The consequence is a small, concrete wiring question, owned by **Phase 6 CLOSE-03**: either wire
`EncryptionService::encrypt_image_data` into the vision execution path, or record that the
service is a consumer-facing utility the framework deliberately does not impose on every vision
call. No new phase and no new requirement is created for it; this ADR records the disposition and
hands the decision forward.

## Considered Options

- Deprecating `VisionCapableLlm` in favour of `VisionPort` — rejected. Both ship deliberately at
  different layers (adapter-author surface vs. application entry point), and VERIFY-04 forbids
  planning a migration on the strength of the PRD conflict alone.
- Deprecating `VisionPort` in favour of `VisionCapableLlm` — rejected for the same reason; neither
  surface is legacy, so there is no direction in which a deprecation would be correct.
- Recording the encryption requirement as dropped — rejected. `EncryptionService`,
  `DataRetentionPolicy`, and `VisionError::EncryptionError` demonstrably ship, compile, and carry
  their own tests; "dropped" would misstate what is actually in the tree.
- Recording the encryption requirement as shipped/satisfied — rejected. It has zero consumers
  outside `src/infrastructure/security/`, so no vision-path image bytes are actually encrypted at
  rest. Recording it as shipped would cause a downstream reader to skip real security work that
  Phase 6 CLOSE-03 still needs to decide.
- Silently omitting the zero-consumer finding and treating "the code exists" as sufficient —
  rejected. Per this phase's threat register (T-05-06), spoofing a capability's wired state is a
  real risk to a downstream reader sizing follow-up work.

## Code Locations

- `crates/paladin-ports/src/output/vision_port.rs:47` — `VisionPort` trait, the recommended
  application-code entry point
- `crates/paladin-ports/src/output/vision_llm_port.rs:52` — `VisionCapableLlm: LlmPort`, the
  adapter-author surface
- `src/application/services/paladin/paladin_builder.rs:517` — `PaladinBuilder::enable_vision`,
  the entry point for the adapter-author surface
- `src/application/services/paladin/paladin_execution_service.rs:517` —
  `PaladinExecutionService::execute_with_vision`, the entry point for the recommended surface
- `crates/paladin-core/src/platform/container/vision.rs:210-212` — `VisionError::EncryptionError`
  variant, contradicting the ingest record's claim that it does not exist
- `src/infrastructure/security/encryption.rs:200` — `EncryptionService::encrypt_image_data`
- `src/infrastructure/security/encryption.rs:217` — `EncryptionService::decrypt_image_data`
- `src/infrastructure/security/encryption.rs:68` — `SecureData`'s
  `#[derive(Zeroize, ZeroizeOnDrop)]`, the zeroization the ingest record claimed was absent
- `src/infrastructure/security/encryption.rs:95` — `SecureData::is_expired`, a method on
  `SecureData` (distinct from `DataRetentionPolicy::should_retain` below; the two are two methods
  on two different types, not one method under two names)
- `src/infrastructure/security/encryption.rs:131` — `DataRetentionPolicy::should_retain`, the
  retention-policy check the ingest record claimed was absent
- `src/infrastructure/security/mod.rs:44` — `pub use encryption::{DataRetentionPolicy,
  EncryptionError, EncryptionService, SecureData};`, the public re-export
- `src/infrastructure/mod.rs:47` — `pub mod security;`, the module wiring that makes the above
  reachable at all
- `Cargo.toml:134` — `chacha20poly1305 = "0.10"`, unconditional, no feature gate
- `Cargo.toml:135` — `zeroize = { version = "1.8", features = ["derive"] }`, unconditional, no
  feature gate

## Code Conformance

must change

**CLOSE-03 in Phase 6** is the requirement that executes the consequence of this ADR. The vision
half is documentation only: rustdoc entry-point guidance naming `VisionPort` as recommended and
`VisionCapableLlm` as the adapter-author surface, with no removal of either trait. The encryption
half is a recorded wiring decision, not an implementation — CLOSE-03 either wires
`EncryptionService::encrypt_image_data` into the vision path or records the service as a
deliberately unimposed consumer-facing utility. Nothing in this phase edits Rust source; this ADR
records the decision only.

## Downstream Consumers

- Phase 6 CLOSE-03 — implements the entry-point rustdoc guidance and resolves the encryption
  wiring question this ADR names
- Phase 5 ledger plan 05-08 — rewrites the `REQ-vision-security-encryption` row from "Not found in
  tree" to `present, unproven`, citing this ADR and the unwired-consumer finding
- Any adapter author choosing a vision entry point — reads this ADR's `## Decision` section for
  which surface to implement or call
