---
status: testing
phase: 11-facade-residue-deferred-register-disposition
source: [11-VERIFICATION.md]
started: 2026-08-09T00:45:00Z
updated: 2026-08-09T00:45:00Z
---

## Current Test

number: 1
name: Confirm the FACADE-02 cross-register backstop truth — `user_service.rs` has exactly three non-overlapping owners
expected: |
  Read ADR-0034's D2 sub-decision (lines ~106-118), the run-3 v2 tech-debt item, and
  Deferred-QA Epic 28 / DEFER-02's entry side by side. Confirm the three owners are distinct
  and non-overlapping: the *split* is withdrawn and owned by nobody; the *full relocation* is
  owned by the run-3 v2 tech-debt item; the *tests* are owned by DEFER-02 / Phase 15. No
  document should be left proposing two incompatible next actions on that one file.
awaiting: user response

## Tests

### 1. FACADE-02 cross-register non-collision on `user_service.rs`
expected: ADR-0034's D2 sub-decision, the run-3 v2 tech-debt item, and DEFER-02/Epic 28 name three distinct, non-overlapping owners; no file proposes two incompatible next actions.
why_human: This must_have is marked `verification: backstop`, and 11-VALIDATION.md independently classes it manual-only — no single command proves a cross-document non-collision. The verifier found strong supporting evidence (ADR-0034 names all three owners explicitly; REQUIREMENTS.md lines 2314-2339, 3255-3281 and 4076 corroborate the collision this withdrawal resolves) but abstained rather than pass on inference.
result: [pending]

### 2. ADR-allocation checkpoint was the right call
expected: Reviewing option-a (two ADRs — 0034 for the D1–D4 set, 0035 for `paladin-ml` placement) against option-b (five ADRs, one per item) and option-c (one ADR + ledger rows), confirm option-a was an acceptable resolution. The accepted cost is a coarser supersession unit: a future phase revisiting only D3's verdict must supersede an ADR that also carries D1, D2 and D4.
why_human: The `checkpoint:decision` gate (`gate="blocking"`) was auto-selected by the orchestrator under `AUTO_MODE=true` (a consequence of `--chain`), not independently reasoned by a human. The selection and its trade-off are transparently recorded in 11-01-SUMMARY.md, which flags the item `human_judgment: true`. ADR numbers are never reused (`PROMOTION.md:141`), so 0034/0035 are now spent either way.
result: [pending]

### 3. Spot-check the 8 descriptor-less prohibitions
expected: Each judgment-tier prohibition holds under closer human reading of the full annotated documents — notably: original text is annotated, never deleted or reworded; ADR-0031's unratified status is not laundered; the CLI reintroduction record does not imply purely mechanical re-wiring (it carries a security note); and the ROADMAP criterion-1 amendment remains falsifiable rather than retro-fitted to whatever was found.
why_human: These 8 prohibitions were authored **descriptor-less** (no `check_*` wiring) by the spec-less probe fallback, so each disposes flagged-unverified by design. The verifier's own LLM-judge disposition for all 8 is PASS, backed by `git diff --numstat` zero-deletion evidence and targeted greps — but an LLM-judge confirmation of a judgment-tier prohibition is non-authoritative by protocol.
result: [pending]

## Summary

total: 3
passed: 0
issues: 0
pending: 3
skipped: 0
blocked: 0

## Gaps
