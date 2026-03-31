---
label: "tests"
---

## Goal

Ensure that the core command logic is covered by tests, verifying that commands parse arguments and execute the correct adapter methods or correctly handle errors.

## Current State

The CLI command implementation modules in `crates/mev-internal/src/app/commands/` have 0% test coverage.
- `crates/mev-internal/src/app/commands/gh/labels_deploy.rs`: This command orchestrates reading existing labels and creating/replacing labels via the GhAdapter, but the logic is untested.
- `crates/mev-internal/src/app/commands/gh/labels_reset.rs`: This command orchestrates listing and deleting labels via the GhAdapter, but the logic is untested.
- `crates/mev-internal/src/app/commands/git/delete_submodule.rs`: This command orchestrates the steps to delete a git submodule via the GitAdapter, but the logic is untested.

## Plan

1. Add a `#[cfg(test)] mod tests` module to `crates/mev-internal/src/app/commands/gh/labels_deploy.rs`.
   - Write test functions invoking `run(args)` testing different argument combinations, ensuring adapters mock their CLI dependencies properly using `env_mock::create_mock_bin`.
   - Ensure `#[serial]` is used on tests due to `PATH` environment modifications inside `env_mock::PathGuard`.
2. Add a `#[cfg(test)] mod tests` module to `crates/mev-internal/src/app/commands/gh/labels_reset.rs`.
   - Write test functions invoking `run(args)` testing different argument combinations, ensuring adapters mock their CLI dependencies properly using `env_mock::create_mock_bin`.
   - Ensure `#[serial]` is used on tests due to `PATH` environment modifications inside `env_mock::PathGuard`.
3. Add a `#[cfg(test)] mod tests` module to `crates/mev-internal/src/app/commands/git/delete_submodule.rs`.
   - Write test functions invoking `run(args)` testing different argument combinations, ensuring adapters mock their CLI dependencies properly using `env_mock::create_mock_bin`.
   - Ensure `#[serial]` is used on tests due to `PATH` environment modifications inside `env_mock::PathGuard`.
4. Run `cargo test` to verify the tests compile and pass.

## Acceptance Criteria

- Command logic correctly parsing arguments and orchestrating adapters is verified by tests.

## Risks

- Test environment modifications (PATH, current dir) may leak across tests if not properly managed using `#[serial]` and testing utilities.