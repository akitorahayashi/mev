---
label: "refacts"
implementation_ready: false
---

## Goal

Rename `backup_target` to `backup_scope` or `backup_component` to align it with domain terminology and distinguish it from build/repo targets.

## Problem

The term `target` is overloaded. It refers to a logical backup unit (`BackupTarget::System`), but also to build outputs and directories. A precise domain term is required.

## Evidence

- source_event: "backup_target_taxonomy.md"
  path: "src/domain/backup_target.rs"
  loc: "enum BackupTarget"
  note: "Defines `BackupTarget` enum to represent backup entities like `System` and `Vscode`."

## Change Scope

- `src/domain/backup_target.rs`
- `src/domain/error.rs`
- `src/app/cli/backup.rs`
- `src/app/commands/backup/mod.rs`

## Constraints

- Eliminate overloaded use of "target".

## Acceptance Criteria

- `BackupTarget` and related variables are renamed to `BackupScope` or another clear domain noun.