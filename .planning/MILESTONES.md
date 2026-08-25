# Milestones

## v0.8.0 Milestone 2-12 close-out & Provider Expansion (Shipped: 2026-08-24)

**Phases completed:** 14 phases (5-17, including inserted 15.1), 149 plans
**Requirements:** 65/65 satisfied (VERIFY-01…06, CLOSE-01…03, ARCH-01…07, DEBT-01…05,
SEC-01…05, HARD-01…07, FACADE-01…04, SUPPLY-01…03, ORCH-01…05, WEB-01…04, PIPE-01…05,
DEFER-01…03, DOCS-01…04, PROV-01…04)
**Timeline:** 2026-08-04 → 2026-08-24 (21 days, 1,014 commits)
**Git range:** `be2ff05` → `48ac11a5`
**Closeout type:** override_closeout — 0 verification overrides (all 14 phases `passed`), 1 open
artifact acknowledged; see STATE.md *Deferred Items*
**Audit:** `milestones/v0.8.0-MILESTONE-AUDIT.md` (status `tech_debt` — all requirements
satisfied, no critical blockers, 25 debt items recorded with owners)

**Delivered:** The four remaining ingest-derived milestone blocks are closed out and the planning
record now describes the shipped tree across all twelve historical milestones — every contested
position answered by an evidence-cited ADR, every verified defect fixed, the quality gates that
were only ever specified now built and measuring, and the first forward work beyond the ingest
shipped as six new LLM provider adapters.

**Key accomplishments:**

- **The record now matches the tree across twelve milestones.** Five as-shipped ledgers carry 554
  `REQ-*` rows with `file:line` verdicts — 118 for Milestone 2-3, 115 for Milestone 4-6, 86 for
  Milestone 7-8, 120 for Milestone 9-12 — replacing PRD paths that predate the workspace
  decomposition. Phases 5, 7, 10 and 13 touched zero `.rs` files, each boundary independently
  re-measured at close.

- **The quality gates Deferred-QA Epic 25 specified and nobody started now exist and run.** The
  `coverage`, `cli-tests`, `bench-check` and `actionlint` CI jobs are wired and green, with the 82%
  line-coverage floor single-sourced across `ci.yml`, the `Makefile` and ADR-0006. Two previously
  blind modules measure 94.21% and 96.90%.

- **Nine LLM providers ship where three did.** Kimi, Qwen, Grok, Ollama, Gemini and a generic
  operator-configured OpenAI-compatible adapter join OpenAI, Anthropic and DeepSeek — five on a
  shared extracted `CompatEngine`, Gemini on Google's own `generateContent` protocol. Live
  four-vendor testing found and closed four real defects, including Grok rejecting every request
  because the shared engine sent `presence_penalty` unconditionally.

- **Security governance became mechanical rather than asserted.** Four divergent RustSec exception
  sets collapsed to one register with an enforcing guard (`scripts/check-advisory-register.sh`),
  the duplicate `cargo audit` job that falsified a completed milestone's success metric was
  deleted, and every suppression carries an owner and a review date.

- **Branch protection went from nothing to enforced.** Three GitHub rulesets are applied and
  verified live — `main` protected with 44 required contexts and no bypass on the merge gate —
  after a 994-commit fast-forward reconciled a trunk that sat 921 commits behind an integration
  branch being used as `develop`.

- **Snyk was measured and removed rather than trusted.** A probe carrying four deliberate
  vulnerabilities returned 0 findings in Rust while the identical four in JavaScript returned 3.
  The mandate was unsatisfiable and had blocked verification in six plans. The resulting honest
  gap — no static taint analysis for first-party Rust — is now owned by Phase 18 in v0.9.0.

### Known Gaps

- **No merge-gating Rust SAST — settled by Phase 18, 2026-08-25.** CodeQL was measured and
  disqualified as a required-check-grade Rust SAST (version-scoped: CodeQL `2.26.3` /
  `rust-queries` `0.1.40` — 3 of 4 rule-aligned, source-wired classes never fired across four
  independent measurements; `385/385` file coverage held on every run). `.github/workflows/codeql.yml`
  is retained advisory-only, not promoted to a required check. `cargo-audit`/`cargo-deny` scan
  dependencies; clippy is a lint; the manual credential-handling review remains the primary
  control. Open item — owner Am0rfu5, revisit 2027-02-25 or on a qualifying CodeQL/`rust-queries`
  release, whichever is first. See
  `.planning/phases/18-rust-sast-evaluate-and-adopt-codeql/18-CODEQL-EVIDENCE.md`.
- **Local coverage reproduction unverified.** CI's 82.39% is confirmed (run `31727496744`); the
  documented local procedure has never been walked on a Docker-capable machine. Owner: repo
  maintainer.
- **Nyquist validation unreconciled** for all 14 phases — every `VALIDATION.md` reads
  `status: draft`, so `nyquist_compliant` is not authoritative (#2117). Phase 06 has none at all.
  A coverage TODO, not a compliance failure.
- **No git tag was cut.** v0.8.0 ships through the normal release process from `main`; this
  milestone closed on an unmerged branch, and the repository enforces main-only tags.

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
