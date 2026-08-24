# Deferred Items — Phase 08 (Verified Defect Closure)

## From Plan 08-05 (DEBT-01 requirement-text half)

### Four additional `project/current-exports.txt` propagation sites, out of scope for this plan

**Found during:** Task 1's acceptance-criteria verification (the repository-wide
`grep -rn 'project/current-exports.txt' .project/ --include='prd-*.md' | grep -v '~~' | grep -v '\.project/current-exports'`
command the plan itself specifies).

**Disposition:** Logged, not fixed — SCOPE BOUNDARY (executor deviation rules): these are
pre-existing issues not caused by this task's changes, and plan 08-05's `<files>` frontmatter and
Task 1 acceptance criteria explicitly enumerate exactly five `.project/` documents
(`git status --short .project/` is required to list "exactly the five PRD files"). Fixing these
four would violate that explicit criterion. Reported here instead, per the deviation-rules
protocol ("Log out-of-scope discoveries to `deferred-items.md`... Do NOT fix them").

**Consequence for the plan's own must_haves:** the backstop truth "No sixth propagation site
exists — a repository-wide grep for the stale literal outside `.project/` task-list history and
outside the corrected sites returns nothing new" is **not actually true**. Four more sites exist.
DEBT-01's corpus-recorded count of "nine total references" (five tooling + five requirement-text)
underestimates the real propagation — it is at least thirteen counting these four, none of which
match the "past-tense task-list" exemption (all four are forward-instructing prose, the same
defect class the five corrected documents carry).

**The four sites, with `file:line` and character:**

1. `.project/Milestone_12-Web-API/Epic_4/prd-api-cross-cutting-concerns.md:225-227` — an "API
   surface" bullet in §7 Technical Considerations, structurally identical to the corrected Epic 1 /
   Epic 5 / Epic 6 bullets: "new public items (`ApiError`, health handlers, layer/config types)
   change `project/current-exports.txt` ... — regenerate the baseline". **This file matches the
   plan's own `prd-*.md` verification glob**, so this is a direct, mechanically-detected miss
   against DEBT-01's original nine-reference count, not a stretch interpretation.
2. `.project/Milestone_11-Documentation-Overhaul-Publish/Epic_6/prd-deployment-topologies-documentation.md:254`
   — FR-14: "Add a `[Unreleased]` entry to `CHANGELOG.md` ... Do **not** regenerate
   `project/current-exports.txt` (no API change)." A negative instruction (telling the implementer
   not to touch the baseline), but it still names the stale, nonexistent path as if it were the
   real artifact path. **Also matches the `prd-*.md` glob.**
3. `.project/Milestone_12-Web-API/overview/Milestone-12_Web-API.md:318` — Epic 7 summary, "Task
   7.6: CHANGELOG & API surface. Update `CHANGELOG.md [Unreleased]`, regenerate
   `project/current-exports.txt`, and bump versions toward v0.6.0." An epic-overview rollup
   document, not `prd-*`-named, so it falls outside even the plan's own mechanical check pattern.
4. `.project/Milestone_8-Facade-Cleanup-Shim-Resolution/Epic_7/Milestone_8-Epic_7-paladin-web-single-framework-axum.md:40`
   — an Epic-summary companion document (not `prd-*`-named) to the corrected
   `prd-paladin-web-single-framework-axum.md`: "In scope: ... `deny.toml`;
   `project/current-exports.txt`; `CHANGELOG.md`."

**Recommendation:** route to plan 08-09 (which owns DEBT-01's checkbox/status closure behind
evidence) or a small follow-up plan applying the same D-00c annotation pattern to these four
files before DEBT-01's checkbox is ticked. Do not tick DEBT-01's checkbox on the assumption that
"nine of nine" references are closed — the true count is at least thirteen, and four remain open.

## Addition, dated 2026-08-06 (plan 08-09, on the orchestrator's initiative at seal time — NOT requested by the approving human)

**DEBT-01's checkbox was ticked on 2026-08-06** (its own literal done-condition — five tooling plus
five requirement-text references — is fully met; see `REQUIREMENTS.md`'s DEBT-01 closure note and
`.planning/ledgers/milestone-04-06.md`'s `REQ-api-surface-ci` row). The four sites above remain
open and are **not** part of that done-condition. The approving human selected plain "Approve and seal" and did **not** request an owner for this
residual — a separate residual-assignment option was offered at that checkpoint and was not chosen.
The recommendation below is the **orchestrator's own initiative** at seal time, on the reasoning that
an unowned residual is the exact pattern this corpus keeps getting burned by. It carries no human
authority and Phase 13's planner is free to reject it.

**Recommended owner: Phase 13 (Milestone 9-12 Ground Truth & Recorded Account).** Three of the four
sites are Milestone 12 records (items 1 and 3 above, plus the M12 share of item 2's neighbourhood)
and Phase 13's whole scope is the M9-M12 recorded account; the fourth site (item 4 above) is
Milestone 8, the closest-fitting adjacent pickup since no other phase in the roadmap currently owns
`.project/` record corrections at that milestone range. **This is a recommendation, not a binding
assignment** — Phase 13's own planner (at its discussion/planning stage) should accept it, reassign
it, or explicitly decline it with a reason; this close-out plan has no authority to commit Phase 13's
scope on its behalf.
