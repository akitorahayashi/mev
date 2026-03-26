---
label: "tests"
---

## Goal

Ensure tests modifying global state use the `serial_test` crate (`#[serial]`) to avoid test flakiness and race conditions. Specifically, address `DirGuard` in `crates/mev-internal/src/testing/env_mock.rs` and update tests in `crates/mev-internal/tests/gh_contracts.rs`.

## Current State

- `crates/mev-internal/src/testing/env_mock.rs`: Contains a `DirGuard` structure that uses `env::set_current_dir` to modify the current working directory, which alters process-global state. This structure lacks documentation enforcing the use of `#[serial]` in tests, and does not wrap `set_current_dir` in `unsafe` blocks like `PathGuard` does to highlight thread unsafety.
- `crates/mev-internal/tests/gh_contracts.rs`: Uses `PathGuard` and has `#[serial(env_path)]` applied to its tests.

## Plan

1. Update `crates/mev-internal/src/testing/env_mock.rs` to add a doc comment to `DirGuard` explicitly stating that tests using it must be marked with `#[serial]`.
2. Wrap `env::set_current_dir` calls in `unsafe { ... }` blocks within `DirGuard` in `crates/mev-internal/src/testing/env_mock.rs` and add a comment explaining that `serial_test` ensures thread safety, matching the `PathGuard` pattern.
3. Update `crates/mev-internal/tests/gh_contracts.rs` to use plain `#[serial]` instead of `#[serial(env_path)]` on the tests, ensuring consistency across global state modifiers.
4. Run `cargo test` and `cd crates/mev-internal && cargo test` to verify the changes and ensure no regressions.
5. Complete pre-commit steps to ensure proper testing, verification, review, and reflection are done.

## Acceptance Criteria

- `DirGuard` is documented with a requirement to use `#[serial]`.
- `env::set_current_dir` calls in `DirGuard` are wrapped in `unsafe { ... }` with a thread-safety comment.
- Tests in `gh_contracts.rs` use `#[serial]`.
- All tests pass locally.

## Risks

- Forgetting to use `#[serial]` on a new test might introduce flakiness, but documentation mitigates this.
