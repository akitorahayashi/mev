---
label: "refacts"
---

## Goal

Replace all instances of generic `Box<dyn std::error::Error>` with explicitly typed boundary and domain errors (`AppError` in `mev`, `DomainError` in `mev-internal`) to enforce typed error invariants.

## Current State

The codebase frequently uses `Box<dyn std::error::Error>` to propagate errors, which erases semantic meaning, prevents matching on specific errors upstream, and violates the architectural rule: "All domain and boundary errors must use explicit typed errors."

- `src/app/container.rs`: `DependencyContainer::new()` and `for_identity()` return `Result<Self, Box<dyn std::error::Error>>` instead of `AppError`.
- `src/adapters/ansible/executor.rs`: `AnsibleAdapter::new()` and `load_catalog()` use `Box<dyn std::error::Error>` instead of `AppError`.
- `crates/mev-internal/src/domain/error.rs`: This file does not exist. There is no typed error representing domain and boundary errors for the `mev-internal` crate.
- `crates/mev-internal/src/domain/repository_ref.rs`, `crates/mev-internal/src/domain/repo_target.rs`, `crates/mev-internal/src/domain/submodule_path.rs`, `crates/mev-internal/src/domain/label_catalog.rs`: These domain rules return `Box<dyn std::error::Error>`.
- `crates/mev-internal/src/app/commands/git/delete_submodule.rs`, `crates/mev-internal/src/app/commands/gh/labels_deploy.rs`, `crates/mev-internal/src/app/commands/gh/labels_reset.rs`: CLI commands return `Box<dyn std::error::Error>`.
- `crates/mev-internal/src/app/cli/gh.rs`, `crates/mev-internal/src/app/cli/git.rs`: CLI parsers return `Box<dyn std::error::Error>`.
- `crates/mev-internal/src/adapters/gh.rs`, `crates/mev-internal/src/adapters/git.rs`, `crates/mev-internal/src/adapters/process.rs`: System adapters return `Box<dyn std::error::Error>`.

## Plan

1. Write `crates/mev-internal/src/domain/error.rs` containing a typed enum `DomainError` that implements `std::error::Error` and `std::fmt::Display`. It should derive `Debug` and encapsulate the various boundary errors encountered in the internal crate (e.g., IO, JSON parsing, validation errors, subprocess failures).
2. Read the file `crates/mev-internal/src/domain/error.rs` to verify the new error module was written correctly.
3. Register the `error` module in `crates/mev-internal/src/domain/mod.rs` (`pub mod error;`).
4. Replace all uses of `Box<dyn std::error::Error>` with `crate::domain::error::AppError` in `src/app/container.rs` by mapping any underlying creation errors.
5. Replace all uses of `Box<dyn std::error::Error>` with `AppError` in `src/adapters/ansible/executor.rs`.
6. Update `crates/mev-internal/src/domain/repository_ref.rs`, `crates/mev-internal/src/domain/repo_target.rs`, `crates/mev-internal/src/domain/submodule_path.rs`, and `crates/mev-internal/src/domain/label_catalog.rs` to replace `Box<dyn std::error::Error>` with the newly created `DomainError`.
7. Update `crates/mev-internal/src/adapters/gh.rs`, `crates/mev-internal/src/adapters/git.rs`, and `crates/mev-internal/src/adapters/process.rs` to return `DomainError` instead of `Box<dyn std::error::Error>`.
8. Update `crates/mev-internal/src/app/cli/gh.rs`, `crates/mev-internal/src/app/cli/git.rs`, `crates/mev-internal/src/app/commands/git/delete_submodule.rs`, `crates/mev-internal/src/app/commands/gh/labels_deploy.rs`, and `crates/mev-internal/src/app/commands/gh/labels_reset.rs` to return `DomainError` from their run functions.
9. Delete the processed requirement file (`.jules/exchange/requirements/replace_generic_errors.md`).
10. Run all relevant tests (e.g., `cargo test` and `cd crates/mev-internal && cargo test`) to ensure the changes are correct and have not introduced regressions.
11. Complete pre-commit steps to ensure proper testing, verification, review, and reflection are done.

## Acceptance Criteria

- All specified files use explicit typed errors instead of `Box<dyn std::error::Error>`.
- The codebase compiles with the updated explicit errors.
- Tests pass, validating behavior across boundaries.

## Risks

- Defining `DomainError` with missing variant variants could necessitate broader changes.
- Conversion traits mapping underlying libraries (e.g., `std::io::Error`, `serde_json::Error`) into typed errors may introduce friction or boilerplate.
