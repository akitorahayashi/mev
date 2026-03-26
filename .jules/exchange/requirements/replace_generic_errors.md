---
label: "refacts"
implementation_ready: false
---

## Goal

Replace all instances of generic `Box<dyn std::error::Error>` with explicitly typed boundary and domain errors (e.g., `AppError` in `mev`, `DomainError` in `mev-internal`) to enforce typed error invariants.

## Problem

The codebase frequently uses `Box<dyn std::error::Error>` across multiple layers, including application bounds, domain parsing, and adapter functions in both `mev` and `mev-internal`. This erases semantic meaning, making classification and upstream error matching impossible, and directly violates the architectural rule: "All domain and boundary errors must use explicit typed errors."

## Evidence

- source_event: "explicit_typed_errors_rustacean.md"
  path: "src/app/container.rs"
  loc: "40, 60"
  note: "Uses Box<dyn std::error::Error> as the return type for new() and for_identity()."
- source_event: "missing_explicit_domain_error_mev_data_arch.md"
  path: "src/app/container.rs"
  loc: "DependencyContainer::new()"
  note: "`DependencyContainer::new` returns `Result<Self, Box<dyn std::error::Error>>` instead of `Result<Self, AppError>`."
- source_event: "missing_explicit_domain_error_mev_internal_data_arch.md"
  path: "crates/mev-internal/src/domain/repository_ref.rs"
  loc: "from_repo_arg()"
  note: "`from_repo_arg` returns `Result<Self, Box<dyn std::error::Error>>` instead of a typed error."

## Change Scope

- `src/app/container.rs`
- `src/adapters/ansible/executor.rs`
- `crates/mev-internal/src/domain/error.rs`
- `crates/mev-internal/src/domain/repository_ref.rs`
- `crates/mev-internal/src/domain/repo_target.rs`
- `crates/mev-internal/src/domain/submodule_path.rs`
- `crates/mev-internal/src/domain/label_catalog.rs`
- `crates/mev-internal/src/app/commands/git/delete_submodule.rs`
- `crates/mev-internal/src/app/commands/gh/labels_deploy.rs`
- `crates/mev-internal/src/app/commands/gh/labels_reset.rs`
- `crates/mev-internal/src/app/cli/gh.rs`
- `crates/mev-internal/src/app/cli/git.rs`
- `crates/mev-internal/src/adapters/gh.rs`
- `crates/mev-internal/src/adapters/git.rs`
- `crates/mev-internal/src/adapters/process.rs`

## Constraints

- Ensure `AppError` is used in `mev`.
- Create and use `DomainError` in `mev-internal` if it doesn't exist, or use standard enums per boundary.

## Acceptance Criteria

- All specified files use explicit typed errors instead of `Box<dyn std::error::Error>`.
- Code compiles correctly with the updated typed errors.