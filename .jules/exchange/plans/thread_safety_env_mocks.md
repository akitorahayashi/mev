---
label: "tests"
---

## Goal

Ensure all test functions that rely on environment state modification explicitly denote thread-unsafety through the `#[serial]` macro and wrap the use of state-modifying mocks in `unsafe { ... }` with `#[allow(unused_unsafe)]`. Modify global process state functions to be properly encapsulated.

## Current State

Tests in the system are missing proper thread-safety constraints when using state-modifying mocks.
- `crates/mev-internal/src/app/commands/git/delete_submodule.rs`: `fails_on_invalid_submodule_path` is missing `#[serial]`. `deletes_submodule_successfully` uses `PathGuard` and `DirGuard` without `unsafe` encapsulation.
- `crates/mev-internal/tests/gh_contracts.rs`: `test_gh_labels_deploy` and `test_gh_labels_reset` use `PathGuard` without `unsafe` blocks.
- `crates/mev-internal/src/adapters/git.rs`: `remove_submodule_module_dir_removes_directory` lacks `#[serial]`.
- `src/app/commands/backup/system.rs`: `test_format_string` modifies global environment (`HOME`) directly instead of injecting the environment dependency.
- `crates/mev-internal/src/testing/env_mock.rs`: `DirGuard::new` and `DirGuard::drop` do not wrap their global state mutations (`env::set_current_dir`) in `unsafe` blocks.

## Plan

1. Update `crates/mev-internal/src/testing/env_mock.rs` to wrap global state mutations (`env::set_current_dir`) within `unsafe { ... }` and `#[allow(unused_unsafe)]` for `DirGuard::new` and `DirGuard::drop`.
2. Update `crates/mev-internal/src/app/commands/git/delete_submodule.rs` to add `#[serial]` to `fails_on_invalid_submodule_path`, and wrap the instantiations of `PathGuard` and `DirGuard` in `deletes_submodule_successfully` within `unsafe { ... }` with `#[allow(unused_unsafe)]`.
3. Update `crates/mev-internal/tests/gh_contracts.rs` to wrap `PathGuard` instantiations in `unsafe { ... }` with `#[allow(unused_unsafe)]` for `test_gh_labels_deploy` and `test_gh_labels_reset`.
4. Update `crates/mev-internal/src/adapters/git.rs` to add `#[serial]` to `remove_submodule_module_dir_removes_directory`.
5. Update `src/app/commands/backup/system.rs` to refactor `format_string` to accept `home_dir: &str` as an explicit parameter (dependency injection) to avoid reading and mutating the global `HOME` environment variable, updating corresponding tests and call sites (`format_value` etc.) to remove the unsafe environment mocking block in `test_format_string`.

## Constraints

- Code changes must adhere to the project's strict design principles, such as single responsibility and accurate domain modeling.
- Modifications should not inadvertently break unconnected tests or configurations.

## Acceptance Criteria

- All tests using environment mocks are correctly annotated with `#[serial]`.
- All instantiations of environment mock guards (`PathGuard`, `DirGuard`) are encapsulated within `unsafe { ... }` and `#[allow(unused_unsafe)]`.
- `format_string` uses dependency injection for `HOME` path instead of reading global environment.
- Tests pass deterministically.
