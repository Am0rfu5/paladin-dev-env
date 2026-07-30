# Milestones 8–11: Dependency Graph and Parallelization Guide

**Created:** 2026-05-29
**Document Version:** 1.0

---

## Milestone Summary

| Milestone | Title | Version | Est. Duration | Hard Dependencies |
|-----------|-------|---------|---------------|-------------------|
| **8** | Facade Cleanup & Shim Resolution | v0.2.0 | 3–4 sprints | None |
| **9** | Classic Orchestrator Completion | v0.3.0 | 6–10 sprints | Milestone 8 |
| **10** | CI Hardening & Release Automation | v0.4.0 | 2–3 sprints | None |
| **11** | Documentation Overhaul & Publish | v0.5.0 | 4–6 sprints | Milestones 8, 9 (partial) |

---

## Dependency Graph

```
                    ┌─────────────────┐
     ┌──────────────│  Milestone 8    │──────────────┐
     │              │ Facade Cleanup  │              │
     │              │ v0.2.0          │              │
     │              └────────┬────────┘              │
     │                       │                       │
     │              HARD DEP │              HARD DEP │ (paths stable)
     │                       │                       │
     │              ┌────────▼────────┐    ┌─────────▼────────┐
     │              │  Milestone 9    │    │  Milestone 11    │
     │              │ Orchestrator    │    │  Documentation   │
     │              │ v0.3.0          │    │  v0.5.0          │
     │              └────────┬────────┘    │                  │
     │                       │             │  Epics 1,2 start │
     │              HARD DEP │             │  immediately     │
     │              (APIs)   │             │                  │
     │                       │             │  Epics 3,4 wait  │
     │                       └─────────────│→ for M9 E1-E3    │
     │                                     └──────────────────┘
     │
     │  NO DEPENDENCY
     │
     │              ┌─────────────────┐
     └──(soft)──────│  Milestone 10   │
                    │ CI Hardening    │
                    │ v0.4.0          │
                    │                 │
                    │ Epics 1,2 start │
                    │ immediately     │
                    │                 │
                    │ Epic 3 waits    │
                    │ for M8 (soft)   │
                    └─────────────────┘
```

---

## Parallel Execution Plan

### Sprint-by-Sprint Recommended Schedule

Assumes a single developer or small team that can context-switch between milestones.

```
Sprint 1-2:  ┌─ M8 Epics 1-2 (Audit + Shim Removal) ─────────┐
             │  M10 Epics 1-2 (Pre-commit + Security Scanning) │
             │  M11 Epics 1-2 (Doc Audit + MDBook Setup)       │
             └──────────────────────────────────────────────────┘

Sprint 3-4:  ┌─ M8 Epics 3-5 (Relocate + Rename + Finalize) ──┐
             │  M10 Epic 3 (Release Automation)                 │
             └──────────────────────────────────────────────────┘
             → v0.2.0 release (M8)

Sprint 5-7:  ┌─ M9 Epic 1 (Orchestrator E2E) ──────────────────┐
             │  M10 Epic 4 (Finalize)                           │
             └──────────────────────────────────────────────────┘
             → v0.4.0 release (M10)

Sprint 7-10: ┌─ M9 Epics 2-4 (parallel: Queue/Scheduler,      ┐
             │   Content→Agent, Agent→Orchestrator)             │
             │  M11 Epics 3-4 (Rewrite + New Docs) ← starts    │
             │   once M9 E1-E3 APIs stable                     │
             └──────────────────────────────────────────────────┘

Sprint 10-12:┌─ M9 Epics 5-6 (User/Admin + Finalize) ─────────┐
             │  M11 Epic 5 (Publish + Finalize)                 │
             └──────────────────────────────────────────────────┘
             → v0.3.0 release (M9)
             → v0.5.0 release (M11)
```

### Two-Developer Parallel Track

If two developers (or dev + tech writer) are available:

```
Developer A (Code):            Developer B (Infra/Docs):
─────────────────              ──────────────────────────
Sprint 1-2: M8 Epics 1-3      M10 Epics 1-2
Sprint 3-4: M8 Epics 4-5      M10 Epic 3 + M11 Epics 1-2
            → v0.2.0           → v0.4.0 (M10)
Sprint 5-7: M9 Epic 1         M11 structural work
Sprint 7-10: M9 Epics 2-4     M11 Epics 3-4 (rewrite)
Sprint 10-12: M9 Epics 5-6    M11 Epic 5 (publish)
              → v0.3.0         → v0.5.0
```

---

## Version Release Sequence

Each milestone produces its own release. The version sequence:

| Release | Milestone | Gate Criteria |
|---------|-----------|---------------|
| **v0.2.0** | M8: Facade Cleanup | All shims resolved, `services/` rename complete, quality gates green |
| **v0.3.0** | M9: Orchestrator | Orchestrator E2E, both bridges working, user/admin auth functional |
| **v0.4.0** | M10: CI Hardening | Pre-commit hooks, security scanning, release automation operational |
| **v0.5.0** | M11: Documentation | MDBook published, all docs current, all code examples compile |

**Note:** v0.4.0 (M10) may release before v0.3.0 (M9) since M10 has no dependency on M9 and is smaller. The version numbers are logical targets, not strict ordering constraints. If M10 ships first, the version sequence becomes v0.2.0 → v0.4.0 → v0.3.0 → v0.5.0. If this is unacceptable, hold M10's tag until M9 ships.

**Alternative if strict ordering is preferred:** use v0.2.0 (M8), v0.2.1 (M10 — minor CI tooling, no API changes), v0.3.0 (M9 — new features), v0.4.0 (M11 — docs). This keeps feature releases on minor bumps and tooling on patches.

---

## Critical Path

The longest sequential chain determines the earliest possible completion:

```
M8 (3-4 sprints) → M9 (6-10 sprints) → M11 Epics 3-5 (2-3 sprints)
                                         ↑
                                   M11 Epics 1-2 start at Sprint 1
                                   (net addition: 0 sprints)
```

**Critical path: 11–17 sprints total.**

M10 is entirely off the critical path and can be completed during M8 or early M9 sprints.

M11 Epics 1–2 (audit + MDBook setup) are off the critical path — they execute during M8/M9 time. Only M11 Epics 3–5 (rewrite, new content, publish) extend the critical path, and only by the time they take after M9 APIs stabilize.

---

## Risk Mitigation for Parallel Execution

| Risk | Mitigation |
|------|-----------|
| M8 `services/` rename invalidates M9 import paths written concurrently | M9 work should not begin until M8 Epic 4 (rename) is complete |
| M10 release automation references crate names that M8 might change | M10 Epic 3 (release config) waits for M8 to finalize crate structure |
| M11 documents APIs that M9 changes during development | M11 Epics 3–4 use "document after merge" — only write about APIs that are merged to `main` |
| Developer context-switching between milestones reduces velocity | Batch milestone work in 2-sprint blocks; don't interleave within a single sprint |
| M9 scope creep delays all downstream milestones | M9 has a clear cut line: if User/Admin (Epic 5) isn't ready, ship v0.3.0 without it and defer to v0.3.1 |
