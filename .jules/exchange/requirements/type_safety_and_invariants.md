---
label: "refacts"
implementation_ready: false
---

## Goal

Improve structural type safety by enforcing domain variants instead of stringly-typed errors and preserving the `ResolvedAnsibleDir` invariant through the application boundary.

## Problem

The application currently converts domain invariants (like `ResolvedAnsibleDir`) into base primitives too early, forcing redundant validation downstream. Additionally, it frequently uses stringly-typed variants (`AppError::Config(String)`) and broad boxed error types (`Box<dyn std::error::Error>`) at boundary endpoints, eroding type safety and making programmatic recovery difficult.

## Evidence

- source_event: "path_invariants_rustacean.md"
  path: "src/app/container.rs"
  loc: "43"
  note: "The ResolvedAnsibleDir is destructed into its parts."
- source_event: "path_invariants_rustacean.md"
  path: "src/app/container.rs"
  loc: "45"
  note: "A raw PathBuf is cloned and passed to AnsibleAdapter::new."
- source_event: "path_invariants_rustacean.md"
  path: "src/adapters/ansible/executor.rs"
  loc: "67"
  note: "AnsibleAdapter::new accepts a raw PathBuf."
- source_event: "stringly_typed_errors_rustacean.md"
  path: "src/domain/error.rs"
  loc: "17-25"
  note: "AppError variants like Config, Update, and Backup encapsulate String payloads instead of distinct inner types or enums."
- source_event: "stringly_typed_errors_rustacean.md"
  path: "src/app/container.rs"
  loc: "40, 60"
  note: "DependencyContainer::new and for_identity return Result<Self, Box<dyn std::error::Error>> instead of a specific domain error type."
- source_event: "stringly_typed_errors_rustacean.md"
  path: "src/adapters/ansible/executor.rs"
  loc: "69, 208"
  note: "AnsibleAdapter::new and load_catalog return Result<..., Box<dyn std::error::Error>>."

## Change Scope

- `src/app/container.rs`
- `src/adapters/ansible/executor.rs`
- `src/domain/error.rs`

## Constraints

- Implement structured error variations that replace the stringly-typed parameters without significantly bloating the error space.

## Acceptance Criteria

- `ResolvedAnsibleDir` is passed directly and preserved throughout the `DependencyContainer`.
- `AppError` variants use structured data rather than pure Strings.
- Functions explicitly return defined domain errors rather than boxed generic trait objects where applicable.
