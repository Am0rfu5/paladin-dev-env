Relevant Files

.github/ISSUE_TEMPLATE/ai_security_ticket.md - AI‑first issue template capturing acceptance criteria, tests, threat model, and YAML frontmatter for automation.
.github/PULL_REQUEST_TEMPLATE.md - PR template with ENV/Secrets section, install/build steps, security checklist, and required checklist for human review.
.github/PULL_REQUEST_TEMPLATE_LARGE.md - Variant for documented large changes (requires extra reviewer + rationale).
.github/workflows/ci.yml - Primary CI pipeline: checkout (with submodules), install, build, tests, linters, caching.
.github/workflows/security.yml - Security scans: CodeQL, dependency scanning (Dependabot/Snyk), container scanning, infra-as-code checks.
.github/workflows/ai_review.yml - Job to run AI code review and post annotated feedback/comments to PRs.
.github/workflows/submodule-sync.yml - Action to automate submodule updates and create PRs across repos when shared resources change.
shared-resources/devcontainer/devcontainer.json - Shared devcontainer configuration (dotfiles, extensions, common tasks).
shared-resources/shared-libs/README.md - Docs & bootstrap for shared libraries and versioning policy.
shared-resources/deployment-records/README.md - Structure for deployment records (artifact hash, ticket/PR link) and format expectations (JSON/YAML).
tools/update-submodules.sh - CLI/script to update submodules and create linked PRs.
tools/cli/pm-helper - CLI helper for branch creation, ticket validation, and running local preflight checks (stub & docs).
tools/ai/prompts/ticket_prompt.md - Versioned AI prompt used to generate tickets from short descriptions.
tools/ai/prompts/prd_prompt.md - Prompt to expand tickets into concise PRDs.
docs/process/triage-and-roles.md - Triage rotation, security champion playbook, and on-call responsibilities.
docs/templates/prd-template.md - PRD template consumed by AI when expanding tickets.
ci/tests/unit/ - Example folders for unit tests (language-specific).
deployment-records/records.yml - Canonical file (or repo) where deployment writeups are appended by release jobs.

Notes

Put unit tests beside implementation files where appropriate (e.g., pkg/foo/foo.test.js next to pkg/foo/foo.js).
Keep AI prompts in Markdown with minimal YAML frontmatter for machine parsing.
Use clear file names (no spaces) so automation can discover templates reliably.


Instructions for Completing Tasks
IMPORTANT: As you complete each task, change - [ ] to - [x] in this file. Update after completing each sub-task (not only after finishing a parent task). Use the ticket/branch referenced in 0.1 for all commits (branch naming: feature/<ticket-id>-<slug>).
Example:

- [ ] 1.1 Create template → - [x] 1.1 Create template (after completing)


Tasks

 
0.0 Create feature branch

 0.1 Create and checkout a new branch for this feature (e.g., git checkout -b feature/PMSYS-001-project-management-system-setup)
 0.2 Push branch and open placeholder PR that references the ticket ID


 
1.0 Define & publish AI‑first templates (tickets, PRs, PRDs)

 1.1 Draft .github/ISSUE_TEMPLATE/ai_security_ticket.md with:
YAML frontmatter fields: ticket-id (auto), ticket-type, risk-level, data-classification, ai_assist:true/false
Prompt fields: summary, description, acceptance_criteria, tests_required, security_considerations (threat model)


 1.2 Draft .github/PULL_REQUEST_TEMPLATE.md with:
Top highlighted ENV/Secrets required block
Steps to install/build/test (1–3 commands)
Security checklist section and link placeholder to ticket/PRD
Required checklist items that gate human review


 1.3 Draft docs/templates/prd-template.md used by AI to expand tickets into PRDs (1-page format)
 1.4 Add variant .github/PULL_REQUEST_TEMPLATE_LARGE.md for large/architectural changes with extra reviewer fields
 1.5 Commit templates to shared-resources or .github in each repo as appropriate and open PR(s)
 1.6 Add unit/integration test examples for templates parsing automation (simple parser test)


 
2.0 Implement Git repo organization and submodule workflow

 2.1 Create shared-resources repo (if not present) and add directories: devcontainer/, shared-libs/, deployment-records/, tools/ai/prompts/
 2.2 Add devcontainer.json and a README.md with instructions for using the devcontainer across repos
 2.3 Add deployment-records/README.md defining record format (yaml/json) and sample entry
 2.4 Add shared-libs/ skeleton and versioning guidelines (semver policy, release process)
 2.5 Add tools/update-submodules.sh and tools/cli/pm-helper stubs to shared-resources
 2.6 In each code repo, add the shared-resources as a git submodule at a consistent path (e.g., shared/)
 2.7 Document submodule update workflow in docs/process/submodule_workflow.md (how to update submodule commit, create linked PRs, and test locally)
 2.8 Add a GitHub Action workflow (.github/workflows/submodule-sync.yml) to automate:
creating PRs in parent repos when submodule commits update
linking ticket IDs across PRs


 2.9 Add tests or a CI job that checks submodule pins are correct (failing if submodule ref is not up-to-date)


 
3.0 Build CI/CD pipelines (install/build/test + security + AI review)

 3.1 Draft .github/workflows/ci.yml with jobs:
checkout (with submodules: recursive)
cache dependencies
install dependencies (use lockfile)
build artifacts
unit test job (fast, required)
integration test job (conditional / matrix)
linter & formatter job (auto-fix step optional)
upload test reports/coverage artifacts


 3.2 Draft .github/workflows/security.yml with jobs:
CodeQL analysis (scheduled and on PR)
Dependency scanning (Dependabot/Snyk integration)
Container image scanning (trivy / GitHub Advanced Security)
Infra-as-code scanning (tfsec / checkov) if IaC present
Configure security job to run in parallel but gate merge on completion


 3.3 Draft .github/workflows/ai_review.yml with a job that:
runs after unit tests pass
sends code diff + metadata to LLM provider (obey policy; include max token guard)
receives suggestions and posts an annotated ADVICE report to PR (inline comments and summary)
stores AI report in ticket history (artifact or comment)


 3.4 Implement failure annotations: use GitHub Checks API to annotate failures inline and fail PR status checks when necessary
 3.5 Create reusable CI action steps (composite actions) for install/build/test so other repos reuse them
 3.6 Add caching strategies for node/pip/maven as appropriate to speed CI
 3.7 Add a job to produce reproducible build artifacts and publish them to a staging artifact store (or as GitHub release draft)
 3.8 Add tests for CI workflows (e.g., dry-run via act or pipeline linter)


 
4.0 Integrate AI assistant (ticket/PRD generation & CI code review)

 4.1 Select or approve an LLM provider (list options and privacy/SLAs): note provider choice and ensure contract/policy clearance
 4.2 Create secret entries for AI provider credentials in GitHub Secrets (or Vault) with namespaced keys (e.g., AI_PROVIDER_API_KEY)
 4.3 Implement tools/ai/prompts/ticket_prompt.md and tools/ai/prompts/prd_prompt.md with frontmatter and sample inputs
 4.4 Implement a lightweight service or GitHub Action that:
generates ticket drafts from short prompts and creates GitHub issues (use GitHub API)
expands tickets into PRDs and attaches to issue as comment / file


 4.5 Wire AI code review into ai_review.yml (ensure CI waits for its job)
 4.6 Ensure AI outputs are logged to ticket history and flagged: ai_suggested: true and require human approval checkbox
 4.7 Add tests / acceptance criteria for AI Action (mock responses, verify formatting, verify frontmatter metadata)
 4.8 Document AI usage policy in docs/process/ai_usage.md (consent, audit logs, when to trust suggestions)


 
5.0 Implement tooling & developer UX (CLI helpers, devcontainer, preflight)

 5.1 Implement tools/cli/pm-helper commands:
pm-helper new-branch <ticket-id>: validates ticket and creates branch with correct naming
pm-helper preflight: runs local quick checks that mirror CI (lint, unit tests)
pm-helper create-ticket: CLI wrapper that invokes AI to draft issue


 5.2 Containerize pm-helper or publish as a small node/go binary; provide install instructions in shared-resources/README.md
 5.3 Add devcontainer.json with recommended extensions and workspace settings; verify it mounts shared/ submodule
 5.4 Add local preflight script tools/preflight.sh that runs:
static format/lint fix
unit test quick-run
dependency audit


 5.5 Document developer on-boarding steps in docs/onboarding.md including:
cloning repos with --recurse-submodules
using devcontainer
running pm-helper preflight


 5.6 Add automatic pre-commit hooks (husky/lefthook) to run lint/format checks locally


 
6.0 Branch protection, policies, and governance automation

 6.1 Create GitHub branch protection rules (apply to main, release/*, and set default protections):
require status checks: ci, security, ai-review (names match workflows)
require PR review (1 reviewer minimum for small team; configurable)
require signed commits (optional)
require up-to-date branch before merge (merge queue recommended)


 6.2 Add required check names to CI jobs to ensure checks are enforced in protection rules
 6.3 Add dependabot.yml to configure dependency updates and security auto-merge policy for low-risk deps
 6.4 Implement GitHub CODEOWNERS file for shared code and components to route reviews to the right team
 6.5 Add automation to create/rotate triage owner in docs/process/triage-and-roles.md (cron job or GitHub issue rotation)
 6.6 Set up an on-call/rotation calendar (Google calendar / GitHub Project + automation) and document responsibilities
 6.7 Add a lightweight approval flow for security-flagged tickets that requires security champion sign-off


 
7.0 Rollout, training, and KPIs (monitoring + iteration plan)

 7.1 Prepare rollout checklist and communicate to team: templates merged, CI enabled, branch protections set
 7.2 Run a pilot with 2–3 real tickets:
enable AI assist for those tickets
iterate on prompts and templates based on feedback


 7.3 Configure KPI collection:
PR -> ticket linkage rate (script runs weekly; report in repo)
CI pass rate on first run (aggregate from GitHub checks)
median human review time (calculate via PR open -> first human approval)
mean time to merge
security finding counts (CodeQL/Denylist)


 7.4 Create a weekly report action that posts KPI summary to a team Slack channel (or GitHub Discussions)
 7.5 Train team with a short runbook + 1-hour workshop:
how to use templates, pm-helper, preflight, and devcontainer
how AI is used and approval expectations


 7.6 Schedule a retrospective at Week 4 and Week 12 to iterate on templates, AI prompts, and CI gating
 7.7 Finalize escalation & incident process (tie deploy records to incident playbook)




Quick Implementation Notes / Priorities for a Senior Engineer

Start with parent tasks 1.0, 3.0, and 2.0 in parallel: templates, a minimal CI, and establishing shared-resources submodule — these unblock other work.
Make CI checks idempotent and fast; prioritize unit tests and linting to maximize first-run pass rate.
Keep AI integration feature-flagged (configurable by repo / team) so you can iterate prompts safely.
Treat submodules as a potentially high-friction area — include clear docs and automation to hide complexity from day-to-day devs.
Use lightweight artifacts (signed build hash) and a reproducible deployment-records schema so audits are simple.
