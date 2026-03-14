---
label: "refacts"
created_at: "2024-03-14"
author_role: "cli_sentinel"
confidence: "high"
---

## Problem

The `backup` command uses a `--list` (or `-l`) option to alter the primary execution verb (from "backup" to "list"), breaking the structural consistency of the CLI commands.

## Goal

Ensure the CLI commands adhere to the strict 'verb [object] arguments' structure and options do not substitute a required object or alter the primary execution verb.

## Context

Using an option to perform a distinct action (like listing available targets instead of performing a backup) violates the CLI Command Structure design rule. Options should only be used for additive behavior, limited-use edge cases, modifying existing behavior, output/safety control, or explicit override of auto-resolved context. A separate subcommand (e.g., `mev backup list` or integrating into `mev list`) should be used for this distinct action.

## Evidence

- path: "src/app/cli/backup.rs"
  loc: "BackupArgs"
  note: "The `list` boolean flag is defined as an option that alters the command's primary behavior."
- path: "src/app/cli/backup.rs"
  loc: "run"
  note: "The `run` function branches its execution based on the `list` flag, either calling `commands::backup::list_targets()` or `commands::backup::execute()`, demonstrating that the option changes the primary execution verb."

## Change Scope

- `src/app/cli/backup.rs`
- `src/app/commands/backup/mod.rs`
