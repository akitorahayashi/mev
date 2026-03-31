---
label: "refacts"
---

## Goal

Replace the generic `Box<dyn std::error::Error>` boundaries with an explicit domain typed error enum (e.g., `DomainError`) across `crates/mev-internal` to model explicit failure states.

## Current State

The internal crate (`crates/mev-internal`) and some adapters use `Box<dyn std::error::Error>` extensively for error handling, collapsing errors and making it difficult to handle specific error types at boundaries safely.

- `crates/mev-internal/src/domain/submodule_path.rs`: `validate_submodule_path` returns `Result<(), Box<dyn std::error::Error>>`
- `crates/mev-internal/src/domain/repository_ref.rs`: `from_repo_arg`, `from_remote_url`, `new`, and parsing functions return `Result<_, Box<dyn std::error::Error>>`
- `crates/mev-internal/src/domain/repo_target.rs`: `resolve_repo_ref` returns `Result<RepositoryRef, Box<dyn std::error::Error>>`
- `crates/mev-internal/src/domain/label_catalog.rs`: `load_bundled_labels` returns `Result<Vec<LabelSpec>, Box<dyn std::error::Error>>`
- `crates/mev-internal/src/adapters/process.rs`: `run_status` and `run_output` return `Result<_, Box<dyn std::error::Error>>`
- `crates/mev-internal/src/adapters/git.rs`: Adapter methods return `Result<_, Box<dyn std::error::Error>>`
- `crates/mev-internal/src/adapters/gh.rs`: Adapter methods return `Result<_, Box<dyn std::error::Error>>`
- `crates/mev-internal/src/app/commands/git/delete_submodule.rs`: `run` returns `Result<(), Box<dyn std::error::Error>>`
- `crates/mev-internal/src/app/commands/gh/labels_deploy.rs`: `run` returns `Result<(), Box<dyn std::error::Error>>`
- `crates/mev-internal/src/app/commands/gh/labels_reset.rs`: `run` returns `Result<(), Box<dyn std::error::Error>>`
- `crates/mev-internal/src/app/cli/gh.rs`: `run` and `run_labels` return `Result<(), Box<dyn std::error::Error>>`
- `crates/mev-internal/src/app/cli/git.rs`: `run` returns `Result<(), Box<dyn std::error::Error>>`

## Plan

1. Create a typed error enum in `crates/mev-internal/src/domain/error.rs` using `thiserror`. Provide error variants that wrap `std::io::Error`, `std::string::FromUtf8Error`, `serde_json::Error`, and specific domain variants for specific validation errors like `InvalidRepositoryRef`, `InvalidRemoteUrl`, `UnsupportedRemoteUrl`, `InvalidSubmodulePath`, `ProcessFailed`, etc. (e.g., `DomainError`).
2. Add `pub mod error;` to `crates/mev-internal/src/domain/mod.rs`.
3. Update modules in `crates/mev-internal/src/domain` to use `Result<_, crate::domain::error::DomainError>` instead of `Box<dyn std::error::Error>`.
4. Update `crates/mev-internal/src/adapters` (git.rs, gh.rs, process.rs) to return `crate::domain::error::DomainError`. Add error mapping where necessary.
5. Update commands in `crates/mev-internal/src/app/commands/` and `crates/mev-internal/src/app/cli/` to return the `crate::domain::error::DomainError` or handle it.
6. Run `cargo test` to verify no regressions and correct behavior.

## Acceptance Criteria

- Typed errors are used in `mev-internal` and boundaries instead of generic `Box<dyn std::error::Error>`.
- Error contexts are preserved when mapping to typed errors.
- `cargo test` passes.

## Risks

- Overly broad error types might just rename `Box<dyn Error>` instead of adding semantic value. Ensure specific error variants are added for known failure states.
- Changing error types might break callers, but since `mev-internal` is internal to `mev`, we control all the call sites in `app`.
