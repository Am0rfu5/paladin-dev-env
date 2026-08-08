# Epic 4 License Compatibility Decision Checklist

> **CONFIRMED AND NOW DECLARED — 2026-08-08, per [ADR-0025](../../../.planning/decisions/0025-licence-posture.md).**
> This checklist's target policy, `MIT OR Apache-2.0`, was the only signed governance artefact in
> the corpus with a named approver (`DF3NDR`) while the shipped root `Cargo.toml` and all ten
> library crate manifests declared `license = "MIT"` — a three-way split with the M7 Epic 4 PRD
> §4.7.7 and the M7 overview Acceptance Criterion 1, which also said `MIT`. SEC-02 required this be
> resolved by an explicit human decision, not by inference. At a blocking checkpoint on 2026-08-08,
> the repository owner (`DF3NDR`) selected the dual expression this checklist already recorded.
> Phase 9 Plan 05, Task 2 then set `license = "MIT OR Apache-2.0"` in the root package and all ten
> library crates, renamed `LICENSE` to `LICENSE-MIT`, added a verbatim `LICENSE-APACHE`, and updated
> `README.md` and `Dockerfile.chef`'s OCI label to match. This artefact's policy is no longer merely
> signed — it is now what the tree states. The original text below is retained unmodified as the
> historical sign-off record; nothing below this banner was rewritten.

Date: 2026-05-28
Scope: Task 5.7 (transitive dependency license review)

## Target Policy

Project licensing model: MIT OR Apache-2.0 (Rust-style dual licensing).

Approval rule:
- Licenses that are MIT, Apache-2.0, or an SPDX expression containing a permissive MIT/Apache branch are acceptable by default.
- Non-permissive or unresolved entries require explicit decision and sign-off.

## Current Inventory Summary

- Total inventoried packages: 551
- Unknown license entries: 0 (resolved)
- MPL-2.0 entries observed: `colored 2.2.0`, `colored 3.0.0`
- SPDX alternative expression noted: `r-efi 5.3.0` (`MIT OR Apache-2.0 OR LGPL-2.1-or-later`)

Resolved verification:
- `fuchsia-cprng 0.1.1` is no longer treated as unknown after artifact inspection.
- Evidence: crates.io package `fuchsia-cprng-0.1.1.crate` includes `LICENSE` and `Cargo.toml` with `license-file = "LICENSE"`.
- Effective classification: BSD-3-Clause-style permissive license text.

## Decision Checklist

### A. Policy Alignment

- [x] A1. Confirm formal project policy is MIT OR Apache-2.0 for release decisions.
- [x] A2. Confirm SPDX-alternative handling rule: if MIT or Apache branch exists, dependency is acceptable.
- [x] A3. Record policy approver (name/team) and approval date.

Policy approver:
- `DF3NDR` (repository owner)
- Approval date: 2026-05-28
- Decision: Explicit acceptance of MPL-2.0 dependencies for unmodified use in this project.

### B. Blocking Decisions

- [x] B1. Decide whether `MPL-2.0` dependencies are acceptable in this project policy context.
- [x] B2. If `MPL-2.0` is not acceptable, assign replacement task for `colored` dependency chain. [N/A: MPL-2.0 accepted for unmodified use.]
- [x] B3. Verify `fuchsia-cprng 0.1.1` effective license from upstream source and update inventory status.

### C. Evidence and Traceability

- [x] C1. Attach final license inventory evidence used for sign-off.
- [x] C2. Update `release-readiness-audit-report.md` with final pass/fail for Task 5.7.
- [x] C3. Update Task 5.7 status in `tasks-api-stabilization-pre-release-preparation.md`.

Sign-off evidence summary:
- Transitive license inventory reviewed (`cargo metadata` + `jq`).
- `fuchsia-cprng 0.1.1` verified from crates.io artifact as BSD-3-Clause-style (`license-file = "LICENSE"`).
- `r-efi` expression accepted via permissive MIT/Apache branch under policy.
- `colored` (`MPL-2.0`) explicitly accepted for unmodified use.

## Go/No-Go Gate for Task 5.7

Task 5.7 can be marked complete when:
1. MIT OR Apache-2.0 policy approval is recorded.
2. MPL-2.0 has an explicit accept-or-replace decision.
3. Unknown license entries are resolved or replaced.
4. Epic 4 report and task list are updated with sign-off evidence.

Status: COMPLETE
