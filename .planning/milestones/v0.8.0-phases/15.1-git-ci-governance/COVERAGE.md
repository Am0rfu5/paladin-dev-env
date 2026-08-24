# API Coverage — GitHub REST API, repository rulesets (`/repos/{owner}/{repo}/rulesets`)

> Full coverage by default. Opt-outs are explicit, reasoned decisions.

**Detector result:** `detected: true` (signals `(surface)/api`, `wiring/api`). Confirmed by
re-reading the phase scope: **this phase genuinely integrates an external API.** CONTEXT.md **D-06**
requires a live `POST /repos/DF3NDR/paladin-dev-env/rulesets` via `gh api`, followed by a
read-back whose response is recorded verbatim as the **D-00e** evidence SC2 closes against. A
`No external API integration: …` declaration would be false here, so a real matrix follows.

**Surface enumerated from:** `docs.github.com/en/rest/repos/rules` (fetched 2026-08-13, recorded in
`15.1-RESEARCH.md` §1) plus the adjacent repository/git-ref endpoints this phase's cutover uses.

## Repository rulesets

| capability | decision | reason |
|---|---|---|
| `GET /repos/{o}/{r}/rulesets` (list) | INTEGRATE | The D-00e read-back. This is the evidence SC2 closes against; it currently returns `[]`. |
| `POST /repos/{o}/{r}/rulesets` (create) | INTEGRATE | D-06's core action, applied for `protect-main-branch.json`, `protect-release-branches.json` and `protect-release-tags.json`. |
| `GET /repos/{o}/{r}/rulesets/{id}` (get one) | INTEGRATE | Used to capture each created ruleset's server-assigned `id` and its normalised rule array, which the list endpoint abbreviates. |
| `PUT /repos/{o}/{r}/rulesets/{id}` (update) | INTEGRATE | Required for idempotent re-application: if a ruleset with the same `name` already exists at execution time, the plan updates rather than creating a duplicate. |
| `DELETE /repos/{o}/{r}/rulesets/{id}` | INTEGRATE | The documented rollback path. D-06's reversibility claim ("a ruleset can be deleted via the same API") is only true if the rollback command is written down; plan `15.1-08` records it. |
| `GET /repos/{o}/{r}/rules/branches/{branch}` (rules for a ref) | INTEGRATE | The second, independent confirmation that the applied ruleset actually evaluates against `refs/heads/main` — a ruleset can exist and still match no ref if `conditions.ref_name` is wrong. |
| `GET /repos/{o}/{r}/rulesets/rule-suites` (list evaluations) | OPT-OUT | Not needed yet — rule-suite history only becomes informative after merges have been evaluated against the applied ruleset, which is post-phase. Named as the natural surface for the deferred `check-branch-protection.sh` drift check. |
| `GET /repos/{o}/{r}/rulesets/rule-suites/{id}` (get evaluation) | OPT-OUT | Not needed yet — same reason as the list form. |
| `GET|POST|PUT|DELETE /orgs/{org}/rulesets` (org-level) | OPT-OUT | Explicitly out of scope — both target rulesets are repository-scoped (`target: branch` / `target: tag`, no org-level fields). Verified: `DF3NDR` is an Organization, but no org-level ruleset is needed for this phase's refs. |
| `GET /repos/{o}/{r}/rulesets/{id}/history` (version history) | OPT-OUT | Not needed — this phase creates the first version of each ruleset; there is no prior version to diff against. |

## Adjacent repository / ref endpoints this phase calls

| capability | decision | reason |
|---|---|---|
| `GET /repos/{o}/{r}` (repo metadata) | INTEGRATE | Reads `default_branch`, `visibility` and `permissions.admin` before the cutover; D-06 depends on the admin flag being re-checked at execution time, not assumed. |
| `PATCH /repos/{o}/{r}` (repo settings) | INTEGRATE | Sets `delete_branch_on_merge: true` — the Claude's-Discretion item resolved in plan `15.1-07`. |
| `DELETE /repos/{o}/{r}/git/refs/heads/{branch}` | INTEGRATE | Retires `develop` and `release/v0.7.0` under D-01/D-02. |
| `GET /repos/{o}/{r}/commits/{sha}/check-runs` | INTEGRATE | Produces the **empirical** required-status-check context names D-05's ruleset pins, instead of names guessed from `name:` fields. Also detects duplicate context names (finding 6's defect class). |
| `GET /repos/{o}/{r}/branches/{branch}/protection` (legacy branch protection) | OPT-OUT | Explicitly out of scope — this is the pre-rulesets API. It is read once as a *before* datum (currently `404 Branch not protected`) and is never written; D-06 chose rulesets, and writing both layers would create two competing sources of truth. |
| `GET /repos/{o}/{r}/collaborators` | INTEGRATE | Re-confirms the sole-collaborator premise D-07 rests on before the bypass is removed. |
| `POST /repos/{o}/{r}/pulls`, `PUT .../pulls/{n}/merge` | INTEGRATE | The dogfooding path — every post-cutover plan lands as a PR into the protected trunk. |

## Notes

- **No `OPT-OUT` row is undecided.** Every one carries a reason, per the gate's own rule.
- **Second-integration rule, noted for the future:** if a later phase adds org-level rulesets or a
  second protected repository, it re-decides this whole matrix from the full-coverage baseline
  rather than inheriting these opt-outs.
- **Token scope is the failure mode to detect, not assume:** `gh auth status` reporting the *user's*
  `admin: true` role is necessary but not sufficient — a fine-grained PAT additionally needs
  `Administration: write`, which only a write attempt proves (HTTP 403 naming the missing
  permission). Plan `15.1-08` smoke-tests the cheaper tag ruleset first for exactly this reason.
