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

**Status of this document: complete.** Task 1 (authorisation) and Task 2 (the
human-driven rehearsal) are both recorded below. Task 2 ran **twice**: once as
`v0.8.1-rc.3` against the pre-Phase-20 pipeline (Phase 20 was not yet merged to
`main` when the tag was cut), and once as `v0.8.1-rc.4` after Phase 20's own
machinery — the pre-publish consistency gate, `create-or-reuse-release.sh`, and
the rewritten `publish-crates.sh` outcome-table loop — was live. Both rehearsals
are recorded in full below because each proves something the other does not: the
`rc.3` rehearsal is the complete three-moment registry snapshot the plan's
must-haves ask for; the `rc.4` rehearsal is the first live exercise of Phase 20's
own gate and recovery scripts, including two gate bugs the rehearsal itself
found and fixed live (Findings 5 and 6, below).

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

## Task 2 — the rehearsal, executed twice

Precondition met before either run: Phase 20's own branch (`chore/19-trusted-publishing`
carrying the full Phase 20 history) merged to `main` via **PR #43**, so `make
release` and the `crates-io` environment's `v*.*.*`-tag deployment policy both
had an eligible `main` history to cut a tag from.

### Rehearsal 1 — `v0.8.1-rc.3` (pre-Phase-20 pipeline)

**Date:** 2026-08-28. Ran against the pipeline as it stood **before** Phase 20's
branch was merged — the pre-publish consistency gate did not exist yet, and the
publish loop was the old grep/sleep-tolerance version PUBOPS-03/04 describe as
broken. This rehearsal exists to prove the *documented recovery procedure itself*
(re-running the same tag's workflow run) against a real induced partial failure,
independent of whichever loop implementation happens to be live.

**Attempt 1 — refused before anything published.** The rehearsal's tag was
pushed before the release commit had landed on `main`. Run
[33210072054](https://github.com/DF3NDR/paladin-dev-env/actions/runs/33210072054),
attempt 1: `verify-tag-source` **FAILED** — correct refusal, since the tagged
commit was not yet an ancestor of `main`. Every downstream job was skipped;
nothing was published. This is the documented `verify-tag-source` gate working
exactly as designed on a premature tag.

**Attempt 2 — induced interruption, after the merge.** After **PR #42** merged
the release commit to `main` (merge commit), the *same tag*'s workflow run was
re-run ("Re-run failed jobs") against the pre-Phase-20 pipeline still in that
tree at the time (the gate job did not exist in this attempt's job graph; the
publish loop was the old grep/sleep version). The publish loop ran and the
operator **cancelled the run mid-loop, after 4 crates**.

Mid-interruption registry snapshot (captured live, 2026-08-28), `v0.8.1-rc.3`:

| Crate | HTTP status |
|---|---|
| `paladin-ai-core` | 200 |
| `paladin-ports` | 200 |
| `paladin-herald` | 200 |
| `paladin-battalion` | 200 |
| `paladin-llm` | 404 |
| `paladin-memory` | 404 |
| `paladin-web` | 404 |
| `paladin-notifications` | 404 |
| `paladin-content` | 404 |
| `paladin-storage` | 404 |
| `paladin-ai` | 404 |

Four of eleven crates at 200, seven at 404 — a genuine split, not a simulated
one. Crate in flight at the moment of cancellation: **after `paladin-battalion`,
before `paladin-llm`** (matching the committed publish order). This snapshot's
**before** counterpart is Task 1's own pre-collision check above (all eleven
404 at `0.8.1-rc.3`, 2026-08-28T17:22:40Z) — registry state can only move
404→200, never back, so that earlier check is the valid "before the tag was
pushed" moment for this same version.

**Attempt 3 — recovery, same tag, "Re-run failed jobs".** In the same run
(33210072054), "Re-run failed jobs" was used — no re-tag, no new dispatch. The
`Publish to crates.io` job reported **success**. The old loop's grep-based
tolerance fired exactly as designed on the crate already landed in attempt 2:
`##[warning]paladin-battalion version already published — continuing.` The
remaining seven crates published in this attempt (log shows `paladin-llm`,
`paladin-memory`, `paladin-web`, `paladin-notifications`, `paladin-content`,
`paladin-storage`, `paladin-ai` each logging `published.`).

After-state registry snapshot (2026-08-28), `v0.8.1-rc.3`: all eleven crates
HTTP 200.

| Crate | HTTP status |
|---|---|
| `paladin-ai-core` | 200 |
| `paladin-ports` | 200 |
| `paladin-herald` | 200 |
| `paladin-battalion` | 200 |
| `paladin-llm` | 200 |
| `paladin-memory` | 200 |
| `paladin-web` | 200 |
| `paladin-notifications` | 200 |
| `paladin-content` | 200 |
| `paladin-storage` | 200 |
| `paladin-ai` | 200 |

**`create-release` was not re-executed on the recovery attempt.** It had
already succeeded in attempt 2 (the cancelled attempt); "Re-run failed jobs"
reused its outputs rather than re-running it. This matters concretely: at the
time of this rehearsal `release.yml` still used the archived, non-idempotent
`actions/create-release@v1`, which 422s on a duplicate release — "Re-run ALL
jobs" (rather than "Re-run failed jobs") would have hit that 422 had it been
used instead. The rehearsal used the correct, documented recovery shape and did
not exercise the failure mode the wrong shape would have hit.

**Assumption A3 — PROVEN.** The OIDC token exchange succeeded on a re-run of the
same tag-push run. Verified independently via the crates.io API (not taken from
the workflow's self-report): `trustpub_data` for the crates recovered in attempt
3 records `provider: github, run_id: 33210072054` for both `paladin-llm` and
`paladin-ai` — the run ID matches the tag-push run whose third attempt actually
performed the publish, proving the `crates-io` environment's deployment policy
and the OIDC subject claim both still resolved correctly against the *same* tag
ref on a re-run, exactly as the runbook's §3 claims.

**Overall run conclusion:** `failure` — sourced entirely from the four
pre-existing Build Binaries matrix failures (Phase 21 defect, WR-05: not
test-gated, does not gate `publish-crates`). Every job on the actual publish
path (`verify-tag-source` attempt 3, `test`, `create-release` reuse,
`publish-crates`) succeeded. Judging this run by its aggregate red/green would
misread an unrelated, already-tracked defect as a Trusted Publishing failure —
the same trap `19-PUBLISH-EVIDENCE.md` names for the bootstrap run.

**Run URL:** https://github.com/DF3NDR/paladin-dev-env/actions/runs/33210072054
(all three attempts live under this one run ID — GitHub's "re-run" mechanics
attach subsequent attempts to the original run rather than minting a new run
URL; see the note under "Two run URLs, read literally" below for how this
squares with the plan's original two-run-URL framing.)

### Rehearsal 2 — `v0.8.1-rc.4` (Phase 20's own machinery, live)

**Dates:** rc.4 events 2026-08-29 through 2026-08-30.

**Preconditions for this rehearsal:** Phase 20 merged to `main` via **PR #43**
(merge `cb5164c0`). `make release VERSION=0.8.1-rc.4` on that `main` ran the
**new** `release` target for the first time against a real cut: lockstep
version bump, OpenAPI baseline regeneration (Finding 1's fix — first live use),
and `finalize-crate-changelogs` (all ten crate changelogs plus the root gained
a `## [0.8.1-rc.4]` section — first real run of that step). The local
`make check-release-consistency` self-check passed before anything was pushed.
As expected under the repository's PR-only `main` ruleset (Finding 2, below),
the direct push to `main` was rejected; the release commit `4d30bc57` travelled
to `main` via **PR #44** (merge commit), and the tag was pushed separately
after that merge — the same PR-decomposed shape `19-PUBLISH-EVIDENCE.md`
recorded as necessary for the `0.8.1-rc.1`/`rc.2` releases.

**Gate live failure — `CI_LOOKUP_FAILED` (run 33275930028).** The pre-publish
consistency gate, live for the first time against a real tag, failed with
`gh: Not Found (HTTP 404)` on its CI-conclusion lookup. Two stacked bugs, both
found and fixed live as part of this rehearsal:

- **Finding 5:** the gate job's `permissions:` block lacked `actions: read`.
  GitHub's API returns a 404 rather than a 403 for a missing scope on this
  endpoint, which is indistinguishable from "no such run" without knowing the
  permissions were wrong. Fixed in **PR #45** (merge `bf64a130`).
- **Finding 6:** `gh api` defaults to a `POST` request whenever `-f` fields are
  present on the command line; the workflow-runs endpoint answers a `POST`
  with 404 regardless of whether the run exists, so both the filename-based
  lookup path and the numeric-run-ID fallback path failed identically for the
  same underlying reason. Fixed by forcing `--method GET` on both call sites in
  **PR #46** (merge `22eda9d7`), verified live against a real run before
  merging.

The unpublished-tag exception the runbook does not otherwise grant was invoked
twice here, deliberately and within the bounds §4 of the runbook states:
nothing had reached crates.io on either occasion (the gate blocked before the
first `cargo publish` both times), so moving the tag to re-cut after each fix
was the documented-safe case, not an exception to it.

**Final run — 33322587044.**

- **Attempt 1 — refused, `CI_MISMATCH`.** The tag was pushed before the tagged
  commit's `ci.yml` run had completed. The gate correctly refused with
  `CI_MISMATCH` rather than a false pass. Recovered with **no re-tag** — the
  operator simply waited for CI to record success on that same commit and
  re-ran the gate. This is the runbook's `CI_MISMATCH` remedy #1 exercised
  live: re-run rather than re-tag.
- **Attempt 2 — gate passed, publish loop interrupted.** Gate output:
  `"✅ OK: 11 publishable package(s) checked, all match tag version
  '0.8.1-rc.4' with a changelog section for it."` The publish loop then ran and
  the operator cancelled it mid-loop. Outcome table (from
  `$GITHUB_STEP_SUMMARY`):

  | Crate | Outcome |
  |---|---|
  | `paladin-ai-core` | published-now |
  | `paladin-ports` | published-now |
  | `paladin-herald` | published-now |
  | `paladin-battalion` | published-now |
  | `paladin-llm` | published-now |
  | `paladin-memory` | **failed** (in flight at cancellation) |
  | `paladin-web` | skipped |
  | `paladin-notifications` | skipped |
  | `paladin-content` | skipped |
  | `paladin-storage` | skipped |
  | `paladin-ai` | skipped |

  The loop recorded the interruption honestly per crate — `failed` for the
  crate genuinely in flight when cancelled, `skipped` (not silently omitted)
  for everything dependency-ordered after it, rather than reporting a blanket
  job failure with no per-crate detail.

- **Attempt 3 — recovery, "Re-run failed jobs".** Gate re-passed. This attempt
  is where `create-or-reuse-release.sh` (PUBOPS-03's `create-release@v1`
  replacement) was exercised live for the first time on a genuine reuse case: a
  release object (`379337092`) already existed from attempt 2, the script took
  the **reuse path**, the job succeeded, and it emitted the same `upload_url`
  the create path would have — the exact scenario where the archived
  `create-release@v1` action would have 422'd (as it did for real in
  `19-PUBLISH-EVIDENCE.md`'s `rc.1` bootstrap, and as the `rc.3` rehearsal above
  notes it would have hit had "Re-run ALL jobs" been used instead of "Re-run
  failed jobs"). Publish-loop outcome table:

  | Crate | Outcome |
  |---|---|
  | `paladin-ai-core` | already-at-this-version |
  | `paladin-ports` | already-at-this-version |
  | `paladin-herald` | already-at-this-version |
  | `paladin-battalion` | already-at-this-version |
  | `paladin-llm` | already-at-this-version |
  | `paladin-memory` | published-now |
  | `paladin-web` | published-now |
  | `paladin-notifications` | published-now |
  | `paladin-content` | published-now |
  | `paladin-storage` | published-now |
  | `paladin-ai` | published-now |

  Five `already-at-this-version`, six `published-now` — both counts non-zero,
  which is exactly PUBOPS-04's mixed-split criterion and this plan's own
  acceptance criterion ("a non-zero count of `already-at-this-version` and a
  non-zero count of `published-now`"). A table that was entirely
  `published-now` would mean the interruption never actually landed anything;
  this one proves it did.

**After-state registry snapshot** (2026-08-30, live API), `v0.8.1-rc.4`: all
eleven crates HTTP 200.

**OIDC provenance on the recovery run**, independently re-queried via the
crates.io API: `trustpub_data` for `paladin-battalion`, `paladin-memory` and
`paladin-ai` all record `run_id: 33322587044` — again confirming the OIDC
exchange succeeded on the same-tag re-run, this time under Phase 20's own gate
and Trust Publisher configuration rather than the pre-Phase-20 pipeline.

**Overall run conclusion:** `failure` — again sourced solely from the four
Build Binaries legs (Phase 21, not on the publish path).

**Run URL:** https://github.com/DF3NDR/paladin-dev-env/actions/runs/33322587044

**Before-state, stated honestly.** Unlike the `rc.3` rehearsal, no standalone
all-404 registry snapshot was captured for `rc.4` immediately before the tag
was pushed. The before-state for `rc.4` is evidenced instead by the publish
loop's own per-crate registry pre-check: a `published-now` outcome in attempt
2's table is only possible if that crate's pre-check returned 404 immediately
before the loop attempted it, since `publish-crates.sh` decides
already-published from a live registry query, not a cached assumption. Six
crates read `published-now` across the two attempts (five in attempt 2, plus
the loop's own pre-checks for the crates recorded `already-at-this-version` in
attempt 3, which by definition were 404 before attempt 2's publish), which
positively demonstrates a pre-publish 404 state for those crates without a
separately saved snapshot. This is stated plainly rather than claiming a
snapshot that was not taken — the `rc.3` rehearsal's Task-1-derived snapshot
above is the document's authoritative three-moment record; this one is
corroborating.

### Two run URLs, read literally

The plan's Task 2 instructions ask for "two run URLs... the interrupted run and
the recovery run." In practice, GitHub's "Re-run failed jobs" mechanic attaches
each subsequent attempt to the **same** run ID rather than minting a new one —
so both rehearsals literally produced one run URL apiece (33210072054 for
`rc.3`, 33322587044 for `rc.4`), each carrying multiple numbered attempts
(`gh run view <id> --attempt <n>`). This is recorded here as observed fact
rather than silently reshaped to match the plan's original framing: the
runbook's §3 documents exactly this same-run-ID behaviour ("re-running the same
tag's existing workflow run"), and both rehearsals confirm the runbook is
describing the actual GitHub mechanic, not an approximation of it.

### `cargo publish --dry-run`

**Not used anywhere in either rehearsal, at any step, as evidence of anything.**
Stated plainly per the plan's prohibition: a dry run never reaches the publish
endpoint, so it can neither create the half-published state this rehearsal
needed nor prove recovery from one. Every registry-state claim in this document
— all six per-crate tables above — comes from either a live `curl` query
against `https://crates.io/api/v1/crates/<name>/<version>` or the workflow's own
`$GITHUB_STEP_SUMMARY` outcome table, both against real, non-dry-run publishes.

### Rate limiting and index-visibility timing

No `429` rate-limit response was reported by the operator, on any crate, in
either rehearsal — stated here as "not observed" rather than "ruled out": the
operator's rehearsal record does not mention encountering one, which is the
only evidence available on Assumption A1's sibling concern for this document.
Likewise, the operator's rehearsal record does not include per-crate
publish-to-index-visible timing measurements against `publish-crates.sh`'s
180-second poll bound (unlike `19-PUBLISH-EVIDENCE.md`'s explicit
auth-to-last-publish span for the OIDC proof run) — this document does not
invent a timing figure that was not measured. Both gaps are carried forward
into Assumptions and limits below rather than papered over.

## Findings ledger

Six findings surfaced across both rehearsals; four were fixed before this
document was written, two are accepted by design.

| # | Finding | Fix |
|---|---|---|
| 1 | `make release` did not regenerate the version-embedding OpenAPI baseline, tripping the pre-push drift guard. | Fixed in `Makefile` (commit `67b61dc0`, merged in PR #43); proven live in the `rc.4` cut. |
| 2 | `make release` assumes a direct push to `main`; the repository ruleset requires a PR plus all required checks, so the release commit must travel via PR (merge commit only — a squash or rebase merge would orphan the tag) with the tag pushed separately after merge. | Not fixed in tooling this phase, by design — documented as procedure instead (runbook update, below). |
| 3 | A tag pushed before its commit lands on `main` is refused by `verify-tag-source` (`rc.3`, attempt 1) and by the gate's `CI_MISMATCH` check when the commit is on `main` but CI has not yet completed (`rc.4`, attempt 1). Both recover via a re-run of the same tag, with no re-tag needed. | Working as designed; documented as a runbook lesson (below). |
| 4 | Ambient `GITHUB_ACTIONS=true` leaked into the gate's local test harness in CI, tripping fail-closed `MISSING_SHA` in every fixture case. | Fixed (commit `616160c0`, PR #43). |
| 5 | The gate job's `permissions:` block lacked `actions: read`; GitHub answers the missing scope as a 404 rather than a 403, masking the real cause as `CI_LOOKUP_FAILED`. | Fixed (PR #45). |
| 6 | `gh api` defaults to `POST` whenever `-f` fields are present; the workflow-runs endpoint answers `POST` with 404 regardless of whether the run exists, breaking both the filename-based and numeric-ID lookup paths identically. | Fixed with `--method GET` on both call sites (PR #46), verified live before merging. |

Positive observations, not findings: the gate refused a not-yet-CI-verified
commit (`CI_MISMATCH`, `rc.4` attempt 1) rather than assuming success, and its
two refusal codes were never conflated — `CI_LOOKUP_FAILED` (the check could
not be performed) stayed distinct from `CI_MISMATCH` (the check ran and found a
non-success or missing conclusion) across both rehearsals, exactly as
`release-recovery.md`'s existing §6 documentation for those two tokens
describes.

## Assumptions and limits

What this document proves, stated in one place:

- **Assumption A3 is proven, twice, independently.** A re-run of the same
  tag-push workflow run — under both the pre-Phase-20 pipeline (`rc.3`) and
  Phase 20's own gate and `create-or-reuse-release.sh` (`rc.4`) — preserves the
  tag ref the `crates-io` environment's deployment policy and the OIDC subject
  claim both require, and the token exchange succeeds. Verified independently
  against the crates.io API's `trustpub_data.run_id` field, not taken from
  either workflow run's self-report.
- **The recovery procedure completes a genuinely half-published release.** Both
  rehearsals show a real, induced partial state (four-of-eleven at 200 for
  `rc.3`; a `failed`+`skipped` outcome table for `rc.4`), followed by a
  same-tag re-run that reaches all-eleven-at-200, with the recovery run's own
  outcome table showing non-zero counts of both `already-at-this-version` and
  `published-now` on the `rc.4` rehearsal.
- **`cargo publish --dry-run` was not used anywhere in either rehearsal as
  evidence of anything**, per the plan's explicit prohibition — see above.

What this document does **not** prove:

- **`workflow_dispatch` eligibility under Trusted Publishing (Assumption A1)
  remains untested.** Both rehearsals used a tag push, exactly as D-15 and the
  runbook's §3 specify; neither incidentally exercised `workflow_dispatch`, so
  this document adds no new evidence toward or against A1.
- **Behaviour under a failure mode other than operator cancellation was not
  exercised.** Both rehearsals induced the partial-publish state by cancelling
  a live run; neither tested a genuine crates.io outage, an expired OIDC token
  mid-loop, or a misconfigured Trust Publisher Configuration.
- **Behaviour on a stable (non-prerelease) version was not exercised**, by
  design — both rehearsal versions (`0.8.1-rc.3`, `0.8.1-rc.4`) are prereleases
  chosen specifically because Phase 19's D-04 established that prerelease
  versions never win default dependency resolution, keeping the one-way blast
  radius small.
- **Per-crate publish-to-index-visible timing against the 180-second poll
  bound was not measured or recorded** by the operator in either rehearsal;
  this document does not manufacture a figure that was not observed. A future
  reader judging whether the bound is still right should treat this as an open
  gap, not a quiet pass.
- **No `429` rate-limit response was reported**, but absence-of-report across
  two rehearsals covering, at most, twenty-two individual crate publishes is
  thin evidence for research Assumption A1's sibling concern about registry
  rate limiting under load — recorded as "not observed" rather than "ruled
  out."
- **`rc.4`'s before-state rests on the publish loop's own pre-check evidence,
  not a standalone snapshot** — see "Before-state, stated honestly" above. The
  document's authoritative three-moment, eleven-row-per-moment record is the
  `rc.3` rehearsal, which does have a standalone before/mid/after snapshot at
  every moment.
