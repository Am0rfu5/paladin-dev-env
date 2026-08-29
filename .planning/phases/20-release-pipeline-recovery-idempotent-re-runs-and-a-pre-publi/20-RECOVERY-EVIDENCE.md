# Phase 20: Release Pipeline Recovery — Recovery Rehearsal Evidence Log

Phase: 20-release-pipeline-recovery-idempotent-re-runs-and-a-pre-publi
Requirements: PUBOPS-03, PUBOPS-05
Plan: 20-07

This document is the phase's rehearsal evidence log, following the shape of
`19-PUBLISH-EVIDENCE.md`: measured facts, dated and sourced, not summarized-away.
Every crates.io API call in this document requires a `User-Agent` header —
crates.io answers `403` without one (ADR-0026 / `19-PUBLISH-EVIDENCE.md`
convention). Every `curl` call below used
`-H 'User-Agent: paladin-release-check (github.com/DF3NDR/paladin-dev-env)'`.

**Status of this document: in progress.** Task 1 (authorisation) is recorded
below and complete. Task 2 (the human-driven rehearsal itself) has not yet run —
see "Task 2 — not yet executed" below for why and what is required first. This
file will be completed by Task 3 once Task 2's evidence (two run URLs, three
registry snapshots, both outcome tables) is available. Do not read the sections
below as a completed rehearsal record.

## Task 1 — Rehearsal authorisation (decision record)

**Date:** 2026-08-28

**Selected option:** `rehearse-rc3` — "Rehearse on 0.8.1-rc.3 (Recommended)"

**Version chosen:** `0.8.1-rc.3`

**Reasoning (from the plan's Task 1 context, D-14/D-15):** the current registry
state established in `19-PUBLISH-EVIDENCE.md` has all eleven crates at
`0.8.1-rc.2`, so `0.8.1-rc.3` is the next unoccupied prerelease on this line.
Prerelease versions never win default dependency resolution (Phase 19 D-04), so
the blast radius of permanently occupying this version number is minimal. This
is the first live test of Assumption A3 (that a re-run preserves the tag ref the
`crates-io` environment's deployment policy and the OIDC subject claim both
require).

### Pre-collision check (Task 1 acceptance criterion)

Before Task 2 begins, a crates.io query for `0.8.1-rc.3` must return 404 for all
eleven crates — the rehearsal version must not already exist. Queried
2026-08-28T17:22:40Z:

```bash
VERSION="0.8.1-rc.3"
for name in paladin-ai-core paladin-ports paladin-herald paladin-battalion paladin-llm \
            paladin-memory paladin-web paladin-notifications paladin-content \
            paladin-storage paladin-ai; do
  status=$(curl -s -o /dev/null -w '%{http_code}' \
    -H 'User-Agent: paladin-release-check (github.com/DF3NDR/paladin-dev-env)' \
    "https://crates.io/api/v1/crates/${name}/${VERSION}")
  echo "${name}: HTTP ${status}"
done
```

| Crate | HTTP status |
|---|---|
| `paladin-ai-core` | 404 |
| `paladin-ports` | 404 |
| `paladin-herald` | 404 |
| `paladin-battalion` | 404 |
| `paladin-llm` | 404 |
| `paladin-memory` | 404 |
| `paladin-web` | 404 |
| `paladin-notifications` | 404 |
| `paladin-content` | 404 |
| `paladin-storage` | 404 |
| `paladin-ai` | 404 |

All eleven crates return 404 for `0.8.1-rc.3` — no collision. Task 1's
acceptance criterion is satisfied and Task 2 may proceed once its own
precondition (below) is met.

**Note on this table's relationship to Task 2's "before state" snapshot:**
this table satisfies Task 1's own acceptance criterion (a pre-collision check
run before Task 2 begins). Task 2 step 1 requires its own "before state"
snapshot to be captured immediately before the tag is cut, as part of the human
action itself — registry state can only drift forward (a 404 becoming a 200),
never the reverse, but the operator must still capture a fresh snapshot at that
point rather than reusing this one, since time will have passed and this is the
authoritative pre-tag-push record the rehearsal's proof depends on.

## Task 2 — not yet executed

**Status:** blocked on an unmet precondition, not declined.

Task 2's precondition reads: "This phase's branch is merged to `main` and the
local checkout is on `main` and up to date with `origin/main` — `make release`
refuses to run otherwise, and the `crates-io` environment's deployment policy
admits only a `v*.*.*` tag cut from that history."

Checked 2026-08-28T17:22:40Z: the working branch is `chore/19-trusted-publishing`,
which is 76 commits ahead of `main` and has not been merged. The precondition is
unmet. Per the executor's checkpoint protocol, an unmet precondition is never
auto-approved and is not a step the executor performs itself (merging, tagging
and publishing are exactly the human-checkpoint acts Task 2 exists to gate) — it
is surfaced back to the operator as a `checkpoint:human-action`.

This section will be replaced by Task 3 with the full rehearsal record — three
registry snapshots, two run URLs and their conclusions, both per-crate outcome
tables, the OIDC/environment-gate result on the re-run, and the
publish-to-index-visible timing observations — once the operator has merged to
`main` and executed Task 2's instructions, or declined the rehearsal outright
(in which case this section is replaced with the decline record instead).

## Assumptions and limits (preliminary)

This section is provisional pending Task 2/Task 3 completion and will be
finalized then. What is already known not to be proven by this document as it
stands:

- The recovery path itself (re-running a tag-push workflow run after a real
  mid-loop interruption) has not been exercised — this is exactly what Task 2
  is gated on.
- `cargo publish --dry-run` has not been used anywhere in this document as
  evidence, and will not be: a dry run never reaches the publish endpoint and
  can neither create nor recover from a half-published state.
- Assumption A3 (whether a re-run's OIDC token exchange succeeds against the
  `crates-io` environment's deployment policy) remains untested until Task 2
  runs.
- Assumption A1 (`workflow_dispatch` eligibility under Trusted Publishing)
  remains untested regardless of Task 2's outcome unless incidentally observed
  (D-15); no dedicated test is scoped.
