---
status: complete
phase: 19-crates-io-trusted-publishing-replace-the-long-lived-registry
source: [19-VERIFICATION.md]
started: 2026-08-27T19:21:34Z
updated: 2026-08-28T12:25:06Z
---

## Current Test

[testing complete]

## Tests

### 1. Confirm the crates.io token revocation on the registry side
expected: Token "Paladin" absent from the active API-token list; no other publish-scoped token present on the account. Optionally report the last-used timestamp and revocation time so the Revocation Ledger's "not reported" fields can be filled.
result: pass
verified_by: operator (Am0rfu5), 2026-08-28 — token "Paladin" absent, no other publish-scoped token

## Summary

total: 1
passed: 1
issues: 0
pending: 0
skipped: 0
blocked: 0

## Gaps
