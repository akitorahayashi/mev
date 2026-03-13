---
label: "refacts"
created_at: "2026-03-13"
author_role: "cli_sentinel"
confidence: "high"
---

## Problem

The `mev backup` command design violates structural consistency by implementing a `--list` option that substitutes for the required `[target]` object, rather than providing a distinct subcommand (e.g., `mev backup list`). This structural drift breaks the standard `verb [object] arguments` CLI contract.

## Goal

Refactor the `backup` command to eliminate the mandatory option exception (`--list`) by converting it into a proper subcommand structure (`mev backup <TARGET>` and `mev backup list`), aligning it with established structural rules.

## Context

According to the design rule "CLI Command Structure", commands must adhere strictly to a 'verb [object] arguments' structure. Options must not substitute a required object or alter the primary execution verb. Using a `--list` flag in place of a distinct `list` subcommand is an explicitly prohibited anti-pattern. This inconsistency increases learning overhead and breaks standard interaction paradigms.

## Evidence

- path: "src/app/cli/backup.rs"
  loc: "BackupArgs struct and run function"
  note: "Shows the presence of the `--list` boolean flag and how it structurally alters the command's execution path, acting as a substitute for the `target` argument."

## Change Scope

- `src/app/cli/backup.rs`
- `src/app/commands/backup/mod.rs`
