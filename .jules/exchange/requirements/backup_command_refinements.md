---
label: "tests"
implementation_ready: false
---

## Goal
Improve the `backup` command's CLI structure, IO separation, and test coverage to ensure safe and predictable file processing.

## Problem
The `backup` command exhibits multiple structural issues: it relies on a structural anti-pattern by using a `--list` option as a subcommand, breaking standard CLI argument parsing. It also pollutes `stdout` with fallback warnings instead of routing them to `stderr`. Furthermore, the core orchestrating logic for system defaults and VSCode has critically low test coverage (8.5%), posing a risk for regressions that could lead to data loss.

## Context
This requirement aggregates observer events related to the problem statement above.

## Evidence
- source_event: "backup_command_structure_drift_cli_sentinel.md"
  path: "src/app/cli/backup.rs"
  loc: "BackupArgs"
  note: "Defines the `--list` toggle and models the `target` argument as an `Option<String>`."
- source_event: "backup_io_separation_warning_cli_sentinel.md"
  path: "src/app/commands/backup/mod.rs"
  loc: "57-61"
  note: "Uses `println!` to emit 'Local definitions not found at ...' instead of `eprintln!`."
- source_event: "backup_command_missing_coverage_cov.md"
  path: "src/app/commands/backup/mod.rs"
  loc: "44-310"
  note: "Line coverage is at 13/153 lines (8.5%). Untested logic includes `execute_system` and `execute_vscode`."

## Change Scope
- `src/app/cli/backup.rs`
- `src/app/commands/backup/mod.rs`
- `tests/cli/backup.rs`

## Constraints
- The `backup` command structure must conform to `verb [object] arguments`.
- Warnings and non-data logs must strictly output to `stderr`.

## Acceptance Criteria
- `--list` flag is replaced by a subcommand or robust positional parsing.
- Missing local definitions fallback warning is routed to `stderr`.
- Coverage for `backup` orchestrator logic exceeds baseline thresholds via robust unit/integration tests.
