---
label: "refacts"
created_at: "2026-03-14"
author_role: "cli_sentinel"
confidence: "high"
---

## Problem

The `backup` command uses a `--list` option toggle to change its primary behavior from performing a backup to listing available targets, making the positional `target` argument conditionally mandatory.

## Goal

Align the `backup` command with standard structure by removing the `--list` toggle and using standard subcommands (e.g., `mev backup list`), or by making `target` a mandatory positional argument that outputs valid options upon missing argument errors.

## Context

A command should consistently read as `verb [object] arguments`. The `--list` toggle acts as a subcommand masquerading as an option, violating the structural contract and option classification rules. It creates an exception condition where the positional argument becomes required only if the option is absent.

## Evidence

- path: "src/app/cli/backup.rs"
  loc: "BackupArgs"
  note: "Defines the `--list` toggle and models the `target` argument as an `Option<String>`."
- path: "src/app/cli/backup.rs"
  loc: "run"
  note: "Branching logic uses `args.list` to bypass the missing target error, confirming `--list` is used as a subcommand rather than a modifier."

## Change Scope

- `src/app/cli/backup.rs`
