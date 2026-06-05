# paladin-herald

Herald output-formatter adapters for the [Paladin](https://github.com/DF3NDR/paladin-dev-env)
framework.

The `Herald` trait (the output-formatting port) lives in `paladin-core`. This crate provides the
concrete rendering implementations and their presentation dependencies, keeping the pure domain
crate dependency-light:

- `JsonHerald` — structured JSON output
- `MarkdownHerald` — Markdown / ANSI-colored terminal output (via `colored`)
- `TableHerald` — compact table output (via `comfy-table`)

Each implements `paladin_core::platform::container::herald::Herald`.
