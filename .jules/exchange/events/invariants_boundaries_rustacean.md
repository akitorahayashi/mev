---
label: "refacts"
created_at: "2024-05-23"
author_role: "rustacean"
confidence: "high"
---

## Problem

`unwrap_or` and `unwrap_or_default` are used in production paths (e.g., `src/adapters/ansible/executor.rs` and `src/app/commands/backup/mod.rs`), representing silent fallback behaviors.

## Goal

Expose and remove unhandled fallbacks (`unwrap_or`/`unwrap_or_default`) in boundary interactions or production code, enforcing the explicit propagation of errors.

## Context

Using `unwrap_or` or `unwrap_or_default` in production code can lead to silent failures when assumptions are violated. They should be replaced with explicit error handling (e.g., returning `Result`) to handle cases gracefully without masking the application state.

## Evidence

- path: "src/adapters/ansible/executor.rs"
  loc: "AnsibleAdapter"
  note: "`code.unwrap_or(-1)` is used without typed propagation, masking potential absence of exit codes silently."

- path: "src/adapters/ansible/locator.rs"
  loc: "locate_ansible_dir"
  note: "Multiple `unwrap_or_default()` calls on paths without propagating errors explicitly."

- path: "src/app/commands/backup/mod.rs"
  loc: "format_numeric"
  note: "Using `unwrap_or` to silently handle missing conversions when serializing values."

## Change Scope

- `src/adapters/ansible/executor.rs`
- `src/adapters/ansible/locator.rs`
- `src/app/commands/backup/mod.rs`