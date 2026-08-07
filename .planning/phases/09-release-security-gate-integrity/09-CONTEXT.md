# Phase 9: Release & Security Gate Integrity - Context

**Gathered:** 2026-08-07
**Status:** Ready for planning
**Mode:** `--auto` — every gray area was auto-resolved to its recommended option. Each decision below
carries the reasoning that produced it; **none was confirmed by a human.** Four decisions are flagged
`⚠ HUMAN REVIEW` — a licence change on already-published crates, a re-scoping of Phase 12, a change
of the named security-exception owner, and the deletion of a PRD-mandated Dockerfile block. Read
those first if you read nothing else.

<domain>
## Phase Boundary

Make the security, licensing and release gates this project *believes* it has actually hold. Five
requirements, SEC-01 … SEC-05, all confirmed by direct tree inspection on 2026-07-30 and **re-read
in this session on 2026-08-07** — where the tree has moved since, this file records the newer state.

**This is a code- and config-changing phase, like Phase 8.** It touches `deny.toml`,
`.cargo/audit.toml`, `.github/workflows/ci.yml`, eleven `Cargo.toml` manifests, `Dockerfile.chef`,
`scripts/`, `LICENSE*` and one new `crates/paladin-herald/CHANGELOG.md`. It does **not** touch `.rs`
source. Every plan is subject to the CLAUDE.md workspace gate (`cargo test` → `cargo fmt --check` →
`cargo clippy -- -D warnings`) and to ADR-0006's 84% workspace line-coverage floor — though a phase
that changes no `.rs` file should not move coverage at all, and the close-out re-check should confirm
exactly that.

**Five deliverable classes:**

1. **SEC-01 — one RustSec exception register with governance, and the 2026-09-30 acceptance
   disposed of.** One authoritative register; `.cargo/audit.toml` and `deny.toml` mirroring it under
   a mechanical guard; owner, expiry, affected scope and compensating control on every live
   suppression; the second `cargo audit` CI job with inline `--ignore` flags deleted. **This carries
   the only dated item in the entire 199-document corpus** — a formal risk acceptance whose
   review/expiry target is **2026-09-30**, roughly eight weeks out.
2. **SEC-02 — one licence answer that the manifests declare.** The three-way split between a signed
   `MIT OR Apache-2.0` policy (approver `DF3NDR`, dated 2026-05-28, 551 packages reviewed), the M7
   PRD's `MIT`, and the `license = "MIT"` all eleven manifests actually carry.
3. **SEC-03 — crates.io name collisions catchable before a release cycle**, or reliance on the
   late `publish --dry-run` recorded as an accepted decision with its known cost.
4. **SEC-04 — `crates/paladin-herald/CHANGELOG.md` exists**, or its exemption is recorded. Nine of
   ten library crates have one; herald was created after the audit that marked the criterion Met.
5. **SEC-05 — `Dockerfile.chef`'s planner stage cannot silently go stale as crates are added.**
   The done-condition is explicit that an enumerated list is *the defect*, not just the one missing
   herald line.

**Not in this phase:**

- **Milestone 7-8 ground truth (Phase 10 / HARD-01 … HARD-07).** In particular the "is PDF
  extraction supported?" capability question (HARD-06), the `cargo doc --workspace --no-deps`
  warning bar (HARD-07), the version-trajectory record (HARD-03) and the leaf-crate dependency rule
  (HARD-05). **D-19 removes SEC-01's dependency on HARD-06 with tree evidence** — it does not answer
  HARD-06.
- **The facade residue and deferred registers (Phase 11).**
- **The `actions-rs/toolchain@v1` and other deprecated GitHub Action references** — Phase 15 /
  PIPE-04 owns the full sweep. Two of them sit inside jobs this phase edits. **Do not
  opportunistically bump them** (the Phase 8 D-24 rule, restated).
- **Any `.rs` source change.** If a suppression can only be dropped by changing code, the register
  records the condition and the work goes to the phase that owns that code.
- **The stray untracked-looking root artefacts** (`api_surface_current.txt`, `final-api.txt`,
  `flat`, `lcov.info`) — noticed while scouting, out of scope, see Deferred Ideas.

</domain>

<decisions>
## Implementation Decisions

### Inherited from Phases 1, 5, 7 and 8 — locked, not re-litigated

- **D-00a:** ADRs live in `.planning/decisions/`, flat sequential numbering, file shape
  `Status / Context / Decision / Considered Options / Code Locations / Code Conformance /
  Downstream Consumers`, **no frontmatter** (matching 0001-0023, not the `adr-parser.cjs` schema).
  **`PROMOTION.md` records 0024 as next free** — verified this session at `PROMOTION.md:47`.
  *(Phase 1 D-01/D-03, Phase 7 D-00a/D-00h, Phase 8 D-00a)*
- **D-00b:** Precedence order is **ADR → shipped tree → `.planning/codebase/` map →
  `intel/code-verification.md` → PRD → DOC → task-list checkbox.** An ADR that contradicts shipped
  code is an instruction to change the code. **This phase uses the PRD tier of that order twice** —
  D-16 supersedes M7 Epic 2 FR-01 and D-06 supersedes M10 Epic 2 FR-3's four-field schema — and both
  supersessions must be recorded in an ADR, not assumed. *(Phase 1 D-02)*
- **D-00c:** Source corrections under `.project/` are **annotation, not rewriting** — a dated
  correction banner naming what was wrong and pointing at the ADR or requirement, each defective
  claim corrected inline with the original text retained and marked superseded. *(Phase 5 D-08)*
- **D-00d:** Ledgers are **amended in place**, dated, superseded text retained. Never a separate
  corrections file. *(Phase 2 D-02)*
- **D-00e:** Evidence bar (the "D-19 bar"): no claim of closure without the exact command or
  `file:line` that produced it, recorded verbatim. *(Phases 3, 5, 7, 8)*
- **D-00f [informational]:** Medieval-military ubiquitous language is mandatory in code, docs and
  comments. *(CLAUDE.md — a standing project-wide convention.)*
- **D-00g:** Contested positions get ADRs; code-settled defects get ledger rows and no ADR.
  *(Phase 7 D-17, applied by Phase 8 D-22.)*

---

### SEC-01(a) — which surface is authoritative, and where governance lives

- **D-01: `.cargo/audit.toml` is the authoritative suppression surface.**
  `deny.toml` mirrors its vulnerability class exactly and adds one clearly-labelled second class; a
  new root-level `SECURITY-EXCEPTIONS.md` is the authoritative *governance* register.
  Three surfaces, one register, one direction of truth.
  Verified this session: `ci.yml:75-77` already declares `.cargo/audit.toml` "the single source of
  truth", `Makefile:246` says the same, and `deny.toml`'s own header says "Keep these two files in
  sync". The invariant is already asserted in three places — it has never been *enforced*, and it is
  currently satisfied by accident.
  Chosen over making `deny.toml` authoritative (it holds a superset that `cargo audit` cannot
  consume) and over generating both from one YAML (a build step nobody will maintain, for two files
  totalling ~40 live lines).
  **Why a separate register file rather than richer TOML comments:** the four governance fields
  criterion 2 demands — owner, expiry date, affected scope, compensating control — cannot be
  structured data inside `[advisories] ignore`, which is an array of strings. They can only ever be
  comments there. A comment is not queryable and cannot be gated. `SECURITY-EXCEPTIONS.md` gets one
  row per advisory ID with all four fields; each TOML entry keeps a one-line comment plus a pointer
  to the register.
  **Why repo root, not `.planning/`:** the register must be adjacent to the config it governs, so
  the next person editing `deny.toml` sees it. `.planning/` is invisible to anyone not running GSD,
  and to every downstream consumer of a published crate family. There is no `SECURITY.md` today, so
  the name is free.
  — **Reversibility:** reversible — a new documentation file plus comment edits; nothing published
  depends on it.

- **D-02: The sync invariant becomes a script, not a comment.**
  `scripts/check-advisory-register.sh`, wired into CI next to the existing `cargo-deny` job, asserts
  three things and exits non-zero on any failure:
  1. the vulnerability-class IDs in `deny.toml` equal `.cargo/audit.toml`'s `ignore` array exactly;
  2. every ID in either file has a row in `SECURITY-EXCEPTIONS.md` carrying all four governance
     fields, and every register row maps to a live suppression;
  3. **every suppressed advisory's named crate still appears in `Cargo.lock`** — the staleness check
     that D-04 exists because nothing performed.
  Chosen over trusting review: this repository has now shipped two governance invariants that were
  asserted in comments and enforced by nothing (`deny.toml`'s sync header, and
  `check-deprecations.sh` which Phase 8 D-05 found could not fail). The phase idiom is *mechanism,
  not assertion* — SEC-05's done-condition states it outright.
  Must be provable offline: it reads three files in the repo and touches no network.

- **D-03: The two `deny.toml` classes stay separated and stay labelled.**
  Vulnerability advisories and unmaintained/maintenance-mode notices are different objects with
  different governance. `cargo audit` does not fail on unmaintained crates; `cargo deny` does. Ten of
  the entries are explicitly sanctioned by M10 Epic 4 FR-1 step 5. Run 5 already verified the sync
  invariant holds and withdrew run 4's contrary finding — **do not re-open it.** The register carries
  a `class` column so the distinction survives outside a TOML comment header.

### SEC-01(b) — what Phase 8 did to the suppression set

- **D-04: Four `deny.toml` suppressions are now dead and get deleted, not backfilled.** ⚠ **Fresh
  finding — in no ingest document, and it changes SEC-01's size.**
  Phase 8's D-13 migrated `src/main.rs` from `structopt` to `clap` v4 and removed `structopt` from
  the manifest entirely. Verified this session against `Cargo.lock`:

  | Advisory | Crate | `grep -c '^name = "<crate>"$' Cargo.lock` |
  |---|---|---|
  | RUSTSEC-2022-0104 | `structopt` | **0** |
  | RUSTSEC-2021-0139 | `ansi_term` | **0** |
  | RUSTSEC-2024-0375 | `atty` | **0** |
  | RUSTSEC-2024-0370 | `proc-macro-error` | **0** |

  All four `deny.toml` comments justify the entry "via structopt" or "via clap 2.x/structopt". The
  parent is gone; the entries suppress nothing. Backfilling owner and expiry onto a dead suppression
  would be governance theatre.
  **Chosen over keeping them "just in case"** — D-02's clause 3 makes the dead-entry state
  unreachable going forward, so keeping them is both unnecessary and inconsistent with the guard
  landing in the same phase.
  — **Reversibility:** reversible — four lines, restorable from git if a dependency returns.

- **D-05: The "fifteen entries / ten unmaintained" figure is stale by one; correct it at source.**
  Read directly this session: `deny.toml`'s `[advisories] ignore` holds **fourteen** entries, not
  fifteen, and **nine** unmaintained, not ten. `RUSTSEC-2025-0121` (`gcc`) is absent from
  `deny.toml`, absent from `.cargo/audit.toml`, and `gcc` returns 0 hits in `Cargo.lock`. Someone
  removed it and no record caught it.
  The affected records get the D-00c / D-00d treatment: `REQUIREMENTS.md`'s SEC-01 block and its
  run-5 correction banner, and `.planning/codebase/CONCERNS.md:257-268` (dated 2026-07-30, still
  listing all ten including `gcc` and all four D-04 dead ones).
  **After D-04 and D-05 the live suppression surface is ten entries: five vulnerability-class
  (identical in both files) and five unmaintained (`dotenv`, `fxhash`, `number_prefix`,
  `rustls-pemfile`, `paste` — all four confirmed present in `Cargo.lock` this session, plus
  `paste`).** That is the set the governance backfill covers. **Do not plan a backfill of fifteen.**

### SEC-01(c) — the governance schema and the CI reconciliation

- **D-06: Extend M10 Epic 2 FR-3's four-field schema with owner and expiry, via an ADR.**
  Record the extension in an ADR rather than treating it as a defect fix.
  REQUIREMENTS.md is explicit that FR-3's schema (advisory ID, affected crate and why present, why
  not yet fixable, revisit condition) **does not require an owner or an expiry** — only
  `rustsec-remediation-plan.md` adds those, and only for the original two. So ROADMAP criterion 2's
  "owner, expiry date, affected scope, compensating control" is a *stricter* bar than the governing
  PRD. Under D-00b the criterion governs and the PRD is superseded — but a supersession of a
  completed milestone's own acceptance criteria must be written down, not inferred.
  The register's schema is the union: advisory ID · class · affected crate and path · why present ·
  why not yet fixable · **owner** · **expiry / review date** · affected scope · compensating control ·
  revisit condition.

- **D-07: Delete `ci.yml`'s second `cargo audit` job here; re-scope Phase 12's SUPPLY-01.**
  SUPPLY-01 becomes verification-only. ⚠ **HUMAN REVIEW — this changes another phase's scope.**
  ROADMAP criterion 3 ("`make audit`, `ci.yml:77` and `ci.yml:406` cannot pass different advisory
  sets, because there is one configuration rather than three") **cannot be satisfied without the
  deletion.** REQUIREMENTS.md delegates the concrete deletion to SUPPLY-01 (Phase 12) and warns "Do
  not plan the same fix twice". Both cannot hold. Resolution: Phase 9 makes the deletion — it is
  running now, the criterion is its own, and the roadmap itself notes Phase 12 "should not wait for
  Phase 9" in the other direction — and the close-out plan marks SUPPLY-01 **closed by Phase 9 with
  the commit SHA** in `REQUIREMENTS.md`, so Phase 12 inherits a closed item rather than re-planning
  it. The same treatment applies to **SUPPLY-02**, whose three clauses (ratify or remove the three
  unauthorised 2026 vulnerability ignores; extend the FR-3 schema with owner and expiry; backfill
  the rest) are executed here by D-06, D-08 and D-09 respectively.
  **Line numbers have moved and the record is stale:** REQUIREMENTS.md and the roadmap cite
  `ci.yml:389-406` and `ci.yml:406`. Verified this session, the job is **`ci.yml:465-482`** — comment
  at `:465`, `security:` at `:466`, `run: cargo audit --ignore RUSTSEC-2023-0071 --ignore
  RUSTSEC-2025-0111` at **`:482`**. The surviving job is `security-audit:` at **`:61-78`** (bare
  `cargo audit` at `:78`). **Both jobs are named `Security Audit`** — deleting one also closes the
  duplicate-display-name defect SUPPLY-01 clause 1 describes.
  — **Reversibility:** reversible — an 18-line deletion in a workflow file.

- **D-08: The three 2026 vulnerability ignores are ratified, not removed.**
  The ratification is written into the register.
  `RUSTSEC-2026-0187` (lopdf via `pdf-extract`), `-0194` and `-0195` (quick-xml via
  `rust-s3`/`aws-creds`) are authorised by no ingested document; M10 Epic 2 FR-3 §5 names exactly
  two. That is an authorisation gap, not a technical error: all three are real, all three are
  DoS-class, all three have documented no-clean-upstream-fix reasoning already in both files, and
  removing them turns CI red on advisories nobody can fix. Ratifying them via ADR-0024 converts an
  unauthorised expansion into an authorised one and is the outcome SUPPLY-02 asks for ("ratified by
  a recorded decision **or** removed").
  Each gets a compensating control stated concretely, not generically — for `-0187`, that Paladin
  never feeds attacker-controlled PDFs in a default build; for `-0194`/`-0195`, that the `s3`
  feature's XML input is S3 API responses from a configured endpoint.

- **D-09: Owner becomes the repository owner, not "Platform Security (Milestone 7)".** ⚠ **HUMAN
  REVIEW — this reassigns accountability for a signed security acceptance.**
  `rustsec-remediation-plan.md:39` names the owner as a team label attached to a milestone that
  ended. The only named human approver anywhere in this corpus is **`DF3NDR` (repository owner)**,
  from `license-compatibility-decision-checklist.md`. A security exception whose owner is a closed
  milestone has no owner.

- **D-10: Renew the 2026-09-30 acceptance to a per-advisory review date of 2026-12-31.**
  Not closed, not blanket-renewed.
  Closing requires an upstream fix; verified this session that none exists for either original
  advisory (`rsa` via `sqlx-mysql`, `tokio-tar` via `testcontainers`, both dev/test-scoped). A single
  blanket date is what produced the current state, where one date governs two advisories and nine
  others have none. Per-advisory dates in the register, all set to **2026-12-31** initially so the
  first review is a single event, and diverging thereafter as each advisory's upstream moves.
  Rationale for the date: ~5 months out, inside the next two quarters, and short enough that a
  security acceptance never runs a full year unreviewed. The original cadence was ~4 months
  (2026-05-28 → 2026-09-30).
  **This is the corpus's only dated item and it expires eight weeks from now. If this phase slips
  past 2026-09-30, the acceptance lapses.**

### SEC-02 — the licence posture

- **D-11: Adopt `MIT OR Apache-2.0` across the root package and all ten library crates.**
  This matches the signed checklist.
  ⚠ **HUMAN REVIEW — this is a licence change on already-published crates, and
  SEC-02 says in terms that it "must not be resolved by inference".** This decision is the
  *recommendation*; it must be confirmed by `DF3NDR` before any plan executes it.
  Verified this session: `license = "MIT"` appears in exactly eleven manifests — root `Cargo.toml:40`
  (`[workspace.package]`) and each of the ten `crates/*/Cargo.toml`. A single `LICENSE` file sits at
  root; there is no `LICENSE-APACHE`.
  Reasoning, in order of weight:
  1. **The checklist is the only signed governance artefact in the corpus** — named approver
     (`DF3NDR`), date (2026-05-28), 551 packages inventoried, zero unknown. Withdrawing it destroys
     the project's only completed compliance review; changing eleven metadata fields does not.
  2. **The 551-package sign-off depends on the dual-licence rule.** "Any SPDX expression containing a
     permissive MIT/Apache branch is acceptable by default" was the stated basis for accepting
     `r-efi 5.3.0`'s `MIT OR Apache-2.0 OR LGPL-2.1-or-later`. Under MIT-only that rationale is, in
     REQUIREMENTS.md's own words, "weaker than recorded" — so choosing MIT means re-justifying a
     completed 551-package review.
  3. **Direction of change is safe.** Adding Apache-2.0 as an *alternative* is a grant of additional
     permission to every existing consumer of the published 0.1.0 crates. It takes nothing away. The
     reverse — narrowing a published `MIT OR Apache-2.0` to `MIT` — would not be safe, which is why
     this direction is the one that can be taken on already-published crates at all.
  4. `MIT OR Apache-2.0` is the Rust ecosystem norm and gives consumers an explicit patent grant.
  **Cost this decision accepts, and the planner must budget for it:** `LICENSE` becomes
  `LICENSE-MIT`, a verbatim `LICENSE-APACHE` is added, `README.md`'s licence section and any badge
  are updated, eleven manifests change, `deny.toml`'s allow-list already permits both (verified:
  `MIT` and `Apache-2.0` are both in `[licenses] allow`), and `CHANGELOG.md` records it. The
  `license-compatibility-decision-checklist.md` is annotated per D-00c as **confirmed and now
  declared**, and the M7 Epic 4 PRD §4.7.7 / overview AC 1 `MIT` claims are annotated as superseded.
  — **Reversibility:** one-way — the 0.1.0 crates are published on crates.io under MIT; publishing a
  later version under `MIT OR Apache-2.0` cannot be retracted, and narrowing back to MIT afterwards
  would revoke a permission consumers were granted.

- **D-12: If the human review lands on MIT-only instead, the deliverable is the same shape.**
  The checklist gets a D-00c annotation recording it **superseded** with the reason, the dual-licence
  approval rule's effect on `r-efi` is re-justified explicitly (not left implicit), the eleven
  manifests are unchanged, and `deny.toml`'s allow-list is unchanged. Either answer closes SEC-02;
  only D-11 requires files to move. **A plan must not be written that assumes the answer** — this is
  the one gray area in the phase whose auto-resolution is genuinely a placeholder for a human.

### SEC-03 — the crates.io name-collision guard

- **D-13: Add an offline name-guard, not a network availability check.**
  A committed `.crate-names.txt` (or an equivalent list inside `deny.toml`'s bans section — planner's
  choice) enumerates the eleven package names this project already owns on crates.io. A CI step —
  and a `make` target — fails when a workspace `[package] name` is not on that list. Adding a new
  crate therefore requires a deliberate one-line addition, at which point the release runbook
  instructs a manual availability check.
  Chosen over a live crates.io sparse-index query for three reasons: (1) **it is provable in this
  environment** — `crates.io` returns HTTP 403 here, so a network check could be written but never
  demonstrated, which is the Phase 8 D-03 failure mode; (2) it cannot be flaky in CI; (3) it catches
  the exact failure Epic 4 actually hit — a **new** name colliding, which cost two package renames
  (`paladin-core` → `paladin-ai-core`, root → `paladin-ai`, the reason every crate's package name and
  lib name now diverge) and a full NO-GO cycle. The eleven existing names are already owned, so
  their collision risk is zero.
  Chosen over pure acceptance of dry-run reliance (SEC-03's second permitted answer): the guard is
  perhaps twenty lines and runs on every PR, whereas today's earliest guard is `ci.yml:902`
  `publish-dry-run`, **main-branch-only** (verified this session at `:901-929`), with
  `release.yml:406-427` doing per-crate dry runs inside the release job itself.
  Recorded in ADR-0026 with the accepted residual cost stated: a genuinely novel name is still
  checked by a human, not by CI.

### SEC-04 — the herald changelog

- **D-14: Create `crates/paladin-herald/CHANGELOG.md`. No exemption.**
  Verified this session: nine of ten library crates carry `CHANGELOG.md`; `crates/paladin-herald/`
  holds `Cargo.toml`, `README.md` and `src/` only. Herald ships on crates.io under a release-gate
  criterion its own completion summary recorded **Met**. The crate was created after that audit, by
  reconciliation commit `66f6c4e`.
  Backfill content from real history, not a stub: creation by the 2026-06-04 facade-cleanup
  reconciliation, and **Phase 8's D-14 feature-gating of `colored` / `comfy-table`, which shrank the
  crate's default public API** — a user-visible published-contract change that already owes a
  changelog entry. Keep-a-Changelog format, matching the nine siblings.

- **D-15: Add a guard asserting every library crate directory carries a CHANGELOG.md.**
  Folded into `scripts/check-advisory-register.sh`'s sibling or into the release workflow —
  planner's choice of home, but it must exist. Herald was missed because nothing checked. This is the
  same mechanism-not-assertion idiom as D-02 and D-16, and it costs one `for` loop.

### SEC-05 — the Dockerfile.chef planner stage

- **D-16: Delete the nine-manifest enumeration rather than extend it to ten.** ⚠ **HUMAN REVIEW —
  this supersedes M7 Epic 2 FR-01, a PRD requirement on a milestone recorded complete.**
  SEC-05's done-condition is explicit: *"an enumerated list that goes stale on every crate addition
  is the defect, not just the one missing line."* Adding the herald line satisfies the letter and
  reproduces the defect.
  Verified this session, `Dockerfile.chef:24-38`: the planner copies `Cargo.toml Cargo.lock`, then
  **nine** crate manifests by name at `:25-33` (herald omitted), then `COPY src ./src` at `:35`,
  `COPY crates ./crates` at `:36`, `COPY benches ./benches` at `:37`, then `cargo chef prepare` at
  `:38`.
  **The finding is sharper than the record:** because `COPY crates ./crates` at `:36` runs *before*
  `chef prepare` at `:38`, the per-manifest copies at `:25-33` cannot deliver the cache-tightness §6
  describes — the later full-tree copy invalidates on any source change, for all ten crates, not just
  herald. And `cargo chef prepare` distils a content-independent `recipe.json` skeleton, so the
  planner layer's own cache is not what makes the pattern work. On that reading the enumeration is
  **decorative**, and deleting it makes staleness structurally impossible rather than merely guarded.
  **This decision is contingent on that reading and the researcher must confirm it.** `docker` is
  absent from this environment (recorded by Phase 4 at `04-ci-gate-deferrals.md`), so the cache
  behaviour cannot be measured here — it must be established from cargo-chef's documented recipe
  semantics (Context7 / upstream docs), not assumed.
  **Fallback, if research contradicts the reading:** keep the enumeration, add the herald line, and
  add a guard script asserting every `crates/*/Cargo.toml` appears in `Dockerfile.chef`'s planner
  stage. That also satisfies the done-condition — Docker's `COPY` cannot preserve per-directory paths
  through a glob, so enumeration is forced by the tool and the anti-staleness burden moves to the
  guard. **Both branches close SEC-05; the plan must state which one it took and why.**
  — **Reversibility:** reversible — a Dockerfile block, restorable from git; no published artefact
  depends on it.

### SEC-01's forward coupling

- **D-17: SEC-01 does not wait for HARD-06; this phase supplies the decoupling evidence.**
  ⚠ **Fresh finding.**
  REQUIREMENTS.md sequences SEC-01 behind HARD-06 (Phase 10) because "the `RUSTSEC-2026-0187`
  suppression rests on `pdf-extract` being reachable". Verified this session at
  `crates/paladin-content/Cargo.toml:41`: `pdf-extract = { version = "0.7" }` — **unconditional**,
  not `optional = true`, and not gated by the crate's `pdf = []` feature (`:18`), which is empty and
  gates nothing. `pdf-extract` and `lopdf` both appear in `Cargo.lock`. So `lopdf` is in the graph
  whenever `paladin-content` builds, and the suppression is warranted **regardless of how HARD-06
  answers the capability question**.
  SEC-01 therefore ratifies `-0187` on tree evidence (D-08) and hands HARD-06 the `file:line` finding
  as an input. **This phase does not answer "is PDF extraction supported?"** — the mismatch between
  an empty `pdf` feature and a mandatory `pdf-extract` dependency is exactly HARD-06's subject, and
  it stays there.

### Cross-cutting

- **D-18: ADR allocation — 0024, 0025, 0026, 0027; `PROMOTION.md` advances to 0028.**
  - **ADR-0024** — RustSec exception governance: the authoritative register, the extended schema
    (D-06), the ratification of the three 2026 advisories (D-08), the owner change (D-09), and the
    2026-09-30 disposition (D-10). Supersedes M10 Epic 2 FR-3's four-field schema.
  - **ADR-0025** — licence posture (D-11 / D-12). **Blocking on human confirmation.**
  - **ADR-0026** — crates.io name-collision guard, with the accepted residual cost (D-13).
  - **ADR-0027** — `Dockerfile.chef` planner-stage supersession of M7 Epic 2 FR-01 (D-16), stating
    which branch was taken and the cargo-chef evidence behind it.
  **SEC-04 gets no ADR** — creating a missing changelog is a plain defect fix with no competing
  defensible position (D-00g). D-04's four dead-suppression deletions get no ADR either; they are
  recorded inside ADR-0024 as a consequence of Phase 8.

- **D-19: Every closure claim is proved by a command run in this environment and recorded verbatim.**
  The D-00e bar. Runnable here and expected: `grep`/`sed` reads of `deny.toml`, `.cargo/audit.toml`,
  `ci.yml`, `Cargo.lock`, the eleven manifests, `Dockerfile.chef`. **Not runnable here:** `cargo
  audit` and `cargo deny` (both need `cargo install` against a crates.io that returns HTTP 403), and
  anything Docker. Where a gate cannot be executed, land the change, record the exact command a CI
  runner will execute, and scope the closure claim honestly — never infer a pass. This is Phase 8's
  D-03 lesson, and it applies harder here because **four of this phase's five requirements are about
  gates that cannot be run locally.**

- **D-20: Record SEC rows in `REQUIREMENTS.md` and hand them to Phase 10.**
  The Milestone 7-8 ledger does not yet exist.
  HARD-01 (Phase 10) builds the Milestone 7-8 ledger for all 86 run-4 requirement IDs. Phase 9 runs
  first, so there is no ledger to amend in place. The close-out therefore flips the SEC-01 … SEC-05
  checkboxes in `REQUIREMENTS.md` behind evidence, updates the traceability rows at
  `REQUIREMENTS.md:3755-3759`, and writes an explicit hand-off block naming every `REQ-*` row Phase
  10's ledger must record as already-closed: `REQ-rustsec-risk-acceptance`,
  `REQ-rustsec-hardening-actions`, `REQ-license-policy-signoff`, `REQ-crate-metadata-completion`,
  `REQ-per-crate-changelog`, `REQ-docker-workspace-build`,
  `REQ-paladin-ports-publish-verification-closed`.

- **D-21: `.planning/codebase/CONCERNS.md` is corrected in this phase, not left to drift.**
  Its advisory section (`:257-268`, dated 2026-07-30) lists ten unmaintained advisories including
  `gcc` (removed) and the four D-04 dead ones. It sits at the `.planning/codebase/` map tier of the
  precedence order — above PRDs — so leaving it wrong actively misleads. Phase 7 set the precedent by
  correcting `STRUCTURE.md`.

- **D-22 [informational]: Suggested decomposition — ~7 plans, 3 waves.**
  - **Wave 1 (fully parallel — zero file overlap):**
    ① **ADR-0024** + `SECURITY-EXCEPTIONS.md` — the register authored before the config it governs
    (D-01, D-06, D-08, D-09, D-10).
    ② **SEC-04** — `crates/paladin-herald/CHANGELOG.md` + the every-crate guard (D-14, D-15).
    ③ **ADR-0027** + `Dockerfile.chef` (D-16), including the cargo-chef research that picks the
    branch.
    ④ **ADR-0026** + the name guard (D-13).
  - **Wave 2 (blocked on wave 1 ①):**
    ⑤ **SEC-01 config** — the four dead deletions (D-04), the `ci.yml:465-482` job deletion (D-07),
    both TOMLs' comment/pointer updates, and `scripts/check-advisory-register.sh` (D-02). One plan,
    because `deny.toml` and the guard that validates it must land together or CI is red between them.
    ⑥ **SEC-02** — ADR-0025 plus the eleven manifests, `LICENSE-MIT`/`LICENSE-APACHE`, `README.md`
    and `CHANGELOG.md` (D-11). **Gate this plan on a blocking human checkpoint before task 1** —
    D-11 is `one-way` and SEC-02 forbids resolution by inference.
  - **Wave 3:**
    ⑦ **Close-out** — `REQUIREMENTS.md` checkbox flips and traceability rows behind evidence (D-20),
    the D-05 source corrections, `CONCERNS.md` (D-21), `PROMOTION.md` → 0028, `PROJECT.md` Key
    Decisions rows, the SUPPLY-01 / SUPPLY-02 closure notes for Phase 12 (D-07), the HARD-06 hand-off
    (D-17), `COVERAGE.md`, and the ADR-0006 floor re-check (expected: unchanged, no `.rs` touched).
  Plan-file naming is `09-NN-PLAN.md`.
  **File contention to respect:** `deny.toml` is touched only by ⑤. `ci.yml` is touched by ⑤ and
  possibly ②/④ if their guards become CI steps — if so, serialise the CI-step additions into ⑤ or
  give each a distinct job block, and note that ⑤'s deletion at `:465-482` shifts every later line
  number by 18.

### Claude's Discretion

- The register file's exact name and format — `SECURITY-EXCEPTIONS.md` and a Markdown table are the
  recommendation; a `.toml` or `.yml` the guard script parses is defensible and arguably better for
  D-02's clause 2. The constraint is: repo root or immediately adjacent to the configs, and
  machine-checkable.
- Whether the name guard (D-13) and the changelog guard (D-15) are separate scripts, one script, or
  steps inside an existing CI job.
- The home of `.crate-names.txt` — a standalone file, a `deny.toml` section, or a `Makefile`
  variable.
- Whether ADR-0024 through 0027 are authored in their own plans or fold into the plans that execute
  them. D-22 puts 0024, 0026 and 0027 with their work and 0025 ahead of its work; a planner with a
  reason to differ may.
- Banner wording and inline-correction markup for the D-05 and D-11 `.project/` annotations (D-00c
  fixes the pattern, not the prose).
- Whether `SECURITY-EXCEPTIONS.md` also becomes a `SECURITY.md` for GitHub's advisory UI, or stays
  purely an exception register. There is no `SECURITY.md` today; adding one is adjacent but is a
  separate deliverable — if it is skipped, note it rather than silently omitting it.

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Phase scope and requirements

- `.planning/ROADMAP.md` §"Phase 9: Release & Security Gate Integrity" (line 618) — the goal, the
  dependency note, and the five success criteria. **Criterion 3 requires the CI job deletion that
  REQUIREMENTS.md delegates to Phase 12; see D-07.**
- `.planning/REQUIREMENTS.md` lines 1055-1203 — **SEC-01 … SEC-05 in full**, including the
  **run-5 correction banner at lines 1060-1081** which withdraws run 4's "deny.toml out of sync"
  finding. **This is the authoritative statement of scope** and it is much longer than the ROADMAP
  summary. Read the banner before touching `deny.toml`.
- `.planning/REQUIREMENTS.md` lines 3755-3759 — the SEC-01 … SEC-05 traceability rows this phase
  flips.
- `.planning/REQUIREMENTS.md` lines 3843-3848 — the cross-phase coupling table:
  HARD-06 → SEC-01 (**D-17 decouples it with tree evidence**) and SEC-01 → SUPPLY-01 / SUPPLY-02
  (**D-07 absorbs both**).
- `.planning/REQUIREMENTS.md` lines 3027-3038 — the Milestone 7-8 as-shipped ledger rows whose
  verdicts this phase changes (`REQ-rustsec-risk-acceptance`, `REQ-license-policy-signoff`,
  `REQ-per-crate-changelog`, `REQ-docker-workspace-build`, and the rest of D-20's list).
- `.planning/ROADMAP.md` §"Phase 12: Supply-Chain Gate Integrity" (line 667) — read it, because
  D-07 changes what is left in it.
- `.planning/ROADMAP.md` §"Phase 10" (line 633), criterion 6 — HARD-06's PDF question, which D-17
  feeds and does not answer.

### Source documents this phase reconciles

- `.project/Milestone_7-Production-Hardening/Epic_4/rustsec-remediation-plan.md` — **the formal risk
  acceptance.** `:39` owner "Platform Security (Milestone 7)" (D-09), `:40` the **2026-09-30**
  review/expiry target (D-10), `:53` the risk-acceptance rationale requirement, `:73` the open action
  item "add `audit.toml` exception entries only if approved, each with expiry date and owner",
  `:79-81` the formal-acceptance field list. **The only dated item in the corpus.**
- `.project/Milestone_7-Production-Hardening/Epic_4/license-compatibility-decision-checklist.md` —
  the signed `MIT OR Apache-2.0` policy: approver `DF3NDR`, date 2026-05-28, 551 packages, the
  dual-licence approval rule, and the MPL-2.0 acceptance. **The only signed governance artefact in
  the corpus** (D-11).
- `.project/Milestone_7-Production-Hardening/Epic_4/deferred-paladin-ports-publish-verification.md` —
  Task 5.5 Resolved, leaving exactly one residue: "keep CI/package guardrails that detect crates.io
  package-name collisions early" (D-13).
- M10 Epic 2's dependency/security/licence PRD — FR-3's four-field exception schema (which **omits**
  owner and expiry, D-06), FR-14(a)/(b) licence justification and exception patterns, and §8's
  success metric "no inline advisory-ignore flags remain in CI" (D-07).
- M10 Epic 4 FR-1 step 5 — the authorisation for scoped unmaintained-advisory ignores (D-03).
- M7 Epic 2 FR-01 and §6 — the `Dockerfile.chef` planner requirement and its cache-tightness
  rationale, which **D-16 supersedes**.

### Prior decisions this phase builds on

- `.planning/decisions/PROMOTION.md` — the numbering index, **next free 0024** (`:47`), and the
  five-step append procedure. Read before writing ADR-0024…0027; advance to 0028 in the close-out.
- `.planning/decisions/0001-battalion-config.md` … `0023-cli-dependency-isolation.md` — the ADR file
  shape. **0024-0027 must match it** (no frontmatter, seven headings, per D-00a).
- `.planning/decisions/0023-cli-dependency-isolation.md` — Phase 8's `structopt` → clap v4 migration
  and the herald feature gating. **This is the direct cause of D-04 (four dead suppressions) and of
  the content D-14's changelog must record.**
- `.planning/decisions/0006-coverage-gate.md` — the 84% workspace line-coverage floor. This phase
  changes no `.rs`; the close-out re-check should confirm the number is unmoved.
- `.planning/decisions/0008-workspace-version-0-7-0.md` — the version story any `CHANGELOG.md` entry
  written here must be consistent with.
- `.planning/phases/08-verified-defect-closure/08-CONTEXT.md` — the source of D-00a…D-00g and the
  precedent for the ⚠ HUMAN REVIEW convention used above.
- `.planning/phases/08-verified-defect-closure/08-09-SUMMARY.md` — Phase 8's close-out; read its
  hand-off section, and confirm what it recorded about the removed CLI dependencies before acting on
  D-04.

### Defect and change sites — all verified this session, 2026-08-07

**SEC-01:**
- `.cargo/audit.toml` — `[advisories] ignore` holds **five** vulnerability advisories
  (`RUSTSEC-2023-0071`, `-2025-0111`, `-2026-0187`, `-2026-0194`, `-2026-0195`); the comment block
  mentions `-2026-0185` and `-2026-0190` as **upgraded rather than ignored**.
- `deny.toml` `[advisories] ignore` — **fourteen** entries in three labelled classes: 2 mirrored
  vulnerability + **9** unmaintained + 3 new-2026 vulnerability. **`RUSTSEC-2025-0121` (gcc) is
  absent** — the record's "fifteen / ten" is stale (D-05).
- `Cargo.lock` — `structopt`, `ansi_term`, `atty`, `proc-macro-error`, `gcc` all return **0**;
  `dotenv`, `fxhash`, `number_prefix`, `rustls-pemfile`, `paste` all return **1** (D-04).
- `.github/workflows/ci.yml:61-78` — `security-audit:` / display name `Security Audit`, bare
  `cargo audit` at `:78`, under a comment at `:75-77` declaring `.cargo/audit.toml` the single source
  of truth. **This is the job that survives.**
- `.github/workflows/ci.yml:465-482` — `security:` / display name **also `Security Audit`**, `cargo
  audit --ignore RUSTSEC-2023-0071 --ignore RUSTSEC-2025-0111` at `:482`. **This is the job D-07
  deletes.** The record's `:389-406` / `:406` citations are stale.
- `.github/workflows/ci.yml:105-106` — `cargo deny check`, the `cargo-deny` job.
- `Makefile:243-247` — `make audit`, bare `cargo audit`, comment naming `.cargo/audit.toml` the
  single source of truth. `:249-252` `make deny`. `:260-261` `make security` = audit + deny.
- `crates/paladin-content/Cargo.toml:41` — `pdf-extract = { version = "0.7" }`, **unconditional**;
  `:18` `pdf = []`, an empty feature. `pdf-extract` and `lopdf` both in `Cargo.lock` (D-17).

**SEC-02:**
- `Cargo.toml:40` — `license = "MIT"` in `[workspace.package]`.
- `crates/paladin-{core,ports,battalion,herald,llm,memory,storage,notifications,content,web}/Cargo.toml`
  — `license = "MIT"` in all ten (lines `:6` or `:8`).
- `LICENSE` — a single root file; **no `LICENSE-APACHE`** (D-11's cost).
- `deny.toml [licenses] allow` — permissive-only, **already includes both `MIT` and `Apache-2.0`**,
  plus `BSD-2-Clause`, `BSD-3-Clause`, `ISC`, `Zlib`, `Unicode-3.0`, `0BSD`, `CC0-1.0`,
  `CDLA-Permissive-2.0`, each with an FR-14(a) justification comment; eight `[[licenses.exceptions]]`
  MPL-2.0 entries. **No change needed here under either D-11 or D-12.**

**SEC-03:**
- `.github/workflows/ci.yml:901-929` — `publish-dry-run`, **main-branch only**, `cargo publish
  --workspace --dry-run` at `:929`, with a comment explaining why per-crate dry runs cannot work on a
  version bump.
- `.github/workflows/release.yml:353-427` — `publish-crates`, the dependency-first publish order at
  `:392`, `publish_one()` at `:406`, per-crate `cargo publish --dry-run -p` at `:410`.

**SEC-04:**
- `crates/paladin-herald/` — contains `Cargo.toml`, `README.md`, `src/` and **no `CHANGELOG.md`**.
- The other nine `crates/*/CHANGELOG.md` all exist — the format to match.

**SEC-05:**
- `Dockerfile.chef:24` `COPY Cargo.toml Cargo.lock ./`; `:25-33` the **nine** named crate manifests,
  herald omitted; `:35` `COPY src ./src`; `:36` `COPY crates ./crates`; `:37` `COPY benches
  ./benches`; `:38` `RUN cargo chef prepare --recipe-path recipe.json` (D-16).
- `Dockerfile.chef:10` `FROM rust:1.93-slim-bookworm`, `:14` `cargo install cargo-chef --version
  0.1.77 --locked`, `:55` `cargo chef cook --release --workspace`.
- `.planning/phases/04-release-coherence/04-ci-gate-deferrals.md` — the record that **docker is
  absent from this environment**, so nothing here can be measured (D-16, D-19).

### Codebase maps and conventions

- `.planning/codebase/CONCERNS.md:257-284` — the advisory sections. **Stale, dated 2026-07-30, and
  corrected by D-21.** `:278-284` cites `deny.toml:141-147` for the three 2026 advisories; verify the
  line numbers before citing them.
- `.planning/codebase/STACK.md`, `.planning/codebase/INTEGRATIONS.md` — the dependency and CI
  surface this phase edits.
- `CLAUDE.md` and `.github/instructions/rust.instructions.md` — the workspace gate
  (`cargo test` → `cargo fmt --check` → `cargo clippy -- -D warnings`) and the medieval-military
  ubiquitous-language requirement.

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets

- **`.cargo/audit.toml`'s existing comment structure** — already carries advisory ID, affected crate,
  transitive path, why-not-fixable and a revisit condition for all five entries. That is four of
  D-06's ten register fields already written; the register lifts them rather than re-deriving them.
- **`deny.toml`'s three-class labelling and its FR-14(a)/(b) licence pattern** — the `[licenses]`
  section is described in REQUIREMENTS.md as "textbook FR-14(b) compliance". It is the model for how
  the advisory section should read once D-06's schema lands, and it needs **no change** under either
  licence outcome.
- **The nine existing `crates/*/CHANGELOG.md`** — the Keep-a-Changelog shape D-14 copies. Do not
  invent a format.
- **`.planning/decisions/0001`-`0023`** — twenty-three ADRs in the target format. **0023 is this
  phase's direct input** (it caused D-04); 0020 shows how to judge a stale artefact rather than
  reconcile it (the model for D-05 and D-16); 0006 and 0008 show how a later phase cites an earlier
  answer instead of re-deciding it.
- **`scripts/check-deprecations.sh` and `scripts/check-api-surface.sh`** — the existing repo-guard
  script idiom D-02, D-13 and D-15 follow. **Read `check-deprecations.sh` as a cautionary example
  first**: Phase 8 D-05 found both its branches `exit 0`, so it presented as a gate and could not
  fail. Every guard this phase writes must be demonstrated failing as well as passing.

### Established Patterns

- **Mechanism, not assertion.** This phase's own done-conditions say it twice — SEC-05's "the
  mechanism cannot miss an eleventh crate" and SEC-01's "one register is authoritative and the other
  surfaces mirror it exactly". The repository has now shipped at least three invariants asserted in
  comments and enforced by nothing: `deny.toml`'s "keep these two files in sync" header,
  `check-deprecations.sh`'s unfailable gate, and the per-crate changelog criterion that missed
  herald. **Every clause this phase closes should end in a script or a CI step**, not a paragraph.
- **Retain superseded text; amend in place; date every amendment** (D-00c, D-00d) — applies to
  `REQUIREMENTS.md`, `CONCERNS.md`, and every `.project/` annotation.
- **Contested positions get ADRs; code-settled defects get rows** (D-00g) — D-18 allocates against
  it: four ADRs for four contested decisions, none for the herald changelog.
- **Documents lie about themselves in both directions.** Phase 8's central lesson, and this session
  reproduced it four times: the corpus says fifteen `deny.toml` entries (it is fourteen), says ten
  unmaintained (nine), says `ci.yml:406` (it is `:482`), and sequences SEC-01 behind HARD-06 for a
  reachability question the manifest already answers. **Re-read every cited `file:line` before acting
  on it** — this phase's four sharpest findings all came from doing exactly that.
- **Phase 8 moved the ground under this phase.** The `structopt` removal invalidated four
  suppressions and their justifications. Any Phase 9 plan that reads only the ingest record and not
  `Cargo.lock` will plan governance for dependencies that no longer exist.

### Integration Points

- **`deny.toml`** — D-04 deletes four entries, D-06 rewrites the comment structure, D-01 adds the
  register pointer. Single-plan ownership (D-22 ⑤).
- **`.cargo/audit.toml`** — comment/pointer updates only; its five entries are all live and all stay.
- **`.github/workflows/ci.yml`** — `:465-482` deleted (D-07), and up to three guard steps added
  (D-02, D-13, D-15). **The deletion shifts every subsequent line number by 18** — anything citing
  `ci.yml:9xx` after this phase must be re-derived, including `publish-dry-run` at `:901-929`.
- **Eleven `Cargo.toml` manifests + `LICENSE` + `README.md` + `CHANGELOG.md`** — D-11's blast radius,
  gated on human confirmation.
- **`Dockerfile.chef:25-33`** — deleted or extended-plus-guarded (D-16). `Dockerfile` and
  `Dockerfile.server` are **not** in scope; confirm neither carries the same enumeration before
  closing.
- **`crates/paladin-herald/CHANGELOG.md`** — new file (D-14).
- **`SECURITY-EXCEPTIONS.md`** — new root file (D-01).
- **`scripts/`** — one to three new guard scripts (D-02, D-13, D-15).
- **Phase 10 / HARD-01** — receives D-20's hand-off block naming seven already-closed `REQ-*` rows,
  and D-17's PDF reachability finding as HARD-06 input.
- **Phase 12 / SUPPLY-01 + SUPPLY-02** — receives closure notes rather than work (D-07). **Phase 12
  shrinks to SUPPLY-03 plus verification.**
- **Phase 15 / PIPE-04** — inherits the deprecated-Action sweep, including any inside jobs this phase
  edits. Untouched here.

</code_context>

<specifics>
## Specific Ideas

**Five findings surfaced during this session that neither the ingest record nor Phase 8 contains.**
Each was read from the tree on 2026-08-07. Treat them as verified starting points, not hypotheses.

1. **Phase 8 deleted four of this phase's suppressions out from under it.** `structopt`, `ansi_term`,
   `atty` and `proc-macro-error` all return **0** hits in `Cargo.lock` after ADR-0023's clap v4
   migration, and every one of their `deny.toml` justifications reads "via structopt" or "via clap
   2.x/structopt". SEC-01's governance backfill therefore covers **ten** live entries, not fifteen —
   five vulnerability-class and five unmaintained. A planner sizing from REQUIREMENTS.md will plan a
   50% larger backfill than exists, and will attach an owner and an expiry to four suppressions that
   suppress nothing.

2. **The corpus's `deny.toml` arithmetic is off by one in a way nothing caught.** The record says
   fifteen entries and ten unmaintained; the file holds **fourteen** and **nine**.
   `RUSTSEC-2025-0121` (`gcc`) is gone from `deny.toml`, from `.cargo/audit.toml` and from
   `Cargo.lock`, with no record of its removal anywhere. `.planning/codebase/CONCERNS.md:257-268`
   still lists it. This is the second time a suppression set has drifted silently — which is the
   argument for D-02's guard, and it should be stated that way in ADR-0024.

3. **The `pdf-extract` sequencing dependency is already answered by the manifest.**
   `crates/paladin-content/Cargo.toml:41` declares `pdf-extract = { version = "0.7" }` with **no
   `optional = true`**, while `:18`'s `pdf = []` is an empty feature gating nothing. `lopdf` is
   consequently in the graph whenever `paladin-content` builds. So `RUSTSEC-2026-0187`'s suppression
   is warranted on tree evidence and SEC-01 does not have to wait for HARD-06 (Phase 10). The
   *contradiction* — a mandatory dependency behind a feature flag that gates nothing — is real and
   is exactly HARD-06's subject; hand it over, do not solve it.

4. **`Dockerfile.chef`'s manifest enumeration may be decorative, not merely incomplete.** `COPY
   crates ./crates` at `:36` runs before `cargo chef prepare` at `:38`, so the per-manifest copies at
   `:25-33` cannot deliver the layer-cache tightness §6 claims — for any crate, not just herald. If
   cargo-chef's recipe distillation works as documented, the correct fix is **deletion**, which is
   also the only version of the fix that satisfies SEC-05's "cannot miss an eleventh crate". **This
   needs research confirmation** — docker is absent here, so it cannot be measured; the fallback
   (enumerate + guard) is written into D-16.

5. **The record's `ci.yml` line numbers are ~77 lines stale, and both audit jobs share a display
   name.** REQUIREMENTS.md and SUPPLY-01 cite `ci.yml:389-406` / `:406`; the job is at **`:465-482`**
   today. Both `security-audit:` (`:61`) and `security:` (`:466`) render as **"Security Audit"** in
   the GitHub UI, which is why two jobs configured to reach different verdicts on the same
   `Cargo.lock` were invisible. **Re-derive every `ci.yml` citation before editing**, and note that
   D-07's deletion moves everything below `:482` up by 18 lines.

**Scale note for the planner:** five requirements, ~7 plans, and a wide spread of sizes and risks.
SEC-04 is one new file. SEC-05 is a Dockerfile block plus a research question. SEC-03 is ~20 lines of
guard plus an ADR. SEC-01 is the bulk — one register, two config files, one CI deletion, one guard
script, and a re-scoping of Phase 12. SEC-02 is small in diff and the largest in consequence: it is
`one-way`, it touches a published licence, and SEC-02 states in terms that it "must not be resolved
by inference". **Do not size this phase from the ROADMAP's five-bullet summary** — REQUIREMENTS.md
lines 1055-1203 carry a correction banner that changes SEC-01's shape entirely, and this session's
findings 1 and 2 change its size again.

**The clock:** the 2026-09-30 acceptance is **eight weeks out** from today. Of everything in this
phase, D-10 is the only item with an external deadline.

</specifics>

<deferred>
## Deferred Ideas

- **"Is PDF extraction supported?"** — the contradiction between `paladin-content`'s empty `pdf = []`
  feature and its mandatory `pdf-extract` dependency. **Phase 10 / HARD-06**, criterion 6. D-17
  supplies the `file:line` evidence and explicitly declines to answer it.
- **The `cargo doc --workspace --no-deps` warning bar** — Phase 10 / HARD-07. Untouched, as in
  Phase 8's D-12.
- **A `SECURITY.md` for GitHub's advisory / private-reporting UI** — adjacent to D-01's register and
  genuinely missing (there is none today), but a separate deliverable aimed at a different audience.
  Not a SEC-01 … SEC-05 clause. Candidate for Phase 16's documentation work.
- **Replacing `dotenv` with `dotenvy`** — `deny.toml`'s own comment recommends it, and it would
  retire `RUSTSEC-2021-0141` rather than suppress it. That is a dependency change with code
  consequences; this phase governs suppressions, it does not remove them by editing `.rs`.
- **The other four live unmaintained advisories' upstream paths** (`fxhash` via `scraper`,
  `number_prefix` via `indicatif`, `rustls-pemfile` via `tonic`/`testcontainers`, `paste` via
  `utoipa`) — each has a documented revisit condition and no drop-in replacement. The register
  records the condition; acting on any of them is dependency-upgrade work for a later phase.
- **A CI dependency-allowlist check built on `cargo tree`** — Phase 15, from ADR-0015. Carried
  forward unresolved from Phase 8's D-16.
- **The eight deprecated GitHub Action references** — Phase 15 / PIPE-04 owns the full sweep. Two
  sit inside jobs this phase edits; do not opportunistically bump them.
- **Stray root artefacts** — `api_surface_current.txt`, `final-api.txt`, `flat`, `lcov.info` sit
  untracked-looking at repo root alongside the real deliverables. Noticed while scouting; cleaning
  them up is housekeeping, not a SEC requirement. Worth a one-line item in a later close-out.
- **Retiring or replacing `src/main.rs`, the legacy content-aggregator entry point** — carried
  forward unresolved from Phases 7 and 8.
- **Nyquist validation for Phases 1-4** — carried forward unresolved from Phases 5, 7 and 8.
  Owner: `/gsd-validate-phase 1`…`4`.
- **Whether ADRs should be published to the mdbook for framework consumers** — carried forward
  unanswered from Phases 1, 5, 7 and 8. Belongs with Phase 16's documentation work.

</deferred>

---

*Phase: 9-release-security-gate-integrity*
*Context gathered: 2026-08-07*
