---
label: "bugs"
implementation_ready: false
---

## Goal

Migrate all operational logs, progress messages, and non-result output to `stderr` (e.g., using `eprintln!`), reserving `stdout` strictly for command result data.

## Problem

Operational logs, progress updates, and success messages are written to `stdout` instead of `stderr`, mixing diagnostic information with result data. Commands like `create`, `make`, `backup`, and `update` use `println!` for logging progress and status updates. This mixes streams and causes automation failures, as piped output will include non-data logs rather than pure result streams.

## Evidence

- source_event: "io_separation_operational_logs_cli_sentinel.md"
  path: "src/app/commands/create/mod.rs"
  loc: "println!(\"[{step}/{total}] Running: {tag}\");"
  note: "Progress logs are emitted to stdout."
- source_event: "io_separation_operational_logs_cli_sentinel.md"
  path: "src/app/commands/make/mod.rs"
  loc: "println!(\"Running tags: {}\", plan.tags.join(\", \"));"
  note: "Operation logs are emitted to stdout."
- source_event: "io_separation_operational_logs_cli_sentinel.md"
  path: "src/app/commands/backup/mod.rs"
  loc: "println!(\"Running backup: {}\", target.description());"
  note: "Start logs are emitted to stdout."
- source_event: "io_separation_operational_logs_cli_sentinel.md"
  path: "src/app/commands/update/mod.rs"
  loc: "println!(\"Running upgrade...\");"
  note: "Process logs are emitted to stdout."
- source_event: "io_separation_operational_logs_cli_sentinel.md"
  path: "src/app/commands/deploy_configs.rs"
  loc: "println!(\"  Deployed config for {role}\");"
  note: "Deployment progress logs are emitted to stdout."

## Change Scope

- `src/app/commands/create/mod.rs`
- `src/app/commands/make/mod.rs`
- `src/app/commands/backup/mod.rs`
- `src/app/commands/update/mod.rs`
- `src/app/commands/deploy_configs.rs`
- `src/app/commands/config/mod.rs`

## Constraints

- Operational logs and progress updates must be written to `stderr` (e.g., using `eprintln!`) rather than `stdout` to separate I/O streams and prevent breaking automation pipelines.
- Standard output (`stdout`) should exclusively carry the output data result of commands.

## Acceptance Criteria

- All `println!` usage for diagnostic or progress output in commands is replaced with `eprintln!`.
- Piping the commands listed in the scope through command line utilities (like `grep` or standard redirection) cleanly outputs data without operational message pollution on `stdout`.
