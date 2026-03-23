---
label: "refacts"
---

## Goal

Ensure all domain and boundary errors use explicit typed errors instead of `Box<dyn std::error::Error>`, and eliminate silent `.unwrap_or_default()` fallbacks in favor of explicit error propagation.

## Current State

- `src/adapters/git/cli.rs`: `read_config` uses `.unwrap_or_default()` if `git config` fails, silently suppressing config fetch errors.
- `src/adapters/ansible/locator.rs`: `locate_ansible_dir_with` logic defaults silently to returning an empty string when `embedded_detail` materialization fails.
- `crates/mev-internal/src/domain/repository_ref.rs`: `from_repo_arg` and `from_remote_url` return `Result<Self, Box<dyn std::error::Error>>`, erasing the distinction between parsing and IO failures.
- `src/app/container.rs`: `DependencyContainer::new` and `DependencyContainer::for_identity` return dynamic errors instead of `AppError`.
- `tests`: Existing test modules in `src/adapters/ansible/locator.rs` and `crates/mev-internal/src/domain/repository_ref.rs` expect successful execution paths and do not test how the system reacts to these components explicitly failing and returning bounded errors instead of panicking or returning silent fallback data.
- `docs/architecture`: The architecture and contribution documentation do not explicitly outline the new typed `DomainError` in `mev-internal` or explicitly forbid `.unwrap_or_default()` fallbacks for adapters.

## Plan

1. In `src/adapters/git/cli.rs`, update `read_config` to return `Result<String, AppError>`, removing `.unwrap_or_default()` and explicitly propagating the error. Update `GitCli::get_identity` to handle the new return type.
2. In `src/adapters/ansible/locator.rs`, update `locate_ansible_dir_with` to propagate errors instead of swallowing embedded materialization failures with `.unwrap_or_default()` for `embedded_detail`.
3. In `crates/mev-internal/src/domain/error.rs`, create a new typed `DomainError` enum implementing `std::error::Error` and `std::fmt::Display`. Expose it via `crates/mev-internal/src/domain/mod.rs`.
4. In `crates/mev-internal/src/domain/repository_ref.rs`, replace all `Box<dyn std::error::Error>` return types with `DomainError`. Update downstream consumers to map or propagate this typed error.
5. In `src/app/container.rs`, update `DependencyContainer::new` and `DependencyContainer::for_identity` to return `Result<Self, AppError>`. Update `ansible_context` and `identity_context` in `src/app/api.rs` to not wrap `AppError` in another `AppError::Config`.
6. Add tests in `src/adapters/ansible/locator.rs` and `crates/mev-internal/src/domain/repository_ref.rs` that explicitly verify failure paths. Ensure error variants correspond correctly to `AppError` and `DomainError`. Update adapter contract tests where failures should no longer be silently swallowed.
7. Update `docs/architecture` to outline the explicit typing requirement using `DomainError` and strictly prohibit the use of silent fallbacks (like `.unwrap_or_default()`) for boundary IO fetching, surfacing them as explicit failures instead.

## Acceptance Criteria

- `read_config` and `locate_ansible_dir` do not use silent `unwrap_or_default()` and propagate errors explicitly.
- `from_repo_arg`, `from_remote_url`, and container initialization functions return typed errors rather than `Box<dyn Error>`.
- Tests accurately reflect failure bounds on error propagation instead of silent strings or boxes.
- Documentation defines explicit handling boundaries without imperative verbiage.

## Risks

- Call sites expecting `Box<dyn std::error::Error>` may fail to compile when `DomainError` is introduced, requiring cascading type updates across the internal boundary.
- Removing silent fallbacks in configuration parsing could surface legitimate environment issues that were previously ignored, causing command failures.