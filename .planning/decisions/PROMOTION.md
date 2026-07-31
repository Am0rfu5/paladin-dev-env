# ADR Conventions and Promotion

This file is the small shared index that every phase appending to `.planning/decisions/` reads
before writing an ADR. It answers four questions the numbering scheme, the required headings, and
the supersession mechanism raise once — so Phases 5, 7, 10 and 13 do not have to re-derive them.

## Numbering scheme

ADR files use a **flat, zero-padded, monotonic counter**: `NNNN-kebab-slug.md`.

Chosen over a phase-scoped prefix (`p01-…`, `p05-…`) because a phase prefix breaks the moment an
ADR is superseded by a *later* phase's ADR — the reader would have to know which phase number is
"newer" rather than just comparing the counter. A flat counter surviving Phases 1, 5, 7, 10 and 13
needs only one shared piece of state: the next free number, tracked below.

## Numbering index

Reserved for Phase 1 (this phase authors ADR-0005 only; 0001-0004 and 0006 are reserved slots for
the plans that follow in this same phase):

| Number | Slug | Subject |
|---|---|---|
| 0001 | `battalion-config` | `BattalionConfig` field set (RECON-02) |
| 0002 | `battalion-result` | `BattalionResult` field set (RECON-03) |
| 0003 | `formation-min-paladins` | Formation minimum Paladin count (RECON-04) |
| 0004 | `temperature-validation` | Provider-aware temperature range (RECON-05) |
| 0005 | `herald-trait` | `Herald` trait signature (RECON-06) |
| 0006 | `coverage-gate` | Project-wide test coverage gate (RECON-07) |

**Next free ADR number: 0007**

Phases 5, 7, 10 and 13 take the next free number from this line when they author further ADRs —
they do not need to `ls` the directory to find it. Each phase updates this line when it appends.

## Required heading set

Every ADR uses the following H2 headings, in this order:

- `## Status`
- `## Context`
- `## Decision`
- `## Considered Options`
- `## Code Locations`
- `## Code Conformance`
- `## Downstream Consumers`

`## Code Locations` and `## Considered Options` are **bulleted lists, never prose paragraphs** —
`.claude/gsd-core/bin/lib/adr-parser.cjs`'s `splitEntries` only yields structured entries from
bullet or numbered lines; a paragraph collapses into one opaque blob and defeats the whole point of
citable, checkable entries.

`## Code Conformance` and `## Downstream Consumers` have no synonym in `adr-parser.cjs`'s
`CANONICAL_HEADERS` table and land in the parser's `unmapped_headers` bucket. That is acceptable —
nothing currently consumes either field programmatically — but they are still required, since
`## Code Conformance` is D-03's contract (every ADR MUST carry a `conforms` / `must change` verdict)
and `## Downstream Consumers` names who reads the decision next.

## Supersession mechanism

Exactly one live ADR answers each question at any time. When a later ADR supersedes an earlier one:

- The **superseded ADR keeps its file** — it is never deleted or renamed.
- Its `## Status` body becomes the bare word `Superseded`, followed by a prose line naming the
  superseding ADR's number and the reason it no longer holds.
- The **superseding ADR** carries a `## Supersedes` line naming the ADR number it replaces.
- `adr-parser.cjs` recognises `superseded` as a status word (see `STATUS_REJECT_SET` /
  `parseStatusFromSections`), so a downstream consumer can mechanically tell a live ADR from a
  retired one without reading prose.

## Promotion procedure for existing ADR candidates

*(Appended by Task 2 of this plan — see the section below.)*
