---
label: "tests"
implementation_ready: false
---

## Goal

Ensure all test functions that rely on environment state modification (via `env_mock`) explicitly denote thread-unsafety through the `#[serial]` macro and wrap the use of state-modifying mocks in `unsafe { ... }` with `#[allow(unused_unsafe)]`.

Ensure that global process state modification is fully encapsulated or clearly scoped to prevent thread-safety issues during testing, adhering to the "safety contract" requirements for `unsafe` blocks.

## Problem

Tests in `mev-internal` use global state mocking utilities (`env_mock::PathGuard` and `env_mock::DirGuard`) that modify `env::set_var("PATH", ...)` and `env::set_current_dir(...)`. Currently, some of these test cases are not explicitly annotated with `#[serial]` nor do they wrap the test bodies or guard initializations in `unsafe { ... }` blocks with `#[allow(unused_unsafe)]`, violating thread safety constraints when executed in a parallel test runner.

Global process state (`std::env::set_var`) is being modified within tests. This is inherently thread-unsafe and can cause unpredictable test failures if tests run concurrently, even if marked with `#[serial]` to prevent local execution overlaps, as other system states might interact with the environment variable modifications in unexpected ways.

## Context

The rule `Testing Rule (Thread Safety Marking)` requires explicit marking of thread-unsafe environment modifications. The `mev-internal` testing utilities (`env_mock::PathGuard` and `env_mock::DirGuard`) modify global process state (environment variables and working directory), which creates race conditions if tests are run in parallel. Explicitly using `#[serial]` and `unsafe` highlights these risks.

The `unsafe` block in `test_format_string` modifies `std::env::set_var("HOME", "/mock/home")`. While it has `#[serial]` and `#[allow(unused_unsafe)]`, modifying the environment is a process-global operation that poses a risk in Rust tests if not strictly managed, as tests default to running concurrently. This requires better isolation, ideally using dependency injection or a safe abstraction instead of mutating global state.

## Evidence

- path: "crates/mev-internal/src/app/commands/git/delete_submodule.rs"
  loc: "line 69"
  note: "Test `fails_on_invalid_submodule_path` is missing `#[serial]` although not directly using the guards, the module uses them."
- path: "crates/mev-internal/src/app/commands/git/delete_submodule.rs"
  loc: "lines 52-53"
  note: "Test `deletes_submodule_successfully` initializes `PathGuard` and `DirGuard` without wrapping the instantiation in `unsafe { ... }` with `#[allow(unused_unsafe)]`."
- path: "crates/mev-internal/tests/gh_contracts.rs"
  loc: "line 22"
  note: "`test_gh_labels_deploy` uses `PathGuard` without wrapping its instantiation in `unsafe { ... }` with `#[allow(unused_unsafe)]`."
- path: "crates/mev-internal/tests/gh_contracts.rs"
  loc: "line 43"
  note: "`test_gh_labels_reset` uses `PathGuard` without wrapping its instantiation in `unsafe { ... }` with `#[allow(unused_unsafe)]`."
- path: "crates/mev-internal/src/adapters/git.rs"
  loc: "line 164"
  note: "Test `remove_submodule_module_dir_removes_directory` uses `tempdir` without `#[serial]`, risking race conditions if other tests mutate global state concurrently."
- path: "src/app/commands/backup/system.rs"
  loc: "258"
  note: "Test function `test_format_string` mutates global environment `std::env::set_var` inside an `unsafe` block without strict safe encapsulation or dependency injection of the home directory path."

- path: "crates/mev-internal/src/testing/env_mock.rs"
  loc: "57, 69, 75"
  note: "Mocking functions modify global process state (`std::env::set_var`, `std::env::set_current_dir`) within `unsafe` blocks. Although annotated with `#[allow(unused_unsafe)]` and typically used with `#[serial]`, this modifies global environment state and could lead to flaky tests or un-encapsulated obligations for users."

## Change Scope

- `crates/mev-internal/tests/gh_contracts.rs`
- `crates/mev-internal/src/app/commands/git/delete_submodule.rs`
- `crates/mev-internal/src/adapters/git.rs`
- `src/app/commands/backup/system.rs`
- `crates/mev-internal/src/testing/env_mock.rs`

## Constraints

- Code changes must adhere to the project's strict design principles, such as single responsibility and accurate domain modeling.
- Modifications should not inadvertently break unconnected tests or configurations.

## Acceptance Criteria

- The core issues detailed in the problem statements are resolved.
- Required tests are written or passing after the change.
- The identified file paths in the change scope have been appropriately modified according to the goal.
