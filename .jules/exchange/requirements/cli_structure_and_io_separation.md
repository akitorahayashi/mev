---
label: "refacts"
implementation_ready: false
---

## Goal

Restructure the CLI to strictly separate output streams (data to `stdout`, logs/diagnostics to `stderr`) and ensure structural command predictability by removing behavior-altering option flags like `--list`.

## Problem

Diagnostic logs and human-readable messages are currently written to `stdout`, which breaks parsability for automated tools. Additionally, the `backup` command relies on a mutually exclusive `--list` flag to change its fundamental behavior (verb) rather than using a dedicated subcommand, violating structural consistency.

## Evidence

- source_event: "backup_list_option_cli_sentinel.md"
  path: "src/app/cli/backup.rs"
  loc: "pub struct BackupArgs { ... pub list: bool ... pub target: Option<String> ... }"
  note: "Defines a `--list` flag that mutually excludes the positional `target` argument."
- source_event: "backup_list_option_cli_sentinel.md"
  path: "src/app/cli/backup.rs"
  loc: "if args.list { ... } else if let Some(target) = args.target { ... }"
  note: "Executes completely different code paths based on the presence of the `--list` option, replacing the command's primary verb."
- source_event: "backup_list_option_cli_sentinel.md"
  path: "src/app/commands/backup/mod.rs"
  loc: "pub fn list_targets()"
  note: "Implements the listing action that currently relies on the `--list` option rather than a structural command."
- source_event: "io_separation_logs_stdout_cli_sentinel.md"
  path: "src/app/commands/create/mod.rs"
  loc: "println!(\"mev: Creating {} environment\", plan.profile);"
  note: "Outputs raw log text to `stdout` during environment creation."
- source_event: "io_separation_logs_stdout_cli_sentinel.md"
  path: "src/app/commands/switch/mod.rs"
  loc: "println!(\"Switching to {} identity...\", identity);"
  note: "Outputs state changes and progress logs to `stdout`."
- source_event: "io_separation_logs_stdout_cli_sentinel.md"
  path: "src/app/commands/backup/mod.rs"
  loc: "println!(\"Running backup: {}\", target.description());"
  note: "Informational message about running backups sent to `stdout`."
- source_event: "io_separation_logs_stdout_cli_sentinel.md"
  path: "src/app/commands/make/mod.rs"
  loc: "println!(\"Running tags: {}\", plan.tags.join(\", \"));"
  note: "Diagnostic progress information sent to `stdout`."

## Change Scope

- `src/app/cli/backup.rs`
- `src/app/commands/backup/mod.rs`
- `src/app/commands/create/mod.rs`
- `src/app/commands/make/mod.rs`
- `src/app/commands/switch/mod.rs`
- `src/app/commands/deploy_configs.rs`

## Constraints

- Structured output must be on `stdout`.
- Diagnostic output (informational strings, logs) must use `stderr`.

## Acceptance Criteria

- Human-readable logs and progress markers use `eprintln!` instead of `println!`.
- The `backup` command's `--list` flag is removed and replaced by a structural approach (e.g., `backup list` subcommand).
