---
schema_version: 1
open_count: 15
waived_count: 0
fixed_count: 1
total_count: 16
last_updated: 2026-08-17T20:01:16.281Z
---

# Broken Windows Ledger

> Cross-phase defect register. `/gsd-ship` blocks while `open_count > 0`.
> Waive with `gsd-tools windows waive <id> "<reason>"` (reason required).
> Mark fixed with `gsd-tools windows fixed <id>`.

| id | phase | kind | file | line | description | status | reason | recorded_at | resolved_at |
|----|-------|------|------|------|-------------|--------|--------|-------------|-------------|
| 1 | 01 | unmet-truth | .planning/ledgers/milestone-01.md |  | REQ-battalion-result-v1 (Epic 4 FR-4.2, cited in ADR-0002's Considered Options as 'superseded by the shipped superset') has no row anywhere in the Milestone 1 ledger's Epic 4 table, even though REQUIREMENTS.md's original ledger body carried it as 'Variant (group 4)'. Plan 01-08 Task 2's subset-check safety gate caught this and HALTED per the plan's explicit instruction rather than reducing REQUIREMENTS.md's Milestone 1 body to a pointer at an incomplete destination. | fixed |  | 2026-07-31T13:22:57.385Z | 2026-07-31T14:46:37.492Z |
| 2 | 03 | deviation | crates/paladin-storage/src/redis.rs |  | Live-server code paths of redis.rs (everything reaching through self.conn) remain uncovered by unit tests; deferred with reason, owner Phase 15 (PIPE), exerciser tests/integration/redis_queue_integration_test.rs (requires Docker) | open |  | 2026-08-02T15:41:28.892Z |  |
| 3 | 07 | deviation | .project/Milestone_5-Workspace-Decomposition/Epic_2/prd-paladin-ports-extraction.md |  | Task 3's requested single combined commit for ADR-0016 + PRD annotation was split into two atomic commits (9e8db80, 71ea46e) per standard task_commit_protocol; both files present, no content impact. | open |  | 2026-08-06T18:09:04.871Z |  |
| 4 | 07 | deviation | .project/Milestone_5-Workspace-Decomposition/Epic_2/prd-paladin-ports-extraction.md |  | No fabricated 3rd strikethrough correction for CONTEXT.md D-08(5)'s anticipated section-1 Milestone 1/Epic 2 cross-reference — re-verified absent from live tree (matches ADR-0014's own flagged drift); acceptance criterion expecting >=3 strikethrough lines not met by design. | open |  | 2026-08-06T18:09:08.207Z |  |
| 5 | 14 | unrun-verify | Cargo.toml |  | cargo test --workspace not run to completion for 14-01: system-wide disk exhaustion (830G/875G used, 0 avail on /workspace mount) blocked full workspace compile; targeted plan <verify> commands (paladin-ai lib config::agents, paladin-web full suite, paladin-server binary build, openapi drift guard, check-api-surface.sh) all passed | open |  | 2026-08-12T16:51:08.832Z |  |
| 6 | 14 | unrun-verify | N/A (workspace-wide) |  | 14-04: full 'cargo test --workspace' not run — shared /workspace mount at 99%25 (13G free), matching 14-01's documented disk-exhaustion condition; the plan's own targeted verify (cargo test --bin paladin-server --features web-server, cargo fmt --check, cargo clippy --all-targets --features web-server -- -D warnings) all ran to completion and passed | open |  | 2026-08-12T17:13:58.989Z |  |
| 7 | 14 | deviation | CHANGELOG.md |  | 14-08's acceptance criterion expected >=2 'BREAKING' lines under the dated 0.8.0 section in root CHANGELOG.md; only 1 is present. 14-01 split the phase's two consumer-break BREAKING entries across root CHANGELOG.md (config-key rename) and crates/paladin-web/CHANGELOG.md (AgentAuthConfig field + OpenAPI scheme rename), one per file, per 14-01-SUMMARY.md's own D4 verification and this plan's own instruction to leave per-crate changelogs untouched. Both breaks are documented with a BREAKING entry and cite ADR-0040; only the single-file grep count in the plan's acceptance criteria was miscalibrated. | open |  | 2026-08-12T18:05:57.086Z |  |
| 8 | 15.1 | unrun-verify | SECURITY-EXCEPTIONS.md |  | Plan 15.1-01 Task 2's inline verify python one-liner (block-split regex over the machine-readable register) fails with a pre-existing TOML parse error on the LAST exception block, because its lookahead doesn't stop before the trailing markdown code fence -- reproduced against the pre-edit file too, unrelated to this task's new row. Substituted an isolated per-block parse of just the new RUSTSEC-2026-0249 row (11/11 fields present) plus the real repo guard scripts/check-advisory-register.sh (exit 0) as equivalent proof. | open |  | 2026-08-14T00:49:21.261Z |  |
| 9 | 15.1 | unrun-verify | .github/workflows/ci.yml |  | Task 1 acceptance criterion 'git diff \| grep -c "^[+-].*cargo "' returns 4 not 0 -- matches step *name* text ('Cache cargo registry' etc.) removed by the migration, not actual cargo invocations. Verified via 'run: cargo' scoped grep returning 0 changed invocations. | open |  | 2026-08-14T14:22:48.884Z |  |
| 10 | 15.1 | unrun-verify | .github/workflows/integration-tests.yml |  | Task 2's first automated verify literally asserts survivors=={pre-commit.yml} after migration, but integration-tests.yml (3 hand-rolled cache blocks) is still present -- deletion is plan 15.1-05's job, not yet executed in this wave, exactly per this plan's own Recorded discretion resolutions section. Substituted an assertion expecting survivors=={pre-commit.yml, integration-tests.yml}, both counts matching (1 and 3 respectively). | open |  | 2026-08-14T14:22:56.039Z |  |
| 11 | 15.1 | unrun-verify | .github/workflows/ci.yml |  | Task 2 acceptance criterion 'grep -rc restore-keys ci.yml feature-flags.yml release.yml' returns 0 for ci.yml -- returns 2, both from pre-existing prose comments in the examples job (added by plan 15.1-01, lines ~268/271) explaining why a restore-keys fallback alone is insufficient, not an actual YAML restore-keys: key. Verified via structural YAML walk: no step's with block contains a restore-keys key in any of the three files. | open |  | 2026-08-14T14:23:03.057Z |  |
| 12 | 17 | unrun-verify | tests/integration/ollama_docker_test.rs |  | Ollama Docker-gated Tier 2 suite (17-07 Task 2) authored and proven to compile/clippy-clean/skip-gracefully, but never run against a real Ollama server -- no Docker daemon in the execution sandbox. Runtime behavior (generate/generate_stream/get_available_models/validate_model against real qwen2.5:0.5b) is unverified. | open |  | 2026-08-17T14:17:30.134Z |  |
| 13 | 17 | unrun-verify | Makefile |  | 17-07 Task 3: the workspace 82% line-coverage gate (make coverage) could not be run in this execution sandbox -- Redis (6380) and MinIO (9010) are unreachable because no Docker daemon is available, and the coverage target's own preflight fails fast on both. The coverage percentage with all six new adapters counted is UNMEASURED, not failing. cargo doc -p paladin-llm --no-deps (0 missing-docs warnings under the six new features) and a scoped clippy pass on touched targets were verified instead. | open |  | 2026-08-17T14:17:37.112Z |  |
| 14 | 17 | deviation | docker/docker-compose.test.yml |  | 17-07 Task 2: ollama-test healthcheck uses 'ollama list' (native /api/tags) instead of the plan's preferred curl-based /v1/models check, because curl/wget availability in the ollama/ollama:0.3.14 base image could not be verified without Docker in this sandbox. 'ollama list' is a well-precedented dependency-free healthcheck for this exact image. Compose file syntax validated via python yaml.safe_load only -- 'docker compose config' itself was never run. | open |  | 2026-08-17T14:17:46.408Z |  |
| 15 | 17 | unrun-verify | crates/paladin-llm/src/gemini/adapter.rs |  | Snyk code scan (per snyk_rules.instructions.md) could not be run — no Snyk MCP tool or CLI available in this worktree's runtime (no network egress); recorded as not-run, never as passed | open |  | 2026-08-17T19:33:52.477Z |  |
| 16 | 17 | unrun-verify | crates/paladin-llm/src/compat/engine.rs,crates/paladin-llm/src/kimi/adapter.rs,crates/paladin-llm/src/qwen/adapter.rs,crates/paladin-llm/src/grok/adapter.rs,crates/paladin-llm/src/ollama/adapter.rs,crates/paladin-llm/src/gemini/adapter.rs |  | Plan 17-10 verification step 7 (Snyk code scan over the five modified WR-04 adapter files plus compat/engine.rs) was not run — snyk_code_scan MCP tool unavailable in the executor runtime | open |  | 2026-08-17T20:01:16.281Z |  |

````json
[
  {
    "id": 1,
    "kind": "unmet-truth",
    "phase": "01",
    "file": ".planning/ledgers/milestone-01.md",
    "line": null,
    "description": "REQ-battalion-result-v1 (Epic 4 FR-4.2, cited in ADR-0002's Considered Options as 'superseded by the shipped superset') has no row anywhere in the Milestone 1 ledger's Epic 4 table, even though REQUIREMENTS.md's original ledger body carried it as 'Variant (group 4)'. Plan 01-08 Task 2's subset-check safety gate caught this and HALTED per the plan's explicit instruction rather than reducing REQUIREMENTS.md's Milestone 1 body to a pointer at an incomplete destination.",
    "status": "fixed",
    "reason": "",
    "recorded_at": "2026-07-31T13:22:57.385Z",
    "resolved_at": "2026-07-31T14:46:37.492Z"
  },
  {
    "id": 2,
    "kind": "deviation",
    "phase": "03",
    "file": "crates/paladin-storage/src/redis.rs",
    "line": null,
    "description": "Live-server code paths of redis.rs (everything reaching through self.conn) remain uncovered by unit tests; deferred with reason, owner Phase 15 (PIPE), exerciser tests/integration/redis_queue_integration_test.rs (requires Docker)",
    "status": "open",
    "reason": "",
    "recorded_at": "2026-08-02T15:41:28.892Z",
    "resolved_at": null
  },
  {
    "id": 3,
    "kind": "deviation",
    "phase": "07",
    "file": ".project/Milestone_5-Workspace-Decomposition/Epic_2/prd-paladin-ports-extraction.md",
    "line": null,
    "description": "Task 3's requested single combined commit for ADR-0016 + PRD annotation was split into two atomic commits (9e8db80, 71ea46e) per standard task_commit_protocol; both files present, no content impact.",
    "status": "open",
    "reason": "",
    "recorded_at": "2026-08-06T18:09:04.871Z",
    "resolved_at": null
  },
  {
    "id": 4,
    "kind": "deviation",
    "phase": "07",
    "file": ".project/Milestone_5-Workspace-Decomposition/Epic_2/prd-paladin-ports-extraction.md",
    "line": null,
    "description": "No fabricated 3rd strikethrough correction for CONTEXT.md D-08(5)'s anticipated section-1 Milestone 1/Epic 2 cross-reference — re-verified absent from live tree (matches ADR-0014's own flagged drift); acceptance criterion expecting >=3 strikethrough lines not met by design.",
    "status": "open",
    "reason": "",
    "recorded_at": "2026-08-06T18:09:08.207Z",
    "resolved_at": null
  },
  {
    "id": 5,
    "kind": "unrun-verify",
    "phase": "14",
    "file": "Cargo.toml",
    "line": null,
    "description": "cargo test --workspace not run to completion for 14-01: system-wide disk exhaustion (830G/875G used, 0 avail on /workspace mount) blocked full workspace compile; targeted plan <verify> commands (paladin-ai lib config::agents, paladin-web full suite, paladin-server binary build, openapi drift guard, check-api-surface.sh) all passed",
    "status": "open",
    "reason": "",
    "recorded_at": "2026-08-12T16:51:08.832Z",
    "resolved_at": null
  },
  {
    "id": 6,
    "kind": "unrun-verify",
    "phase": "14",
    "file": "N/A (workspace-wide)",
    "line": null,
    "description": "14-04: full 'cargo test --workspace' not run — shared /workspace mount at 99%25 (13G free), matching 14-01's documented disk-exhaustion condition; the plan's own targeted verify (cargo test --bin paladin-server --features web-server, cargo fmt --check, cargo clippy --all-targets --features web-server -- -D warnings) all ran to completion and passed",
    "status": "open",
    "reason": "",
    "recorded_at": "2026-08-12T17:13:58.989Z",
    "resolved_at": null
  },
  {
    "id": 7,
    "kind": "deviation",
    "phase": "14",
    "file": "CHANGELOG.md",
    "line": null,
    "description": "14-08's acceptance criterion expected >=2 'BREAKING' lines under the dated 0.8.0 section in root CHANGELOG.md; only 1 is present. 14-01 split the phase's two consumer-break BREAKING entries across root CHANGELOG.md (config-key rename) and crates/paladin-web/CHANGELOG.md (AgentAuthConfig field + OpenAPI scheme rename), one per file, per 14-01-SUMMARY.md's own D4 verification and this plan's own instruction to leave per-crate changelogs untouched. Both breaks are documented with a BREAKING entry and cite ADR-0040; only the single-file grep count in the plan's acceptance criteria was miscalibrated.",
    "status": "open",
    "reason": "",
    "recorded_at": "2026-08-12T18:05:57.086Z",
    "resolved_at": null
  },
  {
    "id": 8,
    "kind": "unrun-verify",
    "phase": "15.1",
    "file": "SECURITY-EXCEPTIONS.md",
    "line": null,
    "description": "Plan 15.1-01 Task 2's inline verify python one-liner (block-split regex over the machine-readable register) fails with a pre-existing TOML parse error on the LAST exception block, because its lookahead doesn't stop before the trailing markdown code fence -- reproduced against the pre-edit file too, unrelated to this task's new row. Substituted an isolated per-block parse of just the new RUSTSEC-2026-0249 row (11/11 fields present) plus the real repo guard scripts/check-advisory-register.sh (exit 0) as equivalent proof.",
    "status": "open",
    "reason": "",
    "recorded_at": "2026-08-14T00:49:21.261Z",
    "resolved_at": null
  },
  {
    "id": 9,
    "kind": "unrun-verify",
    "phase": "15.1",
    "file": ".github/workflows/ci.yml",
    "line": null,
    "description": "Task 1 acceptance criterion 'git diff | grep -c \"^[+-].*cargo \"' returns 4 not 0 -- matches step *name* text ('Cache cargo registry' etc.) removed by the migration, not actual cargo invocations. Verified via 'run: cargo' scoped grep returning 0 changed invocations.",
    "status": "open",
    "reason": "",
    "recorded_at": "2026-08-14T14:22:48.884Z",
    "resolved_at": null
  },
  {
    "id": 10,
    "kind": "unrun-verify",
    "phase": "15.1",
    "file": ".github/workflows/integration-tests.yml",
    "line": null,
    "description": "Task 2's first automated verify literally asserts survivors=={pre-commit.yml} after migration, but integration-tests.yml (3 hand-rolled cache blocks) is still present -- deletion is plan 15.1-05's job, not yet executed in this wave, exactly per this plan's own Recorded discretion resolutions section. Substituted an assertion expecting survivors=={pre-commit.yml, integration-tests.yml}, both counts matching (1 and 3 respectively).",
    "status": "open",
    "reason": "",
    "recorded_at": "2026-08-14T14:22:56.039Z",
    "resolved_at": null
  },
  {
    "id": 11,
    "kind": "unrun-verify",
    "phase": "15.1",
    "file": ".github/workflows/ci.yml",
    "line": null,
    "description": "Task 2 acceptance criterion 'grep -rc restore-keys ci.yml feature-flags.yml release.yml' returns 0 for ci.yml -- returns 2, both from pre-existing prose comments in the examples job (added by plan 15.1-01, lines ~268/271) explaining why a restore-keys fallback alone is insufficient, not an actual YAML restore-keys: key. Verified via structural YAML walk: no step's with block contains a restore-keys key in any of the three files.",
    "status": "open",
    "reason": "",
    "recorded_at": "2026-08-14T14:23:03.057Z",
    "resolved_at": null
  },
  {
    "id": 12,
    "kind": "unrun-verify",
    "phase": "17",
    "file": "tests/integration/ollama_docker_test.rs",
    "line": null,
    "description": "Ollama Docker-gated Tier 2 suite (17-07 Task 2) authored and proven to compile/clippy-clean/skip-gracefully, but never run against a real Ollama server -- no Docker daemon in the execution sandbox. Runtime behavior (generate/generate_stream/get_available_models/validate_model against real qwen2.5:0.5b) is unverified.",
    "status": "open",
    "reason": "",
    "recorded_at": "2026-08-17T14:17:30.134Z",
    "resolved_at": null
  },
  {
    "id": 13,
    "kind": "unrun-verify",
    "phase": "17",
    "file": "Makefile",
    "line": null,
    "description": "17-07 Task 3: the workspace 82% line-coverage gate (make coverage) could not be run in this execution sandbox -- Redis (6380) and MinIO (9010) are unreachable because no Docker daemon is available, and the coverage target's own preflight fails fast on both. The coverage percentage with all six new adapters counted is UNMEASURED, not failing. cargo doc -p paladin-llm --no-deps (0 missing-docs warnings under the six new features) and a scoped clippy pass on touched targets were verified instead.",
    "status": "open",
    "reason": "",
    "recorded_at": "2026-08-17T14:17:37.112Z",
    "resolved_at": null
  },
  {
    "id": 14,
    "kind": "deviation",
    "phase": "17",
    "file": "docker/docker-compose.test.yml",
    "line": null,
    "description": "17-07 Task 2: ollama-test healthcheck uses 'ollama list' (native /api/tags) instead of the plan's preferred curl-based /v1/models check, because curl/wget availability in the ollama/ollama:0.3.14 base image could not be verified without Docker in this sandbox. 'ollama list' is a well-precedented dependency-free healthcheck for this exact image. Compose file syntax validated via python yaml.safe_load only -- 'docker compose config' itself was never run.",
    "status": "open",
    "reason": "",
    "recorded_at": "2026-08-17T14:17:46.408Z",
    "resolved_at": null
  },
  {
    "id": 15,
    "kind": "unrun-verify",
    "phase": "17",
    "file": "crates/paladin-llm/src/gemini/adapter.rs",
    "line": null,
    "description": "Snyk code scan (per snyk_rules.instructions.md) could not be run — no Snyk MCP tool or CLI available in this worktree's runtime (no network egress); recorded as not-run, never as passed",
    "status": "open",
    "reason": "",
    "recorded_at": "2026-08-17T19:33:52.477Z",
    "resolved_at": null
  },
  {
    "id": 16,
    "kind": "unrun-verify",
    "phase": "17",
    "file": "crates/paladin-llm/src/compat/engine.rs,crates/paladin-llm/src/kimi/adapter.rs,crates/paladin-llm/src/qwen/adapter.rs,crates/paladin-llm/src/grok/adapter.rs,crates/paladin-llm/src/ollama/adapter.rs,crates/paladin-llm/src/gemini/adapter.rs",
    "line": null,
    "description": "Plan 17-10 verification step 7 (Snyk code scan over the five modified WR-04 adapter files plus compat/engine.rs) was not run — snyk_code_scan MCP tool unavailable in the executor runtime",
    "status": "open",
    "reason": "",
    "recorded_at": "2026-08-17T20:01:16.281Z",
    "resolved_at": null
  }
]
````
