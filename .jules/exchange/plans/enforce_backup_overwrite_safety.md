---
label: "bugs"
---

## Goal
Add an `--overwrite` flag to the `backup` command and enforce it or a confirmation prompt when the target destination file already exists.

## Problem
The `backup` command performs destructive file writes (overwriting existing backup files) without prompting for confirmation or requiring a safety override flag like `--overwrite`.

## Affected Areas

### CLI Command Backup
- `src/app/cli/backup.rs`
- `src/app/commands/backup/mod.rs`

## Constraints
- Destructive operations (like file writes) must require explicit confirmation or safety flags such as `--overwrite`.

## Risks
- Accidental data loss due to overwriting existing backup files.

## Acceptance Criteria
- The `backup` command's CLI args `BackupArgs` accept an `--overwrite` boolean flag.
- When `backup` is called, it verifies if the target file exists before writing.
- If the file exists and `--overwrite` is not passed, an error must be returned avoiding accidental data loss.

## Implementation Plan
1. Add `overwrite: bool` with `#[arg(short, long)]` to `BackupArgs` struct in `src/app/cli/backup.rs`.
2. In `src/app/cli/backup.rs`, update `run` to pass `args.overwrite` to `commands::backup::execute`.
3. In `src/app/commands/backup/mod.rs`, update the signature of `execute` to accept `overwrite: bool`.
4. In `src/app/commands/backup/mod.rs`, update the signatures of `execute_system` and `execute_vscode` to accept `overwrite: bool`. Pass `overwrite` to them in `execute`.
5. In `src/app/commands/backup/mod.rs`, inside `execute_system` and `execute_vscode`, before writing the file with `ctx.fs.write`, check if `ctx.fs.exists(output_file)`. If it exists and `!overwrite`, return an appropriate `AppError` variant (like `AppError::Backup`) with an appropriate message.
