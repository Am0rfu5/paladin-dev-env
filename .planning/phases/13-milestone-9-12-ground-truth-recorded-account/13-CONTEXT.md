# Phase 13: Milestone 9-12 Ground Truth & Recorded Account - Context

**Gathered:** 2026-08-10
**Status:** Ready for planning
**Mode:** `--auto` — every gray area was auto-resolved to its recommended option. Each decision below
carries the reasoning that produced it; **none was confirmed by a human.** Two decisions are flagged
`⚠ HUMAN REVIEW` — the `AgentProvisioner` placement that the queue/worker and sidecar topologies are
built against (**D-14**), and recording the absence of Garrison and Arsenal on HTTP-served agents as a
property of the topology rather than as planned scope (**D-15**). Read those first if you read
nothing else.

**Ten gray areas were identified and all ten were auto-selected and resolved:** ledger home, shape and
evidence bar (ORCH-01) · the new verdict class and its second half (ORCH-01) · the five checkbox
verdicts and where the five-run pattern lives (ORCH-02) · the agent route surface (ORCH-03a) · the
four stale paths (ORCH-03 b-e) · `AgentProvisioner` placement (ORCH-04a) · Garrison and Arsenal for
HTTP-served agents (ORCH-04b) · the version trajectory and the numbering prediction (ORCH-05) · this
phase's code-change boundary (cross-cutting) · ADR allocation and plan decomposition.

**Five findings this session change the shape of the phase.** They are numbered in `<specifics>` and
each is verified against the tree on 2026-08-10. In summary: ORCH-01's "remaining 104" arithmetic
counts two different populations (specifics 1); ORCH-03(a) is already answered by the committed
`openapi.json`, and the live defect it should have named is in a shipped mdbook page, not in the four
Epics it does name (specifics 2); ORCH-04(a)'s framing rests on an Epic §7 claim that omits the one
type which decides the question (specifics 3); ORCH-04(b) is a documented contradiction, not an
under-surfaced non-goal (specifics 4); and ORCH-05's second half was already discharged by Phase 10's
ADR-0030 (specifics 5).

<domain>
## Phase Boundary

Make `.planning/` a cited, truthful account of the four milestones that finished, hardened,
documented and exposed this framework — Milestone 9 (classic orchestrator completion), Milestone 10
(CI hardening and release automation), Milestone 11 (documentation overhaul and publish) and
Milestone 12 (Web API), plus the `Deferred-QA-CICD-Completion` register and the `project-management`
master plan — and give the two seams Milestone 12 left as *defaults* exactly one recorded decision
each. Five requirements, ORCH-01 … ORCH-05.

**Four deliverable classes:**

1. **A cited status ledger** (ORCH-01) — `.planning/ledgers/milestone-09-12.md`, the **fifth and final
   sibling** in a series all four existing ledgers already name by filename, with a `file:line`-cited
   verdict for all **120** run-5 requirement IDs across 24 sections. This is the largest ledger in the
   corpus and the one with the most shipped rows. Two classes must be unmissable:
   **`Shipped, one acceptance criterion false`** (Milestone 10 — the corpus's only instance, and both
   halves of it), and **`Verified open`** (the Deferred-QA Epics 25-27 rows, the highest-confidence
   forward-work signal in the corpus and the direct input to Phases 14-16).
2. **Three new ADRs** (ORCH-03a, ORCH-04a, ORCH-04b) — **0037-0039**. ORCH-01 and ORCH-02 get no ADR;
   a ledger and a set of checkbox verdicts are not contested positions (D-00g). **ORCH-05 gets no ADR
   either** — it appends to ADR-0029 and cites ADR-0030 (D-16, D-17).
3. **In-repo source corrections under `.project/`** (ORCH-03) — dated correction banners plus inline
   annotation, superseded text retained, per D-00c.
4. **A narrow, named documentation surface under `docs/src/`** — three files, all in
   `deployment-topologies/`. **No `.rs` file is touched.** See D-18 for the boundary rule and why it
   is not zero.

**Not in this phase:**

- **Executing the `AgentProvisioner` placement decision.** D-14 records the placement and the reason;
  the trait stays where it is, so there is nothing mechanical to execute. If a human overturns D-14,
  the move is `.rs` work across two published crates and belongs to Phase 14, not here.
- **Wiring Garrison or Arsenal into HTTP-served agents.** D-15 records the topology's shape; building
  the capability is feature work no milestone has scheduled, and inventing a target here would be
  scope creep into an unplanned milestone.
- **WEB-01 … WEB-04 (Phase 14).** ORCH-01's ledger rows *record* the JWT-vs-opaque contradiction, the
  multi-replica token problem and the `ProviderCapabilities` over-report as `Verified open` with
  citations. It does not resolve any of them. ORCH-04(b) explicitly couples to WEB-04's
  "two tool surfaces would need a stated relationship" clause and hands it forward.
- **PIPE-01 … (Phase 15) and DOCS-01 … (Phase 16).** ORCH-02's verdict on Milestone 11's 26 open items
  is *"the only genuinely open count in run 5, settleable only by content"* — it is carried to
  DOCS-01, not settled here. D-08's corrected `ci.yml` job list is handed to PIPE-01.
- **Re-opening REL-01.** Already `[x] Complete` (Phase 4, ADR-0008), and Phase 10's hand-off says so in
  terms. ORCH-05 applies the converged result (D-16).
- **Any `.rs` source change.**

</domain>

<decisions>
## Implementation Decisions

### Inherited from Phases 1, 5, 7, 8, 9, 10 and 12 — locked, not re-litigated

- **D-00a:** ADRs live in `.planning/decisions/`, flat sequential numbering, file shape
  `Status / Context / Decision / Considered Options / Code Locations / Code Conformance /
  Downstream Consumers`, **no frontmatter** (matching 0001-0036, not the `adr-parser.cjs` schema).
  **`PROMOTION.md:60` records 0037 as next free** — verified this session.
  *(Phase 1 D-01/D-03, Phase 7 D-00a/D-00h, Phase 9 D-00a, Phase 10 D-00a)*
- **D-00b:** Precedence order is **ADR → shipped tree → `.planning/codebase/` map →
  `intel/code-verification.md` → PRD → DOC → task-list checkbox.** An ADR that contradicts shipped
  code is an instruction to change the code. *(Phase 1 D-02)*
- **D-00c:** Source corrections under `.project/` are **annotation, not rewriting** — a dated
  correction banner naming what was wrong and pointing at the ADR or requirement, each defective
  claim corrected inline with the original text retained and marked superseded. *(Phase 5 D-08)*
- **D-00d:** Ledgers are **amended in place**, dated, superseded text retained. Never a separate
  corrections file. *(Phase 2 D-02)*
- **D-00e:** Evidence bar: no claim of closure without the exact command or `file:line` that
  produced it, recorded verbatim. *(Phases 3, 5, 7, 8, 9, 10, 12)*
- **D-00f:** Primary key is the `REQ-*` ID; outstanding task items nest under the requirement they
  belong to rather than getting invented identifiers. Two requirements citing the same artefact keep
  separate rows and separate verdicts. *(Phase 1 D-18, Phase 7 D-00e, Phase 10 D-00f)*
- **D-00g:** Contested positions get ADRs; code-settled defects get ledger rows and no ADR.
  *(Phase 7 D-17, applied by Phases 8, 9, 10 and 12.)*
- **D-00h [informational]:** Medieval-military ubiquitous language is mandatory in code, docs and
  comments. *(CLAUDE.md — a standing project-wide convention.)*
- **D-00i:** Provenance of `--auto` decisions is carried forward rather than laundered. Where a
  decision that changed scope was made under `--auto` and never ratified by a human, the ledger
  records **the closure and its provenance**, not a bare `Complete`. *(Phase 12 hand-off item 6.)*

---

### ORCH-01 — the ledger: home, vocabulary, evidence bar, and the arithmetic

- **D-01: New file `.planning/ledgers/milestone-09-12.md`; REQUIREMENTS.md's section becomes a pointer.**
  Not a judgement call — a commitment made four times. `milestone-01.md:5`, `milestone-02-03.md:5`,
  `milestone-04-06.md:5` and `milestone-07-08.md:5` each name `milestone-09-12.md` as the sibling that
  completes the series, and `REQUIREMENTS.md:3552`, `:3565` and `:3581` repeat it. The
  `## Milestone 9-12 as-shipped ledger` section runs **3607-3931** (325 lines, 120 rows across 24
  epic sections) and is reduced to a pointer by the scaffold plan, exactly as Phase 7's D-26 did for
  Milestone 4-6 and Phase 10's D-01 did for Milestone 7-8.
  **Do not leave the REQUIREMENTS.md section in place as a second, diverging copy.**
  **Phase 12 deliberately left this file uncreated** — not even a stub — so that its 120-row scope
  would not be silently constrained (`REQUIREMENTS.md:2109-2116`, Phase 12's D-09). `ls
  .planning/ledgers/` returns exactly four files, verified this session. This phase creates the fifth.

- **D-02:** **The ledger's vocabulary is the run-5 status key that is already written, not the series'
  seven classes — with one class retired.**
  `REQUIREMENTS.md:3634-3637` already carries an eleven-class key that extends the run-4 key:
  `Shipped` · `Shipped (relocated)` · `Shipped, superseded` · **`Shipped, one acceptance criterion
  false`** · **`Superseded by outcome`** = do not plan as written · **`Verified open`** = confirmed
  absent from the tree · `Verify` → ORCH-01 · `Variant` · `Contract diverges` · `Open defect → X` ·
  `Provenance only`. All 120 rows are already written against it. Re-keying them onto the earlier
  seven-class vocabulary is churn with no reader benefit; instead **map the eleven onto the series'
  seven in the head note**, as Phase 10's D-02 mapped four onto seven.
  **`Verify` is retired.** It is not a verdict — it is a marker meaning *"ORCH-01 owes one here"*.
  Measured this session, **35 of the 120 rows are a bare `Verify`** and every one must become a real
  verdict. A ledger that ships with any `Verify` row has not done ORCH-01's job.
  **Two classes get dedicated head-note treatment**, for the same reason Phase 10 gave
  `Superseded by outcome` a summary table — a planner must not have to scan 120 rows to find them:
  `Shipped, one acceptance criterion false` (one row, both halves, D-05) and `Verified open` (the
  Deferred-QA rows, which are the direct input to Phases 14, 15 and 16).

- **D-03: Phase 10's evidence bar carries over verbatim, manifest carve-out included.**
  No row gets a `Shipped` verdict without a `file:line` citation **plus** something that exercises it.
  Milestones 10 and 12 are structural/infrastructural in large part, so a manifest line, workflow job
  or `Makefile` target plus a named consumer is the exercising artefact (Phase 7 D-01). Behavioural
  requirements still need a test, example or command.
  **The 53 bare `Shipped` rows are re-derived, not carried.** An ingest-era status word is exactly the
  "the code exists" claim the bar exists to reject. Run 5 verified 37 claims directly and every one
  held — those 37 are the cheapest rows in the ledger, but the citation is re-run, because Phase 10's
  single most productive move was re-reading `file:line` references that had gone stale.

- **D-04:** **ORCH-01's "sixteen already have it, the remaining 104 need the same treatment" counts two
  different populations. Correct the arithmetic at source.**
  ⚠ **Fresh finding.** ORCH-01 (`REQUIREMENTS.md:2210-2212`) says *"Sixteen entries already carry
  `settled-by` pointers into `intel/code-verification.md` run 5; those are facts about the tree, not
  decisions, and the remaining 104 need the same treatment."* Verified this session: the sixteen
  `settled-by` entries are **variant-register entries**, not ledger rows — `intel/SYNTHESIS.md:546`
  ("Sixteen entries carry `- settled-by:` lines") sits under the variants section, and
  `SYNTHESIS.md:335` defines the mechanism as *"where the shipped tree settles a **variant**, the
  entry carries a `- settled-by:` line"*. `grep -c "settled-by" .planning/REQUIREMENTS.md` → **10**,
  none of them inside the ledger region; `grep "settled-by"` across rows 3607-3931 → **0**.
  So `120 − 16 = 104` subtracts variant-register entries from ledger rows. **All 120 rows need a
  verdict.** The real split, measured this session across `REQUIREMENTS.md:3607-3931`:
  - **35** rows are a bare `Verify` — no verdict at all.
  - **53** rows are a bare `Shipped` — a status word, which D-03 rejects as evidence.
  - **32** rows already carry a richer verdict (`Verified open` ×14, the `Shipped, …` qualified
    variants, `Contract diverges`, `Provenance only`, `Open …`).
  Correct the figure in ORCH-01's own text per D-00d. This is the same class of error as Phase 10's
  D-05 (a "14-row table" holding 13 rows), sitting inside the requirement that exists to retire it —
  and a planner who budgets 104 rows of work will be wrong in both directions at once.

- **D-05: The new verdict class carries both halves, dated, and Phase 12 gets to date the second.**
  Phase 12's hand-off is explicit (`REQUIREMENTS.md:2100-2107`) and this is its stated deliverable:
  Milestone 10 is recorded 100% complete, ships every artefact it promised, **and failed one of its
  own acceptance criteria** — M10 Epic 2 §8's *"`audit.toml` and `deny.toml` are the only places
  policy/exceptions are defined; no inline advisory-ignore flags remain in CI"* — **and, as of
  2026-08-08, no longer does.** Phase 9 made it true (plan 09-06, commit `cb75b2b`, deleting the
  duplicate `security:` job at pre-deletion `ci.yml:465-482`); Phase 12 promoted it to ADR-0036 and put
  `scripts/check-workflow-suppressions.sh` behind it so it stays true.
  Verified independently this session: `grep -n "cargo audit --ignore" .github/workflows/ci.yml`
  returns **nothing** — the deletion held. A row recording only the failure, or only the fix, is wrong.

- **D-06: The three SUPPLY closures are cited with their provenance, not re-verified.**
  Phase 12's hand-off item 1 points at the evidence rather than repeating it: SUPPLY-01
  (`REQUIREMENTS.md:1855-1940`), SUPPLY-02 (`:1941-2046`), SUPPLY-03 (`:2047-2082`), plus ADR-0036 and
  its enforcement wiring at `Makefile:171-176` and `.github/workflows/ci.yml:103-104`. Cite them; do
  not re-open them.
  **Two provenance facts travel with the closure (D-00i):** Phase 9's **D-07** re-scoped Phase 12 from
  work to verification under `--auto`, flagged `⚠ HUMAN REVIEW`, never itself ratified; and Phase 12's
  own **D-01** and **D-08** were resolved only when a human selected `option-a` at plan 12-01's
  blocking checkpoint, dated 2026-08-09. The ledger records the closure **and** how it was obtained.
  **One clause is genuinely pending with a named trigger:** SUPPLY-01's *"confirming the required
  status check still resolves on the first real CI run after the deletion"*
  (`REQUIREMENTS.md:1893-1895`) has never had the opportunity to fire — the newest run against
  `release/v0.7.0` is still `30861568499` (2026-08-03), predating the 2026-08-08 deletion. **Trigger:
  the next push to `release/v0.7.0`.** A row marking it done without a `gh run` citation newer than
  `30861568499` is a false positive.
  **One finding is owner-only:** `.github/rulesets/` is version-controlled but unapplied and `main` is
  unprotected (`gh api repos/:owner/:repo/rulesets` → `[]`). Only the repository owner can change
  that; the ledger records it against the milestone close-out, and this phase applies nothing.

- **D-07: Phase 12's measured stale-citation inventory is inherited, not re-derived.**
  Plan 12-01 measured **87 hits across 25 files** for the stale `ci.yml:389-406` citation, corrected
  **8 sites across four canonical governance documents**, and excluded the rest by a stated
  class-by-class scoping rule (frozen milestone snapshots, prior-phase records, closed ingest outputs,
  a closed ledger row, ADR-0024's self-annotated citation, Phase 9's own correction banners, and Phase
  12's own files). The inventory lives in `12-01-SUMMARY.md` §Grep Inventory. ORCH-01 reads it and
  knows which sites are **deliberately** left as historical record. Re-running the grep and "fixing"
  the excluded classes would undo a deliberate decision.

- **D-08:** **`ci.yml`'s job list that run 5 recorded is stale — measure it once, here, and hand the
  number to Phase 15.**
  ⚠ **Fresh finding.** `intel/code-verification.md:539-540` records *"`ci.yml`'s 14 job ids are
  `lint`, `security-audit`, `cargo-deny`, `osv-scanner`, `api-surface`, `test`, `crate-isolation`,
  `integration-tests`, `security`, `docker`, `e2e-tests`, `benchmark`,
  `benchmark-regression-signal`, `publish-dry-run`"*, and **PIPE-01 quotes that list verbatim**
  (`REQUIREMENTS.md:2434-2436`). Measured this session
  (`grep -nE '^  [a-z][a-z0-9-]*:$' .github/workflows/ci.yml`), the file has **15** jobs:
  `lint`(:21) · `security-audit`(:61) · `cargo-deny`(:81) · `osv-scanner`(:126) · `api-surface`(:155)
  · `test`(:206) · **`examples`(:245)** · `crate-isolation`(:319) · `integration-tests`(:374) ·
  `docker`(:494) · **`kubernetes-smoke`(:611)** · `e2e-tests`(:718) · `benchmark`(:779) ·
  `benchmark-regression-signal`(:812) · `publish-dry-run`(:898).
  `security` is gone (Phase 9, D-05) and two jobs the run-5 list does not name are present. The ledger
  records the measured list with its line numbers; **PIPE-01's copy is corrected at source** so
  Phase 15 does not plan against a 14-job pipeline that no longer exists. Do not attribute the two
  additions without checking `git log` — record what is there.

- **D-09:** **DEBT-01's stale-path defect is half closed, and the four Milestone 12 references stay ledger
  rows rather than becoming a sixth ORCH-03 item.**
  ⚠ **Fresh finding.** Run-5 finding 8 (`intel/code-verification.md:614-620`) records nine references
  to `project/current-exports.txt`, a file that does not exist, and concludes *"`check-api-surface.sh`
  exits 1 with 'No baseline found' when the file is absent, so the `api-surface` CI job fails on every
  run."* Verified this session: **`scripts/check-api-surface.sh:6` reads
  `BASELINE="${1:-.project/current-exports.txt}"`** — the dotted path — and
  `.project/current-exports.txt` **exists** (442 KB). The consequence clause is no longer true.
  What remains true is the *documentation* half: Milestone 12 Epics 1 §7, 5 §7, 6 `cross_refs` and 7
  FR-4.6 still name the undotted path in their requirement text. That is ORCH-03-shaped, but **ORCH-03
  names five specific items and this is not one of them.** Record it in the four affected ledger rows
  with the `check-api-surface.sh:6` citation and hand it to Phase 15 with the D-08 job list. Do not
  grow ORCH-03 to six items.

### ORCH-02 — the five checkbox verdicts and the corpus-level pattern

- **D-10:** **Five verdicts, no ADR, and the five-run pattern written in exactly one place — the ledger's
  head note.**
  ORCH-02 states the five verdicts almost completely already (`REQUIREMENTS.md:2223-2241`); the
  requirement's real content is *where they live* and *that none becomes a task*. All five are
  corroborated against `intel/task-completion-state.md` and `code-verification.md:622-659` this
  session:
  - **M9 — 0 open: corroborated.** Every Epic 1-5 deliverable present.
  - **M10 — 0 open: corroborated in artefacts, contradicted in one acceptance criterion.** D-05's row.
  - **M11 — 26 open: the only genuinely open count in run 5**, and the only one of all 542 across 75
    task lists that survives verification. `tasks-content-rewrite.md` task 6.0 (six user-guide
    updates), task 7.0 (eight deployment/operations updates), task 1.2 (linkcheck review). **All
    fourteen target files exist**; whether their *content* is current is settleable only by reading
    them. **Carried to DOCS-01 (Phase 16), not settled here.**
  - **M12 — 3 open: vacuous.** All three are Task 0.0 scaffolding (create a feature branch, check out
    `feature/m12-epic5-api-security-authorization`, confirm a clean baseline) while the Epic 5 work
    ships as `crates/paladin-web/src/agent_auth.rs`.
  - **project-management — 1 open: nonexistent.** `- [ ] 1.1 Create template → - [x] 1.1 Create
    template (after completing)` — a formatting example inside a template file.
  **The pattern goes in the ledger head note, not in a sixth document.** Across five runs:
  *understated → accurate → overstated → contradicted → vacuous*, and the corpus position is
  `code-verification.md:647-659`'s — **checkbox arithmetic is not a backlog.** Writing it in the
  ledger (rather than a new file) is what stops a sixth rediscovery, because the ledger is what a
  planner opens. No ADR: this is a description of the corpus, not a contested position (D-00g).

### ORCH-03 — the positions the tree contradicts

- **D-11:** **(a) The agent route surface is `/v1`. Confirmed against the committed drift-guard baseline;
  Epic 1-5 route text becomes superseded provenance. ADR-0037.**
  Verified this session — every agent path in `crates/paladin-web/openapi.json` carries the prefix:
  `/v1/agents`, `/v1/agents/{id}`, `/v1/agents/{id}/execute`, `/v1/agents/{id}/execute/stream`,
  `/v1/agents/{id}/jobs`, `/v1/agents/{id}/jobs/{job_id}`. **Epic 6 §4.3 won** ("the agent API is
  served under `/v1`; operational/docs endpoints remain unversioned"), and the four Epics whose
  acceptance criteria, test assertions and examples name unprefixed paths (Epics 1, 3, 4, 5) are
  **superseded provenance, not a live contract** — annotated at source per D-00c, never rewritten.
  This is a contested position across five Epics, so it gets an ADR under D-00g. ORCH-03(a)'s
  instruction to preserve it as a run-5 unsettled position in the *variants register* still holds —
  the ADR records the answer, the register keeps the disagreement.

- **D-12:** **(a′) The one *live* route defect is in shipped documentation, and ORCH-03 does not name it.**
  ⚠ **Fresh finding.** `grep -rn "POST /agents\|GET /agents\|\`/agents" docs/src/ examples/ README.md`
  returns exactly one hit: **`docs/src/deployment-topologies/sidecar.md:29`** — *"the agent runs behind
  the HTTP service host exactly as documented there (`POST /agents/{id}/execute`)"*. Meanwhile
  `docs/src/deployment-topologies/http-service-host.md` is the only file under `docs/src/` that uses
  `/v1/agents`. So the sidecar page tells a reader to call a route the server does not serve.
  This is **not** superseded provenance in a `.project/` PRD — it is a published mdbook page, a live
  contract, and it lands squarely inside ORCH-03's own done-when: *"anyone applying a run-5
  requirement literally cannot write to a path that does not exist."* One-line correction:
  `/agents/{id}/execute` → `/v1/agents/{id}/execute`. ADR-0037 records it as the answer's one
  in-tree consequence.

- **D-13: (b)-(e) All four relocations verified; recorded as relocations, corrected at source, no ADR.**
  Read directly this session — each old path absent, each new path present:
  - **(b)** `src/core/platform/manager/listener_service.rs` — **absent**. Ships as
    `src/application/services/orchestration/listener.rs` (`ListenerOrchestrator`), Milestone 6 Epic 2.
    `REQ-listener-service-test-coverage` corrected.
  - **(c)** `src/application/ports/output/llm_port.rs` — **absent**; the whole `src/application/ports/`
    directory was deleted by Milestone 5 Epic 2. Ships as
    `crates/paladin-ports/src/output/llm_port.rs`. `REQ-llm-tool-calling-port` corrected.
    *(Note for Phase 14: this is the same file WEB-03 and WEB-04 act on.)*
  - **(d)** `docs/Design/Design_and_Architecture.md` — **absent**. Ships as
    `docs/src/appendix/design-and-architecture.md`, Milestone 11 overhaul.
    `REQ-arch-doc-modernization` corrected. **The relocation hid a gap rather than closing one** —
    run-5 finding 4 measured the file at exactly 311 lines, the identical figure the February 2026 PRD
    cites as the *pre-rewrite* state, with zero mermaid blocks and zero mentions of Commander,
    Council, Conclave, Grove, Maneuver, Sanctum or Sentinel. The path correction and the content gap
    are **two different facts** and get two different ledger cells: `Shipped (relocated)` for the
    move, `Verified open` for the rewrite. **The rewrite is DOCS-02's, Phase 16's.**
  - **(e)** `REQ-asciinema-demos` requires README embedding; the README is 193 lines with **zero**
    matches for `asciinema` or `demo`, rewritten by Milestone 11 Epic 5 into a concise landing page.
    `docs/assets/` exists and is empty; `docs/DEMOS.md` does not exist. The clause targets a document
    that changed shape — recorded as such, handed to DOCS-04 (Phase 16), not planned here.
  These are code-settled defects, not contested positions: ledger rows plus D-00c annotations, **no
  ADR** (D-00g).

### ORCH-04 — the two seams recorded as defaults

- **D-14:** **(a) `AgentProvisioner` stays in `paladin-web` — but not for the reason the default gives.
  The reason is that its parameter type is an OpenAPI-annotated HTTP DTO, and that reason survives a
  second consumer. ADR-0038.**
  ⚠ **HUMAN REVIEW — this is the placement the queue/worker and sidecar topologies are built
  against, and it retires a "promote when a second consumer appears" default by denying its premise.**
  The two permitted answers were (a) keep it in `paladin-web`, or (b) promote it to `paladin-ports`
  for reuse. **(a) is recommended, and Epic 1 §7's stated grounds for calling the choice a coin-flip
  are wrong.**
  §7 claims *"either placement is clean since it references `Paladin` + `PaladinExecutorPort`, both
  already in core/ports"*. Verified this session, the trait signature is
  `async fn provision(&self, spec: &AgentSpec) -> Result<ProvisionedAgent, ProvisionError>`
  (`crates/paladin-web/src/agent_registry.rs:103-110`) — it references **four** types, and §7 omits
  the one that decides the question:
  - `crates/paladin-web/src/agent_registry.rs:55-56` — **`AgentSpec` derives
    `utoipa::ToSchema`**, and its `allowed_roles` field carries `#[schema(value_type = Vec<String>)]`.
    Its own doc comment says *"Sent in the body of `POST /agents`."* It is an OpenAPI-annotated HTTP
    request DTO, not a domain type.
  - `crates/paladin-ports/Cargo.toml` has **no `utoipa`** — its dependencies are `paladin_core`,
    `async-trait`, `serde`, `thiserror`, `uuid`, `chrono`, `tokio`, `serde_json`, `futures`, `md5`,
    `mime_guess`. Promoting the trait drags `AgentSpec` and therefore `utoipa` into the ports crate.
  - **ADR-0031 and ADR-0015 both forbid the direction.** ADR-0015(i) states `paladin-core` and
    `paladin-ports` may carry no web framework, and CLAUDE.md's hexagonal rule is that application
    and ports import core only, never infrastructure. Moving an OpenAPI request-body schema into
    `paladin-ports` is the clearest possible violation of both.
  Reasoning, in order of weight:
  1. **A worker or sidecar must not reuse a trait keyed on an HTTP request body.** The right shape for
     a second topology is its own spec type over the same `PaladinBuilder` path — which is exactly
     what already ships: `docs/src/deployment-topologies/queue-worker.md:55` states *"each worker is
     itself an embedded agent host"*, and `queue-worker.md:25-26` describes the worker running the
     agent through a `PaladinExecutionService` directly. **No second consumer of this trait is
     pending**, and the topology docs never mention provisioning from a spec — verified: `grep -in
     "provision\|AgentSpec"` over `queue-worker.md` and `sidecar.md` returns **zero matches**.
  2. **The default's own escape clause would have fired on a false signal.** A second implementation
     already exists — `FacadeProvisioner` at `src/infrastructure/web/facade_provisioner.rs:70`, in the
     root `paladin-ai` crate. It is **`#[cfg(feature = "web-server")]`-gated**
     (`src/infrastructure/web/mod.rs:2`, `src/infrastructure/mod.rs:48`) and `Cargo.toml:276` defines
     `web-server = ["dep:paladin-web", "dep:axum"]`. So the "second consumer" is the composition root
     wiring the HTTP host — not a new topology. Promoting on that signal would have moved an HTTP DTO
     into ports to serve a consumer that only exists when the HTTP stack is compiled in.
  3. **What the placement actually costs is nothing today, and the ADR must say what it would cost
     later.** The concrete price of (a) is that any future non-HTTP consumer needing spec-driven
     provisioning takes `paladin-web` + `axum`, or writes its own trait. The ADR records that the
     second is correct and the first is not, so a later reader does not re-derive the question.
  ADR-0038 names **Phase 14** in `Downstream Consumers` — it is the phase already opening both
  `crates/paladin-ports/src/output/llm_port.rs` (WEB-03/04) and `crates/paladin-web/src/agent_auth.rs`
  (WEB-01), and the one that would execute a move if a human overturns this.
  **Fallback if a human overturns D-14:** promotion is not a one-line move. It requires splitting
  `AgentSpec` into a domain spec in `paladin-ports` and an HTTP DTO in `paladin-web` with a `From`
  conversion, moving `ProvisionError` and `ProvisionedAgent`, and leaving a deprecated re-export at
  `crates/paladin-web/src/lib.rs:46`. That is architecture work across two published crates, outside
  this phase's boundary (D-18), and belongs in Phase 14.
  — **Reversibility:** costly — the trait and its four types are public API on two published crates;
  reversing means the split described above plus a deprecation cycle.

- **D-15:** **(b) HTTP-served agents have no Garrison and no Arsenal. Record it as a property of the
  shipped topology, state it in the decision matrix, and correct the page that currently promises the
  opposite. ADR-0039.**
  ⚠ **HUMAN REVIEW — this writes a capability limitation into published user documentation and
  declines to schedule the capability.**
  ORCH-04(b) permits two answers: planned scope with a target, or a permanent property of the
  topology stated explicitly in the decision matrix. **The second is recommended**, and the situation
  is worse than ORCH-04(b) describes.
  ⚠ **Fresh finding.** ORCH-04(b) frames this as under-surfaced — *"one line in a non-goal is not
  enough surface"*. Verified this session, the decision matrix does not merely omit it; the HTTP page
  **advertises the opposite**:
  - `docs/src/deployment-topologies/http-service-host.md:54` — the sequence diagram reads
    `Service->>Agent: run (LLM + tools + memory)`. Tools and memory are Arsenal and Garrison.
  - `docs/src/deployment-topologies/overview.md` — the five-topology comparison table has `Use when`
    and `Avoid when` columns and says nothing about it; `grep -in "garrison\|arsenal"` across the
    whole `deployment-topologies/` directory hits **only** `embedded-library.md:31-32`, where the
    embedded topology correctly advertises *"memory (Garrison), and tools (Arsenal)"*.
  So a reader routed to the HTTP host by M11 Epic 6 FR-8's *"single source of routing"* is shown a
  diagram promising the capability the embedded page also promises — and gets neither.
  Why "property of the topology" rather than "planned scope with a target": `AgentSpec`
  (`agent_registry.rs:55-79`) has no fields for memory or tools, and adding them is genuine API design
  (how an MCP server, its credentials and its lifetime are expressed in a JSON request body). M12
  excluded it deliberately, twice — Epic 2's non-goal and Epic 3's restatement. **No milestone has
  scheduled it**, and inventing a target here would commit an unplanned milestone. The honest, useful
  answer is the routing one, and the docs already support it: a consumer needing Garrison or Arsenal
  uses the embedded-library topology, and `queue-worker.md:55` already says each worker *is* an
  embedded host — so the routing story is coherent without new work.
  Three deliverables: ADR-0039 records the position and its reasoning; `overview.md`'s comparison
  table and `http-service-host.md` state the limitation where a reader choosing a topology will see
  it; and `http-service-host.md:54` is corrected so the diagram stops promising tools and memory.
  ADR-0039 names **WEB-04 (Phase 14)** in `Downstream Consumers` — WEB-04's own text says Arsenal/MCP
  and LLM tool calling *"would need a stated relationship"*, and this ADR is half of it.
  — **Reversibility:** costly — reversing means retracting a published capability statement in the
  mdbook and re-opening a seam two Epics closed.

### ORCH-05 — the version trajectory and the numbering prediction

- **D-16:** **Append four rows to ADR-0029's `## Trajectory` table. No second version ADR. REL-01 is not
  re-opened.**
  Phase 10's hand-off is explicit (`REQUIREMENTS.md:2177-2202`) and ADR-0029 names Phase 13 / ORCH-05
  in its own `## Downstream Consumers`, verified this session. Append **`v0.3.0` (M9), `v0.4.0` (M10),
  `v0.5.0` (M11), `v0.6.0` (M12)** in ascending order, without re-sorting or re-keying the existing
  rows. Each is a lockstep bump: the milestone's finalization Epic bumps the root crate and every
  workspace member together and cuts a tag (`REQ-lockstep-versioning`).
  **Writing a rival version ADR is prohibited** — HARD-03 (ADR-0029) covers rc.1 → v0.2.0, ORCH-05
  extends the same table through v0.6.0, REL-01 (ADR-0008, Phase 4, done) covers the landing at
  v0.7.0. REL-01 is `[x]` at `REQUIREMENTS.md:360` with a `Phase 4 | Complete` traceability row;
  ORCH-05 **applies** the converged result.

- **D-17:** **ORCH-05's second half is already discharged by ADR-0030. Cite it; run the confirmation; do
  not re-decide.**
  ⚠ **Fresh finding.** ORCH-05's done-when asks that *"the prediction is recorded closed — or, if a
  fifth collision is found on closer reading, it is corrected at source"*. Read this session,
  `.planning/decisions/0030-milestone-7-self-numbering.md:79-84` **already records it**: *"The Roadmap
  Extension Protocol's predicted fifth instance is closed with this fourth instance… run 5 found none,
  and ORCH-05 records the prediction closed. This ADR records the Roadmap Extension Protocol item
  discharged, so no later phase inherits a standing prediction to check."* Phase 10's D-14 did this
  deliberately. ORCH-05's second half therefore shrinks to: **cite ADR-0030, and run the one check
  ORCH-05 owns** — confirm the run-5 provenance keys resolve directly against directory numbering
  (a grep over the run-5 `REQ-*` provenance against `.project/Milestone_9…12` directory numbers). If
  that check finds a fifth collision, ADR-0030 is **amended in place** per D-00d, not superseded by a
  rival ADR. Recording a closure that another ADR already recorded, in a second place, is how the
  numbering convention starts disagreeing with itself — which is the defect ADR-0030 exists to fix.

- **D-18:** **ORCH-05's own current-state figures are two releases stale — the same defect Phase 10
  corrected in HARD-03, recurring verbatim. Correct at source.**
  ⚠ **Fresh finding.** ORCH-05 states (`REQUIREMENTS.md:2300-2301`) *"That chain terminates exactly
  where the tree is: root `Cargo.toml` at `version = "0.6.0"`, branch `release/v0.7.0`, latest tag
  `v0.5.1`."* Verified this session:
  - `Cargo.toml:34` — **`version = "0.7.0"`** (Phase 4 plan 04-05, commit `c2e20a1`).
  - `git tag --sort=-v:refname | head` — **`v0.7.1`, `v0.7.0`**, then `v0.5.1`, `v0.5.0`, `v0.4.3`.
  - Branch is `release/v0.7.0` — the one accurate clause of the three.
  `intel/code-verification.md:469` carries the same stale figure (*"Workspace at v0.6.0 … root
  `Cargo.toml:34` `version = "0.6.0"`"*), correct as of the 2026-07-30 ingest and superseded since.
  The **historical** facts ORCH-05 records — the four lockstep gates M9→v0.3.0, M10→v0.4.0,
  M11→v0.5.0, M12→v0.6.0 — are unchanged and are what the trajectory rows transcribe. Only the
  *current-state* clause gets the D-00c/D-00d treatment, in ORCH-05's text and in the ROADMAP's
  Phase 13 section. Phase 10's D-11 made exactly this correction to HARD-03 and the same sentence
  regrew one requirement later; the ledger head note should say so once.

### Cross-cutting

- **D-19:** **Phase 13's code-change boundary — record-writing, plus a three-file documentation surface,
  and no `.rs`.**
  Phase 7 was record-only; Phase 9 was config-changing; Phase 10 sat between them with a three-file
  config surface. Phase 13 is the same shape with the surface moved from config to docs, because
  ORCH-03's and ORCH-04's done-whens both require the record and the **published documentation** to
  agree. The complete permitted surface:
  - `docs/src/deployment-topologies/sidecar.md:29` — `POST /agents/{id}/execute` →
    `POST /v1/agents/{id}/execute` (D-12).
  - `docs/src/deployment-topologies/http-service-host.md:54` — the mermaid line promising
    `(LLM + tools + memory)`, corrected, plus the stated limitation (D-15).
  - `docs/src/deployment-topologies/overview.md` — the limitation surfaced in the five-topology
    comparison table where a reader chooses (D-15).
  Everything else this phase touches is under `.planning/` or `.project/`. **No `.rs` file is
  modified** — assert it the way Phase 10 did, with `git diff --name-only <base>..HEAD -- '*.rs' | wc
  -l` → `0` at close-out, so the boundary is checkable rather than merely claimed. Any plan proposing
  a `.rs` change has found new scope and should say so rather than absorb it.
  `docs/book.toml` sets `[output.linkcheck] warning-policy = "error"` with `follow-web-links = false`,
  so the three edits must not break an internal link — none of them touches a link target, but the
  close-out should build the book or state why it could not. Every plan remains subject to the
  CLAUDE.md workspace gate (`cargo test` → `cargo fmt --check` → `cargo clippy -- -D warnings`) and to
  ADR-0006's 84% coverage floor; a phase that changes no `.rs` should not move coverage, and the
  close-out confirms exactly that.

- **D-20: ADR allocation — 0037 through 0039; `PROMOTION.md` advances to 0040.**
  Next free is **0037**, verified this session at `PROMOTION.md:60`.
  - **ADR-0037** — The agent route surface: `/v1` confirmed against `crates/paladin-web/openapi.json`,
    Epics 1/3/4/5 route text as superseded provenance, and the `sidecar.md:29` correction (D-11,
    D-12). Conformance: `must change`, executed in this phase (one doc line).
  - **ADR-0038** — `AgentProvisioner` placement: stays in `paladin-web` because `AgentSpec` is an
    OpenAPI-annotated HTTP DTO and promotion would put `utoipa` in `paladin-ports`, against ADR-0015(i)
    (D-14). Conformance: `conforms`. `Downstream Consumers`: Phase 14.
  - **ADR-0039** — Garrison and Arsenal for HTTP-served agents: a property of the shipped topology,
    stated in the decision matrix, with the `http-service-host.md:54` correction (D-15). Conformance:
    `must change`, executed in this phase (two doc files). `Downstream Consumers`: WEB-04 / Phase 14.
  **ORCH-01, ORCH-02 and ORCH-05 get no ADR.** A ledger, a set of checkbox verdicts and an append to
  an existing trajectory table are not contested positions (D-00g); ORCH-05 amends ADR-0029 and cites
  ADR-0030 (D-16, D-17).

- **D-21: Every closure claim is proved by a command run in this environment and recorded verbatim.**
  The D-00e bar. This phase is well-placed for it: nearly every ORCH-01 verdict is a `grep`/`sed`/`git
  log` read of files in this checkout, and the largest single evidence source (`openapi.json`) is
  committed.
  **Not runnable here** (unchanged from Phases 9, 10 and 12): `cargo audit`, `cargo deny`,
  `cargo llvm-cov` and anything Docker — `crates.io` returns HTTP 403 and `docker` is absent
  (`.planning/phases/04-release-coherence/04-ci-gate-deferrals.md`). `gh` reads worked for Phase 12
  and should be attempted for the SUPPLY-01 trigger check (D-06). `mdbook build` should be
  **attempted** for the three doc edits; if it cannot run, say so and scope the claim to CI rather
  than inferring a pass.

- **D-22: Forward hand-offs are written explicitly, in the shape Phases 9, 10 and 12 used.**
  `REQUIREMENTS.md:2084-2202` holds the two blocks this phase inherits; it owes three. This is the
  **last** ground-truth phase, so these hand-offs are the corpus's final forward-work signal:
  - **Phase 14 / WEB-01 … WEB-04** — the `Verified open` ledger rows for the JWT-vs-opaque
    contradiction (run-5 finding 7), the multi-replica token problem, and
    `ProviderCapabilities`/Epic 27; ADR-0038's placement answer; and ADR-0039 as half of WEB-04's
    "stated relationship" between Arsenal/MCP and LLM tool calling.
  - **Phase 15 / PIPE-01 …** — D-08's measured 15-job `ci.yml` list replacing the stale 14, and D-09's
    `check-api-surface.sh:6` finding (the baseline path defect is closed in the script and open in
    four Milestone 12 requirement texts).
  - **Phase 16 / DOCS-01 … DOCS-04** — ORCH-02's verdict that M11's 26 open items are the corpus's only
    genuine open count and are settleable only by content; D-13(d)'s split of the architecture
    document into a closed relocation and an open 311-line rewrite; D-13(e)'s README/demos finding.

- **D-23 [informational]: Suggested decomposition — ~11 plans, 4 waves.**
  Sized against Phase 10's 11 plans for 86 rows and Phase 7's 13 for 115. Phase 13 has **120 rows**,
  the largest of the five — but 53 are `Shipped` with run-5 evidence already assembled and 32 already
  carry a rich verdict, so the expensive population is the **35 bare `Verify` rows** (D-04).
  - **Wave 1:** ① **Ledger scaffold** — `.planning/ledgers/milestone-09-12.md` with the head notes
    (D-02 vocabulary and its mapping onto the series' seven classes, D-03 evidence bar, D-04's
    corrected 35/53/32 arithmetic, D-10's five-run pattern, D-18's version-figure note), all 120 row
    stubs keyed by `REQ-*`, the two head-note class tables (D-02), and the REQUIREMENTS.md pointer
    (D-01). Plus the ORCH-01/ORCH-05 source corrections from D-04 and D-18.
  - **Wave 2 (fully parallel, blocked on ①) — ledger fan-out by disjoint section range:**
    ② M9 Epics 1-6 (25 IDs) · ③ M10 Epics 1-5 (23 IDs, carrying D-05's both-halves row and D-06's
    SUPPLY provenance) · ④ M11 Epics 1-7 (20 IDs, carrying D-10's "only genuine open count" and
    D-13(d)(e)) · ⑤ M12 Epics 1-7 (34 IDs, the largest, carrying D-09's four stale-path rows) ·
    ⑥ Deferred-QA Epics 25-29 + `project-management` (18 IDs — the `Verified open` block, and the
    highest-value rows in the ledger for Phases 14-16).
  - **Wave 3 (parallel with wave 2 — no file overlap with the ledger):**
    ⑦ **ADR-0037** + the four M12 Epic route-text annotations + the `sidecar.md:29` fix (D-11, D-12).
    ⑧ **ADR-0038 + ADR-0039** + the `http-service-host.md` and `overview.md` edits (D-14, D-15).
    **Gate ⑧ on a blocking `checkpoint:decision` before its first task** — both D-14 and D-15 are
    flagged `⚠ HUMAN REVIEW`, both are rated `costly`, and D-15 writes a capability limitation into
    published documentation.
    ⑨ **ORCH-03 (b)-(e) source annotations** + their four ledger rows (D-13).
  - **Wave 4:** ⑩ **ORCH-05** — the four ADR-0029 trajectory rows, the ADR-0030 citation, and the
    provenance-key confirmation (D-16, D-17). ⑪ **Close-out** — five evidence-backed checkbox flips,
    the five traceability rows at `REQUIREMENTS.md:4251-4255`, `PROMOTION.md` → 0040, PROJECT.md Key
    Decisions rows, the three D-22 hand-off blocks, the ledger close-out amendment, the
    `git diff --name-only … -- '*.rs' | wc -l` → `0` assertion (D-19), and the phase gate.
  Plan-file naming is `13-NN-PLAN.md`.
  **File contention to respect:** `.planning/ledgers/milestone-09-12.md` is written by ① and appended
  by ②-⑥ — give each fan-out plan a disjoint section range and it is append-only per section.
  `REQUIREMENTS.md` is touched by ①(pointer + D-04/D-18 corrections), ⑦ and ⑨(row verdicts) and
  ⑪(checkboxes) — they are already in different waves; keep them serialised.
  `docs/src/deployment-topologies/http-service-host.md` is touched only by ⑧; `sidecar.md` only by ⑦.

### Claude's Discretion

- Whether the two head-note class tables (D-02) sit at the head of the ledger, at the foot, or both.
  The constraint is that a planner must not scan 120 rows to find the `Verified open` block or the
  single `Shipped, one acceptance criterion false` row.
- Whether ADR-0038 and ADR-0039 are two files or one. D-20 recommends two (one question per ADR,
  matching 0001-0036); both must remain separately citable because Phase 14 depends on each by number.
- The exact wording of ADR-0039's limitation statement in `overview.md` — whether it lands in the
  comparison table's `Avoid when` cell, as a note under the table, or both — provided a reader
  choosing a topology cannot miss it.
- Whether the ledger presents run-5's 37 verified-shipped claims inline per row or as a
  cross-reference block. Phases 7 and 10 both left this open.
- Exact banner wording and inline-correction markup for every `.project/` annotation (D-00c fixes the
  pattern, not the prose).
- Whether ORCH-05's provenance-key confirmation (D-17) rides in plan ⑩ or folds into the close-out.
- Whether the ledger's Deferred-QA section is ordered by epic number or by the recommended
  implementation order the register itself names (Epic 25 → 28 → 29 → 26/27).
- Whether D-08's corrected `ci.yml` job list is written into the ledger head note, PIPE-01's text, or
  both. It must reach Phase 15 either way.

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Phase scope and requirements

- `.planning/ROADMAP.md` §"Phase 13: Milestone 9-12 Ground Truth & Recorded Account" (**line 816**) —
  the goal, the independence note, and the **six** success criteria. Criterion 4 ("cannot write to a
  path that does not exist") sets D-12's and D-13's bar; criterion 5 is ORCH-04's two seams.
- `.planning/REQUIREMENTS.md` **lines 2204-2315** — **ORCH-01 … ORCH-05 in full**, with their
  *Derives* provenance. **This is the authoritative statement of scope** and it is much longer than
  the ROADMAP summary. Note that **D-04** (the 16/104 arithmetic) and **D-18** (the stale version
  figures) both correct text inside this block.
- `.planning/REQUIREMENTS.md` **lines 2084-2176** — **Phase 12's hand-off block to ORCH-01**: the
  three SUPPLY closures and where their evidence lives, the both-halves verdict class, why the ledger
  was deliberately not stubbed, the pending SUPPLY-01 trigger, the unapplied rulesets, the `--auto`
  provenance chain, and the 87-hit stale-citation inventory (D-05, D-06, D-07).
- `.planning/REQUIREMENTS.md` **lines 2177-2202** — **Phase 10's hand-off block to ORCH-05**: the
  ADR-0029 table to append to, and REL-01's already-converged status (D-16).
- `.planning/REQUIREMENTS.md` **lines 3607-3931** — **the existing Milestone 9-12 as-shipped ledger**,
  120 rows across 24 sections with its own eleven-class status key at `:3634-3637`. **This is the
  input D-03 upgrades and D-01 replaces with a pointer.**
- `.planning/REQUIREMENTS.md` **lines 4251-4255** — the five ORCH traceability rows, all `Pending`,
  flipped by the close-out.
- `.planning/REQUIREMENTS.md` **line 4333** and **line 4336** — the cross-phase coupling rows:
  HARD-03 → ORCH-05, and ORCH-04(a)'s "cheap now, expensive after a second consumer" note that D-14
  answers.
- `.planning/ROADMAP.md` §"Phase 14: API Contract Truthfulness" (**line 832**), §"Phase 15: Coverage &
  CI Quality Gates" (**line 846**), §"Phase 16: Documentation Currency & the Architecture Gap"
  (**line 862**) — the three phases D-22 hands off to.
- `.planning/REQUIREMENTS.md` **lines 2317-2410** — **WEB-01 … WEB-04**, the requirements ORCH-01's
  `Verified open` rows feed. Read to size the hand-off, not to act on.

### Conventions this phase inherits

- `.planning/decisions/PROMOTION.md` — the numbering index, **next free 0037** (`:60`), and the
  five-step append procedure at `:165-174`. Read before writing ADR-0037…0039; advance to **0040** in
  the close-out. Note `:116` names Phase 13 explicitly as a taker of the next free number.
- `.planning/decisions/0001-battalion-config.md` … `0036-audit-suppression-single-source-topology.md`
  — the ADR file shape. **0037-0039 must match it** (no frontmatter, seven headings, per D-00a).
- `.planning/ledgers/milestone-07-08.md` — **the shape to copy**, and the most recent sibling. Its
  head note at `:5` already names `milestone-09-12.md` as this phase's deliverable (D-01). Read its
  head notes for the evidence bar, the manifest carve-out, and the "an ingest status word is the claim
  the bar rejects" rule; read its `## Phase 10 close-out amendments (2026-08-08)` section for the
  close-out shape ⑪ copies.
- `.planning/ledgers/milestone-01.md`, `milestone-02-03.md`, `milestone-04-06.md` — the first three
  instances; `milestone-01.md` demonstrates the D-00d in-place amendment sections.
- `.planning/phases/10-milestone-7-8-ground-truth-recorded-account/10-CONTEXT.md` — **the closest
  analogue to this phase**, and the source of most of the D-00 series. Its D-01…D-06 are this phase's
  D-01…D-04 one ledger later; its **D-11** made the identical version-figure correction that D-18
  makes again; its **D-23** is the boundary rule D-19 restates; its **D-27** is the decomposition
  D-23 mirrors.
- `.planning/phases/12-supply-chain-gate-integrity/12-CONTEXT.md` — the immediately prior phase.
  Source of the `⚠ HUMAN REVIEW` provenance convention D-00i formalises, and of D-01/D-08, the two
  unratified `--auto` decisions D-06 carries forward.
- `.planning/phases/09-release-security-gate-integrity/09-CONTEXT.md` — source of D-00a…D-00h and of
  **D-07**, the unratified `--auto` re-scope whose provenance D-06 records.

### The recorded answers this phase cites but does not re-decide

- `.planning/decisions/0029-version-trajectory-history.md` — **the single home for the whole version
  line.** Its `## Trajectory` table is what ORCH-05 appends four rows to, and its
  `## Downstream Consumers` names Phase 13 / ORCH-05 explicitly (D-16). **Writing a rival version ADR
  is prohibited.**
- `.planning/decisions/0030-milestone-7-self-numbering.md` **`:79-84`** — **already records the
  Roadmap Extension Protocol's fifth-instance prediction closed** (D-17). ORCH-05 cites it; if a fifth
  collision is found, this ADR is amended in place.
- `.planning/decisions/0008-workspace-version-0-7-0.md` — Phase 4's convergence on `0.7.0`. **REL-01 is
  closed by this**; ORCH-05 applies it (D-16).
- `.planning/decisions/0015-core-ports-dependency-allowlist.md` **§Decision (i)** — `paladin-core` and
  `paladin-ports` may carry no provider SDK, transport client, storage driver **or web framework**.
  **This is the invariant D-14 turns on.**
- `.planning/decisions/0031-extracted-crate-dependency-rule.md` — Phase 10's default-build invariant.
  D-14 is the same principle applied to a trait's parameter type rather than to a manifest edge.
- `.planning/decisions/0036-audit-suppression-single-source-topology.md` — Phase 12's ADR closing
  SUPPLY-03; with `scripts/check-workflow-suppressions.sh`, `Makefile:171-176` and
  `.github/workflows/ci.yml:103-104`, it is the "and no longer does" half of D-05.
- `.planning/decisions/0024-rustsec-exception-governance.md` and `SECURITY-EXCEPTIONS.md` — Phase 9's
  register, cited by the M10 Epic 2 ledger rows.
- `.planning/decisions/0006-coverage-gate.md` — the 84% workspace line-coverage floor. No `.rs`
  changes here; the close-out confirms the number is unmoved (D-19).

### Verification inputs

- `.planning/intel/code-verification.md` **lines 417-659** — **the run-5 verification block.** The
  37-row Verified SHIPPED table (`:428-471`), the `deny.toml` correction to run 4 (`:475-493`), the
  **eight** Verified OPEN findings (`:495-620`), the four checkbox verdicts (`:622-645`), and the
  final corpus position on open-checkbox counts (`:647-659`). **Third in the precedence order.**
  **Two of its statements are superseded and this phase corrects them:** the 14-job `ci.yml` list at
  `:539-540` (D-08), the `v0.6.0` current-state figure at `:469` (D-18), and finding 8's "fails on
  every run" consequence clause at `:619-620` (D-09).
- `.planning/intel/SYNTHESIS.md` **`:330-340`** and **`:546`** — the `settled-by` mechanism and the
  sixteen entries that carry it. **These are variant-register entries, not ledger rows** (D-04).
- `.planning/intel/task-completion-state.md` **`:25-40`, `:96-105`, `:117`** — M9 100%/0 open,
  M10 100%/0 open, M11 92%/26 open (all in `Epic_3/tasks-content-rewrite.md`), M12 99%/3 open (all in
  `Epic_5/tasks-api-security-authorization.md`), project-management 0%/1 open. **Do not re-derive
  these counts, and do not trust them** — D-10 supplies the verdict on each.
- `.planning/INGEST-CONFLICTS.md` — the run-5 warnings on the competing agent route surface (D-11),
  the `AgentProvisioner` placement (D-14), HTTP-served agents having no Garrison and no Arsenal
  (D-15), the competing token mechanism (group 29, handed to Phase 14) and Epic 27.
- `.planning/phases/12-supply-chain-gate-integrity/12-01-SUMMARY.md` §Grep Inventory — the 87-hit
  stale-citation measurement and its scoping rule (D-07). **Inherit it; do not re-run it.**

### Source documents this phase records or corrects

- `.project/Milestone_12-Web-API/Epic_1/`, `Epic_3/`, `Epic_4/`, `Epic_5/` — the acceptance criteria,
  test assertions and examples naming unprefixed `/agents…` paths, annotated as superseded provenance
  (D-11).
- `.project/Milestone_12-Web-API/Epic_6/` §4.3 — *"the agent API is served under `/v1`; operational/docs
  endpoints remain unversioned"* — **the position that shipped** (D-11).
- `.project/Milestone_12-Web-API/Epic_1/` §4.4 FR-15 and **Open Question 2** — the `AgentProvisioner`
  placement default, and **§7's claim that "either placement is clean since it references `Paladin` +
  `PaladinExecutorPort`"**, which D-14 shows omits `AgentSpec`.
- `.project/Milestone_12-Web-API/Epic_2/` §4.2 non-goal and `Epic_3/`'s restatement — *"Garrison
  (memory) and Arsenal (tools/MCP) wiring for agents — a later enhancement; agents are LLM + prompt
  only here"* (D-15).
- `.project/Milestone_12-Web-API/Epic_5/` **Open Question 4** — unanswerable for the shipped adapter;
  recorded by ORCH-01, resolved by WEB-01 (Phase 14).
- `.project/Milestone_11-Documentation-Overhaul-Publish/Epic_6/` FR-8 — `overview.md` as *"the single
  source of routing"* between topologies. **This is why D-15 puts the limitation there.**
- `.project/Milestone_11-Documentation-Overhaul-Publish/Epic_3/` non-goals — *"the 35 appendix files
  are reference/archive material and are not rewritten in this Epic"* — the exemption that froze the
  architecture gap (D-13d).
- `.project/Milestone_11-Documentation-Overhaul-Publish/Epic_3/tasks-content-rewrite.md` — the 26 open
  items, tasks 6.0, 7.0 and 1.2 (D-10).
- `.project/Deferred-QA-CICD-Completion/` Epics 25, 26, 27 and the Epic 28/29 coverage register — the
  only ingested epic-set verified open item by item.
- `.project/project-management/tasks-project-management-setup.md` — the one "open" item that is a
  formatting example (D-10).
- Requirement texts naming stale paths: `REQ-listener-service-test-coverage`,
  `REQ-llm-tool-calling-port`, `REQ-arch-doc-modernization`, `REQ-asciinema-demos` (D-13), plus the
  four Milestone 12 references to `project/current-exports.txt` (D-09).

### Defect and change sites — all verified this session, 2026-08-10

**ORCH-03(a) / D-11, D-12:**
- `crates/paladin-web/openapi.json` — six agent paths, **all `/v1`-prefixed**: `/v1/agents`,
  `/v1/agents/{id}`, `/v1/agents/{id}/execute`, `/v1/agents/{id}/execute/stream`,
  `/v1/agents/{id}/jobs`, `/v1/agents/{id}/jobs/{job_id}`.
- `docs/src/deployment-topologies/sidecar.md:29` — **`POST /agents/{id}/execute`**, the only
  unprefixed route reference anywhere in `docs/src/`, `examples/` or `README.md`. The one live defect.
- `docs/src/deployment-topologies/http-service-host.md` — the only file under `docs/src/` using
  `/v1/agents`; `:51` shows the correct form.

**ORCH-03(b)-(e) / D-13:**
- `src/core/platform/manager/listener_service.rs` — **absent**;
  `src/application/services/orchestration/listener.rs` — **present**.
- `src/application/ports/output/llm_port.rs` — **absent**;
  `crates/paladin-ports/src/output/llm_port.rs` — **present**.
- `docs/Design/Design_and_Architecture.md` — **absent**;
  `docs/src/appendix/design-and-architecture.md` — **present, 311 lines, zero mermaid blocks**.
- `README.md` — **193 lines**, zero matches for `asciinema` or `demo`.

**ORCH-04(a) / D-14:**
- `crates/paladin-web/src/agent_registry.rs:103-110` — `pub trait AgentProvisioner` and its
  `provision(&self, spec: &AgentSpec) -> Result<ProvisionedAgent, ProvisionError>` signature.
- `:55-56` — `#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)] pub struct
  AgentSpec`, doc-commented *"Sent in the body of `POST /agents`"*; `:76-78` —
  `#[schema(value_type = Vec<String>)]`.
- `crates/paladin-web/src/lib.rs:46` — the public re-export of all five types.
- `crates/paladin-web/Cargo.toml:35-37` — `utoipa`, `utoipa-axum`, `utoipa-swagger-ui`.
- `crates/paladin-ports/Cargo.toml` `[dependencies]` — eleven entries, **no `utoipa`**.
- `src/infrastructure/web/facade_provisioner.rs:70` — `impl AgentProvisioner for FacadeProvisioner`,
  the only implementation outside `paladin-web`'s own tests.
- `src/infrastructure/web/mod.rs:2` and `src/infrastructure/mod.rs:48` — `#[cfg(feature =
  "web-server")]`; `Cargo.toml:276` — `web-server = ["dep:paladin-web", "dep:axum"]`.
- `docs/src/deployment-topologies/queue-worker.md:25-26,55` — the worker runs the agent through
  `PaladinExecutionService` and *"is itself an embedded agent host"*; `grep -in
  "provision\|AgentSpec"` over `queue-worker.md` and `sidecar.md` → **zero matches**.

**ORCH-04(b) / D-15:**
- `docs/src/deployment-topologies/http-service-host.md:54` — `Service->>Agent: run (LLM + tools +
  memory)`.
- `docs/src/deployment-topologies/overview.md` — the five-topology comparison table with `Use when` /
  `Avoid when` columns, and the mermaid routing flowchart. No mention of Garrison or Arsenal.
- `docs/src/deployment-topologies/embedded-library.md:31-32` — *"memory (Garrison), and tools
  (Arsenal)"*, the correct advertisement for the embedded topology.
- `grep -in "garrison\|arsenal" docs/src/deployment-topologies/*.md` — hits **only**
  `embedded-library.md`.

**ORCH-05 / D-16, D-17, D-18:**
- `Cargo.toml:34` — **`version = "0.7.0"`** (ORCH-05's text says `0.6.0`).
- `git tag --sort=-v:refname | head` — **`v0.7.1`, `v0.7.0`**, `v0.5.1`, `v0.5.0`, `v0.4.3`, `v0.4.2`,
  `v0.4.1`, `v0.4.0` (ORCH-05's text says latest is `v0.5.1`).
- `git branch --show-current` — `release/v0.7.0` (accurate).
- `.planning/decisions/0029-version-trajectory-history.md` `## Trajectory` — the table and its
  Phase 13 / ORCH-05 placeholder row.
- `.planning/decisions/0030-milestone-7-self-numbering.md:79-84` — the prediction already closed.

**ORCH-01 / D-04, D-05, D-08, D-09:**
- `.planning/REQUIREMENTS.md:3607-3931` — 120 rows; `grep -c '^| REQ-'` over the range → **120**;
  bare `Verify` → **35**; bare `Shipped` → **53**; `grep "settled-by"` over the range → **0**.
- `.planning/decisions/PROMOTION.md:60` — `**Next free ADR number: 0037**`.
- `ls .planning/ledgers/` — exactly four files; `milestone-09-12.md` **does not exist**.
- `grep -nE '^  [a-z][a-z0-9-]*:$' .github/workflows/ci.yml` — **15 jobs** (D-08's list).
- `grep -n "cargo audit --ignore" .github/workflows/ci.yml` — **no matches**; Phase 9's deletion held.
- `scripts/check-api-surface.sh:6` — `BASELINE="${1:-.project/current-exports.txt}"`;
  `.project/current-exports.txt` exists (442 KB); `project/current-exports.txt` does not.

### Codebase maps and conventions

- `.planning/codebase/STRUCTURE.md` — the workspace shape; corrected by Phase 7, unchanged since.
- `.planning/codebase/CONVENTIONS.md` — naming and module conventions (analysis date 2026-07-30).
- `.planning/codebase/ARCHITECTURE.md` — the hexagonal layering D-14 turns on.
- `docs/book.toml` — `[output.linkcheck] follow-web-links = false`, `warning-policy = "error"`; the
  gate the three doc edits must not break (D-19).
- `CLAUDE.md` and `.github/instructions/rust.instructions.md` — the workspace gate
  (`cargo test` → `cargo fmt --check` → `cargo clippy -- -D warnings`) and the medieval-military
  ubiquitous-language requirement.

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets

- **Four complete ledgers** — `.planning/ledgers/milestone-01.md` (121 KB), `milestone-02-03.md`
  (213 KB), `milestone-04-06.md` (150 KB), `milestone-07-08.md` (136 KB). The fifth is a known
  quantity: copy the head-note structure, the verdict vocabulary mapping, and the in-place amendment
  convention. **Do not reinvent the format.** `milestone-07-08.md` is the most recent, was written by
  the closest-analogue phase, and already names this file.
- **REQUIREMENTS.md's existing 120-row run-5 ledger** (`:3607-3931`) — already carries per-`REQ-*`
  verdicts, epic-level section headers, an eleven-class status key and a substantial head note
  covering the new verdict class and the Deferred-QA register. D-03 upgrades it; it is a starting
  point, not a blank page, and roughly two-thirds of the rows already carry something.
- **`intel/code-verification.md`'s run-5 block** (`:417-659`) — 37 verified-shipped claims with tree
  evidence, eight verified-open findings, four checkbox verdicts and the corpus-level position. This
  is the largest pre-assembled evidence set of the five runs, and it is why 53 of the 120 rows are
  cheap.
- **`crates/paladin-web/openapi.json`** — a committed drift-guard baseline that answers ORCH-03(a)
  outright. Run 5's own note is that it *"locks in whichever form actually shipped"*; reading it is
  cheaper and more authoritative than reading five Epics.
- **`.planning/decisions/0001`-`0036`** — thirty-six ADRs in the target format. **0015 is D-14's
  governing invariant**; **0029 is ORCH-05's whole deliverable**; **0030 already contains ORCH-05's
  second half**; 0008 and 0031 show how a later phase cites an earlier answer instead of re-deciding
  it — the move D-14, D-16 and D-17 all make.
- **Phase 12's grep inventory** (`12-01-SUMMARY.md` §Grep Inventory) — 87 hits across 25 files with a
  class-by-class scoping rule. Inherited whole (D-07).

### Established Patterns

- **Precedence is the project's core mechanic** (D-00b), and this phase writes at three levels of it —
  ADRs (top), the ledger, and `.project/` corrections (fifth/sixth). It also writes at a level the
  four prior ground-truth phases did not: **published `docs/src/` pages**, which sit between the tree
  and the maps because a reader treats them as the contract. That is what makes D-12 and D-15
  corrections rather than annotations.
- **Contested positions get ADRs; code-settled defects get ledger rows** (D-00g) — D-20 allocates
  three ADRs against five requirements, and ORCH-01, ORCH-02 and ORCH-05 correctly get none.
- **Retain superseded text; amend in place; date every amendment** (D-00c, D-00d).
- **Documents lie about themselves in both directions, and this session reproduced it five times
  inside ORCH-01…ORCH-05's own text**: the 16/104 arithmetic counts two populations (D-04); the
  `ci.yml` job list is one job short and two jobs stale (D-08); DEBT-01's "fails on every run" is
  closed in the script and open in the docs (D-09); ORCH-05's current-state figures are two releases
  behind (D-18); ORCH-05's second half was discharged by ADR-0030 five days before this session
  (D-17). **Re-read every cited `file:line` before acting on it.**
- **Later phases move the ground under this one.** Phase 8 closed DEBT-01's script half; Phase 9
  deleted the duplicate `security` job that is the *subject* of ORCH-01's headline verdict class;
  Phase 10 wrote ORCH-05's target table and pre-closed its second half; Phase 12 closed all three
  SUPPLY requirements and measured the stale-citation inventory. **A planner reading only the ingest
  record will plan a phase materially larger than the one that exists** — but note the direction
  differs from Phase 10: here the *ledger* is bigger (120 rows, the largest) while the *decision* work
  is smaller (three ADRs, and two of the five requirements are citation work).
- **The committed `openapi.json` is this run's equivalent of Milestone 8's reconciliation** — the one
  artefact in the corpus that settles a five-document disagreement by construction. Where it and an
  Epic disagree, it wins.

### Integration Points

- **`.planning/ledgers/milestone-09-12.md`** — new file, **fifth and final sibling**. The series is
  complete after this phase; the close-out amendment should say so.
- **`.planning/decisions/0037`…`0039`** — new files; `PROMOTION.md:60` advances to 0040.
- **`.planning/decisions/0029-version-trajectory-history.md`** — **amended, not replaced**: four rows
  appended to `## Trajectory`, the Phase 13 placeholder row resolved.
- **`REQUIREMENTS.md`** — §"Milestone 9-12 as-shipped ledger" (`:3607-3931`) reduced to a pointer
  (D-01); ORCH-01's 16/104 sentence (`:2210-2212`) and ORCH-05's version figures (`:2300-2301`)
  corrected (D-04, D-18); PIPE-01's job list (`:2434-2436`) corrected (D-08); ORCH-01…05 checkboxes
  flipped; traceability rows `:4251-4255` updated; three hand-off blocks written (D-22).
- **`docs/src/deployment-topologies/sidecar.md:29`** — one route corrected (D-12).
- **`docs/src/deployment-topologies/http-service-host.md:54`** — one diagram line corrected, plus the
  stated limitation (D-15).
- **`docs/src/deployment-topologies/overview.md`** — the limitation surfaced in the routing matrix
  (D-15). **These three are the phase's entire in-tree surface.**
- **Roughly ten `.project/` documents** — annotated, never rewritten.
- **Phase 14 / WEB-01…WEB-04** — receives ADR-0038, ADR-0039, and the `Verified open` rows.
- **Phase 15 / PIPE-01…** — receives D-08's job list and D-09's baseline-path finding.
- **Phase 16 / DOCS-01…DOCS-04** — receives ORCH-02's M11 verdict and D-13(d)(e).

</code_context>

<specifics>
## Specific Ideas

**Five findings surfaced during this session that neither the ingest record nor Phases 9-12 contain.**
Each was read from the tree on 2026-08-10. Treat them as verified starting points, not hypotheses.

1. **ORCH-01's own arithmetic counts two different populations.** ORCH-01 says sixteen entries already
   carry `settled-by` pointers "and the remaining 104 need the same treatment". The sixteen are
   **variant-register** entries (`intel/SYNTHESIS.md:546`, under the variants section, with the
   mechanism defined at `:335` as applying to *variants*); `grep "settled-by"` across the ledger's own
   rows (`REQUIREMENTS.md:3607-3931`) returns **zero**. All 120 rows need verdicts. The population
   that actually matters is the **35 bare `Verify` rows** — 53 more are a bare `Shipped`, which D-03's
   evidence bar rejects as a claim, and 32 already carry something richer. A planner budgeting 104
   rows will be wrong in both directions simultaneously.

2. **ORCH-03(a) is already answered, and the live defect it should have named is somewhere else.**
   `crates/paladin-web/openapi.json` prefixes all six agent paths with `/v1` — Epic 6 §4.3 won, and
   the four Epics naming unprefixed paths are provenance. But the four Epics are `.project/` PRDs
   nobody executes against. The one place an unprefixed route is *published* is
   `docs/src/deployment-topologies/sidecar.md:29`, which tells a reader to `POST /agents/{id}/execute`
   against a server that serves `/v1/agents/{id}/execute` — and it is the **only** such reference in
   `docs/src/`, `examples/` or `README.md`. ORCH-03's done-when ("cannot write to a path that does not
   exist") is satisfied by a one-line documentation fix it does not mention.

3. **ORCH-04(a)'s framing rests on an Epic §7 claim that omits the type which decides the question.**
   §7 says either placement is clean "since it references `Paladin` + `PaladinExecutorPort`, both
   already in core/ports". The trait signature also references **`AgentSpec`**, which derives
   `utoipa::ToSchema` (`agent_registry.rs:55`) and is doc-commented *"Sent in the body of `POST
   /agents`"*. `paladin-ports` carries no `utoipa`, and ADR-0015(i) bars web-framework dependencies
   from it. **Promotion is not "clean" — it moves an OpenAPI request-body schema into the core-tier
   ports crate.** This converts the question from a judgement call ("is one consumer enough?") into a
   determined one, and it means the default's escape clause ("promote when a second consumer appears")
   would have fired on the wrong signal: a second implementation already exists
   (`src/infrastructure/web/facade_provisioner.rs:70`) but it is `web-server`-feature-gated, i.e. the
   HTTP composition root, not a new topology.

4. **ORCH-04(b) is a contradiction, not an omission.** ORCH-04(b) argues that one line in a non-goal
   is not enough surface for the capability difference. Verified: `http-service-host.md:54`'s sequence
   diagram reads `Service->>Agent: run (LLM + tools + memory)` — the HTTP topology page **advertises**
   Arsenal and Garrison, in the same words `embedded-library.md:31-32` uses for the topology that
   actually has them. `overview.md`, which M11 Epic 6 FR-8 makes "the single source of routing", says
   nothing either way. So the reader is not under-informed; they are actively misinformed by the page
   the decision matrix routes them to. The fix has a mandatory component (`:54`) that ORCH-04(b) does
   not anticipate.

5. **ORCH-05's second half was closed by Phase 10, and ORCH-05's own current-state figures are two
   releases stale.** `0030-milestone-7-self-numbering.md:79-84` already records the Roadmap Extension
   Protocol's fifth-instance prediction discharged, explicitly so that "no later phase inherits a
   standing prediction to check" — Phase 10's D-14, dated 2026-08-08. And ORCH-05's text describes the
   tree as `Cargo.toml` `0.6.0` with latest tag `v0.5.1`; it is `0.7.0` (`Cargo.toml:34`) with
   `v0.7.1` and `v0.7.0` both present. **This is the same sentence Phase 10's D-11 corrected in
   HARD-03, regrown one requirement later** — the strongest available argument for correcting it at
   source rather than only in the ledger.

**Scale note for the planner:** 120 requirements across 24 sections — the largest of the five ledgers,
against Phase 10's 86 across 12 and Phase 7's 115 across 13 — but the *decision* surface is the
smallest of any ground-truth phase: three ADRs, one ADR amendment, three documentation lines, and no
`.rs`. **Do not size this phase from row count alone.** Findings 2, 3 and 5 shrink ORCH-03(a),
ORCH-04(a) and ORCH-05 to citation-plus-one-line work; finding 1 changes the ledger's own arithmetic;
and the expensive population is the 35 bare `Verify` rows plus the 53 `Shipped` rows whose citations
must be re-derived rather than carried. Budget the savings into the **Deferred-QA `Verified open`
block** — those rows are the direct and only input to Phases 14, 15 and 16, and this is the last
ground-truth phase that will touch them.

</specifics>

<deferred>
## Deferred Ideas

- **WEB-01's JWT-vs-opaque token mechanism** — ORCH-01 ledgers it as `Contract diverges` with
  citations (`crates/paladin-web/src/agent_auth.rs`, `src/infrastructure/adapters/auth/
  in_memory_token_auth_adapter.rs`, and the absence of `jsonwebtoken` anywhere in the workspace).
  Deciding it is **Phase 14's**, and run 5 records it as the corpus's only variant shipped code does
  not settle (group 29).
- **WEB-02's multi-replica token verification** — `k8s/deployment.yaml` ships probes for
  multi-process serving against an in-process token store. Recorded as `Verified open`; **Phase 14**
  owns the fix or the `replicas: 1` pin.
- **WEB-03 / WEB-04, `ProviderCapabilities` and LLM tool calling** — Epic 27 verified entirely
  unimplemented. ADR-0039 supplies half of WEB-04's required "stated relationship" between Arsenal/MCP
  and a future `LlmPort` tool surface; the rest is **Phase 14's**.
- **Wiring Garrison and Arsenal into HTTP-served agents** — the option D-15 declined. It needs
  `AgentSpec` fields for memory and tool configuration, which is real API design (MCP server identity,
  credentials, lifetimes) that no milestone has scheduled. Recorded here so a later reader sees it was
  considered and closed rather than forgotten. If a human overturns D-15, it is a milestone, not a
  clause.
- **Promoting `AgentProvisioner` to `paladin-ports`** — the option D-14 declined, and the shape it
  would have to take (split `AgentSpec` into a domain spec and an HTTP DTO with a `From` conversion,
  move `ProvisionError`/`ProvisionedAgent`, deprecate the `paladin-web` re-export). Phase 14 if
  overturned.
- **Deferred-QA Epic 25's coverage-threshold variant** — a **78% hard gate** (parent PRD FR-25.3)
  versus a **phased 70 → 74 → 78 ramp** (Epic 25 FR-25.6), with the parent PRD's own OQ-3 recorded
  Open. ORCH-01 ledgers both sides; **Phase 15** picks one.
- **The eight deprecated GitHub Action references** — `actions-rs/toolchain@v1` ×4,
  `actions/cache@v3` ×3, `codecov/codecov-action@v3` ×1. **Phase 15 / PIPE-04.** Untouched here, as in
  Phases 8, 9, 10 and 12. Note the line numbers in run 5's list are stale by D-08.
- **The 311-line architecture document rewrite** — D-13(d) records the relocation as closed and the
  content gap as open; DOCS-02 / **Phase 16** owns the rewrite, the seven missing subsystems and the
  four missing mermaid diagrams.
- **`docs/assets/` (empty) and the missing `docs/DEMOS.md`** — D-13(e)'s other half. **Phase 16.**
- **`scraper`, `rss` and `tiktoken-rs`: three optional dependencies in `paladin-content` that no code
  consumes** — carried forward unresolved from Phase 10 (its specifics 4) and named to Phase 15 by
  Phase 10's hand-off.
- **The seven crates still setting `[lib] doctest = false`** — recorded by Phase 10's D-22, owned by
  **Phase 15**.
- **A CI dependency-allowlist check built on `cargo tree`** — Phase 15, from ADR-0015, with Phase 10's
  D-15 `--no-default-features` clause. Carried forward unresolved from Phases 7, 8, 9 and 10.
  **D-14 adds a third clause worth checking there:** that `paladin-ports` acquires no OpenAPI or web
  dependency.
- **Applying the GitHub rulesets and protecting `main`** — `.github/rulesets/` is version-controlled
  and unapplied. **Owner: the repository owner / milestone close-out**, per Phase 12's hand-off item 5.
  This phase records it and applies nothing.
- **Stray root artefacts** — `api_surface_current.txt`, `final-api.txt`, `flat`, `lcov.info`. Carried
  forward from Phases 9 and 10; housekeeping.
- **Replacing `dotenv` with `dotenvy`** and the other unmaintained-advisory upstream paths — carried
  forward from Phases 9, 10 and 12. `SECURITY-EXCEPTIONS.md` records the conditions.
- **Owner and expiry fields for the 13 `deny.toml` ignores that have neither** — run-5 finding 2's
  governance-surface gap. Recorded by ORCH-01; no phase owns it. **Candidate for the milestone
  close-out**, since ADR-0024's governance is Phase 9's and the review dates are an owner decision.
- **A `SECURITY.md` for GitHub's advisory UI** — carried forward from Phases 9 and 10. Phase 16.
- **Retiring or replacing `src/main.rs`, the legacy content-aggregator entry point** — carried forward
  unresolved from Phases 7, 8, 9 and 10.
- **Nyquist validation for Phases 1-4** — carried forward unresolved from Phases 5, 7, 8, 9 and 10.
  Owner: `/gsd-validate-phase 1`…`4`.
- **Whether ADRs should be published to the mdbook for framework consumers** — carried forward
  unanswered from Phases 1, 5, 7, 8, 9 and 10. **Seven phases is enough; Phase 16 should answer it or
  record it declined.** Note this phase adds three more ADRs to the unpublished set.

</deferred>

---

*Phase: 13-milestone-9-12-ground-truth-recorded-account*
*Context gathered: 2026-08-10*
</content>
</invoke>
