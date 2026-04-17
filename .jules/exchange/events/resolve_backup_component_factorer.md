---
label: "refacts"
created_at: "2024-04-17"
author_role: "factorer"
confidence: "high"
---

## Problem

Wrapper Sprawl and Discrepancy between name and contract: `src/domain/backup_component.rs` contains `resolve_backup_component` which looks up a string alias and returns `Option<BackupComponent>`, and `validate_backup_component` which wraps it to return a `Result<BackupComponent, AppError>`. `validate_backup_component` is exactly parsing a string into an enum variant.

## Goal

Implement standard traits (`FromStr` and potentially `TryFrom<&str>`) for `BackupComponent` to remove unnecessary indirection.

## Context

The separation between resolve and validate represents wrapper sprawl and an unnecessary indirection that could be simplified by using idiomatic Rust trait implementations.

## Evidence

- path: "src/domain/backup_component.rs"
  loc: "68-88"
  note: "Contains resolve_backup_component and validate_backup_component wrappers."
- path: "src/app/commands/backup/mod.rs"
  loc: "16"
  note: "User input is parsed via validate_backup_component."

## Change Scope

- `src/domain/backup_component.rs`
- `src/app/commands/backup/mod.rs`
