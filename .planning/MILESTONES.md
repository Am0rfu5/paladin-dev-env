# Milestones

## v0.7.1 Milestone 1 close-out (Shipped: 2026-08-04)

**Phases completed:** 4 phases, 38 plans, 88 tasks
**Requirements:** 25/25 satisfied (RECON-01…08, GAP-01…07, QUAL-01…05, REL-01…05)
**Timeline:** 2026-07-30 → 2026-08-03 (5 days, 255 commits)
**Changes:** 213 files, +44,803 / −888
**Git range:** `b926336` → `be2ff05`
**Closeout type:** override_closeout — see Known Verification Overrides below
**Audit:** `milestones/v0.7.1-MILESTONE-AUDIT.md` (status `tech_debt` — all requirements
satisfied, no critical blockers, deferrals recorded with named owners)

**Delivered:** The planning record now describes the shipped v0.7.0 tree as it actually is —
every contested definition settled by an evidence-cited ADR, the residual Milestone-1
functionality finished, coverage and error-path testing made real and measured, and the release's
version, edition, dependency and documentation posture made coherent.

**Key accomplishments:**

- **Nine ADRs settle every contested definition** (`0001`–`0009`), each naming its chosen variant
  and the shipped code it was checked against — `BattalionConfig`, `BattalionResult`, Formation's
  minimum Paladin count, provider-aware temperature range, the `Herald` trait signature, the
  coverage gate, battalion cancellation, workspace version, and Rust edition. `.planning/decisions/`
  and `.planning/ledgers/` were stood up as new document classes to hold them.
- **The Phase 1 decisions were applied in code, not just recorded.** `ProviderCapabilities` gained
  `temperature_range: Option<(f32, f32)>` (making DeepSeek's 0.0–2.0 reachable through
  `PaladinBuilder`), Formation now constructs from a single Paladin, and the citadel placeholder was
  renamed `BattalionCheckpointConfig` across all consumers — including two the plan's own research
  had missed.
- **A real multi-byte panic was found and fixed behind a self-confirming test.**
  `TableHerald::truncate_text` sliced by byte index and panicked on multi-byte UTF-8; it now
  measures by Unicode scalar values, along with the two adjacent panic paths (`format_error`, and a
  `usize` underflow at sub-ellipsis widths) that shared the same defective helper.
- **Coverage was measured offline and gated on one number.** A fully offline
  `rustc -C instrument-coverage` → `llvm-profdata` → `llvm-cov` pipeline (no `cargo-llvm-cov`, no
  network, no Docker) measured 84.79%, which became ADR-0006's single 84% hard-fail floor; Phase 3
  reproduced it verbatim at 85.56% entry and 85.92% exit, closing 4 of 5 zero-coverage files.
- **Previously dead tests were compiled and run for the first time.** 25 `tests/unit/llm/` functions
  and 37 `tests/cli/` tests had never been wired into any test target; all were activated and fixed
  without deleting one. Four `#[ignore]`d Commander stubs became real error-path tests driven by a
  new `FaultyPaladinPort` harness.
- **The release was made coherent and provably green.** All twelve manifests converged on version
  0.7.0 and edition 2024, `cargo audit`/`cargo deny` verdicts were recorded to a provenance
  standard, and the gate suite was measured — 2,924 tests passing, 185 doc tests, all 47 example
  targets building across a four-invocation feature matrix.

**Notable process outcome:** three separate premature or incorrect completions were caught and
reverted rather than shipped — a RECON-07 checkbox flipped before its ADR existed, a stale
"22 examples" figure traced to a Milestone-1 report and corrected at five source locations, and an
OpenAPI baseline invalidated by the version bump. The record self-corrected in each case.

### Known Verification Overrides

**1 override.** Phase 1's `01-VERIFICATION.md` records `passed`, 5/5 must-haves, at
2026-07-31T16:46:51Z, but commit `be2ff05` (2026-08-03) later added `01-04-SUMMARY.md` — a
disposition record for a superseded plan — which pushed the phase directory past that timestamp.
`init.manager` therefore reports phase 1 as `verification_status: stale` /
`phase_complete: false`, which blocks `verified_closeout`.

The commit is documentation-only and states "No ADR, measurement, or source changes"; the passed
verdict stands on its own evidence. Accepted as an override rather than re-verified. See STATE.md
→ Deferred Items.

### Known Gaps and Deferred Work

No unsatisfied requirements. The following are recorded with named owners:

| Item | Owner |
|---|---|
| Herald not reachable from Campaign, Chain of Command, or the Commander router (WARN-01) | Unassigned — candidate for Phase 6 |
| Nyquist validation never reconciled — all 4 phases' `VALIDATION.md` read `status: draft` | `/gsd-validate-phase 1`–`4` |
| Multi-arch Docker build within 500 MB / 300 s (time measured 2946 s; size gate hard, last 86 MB) | Phase 15 / PIPE |
| Kubernetes smoke test within 30 s startup budget | Phase 15 / PIPE |
| Real readiness probes (`k8s/deployment.yaml` runs a placeholder sleep) | Phase 14 / WEB |
| CI observed running on a `release/**` push | Human release gate (D-03) |
| `src/bin/paladin-server.rs` at 0.00% coverage | Phase 5 / VERIFY-05 |
| `minio.rs` outside ADR-0006 default-feature scope | VERIFY-05 / PIPE-02 |
| Two absent bench targets (Paladin execution loop, Arsenal invocation) | No owner, per Phase 3 CONTEXT.md D-12 |
| CR-01 OpenAI adapter reads `user_prompt.context` instead of `.query` (pre-existing since `240eb1f`) | Deferred forward from plan 02-10 |

---
