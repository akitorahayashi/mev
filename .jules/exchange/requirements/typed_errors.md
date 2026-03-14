---
label: "refacts"
implementation_ready: false
---

## Goal

Emphasize the need for typed errors and meaningful context attachment instead of stringly-typed or dynamically dispatched errors, ensuring semantic meaning is preserved at boundaries.

## Problem

Errors are heavily collapsed into `String` or `Box<dyn std::error::Error>`, losing semantic meaning, domain classification, and preventing structured error handling downstream.

## Context

`AppError` variants like `InvalidProfile(String)`, `InvalidIdentity(String)`, `InvalidTag(String)`, `Config(String)`, `Update(String)`, and `Backup(String)` embed unstructured strings. Internal tools (`crates/mev-internal/src/adapters/process.rs`, `gh.rs`, `git.rs`, `domain/repository_ref.rs`, etc.) heavily rely on `Result<..., Box<dyn std::error::Error>>`.

## Evidence

- source_event: "typed_errors_rustacean.md"
  path: "src/domain/error.rs"
  loc: "AppError"
  note: "Enum variants use `String` directly to store error context, losing structured classification."
- source_event: "typed_errors_rustacean.md"
  path: "crates/mev-internal/src/adapters/process.rs"
  loc: "run_status"
  note: "Uses `Box<dyn std::error::Error>` for error returns, erasing the original type and semantics."
- source_event: "typed_errors_rustacean.md"
  path: "crates/mev-internal/src/adapters/git.rs"
  loc: "delete_submodule_worktree"
  note: "Uses `Box<dyn std::error::Error>` for error returns, erasing the original type and semantics."

## Change Scope

- `src/domain/error.rs`
- `crates/mev-internal/src/adapters/process.rs`
- `crates/mev-internal/src/adapters/git.rs`

## Constraints

- Ensure all changes align with architecture and design rules.
- Maintain tests for all new logic.

## Acceptance Criteria

- The problem is fully resolved.
- Pre-commit checks and tests pass.
