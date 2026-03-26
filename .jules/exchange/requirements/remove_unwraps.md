---
label: "refacts"
implementation_ready: false
---

## Goal

Ensure all production code uses explicit typed errors or safely falls back using established error mechanisms, rather than relying on `unwrap()` or `expect()`.

## Problem

The codebase heavily uses `.unwrap()` or `.expect()` in non-test paths (e.g., parsing env variables, locator/executor directories), presenting a risk of unhandled panics and silent failures in critical execution paths.

## Evidence

- source_event: "remove_unwraps_rustacean.md"
  path: "src/adapters/ansible/locator.rs"
  loc: "98, 99"
  note: "Locator functions use unwrap() to create directories or write files directly, bypassing error handling."

## Change Scope

- `src/adapters/ansible/locator.rs`
- `src/adapters/ansible/executor.rs`
- `crates/mev-internal/src/adapters/gh.rs`
- `crates/mev-internal/src/adapters/git.rs`

## Constraints

- Errors must be safely propagated, matched, or fallback mechanisms used. No unwraps on production paths.

## Acceptance Criteria

- `unwrap()` and `expect()` are removed from specified files and appropriately handled.