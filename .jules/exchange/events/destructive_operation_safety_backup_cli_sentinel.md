---
label: "bugs"
created_at: "2026-03-11"
author_role: "cli_sentinel"
confidence: "high"
---

## Problem

The `backup` command performs destructive file writes (overwriting existing backup files) without prompting for confirmation or requiring a safety override flag like `--overwrite`.

## Goal

Add an `--overwrite` flag to the `backup` command and enforce it or a confirmation prompt when the target destination file already exists.

## Context

Destructive operations (such as file writes that replace existing data) must employ uniform safety contracts. Other commands like `create`, `make`, and `config create` already utilize the `--overwrite` flag to prevent accidental data loss. The `backup` command performs file overwrites directly but lacks this required safety measure.

## Evidence

- path: "src/app/cli/backup.rs"
  loc: "pub struct BackupArgs"
  note: "The `--overwrite` flag is missing from the CLI arguments definition."
- path: "src/app/commands/backup/mod.rs"
  loc: "ctx.fs.write(output_file, lines.join(\"\\n\").as_bytes())?;"
  note: "The command writes system backup directly without checking if the file exists or requiring an overwrite flag."
- path: "src/app/commands/backup/mod.rs"
  loc: "ctx.fs.write(output_file, format!(\"{content}\\n\").as_bytes())?;"
  note: "The command writes vscode extension backups directly without any safety checks."

## Change Scope

- `src/app/cli/backup.rs`
- `src/app/commands/backup/mod.rs`
