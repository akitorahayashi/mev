---
label: "tests"
implementation_ready: false
---

## Goal

Ensure that the core command logic is covered by tests, verifying that commands parse arguments and execute the correct adapter methods or correctly handle errors.

## Problem

The CLI command implementation modules in `crates/mev-internal/src/app/commands/` have 0% test coverage.

## Context

These command modules are the entry points for the CLI functionality in `mev-internal`. While the underlying adapters (like `GitAdapter` and `GhAdapter`) have some tests, the command logic itself (argument validation, orchestration of adapter calls) is completely untested. This creates a regression risk where changes to argument parsing or command orchestration could break functionality without being detected. The coverage gaps were identified using default `cargo tarpaulin` configuration, evaluating the line coverage metric.

## Evidence

- path: "crates/mev-internal/src/app/commands/gh/labels_deploy.rs"
  loc: "run"
  note: "This command orchestrates reading existing labels and creating/replacing labels via the GhAdapter, but the logic is untested."
- path: "crates/mev-internal/src/app/commands/gh/labels_reset.rs"
  loc: "run"
  note: "This command orchestrates listing and deleting labels via the GhAdapter, but the logic is untested."
- path: "crates/mev-internal/src/app/commands/git/delete_submodule.rs"
  loc: "run"
  note: "This command orchestrates the steps to delete a git submodule via the GitAdapter, but the logic is untested."

## Change Scope

- `crates/mev-internal/src/app/commands/gh/labels_deploy.rs`
- `crates/mev-internal/src/app/commands/gh/labels_reset.rs`
- `crates/mev-internal/src/app/commands/git/delete_submodule.rs`

## Constraints

- Mock underlying adapters if necessary.

## Acceptance Criteria

- Command logic correctly parsing arguments and orchestrating adapters is verified by tests.
