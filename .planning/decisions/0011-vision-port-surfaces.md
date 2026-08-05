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

**Resolution note (Phase 6, 2026-08-05):** CLOSE-03 resolved the open wiring question above in
favour of the deliberately-unimposed-utility option, on ground stronger than this ADR had when it
was written: the shipped vision path never stores image bytes.
`PaladinExecutionService::execute_with_vision`
(`src/application/services/paladin/paladin_execution_service.rs:517`) takes caller-supplied
`Vec<VisionContent>` — URL, base64, or file path — and hands it straight to the vision adapter,
with no framework-owned temporary file or cache anywhere in that path. Epic 13 FR-11's premise,
"encryption at rest for *temporarily stored* image data," therefore has no storage in the shipped
design to protect. `EncryptionService`, `SecureData`, and `DataRetentionPolicy` are documented as
a consumer-facing utility a caller who does hold image bytes at rest may call directly; the
framework itself does not invoke them on the vision path. See D-16 and D-17 in
`.planning/phases/06-verified-gap-closure/06-CONTEXT.md`.

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
- **(Phase 6, D-16)** Wiring `SecureData` zeroization into the in-flight vision path — rejected.
  This would be a real behaviour change, and one that only partly satisfies FR-11 even so, since
  zeroizing a value that is never persisted protects nothing at rest.
- **(Phase 6, D-16)** Wiring `EncryptionService::encrypt_image_data` fully into the vision
  execution path — rejected. With no persistence between encrypt and decrypt on the shipped
  path, this is ceremony rather than protection, and it costs an encrypt/decrypt cycle on every
  vision request for no security benefit.

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

conforms

**CLOSE-03 in Phase 6** is the requirement that executed the consequence of this ADR. The
executed change is rustdoc: a "Choosing a vision surface" section on `VisionPort` and the mirror
section on `VisionCapableLlm` naming the entry points recorded above, plus a "Framework usage"
section on `EncryptionService` recording the deliberately-unimposed-utility disposition this
ADR's resolution note above records. No source behaviour was altered, and neither vision trait
was removed, deprecated, or documented as legacy. The requirement's original framing — either
wire `encrypt_image_data` into the vision path or record the service as unimposed — is resolved
in favour of the latter; see the resolution note in `## Decision` above for the reason.

## Downstream Consumers

- Phase 6 CLOSE-03 — implements the entry-point rustdoc guidance and resolves the encryption
  wiring question this ADR names
- Phase 5 ledger plan 05-08 — rewrites the `REQ-vision-security-encryption` row from "Not found in
  tree" to `present, unproven`, citing this ADR and the unwired-consumer finding
- Any adapter author choosing a vision entry point — reads this ADR's `## Decision` section for
  which surface to implement or call
- Phase 6 plan 06-07 — amends the `REQ-vision-security-encryption` ledger row to the
  deliberately-unimposed-utility verdict this ADR's resolution note records, carrying the same
  disposition through to the ledger
