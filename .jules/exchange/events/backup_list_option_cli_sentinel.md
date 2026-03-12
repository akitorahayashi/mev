---
label: "refacts"
created_at: "2023-10-25"
author_role: "cli_sentinel"
confidence: "high"
---

## Problem

The `backup` command relies on an option flag (`--list`) to completely change its primary execution action from performing a backup to listing available targets, creating a mutually exclusive relationship with the positional `target` argument. This violates option classification principles and command structure (verb + [object]).

## Goal

Restructure the `backup` command to eliminate the `--list` flag. Listing should be handled as a standard subcommand (e.g., `backup list`) or by displaying targets when `backup` is invoked without a target, strictly separating the primary command verb from its options.

## Context

Using an option to perform a primary verb action (like listing) adds help-text noise and breaks the predictability of the command structure. Options should modify behavior, not define the action itself. The current design introduces conditional mandatory arguments ("Target is required unless --list is used"), which indicates structural drift.

## Evidence

- path: "src/app/cli/backup.rs"
  loc: "pub struct BackupArgs { ... pub list: bool ... pub target: Option<String> ... }"
  note: "Defines a `--list` flag that mutually excludes the positional `target` argument."
- path: "src/app/cli/backup.rs"
  loc: "if args.list { ... } else if let Some(target) = args.target { ... }"
  note: "Executes completely different code paths based on the presence of the `--list` option, replacing the command's primary verb."
- path: "src/app/commands/backup/mod.rs"
  loc: "pub fn list_targets()"
  note: "Implements the listing action that currently relies on the `--list` option rather than a structural command."

## Change Scope

- `src/app/cli/backup.rs`
- `src/app/commands/backup/mod.rs`
