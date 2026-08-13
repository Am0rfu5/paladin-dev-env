---
created: 2026-08-13T18:18:29Z
title: Verify local make coverage reproduces CI's 82.39% figure
area: testing
severity: minor
files:
  - docs/src/contributing/testing-guide.md
  - Makefile:251-256
  - .github/workflows/ci.yml:605-613
---

## Problem

Plan 15-04 (PIPE-03/PIPE-05) wrote a Code Coverage section in
`docs/src/contributing/testing-guide.md` documenting the two-step local procedure
(`make services-up` then `make coverage`) as reproducing the same measurement CI's `coverage`
job runs. This has never actually been walked end-to-end by a human on a Docker-capable
machine — no authoring environment in Phase 15 has Docker or `cargo-llvm-cov` installed, so the
local half of the reproduction claim is unverified.

The CI half IS confirmed: run 31727496744 (workflow `ci.yml`, commit
`e9e3267f9ae6d8483be3ee52c04ffe6a763cbb37`, the base this plan's worktree branched from) reported

```
Scope: --workspace --features integration-tests (the gated measurement)
Lines:     39233/47618 = 82.39%
Functions: 4604/6115 = 75.29%
```

82.39% truncates toward zero to 82, which is at-or-above the `--fail-under-lines 82` floor
ADR-0006 records — the gate passes. What's NOT confirmed is that a contributor following the
testing guide's documented steps, on a real machine, arrives at the same 82.39% (or a
whole-percent-agreeing) figure locally, and that every command the guide names is sufficient —
i.e., that the guide has no undocumented prerequisite step.

## Solution

On a machine with Docker available:

1. Check out the repo at (or after) commit `e9e3267f9ae6d8483be3ee52c04ffe6a763cbb37`.
2. Follow `docs/src/contributing/testing-guide.md`'s Code Coverage section from the top, running
   only the commands it gives — do not fill in any step from prior knowledge. If a command is
   needed that the page doesn't give, that is a defect in the page; fix it.
3. Run `make services-up`, then `make coverage`. Record the reported line-coverage percentage.
4. Compare against CI's figure for the same commit (82.39% lines, per run 31727496744) at
   whole-percent precision (82).
5. If they disagree, record the difference and, if identifiable, the cause (scope mismatch,
   service not up, toolchain version drift).
6. Confirm `make ci-full`, `make test-cli`, and `make bench-check` each pass locally.
7. Fix any documentation gap found in `docs/src/contributing/testing-guide.md`.

This item deliberately does NOT carry a `resolves_phase` tag — it is expected to outlive Phase 15
and should not be silently closed when that phase completes. Owner: repo maintainer (the user).
