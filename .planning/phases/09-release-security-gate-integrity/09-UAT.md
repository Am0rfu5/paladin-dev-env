---
status: complete
phase: 09-release-security-gate-integrity
source: [09-01-SUMMARY.md, 09-02-SUMMARY.md, 09-03-SUMMARY.md, 09-04-SUMMARY.md, 09-05-SUMMARY.md, 09-06-SUMMARY.md, 09-07-SUMMARY.md]
started: 2026-08-15T17:44:37Z
updated: 2026-08-15T17:52:10Z
---

## Current Test

[testing complete]

## Tests

### 1. Cold Start Smoke Test (Docker build from clean state)
expected: Dockerfile.chef builds from scratch after the per-crate manifest enumeration was removed; image size budget holds and docker-compose comes up. CI run 31898200704 "Docker Build" job green end to end.
result: pass

### 2. Changelog guard executes on a real CI runner
expected: The "Check per-crate changelogs" step runs inside the cargo-deny job whose required status-check context is "License & Dependency Policy", and passes on a real GitHub Actions runner (not just structurally verified by grep in the sandbox). CI run 31898200704: step present, conclusion success.
result: pass
coverage_id: D4 (09-01)

### 3. Crate-name collision guard executes on a real CI runner
expected: The "Check crates.io package names" step runs inside the same required cargo-deny job, immediately after the changelog step, and passes on a real runner. CI run 31898200704: step present in that position, conclusion success.
result: pass
coverage_id: D4 (09-04)

### 4. Single Security Audit job, advisory-register guard in the required job
expected: Exactly one "Security Audit" job remains in CI (the duplicate was deleted), and the "Check advisory exception register" guard runs inside the required cargo-deny job. CI run 31898200704: one Security Audit job (success, running cargo-audit with exceptions from .cargo/audit.toml), and the register guard step green inside "License & Dependency Policy".
result: pass
coverage_id: D3 (09-06)

### 5. cargo audit and cargo deny pass against the reconciled configuration
expected: The reconciled deny.toml (10 suppressions) and .cargo/audit.toml (5 mirrored) actually satisfy the tools — which could not be run in the sandbox (crates.io HTTP 403). CI run 31898200704: "Run cargo-deny check" green in License & Dependency Policy, "Run cargo-audit (exceptions from .cargo/audit.toml)" green in Security Audit.
result: pass
coverage_id: D4 (09-07)

### 6. paladin-herald CHANGELOG.md created, matching sibling Keep-a-Changelog shape
expected: CHANGELOG.md exists, backfills the crate's creation history (66f6c4e), records ADR-0023's comfy-table/colored feature gating as a breaking default-features change
result: pass
source: automated
coverage_id: D1 (09-01)

### 7. scripts/check-changelogs.sh guard, demonstrably failable
expected: Clean tree exits 0; four negative-path invocations all exit non-zero
result: pass
source: automated
coverage_id: D2 (09-01)

### 8. Makefile check-changelogs target
expected: make -n check-changelogs wraps the guard
result: pass
source: automated
coverage_id: D3 (09-01)

### 9. Changelog guard failure modes demonstrated
expected: Missing changelog, multiple missing, exemption-by-field not by name, vacuous-pass resistance — all non-zero with verbatim transcripts
result: pass
source: automated
coverage_id: D5 (09-01)

### 10. SECURITY-EXCEPTIONS.md with ten fully-governed rows
expected: Machine-parseable delimited TOML block; 10 rows, 11 non-empty fields each, 5/5 class split, owner DF3NDR, future review_date, distinct compensating controls
result: pass
source: automated
coverage_id: D1 (09-02)

### 11. ADR-0024 authored in the ADR-0022/0023 shape
expected: No frontmatter, seven headings, verbatim in-session Cargo.lock liveness transcript as evidence
result: pass
source: automated
coverage_id: D2 (09-02)

### 12. Dockerfile.chef per-crate enumeration deleted
expected: Nine-manifest enumeration removed; crate coverage now structural via COPY crates ./crates; no added COPY/RUN lines
result: pass
source: automated
coverage_id: D1 (09-03)

### 13. ADR-0027 records the FR-01 supersession
expected: Seven-heading shape, D-16 branch taken, both upstream cargo-chef citations, not-measured evidence disclaimer
result: pass
source: automated
coverage_id: D2 (09-03)

### 14. Dockerfile and Dockerfile.server carry no manifest enumeration
expected: SEC-05 closed across the whole Docker surface
result: pass
source: automated
coverage_id: D3 (09-03)

### 15. .crate-names.txt allow-list created
expected: Eleven package names this project owns on crates.io, matching the tree's [package].name values exactly
result: pass
source: automated
coverage_id: D1 (09-04)

### 16. scripts/check-crate-names.sh bidirectional set-equality guard
expected: Clean tree exits 0; six negative-path invocations all exit 1; publish=false exemption by field
result: pass
source: automated
coverage_id: D2 (09-04)

### 17. Makefile check-crate-names target
expected: make -n check-crate-names wraps the guard
result: pass
source: automated
coverage_id: D3 (09-04)

### 18. ADR-0026 records the offline-guard decision
expected: Rejected live-query and dry-run-only alternatives with concrete reasons (HTTP 403, release-time-only detection), verbatim failing transcript, accepted residual cost
result: pass
source: automated
coverage_id: D5 (09-04)

### 19. Crate-name guard failure modes demonstrated
expected: Unlisted name, stale entry, emptied allow-list, missing allow-list, case-only variant, publish-field exemption flip, plus reordering pass-through; tree restored byte-identical after each
result: pass
source: automated
coverage_id: D6 (09-04)

### 20. All eleven manifests declare MIT OR Apache-2.0
expected: One distinct license expression under sort -u; cargo metadata and cargo check --workspace pass offline
result: pass
source: automated
coverage_id: D1 (09-05)

### 21. LICENSE renamed to LICENSE-MIT, LICENSE-APACHE added
expected: git mv preserves history; verbatim Apache text; README badge/section and Dockerfile.chef OCI label updated
result: pass
source: automated
coverage_id: D2 (09-05)

### 22. ADR-0025 records the licence posture
expected: Seven-heading shape, decision/approver/date, both branches recorded fairly; source documents annotated in place with original text retained
result: pass
source: automated
coverage_id: D3 (09-05)

### 23. deny.toml and .cargo/audit.toml reconciled to the register
expected: Ten live suppressions, four dead entries deleted, audit mirrors deny
result: pass
source: automated
coverage_id: D1 (09-06)

### 24. scripts/check-advisory-register.sh guard
expected: Three clauses, nine demonstrated failure modes, idempotent, order-insensitive
result: pass
source: automated
coverage_id: D2 (09-06)

### 25. SEC-01..SEC-05 closed in REQUIREMENTS.md with citations
expected: All five read [x], each with a closure note carrying a verbatim commit/file:line citation; SEC-01's note names both dependency auditors as not run in this environment
result: pass
source: automated
coverage_id: D1 (09-07)

### 26. All five SEC traceability rows read Complete
expected: Stale suppression arithmetic corrected at source in REQUIREMENTS.md and CONCERNS.md with original text retained; zero real content deletions
result: pass
source: automated
coverage_id: D2 (09-07)

### 27. Governance records advanced
expected: PROMOTION.md at "Next free ADR number: 0028" with four new index rows and a dated note explaining the jump; PROJECT.md gains four Key Decisions rows; ROADMAP.md Phase 12 carries a dated closure note
result: pass
source: automated
coverage_id: D3 (09-07)

## Summary

total: 27
passed: 27
issues: 0
pending: 0
skipped: 0
blocked: 0

## Gaps

[none yet]
