# Task List Management (Rust)

Guidelines for managing task lists in markdown files to track progress on completing a PRD for Rust projects.

## Task Implementation

- **Completion protocol:**
  1. When you finish a **sub‑task**, immediately mark it as completed by changing `[ ]` to `[x]`.
  2. If **all** subtasks underneath a parent task are now `[x]`, follow this sequence:
     - **First**: Run the full test suite with `cargo test`
     - **Check formatting**: Run `cargo fmt --check` to ensure code follows Rust style guidelines
     - **Run linter**: Run `cargo clippy` and address any warnings
     - **Only if all tests pass and checks succeed**: Stage changes (`git add .`)
     - **Clean up**: Remove any temporary files, debug prints (`dbg!`, `println!`), and temporary code before committing
     - **Commit**: Use a descriptive commit message that:
       - Uses conventional commit format (`feat:`, `fix:`, `refactor:`, etc.)
       - Summarizes what was accomplished in the parent task
       - Lists key changes and additions
       - References the task number and PRD context
       - **Formats the message as a single-line command using `-m` flags**, e.g.:

         ```bash
         git commit -m "feat: add payment validation logic" -m "- Validates card type and expiry" -m "- Adds unit tests for edge cases" -m "Related to T123 in PRD"
         ```
  3. Once all the subtasks are marked completed and changes have been committed, mark the **parent task** as completed.

## Rust-Specific Commands

| Purpose | Command |
|---------|---------|
| Run all tests | `cargo test` |
| Run tests with output | `cargo test -- --nocapture` |
| Run specific test | `cargo test test_name` |
| Run tests in specific module | `cargo test module_name::` |
| Check code compiles | `cargo check` |
| Format code | `cargo fmt` |
| Check formatting | `cargo fmt --check` |
| Run linter | `cargo clippy` |
| Run linter strictly | `cargo clippy -- -D warnings` |
| Build release | `cargo build --release` |
| Generate docs | `cargo doc --open` |

## Task List Maintenance

1. **Update the task list as you work:**
   - Mark tasks and subtasks as completed (`[x]`) per the protocol above.
   - Add new tasks as they emerge.

2. **Maintain the "Relevant Files" section:**
   - List every file created or modified.
   - Give each file a one‑line description of its purpose.
   - Follow Rust conventions for file organization:
     - `src/lib.rs` - Library crate root
     - `src/main.rs` - Binary crate entry point
     - `src/module_name.rs` or `src/module_name/mod.rs` - Module files
     - `tests/` - Integration tests
     - `benches/` - Benchmarks

## Rust Project Structure Notes

- **Unit tests** in Rust are typically placed in the same file as the code they test, inside a `#[cfg(test)]` module:
  ```rust
  #[cfg(test)]
  mod tests {
      use super::*;

      #[test]
      fn test_example() {
          // test code
      }
  }
  ```
- **Integration tests** go in the `tests/` directory at the project root.
- **Documentation tests** are written in doc comments and run with `cargo test`.

## AI Instructions

When working with Rust task lists, the AI must:

1. Regularly update the task list file after finishing any significant work.
2. Follow the completion protocol:
   - Mark each finished **sub‑task** `[x]`.
   - Run `cargo test`, `cargo fmt --check`, and `cargo clippy` before committing.
   - Mark the **parent task** `[x]` once **all** its subtasks are `[x]` and code passes all checks.
3. Add newly discovered tasks.
4. Keep "Relevant Files" accurate and up to date.
5. Before starting work, check which sub‑task is next.
6. After implementing a sub‑task, update the file and proceed with the next sub-task.
7. **Rust-specific considerations:**
   - Ensure all public items have documentation comments (`///` or `//!`).
   - Handle `Result` and `Option` types appropriately—avoid excessive `.unwrap()` in production code.
   - Use `thiserror` or similar for custom error types when appropriate.
   - Prefer `&str` over `String` for function parameters when ownership isn't needed.
   - Run `cargo clippy` and address warnings before marking tasks complete.
