# Phase 12: Supply-Chain Gate Integrity - Context

**Gathered:** 2026-08-09
**Status:** Ready for planning
**Mode:** `--auto` — all seven gray areas were auto-selected and auto-resolved to their recommended
option. Each decision below carries the reasoning that produced it; **none was confirmed by a
human.** Three decisions are flagged `⚠ HUMAN REVIEW` — one that corrects a governing requirement's
own "this requirement does not act" clause, one that adds a CI guard the requirement did not ask
for, and one inherited risk that this entire phase's scope rests on. Read those first if you read
nothing else.

**Read this second.** Two premises this phase was written against are now false, and both were
re-measured this session:

1. **`cargo audit` and `cargo deny check` both run and both exit `0` in this environment.** Phases 9
   and 10 each recorded the verification as CI-only because `crates.io` returned HTTP 403. That
   caveat no longer holds — the tools are installed and the advisory DB fetches. Phase 12's
   inherited "verify SUPPLY-01/SUPPLY-02" obligation can be discharged **in-repo**, not deferred
   (D-09).
2. **Only one of SUPPLY-03's "two supply-chain ADR candidates" is still open.** The other —
   `Milestone_7/Epic_4/rustsec-remediation-plan.md`, the corpus's only dated item — was **already
   promoted on 2026-08-08 by Phase 9 as ADR-0024** and is recorded as closed in `PROMOTION.md:185-189`.
   SUPPLY-03's live scope is **one** candidate (D-02).

<domain>
## Phase Boundary

Make the supply-chain gates this project runs on every push reach **one verdict**, and give the
last open supply-chain ADR candidate a deliberate promote-or-decline decision. Three requirements,
SUPPLY-01 … SUPPLY-03 — but **only SUPPLY-03 is open work.**

**Three deliverable classes:**

1. **A discharged verification of SUPPLY-01 and SUPPLY-02** — both were executed by Phase 9 (plans
   09-02 and 09-06; commits `a587e5a`, `7ee741c`, `6513cb7`, `9cef391`, `cb75b2b`) and are inherited
   as **closed items to verify, not work to re-plan**. The verification is now runnable locally
   (D-09) plus one CI observation that has never had the chance to fire (D-10).
2. **One ADR promoting the audit-suppression single-source invariant** (SUPPLY-03) — PROMOTION.md
   Part B candidate 7, `Milestone_10/Epic_2/prd-dependency-security-license-compliance.md` FR-1 + §8,
   whose **Owner phase is explicitly Phase 12**. Promoted with a `conforms` verdict, plus a
   regression guard that makes it a gate rather than an observation (D-05, D-06, D-11).
3. **Three stale source corrections** (per D-00c) — SUPPLY-03's own "this requirement does not act"
   clause, PROJECT.md's matching *Out of Scope* bullet, and the `ci.yml:389-406` line citation that
   three documents still carry.

**Not in this phase:**

- **Re-litigating the suppression set.** ADR-0024 owns which advisories are suppressed, their
  owners, review dates, schema and compensating controls. Phase 12 does not add, remove or re-date
  a single entry (D-00i).
- **Re-deriving `pdf-extract` reachability.** ADR-0032 settled it; the Phase 10 hand-off says
  inherit it as an answer (D-00j).
- **The `scraper` / `rss` / `tiktoken-rs` dead-dependency finding.** The Phase 10 hand-off names
  **Phase 15** as owner of record. `ROADMAP.md:714`'s phrasing reads ambiguously; it is not this
  phase's (D-00k).
- **Promoting any of the other nine ADR candidates.** Candidates 4, 6 (Phase 10), 8, 9 (Phase 13),
  10 (Phase 14) and 11 (Phase 15) keep their owning phases.
- **Building the Milestone 9-12 ledger.** That is ORCH-01 / Phase 13's deliverable (D-12).
- **Applying the committed GitHub rulesets to the live repository.** A finding, recorded and handed
  off, not an action (D-13).
- **Closing milestone v0.7.2 or reconciling the ROADMAP Milestones table.** STATE.md flags this as
  a live boundary discrepancy; it is a `/gsd-complete-milestone` decision, not phase work.
- **Fixing the `API Surface Tracking` CI job.** It is the only failing job in the last run and it is
  DEBT-01's, not SUPPLY's.

</domain>

<decisions>
## Implementation Decisions

### Inherited from Phases 1, 5, 7, 8, 9, 10 and 11 — locked, not re-litigated

- **D-00a:** ADRs live in `.planning/decisions/`, flat zero-padded monotonic numbering, file shape
  `Status / Context / Decision / Considered Options / Code Locations / Code Conformance /
  Downstream Consumers`, **no frontmatter**. `## Code Locations` and `## Considered Options` are
  **bulleted lists, never prose** — `adr-parser.cjs`'s `splitEntries` yields nothing from a
  paragraph. `PROMOTION.md:59` records **0036 as next free** — verified this session.
  *(Phase 1 D-01/D-03; PROMOTION.md §Numbering index, §Required heading set.)*
- **D-00b:** Precedence order is **ADR → shipped tree → `.planning/codebase/` map →
  `intel/code-verification.md` → PRD → DOC → task-list checkbox.** An ADR that contradicts shipped
  code is an instruction to change the code. *(Phase 1 D-02.)*
- **D-00c:** Source corrections under `.project/` and `.planning/` are **annotation, not
  rewriting** — a dated correction banner naming what was wrong and pointing at the ADR or
  requirement, each defective claim corrected inline with the original text retained and marked
  superseded. *(Phase 5 D-08.)*
- **D-00d:** Ledgers are **amended in place**, dated, superseded text retained. Never a separate
  corrections file. *(Phase 2 D-02.)*
- **D-00e:** Evidence bar (the "D-19 bar"): no claim of closure without the exact command or
  `file:line` that produced it, **recorded verbatim**. *(Phases 3, 5, 7, 8, 9, 10, 11.)*
- **D-00f:** Primary key is the `REQ-*` / requirement ID; outstanding task items nest under the
  requirement they belong to rather than getting invented identifiers. *(Phase 1 D-18.)*
- **D-00g:** Contested positions get ADRs; code-settled defects get ledger rows and no ADR.
  *(Phase 7 D-17, applied by Phases 8, 9, 10 and 11.)*
- **D-00h [informational]:** Medieval-military ubiquitous language is mandatory in code, docs and
  comments; conventional commits; no `unwrap`/`expect`/`panic!` in library code.
  *(CLAUDE.md — standing project-wide convention.)*

**Inherited from Phases 9 and 10, specific to this phase:**

- **D-00i:** **ADR-0024 owns the exception set and its governance.** The five `.cargo/audit.toml`
  vulnerability ignores, the ten `deny.toml` entries, the eleven-field register schema, owner
  `DF3NDR`, the per-advisory `2026-12-31` review dates and the ratification of `RUSTSEC-2026-0187`
  / `-0194` / `-0195` with named compensating controls are all **settled**. Phase 12 does not add,
  remove, re-date or re-justify a single suppression. *(Phase 9 D-06, D-08, D-09, D-10.)*
- **D-00j:** **ADR-0032 settled `pdf-extract` reachability** — it is an unconditional dependency of
  `paladin-content` (`crates/paladin-content/Cargo.toml:40`), gated one level up by the facade's
  optional `paladin-content` dependency (`Cargo.toml:59`). `.cargo/audit.toml:26-29` already carries
  the corrected wording. **Inherit as an answer, not a question.** *(Phase 10 hand-off, item 3.)*
- **D-00k:** **The `scraper` / `rss` / `tiktoken-rs` dead-dependency finding belongs to Phase 15.**
  The Phase 10 hand-off assigns it explicitly ("Phase 15 is the named owner of record").
  `ROADMAP.md:714` phrases it as "Phase 12 (SUPPLY-02/03, including the … dead-dependency finding
  named to Phase 15)", which a planner can misread as Phase 12 scope. It is not.
  *(Phase 10 hand-off, item 4.)*

⚠ **D-00l [inheritance risk — flagged, not blocking]: this entire phase's scope rests on a Phase 9
decision that no human confirmed.** Phase 9 ran `--auto`; `09-CONTEXT.md:4-8` states plainly that
"**none was confirmed by a human**" and flags four decisions `⚠ HUMAN REVIEW` — one of which,
**D-07, is the re-scoping of Phase 12 itself** ("⚠ HUMAN REVIEW — this changes another phase's
scope"). Phase 9's `09-VERIFICATION.md` scores 5/5 and does not record D-07 as ratified. Phase 12
proceeds on it because the re-scope is now **recorded at source in the governing document** —
`ROADMAP.md` §Phase 12 carries dated per-criterion "Satisfied by Phase 9, dated 2026-08-08 (plan
09-07)" notes plus a dated closure note, and `REQUIREMENTS.md:4085` carries the matching amendment —
so the scope narrowing is durable and auditable rather than merely asserted in a prior phase's
context file. **A planner must not treat D-07 as human-ratified.** If a human disagrees, the
consequence is that SUPPLY-01/SUPPLY-02 return to Phase 12 as work rather than verification — the
ADR-promotion decisions below are unaffected either way.

---

### Gray area 1 — Does SUPPLY-03 act, or only record? (the promotion-viability contradiction)

- **D-01: SUPPLY-03 acts. It writes an ADR. The "does not act" clause is stale and gets corrected at source.** ⚠ **HUMAN REVIEW — this overrides a governing requirement's own explicit self-limitation.**
  Three documents say promotion is impossible from inside a planning artefact:
  `REQUIREMENTS.md:1937-1939` ("**This requirement does not act.** Promotion requires re-tagging the
  source documents and re-running ingest, which is a user-owned step outside any planning artefact;
  entering a lock here would fabricate authority the corpus does not contain"), `REQUIREMENTS.md:102-110`,
  and `PROJECT.md` §*Out of Scope* ("**Promoting the two ADR candidates into locked decisions** —
  doing so requires re-tagging the source documents via `--manifest` and re-running ingest, not an
  edit here").
  **All three are superseded by `PROMOTION.md` §Part A**, authored by Phase 1: *"Before this phase,
  promoting a candidate required re-tagging its source document via `--manifest` and re-running the
  ingest classifier — and the ingest is closed … That path no longer exists. It is not needed
  either: ADRs now live in `.planning/decisions/` as their own document class, independent of the
  ingest manifest, and top the precedence order (D-01, D-02). Promotion is now an ordinary write to
  a directory plus a table row."*
  **And the practice already settles it, four times over.** `PROMOTION.md` Part B records candidates
  1, 2, 3 and 5 as *Closed* by ADR-0016, ADR-0021, ADR-0024 and ADR-0025 — promoted by Phases 7 and
  9 as ordinary writes, with no re-ingest. Under D-00b, shipped practice plus the higher-precedence
  ADR-class document outrank the PRD/DOC-tier requirement text.
  **Done when** ADR-0036 exists **and** all three stale passages carry dated correction banners per
  D-00c with the original text retained.
  Chosen over "record a recommendation without acting", which would leave the candidate open with
  its owning phase spent and force a future reader to re-derive this same contradiction.
  — **Reversibility:** costly — an ADR promoted into `.planning/decisions/` tops the precedence
  order and any later phase that disagrees must supersede it through the `## Status` /
  `## Supersedes` mechanism rather than delete it; the correction banners on three source documents
  also have to be re-superseded rather than reverted.

### Gray area 2 — How many candidates are actually live?

- **D-02: SUPPLY-03's live scope is ONE candidate, not two. Record the correction; do not re-promote ADR-0024's subject.**
  SUPPLY-03 names "the two supply-chain ADR candidates". Verified this session against
  `PROMOTION.md:185-189`: candidate 3, `Milestone_7/Epic_4/rustsec-remediation-plan.md` — the
  corpus's only dated item, review/expiry target **2026-09-30** — is annotated **"Owner phase:
  Phase 9. Closed 2026-08-08 by ADR-0024 (`0024-rustsec-exception-governance.md`) — renewed to
  per-advisory `2026-12-31` review dates, owner reassigned to `DF3NDR`."** Candidate 7,
  `Milestone_10/Epic_2/prd-dependency-security-license-compliance.md` FR-1 + §8, carries
  **"Owner phase: Phase 12"** and no closure note.
  `REQUIREMENTS.md:4085` already says the same thing in prose ("what remains open for Phase 12 to
  actually plan is **SUPPLY-03** alone"), but does not say that SUPPLY-03 itself has halved.
  **Done when** SUPPLY-03's text carries a dated banner recording that one of its two subjects was
  discharged by ADR-0024 and naming the surviving one.
  Chosen over writing a second ADR that restates ADR-0024's subject — which `REQUIREMENTS.md:1801`
  already prohibits by precedent ("writing a second, competing version ADR … is prohibited") and
  which would put two live ADRs on one question, breaking `PROMOTION.md` §Supersession's
  "exactly one live ADR answers each question at any time".

### Gray area 3 — Promote or decline candidate 7?

- **D-03: Promote. The invariant becomes ADR-0036, with a `conforms` verdict.**
  The invariant is *exceptions live only in `audit.toml` and `deny.toml`; no inline advisory-ignore
  flags in CI*. `PROMOTION.md` Part B candidate 7 recorded it as "currently violated by the tree" —
  **that is no longer true.** Verified this session:
  `grep -c 'run: cargo audit' .github/workflows/ci.yml` → **1**; the only `--ignore` occurrences
  anywhere in `.github/workflows/` are `mc mb --ignore-existing` (`ci.yml:428-429`) and
  `cargo test -- --ignored` (`ci.yml:463,466,755,757`), none of them advisory suppressions.
  Promoting an invariant the tree already satisfies is the cheapest possible moment to lock it — the
  ADR ratifies a true state rather than mandating a change, which is exactly the shape ADR-0031 set
  the precedent for (restate rather than instruct where the tree conforms).
  Declining would leave the invariant at PRD precedence, where SUPPLY-03's own text says "any future
  document can override them", one phase after the project paid for the violation.
  **Done when** ADR-0036 exists with `## Code Conformance` = `conforms`, `## Code Locations` citing
  `ci.yml:62-78`, `ci.yml:101`, `ci.yml:118`, `.cargo/audit.toml`, `deny.toml` and
  `SECURITY-EXCEPTIONS.md`, and `## Downstream Consumers` naming Phase 13 / ORCH-01 (the ledger that
  must carry the "100% complete with one false acceptance criterion" verdict class) and Phase 15 /
  PIPE-01.

- **D-04: One ADR, not two, and it is numbered 0036.**
  `PROMOTION.md:59` — `Next free ADR number: 0036` — verified this session; `.planning/decisions/`
  holds `0001`…`0035` plus `PROMOTION.md`. The phase advances the line by one. Per D-00g only the
  contested position earns an ADR: the single-source invariant is contested (it sat at PRD
  precedence while the tree violated it). The SUPPLY-01/SUPPLY-02 verification results are
  **code-settled** and get requirement rows plus a hand-off, not an ADR.

### Gray area 4 — How ADR-0036 relates to ADR-0024

- **D-05: ADR-0036 stands alone and cites ADR-0024 as a related decision. It does NOT supersede it, and ADR-0024's `## Status` is not touched.**
  They answer different questions. **ADR-0024 governs the *contents* of the exception set** — which
  advisories, whose, reviewed when, with what compensating control, under what eleven-field schema.
  **ADR-0036 governs the *topology*** — that exactly two mechanical surfaces may carry a
  suppression, and that CI may not carry one inline. A reader asking "who owns
  `RUSTSEC-2026-0187`?" goes to ADR-0024; a reader asking "may I add `--ignore` to a workflow step?"
  goes to ADR-0036. Neither answer changes the other.
  Chosen over folding the invariant into ADR-0024 as an amendment, which would make one ADR answer
  two questions and blur the supersession mechanism, and over marking ADR-0024 superseded, which is
  simply false — it is live and its review dates are in force.

### Gray area 5 — The evidence bar for SUPPLY-01 / SUPPLY-02 verification

- **D-06: Verify locally. The CI-only caveat is dead — all three gates run and pass in this environment.**
  Phase 9 recorded "neither tool is installable in Phase 9's sandboxed environment — `crates.io`
  returns HTTP 403" and Phase 10 restated it unchanged. **Re-measured this session, all three pass:**
  - `cargo audit` → exit `0`; *"Loaded 1190 security advisories"*, *"Scanning Cargo.lock for
    vulnerabilities (677 crate dependencies)"*, `warning: 8 allowed warnings found`, zero
    vulnerabilities.
  - `cargo deny check` → exit `0`; `advisories ok, bans ok, licenses ok, sources ok`.
  - `./scripts/check-advisory-register.sh` → exit `0`;
    `✅ 10 register row(s) checked against 10 deny.toml and 5 .cargo/audit.toml ignore entries; all clauses satisfied.`
  Per D-00e every one of those transcripts is recorded **verbatim** in the phase's artefacts. The
  planner must re-run them at execution time rather than quoting this file — a context file is not
  evidence, the command output is.
  **Done when** SUPPLY-01 and SUPPLY-02 each carry a dated closure row citing the command, its exit
  status and its output, and the three documents that still say the check is CI-only
  (`REQUIREMENTS.md` §Phase-10 hand-off item 3, the ROADMAP dated closure note, `09-CONTEXT.md`'s
  environment caveat) carry dated banners per D-00c recording that the blocker lifted.

- **D-07: The one clause that genuinely cannot be closed in-repo is the CI-run observation — record it as pending with a named trigger, do not fake it.**
  SUPPLY-01's inherited clause is "confirming the required status check still resolves on the first
  real CI run after the deletion". Verified this session: **the most recent `ci.yml` run is
  `30861568499`, dated 2026-08-03** — five days *before* Phase 9's 2026-08-08 deletion. **No CI run
  has happened since the change.** The clause has not failed; it has never had the opportunity to
  fire. Record it as *pending — trigger: the next push to `release/v0.7.0`*, with the run ID that
  establishes the boundary. Also record that in that last pre-deletion run the **only** failing job
  was `API Surface Tracking` (DEBT-01's, not SUPPLY's) — the `Security Audit` job passed.
  Chosen over blocking the phase on a CI run, which would make a documentation phase wait on a push
  it does not own, and over silently claiming the clause closed.

### Gray area 6 — Does the invariant get a regression guard?

- **D-08: Yes — add a minimal offline guard that fails if an advisory-ignore flag reappears in any workflow. This is what turns the ADR from an observation into a gate.** ⚠ **HUMAN REVIEW — this adds a CI check no requirement explicitly asks for.**
  SUPPLY-03's own text states the intent: *"Promoting them together … would turn the run-5
  supply-chain finding from an **observation** into a **gate**."* An ADR alone is prose; prose did
  not stop the duplicate job the first time. Today's enforcement has a hole: `check-advisory-register.sh`
  asserts register ↔ `deny.toml` ↔ `.cargo/audit.toml` ↔ `Cargo.lock` agreement (three clauses,
  verified this session), but **nothing anywhere asserts that a workflow file carries no inline
  advisory suppression** — the exact defect SUPPLY-01 existed to fix could be reintroduced tomorrow
  and every gate would stay green.
  Scope is deliberately minimal: extend `scripts/check-advisory-register.sh` with a fourth clause,
  or add a sibling guard invoked from the same `make check-gates` target and the same `ci.yml:101`
  step. It must (a) be offline — the whole `check-gates` family is, and that is why it is runnable
  here; (b) match `cargo audit`/`cargo deny` invocations carrying an advisory-ignore flag, **not**
  bare `--ignore`, so `mc mb --ignore-existing` and `cargo test -- --ignored` do not false-positive;
  (c) assert `cargo audit` appears exactly once across `.github/workflows/`; (d) report every
  violation rather than stopping at the first, matching the existing script's stated contract.
  Chosen over ADR-only (prose with no enforcement, which is the state that produced the defect) and
  over a full CI-policy linter (scope creep — a new capability with its own phase).
  — **Reversibility:** reversible — a bash guard and one `make` wiring line; deleting it restores
  today's behaviour exactly.

### Gray area 7 — Where closure is recorded, and the rulesets finding

- **D-09: SUPPLY closure rows go in `REQUIREMENTS.md` plus a hand-off block to Phase 13. This phase does NOT create `.planning/ledgers/milestone-09-12.md`.**
  `.planning/ledgers/` holds `milestone-01`, `-02-03`, `-04-06`, `-07-08` — there is no `-09-12`,
  and building it is **ORCH-01's** stated deliverable ("the *Milestone 9-12 as-shipped ledger* below
  is upgraded from component-level file evidence to per-criterion verdicts", 120 requirement IDs).
  Creating a stub here would either be re-planned by Phase 13 or would silently constrain its shape.
  Phase 12 writes its evidence where every prior phase wrote pre-ledger evidence: the requirement
  rows themselves, plus a dated `#### Hand-off to Phase 13 / ORCH-01` block in the same style as the
  three hand-off blocks already in `REQUIREMENTS.md`.
  **The hand-off must carry ORCH-01's named verdict class explicitly:** Milestone 10 is recorded
  100% complete, ships every artefact it promised, **and failed one of its own acceptance criteria**
  — and, as of Phase 9, no longer does. ORCH-01 is required to carry that verdict class into the
  ledger; Phase 12 is the phase that can date its closure.

- **D-10: The committed-but-unapplied GitHub rulesets are a recorded finding with an owner, not work.**
  Verified this session: `.github/rulesets/` exists in the tree (Milestone 10 Epic 5's "committed
  GitHub rulesets for the main branch and release tags", recorded in PROJECT.md as shipped), but
  `gh api repos/:owner/:repo/rulesets` returns **empty** and
  `gh api repos/:owner/:repo/branches/main/protection` returns **404 "Branch not protected"**. The
  ruleset JSON is version-controlled and **not applied to the live repository** — so "the required
  status check" in SUPPLY-01's clause has no live enforcement point on `main` at all.
  This is genuinely adjacent to the phase goal ("the gates this project runs on every push give one
  verdict") but it is repository-administration state, not a gate configuration this phase owns, and
  applying a ruleset is an outward-facing change to a live repository. **Record the finding with the
  exact commands and their output per D-00e, name it for the milestone-close-out decision, and do
  not apply anything.** Flag it for the human, since only the repository owner can act on it.

### Claude's Discretion

- **Whether the D-08 guard is a fourth clause inside `scripts/check-advisory-register.sh` or a
  separate sibling script.** The constraints in D-08 (offline, precise matching, exhaustive
  reporting, wired into `make check-gates` and `ci.yml:101`) are fixed; the file layout is the
  planner's call. A separate script keeps the register script's single responsibility; a fourth
  clause keeps one invocation site. Either satisfies D-08.
- **Whether ADR-0036's `## Considered Options` reproduces the decline branch as a rejected option or
  only names it.** `PROMOTION.md` §Required heading set demands the section be a bulleted list;
  depth is the planner's.
- **Plan decomposition and wave assignment.** The verification work (D-06, D-07), the ADR promotion
  (D-01…D-05) and the source corrections (D-01, D-02, D-06) are mutually independent and can run in
  parallel. The D-08 guard should land in the same plan as, or after, ADR-0036 so the ADR can cite
  it in `## Code Locations`. `PROMOTION.md:59` must be updated **last**, per its own procedure at
  `PROMOTION.md:151-163`.
- **Exact wording and placement of the dated correction banners**, subject to D-00c (original text
  retained, marked superseded, pointing at ADR-0036 or the requirement).

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### This phase's requirements and goal
- `.planning/ROADMAP.md` §`### Phase 12: Supply-Chain Gate Integrity` (line 755 onward) — goal, five
  success criteria, and the **dated closure note** recording that criteria 1-4 are satisfied by
  Phase 9 and criterion 5 is half-satisfied. **This is the document that makes the re-scope
  durable — read it before acting on D-00l.**
- `.planning/REQUIREMENTS.md:1833-1941` — SUPPLY-01, SUPPLY-02 and SUPPLY-03 full text.
  **SUPPLY-03's "This requirement does not act" clause is at `:1937-1939` and is corrected by D-01.**
- `.planning/REQUIREMENTS.md:1790-1812` — the **Phase 10 hand-off to SUPPLY-02/SUPPLY-03**.
  Inherited as D-00j and D-00k. Do not re-derive.
- `.planning/REQUIREMENTS.md:4085` — the SEC-01 → SUPPLY-01/02 coupling row, amended by Phase 9 to
  record the discharge.
- `.planning/REQUIREMENTS.md:100-110` — the eleven-candidate / zero-locked framing, including the
  stale re-ingest claim corrected by D-01.

### The ADR machinery this phase uses
- `.planning/decisions/PROMOTION.md` — **the single most important ref for this phase.**
  §Numbering index line 59 (`Next free ADR number: 0036`); §Required heading set (lines 107-128, and
  the `adr-parser.cjs` bullet-list constraint); §Supersession mechanism (lines 130-140);
  **§Part A lines 165-170 — the passage that supersedes SUPPLY-03's "does not act" clause (D-01)**;
  **§Part B candidate 7 at lines 203-205 — "Owner phase: Phase 12"**; §Part B candidate 3 at lines
  185-189 — **already closed by ADR-0024 (D-02)**.
- `.planning/decisions/0024-rustsec-exception-governance.md` — owns the exception set, the schema,
  owner `DF3NDR`, the `2026-12-31` review dates and the three-advisory ratification. **Cited by
  ADR-0036, not superseded by it (D-05).**
- `.planning/decisions/0032-pdf-extraction-capability.md` — `pdf-extract` reachability, inherited as
  an answer (D-00j).
- `.planning/decisions/0031-extracted-crate-dependency-rule.md` — the closest **shape** model for
  ADR-0036: an ADR that *restates* an invariant the tree already satisfies rather than instructing a
  change. ⚠ Authored under Phase 10 `--auto`, never human-ratified.
- `.planning/decisions/0005-herald-trait.md` — the worked example `PROMOTION.md` §Part A step 2
  points at for file shape.

### The source document being promoted
- `.project/Milestone_10-CI-Hardening-Release-Automation/Epic_2/prd-dependency-security-license-compliance.md`
  **FR-1 + §8** — the audit-suppression single-source invariant. **Path resolved this session by
  `find`** (`PROMOTION.md` records only the `Milestone_10/Epic_2/…` short form). Cite the full path
  in ADR-0036's `## Code Locations` per `PROMOTION.md` §Part A step 4, and give it a dated banner
  per D-00c.
- `.project/Milestone_7-Production-Hardening/Epic_4/rustsec-remediation-plan.md` — candidate 3, the
  2026-09-30 dated item. Path resolved this session. **Already promoted as ADR-0024. Do not
  re-promote (D-02).**

### The gates themselves — all re-measured 2026-08-09
- `.github/workflows/ci.yml:60-78` — the surviving `security-audit:` job, display name
  `Security Audit`, bare `cargo audit` at `:78`, with the "no inline `--ignore`" rationale comment
  at `:76`.
- `.github/workflows/ci.yml:101` — `run: ./scripts/check-advisory-register.sh`. **The wiring point
  for D-08's guard.**
- `.github/workflows/ci.yml:118` — `run: cargo deny check`.
- `scripts/check-advisory-register.sh` — 10,811 bytes, executable. Its header comment (lines 1-42)
  states the three clauses, the report-everything contract, the class-discovery-by-set-equality rule
  and the never-scrape-comment-prose rule. **D-08's fourth clause must respect all four.**
- `SECURITY-EXCEPTIONS.md` (repo root) — the authoritative governance register, 10 rows, eleven
  fields each.
- `.cargo/audit.toml` — 5 vulnerability ignores; `deny.toml:118-129` — 5 vulnerability + 5
  unmaintained.
- `Makefile:167-172` — `check-advisory-register` and the `check-gates` aggregate target.

### Prior-phase context and precedent
- `.planning/phases/09-release-security-gate-integrity/09-CONTEXT.md` — **D-07 at line 188 is the
  re-scoping of this phase, flagged `⚠ HUMAN REVIEW` and never ratified (D-00l).** D-01…D-10 are the
  suppression-set decisions inherited as D-00i. Its header (lines 4-8) states the `--auto`,
  none-human-confirmed status.
- `.planning/phases/11-facade-residue-deferred-register-disposition/11-CONTEXT.md` — the D-00a…D-00h
  convention block inherited verbatim, and the D-00m precedent for how to carry an unratified
  `--auto` inheritance forward.
- `.planning/codebase/CONCERNS.md:257-296` — the supply-chain risk entries, **already amended by
  Phase 9 (plan 09-07)** with the corrected counts. Do not re-correct; verify the amendment is
  still accurate and leave it.
- `.planning/PROJECT.md` §*Out of Scope* — the "Promoting the two ADR candidates … requires
  re-tagging via `--manifest`" bullet. **Corrected by D-01.**
- `CLAUDE.md` + `.github/instructions/rust.instructions.md` — conventional commits, ubiquitous
  language, no `unwrap`/`expect`/`panic!` in library code.

### Forward coupling
- **Phase 13 / ORCH-01** — receives the D-09 hand-off block, including the "100% complete with one
  false acceptance criterion, now closed" verdict class and the dated SUPPLY closure rows. Also the
  named `## Downstream Consumers` of ADR-0036.
- **Phase 15 / PIPE-01** — owns the `scraper`/`rss`/`tiktoken-rs` dead-dependency finding (D-00k)
  and is a `## Downstream Consumers` entry on ADR-0036, since new CI jobs must not reintroduce
  inline suppressions.
- **Milestone close-out** — receives the D-10 rulesets finding and the STATE.md milestone-boundary
  discrepancy.

</canonical_refs>

<code_context>
## Existing Code Insights

### Verified ground truth (re-measured this session, 2026-08-09)

- **All three supply-chain gates pass locally.** `cargo audit` → exit `0`, 1190 advisories loaded,
  677 crate dependencies scanned, `warning: 8 allowed warnings found`, zero vulnerabilities.
  `cargo deny check` → exit `0`, `advisories ok, bans ok, licenses ok, sources ok`.
  `./scripts/check-advisory-register.sh` → exit `0`,
  `✅ 10 register row(s) checked against 10 deny.toml and 5 .cargo/audit.toml ignore entries; all clauses satisfied.`
  **`cargo-audit` and `cargo-deny` are both on `PATH`.** This directly contradicts the environment
  caveat Phases 9 and 10 both recorded.
- **SUPPLY-01's structural criteria hold.** `grep -c 'run: cargo audit' .github/workflows/ci.yml` →
  **1**. The single `Security Audit` display name is at `ci.yml:62`, its job body at `:60-78`. Zero
  advisory-ignore flags anywhere in `.github/workflows/`; the only `--ignore*` tokens are
  `mc mb --ignore-existing` (`:428-429`) and `cargo test -- --ignored` (`:463`, `:466`, `:755`, `:757`).
- **SUPPLY-02's register is live and complete.** `SECURITY-EXCEPTIONS.md` holds 10 rows, matched
  exactly against `deny.toml`'s 10 ignores and `.cargo/audit.toml`'s 5 — asserted mechanically, not
  by eye.
- **No CI run exists since the Phase 9 deletion.** Most recent `ci.yml` run: `30861568499`,
  `2026-08-03T23:14:24Z`, branch `release/v0.7.0`, conclusion **failure**. Its only failing job is
  `API Surface Tracking` (DEBT-01). The deletion landed 2026-08-08. (D-07.)
- **The committed rulesets are not applied.** `.github/rulesets/` exists in-tree;
  `gh api repos/:owner/:repo/rulesets` → empty; `gh api …/branches/main/protection` → 404
  `Branch not protected`. (D-10.)
- **ADR series state.** `.planning/decisions/` holds `0001`…`0035` plus `PROMOTION.md`; next free
  **0036** per `PROMOTION.md:59`.
- **`.cargo/audit.toml:26-29` already carries ADR-0032's corrected `RUSTSEC-2026-0187` wording** —
  the Phase 10 hand-off's item 1 is visibly discharged in the file.

### Reusable Assets

- **`scripts/check-advisory-register.sh`** — the extension point for D-08's guard. Already offline,
  already invoked from both `Makefile:169` and `ci.yml:101`, already structured to report every
  failure rather than stopping at the first. Adding a fourth clause needs no new wiring.
- **`make check-gates`** (`Makefile:172`) — the aggregate offline release-gate target
  (`check-changelogs check-crate-names check-advisory-register`). A sibling guard added here is
  picked up by both local and CI paths for free.
- **The Phase 9 / Phase 10 / Phase 11 correction-banner pattern** — dated banner, what was wrong,
  pointer to the ADR or requirement, original text retained verbatim. Three separate phases have
  now applied it; copy the shape rather than inventing one.
- **ADR-0031 and ADR-0024** — the two closest ADR models. ADR-0031 for an invariant the tree already
  satisfies (`conforms`); ADR-0024 for a supply-chain subject with a live register behind it.

### Established Patterns

- **`.planning/`-only phases are the norm here.** Phases 10 and 11 each changed **zero** `.rs`
  files. Phase 12 is expected to touch `.planning/`, `.project/` banners, one `scripts/` guard and
  possibly `Makefile`/`ci.yml` wiring — **no `crates/` or `src/` change is anticipated.** If a
  planner concludes a Rust change is needed, that is a signal to re-check D-03, not to widen scope.
- **`PROMOTION.md` is updated last** and the phase writes a dated note under §Numbering index
  explaining how far the line advanced and why — Phases 9, 10 and 11 each did.
- **Evidence is a transcript, not a claim** (D-00e). Every prior phase records the literal command
  and its literal output.

### Integration Points

- `PROMOTION.md:59` (`Next free ADR number`) + §Part B candidate 7 (add a "Closed by ADR-0036" note
  in the same shape as candidates 1, 2, 3 and 5) + §Numbering index (add the 0036 row).
- `.planning/PROJECT.md` §Key Decisions table — `PROMOTION.md` §Part A step 6 requires a row per
  promoted ADR. Also §*Out of Scope*, corrected by D-01.
- `.planning/REQUIREMENTS.md` — SUPPLY-01/02/03 rows, the traceability table at `:4008-4010`
  (all three currently `Pending`), and a new dated hand-off block for Phase 13 / ORCH-01.
- `.planning/ROADMAP.md` §Phase 12 — criterion 5's ADR-promotion half closes here.
- `scripts/check-advisory-register.sh` + `Makefile:167-172` + `.github/workflows/ci.yml:101` — D-08.

</code_context>

<specifics>
## Specific Ideas

- **"An observation into a gate."** SUPPLY-03's own phrase, and the acceptance test for this phase.
  An ADR that no script enforces is an observation with better formatting. D-08 exists because the
  requirement says the word *gate* and means it.
- **"The cheapest phase in this roadmap."** The ROADMAP's own framing, and it is now cheaper than
  when written — SUPPLY-01's eighteen-line deletion and all three SUPPLY-02 clauses were paid for by
  Phase 9. A planner who produces a large plan set here has misread the closure note.
- **Three documents say this requirement cannot act; a fourth says it can, and four ADRs prove it.**
  Worth stating plainly wherever D-01 is recorded. The corpus's rule that "nothing is locked" was
  itself superseded by Phase 1 building `.planning/decisions/`, and the requirement text never caught
  up. This is a fact about the corpus, not just about SUPPLY-03.
- **The gate that verifies the gates was itself unverifiable until today.** Phases 9 and 10 both
  wrote "CI-only — `crates.io` returns HTTP 403" and both were right at the time. Recording *when
  the blocker lifted* matters as much as recording that it did, so the next reader does not treat
  the caveat as permanent.
- **Milestone 10 is the corpus's only "100% complete and one acceptance criterion false" verdict.**
  Phase 12 is the phase that gets to date the second half of that sentence — *and no longer is, as
  of 2026-08-08*. ORCH-01 must carry both halves.

</specifics>

<deferred>
## Deferred Ideas

- **Applying the committed GitHub rulesets to the live repository** — `.github/rulesets/` is
  version-controlled but not in force on `main`, which is why SUPPLY-01's "required status check"
  clause has no enforcement point. Repository-administration action, owner-only. Recorded with
  evidence (D-10); handed to the milestone close-out.
- **Closing milestone v0.7.2 and reconciling the ROADMAP `## Milestones` table** — STATE.md flags
  the discrepancy (7/7 phases complete against a table that still scopes v0.7.2 to phases 5-6 and
  marks phases 7-11's blocks "Not started"). Phase 12 opens the Milestone 9-12 block, so this is a
  boundary either way. `/gsd-complete-milestone` decision, not phase work.
- **Fixing the `API Surface Tracking` CI job** — the only failing job in the most recent run.
  DEBT-01's, and Phase 8's.
- **The `scraper` / `rss` / `tiktoken-rs` dead optional dependencies** — declared in
  `crates/paladin-content/Cargo.toml`, consumed by no code, confirmed twice by Phase 10. Named to
  **Phase 15** by the Phase 10 hand-off (D-00k). Not re-opened here.
- **Human ratification of Phase 9's D-07, D-09 and D-16** — the `--auto` decisions that re-scoped
  this phase, reassigned the security-exception owner, and deleted a PRD-mandated Dockerfile block.
  Flagged as D-00l. Not blocking, because the re-scope is recorded at source in the ROADMAP.
- **Promoting the other nine ADR candidates** — candidates 4 and 6 (Phase 10, still open), 8 and 9
  (Phase 13), 10 (Phase 14), 11 (Phase 15). Each keeps its owning phase per `PROMOTION.md` Part B.
- **A general CI-policy linter** — D-08 deliberately builds one narrow guard. A broader "workflows
  may not do X" checker is a new capability and belongs in its own phase.

</deferred>

---

*Phase: 12-supply-chain-gate-integrity*
*Context gathered: 2026-08-09*
</content>
</invoke>
