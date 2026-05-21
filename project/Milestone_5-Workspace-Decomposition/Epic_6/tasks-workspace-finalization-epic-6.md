## Relevant Files

- `.devcontainer/Dockerfile.dev` — Add GitHub CLI (`gh`) apt installation block (FR-2.0).
- `Cargo.toml` — Add `paladin-core` and `paladin-ports` to `[workspace.dependencies]` (OQ-1 resolution).
- `src/lib.rs` — Add any missing top-level re-exports found during the facade audit; add `pub mod prelude;` declaration (FR-1.1 – FR-1.3).
- `src/prelude.rs` — New file: curated `paladin::prelude` module with ~20 most commonly used types (FR-1.4 – FR-1.5).
- `.github/workflows/ci.yml` — Add `crate-isolation` matrix job; update `test` job to `--workspace`; upgrade deprecated `actions-rs/toolchain@v1`; update doc check to `--workspace` (FR-2.1 – FR-2.9).
- `.github/workflows/feature-flags.yml` — Update build and test commands to `--workspace` (FR-2.4).
- `scripts/benchmark-builds.sh` — New automation script for measuring clean and incremental build times (FR-3.1 – FR-3.2).
- `project/Milestone_5-Workspace-Decomposition/Epic_6/build-benchmarks.md` — New report committing raw timings and the improvement summary table (FR-3.3 – FR-3.5).

### Notes

- This task list implements [PRD: Workspace Finalization — Facade Crate, CI Pipeline, and Build Benchmarks](./prd-workspace-finalization-epic-6.md).
- All Rust-specific quality gates use `cargo test --workspace`, `cargo fmt --all -- --check`, and `cargo clippy --workspace -- -D warnings`.
- The devcontainer `gh` CLI installation (Task 1.0) is a **prerequisite for Task 4.0**. Complete it first and rebuild the container before attempting to trigger or monitor GitHub Actions workflows.
- The facade audit (Task 2.0) confirms that deep module paths (`use paladin::core::…`, `use paladin::infrastructure::…`, `use paladin::application::…`) are already covered by `pub mod core/infrastructure/application` in `src/lib.rs`. The audit focuses on top-level short-name re-exports (`use paladin::PaladinBuilder`, etc.) that must be present in `src/lib.rs` for the prelude and convenient consumer use.
- `cargo test --workspace` currently passes with 2533 tests and 0 failures. This is the baseline that must not regress.

## Instructions for Completing Tasks

**IMPORTANT:** As you complete each task, you must check it off in this markdown file by changing `- [ ]` to `- [x]`. This helps track progress and ensures you don't skip any steps.

Example:

- `- [ ] 1.1 Read file` → `- [x] 1.1 Read file` (after completing)

Update the file after completing each sub-task, not just after completing an entire parent task.

## Tasks

- [x] 0.0 Create feature branch
  - [x] 0.1 Run `git branch --show-current` and confirm you are on the branch that contains the merged state of Epics 1–5 (expected: `feature/milestone_5-epic_5-paladin-memory-extraction` or `main` if already merged).
  - [x] 0.2 Create and checkout the Epic 6 branch: `git checkout -b feature/milestone_5-epic_6-workspace-finalization`
  - [x] 0.3 Push the branch to origin: `git push -u origin feature/milestone_5-epic_6-workspace-finalization`

- [ ] 1.0 Install GitHub CLI in devcontainer
  - [x] 1.1 Open `.devcontainer/Dockerfile.dev` and locate the existing `apt-get install` block (ends with `&& rm -rf /var/lib/apt/lists/*`).
  - [x] 1.2 Add the following new `RUN` block **immediately after** the existing apt-get block and **before** the `ENV DEBIAN_FRONTEND=dialog` line:
    ```dockerfile
    # Install GitHub CLI (gh) — required for triggering and monitoring CI workflows
    RUN curl -fsSL https://cli.github.com/packages/githubcli-archive-keyring.gpg \
          -o /usr/share/keyrings/githubcli-archive-keyring.gpg \
     && chmod go+r /usr/share/keyrings/githubcli-archive-keyring.gpg \
     && echo "deb [arch=$(dpkg --print-architecture) signed-by=/usr/share/keyrings/githubcli-archive-keyring.gpg] https://cli.github.com/packages stable main" \
          | tee /etc/apt/sources.list.d/github-cli.list > /dev/null \
     && apt-get update \
     && apt-get install -y --no-install-recommends gh \
     && apt-get clean -y \
     && rm -rf /var/lib/apt/lists/*
    ```
  - [ ] 1.3 Rebuild the devcontainer: open the VS Code Command Palette (`Ctrl+Shift+P`), select **"Dev Containers: Rebuild Container"**, and wait for the build to complete.
  - [ ] 1.4 Verify installation inside the container: `gh --version` — confirm it prints a version string (e.g., `gh version 2.x.x`).
  - [ ] 1.5 Create a GitHub **Fine-Grained Personal Access Token** for devcontainer use:
    - Go to **GitHub → Settings → Developer settings → Personal access tokens → Fine-grained tokens** → "Generate new token".
    - Give it a descriptive name (e.g., `paladin-devcontainer-gh-cli`).
    - Set **Resource owner** to your personal account (`DF3NDR`).
    - Set **Repository access** to **"Only select repositories"** → choose `paladin-dev-env`.
    - Under **Repository permissions**, grant:
      - **Actions: Read and Write** — covers triggering workflows (`gh workflow run`) and reading run status/logs (`gh run watch`, `gh run list`, `gh run view --log-failed`).
      - **Metadata: Read** — automatically included; required for `gh repo view`.
    - No other permissions are needed. `read:org` is a classic PAT scope only — it does not exist in fine-grained tokens and is not required for this repository.
    - Set an expiration appropriate for your team's security policy (90 days is a reasonable default).
    - Copy the generated token — it will only be shown once.
  - [ ] 1.6 Authenticate `gh` inside the container using the PAT. **Type this command in the terminal and paste the token when prompted — do not paste the token into any other tool or text field:**
    ```bash
    gh auth login --hostname github.com --git-protocol https --with-token
    ```
    The command gives **no prompt** — it silently waits for stdin. Paste your PAT and press **Enter**, then **Ctrl+D** to signal end-of-input. It will authenticate and exit.

    > **Note on `--with-token` hanging:** The command blocks waiting for stdin input because it expects the token to be piped. It is not frozen — it is waiting. Pasting the token + Enter + Ctrl+D always unblocks it.
  - [ ] 1.7 Confirm authentication: run `gh auth status` — confirm it shows `Logged in to github.com` with the correct username and the `repo`, `workflow`, and `read:org` scopes listed.
  - [ ] 1.8 Smoke-test the CLI: run `gh repo view DF3NDR/paladin-dev-env` and confirm it returns the repository description without errors.

- [x] 2.0 Facade crate re-export audit and gap fill
  - [x] 2.1 Run the following command to collect every top-level `use paladin::<Symbol>` reference (single-segment names only) used in `examples/` and `tests/`:
    ```bash
    grep -rh "use paladin::" examples/ tests/ --include="*.rs" \
      | grep -oP "use paladin::\K[A-Z][A-Za-z]+" | sort -u
    ```
    This reveals all short-name symbols that consumers reference directly from the `paladin` crate root.
  - [x] 2.2 Open `src/lib.rs` and verify that every symbol found in step 2.1 appears in a `pub use …` statement at the crate root. Produce a checklist (can be a temporary scratch file or terminal output) with each symbol marked as covered or missing.
  - [x] 2.3 For any missing top-level symbol, add the appropriate `pub use <source_path>::<Symbol>;` line to `src/lib.rs` in the relevant section (Port Traits, Domain Entities, Builder Types, Error Types, etc.). Keep the existing section groupings.
  - [x] 2.4 Verify that `pub mod core`, `pub mod application`, and `pub mod infrastructure` are still declared in `src/lib.rs` — these are what make deep module paths like `use paladin::core::platform::container::paladin::Paladin` resolve correctly. Do not remove them.
  - [x] 2.5 Add `paladin-core` and `paladin-ports` to `[workspace.dependencies]` in the root `Cargo.toml` (e.g., `paladin-core = { path = "crates/paladin-core" }` and `paladin-ports = { path = "crates/paladin-ports" }`). This resolves OQ-1 from the PRD.
  - [x] 2.6 Run `cargo build --workspace` and confirm zero errors. Fix any compilation errors caused by the new re-exports before proceeding.
  - [x] 2.7 Run `cargo test --workspace 2>&1 | grep -E "^test result:|FAILED"` and confirm all results show `0 failed`. The total passing count must be ≥ 2533 (the baseline from Epics 1–5).
  - [x] 2.8 Run `cargo doc -p paladin --no-deps 2>&1 | grep -iE "warn|error"` — confirm no output (zero broken links or missing doc warnings).

- [x] 3.0 Create `paladin::prelude` convenience module
  - [x] 3.1 Create `src/prelude.rs` with the following structure. All types must be re-exported via `pub use crate::…` (not direct crate paths) so the prelude stays in sync with `src/lib.rs` automatically:
    ```rust
    //! Convenient re-exports of the most commonly used Paladin types.
    //!
    //! Import everything you need to build and run Paladin agents in one line:
    //!
    //! ```rust,no_run
    //! use paladin::prelude::*;
    //! ```
    //!
    //! This module re-exports the types used in the majority of Paladin programs.
    //! For less common types, import them directly from their source modules.

    pub use crate::{
        // Agent types
        Paladin, PaladinBuilder, PaladinConfig, PaladinData, PaladinError, PaladinStatus,
        // Battalion / orchestration types
        BattalionConfig, BattalionError, BattalionResult,
        Campaign, ChainOfCommand, CommanderBuilder, Formation, Phalanx,
        CouncilBuilder, GroveBuilder,
        // LLM port and request/response types
        LlmError, LlmPort, LlmRequest, LlmResponse,
        // Memory port types
        GarrisonError, GarrisonPort,
        SanctumError, SanctumPort,
        // Memory adapter types (always available)
        InMemoryGarrison, InMemorySanctum,
        // Tool / arsenal types
        Armament, ArsenalPort, ArsenalRegistry,
        // Execution result types
        PaladinResult, StopReason,
    };
    ```
  - [x] 3.2 Add `pub mod prelude;` to `src/lib.rs` in the module declarations section, immediately after the `pub mod infrastructure;` declaration.
  - [x] 3.3 Run `cargo build -p paladin` and confirm zero errors. If any type in the prelude is not yet exported at the `paladin` crate root, add the missing `pub use` line to `src/lib.rs` first (back to Task 2.3).
  - [x] 3.4 Write a compile-check doc test inside `src/prelude.rs` that uses `use paladin::prelude::*` and constructs at least one type (e.g., verifies `PaladinStatus::Idle` is accessible). Mark it `no_run` since it requires a live LLM port:
    ```rust
    //! ```rust,no_run
    //! use paladin::prelude::*;
    //! // Verify core types are in scope
    //! let _status = PaladinStatus::Idle;
    //! ```
    ```
  - [x] 3.5 Run `cargo doc -p paladin --no-deps 2>&1 | grep -iE "warn|error"` and confirm no output.
  - [x] 3.6 Run `cargo fmt --all -- --check` and run `cargo fmt --all` to fix any formatting differences introduced by the new file.
  - [x] 3.7 Run `cargo clippy -p paladin -- -D warnings` and resolve any warnings in the new prelude module.
  - [x] 3.8 Stage and commit: `git add src/lib.rs src/prelude.rs Cargo.toml` then commit with message `feat: add paladin::prelude module and fill facade re-export gaps (Task 2.0 & 3.0)`.

- [ ] 4.0 Upgrade GitHub Actions CI workflows for workspace
  - [ ] 4.1 Open `.github/workflows/ci.yml`. In the `lint` job, replace the `cargo clippy` step command from `cargo clippy --all-targets --all-features -- -D warnings` with `cargo clippy --workspace --all-targets --all-features -- -D warnings`.
  - [ ] 4.2 In the `lint` job, replace the `cargo doc` step command from `cargo doc --no-deps --document-private-items` with `cargo doc --workspace --no-deps 2>&1 | tee /tmp/doc-output.txt && ! grep -q "warning:" /tmp/doc-output.txt`.
  - [ ] 4.3 In the `test` job, replace `cargo test --lib --bins` with `cargo test --workspace --lib --bins` and replace `cargo test --doc` with `cargo test --workspace --doc`.
  - [ ] 4.4 In both the `lint` and `test` jobs, replace the `actions-rs/toolchain@v1` step with the modern equivalent:
    ```yaml
    - name: Install Rust toolchain
      uses: dtolnay/rust-toolchain@stable
      with:
        components: rustfmt, clippy
    ```
    For the `test` job matrix, replace with `dtolnay/rust-toolchain@${{ matrix.rust-version }}` (no `components` needed there).
  - [ ] 4.5 Add a new `crate-isolation` job to `ci.yml` using a matrix strategy. Place it after the `test` job definition. The job must:
    - Check out code with `actions/checkout@v4`
    - Install the Rust toolchain with `dtolnay/rust-toolchain@stable`
    - Restore the Cargo cache (copy the same `actions/cache@v3` block used in the `test` job)
    - Build the crate with default features: `cargo build -p ${{ matrix.crate }}`
    - Build the crate with no default features: `cargo build -p ${{ matrix.crate }} --no-default-features`
    - Build and test the crate with all features: `cargo build -p ${{ matrix.crate }} ${{ matrix.extra_flags }}` and `cargo test -p ${{ matrix.crate }} ${{ matrix.extra_flags }}`
    - Use this matrix:
      ```yaml
      strategy:
        fail-fast: false
        matrix:
          include:
            - crate: paladin-core
              extra_flags: ""
            - crate: paladin-ports
              extra_flags: ""
            - crate: paladin-battalion
              extra_flags: ""
            - crate: paladin-llm
              extra_flags: "--all-features"
            - crate: paladin-memory
              extra_flags: "--all-features"
            - crate: paladin
              extra_flags: ""
      ```
  - [ ] 4.6 Open `.github/workflows/feature-flags.yml`. In the `feature-matrix` job steps, find the `cargo build` and `cargo test` commands and add `--workspace` to both so that feature flag combinations are validated against all workspace members, not just the root crate.
  - [ ] 4.7 Validate the workflow YAML syntax locally: run `python3 -c "import yaml; yaml.safe_load(open('.github/workflows/ci.yml'))"` and `python3 -c "import yaml; yaml.safe_load(open('.github/workflows/feature-flags.yml'))"`. Both must exit with code 0.
  - [ ] 4.8 Stage the workflow changes: `git add .github/workflows/ci.yml .github/workflows/feature-flags.yml`
  - [ ] 4.9 Commit: `git commit -m "ci: upgrade GitHub Actions workflows for workspace multi-crate support (Task 4.0)"` — then push the branch: `git push`.
  - [ ] 4.10 Trigger the CI pipeline from inside the devcontainer: `gh workflow run ci.yml --ref feature/milestone_5-epic_6-workspace-finalization`. Then watch the run: `gh run watch --exit-status` (the `--exit-status` flag makes the command exit non-zero if any job fails).
  - [ ] 4.11 Check the results: `gh run list --workflow=ci.yml --limit=1`. Confirm all jobs (`lint`, `api-surface`, `test`, `integration-tests`, `crate-isolation`) show a `✓` (completed/success) status. If any job fails, inspect the logs with `gh run view <run-id> --log-failed` and fix the issue before proceeding.
  - [ ] 4.12 Trigger the feature-flags matrix: `gh workflow run feature-flags.yml --ref feature/milestone_5-epic_6-workspace-finalization` and confirm all matrix permutations pass with `gh run watch --exit-status`.

- [ ] 5.0 Produce build-time benchmark report
  - [ ] 5.1 Identify the pre-decomposition baseline commit: run `git log --oneline origin/main | head -20` and locate the last commit on `main` before any Milestone 5 Epic branches were merged. Record this commit SHA.
  - [ ] 5.2 Create `scripts/benchmark-builds.sh` as a bash script that:
    - Accepts no arguments (hardcodes the five scenarios from FR-3.1).
    - For each scenario, runs the command **three times**, capturing the `real` wall-clock time from `time cargo build …` using `{ time cargo build …; } 2>&1 | grep real`.
    - Prints the median of the three runs for each scenario.
    - Prints a markdown-formatted summary table at the end.
    - Make it executable: `chmod +x scripts/benchmark-builds.sh`.
  - [ ] 5.3 Run the workspace benchmark scenarios (current workspace state) and record the raw timings. Run each scenario three times and take the median:
    - Scenario A: `cargo clean && cargo build --workspace` (clean build)
    - Scenario B: `touch crates/paladin-core/src/lib.rs && cargo build --workspace` (core incremental)
    - Scenario C: `touch crates/paladin-llm/src/lib.rs && cargo build --workspace` (LLM adapter incremental)
    - Scenario D: `touch crates/paladin-memory/src/lib.rs && cargo build --workspace` (memory adapter incremental)
    - Scenario E: `cargo build -p paladin-battalion` after a `touch crates/paladin-battalion/src/lib.rs` (battalion-only incremental, no LLM/memory in dep tree)
  - [ ] 5.4 Check out the pre-decomposition baseline commit into a temporary worktree: `git worktree add /tmp/paladin-baseline <SHA>`. Run `cargo clean && cargo build` from `/tmp/paladin-baseline` for the equivalent scenarios (clean build + touch `src/lib.rs` for incremental). Record the raw timings. Remove the worktree when done: `git worktree remove /tmp/paladin-baseline`.
  - [ ] 5.5 Create `project/Milestone_5-Workspace-Decomposition/Epic_6/build-benchmarks.md` containing:
    - **Environment** section: CPU model (`lscpu | grep "Model name"`), RAM, OS, Rust toolchain version (`rustc --version`), date of measurement.
    - **Raw Timings** table: one row per scenario, three-run values plus median for both workspace and baseline.
    - **Summary Table**: scenario name, monolith median, workspace median, improvement percentage, meets ≥ 50% target (yes/no).
    - **Analysis**: one paragraph per scenario calling out any regressions with a root-cause explanation (per FR-3.4).
    - **Conclusion**: explicit statement of whether the ≥ 50% incremental improvement target was achieved overall, and any recommended follow-up actions if it was not (per FR-3.5).
  - [ ] 5.6 Run `cargo fmt --all -- --check` and fix any formatting issues.
  - [ ] 5.7 Stage and commit: `git add scripts/benchmark-builds.sh project/Milestone_5-Workspace-Decomposition/Epic_6/build-benchmarks.md` then `git commit -m "feat: add build-time benchmark report and automation script (Task 5.0)"`.

- [ ] 6.0 Final quality gates and commit
  - [ ] 6.1 Run `cargo build --workspace` — confirm `Finished` with zero errors.
  - [ ] 6.2 Run `cargo test --workspace 2>&1 | grep -E "^test result:|FAILED"` — confirm all results show `0 failed` and total passed count ≥ 2533.
  - [ ] 6.3 Run `cargo clippy --workspace -- -D warnings 2>&1 | grep -E "^error|^warning\["` — confirm no output.
  - [ ] 6.4 Run `cargo fmt --all -- --check` — confirm no output (clean). If there are diffs, run `cargo fmt --all` and re-check.
  - [ ] 6.5 Run `cargo doc --workspace --no-deps 2>&1 | grep -iE "warn|error"` — confirm no output.
  - [ ] 6.6 Verify the devcontainer SM-0 metric: run `gh --version` inside the container and confirm it outputs a valid version string.
  - [ ] 6.7 Verify SM-2 (prelude completeness): run `cargo doc -p paladin --no-deps --open` and visually confirm the `prelude` module page lists all types from FR-1.4 of the PRD.
  - [ ] 6.8 Stage all remaining changes: `git add -A`
  - [ ] 6.9 Final commit: `git commit -m "feat: workspace finalization — facade audit, prelude, CI upgrade, benchmarks (Epic 6)"` with the following body lines:
    - `-m "- Facade re-export audit: all use paladin::... paths verified and gaps filled"`
    - `-m "- Add paladin::prelude with 20+ commonly used types (FR-1.4)"`
    - `-m "- Install gh CLI in .devcontainer/Dockerfile.dev (FR-2.0)"`
    - `-m "- Upgrade ci.yml: crate-isolation matrix job, --workspace flags, dtolnay/rust-toolchain"`
    - `-m "- Upgrade feature-flags.yml: --workspace propagation"`
    - `-m "- Commit build-time benchmark report to Epic_6/build-benchmarks.md"`
    - `-m "- Add paladin-core and paladin-ports to [workspace.dependencies]"`
    - `-m "Closes Epic 6 — Milestone 5 Workspace Decomposition complete"`
  - [ ] 6.10 Push the branch: `git push`
  - [ ] 6.11 Trigger one final full CI run: `gh workflow run ci.yml --ref feature/milestone_5-epic_6-workspace-finalization && gh run watch --exit-status` — confirm all jobs green before considering this Epic complete.
