# Phase 9: Release & Security Gate Integrity - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.
> Decisions are captured in CONTEXT.md — this log preserves the alternatives considered.

**Date:** 2026-08-07
**Phase:** 9-release-security-gate-integrity
**Mode:** `--auto` — all nine gray areas auto-selected; every answer is Claude's recommended option.
**No human confirmed any of these.** Four are flagged ⚠ HUMAN REVIEW in CONTEXT.md.
**Areas discussed:** Authoritative advisory surface · Governance metadata home · Dead suppressions ·
Phase 12 scope overlap · The 2026-09-30 disposition · Licence posture · Name-collision guard ·
Herald changelog · Dockerfile.chef staleness mechanism

---

## Authoritative advisory surface

| Option | Description | Selected |
|--------|-------------|----------|
| `.cargo/audit.toml` authoritative, `deny.toml` mirrors + one labelled second class | Matches what `ci.yml:75-77` and `Makefile:246` already assert; `cargo audit` cannot consume `deny.toml`'s superset | ✓ |
| `deny.toml` authoritative | Holds the superset, but `cargo audit` cannot read it — the primary gate would follow the secondary | |
| Generate both from one YAML source | Structurally clean; a build step nobody maintains, for two files totalling ~40 live lines | |

**Choice:** `.cargo/audit.toml` authoritative (D-01).
**Notes:** The invariant is already asserted in three places and has never been enforced — it is
currently satisfied by accident. D-02 converts it into `scripts/check-advisory-register.sh`.
Run 5's finding that the sync invariant *does* hold was honoured; run 4's contrary finding stays
withdrawn (D-03).

---

## Governance metadata home

| Option | Description | Selected |
|--------|-------------|----------|
| New root `SECURITY-EXCEPTIONS.md` register | Adjacent to the configs it governs; visible to anyone editing `deny.toml`; machine-checkable | ✓ |
| Richer TOML comments | `[advisories] ignore` is an array of strings — the four required fields can only ever be comments, which are neither queryable nor gateable | |
| A file under `.planning/` | Invisible to anyone not running GSD, and to every consumer of a published crate family | |

**Choice:** root-level register (D-01), schema = FR-3's four fields **plus** owner and expiry (D-06).
**Notes:** Extending M10 Epic 2 FR-3's schema is a PRD supersession under the project's precedence
order, so it is recorded in ADR-0024 rather than treated as a defect fix.

---

## Dead suppressions after Phase 8

| Option | Description | Selected |
|--------|-------------|----------|
| Delete the four orphaned entries | `structopt`, `ansi_term`, `atty`, `proc-macro-error` all return 0 in `Cargo.lock` after ADR-0023's clap v4 migration | ✓ |
| Backfill owner/expiry onto all fifteen | Attaches governance to four suppressions that suppress nothing | |
| Keep them "just in case" | D-02's staleness check makes the state unreachable going forward, so keeping them is inconsistent with the guard landing in the same phase | |

**Choice:** delete (D-04), and correct the corpus's stale arithmetic at source (D-05).
**Notes:** Fresh finding, in no ingest document. `deny.toml` holds **fourteen** entries and **nine**
unmaintained, not fifteen and ten — `RUSTSEC-2025-0121` (gcc) is already gone with no record of its
removal. After D-04, the live backfill surface is **ten** entries, not fifteen.

---

## Phase 12 scope overlap (SUPPLY-01 / SUPPLY-02)

| Option | Description | Selected |
|--------|-------------|----------|
| Phase 9 deletes `ci.yml:465-482` and marks SUPPLY-01/02 closed | ROADMAP criterion 3 cannot be met otherwise; Phase 12 inherits closure notes with a commit SHA | ✓ |
| Leave the deletion to Phase 12 | Leaves Phase 9 carrying a success criterion it cannot satisfy | |
| Run Phase 12 first | The roadmap's own preference, but Phase 9 is the phase that is running, and it carries the dated item | |

**Choice:** absorb into Phase 9 (D-07). ⚠ HUMAN REVIEW — changes another phase's scope.
**Notes:** REQUIREMENTS.md warns "Do not plan the same fix twice" while ROADMAP criterion 3 requires
the fix — the two cannot both hold. Recorded line-number drift: the job is `ci.yml:465-482`, not the
`:389-406` the record cites, and **both** audit jobs render as "Security Audit" in the GitHub UI.

---

## The 2026-09-30 disposition

| Option | Description | Selected |
|--------|-------------|----------|
| Renew with per-advisory review dates, all initially 2026-12-31 | ~5 months out, inside the next two quarters; never lets an acceptance run a full year | ✓ |
| Close it | Requires an upstream fix; verified none exists for either original advisory | |
| One blanket renewal date | Reproduces the current state, where one date governs two advisories and nine others carry none | |

**Choice:** per-advisory renewal to 2026-12-31, owner reassigned from "Platform Security
(Milestone 7)" to the repository owner `DF3NDR` (D-09, D-10). ⚠ HUMAN REVIEW on the owner change.
**Notes:** This is the only dated item in the 199-document corpus, and it lapses in eight weeks.

---

## Licence posture

| Option | Description | Selected |
|--------|-------------|----------|
| `MIT OR Apache-2.0` — change eleven manifests to match the signed checklist | Preserves the corpus's only signed governance artefact and the 551-package sign-off's own dual-licence rule; adding a licence alternative is a grant, not a revocation | ✓ |
| `MIT` — annotate the checklist superseded | Cheaper in diff; forces re-justifying a completed 551-package review, and weakens the stated basis for accepting `r-efi 5.3.0` | |

**Choice:** `MIT OR Apache-2.0` (D-11). ⚠ **HUMAN REVIEW — blocking.**
**Notes:** SEC-02 states in terms that this "must not be resolved by inference", and the crates are
already published at 0.1.0 under MIT. Rated `one-way`. D-12 records that the MIT-only branch produces
the same deliverable shape with no files moved, so **a plan must not be written that assumes the
answer**. Cost budgeted: `LICENSE` → `LICENSE-MIT`, add `LICENSE-APACHE`, update `README.md` and
`CHANGELOG.md`. `deny.toml`'s allow-list already permits both and needs no change either way.

---

## crates.io name-collision guard

| Option | Description | Selected |
|--------|-------------|----------|
| Offline allow-list of the eleven owned names, checked in CI | Provable in this environment, never flaky, catches the exact failure Epic 4 hit (a *new* name colliding) | ✓ |
| Live crates.io sparse-index query | Could be written but never demonstrated here — `crates.io` returns HTTP 403 (the Phase 8 D-03 failure mode) | |
| Accept dry-run reliance and record the cost | Permitted by SEC-03's done-when, but today's earliest guard is main-branch-only at `ci.yml:901-929` | |

**Choice:** offline allow-list guard plus ADR-0026 recording the residual cost (D-13).
**Notes:** The eleven existing names are already owned, so their collision risk is zero; the guard
exists for the eleventh crate. A genuinely novel name is still checked by a human, and that is stated
as the accepted cost rather than left implicit.

---

## Herald changelog

| Option | Description | Selected |
|--------|-------------|----------|
| Create `crates/paladin-herald/CHANGELOG.md`, backfilled from real history | Nine of ten siblings have one; herald ships on crates.io under a criterion its own summary recorded Met | ✓ |
| Record an exemption | Permitted by SEC-04, but there is no reason for it — the crate is published like the other nine | |

**Choice:** create it (D-14), plus a guard asserting every `crates/*/` carries one (D-15).
**Notes:** Content comes from the crate's real history — creation by reconciliation commit `66f6c4e`,
and Phase 8 D-14's feature-gating of `colored` / `comfy-table`, which shrank the default public API
and already owes an entry. No ADR: a plain defect fix with no competing defensible position.

---

## Dockerfile.chef staleness mechanism

| Option | Description | Selected |
|--------|-------------|----------|
| Delete the nine-manifest enumeration | `COPY crates ./crates` at `:36` already supersedes it before `chef prepare` at `:38`; deletion makes staleness structurally impossible | ✓ |
| Add the herald line only | Satisfies the letter and reproduces the defect the done-condition names explicitly | |
| Enumerate + a guard script asserting every manifest appears | Also satisfies the done-condition; kept as the **fallback** if research contradicts the cargo-chef reading | |
| Glob the manifests | Docker's `COPY` flattens paths through a glob — cannot preserve per-crate directories | |

**Choice:** delete, with the enumerate-plus-guard fallback written into D-16. ⚠ HUMAN REVIEW —
supersedes M7 Epic 2 FR-01 on a milestone recorded complete.
**Notes:** The reading is sharper than the record — the enumeration is likely decorative for *all
ten* crates, not merely incomplete for herald. **Docker is absent from this environment**, so it
cannot be measured; the plan must confirm cargo-chef's recipe-distillation semantics from upstream
docs and state which branch it took.

---

## HARD-06 coupling (raised during analysis, resolved without a question)

REQUIREMENTS.md sequences SEC-01 behind HARD-06 (Phase 10). Reading
`crates/paladin-content/Cargo.toml:41` settled it: `pdf-extract` is an **unconditional** dependency
and `pdf = []` at `:18` gates nothing, so `RUSTSEC-2026-0187`'s suppression is warranted regardless
of HARD-06's answer. SEC-01 ratifies on tree evidence and hands HARD-06 the finding (D-17). The
contradiction itself is **not** answered here.

---

## Claude's Discretion

Auto-mode resolved every gray area, so nothing was deferred to Claude by a user. CONTEXT.md's
"Claude's Discretion" section instead records the six choices deliberately left open to the planner:
the register's file name and format; whether the three guards are one script or several; where
`.crate-names.txt` lives; whether ADR-0024…0027 are authored in their own plans or fold into the
plans that execute them; the D-05 / D-11 annotation prose; and whether the register also becomes a
`SECURITY.md`.

## Deferred Ideas

Eleven items, recorded in full in CONTEXT.md `<deferred>`. The ones that surfaced *during* this
session rather than being carried forward:

- **"Is PDF extraction supported?"** — the empty `pdf` feature vs the mandatory `pdf-extract`
  dependency. Phase 10 / HARD-06.
- **A `SECURITY.md`** for GitHub's private-reporting UI — genuinely missing, adjacent to D-01's
  register, different audience. Phase 16 candidate.
- **Replacing `dotenv` with `dotenvy`** — `deny.toml`'s own comment recommends it, and it would
  retire `RUSTSEC-2021-0141` rather than suppress it. Dependency-upgrade work, not suppression
  governance.
- **The four other live unmaintained advisories' upstream paths** — `fxhash` via `scraper`,
  `number_prefix` via `indicatif`, `rustls-pemfile` via `tonic`/`testcontainers`, `paste` via
  `utoipa`. The register records each revisit condition; acting on them is a later phase.
- **Stray root artefacts** — `api_surface_current.txt`, `final-api.txt`, `flat`, `lcov.info`.
  Housekeeping, not a SEC requirement.
