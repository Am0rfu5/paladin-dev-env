No external API integration: Phase 5 writes planning records only — one 118-row cited ledger, three ADRs, one in-place ADR amendment, and a correction to a historical release-notes document; it integrates, wraps, or consumes no external API, SDK, or service.

---

**Why the detector fired.** `api-coverage.cjs` matched the noun `api` in the phase's own success
criterion 3 — *"the divergent Council and Maneuver **API** forms"*. That phrase refers to this
project's **own shipped Rust surfaces** (`crates/paladin-battalion/src/council_service.rs`,
`crates/paladin-battalion/src/maneuver/mod.rs`), which a Milestone-3 release-notes document
describes with the wrong method names. Phase 5 **corrects the document's description** of those
surfaces; it does not call, wrap, or integrate against any API — external or otherwise.

Re-read of the phase scope confirms the phase modifies zero `.rs` files (CONTEXT.md `<domain>`:
*"This phase writes records and decisions. It does not change product code."*). Every code
consequence is routed to Phase 6 (CLOSE-03) or Phase 15 (PIPE-02).

Recorded per the api-coverage contribution's own false-positive path rather than fabricating a
matrix row for a capability surface that does not exist.
