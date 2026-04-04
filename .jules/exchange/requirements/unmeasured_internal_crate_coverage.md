---
label: "tests"
implementation_ready: true
---

## Goal

Determine why the `cargo tarpaulin` coverage report generated via `just coverage` is completely omitting the test execution for `mev-internal` unit tests, and adjust the coverage command or testing bounds to capture these critical path assertions.

## Problem

The internal crate modules `crates/mev-internal/src/domain/repo_target.rs` and `crates/mev-internal/src/domain/repository_ref.rs` are reported as having 0% code coverage despite having comprehensive unit test modules in their files.

## Context

The `repo_target` and `repository_ref` modules validate and resolve Git remote targets. Failure in this path will break label provisioning tools. The codebase clearly contains tests for these domains (e.g. `prefers_explicit_repo`, `parses_owner_name_repo_arg`), but they are not reflected in the coverage metrics. This is a false negative in the risk signal because `just coverage` calls `cargo tarpaulin --packages mev`, explicitly omitting `mev-internal` from the report.

## Evidence

- path: "justfile"
  loc: "line 53 (`--packages mev`)"
  note: "The tarpaulin invocation is restricted to the `mev` package and explicitly excludes workspace members."
- path: "crates/mev-internal/src/domain/repo_target.rs"
  loc: "0/8 lines covered"
  note: "Tests are present in the file but not run during the coverage step."
- path: "crates/mev-internal/src/domain/repository_ref.rs"
  loc: "0/50 lines covered"
  note: "Tests are present in the file but not run during the coverage step."

## Change Scope

- `justfile`

## Constraints

- Code changes must adhere to the project's strict design principles, such as single responsibility and accurate domain modeling.
- Modifications should not inadvertently break unconnected tests or configurations.

## Acceptance Criteria

- The core issues detailed in the problem statements are resolved.
- Required tests are written or passing after the change.
- The identified file paths in the change scope have been appropriately modified according to the goal.
