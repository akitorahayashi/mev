---
label: "refacts"
created_at: "2024-05-18"
author_role: "taxonomy"
confidence: "high"
---

## Problem

The term `target` is overloaded across the codebase. It refers to:
1. `backup_target.rs` (`BackupTarget::System`, `BackupTarget::Vscode`) - representing a logical backup unit or operation.
2. Build outputs (e.g. "target output", "target directory", "all-targets") in comments and aliases.

Specifically, `backup_target` is a problematic name. In a domain where tasks are executed using Ansible "tags" (e.g., `system`, `vscode`, `rust`), the `backup` command operates on the exact same domain nouns (e.g. "system", "vscode"), but it calls them "backup targets" instead of "backup scopes", "backup entities", or even just "backup tags". This creates two parallel terms (tags vs targets) pointing to essentially the same underlying concept (a logical unit of configuration), muddying the domain vocabulary.

## Goal

Rename `backup_target` to a more precise domain term, such as `backup_scope` or `backup_component`, to differentiate it from other uses of the word "target" (like repositories or build artifacts) and to align it better with the broader system terminology. Given that we backup a "system" or "vscode", "scope" is a fitting noun. So, `backup_target` becomes `backup_scope`, `BackupTarget` becomes `BackupScope`, and its variants stay the same.

## Context

Using `target` generically hides the actual responsibility. In `src/app/commands/backup/mod.rs`, we do `execute(ctx: &DependencyContainer, target_input: &str)` and resolve `BackupTarget`. Changing it to `scope` makes it explicit that we are backing up a specific scope of the system.

## Evidence

- path: "src/domain/backup_target.rs"
  loc: "enum BackupTarget"
  note: "Defines `BackupTarget` enum to represent backup entities like `System` and `Vscode`."
- path: "src/app/cli/backup.rs"
  loc: "struct BackupArgs"
  note: "CLI argument is named `target`."

## Change Scope

- `src/domain/backup_target.rs`
- `src/domain/error.rs`
- `src/app/cli/backup.rs`
- `src/app/commands/backup/mod.rs`