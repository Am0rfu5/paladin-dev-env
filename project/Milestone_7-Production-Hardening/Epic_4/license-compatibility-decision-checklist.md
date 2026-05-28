# Epic 4 License Compatibility Decision Checklist

Date: 2026-05-28
Scope: Task 5.7 (transitive dependency license review)

## Target Policy

Project licensing model: MIT OR Apache-2.0 (Rust-style dual licensing).

Approval rule:
- Licenses that are MIT, Apache-2.0, or an SPDX expression containing a permissive MIT/Apache branch are acceptable by default.
- Non-permissive or unresolved entries require explicit decision and sign-off.

## Current Inventory Summary

- Total inventoried packages: 551
- Unknown license entries: 1 (`fuchsia-cprng 0.1.1`)
- MPL-2.0 entries observed: `colored 2.2.0`, `colored 3.0.0`
- SPDX alternative expression noted: `r-efi 5.3.0` (`MIT OR Apache-2.0 OR LGPL-2.1-or-later`)

## Decision Checklist

### A. Policy Alignment

- [ ] A1. Confirm formal project policy is MIT OR Apache-2.0 for release decisions.
- [ ] A2. Confirm SPDX-alternative handling rule: if MIT or Apache branch exists, dependency is acceptable.
- [ ] A3. Record policy approver (name/team) and approval date.

### B. Blocking Decisions

- [ ] B1. Decide whether `MPL-2.0` dependencies are acceptable in this project policy context.
- [ ] B2. If `MPL-2.0` is not acceptable, assign replacement task for `colored` dependency chain.
- [ ] B3. Verify `fuchsia-cprng 0.1.1` effective license from upstream source and update inventory status.

### C. Evidence and Traceability

- [ ] C1. Attach final license inventory evidence used for sign-off.
- [ ] C2. Update `release-readiness-audit-report.md` with final pass/fail for Task 5.7.
- [ ] C3. Update Task 5.7 status in `tasks-api-stabilization-pre-release-preparation.md`.

## Go/No-Go Gate for Task 5.7

Task 5.7 can be marked complete when:
1. MIT OR Apache-2.0 policy approval is recorded.
2. MPL-2.0 has an explicit accept-or-replace decision.
3. Unknown license entry is resolved or replaced.
4. Epic 4 report and task list are updated with sign-off evidence.
