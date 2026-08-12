---
schema_version: 1
open_count: 4
waived_count: 0
fixed_count: 1
total_count: 5
last_updated: 2026-08-12T16:51:08.832Z
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
  }
]
````
