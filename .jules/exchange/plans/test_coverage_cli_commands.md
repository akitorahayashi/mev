---
label: "tests"
---

## Goal

Increase test coverage for CLI commands to guarantee command workflows, configuration modifications, and integrations function flawlessly under various usage scenarios, ensuring predictable execution flows and error handling.

## Current State

The test coverage for CLI commands in `src/app/commands/` and `crates/mev-internal/src/app/commands/` modules is noticeably deficient or entirely non-existent. This exposes users to hidden bugs in system state management and error propagation. Essential features remain untested, meaning breaking changes might inadvertently impact the end-user experience.
- `crates/mev-internal/src/app/commands/gh/labels_deploy.rs`: This file orchestrates the deployment of GitHub labels by reading local JSON catalogs and interacting with the `gh` CLI. It currently has 0% test coverage and is untested at the boundary.
- `src/app/commands/create/mod.rs`: Orchestrates the execution of a complete development environment setup using Ansible tags. It has minimal flag-level testing but lacks full execution flow validation.
- `src/app/commands/list/mod.rs`: Outputs the available tags, tag groups, and profiles to stdout. It has minimal help testing but does not assert on the actual output list.

## Plan

1. Create a mock CLI integration test harness in `crates/mev-internal/tests/harness/mod.rs` and `crates/mev-internal/tests/harness/test_context.rs` (mirroring `tests/harness/test_context.rs`). Ensure the `TestContext` modifies `$PATH` to point to a local `mocks` directory inside its temporary workdir, allowing tests to intercept external command calls like `gh` and `git` by writing simple bash scripts.
2. Add integration tests for `mev-internal` in `crates/mev-internal/tests/cli_gh_labels_deploy.rs` (or inside `crates/mev-internal/tests/cli/gh/labels_deploy.rs` and register it).
   - Test that `mev-internal gh labels deploy` reads from a mocked `gh label list --json` and correctly calls `gh label create` and `gh label delete` for each label defined in the bundled labels catalog.
   - Assert the outputs of these commands matching the expected stdout.
3. Add integration tests for `mev` in `tests/cli/create.rs`.
   - Use the existing `TestContext` and create a mock `ansible-playbook` bash script in the test's `$PATH`.
   - Test `mev create macbook` and verify that the output indicates a successful environment creation and that the mock `ansible-playbook` script was executed with the expected tags (since `ansible-playbook` outputs to stdout, the mock script can echo something specific or touch a file).
   - Test the invalid tags scenario by modifying the catalog or passing an invalid profile if possible, though `FULL_SETUP_TAGS` are hardcoded. A better test is ensuring the successful flow runs and outputs `✓ Environment created successfully!`.
4. Add integration tests for `mev` in `tests/cli/list.rs`.
   - Run `mev list` using `ctx.cli().args(["list"])`.
   - Assert that the stdout contains expected strings such as `Available Tags`, `Tag Groups (expanded automatically):`, `Profiles:`, and known tags like `brew-formulae`.

## Acceptance Criteria

- Code coverage for `crates/mev-internal/src/app/commands/gh/labels_deploy.rs` is at an acceptable level.
- Code coverage for `src/app/commands/create/mod.rs` is at an acceptable level.
- Code coverage for `src/app/commands/list/mod.rs` is at an acceptable level.
- Tests assert externally observable behavior at the owning boundary using `assert_cmd` and `TestContext` without duplicating knowledge of internal implementation details.

## Risks

- Integrating mocks into the `$PATH` in integration tests requires careful handling of shared state. If the `TestContext` doesn't properly isolate `$PATH` modifications per test, tests might fail sporadically. This risk is mitigated by running commands in isolated temp directories and using isolated `$PATH` or isolated mock scripts per test context.
- The `mev-internal` tests require setup of the `tests/` directory and testing dependencies in its `Cargo.toml`.
