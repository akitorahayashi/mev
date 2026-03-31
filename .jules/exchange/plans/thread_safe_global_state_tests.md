---
label: "tests"
---

## Goal

Ensure functions modifying global process state in tests are explicitly wrapped in `unsafe { ... }` blocks and annotated with `#[allow(unused_unsafe)]`, and test functions executing these paths are explicitly marked with `#[serial]`.

## Current State

Modifying global state in Rust is intrinsically unsafe in a multi-threaded context like `cargo test`. Memory specifically dictates: "When using functions that modify global process state (e.g., `env::set_current_dir`) in tests, explicitly wrap them in an `unsafe { ... }` block and annotate with `#[allow(unused_unsafe)]` to emphasize their thread-unsafe nature...". `PathGuard` and `DirGuard` are used in parallel test runs unless serialized.
- `crates/mev-internal/src/testing/env_mock.rs`: `DirGuard::new()` and `DirGuard::drop()` modify global process state (`env::set_current_dir`) without an `unsafe` block or `#[allow(unused_unsafe)]` annotation. `env::set_var` and `env::remove_var` in `PathGuard::new` and `PathGuard::drop` are used with `unsafe` but missing `#[allow(unused_unsafe)]` annotations.
- `crates/mev-internal/src/adapters/git.rs`: Tests in `crates/mev-internal/src/adapters/git.rs` use mock binaries and potentially global state (e.g., `git` bin location) but lack `#[serial]` markings.
- `crates/mev-internal/src/adapters/gh.rs`: Tests in `crates/mev-internal/src/adapters/gh.rs` lack `#[serial]` markings while relying on environmental mocking structures.

## Plan

1. Modify `crates/mev-internal/src/testing/env_mock.rs` to add `#[allow(unused_unsafe)]` and `unsafe` blocks:
   - In `DirGuard::new` around `env::set_current_dir(target_dir).unwrap();`.
   - In `DirGuard::drop` around `let _ = env::set_current_dir(&self.original_dir);`.
   - In `PathGuard::new` on the `unsafe` block around `env::set_var("PATH", new_path);`.
   - In `PathGuard::drop` on both `unsafe` blocks around `env::set_var("PATH", original);` and `env::remove_var("PATH");`.
2. Modify `crates/mev-internal/src/adapters/git.rs` test module:
   - Add `use serial_test::serial;` at the top of the test module.
   - Add `#[serial]` to all test functions in the test module.
3. Modify `crates/mev-internal/src/adapters/gh.rs` test module:
   - Add `use serial_test::serial;` at the top of the test module.
   - Add `#[serial]` to all test functions in the test module.
4. Run standard workspace tests using `cargo test` to ensure changes are correct and regressions have not been introduced.

## Acceptance Criteria

- `DirGuard` methods use `unsafe { ... }` blocks and `#[allow(unused_unsafe)]` for `env::set_current_dir`.
- `PathGuard` methods use `#[allow(unused_unsafe)]` for `env::set_var` and `env::remove_var`.
- Tests in `git.rs` and `gh.rs` are annotated with `#[serial]`.

## Risks

- Marking tests as serial may increase the overall test execution time.
