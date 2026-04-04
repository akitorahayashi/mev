---
label: "refacts"
---

## Goal

Replace usages of `unwrap()` and `expect()` with explicit error propagation using `Result<(), Box<dyn std::error::Error>>` (for tests) or custom typed errors (for domain types) to adhere to the explicit error handling principles.

## Current State

Widespread use of `unwrap()` and `expect()` bypasses structured error handling and poses a panicking risk, especially in testing and domain models where errors should be clearly propagated and typed.

- `crates/mev-internal/src/testing/env_mock.rs`: Uses `unwrap()` and `expect()` heavily for file system and environment manipulation during testing.
- `crates/mev-internal/src/domain/repository_ref.rs`: Tests use `.expect()` extensively instead of explicitly propagating errors and using `?`.
- `src/testing/fs.rs`: Uses `.unwrap()` to strip path prefixes.

## Plan

1. In `crates/mev-internal/src/testing/env_mock.rs`, refactor functions to return `Result<T, Box<dyn std::error::Error>>` and bubble up the errors using `?` instead of `.unwrap()` and `.expect()`.
2. In `crates/mev-internal/src/domain/repository_ref.rs`, modify the test functions to return `Result<(), Box<dyn std::error::Error>>` and use the `?` operator instead of `.expect()`.
3. In `src/testing/fs.rs`, replace `.unwrap()` in `strip_prefix` with explicit error handling (e.g. returning an `AppError`).
4. Update call sites in tests to handle the new `Result` returns from `env_mock.rs`.

## Constraints

- Code changes must adhere to the project's strict design principles, such as single responsibility and accurate domain modeling.
- Modifications should not inadvertently break unconnected tests or configurations.
- Tests should use explicit error handling (e.g. `Result<(), Box<dyn std::error::Error>>`) and not `.unwrap()` or `.expect()`.

## Acceptance Criteria

- The core issues detailed in the problem statements are resolved.
- Required tests are written or passing after the change.
- The identified file paths in the change scope have been appropriately modified according to the goal.