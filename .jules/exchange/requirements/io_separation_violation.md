---
label: "refacts"
implementation_ready: false
---

## Goal

Ensure that CLI tools strictly output only structured, script-parseable result data to `stdout`, and write all human-readable diagnostics, progress indicators, logs, and errors to `stderr` (`eprintln!`).

## Problem

Multiple commands (`create`, `make`, `backup`, `update`, `config`, `switch`, `identity`) write human-readable diagnostics, progress indicators, and logs to `stdout` (`println!`), violating the CLI I/O separation contract.

## Context

The Domain I/O Decoupling and CLI I/O Separation rules mandate that `stdout` must carry result data and `stderr` must carry warnings, logs, and errors. Mixed streams break automation pipelines (e.g., piping results to `jq` fails if logs pollute `stdout`).

## Evidence

- source_event: "io_separation_violation_cli_sentinel.md"
  path: "src/app/commands/create/mod.rs"
  loc: "execute"
  note: "Writes progress indicators (e.g., `println!(\"mev: Creating {} environment\", plan.profile);`, `println!(\"[{step}/{total}] Running: {tag}\");`) to stdout."
- source_event: "io_separation_violation_cli_sentinel.md"
  path: "src/app/commands/backup/mod.rs"
  loc: "execute"
  note: "Writes progress messages (e.g., `println!(\"Running backup: {}\", target.description());`, `println!(\"✓ Backup completed successfully!\");`) to stdout."
- source_event: "io_separation_violation_cli_sentinel.md"
  path: "src/app/commands/update/mod.rs"
  loc: "execute"
  note: "Writes log messages to stdout (e.g., `println!(\"Running upgrade...\");`)."
- source_event: "io_separation_violation_cli_sentinel.md"
  path: "src/app/commands/switch/mod.rs"
  loc: "execute"
  note: "Writes log messages to stdout (e.g., `println!(\"Switching to {} identity...\", identity);`)."

## Change Scope

- `src/app/commands/backup/mod.rs`
- `src/app/commands/config/mod.rs`
- `src/app/commands/create/mod.rs`
- `src/app/commands/identity/mod.rs`
- `src/app/commands/make/mod.rs`
- `src/app/commands/switch/mod.rs`
- `src/app/commands/update/mod.rs`

## Constraints

- Ensure all changes align with architecture and design rules.
- Maintain tests for all new logic.

## Acceptance Criteria

- The problem is fully resolved.
- Pre-commit checks and tests pass.
