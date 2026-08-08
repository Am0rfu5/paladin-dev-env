# ADR-0025: Licence posture — `MIT OR Apache-2.0`

## Status

Accepted

**Date:** 2026-08-08

## Context

Three live positions stated the project's licence differently. `Epic_4/license-compatibility-decision-checklist.md`
records the target policy as `MIT OR Apache-2.0` (Rust-style dual licensing), signed by approver
`DF3NDR` (repository owner) on 2026-05-28, backed by a 551-package transitive-dependency inventory
with zero unknown entries and an explicit acceptance of MPL-2.0 for unmodified use. The M7 Epic 4
PRD §4.7.7 and the M7 overview Acceptance Criterion 1 both instead state the project's licensing
posture as `MIT`. And the shipped root `Cargo.toml` and all ten library crate manifests, verified
directly by grep this session before any change, declared `license = "MIT"` — agreeing with the PRD
and overview, not the checklist.

The checklist's dual-licence approval rule — "any SPDX expression containing a permissive
MIT/Apache branch is acceptable by default" — was the stated basis for accepting `r-efi 5.3.0`'s
`MIT OR Apache-2.0 OR LGPL-2.1-or-later` expression during the 551-package review. Under a
single-licence (`MIT`-only) posture, that rationale is weaker than the review recorded, because the
project itself would not carry the Apache branch the rule leans on.

The enforcement surface already followed the checklist regardless of which position won:
`deny.toml`'s `[licenses] allow` list (`deny.toml:24-46`) has permitted both `MIT` and `Apache-2.0`
since Milestone 10 Epic 2, and needed no change under either branch.

SEC-02 (`.planning/REQUIREMENTS.md:1148-1164`) names this as a sign-off artefact with a named
approver and states in terms that it "must not be resolved by inference." This decision was
**not** inferred: it was put to the repository owner (`DF3NDR`) at a blocking `checkpoint:decision`
(Phase 9 Plan 05, Task 1, `gate="blocking"`) presenting both branches with their full costs, and the
repository owner selected the dual expression on **2026-08-08**.

The decision is also a one-way door. All ten library crates are published on crates.io at `0.1.0`
under the single `MIT` expression. Publishing a later version under `MIT OR Apache-2.0` grants every
existing consumer of those published crates an additional permission — the option to instead rely on
the Apache-2.0 branch, including its explicit patent grant. That grant cannot be retracted: narrowing
a published `MIT OR Apache-2.0` expression back to `MIT` in a later version would revoke a permission
consumers already hold and have potentially relied upon. The reverse direction — narrowing — is not
available as a safe option on already-published crates at any point after this decision; only the
additive direction was ever executable.

## Decision

The root package and all ten library crates (`paladin-ai`, `paladin-ai-core`, `paladin-ports`,
`paladin-battalion`, `paladin-herald`, `paladin-llm`, `paladin-memory`, `paladin-storage`,
`paladin-notifications`, `paladin-content`, `paladin-web`) declare `license = "MIT OR Apache-2.0"`.
Approver: `DF3NDR` (repository owner), at a blocking checkpoint, dated **2026-08-08**.

The direction of change taken — adding Apache-2.0 as an alternative rather than narrowing to a
single expression — was the only direction safe to execute on crates already published at `0.1.0`.
It grants every existing consumer of those published crates an additional permission it did not
previously have; it takes nothing away from anyone who already depends on the MIT-licensed
`0.1.0` releases, since the MIT grant remains fully intact as one of the two alternatives. Had the
review instead landed on confirming the single `MIT` expression, no change would have been made to
any manifest and nothing would have become irrevocable — that branch was available and is recorded
fully below, but it was not the branch selected.

## Considered Options

- **Dual expression — `MIT OR Apache-2.0`, matching the signed checklist (accepted).** Preserves the
  only completed compliance review in the corpus and the 551-package sign-off that rests on it;
  keeps the dual-licence approval rule's rationale intact rather than leaving it weaker than
  recorded; adds a permission rather than removing one, the only direction safe on already-published
  crates; matches the Rust ecosystem norm and gives consumers an explicit patent grant;
  `deny.toml` already permitted it, so the enforcement surface needed no change. Cost paid: eleven
  manifests changed, the root licence file was renamed (`LICENSE` → `LICENSE-MIT`, history preserved
  via `git mv`), a second verbatim legal text (`LICENSE-APACHE`) was added, the README badge and
  License section and the `Dockerfile.chef` OCI image label all needed matching edits, and two
  source documents (the checklist, and the PRD/overview `MIT` claims) needed dated annotations. Once
  published under this expression, narrowing back is unsafe and this direction cannot be reversed.

- **Single expression — confirm the manifests' and PRD's existing `MIT` (rejected).** No file moves
  would have been required: the manifests, licence file, README, image label and `deny.toml` were
  already consistent with it, and the tree would have stayed aligned with its governing PRD. The
  cost this branch would have paid instead: it would withdraw the project's only signed governance
  artefact, requiring the checklist to be annotated superseded with its reason recorded explicitly;
  the 551-package review's acceptance of `r-efi`'s `MIT OR Apache-2.0 OR LGPL-2.1-or-later` expression
  rests on the dual-licence approval rule, and under a single-licence posture that rule's effect on
  this specific dependency would need to be re-justified standalone rather than left resting on a
  policy the project no longer follows; and consumers would gain no explicit patent grant. This
  branch remained fully executable and was written out in Task 2's action block in exactly as much
  detail as the accepted branch — it was not selected because the checklist's compliance review and
  the additive-permission argument outweighed the cost of the annotation work the rejected branch
  would still have required.

## Code Locations

- `Cargo.toml:40` — root package `license` field, now `"MIT OR Apache-2.0"`.
- `crates/paladin-core/Cargo.toml:6`, `crates/paladin-ports/Cargo.toml:8`,
  `crates/paladin-battalion/Cargo.toml:6`, `crates/paladin-herald/Cargo.toml:6`,
  `crates/paladin-llm/Cargo.toml:6`, `crates/paladin-memory/Cargo.toml:6`,
  `crates/paladin-storage/Cargo.toml:8`, `crates/paladin-notifications/Cargo.toml:8`,
  `crates/paladin-content/Cargo.toml:8`, `crates/paladin-web/Cargo.toml:8` — the ten library crate
  `license` fields, all now `"MIT OR Apache-2.0"`. No `[workspace.package]` inheritance point exists
  in this workspace; each is an independent literal.
- `LICENSE-MIT` — the former root `LICENSE` file (MIT text, copyright holder `Am0rfu5`, year 2026),
  renamed via `git mv` with history preserved (`git log --follow`).
- `LICENSE-APACHE` — new root file, the canonical Apache License, Version 2.0 text verbatim,
  including its unfilled boilerplate appendix (no project-specific text substituted into the
  bracketed fields — Apache-2.0 permits verbatim redistribution and any deviation from the canonical
  text is a legal risk with no corresponding benefit).
- `README.md:7` — the shields.io licence badge, now reading `license: MIT OR Apache-2.0` and linking
  `LICENSE-MIT`.
- `README.md:187-193` (renumbered by the edit) — the `## License` section, now naming both licences
  and the reader's choice between them, plus the standard Rust dual-licence contribution clause.
- `Dockerfile.chef:87` — `LABEL org.opencontainers.image.licenses="MIT OR Apache-2.0"`, re-derived at
  this line (not the stale `:93` `09-RESEARCH.md` recorded; Plan 09-03's nine-line deletion in wave 1
  moved it).
- `CHANGELOG.md` — a `## [Unreleased]` → `### Changed` entry recording the relicensing, the approver,
  the date, and that it is an additive grant.
- `deny.toml:24-46` — the `[licenses] allow` list, confirmed by reading to already contain both `MIT`
  and `Apache-2.0`; `git diff -- deny.toml` is empty. No change required under either branch.
- `.project/Milestone_7-Production-Hardening/Epic_4/license-compatibility-decision-checklist.md` —
  annotated in place with a dated confirmation banner, original text retained.
- `.project/Milestone_7-Production-Hardening/Epic_4/prd-api-stabilization-pre-release-preparation.md`
  §4.7.7 — annotated in place as superseded, original text retained.
- `.project/Milestone_7-Production-Hardening/overview/Milestone_7-Tier_4-Production_Hardening.md`
  Acceptance Criterion 1 — annotated in place as superseded, original text retained.

## Code Conformance

must change

Phase 9 Plan 05's Task 2 is the executor: it set the licence field in all eleven manifests, performed
the `git mv` rename, added `LICENSE-APACHE`, updated `README.md` and `Dockerfile.chef`, added the
`CHANGELOG.md` entry, and confirmed `deny.toml` required no change — verified by
`cargo metadata --offline --no-deps --format-version 1` and `cargo check --offline --workspace` both
exiting 0 against the edited manifests, and `git diff --stat -- '*.rs' | wc -l` returning `0` (no
source file was touched by this decision). Task 3 (this ADR and the document annotations) is the
executor for the documentary half.

## Downstream Consumers

- **The next real release cycle.** crates.io accepts or rejects the `MIT OR Apache-2.0` expression at
  publish time for a new version of each crate; that is out of this phase's scope and cannot be
  exercised in this environment (crates.io returned HTTP 403 to this session). No publish happens in
  Phase 9.
- **Phase 10 / HARD-01's ledger.** The licence sign-off and crate metadata rows in the Milestone 7-8
  as-shipped ledger must be updated to cite this ADR rather than the now-annotated checklist and PRD
  claims directly.
- **Any future consumer of the published crate family**, present or future, who now has the option to
  rely on either the MIT or the Apache-2.0 grant — including the explicit patent licence the
  Apache-2.0 branch carries that the prior MIT-only posture did not.
