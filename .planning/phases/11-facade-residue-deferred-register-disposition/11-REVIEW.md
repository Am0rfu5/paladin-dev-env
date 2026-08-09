---
phase: 11-facade-residue-deferred-register-disposition
reviewed: 2026-08-09T00:00:00Z
depth: standard
files_reviewed: 2
files_reviewed_list:
  - .project/Milestone_8-Facade-Cleanup-Shim-Resolution/deferred-features.md
  - .project/Milestone_8-Facade-Cleanup-Shim-Resolution/deferred-items.md
findings:
  critical: 0
  warning: 1
  info: 0
  total: 1
status: issues_found
---

# Phase 11: Code Review Report

**Reviewed:** 2026-08-09T00:00:00Z
**Depth:** standard
**Files Reviewed:** 2
**Status:** issues_found

## Summary

Phase 11 changed zero `.rs` files — confirmed via `git diff --name-only b417326..HEAD`, which
touches only the two prose planning records in scope plus `.planning/` artifacts excluded from
this review. Both files received purely additive, dated correction banners; no original text was
deleted (verified via `git diff b417326..HEAD` on both files — every hunk is a pure insertion,
and the one place original text is retracted, D5's "Recommendation"/"Effort-risk" lines and the
"Quick wins" grouping line in `deferred-items.md`, it is wrapped in `~~strikethrough~~` and kept
inline rather than removed, matching the documents' own "nothing deleted" claim).

I cross-checked essentially every falsifiable factual claim added by the new banners against the
live repository: the `println!`/`eprintln!`/`dbg!` count and rustdoc-only characterization (17
across 6 files, 0 outside doc comments), all four `.planning/decisions/0034...`-cited file line
counts (content_service.rs 385, event_manager.rs 345, user_service.rs 583, the four D3 files
totaling 2,749, content_ingestion_service.rs 1,211), the `crate::core::` importer count (49), the
existence of every linked register/ADR (`facade-03-removed-features.md`, ADR-0035, ADR-0034,
`facade-01-rustdoc-stdout-disposition.md`, ADR-0018, ADR-0031), the existence of commits
`6704807` and `66f6c4e` and their cited commit-message content, and the `PROJECT.md` "Out of
Scope" references. All of these check out exactly as claimed.

One claim does not check out: `deferred-features.md`'s new banner gets a git ancestry direction
backwards when describing the relationship between the `chore/facade-cleanup-m8-finish`
remote-tracking ref and the removal commit `3d48768`. See WR-01.

## Warnings

### WR-01: Git ancestor-direction claim is backwards

**File:** `.project/Milestone_8-Facade-Cleanup-Shim-Resolution/deferred-features.md:16-19`

**Issue:** The new correction banner states:

> Measured this session: no *local* branch named `chore/facade-cleanup-m8-finish` exists in this
> checkout, while a *remote-tracking* ref of that name does resolve
> (`refs/remotes/origin/chore/facade-cleanup-m8-finish`) and **is an ancestor of the removal
> commit**.

This asserts `refs/remotes/origin/chore/facade-cleanup-m8-finish` is an *ancestor of* `3d48768`.
Verified against the actual repository:

```
$ git merge-base --is-ancestor refs/remotes/origin/chore/facade-cleanup-m8-finish 3d48768; echo $?
1   # false — the branch ref is NOT an ancestor of 3d48768

$ git merge-base --is-ancestor 3d48768 refs/remotes/origin/chore/facade-cleanup-m8-finish; echo $?
0   # true — 3d48768 IS an ancestor of the branch ref
```

The relationship is the reverse of what's stated: `3d48768` (2026-06-04) is an ancestor of the
branch tip `4bf6745` (2026-06-05, 4 commits later) — the branch *contains* the removal commit and
extends past it, rather than terminating at or before it. The banner's own local-vs-remote
distinction (the specific, checkable claim the phase constraint file flagged as worth verifying)
is correct; only the ancestry direction is wrong.

This does not undermine the banner's actual conclusion — "a branch ref is mutable and deletable
regardless of whether it happens to resolve today, so cite the immutable SHA instead" holds
independent of which commit is upstream of which. But the banner presents the ancestry claim as
a specifically re-measured fact ("Measured this session"), and as written it is not accurate, in
a document whose sole purpose this phase is to make more accurate.

**Fix:** Correct the clause to state the relationship in the right direction, e.g.:

```markdown
while a *remote-tracking* ref of that name does resolve
(`refs/remotes/origin/chore/facade-cleanup-m8-finish`), and the removal commit is an ancestor of
that ref (the branch's history runs through and past `3d48768`, 4 commits later).
```

Per the phase's additive-only discipline, this should be added as a further dated correction
rather than editing the existing banner text in place.

---

_Reviewed: 2026-08-09T00:00:00Z_
_Reviewer: Claude (gsd-code-reviewer)_
_Depth: standard_
