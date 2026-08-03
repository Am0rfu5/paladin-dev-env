# ADR-0008: Workspace version is 0.7.0

## Status

Accepted

**Date:** 2026-08-03

## Context

Version metadata disagreed three ways at the moment this phase began: branch `release/v0.7.0`
declares its own intended target in its name; the root `Cargo.toml` and all eleven member-crate
manifests were pinned at `0.6.0`; and the latest git tag was `v0.5.1` — `v0.6.0` was never tagged
at all. `CHANGELOG.md`'s `## [0.6.0]` heading was, before this phase, the only version heading in
the file carrying no date. None of the three surfaces (branch name, manifests, tag) agreed with
either of the other two, and REL-01 (Phase 4) names this exact three-way disagreement as its
"Current state."

**HARD-03** (Phase 10) records the version trajectory as history: all ten crates first published
at `0.1.0`, tagged `v0.1.0-rc.1` at commit `a9530fc` on 2026-05-28, and explicitly forbids REL-01
from converging on an `rc.1` figure ever again. **ORCH-05** (Phase 13) completes the line from that
point forward: a lockstep-versioned chain in which each milestone's finalization epic bumped the
root crate and every workspace member together and cut a tag — `M9 → v0.3.0`, `M10 → v0.4.0`,
`M11 → v0.5.0`, `M12 → v0.6.0` — terminating exactly where the tree sat when this phase began.

The requirements corpus's own **cross-phase coupling table** (`REQUIREMENTS.md`, "Cross-phase
couplings") assigns "whether Milestone 6's facade re-export removal forces a major version bump"
to **ARCH-04 (Phase 7)**, with REL-01 (this phase) as the applying requirement. Phase 4 runs before
Phase 7 in the roadmap. REL-02's own requirement text already states the applicable convention for
exactly this situation: "Whichever of Phase 4 / Phase 7 executes first records the answer, the
other applies it." Phase 4 runs first here, so Phase 4 records the version answer rather than
waiting nine phases for Phase 7 to decide it — the same convention ADR-0009 uses for the edition
question in the other direction.

## Decision

**The workspace version is `0.7.0`.**

Milestone 6's facade cleanup **was** a breaking change — `src/application/use_cases/` no longer
exists anywhere in a tree that has been publishing crates since `v0.1.0-rc.1` — but it already
shipped inside the pre-1.0 (`0.x`) series before this phase ran. Under SemVer, a project that has
not yet reached `1.0.0` expresses breaking changes as **minor** version bumps, not major ones: the
leading `0` is the signal to consumers that the public API has not yet stabilized, and every minor
bump within `0.x` is permitted to break compatibility. `0.7.0` is therefore the correct next
lockstep figure — the branch's own declared name, the next minor after `0.6.0` in the ORCH-05
chain, and explicitly not an `rc.1` figure, honoring HARD-03's prohibition.

A `1.0.0` bump was rejected because it would assert an API-stability guarantee this corpus has not
established — this project is still resolving port ownership (ARCH-03's four competing-variant
pairs) and has not committed to a stable public surface. Declaring `1.0.0` now would make every
future breaking change a major bump in a codebase still actively reconciling its own architecture.

Twelve manifests (root `Cargo.toml` plus eleven member crates, including `crates/doc-examples`)
and every internal `[workspace.dependencies]` pin — plus the one exact `=0.7.0` pin in
`crates/paladin-ports/Cargo.toml` — now agree on `0.7.0`. `CHANGELOG.md` carries a dated
`## [0.7.0] - 2026-08-03` heading holding the former `## [Unreleased]` content, and the previously
undated `## [0.6.0]` heading now carries its derived date, `2026-06-10`, sourced from
`git log -S'## [0.6.0]' -- CHANGELOG.md` → commit `67b6207`.

**The human user confirmed `0.7.0` on 2026-08-03**, selecting local-only execution scope: bump
manifests, finalize `CHANGELOG.md`, and stop short of pushing the branch or the tag. This ADR
records the confirmed answer; it does not itself authorize the push/publish sequence, which
remains a documented, unexecuted human gate (see plan 04-05's measurement record).

## Considered Options

- **`0.7.0`** (chosen) — the branch's own declared intent, the next lockstep minor in the ORCH-05
  chain, and not an `rc.1` figure. Consistent with SemVer's pre-1.0 convention that breaking
  changes are expressed as minor bumps.
- **`1.0.0`** — rejected. Asserts an API-stability guarantee the corpus does not support: ARCH-03's
  four competing-variant pairs (edition, dependency allowlist, port value-type ownership, LLM
  config bridge location) are still open, and declaring `1.0.0` would make every future breaking
  change in an unsettled architecture a major bump.
- **Block on Phase 7 deciding first** — rejected. A nine-phase-deep circular wait: ARCH-04 (Phase 7)
  would decide the major-bump question that REL-01 (Phase 4, which runs first) needs answered now.
  REL-02's own text already authorizes whichever phase runs first to record the answer for exactly
  this reason.

## Code Locations

- `Cargo.toml` (root) and all eleven `crates/*/Cargo.toml` manifests, including
  `crates/doc-examples/Cargo.toml` — twelve `[package] version = "0.7.0"` fields, plus ten
  `[workspace.dependencies]` internal pins and the one exact `=0.7.0` pin in
  `crates/paladin-ports/Cargo.toml`, all converged by plan 04-05's `cargo release version 0.7.0`
  invocation.
- `CHANGELOG.md` — the dated `## [0.7.0] - 2026-08-03` heading (former `## [Unreleased]` content)
  and the retroactively dated `## [0.6.0] - 2026-06-10` heading.
- `.planning/phases/04-release-coherence/04-release-measurement.md`, § "Entry measurement — version
  convergence to 0.7.0" and § "Entry measurement — CHANGELOG finalize, tag deferral, and the human
  release gate" — the plan 04-05 measurement sections that recorded the bump with full D-17
  provenance, including the checkpoint resolution, the exact `cargo release` command run, and the
  documented (unexecuted) push/publish sequence.

## Code Conformance

must change

**REL-01** is the executing requirement, and it has already executed this decision within this
phase: all twelve manifests, every internal pin, and `CHANGELOG.md` were bumped to `0.7.0` by plan
04-05, proven by `cargo build --workspace --offline` exiting 0 against the new pins. The verdict is
therefore `satisfied` at the time of writing, not pending — this ADR records the reasoning behind
an already-executed code change, the same relationship ADR-0009 has to REL-02.

## Downstream Consumers

- **Phase 7's ARCH-04** — the Milestone 6 facade re-export policy and its major-version-bump
  question. ARCH-04's requirement text is amended (plan 04-07, Task 2) to cite this ADR instead of
  re-adjudicating the major-bump question; whatever else ARCH-04 owns beyond that question (the
  facade re-export policy itself) is untouched by this ADR.
- **Phase 10's HARD-03** — the version-trajectory history requirement. HARD-03 already forbids
  converging on an `rc.1` figure; this ADR's `0.7.0` answer is consistent with that constraint and
  HARD-03 inherits it as the trajectory's next confirmed point rather than re-deciding it.
