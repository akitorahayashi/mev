---
label: "tests"
implementation_ready: false
---

## Goal

Add CLI tests for the `config` command to verify that `mev config deploy` properly triggers the domain logic.

## Problem

The `config` command CLI contract tests are entirely missing, and the command logic (`src/app/commands/config/mod.rs`) only has 16/33 lines covered.

## Context

The `config` command orchestration interacts heavily with `AnsiblePort` and `FsPort` to orchestrate file system modifications (moving role config directories into `.config`). A failure in this path would silently fail to configure essential development tools on new environments. The unit test `test_deploy_config_success` checks a happy path, but there are multiple paths in `deploy_internal` (e.g. invalid role, no roles, existing target without overwrite) and zero CLI contract tests in `tests/cli/config.rs`.

## Evidence

- path: "src/app/commands/config/mod.rs"
  loc: "16/33 lines tested"
  note: "Only the primary success path is unit-tested. Error paths like invalid role are uncovered."
- path: "tests/cli/config.rs"
  loc: "whole file"
  note: "File exists but contains no test code."

## Change Scope

- `src/app/commands/config/mod.rs`
- `tests/cli/config.rs`

## Constraints

- Code changes must adhere to the project's strict design principles, such as single responsibility and accurate domain modeling.
- Modifications should not inadvertently break unconnected tests or configurations.

## Acceptance Criteria

- The core issues detailed in the problem statements are resolved.
- Required tests are written or passing after the change.
- The identified file paths in the change scope have been appropriately modified according to the goal.
