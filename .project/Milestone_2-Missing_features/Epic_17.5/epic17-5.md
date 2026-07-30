# Epic 17.5 - CLI Consolidation Refactoring

## Current Situation

Based on the file references in the codebase:

### cli (Epic 10 - Armory CLI Tools)
- mod.rs - Original CLI module
- commands - Agent, Battalion, Arsenal, Maneuver commands
- config - Configuration loading (paladin_config.rs, battalion_config.rs)
- output - Error handling and formatting
- templates - Template generation
- interactive.rs - Interactive utilities
- user_commands.rs - User management commands

### cli (Epic 18 - CLI Enhancement)
- mod.rs - New CLI module
- commands - Onboarding, SetupCheck, Features, Muster, Council
- error.rs - CLI error types
- formatters - Rich output formatters
- interactive - Wizard framework
- templates - Environment templates

## Problems with Current Structure

1. **Duplicate Functionality**: Both have `commands/`, `templates/`, and interactive utilities
2. **Inconsistent Import Paths**: Some commands use `paladin::cli::`, others use `paladin::application::cli::`
3. **Violates DRY**: Output formatting exists in both formatter.rs and output.rs
4. **Confusing Organization**: Developers won't know where to add new CLI features

## Recommended Consolidation: Choose cli

**Rationale:**
- **Hexagonal Architecture**: CLI is an **input adapter** in the application layer, not infrastructure
- **Epic 18 Foundation**: The new structure in cli has richer formatters, wizard framework, and better error handling
- **Future-Proof**: The application layer structure is more maintainable and extensible

## Consolidation Plan

Merge cli into cli with this structure:

```
src/application/cli/
├── mod.rs                          # Main CLI module
├── error.rs                        # CLI error types (keep from application/cli)
├── commands/
│   ├── mod.rs
│   ├── agent.rs                    # MOVE from src/cli/commands/agent.rs
│   ├── battalion.rs                # MOVE from src/cli/commands/battalion.rs
│   ├── arsenal.rs                  # MOVE from src/cli/commands/arsenal.rs
│   ├── maneuver.rs                 # MOVE from src/cli/commands/maneuver.rs
│   ├── onboarding.rs               # KEEP (Epic 18)
│   ├── setup_check.rs              # KEEP (Epic 18)
│   ├── features.rs                 # KEEP (Epic 18)
│   ├── muster.rs                   # KEEP (Epic 18)
│   └── council.rs                  # KEEP (Epic 18)
├── config/
│   ├── mod.rs
│   ├── loader.rs                   # MOVE from src/cli/config/loader.rs
│   ├── paladin_config.rs           # MOVE from src/cli/config/paladin_config.rs
│   └── battalion_config.rs         # MOVE from src/cli/config/battalion_config.rs
├── formatters/                     # KEEP (Epic 18 - richer implementation)
│   ├── mod.rs
│   ├── output.rs
│   ├── table.rs
│   └── progress.rs
├── interactive/                    # KEEP (Epic 18 - wizard framework)
│   ├── mod.rs
│   ├── prompts.rs
│   └── wizard.rs
├── templates/
│   ├── mod.rs
│   ├── paladin_template.rs         # MOVE from src/cli/templates/
│   ├── battalion_template.rs       # MOVE from src/cli/templates/
│   └── env.rs                      # KEEP (Epic 18)
└── user_commands.rs                # MOVE from src/cli/user_commands.rs
```

## Migration Steps

1. **Move command files** from commands to commands
2. **Merge formatters**: Keep Epic 18's richer formatters, delete formatter.rs
3. **Consolidate error handling**: Use error.rs, migrate from errors.rs
4. **Move config loading**: Transfer config to `src/application/cli/config/`
5. **Update imports**: Change all `use paladin::cli::` to `use paladin::application::cli::`
6. **Update paladin-cli.rs**: Change imports to use `paladin::application::cli::`
7. **Remove cli**: Delete the entire directory after migration
8. **Run tests**: Ensure `cargo test`, `cargo clippy`, and `cargo fmt --check` pass

## Update lib.rs

Change from:
```rust
pub mod cli;
pub mod application;
```

To:
```rust
pub mod application;
// CLI is now part of application layer
```
