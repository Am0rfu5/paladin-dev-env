# Milestone 1 cited status ledger

This file **supersedes** `REQUIREMENTS.md`'s `## Milestone 1 as-shipped ledger` section (D-17).
That section becomes a pointer to this file. Phases 5, 7, 10 and 13 each add a sibling ledger
(`milestone-02-03.md`, `milestone-04-06.md`, `milestone-07-08.md`, `milestone-09-12.md`) rather than
growing REQUIREMENTS.md further — REQUIREMENTS.md is already ~4,000 lines and five inline sets of
`file:line`-cited verdicts would make it unreadable.

**Primary key: the `REQ-*` requirement ID.** Outstanding task items are nested under the
requirement they belong to, not given their own identifiers — the ~40 outstanding Milestone-1 task
items are numbered positions inside `.project/` task-list files with no `REQ-*` key of their own,
so nesting them keeps this ledger joinable to `REQUIREMENTS.md` and `ROADMAP.md` without inventing
new IDs (D-18).

**Evidence bar.** A `satisfied` verdict requires a `file:line` citation **plus** a named passing
test, example, or command that exercises it. A `file:line` citation with nothing exercising it is
not `satisfied` — it gets its own verdict, `present, unproven` (D-19). This bar exists because "the
code exists" has already produced false-positive completions in this corpus: Milestone 4 Epic 3's
task list is fully checked while three CLI-only dependencies remain unconditional in library builds.

## Verdict legend

| Verdict | Meaning |
|---|---|
| `satisfied` | `file:line` citation **and** a named passing test, example, or command exercising it |
| `present, unproven` | `file:line` citation exists, but nothing exercises it |
| `genuinely outstanding` | No shipped code satisfies the requirement |
| `deferred with reason` | Explicitly deferred, with the deferring document and reason cited |
| `superseded by shipped code` | Shipped code answers the requirement differently than the ingested document specified, and the shipped answer is recorded as authoritative |

## Divergences — shipped code superseded an ingested requirement

> **This divergence is a documented non-goal that shipped anyway.** Epic 9 explicitly declared "no
> REPL or interactive shell" a non-goal (NG-7). An interactive REPL now ships. This is the corpus's
> own evidence for why nothing in this planning record is treated as locked — even an explicit,
> written non-goal was superseded by later work with no recorded decision reversing it.

| Requirement | Ingested position | Shipped position | Verdict |
|---|---|---|---|
| `REQ-cli-interactive-mode` (Epic 9 non-goal NG-7) | "No REPL or interactive shell" — explicitly out of scope | An interactive REPL ships in the Armory CLI | shipped as **an interactive REPL**, not the declared non-goal of no REPL at all; **superseded by shipped code** |

Plan 01-05 adds the other two known divergences (MCP Streamable-HTTP superseding the Milestone-1
PRD's specified SSE transport; Qdrant/Sanctum superseding the specified `sqlite-vss` extension) and
RECON-08's Epic 10 Task 7.0 disputed-completion row. The per-epic sections below are left as
headings for that and later plans to fill; this plan authors the header, the legend, and the one
divergence row above only.

## Epic 1 — Paladin Domain Foundation

*(Filled by a later plan in this phase.)*

## Epic 2 — Garrison Memory System

*(Filled by a later plan in this phase.)*

## Epic 3 — Arsenal Tool System

*(Filled by a later plan in this phase.)*

## Epic 4 — Battalion Orchestration

*(Filled by a later plan in this phase.)*

## Epic 5 — Commander Strategy Router

*(Filled by a later plan in this phase.)*

## Epic 6 — Provider Expansion

*(Filled by a later plan in this phase.)*

## Epic 7 — Citadel State Persistence

*(Filled by a later plan in this phase.)*

## Epic 8 — Herald Output Formatting

*(Filled by a later plan in this phase.)*

## Epic 9 — Armory CLI

*(Filled by a later plan in this phase.)*

## Epic 10 — Validation and Documentation

*(Filled by a later plan in this phase; carries RECON-08's Task 7.0 disputed-completion row.)*
