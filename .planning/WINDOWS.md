---
schema_version: 1
open_count: 0
waived_count: 0
fixed_count: 1
total_count: 1
last_updated: 2026-07-31T14:46:37.492Z
---

# Broken Windows Ledger

> Cross-phase defect register. `/gsd-ship` blocks while `open_count > 0`.
> Waive with `gsd-tools windows waive <id> "<reason>"` (reason required).
> Mark fixed with `gsd-tools windows fixed <id>`.

| id | phase | kind | file | line | description | status | reason | recorded_at | resolved_at |
|----|-------|------|------|------|-------------|--------|--------|-------------|-------------|
| 1 | 01 | unmet-truth | .planning/ledgers/milestone-01.md |  | REQ-battalion-result-v1 (Epic 4 FR-4.2, cited in ADR-0002's Considered Options as 'superseded by the shipped superset') has no row anywhere in the Milestone 1 ledger's Epic 4 table, even though REQUIREMENTS.md's original ledger body carried it as 'Variant (group 4)'. Plan 01-08 Task 2's subset-check safety gate caught this and HALTED per the plan's explicit instruction rather than reducing REQUIREMENTS.md's Milestone 1 body to a pointer at an incomplete destination. | fixed |  | 2026-07-31T13:22:57.385Z | 2026-07-31T14:46:37.492Z |

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
  }
]
````
