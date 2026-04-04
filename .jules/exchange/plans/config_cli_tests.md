---
label: "tests"
---

## Goal

Add CLI tests for the `config` command to verify that `mev config deploy` properly triggers the domain logic, and expand unit test coverage for `deploy_internal`.

## Current State

- `src/app/commands/config/mod.rs`: Only the primary success path is unit-tested. Error paths like invalid role are uncovered.
- `tests/cli/config.rs`: File exists but contains no test code.

## Plan

1. Update `src/app/commands/config/mod.rs` to include unit tests for `deploy_internal` covering all logic paths (invalid role, no roles, existing target without overwrite).
2. Update `tests/cli/config.rs` to include a CLI contract test exercising `mev config deploy` using `TestContext` to ensure domain orchestration triggers correctly.

## Constraints

- Code changes must adhere to the project's strict design principles, such as single responsibility and accurate domain modeling.
- Modifications should not inadvertently break unconnected tests or configurations.
- Tests assert externally observable behavior at the owning boundary.

## Acceptance Criteria

- The core issues detailed in the problem statements are resolved.
- Required tests are written or passing after the change.
- The identified file paths in the change scope have been appropriately modified according to the goal.
