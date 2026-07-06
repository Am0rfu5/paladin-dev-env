---
description: Generate a step-by-step task list from requirements/PRD into project/
argument-hint: [PRD path or feature description]
---

# Rule: Generating a Task List from User Requirements

## Goal

Create a detailed, step-by-step task list in Markdown that guides a developer through
implementing a feature, based on the requirements / PRD provided.

Input (PRD path or requirements): $ARGUMENTS

## Output

- **Format:** Markdown (`.md`)
- **Location:** `project/`
- **Filename:** `tasks-[feature-name].md`

## Process

1. **Receive Requirements:** Read the PRD/requirements referenced above.
2. **Analyze Requirements:** Identify functional requirements, user needs, and implementation
   scope. Map work onto the hexagonal layers and the relevant `crates/`.
3. **Phase 1 — Generate Parent Tasks:** Create the file and write the high-level tasks.
   **IMPORTANT: Always include task `0.0 Create feature branch` as the first task**, unless the
   user requests not to branch. Aim for ~5–10 total high-level tasks. Then **pause and ask the
   user to confirm** before expanding sub-tasks.
4. **Phase 2 — Generate Sub-Tasks:** After confirmation, break each parent task into actionable
   sub-tasks (write tests first per TDD).
5. **Identify Relevant Files:** List files to create/modify with one-line descriptions, including
   test files (Rust unit tests live in-file under `#[cfg(test)]`; integration tests in `tests/`).
6. **Generate Final Output:** Combine parent tasks, sub-tasks, relevant files, and notes.

## Output Format

```markdown
## Relevant Files

- `crates/<crate>/src/<module>.rs` - Brief description of why this file is relevant.
- `crates/<crate>/tests/<name>_test.rs` - Integration tests for the feature.

### Notes

- Rust unit tests live in the same file under `#[cfg(test)] mod tests { ... }`.
- Run tests with `cargo test` (or `cargo test -p <crate>` for a single crate).

## Tasks

- [ ] 0.0 Create feature branch
  - [ ] 0.1 Create and checkout a new branch (e.g., `git checkout -b feature/[name]`)
- [ ] 1.0 Parent Task Title
  - [ ] 1.1 [Sub-task]
  - [ ] 1.2 [Sub-task]
- [ ] 2.0 Parent Task Title
  - [ ] 2.1 [Sub-task]
```

## Target Audience

Assume the reader is a **junior developer** implementing the feature.
