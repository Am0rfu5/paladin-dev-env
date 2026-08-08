# ADR-0024: RustSec exception governance

## Status

Accepted

**Date:** 2026-08-08

## Context

Four surfaces encode this workspace's RustSec suppression posture today, and they were built at
four different times by four different mechanisms, verified directly this session:

- **`.project/Milestone_7-Production-Hardening/Epic_4/rustsec-remediation-plan.md:39-40`** is the
  only *formal* risk acceptance in the corpus: it names owner **"Platform Security (Milestone 7)"**
  and a review/expiry target of **2026-09-30**, for exactly two advisories
  (`RUSTSEC-2023-0071` rsa, `RUSTSEC-2025-0111` tokio-tar).
- **`.cargo/audit.toml` `[advisories] ignore`** holds five entries — those two plus
  `RUSTSEC-2026-0187`, `-0194`, `-0195` — with a comment block carrying crate, transitive path and
  why-not-fixable reasoning for all five, but no owner or expiry field for any of them.
- **`deny.toml` `[advisories] ignore`** holds fourteen entries in three labelled blocks: the same
  two vulnerability advisories mirrored from `.cargo/audit.toml`, nine `unmaintained`-class
  advisories, and the same three 2026 vulnerability advisories, under a header stating "the same
  advisory IDs are mirrored here … Keep these two files in sync" — an invariant asserted in prose
  and enforced by nothing.
- **`.github/workflows/ci.yml`** runs two independent `cargo audit` jobs that both render under the
  display name **"Security Audit"** in the GitHub UI: `security-audit:` at `:61-78` (job id `:61`,
  bare `cargo audit` at `:78`, under a comment declaring `.cargo/audit.toml` the single source of
  truth) and `security:` at `:466-482` (job id `:466`, `cargo audit --ignore RUSTSEC-2023-0071
  --ignore RUSTSEC-2025-0111` at `:482`, preceded by a `# Security audit` comment at `:465`). These
  two jobs are configured to reach different verdicts on the identical `Cargo.lock`: one reads all
  five current suppressions, the other reads only the original two.

**The corpus's own arithmetic was wrong twice, in the same document.** The ingested record states
`deny.toml` holds fifteen entries and ten unmaintained notices. Read directly this session,
`deny.toml`'s `[advisories] ignore` array holds **fourteen** entries and **nine** unmaintained
notices. `RUSTSEC-2025-0121` (`gcc`) is absent from `deny.toml`, absent from `.cargo/audit.toml`,
and absent from `Cargo.lock` — someone removed the suppression at some point and no record anywhere
caught or logged the removal. This is the **second** time a suppression set has drifted out from
under its own governing document without anyone noticing at the time it happened (the first being
the four-entry drift below), and it is the argument for the guard `scripts/check-advisory-register.sh`
supplies in plan 09-06 — not a footnote to be corrected and forgotten, but the concrete case for why
a mechanically-enforced register exists at all.

**Phase 8's clap v4 migration removed four suppressions' parent crates from the dependency graph
entirely.** `ADR-0023`'s `structopt` → `clap` v4 migration deleted `structopt = "0.3"` from the root
manifest. Four `deny.toml` `unmaintained`-class entries justify themselves as "via structopt" or
"via clap 2.x/structopt": `RUSTSEC-2022-0104` (`structopt`), `RUSTSEC-2021-0139` (`ansi_term`),
`RUSTSEC-2024-0375` (`atty`), `RUSTSEC-2024-0370` (`proc-macro-error`). Re-verified in this session
(see `## Code Locations` for the verbatim transcript), all four crates now return **0** hits in
`Cargo.lock`. None of the four is reachable under any feature combination today, and none was
reachable at the moment `deny.toml` last named them without anyone recording that the ground had
moved.

**Milestone 10 Epic 2's own governing PRD sets a schema without an owner or an expiry field.** Its
FR-3 four-field exception schema asks for advisory ID, affected crate, why present, why not yet
fixable, and revisit condition — nothing else. Every `.cargo/audit.toml` entry already satisfies
all four. Only `rustsec-remediation-plan.md` adds owner and expiry, and only for the original two
advisories it names. ROADMAP criterion 2 for this phase requires owner, expiry date, affected
scope, and compensating control on **every** suppressed advisory — a strictly stronger bar than the
PRD that is supposed to govern this area. Under this corpus's precedence order (ADR → shipped tree
→ codebase map → code-verification → PRD → DOC → checkbox), a criterion stricter than its own
governing PRD is a supersession, and a supersession has to be written down here rather than
silently assumed by whichever plan happens to act on the stricter reading first.

## Decision

1. **`.cargo/audit.toml` is the authoritative suppression surface** for `cargo audit`. `deny.toml`
   mirrors its `vulnerability`-class entries exactly and adds one clearly-labelled `unmaintained`
   class of its own advisories that `cargo audit` does not gate on but `cargo deny` does.
   **`SECURITY-EXCEPTIONS.md`** (repository root, authored by this phase's plan 09-02 alongside this
   ADR) is the authoritative *governance* register — the one place an owner, a review date, an
   affected scope, and a compensating control live for every suppression, structured as TOML rather
   than comment prose so a script can read it.

2. **The register's schema is the union of M10 Epic 2 FR-3's four fields with owner, review date,
   affected scope, and compensating control, and this union explicitly supersedes FR-3's schema.**
   FR-3's four fields (advisory ID, affected crate/path, why present, why not yet fixable, revisit
   condition) remain in the register verbatim, lifted from `.cargo/audit.toml`'s existing comment
   block rather than re-derived. The four added fields are what ROADMAP criterion 2 demands and
   FR-3 does not: `owner`, `review_date`, `scope`, `compensating_control`.

3. **The three 2026 vulnerability advisories — `RUSTSEC-2026-0187`, `-0194`, `-0195` — are
   ratified, not removed.** No ingested document authorises them; M10 Epic 2 FR-3 §5 names exactly
   two. That is an authorisation gap, not a technical defect: all three are real, all three are
   DoS-class with no clean upstream fix, and all three already carry documented no-fix reasoning in
   both TOML files. Removing them would turn CI red on advisories nobody can currently fix, for no
   security benefit. Ratifying them here converts an unauthorised three-advisory expansion of the
   suppression set into an authorised one, which is the outcome this phase's governing requirement
   (SEC-01, absorbing SUPPLY-02's ratify-or-remove clause) calls for. Each carries a compensating
   control naming the actual reachable input path in `SECURITY-EXCEPTIONS.md` — for `-0187`, that
   Paladin does not feed attacker-controlled PDF input to `lopdf` in a default build; for `-0194`
   and `-0195`, that the `s3` feature's XML input is S3 API response bodies from a configured
   endpoint, not third-party documents.

4. **The owner of every suppression becomes `DF3NDR`, the repository owner, replacing "Platform
   Security (Milestone 7)".** A team label attached to a milestone that closed in 2026 has no
   present-day accountable party to hold to a review. `DF3NDR` is the only named human approver
   anywhere in the 263-document corpus (`license-compatibility-decision-checklist.md`, approved
   2026-05-28), and is the accountable party this project actually has.

5. **The 2026-09-30 acceptance is renewed, not closed, with per-advisory review dates of
   2026-12-31.** Closing it would require an upstream fix, and none exists for either original
   advisory (`rsa` via `sqlx-mysql`, `tokio-tar` via `testcontainers`, both confirmed dev/test-scoped
   this session). A single blanket expiry date is what produced today's state, where one date
   governs two of the ten live advisories and the other eight carry none at all. Every row in
   `SECURITY-EXCEPTIONS.md` gets its own `review_date`, all initially set to the same
   **2026-12-31** so the first review is one coordinated event, diverging thereafter as each
   advisory's upstream situation moves independently.

## Considered Options

- **Make `deny.toml` the authoritative suppression surface instead of `.cargo/audit.toml`**
  (rejected) — `deny.toml` holds a strict superset (the `unmaintained` class), which `cargo audit`
  cannot consume at all; making the superset authoritative would require `cargo audit` to read a
  file whose format it does not understand.
- **Generate both TOML files from one source document** (rejected) — a build step nobody currently
  maintains, added to solve the synchronisation of roughly forty total live lines across two files.
  The cost of the tooling exceeds the size of the problem it solves.
- **Richer inline TOML comments instead of a separate register** (rejected) — the four governance
  fields ROADMAP criterion 2 demands cannot be structured data inside `[advisories] ignore`, which
  is an array of bare strings. A comment above the array is not queryable by a script and cannot be
  gated on; `SECURITY-EXCEPTIONS.md`'s TOML payload is a real parse target, a comment is not.
- **Keep the four Phase-8-orphaned entries "just in case" a dependency returns** (rejected) — a
  suppression that suppresses nothing is not documented risk, it is stale bookkeeping; attaching
  an owner and a review date to it is governance theatre that dilutes the register's meaning for
  the ten entries that are actually live. If `structopt` or its siblings return to the dependency
  graph, restoring the four-line `deny.toml` entries from git history is trivial.
- **Remove the three 2026 vulnerability advisories instead of ratifying them** (rejected) — all
  three are real DoS-class findings with no clean upstream fix available today; removing the
  suppression would fail every CI run on an advisory nobody can act on, with no corresponding
  security improvement, since the underlying code path is unchanged either way.
- **A single blanket renewal date for all ten suppressions** (rejected) — this is the mechanism
  that produced the current defect: one date (2026-09-30) governing two of fifteen nominal entries,
  and none of the rest. Per-advisory dates that start aligned and diverge as each upstream moves
  independently is what a review actually needs to track.

## Code Locations

- `SECURITY-EXCEPTIONS.md` — the new authoritative governance register this ADR ratifies, authored
  in this same phase's plan 09-02, ten rows (five `vulnerability`, five `unmaintained`), all eleven
  fields non-empty on every row.
- `.cargo/audit.toml:28-34` — the five-entry `[advisories] ignore` array this ADR declares
  authoritative for `cargo audit`; unchanged in substance by this ADR (the comment/pointer rewrite
  pointing at the register is plan 09-06's work, not this ADR's).
- `deny.toml:112-146` — the fourteen-entry `[advisories] ignore` array in three labelled blocks
  (2 mirrored vulnerability, 9 unmaintained, 3 new-2026 vulnerability); four of the nine
  `unmaintained` entries are dead per the transcript below and are deleted by plan 09-06.
- `.github/workflows/ci.yml:61-78` — `security-audit:` job, display name `Security Audit`, the
  surviving bare `cargo audit` invocation that reads `.cargo/audit.toml`'s full five-entry array.
- `.github/workflows/ci.yml:465-482` — `security:` job, also displaying as `Security Audit`, running
  `cargo audit --ignore RUSTSEC-2023-0071 --ignore RUSTSEC-2025-0111` — the duplicate this ADR's
  Code Conformance instructs plan 09-06 to delete.
- `.github/rulesets/protect-main-branch.json:34-44` — the committed branch-protection ruleset,
  requiring the status-check **context string** `"Security Audit"`, not a job ID. Both jobs above
  post a check run under that identical context string, so deleting `security:` at `:465-482`
  removes zero required-status-check coverage; the surviving `security-audit:` job continues to
  satisfy the ruleset alone.
- `crates/paladin-content/Cargo.toml:18,41` — `pdf = []` (an empty feature gating nothing) and
  `pdf-extract = { version = "0.7" }` (unconditional, not `optional = true`). This is the tree
  evidence that warrants `RUSTSEC-2026-0187`'s suppression regardless of how Phase 10 / HARD-06
  answers whether PDF extraction is a supported capability.
- `.project/Milestone_7-Production-Hardening/Epic_4/rustsec-remediation-plan.md:39-40` — the formal
  risk acceptance this ADR renews: owner "Platform Security (Milestone 7)", review/expiry target
  2026-09-30, superseded by decisions 4 and 5 above.

**Verbatim liveness transcript, run in this session against the current `Cargo.lock`:**

```
$ grep -c '^name = "rsa"$' Cargo.lock
1
$ grep -c '^name = "tokio-tar"$' Cargo.lock
1
$ grep -c '^name = "lopdf"$' Cargo.lock
1
$ grep -c '^name = "quick-xml"$' Cargo.lock
2
$ grep -c '^name = "dotenv"$' Cargo.lock
1
$ grep -c '^name = "fxhash"$' Cargo.lock
1
$ grep -c '^name = "number_prefix"$' Cargo.lock
1
$ grep -c '^name = "rustls-pemfile"$' Cargo.lock
1
$ grep -c '^name = "paste"$' Cargo.lock
1
$ grep -c '^name = "structopt"$' Cargo.lock
0
$ grep -c '^name = "ansi_term"$' Cargo.lock
0
$ grep -c '^name = "atty"$' Cargo.lock
0
$ grep -c '^name = "proc-macro-error"$' Cargo.lock
0
$ grep -c '^name = "gcc"$' Cargo.lock
0
```

All ten currently-registered suppressions (five vulnerability, five unmaintained) return at least
one hit; `structopt`, `ansi_term`, `atty`, `proc-macro-error` and the already-removed `gcc` all
return zero. The live suppression surface is confirmed at exactly ten entries, not the corpus's
remembered fifteen and not `deny.toml`'s own fourteen.

## Code Conformance

must change

Plan 09-06 (wave 2 of this phase) executes the code consequences of this decision: it deletes the
four dead `deny.toml` entries confirmed above (`structopt`, `ansi_term`, `atty`,
`proc-macro-error`), rewrites both `.cargo/audit.toml`'s and `deny.toml`'s comment blocks to point
at `SECURITY-EXCEPTIONS.md` as the governance source of record, deletes the duplicate
`.github/workflows/ci.yml:465-482` `security:` job (the 18-line block: comment at `:465` through
the `run:` line at `:482`), and lands `scripts/check-advisory-register.sh` wired into CI to enforce
this ADR's invariant mechanically rather than by header comment. Plan 09-07 (the phase close-out)
records the closure evidence in `REQUIREMENTS.md`, flips the SEC-01 checkbox behind that evidence,
and hands Phase 12 the closure notes for SUPPLY-01 and SUPPLY-02, which this phase's execution
absorbs.

## Downstream Consumers

- **Phase 12 / SUPPLY-01 and SUPPLY-02** — both requirements are absorbed by this phase's execution
  rather than left as open work. SUPPLY-01's `ci.yml:389-406`-framed deletion (line numbers stale;
  re-derived here as `:465-482`) is performed by plan 09-06. SUPPLY-02's three clauses — ratify or
  remove the three 2026 vulnerability ignores, extend the FR-3 schema with owner and expiry, and
  backfill the remaining suppressions — are satisfied by decisions 2, 3 and 5 above. Phase 12
  inherits these as closed items with this ADR's number and plan 09-06's commit as evidence, not as
  work to re-plan.
- **Phase 10 / HARD-06** — receives the `crates/paladin-content/Cargo.toml:18,41` finding as an
  input to its own open question ("is PDF extraction a supported capability?"). This ADR explicitly
  does not answer that question; it only establishes that `lopdf` is reachable in the graph today,
  which is sufficient to warrant `RUSTSEC-2026-0187`'s suppression independent of how HARD-06
  resolves.
- **The 2026-12-31 review event** — the date every row in `SECURITY-EXCEPTIONS.md` now carries.
  Whichever phase or process is active at that point must re-evaluate each of the ten rows against
  its `revisit_condition` and either close, renew, or escalate it; this ADR does not itself name an
  owning phase for that future review, since no phase in the current milestone extends past
  2026-12-31.
