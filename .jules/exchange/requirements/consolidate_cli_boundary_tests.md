---
label: "tests"
implementation_ready: false
---

## Goal

Consolidate basic CLI shape checks (like validating help outputs for common or global flags) into a single boundary test (e.g., `tests/cli/help_and_version.rs` or similar) to ensure the CLI structure is validated without copy-pasting tests for every subcommand.

## Problem

Tests verifying CLI flag parsing (like `--overwrite` or `--verbose` in help text) are duplicated across multiple command tests (e.g., `tests/cli/create.rs`, `tests/cli/make.rs`). This leads to redundancy, increasing maintenance cost without adding coverage for the underlying CLI definition logic.

## Evidence

- source_event: "unnecessary_cli_command_test_duplication_qa.md"
  path: "tests/cli/create.rs"
  loc: "8"
  note: "Tests `create_help_shows_overwrite_flag` and `create_help_shows_verbose_flag`."
- source_event: "unnecessary_cli_command_test_duplication_qa.md"
  path: "tests/cli/make.rs"
  loc: "7"
  note: "Tests `make_help_shows_overwrite_flag` and `make_help_shows_verbose_flag`."

## Change Scope

- `tests/cli/create.rs`
- `tests/cli/make.rs`
- `tests/cli/help_and_version.rs`

## Constraints

- Basic CLI shape checks must be consolidated into a single boundary test.
- Tests should not be duplicated across subcommand test files for global flags.

## Acceptance Criteria

- Global flags are verified in a single consolidated CLI boundary test file.
- Redundant global flag tests are removed from `tests/cli/create.rs`, `tests/cli/make.rs`, and similar subcommand tests.