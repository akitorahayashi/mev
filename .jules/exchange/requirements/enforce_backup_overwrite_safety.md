---
label: "bugs"
implementation_ready: false
---

## Goal

Add an `--overwrite` flag to the `backup` command and enforce it or a confirmation prompt when the target destination file already exists.

## Problem

The `backup` command performs destructive file writes (overwriting existing backup files) without prompting for confirmation or requiring a safety override flag like `--overwrite`. Other commands like `create`, `make`, and `config create` already utilize the `--overwrite` flag to prevent accidental data loss.

## Evidence

- source_event: "destructive_operation_safety_backup_cli_sentinel.md"
  path: "src/app/cli/backup.rs"
  loc: "pub struct BackupArgs"
  note: "The `--overwrite` flag is missing from the CLI arguments definition."
- source_event: "destructive_operation_safety_backup_cli_sentinel.md"
  path: "src/app/commands/backup/mod.rs"
  loc: "ctx.fs.write(output_file, lines.join(\"\\n\").as_bytes())?;"
  note: "The command writes system backup directly without checking if the file exists or requiring an overwrite flag."
- source_event: "destructive_operation_safety_backup_cli_sentinel.md"
  path: "src/app/commands/backup/mod.rs"
  loc: "ctx.fs.write(output_file, format!(\"{content}\\n\").as_bytes())?;"
  note: "The command writes vscode extension backups directly without any safety checks."

## Change Scope

- `src/app/cli/backup.rs`
- `src/app/commands/backup/mod.rs`

## Constraints

- Destructive operations (like file writes) must require explicit confirmation or safety flags such as `--overwrite`.

## Acceptance Criteria

- The `backup` command's CLI args `BackupArgs` accept an `--overwrite` boolean flag.
- When `backup` is called, it verifies if the target file exists before writing.
- If the file exists and `--overwrite` is not passed, an error must be returned avoiding accidental data loss.
