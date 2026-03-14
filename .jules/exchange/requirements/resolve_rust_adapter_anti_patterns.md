---
label: "refacts"
implementation_ready: false
---

## Goal

Expose and remove unhandled fallbacks (`unwrap_or`/`unwrap_or_default`) in boundary interactions or production code, enforcing the explicit propagation of errors. Highlight instances where ownership transfer or reference passing should replace reflexive cloning, minimizing unnecessary allocations.

## Problem

`unwrap_or` and `unwrap_or_default` are used in production paths (e.g., `src/adapters/ansible/executor.rs` and `src/app/commands/backup/mod.rs`), representing silent fallback behaviors. Scattered `.clone()` calls without a documented boundary rationale (lifetime, caching, threading, ergonomics) obscure the single source of truth and indicate potential architecture or ownership flaws.

## Context

Using `unwrap_or` or `unwrap_or_default` in production code can lead to silent failures when assumptions are violated. They should be replaced with explicit error handling (e.g., returning `Result`) to handle cases gracefully without masking the application state. Reflexive use of `.clone()` appease the borrow checker but often points to a lack of clear ownership semantics. They add overhead and can mask design problems that could be solved with better lifetimes or ownership transfer.

## Evidence

- source_event: "invariants_boundaries_rustacean.md"
  path: "src/adapters/ansible/executor.rs"
  loc: "AnsibleAdapter"
  note: "`code.unwrap_or(-1)` is used without typed propagation, masking potential absence of exit codes silently."
- source_event: "invariants_boundaries_rustacean.md"
  path: "src/adapters/ansible/locator.rs"
  loc: "locate_ansible_dir"
  note: "Multiple `unwrap_or_default()` calls on paths without propagating errors explicitly."
- source_event: "invariants_boundaries_rustacean.md"
  path: "src/app/commands/backup/mod.rs"
  loc: "format_numeric"
  note: "Using `unwrap_or` to silently handle missing conversions when serializing values."
- source_event: "ownership_allocations_rustacean.md"
  path: "src/adapters/ansible/executor.rs"
  loc: "AnsibleAdapter"
  note: "`self.tags_by_role.clone()` performs a deep clone of a HashMap without a stated rationale."
- source_event: "ownership_allocations_rustacean.md"
  path: "src/adapters/ansible/locator.rs"
  loc: "locate_ansible_dir"
  note: "Multiple `.clone()` calls on `embedded_dir` and `manifest_dir` without a clear boundary justification."
- source_event: "ownership_allocations_rustacean.md"
  path: "src/app/container.rs"
  loc: "DependencyContainer"
  note: "Clones `ansible_dir` and `local_config_root` when creating adapters, instead of passing references or taking ownership."

## Change Scope

- `src/adapters/ansible/executor.rs`
- `src/adapters/ansible/locator.rs`
- `src/app/commands/backup/mod.rs`
- `src/app/container.rs`

## Constraints

- Ensure all changes align with architecture and design rules.
- Maintain tests for all new logic.

## Acceptance Criteria

- The problem is fully resolved.
- Pre-commit checks and tests pass.
