---
label: "tests"
created_at: "2024-05-18"
author_role: "auditor"
confidence: "high"
---

## Problem

Tests in `mev-internal` use global state mocking utilities (`env_mock::PathGuard` and `env_mock::DirGuard`) that modify `env::set_var("PATH", ...)` and `env::set_current_dir(...)`. Currently, some of these test cases are not explicitly annotated with `#[serial]` nor do they wrap the test bodies or guard initializations in `unsafe { ... }` blocks with `#[allow(unused_unsafe)]`, violating thread safety constraints when executed in a parallel test runner.

## Goal

Ensure all test functions that rely on environment state modification (via `env_mock`) explicitly denote thread-unsafety through the `#[serial]` macro and wrap the use of state-modifying mocks in `unsafe { ... }` with `#[allow(unused_unsafe)]`.

## Context

The rule `Testing Rule (Thread Safety Marking)` requires explicit marking of thread-unsafe environment modifications. The `mev-internal` testing utilities (`env_mock::PathGuard` and `env_mock::DirGuard`) modify global process state (environment variables and working directory), which creates race conditions if tests are run in parallel. Explicitly using `#[serial]` and `unsafe` highlights these risks.

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

## Change Scope

- `crates/mev-internal/tests/gh_contracts.rs`
- `crates/mev-internal/src/app/commands/git/delete_submodule.rs`
- `crates/mev-internal/src/adapters/git.rs`