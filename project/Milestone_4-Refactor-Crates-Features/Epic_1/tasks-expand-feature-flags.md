# Task List: Expand Feature Flags to Gate the Full Optional Surface

**Epic:** Milestone 4, Epic 1
**PRD:** `prd-expand-feature-flags.md`
**Status:** In Progress

---

## Relevant Files

> _To be completed after sub-tasks are generated._

---

## Instructions for Completing Tasks

**IMPORTANT:** As you complete each task, check it off by changing `- [ ]` to `- [x]`. Update after each **sub-task**, not just parent tasks.

Example:
- `- [ ] 1.1 Read file` → `- [x] 1.1 Read file`

Run the following after every parent task is completed before marking it `[x]`:
```bash
cargo test
cargo fmt --check
cargo clippy -- -D warnings
```

---

## Tasks

- [ ] 0.0 Create feature branch
- [ ] 1.0 Audit and classify all Cargo.toml dependencies
- [ ] 2.0 Implement LLM Provider feature flags (`llm-openai`, `llm-anthropic`, `llm-deepseek`, `llm-all`)
- [ ] 3.0 Implement Content Processing feature flag (`content-processing`)
- [ ] 4.0 Implement Web Server feature flag (`web-server`)
- [ ] 5.0 Implement Notifications feature flag (`notifications`)
- [ ] 6.0 Implement Vision feature flag (`vision`)
- [ ] 7.0 Implement MCP Arsenal feature flag (`mcp-arsenal`)
- [ ] 8.0 Revise default feature set and add `full` convenience flag
- [ ] 9.0 Configure CI feature flag matrix
- [ ] 10.0 Update documentation and examples

---

_I have generated the high-level tasks based on your requirements. Ready to generate the sub-tasks? Respond with **Go** to proceed._
