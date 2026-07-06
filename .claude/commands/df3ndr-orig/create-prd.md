---
description: Generate a Product Requirements Document (PRD) into project/
argument-hint: [feature description]
---

# Rule: Generating a Product Requirements Document (PRD)

## Goal

Create a detailed Product Requirements Document (PRD) in Markdown, based on the user's
feature request. The PRD must be clear, actionable, and suitable for a **junior developer**
to understand and implement.

Feature request: $ARGUMENTS

## Process

1. **Receive Initial Prompt:** Use the feature request above (ask the user for it if empty).
2. **Ask Clarifying Questions:** Before writing the PRD, you _must_ ask only the most essential
   clarifying questions (limit to 3–5 critical gaps). Focus on the "what" and "why", not the
   "how". Number questions (1, 2, 3) and list options as A, B, C, D so the user can reply with
   selections like "1A, 2C, 3B". Only ask when the answer isn't reasonably inferable. Mark your recommended choice as (Recommended)
3. **Generate PRD:** Incorporate the user's answers, then write the PRD using the structure below.
4. **Save PRD:** Save as `project/prd-[feature-name].md`.

## PRD Structure

1. **Introduction/Overview** — the feature, the problem it solves, the goal.
2. **Goals** — specific, measurable objectives.
3. **User Stories** — narratives describing usage and benefit.
4. **Functional Requirements** — numbered, clear, unambiguous ("The system must…").
5. **Non-Goals (Out of Scope)** — what this feature will _not_ include.
6. **Design Considerations (Optional)** — mockups, UI/UX, relevant components.
7. **Technical Considerations (Optional)** — constraints, dependencies, hexagonal-architecture
   placement (which crate/layer), integration points.
8. **Success Metrics** — how success is measured.
9. **Open Questions** — remaining clarifications.

## Final instructions

1. Do **NOT** start implementing the PRD.
2. Make sure to ask the user the clarifying questions first.
3. Improve the PRD using the user's answers before saving.
