---
label: "tests"
---

## Goal

Implement CLI contract tests for the `make` command to ensure the execution plan creation, tag validation, and Ansible playbook invocation are covered and protected from regressions.

## Current State

The `make` command orchestration has 0% line coverage, and its corresponding CLI contract test file is completely empty. This lack of testing leaves critical state-transitioning paths unprotected from regressions.
- `src/app/commands/make/mod.rs`: Contains the execution logic for the `make` command, but it has 0 lines covered by tests according to coverage reports.
- `tests/cli/make.rs`: An initialized but empty file that currently contains no test assertions or test logic for the `make` command.

## Plan

1. Implement a test `make_executes_ansible_playbook_successfully` in `tests/cli/make.rs` that mocks the `ansible-playbook` execution and asserts that calling the CLI with `make shell` output indicates successful configuration and execution.
2. Implement a test `make_invalid_tag_fails` in `tests/cli/make.rs` that calls the CLI with `make invalid-tag` and asserts a failure output matching the "invalid tag" error.

## Constraints

- Code changes must adhere to the project's strict design principles, such as single responsibility and accurate domain modeling.
- Modifications should not inadvertently break unconnected tests or configurations.
- Tests assert externally observable behavior at the owning boundary.

## Acceptance Criteria

- The core issues detailed in the problem statements are resolved.
- Required tests are written or passing after the change.
- The identified file paths in the change scope have been appropriately modified according to the goal.
