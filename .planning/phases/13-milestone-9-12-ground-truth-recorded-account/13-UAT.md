---
status: testing
phase: 13-milestone-9-12-ground-truth-recorded-account
source: [13-VERIFICATION.md]
started: 2026-08-10T21:44:37Z
updated: 2026-08-10T21:44:37Z
---

## Current Test

number: 1
name: Ratify the CR-01 / ORCH-03 scope trade-off
expected: |
  A human confirms this is an acceptable scope boundary for ORCH-03's `[x]` — i.e., that
  ORCH-03's done-when ("anyone applying a run-5 requirement literally cannot write to a path
  that does not exist") is read as scoped to the five items (a)-(e) it names, matching
  ROADMAP.md Phase 13 Success Criterion 4's own narrower wording ("the four stale module and
  document paths are corrected at source, and the agent API's route surface has one answer"),
  not as an unbounded claim over every code example in the tree — OR overrides that reading and
  reopens ORCH-03 / directs an out-of-band `.rs` fix that breaches D-19 deliberately.
awaiting: user response

## Tests

### 1. Ratify the CR-01 / ORCH-03 scope trade-off

expected: `crates/doc-examples/src/sidecar.rs:34` (and its `:25` doc comment) still builds the
unprefixed `/agents/{agent}/execute` URL, embedded verbatim into the rendered
`docs/src/deployment-topologies/sidecar.md` page via mdBook `{{#include}}`, even though that page's
own prose (lines 29-30) correctly states the `/v1`-prefixed route. A reader who copies the rendered
example writes a client that 404s against the live server (`API_V1_PREFIX = "/v1"` at
`crates/paladin-web/src/agent_controller.rs:723`, asserted by `spec_paths_are_versioned_under_v1` at
`crates/paladin-web/src/openapi.rs:103`).

Phase 13 left this uncorrected because fixing it requires a `.rs` edit, which would breach the
phase's own D-19 zero-`.rs` boundary — the same boundary this phase's close-out independently
measures and proves held (`git diff --name-only <base>..HEAD -- '*.rs'` → 0, re-confirmed
independently during verification). The defect, its cause, its exact fix, and its owner (Phase 14)
are recorded in REQUIREMENTS.md's Phase-14 hand-off item 6 and in `13-REVIEW.md` CR-01, with exact
file:line evidence.

why_human: This is an editorial scope-interpretation judgment, not a fact a grep can settle. The
literal done-when sentence is broader than the five named items, but the roadmap's own success
criterion for the same phase is scoped identically to those five items, and the finding was honestly
self-disclosed (not hidden) with a working fix and a named owner.

verifier_recommendation: DEFENSIBLE AS DOCUMENTED — disclosure and deferral done right, not silent
scope-cutting. Do not reopen ORCH-03; confirm Phase 14 hand-off item 6 is the fix vehicle.

result: [pending]

## Summary

total: 1
passed: 0
issues: 0
pending: 1
skipped: 0
blocked: 0

## Gaps
