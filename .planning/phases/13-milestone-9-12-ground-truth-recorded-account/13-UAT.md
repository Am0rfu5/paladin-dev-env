---
status: complete
phase: 13-milestone-9-12-ground-truth-recorded-account
source: [13-VERIFICATION.md]
started: 2026-08-10T21:44:37Z
updated: 2026-08-10T22:05:00Z
---

## Current Test

[testing complete]

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

result: pass
ratified_by: human operator, 2026-08-10
ratification_provenance: |
  Obtained interactively from the human operator during the `/gsd-verify-work 13` session on
  2026-08-10, via the runtime's AskUserQuestion prompt. The operator was shown the scope trade-off
  in full — the residue's exact location (`crates/doc-examples/src/sidecar.rs:34` and its `:25` doc
  comment), the two competing readings of ORCH-03's done-when, the D-19 boundary cost of the
  alternative, and the verifier's recommendation — alongside two explicit override options
  ("Reopen ORCH-03" and "Ratify, but fix the .rs now"). The operator selected
  "Ratify — ORCH-03 stays [x]". This is a relayed human decision, not an agent inference, and was
  not auto-approved (auto-mode off for this run).
ratification: |
  ORCH-03's done-when is read as scoped to the five items (a)-(e) it names, matching ROADMAP.md
  Phase 13 Success Criterion 4's identical wording — not as an unbounded claim over every code
  example in the tree. ORCH-03 remains `[x]`. Phase 14 hand-off item 6 in REQUIREMENTS.md is
  confirmed as the fix vehicle for the CR-01 residue.

## Summary

total: 1
passed: 1
issues: 0
pending: 0
skipped: 0
blocked: 0

## Acknowledged Gate Overrides

- gate: api-coverage.verify-pre (ai-integration capability, blocking)
  fired: 2026-08-10, during `/gsd-verify-work 13`
  verdict: false positive — overridden by the human operator via AskUserQuestion, 2026-08-10
  signal: single match, `verb: "(surface)"` + `noun: "api"`
  actual_trigger: |
    The literal token `api-surface` — the name of an internal CI job and script
    (`scripts/check-api-surface.sh`, the `api-surface` job in `.github/workflows/ci.yml`) that
    checks Paladin's OWN public-export baseline. Plans 13-10 and 13-11 corrected stale references
    to that job name, so the token recurs throughout their artifacts. The gate matched the phase's
    subject matter (documentation about an internally-named API-surface job) rather than its
    activity (integrating an external API).
  evidence: |
    - `git diff --name-only ca7afb6..HEAD -- '*.rs' | wc -l` → 0 (no source changed)
    - `git diff --name-only ca7afb6..HEAD -- '*Cargo.toml' | wc -l` → 0 (no dependency added)
    - the same gate returns `detected: false` on completed phases 11 and 12
    - Phase 13 integrates no external API; a COVERAGE.md would enumerate an empty set
  note: |
    The gate remains ENABLED (`workflow.api_coverage_gate: true`). This override is scoped to
    Phase 13 only. Phase 14 carries WEB-01/WEB-02 on the live web surface, where this gate is
    expected to do real work — it was deliberately not disabled.

## Gaps
