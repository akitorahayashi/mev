---
label: "tests"
---

## Goal

Provide comprehensive integration test coverage for backup orchestrations, GitHub label management, and VCS identity switching to prevent silent failures and regressions. Tests must assert externally observable behavior at the owning boundary.

## Current State

- `src/app/commands/backup/mod.rs`: Critical methods lack line-level tests, risking regressions when data formats change.
- `src/app/commands/switch/mod.rs`: Performs critical auth/state transitions but lacks integration tests validating the success or failure paths based on configuration state.
- `crates/mev-internal/src/app/commands/gh/labels_reset.rs`: Deletes all labels from a GitHub repository but has 0% coverage.
- `crates/mev-internal/src/app/commands/gh/labels_deploy.rs`: Manipulates repository labels by replacing or creating them based on a bundled catalog, but has 0% coverage.
- `tests/cli/backup.rs`: Has CLI contract tests but lacks execution logic coverage for `backup system` and `backup vscode` paths.
- `tests/cli/switch.rs`: Has CLI contract tests but lacks execution logic coverage for `switch <identity>` success and failure paths.
- `crates/mev-internal/tests/gh_contracts.rs`: Does not exist. Needs to be created to house `gh` sub-command contracts.

## Plan

1. Modify `tests/cli/backup.rs` to add integration tests validating `backup system` and `backup vscode` success and failure paths, mocking necessary filesystem and command interactions via `TestContext` overrides (e.g. creating setting definitions in the isolated work directory).
2. Modify `tests/cli/switch.rs` to add integration tests validating `switch <identity>` success and failure paths. Use `TestContext` to create mock identities and verify that `mev` issues appropriate `git config` and `jj config` commands (via mocked `$PATH` binaries).
3. Create `crates/mev-internal/tests/gh_contracts.rs` to validate `gh labels deploy` and `gh labels reset`. Test should inject a mock `gh` binary via `$PATH` modification to capture arguments passed to the `gh` command and verify proper invocation of `list`, `delete`, and `create` operations. The `mev-internal` crate tests can invoke its commands programmatically.
4. Verify tests pass by running `cargo test` and `cd crates/mev-internal && cargo test`.
5. Complete pre-commit steps to ensure proper testing, verification, review, and reflection are done.
6. Submit the change with a descriptive commit message.

## Acceptance Criteria

- Integration tests cover the success and failure paths for `backup`, `switch`, `gh labels deploy`, and `gh labels reset`.
- Test coverage ensures fallback logic, error handling, and file creation behaviors are validated.
- Tests assert externally observable behavior at the owning boundary.
- Tests must not rely on mutating the developer's global system state.

## Risks

- Mocking commands like `gh` or `git` via `$PATH` overrides could be brittle if the format of the mock script is slightly off or if concurrency issues arise with `std::env::set_var`. Need to use `serial_test` crate `#[serial]` macro when modifying global state in tests.
