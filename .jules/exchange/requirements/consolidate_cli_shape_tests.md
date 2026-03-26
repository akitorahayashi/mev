---
label: "tests"
implementation_ready: false
---

## Goal

Consolidate basic CLI shape checks into a central boundary test and provide foundational test coverage for untrusted/uncovered boundary interfaces and CLI logic in the main app and internal crates.

## Problem

Duplicate boundary test coverage for CLI argument checks exists across subcommand test files instead of centralizing global arguments and shape tests. Simultaneously, the main app's CLI layer and several critical CLI commands, adapters, and domain modules completely lack test coverage, specifically in `crates/mev-internal`.

## Evidence

- source_event: "duplicate_help_and_shape_tests_qa.md"
  path: "tests/cli/make.rs"
  loc: "6-24"
  note: "Verifies make help shows overwrite flag, verbose flag, and profile flag."
- source_event: "uncovered_commands_and_adapters_cov.md"
  path: "crates/mev-internal/src/app/cli/gh.rs"
  loc: "0/7"
  note: "Only tests exist for subcommand shape but zero line coverage recorded in tarpaulin output."
- source_event: "unverified_cli_contracts_cov.md"
  path: "src/app/cli/make.rs"
  loc: "0/3 lines"
  note: "Command parsing entirely uncovered."

## Change Scope

- `tests/cli/help_and_version.rs`
- `tests/cli/create.rs`
- `tests/cli/make.rs`
- `tests/cli/list.rs`
- `tests/cli/backup.rs`
- `tests/cli/switch.rs`
- `tests/cli/config.rs`
- `tests/cli/identity.rs`
- `crates/mev-internal/tests/`
- `crates/mev-internal/src/app/commands/`
- `crates/mev-internal/src/adapters/`
- `src/app/cli/mod.rs`

## Constraints

- Centralize CLI shape tests instead of duplicating them.
- Add coverage for missing boundaries in `crates/mev-internal` and `src/app/cli`.

## Acceptance Criteria

- Basic CLI shape checks are consolidated into a single central test file (e.g., `tests/cli/help_and_version.rs`).
- Coverage boundaries for missing CLI/adapter areas are established.