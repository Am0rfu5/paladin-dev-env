---
description: Drive a task list to completion using the Rust completion protocol
argument-hint: [path to tasks-*.md]
---

# Task List Management (Rust)

Manage and execute the task list to track progress on a PRD. Work the list one sub-task at a
time, stopping for go-ahead after each **parent** task.

Task list: $ARGUMENTS  (if empty, ask which `project/tasks-*.md` to work)

## Task Implementation

1. Before starting, check which sub-task is next.
2. When you finish a **sub-task**, immediately mark it `[ ]` → `[x]` in the file.
3. When **all** sub-tasks under a parent are `[x]`, run the completion sequence:
   - **Test:** `cargo test` (or `cargo test -p <crate>`)
   - **Format:** `cargo fmt --check`
   - **Lint:** `cargo clippy -- -D warnings` and fix any warnings
   - **Only if all pass:** stage with `git add .`
   - **Clean up:** remove temp files, `dbg!`, stray `println!`, and scratch code
   - **Commit** with a conventional-commit message using multiple `-m` flags, e.g.:
     ```bash
     git commit -m "feat: add payment validation logic" \
       -m "- Validates card type and expiry" \
       -m "- Adds unit tests for edge cases" \
       -m "Related to T123 in PRD"
     ```
4. Once committed, mark the **parent task** `[x]`.
5. **Stop after each parent task and wait for the user's go-ahead.**

## Maintenance

- Keep the task list current: mark progress, add newly discovered tasks.
- Keep the **Relevant Files** section accurate — every file created/modified with a one-line
  description.

## Rust considerations

- All public items need rustdoc (`///` / `//!`).
- Handle `Result`/`Option` properly; avoid `unwrap()`/`expect()`/`panic!` in library code.
- Use `thiserror` for custom error types; prefer `&str` params when ownership isn't needed.
- Respect hexagonal boundaries: dependencies flow inward only.
- Run `cargo clippy` and resolve warnings before marking tasks complete.
